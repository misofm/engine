//! Opt-in delivery of scalar compressor makeup Points to planar PCM.
//!
//! This endpoint binds exactly two revision-scoped handles to the native compressor's Left and
//! Right makeup parameter. Admission accepts complete protocol batches; application occurs only
//! after control-side handoff and at render boundaries. Points retain their order and are never
//! coalesced. Admission credit remains owned until terminal collection or cancellation, even after
//! native application has completed.
//!
//! Times are absolute samples. A snapshot distinguishes the next required render sample, the last
//! fully observed sample, native current and target values, and the sole render-side claimed batch.
//! A future Point, including one exactly at the block end, remains pending until a later block.
//!
//! Preparation exclusively borrows an already prepared compressor. The unstarted render owner is
//! transferable; `start` attests the floating-point environment and produces a thread-affine
//! owner, while `stop` returns the transferable owner. Snapshot and render native access run under
//! the canonical floating-point environment.
//!
//! Valid render envelopes have one exact prepared quantum and contiguous time. An unexpected
//! native or delivery error becomes sticky: the complete valid output block is silenced, the
//! consumed envelope advances once, and later valid renders return the first fault without DSP or
//! clock advancement. Fault progress distinguishes Points actually accepted by native code from
//! the prefix recorded by the delivery service. A faulted endpoint services a cancellation barrier
//! only through `cancel_boundary`, which performs no DSP or parameter access.
//!
//! Resource reporting separates the unchanged delivery service's heap payload from the inline
//! control/render owner sizes. It excludes the borrowed compressor, caller PCM, and their storage.

use core::marker::PhantomData;
use effect_contract::{
    EffectProcessBlock, EffectQuality, LinkMode, ParameterAccessError, ParameterChannel,
    ParameterChannelPolicy, ParameterDomain, ParameterUnit, PreparedNativeEffect,
    PreparedParameterState, ProcessBlockError, ProcessReport, SmoothingRule,
};
use lane::CanonicalFpEnv;
use protocol::{
    AutomationBatchError, AutomationBatchSlot, AutomationCancellationReason,
    AutomationEnqueueError, DeliveryError, DeliveryTicket, HandoffResult, ParameterHandle,
    PreparedAutomationDelivery, PreparedDeliveryCapabilities, ProtocolQueueConfig, QueueKind,
    QueueReport, SampleTime, SessionRevision,
};

const MAKEUP_INDEX: u32 = 5;
const MAKEUP_ID: u32 = 6;

/// Why endpoint preparation rejected the supplied processor, bindings, or resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointPrepareError {
    /// The effect is not the required Normal-quality, enabled, dual-mono native compressor with
    /// the frozen makeup descriptor, quantum, rate, and unconnected optional sidechain.
    InvalidProcessor,
    /// The Left/Right handles are zero, equal, reversed, or cannot form exact Point capabilities.
    InvalidBindings,
    /// The configured quantum is zero or cannot be represented by the endpoint's `u32` contract.
    InvalidQueue,
    /// The bounded delivery service rejected the queue configuration or initial sequence.
    Delivery(protocol::ProtocolQueueError),
}
/// Delivery heap and endpoint inline accounting; excludes borrowed compressor and caller PCM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointResources {
    /// Heap payload retained by the underlying prepared automation-delivery service.
    pub delivery: protocol::DeliveryResourceReport,
    /// Inline byte size of [`ScalarPointControl`], excluding its owned delivery allocations.
    pub control_size: usize,
    /// Inline byte size of [`PreparedScalarPointRender`], excluding the borrowed effect and PCM.
    pub render_size: usize,
}
/// Endpoint admission rejection retaining the complete batch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScalarPointAdmissionError {
    /// The batch's own record, length, ordering, overlap, or time-shape validation failed.
    InvalidBatch {
        /// The untouched batch returned to its caller.
        batch: AutomationBatchSlot,
        /// The protocol batch validation failure.
        error: AutomationBatchError,
    },
    /// The batch uses a session revision other than the one fixed at preparation.
    WrongRevision {
        /// The untouched batch returned to its caller.
        batch: AutomationBatchSlot,
    },
    /// At least one record does not use the prepared Left or Right makeup handle.
    UnknownBinding {
        /// The untouched batch returned to its caller.
        batch: AutomationBatchSlot,
    },
    /// At least one endpoint value is outside the inclusive `-24..=24` dB makeup domain.
    InvalidValue {
        /// The untouched batch returned to its caller.
        batch: AutomationBatchSlot,
    },
    /// The validated batch was refused by the bounded delivery service and remains caller-owned.
    Service(AutomationEnqueueError),
}

/// Control-thread owner of admission, handoff, terminal credit, and cancellation.
pub struct ScalarPointControl {
    delivery: protocol::AutomationDeliveryControl,
    revision: SessionRevision,
    handles: [ParameterHandle; 2],
    capabilities: PreparedDeliveryCapabilities,
}
impl ScalarPointControl {
    /// Validates and admits one complete batch at control time without applying native state.
    #[allow(clippy::result_large_err)] // Frozen rejection returns the untouched fixed batch.
    pub fn try_admit(
        &mut self,
        now: SampleTime,
        batch: AutomationBatchSlot,
    ) -> Result<(), ScalarPointAdmissionError> {
        if let Err(error) = batch.validate_records() {
            return Err(ScalarPointAdmissionError::InvalidBatch { batch, error });
        }
        if batch.revision != self.revision {
            return Err(ScalarPointAdmissionError::WrongRevision { batch });
        }
        if batch
            .as_slice()
            .iter()
            .any(|r| !self.handles.contains(&r.handle))
        {
            return Err(ScalarPointAdmissionError::UnknownBinding { batch });
        }
        if batch.as_slice().iter().any(|r| {
            !(-24.0..=24.0).contains(&r.start_value) || !(-24.0..=24.0).contains(&r.end_value)
        }) {
            return Err(ScalarPointAdmissionError::InvalidValue { batch });
        }
        self.delivery
            .try_admit(now, batch)
            .map_err(ScalarPointAdmissionError::Service)
    }
    /// Transfers the FIFO head to render ownership when the whole batch is Point-supported.
    ///
    /// A batch containing any other automation kind remains whole and reports
    /// [`HandoffResult::PendingUnsupported`], blocking later batches until cancellation.
    pub fn try_handoff_next(&mut self) -> Result<HandoffResult, DeliveryError> {
        self.delivery.try_handoff_next(&self.capabilities)
    }
    /// Reconciles and releases one fully terminal ticket, returning its applied-prefix record.
    pub fn collect_terminal(
        &mut self,
        t: DeliveryTicket,
    ) -> Result<protocol::TerminalAutomation, DeliveryError> {
        self.delivery.collect_terminal(t)
    }
    /// Begins ordered cancellation of every accepted owner at a later render boundary.
    pub fn begin_cancel(
        &mut self,
        r: AutomationCancellationReason,
    ) -> Result<protocol::CancelToken, DeliveryError> {
        self.delivery.begin_cancel(r, self.revision)
    }
    /// Polls until the render side has observed the cancellation barrier and owners are released.
    pub fn poll_cancel_boundary(
        &mut self,
        t: protocol::CancelToken,
    ) -> Result<Option<protocol::CancelComplete>, DeliveryError> {
        self.delivery.poll_cancel_boundary(t)
    }
    /// Dequeues one reliable cancellation event produced during control-side reconciliation.
    pub fn try_dequeue_event(
        &mut self,
    ) -> Result<protocol::ReliableSlot, engine::realtime::QueueEmpty> {
        self.delivery.try_dequeue_event()
    }
    /// Returns all accepted owners, including queued, staged, handed-off, and terminal work.
    pub fn outstanding(&self) -> usize {
        self.delivery.outstanding()
    }
    /// Returns batches still resident in the public automation queue before ownership handoff.
    pub fn resident_automation(&self) -> u64 {
        self.delivery.resident_automation()
    }
    /// Returns the underlying automation queue's bounded occupancy and saturation counters.
    pub fn automation_status(&self) -> QueueReport {
        self.delivery.queues().report(QueueKind::Automation)
    }
}

/// Between-block native state and the sole bounded render claim.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalarPointSnapshot {
    /// Absolute first sample required by the next valid render or cancellation-only boundary.
    pub next_sample: SampleTime,
    /// End sample of the most recent block whose DSP result was successfully observed.
    pub observed_sample: SampleTime,
    /// Native makeup current/target states in `[Left, Right]` order.
    pub state: [PreparedParameterState; 2],
    /// Most recent actual application sample in `[Left, Right]` order.
    pub last_application: [Option<SampleTime>; 2],
    /// Saturating count of Points successfully accepted by the native compressor.
    pub applied: u64,
    /// Saturating subset of `applied` whose requested time preceded its application boundary.
    pub late: u64,
    /// Claimed `(ticket, applied prefix, record count, next unapplied sample)` if one is active.
    pub pending: Option<(DeliveryTicket, u16, u16, Option<SampleTime>)>,
}
/// Native parameter operation that produced a sticky endpoint fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointNativeOperation {
    /// Reading the native makeup current/target state failed.
    Read,
    /// Applying a native makeup Point target failed.
    Apply,
}
/// Delivery-service operation that produced a sticky invariant fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointDeliveryOperation {
    /// Reading the currently claimed ticket failed.
    Pending,
    /// Recording a successfully applied native prefix failed.
    MarkApplied,
    /// Publishing a fully applied ticket as terminal failed.
    FinishApplied,
}
/// Typed cause retained by the first sticky endpoint fault.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointFaultCause {
    /// A native parameter read or apply returned an unexpected typed error.
    Native {
        /// Whether the endpoint was reading state or applying a Point.
        operation: ScalarPointNativeOperation,
        /// Compressor lane involved in the failed access.
        channel: ParameterChannel,
        /// Native parameter-access error returned by the processor.
        error: ParameterAccessError,
    },
    /// The trusted bounded delivery service violated the endpoint's expected ownership sequence.
    Delivery {
        /// Delivery operation that failed.
        operation: ScalarPointDeliveryOperation,
        /// Underlying delivery error.
        error: DeliveryError,
    },
    /// An internally constructed native process slice unexpectedly failed envelope validation.
    ProcessEnvelope(ProcessBlockError),
}
/// Applied-prefix detail captured when a sticky fault occurs with a claimed ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointFaultProgress {
    /// Claimed delivery ticket.
    pub ticket: DeliveryTicket,
    /// Total records retained by that ticket.
    pub record_count: u16,
    /// Prefix actually accepted by native parameter application.
    pub native_applied_prefix: u16,
    /// Prefix successfully recorded by the delivery service.
    pub delivery_applied_prefix: u16,
}
/// First sticky failure, its exact sample, and irreversible progress made before silence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointFault {
    /// Typed native, delivery, or internal process-envelope cause.
    pub cause: ScalarPointFaultCause,
    /// Sample at which the failing operation was attempted.
    pub at_sample: SampleTime,
    /// Native frames processed earlier in the same failing render call.
    pub native_processed_frames: u32,
    /// Claimed-ticket progress at the failure, when a claim existed.
    pub progress: Option<ScalarPointFaultProgress>,
}
/// Rejection or sticky failure returned by a render call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointRenderError {
    /// PCM planes do not both contain exactly the prepared quantum.
    InvalidShape,
    /// `first` is not the endpoint's next required absolute sample.
    DiscontinuousTime,
    /// Adding the prepared quantum to `first` overflowed absolute sample time.
    SampleOverflow,
    /// A valid envelope encountered, or followed, the retained first endpoint fault.
    Fault(ScalarPointFault),
}
/// Rejection returned by the DSP-free cancellation-only boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointCancelBoundaryError {
    /// Cancellation-only boundaries are unavailable while the endpoint is healthy.
    NotFaulted,
    /// The boundary does not start at the endpoint's next required sample.
    DiscontinuousTime,
    /// Adding the prepared quantum would overflow absolute sample time.
    SampleOverflow,
}

/// Transferable unstarted owner holding the exclusive processor borrow and render-side delivery.
///
/// The owner may move to the selected render thread before [`Self::start`].
///
/// ```
/// fn assert_send<T: Send>() {}
/// assert_send::<host_core::PreparedScalarPointRender<'static>>();
/// ```
pub struct PreparedScalarPointRender<'a> {
    processor: &'a mut dyn PreparedNativeEffect,
    delivery: protocol::AutomationDeliveryRender,
    handles: [ParameterHandle; 2],
    quantum: u32,
    next_sample: SampleTime,
    observed_sample: SampleTime,
    last_application: [Option<SampleTime>; 2],
    applied: u64,
    late: u64,
    claimed: Option<(DeliveryTicket, u16, u16, Option<SampleTime>)>,
    fault: Option<ScalarPointFault>,
}
/// Aggregate result of all native process slices used for one endpoint quantum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointRenderReport {
    /// Field-wise saturating sum of the native compressor's per-slice reports.
    pub native: ProcessReport,
    /// Number of native process calls made after splitting the quantum at Point offsets.
    pub process_invocations: u32,
}
/// Thread-affine started owner used for rendering and fault-only cancellation boundaries.
///
/// Starting pins use to the current thread until [`Self::stop`] returns the transferable owner.
///
/// ```compile_fail
/// fn assert_send<T: Send>() {}
/// assert_send::<host_core::StartedScalarPointRender<'static>>();
/// ```
///
/// ```compile_fail
/// fn assert_sync<T: Sync>() {}
/// assert_sync::<host_core::StartedScalarPointRender<'static>>();
/// ```
pub struct StartedScalarPointRender<'a> {
    inner: PreparedScalarPointRender<'a>,
    _thread: PhantomData<*const ()>,
}
impl<'a> PreparedScalarPointRender<'a> {
    /// Attests the current floating-point environment and pins the complete owner to this thread.
    ///
    /// Failure returns the unchanged prepared owner together with the attestation rejection.
    #[allow(clippy::result_large_err)] // Failed attestation must return the complete prepared owner.
    pub fn start(
        self,
    ) -> Result<StartedScalarPointRender<'a>, (Self, lane::fpenv::FpEnvironmentRejection)> {
        match lane::attest_fp_environment() {
            Ok(()) => Ok(StartedScalarPointRender {
                inner: self,
                _thread: PhantomData,
            }),
            Err(e) => Err((self, e)),
        }
    }
    /// Reads native Left/Right current and target values plus endpoint progress between blocks.
    ///
    /// An unexpected native read failure becomes the first sticky fault without advancing DSP or
    /// time. Native reads run under the canonical floating-point environment.
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointFault> {
        let _fp = CanonicalFpEnv::enter();
        self.snapshot_inner()
    }
    /// Returns the retained first fault without reading native state or mutating progress.
    pub const fn fault(&self) -> Option<ScalarPointFault> {
        self.fault
    }
}
impl<'a> StartedScalarPointRender<'a> {
    /// Applies due Points and renders one exact contiguous quantum into the borrowed PCM planes.
    ///
    /// Ordinary envelope rejection is a complete no-op. A valid call that encounters a sticky
    /// fault silences the whole block; the first failing call consumes that envelope, while later
    /// valid faulted calls do not advance time or delivery state.
    pub fn render(
        &mut self,
        l: &mut [f32],
        r: &mut [f32],
        first: SampleTime,
    ) -> Result<ScalarPointRenderReport, ScalarPointRenderError> {
        let _fp = CanonicalFpEnv::enter();
        self.inner.render_inner(l, r, first)
    }
    /// Reads native Left/Right current and target values plus endpoint progress between blocks.
    ///
    /// Read failures latch and return the first sticky fault without advancing DSP or time.
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointFault> {
        let _fp = CanonicalFpEnv::enter();
        self.inner.snapshot_inner()
    }
    /// Returns the retained first fault without reading native state or mutating progress.
    pub const fn fault(&self) -> Option<ScalarPointFault> {
        self.inner.fault
    }
    /// Services one contiguous delivery cancellation boundary after a sticky fault.
    ///
    /// This calls no DSP or native parameter access. A valid boundary advances the endpoint clock
    /// once and refreshes the claimed-ticket view while preserving the immutable first fault.
    pub fn cancel_boundary(
        &mut self,
        first: SampleTime,
    ) -> Result<(), ScalarPointCancelBoundaryError> {
        if self.inner.fault.is_none() {
            return Err(ScalarPointCancelBoundaryError::NotFaulted);
        }
        if first != self.inner.next_sample {
            return Err(ScalarPointCancelBoundaryError::DiscontinuousTime);
        }
        let end = first
            .0
            .checked_add(u64::from(self.inner.quantum))
            .ok_or(ScalarPointCancelBoundaryError::SampleOverflow)?;
        self.inner.refresh(first);
        self.inner.next_sample = SampleTime(end);
        Ok(())
    }
    /// Ends thread-affine use and returns the complete transferable prepared owner.
    pub fn stop(self) -> PreparedScalarPointRender<'a> {
        self.inner
    }
}
impl PreparedScalarPointRender<'_> {
    fn progress(&self) -> Option<ScalarPointFaultProgress> {
        self.claimed
            .map(|(ticket, prefix, count, _)| ScalarPointFaultProgress {
                ticket,
                record_count: count,
                native_applied_prefix: prefix,
                delivery_applied_prefix: prefix,
            })
    }
    fn latch(
        &mut self,
        cause: ScalarPointFaultCause,
        at: SampleTime,
        frames: u32,
        progress: Option<ScalarPointFaultProgress>,
    ) -> ScalarPointFault {
        if let Some(f) = self.fault {
            return f;
        }
        let f = ScalarPointFault {
            cause,
            at_sample: at,
            native_processed_frames: frames,
            progress,
        };
        self.fault = Some(f);
        f
    }
    fn snapshot_inner(&mut self) -> Result<ScalarPointSnapshot, ScalarPointFault> {
        if let Some(f) = self.fault {
            return Err(f);
        }
        let mut state = [PreparedParameterState {
            current_value: 0.0,
            target_value: 0.0,
        }; 2];
        for (i, ch) in [ParameterChannel::Left, ParameterChannel::Right]
            .into_iter()
            .enumerate()
        {
            match self.processor.parameter_state(MAKEUP_INDEX, ch) {
                Ok(v) => state[i] = v,
                Err(error) => {
                    let p = self.progress();
                    let f = self.latch(
                        ScalarPointFaultCause::Native {
                            operation: ScalarPointNativeOperation::Read,
                            channel: ch,
                            error,
                        },
                        self.next_sample,
                        0,
                        p,
                    );
                    return Err(f);
                }
            }
        }
        Ok(ScalarPointSnapshot {
            next_sample: self.next_sample,
            observed_sample: self.observed_sample,
            state,
            last_application: self.last_application,
            applied: self.applied,
            late: self.late,
            pending: self.claimed,
        })
    }
    fn refresh(&mut self, first: SampleTime) {
        self.claimed = self.delivery.begin_boundary(first).map(|p| {
            let count = p.records.len() as u16;
            let next = p
                .records
                .get(usize::from(p.applied_prefix))
                .map(|r| r.start);
            (p.ticket, p.applied_prefix, count, next)
        })
    }
    #[allow(clippy::too_many_arguments)] // Keeps all fixed fault progress explicit and allocation-free.
    fn fail<T>(
        &mut self,
        l: &mut [f32],
        r: &mut [f32],
        end: u64,
        cause: ScalarPointFaultCause,
        at: SampleTime,
        frames: u32,
        progress: Option<ScalarPointFaultProgress>,
    ) -> Result<T, ScalarPointRenderError> {
        let f = self.latch(cause, at, frames, progress);
        l.fill(0.0);
        r.fill(0.0);
        self.next_sample = SampleTime(end);
        Err(ScalarPointRenderError::Fault(f))
    }
    fn process(
        &mut self,
        l: &mut [f32],
        r: &mut [f32],
        first: SampleTime,
    ) -> Result<ProcessReport, ProcessBlockError> {
        EffectProcessBlock::new(l, r, None, first.0, &[], self.quantum)
            .map(|b| self.processor.process(b))
    }
    fn render_inner(
        &mut self,
        l: &mut [f32],
        r: &mut [f32],
        first: SampleTime,
    ) -> Result<ScalarPointRenderReport, ScalarPointRenderError> {
        if l.len() != self.quantum as usize || r.len() != l.len() {
            return Err(ScalarPointRenderError::InvalidShape);
        }
        if first != self.next_sample {
            return Err(ScalarPointRenderError::DiscontinuousTime);
        }
        let end = first
            .0
            .checked_add(u64::from(self.quantum))
            .ok_or(ScalarPointRenderError::SampleOverflow)?;
        if let Some(f) = self.fault {
            l.fill(0.0);
            r.fill(0.0);
            return Err(ScalarPointRenderError::Fault(f));
        }
        let (mut report, mut inv, mut cursor, mut processed) =
            (ProcessReport::default(), 0u32, 0usize, 0u32);
        self.refresh(first);
        if let Some((ticket, mut delivered, count, _)) = self.claimed {
            let mut native = delivered;
            while native < count {
                let rec = match self.delivery.pending(ticket) {
                    Ok(p) => p.records[usize::from(native)],
                    Err(error) => {
                        return self.fail(
                            l,
                            r,
                            end,
                            ScalarPointFaultCause::Delivery {
                                operation: ScalarPointDeliveryOperation::Pending,
                                error,
                            },
                            SampleTime(first.0 + cursor as u64),
                            processed,
                            Some(ScalarPointFaultProgress {
                                ticket,
                                record_count: count,
                                native_applied_prefix: native,
                                delivery_applied_prefix: delivered,
                            }),
                        );
                    }
                };
                let off =
                    usize::try_from(rec.start.0.saturating_sub(first.0)).unwrap_or(usize::MAX);
                if off >= l.len() {
                    self.claimed = Some((ticket, delivered, count, Some(rec.start)));
                    break;
                }
                if off > cursor {
                    let at = SampleTime(first.0 + cursor as u64);
                    match self.process(&mut l[cursor..off], &mut r[cursor..off], at) {
                        Ok(x) => {
                            report = add_report(report, x);
                            inv += 1;
                            processed += (off - cursor) as u32;
                            cursor = off
                        }
                        Err(e) => {
                            return self.fail(
                                l,
                                r,
                                end,
                                ScalarPointFaultCause::ProcessEnvelope(e),
                                at,
                                processed,
                                Some(ScalarPointFaultProgress {
                                    ticket,
                                    record_count: count,
                                    native_applied_prefix: native,
                                    delivery_applied_prefix: delivered,
                                }),
                            );
                        }
                    }
                }
                let (ch, ix) = if rec.handle == self.handles[0] {
                    (ParameterChannel::Left, 0)
                } else {
                    (ParameterChannel::Right, 1)
                };
                let at = SampleTime(rec.start.0.max(first.0));
                if let Err(error) =
                    self.processor
                        .apply_parameter_point(MAKEUP_INDEX, ch, rec.start_value)
                {
                    return self.fail(
                        l,
                        r,
                        end,
                        ScalarPointFaultCause::Native {
                            operation: ScalarPointNativeOperation::Apply,
                            channel: ch,
                            error,
                        },
                        at,
                        processed,
                        Some(ScalarPointFaultProgress {
                            ticket,
                            record_count: count,
                            native_applied_prefix: native,
                            delivery_applied_prefix: delivered,
                        }),
                    );
                }
                native += 1;
                self.applied = self.applied.saturating_add(1);
                self.last_application[ix] = Some(at);
                if rec.start.0 < first.0 {
                    self.late = self.late.saturating_add(1)
                }
                if let Err(error) = self.delivery.mark_applied(ticket, native) {
                    return self.fail(
                        l,
                        r,
                        end,
                        ScalarPointFaultCause::Delivery {
                            operation: ScalarPointDeliveryOperation::MarkApplied,
                            error,
                        },
                        at,
                        processed,
                        Some(ScalarPointFaultProgress {
                            ticket,
                            record_count: count,
                            native_applied_prefix: native,
                            delivery_applied_prefix: delivered,
                        }),
                    );
                }
                delivered = native;
                let next = match self.delivery.pending(ticket) {
                    Ok(p) => p.records.get(usize::from(delivered)).map(|r| r.start),
                    Err(error) => {
                        return self.fail(
                            l,
                            r,
                            end,
                            ScalarPointFaultCause::Delivery {
                                operation: ScalarPointDeliveryOperation::Pending,
                                error,
                            },
                            SampleTime(first.0 + cursor as u64),
                            processed,
                            Some(ScalarPointFaultProgress {
                                ticket,
                                record_count: count,
                                native_applied_prefix: native,
                                delivery_applied_prefix: delivered,
                            }),
                        );
                    }
                };
                self.claimed = Some((ticket, delivered, count, next));
            }
            if delivered == count {
                match self.delivery.finish_applied(ticket, delivered) {
                    Ok(()) => self.claimed = None,
                    Err(DeliveryError::Full) => {
                        self.claimed = Some((ticket, delivered, count, None))
                    }
                    Err(error) => {
                        return self.fail(
                            l,
                            r,
                            end,
                            ScalarPointFaultCause::Delivery {
                                operation: ScalarPointDeliveryOperation::FinishApplied,
                                error,
                            },
                            SampleTime(first.0 + cursor as u64),
                            processed,
                            Some(ScalarPointFaultProgress {
                                ticket,
                                record_count: count,
                                native_applied_prefix: delivered,
                                delivery_applied_prefix: delivered,
                            }),
                        );
                    }
                }
            }
        }
        if cursor < l.len() {
            let at = SampleTime(first.0 + cursor as u64);
            match self.process(&mut l[cursor..], &mut r[cursor..], at) {
                Ok(x) => {
                    report = add_report(report, x);
                    inv += 1
                }
                Err(e) => {
                    let p = self.progress();
                    return self.fail(
                        l,
                        r,
                        end,
                        ScalarPointFaultCause::ProcessEnvelope(e),
                        at,
                        processed,
                        p,
                    );
                }
            }
        }
        self.next_sample = SampleTime(end);
        self.observed_sample = SampleTime(end);
        Ok(ScalarPointRenderReport {
            native: report,
            process_invocations: inv,
        })
    }
}
fn add_report(mut a: ProcessReport, b: ProcessReport) -> ProcessReport {
    a.sanitized_main_samples = a
        .sanitized_main_samples
        .saturating_add(b.sanitized_main_samples);
    a.sanitized_sidechain_samples = a
        .sanitized_sidechain_samples
        .saturating_add(b.sanitized_sidechain_samples);
    a.invalid_spans = a.invalid_spans.saturating_add(b.invalid_spans);
    a.nonfinite_left_blocks = a
        .nonfinite_left_blocks
        .saturating_add(b.nonfinite_left_blocks);
    a.nonfinite_right_blocks = a
        .nonfinite_right_blocks
        .saturating_add(b.nonfinite_right_blocks);
    a
}

/// Validates and prepares fixed Left/Right compressor makeup Point delivery.
///
/// `handles` must be distinct nonzero values in Left-then-Right order. `processor` remains
/// exclusively borrowed until the returned render owner is dropped. Preparation allocates only
/// the underlying bounded delivery service; endpoint bindings, counters, and capabilities are
/// inline. The returned resource report excludes the processor and caller-owned PCM.
pub fn prepare_scalar_point_endpoint<'a>(
    processor: &'a mut dyn PreparedNativeEffect,
    revision: SessionRevision,
    handles: [ParameterHandle; 2],
    queues: ProtocolQueueConfig,
    sequence: u64,
) -> Result<
    (
        ScalarPointControl,
        PreparedScalarPointRender<'a>,
        ScalarPointResources,
    ),
    ScalarPointPrepareError,
> {
    if handles[0].0 == 0 || handles[1].0 == 0 || handles[0] >= handles[1] {
        return Err(ScalarPointPrepareError::InvalidBindings);
    }
    let quantum = u32::try_from(queues.quantum_frames.get())
        .ok()
        .filter(|q| *q != 0)
        .ok_or(ScalarPointPrepareError::InvalidQueue)?;
    let m = processor.metadata();
    if m.descriptor.id.as_str() != "miso.compressor"
        || !engine::is_launch_sample_rate(engine::SampleRateHz(m.sample_rate))
        || m.quality != EffectQuality::Normal
        || m.link_mode != LinkMode::DualMono
        || m.bypass
        || m.quantum != quantum
        || !matches!(m.ports.sidechain,effect_contract::PreparedSidechainPort::Unconnected{id,required:false}if id.as_str()=="sidechain-in")
    {
        return Err(ScalarPointPrepareError::InvalidProcessor);
    }
    let p = m
        .descriptor
        .parameters
        .get(MAKEUP_INDEX as usize)
        .ok_or(ScalarPointPrepareError::InvalidProcessor)?;
    if p.id.0 != MAKEUP_ID
        || !p.readable
        || !p.automatable
        || p.unit != ParameterUnit::Db
        || p.domain != ParameterDomain::Continuous
        || p.channel_policy != ParameterChannelPolicy::PerLane
        || p.minimum != Some(-24.0)
        || p.maximum != Some(24.0)
        || p.smoothing != SmoothingRule::Linear
        || p.smoothing_samples != 64
    {
        return Err(ScalarPointPrepareError::InvalidProcessor);
    }
    {
        let _fp = CanonicalFpEnv::enter();
        processor
            .parameter_state(MAKEUP_INDEX, ParameterChannel::Left)
            .map_err(|_| ScalarPointPrepareError::InvalidProcessor)?;
        processor
            .parameter_state(MAKEUP_INDEX, ParameterChannel::Right)
            .map_err(|_| ScalarPointPrepareError::InvalidProcessor)?;
    }
    let caps = PreparedDeliveryCapabilities::new_exact(&[
        (handles[0], protocol::AutomationKind::Point),
        (handles[1], protocol::AutomationKind::Point),
    ])
    .ok_or(ScalarPointPrepareError::InvalidBindings)?;
    let resources = ScalarPointResources {
        delivery: PreparedAutomationDelivery::resource_report_for_config(queues)
            .map_err(ScalarPointPrepareError::Delivery)?,
        control_size: core::mem::size_of::<ScalarPointControl>(),
        render_size: core::mem::size_of::<PreparedScalarPointRender<'static>>(),
    };
    let (c, r) = PreparedAutomationDelivery::prepare(queues, sequence)
        .map_err(ScalarPointPrepareError::Delivery)?;
    Ok((
        ScalarPointControl {
            delivery: c,
            revision,
            handles,
            capabilities: caps,
        },
        PreparedScalarPointRender {
            processor,
            delivery: r,
            handles,
            quantum,
            next_sample: SampleTime(0),
            observed_sample: SampleTime(0),
            last_application: [None; 2],
            applied: 0,
            late: 0,
            claimed: None,
            fault: None,
        },
        resources,
    ))
}
