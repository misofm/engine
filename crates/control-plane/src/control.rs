//! The control-protocol session: commands, events, source control and plan replacement.

use super::*;

/// One plan's live fader/mute and matrix/pan producers (#1256 D2), kept with its provider epoch.
///
/// `controls` is in `HostLiveControlHandles::strips` order: the tracks, then the submixes. Each
/// producer's `input` is `None`, because the C ABI attaches no input lane. `commit_live` pushes a
/// value-only transaction's records through them (#1257). The rings are `Arc`s shared with the
/// plan's consumers, so they outlive whichever of the plan and the epoch drops first, and the last
/// owner to drop frees them, on the control thread: `synchronize_plan_epochs` drops a reclaimed
/// plan before its provider.
pub(crate) struct StripLanes {
    pub(crate) controls: Box<[host_core::TrackControlProducer]>,
    /// How many leading entries of `controls` are tracks.
    pub(crate) track_count: usize,
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
    /// parametric EQ's producer also owns its prepared-target owner. `commit_live` pushes a
    /// value-only transaction's effect parameter records through them (#1264). As for the strip
    /// producers, the rings are `Arc`s shared with the plan's lanes, so the epoch may drop after
    /// its plan, and does: `synchronize_plan_epochs` drops a reclaimed plan before its provider.
    pub(crate) effects: Box<[host_core::EffectControlProducer]>,
    /// The capi resources this epoch's plan was prepared with (#1257 D4).
    pub(crate) capi: CapiResources,
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
    ) -> Self {
        let owner = Self {
            epoch: 0,
            sources,
            inventory,
            strips,
            effects,
            capi,
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
    ) -> Self {
        let owner = Self {
            epoch: u64::MAX,
            sources,
            inventory,
            strips,
            effects,
            capi,
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
    /// The live arm, after every fallible check and before its first push (#1257 D7).
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
    /// The automation queue.
    pub automation: protocol::QueueReport,
    /// The telemetry queue.
    pub telemetry: protocol::QueueReport,
    /// The telemetry counters.
    pub telemetry_counters: protocol::TelemetryCounters,
    /// The controller's, provider's and replay cache's retained capacities.
    pub retained_capacities: [usize; 7],
    /// Every strip producer's free fader and matrix room, current epoch first, then each pending
    /// candidate: `(epoch, strip ID, fader room, matrix room)`.
    pub live_rooms: Vec<(u64, Box<str>, usize, usize)>,
    /// Every effect producer's free room, current epoch first, then each pending candidate:
    /// `(epoch, strip ID, effect instance ID, room)`.
    pub effect_rooms: Vec<(u64, Box<str>, Box<str>, usize)>,
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
    /// `u16` scratch the protocol decoder writes field offsets into. It is exactly
    /// `max_frame_bytes / 2` entries; the odd trailing byte of an odd frame limit used to buy a
    /// one-byte box that nothing ever read (audit F7).
    pub(crate) decode_fields: Box<[u16]>,
    pub(crate) response_scratch: Box<[u8]>,
    pub(crate) shared: Arc<SharedPlanState<A::Row>>,
    pub(crate) observed_render_sequence: u64,
    pub(crate) render_diagnostics: Box<[RenderDiagnosticSlot]>,
    pub(crate) render_diagnostic_head: usize,
    pub(crate) render_diagnostic_len: usize,
    pub(crate) protocol_reliable_pending: bool,
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

    pub(crate) fn commit(
        mut self,
        controller: &mut ObservedController,
    ) -> Result<protocol::CommittedCommandFrame, ()> {
        let prepared = self.inner.take().expect("observed token commits once");
        #[cfg(feature = "test-support")]
        let committed = if take_test_fault_state(TestStructuralFaultPhase::ProtocolCommit) {
            drop(prepared);
            Err(())
        } else {
            controller
                .commit_prepared_structural(*prepared)
                .map_err(|_| ())
        };
        #[cfg(not(feature = "test-support"))]
        let committed = controller.commit_prepared_structural(*prepared);
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
    /// A live lane of the newest plan has too little room for a value-only transaction's records
    /// (#1257 D8). capi reports it as `RESULT_BACKPRESSURE` with `control.live.backpressure`.
    LiveBackpressure,
    /// The prospective session does not prepare or is not admitted.
    CompileRejected(CompileFailure),
    /// The control plane's own bookkeeping failed.
    Internal,
}

/// What the live arm did with a structural token (#1257 D2).
pub(crate) enum LiveCommit {
    /// The delta was live: the arm committed it, or refused it with nothing changed.
    Done(Result<usize, CommandError>),
    /// The delta needs a plan rebuild; the token comes back untouched.
    Rebuild(ObservedPreparedToken),
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

pub(crate) fn map_encode_error(error: EncodeError) -> CommandError {
    match error {
        EncodeError::OutputTooSmall { required } => CommandError::BufferTooSmall {
            required: u64::try_from(required).unwrap_or(u64::MAX),
        },
        EncodeError::MessageKindMismatch | EncodeError::LimitExceeded => CommandError::Internal,
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

    pub(crate) fn collect_render_activity(&mut self) {
        self.refresh_render_peak_gate();
        let sequence = self.shared.render_sequence.load(Ordering::Acquire);
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
        let counter_ids = &self.controller.telemetry_configuration().counter_ids;
        if !counter_ids.is_empty() {
            let values = counter_ids
                .iter()
                .map(|&id| protocol::CounterValue {
                    id,
                    value: sequence,
                })
                .collect();
            let _ = self.controller.stage_counter_snapshot_event(
                revision,
                &protocol::CounterSnapshot {
                    observed_sample,
                    values,
                },
            );
        }
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
            automation: self
                .controller
                .queues()
                .report(protocol::QueueKind::Automation),
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
            live_rooms: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.strips.controls.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.fader.available_capacity(),
                            producer.producer.available_capacity(),
                        )
                    })
                })
                .collect(),
            effect_rooms: core::iter::once(&self.providers)
                .chain(&self.pending_providers)
                .flat_map(|provider| {
                    provider.effects.iter().map(|producer| {
                        (
                            provider.epoch,
                            producer.track_id.clone(),
                            producer.effect_id.clone(),
                            producer.producer().available_capacity(),
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

    /// Runs [`Self::synchronize_plan_epochs`].
    #[cfg(feature = "test-support")]
    pub fn test_synchronize_plan_epochs(&mut self) -> Result<(), CommandError> {
        self.synchronize_plan_epochs()
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

    /// Runs one control command frame. A success returns the response's length in the response
    /// scratch ([`Self::command_response`]); a structural command either rides the newest plan's
    /// live lanes or prepares, admits and publishes a replacement plan.
    pub fn command(&mut self, request: &[u8], output_capacity: u64) -> Result<usize, CommandError> {
        self.synchronize_plan_epochs()?;
        self.collect_render_activity();
        let telemetry_counters = self.controller.queues().telemetry_counters();
        self.controller
            .provider_mut()
            .set_telemetry_counters(telemetry_counters);
        let output_capacity = usize::try_from(output_capacity).unwrap_or(usize::MAX);
        let prepared = self
            .controller
            .prepare_command_frame(
                request,
                &mut DecodeScratch::new(&mut self.decode_fields),
                output_capacity,
            )
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
                let prepared = match self.commit_live(prepared) {
                    LiveCommit::Done(result) => return result,
                    LiveCommit::Rebuild(prepared) => prepared,
                };
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
                // Issue #1273 D1: the candidate succeeds the newest plan, so every source the
                // transaction leaves unchanged is prepared to carry that plan's ring.
                let prepared_runtime = prepare_runtime::<A>(
                    prepared.get().prospective_session().compiled(),
                    self.limits,
                    Some(SuccessorBase {
                        inventory: &self.newest_providers().inventory,
                        committed: self.controller.session().compiled().normalized_model(),
                    }),
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
                } = prepared_runtime;
                let mut candidate_provider =
                    ProviderEpoch::candidate(sources, inventory, strips, effects, prospective_capi);
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
                        prepared.get().prospective_session().compiled(),
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
                let reservation = match self.publisher.reserve_replacement(candidate_plan.take()) {
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

                let committed = prepared
                    .commit(&mut self.controller)
                    .map_err(|_| CommandError::Internal)?;
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
                    .replace_session_catalog(candidate_catalog);
                self.pending_providers.push(candidate_provider);
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
    /// the room check on every other queue the delta touches, every pushed effect record's
    /// preflight and every changed value's readback handle (#1264 D3, D4), and the protocol's own
    /// commit predicate. Each of them refuses before any queue or the model changes, and a refusal
    /// after an owner was begun discards every begun owner (`OpenOwners`). An effect instance
    /// whose records, or an EQ's targets, outnumber its queue's whole capacity could never fit,
    /// so the token goes back for a rebuild instead of an endless `BACKPRESSURE` (#1264 D3,
    /// #1265 D2). An instance's `Bypass` record (#1266 D2) is an ordinary pushed record on every
    /// instance, the EQ's included, where it rides with the targets (no order is promised to the
    /// host, under #1053 D2) and counts toward their room. Only then does it push every record,
    /// publish every EQ owner's targets, commit the token and commit every owner, and none of
    /// them can fail: the room was checked, every
    /// effect record passed its producer's preflight and every target prefix its owner's, the
    /// control thread is the only producer, a render pop only grows the room, and the predicate is
    /// the commit's own under this `&mut` borrow. So no acked edit is ever dropped, and no refused
    /// edit leaves a record, a target or an open owner behind.
    ///
    /// It never reserves a plan, adds a report row or replaces the provider catalog. After the
    /// commit it sets each live effect value in the catalog in place, so the parameter readback
    /// reports what a rebuild's catalog would (#1264 D4, #1265 D3).
    fn commit_live(&mut self, prepared: ObservedPreparedToken) -> LiveCommit {
        let next = prepared
            .get()
            .prospective_session()
            .compiled()
            .normalized_model();
        let Ok(delta) = host_core::classify_live_delta(
            self.controller.session().compiled().normalized_model(),
            next,
            host_core::LiveRamps::for_session(next),
        ) else {
            return LiveCommit::Rebuild(prepared);
        };

        // 1. Admission (#1053 D8). Its refusal is the compile diagnostic a rebuild would give.
        if let Err(error) = self.live_admission(&prepared) {
            return LiveCommit::Done(Err(error));
        }

        // 2. Resolve each strip's producer by ID in the newest epoch (#1053 D7). The delta and the
        // producer table are both in canonical track order, so the search resumes where the last
        // one stopped; it still finds a strip anywhere, since it wraps.
        let ProviderEpoch {
            strips, effects, ..
        } = match self.pending_providers.last_mut() {
            Some(provider) => provider,
            None => &mut self.providers,
        };
        let track_count = strips.track_count;
        let tracks = &mut strips.controls[..track_count];
        let mut resolved = Vec::new();
        if resolved.try_reserve_exact(delta.strips.len()).is_err() {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        let mut cursor = 0;
        for strip in &delta.strips {
            let found = (0..tracks.len())
                .map(|step| (cursor + step) % tracks.len())
                .find(|&index| &*tracks[index].track_id == strip.strip_id);
            let Some(index) = found else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            resolved.push(index);
            cursor = index + 1;
        }

        // The effect instances, by strip ID and live address (#1264 D3). An instance whose
        // queue records -- its records, or an EQ's targets (#1265 D2) -- outnumber its queue's
        // whole capacity could never fit: rebuild instead.
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
            let Some(index) = effects.iter().position(|producer| {
                &*producer.track_id == instance.strip_id && producer.address == instance.address
            }) else {
                return LiveCommit::Done(Err(CommandError::Internal));
            };
            let producer = &effects[index];
            let queued = match &instance.targets {
                // An EQ's `Bypass` record (#1266 D2) is pushed ahead of its targets.
                Some(targets) if producer.has_owner() => {
                    bypass_records(&instance.records) + targets.len()
                }
                Some(_) => return LiveCommit::Done(Err(CommandError::Internal)),
                None => instance.records.len(),
            };
            if queued > producer.capacity() {
                drop(delta);
                return LiveCommit::Rebuild(prepared);
            }
            resolved_effects.push(index);
        }

        // 3. Each EQ owner's transaction (#1265 D2): begin at its committed revision, apply the
        // edits, and preflight the designed targets (revision, validation and complete queue
        // room). From here on, every refusal discards every owner begun, as `OpenOwners` drops.
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
            match producer.preflight_candidate_targets(base_revision, targets) {
                Ok(()) => {}
                Err(host_core::EffectControlOwnerError::Capacity) => {
                    return LiveCommit::Done(Err(CommandError::LiveBackpressure));
                }
                Err(_) => return LiveCommit::Done(Err(CommandError::Internal)),
            }
        }

        // 4. Room on every other queue the delta touches. Each strip and each effect instance
        // appears once in the delta, so a queue's need is its own record count.
        for (strip, &index) in delta.strips.iter().zip(&resolved) {
            let producer = &tracks[index];
            if producer.fader.available_capacity() < strip.fader_records().count()
                || producer.producer.available_capacity() < usize::from(strip.matrix.is_some())
            {
                return LiveCommit::Done(Err(CommandError::LiveBackpressure));
            }
        }
        // An EQ owner's preflight checked room for its targets alone; its `Bypass` record is
        // pushed ahead of them, so their sum must fit (#1266 D2).
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let pushed = match &instance.targets {
                None => instance.records.len(),
                Some(targets) => bypass_records(&instance.records) + targets.len(),
            };
            if owners.effects[index].producer().available_capacity() < pushed {
                return LiveCommit::Done(Err(CommandError::LiveBackpressure));
            }
        }

        // Every pushed effect record passes its producer's preflight, and each changed value's
        // readback row exists, before the first push (#1264 D3, D4). The classifier refuses
        // every record a producer would, so a refusal here is an internal fault. An EQ's records
        // are its owner's edits, never pushed: its targets were preflighted above.
        let record_count = delta
            .effects
            .iter()
            .map(|instance| instance.records.len())
            .sum();
        let mut readback = Vec::new();
        if readback.try_reserve_exact(record_count).is_err() {
            return LiveCommit::Done(Err(CommandError::Internal));
        }
        let provider = self.controller.provider_mut();
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let producer = &owners.effects[index];
            let rack = match instance.address.rack {
                host_core::LiveEffectRack::Console => protocol::ParameterRack::Console,
                host_core::LiveEffectRack::Inserts => protocol::ParameterRack::Inserts,
            };
            for &record in &instance.records {
                let (parameter_index, channel, value) = match record {
                    effect_contract::EffectControlRecord::Parameter {
                        parameter_index,
                        channel,
                        value,
                    } => (parameter_index, channel, value),
                    // Pushed on every instance, the EQ's too; no parameter row reads it.
                    effect_contract::EffectControlRecord::Bypass(_) => {
                        if producer.preflight(record).is_err() {
                            return LiveCommit::Done(Err(CommandError::Internal));
                        }
                        continue;
                    }
                    _ => return LiveCommit::Done(Err(CommandError::Internal)),
                };
                if instance.targets.is_none() && producer.preflight(record).is_err() {
                    return LiveCommit::Done(Err(CommandError::Internal));
                }
                let Some(parameter) = producer.descriptor.parameters.get(parameter_index as usize)
                else {
                    return LiveCommit::Done(Err(CommandError::Internal));
                };
                let channel = match channel {
                    effect_contract::ParameterChannel::Left => protocol::ParameterChannel::Left,
                    effect_contract::ParameterChannel::Right => protocol::ParameterChannel::Right,
                    effect_contract::ParameterChannel::Both => protocol::ParameterChannel::Both,
                };
                let Some(handle) = provider.parameter_handle(
                    instance.strip_id,
                    rack,
                    &producer.effect_id,
                    parameter.id.0,
                    channel,
                ) else {
                    return LiveCommit::Done(Err(CommandError::Internal));
                };
                readback.push((handle, value));
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

        // 6. Push, then publish each EQ owner's targets. Nothing above changed anything but the
        // owners' open candidates; nothing below can fail.
        for (strip, &index) in delta.strips.iter().zip(&resolved) {
            let producer = &mut tracks[index];
            for record in strip.fader_records() {
                producer
                    .fader
                    .try_push(record)
                    .unwrap_or_else(|_| unreachable!("the fader queue's room was checked"));
            }
            if let Some(record) = strip.matrix {
                producer
                    .producer
                    .try_push(record)
                    .unwrap_or_else(|_| unreachable!("the matrix queue's room was checked"));
            }
        }
        for (instance, &index) in delta.effects.iter().zip(&resolved_effects) {
            let producer = &mut owners.effects[index];
            match &instance.targets {
                None => {
                    for &record in &instance.records {
                        producer.try_push(record).unwrap_or_else(|_| {
                            unreachable!("the effect queue's room and the preflight were checked")
                        });
                    }
                }
                Some(targets) => {
                    for &record in &instance.records {
                        if matches!(record, effect_contract::EffectControlRecord::Bypass(_)) {
                            producer.try_push(record).unwrap_or_else(|_| {
                                unreachable!(
                                    "the effect queue's room and the preflight were checked"
                                )
                            });
                        }
                    }
                    let base_revision = producer
                        .owner()
                        .map(|owner| owner.committed_revision())
                        .expect("an owner was begun");
                    producer
                        .publish_candidate_targets(base_revision, targets)
                        .expect("the owner's targets were preflighted");
                }
            }
        }
        drop(delta);
        let published = core::mem::take(&mut owners.begun);

        // 7. Commit, then each EQ owner; the readback follows every live effect value (#1264 D4,
        // #1265 D3); 8. respond.
        let committed = prepared
            .commit(&mut self.controller)
            .expect("the commit predicate was checked under this borrow");
        for index in published {
            owners.effects[index]
                .commit_owner()
                .expect("the owner's targets were published");
        }
        drop(owners);
        let provider = self.controller.provider_mut();
        for (handle, value) in readback {
            provider.set_parameter_value(handle, value);
        }
        LiveCommit::Done(Ok(committed.write_into(&mut self.response_scratch).expect(
            "prepared response capacity was admitted before the live commit",
        )))
    }

    /// The live admission's inputs, read from the report table and the epochs (#1053 D8).
    fn live_admission(&self, prepared: &ObservedPreparedToken) -> Result<(), CommandError> {
        let models = compiled_model_admission(
            self.controller.session().compiled(),
            prepared.get().prospective_session().compiled(),
        )
        .map_err(CommandError::CompileRejected)?;
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
        validate_live_peak(current, pending, models, self.limits)
            .map_err(CommandError::CompileRejected)
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
        self.synchronize_plan_epochs()
            .map_err(|error| match error {
                CommandError::Backpressure => EventError::Backpressure,
                _ => EventError::Internal,
            })?;
        self.collect_render_activity();
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

    fn newest_providers_mut(&mut self) -> &mut ProviderEpoch {
        self.pending_providers
            .last_mut()
            .unwrap_or(&mut self.providers)
    }

    /// Feeds a source of the newest committed session. A source that persists across a pending
    /// structural transaction is fed through its moved producer into the ring the running plan
    /// still renders and its successor carries (issue #1273 D3).
    pub fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.synchronize_plan_epochs()
            .map_err(|_| SourceFailure::Internal)?;
        self.newest_providers_mut()
            .sources
            .submit(id, submission)
            .map_err(SourceFailure::Control)
    }

    /// Queues a seek on a source of the newest committed session.
    pub fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.synchronize_plan_epochs()
            .map_err(|_| SourceFailure::Internal)?;
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
        self.synchronize_plan_epochs()
            .map_err(|_| SourceFailure::Internal)?;
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

/// How many of an instance's records are `Bypass` records: none or one (#1266 D2).
fn bypass_records(records: &[effect_contract::EffectControlRecord]) -> usize {
    records
        .iter()
        .filter(|record| matches!(record, effect_contract::EffectControlRecord::Bypass(_)))
        .count()
}
