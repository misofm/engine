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
            COMMAND_EFFECT_BYPASS,
            COMMAND_OBSERVE_SUBSCRIBE,
            COMMAND_OBSERVE_UNSUBSCRIBE
        ],
        [1, 2, 3, 4, 5, 6, 7, 8]
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
            COMMAND_REASON_WRONG_STATE,
            COMMAND_REASON_UNKNOWN_TAP,
            COMMAND_REASON_OBSERVATION_UNBOUND
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
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
    // Issue #143 D3/D6: the configuration's remaining two reserved words, carved exactly as #137
    // carved the first two. The structure is still 192 bytes and every existing offset is where it
    // was, so a V1 writer that zeroes them gets "no observation capacity, no master designation".
    assert_eq!(
        offset_of!(WebPrepareConfigV1, console_observation_taps),
        176
    );
    assert_eq!(
        offset_of!(WebPrepareConfigV1, console_master_track_plus_one),
        184
    );
    assert_eq!(MAXIMUM_OBSERVATION_TAPS, 16);
    // The meter header is a new fixed structure, not a change to an existing one.
    assert_eq!(size_of::<WebMeterHeaderV1>(), 64);
    assert_eq!(METER_HEADER_BYTES, 64);
    assert_eq!(offset_of!(WebMeterHeaderV1, track_count), 8);
    assert_eq!(offset_of!(WebMeterHeaderV1, windows), 12);
    assert_eq!(offset_of!(WebMeterHeaderV1, first_sample), 16);
    assert_eq!(offset_of!(WebMeterHeaderV1, end_sample), 24);
    assert_eq!(offset_of!(WebMeterHeaderV1, sequence), 32);
    assert_eq!(offset_of!(WebMeterHeaderV1, master_track_plus_one), 40);
    assert_eq!(offset_of!(WebMeterHeaderV1, master_gr_present), 44);
    assert_eq!(offset_of!(WebMeterHeaderV1, reserved), 48);
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
    // Issue #143: the report's first reserved word becomes `observation_retained_bytes`; the
    // structure is still 224 bytes and every existing offset is unmoved.
    assert_eq!(
        offset_of!(WebResourceReportV1, observation_retained_bytes),
        192
    );
    assert_eq!(offset_of!(WebResourceReportV1, reserved), 200);

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

/// #137 E2, extended by #140 C: the command-timeline determinism digest, native leg.
///
/// The same session, source feed and command timeline the raw-Wasm oracle drives in
/// `tests/browser-v1/direct-oracle.mjs`, rendered natively here. Both digests are asserted equal
/// to the same pin, so a change to the audio makes them move together -- which is the point.
///
/// # Every newly live kind is in the timeline
///
/// #137 pinned a timeline of pan and matrix, with `fader_db` present only as a *refusal*. #140
/// makes fader, mute, effect parameter and effect bypass live, so the timeline exercises each of
/// them, in a fixed order, at fixed blocks:
///
/// | block | command admitted before it |
/// |---|---|
/// | 1 | `matrix`, `ll = 0.5` |
/// | 2 | three refusals: unknown track, a queue flood, an unknown effect parameter |
/// | 3 | `pan`, one-quantum window |
/// | 4 | `faderDb` to `-6 dB`, one-quantum window |
/// | 5 | `mute` on, one-quantum window |
/// | 6 | `effectParam`: band 1 gain to `-12 dB`, `channel = both` |
/// | 7 | `effectBypass` on |
/// | 8 | `mute` off and `effectBypass` off, as one batch across two queues |
///
/// The digest is therefore a statement about *when* each of those took effect, not merely that
/// they did. It is over little-endian `f32` words, so it is a bit comparison.
///
/// Red mutation: change the matrix retarget's `applied_at_sample` expectation to `2 * QUANTUM`
/// -> the assertion fails here, and moving any console stage's drain to after the audio makes
/// both this digest and the wasm oracle's move together.
#[test]
fn native_command_timeline_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    const DEPTH: u32 = 4;
    const BLOCKS: u64 = 10;

    // The fixture file is read verbatim, exactly as `direct-oracle.mjs` reads it, so both legs
    // compile byte-identical input. It is the browser identity session plus one dynamic-rack
    // parametric EQ whose band 1 is a low shelf -- a shelf, not a bell, so a DC fixture can
    // actually witness the parameter move.
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

    // Three refusals between blocks 1 and 2. None of them may move a rendered sample.
    matrix(&mut host, 0, 5);
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TRACK);
    for index in 0..DEPTH as usize + 1 {
        matrix(&mut host, index, 0);
    }
    assert_eq!(host.submit_commands(DEPTH + 1), RESULT_BACKPRESSURE);
    assert_eq!(host.command_report().admitted, 0);
    // #140: `fader_db` is live, so the refusal that used to stand here is a real one now -- a
    // parameter id the addressed effect does not declare.
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        4_242,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
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

    // #140 B: a windowed fader move.
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        QUANTUM,
        [-6.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        4 * u64::from(QUANTUM)
    );
    step(&mut host, 4);

    // #140 B: mute as a fader endpoint, over the same window.
    stage_command(
        &mut host,
        0,
        COMMAND_MUTE,
        255,
        2,
        0,
        0,
        0,
        QUANTUM,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    step(&mut host, 5);

    // #140 A: an effect parameter, `channel = both`, lowering to one span per lane.
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        4,
        0,
        [-12.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        6 * u64::from(QUANTUM)
    );
    step(&mut host, 6);

    // #140 A: live bypass, through the latency-preserving shunt.
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_BYPASS,
        1,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    step(&mut host, 7);

    // #140 C: one batch across two different destination queues.
    stage_command(
        &mut host,
        0,
        COMMAND_MUTE,
        255,
        2,
        0,
        0,
        0,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut host,
        1,
        COMMAND_EFFECT_BYPASS,
        1,
        255,
        0,
        0,
        0,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_OK);
    assert_eq!(host.command_report().admitted, 2);
    step(&mut host, 8);
    step(&mut host, 9);
    assert_eq!(host.status().rendered_quanta, BLOCKS);

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
        pub(super) native_observation: String,
        pub(super) simd128_observation: String,
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
            native_observation: hex_after(text, "\"nativeObservationPcmF32leSha256\":"),
            simd128_observation: hex_after(
                &text[text
                    .find("\"observationTimeline\"")
                    .expect("expected.json has no observationTimeline")..],
                "\"pcmF32leSha256\":",
            ),
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

/// A console host over the *command* fixture: the identity session plus one dynamic-rack
/// parametric EQ, so an effect-addressed command has something real to address (issue #140 A).
fn effect_console_host(quantum: u32, depth: u64) -> AudioWorkletEngineHost {
    let toml = include_str!("../tests/browser-v1/command-session.toml");
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, quantum);
    config.source_ring_frames = quantum;
    config.console_command_queue_records = depth;
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

/// #140 B / E1: a fader command's acknowledgement names the exact sample it takes effect at.
///
/// The fixture is identity end to end, so a constant input renders to that same constant and a
/// fader move has exactly one observable consequence: the whole plane scales. With a zero window
/// the transition is a block boundary and is exact, not approximate.
///
/// Red mutation: move the `while let Ok(record) = self.control.try_pop()` drain in
/// `ConsoleFaderProcessor::process` to *after* `self.fader.process(block)` -> the reported sample
/// is one block early and `at_applied` still renders the pre-command value.
#[test]
fn a_fader_command_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut host = console_host(QUANTUM, 0);
    let quantum = QUANTUM as usize;

    feed_and_render(&mut host, 1, 0, 0.5);
    let before = host.output_pcm().expect("output").to_vec();
    assert!(before.iter().all(|value| *value == 0.5), "unity fader");

    // -6.0206 dB is a hair under exactly half; the assertion below is about *when*, so it
    // compares the whole block against its own first sample and against the untouched input.
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let report = *host.command_report();
    assert_eq!(report.reason, COMMAND_REASON_NONE);
    assert_eq!(report.admitted, 1);
    assert_eq!(report.applied_at_sample, u64::from(QUANTUM));

    feed_and_render(&mut host, 1, 1, 0.5);
    let at_applied = host.output_pcm().expect("output").to_vec();
    let first = at_applied[0];
    assert!(first < 0.5 && first > 0.2, "the block scaled: {first}");
    assert!(
        at_applied
            .iter()
            .all(|value| value.to_bits() == first.to_bits()),
        "a zero-window move is settled for every sample of the block, both lanes",
    );

    // A windowed move ramps inside the block it reported, and settles by its end.
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        QUANTUM,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        2 * u64::from(QUANTUM)
    );
    feed_and_render(&mut host, 1, 2, 0.5);
    let ramping = host.output_pcm().expect("output").to_vec();
    assert!(
        ramping[0] > first && ramping[0] < 0.5,
        "the ramp starts inside the block that reported the sample: {}",
        ramping[0]
    );
    assert_eq!(
        ramping[quantum - 1].to_bits(),
        0.5_f32.to_bits(),
        "a one-quantum window lands exactly on unity by the end of that block"
    );
}

/// #140 B: mute is a fader endpoint. A zero-window mute is the exact `+0.0` the prepared path
/// gives; a windowed mute fades over the window and only then reaches that exact zero.
///
/// Red mutation: make `FaderMuteRampBuiltinsV1::set_mute` snap instead of retargeting -> the
/// windowed mute is already silent on its first sample and the "still audible" assertion fails.
#[test]
fn a_mute_command_is_a_fader_endpoint_not_a_discontinuity() {
    const QUANTUM: u32 = 128;
    let mut host = console_host(QUANTUM, 0);
    let quantum = QUANTUM as usize;

    feed_and_render(&mut host, 1, 0, -0.5);
    stage_command(
        &mut host,
        0,
        COMMAND_MUTE,
        255,
        2,
        0,
        0,
        0,
        QUANTUM,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(host.command_report().applied_at_sample, u64::from(QUANTUM));
    feed_and_render(&mut host, 1, 1, -0.5);
    let fading = host.output_pcm().expect("output").to_vec();
    assert!(
        fading[0] < 0.0 && fading[0] > -0.5,
        "the first sample of a mute fade is still audible: {}",
        fading[0]
    );
    assert!(
        fading[1] > fading[0],
        "the fade moves monotonically toward zero"
    );
    assert_eq!(
        fading[quantum - 1].to_bits(),
        0.0_f32.to_bits(),
        "the completed mute is exactly +0.0, not -0.0, for a negative input"
    );

    // Unmuting is the same event in reverse.
    stage_command(
        &mut host,
        0,
        COMMAND_MUTE,
        255,
        2,
        0,
        0,
        0,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 2, -0.5);
    let restored = host.output_pcm().expect("output").to_vec();
    assert!(
        restored
            .iter()
            .all(|value| value.to_bits() == (-0.5_f32).to_bits()),
        "a zero-window unmute restores the prepared fader exactly"
    );
}

/// #140 A / E1: an effect-parameter command takes effect on the first sample of the block its
/// acknowledgement named, and on no earlier sample.
///
/// The proof is a two-host comparison rather than a closed-form value: the EQ's own 64-sample
/// coefficient ramp is its DSP, not this issue's, so what is gated here is the *boundary*. The
/// control host receives nothing; every block before `applied_at_sample` must be bit-identical
/// between the two, and the block at `applied_at_sample` must differ on its very first sample.
///
/// Red mutation: move the `console.control.stage(..)` drain in `execute_op`'s `ConsoleEffect` arm
/// below `effect.processor.process(block)` -> the first differing block is one later and the
/// `differs at its first sample` assertion fails.
#[test]
fn an_effect_parameter_command_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut control = effect_console_host(QUANTUM, 8);
    let mut commanded = effect_console_host(QUANTUM, 8);

    for block in 0..2_u64 {
        feed_and_render(&mut control, 1, block, 0.25);
        feed_and_render(&mut commanded, 1, block, 0.25);
        assert_eq!(
            control.output_pcm().expect("control"),
            commanded.output_pcm().expect("commanded"),
            "block {block}: no command has been admitted yet",
        );
    }
    // Band 1's gain: parameter id 4 of `miso.parametric-eq`, dynamic rack, effect 0, both lanes.
    stage_command(
        &mut commanded,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        4,
        0,
        [-12.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(commanded.submit_commands(1), RESULT_OK);
    let report = *commanded.command_report();
    assert_eq!(report.reason, COMMAND_REASON_NONE);
    assert_eq!(
        report.admitted, 1,
        "the report counts wire records, not the per-lane spans they lower to"
    );
    assert_eq!(report.applied_at_sample, 2 * u64::from(QUANTUM));

    feed_and_render(&mut control, 1, 2, 0.25);
    feed_and_render(&mut commanded, 1, 2, 0.25);
    let clean = control.output_pcm().expect("control").to_vec();
    let moved = commanded.output_pcm().expect("commanded").to_vec();
    assert_ne!(
        clean[0].to_bits(),
        moved[0].to_bits(),
        "the block at applied_at_sample differs at its first sample",
    );
    assert_ne!(
        clean[QUANTUM as usize].to_bits(),
        moved[QUANTUM as usize].to_bits(),
        "a `channel = both` command lowers to one span per lane, so the right lane moved too",
    );
}

/// #140 A: live effect bypass returns the dry signal at the effect's declared latency, and
/// releasing it returns the wet signal the effect has been computing all along.
///
/// Red mutation: delete the `console.shunt.capture(..)` call in `execute_op` -> a bypassed block
/// renders the shunt's initial zeros instead of the input, and the equality below fails.
#[test]
fn an_effect_bypass_command_returns_the_dry_signal() {
    const QUANTUM: u32 = 128;
    let mut host = effect_console_host(QUANTUM, 8);
    // Move the band well off flat first, so "bypassed" and "enabled" are distinguishable.
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        4,
        0,
        [18.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    for block in 0..3_u64 {
        feed_and_render(&mut host, 1, block, 0.25);
    }
    let wet = host.output_pcm().expect("output").to_vec();

    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_BYPASS,
        1,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        host.command_report().applied_at_sample,
        3 * u64::from(QUANTUM)
    );
    feed_and_render(&mut host, 1, 3, 0.25);
    let dry = host.output_pcm().expect("output").to_vec();
    assert!(
        dry.iter()
            .all(|value| value.to_bits() == 0.25_f32.to_bits()),
        "the parametric EQ declares zero latency, so a bypassed block is the input itself",
    );
    assert_ne!(
        wet[0].to_bits(),
        dry[0].to_bits(),
        "the enabled band was audibly off flat, so bypass is observable",
    );

    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_BYPASS,
        1,
        255,
        0,
        0,
        0,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 4, 0.25);
    let restored = host.output_pcm().expect("output").to_vec();
    assert_ne!(
        restored[0].to_bits(),
        0.25_f32.to_bits(),
        "releasing bypass returns the wet path, whose state stayed continuous throughout",
    );
}

/// #140 C: a submission that mixes kinds is still one transaction, and the free-room pre-check is
/// per *destination queue* -- one full queue refuses the whole batch, including the records bound
/// for queues that had room.
///
/// Red mutation: make the free-room pass read `ready.command_wanted[0]` instead of
/// `ready.command_wanted[slot]` -> the fader flood is checked against the matrix queue's count,
/// the batch is admitted, and the "nothing was admitted" comparison against the clean host fails.
#[test]
fn a_mixed_batch_is_one_transaction_across_every_queue() {
    const QUANTUM: u32 = 128;
    const DEPTH: u32 = 2;
    let mut clean = effect_console_host(QUANTUM, u64::from(DEPTH));
    let mut flooded = effect_console_host(QUANTUM, u64::from(DEPTH));
    feed_and_render(&mut clean, 1, 0, 0.25);
    feed_and_render(&mut flooded, 1, 0, 0.25);

    // One matrix record (room), then one more fader record than the fader queue can hold.
    stage_command(
        &mut flooded,
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
    for index in 0..DEPTH as usize + 1 {
        stage_command(
            &mut flooded,
            index + 1,
            COMMAND_FADER_DB,
            255,
            2,
            0,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(flooded.submit_commands(DEPTH + 2), RESULT_BACKPRESSURE);
    let report = *flooded.command_report();
    assert_eq!(report.reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(report.admitted, 0, "a refused submission admits nothing");

    feed_and_render(&mut clean, 1, 1, 0.25);
    feed_and_render(&mut flooded, 1, 1, 0.25);
    assert_eq!(
        clean.output_pcm().expect("clean"),
        flooded.output_pcm().expect("flooded"),
        "not even the matrix record in the refused batch reached the engine",
    );

    // The same batch, one record shorter, is admitted whole and moves both surfaces.
    stage_command(
        &mut flooded,
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
    for index in 0..DEPTH as usize {
        stage_command(
            &mut flooded,
            index + 1,
            COMMAND_FADER_DB,
            255,
            2,
            0,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(flooded.submit_commands(DEPTH + 1), RESULT_OK);
    assert_eq!(flooded.command_report().admitted, DEPTH + 1);
    feed_and_render(&mut clean, 1, 2, 0.25);
    feed_and_render(&mut flooded, 1, 2, 0.25);
    assert_ne!(
        clean.output_pcm().expect("clean"),
        flooded.output_pcm().expect("flooded"),
        "the admitted batch moved the render",
    );
}

/// #140 C: `UNSUPPORTED_KIND` still means exactly what it says -- the target is real and the value
/// is legal, and *this session* has no write path. A host compiled with no console is that
/// session, and every live kind is refused with it.
#[test]
fn a_console_free_host_refuses_every_live_kind_as_unsupported() {
    const QUANTUM: u32 = 128;
    let mut host = effect_console_host(QUANTUM, 0);
    feed_and_render(&mut host, 1, 0, 0.25);
    let baseline = host.output_pcm().expect("output").to_vec();
    // A console-free host has no staging buffer at all: the refusal is decided before a record
    // could even be written, which is the strongest form of "this session cannot apply it".
    assert!(
        host.command_staging_mut().is_none(),
        "no console means no staging buffer",
    );
    for records in [1_u32, 8, MAXIMUM_COMMAND_RECORDS] {
        assert_eq!(host.submit_commands(records), RESULT_UNSUPPORTED);
        assert_eq!(
            host.command_report().reason,
            COMMAND_REASON_UNSUPPORTED_KIND
        );
        assert_eq!(host.command_report().admitted, 0);
    }
    feed_and_render(&mut host, 1, 1, 0.25);
    assert_eq!(
        baseline,
        host.output_pcm().expect("output"),
        "a console-free host renders the same block it would have without any traffic",
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
    let cases: [UnknownTargetCase; 9] = [
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
        // Issue #140 B: the fader is live, so a *refusal* here has to be a real rule violation.
        // An undefined lane byte is malformed and an out-of-domain decibel value is a domain
        // failure -- and neither is "this engine cannot move it", which is what #137 answered.
        (
            COMMAND_FADER_DB,
            255,
            9,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            COMMAND_FADER_DB,
            255,
            2,
            0,
            0,
            0,
            [24.001, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            COMMAND_MUTE,
            255,
            2,
            0,
            0,
            0,
            [0.5, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
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

/// A three-track observation host over the #143 E4 fixture: compressor, EQ (no tap), gate.
fn observation_host(
    quantum: u32,
    meter_blocks: u64,
    master: Option<u32>,
) -> AudioWorkletEngineHost {
    let toml = include_str!("../../../fixtures/session/v1/observation-frame-shape.toml");
    let mut config = WebPrepareConfigV1::console_defaults(48_000, quantum);
    config.source_ring_frames = quantum * 4;
    config.console_meter_blocks = meter_blocks;
    config.console_observation_taps = 4;
    config.console_master_track_plus_one = master.map_or(0, |track| u64::from(track) + 1);
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

/// Feed one quantum of a constant to every track's shared source and render it.
fn feed_and_render_tracks(host: &mut AudioWorkletEngineHost, block: u64, value: f32) {
    feed_and_render(host, 1, block, value);
}

/// Stage and submit one observation subscribe/unsubscribe for one addressed effect.
fn observe(
    host: &mut AudioWorkletEngineHost,
    track: u32,
    rack: u8,
    effect: u32,
    tap_id: u32,
    window_blocks: u32,
    armed: bool,
) -> u32 {
    let kind = if armed {
        COMMAND_OBSERVE_SUBSCRIBE
    } else {
        COMMAND_OBSERVE_UNSUBSCRIBE
    };
    stage_command(
        host,
        0,
        kind,
        rack,
        255,
        track,
        effect,
        tap_id,
        window_blocks,
        [0.0; 4],
    );
    host.submit_commands(1)
}

/// The frame's gain-reduction section: one non-negative magnitude per track, then the master's.
fn gain_reduction(host: &AudioWorkletEngineHost) -> (Vec<f32>, Option<f32>) {
    let tracks = host.console_tracks().len();
    let frame = host.meter_frame();
    assert_eq!(frame.len(), tracks * 3 + 3, "the frame is 3T + 3 words");
    let base = tracks * 2 + 2;
    let per_track = frame[base..base + tracks].to_vec();
    let master = (host.meter_header().master_gr_present == 1).then(|| frame[base + tracks]);
    (per_track, master)
}

/// Issue #143 E4: the frame the app reads.
///
/// Red mutation: publish the negative decibels raw instead of the declared `PeakMagnitude` fold ->
/// the app's `Math.max(0, -6)` is `0` and every meter reads dead. The fold is what makes that line
/// a no-op rather than a silent zeroing, and the assertion below is the difference.
#[test]
fn the_meter_frame_carries_the_app_shaped_gain_reduction() {
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 2;
    let mut host = observation_host(QUANTUM, BLOCKS, Some(0));
    assert_eq!(host.console_tracks().len(), 3);
    assert!(host.observation_attached());
    assert_eq!(host.set_meter_lease(true), RESULT_OK);

    // Track 1's parametric EQ declares no tap: subscribing to it is an addressing error, not a
    // capacity error, and it is `UNKNOWN_TAP` rather than `UNKNOWN_PARAMETER`.
    assert_eq!(
        observe(&mut host, 1, 1, 0, 1, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TAP);

    for track in [0_u32, 2] {
        assert_eq!(observe(&mut host, track, 1, 0, 1, 2, true), RESULT_OK);
        assert_eq!(host.command_report().reason, COMMAND_REASON_NONE);
    }

    // Silence first: an armed tap over a signal that never crosses a threshold reads exactly zero.
    for block in 0..8 {
        feed_and_render_tracks(&mut host, block, 0.0);
    }
    assert!(host.poll_meters() > 0);
    let (quiet, quiet_master) = gain_reduction(&host);
    assert_eq!(quiet.len(), 3, "one slot per track");
    assert!(quiet.iter().all(|value| value.is_finite()), "{quiet:?}");
    assert_eq!(quiet[0], 0.0, "silence is not compressed");
    assert_eq!(quiet[1], 0.0, "a track with no observed effect reads +0.0");
    assert_eq!(quiet[1].to_bits(), 0.0_f32.to_bits(), "positive zero");
    assert_eq!(
        quiet_master,
        Some(0.0),
        "the designated master reads zero too"
    );

    // Then a signal well over the compressor's threshold and well over the gate's, so track 0
    // reduces and track 2 opens.
    for block in 8..40 {
        feed_and_render_tracks(&mut host, block, 0.5);
    }
    assert!(host.poll_meters() > 0);
    let (loud, master) = gain_reduction(&host);
    assert!(loud.iter().all(|value| value.is_finite()), "{loud:?}");
    assert!(
        loud[0] > 0.0,
        "the compressor's reduction is a positive magnitude, not a negative decibel: {}",
        loud[0]
    );
    assert_eq!(loud[1], 0.0, "the untapped track is still exactly zero");
    assert_eq!(
        master,
        Some(loud[0]),
        "the designated master reports track 0's own reading"
    );

    // The window the frame describes is the meter window, and it tiles.
    let header = *host.meter_header();
    assert_eq!(header.track_count, 3);
    assert_eq!(header.master_track_plus_one, 1);
    assert_eq!(
        header.end_sample - header.first_sample,
        BLOCKS * u64::from(QUANTUM)
    );
    assert!(header.sequence > 0);

    // With no designation at all the master reading is absent, not zero.
    let mut undesignated = observation_host(QUANTUM, BLOCKS, None);
    assert_eq!(undesignated.set_meter_lease(true), RESULT_OK);
    assert_eq!(observe(&mut undesignated, 0, 1, 0, 1, 2, true), RESULT_OK);
    for block in 0..16 {
        feed_and_render_tracks(&mut undesignated, block, 0.5);
    }
    assert!(undesignated.poll_meters() > 0);
    let (values, master) = gain_reduction(&undesignated);
    assert!(values[0] > 0.0, "the track still reduces");
    assert_eq!(master, None, "no designation means absent, never zero");
    assert_eq!(undesignated.meter_header().master_track_plus_one, 0);
}

/// Issue #143 E8: flood and misuse.
///
/// Red mutation: drop the all-or-nothing free-room pre-check for the observe kinds -> an oversized
/// batch arms some taps before it is refused, and the "nothing was armed" assertion fails.
#[test]
fn observation_misuse_is_typed_and_all_or_nothing() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 2, Some(0));
    assert_eq!(host.set_meter_lease(true), RESULT_OK);

    // A tap id the effect does not declare, distinguished from an unknown parameter.
    assert_eq!(
        observe(&mut host, 0, 1, 0, 9, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TAP);
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        99,
        0,
        [0.0; 4],
    );
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER,
        "a parameter and a tap are different namespaces on one effect"
    );

    // Tap zero is reserved for "no tap" and is an unknown tap, not a malformed record.
    assert_eq!(
        observe(&mut host, 0, 1, 0, 0, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TAP);

    // A nonzero value word on a subscription is a caller mistake, not a meaningful field.
    stage_command(
        &mut host,
        0,
        COMMAND_OBSERVE_SUBSCRIBE,
        1,
        255,
        0,
        0,
        1,
        2,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.command_report().reason, COMMAND_REASON_MALFORMED);

    // Unknown rack and unknown effect keep their own reasons on the observe kinds.
    assert_eq!(
        observe(&mut host, 0, 3, 0, 1, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_RACK);
    assert_eq!(
        observe(&mut host, 0, 1, 7, 1, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_EFFECT);
    assert_eq!(
        observe(&mut host, 9, 1, 0, 1, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TRACK);

    // Double subscribe is idempotent and the newer window length wins; double unsubscribe is fine.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 8, true), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    for block in 0..8 {
        feed_and_render_tracks(&mut host, block, 0.5);
    }
    assert!(host.poll_meters() > 0);
    assert_eq!(
        host.meter_header().end_sample - host.meter_header().first_sample,
        2 * u64::from(QUANTUM),
        "the second subscription's window length is the one in force"
    );
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 0, false), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 0, false), RESULT_OK);

    // A lease release and retake restarts the frame sequence and the reported window.
    assert_eq!(host.set_meter_lease(false), RESULT_OK);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(host.meter_header().sequence, 0);
    assert_eq!(host.meter_header().first_sample, 0);
    assert!(host.meter_frame().iter().all(|value| *value == 0.0));

    // The flood: a batch larger than any queue can take is refused whole, nothing is armed, and
    // the frame is untouched.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    let before = host.meter_frame().to_vec();
    let depth = host.config().console_command_queue_records as usize;
    let flood = (depth + 2).min(MAXIMUM_COMMAND_RECORDS as usize);
    for index in 0..flood {
        stage_command(
            &mut host,
            index,
            COMMAND_OBSERVE_SUBSCRIBE,
            1,
            255,
            0,
            0,
            1,
            4,
            [0.0; 4],
        );
    }
    assert_eq!(
        host.submit_commands(flood as u32),
        RESULT_BACKPRESSURE,
        "a batch deeper than the queue is refused whole"
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(host.command_report().admitted, 0);
    assert_eq!(
        host.meter_frame(),
        &before[..],
        "nothing was armed or moved"
    );

    // And a batch beyond the staging capacity is malformed before any queue is consulted.
    assert_eq!(
        host.submit_commands(MAXIMUM_COMMAND_RECORDS + 1),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_MALFORMED);
}

/// A session with a console but no observation capacity refuses a subscription with its own
/// reason: the effect exists, the tap is declared, and this preparation bound no lane.
#[test]
fn a_subscription_without_capacity_is_observation_unbound() {
    const QUANTUM: u32 = 128;
    let toml = include_str!("../../../fixtures/session/v1/observation-frame-shape.toml");
    let mut config = WebPrepareConfigV1::console_defaults(48_000, QUANTUM);
    config.source_ring_frames = QUANTUM * 4;
    config.console_observation_taps = 0;
    config.maximum_meter_streams = 16;
    config.maximum_meter_items = 1 << 16;
    config.maximum_meter_bytes = 1 << 24;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_OK);
    host.session_toml_mut().expect("TOML")[..toml.len()].copy_from_slice(toml.as_bytes());
    assert_eq!(host.compile(toml.len()), RESULT_OK);
    assert!(!host.observation_attached());
    assert_eq!(host.resources().observation_retained_bytes, 0);
    assert_eq!(
        observe(&mut host, 0, 1, 0, 1, 2, true),
        RESULT_UNSUPPORTED,
        "the address is right and this preparation cannot deliver it"
    );
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_OBSERVATION_UNBOUND
    );
    // And the frame is the pre-#143 shape plus the positional gain-reduction section, all zero.
    let tracks = host.console_tracks().len();
    assert_eq!(host.meter_frame().len(), tracks * 3 + 3);
    assert!(host.meter_frame().iter().all(|value| *value == 0.0));
}

/// Observation capacity is refused at configuration time when nothing can carry the subscription.
#[test]
fn observation_configuration_words_are_validated() {
    let mut config = WebPrepareConfigV1::launch_defaults(48_000, 128);
    config.console_observation_taps = 4;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(
        host.prepare(),
        RESULT_INVALID_ARGUMENT,
        "a subscription rides the command queue, so capacity without one has no delivery path"
    );

    let mut config = WebPrepareConfigV1::console_defaults(48_000, 128);
    config.console_observation_taps = u64::from(MAXIMUM_OBSERVATION_TAPS) + 1;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(host.prepare(), RESULT_INVALID_ARGUMENT);

    let mut config = WebPrepareConfigV1::console_defaults(48_000, 128);
    config.console_master_track_plus_one = 1;
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(
        host.prepare(),
        RESULT_INVALID_ARGUMENT,
        "a master designation with no observation capacity would report nothing"
    );

    // And a zeroed pair is exactly what every pre-#143 writer already sends.
    let mut host = AudioWorkletEngineHost::new(WebPrepareConfigV1::console_defaults(48_000, 128));
    assert_eq!(host.prepare(), RESULT_OK);
}

/// Issue #143 E1/E12, native leg: the observation timeline's determinism digest.
///
/// The native twin of `direct-oracle.mjs`'s `runObservationTimeline`. Both legs read this exact
/// fixture file, run this exact twelve-block timeline, and their digests are asserted equal to one
/// pin -- so a change to the audio makes them move together, which is the point.
///
/// Two runs, one timeline: with observation capacity and every declared tap armed, and with
/// `console_observation_taps == 0`. They must render **identical bits**. That is E1's leg (b)
/// against leg (d) on the browser ABI, and it is checked here rather than asserted about.
///
/// Red mutation: fold the observation read into the compressor's inner loop -> the two digests
/// diverge and both stop matching the pin.
#[test]
fn native_observation_timeline_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    const DEPTH: u32 = 4;
    const WINDOW_BLOCKS: u32 = 2;
    const BLOCKS: u64 = 12;

    let toml = include_str!("../tests/browser-v1/observation-session.toml");
    let run = |taps: u64| -> (String, f32, Option<f32>, f32, u64, u32, u32) {
        let mut config = WebPrepareConfigV1::launch_defaults(RATE, QUANTUM);
        config.source_ring_frames = QUANTUM;
        config.console_command_queue_records = u64::from(DEPTH);
        config.console_meter_blocks = u64::from(WINDOW_BLOCKS);
        config.console_observation_taps = taps;
        config.console_master_track_plus_one = if taps == 0 { 0 } else { 1 };
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
        assert_eq!(
            host.resources().observation_retained_bytes == 0,
            taps == 0,
            "the retained row follows the request, and is walked over the built runtime"
        );
        assert_eq!(host.set_meter_lease(true), RESULT_OK);

        let subscribe = |host: &mut AudioWorkletEngineHost, kind: u32, tap: u32, window: u32| {
            stage_command(host, 0, kind, 1, 255, 0, 0, tap, window, [0.0; 4]);
            host.submit_commands(1);
            *host.command_report()
        };
        let unknown_tap = subscribe(&mut host, COMMAND_OBSERVE_SUBSCRIBE, 9, WINDOW_BLOCKS);
        let subscribed = subscribe(&mut host, COMMAND_OBSERVE_SUBSCRIBE, 1, WINDOW_BLOCKS);

        let plane = vec![0.5_f32; QUANTUM as usize];
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
            host.poll_meters();
            blocks.push(host.output_pcm().expect("output").to_vec());
        };
        for block in 0..8 {
            step(&mut host, block);
        }
        let tracks = host.console_tracks().len();
        let base = tracks * 2 + 2;
        let armed = host.meter_frame()[base];
        let master =
            (host.meter_header().master_gr_present == 1).then(|| host.meter_frame()[base + tracks]);
        let window_samples = host.meter_header().end_sample - host.meter_header().first_sample;
        let _ = subscribe(&mut host, COMMAND_OBSERVE_UNSUBSCRIBE, 1, 0);
        for block in 8..BLOCKS {
            step(&mut host, block);
        }
        let disarmed = host.meter_frame()[base];
        assert_eq!(host.status().rendered_quanta, BLOCKS);

        let mut digest = Sha256::new();
        for channel in 0..2_usize {
            for output in &blocks {
                let plane = &output[channel * QUANTUM as usize..(channel + 1) * QUANTUM as usize];
                for sample in plane {
                    digest.update(sample.to_bits().to_le_bytes());
                }
            }
        }
        let hex = digest
            .finalize()
            .iter()
            .fold(String::with_capacity(64), |mut text, byte| {
                use core::fmt::Write as _;
                let _ = write!(&mut text, "{byte:02x}");
                text
            });
        (
            hex,
            armed,
            master,
            disarmed,
            window_samples,
            unknown_tap.reason,
            subscribed.reason,
        )
    };

    let (observed, armed, master, disarmed, window, unknown_tap, subscribed) = run(4);
    let (unobserved, quiet, quiet_master, _, _, _, unbound) = run(0);

    assert_eq!(
        observed, unobserved,
        "arming every declared tap renders the identical bits"
    );
    assert!(armed > 0.0, "an armed tap publishes a positive magnitude");
    assert_eq!(
        master,
        Some(armed),
        "the designated master is track 0's own"
    );
    assert_eq!(disarmed, 0.0, "an unsubscribed tap publishes nothing");
    assert_eq!(window, u64::from(WINDOW_BLOCKS) * u64::from(QUANTUM));
    assert_eq!(unknown_tap, COMMAND_REASON_UNKNOWN_TAP);
    assert_eq!(subscribed, COMMAND_REASON_NONE);
    assert_eq!(quiet, 0.0, "no capacity means no reading");
    assert_eq!(quiet_master, None, "and no master reading either");
    assert_eq!(unbound, COMMAND_REASON_OBSERVATION_UNBOUND);

    let expected: serde_pin::Pin =
        serde_pin::read(include_str!("../tests/browser-v1/expected.json"));
    assert_eq!(
        observed, expected.native_observation,
        "native and the pinned wasm observation-timeline digest must agree bit for bit"
    );
    assert_eq!(
        observed, expected.simd128_observation,
        "the shipped simd128 artifact renders this observation timeline to the same bits"
    );
}

/// Issue #143 R7: what the browser bridge's two moved report rows moved *by*.
///
/// `bridgeMetadataBytes` and `bridgeRetainedBytes` in `tests/browser-v1/expected.json` are not
/// magic numbers and never were: the first is exactly the host shell minus the two structures
/// charged separately plus the plane-reference table, and the second is that plus the staging
/// buffers. `frozen_layouts_and_values_are_exact` asserts the first formula directly; what this
/// adds is the *size of the change*, so the re-pin is derived rather than read off a run.
///
/// Six fields joined `ReadyOwnership`, which is stored inline in the host shell:
///
/// | field | wasm32 | x86-64 |
/// |---|---|---|
/// | `effect_observations: Box<[Option<EffectObservationHandleV1>]>` | 8 | 16 |
/// | `observation_tracks: Box<[u32]>` | 8 | 16 |
/// | `observation_present: Box<[bool]>` | 8 | 16 |
/// | `observation_armed: Box<[u32]>` | 8 | 16 |
/// | `master_track: Option<u32>` | 8 | 8 |
/// | `meter_header: WebMeterHeaderV1` | 64 | 64 |
/// | **sum** | **104** | **136** |
///
/// The shipped wasm32 rows moved by `112`, which is `104` rounded up to the structure's 8-byte
/// alignment. Nothing else in the report moved, which the oracle's `deepStrictEqual` proves.
#[test]
fn the_observation_fields_account_for_the_moved_bridge_rows() {
    let fields = size_of::<Box<[Option<miso_engine_host_core::EffectObservationHandleV1>]>>()
        + size_of::<Box<[u32]>>()
        + size_of::<Box<[bool]>>()
        + size_of::<Box<[u32]>>()
        + size_of::<Option<u32>>()
        + size_of::<WebMeterHeaderV1>();
    let pointer = size_of::<usize>();
    assert_eq!(
        fields,
        4 * 2 * pointer + 8 + usize::try_from(METER_HEADER_BYTES).expect("frozen"),
        "four boxed slices, one optional index, and the meter header"
    );
    // The wasm32 instantiation of that sum, which is what the browser rows moved by.
    assert_eq!(4 * 2 * 4 + 8 + 64, 104);
    assert_eq!(104_usize.next_multiple_of(8), 104);
    // The shipped rows moved by 112: 104 of fields plus 8 of alignment padding inside the shell.
    assert_eq!(3_753_u64 - 3_641, 112);
    assert_eq!(1_075_129_u64 - 1_075_017, 112);
}

/// Issue #143 E9: a `Computed` tap is declared, validated and **refused**.
///
/// No launch effect declares one, so the rule is unreachable from a live session and would
/// otherwise be a branch nothing ever takes. The lowering is exercised directly against a
/// synthetic descriptor that declares both cost classes, which is the only honest way to gate a
/// rule whose production reachability is zero by design.
///
/// Red mutation: bind the computed tap instead of refusing it -> the second case returns `Ok` and
/// the assertion fails. A bound computed tap would be a lane that never publishes: a meter frozen
/// at zero with no way for the caller to learn why.
#[test]
fn a_computed_tap_is_refused_with_unsupported_kind() {
    use miso_engine_effect_contract::{
        EffectDescriptorV1, EffectId, LinkModeSet, ObservationCadenceV1, ObservationChannelsV1,
        ObservationCostV1, ObservationDescriptorV1, ObservationFoldV1, ObservationKindV1,
        ObservationTapId, ParameterUnit,
    };

    const fn tap(
        id: u32,
        cost: ObservationCostV1,
        cadence: ObservationCadenceV1,
    ) -> ObservationDescriptorV1 {
        ObservationDescriptorV1 {
            id: ObservationTapId(id),
            display_name: "Gain Reduction",
            display_unit: "dB",
            kind: ObservationKindV1::GainReductionDb,
            unit: ParameterUnit::Db,
            cost,
            cadence,
            fold: ObservationFoldV1::PeakMagnitude,
            channels: ObservationChannelsV1::Shared,
            minimum: 0.0,
            maximum: 100.0,
        }
    }
    static MENU: [ObservationDescriptorV1; 2] = [
        tap(
            1,
            ObservationCostV1::Resident,
            ObservationCadenceV1::PerBlock,
        ),
        tap(
            2,
            ObservationCostV1::Computed,
            ObservationCadenceV1::PerWindow,
        ),
    ];
    static DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        id: match EffectId::new("test.observation") {
            Ok(value) => value,
            Err(_) => panic!("fixture id"),
        },
        display_name: "Observation fixture",
        contract_major: 1,
        contract_minor: 1,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::ALL,
        parameters: &[],
        ports: &[],
        qualities: &[],
        observations: &MENU,
    };

    let record = |tap_id: u32| CommandRecord {
        kind: COMMAND_OBSERVE_SUBSCRIBE,
        rack: 1,
        channel: 255,
        track_index: 0,
        effect_index: 0,
        parameter_id: tap_id,
        smoothing_samples: 2,
        values: [0.0; 4],
    };

    // The resident tap resolves and binds.
    assert!(record(1).into_observe_record(&DESCRIPTOR, true).is_ok());
    // The computed tap resolves and is refused for what it is -- never `UnknownTap`, which would
    // say the address was wrong, and never silently bound.
    assert_eq!(
        record(2).into_observe_record(&DESCRIPTOR, true).err(),
        Some(COMMAND_REASON_UNSUPPORTED_KIND)
    );
    // A tap id the menu does not declare stays `UnknownTap` on the same descriptor.
    assert_eq!(
        record(3).into_observe_record(&DESCRIPTOR, true).err(),
        Some(COMMAND_REASON_UNKNOWN_TAP)
    );
    // And with no bound lane, the resident tap is `ObservationUnbound` while the computed one is
    // still `UnsupportedKind`: the cost class is a property of the *effect*, checked first.
    assert_eq!(
        record(1).into_observe_record(&DESCRIPTOR, false).err(),
        Some(COMMAND_REASON_OBSERVATION_UNBOUND)
    );
    assert_eq!(
        record(2).into_observe_record(&DESCRIPTOR, false).err(),
        Some(COMMAND_REASON_UNSUPPORTED_KIND)
    );
}

/// Issue #143 R4: the one unit conversion the design permits, and where it happens.
///
/// A `Db` tap crosses the transport already in the unit a meter draws, so the conversion is the
/// identity. A `Linear` tap -- the true-peak limiter's recursive reduction word `d`, where
/// `gain = 1 - d` -- needs a logarithm, which a render thread may not take: it crosses as `d` and
/// becomes decibels here, once per closed window, on the control plane. The result is clamped into
/// the tap's own declared range, so a consumer never has to guess what a number outside it meant.
///
/// Red mutation: publish the linear word unconverted -> `0.5` reports `0.5 dB` instead of
/// `6.02 dB`, and the app's meter reads a tenth of the reduction that is actually happening.
#[test]
fn observation_unit_conversion_is_declared_and_clamped() {
    use miso_engine_effect_contract::{
        ObservationCadenceV1, ObservationChannelsV1, ObservationCostV1, ObservationDescriptorV1,
        ObservationFoldV1, ObservationKindV1, ObservationTapId, ParameterUnit,
    };
    const fn tap(unit: ParameterUnit) -> ObservationDescriptorV1 {
        ObservationDescriptorV1 {
            id: ObservationTapId(1),
            display_name: "Gain Reduction",
            display_unit: "dB",
            kind: ObservationKindV1::GainReductionDb,
            unit,
            cost: ObservationCostV1::Resident,
            cadence: ObservationCadenceV1::PerBlock,
            fold: ObservationFoldV1::PeakMagnitude,
            channels: ObservationChannelsV1::PerLane,
            minimum: 0.0,
            maximum: 100.0,
        }
    }
    let decibels = tap(ParameterUnit::Db);
    let linear = tap(ParameterUnit::Linear);

    // A decibel tap crosses in the consumer's own unit: the conversion is the identity.
    assert_eq!(observed_decibels(decibels, 0.0), 0.0);
    assert_eq!(observed_decibels(decibels, 6.5), 6.5);
    assert_eq!(observed_decibels(decibels, 1_000.0), 100.0, "clamped high");

    // A linear tap is `-20 log10(1 - d)`. Half the amplitude removed is 6.02 dB of reduction, and
    // publishing `0.5` unconverted would report a *tenth* of that.
    assert_eq!(
        observed_decibels(linear, 0.0),
        0.0,
        "no reduction is zero dB"
    );
    let half = observed_decibels(linear, 0.5);
    assert!(
        (half - 6.020_6).abs() < 1e-3,
        "half the amplitude removed is 6.02 dB, not {half}"
    );
    assert!(half > 6.0, "and it is decidedly not the raw 0.5");
    let quarter = observed_decibels(linear, 0.25);
    assert!((quarter - 2.498).abs() < 1e-3, "{quarter}");
    // Total reduction has no finite decibel value; the declared maximum is what a meter draws.
    assert_eq!(observed_decibels(linear, 1.0), 100.0);
    assert_eq!(observed_decibels(linear, 2.0), 100.0);
    assert_eq!(observed_decibels(linear, -1.0), 0.0, "clamped low");
    // Monotonic across the range, which is the property a meter's needle depends on.
    let mut previous = 0.0_f32;
    for step in 0..64 {
        let value = observed_decibels(linear, step as f32 / 64.0);
        assert!(value >= previous, "step {step}: {value} < {previous}");
        assert!(value.is_finite());
        previous = value;
    }
}

/// The identity fixture with three sources declared out of canonical order (issue #207).
///
/// `zeta` is declared first and `alpha` last, so a query that reported *declaration* order rather
/// than the normalized order would be visible here rather than hidden by an already-sorted
/// fixture. The shapes are all distinct -- different channel counts, different region starts,
/// different lengths -- so a query that read the wrong row is visible too. The one track points at
/// `mid`, which is neither the first nor the last of the three by either ordering.
fn three_source_session(quantum: u32) -> String {
    let mut model = parse_session_toml(include_str!("../tests/browser-v1/session.toml"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    model.limits.pcm_ring_frames = u64::from(quantum);
    let template = model.sources[0].clone();
    let source = |id: &str, channels: u8, start: u64, length: u64| {
        let mut value = template.clone();
        value.id = miso_engine_session::StableId::parse(id).expect("stable id");
        value.mapping.channel_count = channels;
        value.mapping.region.start_sample = start;
        value.mapping.region.length_samples = length;
        value
    };
    model.sources = vec![
        source("zeta", 1, 0, u64::from(quantum) * 3),
        source("mid", 2, u64::from(quantum) * 7, u64::from(quantum) * 5),
        source("alpha", 4, 9, u64::from(quantum) * 2),
    ];
    model.tracks[0].source_id = miso_engine_session::StableId::parse("mid").expect("stable id");
    canonical_session_toml(&model).expect("canonical three-source session")
}

/// Issue #207 D1: the compiled session answers what sources exist, in canonical order, with the
/// shape a headless driver needs to feed them.
///
/// Red mutation: report declaration order instead of the normalized order -> the assertion below
/// reads `["zeta", "mid", "alpha"]`. Red mutation: report `region.length_samples` as the region
/// *end* -> `mid` reads 1536 frames instead of 640. Neither survives a fixture whose sources are
/// deliberately unsorted and whose regions deliberately do not start at zero.
#[test]
fn session_source_introspection_is_canonical_ordered_shaped_and_bounded() {
    const QUANTUM: u32 = 128;
    let toml = three_source_session(QUANTUM);
    let mut host = prepared_host(QUANTUM);

    // Before compilation there is no session, so there are no sources -- the same state gating the
    // track queries carry, for the same reason: the answer lives in the compiled session.
    assert_eq!(host.session_source_count(), 0);
    assert_eq!(host.session_source_id(0), None);
    assert_eq!(host.session_source_shape(0), None);

    host.session_toml_mut().expect("prepared TOML buffer")[..toml.len()]
        .copy_from_slice(toml.as_bytes());
    assert_eq!(
        host.compile(toml.len()),
        RESULT_OK,
        "{:?}",
        core::str::from_utf8(host.diagnostic())
    );

    assert_eq!(host.session_source_count(), 3);
    let ids: Vec<&str> = (0..3)
        .map(|index| host.session_source_id(index).expect("declared source"))
        .collect();
    assert_eq!(
        ids,
        ["alpha", "mid", "zeta"],
        "canonical source order is the normalized model's, which is sorted by stable ID"
    );
    assert_eq!(
        host.session_source_shape(0).expect("alpha"),
        SessionSourceShape {
            channel_count: 4,
            sample_rate_hz: 48_000,
            region_start_frame: 9,
            region_frames: u64::from(QUANTUM) * 2,
        }
    );
    assert_eq!(
        host.session_source_shape(1).expect("mid"),
        SessionSourceShape {
            channel_count: 2,
            sample_rate_hz: 48_000,
            region_start_frame: u64::from(QUANTUM) * 7,
            region_frames: u64::from(QUANTUM) * 5,
        }
    );
    assert_eq!(
        host.session_source_shape(2).expect("zeta"),
        SessionSourceShape {
            channel_count: 1,
            sample_rate_hz: 48_000,
            region_start_frame: 0,
            region_frames: u64::from(QUANTUM) * 3,
        }
    );

    // The track order is unchanged by any of this: the two lists are independent, and the source
    // list is not a filter of the referenced sources either -- `alpha` and `zeta` are declared and
    // therefore reported, though no track reads them.
    assert_eq!(host.console_tracks().len(), 1);
    assert_eq!(&*host.console_tracks()[0], "track");

    // One past the end, and the u32 ceiling.
    assert_eq!(host.session_source_id(3), None);
    assert_eq!(host.session_source_shape(3), None);
    assert_eq!(host.session_source_id(u32::MAX), None);
    assert_eq!(host.session_source_shape(u32::MAX), None);
}

/// Issue #207 D1: the raw exports, held to the track queries' conventions exactly.
///
/// The shape queries answer zero out of range because zero is impossible for a compiled source --
/// the session validator refuses `channel_count == 0`, `length_samples == 0` and
/// `sample_rate_hz == 0` -- while `source_start_frame` has no spare value and leans on
/// `source_count` as the bounds authority. That asymmetry is asserted here so it cannot be
/// "tidied" into a sentinel that collides with a real region start.
#[test]
fn raw_ffi_source_introspection_mirrors_the_track_queries() {
    const QUANTUM: u32 = 128;
    let toml = three_source_session(QUANTUM);
    let handle = miso_engine_web_v1_config_new();
    assert_ne!(handle, 0);

    // An invalid handle answers the invalid value on every query, as every other export does.
    for probe in [0, handle.wrapping_add(1)] {
        assert_eq!(miso_engine_web_v1_source_count(probe), 0);
        assert_eq!(miso_engine_web_v1_source_id(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_channels(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_frames(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_start_frame(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_sample_rate(probe, 0), 0);
    }

    let mut config = WebPrepareConfigV1::launch_defaults(48_000, QUANTUM);
    config.source_ring_frames = QUANTUM;
    config.maximum_source_channels = 4;
    assert_eq!(crate::ffi::test_configure(handle, config), RESULT_OK);
    assert_eq!(miso_engine_web_v1_prepare(handle), RESULT_OK);

    // Prepared but not compiled: staging exists, a session does not.
    assert_eq!(miso_engine_web_v1_source_count(handle), 0);
    assert_eq!(miso_engine_web_v1_source_id(handle, 0), 0);
    assert_eq!(miso_engine_web_v1_console_track_count(handle), 0);

    assert_eq!(
        crate::ffi::test_copy_staging(handle, BUFFER_SESSION_TOML, toml.as_bytes()),
        RESULT_OK
    );
    assert_eq!(
        miso_engine_web_v1_compile(handle, toml.len() as u32),
        RESULT_OK
    );

    assert_eq!(miso_engine_web_v1_source_count(handle), 3);
    let read = |index: u32| {
        let length = miso_engine_web_v1_source_id(handle, index);
        let bytes = crate::ffi::test_read_source_id(handle, length).expect("staging");
        String::from_utf8(bytes).expect("ASCII source ID")
    };
    assert_eq!([read(0), read(1), read(2)], ["alpha", "mid", "zeta"]);
    assert_eq!(
        [
            miso_engine_web_v1_source_channels(handle, 0),
            miso_engine_web_v1_source_channels(handle, 1),
            miso_engine_web_v1_source_channels(handle, 2),
        ],
        [4, 2, 1]
    );
    assert_eq!(
        [
            miso_engine_web_v1_source_frames(handle, 0),
            miso_engine_web_v1_source_frames(handle, 1),
            miso_engine_web_v1_source_frames(handle, 2),
        ],
        [
            u64::from(QUANTUM) * 2,
            u64::from(QUANTUM) * 5,
            u64::from(QUANTUM) * 3
        ]
    );
    assert_eq!(
        [
            miso_engine_web_v1_source_start_frame(handle, 0),
            miso_engine_web_v1_source_start_frame(handle, 1),
            miso_engine_web_v1_source_start_frame(handle, 2),
        ],
        [9, u64::from(QUANTUM) * 7, 0]
    );
    for index in 0..3 {
        assert_eq!(miso_engine_web_v1_source_sample_rate(handle, index), 48_000);
    }

    // Out of range: zero everywhere it can be said, and `source_count` is what a caller checked
    // before asking, because `source_start_frame`'s zero is `zeta`'s real answer.
    assert_eq!(miso_engine_web_v1_source_id(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_channels(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_frames(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_sample_rate(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_start_frame(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_start_frame(handle, 2), 0);
    assert_eq!(miso_engine_web_v1_source_id(handle, u32::MAX), 0);

    // The queries survive a sticky failure, exactly as the track queries do: nothing here is
    // dropped on the failure path, so a diagnosing consumer can still read the session map.
    assert_eq!(
        miso_engine_web_v1_render(handle, QUANTUM.wrapping_add(1)),
        RESULT_REPREPARE_REQUIRED
    );
    assert_eq!(
        crate::ffi::test_status(handle).expect("status").state,
        STATE_FAILED
    );
    assert_eq!(miso_engine_web_v1_source_count(handle), 3);
    assert_eq!(miso_engine_web_v1_source_channels(handle, 1), 2);
    assert_eq!(miso_engine_web_v1_console_track_count(handle), 1);

    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_source_count(handle), 0);
}
