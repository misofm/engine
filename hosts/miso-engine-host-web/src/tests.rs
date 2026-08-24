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

/// The browser fixture's identity session, re-shaped for one test.
///
/// Identity end to end: no polarity, trim, HPF or LPF, no effects in any rack, unity fader, and a
/// hard-left/hard-right pan whose 2x2 matrix is the identity. The output is therefore the submitted
/// source frames, which is what makes the submitted ramp its own oracle.
fn identity_session(quantum: u32, ring_frames: u32, length_samples: u64) -> String {
    let mut model = parse_session_toml(include_str!("../tests/browser-v1/session.toml"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    model.limits.pcm_ring_frames = u64::from(ring_frames);
    model.sources[0].mapping.region.length_samples = length_samples;
    canonical_session_toml(&model).expect("canonical identity session")
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
            BUFFER_OUTPUT_PCM,
            BUFFER_COMMAND,
            BUFFER_METER_FRAME
        ],
        [1, 2, 3, 4, 5, 6, 7]
    );
    // Issue #137 D1: the two console words are the first two of the frozen configuration's four
    // reserved words. Every V1 writer already sets them to zero, which is exactly "default command
    // queue depth, no meters attached", so the 192-byte layout and every existing caller stand.
    assert_eq!(size_of::<WebCommandReportV1>(), 48);
    assert_eq!(COMMAND_REPORT_BYTES, 48);
    assert_eq!(COMMAND_RECORD_BYTES, 48);
    assert_eq!(MAXIMUM_COMMAND_RECORDS, 256);
    assert_eq!(DEFAULT_COMMAND_QUEUE_RECORDS, 64);
    assert_eq!(DEFAULT_METER_BLOCKS, 12);
    assert_eq!(
        [
            COMMAND_PAN,
            COMMAND_MATRIX,
            COMMAND_FADER_DB,
            COMMAND_MUTE,
            COMMAND_EFFECT_PARAM,
            COMMAND_EFFECT_BYPASS
        ],
        [1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        [
            COMMAND_REASON_NONE,
            COMMAND_REASON_MALFORMED,
            COMMAND_REASON_UNKNOWN_TRACK,
            COMMAND_REASON_UNKNOWN_RACK,
            COMMAND_REASON_UNKNOWN_EFFECT,
            COMMAND_REASON_UNKNOWN_PARAMETER,
            COMMAND_REASON_DOMAIN,
            COMMAND_REASON_UNSUPPORTED_KIND,
            COMMAND_REASON_BACKPRESSURE,
            COMMAND_REASON_WRONG_STATE
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
    assert_eq!(offset_of!(WebPrepareConfigV1, struct_size), 0);
    assert_eq!(offset_of!(WebPrepareConfigV1, quantum_frames), 12);
    assert_eq!(offset_of!(WebPrepareConfigV1, maximum_tracks), 40);
    assert_eq!(offset_of!(WebPrepareConfigV1, maximum_meter_bytes), 152);
    assert_eq!(
        offset_of!(WebPrepareConfigV1, console_command_queue_records),
        160
    );
    assert_eq!(offset_of!(WebPrepareConfigV1, console_meter_blocks), 168);
    assert_eq!(offset_of!(WebPrepareConfigV1, reserved), 176);
    assert_eq!(offset_of!(WebCommandReportV1, result), 8);
    assert_eq!(offset_of!(WebCommandReportV1, rejected_index), 16);
    assert_eq!(offset_of!(WebCommandReportV1, applied_at_sample), 24);
    assert_eq!(offset_of!(WebCommandReportV1, reserved), 32);
    assert_eq!(offset_of!(WebStatusV1, state), 8);
    assert_eq!(offset_of!(WebStatusV1, next_absolute_sample), 32);
    assert_eq!(offset_of!(WebStatusV1, reserved), 48);
    assert_eq!(offset_of!(WebResourceReportV1, config_bytes), 32);
    assert_eq!(
        offset_of!(WebResourceReportV1, largest_named_allocation_bytes),
        184
    );
    assert_eq!(offset_of!(WebResourceReportV1, reserved), 192);

    // Issue #137: `bridgeMetadataBytes` in `tests/browser-v1/expected.json` is not a magic number
    // and never was. It is exactly this formula over the host shell, so when the shell grows the
    // pinned row moves by exactly that growth and by nothing else -- which is how the two rows
    // that moved for #137 (`bridgeMetadataBytes`, `bridgeRetainedBytes`, both +152 on wasm32) were
    // derived rather than read off a run.
    let host = prepared_host(128);
    let plane_references = 8 * size_of::<&[f32]>() as u64;
    assert_eq!(
        host.resources().bridge_metadata_bytes,
        size_of::<AudioWorkletEngineHost>() as u64
            - u64::from(PREPARE_CONFIG_BYTES)
            - u64::from(STATUS_BYTES)
            + plane_references,
        "the bridge metadata row is the host shell minus the two structures charged separately"
    );
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

/// #106 F3. The pinned default ring capacity at every launch rate and a spread of quanta.
///
/// The oracle is the derivation in [`default_source_ring_frames`]'s documentation, evaluated by
/// hand: `(ceil(100 ms * fs / quantum) + 2) * quantum`.
///
/// Red mutation: change the `+ 2` to `+ 1` -> 48 000/128 yields 4 992, not 5 120.
#[test]
fn default_ring_covers_stall_tolerance() {
    assert_eq!(SOURCE_STALL_TOLERANCE_MS, 100);
    for (sample_rate_hz, quantum, expected) in [
        (48_000_u32, 128_u32, 5_120_u32),
        (44_100, 128, 4_736),
        (88_200, 128, 9_088),
        (96_000, 128, 9_856),
        (48_000, 256, 5_376),
        (48_000, 64, 4_928),
    ] {
        let frames = default_source_ring_frames(sample_rate_hz, quantum);
        assert_eq!(frames, expected, "{sample_rate_hz} Hz / {quantum} frames");
        assert_eq!(
            frames % quantum,
            0,
            "a ring capacity is a whole number of quanta"
        );
        let stall_frames = u64::from(sample_rate_hz) * u64::from(SOURCE_STALL_TOLERANCE_MS) / 1000;
        assert!(
            u64::from(frames) >= stall_frames + 2 * u64::from(quantum),
            "the ring must cover the stall plus the consumer and recycle quanta"
        );
        assert_eq!(
            WebPrepareConfigV1::launch_defaults(sample_rate_hz, quantum).source_ring_frames,
            expected,
            "launch defaults are the only place the formula is applied"
        );
    }
}

/// #106 F3. A full default ring renders the whole stall tolerance with no submission at all.
///
/// The identity session passes source frames to the output unchanged, so the oracle is the
/// submitted ramp itself -- compared as `to_bits`, never as floats, because `==` equates `-0.0`
/// with `+0.0`. Filling until backpressure and then rendering 38 quanta in silence is exactly the
/// 100 ms main-thread stall the default ring exists to hide.
///
/// Red mutation: set `SOURCE_STALL_TOLERANCE_MS = 50` (a 21-quantum ring) -> the ring runs dry and
/// the first starved quantum renders zeros instead of the ramp.
#[test]
fn ring_prefill_survives_stall() {
    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    let ring_frames = default_source_ring_frames(RATE, QUANTUM);
    let stall_quanta = (u64::from(SOURCE_STALL_TOLERANCE_MS) * u64::from(RATE) / 1000)
        .div_ceil(u64::from(QUANTUM)) as u32;
    assert_eq!(stall_quanta, 38, "100 ms at 48 kHz / 128 is 38 quanta");

    let length_samples = u64::from(ring_frames) + 2 * u64::from(QUANTUM);
    let toml = identity_session(QUANTUM, ring_frames, length_samples);
    let mut host = AudioWorkletEngineHost::new(WebPrepareConfigV1::launch_defaults(RATE, QUANTUM));
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );

    // A distinct value per absolute frame, so a stale or repeated block cannot pass by accident.
    let ramp = |block: u32, index: u32| (block * QUANTUM + index) as f32 / 65_536.0;
    let mut submitted: Vec<Vec<f32>> = Vec::new();
    loop {
        let block = submitted.len() as u32;
        let left: Vec<f32> = (0..QUANTUM).map(|index| ramp(block, index)).collect();
        let right = vec![0.0_f32; QUANTUM as usize];
        let planes: [&[f32]; 2] = [&left, &right];
        let start = u64::from(block) * u64::from(QUANTUM);
        let end_of_region = start + u64::from(QUANTUM) == length_samples;
        let result = host.submit_source(
            b"fixture-source",
            1,
            start,
            RATE,
            &planes,
            QUANTUM,
            end_of_region,
        );
        if result == RESULT_BACKPRESSURE {
            break;
        }
        assert_eq!(result, RESULT_OK, "block {block}");
        submitted.push(left);
        assert!(
            submitted.len() < 128,
            "the ring must saturate, not grow forever"
        );
    }
    assert!(
        submitted.len() as u32 >= stall_quanta,
        "a full default ring holds at least the stall tolerance: {} < {stall_quanta}",
        submitted.len()
    );

    // The stall: 38 quanta rendered with no submission whatsoever.
    for (block, expected) in submitted.iter().enumerate().take(stall_quanta as usize) {
        assert_eq!(host.render_next(), RESULT_OK, "block {block}");
        let output = host.output_pcm().expect("prepared output");
        let left = &output[..QUANTUM as usize];
        for (index, (sample, want)) in left.iter().zip(expected).enumerate() {
            assert_eq!(
                sample.to_bits(),
                want.to_bits(),
                "block {block} frame {index} underran the stall"
            );
        }
    }
    assert_eq!(host.status().rendered_quanta, u64::from(stall_quanta));
}

/// #106 F4 (as amended by #83 W4-D1), native leg.
///
/// W4-D1 removed the scalar artifact, so the old artifact-level scalar↔simd128 comparison has no
/// second artifact to compare against. Its replacement is two `to_bits` identities: #83's G5 corpus
/// (native Scalar/Simd4/Simd8 against both wasm builds under wasmtime) covers the kernels, and this
/// covers the whole host path -- the same session, the same source transcript, the same three
/// render calls, rendered natively here and through the shipped simd128 artifact by
/// `tests/browser-v1/direct-oracle.mjs`, which asserts its own digest equals the pin this test
/// writes.
///
/// The digest is over little-endian `f32` words, so it is a bit comparison: a float comparison
/// would equate `-0.0` with `+0.0`, and `+0.0` is exactly what a silent or starved block produces.
///
/// Red mutation: change `leftBase` of the first block in `tests/browser-v1/source.json` -> both
/// this test and `direct-oracle.mjs` fail against the pin, and they fail with the same value, which
/// is the point.
#[test]
fn native_identity_session_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;

    // The transcript of `tests/browser-v1/source.json`, replayed exactly as `direct-oracle.mjs`
    // and `browser-correctness.js` replay it.
    let block = |base: f32, step: f32| -> Vec<f32> {
        (0..QUANTUM)
            .map(|index| base + step * index as f32)
            .collect()
    };
    let first = block(0.125, 0.0009765625);
    let second = block(-0.25, 0.00048828125);
    let silent = vec![0.0_f32; QUANTUM as usize];

    let toml = identity_session(QUANTUM, QUANTUM, 256);
    let mut host = AudioWorkletEngineHost::new(WebPrepareConfigV1::launch_defaults(RATE, QUANTUM));
    // The browser fixture pins a one-quantum ring, which is what makes its backpressure
    // observable; the default ring would swallow the second submission.
    host.config_mut().expect("config state").source_ring_frames = QUANTUM;
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        host.diagnostic()
    );

    let submit = |host: &mut AudioWorkletEngineHost, left: &[f32], generation, start, last| {
        let planes: [&[f32]; 2] = [left, &silent];
        host.submit_source(
            b"fixture-source",
            generation,
            start,
            RATE,
            &planes,
            QUANTUM,
            last,
        )
    };

    let mut blocks: Vec<Vec<f32>> = Vec::new();
    let mut capture = |host: &AudioWorkletEngineHost| {
        blocks.push(host.output_pcm().expect("prepared output").to_vec());
    };

    assert_eq!(submit(&mut host, &first, 1, 0, false), RESULT_OK);
    assert_eq!(
        submit(&mut host, &second, 1, u64::from(QUANTUM), true),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    capture(&host);
    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    assert_eq!(submit(&mut host, &first, 2, 0, false), RESULT_OK);
    assert_eq!(
        submit(&mut host, &second, 2, u64::from(QUANTUM), true),
        RESULT_BACKPRESSURE
    );
    assert_eq!(host.render_next(), RESULT_OK);
    capture(&host);
    assert_eq!(
        submit(&mut host, &second, 2, u64::from(QUANTUM), true),
        RESULT_OK
    );
    assert_eq!(host.render_next(), RESULT_OK);
    capture(&host);

    // Left plane of every block, then right plane of every block: the oracle's channel order.
    let mut digest = Sha256::new();
    for channel in 0..2_usize {
        for output in &blocks {
            let plane = &output[channel * QUANTUM as usize..(channel + 1) * QUANTUM as usize];
            for sample in plane {
                digest.update(sample.to_bits().to_le_bytes());
            }
        }
    }
    let native = digest
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut text, byte| {
            use core::fmt::Write as _;
            let _ = write!(&mut text, "{byte:02x}");
            text
        });

    let expected: serde_pin::Pin =
        serde_pin::read(include_str!("../tests/browser-v1/expected.json"));
    assert_eq!(
        native, expected.native,
        "native and the pinned wasm digest must agree bit for bit; \
         if this moved, the signal path moved"
    );
    assert_eq!(
        native, expected.simd128,
        "the shipped simd128 artifact renders this session to the same bits as native"
    );
}

/// #137 E2, native leg: the same session, source feed and command timeline the shipped artifact
/// runs in `tests/browser-v1/direct-oracle.mjs`, rendered natively here.
///
/// The fixture is identity end to end, so a constant input renders to that same constant and the
/// only thing that can move a sample is a command. Six blocks, one matrix retarget, one refused
/// unknown-track record, one refused flood, one refused unsupported kind, and one smoothed pan
/// retarget: the digest is a statement about *when* each of those took effect, not merely that
/// they did. The digest is over little-endian `f32` words, so it is a bit comparison.
///
/// Red mutation: change the matrix retarget's `applied_at_sample` expectation to `2 * QUANTUM`
/// -> the assertion fails here, and moving the drain in `ConsoleMatrixProcessor::process` to after
/// the audio makes both this digest and the wasm oracle's move together, which is the point.
#[test]
fn native_command_timeline_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    const DEPTH: u32 = 4;

    // The fixture file is read verbatim, exactly as `direct-oracle.mjs` reads it, so both legs
    // compile byte-identical input. It is the browser identity session with a six-block region.
    let toml = include_str!("../tests/browser-v1/command-session.toml");
    let mut config = WebPrepareConfigV1::launch_defaults(RATE, QUANTUM);
    config.source_ring_frames = QUANTUM;
    config.console_command_queue_records = u64::from(DEPTH);
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        core::str::from_utf8(host.diagnostic())
    );
    assert_eq!(host.console_tracks().len(), 1);

    let plane = vec![0.25_f32; QUANTUM as usize];
    let mut blocks: Vec<Vec<f32>> = Vec::new();
    let mut step = |host: &mut AudioWorkletEngineHost, block: u64| {
        let planes: [&[f32]; 2] = [&plane, &plane];
        assert_eq!(
            host.submit_source(
                b"fixture-source",
                1,
                block * u64::from(QUANTUM),
                RATE,
                &planes,
                QUANTUM,
                false,
            ),
            RESULT_OK,
        );
        assert_eq!(host.render_next(), RESULT_OK);
        blocks.push(host.output_pcm().expect("prepared output").to_vec());
    };

    let matrix = |host: &mut AudioWorkletEngineHost, index: usize, track: u32| {
        stage_command(
            host,
            index,
            COMMAND_MATRIX,
            255,
            255,
            track,
            0,
            0,
            0,
            [0.5, 0.0, 0.0, 1.0],
        );
    };

    step(&mut host, 0);
    matrix(&mut host, 0, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(host.command_report().applied_at_sample, u64::from(QUANTUM));
    step(&mut host, 1);

    matrix(&mut host, 0, 5);
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TRACK);
    for index in 0..DEPTH as usize + 1 {
        matrix(&mut host, index, 0);
    }
    assert_eq!(host.submit_commands(DEPTH + 1), RESULT_BACKPRESSURE);
    assert_eq!(host.command_report().admitted, 0);
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        0,
        0,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_UNSUPPORTED);
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_UNSUPPORTED_KIND
    );
    step(&mut host, 2);

    stage_command(
        &mut host,
        0,
        COMMAND_PAN,
        255,
        255,
        0,
        0,
        0,
        QUANTUM,
        [-1.0, 1.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        3 * u64::from(QUANTUM)
    );
    step(&mut host, 3);
    step(&mut host, 4);
    step(&mut host, 5);
    assert_eq!(host.status().rendered_quanta, 6);

    let mut digest = Sha256::new();
    for channel in 0..2_usize {
        for output in &blocks {
            let plane = &output[channel * QUANTUM as usize..(channel + 1) * QUANTUM as usize];
            for sample in plane {
                digest.update(sample.to_bits().to_le_bytes());
            }
        }
    }
    let native = digest
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut text, byte| {
            use core::fmt::Write as _;
            let _ = write!(&mut text, "{byte:02x}");
            text
        });

    let expected: serde_pin::Pin =
        serde_pin::read(include_str!("../tests/browser-v1/expected.json"));
    assert_eq!(
        native, expected.native_command_timeline,
        "native and the pinned wasm command-timeline digest must agree bit for bit"
    );
    assert_eq!(
        native, expected.simd128_command_timeline,
        "the shipped simd128 artifact renders this command timeline to the same bits as native"
    );
}

/// A three-field reader for `expected.json`, so the test needs no JSON dependency.
///
/// The file is generated by `direct-oracle.mjs` and is machine-formatted, so the two hashes are
/// found by their key names rather than by position.
mod serde_pin {
    pub(super) struct Pin {
        pub(super) native: String,
        pub(super) simd128: String,
        pub(super) native_command_timeline: String,
        pub(super) simd128_command_timeline: String,
    }

    fn hex_after(text: &str, key: &str) -> String {
        let start = text
            .find(key)
            .unwrap_or_else(|| panic!("expected.json has no {key}"))
            + key.len();
        let rest = &text[start..];
        let open = rest.find('"').expect("value opens") + 1;
        let close = rest[open..].find('"').expect("value closes") + open;
        rest[open..close].to_owned()
    }

    pub(super) fn read(text: &str) -> Pin {
        // `pcmF32leSha256` appears once per oracle leg, in file order: the frozen render
        // transcript first, then the #137 command timeline.
        let timeline = text
            .find("\"commandTimeline\"")
            .expect("expected.json has no commandTimeline");
        Pin {
            native: hex_after(text, "\"nativePcmF32leSha256\":"),
            simd128: hex_after(text, "\"pcmF32leSha256\":"),
            native_command_timeline: hex_after(text, "\"nativeCommandTimelinePcmF32leSha256\":"),
            simd128_command_timeline: hex_after(&text[timeline..], "\"pcmF32leSha256\":"),
        }
    }
}

/// Stage one `miso.command.v1` record into the fixed command buffer at `index`.
#[allow(clippy::too_many_arguments)]
fn stage_command(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    kind: u32,
    rack: u8,
    channel: u8,
    track_index: u32,
    effect_index: u32,
    parameter_id: u32,
    smoothing_samples: u32,
    values: [f32; 4],
) {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    let staging = host
        .command_staging_mut()
        .expect("prepared command staging");
    let record = &mut staging[index * record_bytes..(index + 1) * record_bytes];
    record.fill(0);
    record[0] = u8::try_from(kind).expect("frozen kind");
    record[1] = rack;
    record[2] = channel;
    record[4..8].copy_from_slice(&track_index.to_le_bytes());
    record[8..12].copy_from_slice(&effect_index.to_le_bytes());
    record[12..16].copy_from_slice(&parameter_id.to_le_bytes());
    record[16..20].copy_from_slice(&smoothing_samples.to_le_bytes());
    for (slot, value) in values.iter().enumerate() {
        record[24 + slot * 4..28 + slot * 4].copy_from_slice(&value.to_le_bytes());
    }
}

/// A console host over the browser identity fixture: one track, unity everything, one-quantum ring.
fn console_host(quantum: u32, meter_blocks: u64) -> AudioWorkletEngineHost {
    let toml = identity_session(quantum, quantum, u64::from(quantum) * 64);
    let mut config = WebPrepareConfigV1::console_defaults(48_000, quantum);
    config.source_ring_frames = quantum;
    config.console_meter_blocks = meter_blocks;
    config.maximum_meter_streams = 16;
    config.maximum_meter_items = 1 << 16;
    config.maximum_meter_bytes = 1 << 24;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        core::str::from_utf8(host.diagnostic())
    );
    host
}

/// Feed one full quantum of a constant left plane and render it.
fn feed_and_render(host: &mut AudioWorkletEngineHost, generation: u64, block: u64, value: f32) {
    let quantum = host.config().quantum_frames as usize;
    let left = vec![value; quantum];
    let right = vec![value; quantum];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            generation,
            block * quantum as u64,
            48_000,
            &planes,
            quantum as u32,
            false,
        ),
        RESULT_OK,
    );
    assert_eq!(host.render_next(), RESULT_OK);
}

/// #137 E1: a command's acknowledgement names the exact sample it takes effect at, and the
/// rendered output changes at that sample and not one sample before.
///
/// The fixture is identity end to end, so a constant input renders to that same constant. A
/// `COMMAND_MATRIX` that halves `ll` therefore has exactly one observable consequence: the left
/// plane halves. The test asserts the last block before `applied_at_sample` is untouched and the
/// first block at `applied_at_sample` is fully changed -- the matrix stage drains its queue at the
/// top of the block, so the transition is on a block boundary and is exact, not approximate.
///
/// Red mutation: move the `while let Ok(record) = self.control.try_pop()` drain in
/// `ConsoleMatrixProcessor::process` to *after* `self.matrix.process(block)` -> the reported
/// sample is one block early and `at_applied` still renders the pre-command value.
#[test]
fn command_ack_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut host = console_host(QUANTUM, 0);
    let quantum = QUANTUM as usize;

    feed_and_render(&mut host, 1, 0, 0.5);
    let before = host.output_pcm().expect("output").to_vec();
    assert!(
        before[..quantum].iter().all(|value| *value == 0.5),
        "identity fixture renders its input"
    );

    // Admit the retarget between two blocks; the report names the block that will carry it.
    stage_command(
        &mut host,
        0,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [0.5, 0.0, 0.0, 1.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let report = *host.command_report();
    assert_eq!(report.result, RESULT_OK);
    assert_eq!(report.reason, COMMAND_REASON_NONE);
    assert_eq!(report.admitted, 1);
    assert_eq!(
        report.applied_at_sample,
        u64::from(QUANTUM),
        "the next block is the one that drains the queue"
    );
    assert_eq!(report.applied_at_sample, host.status().next_absolute_sample);

    feed_and_render(&mut host, 1, 1, 0.5);
    let at_applied = host.output_pcm().expect("output").to_vec();
    assert!(
        at_applied[..quantum].iter().all(|value| *value == 0.25),
        "every sample of the block at applied_at_sample carries the new matrix"
    );
    assert!(
        at_applied[quantum..].iter().all(|value| *value == 0.5),
        "the right lane is untouched by an ll-only retarget"
    );

    // A smoothed retarget is still admitted at a block boundary; the ramp starts there.
    stage_command(
        &mut host,
        0,
        COMMAND_PAN,
        255,
        255,
        0,
        0,
        0,
        QUANTUM,
        [-1.0, 1.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        2 * u64::from(QUANTUM)
    );
    feed_and_render(&mut host, 1, 2, 0.5);
    let ramping = host.output_pcm().expect("output").to_vec();
    assert!(
        ramping[0] > 0.25 && ramping[0] < 0.5,
        "the ramp starts inside the block that reported the sample: {}",
        ramping[0]
    );
    assert!(
        (ramping[quantum - 1] - 0.5).abs() < 1e-6,
        "a one-quantum window settles by the end of that block: {}",
        ramping[quantum - 1]
    );
}

/// #137 E3: flooding past the bounded queue is a typed local rejection that admits nothing and
/// disturbs no rendered sample.
///
/// Red mutation: delete the free-room pre-check loop in `admit_commands` -> the flood is admitted
/// record by record until `try_push` fails, the transaction is no longer all-or-nothing, and the
/// flooded run's digest differs from the clean run's.
#[test]
fn command_flood_is_typed_backpressure_and_leaves_the_render_untouched() {
    const QUANTUM: u32 = 128;
    let depth = DEFAULT_COMMAND_QUEUE_RECORDS;

    let mut clean = console_host(QUANTUM, 0);
    let mut flooded = console_host(QUANTUM, 0);
    for block in 0..4_u64 {
        feed_and_render(&mut clean, 1, block, 0.25);
        feed_and_render(&mut flooded, 1, block, 0.25);
        // One more record than the queue can hold, submitted as one batch.
        for index in 0..depth as usize + 1 {
            stage_command(
                &mut flooded,
                index,
                COMMAND_MATRIX,
                255,
                255,
                0,
                0,
                0,
                0,
                [0.5, 0.0, 0.0, 1.0],
            );
        }
        assert_eq!(flooded.submit_commands(depth + 1), RESULT_BACKPRESSURE);
        let report = *flooded.command_report();
        assert_eq!(report.reason, COMMAND_REASON_BACKPRESSURE);
        assert_eq!(report.admitted, 0, "a refused submission admits nothing");
        assert_eq!(
            clean.output_pcm().expect("clean output"),
            flooded.output_pcm().expect("flooded output"),
            "block {block}: control traffic never disturbs a rendered sample",
        );
    }
    assert_eq!(
        clean.status().rendered_quanta,
        flooded.status().rendered_quanta
    );
}

/// #137 E4: every unknown target is a typed refusal that leaves the engine exactly as it was.
///
/// Red mutation: delete the `track >= track_count` leg in `admit_commands` -> the unknown-track
/// record reaches `ready.controls.get(track)`, is refused as `UNSUPPORTED` instead of
/// `INVALID_ARGUMENT`/`UNKNOWN_TRACK`, and the reason assertion below fails.
#[test]
fn unknown_targets_are_typed_and_leave_the_engine_untouched() {
    const QUANTUM: u32 = 128;
    let mut host = console_host(QUANTUM, 0);
    feed_and_render(&mut host, 1, 0, 0.75);
    let baseline = host.output_pcm().expect("output").to_vec();
    let baseline_status = *host.status();

    /// `(kind, rack, channel, track, effect, parameter, values, result, reason)`.
    type UnknownTargetCase = (u32, u8, u8, u32, u32, u32, [f32; 4], u32, u32);
    let cases: [UnknownTargetCase; 7] = [
        // kind, rack, channel, track, effect, parameter, values, result, reason
        (
            COMMAND_MATRIX,
            255,
            255,
            9,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_TRACK,
        ),
        (
            COMMAND_EFFECT_PARAM,
            7,
            2,
            0,
            0,
            1,
            [0.0; 4],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_RACK,
        ),
        (
            COMMAND_EFFECT_PARAM,
            1,
            2,
            0,
            3,
            1,
            [0.0; 4],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_EFFECT,
        ),
        (
            COMMAND_MATRIX,
            255,
            255,
            0,
            0,
            0,
            [2.0, 0.0, 0.0, 1.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            COMMAND_PAN,
            255,
            255,
            0,
            0,
            0,
            [-2.0, 1.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            COMMAND_FADER_DB,
            255,
            0,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
            RESULT_UNSUPPORTED,
            COMMAND_REASON_UNSUPPORTED_KIND,
        ),
        (
            COMMAND_MATRIX,
            0,
            255,
            0,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
    ];
    for (index, case) in cases.into_iter().enumerate() {
        let (kind, rack, channel, track, effect, parameter, values, result, reason) = case;
        stage_command(
            &mut host, 0, kind, rack, channel, track, effect, parameter, 0, values,
        );
        assert_eq!(host.submit_commands(1), result, "case {index}");
        let report = *host.command_report();
        assert_eq!(report.reason, reason, "case {index}");
        assert_eq!(report.admitted, 0, "case {index}");
        assert_eq!(report.rejected_index, 0, "case {index}");
    }
    // An unknown kind byte is malformed, not an unknown target.
    stage_command(&mut host, 0, 0, 255, 255, 0, 0, 0, 0, [0.0; 4]);
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.command_report().reason, COMMAND_REASON_MALFORMED);

    let status = *host.status();
    assert_eq!(
        (
            status.state,
            status.next_absolute_sample,
            status.rendered_quanta
        ),
        (
            baseline_status.state,
            baseline_status.next_absolute_sample,
            baseline_status.rendered_quanta
        ),
        "no refusal moved the state or the clock",
    );
    feed_and_render(&mut host, 1, 1, 0.75);
    assert_eq!(
        host.output_pcm().expect("output"),
        &baseline[..],
        "a refused command changed no coefficient",
    );
}

/// #137 E5: the decimated meter frame equals an offline fold of the rendered PCM, and metering
/// changes no rendered sample.
///
/// The track meter observes the post-matrix boundary and the master peaks are folded over the
/// host's own output plane, so for this identity fixture both must equal the maximum magnitude of
/// the submitted block -- exactly, not within a tolerance.
///
/// Red mutation: change `console_meter_blocks` handling so the period is `blocks` frames instead
/// of `blocks * quantum_frames` -> a window closes mid-block, `poll_meters` reports more windows
/// than blocks rendered, and the cadence assertion fails.
#[test]
fn meter_frames_equal_an_offline_fold_and_cost_the_render_nothing() {
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 2;
    let mut off = console_host(QUANTUM, BLOCKS);
    let mut on = console_host(QUANTUM, BLOCKS);
    assert!(on.meters_attached());
    assert_eq!(on.set_meter_lease(true), RESULT_OK);
    assert_eq!(off.set_meter_lease(false), RESULT_OK);

    let values = [0.25_f32, 0.5, 0.125, 1.0, 0.0625, 0.75];
    let mut windows = 0_u32;
    let mut folded = Vec::new();
    for (block, value) in values.into_iter().enumerate() {
        feed_and_render(&mut off, 1, block as u64, value);
        feed_and_render(&mut on, 1, block as u64, value);
        assert_eq!(
            off.output_pcm().expect("off"),
            on.output_pcm().expect("on"),
            "block {block}: the meter lease changes no rendered sample",
        );
        assert_eq!(off.poll_meters(), 0, "a released lease drains nothing");
        let count = on.poll_meters();
        windows += count;
        if count > 0 {
            folded.push((block, on.meter_frame().to_vec()));
        }
    }
    assert_eq!(
        windows as u64,
        values.len() as u64 / BLOCKS,
        "one frame per {BLOCKS}-block window and no more",
    );
    for (block, frame) in &folded {
        let expected = values[block - 1].max(values[*block]);
        assert_eq!(frame[0], expected, "track left peak at block {block}");
        assert_eq!(frame[1], expected, "track right peak at block {block}");
        assert_eq!(frame[2], expected, "master left peak at block {block}");
        assert_eq!(frame[3], expected, "master right peak at block {block}");
    }

    // A host with no observers refuses the lease rather than reporting zeros.
    let mut bare = console_host(QUANTUM, 0);
    assert!(!bare.meters_attached());
    assert_eq!(bare.set_meter_lease(true), RESULT_UNSUPPORTED);
    assert_eq!(bare.poll_meters(), 0);
}
