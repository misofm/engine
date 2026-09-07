//! Prepared, typed delivery of bounded fader and matrix batches to a live host console.
//!
//! The endpoint owns the host preparation transaction and keeps the raw builtin producers inside
//! the render owner. Control publishes one fixed batch through a generic delivery core; the render
//! owner claims at most one batch, copies its complete contents into the already prepared builtin
//! queues, and only finishes the ticket after the graph has rendered the block.

use core::num::NonZeroUsize;

use builtins_compiler::{TrackControlProducer, TrackControlRecord, TrackFaderRecord};
use engine::realtime::{
    Producer, QueueGeneration, RenderError, bounded_spsc, bounded_spsc_retained_payload,
};
use protocol::{
    CoreCancelComplete, CoreCancelToken, CoreCompletion, CoreTerminalDisposition, CoreTicket,
    DeliveryCoreControl, DeliveryCoreRender, DeliveryError, DeliveryResourceReport,
    PreparedDelivery, SampleTime, SessionRevision,
};

use crate::diagnostics::PrepareDiagnostics;
use crate::prepare::{
    HostConsoleRequest, HostPrepareCaps, HostPrepareReport, PreparedHost,
    prepare_host_runtime_between_render_calls_with_backend,
};
use crate::render_session::StartedRenderSession;
use crate::source::SourceControlSet;
#[cfg(test)]
use graph_compiler::Backend;

/// The maximum number of records in one prepared builtin batch.
pub const BUILTIN_BATCH_MAX_RECORDS: usize = 256;

/// One addressed live fader or matrix operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltinBatchRecord {
    /// Retarget one track's fader or mute stage.
    Fader {
        /// Canonical normalized track index from the prepared host.
        track_index: u32,
        /// The existing typed fader/mute record.
        record: TrackFaderRecord,
    },
    /// Retarget one track's matrix stage.
    Matrix {
        /// Canonical normalized track index from the prepared host.
        track_index: u32,
        /// The existing typed matrix record.
        record: TrackControlRecord,
    },
}

/// A fixed, copyable batch admitted by [`BuiltinBatchControl`].
///
/// Slots after `record_count` are ignored and remain `None`. The fixed backing array makes the
/// render claim bounded and keeps publication independent of the caller's slice lifetime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinBatch {
    /// Exact prepared session revision.
    pub revision: SessionRevision,
    /// Requested absolute first sample of the block.
    pub requested_sample: SampleTime,
    record_count: u16,
    records: [Option<BuiltinBatchRecord>; BUILTIN_BATCH_MAX_RECORDS],
}

/// Batch construction failure, retaining no borrowed input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinBatchBuildError {
    /// A batch must contain at least one record and at most 256 records.
    RecordCount,
}

impl BuiltinBatch {
    /// Build a fixed batch by copying the supplied records.
    pub fn new(
        revision: SessionRevision,
        requested_sample: SampleTime,
        records: &[BuiltinBatchRecord],
    ) -> Result<Self, BuiltinBatchBuildError> {
        if records.is_empty() || records.len() > BUILTIN_BATCH_MAX_RECORDS {
            return Err(BuiltinBatchBuildError::RecordCount);
        }
        let mut slots = [None; BUILTIN_BATCH_MAX_RECORDS];
        for (slot, record) in slots.iter_mut().zip(records.iter().copied()) {
            *slot = Some(record);
        }
        Ok(Self {
            revision,
            requested_sample,
            record_count: records.len() as u16,
            records: slots,
        })
    }

    /// Number of addressed records.
    #[must_use]
    pub const fn record_count(self) -> u16 {
        self.record_count
    }

    /// Borrow the initialized record prefix.
    pub fn records(&self) -> impl Iterator<Item = &BuiltinBatchRecord> {
        self.records[..usize::from(self.record_count)]
            .iter()
            .filter_map(Option::as_ref)
    }
}

/// Why control-side batch admission refused an owned batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinBatchAdmissionReason {
    /// The revision differs from the prepared session.
    WrongRevision,
    /// The requested sample is not aligned to the prepared quantum.
    MisalignedSample,
    /// A track index is outside the prepared normalized track set.
    TrackIndex,
    /// A fader value or matrix coefficient is outside its declared finite domain.
    InvalidValue,
    /// The requested sample cannot contain one complete prepared quantum.
    SampleOverflow,
}

/// Typed admission refusal. The original fixed batch is returned unchanged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltinBatchAdmissionError {
    /// Endpoint envelope or record validation failed.
    Invalid {
        /// The caller-owned batch that was not published.
        batch: BuiltinBatch,
        /// The first deterministic validation reason.
        reason: BuiltinBatchAdmissionReason,
    },
    /// The bounded generic delivery service refused publication.
    Delivery {
        /// The caller-owned batch that was not published.
        batch: BuiltinBatch,
        /// The bounded delivery refusal.
        error: DeliveryError,
    },
}

/// An applied batch's requested/actual sample association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinBatchApplication {
    /// Delivery ticket associated with this application.
    pub ticket: CoreTicket,
    /// Requested batch sample.
    pub requested_sample: SampleTime,
    /// First render boundary that consumed the batch.
    pub actual_sample: SampleTime,
    /// Whether the requested sample was already behind the boundary.
    pub late: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BuiltinBatchOutcome {
    ticket: CoreTicket,
    requested_sample: SampleTime,
    actual_sample: SampleTime,
}

/// One fully reconciled endpoint ticket.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BuiltinBatchCompletion {
    /// The generic ticket.
    pub ticket: CoreTicket,
    /// The complete retained batch.
    pub batch: BuiltinBatch,
    /// Applied or canceled terminal disposition.
    pub disposition: CoreTerminalDisposition,
    /// Number of records injected before the terminal state.
    pub applied_prefix: u16,
    /// Number of records that remained unapplied.
    pub remaining_count: u16,
    /// Requested sample from the batch header.
    pub requested_sample: SampleTime,
    /// Actual application sample for Applied, or None for Canceled.
    pub actual_sample: Option<SampleTime>,
    /// Applied batches whose requested sample was late.
    pub late: bool,
    /// Cancellation boundary sample for canceled batches.
    pub acknowledged_sample: Option<SampleTime>,
}

/// The endpoint's retained resource projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinBatchResources {
    /// Unchanged host preparation report, including actual live-console queues.
    pub host: HostPrepareReport,
    /// Generic typed delivery storage.
    pub delivery: DeliveryResourceReport,
    /// Endpoint outcome queue storage.
    pub outcome: DeliveryResourceReport,
    /// Retained bytes reported by existing host preparation.
    pub host_builtin_retained_payload_bytes: u64,
    /// Retained heap bytes added by the generic delivery and endpoint outcome queues.
    pub endpoint_retained_heap_bytes: u64,
    /// Checked host-plus-endpoint retained heap composition.
    pub composed_retained_heap_bytes: u64,
    /// Largest endpoint-owned retained heap allocation.
    pub largest_endpoint_heap_allocation_bytes: u64,
    /// Largest retained heap allocation reported by the unchanged host preparation.
    pub largest_host_engine_allocation_bytes: u64,
    /// Largest actual heap allocation across host and endpoint rows.
    pub largest_composed_heap_allocation_bytes: u64,
    /// Prepared endpoint owner bytes held inline by the control/render owners.
    pub inline_owner_bytes: u64,
    /// Largest inline owner size, reported separately from heap allocations.
    pub largest_inline_owner_bytes: u64,
    /// Inline control owner size.
    pub control_inline_bytes: usize,
    /// Inline prepared render wrapper size before FP/thread attestation.
    pub prepared_render_inline_bytes: usize,
    /// Inline started render wrapper size, including the plan and sticky fault field.
    pub started_render_inline_bytes: usize,
}

/// Endpoint construction failure before any owner is published.
#[derive(Debug)]
pub enum BuiltinBatchPrepareError {
    /// Existing host preparation rejected the session.
    Host(PrepareDiagnostics),
    /// Generic or outcome queue preparation failed.
    Delivery(protocol::ProtocolQueueError),
    /// The endpoint's composed retained report exceeded the host's builtin cap.
    ResourceLimit,
}

/// Control-side owner of typed batch admission, cancellation and terminal collection.
pub struct BuiltinBatchControl {
    delivery: DeliveryCoreControl<BuiltinBatch>,
    outcomes: engine::realtime::Consumer<BuiltinBatchOutcome>,
    revision: SessionRevision,
    quantum: u32,
    track_count: u32,
    sources: SourceControlSet,
    #[cfg(test)]
    meters: Vec<builtins_compiler::MeterConsumer>,
    report: HostPrepareReport,
    outstanding: usize,
    staged_outcome: Option<BuiltinBatchOutcome>,
    cancellation_started: bool,
    cancel_complete: Option<CoreCancelComplete>,
    cancel_token: Option<CoreCancelToken>,
    cancel_completion_reported: bool,
    last_collected: Option<CoreTicket>,
}

impl BuiltinBatchControl {
    /// Validate the complete batch and publish it atomically into the bounded delivery core.
    #[allow(clippy::result_large_err)]
    pub fn try_publish(
        &mut self,
        batch: BuiltinBatch,
    ) -> Result<CoreTicket, BuiltinBatchAdmissionError> {
        if self.cancellation_started || self.cancel_complete.is_some() {
            return Err(BuiltinBatchAdmissionError::Delivery {
                batch,
                error: DeliveryError::CancellationPending,
            });
        }
        if batch.revision != self.revision {
            return Err(BuiltinBatchAdmissionError::Invalid {
                batch,
                reason: BuiltinBatchAdmissionReason::WrongRevision,
            });
        }
        if !batch
            .requested_sample
            .0
            .is_multiple_of(u64::from(self.quantum))
        {
            return Err(BuiltinBatchAdmissionError::Invalid {
                batch,
                reason: BuiltinBatchAdmissionReason::MisalignedSample,
            });
        }
        if batch
            .requested_sample
            .0
            .checked_add(u64::from(self.quantum))
            .is_none()
        {
            return Err(BuiltinBatchAdmissionError::Invalid {
                batch,
                reason: BuiltinBatchAdmissionReason::SampleOverflow,
            });
        }
        for record in batch.records() {
            let (track_index, valid) = match *record {
                BuiltinBatchRecord::Fader {
                    track_index,
                    record,
                } => (track_index, valid_fader(record)),
                BuiltinBatchRecord::Matrix {
                    track_index,
                    record,
                } => (track_index, valid_matrix(record)),
            };
            if track_index >= self.track_count {
                return Err(BuiltinBatchAdmissionError::Invalid {
                    batch,
                    reason: BuiltinBatchAdmissionReason::TrackIndex,
                });
            }
            if !valid {
                return Err(BuiltinBatchAdmissionError::Invalid {
                    batch,
                    reason: BuiltinBatchAdmissionReason::InvalidValue,
                });
            }
        }
        let ticket = self
            .delivery
            .try_publish(batch, batch.record_count)
            .map_err(|error| BuiltinBatchAdmissionError::Delivery { batch, error })?;
        self.outstanding = self.outstanding.saturating_add(1);
        Ok(ticket)
    }

    /// Begin ordered cancellation of all accepted batches.
    pub fn begin_cancel(&mut self) -> Result<CoreCancelToken, DeliveryError> {
        if self.cancellation_started || self.cancel_complete.is_some() {
            return Err(DeliveryError::CancellationPending);
        }
        let token = self.delivery.begin_cancel()?;
        self.cancellation_started = true;
        self.cancel_complete = None;
        self.cancel_token = Some(token);
        self.cancel_completion_reported = false;
        Ok(token)
    }

    /// Poll cancellation until the render boundary has acknowledged and all terminals are ready.
    pub fn poll_cancel_boundary(
        &mut self,
        token: CoreCancelToken,
    ) -> Result<Option<CoreCancelComplete>, DeliveryError> {
        if self.cancel_token != Some(token) {
            return Err(DeliveryError::StaleTicket);
        }
        if let Some(complete) = self.cancel_complete {
            if self.outstanding != 0 {
                return Ok(None);
            }
            return self.finalize_cancel_completion(complete);
        }
        let complete = self.delivery.poll_cancel_boundary(token)?;
        if let Some(complete) = complete {
            self.cancel_complete = Some(complete);
            if self.outstanding == 0 {
                return self.finalize_cancel_completion(complete);
            }
        }
        Ok(None)
    }

    fn finalize_cancel_completion(
        &mut self,
        complete: CoreCancelComplete,
    ) -> Result<Option<CoreCancelComplete>, DeliveryError> {
        if self.cancel_completion_reported {
            return Err(DeliveryError::StaleTicket);
        }
        self.cancel_completion_reported = true;
        self.cancellation_started = false;
        self.cancel_complete = None;
        self.cancel_token = None;
        Ok(Some(complete))
    }

    /// Reconcile one terminal and release its generic credit only after outcome reconciliation.
    pub fn collect(&mut self, ticket: CoreTicket) -> Result<BuiltinBatchCompletion, DeliveryError> {
        // Applied endpoint metadata is staged and checked before generic collection can clear
        // the ledger slot.  A mismatch therefore leaves both queues and the ticket credit intact.
        if self.staged_outcome.is_none() {
            match self.outcomes.try_pop() {
                Ok(outcome) if outcome.ticket == ticket => self.staged_outcome = Some(outcome),
                Ok(outcome) => {
                    self.staged_outcome = Some(outcome);
                    return Err(DeliveryError::StaleTicket);
                }
                Err(_) if !self.cancellation_started || self.cancel_complete.is_none() => {
                    return Err(if self.last_collected == Some(ticket) {
                        DeliveryError::StaleTicket
                    } else {
                        DeliveryError::Empty
                    });
                }
                Err(_) => {}
            }
        } else if self
            .staged_outcome
            .is_some_and(|outcome| outcome.ticket != ticket)
        {
            return Err(DeliveryError::StaleTicket);
        }
        let completion = self.delivery.collect(ticket)?;
        let outcome = if completion.disposition == CoreTerminalDisposition::Applied {
            Some(self.staged_outcome.take().ok_or(DeliveryError::Empty)?)
        } else {
            None
        };
        self.outstanding = self.outstanding.saturating_sub(1);
        self.last_collected = Some(ticket);
        Ok(completion_from_core(completion, outcome))
    }

    /// Number of accepted tickets still awaiting collection.
    #[must_use]
    pub fn outstanding(&self) -> usize {
        self.outstanding
    }

    /// Borrow the existing host source control set retained by this endpoint.
    pub fn sources(&mut self) -> &mut SourceControlSet {
        &mut self.sources
    }

    /// Address-free host preparation report.
    #[must_use]
    pub const fn report(&self) -> HostPrepareReport {
        self.report
    }
}

/// Transferable endpoint render owner before thread-affine FP attestation.
pub struct PreparedBuiltinBatchRender {
    delivery: DeliveryCoreRender<BuiltinBatch>,
    outcomes: Producer<BuiltinBatchOutcome>,
    track_controls: Vec<TrackControlProducer>,
    pending: Option<(CoreTicket, BuiltinBatch)>,
    quantum: u32,
    output_channels: usize,
}

/// Thread-affine render owner with private raw builtin producers.
///
/// The started owner is deliberately neither `Send` nor `Sync`, because it owns the
/// thread-attested render session. These compile-fail checks keep that boundary explicit:
///
/// ```compile_fail
/// fn requires_send<T: Send>() {}
/// requires_send::<host_core::StartedBuiltinBatchRender>();
/// ```
///
/// ```compile_fail
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<host_core::StartedBuiltinBatchRender>();
/// ```
pub struct StartedBuiltinBatchRender {
    plan: StartedRenderSession,
    delivery: DeliveryCoreRender<BuiltinBatch>,
    outcomes: Producer<BuiltinBatchOutcome>,
    track_controls: Vec<TrackControlProducer>,
    pending: Option<(CoreTicket, BuiltinBatch)>,
    quantum: u32,
    output_channels: usize,
    fault: Option<BuiltinBatchRenderError>,
    #[cfg(test)]
    fail_after_graph: bool,
    #[cfg(test)]
    claim_hold_sender: Option<std::sync::mpsc::SyncSender<()>>,
    #[cfg(test)]
    claim_release_receiver: Option<std::sync::mpsc::Receiver<()>>,
}

/// Result of one healthy endpoint render boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinBatchRenderReport {
    /// The underlying graph report when a PCM block was rendered.
    pub graph: Option<engine::realtime::RenderReport>,
    /// True when this was a cancellation-only boundary and the graph was untouched.
    pub cancellation_only: bool,
    /// The ticket claimed and applied at this boundary, if any.
    pub applied: Option<BuiltinBatchApplication>,
}

/// Sticky endpoint render failure. Once present, no later block injects or advances time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltinBatchRenderError {
    /// The output envelope is not the prepared two-channel quantum.
    InvalidShape,
    /// The endpoint was called at a sample other than its next required sample.
    DiscontinuousTime {
        /// Next required contiguous sample.
        expected: u64,
    },
    /// A generic delivery operation failed after ownership had been claimed.
    Delivery(DeliveryError),
    /// A raw prepared builtin queue could not accept the complete claimed batch.
    QueueOverflow,
    /// The graph rejected the prepared output envelope.
    Graph(RenderError),
    /// A prior valid render encountered an irreversible application uncertainty.
    Fault,
}

impl StartedBuiltinBatchRender {
    #[cfg(test)]
    #[allow(dead_code)]
    fn inject_post_graph_fault_for_test(&mut self) {
        self.fail_after_graph = true;
    }

    #[cfg(test)]
    fn hold_after_claim_for_test(
        &mut self,
        sender: std::sync::mpsc::SyncSender<()>,
        receiver: std::sync::mpsc::Receiver<()>,
    ) {
        self.claim_hold_sender = Some(sender);
        self.claim_release_receiver = Some(receiver);
    }

    /// Render one prepared planar block and service cancellation before any builtin injection.
    pub fn render(
        &mut self,
        samples: &mut [f32],
        channels: usize,
        frames: usize,
        plane_stride: usize,
        first: SampleTime,
    ) -> Result<BuiltinBatchRenderReport, BuiltinBatchRenderError> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        let expected = self.plan.next_absolute_sample();
        if first.0 != expected {
            return Err(BuiltinBatchRenderError::DiscontinuousTime { expected });
        }
        if channels != self.output_channels
            || frames != self.quantum as usize
            || plane_stride < frames
            || channels
                .checked_sub(1)
                .and_then(|last| last.checked_mul(plane_stride))
                .and_then(|offset| offset.checked_add(frames))
                .is_none_or(|required| required > samples.len())
        {
            return Err(BuiltinBatchRenderError::InvalidShape);
        }
        if first.0.checked_add(u64::from(self.quantum)).is_none() {
            return Err(BuiltinBatchRenderError::Graph(RenderError::TimeOverflow));
        }
        match self.delivery.cancel_boundary(first) {
            Ok(()) => {
                self.pending = None;
                return Ok(BuiltinBatchRenderReport {
                    graph: None,
                    cancellation_only: true,
                    applied: None,
                });
            }
            Err(DeliveryError::Empty) => {}
            Err(error) => return self.sticky(BuiltinBatchRenderError::Delivery(error)),
        }
        if self.pending.is_none() {
            match self.delivery.begin() {
                Ok((ticket, batch)) => {
                    self.pending = Some((ticket, batch));
                    #[cfg(test)]
                    if let Some(sender) = self.claim_hold_sender.take() {
                        let _ = sender.send(());
                        if let Some(receiver) = self.claim_release_receiver.take()
                            && receiver.recv().is_err()
                        {
                            return self.sticky(BuiltinBatchRenderError::Fault);
                        }
                    }
                }
                Err(DeliveryError::Empty) => {}
                Err(error) => return self.sticky(BuiltinBatchRenderError::Delivery(error)),
            }
        }
        let apply = self
            .pending
            .filter(|(_, batch)| batch.requested_sample.0 <= first.0);
        if let Some((ticket, batch)) = apply
            && let Err(error) = self.inject(ticket, batch)
        {
            return self.sticky(error);
        }
        let graph = match self
            .plan
            .render_planar(samples, channels, frames, plane_stride, first.0)
        {
            Ok(report) => report,
            Err(error) => return self.sticky(BuiltinBatchRenderError::Graph(error)),
        };
        #[cfg(test)]
        if self.fail_after_graph {
            // Private mutation seam: graph execution has happened, but no endpoint terminal or
            // outcome may be published after this point.
            return self.sticky(BuiltinBatchRenderError::Fault);
        }
        let applied = if let Some((ticket, batch)) = apply {
            if let Err(error) = self.delivery.mark_progress(ticket, batch.record_count) {
                return self.sticky(BuiltinBatchRenderError::Delivery(error));
            }
            if self
                .outcomes
                .try_push(BuiltinBatchOutcome {
                    ticket,
                    requested_sample: batch.requested_sample,
                    actual_sample: first,
                })
                .is_err()
            {
                return self.sticky(BuiltinBatchRenderError::QueueOverflow);
            }
            if let Err(error) = self.delivery.finish(ticket, batch.record_count) {
                return self.sticky(BuiltinBatchRenderError::Delivery(error));
            }
            self.pending = None;
            Some(BuiltinBatchApplication {
                ticket,
                requested_sample: batch.requested_sample,
                actual_sample: first,
                late: batch.requested_sample.0 < first.0,
            })
        } else {
            None
        };
        Ok(BuiltinBatchRenderReport {
            graph: Some(graph),
            cancellation_only: false,
            applied,
        })
    }

    fn sticky<T>(&mut self, error: BuiltinBatchRenderError) -> Result<T, BuiltinBatchRenderError> {
        self.fault = Some(error);
        Err(error)
    }

    fn inject(
        &mut self,
        _ticket: CoreTicket,
        batch: BuiltinBatch,
    ) -> Result<(), BuiltinBatchRenderError> {
        for record in batch.records() {
            let (track_index, push) = match *record {
                BuiltinBatchRecord::Fader {
                    track_index,
                    record,
                } => (track_index, RawPush::Fader(record)),
                BuiltinBatchRecord::Matrix {
                    track_index,
                    record,
                } => (track_index, RawPush::Matrix(record)),
            };
            let Some(control) = self.track_controls.get_mut(track_index as usize) else {
                return Err(BuiltinBatchRenderError::Fault);
            };
            match push {
                RawPush::Fader(record) => control
                    .fader
                    .try_push(record)
                    .map_err(|_| BuiltinBatchRenderError::QueueOverflow)?,
                RawPush::Matrix(record) => control
                    .producer
                    .try_push(record)
                    .map_err(|_| BuiltinBatchRenderError::QueueOverflow)?,
            }
        }
        Ok(())
    }

    /// Return all render storage for control-thread teardown after render quiescence.
    pub fn stop(self) -> StoppedBuiltinBatchRender {
        let Self {
            plan,
            delivery,
            outcomes,
            track_controls,
            pending,
            quantum: _,
            output_channels: _,
            fault,
            #[cfg(test)]
                fail_after_graph: _,
            #[cfg(test)]
                claim_hold_sender: _,
            #[cfg(test)]
                claim_release_receiver: _,
        } = self;
        StoppedBuiltinBatchRender {
            plan: plan.stop(),
            delivery,
            outcomes,
            track_controls,
            pending,
            fault,
        }
    }
}

/// Render storage returned by [`StartedBuiltinBatchRender::stop`]. Drop this on a control thread.
#[allow(
    dead_code,
    reason = "fields are intentionally retained together for off-render teardown"
)]
pub struct StoppedBuiltinBatchRender {
    plan: engine::realtime::PreparedRenderPlan,
    delivery: DeliveryCoreRender<BuiltinBatch>,
    outcomes: Producer<BuiltinBatchOutcome>,
    track_controls: Vec<TrackControlProducer>,
    pending: Option<(CoreTicket, BuiltinBatch)>,
    fault: Option<BuiltinBatchRenderError>,
}

enum RawPush {
    Fader(TrackFaderRecord),
    Matrix(TrackControlRecord),
}

fn outcome_resource_report(
    capacity: NonZeroUsize,
) -> Result<DeliveryResourceReport, BuiltinBatchPrepareError> {
    let layout = bounded_spsc_retained_payload::<BuiltinBatchOutcome>(capacity).map_err(|_| {
        BuiltinBatchPrepareError::Delivery(protocol::ProtocolQueueError::CapacityOverflow)
    })?;
    let retained_payload_bytes = layout
        .total_bytes()
        .ok_or(BuiltinBatchPrepareError::ResourceLimit)
        .and_then(|bytes| {
            u64::try_from(bytes).map_err(|_| BuiltinBatchPrepareError::ResourceLimit)
        })?;
    let largest_allocation_bytes = u64::try_from(layout.largest_allocation_bytes())
        .map_err(|_| BuiltinBatchPrepareError::ResourceLimit)?;
    Ok(DeliveryResourceReport {
        retained_payload_bytes,
        largest_allocation_bytes,
    })
}

struct PreparedEndpointQueues {
    control: DeliveryCoreControl<BuiltinBatch>,
    render: DeliveryCoreRender<BuiltinBatch>,
    outcomes: engine::realtime::Consumer<BuiltinBatchOutcome>,
    outcome_producer: Producer<BuiltinBatchOutcome>,
}

fn endpoint_queue_reports(
    ticket_capacity: NonZeroUsize,
) -> Result<(DeliveryResourceReport, DeliveryResourceReport), BuiltinBatchPrepareError> {
    let delivery_report = PreparedDelivery::<BuiltinBatch>::resource_report(ticket_capacity)
        .map_err(BuiltinBatchPrepareError::Delivery)?;
    let outcome_report = outcome_resource_report(ticket_capacity)?;
    Ok((delivery_report, outcome_report))
}

fn prepare_endpoint_queues(
    ticket_capacity: NonZeroUsize,
    delivery_report: DeliveryResourceReport,
    outcome_report: DeliveryResourceReport,
) -> Result<PreparedEndpointQueues, BuiltinBatchPrepareError> {
    let _preflight_reports = (delivery_report, outcome_report);
    let (control, render) = PreparedDelivery::<BuiltinBatch>::prepare(ticket_capacity)
        .map_err(BuiltinBatchPrepareError::Delivery)?;
    let (outcome_producer, outcomes) =
        bounded_spsc(ticket_capacity, QueueGeneration(17)).map_err(|_| {
            BuiltinBatchPrepareError::Delivery(protocol::ProtocolQueueError::CapacityOverflow)
        })?;
    Ok(PreparedEndpointQueues {
        control,
        render,
        outcomes,
        outcome_producer,
    })
}

/// Prepare the host, console channels, typed delivery core and outcome storage as one transaction.
pub fn prepare_builtin_batch_endpoint(
    compiled: &session::CompiledSession,
    caps: &HostPrepareCaps,
    revision: SessionRevision,
    ticket_capacity: NonZeroUsize,
) -> Result<PreparedBuiltinBatchEndpoint, BuiltinBatchPrepareError> {
    prepare_builtin_batch_endpoint_with_backend(
        compiled,
        caps,
        revision,
        ticket_capacity,
        graph_compiler::Backend::current(),
        false,
    )
}

#[cfg(test)]
fn prepare_builtin_batch_endpoint_for_test(
    compiled: &session::CompiledSession,
    caps: &HostPrepareCaps,
    revision: SessionRevision,
    ticket_capacity: NonZeroUsize,
    backend: Backend,
) -> Result<PreparedBuiltinBatchEndpoint, BuiltinBatchPrepareError> {
    prepare_builtin_batch_endpoint_for_test_with_meters(
        compiled,
        caps,
        revision,
        ticket_capacity,
        backend,
        true,
    )
}

#[cfg(test)]
fn prepare_builtin_batch_endpoint_for_test_with_meters(
    compiled: &session::CompiledSession,
    caps: &HostPrepareCaps,
    revision: SessionRevision,
    ticket_capacity: NonZeroUsize,
    backend: Backend,
    test_meters: bool,
) -> Result<PreparedBuiltinBatchEndpoint, BuiltinBatchPrepareError> {
    prepare_builtin_batch_endpoint_with_backend(
        compiled,
        caps,
        revision,
        ticket_capacity,
        backend,
        test_meters,
    )
}

fn prepare_builtin_batch_endpoint_with_backend(
    compiled: &session::CompiledSession,
    caps: &HostPrepareCaps,
    revision: SessionRevision,
    ticket_capacity: NonZeroUsize,
    backend: graph_compiler::Backend,
    test_meters: bool,
) -> Result<PreparedBuiltinBatchEndpoint, BuiltinBatchPrepareError> {
    // Project endpoint storage before asking host preparation to allocate anything.  This makes
    // the endpoint's own aggregate and largest-allocation caps fail atomically.
    let (delivery_report, outcome_report) = endpoint_queue_reports(ticket_capacity)?;
    let endpoint_retained_bytes = delivery_report
        .retained_payload_bytes
        .checked_add(outcome_report.retained_payload_bytes)
        .ok_or(BuiltinBatchPrepareError::ResourceLimit)?;
    let control_inline_bytes = u64::try_from(core::mem::size_of::<BuiltinBatchControl>())
        .map_err(|_| BuiltinBatchPrepareError::ResourceLimit)?;
    let prepared_render_inline_bytes =
        u64::try_from(core::mem::size_of::<PreparedBuiltinBatchRender>())
            .map_err(|_| BuiltinBatchPrepareError::ResourceLimit)?;
    let started_render_inline_bytes =
        u64::try_from(core::mem::size_of::<StartedBuiltinBatchRender>())
            .map_err(|_| BuiltinBatchPrepareError::ResourceLimit)?;
    let inline_owner_bytes = control_inline_bytes
        .checked_add(started_render_inline_bytes)
        .ok_or(BuiltinBatchPrepareError::ResourceLimit)?;
    let endpoint_largest = delivery_report
        .largest_allocation_bytes
        .max(outcome_report.largest_allocation_bytes);
    let largest_inline_owner = control_inline_bytes
        .max(prepared_render_inline_bytes)
        .max(started_render_inline_bytes);
    if endpoint_retained_bytes > caps.maximum_builtin_retained_bytes
        || endpoint_largest > caps.maximum_named_allocation_bytes
    {
        return Err(BuiltinBatchPrepareError::ResourceLimit);
    }
    let console = HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(BUILTIN_BATCH_MAX_RECORDS).unwrap()),
        ..HostConsoleRequest::default()
    };
    #[cfg(test)]
    let console = if test_meters {
        HostConsoleRequest {
            meter_period_frames: Some(core::num::NonZeroU32::new(compiled.quantum().0).unwrap()),
            meter_queue_depth: NonZeroUsize::new(2).unwrap(),
            meter_tap: builtins::MeterTap::PostFader,
            ..console
        }
    } else {
        console
    };
    let _ = test_meters;
    let host_result =
        prepare_host_runtime_between_render_calls_with_backend(compiled, caps, &console, backend);
    let (host, handles) = host_result.map_err(BuiltinBatchPrepareError::Host)?;
    let host_report = host.report;
    let queues = prepare_endpoint_queues(ticket_capacity, delivery_report, outcome_report)?;
    let PreparedEndpointQueues {
        control: delivery,
        render: render_delivery,
        outcomes: outcome_consumer,
        outcome_producer,
    } = queues;
    let retained_bytes = host_report
        .builtin_retained_payload_bytes
        .checked_add(endpoint_retained_bytes)
        .ok_or(BuiltinBatchPrepareError::ResourceLimit)?;
    let largest = host_report
        .largest_engine_allocation_bytes
        .max(endpoint_largest);
    if retained_bytes > caps.maximum_builtin_retained_bytes
        || largest > caps.maximum_named_allocation_bytes
    {
        return Err(BuiltinBatchPrepareError::ResourceLimit);
    }
    let track_count = u32::try_from(compiled.normalized_model().tracks.len())
        .map_err(|_| BuiltinBatchPrepareError::ResourceLimit)?;
    Ok(PreparedBuiltinBatchEndpoint {
        control: delivery,
        outcomes: outcome_consumer,
        #[cfg(test)]
        meters: if test_meters {
            handles.meters
        } else {
            Vec::new()
        },
        host,
        render: PreparedBuiltinBatchRender {
            delivery: render_delivery,
            outcomes: outcome_producer,
            track_controls: handles.track_controls,
            pending: None,
            quantum: compiled.quantum().0,
            output_channels: usize::from(compiled.output_shape().channels),
        },
        revision,
        quantum: compiled.quantum().0,
        track_count,
        resources: BuiltinBatchResources {
            host: host_report,
            delivery: delivery_report,
            outcome: outcome_report,
            host_builtin_retained_payload_bytes: host_report.builtin_retained_payload_bytes,
            endpoint_retained_heap_bytes: endpoint_retained_bytes,
            composed_retained_heap_bytes: retained_bytes,
            largest_endpoint_heap_allocation_bytes: endpoint_largest,
            largest_host_engine_allocation_bytes: host_report.largest_engine_allocation_bytes,
            largest_composed_heap_allocation_bytes: largest,
            inline_owner_bytes,
            largest_inline_owner_bytes: largest_inline_owner,
            control_inline_bytes: usize::try_from(control_inline_bytes).unwrap_or(usize::MAX),
            prepared_render_inline_bytes: usize::try_from(prepared_render_inline_bytes)
                .unwrap_or(usize::MAX),
            started_render_inline_bytes: usize::try_from(started_render_inline_bytes)
                .unwrap_or(usize::MAX),
        },
    })
}

/// Prepared endpoint awaiting render-thread FP attestation and control/render split.
pub struct PreparedBuiltinBatchEndpoint {
    control: DeliveryCoreControl<BuiltinBatch>,
    outcomes: engine::realtime::Consumer<BuiltinBatchOutcome>,
    #[cfg(test)]
    meters: Vec<builtins_compiler::MeterConsumer>,
    host: PreparedHost,
    render: PreparedBuiltinBatchRender,
    revision: SessionRevision,
    quantum: u32,
    track_count: u32,
    resources: BuiltinBatchResources,
}

impl PreparedBuiltinBatchEndpoint {
    /// Start the endpoint on the render thread and return the control owner to its caller.
    #[allow(clippy::result_large_err)]
    pub fn start(
        self,
    ) -> Result<
        (
            BuiltinBatchControl,
            StartedBuiltinBatchRender,
            BuiltinBatchResources,
        ),
        (Self, lane::fpenv::FpEnvironmentRejection),
    > {
        let Self {
            control,
            outcomes: control_outcomes,
            #[cfg(test)]
            meters,
            host,
            render,
            revision,
            quantum,
            track_count,
            resources,
        } = self;
        let (started, sources, report) = match host.start_render_session() {
            Ok(value) => value,
            Err((host, error)) => {
                return Err((
                    Self {
                        control,
                        outcomes: control_outcomes,
                        #[cfg(test)]
                        meters,
                        host,
                        render,
                        revision,
                        quantum,
                        track_count,
                        resources,
                    },
                    error,
                ));
            }
        };
        let PreparedBuiltinBatchRender {
            delivery,
            outcomes: render_outcomes,
            track_controls,
            pending,
            quantum: render_quantum,
            output_channels,
        } = render;
        debug_assert_eq!(render_quantum, quantum);
        let render = StartedBuiltinBatchRender {
            plan: started,
            delivery,
            outcomes: render_outcomes,
            track_controls,
            pending,
            quantum,
            output_channels,
            fault: None,
            #[cfg(test)]
            fail_after_graph: false,
            #[cfg(test)]
            claim_hold_sender: None,
            #[cfg(test)]
            claim_release_receiver: None,
        };
        let control = BuiltinBatchControl {
            delivery: control,
            outcomes: control_outcomes,
            revision,
            quantum,
            track_count,
            sources,
            #[cfg(test)]
            meters,
            report,
            outstanding: 0,
            staged_outcome: None,
            cancellation_started: false,
            cancel_complete: None,
            cancel_token: None,
            cancel_completion_reported: false,
            last_collected: None,
        };
        Ok((control, render, resources))
    }
}

fn valid_fader(record: TrackFaderRecord) -> bool {
    match record {
        TrackFaderRecord::FaderDb {
            db,
            smoothing_samples: _,
            lanes: _,
        } => db.is_finite() && (-144.0..=24.0).contains(&db),
        TrackFaderRecord::Mute { .. } => true,
    }
}

fn valid_matrix(record: TrackControlRecord) -> bool {
    record.matrix.checked().is_ok()
}

fn completion_from_core(
    completion: CoreCompletion<BuiltinBatch>,
    outcome: Option<BuiltinBatchOutcome>,
) -> BuiltinBatchCompletion {
    let applied = outcome.map(|value| value.actual_sample);
    BuiltinBatchCompletion {
        ticket: completion.ticket,
        batch: completion.payload,
        disposition: completion.disposition,
        applied_prefix: completion.applied_prefix,
        remaining_count: completion.remaining_count,
        requested_sample: completion.payload.requested_sample,
        actual_sample: applied,
        late: outcome.is_some_and(|value| value.requested_sample.0 < value.actual_sample.0),
        acknowledged_sample: completion.acknowledged_sample,
    }
}

#[cfg(all(test, feature = "control-provider"))]
mod tests {
    use super::*;
    use builtins::BuiltinLaneSelector;

    fn caps() -> HostPrepareCaps {
        HostPrepareCaps {
            shape: crate::HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 1_024,
            maximum_source_channels: None,
            maximum_automation_spans_per_block: 256,
            maximum_tracks: 64,
            maximum_sources: 64,
            maximum_routes: 128,
            maximum_effects: 128,
            maximum_graph_session_plus_plan_bytes: u64::MAX,
            maximum_source_total_bytes: u64::MAX,
            maximum_source_overhead_bytes: u64::MAX,
            maximum_effect_state_bytes: u64::MAX,
            maximum_effect_scratch_bytes: u64::MAX,
            maximum_builtin_retained_bytes: u64::MAX,
            maximum_named_allocation_bytes: u64::MAX,
            maximum_meter_streams: 1,
            maximum_meter_items: 1,
            maximum_meter_bytes: 1,
        }
    }

    #[test]
    fn private_post_graph_fault_is_sticky_before_terminal_publication() {
        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let compiled = crate::compile_host_session(document, &caps()).expect("compile fixture");
        let (mut control, mut render, _) = prepare_builtin_batch_endpoint(
            &compiled,
            &caps(),
            SessionRevision(42),
            NonZeroUsize::new(1).unwrap(),
        )
        .expect("prepare")
        .start()
        .unwrap_or_else(|_| panic!("start"));
        let batch = BuiltinBatch::new(
            SessionRevision(42),
            SampleTime(0),
            &[BuiltinBatchRecord::Fader {
                track_index: 0,
                record: TrackFaderRecord::Mute {
                    lanes: BuiltinLaneSelector::Both,
                    muted: true,
                    smoothing_samples: 0,
                },
            }],
        )
        .expect("batch");
        let ticket = control.try_publish(batch).expect("publish");
        render.inject_post_graph_fault_for_test();
        let mut samples = [0.0_f32; 256];
        assert_eq!(
            render.render(&mut samples, 2, 128, 128, SampleTime(0)),
            Err(BuiltinBatchRenderError::Fault)
        );
        assert_eq!(control.outstanding(), 1);
        assert_eq!(
            render.render(&mut samples, 2, 128, 128, SampleTime(0)),
            Err(BuiltinBatchRenderError::Fault)
        );
        let cancel = control.begin_cancel().expect("cancel after fault");
        assert_eq!(control.poll_cancel_boundary(cancel), Ok(None));
        assert_eq!(control.collect(ticket), Err(DeliveryError::Empty));
        assert_eq!(control.outstanding(), 1);
    }

    #[test]
    fn forced_scalar_and_native_bank_match_with_state_and_post_fader_witnesses() {
        use builtins::{BuiltinLaneSelector, Matrix2x2, MeterTap};
        use builtins_compiler::{
            test_only_fader_matrix_witness, test_only_reset_fader_matrix_witness,
            test_only_scalar_state_trace,
        };

        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let mut meter_caps = caps();
        meter_caps.maximum_meter_streams = 64;
        meter_caps.maximum_meter_items = 1 << 16;
        meter_caps.maximum_meter_bytes = 1 << 24;
        let compiled = crate::compile_host_session(document, &meter_caps).expect("compile fixture");
        let console = HostConsoleRequest {
            control_queue_depth: Some(NonZeroUsize::new(BUILTIN_BATCH_MAX_RECORDS).unwrap()),
            meter_period_frames: Some(core::num::NonZeroU32::new(128).unwrap()),
            meter_queue_depth: NonZeroUsize::new(2).unwrap(),
            meter_tap: MeterTap::PostFader,
            ..HostConsoleRequest::default()
        };
        let batch = BuiltinBatch::new(
            SessionRevision(42),
            SampleTime(0),
            &[
                BuiltinBatchRecord::Fader {
                    track_index: 0,
                    record: TrackFaderRecord::FaderDb {
                        lanes: BuiltinLaneSelector::Left,
                        db: -6.0,
                        smoothing_samples: 16,
                    },
                },
                BuiltinBatchRecord::Matrix {
                    track_index: 8,
                    record: TrackControlRecord {
                        matrix: Matrix2x2 {
                            ll: 0.5,
                            lr: 0.25,
                            rl: -0.25,
                            rr: 0.75,
                        },
                        smoothing_samples: 11,
                    },
                },
                BuiltinBatchRecord::Fader {
                    track_index: 8,
                    record: TrackFaderRecord::FaderDb {
                        lanes: BuiltinLaneSelector::Right,
                        db: -3.0,
                        smoothing_samples: 13,
                    },
                },
            ],
        )
        .expect("batch");
        let source_left = [0.25_f32; 128];
        let source_right = [-0.5_f32; 128];
        let submission = crate::SourceSubmission {
            generation: 1,
            start_frame: 0,
            sample_rate_hz: 48_000,
            planes: &[&source_left, &source_right],
            frames: 128,
            end_of_region: false,
        };

        let prepare_console = |backend| {
            crate::prepare::prepare_host_runtime_with_console_backend(
                &compiled,
                &meter_caps,
                &console,
                backend,
            )
            .expect("prepare console")
        };
        let (bank_host, mut bank_handles) = prepare_console(Backend::current());
        let (mut bank_render, mut bank_sources, _) =
            bank_host.start_render_session().expect("bank start");
        let (scalar_host, mut scalar_handles) = prepare_console(Backend::Scalar);
        let (mut scalar_render, mut scalar_sources, _) =
            scalar_host.start_render_session().expect("scalar start");
        let (mut bank_control, mut bank_endpoint_render, _) =
            prepare_builtin_batch_endpoint_for_test(
                &compiled,
                &meter_caps,
                SessionRevision(42),
                NonZeroUsize::new(2).unwrap(),
                Backend::current(),
            )
            .expect("bank endpoint")
            .start()
            .unwrap_or_else(|_| panic!("bank endpoint start"));
        let (mut scalar_control, mut scalar_endpoint_render, _) =
            prepare_builtin_batch_endpoint_for_test(
                &compiled,
                &meter_caps,
                SessionRevision(42),
                NonZeroUsize::new(2).unwrap(),
                Backend::Scalar,
            )
            .expect("scalar endpoint")
            .start()
            .unwrap_or_else(|_| panic!("scalar endpoint start"));
        bank_sources
            .submit(b"fixture-source", submission)
            .expect("bank source");
        scalar_sources
            .submit(b"fixture-source", submission)
            .expect("scalar source");
        bank_control
            .sources()
            .submit(b"fixture-source", submission)
            .expect("bank endpoint source");
        scalar_control
            .sources()
            .submit(b"fixture-source", submission)
            .expect("scalar endpoint source");
        for handles in [&mut bank_handles, &mut scalar_handles] {
            handles.track_controls[0]
                .fader
                .try_push(TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Left,
                    db: -6.0,
                    smoothing_samples: 16,
                })
                .expect("fader");
            handles.track_controls[8]
                .producer
                .try_push(TrackControlRecord {
                    matrix: Matrix2x2 {
                        ll: 0.5,
                        lr: 0.25,
                        rl: -0.25,
                        rr: 0.75,
                    },
                    smoothing_samples: 11,
                })
                .expect("matrix");
            handles.track_controls[8]
                .fader
                .try_push(TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Right,
                    db: -3.0,
                    smoothing_samples: 13,
                })
                .expect("scalar fader");
        }
        let bank_ticket = bank_control.try_publish(batch).expect("bank ticket");
        let scalar_ticket = scalar_control.try_publish(batch).expect("scalar ticket");
        let mut bank_reference = [0.0_f32; 256];
        let mut scalar_reference = [0.0_f32; 256];
        let mut bank_endpoint = [0.0_f32; 256];
        let mut scalar_endpoint = [0.0_f32; 256];
        test_only_reset_fader_matrix_witness();
        bank_render
            .render_planar(&mut bank_reference, 2, 128, 128, 0)
            .expect("bank reference render");
        let bank_reference_witness = test_only_fader_matrix_witness();
        test_only_reset_fader_matrix_witness();
        bank_endpoint_render
            .render(&mut bank_endpoint, 2, 128, 128, SampleTime(0))
            .expect("bank endpoint render");
        let bank_endpoint_witness = test_only_fader_matrix_witness();
        test_only_reset_fader_matrix_witness();
        scalar_render
            .render_planar(&mut scalar_reference, 2, 128, 128, 0)
            .expect("scalar reference render");
        let scalar_reference_state = test_only_scalar_state_trace();
        let scalar_reference_witness = test_only_fader_matrix_witness();
        test_only_reset_fader_matrix_witness();
        scalar_endpoint_render
            .render(&mut scalar_endpoint, 2, 128, 128, SampleTime(0))
            .expect("scalar endpoint render");
        let scalar_endpoint_state = test_only_scalar_state_trace();
        let scalar_endpoint_witness = test_only_fader_matrix_witness();
        let to_bits = |samples: &[f32]| {
            samples
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>()
        };
        assert_eq!(to_bits(&bank_reference), to_bits(&bank_endpoint));
        assert_eq!(to_bits(&scalar_reference), to_bits(&scalar_endpoint));
        assert!(scalar_endpoint_state.fader_len > 0);
        assert!(scalar_endpoint_state.matrix_len > 0);
        assert_eq!(scalar_endpoint_state, scalar_reference_state);
        assert_eq!(bank_endpoint_witness, bank_reference_witness);
        assert_eq!(scalar_endpoint_witness, scalar_reference_witness);
        assert_eq!(bank_endpoint_witness.process_calls, 0);
        assert_eq!(bank_endpoint_witness.factory_calls, 0);
        assert_eq!(bank_endpoint_witness.fused_calls, 0);
        assert_eq!(bank_endpoint_witness.fallback_calls, 0);
        assert_eq!(bank_endpoint_witness.process_members, 0);
        assert_eq!(bank_endpoint_witness.factory_members, 0);
        assert_eq!(scalar_endpoint_witness.process_calls, 0);
        assert_eq!(scalar_endpoint_witness.factory_calls, 0);
        assert_eq!(scalar_endpoint_witness.fused_calls, 0);
        assert_eq!(scalar_endpoint_witness.fallback_calls, 0);
        assert_eq!(scalar_endpoint_witness.process_members, 0);
        assert_eq!(scalar_endpoint_witness.factory_members, 0);
        let bank_peak = bank_handles
            .meters
            .iter_mut()
            .find(|meter| meter.track_id.as_ref() == "eq8")
            .expect("bank reference meter")
            .consumer
            .try_pop()
            .expect("bank reference snapshot");
        let scalar_peak = scalar_handles
            .meters
            .iter_mut()
            .find(|meter| meter.track_id.as_ref() == "eq8")
            .expect("scalar reference meter")
            .consumer
            .try_pop()
            .expect("scalar reference snapshot");
        let bank_endpoint_peak = bank_control
            .meters
            .iter_mut()
            .find(|meter| meter.track_id.as_ref() == "eq8")
            .expect("bank endpoint meter")
            .consumer
            .try_pop()
            .expect("bank endpoint snapshot");
        let scalar_endpoint_peak = scalar_control
            .meters
            .iter_mut()
            .find(|meter| meter.track_id.as_ref() == "eq8")
            .expect("scalar endpoint meter")
            .consumer
            .try_pop()
            .expect("scalar endpoint snapshot");
        assert_eq!(
            bank_peak.left.sample_peak.to_bits(),
            bank_endpoint_peak.left.sample_peak.to_bits()
        );
        assert_eq!(
            scalar_peak.left.sample_peak.to_bits(),
            scalar_endpoint_peak.left.sample_peak.to_bits()
        );
        assert_eq!(
            bank_peak.right.sample_peak.to_bits(),
            bank_endpoint_peak.right.sample_peak.to_bits()
        );
        assert_eq!(
            scalar_peak.right.sample_peak.to_bits(),
            scalar_endpoint_peak.right.sample_peak.to_bits()
        );
        bank_control.collect(bank_ticket).expect("bank collect");
        scalar_control
            .collect(scalar_ticket)
            .expect("scalar collect");
    }

    #[test]
    fn endpoint_selects_existing_pair_factories_without_observer_barriers() {
        use builtins::{BuiltinLaneSelector, Matrix2x2};
        use builtins_compiler::{
            test_only_fader_matrix_witness, test_only_reset_fader_matrix_witness,
        };

        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let compiled = crate::compile_host_session(document, &caps()).expect("compile fixture");
        let console = HostConsoleRequest {
            control_queue_depth: Some(NonZeroUsize::new(BUILTIN_BATCH_MAX_RECORDS).unwrap()),
            ..HostConsoleRequest::default()
        };
        let batch = || {
            BuiltinBatch::new(
                SessionRevision(42),
                SampleTime(0),
                &[
                    BuiltinBatchRecord::Fader {
                        track_index: 8,
                        record: TrackFaderRecord::FaderDb {
                            lanes: BuiltinLaneSelector::Both,
                            db: -6.0,
                            smoothing_samples: 16,
                        },
                    },
                    BuiltinBatchRecord::Matrix {
                        track_index: 8,
                        record: TrackControlRecord {
                            matrix: Matrix2x2 {
                                ll: 0.5,
                                lr: 0.25,
                                rl: -0.25,
                                rr: 0.75,
                            },
                            smoothing_samples: 11,
                        },
                    },
                ],
            )
            .expect("batch")
        };
        let run = |backend| {
            let (host, mut handles) = crate::prepare::prepare_host_runtime_with_console_backend(
                &compiled,
                &caps(),
                &console,
                backend,
            )
            .expect("prepare reference");
            let (mut reference, mut reference_sources, _) =
                host.start_render_session().expect("reference start");
            test_only_reset_fader_matrix_witness();
            let (mut control, mut render, _) = prepare_builtin_batch_endpoint_for_test_with_meters(
                &compiled,
                &caps(),
                SessionRevision(42),
                NonZeroUsize::new(2).unwrap(),
                backend,
                false,
            )
            .expect("endpoint prepare")
            .start()
            .unwrap_or_else(|_| panic!("endpoint start"));
            let source_left = [0.25_f32; 128];
            let source_right = [-0.5_f32; 128];
            let submission = crate::SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&source_left, &source_right],
                frames: 128,
                end_of_region: false,
            };
            reference_sources
                .submit(b"fixture-source", submission)
                .expect("reference source");
            control
                .sources()
                .submit(b"fixture-source", submission)
                .expect("endpoint source");
            handles.track_controls[8]
                .fader
                .try_push(TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -6.0,
                    smoothing_samples: 16,
                })
                .expect("reference fader");
            handles.track_controls[8]
                .producer
                .try_push(TrackControlRecord {
                    matrix: Matrix2x2 {
                        ll: 0.5,
                        lr: 0.25,
                        rl: -0.25,
                        rr: 0.75,
                    },
                    smoothing_samples: 11,
                })
                .expect("reference matrix");
            let ticket = control.try_publish(batch()).expect("ticket");
            let mut reference_output = [0.0_f32; 256];
            let mut endpoint_output = [0.0_f32; 256];
            reference
                .render_planar(&mut reference_output, 2, 128, 128, 0)
                .expect("reference render");
            render
                .render(&mut endpoint_output, 2, 128, 128, SampleTime(0))
                .expect("endpoint render");
            let witness = test_only_fader_matrix_witness();
            control.collect(ticket).expect("collect");
            let bits = |samples: &[f32]| {
                samples
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>()
            };
            (bits(&reference_output), bits(&endpoint_output), witness)
        };

        let (bank_reference, bank_endpoint, bank_witness) = run(Backend::current());
        assert_eq!(bank_reference, bank_endpoint);
        assert!(
            bank_witness.factory_calls > 0,
            "native bank pair factory was not selected: {bank_witness:?}"
        );
        assert!(
            bank_witness.process_calls > 0,
            "native bank pair was not processed: {bank_witness:?}"
        );
        assert!(bank_witness.factory_members > 0);
        assert!(bank_witness.process_members > 0);

        let (scalar_reference, scalar_endpoint, scalar_witness) = run(Backend::Scalar);
        assert_eq!(scalar_reference, scalar_endpoint);
        assert!(
            scalar_witness.factory_calls > 0,
            "scalar pair factory was not selected: {scalar_witness:?}"
        );
        assert!(
            scalar_witness.process_calls > 0,
            "scalar pair was not processed: {scalar_witness:?}"
        );
        assert!(scalar_witness.factory_members > 0);
        assert!(scalar_witness.process_members > 0);
        assert!(
            scalar_witness
                .scalar_fader_words
                .iter()
                .any(|word| *word != 0)
        );
        assert!(
            scalar_witness
                .scalar_matrix_words
                .iter()
                .any(|word| *word != 0)
        );
    }

    #[test]
    fn pcm_bitwise_discriminator_rejects_signed_zero() {
        let to_bits = |samples: &[f32]| {
            samples
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>()
        };
        assert_ne!(to_bits(&[0.0]), to_bits(&[-0.0]));
    }

    #[test]
    fn endpoint_queue_retention_matches_layout_and_reclaims_off_render() {
        use bench_support::alloc as bench_alloc;

        bench_alloc::assert_installed();
        let warm = Box::new([0_u8; 64]);
        std::hint::black_box(&warm);
        drop(warm);
        let capacity = NonZeroUsize::new(2).unwrap();
        let thread_mark = bench_alloc::current_thread_counters();
        let (delivery_report, outcome_report) =
            endpoint_queue_reports(capacity).expect("queue report");
        let queues = prepare_endpoint_queues(capacity, delivery_report, outcome_report)
            .expect("queue preparation");
        let thread_live = bench_alloc::current_thread_delta_since(thread_mark);
        let expected_bytes = delivery_report
            .retained_payload_bytes
            .saturating_add(outcome_report.retained_payload_bytes);
        assert_eq!(thread_live.deallocations, 0);
        assert_eq!(thread_live.reallocations, 0);
        assert_eq!(thread_live.requested_bytes, expected_bytes);
        assert!(thread_live.allocations > 0);
        let allocations = thread_live.allocations;
        drop(queues);
        let thread_released = bench_alloc::current_thread_delta_since(thread_mark);
        assert_eq!(thread_released.allocations, allocations);
        assert_eq!(thread_released.deallocations, allocations);
    }

    #[test]
    fn private_post_claim_hold_rejects_same_block_second_claim() {
        use std::sync::mpsc::sync_channel;

        let document = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let compiled = crate::compile_host_session(document, &caps()).expect("compile fixture");
        std::thread::scope(|scope| {
            let (control_tx, control_rx) = sync_channel(1);
            let (step_tx, step_rx) = sync_channel(0);
            let (report_tx, report_rx) = sync_channel(1);
            let (claimed_tx, claimed_rx) = sync_channel(0);
            let (release_tx, release_rx) = sync_channel(0);
            let render_thread = scope.spawn(move || {
                let (control, mut render, _) = prepare_builtin_batch_endpoint(
                    &compiled,
                    &caps(),
                    SessionRevision(42),
                    NonZeroUsize::new(2).unwrap(),
                )
                .expect("prepare")
                .start()
                .unwrap_or_else(|_| panic!("start"));
                render.hold_after_claim_for_test(claimed_tx, release_rx);
                control_tx.send(control).expect("control handoff");
                for first in [0_u64, 128] {
                    step_rx.recv().expect("render step");
                    let mut samples = [0.0_f32; 256];
                    let report = render
                        .render(&mut samples, 2, 128, 128, SampleTime(first))
                        .expect("render");
                    report_tx.send(report).expect("render report");
                }
            });
            let mut control = control_rx.recv().expect("control owner");
            let batch = |track_index| {
                BuiltinBatch::new(
                    SessionRevision(42),
                    SampleTime(0),
                    &[BuiltinBatchRecord::Fader {
                        track_index,
                        record: TrackFaderRecord::Mute {
                            lanes: builtins::BuiltinLaneSelector::Both,
                            muted: track_index == 0,
                            smoothing_samples: 0,
                        },
                    }],
                )
                .expect("batch")
            };
            let first = control.try_publish(batch(0)).expect("first publish");
            step_tx.send(()).expect("first step");
            claimed_rx.recv().expect("claim hold");
            let second = control.try_publish(batch(1)).expect("post-claim publish");
            assert_eq!(control.outstanding(), 2);
            release_tx.send(()).expect("claim release");
            let first_report = report_rx.recv().expect("first report");
            let first_application = first_report.applied.expect("first application");
            assert_eq!(first_application.ticket, first);
            assert_eq!(first_application.actual_sample, SampleTime(0));
            control.collect(first).expect("first collect");
            step_tx.send(()).expect("second step");
            let second_report = report_rx.recv().expect("second report");
            let second_application = second_report.applied.expect("second application");
            assert_eq!(second_application.ticket, second);
            assert_eq!(second_application.actual_sample, SampleTime(128));
            assert!(second_application.late);
            let second_completion = control.collect(second).expect("second collect");
            assert_eq!(second_completion.actual_sample, Some(SampleTime(128)));
            assert!(second_completion.late);
            render_thread.join().expect("render join");
        });
    }
}
