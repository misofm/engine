use core::mem::{offset_of, size_of, size_of_val};

use builtins::MeterSnapshot;
use host_core::{
    SpectrumCaptureCollectionRequest, SpectrumCaptureRequest, SpectrumChannels, SpectrumTarget,
};
// #1582 D3: the strip word's conversion and fold moved into the core under these names.
use host_core::meters::{
    reduction_db as observed_decibels, strip_reduction_db as strip_reduction_decibels,
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

/// The staging a boot allocates holds any ID and any source the grammar allows, whatever the boot
/// session declared, and an ID that does not fit is a typed error, never a panic (a panic inside
/// a render-locked export is a trap).
///
/// Test value: red if the projection goes back to sizing from the boot session (a rebuild that adds
/// a longer ID or a wider source would then find no room), or if `copy_id_into_staging` indexes
/// past its buffer. A boot of a short, mono session never meets either; only a rebuild does.
#[test]
fn staging_holds_the_grammars_longest_id_and_widest_source_and_refuses_an_overlong_id() {
    let projection =
        project_buffers(1, 48_000, 128, DEFAULT_CONTROL_FRAME_BYTES).expect("bridge projection");
    let (mut buffers, _) = allocate_buffers(projection).expect("staging");
    assert_eq!(buffers.source_id.len(), session::STABLE_ID_MAXIMUM_BYTES);
    assert_eq!(
        buffers.source_pcm.len(),
        session::SOURCE_CHANNELS_LAUNCH_MAXIMUM as usize * 128
    );
    assert_eq!(
        buffers.plane_references.len(),
        session::SOURCE_CHANNELS_LAUNCH_MAXIMUM as usize
    );
    let longest = [b'x'; session::STABLE_ID_MAXIMUM_BYTES];
    assert_eq!(
        AudioWorkletEngineHost::copy_id_into_staging(Some(&mut buffers), &longest),
        Ok(session::STABLE_ID_MAXIMUM_BYTES as u32)
    );
    assert_eq!(&buffers.source_id[..], &longest[..]);
    assert_eq!(
        AudioWorkletEngineHost::copy_id_into_staging(
            Some(&mut buffers),
            &[b'x'; session::STABLE_ID_MAXIMUM_BYTES + 1]
        ),
        Err(IdStagingError::TooLong)
    );
    assert_eq!(
        AudioWorkletEngineHost::copy_id_into_staging(None, b"a"),
        Err(IdStagingError::NoStaging)
    );
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
        crate::resolved_control_frame_bytes(&options).expect("a valid frame word"),
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
    const FOOTER: &str = r#"],"offset":{"left_db":0.0,"right_db":0.0}}]"#;

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
            BUFFER_METER_FRAME
        ],
        [2, 3, 4, 5, 7]
    );
    assert_eq!(DEFAULT_METER_BLOCKS, 12);
    assert_eq!(offset_of!(WebBootOptions, struct_size), 0);
    assert_eq!(offset_of!(WebBootOptions, require_quantum_frames), 12);
    assert_eq!(offset_of!(WebBootOptions, source_ring_frames), 16);
    assert_eq!(offset_of!(WebBootOptions, maximum_memory_bytes), 24);
    assert_eq!(offset_of!(WebBootOptions, maximum_control_frame_bytes), 32);
    assert_eq!(offset_of!(WebBootOptions, live_control_meter_blocks), 40);
    // Issue #143 D3/D6 carved the configuration's remaining two reserved words; #1590 D5 and
    // #1689 retired them again (`reserved3`, `reserved2`), and every offset is where it was.
    assert_eq!(offset_of!(WebBootOptions, reserved3), 48);
    assert_eq!(offset_of!(WebBootOptions, reserved2), 56);
    // The meter header is a new fixed structure, not a change to an existing one.
    // Issue #1209 D2 appended `submix_count` and its pad: 64 bytes became 72, nothing moved.
    // Issue #1290d1 turned the pad into `retired` and appended the frame and successor pointers:
    // 72 bytes became 80, nothing moved. Issue #1562b appended `structure_epoch`: 80 became 88.
    assert_eq!(size_of::<WebMeterHeader>(), 88);
    assert_eq!(METER_HEADER_BYTES, 88);
    assert_eq!(offset_of!(WebMeterHeader, track_count), 8);
    assert_eq!(offset_of!(WebMeterHeader, windows), 12);
    assert_eq!(offset_of!(WebMeterHeader, first_sample), 16);
    assert_eq!(offset_of!(WebMeterHeader, end_sample), 24);
    assert_eq!(offset_of!(WebMeterHeader, sequence), 32);
    assert_eq!(offset_of!(WebMeterHeader, master_track_plus_one), 40);
    assert_eq!(offset_of!(WebMeterHeader, master_gr_present), 44);
    assert_eq!(offset_of!(WebMeterHeader, reserved), 48);
    assert_eq!(offset_of!(WebMeterHeader, submix_count), 64);
    assert_eq!(offset_of!(WebMeterHeader, retired), 68);
    assert_eq!(offset_of!(WebMeterHeader, frame_pointer), 72);
    assert_eq!(offset_of!(WebMeterHeader, successor_pointer), 76);
    assert_eq!(offset_of!(WebMeterHeader, structure_epoch), 80);
    assert_eq!(offset_of!(WebStatus, state), 8);
    assert_eq!(offset_of!(WebStatus, blocking_rebuilds), 28);
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

/// #1697 gate 2 (#1386 gate 8, browser half): `miso_engine_web_v1_replace_document_ptr` stages a
/// replacement document of more than 64 KiB, and one of exactly `MAXIMUM_DOCUMENT_BYTES`, and the
/// `SESSION_REPLACE` frame that names it commits through `miso_engine_web_v1_control_command`;
/// one byte more, or none, stages nothing. After every command the staging holds no byte, and the
/// first command after boot frees the boot's copy.
///
/// Test value: red if the staging's bound is off by one either way, if a refused staging leaves
/// an earlier document for the next command, or if the command (or the boot) keeps the document's
/// bytes after the reply.
#[test]
fn a_replace_document_up_to_the_boot_bound_commits_and_is_freed_after_its_command() {
    use protocol::{
        CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
        EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionReplaceRequest,
        SessionRevision, TypedCommandFrame,
    };
    use sha2::{Digest, Sha256};
    let frame = |request: u64, payload: CommandPayload<'_>, expected: ExpectedRevision| {
        let mut bytes = vec![0_u8; 4_096];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: expected,
                    payload,
                },
                &mut bytes,
            )
            .expect("a command frame");
        bytes.truncate(length);
        bytes
    };
    let maximum = MAXIMUM_DOCUMENT_BYTES as usize;
    let booted = one_track_session(128);
    let handle = crate::ffi::test_boot(booted.as_bytes(), boot_options(128));
    assert_ne!(handle, 0);
    assert!(
        crate::ffi::test_document_staging().2 >= booted.len(),
        "the boot keeps its copy until the first command"
    );
    let (result, _) = ffi::native_staging::control_command(
        handle,
        &frame(1, CommandPayload::CapabilitiesGet, ExpectedRevision::Any),
    )
    .expect("a live handle");
    assert_eq!(result, RESULT_OK);
    assert_eq!(
        crate::ffi::test_document_staging(),
        (false, 0, 0),
        "the first command frees the boot's copy"
    );

    let mut model = parse_session_json(&booted).expect("the booted document");
    for (request, (size, db)) in [((64 << 10) + 1, -3.0), (maximum, -6.0)]
        .into_iter()
        .enumerate()
    {
        let request = request as u64 + 2;
        model.tracks[0].fader.left_db = db;
        let mut document = canonical_session_json(&model).expect("a valid document");
        assert!(document.len() < size);
        // Whitespace after the value pads the document to `size` without changing it.
        document.extend(core::iter::repeat_n(' ', size - document.len()));
        assert!(crate::ffi::test_replace_document(
            handle,
            document.as_bytes()
        ));
        assert_eq!(crate::ffi::test_document_staging(), (true, size, size));
        let revision = crate::ffi::miso_engine_web_v1_control_revision(handle);
        let replace = frame(
            request,
            CommandPayload::SessionReplace(SessionReplaceRequest {
                document_bytes: size as u32,
                document_sha256: Sha256::digest(document.as_bytes()).into(),
            }),
            ExpectedRevision::Exact(SessionRevision(revision)),
        );
        let (result, reply) =
            ffi::native_staging::control_command(handle, &replace).expect("a live handle");
        assert_eq!(result, RESULT_OK, "{}", String::from_utf8_lossy(&reply));
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SessionReplaced(applied),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("a typed reply")
        else {
            panic!("{size} bytes: a committed replace")
        };
        assert_eq!(applied.edit_path, EditPath::Live, "{size} bytes");
        assert_eq!(
            crate::ffi::miso_engine_web_v1_control_revision(handle),
            revision + 1
        );
        assert_eq!(
            crate::ffi::test_document_staging(),
            (false, 0, 0),
            "{size} bytes: the command frees the document"
        );
    }

    // A staged document and a command with a wrong handle: the refusal frees it too.
    assert!(crate::ffi::test_replace_document(handle, b"{}"));
    assert_eq!(
        crate::ffi::miso_engine_web_v1_control_command(handle.wrapping_add(1), 2),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(crate::ffi::test_document_staging(), (false, 0, 0));
    // A staged document, then a refused staging: nothing is left for the next command.
    assert!(crate::ffi::test_replace_document(handle, b"{}"));
    for refused in [maximum + 1, 0] {
        assert!(
            !crate::ffi::test_replace_document(handle, &vec![b' '; refused]),
            "{refused} bytes"
        );
        assert_eq!(
            crate::ffi::test_document_staging(),
            (false, 0, 0),
            "{refused} bytes stage nothing"
        );
    }
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
        // Source PCM staging holds the most channels a source can declare, not the boot session's.
        assert_eq!(
            host.source_pcm_mut().expect("PCM").len(),
            session::SOURCE_CHANNELS_LAUNCH_MAXIMUM as usize * quantum as usize
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
    // The 16 KiB frame floor keeps the retained rows under this parse projection (#2191).
    let accepted = WebBootOptions {
        maximum_memory_bytes: parse_projection,
        maximum_control_frame_bytes: u64::from(MINIMUM_CONTROL_FRAME_BYTES),
        ..boot_options(128)
    };
    AudioWorkletEngineHost::boot(document.as_bytes(), accepted).expect("inclusive budget");
    let refused = WebBootOptions {
        maximum_memory_bytes: parse_projection - 1,
        ..accepted
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
        (
            WebBootOptions {
                maximum_memory_bytes: u64::from(WASM_MAXIMUM_MEMORY_PAGES) * WASM_PAGE_BYTES + 1,
                ..boot_options(128)
            },
            b"web.boot.memory_maximum\t$\n".as_slice(),
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
    // #1380 D2: the module's maximum memory itself is an accepted budget.
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
        WebBootOptions {
            maximum_memory_bytes: u64::from(WASM_MAXIMUM_MEMORY_PAGES) * WASM_PAGE_BYTES,
            ..boot_options(128)
        },
    )
    .expect("a budget equal to the module's maximum memory boots");
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

/// The bridge rows are charged exactly, and observation adds no bridge staging (#1589c deleted the
/// observation-command staging and its decode array: an `Observe` edit rides the control frame
/// staging every boot has; since #1590 D5 every plan binds every declared tap).
///
/// Test value: red if the browser keeps the retired mirrors' bytes (input-filter shadows, ramp
/// table, VCA and strip-value mirrors, the command staging) in the bridge rows, or if the plan's
/// observation index (#1586 D3) is not charged.
#[test]
fn bridge_resource_rows_are_exact() {
    // One console EQ, which declares no tap: its instance and its observation index entry are
    // charged to the bridge rows (#1586a) while the engine's observation row stays zero.
    let document = one_track_session(128);
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
    assert_eq!(shape.effect_count, 1, "one console EQ");

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
    let projection = project_buffers(
        document.len() as u32,
        shape.sample_rate_hz,
        shape.quantum_frames,
        DEFAULT_CONTROL_FRAME_BYTES,
    )
    .expect("bridge projection");
    let (name, options) = ("boot", boot_options(128));
    let source_ring_frames = if options.source_ring_frames == 0 {
        default_source_ring_frames(shape.sample_rate_hz, shape.quantum_frames)
    } else {
        options.source_ring_frames
    };
    let caps = prepare_caps(&compiled, options, source_ring_frames, u64::MAX);
    let live_controls =
        live_control_request(options, shape.quantum_frames).expect("live-control request");
    // The browser's own entry: a boot word of `0` binds the core's strip meters at the default
    // window (#1581 D2).
    let (engine, _) = host_core::prepare_host_runtime_with_live_lanes(
        &compiled,
        &caps,
        &live_controls,
        HostLiveLanes {
            effects: true,
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
    assert!(
        engine.report.builtin_meter_payload_bytes > 0,
        "{name}: the strip meters"
    );
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
    let ready = host.test_render();
    assert_eq!(
        ready.shape.effect_instances.len(),
        1,
        "{name}: one effect slot"
    );
    assert_eq!(
        ready.meters.tables().meter_count(),
        ready.shape.tracks.len() + ready.shape.submixes.len(),
        "{name}: one meter per strip"
    );
    assert_eq!(host.resources().observation_retained_bytes, 0);

    // The control plane's own retained row (its catalog, producers, protocol queues, replay
    // cache, the plan's inventory and the superseded count) is charged once, and holds the
    // rows an independent preparation of the same session reports.
    let control_retained = host
        .test_control()
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
    // The published meter frame is part of the plan's tables, charged with them (#1290d1):
    // `3(T + S) + 3` words, held whether or not metering is on.
    let model = compiled.normalized_model();
    let frame_bytes = (3 * (model.tracks.len() + model.submixes.len()) as u64 + 3) * 4;
    // The strip meters' delivery rows, bound with the boot word `0` too (#1581 D2): per strip
    // a pending snapshot, a remaining count and a carried-from handle. The master windows are
    // the control plane's output meter, in its retained row (#1581 D5).
    let strips = (model.tracks.len() + model.submixes.len()) as u64;
    let meter_delivery_bytes = strips
        * (size_of::<Option<MeterSnapshot>>() + size_of::<usize>() + size_of::<u64>()) as u64;
    // #1582 D2: the core meter frame's per-strip tables, a peak pair, a gain-reduction word
    // and its presence.
    let meter_delivery_bytes = meter_delivery_bytes
        + strips * (size_of::<[f32; 2]>() + size_of::<f32>() + size_of::<bool>()) as u64;
    let frame_bytes = frame_bytes + meter_delivery_bytes;
    // The one EQ's dense instance table, its observation-strip word and the IDs it copies. Its
    // observation index entry is the epoch's (#1589 R2), in the control plane's row.
    let effect_ids = (model.tracks[0].id.as_str().len()
        + model.console.pre_insert[0].slot.as_str().len()) as u64;
    let instance_bytes =
        (size_of::<Option<EffectInstance>>() + size_of::<u32>()) as u64 + effect_ids;
    let frame_bytes = frame_bytes + instance_bytes;
    // #1332 S5: the boot plan's own words, and what the two halves share: the successor record,
    // the lifecycle word (an `Arc`'s two counts and the word, padded to a count's alignment) and
    // the hand-back ring, whose bytes the ring itself walks.
    let words = engine::realtime::published_words_retained_bytes::<2>() as u64;
    let lifecycle = (3 * size_of::<usize>()) as u64;
    let ring = host.test_control().retired_shapes.retained_bytes();
    let frame_bytes = frame_bytes + 2 * words + lifecycle + ring;
    let expected_bridge_metadata = projection
        .report
        .bridge_metadata_bytes
        .checked_add(frame_bytes)
        .and_then(|bytes| bytes.checked_add(control_retained))
        .and_then(|bytes| bytes.checked_add(session_model_bytes))
        .expect("bridge metadata arithmetic");
    let expected_bridge_retained = projection
        .report
        .bridge_retained_bytes
        .checked_add(frame_bytes)
        .and_then(|bytes| bytes.checked_add(control_retained))
        .and_then(|bytes| bytes.checked_add(session_model_bytes))
        .expect("bridge retained arithmetic");
    assert_eq!(
        host.resources().bridge_metadata_bytes,
        expected_bridge_metadata
    );
    assert_eq!(
        host.resources().bridge_retained_bytes,
        expected_bridge_retained
    );

    // The largest assertion includes the independent projection and then folds the engine's
    // own largest allocation into the named maximum.
    let bridge_largest = projection
        .report
        .largest_named_allocation_bytes
        .max(control_table_bytes(shape.source_count as usize).expect("table largest"))
        .max(source_id_arena_bytes(source_id_bytes as usize).expect("ID largest"))
        .max(compiled.resource_estimate().single_allocation_bytes);
    assert_eq!(
        host.resources().largest_bridge_allocation_bytes,
        bridge_largest
    );
    assert_eq!(
        host.resources().largest_named_allocation_bytes,
        bridge_largest.max(engine.report.largest_engine_allocation_bytes)
    );
}

/// The exact retained aggregate is one budget: it boots at exactly its value and is refused one
/// byte below, at the 16 KiB frame floor and at the 256 KiB default frame alike. Issue #2191 gate
/// 2: between the two frames the aggregate grows by at least the rows the frame scales (the
/// page's staging, the core's request and response scratch, the prepared token's two frames and
/// the replay cache of `max(64 KiB, 2 * frame)`), and the bridge and control rows carry all of it.
///
/// Test value: red if the budget charges independent caps instead of one aggregate, if a
/// frame-scaled buffer is allocated at the booted frame but charged at a compiled one (the
/// aggregate grows by less than the rows), or if the replay cache does not follow the frame.
#[test]
fn exact_retained_total_is_checked_as_one_budget_not_independent_caps() {
    let document = one_track_session(128);
    let source_ring_frames = 1 << 20;
    let mut charged = Vec::new();
    for frame in [MINIMUM_CONTROL_FRAME_BYTES, DEFAULT_CONTROL_FRAME_BYTES] {
        let options = WebBootOptions {
            source_ring_frames,
            maximum_control_frame_bytes: u64::from(frame),
            ..boot_options(128)
        };
        let baseline =
            AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("baseline boot");
        let ready = baseline.test_render();
        let dense_effect_table_bytes = size_of_val(ready.shape.effect_instances.as_ref()) as u64;
        assert_eq!(
            ready.shape.effect_instances.len(),
            1,
            "one-track fixture has one effect"
        );
        assert!(
            baseline.resources().bridge_retained_bytes >= dense_effect_table_bytes,
            "the bridge rows charge the dense effect table"
        );
        let exact = exact_retained_report_total(baseline.resources());
        let control_retained = baseline
            .test_control()
            .queries
            .resources()
            .control_retained_bytes;
        charged.push((frame, exact, control_retained));
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
    let [(small, small_exact, small_control), (large, large_exact, large_control)] = charged[..]
    else {
        panic!("two frames")
    };
    let growth = u64::from(large - small);
    let replay_growth =
        control_replay_bytes(u64::from(large)) - control_replay_bytes(u64::from(small));
    // The page's staging is the bridge's row; everything else the frame scales is the control
    // plane's, and nothing else in the session moves.
    assert_eq!(
        large_exact - small_exact,
        growth + (large_control - small_control),
        "the staging and the control rows carry the whole growth"
    );
    assert!(
        large_control - small_control >= 4 * growth + replay_growth,
        "the control rows grow by the scratch, the token and the replay cache"
    );
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
    // three shipped shapes pin that drift, each with its own bound in thousandths of the
    // projection: the measured `gap / projection` plus about 8% headroom for harmless
    // allocator/layout movement, so any material projector drift is an explicit review and
    // re-pin. Dropping a projected retained row cannot hide behind the deliberately broad rollback
    // allowance. Re-pinned by #1590, which deleted the 55,527-byte observation staging from the
    // projection and the exact row alike (the gaps did not move, the ratios rose):
    // identity 628,275 / 57,268 = 10.97, nine-track 954,987 / 86,248 = 11.07, console
    // 3,846,216 / 640,894 = 6.00 (its bound did not rise: the shared multiplier was 7).
    // Issue #2191: the pins are at the 16 KiB frame they were measured at. The control plane's
    // frame-scaled rows (about 15 bytes per frame byte) are charged by the exact check, not the
    // pre-prepare projection, so at the 256 KiB default they would dominate the gap (14.9, 14.6
    // and 8.8 times the projection) and hide a dropped row; the frame's own charge is
    // `exact_retained_total_is_checked_as_one_budget_not_independent_caps`.
    let floor = WebBootOptions {
        maximum_control_frame_bytes: u64::from(MINIMUM_CONTROL_FRAME_BYTES),
        ..boot_options(128)
    };
    let representatives = [
        (
            "identity-one-track",
            11_850_u64,
            one_track_session(128),
            WebBootOptions {
                source_ring_frames: 128,
                ..floor
            },
        ),
        (
            "parametric-eq-nine-track",
            11_960_u64,
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json").to_owned(),
            WebBootOptions {
                source_ring_frames: 512,
                ..floor
            },
        ),
        (
            "console-sixty-four-track",
            6_480_u64,
            include_str!("../../../fixtures/session/v1/console-sixty-four-track.json").to_owned(),
            WebBootOptions {
                source_ring_frames: 512,
                live_control_meter_blocks: u64::from(DEFAULT_METER_BLOCKS),
                ..floor
            },
        ),
    ];

    for (name, bound_permille, document, options) in representatives {
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
            .checked_mul(bound_permille)
            .expect("bounded representative projection")
            / 1_000;
        assert!(
            gap <= maximum_gap,
            "{name}: exact/projection drift gap {gap} exceeds documented bound {maximum_gap} (projection {projection}, exact {exact})"
        );
    }
}

/// The exports of an identity session at one quantum, driven as the page's message handler does:
/// the source ID staged, then `seek_at`, `source_submit`, `render` and `seek_report` as raw calls.
struct SeekExports {
    handle: u32,
    quantum: u32,
}

impl SeekExports {
    const SOURCE: &'static [u8] = b"fixture-source";

    fn boot() -> Self {
        let quantum = 128;
        let document = identity_session(quantum, 480_000);
        let options = WebBootOptions {
            source_ring_frames: 512,
            ..boot_options(quantum)
        };
        let handle = crate::ffi::test_boot(document.as_bytes(), options);
        assert_ne!(handle, 0);
        let rig = Self { handle, quantum };
        rig.stage(Self::SOURCE);
        assert_eq!(crate::ffi::test_fill_source_pcm(handle, 0.25), RESULT_OK);
        rig
    }

    fn stage(&self, id: &[u8]) {
        assert_eq!(
            crate::ffi::test_copy_staging(self.handle, BUFFER_SOURCE_ID, id),
            RESULT_OK
        );
    }

    fn seek_at(&self, generation: u64, frame: u64, anchor: u64) -> u32 {
        miso_engine_web_v1_source_seek_at(
            self.handle,
            Self::SOURCE.len() as u32,
            generation,
            frame,
            anchor,
        )
    }

    fn submit(&self, generation: u64, start_frame: u64) {
        assert_eq!(
            miso_engine_web_v1_source_submit(
                self.handle,
                Self::SOURCE.len() as u32,
                generation,
                start_frame,
                2,
                self.quantum,
                0,
            ),
            RESULT_OK
        );
    }

    fn render(&self) -> Vec<f32> {
        assert_eq!(
            miso_engine_web_v1_render(self.handle, self.quantum),
            RESULT_OK
        );
        crate::ffi::native_staging::output_pcm(self.handle).expect("output")
    }

    fn report(&self) -> (u32, WebSeekReport) {
        let code = miso_engine_web_v1_source_seek_report(self.handle, Self::SOURCE.len() as u32);
        (
            code,
            crate::ffi::native_staging::seek_report(self.handle).expect("record"),
        )
    }
}

/// Issue #1387: `source_fill` reads the producer's ring occupancy through the raw export: the
/// queued chunks in the high word and the chunks a submit accepts now in the low word, for the
/// 4-quantum ring of this rig; render lowers the queue; a full ring has no room and a submit then
/// is refused; an unknown source and a wrong handle are refused with `u64::MAX`.
///
/// Test value: red if the export reports the consumer's count or the ring's capacity instead of
/// the producer's live occupancy, swaps the words, or answers a refused read with a count.
#[test]
fn source_fill_reads_the_producers_ring_occupancy() {
    let rig = SeekExports::boot();
    let fill = || miso_engine_web_v1_source_fill(rig.handle, SeekExports::SOURCE.len() as u32);
    let words = |value: u64| (value >> 32, value & 0xffff_ffff);
    assert_eq!(
        words(fill()),
        (0, 4),
        "an empty ring accepts its four quanta"
    );
    rig.submit(1, 0);
    rig.submit(1, 128);
    rig.submit(1, 256);
    assert_eq!(words(fill()), (3, 1));
    rig.render();
    let (queued, _) = words(fill());
    assert_eq!(queued, 2, "render took one chunk");
    rig.submit(1, 384);
    rig.submit(1, 512);
    let (queued, room) = words(fill());
    assert_eq!(room, 0, "a full ring accepts nothing");
    assert_eq!(queued, 4);
    assert_eq!(
        miso_engine_web_v1_source_submit(
            rig.handle,
            SeekExports::SOURCE.len() as u32,
            1,
            640,
            2,
            128,
            0
        ),
        RESULT_BACKPRESSURE,
        "the export's zero room is the submit's backpressure"
    );
    rig.stage(b"no-such-source");
    assert_eq!(miso_engine_web_v1_source_fill(rig.handle, 14), u64::MAX);
    rig.stage(SeekExports::SOURCE);
    assert_eq!(
        miso_engine_web_v1_source_fill(0, SeekExports::SOURCE.len() as u32),
        u64::MAX
    );
    assert_eq!(
        miso_engine_web_v1_source_fill(rig.handle, u32::MAX),
        u64::MAX
    );
}

/// Issue #1293 gates 3 and 4 through the raw exports: an anchored seek two quanta ahead is
/// accepted (`RESULT_OK`, not the `prepare_source_seek` path's `RESULT_INTERNAL` for a held seek),
/// the source plays silence until the anchor block and the sought frame from it on, an unaligned
/// anchor is invalid, and the 64-byte record reads the generation, the anchor as `first_sample`,
/// and the sought frame.
///
/// Test value: red if `source_seek_at` is wired through `plan.prepare_source_seek` (the D4 trap:
/// the future anchor would be `RESULT_INTERNAL`), if it queues a plain seek (sound from the next
/// block), if the report export reads another word order or a stale record, or if either seek
/// export maps an unknown source to a result other than `RESULT_INVALID_ARGUMENT`. The session has
/// one source, so this test cannot show which source's ring the report reads.
#[test]
fn the_anchored_seek_exports_play_at_the_anchor_and_report_where_it_landed() {
    let rig = SeekExports::boot();
    assert_eq!(SEEK_REPORT_BYTES, 64);
    // Before any seek: a zero-filled record, then generation 1's.
    assert_eq!(
        crate::ffi::native_staging::seek_report(rig.handle),
        Some(WebSeekReport::default())
    );

    // An anchor that is not a quantum multiple, and an unknown source, are invalid.
    assert_eq!(rig.seek_at(2, 10_000, 130), RESULT_INVALID_ARGUMENT);
    rig.stage(b"no-such-source");
    assert_eq!(
        miso_engine_web_v1_source_seek_at(rig.handle, 14, 2, 10_000, 256),
        RESULT_INVALID_ARGUMENT
    );
    // The plain seek export refuses an unknown source the same way.
    assert_eq!(
        miso_engine_web_v1_source_seek(rig.handle, 14, 2, 10_000),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        miso_engine_web_v1_source_seek_report(rig.handle, 14),
        RESULT_INVALID_ARGUMENT
    );
    rig.stage(SeekExports::SOURCE);

    // Two quanta ahead: accepted, with the new generation's PCM queued behind the anchor.
    assert_eq!(rig.seek_at(2, 10_000, 256), RESULT_OK);
    rig.submit(2, 10_000);
    rig.submit(2, 10_128);
    for block in 0..2 {
        let output = rig.render();
        assert!(
            output.iter().all(|sample| *sample == 0.0),
            "block {block}: silence until the anchor"
        );
    }
    let output = rig.render();
    assert!(
        output.iter().all(|sample| *sample == 0.25),
        "the anchor block plays the sought frame"
    );
    let (code, record) = rig.report();
    assert_eq!(code, RESULT_OK);
    assert_eq!(
        (
            record.struct_size,
            record.reserved0,
            record.generation,
            record.first_sample,
            record.source_frame,
            record.reserved
        ),
        (SEEK_REPORT_BYTES, 0, 2, 256, 10_000, [0; 2])
    );
    assert_eq!(
        record.cumulative_underrun_frames, 0,
        "no frame was missing in-region"
    );
    assert_eq!(
        record.cumulative_held_frames, 256,
        "two quanta were held until the anchor"
    );
}

/// Issue #1293 gate 5: on a host that is not ready (no control half), `source_seek_at` and
/// `source_seek_report` refuse with `RESULT_REFUSED_LIFECYCLE`, the report record keeps its last
/// words, and a wrong handle or an ID count past the staging is `RESULT_INVALID_ARGUMENT`.
///
/// Test value: red if either export runs on a host with no control half (it would reach for
/// control state the instance does not hold) or overwrites the record on a refusal, if the
/// pointer export names anything but the record while ready or anything at all once not ready,
/// or if the report export records a staged-ID refusal as the last result (it records none) or
/// refuses it without the core's `source.id.invalid` text (#1706).
#[test]
fn the_seek_exports_refuse_a_host_with_no_control_half_and_change_nothing() {
    let rig = SeekExports::boot();
    assert_eq!(rig.seek_at(2, 10_000, 256), RESULT_OK);
    rig.submit(2, 10_000);
    let _ = rig.render();
    let (code, before) = rig.report();
    assert_eq!(code, RESULT_OK);
    // A ready host's pointer export names the record.
    let (pointer, record) = crate::ffi::test_seek_report_addresses(rig.handle);
    assert!(record != 0 && pointer == record);

    // Argument checks on a live host.
    assert_eq!(
        miso_engine_web_v1_source_seek_at(0, 14, 3, 0, 512),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        miso_engine_web_v1_source_seek_report(0, 14),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        miso_engine_web_v1_source_seek_at(rig.handle, u32::MAX, 3, 0, 512),
        RESULT_INVALID_ARGUMENT
    );
    // The anchored seek records its refusal; the report leaves the last result alone.
    assert_eq!(
        crate::ffi::test_status(rig.handle)
            .expect("status")
            .last_result,
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(rig.seek_at(3, 0, 1024), RESULT_OK);
    assert_eq!(
        miso_engine_web_v1_source_seek_report(rig.handle, u32::MAX),
        RESULT_INVALID_ARGUMENT
    );
    // #1706 D4: an ID count past the staging (`STABLE_ID_MAXIMUM_BYTES`) is the core's
    // malformed-ID refusal, text included, on the export the page calls.
    assert_eq!(
        miso_engine_web_v1_source_seek_report(
            rig.handle,
            session::STABLE_ID_MAXIMUM_BYTES as u32 + 1
        ),
        RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        crate::ffi::test_control_response(rig.handle),
        b"source.id.invalid"
    );
    assert_eq!(
        crate::ffi::test_status(rig.handle)
            .expect("status")
            .last_result,
        RESULT_OK
    );
    assert_eq!(
        crate::ffi::native_staging::seek_report(rig.handle),
        Some(before)
    );

    // An output-shape mismatch makes the host sticky-failed: no control half to run on.
    assert_eq!(
        miso_engine_web_v1_render(rig.handle, 0),
        RESULT_REPREPARE_REQUIRED
    );
    assert_eq!(
        crate::ffi::test_status(rig.handle).expect("status").state,
        STATE_FAILED
    );
    assert_eq!(rig.seek_at(3, 0, 1024), RESULT_REFUSED_LIFECYCLE);
    assert_eq!(rig.report().0, RESULT_REFUSED_LIFECYCLE);
    assert_eq!(
        crate::ffi::native_staging::seek_report(rig.handle),
        Some(before),
        "a refusal leaves the record alone"
    );
    // A host that is not ready has no record address to give.
    let (pointer, record) = crate::ffi::test_seek_report_addresses(rig.handle);
    assert!(record != 0 && pointer == 0);
    assert_eq!(miso_engine_web_v1_source_seek_report_ptr(0), 0);
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
/// Red mutation: restore `self.ready = None;` as the first line of `RenderHalf::fail` → the
/// `host.render.ready.is_some()` assertion fails.
#[test]
fn render_failure_retains_ownership_and_silences() {
    let mut host = prepared_host(128);
    host.render
        .buffers
        .as_mut()
        .expect("prepared buffers")
        .output_pcm
        .fill(-1.0);
    host.render.status.next_absolute_sample = u64::MAX;

    assert_eq!(host.render_next(), RESULT_RENDER_REJECTED);
    assert!(
        host.render.ready.is_some() && host.control.is_some(),
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
        host.render.ready.is_some() && host.control.is_some(),
        "the retirement slot survives re-entry"
    );

    assert_eq!(host.dispose(), RESULT_OK);
    assert!(
        host.render.ready.is_none() && host.control.is_none(),
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

/// #106 F3. A full default ring renders the whole stall tolerance with no submission at all.
///
/// The identity session passes source frames to the output unchanged, so the oracle is the
/// submitted ramp itself -- compared as `to_bits`, never as floats, because `==` equates `-0.0`
/// with `+0.0`. Filling until backpressure and then rendering 38 quanta in silence is exactly the
/// 100 ms main-thread stall the default ring exists to hide.
///
/// Red mutation: set `SOURCE_STALL_TOLERANCE_MS = 50` -> the stall is 19 quanta and the ring 38
/// (a 21-quantum stall ring, plus the 16-quantum prime and one quantum, #1406 D2), and the
/// `stall_quanta == 38` assertion fails.
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
            expected_effect: None,
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

/// A `SET_CONTROL` frame that turns strip `strip`'s `master` monitor row (#1683) on, request
/// `request` at revision `revision`: since #1689 the one way to designate the master on the
/// browser, as on the C ABI. A monitor write spends no revision. `master_designation_frame("track",
/// 1, 1)` is committed as `tests/browser-v1/set-control-frames/master.bin`, the frame the raw-Wasm
/// oracle's observation timeline sends after boot.
fn master_designation_frame(strip: &str, request: u64, revision: u64) -> Vec<u8> {
    use protocol::{
        CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
        TypedCommandFrame,
    };
    let edits = [SessionEdit::SetControl {
        expected_effect: None,
        address: controls::ControlAddress {
            owner: controls::Owner::Strip(session::StableId::parse(strip).expect("strip ID")),
            section: controls::Section::Own,
            control: 4,
            lane: session::ParameterChannel::Both,
        },
        value: 1.0,
        ramp_samples: None,
    }];
    let mut bytes = vec![0_u8; 4_096];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    bytes
}

/// The three `Observe` frames of the observation timeline (#1589c), in order: an arm of a tap the
/// compressor does not declare (tap 9), the arm of its gain-reduction tap (tap 1) and that tap's
/// disarm, each with a two-block window, requests 2 to 4 at revision 1 (a monitor edit spends no
/// revision). The disarm names the arm's tag (#2087): the session's first record, tag 1 (the
/// refused arm of tap 9 issues none). Committed as `tests/browser-v1/set-control-frames/observe-{unknown-tap,arm,disarm}.bin`,
/// the frames the raw-Wasm oracle's observation timeline sends.
fn observation_timeline_frames() -> [(&'static str, Vec<u8>); 3] {
    use protocol::{
        CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionRevision,
        TypedCommandFrame,
    };
    let frame = |request: u64, tap_id: u32, armed: bool| {
        let edits = [tagged(
            observe_edit("track", RACK_INSERTS, "comp", tap_id, 2, armed),
            if armed { 0 } else { 1 },
        )];
        let mut bytes = vec![0_u8; 4_096];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(1)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                &mut bytes,
            )
            .expect("an observe frame");
        bytes.truncate(length);
        bytes
    };
    [
        ("observe-unknown-tap", frame(2, 9, true)),
        ("observe-arm", frame(3, 1, true)),
        ("observe-disarm", frame(4, 1, false)),
    ]
}

/// The status word and the diagnostic codes of one response frame.
fn reply_status(response: &[u8]) -> (u16, Vec<String>) {
    use protocol::{DecodeScratch, DecodedTypedResponseFrame, ProtocolCodec};
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a response frame")
    {
        DecodedTypedResponseFrame::Success { header, .. } => (header.status as u16, Vec::new()),
        DecodedTypedResponseFrame::NonOk { header, payload } => (
            header.status as u16,
            payload
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect(),
        ),
    }
}

/// Designate strip `strip` the master through the production control entry (#1689 D6): the
/// header names it at once.
fn designate_master(host: &mut AudioWorkletEngineHost, strip: &str) {
    let frame = master_designation_frame(strip, next_request_id(), host.committed_revision());
    host.control_request_mut(frame.len() as u32)
        .expect("the control staging")[..frame.len()]
        .copy_from_slice(&frame);
    assert_eq!(host.control_command(frame.len() as u32), RESULT_OK);
    let index = host
        .live_control_tracks()
        .iter()
        .position(|id| &**id == strip);
    assert!(
        index.is_none_or(|index| host.meter_header().master_track_plus_one == index as u32 + 1),
        "the header names the designated track"
    );
}

/// The committed frame fixtures are exactly what [`set_control_timeline_frames`],
/// [`master_designation_frame`] and [`observation_timeline_frames`] encode.
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
        std::fs::write(
            format!("{directory}/master.bin"),
            master_designation_frame("track", 1, 1),
        )
        .expect("fixture file");
        for (name, frame) in observation_timeline_frames() {
            std::fs::write(format!("{directory}/{name}.bin"), frame).expect("fixture file");
        }
    }
    for (index, frame) in frames.iter().enumerate() {
        let committed = std::fs::read(format!("{directory}/frame-{}.bin", index + 1))
            .expect("the committed frame fixture");
        assert_eq!(&committed, frame, "frame {}", index + 1);
    }
    let committed =
        std::fs::read(format!("{directory}/master.bin")).expect("the committed master frame");
    assert_eq!(
        committed,
        master_designation_frame("track", 1, 1),
        "master.bin"
    );
    for (name, frame) in observation_timeline_frames() {
        let committed =
            std::fs::read(format!("{directory}/{name}.bin")).expect("the committed observe frame");
        assert_eq!(committed, frame, "{name}.bin");
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

/// The #1669 automation transcript (D2): 96 blocks of 128 frames, one plain session seek to
/// timeline 30,000 under generation 2 before block 40, one chunk per source before every render.
/// It returns each block's output (left plane, then right plane). `direct-oracle.mjs`'s
/// `runAutomationTimeline` replays the same integer PCM formula and chunk schedule.
fn automation_transcript(document: &[u8]) -> Vec<Vec<f32>> {
    const QUANTUM: u32 = 128;
    const RATE: u32 = 48_000;
    const BLOCKS: u64 = 96;
    const SEEK_BLOCK: u64 = 40;
    const SEEK_TIMELINE: u64 = 30_000;
    // `((f*m) mod 65536 - 32768) / 65536`, exact in `f32`, per source and lane.
    const SOURCES: [(&[u8], [u32; 2]); 2] =
        [(b"src-t1", [40_503, 30_011]), (b"src-t2", [52_153, 18_749])];
    let pcm = |frame: u64, multiplier: u32| -> f32 {
        let frame = u32::try_from(frame).expect("frame fits u32");
        let residue = (frame.wrapping_mul(multiplier) & 0xffff) as i32 - 32_768;
        residue as f32 / 65_536.0
    };

    let options = WebBootOptions {
        source_ring_frames: QUANTUM,
        ..boot_options(QUANTUM)
    };
    let mut host = AudioWorkletEngineHost::boot(document, options).unwrap_or_else(|failure| {
        panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic()))
    });
    let mut blocks = Vec::new();
    for block in 0..BLOCKS {
        if block == SEEK_BLOCK {
            assert_eq!(
                host.session_seek(2, SEEK_TIMELINE, None),
                RESULT_OK,
                "the session seek"
            );
        }
        let (generation, start) = if block < SEEK_BLOCK {
            (1, block * u64::from(QUANTUM))
        } else {
            (2, SEEK_TIMELINE + (block - SEEK_BLOCK) * u64::from(QUANTUM))
        };
        for (source, multipliers) in SOURCES {
            let [left, right] = multipliers.map(|multiplier| {
                (start..start + u64::from(QUANTUM))
                    .map(|frame| pcm(frame, multiplier))
                    .collect::<Vec<f32>>()
            });
            let planes: [&[f32]; 2] = [&left, &right];
            assert_eq!(
                host.submit_source(source, generation, start, RATE, &planes, QUANTUM, false),
                RESULT_OK,
                "block {block} submit"
            );
        }
        assert_eq!(host.render_next(), RESULT_OK, "block {block} render");
        blocks.push(host.output_pcm().expect("prepared output").to_vec());
    }
    assert_eq!(host.status().rendered_quanta, BLOCKS);
    blocks
}

/// #1669 D3: the stored-automation timeline's native digest. `tests/browser-v1/automation-session.json`
/// automates every row the engine renders today -- the strip fader (`both` and one lane, with
/// stored offsets), the strip mute (one lane), the VCA offset and the VCA mute -- with a following
/// and a non-following send, and is seeked once while it plays (D2). `direct-oracle.mjs` renders
/// the same session and transcript through the shipped simd128 artifact and asserts its digest
/// equals the pin this test checks. The digest is over little-endian `f32` words (every left
/// block, then every right block), so it is a bit comparison.
///
/// Test value: red if the simd128 artifact renders an automation event, a composition (fader with
/// VCA offset, mute OR, the follow gate) or the landing of a plain session seek differently from
/// x86-64 native. The other artifact-versus-native renders have empty automation.
#[test]
fn native_automation_timeline_digest_pins_the_wasm_parity() {
    use sha2::{Digest, Sha256};

    const QUANTUM: usize = 128;
    // The fixture file is read verbatim, exactly as `direct-oracle.mjs` reads it.
    let document = include_str!("../tests/browser-v1/automation-session.json");
    let blocks = automation_transcript(document.as_bytes());

    // The automation is heard on both sides of the seek: the same session with no entry renders
    // other bits before the seek and after it.
    let mut plain = parse_session_json(document).expect("the automation fixture");
    plain.automation.clear();
    let plain = automation_transcript(
        canonical_session_json(&plain)
            .expect("canonical plain session")
            .as_bytes(),
    );
    assert_ne!(
        blocks[..40],
        plain[..40],
        "automation heard before the seek"
    );
    assert_ne!(blocks[40..], plain[40..], "automation heard after the seek");
    // The seek's landing is heard (review fold 1, M1): `t1-fader` holds `value` dB over
    // 30000-30064 only, so two values differ only where the seek lands, and block 40 differs.
    let landing = |value: f32| {
        let mut model = parse_session_json(document).expect("the automation fixture");
        let ride = model
            .automation
            .iter_mut()
            .find(|entry| entry.id.as_str() == "t1-fader")
            .expect("t1-fader");
        let segment = |shape, span: (u64, u64), values: (f32, f32)| session::AutomationSegment {
            shape,
            start_sample: span.0,
            end_sample: span.1,
            start_value: values.0,
            end_value: values.1,
            unit: session::ParameterUnit::Db,
        };
        let (step, linear) = (
            session::AutomationShape::Step,
            session::AutomationShape::Linear,
        );
        ride.segments.splice(
            2..3,
            [
                segment(step, (29_000, 30_000), (-9.0, value)),
                segment(step, (30_000, 30_064), (value, -9.0)),
                segment(linear, (30_064, 33_000), (-9.0, 3.0)),
            ],
        );
        automation_transcript(
            canonical_session_json(&model)
                .expect("canonical")
                .as_bytes(),
        )
    };
    assert_ne!(
        landing(-6.0)[40],
        landing(20.0)[40],
        "the seek's fader landing is heard"
    );

    let mut digest = Sha256::new();
    for channel in 0..2_usize {
        for output in &blocks {
            for sample in &output[channel * QUANTUM..(channel + 1) * QUANTUM] {
                digest.update(sample.to_bits().to_le_bytes());
            }
        }
    }
    let native = engine::hex_lower(&digest.finalize());

    let expected: serde_pin::Pin =
        serde_pin::read(include_str!("../tests/browser-v1/expected.json"));
    assert_eq!(
        native, expected.native_automation,
        "native and the pinned wasm automation-timeline digest must agree bit for bit"
    );
    assert_eq!(
        native, expected.simd128_automation,
        "the shipped simd128 artifact renders this automation timeline to the same bits as native"
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
        pub(super) native_automation: String,
        pub(super) simd128_automation: String,
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
            native_automation: hex_after(text, "\"nativeAutomationPcmF32leSha256\":"),
            simd128_automation: hex_after(
                &text[text
                    .find("\"automationTimeline\"")
                    .expect("expected.json has no automationTimeline")..],
                "\"pcmF32leSha256\":",
            ),
        }
    }
}

/// The stable key of the strip at `track` (tracks, then submixes) and of the console slot or insert
/// at `effect` of `rack` (a console slot's index over `pre_insert` then `post_insert`), on the
/// committed model, the one an `Observe` edit and an `OBSERVATION_READ` resolve against; an index
/// the model does not have names an ID it does not have either.
fn key_at(host: &AudioWorkletEngineHost, track: u32, rack: u8, effect: u32) -> (Vec<u8>, Vec<u8>) {
    let model = host
        .test_control()
        .control
        .committed_session()
        .normalized_model();
    let strip = track as usize;
    let inserts = model
        .tracks
        .get(strip)
        .map(|track| (&track.id, &track.inserts))
        .or_else(|| {
            model
                .submixes
                .get(strip.checked_sub(model.tracks.len())?)
                .map(|submix| (&submix.id, &submix.inserts))
        });
    let strip_id = inserts.map_or_else(
        || format!("no-strip-{track}").into_bytes(),
        |(id, _)| id.as_str().as_bytes().to_vec(),
    );
    let index = effect as usize;
    let effect_id = match rack {
        RACK_INSERTS => inserts
            .and_then(|(_, rack)| rack.effects.get(index))
            .map(|effect| effect.id.as_str()),
        RACK_CONSOLE => model
            .console
            .pre_insert
            .iter()
            .chain(model.console.post_insert.iter())
            .nth(index)
            .map(|slot| slot.slot.as_str()),
        _ => None,
    }
    .map_or_else(
        || format!("no-effect-{effect}").into_bytes(),
        |id| id.as_bytes().to_vec(),
    );
    (strip_id, effect_id)
}

/// A tap by the positions the deleted numeric observation address used (#1590): [`read_taps`]
/// reads it by its stable key ([`key_at`]) through `OBSERVATION_READ`.
#[derive(Clone, Copy, Debug)]
struct TapAt {
    track_index: u32,
    rack: u8,
    effect_index: u32,
    tap_id: u32,
    channels: protocol::ParameterChannel,
}

/// One row of [`read_taps`]: the core's `OBSERVATION_READ` row.
type TapRow = protocol::ObservationReadRow;

/// One `OBSERVATION_READ` (revision `Any`) of `taps` through the browser's control entry (#1590:
/// the browser's own observation read is gone): the rows in order, or the codes of the refusal's
/// diagnostics (`result N` for a control call that failed with result `N`).
fn read_taps(
    host: &mut AudioWorkletEngineHost,
    taps: &[TapAt],
) -> Result<Vec<TapRow>, Vec<String>> {
    use protocol::{
        CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
        ExpectedRevision, ObservationReadRequest, ObservationReadSelection, ProtocolCodec,
        RequestId, TypedCommandFrame,
    };
    let id = |bytes: Vec<u8>| {
        session::StableId::parse(core::str::from_utf8(&bytes).expect("an ASCII ID"))
            .expect("a stable ID")
    };
    let request = ObservationReadRequest {
        selections: taps
            .iter()
            .map(|tap| {
                let (strip, effect) = key_at(host, tap.track_index, tap.rack, tap.effect_index);
                let effect = id(effect);
                ObservationReadSelection {
                    strip_id: id(strip),
                    section: if tap.rack == RACK_CONSOLE {
                        controls::TapSection::Console(effect)
                    } else {
                        controls::TapSection::Inserts(effect)
                    },
                    tap_id: tap.tap_id,
                    channels: tap.channels,
                }
            })
            .collect(),
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(next_request_id()).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::ObservationRead(&request),
            },
            &mut bytes,
        )
        .expect("an observation read frame");
    let staged = u32::try_from(length).expect("frame length");
    host.control_request_mut(staged)
        .expect("the control staging")[..length]
        .copy_from_slice(&bytes[..length]);
    let result = host.control_command(staged);
    if result != RESULT_OK {
        return Err(vec![format!("result {result}")]);
    }
    let mut fields = [0_u16; 256];
    match codec
        .decode_typed_response(
            host.control_response(),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a response frame")
    {
        DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ObservationRead(page),
            ..
        } => Ok(page.rows.into_owned()),
        DecodedTypedResponseFrame::NonOk { payload, .. } => Err(payload
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.to_string())
            .collect()),
        DecodedTypedResponseFrame::Success { .. } => panic!("an observation read page"),
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
#[cfg(feature = "test-support")]
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

/// #1689 gate 2, #1590 gate 4: the boot words at offsets 48 and 56 are reserved. A nonzero value
/// is refused `web.options.reserved3` (the word was the observation-taps word until every plan
/// bound every declared tap) or `web.options.reserved2` (the word was the master boot word until
/// the `master` row replaced it). Issue #2191 gate 3: the control frame word at offset 32 is
/// refused `web.options.control_frame_bytes` below 16 KiB and above 16 MiB.
///
/// Test value: red if a deleted word still changes behaviour (a writer that still sets it is
/// accepted and ignored), or if a frame word outside its bounds boots (one that starves every
/// reply, or one past the `u32` staging addresses).
#[test]
fn the_retired_boot_words_are_reserved_and_the_frame_word_is_bounded() {
    const QUANTUM: u32 = 128;
    let document = include_str!("../tests/browser-v1/command-session.json");
    let refusal = |options: WebBootOptions| {
        AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .err()
            .expect("a nonzero reserved word must not boot")
    };
    for (options, code) in [
        (
            WebBootOptions {
                maximum_control_frame_bytes: u64::from(MINIMUM_CONTROL_FRAME_BYTES) - 1,
                ..boot_options(QUANTUM)
            },
            &b"web.options.control_frame_bytes\t$\n"[..],
        ),
        (
            WebBootOptions {
                maximum_control_frame_bytes: u64::from(MAXIMUM_CONTROL_FRAME_BYTES) + 1,
                ..boot_options(QUANTUM)
            },
            &b"web.options.control_frame_bytes\t$\n"[..],
        ),
        (
            WebBootOptions {
                reserved3: 4,
                ..boot_options(QUANTUM)
            },
            &b"web.options.reserved3\t$\n"[..],
        ),
        (
            WebBootOptions {
                reserved2: 1,
                ..boot_options(QUANTUM)
            },
            &b"web.options.reserved2\t$\n"[..],
        ),
        (
            WebBootOptions {
                reserved2: u64::MAX,
                ..boot_options(QUANTUM)
            },
            &b"web.options.reserved2\t$\n"[..],
        ),
    ] {
        let refused = refusal(options);
        assert_eq!(refused.result(), RESULT_REFUSED_OPTIONS);
        assert_eq!(refused.diagnostic(), code);
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

/// Answer one `COUNTERS_GET` frame for counter `id` through the browser's control entry, after a
/// service step, and return its value.
fn served_counter(host: &mut AudioWorkletEngineHost, request: u64, id: protocol::CounterId) -> u64 {
    use protocol::{
        CommandPayload, CountersRequest, DecodeScratch, DecodedSuccessResponsePayload,
        DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec, RequestId, TypedCommandFrame,
    };
    assert_eq!(host.service(), RESULT_OK);
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::CountersGet(&CountersRequest {
                    all: false,
                    ids: vec![id as u32],
                }),
            },
            &mut bytes,
        )
        .expect("a counters frame");
    let staged = u32::try_from(length).expect("frame length");
    host.control_request_mut(staged)
        .expect("the control request staging")[..length]
        .copy_from_slice(&bytes[..length]);
    assert_eq!(host.control_command(staged), RESULT_OK);
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
        ..
    } = codec
        .decode_typed_response(
            host.control_response(),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a counters response")
    else {
        panic!("expected a counter snapshot")
    };
    assert_eq!(snapshot.values.len(), 1);
    assert_eq!(snapshot.values[0].id, id);
    snapshot.values[0].value
}

/// Issue #1537 gate 1: `COUNTERS_GET` through the browser's control entry reports
/// `SOURCE_UNDERRUN_BLOCKS`, the rendered blocks in which a source played zeros because the host
/// had not delivered PCM it owed. Each case feeds blocks 0 and 1, then renders two blocks with
/// nothing submitted, and reads the counter at boot and after each render's service step. With an
/// eight-block source the two starved blocks count (`[0, 0, 1, 2]`); with a two-block source
/// block 1 ends the region, so the next two blocks are past its end and count nothing.
///
/// Test value: red if the count never moves, counts every block, counts the previous block's
/// flag or a stale plan's, or counts silence past the end of a region as starvation; no other
/// test reads an underrun count from a host.
#[test]
fn the_counters_get_reply_counts_each_block_a_source_underran() {
    fn underrun_blocks(host: &mut AudioWorkletEngineHost, request: u64) -> u64 {
        served_counter(host, request, protocol::CounterId::SourceUnderrunBlocks)
    }
    for (source_blocks, expected) in [(8_u64, [0_u64, 0, 1, 2]), (2, [0, 0, 0, 0])] {
        let mut host = meter_tail_host_for_blocks(48_000, 128, 32, source_blocks);
        assert_eq!(
            underrun_blocks(&mut host, 1),
            0,
            "{source_blocks} blocks: boot"
        );
        let mut seen = [0_u64; 4];
        for (block, count) in (0_u64..).zip(seen.iter_mut()) {
            if block < 2 {
                feed_and_render_finite_source(&mut host, block, source_blocks, 0.25);
            } else {
                assert_eq!(host.render_next(), RESULT_OK);
            }
            *count = underrun_blocks(&mut host, block + 2);
        }
        assert_eq!(seen, expected, "{source_blocks}-block source");
    }
}

/// Issue #1699 gates 1-3: `COUNTERS_GET` reports `SOURCE_FRAMES_DISCARDED`, the accepted
/// frames render never read because the caller's own transaction removed their source. The one
/// source is fed three blocks and render plays one, so two (256 frames) are unread when the
/// removal commits. Until the plan without the source is adopted at `S`, after the removed
/// strip's ramp (#1325 D3, D4), the source feeds the ramp one block a block and nothing is
/// counted; the adoption retires the two unread blocks, and the service step after it counts
/// them. Case "empty" plays every fed block first, feeds nothing more, and counts 0. Case
/// "supersede" commits an unrelated rebuild first, which carries the source unchanged and waits
/// unrendered, then a removal that supersedes it: it counts the same as the direct removal.
///
/// Test value: red if the count never moves, counts a block render played, or misses the frames
/// a superseding removal discards; no other test reads a discard count from a host.
#[test]
fn the_counters_get_reply_counts_the_frames_a_source_removal_discards() {
    use protocol::{
        CommandPayload, CounterId, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
        SessionRevision, TypedCommandFrame,
    };
    const QUANTUM: u32 = 128;
    fn commit(host: &mut AudioWorkletEngineHost, request: u64, edits: &[SessionEdit]) {
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(
                        host.committed_revision(),
                    )),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        let staged = u32::try_from(length).expect("frame length");
        host.control_request_mut(staged)
            .expect("the control request staging")[..length]
            .copy_from_slice(&bytes[..length]);
        assert_eq!(host.control_command(staged), RESULT_OK);
        let mut fields = [0_u16; 64];
        assert!(
            matches!(
                codec.decode_typed_response(
                    host.control_response(),
                    &mut protocol::DecodeScratch::new(&mut fields)
                ),
                Ok(protocol::DecodedTypedResponseFrame::Success { .. })
            ),
            "request {request} commits"
        );
    }
    let mut model = parse_session_json(include_str!("../tests/browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.sources[0].frames = u64::from(QUANTUM) * 8;
    let removal = vec![
        SessionEdit::RemoveRoute {
            route_id: model.routes[0].id.clone(),
        },
        SessionEdit::RemoveTrack {
            track_id: model.tracks[0].id.clone(),
        },
        SessionEdit::RemoveSource {
            source_id: model.sources[0].id.clone(),
        },
    ];
    let mut extra = model.tracks[0].clone();
    extra.id = session::StableId::parse("track-extra").expect("track id");
    extra.fader.left_mute = true;
    extra.fader.right_mute = true;
    let document = canonical_session_json(&model).expect("canonical fixture");
    let options = WebBootOptions {
        require_sample_rate_hz: 48_000,
        require_quantum_frames: QUANTUM,
        source_ring_frames: QUANTUM * 4,
        ..WebBootOptions::explicit_defaults()
    };
    let samples = [0.25_f32; QUANTUM as usize];
    let planes: [&[f32]; 2] = [&samples, &samples];
    for (case, played, expected) in [
        ("direct", 1_u64, 256_u64),
        ("empty", 3, 0),
        ("supersede", 1, 256),
    ] {
        let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
        for block in 0..3_u64 {
            assert_eq!(
                host.submit_source(
                    b"fixture-source",
                    1,
                    block * u64::from(QUANTUM),
                    48_000,
                    &planes,
                    QUANTUM,
                    false,
                ),
                RESULT_OK,
                "{case}: block {block}"
            );
        }
        for _ in 0..played {
            assert_eq!(host.render_next(), RESULT_OK);
        }
        assert_eq!(
            served_counter(&mut host, 1, CounterId::SourceFramesDiscarded),
            0,
            "{case}: before the removal"
        );
        if case == "supersede" {
            commit(
                &mut host,
                2,
                &[SessionEdit::UpsertTrack {
                    track: extra.clone(),
                }],
            );
            let mut edits = vec![SessionEdit::RemoveTrack {
                track_id: extra.id.clone(),
            }];
            edits.extend(removal.iter().cloned());
            commit(&mut host, 3, &edits);
        } else {
            commit(&mut host, 2, &removal);
        }
        let committed = host.committed_revision();
        let mut next_block = 3_u64;
        let mut request = 4;
        loop {
            assert_eq!(host.render_next(), RESULT_OK);
            assert_eq!(host.read_watermark(), RESULT_OK);
            if host.watermark().revision >= committed {
                break;
            }
            assert!(
                next_block < 8,
                "{case}: the removal is adopted inside the region"
            );
            assert_eq!(
                served_counter(&mut host, request, CounterId::SourceFramesDiscarded),
                0,
                "{case}: nothing is discarded before S"
            );
            request += 1;
            if expected > 0 {
                assert_eq!(
                    host.submit_source(
                        b"fixture-source",
                        1,
                        next_block * u64::from(QUANTUM),
                        48_000,
                        &planes,
                        QUANTUM,
                        next_block + 1 == 8,
                    ),
                    RESULT_OK,
                    "{case}: the removed source feeds the ramp"
                );
                next_block += 1;
            }
        }
        assert_eq!(
            served_counter(&mut host, request, CounterId::SourceFramesDiscarded),
            expected,
            "{case}"
        );
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

// It plants a window through host-core's `test-support` writers.
#[cfg(feature = "test-support")]
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
        host.test_render_mut()
            .meters
            .test_only_output_mut()
            .windows_mut()
            .try_pop()
            .ok()
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

// It plants a window through host-core's `test-support` writers.
#[cfg(feature = "test-support")]
#[test]
fn meter_producer_reset_starts_a_fresh_delivery_epoch() {
    let mut host = live_control_host(128, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    feed_and_render(&mut host, 1, 0, 0.9);
    assert_eq!(host.poll_meters(), 1);
    let published_frame = host.meter_frame().to_vec();
    let published_header = *host.meter_header();

    feed_and_render(&mut host, 1, 1, 0.2);
    let ready = host.test_render_mut();
    let companions = ready
        .plan
        .active_attachment_mut::<RenderCompanions>()
        .expect("companions");
    for (meter, pending) in companions
        .meters
        .iter_mut()
        .zip(ready.meters.test_only_pending_mut().iter_mut())
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
        // #1581 D5: the four output windows the full queue dropped (blocks 8 to 11) are counted
        // from the first frame on; the strip's own drops reach the count with its next snapshot.
        assert_eq!(host.meter_header().reserved[1] >> METER_LOSS_SHIFT, 4);
    }
    assert_eq!(host.poll_meters(), 0);
    feed_and_render(&mut host, 1, 12, 0.3);
    assert_eq!(host.poll_meters(), 1);
    assert_eq!(host.meter_header().first_sample, 12 * 128);
    assert_eq!(&host.meter_frame()[..4], &[0.3, 0.3, 0.3, 0.3]);
    assert_ne!(host.meter_header().reserved[1] & METER_VALID_LOSS, 0);
    // Exact: the four output drops, the strip's four, and the gap from block 8 to block 12.
    assert_eq!(
        host.meter_header().reserved[1] >> METER_LOSS_SHIFT,
        4 + 4 + 1
    );
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
    // The retake restarts the frame sequence and the reported window, and clears the frame.
    assert_eq!((empty_header.sequence, empty_header.first_sample), (0, 0));
    assert!(empty_frame.iter().all(|value| *value == 0.0));

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

/// One `METER_BATCH`: its span, its window sequence and its records' values in order.
type MeterBatchRow = (u64, u64, u64, Vec<f32>);

/// Run one service step and dequeue every lossy event, keeping the `METER_BATCH`es.
fn served_meter_batches(host: &mut AudioWorkletEngineHost) -> Vec<MeterBatchRow> {
    use protocol::{DecodeScratch, DecodedEventPayload, DecodedTypedEventFrame, ProtocolCodec};
    assert_eq!(host.service(), RESULT_OK);
    let mut batches = Vec::new();
    loop {
        assert_eq!(host.control_event(EVENT_LANE_LOSSY), RESULT_OK);
        let event = host.control_response();
        if event.is_empty() {
            return batches;
        }
        let mut fields = [0_u16; 64];
        if let DecodedTypedEventFrame {
            payload: DecodedEventPayload::MeterBatch(batch),
            ..
        } = ProtocolCodec::default()
            .decode_typed_event(event, &mut DecodeScratch::new(&mut fields))
            .expect("a lossy event")
        {
            batches.push((
                batch.first_sample.0,
                batch.observed_sample.0,
                batch.window_sequence,
                (0..usize::from(batch.count))
                    .map(|index| batch.record(index).expect("a meter record").value)
                    .collect(),
            ));
        }
    }
}

/// #1588 gate 2: the lease and a protocol meter table at once. Under the lease every batch of
/// `k = 2` covers exactly two consecutive lease frames' spans, and its output and track peaks are
/// the two frames' maxima. Releasing the lease restarts the delivery epoch once: the partial
/// batch it cuts is counted in `METER_WINDOWS_LOST`, the batches continue on the new epoch, and
/// with the table alone the lease frame and its header stay as the release left them.
///
/// Test value: red if the two clients fold different windows (a batch that is not the two lease
/// frames' span or maxima), if one client's change moves the other's data without a counted
/// restart, or if the table alone writes the lease frame.
#[test]
fn the_lease_and_a_meter_table_fold_one_window_grid_and_a_release_restarts_it_once() {
    use protocol::{
        CommandPayload, CounterId, DiagnosticSeverity, ExpectedRevision, MeterEntry, MeterMetric,
        MeterTarget, ProtocolCodec, RequestId, SessionRevision, TelemetryConfiguration,
        TypedCommandFrame,
    };
    const QUANTUM: u64 = 128;
    let mut host = live_control_host(QUANTUM as u32, 1);
    assert_eq!(host.set_meter_lease(true), RESULT_OK);
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(
                    host.committed_revision(),
                )),
                payload: CommandPayload::TelemetryConfigure(&TelemetryConfiguration {
                    meters: vec![
                        MeterEntry {
                            target: MeterTarget::Output,
                            metric: MeterMetric::Peak,
                        },
                        MeterEntry {
                            target: MeterTarget::Strip("track".into()),
                            metric: MeterMetric::Peak,
                        },
                    ],
                    meter_period_blocks: 2,
                    meter_window_blocks: None,
                    counter_ids: Vec::new(),
                    counter_period_blocks: 0,
                    diagnostics_enabled: false,
                    minimum_diagnostic_severity: DiagnosticSeverity::Info,
                }),
            },
            &mut bytes,
        )
        .expect("a telemetry frame");
    let staged = u32::try_from(length).expect("frame length");
    host.control_request_mut(staged)
        .expect("the control request staging")[..length]
        .copy_from_slice(&bytes[..length]);
    assert_eq!(host.control_command(staged), RESULT_OK);

    // Seven lease frames: three whole batches and one frame of a fourth.
    let values = [0.25_f32, 0.5, 0.125, 1.0, 0.0625, 0.75, 0.375];
    let mut leased = Vec::new();
    let mut batches = Vec::new();
    for (block, value) in values.into_iter().enumerate() {
        feed_and_render(&mut host, 1, block as u64, value);
        assert_eq!(host.poll_meters(), 1, "block {block}: one lease frame");
        let header = *host.meter_header();
        let frame = host.meter_frame();
        leased.push((header.first_sample, header.end_sample, frame[0], frame[2]));
        batches.extend(served_meter_batches(&mut host));
    }
    assert_eq!(batches.len(), 3, "a batch every two lease frames");
    for (index, (first, end, sequence, records)) in batches.iter().enumerate() {
        let (one, two) = (leased[2 * index], leased[2 * index + 1]);
        assert_eq!(two.0, one.1, "batch {index}: two consecutive lease frames");
        assert_eq!((*first, *end), (one.0, two.1), "batch {index}: their span");
        assert_eq!(*sequence, index as u64 + 1);
        let (output, track) = (one.3.max(two.3), one.2.max(two.2));
        assert_eq!(
            records,
            &vec![output, output, track, track],
            "batch {index}: their maxima"
        );
    }
    let lost = served_counter(&mut host, 2, CounterId::MeterWindowsLost);

    // The release restarts the epoch at the next block; the table alone then folds in render.
    assert_eq!(host.set_meter_lease(false), RESULT_OK);
    let released = (host.meter_frame().to_vec(), *host.meter_header());
    let restart = values.len() as u64;
    let mut after = Vec::new();
    for block in restart..restart + 4 {
        feed_and_render(&mut host, 1, block, 0.5);
        after.extend(served_meter_batches(&mut host));
    }
    assert_eq!(
        after
            .iter()
            .map(|(first, end, sequence, _)| (*first, *end, *sequence))
            .collect::<Vec<_>>(),
        [
            (restart * QUANTUM, (restart + 2) * QUANTUM, 4),
            ((restart + 2) * QUANTUM, (restart + 4) * QUANTUM, 5),
        ],
        "the batches continue from the restart"
    );
    assert_eq!(
        served_counter(&mut host, 3, CounterId::MeterWindowsLost),
        lost + 1,
        "the restart's one counted window: the partial batch it cut"
    );
    assert_eq!(
        (host.meter_frame().to_vec(), *host.meter_header()),
        released,
        "the table alone leaves the lease frame as the release left it"
    );
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
        ..boot_options(quantum)
    };
    let mut host =
        AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("observation boot");
    if let Some(track) = master {
        let strip = host.live_control_tracks()[track as usize].to_string();
        designate_master(&mut host, &strip);
    }
    host
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
                link_mode: session::ConsoleLinkMode::Slot,
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
/// producer and observation handle under its strip.
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
    let ready = host.test_render();
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
        // The dense table files a strip's effects together (#1590 deleted the per-strip address
        // tables the numeric observation read used): each instance names its strip, its effect
        // and, for an effect that declares a tap, its observer and the strip's index.
        let mut expected: Vec<&str> = ["miso.parametric-eq", "miso.true-peak-limiter"]
            .into_iter()
            .chain(inserts[strip].iter().copied())
            .collect();
        expected.sort_unstable();
        let mut natives = Vec::new();
        for (slot, producer) in ready.shape.effect_instances.iter().enumerate() {
            let Some(producer) = producer
                .as_ref()
                .filter(|producer| &*producer.track_id == *id)
            else {
                continue;
            };
            let native = producer.descriptor.id.as_str();
            natives.push(native);
            if native == "miso.parametric-eq" {
                assert!(producer.reader.is_none(), "{id}: an EQ declares no tap");
                assert_eq!(ready.shape.observation_tracks[slot], u32::MAX);
            } else {
                assert!(
                    producer.reader.is_some(),
                    "{id} {native} has a filed observer"
                );
                assert_eq!(ready.shape.observation_tracks[slot] as usize, strip);
            }
            filed += 1;
        }
        natives.sort_unstable();
        assert_eq!(
            natives, expected,
            "{id}: every effect of the strip is filed"
        );
    }
    assert_eq!(filed, 13);
    assert_eq!(
        ready.shape.effect_instances.len(),
        13,
        "no producer slot beyond the strips'"
    );
    assert_eq!(
        ready.meters.frame().strip_gain_reduction_present.len(),
        strips.len()
    );
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
    bus_meter_host_booted(quantum, buses, meter_blocks, source_quanta, None)
}

/// [`bus_meter_host_with`] booted with `boot_command` staged, as `miso_engine_web_v1_boot_command_ptr`
/// stages it (#2166).
fn bus_meter_host_booted(
    quantum: u32,
    buses: bool,
    meter_blocks: u64,
    source_quanta: u64,
    boot_command: Option<&[u8]>,
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
    AudioWorkletEngineHost::boot_with_command(document.as_bytes(), options, boot_command)
        .unwrap_or_else(|failure| {
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
        let ready = host.test_render_mut();
        let companions = ready
            .plan
            .active_attachment_mut::<RenderCompanions>()
            .expect("companions");
        companions
            .meters
            .iter()
            .zip(ready.meters.tables().pending().iter())
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
    assert_eq!(header.struct_size, 88);
    assert_eq!(header.track_count, 3);
    assert_eq!(header.submix_count, 2);
    assert_eq!(header.retired, 0);
    assert_eq!(header.successor_pointer, 0);
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

/// One stereo spectrum tap on `target_id`, encoded as a `SPECTRUM_CONFIGURE` command frame: the
/// frame the SDK stages as the boot command (#2166), or sends through the command entry.
fn spectrum_configure_frame(target_kind: protocol::SpectrumTargetKind, target_id: &str) -> Vec<u8> {
    spectrum_frame(
        1,
        protocol::CommandPayload::SpectrumConfigure(&protocol::SpectrumConfigureRequest {
            form: protocol::SpectrumForm::Single,
            entries: vec![protocol::SpectrumEntry {
                target_kind,
                target_id: target_id.to_owned(),
                channels: protocol::ParameterChannel::Both,
            }],
            maximum_capture_bytes: host_core::SPECTRUM_MAXIMUM_CAPTURE_BYTES,
            hop_frames: None,
        }),
    )
}

/// `payload` encoded as one command frame with request ID `request` and no expected revision.
fn spectrum_frame(request: u64, payload: protocol::CommandPayload<'_>) -> Vec<u8> {
    let mut bytes = vec![0_u8; 4_096];
    let length = protocol::ProtocolCodec::default()
        .encode_command_frame_into(
            &protocol::TypedCommandFrame {
                request_id: protocol::RequestId::new(request).expect("nonzero request"),
                expected_revision: protocol::ExpectedRevision::Any,
                payload,
            },
            &mut bytes,
        )
        .expect("a spectrum frame");
    bytes.truncate(length);
    bytes
}

/// Run `frame` through `host`'s command entry and hand its success payload to `read`.
fn spectrum_command<R>(
    host: &mut AudioWorkletEngineHost,
    frame: &[u8],
    read: impl FnOnce(protocol::DecodedSuccessResponsePayload<'_>) -> R,
) -> R {
    use protocol::{DecodeScratch, DecodedTypedResponseFrame, ProtocolCodec};
    let len = u32::try_from(frame.len()).expect("a short frame");
    host.control_request_mut(len).expect("the request staging")[..frame.len()]
        .copy_from_slice(frame);
    assert_eq!(host.control_command(len), RESULT_OK);
    let mut fields = vec![0_u16; 8_192];
    match ProtocolCodec::default()
        .decode_typed_response(
            host.control_response(),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a spectrum reply")
    {
        DecodedTypedResponseFrame::Success { payload, .. } => read(payload),
        DecodedTypedResponseFrame::NonOk { payload, .. } => {
            panic!("the spectrum command was refused: {payload:?}")
        }
    }
}

/// Issue #2166 claim (a): a boot whose staged command sets a spectrum tap counts no blocking
/// rebuild, and the tap's stream delivers bins once a window has rendered: the tap is the first
/// plan's.
///
/// Red if the boot drops the command (the stream start answers `Unprepared`), if the monitor does
/// not adopt the boot plan's captures (likewise), or if the tap is set by a plan that replaces the
/// boot's (`blocking_rebuilds` is 1).
#[test]
fn a_boot_spectrum_tap_is_the_first_plans_and_streams_with_no_rebuild() {
    const QUANTUM: u32 = 128;
    let configure = spectrum_configure_frame(protocol::SpectrumTargetKind::Output, "main-out");
    let mut host = bus_meter_host_booted(QUANTUM, false, 2, 64, Some(&configure));
    assert_eq!(host.blocking_rebuilds(), 0, "the boot rebuilt nothing");
    // The SDK's confirmation: the same configuration again is equal, so it rebuilds nothing.
    let (rebuild, epochs) = spectrum_command(&mut host, &configure, |payload| match payload {
        protocol::DecodedSuccessResponsePayload::SpectrumConfigured(reply) => {
            (reply.rebuild, (reply.configured_epoch, reply.adopted_epoch))
        }
        _ => panic!("a configure reply"),
    });
    assert_eq!(
        (rebuild, epochs),
        (false, (1, 1)),
        "the confirmation is a no-op"
    );
    assert_eq!(
        host.blocking_rebuilds(),
        0,
        "the confirmation rebuilt nothing"
    );
    let start = spectrum_frame(
        2,
        protocol::CommandPayload::SpectrumStream(&protocol::SpectrumStreamRequest::Start {
            smoothing_ms: 0.0,
        }),
    );
    let adopted = spectrum_command(&mut host, &start, |payload| match payload {
        protocol::DecodedSuccessResponsePayload::SpectrumStreamed(reply) => reply.adopted_epoch,
        _ => panic!("a stream reply"),
    });
    assert_eq!(adopted, 1, "the boot plan carries the configuration");
    let windows = host_core::SPECTRUM_WINDOW_FRAMES as u64 / u64::from(QUANTUM);
    for block in 0..windows {
        feed_and_render_channels(&mut host, block, 0.5, 0.25);
    }
    let read = spectrum_frame(
        3,
        protocol::CommandPayload::SpectrumStream(&protocol::SpectrumStreamRequest::Read),
    );
    spectrum_command(&mut host, &read, |payload| {
        let protocol::DecodedSuccessResponsePayload::SpectrumStreamed(reply) = payload else {
            panic!("a stream reply");
        };
        assert_eq!(reply.status, protocol::SpectrumStreamStatus::Ready);
        let bins = reply.bins.expect("a ready read carries bins");
        assert!(bins.left_db.is_some() && bins.right_db.is_some());
    });
    assert_eq!(host.blocking_rebuilds(), 0, "rendering rebuilt nothing");
}

/// #2166: a refused boot takes the staged command with it, so the next boot does not carry a
/// command staged for another document.
///
/// Red if the boot takes the command only after its document check passes: a boot refused there
/// would leave the frame for the next boot.
#[test]
fn a_refused_boot_takes_the_staged_boot_command() {
    let configure = spectrum_configure_frame(protocol::SpectrumTargetKind::Output, "main-out");
    let document = b"{}";
    crate::ffi::test_stage_document(document);
    assert!(crate::ffi::test_stage_boot_command(&configure));
    // A length that is not the staged document's: refused at the document check.
    assert_eq!(
        crate::ffi::miso_engine_web_v1_boot(document.len() as u32 + 1),
        0
    );
    assert_eq!(crate::ffi::test_staged_boot_command_bytes(), 0);
}

/// Issue #2166 claim (b): a `SPECTRUM_CONFIGURE` that changes the taps of a running session is a
/// plan rebuild the browser counts, once.
///
/// Red if the browser stops counting a rebuild that changes no revision (a spectrum configure
/// commits no edit), as a fix of claim (a) that excused the epoch change rather than avoiding it
/// would.
#[test]
fn a_running_spectrum_configure_counts_one_blocking_rebuild() {
    const QUANTUM: u32 = 128;
    let mut host = bus_meter_host_with(QUANTUM, false, 2, 64);
    feed_and_render_channels(&mut host, 0, 0.5, 0.25);
    let configure = spectrum_configure_frame(protocol::SpectrumTargetKind::Output, "main-out");
    let rebuild = spectrum_command(&mut host, &configure, |payload| match payload {
        protocol::DecodedSuccessResponsePayload::SpectrumConfigured(reply) => reply.rebuild,
        _ => panic!("a configure reply"),
    });
    assert!(rebuild, "the configure prepared a plan");
    assert_eq!(host.blocking_rebuilds(), 1);
}

/// Issue #1581 gate 2: a session with a spectrum tap meters the submixes as the plain boot does,
/// so the bus fixture's frame is the plain boot's, word for word, with nonzero submix peaks. The
/// tap is the boot command's (#2166), as a boot's taps are.
///
/// Red on the base: the spectrum entries metered the tracks alone, so the submix pairs stayed `0`.
#[test]
fn a_spectrum_tap_meters_every_submix_like_the_plain_boot() {
    const QUANTUM: u32 = 128;
    const BLOCKS: u64 = 8;
    let mut plain = bus_meter_host(QUANTUM, true);
    let expected = bus_meter_frame(&mut plain, BLOCKS);
    let configure = spectrum_configure_frame(protocol::SpectrumTargetKind::Output, "main-out");
    let mut host = bus_meter_host_booted(QUANTUM, true, 2, 64, Some(&configure));
    let frame = bus_meter_frame(&mut host, BLOCKS);
    let tracks = 3;
    for (word, peak) in frame
        .iter()
        .enumerate()
        .take(2 * (tracks + 2))
        .skip(2 * tracks)
    {
        assert!(*peak > 0.0, "submix peak word {word} is metered");
    }
    assert_eq!(
        frame.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
        expected
            .iter()
            .map(|word| word.to_bits())
            .collect::<Vec<_>>(),
        "the configured session's frame is the plain boot's"
    );
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
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), Ok(()));
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
    // #1581 gate 7: the measured block folds the core output meter, whose gate the lease opened.
    assert!(
        host.test_render().meters.output_gate_from() <= host.status().next_absolute_sample,
        "the output meter's gate is open for the measured block"
    );
    let ((render, windows), allocations, deallocations) =
        crate::ffi::allocation_counter::measured(|| {
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

/// The dense effect slot and the tap index of the tap at `tap`, through the plan's observation
/// index (#1586 D3, the epoch's since #1589 R2), so a test reaches the reader the read reads.
fn resolve_observation(host: &AudioWorkletEngineHost, tap: &TapAt) -> Option<(usize, usize)> {
    let ready = host.render.ready.as_ref()?;
    let (strip, effect) = key_at(host, tap.track_index, tap.rack, tap.effect_index);
    let rack = if tap.rack == RACK_CONSOLE {
        LiveEffectRack::Console
    } else {
        LiveEffectRack::Inserts
    };
    let entry = host.observation_index()?.find(
        core::str::from_utf8(&strip).ok()?,
        rack,
        core::str::from_utf8(&effect).ok()?,
    )?;
    let handle = entry.handle?;
    let slot = ready.shape.effect_instances.iter().position(|instance| {
        instance.as_ref().and_then(|instance| instance.reader) == Some(handle)
    })?;
    let index = entry
        .effect
        .observations
        .iter()
        .position(|declared| declared.id.0 == tap.tap_id)?;
    Some((slot, index))
}

/// The row window of a reader's published window.
const fn row_window(window: engine::realtime::ObservationWindow) -> protocol::ObservationRowWindow {
    protocol::ObservationRowWindow {
        first_sample: window.first_sample,
        end_sample: window.end_sample,
        sequence: window.sequence,
        blocks: window.blocks,
    }
}

/// The browser's `OBSERVATION_READ` (#1590: the core's read, through the control entry) is
/// bounded, resolves stable keys, and consumes nothing.
///
/// Test value: red if the browser's session serves a read that consumes the reader, that shows a
/// re-armed tap's old window, that keeps a disarmed tap ready, or that returns rows for a batch
/// with a bad or a repeated selection.
#[test]
fn selected_observation_reads_are_bounded_stable_and_non_consuming() {
    use protocol::{ObservationRowStatus as Status, ParameterChannel as Channel};
    const QUANTUM: u32 = 128;
    let mut host = observation_host(QUANTUM, 2, None);
    let selection = TapAt {
        track_index: 0,
        rack: RACK_INSERTS,
        effect_index: 0,
        tap_id: 1,
        channels: Channel::Both,
    };

    let unarmed = read_taps(&mut host, &[selection]).expect("unarmed selected read");
    assert_eq!(unarmed[0].status, Status::Unarmed);
    assert_eq!(unarmed[0].window, None);

    assert_eq!(
        observe(&mut host, 0, 1, 0, 1, 2, true),
        Ok(()),
        "the core's Observe edit arms the tap"
    );
    let pending = read_taps(&mut host, &[selection]).expect("pending selected read");
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].status, Status::Pending);
    assert_eq!(pending[0].window, None);

    // #1585 D4: render has applied the arm (its slot publishes the issued tag) and no window has
    // closed yet.
    feed_and_render_tracks(&mut host, 0, 0.0);
    let applied = read_taps(&mut host, &[selection]).expect("applied selected read");
    assert_eq!(applied[0].status, Status::Pending);
    assert_eq!(applied[0].window, None);
    feed_and_render_tracks(&mut host, 1, 0.0);
    let expected = {
        let (effect, tap) = resolve_observation(&host, &selection).expect("resolved selection");
        host.test_observation_handle(effect)
            .expect("observation handle")
            .readers[tap]
            .read()
            .expect("published window")
    };
    let rows = read_taps(&mut host, &[selection]).expect("ready selected read");
    assert_eq!(rows[0].status, Status::Ready);
    assert_eq!(rows[0].window, Some(row_window(expected)));
    assert_eq!(rows[0].left, Some(expected.left));
    assert_eq!(rows[0].right, Some(expected.right));

    // A re-arm is admitted before the next render block, but the reader still retains the prior
    // resident cell. That old publication is before the admitted application sample and must not
    // look ready while a replacement window is being built. (The disarm of the two-block window
    // and the arm of a one-block window in one transaction is the core's re-arm: it restarts the
    // window, and the effect's one-slot queue holds just it.)
    let rearm = [
        observe_edit_at(&mut host, 0, 1, 0, 1, 2, false),
        observe_edit_at(&mut host, 0, 1, 0, 1, 1, true),
    ];
    assert_eq!(apply_observe(&mut host, &rearm), Ok(()));
    let rearmed_pending = read_taps(&mut host, &[selection]).expect("re-arm pending selected read");
    assert_eq!(rearmed_pending[0].status, Status::Pending);
    assert_eq!(rearmed_pending[0].window, None);

    // A second observed effect is read in the same caller-ordered batch. Both rows are sourced
    // from their own resident cells, and neither selected read consumes the publication.
    let gate_selection = TapAt {
        track_index: 2,
        channels: Channel::Left,
        ..selection
    };
    assert_eq!(observe(&mut host, 2, 1, 0, 1, 2, true), Ok(()));
    for block in 2..4 {
        feed_and_render_tracks(&mut host, block, 0.0);
    }
    let expected_gate = {
        let (effect, tap) = resolve_observation(&host, &gate_selection).expect("resolved gate");
        host.test_observation_handle(effect)
            .expect("gate observation handle")
            .readers[tap]
            .read()
            .expect("gate published window")
    };
    let pair =
        read_taps(&mut host, &[selection, gate_selection]).expect("two-effect selected read");
    assert_eq!(pair.len(), 2);
    // The re-arm applied at block 2 (sample 256); the latest of its one-block windows opened at
    // block 3.
    assert_eq!(pair[0].window.map(|window| window.first_sample), Some(384));
    assert_eq!(pair[1].window, Some(row_window(expected_gate)));
    assert_eq!(pair[1].left, Some(expected_gate.left));
    assert_eq!(pair[1].right, None);
    let pair_again = read_taps(&mut host, &[selection, gate_selection])
        .expect("repeat two-effect selected read");
    assert_eq!(pair_again[0].window, pair[0].window);
    assert_eq!(pair_again[1].window, pair[1].window);
    let rearmed_window = {
        let (effect, tap) = resolve_observation(&host, &selection).expect("resolved selection");
        host.test_observation_handle(effect)
            .expect("observation handle")
            .readers[tap]
            .read()
            .expect("re-armed compressor window")
    };

    // A selected read does not acknowledge the shared reader. The same exact publication is
    // returned again, while a different requested channel only changes the owned projection.
    let right = read_taps(
        &mut host,
        &[TapAt {
            channels: Channel::Right,
            ..selection
        }],
    )
    .expect("repeat selected read");
    assert_eq!(right[0].window, Some(row_window(rearmed_window)));
    assert_eq!(right[0].left, None);
    assert_eq!(right[0].right, Some(rearmed_window.right));

    // #1585: a disarm is in effect when it is accepted, before a block drains it; no later window
    // belongs to the subscription it ends.
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 1, false), Ok(()));
    let disarmed = read_taps(&mut host, &[selection]).expect("disarmed selected read");
    assert_eq!(disarmed[0].status, Status::Unarmed);
    assert_eq!(disarmed[0].window, None);

    let invalid = [
        selection,
        TapAt {
            effect_index: 7,
            ..selection
        },
    ];
    assert!(
        read_taps(&mut host, &invalid).is_err(),
        "the complete batch is validated before any row is returned"
    );
    assert!(read_taps(&mut host, &[selection, selection]).is_err());
}

/// Two effects of one strip keep their own windows and sequences through the browser's read, and
/// the read consumes neither.
///
/// Test value: red if the read gives one effect's window to the other, or acknowledges a reader.
#[test]
fn selected_observation_reads_keep_same_track_owner_windows_and_sequences() {
    use protocol::ParameterChannel as Channel;
    const QUANTUM: u32 = 128;
    let mut host = same_track_observation_host(QUANTUM);
    let compressor = TapAt {
        track_index: 0,
        rack: RACK_INSERTS,
        effect_index: 0,
        tap_id: 1,
        channels: Channel::Both,
    };
    let gate = TapAt {
        effect_index: 1,
        ..compressor
    };
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), Ok(()));
    assert_eq!(observe(&mut host, 0, 1, 1, 1, 3, true), Ok(()));
    for block in 0..3 {
        feed_and_render_channels(&mut host, block, 0.75, 0.125);
    }

    let reader_state = |host: &mut AudioWorkletEngineHost, tap: &TapAt| {
        let (effect, index) = resolve_observation(host, tap).expect("resolved tap");
        let reader = &host
            .test_observation_handle(effect)
            .expect("observation handle")
            .readers[index];
        (
            reader.read().expect("published window"),
            reader.consumed_sequence(),
        )
    };
    let (compressor_window, compressor_consumed) = reader_state(&mut host, &compressor);
    let (gate_window, gate_consumed) = reader_state(&mut host, &gate);
    assert_eq!(compressor_window.blocks, 2);
    assert_eq!(gate_window.blocks, 3);
    assert_ne!(compressor_window.left, compressor_window.right);
    assert_ne!(gate_window.left, gate_window.right);
    assert_eq!(compressor_consumed, 0);
    assert_eq!(gate_consumed, 0);

    let rows =
        read_taps(&mut host, &[compressor, gate]).expect("same-track two-effect selected read");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].window, Some(row_window(compressor_window)));
    assert_eq!(rows[1].window, Some(row_window(gate_window)));
    assert_eq!(rows[0].left, Some(compressor_window.left));
    assert_eq!(rows[0].right, Some(compressor_window.right));
    assert_eq!(rows[1].left, Some(gate_window.left));
    assert_eq!(rows[1].right, Some(gate_window.right));

    assert_eq!(reader_state(&mut host, &compressor).1, compressor_consumed);
    assert_eq!(reader_state(&mut host, &gate).1, gate_consumed);
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

/// Arm (`armed`) or disarm the tap `tap_id` of the effect at `effect` in rack `rack` of the strip
/// at `track`, by the stable key the fixture's plan gives those positions (see [`key_at`]).
fn observe(
    host: &mut AudioWorkletEngineHost,
    track: u32,
    rack: u8,
    effect: u32,
    tap_id: u32,
    window_blocks: u32,
    armed: bool,
) -> Result<(), Vec<String>> {
    let edit = observe_edit_at(host, track, rack, effect, tap_id, window_blocks, armed);
    apply_observe(host, &[edit])
}

/// The `Observe` edit [`observe`] sends; a disarm names the arm the host has bound now.
fn observe_edit_at(
    host: &mut AudioWorkletEngineHost,
    track: u32,
    rack: u8,
    effect: u32,
    tap_id: u32,
    window_blocks: u32,
    armed: bool,
) -> protocol::SessionEdit {
    let (strip, effect) = key_at(host, track, rack, effect);
    let edit = observe_edit(
        core::str::from_utf8(&strip).expect("an ASCII strip ID"),
        rack,
        core::str::from_utf8(&effect).expect("an ASCII effect ID"),
        tap_id,
        window_blocks,
        armed,
    );
    with_bound_tag(host, edit)
}

/// One `Observe` edit of the tap named by its stable key: the strip's ID, the console slot's or
/// insert's ID per `rack`, and the tap's ID. A disarm names no arm (tag `0`) until [`tagged`] or
/// [`with_bound_tag`] gives it one (#2087).
fn observe_edit(
    strip_id: &str,
    rack: u8,
    effect_id: &str,
    tap_id: u32,
    window_blocks: u32,
    armed: bool,
) -> protocol::SessionEdit {
    let id = |id: &str| session::StableId::parse(id).expect("a stable ID");
    let section = match rack {
        RACK_INSERTS => controls::TapSection::Inserts(id(effect_id)),
        RACK_CONSOLE => controls::TapSection::Console(id(effect_id)),
        other => panic!("rack {other} names no effect section"),
    };
    protocol::SessionEdit::Observe {
        strip_id: id(strip_id),
        section,
        tap_id,
        armed,
        window_blocks,
        arm_tag: 0,
    }
}

/// `edit` with the arm tag `arm_tag` (#2087).
fn tagged(edit: protocol::SessionEdit, arm_tag: u64) -> protocol::SessionEdit {
    match edit {
        protocol::SessionEdit::Observe {
            strip_id,
            section,
            tap_id,
            armed,
            window_blocks,
            ..
        } => protocol::SessionEdit::Observe {
            strip_id,
            section,
            tap_id,
            armed,
            window_blocks,
            arm_tag,
        },
        other => other,
    }
}

/// A disarm `edit` naming the arm of its tap the host has bound now (`0` when none is), as a
/// subscriber that holds the arm names it (#2087); an arm unchanged.
fn with_bound_tag(
    host: &mut AudioWorkletEngineHost,
    edit: protocol::SessionEdit,
) -> protocol::SessionEdit {
    let protocol::SessionEdit::Observe {
        strip_id,
        section,
        tap_id,
        armed: false,
        ..
    } = &edit
    else {
        return edit;
    };
    let key = controls::TapKey {
        strip: strip_id.clone(),
        section: section.clone(),
        tap_id: *tap_id,
    };
    let arm_tag = host
        .test_control_mut()
        .control
        .observation_bindings()
        .binding(&key)
        .map_or(0, |binding| binding.arm_tag);
    tagged(edit, arm_tag)
}

/// [`observe`] naming the tap by its stable key, whatever the plan that renders calls its
/// position. Test support only: every caller is in a `test-support` module.
#[cfg(feature = "test-support")]
fn observe_key(
    host: &mut AudioWorkletEngineHost,
    strip_id: &str,
    rack: u8,
    effect_id: &str,
    tap_id: u32,
    window_blocks: u32,
    armed: bool,
) -> Result<(), Vec<String>> {
    let edit = observe_edit(strip_id, rack, effect_id, tap_id, window_blocks, armed);
    let edit = with_bound_tag(host, edit);
    apply_observe(host, &[edit])
}

/// A request ID above every one a test sent before it: the replay cache refuses an ID at or below
/// the highest it has seen (`REPLAY_EXPIRED`), so the helpers that send frames on a test's behalf
/// ([`apply_observe`], [`designate_master`]) and a test that mixes them with its own frames draw
/// from this one monotone sequence.
fn next_request_id() -> u64 {
    use core::sync::atomic::{AtomicU64, Ordering};
    static REQUEST: AtomicU64 = AtomicU64::new(1 << 40);
    REQUEST.fetch_add(1, Ordering::Relaxed)
}

/// Send `edits` in one `SESSION_TRANSACTION_APPLY` at the committed revision through the browser's
/// control entry, as an app arms and disarms a tap since #1589c deleted the observation records.
/// `Ok` when the core applied the transaction, else the codes of its diagnostics, or `result N`
/// for a control call that failed with result `N` (a full observation queue is `result 6`,
/// backpressure).
fn apply_observe(
    host: &mut AudioWorkletEngineHost,
    edits: &[protocol::SessionEdit],
) -> Result<(), Vec<String>> {
    use protocol::{
        CommandPayload, DecodeScratch, DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec,
        RequestId, SessionRevision, TypedCommandFrame,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(next_request_id()).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(
                    host.committed_revision(),
                )),
                payload: CommandPayload::SessionTransactionApply(edits),
            },
            &mut bytes,
        )
        .expect("an observe frame");
    let staged = u32::try_from(length).expect("frame length");
    host.control_request_mut(staged)
        .expect("the control staging")[..length]
        .copy_from_slice(&bytes[..length]);
    let result = host.control_command(staged);
    if result != RESULT_OK {
        return Err(vec![format!("result {result}")]);
    }
    let mut fields = [0_u16; 64];
    match codec
        .decode_typed_response(
            host.control_response(),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a transaction response")
    {
        DecodedTypedResponseFrame::Success { .. } => Ok(()),
        DecodedTypedResponseFrame::NonOk { payload, .. } => Err(payload
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.to_string())
            .collect()),
    }
}

/// The armed-tap mask of the insert `insert_id` of strip `strip_id` on the plan that renders, as
/// its taps' slots report it (#1585 D6), or `None` when the plan has no such insert. Test support
/// only: every caller is in a `test-support` module.
#[cfg(feature = "test-support")]
fn armed_mask(host: &mut AudioWorkletEngineHost, strip_id: &str, insert_id: &str) -> Option<u32> {
    let reader = host
        .observation_index()?
        .find(strip_id, LiveEffectRack::Inserts, insert_id)?
        .handle?;
    let companions = host
        .render
        .ready
        .as_mut()?
        .plan
        .active_attachment_mut::<RenderCompanions>()?;
    let handle = companions.effect_observations.get(reader as usize)?;
    Some(
        handle
            .readers
            .iter()
            .enumerate()
            .filter(|(_, reader)| reader.read_state().is_some_and(|state| state.armed))
            .fold(0, |mask, (tap, _)| mask | (1_u32 << tap)),
    )
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
    // capacity error.
    assert_eq!(
        observe(&mut host, 1, 1, 0, 1, 2, true),
        Err(vec!["observation.tap.unknown".to_owned()])
    );

    for track in [0_u32, 2] {
        assert_eq!(observe(&mut host, track, 1, 0, 1, 2, true), Ok(()));
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
    assert_eq!(observe(&mut undesignated, 0, 1, 0, 1, 2, true), Ok(()));
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
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, true), Ok(()));

    let selection = TapAt {
        track_index: 0,
        rack: RACK_INSERTS,
        effect_index: 0,
        tap_id: 1,
        channels: protocol::ParameterChannel::Both,
    };
    // The tap's own cell, read without consuming it.
    let cell_first_sample = |host: &mut AudioWorkletEngineHost| {
        let (effect, tap) = resolve_observation(host, &selection).expect("resolved selection");
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
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 2, false), Ok(()));
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

    // Leg 1: re-subscribe after the unsubscribe. Render applies the arm at the next block.
    let resubscribed = host.status().next_absolute_sample;
    assert_eq!(observe(&mut host, 0, 1, 0, 1, 8, true), Ok(()));
    assert!(
        cell_first_sample(&mut host).is_some_and(|first| first < resubscribed),
        "a stale window exists to leak"
    );
    drive(&mut host, &mut block, resubscribed, 12);

    // Leg 2: re-arm an armed tap with another window, the SDK `update()` path: the disarm of the
    // old window and the arm of the new one in one transaction, the core's one re-arm.
    let rearmed = host.status().next_absolute_sample;
    let rearm = [
        observe_edit_at(&mut host, 0, 1, 0, 1, 8, false),
        observe_edit_at(&mut host, 0, 1, 0, 1, 4, true),
    ];
    assert_eq!(apply_observe(&mut host, &rearm), Ok(()));
    assert!(
        cell_first_sample(&mut host).is_some_and(|first| first < rearmed),
        "the armed tap's old window predates the re-arm"
    );
    drive(&mut host, &mut block, rearmed, 8);
}

/// Issue #143 E1/E12, native leg: the observation timeline's determinism digest.
///
/// The native twin of `direct-oracle.mjs`'s `runObservationTimeline`. Both legs read this exact
/// fixture file, run this exact twelve-block timeline, and their digests are asserted equal to one
/// pin -- so a change to the audio makes them move together, which is the point.
///
/// Two runs, one timeline: with every declared tap armed and the master designated, and with no
/// frame sent at all. Since #1590 D5 both plans bind every declared tap. They must render
/// **identical bits**. That is E1 on the browser ABI, and it is checked here rather than asserted
/// about.
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
    type Reply = (u16, Vec<String>);
    type Run = (String, f32, Option<f32>, f32, u64, Option<(Reply, Reply)>);
    let run = |observe: bool| -> Run {
        let options = WebBootOptions {
            source_ring_frames: QUANTUM,
            live_control_meter_blocks: u64::from(WINDOW_BLOCKS),
            ..boot_options(QUANTUM)
        };
        let mut host = AudioWorkletEngineHost::boot(document.as_bytes(), options).expect("boot");
        // The oracle designates the master by the committed `master.bin` frame in its observing
        // leg (#1689 D6); this leg sends the same frame.
        if observe {
            let frame = master_designation_frame("track", 1, 1);
            host.control_request_mut(frame.len() as u32)
                .expect("the control staging")[..frame.len()]
                .copy_from_slice(&frame);
            assert_eq!(host.control_command(frame.len() as u32), RESULT_OK);
        }
        assert_eq!(host.live_control_tracks().len(), 1);
        assert!(
            host.resources().observation_retained_bytes > 0,
            "every plan binds the compressor's declared tap (#1590 D5)"
        );
        assert_eq!(host.set_meter_lease(true), RESULT_OK);

        // The oracle sends the same committed `Observe` frames (#1589c) through the same entry,
        // and reads each reply's status word.
        let [unknown_tap, arm, disarm] = observation_timeline_frames();
        let send = |host: &mut AudioWorkletEngineHost, frame: &[u8]| {
            host.control_request_mut(frame.len() as u32)
                .expect("the control staging")[..frame.len()]
                .copy_from_slice(frame);
            assert_eq!(host.control_command(frame.len() as u32), RESULT_OK);
            reply_status(host.control_response())
        };
        let replies = observe.then(|| {
            let unknown_tap = send(&mut host, &unknown_tap.1);
            let subscribed = send(&mut host, &arm.1);
            (unknown_tap, subscribed)
        });

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
        if observe {
            let _ = send(&mut host, &disarm.1);
        }
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
        (hex, armed, master, disarmed, window_samples, replies)
    };

    let (observed, armed, master, disarmed, window, replies) = run(true);
    let (unobserved, quiet, quiet_master, _, _, _) = run(false);
    let (unknown_tap, subscribed) = replies.expect("the observing leg's replies");

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
    // The statuses the oracle pins (`expected.json`'s `observationTimeline`), with the codes.
    assert_eq!(
        unknown_tap,
        (
            protocol::StatusCode::ValidationFailed as u16,
            vec!["observation.tap.unknown".to_owned()]
        )
    );
    assert_eq!(subscribed, (protocol::StatusCode::Ok as u16, Vec::new()));
    assert_eq!(quiet, 0.0, "no arm means no reading");
    assert_eq!(quiet_master, None, "and no master reading either");

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
            bypass_observation: Some(0.0),
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

    // Issue #1918: a signed gain-change tap keeps its sign through the clamp, and the strip's
    // reduction slot takes only a cut's depth, so a boost never reads as reduction.
    let signed = ObservationDescriptor {
        kind: ObservationKind::GainChangeDb,
        fold: ObservationFold::PeakSigned,
        minimum: -24.0,
        maximum: 24.0,
        ..decibels
    };
    assert_eq!(observed_decibels(signed, -6.0), -6.0, "not clamped at 0");
    assert_eq!(
        strip_reduction_decibels(signed, -6.0, 2.0),
        6.0,
        "a cut's depth"
    );
    assert_eq!(
        strip_reduction_decibels(signed, 6.0, 6.0),
        0.0,
        "a boost is no reduction"
    );
    assert_eq!(
        strip_reduction_decibels(signed, f32::INFINITY, 0.0),
        0.0,
        "+inf is no cut"
    );
    assert_eq!(
        strip_reduction_decibels(signed, 0.0, f32::NAN),
        0.0,
        "NaN is no cut"
    );
}

/// The identity fixture with three sources declared out of canonical order (issue #207).
///
/// `zeta` is declared first and `alpha` last, so a query that reported *declaration* order rather
/// than the normalized order would be visible here rather than hidden by an already-sorted
/// fixture. The shapes are all distinct -- a source is mono or stereo, so the lengths differ -- so a
/// query that read the wrong row is visible too. The one track points at `mid`, which is neither the first nor
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
        source("alpha", 1, u64::from(quantum) * 2),
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
            channel_count: 1,
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
        [1, 2, 1]
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

fn strip_options(meter_blocks: u64) -> WebBootOptions {
    WebBootOptions {
        source_ring_frames: STRIP_QUANTUM * 4,
        live_control_meter_blocks: meter_blocks,
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
        strip_boot(&bus, strip_options(meter_blocks)),
        strip_boot(&reference, strip_options(meter_blocks)),
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
            Ok(())
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
    let address = |track_index| TapAt {
        track_index,
        rack: RACK_INSERTS,
        effect_index: 0,
        tap_id: 1,
        channels: protocol::ParameterChannel::Both,
    };
    assert_eq!(
        key_at(&a, STRIP_BUS, RACK_INSERTS, 0),
        (b"bus".to_vec(), b"comp".to_vec())
    );
    let bus_read =
        read_taps(&mut a, &[address(STRIP_BUS)]).expect("a selected read of the bus's tap");
    let track_read = read_taps(&mut b, &[address(0)]).expect("the same read on the track");
    assert_eq!(bus_read[0].status, track_read[0].status);
    assert_eq!(
        bus_read[0].left.map(f32::to_bits),
        track_read[0].left.map(f32::to_bits)
    );
    assert_eq!(
        bus_read[0].right.map(f32::to_bits),
        track_read[0].right.map(f32::to_bits)
    );
    assert!(
        read_taps(&mut a, &[address(STRIP_BUS + 1)]).is_err(),
        "one past the last strip"
    );

    assert_eq!(
        observe(&mut a, STRIP_BUS, RACK_INSERTS, 0, 1, WINDOW as u32, false),
        Ok(())
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

/// The gain-reduction word a tap's own latest window reads as: the same
/// `strip_reduction_decibels` that `poll_meters` folds into the strip's slot.
fn tap_gain_reduction(host: &mut AudioWorkletEngineHost, effect_index: u32) -> f32 {
    let rows = read_taps(
        host,
        &[TapAt {
            track_index: 0,
            rack: RACK_INSERTS,
            effect_index,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        }],
    )
    .expect("a selected read of the tap");
    let (Some(left), Some(right)) = (rows[0].left, rows[0].right) else {
        panic!("the tap has no published window: {:?}", rows[0].status);
    };
    let ready = host.test_render();
    let tap = ready.shape.effect_instances[effect_index as usize]
        .as_ref()
        .filter(|instance| instance.reader.is_some())
        .expect("observation handle")
        .descriptor
        .observations[0];
    strip_reduction_decibels(tap, left, right)
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
    // One transaction arms both taps; a tap's queue holds one record until a block drains it.
    let arms = [0, 1].map(|effect| observe_edit_at(&mut host, 0, RACK_INSERTS, effect, 1, 2, true));
    assert_eq!(apply_observe(&mut host, &arms), Ok(()));
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
        Ok(())
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
        Ok(())
    );
    render_window(&mut host);
    assert!(host.poll_meters() >= 1, "a frame follows the withdrawal");
    assert_eq!(
        strip_gain_word(&host, 0).to_bits(),
        0,
        "no tap is armed, so the slot is +0.0 at once"
    );
}

/// Issue #1213 gate 6 (VERIFY-2 M6, #1208's condition): the bus's `master` row designates it
/// (#1689: the boot word is gone), and its true-peak limiter insert is driven over its -6 dB
/// ceiling; with the limiter's tap armed at the bus's index, the header reports a master reading
/// and the master word equals the bus's word, nonzero.
///
/// Test value: red if the master resolves to tracks only, or the master reading is taken from
/// another strip's word or never folded for a bus.
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
    let mut host = strip_boot(&bus, strip_options(WINDOW));
    designate_master(&mut host, "bus");
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
        Ok(())
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
}

/// Issue #1213 gate 7, as #1589c leaves it: the block that applies a bus observation arm renders
/// without allocating or freeing. The arm is an `Observe` edit through the core
/// (`control_command` is control work and allocates by design), so only the render is measured.
///
/// Test value: red if the render that drains a strip-sized observation arm allocates. The bus
/// gain-reduction fold runs in `poll_meters`, outside the measured closure; #1209's
/// `bus_meters_render_and_poll_without_allocating` covers it.
#[test]
fn a_bus_observation_arm_renders_without_allocating() {
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
    assert_eq!(
        observe(&mut a, STRIP_BUS, RACK_INSERTS, 0, 1, 2, true),
        Ok(()),
        "the bus observation is armed"
    );
    let planes: Vec<[Vec<f32>; 2]> = (0..3).map(|feeder| strip_planes(feeder, 2)).collect();
    for (feeder, planes) in planes.iter().enumerate() {
        submit_strip_source(&mut a, &format!("t{feeder}"), 2, planes);
    }
    let (render, allocations, deallocations) =
        crate::ffi::allocation_counter::measured(|| a.render_next());
    assert_eq!(render, RESULT_OK);
    assert_eq!(a.observation_armed_taps(), 1, "the block applied the arm");
    assert_eq!(allocations, 0, "the render allocated");
    assert_eq!(deallocations, 0, "the render freed");
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
        strip_options(0),
    );
    let mut zero = strip_boot(
        &subnormal_route_document([0.0, 0.0, 0.5, 0.0]),
        strip_options(0),
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

fn companion_request(capture: CompanionCapture) -> Option<host_core::SpectrumPreparation> {
    let target = SpectrumTarget::TrackPostMatrix("eq0".into());
    match capture {
        CompanionCapture::None => None,
        CompanionCapture::Single => Some(host_core::SpectrumPreparation::Single(
            SpectrumCaptureRequest {
                target,
                channels: SpectrumChannels::Stereo,
                maximum_capture_bytes: u64::MAX,
            },
        )),
        CompanionCapture::Collection => Some(host_core::SpectrumPreparation::Collection(
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
    let (host, _, _) = host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
        &compiled,
        &caps,
        &live_control_request(options, 128).expect("request"),
        controls::live_lanes(compiled.normalized_model()).into(),
        None,
        None,
        &host_core::LiveValuesSuperseded::new(),
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
        let record = Arc::new(Mutex::new(BootRecord::default()));
        let request = companion_request(capture);
        let preparer = WebRuntimePreparer::new(options).recording(Arc::clone(&record));
        let superseded = host_core::LiveValuesSuperseded::new();
        let check = |host: &mut PreparedHost,
                     handles: &host_core::HostLiveControlHandles,
                     captures: Option<&PreparedSpectrumCaptures>,
                     at: &str| {
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
            // #1596 D3: the preparer returns the captures to the session, as the C ABI's does.
            let shape = match captures {
                None => CompanionCapture::None,
                Some(PreparedSpectrumCaptures::Single(_)) => CompanionCapture::Single,
                Some(PreparedSpectrumCaptures::Collection(_)) => CompanionCapture::Collection,
            };
            assert_eq!(shape, capture, "{capture:?} {at}: the branch's capture");
        };

        let (mut boot, boot_handles, boot_captures) = preparer
            .prepare(&compiled, &caps, lanes, request.as_ref(), None, &superseded)
            .unwrap_or_else(|failure| panic!("{capture:?} boot: {failure:?}"));
        check(&mut boot, &boot_handles, boot_captures.as_ref(), "boot");
        // The attachment is charged in the plan's rows (the control plane builds its plan
        // resource row from this report): the same preparation without the attachment, plus the
        // attachment's own box and vector buffers, in both plan rows and in the largest allocation.
        let (reference, _, _) = host_core::prepare_host_runtime_with_live_lanes_and_spectrum(
            &compiled,
            &caps,
            &live_control_request(options, 128).expect("request"),
            lanes,
            request.as_ref(),
            None,
            &superseded,
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
            edit_ramps: &edit_ramps,
            ..host_core::SuccessorBase::new(&boot.inventory, model, 1)
        };
        let (mut successor, successor_handles, successor_captures) = preparer
            .prepare(
                &compiled,
                &caps,
                lanes,
                request.as_ref(),
                Some(base),
                &superseded,
            )
            .unwrap_or_else(|failure| panic!("{capture:?} successor: {failure:?}"));
        check(
            &mut successor,
            &successor_handles,
            successor_captures.as_ref(),
            "successor",
        );
        // Every successor carries the timeline's command queue (#1641 D7), which is overhead
        // only; a carried ring also brings its PCM payload.
        assert!(
            successor.report.carried_source_total_bytes
                > successor.report.carried_source_overhead_bytes,
            "{capture:?}: the successor prepares the unchanged source vacant"
        );
    }
}

// Issue #1290a: the browser host applies a structural transaction while it plays. The control half
// prepares the successor from the committed model, publishes it and returns; the render half
// adopts it at a block boundary. The native gates run the one instance on one thread, the
// message handler's place between render calls (decision 17, "single instance first"), through
// the production control entry (`control_request_mut` and `control_command`), as the page's
// `control` message does (#1562b).
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
        fn boot(model: &SessionModel, options: WebBootOptions) -> Self {
            let document = canonical_session_json(model).expect("canonical session");
            let host = AudioWorkletEngineHost::boot(document.as_bytes(), options)
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

        /// Run one frame through the production control entry, as the page's `control` message
        /// does. Returns the entry's result word and what it left to read (the response frame
        /// after an ok result, else the refusal's diagnostic text).
        fn entry(&mut self, bytes: &[u8]) -> (u32, Vec<u8>) {
            let staging = self
                .host
                .control_request_mut(bytes.len() as u32)
                .expect("the control staging");
            staging[..bytes.len()].copy_from_slice(bytes);
            let result = self.host.control_command(bytes.len() as u32);
            (result, self.host.control_response().to_vec())
        }

        /// Apply a structural transaction through the control entry and decode its response.
        fn apply(&mut self, edits: &[SessionEdit], request: u64) -> Outcome {
            let revision = self.host.committed_revision();
            let (result, response) = self.entry(&frame(edits, request, revision));
            assert_eq!(result, RESULT_OK, "the control plane answers the frame");
            outcome(&response)
        }

        /// The structure epoch a `CONTROL_LIST_GET` page carries now: the control list's own,
        /// read through the production control entry as a client reads it.
        fn list_epoch(&mut self, request: u64) -> u64 {
            let codec = ProtocolCodec::default();
            let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
            let length = codec
                .encode_command_frame_into(
                    &TypedCommandFrame {
                        request_id: RequestId::new(request).expect("nonzero request"),
                        expected_revision: ExpectedRevision::Any,
                        payload: CommandPayload::ControlListGet(&protocol::ControlListRequest {
                            structure_epoch: 0,
                            after_handle: 0,
                            limit: 1,
                            owner: None,
                        }),
                    },
                    &mut bytes,
                )
                .expect("a control list frame");
            bytes.truncate(length);
            let (result, response) = self.entry(&bytes);
            assert_eq!(
                result, RESULT_OK,
                "the control plane answers the list frame"
            );
            let mut fields = [0_u16; 64];
            let DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::ControlListPage(page),
                ..
            } = codec
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("a control list page")
            else {
                panic!("expected a control list page")
            };
            page.structure_epoch
        }

        fn epoch(&self) -> u64 {
            self.host.active_plan_epoch_for_test().expect("a plan")
        }

        /// Whether a committed replacement waits for the block that adopts it.
        fn pending(&self) -> bool {
            self.host.test_control().control.newest_plan_epoch() != self.epoch()
        }

        fn watermark(&mut self) -> WebPlanWatermark {
            assert_eq!(self.host.read_watermark(), RESULT_OK);
            *self.host.watermark()
        }

        /// Steps until the plan applies the committed revision, at least one block: a rebuild
        /// that restarts a strip is a duck-swap, adopted at its `S`, not at the next block
        /// (#1324a). The adoption block's first sample.
        fn step_through_adoption(&mut self) -> u64 {
            let revision = self.host.committed_revision();
            let mut blocks = 0;
            loop {
                assert!(
                    blocks < 64,
                    "revision {revision} is applied within 64 blocks"
                );
                self.step();
                blocks += 1;
                if self.watermark().revision >= revision {
                    break;
                }
            }
            let first = self.watermark().first_sample;
            assert_eq!(
                first,
                (self.block - 1) * u64::from(QUANTUM),
                "the last block adopted it"
            );
            first
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
    fn gap_free_acceptance() {
        let model = gate_model();
        let committed = with_muted_track(&model, ADDED);

        let mut fresh = Rig::boot(&committed, options());
        for _ in 0..BLOCKS {
            fresh.step();
        }

        let mut rig = Rig::boot(&model, options());
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
    }

    /// Gate 1. Test value: red if the apply reboots internally (fresh rings or state: the filters
    /// and ramps restart and the bits differ), if the successor's clock restarts at zero, or if
    /// `apply` blocks until adoption (the plan that renders would already be the successor).
    #[test]
    fn a_rebuild_swaps_without_a_gap() {
        gap_free_acceptance();
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
        let mut without = Rig::boot(&model, options);
        for _ in 0..BLOCKS {
            without.step();
        }
        let mut with = Rig::boot(&model, options);
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
            let (result, response) = rig.entry(&bytes);
            assert_ne!(result, RESULT_OK, "a truncated frame is refused");
            format!("{result} {}", String::from_utf8_lossy(&response))
        });
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
                let revision = rig.host.committed_revision();
                let (result, response) =
                    rig.entry(&frame(std::slice::from_ref(&edit), 1, revision));
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
            exact_retained_report_total(Rig::boot(&model, options()).host.resources());
        let tight = WebBootOptions {
            maximum_memory_bytes: fits_one_plan,
            ..options()
        };
        // The budget boots the session (the boot is inclusive of its own total) ...
        Rig::boot(&model, tight);
        // ... and refuses a second plan beside it.
        let refused = assert_refusal_changes_nothing(tight, |rig, model| {
            let revision = rig.host.committed_revision();
            let (result, response) = rig.entry(&frame(&add_muted_track(model, ADDED), 1, revision));
            assert_eq!(
                result, RESULT_COMPILE_REJECTED,
                "a replacement that does not fit beside the running plan is refused, typed"
            );
            String::from_utf8_lossy(&response).into_owned()
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
        let mut rig = Rig::boot(&model, roomy);
        assert!(matches!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied { .. }
        ));
    }

    /// Gate 6, in the form a host can drive without a browser: what is still bound to the boot
    /// plan's indices describes the successor from the adoption block on. The observed effects'
    /// strip indices and the strip order are the plan's tables, and the source table is the
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
        let mut rig = Rig::boot(&model, options());
        let ids = |host: &AudioWorkletEngineHost| {
            host.live_control_tracks()
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
        };
        // The strip index of each observed effect in the plan's tables, which the meter frame's
        // gain-reduction fold reads.
        let binding_tracks = |host: &AudioWorkletEngineHost| {
            host.test_render()
                .shape
                .observation_tracks
                .iter()
                .copied()
                .filter(|&strip| strip != u32::MAX)
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
        // An observation record is admitted against the successor, as a live value is, and names
        // its tap by key (#1677): "track"'s compressor, which sits at index 1 in the successor
        // (the added "a-track", with a compressor of the same ID, took index 0), is the one that
        // arms. Nothing is armed on the plan that renders.
        let key = |host: &mut AudioWorkletEngineHost, strip: &str| {
            observe_key(host, strip, RACK_INSERTS, "comp", 1, 2, true)
        };
        // A key the successor does not have is refused with its typed code and queues nothing,
        // whichever plan has it: "gone" has no strip anywhere, "a-track" is the successor's.
        let unknown = |code: &str| Err(vec![code.to_owned()]);
        assert_eq!(
            key(&mut rig.host, "gone"),
            unknown("observation.address.unknown")
        );
        assert_eq!(
            observe_key(&mut rig.host, "track", RACK_INSERTS, "gone", 1, 2, true),
            unknown("observation.address.unknown")
        );
        assert_eq!(
            observe_key(&mut rig.host, "track", RACK_INSERTS, "comp", 9, 2, true),
            unknown("observation.tap.unknown")
        );
        assert_eq!(
            key(&mut rig.host, "track"),
            Ok(()),
            "an observation record between the commit and the adoption block is admitted"
        );
        assert_eq!(
            rig.host.observation_armed_taps(),
            0,
            "the plan that renders is not armed by a record addressed to its successor"
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
        assert_eq!(
            (
                armed_mask(&mut rig.host, "track", "comp"),
                armed_mask(&mut rig.host, "a-track", "comp")
            ),
            (Some(1), Some(0)),
            "the record armed the compressor it named, not the one that took its old index"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
        for _ in 0..4 {
            rig.step();
        }
        let tap = TapAt {
            track_index: 1,
            rack: RACK_INSERTS,
            effect_index: 0,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        };
        assert_eq!(
            read_taps(&mut rig.host, &[tap]).map(|rows| rows[0].armed),
            Ok(true),
            "the read reads the successor's armed reader"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
    }

    /// #1323 gate 2 on the browser: the #1285 removal of a muted track behind a true-peak limiter
    /// keeps A's latency; `declare_discontinuity`, the host method
    /// `miso_engine_web_v1_declare_discontinuity` runs, then publishes the committed session with no
    /// floors. After one render the plan's latency is the natural one, and from that block every
    /// block equals a fresh boot of the committed model fed from the frame the carried source stood
    /// at. The revision does not move. A wrong handle is refused.
    ///
    /// Test value: red if the browser entry does not reach the control plane's declaration (the
    /// floors survive, no plan is pending and the remaining tracks stay late), or if the successor
    /// carries any state but the source (the filters differ from a fresh boot's).
    #[test]
    fn a_declared_discontinuity_resets_the_floors_on_the_browser() {
        const DECLARED_AT: u64 = 10;
        let mut model = gate_model();
        let limited = &mut model.tracks[0];
        limited.fader.left_mute = true;
        limited.fader.right_mute = true;
        limited.inserts.effects.push(session::Effect {
            id: StableId::parse("lim").expect("insert ID"),
            identity: session::EffectIdentity::Native {
                effect_id: StableId::parse("miso.true-peak-limiter").expect("limiter id"),
            },
            quality: session::EffectQuality::Normal,
            bypass: false,
            link_mode: session::LinkMode::Maximum,
            params: Vec::new(),
            sidechain: session::SidechainDeclaration::None,
        });
        let edits = [
            SessionEdit::RemoveRoute {
                route_id: model.routes[0].id.clone(),
            },
            SessionEdit::RemoveTrack {
                track_id: model.tracks[0].id.clone(),
            },
        ];
        let latency = |rig: &Rig| rig.host.test_control().queries.resources().latency_samples;
        assert_eq!(
            crate::ffi::miso_engine_web_v1_declare_discontinuity(u32::MAX),
            RESULT_INVALID_ARGUMENT
        );
        let mut rig = Rig::boot(&model, options());
        let floored = latency(&rig);
        assert!(floored > 0, "the limiter delays A");
        for _ in 0..APPLY_BEFORE {
            rig.step();
        }
        assert!(matches!(
            rig.apply(&edits, 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        while rig.block < DECLARED_AT {
            rig.step();
        }
        assert_eq!(
            latency(&rig),
            floored,
            "the removal keeps A's latency (#1285)"
        );
        let revision = rig.host.committed_revision();
        assert_eq!(rig.host.declare_discontinuity(), RESULT_OK);
        assert!(rig.pending(), "the declaration published a plan");
        while rig.block < BLOCKS {
            rig.step();
        }
        assert_eq!(rig.host.committed_revision(), revision);

        let document = canonical_session_json(
            rig.host
                .test_control()
                .control
                .committed_session()
                .normalized_model(),
        )
        .expect("canonical session");
        let committed = parse_session_json(&document).expect("the committed model parses");
        let mut fresh = Rig::boot(&committed, options());
        let natural = latency(&fresh);
        assert!(natural < floored, "B's natural latency is lower");
        assert_eq!(
            latency(&rig),
            natural,
            "the plan runs at its natural latency"
        );
        // The fresh boot reads the PCM from the frame the carried source stood at.
        let quantum = u64::from(QUANTUM);
        let offset = DECLARED_AT * quantum;
        let expected: Vec<Vec<u32>> = (0..BLOCKS - DECLARED_AT)
            .map(|block| {
                let lane = |lane: u64| {
                    (block * quantum..(block + 1) * quantum)
                        .map(|frame| strip_sample(0, lane, frame + offset))
                        .collect::<Vec<f32>>()
                };
                let planes = [lane(0), lane(1)];
                let views: [&[f32]; 2] = [&planes[0], &planes[1]];
                assert_eq!(
                    fresh.host.submit_source(
                        SOURCE,
                        1,
                        block * quantum,
                        48_000,
                        &views,
                        QUANTUM,
                        false
                    ),
                    RESULT_OK
                );
                assert_eq!(fresh.host.render_next(), RESULT_OK);
                fresh
                    .host
                    .output_pcm()
                    .expect("output")
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect()
            })
            .collect();
        assert!(
            expected.iter().flatten().any(|bits| *bits != 0),
            "the fresh boot plays"
        );
        assert_eq!(
            first_difference(&rig.bits[DECLARED_AT as usize..], &expected),
            None,
            "from the declaration on, the output is a fresh boot's"
        );
        assert_eq!(rig.host.service(), RESULT_OK);
    }

    /// Root ruling B16-R9's bookkeeping on the browser: a plain session seek declares the
    /// discontinuity in the same call, so the host keeps and counts the successor it publishes as
    /// it keeps a declaration's. From #1285's floors (a muted track behind a true-peak limiter
    /// removed), swapped or still waiting for its block, the plain seek publishes one successor;
    /// the host holds that plan's tables for its adoption block, counts one blocking rebuild,
    /// names the successor's meter header from the rendering plan, and admits a command
    /// submission against it before the swap. The next block adopts it and the host stays ready.
    /// Mirrors the C ABI's `an_accepted_plain_session_seek_publishes_one_successor_and_applies`.
    ///
    /// Test value: red if the browser's plain seek reaches the control plane without the host's
    /// plan bookkeeping (`run_control`): the audio and the latency are the same, but the
    /// successor has no tables, is not counted and is not named, and the first submission before
    /// its swap fails the host (`web.internal.pending_shape`).
    #[test]
    fn a_plain_session_seek_keeps_and_counts_its_successor_on_the_browser() {
        for swap in [true, false] {
            let mut model = gate_model();
            let limited = &mut model.tracks[0];
            limited.fader.left_mute = true;
            limited.fader.right_mute = true;
            limited.inserts.effects.push(session::Effect {
                id: StableId::parse("lim").expect("insert ID"),
                identity: session::EffectIdentity::Native {
                    effect_id: StableId::parse("miso.true-peak-limiter").expect("limiter id"),
                },
                quality: session::EffectQuality::Normal,
                bypass: false,
                link_mode: session::LinkMode::Maximum,
                params: Vec::new(),
                sidechain: session::SidechainDeclaration::None,
            });
            let edits = [
                SessionEdit::RemoveRoute {
                    route_id: model.routes[0].id.clone(),
                },
                SessionEdit::RemoveTrack {
                    track_id: model.tracks[0].id.clone(),
                },
            ];
            let mut rig = Rig::boot(&model, options());
            for _ in 0..APPLY_BEFORE {
                rig.step();
            }
            assert!(matches!(
                rig.apply(&edits, 1),
                Outcome::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ));
            if swap {
                // The removed strip ramps out, then the removal is adopted at `S` (#1325 D3).
                let mut blocks = 0;
                while rig.pending() {
                    rig.step();
                    blocks += 1;
                    assert!(blocks <= 64, "swap {swap}: the removal was adopted");
                }
            } else {
                assert!(
                    rig.pending(),
                    "swap {swap}: the removal waits for its block"
                );
            }
            let counted = rig.host.blocking_rebuilds();
            let newest = |rig: &Rig| rig.host.test_control().control.newest_plan_epoch();
            let before = newest(&rig);

            const FRAME: u64 = 1_280;
            assert_eq!(rig.host.session_seek(2, FRAME, None), RESULT_OK);
            let successor = newest(&rig);
            assert_eq!(successor, before + 1, "swap {swap}: one successor");
            assert_eq!(
                rig.host.blocking_rebuilds(),
                counted + 1,
                "swap {swap}: the successor is counted"
            );
            // The rendering plan names the successor's header (#1332 S4: the successor's own, which
            // its plan carries), checked at the adoption below.
            let named = rig.host.meter_successor_address();
            assert_ne!(
                named, 0,
                "swap {swap}: the rendering plan names a successor"
            );
            assert_eq!(rig.host.status().state, STATE_READY, "swap {swap}");

            let quantum = u64::from(QUANTUM);
            let lane = |lane: u64| {
                (FRAME..FRAME + quantum)
                    .map(|frame| strip_sample(0, lane, frame))
                    .collect::<Vec<f32>>()
            };
            let planes = [lane(0), lane(1)];
            let views: [&[f32]; 2] = [&planes[0], &planes[1]];
            assert_eq!(
                rig.host
                    .submit_source(SOURCE, 2, FRAME, 48_000, &views, QUANTUM, false),
                RESULT_OK
            );
            assert_eq!(rig.host.render_next(), RESULT_OK, "swap {swap}");
            assert_eq!(
                rig.epoch(),
                successor,
                "swap {swap}: the next block adopts it"
            );
            assert_eq!(
                core::ptr::from_ref(rig.host.meter_header()).addr(),
                named,
                "swap {swap}: the header named before the block is the one it adopted"
            );
            assert_eq!(rig.host.status().state, STATE_READY, "swap {swap}");
            assert_eq!(rig.host.service(), RESULT_OK);
        }
    }

    /// #1310 on a paused page (batch 15a review fold B1): eight structural edits with no render
    /// between them each commit, each superseding the candidate before it, as on the C ABI. Each
    /// candidate carries its own tables (#1332 S4), so a superseded one's drop with it and no
    /// edit waits for room; then the next block adopts the newest plan, which holds all eight
    /// edits, and from that block on the output equals a fresh boot of the committed model.
    ///
    /// Test value: red if a paused structural edit is refused (with `BACKPRESSURE`, as before
    /// the batch 15a fold), or if the adopted plan misses an edit.
    #[test]
    fn paused_structural_edits_all_commit_and_the_newest_renders_as_a_fresh_boot() {
        const EDITS: u64 = 8;
        let model = gate_model();
        let mut rig = Rig::boot(&model, options());
        for _ in 0..APPLY_BEFORE {
            rig.step();
        }
        let base = rig.host.committed_revision();
        let mut results = Vec::new();
        for edit in 0..EDITS {
            let committed = rig
                .host
                .test_control()
                .control
                .committed_session()
                .normalized_model()
                .clone();
            let edits = add_muted_track(&committed, &format!("a-muted-{edit}"));
            let (result, response) =
                rig.entry(&frame(&edits, edit + 1, rig.host.committed_revision()));
            results.push(result);
            // A client drains the committed events as it goes, so the event lane never refuses.
            while rig.host.control_event(EVENT_LANE_RELIABLE) == RESULT_OK
                && !rig.host.control_response().is_empty()
            {}
            if result == RESULT_OK {
                assert_eq!(
                    outcome(&response),
                    Outcome::Applied {
                        path: EditPath::Rebuild,
                        revision: base + edit + 1
                    },
                    "edit {edit}"
                );
            }
        }
        assert_eq!(
            results,
            vec![RESULT_OK; EDITS as usize],
            "every paused structural edit commits"
        );
        assert_eq!(rig.epoch(), 0, "nothing rendered: the boot plan still runs");
        let document = canonical_session_json(
            rig.host
                .test_control()
                .control
                .committed_session()
                .normalized_model(),
        )
        .expect("canonical session");
        let committed = parse_session_json(&document).expect("the committed model parses");
        let mut fresh = Rig::boot(&committed, options());
        for _ in 0..BLOCKS {
            fresh.step();
        }
        let before = rig.watermark();
        while rig.block < BLOCKS {
            rig.step();
        }
        assert!(!rig.pending(), "render adopted the newest plan");
        assert_eq!(rig.host.service(), RESULT_OK);
        assert_eq!(
            rig.host.live_control_tracks().len(),
            model.tracks.len() + EDITS as usize,
            "the adopted plan holds every edit's track"
        );
        let watermark = rig.watermark();
        assert_eq!(watermark.revision, base + EDITS);
        assert_eq!(
            watermark.superseded_count,
            before.superseded_count + EDITS - 1,
            "the first seven complete as superseded"
        );
        assert_eq!(
            first_difference(
                &rig.bits[APPLY_BEFORE as usize..],
                &fresh.bits[APPLY_BEFORE as usize..]
            ),
            None,
            "from the adoption block on, every block equals a fresh boot of the committed model"
        );
        assert!(
            fresh.bits[APPLY_BEFORE as usize..]
                .iter()
                .any(|block| block.iter().any(|word| f32::from_bits(*word) != 0.0)),
            "the compared blocks are not silent"
        );
    }

    /// #1310 (root ruling B15-Q4) on the browser, the host that admits observation records: a
    /// subscription acked against a candidate that a later structural edit supersedes arms the
    /// replacement and publishes windows there. The replacement carries "track"'s compressor from
    /// the plan that renders, which never saw the record, so the core queues the record again on
    /// the replacement, whose first block arms the tap in its slot.
    ///
    /// Test value: red if the record is lost with the superseded candidate (the carried tap keeps
    /// the running plan's unarmed state, the read reports it unarmed and no window publishes).
    #[test]
    fn a_subscription_queued_on_a_superseded_candidate_publishes_on_its_replacement() {
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
        let mut later = source.clone();
        later.id = StableId::parse("b-source").expect("source ID");
        let first = [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];
        let second = [SessionEdit::UpsertSource { source: later }];
        let mut rig = Rig::boot(&model, options());
        rig.step();
        assert!(matches!(
            rig.apply(&first, next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        assert_eq!(
            observe_key(&mut rig.host, "track", RACK_INSERTS, "comp", 1, 2, true),
            Ok(()),
            "the subscription is acked against the pending candidate"
        );
        assert!(matches!(
            rig.apply(&second, next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        rig.step();
        assert_eq!(rig.epoch(), 2, "the replacement is adopted");
        assert_eq!(
            (
                armed_mask(&mut rig.host, "track", "comp"),
                armed_mask(&mut rig.host, "a-track", "comp")
            ),
            (Some(1), Some(0)),
            "the replacement's tables hold the subscription"
        );
        for _ in 0..6 {
            rig.step();
        }
        // The old track is strip 1 of the replacement.
        let tap = TapAt {
            track_index: 1,
            rack: RACK_INSERTS,
            effect_index: 0,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        };
        assert_eq!(key_at(&rig.host, 1, RACK_INSERTS, 0).0, b"track");
        let read = read_taps(&mut rig.host, &[tap]).expect("a read");
        assert!(
            read[0].window.is_some(),
            "the subscription publishes windows on the replacement: {:?}",
            read[0].status
        );
    }

    /// A sidechain routed from strip `track`'s own input: a prepared change of an instance (a new
    /// graph edge), which restarts it. (A link change is live since #1371, so it carries.)
    fn own_input_sidechain(track: &session::StableId) -> session::SidechainDeclaration {
        session::SidechainDeclaration::Routed(session::Sidechain {
            source: session::RouteSource::Track {
                track_id: track.clone(),
                tap: session::SendTap::Input,
            },
            port_id: session::StableId::parse("sidechain-in").expect("a port ID"),
        })
    }

    /// #1698 on the browser: a subscription acked against a candidate that restarted "track"'s
    /// compressor (its sidechain changed), which a later structural edit supersedes by putting the
    /// compressor back as the plan that renders has it. The replacement now carries the
    /// compressor from the plan that renders, whose tap never saw the record, so the core queues
    /// the record again on the replacement, as for a compressor the candidate itself carried.
    ///
    /// Test value: red if the record forwarding serves only a tap the candidate carried, and not
    /// one the replacement carries from the plan that renders across a candidate that restarted
    /// it: the carried tap keeps the running plan's unarmed state, its slot reads unarmed and no
    /// window publishes.
    #[test]
    fn a_subscription_on_a_candidate_that_restarted_the_tap_publishes_on_a_replacement_that_reverts_it()
     {
        let mut model =
            parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
                .expect("accepted command fixture");
        model.tracks[0].inserts.effects[0] = observed_compressor_insert();
        let original = model.tracks[0].inserts.effects[0].clone();
        let mut restarted = original.clone();
        restarted.sidechain = own_input_sidechain(&model.tracks[0].id);
        let put = |effect: session::Effect| {
            [SessionEdit::PutTrackEffect {
                track_id: model.tracks[0].id.clone(),
                rack_name: session::RackName::Inserts,
                final_position: 0,
                effect,
            }]
        };
        let mut rig = Rig::boot(&model, options());
        rig.step();
        assert!(matches!(
            rig.apply(&put(restarted), next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        assert_eq!(
            observe_key(&mut rig.host, "track", RACK_INSERTS, "comp", 1, 2, true),
            Ok(()),
            "the subscription is acked against the pending candidate"
        );
        assert!(matches!(
            rig.apply(&put(original), next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        // The replacement inherits the first candidate's duck-swap `S` (#1324a, #1325 D6).
        rig.step_through_adoption();
        assert_eq!(rig.epoch(), 2, "the replacement is adopted");
        assert_eq!(
            armed_mask(&mut rig.host, "track", "comp"),
            Some(1),
            "the replacement's tables hold the subscription"
        );
        for _ in 0..6 {
            rig.step();
        }
        let tap = TapAt {
            track_index: 0,
            rack: RACK_INSERTS,
            effect_index: 0,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        };
        assert_eq!(key_at(&rig.host, 0, RACK_INSERTS, 0).0, b"track");
        let read = read_taps(&mut rig.host, &[tap]).expect("a read");
        assert!(
            read[0].window.is_some(),
            "the subscription publishes windows on the replacement: {:?}",
            read[0].status
        );
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

    /// One published meter frame: the header and the frame's words.
    type Published = (WebMeterHeader, Vec<f32>);

    /// Drive `blocks` more blocks with the lease on, polling after the blocks whose zero-based
    /// index leaves `phase` modulo `every`; every frame published, in order.
    fn poll_cadence(rig: &mut Rig, blocks: u64, every: u64, phase: u64) -> Vec<Published> {
        let mut frames = Vec::new();
        for _ in 0..blocks {
            let index = rig.block;
            rig.step();
            if index % every == phase && rig.host.poll_meters() > 0 {
                frames.push((*rig.host.meter_header(), rig.host.meter_frame().to_vec()));
            }
        }
        frames
    }

    /// The frames a fresh boot of `model` publishes over `blocks` blocks, polled after each, keyed
    /// by the window's first sample: what the swapped host must publish for the same window.
    fn fresh_windows(model: &SessionModel, blocks: u64) -> Vec<Published> {
        let mut rig = Rig::boot(model, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        poll_cadence(&mut rig, blocks, 1, 0)
    }

    fn window_of(frames: &[Published], first_sample: u64) -> &Published {
        frames
            .iter()
            .find(|(header, _)| header.first_sample == first_sample)
            .unwrap_or_else(|| panic!("no window opens at sample {first_sample}"))
    }

    /// The meter delivery across a swap that keeps the strips (#1381b): the predecessor's meters
    /// carry, so the delivery epoch the page sees does not change and the windows follow each
    /// other without a gap.
    ///
    /// Test value: red if the swap block's hook does not pair the meter consumers (the successor's
    /// meters would sit on observers the retiring plan owns and no window would ever fold again),
    /// or if the swap restarts the delivery epoch or discards the windows waiting for a poll.
    #[test]
    fn meter_delivery_carries_through_a_swap_that_keeps_the_strips() {
        let model = gate_model();
        let mut source = model.sources[0].clone();
        source.id = StableId::parse("a-unused").expect("source ID");
        let mut rig = Rig::boot(&model, options());
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
    }

    /// The meter header's structure epoch is the control list's (#1562b): a rebuild that leaves
    /// every address and descriptor alone (a strip's `delay_samples` value) keeps it, and the
    /// header of the plan that renders reads the number `CONTROL_LIST_GET` pages carry, read here
    /// through the control entry after a rebuild and after a live commit that moves the epoch (a
    /// VCA added).
    ///
    /// Test value: red if the header's epoch advances with the revision or the plan epoch rather
    /// than with the list (a client would read the list again at every commit), stays the boot's
    /// when the list moved (the strip order of a frame would be named by a stale list), or is not
    /// stamped by a live commit (`stamp_structure_epoch`): the header then keeps the epoch of the
    /// last rebuild while the page's moved on.
    #[test]
    fn the_headers_structure_epoch_is_the_control_lists() {
        let model = gate_model();
        let mut rig = Rig::boot(&model, options());
        let boot = rig.host.meter_header().structure_epoch;
        assert_eq!(boot, 1, "the boot list is the first structure");
        let delay = SessionEdit::SetControl {
            expected_effect: None,
            address: controls::ControlAddress {
                owner: controls::Owner::Strip(model.tracks[0].id.clone()),
                section: controls::Section::Builtins,
                control: 11,
                lane: session::ParameterChannel::Both,
            },
            value: 64.0,
            ramp_samples: None,
        };
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(std::slice::from_ref(&delay), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            },
            "a delay is a rebuild"
        );
        // The delay restarts the track: a duck-swap, adopted at its `S` (#1324a).
        rig.step_through_adoption();
        rig.step();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(
            rig.host.meter_header().structure_epoch,
            boot,
            "a value-only rebuild keeps the list's structure"
        );
        assert_eq!(
            rig.apply(&add_muted_track(&model, ADDED), 2),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 2
            }
        );
        assert_eq!(
            rig.host.meter_header().structure_epoch,
            boot,
            "the plan that renders keeps its own list until the replacement is adopted"
        );
        rig.step();
        assert_eq!(rig.epoch(), 2);
        assert_eq!(rig.host.meter_header().structure_epoch, boot + 1);
        let page = rig.list_epoch(3);
        assert_eq!(
            rig.host.meter_header().structure_epoch,
            page,
            "after a rebuild the header is the page's"
        );

        // A commit that rides the live lanes can move the list's epoch too (a VCA added: a new
        // owner and new addresses), and stamps the plan that renders, which has no successor
        // waiting. The header then reads what a `CONTROL_LIST_GET` page carries.
        let vca = SessionEdit::UpsertVca {
            vca: session::Vca {
                id: StableId::parse("late-vca").expect("VCA ID"),
                fader: session::DualMonoFader {
                    left_db: -3.0,
                    right_db: -3.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![model.tracks[0].id.clone()],
            },
        };
        // The two rebuilds above queued their events; a client drains them as it goes.
        for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
            while rig.host.control_event(lane) == RESULT_OK
                && !rig.host.control_response().is_empty()
            {}
        }
        let before_vca = rig.list_epoch(4);
        let revision = rig.host.committed_revision();
        assert_eq!(
            rig.apply(std::slice::from_ref(&vca), 5),
            Outcome::Applied {
                path: EditPath::Live,
                revision: revision + 1
            },
            "a VCA added is a live commit"
        );
        assert!(
            !rig.pending(),
            "no replacement waits: the live commit rode the rendering plan"
        );
        let after_vca = rig.list_epoch(6);
        assert!(
            after_vca > before_vca,
            "a VCA added moves the list's structure epoch"
        );
        assert_eq!(
            rig.host.meter_header().structure_epoch,
            after_vca,
            "the rendering plan's header is stamped with the live commit's epoch"
        );
    }

    /// A swap that adds a strip publishes the successor's frame (#1290d1): the frame travels with
    /// the plan, so the window the adoption block closes is described by a `3(T + S) + 3` frame for
    /// the plan's own strips, bit for bit what a fresh boot of the committed model publishes for
    /// that window, in the same stream (generation, sequence, no counted loss).
    ///
    /// Test value: red if the frame keeps the boot's size (the poll would publish nothing, or a
    /// frame whose words name other strips), if the header keeps the predecessor's strip counts,
    /// if the swap restarts the stream, or if the added strip's words land at the wrong index.
    #[test]
    fn a_strip_count_change_publishes_the_successors_frame_from_the_adoption_block() {
        let model = gate_model();
        let committed = with_muted_track(&model, ADDED);
        // Windows close after the odd blocks. The transaction runs before block 5, so the block
        // that adopts the successor is the one that closes the window opening at sample 512.
        let apply_before = 5;
        let reference = fresh_windows(&committed, 8);

        let mut rig = Rig::boot(&model, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        let before = poll_cadence(&mut rig, apply_before, 1, 0);
        let last = before.last().expect("the boot plan published windows");
        assert_eq!(last.0.track_count, 9);
        assert_eq!(last.1.len(), 3 * 9 + 3);
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        // The candidate's header is named by the one that renders, before the adoption block: the
        // header the candidate's plan carries (#1332 S4), which the adoption block publishes.
        // Full width: the `u32` word reads 0 for an address above 4 GiB on a native host, so the
        // check is on the address the commit set beside it.
        let named = rig.host.meter_successor_address();
        assert_ne!(named, 0);
        {
            let ready = rig.host.test_render();
            assert_eq!(ready.shape.meter_header.retired, 0);
            // The strip order changed, so the list's structure epoch did: the rendering plan keeps
            // the epoch of its own list until the adoption block (#1562b).
            assert_eq!(
                ready.shape.meter_header.structure_epoch,
                last.0.structure_epoch
            );
        }
        let after = poll_cadence(&mut rig, 1, 1, 0);
        assert_eq!(rig.epoch(), 1);
        assert_eq!(
            core::ptr::from_ref(rig.host.meter_header()).addr(),
            named,
            "the rendering plan named the candidate's own header"
        );
        assert_eq!(
            after[0].0.structure_epoch,
            last.0.structure_epoch + 1,
            "the adoption block's frame names the successor's list"
        );
        assert_eq!(
            after.len(),
            1,
            "the first block after adoption publishes: the successor's frame is not too small"
        );
        let (header, frame) = &after[0];
        assert_eq!(header.track_count, 10);
        assert_eq!(header.submix_count, 0);
        assert_eq!(header.master_track_plus_one, 0);
        assert_eq!(
            frame.len(),
            3 * 10 + 3,
            "a frame sized for the successor's strips"
        );
        assert_eq!(header.first_sample, 512);
        assert_eq!(
            header.reserved[0], last.0.reserved[0],
            "the stream's generation is the plan's predecessor's"
        );
        assert_eq!(
            header.sequence,
            last.0.sequence + 1,
            "the sequence continues"
        );
        assert_eq!(header.reserved[1] >> 32, 0, "no window was counted lost");
        let (expected_header, expected) = window_of(&reference, 512);
        assert_eq!(header.reserved[1], expected_header.reserved[1]);
        assert_eq!(
            frame.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
            expected
                .iter()
                .map(|word| word.to_bits())
                .collect::<Vec<_>>(),
            "the window is what a fresh boot of the committed model publishes"
        );
        assert!(
            frame.iter().any(|word| *word != 0.0),
            "the frame is not silence"
        );

        // The header the swap retired says so and names the one that replaced it; the adopted
        // header carries no successor (nothing waits). The adopting block parks the retired
        // tables for one block (#1332 S5): nothing is in the hand-back ring yet, so no service
        // step on another thread could free the header the page still reads in this block.
        assert_eq!(rig.host.test_control().retired_shapes.queued_len(), 0);
        let ready = rig.host.test_render();
        let retired = ready
            .parked
            .as_ref()
            .expect("the adopting block parks the retired tables");
        assert_eq!(retired.meter_header.retired, 1);
        assert_eq!(retired.meter_header.track_count, 9);
        assert_ne!(retired.successor_address, 0);
        assert_eq!(
            retired.successor_address,
            core::ptr::from_ref(&ready.shape.meter_header).addr(),
            "the retired header names the adopted one"
        );
        assert_ne!(
            retired.successor_address,
            core::ptr::from_ref(&retired.meter_header).addr()
        );
        assert_eq!(ready.shape.meter_header.retired, 0);
        assert_eq!(ready.shape.meter_header.successor_pointer, 0);
        assert_eq!(
            rig.host.meter_successor_address(),
            0,
            "the adopted header names no successor"
        );

        // And the stream goes on in the successor's frame.
        let later = poll_cadence(&mut rig, 3, 1, 0);
        assert!(!later.is_empty());
        // The next block handed the retired tables back; the service step frees them, on the
        // control side (#1332 S5).
        assert!(rig.host.test_render().parked.is_none());
        assert_eq!(rig.host.test_control().retired_shapes.queued_len(), 1);
        assert_eq!(rig.host.service(), RESULT_OK);
        assert_eq!(rig.host.test_control().retired_shapes.queued_len(), 0);
        for (header, frame) in &later {
            assert_eq!(header.track_count, 10);
            assert_eq!(frame.len(), 33);
            let (_, expected) = window_of(&reference, header.first_sample);
            assert_eq!(frame, expected);
        }
    }

    /// A window the predecessor closed before the swap and the poll finds after it belongs to the
    /// strip it was carried to (#1290d1). Removing the first strip shifts every survivor's meter
    /// handle down by one, so the windows queued under the old handles must still be accepted:
    /// published in the successor's smaller frame with the stream's sequence, nothing counted.
    ///
    /// Test value: red if the poll matches a queued or pending window by the meter's current
    /// handle only (the whole window is dropped and counted, nine times), or if the carried strips
    /// are matched by position instead of identity (the frame would name other strips' peaks).
    #[test]
    fn a_window_closed_before_a_handle_shift_is_published_by_its_carried_strip() {
        // A zero mute ramp and no compensation line: phase 1 of the removal (#1325 D3) is the one
        // block `q`, so a transaction before block 5 is adopted at block 6, the swap this row
        // needs after a closed, unpolled window. The removed strip is muted throughout, so its
        // ramp would sound nothing.
        let mut model = gate_model();
        model.control_smoothing = Some(session::ControlSmoothing {
            mute_ms: 0.0,
            ..session::CONTROL_SMOOTHING_DEFAULT
        });
        let with_added = with_muted_track(&model, ADDED);
        let reference = fresh_windows(&model, 10);

        let mut rig = Rig::boot(&with_added, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        // Windows close after the odd blocks; polling after the even blocks leaves each window
        // queued for one block. The transaction runs before block 5, so block 6 adopts the
        // successor and is polled: the window that closed after block 5 is polled after the swap.
        let before = poll_cadence(&mut rig, 5, 2, 0);
        let last = before.last().expect("the boot plan published windows");
        assert_eq!(last.0.track_count, 10);
        assert_eq!(last.0.first_sample, 256, "the window of blocks 2 and 3");
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(
                &[
                    SessionEdit::RemoveRoute {
                        route_id: StableId::parse(&format!("{ADDED}-main")).expect("route ID"),
                    },
                    SessionEdit::RemoveTrack {
                        track_id: StableId::parse(ADDED).expect("track ID"),
                    },
                ],
                1
            ),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        let after = poll_cadence(&mut rig, 5, 2, 0);
        assert_eq!(rig.epoch(), 1);
        let watermark = rig.watermark();
        assert_eq!(
            (watermark.revision, watermark.first_sample),
            (base + 1, 6 * u64::from(QUANTUM)),
            "the removal is adopted at block 6, the first polled block after the closed window"
        );
        assert_eq!(
            after.len(),
            2,
            "the windows of blocks 4-5 and 6-7 are published"
        );
        let (header, frame) = &after[0];
        assert_eq!(
            header.first_sample, 512,
            "the window the predecessor closed"
        );
        assert_eq!(header.track_count, 9);
        assert_eq!(frame.len(), 3 * 9 + 3);
        assert_eq!(header.sequence, last.0.sequence + 1);
        assert_eq!(header.reserved[0], last.0.reserved[0]);
        assert_eq!(
            header.reserved[1] >> 32,
            0,
            "no window was counted lost to the handle shift"
        );
        let (_, expected) = window_of(&reference, 512);
        assert_eq!(
            frame.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
            expected
                .iter()
                .map(|word| word.to_bits())
                .collect::<Vec<_>>(),
            "each survivor's peaks are at its own index of the smaller frame"
        );
        assert!(frame.iter().any(|word| *word != 0.0));
        let (next, next_frame) = &after[1];
        assert_eq!(next.sequence, header.sequence + 1);
        assert_eq!(next.reserved[1] >> 32, 0);
        assert_eq!(next_frame, &window_of(&reference, 768).1);
    }

    /// The same stream across a swap that adds a strip in front, polled every second block with
    /// the polls aligned to the windows (#1290d1, gate 7): nothing is counted lost, the sequence
    /// continues, and every window is the fresh boot's.
    ///
    /// Test value: red if the swap restarts or loses a window the aligned cadence leaves nothing
    /// to lose (the stream's generation, sequence or loss count moving across the swap).
    #[test]
    fn an_aligned_poll_every_second_block_loses_nothing_across_an_added_strip() {
        let model = gate_model();
        let committed = with_muted_track(&model, ADDED);
        let reference = fresh_windows(&committed, 12);

        let mut rig = Rig::boot(&model, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        let before = poll_cadence(&mut rig, 6, 2, 1);
        let last = before.last().expect("the boot plan published windows");
        let base = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(&add_muted_track(&model, ADDED), 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        let after = poll_cadence(&mut rig, 6, 2, 1);
        assert_eq!(rig.epoch(), 1);
        assert_eq!(rig.host.committed_revision(), base + 1);
        assert_eq!(after.len(), 3);
        let mut sequence = last.0.sequence;
        for (header, frame) in &after {
            sequence += 1;
            assert_eq!(header.sequence, sequence);
            assert_eq!(header.reserved[0], last.0.reserved[0]);
            assert_eq!(header.reserved[1] >> 32, 0, "no counted loss");
            assert_eq!(header.track_count, 10);
            assert_eq!(frame, &window_of(&reference, header.first_sample).1);
        }
    }

    /// An acknowledged subscription survives a swap (#1381b): the tap's slot, which holds its arm
    /// state (#1585 D6), moves with its effect to the successor's effect slot, which a strip added
    /// in front shifts, and the successor's reader is the carried tap's, so its windows continue.
    ///
    /// Test value: red if the swap block's hook does not pair the observation readers (the tap
    /// would read as unarmed and the page's subscription would silently stop after every swap), or
    /// if it maps the readers to the wrong effect slot after the added strip shifts them, if
    /// the selected read reads readers it took at preparation and not the active plan's (#1586a:
    /// after the carry those are the successor's fresh slots, which nothing publishes to), or if
    /// it resolves them through another plan's index than the one that renders (#1589 R2: the
    /// newest committed plan's, between a rebuild's commit and its adoption).
    #[test]
    fn an_armed_tap_and_its_windows_follow_the_effect_through_a_swap() {
        let mut model =
            parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
                .expect("accepted command fixture");
        model.tracks[0].inserts.effects[0] = observed_compressor_insert();
        let mut rig = Rig::boot(&model, options());
        rig.step();
        assert_eq!(
            observe(&mut rig.host, 0, RACK_INSERTS, 0, 1, 2, true),
            Ok(())
        );
        for _ in 0..8 {
            rig.step();
        }
        assert_eq!(rig.host.observation_armed_taps(), 1);
        let address = |track_index| TapAt {
            track_index,
            rack: RACK_INSERTS,
            effect_index: 0,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        };
        let before = read_taps(&mut rig.host, &[address(0)]).expect("a read")[0]
            .window
            .expect("the armed tap published a window")
            .sequence;

        let base = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(&add_muted_track(&model, ADDED), next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision
            } if revision == base + 1
        ));
        // The successor waits for its block: the read resolves through the rendering plan's index.
        // The tap is named by its key, the committed model's strip 1 (the added track took 0).
        assert!(
            read_taps(&mut rig.host, &[address(1)]).expect("a read before the adoption")[0]
                .window
                .is_some(),
            "the rendering plan's tap reads its window before the adoption"
        );
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
        let after = read_taps(&mut rig.host, &[address(1)]).expect("a read at the shifted strip")
            [0]
        .window
        .expect("the carried tap kept publishing")
        .sequence;
        assert!(after > before, "the windows continue ({before} to {after})");
    }

    /// #1590 D5: every boot binds every declared tap, as the C ABI does, so the browser serves
    /// the `Observe` edit and lists `OBSERVATION_READ` and capability bit 15 with no boot word.
    ///
    /// Test value: red if a boot binds no queue for a declared tap (the edit refused
    /// `observation.unavailable`) or if the session withholds the read from its capabilities.
    #[test]
    fn observe_and_its_read_are_served_on_every_boot() {
        let model = observed_model();
        let mut rig = Rig::boot(&model, options());
        let arm = SessionEdit::Observe {
            strip_id: model.tracks[0].id.clone(),
            section: controls::TapSection::Inserts(StableId::parse("comp").expect("insert ID")),
            tap_id: 1,
            armed: true,
            window_blocks: 1,
            arm_tag: 0,
        };
        match rig.apply(&[arm], 1) {
            Outcome::Applied { .. } => {}
            Outcome::Refused(_, codes) => panic!("the Observe edit is refused {codes:?}"),
        }

        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(2).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Any,
                    payload: CommandPayload::CapabilitiesGet,
                },
                &mut bytes,
            )
            .expect("a capabilities frame");
        let (result, reply) = rig.entry(&bytes[..length]);
        assert_eq!(result, RESULT_OK);
        let mut fields = [0_u16; 256];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::Capabilities(capabilities),
            ..
        } = codec
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("a capabilities reply")
        else {
            panic!("a capabilities reply")
        };
        assert!(
            capabilities.supported_commands.contains(&0x000f),
            "OBSERVATION_READ listed"
        );
        assert!(
            capabilities.flags.0 & protocol::CapabilityFlags::OBSERVATION.0 != 0,
            "capability bit 15"
        );
    }

    fn meter_loss(rig: &Rig) -> u64 {
        rig.host.test_render().meters.loss_count()
    }

    /// Telemetry is dropped only with a count. The windows the predecessor closed before a swap
    /// that adds a strip and nobody polled before it cannot be described by the successor's frame
    /// (the added strip did not exist to measure them): the poll that finds them drops them with a
    /// count, the swap does not zero what was counted before it, the next frame published is the
    /// successor's, and it reports the loss.
    ///
    /// Test value: red if the windows the successor cannot describe vanish without a count (a page
    /// would see a silent gap), if the swap zeroes the loss count, or if a frame is published
    /// that mixes the two plans' strips.
    #[test]
    fn windows_an_added_strip_cannot_describe_are_counted_and_the_next_frame_is_the_successors() {
        let model = gate_model();
        let mut rig = Rig::boot(&model, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert!(poll_blocks(&mut rig, 6) > 0);
        let published = *rig.host.meter_header();
        // Windows rendered and not polled when the swap comes, and a loss counted before it.
        for _ in 0..4 {
            rig.step();
        }
        let waiting = rig.host.test_render_mut().meters.output_windows_queued() as u64;
        assert!(waiting > 0, "the render side closed windows nobody polled");
        rig.host
            .test_render_mut()
            .meters
            .test_only_set_loss_count(5);
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
        assert_eq!(meter_loss(&rig), 5, "the swap keeps the count");
        assert_eq!(
            rig.host.meter_header().sequence,
            published.sequence,
            "nothing was published meanwhile"
        );

        let mut first = None;
        for _ in 0..8 {
            rig.step();
            if rig.host.poll_meters() > 0 {
                first = Some((*rig.host.meter_header(), rig.host.meter_frame().to_vec()));
                break;
            }
        }
        let (header, frame) = first.expect("the successor's window is published");
        assert_eq!(header.track_count, 10);
        assert_eq!(frame.len(), 33);
        assert!(
            header.first_sample >= 10 * 128,
            "a window from the adoption block on, not one of the predecessor's"
        );
        assert!(header.reserved[1] & METER_VALID_LOSS != 0);
        // Exact: the 5 counted before; per strip of the predecessor, its two windows the
        // successor's group cannot pair (each strip's older head dropped twice); the `waiting`
        // output windows they span, stale against the successor's first window; and one gap, since
        // the published stream ends at block 6 and the next frame starts at block 10.
        let predecessor_strips = (model.tracks.len() + model.submixes.len()) as u64;
        assert_eq!(
            header.reserved[1] >> 32,
            5 + 2 * predecessor_strips + waiting + 1,
            "the windows left behind are counted ({waiting} waiting and 5 before)",
        );
        assert_eq!(header.sequence, published.sequence + 1);
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
            let record = Arc::clone(&rig.host.test_control().record);
            let poisoner = std::thread::spawn(move || {
                let _guard = record.lock().expect("the record is free");
                panic!("poison the record");
            });
            assert!(poisoner.join().is_err());
            let revision = rig.host.committed_revision();
            let (result, response) = rig.entry(&frame(&add_muted_track(model, ADDED), 1, revision));
            assert_eq!(
                result, RESULT_COMPILE_REJECTED,
                "the rebuild is refused, typed"
            );
            String::from_utf8_lossy(&response).into_owned()
        });
        assert!(refused.contains("web.apply.shape_unavailable"), "{refused}");
    }

    /// The model of the observation gates: the command fixture with the observed compressor
    /// (insert `comp`, tap 1) as the first track's only insert.
    fn observed_model() -> SessionModel {
        let mut model =
            parse_session_json(include_str!("../tests/browser-v1/command-session.json"))
                .expect("accepted command fixture");
        model.tracks[0].inserts.effects[0] = observed_compressor_insert();
        model
    }

    fn comp_address() -> TapAt {
        TapAt {
            track_index: 0,
            rack: RACK_INSERTS,
            effect_index: 0,
            tap_id: 1,
            channels: protocol::ParameterChannel::Both,
        }
    }

    /// Every reliable event the control plane has queued, drained the way the worklet does, as
    /// `(revision, payload)` for the two this gate reads.
    fn drain_reliable(host: &mut AudioWorkletEngineHost) -> Vec<(u64, String)> {
        let codec = ProtocolCodec::default();
        let mut events = Vec::new();
        loop {
            assert_eq!(host.control_event(EVENT_LANE_RELIABLE), RESULT_OK);
            if host.control_response().is_empty() {
                return events;
            }
            let bytes = host.control_response().to_vec();
            let mut fields = vec![0_u16; 4_096];
            let decoded = codec
                .decode_typed_event(&bytes, &mut DecodeScratch::new(&mut fields))
                .expect("an event frame");
            let revision = decoded.header.revision.0;
            match decoded.payload {
                protocol::DecodedEventPayload::SessionCommitted(_) => {
                    events.push((revision, "committed".to_owned()));
                }
                protocol::DecodedEventPayload::ObservationEnded(ended) => {
                    for key in &ended.keys {
                        events.push((
                            revision,
                            format!(
                                "ended {} {:?} {} {}",
                                key.strip_id, key.section, key.effect_id, key.tap_id
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    /// #1584 D6b, restart: a subscribed compressor whose sidechain changes is prepared again (a
    /// rebuild with no carry), and its first block publishes the prepared arm in the tap's slot
    /// (#1585 R2). The tap reads armed, the first window opens at the adoption block's first
    /// sample, and the windows continue block after block.
    ///
    /// Test value: red if the successor's first block does not publish the prepared arm (the read
    /// would answer `Unarmed` and the page's subscription would silently stop at every restart of
    /// its effect, as `observation_armed_taps` reads zero).
    #[test]
    fn a_restarted_effects_subscription_keeps_publishing_windows_on_the_browser() {
        let mut model = observed_model();
        // The duck-swap's wait for `S` (#1324a) plays past the fixture source's last frame.
        model.sources[0].frames = 1 << 20;
        let mut rig = Rig::boot(&model, options());
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        rig.step();
        assert_eq!(
            observe(&mut rig.host, 0, RACK_INSERTS, 0, 1, 2, true),
            Ok(())
        );
        for _ in 0..8 {
            rig.step();
        }
        let before = read_taps(&mut rig.host, &[comp_address()]).expect("a read")[0]
            .window
            .expect("the armed tap published a window")
            .sequence;
        assert!(before > 1);

        let mut restarted = model.tracks[0].inserts.effects[0].clone();
        restarted.sidechain = own_input_sidechain(&model.tracks[0].id);
        let base = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(
                &[SessionEdit::PutTrackEffect {
                    track_id: model.tracks[0].id.clone(),
                    rack_name: session::RackName::Inserts,
                    final_position: 0,
                    effect: restarted,
                }],
                next_request_id()
            ),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision
            } if revision == base + 1
        ));
        // The link-mode change restarts the track: a duck-swap, adopted at its `S` (#1324a).
        let adoption_sample = rig.step_through_adoption();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(
            rig.host.observation_armed_taps(),
            1,
            "the successor's tables carry the subscription the host had acknowledged"
        );
        rig.step();
        let first = read_taps(&mut rig.host, &[comp_address()]).expect("a read")[0]
            .window
            .expect("the restarted tap published its first window");
        assert_eq!(
            first.first_sample, adoption_sample,
            "from the adoption block on"
        );
        assert_eq!(
            first.sequence, 1,
            "a restarted tap numbers its windows afresh"
        );
        for _ in 0..4 {
            rig.step();
        }
        let later = read_taps(&mut rig.host, &[comp_address()]).expect("a read")[0]
            .window
            .expect("the windows continue");
        assert_eq!(later.sequence, 3);
        assert!(rig.host.poll_meters() > 0);
        assert!(
            gain_reduction(&rig.host).0[0] > 0.0,
            "the meter frame's gain-reduction word follows the restarted tap"
        );
    }

    /// #1584 D6b, removal: the removed insert has no slot on the successor, the browser's read
    /// finds no tap there, and the event drain returns the commit and one `OBSERVATION_ENDED`
    /// that names the tap.
    ///
    /// Test value: red if the browser's drain loses the event (the page would keep a subscription
    /// whose effect is gone and never be told), or if a removed insert's tap stays armed in the
    /// successor's tables.
    #[test]
    fn a_removed_inserts_subscription_ends_with_an_event_and_has_no_slot() {
        let model = observed_model();
        let mut rig = Rig::boot(&model, options());
        rig.step();
        assert_eq!(
            observe(&mut rig.host, 0, RACK_INSERTS, 0, 1, 2, true),
            Ok(())
        );
        for _ in 0..4 {
            rig.step();
        }
        assert_eq!(rig.host.observation_armed_taps(), 1);
        assert!(
            drain_reliable(&mut rig.host).is_empty(),
            "nothing is queued yet"
        );

        let base = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(
                &[SessionEdit::RemoveTrackEffect {
                    track_id: model.tracks[0].id.clone(),
                    rack_name: session::RackName::Inserts,
                    effect_id: model.tracks[0].inserts.effects[0].id.clone(),
                }],
                next_request_id()
            ),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision
            } if revision == base + 1
        ));
        let events = drain_reliable(&mut rig.host);
        assert_eq!(
            events,
            vec![
                (base + 1, "committed".to_owned()),
                (base + 1, "ended track Inserts comp 1".to_owned()),
            ]
        );
        // The removal restarts the track: a duck-swap (#1324a).
        rig.step_through_adoption();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(rig.host.observation_armed_taps(), 0);
        assert!(
            read_taps(&mut rig.host, &[comp_address()]).is_err(),
            "the successor has no insert, so no slot to read"
        );
    }

    /// The `master` monitor row of strip `strip` (#1683), as `SET_CONTROL` addresses it.
    fn master_edit(strip: &str, on: bool) -> SessionEdit {
        SessionEdit::SetControl {
            address: controls::ControlAddress {
                owner: controls::Owner::Strip(StableId::parse(strip).expect("a strip ID")),
                section: controls::Section::Own,
                control: 4,
                lane: controls::Lane::Both,
            },
            value: if on { 1.0 } else { 0.0 },
            ramp_samples: None,
            expected_effect: None,
        }
    }

    /// The gain-reduction word of strip `strip` and the master's word, when the header says it
    /// is present.
    fn master_reading(host: &AudioWorkletEngineHost, strip: usize) -> (f32, Option<f32>) {
        let strips = host.live_control_tracks().len() + host.meter_header().submix_count as usize;
        let frame = host.meter_frame();
        let base = strips * 2 + 2;
        let master = (host.meter_header().master_gr_present == 1).then(|| frame[base + strips]);
        (frame[base + strip], master)
    }

    /// #1683 gate 1, #1689 gate 1: a session that boots empty gets its master through the core
    /// after boot, the one way to designate it (#1689 retired the boot word). A track with a
    /// compressor arrives by a rebuild; one `SET_CONTROL` of its `master` row designates it, a
    /// monitor write that spends no revision; the meter header names it and the master reading is
    /// that strip's own word, bit for bit. A track added before it by a later rebuild moves its
    /// index and the designation follows it by ID; a strip the session does not have is refused
    /// before commit; clearing the row makes the reading absent, never zero.
    ///
    /// Test value: red if the designation cannot be set after boot, is held by index (the reading
    /// would move to the added track), is not carried into a successor plan, reaches the meter
    /// header late or never, spends a revision, or a designation of an unknown strip is admitted.
    #[test]
    fn a_master_designated_after_an_empty_boot_reads_as_its_strips_own_word() {
        const WINDOW: u64 = 2;
        let full = parse_session_json(include_str!(
            "../../../fixtures/session/v1/observation-frame-shape.json"
        ))
        .expect("the observation fixture parses");
        let mut empty = full.clone();
        empty.sources.clear();
        empty.tracks.clear();
        empty.routes.clear();
        let options = WebBootOptions {
            source_ring_frames: QUANTUM * 4,
            live_control_meter_blocks: WINDOW,
            ..boot_options(QUANTUM)
        };
        let mut rig = Rig::boot(&empty, options);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 0);
        let track = |index: usize, id: &str| {
            let mut track = full.tracks[index].clone();
            let mut route = full.routes[index].clone();
            track.id = StableId::parse(id).expect("a track ID");
            route.id = StableId::parse(&format!("{id}-main")).expect("a route ID");
            let RouteSource::Track { track_id, .. } = &mut route.source else {
                panic!("the fixture routes a track")
            };
            *track_id = track.id.clone();
            [
                SessionEdit::UpsertTrack { track },
                SessionEdit::UpsertRoute { route },
            ]
        };
        let mut first = vec![SessionEdit::UpsertSource {
            source: full.sources[0].clone(),
        }];
        first.extend(track(0, "t0"));
        assert!(matches!(
            rig.apply(&first, next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        assert_eq!(rig.host.render_next(), RESULT_OK, "the rebuild is adopted");

        // An unknown strip is refused before commit, and nothing is designated.
        let revision = rig.host.committed_revision();
        assert!(matches!(
            rig.apply(&[master_edit("nobody", true)], next_request_id()),
            Outcome::Refused(..)
        ));
        assert_eq!(rig.host.meter_header().master_track_plus_one, 0);

        assert_eq!(
            rig.apply(&[master_edit("t0", true)], next_request_id()),
            Outcome::Applied {
                path: EditPath::Monitor,
                revision,
            },
            "a monitor write spends no revision"
        );
        assert_eq!(rig.host.meter_header().master_track_plus_one, 1);

        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert_eq!(
            observe_key(
                &mut rig.host,
                "t0",
                RACK_INSERTS,
                "comp",
                1,
                WINDOW as u32,
                true
            ),
            Ok(())
        );
        assert_eq!(rig.host.seek_source(SOURCE, 2, 0), RESULT_OK);
        let mut fed = 0;
        let mut feed = |host: &mut AudioWorkletEngineHost, blocks: u64| {
            for _ in 0..blocks {
                feed_and_render(host, 2, fed, 0.5);
                fed += 1;
            }
        };
        feed(&mut rig.host, 8 * WINDOW);
        assert!(rig.host.poll_meters() > 0, "a window closed");
        let (word, master) = master_reading(&rig.host, 0);
        assert!(word > 0.0, "the compressor reduces: {word}");
        assert_eq!(
            master.map(f32::to_bits),
            Some(word.to_bits()),
            "the master is t0's own word"
        );

        // A track whose ID sorts first arrives by a rebuild: t0 is strip 1 now, and stays the
        // master.
        assert!(matches!(
            rig.apply(&track(1, "a0"), next_request_id()),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        feed(&mut rig.host, 8 * WINDOW);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 2);
        assert!(rig.host.poll_meters() > 0, "a window closed");
        let (word, master) = master_reading(&rig.host, 1);
        assert!(word > 0.0, "t0 still reduces: {word}");
        assert_eq!(master.map(f32::to_bits), Some(word.to_bits()));

        // Cleared: absent, not zero.
        let revision = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&[master_edit("t0", false)], next_request_id()),
            Outcome::Applied {
                path: EditPath::Monitor,
                revision,
            }
        );
        assert_eq!(rig.host.meter_header().master_track_plus_one, 0);
        feed(&mut rig.host, 4 * WINDOW);
        assert!(rig.host.poll_meters() > 0, "a window closed");
        let (word, master) = master_reading(&rig.host, 1);
        assert!(word > 0.0);
        assert_eq!(master, None, "no designation reads absent");
    }

    /// #1581 D5: a swap that carries no strip meter (here the first strip of an empty boot)
    /// restarts the strip meters at its adoption block, and the output meter restarts there with
    /// them, so the frames after the swap publish.
    ///
    /// Test value: red if the output meter keeps the predecessor's window grid across a swap that
    /// restarts the strip meters (no master window would match a strip window again, and every
    /// frame after the swap would be dropped as a loss), if a retaken lease or a producer reset
    /// activates on the absolute grid rather than the adoption block's, or if the output window
    /// the swap left queued is read by the next poll instead of being discarded unread.
    #[test]
    fn an_uncarried_swap_restarts_the_output_meter_at_its_adoption_block() {
        const WINDOW: u64 = 3;
        let full = parse_session_json(include_str!(
            "../../../fixtures/session/v1/observation-frame-shape.json"
        ))
        .expect("the observation fixture parses");
        let mut empty = full.clone();
        empty.sources.clear();
        empty.tracks.clear();
        empty.routes.clear();
        let options = WebBootOptions {
            source_ring_frames: QUANTUM * 4,
            live_control_meter_blocks: WINDOW,
            ..boot_options(QUANTUM)
        };
        let mut rig = Rig::boot(&empty, options);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        // Four blocks: the master's window [0, 3) publishes and the next opens at block 3.
        for _ in 0..4 {
            assert_eq!(rig.host.render_next(), RESULT_OK);
            rig.host.poll_meters();
        }
        let quantum = u64::from(QUANTUM);
        assert_eq!(rig.host.meter_header().end_sample, WINDOW * quantum);
        // Three blocks unpolled: the window [3, 6) waits in the queue when the swap comes.
        for _ in 0..WINDOW {
            assert_eq!(rig.host.render_next(), RESULT_OK);
        }
        const ADOPTION: u64 = 4 + WINDOW;
        let edits = [
            SessionEdit::UpsertSource {
                source: full.sources[0].clone(),
            },
            SessionEdit::UpsertTrack {
                track: full.tracks[0].clone(),
            },
            SessionEdit::UpsertRoute {
                route: full.routes[0].clone(),
            },
        ];
        assert!(matches!(
            rig.apply(&edits, 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        // The adoption block, 7, is off the grid of three-block windows.
        let mut first = None;
        for _ in 0..3 * WINDOW {
            assert_eq!(rig.host.render_next(), RESULT_OK);
            if rig.host.poll_meters() > 0 && rig.host.meter_header().track_count == 1 {
                first = Some(rig.host.meter_header().first_sample);
                break;
            }
        }
        assert_eq!(
            first,
            Some(ADOPTION * quantum),
            "the first strip frame starts at the adoption block"
        );
        // Exact: the queued window [3, 6), discarded unread by the swap. The swap opened a new
        // delivery epoch, so the gap to the adoption block is not a second loss.
        assert_eq!(rig.host.meter_header().reserved[1] >> METER_LOSS_SHIFT, 1);

        // The strip meters tile from the adoption block now, so a lease taken again, and a
        // producer reset, activate on that grid, not on the absolute `sample % period` one.
        let published = |rig: &mut Rig, blocks: u64| {
            let mut frames = Vec::new();
            for _ in 0..blocks {
                assert_eq!(rig.host.render_next(), RESULT_OK);
                if rig.host.poll_meters() > 0 {
                    frames.push(rig.host.meter_header().first_sample);
                }
            }
            frames
        };
        assert_eq!(rig.host.set_meter_lease(false), RESULT_OK);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        let frames = published(&mut rig, 12 * WINDOW);
        assert!(!frames.is_empty(), "a retaken lease publishes");
        assert!(
            frames
                .iter()
                .all(|start| (start / quantum - ADOPTION).is_multiple_of(WINDOW)),
            "on the adoption grid: {frames:?}"
        );

        // A producer reset: the strip's next window, rendered and not polled, names a new
        // generation.
        let queued = |rig: &mut Rig| {
            rig.host
                .test_render_mut()
                .plan
                .active_attachment_mut::<RenderCompanions>()
                .expect("companions")
                .meters[0]
                .consumer
                .available_at_entry()
        };
        for _ in 0..WINDOW {
            if queued(&mut rig) > 0 {
                break;
            }
            assert_eq!(rig.host.render_next(), RESULT_OK);
        }
        let ready = rig.host.test_render_mut();
        let companions = ready
            .plan
            .active_attachment_mut::<RenderCompanions>()
            .expect("companions");
        let mut snapshot = companions.meters[0]
            .consumer
            .try_pop()
            .expect("a closed strip window");
        snapshot.reset_generation = snapshot.reset_generation.saturating_add(1);
        ready.meters.test_only_pending_mut()[0] = Some(snapshot);
        assert_eq!(rig.host.poll_meters(), 0, "the reset group is refused");
        let frames = published(&mut rig, 12 * WINDOW);
        assert!(!frames.is_empty(), "a producer reset recovers");
        assert!(
            frames
                .iter()
                .all(|start| (start / quantum - ADOPTION).is_multiple_of(WINDOW)),
            "on the adoption grid: {frames:?}"
        );

        // A lease still waiting for its activation when the swap adopts: taken at block 1 (its
        // activation, block 3, on the boot grid), and the first track adopted at block 1. The
        // activation moves to the new grid's origin, so the first frame, strips and master paired,
        // is [1, 4).
        let mut rig = Rig::boot(&empty, options);
        assert_eq!(rig.host.render_next(), RESULT_OK);
        assert_eq!(rig.host.set_meter_lease(true), RESULT_OK);
        assert!(matches!(
            rig.apply(&edits, 1),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        let frames = published(&mut rig, 12 * WINDOW);
        assert_eq!(
            frames.first().copied(),
            Some(quantum),
            "a pending lease publishes from the new grid's first boundary: {frames:?}"
        );
        assert_eq!(
            rig.host.meter_header().track_count,
            1,
            "the frames pair the strip with the master"
        );
        assert!(
            frames
                .iter()
                .all(|start| (start / quantum - 1).is_multiple_of(WINDOW)),
            "on the adoption grid: {frames:?}"
        );
    }

    impl Rig {
        /// The strips whose `master` monitor row is on in the control list, read through the
        /// production control entry by the rows' typed fields.
        fn listed_masters(&mut self, request: u64) -> Vec<String> {
            let codec = ProtocolCodec::default();
            let mut masters = Vec::new();
            let (mut epoch, mut after, mut request) = (0, 0, request);
            loop {
                let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
                let length = codec
                    .encode_command_frame_into(
                        &TypedCommandFrame {
                            request_id: RequestId::new(request).expect("nonzero request"),
                            expected_revision: ExpectedRevision::Any,
                            payload: CommandPayload::ControlListGet(
                                &protocol::ControlListRequest {
                                    structure_epoch: epoch,
                                    after_handle: after,
                                    limit: 256,
                                    owner: None,
                                },
                            ),
                        },
                        &mut bytes,
                    )
                    .expect("a control list frame");
                bytes.truncate(length);
                request += 1;
                let (result, response) = self.entry(&bytes);
                assert_eq!(result, RESULT_OK, "the control plane answers the list");
                let mut fields = [0_u16; 8192];
                let DecodedTypedResponseFrame::Success {
                    payload: DecodedSuccessResponsePayload::ControlListPage(page),
                    ..
                } = codec
                    .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                    .expect("a control list page")
                else {
                    panic!("expected a control list page")
                };
                masters.extend(page.entries.iter().filter_map(|entry| match &entry.body {
                    protocol::ControlBody::Control(row)
                        if row.name == "master" && row.value == 1.0 =>
                    {
                        entry.address.owner_id.clone()
                    }
                    _ => None,
                }));
                if page.eof {
                    return masters;
                }
                epoch = page.structure_epoch;
                after = page.last_handle;
            }
        }
    }

    /// The observation fixture's three tracks (`t0`, `t1`, `t2`), booted with no master and then
    /// designating track `master` by its `master` row (#1689 D6), a monitor write.
    fn row_designated(master: usize) -> (SessionModel, Rig) {
        let model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/observation-frame-shape.json"
        ))
        .expect("the observation fixture parses");
        let options = WebBootOptions {
            source_ring_frames: QUANTUM * 4,
            live_control_meter_blocks: 2,
            ..boot_options(QUANTUM)
        };
        let mut rig = Rig::boot(&model, options);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 0);
        let revision = rig.host.committed_revision();
        assert_eq!(
            rig.apply(&[master_edit(model.tracks[master].id.as_str(), true)], 1),
            Outcome::Applied {
                path: EditPath::Monitor,
                revision,
            }
        );
        (model, rig)
    }

    /// #1683 review fold: the master is the core's `master` row of its strip. A live `SET_CONTROL`
    /// of another row leaves the row on `t1`, listed in the control list, and the header's index
    /// unmoved. (A rebuild that moves the strip's index is
    /// `a_master_designated_after_an_empty_boot_reads_as_its_strips_own_word`'s.)
    ///
    /// Test value: red if a live write of another row moves or clears the designation, or if the
    /// control list stops listing the master's row.
    #[test]
    fn a_row_designated_master_survives_a_live_write_of_another_row() {
        let (_, mut rig) = row_designated(1);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 2);
        assert_eq!(rig.listed_masters(2), ["t1"]);

        let fader = SessionEdit::SetControl {
            expected_effect: None,
            address: controls::ControlAddress {
                owner: controls::Owner::Strip(StableId::parse("t2").expect("strip ID")),
                section: controls::Section::Builtins,
                control: 5,
                lane: session::ParameterChannel::Both,
            },
            value: -6.0,
            ramp_samples: None,
        };
        let live = rig.apply(&[fader], 1_000);
        assert!(
            matches!(
                live,
                Outcome::Applied {
                    path: EditPath::Live,
                    ..
                }
            ),
            "{live:?}"
        );
        assert_eq!(rig.host.render_next(), RESULT_OK);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 2);
        assert_eq!(rig.listed_masters(1_001), ["t1"]);
    }

    /// #1683 review fold: a rebuild that removes the master strip ends the designation in the
    /// committed model and the replacement's tables at once, but the plan still rendering keeps its
    /// master until the block that adopts the replacement.
    ///
    /// Test value: red if the commit rewrites the rendering plan's header (its reading would vanish
    /// before render has moved to the plan without the strip), or if the adopted plan keeps a
    /// master the session no longer has.
    #[test]
    fn a_rebuild_that_removes_the_master_keeps_the_rendering_plans_master_until_adoption() {
        let (_, mut rig) = row_designated(0);
        assert_eq!(rig.host.meter_header().master_track_plus_one, 1);
        assert!(matches!(
            rig.apply(
                &[
                    SessionEdit::RemoveRoute {
                        route_id: StableId::parse("t0-main").expect("a route ID"),
                    },
                    SessionEdit::RemoveTrack {
                        track_id: StableId::parse("t0").expect("a track ID"),
                    },
                ],
                2
            ),
            Outcome::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        assert!(rig.pending(), "the replacement waits for its adoption");
        assert_eq!(
            rig.host.meter_header().master_track_plus_one,
            1,
            "the rendering plan keeps its master until adoption"
        );
        assert_eq!(rig.listed_masters(3), Vec::<String>::new());
        // The removed strip ramps out, then the rebuild is adopted at `S` (#1325 D3); every block
        // before it still renders the plan with the master.
        let mut blocks = 0;
        while rig.pending() {
            assert_eq!(
                rig.host.meter_header().master_track_plus_one,
                1,
                "the rendering plan keeps its master until adoption"
            );
            assert_eq!(rig.host.render_next(), RESULT_OK, "the rebuild is adopted");
            blocks += 1;
            assert!(blocks <= 64, "the rebuild is adopted");
        }
        assert_eq!(
            rig.host.meter_header().master_track_plus_one,
            0,
            "the adopted plan has no master"
        );
    }
    /// The latency and tail words a fresh boot of `model` reports, the oracle a rebuilt plan's
    /// record is compared with.
    fn plan_latency_of_boot(model: &SessionModel) -> WebPlanLatency {
        let mut fresh = Rig::boot(model, options());
        assert_eq!(fresh.host.read_plan_latency(), RESULT_OK);
        *fresh.host.plan_latency()
    }

    /// Issue #1688 gate 2: the plan latency record is the control plane's row for the plan that
    /// renders now. It is refused (and left zero) while the host is not ready, holds the boot
    /// plan's words after boot, keeps them while a rebuild that adds a true-peak limiter insert
    /// waits for its adoption block, and holds the successor's words from the first read after it,
    /// equal to a fresh boot of the committed model.
    ///
    /// Test value: red if the read reports the candidate's latency before render adopts it, keeps
    /// the boot value after a rebuild (a boot-time copy like `WebResourceReport`), or answers a
    /// host that is not ready.
    #[test]
    fn the_plan_latency_record_follows_the_active_plan() {
        const LIMITER_ID: &str = "added-limiter";
        let model = gate_model();
        let mut rig = Rig::boot(&model, options());
        assert_eq!(PLAN_LATENCY_BYTES, 64);

        // A host that is not ready refuses and leaves the record alone.
        rig.host.set_state_for_test(STATE_FAILED);
        assert_eq!(rig.host.read_plan_latency(), RESULT_WRONG_STATE);
        assert_eq!(*rig.host.plan_latency(), WebPlanLatency::default());
        rig.host.set_state_for_test(STATE_READY);

        // After boot the record holds the boot plan's row, copied.
        assert_eq!(rig.host.read_plan_latency(), RESULT_OK);
        let boot = *rig.host.plan_latency();
        let row = rig.host.test_control().queries.resources();
        assert_eq!(boot.struct_size, PLAN_LATENCY_BYTES);
        assert_eq!(boot.reserved0, 0);
        assert_eq!(boot.reserved, [0; 4]);
        assert_eq!(boot.latency_samples, row.latency_samples);
        assert_eq!(
            (boot.tail_kind, boot.tail_samples),
            control_plane::tail_words(row.output_tail_every_peak)
        );

        for _ in 0..APPLY_BEFORE {
            rig.step();
        }
        let (_, _, _, mut limiter, _) = strip_base();
        limiter.id = strip_id(LIMITER_ID);
        limiter.identity = session::EffectIdentity::Native {
            effect_id: strip_id("miso.true-peak-limiter"),
        };
        limiter.params = vec![session::EffectParam {
            parameter_id: 1,
            channel: session::ParameterChannel::Both,
            unit: session::ParameterUnit::Db,
            value: -6.0,
        }];
        let mut committed = model.clone();
        committed.tracks[0].inserts.effects.push(limiter.clone());
        committed.revision += 1;
        let base = rig.host.committed_revision();
        assert_eq!(
            rig.apply(
                &[SessionEdit::PutTrackEffect {
                    track_id: model.tracks[0].id.clone(),
                    rack_name: session::RackName::Inserts,
                    final_position: 0,
                    effect: limiter,
                }],
                1
            ),
            Outcome::Applied {
                path: EditPath::Rebuild,
                revision: base + 1
            }
        );
        let successor = plan_latency_of_boot(&committed);
        assert!(
            successor.latency_samples > boot.latency_samples,
            "the limiter's lookahead moves the plan latency ({} -> {})",
            boot.latency_samples,
            successor.latency_samples
        );

        // Committed but not adopted: the plan that renders is the boot plan.
        assert!(rig.pending());
        assert_eq!(rig.host.read_plan_latency(), RESULT_OK);
        assert_eq!(
            *rig.host.plan_latency(),
            boot,
            "the record is the active plan's"
        );

        // The first read after the adopting block holds the successor's words. The limiter
        // restarts the track: a duck-swap, adopted at its `S` (#1324a).
        rig.step_through_adoption();
        assert_eq!(rig.epoch(), 1);
        assert_eq!(rig.host.read_plan_latency(), RESULT_OK);
        assert_eq!(
            *rig.host.plan_latency(),
            successor,
            "the adopted plan's row equals a fresh boot of the committed model"
        );
    }
}

/// Issue #1643: the session seek exports, driven as the page's message handler drives them.
/// Test support only: it reads the timeline and the slots through `native_staging`'s
/// `test-support` helpers.
#[cfg(feature = "test-support")]
mod session_seek {
    use super::*;
    use protocol::{
        CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
        EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
        TypedCommandFrame,
    };
    use session::{RouteSource, SessionModel, StableId};

    const LONG: &[u8] = b"fixture-source";
    const SHORT: &[u8] = b"short-source";
    const ADDED: &[u8] = b"added-source";
    const Q: u64 = 128;

    /// `base` with one more source, track and route: ID `source_id`, `frames` long.
    fn add_source(
        base: &SessionModel,
        source_id: &str,
        track_id: &str,
        frames: u64,
    ) -> [SessionEdit; 3] {
        let mut source = base.sources[0].clone();
        source.id = StableId::parse(source_id).expect("source ID");
        source.frames = frames;
        let mut track = base.tracks[0].clone();
        track.id = StableId::parse(track_id).expect("track ID");
        track.source_id = source.id.clone();
        let mut route = base.routes[0].clone();
        route.id = StableId::parse(&format!("{track_id}-main")).expect("route ID");
        let RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ]
    }

    /// The identity fixture at quantum 128 with `fixture-source` 10,000 frames long.
    fn one_source() -> SessionModel {
        let mut model = parse_session_json(&identity_session(Q as u32, 10_000)).expect("session");
        model.sources[0].frames = 10_000;
        model
    }

    /// `fixture-source` (10,000 frames) and `short-source` (4,000 frames), one track each.
    fn two_sources() -> SessionModel {
        let mut model = one_source();
        let [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ] = add_source(&model, "short-source", "t2", 4_000)
        else {
            panic!("three upserts")
        };
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
        model
    }

    /// One booted host driven through the raw exports, with `fixture-source` primed for four
    /// blocks of generation 1 (and `short-source`, if the session has it).
    struct Rig {
        handle: u32,
        block: u64,
        sources: Vec<&'static [u8]>,
    }

    impl Rig {
        fn boot(model: &SessionModel, sources: &[&'static [u8]]) -> Self {
            let document = canonical_session_json(model).expect("canonical session");
            let options = WebBootOptions {
                source_ring_frames: 512,
                ..boot_options(Q as u32)
            };
            let handle = crate::ffi::test_boot(document.as_bytes(), options);
            assert_ne!(handle, 0);
            assert_eq!(crate::ffi::test_fill_source_pcm(handle, 0.25), RESULT_OK);
            let mut rig = Self {
                handle,
                block: 0,
                sources: sources.to_vec(),
            };
            for _ in 0..4 {
                for id in sources {
                    assert_eq!(rig.submit(id, 1, rig.block * Q), RESULT_OK);
                }
                rig.render();
            }
            rig
        }

        fn stage(&self, id: &[u8]) {
            assert_eq!(
                crate::ffi::test_copy_staging(self.handle, BUFFER_SOURCE_ID, id),
                RESULT_OK
            );
        }

        /// One quantum of `id`'s generation `generation` from source frame `frame`.
        fn submit(&self, id: &[u8], generation: u64, frame: u64) -> u32 {
            self.stage(id);
            miso_engine_web_v1_source_submit(
                self.handle,
                id.len() as u32,
                generation,
                frame,
                2,
                Q as u32,
                0,
            )
        }

        fn feed(&self, generation: u64, frame: u64) {
            for id in &self.sources {
                assert_eq!(self.submit(id, generation, frame), RESULT_OK);
            }
        }

        fn render(&mut self) -> Vec<u32> {
            assert_eq!(miso_engine_web_v1_render(self.handle, Q as u32), RESULT_OK);
            self.block += 1;
            crate::ffi::native_staging::output_pcm(self.handle)
                .expect("output")
                .iter()
                .map(|sample| sample.to_bits())
                .collect()
        }

        /// The first sample of the next block.
        fn next(&self) -> u64 {
            self.block * Q
        }

        fn timeline(&self, sample: u64) -> i64 {
            crate::ffi::native_staging::timeline_at(self.handle, sample).expect("a timeline")
        }

        /// `(generation, first_sample, source_frame)` of `id`'s seek report.
        fn landed(&self, id: &[u8]) -> (u64, u64, u64) {
            self.stage(id);
            assert_eq!(
                miso_engine_web_v1_source_seek_report(self.handle, id.len() as u32),
                RESULT_OK
            );
            let record = crate::ffi::native_staging::seek_report(self.handle).expect("record");
            (record.generation, record.first_sample, record.source_frame)
        }

        fn last_result(&self) -> u32 {
            crate::ffi::test_status(self.handle)
                .expect("status")
                .last_result
        }
    }

    impl Drop for Rig {
        fn drop(&mut self) {
            assert_eq!(miso_engine_web_v1_dispose(self.handle), RESULT_OK);
        }
    }

    /// Gate 1 up to its anchored seek: `session_seek(2, 6_000)` and the block that applied it.
    fn after_plain_seek() -> Rig {
        let mut rig = Rig::boot(&two_sources(), &[LONG, SHORT]);
        assert_eq!(
            miso_engine_web_v1_session_seek(rig.handle, 2, 6_000),
            RESULT_OK
        );
        // The long source's PCM from the sought frame. The short source clamps to its end, 4,000,
        // where it has no frame to submit: its seek report still names that frame.
        assert_eq!(rig.submit(LONG, 2, 6_000), RESULT_OK);
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG), (2, sample, 6_000));
        assert_eq!(rig.landed(SHORT), (2, sample, 4_000), "clamped to its end");
        assert_eq!(rig.timeline(sample), 6_000);
        rig
    }

    /// Gate 1, both forms move the timeline and every source in the one block.
    ///
    /// Test value: red if the export seeks the consumers in different blocks (for example through
    /// `prepare_source_seek` for the sources only, which would also leave the timeline behind),
    /// clamps a short source wrongly, or drops the anchor of the anchored form.
    #[test]
    fn both_forms_move_the_timeline_and_every_source_at_one_block() {
        let mut rig = after_plain_seek();
        let anchor = rig.next() + 2 * Q;
        assert_eq!(
            miso_engine_web_v1_session_seek_at(rig.handle, 3, 1_000, anchor),
            RESULT_OK
        );
        // Held before the anchor: the plain seek's generation and a timeline that runs on.
        for block in 1..=2 {
            let sample = rig.next();
            rig.render();
            assert_eq!(rig.landed(LONG).0, 2);
            assert_eq!(rig.landed(SHORT).0, 2);
            assert_eq!(rig.timeline(sample), (6_000 + block * Q) as i64);
        }
        rig.feed(3, 1_000);
        let sample = rig.next();
        assert_eq!(sample, anchor);
        rig.render();
        assert_eq!(rig.landed(LONG), (3, anchor, 1_000));
        assert_eq!(rig.landed(SHORT), (3, anchor, 1_000));
        assert_eq!(rig.timeline(anchor), 1_000);
    }

    /// Gate 1, one full source slot refuses every consumer.
    ///
    /// Test value: red if a slot is pushed before every slot is checked (the partial seek the
    /// acked-batch rule forbids): the short source or the timeline would have moved.
    #[test]
    fn one_full_slot_refuses_every_consumer() {
        let mut rig = Rig::boot(&two_sources(), &[LONG, SHORT]);
        let anchor = rig.next() + 4 * Q;
        // The long source's slot holds this seek until the next block's pop; it is full here
        // only because no block renders between the two calls.
        rig.stage(LONG);
        assert_eq!(
            miso_engine_web_v1_source_seek_at(rig.handle, LONG.len() as u32, 2, 640, anchor),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_web_v1_session_seek(rig.handle, 3, 2_000),
            RESULT_BACKPRESSURE
        );
        assert_eq!(rig.last_result(), RESULT_BACKPRESSURE);
        // The short source's slot and the timeline's stayed empty.
        rig.stage(SHORT);
        assert_eq!(
            miso_engine_web_v1_source_seek_at(rig.handle, SHORT.len() as u32, 3, 100, anchor),
            RESULT_OK
        );
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(SHORT).0, 1, "held until the anchor");
        assert_eq!(
            rig.timeline(sample),
            sample as i64,
            "the timeline did not move"
        );
    }

    /// Gate 3a, the role: on a host that is not ready (a sticky-failed host, which still holds its
    /// control state) both exports refuse with `RESULT_REFUSED_LIFECYCLE` and change nothing; a
    /// null handle is `RESULT_INVALID_ARGUMENT`.
    ///
    /// Test value: red if either export runs without checking the host is ready (the failed host
    /// still holds its control state, so the seek would be queued in its slots, which no block
    /// empties), or moves the report record or the timeline on a refusal.
    #[test]
    fn the_session_seek_exports_refuse_a_sticky_failed_host_and_change_nothing() {
        let mut rig = Rig::boot(&one_source(), &[LONG]);
        assert_eq!(
            miso_engine_web_v1_session_seek(0, 2, 0),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_session_seek_at(0, 2, 0, 256),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            miso_engine_web_v1_session_seek(rig.handle, 2, 3_000),
            RESULT_OK
        );
        assert_eq!(rig.submit(LONG, 2, 3_000), RESULT_OK);
        let sample = rig.next();
        rig.render();
        let landed = rig.landed(LONG);
        assert_eq!(landed, (2, sample, 3_000));
        let timeline = rig.timeline(sample);
        assert_eq!(timeline, 3_000);

        // An output-shape mismatch makes the host sticky-failed: no control half to run on.
        assert_eq!(
            miso_engine_web_v1_render(rig.handle, 0),
            RESULT_REPREPARE_REQUIRED
        );
        assert_eq!(
            crate::ffi::test_status(rig.handle).expect("status").state,
            STATE_FAILED
        );
        let record = crate::ffi::native_staging::seek_report(rig.handle);
        assert_eq!(
            miso_engine_web_v1_session_seek(rig.handle, 3, 0),
            RESULT_REFUSED_LIFECYCLE
        );
        assert_eq!(
            miso_engine_web_v1_session_seek_at(rig.handle, 4, 0, 1_024),
            RESULT_REFUSED_LIFECYCLE
        );
        assert_eq!(crate::ffi::native_staging::seek_report(rig.handle), record);
        assert_eq!(rig.timeline(sample), timeline);
        // A failed host renders no block, so a stray push would wait in its slot: none does.
        assert_eq!(
            crate::ffi::native_staging::session_seek_slots_empty(rig.handle),
            Some(true),
            "the refusals queued no seek"
        );
    }

    /// Gate 3a, the pending candidate: a transaction that adds a source is committed through the
    /// control entry, and a session seek before its swap reaches the carried source, the added
    /// source and the timeline in the swap block.
    ///
    /// Test value: red if the export seeks the running plan's producers instead of the newest
    /// committed session's: the added source would not be sought, and the carried source's seek
    /// would land in a ring the candidate does not hold.
    #[test]
    fn a_seek_before_the_swap_reaches_the_carried_and_the_added_source_and_the_timeline() {
        let model = one_source();
        let mut rig = Rig::boot(&model, &[LONG]);
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(1).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(model.revision)),
                    payload: CommandPayload::SessionTransactionApply(&add_source(
                        &model,
                        "added-source",
                        "t2",
                        5_000,
                    )),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        let (result, response) =
            crate::ffi::native_staging::control_command(rig.handle, &bytes[..length])
                .expect("the control staging");
        assert_eq!(result, RESULT_OK, "the control plane answers the frame");
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
            ..
        } = codec
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a transaction response")
        else {
            panic!("the transaction is applied")
        };
        assert_eq!(applied.edit_path, EditPath::Rebuild);
        assert_eq!(
            crate::ffi::native_staging::active_plan_epoch(rig.handle),
            Some(0),
            "the candidate waits for its swap"
        );

        assert_eq!(
            miso_engine_web_v1_session_seek(rig.handle, 2, 3_000),
            RESULT_OK
        );
        assert_eq!(rig.submit(LONG, 2, 3_000), RESULT_OK);
        let swap = rig.next();
        rig.render();
        assert_eq!(
            crate::ffi::native_staging::active_plan_epoch(rig.handle),
            Some(1),
            "the seek block is the swap block"
        );
        assert_eq!(rig.landed(LONG), (2, swap, 3_000));
        assert_eq!(rig.landed(ADDED), (2, swap, 3_000));
        assert_eq!(rig.timeline(swap), 3_000);
    }
}

/// #1646 gate 1 (browser): the parity fixture's first three tracks, with empty console sections,
/// no inserts and the input filters off, so the static fader is the one value a case moves.
fn flat_fader_base() -> session::SessionModel {
    let mut model = parse_session_json(include_str!(
        "../../../fixtures/session/v1/parametric-eq-nine-track.json"
    ))
    .expect("accepted nine-track fixture");
    model.tracks.truncate(3);
    model.routes.truncate(3);
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for builtins in [&mut track.builtins.left, &mut track.builtins.right] {
            builtins.hpf_hz = 0.0;
            builtins.lpf_hz = 0.0;
        }
    }
    model
}

/// A VCA `id` at `offset` dB on both lanes, holding the tracks `members`.
fn flat_fader_vca(
    model: &session::SessionModel,
    id: &str,
    offset: f32,
    members: &[usize],
) -> session::Vca {
    session::Vca {
        id: session::StableId::parse(id).expect("VCA ID"),
        fader: session::DualMonoFader {
            left_db: offset,
            right_db: offset,
            left_mute: false,
            right_mute: false,
        },
        members: members
            .iter()
            .map(|track| model.tracks[*track].id.clone())
            .collect(),
    }
}

/// One VCA of a gate 1 case: its ID, its offset in dB and the tracks it holds.
type FlatFaderVca<'a> = (&'a str, f32, &'a [usize]);

/// `(label, automated, reference)`: the automated session holds a `both` flat fader automation
/// at `value` on `track`, whose static fader is `static_db`; the reference holds the static fader
/// at `value` and no automation. The four cases are the C ABI's gate 1 cases
/// (`crates/capi/src/runtime/tests.rs`, `flat_fader_cases`).
fn flat_fader_sessions() -> Vec<(&'static str, session::SessionModel, session::SessionModel)> {
    let crafted_offsets = [0xc1af_fc6c, 24.0_f32.to_bits(), 0x2624_8cd2].map(f32::from_bits);
    let crafted = f32::from_bits(0x405a_f905);
    let cases: [(&str, usize, f32, f32, &[FlatFaderVca]); 4] = [
        ("no VCA", 0, -7.25, 0.0, &[]),
        ("one +3 dB VCA", 1, -7.25, 0.0, &[("v-plus", 3.0, &[1])]),
        (
            "three VCAs, crafted",
            1,
            crafted,
            0.0,
            &[
                ("va", crafted_offsets[0], &[1]),
                ("vb", crafted_offsets[1], &[1]),
                ("vc", crafted_offsets[2], &[1]),
            ],
        ),
        (
            "-0.0 dB, two VCAs elsewhere",
            2,
            -0.0,
            -3.0,
            &[("va", -1.0, &[0]), ("vb", 2.0, &[0, 1])],
        ),
    ];
    // The crafted case: the two composition orders give different gain bits, so a render that
    // kept the member-first order could not equal the static path (#1646 D6).
    let member_first = crafted_offsets
        .iter()
        .fold(f64::from(crafted), |sum, offset| sum + f64::from(*offset))
        as f32;
    let offsets_first = session::vca_compose_db(
        crafted,
        session::vca_offsets_sum(crafted_offsets).expect("three offsets"),
    );
    let gain = |db: f32| {
        builtins::checked_fader_gain(db)
            .expect("in domain")
            .to_bits()
    };
    assert_ne!(gain(member_first), gain(offsets_first));

    let base = flat_fader_base();
    cases
        .into_iter()
        .map(|(label, track, value, static_db, vcas)| {
            let mut reference = base.clone();
            reference.vcas = vcas
                .iter()
                .map(|(id, offset, members)| flat_fader_vca(&reference, id, *offset, members))
                .collect();
            let fader = &mut reference.tracks[track].fader;
            (fader.left_db, fader.right_db) = (value, value);
            let mut automated = reference.clone();
            let fader = &mut automated.tracks[track].fader;
            (fader.left_db, fader.right_db) = (static_db, static_db);
            automated.automation = vec![session::Automation {
                id: session::StableId::parse("fader-ride").expect("automation ID"),
                target: session::ControlAddress {
                    owner: session::Owner::Strip(automated.tracks[track].id.clone()),
                    section: session::Section::Builtins,
                    control: 5,
                    lane: session::ParameterChannel::Both,
                },
                segments: vec![session::AutomationSegment {
                    shape: session::AutomationShape::Linear,
                    start_sample: 0,
                    end_sample: 1 << 20,
                    start_value: value,
                    end_value: value,
                    unit: session::ParameterUnit::Db,
                }],
                offset: session::AutomationOffset::default(),
            }];
            (label, automated, reference)
        })
        .collect()
}

/// Boots `model` on a live-controlled browser host and renders `blocks` blocks of
/// [`strip_planes`], returning each block's output bits.
fn render_flat_fader_web(model: &session::SessionModel, blocks: u64) -> Vec<Vec<u32>> {
    let document = canonical_session_json(model).expect("canonical");
    let mut host = strip_boot(
        &document,
        WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: STRIP_QUANTUM,
            source_ring_frames: STRIP_QUANTUM * 4,
            ..WebBootOptions::live_control_defaults()
        },
    );
    (0..blocks)
        .map(|block| {
            submit_strip_source(&mut host, "fixture-source", block, &strip_planes(0, block));
            assert_eq!(host.render_next(), RESULT_OK, "block {block}");
            host.output_pcm()
                .expect("output")
                .iter()
                .map(|sample| sample.to_bits())
                .collect()
        })
        .collect()
}

/// #1646 gate 1 (browser): a flat stored fader automation renders, bit for bit, the static fader
/// at its value for 64 blocks on a live-controlled browser host, in the C ABI's four cases (no
/// VCA, one `+3` dB VCA, the crafted three-VCA offsets, `-0.0` dB on a lane no VCA reaches), and
/// each automated session renders differently without its automation.
///
/// Test value: red if the browser's preparation composes the offsets in another order than the
/// static path, if the `+0.0` of a lane no VCA reaches changes the gain bits, if the prepared value
/// is not the curve's, or if the browser converts dB differently from the static fader.
#[test]
fn a_flat_fader_automation_renders_the_static_fader_bits_in_the_browser() {
    const BLOCKS: u64 = 64;
    for (label, automated, reference) in flat_fader_sessions() {
        let rendered = render_flat_fader_web(&automated, BLOCKS);
        let expected = render_flat_fader_web(&reference, BLOCKS);
        let mut unautomated = automated.clone();
        unautomated.automation.clear();
        let without = render_flat_fader_web(&unautomated, BLOCKS);
        assert!(
            expected
                .iter()
                .flatten()
                .any(|bits| f32::from_bits(*bits) != 0.0),
            "{label}: the reference plays"
        );
        assert_ne!(without, expected, "{label}: the automation is heard");
        for (block, (rendered, expected)) in rendered.iter().zip(&expected).enumerate() {
            assert_eq!(
                rendered, expected,
                "{label}: block {block} renders the static fader's bits"
            );
        }
    }
}


// Issue #2191: the control frame is a boot word. The frame-scaled control rows, the replay cache
// and the page's staging follow it, so a bus add over a 64-track console (one route per track,
// more than the 16 KiB the browser had compiled in) commits under a larger word.
mod configured_frame {
    use super::*;
    use protocol::{
        CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
        ExpectedRevision, ProtocolCodec, ProtocolLimits, RequestId, SessionEdit, SessionRevision,
        TypedCommandFrame,
    };
    use session::{RouteDestination, RouteSource, SendTap, StableId};

    const CONSOLE: &str =
        include_str!("../../../fixtures/session/v1/console-sixty-four-track.json");

    fn id(text: &str) -> StableId {
        StableId::parse(text).expect("a stable ID")
    }

    /// An encoder for frames larger than any boot's, so the host, not the encoder, refuses.
    fn codec() -> ProtocolCodec {
        ProtocolCodec::new(ProtocolLimits {
            max_frame_bytes: 1 << 20,
            max_tlv_count: 1 << 19,
            max_string_bytes: 1 << 20,
            max_nesting: 4,
        })
    }

    fn encode(payload: CommandPayload<'_>, request: u64, expected: ExpectedRevision) -> Vec<u8> {
        let mut bytes = vec![0_u8; 1 << 20];
        let length = codec()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(request).expect("nonzero request"),
                    expected_revision: expected,
                    payload,
                },
                &mut bytes,
            )
            .expect("a command frame");
        bytes.truncate(length);
        bytes
    }

    /// The console's atomic bus add: a submix, its route to the output and one muted post-fader
    /// route from every track into it.
    fn bus_add(model: &session::SessionModel) -> Vec<SessionEdit> {
        let bus = id("bus");
        let mut to_output = model.routes[0].clone();
        to_output.id = id("bus-main");
        to_output.source = RouteSource::Submix {
            submix_id: bus.clone(),
            tap: SendTap::PostPan,
        };
        let mut edits = vec![
            SessionEdit::UpsertSubmix {
                submix: session::Submix::unity(bus.clone(), &model.console),
            },
            SessionEdit::UpsertRoute { route: to_output },
        ];
        for track in &model.tracks {
            let mut route = model.routes[0].clone();
            route.id = id(&format!("{}-bus", track.id.as_str()));
            route.source = RouteSource::Track {
                track_id: track.id.clone(),
                tap: SendTap::PostFader,
            };
            route.destination = RouteDestination::SubmixInput {
                submix_id: bus.clone(),
            };
            route.mute = true;
            edits.push(SessionEdit::UpsertRoute { route });
        }
        edits
    }

    fn command(host: &mut AudioWorkletEngineHost, frame: &[u8]) -> (u32, Vec<u8>) {
        let length = u32::try_from(frame.len()).expect("a u32 frame");
        if let Some(staging) = host.control_request_mut(length) {
            staging[..frame.len()].copy_from_slice(frame);
        }
        let result = host.control_command(length);
        (result, host.control_response().to_vec())
    }

    /// Issue #2191 gate 1: a boot reports and enforces the frame its word selects. `CAPABILITIES_GET`
    /// names the frame and a replay cache of at least two frames; a bus add over 64 tracks (more
    /// than 16 KiB) commits at a 32 KiB word and at the 256 KiB default, and is refused at the 16
    /// KiB floor; a frame one byte over the boot's frame is refused with a diagnostic that names
    /// its length and the frame.
    ///
    /// Test value: red if a compiled 16 KiB survives anywhere on the frame's path (the staging,
    /// the command gate, the core's codec, the capabilities), if the replay cache stays at 64 KiB
    /// (the core answers every large command backpressure), or if an oversized frame is refused
    /// without naming its bytes and the limit.
    #[test]
    fn a_bus_add_over_sixty_four_tracks_commits_within_the_booted_frame() {
        let model = parse_session_json(CONSOLE).expect("the console fixture");
        assert_eq!(model.tracks.len(), 64);
        let add = bus_add(&model);
        for (word, frame) in [
            (u64::from(MINIMUM_CONTROL_FRAME_BYTES), MINIMUM_CONTROL_FRAME_BYTES),
            (32 << 10, 32 << 10),
            (0, DEFAULT_CONTROL_FRAME_BYTES),
        ] {
            let mut host = AudioWorkletEngineHost::boot(
                CONSOLE.as_bytes(),
                WebBootOptions {
                    maximum_control_frame_bytes: word,
                    ..WebBootOptions::explicit_defaults()
                },
            )
            .unwrap_or_else(|failure| {
                panic!("{word}: {}", String::from_utf8_lossy(failure.diagnostic()))
            });
            assert_eq!(host.control_frame_bytes(), frame);

            let (result, reply) = command(
                &mut host,
                &encode(CommandPayload::CapabilitiesGet, 1, ExpectedRevision::Any),
            );
            assert_eq!(result, RESULT_OK, "{word}");
            let mut fields = [0_u16; 256];
            let DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::Capabilities(capabilities),
                ..
            } = codec()
                .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
                .expect("a capabilities reply")
            else {
                panic!("{word}: a capabilities reply")
            };
            assert_eq!(capabilities.maximum_frame_bytes, u64::from(frame));
            assert!(capabilities.replay_bytes >= 2 * u64::from(frame), "{word}");

            let revision = host.committed_revision();
            let frame_bytes = encode(
                CommandPayload::SessionTransactionApply(&add),
                2,
                ExpectedRevision::Exact(SessionRevision(revision)),
            );
            let length = frame_bytes.len() as u32;
            assert!(length > MINIMUM_CONTROL_FRAME_BYTES && length < 32 << 10);
            let (result, reply) = command(&mut host, &frame_bytes);
            if length > frame {
                assert_eq!(result, RESULT_INVALID_ARGUMENT, "{word}");
                assert_eq!(
                    reply,
                    format!(
                        "web.control.frame_bytes\t$[frame_bytes={length},limit_bytes={frame}]\n"
                    )
                    .into_bytes()
                );
            } else {
                assert_eq!(result, RESULT_OK, "{word}");
                let mut fields = [0_u16; 64];
                let decoded = ProtocolCodec::default()
                    .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
                    .expect("a transaction response");
                match decoded {
                    DecodedTypedResponseFrame::Success {
                        payload: DecodedSuccessResponsePayload::SessionTransactionApplied(_),
                        ..
                    } => {}
                    DecodedTypedResponseFrame::NonOk { payload, .. } => panic!(
                        "{word}: the bus add is refused {:?}",
                        payload
                            .diagnostics
                            .iter()
                            .map(|diagnostic| diagnostic.code.to_string())
                            .collect::<Vec<_>>()
                    ),
                    DecodedTypedResponseFrame::Success { .. } => panic!("{word}: not an apply"),
                }
                assert_eq!(host.committed_revision(), revision + 1);
            }

            assert!(host.control_request_mut(frame).is_some_and(|staging| staging.len() == frame as usize));
            assert!(host.control_request_mut(frame + 1).is_none());
            assert_eq!(host.control_command(frame + 1), RESULT_INVALID_ARGUMENT);
            assert_eq!(
                host.control_response(),
                format!(
                    "web.control.frame_bytes\t$[frame_bytes={},limit_bytes={frame}]\n",
                    frame + 1
                )
                .as_bytes()
            );
        }
    }
}
