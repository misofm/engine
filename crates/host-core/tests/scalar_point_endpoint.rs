//! Product gates for borrowed scalar compressor Point delivery.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;
use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ObservationSample, ParameterAccessError, ParameterChannel,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedParameterState, PreparedPorts, PreparedSidechainPort,
    ProcessReport, ResetKind, StatePayloadError, StatePayloadInput, StatePayloadOutput,
};
use host_core::{
    ScalarPointAdmissionError, ScalarPointCancelBoundaryError, ScalarPointFault,
    ScalarPointFaultCause, ScalarPointFaultProgress, ScalarPointNativeOperation,
    ScalarPointRenderError, prepare_scalar_point_endpoint,
};
use protocol::{
    AutomationBatchError, AutomationBatchSlot, AutomationCancellationReason,
    AutomationEnqueueError, AutomationKind, AutomationRecord, DeliveryError, HandoffResult,
    ParameterHandle, ProtocolQueueConfig, ReliablePayload, RequestId, SampleTime, SessionRevision,
};

const Q: usize = 16;
const REV: SessionRevision = SessionRevision(7);
const H: [ParameterHandle; 2] = [ParameterHandle(11), ParameterHandle(12)];
fn config() -> ProtocolQueueConfig {
    ProtocolQueueConfig {
        control_command_slots: NonZeroUsize::new(2).unwrap(),
        control_command_bytes: NonZeroUsize::new(256).unwrap(),
        automation_batch_slots: NonZeroUsize::new(2).unwrap(),
        reliable_response_slots: NonZeroUsize::new(2).unwrap(),
        reliable_event_slots: NonZeroUsize::new(4).unwrap(),
        telemetry_slots: NonZeroUsize::new(2).unwrap(),
        per_block_automation_density: NonZeroUsize::new(256).unwrap(),
        quantum_frames: NonZeroUsize::new(Q).unwrap(),
    }
}
fn effect() -> Box<dyn PreparedNativeEffect> {
    let values: Vec<_> = compressor::COMPRESSOR_PARAMETERS
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            [ParameterChannel::Left, ParameterChannel::Right].map(move |channel| {
                InitialParameterValue {
                    parameter_index: i as u32,
                    channel,
                    value: p.default_value,
                }
            })
        })
        .collect();
    compressor::CompressorFactory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: Q as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: effect_contract::PortId::new("sidechain-in").unwrap(),
                    required: false,
                },
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 12,
                maximum_automation_spans_per_block: 256,
            },
        })
        .unwrap()
}
fn asymmetric_effect() -> Box<dyn PreparedNativeEffect> {
    let pairs = [
        (-31.0, -13.0),
        (3.0, 7.0),
        (4.0, 10.0),
        (8.0, 30.0),
        (80.0, 300.0),
        (-3.0, 4.0),
        (0.25, 0.75),
        (10.0, 15.0),
    ];
    let values: Vec<_> = pairs
        .iter()
        .enumerate()
        .flat_map(|(parameter, &(left, right))| {
            [
                (ParameterChannel::Left, left),
                (ParameterChannel::Right, right),
            ]
            .map(move |(channel, value)| InitialParameterValue {
                parameter_index: parameter as u32,
                channel,
                value,
            })
        })
        .collect();
    compressor::CompressorFactory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: Q as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: effect_contract::PortId::new("sidechain-in").unwrap(),
                    required: false,
                },
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 12,
                maximum_automation_spans_per_block: 256,
            },
        })
        .unwrap()
}
fn signal(frames: usize, seed: u32, scale: f32) -> Vec<f32> {
    let mut state = seed;
    (0..frames)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((state >> 8) as f32 / 8_388_608.0 - 1.0) * scale
        })
        .collect()
}
fn process(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first: u64,
    spans: &[PreparedAutomationSpan],
) -> ProcessReport {
    effect.process(EffectProcessBlock::new(left, right, None, first, spans, Q as u32).unwrap())
}
fn add_report(total: &mut ProcessReport, report: ProcessReport) {
    total.sanitized_main_samples += report.sanitized_main_samples;
    total.sanitized_sidechain_samples += report.sanitized_sidechain_samples;
    total.invalid_spans += report.invalid_spans;
    total.nonfinite_left_blocks += report.nonfinite_left_blocks;
    total.nonfinite_right_blocks += report.nonfinite_right_blocks;
}
fn warm(effect: &mut dyn PreparedNativeEffect) {
    const FRAMES: usize = 1_152;
    let mut left = signal(FRAMES, 0x0528_1001, 0.73);
    let mut right = signal(FRAMES, 0x0528_1002, 0.61);
    for (block, (left, right)) in left.chunks_mut(Q).zip(right.chunks_mut(Q)).enumerate() {
        process(effect, left, right, (block * Q) as u64, &[]);
    }
}
fn point_span(at: u64, channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: 5,
        start_sample: at,
        end_sample: at,
        start_value: value,
        end_value: value,
    }
}
fn payload(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>) {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0; sizes.left_bytes as usize];
    let mut right = vec![0; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).unwrap(),
        )
        .unwrap();
    (left, right)
}
fn resident(effect: &dyn PreparedNativeEffect) -> ObservationSample {
    let mut result = ObservationSample::default();
    assert!(effect.observe_resident(0, &mut result));
    result
}
fn assert_pcm_bits(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (sample, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(actual.to_bits(), expected.to_bits(), "PCM sample {sample}");
    }
}
fn assert_state_bits(actual: PreparedParameterState, expected: PreparedParameterState) {
    assert_eq!(
        actual.current_value.to_bits(),
        expected.current_value.to_bits()
    );
    assert_eq!(
        actual.target_value.to_bits(),
        expected.target_value.to_bits()
    );
}
fn assert_resident_bits(actual: ObservationSample, expected: ObservationSample) {
    assert_eq!(actual.left.to_bits(), expected.left.to_bits());
    assert_eq!(actual.right.to_bits(), expected.right.to_bits());
}
struct FailingAccessEffect {
    inner: Box<dyn PreparedNativeEffect>,
    fail_apply_channel: Option<ParameterChannel>,
    fail_read_after_process: bool,
    processed_frames: u64,
    apply_calls: u64,
    read_calls: core::cell::Cell<u64>,
}
impl FailingAccessEffect {
    fn apply_failure() -> Self {
        Self {
            inner: asymmetric_effect(),
            fail_apply_channel: Some(ParameterChannel::Right),
            fail_read_after_process: false,
            processed_frames: 0,
            apply_calls: 0,
            read_calls: core::cell::Cell::new(0),
        }
    }
    fn read_failure() -> Self {
        Self {
            inner: asymmetric_effect(),
            fail_apply_channel: None,
            fail_read_after_process: true,
            processed_frames: 0,
            apply_calls: 0,
            read_calls: core::cell::Cell::new(0),
        }
    }
}
impl PreparedNativeEffect for FailingAccessEffect {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.inner.metadata()
    }
    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        self.processed_frames += block.frames() as u64;
        self.inner.process(block)
    }
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        self.inner.observe_resident(tap_index, out)
    }
    fn apply_parameter_point(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), ParameterAccessError> {
        self.apply_calls += 1;
        if self.processed_frames != 0 && self.fail_apply_channel == Some(channel) {
            Err(ParameterAccessError::Unsupported)
        } else {
            self.inner
                .apply_parameter_point(parameter_index, channel, value)
        }
    }
    fn parameter_state(
        &self,
        parameter_index: u32,
        channel: ParameterChannel,
    ) -> Result<PreparedParameterState, ParameterAccessError> {
        self.read_calls.set(self.read_calls.get() + 1);
        if self.processed_frames != 0 && self.fail_read_after_process {
            Err(ParameterAccessError::Unsupported)
        } else {
            self.inner.parameter_state(parameter_index, channel)
        }
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.snapshot_state_payload(output)
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.restore_state_payload(version, input)
    }
    fn channel_symmetry(&self) -> bool {
        self.inner.channel_symmetry()
    }
}
fn pcm_case(records: &[AutomationRecord], splits: &[(usize, &[PreparedAutomationSpan])]) {
    let _fp = lane::CanonicalFpEnv::enter();
    let (mut endpoint_effect, mut reference_effect, mut no_event_effect) = (
        asymmetric_effect(),
        asymmetric_effect(),
        asymmetric_effect(),
    );
    warm(&mut *endpoint_effect);
    warm(&mut *reference_effect);
    warm(&mut *no_event_effect);
    assert_eq!(payload(&*endpoint_effect), payload(&*reference_effect));
    assert_eq!(payload(&*endpoint_effect), payload(&*no_event_effect));

    let (mut control, render, _) =
        prepare_scalar_point_endpoint(&mut *endpoint_effect, REV, H, config(), 1).unwrap();
    control.try_admit(SampleTime(0), batch(1, records)).unwrap();
    let ticket = match control.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        _ => panic!(),
    };
    let mut render = render.start().unwrap_or_else(|_| panic!());
    let mut actual_left = signal(Q, 0x0528_2001, 0.69);
    let mut actual_right = signal(Q, 0x0528_2002, 0.57);
    let mut expected_left = actual_left.clone();
    let mut expected_right = actual_right.clone();
    let mut unchanged_left = actual_left.clone();
    let mut unchanged_right = actual_right.clone();
    let endpoint_report = render
        .render(&mut actual_left, &mut actual_right, SampleTime(0))
        .unwrap();
    let snapshot = render.snapshot().unwrap();
    assert_eq!(snapshot.applied, records.len() as u64);
    assert_eq!(snapshot.next_sample, SampleTime(Q as u64));
    assert_eq!(snapshot.observed_sample, SampleTime(Q as u64));
    for (index, handle) in H.into_iter().enumerate() {
        assert_eq!(
            snapshot.last_application[index],
            records
                .iter()
                .rev()
                .find(|record| record.handle == handle)
                .map(|record| record.start)
        );
    }

    let mut reference_report = ProcessReport::default();
    let mut begin = 0;
    for &(end, spans) in splits {
        add_report(
            &mut reference_report,
            process(
                &mut *reference_effect,
                &mut expected_left[begin..end],
                &mut expected_right[begin..end],
                begin as u64,
                spans,
            ),
        );
        begin = end;
    }
    assert_eq!(begin, Q);
    let no_event_report = process(
        &mut *no_event_effect,
        &mut unchanged_left,
        &mut unchanged_right,
        0,
        &[],
    );

    assert_eq!(endpoint_report.native, reference_report);
    assert_eq!(endpoint_report.process_invocations as usize, splits.len());
    assert_pcm_bits(&actual_left, &expected_left);
    assert_pcm_bits(&actual_right, &expected_right);
    assert!(
        actual_left
            .iter()
            .zip(&unchanged_left)
            .chain(actual_right.iter().zip(&unchanged_right))
            .any(|(&actual, &unchanged)| actual.to_bits() != unchanged.to_bits()),
        "warmed Point render matched the no-event PCM after compressor lookahead"
    );
    assert_eq!(no_event_report, ProcessReport::default());
    for (index, channel) in [ParameterChannel::Left, ParameterChannel::Right]
        .into_iter()
        .enumerate()
    {
        let expected = reference_effect.parameter_state(5, channel).unwrap();
        assert_state_bits(snapshot.state[index], expected);
    }
    assert_eq!(
        control.collect_terminal(ticket).unwrap().applied_prefix,
        records.len() as u16
    );
    drop(render.stop());
    assert_resident_bits(resident(&*endpoint_effect), resident(&*reference_effect));
    assert_eq!(payload(&*endpoint_effect), payload(&*reference_effect));
}
fn record(handle: ParameterHandle, time: u64, value: f32) -> AutomationRecord {
    AutomationRecord {
        kind: AutomationKind::Point,
        handle,
        start: SampleTime(time),
        end: SampleTime(time),
        start_value: value,
        end_value: value,
    }
}
fn batch(id: u64, records: &[AutomationRecord]) -> AutomationBatchSlot {
    AutomationBatchSlot::new(REV, RequestId::new(id).unwrap(), records).unwrap()
}
#[test]
fn points_slice_real_pcm_and_readback() {
    let left_3 = [point_span(3, ParameterChannel::Left, 6.0)];
    let right_7 = [point_span(7, ParameterChannel::Right, -6.0)];
    pcm_case(
        &[record(H[0], 3, 6.0), record(H[1], 7, -6.0)],
        &[(3, &[]), (7, &left_3), (Q, &right_7)],
    );

    let simultaneous = [
        point_span(7, ParameterChannel::Left, 9.0),
        point_span(7, ParameterChannel::Right, -9.0),
    ];
    pcm_case(
        &[record(H[0], 7, 9.0), record(H[1], 7, -9.0)],
        &[(7, &[]), (Q, &simultaneous)],
    );
}

#[test]
fn claim_state_is_truthful_until_terminal_collection() {
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(2, &[record(H[0], 8, 4.0), record(H[1], 16, 5.0)]),
    )
    .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(t) => t,
        _ => panic!(),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.2; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let s = r.snapshot().unwrap();
    assert_eq!(s.observed_sample, SampleTime(16));
    assert_eq!(s.pending.unwrap().1, 1);
    assert_eq!(s.pending.unwrap().3, Some(SampleTime(16)));
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::Empty));
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    assert_eq!(c.outstanding(), 1);
    c.collect_terminal(ticket).unwrap();
    assert_eq!(c.outstanding(), 0);
}

#[test]
fn late_points_apply_in_order_and_second_ticket_waits() {
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(3, &[record(H[0], 1, 1.0), record(H[0], 2, 2.0)]),
    )
    .unwrap();
    c.try_admit(SampleTime(0), batch(4, &[record(H[1], 3, 3.0)]))
        .unwrap();
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.2; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    c.try_handoff_next().unwrap();
    c.try_handoff_next().unwrap();
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    let s = r.snapshot().unwrap();
    assert_eq!((s.applied, s.late), (2, 2));
    assert_eq!(s.last_application[0], Some(SampleTime(16)));
    r.render(&mut l, &mut x, SampleTime(32)).unwrap();
    let s = r.snapshot().unwrap();
    assert_eq!((s.applied, s.late), (3, 3));
    assert_eq!(s.last_application[1], Some(SampleTime(32)));
    r.render(&mut l, &mut x, SampleTime(48)).unwrap();
    assert_eq!(
        (r.snapshot().unwrap().applied, r.snapshot().unwrap().late),
        (3, 3)
    );
}

#[test]
fn real_cancellation_preserves_applied_prefix() {
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(5, &[record(H[0], 3, 2.0), record(H[1], 30, 3.0)]),
    )
    .unwrap();
    c.try_handoff_next().unwrap();
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.2; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    let token = c
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!((done.applied_records, done.canceled_records), (1, 1));
    assert_eq!(
        r.cancel_boundary(SampleTime(32)),
        Err(ScalarPointCancelBoundaryError::NotFaulted)
    );
}

#[test]
fn malformed_admission_and_render_envelopes_are_noops() {
    let _fp = lane::CanonicalFpEnv::enter();
    let mut fx = effect();
    let (mut c, mut r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    let pristine = r.snapshot().unwrap();
    let mut reject =
        |candidate: AutomationBatchSlot,
         expected: fn(Result<(), ScalarPointAdmissionError>) -> bool| {
            assert!(expected(c.try_admit(SampleTime(10), candidate)));
            assert_eq!(c.outstanding(), 0);
            assert_eq!(c.resident_automation(), 0);
            assert_eq!(r.snapshot().unwrap(), pristine);
        };

    let mut empty = batch(60, &[record(H[0], 10, 1.0)]);
    empty.len = 0;
    reject(empty, |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::InvalidBatch {
                error: AutomationBatchError::EmptyBatch,
                ..
            })
        )
    });
    let mut too_long = batch(61, &[record(H[0], 10, 1.0)]);
    too_long.len = 257;
    reject(too_long, |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::InvalidBatch {
                error: AutomationBatchError::TooManyRecords,
                ..
            })
        )
    });
    let mut unequal_point = batch(62, &[record(H[0], 10, 1.0)]);
    unequal_point.records[0].end_value = 2.0;
    reject(unequal_point, |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::InvalidBatch {
                error: AutomationBatchError::InvalidPoint,
                ..
            })
        )
    });
    let mut out_of_order = batch(63, &[record(H[0], 10, 1.0), record(H[1], 11, 2.0)]);
    out_of_order.records.swap(0, 1);
    reject(out_of_order, |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::InvalidBatch {
                error: AutomationBatchError::OutOfOrder,
                ..
            })
        )
    });
    let wrong = AutomationBatchSlot::new(
        SessionRevision(8),
        RequestId::new(6).unwrap(),
        &[record(H[0], 10, 1.0)],
    )
    .unwrap();
    reject(wrong, |result| {
        matches!(result, Err(ScalarPointAdmissionError::WrongRevision { .. }))
    });
    reject(
        batch(64, &[record(ParameterHandle(99), 10, 1.0)]),
        |result| {
            matches!(
                result,
                Err(ScalarPointAdmissionError::UnknownBinding { .. })
            )
        },
    );
    reject(batch(65, &[record(H[0], 10, 24.5)]), |result| {
        matches!(result, Err(ScalarPointAdmissionError::InvalidValue { .. }))
    });
    let mut invalid_last = batch(66, &[record(H[0], 10, 1.0), record(H[1], 11, 2.0)]);
    invalid_last.records[1].start_value = f32::NAN;
    invalid_last.records[1].end_value = f32::NAN;
    reject(invalid_last, |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::InvalidBatch {
                error: AutomationBatchError::NonFiniteValue,
                ..
            })
        )
    });
    reject(batch(67, &[record(H[0], 9, 1.0)]), |result| {
        matches!(
            result,
            Err(ScalarPointAdmissionError::Service(
                AutomationEnqueueError::Invalid {
                    error: AutomationBatchError::TimeInPast,
                    ..
                }
            ))
        )
    });
    drop(reject);

    let mut r = r.start().unwrap_or_else(|_| panic!());
    let before = r.snapshot().unwrap();
    let (mut short, mut full) = ([1.0; Q - 1], [1.0; Q]);
    let short_before = short;
    let full_before = full;
    assert_eq!(
        r.render(&mut short, &mut full, SampleTime(0)),
        Err(ScalarPointRenderError::InvalidShape)
    );
    assert_eq!(short, short_before);
    assert_eq!(full, full_before);
    let (mut left, mut right) = ([0.31; Q], [0.27; Q]);
    let input = (left, right);
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(1)),
        Err(ScalarPointRenderError::DiscontinuousTime)
    );
    assert_eq!((left, right), input);
    assert_eq!(r.snapshot().unwrap(), before);
    assert_eq!(
        r.cancel_boundary(SampleTime(0)),
        Err(ScalarPointCancelBoundaryError::NotFaulted)
    );
    drop(r.stop());
    let after_rejections = payload(&*fx);
    let fresh = effect();
    assert_eq!(after_rejections, payload(&*fresh));

    let mut failing = FailingAccessEffect::apply_failure();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut failing, REV, H, config(), 70).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(70, &[record(H[0], 3, 7.0), record(H[1], 7, -7.0)]),
    )
    .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut left, mut right) = ([0.42; Q], [0.38; Q]);
    let fault = ScalarPointFault {
        cause: ScalarPointFaultCause::Native {
            operation: ScalarPointNativeOperation::Apply,
            channel: ParameterChannel::Right,
            error: ParameterAccessError::Unsupported,
        },
        at_sample: SampleTime(7),
        native_processed_frames: 7,
        progress: Some(ScalarPointFaultProgress {
            ticket,
            record_count: 2,
            native_applied_prefix: 1,
            delivery_applied_prefix: 1,
        }),
    };
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(0)),
        Err(ScalarPointRenderError::Fault(fault))
    );
    assert!(
        left.iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0)
    );
    assert_eq!(r.fault(), Some(fault));
    assert_eq!(r.snapshot(), Err(fault));
    let (mut invalid_left, mut invalid_right) = ([0.6; Q - 1], [0.5; Q]);
    let invalid_before = (invalid_left, invalid_right);
    assert_eq!(
        r.render(&mut invalid_left, &mut invalid_right, SampleTime(16)),
        Err(ScalarPointRenderError::InvalidShape)
    );
    assert_eq!((invalid_left, invalid_right), invalid_before);
    let (mut silent_left, mut silent_right) = ([0.6; Q], [0.5; Q]);
    assert_eq!(
        r.render(&mut silent_left, &mut silent_right, SampleTime(16)),
        Err(ScalarPointRenderError::Fault(fault))
    );
    assert!(
        silent_left
            .iter()
            .chain(&silent_right)
            .all(|sample| sample.to_bits() == 0)
    );
    assert_eq!(
        r.cancel_boundary(SampleTime(17)),
        Err(ScalarPointCancelBoundaryError::DiscontinuousTime)
    );
    let token = c
        .begin_cancel(AutomationCancellationReason::EndpointShutdown)
        .unwrap();
    assert!(c.poll_cancel_boundary(token).unwrap().is_none());
    r.cancel_boundary(SampleTime(16)).unwrap();
    assert_eq!(r.fault(), Some(fault));
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!(done.effective_sample, SampleTime(16));
    assert_eq!(
        (
            done.applied_records,
            done.canceled_records,
            done.canceled_events
        ),
        (1, 1, 1)
    );
    assert_eq!(c.outstanding(), 0);
    let event = c.try_dequeue_event().unwrap();
    assert_eq!(event.revision, REV);
    assert!(matches!(
        event.payload,
        ReliablePayload::AutomationCanceled {
            origin_request_id,
            canceled_records: 1,
            reason: AutomationCancellationReason::EndpointShutdown,
            effective_sample: Some(SampleTime(16)),
            ..
        } if origin_request_id == RequestId::new(70).unwrap()
    ));
    assert_eq!(c.collect_terminal(ticket), Err(DeliveryError::StaleTicket));
    assert_eq!(r.snapshot(), Err(fault));
    drop(r.stop());
    assert_eq!((failing.processed_frames, failing.apply_calls), (7, 2));

    let mut read_failing = FailingAccessEffect::read_failure();
    let (mut c, r, _) =
        prepare_scalar_point_endpoint(&mut read_failing, REV, H, config(), 80).unwrap();
    c.try_admit(SampleTime(0), batch(80, &[record(H[0], 30, 8.0)]))
        .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(ticket) => ticket,
        other => panic!("{other:?}"),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut left, mut right) = ([0.4; Q], [0.3; Q]);
    r.render(&mut left, &mut right, SampleTime(0)).unwrap();
    let read_fault = ScalarPointFault {
        cause: ScalarPointFaultCause::Native {
            operation: ScalarPointNativeOperation::Read,
            channel: ParameterChannel::Left,
            error: ParameterAccessError::Unsupported,
        },
        at_sample: SampleTime(16),
        native_processed_frames: 0,
        progress: Some(ScalarPointFaultProgress {
            ticket,
            record_count: 1,
            native_applied_prefix: 0,
            delivery_applied_prefix: 0,
        }),
    };
    assert_eq!(r.snapshot(), Err(read_fault));
    assert_eq!(r.snapshot(), Err(read_fault));
    let (mut left, mut right) = ([0.4; Q], [0.3; Q]);
    assert_eq!(
        r.render(&mut left, &mut right, SampleTime(16)),
        Err(ScalarPointRenderError::Fault(read_fault))
    );
    assert!(
        left.iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0)
    );
    let token = c
        .begin_cancel(AutomationCancellationReason::ProviderUnavailable)
        .unwrap();
    r.cancel_boundary(SampleTime(16)).unwrap();
    let done = c.poll_cancel_boundary(token).unwrap().unwrap();
    assert_eq!((done.applied_records, done.canceled_records), (0, 1));
    assert_eq!(done.effective_sample, SampleTime(16));
    assert_eq!(r.fault(), Some(read_fault));
    drop(r.stop());
    assert_eq!(read_failing.processed_frames, Q as u64);
    assert_eq!(read_failing.apply_calls, 0);
    assert_eq!(read_failing.read_calls.get(), 3);
}

#[test]
fn preparation_resources_and_success_path_are_bounded() {
    let mut fx = effect();
    let (mut c, r, res) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    assert!(res.delivery.retained_payload_bytes > 0);
    assert!(res.control_size > 0 && res.render_size > 0);
    c.try_admit(SampleTime(0), batch(7, &[record(H[0], 0, 2.0)]))
        .unwrap();
    c.try_handoff_next().unwrap();
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.4; Q], [0.3; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    assert_eq!(r.snapshot().unwrap().applied, 1);
}
