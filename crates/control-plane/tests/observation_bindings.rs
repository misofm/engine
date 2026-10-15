//! #1584 D6a: the subscriptions a host acknowledged survive every rebuild, and a rebuild that
//! removes their effect tells the subscribers.
//!
//! The session holds the table of armed taps next to the monitor overlay. A push to a tap's queue
//! writes it; a rebuild prepares its successor from it and drops the bindings whose target the
//! transaction removed, emitting ONE reliable `OBSERVATION_ENDED` per commit, carrying every ended
//! key, after the transaction's `SESSION_COMMITTED`, the room for it reserved before the commit.
#![cfg(feature = "test-support")]

use control_plane::{
    CommandError, CompileFailure, CompiledChildren, ControlAdapter, ControlLimits, EventLane,
    ObservationChange, ObservationKeyError, ObservationKeyRef, ObservationLimit,
    ObservationRefusal, PlanResources, PlanState, ResolvedObservationTap, ResourceFault,
    RuntimePreparer, SessionState, compile_children,
};
use controls::{TapKey, TapSection};
use host_core::{
    CompiledSession, HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes,
    HostPrepareCaps, LiveEffectAddress, LiveEffectRack, LiveValuesSuperseded, PrepareDiagnostics,
    PreparedHost, SuccessorBase, prepare_host_runtime_with_live_lanes,
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
        Self::with_limits(limits())
    }

    fn with_limits(limits: ControlLimits) -> Self {
        Self::with_session(SESSION, limits)
    }

    fn with_session(document: &str, limits: ControlLimits) -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(document, limits, Box::new(ObservingPreparer))
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
        self.observe(position, true).expect("admitted");
    }

    fn unsubscribe(&mut self, strip: &str) {
        let position = self.position(strip);
        self.observe(position, false).expect("admitted");
    }

    /// Arm or disarm tap 0 of the producer at `position`, as a batch of one: the one way a host
    /// queues an observation record (#1676).
    fn observe(&mut self, position: usize, armed: bool) -> Result<(), ObservationRefusal> {
        self.batch(&[change(position, armed)])
    }

    fn batch(&mut self, changes: &[ObservationChange]) -> Result<(), ObservationRefusal> {
        self.session
            .admit_and_push_observation_batch(changes.iter().copied())
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

/// Arm (with the test's window) or disarm tap 0 of the producer at `position`.
fn change(position: usize, armed: bool) -> ObservationChange {
    ObservationChange {
        position,
        tap_index: 0,
        armed,
        window_blocks: if armed { WINDOW_BLOCKS } else { 0 },
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
    let refused = rig.observe(position, false);
    assert_eq!(
        refused,
        Err(ObservationRefusal::Full { index: 0 }),
        "the queue has no room for the disarm"
    );
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
        assert_eq!(event.keys.len(), 1, "{what}");
        assert_eq!(event.keys[0].strip_id, "comp1", "{what}");
        assert_eq!(event.keys[0].section, ObservationSection::Inserts, "{what}");
        assert_eq!(event.keys[0].effect_id, "comp", "{what}");
        assert_eq!(event.keys[0].tap_id, 1, "{what}");
        // Sequences are endpoint-monotonic: the end follows the commit's by one.
        let DecodedEventPayload::SessionCommitted(commit) = &events[0].1 else {
            panic!("{what}: first event");
        };
        assert_eq!(event.event_sequence, commit.event_sequence + 1, "{what}");
    }
}

/// The room for the end event is checked with the commit's own event before anything changes (the
/// acked-batch rule): a lane that holds an unread event, leaving room for `SESSION_COMMITTED` and
/// not for the end, refuses the transaction as backpressure, and the model, the revision, the table
/// and the lane are as they were; after a drain the same request commits. Backpressure shows only
/// while the lane holds unread events: the empty lane takes the same removal.
///
/// Red if the commit goes ahead and drops the end, or if a refusal leaves the table emptied or
/// the revision moved.
#[test]
fn no_room_for_the_end_refuses_the_removal_and_changes_nothing() {
    let mut rig = Rig::new();
    rig.subscribe("comp1");
    // One committed event unread in the two-slot lane: one slot is free, the removal needs two.
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

/// Root ruling A: a transaction that removes a strip with N subscribed inserts commits whatever N is
/// (the lane has two slots), and gives exactly one `OBSERVATION_ENDED` after its
/// `SESSION_COMMITTED`, at the committing revision, with the N stable keys in key order.
///
/// Red if the keys are split across events (the second no longer fits the two-slot lane and the
/// removal is refused as backpressure forever), if the event holds fewer or more keys than the
/// commit ended, or if the keys are out of key order.
#[test]
fn a_strip_removed_with_many_subscribed_inserts_gives_one_event_with_every_key() {
    fn put_compressor(strip: &str, id: &str) -> SessionEdit {
        SessionEdit::PutTrackEffect {
            track_id: StableId::parse(strip).expect("strip"),
            rack_name: RackName::Inserts,
            final_position: 1,
            effect: Effect {
                id: StableId::parse(id).expect("insert"),
                identity: EffectIdentity::Native {
                    effect_id: StableId::parse("miso.compressor").expect("compressor"),
                },
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::DualMono,
                params: Vec::new(),
                sidechain: SidechainDeclaration::None,
            },
        }
    }
    for count in [1_u32, 2, 7, 40] {
        let mut rig = Rig::new();
        let ids: Vec<String> = (0..count).map(|index| format!("ins{index:02}")).collect();
        let puts: Vec<SessionEdit> = ids.iter().map(|id| put_compressor("comp1", id)).collect();
        rig.apply(&puts).expect("the inserts are added");
        // The plan with the inserts is adopted at a block, so the next rebuild has room.
        rig.render();
        rig.render();
        rig.drain();
        for id in &ids {
            let position = rig
                .session
                .effect_producers()
                .iter()
                .position(|producer| &*producer.track_id == "comp1" && &*producer.effect_id == id)
                .expect("the insert has a producer");
            rig.observe(position, true).expect("under the cap");
        }
        assert_eq!(rig.session.observation_bindings().len(), ids.len());
        let before = rig.revision();
        rig.apply(&[
            SessionEdit::RemoveRoute {
                route_id: StableId::parse("comp1-main").expect("route"),
            },
            SessionEdit::RemoveTrack {
                track_id: StableId::parse("comp1").expect("strip"),
            },
        ])
        .unwrap_or_else(|error| panic!("{count} subscribed inserts: {error:?}"));
        assert_eq!(rig.revision(), before + 1, "{count}");
        assert!(rig.session.observation_bindings().is_empty(), "{count}");
        let events = rig.drain();
        assert_eq!(events.len(), 2, "{count}: SESSION_COMMITTED and one end");
        let ended = ended(&events);
        assert_eq!(ended.len(), 1, "{count}: exactly one event");
        let (revision, event) = &ended[0];
        assert_eq!(*revision, before + 1, "{count}");
        let got: Vec<(&str, &str, u32)> = event
            .keys
            .iter()
            .map(|key| (&*key.strip_id, &*key.effect_id, key.tap_id))
            .collect();
        let want: Vec<(&str, &str, u32)> = ids.iter().map(|id| ("comp1", id.as_str(), 1)).collect();
        assert_eq!(got, want, "{count}: every key, in key order");
        assert!(
            event
                .keys
                .iter()
                .all(|key| key.section == ObservationSection::Inserts),
            "{count}"
        );
    }
}

/// An `OBSERVATION_ENDED` event that cannot fit one control frame can never be sent, so the removal
/// that would end that many subscriptions is refused as a resource limit (not retryable
/// backpressure) before anything changes: the revision, the canonical session, the bindings, the
/// plan epoch and the event lane are as they were. A strip with two subscribed inserts (an end
/// that fits) commits under the same small limit. This is the defensive path: a subscription
/// is admitted against the cap (#1676), so a valid removal never reaches it; the test pushes
/// past the cap on purpose, through the test-only `test_push_observe_unadmitted`, the one way
/// past admission.
///
/// Red if the commit goes ahead and drops the end (an acked removal whose subscribers never hear),
/// if the refusal is backpressure (the host would retry forever), or if the refusal leaves the
/// table emptied, a plan published or an event queued.
#[test]
fn an_end_event_over_one_control_frame_refuses_the_removal_and_changes_nothing() {
    fn put_compressor(strip: &str, id: &str) -> SessionEdit {
        SessionEdit::PutTrackEffect {
            track_id: StableId::parse(strip).expect("strip"),
            rack_name: RackName::Inserts,
            final_position: 1,
            effect: Effect {
                id: StableId::parse(id).expect("insert"),
                identity: EffectIdentity::Native {
                    effect_id: StableId::parse("miso.compressor").expect("compressor"),
                },
                quality: EffectQuality::Normal,
                bypass: false,
                link_mode: LinkMode::DualMono,
                params: Vec::new(),
                sidechain: SidechainDeclaration::None,
            },
        }
    }
    let remove_strip = || {
        vec![
            SessionEdit::RemoveRoute {
                route_id: StableId::parse("comp1-main").expect("route"),
            },
            SessionEdit::RemoveTrack {
                track_id: StableId::parse("comp1").expect("strip"),
            },
        ]
    };
    let mut small = limits();
    small.maximum_control_frame_bytes = 1_024;
    // The strip with `count` subscribed inserts, under the small frame.
    let build = |count: u32| {
        let mut rig = Rig::with_limits(small);
        let ids: Vec<String> = (0..count).map(|index| format!("ins{index:02}")).collect();
        // One insert per request: the small frame holds one put, not sixty.
        for id in &ids {
            rig.apply(&[put_compressor("comp1", id)])
                .expect("the insert is added");
            rig.render();
            rig.render();
            rig.drain();
            let position = rig
                .session
                .effect_producers()
                .iter()
                .position(|producer| &*producer.track_id == "comp1" && &*producer.effect_id == id)
                .expect("the insert has a producer");
            // Past the cap on purpose: the defence below is only reachable without admission.
            rig.session
                .test_push_observe_unadmitted(position, 0, true, WINDOW_BLOCKS)
                .expect("a producer")
                .expect("room in the queue");
        }
        assert_eq!(rig.session.observation_bindings().len(), ids.len());
        rig
    };

    // A few keys fit the frame: the same removal commits under the same limit.
    let mut fits = build(2);
    let before = fits.revision();
    fits.apply(&remove_strip())
        .expect("two ended keys fit one frame");
    assert_eq!(fits.revision(), before + 1);
    assert_eq!(ended(&fits.drain()).len(), 1);

    let mut rig = build(60);
    let revision = rig.revision();
    let canonical = rig.session.test_transaction_snapshot().canonical;
    let bindings = rig.session.observation_bindings().clone();
    let epoch = rig.session.newest_plan_epoch();
    let refused = rig.apply(&remove_strip());
    assert!(
        matches!(
            refused,
            Err(CommandError::CompileRejected(CompileFailure::Resource(
                ResourceFault::Limit
            )))
        ),
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
    assert!(rig.drain().is_empty(), "no event was queued");
}

/// A stable key names one tap on the newest committed plan, whatever position the plan gives its
/// effect, and a key the plan has not is refused with its own reason before anything is written
/// (#1677): the one function the browser's record and the C ABI's observe edit share.
///
/// Red if the key resolves to a position of an older plan (after the removal the producers move up
/// and `comp2` is another position), if a strip without the effect is reported as an unknown strip
/// (or an unknown strip as an unknown effect), or if the rack, the effect ID or the tap ID is not
/// part of the key.
#[test]
fn a_stable_key_resolves_on_the_newest_plan_and_an_unknown_key_is_refused() {
    let resolve = |rig: &Rig, strip: &str, rack: LiveEffectRack, effect: &str, tap: u32| {
        rig.session.resolve_observation_key(&ObservationKeyRef {
            strip_id: strip.as_bytes(),
            rack,
            effect_id: effect.as_bytes(),
            tap_id: tap,
        })
    };
    let mut rig = Rig::new();
    let before = rig.position("comp2");
    assert_eq!(
        resolve(&rig, "comp2", LiveEffectRack::Inserts, "comp", 1),
        Ok(ResolvedObservationTap {
            producer: before,
            tap_index: 0
        })
    );
    for (what, found, expected) in [
        (
            "an unknown strip",
            resolve(&rig, "nowhere", LiveEffectRack::Inserts, "comp", 1),
            ObservationKeyError::UnknownStrip,
        ),
        (
            "an unknown insert",
            resolve(&rig, "comp2", LiveEffectRack::Inserts, "gone", 1),
            ObservationKeyError::UnknownEffect,
        ),
        (
            "the insert's ID in the console rack",
            resolve(&rig, "comp2", LiveEffectRack::Console, "comp", 1),
            ObservationKeyError::UnknownEffect,
        ),
        (
            "an undeclared tap",
            resolve(&rig, "comp2", LiveEffectRack::Inserts, "comp", 99),
            ObservationKeyError::UnknownTap,
        ),
        (
            "the reserved tap zero",
            resolve(&rig, "comp2", LiveEffectRack::Inserts, "comp", 0),
            ObservationKeyError::UnknownTap,
        ),
    ] {
        assert_eq!(found, Err(expected), "{what}");
    }

    // A rebuild is committed and not rendered: the newest plan has no insert on comp1, and comp2's
    // producer has moved up one position. The key follows the effect, not the position.
    rig.apply(&[remove_insert("comp1")]).expect("a rebuild");
    let after = rig.position("comp2");
    assert_ne!(after, before, "the removal moved comp2's producer");
    assert_eq!(
        resolve(&rig, "comp2", LiveEffectRack::Inserts, "comp", 1),
        Ok(ResolvedObservationTap {
            producer: after,
            tap_index: 0
        })
    );
    assert_eq!(
        &*rig.session.effect_producers()[after].track_id,
        "comp2",
        "the position names the producer of the key's strip"
    );
    assert_eq!(
        resolve(&rig, "comp1", LiveEffectRack::Inserts, "comp", 1),
        Err(ObservationKeyError::UnknownEffect),
        "comp1 is still a strip, with no such insert"
    );
    // The block that adopts it, then a rebuild that removes the whole strip.
    rig.render();
    rig.apply(&[
        SessionEdit::RemoveRoute {
            route_id: StableId::parse("comp1-main").expect("route"),
        },
        SessionEdit::RemoveTrack {
            track_id: StableId::parse("comp1").expect("strip"),
        },
    ])
    .expect("a strip removed");
    assert_eq!(
        resolve(&rig, "comp1", LiveEffectRack::Inserts, "comp", 1),
        Err(ObservationKeyError::UnknownStrip)
    );
}

fn put_compressor(strip: &str, id: &str) -> SessionEdit {
    SessionEdit::PutTrackEffect {
        track_id: StableId::parse(strip).expect("strip"),
        rack_name: RackName::Inserts,
        final_position: 1,
        effect: Effect {
            id: StableId::parse(id).expect("insert"),
            identity: EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("compressor"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        },
    }
}

impl Rig {
    /// The position of the producer of `strip`'s insert `id`.
    fn insert_position(&self, strip: &str, id: &str) -> usize {
        self.session
            .effect_producers()
            .iter()
            .position(|producer| &*producer.track_id == strip && &*producer.effect_id == id)
            .expect("the insert has a producer")
    }

    /// Add one compressor insert per ID, one request each, and let the plan adopt each.
    fn add_inserts(&mut self, ids: &[String]) {
        self.add_inserts_to("comp1", ids);
    }

    /// [`Self::add_inserts`] on `strip`.
    fn add_inserts_to(&mut self, strip: &str, ids: &[String]) {
        for id in ids {
            self.apply(&[put_compressor(strip, id)])
                .expect("the insert is added");
            self.render();
            self.render();
            self.drain();
        }
    }

    /// Every observation slot of the newest plan.
    fn slots(&self) -> usize {
        self.session
            .effect_producers()
            .iter()
            .map(host_core::EffectControlProducer::observe_capacity)
            .sum()
    }

    fn last_retained(&self) -> u64 {
        self.session
            .test_transaction_snapshot()
            .resource_rows
            .last()
            .expect("a plan row")
            .1
            .control_retained_bytes
    }
}

/// The cap is the number of keys one control frame carries with every ID at its longest: the event
/// of exactly that many such keys fits the frame, and one more does not.
///
/// Red if the cap is computed from short IDs (a removal of that many long-named taps could not be
/// sent), from the wrong frame limit, or rounds up.
#[test]
fn the_cap_is_what_one_frame_carries_with_the_longest_ids() {
    let id = "a".repeat(session::STABLE_ID_MAXIMUM_BYTES);
    let encoded = |count: usize, frame: usize| {
        let event = protocol::ObservationEnded {
            event_sequence: u64::MAX,
            keys: (0..count)
                .map(|tap| protocol::ObservationTapKey {
                    strip_id: id.clone(),
                    section: ObservationSection::Inserts,
                    effect_id: id.clone(),
                    tap_id: u32::try_from(tap).expect("tap"),
                })
                .collect(),
        };
        let codec = ProtocolCodec::new(protocol::ProtocolLimits {
            max_frame_bytes: frame,
            ..protocol::ProtocolLimits::default()
        });
        codec.encode_event_frame_into(
            &protocol::TypedEventFrame {
                revision: SessionRevision(u64::MAX),
                payload: protocol::EventPayload::ObservationEnded(&event),
            },
            &mut [],
        )
    };
    let mut previous = 0;
    for frame in [1_024_u64, 4_096, 16_384] {
        let mut small = limits();
        small.maximum_control_frame_bytes = frame;
        let cap = Rig::with_limits(small).session.observation_cap();
        assert!(cap > previous, "{frame}: the cap grows with the frame");
        previous = cap;
        let frame = usize::try_from(frame).expect("frame");
        assert!(
            matches!(encoded(cap, frame), Err(protocol::EncodeError::OutputTooSmall { required }) if required <= frame),
            "{frame}: {cap} longest keys fit"
        );
        assert!(
            !matches!(encoded(cap + 1, frame), Err(protocol::EncodeError::OutputTooSmall { required }) if required <= frame),
            "{frame}: one more does not"
        );
    }
}

/// `comp1` renamed to a strip ID of the longest length, so its ended keys are as long as the keys
/// the cap is computed from.
fn long_strip() -> String {
    let id = format!("comp1{}", "x".repeat(session::STABLE_ID_MAXIMUM_BYTES - 5));
    assert_eq!(id.len(), session::STABLE_ID_MAXIMUM_BYTES);
    id
}

/// The fixture with `comp1` named [`long_strip`]; its route keeps the ID `comp1-main`.
fn long_session() -> String {
    let renamed = SESSION.replace("\"comp1\"", &format!("\"{}\"", long_strip()));
    assert_eq!(
        renamed.matches(&long_strip()).count(),
        2,
        "the track and its route source"
    );
    renamed
}

/// An insert ID of the longest length.
fn long_insert(index: usize) -> String {
    let id = format!(
        "ins{index:02}{}",
        "x".repeat(session::STABLE_ID_MAXIMUM_BYTES - 5)
    );
    assert_eq!(id.len(), session::STABLE_ID_MAXIMUM_BYTES);
    id
}

/// The edits that remove [`long_strip`] with its route: every subscribed insert goes with it.
fn remove_long_strip() -> [SessionEdit; 2] {
    [
        SessionEdit::RemoveRoute {
            route_id: StableId::parse("comp1-main").expect("route"),
        },
        SessionEdit::RemoveTrack {
            track_id: StableId::parse(&long_strip()).expect("strip"),
        },
    ]
}

/// A small-frame rig with [`long_strip`] carrying `count` subscribable inserts of the longest IDs,
/// and their producer positions.
fn long_rig(count: usize) -> (Rig, Vec<usize>) {
    let mut small = limits();
    small.maximum_control_frame_bytes = 1_024;
    let mut rig = Rig::with_session(&long_session(), small);
    let strip = long_strip();
    let ids: Vec<String> = (0..count).map(long_insert).collect();
    rig.add_inserts_to(&strip, &ids);
    let positions = ids
        .iter()
        .map(|id| rig.insert_position(&strip, id))
        .collect();
    (rig, positions)
}

/// A subscription past the cap is refused before it is acknowledged and changes nothing; removing
/// every subscribed target afterwards commits, with one event holding every key. Every ID is of the
/// longest length, so the event that ends `cap` taps fills the frame: the cap is exactly what one
/// frame carries, and one tap more (pushed past admission) reaches the over-frame defence.
///
/// Red if the cap is not enforced at admission, or is one too high (the `cap + 1`-th arm is then
/// admitted, and the removal of `cap + 1` longest keys is refused for its event); if a refused batch
/// moves the armed count or queues a record; if an arm of an armed tap counts as a new one; if the
/// refusal names the wrong change of the batch; or if a disarm in the batch is not counted against
/// its arms.
#[test]
fn a_subscription_past_the_cap_is_refused_and_the_removal_of_all_still_commits() {
    let cap = {
        let mut small = limits();
        small.maximum_control_frame_bytes = 1_024;
        Rig::with_limits(small).session.observation_cap()
    };
    assert!(cap >= 2, "the small frame still holds some keys: {cap}");
    let (mut rig, positions) = long_rig(cap + 1);
    // Arm the first `cap` taps.
    for position in &positions[..cap] {
        rig.observe(*position, true).expect("under the cap");
    }
    assert_eq!(rig.session.observation_armed(), cap);
    rig.render();
    // Re-arming an armed tap adds nothing.
    rig.observe(positions[0], true)
        .expect("a window change adds no tap");
    rig.render();
    // One more tap is past the cap, alone or after a change that adds nothing; the refusal names
    // the arm that crossed, and nothing moves or is queued.
    let room = |rig: &Rig| -> Vec<usize> {
        rig.session
            .effect_producers()
            .iter()
            .map(host_core::EffectControlProducer::observe_available_capacity)
            .collect()
    };
    let (bindings, free) = (rig.session.observation_bindings().clone(), room(&rig));
    assert_eq!(
        rig.observe(positions[cap], true),
        Err(ObservationRefusal::Limit(ObservationLimit {
            index: 0,
            cap
        }))
    );
    assert_eq!(
        rig.batch(&[change(positions[1], true), change(positions[cap], true)]),
        Err(ObservationRefusal::Limit(ObservationLimit {
            index: 1,
            cap
        })),
        "the second change crosses: the first re-arms an armed tap"
    );
    assert_eq!(
        rig.session.observation_armed(),
        cap,
        "a refusal moves nothing"
    );
    assert_eq!(room(&rig), free, "a refusal queues nothing");
    rig.render();
    assert_eq!(rig.session.observation_bindings(), &bindings);
    // A disarm later in the batch makes room for an arm earlier in it.
    rig.batch(&[change(positions[cap], true), change(positions[0], false)])
        .expect("a disarm in the batch makes room");
    assert_eq!(rig.session.observation_armed(), cap);
    rig.render();
    // Remove every subscribed target: `cap` keys of the longest IDs, in one event in one frame.
    let before = rig.revision();
    rig.apply(&remove_long_strip())
        .expect("the removal of every subscribed tap commits");
    assert_eq!(rig.revision(), before + 1);
    assert_eq!(rig.session.observation_armed(), 0);
    let events = rig.drain();
    let ended = ended(&events);
    assert_eq!(ended.len(), 1, "one event");
    assert_eq!(ended[0].1.keys.len(), cap, "holding every key");
    assert!(
        ended[0]
            .1
            .keys
            .iter()
            .all(|key| key.strip_id.len() == session::STABLE_ID_MAXIMUM_BYTES
                && key.effect_id.len() == session::STABLE_ID_MAXIMUM_BYTES),
        "every key is of the longest length"
    );

    // One tap more than the cap, pushed past admission: its end does not fit one frame, so the
    // same removal reaches the over-frame defence and is refused whole.
    let (mut over, positions) = long_rig(cap + 1);
    for position in &positions {
        over.session
            .test_push_observe_unadmitted(*position, 0, true, WINDOW_BLOCKS)
            .expect("a producer")
            .expect("room in the queue");
    }
    assert_eq!(over.session.observation_armed(), cap + 1);
    over.render();
    let revision = over.revision();
    let refused = over.apply(&remove_long_strip());
    assert!(
        matches!(
            refused,
            Err(CommandError::CompileRejected(CompileFailure::Resource(
                ResourceFault::Limit
            )))
        ),
        "cap + 1 longest keys do not fit one frame: {refused:?}"
    );
    assert_eq!(over.revision(), revision);
}

/// A rebuild keeps the armed count of the taps it carries, so the cap holds across plans.
///
/// Red if a successor starts with an empty count (the cap would then admit twice the taps) or
/// counts a tap whose target the transaction removed.
#[test]
fn the_armed_count_follows_the_bindings_across_a_rebuild() {
    let mut rig = Rig::new();
    let ids: Vec<String> = (0..3).map(|index| format!("ins{index:02}")).collect();
    rig.add_inserts(&ids);
    for id in &ids {
        let position = rig.insert_position("comp1", id);
        rig.observe(position, true).expect("under the cap");
    }
    assert_eq!(rig.session.observation_armed(), 3);
    // Remove one subscribed insert: the other two stay armed in the successor.
    rig.apply(&[SessionEdit::RemoveTrackEffect {
        track_id: StableId::parse("comp1").expect("strip"),
        rack_name: RackName::Inserts,
        effect_id: StableId::parse("ins00").expect("insert"),
    }])
    .expect("the insert is removed");
    assert_eq!(rig.session.observation_armed(), 2);
    assert_eq!(rig.session.observation_bindings().len(), 2);
    // The successor knows them armed: disarming one lowers the count, a second disarm does not.
    let position = rig.insert_position("comp1", "ins01");
    for expected in [1, 1] {
        rig.observe(position, false).expect("a disarm is admitted");
        rig.render();
        assert_eq!(rig.session.observation_armed(), expected);
    }
}

/// The memory reserved for the ended-event cells follows the observation slots the session has had,
/// not the frame: the same plan reserves the same at a 4 KiB and a 16 KiB frame, and a plan that
/// shrank keeps the reserve of the larger plan whose event may still wait in a cell.
///
/// Red if the reserve scales with the frame size again, or if a shrunk plan charges less than the
/// event of its larger predecessor can hold.
#[test]
fn the_reserve_follows_the_slots_and_a_shrunk_plan_keeps_the_larger_reserve() {
    let reserve = |frame: u64, slots: usize| {
        control_plane::observation_reserve_bytes(frame, slots).expect("reserve")
    };
    assert_eq!(
        reserve(4_096, 4),
        reserve(16_384, 4),
        "a few slots do not scale with the frame"
    );
    assert!(
        reserve(16_384, 100_000).0 > reserve(4_096, 100_000).0,
        "many slots are capped by the frame"
    );

    let mut grown = Rig::new();
    let baseline = Rig::new();
    let (small_slots, small_retained) = (baseline.slots(), baseline.last_retained());
    let ids: Vec<String> = (0..6).map(|index| format!("ins{index:02}")).collect();
    grown.add_inserts(&ids);
    let big_slots = grown.slots();
    assert!(big_slots > small_slots);
    let removals: Vec<SessionEdit> = ids
        .iter()
        .map(|id| SessionEdit::RemoveTrackEffect {
            track_id: StableId::parse("comp1").expect("strip"),
            rack_name: RackName::Inserts,
            effect_id: StableId::parse(id).expect("insert"),
        })
        .collect();
    grown.apply(&removals).expect("the inserts are removed");
    assert_eq!(
        grown.slots(),
        small_slots,
        "the plan shrank to the boot plan"
    );
    let frame = limits().maximum_control_frame_bytes;
    let extra = reserve(frame, big_slots).0 - reserve(frame, small_slots).0;
    assert!(extra > 0);
    // Other rows carry a little history (the shrunk plan's graph differs from the boot plan's by
    // a few bytes), so the charge is at least the larger reserve, not exactly it.
    assert!(
        grown.last_retained() - small_retained >= extra,
        "the shrunk plan charges the larger plan's reserve"
    );
}
