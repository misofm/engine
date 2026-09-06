//! Opt-in borrowed scalar compressor Point delivery.
use core::marker::PhantomData;

use effect_contract::{
    EffectProcessBlock, EffectQuality, LinkMode, ParameterAccessError, ParameterChannel,
    PreparedNativeEffect, PreparedParameterState, ProcessReport,
};
use lane::CanonicalFpEnv;
use protocol::{
    AutomationBatchSlot, AutomationCancellationReason, AutomationEnqueueError, DeliveryError,
    DeliveryTicket, HandoffResult, ParameterHandle, PreparedAutomationDelivery,
    PreparedDeliveryCapabilities, ProtocolQueueConfig, SampleTime, SessionRevision,
};

const MAKEUP_INDEX: u32 = 5;
const MAKEUP_ID: u32 = 6;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointPrepareError {
    InvalidProcessor,
    InvalidBindings,
    InvalidQueue,
    Delivery(protocol::ProtocolQueueError),
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointResources {
    pub delivery: protocol::DeliveryResourceReport,
    pub control_size: usize,
    pub render_size: usize,
}

#[allow(missing_docs)]
pub struct ScalarPointControl {
    delivery: protocol::AutomationDeliveryControl,
    revision: SessionRevision,
    handles: [ParameterHandle; 2],
}

#[allow(missing_docs)]
impl ScalarPointControl {
    pub fn try_admit(
        &mut self,
        current_sample: SampleTime,
        batch: AutomationBatchSlot,
    ) -> Result<(), AutomationEnqueueError> {
        if let Err(error) = batch.validate_records() {
            return Err(AutomationEnqueueError::Invalid { batch, error });
        }
        if batch.revision != self.revision {
            return Err(AutomationEnqueueError::Invalid {
                batch,
                error: protocol::AutomationBatchError::GlobalTimeBackwards,
            });
        }
        for record in batch.as_slice() {
            if !self.handles.contains(&record.handle)
                || !(-24.0..=24.0).contains(&record.start_value)
                || !(-24.0..=24.0).contains(&record.end_value)
            {
                return Err(AutomationEnqueueError::Invalid {
                    batch,
                    error: protocol::AutomationBatchError::InvalidRange,
                });
            }
        }
        self.delivery.try_admit(current_sample, batch)
    }

    pub fn try_handoff_next(&mut self) -> Result<HandoffResult, DeliveryError> {
        let capabilities = PreparedDeliveryCapabilities::new_exact(&[
            (self.handles[0], protocol::AutomationKind::Point),
            (self.handles[1], protocol::AutomationKind::Point),
        ])
        .expect("fixed capability");
        self.delivery.try_handoff_next(&capabilities)
    }
    pub fn collect_terminal(
        &mut self,
        ticket: DeliveryTicket,
    ) -> Result<protocol::TerminalAutomation, DeliveryError> {
        self.delivery.collect_terminal(ticket)
    }
    pub fn begin_cancel(
        &mut self,
        reason: AutomationCancellationReason,
    ) -> Result<protocol::CancelToken, DeliveryError> {
        self.delivery.begin_cancel(reason, self.revision)
    }
    pub fn poll_cancel_boundary(
        &mut self,
        token: protocol::CancelToken,
    ) -> Result<Option<protocol::CancelComplete>, DeliveryError> {
        self.delivery.poll_cancel_boundary(token)
    }
    pub fn try_dequeue_event(
        &mut self,
    ) -> Result<protocol::ReliableSlot, engine::realtime::QueueEmpty> {
        self.delivery.try_dequeue_event()
    }
    pub fn outstanding(&self) -> usize {
        self.delivery.outstanding()
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalarPointSnapshot {
    pub next_sample: SampleTime,
    pub observed_sample: SampleTime,
    pub state: [PreparedParameterState; 2],
    pub last_application: [Option<SampleTime>; 2],
    pub applied: u64,
    pub late: u64,
    pub pending: Option<(DeliveryTicket, u16, u16, Option<SampleTime>)>,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarPointRenderError {
    InvalidShape,
    DiscontinuousTime,
    SampleOverflow,
    Native(ParameterAccessError),
    Delivery(DeliveryError),
}

#[allow(missing_docs)]
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
    fault: Option<ScalarPointRenderError>,
    _thread: PhantomData<*const ()>,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarPointRenderReport {
    pub native: ProcessReport,
    pub process_invocations: u32,
}

#[allow(missing_docs)]
pub struct StartedScalarPointRender<'a>(PreparedScalarPointRender<'a>);

#[allow(missing_docs)]
impl<'a> PreparedScalarPointRender<'a> {
    pub fn start(
        self,
    ) -> Result<StartedScalarPointRender<'a>, (Self, lane::fpenv::FpEnvironmentRejection)> {
        match lane::attest_fp_environment() {
            Ok(()) => Ok(StartedScalarPointRender(self)),
            Err(error) => Err((self, error)),
        }
    }
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointRenderError> {
        self.snapshot_inner()
    }
}

#[allow(missing_docs)]
impl<'a> StartedScalarPointRender<'a> {
    pub fn render(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first_sample: SampleTime,
    ) -> Result<ScalarPointRenderReport, ScalarPointRenderError> {
        let _fp = CanonicalFpEnv::enter();
        self.0.render_inner(left, right, first_sample)
    }
    pub fn snapshot(&mut self) -> Result<ScalarPointSnapshot, ScalarPointRenderError> {
        let _fp = CanonicalFpEnv::enter();
        self.0.snapshot_inner()
    }
    pub fn cancel_boundary(&mut self, first: SampleTime) -> Result<(), ScalarPointRenderError> {
        if self.0.fault.is_none() {
            return Err(ScalarPointRenderError::Delivery(DeliveryError::Unsupported));
        }
        if first != self.0.next_sample {
            return Err(ScalarPointRenderError::DiscontinuousTime);
        }
        let end = first
            .0
            .checked_add(u64::from(self.0.quantum))
            .ok_or(ScalarPointRenderError::SampleOverflow)?;
        let _ = self.0.delivery.begin_boundary(first);
        self.0.next_sample = SampleTime(end);
        Ok(())
    }
    pub fn stop(self) -> PreparedScalarPointRender<'a> {
        self.0
    }
}

impl PreparedScalarPointRender<'_> {
    fn snapshot_inner(&mut self) -> Result<ScalarPointSnapshot, ScalarPointRenderError> {
        if let Some(error) = self.fault {
            return Err(error);
        }
        let state = [
            self.processor
                .parameter_state(MAKEUP_INDEX, ParameterChannel::Left)
                .map_err(ScalarPointRenderError::Native)?,
            self.processor
                .parameter_state(MAKEUP_INDEX, ParameterChannel::Right)
                .map_err(ScalarPointRenderError::Native)?,
        ];
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
    fn render_inner(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first: SampleTime,
    ) -> Result<ScalarPointRenderReport, ScalarPointRenderError> {
        if left.len() != usize::try_from(self.quantum).unwrap_or(usize::MAX)
            || right.len() != left.len()
        {
            return Err(ScalarPointRenderError::InvalidShape);
        }
        if first != self.next_sample {
            return Err(ScalarPointRenderError::DiscontinuousTime);
        }
        let end = first
            .0
            .checked_add(u64::from(self.quantum))
            .ok_or(ScalarPointRenderError::SampleOverflow)?;
        if let Some(error) = self.fault {
            left.fill(0.0);
            right.fill(0.0);
            return Err(error);
        }
        let mut report = ProcessReport::default();
        let mut invocations = 0;
        let mut cursor = 0usize;
        let pending = self.delivery.begin_boundary(first).map(|pending| {
            (
                pending.ticket,
                pending.applied_prefix,
                pending.records.len() as u16,
            )
        });
        if let Some((ticket, mut prefix, record_count)) = pending {
            self.claimed = Some((ticket, prefix, record_count, None));
            while prefix < record_count {
                let record = self
                    .delivery
                    .pending(ticket)
                    .map_err(ScalarPointRenderError::Delivery)?
                    .records[usize::from(prefix)];
                let offset =
                    usize::try_from(record.start.0.saturating_sub(first.0)).unwrap_or(usize::MAX);
                if offset >= left.len() {
                    break;
                }
                if offset > cursor {
                    let r = self.process(
                        &mut left[cursor..offset],
                        &mut right[cursor..offset],
                        first.0 + cursor as u64,
                        &mut report,
                    );
                    report = add_report(report, r);
                    invocations += 1;
                    cursor = offset;
                }
                let channel = if record.handle == self.handles[0] {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                };
                self.processor
                    .apply_parameter_point(MAKEUP_INDEX, channel, record.start_value)
                    .map_err(ScalarPointRenderError::Native)?;
                self.last_application[if channel == ParameterChannel::Left {
                    0
                } else {
                    1
                }] = Some(record.start);
                self.applied = self.applied.saturating_add(1);
                if record.start.0 < first.0 {
                    self.late = self.late.saturating_add(1);
                }
                prefix += 1;
                self.delivery
                    .mark_applied(ticket, prefix)
                    .map_err(ScalarPointRenderError::Delivery)?;
                let next_unapplied_sample = self
                    .delivery
                    .pending(ticket)
                    .ok()
                    .and_then(|p| p.records.get(usize::from(prefix)).map(|r| r.start));
                self.claimed = Some((ticket, prefix, record_count, next_unapplied_sample));
            }
            if prefix == record_count {
                if let Err(error) = self.delivery.finish_applied(ticket, prefix) {
                    if error != DeliveryError::Full {
                        return Err(ScalarPointRenderError::Delivery(error));
                    }
                }
                if self.delivery.pending(ticket).is_err() {
                    self.claimed = None;
                }
            }
        }
        let r = self.process(
            &mut left[cursor..],
            &mut right[cursor..],
            first.0 + cursor as u64,
            &mut report,
        );
        report = add_report(report, r);
        invocations += 1;
        self.next_sample = SampleTime(end);
        self.observed_sample = first;
        Ok(ScalarPointRenderReport {
            native: report,
            process_invocations: invocations,
        })
    }
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first: u64,
        _total: &mut ProcessReport,
    ) -> ProcessReport {
        // The caller proves nonempty, equal-length slices and a checked first/end pair before
        // entering this helper. Keep construction explicit so a future contract change cannot
        // turn a render-path invariant into a panic.
        let Ok(block) = EffectProcessBlock::new(left, right, None, first, &[], self.quantum) else {
            return ProcessReport {
                invalid_spans: 1,
                ..ProcessReport::default()
            };
        };
        self.processor.process(block)
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

#[allow(missing_docs)]
pub fn prepare_scalar_point_endpoint<'a>(
    processor: &'a mut dyn PreparedNativeEffect,
    revision: SessionRevision,
    handles: [ParameterHandle; 2],
    queues: ProtocolQueueConfig,
    initial_reliable_event_sequence: u64,
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
    let metadata = processor.metadata();
    if metadata.descriptor.id.as_str() != "miso.compressor"
        || !engine::is_launch_sample_rate(engine::SampleRateHz(metadata.sample_rate))
        || metadata.quality != EffectQuality::Normal
        || metadata.link_mode != LinkMode::DualMono
        || metadata.bypass
        || metadata.quantum != u32::try_from(queues.quantum_frames.get()).unwrap_or(0)
        || !matches!(metadata.ports.sidechain, effect_contract::PreparedSidechainPort::Unconnected { id, required: false } if id.as_str() == "sidechain-in")
    {
        return Err(ScalarPointPrepareError::InvalidProcessor);
    }
    let p = metadata
        .descriptor
        .parameters
        .get(MAKEUP_INDEX as usize)
        .ok_or(ScalarPointPrepareError::InvalidProcessor)?;
    if p.id.0 != MAKEUP_ID
        || !p.readable
        || !p.automatable
        || p.minimum != Some(-24.0)
        || p.maximum != Some(24.0)
        || p.smoothing_samples != 64
    {
        return Err(ScalarPointPrepareError::InvalidProcessor);
    }
    processor
        .parameter_state(MAKEUP_INDEX, ParameterChannel::Left)
        .map_err(|_| ScalarPointPrepareError::InvalidProcessor)?;
    processor
        .parameter_state(MAKEUP_INDEX, ParameterChannel::Right)
        .map_err(|_| ScalarPointPrepareError::InvalidProcessor)?;
    let (control, render) =
        PreparedAutomationDelivery::prepare(queues, initial_reliable_event_sequence)
            .map_err(ScalarPointPrepareError::Delivery)?;
    let resources = ScalarPointResources {
        delivery: PreparedAutomationDelivery::resource_report_for_config(queues)
            .map_err(ScalarPointPrepareError::Delivery)?,
        control_size: core::mem::size_of::<ScalarPointControl>(),
        render_size: core::mem::size_of::<PreparedScalarPointRender<'static>>(),
    };
    Ok((
        ScalarPointControl {
            delivery: control,
            revision,
            handles,
        },
        PreparedScalarPointRender {
            processor,
            delivery: render,
            handles,
            quantum: queues.quantum_frames.get() as u32,
            next_sample: SampleTime(0),
            observed_sample: SampleTime(0),
            last_application: [None; 2],
            applied: 0,
            late: 0,
            claimed: None,
            fault: None,
            _thread: PhantomData,
        },
        resources,
    ))
}
