use core::mem::{offset_of, size_of, size_of_val};

use host_core::{
    SpectrumSmoothingConfig, prepare_host_runtime_with_selected_meters_between_render_calls,
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
    const HEADER: &str = r#""automation": [{"id":"dense-invalid","target":{"owner":"strip","owner_id":"track","section":"builtins","control":5,"lane":"both"},"segments":["#;
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
    // Issue #137 D1: the two live-control words were the first two of the frozen configuration's
    // four reserved words; since #1565 the first is `reserved1` and must be zero. Every V1 writer
    // already sets it to zero, so the layout and every existing caller stand.
    assert_eq!(size_of::<WebCommandReport>(), 48);
    assert_eq!(COMMAND_REPORT_BYTES, 48);
    assert_eq!(COMMAND_RECORD_BYTES, 48);
    assert_eq!(MAXIMUM_COMMAND_RECORDS, 256);
    assert_eq!(DEFAULT_METER_BLOCKS, 12);
    // Kinds 1-6 and 9-17 are deleted (#1565A) and their numbers are never reallocated.
    assert_eq!(
        [COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE],
        [7, 8]
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
    assert_eq!(offset_of!(WebBootOptions, struct_size), 0);
    assert_eq!(offset_of!(WebBootOptions, require_quantum_frames), 12);
    assert_eq!(offset_of!(WebBootOptions, source_ring_frames), 16);
    assert_eq!(offset_of!(WebBootOptions, maximum_memory_bytes), 24);
    assert_eq!(offset_of!(WebBootOptions, reserved1), 32);
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
    assert_eq!(offset_of!(WebStatus, live_values_superseded), 48);
    assert_eq!(offset_of!(WebStatus, watermark_revision), 56);
    assert_eq!(offset_of!(WebStatus, watermark_first_sample), 64);
    assert_eq!(offset_of!(WebStatus, watermark_outcome_flags), 72);
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

/// The observation command staging and its decode array are charged exactly, and exist exactly
/// when the boot asked for observation taps (#1565A): with taps off there is no wire staging and no
/// decode array; with taps on, `MAXIMUM_COMMAND_RECORDS` records of wire staging and as many
/// decoded entries, each in the bridge rows.
///
/// Test value: red if the browser keeps the retired mirrors' bytes (input-filter shadows, ramp
/// table, VCA and strip-value mirrors) in the bridge rows, or if the decode array outlives the
/// staging it decodes.
#[test]
fn observation_staging_resource_is_exact_for_the_taps_off_and_on_modes() {
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
    for (name, options, expected_records) in [
        ("off", boot_options(128), 0_u32),
        (
            "on",
            WebBootOptions {
                live_control_meter_blocks: 0,
                live_control_observation_taps: 1,
                ..boot_options(128)
            },
            MAXIMUM_COMMAND_RECORDS,
        ),
    ] {
        let expected_wire_bytes = u64::from(expected_records * COMMAND_RECORD_BYTES);
        let decoded_bytes = u64::from(expected_records) * size_of::<StagedObserve>() as u64;
        let projection = project_buffers(
            document.len() as u32,
            shape.sample_rate_hz,
            shape.quantum_frames,
            shape.maximum_source_channels,
            shape
                .longest_source_id_bytes
                .max(shape.longest_track_id_bytes)
                .max(shape.longest_submix_id_bytes),
            options,
            (false, (0, 0)),
        )
        .expect("bridge projection");
        assert_eq!(projection.command_records, expected_records, "{name}");

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
            HostLiveLanes {
                effects: options.live_control_observation_taps != 0,
                ..controls::live_lanes(compiled.normalized_model()).into()
            },
            &host_core::LiveValuesSuperseded::default(),
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
        // The control catalog the preparation builds and `ReadyOwnership.host` keeps (#1400
        // amendment).
        let catalog_bytes = engine.control_catalog.resources().retained_bytes;
        assert!(catalog_bytes > 0, "{name}: the catalog holds its rows");

        let host =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).unwrap_or_else(|failure| {
                panic!("{name}: {}", String::from_utf8_lossy(failure.diagnostic()))
            });
        let ready = host.ready.as_ref().expect("ready ownership");
        assert_eq!(ready.command_decoded.len(), expected_records as usize);
        assert_eq!(
            size_of_val(ready.command_decoded.as_ref()) as u64,
            decoded_bytes,
            "{name}: typed decoded backing has exact element size"
        );
        assert_eq!(host.command_staging_bytes(), expected_wire_bytes);
        assert!(
            ready.shape.effect_instances.is_empty(),
            "no effect additions"
        );
        assert!(ready.shape.meter_pending.is_empty(), "no meter additions");
        assert!(
            ready
                .shape
                .rack_effects
                .iter()
                .all(|counts| *counts == [0, 0, 0])
        );
        assert_eq!(host.resources().observation_retained_bytes, 0);

        // The control plane's own retained row (its catalog, producers, protocol queues, replay
        // cache, the plan's inventory and the superseded count) is charged once, and holds the
        // rows an independent preparation of the same session reports.
        let control_retained = host
            .ready
            .as_ref()
            .expect("ready ownership")
            .queries
            .resources()
            .control_retained_bytes;
        assert!(
            control_retained
                >= source_control_bytes
                    + inventory_bytes
                    + catalog_bytes
                    + host_core::LiveValuesSuperseded::retained_bytes(),
            "{name}: the control plane's row holds the rows an independent preparation reports"
        );
        let expected_bridge_metadata = projection
            .report
            .bridge_metadata_bytes
            .checked_add(control_retained)
            .and_then(|bytes| bytes.checked_add(session_model_bytes))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
            .expect("bridge metadata arithmetic");
        let expected_bridge_retained = projection
            .report
            .bridge_retained_bytes
            .checked_add(control_retained)
            .and_then(|bytes| bytes.checked_add(session_model_bytes))
            .and_then(|bytes| bytes.checked_add(decoded_bytes))
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
            .max(decoded_bytes);
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
            live_control_meter_blocks: 0,
            live_control_observation_taps: 1,
            ..boot_options(128)
        },
    ] {
        let baseline =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("baseline boot");
        let ready = baseline.ready.as_ref().expect("ready ownership");
        let dense_effect_table_bytes = size_of_val(ready.shape.effect_instances.as_ref()) as u64;
        assert_eq!(
            ready.shape.effect_instances.len(),
            1,
            "one-track fixture has one effect"
        );
        assert!(
            baseline.resources().bridge_retained_bytes >= dense_effect_table_bytes,
            "the bridge rows charge the dense effect table in both modes"
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

/// The `SET_CONTROL` timeline's four command frames (decision 17 D6, #1565A): a fader move, a pan
/// move, an effect parameter and one refused value, each a `SESSION_TRANSACTION_APPLY` frame at
/// the revision the session reaches before it. They are committed as `tests/browser-v1/
/// set-control-frames/frame-N.bin` so the raw-Wasm oracle (`direct-oracle.mjs`) sends the very
/// bytes this test sends; `set_control_frames_are_the_committed_fixtures` pins the files to this
/// encoder.
fn set_control_timeline_frames() -> Vec<Vec<u8>> {
    use controls::{ControlAddress, Owner, Section};
    use protocol::{
        CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
        TypedCommandFrame,
    };

    const QUANTUM: u32 = 128;
    let row = |section: Section, control: u32, lane: session::ParameterChannel| ControlAddress {
        owner: Owner::Strip(session::StableId::parse("track").expect("strip ID")),
        section,
        control,
        lane,
    };
    let eq = || Section::Inserts(session::StableId::parse("eq").expect("effect ID"));
    let set =
        |address: ControlAddress, value: f32, ramp_samples: Option<u32>| SessionEdit::SetControl {
            address,
            value,
            ramp_samples,
        };
    let transactions: [Vec<SessionEdit>; 4] = [
        // builtins `fader_db` (5), both lanes, over one quantum.
        vec![set(
            row(Section::Builtins, 5, session::ParameterChannel::Both),
            -6.0,
            Some(QUANTUM),
        )],
        // builtins `pan` (12): the left and right lanes, over one quantum.
        vec![
            set(
                row(Section::Builtins, 12, session::ParameterChannel::Left),
                -0.25,
                Some(QUANTUM),
            ),
            set(
                row(Section::Builtins, 12, session::ParameterChannel::Right),
                0.75,
                Some(QUANTUM),
            ),
        ],
        // the EQ's band 1 gain (4), both lanes, over the session's own length.
        vec![set(
            row(eq(), 4, session::ParameterChannel::Both),
            -12.0,
            None,
        )],
        // a value outside the row's domain: refused by the core, which changes nothing.
        vec![set(
            row(Section::Builtins, 5, session::ParameterChannel::Both),
            100.0,
            None,
        )],
    ];
    transactions
        .iter()
        .enumerate()
        .map(|(index, edits)| {
            let mut bytes = vec![0_u8; 4_096];
            let length = ProtocolCodec::default()
                .encode_command_frame_into(
                    &TypedCommandFrame {
                        request_id: RequestId::new(index as u64 + 1).expect("nonzero request"),
                        // The session boots at revision 1 and each of the first three frames
                        // commits one.
                        expected_revision: ExpectedRevision::Exact(SessionRevision(
                            1 + index.min(3) as u64,
                        )),
                        payload: CommandPayload::SessionTransactionApply(edits),
                    },
                    &mut bytes,
                )
                .expect("a transaction frame");
            bytes.truncate(length);
            bytes
        })
        .collect()
}

/// The committed frame fixtures are exactly what [`set_control_timeline_frames`] encodes.
///
/// Test value: red if the encoder or the protocol changes a byte of a frame without the fixtures
/// the raw-Wasm oracle sends moving with it, so the native leg and the shipped artifact would
/// digest different timelines. Set `MISO_ENGINE_WRITE_SET_CONTROL_FRAMES=1` to rewrite the files.
#[test]
fn set_control_frames_are_the_committed_fixtures() {
    let directory = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/browser-v1/set-control-frames"
    );
    let frames = set_control_timeline_frames();
    if std::env::var_os("MISO_ENGINE_WRITE_SET_CONTROL_FRAMES").is_some() {
        std::fs::create_dir_all(directory).expect("fixture directory");
        for (index, frame) in frames.iter().enumerate() {
            std::fs::write(format!("{directory}/frame-{}.bin", index + 1), frame)
                .expect("fixture file");
        }
    }
    for (index, frame) in frames.iter().enumerate() {
        let committed = std::fs::read(format!("{directory}/frame-{}.bin", index + 1))
            .expect("the committed frame fixture");
        assert_eq!(&committed, frame, "frame {}", index + 1);
    }
}

/// The `SET_CONTROL` timeline's native digest: the same session, source feed and control frames
/// the raw-Wasm oracle (`tests/browser-v1/direct-oracle.mjs`) drives through the shipped artifact,
/// run natively through `control_command`, the entry the exports run (decision 17 D6, #1565A). It
/// replaced #137 E2's command-record timeline when the browser's own value kinds were deleted.
///
/// The fixture is identity apart from one parametric EQ insert whose band 1 is a low shelf, so a
/// constant input renders to that same constant until a control moves it. Ten blocks of a constant
/// 0.25 source; before block 1 a fader move to -6 dB over one quantum, before block 2 a pan move
/// over one quantum, before block 3 the EQ band gain to -12 dB, before block 4 a refused
/// out-of-domain fader value that changes no rendered sample. The digest is therefore a statement
/// about *when* each control took effect through the one core path, not merely that it did. It is
/// over little-endian `f32` words, so it is a bit comparison.
///
/// Test value: red if the browser's control path renders a transaction at another block than the
/// shipped artifact does, or if a refused transaction moves a cell.
#[test]
fn native_set_control_timeline_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    const BLOCKS: u64 = 10;

    // The fixture file is read verbatim, exactly as `direct-oracle.mjs` reads it, so both legs
    // compile byte-identical input.
    let document = include_str!("../tests/browser-v1/command-session.json");
    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
    assert_eq!(host.live_control_tracks().len(), 1);
    assert_eq!(host.committed_revision(), 1);

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
    let send = |host: &mut AudioWorkletEngineHost, frame: &[u8]| {
        let length = u32::try_from(frame.len()).expect("frame length");
        host.control_request_mut(length)
            .expect("the control request staging")[..frame.len()]
            .copy_from_slice(frame);
        assert_eq!(host.control_command(length), RESULT_OK);
        // A commit queues its reliable events; drain them the way the worklet does.
        for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
            loop {
                assert_eq!(host.control_event(lane), RESULT_OK);
                if host.control_response().is_empty() {
                    break;
                }
            }
        }
    };

    let frames = set_control_timeline_frames();
    step(&mut host, 0);
    for (index, frame) in frames.iter().enumerate() {
        send(&mut host, frame);
        // The first three frames commit one revision each; the refused fourth commits none.
        assert_eq!(
            host.committed_revision(),
            1 + (index as u64 + 1).min(3),
            "frame {}",
            index + 1
        );
        step(&mut host, index as u64 + 1);
    }
    for block in 5..BLOCKS {
        step(&mut host, block);
    }
    assert_eq!(host.status().rendered_quanta, BLOCKS);
    // The timeline is not vacuous: each of the three accepted frames moved what the next block
    // renders.
    for block in 1..=3 {
        assert_ne!(blocks[block], blocks[block - 1], "block {block}");
    }

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
        native, expected.native_set_control_timeline,
        "native and the pinned wasm SET_CONTROL-timeline digest must agree bit for bit"
    );
    assert_eq!(
        native, expected.simd128_set_control_timeline,
        "the shipped simd128 artifact renders this SET_CONTROL timeline to the same bits as native"
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
        pub(super) native_set_control_timeline: String,
        pub(super) simd128_set_control_timeline: String,
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
        // transcript first, then the `SET_CONTROL` timeline.
        let timeline = text
            .find("\"setControlTimeline\"")
            .expect("expected.json has no setControlTimeline");
        Pin {
            native: hex_after(text, "\"nativePcmF32leSha256\":"),
            simd128: hex_after(text, "\"pcmF32leSha256\":"),
            native_set_control_timeline: hex_after(
                text,
                "\"nativeSetControlTimelinePcmF32leSha256\":",
            ),
            simd128_set_control_timeline: hex_after(&text[timeline..], "\"pcmF32leSha256\":"),
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
        live_control_meter_blocks: meter_blocks,
        ..WebBootOptions::explicit_defaults()
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("live-control boot")
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

/// The observation fixture's compressor: the one launch effect with a resident tap (gain
/// reduction, tap 1) that a session in this file can bind, so an insert queue can be filled with
/// observation subscriptions, which stay a FIFO (D15-2 condition 1; #1345 D2), where a fader,
/// matrix or input value is a latest-target cell with no room to fill (#1312, #1346).
fn observed_compressor_insert() -> session::Effect {
    parse_session_json(include_str!(
        "../../../fixtures/session/v1/observation-frame-shape.json"
    ))
    .expect("accepted observation fixture")
    .tracks[0]
        .inserts
        .effects[0]
        .clone()
}

/// Stage one observation subscription (tap 1, a two-block window) for insert `effect` of `track`.
fn stage_observe_subscribe_at(
    host: &mut AudioWorkletEngineHost,
    index: usize,
    track: u32,
    effect: u32,
) {
    stage_command(
        host,
        index,
        COMMAND_OBSERVE_SUBSCRIBE,
        RACK_INSERTS,
        255,
        track,
        effect,
        1,
        2,
        [0.0; 4],
    );
}

/// Stage one subscription for the first insert of track 0, the record the queue-credit tests fill
/// that insert's queue with. A repeated subscription is idempotent in effect and costs one record.
fn stage_observe_subscribe(host: &mut AudioWorkletEngineHost, index: usize) {
    stage_observe_subscribe_at(host, index, 0, 0);
}

/// The command fixture with its parametric EQ insert swapped for the observed compressor and
/// observation lanes bound: track 0 keeps one insert queue that observation records can fill.
fn observed_effect_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    let mut model = parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
        .expect("accepted command fixture");
    model.tracks[0].inserts.effects[0] = observed_compressor_insert();
    let document = canonical_session_json(&model).expect("canonical observed command fixture");
    let options = WebBootOptions {
        source_ring_frames: quantum,
        live_control_observation_taps: 4,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options)
        .expect("observed effect live-control boot")
}

/// #514 IO7: command-free blocks skip the dense admission-counter reset while command ownership
/// and the existing transactional capacity contract remain exact.
#[test]
fn idle_render_skips_admission_counter_clear_without_losing_queue_credit() {
    const QUANTUM: u32 = 128;

    let mut host = observed_effect_live_control_host(QUANTUM);
    // The observed compressor declares one tap, so its observation queue holds one subscription
    // between two renders (#1345 D2: one slot per declared tap).
    let ready = host.ready.as_ref().expect("ready");
    let queue_depth = ready
        .effect_producer_position(0)
        .and_then(|position| ready.control.effect_producers().get(position))
        .map(|producer| producer.observe_capacity())
        .expect("the compressor's producer");
    assert_eq!(queue_depth, 1);
    let depth = queue_depth as u32;
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
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // Repeated observation subscriptions on the insert queue (the one queue kind that stays a FIFO,
    // #1345 D2) preserve the exact per-queue count until the successful render drains it.
    stage_observe_subscribe(&mut host, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
    let application_sample = 3 * u64::from(QUANTUM);
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
        host.ready.as_ref().expect("ready").shape.in_flight[..],
        [depth]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);

    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 3, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 1));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    feed_and_render(&mut host, 1, 4, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 1));

    // A healthy drained host can refill a queue to its exact old capacity; one more record is the
    // existing typed refusal and leaves the accepted prefix's ownership untouched.
    for index in 0..depth as usize {
        stage_observe_subscribe(&mut host, index);
    }
    assert_eq!(host.submit_commands(depth), RESULT_OK);
    assert_eq!(
        host.ready.as_ref().expect("ready").shape.in_flight[0],
        depth
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);
    stage_observe_subscribe(&mut host, 0);
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
        host.ready.as_ref().expect("ready").shape.in_flight[..],
        [depth]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);

    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 5, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 1));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // The successful drain renews the full old capacity. Drain that refill, then prove the next
    // command-free block performs no physical clear.
    for index in 0..depth as usize {
        stage_observe_subscribe(&mut host, index);
    }
    assert_eq!(host.submit_commands(depth), RESULT_OK);
    assert_eq!(host.command_report().admitted, depth);
    assert_eq!(
        host.command_report().applied_at_sample,
        6 * u64::from(QUANTUM)
    );
    assert_eq!(
        host.ready.as_ref().expect("ready").shape.in_flight[..],
        [depth]
    );
    assert!(host.ready.as_ref().expect("ready").has_in_flight_commands);
    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 6, 0.25);
    assert_eq!(admission_counter_clear_stats(), (1, 1));
    assert!(!host.ready.as_ref().expect("ready").has_in_flight_commands);
    assert!(
        host.ready
            .as_ref()
            .expect("ready")
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    reset_admission_counter_clear();
    feed_and_render(&mut host, 1, 7, 0.25);
    assert_eq!(admission_counter_clear_stats(), (0, 0));

    // Malformed submissions on an idle host cannot create pending ownership, a submission that
    // names one observation tap twice included (a single submission cannot overfill the queue, so
    // that is the only way for one to be refused here).
    let mut refused = observed_effect_live_control_host(QUANTUM);
    stage_command(
        &mut refused,
        0,
        3,
        255,
        2,
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
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );
    for index in 0..=depth as usize {
        stage_observe_subscribe(&mut refused, index);
    }
    assert_eq!(refused.submit_commands(depth + 1), RESULT_INVALID_ARGUMENT);
    assert_eq!(refused.command_report().reason, COMMAND_REASON_MALFORMED);
    assert_eq!(refused.command_report().rejected_index, depth);
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
            .shape
            .in_flight
            .iter()
            .all(|count| *count == 0)
    );

    // A refusal after accepted records preserves those records and the dirty flag.
    stage_observe_subscribe(&mut refused, 0);
    assert_eq!(refused.submit_commands(1), RESULT_OK);
    stage_command(
        &mut refused,
        0,
        3,
        255,
        2,
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
    assert_eq!(refused.ready.as_ref().expect("ready").shape.in_flight[0], 1);

    // A render failure is sticky: pending ownership and the untouched clear counters survive the
    // failed block and the subsequent WRONG_STATE re-entry.
    let mut failed = observed_effect_live_control_host(QUANTUM);
    stage_observe_subscribe(&mut failed, 0);
    assert_eq!(failed.submit_commands(1), RESULT_OK);
    let before = failed
        .ready
        .as_ref()
        .expect("ready")
        .shape
        .in_flight
        .to_vec();
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
        &failed.ready.as_ref().expect("ready").shape.in_flight[..],
        &before[..]
    );
    assert!(failed.ready.as_ref().expect("ready").has_in_flight_commands);
    assert_eq!(failed.render_next(), RESULT_WRONG_STATE);
    assert_eq!(failed.status().state, STATE_FAILED);
    assert_eq!(failed.diagnostic(), b"web.render.rejected\t$\n");
    assert_eq!(admission_counter_clear_stats(), (0, 0));
    assert_eq!(
        &failed.ready.as_ref().expect("ready").shape.in_flight[..],
        &before[..]
    );
    assert!(failed.ready.as_ref().expect("ready").has_in_flight_commands);
}

/// A live-control host over the *command* fixture: the identity session plus one dynamic-rack
/// parametric EQ, so an effect-addressed command has something real to address (issue #140 A).
fn effect_live_control_host(quantum: u32) -> AudioWorkletEngineHost {
    let document = include_str!("../tests/browser-v1/command-session.json");
    let options = WebBootOptions {
        source_ring_frames: quantum,
        ..boot_options(quantum)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("effect live-control boot")
}

/// #140 C: `UNSUPPORTED_KIND` still means exactly what it says -- the target is real and the value
/// is legal, and *this session* has no write path. A host compiled with no live controls is that
/// session, and every live kind is refused with it.
#[test]
fn a_live_control_free_host_refuses_every_live_kind_as_unsupported() {
    const QUANTUM: u32 = 128;
    let mut host = effect_live_control_host(QUANTUM);
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

/// #1565A gate 1: a record of a deleted value kind (1-6 and 9-17) is refused `MALFORMED` at its
/// index, before anything is admitted, and changes no cell. The browser keeps no value admission of
/// its own: every value goes through `control_command`, so these numbers are only retired.
///
/// The records are otherwise shaped as the kind they had (the shapes a live kind accepted before
/// the deletion), alone and behind a valid observation record that must not be armed.
///
/// Test value: red if a value kind still reaches the cells without the core, or if a refused
/// submission arms the observation record in front of it.
#[test]
fn every_deleted_kind_is_refused_malformed_at_its_index_and_changes_no_cell() {
    const QUANTUM: u32 = 128;
    let mut host = observed_effect_live_control_host(QUANTUM);
    // A twin that sees no traffic at all: the compressor's envelope moves between blocks, so the
    // second block is compared with the twin's second block, not with the first.
    let mut twin = observed_effect_live_control_host(QUANTUM);
    feed_and_render(&mut host, 1, 0, 0.75);
    feed_and_render(&mut twin, 1, 0, 0.75);
    let baseline_status = *host.status();
    let watermark_before = host.read_watermark();
    let revision_before = host.committed_revision();

    for kind in (1..=6_u32).chain(9..=17) {
        // Alone: refused at index 0.
        stage_command(
            &mut host,
            0,
            kind,
            255,
            2,
            0,
            0,
            0,
            QUANTUM,
            [-6.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            host.submit_commands(1),
            RESULT_INVALID_ARGUMENT,
            "kind {kind}"
        );
        let report = *host.command_report();
        assert_eq!(report.reason, COMMAND_REASON_MALFORMED, "kind {kind}");
        assert_eq!(report.rejected_index, 0, "kind {kind}");
        assert_eq!(report.admitted, 0, "kind {kind}");
        // Behind a valid subscription: refused at index 1, and the subscription is not armed.
        stage_observe_subscribe(&mut host, 0);
        stage_command(
            &mut host,
            1,
            kind,
            255,
            2,
            0,
            0,
            0,
            QUANTUM,
            [-6.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            host.submit_commands(2),
            RESULT_INVALID_ARGUMENT,
            "kind {kind}"
        );
        let report = *host.command_report();
        assert_eq!(report.reason, COMMAND_REASON_MALFORMED, "kind {kind}");
        assert_eq!(report.rejected_index, 1, "kind {kind}");
        assert_eq!(report.admitted, 0, "kind {kind}");
        assert_eq!(host.observation_armed_taps(), 0, "kind {kind}");
        assert!(
            !host.ready.as_ref().expect("ready").has_in_flight_commands,
            "kind {kind}: nothing was queued"
        );
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
    assert_eq!(host.committed_revision(), revision_before);
    assert_eq!(host.read_watermark(), watermark_before);
    feed_and_render(&mut host, 1, 1, 0.75);
    feed_and_render(&mut twin, 1, 1, 0.75);
    assert_eq!(
        host.output_pcm().expect("output"),
        twin.output_pcm().expect("twin output"),
        "a refused command changed no coefficient",
    );
}

/// #1565A gate 2: the boot word at offset 32 is reserved. A nonzero value is refused
/// `web.options.reserved1` (the word was the command-queue size until the browser's own value
/// commands were deleted); a boot with observation taps accepts the observation kinds; a boot
/// without taps has no command staging and refuses them `UNSUPPORTED` with
/// `COMMAND_REASON_UNSUPPORTED_KIND`.
///
/// Test value: red if the deleted word still changes behaviour, if observation loses its staging,
/// or if a host with no taps accepts a record.
#[test]
fn the_boot_word_is_reserved_and_observation_staging_follows_the_taps() {
    const QUANTUM: u32 = 128;
    let document = include_str!("../tests/browser-v1/command-session.json");
    let refused = AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            reserved1: 64,
            ..boot_options(QUANTUM)
        },
    )
    .err()
    .expect("a nonzero reserved1 must not boot");
    assert_eq!(refused.result(), RESULT_REFUSED_OPTIONS);
    assert_eq!(refused.diagnostic(), b"web.options.reserved1\t$\n");

    let mut observed = observed_effect_live_control_host(QUANTUM);
    assert!(observed.live_controls_attached());
    assert_eq!(
        observed.command_staging_bytes(),
        u64::from(MAXIMUM_COMMAND_RECORDS * COMMAND_RECORD_BYTES)
    );
    stage_observe_subscribe(&mut observed, 0);
    assert_eq!(observed.submit_commands(1), RESULT_OK);
    assert_eq!(observed.command_report().admitted, 1);

    let mut bare = AudioWorkletEngineHost::boot(document.as_bytes(), boot_options(QUANTUM))
        .expect("a boot without taps");
    assert!(!bare.live_controls_attached());
    assert_eq!(bare.command_staging_bytes(), 0);
    for count in [1_u32, 8, MAXIMUM_COMMAND_RECORDS] {
        assert_eq!(bare.submit_commands(count), RESULT_UNSUPPORTED);
        assert_eq!(
            bare.command_report().reason,
            COMMAND_REASON_UNSUPPORTED_KIND
        );
        assert_eq!(bare.command_report().admitted, 0);
    }
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
    let (Some(ready), Some(plan)) = (host.ready.as_mut(), host.plan.as_mut()) else {
        panic!("a ready host");
    };
    let companions = plan
        .active_attachment_mut::<RenderCompanions>()
        .expect("companions");
    for (meter, pending) in companions
        .meters
        .iter_mut()
        .zip(ready.shape.meter_pending.iter_mut())
    {
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
    same_track_observation_host_metered(quantum, 0)
}

/// [`same_track_observation_host`] with `meter_blocks` meter blocks, so it publishes a meter frame.
fn same_track_observation_host_metered(quantum: u32, meter_blocks: u64) -> AudioWorkletEngineHost {
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
    model.sources[0].frames = u64::from(quantum) * 64;
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.tracks[0].inserts.effects.push(second_effect);
    let document = canonical_session_json(&model).expect("canonical two-effect session");
    let options = WebBootOptions {
        source_ring_frames: quantum * 4,
        live_control_meter_blocks: meter_blocks,
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
    assert_eq!(ready.shape.tracks, [Box::from("t0"), Box::from("t1")]);
    assert_eq!(
        ready.shape.submixes,
        [Box::from("aaa-bus"), Box::from("zzz-bus")]
    );
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
            let slot = dense_effect_slot(
                ready.shape.effect_base[strip],
                ready.shape.rack_effects[strip],
                address,
            )
            .unwrap_or_else(|| panic!("{id} {address:?} has a dense slot"));
            let producer = ready.shape.effect_instances[slot]
                .as_ref()
                .unwrap_or_else(|| panic!("{id} {address:?} has a filed producer"));
            assert_eq!(&*producer.track_id, *id, "slot {slot}'s producer owner");
            assert_eq!(producer.address, address, "slot {slot}'s producer address");
            assert_eq!(
                producer.descriptor.id.as_str(),
                native,
                "slot {slot}'s effect"
            );
            let observed = producer.reader;
            if native == "miso.parametric-eq" {
                assert!(
                    observed.is_none(),
                    "{id} {address:?}: an EQ declares no tap"
                );
                assert_eq!(ready.shape.observation_tracks[slot], u32::MAX);
            } else {
                assert!(observed.is_some(), "{id} {address:?} has a filed observer");
                assert_eq!(ready.shape.observation_tracks[slot] as usize, strip);
            }
            filed += 1;
        }
    }
    assert_eq!(filed, 13);
    assert_eq!(
        ready.shape.effect_instances.len(),
        13,
        "no producer slot beyond the strips'"
    );
    assert_eq!(ready.shape.observation_present.len(), strips.len());
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
    let backlog = |host: &mut AudioWorkletEngineHost| -> usize {
        let (Some(ready), Some(plan)) = (host.ready.as_ref(), host.plan.as_mut()) else {
            panic!("a ready host");
        };
        let companions = plan
            .active_attachment_mut::<RenderCompanions>()
            .expect("companions");
        companions
            .meters
            .iter()
            .zip(ready.shape.meter_pending.iter())
            .map(|(meter, pending)| {
                meter.consumer.available_at_entry() + usize::from(pending.is_some())
            })
            .min()
            .expect("meters")
    };
    assert_eq!(host.test_companions().meters.len(), 5);
    for block in 0..40_u64 {
        feed_and_render_channels(&mut host, block, 0.5, 0.25);
        let dropped: &[usize] = match block {
            10 => &[2],
            20 => &[0],
            30 => &[1, 3],
            _ => &[],
        };
        let companions = host.test_companions();
        for &meter in dropped {
            assert!(
                companions.meters[meter].consumer.try_pop().is_ok(),
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
            assert_eq!(
                backlog(&mut host),
                0,
                "block {block}: no window left queued"
            );
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
        host.test_observation_handle(effect)
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
    // look ready while a replacement window is being built. (A subscription to an armed tap is
    // the re-arm: it restarts the window, and the effect's one-slot queue holds just it.)
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
        host.test_observation_handle(effect)
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
        let reader = &host
            .test_observation_handle(effect)
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
        let reader = &host
            .test_observation_handle(effect)
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
            host.test_observation_handle(compressor_effect)
                .expect("compressor observation handle")
                .readers[compressor_tap]
                .consumed_sequence(),
            host.test_observation_handle(gate_effect)
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

/// Issue #1534: the meter frame leaves a tap out until a window that opened at or after its latest
/// subscribe closes, for a re-subscribe after an unsubscribe and for a re-arm of an armed tap.
///
/// Test value: red if the meter frame folds a tap's window from before its latest subscribe, so a
/// re-subscribed (or re-armed) tap shows its old reduction as new, or if the fold drops the first
/// window the subscribe opened. No other test re-subscribes a tap and then reads the meter frame.
#[test]
fn a_resubscribed_tap_leaves_the_meter_frame_until_its_first_new_window() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 2, Some(0));
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);

    let selection = ObservationSelection {
        track_id: "t0",
        rack: LiveEffectRack::Inserts,
        effect_slot_id: "comp",
        tap_id: 1,
        channels: ObservationReadChannels::Both,
    };
    // The tap's own cell, read without consuming it.
    let cell_first_sample = |host: &mut AudioWorkletEngineHost| {
        let ready = host.ready.as_ref().expect("ready ownership");
        let (effect, tap) = resolve_observation(ready, &selection).expect("resolved selection");
        host.test_observation_handle(effect)
            .expect("observation handle")
            .readers[tap]
            .read()
            .map(|window| window.first_sample)
    };

    let mut block = 0_u64;
    for _ in 0..32 {
        feed_and_render_tracks(&mut host, block, 0.5);
        block += 1;
    }
    while host.poll_meters() != 0 {}
    let (armed, armed_master) = gain_reduction(&host);
    assert!(armed[0] > 0.0, "the stale value the defect leaks");
    assert_eq!(armed_master, Some(armed[0]));

    // Unsubscribe: the frame leaves the tap out at once, though its cell keeps the last window.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, false), RESULT_OK);
    for _ in 0..2 {
        feed_and_render_tracks(&mut host, block, 0.5);
        block += 1;
        while host.poll_meters() != 0 {
            let (slots, master) = gain_reduction(&host);
            assert_eq!(slots[0].to_bits(), 0.0_f32.to_bits(), "disarmed tap");
            assert_eq!(master, None);
        }
    }
    assert!(
        cell_first_sample(&mut host).is_some(),
        "the cell keeps a window"
    );

    // Renders `blocks` blocks, polling every frame. A frame whose tap has no window since `arm`
    // reads zero and an absent master; a frame whose tap has one agrees with it.
    let drive = |host: &mut AudioWorkletEngineHost, block: &mut u64, arm: u64, blocks: u32| {
        let (mut stale_frames, mut fresh_frames) = (0_u32, 0_u32);
        let mut first_fresh = None;
        for _ in 0..blocks {
            feed_and_render_tracks(host, *block, 0.5);
            *block += 1;
            let first_sample = cell_first_sample(host);
            let fresh = first_sample.is_some_and(|first| first >= arm);
            if fresh && first_fresh.is_none() {
                first_fresh = first_sample;
            }
            while host.poll_meters() != 0 {
                let (slots, master) = gain_reduction(host);
                if fresh {
                    fresh_frames += 1;
                    assert!(slots[0] > 0.0, "a fresh window folds: {slots:?}");
                    assert_eq!(master, Some(slots[0]));
                } else {
                    stale_frames += 1;
                    assert_eq!(
                        slots[0].to_bits(),
                        0.0_f32.to_bits(),
                        "a window from before the subscribe is left out"
                    );
                    assert_eq!(master, None);
                }
            }
        }
        assert!(stale_frames > 0, "a frame before the first new window");
        assert!(fresh_frames > 0, "a frame after the first new window");
        assert_eq!(
            first_fresh,
            Some(arm),
            "the first new window opens at the arm"
        );
    };

    // Leg 1: re-subscribe after the unsubscribe.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 8, true), RESULT_OK);
    let resubscribed = host.command_report().applied_at_sample;
    assert!(
        cell_first_sample(&mut host).is_some_and(|first| first < resubscribed),
        "a stale window exists to leak"
    );
    drive(&mut host, &mut block, resubscribed, 12);

    // Leg 2: re-arm an armed tap, the SDK `update()` path.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 4, true), RESULT_OK);
    let rearmed = host.command_report().applied_at_sample;
    assert!(
        cell_first_sample(&mut host).is_some_and(|first| first < rearmed),
        "the armed tap's old window predates the re-arm"
    );
    drive(&mut host, &mut block, rearmed, 8);
}

/// Issue #143 E8: misuse and the typed refusals of an observation subscription.
///
/// Test value: red if a refused submission arms or moves anything (the "nothing was armed or
/// moved" assertion), or if an operation on a tap whose one slot an earlier submission holds is
/// admitted again before a block has drained it (the `RESULT_BACKPRESSURE` assertions).
#[test]
fn observation_misuse_is_typed_and_all_or_nothing() {
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 2, Some(0));
    assert_eq!(host.set_meter_lease(true), RESULT_OK);

    // A tap id the effect does not declare.
    assert_eq!(
        observe(&mut host, 0, 1, 0, 9, 2, true),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_UNKNOWN_TAP);
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
    // A subscription is an ordered queue record (#1345 D2) and the effect's queue holds one per
    // declared tap, so a second one in the same interval is typed backpressure that changes
    // nothing, and is accepted once a block has drained the first.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 8, true), RESULT_OK);
    assert_eq!(
        observe(&mut host, 0, 1, 0, 1, 2, true),
        RESULT_BACKPRESSURE,
        "the queue's one slot is taken until a block drains it"
    );
    assert_eq!(host.command_report().admitted, 0);
    feed_and_render_tracks(&mut host, 0, 0.5);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    for block in 1..9 {
        feed_and_render_tracks(&mut host, block, 0.5);
    }
    assert!(host.poll_meters() > 0);
    assert_eq!(
        host.meter_header().end_sample - host.meter_header().first_sample,
        2 * u64::from(QUANTUM),
        "the second subscription's window length is the one in force"
    );
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 0, false), RESULT_OK);
    feed_and_render_tracks(&mut host, 9, 0.5);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 0, false), RESULT_OK);
    feed_and_render_tracks(&mut host, 10, 0.5);

    // A lease release and retake restarts the frame sequence and the reported window.
    assert_eq!(host.set_meter_lease(false), RESULT_OK);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    assert_eq!(host.meter_header().sequence, 0);
    assert_eq!(host.meter_header().first_sample, 0);
    assert!(host.meter_frame().iter().all(|value| *value == 0.0));

    // A submission names each tap once: a record overwrites its tap's whole state and a
    // submission drains in one block, so a second record for the tap is malformed at its own wire
    // index, and nothing of the batch is armed or moved (#1345 D2). The queue's one slot is taken
    // by an earlier submission, so the duplicate is refused before the room pass: had the room
    // pass come first, the answer would be backpressure, which invites a retry that can never
    // succeed.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    feed_and_render_tracks(&mut host, 11, 0.5);
    let before = host.meter_frame().to_vec();
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);
    for index in 0..2 {
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
        host.submit_commands(2),
        RESULT_INVALID_ARGUMENT,
        "a submission naming one tap twice is malformed, whatever room the queue has"
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_MALFORMED);
    assert_eq!(host.command_report().rejected_index, 1);
    assert_eq!(host.command_report().admitted, 0);
    assert_eq!(
        host.meter_frame(),
        &before[..],
        "nothing was armed or moved"
    );

    // The same tap in a later submission, with the queue still full from the earlier one, is
    // backpressure: records of earlier submissions are not yet drained, and a block clears it.
    assert_eq!(
        observe(&mut host, 0, 1, 0, 1, 2, true),
        RESULT_BACKPRESSURE,
        "the queue's one slot is taken until a block drains it"
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_BACKPRESSURE);
    assert_eq!(host.command_report().admitted, 0);
    feed_and_render_tracks(&mut host, 12, 0.5);
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), RESULT_OK);

    // And a batch beyond the staging capacity is malformed before any queue is consulted.
    assert_eq!(
        host.submit_commands(MAXIMUM_COMMAND_RECORDS + 1),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(host.command_report().reason, COMMAND_REASON_MALFORMED);
}

/// #1345 D2: a submission names each observation tap once. An `Observe` record overwrites its tap's
/// whole state and one submission drains in one block, so the second record for a tap is malformed
/// at its own wire index, before the room pass, any cell write or the stamp, whatever the queue's
/// room. Two taps of different effects in one submission are fine.
///
/// Test value: red if the duplicate check goes, or runs after the room pass (the second record then
/// finds no room in the tap's one-slot queue and the answer is backpressure, which invites a retry
/// that can never succeed; the reason code is what catches it, and the armed-tap assertion states
/// the all-or-nothing outcome beside it), or if it refuses taps of different
/// effects.
#[test]
fn a_submission_names_each_observation_tap_once() {
    const QUANTUM: u32 = 128;
    let mut host = same_track_observation_host(QUANTUM);
    let mut clean = same_track_observation_host(QUANTUM);
    feed_and_render_tracks(&mut host, 0, 0.5);
    feed_and_render_tracks(&mut clean, 0, 0.5);

    // Two effects' taps, one record each, are admitted whole.
    stage_observe_subscribe_at(&mut host, 0, 0, 0);
    stage_observe_subscribe_at(&mut host, 1, 0, 1);
    assert_eq!(host.submit_commands(2), RESULT_OK);
    assert_eq!(host.command_report().admitted, 2);
    feed_and_render_tracks(&mut host, 1, 0.5);
    feed_and_render_tracks(&mut clean, 1, 0.5);
    let armed = host.observation_armed_taps();
    assert_eq!(armed, 2);

    // The first effect's tap armed, then withdrawn: refused at the withdrawal, the second record,
    // with the queue empty and nothing armed or moved.
    stage_observe_subscribe_at(&mut host, 0, 0, 0);
    stage_command(
        &mut host,
        1,
        COMMAND_OBSERVE_UNSUBSCRIBE,
        RACK_INSERTS,
        255,
        0,
        0,
        1,
        2,
        [0.0; 4],
    );
    assert_eq!(host.submit_commands(2), RESULT_INVALID_ARGUMENT);
    let report = *host.command_report();
    assert_eq!(report.reason, COMMAND_REASON_MALFORMED);
    assert_eq!(report.rejected_index, 1, "the second record of the tap");
    assert_eq!(report.admitted, 0);
    let ready = host.ready.as_ref().expect("ready");
    assert!(ready.shape.in_flight.iter().all(|count| *count == 0));
    assert_eq!(host.observation_armed_taps(), armed);
    feed_and_render_tracks(&mut host, 2, 0.5);
    feed_and_render_tracks(&mut clean, 2, 0.5);
    assert_eq!(
        host.output_pcm().expect("host output"),
        clean.output_pcm().expect("clean output"),
        "the refused batch moved no rendered sample"
    );

    // The refusal left the tap usable: the same tap alone is admitted.
    stage_observe_subscribe_at(&mut host, 0, 0, 0);
    assert_eq!(host.submit_commands(1), RESULT_OK);
}

/// A session with no observation taps has no command staging, so a subscription is refused as
/// unsupported (#1565A D4): the effect exists, the tap is declared, and this preparation bound no
/// lane and no staging to carry it.
#[test]
fn a_subscription_without_capacity_is_unsupported() {
    const QUANTUM: u32 = 128;
    let document = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
    let options = WebBootOptions {
        source_ring_frames: QUANTUM * 4,
        live_control_meter_blocks: DEFAULT_METER_BLOCKS as u64,
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
    assert!(!host.observation_attached());
    assert_eq!(host.resources().observation_retained_bytes, 0);
    assert!(
        host.command_staging_mut().is_none(),
        "no taps means no staging"
    );
    assert_eq!(
        host.submit_commands(1),
        RESULT_UNSUPPORTED,
        "this preparation cannot deliver a subscription"
    );
    assert_eq!(
        host.command_report().reason,
        COMMAND_REASON_UNSUPPORTED_KIND
    );
    // And the frame is the pre-#143 shape plus the positional gain-reduction section, all zero.
    let tracks = host.live_control_tracks().len();
    assert_eq!(host.meter_frame().len(), tracks * 3 + 3);
    assert!(host.meter_frame().iter().all(|value| *value == 0.0));
}

/// The observation words are validated at configuration time: more taps than the cap, a master
/// designation with no taps to designate it from, and the reserved word are all refused.
#[test]
fn observation_configuration_words_are_validated() {
    let document = one_track_session(128);
    for options in [
        WebBootOptions {
            live_control_observation_taps: u64::from(MAXIMUM_OBSERVATION_TAPS) + 1,
            ..boot_options(128)
        },
        WebBootOptions {
            live_control_master_track_plus_one: 1,
            ..boot_options(128)
        },
        WebBootOptions {
            reserved1: 1,
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
    const WINDOW_BLOCKS: u32 = 2;
    const BLOCKS: u64 = 12;

    let document = include_str!("../tests/browser-v1/observation-session.json");
    let run = |taps: u64| -> (String, f32, Option<f32>, f32, u64, u32, u32) {
        let options = WebBootOptions {
            source_ring_frames: QUANTUM,
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
            // A boot with no taps has no command staging; its submission is refused as such.
            if host.command_staging_mut().is_some() {
                stage_command(host, 0, kind, 1, 255, 0, 0, tap, window, [0.0; 4]);
            }
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
    assert_eq!(unbound, COMMAND_REASON_UNSUPPORTED_KIND);

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

    // The queries survive a sticky failure: nothing here is dropped on the failure path, so a
    // diagnosing consumer can still read the session map.
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

    assert_eq!(miso_engine_web_v1_dispose(handle), RESULT_OK);
    assert_eq!(miso_engine_web_v1_source_count(handle), 0);
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
    single
        .start_spectrum_stream(SpectrumSmoothingConfig::default(), None)
        .expect("single explicit-hop start");
    assert_eq!(
        single
            .spectrum_stream_cadence()
            .expect("single stream cadence")
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
        collection.select_spectrum_target(target.as_ref(), SpectrumChannels::Stereo, None, None),
        Ok(false)
    );
    collection
        .start_spectrum_stream(SpectrumSmoothingConfig::default(), None)
        .expect("collection explicit-hop start");
    assert_eq!(
        collection
            .spectrum_stream_cadence()
            .expect("collection stream cadence")
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
    host.start_spectrum_stream(SpectrumSmoothingConfig::default(), None)
        .expect("default-hop start");
    assert_eq!(
        host.spectrum_stream_cadence().expect("stream cadence"),
        SpectrumCadence::new(48_000, 128).expect("default cadence")
    );
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

fn strip_options(meter_blocks: u64, master_plus_one: u64) -> WebBootOptions {
    WebBootOptions {
        source_ring_frames: STRIP_QUANTUM * 4,
        live_control_meter_blocks: meter_blocks,
        live_control_observation_taps: 1,
        live_control_master_track_plus_one: master_plus_one,
        ..boot_options(STRIP_QUANTUM)
    }
}

/// Gate 1's hosts: A over the bus session, B over the reference session.
fn strip_pair(
    insert: Option<session::Effect>,
    meter_blocks: u64,
) -> (AudioWorkletEngineHost, AudioWorkletEngineHost) {
    let (bus, reference) = strip_pair_documents(insert);
    (
        strip_boot(&bus, strip_options(meter_blocks, 0)),
        strip_boot(&reference, strip_options(meter_blocks, 0)),
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
    let (mut a, mut b) = strip_pair(None, WINDOW);
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

/// The gain-reduction word a tap's own latest window reads as: the same `observed_decibels` of the
/// larger lane that `poll_meters` folds into the strip's slot.
fn tap_gain_reduction(host: &mut AudioWorkletEngineHost, effect_index: u32) -> f32 {
    let rows = host
        .read_observation_addresses(&[ObservationAddress {
            track_index: 0,
            rack: LiveEffectRack::Inserts,
            effect_index,
            tap_id: 1,
            channels: ObservationReadChannels::Both,
        }])
        .expect("a selected read of the tap");
    let (Some(left), Some(right)) = (rows[0].left, rows[0].right) else {
        panic!("the tap has no published window: {:?}", rows[0].status);
    };
    let ready = host.ready.as_ref().expect("ready");
    let tap = ready.shape.effect_instances[effect_index as usize]
        .as_ref()
        .filter(|instance| instance.reader.is_some())
        .expect("observation handle")
        .descriptor
        .observations[0];
    observed_decibels(tap, left.max(right))
}

/// #1345 Amendment 6: the gain-reduction section is recomputed from the taps armed when each frame
/// is polled. Two taps (the compressor's and the gate's) reduce on one track; the slot is the larger
/// of their latest windows, and withdrawing a tap takes only that tap out, at the first frame
/// polled after the withdrawal was acknowledged.
///
/// Test value: red if a withdrawn tap's last window outlives its withdrawal for any number of
/// polls (a fold that ages out over `windowBlocks`) while the existing bus test, which only looks
/// after `2 * WINDOW` blocks and a full drain, stays green; and red if withdrawing one tap clears
/// the whole strip's slot or every armed tap of the track, which no test with one tap per strip
/// can see.
#[test]
fn a_withdrawn_tap_leaves_the_first_polled_frame_and_only_its_own_reading() {
    const QUANTUM: u32 = 128;
    const WINDOW: u64 = 2;
    let mut host = same_track_observation_host_metered(QUANTUM, WINDOW);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    // One submission arms both taps; a tap's queue holds one record until a block drains it.
    for effect in 0..2 {
        stage_observe_subscribe_at(&mut host, effect, 0, effect as u32);
    }
    assert_eq!(host.submit_commands(2), RESULT_OK);
    let mut block = 0;
    let mut render_window = |host: &mut AudioWorkletEngineHost| {
        for _ in 0..WINDOW {
            feed_and_render_channels(host, block, 0.75, 0.125);
            block += 1;
        }
    };
    for _ in 0..8 {
        render_window(&mut host);
    }
    while host.poll_meters() != 0 {}
    let reduction = [
        tap_gain_reduction(&mut host, 0),
        tap_gain_reduction(&mut host, 1),
    ];
    assert!(
        reduction[0] > 0.0 && reduction[1] > 0.0 && reduction[0] != reduction[1],
        "both taps reduce, by different amounts: {reduction:?}"
    );
    let (larger, smaller) = if reduction[0] > reduction[1] {
        (0, 1)
    } else {
        (1, 0)
    };
    assert_eq!(
        strip_gain_word(&host, 0).to_bits(),
        reduction[larger].to_bits(),
        "the slot is the larger tap's window"
    );

    // Withdraw the larger tap. The first frame polled after the acknowledgement carries the other
    // tap's window, bit for bit: not the larger one still, not zero.
    assert_eq!(
        observe(
            &mut host,
            0,
            RACK_INSERTS,
            larger as u32,
            1,
            WINDOW as u32,
            false
        ),
        RESULT_OK
    );
    render_window(&mut host);
    assert!(host.poll_meters() >= 1, "a frame follows the withdrawal");
    let remaining = tap_gain_reduction(&mut host, smaller as u32);
    assert!(
        remaining > 0.0,
        "the remaining tap still reduces: {remaining}"
    );
    assert_eq!(
        strip_gain_word(&host, 0).to_bits(),
        remaining.to_bits(),
        "the remaining tap alone sets the slot"
    );

    // Withdraw the other one: the very next frame polled reads exactly zero.
    assert_eq!(
        observe(
            &mut host,
            0,
            RACK_INSERTS,
            smaller as u32,
            1,
            WINDOW as u32,
            false
        ),
        RESULT_OK
    );
    render_window(&mut host);
    assert!(host.poll_meters() >= 1, "a frame follows the withdrawal");
    assert_eq!(
        strip_gain_word(&host, 0).to_bits(),
        0,
        "no tap is armed, so the slot is +0.0 at once"
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
    let mut host = strip_boot(&bus, strip_options(WINDOW, u64::from(STRIP_BUS) + 1));
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
        strip_options(WINDOW, u64::from(STRIP_BUS) + 2),
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

/// Issue #1213 gate 7, as #1565A leaves it: admitting a bus observation subscription, then
/// rendering, allocates and frees nothing. The bus value edits of the gate's earlier form go
/// through the core now (`control_command` is control work and allocates by design).
///
/// Test value: red if strip-sized observation admission allocates on the admission or render path.
/// The bus gain-reduction fold runs in `poll_meters`, outside the measured closure; #1209's
/// `bus_meters_render_and_poll_without_allocating` covers it.
#[test]
fn a_bus_observation_admits_and_renders_without_allocating() {
    let (mut a, _) = strip_pair(None, 2);
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
        COMMAND_OBSERVE_SUBSCRIBE,
        RACK_INSERTS,
        255,
        STRIP_BUS,
        0,
        1,
        2,
        [0.0; 4],
    );
    let planes: Vec<[Vec<f32>; 2]> = (0..3).map(|feeder| strip_planes(feeder, 2)).collect();
    for (feeder, planes) in planes.iter().enumerate() {
        submit_strip_source(&mut a, &format!("t{feeder}"), 2, planes);
    }
    let ((admission, render), allocations, deallocations) =
        crate::ffi::live_response_ffi_tests::measured(|| (a.submit_commands(1), a.render_next()));
    assert_eq!(admission, RESULT_OK, "the bus observation is admitted");
    assert_eq!(render, RESULT_OK);
    assert_eq!(allocations, 0, "admission/render allocated");
    assert_eq!(deallocations, 0, "admission/render freed");
}

// Issue #1222: browser live send commands. A send is addressed by its live-route index: its
// position among the routes into submixes, in canonical route-ID order.

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
        strip_options(0, 0),
    );
    let mut zero = strip_boot(
        &subnormal_route_document([0.0, 0.0, 0.5, 0.0]),
        strip_options(0, 0),
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

// ---------------------------------------------------------------------------------------------
// #1400 gate 4: the browser's preparer, three branches, at boot and as a successor
// ---------------------------------------------------------------------------------------------

/// The shape of a spectrum request a preparer was given, and what its plan's companions hold.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompanionCapture {
    None,
    Single,
    Collection,
}

fn companion_request(capture: CompanionCapture) -> Option<SpectrumPreparationRequest> {
    let target = SpectrumTarget::TrackPostMatrix("eq0".into());
    match capture {
        CompanionCapture::None => None,
        CompanionCapture::Single => {
            Some(SpectrumPreparationRequest::Single(SpectrumCaptureRequest {
                target,
                channels: SpectrumChannels::Stereo,
                maximum_capture_bytes: u64::MAX,
            }))
        }
        CompanionCapture::Collection => Some(SpectrumPreparationRequest::Collection(
            SpectrumCaptureCollectionRequest {
                entries: vec![host_core::SpectrumCaptureCollectionEntry {
                    target,
                    channels: SpectrumChannels::Stereo,
                }],
                maximum_capture_bytes: u64::MAX,
            },
        )),
    }
}

/// The attachment's charge is checked against the plan caps before the attachment is boxed. The
/// rows the cap governs are the graph, the plan and the compiled model together, so the check adds
/// the model to the graph row; one byte under the charged total refuses with host-core's own
/// graph code, and one byte under the largest allocation with its allocation code. Host-core's
/// admission already counts effect and route controls, which these rows leave out, so for a
/// session it admitted this check is a backstop; it is tested on the report's own figures.
///
/// Red if the charge is added without a cap check, or the check leaves the model out, so a plan
/// that fits only without its companions is boxed and published.
#[test]
fn the_attachment_charge_is_checked_against_the_caps() {
    let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
    let options = WebBootOptions {
        source_ring_frames: 512,
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        ..WebBootOptions::live_control_defaults()
    };
    let parsed = parse_host_session(document).expect("nine-track parse");
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
    .expect("nine-track compile");
    let caps = prepare_caps(&compiled, options, 512, u64::MAX);
    let (host, _, _) = prepare_web_host(
        &compiled,
        &caps,
        &live_control_request(options, 128).expect("request"),
        None,
        controls::live_lanes(compiled.normalized_model()).into(),
        &host_core::LiveValuesSuperseded::new(),
        None,
    )
    .expect("reference preparation");
    let base = host.report;
    let (retained, largest) = (808_u64, base.largest_engine_allocation_bytes + 1);
    let total = base.graph_session_plus_plan_bytes + retained + base.session_model_bytes;
    let limit = |graph: u64, named: u64| HostPrepareCaps {
        maximum_graph_session_plus_plan_bytes: graph,
        maximum_named_allocation_bytes: named,
        ..caps
    };
    let diagnostic = |code: &str| PrepareDiagnostics::fixed(PrepareRejection::Resource, code);

    let mut report = base;
    assert_eq!(
        charge_attachment(&mut report, &limit(total - 1, u64::MAX), retained, largest),
        Err(diagnostic("host.graph.resource.limit"))
    );
    assert_eq!(
        charge_attachment(
            &mut report,
            &limit(u64::MAX, largest - 1),
            retained,
            largest
        ),
        Err(diagnostic("host.resource.limit"))
    );
    assert_eq!(report, base, "a refusal charges nothing");
    assert_eq!(
        charge_attachment(&mut report, &limit(total, largest), retained, largest),
        Ok(())
    );
    assert_eq!(
        report.graph_session_plus_plan_bytes,
        base.graph_session_plus_plan_bytes + retained
    );
    assert_eq!(
        report.graph_incremental_plan_bytes,
        base.graph_incremental_plan_bytes + retained
    );
    assert_eq!(report.largest_engine_allocation_bytes, largest);
}

/// Gate 4. Red if one branch drops its capture or leaves a meter consumer or an observation
/// handle in the returned handles (the control plane would drop it on the control thread and its
/// meter would go silent), if the plan carries no companions, or if a successor ignores its base
/// (every unchanged source would allocate a new ring, so nothing would be carried).
#[test]
fn web_runtime_preparer_attaches_the_companions_on_every_branch() {
    use control_plane::RuntimePreparer;

    let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
    let options = WebBootOptions {
        source_ring_frames: 512,
        live_control_observation_taps: 4,
        ..WebBootOptions::live_control_defaults()
    };
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: 128,
        ..options
    };
    let parsed = parse_host_session(document).expect("nine-track parse");
    let compile_caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    let compiled = compile_host_model(&parsed, compile_caps).expect("nine-track compile");
    let model = compiled.normalized_model();
    let strips = model.tracks.len() + model.submixes.len();
    assert_eq!(model.tracks.len(), 9, "the nine-track fixture");
    let caps = prepare_caps(&compiled, options, 512, u64::MAX);
    let lanes = controls::live_lanes(model).into();

    for capture in [
        CompanionCapture::None,
        CompanionCapture::Single,
        CompanionCapture::Collection,
    ] {
        let preparer = WebRuntimePreparer::new(options, companion_request(capture));
        let superseded = host_core::LiveValuesSuperseded::new();
        let check =
            |host: &mut PreparedHost, handles: &host_core::HostLiveControlHandles, at: &str| {
                assert!(
                    handles.meters.is_empty(),
                    "{capture:?} {at}: no meter in the handles"
                );
                assert!(
                    handles.effect_observations.is_empty(),
                    "{capture:?} {at}: no observation handle in the handles"
                );
                let companions = host
                    .plan
                    .host_attachment_mut::<RenderCompanions>()
                    .unwrap_or_else(|| panic!("{capture:?} {at}: the plan carries its companions"));
                assert_eq!(
                    companions.meters().len(),
                    strips,
                    "{capture:?} {at}: one meter per strip"
                );
                let shape = match companions.spectrum_capture {
                    None => CompanionCapture::None,
                    Some(PreparedSpectrumCaptures::Single(_)) => CompanionCapture::Single,
                    Some(PreparedSpectrumCaptures::Collection(_)) => CompanionCapture::Collection,
                };
                assert_eq!(shape, capture, "{capture:?} {at}: the branch's capture");
                assert_eq!(
                    companions.has_spectrum_capture(),
                    capture != CompanionCapture::None
                );
            };

        let (mut boot, boot_handles) = preparer
            .prepare(&compiled, &caps, lanes, None, &superseded)
            .unwrap_or_else(|failure| panic!("{capture:?} boot: {failure:?}"));
        check(&mut boot, &boot_handles, "boot");
        // The attachment is charged in the plan's rows (the control plane builds its plan
        // resource row from this report): the same preparation without the attachment, plus the
        // attachment's own box and vector buffers, in both plan rows and in the largest allocation.
        let (reference, _, _) = prepare_web_host(
            &compiled,
            &caps,
            &live_control_request(options, 128).expect("request"),
            companion_request(capture).as_ref(),
            lanes,
            &superseded,
            None,
        )
        .unwrap_or_else(|failure| panic!("{capture:?} reference: {failure:?}"));
        let (retained, largest) = boot
            .plan
            .host_attachment_mut::<RenderCompanions>()
            .and_then(|companions| companions.retained_bytes())
            .expect("the companions' retained bytes");
        assert!(
            retained >= (strips * size_of::<MeterConsumer>()) as u64,
            "{capture:?}: the meter consumers' buffer is charged"
        );
        assert_eq!(
            boot.report.graph_session_plus_plan_bytes,
            reference.report.graph_session_plus_plan_bytes + retained,
            "{capture:?}: the attachment is charged in the plan row"
        );
        assert_eq!(
            boot.report.graph_incremental_plan_bytes,
            reference.report.graph_incremental_plan_bytes + retained,
            "{capture:?}: and in the incremental row"
        );
        assert_eq!(
            boot.report.largest_engine_allocation_bytes,
            reference
                .report
                .largest_engine_allocation_bytes
                .max(largest),
            "{capture:?}: and in the largest allocation"
        );
        assert_eq!(
            boot.report.carried_source_total_bytes, 0,
            "{capture:?}: boot carries none"
        );

        let edit_ramps = session::EditRamps::new();
        let base = host_core::SuccessorBase {
            inventory: &boot.inventory,
            committed: model,
            revision: 1,
            edit_ramps: &edit_ramps,
            monitor: host_core::MonitorBase::NONE,
        };
        let (mut successor, successor_handles) = preparer
            .prepare(&compiled, &caps, lanes, Some(base), &superseded)
            .unwrap_or_else(|failure| panic!("{capture:?} successor: {failure:?}"));
        check(&mut successor, &successor_handles, "successor");
        assert!(
            successor.report.carried_source_total_bytes > 0,
            "{capture:?}: the successor prepares the unchanged source vacant"
        );
    }
}

// Issue #1290a: the browser host applies a structural transaction while it plays. The control half
// prepares the successor from the committed model, publishes it and returns; the render half
// adopts it at a block boundary. The native gates run the one instance on one thread, the
// message handler's place between render calls (decision 17, "single instance first"), through
// the test-support switch that lets the rebuild arm run past `refuse_rebuild` (deleting that
// refusal is #1562b's).
//
// The sessions here have no console slot and no insert: an effect's state carries across a swap
// only with #1280a, and a session that holds one would not render the same bits after the swap.
#[cfg(feature = "test-support")]
mod structural_apply {
    use super::*;
    use protocol::{
        CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
        EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
        StatusCode, TypedCommandFrame,
    };
    use session::{MatrixOrPan, RouteSource, SessionModel, StableId};

    const FIXTURE: &str =
        include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
    const SOURCE: &[u8] = b"fixture-source";
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 14;
    /// The block before which the structural transaction runs.
    const APPLY_BEFORE: u64 = 6;
    const ADDED: &str = "a-muted";

    /// Nine stereo tracks over one source with both console sections empty and no insert, every
    /// track's high-pass and low-pass enabled at its own cutoffs, and a non-centre pan.
    fn gate_model() -> SessionModel {
        let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            track.console.clear();
            track.inserts.effects.clear();
            let step = index as f32;
            for (side, channel) in [&mut track.builtins.left, &mut track.builtins.right]
                .into_iter()
                .enumerate()
            {
                channel.hpf_hz = 30.0 + 11.0 * step + 7.0 * side as f32;
                channel.lpf_hz = 9_000.0 - 450.0 * step - 600.0 * side as f32;
            }
            track.matrix_or_pan = MatrixOrPan::Pan {
                left: 0.9 - 0.05 * step,
                right: 0.4 + 0.05 * step,
                smoothing_samples: 16,
            };
        }
        model
    }

    /// A muted copy of the first track, whose ID sorts first, and its route to the output.
    fn muted_track(model: &SessionModel, id: &str) -> (session::Track, session::Route) {
        let mut track = model.tracks[0].clone();
        track.id = StableId::parse(id).expect("track ID");
        track.fader.left_mute = true;
        track.fader.right_mute = true;
        let mut route = model.routes[0].clone();
        route.id = StableId::parse(&format!("{id}-main")).expect("route ID");
        let RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        (track, route)
    }

    fn add_muted_track(model: &SessionModel, id: &str) -> Vec<SessionEdit> {
        let (track, route) = muted_track(model, id);
        vec![
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ]
    }

    /// The committed model after [`add_muted_track`]: what a fresh boot is compared against.
    fn with_muted_track(model: &SessionModel, id: &str) -> SessionModel {
        let (track, route) = muted_track(model, id);
        let mut next = model.clone();
        next.tracks.insert(0, track);
        next.routes.push(route);
        next.revision += 1;
        next
    }

    fn frame(edits: &[SessionEdit], request: u64, revision: u64) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        bytes.truncate(length);
        bytes
    }

    /// What one transaction's response frame says.
    #[derive(Debug, Eq, PartialEq)]
    enum Outcome {
        Applied { path: EditPath, revision: u64 },
        Refused(StatusCode, Vec<String>),
    }

    fn outcome(response: &[u8]) -> Outcome {
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
            .expect("a transaction response")
        {
            DecodedTypedResponseFrame::Success { header, payload } => {
                let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload
                else {
                    panic!("a transaction success")
                };
                Outcome::Applied {
                    path: applied.edit_path,
                    revision: header.revision.0,
                }
            }
            DecodedTypedResponseFrame::NonOk { header, payload } => Outcome::Refused(
                header.status,
                payload
                    .diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.code.to_string())
                    .collect(),
            ),
        }
    }

    fn options() -> WebBootOptions {
        WebBootOptions {
            live_control_meter_blocks: 2,
            live_control_observation_taps: 4,
            ..WebBootOptions::explicit_defaults()
        }
    }

    /// One booted host, the blocks it rendered (as bits) and the next block to feed.
    struct Rig {
        host: AudioWorkletEngineHost,
        block: u64,
        bits: Vec<Vec<u32>>,
    }

    impl Rig {
        fn boot(
            model: &SessionModel,
            options: WebBootOptions,
            spectrum: Option<SpectrumPreparationRequest>,
        ) -> Self {
            let document = canonical_session_json(model).expect("canonical session");
            let host =
                AudioWorkletEngineHost::boot_with_spectrum(document.as_bytes(), options, spectrum)
                    .expect("the session boots");
            Self {
                host,
                block: 0,
                bits: Vec::new(),
            }
        }

        /// Feed the next block to the source, then render it.
        fn step(&mut self) {
            let quantum = u64::from(QUANTUM);
            let lane = |lane: u64| {
                (self.block * quantum..(self.block + 1) * quantum)
                    .map(|frame| strip_sample(0, lane, frame))
                    .collect::<Vec<f32>>()
            };
            let planes = [lane(0), lane(1)];
            let views: [&[f32]; 2] = [&planes[0], &planes[1]];
            assert_eq!(
                self.host.submit_source(
                    SOURCE,
                    1,
                    self.block * quantum,
                    48_000,
                    &views,
                    QUANTUM,
                    false
                ),
                RESULT_OK,
                "block {}: the source feed",
                self.block
            );
            assert_eq!(self.host.render_next(), RESULT_OK, "block {}", self.block);
            self.bits.push(
                self.host
                    .output_pcm()
                    .expect("output")
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect(),
            );
            self.block += 1;
        }

        /// Apply a structural transaction through the control half and decode its response.
        fn apply(&mut self, edits: &[SessionEdit], request: u64) -> Outcome {
            let revision = self.host.committed_revision();
            let response = self
                .host
                .structural_command_for_test(&frame(edits, request, revision))
                .expect("the control plane answers the frame");
            outcome(&response)
        }

        fn epoch(&self) -> u64 {
            self.host.active_plan_epoch_for_test().expect("a plan")
        }

        /// Whether a committed replacement waits for the block that adopts it.
        fn pending(&self) -> bool {
            self.host
                .ready
                .as_ref()
                .expect("ready")
                .control
                .newest_plan_epoch()
                != self.epoch()
        }

        fn watermark(&mut self) -> WebPlanWatermark {
            assert_eq!(self.host.read_watermark(), RESULT_OK);
            *self.host.watermark()
        }
    }

    fn first_difference(left: &[Vec<u32>], right: &[Vec<u32>]) -> Option<(usize, usize)> {
        left.iter()
            .zip(right)
            .enumerate()
            .find_map(|(block, (a, b))| {
                a.iter()
                    .zip(b)
                    .position(|(x, y)| x != y)
                    .map(|sample| (block, sample))
            })
    }

    /// Gate 1, in its single-thread form: the structural transaction returns before the render
    /// half adopts, and from the adoption block on every block equals a fresh boot of the committed
    /// model fed the same PCM from frame zero.
    fn gap_free_acceptance(spectrum: impl Fn() -> Option<SpectrumPreparationRequest>) {
        let model = gate_model();
        let committed = with_muted_track(&model, ADDED);

        let mut fresh = Rig::boot(&committed, options(), spectrum());
        for _ in 0..BLOCKS {
            fresh.step();
        }

        let mut rig = Rig::boot(&model, options(), spectrum());
        for _ in 0..APPLY_BEFORE {
            rig.step();
        }
        let base = rig.host.committed_revision();
        assert_eq!(rig.epoch(), 0);
        let applied = rig.apply(&add_muted_track(&model, ADDED), 1);
        assert_eq!(
            applied,
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            },
            "the response is the committed revision and the rebuild path"
        );
        // It returned before render adopted: the plan that renders is the boot plan, a successor
        // waits, and the watermark is not early.
        assert_eq!(rig.epoch(), 0, "apply never waits for render");
        assert!(rig.pending());
        assert_eq!(
            rig.watermark().revision,
            base,
            "the watermark is never early"
        );
        assert_eq!(rig.host.blocking_rebuilds(), 1);

        while rig.block < BLOCKS {
            rig.step();
        }
        assert_eq!(rig.epoch(), 1);
        assert!(!rig.pending(), "render adopted the plan");
        assert_eq!(rig.host.service(), RESULT_OK);
        assert!(!rig.pending());
        assert_eq!(
            rig.host.status().next_absolute_sample,
            BLOCKS * u64::from(QUANTUM),
            "the clock continues"
        );
        let watermark = rig.watermark();
        assert_eq!(watermark.revision, base + 1);
        assert_eq!(watermark.outcome_flags, 1, "exact");
        assert_eq!(
            watermark.first_sample,
            APPLY_BEFORE * u64::from(QUANTUM),
            "the adoption block's first sample"
        );
        assert_eq!(
            first_difference(
                &rig.bits[APPLY_BEFORE as usize..],
                &fresh.bits[APPLY_BEFORE as usize..]
            ),
            None,
            "from the adoption block on, every block equals a fresh boot of the committed model"
        );
        assert_eq!(
            first_difference(
                &rig.bits[..APPLY_BEFORE as usize],
                &fresh.bits[..APPLY_BEFORE as usize]
            ),
            None,
            "and the blocks before it, since the added track is muted"
        );
        assert!(
            rig.bits[BLOCKS as usize - 1].iter().any(|bits| *bits != 0),
            "the blocks are not silence"
        );

        // A response capture after the swap is stamped with the continued clock.
        struct Sink;
        impl ResponseSnapshotSink for Sink {
            fn copy_owner(
                &mut self,
                _: engine::realtime::ResponseSnapshotOwnerInfo<'_>,
                _: &[engine::realtime::ResponseSnapshotSection],
                _: &[engine::realtime::ResponseSnapshotSection],
            ) -> Result<(), ResponseSnapshotError> {
                Ok(())
            }
        }
        let capture = rig
            .host
            .copy_response_snapshot(model.tracks[0].id.as_str(), &mut Sink)
            .expect("a response capture on the successor");
        assert_eq!(
            capture.captured_sample,
            BLOCKS * u64::from(QUANTUM),
            "the capture is stamped with the continued clock"
        );
    }

    /// Gate 1. Test value: red if the apply reboots internally (fresh rings or state: the filters
    /// and ramps restart and the bits differ), if the successor's clock restarts at zero, or if
    /// `apply` blocks until adoption (the plan that renders would already be the successor).
    #[test]
    fn a_rebuild_swaps_without_a_gap() {
        gap_free_acceptance(|| None);
    }

    fn single_capture() -> Option<SpectrumPreparationRequest> {
        Some(SpectrumPreparationRequest::Single(SpectrumCaptureRequest {
            target: SpectrumTarget::TrackPostMatrix(model_track_id().into()),
            channels: SpectrumChannels::Stereo,
            maximum_capture_bytes: u64::MAX,
        }))
    }

    fn capture_collection() -> Option<SpectrumPreparationRequest> {
        Some(SpectrumPreparationRequest::Collection(
            SpectrumCaptureCollectionRequest {
                entries: vec![host_core::SpectrumCaptureCollectionEntry {
                    target: SpectrumTarget::TrackPostMatrix(model_track_id().into()),
                    channels: SpectrumChannels::Stereo,
                }],
                maximum_capture_bytes: u64::MAX,
            },
        ))
    }

    fn model_track_id() -> String {
        gate_model().tracks[3].id.as_str().to_owned()
    }

    /// Gate 5. Test value: red if a successor of either spectrum branch is prepared without its
    /// capture, or by a different path than the boot's (the swap would refuse or the bits move).
    #[test]
    fn a_rebuild_swaps_with_a_spectrum_capture_or_a_collection_configured() {
        gap_free_acceptance(single_capture);
        gap_free_acceptance(capture_collection);
    }

    /// The ramp of the live fader edit, in samples: longer than the blocks between the edit and
    /// the adoption block, so the ramp is in flight when the plan swaps.
    const FADER_RAMP: u32 = 400;

    /// A live fader edit of strip `id` through the production control entry, as the SDK sends it.
    fn live_fader(rig: &mut Rig, id: &str, db: f32, request: u64) {
        let edits = [SessionEdit::SetControl {
            address: controls::ControlAddress {
                owner: controls::Owner::Strip(StableId::parse(id).expect("strip ID")),
                section: controls::Section::Builtins,
                control: 5,
                lane: session::ParameterChannel::Both,
            },
            value: db,
            ramp_samples: Some(FADER_RAMP),
        }];
        let bytes = frame(&edits, request, rig.host.committed_revision());
        let staging = rig
            .host
            .control_request_mut(bytes.len() as u32)
            .expect("the control staging");
        staging[..bytes.len()].copy_from_slice(&bytes);
        assert_eq!(rig.host.control_command(bytes.len() as u32), RESULT_OK);
        assert_eq!(
            outcome(rig.host.control_response()),
            Outcome::Applied {
                path: EditPath::Live,
                revision: rig.host.committed_revision()
            }
        );
    }

    /// Gate 2. Test value: red if the successor is prepared from the boot document instead of the
    /// committed model, which would lose the live edit (the fader would snap back after the swap).
    #[test]
    fn a_live_edit_is_part_of_the_base_the_successor_is_prepared_from() {
        // The edit starts at sample 640 and its ramp ends at 1040, in block 8: it crosses the
        // adoption at block 6.
        const EDIT_BEFORE: u64 = 5;
        const SETTLED_FROM: usize = 9;
        let model = gate_model();
        let edited_id = model.tracks[3].id.as_str().to_owned();
        let edited_db = -7.5;

        // The committed model: the live edit, then the added track.
        let mut committed = with_muted_track(&model, ADDED);
        let edited = committed
            .tracks
            .iter_mut()
            .find(|track| track.id.as_str() == edited_id)
            .expect("the edited track");
        edited.fader.left_db = edited_db;
        edited.fader.right_db = edited_db;
        let mut fresh = Rig::boot(&committed, options(), None);
        for _ in 0..BLOCKS {
            fresh.step();
        }

        // The same live edit and no structural edit: what the swap must not disturb.
        let mut live_only = Rig::boot(&model, options(), None);
        // The unedited session: the edit must be audible.
        let mut unedited = Rig::boot(&model, options(), None);
        let mut swapped = Rig::boot(&model, options(), None);
        for block in 0..BLOCKS {
            if block == EDIT_BEFORE {
                live_fader(&mut live_only, &edited_id, edited_db, 1);
                live_fader(&mut swapped, &edited_id, edited_db, 1);
            }
            if block == APPLY_BEFORE {
                let base = swapped.host.committed_revision();
                assert_eq!(
                    swapped.apply(&add_muted_track(&model, ADDED), 2),
                    Outcome::Applied {
                        path: EditPath::Rebuild,
                        revision: base + 1
                    }
                );
            }
            live_only.step();
            unedited.step();
            swapped.step();
        }
        assert_eq!(swapped.epoch(), 1);
        assert_eq!(
            first_difference(&swapped.bits, &live_only.bits),
            None,
            "the swap disturbs nothing, the ramp in flight included: every block equals the run \
             with the live edit alone"
        );
        let last = BLOCKS as usize - 1;
        assert_ne!(
            swapped.bits[last], unedited.bits[last],
            "the edit is audible after the swap"
        );
        assert_eq!(
            first_difference(&swapped.bits[SETTLED_FROM..], &fresh.bits[SETTLED_FROM..]),
            None,
            "once the ramp ends the output equals a fresh boot of the committed model"
        );
    }

    /// Everything a refusal must leave as it was.
    #[derive(Debug, Eq, PartialEq)]
    struct Untouched {
        revision: u64,
        epoch: u64,
        pending: bool,
        blocking_rebuilds: u32,
    }

    fn untouched(rig: &Rig) -> Untouched {
        Untouched {
            revision: rig.host.committed_revision(),
            epoch: rig.epoch(),
            pending: rig.pending(),
            blocking_rebuilds: rig.host.blocking_rebuilds(),
        }
    }

    /// Run the script with `refusal` called before block `APPLY_BEFORE`, and the same script
    /// without it; assert the refusal changed nothing the host can see.
    fn assert_refusal_changes_nothing(
        options: WebBootOptions,
        refusal: impl FnOnce(&mut Rig, &SessionModel) -> String,
    ) -> String {
        let model = gate_model();
        let mut without = Rig::boot(&model, options, None);
        for _ in 0..BLOCKS {
            without.step();
        }
        let mut with = Rig::boot(&model, options, None);
        for _ in 0..APPLY_BEFORE {
            with.step();
        }
        let before = untouched(&with);
        let refused = refusal(&mut with, &model);
        assert_eq!(
            untouched(&with),
            before,
            "a refusal leaves the revision, the plan and the counters as they were"
        );
        while with.block < BLOCKS {
            with.step();
        }
        assert_eq!(with.epoch(), 0, "no candidate was published");
        assert_eq!(with.host.status().state, STATE_READY);
        assert_eq!(
            first_difference(&with.bits, &without.bits),
            None,
            "the blocks after a refusal are bit-identical to a run without the call"
        );
        refused
    }

    /// Gate 3, case one. Test value: red if a malformed transaction reaches the preparer or leaves
    /// a candidate, a revision bump or a counted rebuild behind.
    #[test]
    fn a_malformed_transaction_is_refused_and_changes_nothing() {
        assert_refusal_changes_nothing(options(), |rig, model| {
            let mut bytes = frame(
                &add_muted_track(model, ADDED),
                1,
                rig.host.committed_revision(),
            );
            bytes.truncate(bytes.len() - 5);
            let error = rig
                .host
                .structural_command_for_test(&bytes)
                .expect_err("a truncated frame is refused");
            format!("{error:?}")
        });
    }

    /// Run one frame through the production control entry with the rebuild arm open, as the SDK's
    /// `control` message will once #1562b deletes the refusal. Returns the entry's result word and
    /// what it left to read.
    fn entry_with_rebuild_open(
        rig: &mut Rig,
        edits: &[SessionEdit],
        request: u64,
    ) -> (u32, Vec<u8>) {
        let bytes = frame(edits, request, rig.host.committed_revision());
        let staging = rig
            .host
            .control_request_mut(bytes.len() as u32)
            .expect("the control staging");
        staging[..bytes.len()].copy_from_slice(&bytes);
        let allow = Arc::clone(&rig.host.ready.as_ref().expect("ready").allow_rebuild);
        allow.store(true, Ordering::Release);
        let result = rig.host.control_command(bytes.len() as u32);
        allow.store(false, Ordering::Release);
        (result, rig.host.control_response().to_vec())
    }

    /// Gate 3, case two. Test value: red if a transaction that changes the rate or the quantum
    /// reaches publication (the plan exchange answers it as retryable backpressure, a retry that
    /// can never succeed), or if the entry reports it as anything but a required re-prepare.
    #[test]
    fn a_rate_or_quantum_change_is_refused_as_a_required_reprepare() {
        for edit in [
            SessionEdit::SetSampleRateHz {
                sample_rate_hz: 44_100,
            },
            SessionEdit::SetQuantumFrames {
                quantum_frames: 256,
            },
        ] {
            assert_refusal_changes_nothing(options(), |rig, _| {
                let (result, response) =
                    entry_with_rebuild_open(rig, std::slice::from_ref(&edit), 1);
                assert_eq!(result, RESULT_REPREPARE_REQUIRED, "{edit:?}");
                let text = String::from_utf8(response).expect("diagnostic text");
                assert!(
                    text.starts_with("control.apply.reprepare_required"),
                    "{text}"
                );
                text
            });
        }
    }

    /// Gate 3, case three. Test value: red if the projection charges the successor alone instead
    /// of the running plan and the successor together (D3): a budget that fits one plan but not
    /// two would then publish a replacement the page cannot hold.
    #[test]
    fn a_transaction_over_the_memory_budget_is_refused_and_changes_nothing() {
        let model = gate_model();
        let fits_one_plan =
            exact_retained_report_total(Rig::boot(&model, options(), None).host.resources());
        let tight = WebBootOptions {
            maximum_memory_bytes: fits_one_plan,
            ..options()
        };
        // The budget boots the session (the boot is inclusive of its own total) ...
        Rig::boot(&model, tight, None);
        // ... and refuses a second plan beside it.
        let refused = assert_refusal_changes_nothing(tight, |rig, model| {
            let error = rig
                .host
                .structural_command_for_test(&frame(
                    &add_muted_track(model, ADDED),
                    1,
                    rig.host.committed_revision(),
                ))
                .expect_err("a replacement that does not fit beside the running plan is refused");
            let control_plane::CommandError::CompileRejected(failure) = &error else {
                panic!("a typed refusal, not {error:?}")
            };
            String::from_utf8_lossy(&failure.diagnostic()).into_owned()
        });
        assert!(
            refused.contains("control.resource.replacement_peak"),
            "{refused}"
        );
        // The same transaction fits a budget with room for a second plan.
        let roomy = WebBootOptions {
            maximum_memory_bytes: fits_one_plan * 2,
            ..options()
        };
        let mut rig = Rig::boot(&model, roomy, None);
        assert!(matches!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied { .. }
        ));
    }

    /// Gate 6, in the form a host can drive without a browser: what is still bound to the boot
    /// plan's indices describes the successor from the adoption block on. The observation catalog
    /// (`observation_binding*`, the strip order) is the plan's tables, and the source table is the
    /// committed model's.
    ///
    /// Test value: red if the tables stay the boot plan's after the swap (a track added in front
    /// shifts every index: an observation armed at the catalog's index would arm another strip's
    /// effect and read another reader), or if an observation record submitted between the commit
    /// and the adoption block is refused (the C ABI admits it), is addressed by the rendering
    /// plan's tables or queued on its producers (it would arm another strip's effect, or none, and
    /// be lost with the retiring plan), or does not arm the successor's effect at its adoption.
    #[test]
    fn the_catalogs_describe_the_successor_from_the_adoption_block_on() {
        let mut model =
            parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
                .expect("accepted command fixture");
        model.tracks[0].inserts.effects[0] = observed_compressor_insert();
        let mut source = model.sources[0].clone();
        source.id = StableId::parse("a-source").expect("source ID");
        let mut track = model.tracks[0].clone();
        track.id = StableId::parse("a-track").expect("track ID");
        track.source_id = source.id.clone();
        let mut route = model.routes[0].clone();
        route.id = StableId::parse("a-track-main").expect("route ID");
        let RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        let edits = [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];
        let mut rig = Rig::boot(&model, options(), None);
        let ids = |host: &AudioWorkletEngineHost| {
            host.live_control_tracks()
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        };
        let binding_tracks = |host: &AudioWorkletEngineHost| {
            (0..host.observation_binding_count() as u32)
                .map(|index| {
                    host.observation_binding(index)
                        .expect("a binding")
                        .address
                        .track_index
                })
                .collect::<Vec<_>>()
        };
        rig.step();
        assert_eq!(ids(&rig.host), ["track"]);
        assert_eq!(binding_tracks(&rig.host), [0]);
        assert_eq!(rig.host.session_source_count(), 1);

        assert!(matches!(
            rig.apply(&edits, 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        // Committed, not adopted: the source table is the committed model's and the catalog is
        // still the plan that renders.
        assert_eq!(rig.host.session_source_count(), 2);
        assert_eq!(rig.host.session_source_id(0), Some("a-source"));
        assert_eq!(rig.host.session_source_id(1), Some("fixture-source"));
        assert_eq!(ids(&rig.host), ["track"]);
        assert_eq!(binding_tracks(&rig.host), [0]);
        // An observation record is admitted against the successor, as a live value is: index 1
        // exists only in the successor's tables (the old track's compressor, shifted by the added
        // track), and nothing is armed on the plan that renders.
        assert_eq!(
            observe(&mut rig.host, 1, RACK_INSERTS, 0, 1, 2, true),
            RESULT_OK,
            "an observation record between the commit and the adoption block is admitted"
        );
        assert_eq!(rig.host.command_report().admitted, 1);
        assert_eq!(
            rig.host.observation_armed_taps(),
            0,
            "the plan that renders is not armed by a record addressed to its successor"
        );
        assert_eq!(
            observe(&mut rig.host, 1, RACK_INSERTS, 0, 1, 2, true),
            RESULT_BACKPRESSURE,
            "the record is queued on the successor: its one-slot queue is full until its block"
        );

        rig.step();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(ids(&rig.host), ["a-track", "track"]);
        assert_eq!(
            binding_tracks(&rig.host),
            [0, 1],
            "both compressors are bound, the old one at its shifted index"
        );
        assert_eq!(
            rig.host.observation_armed_taps(),
            1,
            "the record took effect on the successor: the old track's compressor is armed"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
        for _ in 0..4 {
            rig.step();
        }
        let binding = rig.host.observation_binding(1).expect("a binding");
        let mut address = binding.address;
        address.tap_id = 1;
        assert!(
            rig.host.read_observation_addresses(&[address]).is_ok(),
            "the reader the catalog names is the successor's"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
    }

    /// Drive `blocks` more blocks with the lease on, polling after each; the windows folded.
    fn poll_blocks(rig: &mut Rig, blocks: u64) -> u32 {
        let mut windows = 0;
        for _ in 0..blocks {
            rig.step();
            windows += rig.host.poll_meters();
        }
        windows
    }

    /// The meter delivery across a swap (#1381b): a swap that keeps the strip count carries the
    /// predecessor's meters, so the delivery epoch the page sees does not change and the windows
    /// follow each other without a gap, and a swap that changes the count publishes nothing rather
    /// than a frame whose words name other strips (the worklet's views over the frame keep the
    /// boot's size, #1290d).
    ///
    /// Test value: red if the swap block's hook does not pair the meter consumers (the successor's
    /// meters would sit on observers the retiring plan owns and no window would ever fold again),
    /// if the swap restarts the delivery epoch or discards the windows waiting for a poll, or if
    /// the poll reads past the boot-sized frame or the successor's meters after a swap that adds a
    /// strip.
    #[test]
    fn meter_delivery_carries_through_a_swap_and_never_publishes_a_misshaped_frame() {
        let model = gate_model();

        // A swap that keeps the strips: an unused source is added.
        let mut source = model.sources[0].clone();
        source.id = StableId::parse("a-unused").expect("source ID");
        let mut rig = Rig::boot(&model, options(), None);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert!(poll_blocks(&mut rig, 6) > 0, "the boot plan's windows fold");
        let generation = rig.host.meter_header().reserved[0];
        let published = *rig.host.meter_header();
        let strips = rig.host.live_control_tracks().len();
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&[SessionEdit::UpsertSource { source }], 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        let windows = poll_blocks(&mut rig, 8);
        assert_eq!(rig.epoch(), 1);
        assert_eq!(rig.host.live_control_tracks().len(), strips);
        assert!(windows > 0, "the successor's windows fold");
        assert_eq!(
            rig.host.meter_header().reserved[0],
            generation,
            "the swap is not a new delivery epoch: the meters carry"
        );
        assert!(
            rig.host.meter_header().sequence >= published.sequence + u64::from(windows),
            "every window the successor closed was published"
        );
        assert_eq!(
            rig.host.meter_header().reserved[1],
            published.reserved[1],
            "no window was lost to the swap"
        );

        // A swap that adds a strip.
        let mut rig = Rig::boot(&model, options(), None);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert!(poll_blocks(&mut rig, 6) > 0);
        let frame = rig.host.meter_frame().to_vec();
        let header = *rig.host.meter_header();
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        assert_eq!(poll_blocks(&mut rig, 8), 0, "no frame describes ten strips");
        assert_eq!(rig.host.live_control_tracks().len(), 10);
        assert_eq!(rig.host.meter_frame(), &frame[..], "the frame is untouched");
        assert_eq!(rig.host.meter_header().sequence, header.sequence);
    }

    /// An acknowledged subscription survives a swap (#1381b): the tap's armed bit and arm sample
    /// move with its effect to the successor's slot, which a strip added in front shifts, and the
    /// successor's reader is the carried tap's, so its windows continue.
    ///
    /// Test value: red if the swap block's hook does not pair the observation readers or the
    /// table hand-over does not carry the armed bits through the reader-to-slot map (the tap would
    /// read as unarmed and the page's subscription would silently stop after every swap), or if
    /// it maps the readers to the wrong slot after the added strip shifts them.
    #[test]
    fn an_armed_tap_and_its_windows_follow_the_effect_through_a_swap() {
        let mut model =
            parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
                .expect("accepted command fixture");
        model.tracks[0].inserts.effects[0] = observed_compressor_insert();
        let mut rig = Rig::boot(&model, options(), None);
        rig.step();
        assert_eq!(
            observe(&mut rig.host, 0, RACK_INSERTS, 0, 1, 2, true),
            RESULT_OK
        );
        for _ in 0..8 {
            rig.step();
        }
        assert_eq!(rig.host.observation_armed_taps(), 1);
        let address = |track_index| ObservationAddress {
            track_index,
            rack: LiveEffectRack::Inserts,
            effect_index: 0,
            tap_id: 1,
            channels: ObservationReadChannels::Both,
        };
        let before = rig
            .host
            .read_observation_addresses(&[address(0)])
            .expect("a read")[0]
            .window
            .expect("the armed tap published a window")
            .sequence;

        let base = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision
            } if revision == base + 1
        ));
        rig.step();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(
            rig.host.observation_armed_taps(),
            1,
            "the subscription the plan had acknowledged is armed on the successor"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
        for _ in 0..4 {
            rig.step();
        }
        let after = rig
            .host
            .read_observation_addresses(&[address(1)])
            .expect("a read at the shifted strip")[0]
            .window
            .expect("the carried tap kept publishing")
            .sequence;
        assert!(after > before, "the windows continue ({before} to {after})");
    }

    fn meter_loss(rig: &Rig) -> u64 {
        rig.host.ready.as_ref().expect("ready").meter_loss_count
    }

    /// Telemetry is dropped only with a count. A swap that changes the strip count publishes no
    /// frame (the carried meters keep the delivery state, #1381b), so every window it leaves
    /// unpublished is counted when the poll finds it unpublishable and added to the loss count
    /// the next published frame reports, and the swap does not zero what was counted before it.
    ///
    /// Test value: red if the unpublished windows of a strip-count change vanish without a count
    /// (a page would see a silent gap), or if the swap zeroes the loss count.
    #[test]
    fn meter_windows_a_strip_count_change_leaves_unpublished_are_counted() {
        let model = gate_model();
        let mut rig = Rig::boot(&model, options(), None);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert!(poll_blocks(&mut rig, 6) > 0);
        // Windows rendered and not polled when the swap comes, and a loss counted before it.
        for _ in 0..4 {
            rig.step();
        }
        let waiting = rig.host.ready.as_ref().expect("ready").master_count as u64;
        assert!(waiting > 0, "the render side closed windows nobody polled");
        let ready = rig.host.ready.as_mut().expect("ready");
        ready.meter_loss_count = 5;
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        rig.step();
        assert_eq!(rig.epoch(), 1);
        let after_swap = meter_loss(&rig);
        assert_eq!(
            after_swap, 5,
            "the swap keeps the count; the {waiting} windows waiting are counted by the poll"
        );

        // Every window closed afterwards is polled, found unpublishable and counted.
        let windows_before = rig.host.ready.as_ref().expect("ready").master_count as u64;
        let mut counted = after_swap;
        for _ in 0..8 {
            rig.step();
            let closed = rig.host.ready.as_ref().expect("ready").master_count as u64;
            assert_eq!(rig.host.poll_meters(), 0);
            let ready = rig.host.ready.as_ref().expect("ready");
            assert_eq!(
                ready.master_count, 0,
                "the poll drains what it cannot publish"
            );
            assert_eq!(meter_loss(&rig), counted + closed);
            counted += closed;
        }
        assert!(
            counted > after_swap + windows_before,
            "windows were closed and counted"
        );
    }

    /// Run the production control entry on a record the preparer cannot leave its tables in: the
    /// rebuild is refused before the commit, so no revision, plan or counter moves and no
    /// acknowledgement is dropped.
    ///
    /// Test value: red if a preparation that cannot hand its plan's tables to the host still
    /// commits (the client would be told the command failed after the revision advanced, and the
    /// host would swap plans without tables).
    #[test]
    fn a_rebuild_whose_tables_cannot_reach_the_host_is_refused_before_the_commit() {
        let refused = assert_refusal_changes_nothing(options(), |rig, model| {
            let record = Arc::clone(&rig.host.ready.as_ref().expect("ready").record);
            let poisoner = std::thread::spawn(move || {
                let _guard = record.lock().expect("the record is free");
                panic!("poison the record");
            });
            assert!(poisoner.join().is_err());
            let error = rig
                .host
                .structural_command_for_test(&frame(
                    &add_muted_track(model, ADDED),
                    1,
                    rig.host.committed_revision(),
                ))
                .expect_err("the rebuild is refused");
            let control_plane::CommandError::CompileRejected(failure) = &error else {
                panic!("a typed refusal, not {error:?}")
            };
            String::from_utf8_lossy(&failure.diagnostic()).into_owned()
        });
        assert!(refused.contains("web.apply.shape_unavailable"), "{refused}");
    }
}
