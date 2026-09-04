//! The control-protocol session: commands, events, source control and plan replacement.

use super::*;

/// One epoch's worth of host-owned source producers.
///
/// The table itself lives in `host-core`; this wrapper adds only the epoch tag and the
/// lifecycle counters the structural-replacement tests observe.
pub(crate) struct ProviderEpoch {
    pub(crate) epoch: u64,
    pub(crate) sources: SourceControlSet,
}

impl ProviderEpoch {
    pub(crate) fn current(sources: SourceControlSet) -> Self {
        let owner = Self { epoch: 0, sources };
        #[cfg(test)]
        update_test_owners(|owners| owners.current_provider_constructed += 1);
        owner
    }

    pub(crate) fn candidate(sources: SourceControlSet) -> Self {
        let owner = Self {
            epoch: u64::MAX,
            sources,
        };
        #[cfg(test)]
        update_test_owners(|owners| owners.candidate_provider_constructed += 1);
        owner
    }
}

impl Drop for ProviderEpoch {
    fn drop(&mut self) {
        #[cfg(test)]
        update_test_owners(|owners| {
            if self.epoch == 0 {
                owners.current_provider_disposed += 1;
            } else {
                owners.candidate_provider_disposed += 1;
            }
        });
    }
}

/// Structural plans own independent source rings; buffered host state never crosses an epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StructuralSourceStatePolicy {
    ResetAtReplacementBoundary,
}

pub(crate) const STRUCTURAL_SOURCE_STATE_POLICY: StructuralSourceStatePolicy =
    StructuralSourceStatePolicy::ResetAtReplacementBoundary;
pub(crate) const RENDER_DIAGNOSTIC_SLOTS: usize = 2;
pub(crate) const RENDER_DIAGNOSTIC_CODE: &str = "capi.render.activity";

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TestStructuralFaultPhase {
    AfterProtocolPrepare,
    AfterResourceProjection,
    AfterRuntimePrepare,
    AfterAdmission,
    AfterPlanReservation,
    BeforeProtocolCommit,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct TestOwnerCounters {
    pub(crate) current_provider_constructed: u64,
    pub(crate) current_provider_disposed: u64,
    pub(crate) candidate_provider_constructed: u64,
    pub(crate) candidate_provider_disposed: u64,
    pub(crate) candidate_provider_published: u64,
    pub(crate) current_plan_constructed: u64,
    pub(crate) current_plan_disposed: u64,
    pub(crate) candidate_plan_constructed: u64,
    pub(crate) candidate_plan_disposed: u64,
    pub(crate) candidate_plan_published: u64,
    pub(crate) token_constructed: u64,
    pub(crate) token_disposed: u64,
    pub(crate) replay_current_constructed: u64,
    pub(crate) replay_current_disposed: u64,
    pub(crate) replay_candidate_constructed: u64,
    pub(crate) replay_candidate_disposed: u64,
    pub(crate) replay_candidate_published: u64,
    pub(crate) reservation_constructed: u64,
    pub(crate) reservation_canceled: u64,
    pub(crate) reservation_committed: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TestTransactionSnapshot {
    pub(crate) revision: u64,
    pub(crate) canonical: Vec<u8>,
    pub(crate) model: session::ResourceEstimate,
    pub(crate) replay_entries: usize,
    pub(crate) provider_epoch: u64,
    pub(crate) pending_provider_epochs: Vec<u64>,
    pub(crate) retired_provider_epochs: Vec<u64>,
    pub(crate) active_plan_epoch: u64,
    pub(crate) resource_rows: Vec<(u64, PlanResourceReport)>,
    pub(crate) reliable_event: protocol::QueueReport,
    pub(crate) reliable_response: protocol::QueueReport,
    pub(crate) automation: protocol::QueueReport,
    pub(crate) telemetry: protocol::QueueReport,
    pub(crate) telemetry_counters: protocol::TelemetryCounters,
    pub(crate) retained_capacities: [usize; 7],
}

#[cfg(test)]
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

#[cfg(test)]
pub(crate) fn update_test_owners(update: impl FnOnce(&mut TestOwnerCounters)) {
    TEST_OWNER_STATE.with(|state| {
        let mut value = state.get();
        update(&mut value);
        state.set(value);
    });
}

#[cfg(test)]
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

#[cfg(test)]
pub(crate) fn test_reset_lifecycle_observer() {
    TEST_FAULT_STATE.with(|state| state.set(([None; 2], 0)));
    TEST_OWNER_STATE.with(|state| state.set(TestOwnerCounters::default()));
}

#[cfg(test)]
pub(crate) fn test_lifecycle_counters() -> TestOwnerCounters {
    TEST_OWNER_STATE.with(core::cell::Cell::get)
}

pub(crate) struct SessionState {
    pub(crate) controller: ObservedController,
    pub(crate) providers: ProviderEpoch,
    pub(crate) pending_providers: Vec<ProviderEpoch>,
    pub(crate) retired_providers: Vec<ProviderEpoch>,
    pub(crate) publisher: PlanPublisher,
    pub(crate) retirer: PlanRetirer,
    pub(crate) limits: CompileLimits,
    /// `u16` scratch the protocol decoder writes field offsets into. It is exactly
    /// `max_frame_bytes / 2` entries; the odd trailing byte of an odd frame limit used to buy a
    /// one-byte box that nothing ever read (audit F7).
    pub(crate) decode_fields: Box<[u16]>,
    pub(crate) response_scratch: Box<[u8]>,
    pub(crate) shared: Arc<SharedPlanState>,
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
        #[cfg(test)]
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
        #[cfg(test)]
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
            .map_err(|_| failure("capi.resource.allocation"))?;
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
        .map_err(|_| failure("capi.resource.allocation"))?;
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
        #[cfg(test)]
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
        let committed = controller.commit_prepared_structural(*prepared);
        #[cfg(test)]
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
            #[cfg(test)]
            update_test_owners(|owners| {
                owners.token_disposed += 1;
                owners.replay_candidate_disposed += 1;
            });
        }
    }
}

#[derive(Debug)]
pub(crate) enum CommandError {
    Invalid,
    BufferTooSmall { required: u64 },
    Backpressure,
    CompileRejected(CompileFailure),
    Internal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EventLane {
    Reliable,
    Lossy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EventError {
    BufferTooSmall { required: u64 },
    Backpressure,
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

impl SessionState {
    #[cfg(test)]
    pub(crate) fn test_set_structural_faults(
        &mut self,
        faults: [Option<TestStructuralFaultPhase>; 2],
    ) {
        let _ = self;
        TEST_FAULT_STATE.with(|state| state.set((faults, 0)));
    }

    #[cfg(test)]
    pub(crate) fn test_owner_counters(&self) -> TestOwnerCounters {
        let _ = self;
        TEST_OWNER_STATE.with(core::cell::Cell::get)
    }

    #[cfg(test)]
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
        let counter_ids = self
            .controller
            .telemetry_configuration()
            .counter_ids
            .clone();
        if !counter_ids.is_empty() {
            let values = counter_ids
                .into_iter()
                .map(|id| protocol::CounterValue {
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

    #[cfg(test)]
    pub(crate) fn test_state_summary(&self) -> (u64, usize, u64, usize) {
        (
            self.controller.session().revision().0,
            self.controller.replay().len(),
            self.providers.epoch,
            self.pending_providers.len(),
        )
    }

    #[cfg(test)]
    pub(crate) fn test_transaction_snapshot(&self) -> TestTransactionSnapshot {
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
        }
    }

    #[cfg(test)]
    pub(crate) fn test_telemetry_counters(&self) -> protocol::TelemetryCounters {
        self.controller.queues().telemetry_counters()
    }

    #[cfg(test)]
    pub(crate) fn test_retained_capacities(&self) -> [usize; 7] {
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

    pub(crate) fn active_resource_report(&self) -> Result<PlanResourceReport, CommandError> {
        let active = self.shared.active_epoch.load(Ordering::Acquire);
        self.shared
            .reports
            .lock()
            .map_err(|_| CommandError::Internal)?
            .iter()
            .find_map(|(epoch, report)| (*epoch == active).then_some(*report))
            .ok_or(CommandError::Internal)
    }

    pub(crate) fn synchronize_plan_epochs(&mut self) -> Result<(), CommandError> {
        let active_epoch = self.shared.active_epoch.load(Ordering::Acquire);
        if active_epoch != self.providers.epoch {
            let index = self
                .pending_providers
                .iter()
                .position(|provider| provider.epoch == active_epoch)
                .ok_or(CommandError::Internal)?;
            let next = self.pending_providers.remove(index);
            let previous = core::mem::replace(&mut self.providers, next);
            if self.retired_providers.len() == self.retired_providers.capacity() {
                return Err(CommandError::Internal);
            }
            self.retired_providers.push(previous);
        }

        while let Ok((retired_epoch, retired_plan)) = self.retirer.try_reclaim() {
            drop(ObservedRetiredPlan::new(retired_plan));
            if let Some(index) = self
                .retired_providers
                .iter()
                .position(|provider| provider.epoch == retired_epoch.0)
            {
                self.retired_providers.remove(index);
            } else {
                return Err(CommandError::Internal);
            }
            let active = self.shared.active_epoch.load(Ordering::Acquire);
            let mut reports = self
                .shared
                .reports
                .lock()
                .map_err(|_| CommandError::Internal)?;
            if retired_epoch.0 != active
                && let Some(index) = reports
                    .iter()
                    .position(|(epoch, _)| *epoch == retired_epoch.0)
            {
                reports.remove(index);
            }
        }
        Ok(())
    }

    pub(crate) fn command(
        &mut self,
        request: &[u8],
        output_capacity: u64,
    ) -> Result<usize, CommandError> {
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
                #[cfg(test)]
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
                let prepared_runtime = match STRUCTURAL_SOURCE_STATE_POLICY {
                    StructuralSourceStatePolicy::ResetAtReplacementBoundary => prepare_runtime(
                        prepared.get().prospective_session().compiled(),
                        self.limits,
                    ),
                }
                .map_err(CommandError::CompileRejected)?;
                let PreparedRuntime {
                    sources,
                    plan: candidate_plan,
                    resources,
                    control_catalog: candidate_catalog,
                    capi: prospective_capi,
                } = prepared_runtime;
                #[cfg(test)]
                if self.take_test_fault(TestStructuralFaultPhase::AfterResourceProjection) {
                    drop(candidate_plan);
                    drop(candidate_catalog);
                    drop(sources);
                    drop(prepared);
                    return Err(CommandError::Backpressure);
                }
                let mut candidate_provider = ProviderEpoch::candidate(sources);
                let candidate_plan = ObservedCandidatePlan::new(candidate_plan);
                #[cfg(test)]
                {
                    if self.take_test_fault(TestStructuralFaultPhase::AfterRuntimePrepare) {
                        drop(candidate_plan);
                        drop(candidate_provider);
                        drop(prepared);
                        return Err(CommandError::Backpressure);
                    }
                }
                validate_replacement_peak(
                    self.active_resource_report()?,
                    resources,
                    prospective_capi,
                    compiled_model_admission(
                        self.controller.session().compiled(),
                        prepared.get().prospective_session().compiled(),
                    )
                    .map_err(CommandError::CompileRejected)?,
                    self.limits,
                )
                .map_err(CommandError::CompileRejected)?;
                #[cfg(test)]
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
                #[cfg(test)]
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

                #[cfg(test)]
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
                self.controller
                    .provider_mut()
                    .replace_session_catalog(candidate_catalog);
                self.pending_providers.push(candidate_provider);
                reports.push((epoch, resources));
                reservation.commit();
                #[cfg(test)]
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

    pub(crate) fn command_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    pub(crate) fn dequeue_event(
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

    pub(crate) fn event_response(&self, bytes: usize) -> &[u8] {
        &self.response_scratch[..bytes]
    }

    pub(crate) fn submit(
        &mut self,
        id: &[u8],
        submission: SourceSubmission<'_>,
    ) -> Result<source::SubmitReport, SourceFailure> {
        self.synchronize_plan_epochs()
            .map_err(|_| SourceFailure::Internal)?;
        self.providers
            .sources
            .submit(id, submission)
            .map_err(SourceFailure::Control)
    }

    pub(crate) fn seek(
        &mut self,
        id: &[u8],
        generation: u64,
        source_frame: u64,
    ) -> Result<(), SourceFailure> {
        self.synchronize_plan_epochs()
            .map_err(|_| SourceFailure::Internal)?;
        self.providers
            .sources
            .seek(id, generation, source_frame)
            .map_err(SourceFailure::Control)
    }
}

/// A source submission or seek that the C boundary must report.
///
/// `Control` carries the facade's typed rejection unchanged (audit F6: the boundary used to
/// collapse every one of the seventeen source failures to `RESULT_INVALID_ARGUMENT` with no
/// diagnostic); `Internal` is capi's own epoch-synchronisation failure.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SourceFailure {
    /// The facade rejected the submission or seek.
    Control(SourceControlError),
    /// capi could not synchronise its plan epochs.
    Internal,
}

impl SourceFailure {
    /// The result code and diagnostic text this failure reports across the C boundary.
    pub(crate) fn report(self) -> (u32, &'static [u8]) {
        match self {
            Self::Internal => (RESULT_INTERNAL, b"capi.source.epoch"),
            Self::Control(error) => {
                let code = if error.is_backpressure() {
                    RESULT_BACKPRESSURE
                } else if error.is_internal() {
                    RESULT_INTERNAL
                } else {
                    RESULT_INVALID_ARGUMENT
                };
                (code, error.diagnostic().as_bytes())
            }
        }
    }
}
