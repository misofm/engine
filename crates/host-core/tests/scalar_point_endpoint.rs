//! Product gates for borrowed scalar compressor Point delivery.

use core::num::NonZeroUsize;
use effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect, PreparedPorts,
    PreparedSidechainPort,
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
    let mut fx = effect();
    let (mut c, r, _) = prepare_scalar_point_endpoint(&mut *fx, REV, H, config(), 1).unwrap();
    c.try_admit(
        SampleTime(0),
        batch(1, &[record(H[0], 3, 6.0), record(H[1], 7, -6.0)]),
    )
    .unwrap();
    let ticket = match c.try_handoff_next().unwrap() {
        HandoffResult::HandedOff(t) => t,
        _ => panic!(),
    };
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let mut l = [0.4; Q];
    let mut rr = [0.3; Q];
    assert_eq!(
        r.render(&mut l, &mut rr, SampleTime(0))
            .unwrap()
            .process_invocations,
        3
    );
    let s = r.snapshot().unwrap();
    assert_eq!(s.applied, 2);
    assert_eq!(
        s.last_application,
        [Some(SampleTime(3)), Some(SampleTime(7))]
    );
    assert_eq!(c.collect_terminal(ticket).unwrap().applied_prefix, 2);
    assert!(l.iter().any(|x| x.to_bits() != 0.4f32.to_bits()));
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
    c.try_handoff_next().unwrap();
    c.try_handoff_next().unwrap();
    let mut r = r.start().unwrap_or_else(|_| panic!());
    let (mut l, mut x) = ([0.2; Q], [0.2; Q]);
    r.render(&mut l, &mut x, SampleTime(0)).unwrap();
    assert_eq!(r.snapshot().unwrap().applied, 2);
    r.render(&mut l, &mut x, SampleTime(16)).unwrap();
    let s = r.snapshot().unwrap();
    assert_eq!((s.applied, s.late), (3, 1));
    assert_eq!(s.last_application[1], Some(SampleTime(16)));
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
