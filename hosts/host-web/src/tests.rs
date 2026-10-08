use core::mem::{offset_of, size_of, size_of_val};

use effect_contract::{PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedEffectTarget};
use host_core::{
    EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer, InputFilterPreparer,
};
use session::{canonical_session_json, parse_session_json};

use super::*;

fn one_track_session(quantum: u32) -> String {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("accepted fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 2;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    canonical_session_json(&model).expect("canonical one-track session")
}

fn one_track_resource_session(quantum: u32) -> String {
    let mut model = parse_session_json(&one_track_session(quantum)).expect("one-track session");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
    }
    canonical_session_json(&model).expect("canonical resource session")
}

fn one_track_compressor_session(quantum: u32) -> String {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/compressor-dynamic-observation.json"
    ))
    .expect("accepted compressor fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 4;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    canonical_session_json(&model).expect("canonical compressor session")
}

fn one_track_multiband_session(quantum: u32) -> String {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/compressor-dynamic-observation.json"
    ))
    .expect("accepted compressor fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 4;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let effect = &mut model.tracks[0].inserts.effects[0];
    effect.identity = session::EffectIdentity::Native {
        effect_id: session::StableId::parse("miso.multiband-compressor")
            .expect("multiband effect ID"),
    };
    effect.params.clear();
    canonical_session_json(&model).expect("canonical multiband session")
}

/// The browser fixture's identity session, re-shaped for one test.
///
/// Identity end to end: no polarity, trim, HPF or LPF, no effects in any rack, unity fader, and a
/// hard-left/hard-right pan whose 2x2 matrix is the identity. The output is therefore the submitted
/// source frames, which is what makes the submitted ramp its own oracle.
fn identity_session(quantum: u32, length_samples: u64) -> String {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = length_samples;
    canonical_session_json(&model).expect("canonical identity session")
}

fn prepared_host(quantum: u32) -> AudioWorkletEngineHost {
    let document = one_track_session(quantum);
    AudioWorkletEngineHost::boot(document.as_bytes(), boot_options(quantum))
        .unwrap_or_else(|failure| panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic())))
}

fn boot_options(quantum: u32) -> WebBootOptions {
    WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: quantum,
        ..WebBootOptions::explicit_defaults()
    }
}

fn compressor_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    let document = one_track_compressor_session(quantum);
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: quantum,
            live_control_command_queue_records: 4,
            ..boot_options(quantum)
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "compressor boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

fn multiband_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    let document = one_track_multiband_session(quantum);
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: quantum,
            live_control_command_queue_records: 4,
            ..boot_options(quantum)
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "multiband boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

fn feed_compressor_block(host: &mut AudioWorkletEngineHost, quantum: u32, block: u64) {
    let plane = vec![0.25_f32; quantum as usize];
    let planes: [&[f32]; 2] = [&plane, &plane];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            block * u64::from(quantum),
            48_000,
            &planes,
            quantum,
            false,
        ),
        RESULT_OK
    );
    assert_eq!(host.render_next(), RESULT_OK);
}

/// The actual three-track session fixture with its dynamic-rack causal gate at track `t2`.
fn gate_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    observation_host(quantum, 0, None)
}

fn feed_gate_block(host: &mut AudioWorkletEngineHost, block: u64) {
    feed_and_render_tracks(host, block, 0.25);
}

fn retained_projection(document: &[u8], options: WebBootOptions) -> u64 {
    let model = parse_host_session(core::str::from_utf8(document).expect("UTF-8 session"))
        .expect("accepted host session");
    let compiled = compile_host_model(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled host session");
    let shape = compiled_session_shape(&compiled).expect("compiled session shape");
    let source_ring_frames = if options.source_ring_frames == 0 {
        default_source_ring_frames(shape.sample_rate_hz, shape.quantum_frames)
    } else {
        options.source_ring_frames
    };
    let projection = project_buffers(
        u32::try_from(document.len()).expect("bounded fixture document"),
        shape.sample_rate_hz,
        shape.quantum_frames,
        shape.maximum_source_channels,
        shape
            .longest_source_id_bytes
            .max(shape.longest_track_id_bytes)
            .max(shape.longest_submix_id_bytes)
            .max(shape.longest_route_id_bytes)
            .max(shape.longest_vca_id_bytes),
        options,
        (false, (0, 0)),
    )
    .expect("bridge projection");
    projected_retained_bytes(
        &compiled,
        source_ring_frames,
        projection.report.bridge_retained_bytes,
    )
    .expect("retained projection")
}

fn exact_retained_report_total(resources: &WebResourceReport) -> u64 {
    resources
        .bridge_retained_bytes
        .checked_add(resources.graph_session_plus_plan_bytes)
        .and_then(|total| total.checked_add(resources.source_total_bytes))
        .expect("independent exact retained sum")
}

struct DenseInvalidAutomationDocument {
    bytes: Vec<u8>,
    segment_count: usize,
}

fn maximum_document_with_dense_invalid_automation() -> DenseInvalidAutomationDocument {
    const BASE: &str = include_str!("../tests/browser-v1/session.json");
    const EMPTY_AUTOMATION: &str = "\"automation\": []";
    const HEADER: &str = r#""automation": [{"id":"dense-invalid","target":{"entity_id":"track","rack":"builtins","effect_id":"strip","parameter_id":5,"channel":"both"},"segments":["#;
    const INVALID_SEGMENT: &str = "{\"shape\":\"step\",\"start_sample\":\"0\",\"end_sample\":\"0\",\"start_value\":0.0,\"end_value\":0.0,\"unit\":\"db\"}";
    const FOOTER: &str = "]}]";

    let (before, after) = BASE
        .split_once(EMPTY_AUTOMATION)
        .expect("browser fixture has the automation replacement seam");
    let maximum = MAXIMUM_DOCUMENT_BYTES as usize;
    let fixed_bytes = before.len() + HEADER.len() + FOOTER.len() + after.len();
    let segment_count = (maximum - fixed_bytes + 1) / (INVALID_SEGMENT.len() + 1);
    assert!(
        segment_count > 10_000,
        "fixture remains densely adversarial"
    );

    let mut document = String::with_capacity(maximum);
    document.push_str(before);
    document.push_str(HEADER);
    for position in 0..segment_count {
        if position != 0 {
            document.push(',');
        }
        document.push_str(INVALID_SEGMENT);
    }
    document.push_str(FOOTER);
    document.push_str(after);
    let padding = maximum - document.len();
    document.extend(core::iter::repeat_n(' ', padding));
    assert_eq!(document.len(), maximum);
    DenseInvalidAutomationDocument {
        bytes: document.into_bytes(),
        segment_count,
    }
}

#[test]
fn maximum_document_dense_invalid_fixture_reaches_bounded_semantic_validation() {
    let fixture = maximum_document_with_dense_invalid_automation();
    assert_eq!(fixture.bytes.len(), MAXIMUM_DOCUMENT_BYTES as usize);
    assert!(fixture.segment_count > 10_000);
    let source = core::str::from_utf8(&fixture.bytes).expect("fixture is UTF-8 JSON");
    assert!(
        !source.contains("{}"),
        "fixture has no empty sentinel segment"
    );
    assert_eq!(
        source
            .matches("\"start_sample\":\"0\",\"end_sample\":\"0\"")
            .count(),
        fixture.segment_count,
        "every repeated segment has equal bounds"
    );

    let failure = parse_session_json(source).expect_err("equal segment bounds must be invalid");
    let diagnostics = failure.diagnostics();
    assert_eq!(diagnostics.len(), 64, "semantic diagnostics stay bounded");
    for (position, diagnostic) in diagnostics.iter().enumerate() {
        assert_eq!(
            diagnostic.code,
            session::DiagnosticCode::AutomationInvalidRange
        );
        assert_eq!(
            diagnostic.path.to_string(),
            format!("$.automation[0].segments[{position}].end_sample")
        );
    }
}

#[test]
fn frozen_layouts_and_values_are_exact() {
    assert_eq!(ABI_VERSION, 0x0001_0000);
    assert_eq!(size_of::<WebBootOptions>(), 64);
    assert_eq!(size_of::<WebStatus>(), 80);
    assert_eq!(size_of::<WebResourceReport>(), 224);
    assert_eq!(MAXIMUM_DOCUMENT_BYTES, 1 << 20);
    assert_eq!(PARSE_TRANSIENT_MULTIPLIER, 20);
    assert_eq!(DEFAULT_MAXIMUM_MEMORY_BYTES, 512 << 20);
    assert_eq!(DIAGNOSTIC_BYTES, 1 << 14);
    assert_eq!(
        [
            RESULT_REFUSED_DOCUMENT,
            RESULT_REFUSED_OPTIONS,
            RESULT_REFUSED_BUDGET,
            RESULT_REFUSED_LIFECYCLE,
            RESULT_REPREPARE_REQUIRED,
        ],
        [1, 2, 5, 3, 9]
    );
    assert_eq!(
        [
            RESULT_OK,
            RESULT_INVALID_ARGUMENT,
            RESULT_ABI_MISMATCH,
            RESULT_WRONG_STATE,
            RESULT_BUFFER_TOO_SMALL,
            RESULT_REFUSED_BUDGET,
            RESULT_BACKPRESSURE,
            RESULT_UNSUPPORTED,
            RESULT_RENDER_REJECTED,
            RESULT_REPREPARE_REQUIRED,
            RESULT_INTERNAL,
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255]
    );
    assert_eq!([STATE_READY, STATE_FAILED, STATE_DISPOSED], [2, 3, 4]);
    assert_eq!([BACKEND_SCALAR, BACKEND_SIMD128], [0, 1]);
    assert_eq!(
        [
            BUFFER_SOURCE_ID,
            BUFFER_SOURCE_PCM,
            BUFFER_DIAGNOSTIC,
            BUFFER_OUTPUT_PCM,
            BUFFER_COMMAND,
            BUFFER_METER_FRAME
        ],
        [2, 3, 4, 5, 6, 7]
    );
    // Issue #137 D1: the two live-control words are the first two of the frozen configuration's
    // four reserved words. Every V1 writer already sets them to zero, which is exactly "default
    // command queue depth, no meters attached", so the 192-byte layout and every existing caller
    // stand.
    assert_eq!(size_of::<WebCommandReport>(), 48);
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
            COMMAND_REASON_OBSERVATION_UNBOUND,
            COMMAND_REASON_NOT_SOLOABLE
        ],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    );
    assert_eq!(offset_of!(WebBootOptions, struct_size), 0);
    assert_eq!(offset_of!(WebBootOptions, require_quantum_frames), 12);
    assert_eq!(offset_of!(WebBootOptions, source_ring_frames), 16);
    assert_eq!(offset_of!(WebBootOptions, maximum_memory_bytes), 24);
    assert_eq!(
        offset_of!(WebBootOptions, live_control_command_queue_records),
        32
    );
    assert_eq!(offset_of!(WebBootOptions, live_control_meter_blocks), 40);
    // Issue #143 D3/D6: the configuration's remaining two reserved words, carved exactly as #137
    // carved the first two. The structure is still 192 bytes and every existing offset is where it
    // was, so a V1 writer that zeroes them gets "no observation capacity, no master designation".
    assert_eq!(
        offset_of!(WebBootOptions, live_control_observation_taps),
        48
    );
    assert_eq!(
        offset_of!(WebBootOptions, live_control_master_track_plus_one),
        56
    );
    assert_eq!(MAXIMUM_OBSERVATION_TAPS, 16);
    // The meter header is a new fixed structure, not a change to an existing one.
    // Issue #1209 D2 appended `submix_count` and its pad: 64 bytes became 72, nothing moved.
    assert_eq!(size_of::<WebMeterHeader>(), 72);
    assert_eq!(METER_HEADER_BYTES, 72);
    assert_eq!(offset_of!(WebMeterHeader, track_count), 8);
    assert_eq!(offset_of!(WebMeterHeader, windows), 12);
    assert_eq!(offset_of!(WebMeterHeader, first_sample), 16);
    assert_eq!(offset_of!(WebMeterHeader, end_sample), 24);
    assert_eq!(offset_of!(WebMeterHeader, sequence), 32);
    assert_eq!(offset_of!(WebMeterHeader, master_track_plus_one), 40);
    assert_eq!(offset_of!(WebMeterHeader, master_gr_present), 44);
    assert_eq!(offset_of!(WebMeterHeader, reserved), 48);
    assert_eq!(offset_of!(WebMeterHeader, submix_count), 64);
    assert_eq!(offset_of!(WebMeterHeader, reserved_pad), 68);
    assert_eq!(offset_of!(WebCommandReport, result), 8);
    assert_eq!(offset_of!(WebCommandReport, rejected_index), 16);
    assert_eq!(offset_of!(WebCommandReport, applied_at_sample), 24);
    assert_eq!(offset_of!(WebCommandReport, reserved), 32);
    assert_eq!(offset_of!(WebStatus, state), 8);
    assert_eq!(offset_of!(WebStatus, next_absolute_sample), 32);
    assert_eq!(offset_of!(WebStatus, reserved), 48);
    assert_eq!(offset_of!(WebResourceReport, options_bytes), 32);
    assert_eq!(offset_of!(WebResourceReport, id_staging_bytes), 64);
    assert_eq!(
        offset_of!(WebResourceReport, largest_named_allocation_bytes),
        184
    );
    // Issue #143: the report's first reserved word becomes `observation_retained_bytes`; the
    // structure is still 224 bytes and every existing offset is unmoved.
    assert_eq!(
        offset_of!(WebResourceReport, observation_retained_bytes),
        192
    );
    assert_eq!(offset_of!(WebResourceReport, reserved), 200);

    // Issue #137: `bridgeMetadataBytes` in `tests/browser-v1/expected.json` is not a magic number
    // and never was. It is exactly this formula over the host shell, so when the shell grows the
    // pinned row moves by exactly that growth and by nothing else -- which is how the two rows
    // that moved for #137 (`bridgeMetadataBytes`, `bridgeRetainedBytes`, both +152 on wasm32) were
    // derived rather than read off a run.
    let host = prepared_host(128);
    let plane_references = 8 * size_of::<&[f32]>() as u64;
    assert!(
        host.resources().bridge_metadata_bytes
            >= size_of::<AudioWorkletEngineHost>() as u64
                - u64::from(BOOT_OPTIONS_BYTES)
                - u64::from(STATUS_BYTES)
                + plane_references,
        "ready metadata is added to the exact fixed bridge projection"
    );
}

#[test]
fn raw_ffi_validates_handle_layout_overflow_and_transactional_failure() {
    assert_eq!(miso_engine_web_v1_abi_version(), ABI_VERSION);
    assert_eq!(miso_engine_web_v1_dispose(0), RESULT_OK);
    crate::ffi::test_stage_document(b"no=");
    assert_eq!(miso_engine_web_v1_boot(3), 0);
    assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_DOCUMENT);
    assert_eq!(
        miso_engine_web_v1_boot_diagnostic_bytes(),
        3,
        "diagnostic replacement is truncated to the staged document capacity"
    );
    assert_ne!(crate::ffi::test_staged_document(), b"no=");
    assert_eq!(miso_engine_web_v1_status_ptr(0), 0);
    assert_eq!(miso_engine_web_v1_resource_ptr(0), 0);
    assert_eq!(miso_engine_web_v1_buffer_ptr(0, BUFFER_DIAGNOSTIC), 0);
    assert_eq!(miso_engine_web_v1_boot(3), 0, "refusal invalidates staging");
    assert_eq!(miso_engine_web_v1_boot_diagnostic_bytes(), 0);
    assert_eq!(
        miso_engine_web_v1_document_ptr(MAXIMUM_DOCUMENT_BYTES + 1),
        0
    );
    assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_DOCUMENT);

    let document = one_track_session(128);
    let handle = crate::ffi::test_boot(document.as_bytes(), boot_options(128));
    assert_ne!(handle, 0);
    crate::ffi::test_stage_document(document.as_bytes());
    assert_eq!(miso_engine_web_v1_boot(document.len() as u32), 0);
    assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_LIFECYCLE);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
}

#[test]
fn raw_ffi_uses_stable_staging_and_exact_output_quantum_without_growth() {
    let quantum = 64_u32;
    let document = one_track_session(quantum);
    let options = WebBootOptions {
        source_ring_frames: quantum,
        ..boot_options(quantum)
    };
    let handle = crate::ffi::test_boot(document.as_bytes(), options);
    assert_ne!(handle, 0);
    let status_address = crate::ffi::test_status_address(handle);
    let resource_address = crate::ffi::test_resource_address(handle);
    let addresses = [
        BUFFER_SOURCE_ID,
        BUFFER_SOURCE_PCM,
        BUFFER_DIAGNOSTIC,
        BUFFER_OUTPUT_PCM,
    ]
    .map(|kind| crate::ffi::test_buffer_address(handle, kind));
    assert!(addresses.into_iter().all(|address| address != 0));
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
    assert!(resources.bridge_retained_bytes <= DEFAULT_MAXIMUM_MEMORY_BYTES);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_INVALID_ARGUMENT);
    let replacement = crate::ffi::test_boot(document.as_bytes(), options);
    assert_ne!(replacement, 0);
    assert_ne!(replacement, handle);
    assert_eq!(
        crate::ffi::test_copy_staging(replacement, BUFFER_SOURCE_ID, b"fixture-source"),
        RESULT_OK
    );
    assert_eq!(
        crate::ffi::test_fill_source_pcm(replacement, 0.5),
        RESULT_OK
    );
    assert_eq!(
        miso_engine_web_v1_source_submit(replacement, 14, 1, 0, 2, quantum, 0),
        RESULT_OK
    );
    assert_eq!(miso_engine_web_v1_render(replacement, quantum), RESULT_OK);
    assert_eq!(miso_engine_web_v1_dispose(replacement), RESULT_OK);
}

#[test]
fn preparation_accepts_explicit_64_128_and_256_quanta_with_stable_buffers() {
    for quantum in [64, 128, 256] {
        let mut host = prepared_host(quantum);
        assert_eq!(host.status().state, STATE_READY);
        let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
        let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
        let output_ptr = host.output_pcm().expect("output").as_ptr();
        assert_eq!(
            host.source_pcm_mut().expect("PCM").len(),
            2 * quantum as usize
        );
        assert_eq!(
            host.output_pcm().expect("output").len(),
            2 * quantum as usize
        );
        assert_eq!(source_id_ptr, host.source_id_mut().expect("ID").as_ptr());
        assert_eq!(source_pcm_ptr, host.source_pcm_mut().expect("PCM").as_ptr());
        assert_eq!(output_ptr, host.output_pcm().expect("output").as_ptr());
    }
}

#[test]
fn malformed_config_and_atomic_compile_failure_are_sticky() {
    let mut bad = boot_options(128);
    bad.abi_version = 1;
    let failure = AudioWorkletEngineHost::boot(one_track_session(128).as_bytes(), bad)
        .err()
        .expect("wrong ABI");
    assert_eq!(failure.result(), RESULT_REFUSED_OPTIONS);
    assert_eq!(failure.diagnostic(), b"web.options.abi_version\t$\n");

    let failure = AudioWorkletEngineHost::boot(b"no=", boot_options(128))
        .err()
        .expect("bad document");
    assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT);
    assert!(!failure.diagnostic().is_empty());
}

/// Issue #387: json-syntax 0.12.5 never calls `end_fragment` for an empty JSON object, so the
/// reserved `CodeMap` entry keeps `volume = 0` and every later member is read one slot off --
/// `session::parse::Parser::keys` used to hit `Option::unwrap()` on the resulting `None`, which
/// traps the AudioWorklet module under this workspace's `panic = "abort"` release profile
/// (`scripts/build-web-audioworklet.sh` builds `--release`). The session preflight now refuses
/// `{}` anywhere in the document before the typed walk, so `AudioWorkletEngineHost::boot` --
/// the same call `miso_engine_web_v1_boot` (`hosts/host-web/src/ffi.rs`) makes -- returns a
/// documented `RESULT_REFUSED_DOCUMENT` diagnostic instead of unwinding into the abort.
#[test]
fn empty_object_document_boots_to_a_diagnostic_not_a_trap() {
    let document = one_track_session(128);
    let start = document
        .find("\"render_profile\": {")
        .expect("render_profile key");
    let open = start + "\"render_profile\": ".len();
    let mut depth = 0i32;
    let mut cursor = open;
    let close = loop {
        match document.as_bytes()[cursor] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    break cursor + 1;
                }
            }
            _ => {}
        }
        cursor += 1;
    };
    let mutated = format!("{}{{}}{}", &document[..open], &document[close..]);
    assert_ne!(mutated, document, "render_profile was not actually emptied");

    let failure = AudioWorkletEngineHost::boot(mutated.as_bytes(), boot_options(128))
        .err()
        .expect("empty object must refuse, not abort");
    assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT);
    assert_eq!(failure.diagnostic(), b"json.syntax\t$.render_profile\n");

    // The same document through the raw C ABI boot entry the wasm artifact exports: staged,
    // booted, and refused without ever reaching a live handle.
    assert_eq!(miso_engine_web_v1_dispose(0), RESULT_OK);
    crate::ffi::test_stage_document(mutated.as_bytes());
    assert_eq!(
        miso_engine_web_v1_boot(mutated.len() as u32),
        0,
        "a refused boot never returns a live handle"
    );
    assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_DOCUMENT);
    assert!(
        miso_engine_web_v1_boot_diagnostic_bytes() > 0,
        "the refusal leaves a nonempty diagnostic prefix for the host to read"
    );
    assert_eq!(miso_engine_web_v1_status_ptr(0), 0);
}

#[test]
fn compile_resource_caps_are_inclusive_and_one_below_rejects() {
    let mut document = one_track_session(128);
    // Keep this a parser-projection boundary above the fixed live-response staging allocation:
    // insignificant trailing whitespace raises only parser input.
    document.extend(core::iter::repeat_n(' ', 131_072));
    let parse_projection = document.len() as u64 * PARSE_TRANSIENT_MULTIPLIER;
    let accepted = WebBootOptions {
        maximum_memory_bytes: parse_projection,
        ..boot_options(128)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), accepted).expect("inclusive budget");
    let refused = WebBootOptions {
        maximum_memory_bytes: parse_projection - 1,
        ..boot_options(128)
    };
    let failure = AudioWorkletEngineHost::boot(document.as_bytes(), refused)
        .err()
        .expect("one byte below parse projection");
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
    assert_eq!(
        failure.diagnostic(),
        format!(
            "host.budget.parse_projection\t$.maximum_memory_bytes[projected_bytes={parse_projection},budget_bytes={}]\n",
            parse_projection - 1
        )
        .as_bytes()
    );
}

/// Issue #240 built-in eval 4: the complete production refusal is typed and returns the fixed,
/// bounded diagnostic set for the maximum document. The separate phase oracle proves that the
/// exact 1 MiB typed document reaches semantic validation and retains only the first 64
/// invalid-range source spans.
///
/// The wall-clock half of this claim ("finishes under one second") is deliberately not measured
/// here: a debug-build assertion on a shared CI runner has no fixed relationship to the shipped
/// profile's speed (issue #359 WP-2, §10) and was one of this workspace's five worst-offending
/// false-red causes. It is [`maximum_document_dense_invalid_boot_finishes_under_one_second_in_release`]
/// below, `#[ignore]`d for nightly, release-mode measurement.
#[test]
fn maximum_document_dense_invalid_boot_is_typed_and_bounded() {
    let fixture = maximum_document_with_dense_invalid_automation();
    assert_eq!(fixture.bytes.len(), MAXIMUM_DOCUMENT_BYTES as usize);
    let failure = AudioWorkletEngineHost::boot(&fixture.bytes, WebBootOptions::default())
        .err()
        .expect("dense invalid automation must refuse");
    assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT);
    assert_eq!(
        failure
            .diagnostic()
            .split(|byte| *byte == b'\n')
            .next()
            .expect("first diagnostic"),
        b"automation.invalid_range\t$.automation[0].segments[0].end_sample"
    );
    assert_eq!(
        failure
            .diagnostic()
            .iter()
            .filter(|byte| **byte == b'\n')
            .count(),
        host_core::diagnostics::MAXIMUM_PREPARE_DIAGNOSTIC_LINES,
        "the full boot returns the fixed diagnostic count"
    );
    assert!(
        failure
            .diagnostic()
            .ends_with(b"automation.invalid_range\t$.automation[0].segments[63].end_sample\n"),
        "the final retained semantic diagnostic is segment 63: {}",
        String::from_utf8_lossy(failure.diagnostic())
    );
}

/// Release-mode half of the boot budget above: the complete production refusal for the maximum
/// dense-invalid document finishes in under one second in the shipped profile. The 234 ms
/// worst-accepted Wasm boot measured for the brief leaves a 4.27x margin under this fixed
/// one-second wall. Debug-mode runner variance (~1.8x observed) makes this assertion a coin flip
/// at P95 on a shared 4-vCPU CI runner in debug, so it runs only in release, nightly, `--ignored`.
#[test]
#[ignore = "release-mode budget; runs nightly"]
fn maximum_document_dense_invalid_boot_finishes_under_one_second_in_release() {
    use std::time::{Duration, Instant};

    let fixture = maximum_document_with_dense_invalid_automation();
    assert_eq!(fixture.bytes.len(), MAXIMUM_DOCUMENT_BYTES as usize);
    let started = Instant::now();
    let failure = AudioWorkletEngineHost::boot(&fixture.bytes, WebBootOptions::default())
        .err()
        .expect("dense invalid automation must refuse");
    let elapsed = started.elapsed();
    assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT);
    assert!(
        elapsed < Duration::from_secs(1),
        "exact-1-MiB dense invalid full boot took {elapsed:?}"
    );
}

#[test]
fn quoted_root_shape_keys_self_configure_without_a_second_parser() {
    for (sample_rate_hz, quantum_frames) in [(48_000, 128), (96_000, 127)] {
        let mut model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("accepted fixture");
        model.sample_rate_hz = sample_rate_hz;
        model.quantum_frames = quantum_frames;
        model.sources[0].frames = u64::from(quantum_frames) * 2;
        model.tracks.truncate(1);
        model.routes.truncate(1);
        let document = canonical_session_json(&model).expect("canonical shape fixture");
        let host = AudioWorkletEngineHost::boot(&document.into_bytes(), WebBootOptions::default())
            .expect("quoted-key document self-configures");
        assert_eq!(host.status().sample_rate_hz, sample_rate_hz);
        assert_eq!(host.status().quantum_frames, quantum_frames);
        assert_eq!(
            host.options().source_ring_frames,
            0,
            "the document-derived ring remains an internal boot choice"
        );
    }
}

#[test]
fn each_boot_option_rule_has_its_own_typed_refusal() {
    let document = one_track_session(128);
    for (options, diagnostic) in [
        (
            WebBootOptions {
                struct_size: BOOT_OPTIONS_BYTES - 1,
                ..boot_options(128)
            },
            b"web.options.struct_size\t$\n".as_slice(),
        ),
        (
            WebBootOptions {
                abi_version: ABI_VERSION - 1,
                ..boot_options(128)
            },
            b"web.options.abi_version\t$\n".as_slice(),
        ),
        (
            WebBootOptions {
                reserved0: 1,
                ..boot_options(128)
            },
            b"web.options.reserved0\t$\n".as_slice(),
        ),
        (
            WebBootOptions {
                source_ring_frames: 129,
                ..boot_options(128)
            },
            b"web.options.source_ring_frames\t$\n".as_slice(),
        ),
    ] {
        let failure = AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .err()
            .expect("invalid option must refuse");
        assert_eq!(failure.result(), RESULT_REFUSED_OPTIONS);
        assert_eq!(failure.diagnostic(), diagnostic);
    }
    AudioWorkletEngineHost::boot(document.as_bytes(), WebBootOptions::default())
        .expect("all-zero options select defaults");
}

/// Owner ruling R5 (#1036): browser boot refuses a document at a former extended research rate
/// (176.4-384 kHz) with the session's typed launch-rate diagnostic, both through the host and
/// through the raw boot export the shipped module exposes, and an AudioContext running at such a
/// rate cannot boot a launch-rate document: the physical shape mismatch refuses first.
#[test]
fn extended_rates_refuse_typed_at_browser_boot() {
    const RATE: &str = "\"sample_rate_hz\": 48000";
    const DIAGNOSTIC: &[u8] = b"sample_rate.unsupported_at_launch\t$.sample_rate_hz\n";
    let launch = one_track_session(128);
    assert_eq!(launch.matches(RATE).count(), 1, "fixture shape drifted");
    for rate in [176_400_u32, 192_000, 352_800, 384_000] {
        let document = launch.replacen(RATE, &format!("\"sample_rate_hz\": {rate}"), 1);
        for options in [
            WebBootOptions::default(),
            WebBootOptions {
                require_sample_rate_hz: rate,
                ..boot_options(128)
            },
        ] {
            let failure = AudioWorkletEngineHost::boot(document.as_bytes(), options)
                .err()
                .expect("an extended-rate document must refuse");
            assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT, "{rate}");
            assert_eq!(failure.diagnostic(), DIAGNOSTIC, "{rate}");
        }

        assert_eq!(miso_engine_web_v1_dispose(0), RESULT_OK);
        assert_eq!(
            crate::ffi::test_boot(document.as_bytes(), WebBootOptions::default()),
            0,
            "a refused boot never returns a live handle"
        );
        assert_eq!(miso_engine_web_v1_boot_result(), RESULT_REFUSED_DOCUMENT);
        let diagnostic_bytes = miso_engine_web_v1_boot_diagnostic_bytes() as usize;
        assert_eq!(
            &crate::ffi::test_staged_document()[..diagnostic_bytes],
            DIAGNOSTIC,
            "{rate}"
        );

        let failure = AudioWorkletEngineHost::boot(
            launch.as_bytes(),
            WebBootOptions {
                require_sample_rate_hz: rate,
                ..boot_options(128)
            },
        )
        .err()
        .expect("a launch-rate document must not boot on an extended-rate context");
        assert_eq!(failure.result(), RESULT_REPREPARE_REQUIRED, "{rate}");
        assert_eq!(failure.diagnostic(), b"host.session.shape\t$\n", "{rate}");
    }
}

#[test]
fn decoded_command_resource_is_exact_for_live_control_modes_without_effects_or_meters() {
    let document = one_track_resource_session(128);
    let parsed = parse_host_session(&document).expect("resource session parse");
    let compiled = compile_host_model(
        &parsed,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("resource session compile");
    let shape = compiled_session_shape(&compiled).expect("resource session shape");
    assert_eq!(shape.effect_count, 0, "resource fixture has no effects");

    let source_id_bytes: u64 = compiled
        .normalized_model()
        .sources
        .iter()
        .map(|source| source.id.as_str().len() as u64)
        .sum();
    let source_control_bytes = control_table_bytes(shape.source_count as usize)
        .expect("source control table projection")
        + source_id_arena_bytes(source_id_bytes as usize).expect("source ID projection");
    let session_model_bytes = compiled.resource_estimate().compiled_model_bytes;
    let expected_decoded_count = (2 * MAXIMUM_COMMAND_RECORDS as usize)
        .checked_add(2 * shape.track_count as usize)
        .expect("decoded record count");
    let decoded_bytes = (expected_decoded_count * size_of::<StagedCommand>()) as u64;
    for (name, options, expected_wire_bytes) in [
        ("off", boot_options(128), 0_u64),
        (
            "on",
            WebBootOptions {
                live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
                live_control_meter_blocks: 0,
                live_control_observation_taps: 0,
                ..boot_options(128)
            },
            (MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES) as u64,
        ),
    ] {
        let projection = project_buffers(
            document.len() as u32,
            shape.sample_rate_hz,
            shape.quantum_frames,
            shape.maximum_source_channels,
            shape
                .longest_source_id_bytes
                .max(shape.longest_track_id_bytes)
                .max(shape.longest_submix_id_bytes)
                .max(shape.longest_route_id_bytes)
                .max(shape.longest_vca_id_bytes),
            options,
            (false, (0, 0)),
        )
        .expect("bridge projection");
        assert_eq!(
            u64::from(projection.command_records) * u64::from(COMMAND_RECORD_BYTES),
            expected_wire_bytes
        );

        let source_ring_frames = if options.source_ring_frames == 0 {
            default_source_ring_frames(shape.sample_rate_hz, shape.quantum_frames)
        } else {
            options.source_ring_frames
        };
        let caps = prepare_caps(&compiled, options, source_ring_frames, u64::MAX);
        let live_controls =
            live_control_request(options, shape.quantum_frames).expect("live-control request");
        let (engine, _) = prepare_host_runtime_with_selected_meters_between_render_calls(
            &compiled,
            &caps,
            &live_controls,
            &[],
        )
        .expect("independent engine preparation");
        assert_eq!(
            engine.report.control_retained_bytes, source_control_bytes,
            "{name}: source-control table and ID arena are exact"
        );
        assert_eq!(
            engine.report.session_model_bytes, session_model_bytes,
            "{name}: compiled model retention is exact"
        );
        assert_eq!(engine.report.observation_retained_bytes, 0);
        assert_eq!(engine.report.builtin_meter_payload_bytes, 0);
        // The prepared plan's state inventory the browser keeps for a successor (#1272), walked
        // from the inventory itself rather than read from the report row.
        let inventory_bytes = engine.inventory.retained_bytes();
        assert!(inventory_bytes > 0, "{name}: the inventory holds its rows");

        let host =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
                panic!("{name}: {}", String::from_utf8_lossy(failure.diagnostic()))
            });
        let ready = host.ready.as_ref().expect("ready ownership");
        assert_eq!(ready.command_decoded.len(), expected_decoded_count);
        assert_eq!(
            size_of_val(ready.command_decoded.as_ref()) as u64,
            decoded_bytes,
            "{name}: typed decoded backing has exact element size"
        );
        assert_eq!(host.command_staging_bytes(), expected_wire_bytes);
        assert!(
            !ready.command_decoded.is_empty(),
            "decoded backing is retained in live-control-off mode"
        );
        assert!(
            ready.effect_controls.is_empty(),
            "no effect control additions"
        );
        assert!(
            ready.effect_observations.is_empty(),
            "no observation additions"
        );
        assert!(ready.meters.is_empty(), "no meter additions");
        assert!(ready.rack_effects.iter().all(|counts| *counts == [0, 0, 0]));
        assert_eq!(host.resources().observation_retained_bytes, 0);

        let input_shadow_bytes = if options.live_control_command_queue_records == 0 {
            0
        } else {
            (shape.track_count as usize * size_of::<BuiltinInputShadow>()) as u64
        };
        let expected_bridge_metadata = projection
            .report
            .bridge_metadata_bytes
            .checked_add(source_control_bytes)
            .and_then(|bytes| bytes.checked_add(session_model_bytes))
            .and_then(|bytes| bytes.checked_add(inventory_bytes))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
            .and_then(|bytes| bytes.checked_add(input_shadow_bytes))
            .expect("bridge metadata arithmetic");
        let expected_bridge_retained = projection
            .report
            .bridge_retained_bytes
            .checked_add(source_control_bytes)
            .and_then(|bytes| bytes.checked_add(session_model_bytes))
            .and_then(|bytes| bytes.checked_add(inventory_bytes))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
            .and_then(|bytes| bytes.checked_add(input_shadow_bytes))
            .expect("bridge retained arithmetic");
        assert_eq!(
            host.resources().bridge_metadata_bytes,
            expected_bridge_metadata
        );
        assert_eq!(
            host.resources().bridge_retained_bytes,
            expected_bridge_retained
        );

        // The 1 MiB live-response capture is deliberately larger than this fixture's decoded
        // array. Keep the largest assertion honest: it includes that independent projection and
        // then folds the engine's own largest allocation into the named maximum.
        let bridge_largest = projection
            .report
            .largest_named_allocation_bytes
            .max(control_table_bytes(shape.source_count as usize).expect("table largest"))
            .max(source_id_arena_bytes(source_id_bytes as usize).expect("ID largest"))
            .max(compiled.resource_estimate().single_allocation_bytes)
            .max(decoded_bytes)
            .max(input_shadow_bytes);
        assert_eq!(
            host.resources().largest_bridge_allocation_bytes,
            bridge_largest
        );
        assert_eq!(
            host.resources().largest_named_allocation_bytes,
            bridge_largest.max(engine.report.largest_engine_allocation_bytes)
        );
    }
}

#[test]
fn exact_retained_total_is_checked_as_one_budget_not_independent_caps() {
    let document = one_track_session(128);
    let source_ring_frames = 1 << 20;
    for options in [
        WebBootOptions {
            source_ring_frames,
            ..boot_options(128)
        },
        WebBootOptions {
            source_ring_frames,
            live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
            live_control_meter_blocks: 0,
            live_control_observation_taps: 0,
            ..boot_options(128)
        },
    ] {
        let baseline =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("baseline boot");
        let ready = baseline.ready.as_ref().expect("ready ownership");
        let dense_effect_table_bytes =
            (ready.effect_controls.len() * size_of::<Option<EffectControlProducer>>()) as u64;
        assert_eq!(
            size_of_val(ready.effect_controls.as_ref()) as u64,
            dense_effect_table_bytes,
            "the browser owns the actual dense effect table in both live-control modes"
        );
        assert_eq!(
            ready.effect_controls.len(),
            1,
            "one-track fixture has one effect"
        );
        assert!(
            baseline.resources().bridge_retained_bytes >= dense_effect_table_bytes,
            "live-control-off preparation still charges the dense replacement table"
        );
        let exact = exact_retained_report_total(baseline.resources());
        drop(baseline);
        AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                maximum_memory_bytes: exact,
                ..options
            },
        )
        .expect("exact aggregate budget must accept");
        let failure = AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                maximum_memory_bytes: exact - 1,
                ..options
            },
        )
        .err()
        .expect("one byte below exact aggregate must refuse");
        assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
        assert_eq!(
            failure.diagnostic(),
            format!(
                "host.budget.retained_exact\t$.maximum_memory_bytes[exact_bytes={exact},budget_bytes={}]\n",
                exact - 1
            )
            .as_bytes()
        );
    }
}

#[test]
fn retained_projection_budget_diagnostic_names_projected_bytes() {
    let document = one_track_session(128);
    let options = WebBootOptions {
        source_ring_frames: 1 << 20,
        ..boot_options(128)
    };
    let projection = retained_projection(document.as_bytes(), options);
    let budget = projection - 1;
    let failure = AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: budget,
            ..options
        },
    )
    .err()
    .expect("one byte below retained projection must refuse before preparation");
    assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
    assert_eq!(
        failure.diagnostic(),
        format!(
            "host.budget.retained_projection\t$.maximum_memory_bytes[projected_bytes={projection},budget_bytes={budget}]\n"
        )
        .as_bytes()
    );
}

#[test]
fn representative_retained_projection_tracks_the_post_prepare_exact_aggregate() {
    // #239 ruling 5459221452 authorizes an A5 boundary that deliberately leaves the
    // transactionally rolled-back preparation delta out of the pre-prepare projection. These
    // three shipped shapes pin that drift: the largest measured `gap / projection` is the
    // console's 2,679,317 / 409,396 = 6.545. Seven leaves 6.9% headroom for harmless
    // allocator/layout movement while still making any material projector drift an explicit
    // review and re-pin. In particular, dropping a projected retained row cannot hide behind the
    // deliberately broad rollback allowance.
    const MAXIMUM_PREPARATION_GAP_MULTIPLIER: u64 = 7;
    let representatives = [
        (
            "identity-one-track",
            one_track_session(128),
            WebBootOptions {
                source_ring_frames: 128,
                ..boot_options(128)
            },
        ),
        (
            "parametric-eq-nine-track",
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json").to_owned(),
            WebBootOptions {
                source_ring_frames: 512,
                ..boot_options(128)
            },
        ),
        (
            "console-sixty-four-track",
            include_str!("../../../fixtures/session/v1/console-sixty-four-track.json").to_owned(),
            WebBootOptions {
                source_ring_frames: 512,
                live_control_command_queue_records: u64::from(DEFAULT_COMMAND_QUEUE_RECORDS),
                live_control_meter_blocks: u64::from(DEFAULT_METER_BLOCKS),
                ..boot_options(128)
            },
        ),
    ];

    for (name, document, options) in representatives {
        let projection = retained_projection(document.as_bytes(), options);
        let host =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
                panic!("{name}: {}", String::from_utf8_lossy(failure.diagnostic()))
            });
        let exact = exact_retained_report_total(host.resources());
        let gap = exact.checked_sub(projection).unwrap_or_else(|| {
            panic!("{name}: projection {projection} exceeds exact retained aggregate {exact}")
        });
        let maximum_gap = projection
            .checked_mul(MAXIMUM_PREPARATION_GAP_MULTIPLIER)
            .expect("bounded representative projection");
        assert!(
            gap <= maximum_gap,
            "{name}: exact/projection drift gap {gap} exceeds documented bound {maximum_gap} (projection {projection}, exact {exact})"
        );
    }
}

#[test]
fn source_backpressure_seek_render_and_stable_output_are_bounded() {
    let quantum = 128_usize;
    let document = one_track_session(quantum as u32);
    let options = WebBootOptions {
        source_ring_frames: quantum as u32,
        ..boot_options(quantum as u32)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
    let source_id_ptr = host.source_id_mut().expect("ID").as_ptr();
    let source_pcm_ptr = host.source_pcm_mut().expect("PCM").as_ptr();
    let output_ptr = host.output_pcm().expect("output").as_ptr();
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
    assert_eq!(host.seek_source(b"fixture-source", 3, 0), RESULT_OK);
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.status().rendered_quanta, 2);
}

#[test]
fn paused_seek_recycles_full_internal_queue_before_first_target_quantum() {
    let quantum = 128;
    let document = identity_session(quantum, 480_000);
    let options = WebBootOptions {
        source_ring_frames: 512,
        ..boot_options(quantum)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap();
    let old = [0.25; 128];
    for block in 0..4 {
        assert_eq!(
            host.submit_source(
                b"fixture-source",
                1,
                block * 128,
                48_000,
                &[&old, &old],
                quantum,
                false
            ),
            RESULT_OK
        );
    }
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            512,
            48_000,
            &[&old, &old],
            quantum,
            false
        ),
        RESULT_BACKPRESSURE
    );
    assert_eq!(
        host.seek_source(b"unknown", 2, 10_000),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.seek_source(b"fixture-source", 2, 10_000), RESULT_OK);
    assert_eq!(host.status().next_absolute_sample, 0);
    assert_eq!(host.status().rendered_quanta, 0);
    let left = core::array::from_fn::<_, 128, _>(|index| (index + 1) as f32 / 256.0);
    let right = core::array::from_fn::<_, 128, _>(|index| -(index as f32 + 1.0) / 512.0);
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            2,
            10_000,
            48_000,
            &[&left, &right],
            quantum,
            false
        ),
        RESULT_OK
    );
    assert_eq!(host.render_next(), RESULT_OK);
    let output = host.output_pcm().unwrap();
    assert_eq!(&output[..128], &left);
    assert_eq!(&output[128..256], &right);
    assert_eq!(host.status().next_absolute_sample, 128);
}

#[test]
fn output_mismatch_and_disposal_are_sticky_and_idempotent() {
    let mut host = prepared_host(64);
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
    let mut host = prepared_host(128);
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
    let document = one_track_session(quantum as u32);
    let options = WebBootOptions {
        source_ring_frames: quantum as u32,
        ..boot_options(quantum as u32)
    };
    let host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");

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
    let document = identity_session(QUANTUM, length_samples);
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), boot_options(QUANTUM))
        .expect("boot with derived ring");

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

    let document = identity_session(QUANTUM, 256);
    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");

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
    let native = engine::hex_lower(&digest.finalize());

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
/// -> the assertion fails here, and moving any live-control stage's drain to after the audio makes
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
    let document = include_str!("../tests/browser-v1/command-session.json");
    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        live_control_command_queue_records: u64::from(DEPTH),
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
    assert_eq!(host.live_control_tracks().len(), 1);

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
    assert_eq!(
        submit_prepared_eq_parameter(&mut host, 0, 0, 1, 0, 4, -12.0),
        RESULT_OK
    );
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
    let native = engine::hex_lower(&digest.finalize());

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

#[test]
fn real_compressor_id_eight_rejects_atomically_at_the_web_command_boundary() {
    const QUANTUM: u32 = 128;
    let mut baseline = compressor_live_control_host(QUANTUM);
    let mut candidate = compressor_live_control_host(QUANTUM);

    feed_compressor_block(&mut baseline, QUANTUM, 0);
    feed_compressor_block(&mut candidate, QUANTUM, 0);
    let session_before = candidate
        .ready
        .as_ref()
        .expect("ready compressor session")
        .session
        .canonical_json()
        .to_owned();
    let resources_before = *candidate.resources();
    let tracks_before = candidate.live_control_tracks().to_vec();
    let sources_before = candidate.session_source_count();

    // The retired compressor address is rejected while a valid makeup-gain update shares the
    // same staged batch. The first-pass validator must therefore admit neither record.
    stage_command(
        &mut candidate,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        8,
        0,
        [12.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut candidate,
        1,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        6,
        0,
        [12.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(candidate.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        candidate.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
    );
    assert_eq!(candidate.command_report().rejected_index, 0);
    assert_eq!(candidate.command_report().admitted, 0);
    assert_eq!(
        candidate
            .ready
            .as_ref()
            .expect("ready compressor session")
            .session
            .canonical_json(),
        session_before
    );
    assert_eq!(*candidate.resources(), resources_before);
    assert_eq!(candidate.live_control_tracks(), tracks_before.as_slice());
    assert_eq!(candidate.session_source_count(), sources_before);

    // A valid same-batch update would change makeup gain. Identical next-block PCM proves that
    // the valid record was not admitted behind the rejected retired address.
    feed_compressor_block(&mut baseline, QUANTUM, 1);
    feed_compressor_block(&mut candidate, QUANTUM, 1);
    assert_eq!(candidate.output_pcm(), baseline.output_pcm());
}

#[test]
fn real_multiband_id_two_rejects_without_ack_or_revision_change() {
    const QUANTUM: u32 = 128;
    let mut baseline = multiband_live_control_host(QUANTUM);
    let mut candidate = multiband_live_control_host(QUANTUM);

    feed_compressor_block(&mut baseline, QUANTUM, 0);
    feed_compressor_block(&mut candidate, QUANTUM, 0);
    let session_before = candidate
        .ready
        .as_ref()
        .expect("ready multiband session")
        .session
        .canonical_json()
        .to_owned();
    let resources_before = *candidate.resources();
    let tracks_before = candidate.live_control_tracks().to_vec();
    let sources_before = candidate.session_source_count();
    let status_before = *candidate.status();

    // A valid low-threshold update shares one transaction with the retired lookahead ID 2.
    // The actual multiband descriptor must reject the whole batch before either record is
    // acknowledged or the control/render revision advances.
    stage_command(
        &mut candidate,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        3,
        0,
        [-12.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut candidate,
        1,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        2,
        0,
        [1_000.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(candidate.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        candidate.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
    );
    assert_eq!(candidate.command_report().rejected_index, 1);
    assert_eq!(candidate.command_report().admitted, 0);
    assert_eq!(
        candidate.command_report().applied_at_sample,
        status_before.next_absolute_sample
    );
    assert_eq!(
        candidate
            .ready
            .as_ref()
            .expect("ready multiband session")
            .session
            .canonical_json(),
        session_before
    );
    assert_eq!(*candidate.resources(), resources_before);
    assert_eq!(candidate.live_control_tracks(), tracks_before.as_slice());
    assert_eq!(candidate.session_source_count(), sources_before);
    assert_eq!(
        (
            candidate.status().next_absolute_sample,
            candidate.status().rendered_quanta,
        ),
        (
            status_before.next_absolute_sample,
            status_before.rendered_quanta,
        )
    );

    // The valid threshold update was not admitted behind ID 2: the next block remains identical
    // to the untouched host, including the render clock.
    feed_compressor_block(&mut baseline, QUANTUM, 1);
    feed_compressor_block(&mut candidate, QUANTUM, 1);
    assert_eq!(candidate.output_pcm(), baseline.output_pcm());
    assert_eq!(
        (
            candidate.status().next_absolute_sample,
            candidate.status().rendered_quanta,
        ),
        (
            baseline.status().next_absolute_sample,
            baseline.status().rendered_quanta,
        )
    );
}

#[test]
fn real_gate_id_eight_rejects_atomically_at_the_web_command_boundary() {
    const QUANTUM: u32 = 128;
    let mut baseline = gate_live_control_host(QUANTUM);
    let mut candidate = gate_live_control_host(QUANTUM);

    feed_gate_block(&mut baseline, 0);
    feed_gate_block(&mut candidate, 0);
    let session_before = candidate
        .ready
        .as_ref()
        .expect("ready gate session")
        .session
        .canonical_json()
        .to_owned();
    let resources_before = *candidate.resources();
    let tracks_before = candidate.live_control_tracks().to_vec();
    let sources_before = candidate.session_source_count();
    let status_before = *candidate.status();

    // The actual observation-frame fixture addresses gate on track t2, dynamic rack (wire rack 1), slot 0.
    // A valid threshold update shares the batch with the retired lookahead ID 8. Atomic admission
    // must reject both before changing the prepared model or the render/control revision.
    stage_command(
        &mut candidate,
        0,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        2,
        0,
        1,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut candidate,
        1,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        2,
        0,
        8,
        0,
        [2.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(candidate.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_eq!(
        candidate.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
    );
    assert_eq!(candidate.command_report().rejected_index, 1);
    assert_eq!(candidate.command_report().admitted, 0);
    assert_eq!(
        candidate
            .ready
            .as_ref()
            .expect("ready gate session")
            .session
            .canonical_json(),
        session_before
    );
    assert_eq!(*candidate.resources(), resources_before);
    assert_eq!(candidate.live_control_tracks(), tracks_before.as_slice());
    assert_eq!(candidate.session_source_count(), sources_before);
    let status_after = *candidate.status();
    assert_eq!(
        (
            status_after.state,
            status_after.backend,
            status_after.sample_rate_hz,
            status_after.quantum_frames,
            status_after.next_absolute_sample,
            status_after.rendered_quanta,
            status_after.reserved,
        ),
        (
            status_before.state,
            status_before.backend,
            status_before.sample_rate_hz,
            status_before.quantum_frames,
            status_before.next_absolute_sample,
            status_before.rendered_quanta,
            status_before.reserved,
        ),
        "rejection changes only the typed last-result report",
    );

    // The valid threshold record was not admitted behind ID 8: the next rendered PCM and render
    // clock remain identical to the untouched actual-gate host.
    feed_gate_block(&mut baseline, 1);
    feed_gate_block(&mut candidate, 1);
    assert_eq!(candidate.output_pcm(), baseline.output_pcm());
    assert_eq!(
        (
            candidate.status().next_absolute_sample,
            candidate.status().rendered_quanta,
        ),
        (
            baseline.status().next_absolute_sample,
            baseline.status().rendered_quanta,
        )
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

/// Stage one EQ edit with the same control-plane target preparation and opaque companion that the
/// browser SDK submits. The render-side producer receives only the already-designed target.
fn stage_prepared_eq_parameter(
    host: &mut AudioWorkletEngineHost,
    wire_index: usize,
    track_index: u32,
    rack: u8,
    effect_index: u32,
    parameter_id: u32,
    value: f32,
) {
    stage_command(
        host,
        wire_index,
        COMMAND_EFFECT_PARAM,
        rack,
        2,
        track_index,
        effect_index,
        parameter_id,
        0,
        [value, 0.0, 0.0, 0.0],
    );
    let preparer = EqTargetPreparer::new(
        host_core::parametric_eq_target_preparation_factory().expect("EQ capability"),
    )
    .expect("EQ target preparer");
    assert_eq!(
        host.copy_eq_target_config(track_index, u32::from(rack), effect_index),
        RESULT_OK
    );
    let config = host.eq_target_config().expect("EQ target config");
    let generation = u64::from_le_bytes(config[16..24].try_into().expect("generation"));
    let base_revision = u64::from_le_bytes(config[24..32].try_into().expect("revision"));
    let mut seeds = [0.0_f32; EQ_VALUE_COUNT];
    for (index, seed) in seeds.iter_mut().enumerate() {
        let offset = 32 + index * 4;
        *seed = f32::from_bits(u32::from_le_bytes(
            config[offset..offset + 4].try_into().expect("seed"),
        ));
    }
    let mut targets = [PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Both,
        words: [0; PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_TARGET_CAPACITY];
    let (_, target_count) = preparer
        .prepare(
            48_000,
            &seeds,
            &[EqTargetEdit {
                parameter_id,
                channel: ParameterChannel::Both,
                value,
            }],
            &mut targets,
        )
        .expect("prepared EQ target");
    assert_eq!(target_count, 1, "one edit touches one EQ section");
    let companion = host.prepared_companion_mut().expect("prepared companion");
    companion.fill(0);
    companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
    companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    companion[8..16].copy_from_slice(&generation.to_le_bytes());
    companion[16..20].copy_from_slice(&(target_count as u32).to_le_bytes());
    let record = &mut companion[24..104];
    record[0..4].copy_from_slice(&track_index.to_le_bytes());
    record[4..8].copy_from_slice(&u32::from(rack).to_le_bytes());
    record[8..12].copy_from_slice(&effect_index.to_le_bytes());
    record[16..24].copy_from_slice(&base_revision.to_le_bytes());
    record[24..28].copy_from_slice(&targets[0].slot.to_le_bytes());
    record[28..32].copy_from_slice(&2_u32.to_le_bytes());
    for (index, word) in targets[0].words.iter().enumerate() {
        let offset = 32 + index * 4;
        record[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
    }
}

fn submit_prepared_eq_parameter(
    host: &mut AudioWorkletEngineHost,
    wire_index: usize,
    track_index: u32,
    rack: u8,
    effect_index: u32,
    parameter_id: u32,
    value: f32,
) -> u32 {
    stage_prepared_eq_parameter(
        host,
        wire_index,
        track_index,
        rack,
        effect_index,
        parameter_id,
        value,
    );
    host.submit_prepared_commands(1, 104)
}

fn input_filter_live_control_host(quantum: u32, queue_depth: u64) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(&identity_session(quantum, u64::from(quantum) * 4))
        .expect("accepted identity fixture");
    let builtins = &mut model.tracks[0].builtins;
    builtins.left.hpf_hz = 100.0;
    builtins.left.lpf_hz = 1_000.0;
    builtins.right.hpf_hz = 100.0;
    builtins.right.lpf_hz = 1_000.0;
    let document = canonical_session_json(&model).expect("canonical input filter fixture");
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: quantum,
            live_control_command_queue_records: queue_depth,
            ..boot_options(quantum)
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "input filter boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

#[cfg(feature = "test-support")]
fn effect_input_filter_live_control_host(quantum: u32, queue_depth: u64) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
        .expect("accepted command fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 4;
    let builtins = &mut model.tracks[0].builtins;
    builtins.left.hpf_hz = 100.0;
    builtins.left.lpf_hz = 1_000.0;
    builtins.right.hpf_hz = 100.0;
    builtins.right.lpf_hz = 1_000.0;
    let document = canonical_session_json(&model).expect("canonical mixed fixture");
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: quantum,
            live_control_command_queue_records: queue_depth,
            ..boot_options(quantum)
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "mixed input/EQ boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

fn stage_prepared_input_filter(
    host: &mut AudioWorkletEngineHost,
    wire_index: usize,
    track_index: u32,
    hpf_hz: f32,
    lpf_hz: f32,
) -> u32 {
    stage_command(
        host,
        wire_index,
        COMMAND_INPUT_FILTERS,
        255,
        2,
        track_index,
        0,
        0,
        0,
        [hpf_hz, lpf_hz, 0.0, 0.0],
    );
    assert_eq!(host.copy_input_filter_config(track_index), RESULT_OK);
    let config = host.eq_target_config().expect("input filter config");
    let generation = u64::from_le_bytes(config[16..24].try_into().expect("generation"));
    let base_revision = u64::from_le_bytes(config[24..32].try_into().expect("revision"));
    let sample_rate = u32::from_le_bytes(config[8..12].try_into().expect("sample rate"));
    let mut seeds = [0.0_f32; 4];
    for (index, seed) in seeds.iter_mut().enumerate() {
        let offset = 32 + index * 4;
        *seed = f32::from_bits(u32::from_le_bytes(
            config[offset..offset + 4].try_into().expect("seed"),
        ));
    }
    let mut targets = [PreparedInputFilterTarget {
        lanes: BuiltinLaneSelector::Both,
        section: 0,
        pair: [0.0; 2],
        coefficients: [0.0; 6],
    }; 4];
    let (_, target_count) = InputFilterPreparer
        .prepare(
            sample_rate,
            &seeds,
            &[InputFilterEdit {
                parameter_id: 0,
                channel: ParameterChannel::Both,
                value0: hpf_hz,
                value1: lpf_hz,
            }],
            &mut targets,
        )
        .expect("prepared input filter target");
    assert_eq!(target_count, 2, "whole pair touches both sections");
    let companion_bytes = 24 + target_count * 80;
    let companion = host.prepared_companion_mut().expect("prepared companion");
    companion.fill(0);
    companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
    companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    companion[8..16].copy_from_slice(&generation.to_le_bytes());
    companion[16..20].copy_from_slice(&(target_count as u32).to_le_bytes());
    for (index, target) in targets[..target_count].iter().enumerate() {
        let record = &mut companion[24 + index * 80..104 + index * 80];
        record[0..4].copy_from_slice(&track_index.to_le_bytes());
        record[4..8].copy_from_slice(&255_u32.to_le_bytes());
        record[8..12].copy_from_slice(&0_u32.to_le_bytes());
        record[16..24].copy_from_slice(&base_revision.to_le_bytes());
        record[24..28].copy_from_slice(&target.section.to_le_bytes());
        record[28..32].copy_from_slice(
            &match target.lanes {
                BuiltinLaneSelector::Left => 0_u32,
                BuiltinLaneSelector::Right => 1,
                BuiltinLaneSelector::Both => 2,
            }
            .to_le_bytes(),
        );
        record[32..36].copy_from_slice(&target.pair[0].to_bits().to_le_bytes());
        record[36..40].copy_from_slice(&target.pair[1].to_bits().to_le_bytes());
        for (word, value) in target.coefficients.iter().copied().enumerate() {
            record[56 + word * 4..60 + word * 4].copy_from_slice(&value.to_bits().to_le_bytes());
        }
    }
    companion_bytes as u32
}

/// Build one companion for the existing EQ owner and the builtin input owner. Each helper is
/// called separately so both targets are prepared from their own committed revision, then the
/// already designed records are coalesced into the shared companion wire image.
#[cfg(feature = "test-support")]
fn stage_mixed_prepared_eq_and_input_filter(host: &mut AudioWorkletEngineHost) -> u32 {
    stage_prepared_eq_parameter(host, 0, 0, 1, 0, 4, -12.0);
    let eq_companion = host.prepared_companion_mut().expect("prepared companion")[..104].to_vec();

    let input_bytes = stage_prepared_input_filter(host, 1, 0, 300.0, 2_000.0) as usize;
    let input_companion =
        host.prepared_companion_mut().expect("prepared companion")[..input_bytes].to_vec();
    let input_target_bytes = input_bytes.checked_sub(24).expect("input companion header");
    let total_bytes = 24 + 80 + input_target_bytes;
    let companion = host.prepared_companion_mut().expect("prepared companion");
    companion.fill(0);
    companion[..24].copy_from_slice(&eq_companion[..24]);
    companion[16..20].copy_from_slice(&3_u32.to_le_bytes());
    companion[24..104].copy_from_slice(&eq_companion[24..104]);
    companion[104..total_bytes].copy_from_slice(&input_companion[24..input_bytes]);
    total_bytes as u32
}

/// A live-control host over the browser identity fixture: one track, unity everything, one-quantum
/// ring.
fn live_control_host(quantum: u32, meter_blocks: u64) -> AudioWorkletEngineHost {
    live_control_host_at_rate(48_000, quantum, meter_blocks)
}

fn live_control_host_at_rate(
    sample_rate_hz: u32,
    quantum: u32,
    meter_blocks: u64,
) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(&identity_session(quantum, u64::from(quantum) * 64))
        .expect("identity model");
    model.sample_rate_hz = sample_rate_hz;
    let document = canonical_session_json(&model).expect("canonical identity session");
    let options = WebBootOptions {
        require_sample_rate_hz: sample_rate_hz,
        require_quantum_frames: quantum,
        source_ring_frames: quantum,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        live_control_meter_blocks: meter_blocks,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("live-control boot")
}

#[cfg(feature = "test-support")]
fn paired_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 2;
    let template = model.tracks[0].clone();
    model.tracks.clear();
    for index in 0..9 {
        let mut track = template.clone();
        track.id = session::StableId::parse(&format!("track-{index}")).expect("track id");
        model.tracks.push(track);
    }
    model.routes[0].source = session::RouteSource::Track {
        track_id: session::StableId::parse("track-0").expect("route track"),
        tap: session::SendTap::PostPan,
    };
    let document = canonical_session_json(&model).expect("canonical paired fixture");
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: quantum,
        source_ring_frames: quantum,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("paired live-control boot")
}

fn meter_tail_host(sample_rate_hz: u32, quantum: u32) -> AudioWorkletEngineHost {
    meter_tail_host_for_blocks(sample_rate_hz, quantum, 1, 4)
}

fn meter_tail_host_for_blocks(
    sample_rate_hz: u32,
    quantum: u32,
    meter_blocks: u64,
    source_blocks: u64,
) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.sample_rate_hz = sample_rate_hz;
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * source_blocks;
    let template = model.tracks[0].clone();
    model.tracks.clear();
    for index in 0..9 {
        let mut track = template.clone();
        track.id = session::StableId::parse(&format!("track-{index}")).expect("track id");
        model.tracks.push(track);
    }
    model.routes[0].source = session::RouteSource::Track {
        track_id: session::StableId::parse("track-0").expect("route track"),
        tap: session::SendTap::PostPan,
    };
    let document = canonical_session_json(&model).expect("canonical meter-tail fixture");
    let options = WebBootOptions {
        require_sample_rate_hz: sample_rate_hz,
        require_quantum_frames: quantum,
        source_ring_frames: quantum,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        live_control_meter_blocks: meter_blocks,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("meter-tail boot")
}

/// Feed one full quantum of a constant left plane and render it.
fn feed_and_render(host: &mut AudioWorkletEngineHost, generation: u64, block: u64, value: f32) {
    let quantum = host.status().quantum_frames as usize;
    let samples = vec![value; quantum];
    let planes: [&[f32]; 2] = [&samples, &samples];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            generation,
            block * quantum as u64,
            host.status().sample_rate_hz,
            &planes,
            quantum as u32,
            false,
        ),
        RESULT_OK,
    );
    assert_eq!(host.render_next(), RESULT_OK);
}

/// #514 IO7: command-free blocks skip the dense admission-counter reset while command ownership
/// and the existing transactional capacity contract remain exact.
#[test]
fn idle_render_skips_admission_counter_clear_without_losing_queue_credit() {
    const QUANTUM: u32 = 128;
    const QUEUE_DEPTH: usize = DEFAULT_COMMAND_QUEUE_RECORDS as usize;

    let mut host = live_control_host(QUANTUM, 0);
    reset_admission_counter_clear();
    for block in 0..3_u64 {
        feed_and_render(&mut host, 1, block, 0.25);
    }
    assert_eq!(admission_counter_clear_stats(), (0, 0));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // Repeated records on one destination and one record on a second destination preserve the
    // exact per-queue counts until the successful render drains them.
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
        [1.0, 0.0, 0.0, 1.0],
    );
    stage_command(
        &mut host,
        1,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_OK);
    let application_sample = 3 * u64::from(QUANTUM);
    assert_eq!(
        *host.command_report(),
        WebCommandReport {
            struct_size: COMMAND_REPORT_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_OK,
            reason: COMMAND_REASON_NONE,
            rejected_index: 0,
            admitted: 2,
            applied_at_sample: application_sample,
            reserved: [0; 2],
        }
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").in_flight[..3],
        [2, 0, 0]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);

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
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        *host.command_report(),
        WebCommandReport {
            struct_size: COMMAND_REPORT_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_OK,
            reason: COMMAND_REASON_NONE,
            rejected_index: 0,
            admitted: 1,
            applied_at_sample: application_sample,
            reserved: [0; 2],
        }
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").in_flight[..3],
        [2, 1, 0]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);

    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 3, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 3));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    feed_and_render(&mut host, 1, 4, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 3));

    // A healthy drained host can refill a queue to its exact old capacity; one more record is the
    // existing typed refusal and leaves the accepted prefix's ownership untouched.
    for index in 0..QUEUE_DEPTH {
        stage_command(
            &mut host,
            index,
            COMMAND_MATRIX,
            255,
            255,
            0,
            0,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
        );
    }
    assert_eq!(
        host.submit_commands(DEFAULT_COMMAND_QUEUE_RECORDS),
        RESULT_OK
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").in_flight[0],
        DEFAULT_COMMAND_QUEUE_RECORDS
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);
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
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_BACKPRESSURE);
    assert_eq!(
        *host.command_report(),
        WebCommandReport {
            struct_size: COMMAND_REPORT_BYTES,
            abi_version: ABI_VERSION,
            result: RESULT_BACKPRESSURE,
            reason: COMMAND_REASON_BACKPRESSURE,
            rejected_index: 0,
            admitted: 0,
            applied_at_sample: 5 * u64::from(QUANTUM),
            reserved: [0; 2],
        }
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").in_flight[..3],
        [DEFAULT_COMMAND_QUEUE_RECORDS, 0, 0]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);

    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 5, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 3));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // The successful drain renews the full old capacity. Drain that refill, then prove the next
    // command-free block performs no physical clear.
    assert_eq!(
        host.submit_commands(DEFAULT_COMMAND_QUEUE_RECORDS),
        RESULT_OK
    );
    assert_eq!(
        host.command_report().admitted,
        DEFAULT_COMMAND_QUEUE_RECORDS
    );
    assert_eq!(
        host.command_report().applied_at_sample,
        6 * u64::from(QUANTUM)
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").in_flight[..3],
        [DEFAULT_COMMAND_QUEUE_RECORDS, 0, 0]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);
    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 6, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 3));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 7, 0.25);
    assert_eq!(admission_counter_clear_stats(), (0, 0));

    // Malformed and backpressured submissions on an idle host cannot create pending ownership.
    let mut refused = live_control_host(QUANTUM, 0);
    stage_command(
        &mut refused,
        0,
        COMMAND_FADER_DB,
        255,
        9,
        0,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(refused.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert!(
        !refused
            .ready
            .as_ref()
            .expect("ready")
            .has_in_flight_commands
    );
    assert!(
        refused
            .ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    for index in 0..=QUEUE_DEPTH {
        stage_command(
            &mut refused,
            index,
            COMMAND_MATRIX,
            255,
            255,
            0,
            0,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
        );
    }
    assert_eq!(
        refused.submit_commands(DEFAULT_COMMAND_QUEUE_RECORDS + 1),
        RESULT_BACKPRESSURE
    );
    assert!(
        !refused
            .ready
            .as_ref()
            .expect("ready")
            .has_in_flight_commands
    );
    assert!(
        refused
            .ready
            .as_ref()
            .expect("ready")
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // A refusal after accepted records preserves those records and the dirty flag.
    stage_command(
        &mut refused,
        0,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(refused.submit_commands(1), RESULT_OK);
    stage_command(
        &mut refused,
        0,
        COMMAND_FADER_DB,
        255,
        9,
        0,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(refused.submit_commands(1), RESULT_INVALID_ARGUMENT);
    assert!(
        refused
            .ready
            .as_ref()
            .expect("ready")
            .has_in_flight_commands
    );
    assert_eq!(refused.ready.as_ref().expect("ready").in_flight[0], 1);

    // Solo can be a successful zero-emission submission; it must not mark the counters dirty.
    let mut solo = live_control_host(QUANTUM, 0);
    feed_and_render(&mut solo, 1, 0, -0.5);
    stage_mute(&mut solo, 0, 0, true, 0);
    assert_eq!(solo.submit_commands(1), RESULT_OK);
    feed_and_render(&mut solo, 1, 1, -0.5);
    reset_admission_counter_clear();
    stage_solo(&mut solo, 0, 0, true, QUANTUM);
    assert_eq!(solo.submit_commands(1), RESULT_OK);
    assert_eq!(solo.command_report().admitted, 1);
    assert!(!solo.ready.as_ref().expect("ready").has_in_flight_commands);
    feed_and_render(&mut solo, 1, 2, -0.5);
    assert_eq!(admission_counter_clear_stats(), (0, 0));

    // A render failure is sticky: pending ownership and the untouched clear counters survive the
    // failed block and the subsequent WRONG_STATE re-entry.
    let mut failed = live_control_host(QUANTUM, 0);
    stage_command(
        &mut failed,
        0,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(failed.submit_commands(1), RESULT_OK);
    let before = failed.ready.as_ref().expect("ready").in_flight.to_vec();
    assert!(failed.ready.as_ref().expect("ready").has_in_flight_commands);
    failed
        .buffers
        .as_mut()
        .expect("prepared buffers")
        .output_pcm
        .fill(-1.0);
    reset_admission_counter_clear();
    failed.status.next_absolute_sample = u64::MAX;
    assert_eq!(failed.render_next(), RESULT_RENDER_REJECTED);
    assert_eq!(failed.status().state, STATE_FAILED);
    assert!(
        failed
            .output_pcm()
            .expect("prepared output")
            .iter()
            .all(|sample| sample.to_bits() == 0),
        "a failed render with pending commands emits positive-zero silence"
    );
    assert_eq!(failed.diagnostic(), b"web.render.rejected\t$\n");
    assert_eq!(admission_counter_clear_stats(), (0, 0));
    assert_eq!(
        &failed.ready.as_ref().expect("ready").in_flight[..],
        &before[..]
    );
    assert!(failed.ready.as_ref().expect("ready").has_in_flight_commands);
    assert_eq!(failed.render_next(), RESULT_WRONG_STATE);
    assert_eq!(failed.status().state, STATE_FAILED);
    assert_eq!(failed.diagnostic(), b"web.render.rejected\t$\n");
    assert_eq!(admission_counter_clear_stats(), (0, 0));
    assert_eq!(
        &failed.ready.as_ref().expect("ready").in_flight[..],
        &before[..]
    );
    assert!(failed.ready.as_ref().expect("ready").has_in_flight_commands);
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
/// Red mutation: move the `drain_controls` call in
/// `LiveControlMatrixProcessor::process` to *after* `self.matrix.process(block)` -> the reported
/// sample is one block early and `at_applied` still renders the pre-command value.
#[test]
fn command_ack_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut host = live_control_host(QUANTUM, 0);
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

/// #430 gate 2: WebEngine declares between-render delivery for both actual owners. A fader and
/// matrix command admitted together must therefore drain from their distinct queues onto the
/// same acknowledged first sample before the paired stage decides whether it is settled.
#[test]
fn paired_fader_and_matrix_commands_share_the_acknowledged_application_sample() {
    const QUANTUM: u32 = 128;
    let mut host = live_control_host(QUANTUM, 0);
    let quantum = QUANTUM as usize;

    feed_and_render(&mut host, 1, 0, 0.5);
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
    stage_command(
        &mut host,
        1,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [0.5, 0.0, 0.0, 1.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_OK);
    let report = *host.command_report();
    assert_eq!(report.admitted, 2);
    assert_eq!(report.applied_at_sample, u64::from(QUANTUM));

    feed_and_render(&mut host, 1, 1, 0.5);
    let output = host.output_pcm().expect("output");
    let expected_left = output[0];
    assert!(expected_left > 0.11 && expected_left < 0.14);
    assert!(
        output[..quantum]
            .iter()
            .all(|sample| sample.to_bits() == expected_left.to_bits()),
        "both immediate records apply before sample one"
    );
    assert!(
        output[quantum..]
            .iter()
            .all(|sample| sample.to_bits() == (expected_left * 2.0).to_bits()),
        "the matrix leaves the right plane at the fader output"
    );
}

/// #459: the acknowledgement and the actual serialized composite belong to one render call.
#[cfg(feature = "test-support")]
#[test]
fn acknowledged_pair_render_records_the_same_live_dispatch() {
    const QUANTUM: u32 = 128;
    let mut host = paired_live_control_host(QUANTUM);
    feed_and_render(&mut host, 1, 0, 0.5);
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        8,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut host,
        1,
        COMMAND_MATRIX,
        255,
        255,
        8,
        0,
        0,
        0,
        [0.5, 0.0, 0.0, 1.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_OK);
    let report = *host.command_report();
    assert_eq!(report.admitted, 2);
    assert_eq!(report.applied_at_sample, u64::from(QUANTUM));

    // The nine tracks share this bounded source ring, so explicitly seek/refill the second
    // render's source generation after the first block has exercised every reader.
    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    builtins_compiler::test_only_reset_fader_matrix_witness();
    feed_and_render(&mut host, 2, 0, 0.5);
    let witness = builtins_compiler::test_only_fader_matrix_witness();
    // #916 (608f0379, scope amendment 9c762d7c) made the session Output dedicated storage, so
    // track 0 pairs: tracks 0-7 now take one fused eight-lane bank call before the track-8 tail.
    // `process_members` sums both calls. The per-call witness is the first output sample, which
    // the last call records: only track 8 carries the acknowledged -6 dB and [0.5, 0, 0, 1].
    assert_eq!(
        witness.process_calls, 2,
        "one eight-lane bank call over tracks 0-7 and the scalar track-8 tail"
    );
    assert_eq!(
        witness.process_members, 9,
        "eight bank members plus the one scalar track-8 tail member"
    );
    assert_eq!(witness.fader_records_drained, 1);
    assert_eq!(witness.matrix_records_drained, 1);
    let selected_left = f32::from_bits(witness.first_left_bits);
    let selected_right = f32::from_bits(witness.first_right_bits);
    assert!(
        selected_left > 0.11 && selected_left < 0.14,
        "the acknowledged records reached the selected track-8 tail, the last call: {selected_left}"
    );
    assert_eq!(selected_right.to_bits(), (selected_left * 2.0).to_bits());
    assert_eq!(witness.fused_calls + witness.fallback_calls, 2);
    assert_eq!(
        witness.fused_calls, 2,
        "both calls fuse: the acknowledged commands settle before the tail call"
    );
    assert_eq!(report.applied_at_sample, u64::from(QUANTUM));
    let output = host.output_pcm().expect("output");
    let quantum = QUANTUM as usize;
    let expected_left = output[0];
    assert_eq!(expected_left.to_bits(), 0.5_f32.to_bits());
    assert!(
        output[..quantum]
            .iter()
            .all(|sample| sample.to_bits() == expected_left.to_bits()),
        "the routed reference remains nonzero on the acknowledged block"
    );
    assert!(
        output[quantum..]
            .iter()
            .all(|sample| sample.to_bits() == expected_left.to_bits()),
        "the routed reference remains dual-mono identity"
    );
}

/// A live-control host over the *command* fixture: the identity session plus one dynamic-rack
/// parametric EQ, so an effect-addressed command has something real to address (issue #140 A).
fn effect_live_control_host(quantum: u32, depth: u64) -> AudioWorkletEngineHost {
    let document = include_str!("../tests/browser-v1/command-session.json");
    let options = WebBootOptions {
        source_ring_frames: quantum,
        live_control_command_queue_records: depth,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("effect live-control boot")
}

#[cfg(feature = "test-support")]
fn bank_effect_live_control_host(quantum: u32, depth: u64) -> AudioWorkletEngineHost {
    let document = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
    let options = WebBootOptions {
        source_ring_frames: quantum * 2,
        live_control_command_queue_records: depth,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("bank EQ live-control boot")
}

#[test]
fn effect_control_browser_table_and_payload_reach_exact_budget_gate() {
    let document = include_str!("../tests/browser-v1/command-session.json");
    let parsed = parse_host_session(document).expect("effect fixture parse");
    let compiled = compile_host_model(
        &parsed,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("effect fixture compile");
    let shape = compiled_session_shape(&compiled).expect("effect fixture shape");
    let source_id_bytes = compiled
        .normalized_model()
        .sources
        .iter()
        .map(|source| source.id.as_str().len() as u64)
        .sum::<u64>();
    let source_control_bytes = control_table_bytes(shape.source_count as usize)
        .expect("source control table")
        + source_id_arena_bytes(source_id_bytes as usize).expect("source ID arena");
    let session_model_bytes = compiled.resource_estimate().compiled_model_bytes;
    const SOURCE_RING_FRAMES: u32 = 1 << 20;

    for (name, live_control_command_queue_records) in
        [("off", 0_u64), ("on", DEFAULT_COMMAND_QUEUE_RECORDS as u64)]
    {
        let options = WebBootOptions {
            source_ring_frames: SOURCE_RING_FRAMES,
            live_control_command_queue_records,
            ..boot_options(128)
        };
        let projection = project_buffers(
            document.len() as u32,
            shape.sample_rate_hz,
            shape.quantum_frames,
            shape.maximum_source_channels,
            shape
                .longest_source_id_bytes
                .max(shape.longest_track_id_bytes)
                .max(shape.longest_submix_id_bytes)
                .max(shape.longest_route_id_bytes)
                .max(shape.longest_vca_id_bytes),
            options,
            (false, (0, 0)),
        )
        .expect("bridge projection");
        let caps = prepare_caps(&compiled, options, SOURCE_RING_FRAMES, u64::MAX);
        let live_controls =
            live_control_request(options, shape.quantum_frames).expect("live-control request");
        let (engine, handles) = prepare_host_runtime_with_selected_meters_between_render_calls(
            &compiled,
            &caps,
            &live_controls,
            &[],
        )
        .expect("independent effect preparation");
        let dense_table = shape
            .effect_count
            .checked_mul(size_of::<Option<EffectControlProducer>>() as u64)
            .expect("dense effect table arithmetic");
        assert_eq!(shape.effect_count, 1, "fixture has one effect");
        let native_table =
            (handles.effect_controls.capacity() * size_of::<EffectControlProducer>()) as u64;
        let string_payload = handles
            .effect_controls
            .iter()
            .flat_map(|producer| [producer.track_id.len(), producer.effect_id.len()])
            .map(|bytes| bytes as u64)
            .sum::<u64>();
        let string_largest = handles
            .effect_controls
            .iter()
            .flat_map(|producer| [producer.track_id.len(), producer.effect_id.len()])
            .map(|bytes| bytes as u64)
            .max()
            .unwrap_or(0);
        assert_eq!(
            engine.report.effect_control_resources.producer_table_bytes, native_table,
            "{name}: native table uses actual Vec capacity"
        );
        let mut owner_payload = 0_u64;
        let mut owner_largest = 0_u64;
        for owner in handles
            .effect_controls
            .iter()
            .filter_map(EffectControlProducer::owner)
        {
            let (factory_layout, _) =
                core::alloc::Layout::new::<[core::sync::atomic::AtomicUsize; 2]>()
                    .extend(core::alloc::Layout::for_value(owner.factory().as_ref()))
                    .expect("factory Arc layout");
            let allocations = [
                core::mem::size_of_val(owner),
                core::mem::size_of_val(owner.committed()),
                core::mem::size_of_val(owner.candidate()),
                core::mem::size_of_val(owner.dirty()),
                // #1469 Amendment 1: a factory the launch registry owns is charged to no plan.
                if host_core::launch_registry_owns_factory(owner.factory()) {
                    0
                } else {
                    factory_layout.pad_to_align().size()
                },
            ];
            owner_payload += allocations.iter().sum::<usize>() as u64;
            owner_largest = owner_largest.max(*allocations.iter().max().unwrap() as u64);
        }
        assert_eq!(
            engine.report.effect_control_resources.owned_payload_bytes,
            string_payload + owner_payload,
            "{name}: exact owner slices, owner box and once-retained factory"
        );
        let decoded_count =
            command_staging_count(shape.track_count as usize, 0, 0).expect("decoded command count");
        let decoded_bytes = (decoded_count * size_of::<StagedCommand>()) as u64;
        let input_shadow_bytes = if live_control_command_queue_records == 0 {
            0
        } else {
            (shape.track_count as usize * size_of::<BuiltinInputShadow>()) as u64
        };
        let observation_arm_table = shape
            .effect_count
            .checked_mul(size_of::<Box<[u64]>>() as u64)
            .expect("observation arm table arithmetic");
        let effect_payload = string_payload + owner_payload;
        let effect_largest = string_largest.max(owner_largest);
        let effect_retained = dense_table + effect_payload;
        let ready_metadata =
            source_control_bytes + session_model_bytes + engine.inventory.retained_bytes();
        let expected_bridge_metadata = projection
            .report
            .bridge_metadata_bytes
            .checked_add(ready_metadata)
            .and_then(|bytes| bytes.checked_add(effect_retained))
            .and_then(|bytes| bytes.checked_add(observation_arm_table))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
            .and_then(|bytes| bytes.checked_add(input_shadow_bytes))
            .expect("bridge metadata arithmetic");
        let expected_bridge_retained = projection
            .report
            .bridge_retained_bytes
            .checked_add(ready_metadata)
            .and_then(|bytes| bytes.checked_add(effect_retained))
            .and_then(|bytes| bytes.checked_add(observation_arm_table))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
            .and_then(|bytes| bytes.checked_add(input_shadow_bytes))
            .expect("bridge retained arithmetic");
        let expected_bridge_largest = projection
            .report
            .largest_bridge_allocation_bytes
            .max(control_table_bytes(shape.source_count as usize).expect("control largest"))
            .max(source_id_arena_bytes(source_id_bytes as usize).expect("ID largest"))
            .max(engine.report.session_largest_allocation_bytes)
            .max(dense_table)
            .max(string_largest)
            .max(effect_largest)
            .max(observation_arm_table)
            .max(decoded_bytes)
            .max(input_shadow_bytes);
        let expected_named_largest =
            expected_bridge_largest.max(engine.report.largest_engine_allocation_bytes);
        let host =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
                panic!("{name}: {}", String::from_utf8_lossy(failure.diagnostic()))
            });
        let ready = host.ready.as_ref().expect("ready ownership");
        assert_eq!(
            size_of_val(ready.effect_controls.as_ref()) as u64,
            dense_table,
            "{name}: browser owns the actual dense replacement table"
        );
        assert_eq!(
            host.resources().bridge_metadata_bytes,
            expected_bridge_metadata,
            "{name}: exact bridge metadata includes dense table and payload"
        );
        assert_eq!(
            host.resources().bridge_retained_bytes,
            expected_bridge_retained,
            "{name}: exact bridge retained bytes include dense table and payload"
        );
        assert_eq!(
            host.resources().largest_bridge_allocation_bytes,
            expected_bridge_largest,
            "{name}: largest browser allocation excludes consumed native table"
        );
        assert_eq!(
            host.resources().largest_named_allocation_bytes,
            expected_named_largest
        );

        let exact = exact_retained_report_total(host.resources());
        AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                maximum_memory_bytes: exact,
                ..options
            },
        )
        .expect("exact retained effect budget must admit");
        let failure = AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                maximum_memory_bytes: exact - 1,
                ..options
            },
        )
        .err()
        .unwrap_or_else(|| panic!("{name}: one byte below retained effect budget must refuse"));
        assert_eq!(failure.result(), RESULT_REFUSED_BUDGET);
        assert_eq!(
            failure.diagnostic(),
            format!(
                "host.budget.retained_exact\t$.maximum_memory_bytes[exact_bytes={exact},budget_bytes={}]\n",
                exact - 1
            )
            .as_bytes(),
            "{name}: one-byte refusal reaches final exact aggregate gate"
        );
    }
}

#[test]
fn production_effect_delivery_refuses_prepared_target_without_queue_or_full_mutation() {
    let mut host = effect_live_control_host(128, DEFAULT_COMMAND_QUEUE_RECORDS as u64);
    let target = EffectControlRecord::PreparedTarget(PreparedEffectTarget {
        slot: 2,
        channel: ParameterChannel::Left,
        words: [0x55; PREPARED_EFFECT_TARGET_WORDS],
    });
    let ready = host.ready.as_mut().expect("ready ownership");
    let effect_slot = ready
        .effect_slot(0, LiveEffectAddress::insert(0))
        .expect("inserted EQ slot");
    let queue_slot = ready.tracks.len() * 3 + effect_slot;
    let producer = ready
        .effect_controls
        .get(effect_slot)
        .and_then(Option::as_ref)
        .expect("effect producer");
    let before_success = producer.success_count();
    let before_full = producer.full_count();
    let before_in_flight = ready.in_flight[queue_slot];

    assert!(ready.preflight_effect(queue_slot, target).is_err());
    assert!(
        ready
            .push(queue_slot, AdmittedCommand::Effect(target), 0)
            .is_err()
    );

    let producer = ready
        .effect_controls
        .get(effect_slot)
        .and_then(Option::as_ref)
        .expect("effect producer remains");
    assert_eq!(producer.success_count(), before_success);
    assert_eq!(producer.full_count(), before_full);
    assert_eq!(ready.in_flight[queue_slot], before_in_flight);
}

/// The production owner receives complete, already-designed targets on both the scalar dynamic
/// fixture and the SIMD-bank fixture. Preparation is the only place that may invoke the EQ
/// designer; admission and the following render are measured through the existing FFI allocator.
#[cfg(feature = "test-support")]
#[test]
fn prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation() {
    const QUANTUM: u32 = 128;

    let mut scalar = effect_live_control_host(QUANTUM, 8);
    feed_and_render(&mut scalar, 1, 0, 0.25);
    host_core::test_only_reset_parametric_eq_design_calls();
    stage_prepared_eq_parameter(&mut scalar, 0, 0, 1, 0, 4, -12.0);
    assert!(
        host_core::test_only_parametric_eq_design_call_count() > 0,
        "off-thread EQ preparation must invoke the real designer"
    );
    assert_eq!(
        scalar.submit_prepared_commands(1, 104),
        RESULT_OK,
        "first prepared ACK"
    );

    // A second transaction is acknowledged before the render boundary, using the owner's new
    // committed revision and a fresh target. The measured operation contains no preparation.
    host_core::test_only_reset_parametric_eq_design_calls();
    stage_prepared_eq_parameter(&mut scalar, 0, 0, 1, 0, 4, 18.0);
    assert!(host_core::test_only_parametric_eq_design_call_count() > 0);
    host_core::test_only_reset_parametric_eq_design_calls();
    let left = [0.25_f32; QUANTUM as usize];
    let right = [0.25_f32; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        scalar.submit_source(
            b"fixture-source",
            1,
            u64::from(QUANTUM),
            48_000,
            &planes,
            QUANTUM,
            false,
        ),
        RESULT_OK
    );
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            let admission = scalar.submit_prepared_commands(1, 104);
            let render = scalar.render_next();
            (admission, render)
        });
    assert_eq!(admission, RESULT_OK, "second prepared ACK");
    assert_eq!(render, RESULT_OK, "prepared render");
    assert_eq!(allocations, 0, "prepared admission/render allocated");
    assert_eq!(deallocations, 0, "prepared admission/render freed");
    assert_eq!(
        host_core::test_only_parametric_eq_design_call_count(),
        0,
        "admission/render must not redesign"
    );
    let config = {
        assert_eq!(scalar.copy_eq_target_config(0, 1, 0), RESULT_OK);
        scalar.eq_target_config().expect("scalar config").to_vec()
    };
    assert_eq!(
        u64::from_le_bytes(config[24..32].try_into().expect("revision")),
        2,
        "two ACKs commit two owner revisions before/at the render boundary"
    );

    let mut bank = bank_effect_live_control_host(QUANTUM, 8);
    feed_and_render_tracks(&mut bank, 0, 0.25);
    host_core::test_only_reset_parametric_eq_design_calls();
    // The bank fixture's EQ is console slot 0 (issue #1096).
    stage_prepared_eq_parameter(&mut bank, 0, 0, RACK_CONSOLE, 0, 4, 12.0);
    assert!(host_core::test_only_parametric_eq_design_call_count() > 0);
    host_core::test_only_reset_parametric_eq_design_calls();
    let left = [0.25_f32; QUANTUM as usize];
    let right = [0.25_f32; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        bank.submit_source(
            b"fixture-source",
            1,
            u64::from(QUANTUM),
            48_000,
            &planes,
            QUANTUM,
            false,
        ),
        RESULT_OK
    );
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            let admission = bank.submit_prepared_commands(1, 104);
            let render = bank.render_next();
            (admission, render)
        });
    assert_eq!(admission, RESULT_OK, "bank prepared render");
    assert_eq!(render, RESULT_OK, "bank second render");
    assert_eq!(allocations, 0, "bank render allocated");
    assert_eq!(deallocations, 0, "bank render freed");
    assert_eq!(
        host_core::test_only_parametric_eq_design_call_count(),
        0,
        "bank render must not redesign"
    );
}

fn eq_owner_snapshot(host: &mut AudioWorkletEngineHost) -> (Vec<u8>, u64, u64) {
    assert_eq!(host.copy_eq_target_config(0, 1, 0), RESULT_OK);
    let config = host.eq_target_config().expect("EQ owner config").to_vec();
    let revision = u64::from_le_bytes(config[24..32].try_into().expect("revision"));
    let effect = host
        .ready
        .as_ref()
        .expect("ready")
        .effect_slot(0, LiveEffectAddress::insert(0))
        .expect("EQ slot");
    let success = host
        .ready
        .as_ref()
        .expect("ready")
        .effect_controls
        .get(effect)
        .and_then(Option::as_ref)
        .expect("EQ producer")
        .success_count();
    (config, revision, success)
}

#[test]
fn prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches() {
    const QUANTUM: u32 = 128;

    let mut host = input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_prepared_input_filter(&mut host, 0, 0, 300.0, 2_000.0);
    stage_command(
        &mut host,
        1,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        0,
        [-3.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(
        host.submit_prepared_commands(2, companion_bytes),
        RESULT_OK,
        "builtin and fader records share one admission"
    );
    let input_slot = 2;
    let ready = host.ready.as_ref().expect("ready ownership");
    assert_eq!(ready.input_filter_shadows[0].revision, 1);
    assert_eq!(
        ready.input_filter_shadows[0].committed,
        [300.0, 2_000.0, 300.0, 2_000.0]
    );
    assert_eq!(
        ready.in_flight[input_slot], 2,
        "both prepared sections reached input queue"
    );
    assert_eq!(
        ready.in_flight[1], 1,
        "mixed fader record reached its own queue"
    );

    assert_eq!(host.copy_input_filter_config(0), RESULT_OK);
    let config = host.eq_target_config().expect("shared config workspace");
    assert_eq!(u32::from_le_bytes(config[0..4].try_into().unwrap()), 48);
    assert_eq!(u32::from_le_bytes(config[12..16].try_into().unwrap()), 4);
    assert_eq!(u64::from_le_bytes(config[24..32].try_into().unwrap()), 1);
    assert_eq!(
        [0, 1, 2, 3].map(|index| {
            f32::from_bits(u32::from_le_bytes(
                config[32 + index * 4..36 + index * 4].try_into().unwrap(),
            ))
        }),
        [300.0, 2_000.0, 300.0, 2_000.0]
    );
    feed_and_render(&mut host, 1, 0, 0.25);
    assert_eq!(host.ready.as_ref().unwrap().in_flight[input_slot], 0);

    // A later invalid original transition rolls back the candidate before either target is
    // published, even though the first command in the batch is valid.
    let mut late = input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_prepared_input_filter(&mut late, 0, 0, 300.0, 2_000.0);
    stage_command(
        &mut late,
        1,
        COMMAND_INPUT_FILTERS,
        255,
        0,
        0,
        0,
        3,
        0,
        [3_000.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(
        late.submit_prepared_commands(2, companion_bytes),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(late.command_report().rejected_index, 1);
    assert_eq!(late.command_report().reason, COMMAND_REASON_DOMAIN);
    let late_ready = late.ready.as_ref().unwrap();
    assert_eq!(late_ready.input_filter_shadows[0].revision, 0);
    assert_eq!(
        late_ready.input_filter_shadows[0].committed,
        [100.0, 1_000.0, 100.0, 1_000.0]
    );
    assert_eq!(late_ready.in_flight[input_slot], 0);

    // A stale base revision is rejected by the same companion address path.
    let mut stale = input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_prepared_input_filter(&mut stale, 0, 0, 300.0, 2_000.0);
    stale.prepared_companion_mut().unwrap()[40..48].copy_from_slice(&1_u64.to_le_bytes());
    assert_eq!(
        stale.submit_prepared_commands(1, companion_bytes),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(stale.command_report().reason, COMMAND_REASON_MALFORMED);
    assert_eq!(stale.command_report().rejected_index, 0);
    assert_eq!(
        stale.ready.as_ref().unwrap().input_filter_shadows[0].revision,
        0
    );

    // Mutate the existing valid frame, exercising the builtin target coverage/safety checks.
    for mutation in ["missing", "duplicate", "unsafe", "reserved"] {
        let mut malformed = input_filter_live_control_host(QUANTUM, 8);
        let mut frame_bytes = stage_prepared_input_filter(&mut malformed, 0, 0, 300.0, 2_000.0);
        let frame = malformed.prepared_companion_mut().unwrap();
        match mutation {
            "missing" => {
                frame[16..20].copy_from_slice(&1_u32.to_le_bytes());
                frame_bytes = 24 + 80;
            }
            "duplicate" => frame.copy_within(24..104, 104),
            "unsafe" => frame[80..84].copy_from_slice(&f32::NAN.to_bits().to_le_bytes()),
            "reserved" => frame[64..68].copy_from_slice(&1_u32.to_le_bytes()),
            _ => unreachable!(),
        }
        assert_eq!(
            malformed.submit_prepared_commands(1, frame_bytes),
            RESULT_INVALID_ARGUMENT,
            "{mutation}"
        );
        assert_eq!(malformed.command_report().reason, COMMAND_REASON_MALFORMED);
        assert_eq!(malformed.command_report().admitted, 0);
        let ready = malformed.ready.as_ref().unwrap();
        assert_eq!(ready.input_filter_shadows[0].revision, 0);
        assert_eq!(
            ready.input_filter_shadows[0].committed,
            [100.0, 1_000.0, 100.0, 1_000.0]
        );
        assert!(ready.in_flight.iter().all(|count| *count == 0));
    }

    // A command selector outside the builtin pair vocabulary is malformed wire shape, rather
    // than an unknown effect parameter: the builtin command owns selectors 0, 3 and 4 only.
    let mut invalid_selector = input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_prepared_input_filter(&mut invalid_selector, 0, 0, 300.0, 2_000.0);
    stage_command(
        &mut invalid_selector,
        1,
        COMMAND_INPUT_FILTERS,
        255,
        2,
        0,
        0,
        7,
        0,
        [300.0, 2_000.0, 0.0, 0.0],
    );
    assert_eq!(
        invalid_selector.submit_prepared_commands(2, companion_bytes),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        invalid_selector.command_report().reason,
        COMMAND_REASON_MALFORMED
    );
    assert_eq!(invalid_selector.command_report().rejected_index, 1);
    let ready = invalid_selector.ready.as_ref().unwrap();
    assert_eq!(ready.input_filter_shadows[0].revision, 0);
    assert!(ready.in_flight.iter().all(|count| *count == 0));

    // A full unrelated matrix queue refuses the mixed transaction while leaving the builtin
    // shadow at its committed seed.
    let mut full = input_filter_live_control_host(QUANTUM, 2);
    for index in 0..2 {
        stage_command(
            &mut full,
            index,
            COMMAND_MATRIX,
            255,
            255,
            0,
            0,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
        );
    }
    assert_eq!(full.submit_commands(2), RESULT_OK);
    let companion_bytes = stage_prepared_input_filter(&mut full, 0, 0, 300.0, 2_000.0);
    stage_command(
        &mut full,
        1,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 1.0],
    );
    assert_eq!(
        full.submit_prepared_commands(2, companion_bytes),
        RESULT_BACKPRESSURE
    );
    assert_eq!(full.command_report().rejected_index, 1);
    assert_eq!(
        full.ready.as_ref().unwrap().input_filter_shadows[0].revision,
        0
    );
    assert_eq!(full.ready.as_ref().unwrap().in_flight[input_slot], 0);
}

/// A single prepared admission can commit the existing EQ owner, the builtin input owner and a
/// fader together. A late semantic refusal rolls both prepared owners back before publication;
/// the successful admission/render path stays allocation-free under the existing FFI counter.
#[cfg(feature = "test-support")]
#[test]
fn prepared_mixed_eq_builtin_fader_commits_and_refuses_atomically() {
    const QUANTUM: u32 = 128;

    let mut host = effect_input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_mixed_prepared_eq_and_input_filter(&mut host);
    stage_command(
        &mut host,
        2,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        0,
        [-3.0, 0.0, 0.0, 0.0],
    );
    feed_and_render(&mut host, 1, 0, 0.25);
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            let admission = host.submit_prepared_commands(3, companion_bytes);
            let render = host.render_next();
            (admission, render)
        });
    assert_eq!(admission, RESULT_OK, "mixed prepared ACK");
    assert_eq!(render, RESULT_OK, "mixed prepared render");
    assert_eq!(allocations, 0, "mixed prepared admission/render allocated");
    assert_eq!(deallocations, 0, "mixed prepared admission/render freed");
    assert_eq!(eq_owner_snapshot(&mut host).1, 1, "EQ owner committed");
    let ready = host.ready.as_ref().expect("ready ownership");
    assert_eq!(ready.input_filter_shadows[0].revision, 1);
    assert_eq!(ready.in_flight[1], 0, "fader drained at render boundary");
    assert_eq!(
        ready.in_flight[2], 0,
        "builtin targets drained at render boundary"
    );

    let mut refused = effect_input_filter_live_control_host(QUANTUM, 8);
    let companion_bytes = stage_mixed_prepared_eq_and_input_filter(&mut refused);
    stage_command(
        &mut refused,
        2,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        0,
        [-3.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut refused,
        3,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        9_999,
        0,
        [0.0; 4],
    );
    assert_eq!(
        refused.submit_prepared_commands(4, companion_bytes),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(refused.command_report().rejected_index, 3);
    assert_eq!(
        refused.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
    );
    assert_eq!(eq_owner_snapshot(&mut refused).1, 0, "EQ rollback");
    let ready = refused.ready.as_ref().expect("ready ownership");
    assert_eq!(
        ready.input_filter_shadows[0].revision, 0,
        "builtin rollback"
    );
    assert!(ready.in_flight.iter().all(|count| *count == 0));
}

/// The host's prepared-owner transaction rejects malformed, stale and late-invalid companions
/// atomically. The final case fills an unrelated matrix queue to prove the EQ candidate rolls
/// back when a later destination has no room, then verifies a valid retry recovers.
#[test]
fn prepared_eq_owner_refusals_preserve_config_revision_indexes_and_queues() {
    const QUANTUM: u32 = 128;

    // A valid prepared EQ edit followed by an invalid edit names the original second wire index
    // and leaves the first owner's candidate unpublished.
    let mut late = effect_live_control_host(QUANTUM, 8);
    stage_prepared_eq_parameter(&mut late, 0, 0, 1, 0, 4, -12.0);
    stage_command(
        &mut late,
        1,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        0,
        0,
        9_999,
        0,
        [0.0; 4],
    );
    assert_eq!(
        late.submit_prepared_commands(2, 104),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        late.command_report().reason,
        COMMAND_REASON_UNKNOWN_PARAMETER
    );
    assert_eq!(late.command_report().rejected_index, 1);
    assert_eq!(late.command_report().admitted, 0);
    let (config, revision, success) = eq_owner_snapshot(&mut late);
    assert_eq!(revision, 0);
    assert_eq!(success, 0);
    assert_eq!(
        submit_prepared_eq_parameter(&mut late, 0, 0, 1, 0, 4, -12.0),
        RESULT_OK,
        "valid retry after late invalid edit"
    );
    let (recovered, recovered_revision, recovered_success) = eq_owner_snapshot(&mut late);
    assert_ne!(recovered, config);
    assert_eq!(recovered_revision, 1);
    assert_eq!(recovered_success, 1);

    // A missing target record is unsafe even though the semantic edit itself is valid.
    let mut missing = effect_live_control_host(QUANTUM, 8);
    stage_command(
        &mut missing,
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
    let missing_before = eq_owner_snapshot(&mut missing);
    let generation = missing.host_generation;
    let companion = missing
        .prepared_companion_mut()
        .expect("prepared companion");
    companion.fill(0);
    companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
    companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    companion[8..16].copy_from_slice(&generation.to_le_bytes());
    assert_eq!(
        missing.submit_prepared_commands(1, 24),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(missing.command_report().reason, COMMAND_REASON_MALFORMED);
    assert_eq!(missing.command_report().rejected_index, 0);
    assert_eq!(missing.command_report().admitted, 0);
    let (_, revision, success) = eq_owner_snapshot(&mut missing);
    assert_eq!(revision, 0);
    assert_eq!(success, 0);

    assert_eq!(eq_owner_snapshot(&mut missing), missing_before);

    // Unsafe coefficients and invalid original edits cannot be hidden by a valid final target.
    for invalid_edit in [false, true] {
        let mut host = effect_live_control_host(QUANTUM, 8);
        let before = eq_owner_snapshot(&mut host);
        stage_prepared_eq_parameter(&mut host, usize::from(invalid_edit), 0, 1, 0, 4, -12.0);
        if invalid_edit {
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
                [f32::NAN, 0.0, 0.0, 0.0],
            );
        } else {
            host.prepared_companion_mut().unwrap()[80..84]
                .copy_from_slice(&f32::NAN.to_bits().to_le_bytes());
        }
        assert_ne!(
            host.submit_prepared_commands(1 + u32::from(invalid_edit), 104),
            RESULT_OK
        );
        assert_eq!(host.command_report().admitted, 0);
        assert_eq!(host.command_report().rejected_index, 0);
        assert_eq!(eq_owner_snapshot(&mut host), before);
    }

    // A valid target with an unrelated addressed extra record is rejected before publication.
    let mut extra = effect_live_control_host(QUANTUM, 8);
    stage_prepared_eq_parameter(&mut extra, 0, 0, 1, 0, 4, -12.0);
    {
        let companion = extra.prepared_companion_mut().expect("prepared companion");
        companion.copy_within(24..104, 104);
        companion[16..20].copy_from_slice(&2_u32.to_le_bytes());
        companion[104..108].copy_from_slice(&99_u32.to_le_bytes());
    }
    assert_eq!(
        extra.submit_prepared_commands(1, 184),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(extra.command_report().reason, COMMAND_REASON_MALFORMED);
    assert_eq!(extra.command_report().rejected_index, 0);
    assert_eq!(extra.command_report().admitted, 0);
    let (_, revision, success) = eq_owner_snapshot(&mut extra);
    assert_eq!(revision, 0);
    assert_eq!(success, 0);

    // Both a stale owner revision and a stale host generation refuse atomically.
    for (label, offset) in [("revision", 40_usize), ("generation", 8_usize)] {
        let mut stale = effect_live_control_host(QUANTUM, 8);
        stage_prepared_eq_parameter(&mut stale, 0, 0, 1, 0, 4, -12.0);
        stale.prepared_companion_mut().expect("prepared companion")[offset..offset + 8]
            .copy_from_slice(&99_u64.to_le_bytes());
        assert_eq!(
            stale.submit_prepared_commands(1, 104),
            RESULT_INVALID_ARGUMENT,
            "stale {label} result"
        );
        assert_eq!(stale.command_report().reason, COMMAND_REASON_MALFORMED);
        assert_eq!(stale.command_report().rejected_index, 0);
        assert_eq!(stale.command_report().admitted, 0);
        let (_, revision, success) = eq_owner_snapshot(&mut stale);
        assert_eq!(revision, 0, "stale {label} changed revision");
        assert_eq!(success, 0, "stale {label} changed queue");
    }

    // Fill the matrix queue, then submit a valid prepared EQ edit plus a later matrix record.
    // The unrelated full queue refuses the whole batch; after one render, the EQ edit recovers.
    let mut full = effect_live_control_host(QUANTUM, 1);
    stage_command(
        &mut full,
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
    assert_eq!(full.submit_commands(1), RESULT_OK);
    stage_prepared_eq_parameter(&mut full, 0, 0, 1, 0, 4, -12.0);
    stage_command(
        &mut full,
        1,
        COMMAND_MATRIX,
        255,
        255,
        0,
        0,
        0,
        0,
        [0.25, 0.0, 0.0, 1.0],
    );
    assert_eq!(full.submit_prepared_commands(2, 104), RESULT_BACKPRESSURE);
    assert_eq!(full.command_report().reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(full.command_report().rejected_index, 1);
    assert_eq!(full.command_report().admitted, 0);
    let (_, revision, success) = eq_owner_snapshot(&mut full);
    assert_eq!(revision, 0);
    assert_eq!(success, 0);
    assert_eq!(full.render_next(), RESULT_OK);
    assert_eq!(
        full.submit_prepared_commands(2, 104),
        RESULT_OK,
        "the same mixed EQ/matrix batch recovers after the unrelated queue drains"
    );
    assert_eq!(full.command_report().admitted, 2);
    assert_eq!(eq_owner_snapshot(&mut full).1, 1);
}

#[test]
fn late_mixed_effect_refusal_preserves_observation_queue_solo_and_wire_index() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 1, None);
    let queue_counters = |ready: &ReadyOwnership| {
        (
            ready
                .controls
                .iter()
                .map(|owner| {
                    [
                        (owner.producer.success_count(), owner.producer.full_count()),
                        (owner.fader.success_count(), owner.fader.full_count()),
                        {
                            let input = owner.input.as_ref().expect("browser input lane");
                            (input.success_count(), input.full_count())
                        },
                    ]
                })
                .collect::<Vec<_>>(),
            ready
                .effect_controls
                .iter()
                .map(|owner| {
                    owner
                        .as_ref()
                        .map(|owner| (owner.success_count(), owner.full_count()))
                })
                .collect::<Vec<_>>(),
        )
    };
    let ready = host.ready.as_ref().expect("ready ownership");
    let before_counters = queue_counters(ready);
    let before_masks = ready.observation_armed.clone();
    let before_arm_samples = ready.observation_arm_samples.clone();
    let before_queues = host
        .ready
        .as_ref()
        .expect("ready ownership")
        .in_flight
        .to_vec();
    let before_solo = {
        let state = host.live_control_solo().expect("solo state");
        (
            state.solo_count(),
            (0..state.strip_count())
                .map(|track| {
                    (
                        state.solo(track),
                        [state.user_mute(track, 0), state.user_mute(track, 1)],
                        [state.emitted_mute(track, 0), state.emitted_mute(track, 1)],
                    )
                })
                .collect::<Vec<_>>(),
        )
    };
    let before_armed = host.observation_armed_taps();

    // The first two commands are valid and touch separate control destinations. The final
    // original-band enabled row is immutable. Its refusal must name wire index 2 and prevent
    // either earlier command from being published.
    stage_solo(&mut host, 0, 0, true, 0);
    stage_command(
        &mut host,
        1,
        COMMAND_OBSERVE_SUBSCRIBE,
        1,
        255,
        0,
        0,
        1,
        1,
        [0.0; 4],
    );
    stage_command(
        &mut host,
        2,
        COMMAND_EFFECT_PARAM,
        1,
        2,
        1,
        0,
        1,
        0,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(3), RESULT_UNSUPPORTED);
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_UNSUPPORTED_KIND
    );
    assert_eq!(host.command_report().rejected_index, 2);
    assert_eq!(host.command_report().admitted, 0);

    assert_eq!(host.observation_armed_taps(), before_armed);
    let after_queues = host
        .ready
        .as_ref()
        .expect("ready ownership")
        .in_flight
        .to_vec();
    assert_eq!(after_queues, before_queues);
    let ready = host.ready.as_ref().expect("ready ownership");
    assert_eq!(queue_counters(ready), before_counters);
    assert_eq!(ready.observation_armed, before_masks);
    assert_eq!(ready.observation_arm_samples, before_arm_samples);

    let after_solo = {
        let state = host.live_control_solo().expect("solo state");
        (
            state.solo_count(),
            (0..state.strip_count())
                .map(|track| {
                    (
                        state.solo(track),
                        [state.user_mute(track, 0), state.user_mute(track, 1)],
                        [state.emitted_mute(track, 0), state.emitted_mute(track, 1)],
                    )
                })
                .collect::<Vec<_>>(),
        )
    };
    assert_eq!(after_solo, before_solo);
}

/// #140 B / E1: a fader command's acknowledgement names the exact sample it takes effect at.
///
/// The fixture is identity end to end, so a constant input renders to that same constant and a
/// fader move has exactly one observable consequence: the whole plane scales. With a zero window
/// the transition is a block boundary and is exact, not approximate.
///
/// Red mutation: move the `drain_controls` call in
/// `LiveControlFaderProcessor::process` to *after* `self.fader.process(block)` -> the reported
/// sample is one block early and `at_applied` still renders the pre-command value.
#[test]
fn a_fader_command_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut host = live_control_host(QUANTUM, 0);
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
/// Red mutation: make `FaderMuteRampBuiltins::set_mute` snap instead of retargeting -> the
/// windowed mute is already silent on its first sample and the "still audible" assertion fails.
#[test]
fn a_mute_command_is_a_fader_endpoint_not_a_discontinuity() {
    const QUANTUM: u32 = 128;
    let mut host = live_control_host(QUANTUM, 0);
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
/// acknowledgement named, with the current coefficient used for that sample before the ramp
/// advances. The first changed sample is therefore one sample later.
///
/// The proof is a two-host comparison rather than a closed-form value: the EQ's own 64-sample
/// coefficient ramp is its DSP, not this issue's, so what is gated here is the *boundary*. The
/// control host receives nothing; every block before `applied_at_sample` must be bit-identical
/// between the two. At `applied_at_sample`, both channels remain bit-identical for sample 0 and
/// differ at sample 1 as the current-then-advance ramp contract takes effect.
///
/// Red mutation: move the `live.control.stage(..)` drain in `execute_op`'s `LiveControlEffect` arm
/// below `effect.processor.process(block)` -> the first differing block is one later and the
/// sample-1 assertions fail.
#[test]
fn an_effect_parameter_command_names_the_exact_application_sample() {
    const QUANTUM: u32 = 128;
    let mut control = effect_live_control_host(QUANTUM, 8);
    let mut commanded = effect_live_control_host(QUANTUM, 8);

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
    assert_eq!(
        submit_prepared_eq_parameter(&mut commanded, 0, 0, 1, 0, 4, -12.0),
        RESULT_OK
    );
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
    assert_eq!(
        clean[0].to_bits(),
        moved[0].to_bits(),
        "the current coefficient remains in force for sample 0 on the left",
    );
    assert_eq!(
        clean[QUANTUM as usize].to_bits(),
        moved[QUANTUM as usize].to_bits(),
        "the current coefficient remains in force for sample 0 on the right",
    );
    assert_ne!(
        clean[1].to_bits(),
        moved[1].to_bits(),
        "the left coefficient changes at sample 1 after the ramp advances",
    );
    assert_ne!(
        clean[QUANTUM as usize + 1].to_bits(),
        moved[QUANTUM as usize + 1].to_bits(),
        "a `channel = both` command lowers to one span per lane, so the right lane changes at sample 1",
    );
}

/// #140 A: live effect bypass returns the dry signal at the effect's declared latency, and
/// releasing it returns the wet signal the effect has been computing all along.
///
/// Red mutation: delete the `live.shunt.capture(..)` call in `execute_op` -> a bypassed block
/// renders the shunt's initial zeros instead of the input, and the equality below fails.
#[test]
fn an_effect_bypass_command_returns_the_dry_signal() {
    const QUANTUM: u32 = 128;
    let mut host = effect_live_control_host(QUANTUM, 8);
    // Move the band well off flat first, so "bypassed" and "enabled" are distinguishable.
    assert_eq!(
        submit_prepared_eq_parameter(&mut host, 0, 0, 1, 0, 4, 18.0),
        RESULT_OK
    );
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
    let mut clean = effect_live_control_host(QUANTUM, u64::from(DEPTH));
    let mut flooded = effect_live_control_host(QUANTUM, u64::from(DEPTH));
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
/// is legal, and *this session* has no write path. A host compiled with no live controls is that
/// session, and every live kind is refused with it.
#[test]
fn a_live_control_free_host_refuses_every_live_kind_as_unsupported() {
    const QUANTUM: u32 = 128;
    let mut host = effect_live_control_host(QUANTUM, 0);
    feed_and_render(&mut host, 1, 0, 0.25);
    let baseline = host.output_pcm().expect("output").to_vec();
    // A live-control-free host has no staging buffer at all: the refusal is decided before a record
    // could even be written, which is the strongest form of "this session cannot apply it".
    assert!(
        host.command_staging_mut().is_none(),
        "no live controls means no staging buffer",
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
        "a live-control-free host renders the same block it would have without any traffic",
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

    let mut clean = live_control_host(QUANTUM, 0);
    let mut flooded = live_control_host(QUANTUM, 0);
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
    let mut host = live_control_host(QUANTUM, 0);
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
/// Red mutation: change `live_control_meter_blocks` handling so the period is `blocks` frames
/// instead of `blocks * quantum_frames` -> a window closes mid-block, `poll_meters` reports more
/// windows than blocks rendered, and the cadence assertion fails.
#[test]
fn meter_frames_equal_an_offline_fold_and_cost_the_render_nothing() {
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 2;
    let mut off = live_control_host(QUANTUM, BLOCKS);
    let mut on = live_control_host(QUANTUM, BLOCKS);
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
    let mut bare = live_control_host(QUANTUM, 0);
    assert!(!bare.meters_attached());
    assert_eq!(bare.set_meter_lease(true), RESULT_UNSUPPORTED);
    assert_eq!(bare.poll_meters(), 0);
}

#[test]
fn meter_empty_poll_preserves_early_peak_and_publication_state() {
    let mut host = live_control_host(128, 2);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 1.0);
    assert_eq!(
        host.poll_meters(),
        0,
        "the first half-window is still pending"
    );
    let pending_frame = host.meter_frame().to_vec();
    let pending_header = *host.meter_header();
    feed_and_render(&mut host, 1, 1, 0.1);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(
        host.meter_frame()[0],
        1.0,
        "the early impulse survived the empty poll"
    );
    let published_frame = host.meter_frame().to_vec();
    let published_header = *host.meter_header();
    assert_ne!(published_frame, pending_frame);
    assert_eq!(published_header.first_sample, 0);
    assert_eq!(published_header.end_sample, 256);
    assert_eq!(
        published_frame[2], 1.0,
        "master left retained the early peak"
    );
    assert_eq!(
        published_frame[3], 1.0,
        "master right retained the early peak"
    );
    assert_eq!(
        host.poll_meters(),
        0,
        "an empty poll does not manufacture a new frame"
    );
    assert_eq!(host.meter_frame(), &published_frame[..]);
    assert_eq!(*host.meter_header(), published_header);
    assert_eq!(pending_header.sequence, 0);
}

#[test]
fn incomplete_master_periods_skip_all_track_pops_and_effect_scans() {
    const QUANTUM: u32 = 128;
    for blocks in [2_u64, 8, 32] {
        let mut eager = live_control_host(QUANTUM, blocks);
        let mut boundary = live_control_host(QUANTUM, blocks);
        assert_eq!(eager.set_meter_lease(true), RESULT_OK);
        assert_eq!(boundary.set_meter_lease(true), RESULT_OK);
        let initial_frame = eager.meter_frame().to_vec();
        let initial_header = *eager.meter_header();
        let initial_work = eager.meter_poll_work();

        for block in 0..blocks {
            let value = if block == 0 { 1.0 } else { 0.125 };
            feed_and_render(&mut eager, 1, block, value);
            feed_and_render(&mut boundary, 1, block, value);
            if block + 1 < blocks {
                assert_eq!(eager.poll_meters(), 0, "period {blocks}, block {block}");
                assert_eq!(eager.meter_poll_work(), initial_work);
                assert_eq!(eager.meter_frame(), &initial_frame[..]);
                assert_eq!(*eager.meter_header(), initial_header);
            }
        }
        assert_eq!(eager.poll_meters(), 1, "period {blocks}");
        assert_eq!(boundary.poll_meters(), 1, "boundary period {blocks}");
        assert_eq!(eager.meter_frame(), boundary.meter_frame());
        assert_eq!(eager.meter_header(), boundary.meter_header());

        let published_frame = eager.meter_frame().to_vec();
        let published_header = *eager.meter_header();
        let completed_work = eager.meter_poll_work();
        feed_and_render(&mut eager, 1, blocks, 0.75);
        assert_eq!(eager.poll_meters(), 0, "partial period {blocks}");
        assert_eq!(eager.meter_poll_work(), completed_work);
        assert_eq!(eager.meter_frame(), &published_frame[..]);
        assert_eq!(*eager.meter_header(), published_header);
    }

    let mut observed = observation_host(QUANTUM, 8, Some(0));
    assert!(observed.observation_attached());
    assert_eq!(observed.set_meter_lease(true), RESULT_OK);
    let before = observed.meter_poll_work();
    feed_and_render_tracks(&mut observed, 0, 0.5);
    assert_eq!(observed.poll_meters(), 0);
    assert_eq!(observed.meter_poll_work(), before);
}

/// Feed a finite source, marking only the chunk that reaches its prepared boundary.
fn feed_and_render_finite_source(
    host: &mut AudioWorkletEngineHost,
    block: u64,
    source_blocks: u64,
    value: f32,
) {
    assert!(block < source_blocks);
    let quantum = host.status().quantum_frames;
    let samples = vec![value; quantum as usize];
    let planes: [&[f32]; 2] = [&samples, &samples];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            block * u64::from(quantum),
            host.status().sample_rate_hz,
            &planes,
            quantum,
            block + 1 == source_blocks,
        ),
        RESULT_OK,
    );
    assert_eq!(host.render_next(), RESULT_OK);
}

#[test]
fn finite_source_boundary_requires_only_the_actual_final_marker() {
    const QUANTUM: u32 = 128;
    const SOURCE_BLOCKS: u64 = 2;
    let mut host = meter_tail_host_for_blocks(48_000, QUANTUM, 32, SOURCE_BLOCKS);
    let samples = [0.25; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&samples, &samples];
    for block in 0..SOURCE_BLOCKS {
        // Reject an early marker first, then an unmarked final chunk on the same host.
        assert_eq!(
            host.submit_source(
                b"fixture-source",
                1,
                block * u64::from(QUANTUM),
                48_000,
                &planes,
                QUANTUM,
                block == 0,
            ),
            RESULT_INVALID_ARGUMENT,
            "wrong boundary marker at block {block}",
        );
        // Rendering each accepted chunk drains the one-quantum ring before the next admission.
        feed_and_render_finite_source(&mut host, block, SOURCE_BLOCKS, 0.25);
    }
}

#[test]
fn meter_delayed_poll_delivers_each_queued_window_with_its_own_peak() {
    let mut host = live_control_host(128, 2);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    for (block, value) in [1.0_f32, 0.2, 0.3, 0.4].into_iter().enumerate() {
        feed_and_render(&mut host, 1, block as u64, value);
    }
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().windows, 1);
    assert_eq!(host.meter_header().first_sample, 0);
    assert_eq!(host.meter_header().end_sample, 256);
    assert_eq!(host.meter_frame()[0], 1.0);
    assert_eq!(host.meter_frame()[2], 1.0);
    assert_eq!(host.meter_frame()[3], 1.0);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().first_sample, 256);
    assert_eq!(host.meter_header().end_sample, 512);
    assert_eq!(host.meter_frame()[0], 0.4);
    assert_eq!(host.meter_frame()[2], 0.4);
    assert_eq!(host.meter_frame()[3], 0.4);

    feed_and_render(&mut host, 1, 4, 0.8);
    let partial_frame = host.meter_frame().to_vec();
    let partial_header = *host.meter_header();
    assert_eq!(
        host.poll_meters(),
        0,
        "a partial trailing period is not published"
    );
    assert_eq!(host.meter_frame(), partial_frame);
    assert_eq!(*host.meter_header(), partial_header);
}

#[test]
fn meter_invalid_master_rejects_transactionally_then_recovers_with_loss() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.9);
    assert_eq!(host.poll_meters(), 1);
    let published_frame = host.meter_frame().to_vec();
    let published_header = *host.meter_header();

    feed_and_render(&mut host, 1, 1, 0.2);
    assert!(
        host.ready
            .as_mut()
            .expect("ready")
            .pop_master_window()
            .is_some()
    );
    assert_eq!(host.poll_meters(), 0);
    assert_eq!(host.meter_frame(), published_frame);
    assert_eq!(*host.meter_header(), published_header);

    feed_and_render(&mut host, 1, 2, 0.3);
    assert_eq!(
        host.poll_meters(),
        0,
        "the first ready poll rejects the older track-only interval"
    );
    assert_eq!(host.poll_meters(), 1, "the same bounded readiness recovers");
    assert_eq!(host.meter_header().first_sample, 256);
    assert_eq!(&host.meter_frame()[..4], &[0.3, 0.3, 0.3, 0.3]);
    assert_ne!(host.meter_header().reserved[1] & METER_VALID_LOSS, 0);
}

#[test]
fn meter_producer_reset_starts_a_fresh_delivery_epoch() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.9);
    assert_eq!(host.poll_meters(), 1);
    let published_frame = host.meter_frame().to_vec();
    let published_header = *host.meter_header();

    feed_and_render(&mut host, 1, 1, 0.2);
    let ready = host.ready.as_mut().expect("ready");
    for (meter, pending) in ready.meters.iter_mut().zip(ready.meter_pending.iter_mut()) {
        let mut snapshot = meter.consumer.try_pop().expect("queued snapshot");
        snapshot.reset_generation = snapshot.reset_generation.saturating_add(1);
        snapshot.window_sequence = 0;
        *pending = Some(snapshot);
    }
    assert_eq!(host.poll_meters(), 0);
    assert_eq!(host.meter_frame(), published_frame);
    assert_eq!(*host.meter_header(), published_header);

    feed_and_render(&mut host, 1, 2, 0.3);
    assert_eq!(host.poll_meters(), 1);
    assert!(host.meter_header().reserved[0] > published_header.reserved[0]);
    assert_eq!(host.meter_header().first_sample, 256);
    assert_eq!(&host.meter_frame()[..4], &[0.3, 0.3, 0.3, 0.3]);
    assert_ne!(host.meter_header().reserved[1] & METER_VALID_LOSS, 0);
}

#[test]
fn meter_queue_saturation_recovers_with_explicit_loss() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    for block in 0..12 {
        feed_and_render(&mut host, 1, block, 0.2);
    }
    for block in 0..8 {
        assert_eq!(host.poll_meters(), 1);
        assert_eq!(host.meter_header().first_sample, block * 128);
    }
    assert_eq!(host.poll_meters(), 0);
    feed_and_render(&mut host, 1, 12, 0.3);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().first_sample, 12 * 128);
    assert_eq!(&host.meter_frame()[..4], &[0.3, 0.3, 0.3, 0.3]);
    assert_ne!(host.meter_header().reserved[1] & METER_VALID_LOSS, 0);
    assert!(host.meter_header().reserved[1] >> METER_LOSS_SHIFT > 0);
}

#[test]
fn meter_spans_cover_all_launch_rates_and_a_nine_track_tail() {
    const QUANTUM: u32 = 128;
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        let mut host = meter_tail_host(sample_rate_hz, QUANTUM);
        assert_eq!(host.set_meter_lease(true), RESULT_OK);
        feed_and_render(&mut host, 1, 0, 0.625);
        assert_eq!(host.poll_meters(), 1, "{sample_rate_hz} Hz");
        assert_eq!(host.meter_header().first_sample, 0);
        assert_eq!(host.meter_header().end_sample, u64::from(QUANTUM));
        assert_eq!(host.meter_header().track_count, 9);
        for track in 0..9 {
            assert_eq!(
                host.meter_frame()[track * 2],
                0.625,
                "rate {sample_rate_hz}, track {track} L"
            );
            assert_eq!(
                host.meter_frame()[track * 2 + 1],
                0.625,
                "rate {sample_rate_hz}, track {track} R"
            );
        }
        assert_eq!(
            host.meter_frame()[18],
            0.625,
            "{sample_rate_hz} Hz master L"
        );
        assert_eq!(
            host.meter_frame()[19],
            0.625,
            "{sample_rate_hz} Hz master R"
        );
    }
}

#[test]
fn source_seek_keeps_the_meter_epoch_and_absolute_peak_clock() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.4);
    assert_eq!(host.poll_meters(), 1);
    let meter_generation = host.meter_header().reserved[0];

    assert_eq!(host.seek_source(b"fixture-source", 2, 0), RESULT_OK);
    let plane = vec![0.7_f32; 128];
    let planes: [&[f32]; 2] = [&plane, &plane];
    assert_eq!(
        host.submit_source(b"fixture-source", 2, 0, 48_000, &planes, 128, false),
        RESULT_OK
    );
    assert_eq!(host.render_next(), RESULT_OK);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().reserved[0], meter_generation);
    assert_eq!(host.meter_header().first_sample, 128);
    assert_eq!(host.meter_header().end_sample, 256);
    assert_eq!(&host.meter_frame()[..4], &[0.7, 0.7, 0.7, 0.7]);
}

#[test]
fn meter_lease_reacquisition_waits_for_a_clean_boundary() {
    let mut host = live_control_host(128, 2);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.9);
    assert_eq!(host.set_meter_lease(false), RESULT_OK);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    let generation = host.meter_header().reserved[0];
    assert!(generation >= 2);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(host.meter_header().reserved[0], generation);
    for block in 1..4_u64 {
        feed_and_render(&mut host, 1, block, 0.2);
        if block < 3 {
            assert_eq!(host.poll_meters(), 0);
        }
    }
    assert_eq!(host.poll_meters(), 1);
    assert!(host.meter_header().first_sample >= 256);
    assert_eq!(host.meter_header().reserved[0], generation);
    assert_eq!(host.meter_frame()[0], 0.2);
    assert_eq!(host.meter_frame()[2], 0.2);
    assert_eq!(host.meter_frame()[3], 0.2);
}

#[test]
fn meter_reacquisition_rejects_a_full_stale_queue_then_recovers() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.9);
    assert_eq!(host.poll_meters(), 1);
    let old_generation = host.meter_header().reserved[0];

    // Fill the prepared producer queue while the consumer is delayed.
    for block in 1..=8 {
        feed_and_render(&mut host, 1, block, 0.2);
    }
    assert_eq!(host.set_meter_lease(false), RESULT_OK);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    let empty_frame = host.meter_frame().to_vec();
    let empty_header = *host.meter_header();
    assert!(empty_header.reserved[0] > old_generation);

    // The first fresh track window is dropped while stale producer slots remain full. One bounded
    // poll rejects those old windows and publishes nothing.
    feed_and_render(&mut host, 1, 9, 0.3);
    assert_eq!(host.poll_meters(), 0);
    assert_eq!(host.meter_frame(), empty_frame);
    assert_eq!(*host.meter_header(), empty_header);

    // With queue room restored, the next exact track/master interval publishes in the new epoch
    // and reports both the producer drop and discarded stale master interval.
    feed_and_render(&mut host, 1, 10, 0.4);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().reserved[0], empty_header.reserved[0]);
    assert_eq!(host.meter_header().first_sample, 10 * 128);
    assert_eq!(&host.meter_frame()[..4], &[0.4, 0.4, 0.4, 0.4]);
    assert_ne!(host.meter_header().reserved[1] & METER_VALID_LOSS, 0);
}

/// A three-track observation host over the #143 E4 fixture: compressor, EQ (no tap), gate.
fn observation_host(
    quantum: u32,
    meter_blocks: u64,
    master: Option<u32>,
) -> AudioWorkletEngineHost {
    let document = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        live_control_meter_blocks: meter_blocks,
        live_control_observation_taps: 4,
        live_control_master_track_plus_one: master.map_or(0, |track| u64::from(track) + 1),
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("observation boot")
}

/// One owner track with two resident effects. The gate is copied from the existing three-track
/// observation fixture so this test exercises two effect owners without introducing a second
/// session corpus.
fn same_track_observation_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture");
    let mut second_effect = model.tracks[2].inserts.effects[0].clone();
    // Remove the fixture's long hold so the quieter right lane closes during this short test
    // window and publishes a distinct, nonzero pair of resident values.
    second_effect.params[5].value = 0.0;
    second_effect.params[6].value = 5.0;
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 8;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.tracks[0].inserts.effects.push(second_effect);
    let document = canonical_session_json(&model).expect("canonical two-effect session");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        live_control_observation_taps: 4,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("two-effect observation boot")
}

/// Issue #1207 gates 1 and 4: two tracks and two submixes over the observation fixture.
///
/// The session declares console slots `pre_insert: [eq]` and `post_insert: [true-peak limiter]`.
/// Tracks `t0` (compressor insert) and `t1` (EQ insert) feed the submixes `aaa-bus`, which sorts
/// before every track ID, and `zzz-bus`, which sorts after; each bus carries every console slot
/// and a `miso.compressor` insert, and both buses feed the output. A lookup that binary-searched
/// the unsorted strip list `[t0, t1, aaa-bus, zzz-bus]` would miss `aaa-bus`.
///
/// This supersedes the K1 boot test (#1200 gate 6, extended by #1202 gate 7), whose
/// live-controlled bus boot it repeats with meters on.
fn bus_effect_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture");
    let slot = |slot: &str, effect: &str| session::ConsoleSlot {
        slot: session::StableId::parse(slot).expect("slot id"),
        identity: session::EffectIdentity::Native {
            effect_id: session::StableId::parse(effect).expect("effect id"),
        },
        quality: session::EffectQuality::Normal,
        link_mode: session::LinkMode::DualMono,
    };
    model.console.pre_insert = vec![slot("desk-eq", "miso.parametric-eq")];
    model.console.post_insert = vec![slot("desk-limit", "miso.true-peak-limiter")];
    model.tracks.truncate(2);
    model.routes.truncate(2);
    for track in &mut model.tracks {
        track.console = model
            .console
            .slots()
            .map(|slot| session::ConsoleEntry {
                slot: slot.slot.clone(),
                bypass: false,
                params: Vec::new(),
            })
            .collect();
    }
    let compressor = model.tracks[0].inserts.effects[0].clone();
    // The buses differ in shape (#1207 verdict MINOR-1): `aaa-bus` carries the compressor then
    // `t1`'s EQ, `zzz-bus` the compressor alone, so a host table filled in another submix order
    // than `handles.strips` misfiles every bus effect after the first.
    let equalizer = model.tracks[1].inserts.effects[0].clone();
    for (bus, feeder, inserts) in [
        ("aaa-bus", 0_usize, &[&compressor, &equalizer][..]),
        ("zzz-bus", 1, &[&compressor][..]),
    ] {
        let id = session::StableId::parse(bus).expect("bus id");
        let mut submix = session::Submix::unity(id.clone(), &model.console);
        for entry in &mut submix.console {
            entry.bypass = false;
        }
        submix
            .inserts
            .effects
            .extend(inserts.iter().map(|&effect| effect.clone()));
        model.submixes.push(submix);
        let mut out = model.routes[feeder].clone();
        model.routes[feeder].destination = session::RouteDestination::SubmixInput {
            submix_id: id.clone(),
        };
        out.id = session::StableId::parse(&format!("{bus}-main")).expect("route id");
        out.source = session::RouteSource::Submix {
            submix_id: id,
            tap: session::SendTap::PostPan,
        };
        model.routes.push(out);
    }
    model.quantum_frames = quantum;
    // Longer than the rendered blocks, so the last one is not the region's final block.
    model.sources[0].frames = u64::from(quantum) * 16;
    let document = canonical_session_json(&model).expect("canonical bus session");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: 64,
        live_control_meter_blocks: 2,
        live_control_observation_taps: 1,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
        panic!(
            "live-controlled bus boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

/// Issue #1207 gate 1: a bus session boots live-controlled, renders, and files every bus effect's
/// producer and observation handle at its strip's dense slot.
///
/// Red if a bus producer or observation handle is refused or misfiled, if a lookup
/// binary-searches the unsorted strip list (`aaa-bus` is missed and boot refuses), or if the K1
/// interim (DESIGN P16) is left in place (the bus slots stay `None`). The buses' insert chains
/// differ in length, so it is also red if host-web fills its per-strip tables in another submix
/// order than `handles.strips` (#1207 verdict MINOR-1).
#[test]
fn a_bus_session_boots_live_controlled_and_files_every_bus_effect() {
    const QUANTUM: u32 = 128;
    let mut host = bus_effect_host(QUANTUM);
    for block in 0..8 {
        feed_and_render_tracks(&mut host, block, 0.25);
    }
    let ready = host.ready.as_ref().expect("ready ownership");
    assert_eq!(ready.tracks, [Box::from("t0"), Box::from("t1")]);
    assert_eq!(ready.submixes, [Box::from("aaa-bus"), Box::from("zzz-bus")]);
    assert_eq!(host.live_control_tracks().len(), 2, "the track prefix only");
    // Strip order, computed here: the tracks, then the submixes at `T + j`.
    let strips = ["t0", "t1", "aaa-bus", "zzz-bus"];
    // Every strip carries the two console slots (slot 0 the EQ, slot 1 the limiter) and its own
    // inserts: the compressor on `t0`, the EQ on `t1`, the compressor then the EQ on `aaa-bus`,
    // the compressor alone on `zzz-bus`.
    let inserts: [&[&str]; 4] = [
        &["miso.compressor"],
        &["miso.parametric-eq"],
        &["miso.compressor", "miso.parametric-eq"],
        &["miso.compressor"],
    ];
    let mut filed = 0;
    for (strip, id) in strips.iter().enumerate() {
        let console = [
            (
                LiveEffectAddress {
                    rack: LiveEffectRack::Console,
                    index: 0,
                },
                "miso.parametric-eq",
            ),
            (
                LiveEffectAddress {
                    rack: LiveEffectRack::Console,
                    index: 1,
                },
                "miso.true-peak-limiter",
            ),
        ];
        let chain = inserts[strip].iter().enumerate().map(|(index, &native)| {
            (
                LiveEffectAddress {
                    rack: LiveEffectRack::Inserts,
                    index: u32::try_from(index).expect("insert index"),
                },
                native,
            )
        });
        for (address, native) in console.into_iter().chain(chain) {
            let slot =
                dense_effect_slot(ready.effect_base[strip], ready.rack_effects[strip], address)
                    .unwrap_or_else(|| panic!("{id} {address:?} has a dense slot"));
            let producer = ready.effect_controls[slot]
                .as_ref()
                .unwrap_or_else(|| panic!("{id} {address:?} has a filed producer"));
            assert_eq!(&*producer.track_id, *id, "slot {slot}'s producer owner");
            assert_eq!(producer.address, address, "slot {slot}'s producer address");
            assert_eq!(
                producer.descriptor.id.as_str(),
                native,
                "slot {slot}'s effect"
            );
            let observed = ready.effect_observations[slot].as_ref();
            if native == "miso.parametric-eq" {
                assert!(
                    observed.is_none(),
                    "{id} {address:?}: an EQ declares no tap"
                );
                assert_eq!(ready.observation_tracks[slot], u32::MAX);
            } else {
                let handle =
                    observed.unwrap_or_else(|| panic!("{id} {address:?} has a filed observer"));
                assert_eq!(&*handle.track_id, *id, "slot {slot}'s observer owner");
                assert_eq!(handle.address, address, "slot {slot}'s observer address");
                assert_eq!(ready.observation_tracks[slot] as usize, strip);
            }
            filed += 1;
        }
    }
    assert_eq!(filed, 13);
    assert_eq!(
        ready.effect_controls.len(),
        13,
        "no producer slot beyond the strips'"
    );
    assert_eq!(ready.observation_present.len(), strips.len());
}

/// Issue #1207 gate 4: with every bus effect filed, a live-control submission to a track effect
/// plus `render_next` allocates and frees nothing.
///
/// Red if filing bus effects adds a render- or admission-time allocation, for example a
/// per-block lookup that builds a table.
#[test]
fn a_bus_session_admits_and_renders_without_allocating() {
    const QUANTUM: u32 = 128;
    let mut host = bus_effect_host(QUANTUM);
    for block in 0..2 {
        feed_and_render_tracks(&mut host, block, 0.25);
    }
    // `t0`'s compressor insert: track 0, rack `inserts` (1), effect 0, bypassed.
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
    let left = [0.25_f32; QUANTUM as usize];
    let right = [0.25_f32; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            2 * u64::from(QUANTUM),
            48_000,
            &planes,
            QUANTUM,
            false,
        ),
        RESULT_OK
    );
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            let admission = host.submit_commands(1);
            let render = host.render_next();
            (admission, render)
        });
    assert_eq!(
        admission, RESULT_OK,
        "the track effect's bypass is admitted"
    );
    assert_eq!(render, RESULT_OK, "the bus session renders");
    assert_eq!(allocations, 0, "admission/render allocated");
    assert_eq!(deallocations, 0, "admission/render freed");
}

/// Issue #1209 gates 1 and 5: three effect-free tracks over the observation fixture's source,
/// each with its own fader pair so every track lane carries a distinct constant level.
///
/// With `buses`, `t0` and `t1` route into the unity submix `aaa-bus` and `t2` into `zzz-bus`, and
/// both buses feed the output; without, every track feeds the output directly (the `S = 0` twin
/// whose track and master words are today's computation).
fn bus_meter_host(quantum: u32, buses: bool) -> AudioWorkletEngineHost {
    bus_meter_host_with(quantum, buses, 2, 64)
}

/// [`bus_meter_host`] with its meter period (`meter_blocks` render quanta per window) and its
/// source length (`source_quanta` render quanta) chosen by the caller (issue #1448 D6).
fn bus_meter_host_with(
    quantum: u32,
    buses: bool,
    meter_blocks: u64,
    source_quanta: u64,
) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture");
    for (track, (left_db, right_db)) in
        model
            .tracks
            .iter_mut()
            .zip([(-6.0, -3.0), (-12.0, -1.0), (-2.0, -9.0)])
    {
        track.inserts.effects.clear();
        track.fader.left_db = left_db;
        track.fader.right_db = right_db;
    }
    if buses {
        for (bus, feeders) in [("aaa-bus", &[0_usize, 1][..]), ("zzz-bus", &[2][..])] {
            let id = session::StableId::parse(bus).expect("bus id");
            model
                .submixes
                .push(session::Submix::unity(id.clone(), &model.console));
            let mut out = model.routes[feeders[0]].clone();
            for &feeder in feeders {
                model.routes[feeder].destination = session::RouteDestination::SubmixInput {
                    submix_id: id.clone(),
                };
            }
            out.id = session::StableId::parse(&format!("{bus}-main")).expect("route id");
            out.source = session::RouteSource::Submix {
                submix_id: id,
                tap: session::SendTap::PostPan,
            };
            model.routes.push(out);
        }
    }
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * source_quanta;
    let document = canonical_session_json(&model).expect("canonical bus meter session");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: 64,
        live_control_meter_blocks: meter_blocks,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
        panic!(
            "bus meter boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

/// Issue #1448 D6: after a window is lost on one or two meters, the poll that recovers publishes
/// the window just rendered and leaves no complete window queued, at one block per window. A poll
/// whose passes were bounded by the minimum over the queues would run out of passes before it
/// could publish, and lag one window behind render for as long as the stream runs.
#[test]
fn meter_gap_on_one_meter_leaves_no_backlog_at_one_block_windows() {
    let mut host = bus_meter_host_with(128, true, 1, 64);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    let backlog = |host: &AudioWorkletEngineHost| -> usize {
        let ready = host.ready.as_ref().expect("ready");
        ready
            .meters
            .iter()
            .zip(ready.meter_pending.iter())
            .map(|(meter, pending)| {
                meter.consumer.available_at_entry() + usize::from(pending.is_some())
            })
            .min()
            .expect("meters")
    };
    assert_eq!(host.ready.as_ref().expect("ready").meters.len(), 5);
    for block in 0..40_u64 {
        feed_and_render_channels(&mut host, block, 0.5, 0.25);
        let dropped: &[usize] = match block {
            10 => &[2],
            20 => &[0],
            30 => &[1, 3],
            _ => &[],
        };
        let ready = host.ready.as_mut().expect("ready");
        for &meter in dropped {
            assert!(
                ready.meters[meter].consumer.try_pop().is_ok(),
                "block {block}: meter {meter} holds the window just rendered"
            );
        }
        let published = host.poll_meters();
        if matches!(block, 11..=19 | 21..=29 | 31..=39) {
            assert_eq!(published, 1, "block {block}: one window published");
            assert_eq!(
                host.meter_header().first_sample,
                block * 128,
                "block {block}: the window just rendered"
            );
            assert_eq!(backlog(&host), 0, "block {block}: no window left queued");
        }
        if matches!(block, 11 | 21 | 31) {
            assert_ne!(
                host.meter_header().reserved[1] & METER_VALID_LOSS,
                0,
                "block {block}: the loss is reported"
            );
        }
    }
}

/// Render `blocks` quanta of the constant `(0.5, 0.25)` source pair and poll once.
fn bus_meter_frame(host: &mut AudioWorkletEngineHost, blocks: u64) -> Vec<f32> {
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    for block in 0..blocks {
        feed_and_render_channels(host, block, 0.5, 0.25);
    }
    assert_eq!(host.poll_meters(), 1, "one complete window");
    host.meter_frame().to_vec()
}

/// Issue #1209 gate 1: with `T = 3` and `S = 2` the frame is `3(T + S) + 3` words, each bus's
/// `PostMatrix` peak pair sits after the tracks, the master follows the buses, and every gain
/// reduction word is zero.
///
/// Red if any writer still assumes `3T + 3`, if submix peaks land in the master's slots, or if a
/// bus is unmetered.
#[test]
fn the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix() {
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 8;
    let mut twin = bus_meter_host(QUANTUM, false);
    let today = bus_meter_frame(&mut twin, BLOCKS);
    assert_eq!(today.len(), 3 * 3 + 3, "the S = 0 twin keeps 3T + 3");
    assert_eq!(twin.meter_header().submix_count, 0);

    let mut host = bus_meter_host(QUANTUM, true);
    let frame = bus_meter_frame(&mut host, BLOCKS);
    let header = *host.meter_header();
    assert_eq!(header.struct_size, 72);
    assert_eq!(header.track_count, 3);
    assert_eq!(header.submix_count, 2);
    assert_eq!(header.reserved_pad, 0);
    let (tracks, strips) = (3_usize, 5_usize);
    assert_eq!(frame.len(), 3 * strips + 3, "3(T + S) + 3 words");

    // Track peaks: today's words, bit for bit (track processing does not see the routing).
    for word in 0..2 * tracks {
        assert_eq!(
            frame[word].to_bits(),
            today[word].to_bits(),
            "track peak word {word}"
        );
    }
    let close = |actual: f32, expected: f32| (actual - expected).abs() <= expected.abs() * 1e-6;
    // Bus peaks at `2(T + j)`: a unity bus's `PostMatrix` level is the sum of its feeders'.
    for lane in 0..2 {
        let aaa = frame[2 * tracks + lane];
        let zzz = frame[2 * (tracks + 1) + lane];
        assert!(aaa > 0.0 && zzz > 0.0, "lane {lane}: every bus is metered");
        assert!(
            close(aaa, frame[lane] + frame[2 + lane]),
            "lane {lane}: aaa-bus {aaa} is t0 + t1"
        );
        assert!(
            close(zzz, frame[4 + lane]),
            "lane {lane}: zzz-bus {zzz} is t2"
        );
        // Master at `2(T + S)`: today's master, which no longer sits after the tracks.
        let master = frame[2 * strips + lane];
        assert!(
            close(master, today[2 * tracks + lane]),
            "lane {lane}: master {master} against today's {}",
            today[2 * tracks + lane]
        );
        assert!(
            close(master, aaa + zzz),
            "lane {lane}: the master is the bus sum"
        );
    }
    // Gain reduction from `2(T + S) + 2`: tracks, buses, master; nothing is armed.
    for (word, value) in frame.iter().enumerate().skip(2 * strips + 2) {
        assert_eq!(value.to_bits(), 0.0_f32.to_bits(), "gain word {word}");
    }
    assert_eq!(header.master_gr_present, 0);
}

/// Issue #1209 gate 5 (also #1207 verdict MINOR-2): on a session with submix strips carrying
/// effects, with a meter on every strip and a track observation armed, a render that closes a
/// window plus the per-block `poll_meters` that publishes the widened frame -- peaks for every
/// strip and the gain-reduction fold -- allocates and frees nothing.
///
/// Red if the widened poll, its gain-reduction fold or the per-strip meters allocate per window;
/// #1207's gate 4 measures admission and render only, never the poll.
#[test]
fn bus_meters_render_and_poll_without_allocating() {
    const QUANTUM: u32 = 128;
    let mut host = bus_effect_host(QUANTUM);
    assert_eq!(host.meter_header().submix_count, 2);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    // `t0`'s compressor insert, tap 1, so the poll's gain-reduction fold reads a window.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    for block in 0..5 {
        feed_and_render_tracks(&mut host, block, 0.5);
    }
    while host.poll_meters() > 0 {}
    let left = [0.5_f32; QUANTUM as usize];
    let right = [0.5_f32; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            5 * u64::from(QUANTUM),
            48_000,
            &planes,
            QUANTUM,
            false,
        ),
        RESULT_OK
    );
    let ((render, windows), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            let render = host.render_next();
            (render, host.poll_meters())
        });
    assert_eq!(render, RESULT_OK);
    assert_eq!(windows, 1, "the measured poll published the widened frame");
    assert_ne!(
        host.meter_header().reserved[1] & METER_VALID_GAIN_REDUCTION,
        0,
        "the measured poll folded gain reduction"
    );
    assert_eq!(host.meter_frame().len(), 3 * 4 + 3, "3(T + S) + 3 words");
    assert_eq!(allocations, 0, "render/poll allocated");
    assert_eq!(deallocations, 0, "render/poll freed");
}

/// Issue #1210 gate 1: three tracks over the observation fixture, and with `buses` two unity
/// submixes declared out of canonical order: a 63-byte `zz-` bus, longer than every source and
/// track ID, fed by `t0` and `t1`, then `a-bus`, which sorts before every track ID, fed by `t2`.
fn submix_name_document(quantum: u32, buses: bool) -> (String, [String; 2]) {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture");
    let long = format!("zz-{}", "x".repeat(60));
    let canonical = ["a-bus".to_owned(), long.clone()];
    if buses {
        for (bus, feeders) in [(long.as_str(), &[0_usize, 1][..]), ("a-bus", &[2][..])] {
            let id = session::StableId::parse(bus).expect("bus id");
            model
                .submixes
                .push(session::Submix::unity(id.clone(), &model.console));
            let mut out = model.routes[feeders[0]].clone();
            for &feeder in feeders {
                model.routes[feeder].destination = session::RouteDestination::SubmixInput {
                    submix_id: id.clone(),
                };
            }
            out.id = session::StableId::parse(&format!("{bus}-main")).expect("route id");
            out.source = session::RouteSource::Submix {
                submix_id: id,
                tap: session::SendTap::PostPan,
            };
            model.routes.push(out);
        }
    }
    model.quantum_frames = quantum;
    let document = canonical_session_json(&model).expect("canonical submix name session");
    (document, canonical)
}

/// Issue #1210 gate 1: the submix enumeration exports report every submix ID byte for byte in
/// canonical order, through ID staging sized for the longest submix ID.
///
/// Red if the staging capacity ignores submix IDs (the long ID's copy overruns the buffer and
/// traps) or if the export orders submixes other than canonically (the frame's order, #1209 D3).
#[test]
fn submix_ids_enumerate_in_canonical_order_through_staging_sized_for_them() {
    const QUANTUM: u32 = 128;
    let options = WebBootOptions {
        source_ring_frames: QUANTUM * 4,
        ..boot_options(QUANTUM)
    };
    let (document, canonical) = submix_name_document(QUANTUM, true);
    let handle = crate::ffi::test_boot(document.as_bytes(), options);
    assert_ne!(handle, 0, "the submix session boots");
    let resources = crate::ffi::test_resources(handle).expect("resource report");
    // A ceiling claim (#1210 NIT-4): staging holds at least the longest submix ID.
    assert!(resources.id_staging_bytes >= canonical[1].len() as u64);

    assert_eq!(miso_engine_web_v1_live_control_submix_count(handle), 2);
    assert_eq!(miso_engine_web_v1_live_control_track_count(handle), 3);
    for (index, expected) in canonical.iter().enumerate() {
        let length = miso_engine_web_v1_live_control_submix_id(handle, index as u32);
        assert_eq!(length, expected.len() as u32);
        assert_eq!(
            crate::ffi::test_read_source_id(handle, length).expect("staged submix ID"),
            expected.as_bytes()
        );
    }
    assert_eq!(miso_engine_web_v1_live_control_submix_id(handle, 2), 0);
    assert_eq!(
        miso_engine_web_v1_live_control_submix_id(handle, u32::MAX),
        0
    );
    // An invalid handle answers zero, as the track queries do.
    assert_eq!(
        miso_engine_web_v1_live_control_submix_count(handle.wrapping_add(1)),
        0
    );
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_live_control_submix_count(handle), 0);

    let (document, _) = submix_name_document(QUANTUM, false);
    let handle = crate::ffi::test_boot(document.as_bytes(), options);
    assert_ne!(handle, 0, "the submix-free session boots");
    assert_eq!(miso_engine_web_v1_live_control_submix_count(handle), 0);
    assert_eq!(miso_engine_web_v1_live_control_submix_id(handle, 0), 0);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
}

/// Issue #1223 gate 4 and D1: the route exports enumerate exactly the live sends, in the order a
/// send kind's index word reads, through ID staging sized for the longest route ID.
///
/// Two tracks feed one unity bus through two sends: a 63-byte `zz-` send, longer than every
/// source, track and submix ID, and `send-a`; every strip also routes to the output, and
/// `bx-main` sorts before both sends. Red if the staging capacity ignores route IDs (the long
/// ID's copy overruns the buffer and traps), or if the export enumerates any order or subset
/// other than the live send producers' (all routes would put `bx-main` at index 0).
#[test]
fn live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them() {
    let long = format!("zz-{}", "x".repeat(60));
    let (mut model, source, track, _, route) = strip_base();
    let output = route.destination.clone();
    for id in ["t0", "t1"] {
        strip_add_track(&mut model, &source, &track, id);
    }
    model.submixes = vec![session::Submix::unity(strip_id("bx"), &model.console)];
    for (id, from) in [(long.as_str(), "t0"), ("send-a", "t1")] {
        model.routes.push(strip_route(
            &route,
            id,
            strip_post_pan(from, false),
            strip_into("bx"),
            [1.0, 0.0, 0.0, 1.0],
        ));
    }
    for strip in ["t0", "t1", "bx"] {
        model.routes.push(strip_route(
            &route,
            &format!("{strip}-main"),
            strip_post_pan(strip, strip == "bx"),
            output.clone(),
            [1.0, 0.0, 0.0, 1.0],
        ));
    }
    let document = canonical_session_json(&model).expect("long send session canonicalizes");

    let handle = crate::ffi::test_boot(document.as_bytes(), strip_options(4, 0, 0));
    assert_ne!(handle, 0, "the long-send session boots");
    let resources = crate::ffi::test_resources(handle).expect("resource report");
    assert_eq!(resources.id_staging_bytes, long.len() as u64);
    assert_eq!(miso_engine_web_v1_live_control_route_count(handle), 2);
    for (index, expected) in ["send-a", long.as_str()].iter().enumerate() {
        let length = miso_engine_web_v1_live_control_route_id(handle, index as u32);
        assert_eq!(length, expected.len() as u32, "live route {index}");
        assert_eq!(
            crate::ffi::test_read_source_id(handle, length).expect("staged route ID"),
            expected.as_bytes()
        );
    }
    assert_eq!(miso_engine_web_v1_live_control_route_id(handle, 2), 0);
    assert_eq!(
        miso_engine_web_v1_live_control_route_count(handle.wrapping_add(1)),
        0
    );
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);

    // Without live controls there are no send producers, so no live route.
    let handle = crate::ffi::test_boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: STRIP_QUANTUM * 4,
            ..boot_options(STRIP_QUANTUM)
        },
    );
    assert_ne!(
        handle, 0,
        "the long-send session boots without live controls"
    );
    assert_eq!(miso_engine_web_v1_live_control_route_count(handle), 0);
    assert_eq!(miso_engine_web_v1_live_control_route_id(handle, 0), 0);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
}

#[test]
fn selected_observation_reads_are_bounded_stable_and_non_consuming() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 2, None);
    let selection = ObservationSelection {
        track_id: "t0",
        rack: LiveEffectRack::Inserts,
        effect_slot_id: "comp",
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };

    let unarmed = host
        .read_observations(&[selection])
        .expect("unarmed selected read");
    assert_eq!(unarmed[0].status, ObservationReadStatus::Unarmed);
    assert_eq!(unarmed[0].window, None);

    assert_eq!(
        observe(&mut host, 0, 1, 0, 1, 2, true),
        RESULT_OK,
        "the existing observe command remains the only arming path"
    );
    let pending = host
        .read_observations(&[selection])
        .expect("pending selected read");
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, ObservationReadStatus::Pending);
    assert_eq!(pending[0].window, None);
    assert_eq!(pending[0].native_effect_id, "miso.compressor");
    assert_eq!(pending[0].descriptor.display_unit, "dB");

    for block in 0..2 {
        feed_and_render_tracks(&mut host, block, 0.0);
    }
    let expected = {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (effect, tap) = resolve_observation(ready, &selection).expect("resolved selection");
        ready.effect_observations[effect]
            .as_ref()
            .expect("observation handle")
            .readers[tap]
            .read()
            .expect("published window")
    };
    let rows = host
        .read_observations(&[selection])
        .expect("ready selected read");
    assert_eq!(rows[0].status, ObservationReadStatus::Ready);
    assert_eq!(rows[0].window, Some(expected));
    assert_eq!(rows[0].left, Some(expected.left));
    assert_eq!(rows[0].right, Some(expected.right));
    assert_eq!(
        rows[0].window.expect("row window").sequence,
        expected.sequence
    );
    assert_eq!(
        rows[0].window.expect("row window").first_sample,
        expected.first_sample
    );

    // A re-arm is admitted before the next render block, but the reader still retains the prior
    // resident cell. That old publication is before the admitted application sample and must not
    // look ready while a replacement window is being built.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, false), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    let rearmed_pending = host
        .read_observations(&[selection])
        .expect("re-arm pending selected read");
    assert_eq!(rearmed_pending[0].status, ObservationReadStatus::Pending);
    assert_eq!(rearmed_pending[0].window, None);

    // A second observed effect is read in the same caller-ordered batch. Both rows are sourced
    // from their own resident cells, and neither selected read consumes the publication.
    let gate_selection = ObservationSelection {
        track_id: "t2",
        rack: LiveEffectRack::Inserts,
        effect_slot_id: "gate",
        tap_id: 1,
        channels: ObservationReadChannels::Left,
    };
    assert_eq!(observe(&mut host, 2, 1, 0, 1, 2, true), RESULT_OK);
    for block in 2..4 {
        feed_and_render_tracks(&mut host, block, 0.0);
    }
    let expected_gate = {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (effect, tap) = resolve_observation(ready, &gate_selection).expect("resolved gate");
        ready.effect_observations[effect]
            .as_ref()
            .expect("gate observation handle")
            .readers[tap]
            .read()
            .expect("gate published window")
    };
    let pair = host
        .read_observations(&[selection, gate_selection])
        .expect("two-effect selected read");
    assert_eq!(pair.len(), 2);
    assert_eq!(pair[0].track_id.as_ref(), "t0");
    assert_eq!(pair[0].window.map(|window| window.first_sample), Some(256));
    assert_eq!(pair[1].track_id.as_ref(), "t2");
    assert_eq!(pair[1].window, Some(expected_gate));
    assert_eq!(pair[1].left, Some(expected_gate.left));
    assert_eq!(pair[1].right, None);
    let pair_again = host
        .read_observations(&[selection, gate_selection])
        .expect("repeat two-effect selected read");
    assert_eq!(pair_again[0].window, pair[0].window);
    assert_eq!(pair_again[1].window, pair[1].window);
    let expected_rearmed = pair[0].window.expect("re-armed compressor window");

    // A selected read does not acknowledge the shared reader. The same exact publication is
    // returned again, while a different requested channel only changes the owned projection.
    let right = host
        .read_observations(&[ObservationSelection {
            channels: ObservationReadChannels::Right,
            ..selection
        }])
        .expect("repeat selected read");
    assert_eq!(right[0].window, Some(expected_rearmed));
    assert_eq!(right[0].left, None);
    assert_eq!(right[0].right, Some(expected_rearmed.right));

    let invalid = [
        selection,
        ObservationSelection {
            effect_slot_id: "missing",
            ..selection
        },
    ];
    assert_eq!(
        host.read_observations(&invalid).unwrap_err(),
        ObservationReadError::InvalidSelection,
        "the complete batch is validated before any row is returned"
    );
    assert_eq!(
        host.read_observations(&[selection, selection]).unwrap_err(),
        ObservationReadError::InvalidSelection
    );
}

#[test]
fn selected_observation_reads_keep_same_track_owner_windows_and_sequences() {
    const QUANTUM: u32 = 128;
    let mut host = same_track_observation_host(QUANTUM);
    let compressor = ObservationSelection {
        track_id: "t0",
        rack: LiveEffectRack::Inserts,
        effect_slot_id: "comp",
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };
    let gate = ObservationSelection {
        track_id: "t0",
        rack: LiveEffectRack::Inserts,
        effect_slot_id: "gate",
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 1, 1, 3, true), RESULT_OK);
    for block in 0..3 {
        feed_and_render_channels(&mut host, block, 0.75, 0.125);
    }

    let (compressor_window, compressor_consumed) = {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (effect, tap) = resolve_observation(ready, &compressor).expect("compressor address");
        let reader = &ready.effect_observations[effect]
            .as_ref()
            .expect("compressor observation handle")
            .readers[tap];
        (
            reader.read().expect("compressor published window"),
            reader.consumed_sequence(),
        )
    };
    let (gate_window, gate_consumed) = {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (effect, tap) = resolve_observation(ready, &gate).expect("gate address");
        let reader = &ready.effect_observations[effect]
            .as_ref()
            .expect("gate observation handle")
            .readers[tap];
        (
            reader.read().expect("gate published window"),
            reader.consumed_sequence(),
        )
    };
    assert_eq!(compressor_window.blocks, 2);
    assert_eq!(gate_window.blocks, 3);
    assert_ne!(compressor_window.left, compressor_window.right);
    assert_ne!(gate_window.left, gate_window.right);
    assert_eq!(compressor_consumed, 0);
    assert_eq!(gate_consumed, 0);

    let rows = host
        .read_observations(&[compressor, gate])
        .expect("same-track two-effect selected read");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].effect_slot_id.as_ref(), "comp");
    assert_eq!(rows[1].effect_slot_id.as_ref(), "gate");
    assert_eq!(rows[0].window, Some(compressor_window));
    assert_eq!(rows[1].window, Some(gate_window));
    assert_eq!(rows[0].left, Some(compressor_window.left));
    assert_eq!(rows[0].right, Some(compressor_window.right));
    assert_eq!(rows[1].left, Some(gate_window.left));
    assert_eq!(rows[1].right, Some(gate_window.right));

    let (compressor_after, gate_after) = {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (compressor_effect, compressor_tap) =
            resolve_observation(ready, &compressor).expect("compressor address");
        let (gate_effect, gate_tap) = resolve_observation(ready, &gate).expect("gate address");
        (
            ready.effect_observations[compressor_effect]
                .as_ref()
                .expect("compressor observation handle")
                .readers[compressor_tap]
                .consumed_sequence(),
            ready.effect_observations[gate_effect]
                .as_ref()
                .expect("gate observation handle")
                .readers[gate_tap]
                .consumed_sequence(),
        )
    };
    assert_eq!(compressor_after, compressor_consumed);
    assert_eq!(gate_after, gate_consumed);
}

#[test]
fn selected_observation_numeric_read_refuses_failed_host_without_touching_output() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 0, None);
    host.status.next_absolute_sample = u64::MAX;
    assert_eq!(host.render_next(), RESULT_RENDER_REJECTED);
    assert_eq!(host.status().state, STATE_FAILED);
    let address = ObservationAddress {
        track_index: 0,
        rack: LiveEffectRack::Inserts,
        effect_index: 0,
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };
    let sentinel = ObservationReadValues {
        status: ObservationReadStatus::Ready,
        sample_rate_hz: 48_000,
        left: Some(3.5),
        right: Some(-2.0),
        ..ObservationReadValues::default()
    };
    let mut output = [sentinel];
    assert_eq!(
        host.read_observation_addresses_into(&[address], &mut output),
        Err(ObservationReadError::WrongState)
    );
    assert_eq!(
        output,
        [sentinel],
        "a failed read cannot overwrite caller output"
    );
    assert!(
        host.ready.is_some(),
        "the failed host retains its prepared owner"
    );
}

/// Feed one quantum of a constant to every track's shared source and render it.
fn feed_and_render_tracks(host: &mut AudioWorkletEngineHost, block: u64, value: f32) {
    feed_and_render(host, 1, block, value);
}

/// Feed distinct nonzero owner lanes to the shared source and render one quantum.
fn feed_and_render_channels(
    host: &mut AudioWorkletEngineHost,
    block: u64,
    left_value: f32,
    right_value: f32,
) {
    let quantum = host.status().quantum_frames as usize;
    let left = vec![left_value; quantum];
    let right = vec![right_value; quantum];
    let planes: [&[f32]; 2] = [&left, &right];
    assert_eq!(
        host.submit_source(
            b"fixture-source",
            1,
            block * quantum as u64,
            host.status().sample_rate_hz,
            &planes,
            quantum as u32,
            false,
        ),
        RESULT_OK,
    );
    assert_eq!(host.render_next(), RESULT_OK);
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
/// The submix words between them (issue #1209 D1) are skipped.
fn gain_reduction(host: &AudioWorkletEngineHost) -> (Vec<f32>, Option<f32>) {
    let tracks = host.live_control_tracks().len();
    let strips = tracks + host.meter_header().submix_count as usize;
    let frame = host.meter_frame();
    assert_eq!(
        frame.len(),
        strips * 3 + 3,
        "the frame is 3(T + S) + 3 words"
    );
    let base = strips * 2 + 2;
    let per_track = frame[base..base + tracks].to_vec();
    let master = (host.meter_header().master_gr_present == 1).then(|| frame[base + strips]);
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
    assert_eq!(host.live_control_tracks().len(), 3);
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

    // Unknown rack and unknown effect keep their own reasons on the observe kinds. The retired
    // `simd1`/`simd2` codes are unknown racks (issue #1096); this session has no console slot, so
    // `console` is a known rack naming an unknown effect.
    for retired in [0, 2, 4] {
        assert_eq!(
            observe(&mut host, 0, retired, 0, 1, 2, true),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_RACK);
    }
    assert_eq!(
        observe(&mut host, 0, RACK_CONSOLE, 0, 1, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_EFFECT);
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
    let depth = host.options().live_control_command_queue_records as usize;
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

/// A session with live controls but no observation capacity refuses a subscription with its own
/// reason: the effect exists, the tap is declared, and this preparation bound no lane.
#[test]
fn a_subscription_without_capacity_is_observation_unbound() {
    const QUANTUM: u32 = 128;
    let document = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
    let options = WebBootOptions {
        source_ring_frames: QUANTUM * 4,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        live_control_meter_blocks: DEFAULT_METER_BLOCKS as u64,
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
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
    let tracks = host.live_control_tracks().len();
    assert_eq!(host.meter_frame().len(), tracks * 3 + 3);
    assert!(host.meter_frame().iter().all(|value| *value == 0.0));
}

/// Observation capacity is refused at configuration time when nothing can carry the subscription.
#[test]
fn observation_configuration_words_are_validated() {
    let document = one_track_session(128);
    for options in [
        WebBootOptions {
            live_control_observation_taps: 4,
            ..boot_options(128)
        },
        WebBootOptions {
            live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
            live_control_observation_taps: u64::from(MAXIMUM_OBSERVATION_TAPS) + 1,
            ..boot_options(128)
        },
        WebBootOptions {
            live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
            live_control_master_track_plus_one: 1,
            ..boot_options(128)
        },
    ] {
        let failure = AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .err()
            .expect("invalid live-control options");
        assert_eq!(failure.result(), RESULT_REFUSED_OPTIONS);
    }
    AudioWorkletEngineHost::boot(document.as_bytes(), WebBootOptions::live_control_defaults())
        .expect("zero observation words remain valid");
}

/// Issue #143 E1/E12, native leg: the observation timeline's determinism digest.
///
/// The native twin of `direct-oracle.mjs`'s `runObservationTimeline`. Both legs read this exact
/// fixture file, run this exact twelve-block timeline, and their digests are asserted equal to one
/// pin -- so a change to the audio makes them move together, which is the point.
///
/// Two runs, one timeline: with observation capacity and every declared tap armed, and with
/// `live_control_observation_taps == 0`. They must render **identical bits**. That is E1's leg (b)
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

    let document = include_str!("../tests/browser-v1/observation-session.json");
    let run = |taps: u64| -> (String, f32, Option<f32>, f32, u64, u32, u32) {
        let options = WebBootOptions {
            source_ring_frames: QUANTUM,
            live_control_command_queue_records: u64::from(DEPTH),
            live_control_meter_blocks: u64::from(WINDOW_BLOCKS),
            live_control_observation_taps: taps,
            live_control_master_track_plus_one: if taps == 0 { 0 } else { 1 },
            ..boot_options(QUANTUM)
        };
        let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
        assert_eq!(host.live_control_tracks().len(), 1);
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
        let tracks = host.live_control_tracks().len();
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
        let hex = engine::hex_lower(&digest.finalize());
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
    use effect_contract::{
        EffectDescriptor, EffectId, LinkModeSet, ObservationCadence, ObservationChannels,
        ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind, ObservationTapId,
        ParameterUnit,
    };

    const fn tap(
        id: u32,
        cost: ObservationCost,
        cadence: ObservationCadence,
    ) -> ObservationDescriptor {
        ObservationDescriptor {
            id: ObservationTapId(id),
            display_name: "Gain Reduction",
            display_unit: "dB",
            kind: ObservationKind::GainReductionDb,
            unit: ParameterUnit::Db,
            cost,
            cadence,
            fold: ObservationFold::PeakMagnitude,
            channels: ObservationChannels::Shared,
            minimum: 0.0,
            maximum: 100.0,
        }
    }
    static MENU: [ObservationDescriptor; 2] = [
        tap(1, ObservationCost::Resident, ObservationCadence::PerBlock),
        tap(2, ObservationCost::Computed, ObservationCadence::PerWindow),
    ];
    static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
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
        tail_and_rest: |_, _| effect_contract::NodeTailBound {
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
        },
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
    use effect_contract::{
        ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
        ObservationFold, ObservationKind, ObservationTapId, ParameterUnit,
    };
    const fn tap(unit: ParameterUnit) -> ObservationDescriptor {
        ObservationDescriptor {
            id: ObservationTapId(1),
            display_name: "Gain Reduction",
            display_unit: "dB",
            kind: ObservationKind::GainReductionDb,
            unit,
            cost: ObservationCost::Resident,
            cadence: ObservationCadence::PerBlock,
            fold: ObservationFold::PeakMagnitude,
            channels: ObservationChannels::PerLane,
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
/// fixture. The shapes are all distinct -- different channel counts and lengths -- so a query that
/// read the wrong row is visible too. The one track points at `mid`, which is neither the first nor
/// the last of the three by either ordering.
fn three_source_session(quantum: u32) -> String {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    let template = model.sources[0].clone();
    let source = |id: &str, channels: u8, frames: u64| {
        let mut value = template.clone();
        value.id = session::StableId::parse(id).expect("stable id");
        value.channels = channels;
        value.frames = frames;
        value
    };
    model.sources = vec![
        source("zeta", 1, u64::from(quantum) * 3),
        source("mid", 2, u64::from(quantum) * 5),
        source("alpha", 4, u64::from(quantum) * 2),
    ];
    model.tracks[0].source_id = session::StableId::parse("mid").expect("stable id");
    canonical_session_json(&model).expect("canonical three-source session")
}

/// Issue #292 IO-9: track discovery and source submission share one staging buffer, so its
/// projection must cover both ID families rather than assuming source IDs are always longest.
///
/// Red mutation: project only `longest_source_id_bytes` again -> boot still publishes a handle,
/// but `miso_engine_web_v1_live_control_track_id` silently returns zero for this valid track and
/// the browser's live-control bind turns that document-dependent condition into `RESULT_INTERNAL`.
#[test]
fn live_control_track_id_longer_than_every_source_id_boots_and_round_trips() {
    const QUANTUM: u32 = 128;
    const SOURCE_ID: &str = "s";
    const TRACK_ID: &str = "track-id-is-longer";

    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.quantum_frames = QUANTUM;
    model.sources[0].id = session::StableId::parse(SOURCE_ID).expect("source ID");
    model.tracks[0].id = session::StableId::parse(TRACK_ID).expect("track ID");
    model.tracks[0].source_id = session::StableId::parse(SOURCE_ID).expect("source reference");
    let session::RouteSource::Track { track_id, .. } = &mut model.routes[0].source else {
        panic!("the identity fixture routes a track");
    };
    *track_id = session::StableId::parse(TRACK_ID).expect("route track ID");
    let document = canonical_session_json(&model).expect("canonical long-track-ID session");

    let handle = crate::ffi::test_boot(document.as_bytes(), boot_options(QUANTUM));
    assert_ne!(
        handle, 0,
        "the valid session boots instead of refusing internally"
    );
    let resources = crate::ffi::test_resources(handle).expect("resource report");
    assert_eq!(resources.id_staging_bytes, TRACK_ID.len() as u64);
    assert!(resources.id_staging_bytes > SOURCE_ID.len() as u64);

    let length = miso_engine_web_v1_live_control_track_id(handle, 0);
    assert_eq!(length, TRACK_ID.len() as u32);
    assert_eq!(
        crate::ffi::test_read_source_id(handle, length).expect("staged track ID"),
        TRACK_ID.as_bytes()
    );
    assert_eq!(miso_engine_web_v1_live_control_track_id(handle, 1), 0);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
}

/// Issue #207 D1: the compiled session answers what sources exist, in canonical order, with the
/// shape a headless driver needs to feed them.
///
/// Red mutation: report declaration order instead of the normalized order -> the assertion below
/// reads `["zeta", "mid", "alpha"]`. Red mutation: report the source ring length instead of the
/// declared full-source frame count -> each row reads 128. Neither survives a fixture whose
/// sources are deliberately unsorted and whose frame counts are distinct.
#[test]
fn session_source_introspection_is_canonical_ordered_shaped_and_bounded() {
    const QUANTUM: u32 = 128;
    let document = three_source_session(QUANTUM);
    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        ..boot_options(QUANTUM)
    };
    let host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");

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
            frames: u64::from(QUANTUM) * 2,
        }
    );
    assert_eq!(
        host.session_source_shape(1).expect("mid"),
        SessionSourceShape {
            channel_count: 2,
            frames: u64::from(QUANTUM) * 5,
        }
    );
    assert_eq!(
        host.session_source_shape(2).expect("zeta"),
        SessionSourceShape {
            channel_count: 1,
            frames: u64::from(QUANTUM) * 3,
        }
    );

    // The track order is unchanged by any of this: the two lists are independent, and the source
    // list is not a filter of the referenced sources either -- `alpha` and `zeta` are declared and
    // therefore reported, though no track reads them.
    assert_eq!(host.live_control_tracks().len(), 1);
    assert_eq!(&*host.live_control_tracks()[0], "track");

    // One past the end, and the u32 ceiling.
    assert_eq!(host.session_source_id(3), None);
    assert_eq!(host.session_source_shape(3), None);
    assert_eq!(host.session_source_id(u32::MAX), None);
    assert_eq!(host.session_source_shape(u32::MAX), None);
}

/// Issue #207 D1: the raw exports, held to the track queries' conventions exactly.
///
/// The shape queries answer zero out of range because zero is impossible for a compiled source --
/// the session validator refuses `channels == 0` and `frames == 0`.
#[test]
fn raw_ffi_source_introspection_mirrors_the_track_queries() {
    const QUANTUM: u32 = 128;
    let document = three_source_session(QUANTUM);
    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        ..boot_options(QUANTUM)
    };
    let handle = crate::ffi::test_boot(document.as_bytes(), options);
    assert_ne!(handle, 0);

    // An invalid handle answers the invalid value on every query, as every other export does.
    for probe in [0, handle.wrapping_add(1)] {
        assert_eq!(miso_engine_web_v1_source_count(probe), 0);
        assert_eq!(miso_engine_web_v1_source_id(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_channels(probe, 0), 0);
        assert_eq!(miso_engine_web_v1_source_frames(probe, 0), 0);
    }

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
    // Out of range: zero everywhere it can be said.
    assert_eq!(miso_engine_web_v1_source_id(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_channels(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_source_frames(handle, 3), 0);
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
    assert_eq!(miso_engine_web_v1_live_control_track_count(handle), 1);

    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_source_count(handle), 0);
}

// ---------------------------------------------------------------------------------------------
// Issue #210 phase 1: solo in place (SIP).
//
// Solo is 100% control plane. Every eval below therefore drives the *shipped* command path and
// reads the *rendered* output, because that is the only place a control-plane composition can be
// wrong in a way anybody hears. The host-side mirror is asserted where the ABI has no other
// witness for it (`LiveControlSoloState`'s own unit tests carry the algebra).
// ---------------------------------------------------------------------------------------------

/// A multi-track identity session: `tracks` copies of the browser fixture's identity strip, each
/// with its own fader gain so no two tracks contribute the same value, all summed at one output.
///
/// Identity end to end means the rendered block is an exact function of which strips are gated:
/// `sum over unmuted tracks of gain(track) * input`. Distinct gains are what make the oracle
/// discriminate -- with equal gains, muting *any* set of the same size would render the same sum
/// and the mute-set oracle would pass without proving anything.
fn solo_session(quantum: u32, tracks: usize, mutes: &[[bool; 2]]) -> String {
    use session::StableId;

    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 64;
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let id = format!("t{index:02}");
        let mut track = track.clone();
        track.id = StableId::parse(&id).expect("track id");
        // -0 dB, -3 dB, -6 dB, ... : distinct per track, and every one inside the declared domain.
        let gain_db = -3.0 * index as f32;
        track.fader.left_db = gain_db;
        track.fader.right_db = gain_db;
        let [left_mute, right_mute] = mutes.get(index).copied().unwrap_or([false, false]);
        track.fader.left_mute = left_mute;
        track.fader.right_mute = right_mute;
        model.tracks.push(track);

        let mut route = route.clone();
        route.id = StableId::parse(&format!("{id}-main")).expect("route id");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the identity fixture routes a track");
        };
        *track_id = StableId::parse(&id).expect("route track id");
        model.routes.push(route);
    }
    canonical_session_json(&model).expect("canonical solo session")
}

/// A live-control host over [`solo_session`]. No meters and no observation capacity: solo touches
/// neither, and a test that bound them would be measuring something else.
fn solo_host(quantum: u32, tracks: usize, mutes: &[[bool; 2]]) -> AudioWorkletEngineHost {
    let document = solo_session(quantum, tracks, mutes);
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: DEFAULT_COMMAND_QUEUE_RECORDS as u64,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("solo boot")
}

/// Stage one `solo` record: `rack`/`channel` are both `255`, the bit rides `values[0]`.
fn stage_solo(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    track: u32,
    on: bool,
    smoothing: u32,
) {
    let value = if on { 1.0 } else { 0.0 };
    stage_command(
        host,
        index,
        COMMAND_SOLO,
        255,
        255,
        track,
        0,
        0,
        smoothing,
        [value, 0.0, 0.0, 0.0],
    );
}

/// Stage one `mute` record addressed to both lanes.
fn stage_mute(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    track: u32,
    on: bool,
    smoothing: u32,
) {
    let value = if on { 1.0 } else { 0.0 };
    stage_command(
        host,
        index,
        COMMAND_MUTE,
        255,
        2,
        track,
        0,
        0,
        smoothing,
        [value, 0.0, 0.0, 0.0],
    );
}

/// Feed the same constant to both arms and require the rendered blocks to be bit-identical.
fn render_pair_and_compare(
    left: &mut AudioWorkletEngineHost,
    right: &mut AudioWorkletEngineHost,
    blocks: u64,
    first_block: u64,
    value: f32,
    what: &str,
) {
    for block in first_block..first_block + blocks {
        feed_and_render(left, 1, block, value);
        feed_and_render(right, 1, block, value);
        let a = left.output_pcm().expect("left output");
        let b = right.output_pcm().expect("right output");
        assert!(
            a.iter()
                .zip(b.iter())
                .all(|(x, y)| x.to_bits() == y.to_bits()),
            "{what}: block {block} differs",
        );
    }
}

/// P1-1: solo `S` is *exactly* explicit mutes on `complement(S)`, bit-identically, from the very
/// first block the acknowledgement names.
///
/// This is the whole architectural claim in one assertion. Solo composes at admission into the
/// same `TrackFaderRecord::Mute` records an explicit mute lowers to, so a host told "solo these
/// four" and a host told "mute the other four" must put the same bytes in the same queues and
/// render the same samples. The two arms drive the same frozen fader section, and neither arm
/// knows which one it is.
///
/// Red mutation: compose `effective_mute` as `user_mute || any_solo` (drop `&& !my_solo`) -> the
/// soloed tracks silence too and block 1 differs.
#[test]
fn solo_is_bit_identically_mute_on_the_complement() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 8;
    for soloed in [
        vec![1_u32, 4],
        vec![0],
        vec![7],
        vec![0, 1, 2, 3, 4, 5, 6, 7],
        vec![2, 3, 5],
    ] {
        let mut solo = solo_host(QUANTUM, TRACKS, &[]);
        let mut mute = solo_host(QUANTUM, TRACKS, &[]);
        render_pair_and_compare(&mut solo, &mut mute, 1, 0, -0.25, "before any command");

        for (index, track) in soloed.iter().enumerate() {
            stage_solo(&mut solo, index, *track, true, QUANTUM);
        }
        assert_eq!(solo.submit_commands(soloed.len() as u32), RESULT_OK);
        let solo_at = solo.command_report().applied_at_sample;

        let complement: Vec<u32> = (0..TRACKS as u32)
            .filter(|track| !soloed.contains(track))
            .collect();
        for (index, track) in complement.iter().enumerate() {
            stage_mute(&mut mute, index, *track, true, QUANTUM);
        }
        assert_eq!(mute.submit_commands(complement.len() as u32), RESULT_OK);
        assert_eq!(mute.command_report().applied_at_sample, solo_at);

        render_pair_and_compare(&mut solo, &mut mute, 4, 1, -0.25, "solo vs explicit mutes");
    }
}

/// P1-2: disengaging solo restores the exact per-lane user-mute set, and the restored host is
/// bit-identical to one that was never soloed.
///
/// The session bakes an *asymmetric* mute (`left_mute` only) on one track and a full mute on
/// another, so the restore has to reproduce per-lane state that one `Mute{lanes, muted}` record
/// cannot carry -- the two-records-per-track case. Both arms then render the same input and must
/// agree sample for sample once the fades have settled.
///
/// Red mutation: restore from `any_solo` alone (re-emit `muted = false` for every track on the
/// disengage) -> the baked mutes come back unmuted and every block after the settle differs.
#[test]
fn un_solo_restores_the_exact_per_lane_user_mute_set() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 6;
    let mutes = [
        [true, false],
        [false, false],
        [true, true],
        [false, true],
        [false, false],
        [false, false],
    ];
    let mut soloed = solo_host(QUANTUM, TRACKS, &mutes);
    let mut never = solo_host(QUANTUM, TRACKS, &mutes);
    render_pair_and_compare(&mut soloed, &mut never, 1, 0, -0.25, "before any command");

    stage_solo(&mut soloed, 0, 1, true, QUANTUM);
    assert_eq!(soloed.submit_commands(1), RESULT_OK);
    for block in 1..4 {
        feed_and_render(&mut soloed, 1, block, -0.25);
        feed_and_render(&mut never, 1, block, -0.25);
    }
    stage_solo(&mut soloed, 0, 1, false, QUANTUM);
    assert_eq!(soloed.submit_commands(1), RESULT_OK);
    // Block 4 carries the disengage fade; from block 5 every ramp has settled.
    feed_and_render(&mut soloed, 1, 4, -0.25);
    feed_and_render(&mut never, 1, 4, -0.25);
    render_pair_and_compare(
        &mut soloed,
        &mut never,
        4,
        5,
        -0.25,
        "restored vs never soloed",
    );

    // And the host mirror agrees with the session it was prepared from.
    let state = soloed.live_control_solo().expect("solo state");
    assert!(!state.any_solo());
    for (track, expected) in mutes.iter().enumerate() {
        assert_eq!(
            [state.user_mute(track, 0), state.user_mute(track, 1)],
            *expected,
            "track {track} user mute",
        );
        assert_eq!(
            [state.emitted_mute(track, 0), state.emitted_mute(track, 1)],
            *expected,
            "track {track} emitted mute",
        );
    }
}

/// P1-3: a solo gate is the same declicked fader endpoint a mute is -- a linear ramp bounded by
/// the D11 law, and an exact snap when the window is zero.
///
/// Two tracks, both at unity, both identity: the rendered left plane is `input * (1 + gate)`
/// where `gate` walks from 1 to 0 over the window. So the per-sample delta is bounded by
/// `|input| / window` (the one division D11 permits, taken at the event), the walk is monotone,
/// and the settled block is the soloed track alone -- exactly, including the sign of the zero the
/// gated track contributes.
#[test]
fn a_solo_gate_is_a_bounded_ramp_and_a_zero_window_snaps() {
    const QUANTUM: u32 = 128;
    const INPUT: f32 = -0.5;
    let quantum = QUANTUM as usize;

    let mut host = solo_host(QUANTUM, 2, &[]);
    feed_and_render(&mut host, 1, 0, INPUT);
    let settled = host.output_pcm().expect("output")[0];

    stage_solo(&mut host, 0, 0, true, QUANTUM);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 1, INPUT);
    let fade = host.output_pcm().expect("output").to_vec();

    // Track 0 alone at unity is exactly the input; the pair is louder than that.
    assert!(settled < INPUT, "two tracks sum below one: {settled}");
    let bound = (settled - INPUT).abs() / QUANTUM as f32;
    for index in 1..quantum {
        assert!(
            fade[index] >= fade[index - 1],
            "the gate walks monotonically toward silence at {index}: {} then {}",
            fade[index - 1],
            fade[index],
        );
        let step = (fade[index] - fade[index - 1]).abs();
        // The bound is the ramp law's own increment, recomputed here in a different order (the
        // block's endpoints rather than the event's `(target - current) / n`), so it is compared
        // with a rounding allowance and not exactly. Anything a discontinuity would produce is
        // orders of magnitude outside it.
        assert!(
            step <= bound * 1.000_1,
            "sample {index} moves {step}, past the {bound} the D11 ramp law allows",
        );
    }
    assert_eq!(
        fade[quantum - 1].to_bits(),
        INPUT.to_bits(),
        "the settled gate leaves the soloed track exactly, with no negative zero beside it",
    );

    // A zero window snaps on the first sample of the block it is acknowledged at.
    stage_solo(&mut host, 0, 0, false, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 2, INPUT);
    let snapped = host.output_pcm().expect("output").to_vec();
    assert!(
        snapped[..quantum]
            .iter()
            .all(|value| value.to_bits() == settled.to_bits()),
        "a zero-window un-solo restores the prepared console exactly, from sample zero",
    );
}

/// P1-4: user mute and solo are separate states. Neither gesture overwrites the other, in the
/// rendered output and in the host mirror the ABI has no readback for.
///
/// Red mutation: have `set_solo` clear `user_mute` for the soloed track -> the mute-while-soloed
/// leg still silences, but the un-solo brings the muted track back and the last assertion fails.
#[test]
fn mute_and_solo_are_separate_states() {
    const QUANTUM: u32 = 128;
    const INPUT: f32 = -0.5;
    let mut host = solo_host(QUANTUM, 2, &[]);
    feed_and_render(&mut host, 1, 0, INPUT);
    let both = host.output_pcm().expect("output")[0];

    // Solo track 0. A zero window keeps every assertion exact.
    stage_solo(&mut host, 0, 0, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 1, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        INPUT.to_bits()
    );

    // Muting the soloed track silences it: solo is not immunity.
    stage_mute(&mut host, 0, 0, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 2, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        0.0_f32.to_bits(),
        "a muted soloed track is exact positive zero, not a negative zero",
    );
    {
        let state = host.live_control_solo().expect("solo state");
        assert!(state.solo(0) && !state.solo(1));
        assert_eq!([state.user_mute(0, 0), state.user_mute(0, 1)], [true, true]);
        assert_eq!(
            [state.user_mute(1, 0), state.user_mute(1, 1)],
            [false, false]
        );
        assert!(state.effective_mute(1, 0), "the un-soloed track is gated");
    }

    // Re-engaging a solo that is already engaged is idempotent, and idempotent all the way down:
    // it must not disturb the user mute the soloed track is carrying.
    stage_solo(&mut host, 0, 0, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 3, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        0.0_f32.to_bits(),
        "a repeated solo engage un-muted the track it re-engaged",
    );
    {
        let state = host.live_control_solo().expect("solo state");
        assert_eq!(
            [state.user_mute(0, 0), state.user_mute(0, 1)],
            [true, true],
            "a repeated solo engage overwrote the user mute",
        );
    }

    // Mute the *other* track too, while it is already gated by solo. That is user intent and it
    // has to outlive the solo.
    stage_mute(&mut host, 0, 1, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 4, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        0.0_f32.to_bits()
    );

    // Unmute track 0 while it is still soloed: it comes back, alone.
    stage_mute(&mut host, 0, 0, false, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 5, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        INPUT.to_bits()
    );

    // Clearing solo restores exactly the mutes the user set under it -- track 1 stays muted.
    stage_solo(&mut host, 0, 0, false, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 6, INPUT);
    assert_eq!(
        host.output_pcm().expect("output")[0].to_bits(),
        INPUT.to_bits(),
        "track 1 was muted while soloed away and is still muted now",
    );
    assert_ne!(both.to_bits(), INPUT.to_bits());
    let state = host.live_control_solo().expect("solo state");
    assert!(!state.any_solo());
    assert_eq!(
        [state.user_mute(0, 0), state.user_mute(0, 1)],
        [false, false]
    );
    assert_eq!([state.user_mute(1, 0), state.user_mute(1, 1)], [true, true]);
}

/// P1-5: a solo submission refused for backpressure applies *nothing* -- not to a queue, and not
/// to the host's own solo state.
///
/// This is the correction the adversarial verification named as the likeliest implementation bug:
/// pass one mutates solo state while the submission is still being validated, so a pass-two
/// refusal has to leave that state exactly as it was. The proof is a third host that never saw the
/// refused batch: after the refusal, the refused host and the untouched host must render the same
/// samples forever.
///
/// Red mutation: drop the `ready.solo.rollback()` on the refusal path -> the refused engage sticks
/// in host state, the next admitted mute composes against it, and the comparison diverges.
#[test]
fn a_refused_solo_submission_leaves_the_live_controls_untouched() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;
    const DEPTH: u32 = DEFAULT_COMMAND_QUEUE_RECORDS;
    let mut refused = solo_host(QUANTUM, TRACKS, &[]);
    let mut untouched = solo_host(QUANTUM, TRACKS, &[]);
    render_pair_and_compare(
        &mut refused,
        &mut untouched,
        1,
        0,
        -0.25,
        "before any command",
    );

    // Fill track 0's fader queue exactly, without rendering: nothing drains until the next block.
    for _ in 0..DEPTH {
        stage_command(
            &mut refused,
            0,
            COMMAND_FADER_DB,
            255,
            2,
            0,
            0,
            0,
            0,
            [-1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(refused.submit_commands(1), RESULT_OK);
        stage_command(
            &mut untouched,
            0,
            COMMAND_FADER_DB,
            255,
            2,
            0,
            0,
            0,
            0,
            [-1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(untouched.submit_commands(1), RESULT_OK);
    }

    // Soloing track 1 owes track 0 one gate record, and track 0 has no room for it.
    stage_solo(&mut refused, 0, 1, true, QUANTUM);
    assert_eq!(refused.submit_commands(1), RESULT_BACKPRESSURE);
    assert_eq!(refused.command_report().reason, COMMAND_REASON_BACKPRESSURE,);
    assert_eq!(refused.command_report().admitted, 0);
    {
        let state = refused.live_control_solo().expect("solo state");
        assert!(!state.any_solo(), "a refused engage left a solo bit set");
        assert_eq!(state.solo_count(), 0);
        assert!(!state.transaction_open(), "the transaction was left open");
        for track in 0..TRACKS {
            assert!(!state.solo(track));
            assert!(!state.user_mute(track, 0) && !state.user_mute(track, 1));
            assert!(!state.emitted_mute(track, 0) && !state.emitted_mute(track, 1));
        }
    }

    // The refused host is indistinguishable from one that never saw the batch, for good.
    render_pair_and_compare(
        &mut refused,
        &mut untouched,
        3,
        1,
        -0.25,
        "after the refusal",
    );
    stage_solo(&mut refused, 0, 1, true, QUANTUM);
    assert_eq!(refused.submit_commands(1), RESULT_OK);
    stage_solo(&mut untouched, 0, 1, true, QUANTUM);
    assert_eq!(untouched.submit_commands(1), RESULT_OK);
    render_pair_and_compare(&mut refused, &mut untouched, 3, 4, -0.25, "after the retry");
}

/// A malformed or out-of-domain solo record is typed exactly as the mute record it mirrors, and
/// still applies nothing.
#[test]
fn solo_records_are_shape_checked_like_mute_records() {
    const QUANTUM: u32 = 128;
    let mut host = solo_host(QUANTUM, 3, &[]);
    feed_and_render(&mut host, 1, 0, -0.25);

    let cases: [(u8, u8, [f32; 4], u32, u32, u32); 6] = [
        // (rack, channel, values, track, expected result, expected reason)
        (
            0,
            255,
            [1.0, 0.0, 0.0, 0.0],
            0,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            255,
            2,
            [1.0, 0.0, 0.0, 0.0],
            0,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            255,
            255,
            [1.0, 1.0, 0.0, 0.0],
            0,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            255,
            255,
            [0.5, 0.0, 0.0, 0.0],
            0,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            255,
            255,
            [-1.0, 0.0, 0.0, 0.0],
            0,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            255,
            255,
            [1.0, 0.0, 0.0, 0.0],
            3,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_TRACK,
        ),
    ];
    for (rack, channel, values, track, result, reason) in cases {
        stage_command(
            &mut host,
            0,
            COMMAND_SOLO,
            rack,
            channel,
            track,
            0,
            0,
            QUANTUM,
            values,
        );
        assert_eq!(host.submit_commands(1), result, "{values:?}");
        assert_eq!(host.command_report().reason, reason, "{values:?}");
        let state = host.live_control_solo().expect("solo state");
        assert!(
            !state.any_solo(),
            "a refused solo record engaged a bit anyway"
        );
        assert!(!state.transaction_open());
    }
}

/// The no-redundant-record rule, pinned red.
///
/// A solo gesture that changes no lane's effective mute must put *nothing* in a queue. That is not
/// an optimisation: the fader stage's `set_mute` retargets unconditionally, so re-muting an
/// already-settled-muted lane with a nonzero window re-enters the ramp kernel, which multiplies by
/// the current gain instead of filling the plane -- and `gain * negative` is `-0.0` where the
/// settled path gives exact `+0.0`. Digest visible, in the one place the browser fixture's own
/// oracle looks.
///
/// A one-track console is the cleanest witness: soloing the only track changes nothing at all, so
/// the muted plane must stay bit-for-bit `+0.0` across an engage and a disengage.
///
/// Red mutation: emit a record for every track on a solo transition instead of only for the lanes
/// whose effective mute changed -> the muted lane re-enters the ramp and the plane reads `-0.0`.
#[test]
fn a_solo_that_changes_nothing_emits_nothing() {
    const QUANTUM: u32 = 128;
    let mut host = live_control_host(QUANTUM, 0);
    feed_and_render(&mut host, 1, 0, -0.5);
    stage_mute(&mut host, 0, 0, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 1, -0.5);
    assert!(
        host.output_pcm()
            .expect("output")
            .iter()
            .all(|value| value.to_bits() == 0.0_f32.to_bits()),
        "the zero-window mute settled to exact positive zero",
    );

    for (block, on) in [(2_u64, true), (3, false), (4, true)] {
        stage_solo(&mut host, 0, 0, on, QUANTUM);
        assert_eq!(host.submit_commands(1), RESULT_OK);
        feed_and_render(&mut host, 1, block, -0.5);
        let out = host.output_pcm().expect("output");
        assert!(
            out.iter().all(|value| value.to_bits() == 0.0_f32.to_bits()),
            "block {block}: a solo that changes no effective mute re-entered the ramp path",
        );
    }
    let state = host.live_control_solo().expect("solo state");
    assert!(state.any_solo());
    assert!(state.emitted_mute(0, 0) && state.emitted_mute(0, 1));
}

/// The batch-coalescing rule, pinned red.
///
/// A full 256-record batch of alternating solo toggles is one gesture. Applied per command it
/// would fan out up to `2 * track_count` records *per transition* -- 256 transitions would
/// overflow the decode staging and flood every per-track queue. Applied as the design requires --
/// all state changes first, then one net emission -- the batch costs at most two records per
/// track and lands exactly where the two-record batch with the same net effect lands.
///
/// Red mutation: emit the net delta inside the per-record loop instead of after it -> the batch is
/// refused with `backpressure` (or trips the staging bound) and the comparison never runs.
#[test]
fn a_batch_of_alternating_solo_toggles_coalesces_to_its_net_effect() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 8;
    let mut batched = solo_host(QUANTUM, TRACKS, &[]);
    let mut net = solo_host(QUANTUM, TRACKS, &[]);
    render_pair_and_compare(&mut batched, &mut net, 1, 0, -0.25, "before any command");

    let records = MAXIMUM_COMMAND_RECORDS as usize;
    for index in 0..records - 1 {
        stage_solo(&mut batched, index, 0, index % 2 == 0, QUANTUM);
    }
    // 255 alternating toggles leave track 0 engaged (index 254 is even); the last record engages
    // track 5. The net effect is exactly the two-record batch the other arm submits.
    stage_solo(&mut batched, records - 1, 5, true, QUANTUM);
    assert_eq!(
        batched.submit_commands(MAXIMUM_COMMAND_RECORDS),
        RESULT_OK,
        "reason {}",
        batched.command_report().reason,
    );

    stage_solo(&mut net, 0, 0, true, QUANTUM);
    stage_solo(&mut net, 1, 5, true, QUANTUM);
    assert_eq!(net.submit_commands(2), RESULT_OK);

    let state = batched.live_control_solo().expect("solo state");
    assert_eq!(state.solo_count(), 2);
    assert!(state.solo(0) && state.solo(5));
    render_pair_and_compare(&mut batched, &mut net, 4, 1, -0.25, "coalesced vs net");
}

/// The class-A OFF gate, in the one form a unit test can carry: with no solo command ever
/// admitted, a mute gesture puts byte-for-byte what it always put on the wire -- including the
/// wart.
///
/// This has to be an **absolute** oracle, not a two-host comparison: a mutation of the mute path
/// would move both arms of a comparison identically and escape it. So it pins the exact bits
/// today's engine renders for the one gesture a net-emission rule would be tempted to collapse --
/// a *redundant* re-mute of an already-settled-muted lane, with a nonzero window, on a negative
/// input. The fader stage retargets unconditionally, so that gesture re-enters the ramp kernel
/// (which multiplies by the current gain) instead of the settled kernel (which fills the plane),
/// and `gain * negative` is `-0.0` for every sample but the one the ramp assigns its target on.
/// That is what ships today, and solo does not get to quietly improve it: "improving" it is a
/// digest change on a path no solo command touched.
///
/// The rest of the OFF gate is the sweep and the wasm legs, which compare digests across builds.
///
/// Red mutation: route mute through the coalesced net emission (stage nothing per command, let the
/// delta pass decide) -> the redundant re-mute stages nothing, the plane stays `+0.0`, and this
/// fails at sample zero.
#[test]
fn live_controls_that_never_solo_render_what_they_always_did() {
    const QUANTUM: u32 = 128;
    let quantum = QUANTUM as usize;
    let mut host = live_control_host(QUANTUM, 0);
    feed_and_render(&mut host, 1, 0, -0.5);

    stage_mute(&mut host, 0, 0, true, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 1, -0.5);
    assert!(
        host.output_pcm()
            .expect("output")
            .iter()
            .all(|value| value.to_bits() == 0.0_f32.to_bits()),
        "a zero-window mute settles to exact positive zero",
    );

    // The redundant re-mute, with a window. Today: the ramp path, and `-0.0` until it settles.
    stage_mute(&mut host, 0, 0, true, QUANTUM);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    feed_and_render(&mut host, 1, 2, -0.5);
    let block = host.output_pcm().expect("output").to_vec();
    for (index, value) in block[..quantum - 1].iter().enumerate() {
        assert_eq!(
            value.to_bits(),
            (-0.0_f32).to_bits(),
            "sample {index}: the pre-solo mute path is not the one that ran",
        );
    }
    assert_eq!(
        block[quantum - 1].to_bits(),
        0.0_f32.to_bits(),
        "the ramp assigns its target exactly on the frame it settles on",
    );

    let state = host.live_control_solo().expect("solo state");
    assert!(!state.any_solo());
    assert_eq!(state.solo_count(), 0);
    assert!(state.user_mute(0, 0) && state.user_mute(0, 1));
    assert!(state.emitted_mute(0, 0) && state.emitted_mute(0, 1));
}

/// A console whose every track carries the command fixture's parametric EQ.
///
/// The one batch shape that can exhaust the decode staging: `channel = both` effect-parameter
/// records lower to two spans each, and a solo record in the same batch owes a gate record to
/// every track it silences.
fn effect_solo_host(quantum: u32, tracks: usize, depth: u64) -> AudioWorkletEngineHost {
    use session::StableId;

    // This staging-capacity fixture deliberately uses an ordinary compressor. EQ's live path
    // coalesces through prepared targets, while this effect keeps the historical 510-span wire
    // bound exercised by an ordinary per-lane effect.
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/compressor-dynamic-observation.json"
    ))
    .expect("accepted compressor fixture");
    model.quantum_frames = quantum;
    model.sources[0].frames = u64::from(quantum) * 64;
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let id = format!("t{index:02}");
        let mut track = track.clone();
        track.id = StableId::parse(&id).expect("track id");
        model.tracks.push(track);
        let mut route = route.clone();
        route.id = StableId::parse(&format!("{id}-main")).expect("route id");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the command fixture routes a track");
        };
        *track_id = StableId::parse(&id).expect("route track id");
        model.routes.push(route);
    }
    let document = canonical_session_json(&model).expect("canonical effect solo session");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_command_queue_records: depth,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("effect solo boot")
}

/// The decode staging is sized for the worst batch the ABI can describe, and that batch is
/// admitted rather than refused.
///
/// `2 * MAXIMUM_COMMAND_RECORDS` was the whole bound before solo: one wire record lowers to at
/// most two spans. Solo adds a term the wire does not bound at all -- the gate records a solo
/// transition owes every track it silences -- so the two terms **add**. A batch of 255
/// `channel = both` effect-parameter records (510 spans) plus one solo record on a four-track
/// console needs 513 entries, and the pre-solo array held 512.
///
/// Red mutation: size the array `2 * MAXIMUM_COMMAND_RECORDS` again -> the array is one entry
/// short, the batch is refused `malformed` by the staging bound, and the length pin below is red
/// on its own.
#[test]
fn the_decode_staging_holds_a_full_batch_plus_a_solo_transition() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;
    let mut host = effect_solo_host(QUANTUM, TRACKS, DEFAULT_COMMAND_QUEUE_RECORDS as u64 * 4);
    assert_eq!(
        host.command_staging_entries(),
        Some(MAXIMUM_COMMAND_RECORDS as usize * 2 + TRACKS * 2),
        "the decode staging is `2 * MAXIMUM_COMMAND_RECORDS + 2 * track_count`",
    );
    feed_and_render(&mut host, 1, 0, 0.25);

    let records = MAXIMUM_COMMAND_RECORDS as usize;
    for index in 0..records - 1 {
        // Compressor threshold on each track, addressed to both lanes: two spans per wire record.
        stage_command(
            &mut host,
            index,
            COMMAND_EFFECT_PARAM,
            1,
            2,
            (index % TRACKS) as u32,
            0,
            1,
            0,
            [-12.0, 0.0, 0.0, 0.0],
        );
    }
    stage_solo(&mut host, records - 1, 0, true, QUANTUM);
    assert_eq!(
        host.submit_commands(MAXIMUM_COMMAND_RECORDS),
        RESULT_OK,
        "reason {}",
        host.command_report().reason,
    );
    assert_eq!(host.command_report().admitted, MAXIMUM_COMMAND_RECORDS);
    let state = host.live_control_solo().expect("solo state");
    assert!(state.solo(0));
    for track in 1..TRACKS {
        assert!(
            state.emitted_mute(track, 0) && state.emitted_mute(track, 1),
            "track {track} was told about the gate",
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Issue #210 phase 3: command kinds 10 (`trimDb`) and 11 (`polarityInvert`).
// ---------------------------------------------------------------------------------------------

/// One row of the trim/polarity refusal matrix:
/// `(kind, rack, channel, track, values, expected result, expected reason)`.
type TrimRefusalCase = (u32, u8, u8, u32, [f32; 4], u32, u32);

/// Stage one `trimDb` record. `rack` is `255`, the dB rides `values[0]`, the lane is `channel`.
fn stage_trim(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    track: u32,
    channel: u8,
    db: f32,
    smoothing: u32,
) {
    stage_command(
        host,
        index,
        COMMAND_TRIM_DB,
        255,
        channel,
        track,
        0,
        0,
        smoothing,
        [db, 0.0, 0.0, 0.0],
    );
}

/// Stage one `polarityInvert` record. `rack` is `255`, the bit rides `values[0]`.
fn stage_polarity(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    track: u32,
    channel: u8,
    inverted: bool,
    smoothing: u32,
) {
    let value = if inverted { 1.0 } else { 0.0 };
    stage_command(
        host,
        index,
        COMMAND_POLARITY_INVERT,
        255,
        channel,
        track,
        0,
        0,
        smoothing,
        [value, 0.0, 0.0, 0.0],
    );
}

/// The two kinds are admitted, on every lane selector, at every window the fader accepts.
///
/// Red mutation: drop `COMMAND_TRIM_DB` from `CommandRecord::decode`'s whitelist -> every arm
/// refuses `malformed`. Red mutation: drop the two kinds from `admit_commands_staged`'s per-track
/// arm -> they fall to the `_ =>` arm and refuse `malformed` with a *decoded* record, which is the
/// drift the kind-vocabulary gate cannot see because the constant still exists.
#[test]
fn trim_and_polarity_are_admitted_on_every_lane_selector() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;
    for channel in [0_u8, 1, 2] {
        for smoothing in [0_u32, 1, QUANTUM, u32::MAX] {
            let mut host = solo_host(QUANTUM, TRACKS, &[]);
            stage_trim(&mut host, 0, 2, channel, -18.0, smoothing);
            stage_polarity(&mut host, 1, 2, channel, true, smoothing);
            assert_eq!(
                host.submit_commands(2),
                RESULT_OK,
                "channel={channel} smoothing={smoothing} reason={}",
                host.command_report().reason
            );
            assert_eq!(host.command_report().admitted, 2);
            assert_eq!(host.command_report().reason, COMMAND_REASON_NONE);
        }
    }
}

/// The refusal matrix: shape, domain and address, each with the reason the ABI declares.
///
/// The ordering rule is the one every kind follows and the one a new kind is most likely to get
/// wrong: **track bound first** (`unknownTrack`), **then shape and domain** (`malformed`,
/// `domain`), **then "this session has no such queue"** (`unsupportedKind`). A record that is both
/// badly shaped and addressed at a session without live controls reports `malformed`, not
/// `unsupported`.
#[test]
fn trim_and_polarity_refuse_on_the_declared_terms() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;
    let cases: [TrimRefusalCase; 14] = [
        // A rack byte on a builtin-addressed kind is a shape error.
        (
            COMMAND_TRIM_DB,
            0,
            2,
            0,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            COMMAND_POLARITY_INVERT,
            2,
            2,
            0,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        // `255` is not a lane: these kinds address a lane, unlike `solo`.
        (
            COMMAND_TRIM_DB,
            255,
            255,
            0,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            COMMAND_POLARITY_INVERT,
            255,
            3,
            0,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        // Every value word past the first must be zero.
        (
            COMMAND_TRIM_DB,
            255,
            2,
            0,
            [0.0, 1.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        (
            COMMAND_POLARITY_INVERT,
            255,
            2,
            0,
            [0.0, 0.0, 0.0, 1.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_MALFORMED,
        ),
        // `trim_db`'s declared domain is `[-144, 24]`, exactly `fader_db`'s.
        (
            COMMAND_TRIM_DB,
            255,
            2,
            0,
            [-144.001, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            COMMAND_TRIM_DB,
            255,
            2,
            0,
            [24.001, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        // The endpoints themselves are inside it.
        (
            COMMAND_TRIM_DB,
            255,
            2,
            0,
            [-144.0, 0.0, 0.0, 0.0],
            RESULT_OK,
            COMMAND_REASON_NONE,
        ),
        (
            COMMAND_TRIM_DB,
            255,
            2,
            0,
            [24.0, 0.0, 0.0, 0.0],
            RESULT_OK,
            COMMAND_REASON_NONE,
        ),
        // `polarity_invert` is boolean-exact.
        (
            COMMAND_POLARITY_INVERT,
            255,
            2,
            0,
            [0.5, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        (
            COMMAND_POLARITY_INVERT,
            255,
            2,
            0,
            [-1.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_DOMAIN,
        ),
        // The track bound is checked before anything else about the record.
        (
            COMMAND_TRIM_DB,
            255,
            2,
            TRACKS as u32,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_TRACK,
        ),
        (
            COMMAND_POLARITY_INVERT,
            0,
            2,
            TRACKS as u32,
            [0.0, 0.0, 0.0, 0.0],
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_TRACK,
        ),
    ];
    for (kind, rack, channel, track, values, result, reason) in cases {
        let mut host = solo_host(QUANTUM, TRACKS, &[]);
        stage_command(
            &mut host, 0, kind, rack, channel, track, 0, 0, QUANTUM, values,
        );
        assert_eq!(
            host.submit_commands(1),
            result,
            "kind={kind} rack={rack} channel={channel} track={track} values={values:?}"
        );
        assert_eq!(
            host.command_report().reason,
            reason,
            "kind={kind} rack={rack} channel={channel} track={track} values={values:?}"
        );
    }
}

/// A `channel = both` command is **one** record and takes **one** queue slot.
///
/// The departure from the effect-parameter lowering, asserted where it is observable: an
/// `effectParam` on a `PerLane` parameter lowers to two records and takes two slots, while a
/// `trimDb` addressed at both lanes lowers to one carrying `BuiltinLaneSelector::Both`. The reason
/// is the channel-symmetry witness -- two per-lane records present as two `Desymmetrize` events and
/// would retire the track's mono collapse on a command that moves both channels identically -- and
/// the queue arithmetic is where the control plane shows which one it did.
///
/// The queue depth is the live-control default; filling it with `depth` both-commands must be
/// admitted, and one more must be `backpressure`. A two-record lowering would refuse at half that
/// count.
///
/// Red mutation: lower `channel = 2` to two per-lane records the way `into_effect_records` does ->
/// the queue fills at half the depth and the `admitted` count below doubles.
#[test]
fn a_both_lane_trim_command_is_one_record_and_one_queue_slot() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 2;
    // The live-control default queue depth, which is the bound the room pre-check enforces.
    let depth = DEFAULT_COMMAND_QUEUE_RECORDS as usize;
    assert!(depth >= 2, "the live-control default depth is meaningful");

    // Exactly `depth` both-lane commands fit.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    for index in 0..depth {
        stage_trim(&mut host, index, 0, 2, -6.0, QUANTUM);
    }
    assert_eq!(
        host.submit_commands(depth as u32),
        RESULT_OK,
        "reason {}",
        host.command_report().reason
    );
    assert_eq!(
        host.command_report().admitted,
        depth as u32,
        "one wire record lowered to one queue record"
    );

    // And one more does not, because the queue is full rather than because the batch is.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    for index in 0..depth + 1 {
        stage_trim(&mut host, index, 0, 2, -6.0, QUANTUM);
    }
    assert_eq!(host.submit_commands(depth as u32 + 1), RESULT_BACKPRESSURE);
    assert_eq!(host.command_report().reason, COMMAND_REASON_BACKPRESSURE);

    // The other half of "one record": that record addresses **both** lanes. A lowering that
    // emitted one record carrying a single lane would satisfy the arithmetic above and render the
    // wrong audio, so the two halves are asserted together.
    //
    // Red mutation: lower `channel = 2` as `BuiltinLaneSelector::Left` -> the `both` arm renders
    // the `left` arm's bits and this fails.
    let mut both = solo_host(QUANTUM, TRACKS, &[]);
    let mut left_only = solo_host(QUANTUM, TRACKS, &[]);
    let mut right_only = solo_host(QUANTUM, TRACKS, &[]);
    for (host, channel) in [(&mut both, 2_u8), (&mut left_only, 0), (&mut right_only, 1)] {
        stage_trim(host, 0, 0, channel, -144.0, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK);
        feed_and_render(host, 1, 0, -0.25);
    }
    let both_bits: Vec<u32> = both
        .output_pcm()
        .expect("output")
        .iter()
        .map(|value| value.to_bits())
        .collect();
    let left_bits: Vec<u32> = left_only
        .output_pcm()
        .expect("output")
        .iter()
        .map(|value| value.to_bits())
        .collect();
    let right_bits: Vec<u32> = right_only
        .output_pcm()
        .expect("output")
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert_ne!(
        both_bits, left_bits,
        "a `channel = both` trim is not a left-lane trim"
    );
    assert_ne!(
        both_bits, right_bits,
        "a `channel = both` trim is not a right-lane trim"
    );
    assert_ne!(left_bits, right_bits, "the two lanes are distinguishable");
}

/// The input queue is its own destination: filling it does not refuse a fader or matrix command,
/// and filling the fader queue does not refuse a trim command.
///
/// The band the phase added to the frozen slot layout, asserted as a band rather than as
/// arithmetic. A slot collision -- an input command counted against the fader queue's room -- would
/// show here as a `backpressure` on a queue with room.
///
/// **One mutation is deliberately not red here**, and it is worth naming rather than leaving for a
/// reader to find: making `ReadyOwnership::queue_capacity` return `producer.fader.capacity()` for
/// an input slot changes nothing observable, because live controls lease all three of a track's
/// queues at **one** depth -- `TrackControlRequest::queue_capacity` is a single field, and
/// `prepare_session_builtins_with_live_controls` builds the three rings from it. The wrong queue's
/// capacity is the right number. It becomes observable the day the three depths can differ, and
/// the line is written per band anyway so that day is a one-line change rather than a bug.
#[test]
fn the_input_queue_is_a_destination_of_its_own() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 2;
    let depth = DEFAULT_COMMAND_QUEUE_RECORDS as usize;

    // Fill the input queue, then move the fader on the same track.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    for index in 0..depth {
        stage_trim(&mut host, index, 1, 2, -6.0, QUANTUM);
    }
    assert_eq!(host.submit_commands(depth as u32), RESULT_OK);
    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        1,
        0,
        0,
        QUANTUM,
        [-3.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(
        host.submit_commands(1),
        RESULT_OK,
        "a full input queue must not refuse a fader command: reason {}",
        host.command_report().reason
    );

    // And the reverse.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    for index in 0..depth {
        stage_command(
            &mut host,
            index,
            COMMAND_FADER_DB,
            255,
            2,
            1,
            0,
            0,
            QUANTUM,
            [-3.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(host.submit_commands(depth as u32), RESULT_OK);
    stage_trim(&mut host, 0, 1, 2, -6.0, QUANTUM);
    assert_eq!(
        host.submit_commands(1),
        RESULT_OK,
        "a full fader queue must not refuse a trim command: reason {}",
        host.command_report().reason
    );
}

/// A submission is all-or-nothing across the new band too: a batch whose last record is refused
/// pushes none of the earlier ones.
///
/// The three-pass contract, applied to the kind that added a queue. Red mutation: push the input
/// band inside pass one rather than pass three -> the trim below lands and the render moves.
#[test]
fn a_refused_batch_pushes_no_trim_record() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 2;
    let mut refused = solo_host(QUANTUM, TRACKS, &[]);
    let mut untouched = solo_host(QUANTUM, TRACKS, &[]);
    render_pair_and_compare(
        &mut refused,
        &mut untouched,
        1,
        0,
        -0.25,
        "before any command",
    );

    stage_trim(&mut refused, 0, 0, 2, -144.0, 0);
    // A second record the ABI refuses: a polarity value that is neither `0.0` nor `1.0`.
    stage_polarity(&mut refused, 1, 0, 2, false, 0);
    stage_command(
        &mut refused,
        1,
        COMMAND_POLARITY_INVERT,
        255,
        2,
        0,
        0,
        0,
        0,
        [0.25, 0.0, 0.0, 0.0],
    );
    assert_eq!(refused.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_eq!(refused.command_report().reason, COMMAND_REASON_DOMAIN);
    assert_eq!(refused.command_report().rejected_index, 1);
    render_pair_and_compare(
        &mut refused,
        &mut untouched,
        2,
        1,
        -0.25,
        "a refused batch pushed the trim record anyway",
    );
}

/// A trim command reaches the render plane and silences the track it names, at the block the
/// acknowledgement names.
///
/// The end-to-end wire-to-plane assertion for the new band. `-144 dB` is a factor of `6.3e-8`, so
/// the addressed track's contribution is below any nonzero threshold while the untouched arm's is
/// not.
#[test]
fn a_trim_command_reaches_the_render_plane() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 2;
    let mut trimmed = solo_host(QUANTUM, TRACKS, &[]);
    let mut untouched = solo_host(QUANTUM, TRACKS, &[]);
    render_pair_and_compare(
        &mut trimmed,
        &mut untouched,
        1,
        0,
        -0.25,
        "before any command",
    );

    stage_trim(&mut trimmed, 0, 0, 2, -144.0, 0);
    assert_eq!(trimmed.submit_commands(1), RESULT_OK);

    feed_and_render(&mut trimmed, 1, 1, -0.25);
    feed_and_render(&mut untouched, 1, 1, -0.25);
    let moved = trimmed.output_pcm().expect("output").to_vec();
    let still = untouched.output_pcm().expect("output").to_vec();
    assert!(
        moved
            .iter()
            .zip(still.iter())
            .any(|(a, b)| a.to_bits() != b.to_bits()),
        "a -144 dB trim on one of two tracks must move the mix"
    );

    // And a polarity flip on the same track moves it again, in the opposite direction.
    stage_polarity(&mut trimmed, 0, 1, 2, true, 0);
    assert_eq!(trimmed.submit_commands(1), RESULT_OK);
    feed_and_render(&mut trimmed, 1, 2, -0.25);
    feed_and_render(&mut untouched, 1, 2, -0.25);
    let flipped = trimmed.output_pcm().expect("output").to_vec();
    let reference = untouched.output_pcm().expect("output").to_vec();
    assert!(
        flipped
            .iter()
            .zip(reference.iter())
            .any(|(a, b)| a.to_bits() != b.to_bits()),
        "a polarity flip on the untrimmed track must move the mix"
    );
}

/// Kinds 10/11 share the transactional admission with kind 9 and must not disturb it.
///
/// The solo state is mutated inside pass one, before the submission is known to be admissible, and
/// the wrapper closes it: `commit` once pass three has pushed, `rollback` on every refusal. Two
/// kinds that reach the same batch cannot be allowed to leave that half-open.
///
/// Red mutation: refuse a trim record with an early `return Err(...)` from
/// `admit_commands_staged` *after* it has staged, bypassing the `rollback` in `admit_commands` ->
/// the solo bit survives a refused batch and the last assertion fails.
#[test]
fn trim_and_polarity_leave_the_solo_transaction_closed() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;

    // A batch that mixes solo with the two new kinds is admitted whole and closes clean.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    stage_trim(&mut host, 0, 1, 2, -9.0, QUANTUM);
    stage_solo(&mut host, 1, 2, true, QUANTUM);
    stage_polarity(&mut host, 2, 3, 0, true, QUANTUM);
    assert_eq!(
        host.submit_commands(3),
        RESULT_OK,
        "reason {}",
        host.command_report().reason
    );
    let state = host.live_control_solo().expect("solo state");
    assert!(state.solo(2), "the solo bit moved");
    assert!(!state.transaction_open(), "and the transaction closed");
    // `emitted >= user_mute` is the standing invariant: every track the gate silenced was told so,
    // and no track was told it is muted while its user mute says otherwise for a lane the gate
    // does not cover.
    for track in 0..TRACKS {
        for lane in 0..2 {
            assert!(
                state.emitted_mute(track, lane) >= state.user_mute(track, lane),
                "track {track} lane {lane}: emitted must dominate the user mute"
            );
        }
    }

    // And a batch whose *trim* record is refused rolls the solo bit back with everything else.
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    stage_solo(&mut host, 0, 1, true, QUANTUM);
    stage_trim(&mut host, 1, 1, 2, 99.0, QUANTUM);
    assert_eq!(host.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.command_report().reason, COMMAND_REASON_DOMAIN);
    let state = host.live_control_solo().expect("solo state");
    assert!(
        !state.any_solo(),
        "a batch refused by a trim record left a solo bit engaged"
    );
    assert!(!state.transaction_open());
}

/// A trim command is not a mute: it does not touch the solo composition, and solo does not touch
/// the trim.
///
/// The two live on different queues and different stages -- the input chain is the head of the
/// strip, the gate is at the fader -- and the phase must not have coupled them. A trim to `-144 dB`
/// silences a track without setting its user mute, so clearing a solo restores exactly the mutes
/// the caller set and leaves the trim where it was.
#[test]
fn a_trim_is_not_a_mute_and_solo_does_not_move_it() {
    const QUANTUM: u32 = 128;
    const TRACKS: usize = 4;
    let mut host = solo_host(QUANTUM, TRACKS, &[]);
    stage_trim(&mut host, 0, 0, 2, -144.0, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let state = host.live_control_solo().expect("solo state");
    assert!(
        !state.user_mute(0, 0) && !state.user_mute(0, 1),
        "a trim ride is not a mute: the strip's user-mute state is untouched"
    );
    assert!(!state.any_solo());

    // Engage and clear a solo over the top: the mute mirror returns to where it was, and the trim
    // record was never a mute record so nothing about it is restored or re-emitted.
    stage_solo(&mut host, 0, 1, true, QUANTUM);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let state = host.live_control_solo().expect("solo state");
    assert!(state.emitted_mute(0, 0), "track 0 is outside the solo set");
    assert!(!state.user_mute(0, 0), "and its user mute is still clear");

    stage_solo(&mut host, 0, 1, false, QUANTUM);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let state = host.live_control_solo().expect("solo state");
    for track in 0..TRACKS {
        for lane in 0..2 {
            assert!(
                !state.emitted_mute(track, lane),
                "clearing the solo restored exactly the mutes the caller set, which is none"
            );
        }
    }
}

#[test]
fn prepared_companion_preserves_ordinary_batch_atomicity() {
    let mut host = live_control_host(128, 0);
    let generation = host.host_generation;
    let workspace = host
        .buffers
        .as_mut()
        .unwrap()
        .prepared_control
        .as_mut()
        .unwrap();
    workspace.companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
    workspace.companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    workspace.companion[8..16].copy_from_slice(&generation.to_le_bytes());
    workspace.config.fill(0xa5);
    let config_before = workspace.config;
    assert_eq!(host.copy_eq_target_config(0, 0, 0), RESULT_INVALID_ARGUMENT);
    assert_eq!(host.eq_target_config().unwrap(), config_before);
    for (index, track) in [(0, 0), (1, u32::MAX)] {
        stage_command(
            &mut host,
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
    }
    let before = host.ready.as_ref().unwrap().controls[0]
        .producer
        .success_count();
    assert_eq!(
        host.submit_prepared_commands(2, 24),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().rejected_index, 1);
    assert_eq!(host.command_report().admitted, 0);
    assert_eq!(
        host.ready.as_ref().unwrap().controls[0]
            .producer
            .success_count(),
        before
    );
    // A malformed envelope also cannot publish the already valid ordinary command.
    assert_eq!(
        host.submit_prepared_commands(1, 23),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        host.ready.as_ref().unwrap().controls[0]
            .producer
            .success_count(),
        before
    );
    assert_eq!(host.submit_prepared_commands(1, 24), RESULT_OK);
    assert_eq!(host.command_report().admitted, 1);
    assert_eq!(
        host.ready.as_ref().unwrap().controls[0]
            .producer
            .success_count(),
        before + 1
    );

    let mut no_live_controls = prepared_host(128);
    assert!(no_live_controls.prepared_companion_mut().is_none());
    assert_eq!(no_live_controls.prepared_companion_capacity(), 0);
    assert!(no_live_controls.eq_target_config().is_none());
}

#[test]
fn ordinary_spectrum_single_and_collection_use_explicit_prepared_hops() {
    let document = one_track_resource_session(128);
    let target = SpectrumTarget::TrackPostMatrix("eq0".into());

    let single_hop = SpectrumHop::new(256).expect("supported single hop");
    let mut single = AudioWorkletEngineHost::boot_with_spectrum_config(
        document.as_bytes(),
        boot_options(128),
        Some(SpectrumPreparationRequest::Single(SpectrumCaptureRequest {
            target: target.clone(),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: u64::MAX,
        })),
        Some(single_hop),
    )
    .expect("single explicit-hop boot");
    assert_eq!(single.spectrum_hop, Some(single_hop));
    assert_eq!(
        single
            .start_spectrum_stream()
            .expect("single explicit-hop start")
            .hop_frames(),
        single_hop.get()
    );

    let collection_hop = SpectrumHop::new(1024).expect("supported collection hop");
    let mut collection = AudioWorkletEngineHost::boot_with_spectrum_config(
        document.as_bytes(),
        boot_options(128),
        Some(SpectrumPreparationRequest::Collection(
            SpectrumCaptureCollectionRequest {
                entries: vec![host_core::SpectrumCaptureCollectionEntry {
                    target: target.clone(),
                    channels: SpectrumChannels::Stereo,
                }],
                maximum_capture_bytes: u64::MAX,
            },
        )),
        Some(collection_hop),
    )
    .expect("collection explicit-hop boot");
    assert_eq!(
        collection.select_spectrum(&target, SpectrumChannels::Stereo),
        RESULT_OK
    );
    assert_eq!(collection.spectrum_hop, Some(collection_hop));
    assert_eq!(
        collection
            .start_spectrum_stream()
            .expect("collection explicit-hop start")
            .hop_frames(),
        collection_hop.get()
    );
}

#[test]
fn ordinary_spectrum_omitted_hop_keeps_default_cadence() {
    let document = one_track_resource_session(128);
    let mut host = AudioWorkletEngineHost::boot_with_spectrum(
        document.as_bytes(),
        boot_options(128),
        Some(SpectrumPreparationRequest::Single(SpectrumCaptureRequest {
            target: SpectrumTarget::TrackPostMatrix("eq0".into()),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: u64::MAX,
        })),
    )
    .expect("default-hop boot");
    assert_eq!(host.spectrum_hop, None);
    assert_eq!(
        host.start_spectrum_stream().expect("default-hop start"),
        SpectrumCadence::new(48_000, 128).expect("default cadence")
    );
}

// ---------------------------------------------------------------------------------------------
// Issue #1096 (S1c): the browser record addresses a console slot by its slot index and an insert
// by its index.
// ---------------------------------------------------------------------------------------------

const ADDRESSING_QUANTUM: u32 = 128;
const ADDRESSING_BLOCKS: u64 = 8;
/// Track `eq5` of the addressing session: not the first, so a track off-by-one cannot pass.
const ADDRESSING_TRACK: u32 = 5;
/// `(rack, effect_index, the instance it names on every track)`, from the session's own slot
/// order: `pre_insert` [eq, pre-eq2], then `post_insert` [post-eq], and the inserts [ins-eq].
const ADDRESSING_CASES: [(u8, u32, &str); 4] = [
    (RACK_CONSOLE, 0, "eq"),
    (RACK_CONSOLE, 1, "pre-eq2"),
    (RACK_CONSOLE, 2, "post-eq"),
    (RACK_INSERTS, 0, "ins-eq"),
];

/// The EQ bank console widened to two `pre_insert` slots, one `post_insert` slot and one insert
/// on each of its eight tracks, every instance a parametric EQ at its own frequency and a gain that
/// differs by track, with `bypassed` (a track and an instance) set to session `bypass`. The same
/// session as `host-core/tests/live_addressing.rs`.
fn addressing_document(bypassed: Option<(&str, &str)>) -> String {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-bank-console.json"
    ))
    .expect("EQ bank console fixture");
    let template = model.console.pre_insert[0].clone();
    let slot = |id: &str| session::ConsoleSlot {
        slot: session::StableId::parse(id).expect("slot id"),
        ..template.clone()
    };
    model.console.pre_insert = vec![slot("eq"), slot("pre-eq2")];
    model.console.post_insert = vec![slot("post-eq")];
    let band = |params: &[session::EffectParam], frequency_hz: f32, gain_db: f32| {
        params
            .iter()
            .map(|param| {
                let mut param = param.clone();
                match param.parameter_id {
                    3 => param.value = frequency_hz,
                    4 => param.value = gain_db,
                    _ => {}
                }
                param
            })
            .collect::<Vec<_>>()
    };
    for (index, track) in model.tracks.iter_mut().enumerate() {
        let spread = index as f32;
        let params = track.console[0].params.clone();
        let entry = |id: &str, frequency_hz: f32, gain_db: f32| session::ConsoleEntry {
            slot: session::StableId::parse(id).expect("slot id"),
            bypass: bypassed == Some((track.id.as_str(), id)),
            params: band(&params, frequency_hz, gain_db),
        };
        track.console = vec![
            entry("eq", 1_000.0, 1.0 + spread),
            entry("pre-eq2", 300.0, -2.0 - spread),
            entry("post-eq", 5_000.0, 3.0 + spread),
        ];
        track.inserts.effects = vec![session::Effect {
            id: session::StableId::parse("ins-eq").expect("insert id"),
            identity: template.identity.clone(),
            quality: template.quality,
            bypass: bypassed == Some((track.id.as_str(), "ins-eq")),
            link_mode: template.link_mode,
            params: band(&params, 150.0, 12.0 - spread),
            sidechain: session::SidechainDeclaration::None,
        }];
    }
    canonical_session_json(&model).expect("canonical addressing session")
}

fn addressing_host(document: &str) -> AudioWorkletEngineHost {
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: ADDRESSING_QUANTUM * 2,
            live_control_command_queue_records: 8,
            ..boot_options(ADDRESSING_QUANTUM)
        },
    )
    .unwrap_or_else(|failure| {
        panic!(
            "addressing boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

/// A deterministic broadband block, so every EQ band is audible.
fn addressing_noise(block: u64, channel: u32) -> Vec<f32> {
    let mut state = (block as u32).wrapping_mul(0x9e37_79b9) ^ channel.wrapping_mul(0x85eb_ca6b);
    (0..ADDRESSING_QUANTUM)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((state >> 8) as f32 / (1_u32 << 24) as f32 - 0.5) * 0.5
        })
        .collect()
}

/// Render [`ADDRESSING_BLOCKS`] quanta of noise and return every output sample's bits.
fn addressing_render(host: &mut AudioWorkletEngineHost) -> Vec<u32> {
    let mut bits = Vec::new();
    for block in 0..ADDRESSING_BLOCKS {
        let (left, right) = (addressing_noise(block, 0), addressing_noise(block, 1));
        let planes: [&[f32]; 2] = [&left, &right];
        assert_eq!(
            host.submit_source(
                b"fixture-source",
                1,
                block * u64::from(ADDRESSING_QUANTUM),
                48_000,
                &planes,
                ADDRESSING_QUANTUM,
                false,
            ),
            RESULT_OK,
        );
        assert_eq!(host.render_next(), RESULT_OK);
        bits.extend(
            host.output_pcm()
                .expect("output")
                .iter()
                .map(|x| x.to_bits()),
        );
    }
    bits
}

/// `(track_id, effect_id)` of every effect queue holding an admitted, undrained record.
fn loaded_effect_queues(host: &AudioWorkletEngineHost) -> Vec<(String, String)> {
    let ready = host.ready.as_ref().expect("ready");
    let base = ready.tracks.len() * 3;
    ready
        .effect_controls
        .iter()
        .enumerate()
        .filter_map(|(effect, producer)| {
            let producer = producer.as_ref()?;
            (ready.in_flight[base + effect] > 0).then(|| {
                (
                    producer.track_id.to_string(),
                    producer.effect_id.to_string(),
                )
            })
        })
        .collect()
}

/// A browser `effectBypass` record at `(rack, effect_index)` reaches the addressed lane: exactly
/// the addressed instance's queue is loaded, and the render is the session bypass of that instance
/// on that track, bit for bit. A live bypass drained before the first block and a session bypass
/// are the same per-lane shunt (issue #1087), and the shunt selects whole blocks.
///
/// Red mutations: `effect_compiler::declared_live_addresses` giving `post_insert` the base `0`
/// (the `post_insert` slot addressed as `pre_insert` slot 0: boot refuses the colliding table), or
/// `LiveEffectAddress::lower` taking console slot `k >= pre_insert` to `(Simd1, k - pre_insert)`
/// (it bypasses `eq` instead of `post-eq`); decoding rack `3` as the inserts.
#[test]
fn a_browser_bypass_reaches_the_addressed_lane() {
    let base = addressing_render(&mut addressing_host(&addressing_document(None)));
    let mut renders = Vec::new();
    for (rack, effect_index, instance) in ADDRESSING_CASES {
        let mut host = addressing_host(&addressing_document(None));
        stage_command(
            &mut host,
            0,
            COMMAND_EFFECT_BYPASS,
            rack,
            255,
            ADDRESSING_TRACK,
            effect_index,
            0,
            0,
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(host.submit_commands(1), RESULT_OK, "{instance}");
        assert_eq!(
            loaded_effect_queues(&host),
            [("eq5".to_owned(), instance.to_owned())],
            "rack {rack}, effect {effect_index} loads exactly eq5/{instance}"
        );
        let live = addressing_render(&mut host);
        let oracle = addressing_render(&mut addressing_host(&addressing_document(Some((
            "eq5", instance,
        )))));
        assert!(
            live == oracle,
            "rack {rack}, effect {effect_index} renders the session bypass of eq5/{instance}"
        );
        assert!(live != base, "bypassing eq5/{instance} is audible");
        renders.push(live);
    }
    for (index, render) in renders.iter().enumerate() {
        for later in &renders[index + 1..] {
            assert!(
                render != later,
                "each addressed instance moves the mix in its own way"
            );
        }
    }
}

/// A browser EQ parameter change at `(rack, effect_index)` reaches the addressed lane through the
/// prepared path: the configuration copy and the companion record carry the same address, exactly
/// the addressed instance's queue is loaded, and the ride is audible and distinct per instance.
///
/// Red mutations: those of [`a_browser_bypass_reaches_the_addressed_lane`], and resolving the
/// companion record's `rack` through the retired lowered codes.
#[test]
fn a_browser_parameter_reaches_the_addressed_lane() {
    let base = addressing_render(&mut addressing_host(&addressing_document(None)));
    let mut renders = Vec::new();
    for (rack, effect_index, instance) in ADDRESSING_CASES {
        let mut host = addressing_host(&addressing_document(None));
        assert_eq!(
            submit_prepared_eq_parameter(
                &mut host,
                0,
                ADDRESSING_TRACK,
                rack,
                effect_index,
                4,
                -18.0
            ),
            RESULT_OK,
            "{instance}"
        );
        assert_eq!(
            loaded_effect_queues(&host),
            [("eq5".to_owned(), instance.to_owned())],
            "rack {rack}, effect {effect_index} loads exactly eq5/{instance}"
        );
        let live = addressing_render(&mut host);
        assert!(live != base, "riding eq5/{instance} is audible");
        renders.push(live);
    }
    for (index, render) in renders.iter().enumerate() {
        for later in &renders[index + 1..] {
            assert!(
                render != later,
                "each addressed instance rides its own lane"
            );
        }
    }
}

/// Decision 12's wire identity (issue #1096): the retired `simd1` (`0`) and `simd2` (`2`) rack
/// codes are refused on every effect-addressed record, never reinterpreted, and the refusal leaves
/// the engine untouched. Each refused record is otherwise the valid twin of one the console admits: at
/// the base, rack `0` effect `0` was this session's `eq`.
///
/// Red mutation: decode `0` as the console or `2` as the inserts, or accept any `rack <= 3` -> a
/// retired record is admitted.
#[test]
fn retired_rack_codes_are_refused_on_every_browser_record() {
    let document = addressing_document(None);
    let base = addressing_render(&mut addressing_host(&document));
    let mut host = addressing_host(&document);
    // `(kind, channel, parameter_id, values)`: a ride, a bypass and both observation kinds.
    let records: [(u32, u8, u32, [f32; 4]); 4] = [
        (COMMAND_EFFECT_PARAM, 2, 4, [-6.0, 0.0, 0.0, 0.0]),
        (COMMAND_EFFECT_BYPASS, 255, 0, [1.0, 0.0, 0.0, 0.0]),
        (COMMAND_OBSERVE_SUBSCRIBE, 255, 1, [0.0; 4]),
        (COMMAND_OBSERVE_UNSUBSCRIBE, 255, 1, [0.0; 4]),
    ];
    for retired in [0_u8, 2] {
        for (kind, channel, parameter_id, values) in records {
            stage_command(
                &mut host,
                0,
                kind,
                retired,
                channel,
                ADDRESSING_TRACK,
                0,
                parameter_id,
                0,
                values,
            );
            assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
            let report = *host.command_report();
            assert_eq!(
                (report.reason, report.admitted, report.rejected_index),
                (COMMAND_REASON_UNKNOWN_RACK, 0, 0),
                "kind {kind} at retired rack {retired}"
            );
        }
        // The prepared EQ path: the configuration copy refuses the address outright.
        assert_eq!(
            host.copy_eq_target_config(ADDRESSING_TRACK, u32::from(retired), 0),
            RESULT_INVALID_ARGUMENT,
            "configuration copy at retired rack {retired}"
        );
        // A companion record carrying a retired rack beside a valid console record is malformed.
        stage_prepared_eq_parameter(&mut host, 0, ADDRESSING_TRACK, RACK_CONSOLE, 0, 4, -6.0);
        host.prepared_companion_mut().expect("prepared companion")[28..32]
            .copy_from_slice(&u32::from(retired).to_le_bytes());
        assert_eq!(
            host.submit_prepared_commands(1, 104),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            host.command_report().reason,
            COMMAND_REASON_MALFORMED,
            "companion at retired rack {retired}"
        );
    }
    assert!(
        loaded_effect_queues(&host).is_empty(),
        "nothing was admitted"
    );
    // The valid twins are admitted, so each refusal above was the rack's alone.
    assert_eq!(
        host.copy_eq_target_config(ADDRESSING_TRACK, u32::from(RACK_CONSOLE), 0),
        RESULT_OK
    );
    stage_command(
        &mut host,
        0,
        COMMAND_EFFECT_BYPASS,
        RACK_CONSOLE,
        255,
        ADDRESSING_TRACK,
        0,
        0,
        0,
        [0.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    assert_eq!(
        addressing_render(&mut host),
        base,
        "the refusals moved nothing, and un-bypassing an enabled slot renders the base"
    );
}

/// The two exceptions to a live bypass lifting a session bypass (P1 verdict L4, #1100): the delay
/// and the multiband keep their session bypass as a prepared one, so un-bypassing either is
/// admitted and renders nothing different. The compressor beside them lowers its session bypass to
/// the shunt, and un-bypassing it is audible, which is what makes the two exceptions observable.
///
/// Red mutation: drop `miso.delay` or `miso.multiband-compressor` from the prepared-bypass
/// effects (`effect_compiler::lowers_session_bypass`) -> lifting its bypass is audible.
#[test]
fn a_live_bypass_cannot_lift_the_delay_or_multiband_session_bypass() {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/compressor-dynamic-observation.json"
    ))
    .expect("accepted compressor fixture");
    model.quantum_frames = ADDRESSING_QUANTUM;
    model.sources[0].frames = u64::from(ADDRESSING_QUANTUM) * 64;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let mut compressor = model.tracks[0].inserts.effects[0].clone();
    // A threshold under the noise and a makeup gain, so the compressor is never transparent.
    for param in &mut compressor.params {
        match param.parameter_id {
            1 => param.value = -30.0,
            6 => param.value = 6.0,
            _ => {}
        }
    }
    let bypassed = |id: &str, effect_id: &str| session::Effect {
        id: session::StableId::parse(id).expect("insert id"),
        identity: session::EffectIdentity::Native {
            effect_id: session::StableId::parse(effect_id).expect("effect id"),
        },
        bypass: true,
        params: Vec::new(),
        ..compressor.clone()
    };
    model.tracks[0].inserts.effects = vec![
        bypassed("delay", "miso.delay"),
        bypassed("multiband", "miso.multiband-compressor"),
        session::Effect {
            bypass: true,
            ..compressor
        },
    ];
    let source_id = model.sources[0].id.as_str().as_bytes().to_vec();
    let document = canonical_session_json(&model).expect("canonical exceptions session");
    let boot = || {
        AudioWorkletEngineHost::boot(
            document.as_bytes(),
            WebBootOptions {
                source_ring_frames: ADDRESSING_QUANTUM * 2,
                live_control_command_queue_records: 8,
                ..boot_options(ADDRESSING_QUANTUM)
            },
        )
        .unwrap_or_else(|failure| {
            panic!(
                "exceptions boot: {}",
                String::from_utf8_lossy(failure.diagnostic())
            )
        })
    };
    let render = |host: &mut AudioWorkletEngineHost| {
        let mut bits = Vec::new();
        for block in 0..ADDRESSING_BLOCKS {
            let (left, right) = (addressing_noise(block, 0), addressing_noise(block, 1));
            let planes: [&[f32]; 2] = [&left, &right];
            assert_eq!(
                host.submit_source(
                    &source_id,
                    1,
                    block * u64::from(ADDRESSING_QUANTUM),
                    48_000,
                    &planes,
                    ADDRESSING_QUANTUM,
                    false,
                ),
                RESULT_OK,
            );
            assert_eq!(host.render_next(), RESULT_OK);
            bits.extend(
                host.output_pcm()
                    .expect("output")
                    .iter()
                    .map(|x| x.to_bits()),
            );
        }
        bits
    };
    let base = render(&mut boot());
    for (effect_index, lifts) in [(0, false), (1, false), (2, true)] {
        let mut host = boot();
        stage_command(
            &mut host,
            0,
            COMMAND_EFFECT_BYPASS,
            RACK_INSERTS,
            255,
            0,
            effect_index,
            0,
            0,
            [0.0; 4],
        );
        assert_eq!(host.submit_commands(1), RESULT_OK, "insert {effect_index}");
        assert_eq!(host.command_report().reason, COMMAND_REASON_NONE);
        assert_eq!(
            render(&mut host) != base,
            lifts,
            "un-bypassing insert {effect_index} is audible exactly when its bypass is a shunt"
        );
    }
}

// Issue #1213: browser live commands address submix strips. The index word is a strip index:
// tracks first, then submixes in canonical order.

const STRIP_QUANTUM: u32 = 128;
/// Source length of every #1213 session, in quanta: longer than any gate renders.
const STRIP_SOURCE_BLOCKS: u64 = 160;

/// A deterministic sample in `(-0.9, 0.9)`, never zero, for lane `lane` of feed `feed` at
/// absolute frame `frame` (splitmix64 of the three), so every lane of every feed differs and no
/// two frames repeat: a lane swap or an index error cannot hide behind a constant (VERIFY-2 M13).
fn strip_sample(feed: u64, lane: u64, frame: u64) -> f32 {
    let mut z = feed.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ lane.wrapping_add(1).wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ frame.wrapping_add(1).wrapping_mul(0x94D0_49BB_1331_11EB);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let unit = (z >> 40) as f32 / (1_u64 << 24) as f32;
    let sample = (unit * 2.0 - 1.0) * 0.9;
    if sample == 0.0 { 0.5 } else { sample }
}

/// Feed `feed`'s two lanes for one block.
fn strip_planes(feed: u64, block: u64) -> [Vec<f32>; 2] {
    let quantum = u64::from(STRIP_QUANTUM);
    let lane = |lane: u64| {
        (block * quantum..(block + 1) * quantum)
            .map(|frame| strip_sample(feed, lane, frame))
            .collect::<Vec<f32>>()
    };
    [lane(0), lane(1)]
}

fn submit_strip_source(
    host: &mut AudioWorkletEngineHost,
    id: &str,
    block: u64,
    planes: &[Vec<f32>; 2],
) {
    let views: [&[f32]; 2] = [&planes[0], &planes[1]];
    assert_eq!(
        host.submit_source(
            id.as_bytes(),
            1,
            block * u64::from(STRIP_QUANTUM),
            48_000,
            &views,
            STRIP_QUANTUM,
            false,
        ),
        RESULT_OK,
        "source {id} block {block}"
    );
}

fn strip_id(text: &str) -> session::StableId {
    session::StableId::parse(text).expect("stable id")
}

/// The observation fixture emptied of tracks, sources and routes, returning the source, a
/// transparent track template, the fixture's compressor insert and a track-to-output route.
fn strip_base() -> (
    session::SessionModel,
    session::Source,
    session::Track,
    session::Effect,
    session::Route,
) {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture");
    model.quantum_frames = STRIP_QUANTUM;
    let mut source = model.sources.pop().expect("fixture source");
    source.frames = u64::from(STRIP_QUANTUM) * STRIP_SOURCE_BLOCKS;
    let mut track = model.tracks.swap_remove(0);
    let compressor = track.inserts.effects[0].clone();
    let route = model.routes.swap_remove(0);
    model.tracks.clear();
    model.routes.clear();
    model.sources.clear();
    model.automation.clear();
    let unity = session::Submix::unity(strip_id("unused"), &model.console);
    track.builtins = unity.builtins;
    track.console = unity.console;
    track.inserts = unity.inserts;
    track.fader = unity.fader;
    track.matrix_or_pan = unity.matrix_or_pan;
    (model, source, track, compressor, route)
}

/// Adds a source and a track reading it, both named `id`, with `track`'s strip.
fn strip_add_track(
    model: &mut session::SessionModel,
    source: &session::Source,
    track: &session::Track,
    id: &str,
) {
    let mut source = source.clone();
    source.id = strip_id(id);
    model.sources.push(source);
    let mut track = track.clone();
    track.id = strip_id(id);
    track.source_id = strip_id(id);
    model.tracks.push(track);
}

/// A route `id` from `source`'s post-pan point to `destination`, with `matrix` at 0 dB.
fn strip_route(
    template: &session::Route,
    id: &str,
    source: session::RouteSource,
    destination: session::RouteDestination,
    matrix: [f32; 4],
) -> session::Route {
    let mut route = template.clone();
    route.id = strip_id(id);
    route.source = source;
    route.destination = destination;
    route.channel_matrix = session::ChannelMatrix {
        ll: matrix[0],
        lr: matrix[1],
        rl: matrix[2],
        rr: matrix[3],
    };
    route.gain_db = 0.0;
    route
}

fn strip_post_pan(id: &str, submix: bool) -> session::RouteSource {
    if submix {
        session::RouteSource::Submix {
            submix_id: strip_id(id),
            tap: session::SendTap::PostPan,
        }
    } else {
        session::RouteSource::Track {
            track_id: strip_id(id),
            tap: session::SendTap::PostPan,
        }
    }
}

fn strip_into(bus: &str) -> session::RouteDestination {
    session::RouteDestination::SubmixInput {
        submix_id: strip_id(bus),
    }
}

/// Strip `S` of gates 1, 4, 5 and 7: per-lane trims and input filters, `insert` (the fixture's
/// compressor unless a gate swaps it), an asymmetric fader and a non-identity pan; no console
/// slots.
fn strip_s(model: &session::SessionModel, insert: session::Effect) -> session::Submix {
    let mut strip = session::Submix::unity(strip_id("bus"), &model.console);
    strip.builtins.left.trim_db = 3.0;
    strip.builtins.right.trim_db = -2.0;
    strip.builtins.left.hpf_hz = 40.0;
    strip.builtins.left.lpf_hz = 15_000.0;
    strip.builtins.right.hpf_hz = 60.0;
    strip.builtins.right.lpf_hz = 12_000.0;
    strip.inserts.effects = vec![insert];
    strip.fader.left_db = -6.0;
    strip.fader.right_db = -4.0;
    strip.matrix_or_pan = session::MatrixOrPan::Pan {
        left: 0.8,
        right: 0.35,
        smoothing_samples: 0,
    };
    strip
}

/// Gate 1's routes, `(route id, feeding track, matrix)`, in declaration order. The route IDs are
/// not in track order, so route-ID order (the order the bus sums in) is `t1, t2, t0`.
const STRIP_ROUTES: [(&str, usize, [f32; 4]); 3] = [
    ("route-c", 0, [0.8, -0.3, 0.25, 0.6]),
    ("route-a", 1, [-0.5, 0.7, 0.9, -0.2]),
    ("route-b", 2, [0.35, 0.15, -0.65, 0.45]),
];

/// Gate 1's sessions, with strip `S` carrying `insert`:
///
/// * A: tracks `t0..t2` (transparent), each reading its own source, routed by [`STRIP_ROUTES`]
///   into submix `bus` (strip `S`), and `bus` at unity to the output. `bus` is strip index 3.
/// * B: one track `ref` with strip `S`, reading source `ref`, at unity to the output.
fn strip_pair_documents(insert: Option<session::Effect>) -> (String, String) {
    let (mut a, source, track, compressor, route) = strip_base();
    let insert = insert.unwrap_or(compressor);
    let output = route.destination.clone();
    for index in 0..3 {
        strip_add_track(&mut a, &source, &track, &format!("t{index}"));
    }
    for (id, feeder, matrix) in STRIP_ROUTES {
        a.routes.push(strip_route(
            &route,
            id,
            strip_post_pan(&format!("t{feeder}"), false),
            strip_into("bus"),
            matrix,
        ));
    }
    a.submixes = vec![strip_s(&a, insert.clone())];
    a.routes.push(strip_route(
        &route,
        "bus-main",
        strip_post_pan("bus", true),
        output.clone(),
        [1.0, 0.0, 0.0, 1.0],
    ));

    let (mut b, source, mut track, _, route) = strip_base();
    let s = strip_s(&b, insert);
    track.builtins = s.builtins;
    track.console = s.console;
    track.inserts = s.inserts;
    track.fader = s.fader;
    track.matrix_or_pan = s.matrix_or_pan;
    strip_add_track(&mut b, &source, &track, "ref");
    b.routes.push(strip_route(
        &route,
        "ref-main",
        strip_post_pan("ref", false),
        output,
        [1.0, 0.0, 0.0, 1.0],
    ));
    (
        canonical_session_json(&a).expect("bus session canonicalizes"),
        canonical_session_json(&b).expect("reference session canonicalizes"),
    )
}

/// The bus's input for one block: each route's `l' = (lr * r) + (ll * l)` with two roundings,
/// added left to right in route-ID order (DESIGN D3 and D9). The routes are at 0 dB, so the
/// folded matrix is the declared one.
fn strip_bus_sum(block: u64) -> [Vec<f32>; 2] {
    let mut routes = STRIP_ROUTES;
    routes.sort_by(|x, y| x.0.cmp(y.0));
    let frames = STRIP_QUANTUM as usize;
    let mut sum = [vec![0.0_f32; frames], vec![0.0_f32; frames]];
    for (rank, (_, feeder, [ll, lr, rl, rr])) in routes.into_iter().enumerate() {
        let planes = strip_planes(feeder as u64, block);
        for frame in 0..frames {
            let (l, r) = (planes[0][frame], planes[1][frame]);
            let left = (lr * r) + (ll * l);
            let right = (rr * r) + (rl * l);
            if rank == 0 {
                sum[0][frame] = left;
                sum[1][frame] = right;
            } else {
                sum[0][frame] += left;
                sum[1][frame] += right;
            }
        }
    }
    sum
}

/// Index of `bus` in session A: three tracks, then submix 0.
const STRIP_BUS: u32 = 3;

fn strip_boot(document: &str, options: WebBootOptions) -> AudioWorkletEngineHost {
    AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
        panic!(
            "#1213 boot: {}",
            String::from_utf8_lossy(failure.diagnostic())
        )
    })
}

fn strip_options(queue_records: u64, meter_blocks: u64, master_plus_one: u64) -> WebBootOptions {
    WebBootOptions {
        source_ring_frames: STRIP_QUANTUM * 4,
        live_control_command_queue_records: queue_records,
        live_control_meter_blocks: meter_blocks,
        live_control_observation_taps: 1,
        live_control_master_track_plus_one: master_plus_one,
        ..boot_options(STRIP_QUANTUM)
    }
}

/// Gate 1's hosts: A over the bus session, B over the reference session.
fn strip_pair(
    insert: Option<session::Effect>,
    queue_records: u64,
    meter_blocks: u64,
) -> (AudioWorkletEngineHost, AudioWorkletEngineHost) {
    let (bus, reference) = strip_pair_documents(insert);
    (
        strip_boot(&bus, strip_options(queue_records, meter_blocks, 0)),
        strip_boot(&reference, strip_options(queue_records, meter_blocks, 0)),
    )
}

/// Render one block on both hosts -- A's three sources, B fed the bus's sum -- and return whether
/// the block carried any nonzero sample, after requiring the two outputs to be bit-identical.
fn strip_render_pair(
    a: &mut AudioWorkletEngineHost,
    b: &mut AudioWorkletEngineHost,
    block: u64,
    what: &str,
) -> bool {
    for feeder in 0..3_u64 {
        submit_strip_source(
            a,
            &format!("t{feeder}"),
            block,
            &strip_planes(feeder, block),
        );
    }
    submit_strip_source(b, "ref", block, &strip_bus_sum(block));
    assert_eq!(a.render_next(), RESULT_OK, "{what}: A block {block}");
    assert_eq!(b.render_next(), RESULT_OK, "{what}: B block {block}");
    let left = a.output_pcm().expect("A output");
    let right = b.output_pcm().expect("B output");
    assert_eq!(left.len(), right.len());
    for (index, (x, y)) in left.iter().zip(right).enumerate() {
        assert_eq!(
            x.to_bits(),
            y.to_bits(),
            "{what}: block {block} sample {index}: bus {x} vs track {y}"
        );
    }
    left.iter().any(|sample| *sample != 0.0)
}

/// One gate-1 command: `(what, kind, rack, channel, effect, parameter, smoothing, values)`.
type StripCommand = (&'static str, u32, u8, u8, u32, u32, u32, [f32; 4]);

/// Issue #1213 gate 1: every live strip kind sent to a bus renders the bits the same command
/// renders on a track with the same strip, fed the bus's sum.
///
/// Test value: red if an index at or past `T` is refused, lands in a track's band, or reaches
/// another strip's queue, or if any band spelling still uses the track count. No earlier test
/// addressed a bus.
#[test]
fn live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track() {
    for smoothing in [0_u32, 480] {
        let (mut a, mut b) = strip_pair(None, 64, 0);
        let mut block = 0_u64;
        let mut audible = false;
        for _ in 0..2 {
            audible |= strip_render_pair(&mut a, &mut b, block, "before any command");
            block += 1;
        }
        let commands: [StripCommand; 13] = [
            (
                "pan",
                COMMAND_PAN,
                255,
                255,
                0,
                0,
                smoothing,
                [-0.2, 0.7, 0.0, 0.0],
            ),
            (
                "matrix",
                COMMAND_MATRIX,
                255,
                255,
                0,
                0,
                smoothing,
                [0.9, -0.3, 0.2, 0.6],
            ),
            (
                "fader",
                COMMAND_FADER_DB,
                255,
                2,
                0,
                0,
                smoothing,
                [-9.0, 0.0, 0.0, 0.0],
            ),
            (
                "left fader",
                COMMAND_FADER_DB,
                255,
                0,
                0,
                0,
                smoothing,
                [-1.5, 0.0, 0.0, 0.0],
            ),
            (
                "left mute",
                COMMAND_MUTE,
                255,
                0,
                0,
                0,
                smoothing,
                [1.0, 0.0, 0.0, 0.0],
            ),
            (
                "unmute",
                COMMAND_MUTE,
                255,
                2,
                0,
                0,
                smoothing,
                [0.0, 0.0, 0.0, 0.0],
            ),
            (
                "threshold",
                COMMAND_EFFECT_PARAM,
                RACK_INSERTS,
                2,
                0,
                1,
                smoothing,
                [-18.0, 0.0, 0.0, 0.0],
            ),
            (
                "bypass",
                COMMAND_EFFECT_BYPASS,
                RACK_INSERTS,
                255,
                0,
                0,
                0,
                [1.0, 0.0, 0.0, 0.0],
            ),
            (
                "unbypass",
                COMMAND_EFFECT_BYPASS,
                RACK_INSERTS,
                255,
                0,
                0,
                0,
                [0.0, 0.0, 0.0, 0.0],
            ),
            (
                "subscribe",
                COMMAND_OBSERVE_SUBSCRIBE,
                RACK_INSERTS,
                255,
                0,
                1,
                2,
                [0.0; 4],
            ),
            (
                "unsubscribe",
                COMMAND_OBSERVE_UNSUBSCRIBE,
                RACK_INSERTS,
                255,
                0,
                1,
                2,
                [0.0; 4],
            ),
            (
                "right trim",
                COMMAND_TRIM_DB,
                255,
                1,
                0,
                0,
                smoothing,
                [4.0, 0.0, 0.0, 0.0],
            ),
            (
                "left polarity",
                COMMAND_POLARITY_INVERT,
                255,
                0,
                0,
                0,
                smoothing,
                [1.0, 0.0, 0.0, 0.0],
            ),
        ];
        for (what, kind, rack, channel, effect, parameter, window, values) in commands {
            let what = format!("{what} (smoothing {smoothing})");
            for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
                stage_command(
                    host, 0, kind, rack, channel, index, effect, parameter, window, values,
                );
                assert_eq!(
                    host.submit_commands(1),
                    RESULT_OK,
                    "{what} at strip {index}"
                );
            }
            assert_eq!(
                a.command_report().applied_at_sample,
                b.command_report().applied_at_sample,
                "{what}"
            );
            for _ in 0..5 {
                audible |= strip_render_pair(&mut a, &mut b, block, &what);
                block += 1;
            }
        }
        // Kind 12 rides the prepared path: the companion carries the targets designed off the
        // render thread from the strip's own committed input-filter configuration.
        let what = format!("input filters (smoothing {smoothing})");
        for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
            let bytes = stage_prepared_input_filter(host, 0, index, 120.0, 9_000.0);
            assert_eq!(
                host.submit_prepared_commands(1, bytes),
                RESULT_OK,
                "{what} at strip {index}"
            );
        }
        // One batch of several kinds to the bus, admitted whole.
        for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
            stage_command(
                host,
                0,
                COMMAND_FADER_DB,
                255,
                1,
                index,
                0,
                0,
                smoothing,
                [-3.0, 0.0, 0.0, 0.0],
            );
            stage_command(
                host,
                1,
                COMMAND_PAN,
                255,
                255,
                index,
                0,
                0,
                smoothing,
                [0.1, -0.1, 0.0, 0.0],
            );
            stage_command(
                host,
                2,
                COMMAND_EFFECT_PARAM,
                RACK_INSERTS,
                2,
                index,
                0,
                1,
                0,
                [-24.0, 0.0, 0.0, 0.0],
            );
            stage_command(
                host,
                3,
                COMMAND_POLARITY_INVERT,
                255,
                0,
                index,
                0,
                0,
                smoothing,
                [0.0, 0.0, 0.0, 0.0],
            );
            assert_eq!(host.submit_commands(4), RESULT_OK, "batch at strip {index}");
        }
        for _ in 0..6 {
            audible |= strip_render_pair(&mut a, &mut b, block, &what);
            block += 1;
        }
        assert!(audible, "the compared output is not silence");
    }
}

/// Issue #1213 gate 1, the prepared-EQ arm (attempt 2, verdict MINOR-1): a live EQ parameter
/// staged through the prepared companion on a bus insert renders the bits the same edit renders
/// on the reference track's insert.
///
/// Test value: red if the prepared-owner EQ path spells its queue base or its admission owner
/// marker with the track count instead of the strip count (`prepared_queue_address`'s Eq band, the
/// admission's EQ `queue_slot`), which gate 1's compressor insert never exercises, or if the
/// admitted bus edit lands with a value other than the reference track's (band 1 is enabled, so
/// the -12 dB gain edit is audible).
#[test]
fn a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track() {
    let (_, _, _, mut eq, _) = strip_base();
    eq.id = strip_id("eq");
    eq.identity = session::EffectIdentity::Native {
        effect_id: strip_id("miso.parametric-eq"),
    };
    // `band-1-enabled` defaults off, which would leave the band-1 gain edit inaudible and the
    // render comparison vacuous.
    eq.params = vec![session::EffectParam {
        parameter_id: 1,
        channel: session::ParameterChannel::Both,
        unit: session::ParameterUnit::Linear,
        value: 1.0,
    }];
    let (bus, reference) = strip_pair_documents(Some(eq));
    let mut a = strip_boot(&bus, strip_options(64, 0, 0));
    let mut b = strip_boot(&reference, strip_options(64, 0, 0));
    let mut block = 0;
    for _ in 0..2 {
        strip_render_pair(&mut a, &mut b, block, "before");
        block += 1;
    }
    for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
        stage_prepared_eq_parameter(host, 0, index, RACK_INSERTS, 0, 4, -12.0);
        assert_eq!(
            host.submit_prepared_commands(1, 104),
            RESULT_OK,
            "prepared EQ at strip {index}"
        );
    }
    let mut audible = false;
    for _ in 0..6 {
        audible |= strip_render_pair(&mut a, &mut b, block, "prepared EQ");
        block += 1;
    }
    assert!(audible);
}

/// Gates 2 and 3's session: tracks `a`, `b` and `c`, each reading its own source; `a` and `b`
/// feed the bus `drums` post-pan, `a` also sends post-fader into the return `verb`, `c` goes
/// straight to the output, and both submixes reach the output. Strip order `[a, b, c, drums,
/// verb]`, so `drums` is strip index 3 and `verb` 4.
fn solo_bus_host(drums_muted: bool) -> AudioWorkletEngineHost {
    solo_bus_host_lanes([drums_muted, drums_muted])
}

/// [`solo_bus_host`] with `drums`' fader booted with per-lane mutes `[left, right]`.
fn solo_bus_host_lanes(drums_muted: [bool; 2]) -> AudioWorkletEngineHost {
    let (mut model, source, track, _, route) = strip_base();
    let output = route.destination.clone();
    for id in ["a", "b", "c"] {
        strip_add_track(&mut model, &source, &track, id);
    }
    let mut drums = session::Submix::unity(strip_id("drums"), &model.console);
    drums.fader.left_db = -3.0;
    drums.fader.left_mute = drums_muted[0];
    drums.fader.right_mute = drums_muted[1];
    let mut verb = session::Submix::unity(strip_id("verb"), &model.console);
    verb.fader.right_db = -5.0;
    model.submixes = vec![drums, verb];
    let identity = [1.0, 0.0, 0.0, 1.0];
    model.routes = vec![
        strip_route(
            &route,
            "a-drums",
            strip_post_pan("a", false),
            strip_into("drums"),
            [0.9, 0.1, -0.2, 0.7],
        ),
        strip_route(
            &route,
            "b-drums",
            strip_post_pan("b", false),
            strip_into("drums"),
            [0.6, -0.4, 0.3, 0.8],
        ),
        strip_route(
            &route,
            "c-main",
            strip_post_pan("c", false),
            output.clone(),
            identity,
        ),
        strip_route(
            &route,
            "drums-main",
            strip_post_pan("drums", true),
            output.clone(),
            identity,
        ),
        strip_route(
            &route,
            "verb-main",
            strip_post_pan("verb", true),
            output,
            [0.5, 0.0, 0.0, 0.5],
        ),
    ];
    let mut send = strip_route(
        &route,
        "a-verb",
        strip_post_pan("a", false),
        strip_into("verb"),
        [0.3, 0.2, 0.2, 0.3],
    );
    send.source = session::RouteSource::Track {
        track_id: strip_id("a"),
        tap: session::SendTap::PostFader,
    };
    model.routes.push(send);
    let document = canonical_session_json(&model).expect("canonical solo bus session");
    strip_boot(
        &document,
        strip_options(DEFAULT_COMMAND_QUEUE_RECORDS as u64, 0, 0),
    )
}

/// Feed `a`, `b` and `c` one block and render; returns the output.
fn solo_bus_render(host: &mut AudioWorkletEngineHost, block: u64) -> Vec<f32> {
    for (feed, id) in ["a", "b", "c"].into_iter().enumerate() {
        submit_strip_source(host, id, block, &strip_planes(10 + feed as u64, block));
    }
    assert_eq!(host.render_next(), RESULT_OK);
    host.output_pcm().expect("output").to_vec()
}

/// Render `blocks` blocks on both hosts from `first`; when `compare`, require identical bits and
/// return whether any compared sample was nonzero.
fn solo_bus_compare(
    left: &mut AudioWorkletEngineHost,
    right: &mut AudioWorkletEngineHost,
    first: u64,
    blocks: u64,
    what: &str,
) -> bool {
    let mut audible = false;
    for block in first..first + blocks {
        let x = solo_bus_render(left, block);
        let y = solo_bus_render(right, block);
        for (index, (p, q)) in x.iter().zip(&y).enumerate() {
            assert_eq!(
                p.to_bits(),
                q.to_bits(),
                "{what}: block {block} sample {index}"
            );
        }
        audible |= x.iter().any(|sample| *sample != 0.0);
    }
    audible
}

/// Issue #1213 gate 2(a): kind 4 at a bus index mutes the bus, bit-identically to a host booted
/// with that bus's fader muted, once the ramp has settled.
///
/// Test value: red if a bus mute has no owner -- refused, dropped, or landed on a track's fader.
#[test]
fn a_bus_mute_command_equals_the_bus_booted_muted() {
    let mut live = solo_bus_host(false);
    let mut booted = solo_bus_host(true);
    stage_mute(&mut live, 0, 3, true, STRIP_QUANTUM);
    assert_eq!(live.submit_commands(1), RESULT_OK, "mute drums");
    // Two blocks cover the one-quantum ramp; both hosts render them, unchecked.
    for block in 0..2 {
        solo_bus_render(&mut live, block);
        solo_bus_render(&mut booted, block);
    }
    assert!(
        solo_bus_compare(&mut live, &mut booted, 2, 4, "drums muted live vs at boot"),
        "c and verb stay audible"
    );
}

/// Issue #1213 gate 2(a), per lane (attempt 2, verdict MINOR-2): a single-lane kind 4 at a bus
/// index mutes exactly that lane, bit-identically to a host booted with only that lane muted.
///
/// Test value: red if kind 4 takes the effective mute of the wrong lane (for example always the
/// left), which stages `Mute { Right, muted: false }` for a right-lane mute with no solo engaged:
/// an acknowledged mute that never mutes.
#[test]
fn a_single_lane_bus_mute_equals_the_bus_booted_with_that_lane_muted() {
    for (channel, lanes) in [(1_u8, [false, true]), (0_u8, [true, false])] {
        let mut live = solo_bus_host_lanes([false, false]);
        let mut booted = solo_bus_host_lanes(lanes);
        stage_command(
            &mut live,
            0,
            COMMAND_MUTE,
            255,
            channel,
            3,
            0,
            0,
            STRIP_QUANTUM,
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            live.submit_commands(1),
            RESULT_OK,
            "mute drums lane {channel}"
        );
        // Two blocks cover the one-quantum ramp; both hosts render them, unchecked.
        for block in 0..2 {
            solo_bus_render(&mut live, block);
            solo_bus_render(&mut booted, block);
        }
        assert!(
            solo_bus_compare(&mut live, &mut booted, 2, 4, "lane mute live vs at boot"),
            "the other lane, c and verb stay audible"
        );
    }
}

/// Issue #1213 gate 2(b): soloing track `a`, which feeds the bus `drums` and the return `verb`,
/// renders bit-identically to explicit mutes on every other track, and the two submixes stay
/// audible.
///
/// Test value: red if solo composition reaches a strip past `T` (the buses would be solo-muted on
/// the solo side and not on the mute side).
#[test]
fn soloing_a_track_keeps_its_bus_and_return_audible() {
    let mut solo = solo_bus_host(false);
    let mut mute = solo_bus_host(false);
    solo_bus_compare(&mut solo, &mut mute, 0, 1, "before any command");
    stage_solo(&mut solo, 0, 0, true, STRIP_QUANTUM);
    assert_eq!(solo.submit_commands(1), RESULT_OK, "solo a");
    stage_mute(&mut mute, 0, 1, true, STRIP_QUANTUM);
    stage_mute(&mut mute, 1, 2, true, STRIP_QUANTUM);
    assert_eq!(mute.submit_commands(2), RESULT_OK, "mute b and c");
    assert_eq!(
        solo.command_report().applied_at_sample,
        mute.command_report().applied_at_sample
    );
    assert!(
        solo_bus_compare(&mut solo, &mut mute, 1, 5, "solo a vs mute b and c"),
        "a is audible through drums and verb"
    );
    // Only the buses carry `a` to the output: with `drums` and `verb` muted too the output is
    // silent, so the audible blocks above are the submixes'.
    for (index, bus) in [3_u32, 4].into_iter().enumerate() {
        stage_mute(&mut mute, index, bus, true, 0);
    }
    assert_eq!(mute.submit_commands(2), RESULT_OK, "mute both submixes");
    let silent = solo_bus_render(&mut mute, 6);
    assert!(
        silent.iter().all(|sample| *sample == 0.0),
        "a reaches the output only via submixes"
    );
}

/// Issue #1213 gate 2(c): kind 9 at a bus index refuses with `notSoloable` at its wire index and
/// stages nothing, even behind a valid record in the same submission.
///
/// Test value: red if a bus can be soloed, or if the refusal reports another reason or index, or
/// if the valid record before it reaches a queue.
#[test]
fn a_solo_at_a_bus_index_refuses_not_soloable_and_stages_nothing() {
    let mut host = solo_bus_host(false);
    solo_bus_render(&mut host, 0);
    for bus in [3_u32, 4] {
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
        stage_solo(&mut host, 1, bus, true, 0);
        assert_eq!(
            host.submit_commands(2),
            RESULT_INVALID_ARGUMENT,
            "solo strip {bus}"
        );
        let report = *host.command_report();
        assert_eq!(report.result, RESULT_INVALID_ARGUMENT);
        assert_eq!(
            report.reason, COMMAND_REASON_NOT_SOLOABLE,
            "solo strip {bus}"
        );
        assert_eq!(report.rejected_index, 1, "the solo record's wire index");
        let ready = host.ready.as_ref().expect("ready");
        assert!(
            ready.in_flight.iter().all(|count| *count == 0),
            "nothing staged"
        );
        assert!(!ready.solo.any_solo(), "no solo bit moved");
        assert!(!ready.solo.transaction_open(), "the transaction closed");
        for (strip, controls) in ready.controls.iter().enumerate() {
            assert_eq!(
                controls.fader.available_capacity(),
                DEFAULT_COMMAND_QUEUE_RECORDS as usize,
                "strip {strip}'s fader queue untouched"
            );
        }
    }
}

/// Issue #1213 gate 3 (VERIFY-2 M4): with `drums` muted at boot and `a` soloed, kind 4 unmuting
/// `drums` makes it audible -- bit-identical, after the ramp, to a host booted with `drums`
/// unmuted under the same solo.
///
/// Test value: red if kind 4 composes the effective mute inline without `solo_safe`: the unmute
/// then stages `Mute { muted: true }` and `drums` stays silent.
#[test]
fn a_bus_can_be_unmuted_while_a_track_is_soloed() {
    let mut unmuted = solo_bus_host(true);
    let mut reference = solo_bus_host(false);
    for host in [&mut unmuted, &mut reference] {
        stage_solo(host, 0, 0, true, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "solo a");
    }
    for block in 0..2 {
        solo_bus_render(&mut unmuted, block);
        solo_bus_render(&mut reference, block);
    }
    stage_mute(&mut unmuted, 0, 3, false, STRIP_QUANTUM);
    assert_eq!(
        unmuted.submit_commands(1),
        RESULT_OK,
        "unmute drums under solo"
    );
    for block in 2..4 {
        solo_bus_render(&mut unmuted, block);
        solo_bus_render(&mut reference, block);
    }
    assert!(
        solo_bus_compare(
            &mut unmuted,
            &mut reference,
            4,
            4,
            "drums unmuted live vs at boot"
        ),
        "drums and verb carry a"
    );
}

/// Issue #1213 gate 4(a): a batch of a valid bus fader record and an out-of-domain track record
/// stages nothing and returns one refusal at the bad record's index.
///
/// Test value: red if admission pushes a bus record before every record is validated.
#[test]
fn a_bus_record_ahead_of_a_bad_track_record_is_never_pushed() {
    let (mut a, _) = strip_pair(None, 4, 0);
    stage_command(
        &mut a,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        STRIP_BUS,
        0,
        0,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut a,
        1,
        COMMAND_FADER_DB,
        255,
        2,
        0,
        0,
        0,
        0,
        [99.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(a.submit_commands(2), RESULT_INVALID_ARGUMENT);
    let report = *a.command_report();
    assert_eq!(report.reason, COMMAND_REASON_DOMAIN);
    assert_eq!(report.rejected_index, 1, "the bad track record");
    assert_eq!(report.admitted, 0);
    let ready = a.ready.as_ref().expect("ready");
    assert!(
        ready.in_flight.iter().all(|count| *count == 0),
        "nothing staged"
    );
    for (strip, controls) in ready.controls.iter().enumerate() {
        assert_eq!(
            controls.fader.available_capacity(),
            4,
            "strip {strip}'s fader queue"
        );
    }
}

/// Issue #1213 gate 4(b): a batch that overfills one bus queue is typed backpressure at the first
/// record that does not fit, and nothing -- the track record before it included -- is pushed.
///
/// Test value: red if the room check skips a bus queue, so admission pushes into it and fails
/// half-way.
#[test]
fn overfilling_a_bus_queue_is_typed_backpressure_with_no_push() {
    let (mut a, _) = strip_pair(None, 4, 0);
    stage_command(
        &mut a,
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
    for index in 1..6 {
        stage_command(
            &mut a,
            index,
            COMMAND_FADER_DB,
            255,
            2,
            STRIP_BUS,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(a.submit_commands(6), RESULT_BACKPRESSURE);
    let report = *a.command_report();
    assert_eq!(report.reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(
        report.rejected_index, 1,
        "the first bus record names the full queue"
    );
    let ready = a.ready.as_ref().expect("ready");
    assert!(
        ready.in_flight.iter().all(|count| *count == 0),
        "nothing staged"
    );
    for (strip, controls) in ready.controls.iter().enumerate() {
        assert_eq!(
            controls.fader.available_capacity(),
            4,
            "strip {strip}'s fader queue"
        );
    }
}

/// The meter frame's gain-reduction word for strip `strip`, given the host's strip count.
fn strip_gain_word(host: &AudioWorkletEngineHost, strip: usize) -> f32 {
    let strips = host.live_control_tracks().len() + host.meter_header().submix_count as usize;
    let frame = host.meter_frame();
    assert_eq!(frame.len(), strips * 3 + 3);
    frame[strips * 2 + 2 + strip]
}

/// Issue #1213 gate 5 (VERIFY-2 M6), plus the selected read of a bus tap (K2 verdict
/// requirement (a)): kind 7 at the bus's index arms its compressor's gain-reduction tap, whose
/// reading folds into the bus's frame word `gain_base + T + 0`, bit-equal to the word a track
/// with the same strip reports; a selected read at the bus's strip index names the bus; kind 8
/// disarms it and the word returns to `0`.
///
/// Test value: red if a bus effect cannot be armed, folds into a track's word, is read at the
/// wrong frame offset or skipped by the fold (#1207's `>= T` skip), or if a selected read
/// resolves the strip index against the tracks only.
#[test]
fn a_bus_compressor_reports_its_gain_reduction_in_the_bus_word() {
    const WINDOW: u64 = 2;
    let (mut a, mut b) = strip_pair(None, 64, WINDOW);
    assert_eq!(a.set_meter_lease(true), RESULT_OK);
    assert_eq!(b.set_meter_lease(true), RESULT_OK);
    for (host, index) in [(&mut a, STRIP_BUS), (&mut b, 0)] {
        assert_eq!(
            observe(host, index, RACK_INSERTS, 0, 1, WINDOW as u32, true),
            RESULT_OK
        );
    }
    let mut block = 0;
    for _ in 0..4 * WINDOW {
        strip_render_pair(&mut a, &mut b, block, "armed");
        block += 1;
    }
    assert!(
        a.poll_meters() >= 1 && b.poll_meters() >= 1,
        "a window closed"
    );
    let bus = strip_gain_word(&a, STRIP_BUS as usize);
    let reference = strip_gain_word(&b, 0);
    assert!(bus > 0.0, "the bus compressor reduces: {bus}");
    assert_eq!(
        bus.to_bits(),
        reference.to_bits(),
        "bus {bus} vs track {reference}"
    );
    for track in 0..3 {
        assert_eq!(
            strip_gain_word(&a, track).to_bits(),
            0,
            "track {track}'s word"
        );
    }
    let address = |track_index| ObservationAddress {
        track_index,
        rack: LiveEffectRack::Inserts,
        effect_index: 0,
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };
    let bus_read = a
        .read_observation_addresses(&[address(STRIP_BUS)])
        .expect("a selected read at the bus's strip index");
    let track_read = b
        .read_observation_addresses(&[address(0)])
        .expect("the same read on the track");
    assert_eq!(&*bus_read[0].track_id, "bus");
    assert_eq!(&*bus_read[0].effect_slot_id, "comp");
    assert_eq!(bus_read[0].status, track_read[0].status);
    assert_eq!(
        bus_read[0].left.map(f32::to_bits),
        track_read[0].left.map(f32::to_bits)
    );
    assert_eq!(
        bus_read[0].right.map(f32::to_bits),
        track_read[0].right.map(f32::to_bits)
    );
    assert_eq!(
        a.read_observation_addresses(&[address(STRIP_BUS + 1)])
            .err(),
        Some(ObservationReadError::InvalidSelection),
        "one past the last strip"
    );

    assert_eq!(
        observe(&mut a, STRIP_BUS, RACK_INSERTS, 0, 1, WINDOW as u32, false),
        RESULT_OK
    );
    for _ in 0..2 * WINDOW {
        for feeder in 0..3_u64 {
            submit_strip_source(
                &mut a,
                &format!("t{feeder}"),
                block,
                &strip_planes(feeder, block),
            );
        }
        assert_eq!(a.render_next(), RESULT_OK);
        block += 1;
    }
    while a.poll_meters() != 0 {}
    assert_eq!(
        strip_gain_word(&a, STRIP_BUS as usize).to_bits(),
        0,
        "disarmed"
    );
}

/// Issue #1213 gate 6 (VERIFY-2 M6, #1208's condition): booting with the master word `T + 0 + 1`
/// designates the bus, whose true-peak limiter insert is driven over its -6 dB ceiling; with the
/// limiter's tap armed at the bus's index, the header reports a master reading and the master word
/// equals the bus's word, nonzero. `T + S + 1` refuses at boot.
///
/// Test value: red if the boot word still indexes tracks only, or the master reading is taken
/// from another strip's word or never folded for a bus.
#[test]
fn a_bus_limiter_can_be_the_designated_master() {
    const WINDOW: u64 = 2;
    let (_, _, _, mut limiter, _) = strip_base();
    limiter.id = strip_id("limit");
    limiter.identity = session::EffectIdentity::Native {
        effect_id: strip_id("miso.true-peak-limiter"),
    };
    limiter.params = vec![session::EffectParam {
        parameter_id: 1,
        channel: session::ParameterChannel::Both,
        unit: session::ParameterUnit::Db,
        value: -6.0,
    }];
    let (bus, _) = strip_pair_documents(Some(limiter));
    let mut host = strip_boot(&bus, strip_options(64, WINDOW, u64::from(STRIP_BUS) + 1));
    assert_eq!(host.meter_header().master_track_plus_one, STRIP_BUS + 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(
        observe(
            &mut host,
            STRIP_BUS,
            RACK_INSERTS,
            0,
            1,
            WINDOW as u32,
            true
        ),
        RESULT_OK
    );
    for block in 0..4 * WINDOW {
        for feeder in 0..3_u64 {
            submit_strip_source(
                &mut host,
                &format!("t{feeder}"),
                block,
                &strip_planes(feeder, block),
            );
        }
        assert_eq!(host.render_next(), RESULT_OK);
    }
    assert!(host.poll_meters() >= 1, "a window closed");
    let header = *host.meter_header();
    assert_eq!(header.master_gr_present, 1, "the designated bus published");
    let bus_word = strip_gain_word(&host, STRIP_BUS as usize);
    let master_word = strip_gain_word(&host, STRIP_BUS as usize + 1);
    assert!(bus_word > 0.0, "the bus limiter reduces: {bus_word}");
    assert_eq!(
        master_word.to_bits(),
        bus_word.to_bits(),
        "master word is the bus's"
    );

    let failure = AudioWorkletEngineHost::boot(
        bus.as_bytes(),
        strip_options(64, WINDOW, u64::from(STRIP_BUS) + 2),
    )
    .err()
    .expect("one past the last strip refuses");
    assert_eq!(failure.result(), RESULT_REFUSED_DOCUMENT);
    assert!(
        failure
            .diagnostic()
            .starts_with(b"host.observation.master_track\t"),
        "{}",
        String::from_utf8_lossy(failure.diagnostic())
    );
}

/// Issue #1213 gate 7: admitting a batch of bus edits, a bus observation and a track solo, then
/// rendering, allocates and frees nothing.
///
/// Test value: red if strip-sized admission or the strip-wide solo coalescing allocates on the
/// admission or render path. The bus gain-reduction fold runs in `poll_meters`, outside the
/// measured closure; #1209's `bus_meters_render_and_poll_without_allocating` covers it.
#[test]
fn bus_edits_and_a_bus_observation_admit_and_render_without_allocating() {
    let (mut a, _) = strip_pair(None, 64, 2);
    assert_eq!(a.set_meter_lease(true), RESULT_OK);
    for block in 0..2 {
        for feeder in 0..3_u64 {
            submit_strip_source(
                &mut a,
                &format!("t{feeder}"),
                block,
                &strip_planes(feeder, block),
            );
        }
        assert_eq!(a.render_next(), RESULT_OK);
    }
    stage_command(
        &mut a,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        STRIP_BUS,
        0,
        0,
        480,
        [-9.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut a,
        1,
        COMMAND_PAN,
        255,
        255,
        STRIP_BUS,
        0,
        0,
        480,
        [0.2, -0.4, 0.0, 0.0],
    );
    stage_command(
        &mut a,
        2,
        COMMAND_TRIM_DB,
        255,
        0,
        STRIP_BUS,
        0,
        0,
        480,
        [2.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut a,
        3,
        COMMAND_EFFECT_PARAM,
        RACK_INSERTS,
        2,
        STRIP_BUS,
        0,
        1,
        0,
        [-20.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut a,
        4,
        COMMAND_OBSERVE_SUBSCRIBE,
        RACK_INSERTS,
        255,
        STRIP_BUS,
        0,
        1,
        2,
        [0.0; 4],
    );
    stage_command(
        &mut a,
        5,
        COMMAND_MUTE,
        255,
        1,
        STRIP_BUS,
        0,
        0,
        480,
        [1.0, 0.0, 0.0, 0.0],
    );
    stage_solo(&mut a, 6, 1, true, 480);
    let planes: Vec<[Vec<f32>; 2]> = (0..3).map(|feeder| strip_planes(feeder, 2)).collect();
    for (feeder, planes) in planes.iter().enumerate() {
        submit_strip_source(&mut a, &format!("t{feeder}"), 2, planes);
    }
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| (a.submit_commands(7), a.render_next()));
    assert_eq!(admission, RESULT_OK, "the bus batch is admitted");
    assert_eq!(render, RESULT_OK);
    assert_eq!(allocations, 0, "admission/render allocated");
    assert_eq!(deallocations, 0, "admission/render freed");
}

// Issue #1222: browser live send commands. A send is addressed by its live-route index: its
// position among the routes into submixes, in canonical route-ID order.

/// The live sends of [`send_document`], in live-route (route-ID) order:
/// `(route id, source strip id, the source is a submix, destination bus)`. Seven sends against
/// six strips, so a valid live-route index reaches past the strip count.
const SEND_ROUTES: [(&str, &str, bool, &str); 7] = [
    ("send-a", "t0", false, "bx"),
    ("send-b", "t0", false, "by"),
    ("send-c", "t1", false, "bx"),
    ("send-d", "t1", false, "by"),
    ("send-e", "t2", false, "by"),
    ("send-f", "t2", false, "bx"),
    ("send-g", "bx", true, "by"),
];
/// The order [`send_document`] declares the sends in: not route-ID order.
const SEND_DECLARATION_ORDER: [usize; 7] = [6, 2, 0, 5, 3, 1, 4];
/// Strips of [`send_document`]: four tracks, then `bx` and `by`.
const SEND_STRIPS: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
struct SendValues {
    gain_db: f32,
    matrix: [f32; 4],
    mute: bool,
}

/// The session's send values: every matrix asymmetric (`ll != rr`, `lr != rl`) and nonzero.
const SEND_SEEDS: [SendValues; 7] = [
    SendValues {
        gain_db: -3.0,
        matrix: [0.8, -0.3, 0.25, 0.6],
        mute: false,
    },
    SendValues {
        gain_db: 2.0,
        matrix: [-0.5, 0.7, 0.9, -0.2],
        mute: false,
    },
    SendValues {
        gain_db: -6.0,
        matrix: [0.35, 0.15, -0.65, 0.45],
        mute: false,
    },
    SendValues {
        gain_db: 0.0,
        matrix: [0.6, 0.4, -0.3, 0.9],
        mute: false,
    },
    SendValues {
        gain_db: -1.5,
        matrix: [-0.7, -0.2, 0.55, 0.3],
        mute: false,
    },
    SendValues {
        gain_db: 1.0,
        matrix: [0.45, -0.85, 0.1, 0.75],
        mute: true,
    },
    SendValues {
        gain_db: -4.5,
        matrix: [0.5, 0.2, -0.4, 0.95],
        mute: false,
    },
];

/// Four tracks `t0..t3`, each reading its own source, with a unity strip; two unity submixes `bx`
/// and `by` (no console slot, no insert, HPF and LPF off); the [`SEND_ROUTES`] sends at `values`;
/// and every track and bus at unity to the output. `t3` sends nothing.
fn send_document(values: &[SendValues; 7]) -> String {
    let (mut model, source, track, _, route) = strip_base();
    let output = route.destination.clone();
    for index in 0..4 {
        strip_add_track(&mut model, &source, &track, &format!("t{index}"));
    }
    model.submixes = vec![
        session::Submix::unity(strip_id("bx"), &model.console),
        session::Submix::unity(strip_id("by"), &model.console),
    ];
    for live in SEND_DECLARATION_ORDER {
        let (id, source, submix, bus) = SEND_ROUTES[live];
        let mut send = strip_route(
            &route,
            id,
            strip_post_pan(source, submix),
            strip_into(bus),
            values[live].matrix,
        );
        send.gain_db = values[live].gain_db;
        send.mute = values[live].mute;
        model.routes.push(send);
    }
    for strip in ["t0", "t1", "t2", "t3", "bx", "by"] {
        model.routes.push(strip_route(
            &route,
            &format!("{strip}-main"),
            strip_post_pan(strip, strip.starts_with('b')),
            output.clone(),
            [1.0, 0.0, 0.0, 1.0],
        ));
    }
    canonical_session_json(&model).expect("send session canonicalizes")
}

fn send_host(values: &[SendValues; 7], queue_records: u64) -> AudioWorkletEngineHost {
    strip_boot(&send_document(values), strip_options(queue_records, 0, 0))
}

/// Feed every track its own two-lane signal for `block` and render it.
fn send_feed(host: &mut AudioWorkletEngineHost, block: u64) {
    for feed in 0..4_u64 {
        submit_strip_source(host, &format!("t{feed}"), block, &strip_planes(feed, block));
    }
}

fn send_render(host: &mut AudioWorkletEngineHost, block: u64) -> Vec<f32> {
    send_feed(host, block);
    assert_eq!(host.render_next(), RESULT_OK, "block {block}");
    host.output_pcm().expect("output").to_vec()
}

/// Stage one send record at wire index `index`.
fn stage_send(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    kind: u32,
    route: u32,
    smoothing: u32,
    values: [f32; 4],
) {
    stage_command(host, index, kind, 255, 255, route, 0, 0, smoothing, values);
}

/// The host's mirror of every live send, as `SendValues`.
fn send_mirror(host: &AudioWorkletEngineHost) -> Vec<SendValues> {
    let routes = &host.ready.as_ref().expect("ready").routes;
    (0..routes.len())
        .map(|route| {
            let entry = routes.get(route).expect("live route");
            SendValues {
                gain_db: entry.gain_db,
                matrix: entry.matrix,
                mute: entry.mute,
            }
        })
        .collect()
}

/// Every route queue's free room.
fn send_queue_room(host: &AudioWorkletEngineHost) -> Vec<usize> {
    host.ready
        .as_ref()
        .expect("ready")
        .route_controls
        .iter()
        .map(RouteControlProducer::free)
        .collect()
}

/// splitmix64, for the randomized gate.
struct SendDraw(u64);

impl SendDraw {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }

    /// Uniform in `[low, high)`.
    fn uniform(&mut self, low: f32, high: f32) -> f32 {
        let unit = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        low + (high - low) * unit
    }

    /// A nonzero coefficient in `±[0.1, 1)`.
    fn coefficient(&mut self) -> f32 {
        let magnitude = self.uniform(0.1, 1.0);
        if self.below(2) == 0 {
            magnitude
        } else {
            -magnitude
        }
    }
}

/// Issue #1222 gate 1: kinds 13, 14 and 15 sent at random live routes settle, bit for bit, on the
/// output of a host booted from a session holding the edited values.
///
/// Every track reads its own source with distinct lanes, the sends reach two different buses
/// (and one bus feeds the other), every matrix is asymmetric, and the sends are declared out of
/// route-ID order. Live-route index 6 is past the strip count.
///
/// Test value: red if a kind's spelling or the index addresses the wrong send (by declaration
/// order, by overall route index or by strip), the matrix words are read in another order, or a
/// record is built from a stale mirror field. No browser path has driven a send before.
#[test]
fn a_live_send_edit_lands_on_a_fresh_plans_bits() {
    let mut draw = SendDraw(0x1222);
    for trial in 0..4 {
        for smoothing in [0_u32, 480] {
            let mut edits: Vec<(u32, usize, [f32; 4])> = Vec::new();
            let mut kinds = vec![
                COMMAND_ROUTE_GAIN_DB,
                COMMAND_ROUTE_MUTE,
                COMMAND_ROUTE_MATRIX,
            ];
            for _ in 0..draw.below(4) {
                kinds.push(COMMAND_ROUTE_GAIN_DB + draw.below(3) as u32);
            }
            for _ in 0..kinds.len() {
                let pick = draw.below(kinds.len() as u64) as usize;
                let last = kinds.len() - 1;
                kinds.swap(pick, last);
            }
            let mut target = SEND_SEEDS;
            for kind in kinds {
                let route = draw.below(SEND_ROUTES.len() as u64) as usize;
                let values = match kind {
                    COMMAND_ROUTE_GAIN_DB => {
                        let gain_db = draw.uniform(-24.0, 6.0);
                        target[route].gain_db = gain_db;
                        [gain_db, 0.0, 0.0, 0.0]
                    }
                    COMMAND_ROUTE_MUTE => {
                        let mute = draw.below(2) == 1;
                        target[route].mute = mute;
                        [f32::from(u8::from(mute)), 0.0, 0.0, 0.0]
                    }
                    _ => {
                        let matrix = [
                            draw.coefficient(),
                            draw.coefficient(),
                            draw.coefficient(),
                            draw.coefficient(),
                        ];
                        target[route].matrix = matrix;
                        matrix
                    }
                };
                edits.push((kind, route, values));
            }
            let what = format!("trial {trial}, smoothing {smoothing}, edits {edits:?}");
            let mut live = send_host(&SEND_SEEDS, 16);
            let mut fresh = send_host(&target, 16);
            for block in 0..2 {
                send_render(&mut live, block);
                send_render(&mut fresh, block);
            }
            // Two batches, one block apart, so a send can be edited across batches too.
            let split = edits.len() / 2;
            for (batch, block) in [(&edits[..split], 2_u64), (&edits[split..], 3)] {
                for (index, (kind, route, values)) in batch.iter().enumerate() {
                    stage_send(&mut live, index, *kind, *route as u32, smoothing, *values);
                }
                if !batch.is_empty() {
                    assert_eq!(
                        live.submit_commands(batch.len() as u32),
                        RESULT_OK,
                        "{what}: batch at block {block}"
                    );
                }
                send_render(&mut live, block);
                send_render(&mut fresh, block);
            }
            assert_eq!(send_mirror(&live), target, "{what}: the mirror");
            // 480 samples settle within four 128-frame blocks of the last batch.
            for block in 4..8 {
                send_render(&mut live, block);
                send_render(&mut fresh, block);
            }
            let mut audible = false;
            for block in 8..12 {
                let left = send_render(&mut live, block);
                let right = send_render(&mut fresh, block);
                for (sample, (x, y)) in left.iter().zip(&right).enumerate() {
                    assert_eq!(
                        x.to_bits(),
                        y.to_bits(),
                        "{what}: block {block} sample {sample}: live {x} vs fresh {y}"
                    );
                }
                audible |= left.iter().any(|sample| *sample != 0.0);
            }
            assert!(audible, "{what}: the output carries signal");
        }
    }
}

/// The sends of [`seeded_send_document`], in live-route (route-ID) order:
/// `(route id, source strip id, the source is a submix, destination bus, follows_mute)`. Every send
/// taps pre-fader, so a muted strip still carries signal at the tap and only `follows_mute` can
/// silence its lanes there. Following sends leave every track for `bx` and three for `by`; `f-h`
/// and `f-j` do not follow; `f-i` follows bus `bx`, whose right lane is muted.
const SEEDED_SENDS: [(&str, &str, bool, &str, bool); 10] = [
    ("f-a", "t0", false, "bx", true),
    ("f-b", "t1", false, "bx", true),
    ("f-c", "t2", false, "bx", true),
    ("f-d", "t3", false, "bx", true),
    ("f-e", "t0", false, "by", true),
    ("f-f", "t1", false, "by", true),
    ("f-g", "t2", false, "by", true),
    ("f-h", "t3", false, "by", false),
    ("f-i", "bx", true, "by", true),
    ("f-j", "t1", false, "by", false),
];

/// Four tracks `t0..t3` with fader lane mutes `[F,F]`, `[T,F]`, `[F,T]` and `[T,T]` (every
/// combination), each reading its own source; unity submixes `bx` (right lane muted) and `by`; the
/// [`SEEDED_SENDS`] at `values`, declared in reverse route-ID order; and every strip at unity to
/// the output. With `with_inserts`, every track carries the fixture's compressor insert, so the
/// effect band is four queues long.
fn seeded_send_document(values: &[SendValues], with_inserts: bool) -> String {
    let (mut model, source, mut track, compressor, route) = strip_base();
    if with_inserts {
        track.inserts.effects = vec![compressor];
    }
    let output = route.destination.clone();
    let mutes = [[false, false], [true, false], [false, true], [true, true]];
    for (index, [left, right]) in mutes.into_iter().enumerate() {
        strip_add_track(&mut model, &source, &track, &format!("t{index}"));
        model.tracks[index].fader.left_mute = left;
        model.tracks[index].fader.right_mute = right;
    }
    let mut bx = session::Submix::unity(strip_id("bx"), &model.console);
    bx.fader.right_mute = true;
    model.submixes = vec![bx, session::Submix::unity(strip_id("by"), &model.console)];
    for (live, (id, source, submix, bus, follows)) in SEEDED_SENDS.iter().enumerate().rev() {
        let tap = session::SendTap::PreFader;
        let source = if *submix {
            session::RouteSource::Submix {
                submix_id: strip_id(source),
                tap,
            }
        } else {
            session::RouteSource::Track {
                track_id: strip_id(source),
                tap,
            }
        };
        let mut send = strip_route(&route, id, source, strip_into(bus), values[live].matrix);
        send.gain_db = values[live].gain_db;
        send.mute = values[live].mute;
        send.follows_mute = *follows;
        model.routes.push(send);
    }
    for strip in ["t0", "t1", "t2", "t3", "bx", "by"] {
        model.routes.push(strip_route(
            &route,
            &format!("{strip}-main"),
            strip_post_pan(strip, strip.starts_with('b')),
            output.clone(),
            [1.0, 0.0, 0.0, 1.0],
        ));
    }
    canonical_session_json(&model).expect("seeded send session canonicalizes")
}

/// [`seeded_send_document`]'s session values: unmuted, every matrix nonzero and drawn at random.
fn seeded_send_values() -> Vec<SendValues> {
    let mut draw = SendDraw(0xFEED);
    (0..SEEDED_SENDS.len())
        .map(|_| SendValues {
            gain_db: draw.uniform(-9.0, 3.0),
            matrix: [
                draw.coefficient(),
                draw.coefficient(),
                draw.coefficient(),
                draw.coefficient(),
            ],
            mute: false,
        })
        .collect()
}

/// One batch of random kind 13, 14 and 15 records on [`seeded_send_document`]'s sends, six trials
/// at smoothing 0 and 480 each: every record must take exactly one slot of its own send's queue,
/// and the output must settle, bit for bit, on a host booted from the edited values.
fn seeded_send_edits_land_on_a_fresh_plan(with_inserts: bool, seed: u64) {
    let seeds = seeded_send_values();
    let mut draw = SendDraw(seed);
    for trial in 0..6 {
        for smoothing in [0_u32, 480] {
            let mut target = seeds.clone();
            let mut edits: Vec<(u32, usize, [f32; 4])> = Vec::new();
            for _ in 0..3 + draw.below(6) {
                let kind = COMMAND_ROUTE_GAIN_DB + draw.below(3) as u32;
                let route = draw.below(SEEDED_SENDS.len() as u64) as usize;
                let values = match kind {
                    COMMAND_ROUTE_GAIN_DB => {
                        let gain_db = draw.uniform(-24.0, 6.0);
                        target[route].gain_db = gain_db;
                        [gain_db, 0.0, 0.0, 0.0]
                    }
                    COMMAND_ROUTE_MUTE => {
                        let mute = draw.below(2) == 1;
                        target[route].mute = mute;
                        [f32::from(u8::from(mute)), 0.0, 0.0, 0.0]
                    }
                    _ => {
                        let matrix = [
                            draw.coefficient(),
                            draw.coefficient(),
                            draw.coefficient(),
                            draw.coefficient(),
                        ];
                        target[route].matrix = matrix;
                        matrix
                    }
                };
                edits.push((kind, route, values));
            }
            let what = format!(
                "inserts {with_inserts}, trial {trial}, smoothing {smoothing}, edits {edits:?}"
            );
            let options = strip_options(16, 0, 0);
            let mut live = strip_boot(&seeded_send_document(&seeds, with_inserts), options);
            let mut fresh = strip_boot(&seeded_send_document(&target, with_inserts), options);
            assert_eq!(
                live.ready.as_ref().expect("ready").route_controls.len(),
                SEEDED_SENDS.len()
            );
            for block in 0..2 {
                send_render(&mut live, block);
                send_render(&mut fresh, block);
            }
            let before = send_queue_room(&live);
            for (index, (kind, route, values)) in edits.iter().enumerate() {
                stage_send(&mut live, index, *kind, *route as u32, smoothing, *values);
            }
            assert_eq!(
                live.submit_commands(edits.len() as u32),
                RESULT_OK,
                "{what}"
            );
            let mut expected = before;
            for (_, route, _) in &edits {
                expected[*route] -= 1;
            }
            assert_eq!(
                send_queue_room(&live),
                expected,
                "{what}: each record on its own send's queue"
            );
            // 480 samples settle within four 128-frame blocks of the batch.
            for block in 2..7 {
                send_render(&mut live, block);
                send_render(&mut fresh, block);
            }
            let mut audible = false;
            for block in 7..11 {
                let left = send_render(&mut live, block);
                let right = send_render(&mut fresh, block);
                for (sample, (x, y)) in left.iter().zip(&right).enumerate() {
                    assert_eq!(
                        x.to_bits(),
                        y.to_bits(),
                        "{what}: block {block} sample {sample}: live {x} vs fresh {y}"
                    );
                }
                audible |= left.iter().any(|sample| *sample != 0.0);
            }
            assert!(audible, "{what}: the output carries signal");
        }
    }
}

/// Issue #1222, from its verdict's MINOR-1 and MINOR-2: live edits on following pre-fader sends
/// whose source strips carry every lane-mute combination -- tracks `[F,F]`, `[T,F]`, `[F,T]`,
/// `[T,T]` and a bus with its right lane muted -- take one slot of their own send's queue each and
/// settle, bit for bit, on a fresh plan, so the mirror's seeded `source_lane_muted` is the
/// prepared plan's follow-zeroed lanes. The session runs twice: without effects, and with an
/// insert on every track, so the send band starts after a four-queue effect band (`3S + E + r`).
///
/// Test value: red if a send record is built without the mirror's `source_lane_muted`, so a live
/// gain, matrix or unmute edit on a send that follows a muted strip leaks the muted lane into the
/// bus; no other test edits a send whose source lanes start muted.
#[test]
fn a_following_sends_seeded_source_lanes_land_on_a_fresh_plans_bits() {
    seeded_send_edits_land_on_a_fresh_plan(false, 0xA11CE);
    seeded_send_edits_land_on_a_fresh_plan(true, 0xB0B);
}

/// Require a refused submission's typed report, an empty staging count, untouched send and fader
/// queues and an unchanged, closed send mirror.
fn assert_send_refusal(
    host: &AudioWorkletEngineHost,
    reason: u32,
    rejected_index: u32,
    room: &[usize],
    fader_room: &[usize],
    what: &str,
) {
    let report = *host.command_report();
    assert_eq!(report.reason, reason, "{what}: reason");
    assert_eq!(report.rejected_index, rejected_index, "{what}: wire index");
    assert_eq!(report.admitted, 0, "{what}: admitted");
    assert_eq!(send_queue_room(host), room, "{what}: send queues");
    let ready = host.ready.as_ref().expect("ready");
    let faders: Vec<usize> = ready
        .controls
        .iter()
        .map(|controls| controls.fader.available_capacity())
        .collect();
    assert_eq!(faders, fader_room, "{what}: fader queues");
    assert!(!ready.routes.transaction_open(), "{what}: mirror closed");
    assert_eq!(send_mirror(host), SEND_SEEDS, "{what}: mirror unchanged");
}

/// Issue #1237 gate 3: a live send edit outside the route domain -- a gain past `[-144, 24]` dB
/// or a matrix coefficient past `[-1, 1]` -- is refused `DOMAIN` and moves nothing; the bounds
/// themselves, and subnormal coefficients of both signs (`±1.0e-45`, `±2^-127`), are admitted
/// and land in the mirror with their exact bits.
///
/// Test value: red if the browser's send admission bounds a live value other than as the session
/// does (no bound, an exclusive one, another limit, or a refusal of a subnormal coefficient the
/// session accepts), so a value no session may hold could be pushed live, or a value a session
/// holds refused.
#[test]
fn a_live_send_edit_outside_the_route_domain_is_refused() {
    const DEPTH: u64 = 4;
    let room = vec![DEPTH as usize; SEND_ROUTES.len()];
    let fader_room = vec![DEPTH as usize; SEND_STRIPS as usize];
    let gain = |value: f32| (COMMAND_ROUTE_GAIN_DB, [value, 0.0, 0.0, 0.0]);
    let matrix = |position: usize, value: f32| {
        let mut values = SEND_SEEDS[0].matrix;
        values[position] = value;
        (COMMAND_ROUTE_MATRIX, values)
    };
    let mut refused = vec![gain(24.5), gain(-144.5)];
    let mut admitted = vec![gain(24.0), gain(-144.0)];
    for position in 0..4 {
        refused.extend([matrix(position, 1.5), matrix(position, -1.5)]);
        admitted.extend([matrix(position, 1.0), matrix(position, -1.0)]);
        // Subnormal coefficients of both signs are in the session's domain (#1237 attempt 2).
        for subnormal in [1.0e-45, f32::MIN_POSITIVE / 2.0] {
            admitted.extend([matrix(position, subnormal), matrix(position, -subnormal)]);
        }
    }
    for (kind, values) in refused {
        let mut host = send_host(&SEND_SEEDS, DEPTH);
        send_render(&mut host, 0);
        stage_send(&mut host, 0, kind, 0, 0, values);
        assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT);
        let what = format!("kind {kind}, {values:?}");
        assert_send_refusal(&host, COMMAND_REASON_DOMAIN, 0, &room, &fader_room, &what);
    }
    for (kind, values) in admitted {
        let mut host = send_host(&SEND_SEEDS, DEPTH);
        send_render(&mut host, 0);
        stage_send(&mut host, 0, kind, 0, 0, values);
        let what = format!("kind {kind}, {values:?}");
        assert_eq!(host.submit_commands(1), RESULT_OK, "{what}");
        let mut expected = SEND_SEEDS;
        if kind == COMMAND_ROUTE_GAIN_DB {
            expected[0].gain_db = values[0];
        } else {
            expected[0].matrix = values;
        }
        assert_eq!(send_mirror(&host), expected, "{what}");
    }
}

/// One track `t0` routed to the output at -144 dB through `matrix`, and nothing else audible.
fn subnormal_route_document(matrix: [f32; 4]) -> String {
    let (mut model, source, track, _, route) = strip_base();
    let output = route.destination.clone();
    strip_add_track(&mut model, &source, &track, "t0");
    let mut main = strip_route(
        &route,
        "t0-main",
        strip_post_pan("t0", false),
        output,
        matrix,
    );
    main.gain_db = -144.0;
    model.routes.push(main);
    canonical_session_json(&model).expect("subnormal-route session canonicalizes")
}

/// Issue #1237 gate 4 (the render): a route whose folded products are subnormal (`±1.0e-35` at
/// -144 dB, about `±6.3e-43`) renders bit-identical to the same route with those coefficients
/// `0.0`, through the browser host. The route is the output's only input, so a subnormal
/// constant would surface as a nonzero subnormal sample.
///
/// Test value: red if a subnormal folded product reaches the bound route constant (the flush
/// removed from `gated_route_coefficients`), since the output would then carry `x * 6.3e-43`
/// instead of a signed zero.
#[test]
fn a_subnormal_route_renders_as_a_zero_coefficient() {
    let tiny = 1.0e-35_f32;
    let mut subnormal = strip_boot(
        &subnormal_route_document([tiny, -tiny, 0.5, -tiny]),
        strip_options(4, 0, 0),
    );
    let mut zero = strip_boot(
        &subnormal_route_document([0.0, 0.0, 0.5, 0.0]),
        strip_options(4, 0, 0),
    );
    let mut nonzero = 0;
    for block in 0..4 {
        let render = |host: &mut AudioWorkletEngineHost| {
            submit_strip_source(host, "t0", block, &strip_planes(0, block));
            assert_eq!(host.render_next(), RESULT_OK, "block {block}");
            host.output_pcm().expect("output").to_vec()
        };
        let left = render(&mut subnormal);
        let right = render(&mut zero);
        for (sample, (x, y)) in left.iter().zip(&right).enumerate() {
            assert_eq!(
                x.to_bits(),
                y.to_bits(),
                "block {block} sample {sample}: subnormal route {x:e} vs zero {y:e}"
            );
            nonzero += usize::from(*y != 0.0);
        }
    }
    // The `rl = 0.5` column is audible, so the comparison is of a rendering route.
    assert!(nonzero > 0, "the route renders something");
}

/// Issue #1222 gate 2: a send batch is all or nothing.
///
/// * A valid `routeGainDb` and a `routeMatrix` with a coefficient outside `[-1, 1]` (finite on
///   the wire, so decode admits it, and refused by the route's own domain rule, issue #1237)
///   stage nothing and refuse `DOMAIN` at the second record.
/// * A batch that overfills one send queue is typed backpressure at its first record on that
///   queue, behind a valid fader record, and pushes nothing.
/// * A valid send record and a valid fader record, with that fader queue already full, push
///   neither; nor, in a session with an insert on every track, do a valid send record and a valid
///   effect-bypass record with that effect queue already full.
///
/// In every case the send mirror is unchanged. And a refused batch after an admitted one keeps
/// the admitted values: the mirror holds them, and a further edit settles on a fresh plan holding
/// them.
///
/// Test value: red if admission pushes a send record, or commits the send mirror, before every
/// record is validated and every queue -- send and effect queues included -- has room, or if a
/// refused batch rolls the mirror back past an admitted batch the render plane applied.
#[test]
fn a_refused_send_batch_pushes_nothing_and_keeps_the_mirror() {
    const DEPTH: u64 = 4;
    let depth = DEPTH as usize;
    let fader_room = vec![depth; SEND_STRIPS as usize];
    let room = vec![depth; SEND_ROUTES.len()];

    // `ll = 1.5` is outside the route coefficient domain `[-1, 1]` (issue #1237; before the
    // domain this drew `3e38`, which folded past `f32::MAX` at +2 dB).
    let mut host = send_host(&SEND_SEEDS, DEPTH);
    send_render(&mut host, 0);
    stage_send(
        &mut host,
        0,
        COMMAND_ROUTE_GAIN_DB,
        0,
        0,
        [-9.0, 0.0, 0.0, 0.0],
    );
    stage_send(
        &mut host,
        1,
        COMMAND_ROUTE_MATRIX,
        1,
        0,
        [1.5, 0.1, 0.2, 0.3],
    );
    assert_eq!(host.submit_commands(2), RESULT_INVALID_ARGUMENT);
    assert_send_refusal(
        &host,
        COMMAND_REASON_DOMAIN,
        1,
        &room,
        &fader_room,
        "domain",
    );

    let mut host = send_host(&SEND_SEEDS, DEPTH);
    send_render(&mut host, 0);
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
    for index in 1..=depth + 1 {
        stage_send(
            &mut host,
            index,
            COMMAND_ROUTE_GAIN_DB,
            2,
            0,
            [-1.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(host.submit_commands(DEPTH as u32 + 2), RESULT_BACKPRESSURE);
    assert_send_refusal(
        &host,
        COMMAND_REASON_BACKPRESSURE,
        1,
        &room,
        &fader_room,
        "send full",
    );

    let mut host = send_host(&SEND_SEEDS, DEPTH);
    send_render(&mut host, 0);
    for index in 0..depth {
        stage_command(
            &mut host,
            index,
            COMMAND_FADER_DB,
            255,
            2,
            3,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(
        host.submit_commands(DEPTH as u32),
        RESULT_OK,
        "fill t3's fader queue"
    );
    stage_send(&mut host, 0, COMMAND_ROUTE_MUTE, 4, 0, [1.0, 0.0, 0.0, 0.0]);
    stage_command(
        &mut host,
        1,
        COMMAND_FADER_DB,
        255,
        2,
        3,
        0,
        0,
        0,
        [-3.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_BACKPRESSURE);
    let mut full = fader_room.clone();
    full[3] = 0;
    assert_send_refusal(
        &host,
        COMMAND_REASON_BACKPRESSURE,
        1,
        &room,
        &full,
        "fader full",
    );

    // With an insert on every track the effect band sits between the strip bands and the send
    // band: a send record beside an effect record whose queue is full pushes neither.
    let mut host = strip_boot(
        &seeded_send_document(&seeded_send_values(), true),
        strip_options(DEPTH, 0, 0),
    );
    send_render(&mut host, 0);
    for index in 0..depth {
        stage_command(
            &mut host,
            index,
            COMMAND_EFFECT_BYPASS,
            RACK_INSERTS,
            255,
            0,
            0,
            0,
            0,
            [(index % 2) as f32, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(
        host.submit_commands(DEPTH as u32),
        RESULT_OK,
        "fill t0's insert queue"
    );
    let room = send_queue_room(&host);
    let mirror = send_mirror(&host);
    stage_send(
        &mut host,
        0,
        COMMAND_ROUTE_GAIN_DB,
        0,
        0,
        [-12.0, 0.0, 0.0, 0.0],
    );
    stage_command(
        &mut host,
        1,
        COMMAND_EFFECT_BYPASS,
        RACK_INSERTS,
        255,
        0,
        0,
        0,
        0,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(2), RESULT_BACKPRESSURE);
    let report = *host.command_report();
    assert_eq!(report.reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(report.rejected_index, 1, "refused at the effect record");
    assert_eq!(report.admitted, 0);
    assert_eq!(
        send_queue_room(&host),
        room,
        "the send record is not pushed"
    );
    assert_eq!(send_mirror(&host), mirror, "the mirror is unchanged");
    assert!(
        !host
            .ready
            .as_ref()
            .expect("ready")
            .routes
            .transaction_open()
    );

    // A refusal after an admission rolls the mirror back to the admitted values, which the render
    // plane has applied, and a further edit is built from them.
    let mut host = send_host(&SEND_SEEDS, DEPTH);
    send_render(&mut host, 0);
    stage_send(
        &mut host,
        0,
        COMMAND_ROUTE_GAIN_DB,
        0,
        0,
        [-9.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(host.submit_commands(1), RESULT_OK);
    send_render(&mut host, 1);
    // `ll = 1.5` is outside the route coefficient domain `[-1, 1]` (issue #1237; it was `3e38`).
    stage_send(
        &mut host,
        0,
        COMMAND_ROUTE_MATRIX,
        0,
        0,
        [0.1, 0.2, 0.3, 0.4],
    );
    stage_send(
        &mut host,
        1,
        COMMAND_ROUTE_MATRIX,
        1,
        0,
        [1.5, 0.1, 0.2, 0.3],
    );
    assert_eq!(host.submit_commands(2), RESULT_INVALID_ARGUMENT);
    let mut admitted = SEND_SEEDS;
    admitted[0].gain_db = -9.0;
    assert_eq!(
        send_mirror(&host),
        admitted,
        "the admitted gain survives the refusal"
    );
    stage_send(&mut host, 0, COMMAND_ROUTE_MUTE, 0, 0, [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let mut fresh = send_host(&admitted, DEPTH);
    send_render(&mut fresh, 0);
    send_render(&mut fresh, 1);
    for block in 2..6 {
        let left = send_render(&mut host, block);
        let right = send_render(&mut fresh, block);
        for (sample, (x, y)) in left.iter().zip(&right).enumerate() {
            assert_eq!(
                x.to_bits(),
                y.to_bits(),
                "block {block} sample {sample}: live {x} vs fresh {y}"
            );
        }
    }
}

/// Issue #1222 gate 3: the index word is checked against the count its kind addresses. A strip
/// kind at or past the strip count is `UNKNOWN_TRACK`; a send kind at or past the live-route
/// count is `unknownRoute`, below the strip count too (a session with fewer sends than strips);
/// and a valid send index past the strip count (a session with more) is admitted.
///
/// Test value: red if the bounds check runs before kind dispatch (send index 6 is refused as an
/// unknown track, send index 5 passes) or against the wrong count, or if a send reuses the track
/// reason.
#[test]
fn every_kind_is_bounded_by_the_count_it_addresses() {
    let room = vec![16_usize; SEND_ROUTES.len()];
    let fader_room = vec![16_usize; SEND_STRIPS as usize];
    let mut host = send_host(&SEND_SEEDS, 16);
    send_render(&mut host, 0);
    for strip in [SEND_STRIPS, SEND_STRIPS + 1, u32::MAX] {
        stage_command(
            &mut host,
            0,
            COMMAND_FADER_DB,
            255,
            2,
            strip,
            0,
            0,
            0,
            [-6.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            host.submit_commands(1),
            RESULT_INVALID_ARGUMENT,
            "strip {strip}"
        );
        assert_send_refusal(
            &host,
            COMMAND_REASON_UNKNOWN_TRACK,
            0,
            &room,
            &fader_room,
            "strip",
        );
    }
    for kind in [
        COMMAND_ROUTE_GAIN_DB,
        COMMAND_ROUTE_MUTE,
        COMMAND_ROUTE_MATRIX,
    ] {
        for route in [7_u32, 8, u32::MAX] {
            stage_send(
                &mut host,
                0,
                COMMAND_ROUTE_GAIN_DB,
                0,
                0,
                [-2.0, 0.0, 0.0, 0.0],
            );
            stage_send(&mut host, 1, kind, route, 0, [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(
                host.submit_commands(2),
                RESULT_INVALID_ARGUMENT,
                "send {route}"
            );
            assert_send_refusal(
                &host,
                COMMAND_REASON_UNKNOWN_ROUTE,
                1,
                &room,
                &fader_room,
                "send past the live routes",
            );
        }
    }
    stage_send(
        &mut host,
        0,
        COMMAND_ROUTE_GAIN_DB,
        6,
        0,
        [-2.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(
        host.submit_commands(1),
        RESULT_OK,
        "send 6, past the strip count"
    );
    let mut room = room;
    room[6] -= 1;
    assert_eq!(send_queue_room(&host), room, "send 6's queue holds it");

    // Fewer sends than strips: three sends, four strips. Index 3 and the bus's strip index are
    // below the strip count and still not sends.
    let (bus, _) = strip_pair_documents(None);
    let mut host = strip_boot(&bus, strip_options(16, 0, 0));
    assert_eq!(host.ready.as_ref().expect("ready").route_controls.len(), 3);
    for route in [3_u32, STRIP_BUS] {
        stage_send(
            &mut host,
            0,
            COMMAND_ROUTE_MUTE,
            route,
            0,
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            host.submit_commands(1),
            RESULT_INVALID_ARGUMENT,
            "send {route}"
        );
        let report = *host.command_report();
        assert_eq!(report.reason, COMMAND_REASON_UNKNOWN_ROUTE, "send {route}");
        assert_eq!(report.rejected_index, 0);
    }
    stage_send(&mut host, 0, COMMAND_ROUTE_MUTE, 2, 0, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(host.submit_commands(1), RESULT_OK, "the last send");
}

/// One malformed send record: `(what, kind, rack, channel, effect, parameter, values)`.
type SendShape = (&'static str, u32, u8, u8, u32, u32, [f32; 4]);

/// Issue #1222 D1: a send record's fixed shape. Every field a send does not use must be its
/// not-applicable value, and `routeMute` takes exactly `0` or `1`.
///
/// Test value: red if a send kind accepts a lane, a rack, an effect or parameter word, or a value
/// past its own, or takes a non-boolean mute as a mute.
#[test]
fn send_records_are_shape_checked() {
    let mut host = send_host(&SEND_SEEDS, 16);
    send_render(&mut host, 0);
    let room = vec![16_usize; SEND_ROUTES.len()];
    let fader_room = vec![16_usize; SEND_STRIPS as usize];
    let gain = [-2.0, 0.0, 0.0, 0.0];
    let malformed: [SendShape; 7] = [
        ("a lane", COMMAND_ROUTE_GAIN_DB, 255, 2, 0, 0, gain),
        (
            "a rack",
            COMMAND_ROUTE_GAIN_DB,
            RACK_INSERTS,
            255,
            0,
            0,
            gain,
        ),
        (
            "an effect",
            COMMAND_ROUTE_MUTE,
            255,
            255,
            1,
            0,
            [1.0, 0.0, 0.0, 0.0],
        ),
        (
            "a parameter",
            COMMAND_ROUTE_MATRIX,
            255,
            255,
            0,
            1,
            [0.5; 4],
        ),
        (
            "a second gain word",
            COMMAND_ROUTE_GAIN_DB,
            255,
            255,
            0,
            0,
            [-2.0, 0.0, 0.0, 1.0],
        ),
        (
            "a second mute word",
            COMMAND_ROUTE_MUTE,
            255,
            255,
            0,
            0,
            [1.0, 1.0, 0.0, 0.0],
        ),
        (
            "a lane on a matrix",
            COMMAND_ROUTE_MATRIX,
            255,
            0,
            0,
            0,
            [0.5; 4],
        ),
    ];
    for (what, kind, rack, channel, effect, parameter, values) in malformed {
        stage_command(
            &mut host, 0, kind, rack, channel, 1, effect, parameter, 0, values,
        );
        assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT, "{what}");
        assert_send_refusal(&host, COMMAND_REASON_MALFORMED, 0, &room, &fader_room, what);
    }
    for value in [0.5_f32, -1.0, 2.0] {
        stage_send(
            &mut host,
            0,
            COMMAND_ROUTE_MUTE,
            1,
            0,
            [value, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            host.submit_commands(1),
            RESULT_INVALID_ARGUMENT,
            "mute {value}"
        );
        assert_send_refusal(
            &host,
            COMMAND_REASON_DOMAIN,
            0,
            &room,
            &fader_room,
            "mute value",
        );
    }
    stage_send(&mut host, 0, COMMAND_ROUTE_GAIN_DB, 1, (1 << 22) + 1, gain);
    assert_eq!(
        host.submit_commands(1),
        RESULT_INVALID_ARGUMENT,
        "ramp past the bound"
    );
    assert_send_refusal(
        &host,
        COMMAND_REASON_DOMAIN,
        0,
        &room,
        &fader_room,
        "ramp length",
    );
}

/// Issue #1222 gate 7: admitting send records of all three kinds -- several on one send -- and
/// rendering them allocates and frees nothing, after one warm-up round.
///
/// Test value: red if admitting a send record or committing the send mirror allocates on the
/// render-call path.
#[test]
fn send_edits_admit_and_render_without_allocating() {
    let mut host = send_host(&SEND_SEEDS, 16);
    send_render(&mut host, 0);
    for block in 1..3_u64 {
        let gain = if block == 1 { -8.0 } else { 1.0 };
        stage_send(
            &mut host,
            0,
            COMMAND_ROUTE_GAIN_DB,
            3,
            480,
            [gain, 0.0, 0.0, 0.0],
        );
        stage_send(
            &mut host,
            1,
            COMMAND_ROUTE_MATRIX,
            3,
            480,
            [0.3, -0.6, 0.2, 0.7],
        );
        stage_send(
            &mut host,
            2,
            COMMAND_ROUTE_MUTE,
            6,
            480,
            [1.0, 0.0, 0.0, 0.0],
        );
        stage_command(
            &mut host,
            3,
            COMMAND_FADER_DB,
            255,
            2,
            4,
            0,
            0,
            480,
            [-3.0, 0.0, 0.0, 0.0],
        );
        send_feed(&mut host, block);
        let ((admission, render), allocations, deallocations) =
            crate::ffi::live_response_ffi_tests::measured(|| {
                (host.submit_commands(4), host.render_next())
            });
        assert_eq!(
            admission, RESULT_OK,
            "block {block}: the send batch is admitted"
        );
        assert_eq!(render, RESULT_OK, "block {block}");
        if block == 2 {
            assert_eq!(allocations, 0, "admission/render allocated");
            assert_eq!(deallocations, 0, "admission/render freed");
        }
    }
}

/// The report of an independent host-core preparation of `document` at `queue_records`: its
/// `route_control_resources` are `graph::route_control_resources` of the attached producers.
fn send_prepare_report(document: &str, queue_records: u64) -> host_core::HostPrepareReport {
    let parsed = parse_host_session(document).expect("send session parse");
    let compiled = compile_host_model(
        &parsed,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("send session compile");
    let options = strip_options(queue_records, 0, 0);
    let caps = prepare_caps(&compiled, options, options.source_ring_frames, u64::MAX);
    let live_controls = live_control_request(options, STRIP_QUANTUM).expect("live-control request");
    let (engine, _handles) = prepare_host_runtime_with_selected_meters_between_render_calls(
        &compiled,
        &caps,
        &live_controls,
        &[],
    )
    .expect("independent send preparation");
    engine.report
}

/// Issue #1222, from the #1221 verdict's MINOR-1 and the #1222 verdict's MINOR-4: the browser's
/// exact retained budget charges the route lanes it attaches and the send mirror.
///
/// * Between two queue depths of one session, the host's retained bridge bytes move by exactly the
///   lanes' depth-dependent charge.
/// * At one depth, removing one send (`send-g`) moves them by exactly that send's whole share:
///   the document-dependent bridge rows (the document, the ID staging and one decoded-command
///   entry for the follow pass), the compiled session model, the route lanes' whole
///   `route_control_resources.total_bytes` and the mirror's two `LiveRoute` entries (the live
///   values and their shadow), each from an independent preparation of the two sessions.
/// * The exact-retained budget refuses one byte below the total, and a session without sends does
///   not move with depth at all.
///
/// Test value: red if the browser leaves any part of the route lanes (queues, render-side owners,
/// producer table, IDs, activity table) or the mirror and its shadow out of its exact retained
/// report, so a budget that cannot hold them admits.
#[test]
fn the_exact_retained_budget_charges_the_send_lanes() {
    let document = send_document(&SEND_SEEDS);
    let mut twin = parse_session_json(&document).expect("send session parses");
    twin.routes.retain(|route| route.id.as_str() != "send-g");
    let twin = canonical_session_json(&twin).expect("twin session canonicalizes");
    let bridge = |document: &str, queue_records: u64, sends: usize| {
        let host = strip_boot(document, strip_options(queue_records, 0, 0));
        assert_eq!(host.ready.as_ref().expect("ready").routes.len(), sends);
        *host.resources()
    };
    let shallow = send_prepare_report(&document, 8);
    let deep = send_prepare_report(&document, 64);
    let twin_deep = send_prepare_report(&twin, 64);
    let (lanes_8, lanes_64) = (
        shallow.route_control_resources,
        deep.route_control_resources,
    );
    assert_eq!(lanes_8.routes, SEND_ROUTES.len() as u64);
    assert_eq!(
        twin_deep.route_control_resources.routes,
        SEND_ROUTES.len() as u64 - 1
    );
    assert!(
        lanes_64.total_bytes > lanes_8.total_bytes,
        "deeper queues cost more"
    );
    let at_8 = bridge(&document, 8, SEND_ROUTES.len());
    let resources = bridge(&document, 64, SEND_ROUTES.len());
    let without = bridge(&twin, 64, SEND_ROUTES.len() - 1);
    assert_eq!(
        resources.bridge_retained_bytes - at_8.bridge_retained_bytes,
        lanes_64.total_bytes - lanes_8.total_bytes
    );
    assert_eq!(
        resources.bridge_metadata_bytes - at_8.bridge_metadata_bytes,
        lanes_64.total_bytes - lanes_8.total_bytes
    );
    assert!(resources.largest_bridge_allocation_bytes >= lanes_64.largest_allocation_bytes);

    let document_rows = (resources.session_document_bytes - without.session_document_bytes)
        + (resources.id_staging_bytes - without.id_staging_bytes)
        + size_of::<StagedCommand>() as u64;
    let lanes = lanes_64.total_bytes - twin_deep.route_control_resources.total_bytes;
    let model = deep.session_model_bytes - twin_deep.session_model_bytes;
    let mirror = 2 * size_of::<host_core::LiveRoute>() as u64;
    assert_eq!(
        resources.bridge_retained_bytes - without.bridge_retained_bytes,
        document_rows + model + lanes + mirror,
        "one send's whole retained share"
    );
    assert_eq!(
        resources.bridge_metadata_bytes - without.bridge_metadata_bytes,
        size_of::<StagedCommand>() as u64 + model + lanes + mirror,
        "one send's whole metadata share"
    );

    let (_, track_only) = strip_pair_documents(None);
    assert_eq!(
        bridge(&track_only, 8, 0).bridge_retained_bytes,
        bridge(&track_only, 64, 0).bridge_retained_bytes,
        "no send, no lane"
    );

    let exact = exact_retained_report_total(&resources);
    let admitted = AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: exact,
            ..strip_options(64, 0, 0)
        },
    );
    assert!(admitted.is_ok(), "the exact retained total is admitted");
    let refused = AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: exact - 1,
            ..strip_options(64, 0, 0)
        },
    )
    .err()
    .expect("one byte below the exact retained total refuses");
    assert_eq!(refused.result(), RESULT_REFUSED_BUDGET);
    assert!(
        refused
            .diagnostic()
            .starts_with(b"host.budget.retained_exact\t"),
        "the exact aggregate gate refuses: {}",
        String::from_utf8_lossy(refused.diagnostic())
    );
}

// Issue #1224: a send with `follows_mute` follows its source strip's effective mute live. Every
// session here feeds each track its own two-lane signal (`strip_planes`), every send matrix is
// asymmetric, and every bus is a unity submix (no console slot, no insert, input section
// identity), so a comparison at the output is exact.

/// One #1224 route: `(id, source strip, the source is a submix, tap, destination bus or `None`
/// for the output, matrix, follows_mute)`.
type FollowRoute<'a> = (
    &'a str,
    &'a str,
    bool,
    session::SendTap,
    Option<&'a str>,
    [f32; 4],
    bool,
);

const FOLLOW_UNITY: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

/// A strip-to-output route at unity from the post-pan tap.
const fn follow_main<'a>(id: &'a str, source: &'a str, bus: bool) -> FollowRoute<'a> {
    (
        id,
        source,
        bus,
        session::SendTap::PostPan,
        None,
        FOLLOW_UNITY,
        false,
    )
}

/// Tracks `tracks` (track `tracks[i]` reads its own source, fed `strip_planes(i, _)`), unity
/// submixes `buses`, the routes `routes`, the fader lane mutes `muted` by strip ID, and a
/// true-peak limiter insert (default parameters) on track `limited`.
fn follow_document(
    tracks: &[&str],
    buses: &[&str],
    routes: &[FollowRoute<'_>],
    muted: &[(&str, [bool; 2])],
    limited: Option<&str>,
) -> String {
    let (mut model, source, track, mut limiter, template) = strip_base();
    let output = template.destination.clone();
    for id in tracks {
        strip_add_track(&mut model, &source, &track, id);
    }
    model.submixes = buses
        .iter()
        .map(|id| session::Submix::unity(strip_id(id), &model.console))
        .collect();
    limiter.id = strip_id("ceiling");
    limiter.identity = session::EffectIdentity::Native {
        effect_id: strip_id("miso.true-peak-limiter"),
    };
    limiter.params = Vec::new();
    for track in &mut model.tracks {
        if limited == Some(track.id.as_str()) {
            track.inserts.effects = vec![limiter.clone()];
        }
    }
    for (id, lanes) in muted {
        let fader = match model.tracks.iter_mut().find(|t| t.id.as_str() == *id) {
            Some(track) => &mut track.fader,
            None => {
                &mut model
                    .submixes
                    .iter_mut()
                    .find(|s| s.id.as_str() == *id)
                    .expect("muted strip")
                    .fader
            }
        };
        (fader.left_mute, fader.right_mute) = (lanes[0], lanes[1]);
    }
    for (id, from, bus, tap, into, matrix, follows) in routes {
        let source = if *bus {
            session::RouteSource::Submix {
                submix_id: strip_id(from),
                tap: *tap,
            }
        } else {
            session::RouteSource::Track {
                track_id: strip_id(from),
                tap: *tap,
            }
        };
        let destination = into.map_or_else(|| output.clone(), strip_into);
        let mut route = strip_route(&template, id, source, destination, *matrix);
        route.follows_mute = *follows;
        model.routes.push(route);
    }
    canonical_session_json(&model).expect("#1224 session canonicalizes")
}

/// Feed `tracks[i]` `strip_planes(i, block)` (or zeros for track `silent`), render, and return
/// the output.
fn follow_render(
    host: &mut AudioWorkletEngineHost,
    tracks: &[&str],
    block: u64,
    silent: Option<&str>,
) -> Vec<f32> {
    for (feed, id) in tracks.iter().enumerate() {
        let planes = if silent == Some(*id) {
            let zeros = vec![0.0_f32; STRIP_QUANTUM as usize];
            [zeros.clone(), zeros]
        } else {
            strip_planes(feed as u64, block)
        };
        submit_strip_source(host, id, block, &planes);
    }
    assert_eq!(host.render_next(), RESULT_OK, "block {block}");
    host.output_pcm().expect("output").to_vec()
}

/// Render `blocks` blocks from `first` on every host in lockstep, requiring every host's output
/// bit-identical to the first's when `checked`. Returns whether any compared block carried signal.
fn follow_lockstep(
    hosts: &mut [&mut AudioWorkletEngineHost],
    tracks: &[&str],
    first: u64,
    blocks: u64,
    checked: bool,
    what: &str,
) -> bool {
    let mut audible = false;
    for block in first..first + blocks {
        let outputs: Vec<Vec<f32>> = hosts
            .iter_mut()
            .map(|host| follow_render(host, tracks, block, None))
            .collect();
        if checked {
            for (other, output) in outputs.iter().enumerate().skip(1) {
                for (sample, (x, y)) in outputs[0].iter().zip(output).enumerate() {
                    assert_eq!(
                        x.to_bits(),
                        y.to_bits(),
                        "{what}: host 0 vs {other}, block {block} sample {sample}: {x} vs {y}"
                    );
                }
            }
            audible |= outputs[0].iter().any(|sample| *sample != 0.0);
        }
    }
    audible
}

/// Stage one kind 4 record on `channel` (0 left, 1 right, 2 both).
fn stage_lane_mute(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    strip: u32,
    channel: u8,
    on: bool,
    smoothing: u32,
) {
    let value = if on { 1.0 } else { 0.0 };
    stage_command(
        host,
        index,
        COMMAND_MUTE,
        255,
        channel,
        strip,
        0,
        0,
        smoothing,
        [value, 0.0, 0.0, 0.0],
    );
}

/// Every live send's follow-zeroed source lanes.
fn follow_lanes(host: &AudioWorkletEngineHost) -> Vec<[bool; 2]> {
    let routes = &host.ready.as_ref().expect("ready").routes;
    (0..routes.len())
        .map(|route| routes.get(route).expect("live route").source_lane_muted)
        .collect()
}

/// Every fader queue's free room, by strip.
fn follow_fader_room(host: &AudioWorkletEngineHost) -> Vec<usize> {
    host.ready
        .as_ref()
        .expect("ready")
        .controls
        .iter()
        .map(|controls| controls.fader.available_capacity())
        .collect()
}

/// Gate 1's tracks, in feed order. Canonical strip order: `bass` 0, `drums` 1, `vocal` 2, then
/// the submixes `room` 3 and `verb` 4. Live sends, in route-ID order: `bass-room` 0,
/// `drums-verb` 1.
const SOLO_FOLLOW_TRACKS: [&str; 3] = ["drums", "bass", "vocal"];
const SOLO_FOLLOW_DRUMS: u32 = 1;
const SOLO_FOLLOW_VOCAL: u32 = 2;
const SOLO_FOLLOW_ROUTES: [FollowRoute<'static>; 7] = [
    (
        "drums-verb",
        "drums",
        false,
        session::SendTap::PreFader,
        Some("verb"),
        [0.8, -0.3, 0.25, 0.6],
        true,
    ),
    (
        "bass-room",
        "bass",
        false,
        session::SendTap::PreFader,
        Some("room"),
        [-0.5, 0.7, 0.9, -0.2],
        true,
    ),
    follow_main("drums-main", "drums", false),
    follow_main("bass-main", "bass", false),
    follow_main("vocal-main", "vocal", false),
    follow_main("verb-main", "verb", true),
    follow_main("room-main", "room", true),
];

fn solo_follow_host(muted: &[(&str, [bool; 2])], queue_records: u64) -> AudioWorkletEngineHost {
    strip_boot(
        &follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            muted,
            None,
        ),
        strip_options(queue_records, 0, 0),
    )
}

/// Issue #1224 gate 1: soloing `vocal` silences `drums`' and `bass`' pre-fader `follows_mute`
/// sends into `verb` and `room` with their faders, bit-identically to a host booted with `drums`
/// and `bass` muted (where #1218 prepares both sends silenced), once the ramps settle; un-soloing
/// restores both, bit-identically to an unedited host.
///
/// Test value: red if solo composition ignores routes (the measured leak: the pre-fader sends stay
/// open under solo), reads the wrong strip or lane, or leaves a residue after the ramp. It checks
/// settled blocks only: the solo state is final when the batch loop ends, so running the follow
/// pass before the coalescing pass is red here only through its `malformed` guard, and
/// `a_solo_follow_ramps_at_the_solo_window` is the audio gate for that ordering (#1224 verdict
/// MINOR-3).
#[test]
fn soloing_a_track_silences_the_followed_pre_fader_sends_of_the_rest() {
    let mut live = solo_follow_host(&[], 16);
    let mut muted = solo_follow_host(&[("drums", [true; 2]), ("bass", [true; 2])], 16);
    let mut open = solo_follow_host(&[], 16);
    follow_lockstep(
        &mut [&mut live, &mut muted, &mut open],
        &SOLO_FOLLOW_TRACKS,
        0,
        2,
        false,
        "warm-up",
    );
    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "solo vocal");
    assert_eq!(
        follow_lanes(&live),
        [[true; 2], [true; 2]],
        "both sends follow"
    );
    // 480 samples settle within four 128-frame blocks.
    follow_lockstep(
        &mut [&mut live, &mut muted],
        &SOLO_FOLLOW_TRACKS,
        2,
        4,
        false,
        "solo ramp",
    );
    follow_render(&mut open, &SOLO_FOLLOW_TRACKS, 2, None);
    for block in 3..6 {
        follow_render(&mut open, &SOLO_FOLLOW_TRACKS, block, None);
    }
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut muted],
            &SOLO_FOLLOW_TRACKS,
            6,
            3,
            true,
            "soloed vs booted muted"
        ),
        "vocal stays audible"
    );
    for block in 6..9 {
        follow_render(&mut open, &SOLO_FOLLOW_TRACKS, block, None);
    }
    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, false, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "un-solo vocal");
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [false; 2]],
        "both sends reopen"
    );
    follow_lockstep(
        &mut [&mut live, &mut open],
        &SOLO_FOLLOW_TRACKS,
        9,
        4,
        false,
        "un-solo ramp",
    );
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut open],
            &SOLO_FOLLOW_TRACKS,
            13,
            3,
            true,
            "un-soloed vs unedited"
        ),
        "the mix is audible"
    );
}

/// Issue #1224 gate 2: no redundant send record.
///
/// * Soloing `drums` (a follow source) moves only `bass-room`; soloing `vocal` on top, which is
///   no `follows_mute` source and changes no follow source's effective mute, pushes no send
///   record.
/// * Muting `drums` pushes one record on `drums-verb`; muting it again pushes none (its strip
///   record is staged, as kind 4 always is).
/// * A redundant solo toggle -- `vocal` on again, and a batch turning it off and back on -- pushes
///   no send record and leaves the output bit-identical to a twin that received neither.
///
/// Test value: red if `delta` re-emits an unchanged target (a settled send's ramp restarts, which
/// is digest visible) or the follow pass emits for a strip whose effective mute did not move.
#[test]
fn a_follow_never_pushes_a_redundant_send_record() {
    let mut live = solo_follow_host(&[], 16);
    let mut twin = solo_follow_host(&[], 16);
    follow_lockstep(
        &mut [&mut live, &mut twin],
        &SOLO_FOLLOW_TRACKS,
        0,
        1,
        false,
        "warm-up",
    );
    let full = vec![16_usize; 2];

    for host in [&mut live, &mut twin] {
        stage_solo(host, 0, SOLO_FOLLOW_DRUMS, true, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "solo drums");
        assert_eq!(send_queue_room(host), [15, 16], "only bass-room follows");
        stage_solo(host, 0, SOLO_FOLLOW_VOCAL, true, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "solo vocal too");
        assert_eq!(send_queue_room(host), [15, 16], "no send record");
    }
    follow_lockstep(
        &mut [&mut live, &mut twin],
        &SOLO_FOLLOW_TRACKS,
        1,
        1,
        false,
        "drain",
    );
    assert_eq!(send_queue_room(&live), full);

    for host in [&mut live, &mut twin] {
        stage_mute(host, 0, SOLO_FOLLOW_DRUMS, true, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "mute drums");
        assert_eq!(send_queue_room(host), [16, 15], "drums-verb follows");
        assert_eq!(follow_lanes(host), [[true; 2], [true; 2]]);
        stage_mute(host, 0, SOLO_FOLLOW_DRUMS, true, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "mute drums again");
        assert_eq!(send_queue_room(host), [16, 15], "no second send record");
    }
    follow_lockstep(
        &mut [&mut live, &mut twin],
        &SOLO_FOLLOW_TRACKS,
        2,
        2,
        true,
        "settled",
    );

    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "vocal on again");
    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, false, 480);
    stage_solo(&mut live, 1, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(live.submit_commands(2), RESULT_OK, "vocal off and on");
    assert_eq!(send_queue_room(&live), full, "no send record");
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut twin],
            &SOLO_FOLLOW_TRACKS,
            4,
            4,
            true,
            "redundant toggles vs none"
        ),
        "vocal is audible"
    );
}

/// Issue #1224 gate 3: a submission whose follow records would overfill a send queue is typed
/// backpressure at the strip mute that asked for them, and nothing moves: no strip mute record
/// and no send record is pushed, and the solo state and the send mirror are unchanged and closed.
/// Once the queue drains, a strip mute whose window is longer than any send ramp is refused
/// `domain` the same way, and then the solo is admitted.
///
/// Test value: red if strip mutes commit while their follows do not -- a follow record pushed
/// before the room check, a send queue left out of it, a mirror committed on refusal, or a follow
/// ramp silently clamped away from its strip's window.
#[test]
fn a_full_send_queue_refuses_the_strip_mutes_it_follows() {
    const DEPTH: u64 = 4;
    let mut host = solo_follow_host(&[], DEPTH);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    for index in 0..DEPTH as usize {
        stage_send(
            &mut host,
            index,
            COMMAND_ROUTE_GAIN_DB,
            1,
            0,
            [-1.0, 0.0, 0.0, 0.0],
        );
    }
    assert_eq!(
        host.submit_commands(DEPTH as u32),
        RESULT_OK,
        "fill drums-verb"
    );
    let room = send_queue_room(&host);
    assert_eq!(room, [DEPTH as usize, 0]);
    let fader_room = follow_fader_room(&host);
    let lanes = follow_lanes(&host);
    let mirror = send_mirror(&host);

    let refused = |host: &AudioWorkletEngineHost, index: u32, what: &str| {
        let report = *host.command_report();
        assert_eq!(report.reason, COMMAND_REASON_BACKPRESSURE, "{what}: reason");
        assert_eq!(report.rejected_index, index, "{what}: wire index");
        assert_eq!(report.admitted, 0, "{what}: admitted");
        assert_eq!(send_queue_room(host), room, "{what}: send queues");
        assert_eq!(follow_fader_room(host), fader_room, "{what}: fader queues");
        assert_eq!(follow_lanes(host), lanes, "{what}: follow lanes");
        assert_eq!(send_mirror(host), mirror, "{what}: send mirror");
        let ready = host.ready.as_ref().expect("ready");
        assert!(
            !ready.routes.transaction_open(),
            "{what}: send mirror closed"
        );
        assert!(!ready.solo.transaction_open(), "{what}: solo state closed");
        assert!(!ready.solo.any_solo(), "{what}: no solo engaged");
        assert!(
            !ready.solo.user_mute(SOLO_FOLLOW_DRUMS as usize, 0),
            "{what}: drums"
        );
    };

    stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(host.submit_commands(1), RESULT_BACKPRESSURE, "solo vocal");
    refused(&host, 0, "solo");

    stage_command(
        &mut host,
        0,
        COMMAND_FADER_DB,
        255,
        2,
        SOLO_FOLLOW_VOCAL,
        0,
        0,
        0,
        [-3.0, 0.0, 0.0, 0.0],
    );
    stage_mute(&mut host, 1, SOLO_FOLLOW_DRUMS, true, 480);
    assert_eq!(host.submit_commands(2), RESULT_BACKPRESSURE, "mute drums");
    refused(&host, 1, "kind 4");

    // A strip mute whose window no send ramp can take (past `ROUTE_RAMP_LENGTH_MAXIMUM`, `2^22`)
    // is refused `domain` at its wire index once a send follows it, rather than clamped.
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 1, None);
    let drained = send_queue_room(&host);
    assert_eq!(drained, [DEPTH as usize; 2]);
    stage_mute(&mut host, 0, SOLO_FOLLOW_DRUMS, true, (1 << 22) + 1);
    assert_eq!(host.submit_commands(1), RESULT_INVALID_ARGUMENT, "overlong");
    let report = *host.command_report();
    assert_eq!(
        (report.reason, report.rejected_index, report.admitted),
        (COMMAND_REASON_DOMAIN, 0, 0)
    );
    assert_eq!(send_queue_room(&host), drained, "overlong: send queues");
    assert_eq!(
        follow_fader_room(&host),
        fader_room,
        "overlong: fader queues"
    );
    assert_eq!(follow_lanes(&host), lanes, "overlong: follow lanes");
    assert!(!host.ready.as_ref().expect("ready").solo.user_mute(1, 0));

    stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(host.submit_commands(1), RESULT_OK, "drained: admitted");
    assert_eq!(follow_lanes(&host), [[true; 2], [true; 2]]);
}

/// Gate 4's tracks, in feed order. Strips: `drums` 0, `vocal` 1, `verb` 2; one live send,
/// `drums-verb`.
const LANE_FOLLOW_TRACKS: [&str; 2] = ["drums", "vocal"];
const LANE_FOLLOW_ROUTES: [FollowRoute<'static>; 4] = [
    (
        "drums-verb",
        "drums",
        false,
        session::SendTap::PreFader,
        Some("verb"),
        [0.8, -0.3, 0.25, 0.6],
        true,
    ),
    follow_main("drums-main", "drums", false),
    follow_main("vocal-main", "vocal", false),
    follow_main("verb-main", "verb", true),
];

fn lane_follow_host(drums: [bool; 2]) -> AudioWorkletEngineHost {
    strip_boot(
        &follow_document(
            &LANE_FOLLOW_TRACKS,
            &["verb"],
            &LANE_FOLLOW_ROUTES,
            &[("drums", drums)],
            None,
        ),
        strip_options(16, 0, 0),
    )
}

/// Issue #1224 gate 4: per lane. From every lane-mute state of `drums` to every other, kind 4
/// records on the lanes that change (one `both` record when both move together) land on the
/// output of a host booted in the target state, whose prepared send has the same source columns
/// zeroed. Then, over a left-only mute, soloing `vocal` follows both lanes, and un-soloing returns
/// the send to the left-only state.
///
/// Test value: red if the follow path maps a source lane to the other column, zeroes both columns
/// for a one-lane mute, reads one lane for both, or loses a user's lane mute across a solo.
#[test]
fn a_one_lane_mute_follows_into_its_own_source_column() {
    let states = [[false, false], [true, false], [false, true], [true, true]];
    for from in states {
        for to in states {
            if from == to {
                continue;
            }
            let what = format!("{from:?} -> {to:?}");
            let mut live = lane_follow_host(from);
            let mut booted = lane_follow_host(to);
            let mut staged = 0;
            if from[0] != to[0] && from[1] != to[1] && to[0] == to[1] {
                stage_lane_mute(&mut live, 0, 0, 2, to[0], 480);
                staged = 1;
            } else {
                for lane in 0..2 {
                    if from[lane] != to[lane] {
                        stage_lane_mute(&mut live, staged, 0, lane as u8, to[lane], 480);
                        staged += 1;
                    }
                }
            }
            assert_eq!(live.submit_commands(staged as u32), RESULT_OK, "{what}");
            assert_eq!(follow_lanes(&live), [to], "{what}: the mirror");
            follow_lockstep(
                &mut [&mut live, &mut booted],
                &LANE_FOLLOW_TRACKS,
                0,
                4,
                false,
                &what,
            );
            assert!(
                follow_lockstep(
                    &mut [&mut live, &mut booted],
                    &LANE_FOLLOW_TRACKS,
                    4,
                    3,
                    true,
                    &what
                ),
                "{what}: audible"
            );
        }
    }

    let mut live = lane_follow_host([true, false]);
    let mut soloed = lane_follow_host([true, true]);
    let mut left = lane_follow_host([true, false]);
    stage_solo(&mut live, 0, 1, true, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "solo vocal");
    assert_eq!(follow_lanes(&live), [[true, true]]);
    follow_lockstep(
        &mut [&mut live, &mut soloed],
        &LANE_FOLLOW_TRACKS,
        0,
        4,
        false,
        "solo",
    );
    follow_lockstep(&mut [&mut left], &LANE_FOLLOW_TRACKS, 0, 4, false, "solo");
    follow_lockstep(
        &mut [&mut live, &mut soloed],
        &LANE_FOLLOW_TRACKS,
        4,
        3,
        true,
        "soloed",
    );
    follow_lockstep(&mut [&mut left], &LANE_FOLLOW_TRACKS, 4, 3, false, "soloed");
    stage_solo(&mut live, 0, 1, false, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "un-solo vocal");
    assert_eq!(
        follow_lanes(&live),
        [[true, false]],
        "the user's left mute stays"
    );
    follow_lockstep(
        &mut [&mut live, &mut left],
        &LANE_FOLLOW_TRACKS,
        7,
        4,
        false,
        "un-solo",
    );
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut left],
            &LANE_FOLLOW_TRACKS,
            11,
            3,
            true,
            "un-soloed"
        ),
        "audible"
    );
}

/// Gate 5's tracks, in feed order. Strips: `t0` 0, `t1` 1, then the submixes `drums` 2 and
/// `verb` 3. Live sends, in route-ID order: `drums-verb` 0, `t0-drums` 1, `t1-drums` 2. `t1` also
/// reaches the output directly, so a fully muted bus leaves the output audible.
const BUS_FOLLOW_TRACKS: [&str; 2] = ["t0", "t1"];
const BUS_FOLLOW_DRUMS: u32 = 2;
const BUS_FOLLOW_ROUTES: [FollowRoute<'static>; 6] = [
    (
        "t0-drums",
        "t0",
        false,
        session::SendTap::PostPan,
        Some("drums"),
        [0.35, 0.15, -0.65, 0.45],
        false,
    ),
    (
        "t1-drums",
        "t1",
        false,
        session::SendTap::PostPan,
        Some("drums"),
        [0.6, 0.4, -0.3, 0.9],
        false,
    ),
    (
        "drums-verb",
        "drums",
        true,
        session::SendTap::PreFader,
        Some("verb"),
        [0.8, -0.3, 0.25, 0.6],
        true,
    ),
    follow_main("drums-main", "drums", true),
    follow_main("verb-main", "verb", true),
    follow_main("t1-main", "t1", false),
];

fn bus_follow_host(drums: [bool; 2]) -> AudioWorkletEngineHost {
    strip_boot(
        &follow_document(
            &BUS_FOLLOW_TRACKS,
            &["drums", "verb"],
            &BUS_FOLLOW_ROUTES,
            &[("drums", drums)],
            None,
        ),
        strip_options(16, 0, 0),
    )
}

/// Issue #1224 gate 5: a bus as the source. Kind 4 muting bus `drums` (both lanes, then its right
/// lane alone) silences its pre-fader `follows_mute` send into `verb` and un-muting restores it,
/// each matching a freshly booted host with the bus in that state. Soloing `t0` leaves the bus,
/// which is solo-safe, and its send alone: no send record.
///
/// Test value: red if follow composition reads only tracks' mutes (a bus mute leaves its sends
/// open), resolves a bus source without the track offset, or solo-mutes a bus's sends.
#[test]
fn muting_a_bus_silences_its_followed_send() {
    for (channel, lanes) in [(2_u8, [true, true]), (1, [false, true])] {
        let what = format!("bus lanes {lanes:?}");
        let mut live = bus_follow_host([false; 2]);
        let mut muted = bus_follow_host(lanes);
        let mut open = bus_follow_host([false; 2]);
        stage_lane_mute(&mut live, 0, BUS_FOLLOW_DRUMS, channel, true, 480);
        assert_eq!(live.submit_commands(1), RESULT_OK, "{what}: mute");
        assert_eq!(follow_lanes(&live)[0], lanes, "{what}: drums-verb follows");
        follow_lockstep(
            &mut [&mut live, &mut muted],
            &BUS_FOLLOW_TRACKS,
            0,
            4,
            false,
            &what,
        );
        follow_lockstep(&mut [&mut open], &BUS_FOLLOW_TRACKS, 0, 4, false, &what);
        assert!(
            follow_lockstep(
                &mut [&mut live, &mut muted],
                &BUS_FOLLOW_TRACKS,
                4,
                3,
                true,
                &what
            ),
            "{what}: audible"
        );
        follow_lockstep(&mut [&mut open], &BUS_FOLLOW_TRACKS, 4, 3, false, &what);
        stage_lane_mute(&mut live, 0, BUS_FOLLOW_DRUMS, channel, false, 480);
        assert_eq!(live.submit_commands(1), RESULT_OK, "{what}: un-mute");
        assert_eq!(follow_lanes(&live)[0], [false; 2], "{what}: reopened");
        follow_lockstep(
            &mut [&mut live, &mut open],
            &BUS_FOLLOW_TRACKS,
            7,
            4,
            false,
            &what,
        );
        assert!(
            follow_lockstep(
                &mut [&mut live, &mut open],
                &BUS_FOLLOW_TRACKS,
                11,
                3,
                true,
                &what
            ),
            "{what}: audible"
        );
    }

    let mut host = bus_follow_host([false; 2]);
    follow_render(&mut host, &BUS_FOLLOW_TRACKS, 0, None);
    stage_solo(&mut host, 0, 0, true, 480);
    assert_eq!(host.submit_commands(1), RESULT_OK, "solo t0");
    assert_eq!(send_queue_room(&host), [16; 3], "no send record");
    assert_eq!(follow_lanes(&host)[0], [false; 2]);
}

/// Gate 6's tracks, in feed order. Strips: `drums` 0, `s` 1, then `verb` 2. `s` carries a
/// true-peak limiter (486 samples at 48 kHz) into `verb`, so PDC delays `drums-verb`, `drums`'
/// only path, by 486 samples. One live send per host is `drums-verb` (route-ID order:
/// `drums-verb` 0, `s-verb` 1).
const DELAYED_FOLLOW_TRACKS: [&str; 2] = ["drums", "s"];
const DELAYED_FOLLOW_LATENCY: usize = 486;

fn delayed_follow_document(follows: bool) -> String {
    follow_document(
        &DELAYED_FOLLOW_TRACKS,
        &["verb"],
        &[
            (
                "drums-verb",
                "drums",
                false,
                session::SendTap::PreFader,
                Some("verb"),
                [0.8, -0.3, 0.25, 0.6],
                follows,
            ),
            (
                "s-verb",
                "s",
                false,
                session::SendTap::PostPan,
                Some("verb"),
                [0.35, 0.15, -0.65, 0.45],
                false,
            ),
            follow_main("verb-main", "verb", true),
        ],
        &[],
        Some("s"),
    )
}

/// Issue #1224 gate 6: a delayed follow-muted send. `drums-verb` carries the 486-sample
/// compensation (asserted from the plan: fed `drums` alone, the output is exactly zero for 486
/// frames and carries `drums` from frame 486). Host A follows: it mutes and un-mutes `drums` with
/// kind 4. Host B has the same session with `follows_mute` false and sends the same kind 4 records
/// plus kind 14 on the send. The mute is one record at 480 samples; the un-mute is two kind 4
/// records, at 200 and then 480 samples, so the follow takes the last one's ramp. Both hosts
/// render the same bits on every block, including the 486 samples after each edit.
///
/// Test value: red if the follow path deactivates a delayed send, re-sends a stale target, or uses
/// a ramp length other than the last strip mute's (the first, or none).
#[test]
fn a_delayed_send_follows_like_an_explicit_send_mute() {
    let probe_document = delayed_follow_document(true);
    let mut probe = strip_boot(&probe_document, strip_options(16, 0, 0));
    assert!(
        probe.resources().graph_delay_bytes > 0,
        "the plan carries a delay line"
    );
    let mut probed = Vec::new();
    for block in 0..5 {
        let output = follow_render(&mut probe, &DELAYED_FOLLOW_TRACKS, block, Some("s"));
        let frames = STRIP_QUANTUM as usize;
        probed.extend((0..frames).map(|frame| (output[frame], output[frames + frame])));
    }
    let first = probed
        .iter()
        .position(|(left, right)| *left != 0.0 || *right != 0.0);
    assert_eq!(
        first,
        Some(DELAYED_FOLLOW_LATENCY),
        "drums reaches verb 486 samples late"
    );

    let mut follow = strip_boot(&probe_document, strip_options(16, 0, 0));
    let mut explicit = strip_boot(&delayed_follow_document(false), strip_options(16, 0, 0));
    follow_lockstep(
        &mut [&mut follow, &mut explicit],
        &DELAYED_FOLLOW_TRACKS,
        0,
        2,
        true,
        "idle",
    );

    stage_lane_mute(&mut follow, 0, 0, 2, true, 480);
    stage_lane_mute(&mut explicit, 0, 0, 2, true, 480);
    stage_send(
        &mut explicit,
        1,
        COMMAND_ROUTE_MUTE,
        0,
        480,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(follow.submit_commands(1), RESULT_OK, "A: mute drums");
    assert_eq!(
        explicit.submit_commands(2),
        RESULT_OK,
        "B: mute drums and its send"
    );
    assert_eq!(follow_lanes(&follow)[0], [true; 2]);
    follow_lockstep(
        &mut [&mut follow, &mut explicit],
        &DELAYED_FOLLOW_TRACKS,
        2,
        8,
        true,
        "muted",
    );

    for host in [&mut follow, &mut explicit] {
        stage_lane_mute(host, 0, 0, 2, false, 200);
        stage_lane_mute(host, 1, 0, 2, false, 480);
    }
    stage_send(&mut explicit, 2, COMMAND_ROUTE_MUTE, 0, 480, [0.0; 4]);
    assert_eq!(follow.submit_commands(2), RESULT_OK, "A: un-mute drums");
    assert_eq!(
        explicit.submit_commands(3),
        RESULT_OK,
        "B: un-mute drums and its send"
    );
    assert_eq!(follow_lanes(&follow)[0], [false; 2]);
    assert!(
        follow_lockstep(
            &mut [&mut follow, &mut explicit],
            &DELAYED_FOLLOW_TRACKS,
            10,
            8,
            true,
            "un-muted"
        ),
        "audible"
    );
}

/// Issue #1224 gate 9: admitting solo toggles and kind 4 mutes that stage follow records, then
/// rendering, allocates and frees nothing.
///
/// Test value: red if the follow pass or the grown staging allocates on the admission or render
/// path.
#[test]
fn follow_records_admit_and_render_without_allocating() {
    let mut host = solo_follow_host(&[], 64);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    // Warm every path once outside the measurement.
    stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, true, 480);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 1, None);
    stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, false, 480);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    for (feed, id) in SOLO_FOLLOW_TRACKS.iter().enumerate() {
        submit_strip_source(&mut host, id, 2, &strip_planes(feed as u64, 2));
    }
    stage_solo(&mut host, 0, SOLO_FOLLOW_VOCAL, true, 480);
    stage_lane_mute(&mut host, 1, SOLO_FOLLOW_DRUMS, 0, true, 128);
    stage_solo(&mut host, 2, SOLO_FOLLOW_VOCAL, false, 480);
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            (host.submit_commands(3), host.render_next())
        });
    assert_eq!(admission, RESULT_OK, "the follow batch is admitted");
    assert_eq!(render, RESULT_OK);
    assert_eq!(follow_lanes(&host), [[false; 2], [true, false]]);
    assert_eq!(allocations, 0, "admission/render allocated");
    assert_eq!(deallocations, 0, "admission/render freed");
}

/// Issue #1224 D4: the decode staging holds a full batch, a solo transition and every follow
/// record it owes. Six tracks, each with a compressor insert and a pre-fader `follows_mute` send
/// into each of six buses (36 live sends): 255 `channel = both` threshold records (510 entries)
/// and one solo of `t0` (five coalesced strip records, then 30 follow records) need 545 entries,
/// past the 536 that `2 * MAXIMUM_COMMAND_RECORDS + 2 * strip_count` holds.
///
/// Test value: red if the staging is not grown by the live-send count, so a legal batch whose
/// solo moves many followed sends is refused `malformed` by the staging bound.
#[test]
fn the_decode_staging_holds_a_full_batch_and_its_follow_records() {
    const TRACKS: [&str; 6] = ["t0", "t1", "t2", "t3", "t4", "t5"];
    const BUSES: [&str; 6] = ["b0", "b1", "b2", "b3", "b4", "b5"];
    let ids: Vec<(String, &str, &str)> = TRACKS
        .iter()
        .flat_map(|track| {
            BUSES
                .iter()
                .map(move |bus| (format!("{track}-{bus}"), *track, *bus))
        })
        .collect();
    let routes: Vec<FollowRoute<'_>> = ids
        .iter()
        .map(|(id, track, bus)| {
            (
                id.as_str(),
                *track,
                false,
                session::SendTap::PreFader,
                Some(*bus),
                [0.8, -0.3, 0.25, 0.6],
                true,
            )
        })
        .collect();
    let (_, _, _, compressor, _) = strip_base();
    let document = follow_document(&TRACKS, &BUSES, &routes, &[], None);
    let mut model = parse_session_json(&document).expect("staging session");
    for track in &mut model.tracks {
        track.inserts.effects = vec![compressor.clone()];
    }
    let document = canonical_session_json(&model).expect("staging session canonicalizes");
    let mut host = strip_boot(&document, strip_options(128, 0, 0));
    let strips = TRACKS.len() + BUSES.len();
    assert_eq!(
        host.command_staging_entries(),
        Some(MAXIMUM_COMMAND_RECORDS as usize * 2 + strips * 2 + routes.len()),
    );
    follow_render(&mut host, &TRACKS, 0, None);
    let records = MAXIMUM_COMMAND_RECORDS as usize;
    for index in 0..records - 1 {
        stage_command(
            &mut host,
            index,
            COMMAND_EFFECT_PARAM,
            1,
            2,
            (index % TRACKS.len()) as u32,
            0,
            1,
            0,
            [-12.0, 0.0, 0.0, 0.0],
        );
    }
    stage_solo(&mut host, records - 1, 0, true, 480);
    assert_eq!(
        host.submit_commands(MAXIMUM_COMMAND_RECORDS),
        RESULT_OK,
        "reason {}",
        host.command_report().reason,
    );
    assert_eq!(host.command_report().admitted, MAXIMUM_COMMAND_RECORDS);
    let lanes = follow_lanes(&host);
    for (route, lanes) in lanes.iter().enumerate() {
        let muted = route >= BUSES.len();
        assert_eq!(*lanes, [muted; 2], "live send {route}");
    }
}

// ---- #1224 verdict MINOR-1 and MINOR-3: the verifier's probes P1-P4, adopted ----

/// The first sample at which two outputs differ in bits, with both values.
fn first_bit_difference(a: &[f32], b: &[f32]) -> Option<(usize, f32, f32)> {
    a.iter()
        .zip(b)
        .enumerate()
        .find(|(_, (x, y))| x.to_bits() != y.to_bits())
        .map(|(i, (x, y))| (i, *x, *y))
}

/// The solo-follow session with `follows_mute` kept or cleared on every send.
fn solo_follow_host_with_follows(follows: bool) -> AudioWorkletEngineHost {
    let routes: Vec<FollowRoute<'static>> = SOLO_FOLLOW_ROUTES
        .iter()
        .map(|r| (r.0, r.1, r.2, r.3, r.4, r.5, r.6 && follows))
        .collect();
    strip_boot(
        &follow_document(&SOLO_FOLLOW_TRACKS, &["verb", "room"], &routes, &[], None),
        strip_options(16, 0, 0),
    )
}

/// Issue #1224 D1 (verdict probe P1): a follow record carries the mirror's live gain, matrix and
/// mute, not the prepared route's. Kinds 13-15 edit `drums-verb`'s gain and matrix and mute
/// `bass-room`; one-lane source mutes then move both sends. Once settled, the output is
/// bit-identical to a host booted with the edited values and the mutes.
///
/// Test value: red if a follow record drops the send's own `mute` (a user-muted send reopens its
/// column on a one-lane source mute) or uses the prepared gain or matrix instead of the mirror's;
/// no other test follows a send whose values were edited live.
#[test]
fn a_follow_record_carries_the_mirrors_live_values() {
    // strips: bass 0, drums 1, vocal 2, room 3, verb 4; sends: bass-room 0, drums-verb 1.
    let edited = [0.45_f32, -0.65, 0.15, 0.9];
    let boot = |muted: &[(&str, [bool; 2])], values: bool| {
        let document = follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            muted,
            None,
        );
        let mut model = parse_session_json(&document).expect("model");
        if values {
            for route in &mut model.routes {
                match route.id.as_str() {
                    "drums-verb" => {
                        route.gain_db = -6.0;
                        route.channel_matrix = session::ChannelMatrix {
                            ll: edited[0],
                            lr: edited[1],
                            rl: edited[2],
                            rr: edited[3],
                        };
                    }
                    "bass-room" => route.mute = true,
                    _ => {}
                }
            }
        }
        strip_boot(
            &canonical_session_json(&model).expect("canonical"),
            strip_options(16, 0, 0),
        )
    };
    let mut live = boot(&[], false);
    stage_send(
        &mut live,
        0,
        COMMAND_ROUTE_GAIN_DB,
        1,
        0,
        [-6.0, 0.0, 0.0, 0.0],
    );
    stage_send(&mut live, 1, COMMAND_ROUTE_MATRIX, 1, 0, edited);
    stage_send(&mut live, 2, COMMAND_ROUTE_MUTE, 0, 0, [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(live.submit_commands(3), RESULT_OK);
    // drums left only; bass right only (bass-room is user-muted: the right lane must not reopen it).
    stage_lane_mute(&mut live, 0, 1, 0, true, 480);
    stage_lane_mute(&mut live, 1, 0, 1, true, 480);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    assert_eq!(follow_lanes(&live), [[false, true], [true, false]]);
    let mut booted = boot(&[("drums", [true, false]), ("bass", [false, true])], true);
    for block in 0..5 {
        follow_render(&mut live, &SOLO_FOLLOW_TRACKS, block, None);
        follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
    }
    for block in 5..8 {
        let a = follow_render(&mut live, &SOLO_FOLLOW_TRACKS, block, None);
        let b = follow_render(&mut booted, &SOLO_FOLLOW_TRACKS, block, None);
        let difference = first_bit_difference(&a, &b);
        assert!(difference.is_none(), "block {block}: {difference:?}");
    }
}

/// Issue #1224 D3 (verdict probe P2): two strips muted in one batch with different windows (480
/// and 37 samples). Each following send ramps with its own source strip's window, block for block
/// like an explicit `routeMute` at that window.
///
/// Test value: red if the ramp lookup ignores the source strip and takes any strip's last mute
/// record; no other test mutes two follow sources at different windows in one batch.
#[test]
fn each_follow_takes_its_own_source_strips_window() {
    let mut follow = solo_follow_host_with_follows(true);
    let mut explicit = solo_follow_host_with_follows(false);
    for host in [&mut follow, &mut explicit] {
        stage_mute(host, 0, 1, true, 480); // drums
        stage_mute(host, 1, 0, true, 37); // bass
    }
    stage_send(
        &mut explicit,
        2,
        COMMAND_ROUTE_MUTE,
        1,
        480,
        [1.0, 0.0, 0.0, 0.0],
    );
    stage_send(
        &mut explicit,
        3,
        COMMAND_ROUTE_MUTE,
        0,
        37,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(follow.submit_commands(2), RESULT_OK);
    assert_eq!(explicit.submit_commands(4), RESULT_OK);
    for block in 0..6 {
        let a = follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        let b = follow_render(&mut explicit, &SOLO_FOLLOW_TRACKS, block, None);
        let difference = first_bit_difference(&a, &b);
        assert!(difference.is_none(), "block {block}: {difference:?}");
    }
}

/// Issue #1224 D3 (verdict probe P4): a solo's follow records ramp at the solo's window, block for
/// block like explicit `routeMute` records at that window (gate 6's comparison, driven by solo).
///
/// Test value: red if the follow pass runs before the solo coalescing pass (it then finds no
/// strip-mute record for the solo's window) or a solo's follow takes any ramp but the solo's.
#[test]
fn a_solo_follow_ramps_at_the_solo_window() {
    let mut follow = solo_follow_host_with_follows(true);
    let mut explicit = solo_follow_host_with_follows(false);
    stage_solo(&mut follow, 0, SOLO_FOLLOW_VOCAL, true, 480);
    stage_solo(&mut explicit, 0, SOLO_FOLLOW_VOCAL, true, 480);
    stage_send(
        &mut explicit,
        1,
        COMMAND_ROUTE_MUTE,
        0,
        480,
        [1.0, 0.0, 0.0, 0.0],
    );
    stage_send(
        &mut explicit,
        2,
        COMMAND_ROUTE_MUTE,
        1,
        480,
        [1.0, 0.0, 0.0, 0.0],
    );
    assert_eq!(follow.submit_commands(1), RESULT_OK);
    assert_eq!(explicit.submit_commands(3), RESULT_OK);
    for block in 0..6 {
        let a = follow_render(&mut follow, &SOLO_FOLLOW_TRACKS, block, None);
        let b = follow_render(&mut explicit, &SOLO_FOLLOW_TRACKS, block, None);
        let difference = first_bit_difference(&a, &b);
        assert!(difference.is_none(), "block {block}: {difference:?}");
    }
}

/// Issue #1224 D3 as amended (verdict MINOR-1, probe P3): `drums`' left lane is muted at 480 in a
/// batch that also carries a no-op kind 4 on its already-unmuted right lane. Host X sends the
/// no-op at 0, host Y at 480, host Z sends none. The strip renders the same in all three (a
/// settled unity lane retargeted to itself is exact), so any difference is the follow record's
/// ramp, which must be the changed lane's 480.
///
/// Test value: red if a strip-mute record that covers no lane whose effective mute changed sets
/// the follow ramp, which steps the send while its strip fades (an audible click on the return).
#[test]
fn a_no_op_record_on_the_other_lane_never_sets_the_follow_ramp() {
    let (mut x, mut y, mut z) = (
        solo_follow_host_with_follows(true),
        solo_follow_host_with_follows(true),
        solo_follow_host_with_follows(true),
    );
    stage_lane_mute(&mut x, 0, 1, 0, true, 480);
    stage_lane_mute(&mut x, 1, 1, 1, false, 0);
    stage_lane_mute(&mut y, 0, 1, 0, true, 480);
    stage_lane_mute(&mut y, 1, 1, 1, false, 480);
    stage_lane_mute(&mut z, 0, 1, 0, true, 480);
    assert_eq!(x.submit_commands(2), RESULT_OK);
    assert_eq!(y.submit_commands(2), RESULT_OK);
    assert_eq!(z.submit_commands(1), RESULT_OK);
    let (mut xy, mut yz) = (None, None);
    for block in 0..6 {
        let a = follow_render(&mut x, &SOLO_FOLLOW_TRACKS, block, None);
        let b = follow_render(&mut y, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut z, &SOLO_FOLLOW_TRACKS, block, None);
        if xy.is_none() {
            xy = first_bit_difference(&a, &b).map(|d| (block, d));
        }
        if yz.is_none() {
            yz = first_bit_difference(&b, &c).map(|d| (block, d));
        }
    }
    assert_eq!(yz, None, "the no-op at 480 is invisible");
    assert_eq!(
        xy, None,
        "a no-op record on the unchanged right lane set the left follow's ramp"
    );
}

/// Issue #1224 D3 as amended (K3 follow-ups verdict MINOR-1, probe V4): one `Both` record whose
/// left lane changes and whose right lane is already muted. `bass` starts with its right lane
/// muted; the batch mutes `bass` on both lanes at 400, re-mutes its right lane at 0 (a no-op) and
/// mutes `drums`' right lane at 64. Host X carries the no-op, host Z does not. The strip renders the
/// same in both, so any difference is `bass-room`'s follow ramp, which must be the `Both`
/// record's 400.
///
/// Test value: red if the `Both` arm of the changed-lane rule asks for both lanes to have changed
/// (`&&` for `||`): that refuses the batch, so a both-lane mute of a strip with one lane already
/// muted and a following send fails; P3 above covers single-lane records only.
#[test]
fn a_both_lane_record_with_one_lane_changed_sets_the_follow_ramp() {
    let muted = [("bass", [false, true])];
    let mut x = solo_follow_host(&muted, 16);
    let mut z = solo_follow_host(&muted, 16);
    // strips: bass 0, drums 1; channel 2 is both lanes.
    stage_lane_mute(&mut x, 0, 0, 2, true, 400);
    stage_lane_mute(&mut x, 1, 0, 1, true, 0);
    stage_lane_mute(&mut x, 2, SOLO_FOLLOW_DRUMS, 1, true, 64);
    stage_lane_mute(&mut z, 0, 0, 2, true, 400);
    stage_lane_mute(&mut z, 1, SOLO_FOLLOW_DRUMS, 1, true, 64);
    assert_eq!(x.submit_commands(3), RESULT_OK);
    assert_eq!(z.submit_commands(2), RESULT_OK);
    let (mut difference, mut audible) = (None, false);
    for block in 0..8 {
        let a = follow_render(&mut x, &SOLO_FOLLOW_TRACKS, block, None);
        let c = follow_render(&mut z, &SOLO_FOLLOW_TRACKS, block, None);
        audible |= a.iter().any(|sample| *sample != 0.0);
        if difference.is_none() {
            difference = first_bit_difference(&a, &c).map(|d| (block, d));
        }
    }
    assert!(audible);
    assert_eq!(
        difference, None,
        "a later no-op lane record set the follow ramp a `Both` record owns"
    );
}

// Issue #1242: a VCA applies at preparation, and the browser's mute state starts from it. VCA `fx`
// (-3 dB) holds `drums` and the solo-safe bus `room` of #1224's follow session; each track is fed
// its own two-lane signal and every bus is a unity submix, so a comparison at the output is exact.
// Strip order: `bass` 0, `drums` 1, `vocal` 2, `room` 3, `verb` 4; live sends in route-ID order:
// `bass-room` 0, `drums-verb` 1.

/// #1224's follow session with the own lane mutes `muted`, `drums-verb` at `send_gain_db`, and VCA
/// `fx` at -3 dB with lane mutes `vca_mutes` over `drums` and `room`.
fn vca_follow_host(
    muted: &[(&str, [bool; 2])],
    vca_mutes: [bool; 2],
    send_gain_db: f32,
) -> AudioWorkletEngineHost {
    let document = follow_document(
        &SOLO_FOLLOW_TRACKS,
        &["verb", "room"],
        &SOLO_FOLLOW_ROUTES,
        muted,
        None,
    );
    let mut model = parse_session_json(&document).expect("#1224 session parses");
    model
        .routes
        .iter_mut()
        .find(|route| route.id.as_str() == "drums-verb")
        .expect("drums-verb")
        .gain_db = send_gain_db;
    model.vcas = vec![session::Vca {
        id: strip_id("fx"),
        fader: session::DualMonoFader {
            left_db: -3.0,
            right_db: -3.0,
            left_mute: vca_mutes[0],
            right_mute: vca_mutes[1],
        },
        members: vec![strip_id("drums"), strip_id("room")],
    }];
    strip_boot(
        &canonical_session_json(&model).expect("#1242 session canonicalizes"),
        strip_options(16, 0, 0),
    )
}

/// Issue #1242 gate 5, in the browser. With VCA `fx` muted, soloing its member `drums` beside
/// `vocal` leaves `drums`, its following send and the solo-safe member `room` muted,
/// bit-identically to a host booted with `bass` muted instead; un-soloing both, then explicitly
/// un-muting `drums` (kind 4, `false`), leaves it muted, bit-identically to a host that received
/// none of those commands.
/// Every command has smoothing 0 (a kind 4 always stages a record, and a non-zero ramp on a
/// settled lane can turn `+0.0` into `-0.0`, which is not this gate's claim).
///
/// Test value: red if solo clears a VCA mute (the soloed member is heard), if solo-safe exempts
/// a submix from it (`room` opens), if an explicit un-mute stages the member's own intent rather
/// than the composed mute, if host-web seeds the solo state without the VCA mute, or if `emitted`
/// is seeded without it (the solo stages a redundant record for `drums`).
#[test]
fn a_vca_muted_member_stays_muted_through_solo_and_an_explicit_unmute() {
    let mut live = vca_follow_host(&[], [true; 2], 0.0);
    let mut soloed = vca_follow_host(&[("bass", [true; 2])], [true; 2], 0.0);
    let mut untouched = vca_follow_host(&[], [true; 2], 0.0);
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [true; 2]],
        "seeded mirror"
    );
    follow_lockstep(
        &mut [&mut live, &mut soloed, &mut untouched],
        &SOLO_FOLLOW_TRACKS,
        0,
        1,
        false,
        "warm-up",
    );
    let room = follow_fader_room(&live);
    stage_solo(&mut live, 0, SOLO_FOLLOW_DRUMS, true, 0);
    stage_solo(&mut live, 1, SOLO_FOLLOW_VOCAL, true, 0);
    assert_eq!(live.submit_commands(2), RESULT_OK, "solo drums and vocal");
    assert_eq!(follow_lanes(&live), [[true; 2], [true; 2]], "solo follow");
    let mut owed = room.clone();
    owed[0] -= 1;
    assert_eq!(
        follow_fader_room(&live),
        owed,
        "the solo stages one record, on `bass`, and none for the VCA-muted `drums` or `room`"
    );
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut soloed],
            &SOLO_FOLLOW_TRACKS,
            1,
            2,
            true,
            "soloed member vs booted muted rest"
        ),
        "the soloed vocal is audible"
    );
    stage_solo(&mut live, 0, SOLO_FOLLOW_DRUMS, false, 0);
    stage_solo(&mut live, 1, SOLO_FOLLOW_VOCAL, false, 0);
    assert_eq!(
        live.submit_commands(2),
        RESULT_OK,
        "un-solo drums and vocal"
    );
    stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, false, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "un-mute drums");
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [true; 2]],
        "restored mirror"
    );
    for block in 1..3 {
        follow_render(&mut untouched, &SOLO_FOLLOW_TRACKS, block, None);
    }
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut untouched],
            &SOLO_FOLLOW_TRACKS,
            3,
            3,
            true,
            "after un-solo and un-mute vs untouched"
        ),
        "bass and vocal are audible"
    );
}

/// Issue #1242 gate 6. VCA `fx` mutes `drums`' left lane, so `drums-verb` (a pre-fader
/// `follows_mute` send) is prepared with its left column zeroed. A kind 13 (`routeGainDb`) edit
/// on that send, with smoothing 0 at a block boundary, renders bit-identically to a host booted
/// from the session with the edited gain: the live mirror started at the prepared gate, so the
/// zeroed column stays zeroed.
///
/// Test value: red if host-web seeds the VCA mute after the live-send mirror, or seeds the
/// mirror from the member's own mute, so the first live send edit reopens a VCA-muted column.
#[test]
fn a_live_send_edit_keeps_a_vca_muted_column_zeroed() {
    const GAIN_DB: f32 = -4.5;
    let mut live = vca_follow_host(&[], [true, false], 0.0);
    let mut fresh = vca_follow_host(&[], [true, false], GAIN_DB);
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [true, false]],
        "seeded mirror"
    );
    follow_lockstep(
        &mut [&mut live, &mut fresh],
        &SOLO_FOLLOW_TRACKS,
        0,
        1,
        false,
        "before the edit",
    );
    stage_send(
        &mut live,
        0,
        COMMAND_ROUTE_GAIN_DB,
        1,
        0,
        [GAIN_DB, 0.0, 0.0, 0.0],
    );
    assert_eq!(
        live.submit_commands(1),
        RESULT_OK,
        "routeGainDb on drums-verb"
    );
    assert!(
        follow_lockstep(
            &mut [&mut live, &mut fresh],
            &SOLO_FOLLOW_TRACKS,
            1,
            3,
            true,
            "edited live send vs fresh plan"
        ),
        "the mix is audible"
    );
}

/// Issue #1242 attempt 2 (verdict MAJOR-1). VCA `fx` mutes one lane of `drums`; a kind 4 un-mute
/// addressed to both lanes (`channel` 2) leaves the VCA-muted lane muted and the other open, so
/// the strip, its following send `drums-verb` and the solo state's mirrors all stay where they
/// were: bit-identical to a host that received nothing (the member's own mute was already
/// `false`). Each lane mute is taken in turn. The same submission un-mutes `bass` (no VCA) on both
/// lanes: `drums`' queue takes two records (the split) and `bass`' takes one.
///
/// Test value: red if a `Both` kind 4 stages one record from the first covered lane's effective
/// mute, which re-opens a VCA-muted right lane, or silences an un-muted right lane while its
/// follow send stays open; or if every `Both` kind 4 splits, doubling fader-queue use for every
/// both-lanes mute.
#[test]
fn a_both_lane_unmute_keeps_a_one_lane_vca_mute() {
    for vca_mutes in [[true, false], [false, true]] {
        let mut live = vca_follow_host(&[], vca_mutes, 0.0);
        let mut untouched = vca_follow_host(&[], vca_mutes, 0.0);
        follow_lockstep(
            &mut [&mut live, &mut untouched],
            &SOLO_FOLLOW_TRACKS,
            0,
            1,
            false,
            "warm-up",
        );
        let room = follow_fader_room(&live);
        stage_lane_mute(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, false, 0);
        stage_lane_mute(&mut live, 1, 0, 2, false, 0);
        assert_eq!(
            live.submit_commands(2),
            RESULT_OK,
            "un-mute drums and bass, both lanes"
        );
        let after = follow_fader_room(&live);
        assert_eq!(room[1] - after[1], 2, "split: two records on drums");
        assert_eq!(room[0] - after[0], 1, "agree: one record on bass");
        let solo = &live.ready.as_ref().expect("ready").solo;
        let drums = SOLO_FOLLOW_DRUMS as usize;
        for (lane, &vca_muted) in vca_mutes.iter().enumerate() {
            assert_eq!(solo.effective_mute(drums, lane), vca_muted, "lane {lane}");
            assert_eq!(solo.emitted_mute(drums, lane), vca_muted, "lane {lane}");
        }
        assert_eq!(
            follow_lanes(&live),
            [[false; 2], vca_mutes],
            "vca {vca_mutes:?}: follow mirror"
        );
        assert!(
            follow_lockstep(
                &mut [&mut live, &mut untouched],
                &SOLO_FOLLOW_TRACKS,
                1,
                3,
                true,
                &format!("vca {vca_mutes:?}: both-lane un-mute vs untouched"),
            ),
            "audible"
        );
    }
}

/// The top-down reach of `strip` in `vcas` (every VCA whose membership closure holds it), sorted by
/// VCA ID: the randomized browser gate's own reference, independent of `SessionModel::vca_reach`.
fn vca_reference_reach(vcas: &[session::Vca], strip: &str) -> Vec<usize> {
    fn below<'a>(
        vcas: &'a [session::Vca],
        index: usize,
        into: &mut std::collections::BTreeSet<&'a str>,
    ) {
        for member in &vcas[index].members {
            if into.insert(member.as_str())
                && let Some(nested) = vcas.iter().position(|vca| vca.id == *member)
            {
                below(vcas, nested, into);
            }
        }
    }
    let mut reach: Vec<usize> = (0..vcas.len())
        .filter(|index| {
            let mut closure = std::collections::BTreeSet::new();
            below(vcas, *index, &mut closure);
            closure.contains(strip)
        })
        .collect();
    reach.sort_by(|left, right| vcas[*left].id.cmp(&vcas[*right].id));
    reach
}

/// Seeds of the randomized browser VCA gate. Attempt 2 ran 1,500 seeds as evidence; this count
/// keeps the debug suite near a second.
const VCA_BROWSER_SEEDS: u64 = 48;

/// Issue #1242 attempt 2 (verdict MAJOR-1 and MINOR-1), a randomized browser differential. Random
/// VCA forests (one to four VCAs, nested, offsets across the domain, drawn lane mutes) over
/// #1224's follow session, whose own faders and mutes are drawn too; then eight random batches of
/// solo toggles and kind 4 edits on any lane selector, each at a smoothing drawn from {0, 64}. The
/// VCA host renders, every block, bit-identically to a host booted with no VCA and every strip's
/// fader written as its effective value (the test's own top-down reach and `f64` sum), which
/// receives the same solos and, for each kind 4, the per-lane equivalent `on || vca_mute[lane]`,
/// at the same smoothing.
///
/// Test value: red if a `Both` kind 4 on a one-lane VCA mute stages one lane's value for both, or
/// if host-web seeds a strip's VCA mute (a submix's included) other than from the effective
/// faders, so a later un-mute of a VCA-muted bus reopens it, or if the split records drop the
/// command's window (a hard-switched lane where the reference ramps).
#[test]
fn a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute() {
    fn chance(draw: &mut SendDraw, numerator: u64, denominator: u64) -> bool {
        draw.below(denominator) < numerator
    }
    let names = ["bass", "drums", "vocal", "room", "verb"];
    let (mut split_unmutes, mut submix_unmutes) = (0_u32, 0_u32);
    for seed in 1..=VCA_BROWSER_SEEDS {
        let mut draw = SendDraw(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let document = follow_document(
            &SOLO_FOLLOW_TRACKS,
            &["verb", "room"],
            &SOLO_FOLLOW_ROUTES,
            &[],
            None,
        );
        let mut model = parse_session_json(&document).expect("#1224 session parses");
        let faders = model
            .tracks
            .iter_mut()
            .map(|track| &mut track.fader)
            .chain(model.submixes.iter_mut().map(|submix| &mut submix.fader));
        for fader in faders {
            fader.left_db = draw.uniform(-12.0, 6.0);
            fader.right_db = draw.uniform(-12.0, 6.0);
            fader.left_mute = chance(&mut draw, 1, 6);
            fader.right_mute = chance(&mut draw, 1, 6);
        }
        let count = 1 + draw.below(4) as usize;
        let levels: Vec<u64> = (0..count).map(|_| draw.below(3)).collect();
        let vca_names: Vec<String> = (0..count).map(|i| format!("v{}", (i * 7) % 11)).collect();
        model.vcas = (0..count)
            .map(|i| {
                let candidates = names.iter().map(|name| (*name).to_owned()).chain(
                    (0..count)
                        .filter(|other| levels[*other] > levels[i])
                        .map(|other| vca_names[other].clone()),
                );
                let members: Vec<session::StableId> = candidates
                    .filter(|_| chance(&mut draw, 1, 2))
                    .map(|member| strip_id(&member))
                    .collect();
                let mut offset = || {
                    if chance(&mut draw, 1, 4) {
                        draw.uniform(-144.0, 24.0)
                    } else {
                        draw.uniform(-12.0, 12.0)
                    }
                };
                let (left_db, right_db) = (offset(), offset());
                session::Vca {
                    id: strip_id(&vca_names[i]),
                    fader: session::DualMonoFader {
                        left_db,
                        right_db,
                        left_mute: chance(&mut draw, 1, 3),
                        right_mute: chance(&mut draw, 1, 3),
                    },
                    members,
                }
            })
            .collect();
        // Strip order is the canonical one: tracks, then submixes, each sorted.
        let canonical = parse_session_json(&canonical_session_json(&model).expect("canonical"))
            .expect("canonical session parses");
        let strip_ids: Vec<String> = canonical
            .tracks
            .iter()
            .map(|track| track.id.as_str().to_owned())
            .chain(canonical.submixes.iter().map(|s| s.id.as_str().to_owned()))
            .collect();
        let mut plain = canonical.clone();
        plain.vcas.clear();
        let mut vca_mute = Vec::new();
        let faders = plain
            .tracks
            .iter_mut()
            .map(|track| &mut track.fader)
            .chain(plain.submixes.iter_mut().map(|submix| &mut submix.fader));
        for (fader, strip) in faders.zip(&strip_ids) {
            let reach = vca_reference_reach(&canonical.vcas, strip);
            let lane = |l: usize| -> (f32, bool) {
                let own = [fader.left_db, fader.right_db][l];
                if reach.is_empty() {
                    return (own, false);
                }
                let mut sum = f64::from(own);
                let mut muted = false;
                for index in &reach {
                    let vca = &canonical.vcas[*index].fader;
                    sum += f64::from([vca.left_db, vca.right_db][l]);
                    muted |= [vca.left_mute, vca.right_mute][l];
                }
                (sum.clamp(-144.0, 24.0) as f32, muted)
            };
            let (left, right) = (lane(0), lane(1));
            fader.left_db = left.0;
            fader.right_db = right.0;
            fader.left_mute |= left.1;
            fader.right_mute |= right.1;
            vca_mute.push([left.1, right.1]);
        }
        let mut live = strip_boot(
            &canonical_session_json(&canonical).expect("canonical"),
            strip_options(64, 0, 0),
        );
        let mut reference = strip_boot(
            &canonical_session_json(&plain).expect("canonical"),
            strip_options(64, 0, 0),
        );
        follow_lockstep(
            &mut [&mut live, &mut reference],
            &SOLO_FOLLOW_TRACKS,
            0,
            1,
            true,
            &format!("seed {seed}: boot"),
        );
        let mut log = Vec::new();
        for block in 1..=8 {
            let (mut a, mut b) = (0_usize, 0_usize);
            for _ in 0..=draw.below(3) {
                let smoothing = [0_u32, 64][draw.below(2) as usize];
                if chance(&mut draw, 1, 2) {
                    let track = draw.below(3) as u32;
                    let on = chance(&mut draw, 1, 2);
                    stage_solo(&mut live, a, track, on, smoothing);
                    stage_solo(&mut reference, b, track, on, smoothing);
                    (a, b) = (a + 1, b + 1);
                    log.push(format!("solo {track} {on}"));
                    continue;
                }
                let strip = draw.below(5) as usize;
                let channel = draw.below(3) as u8;
                let on = chance(&mut draw, 1, 2);
                stage_lane_mute(&mut live, a, strip as u32, channel, on, smoothing);
                a += 1;
                let want = |lane: usize| on || vca_mute[strip][lane];
                if !on && vca_mute[strip] != [false; 2] {
                    split_unmutes += u32::from(channel == 2 && want(0) != want(1));
                    submix_unmutes += u32::from(strip >= 3);
                }
                let lanes: &[(u8, bool)] = match channel {
                    0 => &[(0, want(0))],
                    1 => &[(1, want(1))],
                    _ if want(0) == want(1) => &[(2, want(0))],
                    _ => &[(0, want(0)), (1, want(1))],
                };
                for (lane, value) in lanes {
                    stage_lane_mute(&mut reference, b, strip as u32, *lane, *value, smoothing);
                    b += 1;
                }
                log.push(format!(
                    "mute strip {strip} channel {channel} {on} (vca {:?})",
                    vca_mute[strip]
                ));
            }
            assert_eq!(
                live.submit_commands(a as u32),
                RESULT_OK,
                "seed {seed}: live"
            );
            assert_eq!(
                reference.submit_commands(b as u32),
                RESULT_OK,
                "seed {seed}: reference"
            );
            follow_lockstep(
                &mut [&mut live, &mut reference],
                &SOLO_FOLLOW_TRACKS,
                block,
                1,
                true,
                &format!("seed {seed}: after {log:?}"),
            );
        }
    }
    assert!(
        split_unmutes > 0 && submix_unmutes > 0,
        "the seeds reach a both-lane un-mute of a one-lane VCA mute ({split_unmutes}) and an \
         un-mute of a VCA-muted submix ({submix_unmutes})"
    );
}

// ---- Issue #1245: VCA groups ridden live in the browser ----
//
// Every session is #1224's follow session: three tracks each fed its own two-lane signal, two unity
// submixes, and the pre-fader `follows_mute` sends `bass-room` and `drums-verb`, so a comparison
// at the output is exact (no console slot, no stateful insert, identity input section). Strip
// order: `bass` 0, `drums` 1, `vocal` 2, `room` 3, `verb` 4; live sends in route-ID order:
// `bass-room` 0, `drums-verb` 1. VCA IDs are chosen in canonical order, so a row's position is
// its VCA index.

const VCA_BASS: u32 = 0;
const VCA_STRIPS: [&str; 5] = ["bass", "drums", "vocal", "room", "verb"];

/// One strip's own fader: `(id, [left, right] dB, [left, right] mute)`.
type VcaStripRow<'a> = (&'a str, [f32; 2], [bool; 2]);
/// One VCA: `(id, [left, right] offset dB, [left, right] mute, members)`.
type VcaRow<'a> = (&'a str, [f32; 2], [bool; 2], &'a [&'a str]);

fn vca_strip_fader<'m>(
    model: &'m mut session::SessionModel,
    id: &str,
) -> &'m mut session::DualMonoFader {
    if let Some(track) = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == id)
    {
        return &mut track.fader;
    }
    &mut model
        .submixes
        .iter_mut()
        .find(|submix| submix.id.as_str() == id)
        .expect("#1245 strip")
        .fader
}

/// #1224's follow session with the own faders `strips` and the VCAs `vcas`.
fn vca_ride_model(strips: &[VcaStripRow<'_>], vcas: &[VcaRow<'_>]) -> session::SessionModel {
    let document = follow_document(
        &SOLO_FOLLOW_TRACKS,
        &["verb", "room"],
        &SOLO_FOLLOW_ROUTES,
        &[],
        None,
    );
    let mut model = parse_session_json(&document).expect("#1224 session parses");
    for (id, db, mute) in strips {
        let fader = vca_strip_fader(&mut model, id);
        (fader.left_db, fader.right_db) = (db[0], db[1]);
        (fader.left_mute, fader.right_mute) = (mute[0], mute[1]);
    }
    model.vcas = vcas
        .iter()
        .map(|(id, db, mute, members)| session::Vca {
            id: strip_id(id),
            fader: session::DualMonoFader {
                left_db: db[0],
                right_db: db[1],
                left_mute: mute[0],
                right_mute: mute[1],
            },
            members: members.iter().map(|member| strip_id(member)).collect(),
        })
        .collect();
    model
}

fn vca_ride_host(model: &session::SessionModel, queue_records: u64) -> AudioWorkletEngineHost {
    strip_boot(
        &canonical_session_json(model).expect("#1245 session canonicalizes"),
        strip_options(queue_records, 0, 0),
    )
}

/// Stage one VCA record (kind 16 or 17) at wire index `index`.
fn stage_vca(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    kind: u32,
    vca: u32,
    channel: u8,
    value: f32,
    smoothing: u32,
) {
    stage_command(
        host,
        index,
        kind,
        255,
        channel,
        vca,
        0,
        0,
        smoothing,
        [value, 0.0, 0.0, 0.0],
    );
}

/// Stage one kind 3 (`faderDb`) record at wire index `index`.
fn stage_fader_db(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    strip: u32,
    channel: u8,
    db: f32,
    smoothing: u32,
) {
    stage_command(
        host,
        index,
        COMMAND_FADER_DB,
        255,
        channel,
        strip,
        0,
        0,
        smoothing,
        [db, 0.0, 0.0, 0.0],
    );
}

/// Render `blocks` blocks from `first` on every host; when `compare` names a host, require host 0's
/// output bit-identical to it. Returns whether a compared block carried signal.
fn vca_lockstep(
    hosts: &mut [&mut AudioWorkletEngineHost],
    compare: Option<usize>,
    first: u64,
    blocks: u64,
    what: &str,
) -> bool {
    let mut audible = false;
    for block in first..first + blocks {
        let outputs: Vec<Vec<f32>> = hosts
            .iter_mut()
            .map(|host| follow_render(host, &SOLO_FOLLOW_TRACKS, block, None))
            .collect();
        if let Some(other) = compare {
            if let Some((sample, x, y)) = first_bit_difference(&outputs[0], &outputs[other]) {
                panic!("{what}: block {block} sample {sample}: {x} vs {y}");
            }
            audible |= outputs[0].iter().any(|sample| *sample != 0.0);
        }
    }
    audible
}

/// The live VCA state's effective dB of every strip and lane, and its VCA mute.
fn vca_mirror(host: &AudioWorkletEngineHost) -> Vec<([u32; 2], [bool; 2])> {
    let vcas = &host.ready.as_ref().expect("ready").vcas;
    (0..VCA_STRIPS.len())
        .map(|strip| {
            (
                [
                    vcas.effective_db(strip, 0).to_bits(),
                    vcas.effective_db(strip, 1).to_bits(),
                ],
                vcas.vca_mute(strip),
            )
        })
        .collect()
}

/// Every strip's effective and emitted lane mutes in the strip-mute owner.
fn vca_solo_mirror(host: &AudioWorkletEngineHost) -> Vec<[bool; 4]> {
    let solo = &host.ready.as_ref().expect("ready").solo;
    (0..VCA_STRIPS.len())
        .map(|strip| {
            [
                solo.effective_mute(strip, 0),
                solo.effective_mute(strip, 1),
                solo.emitted_mute(strip, 0),
                solo.emitted_mute(strip, 1),
            ]
        })
        .collect()
}

/// Everything a refused submission must leave as it was: every fader and send queue's room, the
/// VCA state, the strip-mute owner and the send mirror.
type VcaSnapshot = (
    Vec<usize>,
    Vec<usize>,
    Vec<([u32; 2], [bool; 2])>,
    Vec<[bool; 4]>,
    Vec<SendValues>,
    Vec<[bool; 2]>,
);

fn vca_snapshot(host: &AudioWorkletEngineHost) -> VcaSnapshot {
    (
        follow_fader_room(host),
        send_queue_room(host),
        vca_mirror(host),
        vca_solo_mirror(host),
        send_mirror(host),
        follow_lanes(host),
    )
}

/// Submit `count` staged records and require a whole refusal: `result`, `reason` and
/// `rejected_index`, nothing pushed, and every mirror and transaction left as it was.
fn assert_vca_refusal(
    host: &mut AudioWorkletEngineHost,
    count: u32,
    result: u32,
    reason: u32,
    rejected_index: u32,
    what: &str,
) {
    let before = vca_snapshot(host);
    assert_eq!(host.submit_commands(count), result, "{what}: result");
    let report = *host.command_report();
    assert_eq!(report.reason, reason, "{what}: reason");
    assert_eq!(
        report.rejected_index, rejected_index,
        "{what}: rejected index"
    );
    assert_eq!(report.admitted, 0, "{what}: admitted");
    assert_eq!(vca_snapshot(host), before, "{what}: state moved");
    let ready = host.ready.as_ref().expect("ready");
    assert!(!ready.vcas.transaction_open(), "{what}: VCA transaction");
}

/// The reach of the randomized gate, so a generator change that stops reaching a case is red.
#[derive(Debug, Default)]
struct VcaRideReach {
    vca_rides: u32,
    nested_rides: u32,
    submix_rides: u32,
    one_lane_vca_mutes: u32,
    member_moves: u32,
    clamped_lanes: u32,
    follow_moves: u32,
    soloed_vca_muted: u32,
}

/// The test's own effective dB of `strip`'s `lane`: its own value plus every reaching VCA's
/// offset, summed in `f64` and clamped, from its own top-down reach. Also whether it clamped.
fn vca_reference_effective(model: &session::SessionModel, strip: &str, lane: usize) -> (f32, bool) {
    let fader = model
        .tracks
        .iter()
        .map(|track| (track.id.as_str(), &track.fader))
        .chain(model.submixes.iter().map(|s| (s.id.as_str(), &s.fader)))
        .find(|(id, _)| *id == strip)
        .expect("strip")
        .1;
    let mut sum = f64::from([fader.left_db, fader.right_db][lane]);
    let reach = vca_reference_reach(&model.vcas, strip);
    for index in &reach {
        let vca = &model.vcas[*index].fader;
        sum += f64::from([vca.left_db, vca.right_db][lane]);
    }
    let clamped = sum.clamp(-144.0, 24.0);
    (clamped as f32, !reach.is_empty() && clamped != sum)
}

/// Seeds of the randomized ride gate (#1245 gate 1).
const VCA_RIDE_SEEDS: u64 = 8;

/// Issue #1245 gate 1, and the settled-session differential. Random VCA forests (two to four
/// VCAs, nested and overlapping, tracks and submixes as members, offsets across the domain) over
/// #1224's follow session; then six random batches of VCA rides (one lane or both, values across
/// the domain so members clamp, ramps up to 300 samples), one-lane and both-lane VCA mutes,
/// member fader moves, one-lane member mutes and solos. After each batch's ramps settle, the live
/// host renders bit-identically to a host freshly booted from the session the test edited itself
/// (its own intent: VCA values, own faders and own mutes; solos re-sent at boot), and the two
/// hosts' follow mirrors agree.
///
/// Test value: red if live admission emits to the wrong member, lane or slot, composes
/// differently from preparation (order, reach, clamp), stores a clamped value as a member's own,
/// drops a VCA mute from a member's mute or its following sends, or lets solo clear it.
#[test]
fn a_live_vca_ride_lands_on_a_fresh_plans_bits() {
    fn chance(draw: &mut SendDraw, numerator: u64, denominator: u64) -> bool {
        draw.below(denominator) < numerator
    }
    let mut reach = VcaRideReach::default();
    for seed in 1..=VCA_RIDE_SEEDS {
        let mut draw = SendDraw(seed.wrapping_mul(0xA24B_AED4_963E_E407));
        let mut model = vca_ride_model(&[], &[]);
        for strip in VCA_STRIPS {
            let left_db = draw.uniform(-12.0, 6.0);
            let right_db = draw.uniform(-12.0, 6.0);
            let (left_mute, right_mute) = (chance(&mut draw, 1, 8), chance(&mut draw, 1, 8));
            let fader = vca_strip_fader(&mut model, strip);
            (fader.left_db, fader.right_db) = (left_db, right_db);
            (fader.left_mute, fader.right_mute) = (left_mute, right_mute);
        }
        let ids = ["va", "vb", "vc", "vd"];
        let count = 2 + draw.below(3) as usize;
        let levels: Vec<u64> = (0..count).map(|_| draw.below(3)).collect();
        model.vcas = (0..count)
            .map(|i| {
                let candidates: Vec<&str> = VCA_STRIPS
                    .iter()
                    .copied()
                    .chain(
                        (0..count)
                            .filter(|j| levels[*j] > levels[i])
                            .map(|j| ids[j]),
                    )
                    .collect();
                let members = candidates
                    .into_iter()
                    .filter(|_| chance(&mut draw, 1, 2))
                    .map(strip_id)
                    .collect();
                let mut offset = || {
                    if chance(&mut draw, 1, 4) {
                        draw.uniform(-144.0, 24.0)
                    } else {
                        draw.uniform(-12.0, 12.0)
                    }
                };
                let (left_db, right_db) = (offset(), offset());
                session::Vca {
                    id: strip_id(ids[i]),
                    fader: session::DualMonoFader {
                        left_db,
                        right_db,
                        left_mute: chance(&mut draw, 1, 4),
                        right_mute: chance(&mut draw, 1, 4),
                    },
                    members,
                }
            })
            .collect();
        let mut model = parse_session_json(&canonical_session_json(&model).expect("canonical"))
            .expect("canonical session parses");
        let nested: Vec<bool> = model
            .vcas
            .iter()
            .map(|vca| {
                vca.members
                    .iter()
                    .any(|member| ids.contains(&member.as_str()))
                    || model
                        .vcas
                        .iter()
                        .any(|other| other.members.contains(&vca.id))
            })
            .collect();
        let mut live = vca_ride_host(&model, 64);
        {
            // Amendment A1: the boot-time pair count is the reach the live state builds.
            let vcas = &live.ready.as_ref().expect("ready").vcas;
            let pairs: usize = (0..vcas.vca_count())
                .map(|vca| vcas.reached_by(vca).len())
                .sum();
            let shape = browser_vca_shape(&model).expect("within the bounds");
            assert_eq!(shape.pairs, pairs as u64, "seed {seed}: reach pairs");
        }
        follow_render(&mut live, &SOLO_FOLLOW_TRACKS, 0, None);
        let mut block = 1_u64;
        let mut solos = [false; 3];
        let mut log = Vec::new();
        for _ in 0..6 {
            let lanes_before = follow_lanes(&live);
            let records = 1 + draw.below(5) as usize;
            for index in 0..records {
                let channel = draw.below(3) as u8;
                let covers = |lane: usize| channel == 2 || usize::from(channel) == lane;
                match draw.below(10) {
                    0..=3 => {
                        let vca = draw.below(model.vcas.len() as u64) as usize;
                        let db = match draw.below(6) {
                            0 => 24.0,
                            1 => -144.0,
                            2 | 3 => draw.uniform(-144.0, 24.0),
                            _ => draw.uniform(-12.0, 12.0),
                        };
                        let smoothing = [0, 64, 200, 300][draw.below(4) as usize];
                        stage_vca(
                            &mut live,
                            index,
                            COMMAND_VCA_FADER_DB,
                            vca as u32,
                            channel,
                            db,
                            smoothing,
                        );
                        let fader = &mut model.vcas[vca].fader;
                        if covers(0) {
                            fader.left_db = db;
                        }
                        if covers(1) {
                            fader.right_db = db;
                        }
                        reach.vca_rides += 1;
                        reach.nested_rides += u32::from(nested[vca]);
                        let id = model.vcas[vca].id.as_str().to_owned();
                        reach.submix_rides += u32::from(["room", "verb"].iter().any(|bus| {
                            vca_reference_reach(&model.vcas, bus)
                                .iter()
                                .any(|index| model.vcas[*index].id.as_str() == id)
                        }));
                        log.push(format!("vca {vca} ch {channel} {db} dB @{smoothing}"));
                    }
                    4 | 5 => {
                        let vca = draw.below(model.vcas.len() as u64) as usize;
                        let on = chance(&mut draw, 1, 2);
                        let smoothing = [0, 64, 200, 300][draw.below(4) as usize];
                        stage_vca(
                            &mut live,
                            index,
                            COMMAND_VCA_MUTE,
                            vca as u32,
                            channel,
                            if on { 1.0 } else { 0.0 },
                            smoothing,
                        );
                        let fader = &mut model.vcas[vca].fader;
                        if covers(0) {
                            fader.left_mute = on;
                        }
                        if covers(1) {
                            fader.right_mute = on;
                        }
                        reach.one_lane_vca_mutes += u32::from(channel != 2);
                        log.push(format!("vca {vca} ch {channel} mute {on} @{smoothing}"));
                    }
                    6 | 7 => {
                        let strip = draw.below(5) as usize;
                        let db = if chance(&mut draw, 1, 4) {
                            draw.uniform(-144.0, 24.0)
                        } else {
                            draw.uniform(-18.0, 12.0)
                        };
                        stage_fader_db(&mut live, index, strip as u32, channel, db, 0);
                        let fader = vca_strip_fader(&mut model, VCA_STRIPS[strip]);
                        if covers(0) {
                            fader.left_db = db;
                        }
                        if covers(1) {
                            fader.right_db = db;
                        }
                        reach.member_moves += u32::from(
                            !vca_reference_reach(&model.vcas, VCA_STRIPS[strip]).is_empty(),
                        );
                        log.push(format!("strip {strip} ch {channel} {db} dB"));
                    }
                    8 => {
                        let strip = draw.below(5) as usize;
                        let on = chance(&mut draw, 1, 2);
                        stage_lane_mute(&mut live, index, strip as u32, channel, on, 0);
                        let fader = vca_strip_fader(&mut model, VCA_STRIPS[strip]);
                        if covers(0) {
                            fader.left_mute = on;
                        }
                        if covers(1) {
                            fader.right_mute = on;
                        }
                        log.push(format!("strip {strip} ch {channel} mute {on}"));
                    }
                    _ => {
                        let track = draw.below(3) as usize;
                        let on = chance(&mut draw, 1, 2);
                        stage_solo(&mut live, index, track as u32, on, 0);
                        solos[track] = on;
                        log.push(format!("solo {track} {on}"));
                    }
                }
            }
            assert_eq!(
                live.submit_commands(records as u32),
                RESULT_OK,
                "seed {seed}: {log:?} (reason {})",
                live.command_report().reason
            );
            // Every ramp is at most 300 samples, so three 128-frame blocks settle it.
            for ramp in block..block + 3 {
                follow_render(&mut live, &SOLO_FOLLOW_TRACKS, ramp, None);
            }
            let mut fresh = vca_ride_host(&model, 64);
            follow_render(&mut fresh, &SOLO_FOLLOW_TRACKS, 0, None);
            let soloed: Vec<usize> = (0..3).filter(|track| solos[*track]).collect();
            for (index, track) in soloed.iter().enumerate() {
                stage_solo(&mut fresh, index, *track as u32, true, 0);
            }
            if !soloed.is_empty() {
                assert_eq!(fresh.submit_commands(soloed.len() as u32), RESULT_OK);
            }
            for history in 1..block + 3 {
                follow_render(&mut fresh, &SOLO_FOLLOW_TRACKS, history, None);
            }
            assert_eq!(
                follow_lanes(&live),
                follow_lanes(&fresh),
                "seed {seed}: follow mirrors after {log:?}"
            );
            vca_lockstep(
                &mut [&mut live, &mut fresh],
                Some(1),
                block + 3,
                2,
                &format!("seed {seed}: live vs fresh after {log:?}"),
            );
            reach.follow_moves += u32::from(follow_lanes(&live) != lanes_before);
            for strip in VCA_STRIPS {
                for lane in 0..2 {
                    reach.clamped_lanes +=
                        u32::from(vca_reference_effective(&model, strip, lane).1);
                }
            }
            let solo = &live.ready.as_ref().expect("ready").solo;
            reach.soloed_vca_muted += (0..3)
                .filter(|track| {
                    solos[*track] && (solo.vca_mute(*track, 0) || solo.vca_mute(*track, 1))
                })
                .count() as u32;
            block += 5;
        }
    }
    assert!(
        reach.vca_rides > 0
            && reach.nested_rides > 0
            && reach.submix_rides > 0
            && reach.one_lane_vca_mutes > 0
            && reach.member_moves > 0
            && reach.clamped_lanes > 0
            && reach.follow_moves > 0
            && reach.soloed_vca_muted > 0,
        "the seeds reach every case: {reach:?}"
    );
}

/// Issue #1245 gate 2: a member's own move and a VCA move compose, each with smoothing 0 at a
/// block boundary. `drums` and `bass` (own `[-2, -5]` dB) are in VCA `band`:
///
/// * `drums` -3 dB, then `band` -6 dB, then `drums` +2 dB land, after each step, on a host booted
///   at that step's values; `bass` keeps its own balance throughout.
/// * With `drums` at +20 dB, `band` to +24 dB clamps it to +24 dB (as a host booted there does),
///   and `band` back to 0 dB returns it to +20 dB, bit-identical to an unedited host.
/// * With `drums` own-muted and `band` muted, un-muting `band` leaves `drums` muted, as a host
///   booted with only its own mute.
///
/// Test value: red if a member move overwrites the VCA term or the reverse, if the clamp is
/// stored as the member's value, or if a VCA un-mute clears a member's own mute.
#[test]
fn a_member_move_and_a_vca_move_compose() {
    const BAND: &[&str] = &["bass", "drums"];
    let host = |drums: f32, band: f32| {
        vca_ride_host(
            &vca_ride_model(
                &[
                    ("drums", [drums; 2], [false; 2]),
                    ("bass", [-2.0, -5.0], [false; 2]),
                ],
                &[("band", [band; 2], [false; 2], BAND)],
            ),
            16,
        )
    };
    let drums = SOLO_FOLLOW_DRUMS;
    let mut live = host(0.0, 0.0);
    let mut steps = [host(-3.0, 0.0), host(-3.0, -6.0), host(2.0, -6.0)];
    let [step1, step2, step3] = &mut steps;
    vca_lockstep(&mut [&mut live, step1, step2, step3], None, 0, 1, "warm-up");
    stage_fader_db(&mut live, 0, drums, 2, -3.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "drums -3 dB");
    assert!(vca_lockstep(
        &mut [&mut live, step1, step2, step3],
        Some(1),
        1,
        2,
        "drums -3 dB"
    ));
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, -6.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "band -6 dB");
    assert!(vca_lockstep(
        &mut [&mut live, step1, step2, step3],
        Some(2),
        3,
        2,
        "band -6 dB"
    ));
    stage_fader_db(&mut live, 0, drums, 2, 2.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "drums +2 dB");
    assert!(vca_lockstep(
        &mut [&mut live, step1, step2, step3],
        Some(3),
        5,
        2,
        "drums +2 dB in band -6 dB"
    ));

    let mut live = host(20.0, 0.0);
    let mut clamped = host(20.0, 24.0);
    let mut unedited = host(20.0, 0.0);
    vca_lockstep(
        &mut [&mut live, &mut clamped, &mut unedited],
        None,
        0,
        1,
        "warm-up",
    );
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, 24.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "band +24 dB");
    let vcas = &live.ready.as_ref().expect("ready").vcas;
    assert_eq!(vcas.effective_db(drums as usize, 0), 24.0, "drums clamps");
    vca_lockstep(
        &mut [&mut live, &mut clamped, &mut unedited],
        Some(1),
        1,
        2,
        "band +24 dB vs booted there",
    );
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, 0.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "band 0 dB");
    let vcas = &live.ready.as_ref().expect("ready").vcas;
    assert_eq!(vcas.effective_db(drums as usize, 0), 20.0, "drums returns");
    assert!(vca_lockstep(
        &mut [&mut live, &mut clamped, &mut unedited],
        Some(2),
        3,
        2,
        "band back to 0 dB vs unedited"
    ));

    let muted = |band_muted: bool| {
        vca_ride_host(
            &vca_ride_model(
                &[("drums", [0.0; 2], [true; 2])],
                &[("band", [-3.0; 2], [band_muted; 2], BAND)],
            ),
            16,
        )
    };
    let mut live = muted(true);
    let mut own_only = muted(false);
    vca_lockstep(&mut [&mut live, &mut own_only], None, 0, 1, "warm-up");
    stage_vca(&mut live, 0, COMMAND_VCA_MUTE, 0, 2, 0.0, 0);
    assert_eq!(live.submit_commands(1), RESULT_OK, "un-mute band");
    let solo = &live.ready.as_ref().expect("ready").solo;
    for lane in 0..2 {
        assert!(
            solo.effective_mute(drums as usize, lane),
            "drums lane {lane}"
        );
        assert!(
            !solo.effective_mute(VCA_BASS as usize, lane),
            "bass lane {lane}"
        );
    }
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [true; 2]],
        "follow mirror"
    );
    assert!(vca_lockstep(
        &mut [&mut live, &mut own_only],
        Some(1),
        1,
        3,
        "un-muted band vs drums' own mute only"
    ));
}

/// Issue #1245 gate 3: no redundant record.
///
/// * `drums` and `bass` sit at -144 dB in VCA `band` (-10 dB), so both clamp at -144 dB. Riding
///   `band` to -20 dB, 0 dB and -144 dB (both lanes, then the left alone), each with a 480-sample
///   ramp, changes no member's effective value: no fader record is staged, and the output stays
///   bit-identical to a twin that received nothing.
/// * With `drums` and `bass` own-muted, muting `band` stages no mute and no send record, and the
///   output stays bit-identical to the twin.
///
/// Test value: red if composition re-emits an unchanged target, which restarts a settled ramp and
/// moves its bits.
#[test]
fn a_vca_move_that_changes_no_effective_value_stages_nothing() {
    const BAND: &[&str] = &["bass", "drums"];
    let clamped = vca_ride_model(
        &[
            ("drums", [-144.0; 2], [false; 2]),
            ("bass", [-144.0; 2], [false; 2]),
        ],
        &[("band", [-10.0; 2], [false; 2], BAND)],
    );
    let mut live = vca_ride_host(&clamped, 16);
    let mut twin = vca_ride_host(&clamped, 16);
    vca_lockstep(&mut [&mut live, &mut twin], None, 0, 1, "warm-up");
    let rooms = (follow_fader_room(&live), send_queue_room(&live));
    let mut block = 1;
    for (channel, offset) in [(2, -20.0), (2, 0.0), (2, -144.0), (0, -3.0)] {
        stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, channel, offset, 480);
        assert_eq!(live.submit_commands(1), RESULT_OK, "band {offset} dB");
        assert_eq!(
            (follow_fader_room(&live), send_queue_room(&live)),
            rooms,
            "band {offset} dB stages nothing"
        );
        assert!(vca_lockstep(
            &mut [&mut live, &mut twin],
            Some(1),
            block,
            2,
            &format!("band {offset} dB vs twin")
        ));
        block += 2;
    }

    let muted = vca_ride_model(
        &[
            ("drums", [0.0; 2], [true; 2]),
            ("bass", [-3.0; 2], [true; 2]),
        ],
        &[("band", [-10.0; 2], [false; 2], BAND)],
    );
    let mut live = vca_ride_host(&muted, 16);
    let mut twin = vca_ride_host(&muted, 16);
    vca_lockstep(&mut [&mut live, &mut twin], None, 0, 1, "warm-up");
    let rooms = (follow_fader_room(&live), send_queue_room(&live));
    for (channel, on) in [(2, true), (0, false), (2, false)] {
        stage_vca(
            &mut live,
            0,
            COMMAND_VCA_MUTE,
            0,
            channel,
            if on { 1.0 } else { 0.0 },
            480,
        );
        assert_eq!(live.submit_commands(1), RESULT_OK, "band mute {on}");
        assert_eq!(
            (follow_fader_room(&live), send_queue_room(&live)),
            rooms,
            "a VCA mute of user-muted members stages nothing"
        );
    }
    assert!(vca_lockstep(
        &mut [&mut live, &mut twin],
        Some(1),
        1,
        3,
        "muted band over muted members vs twin"
    ));
}

/// Issue #1245 gate 4: all or nothing, at queue depth 4.
///
/// * With `bass`'s fader queue full, a ride of VCA `band` (`bass`, `drums`) is typed backpressure
///   at its wire index: nothing is pushed (not even `drums`' record) and the VCA, strip-mute and
///   send mirrors are unchanged.
/// * With `drums-verb`'s send queue full, a mute of `band`, whose follow records would land there,
///   is refused the same way.
///
/// After a render drains the queues, the identical submission is admitted and renders
/// bit-identically to a twin that never received the refused one.
///
/// Test value: red if any record is pushed, or a mirror commits, before every destination's room
/// is checked.
#[test]
fn a_vca_batch_that_overfills_a_queue_is_refused_whole() {
    let model = vca_ride_model(
        &[("bass", [-2.0, -5.0], [false; 2])],
        &[("band", [-3.0; 2], [false; 2], &["bass", "drums"])],
    );
    let mut live = vca_ride_host(&model, 4);
    let mut twin = vca_ride_host(&model, 4);
    vca_lockstep(&mut [&mut live, &mut twin], None, 0, 1, "warm-up");
    for host in [&mut live, &mut twin] {
        for index in 0..4 {
            stage_fader_db(host, index, VCA_BASS, 2, -2.0 - index as f32, 0);
        }
        assert_eq!(
            host.submit_commands(4),
            RESULT_OK,
            "fill bass's fader queue"
        );
    }
    assert_eq!(follow_fader_room(&live)[VCA_BASS as usize], 0);
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, -9.0, 0);
    assert_vca_refusal(
        &mut live,
        1,
        RESULT_BACKPRESSURE,
        COMMAND_REASON_BACKPRESSURE,
        0,
        "a ride into a full member queue",
    );
    vca_lockstep(
        &mut [&mut live, &mut twin],
        Some(1),
        1,
        1,
        "after the refusal",
    );
    for host in [&mut live, &mut twin] {
        stage_vca(host, 0, COMMAND_VCA_FADER_DB, 0, 2, -9.0, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "the identical ride");
    }
    assert!(vca_lockstep(
        &mut [&mut live, &mut twin],
        Some(1),
        2,
        2,
        "the admitted ride vs twin"
    ));

    for host in [&mut live, &mut twin] {
        for index in 0..4 {
            stage_send(
                host,
                index,
                COMMAND_ROUTE_GAIN_DB,
                1,
                0,
                [-1.0 - index as f32, 0.0, 0.0, 0.0],
            );
        }
        assert_eq!(
            host.submit_commands(4),
            RESULT_OK,
            "fill drums-verb's queue"
        );
    }
    assert_eq!(send_queue_room(&live)[1], 0);
    stage_vca(&mut live, 0, COMMAND_VCA_MUTE, 0, 2, 1.0, 64);
    assert_vca_refusal(
        &mut live,
        1,
        RESULT_BACKPRESSURE,
        COMMAND_REASON_BACKPRESSURE,
        0,
        "a mute whose follow record meets a full send queue",
    );
    vca_lockstep(
        &mut [&mut live, &mut twin],
        Some(1),
        4,
        1,
        "after the refusal",
    );
    for host in [&mut live, &mut twin] {
        stage_vca(host, 0, COMMAND_VCA_MUTE, 0, 2, 1.0, 64);
        assert_eq!(host.submit_commands(1), RESULT_OK, "the identical mute");
        assert_eq!(follow_lanes(host), [[true; 2], [true; 2]]);
    }
    assert!(vca_lockstep(
        &mut [&mut live, &mut twin],
        Some(1),
        5,
        3,
        "the admitted mute vs twin"
    ));
}

/// Issue #1245 gate 5: a VCA mute flows to the members' following sends and survives solo. In
/// #1224's follow fixture with `drums` and `bass` in VCA `band` (-3 dB): muting `band` silences
/// both members and both pre-fader `follows_mute` sends, bit-identically to a host booted with
/// `band` muted; soloing `vocal` (sent to both) and un-soloing it keeps them silent; un-muting
/// `band` restores them bit-identically to an unedited host. Every gesture ramps 480 samples.
///
/// Test value: red if the VCA mute term does not reach the follow composition, if un-soloing
/// clears it, or if the live path and preparation disagree about a VCA-muted member's send.
#[test]
fn a_vca_mute_silences_member_sends_and_survives_solo() {
    let band = |muted: bool| {
        vca_ride_host(
            &vca_ride_model(&[], &[("band", [-3.0; 2], [muted; 2], &["bass", "drums"])]),
            16,
        )
    };
    let mut live = band(false);
    let mut muted = band(true);
    let mut open = band(false);
    assert_eq!(follow_lanes(&muted), [[true; 2], [true; 2]], "prepared");
    let hosts = |live: &mut AudioWorkletEngineHost,
                 muted: &mut AudioWorkletEngineHost,
                 open: &mut AudioWorkletEngineHost,
                 compare: Option<usize>,
                 first: u64,
                 blocks: u64,
                 what: &str| {
        vca_lockstep(&mut [live, muted, open], compare, first, blocks, what)
    };
    hosts(&mut live, &mut muted, &mut open, None, 0, 1, "warm-up");
    stage_vca(&mut live, 0, COMMAND_VCA_MUTE, 0, 2, 1.0, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "mute band");
    assert_eq!(
        follow_lanes(&live),
        [[true; 2], [true; 2]],
        "both sends follow"
    );
    hosts(&mut live, &mut muted, &mut open, None, 1, 4, "mute ramp");
    assert!(hosts(
        &mut live,
        &mut muted,
        &mut open,
        Some(1),
        5,
        2,
        "muted band vs booted muted"
    ));
    for (on, first) in [(true, 7), (false, 13)] {
        for host in [&mut live, &mut muted] {
            stage_solo(host, 0, SOLO_FOLLOW_VOCAL, on, 480);
            assert_eq!(host.submit_commands(1), RESULT_OK, "solo vocal {on}");
            assert_eq!(follow_lanes(host), [[true; 2], [true; 2]], "solo {on}");
        }
        hosts(
            &mut live,
            &mut muted,
            &mut open,
            None,
            first,
            4,
            "solo ramp",
        );
        assert!(hosts(
            &mut live,
            &mut muted,
            &mut open,
            Some(1),
            first + 4,
            2,
            &format!("solo vocal {on}: muted band vs booted muted")
        ));
    }
    stage_vca(&mut live, 0, COMMAND_VCA_MUTE, 0, 2, 0.0, 480);
    assert_eq!(live.submit_commands(1), RESULT_OK, "un-mute band");
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [false; 2]],
        "both sends reopen"
    );
    hosts(
        &mut live,
        &mut muted,
        &mut open,
        None,
        19,
        4,
        "un-mute ramp",
    );
    assert!(hosts(
        &mut live,
        &mut muted,
        &mut open,
        Some(2),
        23,
        3,
        "un-muted band vs unedited"
    ));
}

/// One VCA record shape: `(what, kind, rack, channel, effect, parameter, values, reason)`.
type VcaShape = (&'static str, u32, u8, u8, u32, u32, [f32; 4], u32);

/// Issue #1245 gate 6: addressing and shape.
///
/// * A VCA index at or past the VCA count -- and index 0 in a session without VCAs -- refuses
///   `unknownVca` at its wire index and stages nothing, even behind a valid record.
/// * A wrong `rack`, a nonzero `values[1..]`, a bad `channel`, or a nonzero `effect_index` or
///   `parameter_id` is `malformed`; an offset of 24.5 or -144.5 dB, or a mute of 0.5, is `domain`;
///   the domain edges are admitted.
///
/// (A host without live controls has no command staging to submit from, so its
/// `unsupportedKind` arm, kept as the route kinds keep theirs, is not reachable from the wire.)
///
/// Test value: red if a VCA index is checked against another table, a bad index is refused with a
/// reason that misnames it, or the shape rules differ from kinds 3 and 4's plus the route kinds'
/// zero `effect_index` and `parameter_id`.
#[test]
fn vca_records_are_addressed_and_shape_checked() {
    let model = vca_ride_model(&[], &[("band", [-3.0; 2], [false; 2], &["bass", "drums"])]);
    let mut host = vca_ride_host(&model, 16);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    for kind in [COMMAND_VCA_FADER_DB, COMMAND_VCA_MUTE] {
        for vca in [1_u32, 5, u32::MAX] {
            stage_vca(&mut host, 0, kind, 0, 2, 1.0, 0);
            stage_vca(&mut host, 1, kind, vca, 2, 1.0, 0);
            assert_vca_refusal(
                &mut host,
                2,
                RESULT_INVALID_ARGUMENT,
                COMMAND_REASON_UNKNOWN_VCA,
                1,
                &format!("kind {kind} at VCA {vca}"),
            );
        }
    }
    let one = [1.0, 0.0, 0.0, 0.0];
    let shapes: [VcaShape; 16] = [
        (
            "an inserts rack",
            COMMAND_VCA_FADER_DB,
            1,
            2,
            0,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "a console rack",
            COMMAND_VCA_MUTE,
            3,
            2,
            0,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "channel 3",
            COMMAND_VCA_FADER_DB,
            255,
            3,
            0,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "channel 255",
            COMMAND_VCA_MUTE,
            255,
            255,
            0,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "values[1]",
            COMMAND_VCA_FADER_DB,
            255,
            2,
            0,
            0,
            [1.0, 1.0, 0.0, 0.0],
            COMMAND_REASON_MALFORMED,
        ),
        (
            "values[3]",
            COMMAND_VCA_MUTE,
            255,
            0,
            0,
            0,
            [1.0, 0.0, 0.0, 1.0],
            COMMAND_REASON_MALFORMED,
        ),
        (
            "an effect word",
            COMMAND_VCA_FADER_DB,
            255,
            2,
            1,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "an effect word",
            COMMAND_VCA_MUTE,
            255,
            2,
            1,
            0,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "a parameter word",
            COMMAND_VCA_FADER_DB,
            255,
            1,
            0,
            1,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "a parameter word",
            COMMAND_VCA_MUTE,
            255,
            1,
            0,
            1,
            one,
            COMMAND_REASON_MALFORMED,
        ),
        (
            "24.5 dB",
            COMMAND_VCA_FADER_DB,
            255,
            2,
            0,
            0,
            [24.5, 0.0, 0.0, 0.0],
            COMMAND_REASON_DOMAIN,
        ),
        (
            "-144.5 dB",
            COMMAND_VCA_FADER_DB,
            255,
            0,
            0,
            0,
            [-144.5, 0.0, 0.0, 0.0],
            COMMAND_REASON_DOMAIN,
        ),
        (
            "a mute of 0.5",
            COMMAND_VCA_MUTE,
            255,
            2,
            0,
            0,
            [0.5, 0.0, 0.0, 0.0],
            COMMAND_REASON_DOMAIN,
        ),
        (
            "a mute of 2",
            COMMAND_VCA_MUTE,
            255,
            1,
            0,
            0,
            [2.0, 0.0, 0.0, 0.0],
            COMMAND_REASON_DOMAIN,
        ),
        (
            "a mute of -1",
            COMMAND_VCA_MUTE,
            255,
            0,
            0,
            0,
            [-1.0, 0.0, 0.0, 0.0],
            COMMAND_REASON_DOMAIN,
        ),
        (
            "an offset with a mute's value word",
            COMMAND_VCA_FADER_DB,
            255,
            2,
            0,
            0,
            [0.0, 0.0, 1.0, 0.0],
            COMMAND_REASON_MALFORMED,
        ),
    ];
    for (what, kind, rack, channel, effect, parameter, values, reason) in shapes {
        stage_vca(&mut host, 0, COMMAND_VCA_FADER_DB, 0, 2, -1.0, 0);
        stage_command(
            &mut host, 1, kind, rack, channel, 0, effect, parameter, 0, values,
        );
        assert_vca_refusal(&mut host, 2, RESULT_INVALID_ARGUMENT, reason, 1, what);
    }
    for (block, (kind, value)) in [
        (COMMAND_VCA_FADER_DB, 24.0),
        (COMMAND_VCA_FADER_DB, -144.0),
        (COMMAND_VCA_MUTE, 1.0),
        (COMMAND_VCA_MUTE, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        for channel in 0..3 {
            stage_vca(&mut host, 0, kind, 0, channel, value, 0);
            assert_eq!(
                host.submit_commands(1),
                RESULT_OK,
                "kind {kind} {value} on channel {channel}"
            );
        }
        follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 1 + block as u64, None);
    }

    let mut bare = vca_ride_host(&vca_ride_model(&[], &[]), 16);
    follow_render(&mut bare, &SOLO_FOLLOW_TRACKS, 0, None);
    for kind in [COMMAND_VCA_FADER_DB, COMMAND_VCA_MUTE] {
        stage_vca(&mut bare, 0, kind, 0, 2, 0.0, 0);
        assert_vca_refusal(
            &mut bare,
            1,
            RESULT_INVALID_ARGUMENT,
            COMMAND_REASON_UNKNOWN_VCA,
            0,
            "a session without VCAs",
        );
    }
}

/// Issue #1245 gate 7 (D4): the decode staging holds a full batch and its VCA records. Eight
/// tracks, each with a compressor insert and an own fader of `[-1, -4]` dB, all in VCA `all`:
/// 254 `channel = both` threshold records (508 entries), one ride of `all` (sixteen fader records,
/// both lanes of every member, which differ) and one solo of `t0` (seven coalesced mute records)
/// need 531 entries, past the 528 that `2 * MAXIMUM_COMMAND_RECORDS + 2 * strip_count` holds.
///
/// Test value: red if the staging is not grown by the reached strips, so a full batch plus its
/// VCA records is refused `malformed` by the staging bound.
#[test]
fn the_decode_staging_holds_a_full_batch_and_its_vca_records() {
    const TRACKS: [&str; 8] = ["t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7"];
    let ids: Vec<String> = TRACKS.iter().map(|track| format!("{track}-main")).collect();
    let routes: Vec<FollowRoute<'_>> = ids
        .iter()
        .zip(TRACKS)
        .map(|(id, track)| follow_main(id, track, false))
        .collect();
    let (_, _, _, compressor, _) = strip_base();
    let document = follow_document(&TRACKS, &[], &routes, &[], None);
    let mut model = parse_session_json(&document).expect("staging session");
    for track in &mut model.tracks {
        track.inserts.effects = vec![compressor.clone()];
        (track.fader.left_db, track.fader.right_db) = (-1.0, -4.0);
    }
    model.vcas = vec![session::Vca {
        id: strip_id("all"),
        fader: session::DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: TRACKS.iter().map(|track| strip_id(track)).collect(),
    }];
    let document = canonical_session_json(&model).expect("staging session canonicalizes");
    let mut host = strip_boot(&document, strip_options(128, 0, 0));
    assert_eq!(
        host.command_staging_entries(),
        Some(MAXIMUM_COMMAND_RECORDS as usize * 2 + TRACKS.len() * 2 + TRACKS.len() * 2),
    );
    follow_render(&mut host, &TRACKS, 0, None);
    let room = follow_fader_room(&host);
    let records = MAXIMUM_COMMAND_RECORDS as usize;
    for index in 0..records - 2 {
        stage_command(
            &mut host,
            index,
            COMMAND_EFFECT_PARAM,
            1,
            2,
            (index % TRACKS.len()) as u32,
            0,
            1,
            0,
            [-12.0, 0.0, 0.0, 0.0],
        );
    }
    stage_vca(&mut host, records - 2, COMMAND_VCA_FADER_DB, 0, 2, -6.0, 64);
    stage_solo(&mut host, records - 1, 0, true, 64);
    assert_eq!(
        host.submit_commands(MAXIMUM_COMMAND_RECORDS),
        RESULT_OK,
        "reason {}",
        host.command_report().reason,
    );
    assert_eq!(host.command_report().admitted, MAXIMUM_COMMAND_RECORDS);
    let owed: Vec<usize> = room
        .iter()
        .enumerate()
        .map(|(strip, room)| room - if strip == 0 { 2 } else { 3 })
        .collect();
    assert_eq!(
        follow_fader_room(&host),
        owed,
        "two fader and one mute record each"
    );
}

/// Issue #1245 gate 8: VCA rides, member moves and VCA mutes with follow records admit, and the
/// next block renders, allocating and freeing nothing.
///
/// Test value: red if VCA admission or its passes allocate on the audio thread.
#[test]
fn vca_rides_and_mutes_admit_and_render_without_allocating() {
    let model = vca_ride_model(
        &[("bass", [-2.0, -5.0], [false; 2])],
        &[
            ("band", [-3.0; 2], [false; 2], &["bass", "drums", "fx"]),
            ("fx", [1.5; 2], [false; 2], &["room"]),
        ],
    );
    let mut host = vca_ride_host(&model, 64);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 0, None);
    // Warm every path once outside the measurement.
    stage_vca(&mut host, 0, COMMAND_VCA_FADER_DB, 0, 2, -9.0, 480);
    stage_vca(&mut host, 1, COMMAND_VCA_MUTE, 0, 2, 1.0, 480);
    stage_fader_db(&mut host, 2, SOLO_FOLLOW_DRUMS, 2, -1.0, 0);
    assert_eq!(host.submit_commands(3), RESULT_OK);
    follow_render(&mut host, &SOLO_FOLLOW_TRACKS, 1, None);
    stage_vca(&mut host, 0, COMMAND_VCA_MUTE, 0, 2, 0.0, 480);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    for (feed, id) in SOLO_FOLLOW_TRACKS.iter().enumerate() {
        submit_strip_source(&mut host, id, 2, &strip_planes(feed as u64, 2));
    }
    stage_vca(&mut host, 0, COMMAND_VCA_FADER_DB, 1, 0, 6.0, 480);
    stage_fader_db(&mut host, 1, SOLO_FOLLOW_DRUMS, 2, 3.0, 0);
    stage_vca(&mut host, 2, COMMAND_VCA_MUTE, 0, 1, 1.0, 128);
    stage_solo(&mut host, 3, SOLO_FOLLOW_VOCAL, true, 64);
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| {
            (host.submit_commands(4), host.render_next())
        });
    assert_eq!(admission, RESULT_OK, "the VCA batch is admitted");
    assert_eq!(render, RESULT_OK);
    assert_eq!(follow_lanes(&host), [[true; 2], [true; 2]]);
    assert_eq!(allocations, 0, "admission/render allocated");
    assert_eq!(deallocations, 0, "admission/render freed");
}

/// Issue #1245 D4: the browser's exact retained budget charges the live VCA state. Against the
/// same session without VCAs, a session with two nested VCAs over `bass`, `drums` and `room`
/// retains exactly the document and ID-staging growth, the compiled model's growth, every byte of
/// an independently built `LiveVcaState` (tables, mirrors and shadow) and two decoded-command
/// entries per reached strip; the metadata row moves by all but the document rows. One byte below
/// the exact total refuses.
///
/// Test value: red if the browser leaves the VCA state or the staging growth out of its exact
/// retained report, so a budget that cannot hold them admits.
#[test]
fn the_exact_retained_budget_charges_the_vca_state() {
    let with = canonical_session_json(&vca_ride_model(
        &[],
        &[
            ("band", [-3.0; 2], [false; 2], &["bass", "drums", "fx"]),
            ("fx", [1.5; 2], [false; 2], &["room"]),
        ],
    ))
    .expect("VCA session canonicalizes");
    let without =
        canonical_session_json(&vca_ride_model(&[], &[])).expect("plain session canonicalizes");
    let resources = |document: &str| *strip_boot(document, strip_options(16, 0, 0)).resources();
    let (with_rows, without_rows) = (resources(&with), resources(&without));
    let state = {
        let parsed = parse_host_session(&with).expect("VCA session parse");
        let compiled = compile_host_model(
            &parsed,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("VCA session compile");
        LiveVcaState::try_new(compiled.normalized_model()).expect("VCA state")
    };
    assert_eq!(state.reached_strip_count(), 3);
    let vca = state.retained_bytes();
    assert!(vca > 0);
    let staging = 2 * 3 * size_of::<StagedCommand>() as u64;
    let model = send_prepare_report(&with, 16).session_model_bytes
        - send_prepare_report(&without, 16).session_model_bytes;
    let document_rows = (with_rows.session_document_bytes - without_rows.session_document_bytes)
        + (with_rows.id_staging_bytes - without_rows.id_staging_bytes);
    assert_eq!(
        with_rows.bridge_retained_bytes - without_rows.bridge_retained_bytes,
        document_rows + model + vca + staging,
        "the VCA state's whole retained share"
    );
    assert_eq!(
        with_rows.bridge_metadata_bytes - without_rows.bridge_metadata_bytes,
        model + vca + staging,
        "the VCA state's whole metadata share"
    );
    assert!(with_rows.largest_bridge_allocation_bytes >= state.largest_allocation_bytes());

    let exact = exact_retained_report_total(&with_rows);
    let budget = |maximum_memory_bytes: u64| {
        AudioWorkletEngineHost::boot(
            with.as_bytes(),
            WebBootOptions {
                maximum_memory_bytes,
                ..strip_options(16, 0, 0)
            },
        )
    };
    assert!(
        budget(exact).is_ok(),
        "the exact retained total is admitted"
    );
    let refused = budget(exact - 1)
        .err()
        .expect("one byte below the exact retained total refuses");
    assert_eq!(refused.result(), RESULT_REFUSED_BUDGET);
}

/// A session of `tracks` tracks (each with its own source and a main route) and a chain of `chain`
/// VCAs: `v000` holds the first `chained` tracks and each next VCA holds the one before it, so each
/// of them is reached by every VCA (`chained * chain` pairs); the last VCA of the chain also holds
/// the remaining tracks directly (one pair each). `extra` more VCAs, after the chain in ID order,
/// each hold track `t000` alone.
fn vca_bound_document(tracks: usize, chain: usize, chained: usize, extra: usize) -> String {
    let names: Vec<String> = (0..tracks).map(|track| format!("t{track:03}")).collect();
    let ids: Vec<&str> = names.iter().map(String::as_str).collect();
    let mains: Vec<String> = names.iter().map(|track| format!("{track}-main")).collect();
    let routes: Vec<FollowRoute<'_>> = mains
        .iter()
        .zip(&ids)
        .map(|(id, track)| follow_main(id, track, false))
        .collect();
    let mut model = parse_session_json(&follow_document(&ids, &[], &routes, &[], None))
        .expect("bound session parses");
    let fader = session::DualMonoFader {
        left_db: -0.5,
        right_db: -0.5,
        left_mute: false,
        right_mute: false,
    };
    model.vcas = (0..chain)
        .map(|index| session::Vca {
            id: strip_id(&format!("v{index:03}")),
            fader: fader.clone(),
            members: {
                let mut members: Vec<session::StableId> = if index == 0 {
                    ids[..chained].iter().map(|track| strip_id(track)).collect()
                } else {
                    vec![strip_id(&format!("v{:03}", index - 1))]
                };
                if index + 1 == chain {
                    members.extend(ids[chained..].iter().map(|track| strip_id(track)));
                }
                members
            },
        })
        .chain((0..extra).map(|index| session::Vca {
            id: strip_id(&format!("w{index:03}")),
            fader: fader.clone(),
            members: vec![strip_id("t000")],
        }))
        .collect();
    canonical_session_json(&model).expect("bound session canonicalizes")
}

/// Issue #1245 amendment A1: the browser bounds per-command VCA work at boot.
///
/// * 64 tracks under a chain of 256 VCAs is exactly [`MAXIMUM_BROWSER_VCA_REACH_PAIRS`] (16,384)
///   pairs, every track reached by [`MAXIMUM_BROWSER_VCAS`] VCAs: it boots, and the worst batch
///   for it -- a ride and a mute of the top VCA (every pair recomposed, every strip's mute and
///   follow records) and 254 member moves on two tracks every VCA reaches -- is admitted and
///   renders. Its release-mode time is
///   `a_vca_batch_at_the_bound_fits_a_quantum_in_release`'s.
/// * One pair more (a 65th track held by the top VCA alone) is refused `web.vca.reach_pairs`, and
///   257 VCAs (each holding one track) `web.vca.maximum_vcas`, both `RESULT_REFUSED_BUDGET`,
///   before any table is built; 256 VCAs boot.
///
/// The boot projection of the VCA state's retained bytes equals what the state keeps, and its
/// transient projection covers the reach lists construction holds.
///
/// Test value: red if the bound is not enforced, counts pairs differently from the reach the
/// live state builds, mis-projects the state's bytes, or if the worst batch at the bound is
/// refused.
#[test]
fn the_browser_bounds_vca_reach_and_admits_a_worst_batch_at_the_bound() {
    let refusal = |document: &str| {
        AudioWorkletEngineHost::boot(document.as_bytes(), strip_options(256, 0, 0))
            .err()
            .expect("past the bound refuses")
    };
    for (document, code) in [
        (vca_bound_document(65, 256, 64, 0), "web.vca.reach_pairs"),
        (vca_bound_document(4, 0, 0, 257), "web.vca.maximum_vcas"),
    ] {
        let refused = refusal(&document);
        assert_eq!(
            refused.result(),
            RESULT_REFUSED_BUDGET,
            "{code}: {}",
            String::from_utf8_lossy(refused.diagnostic())
        );
        assert_eq!(
            refused.diagnostic(),
            fixed_diagnostic(code).as_slice(),
            "{code}"
        );
    }
    let fits = vca_bound_document(4, 0, 0, 256);
    assert!(
        AudioWorkletEngineHost::boot(fits.as_bytes(), strip_options(256, 0, 0)).is_ok(),
        "256 VCAs boot"
    );

    let document = vca_bound_document(64, 256, 64, 0);
    let parsed = parse_session_json(&document).expect("bound session parses");
    let shape = browser_vca_shape(&parsed).expect("at the bound");
    assert_eq!(shape.pairs, MAXIMUM_BROWSER_VCA_REACH_PAIRS);
    let mut host = strip_boot(&document, strip_options(256, 0, 0));
    {
        let vcas = &host.ready.as_ref().expect("ready").vcas;
        let pairs: usize = (0..vcas.vca_count())
            .map(|vca| vcas.reached_by(vca).len())
            .sum();
        assert_eq!(pairs as u64, shape.pairs, "the live state's pairs");
        // The boot projection charges exactly what the state keeps, and at least the reach lists
        // its construction holds twice over.
        assert_eq!(shape.retained_bytes(), Some(vcas.retained_bytes()));
        let lists: u64 = parsed
            .vca_reach()
            .iter()
            .map(|list| (size_of::<Vec<usize>>() + list.capacity() * size_of::<usize>()) as u64)
            .sum();
        let faders = 64 * size_of::<session::EffectiveStripFader>() as u64;
        assert!(shape.transient_bytes().expect("projection") >= 2 * lists + faders);
    }
    let elapsed = vca_worst_batch_at_the_bound(&mut host);
    eprintln!("#1245 A1: a worst batch at the VCA bound admitted in {elapsed:?}");
}

/// Admit the worst batch for [`vca_bound_document`]`(64, 256, 64, 0)` on a booted `host` -- a ride
/// and a mute of the top VCA, then 254 member moves on the two tracks every VCA reaches -- and
/// render a block on either side. Returns the admission's wall time.
fn vca_worst_batch_at_the_bound(host: &mut AudioWorkletEngineHost) -> std::time::Duration {
    let tracks: Vec<String> = (0..64).map(|track| format!("t{track:03}")).collect();
    let ids: Vec<&str> = tracks.iter().map(String::as_str).collect();
    follow_render(host, &ids, 0, None);
    let top = 255;
    stage_vca(host, 0, COMMAND_VCA_FADER_DB, top, 0, -7.0, 128);
    stage_vca(host, 1, COMMAND_VCA_MUTE, top, 2, 1.0, 128);
    for index in 2..MAXIMUM_COMMAND_RECORDS as usize {
        stage_fader_db(
            host,
            index,
            (index % 2) as u32,
            (index % 3) as u8,
            -(index as f32) / 32.0,
            0,
        );
    }
    let started = std::time::Instant::now();
    let result = host.submit_commands(MAXIMUM_COMMAND_RECORDS);
    let elapsed = started.elapsed();
    assert_eq!(result, RESULT_OK, "reason {}", host.command_report().reason);
    follow_render(host, &ids, 1, None);
    elapsed
}

/// Release-mode half of the bound test above (issue #1245 amendment A1): the worst batch at the
/// VCA bound is admitted in less than one 128-frame quantum at 48 kHz (2.67 ms) in the shipped
/// profile. A debug build is several times slower, so this runs only in release, nightly,
/// `--ignored` (the `maximum_document_dense_invalid_boot_finishes_under_one_second_in_release`
/// convention).
///
/// Test value: red if a batch at the bound costs the worklet a quantum or more, which no per-PR
/// test measures.
#[test]
#[ignore = "release-mode budget; runs nightly"]
fn a_vca_batch_at_the_bound_fits_a_quantum_in_release() {
    let document = vca_bound_document(64, 256, 64, 0);
    let mut host = strip_boot(&document, strip_options(256, 0, 0));
    let elapsed = vca_worst_batch_at_the_bound(&mut host);
    let quantum = std::time::Duration::from_secs_f64(128.0 / 48_000.0);
    assert!(elapsed < quantum, "{elapsed:?} is not under one quantum");
}

/// Issue #1245 D3 (amendment A1): a kind 4 after a kind 17 in the same batch composes with the
/// new VCA mute. `band` holds `drums` alone; one batch mutes `band` over 480 samples and then
/// sends `drums` an explicit un-mute with smoothing 0. `drums` is already VCA-muted when the kind
/// 4 composes, so its record carries `muted` and snaps it (and its following send) muted at once:
/// the first block after the batch is bit-identical to a host booted with `band` muted.
///
/// Test value: red if the strip-mute owner's VCA term is not refreshed before a later kind 4 in
/// the batch composes, so the coalesced record ramps the member over the VCA's window instead.
#[test]
fn a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it() {
    let band = |muted: bool| {
        vca_ride_host(
            &vca_ride_model(&[], &[("band", [0.0; 2], [muted; 2], &["drums"])]),
            16,
        )
    };
    let mut live = band(false);
    let mut muted = band(true);
    vca_lockstep(&mut [&mut live, &mut muted], None, 0, 1, "warm-up");
    stage_vca(&mut live, 0, COMMAND_VCA_MUTE, 0, 2, 1.0, 480);
    stage_lane_mute(&mut live, 1, SOLO_FOLLOW_DRUMS, 2, false, 0);
    assert_eq!(
        live.submit_commands(2),
        RESULT_OK,
        "mute band, then un-mute drums"
    );
    assert_eq!(
        follow_lanes(&live),
        [[false; 2], [true; 2]],
        "drums-verb follows"
    );
    assert!(vca_lockstep(
        &mut [&mut live, &mut muted],
        Some(1),
        1,
        3,
        "the batch's first block vs booted muted"
    ));
}

/// The test's own effective mute of `strip`'s `lane` with no solo: its own mute or any reaching
/// VCA's, from its own top-down reach.
fn vca_reference_mute(model: &session::SessionModel, strip: usize, lane: usize) -> bool {
    let id = VCA_STRIPS[strip];
    let own = model
        .tracks
        .iter()
        .map(|track| (track.id.as_str(), &track.fader))
        .chain(model.submixes.iter().map(|s| (s.id.as_str(), &s.fader)))
        .find(|(candidate, _)| *candidate == id)
        .expect("strip")
        .1;
    [own.left_mute, own.right_mute][lane]
        || vca_reference_reach(&model.vcas, id).iter().any(|index| {
            let vca = &model.vcas[*index].fader;
            [vca.left_mute, vca.right_mute][lane]
        })
}

/// A random VCA forest for the ramp differential: two to six VCAs over four levels (nested, with
/// diamonds), tracks and submixes as members, offsets that include both domain edges, drawn lane
/// mutes; own faders drawn across the domain.
fn vca_ramp_model(draw: &mut SendDraw) -> session::SessionModel {
    let mut model = vca_ride_model(&[], &[]);
    for strip in VCA_STRIPS {
        let mut own = || {
            if draw.below(6) == 0 {
                draw.uniform(-144.0, 24.0)
            } else {
                draw.uniform(-12.0, 6.0)
            }
        };
        let (left_db, right_db) = (own(), own());
        let (left_mute, right_mute) = (draw.below(8) == 0, draw.below(8) == 0);
        let fader = vca_strip_fader(&mut model, strip);
        (fader.left_db, fader.right_db) = (left_db, right_db);
        (fader.left_mute, fader.right_mute) = (left_mute, right_mute);
    }
    let ids = ["va", "vb", "vc", "vd", "ve", "vf"];
    let count = 2 + draw.below(5) as usize;
    let levels: Vec<u64> = (0..count).map(|_| draw.below(4)).collect();
    model.vcas = (0..count)
        .map(|i| {
            let mut members: Vec<session::StableId> = Vec::new();
            for strip in VCA_STRIPS {
                if draw.below(5) < 2 {
                    members.push(strip_id(strip));
                }
            }
            for j in 0..count {
                if levels[j] > levels[i] && draw.below(2) == 0 {
                    members.push(strip_id(ids[j]));
                }
            }
            let mut offset = || match draw.below(8) {
                0 => 24.0,
                1 => -144.0,
                2 => draw.uniform(-144.0, 24.0),
                _ => draw.uniform(-12.0, 12.0),
            };
            let (left_db, right_db) = (offset(), offset());
            session::Vca {
                id: strip_id(ids[i]),
                fader: session::DualMonoFader {
                    left_db,
                    right_db,
                    left_mute: draw.below(5) == 0,
                    right_mute: draw.below(5) == 0,
                },
                members,
            }
        })
        .collect();
    parse_session_json(&canonical_session_json(&model).expect("canonical"))
        .expect("canonical session parses")
}

/// Seeds of the VCA ramp differential: 40 seeds take about a second in debug.
const VCA_RAMP_SEEDS: u64 = 8;

/// Issue #1245 (verdict MINOR-1, the verifier's probe `sol_probe_ramps_match_direct_member_moves`):
/// every record a VCA ride or mute derives carries the gesture's window. Random VCA forests
/// ([`vca_ramp_model`]); six batches each of a ride (one lane or both, ramp 64 to 500 samples), a
/// mute (one lane or both, its own ramp from the same set), or both. The reference is a VCA-free
/// host whose own faders and mutes are the live host's effective values; for every lane whose
/// effective value a batch changed it is told directly a kind 3 at the ride's ramp, or a kind 4 at
/// the mute's ramp. Every block from the batch through the ramp, not only settled blocks, is
/// bit-identical, and the two hosts' follow mirrors agree.
///
/// Test value: red if the VCA fader pass stages its members' records without the ride's ramp, or
/// if a kind 17 leaves the coalesced mute window unset (members and their following sends then
/// hard-switch, an audible click), which every settled or ramp-0 test misses.
#[test]
fn a_vca_ride_and_mute_ramp_as_direct_member_moves() {
    let mut checked = 0;
    for seed in 1..=VCA_RAMP_SEEDS {
        let mut draw = SendDraw(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xBEEF);
        let model = vca_ramp_model(&mut draw);
        let mut live = vca_ride_host(&model, 256);
        let effective = |model: &session::SessionModel, strip: usize| {
            (
                [0, 1].map(|lane| vca_reference_effective(model, VCA_STRIPS[strip], lane).0),
                [0, 1].map(|lane| vca_reference_mute(model, strip, lane)),
            )
        };
        let mut flat = model.clone();
        for (strip, id) in VCA_STRIPS.iter().enumerate() {
            let (db, mute) = effective(&model, strip);
            let fader = vca_strip_fader(&mut flat, id);
            (fader.left_db, fader.right_db) = (db[0], db[1]);
            (fader.left_mute, fader.right_mute) = (mute[0], mute[1]);
        }
        flat.vcas = Vec::new();
        let mut reference = vca_ride_host(&flat, 256);
        vca_lockstep(
            &mut [&mut live, &mut reference],
            Some(1),
            0,
            1,
            &format!("seed {seed}: boot"),
        );
        let mut block = 1_u64;
        let mut current = model;
        for step in 0..6 {
            let before: Vec<([f32; 2], [bool; 2])> = (0..VCA_STRIPS.len())
                .map(|strip| effective(&current, strip))
                .collect();
            let ride_ramp = [64_u32, 200, 300, 500][draw.below(4) as usize];
            let mute_ramp = [64_u32, 200, 300, 500][draw.below(4) as usize];
            let mut records = 0;
            let kinds = draw.below(3); // 0 a ride, 1 a mute, 2 both
            if kinds != 1 {
                let vca = draw.below(current.vcas.len() as u64) as usize;
                let channel = draw.below(3) as u8;
                let db = draw.uniform(-24.0, 12.0);
                stage_vca(
                    &mut live,
                    records,
                    COMMAND_VCA_FADER_DB,
                    vca as u32,
                    channel,
                    db,
                    ride_ramp,
                );
                records += 1;
                let fader = &mut current.vcas[vca].fader;
                if channel != 1 {
                    fader.left_db = db;
                }
                if channel != 0 {
                    fader.right_db = db;
                }
            }
            if kinds != 0 {
                let vca = draw.below(current.vcas.len() as u64) as usize;
                let channel = draw.below(3) as u8;
                let on = draw.below(2) == 0;
                let value = if on { 1.0 } else { 0.0 };
                stage_vca(
                    &mut live,
                    records,
                    COMMAND_VCA_MUTE,
                    vca as u32,
                    channel,
                    value,
                    mute_ramp,
                );
                records += 1;
                let fader = &mut current.vcas[vca].fader;
                if channel != 1 {
                    fader.left_mute = on;
                }
                if channel != 0 {
                    fader.right_mute = on;
                }
            }
            assert_eq!(
                live.submit_commands(records as u32),
                RESULT_OK,
                "seed {seed} step {step}"
            );
            // The same changes, told directly: per changed lane, a kind 3 at the ride's ramp,
            // then a kind 4 at the mute's ramp.
            let mut direct = 0;
            for (strip, (was_db, _)) in before.iter().enumerate() {
                let (db, _) = effective(&current, strip);
                let changed = [db[0] != was_db[0], db[1] != was_db[1]];
                if changed == [true, true] && db[0] == db[1] {
                    stage_fader_db(&mut reference, direct, strip as u32, 2, db[0], ride_ramp);
                    direct += 1;
                    continue;
                }
                for lane in 0..2 {
                    if changed[lane] {
                        let channel = lane as u8;
                        stage_fader_db(
                            &mut reference,
                            direct,
                            strip as u32,
                            channel,
                            db[lane],
                            ride_ramp,
                        );
                        direct += 1;
                    }
                }
            }
            for (strip, (_, was_mute)) in before.iter().enumerate() {
                let (_, mute) = effective(&current, strip);
                let changed = [mute[0] != was_mute[0], mute[1] != was_mute[1]];
                if changed == [true, true] && mute[0] == mute[1] {
                    stage_lane_mute(&mut reference, direct, strip as u32, 2, mute[0], mute_ramp);
                    direct += 1;
                    continue;
                }
                for lane in 0..2 {
                    if changed[lane] {
                        let channel = lane as u8;
                        stage_lane_mute(
                            &mut reference,
                            direct,
                            strip as u32,
                            channel,
                            mute[lane],
                            mute_ramp,
                        );
                        direct += 1;
                    }
                }
            }
            if direct > 0 {
                assert_eq!(
                    reference.submit_commands(direct as u32),
                    RESULT_OK,
                    "seed {seed} step {step}: direct"
                );
                checked += 1;
            }
            assert_eq!(
                follow_lanes(&live),
                follow_lanes(&reference),
                "seed {seed} step {step}: follow"
            );
            vca_lockstep(
                &mut [&mut live, &mut reference],
                Some(1),
                block,
                6,
                &format!("seed {seed} step {step} (ride @{ride_ramp}, mute @{mute_ramp})"),
            );
            block += 6;
        }
    }
    assert!(
        checked >= VCA_RAMP_SEEDS as usize * 2,
        "the seeds reach batches that move a member ({checked})"
    );
}

/// Issue #1245 (verdict MINOR-1, the verifier's probe
/// `sol_probe_split_member_move_and_last_ride_ramp`). `band` holds `drums` with offsets
/// `[-6, +3]` dB over its own `[0, 0]`: a both-lanes kind 3 on `drums` to -2 dB with a 300-sample
/// ramp splits into effective `[-8, +1]`, and renders, through its ramp, as a VCA-free host told
/// the two lane moves directly at the same ramp. Then two rides of `band` in one batch (ramps 64,
/// then 400) take the last ride's ramp, as D3 says.
///
/// Test value: red if a split member move's two records drop the command's window, or if the VCA
/// fader pass takes the first ride's ramp instead of the last's.
#[test]
fn a_split_member_move_ramps_and_the_last_ride_sets_the_window() {
    let live_model = vca_ride_model(
        &[("drums", [0.0; 2], [false; 2])],
        &[("band", [-6.0, 3.0], [false; 2], &["drums"])],
    );
    let flat = vca_ride_model(&[("drums", [-6.0, 3.0], [false; 2])], &[]);
    let mut live = vca_ride_host(&live_model, 16);
    let mut reference = vca_ride_host(&flat, 16);
    vca_lockstep(&mut [&mut live, &mut reference], Some(1), 0, 1, "boot");
    stage_fader_db(&mut live, 0, SOLO_FOLLOW_DRUMS, 2, -2.0, 300);
    assert_eq!(live.submit_commands(1), RESULT_OK);
    stage_fader_db(&mut reference, 0, SOLO_FOLLOW_DRUMS, 0, -8.0, 300);
    stage_fader_db(&mut reference, 1, SOLO_FOLLOW_DRUMS, 1, 1.0, 300);
    assert_eq!(reference.submit_commands(2), RESULT_OK);
    vca_lockstep(
        &mut [&mut live, &mut reference],
        Some(1),
        1,
        4,
        "split member move ramp",
    );
    stage_vca(&mut live, 0, COMMAND_VCA_FADER_DB, 0, 2, -3.0, 64);
    stage_vca(&mut live, 1, COMMAND_VCA_FADER_DB, 0, 2, 2.0, 400);
    assert_eq!(live.submit_commands(2), RESULT_OK);
    // Effective: own -2 dB + 2 dB = 0 dB on both lanes.
    stage_fader_db(&mut reference, 0, SOLO_FOLLOW_DRUMS, 2, 0.0, 400);
    assert_eq!(reference.submit_commands(1), RESULT_OK);
    vca_lockstep(
        &mut [&mut live, &mut reference],
        Some(1),
        5,
        5,
        "two rides: the last ramp",
    );
}

/// Issue #1245 D3 (verdict MINOR-2, the verifier's probe
/// `sol_probe_long_vca_mute_ramp_refuses_at_first_coalesced_index`): a kind 17 whose ramp exceeds
/// `INDEXED_RAMP_LENGTH_MAXIMUM` and moves a following send (`band` holds `drums`, which
/// `drums-verb` follows) refuses the whole submission `domain` at the coalesced record's wire
/// index: the batch's first kind 9 or 17 record. Behind a fader move that is the long kind 17
/// itself (1); behind a solo, the solo (0); behind an earlier short kind 17, the earlier one (2).
/// Nothing moves.
///
/// Test value: red if a kind 17 does not set the coalesced record's wire index (the refusal then
/// names index 0 whatever the batch), or if an overlong VCA mute ramp is admitted.
#[test]
fn a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index() {
    let model = vca_ride_model(&[], &[("band", [0.0; 2], [false; 2], &["drums"])]);
    let mut live = vca_ride_host(&model, 16);
    follow_render(&mut live, &SOLO_FOLLOW_TRACKS, 0, None);
    let too_long = lane::kernels::INDEXED_RAMP_LENGTH_MAXIMUM + 1;
    stage_fader_db(&mut live, 0, VCA_BASS, 2, -1.0, 0);
    stage_vca(&mut live, 1, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    stage_fader_db(&mut live, 2, VCA_BASS, 2, -2.0, 0);
    assert_vca_refusal(
        &mut live,
        3,
        RESULT_INVALID_ARGUMENT,
        COMMAND_REASON_DOMAIN,
        1,
        "long VCA mute ramp",
    );
    stage_solo(&mut live, 0, SOLO_FOLLOW_VOCAL, false, 0);
    stage_fader_db(&mut live, 1, VCA_BASS, 2, -1.0, 0);
    stage_vca(&mut live, 2, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    assert_vca_refusal(
        &mut live,
        3,
        RESULT_INVALID_ARGUMENT,
        COMMAND_REASON_DOMAIN,
        0,
        "solo, then a long VCA mute ramp",
    );
    stage_fader_db(&mut live, 0, VCA_BASS, 2, -1.0, 0);
    stage_fader_db(&mut live, 1, VCA_BASS, 2, -2.0, 0);
    stage_vca(&mut live, 2, COMMAND_VCA_MUTE, 0, 2, 1.0, 64);
    stage_vca(&mut live, 3, COMMAND_VCA_MUTE, 0, 2, 1.0, too_long);
    assert_vca_refusal(
        &mut live,
        4,
        RESULT_INVALID_ARGUMENT,
        COMMAND_REASON_DOMAIN,
        2,
        "the first kind 17's index",
    );
}

// ---- Issue #1246: VCA groups enumerated for the SDK ----

/// Issue #1246 gate 3 and D1: the VCA exports enumerate the session's VCAs in the order a VCA
/// kind's index word reads, through ID staging sized for the longest VCA ID.
///
/// Three VCAs, listed in the model out of canonical order: a 73-byte `zz-` VCA, longer than every
/// source, track, submix and route ID, that lists `vocal` and the nested `aa`; `mm` over `drums`;
/// and `aa` over `bass`. The host boots the canonical document, which sorts them (`aa`, `mm`,
/// `zz-`), so the order this test checks is the canonical one; independence from a document's
/// declaration order is the SDK eval's (`vcaDocument()` boots a non-canonical document). The three
/// reach distinct strip sets, so a kind 16 at exported index `i` must move exactly the strips of
/// the VCA the export names at `i`.
///
/// Test value: red if the staging capacity ignores VCA IDs (the long ID's copy overruns the
/// buffer and traps), if the export enumerates any order other than the live VCA state's (the
/// index admission bounds and composes by), or if it answers without live controls.
#[test]
fn live_vca_ids_enumerate_in_vca_index_order_through_staging_sized_for_them() {
    let long = format!("zz-{}", "x".repeat(70));
    let rows: [VcaRow<'_>; 3] = [
        (long.as_str(), [0.0; 2], [false; 2], &["vocal", "aa"]),
        ("mm", [0.0; 2], [false; 2], &["drums"]),
        ("aa", [0.0; 2], [false; 2], &["bass"]),
    ];
    let model = vca_ride_model(&[], &rows);
    let document = canonical_session_json(&model).expect("long VCA session canonicalizes");
    let canonical = ["aa", "mm", long.as_str()];

    let handle = crate::ffi::test_boot(document.as_bytes(), strip_options(16, 0, 0));
    assert_ne!(handle, 0, "the long-VCA session boots");
    let resources = crate::ffi::test_resources(handle).expect("resource report");
    assert_eq!(resources.id_staging_bytes, long.len() as u64);
    assert_eq!(miso_engine_web_v1_live_control_vca_count(handle), 3);
    for (index, expected) in canonical.iter().enumerate() {
        let length = miso_engine_web_v1_live_control_vca_id(handle, index as u32);
        assert_eq!(length, expected.len() as u32, "VCA {index}");
        assert_eq!(
            crate::ffi::test_read_source_id(handle, length).expect("staged VCA ID"),
            expected.as_bytes()
        );
    }
    assert_eq!(miso_engine_web_v1_live_control_vca_id(handle, 3), 0);
    assert_eq!(miso_engine_web_v1_live_control_vca_id(handle, u32::MAX), 0);
    assert_eq!(
        miso_engine_web_v1_live_control_vca_count(handle.wrapping_add(1)),
        0
    );
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_live_control_vca_count(handle), 0);

    // Exported VCA `i` is the VCA a kind 16 at index `i` rides: its members' effective faders, and
    // nobody else's, move. Strip order: `bass` 0, `drums` 1, `vocal` 2, `room` 3, `verb` 4.
    let reached: [&[usize]; 3] = [&[0], &[1], &[0, 2]];
    for (index, strips) in reached.iter().enumerate() {
        let mut host = vca_ride_host(&model, 16);
        assert_eq!(host.live_control_vca_count(), 3);
        let length = host.copy_live_control_vca_id(index as u32) as usize;
        let staged = &host.buffers.as_ref().expect("buffers").source_id[..length];
        assert_eq!(staged, canonical[index].as_bytes(), "VCA {index}");
        let before = vca_mirror(&host);
        stage_vca(&mut host, 0, COMMAND_VCA_FADER_DB, index as u32, 2, -6.0, 0);
        assert_eq!(host.submit_commands(1), RESULT_OK, "ride VCA {index}");
        let after = vca_mirror(&host);
        let moved: Vec<usize> = (0..VCA_STRIPS.len())
            .filter(|strip| before[*strip] != after[*strip])
            .collect();
        assert_eq!(moved, *strips, "VCA {index} ({}) moved", canonical[index]);
    }

    // Without live controls the VCA kinds are refused, so no VCA is enumerated.
    let handle = crate::ffi::test_boot(
        document.as_bytes(),
        WebBootOptions {
            source_ring_frames: STRIP_QUANTUM * 4,
            ..boot_options(STRIP_QUANTUM)
        },
    );
    assert_ne!(
        handle, 0,
        "the long-VCA session boots without live controls"
    );
    assert_eq!(miso_engine_web_v1_live_control_vca_count(handle), 0);
    assert_eq!(miso_engine_web_v1_live_control_vca_id(handle, 0), 0);
    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
}
