//! #1654 gate 6 (T4): a send whose own rows have stored automation renders allocation-free
//! through the shared commit, across its hand edits.
//!
//! The script runs through control-plane's shared commit (`compile_children` and
//! `SessionState::command`, the code the browser Worker and the C ABI both run) and host-core's
//! render session (`PlanState::render`): track `eq0` sends `pre_fader` into bus `b` (no source
//! mute curve), and the send rides its `gain_db` (row 1, linear from 0 to -12 dB) and its
//! `matrix_lr` (row 4). A hand move of the automated gain (`SET_CONTROL` row 1, the hand offset,
//! `live`), a hand edit of the automated `matrix_lr` (automation wins: `model_only`), a live edit
//! of the un-automated `matrix_rr` (D6, `live`) and a hand send mute (`live`) commit in the
//! middle of the ride. Every render call after the warm-up allocates and frees nothing, measured
//! with the audited allocator's thread-scoped counters.

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, SourceSubmission,
    SuccessorBase, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ChannelMatrix,
    ControlAddress, Owner, ParameterChannel, ParameterUnit, Route, RouteDestination, RouteSource,
    Section, SendTap, StableId, Submix,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const SOURCE: &[u8] = b"fixture-source";

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

struct Preparer;

impl RuntimePreparer for Preparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
        let live = HostLiveControlRequest::default();
        match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }
    }
}

fn limits() -> ControlLimits {
    ControlLimits {
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_control_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 65_536,
        maximum_replay_bytes: 262_144,
        maximum_replay_entries: 16,
        maximum_submixes: 4,
        maximum_vcas: 0,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

fn matrix(ll: f32, rr: f32) -> ChannelMatrix {
    ChannelMatrix {
        ll,
        lr: 0.0,
        rl: 0.0,
        rr,
    }
}

/// The fixture's first track `eq0` alone, with no effect: `eq0-main` takes its left lane to the
/// output's left plane, `eq0-b` sends its right lane from `pre_fader` into bus `b`, and `b-main`
/// takes the bus to the output, so the output's right plane is the send. The send rides its
/// `gain_db` from 0 to -12 dB and its `matrix_lr` from 0 to 0.5 over 8,192 samples.
fn document() -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    model.automation.clear();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.truncate(1);
    let track = &mut model.tracks[0];
    track.console.clear();
    track.inserts.effects.clear();
    let route = |route: &str, source: RouteSource, destination, channel_matrix| Route {
        id: id(route),
        source,
        destination,
        channel_matrix,
        gain_db: 0.0,
        mute: false,
        follows_mute: false,
    };
    let output = || RouteDestination::OutputInput {
        output_id: id("main-out"),
    };
    model.routes = vec![
        route(
            "eq0-main",
            RouteSource::Track {
                track_id: id("eq0"),
                tap: SendTap::PostPan,
            },
            output(),
            matrix(1.0, 0.0),
        ),
        route(
            "eq0-b",
            RouteSource::Track {
                track_id: id("eq0"),
                tap: SendTap::PreFader,
            },
            RouteDestination::SubmixInput { submix_id: id("b") },
            matrix(0.0, 1.0),
        ),
        route(
            "b-main",
            RouteSource::Submix {
                submix_id: id("b"),
                tap: SendTap::PostPan,
            },
            output(),
            matrix(1.0, 1.0),
        ),
    ];
    model.submixes = vec![Submix::unity(id("b"), &model.console)];
    let ride = |name: &str, control: u32, to: f32, unit| Automation {
        id: id(name),
        target: ControlAddress {
            owner: Owner::Route(id("eq0-b")),
            section: Section::Own,
            control,
            lane: ParameterChannel::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: 8_192,
            start_value: 0.0,
            end_value: to,
            unit,
        }],
        offset: AutomationOffset::default(),
    };
    model.automation = vec![
        ride("send-gain", 1, -12.0, ParameterUnit::Db),
        ride("send-lr", 4, 0.5, ParameterUnit::Linear),
    ];
    (
        session::canonical_session_json(&model).expect("canonical"),
        model.revision,
    )
}

struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    revision: u64,
    request: u64,
    block: u64,
    /// The last block's samples, left plane then right.
    last: Vec<f32>,
}

impl Rig {
    fn new() -> Self {
        let (document, revision) = document();
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document, limits(), Box::new(Preparer))
                .expect("boot compile");
        Self {
            session,
            plan,
            revision,
            request: 1,
            block: 0,
            last: vec![0.0; QUANTUM * 2],
        }
    }

    /// Feed and render one block of a signed signal; the allocator events of the render call.
    fn block(&mut self) -> bench_support::alloc::Counters {
        let start = self.block * QUANTUM as u64;
        let plane: Vec<f32> = (0..QUANTUM as u64)
            .map(|frame| ((start + frame) % 97) as f32 / 97.0 - 0.495)
            .collect();
        self.session
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: start,
                    sample_rate_hz: 48_000,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        self.plan.render(start, output).expect("a rendered block");
        let used = bench_support::alloc::current_thread_delta_since(mark);
        self.last.copy_from_slice(&samples);
        self.block += 1;
        used
    }

    /// Commit route row `control` of the send at `value` as one transaction, which must answer
    /// `path`.
    fn commit(&mut self, control: u32, value: f32, path: EditPath) {
        let address = ControlAddress {
            owner: Owner::Route(id("eq0-b")),
            section: Section::Own,
            control,
            lane: ParameterChannel::Both,
        };
        self.request += 1;
        let edits = [SessionEdit::SetControl {
            expected_effect: None,
            address,
            value,
            ramp_samples: None,
        }];
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        bytes.truncate(length);
        let length = self
            .session
            .command(&bytes, 65_536)
            .expect("the control commits");
        let mut fields = [0_u16; 64];
        let found = match codec
            .decode_typed_response(
                self.session.command_response(length),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a transaction response")
        {
            DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
                ..
            } => applied.edit_path,
            DecodedTypedResponseFrame::NonOk { payload, .. } => {
                panic!("the control is refused: {payload:?}")
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        };
        assert_eq!(found, path, "row {control}: the commit's path");
        self.revision += 1;
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
    }
}

/// Gate 6. Every render block after the warm-up allocates and frees nothing, across a hand move
/// of the automated send gain at block 6 (`live`: the hand offset), a hand edit of the automated
/// `matrix_lr` at block 8 (`model_only`: automation wins), a live edit of the un-automated
/// `matrix_rr` at block 10 (D6) and a hand send mute at block 14 (`live`). Not vacuous: the send
/// (the output's right plane) carries audio before the mute and is silent once its ramp has ended.
///
/// Test value: red if the seven-word cell of a route-row-only send, its record writes or the
/// commit's widened pre-write check allocate or free on render, if a hand move of the automated
/// gain or of an un-automated row is not live, if a hand edit of the automated matrix row is
/// acked live, or if the hand mute does not close the send.
#[test]
fn a_send_with_stored_row_automation_renders_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    for _ in 0..2 {
        rig.block();
    }
    let mut open_before = false;
    let mut silent_after = false;
    while rig.block < 32 {
        match rig.block {
            6 => rig.commit(1, 3.0, EditPath::Live),
            8 => rig.commit(4, 0.75, EditPath::ModelOnly),
            10 => rig.commit(6, 0.5, EditPath::Live),
            14 => rig.commit(2, 1.0, EditPath::Live),
            _ => {}
        }
        let block = rig.block;
        let used = rig.block();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "block {block} allocated or freed"
        );
        let right = &rig.last[QUANTUM..];
        if block == 12 {
            open_before = right.iter().any(|sample| *sample != 0.0);
        }
        if block == 30 {
            silent_after = right.iter().all(|sample| *sample == 0.0);
        }
    }
    assert!(open_before, "the send carries audio before the hand mute");
    assert!(silent_after, "the hand mute closes the send");
}
