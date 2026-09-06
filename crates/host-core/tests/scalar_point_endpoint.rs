//! Product gates for borrowed scalar compressor Point delivery.
#![cfg(feature = "control-provider")]

use core::num::NonZeroUsize;
use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ObservationSample, ParameterChannel, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedParameterState,
    PreparedPorts, PreparedSidechainPort, ProcessReport, StatePayloadOutput,
};
use host_core::{
    ScalarPointAdmissionError, ScalarPointCancelBoundaryError, ScalarPointRenderError,
    prepare_scalar_point_endpoint,
};
use protocol::{
    AutomationBatchSlot, AutomationCancellationReason, AutomationKind, AutomationRecord,
    DeliveryError, HandoffResult, ParameterHandle, ProtocolQueueConfig, RequestId, SampleTime,
    SessionRevision,
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
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    let wrong = AutomationBatchSlot::new(
        SessionRevision(8),
        RequestId::new(6).unwrap(),
        &[record(H[0], 1, 1.0)],
    )
    .unwrap();
    assert!(matches!(
        c.try_admit(SampleTime(0), wrong),
        Err(ScalarPointAdmissionError::WrongRevision { .. })
    ));
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let before = r.snapshot().unwrap();
    let (mut short, mut full) = ([1.0; Q - 1], [1.0; Q]);
    assert_eq!(
        r.render(&mut short, &mut full, SampleTime(0)),
        Err(ScalarPointRenderError::InvalidShape)
    );
    assert_eq!(r.snapshot().unwrap(), before);
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
