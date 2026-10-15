//! #1584 D6a: the subscriptions a host acknowledged survive every rebuild, and a rebuild that
//! removes their effect tells the subscribers.
//!
//! The session holds the table of armed taps next to the monitor overlay. A push to a tap's queue
//! writes it; a rebuild prepares its successor from it and drops the bindings whose target the
//! transaction removed, emitting one reliable `OBSERVATION_ENDED` per binding after the
//! transaction's `SESSION_COMMITTED`, the room for all of them reserved before the commit.
#![cfg(feature = "test-support")]

use control_plane::{
    CommandError, CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources,
    PlanState, RuntimePreparer, SessionState, compile_children,
};
use controls::{TapKey, TapSection};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveEffectAddress, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    SuccessorBase, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_successor,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedEventPayload, ExpectedRevision, ObservationSection,
    ProtocolCodec, RequestId, SessionEdit, SessionRevision, TypedCommandFrame,
};
use session::{
    ControlAddress, Effect, EffectIdentity, EffectQuality, Lane, LinkMode, Owner, RackName,
    Section, SidechainDeclaration, StableId,
};

const SESSION: &str =
    include_str!("../../../fixtures/session/v1/compressor-dynamic-bank-observation.json");
const WINDOW_BLOCKS: u32 = 4;

struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

/// A preparer that asks for observation capacity, as a host with subscriptions does.
struct ObservingPreparer;

impl RuntimePreparer for ObservingPreparer {
    fn prepare(
        &self,
        compiled: &CompiledSession,
        caps: &HostPrepareCaps,
        lanes: HostLiveLanes,
        successor: Option<SuccessorBase<'_>>,
        superseded: &LiveValuesSuperseded,
    ) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
        let live = HostLiveControlRequest {
            observation_taps: 4,
            ..HostLiveControlRequest::default()
        };
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
        maximum_submixes: 0,
        maximum_vcas: 0,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_replacement_peak_bytes: u64::MAX,
    }
}

struct Rig {
    session: SessionState<Adapter>,
    plan: PlanState<Adapter>,
    next_request: u64,
    block: u64,
}

impl Rig {
    fn new() -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(SESSION, limits(), Box::new(ObservingPreparer))
                .expect("boot compile");
        Self {
            session,
            plan,
            next_request: 1,
            block: 0,
        }
    }

    /// Render one block, so the plan drains the observation queues.
    fn render(&mut self) {
        let mut samples = [0.0_f32; 256];
        let output = engine::realtime::PlanarBufferMut::try_new(&mut samples, 2, 128, 128)
            .expect("output planes");
        self.plan
            .render(self.block * 128, output)
            .expect("a rendered block");
        self.block += 1;
    }

    fn revision(&self) -> u64 {
        self.session.test_controller().session().revision().0
    }

    fn frame(&self, request: u64, edits: &[SessionEdit]) -> Vec<u8> {
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(request).expect("nonzero request"),
            expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision())),
            payload: CommandPayload::SessionTransactionApply(edits),
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("typed command");
        bytes.truncate(length);
        bytes
    }

    /// Submit `edits` as a new request.
    fn apply(&mut self, edits: &[SessionEdit]) -> Result<usize, CommandError> {
        let request = self.next_request;
        self.next_request += 1;
        let bytes = self.frame(request, edits);
        self.session.command(&bytes, 65_536)
    }

    /// Arm tap 0 (gain reduction) of `strip`'s `comp` insert, as the browser's kind 7 does.
    fn subscribe(&mut self, strip: &str) {
        let position = self.position(strip);
        self.session
            .push_effect_observe(position, 0, true, WINDOW_BLOCKS)
            .expect("a producer")
            .expect("room in the queue");
    }

    fn unsubscribe(&mut self, strip: &str) {
        let position = self.position(strip);
        self.session
            .push_effect_observe(position, 0, false, 0)
            .expect("a producer")
            .expect("room in the queue");
    }

    fn position(&self, strip: &str) -> usize {
        self.session
            .effect_producers()
            .iter()
            .position(|producer| {
                &*producer.track_id == strip && producer.address == LiveEffectAddress::insert(0)
            })
            .expect("the strip's insert has a producer")
    }

    /// Every reliable event queued, decoded: `(revision, payload)`.
    fn drain(&mut self) -> Vec<(u64, DecodedEventPayload<'static>)> {
        let codec = ProtocolCodec::default();
        let mut events = Vec::new();
        while let Some(length) = self
            .session
            .dequeue_event(EventLane::Reliable, 65_536)
            .expect("event")
        {
            let bytes = self.session.event_response(length).to_vec();
            let mut fields = vec![0_u16; 4_096];
            let mut scratch = DecodeScratch::new(&mut fields);
            let decoded = codec
                .decode_typed_event(&bytes, &mut scratch)
                .expect("an event frame");
            let revision = decoded.header.revision.0;
            // Only the owned payloads this test reads are kept.
            let payload = match decoded.payload {
                DecodedEventPayload::SessionCommitted(value) => {
                    DecodedEventPayload::SessionCommitted(value)
                }
                DecodedEventPayload::ObservationEnded(value) => {
                    DecodedEventPayload::ObservationEnded(value)
                }
                _ => continue,
            };
            events.push((revision, payload));
        }
        events
    }
}

fn key(strip: &str) -> TapKey {
    TapKey {
        strip: StableId::parse(strip).expect("strip"),
        section: TapSection::Inserts(StableId::parse("comp").expect("insert")),
        tap_id: 1,
    }
}

fn insert_address(strip: &str, control: u32) -> ControlAddress {
    ControlAddress {
        owner: Owner::Strip(StableId::parse(strip).expect("strip")),
        section: Section::Inserts(StableId::parse("comp").expect("insert")),
        control,
        lane: Lane::Both,
    }
}

fn remove_insert(strip: &str) -> SessionEdit {
    SessionEdit::RemoveTrackEffect {
        track_id: StableId::parse(strip).expect("strip"),
        rack_name: RackName::Inserts,
        effect_id: StableId::parse("comp").expect("insert"),
    }
}

/// An insert `comp` that is a delay: the ID of the compressor, reused for an effect with no taps.
fn delay_as_comp(strip: &str) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: StableId::parse(strip).expect("strip"),
        rack_name: RackName::Inserts,
        final_position: 0,
        effect: Effect {
            id: StableId::parse("comp").expect("insert"),
            identity: EffectIdentity::Native {
                effect_id: StableId::parse("miso.delay").expect("delay"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        },
    }
}

fn ended(events: &[(u64, DecodedEventPayload<'static>)]) -> Vec<(u64, protocol::ObservationEnded)> {
    events
        .iter()
        .filter_map(|(revision, payload)| match payload {
            DecodedEventPayload::ObservationEnded(value) => Some((*revision, value.clone())),
            _ => None,
        })
        .collect()
}

fn committed(events: &[(u64, DecodedEventPayload<'static>)]) -> usize {
    events
        .iter()
        .filter(|(_, payload)| matches!(payload, DecodedEventPayload::SessionCommitted(_)))
        .count()
}

/// The push is the acknowledgement: it writes the table with it, a disarm clears it, and a push
/// the lane refuses writes nothing.
///
/// Red if the table is written before the push succeeds, ignores a disarm, or keys a tap by
/// anything but its strip, section and tap ID.
#[test]
fn a_push_writes_the_table_and_a_refused_push_writes_nothing() {
    let mut rig = Rig::new();
    assert!(rig.session.observation_bindings().is_empty());
    rig.subscribe("comp1");
    rig.subscribe("comp3");
    let bindings = rig.session.observation_bindings();
    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings.window_of(&key("comp1")), Some(WINDOW_BLOCKS));
    // The queue holds one slot per declared tap, so a disarm before any render is refused, and
    // the binding stays: the table never says less than the lane was told.
    let position = rig.position("comp1");
    let refused = rig
        .session
        .push_effect_observe(position, 0, false, 0)
        .expect("a producer");
    assert!(refused.is_err(), "the queue has no room for the disarm");
    assert_eq!(rig.session.observation_bindings().len(), 2);
    // After a render drained it, the disarm is accepted and clears the binding.
    rig.render();
    rig.unsubscribe("comp1");
    assert_eq!(rig.session.observation_bindings().len(), 1);
    assert_eq!(
        rig.session.observation_bindings().window_of(&key("comp1")),
        None
    );
    assert_eq!(
        rig.session.observation_bindings().window_of(&key("comp3")),
        Some(WINDOW_BLOCKS)
    );
}

/// A subscription survives a rebuild that restarts its effect (here a link-mode change): the
/// table is unchanged and the transaction ends nothing, so no `OBSERVATION_ENDED` follows the
/// `SESSION_COMMITTED`. (That the restarted tap is armed and opens at the adoption block is
/// host-core's gate.)
///
/// Red if a rebuild empties the table or ends a binding whose effect is still there.
#[test]
fn a_restart_keeps_the_binding_and_ends_nothing() {
    let mut rig = Rig::new();
    rig.subscribe("comp1");
    let revision = rig.revision();
    rig.apply(&[SessionEdit::set_control(
        insert_address("comp1", controls::LINK_MODE_CONTROL),
        effect_contract::LinkMode::Average as u32 as f32,
        None,
    )])
    .expect("a rebuild");
    assert_eq!(rig.revision(), revision + 1);
    assert_eq!(rig.session.observation_bindings().len(), 1);
    assert_eq!(
        rig.session.observation_bindings().window_of(&key("comp1")),
        Some(WINDOW_BLOCKS)
    );
    let events = rig.drain();
    assert_eq!(committed(&events), 1);
    assert!(ended(&events).is_empty());
}

/// Each way an effect leaves ends its binding with one reliable event after the transaction's
/// `SESSION_COMMITTED`, at the committing revision, naming the stable key; a binding of another
/// strip stays.
///
/// Red if a removed effect keeps its binding silently (a subscriber waits for windows that never
/// come), if the event names the wrong revision or key, or if a surviving binding is ended.
#[test]
fn a_removed_effect_ends_its_binding_with_an_event() {
    let cases: [(&str, Vec<SessionEdit>); 3] = [
        ("an insert removed", vec![remove_insert("comp1")]),
        (
            "a strip removed",
            vec![
                SessionEdit::RemoveRoute {
                    route_id: StableId::parse("comp1-main").expect("route"),
                },
                SessionEdit::RemoveTrack {
                    track_id: StableId::parse("comp1").expect("strip"),
                },
            ],
        ),
        (
            "an ID reused for an effect without the tap",
            vec![delay_as_comp("comp1")],
        ),
    ];
    for (what, edits) in cases {
        let mut rig = Rig::new();
        rig.subscribe("comp1");
        rig.subscribe("comp2");
        let before = rig.revision();
        rig.apply(&edits)
            .unwrap_or_else(|error| panic!("{what}: {error:?}"));
        let revision = rig.revision();
        assert_eq!(revision, before + 1, "{what}");
        assert_eq!(rig.session.observation_bindings().len(), 1, "{what}");
        assert!(
            rig.session
                .effect_producers()
                .iter()
                .filter(|producer| &*producer.track_id == "comp1")
                .all(|producer| producer.observe_capacity() == 0),
            "{what}: the successor has no tap for the ended binding"
        );
        assert_eq!(
            rig.session.observation_bindings().window_of(&key("comp2")),
            Some(WINDOW_BLOCKS),
            "{what}: the other strip's binding stays"
        );
        let events = rig.drain();
        assert_eq!(
            events
                .iter()
                .map(|(_, payload)| matches!(payload, DecodedEventPayload::SessionCommitted(_)))
                .collect::<Vec<_>>(),
            vec![true, false],
            "{what}: SESSION_COMMITTED, then the end"
        );
        let ended = ended(&events);
        assert_eq!(ended.len(), 1, "{what}");
        let (event_revision, event) = &ended[0];
        assert_eq!(*event_revision, revision, "{what}: the committing revision");
        assert_eq!(event.key.strip_id, "comp1", "{what}");
        assert_eq!(event.key.section, ObservationSection::Inserts, "{what}");
        assert_eq!(event.key.effect_id, "comp", "{what}");
        assert_eq!(event.key.tap_id, 1, "{what}");
        // Sequences are endpoint-monotonic: the end follows the commit's by one.
        let DecodedEventPayload::SessionCommitted(commit) = &events[0].1 else {
            panic!("{what}: first event");
        };
        assert_eq!(event.event_sequence, commit.event_sequence + 1, "{what}");
    }
}

/// The room for the ends is checked with the commit's own event before anything changes (the
/// acked-batch rule): a lane with room for `SESSION_COMMITTED` and not for the end refuses the
/// transaction as backpressure, and the model, the revision, the table and the lane are as they
/// were; after a drain the same request commits.
///
/// Red if the commit goes ahead and drops the end, or if a refusal leaves the table emptied or
/// the revision moved.
#[test]
fn no_room_for_the_end_refuses_the_removal_and_changes_nothing() {
    let mut rig = Rig::new();
    rig.subscribe("comp1");
    // One committed event waiting in the two-slot lane: one slot is free.
    rig.apply(&[SessionEdit::SetSessionId {
        session_id: StableId::parse("another-session").expect("ID"),
    }])
    .expect("a model-only commit");
    let revision = rig.revision();
    let canonical = rig.session.test_transaction_snapshot().canonical;
    let bindings = rig.session.observation_bindings().clone();
    let epoch = rig.session.newest_plan_epoch();
    let request = rig.next_request;
    let bytes = rig.frame(request, &[remove_insert("comp1")]);
    rig.next_request += 1;
    let refused = rig.session.command(&bytes, 65_536);
    assert!(
        matches!(refused, Err(CommandError::Backpressure)),
        "{refused:?}"
    );
    assert_eq!(rig.revision(), revision);
    assert_eq!(rig.session.test_transaction_snapshot().canonical, canonical);
    assert_eq!(rig.session.observation_bindings(), &bindings);
    assert_eq!(
        rig.session.newest_plan_epoch(),
        epoch,
        "no plan was published"
    );
    let events = rig.drain();
    assert_eq!(committed(&events), 1, "only the earlier commit's event");
    assert!(ended(&events).is_empty());

    // The lane is empty: the same request now commits, with its end.
    let bytes = rig.frame(request, &[remove_insert("comp1")]);
    rig.session
        .command(&bytes, 65_536)
        .expect("the same removal commits after a drain");
    assert_eq!(rig.revision(), revision + 1);
    assert!(rig.session.observation_bindings().is_empty());
    let events = rig.drain();
    assert_eq!(committed(&events), 1);
    assert_eq!(ended(&events).len(), 1);
}
