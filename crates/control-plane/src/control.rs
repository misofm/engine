//! The control-protocol session: commands, events, source control and plan replacement.

use super::*;

/// One plan's live fader/mute, matrix/pan, input and send producers (#1256 D2, #1557 D4), kept with
/// its provider epoch.
///
/// `controls` is in `HostLiveControlHandles::strips` order: the tracks, then the submixes. A
/// producer's `input` is `Some` when the plan carries the input lane (#1556). `routes` is in
/// canonical route-ID order, one producer per route into a submix when the plan carries the route
/// lane (#1557). `commit_live` pushes a value-only transaction's records through them (#1257). The rings are `Arc`s shared with the
/// plan's consumers, so they outlive whichever of the plan and the epoch drops first, and the last
/// owner to drop frees them, on the control thread: `synchronize_plan_epochs` drops a reclaimed
/// plan before its provider.
pub(crate) struct StripLanes {
    pub(crate) controls: Box<[host_core::TrackControlProducer]>,
    /// How many leading entries of `controls` are tracks.
    pub(crate) track_count: usize,
    /// One producer per route into a submix, in canonical route-ID order; empty when the plan
    /// carries no route lane.
    pub(crate) routes: Box<[host_core::RouteControlProducer]>,
    /// The writer of the plan's jump-length cell (#1639 D1), held inline; every prepared plan has
    /// one, and preparation refuses a handle set without it.
    pub(crate) jump_lengths: host_core::GraphJumpLengthProducer,
}

/// One epoch's worth of host-owned source producers, strip live-control producers and effect
/// live-control producers.
///
/// The tables themselves live in `host-core`; this wrapper adds the epoch tag, the plan's state
/// inventory its successor is prepared from (issue #1273 D1), the epoch's own capi resource rows
/// (`capi`, which the live admission reads, #1257 D4) and the lifecycle counters the
/// structural-replacement tests observe.
///
/// Public only to the adapters' tests, under `test-support`; crate-private otherwise.
#[cfg_attr(not(feature = "test-support"), allow(unreachable_pub))]
pub struct ProviderEpoch {
    pub(crate) epoch: u64,
    pub(crate) sources: SourceControlSet,
    /// What this epoch's plan holds, for preparing its successor (issue #1273 D1).
    pub(crate) inventory: PlanStateInventory,
    /// The live fader and matrix producers of this epoch's plan (#1256 D2).
    pub(crate) strips: StripLanes,
    /// One live-control producer per prepared effect instance of this epoch's plan, console slots
    /// and inserts alike, in `HostLiveControlHandles::effect_controls` order (#1263 D2). A
    /// parametric EQ's producer also owns its prepared-target owner. `commit_live` writes a
    /// value-only transaction's effect values to their cells through them (#1264). As for the strip
    /// producers, the cells are `Arc`s shared with the plan's lanes, so the epoch may drop after
    /// its plan, and does: `synchronize_plan_epochs` drops a reclaimed plan before its provider.
    pub(crate) effects: Box<[host_core::EffectControlProducer]>,
    /// The capi resources this epoch's plan was prepared with (#1257 D4).
    pub(crate) capi: CapiResources,
    /// The handle of this epoch's plan's revision gate (#1502 D1). Control publishes every
    /// revision the plan carries on it, and #1312 stamps the plan's cell writes from it, so the
    /// gate, the cells and the producers that write them follow one routing. It stays here for
    /// the epoch's life and moves only with the epoch, which holds no open stamp between
    /// commands.
    pub(crate) revision_gate: PlanRevisionGate,
}

impl ProviderEpoch {
    /// This epoch's tag: the plan epoch its producers feed.
    #[cfg(feature = "test-support")]
    pub fn test_epoch(&self) -> u64 {
        self.epoch
    }

    /// This epoch's source producers.
    #[cfg(feature = "test-support")]
    pub fn test_sources(&self) -> &SourceControlSet {
        &self.sources
    }

    /// How many leading strip producers are tracks.
    #[cfg(feature = "test-support")]
    pub fn test_track_count(&self) -> usize {
        self.strips.track_count
    }

    /// This epoch's strip producers: the tracks, then the submixes.
    #[cfg(feature = "test-support")]
    pub fn test_strip_controls(&self) -> &[host_core::TrackControlProducer] {
        &self.strips.controls
    }

    /// This epoch's effect producers.
    #[cfg(feature = "test-support")]
    pub fn test_effects(&self) -> &[host_core::EffectControlProducer] {
        &self.effects
    }

    pub(crate) fn current(
        sources: SourceControlSet,
        inventory: PlanStateInventory,
        strips: StripLanes,
        effects: Box<[host_core::EffectControlProducer]>,
        capi: CapiResources,
        revision_gate: PlanRevisionGate,
    ) -> Self {
        let owner = Self {
            epoch: 0,
            sources,
            inventory,
            strips,
            effects,
            capi,
            revision_gate,
        };
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.current_provider_constructed += 1);
        owner
    }

    pub(crate) fn candidate(
        sources: SourceControlSet,
        inventory: PlanStateInventory,
        strips: StripLanes,
        effects: Box<[host_core::EffectControlProducer]>,
        capi: CapiResources,
        revision_gate: PlanRevisionGate,
    ) -> Self {
        let owner = Self {
            epoch: u64::MAX,
            sources,
            inventory,
            strips,
            effects,
            capi,
            revision_gate,
        };
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.candidate_provider_constructed += 1);
        owner
    }
}

impl Drop for ProviderEpoch {
    fn drop(&mut self) {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            if self.epoch == 0 {
                owners.current_provider_disposed += 1;
            } else {
                owners.candidate_provider_disposed += 1;
            }
        });
    }
}

pub(crate) const RENDER_DIAGNOSTIC_SLOTS: usize = 2;
/// The render-activity diagnostic's code (#1309 D7). It keeps the C ABI's text because `host-core`
/// sizes the provider's render diagnostics by this literal and it is protocol-visible; choosing the
/// browser's code is #1332's.
pub(crate) const RENDER_DIAGNOSTIC_CODE: &str = "capi.render.activity";

/// A structural command's fault-injection point, in the order `command` reaches them.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestStructuralFaultPhase {
    /// After the protocol prepared the token.
    AfterProtocolPrepare,
    /// Before the candidate runtime is prepared.
    BeforeRuntimePrepare,
    /// After the candidate runtime is prepared.
    AfterRuntimePrepare,
    /// After the replacement admission.
    AfterAdmission,
    /// After the plan replacement is reserved.
    AfterPlanReservation,
    /// Before the protocol commit.
    BeforeProtocolCommit,
    /// The protocol commit itself returns `Err` (issue #1273 gate 3).
    ProtocolCommit,
    /// The live arm, after every fallible check and before its first write (#1257 D7).
    BeforeLivePush,
}

/// How many of each owner the structural and live paths constructed, published and disposed.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TestOwnerCounters {
    /// Lifecycle count: current provider constructed.
    pub current_provider_constructed: u64,
    /// Lifecycle count: current provider disposed.
    pub current_provider_disposed: u64,
    /// Lifecycle count: candidate provider constructed.
    pub candidate_provider_constructed: u64,
    /// Lifecycle count: candidate provider disposed.
    pub candidate_provider_disposed: u64,
    /// Lifecycle count: candidate provider published.
    pub candidate_provider_published: u64,
    /// Lifecycle count: current plan constructed.
    pub current_plan_constructed: u64,
    /// Lifecycle count: current plan disposed.
    pub current_plan_disposed: u64,
    /// Lifecycle count: candidate plan constructed.
    pub candidate_plan_constructed: u64,
    /// Lifecycle count: candidate plan disposed.
    pub candidate_plan_disposed: u64,
    /// Lifecycle count: candidate plan published.
    pub candidate_plan_published: u64,
    /// Lifecycle count: token constructed.
    pub token_constructed: u64,
    /// Lifecycle count: token disposed.
    pub token_disposed: u64,
    /// Lifecycle count: replay current constructed.
    pub replay_current_constructed: u64,
    /// Lifecycle count: replay current disposed.
    pub replay_current_disposed: u64,
    /// Lifecycle count: replay candidate constructed.
    pub replay_candidate_constructed: u64,
    /// Lifecycle count: replay candidate disposed.
    pub replay_candidate_disposed: u64,
    /// Lifecycle count: replay candidate published.
    pub replay_candidate_published: u64,
    /// Lifecycle count: reservation constructed.
    pub reservation_constructed: u64,
    /// Lifecycle count: reservation canceled.
    pub reservation_canceled: u64,
    /// Lifecycle count: reservation committed.
    pub reservation_committed: u64,
}

/// One effect producer's row of `SessionControlSnapshot::effect_cells`: `(epoch, strip ID,
/// effect instance ID, writes per cell)`.
#[cfg(feature = "test-support")]
pub type EffectCellRow = (u64, Box<str>, Box<str>, Vec<u64>);

/// Everything a transaction may change, so a test can prove a refusal changed none of it.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestTransactionSnapshot<Row> {
    /// The committed session revision.
    pub revision: u64,
    /// The committed canonical JSON.
    pub canonical: Vec<u8>,
    /// The committed compiled model's resource estimate.
    pub model: session::ResourceEstimate,
    /// Replay cache entries.
    pub replay_entries: usize,
    /// The current provider epoch.
    pub provider_epoch: u64,
    /// Each pending provider's epoch.
    pub pending_provider_epochs: Vec<u64>,
    /// Each retired provider's epoch.
    pub retired_provider_epochs: Vec<u64>,
    /// The epoch the render thread last published.
    pub active_plan_epoch: u64,
    /// The plan resource table: each epoch's adapter row.
    pub resource_rows: Vec<(u64, Row)>,
    /// The reliable event queue.
    pub reliable_event: protocol::QueueReport,
    /// The reliable response queue.
    pub reliable_response: protocol::QueueReport,
    /// The telemetry queue.
    pub telemetry: protocol::QueueReport,
    /// The telemetry counters.
    pub telemetry_counters: protocol::TelemetryCounters,
    /// The controller's, provider's and replay cache's retained capacities.
    pub retained_capacities: [usize; 7],
    /// How many writes every strip producer's cells have taken, current epoch first, then each
    /// pending candidate: `(epoch, strip ID, fader left, fader right, mute left, mute right,
    /// matrix)`. A cell's count is its write sequence (#1312 D3).
    pub strip_cells: Vec<(u64, Box<str>, [u64; 4], u64)>,
    /// How many writes every send producer's cell has taken, current epoch first, then each
    /// pending candidate: `(epoch, route ID, writes)` (#1557).
    pub route_cells: Vec<(u64, Box<str>, u64)>,
    /// How many writes every effect producer's bypass cell, parameter cells, then target cells,
    /// have taken, current epoch first, then each pending candidate: `(epoch, strip ID, effect
    /// instance ID, writes per cell)`. A cell's count is its write sequence (#1345 D1).
    pub effect_cells: Vec<EffectCellRow>,
    /// Every EQ owner's state, current epoch first, then each pending candidate: `(epoch, strip
    /// ID, effect instance ID, committed revision, phase, committed value bits, candidate value
    /// bits)`. The candidate is empty while the owner is idle: it exists only between `begin` and
    /// `commit` or `discard` (#1624).
    pub effect_owners: Vec<TestOwnerState>,
}

/// One EQ owner's observable state in a [`TestTransactionSnapshot`].
#[cfg(feature = "test-support")]
pub type TestOwnerState = (u64, Box<str>, Box<str>, u64, String, Vec<u32>, Vec<u32>);

#[cfg(feature = "test-support")]
thread_local! {
    static TEST_FAULT_STATE: core::cell::Cell<([Option<TestStructuralFaultPhase>; 2], usize)> =
        const { core::cell::Cell::new(([None; 2], 0)) };
    static TEST_NEXT_ADOPTION: core::cell::Cell<Option<PlanAdoption>> =
        const { core::cell::Cell::new(None) };
    static FORCE_FULL_PREPARATION: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// `(commits made from an in-place token, in-place tokens materialized)` on this thread.
    static IN_PLACE_COUNTS: core::cell::Cell<(usize, usize)> = const { core::cell::Cell::new((0, 0)) };
    static TEST_OWNER_STATE: core::cell::Cell<TestOwnerCounters> =
        const { core::cell::Cell::new(TestOwnerCounters {
            current_provider_constructed: 0,
            current_provider_disposed: 0,
            candidate_provider_constructed: 0,
            candidate_provider_disposed: 0,
            candidate_provider_published: 0,
            current_plan_constructed: 0,
            current_plan_disposed: 0,
            candidate_plan_constructed: 0,
            candidate_plan_disposed: 0,
            candidate_plan_published: 0,
            token_constructed: 0,
            token_disposed: 0,
            replay_current_constructed: 0,
            replay_current_disposed: 0,
            replay_candidate_constructed: 0,
            replay_candidate_disposed: 0,
            replay_candidate_published: 0,
            reservation_constructed: 0,
            reservation_canceled: 0,
            reservation_committed: 0,
        }) };
}

#[cfg(feature = "test-support")]
pub(crate) fn update_test_owners(update: impl FnOnce(&mut TestOwnerCounters)) {
    TEST_OWNER_STATE.with(|state| {
        let mut value = state.get();
        update(&mut value);
        state.set(value);
    });
}

#[cfg(feature = "test-support")]
pub(crate) fn take_test_fault_state(phase: TestStructuralFaultPhase) -> bool {
    TEST_FAULT_STATE.with(|state| {
        let (faults, index) = state.get();
        let matched = faults.get(index).copied().flatten() == Some(phase);
        if matched {
            state.set((faults, index + 1));
        }
        matched
    })
}

/// Clears the thread's fault plan and lifecycle counters.
#[cfg(feature = "test-support")]
pub fn test_reset_lifecycle_observer() {
    TEST_FAULT_STATE.with(|state| state.set(([None; 2], 0)));
    TEST_OWNER_STATE.with(|state| state.set(TestOwnerCounters::default()));
    TEST_NEXT_ADOPTION.with(|state| state.set(None));
}

/// Makes this thread's control commands prepare their transactions by the full path (#1613): a
/// clone of the model with the edits applied, compiled, as before the in-place preparation. A test
/// runs one script both ways and compares the two sessions.
#[cfg(feature = "test-support")]
pub fn test_force_full_preparation(force: bool) {
    FORCE_FULL_PREPARATION.with(|state| state.set(force));
}

/// Whether a live commit on this thread checks, in a debug build, that its catalog stage equals a
/// full build of the committed model (on by default; [`host_core::verify_stages`]). The check
/// builds the whole catalog, so a test that counts the allocations of a command turns it off.
#[cfg(feature = "test-support")]
pub fn test_verify_catalog_stages(on: bool) {
    host_core::verify_stages(on);
}

/// How many commits this thread made from a token prepared in place, and how many such tokens it
/// had to materialize into the full preparation, since the last call (#1613), and resets both.
#[cfg(feature = "test-support")]
pub fn test_take_in_place_counts() -> (usize, usize) {
    IN_PLACE_COUNTS.with(core::cell::Cell::take)
}

/// The thread's lifecycle counters.
#[cfg(feature = "test-support")]
pub fn test_lifecycle_counters() -> TestOwnerCounters {
    TEST_OWNER_STATE.with(core::cell::Cell::get)
}

/// Every plan the control plane holds unreclaimed, the running plan first (#1398 D4), without an
/// allocation.
pub(crate) struct HeldEpochs {
    rows: [LiveEpochResources; 1 + PENDING_PROVIDERS],
    len: usize,
}

impl HeldEpochs {
    pub(crate) fn as_slice(&self) -> &[LiveEpochResources] {
        &self.rows[..self.len]
    }
}

/// One session's control plane: the control-protocol controller, the provider epochs, the plan
/// exchange's control ends and the shared plan projection, generic over the adapter (#1309 D4).
pub struct SessionState<A: ControlAdapter> {
    pub(crate) controller: ObservedController,
    pub(crate) providers: ProviderEpoch,
    pub(crate) pending_providers: Vec<ProviderEpoch>,
    pub(crate) retired_providers: Vec<ProviderEpoch>,
    pub(crate) publisher: PlanPublisher,
    pub(crate) retirer: PlanRetirer,
    pub(crate) limits: ControlLimits,
    /// The adapter's preparation (issue #1400 D2): the boot plan and every structural successor
    /// are prepared through it.
    pub(crate) preparer: Box<dyn RuntimePreparer>,
    /// The session's `live_values_superseded` count (#1312 D2): one for every plan the session
    /// prepares. Render adds to it; the control plane copies it into the provider's counters in
    /// the service step (D8).
    pub(crate) live_values_superseded: host_core::LiveValuesSuperseded,
    /// `u16` scratch the protocol decoder writes field offsets into. It is exactly
    /// `max_frame_bytes / 2` entries; the odd trailing byte of an odd frame limit used to buy a
    /// one-byte box that nothing ever read (audit F7).
    pub(crate) decode_fields: Box<[u16]>,
    pub(crate) response_scratch: Box<[u8]>,
    pub(crate) shared: Arc<SharedPlanState<A::Row>>,
    pub(crate) observed_render_sequence: u64,
    /// The render sequence at which the counter snapshot was last staged, or while no counter is
    /// configured the latest one the service step saw: the baseline of the configured period
    /// (#1351 D4).
    pub(crate) counter_snapshot_sequence: u64,
    pub(crate) render_diagnostics: Box<[RenderDiagnosticSlot]>,
    pub(crate) render_diagnostic_head: usize,
    pub(crate) render_diagnostic_len: usize,
    pub(crate) protocol_reliable_pending: bool,
    /// The monitor overlay (decision 17, C8, #1559 D1): the soloed strips. Host-local, never saved
    /// and never in a snapshot; empty at boot. The newest plan's cells hold it (D10), the control
    /// catalog shows it (D11), and it is written only after every fallible step of the
    /// transaction that sets it has passed (D8).
    pub(crate) monitor: controls::MonitorOverlay,
    /// The observation taps the host has armed (#1584 D6a), kept beside the monitor overlay and
    /// apart from it: solo and subscriptions are different things. Host-local, never saved and
    /// never in a snapshot; empty at boot. Written after a successful push to a tap's queue, and
    /// by a rebuild with the commit, which drops each binding whose target the transaction removed
    /// and tells the subscribers. Every successor is prepared from it, so a subscription the host
    /// acknowledged survives a rebuild that restarts its effect.
    pub(crate) observation: controls::ObservationBindings,
    /// The subscription state each observation slot of the newest plan was last pushed, waiting
    /// to be folded into [`Self::observation`] (#1584 D6a). The browser admits a subscription in
    /// the worklet's message handler between render quanta, on the audio thread, where nothing may
    /// allocate, and a binding owns its key's text, so the
    /// push records the absolute state here, in storage sized when the plan is published, and the
    /// next service step folds it into the table. A rebuild folds first: it only starts from
    /// [`Self::command`], which services.
    pub(crate) observation_pending: Box<[ObservationSlot]>,
    /// For each position of the newest plan's effect producers, the first of its slots in
    /// `observation_pending`, and one entry past the last: a producer's slots are
    /// `offsets[position]..offsets[position + 1]`, one per declared tap in declaration order.
    pub(crate) observation_offsets: Box<[u32]>,
    /// How many of [`Self::observation_pending`]'s slots are armed now, pushes not yet folded
    /// included.
    pub(crate) observation_armed: usize,
    /// The encoded bytes of the armed slots' keys in an `OBSERVATION_ENDED` event, pushes not yet
    /// folded included: the figure a subscription is admitted against (#1685 D1). With the frame's
    /// own bytes, never above the control frame limit.
    pub(crate) observation_armed_bytes: usize,
    /// The encoded size of an `OBSERVATION_ENDED` frame as a function of its keys, and the control
    /// frame limit it must fit, so the commit that ends every binding can always say so (#1685
    /// D1). Fixed by the session's frame limit when it compiles.
    pub(crate) observation_wire: crate::compile::ObservationWire,
    /// For each position of the newest plan's effect producers, the records of the batch being
    /// admitted that name it (#1676, batch 13b fold): the room check reads one count per producer
    /// instead of counting the batch again for each record. Sized with
    /// [`Self::observation_offsets`], when the plan's slots are built, so admission allocates
    /// nothing.
    pub(crate) observation_batch_counts: Box<[ObservationBatchCount]>,
    /// The stamp of the batch being admitted: a slot's or a producer count's `mark` equal to it
    /// belongs to this batch, any other value to an earlier one, so the scratch needs no clearing
    /// between batches.
    pub(crate) observation_batch_mark: u32,
    /// The most observation slots any committed plan has had; the ended-event cells and the
    /// binding table are charged for it (#1676 D2).
    pub(crate) observation_slots_high_water: usize,
}

/// The stable key of one observation tap, borrowed from the bytes that name it (#1677): the strip's
/// ID, the rack, the console slot's or insert's ID and the effect-local tap ID. The same key
/// `OBSERVATION_ENDED` names ([`controls::TapKey`]), in a form a host can build from its staging
/// without allocating.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationKeyRef<'a> {
    /// The ID of the track or submix.
    pub strip_id: &'a [u8],
    /// The console slot or the insert section.
    pub rack: host_core::LiveEffectRack,
    /// The console slot's or the insert's ID.
    pub effect_id: &'a [u8],
    /// The effect-local tap ID.
    pub tap_id: u32,
}

/// Where [`SessionState::resolve_observation_key`] found a tap, on the newest plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedObservationTap {
    /// The position of the tap's effect in [`SessionState::effect_producers`].
    pub producer: usize,
    /// The tap's index in its effect's declared menu, which the render side arms.
    pub tap_index: u32,
}

/// Why a key names no tap of the newest plan (#1677). Nothing was written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationKeyError {
    /// The strip ID is no track or submix of the plan.
    UnknownStrip,
    /// The strip has no console slot or insert of that ID.
    UnknownEffect,
    /// The effect declares no tap of that ID.
    UnknownTap,
    /// The tap is declared and its cost is not resident; V1 binds no lane for one.
    ComputedTap,
}

/// A subscription batch would leave armed taps whose keys one `OBSERVATION_ENDED` event cannot
/// carry in one control frame (#1676 D1, #1685 D1): the frame is full. Nothing was written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationLimit {
    /// The position, in the batch, of the first change whose arm makes the event outgrow the
    /// frame.
    pub index: usize,
    /// The encoded bytes the event would need with that arm, every removal of the batch counted.
    pub event_bytes: usize,
    /// The control frame limit.
    pub frame_bytes: usize,
}

/// One subscription change of a batch handed to
/// [`SessionState::admit_and_push_observation_batch`] (#1676).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationChange {
    /// The producer's position in [`SessionState::effect_producers`].
    pub position: usize,
    /// The tap's index in the producer's observation menu.
    pub tap_index: u32,
    /// `true` arms the tap, `false` disarms it.
    pub armed: bool,
    /// Render blocks per published window; ignored by a disarm.
    pub window_blocks: u32,
}

/// Why [`SessionState::admit_and_push_observation_batch`] refused a batch. Nothing was queued.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationRefusal {
    /// The batch would leave armed taps whose ended event outgrows one control frame. Not
    /// retryable as sent: a tap must be disarmed first.
    Limit(ObservationLimit),
    /// The change at `index` needs more room than its producer's observation queue has free.
    /// Retryable once a render drains the queue.
    Full {
        /// The change's position in the batch.
        index: usize,
    },
    /// The change at `index` names a position past [`SessionState::effect_producers`].
    NoProducer {
        /// The change's position in the batch.
        index: usize,
    },
    /// The change at `index` names a tap an earlier change of the same batch already names. A
    /// change sets its tap's whole state and the batch drains in one block, so a second change
    /// would supersede the first, which an observation record never is (D15-2). Not retryable as
    /// sent.
    Duplicate {
        /// The position in the batch of the second change that names the tap.
        index: usize,
    },
    /// The change at `index` names a producer whose observation queue has no slot for its tap:
    /// the tap index is past the producer's observation menu, or the plan bound the producer no
    /// observation queue at all (capacity `0`). Such a record would be acknowledged and never
    /// applied, so it is refused before anything is queued. Not retryable as sent.
    Unbound {
        /// The change's position in the batch.
        index: usize,
    },
}

/// The index in `descriptor`'s observation menu of the tap with ID `tap_id`, the one the render side
/// arms (#1677). A tap the menu does not declare is [`ObservationKeyError::UnknownTap`], and a
/// declared tap that is not resident is [`ObservationKeyError::ComputedTap`]: V1 ships no
/// implementation for one, and saying so is better than binding a lane that would never publish.
///
/// # Errors
///
/// As above.
pub fn resolve_observation_tap(
    descriptor: &effect_contract::EffectDescriptor,
    tap_id: u32,
) -> Result<u32, ObservationKeyError> {
    let (index, tap) = descriptor
        .observations
        .iter()
        .enumerate()
        .find(|(_, tap)| tap.id.0 == tap_id)
        .ok_or(ObservationKeyError::UnknownTap)?;
    if !matches!(tap.cost, effect_contract::ObservationCost::Resident) {
        return Err(ObservationKeyError::ComputedTap);
    }
    u32::try_from(index).map_err(|_| ObservationKeyError::UnknownTap)
}

/// A plan's observation slot offsets, its slot storage, its per-producer admission scratch and how
/// many of its slots are armed.
pub(crate) type ObservationSlots = (
    Box<[u32]>,
    Box<[ObservationSlot]>,
    Box<[ObservationBatchCount]>,
    usize,
    usize,
);

/// The slot storage for `producers`, the newest plan's: the first slot of each producer (and one
/// past the last), and one slot per declared tap (#1584 D6a), each armed when `bindings` holds its
/// tap (#1676), and each slot carries its key's encoded bytes in an `OBSERVATION_ENDED` event
/// (#1685 D2), from the producer's two IDs. The last two figures are the armed slots and their keys'
/// bytes. Built when a plan is prepared, fallibly and before anything commits; the session takes it
/// after the commit and after the fold, so nothing pending is lost to a change of positions.
pub(crate) fn observation_slots(
    producers: &[host_core::EffectControlProducer],
    bindings: &controls::ObservationBindings,
    wire: crate::compile::ObservationWire,
) -> Result<ObservationSlots, std::collections::TryReserveError> {
    let mut offsets = Vec::new();
    offsets.try_reserve_exact(producers.len() + 1)?;
    let mut total = 0_u32;
    for producer in producers {
        offsets.push(total);
        total =
            total.saturating_add(u32::try_from(producer.observe_capacity()).unwrap_or(u32::MAX));
    }
    offsets.push(total);
    let mut counts = Vec::new();
    counts.try_reserve_exact(producers.len())?;
    counts.resize(producers.len(), ObservationBatchCount::default());
    let mut slots = Vec::new();
    slots.try_reserve_exact(total as usize)?;
    let mut armed = 0_usize;
    let mut armed_bytes = 0_usize;
    for producer in producers {
        // Every ID is at most `STABLE_ID_MAXIMUM_BYTES`, so a key is a few hundred bytes.
        let key_bytes = wire.key_bytes(producer.track_id.len(), producer.effect_id.len());
        let key_bytes = u16::try_from(key_bytes).unwrap_or(u16::MAX);
        for tap in 0..producer.observe_capacity() {
            let is_armed = !bindings.is_empty()
                && producer
                    .descriptor
                    .observations
                    .get(tap)
                    .is_some_and(|declared| {
                        bindings.is_armed_by_parts(
                            &producer.track_id,
                            producer.address.rack == host_core::LiveEffectRack::Console,
                            &producer.effect_id,
                            declared.id.0,
                        )
                    });
            armed += usize::from(is_armed);
            if is_armed {
                armed_bytes += usize::from(key_bytes);
            }
            slots.push(ObservationSlot {
                armed: is_armed,
                pending: PendingArm::None,
                mark: 0,
                key_bytes,
            });
        }
    }
    Ok((
        offsets.into_boxed_slice(),
        slots.into_boxed_slice(),
        counts.into_boxed_slice(),
        armed,
        armed_bytes,
    ))
}

/// One producer's share of the batch being admitted (#1676, batch 13b fold): how many of its
/// records name the producer, valid while `mark` is the session's batch stamp.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObservationBatchCount {
    /// The batch stamp `needed` was counted under.
    pub(crate) mark: u32,
    /// Records of that batch that name the producer.
    pub(crate) needed: u32,
}

/// One observation slot: whether its tap is armed now, and what the host pushed to it since the
/// last fold (#1584 D6a, #1676).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ObservationSlot {
    /// Armed now: the table's state with every push since folded in. Lets a subscription be
    /// admitted against the frame without a key lookup, on the audio thread between quanta.
    pub(crate) armed: bool,
    /// The push waiting to be folded into the table.
    pub(crate) pending: PendingArm,
    /// The stamp of the last batch that named this slot (#1676, batch 13b fold): a change whose
    /// slot already carries the current stamp is the batch's second naming of the tap.
    pub(crate) mark: u32,
    /// The encoded bytes of the tap's key in an `OBSERVATION_ENDED` event (#1685 D2): what arming
    /// it adds to the event that would end every armed tap. Set when the plan's slots are built.
    pub(crate) key_bytes: u16,
}

/// What the host last pushed to one observation slot (#1584 D6a).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PendingArm {
    /// Nothing since the last fold.
    #[default]
    None,
    /// Armed, with this window length in blocks (`0` is the plan's meter window).
    Arm(u32),
    /// Disarmed.
    Disarm,
}

/// The stable key of the tap `tap_index` of the effect `producer` addresses, or `None` for an index
/// the effect does not declare.
fn tap_key_of(
    producer: &host_core::EffectControlProducer,
    tap_index: u32,
) -> Option<controls::TapKey> {
    let tap = producer.descriptor.observations.get(tap_index as usize)?;
    let effect = session::StableId::parse(&producer.effect_id)?;
    let section = match producer.address.rack {
        host_core::LiveEffectRack::Console => controls::TapSection::Console(effect),
        host_core::LiveEffectRack::Inserts => controls::TapSection::Inserts(effect),
    };
    Some(controls::TapKey {
        strip: session::StableId::parse(&producer.track_id)?,
        section,
        tap_id: tap.id.0,
    })
}

/// The wire key of an ended binding.
fn ended_key(key: &controls::TapKey) -> protocol::ObservationTapKey {
    protocol::ObservationTapKey {
        strip_id: key.strip.as_str().to_owned(),
        section: if key.section.is_console() {
            protocol::ObservationSection::Console
        } else {
            protocol::ObservationSection::Inserts
        },
        effect_id: key.section.id().as_str().to_owned(),
        tap_id: key.tap_id,
    }
}

pub(crate) struct ObservedController {
    pub(crate) inner: ProtocolController<SessionControlProvider>,
}

impl ObservedController {
    pub(crate) fn new(inner: ProtocolController<SessionControlProvider>) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.replay_current_constructed += 1);
        Self { inner }
    }
}

impl core::ops::Deref for ObservedController {
    type Target = ProtocolController<SessionControlProvider>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ObservedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for ObservedController {
    fn drop(&mut self) {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.replay_current_disposed += 1);
    }
}

pub(crate) struct RenderDiagnosticSlot {
    pub(crate) diagnostic: protocol::Diagnostic,
    pub(crate) reservation: Option<protocol::ReliableEventReservation>,
    pub(crate) protocol_events_before: u64,
    pub(crate) revision: protocol::SessionRevision,
    pub(crate) occupied: bool,
}

impl RenderDiagnosticSlot {
    pub(crate) fn try_new() -> Result<Self, CompileFailure> {
        let mut code = String::new();
        code.try_reserve_exact(RENDER_DIAGNOSTIC_CODE.len())
            .map_err(|_| failure(ResourceFault::Allocation))?;
        code.push_str(RENDER_DIAGNOSTIC_CODE);
        Ok(Self {
            diagnostic: protocol::Diagnostic {
                code,
                severity: protocol::DiagnosticSeverity::Info,
                path: Vec::new(),
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            },
            reservation: None,
            protocol_events_before: 0,
            revision: protocol::SessionRevision(0),
            occupied: false,
        })
    }
}

pub(crate) fn prepare_render_diagnostic_slots()
-> Result<Box<[RenderDiagnosticSlot]>, CompileFailure> {
    let mut slots = Vec::new();
    slots
        .try_reserve_exact(RENDER_DIAGNOSTIC_SLOTS)
        .map_err(|_| failure(ResourceFault::Allocation))?;
    for _ in 0..RENDER_DIAGNOSTIC_SLOTS {
        slots.push(RenderDiagnosticSlot::try_new()?);
    }
    Ok(slots.into_boxed_slice())
}

pub(crate) struct ObservedPreparedToken {
    pub(crate) inner: Option<Box<protocol::PreparedStructuralCommand>>,
}

impl ObservedPreparedToken {
    pub(crate) fn new(inner: Box<protocol::PreparedStructuralCommand>) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            owners.token_constructed += 1;
            owners.replay_candidate_constructed += 1;
        });
        Self { inner: Some(inner) }
    }

    pub(crate) fn get(&self) -> &protocol::PreparedStructuralCommand {
        self.inner.as_deref().expect("observed token is live")
    }

    pub(crate) fn get_mut(&mut self) -> &mut protocol::PreparedStructuralCommand {
        self.inner.as_deref_mut().expect("observed token is live")
    }

    /// Commit the token. `path` is how a model transaction was applied (#1313); a token of
    /// monitor writes only commits no model and takes the monitor path inside the protocol commit.
    pub(crate) fn commit(
        mut self,
        controller: &mut ObservedController,
        path: protocol::EditPath,
    ) -> Result<protocol::CommittedCommandFrame, ()> {
        let prepared = self.inner.take().expect("observed token commits once");
        #[cfg(feature = "test-support")]
        let committed = if take_test_fault_state(TestStructuralFaultPhase::ProtocolCommit) {
            drop(prepared);
            Err(())
        } else {
            controller
                .commit_prepared_structural(*prepared, path)
                .map_err(|_| ())
        };
        #[cfg(not(feature = "test-support"))]
        let committed = controller.commit_prepared_structural(*prepared, path);
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| {
            owners.token_disposed += 1;
            if committed.is_ok() {
                owners.replay_candidate_published += 1;
                owners.replay_current_disposed += 1;
            } else {
                owners.replay_candidate_disposed += 1;
            }
        });
        committed.map_err(|_| ())
    }
}

impl Drop for ObservedPreparedToken {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            drop(inner);
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| {
                owners.token_disposed += 1;
                owners.replay_candidate_disposed += 1;
            });
        }
    }
}

/// The diagnostic code of a transaction that changes the session's sample rate or quantum (#1290a):
/// the plan exchange admits a replacement plan only at the envelope of the plan it replaces, so
/// the session must be re-prepared from its document instead. An adapter maps it to its own
/// "re-prepare" result.
pub const REPREPARE_REQUIRED: &str = "control.apply.reprepare_required";

/// Why a control command was refused.
#[derive(Debug)]
pub enum CommandError {
    /// The request frame cannot be correlated.
    Invalid,
    /// The response needs `required` bytes.
    BufferTooSmall {
        /// Bytes the response needs.
        required: u64,
    },
    /// A plan replacement or a queue has no room now; retry.
    Backpressure,
    /// The prospective session does not prepare or is not admitted.
    CompileRejected(CompileFailure),
    /// The control plane's own bookkeeping failed.
    Internal,
}

impl CommandError {
    /// The diagnostic text of this refusal, the same on every host (decision 17 C1). An adapter
    /// owns only its result-code numbering.
    #[must_use]
    pub fn diagnostic(&self) -> std::borrow::Cow<'_, [u8]> {
        match self {
            Self::Invalid => std::borrow::Cow::Borrowed(b"control.frame.invalid"),
            Self::BufferTooSmall { .. } => std::borrow::Cow::Borrowed(b"control.output.too_small"),
            Self::Backpressure => std::borrow::Cow::Borrowed(b"control.plan.backpressure"),
            Self::CompileRejected(failure) => failure.diagnostic(),
            Self::Internal => std::borrow::Cow::Borrowed(b"control.internal"),
        }
    }
}

impl EventError {
    /// The diagnostic text of this refusal, the same on every host (decision 17 C1). An adapter
    /// owns only its result-code numbering.
    #[must_use]
    pub const fn diagnostic(self) -> &'static [u8] {
        match self {
            Self::BufferTooSmall { .. } => b"control.event.output.too_small",
            Self::Backpressure => b"control.event.backpressure",
            Self::Internal => b"control.event.internal",
        }
    }
}

/// What the live arm did with a structural token (#1257 D2).
pub(crate) enum LiveCommit {
    /// The delta was live: the arm committed it, or refused it with nothing changed.
    Done(Result<usize, CommandError>),
    /// The delta needs a plan rebuild; the token comes back untouched.
    Rebuild(ObservedPreparedToken),
    /// The token was prepared in place (#1613) and the fast classification could not judge it:
    /// the token comes back untouched, and the arm goes on with its full preparation. Never
    /// returned for a token that holds a compilation, and never leaves `commit_live`.
    NeedsFull(ObservedPreparedToken),
}

/// The EQ owners a live commit has begun (#1265 D2). Dropping it discards every owner still in
/// `begun`, so a refusal on any path after the first `begin_owner` leaves each owner idle at its
/// committed revision; a commit takes `begun` first.
struct OpenOwners<'a> {
    effects: &'a mut [host_core::EffectControlProducer],
    begun: Vec<usize>,
}

impl Drop for OpenOwners<'_> {
    fn drop(&mut self) {
        for &index in &self.begun {
            // An open candidate always discards; an error would mean it was never begun.
            let _ = self.effects[index].discard_owner();
        }
    }
}

/// Which event lane a dequeue reads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventLane {
    /// Reliable events, render diagnostics included.
    Reliable,
    /// Lossy telemetry.
    Lossy,
}

/// Why an event dequeue was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventError {
    /// The event needs `required` bytes.
    BufferTooSmall {
        /// Bytes the event needs.
        required: u64,
    },
    /// A queue has no room now; retry.
    Backpressure,
    /// The control plane's own bookkeeping failed.
    Internal,
}

/// Write one preflighted effect record as it is; a bypass record carries its own crossfade ramp
/// (#1341 D1, #1622 D1).
fn write_effect_record(
    producer: &mut host_core::EffectControlProducer,
    stamp: engine::realtime::LiveStamp<'_>,
    record: effect_contract::EffectControlRecord,
) {
    producer
        .write(stamp, record)
        .unwrap_or_else(|_| unreachable!("every effect record passed its producer's preflight"));
}

pub(crate) fn map_encode_error(error: EncodeError) -> CommandError {
    match error {
        EncodeError::OutputTooSmall { required } => CommandError::BufferTooSmall {
            required: u64::try_from(required).unwrap_or(u64::MAX),
        },
        EncodeError::MessageKindMismatch
        | EncodeError::LimitExceeded
        | EncodeError::InvalidField => CommandError::Internal,
    }
}

pub(crate) fn map_command_process_error(error: CommandFrameProcessError) -> CommandError {
    match error {
        CommandFrameProcessError::Uncorrelatable(_) => CommandError::Invalid,
        CommandFrameProcessError::Encode(error) => map_encode_error(error),
        CommandFrameProcessError::OutputReservationTooSmall { required } => {
            CommandError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        CommandFrameProcessError::PreparedCommandOutstanding => CommandError::Backpressure,
        CommandFrameProcessError::Internal => CommandError::Internal,
    }
}

pub(crate) fn map_event_egress_error(error: EventEgressError) -> EventError {
    match error {
        EventEgressError::Encode(EncodeError::OutputTooSmall { required }) => {
            EventError::BufferTooSmall {
                required: u64::try_from(required).unwrap_or(u64::MAX),
            }
        }
        EventEgressError::ReliableQueueFull(_) => EventError::Backpressure,
        EventEgressError::Disabled
        | EventEgressError::DiagnosticStorageFull
        | EventEgressError::ObservationEndedCellMissing
        | EventEgressError::Encode(_) => EventError::Internal,
    }
}

impl<A: ControlAdapter> SessionState<A>
where
    PlanResources: From<A::Row>,
{
    /// Plans up to two structural faults for this thread, taken in order.
    #[cfg(feature = "test-support")]
    pub fn test_set_structural_faults(&mut self, faults: [Option<TestStructuralFaultPhase>; 2]) {
        let _ = self;
        TEST_FAULT_STATE.with(|state| state.set((faults, 0)));
    }

    /// The thread's lifecycle counters.
    #[cfg(feature = "test-support")]
    pub fn test_owner_counters(&self) -> TestOwnerCounters {
        let _ = self;
        TEST_OWNER_STATE.with(core::cell::Cell::get)
    }

    #[cfg(feature = "test-support")]
    pub(crate) fn take_test_fault(&mut self, phase: TestStructuralFaultPhase) -> bool {
        let _ = self;
        take_test_fault_state(phase)
    }

    /// Republish the render thread's peak-scan gate from the accepted telemetry configuration.
    ///
    /// Issue #163 phase 4 item 2. Called wherever the endpoint's telemetry configuration can have
    /// just changed, which is every path that reaches this control state: an accepted
    /// `ConfigureTelemetry` arrives inside `command`, and a plan swap re-reads it here. The flag
    /// only ever lags by the width of one concurrent render, and `publish_render_observation`
    /// marks such a block unmeasured rather than letting it publish a fabricated `0.0`.
    pub(crate) fn refresh_render_peak_gate(&self) {
        let telemetry = self.controller.telemetry_configuration();
        let observed = !telemetry.meter_handles.is_empty() && telemetry.meter_period_blocks != 0;
        self.shared
            .render_peak_observed
            .store(observed, Ordering::Relaxed);
    }

    /// Stages the `COUNTER_SNAPSHOT` event once render has completed at least the configured
    /// `counter_period_blocks` blocks since the last staged one (#1351 D4; the render sequence
    /// counts blocks). Its values are, for each configured ID in order, the provider's current
    /// value for that ID, which the service step refreshed first (D3). Blocks rendered with no
    /// control call in between yield one snapshot at the next call: the lane is lossy and skipped
    /// periods are not replayed.
    fn stage_counter_snapshot(
        &mut self,
        sequence: u64,
        observed_sample: protocol::SampleTime,
        revision: protocol::SessionRevision,
    ) {
        let telemetry = self.controller.telemetry_configuration();
        if telemetry.counter_ids.is_empty()
            || telemetry.counter_period_blocks == 0
            || sequence.saturating_sub(self.counter_snapshot_sequence)
                < u64::from(telemetry.counter_period_blocks)
        {
            return;
        }
        let ids = telemetry.counter_ids.iter().map(|&id| id as u32).collect();
        self.counter_snapshot_sequence = sequence;
        let Ok(snapshot) = protocol::ControlProvider::counters(
            self.controller.provider_mut(),
            &protocol::CountersRequest { all: false, ids },
        ) else {
            return;
        };
        let _ = self.controller.stage_counter_snapshot_event(
            revision,
            &protocol::CounterSnapshot {
                observed_sample,
                values: snapshot.values,
            },
        );
    }

    pub(crate) fn collect_render_activity(&mut self) {
        self.refresh_render_peak_gate();
        let sequence = self.shared.render_sequence.load(Ordering::Acquire);
        // While no counter snapshot is configured the baseline follows render, so a
        // configuration counts its period from the call that accepted it, never from the first
        // block ever rendered.
        {
            let telemetry = self.controller.telemetry_configuration();
            if telemetry.counter_ids.is_empty() || telemetry.counter_period_blocks == 0 {
                self.counter_snapshot_sequence = sequence;
            }
        }
        if sequence == self.observed_render_sequence {
            return;
        }
        self.observed_render_sequence = sequence;
        let sample = self.shared.render_sample.load(Ordering::Acquire);
        let peak = f32::from_bits(self.shared.render_peak_bits.load(Ordering::Acquire));
        let observed_sample = protocol::SampleTime(sample);
        let revision = self.controller.session().revision();
        // A `NaN` peak is the render thread saying it did not measure this block, which happens
        // for exactly one block when telemetry is configured concurrently with a render. Staging
        // it would publish a peak of `0.0` that no sample ever produced; dropping it costs one
        // record on the lane that is documented to coalesce and drop.
        let meter_len = if peak.is_nan() {
            0
        } else {
            self.controller
                .telemetry_configuration()
                .meter_handles
                .len()
        };
        for index in 0..meter_len {
            let handle = self.controller.telemetry_configuration().meter_handles[index];
            let _ = self.controller.stage_meter_batch_event(
                revision,
                observed_sample,
                &[protocol::MeterRecord {
                    handle,
                    component: protocol::MeterComponent::Left,
                    flags: 1,
                    value: peak,
                }],
            );
        }
        self.stage_counter_snapshot(sequence, observed_sample, revision);
        let diagnostics_enabled = {
            let configuration = self.controller.telemetry_configuration();
            configuration.diagnostics_enabled
                && (protocol::DiagnosticSeverity::Info as u8)
                    >= (configuration.minimum_diagnostic_severity as u8)
        };
        if !diagnostics_enabled || self.render_diagnostic_len == self.render_diagnostics.len() {
            return;
        }

        // A reliable-event reservation is the capacity credit for this CAPI-owned event. The
        // barrier records already-published protocol events, preserving their FIFO order while
        // the diagnostic itself remains in fixed, eagerly allocated CAPI storage.
        let protocol_events_before = self
            .controller
            .queues()
            .report(protocol::QueueKind::ReliableEvent)
            .occupancy
            .saturating_add(u64::from(self.protocol_reliable_pending));
        let Ok(reservation) = self.controller.queues_mut().reserve_reliable_event() else {
            return;
        };
        let tail = (self.render_diagnostic_head + self.render_diagnostic_len)
            % self.render_diagnostics.len();
        let slot = &mut self.render_diagnostics[tail];
        debug_assert!(!slot.occupied);
        debug_assert!(slot.reservation.is_none());
        slot.diagnostic.sample_time = Some(observed_sample.0);
        slot.diagnostic.provider_sequence = Some(sequence);
        slot.revision = revision;
        slot.protocol_events_before = protocol_events_before;
        slot.reservation = Some(reservation);
        slot.occupied = true;
        self.controller
            .provider_mut()
            .set_render_diagnostic(tail, sample, sequence, true);
        self.render_diagnostic_len += 1;
    }

    /// `(revision, replay entries, current provider epoch, pending providers)`.
    #[cfg(feature = "test-support")]
    pub fn test_state_summary(&self) -> (u64, usize, u64, usize) {
        (
            self.controller.session().revision().0,
            self.controller.replay().len(),
            self.providers.epoch,
            self.pending_providers.len(),
        )
    }

    /// Everything a transaction may change.
    #[cfg(feature = "test-support")]
    pub fn test_transaction_snapshot(&self) -> TestTransactionSnapshot<A::Row> {
        let controller = self.controller.retained_configuration_capacity();
        let (provider, provider_counters) = self.controller.provider().retained_capacities();
        let replay = self.controller.replay().retained_storage_capacities();
        TestTransactionSnapshot {
            revision: self.controller.session().revision().0,
            canonical: self
                .controller
                .session()
                .canonical_snapshot()
                .as_bytes()
                .to_vec(),
            model: self.controller.session().compiled().resource_estimate(),
            replay_entries: self.controller.replay().len(),
            provider_epoch: self.providers.epoch,
            pending_provider_epochs: self
                .pending_providers
                .iter()
                .map(|provider| provider.epoch)
                .collect(),
            retired_provider_epochs: self
                .retired_providers
                .iter()
                .map(|provider| provider.epoch)
                .collect(),
            active_plan_epoch: self.shared.active_epoch.load(Ordering::Acquire),
            resource_rows: self
                .shared
                .reports
                .lock()
                .expect("test report lock")
                .clone(),
            reliable_event: self
                .controller
                .queues()
                .report(protocol::QueueKind::ReliableEvent),
            reliable_response: self
                .controller
                .queues()
                .report(protocol::QueueKind::ReliableResponse),
            telemetry: self
                .controller
                .queues()
                .report(protocol::QueueKind::Telemetry),
            telemetry_counters: self.controller.queues().telemetry_counters(),
            retained_capacities: [
                controller.meter_handles,
                controller.counter_ids,
                provider.meter_handles,
                provider.counter_ids,
                provider_counters,
                replay.0,
                replay.1,
            ],
            strip_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.controls.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.fader.test_only_write_counts(),
                            producer.producer.test_only_write_count(),
                        )
                    })
                })
                .collect(),
            route_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.routes.iter().map(|producer| {
                        (
                            provider.epoch,
                            Box::<str>::from(producer.route_id()),
                            producer.test_only_write_count(),
                        )
                    })
                })
                .collect(),
            effect_cells: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.effects.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.effect_id.clone(),
                            producer.writer().test_only_write_counts(),
                        )
                    })
                })
                .collect(),
            effect_owners: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.effects.iter().filter_map(|producer| {
                        let owner = producer.owner()?;
                        let bits = |rows: &[effect_contract::InitialParameterValue]| {
                            rows.iter().map(|row| row.value.to_bits()).collect()
                        };
                        Some((
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.effect_id.clone(),
                            owner.committed_revision(),
                            format!("{:?}", owner.phase()),
                            bits(owner.committed()),
                            bits(owner.candidate()),
                        ))
                    })
                })
                .collect(),
        }
    }

    /// The telemetry counters.
    #[cfg(feature = "test-support")]
    pub fn test_telemetry_counters(&self) -> protocol::TelemetryCounters {
        self.controller.queues().telemetry_counters()
    }

    /// The controller's, provider's and replay cache's retained capacities.
    #[cfg(feature = "test-support")]
    pub fn test_retained_capacities(&self) -> [usize; 7] {
        let controller = self.controller.retained_configuration_capacity();
        let (provider, provider_counters) = self.controller.provider().retained_capacities();
        let replay = self.controller.replay().retained_storage_capacities();
        [
            controller.meter_handles,
            controller.counter_ids,
            provider.meter_handles,
            provider.counter_ids,
            provider_counters,
            replay.0,
            replay.1,
        ]
    }

    /// The protocol controller.
    #[cfg(feature = "test-support")]
    pub fn test_controller(&self) -> &ProtocolController<SessionControlProvider> {
        &self.controller
    }

    /// The current provider epoch.
    #[cfg(feature = "test-support")]
    pub fn test_providers(&self) -> &ProviderEpoch {
        &self.providers
    }

    /// The pending provider epochs, oldest first.
    #[cfg(feature = "test-support")]
    pub fn test_pending_providers(&self) -> &[ProviderEpoch] {
        &self.pending_providers
    }

    /// The retired provider epochs, waiting for their plans' reclamation.
    #[cfg(feature = "test-support")]
    pub fn test_retired_providers(&self) -> &[ProviderEpoch] {
        &self.retired_providers
    }

    /// Schedules the next structural publication's adoption (#1348 gate 4): `command` reserves
    /// that one candidate with `adoption` instead of [`PlanAdoption::Next`]. Taken once, by the
    /// next structural publication on this thread.
    #[cfg(feature = "test-support")]
    pub fn test_set_next_adoption(&mut self, adoption: PlanAdoption) {
        let _ = self;
        TEST_NEXT_ADOPTION.with(|state| state.set(Some(adoption)));
    }

    /// The provider's counter snapshot, every counter, as `COUNTERS_GET` would read it now, without
    /// running a control call (and so without the service step's refresh).
    #[cfg(feature = "test-support")]
    pub fn test_provider_counters(&mut self) -> Vec<protocol::CounterValue> {
        use protocol::ControlProvider as _;
        self.controller
            .provider_mut()
            .counters(&protocol::CountersRequest {
                all: true,
                ids: Vec::new(),
            })
            .expect("every counter")
            .values
    }

    /// Every plan the control plane holds unreclaimed, as the admissions read them (#1398 D4): the
    /// running plan first, then every pending or withdrawn candidate, the newest last.
    ///
    /// Each row is the provider's own epoch's, read from the report table, never the atomic's: a
    /// plan render has displaced is reclaimed before any admission runs (`service`), so a row the
    /// atomic still names while it lags is not a plan the control plane holds (#1042).
    pub(crate) fn held_epochs(&self) -> Result<HeldEpochs, CommandError> {
        let reports = self
            .shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?;
        let row = |provider: &ProviderEpoch| {
            reports
                .iter()
                .find_map(|(epoch, report)| (*epoch == provider.epoch).then_some(*report))
                .map(|report| LiveEpochResources {
                    report: PlanResources::from(report),
                    capi: provider.capi,
                })
                .ok_or(CommandError::Internal)
        };
        if self.pending_providers.len() > PENDING_PROVIDERS {
            return Err(CommandError::Internal);
        }
        // No allocation: the live arm's admission runs it too.
        let mut rows = [row(&self.providers)?; 1 + PENDING_PROVIDERS];
        for (slot, provider) in rows[1..].iter_mut().zip(&self.pending_providers) {
            *slot = row(provider)?;
        }
        Ok(HeldEpochs {
            rows,
            len: 1 + self.pending_providers.len(),
        })
    }

    /// Brings the provider epochs and the resource-report table up to the render thread.
    ///
    /// A render call swaps plans and retires the outgoing one at its *start*, but publishes
    /// `active_epoch` only after it returns (issue #1042). A control call in between reads an
    /// atomic that lags the plan actually rendering, so the atomic alone cannot drive promotion:
    /// a reclaimed plan whose epoch is still the current provider's proves that the one pending
    /// candidate has replaced it, and promotes that candidate there. After such a promotion the
    /// atomic lags the providers until the render call returns, so the atomic promotes only when
    /// it is ahead of them.
    fn synchronize_plan_epochs(&mut self) -> Result<(), CommandError> {
        let active_epoch = self.shared.active_epoch.load(Ordering::Acquire);
        if active_epoch > self.providers.epoch {
            self.promote_pending_provider(active_epoch)?;
        }

        while let Ok((retired_epoch, retired_plan)) = self.retirer.try_reclaim() {
            drop(ObservedRetiredPlan::new(retired_plan));
            if retired_epoch.0 == self.providers.epoch {
                let replacement = self
                    .pending_providers
                    .first()
                    .ok_or(CommandError::Internal)?
                    .epoch;
                self.promote_pending_provider(replacement)?;
            }
            let index = self
                .retired_providers
                .iter()
                .position(|provider| provider.epoch == retired_epoch.0)
                .ok_or(CommandError::Internal)?;
            self.retired_providers.remove(index);
        }

        // Keep exactly the rows a reader can still ask for: the atomic's (possibly lagging), the
        // current provider's, and a pending candidate's. The atomic is read under the lock, and
        // `retain` shrinks in place.
        let current = self.providers.epoch;
        let pending = &self.pending_providers;
        let mut reports = self
            .shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?;
        let active = self.shared.active_epoch.load(Ordering::Acquire);
        reports.retain(|(epoch, _)| {
            *epoch == active
                || *epoch == current
                || pending.iter().any(|provider| provider.epoch == *epoch)
        });
        Ok(())
    }

    /// Makes the pending provider of `epoch` current and parks the previous one until its plan
    /// is reclaimed.
    fn promote_pending_provider(&mut self, epoch: u64) -> Result<(), CommandError> {
        let index = self
            .pending_providers
            .iter()
            .position(|provider| provider.epoch == epoch)
            .ok_or(CommandError::Internal)?;
        if self.retired_providers.len() == self.retired_providers.capacity() {
            return Err(CommandError::Internal);
        }
        let next = self.pending_providers.remove(index);
        let previous = core::mem::replace(&mut self.providers, next);
        self.retired_providers.push(previous);
        Ok(())
    }

    /// One bounded step of control work between edits (#1348 D1): the engine owns no thread, so
    /// the host drives it, through `miso_engine_v1_service` or any other session call, each of
    /// which runs it first (D2). In order, it
    ///
    /// 1. brings the provider epochs up to render and reclaims retired plans
    ///    (`synchronize_plan_epochs`);
    /// 2. refreshes the provider's counters that a source outside it serves (D8);
    /// 3. stages the render observation (`collect_render_activity`);
    /// 4. refreshes those counters again, so a record that staging coalesced or dropped is
    ///    counted in this same call.
    ///
    /// **Bounded (D3).** Every retired plan holds one retirement credit, the credits number the
    /// retirement queue's capacity, and only a publication -- never this step -- reserves one, so
    /// one call reclaims at most that capacity of plans. It stages at most one render observation,
    /// never loops on render progress and never waits, so its cost does not grow with the time
    /// since the last call.
    ///
    /// **Pending candidates (D7).** Nothing here takes a pending candidate: it stays in its
    /// mailbox cell, with its plan, epoch, revision gate and retirement credit, until render adopts
    /// it or a publication withdraws it. A provider is promoted only once render has adopted its
    /// plan.
    ///
    /// **Acked-batch question (D6): can an ack ever precede a drop? No.** The step commits and
    /// acknowledges nothing and adds no queue. The only things it discards are plans render has
    /// already displaced; a committed revision's plan is never among them.
    pub fn service(&mut self) -> Result<(), CommandError> {
        self.fold_observation_pending();
        self.synchronize_plan_epochs()?;
        self.refresh_provider_counters();
        self.collect_render_activity();
        self.refresh_provider_counters();
        Ok(())
    }

    /// Sets every counter the provider serves from a source outside it (#1348 D8): the
    /// controller's telemetry counters and the session's `live_values_superseded` count (#1312
    /// D8). A counter added later joins this refresh, never a single
    /// control call.
    fn refresh_provider_counters(&mut self) {
        let telemetry_counters = self.controller.queues().telemetry_counters();
        let superseded = self.live_values_superseded.get();
        let provider = self.controller.provider_mut();
        provider.set_telemetry_counters(telemetry_counters);
        provider.set_live_values_superseded(superseded);
    }

    /// Runs one control command frame. A success returns the response's length in the response
    /// scratch ([`Self::command_response`]); a structural command either rides the newest plan's
    /// live lanes or prepares, admits and publishes a replacement plan.
    pub fn command(&mut self, request: &[u8], output_capacity: u64) -> Result<usize, CommandError> {
        self.service()?;
        let output_capacity = usize::try_from(output_capacity).unwrap_or(usize::MAX);
        // #1613: a transaction of values is prepared in place (no clone, no compile); every other
        // command is prepared as it always was. A test can force the full preparation.
        #[cfg(feature = "test-support")]
        let in_place = !FORCE_FULL_PREPARATION.with(core::cell::Cell::get);
        #[cfg(not(feature = "test-support"))]
        let in_place = true;
        let mut scratch = DecodeScratch::new(&mut self.decode_fields);
        let prepared = if in_place {
            self.controller
                .prepare_command_frame_in_place(request, &mut scratch, output_capacity)
        } else {
            self.controller
                .prepare_command_frame(request, &mut scratch, output_capacity)
        }
        .map_err(map_command_process_error)?;
        match prepared {
            PreparedCommandFrame::Immediate(response) => response
                .write_into(&mut self.response_scratch)
                .map_err(map_encode_error),
            PreparedCommandFrame::Structural(prepared) => {
                let prepared = ObservedPreparedToken::new(prepared);
                #[cfg(feature = "test-support")]
                {
                    if self.take_test_fault(TestStructuralFaultPhase::AfterProtocolPrepare) {
                        drop(prepared);
                        return Err(CommandError::Backpressure);
                    }
                }
                if !self.shared.plan_alive.load(Ordering::Acquire) {
                    return Err(CommandError::Backpressure);
                }
                let response_len = prepared.get().response_len();
                if response_len > self.response_scratch.len() || response_len > output_capacity {
                    return Err(CommandError::BufferTooSmall {
                        required: u64::try_from(response_len).unwrap_or(u64::MAX),
                    });
                }
                // #1257 D1: a value-only delta of track faders, mutes, pans and live effect
                // parameters (#1264) rides the newest plan's live lanes; every other delta takes
                // the rebuild below, unchanged.
                let mut prepared = match self.commit_live(prepared) {
                    LiveCommit::Done(result) => return result,
                    LiveCommit::Rebuild(prepared) => prepared,
                    // `commit_live` materializes an in-place token before it returns.
                    LiveCommit::NeedsFull(_) => return Err(CommandError::Internal),
                };
                // #1290a: a plan's rate and quantum are fixed for its session: the plan exchange
                // admits a replacement only at the same envelope, and would hand a different one
                // back as retryable backpressure. A transaction that changes either is refused
                // here, typed and permanent, before anything is prepared, stamped or queued.
                {
                    let prospective = prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?;
                    let committed = self.controller.session().compiled();
                    if prospective.sample_rate() != committed.sample_rate()
                        || prospective.quantum() != committed.quantum()
                    {
                        drop(prepared);
                        return Err(CommandError::CompileRejected(crate::error::diagnostic(
                            REPREPARE_REQUIRED,
                        )));
                    }
                }
                #[cfg(feature = "test-support")]
                if self.take_test_fault(TestStructuralFaultPhase::BeforeRuntimePrepare) {
                    drop(prepared);
                    return Err(CommandError::Backpressure);
                }
                // #1277 D10: `r`, the revision this rebuild commits as, is read once, here: the
                // successor's base carries it, the preparation stamps its carry retargets with it
                // on the candidate's own gate pin, and the stamp below is the same value again.
                // #1503 D1 holds a prepared transaction's revision in range, so this cannot
                // refuse; were it to, nothing has been built. A transaction of monitor writes
                // only never takes this arm (`commit_live` refuses it), so this revision is
                // always the one the commit gives (#1603).
                let revision = prepared.get().prospective_session().revision().0;
                if !(1..=engine::realtime::GATE_REVISION_MAX).contains(&revision) {
                    return Err(CommandError::Internal);
                }
                // #1559 D9: the successor renders under the overlay the transaction leaves, and
                // its carry joins the effective values under both overlays.
                let monitor_change = self.monitor.after(
                    prepared.get().monitor_edits(),
                    prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?
                        .normalized_model(),
                );
                // Issue #1273 D1: the candidate succeeds the newest plan, so every source the
                // transaction leaves unchanged is prepared to carry that plan's ring.
                // #1584 D6a: the subscriptions the successor is prepared with, and those the
                // transaction ends. The room for their one `OBSERVATION_ENDED` event (every ended
                // key in it) is reserved with the token's `SESSION_COMMITTED`, before anything is
                // prepared, published or queued: a lane holding unread events is retryable
                // backpressure and nothing has changed (the acked-batch rule: a subscription is
                // never dropped without its event). An event over one control frame can never
                // be sent, so it is a resource refusal, not backpressure.
                let observation_change = self
                    .observation
                    .after(
                        prepared
                            .get()
                            .prospective_session()
                            .compiled()
                            .ok_or(CommandError::Internal)?
                            .normalized_model(),
                    )
                    .map_err(|_| {
                        CommandError::CompileRejected(failure(ResourceFault::Allocation))
                    })?;
                if !observation_change.ended.is_empty() {
                    let mut ended = Vec::new();
                    ended
                        .try_reserve_exact(observation_change.ended.len())
                        .map_err(|_| {
                            CommandError::CompileRejected(failure(ResourceFault::Allocation))
                        })?;
                    ended.extend(observation_change.ended.iter().map(ended_key));
                    match self
                        .controller
                        .attach_observation_ended(prepared.get_mut(), ended)
                    {
                        Ok(_) => {}
                        Err(protocol::AttachObservationEndedError::NoRoom(_)) => {
                            drop(prepared);
                            return Err(CommandError::Backpressure);
                        }
                        Err(protocol::AttachObservationEndedError::Unencodable) => {
                            drop(prepared);
                            return Err(CommandError::CompileRejected(failure(
                                ResourceFault::Limit,
                            )));
                        }
                        Err(_) => return Err(CommandError::Internal),
                    }
                }
                let prepared_runtime = prepare_runtime::<A>(
                    &*self.preparer,
                    prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?,
                    self.limits,
                    Some(SuccessorBase {
                        inventory: &self.newest_providers().inventory,
                        committed: self.controller.session().compiled().normalized_model(),
                        revision,
                        // #1394 D6: a carry retarget is the record of the transaction's own
                        // edit, so it takes that edit's ramp length.
                        edit_ramps: prepared.get().prospective_session().edit_ramps(),
                        monitor: host_core::MonitorBase {
                            committed: &self.monitor,
                            next: &monitor_change.overlay,
                            ramp: monitor_change.ramp,
                        },
                        observation: &observation_change.bindings,
                    }),
                    &self.live_values_superseded,
                    self.observation_slots_high_water,
                )
                .map_err(CommandError::CompileRejected)?;
                let PreparedRuntime {
                    sources,
                    strips,
                    effects,
                    plan: candidate_plan,
                    resources,
                    carried,
                    inventory,
                    control_catalog: candidate_catalog,
                    capi: prospective_capi,
                    revision_gate,
                    master_track: _,
                } = prepared_runtime;
                let mut candidate_provider = ProviderEpoch::candidate(
                    sources,
                    inventory,
                    strips,
                    effects,
                    prospective_capi,
                    revision_gate,
                );
                let candidate_plan = ObservedCandidatePlan::new(candidate_plan);
                #[cfg(feature = "test-support")]
                {
                    if self.take_test_fault(TestStructuralFaultPhase::AfterRuntimePrepare) {
                        drop(candidate_plan);
                        drop(candidate_provider);
                        drop(prepared);
                        return Err(CommandError::Backpressure);
                    }
                }
                // One candidate at a time until #1310 supersedes: checked before the admission,
                // which sums every held plan (#1398 D4), so an edit made while a candidate waits
                // is retryable backpressure and never a compile rejection that the candidate's
                // adoption would undo.
                if !self.pending_providers.is_empty() {
                    return Err(CommandError::Backpressure);
                }
                // #1398 D4: the candidate beside every plan the control plane holds unreclaimed,
                // and every model it holds, checked before anything is reserved or committed.
                validate_replacement_peak(
                    self.held_epochs()?.as_slice(),
                    ProspectivePlan {
                        resources,
                        carried,
                        capi: prospective_capi,
                    },
                    compiled_model_admission(
                        self.controller.session().compiled(),
                        prepared
                            .get()
                            .prospective_session()
                            .compiled()
                            .ok_or(CommandError::Internal)?,
                    )
                    .map_err(CommandError::CompileRejected)?,
                    self.limits,
                )
                .map_err(CommandError::CompileRejected)?;
                #[cfg(feature = "test-support")]
                if self.take_test_fault(TestStructuralFaultPhase::AfterAdmission) {
                    drop(candidate_plan);
                    drop(candidate_provider);
                    drop(prepared);
                    return Err(CommandError::Backpressure);
                }
                #[cfg(feature = "test-support")]
                let adoption = TEST_NEXT_ADOPTION
                    .with(core::cell::Cell::take)
                    .unwrap_or(PlanAdoption::Next);
                #[cfg(not(feature = "test-support"))]
                let adoption = PlanAdoption::Next;
                let reservation = match self
                    .publisher
                    .reserve_replacement(candidate_plan.take(), adoption)
                {
                    Ok(reservation) => reservation,
                    Err(error) => {
                        // #1398 D3: the exchange's publication cells and its retirement credits
                        // (#1398 D1) hold every plan a candidate can find there, so a full one is
                        // an accounting fault, not backpressure. An incompatible envelope and an
                        // exhausted epoch stay as they were.
                        let (returned, refusal) = match error {
                            PlanReplacementReservationError::PublicationFull(plan)
                            | PlanReplacementReservationError::RetirementFull(plan) => {
                                debug_assert!(
                                    false,
                                    "#1398 D1 sizes the exchange for every candidate"
                                );
                                (plan, CommandError::Internal)
                            }
                            PlanReplacementReservationError::Incompatible(plan)
                            | PlanReplacementReservationError::EpochExhausted(plan) => {
                                (plan, CommandError::Backpressure)
                            }
                        };
                        drop(ObservedCandidatePlan::returned(returned));
                        return Err(refusal);
                    }
                };
                let reservation = ObservedReservation::new(reservation);
                #[cfg(feature = "test-support")]
                {
                    if take_test_fault_state(TestStructuralFaultPhase::AfterPlanReservation) {
                        drop(reservation);
                        drop(candidate_provider);
                        drop(prepared);
                        return Err(CommandError::Backpressure);
                    }
                }
                let epoch = reservation.epoch();
                candidate_provider.epoch = epoch;
                let mut reports = self
                    .shared
                    .reports
                    .lock()
                    .map_err(|_| CommandError::Internal)?;
                // #1398 D3: the report table and the pending list hold every row a candidate can
                // meet (#1398 D1), so a full one is an accounting fault.
                if reports.len() == reports.capacity()
                    || self.pending_providers.len() == self.pending_providers.capacity()
                {
                    debug_assert!(
                        false,
                        "#1398 D1 sizes the report table and the pending list"
                    );
                    return Err(CommandError::Internal);
                }

                #[cfg(feature = "test-support")]
                if take_test_fault_state(TestStructuralFaultPhase::BeforeProtocolCommit) {
                    drop(reports);
                    drop(reservation);
                    drop(candidate_provider);
                    drop(prepared);
                    return Err(CommandError::Backpressure);
                }

                // #1312 D7: the candidate's gate is stamped with the structural revision, as the
                // last fallible step before the protocol commit. A fresh gate takes any revision
                // in `1..=GATE_REVISION_MAX`, which #1503 holds for a prepared transaction, so
                // this cannot refuse; were it to, the candidate and its handle are dropped here
                // with nothing published. Nothing else is stamped on the handle before the
                // publication below. The handle inherits the open stamp of the carry retargets
                // preparation wrote (#1277 D5), so this is the same revision again, which the pin
                // allows.
                if candidate_provider.revision_gate.stamp(revision).is_err() {
                    drop(reports);
                    drop(reservation);
                    drop(candidate_provider);
                    drop(prepared);
                    return Err(CommandError::Internal);
                }
                // #1584 D6a: the newest plan's pending-subscription storage is built here, with
                // the other fallible steps, so no allocation follows the commit; the commit then
                // only assigns it. A refusal drops what was built and changes nothing.
                let slots = match observation_slots(
                    &candidate_provider.effects,
                    &observation_change.bindings,
                    self.observation_wire,
                ) {
                    Ok(slots) => slots,
                    Err(_) => {
                        drop(reports);
                        drop(reservation);
                        drop(candidate_provider);
                        drop(prepared);
                        return Err(CommandError::CompileRejected(failure(
                            ResourceFault::Allocation,
                        )));
                    }
                };
                // #1313 D2: a transaction that takes this arm is a rebuild, whatever else it holds.
                let committed = prepared
                    .commit(&mut self.controller, protocol::EditPath::Rebuild)
                    .map_err(|_| CommandError::Internal)?;
                // #1559 D8: the overlay is written after the last fallible step, with the commit.
                self.monitor = monitor_change.overlay;
                // #1584 D6a: and the subscriptions the successor was prepared with.
                self.observation = observation_change.bindings;
                // Issue #1273 D2: the protocol commit was the last fallible step, so the
                // persisting producers move now and never move back. Infallible.
                // The newest epoch, spelled by field: the reservation borrows the publisher.
                let newest = self
                    .pending_providers
                    .last_mut()
                    .unwrap_or(&mut self.providers);
                candidate_provider
                    .sources
                    .adopt_persisting(&mut newest.sources);
                self.controller
                    .provider_mut()
                    .replace_session_catalog(candidate_catalog, revision);
                // #1502 D6: the candidate carries the revision just committed. It goes on the
                // candidate's own gate before the candidate is published, so render reports it
                // when it adopts the candidate and never earlier, and nothing is stamped on the
                // handle in between.
                debug_assert_eq!(
                    revision,
                    self.controller.session().revision().0,
                    "the revision stamped is the revision committed (#1312 D7)"
                );
                candidate_provider.revision_gate.publish(revision);
                self.pending_providers.push(candidate_provider);
                // #1584 D6a: the newest plan's slots, built above. The fold ran when this command
                // was serviced and nothing has been pushed since (one thread), so none is lost.
                (
                    self.observation_offsets,
                    self.observation_pending,
                    self.observation_batch_counts,
                    self.observation_armed,
                    self.observation_armed_bytes,
                ) = slots;
                self.observation_slots_high_water = self
                    .observation_slots_high_water
                    .max(self.observation_pending.len());
                reports.push((epoch, A::Row::from(resources)));
                reservation.commit();
                #[cfg(feature = "test-support")]
                {
                    update_test_owners(|owners| {
                        owners.candidate_provider_published += 1;
                    });
                }
                Ok(committed
                    .write_into(&mut self.response_scratch)
                    .expect("prepared response capacity was admitted before protocol commit"))
            }
        }
    }

    /// The live arm of a structural command (#1257 D2, #1053 D6).
    ///
    /// Classifies the committed model against the token's prospective one. A rebuild hands the
    /// token back untouched. A live delta runs, in order: the live admission (#1053 D8), the
    /// producer resolution by strip ID, and by strip ID and live address for an effect, in the
    /// newest epoch (#1053 D7), each EQ owner's transaction -- `begin_owner`, `edit_owner` per
    /// edit and `preflight_candidate_targets` for the classifier's designed targets (#1265 D2) --
    /// every effect record's preflight (#1264 D3),
    /// and the protocol's own commit predicate. Each of them refuses before any cell or the model
    /// changes, and a refusal after an owner was begun discards every begun owner (`OpenOwners`).
    ///
    /// Every live value is a latest-target cell (#1312, #1345): a strip's fader, mute and
    /// matrix/pan values, its input trim, polarity and filter targets (#1556), a send's folded
    /// target and gate (#1557), an effect's parameter values, its bypass and an EQ's targets. A write
    /// needs no room, so nothing refuses a delta for room and nothing turns it into a rebuild
    /// because it outgrew a queue. After the predicate and the test fault hook, the commit takes
    /// its stamp from the newest epoch's gate as its last fallible step (D7). Only then does it
    /// write every strip cell, every effect cell and every EQ owner's target cells, commit the
    /// token and every owner, and publish the stamped revision on the gate, and none of them can
    /// fail: a cell has no room to want, every effect record passed its producer's preflight and
    /// every target set its owner's, and the predicate is the commit's own under this `&mut`
    /// borrow. So no acked edit is ever dropped, and no refused edit leaves a cell write, a
    /// target, an open stamp or an open owner behind. An instance's `Bypass` record (#1266 D2) is
    /// a cell write on every instance, the EQ's included: render reads the bypass cell first and
    /// the parameter and target cells after it (#1345 D3).
    ///
    /// It never reserves a plan or adds a report row. It stages the committed model's control
    /// catalog before the first write and publishes it after the commit, so the control list
    /// reports what a rebuild's catalog would (#1264 D4, #1265 D3, #1552 D3). The stage reads only
    /// the rows of the owners whose values changed and the derived rows, unless an address, a
    /// descriptor or a setting changed, which stages the whole catalog (#1600). The structure
    /// epoch advances if a descriptor or an address changed: a VCA added or removed live adds or
    /// removes its rows (#1558 D5), a member change, an offset or a mute changes none.
    ///
    /// **A token prepared in place (#1613).** A transaction of `SetControl`s, on existing values or
    /// adding parameter entries (#1620), holds the owners it edited and no compilation
    /// (`PreparedSessionTransaction::is_in_place`). The classification, the admission, the catalog stage and the producers' preflight read the
    /// committed model and those owners as one view (`controls::ModelView`), and nothing writes the
    /// committed model before the commit: the protocol commit swaps the owners in as the last write.
    /// If the fast classification cannot judge the transaction, the token is materialized (the full
    /// preparation of its edits) and this arm goes on as for any other token, a rebuild included.
    fn commit_live(&mut self, prepared: ObservedPreparedToken) -> LiveCommit {
        match self.commit_live_once(prepared) {
            LiveCommit::NeedsFull(mut prepared) => {
                // #1613: the fast classification could not judge the in-place transaction. Its
                // edits are prepared by the full path now (nothing was written), and the arm goes
                // on as it always did, a rebuild included.
                #[cfg(feature = "test-support")]
                IN_PLACE_COUNTS.with(|counts| {
                    let (commits, materialized) = counts.get();
                    counts.set((commits, materialized + 1));
                });
                if self
                    .controller
                    .materialize_prepared(prepared.get_mut())
                    .is_err()
                {
                    return LiveCommit::Done(Err(CommandError::Internal));
                }
                self.commit_live_once(prepared)
            }
            other => other,
        }
    }

    /// One attempt of [`Self::commit_live`]: the live arm over the token as it is. A token prepared
    /// in place is classified from its rows over the view of the committed model and its edited
    /// owners; if that cannot judge, the token is handed back as [`LiveCommit::NeedsFull`].
    fn commit_live_once(&mut self, prepared: ObservedPreparedToken) -> LiveCommit {
        let transaction = prepared.get().prospective_session();
        let in_place = transaction.is_in_place();
        let committed = self.controller.session().compiled().normalized_model();
        // The model after the transaction, read as the committed model and the owners the
        // transaction edited (#1613), or the prospective compilation's model, whole.
        let view = match (transaction.edited_owners(), in_place) {
            (Some(owners), _) => controls::ModelView::edited(committed, owners),
            (None, true) => controls::ModelView::whole(committed),
            (None, false) => controls::ModelView::whole(
                transaction
                    .compiled()
                    .expect("a compiled transaction")
                    .normalized_model(),
            ),
        };
        let next = view.structure();
        // #1559 D6, D7: the overlay the transaction leaves, and the classifier's view of both.
        let monitor_change = self.monitor.after(prepared.get().monitor_edits(), next);
        let monitor_only = prepared.get().is_monitor_only();
        let monitor = host_core::MonitorBase {
            committed: &self.monitor,
            next: &monitor_change.overlay,
            ramp: monitor_change.ramp,
        };
        let ramps = host_core::LiveRamps::for_session(next);
        let delta = if in_place {
            // #1613: classified from the rows the edits name over the view, with no model built.
            let rows = transaction.set_control_rows().unwrap_or_default();
            match host_core::classify_live_staged(
                committed,
                view,
                ramps,
                transaction.edit_ramps(),
                monitor,
                rows,
            ) {
                Some(delta) => delta,
                None => return LiveCommit::NeedsFull(prepared),
            }
        } else {
            // #1612: a transaction of `SetControl`s is classified from the rows it names, with no
            // model clone and no canonical JSON; any other, and any row the fast path cannot
            // judge, runs the full classifier inside this one call.
            match host_core::classify_live_edit(
                committed,
                next,
                ramps,
                transaction.edit_ramps(),
                monitor,
                transaction.set_control_rows(),
            ) {
                Ok(delta) => delta,
                Err(_) => {
                    // A monitor-only transaction never rebuilds (#1603 D7): its records are strip
                    // mute and route gate records, which every plan carries, so a rebuild verdict
                    // is a defect.
                    if monitor_only {
                        return LiveCommit::Done(Err(CommandError::Internal));
                    }
                    return LiveCommit::Rebuild(prepared);
                }
            }
        };
        // Whether the commit publishes on the plan gate: every model transaction does (a delta
        // with no record is a revision render has nothing to do for), and a transaction of
        // monitor writes does when the overlay change produced a record (#1559 D4, #1603 D7).
        let publishes = !monitor_only || !delta.is_empty();

        // 1. Admission (#1053 D8), with the catalog of the committed model built first, so that
        // its growth is charged and the commit's last steps cannot fail for room (#1552 D3,
        // #1558 D6). Its refusal is the compile diagnostic a rebuild would give, before any write.
        let catalog = match self.live_admission(&prepared, &monitor_change.overlay, view, &delta) {
            Ok(catalog) => catalog,
            Err(error) => return LiveCommit::Done(Err(error)),
        };

        // 2. Resolve each strip's producer by ID in the newest epoch (#1053 D7), among all strips,
        // tracks then submixes (#1554 D6): a binary search per segment of the strip table, whose
        // IDs are sorted within each segment.
        let ProviderEpoch {
            epoch: newest_epoch,
            strips,
            effects,
            revision_gate,
            capi: newest_capi,
            ..
        } = match self.pending_providers.last_mut() {
            Some(provider) => provider,
            None => &mut self.providers,
        };
        let track_count = strips.track_count;
        let strip_lanes = &mut strips.controls;
        let strip_rank = |id: &str| {
            host_core::strip_position(
                strip_lanes.len(),
                track_count,
                |index| &*strip_lanes[index].track_id,
                id,
            )
        };
        let mut resolved = Vec::new();
        if resolved.try_reserve_exact(delta.strips.len()).is_err() {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for strip in &delta.strips {
            let Some(index) = strip_rank(strip.strip_id) else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            resolved.push(index);
        }

        // The effect instances, by strip ID and live address (#1264 D3), through the producer
        // table's sort (#1554 D6).
        let mut resolved_effects = Vec::new();
        let mut begun = Vec::new();
        if resolved_effects
            .try_reserve_exact(delta.effects.len())
            .is_err()
            || begun.try_reserve_exact(delta.effects.len()).is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for instance in &delta.effects {
            let Some(index) = host_core::effect_producer_position(
                effects,
                strip_rank,
                instance.strip_id,
                instance.address,
            ) else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            // A delta that carries targets addresses an instance that owns them (#1265 D2).
            if instance.targets.is_some() && !effects[index].has_owner() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            resolved_effects.push(index);
        }

        // 2a. Each route's producer, by route ID (#1557 D4): a binary search over the canonical
        // route-ID order. A record for a route with no producer is an internal fault, found
        // before the first write.
        let route_lanes = &mut strips.routes;
        let mut resolved_routes = Vec::new();
        if resolved_routes
            .try_reserve_exact(delta.routes.len())
            .is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        for route in &delta.routes {
            let Some(index) = host_core::route_producer_position(route_lanes, route.route_id)
            else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            resolved_routes.push(index);
        }
        // 2a'. A delta with new jump lengths writes them to the newest plan's cell (#1639 D3),
        // which every epoch holds (D1).
        let jump_lane = &mut strips.jump_lengths;

        // 2b. A strip with input records has an input writer, and every filter target passes the
        // contract the writer assumes (#1556 D5). Both are checked before the first write: the
        // cells' write cannot fail and panics on a target it was never validated for.
        for (strip, &index) in delta.strips.iter().zip(&resolved) {
            if strip.input.is_empty() {
                continue;
            }
            if strip_lanes[index].input.is_none() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            if !strip.input_records_valid() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
        }

        // 3. Each EQ owner's transaction (#1265 D2): begin at its committed revision, apply the
        // edits, and preflight the designed targets (revision and validation; the targets are
        // cells and want no room). From here on, every refusal discards every owner begun, as
        // `OpenOwners` drops.
        let mut owners = OpenOwners { effects, begun };
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let Some(targets) = &instance.targets else {
                continue;
            };
            let producer = &mut owners.effects[index];
            let Some(base_revision) = producer.owner().map(|owner| owner.committed_revision())
            else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            // `begin` allocates the candidate fallibly (#1624): its `Allocation` refusal maps to
            // `INTERNAL`, as every failed `try_reserve_exact` of this commit does, before the
            // stamp. Nothing is written or acknowledged, and `OpenOwners` discards the owners
            // begun so far.
            if producer.begin_owner(base_revision).is_err() {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
            owners.begun.push(index);
            for &record in &instance.records {
                let (parameter_index, channel, value) = match record {
                    effect_contract::EffectControlRecord::Parameter {
                        parameter_index,
                        channel,
                        value,
                    } => (parameter_index, channel, value),
                    // Pushed, not an owner edit (#1266 D2).
                    effect_contract::EffectControlRecord::Bypass(_) => continue,
                    _ => return LiveCommit::Done(Err(CommandError::Internal)),
                };
                if producer
                    .edit_owner(parameter_index, channel, value)
                    .is_err()
                {
                    return LiveCommit::Done(Err(CommandError::Internal));
                }
            }
            if producer
                .preflight_candidate_targets(base_revision, targets)
                .is_err()
            {
                return LiveCommit::Done(Err(CommandError::Internal));
            }
        }

        // 4. Every written effect record passes its producer's preflight before the first write
        // (#1264 D3). The classifier refuses every record a producer would, so a refusal here is
        // an internal fault. An EQ's parameter records are its owner's edits, never written: its
        // targets were preflighted above.
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let producer = &owners.effects[index];
            for &record in &instance.records {
                match record {
                    // Written on every instance, the EQ's too; the record carries the session's
                    // crossfade ramp (#1341 D1).
                    effect_contract::EffectControlRecord::Bypass(_) => {
                        if producer.preflight(record).is_err() {
                            return LiveCommit::Done(Err(CommandError::Internal));
                        }
                    }
                    effect_contract::EffectControlRecord::Parameter { .. } => {
                        if instance.targets.is_none() && producer.preflight(record).is_err() {
                            return LiveCommit::Done(Err(CommandError::Internal));
                        }
                    }
                    _ => return LiveCommit::Done(Err(CommandError::Internal)),
                }
            }
        }

        // 5. The protocol commit's own predicate.
        if self
            .controller
            .check_prepared_structural(prepared.get())
            .is_err()
        {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        #[cfg(feature = "test-support")]
        if take_test_fault_state(TestStructuralFaultPhase::BeforeLivePush) {
            drop(owners);
            drop(prepared);
            return LiveCommit::Done(Err(CommandError::Backpressure));
        }

        // 6. The stamp is the last fallible step (#1312 D7): the revision this transaction commits
        // as, from the gate of the newest epoch, whose producers the writes below target. A
        // refusal writes nothing and uses no revision. Every step from here to the revision's
        // publication cannot fail: a cell write has no room to want, every record passed its
        // preflight, and the commit's predicate held under this borrow.
        // A model transaction stamps its own revision (`(r + 1, 0)` in the gate's order); a
        // monitor-only one spends no revision and stamps a monitor publication under the committed
        // revision `r` (#1603 D7). A monitor-only transaction with no record takes no stamp and
        // publishes nothing. An exhausted publication number refuses, as `INTERNAL`.
        let revision = prepared.get().prospective_session().revision().0;
        let stamp = if !publishes {
            None
        } else {
            let stamped = if monitor_only {
                revision_gate.stamp_monitor(revision)
            } else {
                revision_gate.stamp(revision)
            };
            let Ok(stamp) = stamped else {
                drop(owners);
                drop(prepared);
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            Some(stamp)
        };
        #[cfg(feature = "test-support")]
        if in_place {
            IN_PLACE_COUNTS.with(|counts| {
                let (commits, materialized) = counts.get();
                counts.set((commits + 1, materialized));
            });
        }
        // The key is read into a local now, so the publication below is the stamped one.
        let key = stamp.map(|stamp| stamp.key());
        // Write the strips' cells, then each effect's cells and each EQ owner's target cells.
        // Nothing above changed anything but the owners' open candidates.
        if let Some(stamp) = stamp {
            for (strip, &index) in delta.strips.iter().zip(&resolved) {
                let producer = &mut strip_lanes[index];
                for record in strip.fader_records() {
                    producer.fader.write(stamp, record);
                }
                if let Some(record) = strip.matrix {
                    producer.producer.write(stamp, record);
                }
                if let Some(input) = producer.input.as_mut() {
                    for &record in &strip.input {
                        input.write(stamp, record);
                    }
                }
            }
            for (route, &index) in delta.routes.iter().zip(&resolved_routes) {
                route_lanes[index].write(stamp, route.record);
            }
            // #1639 D3: the new jump lengths, in the same block as the records.
            if let Some(lengths) = delta.jump_lengths {
                jump_lane.write(stamp, lengths);
            }
            for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
                let producer = &mut owners.effects[index];
                match &instance.targets {
                    None => {
                        for &record in &instance.records {
                            write_effect_record(producer, stamp, record);
                        }
                    }
                    Some(targets) => {
                        for &record in &instance.records {
                            if matches!(record, effect_contract::EffectControlRecord::Bypass(_)) {
                                write_effect_record(producer, stamp, record);
                            }
                        }
                        let base_revision = producer
                            .owner()
                            .map(|owner| owner.committed_revision())
                            .expect("an owner was begun");
                        producer
                            .publish_candidate_targets(stamp, base_revision, targets)
                            .expect("the owner's targets were preflighted");
                    }
                }
            }
        }
        // #1313 D2: the path is the classifier's result. A delta with no record of any kind is a
        // commit render has nothing to do for.
        // #1559 D5: a transaction with a model edit is `live` when the model part or the overlay
        // change produced a record, else `model_only`; the commit gives a token of monitor
        // writes only its own path.
        let path = if delta.is_empty() {
            protocol::EditPath::ModelOnly
        } else {
            protocol::EditPath::Live
        };
        drop(delta);
        let published = core::mem::take(&mut owners.begun);

        // 7. Commit, then each EQ owner; the catalog's values follow (#1264 D4, #1265 D3);
        // 8. respond.
        let committed = prepared
            .commit(&mut self.controller, path)
            .expect("the commit predicate was checked under this borrow");
        debug_assert_eq!(
            revision,
            self.controller.session().revision().0,
            "the revision stamped is the revision committed (#1312 D7)"
        );
        // #1559 D8: the overlay is written after the last fallible step, with the commit.
        self.monitor = monitor_change.overlay;
        for index in published {
            owners.effects[index]
                .commit_owner()
                .expect("the owner's targets were published");
        }
        drop(owners);
        // The catalog of the committed model replaces the current one (decision 17, #1552 D3): the
        // values are read back through the same pass a rebuild uses, so a derived row follows its
        // inputs, and the structure epoch advances if any descriptor changed (a `matrix_mode`
        // switch flips `active`). Nothing here allocates or fails.
        let previous_catalog = self.controller.provider().catalog_resources();
        self.controller
            .provider_mut()
            .publish_staged_catalog(catalog, revision);
        let committed_catalog = self.controller.provider().catalog_resources();
        // The newest epoch's rows follow the committed catalog (#1558 D6): it holds the catalog
        // once in its retained bytes and once in the prepared-protocol stand-in, which later
        // admissions charge, and `plan_resources` reports the retained bytes. An older epoch's row
        // keeps the catalog it was prepared with until its plan is gone. Nothing here fails.
        newest_capi.follow_catalog(previous_catalog, committed_catalog);
        {
            let mut reports = self
                .shared
                .reports
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some((_, row)) = reports.iter_mut().find(|(epoch, _)| epoch == newest_epoch) {
                let mut resources = PlanResources::from(*row);
                resources.control_retained_bytes = resources
                    .control_retained_bytes
                    .saturating_sub(previous_catalog.retained_bytes)
                    .saturating_add(committed_catalog.retained_bytes);
                resources.largest_named_allocation_bytes = resources
                    .largest_named_allocation_bytes
                    .max(committed_catalog.largest_allocation_bytes);
                *row = A::Row::from(resources);
            }
        }
        // 8. The revision, as the commit's last write (#1314 D2): published on the gate of the plan
        // that carries its content, the newest epoch's (#1502 D6), which is the routing the
        // producers were chosen by, so the cells a commit writes and the gate that reveals them
        // are one plan's. Every record push, cell write, target publication and owner commit is
        // before it, so render's `Acquire` snapshot of the gate sees them all and the watermark
        // never reports the revision before its values can apply. The stamp's revision is
        // published, read into a local, so the always-on bound of `stamp` covers the publication. A
        // monitor publication carries the committed revision (#1603).
        if let Some(key) = key {
            revision_gate.publish_key(key);
        }
        LiveCommit::Done(Ok(committed.write_into(&mut self.response_scratch).expect(
            "prepared response capacity was admitted before the live commit",
        )))
    }

    /// The live admission's inputs, read from the report table and the epochs (#1053 D8). It
    /// returns the catalog of the committed model, which it built to charge (#1558 D6).
    fn live_admission(
        &self,
        prepared: &ObservedPreparedToken,
        overlay: &controls::MonitorOverlay,
        view: controls::ModelView<'_>,
        delta: &host_core::LiveDelta<'_>,
    ) -> Result<host_core::StagedCatalog, CommandError> {
        let transaction = prepared.get().prospective_session();
        let committed = self.controller.session().compiled();
        let models = if transaction.is_in_place() {
            // #1613: a transaction prepared in place changes values and at most adds parameter
            // entries (#1620), never a count a rebuild checks (the preparation proved it), so those
            // counts are the committed model's, which passed them when it was admitted. The model
            // charge is the one the full path would make: the committed estimate and the next one,
            // which charges the added entries.
            let next = transaction
                .in_place_estimate()
                .unwrap_or_else(|| committed.resource_estimate());
            compiled_model_admission_between(committed.resource_estimate(), next)
                .map_err(CommandError::CompileRejected)?
        } else {
            // A live delta may add a VCA, so it meets the counts a rebuild would (#1558 D6): the
            // same function, the same diagnostic, and nothing is committed on a refusal.
            host_core::check_counts(view.structure(), &prepare_caps(self.limits)).map_err(
                |diagnostics| CommandError::CompileRejected(prepare_failure(diagnostics)),
            )?;
            compiled_model_admission(
                committed,
                transaction.compiled().ok_or(CommandError::Internal)?,
            )
            .map_err(CommandError::CompileRejected)?
        };
        // The new catalog, before anything is written. A VCA added live adds rows to it, so it
        // can outgrow the catalog the epochs were prepared with.
        let catalog = self
            .controller
            .provider()
            .stage_live_catalog(view, overlay, view.structure().sample_rate_hz, delta)
            .map_err(|_| CommandError::Internal)?;
        // A values-only stage holds no table: the catalog stays as large as it is, and only a
        // full catalog is held beside the current one until the commit publishes it.
        let current_catalog = self.controller.provider().catalog_resources();
        let catalog_admission = LiveCatalogAdmission::between(
            current_catalog,
            match &catalog {
                host_core::StagedCatalog::Values(_) => current_catalog,
                host_core::StagedCatalog::Full(full) => full.resources(),
            },
        );
        validate_live_peak(
            self.held_epochs()?.as_slice(),
            models,
            catalog_admission,
            self.limits,
        )
        .map_err(CommandError::CompileRejected)?;
        Ok(catalog)
    }

    /// The response bytes of the last successful command.
    pub fn command_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    /// Encodes the next event of `lane` into the event scratch, and returns its length, or `None`
    /// when the lane is empty.
    pub fn dequeue_event(
        &mut self,
        lane: EventLane,
        output_capacity: u64,
    ) -> Result<Option<usize>, EventError> {
        self.service().map_err(|error| match error {
            CommandError::Backpressure => EventError::Backpressure,
            _ => EventError::Internal,
        })?;
        let capacity = usize::try_from(output_capacity)
            .unwrap_or(usize::MAX)
            .min(self.response_scratch.len());
        let result = match lane {
            EventLane::Reliable => self.dequeue_reliable_event(capacity),
            EventLane::Lossy => self
                .controller
                .dequeue_lossy_event_frame_into(&mut self.response_scratch[..capacity]),
        };
        result.map_err(map_event_egress_error)
    }

    pub(crate) fn dequeue_reliable_event(
        &mut self,
        output_capacity: usize,
    ) -> Result<Option<usize>, EventEgressError> {
        if self.render_diagnostic_len == 0 {
            let result = self
                .controller
                .dequeue_reliable_event_frame_into(&mut self.response_scratch[..output_capacity]);
            self.protocol_reliable_pending = matches!(
                result,
                Err(EventEgressError::Encode(EncodeError::OutputTooSmall { .. }))
            );
            return result;
        }

        let head = self.render_diagnostic_head;
        if self.render_diagnostics[head].protocol_events_before != 0 {
            let result = self
                .controller
                .dequeue_reliable_event_frame_into(&mut self.response_scratch[..output_capacity]);
            match result {
                Ok(Some(bytes)) => {
                    self.render_diagnostics[head].protocol_events_before -= 1;
                    self.protocol_reliable_pending = false;
                    return Ok(Some(bytes));
                }
                Ok(None) => {
                    self.protocol_reliable_pending = false;
                    return Ok(None);
                }
                Err(error) => {
                    self.protocol_reliable_pending = matches!(
                        error,
                        EventEgressError::Encode(EncodeError::OutputTooSmall { .. })
                    );
                    return Err(error);
                }
            }
        }

        let slot = &mut self.render_diagnostics[head];
        let result = ProtocolCodec::default().encode_event_frame_into(
            &protocol::TypedEventFrame {
                revision: slot.revision,
                payload: protocol::EventPayload::Diagnostic(&slot.diagnostic),
            },
            &mut self.response_scratch[..output_capacity],
        );
        match result {
            Ok(bytes) => {
                drop(slot.reservation.take());
                slot.diagnostic.sample_time = None;
                slot.diagnostic.provider_sequence = None;
                slot.protocol_events_before = 0;
                slot.occupied = false;
                self.controller
                    .provider_mut()
                    .set_render_diagnostic(head, 0, 0, false);
                self.render_diagnostic_head =
                    (self.render_diagnostic_head + 1) % self.render_diagnostics.len();
                self.render_diagnostic_len -= 1;
                Ok(Some(bytes))
            }
            Err(error) => Err(EventEgressError::Encode(error)),
        }
    }

    /// The bytes of the last dequeued event.
    pub fn event_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    /// The producers of the newest committed session: the pending candidate's while one waits
    /// for its swap, else the current plan's (issue #1273 D3).
    pub(crate) fn newest_providers(&self) -> &ProviderEpoch {
        self.pending_providers.last().unwrap_or(&self.providers)
    }

    /// Sets the replacement peak ceiling ([`ControlLimits::maximum_replacement_peak_bytes`]) once
    /// the adapter knows its own fixed bytes exactly (#1290a D3): they are known only after the
    /// first plan is prepared, and the limits the session was built with are a plain value.
    pub fn set_replacement_peak_limit(&mut self, bytes: u64) {
        self.limits.maximum_replacement_peak_bytes = bytes;
    }

    /// The epoch of the newest plan (#1290a): the pending candidate's while one waits for its
    /// swap, else the current plan's. It advances by a published rebuild, so a host that compares
    /// it across a command knows whether the command published a replacement plan, and, against
    /// the epoch of the plan it renders, whether a replacement is still waiting for its block. It
    /// is the epoch of the plan whose producers [`Self::effect_producers`] lists.
    #[must_use]
    pub fn newest_plan_epoch(&self) -> u64 {
        self.newest_providers().epoch
    }

    fn newest_providers_mut(&mut self) -> &mut ProviderEpoch {
        self.pending_providers
            .last_mut()
            .unwrap_or(&mut self.providers)
    }

    /// The committed session in its compiled form (#1381a): what the plan was prepared from. A
    /// single-instance adapter reads its addressing tables and source shapes from it, so it keeps
    /// no second copy of the model beside the control plane's own.
    #[must_use]
    pub fn committed_session(&self) -> &CompiledSession {
        self.controller.session().compiled()
    }

    /// The structure epoch of the committed control list (#1562b): the number the `CONTROL_LIST_GET`
    /// pages carry. It advances when a commit changes an address, a header or a descriptor, a
    /// rebuild that adds or removes a strip included. An adapter that publishes a plan's strip
    /// order beside its meter frames stamps the plan with the epoch the commit left, so a client
    /// that holds a list of another epoch knows to read it again.
    #[must_use]
    pub fn structure_epoch(&self) -> u64 {
        self.controller.provider().structure_epoch()
    }

    /// What the session's one control catalog holds (#1381a): the control plane's provider owns it,
    /// and an adapter that charges its own report reads the figures here, not from a catalog of its
    /// own.
    #[must_use]
    pub fn catalog_resources(&self) -> host_core::CatalogResources {
        self.controller.provider().catalog_resources()
    }

    /// The newest plan's effect producers, in `HostLiveControlHandles::effect_controls` order
    /// (#1381a), for reading: each producer's identity and descriptor, and the free room of its
    /// observation queue. The adapter reaches an effect's observation queue for the browser's
    /// observe kinds 7 and 8 (ruling C9) through [`Self::admit_and_push_observation_batch`], and
    /// nothing else of a producer, so it cannot write a live value or open an owner behind the
    /// commit.
    ///
    /// These are the producers of the newest committed plan (#1290a, batch 10 review): the
    /// pending candidate's while one waits for its swap, else the current plan's. That is the plan
    /// a live value, a source feed and a seek already reach, so an observation record is admitted
    /// against the successor the way every other control call is. The adapter pairs them with the
    /// addressing tables of the plan whose epoch is [`Self::newest_plan_epoch`]. The browser's
    /// feature phase deletes this accessor together with those kinds.
    #[must_use]
    pub fn effect_producers(&self) -> &[host_core::EffectControlProducer] {
        &self.newest_providers().effects
    }

    /// Resolves the stable key of an observation tap on the newest committed plan (#1677): the
    /// plan [`Self::effect_producers`] lists, the pending candidate's while one waits for its swap.
    /// A subscribe or unsubscribe names its tap by this key and no position of any plan, so a
    /// rebuild between a client's last read and the record cannot make it arm another effect's
    /// tap. The browser's record calls it, and the future C ABI observe edit (#1584) must call it.
    ///
    /// The strip is looked up in the committed model, so a strip with no effect is still an
    /// unknown effect and not an unknown strip. The effect is the producer of the strip's console
    /// slot or insert of that ID. The comparison is by bytes and the call allocates nothing: the
    /// browser admits a record in the worklet's message handler, on the audio thread between
    /// render quanta.
    ///
    /// # Errors
    ///
    /// The first of [`ObservationKeyError`] that applies, in the order strip, effect, tap. Nothing
    /// is written.
    pub fn resolve_observation_key(
        &self,
        key: &ObservationKeyRef<'_>,
    ) -> Result<ResolvedObservationTap, ObservationKeyError> {
        let model = self.committed_session().normalized_model();
        let strip_known = model
            .tracks
            .binary_search_by(|track| track.id.as_str().as_bytes().cmp(key.strip_id))
            .is_ok()
            || model
                .submixes
                .binary_search_by(|submix| submix.id.as_str().as_bytes().cmp(key.strip_id))
                .is_ok();
        if !strip_known {
            return Err(ObservationKeyError::UnknownStrip);
        }
        let (producer, found) = self
            .newest_providers()
            .effects
            .iter()
            .enumerate()
            .find(|(_, producer)| {
                producer.address.rack == key.rack
                    && producer.track_id.as_bytes() == key.strip_id
                    && producer.effect_id.as_bytes() == key.effect_id
            })
            .ok_or(ObservationKeyError::UnknownEffect)?;
        Ok(ResolvedObservationTap {
            producer,
            tap_index: resolve_observation_tap(found.descriptor, key.tap_id)?,
        })
    }

    /// The encoded bytes of the one `OBSERVATION_ENDED` frame that would end every armed tap,
    /// pushes the table has not folded yet included, or `0` when none is armed (#1685 D1). A
    /// subscription is admitted only while this is at most [`Self::observation_frame_bytes`].
    #[must_use]
    pub fn observation_ended_bytes(&self) -> usize {
        self.observation_wire
            .event_bytes(self.observation_armed_bytes)
    }

    /// The control frame limit the ended event must fit (#1685 D1), the session's
    /// `maximum_control_frame_bytes`.
    #[must_use]
    pub fn observation_frame_bytes(&self) -> usize {
        self.observation_wire.frame()
    }

    /// How many taps are armed now, pushes the table has not folded yet included.
    #[must_use]
    pub fn observation_armed(&self) -> usize {
        self.observation_armed
    }

    /// Admits a batch of subscription changes, then queues it: the one way a host arms or disarms
    /// an observation tap (#1676 D1, attempt 2). A change names the producer's position in
    /// [`Self::effect_producers`] and the tap, as [`Self::resolve_observation_key`] returns them.
    ///
    /// Three checks run before anything is written, in this order, and a refusal names the first
    /// change in the iterator's order that breaks the first check that fails:
    ///
    /// 1. **A slot per tap, and one naming per tap.** A change whose producer has no observation
    ///    slot for its tap (a tap index past the menu, or a producer with no observation queue) is
    ///    [`ObservationRefusal::Unbound`]. A change that names a tap an earlier change of the
    ///    batch named is [`ObservationRefusal::Duplicate`], at the second naming. The frame check
    ///    below counts each tap once, so it is only sound for a batch that names each tap once.
    /// 2. **The frame.** The batch is judged by the state it leaves: arms of taps that are not
    ///    armed add their key's encoded bytes, disarms of armed taps remove theirs, and the rest
    ///    change nothing (#1685 D1). A batch that would leave armed taps whose one
    ///    `OBSERVATION_ENDED` event, every key with its real IDs, outgrows the control frame
    ///    ([`Self::observation_frame_bytes`]) is [`ObservationRefusal::Limit`], at the arm that
    ///    crosses after every removal of the batch is counted. A retry of the same batch never
    ///    succeeds until another tap is disarmed.
    /// 3. **The room.** Each producer's observation queue holds one record per declared tap until
    ///    a render drains it; a batch that needs more than the free room of a producer is
    ///    [`ObservationRefusal::Full`] (retryable after a render), and a position past the list is
    ///    [`ObservationRefusal::NoProducer`]. These two are one check: the first change in order
    ///    that names no producer or a producer without the room the batch needs is refused.
    ///
    /// Then every change is queued. A refusal writes nothing observable, so it is whole and
    /// precedes every acknowledgement. The call is linear in the batch, reads the per-slot state
    /// the pushes keep and per-producer scratch sized with the plan, and allocates nothing, so it
    /// runs in the browser worklet's message handler, on the audio thread between render quanta.
    ///
    /// # Errors
    ///
    /// [`ObservationRefusal`], as above. Nothing was queued and no state moved.
    ///
    /// # Panics
    ///
    /// Never in practice: the room check covers every push of the batch on the one control thread,
    /// so a push that is refused after it is an engine fault, which would break the whole-batch
    /// promise and is not returned as a typed refusal.
    pub fn admit_and_push_observation_batch(
        &mut self,
        changes: impl Iterator<Item = ObservationChange> + Clone,
    ) -> Result<(), ObservationRefusal> {
        let mark = self.next_observation_batch_mark();
        // Pass 1: a tap the producer has no slot for, the second naming of a tap, and each
        // producer's record count for pass 3. A position past the list is pass 3's `NoProducer`.
        for (index, change) in changes.clone().enumerate() {
            if change.position < self.newest_providers().effects.len()
                && self
                    .observation_slot(change.position, change.tap_index)
                    .is_none()
            {
                return Err(ObservationRefusal::Unbound { index });
            }
            if let Some(count) = self.observation_batch_counts.get_mut(change.position) {
                if count.mark != mark {
                    *count = ObservationBatchCount { mark, needed: 0 };
                }
                count.needed = count.needed.saturating_add(1);
            }
            if let Some(slot) = self.observation_slot(change.position, change.tap_index) {
                let entry = &mut self.observation_pending[slot];
                if entry.mark == mark {
                    return Err(ObservationRefusal::Duplicate { index });
                }
                entry.mark = mark;
            }
        }
        // Pass 2: the frame.
        self.admit_observation_changes(changes.clone())
            .map_err(ObservationRefusal::Limit)?;
        // Pass 3: the room, one count per producer from pass 1.
        let producers = &self.newest_providers().effects;
        for (index, change) in changes.clone().enumerate() {
            let Some(producer) = producers.get(change.position) else {
                return Err(ObservationRefusal::NoProducer { index });
            };
            let needed = self
                .observation_batch_counts
                .get(change.position)
                .filter(|count| count.mark == mark)
                .map_or(0, |count| count.needed as usize);
            if needed > producer.observe_available_capacity() {
                return Err(ObservationRefusal::Full { index });
            }
        }
        for change in changes {
            self.push_effect_observe(
                change.position,
                change.tap_index,
                change.armed,
                change.window_blocks,
            )
            .expect("the room check found the producer")
            .expect("the room check found room for every record of the batch");
        }
        debug_assert!(
            self.observation_wire.fits(self.observation_armed_bytes),
            "an admitted batch leaves an ended event of {} bytes past the frame of {}",
            self.observation_ended_bytes(),
            self.observation_frame_bytes()
        );
        Ok(())
    }

    /// A fresh stamp for the batch about to be admitted. A stamp of `0` is what new scratch
    /// holds, so it is never handed out; when the counter wraps, every mark is cleared first (a
    /// bounded pass over storage sized with the plan, which allocates nothing).
    fn next_observation_batch_mark(&mut self) -> u32 {
        self.observation_batch_mark = self.observation_batch_mark.wrapping_add(1);
        if self.observation_batch_mark == 0 {
            for slot in &mut self.observation_pending {
                slot.mark = 0;
            }
            for count in &mut self.observation_batch_counts {
                *count = ObservationBatchCount::default();
            }
            self.observation_batch_mark = 1;
        }
        self.observation_batch_mark
    }

    /// The index in [`Self::observation_pending`] of the tap at `tap_index` of the producer at
    /// `position`, if both name one.
    fn observation_slot(&self, position: usize, tap_index: u32) -> Option<usize> {
        let first = *self.observation_offsets.get(position)?;
        let end = *self.observation_offsets.get(position + 1)?;
        let slot = first.checked_add(tap_index).filter(|slot| *slot < end)?;
        Some(slot as usize).filter(|slot| *slot < self.observation_pending.len())
    }

    /// The frame check of [`Self::admit_and_push_observation_batch`] (#1685 D1): moves nothing and
    /// allocates nothing; each key's bytes are its slot's. Sound only for a batch that names each
    /// tap once, which the caller checks first.
    fn admit_observation_changes(
        &self,
        changes: impl Iterator<Item = ObservationChange> + Clone,
    ) -> Result<(), ObservationLimit> {
        let slot = |change: &ObservationChange| -> Option<&ObservationSlot> {
            self.observation_pending
                .get(self.observation_slot(change.position, change.tap_index)?)
        };
        let removed = changes
            .clone()
            .filter(|change| !change.armed)
            .filter_map(|change| slot(&change).filter(|slot| slot.armed))
            .map(|slot| usize::from(slot.key_bytes))
            .sum::<usize>();
        let mut bytes_after = self.observation_armed_bytes.saturating_sub(removed);
        for (index, change) in changes.enumerate() {
            if let Some(slot) = slot(&change).filter(|slot| change.armed && !slot.armed) {
                bytes_after = bytes_after.saturating_add(usize::from(slot.key_bytes));
                if !self.observation_wire.fits(bytes_after) {
                    return Err(ObservationLimit {
                        index,
                        event_bytes: self.observation_wire.event_bytes(bytes_after),
                        frame_bytes: self.observation_wire.frame(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Queues one observation record past every admission check, the over-frame defence's only
    /// way in (#1676 attempt 2): a test arms taps past the frame to prove that a commit whose
    /// ended event would not fit one frame is still refused.
    ///
    /// # Errors
    ///
    /// As the queue push: `None` for a position past the list, else the producer's refusal.
    #[cfg(feature = "test-support")]
    pub fn test_push_observe_unadmitted(
        &mut self,
        position: usize,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
    ) -> Option<Result<(), effect_contract::ObservePushError>> {
        self.push_effect_observe(position, tap_index, armed, window_blocks)
    }

    /// Queues one observation record on the newest plan's effect producer at `position` of
    /// [`Self::effect_producers`]. Private to the crate: a host reaches it only through
    /// [`Self::admit_and_push_observation_batch`], so no record is queued without the frame check
    /// (#1676 attempt 2).
    ///
    /// # Errors
    ///
    /// `None` for a position past the list, else the producer's refusal (no queue, or full).
    pub(crate) fn push_effect_observe(
        &mut self,
        position: usize,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
    ) -> Option<Result<(), effect_contract::ObservePushError>> {
        let producer = self
            .pending_providers
            .last_mut()
            .unwrap_or(&mut self.providers)
            .effects
            .get_mut(position)?;
        let pushed = producer.try_push_observe(tap_index, armed, window_blocks);
        if pushed.is_ok() {
            // #1584 D6a: the push is the acknowledgement, so the state every later successor is
            // prepared from is recorded with it, after the only fallible step and without
            // allocating: this runs on the browser's audio thread, between quanta. A record the
            // lane refused records nothing. The next service step folds it into the table.
            let slots = self
                .observation_offsets
                .get(position)
                .copied()
                .zip(self.observation_offsets.get(position + 1).copied());
            if let Some((first, end)) = slots
                && let Some(slot) = first.checked_add(tap_index).filter(|slot| *slot < end)
                && let Some(entry) = self.observation_pending.get_mut(slot as usize)
            {
                entry.pending = if armed {
                    PendingArm::Arm(window_blocks)
                } else {
                    PendingArm::Disarm
                };
                // The count and the bytes follow the slot's state (#1676, #1685): an arm of an
                // armed tap only changes its window, and a disarm of a tap that is not armed
                // removes nothing.
                let key_bytes = usize::from(entry.key_bytes);
                match (entry.armed, armed) {
                    (false, true) => {
                        self.observation_armed += 1;
                        self.observation_armed_bytes += key_bytes;
                    }
                    (true, false) => {
                        self.observation_armed -= 1;
                        self.observation_armed_bytes -= key_bytes;
                    }
                    _ => {}
                }
                entry.armed = armed;
            }
        }
        Some(pushed)
    }

    /// Folds what the host pushed to the observation slots into the table of armed taps, in slot
    /// order, and clears it. Control thread: it allocates for a new binding's key.
    pub(crate) fn fold_observation_pending(&mut self) {
        if self
            .observation_pending
            .iter()
            .all(|slot| slot.pending == PendingArm::None)
        {
            return;
        }
        let producers = &self
            .pending_providers
            .last()
            .unwrap_or(&self.providers)
            .effects;
        for (position, producer) in producers.iter().enumerate() {
            let (Some(&first), Some(&end)) = (
                self.observation_offsets.get(position),
                self.observation_offsets.get(position + 1),
            ) else {
                break;
            };
            for slot in first..end {
                let arm = core::mem::take(&mut self.observation_pending[slot as usize].pending);
                let Some(key) = tap_key_of(producer, slot - first) else {
                    continue;
                };
                match arm {
                    PendingArm::None => {}
                    PendingArm::Arm(window) => self.observation.set(key, window),
                    PendingArm::Disarm => {
                        self.observation.clear(&key);
                    }
                }
            }
        }
    }

    /// The designated master strip of the newest commit (#1683): the strip whose `master`
    /// monitor row is on, or `None`. Host-local, never saved; the boot word seeds it and a
    /// `SET_CONTROL` of the row sets or changes it. A host that reports a master reading resolves
    /// it by ID in each plan it holds.
    #[must_use]
    pub fn master_strip(&self) -> Option<&session::StableId> {
        self.monitor.master()
    }

    /// The taps the host has armed (#1584 D6a), for a test or an adapter that reports them.
    #[must_use]
    pub fn observation_bindings(&mut self) -> &controls::ObservationBindings {
        self.fold_observation_pending();
        &self.observation
    }

    /// Feeds a source of the newest committed session. A source that persists across a pending
    /// structural transaction is fed through its moved producer into the ring the running plan
    /// still renders and its successor carries (issue #1273 D3).
    ///
    /// Runs the service step first, so it is a control-thread call: [`Self::service`] takes the
    /// shared report lock, drops retired plans and may allocate. A call made while render is
    /// locked uses [`Self::submit_unserviced`].
    pub fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.submit_unserviced(id, submission)
    }

    /// [`Self::submit`] without the service step: the same routing to the newest committed
    /// session's sources, and nothing else.
    ///
    /// The browser's PCM feed drains its rings inside the AudioWorklet's `process()` and submits
    /// from there, where taking the report lock, dropping a retired plan and staging a counter
    /// snapshot are all forbidden (batch 7 review B1). The path allocates nothing and takes no
    /// lock. The service step still runs, in the host's message handler, before the next control
    /// call, so no state the step maintains is skipped, only deferred.
    pub fn submit_unserviced(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.newest_providers_mut()
            .sources
            .submit(id, submission)
            .map_err(SourceFailure::Control)
    }

    /// Queues a seek on a source of the newest committed session, after the service step.
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.seek_unserviced(id, generation, source_frame)
    }

    /// [`Self::seek`] without the service step; see [`Self::submit_unserviced`].
    pub fn seek_unserviced(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.newest_providers_mut()
            .sources
            .seek(id, generation, source_frame)
            .map_err(SourceFailure::Control)
    }

    /// Queues an anchored seek on a source of the newest committed session: `source_frame` enters
    /// the graph in the block that starts at absolute render sample `anchor_sample` (issue #1275).
    pub fn seek_at(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
        anchor_sample: u64,
    ) -> Result<(), SourceFailure> {
        self.service().map_err(|_| SourceFailure::Internal)?;
        self.newest_providers_mut()
            .sources
            .seek_at(id, generation, source_frame, anchor_sample)
            .map_err(SourceFailure::Control)
    }
}

/// A source submission or seek that the adapter's boundary must report.
///
/// `Control` carries the facade's typed rejection unchanged (audit F6: the boundary used to
/// collapse every one of the seventeen source failures to `RESULT_INVALID_ARGUMENT` with no
/// diagnostic); `Internal` is the control plane's own epoch-synchronisation failure. The adapter
/// maps each to its own result code and diagnostic (#1309 D6).
#[derive(Clone, Copy, Debug)]
pub enum SourceFailure {
    /// The facade rejected the submission or seek.
    Control(SourceControlError),
    /// The control plane could not synchronise its plan epochs.
    Internal,
}

#[cfg(test)]
mod tests {
    use super::EventError;

    /// #1570a D2: the three event refusal texts are the C ABI's `last_error` bytes, which hosts
    /// match on; a reworded text would silently change every adapter's diagnostic.
    #[test]
    fn event_refusals_spell_the_texts_every_host_reports() {
        assert_eq!(
            EventError::BufferTooSmall { required: 9 }.diagnostic(),
            b"control.event.output.too_small"
        );
        assert_eq!(
            EventError::Backpressure.diagnostic(),
            b"control.event.backpressure"
        );
        assert_eq!(EventError::Internal.diagnostic(), b"control.event.internal");
    }
}
