//! Borrowed scalar-compressor makeup Point delivery.

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

/// Preparation rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointPrepareError {
    InvalidProcessor,
    InvalidBindings,
    InvalidQueue,
    Delivery(protocol::ProtocolQueueError),
}
/// Delivery heap and endpoint inline accounting; excludes borrowed compressor and caller PCM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ScalarPointResources {
    pub delivery: protocol::DeliveryResourceReport,
    pub control_size: usize,
    pub render_size: usize,
}
/// Endpoint admission rejection retaining the complete batch.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointAdmissionError {
    InvalidBatch {
        batch: AutomationBatchSlot,
        error: AutomationBatchError,
    },
    WrongRevision {
        batch: AutomationBatchSlot,
    },
    UnknownBinding {
        batch: AutomationBatchSlot,
    },
    InvalidValue {
        batch: AutomationBatchSlot,
    },
    Service(AutomationEnqueueError),
}

/// Control-thread owner of admission, handoff, terminal credit, and cancellation.
pub struct ScalarPointControl {
    delivery: protocol::AutomationDeliveryControl,
    revision: SessionRevision,
    handles: [ParameterHandle; 2],
    capabilities: PreparedDeliveryCapabilities,
}
#[allow(missing_docs)]
impl ScalarPointControl {
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
    pub fn try_handoff_next(&mut self) -> Result<HandoffResult, DeliveryError> {
        self.delivery.try_handoff_next(&self.capabilities)
    }
    pub fn collect_terminal(
        &mut self,
        t: DeliveryTicket,
    ) -> Result<protocol::TerminalAutomation, DeliveryError> {
        self.delivery.collect_terminal(t)
    }
    pub fn begin_cancel(
        &mut self,
        r: AutomationCancellationReason,
    ) -> Result<protocol::CancelToken, DeliveryError> {
        self.delivery.begin_cancel(r, self.revision)
    }
    pub fn poll_cancel_boundary(
        &mut self,
        t: protocol::CancelToken,
    ) -> Result<Option<protocol::CancelComplete>, DeliveryError> {
        self.delivery.poll_cancel_boundary(t)
    }
    pub fn try_dequeue_event(
        &mut self,
    ) -> Result<protocol::ReliableSlot, engine::realtime::QueueEmpty> {
        self.delivery.try_dequeue_event()
    }
    pub fn outstanding(&self) -> usize {
        self.delivery.outstanding()
    }
    pub fn resident_automation(&self) -> u64 {
        self.delivery.resident_automation()
    }
    pub fn automation_status(&self) -> QueueReport {
        self.delivery.queues().report(QueueKind::Automation)
    }
}

/// Between-block native state and the sole bounded render claim.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ScalarPointSnapshot {
    pub next_sample: SampleTime,
    pub observed_sample: SampleTime,
    pub state: [PreparedParameterState; 2],
    pub last_application: [Option<SampleTime>; 2],
    pub applied: u64,
    pub late: u64,
    pub pending: Option<(DeliveryTicket, u16, u16, Option<SampleTime>)>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointNativeOperation {
    Read,
    Apply,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointDeliveryOperation {
    Pending,
    MarkApplied,
    FinishApplied,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointFaultCause {
    Native {
        operation: ScalarPointNativeOperation,
        channel: ParameterChannel,
        error: ParameterAccessError,
    },
    Delivery {
        operation: ScalarPointDeliveryOperation,
        error: DeliveryError,
    },
    ProcessEnvelope(ProcessBlockError),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ScalarPointFaultProgress {
    pub ticket: DeliveryTicket,
    pub record_count: u16,
    pub native_applied_prefix: u16,
    pub delivery_applied_prefix: u16,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ScalarPointFault {
    pub cause: ScalarPointFaultCause,
    pub at_sample: SampleTime,
    pub native_processed_frames: u32,
    pub progress: Option<ScalarPointFaultProgress>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointRenderError {
    InvalidShape,
    DiscontinuousTime,
    SampleOverflow,
    Fault(ScalarPointFault),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ScalarPointCancelBoundaryError {
    NotFaulted,
    DiscontinuousTime,
    SampleOverflow,
}

/// Transferable unstarted owner holding the exclusive processor borrow.
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ScalarPointRenderReport {
    pub native: ProcessReport,
    pub process_invocations: u32,
}
/// Thread-affine started owner.
pub struct StartedScalarPointRender<'a> {
    inner: PreparedScalarPointRender<'a>,
    _thread: PhantomData<*const ()>,
}
#[allow(missing_docs)]
impl<'a> PreparedScalarPointRender<'a> {
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
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointFault> {
        let _fp = CanonicalFpEnv::enter();
        self.snapshot_inner()
    }
    pub const fn fault(&self) -> Option<ScalarPointFault> {
        self.fault
    }
}
#[allow(missing_docs)]
impl<'a> StartedScalarPointRender<'a> {
    pub fn render(
        &mut self,
        l: &mut [f32],
        r: &mut [f32],
        first: SampleTime,
    ) -> Result<ScalarPointRenderReport, ScalarPointRenderError> {
        let _fp = CanonicalFpEnv::enter();
        self.inner.render_inner(l, r, first)
    }
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointFault> {
        let _fp = CanonicalFpEnv::enter();
        self.inner.snapshot_inner()
    }
    pub const fn fault(&self) -> Option<ScalarPointFault> {
        self.inner.fault
    }
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
                let next = self
                    .delivery
                    .pending(ticket)
                    .ok()
                    .and_then(|p| p.records.get(usize::from(delivered)).map(|r| r.start));
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

/// Prepares fixed Left/Right makeup bindings and the bounded ownership service.
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
