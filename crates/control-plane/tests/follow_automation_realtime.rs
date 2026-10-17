//! #1652 gate 5: a send that follows a stored mute renders allocation-free through the shared
//! commit.
//!
//! Gate 1's script runs through control-plane's shared commit (`compile_children` and
//! `SessionState::command`, the code the browser Worker and the C ABI both run) and host-core's
//! render session (`PlanState::render`): track `eq0` sends `pre_fader` with `follows_mute` into bus
//! `b`, and a mute entry on `both` steps `eq0` muted at 1,000 and open again at 2,500 (not grid
//! samples). A live move of `b`'s fader (a value the following route does not hold) commits in
//! the middle of the muted span. Every render call after the warm-up allocates and frees nothing,
//! measured with the audited allocator's thread-scoped counters: the blocks that cross the curve's
//! events, the follow's ramps and the bus move.

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
/// The mute steps of gate 1's curve.
const DOWN: u64 = 1_000;
const UP: u64 = 2_500;

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
/// output's left plane, `eq0-b` sends its right lane from `pre_fader` into bus `b` (and follows its
/// mute), and `b-main` takes the bus to the output, so the output's right plane is the send.
/// `eq0` has a mute entry on `both` with steps 0, 1, 0 at [`DOWN`] and [`UP`].
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
    let mut send = route(
        "eq0-b",
        RouteSource::Track {
            track_id: id("eq0"),
            tap: SendTap::PreFader,
        },
        RouteDestination::SubmixInput { submix_id: id("b") },
        matrix(0.0, 1.0),
    );
    send.follows_mute = true;
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
        send,
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
    let step = |start: u64, end: u64, from: f32, to: f32| AutomationSegment {
        shape: AutomationShape::Step,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit: ParameterUnit::Linear,
    };
    model.automation.push(Automation {
        id: id("eq0-mute"),
        target: ControlAddress {
            owner: Owner::Strip(id("eq0")),
            section: Section::Builtins,
            control: 6,
            lane: ParameterChannel::Both,
        },
        segments: vec![step(0, DOWN, 0.0, 1.0), step(DOWN, UP, 1.0, 0.0)],
        offset: AutomationOffset::default(),
    });
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

    /// Commit bus `b`'s fader at `db` on both lanes live, with the session's length.
    fn move_bus(&mut self, db: f32) {
        self.request += 1;
        let edits = [SessionEdit::SetControl {
            expected_effect: None,
            address: ControlAddress {
                owner: Owner::Strip(id("b")),
                section: Section::Builtins,
                control: 5,
                lane: ParameterChannel::Both,
            },
            value: db,
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
            .expect("the move commits");
        let mut fields = [0_u16; 64];
        let path = match codec
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
                panic!("the move is refused: {payload:?}")
            }
            DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
        };
        assert_eq!(path, EditPath::Live, "the bus move is live");
        self.revision += 1;
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
    }
}

/// Gate 5. Every render block after the warm-up allocates and frees nothing, across the curve's
/// mute at 1,000, a live bus move at 1,792 (inside the muted span) and the unmute at 2,500. Not
/// vacuous: the send (the output's right plane) is silent in a block of the muted span and
/// carries the source again after the unmute.
///
/// Test value: red if the route op's evaluation of the curve, its cursors, its in-block retarget
/// or the cell read allocates or frees on render.
#[test]
fn a_following_send_of_a_stored_mute_renders_allocation_free() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    for _ in 0..2 {
        rig.block();
    }
    let mut silent_while_muted = false;
    let mut open_after = false;
    while rig.block < 32 {
        if rig.block == 14 {
            rig.move_bus(-6.0);
        }
        let block = rig.block;
        let used = rig.block();
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "block {block} allocated or freed"
        );
        let right = &rig.last[QUANTUM..];
        if block == 13 {
            silent_while_muted = right.iter().all(|sample| *sample == 0.0);
        }
        if block == 30 {
            open_after = right.iter().all(|sample| *sample != 0.0);
        }
    }
    assert!(silent_while_muted, "the send follows the curve's mute");
    assert!(open_after, "the send reopens after the unmute");
}
