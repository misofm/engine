//! #1578: a strip `RESPONSE_GET` through the exported C ABI answers a tag, and the strip's curve
//! arrives as one `RESPONSE_READY` event through `miso_engine_v1_dequeue_event`.
//!
//! The session is the nine-track EQ fixture cut to two tracks at 48 kHz (1 MiB frame limit):
//! `eq0` has asymmetric input filters, the EQ console slot `eq`, an EQ insert and a compressor
//! insert (no linear response); `eq1` is the fixture's plain strip.

#![allow(unsafe_code)]

use core::ptr;

use capi::*;
use host_core::{
    HostPrepareCaps, HostShapePolicy, ResponseChannels, ResponsePreviewGrid,
    ResponseSnapshotCollector, TrackResponse, compile_host_session, compose_track_response,
    prepare_host_runtime,
};
use protocol::{
    BackpressureQueueKind, CommandPayload, DecodeScratch, DecodedEventPayload,
    DecodedSuccessResponsePayload, DecodedTypedResponseFrame, ExpectedRevision, ParameterChannel,
    ProtocolCodec, RequestId, ResponseGetRequest, ResponseGrid, ResponseMemberKind,
    ResponseMemberSection, ResponseReady, ResponseReadyOutcome, ResponseResult, ResponseTarget,
    SessionEdit, SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{Effect, EffectIdentity, EffectQuality, LinkMode, SidechainDeclaration, StableId};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const FRAME_BYTES: u64 = 1 << 20;
const QUANTUM: usize = 128;
const POINTS: u32 = 64;
/// The control plane's reliable event slots: one drain returns at most these frames and two more
/// (one held behind the queue, one pending), so a defect that delivers without end fails instead of
/// growing.
const RELIABLE_EVENT_SLOTS: usize = 2;
/// More blocks than any adoption of these edits takes.
const BLOCKS: usize = 256;

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

fn effect(name: &str, effect_id: &str, params: Vec<session::EffectParam>) -> Effect {
    Effect {
        id: sid(name),
        identity: EffectIdentity::Native {
            effect_id: sid(effect_id),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        params,
        sidechain: SidechainDeclaration::None,
    }
}

/// The two-track session and its revision.
fn document() -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.tracks.truncate(2);
    let kept: Vec<_> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| {
        matches!(&route.source, session::RouteSource::Track { track_id, .. } if kept.contains(track_id))
    });
    let mut eq_params = model.tracks[0].console[0].params.clone();
    // Band 2 on the left only, so the insert's two enabled masks differ.
    for (channel, value) in [
        (session::ParameterChannel::Left, 1.0),
        (session::ParameterChannel::Right, 0.0),
    ] {
        eq_params.push(session::EffectParam {
            parameter_id: 17,
            channel,
            unit: session::ParameterUnit::Linear,
            value,
        });
    }
    let track = &mut model.tracks[0];
    track.builtins.left.hpf_hz = 80.0;
    track.builtins.left.lpf_hz = 12_000.0;
    track.builtins.right.hpf_hz = 160.0;
    track.builtins.right.lpf_hz = 6_000.0;
    track.inserts.effects = vec![
        effect("eq-insert", "miso.parametric-eq", eq_params),
        effect("comp", "miso.compressor", Vec::new()),
    ];
    (
        session::canonical_session_json(&model).expect("canonical"),
        model.revision,
    )
}

fn limits(frame_bytes: u64) -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
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
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 30,
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: frame_bytes,
        maximum_replay_bytes: frame_bytes.max(8_192) * 16,
        maximum_replay_entries: 64,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// What one submitted command answered.
#[derive(Debug, PartialEq)]
enum Reply {
    Tag(u64),
    Refused {
        status: StatusCode,
        code: String,
        backpressure: Option<protocol::Backpressure>,
    },
    Other,
}

/// One compiled session, its plan, and the host's clock.
struct Host {
    session: *mut Session,
    plan: *mut Plan,
    next_request: u64,
    revision: u64,
    at: u64,
}

impl Host {
    fn new(document: &str, revision: u64) -> Self {
        Self::with_frame_bytes(document, revision, FRAME_BYTES)
    }

    fn with_frame_bytes(document: &str, revision: u64, frame_bytes: u64) -> Self {
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        let compile_limits = limits(frame_bytes);
        // SAFETY: Every descriptor and output location remains live for the complete call.
        unsafe {
            assert_eq!(
                miso_engine_v1_engine_create(&config, &mut engine),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &compile_limits,
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                RESULT_OK,
                "{}",
                String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize])
            );
            miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            next_request: 1,
            revision,
            at: 0,
        }
    }

    fn frame(&mut self, expected: ExpectedRevision, payload: CommandPayload<'_>) -> Vec<u8> {
        let request_id = RequestId::new(self.next_request).expect("request ID");
        self.next_request += 1;
        let mut bytes = vec![0_u8; 1 << 16];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id,
                    expected_revision: expected,
                    payload,
                },
                &mut bytes,
            )
            .expect("command frame");
        bytes.truncate(length);
        bytes
    }

    fn submit(&self, request: &[u8]) -> Vec<u8> {
        let mut response = vec![0_u8; FRAME_BYTES as usize];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; both buffers are complete.
        let code = unsafe {
            miso_engine_v1_submit_command(
                self.session,
                request.as_ptr(),
                request.len() as u64,
                &mut output,
            )
        };
        assert_eq!(code, RESULT_OK);
        response.truncate(output.required_bytes as usize);
        response
    }

    /// A strip `RESPONSE_GET` of `strip`: its frame and its reply.
    fn ask(&mut self, strip: &str) -> (Vec<u8>, Reply) {
        let request = strip_request(strip);
        let bytes = self.frame(ExpectedRevision::Any, CommandPayload::ResponseGet(&request));
        let reply = decode(&self.submit(&bytes));
        (bytes, reply)
    }

    /// Commit `edits` as one transaction; `false` when it is refused.
    fn transact(&mut self, edits: &[SessionEdit]) -> bool {
        let bytes = self.frame(
            ExpectedRevision::Exact(SessionRevision(self.revision)),
            CommandPayload::SessionTransactionApply(edits),
        );
        let response = self.submit(&bytes);
        let mut fields = [0_u16; 1024];
        match ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("a transaction response")
        {
            DecodedTypedResponseFrame::Success { .. } => {
                self.revision += 1;
                true
            }
            DecodedTypedResponseFrame::NonOk { header, .. } => {
                assert_eq!(header.status, StatusCode::Backpressure, "only backpressure");
                false
            }
        }
    }

    fn render(&mut self) {
        let mut pcm = [0.0_f32; QUANTUM * 2];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM as u32,
            plane_stride_samples: QUANTUM as u32,
            reserved: [0; 2],
        };
        // SAFETY: `plan` is live and `output` describes `pcm`.
        let code = unsafe { miso_engine_v1_render_f32_planar(self.plan, self.at, &output) };
        assert_eq!(code, RESULT_OK);
        self.at += QUANTUM as u64;
        // SAFETY: `session` is live and used by this thread alone.
        assert_eq!(unsafe { miso_engine_v1_service(self.session) }, RESULT_OK);
    }

    /// Drain the reliable lane: the `RESPONSE_READY` events it held, in order, and the number of
    /// other events. Panics past `RELIABLE_EVENT_SLOTS + 2` frames.
    fn drain(&mut self) -> (Vec<ResponseReady>, usize) {
        let mut storage = vec![0_u8; FRAME_BYTES as usize];
        let mut ready = Vec::new();
        let mut others = 0;
        for _ in 0..=RELIABLE_EVENT_SLOTS + 2 {
            let mut event = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: storage.as_mut_ptr(),
                capacity_bytes: storage.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: `session` is live and used by this thread alone.
            let code = unsafe {
                miso_engine_v1_dequeue_event(self.session, EVENT_LANE_RELIABLE, &mut event)
            };
            assert_eq!(code, RESULT_OK);
            if event.required_bytes == 0 {
                return (ready, others);
            }
            let mut fields = [0_u16; 1024];
            let decoded = ProtocolCodec::default()
                .decode_typed_event(
                    &storage[..event.required_bytes as usize],
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("a reliable event");
            match decoded.payload {
                DecodedEventPayload::ResponseReady(value) => ready.push(value),
                _ => others += 1,
            }
        }
        panic!(
            "more than {} reliable frames in one drain",
            RELIABLE_EVENT_SLOTS + 2
        );
    }

    /// Render and service up to `blocks` blocks, draining after each; every `RESPONSE_READY`.
    fn run(&mut self, blocks: usize) -> Vec<ResponseReady> {
        let mut ready = Vec::new();
        for _ in 0..blocks {
            self.render();
            ready.extend(self.drain().0);
        }
        ready
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        // SAFETY: These are the exact live handles `miso_engine_v1_compile_session` returned,
        // destroyed once.
        unsafe {
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

fn strip_request(strip: &str) -> ResponseGetRequest {
    ResponseGetRequest {
        target: ResponseTarget::Strip(strip.to_owned()),
        grid: ResponseGrid::Logarithmic,
        points: POINTS,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: ParameterChannel::Both,
        sections: false,
        overrides: Vec::new(),
        configuration_id: None,
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    }
}

fn decode(response: &[u8]) -> Reply {
    let mut fields = [0_u16; 1024];
    match ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a typed response")
    {
        DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ResponseResult(ResponseResult::Tag(tag)),
            ..
        } => Reply::Tag(tag),
        DecodedTypedResponseFrame::NonOk { header, payload } => Reply::Refused {
            status: header.status,
            code: payload.diagnostics[0].code.clone(),
            backpressure: payload.backpressure,
        },
        DecodedTypedResponseFrame::Success { .. } => Reply::Other,
    }
}

/// The core's own composition of `strip` from a host prepared from `document`.
fn composed(document: &str, strip: &str) -> TrackResponse {
    let caps = HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 16,
        maximum_submixes: 16,
        maximum_vcas: 16,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 16,
        maximum_routes: 16,
        maximum_effects: 16,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    };
    let compiled = compile_host_session(document, &caps).expect("compile");
    let mut host = prepare_host_runtime(&compiled, &caps).expect("prepare");
    let mut collector = ResponseSnapshotCollector::new(
        48_000,
        8,
        effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS,
        127,
    );
    let capture = host
        .copy_response_snapshot(strip, &mut collector)
        .expect("a capture");
    compose_track_response(
        strip,
        &collector.finish(capture),
        ResponsePreviewGrid::Logarithmic {
            points: POINTS as usize,
            minimum_hz: 20.0,
            maximum_hz: 20_000.0,
        },
        ResponseChannels::Both,
    )
    .expect("a composition")
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

fn add_insert(track: &str, name: &str, position: u32) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: sid(track),
        rack_name: session::RackName::Inserts,
        final_position: position,
        effect: effect(name, "miso.compressor", Vec::new()),
    }
}

/// Gates 1 and 3, with the acked-batch lens across a rebuild and a supersession: a strip request
/// answers tag 1 and no event before a render; after one render and `service`, exactly one
/// `RESPONSE_READY` with tag 1, outcome `OK`, four members in signal order (builtins, the console
/// slot, the EQ insert, the compressor with kind 3) and curves bit-equal to the core's composition
/// of a host prepared from the same document. Across a rebuild that adds an insert (a duck-swap,
/// #1324), a request armed before its commit gets one event from the running plan, and one armed
/// after it gets one event from the adopted plan, with the insert. A request on `eq1` armed on a
/// pending rebuild that
/// a removal of `eq1` then supersedes gets one event, `NOT_FOUND` `response.strip.removed`. No tag
/// gets a second event.
///
/// Test value: red if the C ABI's curve or owner list differs from the core's composition (the
/// event's mapping of D5 included), if the event arrives before the plan has rendered, if a request
/// whose strip a superseding edit removed is never answered, if a rebuild leaves a tag without its
/// event or with two, or if a capture is taken from another plan than the request's newest.
#[test]
fn end_to_end_then_removed_strip() {
    let (document, revision) = document();
    let mut host = Host::new(&document, revision);
    assert_eq!(host.ask("eq0").1, Reply::Tag(1));
    assert_eq!(host.drain().0, Vec::new(), "no event before a render");

    let ready = host.run(1);
    assert_eq!(ready.len(), 1, "one event after one render and service");
    let event = &ready[0];
    assert_eq!((event.request_tag, &*event.strip_id), (1, "eq0"));
    let ResponseReadyOutcome::Ready(capture) = &event.outcome else {
        panic!("a capture: {:?}", event.outcome);
    };
    let expected = composed(&document, "eq0");
    assert_eq!(
        (capture.sample_rate_hz, capture.floor_db.to_bits()),
        (expected.sample_rate_hz, expected.floor_db.to_bits())
    );
    assert_eq!(
        bits(&capture.frequencies_hz),
        bits(&expected.frequencies_hz)
    );
    assert_eq!(
        capture.left_db.as_deref().map(bits),
        expected.left_db.as_deref().map(bits)
    );
    assert_eq!(
        capture.right_db.as_deref().map(bits),
        expected.right_db.as_deref().map(bits)
    );
    assert_ne!(
        capture.left_db, capture.right_db,
        "the asymmetric input filters reach both sides"
    );
    assert_eq!(capture.excluded_count, expected.excluded_count);
    let sections: Vec<_> = capture
        .members
        .iter()
        .map(|member| (member.section.clone(), member.kind))
        .collect();
    assert_eq!(
        sections,
        vec![
            (
                ResponseMemberSection::Builtins,
                ResponseMemberKind::InputFilters
            ),
            (
                ResponseMemberSection::Console("eq".to_owned()),
                ResponseMemberKind::ParametricEq
            ),
            (
                ResponseMemberSection::Inserts("eq-insert".to_owned()),
                ResponseMemberKind::ParametricEq
            ),
            (
                ResponseMemberSection::Inserts("comp".to_owned()),
                ResponseMemberKind::Unavailable
            ),
        ]
    );
    assert_ne!(
        capture.members[2].enabled_left, capture.members[2].enabled_right,
        "the insert's masks differ by side"
    );
    for (member, core) in capture.members.iter().zip(&expected.members) {
        assert_eq!(
            (
                &*member.effect,
                member.bypassed,
                member.enabled_left,
                member.enabled_right,
                member.section_count
            ),
            (
                &*core.native_id,
                core.bypassed,
                core.enabled_left,
                core.enabled_right,
                core.section_count
            )
        );
    }
    assert_eq!(host.run(4), Vec::new(), "one event per tag");

    // Across a rebuild that adds an insert. A rebuild that restarts the strip duck-swaps
    // (#1324): render adopts its plan at the end of the duck, not at the next block. Armed before
    // the commit, the request's newest plan is the running one, which renders through the duck:
    // answered once, from it, without the insert.
    let has = |capture: &protocol::TrackResponseCapture, name: &str| {
        capture
            .members
            .iter()
            .any(|member| member.section == ResponseMemberSection::Inserts(name.into()))
    };
    assert_eq!(host.ask("eq0").1, Reply::Tag(2));
    assert!(host.transact(&[add_insert("eq0", "comp-late", 2)]));
    let ready = host.run(BLOCKS);
    assert_eq!(ready.len(), 1, "one event across the rebuild");
    let ResponseReadyOutcome::Ready(capture) = &ready[0].outcome else {
        panic!("a capture: {:?}", ready[0].outcome);
    };
    assert_eq!(ready[0].request_tag, 2);
    assert!(
        !has(capture, "comp-late"),
        "armed before the commit: the running plan's"
    );
    // Armed after the commit of a rebuild, on its pending plan: answered once, only from the
    // adopted plan, with the insert.
    assert!(host.transact(&[add_insert("eq0", "comp-later", 3)]));
    assert_eq!(host.ask("eq0").1, Reply::Tag(3));
    let ready = host.run(BLOCKS);
    assert_eq!(ready.len(), 1, "one event across the adoption");
    let ResponseReadyOutcome::Ready(capture) = &ready[0].outcome else {
        panic!("a capture: {:?}", ready[0].outcome);
    };
    assert_eq!(ready[0].request_tag, 3);
    assert!(
        has(capture, "comp-later"),
        "armed on the pending plan: the adopted plan's"
    );

    // Armed on a pending rebuild that a removal of the strip supersedes.
    assert!(host.transact(&[add_insert("eq1", "comp-1", 0)]));
    assert_eq!(host.ask("eq1").1, Reply::Tag(4));
    assert!(host.transact(&[
        SessionEdit::RemoveRoute {
            route_id: sid("eq1-main"),
        },
        SessionEdit::RemoveTrack {
            track_id: sid("eq1"),
        },
    ]));
    let ready = host.run(BLOCKS);
    assert_eq!(ready.len(), 1, "one event for the removed strip");
    assert_eq!(
        (ready[0].request_tag, &*ready[0].strip_id, &ready[0].outcome),
        (
            4,
            "eq1",
            &ResponseReadyOutcome::Refused {
                status: StatusCode::NotFound,
                diagnostic: "response.strip.removed".to_owned(),
            }
        )
    );
}

/// Gates 2 and 5: while a capture is in flight a second strip request is `BACKPRESSURE`
/// `response.capture.pending` with the capture-slot record; with the reliable event queue full of
/// session events the host has not dequeued, the completed capture is held across renders and
/// services, and once the host drains the queue exactly one `RESPONSE_READY` arrives. The same
/// request ID and bytes then replay the same tag and produce no second event, and the next request
/// is tag 2 (the replay armed nothing; the delivered capture was released). The session is
/// destroyed with a capture in flight.
///
/// Test value: red if an acked request can lose its event (a full queue drops it), two captures
/// run at once, a replay arms a second capture, or a delivered capture is never released.
#[test]
fn one_in_flight_none_lost_and_replay() {
    let (document, revision) = document();
    let mut host = Host::new(&document, revision);
    let (first, reply) = host.ask("eq0");
    assert_eq!(reply, Reply::Tag(1));
    let refused = Reply::Refused {
        status: StatusCode::Backpressure,
        code: "response.capture.pending".to_owned(),
        backpressure: Some(protocol::Backpressure {
            queue_kind: BackpressureQueueKind::ResponseCapture,
            capacity: 1,
            occupancy: 1,
            requested_items: 1,
            generation: None,
            retry_boundary: None,
            requested_bytes: None,
            available_bytes: None,
        }),
    };
    assert_eq!(host.ask("eq1").1, refused, "one capture in flight");

    // Fill the reliable event queue with commits the host does not dequeue.
    let mut commits = 0;
    while host.transact(&[SessionEdit::SetSessionId {
        session_id: sid(&format!("full-{commits}")),
    }]) {
        commits += 1;
        assert!(commits < 64, "the reliable event queue fills");
    }
    assert!(commits > 0, "the queue took at least one commit");
    for _ in 0..4 {
        host.render();
    }
    assert_eq!(
        host.ask("eq1").1,
        refused,
        "the held result is still in flight"
    );
    let (ready, others) = host.drain();
    assert_eq!(others, commits, "every commit's event, none lost");
    assert_eq!(
        ready.len(),
        1,
        "the held capture arrives once the queue drains"
    );
    assert_eq!(ready[0].request_tag, 1);
    assert!(matches!(ready[0].outcome, ResponseReadyOutcome::Ready(_)));

    // Gate 5: the same ID and bytes replay tag 1 and arm nothing.
    assert_eq!(decode(&host.submit(&first)), Reply::Tag(1));
    assert_eq!(host.run(4), Vec::new(), "a replay arms nothing");
    assert_eq!(host.ask("eq1").1, Reply::Tag(2), "released after delivery");
    // Destroyed with tag 2 in flight: nothing more is owed by a session that no longer exists.
}

/// The acked-batch lens on a rebuild that grows the strip: on a 4 KiB control frame limit a
/// request armed on a pending rebuild of `eq1` (so its event bound is checked at `eq0`'s four
/// owners) arms tag 1; two rebuilds then supersede that plan before it renders and add twenty
/// inserts to `eq0`, so the capture, armed again from the plan render adopts, no longer fits a
/// frame. The tag still gets exactly one event, the refusal
/// `LIMIT_EXCEEDED` `response.reply_too_large`, and the next request arms again.
///
/// Test value: red if an event the endpoint cannot encode is held for ever (the tag never
/// answered and the capture never released) or dropped.
#[test]
fn a_capture_grown_past_the_frame_limit_is_answered_by_its_refusal() {
    let (document, revision) = document();
    let mut host = Host::with_frame_bytes(&document, revision, 4_096);
    assert!(host.transact(&[add_insert("eq1", "comp-1", 0)]));
    assert_eq!(host.ask("eq0").1, Reply::Tag(1));
    // Two transactions of ten inserts each (one frame of twenty would not fit 4 KiB).
    for batch in 0..2_u32 {
        let edits: Vec<_> = (0..10)
            .map(|index| {
                let at = 10 * batch + index;
                add_insert("eq0", &format!("grow-{at}"), 2 + at)
            })
            .collect();
        assert!(host.transact(&edits));
        // Read the commit's event (the queue holds two); nothing has rendered.
        assert_eq!(host.drain().0, Vec::new());
    }
    let ready = host.run(BLOCKS);
    assert_eq!(ready.len(), 1, "one event for the grown capture");
    assert_eq!(
        (ready[0].request_tag, &ready[0].outcome),
        (
            1,
            &ResponseReadyOutcome::Refused {
                status: StatusCode::LimitExceeded,
                diagnostic: "response.reply_too_large".to_owned(),
            }
        )
    );
    assert_eq!(host.ask("eq1").1, Reply::Tag(2), "released after delivery");
}

/// The fixture the SDK's track-response codec is held to (#1579a gate 1).
const SDK_FIXTURE: &str = "../../sdk/test/fixtures/response-ready.json";

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut text, byte| {
        write!(text, "{byte:02x}").expect("write to a string");
        text
    })
}

/// A JSON string literal of `text`.
fn json_string(text: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            character if u32::from(character) < 0x20 => {
                write!(out, "\\u{:04x}", u32::from(character)).expect("write to a string");
            }
            character => out.push(character),
        }
    }
    out.push('"');
    out
}

/// Each value's bit pattern, as eight hex digits.
fn bits_json(values: Option<&[f32]>) -> String {
    values.map_or_else(
        || "null".to_owned(),
        |values| {
            let words: Vec<_> = values
                .iter()
                .map(|value| format!("\"{:08x}\"", value.to_bits()))
                .collect();
            format!("[{}]", words.join(", "))
        },
    )
}

/// The checked-in SDK fixture: the document, the strip request the C ABI sent (its frame), and
/// the one `RESPONSE_READY` frame it answered after one render, with the values the protocol
/// codec decodes from it.
fn sdk_fixture() -> String {
    let (document, revision) = document();
    let mut host = Host::new(&document, revision);
    let (request, reply) = host.ask("eq0");
    assert_eq!(reply, Reply::Tag(1));
    host.render();
    let mut storage = vec![0_u8; FRAME_BYTES as usize];
    let mut event = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: `session` is live and used by this thread alone.
    let code =
        unsafe { miso_engine_v1_dequeue_event(host.session, EVENT_LANE_RELIABLE, &mut event) };
    assert_eq!(code, RESULT_OK);
    let frame = storage[..event.required_bytes as usize].to_vec();
    let mut fields = [0_u16; 1024];
    let decoded = ProtocolCodec::default()
        .decode_typed_event(&frame, &mut DecodeScratch::new(&mut fields))
        .expect("a reliable event");
    let DecodedEventPayload::ResponseReady(ready) = decoded.payload else {
        panic!("the first reliable event is the RESPONSE_READY");
    };
    let ResponseReadyOutcome::Ready(capture) = &ready.outcome else {
        panic!("a capture: {:?}", ready.outcome);
    };
    let members: Vec<_> = capture
        .members
        .iter()
        .map(|member| {
            let (section, slot) = match &member.section {
                ResponseMemberSection::Builtins => (4, "null".to_owned()),
                ResponseMemberSection::Console(id) => (5, json_string(id)),
                ResponseMemberSection::Inserts(id) => (2, json_string(id)),
            };
            format!(
                "      {{ \"section\": {section}, \"slotId\": {slot}, \"effect\": {}, \"kind\": {}, \"bypassed\": {}, \"enabledLeft\": {}, \"enabledRight\": {}, \"sectionCount\": {} }}",
                json_string(&member.effect),
                member.kind.code(),
                member.bypassed,
                member.enabled_left,
                member.enabled_right,
                member.section_count,
            )
        })
        .collect();
    format!(
        "{{\n  \"schema\": \"miso.capi.response-ready\",\n  \"document\": {},\n  \"request\": {{ \"requestId\": \"1\", \"trackId\": \"eq0\", \"grid\": {{ \"kind\": \"logarithmic\", \"points\": {POINTS}, \"minimumHz\": 20, \"maximumHz\": 20000 }}, \"channels\": \"both\" }},\n  \"requestHex\": \"{}\",\n  \"eventHex\": \"{}\",\n  \"expect\": {{\n    \"requestTag\": \"{}\",\n    \"stripId\": {},\n    \"capturedSample\": \"{}\",\n    \"sampleRateHz\": {},\n    \"floorDb\": \"{:08x}\",\n    \"excludedCount\": {},\n    \"frequenciesHz\": {},\n    \"leftDb\": {},\n    \"rightDb\": {},\n    \"members\": [\n{}\n    ]\n  }}\n}}\n",
        json_string(&document),
        hex(&request),
        hex(&frame),
        ready.request_tag,
        json_string(&ready.strip_id),
        capture.captured_sample,
        capture.sample_rate_hz,
        capture.floor_db.to_bits(),
        capture.excluded_count,
        bits_json(Some(&capture.frequencies_hz)),
        bits_json(capture.left_db.as_deref()),
        bits_json(capture.right_db.as_deref()),
        members.join(",\n"),
    )
}

/// #1579a gate 1: the checked-in `sdk/test/fixtures/response-ready.json` is exactly what the C ABI
/// sends and answers for the fixture document. The SDK eval (`sdk/test/live-response-evals.mjs`)
/// encodes the same strip request and compares bytes, decodes the same event, and asks the headless
/// engine the same question of the same document.
///
/// Test value: red if the C ABI's strip request frame or its `RESPONSE_READY` changes a byte
/// without the fixture, and with it the SDK codec held to that file, following.
/// Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate change.
#[test]
fn the_sdk_fixture_is_what_the_c_abi_answers() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SDK_FIXTURE);
    let rendered = sdk_fixture();
    if std::env::var_os("MISO_ENGINE_BLESS_FIXTURES").is_some() {
        std::fs::write(&path, &rendered).expect("write the fixture");
        return;
    }
    let checked_in = std::fs::read_to_string(&path).expect("the checked-in fixture exists");
    assert!(
        checked_in == rendered,
        "{SDK_FIXTURE} differs from the C ABI's frames; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
    );
}
