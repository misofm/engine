//! #1400 gate 3: the session prepares through the preparer its adapter supplies, and through
//! nothing else.
//!
//! The preparer here counts its calls and delegates to host-core's live-lanes entries, which is
//! the C ABI's own preparation. The boot compile calls it once, with no base. One structural
//! transaction calls it once more, with the running plan's inventory as the base. A shape the
//! crate refuses before the call never reaches it.
//!
//! #1768 gate 1 runs on the same preparer: the session's `STRIPS_RESTARTED` counter adds the
//! strips each plan render adopts lists as restarted, and nothing for a plan render never adopts.
#![cfg(feature = "test-support")]

use std::sync::{Arc, Mutex};

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, PlanState,
    RuntimePreparer, SessionState, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PlanStateInventory, PrepareDiagnostics, PreparedHost,
    SuccessorBase, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, CounterId, CountersRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, TypedCommandFrame,
};
use session::{
    Effect, EffectIdentity, EffectQuality, LinkMode, RackName, RouteSource, SessionModel,
    SidechainDeclaration, StableId, parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

/// What the preparer saw, one entry per call.
#[derive(Default)]
struct Calls {
    /// Per call: whether it carried a base, and the base's inventory.
    seen: Vec<Option<PlanStateInventory>>,
    /// The inventory each call's plan reports, for the next call's base to equal.
    produced: Vec<PlanStateInventory>,
}

struct CountingPreparer(Arc<Mutex<Calls>>);

impl RuntimePreparer for CountingPreparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
        let live = HostLiveControlRequest::default();
        let base_inventory = successor.as_ref().map(|base| base.inventory.clone());
        let prepared = match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled, caps, &live, lanes, base, superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(compiled, caps, &live, lanes, superseded),
        }?;
        let mut calls = self.0.lock().expect("calls");
        calls.seen.push(base_inventory);
        calls.produced.push(prepared.0.inventory.clone());
        drop(calls);
        Ok(prepared)
    }
}

fn limits(source_ring_frames: u32) -> ControlLimits {
    ControlLimits {
        source_ring_frames,
        maximum_automation_spans_per_block: 128,
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
        maximum_control_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

/// `UpsertTrack` of a muted copy of the first track, and `UpsertRoute` of its route.
fn add_track(model: &SessionModel, id: &str) -> [SessionEdit; 2] {
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse(id).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = StableId::parse(&format!("{id}-main")).expect("route ID");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn transaction_bytes(edits: &[SessionEdit], revision: SessionRevision) -> Vec<u8> {
    command_bytes(
        1,
        ExpectedRevision::Exact(revision),
        CommandPayload::SessionTransactionApply(edits),
    )
}

fn command_bytes(
    request: u64,
    expected_revision: ExpectedRevision,
    payload: CommandPayload<'_>,
) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = TypedCommandFrame {
        request_id: RequestId::new(request).expect("nonzero request"),
        expected_revision,
        payload,
    };
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("typed command");
    bytes.truncate(length);
    bytes
}

/// Test value: red if the structural arm still calls host-core directly with a fixed
/// preparation (the preparer's count stays at one, so a browser successor would get the C ABI's
/// request), if the successor is prepared without the running plan's base (`seen[1]` is `None`,
/// and every source ring would be reallocated), or if the boot compile prepares more than once.
#[test]
fn the_session_prepares_through_its_preparer_at_boot_and_for_a_structural_edit() {
    let calls = Arc::new(Mutex::new(Calls::default()));
    let CompiledChildren {
        mut session,
        plan: _plan,
    } = compile_children::<Adapter>(
        SESSION,
        limits(1_024),
        Box::new(CountingPreparer(Arc::clone(&calls))),
    )
    .expect("boot compile");
    {
        let calls = calls.lock().expect("calls");
        assert_eq!(calls.seen.len(), 1, "boot prepares once");
        assert!(calls.seen[0].is_none(), "boot prepares without a base");
    }

    let model = parse_session_json(SESSION).expect("fixture");
    let edits = add_track(&model, "a-added");
    let revision = session.test_controller().session().revision();
    session
        .command(&transaction_bytes(&edits, revision), 4_096)
        .expect("structural transaction");
    let calls = calls.lock().expect("calls");
    assert_eq!(
        calls.seen.len(),
        2,
        "one structural transaction prepares once"
    );
    let base = calls.seen[1]
        .as_ref()
        .expect("the successor carries a base");
    assert_eq!(
        *base, calls.produced[0],
        "the base is the running plan's inventory"
    );
}

/// Test value: red if the shape check moves after the preparer call (a ring that is not a
/// multiple of the quantum would reach the preparer, and host-core would be asked to allocate
/// for a shape the control plane refuses), or if a refused compile still counts a call.
#[test]
fn a_refused_shape_never_reaches_the_preparer() {
    let calls = Arc::new(Mutex::new(Calls::default()));
    // 1,000 frames is not a multiple of the fixture's 128-frame quantum.
    let refused = compile_children::<Adapter>(
        SESSION,
        limits(1_000),
        Box::new(CountingPreparer(Arc::clone(&calls))),
    );
    assert!(
        refused.is_err(),
        "the ring is not a multiple of the quantum"
    );
    assert!(calls.lock().expect("calls").seen.is_empty());
}

/// The session's `STRIPS_RESTARTED` count, read with `COUNTERS_GET` (request `request`).
fn strips_restarted(session: &mut SessionState<Adapter>, request: u64) -> u64 {
    let bytes = command_bytes(
        request,
        ExpectedRevision::Any,
        CommandPayload::CountersGet(&CountersRequest {
            all: false,
            ids: vec![CounterId::StripsRestarted as u32],
        }),
    );
    let length = session.command(&bytes, 4_096).expect("counters");
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success {
        payload: DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(
            session.command_response(length),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a counters response")
    else {
        panic!("expected a counter snapshot")
    };
    assert_eq!(snapshot.values.len(), 1);
    assert_eq!(snapshot.values[0].id, CounterId::StripsRestarted);
    snapshot.values[0].value
}

/// Commit `edits` as one rebuild transaction (request `request`) and drain the reliable events.
fn rebuild(session: &mut SessionState<Adapter>, request: u64, edits: &[SessionEdit]) {
    let revision = session.test_controller().session().revision();
    let bytes = command_bytes(
        request,
        ExpectedRevision::Exact(revision),
        CommandPayload::SessionTransactionApply(edits),
    );
    let length = session
        .command(&bytes, 4_096)
        .expect("structural transaction");
    let mut fields = [0_u16; 64];
    let path = match ProtocolCodec::default()
        .decode_typed_response(
            session.command_response(length),
            &mut DecodeScratch::new(&mut fields),
        )
        .expect("a transaction response")
    {
        DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
            ..
        } => applied.edit_path,
        DecodedTypedResponseFrame::NonOk { payload, .. } => {
            panic!("request {request} is refused: {payload:?}")
        }
        DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
    };
    assert_eq!(path, EditPath::Rebuild, "request {request} rebuilds");
    while session
        .dequeue_event(EventLane::Reliable, 4_096)
        .expect("an event")
        .is_some()
    {}
}

/// Render block `block` (no source PCM: the block underruns to silence, which is enough to adopt
/// a waiting candidate).
fn render(plan: &mut PlanState<Adapter>, block: u64) {
    let mut samples = [0.0_f32; 256];
    let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, 128, 128)
        .expect("output planes");
    plan.render(block * 128, output).expect("a rendered block");
}

/// A default `miso.compressor` insert at the head of `track`'s inserts: a changed effect chain,
/// so the strip starts at rest on the swap.
fn put_compressor(track: &str) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: StableId::parse(track).expect("track ID"),
        rack_name: RackName::Inserts,
        final_position: 0,
        effect: Effect {
            id: StableId::parse("comp").expect("effect ID"),
            identity: EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("effect"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        },
    }
}

/// The strips the preparer's call `call` listed as restarted.
fn listed(calls: &Mutex<Calls>, call: usize) -> Vec<String> {
    calls.lock().expect("calls").produced[call]
        .restarted_strips()
        .iter()
        .map(|id| id.to_string())
        .collect()
}

/// #1768 gate 1. A rebuild that restarts two strips adds 2 to `STRIPS_RESTARTED`, and only once
/// render adopts it. A candidate withdrawn unadopted adds nothing: its replacement, prepared
/// against the plan render still runs, adds its own list (the withdrawn edit's strip and its own).
/// A rebuild that restarts no strip adds 0.
///
/// Test value: red if the count is added at commit or preparation instead of at adoption (it
/// reads 2 before the first render), if a withdrawn candidate's strips are counted (5, not 4), or
/// if the counter is not copied into the provider (0).
#[test]
fn strips_restarted_counts_the_restarts_of_every_adopted_plan_and_of_no_other() {
    let calls = Arc::new(Mutex::new(Calls::default()));
    let CompiledChildren {
        mut session,
        mut plan,
    } = compile_children::<Adapter>(
        SESSION,
        limits(1_024),
        Box::new(CountingPreparer(Arc::clone(&calls))),
    )
    .expect("boot compile");
    render(&mut plan, 0);
    assert_eq!(
        strips_restarted(&mut session, 2),
        0,
        "boot restarts nothing"
    );

    rebuild(
        &mut session,
        3,
        &[put_compressor("eq1"), put_compressor("eq2")],
    );
    assert_eq!(listed(&calls, 1), ["eq1", "eq2"]);
    assert_eq!(
        strips_restarted(&mut session, 4),
        0,
        "the candidate waits: render has not adopted it"
    );
    render(&mut plan, 1);
    assert_eq!(strips_restarted(&mut session, 5), 2, "render adopted it");

    // Candidate A (eq3) is withdrawn unadopted by B (eq4), prepared against the running plan.
    rebuild(&mut session, 6, &[put_compressor("eq3")]);
    rebuild(&mut session, 7, &[put_compressor("eq4")]);
    assert_eq!(listed(&calls, 2), ["eq3"]);
    assert_eq!(listed(&calls, 3), ["eq3", "eq4"]);
    render(&mut plan, 2);
    assert_eq!(
        strips_restarted(&mut session, 8),
        4,
        "B's two strips, and nothing of the withdrawn A"
    );

    // A content identity change rebuilds and carries every strip.
    let model = parse_session_json(SESSION).expect("fixture");
    let source = &model.sources[0];
    rebuild(
        &mut session,
        9,
        &[SessionEdit::SetSourceContent {
            source_id: source.id.clone(),
            content: format!("blake3:{}", "ab".repeat(32)),
            channels: source.channels,
            bit_depth: source.bit_depth,
            frames: source.frames,
        }],
    );
    assert!(listed(&calls, 4).is_empty());
    render(&mut plan, 3);
    assert_eq!(strips_restarted(&mut session, 10), 4, "no strip restarted");
}
