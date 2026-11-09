//! Issue #2424: a recovered lane-swap defect leaves residue in its plan, and the session's service
//! step replaces the plan, restarting each strip the defect reached.
//!
//! Each test drives control-plane's own session (`SessionState::command` and `service`, the code
//! both hosts run) and its render half (`PlanState::render`) on one thread. The session is the
//! nine-track EQ fixture at 48 kHz: input filters on every strip and the EQ console slot `eq`,
//! nine strips over two banks or more, so the banks form a pack group. `eq0`'s console EQ band
//! gain is raised on its left lane, so its prepared defaults differ from `eq1`'s, the strip it
//! trades with.
//!
//! The defect is a real one on the plan's real banks: `effect_contract::test_only_strand_lane_swaps`
//! makes the next bank lane swap strand as a broken bank's does (the two lanes' defaults rotated,
//! their payloads not), and a trade of the two strips (`graph::test_only_swap_bank_lanes`, the
//! packer's own move) hits it. The rack resets both lanes to the defaults they then hold, the
//! graph pins both strips and raises its defect watch, and each strip renders with the other's
//! defaults: its composed response is the other strip's.
#![cfg(feature = "test-support")]

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, TestStructuralFaultPhase, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost, ResponseChannels,
    ResponsePreviewGrid, ResponseSnapshotCollector, SuccessorBase, TrackResponse,
    compose_track_response, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec,
    RequestId, SessionEdit, SessionRevision, TypedCommandFrame,
};
use session::{ParameterChannel, StableId};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: usize = 128;
const GRID: ResponsePreviewGrid = ResponsePreviewGrid::Logarithmic {
    points: 64,
    minimum_hz: 20.0,
    maximum_hz: 20_000.0,
};
/// The most blocks a rebuild's adoption may take: a duck-swap waits for its `S`, far inside 64
/// blocks of 128 frames.
const MAX_BLOCKS: usize = 64;

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
        _spectrum: Option<&host_core::SpectrumPreparation>,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<
        (
            PreparedHost,
            HostLiveControlHandles,
            Option<host_core::PreparedSpectrumCaptures>,
        ),
        PrepareDiagnostics,
    > {
        let live = HostLiveControlRequest::default();
        let (host, handles) = match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }?;
        Ok((host, handles, None))
    }
}

fn limits() -> ControlLimits {
    ControlLimits {
        source_ring_frames: 1_024,
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
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: 65_536,
        maximum_replay_bytes: 262_144,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 4,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The nine-track session, with `eq0`'s console EQ band gain at +9 dB on its left lane.
fn document() -> (String, u64) {
    let mut model = session::parse_session_json(SESSION).expect("fixture");
    for param in &mut model.tracks[0].console[0].params {
        if param.parameter_id == 4 && param.channel == ParameterChannel::Left {
            param.value = 9.0;
        }
    }
    (
        session::canonical_session_json(&model).expect("canonical"),
        model.revision,
    )
}

/// A muted copy of `eq1` named `name` on its source, with no route: a structural edit that
/// restarts no strip.
fn muted_track(name: &str) -> SessionEdit {
    let model = session::parse_session_json(SESSION).expect("fixture");
    let mut track = model.tracks[1].clone();
    track.id = id(name);
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    SessionEdit::UpsertTrack { track }
}

/// A strip's composed response curves, left and right.
type Curves = (Option<Vec<f32>>, Option<Vec<f32>>);

fn curves(response: &TrackResponse) -> Curves {
    (response.left_db.clone(), response.right_db.clone())
}

struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    revision: u64,
    request: u64,
    at: u64,
}

impl Rig {
    fn new() -> Self {
        let (document, revision) = document();
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document, limits(), Box::new(Preparer))
                .expect("boot compile");
        let mut rig = Self {
            session,
            plan,
            revision,
            request: 1,
            at: 0,
        };
        rig.block();
        rig.session.service().expect("service");
        rig
    }

    /// Render one block (the sources underrun: silence is enough).
    fn block(&mut self) {
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM)
            .expect("output planes");
        self.plan.render(self.at, output).expect("a rendered block");
        self.at += QUANTUM as u64;
    }

    /// The active plan, as render holds it.
    fn active(&mut self) -> &mut engine::realtime::PreparedRenderPlan {
        self.plan.test_owner_mut().active_plan_mut()
    }

    /// The composed response of `strip` from the active plan, at the current boundary.
    fn response(&mut self, strip: &str) -> TrackResponse {
        let mut collector = ResponseSnapshotCollector::new(48_000, 16, 6, 127);
        let capture = self
            .plan
            .copy_response_snapshot(engine::realtime::ResponseSnapshotRequest {
                track_id: strip,
                sink: &mut collector,
            })
            .expect("a direct capture");
        compose_track_response(
            strip,
            &collector.finish(capture),
            GRID,
            ResponseChannels::Both,
        )
        .expect("a composed response")
    }

    /// Both strips' response curves, left and right.
    fn responses(&mut self) -> [Curves; 2] {
        [curves(&self.response("eq0")), curves(&self.response("eq1"))]
    }

    /// Trade `eq0` and `eq1` while the next bank lane swap strands: the plan's recovered defect.
    /// Render-thread work, between blocks, as the packer's move; the trade allocates nothing.
    fn strand_trade(&mut self) {
        let group = graph::test_only_pack_groups(self.active())
            .into_iter()
            .find(|group| group.chains.first().is_some_and(|chain| !chain.is_empty()))
            .expect("the two strips are banked in a pack group");
        let unit = group.chains[0][0] as usize;
        effect_contract::test_only_strand_lane_swaps(1);
        let mark = bench_support::alloc::current_thread_counters();
        let moved = graph::test_only_swap_bank_lanes(self.active(), (unit, 0), (unit, 1));
        let used = bench_support::alloc::current_thread_delta_since(mark);
        effect_contract::test_only_strand_lane_swaps(0);
        assert!(!moved, "the stranded trade moves nothing");
        assert_eq!(
            (used.allocations, used.deallocations),
            (0, 0),
            "the defect path allocates and frees nothing"
        );
        assert_eq!(
            graph::lane_swap_defect_count(self.active()),
            Some(1),
            "the strand is a recovered defect"
        );
    }

    /// The tracks the active plan pins.
    fn pinned(&mut self) -> Vec<Box<str>> {
        graph::test_only_pinned_tracks(self.active())
    }

    /// The active plan's epoch.
    fn epoch(&self) -> u64 {
        self.plan
            .test_active_epoch()
            .load(core::sync::atomic::Ordering::Acquire)
    }

    /// Render and service block by block until the active plan's epoch passes `epoch`; the
    /// blocks rendered.
    fn render_until_past(&mut self, epoch: u64) -> usize {
        for blocks in 1..=MAX_BLOCKS {
            self.block();
            self.session.service().expect("service");
            if self.epoch() > epoch {
                return blocks;
            }
        }
        panic!("no plan after epoch {epoch} within {MAX_BLOCKS} blocks");
    }

    /// Commit `edits` as one transaction.
    fn transact(&mut self, edits: &[SessionEdit]) {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(edits),
                },
                &mut bytes,
            )
            .expect("a transaction frame");
        bytes.truncate(length);
        let length = self
            .session
            .command(&bytes, 65_536)
            .expect("the edit commits");
        let mut fields = [0_u16; 64];
        match codec
            .decode_typed_response(
                self.session.command_response(length),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a transaction response")
        {
            DecodedTypedResponseFrame::Success { .. } => {}
            DecodedTypedResponseFrame::NonOk { payload, .. } => {
                panic!("the edit is refused: {payload:?}")
            }
        }
        self.revision += 1;
        while self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("an event")
            .is_some()
        {}
    }
}

/// A plan whose trade strands holds residue, and the next service step replaces it once: the
/// successor restarts both strips, so each renders its own response again, and pins both, so its
/// packer never trades them again. Further service calls rebuild nothing. On the base nothing
/// read the defect: the plan kept the residue for its life.
///
/// Test value: no rebuild on a defect (the residue stays), a rebuild that carries the strips
/// instead of restarting them, a successor that does not pin them (a bank that strands at every
/// plan would rebuild at every episode), or a rebuild repeated while the residue is gone.
#[test]
fn a_lane_swap_defect_rebuilds_the_plan_once() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    let own = rig.responses();
    assert_ne!(own[0], own[1], "the two strips' defaults differ");
    assert!(
        rig.pinned().is_empty(),
        "nothing is pinned before the defect"
    );
    let before = rig.epoch();
    rig.strand_trade();
    let traded = rig.responses();
    assert_eq!(
        traded,
        [own[1].clone(), own[0].clone()],
        "each strip renders the other's defaults: the residue"
    );
    rig.session.service().expect("service");
    assert_eq!(
        rig.session.lane_swap_rebuilds(),
        (1, 0),
        "one rebuild, published"
    );
    rig.render_until_past(before);
    assert_eq!(
        rig.responses(),
        own,
        "each strip renders its own defaults again"
    );
    assert_eq!(
        rig.pinned(),
        vec![Box::from("eq0"), Box::from("eq1")],
        "the successor pins the defect pair"
    );
    assert_eq!(graph::lane_swap_defect_count(rig.active()), Some(0));
    for _ in 0..8 {
        rig.block();
        rig.session.service().expect("service");
    }
    assert_eq!(
        rig.session.lane_swap_rebuilds(),
        (1, 0),
        "no further rebuild"
    );
}

/// A refused residue rebuild is counted and not retried at every service call; the next
/// successor any edit makes restarts the residue all the same, and pins the pair.
///
/// Test value: a refusal retried at every call (a rebuild storm of refused preparations), or a
/// residue that only the rebuild itself restarts, so an ordinary successor carries it.
#[test]
fn a_refused_rebuild_is_not_retried_and_any_successor_restarts_the_residue() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    let own = rig.responses();
    rig.strand_trade();
    rig.session
        .test_set_structural_faults([Some(TestStructuralFaultPhase::AfterRuntimePrepare), None]);
    for _ in 0..4 {
        rig.block();
        rig.session.service().expect("service");
    }
    assert_eq!(
        rig.session.lane_swap_rebuilds(),
        (0, 1),
        "one refused attempt, never retried"
    );
    assert_eq!(rig.responses()[1], own[0], "the residue stays");
    let before = rig.epoch();
    rig.transact(&[muted_track("eq9")]);
    rig.render_until_past(before);
    assert_eq!(
        rig.responses(),
        own,
        "the edit's successor restarted both strips"
    );
    assert_eq!(rig.pinned(), vec![Box::from("eq0"), Box::from("eq1")]);
    assert_eq!(
        rig.session.lane_swap_rebuilds(),
        (0, 1),
        "no rebuild needed"
    );
}

/// A defect after a successor was prepared, before render adopted it: the successor carried the
/// residue it did not know of, takes it over at its promotion, and the service step rebuilds it.
///
/// Test value: a promotion that drops the displaced plan's late flags, so the successor keeps
/// the residue for its life.
#[test]
fn a_successor_adopted_after_a_late_defect_is_rebuilt() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    let own = rig.responses();
    let before = rig.epoch();
    rig.transact(&[muted_track("eq9")]);
    rig.strand_trade();
    rig.render_until_past(before);
    assert_eq!(
        rig.session.lane_swap_rebuilds(),
        (1, 0),
        "the promotion's inherited residue rebuilt"
    );
    let adopted = rig.epoch();
    rig.render_until_past(adopted);
    assert_eq!(rig.responses(), own, "both strips are their own again");
    assert_eq!(rig.pinned(), vec![Box::from("eq0"), Box::from("eq1")]);
}

/// A residue rebuild is never warm: with the session's warm bounds and a classification forced
/// to `Warm`, it still restarts the residue, so the plan it publishes holds none and the service
/// step does not rebuild again.
///
/// Test value: a residue rebuild classified warm, whose successor restarts nothing and inherits
/// the residue at its promotion, so every promotion rebuilds again.
#[test]
fn a_residue_rebuild_is_never_warm() {
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new();
    let own = rig.responses();
    let before = rig.epoch();
    rig.strand_trade();
    rig.session.test_set_next_zero_lead(Vec::new());
    rig.session.service().expect("service");
    rig.render_until_past(before);
    for _ in 0..8 {
        rig.block();
        rig.session.service().expect("service");
    }
    assert_eq!(rig.session.lane_swap_rebuilds(), (1, 0), "one rebuild");
    assert_eq!(rig.responses(), own);
}
