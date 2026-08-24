use core::mem::{offset_of, size_of};

use miso_engine_session::{canonical_session_toml, parse_session_toml};

use super::*;

fn one_track_session(quantum: u32) -> String {
    let mut model = parse_session_toml(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.toml"
    ))
    .expect("accepted fixture");
    model.quantum_frames = quantum;
    model.limits.pcm_ring_frames = u64::from(quantum);
    model.sources[0].mapping.region.length_samples = u64::from(quantum) * 2;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    canonical_session_toml(&model).expect("canonical one-track session")
}

fn prepared_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut host =
        AudioWorkletEngineHost::new(WebPrepareConfigV1::launch_defaults(48_000, quantum));
    assert_eq!(host.prepare(), RESULT_OK);
    host
}

fn ready_host(quantum: u32) -> AudioWorkletEngineHost {
    let toml = one_track_session(quantum);
    let mut host = prepared_host(quantum);
    host.session_toml_mut().expect("prepared TOML buffer")[..toml.len()]
        .copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );
    host
}

#[test]
fn frozen_layouts_and_values_are_exact() {
    assert_eq!(size_of::<WebPrepareConfigV1>(), 192);
    assert_eq!(size_of::<WebStatusV1>(), 80);
    assert_eq!(size_of::<WebResourceReportV1>(), 224);
    assert_eq!(
        [
            RESULT_OK,
            RESULT_INVALID_ARGUMENT,
            RESULT_ABI_MISMATCH,
            RESULT_WRONG_STATE,
            RESULT_BUFFER_TOO_SMALL,
            RESULT_PREPARE_REJECTED,
            RESULT_BACKPRESSURE,
            RESULT_UNSUPPORTED,
            RESULT_RENDER_REJECTED,
            RESULT_REPREPARE_REQUIRED,
            RESULT_INTERNAL,
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255]
    );
    assert_eq!(
        [
            STATE_CONFIG,
            STATE_PREPARED,
            STATE_READY,
            STATE_FAILED,
            STATE_DISPOSED
        ],
        [0, 1, 2, 3, 4]
    );
    assert_eq!([BACKEND_SCALAR, BACKEND_SIMD128], [0, 1]);
    assert_eq!(
        [
            BUFFER_SESSION_TOML,
            BUFFER_SOURCE_ID,
            BUFFER_SOURCE_PCM,
            BUFFER_DIAGNOSTIC,
            BUFFER_OUTPUT_PCM
        ],
        [1, 2, 3, 4, 5]
    );
    assert_eq!(offset_of!(WebPrepareConfigV1, struct_size), 0);
    assert_eq!(offset_of!(WebPrepareConfigV1, quantum_frames), 12);
    assert_eq!(offset_of!(WebPrepareConfigV1, maximum_tracks), 40);
    assert_eq!(offset_of!(WebPrepareConfigV1, maximum_meter_bytes), 152);
    assert_eq!(offset_of!(WebPrepareConfigV1, reserved), 160);
    assert_eq!(offset_of!(WebStatusV1, state), 8);
    assert_eq!(offset_of!(WebStatusV1, next_absolute_sample), 32);
    assert_eq!(offset_of!(WebStatusV1, reserved), 48);
    assert_eq!(offset_of!(WebResourceReportV1, config_bytes), 32);
    assert_eq!(
        offset_of!(WebResourceReportV1, largest_named_allocation_bytes),
        184
    );
    assert_eq!(offset_of!(WebResourceReportV1, reserved), 192);
}

#[test]
fn raw_ffi_validates_handle_layout_overflow_and_transactional_failure() {
    assert_eq!(miso_engine_web_v1_abi_version(), ABI_VERSION);
    assert_eq!(miso_engine_web_v1_config_bytes(), PREPARE_CONFIG_BYTES);
    assert_eq!(miso_engine_web_v1_prepare(0), RESULT_INVALID_ARGUMENT);
    assert_eq!(miso_engine_web_v1_dispose(0), RESULT_OK);

    let handle = miso_engine_web_v1_config_new();
    assert_ne!(handle, 0);
    assert_eq!(miso_engine_web_v1_config_new(), 0);
    let mut overflow = WebPrepareConfigV1::launch_defaults(48_000, 256);
    overflow.maximum_source_channels = u32::MAX;
    assert_eq!(crate::ffi::test_configure(handle, overflow), RESULT_OK);
    assert_eq!(miso_engine_web_v1_prepare(handle), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        crate::ffi::test_status(handle).expect("status").state,
        STATE_CONFIG
    );
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);

    let handle = miso_engine_web_v1_config_new();
    let config = WebPrepareConfigV1::launch_defaults(48_000, 128);
    assert_eq!(crate::ffi::test_configure(handle, config), RESULT_OK);
    assert_eq!(miso_engine_web_v1_prepare(handle), RESULT_OK);
    assert_eq!(
        miso_engine_web_v1_buffer_capacity(handle, BUFFER_OUTPUT_PCM),
        2 * 128 * 4
    );
    assert_ne!(
        crate::ffi::test_buffer_address(handle, BUFFER_DIAGNOSTIC),
        0
    );
    assert_eq!(
        crate::ffi::test_copy_staging(handle, BUFFER_SESSION_TOML, b"no="),
        RESULT_OK
    );
    assert_eq!(
        miso_engine_web_v1_compile(handle, 3),
        RESULT_PREPARE_REJECTED
    );
    assert_eq!(
        crate::ffi::test_status(handle).expect("status").state,
        STATE_FAILED
    );
    assert_ne!(
        crate::ffi::test_buffer_address(handle, BUFFER_DIAGNOSTIC),
        0
    );
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_prepare(handle), RESULT_INVALID_ARGUMENT);
}

#[test]
fn raw_ffi_uses_stable_staging_and_exact_output_quantum_without_growth() {
    let quantum = 64_u32;
    let toml = one_track_session(quantum);
    let handle = miso_engine_web_v1_config_new();
    assert_ne!(handle, 0);
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, quantum);
    config.source_ring_frames = quantum;
    assert_eq!(crate::ffi::test_configure(handle, config), RESULT_OK);
    let status_address = crate::ffi::test_status_address(handle);
    let resource_address = crate::ffi::test_resource_address(handle);
    assert_eq!(miso_engine_web_v1_prepare(handle), RESULT_OK);
    let addresses = [
        BUFFER_SESSION_TOML,
        BUFFER_SOURCE_ID,
        BUFFER_SOURCE_PCM,
        BUFFER_DIAGNOSTIC,
        BUFFER_OUTPUT_PCM,
    ]
    .map(|kind| crate::ffi::test_buffer_address(handle, kind));
    assert!(addresses.into_iter().all(|address| address != 0));
    assert_eq!(
        crate::ffi::test_copy_staging(handle, BUFFER_SESSION_TOML, toml.as_bytes()),
        RESULT_OK
    );
    assert_eq!(
        miso_engine_web_v1_compile(handle, toml.len() as u32),
        RESULT_OK
    );
    assert_eq!(status_address, crate::ffi::test_status_address(handle));
    assert_eq!(resource_address, crate::ffi::test_resource_address(handle));
    assert_eq!(
        crate::ffi::test_copy_staging(handle, BUFFER_SOURCE_ID, b"fixture-source"),
        RESULT_OK
    );
    assert_eq!(crate::ffi::test_fill_source_pcm(handle, 0.25), RESULT_OK);
    assert_eq!(
        miso_engine_web_v1_source_submit(handle, 14, 1, 0, 2, quantum, 0),
        RESULT_OK
    );
    assert_eq!(
        miso_engine_web_v1_source_submit(handle, 14, 1, u64::from(quantum), 2, quantum, 1),
        RESULT_BACKPRESSURE
    );
    assert_eq!(miso_engine_web_v1_render(handle, quantum), RESULT_OK);
    assert_eq!(
        miso_engine_web_v1_source_submit(handle, 14, 1, u64::from(quantum), 2, quantum, 1),
        RESULT_OK
    );
    assert_eq!(miso_engine_web_v1_source_seek(handle, 14, 2, 0), RESULT_OK);
    assert_eq!(miso_engine_web_v1_render(handle, quantum), RESULT_OK);
    let after = [
        BUFFER_SESSION_TOML,
        BUFFER_SOURCE_ID,
        BUFFER_SOURCE_PCM,
        BUFFER_DIAGNOSTIC,
        BUFFER_OUTPUT_PCM,
    ]
    .map(|kind| crate::ffi::test_buffer_address(handle, kind));
    assert_eq!(addresses, after);
    let status = crate::ffi::test_status(handle).expect("status");
    assert_eq!(status.rendered_quanta, 2);
    assert_eq!(status.next_absolute_sample, u64::from(quantum) * 2);
    assert_eq!(
        miso_engine_web_v1_render(handle, 0),
        RESULT_REPREPARE_REQUIRED
    );
    let mismatch = crate::ffi::test_status(handle).expect("status");
    assert_eq!(mismatch.state, STATE_FAILED);
    assert_eq!(mismatch.rendered_quanta, 2);
    assert_eq!(mismatch.next_absolute_sample, u64::from(quantum) * 2);
    let resources = crate::ffi::test_resources(handle).expect("resources");
    assert!(resources.bridge_retained_bytes <= config.maximum_host_retained_bytes);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_INVALID_ARGUMENT);
    let replacement = miso_engine_web_v1_config_new();
    assert_ne!(replacement, 0);
    assert_ne!(replacement, handle);
    assert_eq!(miso_engine_web_v1_dispose(replacement), RESULT_OK);
}

#[test]
fn preparation_accepts_explicit_64_128_and_256_quanta_with_stable_buffers() {
    for quantum in [64, 128, 256] {
        let mut host = prepared_host(quantum);
        assert_eq!(host.status().state, STATE_PREPARED);
        let toml_ptr = host.session_toml_mut().expect("TOML").as_ptr();
        let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
        let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
        let output_ptr = host.output_pcm().expect("output").as_ptr();
        assert_eq!(
            host.source_pcm_mut().expect("PCM").len(),
            8 * quantum as usize
        );
        assert_eq!(
            host.output_pcm().expect("output").len(),
            2 * quantum as usize
        );
        assert_eq!(toml_ptr, host.session_toml_mut().expect("TOML").as_ptr());
        assert_eq!(source_id_ptr, host.source_id_mut().expect("ID").as_ptr());
        assert_eq!(source_pcm_ptr, host.source_pcm_mut().expect("PCM").as_ptr());
        assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    }
}

#[test]
fn malformed_config_and_atomic_compile_failure_are_sticky() {
    let mut bad = WebPrepareConfigV1::launch_defaults(48_000, 128);
    bad.abi_version = 0;
    let mut host = AudioWorkletEngineHost::new(bad);
    assert_eq!(host.prepare(), RESULT_ABI_MISMATCH);
    assert_eq!(host.status().state, STATE_CONFIG);

    let mut host = prepared_host(128);
    host.session_toml_mut().expect("TOML")[..3].copy_from_slice(b"no=");
    assert_eq!(host.compile(3), RESULT_PREPARE_REJECTED);
    assert_eq!(host.status().state, STATE_FAILED);
    assert!(!host.diagnostic().is_empty());
    assert_eq!(host.compile(3), RESULT_WRONG_STATE);
}

#[test]
fn compile_resource_caps_are_inclusive_and_one_below_rejects() {
    let reference = ready_host(128);
    let expected_host = reference.resources().bridge_retained_bytes;
    let expected_named = reference.resources().largest_named_allocation_bytes;
    drop(reference);

    let toml = one_track_session(128);
    for (host_cap, named_cap, prepare_result, compile_result) in [
        (expected_host, expected_named, RESULT_OK, RESULT_OK),
        (
            expected_host - 1,
            expected_named,
            RESULT_OK,
            RESULT_PREPARE_REJECTED,
        ),
        (
            expected_host,
            expected_named - 1,
            RESULT_PREPARE_REJECTED,
            RESULT_WRONG_STATE,
        ),
    ] {
        let mut config = WebPrepareConfigV1::launch_defaults(48_000, 128);
        config.maximum_host_retained_bytes = host_cap;
        config.maximum_named_allocation_bytes = named_cap;
        let mut host = AudioWorkletEngineHost::new(config);
        assert_eq!(host.prepare(), prepare_result);
        if prepare_result == RESULT_OK {
            host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
        }
        assert_eq!(
            host.compile(toml.len()),
            compile_result,
            "diagnostic={:?} host_cap={host_cap} named_cap={named_cap}",
            host.diagnostic()
        );
        assert_eq!(
            host.status().state,
            if compile_result == RESULT_OK {
                STATE_READY
            } else if prepare_result == RESULT_OK {
                STATE_FAILED
            } else {
                STATE_CONFIG
            }
        );
    }
}

#[test]
fn source_backpressure_seek_render_and_stable_output_are_bounded() {
    let quantum = 128_usize;
    let toml = one_track_session(quantum as u32);
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, quantum as u32);
    config.source_ring_frames = quantum as u32;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    let toml_ptr = host.session_toml_mut().expect("TOML").as_ptr();
    let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
    let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
    let output_ptr = host.output_pcm().expect("output").as_ptr();
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );
    assert_eq!(toml_ptr, host.session_toml_mut().expect("TOML").as_ptr());
    assert_eq!(source_id_ptr, host.source_id_mut().expect("ID").as_ptr());
    assert_eq!(source_pcm_ptr, host.source_pcm_mut().expect("PCM").as_ptr());
    assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    let left = vec![0.25_f32; quantum];
    let right = vec![-0.5_f32; quantum];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            0,
            48_000,
            &planes,
            quantum as u32,
            false
        ),
        RESULT_OK
    );
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            true
        ),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.status().next_absolute_sample, quantum as u64);
    assert_eq!(host.status().rendered_quanta, 1);
    assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            true
        ),
        RESULT_OK
    );
    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    assert_eq!(
        host.seek_source(b"fixture-source", 3, 0),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.status().rendered_quanta, 2);
}

#[test]
fn output_mismatch_and_disposal_are_sticky_and_idempotent() {
    let mut host = ready_host(64);
    assert_eq!(host.reject_output_quantum(128), RESULT_REPREPARE_REQUIRED);
    assert_eq!(host.status().state, STATE_FAILED);
    assert_eq!(host.render_next(), RESULT_WRONG_STATE);
    assert_eq!(host.dispose(), RESULT_OK);
    assert_eq!(host.dispose(), RESULT_OK);
    assert_eq!(host.status().state, STATE_DISPOSED);
    assert!(host.output_pcm().is_none());
}

/// #106 F2. A render failure is sticky, silent, and keeps every allocation alive.
///
/// `PreparedRenderPlan::render` has exactly one failure mode reachable with valid buffers: the
/// `checked_add` on the block start time (`RenderError::TimeOverflow`). Driving it proves the
/// property the call-graph gate asserts statically — nothing reachable from `render_next` frees.
///
/// Red mutation: restore `self.ready = None;` as the first line of `fail` → the
/// `host.ready.is_some()` assertion fails.
#[test]
fn render_failure_retains_ownership_and_silences() {
    let mut host = ready_host(128);
    host.buffers
        .as_mut()
        .expect("prepared buffers")
        .output_pcm
        .fill(-1.0);
    host.status.next_absolute_sample = u64::MAX;

    assert_eq!(host.render_next(), RESULT_RENDER_REJECTED);
    assert!(
        host.ready.is_some(),
        "a render failure must never drop the plan, session or source rings on the audio thread"
    );
    assert_eq!(host.status().state, STATE_FAILED);
    assert!(
        host.output_pcm()
            .expect("prepared output")
            .iter()
            .all(|sample| sample.to_bits() == 0),
        "a failed render emits positive-zero silence"
    );
    assert_eq!(host.diagnostic(), b"web.render.rejected\t$\n");

    assert_eq!(host.render_next(), RESULT_WRONG_STATE);
    assert!(
        host.ready.is_some(),
        "the retirement slot survives re-entry"
    );

    assert_eq!(host.dispose(), RESULT_OK);
    assert!(
        host.ready.is_none(),
        "dispose is the single control-path reclamation point"
    );
    assert_eq!(host.status().state, STATE_DISPOSED);
}

/// #106 F1. Every source rule the browser host applies is now the facade's, in the facade's order.
///
/// Before the facade this host carried its own copy of these checks in its own order, and it had
/// already diverged: it built `SourceGeneration(generation)` directly, so generation `0` -- which
/// is reserved -- reached the ring instead of being named. The browser ABI collapses every
/// malformed submission to `RESULT_INVALID_ARGUMENT`, so that particular divergence is not
/// observable through the result code alone; what it bought is the single typed vocabulary, whose
/// seventeen variants #103's `source_diagnostics.rs` pins one at a time.
///
/// What *is* observable here is the order: end-of-region symmetry is decided before the ring is
/// offered the chunk, so a chunk that ends exactly at the region end without the flag is reported
/// as malformed rather than as bounded backpressure.
///
/// Red mutation (proven): delete the `end_of_region != (end == region_end)` check from
/// `SourceControlSet::submit` -> that submission returns `RESULT_BACKPRESSURE` (6) instead of
/// `RESULT_INVALID_ARGUMENT` (1) and this test fails.
#[test]
fn facade_source_rules_reach_the_browser_host() {
    let quantum = 128_usize;
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, quantum as u32);
    config.source_ring_frames = quantum as u32;
    let toml = one_track_session(quantum as u32);
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );

    let left = vec![0.25_f32; quantum];
    let right = vec![-0.5_f32; quantum];
    let planes: [&[f32]; 2] = [&left, &right];
    let submit = |host: &mut AudioWorkletEngineHost, id: &[u8], generation, start, rate, n| {
        host.submit_source(id, generation, start, rate, &planes, n, false)
    };

    // The divergence this job closed: generation 0 is reserved and is now rejected in Rust.
    let mut host = host;
    assert_eq!(
        submit(&mut host, b"fixture-source", 0, 0, 48_000, quantum as u32),
        RESULT_INVALID_ARGUMENT,
        "generation 0 is reserved and never reaches the ring as a valid tag"
    );
    assert_eq!(
        submit(&mut host, b"absent-source", 1, 0, 48_000, quantum as u32),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        submit(&mut host, b"fixture-source", 1, 0, 44_100, quantum as u32),
        RESULT_INVALID_ARGUMENT,
        "the chunk rate must equal the declared source rate"
    );
    assert_eq!(
        submit(
            &mut host,
            b"fixture-source",
            1,
            1 << 40,
            48_000,
            quantum as u32
        ),
        RESULT_INVALID_ARGUMENT,
        "a chunk outside the mapped region is refused"
    );
    // The staging bound is still the host's: a chunk longer than one quantum could not have been
    // staged by the JavaScript side.
    assert_eq!(
        submit(
            &mut host,
            b"fixture-source",
            1,
            0,
            48_000,
            quantum as u32 + 1
        ),
        RESULT_INVALID_ARGUMENT
    );
    // A valid chunk still succeeds.
    assert_eq!(
        submit(&mut host, b"fixture-source", 1, 0, 48_000, quantum as u32),
        RESULT_OK
    );
    // End-of-region symmetry is checked before the ring is offered the chunk: this chunk ends
    // exactly at the region end, so `end_of_region = false` is the first rule it breaks.
    assert_eq!(
        submit(
            &mut host,
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            quantum as u32
        ),
        RESULT_INVALID_ARGUMENT
    );
    // Correctly flagged, it reaches the one-quantum ring and reports bounded backpressure.
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            true
        ),
        RESULT_BACKPRESSURE
    );
    assert_eq!(
        host.seek_source(b"fixture-source", 0, 0),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    assert_eq!(host.status().state, STATE_READY, "no rejection is sticky");
}
