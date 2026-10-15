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
    /// bits)`.
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
    /// to be folded into [`Self::observation`] (#1584 D6a). The browser admits a subscription
    /// inside `process()`, where nothing may allocate, and a binding owns its key's text, so the
    /// push records the absolute state here, in storage sized when the plan is published, and the
    /// next service step folds it into the table. A rebuild folds first: it only starts from
    /// [`Self::command`], which services.
    pub(crate) observation_pending: Box<[PendingArm]>,
    /// For each position of the newest plan's effect producers, the first of its slots in
    /// `observation_pending`, and one entry past the last: a producer's slots are
    /// `offsets[position]..offsets[position + 1]`, one per declared tap in declaration order.
    pub(crate) observation_offsets: Box<[u32]>,
}

/// The pending-subscription storage for `producers`, the newest plan's: the first slot of each
/// producer (and one past the last), and one slot per declared tap (#1584 D6a). Called when a plan
/// is published, after the fold, so nothing pending is lost to a change of positions.
pub(crate) fn observation_slots(
    producers: &[host_core::EffectControlProducer],
) -> (Box<[u32]>, Box<[PendingArm]>) {
    let mut offsets = Vec::with_capacity(producers.len() + 1);
    let mut total = 0_u32;
    for producer in producers {
        offsets.push(total);
        total =
            total.saturating_add(u32::try_from(producer.observe_capacity()).unwrap_or(u32::MAX));
    }
    offsets.push(total);
    (
        offsets.into_boxed_slice(),
        vec![PendingArm::None; total as usize].into_boxed_slice(),
    )
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

    /// The resource row a structural candidate must fit beside (issue #1042).
    ///
    /// That is the newest plan, the one still live when the candidate would be published: the
    /// pending candidate's while it waits for its swap, else the current provider's. With nothing
    /// pending and the atomic caught up, this is the rendering plan's row, as before. Pairing a
    /// candidate with an older row would report a valid edit as a compile rejection whenever the
    /// two plans could never coexist.
    pub(crate) fn replacement_base_report(&self) -> Result<PlanResources, CommandError> {
        let newest = self
            .pending_providers
            .last()
            .map_or(self.providers.epoch, |provider| provider.epoch);
        self.shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?
            .iter()
            .find_map(|(epoch, report)| (*epoch == newest).then_some(PlanResources::from(*report)))
            .ok_or(CommandError::Internal)
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
                // #1562a D9: an adapter that cannot take a rebuild yet refuses here, with the
                // token dropped unsent (nothing is stamped, prepared or queued, so the frame has
                // no acknowledgement to drop). The C ABI's preparer takes every rebuild.
                if let Some(code) = self.preparer.refuse_rebuild() {
                    drop(prepared);
                    return Err(CommandError::CompileRejected(crate::error::diagnostic(
                        code,
                    )));
                }
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
                // Issue #1042: while the atomic lags the providers, a render call has swapped plans
                // but not yet published the epoch. The retired plan's row stays for any-thread
                // readers, so the report table is full and no candidate could be admitted: refuse
                // before compiling, with retryable backpressure rather than a wasted compile. The
                // live arm above adds no row, so it skips this (#1053 D7).
                if self.shared.active_epoch.load(Ordering::Acquire) < self.providers.epoch {
                    return Err(CommandError::Backpressure);
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
                let observation_change = self.observation.after(
                    prepared
                        .get()
                        .prospective_session()
                        .compiled()
                        .ok_or(CommandError::Internal)?
                        .normalized_model(),
                );
                if !observation_change.ended.is_empty() {
                    let ended = observation_change
                        .ended
                        .iter()
                        .map(ended_key)
                        .collect::<Vec<_>>();
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
                validate_replacement_peak(
                    self.replacement_base_report()?,
                    resources,
                    carried,
                    prospective_capi,
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
                if !self.pending_providers.is_empty() {
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
                        let returned = match error {
                            PlanReplacementReservationError::PublicationFull(plan)
                            | PlanReplacementReservationError::RetirementFull(plan)
                            | PlanReplacementReservationError::Incompatible(plan)
                            | PlanReplacementReservationError::EpochExhausted(plan) => plan,
                        };
                        drop(ObservedCandidatePlan::returned(returned));
                        return Err(CommandError::Backpressure);
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
                if reports.len() == reports.capacity()
                    || self.pending_providers.len() == self.pending_providers.capacity()
                {
                    return Err(CommandError::Backpressure);
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
                // #1584 D6a: the newest plan's slots. The fold ran when this command was
                // serviced and nothing has been pushed since (one thread), so none is lost.
                (self.observation_offsets, self.observation_pending) = observation_slots(
                    &self
                        .pending_providers
                        .last()
                        .unwrap_or(&self.providers)
                        .effects,
                );
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
    /// **A token prepared in place (#1613).** A transaction of `SetControl`s on existing values holds
    /// the owners it edited and no compilation (`PreparedSessionTransaction::is_in_place`). The
    /// classification, the admission, the catalog stage and the producers' preflight read the
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
            // #1613: a transaction prepared in place changes values, never a count (the
            // preparation proved it), so the counts a rebuild would check are those of the
            // committed model, which passed them when it was admitted. The model charge is the one
            // the full path would make: the committed estimate and the next one.
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
        let (current, pending) = {
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
            let current = row(&self.providers)?;
            let pending = self.pending_providers.last().map(row).transpose()?;
            (current, pending)
        };
        validate_live_peak(current, pending, models, catalog_admission, self.limits)
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
    /// observe kinds 7 and 8 (ruling C9) through [`Self::push_effect_observe`], and nothing
    /// else of a producer, so it cannot write a live value or open an owner behind the commit.
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

    /// Queues one observation record on the newest plan's effect producer at `position` of
    /// [`Self::effect_producers`]: the only write the adapter makes to a producer.
    ///
    /// # Errors
    ///
    /// `None` for a position past the list, else the producer's refusal (no queue, or full).
    pub fn push_effect_observe(
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
            // allocating: this runs inside the browser's `process()`. A record the lane refused
            // records nothing. The next service step folds it into the table.
            let slots = self
                .observation_offsets
                .get(position)
                .copied()
                .zip(self.observation_offsets.get(position + 1).copied());
            if let Some((first, end)) = slots
                && let Some(slot) = first.checked_add(tap_index).filter(|slot| *slot < end)
                && let Some(pending) = self.observation_pending.get_mut(slot as usize)
            {
                *pending = if armed {
                    PendingArm::Arm(window_blocks)
                } else {
                    PendingArm::Disarm
                };
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
            .all(|arm| *arm == PendingArm::None)
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
                let arm = core::mem::take(&mut self.observation_pending[slot as usize]);
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
