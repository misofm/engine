//! Control-plane and C-boundary gates for the runtime modules.

use super::*;
use protocol::{ExpectedRevision, RequestId, SessionRevision, StatusCode};
use session::parse_session_json;

/// Region end of the single fixture source, read through the facade's accessor.
fn source_region_end(sources: &SourceControlSet) -> u64 {
    sources
        .region(b"fixture-source")
        .expect("fixture source region")
        .end
}

pub(super) const SESSION: &str =
    include_str!("../../../../fixtures/session/v1/parametric-eq-nine-track.json");

pub(super) fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: crate::COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

pub(super) fn command_bytes(request_id: u64, payload: protocol::CommandPayload<'_>) -> Vec<u8> {
    command_bytes_at_revision(request_id, ExpectedRevision::Any, payload)
}

pub(super) fn command_bytes_at_revision(
    request_id: u64,
    expected_revision: ExpectedRevision,
    payload: protocol::CommandPayload<'_>,
) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = protocol::TypedCommandFrame {
        request_id: RequestId::new(request_id).expect("nonzero request"),
        expected_revision,
        payload,
    };
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let len = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("typed command");
    bytes.truncate(len);
    bytes
}

fn pinned_hex(value: &str) -> Vec<u8> {
    assert!(value.len().is_multiple_of(2));
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |value: u8| match value {
                b'0'..=b'9' => value - b'0',
                b'a'..=b'f' => value - b'a' + 10,
                _ => panic!("pinned lowercase hexadecimal"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

const ALL_COMMAND_RESPONSE_VECTORS: [&str; 11] = [
    "4d49534f43544c00010000003000020001000000c801000001000000000000002a000000000000001b000000000000000100020102000000010000000000000002000201020000000000000000000000030002010200000001000000000000000400020102000000000000000000000005000401080000000010000000000000060003010400000000080000000000000700040108000000001000000000000008000101010000000400000000000000090002010200000000010000000000000a0004010800000001000000000000000b0004010800000000100000000000000c0004010800000001000000000000000d0004010800000001000000000000000e0004010800000002000000000000000f00040108000000010000000000000010000401080000001000000000000000110004010800000000200000000000001200040108000000001000000000000013000401080000008000000000000000140004010800000080000000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01160000000100020003000400050006000700080009000a000b0000001a000c010c000000018002801080208021803080000000001b00040108000000ff3f000000000000",
    // #338 re-pin: canonical JSON snapshot bytes were 16,712 (0x4148), beginning with `{`.
    // #1093 re-pin: 13,729 (0x35a1) after the fixture moved to decision 12's console shape.
    // #1216 re-pin: 13,918 (0x365e), route field `mute` added, value false (21 bytes, 9 routes).
    // #1218 re-pin: 14,179 (0x3763), route field `follows_mute` added, value false (29 bytes,
    // 9 routes).
    // #1240 re-pin: 14,193 (0x3771), root key `vcas` added, empty (`  "vcas": [],` and its
    // newline, 14 bytes).
    "4d49534f43544c000100000030000200020000004000000002000000000000002a000000000000000400000000000000010004010800000071370000000000000200040108000000000000000000000003000a01010000007b0000000000000004000801010000000000000000000000",
    // #1093 re-pin: the first descriptor's rack is `console` (5), was `simd1` (1).
    "4d49534f43544c000100000030000200040000003801000003000000000000002a000000000000000300000000000000010003010400000001000000000000000200080101000000000000000000000003000b011001000010000000000000000100030104000000010000000000000002000901030000006571300000000000030001010100000005000000000000000400090102000000657100000000000005000301040000000100000000000000060001010100000001000000000000000700010101000000010000000000000008000101010000000500000000000000090001010100000002000000000000000c0006010400000000000000000000000d0001010100000004000000000000000e0001010100000003000000000000000f00030104000000000000000000000010000301040000000500000000000000110009000e00000062616e642d312d656e61626c6564000012000900060000006f6e2f6f66660000",
    "4d49534f43544c000100000030000200050000004800000004000000000000002a00000000000000040000000000000001000401080000000000000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010000000000803f00000000",
    "4d49534f43544c000100000030000200060005004800000005000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000",
    "4d49534f43544c000100000030000200070000003000000006000000000000002a000000000000000300000000000000010001010100000001000000000000000200040108000000000000000000000003000401080000000000000000000000",
    "4d49534f43544c000100000030000200080000003000000007000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000",
    "4d49534f43544c000100000030000200090000005000000008000000000000002a00000000000000060000000000000001000d01000000000200030104000000000000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
    "4d49534f43544c0001000000300002000a0000007000000009000000000000002a0000000000000003000000000000000100040108000000000000000000000002000b01280000000200000000000000010003010400000005000000000000000200040108000000000000000000000002000b012800000002000000000000000100030104000000060000000000000002000401080000000000000000000000",
    "4d49534f43544c0001000000300002000b000000200000000a000000000000002a0000000000000002000000000000000100040108000000000000000000000002000801010000000100000000000000",
    "4d49534f43544c00010000003000020003000000100000000b000000000000002b00000000000000010000000000000001000301040000000100000000000000",
];

/// A structural edit that renders identically, for a test that needs a plan rebuild (#1260 D3).
///
/// It changes only the first source's `content` identity, to `blake3:` and `tag` repeated as hex,
/// keeping its length. A session ID edit is model-only and commits without a rebuild, so it can no
/// longer trigger one. Successive rebuilds of one session need distinct tags: rewriting the
/// committed content is a live, record-free delta.
pub(super) fn rebuild_edit(document: &str, tag: u8) -> protocol::SessionEdit {
    let model = parse_session_json(document).expect("rebuild edit session");
    let source = &model.sources[0];
    let content = format!("blake3:{}", format!("{tag:02x}").repeat(32));
    assert_ne!(
        content, source.content,
        "the rebuild edit changes the content"
    );
    protocol::SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content,
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    }
}

pub(super) fn generated_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(SESSION).expect("accepted parity base");
    model.sample_rate_hz = sample_rate_hz;
    model.sources[0].frames = 192;
    if track_count == 1 {
        model.tracks.truncate(1);
        model.routes.truncate(1);
    } else {
        assert_eq!(track_count, 10);
        let mut track = model.tracks[8].clone();
        track.id = session::StableId::parse("eq9").expect("tenth track");
        // The console EQ runs on every track (decision 12), so the tenth track's own effect, a
        // bypassed limiter, is an insert after its bypassed EQ.
        track.console[0].bypass = true;
        let mut effect = model.lower_track(&track).pre_insert[0].clone();
        effect.id = session::StableId::parse("limiter").expect("limiter slot");
        effect.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.true-peak-limiter").expect("limiter id"),
        };
        effect.params.clear();
        effect.bypass = true;
        track.inserts.effects = vec![effect];
        let mut route = model.routes[8].clone();
        route.id = session::StableId::parse("eq9-main").expect("tenth route");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("track route")
        };
        *track_id = track.id.clone();
        model.tracks.push(track);
        model.routes.push(route);
    }
    session::canonical_session_json(&model).expect("canonical parity session")
}

pub(super) fn submit_c(
    session: *mut crate::Session,
    generation: u64,
    start_frame: u64,
    sample_rate_hz: u32,
    left: &[f32],
    right: &[f32],
    final_chunk: bool,
) {
    submit_c_to(
        session,
        b"fixture-source",
        generation,
        start_frame,
        sample_rate_hz,
        left,
        right,
        final_chunk,
    );
}

#[allow(clippy::too_many_arguments)]
fn submit_c_to(
    session: *mut crate::Session,
    source_id: &[u8],
    generation: u64,
    start_frame: u64,
    sample_rate_hz: u32,
    left: &[f32],
    right: &[f32],
    final_chunk: bool,
) {
    let planes = [left.as_ptr(), right.as_ptr()];
    let chunk = crate::SourceChunk {
        struct_size: crate::SOURCE_CHUNK_SIZE,
        sample_rate_hz,
        generation,
        start_frame,
        planes: planes.as_ptr(),
        plane_count: 2,
        frames: left.len() as u32,
        end_of_region: u32::from(final_chunk),
        reserved0: 0,
    };
    let mut report = crate::SubmitReport {
        struct_size: crate::SUBMIT_REPORT_SIZE,
        reserved0: 0,
        accepted_frames: 0,
        cumulative_written_frames: 0,
        active_generation: 0,
    };
    assert_eq!(left.len(), right.len());
    assert_eq!(
        crate::ffi::test_source_submit(session, source_id, &chunk, &mut report),
        crate::RESULT_OK
    );
    assert_eq!(report.accepted_frames, left.len() as u64);
}

pub(super) fn boxed_c_children(session: &str) -> (*mut crate::Session, *mut crate::Plan) {
    boxed_c_children_with_limits(session, limits())
}

pub(super) fn boxed_c_children_with_limits(
    session: &str,
    limits: CompileLimits,
) -> (*mut crate::Session, *mut crate::Plan) {
    let children = compile_children(session, limits).expect("C children");
    (
        Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        ))),
        Box::into_raw(Box::new(crate::Plan::new(children.plan))),
    )
}

pub(super) fn command_c(session: *mut crate::Session, request: &[u8]) -> (u32, Vec<u8>) {
    let (result, _, storage) = command_c_capacity(session, request, 4_096);
    (result, storage)
}

pub(super) fn command_c_capacity(
    session: *mut crate::Session,
    request: &[u8],
    capacity: usize,
) -> (u32, u64, Vec<u8>) {
    let mut storage = vec![0xa5_u8; capacity];
    let mut output = crate::BytesOut {
        struct_size: crate::BYTES_OUT_SIZE,
        reserved0: 0,
        data: if storage.is_empty() {
            core::ptr::null_mut()
        } else {
            storage.as_mut_ptr()
        },
        capacity_bytes: storage.len() as u64,
        required_bytes: u64::MAX,
    };
    let result = crate::ffi::test_submit_command(session, request, &mut output);
    if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
        storage.truncate(output.required_bytes as usize);
    }
    (result, output.required_bytes, storage)
}

#[test]
fn c_commands_publish_the_replacement_parameter_catalog() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    // The EQ is a console slot every track carries (decision 12), so the replacement catalog
    // drops `eq0`'s EQ by removing the track and its route; console edits are #1094's.
    let edits = [
        protocol::SessionEdit::RemoveRoute {
            route_id: session::StableId::parse("eq0-main").expect("route ID"),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: session::StableId::parse("eq0").expect("track ID"),
        },
    ];
    let replace = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(command_c(c_session, &replace).0, crate::RESULT_OK);

    let metadata = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::ParameterMetadataGet(protocol::ParameterMetadataRequest {
            after_handle: 0,
            limit: 1,
        }),
    );
    let (result, response) = command_c(c_session, &metadata);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 512];
    let protocol::DecodedTypedResponseFrame::Success {
        header,
        payload: protocol::DecodedSuccessResponsePayload::ParameterMetadata(page),
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("replacement metadata response")
    else {
        panic!("expected replacement metadata success")
    };
    assert_eq!(header.revision, SessionRevision(43));
    assert_eq!(page.descriptors.len(), 1);
    assert_eq!(page.descriptors[0].handle, 1);
    assert_eq!(page.descriptors[0].track_id, "eq1");
    assert_eq!(page.descriptors[0].rack, protocol::ParameterRack::Console);

    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn c_commands_observe_the_rendered_plan_sample() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let codec = ProtocolCodec::default();
    let initial_transport = command_bytes(1, protocol::CommandPayload::TransportGet);
    let (result, response) = command_c(c_session, &initial_transport);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::TransportGetSnapshot(initial),
        ..
    } = codec
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("initial transport response")
    else {
        panic!("expected initial transport success")
    };
    assert_eq!(initial.effective_sample, protocol::SampleTime(0));

    let mut pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );

    let state = command_bytes(
        2,
        protocol::CommandPayload::ParameterStateGet(&protocol::ParameterStateRequest {
            handles: vec![1],
        }),
    );
    let (result, response) = command_c(c_session, &state);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::ParameterState(state),
        ..
    } = codec
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("rendered parameter-state response")
    else {
        panic!("expected parameter-state success")
    };
    assert_eq!(state.observed_sample, 128);

    let transport = command_bytes(3, protocol::CommandPayload::TransportGet);
    let (result, response) = command_c(c_session, &transport);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::TransportGetSnapshot(transport),
        ..
    } = codec
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("rendered transport response")
    else {
        panic!("expected rendered transport success")
    };
    assert_eq!(transport.effective_sample, protocol::SampleTime(128));

    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

pub(super) fn event_c(session: *mut crate::Session, lane: u32) -> (u32, Vec<u8>) {
    let (result, _, storage) = event_c_capacity(session, lane, 4_096);
    (result, storage)
}

pub(super) fn event_c_capacity(
    session: *mut crate::Session,
    lane: u32,
    capacity: usize,
) -> (u32, u64, Vec<u8>) {
    let mut storage = vec![0xa5_u8; capacity];
    let mut output = crate::BytesOut {
        struct_size: crate::BYTES_OUT_SIZE,
        reserved0: 0,
        data: if storage.is_empty() {
            core::ptr::null_mut()
        } else {
            storage.as_mut_ptr()
        },
        capacity_bytes: storage.len() as u64,
        required_bytes: u64::MAX,
    };
    let result = crate::ffi::test_dequeue_event(session, lane, &mut output);
    if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
        storage.truncate(output.required_bytes as usize);
    }
    (result, output.required_bytes, storage)
}

fn event_c_exact_retry(session: *mut crate::Session, lane: u32, oracle: &[u8]) {
    let (query_result, required, query) = event_c_capacity(session, lane, 0);
    assert_eq!(query_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(required, oracle.len() as u64);
    assert!(query.is_empty());
    let (short_result, short_required, short) = event_c_capacity(session, lane, oracle.len() - 1);
    assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(short_required, oracle.len() as u64);
    assert!(short.iter().all(|byte| *byte == 0xa5));
    let (exact_result, exact_required, exact) = event_c_capacity(session, lane, oracle.len());
    assert_eq!(exact_result, crate::RESULT_OK);
    assert_eq!(exact_required, oracle.len() as u64);
    assert_eq!(exact, oracle);
}

fn render_parity_shape(track_count: usize, sample_rate_hz: u32) {
    let session = generated_parity_session(track_count, sample_rate_hz);
    let mut direct = compile_children(&session, limits()).expect("direct children");
    let wrapped = compile_children(&session, limits()).expect("C children");
    let c_session = Box::into_raw(Box::new(crate::Session::new(
        wrapped.session,
        wrapped.session_error,
    )));
    let c_plan = Box::into_raw(Box::new(crate::Plan::new(wrapped.plan)));
    let quantum = 128_usize;
    let mut first_left = vec![0.0_f32; quantum];
    let mut first_right = vec![0.0_f32; quantum];
    first_left[0] = -0.0;
    first_right[0] = 0.0;
    first_left[1] = 0.25;
    first_right[1] = -0.5;
    let final_left = vec![0.125_f32; 64];
    let final_right = vec![-0.25_f32; 64];

    for block in 0..8_u64 {
        match block {
            0 | 3 => {
                let generation = if block == 0 { 1 } else { 2 };
                if block == 3 {
                    direct
                        .session
                        .seek(b"fixture-source", generation, 0)
                        .expect("direct seek");
                    assert_eq!(
                        crate::ffi::test_source_seek(c_session, b"fixture-source", generation, 0,),
                        crate::RESULT_OK
                    );
                }
                direct
                    .session
                    .submit(
                        b"fixture-source",
                        SourceSubmission {
                            generation,
                            start_frame: 0,
                            sample_rate_hz,
                            planes: &[&first_left, &first_right],
                            frames: quantum as u32,
                            end_of_region: false,
                        },
                    )
                    .expect("direct full chunk");
                submit_c(
                    c_session,
                    generation,
                    0,
                    sample_rate_hz,
                    &first_left,
                    &first_right,
                    false,
                );
            }
            1 | 4 => {
                let generation = if block == 1 { 1 } else { 2 };
                direct
                    .session
                    .submit(
                        b"fixture-source",
                        SourceSubmission {
                            generation,
                            start_frame: 128,
                            sample_rate_hz,
                            planes: &[&final_left, &final_right],
                            frames: 64,
                            end_of_region: true,
                        },
                    )
                    .expect("direct partial final");
                submit_c(
                    c_session,
                    generation,
                    128,
                    sample_rate_hz,
                    &final_left,
                    &final_right,
                    true,
                );
            }
            _ => {}
        }

        let mut direct_pcm = vec![f32::NAN; quantum * 2];
        direct
            .plan
            .render(
                block * quantum as u64,
                PlanarBufferMut::try_new(&mut direct_pcm, 2, quantum, quantum)
                    .expect("direct output"),
            )
            .expect("direct render");
        let mut c_pcm = vec![f32::NAN; quantum * 2];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: c_pcm.as_mut_ptr(),
            sample_capacity: c_pcm.len() as u64,
            frames: quantum as u32,
            plane_stride_samples: quantum as u32,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, block * quantum as u64, &output),
            crate::RESULT_OK
        );
        assert_eq!(
            c_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            direct_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "direct/C parity for {track_count} tracks at {sample_rate_hz} Hz block {block}"
        );
    }
    if track_count == 1 {
        crate::ffi::test_session_destroy(c_session);
        crate::ffi::test_plan_destroy(c_plan);
    } else {
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }
}

#[test]
fn generated_session_prepares_independent_source_and_plan_ownership() {
    let mut children = compile_children(SESSION, limits()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(&failure.diagnostics))
    });
    let resources = children.plan.resources();
    assert_eq!(resources.sample_rate_hz, 48_000);
    assert_eq!(resources.quantum_frames, 128);
    assert_eq!(resources.source_count, 1);
    assert_eq!(resources.track_count, 9);
    assert!(resources.graph_session_plus_plan_bytes > 0);
    assert!(resources.source_total_bytes > 0);
    assert!(resources.effect_scalar_state_bytes > 0);
    assert!(resources.builtin_retained_payload_bytes > 0);
    assert!(resources.capi_retained_bytes > 0);
    assert!(resources.largest_named_allocation_bytes > 0);

    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    let submitted = children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("first source block");
    assert_eq!(submitted.accepted_frames, 128);
    children
        .session
        .seek(b"fixture-source", 2, 48_000)
        .expect("inclusive end seek");
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 48_000,
                sample_rate_hz: 48_000,
                planes: &[&[], &[]],
                frames: 0,
                end_of_region: true,
            },
        )
        .expect("zero-frame final marker");
}

#[test]
fn ring_zero_derives_from_the_document_and_matches_the_explicit_value() {
    let model = parse_session_json(SESSION).expect("session");
    let derived = host_core::default_source_ring_frames(model.sample_rate_hz, model.quantum_frames);
    assert_eq!(derived, 5_120);

    let mut zero = limits();
    zero.source_ring_frames = 0;
    let zero = compile_children(SESSION, zero).expect("zero derives");

    let mut explicit = limits();
    explicit.source_ring_frames = derived;
    let explicit = compile_children(SESSION, explicit).expect("explicit derived value");

    assert_eq!(zero.plan.resources(), explicit.plan.resources());
    assert_eq!(
        zero.session
            .test_providers()
            .test_sources()
            .retained_bytes(),
        explicit
            .session
            .test_providers()
            .test_sources()
            .retained_bytes()
    );
}

#[test]
fn structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic() {
    let mut children = compile_children(SESSION, limits()).expect("children");
    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("old provider source block");
    let mut pcm = [0.0_f32; 256];
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("old output"),
        )
        .expect("old plan block");
    assert!(pcm.iter().any(|sample| *sample != 0.0), "old provider PCM");
    // A rebuild that leaves the source's declaration alone, so its ring carries (#1273 D1).
    let edits = add_muted_track(&parse_session_json(SESSION).expect("model"), "a-muted");
    let first_request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert!(matches!(
        children.session.command(&first_request, 0),
        Err(CommandError::BufferTooSmall { required: 4_096 })
    ));
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(42)
    );
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    assert_eq!(children.plan.owner().active_epoch().0, 0);

    let first_len = children
        .session
        .command(&first_request, 4_096)
        .expect("first structural command");
    let first_response = children.session.command_response(first_len).to_vec();
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(43)
    );
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    assert_eq!(children.session.test_pending_providers()[0].test_epoch(), 1);
    assert_eq!(children.plan.owner().active_epoch().0, 0);
    assert_eq!(children.session.test_controller().replay().len(), 1);
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 128,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("a submission after the commit feeds the newest committed session");

    let required = match children.session.dequeue_event(EventLane::Reliable, 0) {
        Err(EventError::BufferTooSmall { required }) => required,
        other => panic!("expected reliable query length, got {other:?}"),
    };
    let event_len = children
        .session
        .dequeue_event(EventLane::Reliable, required)
        .expect("reliable retry")
        .expect("session event");
    let event = children.session.event_response(event_len).to_vec();
    let mut fields = [0_u16; 64];
    assert!(matches!(
        ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("session event"),
        protocol::DecodedTypedEventFrame {
            header,
            payload: protocol::DecodedEventPayload::SessionCommitted(_),
        } if header.revision == SessionRevision(43)
    ));
    assert_eq!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 0)
            .expect("empty reliable lane"),
        None
    );

    let model = parse_session_json(SESSION).expect("source-changing model");
    let source = &model.sources[0];
    let second_edit = protocol::SessionEdit::SetSourceContent {
        source_id: model.sources[0].id.clone(),
        content: source.content.clone(),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: 512,
    };
    let second_request = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&second_edit)),
    );
    assert!(matches!(
        children.session.command(&second_request, 4_096),
        Err(CommandError::Backpressure)
    ));
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(43)
    );
    assert_eq!(children.session.test_controller().replay().len(), 1);

    // The swap block's continuity is `a_c_abi_structural_transaction_keeps_the_source_playing`'s.
    children
        .plan
        .render(
            128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("replacement boundary");
    assert_eq!(children.plan.owner().active_epoch().0, 1);
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    children
        .session
        .service()
        .expect("control promotion and retirement");
    assert_eq!(children.session.test_providers().test_epoch(), 1);
    assert!(children.session.test_pending_providers().is_empty());

    let second_len = children
        .session
        .command(&second_request, 4_096)
        .expect("source-changing replacement after reclaim");
    assert!(second_len > 0);
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(44)
    );
    assert_eq!(
        source_region_end(children.session.test_providers().test_sources()),
        48_000
    );
    assert_eq!(children.session.test_pending_providers()[0].test_epoch(), 2);
    assert_eq!(
        source_region_end(children.session.test_pending_providers()[0].test_sources()),
        512
    );
    children
        .plan
        .render(
            256,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("second replacement boundary");
    children
        .session
        .service()
        .expect("second provider promotion and retirement");
    // #1273 D1: the successor is diffed against the committed model before the transaction, so
    // the source whose declaration changed (frames 512) restarts in a new ring at generation 1,
    // frame 0, rather than carrying its old ring (written to frame 256) into the new plan.
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("a changed source restarts its feed at generation 1, frame 0");
    assert_eq!(children.session.test_providers().test_epoch(), 2);
    assert_eq!(
        source_region_end(children.session.test_providers().test_sources()),
        512
    );
    assert!(children.session.test_pending_providers().is_empty());
    assert!(children.session.test_retired_providers().is_empty());
    children
        .session
        .seek(b"fixture-source", 2, 384)
        .expect("seek new source-changing provider");
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 384,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: true,
            },
        )
        .expect("new source-changing provider PCM");
    pcm.fill(f32::NAN);
    children
        .plan
        .render(
            384,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("new provider output"),
        )
        .expect("new provider render");
    assert!(
        pcm.iter().any(|sample| *sample != 0.0),
        "source-changing provider produces submitted PCM"
    );

    let replay_len = children
        .session
        .command(&first_request, first_len as u64)
        .expect("exact structural replay");
    assert_eq!(
        children.session.command_response(replay_len),
        first_response
    );
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(44)
    );
    assert!(children.session.test_pending_providers().is_empty());
}

/// A test signal with no exact-zero sample (issue #1269's acceptance shape): every frame and
/// channel differs, so a block rendered from the wrong source position cannot match.
fn continuity_signal(frame: u64, channel: usize, salt: u64) -> f32 {
    let step = ((frame + salt * 131) % 997) as f32 * 1.0e-4;
    if channel == 0 {
        0.125 + step
    } else {
        -0.25 - step
    }
}

/// One block of [`continuity_signal`] at source frame `block * 128`.
fn continuity_block(block: u64, salt: u64) -> (Vec<f32>, Vec<f32>) {
    let start = block * 128;
    (
        (start..start + 128)
            .map(|frame| continuity_signal(frame, 0, salt))
            .collect(),
        (start..start + 128)
            .map(|frame| continuity_signal(frame, 1, salt))
            .collect(),
    )
}

/// The gap-free acceptance session: tracks `eq0`-`eq2` on the fixture source, empty console
/// sections, no inserts, and the input filters off, so no unchanged path holds DSP state.
pub(super) fn stateless_session(sample_rate_hz: u32) -> session::SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture");
    model.sample_rate_hz = sample_rate_hz;
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

/// `UpsertTrack` of a muted copy of the first track, whose ID sorts before every other, and
/// `UpsertRoute` of its route: the added strip contributes exact zeros.
pub(super) fn add_muted_track(
    model: &session::SessionModel,
    id: &str,
) -> [protocol::SessionEdit; 2] {
    let mut track = model.tracks[0].clone();
    track.id = session::StableId::parse(id).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse(&format!("{id}-main")).expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        protocol::SessionEdit::UpsertTrack { track },
        protocol::SessionEdit::UpsertRoute { route },
    ]
}

/// `RemoveRoute` and `RemoveTrack` of a track [`add_muted_track`] added.
fn remove_track(id: &str) -> [protocol::SessionEdit; 2] {
    [
        protocol::SessionEdit::RemoveRoute {
            route_id: session::StableId::parse(&format!("{id}-main")).expect("route ID"),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: session::StableId::parse(id).expect("track ID"),
        },
    ]
}

/// The committed session's canonical JSON, read through `SESSION_SNAPSHOT_GET` in chunks.
fn snapshot_c(session: *mut crate::Session) -> String {
    let mut json = Vec::new();
    for request_id in 1_000_u64.. {
        let request = command_bytes(
            request_id,
            protocol::CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                offset: json.len() as u64,
                maximum_bytes: 2_048,
            }),
        );
        let (result, response) = command_c(session, &request);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(chunk),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("snapshot response")
        else {
            panic!("expected a snapshot chunk")
        };
        json.extend_from_slice(chunk.canonical_json_chunk);
        if chunk.eof {
            break;
        }
    }
    String::from_utf8(json).expect("canonical JSON")
}

/// Renders one 128-frame block through the exported entry point.
fn render_c(plan: *mut crate::Plan, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(plan, block * 128, &output),
        crate::RESULT_OK
    );
    pcm
}

/// Feeds `fixture-source` one block ahead and renders `blocks` blocks from `first`.
fn feed_and_render_c(
    session: *mut crate::Session,
    plan: *mut crate::Plan,
    sample_rate_hz: u32,
    first: u64,
    blocks: u64,
) -> Vec<Vec<f32>> {
    (first..first + blocks)
        .map(|block| {
            let (left, right) = continuity_block(block + 1, 0);
            submit_c(
                session,
                1,
                (block + 1) * 128,
                sample_rate_hz,
                &left,
                &right,
                false,
            );
            render_c(plan, block)
        })
        .collect()
}

/// Issue #1273 gate 1: across a C ABI structural transaction (`UpsertTrack` of a muted track and
/// its route), every block is bit-identical to the committed post-edit session compiled fresh and
/// fed the same PCM from frame 0. The source keeps its ring, generation and position: no seek, no
/// refill.
///
/// Test value: red if the successor allocates a fresh ring for the unchanged source, if the
/// producer stays in the old set, or if a submit after the commit is routed to the old set
/// (`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` pinned the
/// opposite, a silent swap block, before this issue).
fn structural_transaction_keeps_the_source_playing_at(sample_rate_hz: u32) {
    let model = stateless_session(sample_rate_hz);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, sample_rate_hz, &left, &right, false);
    let mut swapped = feed_and_render_c(c_session, c_plan, sample_rate_hz, 0, 6);
    let edits = add_muted_track(&model, "a-muted");
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(command_c(c_session, &transaction).0, crate::RESULT_OK);
    swapped.extend(feed_and_render_c(c_session, c_plan, sample_rate_hz, 6, 6));
    let carried = crate::ffi::test_plan_carry_counts(c_plan);
    assert_eq!(carried, (1, 0), "one swap, and it carried");
    let committed = snapshot_c(c_session);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);

    let (r_session, r_plan) = boxed_c_children(&committed);
    let (left, right) = continuity_block(0, 0);
    submit_c(r_session, 1, 0, sample_rate_hz, &left, &right, false);
    let reference = feed_and_render_c(r_session, r_plan, sample_rate_hz, 0, 12);
    crate::ffi::test_plan_destroy(r_plan);
    crate::ffi::test_session_destroy(r_session);

    for (block, (swapped, reference)) in swapped.iter().zip(&reference).enumerate() {
        assert!(
            reference.iter().all(|sample| *sample != 0.0),
            "{sample_rate_hz} Hz block {block}: the reference plays"
        );
        let swapped_bits = swapped
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>();
        let reference_bits = reference
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>();
        assert_eq!(
            swapped_bits, reference_bits,
            "{sample_rate_hz} Hz block {block}: bit-identical to the fresh post-edit session"
        );
    }
}

#[test]
fn a_c_abi_structural_transaction_keeps_the_source_playing() {
    structural_transaction_keeps_the_source_playing_at(48_000);
    structural_transaction_keeps_the_source_playing_at(96_000);
}

/// Issue #1275 gate 1: a stem a C ABI structural transaction adds (`UpsertSource`, `UpsertTrack`
/// on it and its route) starts in exact time through `miso_engine_v1_source_seek_at`. The host
/// anchors generation 2 at frame `A` to render sample `A`, two blocks past the next render (which
/// also swaps the plan in), and submits it from `A`. Every block is bit-identical to the post-edit
/// session compiled fresh, fed the first stem from frame 0 and the added stem zeros for frames
/// below `A` and the same PCM from `A`.
///
/// Test value: red if the export forwards to the plain seek (observation timing): the seek then
/// applies at the swap block, two blocks before the anchor, and the stem is out of time from the
/// anchor block on (block 8 differs); red as well if the anchored seek goes to the running plan's
/// source set, which does not hold the added source until the swap. With `source_frame` (`F`)
/// unequal to the anchor (`A`), red too if the export or the session swaps the two.
fn an_added_source_starts_at_its_anchor_at(sample_rate_hz: u32, source_frame: u64) {
    const ADDED: &[u8] = b"s2-source";
    const ANCHOR_BLOCK: u64 = 8;
    let anchor = ANCHOR_BLOCK * 128;
    // The stem's frame `F + n` holds the content the reference plays at render frame `A + n`.
    let stem_frame = |render_frame: u64| render_frame - anchor + source_frame;
    let mut model = stateless_session(sample_rate_hz);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, sample_rate_hz, &left, &right, false);
    let mut swapped = feed_and_render_c(c_session, c_plan, sample_rate_hz, 0, 6);

    let mut source = model.sources[0].clone();
    source.id = session::StableId::parse("s2-source").expect("source ID");
    let mut track = model.tracks[0].clone();
    track.id = session::StableId::parse("t2").expect("track ID");
    track.source_id = source.id.clone();
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse("t2-main").expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    let edits = [
        protocol::SessionEdit::UpsertSource { source },
        protocol::SessionEdit::UpsertTrack { track },
        protocol::SessionEdit::UpsertRoute { route },
    ];
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(command_c(c_session, &transaction).0, crate::RESULT_OK);
    assert_eq!(
        crate::ffi::test_source_seek_at(c_session, ADDED, 2, source_frame, anchor),
        crate::RESULT_OK
    );
    for block in 6..16 {
        let next = block + 1;
        let (left, right) = continuity_block(next, 0);
        submit_c(
            c_session,
            1,
            next * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
        if next >= ANCHOR_BLOCK {
            let (left, right) = continuity_block(next, 1);
            submit_c_to(
                c_session,
                ADDED,
                2,
                stem_frame(next * 128),
                sample_rate_hz,
                &left,
                &right,
                false,
            );
        }
        swapped.push(render_c(c_plan, block));
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(c_plan),
        (1, 0),
        "one swap, and it carried the first stem"
    );
    let committed = snapshot_c(c_session);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);

    let (r_session, r_plan) = boxed_c_children(&committed);
    let feed_reference = |block: u64| {
        let (left, right) = continuity_block(block, 0);
        submit_c(
            r_session,
            1,
            block * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
        let (left, right) = if block < ANCHOR_BLOCK {
            (vec![0.0; 128], vec![0.0; 128])
        } else {
            continuity_block(block, 1)
        };
        submit_c_to(
            r_session,
            ADDED,
            1,
            block * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
    };
    feed_reference(0);
    let reference = (0..16)
        .map(|block| {
            feed_reference(block + 1);
            render_c(r_plan, block)
        })
        .collect::<Vec<_>>();
    crate::ffi::test_plan_destroy(r_plan);
    crate::ffi::test_session_destroy(r_session);
    assert_bit_identical(
        &format!("{sample_rate_hz} Hz added stem from frame {source_frame}"),
        0,
        &swapped,
        &reference,
    );
}

#[test]
fn an_added_c_abi_source_starts_at_its_anchored_render_sample() {
    an_added_source_starts_at_its_anchor_at(48_000, 8 * 128);
    an_added_source_starts_at_its_anchor_at(96_000, 8 * 128);
    // `F != A`: the stem's frame 0 enters the graph at the anchor block.
    an_added_source_starts_at_its_anchor_at(48_000, 0);
}

/// Submits block `block` of each of `ids`' signals (salted by position) through the session state.
fn submit_direct(children: &mut CompiledChildren, ids: &[&[u8]], block: u64) {
    for (salt, id) in ids.iter().enumerate() {
        let (left, right) = continuity_block(block, salt as u64);
        let report = children
            .session
            .submit(
                id,
                SourceSubmission {
                    generation: 1,
                    start_frame: block * 128,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("block {block}: {:?}", error.report()));
        assert_eq!(report.accepted_frames, 128);
    }
}

/// Renders block `block` through the plan state.
fn render_direct(children: &mut CompiledChildren, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    children
        .plan
        .render(
            block * 128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("block");
    pcm
}

/// Feeds `ids` one block ahead and renders `blocks` blocks from `first`.
fn feed_and_render_direct(
    children: &mut CompiledChildren,
    ids: &[&[u8]],
    first: u64,
    blocks: u64,
) -> Vec<Vec<f32>> {
    (first..first + blocks)
        .map(|block| {
            submit_direct(children, ids, block + 1);
            render_direct(children, block)
        })
        .collect()
}

fn assert_bit_identical(label: &str, first: u64, actual: &[Vec<f32>], expected: &[Vec<f32>]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let block = first + index as u64;
        assert!(
            expected.iter().all(|sample| *sample != 0.0),
            "{label} block {block}: the reference plays"
        );
        assert_eq!(
            actual
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "{label} block {block}: bit-identical to the reference"
        );
    }
}

/// Issue #1273 gate 2: a transaction that removes a track and its source leaves the remaining
/// source playing from its carried ring, bit-identical to the fresh post-edit session, and a
/// submit for the removed source is refused as unknown.
///
/// Test value: red if the carry joins sources by graph index instead of ID (the remaining
/// source's index moves when the first is removed), or if a removed source's producer stays
/// reachable after the commit.
#[test]
fn removing_a_track_and_its_source_keeps_the_other_source_playing() {
    const AUX: &[u8] = b"aux-source";
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    // The removed source sorts first, so the remaining one changes graph index.
    let mut aux = model.sources[0].clone();
    aux.id = session::StableId::parse("aux-source").expect("source ID");
    model.sources.insert(0, aux);
    model.tracks[0].source_id = session::StableId::parse("aux-source").expect("source ID");
    let both: [&[u8]; 2] = [b"fixture-source", AUX];
    let document = session::canonical_session_json(&model).expect("canonical");
    let mut children = compile_children(&document, limits()).expect("children");
    submit_direct(&mut children, &both, 0);
    feed_and_render_direct(&mut children, &both, 0, 6);
    let edits = [
        protocol::SessionEdit::RemoveRoute {
            route_id: model.routes[0].id.clone(),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: model.tracks[0].id.clone(),
        },
        protocol::SessionEdit::RemoveSource {
            source_id: session::StableId::parse("aux-source").expect("source ID"),
        },
    ];
    children
        .session
        .command(
            &command_bytes_at_revision(
                1,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::SessionTransactionApply(&edits),
            ),
            4_096,
        )
        .expect("removal transaction");
    let (left, right) = continuity_block(7, 1);
    let refused = children.session.submit(
        AUX,
        SourceSubmission {
            generation: 1,
            start_frame: 7 * 128,
            sample_rate_hz: 48_000,
            planes: &[&left, &right],
            frames: 128,
            end_of_region: false,
        },
    );
    let Err(refusal) = refused else {
        panic!("a submit for the removed source is refused")
    };
    assert_eq!(
        refusal.report(),
        (crate::RESULT_INVALID_ARGUMENT, &b"source.id.unknown"[..])
    );
    // #1273 D3: a seek, like a submit, addresses the newest committed session from the commit on.
    let Err(refusal) = children.session.seek(AUX, 2, 0) else {
        panic!("a seek for the removed source is refused")
    };
    assert_eq!(
        refusal.report(),
        (crate::RESULT_INVALID_ARGUMENT, &b"source.id.unknown"[..])
    );
    let swapped = feed_and_render_direct(&mut children, &both[..1], 6, 6);
    assert_eq!(children.plan.owner().carried_count(), 1);

    let committed = session::canonical_session_json(
        children
            .session
            .test_controller()
            .session()
            .compiled()
            .normalized_model(),
    )
    .expect("committed");
    let mut reference = compile_children(&committed, limits()).expect("reference");
    submit_direct(&mut reference, &both[..1], 0);
    let expected = feed_and_render_direct(&mut reference, &both[..1], 0, 12);
    assert_bit_identical("removal", 6, &swapped, &expected[6..]);
}

/// Issue #1273 gate 3: a structural transaction refused at any phase, including a protocol
/// commit that returns `Err`, moves no producer. The source still accepts PCM and the running
/// plan renders it, bit-identical to a session that never saw the transaction.
///
/// Test value: red if the producer hand-over runs before any fallible step: the refused
/// candidate takes the producer with it, and the next submit returns `source.ring.vacated`.
#[test]
fn a_refused_structural_transaction_moves_no_producer() {
    use TestStructuralFaultPhase::{
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterRuntimePrepare,
        BeforeProtocolCommit, BeforeRuntimePrepare, ProtocolCommit,
    };
    let source: [&[u8]; 1] = [b"fixture-source"];
    let model = stateless_session(48_000);
    let document = session::canonical_session_json(&model).expect("canonical");
    let mut reference = compile_children(&document, limits()).expect("reference");
    submit_direct(&mut reference, &source, 0);
    let expected = feed_and_render_direct(&mut reference, &source, 0, 4);
    let edits = add_muted_track(&model, "a-muted");
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    for phase in [
        AfterProtocolPrepare,
        BeforeRuntimePrepare,
        AfterRuntimePrepare,
        AfterAdmission,
        AfterPlanReservation,
        BeforeProtocolCommit,
        ProtocolCommit,
    ] {
        let mut children = compile_children(&document, limits()).expect("children");
        submit_direct(&mut children, &source, 0);
        let mut rendered = feed_and_render_direct(&mut children, &source, 0, 2);
        children
            .session
            .test_set_structural_faults([Some(phase), None]);
        let refused = children.session.command(&transaction, 4_096);
        assert!(
            matches!(
                refused,
                Err(CommandError::Backpressure | CommandError::Internal)
            ),
            "{phase:?}: {refused:?}"
        );
        assert_eq!(
            children.session.test_controller().session().revision(),
            SessionRevision(42),
            "{phase:?}"
        );
        assert!(
            children.session.test_pending_providers().is_empty(),
            "{phase:?}"
        );
        rendered.extend(feed_and_render_direct(&mut children, &source, 2, 2));
        assert_eq!(children.plan.owner().carried_count(), 0, "{phase:?}");
        assert_bit_identical(&format!("{phase:?}"), 0, &rendered, &expected);
    }
}

/// The outcome of one control call, reduced to what issue #1042 distinguishes.
fn control_outcome<T, E: core::fmt::Debug>(result: &Result<T, E>) -> String {
    match result {
        Ok(_) => "ok".to_owned(),
        Err(error) => format!("{error:?}"),
    }
}

/// Issue #1042: a control call that lands inside a plan-swapping render call.
///
/// `RealtimePlanOwner::render_contiguous` swaps plans and commits the retired plan to the
/// retirement queue at the *start* of a render call, while `PlanState::render` publishes
/// `active_epoch` only after that call returns. This test runs the two halves of
/// `PlanState::render` separately and makes control calls between them, so the interleaving a
/// second thread reaches by chance is reached here on every round.
///
/// On the unmodified base the first control call in that window returned `Internal`; the next
/// synchronization parked the old provider in `retired_providers` for good; and the old epoch's
/// report row, skipped because it matched the lagging atomic, kept the table full, so every later
/// structural command returned `Backpressure`.
///
/// Issue #1273 gate 4: the trigger adds and removes a muted track (`UpsertTrack`), so every swap
/// carries the source's ring, and the source is fed twice a round: after the commit, while the
/// candidate is pending, and inside the window. Both submits must land in the carried ring. Test
/// value: red if a submit in either place is routed to the current epoch, whose producer has
/// moved (`source.ring.vacated`), or if the carried ring loses its write position
/// (`source.frame.noncontiguous`).
#[test]
fn control_calls_inside_a_plan_swapping_render_call_keep_replacement_live() {
    let model = parse_session_json(SESSION).expect("fixture");
    let mut children = compile_children(SESSION, limits()).expect("children");
    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    let mut next_frame = 0_u64;
    let mut feed = |children: &mut CompiledChildren, label: &str| {
        let report = children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: next_frame,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("{label}: {:?}", error.report()));
        assert_eq!(report.accepted_frames, 128, "{label}");
        next_frame += 128;
    };
    let mut pcm = [0.0_f32; 256];
    feed(&mut children, "first block");
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("first block");
    // Even rounds add the muted track, odd rounds remove it.
    let structural = |request_id: u64, revision: u64, round: u64| {
        let edits = if round.is_multiple_of(2) {
            add_muted_track(&model, "a-race").to_vec()
        } else {
            remove_track("a-race").to_vec()
        };
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(&edits),
        )
    };
    let mut request_id = 1;
    let mut revision = 42;
    let mut pending = structural(request_id, revision, 0);
    children
        .session
        .command(&pending, 4_096)
        .expect("first structural command");
    revision += 1;

    for round in 1..=4_u64 {
        let sample = round * 128;
        // The candidate is pending: the source's producer has moved to it.
        feed(&mut children, "pending candidate");
        // Drain the round's `SESSION_COMMITTED`, so the reliable lane never backpressures.
        assert!(
            children
                .session
                .dequeue_event(EventLane::Reliable, 4_096)
                .expect("reliable event")
                .is_some()
        );
        let old_epoch = children.session.test_providers().test_epoch();
        let new_epoch = children.session.test_pending_providers()[0].test_epoch();
        assert_eq!(new_epoch, old_epoch + 1);

        // First half of `PlanState::render`: the owner swaps and retires the old plan.
        let report = children
            .plan
            .test_owner_mut()
            .render_contiguous(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
                },
                sample,
            )
            .expect("swapping block");
        assert_eq!(report.swap, engine::realtime::SwapOutcome::Applied);
        assert_eq!(report.active_epoch.0, new_epoch);
        assert_eq!(children.plan.owner().carried_count(), round);
        assert!(
            pcm.iter().all(|sample| *sample != 0.0),
            "round {round}: the swap block plays the carried ring"
        );
        assert_eq!(
            children.plan.test_active_epoch().load(Ordering::Acquire),
            old_epoch,
            "the window is open: the atomic still names the retired plan"
        );

        // Inside the window: a source submit, an immediate command, a lossy dequeue, and a
        // structural command.
        feed(&mut children, "swap window");
        request_id += 1;
        let capabilities = command_bytes(request_id, protocol::CommandPayload::CapabilitiesGet);
        let window_immediate = children.session.command(&capabilities, 4_096);
        let window_lossy = children.session.dequeue_event(EventLane::Lossy, 4_096);
        request_id += 1;
        pending = structural(request_id, revision, round);
        let window_structural = children.session.command(&pending, 4_096);
        let window_revision = children.session.test_controller().session().revision().0;
        // The any-thread query reads the lagging atomic's row, which must still be there.
        let window_query = children.plan.queries().resources();
        let window_rows = children.session.test_transaction_snapshot().resource_rows;
        assert_eq!(
            window_rows
                .iter()
                .find_map(|(epoch, report)| (*epoch == old_epoch).then_some(*report)),
            Some(window_query)
        );

        // Second half of `PlanState::render`: publish the epoch that rendered the block.
        children
            .plan
            .test_active_epoch()
            .store(report.active_epoch.0, Ordering::Release);
        let after_structural = children.session.command(&pending, 4_096);
        let snapshot = children.session.test_transaction_snapshot();
        let rows = snapshot
            .resource_rows
            .iter()
            .map(|(epoch, _)| *epoch)
            .collect::<Vec<_>>();
        assert_eq!(
            (
                control_outcome(&window_immediate),
                control_outcome(&window_lossy),
                control_outcome(&window_structural),
                window_revision,
                control_outcome(&after_structural),
                snapshot.provider_epoch,
                snapshot.retired_provider_epochs,
                snapshot.pending_provider_epochs,
                rows,
            ),
            (
                "ok".to_owned(),
                "ok".to_owned(),
                // The window's structural command waits for the lagging row (transient
                // backpressure), acks nothing, and commits nothing.
                "Backpressure".to_owned(),
                revision,
                "ok".to_owned(),
                new_epoch,
                Vec::new(),
                vec![new_epoch + 1],
                vec![new_epoch, new_epoch + 1],
            ),
            "round {round}: a control call inside the swapping render call"
        );
        revision += 1;
    }
}

/// Issue #1042, attempt 2: a valid structural edit inside the swap window is backpressured, never
/// reported as a compile rejection.
///
/// The peak check pairs a candidate with the resource row of the epoch the atomic names. Inside
/// the window that is the retired plan, not the one rendering. The limit here admits exactly the
/// pairs that really coexist (nine EQs with eight, either way round) and refuses nine with nine.
/// The test removes one EQ and puts it back twice: while the eight-EQ plan is still pending, and
/// inside the window after the swap. Before the fix, both calls paired nine EQs with nine and
/// returned `CompileRejected(effect.resource.limit)`; the same request was admitted once the
/// render call published its epoch.
#[test]
fn a_valid_edit_inside_the_swap_window_is_backpressured_not_compile_rejected() {
    // The fixture's EQ is a console slot on all nine tracks, which a track cannot drop (decision
    // 12), so the edited EQ is a tenth instance: an insert on the first track. Console session
    // edits are #1094's.
    let mut model = parse_session_json(SESSION).expect("fixture");
    let reduced_session = session::canonical_session_json(&model).expect("canonical");
    let mut eq = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    eq.id = session::StableId::parse("eq-insert").expect("insert slot");
    model.tracks[0].inserts.effects.push(eq.clone());
    let full_session = session::canonical_session_json(&model).expect("canonical");
    let full = compile_children(&full_session, limits())
        .expect("ten EQs")
        .plan
        .resources()
        .effect_scalar_state_bytes;
    let reduced = compile_children(&reduced_session, limits())
        .expect("nine EQs")
        .plan
        .resources()
        .effect_scalar_state_bytes;
    assert!(reduced < full, "removing an EQ frees effect state");
    let mut tight = limits();
    tight.maximum_effect_state_bytes = full + reduced;

    let mut children = compile_children(&full_session, tight).expect("tight children");
    let mut pcm = [0.0_f32; 256];
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("first block");
    let track_id = model.tracks[0].id.clone();
    let remove = protocol::SessionEdit::RemoveTrackEffect {
        track_id: track_id.clone(),
        rack_name: session::RackName::Inserts,
        effect_id: eq.id.clone(),
    };
    let put_back = protocol::SessionEdit::PutTrackEffect {
        track_id,
        rack_name: session::RackName::Inserts,
        final_position: 0,
        effect: eq,
    };
    let edit = |request_id: u64, revision: u64, edit: &protocol::SessionEdit| {
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(edit)),
        )
    };
    children
        .session
        .command(&edit(1, 42, &remove), 4_096)
        .expect("nine EQs beside eight fit");
    assert!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("reliable event")
            .is_some()
    );
    let request = edit(2, 43, &put_back);
    // Before the swap the eight-EQ plan is still pending: the edit waits for it, and it is the
    // plan the edit will sit beside, so it is backpressure here too, not a compile rejection.
    let pending = children.session.command(&request, 4_096);

    // First half of `PlanState::render`: swap to the eight-EQ plan.
    let report = children
        .plan
        .test_owner_mut()
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
            },
            128,
        )
        .expect("swapping block");
    assert_eq!(report.swap, engine::realtime::SwapOutcome::Applied);
    let compiled_before = test_lifecycle_counters().candidate_plan_constructed;
    let window = children.session.command(&request, 4_096);
    // Refused before compiling: the report table is full while the atomic lags.
    let window_compiles = test_lifecycle_counters().candidate_plan_constructed - compiled_before;
    let window_revision = children.session.test_controller().session().revision().0;
    // Second half: publish the epoch that rendered the block.
    children
        .plan
        .test_active_epoch()
        .store(report.active_epoch.0, Ordering::Release);
    let after = children.session.command(&request, 4_096);
    assert_eq!(
        (
            control_outcome(&pending),
            control_outcome(&window),
            window_compiles,
            window_revision,
            control_outcome(&after),
            children.session.test_controller().session().revision().0,
        ),
        (
            "Backpressure".to_owned(),
            "Backpressure".to_owned(),
            0,
            43,
            "ok".to_owned(),
            44
        ),
        "the eight-EQ plan rendering beside nine EQs fits; only a retired row says it does not"
    );
}

/// Issue #1042 on one thread: a *rejected* render call can open the window too.
///
/// `RealtimePlanOwner::enter_block` swaps plans before `render_contiguous` checks the block's
/// time, so a render call with the wrong `absolute_sample` swaps and returns
/// `RESULT_RENDER_REJECTED` without publishing `active_epoch`. Before the fix, the next control
/// call returned `RESULT_INTERNAL` and every later structural command `RESULT_BACKPRESSURE`, with
/// no second thread involved.
#[test]
fn a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let mut c_pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: c_pcm.as_mut_ptr(),
        sample_capacity: c_pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    let structural = |request_id: u64, revision: u64| {
        let edit = rebuild_edit(SESSION, u8::try_from(request_id).expect("small request ID"));
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
        )
    };
    let mut request = structural(1, 42);
    assert_eq!(command_c(c_session, &request).0, crate::RESULT_OK);

    for round in 1..=4_u64 {
        let next_sample = round * 128;
        let (reliable, _) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
        let (revision_before, _, provider_before, pending_before) =
            crate::ffi::test_session_state_summary(c_session);
        assert_eq!(pending_before, 1, "round {round}: a candidate is published");
        let rejected = crate::ffi::test_render(c_plan, next_sample + 1, &output);
        let (lossy, _) = event_c(c_session, crate::EVENT_LANE_LOSSY);
        request = structural(round + 1, 42 + round);
        let (window, canary) = command_c(c_session, &request);
        let (revision, _, provider, pending) = crate::ffi::test_session_state_summary(c_session);
        let good = crate::ffi::test_render(c_plan, next_sample, &output);
        let (after, _) = command_c(c_session, &request);
        assert_eq!(
            (
                reliable,
                rejected,
                lossy,
                window,
                canary == vec![0xa5; 4_096],
                revision,
                provider,
                pending,
                good,
                after,
            ),
            (
                crate::RESULT_OK,
                crate::RESULT_RENDER_REJECTED,
                crate::RESULT_OK,
                crate::RESULT_BACKPRESSURE,
                true,
                revision_before,
                provider_before + 1,
                0,
                crate::RESULT_OK,
                crate::RESULT_OK,
            ),
            "round {round}: control after a rejected, swapping render call"
        );
    }
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes() {
    const RESPONSES: [&str; 8] = [
        "4d49534f43544c000100000030000200090000005800000001000000000000002a00000000000000060000000000000001000d010400000001000000000000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000010000000000000006000101010000000100000000000000",
        "4d49534f43544c000100000030000200080000003000000002000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000",
        "4d49534f43544c000100000030000200060000004000000003000000000000002a00000000000000040000000000000001000201020000000100000000000000020004010800000001000000000000000300040108000000010000000000000004000401080000000200000000000000",
        "4d49534f43544c000100000030000200030000001000000004000000000000002b00000000000000010000000000000001000301040000000100000000000000",
        "4d49534f43544c000100000030000200090000005800000005000000000000002b00000000000000060000000000000001000d010400000001000000000000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
        "4d49534f43544c000100000030000200090000005800000006000000000000002b00000000000000060000000000000001000d010800000001000000020000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
        "4d49534f43544c00010000003000020001000000c801000007000000000000002b000000000000001b000000000000000100020102000000010000000000000002000201020000000000000000000000030002010200000001000000000000000400020102000000000000000000000005000401080000000010000000000000060003010400000000080000000000000700040108000000001000000000000008000101010000000400000000000000090002010200000000010000000000000a0004010800000001000000000000000b0004010800000000100000000000000c0004010800000001000000000000000d0004010800000001000000000000000e0004010800000002000000000000000f00040108000000010000000000000010000401080000001000000000000000110004010800000000200000000000001200040108000000001000000000000013000401080000008000000000000000140004010800000080000000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01160000000100020003000400050006000700080009000a000b0000001a000c010c000000018002801080208021803080000000001b00040108000000ff3f000000000000",
        "4d49534f43544c000100000030000200090000005800000008000000000000002b00000000000000060000000000000001000d01000000000200030104000000000000000000000003000d01040000000100000000000000040003010400000001000000000000000500080101000000000000000000000006000101010000000100000000000000",
    ];
    const EVENTS: [&str; 7] = [
        "4d49534f43544c000100000030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000010000000000000002000101010000000200000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000000200000000000000",
        "4d49534f43544c000100000030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000002000000000000000200040108000000040000000000000003000401080000002a0000000000000004000301040000000100000000000000",
        "4d49534f43544c000100000030000300028000006000000000000000000000002b000000000000000600000000000000010004010800000003000000000000000200040108000000030000000000000003000201020000000100000000000000040001010100000001000000000000000500040108000000020000000000000006000400080000000000000000000000",
        "4d49534f43544c000100000030000300308000006000000000000000000000002b00000000000000010000000000000001000b015800000004000000000000000100090114000000636170692e72656e6465722e616374697669747900000000020001010100000001000000000000000600040008000000800000000000000007000400080000000100000000000000",
        "4d49534f43544c000100000030000300208000004800000000000000000000002b00000000000000040000000000000001000401080000008000000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010001000000000000000000",
        "4d49534f43544c000100000030000300208000004800000000000000000000002b00000000000000040000000000000001000401080000008001000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010001000000000000000000",
        "4d49534f43544c000100000030000300218000004000000000000000000000002b0000000000000002000000000000000100040108000000000200000000000002000b012800000002000000000000000100030104000000010000000000000002000401080000000400000000000000",
    ];
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let eager_capacities = crate::ffi::test_retained_capacities(c_session);
    let revision = SessionRevision(42);
    let configuration = protocol::TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let configure = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    );
    let (c_result, c_response) = command_c(c_session, &configure);
    assert_eq!(c_result, crate::RESULT_OK);
    assert_eq!(c_response, pinned_hex(RESPONSES[0]));

    let transport = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::TransportSet(protocol::TransportSetRequest {
            state: protocol::TransportState::Playing,
            position: Some(protocol::SampleTime(0)),
        }),
    );
    assert_eq!(
        command_c(c_session, &transport),
        (crate::RESULT_OK, pinned_hex(RESPONSES[1]))
    );
    event_c_exact_retry(
        c_session,
        crate::EVENT_LANE_RELIABLE,
        &pinned_hex(EVENTS[0]),
    );

    let record = protocol::AutomationRecord {
        kind: protocol::AutomationKind::Point,
        handle: protocol::ParameterHandle(5),
        start: protocol::SampleTime(1),
        end: protocol::SampleTime(1),
        start_value: 100.0,
        end_value: 100.0,
    };
    let automation = command_bytes_at_revision(
        3,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::AutomationEnqueue(protocol::AutomationEnqueue {
            records: core::slice::from_ref(&record),
        }),
    );
    let (automation_result, automation_response) = command_c(c_session, &automation);
    assert_eq!(automation_result, crate::RESULT_OK);
    assert_eq!(automation_response, pinned_hex(RESPONSES[2]));
    let edit = protocol::SessionEdit::SetSessionId {
        session_id: session::StableId::parse("event-origin").expect("stable ID"),
    };
    let structural = command_bytes_at_revision(
        4,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    assert_eq!(
        command_c(c_session, &structural),
        (crate::RESULT_OK, pinned_hex(RESPONSES[3]))
    );
    for event in &EVENTS[1..=2] {
        event_c_exact_retry(c_session, crate::EVENT_LANE_RELIABLE, &pinned_hex(event));
    }

    let mut pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    event_c_exact_retry(
        c_session,
        crate::EVENT_LANE_RELIABLE,
        &pinned_hex(EVENTS[3]),
    );

    let quiet_meter_configuration = protocol::TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let disable_diagnostics = command_bytes_at_revision(
        5,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&quiet_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &disable_diagnostics),
        (crate::RESULT_OK, pinned_hex(RESPONSES[4]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 128, &output),
        crate::RESULT_OK
    );
    let expanded_meter_configuration = protocol::TelemetryConfiguration {
        meter_handles: vec![1, 2],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let expand_meters = command_bytes_at_revision(
        6,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&expanded_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &expand_meters),
        (crate::RESULT_OK, pinned_hex(RESPONSES[5]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 256, &output),
        crate::RESULT_OK
    );
    let collect_third_render = command_bytes(7, protocol::CommandPayload::CapabilitiesGet);
    assert_eq!(
        command_c(c_session, &collect_third_render),
        (crate::RESULT_OK, pinned_hex(RESPONSES[6]))
    );
    assert_eq!(
        crate::ffi::test_telemetry_counters(c_session),
        protocol::TelemetryCounters {
            telemetry_coalesced: 1,
            telemetry_dropped: 1,
        }
    );
    for event in &EVENTS[4..=5] {
        event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &pinned_hex(event));
    }

    let counter_configuration = protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: vec![protocol::CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let configure_counters = command_bytes_at_revision(
        8,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&counter_configuration),
    );
    assert_eq!(
        command_c(c_session, &configure_counters),
        (crate::RESULT_OK, pinned_hex(RESPONSES[7]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 384, &output),
        crate::RESULT_OK
    );
    event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &pinned_hex(EVENTS[6]));

    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_RELIABLE),
        (crate::RESULT_OK, Vec::new())
    );
    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_LOSSY),
        (crate::RESULT_OK, Vec::new())
    );
    assert_eq!(
        crate::ffi::test_retained_capacities(c_session),
        eager_capacities
    );
    crate::ffi::test_session_destroy(c_session);
    crate::ffi::test_plan_destroy(c_plan);
}

#[test]
fn plan_first_destroy_guards_structural_publication_without_visible_mutation() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    crate::ffi::test_plan_destroy(c_plan);
    let edit = rebuild_edit(SESSION, 0x01);
    let request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    let before = crate::ffi::test_session_state_summary(c_session);
    let (short_result, required, short_canary) = command_c_capacity(c_session, &request, 0);
    assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(required, 4_096);
    assert!(short_canary.is_empty());
    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
    let (result, canary) = command_c(c_session, &request);
    assert_eq!(result, crate::RESULT_BACKPRESSURE);
    assert_eq!(canary, vec![0xa5; 4_096]);
    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_RELIABLE),
        (crate::RESULT_OK, Vec::new())
    );
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits() {
    use TestStructuralFaultPhase::{
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterRuntimePrepare,
        BeforeProtocolCommit, BeforeRuntimePrepare,
    };

    const PHASES: [TestStructuralFaultPhase; 6] = [
        AfterProtocolPrepare,
        BeforeRuntimePrepare,
        AfterRuntimePrepare,
        AfterAdmission,
        AfterPlanReservation,
        BeforeProtocolCommit,
    ];
    fn accumulate_fault(counters: &mut TestOwnerCounters, phase: TestStructuralFaultPhase) {
        counters.token_constructed += 1;
        counters.token_disposed += 1;
        counters.replay_candidate_constructed += 1;
        counters.replay_candidate_disposed += 1;
        if matches!(
            phase,
            AfterRuntimePrepare | AfterAdmission | AfterPlanReservation | BeforeProtocolCommit
        ) {
            counters.candidate_provider_constructed += 1;
            counters.candidate_provider_disposed += 1;
            counters.candidate_plan_constructed += 1;
            counters.candidate_plan_disposed += 1;
        }
        if matches!(phase, AfterPlanReservation | BeforeProtocolCommit) {
            counters.reservation_constructed += 1;
            counters.reservation_canceled += 1;
        }
    }
    fn accumulate_success(counters: &mut TestOwnerCounters) {
        counters.token_constructed += 1;
        counters.token_disposed += 1;
        counters.replay_candidate_constructed += 1;
        counters.replay_candidate_published += 1;
        counters.replay_current_disposed += 1;
        counters.candidate_provider_constructed += 1;
        counters.candidate_provider_published += 1;
        counters.candidate_plan_constructed += 1;
        counters.candidate_plan_published += 1;
        counters.reservation_constructed += 1;
        counters.reservation_committed += 1;
    }

    for first in PHASES {
        for second in PHASES {
            crate::ffi::test_reset_lifecycle_observer();
            let (c_session, c_plan) = boxed_c_children(SESSION);
            crate::ffi::test_set_structural_faults(c_session, [Some(first), Some(second)]);
            let edit = rebuild_edit(SESSION, 0x01);
            let request = command_bytes_at_revision(
                1,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
            );
            let before = crate::ffi::test_transaction_snapshot(c_session);
            let plan_before = crate::ffi::test_plan_snapshot(c_plan);
            let mut expected = TestOwnerCounters {
                current_provider_constructed: 1,
                current_plan_constructed: 1,
                replay_current_constructed: 1,
                ..TestOwnerCounters::default()
            };
            for phase in [first, second] {
                let (result, canary) = command_c(c_session, &request);
                assert_eq!(result, crate::RESULT_BACKPRESSURE, "{first:?}/{second:?}");
                assert_eq!(canary, vec![0xa5; 4_096]);
                assert_eq!(
                    crate::ffi::test_transaction_snapshot(c_session),
                    before,
                    "canonical/model/epochs/replay/events/resources/credits {first:?}/{second:?}"
                );
                assert_eq!(
                    crate::ffi::test_plan_snapshot(c_plan),
                    plan_before,
                    "PCM boundary and plan resources {first:?}/{second:?}"
                );
                assert_eq!(
                    event_c(c_session, crate::EVENT_LANE_RELIABLE),
                    (crate::RESULT_OK, Vec::new())
                );
                accumulate_fault(&mut expected, phase);
                assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            }

            let (result, response) = command_c(c_session, &request);
            assert_eq!(result, crate::RESULT_OK);
            assert_ne!(response, vec![0xa5; 4_096]);
            accumulate_success(&mut expected);
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            assert_eq!(
                crate::ffi::test_session_state_summary(c_session),
                (43, 1, 0, 1)
            );

            let mut pcm = [f32::NAN; 256];
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: 128,
                plane_stride_samples: 128,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(c_plan, 0, &output),
                crate::RESULT_OK
            );
            assert_eq!(
                event_c(c_session, crate::EVENT_LANE_RELIABLE).0,
                crate::RESULT_OK
            );
            expected.current_provider_disposed += 1;
            expected.current_plan_disposed += 1;
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);

            let retry_edit = rebuild_edit(SESSION, 0x02);
            let retry = command_bytes_at_revision(
                2,
                ExpectedRevision::Exact(SessionRevision(43)),
                protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                    &retry_edit,
                )),
            );
            assert_eq!(command_c(c_session, &retry).0, crate::RESULT_OK);
            accumulate_success(&mut expected);
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            assert_eq!(
                crate::ffi::test_session_state_summary(c_session),
                (44, 2, 1, 1),
                "reclaim released publication and retirement credit"
            );
            assert_eq!(
                crate::ffi::test_render(c_plan, 128, &output),
                crate::RESULT_OK
            );
            let collect = command_bytes(3, protocol::CommandPayload::CapabilitiesGet);
            assert_eq!(command_c(c_session, &collect).0, crate::RESULT_OK);
            expected.current_plan_disposed += 1;
            expected.candidate_provider_disposed += 1;
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            crate::ffi::test_plan_destroy(c_plan);
            expected.current_plan_disposed += 1;
            crate::ffi::test_session_destroy(c_session);
            expected.candidate_provider_disposed += 1;
            expected.replay_current_disposed += 1;
            assert_eq!(crate::ffi::test_lifecycle_counters(), expected);
        }
    }
}

#[test]
fn capi_controller_dispatches_every_advertised_command_family() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let eager_capacities = crate::ffi::test_retained_capacities(c_session);
    assert!(eager_capacities.iter().all(|capacity| *capacity > 0));
    let codec = ProtocolCodec::default();
    let mut request_id = 0_u64;
    macro_rules! dispatch {
        ($expected:expr, $payload:expr, $status:expr, $revision:expr, $events:expr) => {{
            request_id += 1;
            let mut request = vec![0_u8; 4_096];
            let len = codec
                .encode_command_frame_into(
                    &protocol::TypedCommandFrame {
                        request_id: RequestId::new(request_id).expect("request ID"),
                        expected_revision: $expected,
                        payload: $payload,
                    },
                    &mut request,
                )
                .expect("command frame");
            request.truncate(len);
            let pinned = pinned_hex(ALL_COMMAND_RESPONSE_VECTORS[request_id as usize - 1]);
            let (c_result, c_bytes) = command_c(c_session, &request);
            assert_eq!(c_result, crate::RESULT_OK, "C command {request_id}");
            assert_eq!(c_bytes, pinned, "pinned C bytes {request_id}");

            let (c_replay_result, c_replay) = command_c(c_session, &request);
            assert_eq!(c_replay_result, crate::RESULT_OK, "C replay {request_id}");
            assert_eq!(c_replay, pinned, "pinned replay bytes {request_id}");
            let mut fields = [0_u16; 512];
            let response = codec
                .decode_typed_response(&c_bytes, &mut DecodeScratch::new(&mut fields))
                .expect("typed response");
            let header = match response {
                protocol::DecodedTypedResponseFrame::Success { header, .. }
                | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
            };
            assert_eq!(header.status, $status, "accepted status {request_id}");
            assert_eq!(header.revision, SessionRevision($revision));
            let mut event_ids = Vec::new();
            loop {
                let (event_result, c_event) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
                assert_eq!(event_result, crate::RESULT_OK);
                if c_event.is_empty() {
                    break;
                }
                let mut event_fields = [0_u16; 64];
                event_ids.push(
                    codec
                        .decode_typed_event(&c_event, &mut DecodeScratch::new(&mut event_fields))
                        .expect("typed command event")
                        .header
                        .message_id,
                );
            }
            assert_eq!(
                event_c(c_session, crate::EVENT_LANE_RELIABLE),
                (crate::RESULT_OK, Vec::new())
            );
            assert_eq!(event_ids.as_slice(), $events, "events {request_id}");
            let summary = crate::ffi::test_session_state_summary(c_session);
            assert_eq!(summary.0, $revision as u64);
            assert_eq!(summary.1, request_id as usize);
            assert_eq!(summary.2, 0, "pinned active provider epoch");
            assert_eq!(
                summary.3,
                usize::from(request_id == 11),
                "pinned pending provider count"
            );
            header.message_id
        }};
    }

    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::CapabilitiesGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::CapabilitiesGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                offset: 0,
                maximum_bytes: 1,
            },),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::SessionSnapshotGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::ParameterMetadataGet(protocol::ParameterMetadataRequest {
                after_handle: 0,
                limit: 1,
            },),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::ParameterMetadataGet
    );
    let state = protocol::ParameterStateRequest { handles: vec![1] };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::ParameterStateGet(&state),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::ParameterStateGet
    );
    let automation = [protocol::AutomationRecord {
        kind: protocol::AutomationKind::Point,
        handle: protocol::ParameterHandle(1),
        start: protocol::SampleTime(1),
        end: protocol::SampleTime(1),
        start_value: 0.0,
        end_value: 0.0,
    }];
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::AutomationEnqueue(protocol::AutomationEnqueue {
                records: &automation,
            },),
            StatusCode::InvalidField,
            42,
            &[]
        ),
        protocol::MessageId::AutomationEnqueue
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::TransportGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::TransportGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::TransportSet(protocol::TransportSetRequest {
                state: protocol::TransportState::Playing,
                position: Some(protocol::SampleTime(0)),
            },),
            StatusCode::Ok,
            42,
            &[protocol::MessageId::TransportState]
        ),
        protocol::MessageId::TransportSet
    );
    let telemetry = protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::TelemetryConfigure(&telemetry),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::TelemetryConfigure
    );
    let counters = protocol::CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::CountersGet(&counters),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::CountersGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::DiagnosticsGet(protocol::DiagnosticsRequest {
                after_sequence: 0,
                limit: 1,
                minimum_severity: protocol::DiagnosticSeverity::Info,
            },),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::DiagnosticsGet
    );
    let structural = rebuild_edit(SESSION, 0x01);
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&structural),),
            StatusCode::Ok,
            43,
            &[protocol::MessageId::SessionCommitted]
        ),
        protocol::MessageId::SessionTransactionApply
    );
    assert_eq!(request_id, 11);
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 11, 0, 1),
        "pinned revision/replay/provider/pending vector"
    );
    assert_eq!(
        crate::ffi::test_retained_capacities(c_session),
        eager_capacities
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact() {
    const REUSE: &str = "4d49534f43544c000100000030000200070009004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const EXPIRED: &str = "4d49534f43544c00010000003000020001000a004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const STALE: &str = "4d49534f43544c000100000030000200030007004800000013000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const TRANSPORT_20: &str = "4d49534f43544c000100000030000200080000003000000014000000000000002a000000000000000300000000000000010001010100000001000000000000000200040108000000000000000000000003000401080000000000000000000000";
    const TRANSPORT_21: &str = "4d49534f43544c000100000030000200080000003000000015000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000";
    const EVENT_FULL: &str = "4d49534f43544c00010000003000020003000b00b000000016000000000000002a00000000000000030000000000000001000b01380000000200000000000000010009011500000070726f746f636f6c2e6261636b7072657373757265000000020001010100000003000000000000000200030104000000000000000000000003000b005800000005000000000000000100010101000000040000000000000002000401080000000200000000000000030004010800000002000000000000000400020102000000010000000000000005000400080000000400000000000000";
    const TRANSPORT_EVENT_20: &str = "4d49534f43544c000100000030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000010000000000000002000101010000000100000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000001400000000000000";
    const TRANSPORT_EVENT_21: &str = "4d49534f43544c000100000030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000020000000000000002000101010000000200000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000001500000000000000";
    const STRUCTURAL_23: &str = "4d49534f43544c000100000030000200030000001000000017000000000000002b00000000000000010000000000000001000301040000000100000000000000";
    const COMMIT_EVENT: &str = "4d49534f43544c000100000030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000003000000000000000200040108000000170000000000000003000401080000002a0000000000000004000301040000000100000000000000";
    const RETRY_24: &str = "4d49534f43544c000100000030000200030000001000000018000000000000002c00000000000000010000000000000001000301040000000100000000000000";
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let codec = ProtocolCodec::default();
    let capabilities_response = |request_id: u64| {
        let mut bytes = pinned_hex(ALL_COMMAND_RESPONSE_VECTORS[0]);
        bytes[24..32].copy_from_slice(&request_id.to_le_bytes());
        bytes
    };
    macro_rules! dispatch {
        ($request:expr, $status:expr, $expected:expr) => {{
            let request = $request;
            let (result, bytes) = command_c(c_session, &request);
            assert_eq!(result, crate::RESULT_OK);
            assert_eq!(bytes, $expected);
            let mut fields = [0_u16; 64];
            let decoded = codec
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
                .expect("typed decision");
            let header = match decoded {
                protocol::DecodedTypedResponseFrame::Success { header, .. }
                | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
            };
            assert_eq!(header.status, $status);
            bytes
        }};
    }

    let first = command_bytes(1, protocol::CommandPayload::CapabilitiesGet);
    let first_bytes = dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1));
    assert_eq!(
        dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1)),
        first_bytes
    );
    let conflict = command_bytes(1, protocol::CommandPayload::TransportGet);
    dispatch!(conflict, StatusCode::RequestIdReuse, pinned_hex(REUSE));
    for request_id in 2..=18 {
        dispatch!(
            command_bytes(request_id, protocol::CommandPayload::CapabilitiesGet),
            StatusCode::Ok,
            capabilities_response(request_id)
        );
    }
    dispatch!(first, StatusCode::ReplayExpired, pinned_hex(EXPIRED));

    let edit = rebuild_edit(SESSION, 0x01);
    let stale = command_bytes_at_revision(
        19,
        ExpectedRevision::Exact(SessionRevision(41)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(stale, StatusCode::RevisionConflict, pinned_hex(STALE));

    for (request_id, state, expected) in [
        (20, protocol::TransportState::Stopped, TRANSPORT_20),
        (21, protocol::TransportState::Playing, TRANSPORT_21),
    ] {
        dispatch!(
            command_bytes_at_revision(
                request_id,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::TransportSet(protocol::TransportSetRequest {
                    state,
                    position: Some(protocol::SampleTime(0)),
                },),
            ),
            StatusCode::Ok,
            pinned_hex(expected)
        );
    }
    let event_full = command_bytes_at_revision(
        22,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(event_full, StatusCode::Backpressure, pinned_hex(EVENT_FULL));
    for expected in [TRANSPORT_EVENT_20, TRANSPORT_EVENT_21] {
        let (result, bytes) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(bytes, pinned_hex(expected));
    }

    let first_structural = command_bytes_at_revision(
        23,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(first_structural, StatusCode::Ok, pinned_hex(STRUCTURAL_23));
    let commit_event = event_c(c_session, crate::EVENT_LANE_RELIABLE);
    assert_eq!(commit_event.0, crate::RESULT_OK);
    assert_eq!(commit_event.1, pinned_hex(COMMIT_EVENT));

    let second_edit = rebuild_edit(SESSION, 0x02);
    let publication_full = command_bytes_at_revision(
        24,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&second_edit)),
    );
    let before = crate::ffi::test_session_state_summary(c_session);
    let before_owners = crate::ffi::test_owner_counters(c_session);
    let (result, canary) = command_c(c_session, &publication_full);
    assert_eq!(result, crate::RESULT_BACKPRESSURE);
    assert_eq!(canary, vec![0xa5; 4_096]);
    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
    let mut canceled = before_owners;
    canceled.token_constructed += 1;
    canceled.token_disposed += 1;
    canceled.replay_candidate_constructed += 1;
    canceled.replay_candidate_disposed += 1;
    canceled.candidate_provider_constructed += 1;
    canceled.candidate_provider_disposed += 1;
    canceled.candidate_plan_constructed += 1;
    canceled.candidate_plan_disposed += 1;
    assert_eq!(crate::ffi::test_owner_counters(c_session), canceled);
    assert_eq!(before.0, 43);
    assert_eq!(before.3, 1);

    let mut c_pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: c_pcm.as_mut_ptr(),
        sample_capacity: c_pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    assert!(c_pcm.iter().all(|sample| sample.to_bits() == 0));
    let retry = dispatch!(publication_full, StatusCode::Ok, pinned_hex(RETRY_24));
    assert!(!retry.is_empty());
    canceled.token_constructed += 1;
    canceled.token_disposed += 1;
    canceled.replay_candidate_constructed += 1;
    canceled.replay_candidate_published += 1;
    canceled.replay_current_disposed += 1;
    canceled.candidate_provider_constructed += 1;
    canceled.candidate_provider_published += 1;
    canceled.candidate_plan_constructed += 1;
    canceled.candidate_plan_published += 1;
    canceled.reservation_constructed += 1;
    canceled.reservation_committed += 1;
    canceled.current_provider_disposed += 1;
    canceled.current_plan_disposed += 1;
    assert_eq!(crate::ffi::test_owner_counters(c_session), canceled);
    let after_retry = crate::ffi::test_session_state_summary(c_session);
    assert_eq!(after_retry.0, 44);
    assert_eq!(after_retry.2, 1);
    assert_eq!(after_retry.3, 1);

    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// The parity session stripped to a chain whose builtin tail is finite: no console slot, no
/// insert, and both input filters off. Only such a session tells a live input lane apart, because
/// the input lane makes a strip's builtin tail infinite (#1256 gate 1).
fn bare_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&generated_parity_session(track_count, sample_rate_hz))
        .expect("accepted parity session");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
    }
    session::canonical_session_json(&model).expect("canonical bare session")
}

/// The ten-track parity session with mixed effect cohorts (#1263 verdict NIT 3): an enabled
/// compressor insert on the first and sixth tracks, and a bypassed delay and a bypassed multiband
/// compressor insert on the second. Only an enabled non-EQ effect tells a non-target lane seeded
/// bypassed apart, and the delay and the multiband keep their prepared bypass.
fn mixed_effect_parity_session(sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&generated_parity_session(10, sample_rate_hz))
        .expect("accepted parity session");
    let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    let native = |instance: &str, effect: &str, bypass: bool| {
        let mut native = template.clone();
        native.id = session::StableId::parse(instance).expect("instance ID");
        native.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse(effect).expect("effect ID"),
        };
        native.params.clear();
        native.bypass = bypass;
        native
    };
    for track in [0, 5] {
        model.tracks[track]
            .inserts
            .effects
            .push(native("comp", "miso.compressor", false));
    }
    model.tracks[1].inserts.effects.extend([
        native("dly", "miso.delay", true),
        native("mb", "miso.multiband-compressor", true),
    ]);
    session::canonical_session_json(&model).expect("canonical mixed-effect session")
}

/// One C ABI plan against a lanes-free `host_core::prepare_host_runtime` plan of the same session
/// (#1256 gate 1), and against the C ABI's lane selection without its effect lanes (#1263 gate 1).
fn assert_live_lanes_render_like_lanes_free(document: &str, label: &str, expect_finite: bool) {
    let mut c = compile_children(document, limits()).unwrap_or_else(|failure| {
        panic!(
            "{label}: compile: {}",
            String::from_utf8_lossy(&failure.diagnostics)
        )
    });
    let caps = prepare_caps(limits());
    let compiled = host_core::compile_host_session(document, &caps)
        .unwrap_or_else(|_| panic!("{label}: host compile"));
    let host_core::PreparedHost {
        mut plan,
        mut sources,
        report: host,
        ..
    } = host_core::prepare_host_runtime(&compiled, &caps)
        .unwrap_or_else(|_| panic!("{label}: lanes-free host prepare"));

    // Same latency and tail as the lanes-free plan.
    let resources = c.plan.resources();
    let (tail_kind, tail_samples) = match host.output_tail {
        TailSamples::Finite(samples) => (TAIL_FINITE, samples),
        TailSamples::Infinite => (TAIL_INFINITE, 0),
    };
    if expect_finite {
        assert_eq!(
            tail_kind, TAIL_FINITE,
            "{label}: the lanes-free tail is finite"
        );
    }
    assert_eq!(resources.latency_samples, host.latency_samples, "{label}");
    assert_eq!(resources.tail_kind, tail_kind, "{label}: tail kind");
    assert_eq!(
        resources.tail_samples, tail_samples,
        "{label}: tail samples"
    );

    // #1263 gate 1: attaching the effect lanes moves neither latency nor tail. The reference is
    // the C ABI's selection less its effect lanes, prepared at the same depth.
    let (without_effects, _) = host_core::prepare_host_runtime_with_live_lanes(
        &compiled,
        &caps,
        &HostLiveControlRequest {
            control_queue_depth: Some(LIVE_QUEUE_DEPTH),
            ..HostLiveControlRequest::default()
        },
        HostLiveLanes {
            effects: false,
            ..C_ABI_LIVE_LANES
        },
    )
    .unwrap_or_else(|_| panic!("{label}: host prepare without effect lanes"));
    let without_effects = without_effects.report;
    assert_eq!(
        resources.latency_samples, without_effects.latency_samples,
        "{label}: latency without effect lanes"
    );
    assert_eq!(
        (resources.tail_kind, resources.tail_samples),
        match without_effects.output_tail {
            TailSamples::Finite(samples) => (TAIL_FINITE, samples),
            TailSamples::Infinite => (TAIL_INFINITE, 0),
        },
        "{label}: tail without effect lanes"
    );

    // #1263 gate 1: one effect producer per prepared effect instance -- every strip's console
    // slots and inserts -- and an owner exactly on the parametric EQs.
    let normalized = compiled.normalized_model();
    let mut expected = normalized
        .strips()
        .flat_map(|strip| {
            let racks = normalized.lower_strip(&strip);
            let id = strip.id.as_str().to_owned();
            racks
                .in_chain_order()
                .into_iter()
                .flatten()
                .map(|effect| {
                    let native = match &effect.identity {
                        session::EffectIdentity::Native { effect_id } => effect_id.as_str(),
                        _ => panic!("{label}: a native effect"),
                    };
                    (
                        id.clone(),
                        effect.id.as_str().to_owned(),
                        native == "miso.parametric-eq",
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut kept = c
        .session
        .test_providers()
        .test_effects()
        .iter()
        .map(|producer| {
            (
                producer.track_id.to_string(),
                producer.effect_id.to_string(),
                producer.has_owner(),
            )
        })
        .collect::<Vec<_>>();
    expected.sort();
    kept.sort();
    assert_eq!(
        kept, expected,
        "{label}: one producer per effect instance, owners on the EQs"
    );

    // One producer per strip, in canonical strip order, with no input lane.
    let model = compiled.normalized_model();
    let strips = c.session.test_providers();
    assert_eq!(strips.test_track_count(), model.tracks.len(), "{label}");
    assert_eq!(
        strips
            .test_strip_controls()
            .iter()
            .map(|producer| &*producer.track_id)
            .collect::<Vec<_>>(),
        model
            .strips()
            .map(|strip| strip.id.as_str())
            .collect::<Vec<_>>(),
        "{label}: one producer per strip, in canonical order"
    );
    assert!(
        strips
            .test_strip_controls()
            .iter()
            .all(|producer| producer.input.is_none()),
        "{label}: no strip carries an input lane"
    );

    // Eight blocks fed the same source chunks are bit-identical.
    let rate = compiled.sample_rate().0;
    let quantum = 128_usize;
    let mut first_left = vec![0.0_f32; quantum];
    let mut first_right = vec![0.0_f32; quantum];
    first_left[1] = 0.25;
    first_right[1] = -0.5;
    first_left[7] = -1.0;
    first_right[9] = 0.75;
    let final_left = vec![0.125_f32; 64];
    let final_right = vec![-0.25_f32; 64];
    let mut audible = false;
    for block in 0..8_u64 {
        let chunk = match block {
            0 => Some((0, &first_left, &first_right, false)),
            1 => Some((128, &final_left, &final_right, true)),
            _ => None,
        };
        if let Some((start_frame, left, right, end_of_region)) = chunk {
            let planes = [left.as_slice(), right.as_slice()];
            let submission = || SourceSubmission {
                generation: 1,
                start_frame,
                sample_rate_hz: rate,
                planes: &planes,
                frames: left.len() as u32,
                end_of_region,
            };
            c.session
                .submit(b"fixture-source", submission())
                .unwrap_or_else(|_| panic!("{label}: C submit"));
            sources
                .submit(b"fixture-source", submission())
                .unwrap_or_else(|_| panic!("{label}: host submit"));
        }
        let mut c_pcm = vec![f32::NAN; quantum * 2];
        c.plan
            .render(
                block * quantum as u64,
                PlanarBufferMut::try_new(&mut c_pcm, 2, quantum, quantum).expect("C output"),
            )
            .unwrap_or_else(|code| panic!("{label}: C render {code:?}"));
        let mut host_pcm = vec![f32::NAN; quantum * 2];
        plan.render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut host_pcm, 2, quantum, quantum)
                    .expect("host output"),
            },
            block * quantum as u64,
        )
        .unwrap_or_else(|_| panic!("{label}: host render"));
        assert_eq!(
            c_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            host_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "{label}: block {block}"
        );
        audible |= c_pcm.iter().any(|sample| *sample != 0.0);
    }
    assert!(audible, "{label}: the compared blocks carry signal");
}

#[test]
fn c_abi_plans_with_live_lanes_render_like_lanes_free_plans() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        // #1263 gate 1: the nine-track EQ fixture itself, at the helper's 192-frame source.
        let mut nine = parse_session_json(SESSION).expect("accepted nine-track fixture");
        nine.sample_rate_hz = sample_rate_hz;
        nine.sources[0].frames = 192;
        assert_live_lanes_render_like_lanes_free(
            &session::canonical_session_json(&nine).expect("canonical nine-track session"),
            &format!("nine-track EQ fixture at {sample_rate_hz} Hz"),
            false,
        );
        for track_count in [1, 10] {
            assert_live_lanes_render_like_lanes_free(
                &generated_parity_session(track_count, sample_rate_hz),
                &format!("parity {track_count} tracks at {sample_rate_hz} Hz"),
                false,
            );
            assert_live_lanes_render_like_lanes_free(
                &bare_parity_session(track_count, sample_rate_hz),
                &format!("bare {track_count} tracks at {sample_rate_hz} Hz"),
                true,
            );
        }
        assert_live_lanes_render_like_lanes_free(
            &mixed_effect_parity_session(sample_rate_hz),
            &format!("mixed effects 10 tracks at {sample_rate_hz} Hz"),
            false,
        );
    }
}

#[test]
fn direct_and_c_render_match_one_and_ten_tracks_across_launch_rates() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        render_parity_shape(1, sample_rate_hz);
        render_parity_shape(10, sample_rate_hz);
    }
}

/// How long one side of the lockstep below waits for the other before it fails. Each step takes
/// one block's submit or render, microseconds in a debug build.
const LOCKSTEP_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

/// Waits until `progress` reaches `target`, for the lockstep below.
///
/// # Panics
///
/// When the peer has ended (normally or by a panic: its `StopOnDrop` set `peer_ended`) short of
/// `target`, or after [`LOCKSTEP_DEADLINE`]; the message names `what` this side waited for and the
/// block, `target - 1` (#1251). The message is formatted only on failure, so the wait allocates
/// nothing on its passing path.
fn await_lockstep(
    progress: &std::sync::atomic::AtomicU64,
    target: u64,
    peer_ended: &std::sync::atomic::AtomicBool,
    what: &'static str,
) {
    use std::sync::atomic::Ordering;
    let deadline = std::time::Instant::now() + LOCKSTEP_DEADLINE;
    loop {
        // Read the peer's end before its progress: a peer that ended published its last
        // progress first, so an ended peer short of `target` will never reach it.
        let ended = peer_ended.load(Ordering::Acquire);
        if progress.load(Ordering::Acquire) >= target {
            return;
        }
        let block = target - 1;
        assert!(
            !ended,
            "lockstep: the peer ended before {what} block {block}"
        );
        assert!(
            std::time::Instant::now() < deadline,
            "lockstep: {what} block {block} did not happen within {LOCKSTEP_DEADLINE:?}"
        );
        std::thread::yield_now();
    }
}

#[test]
fn barrier_schedule_separates_one_source_producer_from_exclusive_render() {
    let mut model =
        parse_session_json(&generated_parity_session(1, 48_000)).expect("concurrency session");
    model.sources[0].frames = 1_024;
    let session = session::canonical_session_json(&model).expect("canonical");
    let children = compile_children(&session, limits()).expect("concurrent children");
    let session = Box::into_raw(Box::new(crate::Session::new(
        children.session,
        children.session_error,
    ))) as usize;
    let plan = Box::into_raw(Box::new(crate::Plan::new(children.plan))) as usize;
    // A lockstep of two counters in place of two barriers: the producer submits block `b` and
    // waits until the renderer has consumed it; the renderer waits until block `b` is submitted,
    // renders it and publishes it consumed. Each side's `StopOnDrop` marks it ended however it
    // leaves its loop, so a failed assertion on one side fails the other's wait instead of
    // leaving it blocked at a barrier (#1251).
    let submitted = std::sync::atomic::AtomicU64::new(0);
    let consumed = std::sync::atomic::AtomicU64::new(0);
    let producer_ended = std::sync::atomic::AtomicBool::new(false);
    let render_ended = std::sync::atomic::AtomicBool::new(false);
    std::thread::scope(|scope| {
        let (submitted, consumed) = (&submitted, &consumed);
        let (producer_ended, render_ended) = (&producer_ended, &render_ended);
        scope.spawn(move || {
            let _ended = bench_support::producer::StopOnDrop(producer_ended);
            let session = session as *mut crate::Session;
            let left = [0.25_f32; 128];
            let right = [-0.5_f32; 128];
            for block in 0..6_u64 {
                let (generation, start_frame) = if block < 3 {
                    (1, block * 128)
                } else {
                    if block == 3 {
                        assert_eq!(
                            crate::ffi::test_source_seek(session, b"fixture-source", 2, 512,),
                            crate::RESULT_OK
                        );
                    }
                    (2, 512 + (block - 3) * 128)
                };
                submit_c(
                    session,
                    generation,
                    start_frame,
                    48_000,
                    &left,
                    &right,
                    false,
                );
                submitted.store(block + 1, std::sync::atomic::Ordering::Release);
                await_lockstep(consumed, block + 1, render_ended, "the renderer consumed");
            }
        });
        scope.spawn(move || {
            let _ended = bench_support::producer::StopOnDrop(render_ended);
            let plan = plan as *mut crate::Plan;
            let mut observed_signal = false;
            for block in 0..6_u64 {
                await_lockstep(
                    submitted,
                    block + 1,
                    producer_ended,
                    "the producer submitted",
                );
                let mut pcm = [f32::NAN; 256];
                let output = crate::PlanarOutput {
                    struct_size: crate::PLANAR_OUTPUT_SIZE,
                    channels: 2,
                    samples: pcm.as_mut_ptr(),
                    sample_capacity: pcm.len() as u64,
                    frames: 128,
                    plane_stride_samples: 128,
                    reserved: [0; 2],
                };
                assert_eq!(
                    crate::ffi::test_render(plan, block * 128, &output),
                    crate::RESULT_OK
                );
                assert!(pcm.iter().all(|sample| sample.is_finite()));
                observed_signal |= pcm.iter().any(|sample| *sample != 0.0);
                consumed.store(block + 1, std::sync::atomic::Ordering::Release);
            }
            assert!(observed_signal);
        });
    });
    crate::ffi::test_session_destroy(session as *mut crate::Session);
    crate::ffi::test_plan_destroy(plan as *mut crate::Plan);
}

/// Issue #146 at the C ABI: the render entry pins the environment and hands the caller's back.
///
/// Every DAW audio callback arrives with FTZ and DAZ already set, so this is the shape of the real
/// exposure. Three arms render the same subnormal fade tail through the same compiled session:
///
/// * the C entry with the caller's FTZ/DAZ clear -- the reference bytes;
/// * the C entry with the caller's FTZ+DAZ set -- must be the same bytes;
/// * the plan rendered *behind* the entry with FTZ+DAZ set -- must differ, or the second arm is
///   vacuous.
///
/// Red mutation (recorded in `crates/host-core/tests/MUTATIONS.md`, M-146-2): delete the
/// `CanonicalFpEnv::enter()` line from `miso_engine_v1_render_f32_planar`; the second arm collapses
/// onto the third.
#[cfg(target_arch = "x86_64")]
#[allow(unsafe_code)]
mod fp_environment {
    use super::*;
    use lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};

    const QUANTUM: usize = 128;
    const BLOCKS: u64 = 12;
    /// DAZ, the six exception masks, the rounding-control field and FTZ. The low six bits are
    /// sticky status flags that ordinary arithmetic sets and no restore is expected to hold back.
    const MXCSR_CONTROL_BITS: u32 = 0xFFC0;

    struct Restore(u32);

    impl Drop for Restore {
        fn drop(&mut self) {
            // SAFETY: `self.0` is the word the test read from this thread before it changed it, so
            // it sets no reserved bit, and writing it hands the thread its own word back.
            unsafe { write_mxcsr(self.0) };
        }
    }

    /// One block of a fade tail: every sample is a distinct `f32` subnormal, which is exactly what
    /// DAZ reads as zero and FTZ writes as zero.
    fn tail_block(block: u64) -> (Vec<f32>, Vec<f32>) {
        let mut left = vec![0.0_f32; QUANTUM];
        let mut right = vec![0.0_f32; QUANTUM];
        for (frame, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let step = (block * QUANTUM as u64 + frame as u64) as u32;
            *left = f32::from_bits(0x0004_0000_u32.wrapping_add(step.wrapping_mul(1_031)));
            *right = -f32::from_bits(0x0002_0000_u32.wrapping_add(step.wrapping_mul(613)));
        }
        (left, right)
    }

    /// Renders `BLOCKS` quanta through the C ABI render entry.
    fn render_through_the_c_entry(caller: u32) -> (Vec<u32>, u32) {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let children = compile_children(SESSION, limits()).expect("C children");
        let c_session = Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        )));
        let c_plan = Box::into_raw(Box::new(crate::Plan::new(children.plan)));

        // SAFETY: every caller of this function passes a word read from this thread with only
        // FTZ (bit 15) and DAZ (bit 6) set or cleared, so no reserved bit. The test measures the
        // render under it, and the write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(caller) };
        let mut rendered = Vec::with_capacity(BLOCKS as usize * QUANTUM * 2);
        let mut leaked = 0;
        for block in 0..BLOCKS {
            let (left, right) = tail_block(block);
            submit_c(
                c_session,
                1,
                block * QUANTUM as u64,
                48_000,
                &left,
                &right,
                false,
            );
            let mut pcm = vec![f32::NAN; QUANTUM * 2];
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: QUANTUM as u32,
                plane_stride_samples: QUANTUM as u32,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(c_plan, block * QUANTUM as u64, &output),
                crate::RESULT_OK
            );
            // E2, read the instant the entry returns: every bit of the caller's word, status flags
            // included, because the guard's restore is unconditional.
            leaked |= read_mxcsr() ^ caller;
            rendered.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
        (rendered, leaked)
    }

    /// Renders `BLOCKS` quanta straight into the plan state, behind the C entry and its guard.
    fn render_behind_the_c_entry(caller: u32) -> Vec<u32> {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let mut children = compile_children(SESSION, limits()).expect("direct children");

        // SAFETY: every caller of this function passes a word read from this thread with only
        // FTZ (bit 15) and DAZ (bit 6) set or cleared, so no reserved bit. The test measures the
        // render under it, and the write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(caller) };
        let mut rendered = Vec::with_capacity(BLOCKS as usize * QUANTUM * 2);
        for block in 0..BLOCKS {
            let (left, right) = tail_block(block);
            children
                .session
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: block * QUANTUM as u64,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("direct submit");
            let mut pcm = vec![f32::NAN; QUANTUM * 2];
            children
                .plan
                .render(
                    block * QUANTUM as u64,
                    PlanarBufferMut::try_new(&mut pcm, 2, QUANTUM, QUANTUM).expect("direct output"),
                )
                .expect("direct render");
            rendered.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        rendered
    }

    /// The refusal has its own frozen diagnostic, so a host that hits it is told which rule it
    /// broke rather than reading the catch-all.
    #[test]
    fn the_fp_environment_refusal_names_its_own_check() {
        assert_eq!(
            plan_error::text(plan_error::FP_ENVIRONMENT),
            b"render.fp_environment.invalid"
        );
        assert_ne!(
            plan_error::text(plan_error::FP_ENVIRONMENT),
            plan_error::text(u32::MAX),
            "the refusal must not fall through to render.internal"
        );
    }

    #[test]
    fn the_c_render_entry_is_canonical_under_a_caller_that_set_ftz_and_daz() {
        let saved = read_mxcsr();
        let clear = saved & !(MXCSR_FTZ | MXCSR_DAZ);
        let hostile = clear | MXCSR_FTZ | MXCSR_DAZ;

        let (reference, reference_leak) = render_through_the_c_entry(clear);
        let (guarded, guarded_leak) = render_through_the_c_entry(hostile);
        let behind = render_behind_the_c_entry(hostile);

        assert_eq!(
            reference_leak & MXCSR_CONTROL_BITS,
            0,
            "the entry must restore every control bit"
        );
        assert_eq!(
            guarded_leak, 0,
            "the entry must restore the caller's exact word, status flags included"
        );
        assert_ne!(
            behind, reference,
            "issue #146 is vacuous here: FTZ+DAZ did not move this fixture behind the entry"
        );
        assert_eq!(
            guarded, reference,
            "a caller's FTZ+DAZ reached a render through the C ABI entry"
        );
        assert_eq!(read_mxcsr(), saved, "the test leaked MXCSR state");
    }

    #[test]
    fn a_rejected_c_render_also_restores_the_callers_word() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        // Round-toward-zero and a sticky precision flag alongside FTZ+DAZ: nothing an entry may
        // keep, and nothing it may drop.
        let hostile = (saved & !0x6000) | MXCSR_FTZ | MXCSR_DAZ | 0x6000 | 0x0020;

        let children = compile_children(SESSION, limits()).expect("C children");
        let c_session = Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        )));
        let c_plan = Box::into_raw(Box::new(crate::Plan::new(children.plan)));

        let mut pcm = vec![f32::NAN; QUANTUM * 2];
        // SAFETY: `hostile` sets FTZ, DAZ, RC and a status flag on a word read from this thread,
        // all below bit 16. The test measures the C entry's rejections under it, and the
        // write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(hostile) };

        // A malformed descriptor: refused before the plan is touched at all.
        let malformed = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 3,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM as u32,
            plane_stride_samples: QUANTUM as u32,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, 0, &malformed),
            crate::RESULT_INVALID_ARGUMENT
        );
        assert_eq!(read_mxcsr(), hostile, "a refused descriptor leaked MXCSR");

        // A null plan handle: the earliest return the entry has.
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM as u32,
            plane_stride_samples: QUANTUM as u32,
            reserved: [0; 2],
        };
        assert_ne!(
            crate::ffi::test_render(core::ptr::null_mut(), 0, &output),
            crate::RESULT_OK
        );
        assert_eq!(read_mxcsr(), hostile, "a null handle leaked MXCSR");

        // A discontinuous absolute sample: rejected by the plan, after the guard is installed.
        assert_eq!(
            crate::ffi::test_render(c_plan, 7, &output),
            crate::RESULT_RENDER_REJECTED
        );
        assert_eq!(read_mxcsr(), hostile, "a rejected render leaked MXCSR");

        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }
}

/// A `SESSION_TRANSACTION_APPLY` at `revision` that rebuilds the plan ([`rebuild_edit`] with
/// `tag`).
fn rebuild_command(request_id: u64, revision: u64, tag: u8) -> Vec<u8> {
    let edit = rebuild_edit(SESSION, tag);
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(revision)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    )
}

/// Telemetry that stages one render-peak meter record per meter handle per observed render,
/// configured at the fixture's initial revision.
fn meter_telemetry(request_id: u64, meter_handles: Vec<u32>) -> Vec<u8> {
    let configuration = protocol::TelemetryConfiguration {
        meter_handles,
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    )
}

/// One 128-frame block of silence for `fixture-source` through the exported submit entry point.
fn submit_silence_c(session: *mut crate::Session, generation: u64, start_frame: u64) {
    let silence = [0.0_f32; 128];
    submit_c(
        session,
        generation,
        start_frame,
        48_000,
        &silence,
        &silence,
        false,
    );
}

/// Issue #1348 gate 1: after a committed rebuild and its swap block, `miso_engine_v1_service`
/// alone disposes of the retired plan and promotes the successor's provider, and a second rebuild
/// then reserves.
///
/// Test value: red if `service` does not reclaim or does not promote; no other call does only this
/// step, and every existing reclaim test reaches it through a command or a dequeue.
#[test]
fn service_alone_reclaims_the_retired_plan_and_promotes_its_successor() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        command_c(c_session, &rebuild_command(1, 42, 0x01)).0,
        crate::RESULT_OK
    );
    render_c(c_plan, 0);
    let before = crate::ffi::test_lifecycle_counters();
    // (revision, replay entries, current provider epoch, pending providers)
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 0, 1)
    );

    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let after = crate::ffi::test_lifecycle_counters();
    assert_eq!(
        after.current_plan_disposed,
        before.current_plan_disposed + 1,
        "service disposes of the retired plan"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 1, 0),
        "service promotes the adopted successor's provider"
    );
    assert!(crate::ffi::test_last_error(c_session).is_empty());

    assert_eq!(
        command_c(c_session, &rebuild_command(2, 43, 0x02)).0,
        crate::RESULT_OK,
        "the reclaimed credit lets the next rebuild reserve"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (44, 2, 1, 1)
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 2: a source submission runs the whole service step first: with a meter handle
/// configured, it disposes of the retired plan and stages the swap block's render-peak record.
///
/// Test value: red if a control call skips the telemetry half of the step, as the source submit
/// did before #1348 (only `command` and `dequeue_event` staged render telemetry).
#[test]
fn a_source_submit_services_the_session_first() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        command_c(c_session, &meter_telemetry(1, vec![1])).0,
        crate::RESULT_OK
    );
    assert_eq!(
        command_c(c_session, &rebuild_command(2, 42, 0x01)).0,
        crate::RESULT_OK
    );
    render_c(c_plan, 0);
    let before = crate::ffi::test_lifecycle_counters();
    let telemetry_before = crate::ffi::test_transaction_snapshot(c_session).telemetry;

    // The rebuild changed the source's content, so it restarts at generation 1, frame 0.
    submit_silence_c(c_session, 1, 0);
    let after = crate::ffi::test_lifecycle_counters();
    assert_eq!(
        after.current_plan_disposed,
        before.current_plan_disposed + 1,
        "the submit's service step disposes of the retired plan"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 2, 1, 0)
    );
    let telemetry_after = crate::ffi::test_transaction_snapshot(c_session).telemetry;
    assert_eq!(
        telemetry_after.occupancy,
        telemetry_before.occupancy + 1,
        "the submit's service step stages the swap block's render-peak record"
    );
    let (result, event) = event_c(c_session, crate::EVENT_LANE_LOSSY);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    assert!(matches!(
        ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("lossy event"),
        protocol::DecodedTypedEventFrame {
            payload: protocol::DecodedEventPayload::MeterBatch(_),
            ..
        }
    ));
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 3: the provider's telemetry counters are refreshed by every control call's
/// service step, after its staging. Drops staged by a source submission are in the provider's
/// counter snapshot as soon as the submit returns, equal to the controller's own count, and
/// `COUNTERS_GET` reports them.
///
/// Test value: red if a control call other than `command` leaves the provider's counters stale
/// (the refresh kept in `command` only), or if the step refreshes only before it stages, either of
/// which #1312 and #1351's snapshot readers would then report wrongly.
#[test]
fn every_control_call_refreshes_the_provider_counters() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    // Two meter records per observed render overflow the lossy lane's room for one stream.
    assert_eq!(
        command_c(c_session, &meter_telemetry(1, vec![1, 2])).0,
        crate::RESULT_OK
    );
    // Arm the render-peak gate the configuration enabled.
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let provider_dropped = |session: *mut crate::Session| {
        crate::ffi::test_provider_counters(session)
            .iter()
            .find(|value| value.id == protocol::CounterId::TelemetryDropped)
            .expect("telemetry-dropped counter")
            .value
    };
    for block in 0..4 {
        render_c(c_plan, block);
        submit_silence_c(c_session, 1, block * 128);
        let controller = crate::ffi::test_telemetry_counters(c_session);
        assert_eq!(
            provider_dropped(c_session),
            controller.telemetry_dropped,
            "block {block}: the submit's own service step counted its drops"
        );
    }
    let dropped = crate::ffi::test_telemetry_counters(c_session).telemetry_dropped;
    assert!(dropped > 0, "the telemetry lane dropped records");

    let request = command_bytes(
        2,
        protocol::CommandPayload::CountersGet(&protocol::CountersRequest {
            all: false,
            ids: vec![protocol::CounterId::TelemetryDropped as u32],
        }),
    );
    let (result, response) = command_c(c_session, &request);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("counters response")
    else {
        panic!("expected a counter snapshot")
    };
    assert_eq!(
        snapshot.values,
        vec![protocol::CounterValue {
            id: protocol::CounterId::TelemetryDropped,
            value: dropped,
        }]
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 4 (D7): a published candidate scheduled `NoEarlierThan` 64 blocks ahead
/// survives 1,000 service steps after a rendered block: render still adopts it, exactly at the
/// block that starts at its sample, its provider stays pending until then, no plan or provider is
/// disposed, and the retirement credit it holds comes back only through the adoption's reclaim.
///
/// Test value: red if `synchronize_plan_epochs` treats an unadopted scheduled candidate as stale
/// and disposes of it or promotes its provider early -- for example on the pre-#1311 assumption
/// that any block rendered after a publication adopted it (an acked revision's plan lost, or the
/// control plane addressing a plan render does not run). Every other reclaim test publishes for
/// the next block, where that assumption holds.
#[test]
fn a_scheduled_candidate_survives_service_until_render_adopts_it() {
    const ADOPTION_BLOCK: u64 = 64;
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    crate::ffi::test_set_next_adoption(
        c_session,
        engine::realtime::PlanAdoption::NoEarlierThan(ADOPTION_BLOCK * 128),
    );
    assert_eq!(
        command_c(c_session, &rebuild_command(1, 42, 0x01)).0,
        crate::RESULT_OK
    );
    render_c(c_plan, 0);
    let published = crate::ffi::test_lifecycle_counters();
    // (revision, replay entries, current provider epoch, pending providers)
    let summary = crate::ffi::test_session_state_summary(c_session);
    assert_eq!(summary, (43, 1, 0, 1));

    for _ in 0..1_000 {
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    }
    assert_eq!(
        crate::ffi::test_lifecycle_counters(),
        published,
        "service disposes of no plan and no provider"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        summary,
        "the candidate's provider stays pending"
    );

    for block in 1..ADOPTION_BLOCK {
        render_c(c_plan, block);
    }
    let (result, watermark) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(watermark.revision, 42, "not adopted before its block");
    render_c(c_plan, ADOPTION_BLOCK);
    let (result, watermark) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        (watermark.revision, watermark.first_sample),
        (43, ADOPTION_BLOCK * 128),
        "render adopts the candidate service kept, at the block that starts at its sample"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        summary,
        "no control call yet"
    );
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 1, 0)
    );
    let mut reclaimed = published;
    reclaimed.current_plan_disposed += 1;
    reclaimed.current_provider_disposed += 1;
    assert_eq!(crate::ffi::test_lifecycle_counters(), reclaimed);
    // The reclaim returned the candidate's retirement credit: the next rebuild reserves.
    assert_eq!(
        command_c(c_session, &rebuild_command(2, 43, 0x02)).0,
        crate::RESULT_OK
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (44, 2, 1, 1)
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 5: `miso_engine_v1_service` with nothing pending returns `OK` and changes
/// nothing, and 1,000 consecutive calls with no render change nothing either, even with telemetry
/// configured and one render observation already staged. A null or wrong handle is refused as on
/// every session call.
///
/// Test value: red if the step does work that grows with idle time or mutates state with nothing
/// pending (for example, staging the same render observation again on every call).
#[test]
fn service_with_nothing_pending_changes_nothing() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        crate::ffi::test_service(core::ptr::null_mut()),
        crate::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        crate::ffi::test_service(c_plan.cast()),
        crate::RESULT_WRONG_HANDLE
    );

    let fresh = crate::ffi::test_transaction_snapshot(c_session);
    let lifecycle = crate::ffi::test_lifecycle_counters();
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(crate::ffi::test_transaction_snapshot(c_session), fresh);
    assert_eq!(crate::ffi::test_lifecycle_counters(), lifecycle);

    assert_eq!(
        command_c(c_session, &meter_telemetry(1, vec![1])).0,
        crate::RESULT_OK
    );
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    render_c(c_plan, 0);
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let settled = crate::ffi::test_transaction_snapshot(c_session);
    assert_eq!(settled.telemetry.occupancy, 1, "one observation staged");
    let counters = crate::ffi::test_owner_counters(c_session);
    for _ in 0..1_000 {
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    }
    assert_eq!(crate::ffi::test_transaction_snapshot(c_session), settled);
    assert_eq!(crate::ffi::test_owner_counters(c_session), counters);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}
