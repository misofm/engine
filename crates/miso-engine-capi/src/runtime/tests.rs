//! Control-plane and C-boundary gates for the runtime modules.

use super::*;
use miso_engine_protocol::{ExpectedRevision, RequestId, SessionRevision, StatusCode};
use miso_engine_session::parse_session_toml;

/// Region end of the single fixture source, read through the facade's accessor.
fn source_region_end(sources: &SourceControlSet) -> u64 {
    sources
        .region(b"fixture-source")
        .expect("fixture source region")
        .end
}

const SESSION: &str = include_str!("../../../../fixtures/session/v1/parametric-eq-nine-track.toml");

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: crate::COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_toml_bytes: 1_000_000,
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
        reserved: [0; 4],
    }
}

fn command_bytes(request_id: u64, payload: miso_engine_protocol::CommandPayload<'_>) -> Vec<u8> {
    command_bytes_at_revision(request_id, ExpectedRevision::Any, payload)
}

fn command_bytes_at_revision(
    request_id: u64,
    expected_revision: ExpectedRevision,
    payload: miso_engine_protocol::CommandPayload<'_>,
) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = miso_engine_protocol::TypedCommandFrame {
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

const ALL_COMMAND_RESPONSE_VECTORS: [&str; 12] = [
    "4d49534f43544c00010001003000020001000000c801000001000000000000002a000000000000001b000000000000000100020102000000010000000000000002000201020000000000000000000000030002010200000001000000000000000400020102000000010000000000000005000401080000000010000000000000060003010400000000080000000000000700040108000000001000000000000008000101010000000400000000000000090002010200000000010000000000000a0004010800000001000000000000000b0004010800000000100000000000000c0004010800000001000000000000000d0004010800000001000000000000000e0004010800000002000000000000000f00040108000000010000000000000010000401080000001000000000000000110004010800000000200000000000001200040108000000001000000000000013000401080000008000000000000000140004010800000080000000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01180000000100020003000400050006000700080009000a000b000c001a000c010c000000018002801080208021803080000000001b00040108000000ff3f000000000000",
    "4d49534f43544c000100010030000200020000004000000002000000000000002a00000000000000040000000000000001000401080000003b250000000000000200040108000000000000000000000003000a0101000000730000000000000004000801010000000000000000000000",
    "4d49534f43544c00010001003000020004000000c802000003000000000000002a000000000000000300000000000000010003010400000001000000000000000200080101000000000000000000000003000b01a002000015000000000000000100030104000000010000000000000002000901030000006571300000000000030001010100000001000000000000000400090102000000657100000000000005000301040000000100000000000000060001010100000001000000000000000700010101000000010000000000000008000101010000000500000000000000090001010100000002000000000000000c0006010400000000000000000000000d0001010100000004000000000000000e0001010100000003000000000000000f00030104000000000000000000000010000301040000000100000000000000110009000e00000062616e642d312d656e61626c6564000012000900060000006f6e2f6f6666000014000b004800000004000000000000000100010101000000010000000000000002000601040000000000803f000000000300060104000000000000000000000004000601040000000000803f0000000014000b0048000000040000000000000001000101010000000200000000000000020006010400000000004040000000000300060104000000000000000000000004000601040000000000803f0000000014000b004800000004000000000000000100010101000000030000000000000002000601040000000000a040000000000300060104000000000000000000000004000601040000000000803f0000000014000b0048000000040000000000000001000101010000000400000000000000020006010400000000002041000000000300060104000000000000000000000004000601040000000000803f0000000014000b004800000004000000000000000100010101000000050000000000000002000601040000000000f041000000000300060104000000000000000000000004000601040000000000803f00000000",
    "4d49534f43544c000100010030000200050000004800000004000000000000002a00000000000000040000000000000001000401080000000000000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010000000000000000000000",
    "4d49534f43544c000100010030000200060000004000000005000000000000002a00000000000000040000000000000001000201020000000100000000000000020004010800000001000000000000000300040108000000010000000000000004000401080000000200000000000000",
    "4d49534f43544c000100010030000200070000003000000006000000000000002a000000000000000300000000000000010001010100000001000000000000000200040108000000000000000000000003000401080000000000000000000000",
    "4d49534f43544c000100010030000200080000003000000007000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000",
    "4d49534f43544c000100010030000200090000005000000008000000000000002a00000000000000060000000000000001000d01000000000200030104000000000000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
    "4d49534f43544c0001000100300002000a0000004000000009000000000000002a0000000000000002000000000000000100040108000000000000000000000002000b0128000000020000000000000001000301040000000c0000000000000002000401080000000100000000000000",
    "4d49534f43544c0001000100300002000b000000200000000a000000000000002a0000000000000002000000000000000100040108000000000000000000000002000801010000000100000000000000",
    "4d49534f43544c00010001003000020003000000100000000b000000000000002b00000000000000010000000000000001000301040000000100000000000000",
    "4d49534f43544c0001000100300002000c000000200000000c000000000000002c00000000000000020000000000000001000301040000000500000000000000020006010400000021adf34200000000",
];

fn generated_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_toml(SESSION).expect("accepted parity base");
    model.sample_rate_hz = sample_rate_hz;
    model.sources[0].sample_rate_hz = sample_rate_hz;
    model.sources[0].mapping.region.length_samples = 192;
    if track_count == 1 {
        model.tracks.truncate(1);
        model.routes.truncate(1);
    } else {
        assert_eq!(track_count, 10);
        let mut track = model.tracks[8].clone();
        track.id = miso_engine_session::StableId::parse("eq9").expect("tenth track");
        let effect = &mut track.simd1.effects[0];
        effect.id = miso_engine_session::StableId::parse("limiter").expect("limiter slot");
        effect.identity = miso_engine_session::EffectIdentity::Native {
            effect_id: miso_engine_session::StableId::parse("miso.true-peak-limiter")
                .expect("limiter id"),
        };
        effect.params.clear();
        effect.bypass = true;
        let mut route = model.routes[8].clone();
        route.id = miso_engine_session::StableId::parse("eq9-main").expect("tenth route");
        let miso_engine_session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("track route")
        };
        *track_id = track.id.clone();
        model.tracks.push(track);
        model.routes.push(route);
    }
    miso_engine_session::canonical_session_toml(&model).expect("canonical parity session")
}

fn submit_c(
    session: *mut crate::Session,
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
        crate::ffi::test_source_submit(session, b"fixture-source", &chunk, &mut report,),
        crate::RESULT_OK
    );
    assert_eq!(report.accepted_frames, left.len() as u64);
}

fn boxed_c_children(session: &str) -> (*mut crate::Session, *mut crate::Plan) {
    boxed_c_children_with_limits(session, limits())
}

fn boxed_c_children_with_limits(
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

fn command_c(session: *mut crate::Session, request: &[u8]) -> (u32, Vec<u8>) {
    let (result, _, storage) = command_c_capacity(session, request, 4_096);
    (result, storage)
}

fn command_c_capacity(
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

fn event_c(session: *mut crate::Session, lane: u32) -> (u32, Vec<u8>) {
    let (result, _, storage) = event_c_capacity(session, lane, 4_096);
    (result, storage)
}

fn event_c_capacity(
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
    assert_eq!(children.plan.resources().sample_rate_hz, 48_000);
    assert_eq!(children.plan.resources().quantum_frames, 128);
    assert_eq!(children.plan.resources().source_count, 1);
    assert_eq!(children.plan.resources().track_count, 9);
    assert!(children.plan.resources().graph_session_plus_plan_bytes > 0);
    assert!(children.plan.resources().source_total_bytes > 0);
    assert!(children.plan.resources().effect_scalar_state_bytes > 0);
    assert!(children.plan.resources().builtin_retained_payload_bytes > 0);
    assert!(children.plan.resources().capi_retained_bytes > 0);
    assert!(children.plan.resources().largest_named_allocation_bytes > 0);

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
    let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("capi-replaced").expect("stable ID"),
    };
    let first_request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    assert!(matches!(
        children.session.command(&first_request, 0),
        Err(CommandError::BufferTooSmall { required: 4_096 })
    ));
    assert_eq!(
        children.session.controller.session().revision(),
        SessionRevision(42)
    );
    assert_eq!(children.session.providers.epoch, 0);
    assert_eq!(children.plan.owner.active_epoch().0, 0);

    let first_len = children
        .session
        .command(&first_request, 4_096)
        .expect("first structural command");
    let first_response = children.session.command_response(first_len).to_vec();
    assert_eq!(
        children.session.controller.session().revision(),
        SessionRevision(43)
    );
    assert_eq!(children.session.providers.epoch, 0);
    assert_eq!(children.session.pending_providers[0].epoch, 1);
    assert_eq!(children.plan.owner.active_epoch().0, 0);
    assert_eq!(children.session.controller.replay().len(), 1);
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
        .expect("submission remains routed to old committed provider before boundary");

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
        miso_engine_protocol::DecodedTypedEventFrame {
            header,
            payload: miso_engine_protocol::DecodedEventPayload::SessionCommitted(_),
        } if header.revision == SessionRevision(43)
    ));
    assert_eq!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 0)
            .expect("empty reliable lane"),
        None
    );

    let model = parse_session_toml(SESSION).expect("source-changing model");
    let mut mapping = model.sources[0].mapping.clone();
    mapping.region.length_samples = 512;
    let second_edit = miso_engine_protocol::SessionEditV1::SetSourceMapping {
        source_id: model.sources[0].id.clone(),
        mapping,
    };
    let second_request = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(43)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
            &second_edit,
        )),
    );
    assert!(matches!(
        children.session.command(&second_request, 4_096),
        Err(CommandError::Backpressure)
    ));
    assert_eq!(
        children.session.controller.session().revision(),
        SessionRevision(43)
    );
    assert_eq!(children.session.controller.replay().len(), 1);

    pcm.fill(f32::NAN);
    assert_eq!(
        STRUCTURAL_SOURCE_STATE_POLICY,
        StructuralSourceStatePolicy::ResetAtReplacementBoundary
    );
    children
        .plan
        .render(
            128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("replacement boundary");
    assert!(
        pcm.iter().all(|sample| *sample == 0.0),
        "new provider follows the frozen structural source-state policy"
    );
    assert_eq!(children.plan.owner.active_epoch().0, 1);
    assert_eq!(children.session.providers.epoch, 0);
    children
        .session
        .synchronize_plan_epochs()
        .expect("control promotion and retirement");
    assert_eq!(children.session.providers.epoch, 1);
    assert!(children.session.pending_providers.is_empty());

    let second_len = children
        .session
        .command(&second_request, 4_096)
        .expect("source-changing replacement after reclaim");
    assert!(second_len > 0);
    assert_eq!(
        children.session.controller.session().revision(),
        SessionRevision(44)
    );
    assert_eq!(
        source_region_end(&children.session.providers.sources),
        48_000
    );
    assert_eq!(children.session.pending_providers[0].epoch, 2);
    assert_eq!(
        source_region_end(&children.session.pending_providers[0].sources),
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
        .synchronize_plan_epochs()
        .expect("second provider promotion and retirement");
    assert_eq!(children.session.providers.epoch, 2);
    assert_eq!(source_region_end(&children.session.providers.sources), 512);
    assert!(children.session.pending_providers.is_empty());
    assert!(children.session.retired_providers.is_empty());
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
        children.session.controller.session().revision(),
        SessionRevision(44)
    );
    assert!(children.session.pending_providers.is_empty());
}

#[test]
fn all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes() {
    const RESPONSES: [&str; 8] = [
        "4d49534f43544c000100010030000200090000005800000001000000000000002a00000000000000060000000000000001000d010400000001000000000000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000010000000000000006000101010000000100000000000000",
        "4d49534f43544c000100010030000200080000003000000002000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000",
        "4d49534f43544c000100010030000200060000004000000003000000000000002a00000000000000040000000000000001000201020000000100000000000000020004010800000001000000000000000300040108000000010000000000000004000401080000000200000000000000",
        "4d49534f43544c000100010030000200030000001000000004000000000000002b00000000000000010000000000000001000301040000000100000000000000",
        "4d49534f43544c000100010030000200090000005800000005000000000000002b00000000000000060000000000000001000d010400000001000000000000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
        "4d49534f43544c000100010030000200090000005800000006000000000000002b00000000000000060000000000000001000d010800000001000000020000000200030104000000010000000000000003000d0100000000040003010400000000000000000000000500080101000000000000000000000006000101010000000100000000000000",
        "4d49534f43544c00010001003000020001000000c801000007000000000000002b000000000000001b000000000000000100020102000000010000000000000002000201020000000000000000000000030002010200000001000000000000000400020102000000010000000000000005000401080000000010000000000000060003010400000000080000000000000700040108000000001000000000000008000101010000000400000000000000090002010200000000010000000000000a0004010800000001000000000000000b0004010800000000100000000000000c0004010800000001000000000000000d0004010800000001000000000000000e0004010800000002000000000000000f00040108000000010000000000000010000401080000001000000000000000110004010800000000200000000000001200040108000000001000000000000013000401080000008000000000000000140004010800000080000000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01180000000100020003000400050006000700080009000a000b000c001a000c010c000000018002801080208021803080000000001b00040108000000ff3f000000000000",
        "4d49534f43544c000100010030000200090000005800000008000000000000002b00000000000000060000000000000001000d01000000000200030104000000000000000000000003000d01040000000100000000000000040003010400000001000000000000000500080101000000000000000000000006000101010000000100000000000000",
    ];
    const EVENTS: [&str; 7] = [
        "4d49534f43544c000100010030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000010000000000000002000101010000000200000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000000200000000000000",
        "4d49534f43544c000100010030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000002000000000000000200040108000000040000000000000003000401080000002a0000000000000004000301040000000100000000000000",
        "4d49534f43544c000100010030000300028000006000000000000000000000002b000000000000000600000000000000010004010800000003000000000000000200040108000000030000000000000003000201020000000100000000000000040001010100000001000000000000000500040108000000020000000000000006000400080000000000000000000000",
        "4d49534f43544c000100010030000300308000006000000000000000000000002b00000000000000010000000000000001000b015800000004000000000000000100090114000000636170692e72656e6465722e616374697669747900000000020001010100000001000000000000000600040008000000800000000000000007000400080000000100000000000000",
        "4d49534f43544c000100010030000300208000004800000000000000000000002b00000000000000040000000000000001000401080000008000000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010001000000000000000000",
        "4d49534f43544c000100010030000300208000004800000000000000000000002b00000000000000040000000000000001000401080000008001000000000000020002010200000001000000000000000300020102000000100000000000000004000a011000000001000000010001000000000000000000",
        "4d49534f43544c000100010030000300218000004000000000000000000000002b0000000000000002000000000000000100040108000000000200000000000002000b012800000002000000000000000100030104000000010000000000000002000401080000000400000000000000",
    ];
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let eager_capacities = crate::ffi::test_retained_capacities(c_session);
    let revision = SessionRevision(42);
    let configuration = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let configure = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(revision),
        miso_engine_protocol::CommandPayload::TelemetryConfigure(&configuration),
    );
    let (c_result, c_response) = command_c(c_session, &configure);
    assert_eq!(c_result, crate::RESULT_OK);
    assert_eq!(c_response, pinned_hex(RESPONSES[0]));

    let transport = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(revision),
        miso_engine_protocol::CommandPayload::TransportSet(
            miso_engine_protocol::TransportSetRequest {
                state: miso_engine_protocol::TransportState::Playing,
                position: Some(miso_engine_protocol::SampleTime(0)),
            },
        ),
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

    let record = miso_engine_protocol::AutomationRecord {
        kind: miso_engine_protocol::AutomationKind::Point,
        handle: miso_engine_protocol::ParameterHandle(5),
        start: miso_engine_protocol::SampleTime(1),
        end: miso_engine_protocol::SampleTime(1),
        start_value: 120.0,
        end_value: 120.0,
    };
    let automation = command_bytes_at_revision(
        3,
        ExpectedRevision::Exact(revision),
        miso_engine_protocol::CommandPayload::AutomationEnqueue(
            miso_engine_protocol::AutomationEnqueue {
                records: core::slice::from_ref(&record),
            },
        ),
    );
    let (automation_result, automation_response) = command_c(c_session, &automation);
    assert_eq!(automation_result, crate::RESULT_OK);
    assert_eq!(automation_response, pinned_hex(RESPONSES[2]));
    let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("event-origin").expect("stable ID"),
    };
    let structural = command_bytes_at_revision(
        4,
        ExpectedRevision::Exact(revision),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
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

    let quiet_meter_configuration = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: vec![1],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let disable_diagnostics = command_bytes_at_revision(
        5,
        ExpectedRevision::Exact(SessionRevision(43)),
        miso_engine_protocol::CommandPayload::TelemetryConfigure(&quiet_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &disable_diagnostics),
        (crate::RESULT_OK, pinned_hex(RESPONSES[4]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 128, &output),
        crate::RESULT_OK
    );
    let expanded_meter_configuration = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: vec![1, 2],
        meter_period_blocks: 1,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let expand_meters = command_bytes_at_revision(
        6,
        ExpectedRevision::Exact(SessionRevision(43)),
        miso_engine_protocol::CommandPayload::TelemetryConfigure(&expanded_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &expand_meters),
        (crate::RESULT_OK, pinned_hex(RESPONSES[5]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 256, &output),
        crate::RESULT_OK
    );
    let collect_third_render =
        command_bytes(7, miso_engine_protocol::CommandPayload::CapabilitiesGet);
    assert_eq!(
        command_c(c_session, &collect_third_render),
        (crate::RESULT_OK, pinned_hex(RESPONSES[6]))
    );
    assert_eq!(
        crate::ffi::test_telemetry_counters(c_session),
        miso_engine_protocol::TelemetryCounters {
            telemetry_coalesced: 1,
            telemetry_dropped: 1,
        }
    );
    for event in &EVENTS[4..=5] {
        event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &pinned_hex(event));
    }

    let counter_configuration = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: vec![miso_engine_protocol::CounterId::ControlCommandBackpressure],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    let configure_counters = command_bytes_at_revision(
        8,
        ExpectedRevision::Exact(SessionRevision(43)),
        miso_engine_protocol::CommandPayload::TelemetryConfigure(&counter_configuration),
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
    let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("destroyed-plan").expect("stable ID"),
    };
    let request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
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
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterResourceProjection,
        AfterRuntimePrepare, BeforeProtocolCommit,
    };

    const PHASES: [TestStructuralFaultPhase; 6] = [
        AfterProtocolPrepare,
        AfterResourceProjection,
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
            let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
                session_id: miso_engine_session::StableId::parse("fault-matrix")
                    .expect("stable ID"),
            };
            let request = command_bytes_at_revision(
                1,
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::SessionTransactionApply(
                    core::slice::from_ref(&edit),
                ),
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

            let retry_edit = miso_engine_protocol::SessionEditV1::SetSessionId {
                session_id: miso_engine_session::StableId::parse("fault-matrix-retry")
                    .expect("stable ID"),
            };
            let retry = command_bytes_at_revision(
                2,
                ExpectedRevision::Exact(SessionRevision(43)),
                miso_engine_protocol::CommandPayload::SessionTransactionApply(
                    core::slice::from_ref(&retry_edit),
                ),
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
            let collect = command_bytes(3, miso_engine_protocol::CommandPayload::CapabilitiesGet);
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
                    &miso_engine_protocol::TypedCommandFrame {
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
                miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                | miso_engine_protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
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
            assert_eq!(
                summary.2,
                u64::from(request_id == 12),
                "pinned active provider epoch"
            );
            assert_eq!(
                summary.3,
                usize::from(request_id >= 11),
                "pinned pending provider count"
            );
            header.message_id
        }};
    }

    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::CapabilitiesGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::CapabilitiesGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::SessionSnapshotGet(
                miso_engine_protocol::SessionSnapshotRequest {
                    offset: 0,
                    maximum_bytes: 1,
                },
            ),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::SessionSnapshotGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::ParameterMetadataGet(
                miso_engine_protocol::ParameterMetadataRequest {
                    after_handle: 0,
                    limit: 1,
                },
            ),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::ParameterMetadataGet
    );
    let state = miso_engine_protocol::ParameterStateRequest { handles: vec![1] };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::ParameterStateGet(&state),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::ParameterStateGet
    );
    let automation = [miso_engine_protocol::AutomationRecord {
        kind: miso_engine_protocol::AutomationKind::Point,
        handle: miso_engine_protocol::ParameterHandle(5),
        start: miso_engine_protocol::SampleTime(1),
        end: miso_engine_protocol::SampleTime(1),
        start_value: 120.0,
        end_value: 120.0,
    }];
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::AutomationEnqueue(
                miso_engine_protocol::AutomationEnqueue {
                    records: &automation,
                },
            ),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::AutomationEnqueue
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::TransportGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::TransportGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::TransportSet(
                miso_engine_protocol::TransportSetRequest {
                    state: miso_engine_protocol::TransportState::Playing,
                    position: Some(miso_engine_protocol::SampleTime(0)),
                },
            ),
            StatusCode::Ok,
            42,
            &[
                miso_engine_protocol::MessageId::TransportState,
                miso_engine_protocol::MessageId::AutomationCanceled,
            ]
        ),
        miso_engine_protocol::MessageId::TransportSet
    );
    let telemetry = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::TelemetryConfigure(&telemetry),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::TelemetryConfigure
    );
    let counters = miso_engine_protocol::CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::CountersGet(&counters),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::CountersGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            miso_engine_protocol::CommandPayload::DiagnosticsGet(
                miso_engine_protocol::DiagnosticsRequest {
                    after_sequence: 0,
                    limit: 1,
                    minimum_severity: miso_engine_protocol::DiagnosticSeverity::Info,
                },
            ),
            StatusCode::Ok,
            42,
            &[]
        ),
        miso_engine_protocol::MessageId::DiagnosticsGet
    );
    let structural = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("all-command-families")
            .expect("stable ID"),
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                &structural
            ),),
            StatusCode::Ok,
            43,
            &[miso_engine_protocol::MessageId::SessionCommitted]
        ),
        miso_engine_protocol::MessageId::SessionTransactionApply
    );
    assert_eq!(request_id, 11);
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 11, 0, 1),
        "pinned revision/replay/provider/pending vector"
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
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(43)),
            miso_engine_protocol::CommandPayload::NudgeEffectParam(
                miso_engine_protocol::NudgeEffectParam {
                    parameter_handle: 5,
                    size: miso_engine_protocol::NudgeSize::Xs,
                    count: 1,
                },
            ),
            StatusCode::Ok,
            44,
            &[miso_engine_protocol::MessageId::SessionCommitted]
        ),
        miso_engine_protocol::MessageId::NudgeEffectParam
    );
    assert_eq!(request_id, 12);
    let state_request = miso_engine_protocol::ParameterStateRequest { handles: vec![5] };
    let state_command = command_bytes_at_revision(
        13,
        ExpectedRevision::Any,
        miso_engine_protocol::CommandPayload::ParameterStateGet(&state_request),
    );
    let (state_result, state_response) = command_c(c_session, &state_command);
    assert_eq!(state_result, crate::RESULT_OK);
    let mut state_fields = [0_u16; 64];
    let state_page = match codec
        .decode_typed_response(&state_response, &mut DecodeScratch::new(&mut state_fields))
        .expect("nudge state response")
    {
        miso_engine_protocol::DecodedTypedResponseFrame::Success {
            payload: miso_engine_protocol::DecodedSuccessResponsePayload::ParameterState(page),
            ..
        } => page,
        _ => panic!("typed parameter state success"),
    };
    let registry =
        miso_engine_effect_compiler::launch_native_effect_registry_v1().expect("native registry");
    let parameter = &registry
        .get_ascii("miso.parametric-eq")
        .expect("parametric EQ")
        .descriptor()
        .parameters[2];
    let expected = miso_engine_effect_contract::resolve_parameter_nudge_value_v1(
        parameter,
        120.0,
        miso_engine_protocol::NudgeSize::Xs,
        1,
    )
    .expect("frequency nudge");
    assert_eq!(state_page.records[0].value.to_bits(), expected.to_bits());
    assert_eq!(
        crate::ffi::test_retained_capacities(c_session),
        eager_capacities
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact() {
    const REUSE: &str = "4d49534f43544c000100010030000200070009004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const EXPIRED: &str = "4d49534f43544c00010001003000020001000a004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const STALE: &str = "4d49534f43544c000100010030000200030007004800000013000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const TRANSPORT_20: &str = "4d49534f43544c000100010030000200080000003000000014000000000000002a000000000000000300000000000000010001010100000001000000000000000200040108000000000000000000000003000401080000000000000000000000";
    const TRANSPORT_21: &str = "4d49534f43544c000100010030000200080000003000000015000000000000002a000000000000000300000000000000010001010100000002000000000000000200040108000000000000000000000003000401080000000000000000000000";
    const EVENT_FULL: &str = "4d49534f43544c00010001003000020003000b00b000000016000000000000002a00000000000000030000000000000001000b01380000000200000000000000010009011500000070726f746f636f6c2e6261636b7072657373757265000000020001010100000003000000000000000200030104000000000000000000000003000b005800000005000000000000000100010101000000040000000000000002000401080000000200000000000000030004010800000002000000000000000400020102000000010000000000000005000400080000000400000000000000";
    const TRANSPORT_EVENT_20: &str = "4d49534f43544c000100010030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000010000000000000002000101010000000100000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000001400000000000000";
    const TRANSPORT_EVENT_21: &str = "4d49534f43544c000100010030000300108000005000000000000000000000002a0000000000000005000000000000000100040108000000020000000000000002000101010000000200000000000000030004010800000000000000000000000400040108000000000000000000000005000400080000001500000000000000";
    const STRUCTURAL_23: &str = "4d49534f43544c000100010030000200030000001000000017000000000000002b00000000000000010000000000000001000301040000000100000000000000";
    const COMMIT_EVENT: &str = "4d49534f43544c000100010030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000003000000000000000200040108000000170000000000000003000401080000002a0000000000000004000301040000000100000000000000";
    const RETRY_24: &str = "4d49534f43544c000100010030000200030000001000000018000000000000002c00000000000000010000000000000001000301040000000100000000000000";
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
                miso_engine_protocol::DecodedTypedResponseFrame::Success { header, .. }
                | miso_engine_protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
            };
            assert_eq!(header.status, $status);
            bytes
        }};
    }

    let first = command_bytes(1, miso_engine_protocol::CommandPayload::CapabilitiesGet);
    let first_bytes = dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1));
    assert_eq!(
        dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1)),
        first_bytes
    );
    let conflict = command_bytes(1, miso_engine_protocol::CommandPayload::TransportGet);
    dispatch!(conflict, StatusCode::RequestIdReuse, pinned_hex(REUSE));
    for request_id in 2..=18 {
        dispatch!(
            command_bytes(
                request_id,
                miso_engine_protocol::CommandPayload::CapabilitiesGet
            ),
            StatusCode::Ok,
            capabilities_response(request_id)
        );
    }
    dispatch!(first, StatusCode::ReplayExpired, pinned_hex(EXPIRED));

    let edit = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("pressure-one").expect("stable ID"),
    };
    let stale = command_bytes_at_revision(
        19,
        ExpectedRevision::Exact(SessionRevision(41)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(stale, StatusCode::RevisionConflict, pinned_hex(STALE));

    for (request_id, state, expected) in [
        (
            20,
            miso_engine_protocol::TransportState::Stopped,
            TRANSPORT_20,
        ),
        (
            21,
            miso_engine_protocol::TransportState::Playing,
            TRANSPORT_21,
        ),
    ] {
        dispatch!(
            command_bytes_at_revision(
                request_id,
                ExpectedRevision::Exact(SessionRevision(42)),
                miso_engine_protocol::CommandPayload::TransportSet(
                    miso_engine_protocol::TransportSetRequest {
                        state,
                        position: Some(miso_engine_protocol::SampleTime(0)),
                    },
                ),
            ),
            StatusCode::Ok,
            pinned_hex(expected)
        );
    }
    let event_full = command_bytes_at_revision(
        22,
        ExpectedRevision::Exact(SessionRevision(42)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
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
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(first_structural, StatusCode::Ok, pinned_hex(STRUCTURAL_23));
    let commit_event = event_c(c_session, crate::EVENT_LANE_RELIABLE);
    assert_eq!(commit_event.0, crate::RESULT_OK);
    assert_eq!(commit_event.1, pinned_hex(COMMIT_EVENT));

    let second_edit = miso_engine_protocol::SessionEditV1::SetSessionId {
        session_id: miso_engine_session::StableId::parse("pressure-two").expect("stable ID"),
    };
    let publication_full = command_bytes_at_revision(
        24,
        ExpectedRevision::Exact(SessionRevision(43)),
        miso_engine_protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
            &second_edit,
        )),
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

#[test]
fn direct_and_c_render_match_one_and_ten_tracks_across_launch_rates() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        render_parity_shape(1, sample_rate_hz);
        render_parity_shape(10, sample_rate_hz);
    }
}

#[test]
fn barrier_schedule_separates_one_source_producer_from_exclusive_render() {
    let mut model =
        parse_session_toml(&generated_parity_session(1, 48_000)).expect("concurrency session");
    model.sources[0].mapping.region.length_samples = 1_024;
    let session = miso_engine_session::canonical_session_toml(&model).expect("canonical");
    let children = compile_children(&session, limits()).expect("concurrent children");
    let session = Box::into_raw(Box::new(crate::Session::new(
        children.session,
        children.session_error,
    ))) as usize;
    let plan = Box::into_raw(Box::new(crate::Plan::new(children.plan))) as usize;
    let submitted = std::sync::Arc::new(std::sync::Barrier::new(2));
    let consumed = std::sync::Arc::new(std::sync::Barrier::new(2));
    std::thread::scope(|scope| {
        let producer_submitted = submitted.clone();
        let producer_consumed = consumed.clone();
        scope.spawn(move || {
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
                producer_submitted.wait();
                producer_consumed.wait();
            }
        });
        let render_submitted = submitted.clone();
        let render_consumed = consumed.clone();
        scope.spawn(move || {
            let plan = plan as *mut crate::Plan;
            let mut observed_signal = false;
            for block in 0..6_u64 {
                render_submitted.wait();
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
                render_consumed.wait();
            }
            assert!(observed_signal);
        });
    });
    crate::ffi::test_session_destroy(session as *mut crate::Session);
    crate::ffi::test_plan_destroy(plan as *mut crate::Plan);
}
