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
    prepare_host_runtime_with_console,
};
use crate::render_session::StartedRenderSession;
use crate::source::SourceControlSet;

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
    report: HostPrepareReport,
    outstanding: usize,
    staged_outcome: Option<BuiltinBatchOutcome>,
    cancellation_started: bool,
    cancel_complete: Option<CoreCancelComplete>,
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
        let token = self.delivery.begin_cancel()?;
        self.cancellation_started = true;
        self.cancel_complete = None;
        self.cancel_completion_reported = false;
        Ok(token)
    }

    /// Poll cancellation until the render boundary has acknowledged and all terminals are ready.
    pub fn poll_cancel_boundary(
        &mut self,
        token: CoreCancelToken,
    ) -> Result<Option<CoreCancelComplete>, DeliveryError> {
        if let Some(complete) = self.cancel_complete {
            if self.outstanding != 0 {
                return Ok(None);
            }
            if self.cancel_completion_reported {
                return Err(DeliveryError::StaleTicket);
            }
            self.cancel_completion_reported = true;
            return Ok(Some(complete));
        }
        let complete = self.delivery.poll_cancel_boundary(token)?;
        if let Some(complete) = complete {
            self.cancel_complete = Some(complete);
            if self.outstanding == 0 {
                self.cancellation_started = false;
            }
        }
        if self.outstanding == 0 {
            self.cancel_completion_reported = true;
            Ok(complete)
        } else {
            Ok(None)
        }
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
        if self.outstanding == 0 {
            self.cancellation_started = false;
        }
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

/// Prepare the host, console channels, typed delivery core and outcome storage as one transaction.
pub fn prepare_builtin_batch_endpoint(
    compiled: &session::CompiledSession,
    caps: &HostPrepareCaps,
    revision: SessionRevision,
    ticket_capacity: NonZeroUsize,
) -> Result<PreparedBuiltinBatchEndpoint, BuiltinBatchPrepareError> {
    // Project endpoint storage before asking host preparation to allocate anything.  This makes
    // the endpoint's own aggregate and largest-allocation caps fail atomically.
    let delivery_report = PreparedDelivery::<BuiltinBatch>::resource_report(ticket_capacity)
        .map_err(BuiltinBatchPrepareError::Delivery)?;
    let outcome_report = outcome_resource_report(ticket_capacity)?;
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
    let (host, handles) = prepare_host_runtime_with_console(compiled, caps, &console)
        .map_err(BuiltinBatchPrepareError::Host)?;
    let host_report = host.report;
    let (delivery, render_delivery) = PreparedDelivery::<BuiltinBatch>::prepare(ticket_capacity)
        .map_err(BuiltinBatchPrepareError::Delivery)?;
    let (outcome_producer, outcome_consumer) = bounded_spsc(ticket_capacity, QueueGeneration(17))
        .map_err(|_| {
        BuiltinBatchPrepareError::Delivery(protocol::ProtocolQueueError::CapacityOverflow)
    })?;
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
            report,
            outstanding: 0,
            staged_outcome: None,
            cancellation_started: false,
            cancel_complete: None,
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
