//! #1400 gate 3: the session prepares through the preparer its adapter supplies, and through
//! nothing else.
//!
//! The preparer here counts its calls and delegates to host-core's live-lanes entries, which is
//! the C ABI's own preparation. The boot compile calls it once, with no base. One structural
//! transaction calls it once more, with the running plan's inventory as the base. A shape the
//! crate refuses before the call never reaches it.
#![cfg(feature = "test-support")]

use std::sync::{Arc, Mutex};

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, InputBoundCache, PlanResources,
    RuntimePreparer, compile_children,
};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveValuesSuperseded, PlanStateInventory, PrepareDiagnostics, PreparedHost,
    SuccessorBase, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{RouteSource, SessionModel, StableId, parse_session_json};

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
        bound_cache: &mut InputBoundCache<'_>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
        let live = HostLiveControlRequest::default();
        let base_inventory = successor.as_ref().map(|base| base.inventory.clone());
        let prepared = match successor {
            Some(base) => prepare_host_runtime_with_live_lanes_successor(
                compiled,
                caps,
                &live,
                lanes,
                base,
                Some(bound_cache),
                superseded,
            ),
            None => prepare_host_runtime_with_live_lanes(
                compiled,
                caps,
                &live,
                lanes,
                Some(bound_cache),
                superseded,
            ),
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
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
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
    let codec = ProtocolCodec::default();
    let frame = TypedCommandFrame {
        request_id: RequestId::new(1).expect("nonzero request"),
        expected_revision: ExpectedRevision::Exact(revision),
        payload: CommandPayload::SessionTransactionApply(edits),
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
        None,
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
        None,
    );
    assert!(
        refused.is_err(),
        "the ring is not a multiple of the quantum"
    );
    assert!(calls.lock().expect("calls").seen.is_empty());
}
