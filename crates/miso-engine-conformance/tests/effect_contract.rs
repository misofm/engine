//! Semantic runtime and adversarial detector regression coverage.

use miso_engine_conformance::{
    ConformanceConfig, DUAL_ACCUMULATOR_DELAY_DESCRIPTOR, DualAccumulatorDelayFactory, FaultKind,
    run_effect_conformance,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, EffectDescriptorV1, EffectQuality, InitialParameterValue, LinkMode,
    ParameterChannel, ParameterId, ParameterMapping, ParameterSmoother, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedPortsV1, PreparedSidechainPort,
    SmoothingRule, automation_segment_value, expected_prepared_metadata, inverse_map_normalized,
    inverse_map_stepped_normalized, map_normalized, map_stepped_normalized,
    validate_automation_block, validate_descriptor_v1,
};

#[test]
fn normalized_mapping_endpoints_ties_and_round_trips_are_stable() {
    for mapping in [
        ParameterMapping::Linear,
        ParameterMapping::Logarithmic,
        ParameterMapping::Exponential,
    ] {
        assert_eq!(map_normalized(mapping, 2.0, 8.0, 0.0), Some(2.0));
        assert_eq!(map_normalized(mapping, 2.0, 8.0, 1.0), Some(8.0));
        let value = map_normalized(mapping, 2.0, 8.0, 0.375).unwrap();
        assert!((inverse_map_normalized(mapping, 2.0, 8.0, value).unwrap() - 0.375).abs() < 2e-6);
    }
    assert_eq!(map_stepped_normalized(&[0.0, 1.0], 0.5), Some(0.0));
    assert_eq!(map_stepped_normalized(&[0.0, 1.0, 4.0], 0.75), Some(1.0));
    assert_eq!(
        inverse_map_stepped_normalized(&[0.0, 1.0, 4.0], 4.0),
        Some(1.0)
    );
    assert!(map_normalized(ParameterMapping::Linear, 0.0, 1.0, f32::NAN).is_none());
}

#[test]
fn smoothers_and_segments_finish_on_the_exact_update_or_endpoint() {
    let mut linear = ParameterSmoother::new(0.0, SmoothingRule::Linear, 4).unwrap();
    assert!(linear.set_target(1.0));
    assert_eq!(
        [
            linear.next_value(),
            linear.next_value(),
            linear.next_value(),
            linear.next_value()
        ],
        [0.25, 0.5, 0.75, 1.0]
    );
    let mut pole = ParameterSmoother::new(0.0, SmoothingRule::OnePole99, 4).unwrap();
    assert!(pole.set_target(1.0));
    for _ in 0..3 {
        assert!(pole.next_value() < 1.0);
    }
    assert_eq!(pole.next_value(), 1.0);
    let span = PreparedAutomationSpan {
        kind: AutomationSpanKind::Linear,
        channel: ParameterChannel::Left,
        parameter_index: 0,
        start_sample: 10,
        end_sample: 14,
        start_value: 2.0,
        end_value: 6.0,
    };
    assert_eq!(automation_segment_value(span, 10), Some(2.0));
    assert_eq!(automation_segment_value(span, 12), Some(4.0));
    assert_eq!(automation_segment_value(span, 14), Some(6.0));
}

#[test]
fn correct_mock_passes_every_enabled_conformance_gate() {
    let report = run_effect_conformance(
        &DualAccumulatorDelayFactory::correct(),
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(report.passed(), "{:?}", report.failed_gates);
    assert_eq!(report.prepared_configurations, 16);
    assert!(report.process_calls >= 1_600);
}

#[test]
fn every_faulty_mock_is_detected() {
    for fault in [
        FaultKind::AllocationHook,
        FaultKind::DeallocationHook,
        FaultKind::LockHook,
        FaultKind::IoHook,
        FaultKind::NetworkHook,
        FaultKind::LogHook,
        FaultKind::SyscallHook,
        FaultKind::SharedLaneState,
        FaultKind::ChangingMetadata,
        FaultKind::ChangingTail,
        FaultKind::LatencyChangingBypass,
        FaultKind::BadResources,
        FaultKind::MalformedSpanAcceptance,
        FaultKind::NonfinitePropagation,
        FaultKind::NondeterministicSnapshot,
        FaultKind::PartialSnapshot,
        FaultKind::BadRestore,
        FaultKind::Panic,
    ] {
        let report = run_effect_conformance(
            &DualAccumulatorDelayFactory::faulty(fault),
            ConformanceConfig {
                quantum: 128,
                blocks: 1,
            },
        );
        assert!(!report.passed(), "fault {fault:?} escaped detection");
    }
}

#[test]
fn ten_thousand_descriptor_and_span_mutations_reject_without_panic() {
    for seed in 0..10_000_u32 {
        let mut descriptor: EffectDescriptorV1 = DUAL_ACCUMULATOR_DELAY_DESCRIPTOR;
        match seed % 5 {
            0 => descriptor.contract_major = 2,
            1 => descriptor.state_layout_version = 0,
            2 => descriptor.display_name = "",
            3 => descriptor.ports = &[],
            _ => {
                let mut parameter = descriptor.parameters[0];
                parameter.id = ParameterId(0);
                descriptor.parameters = Box::leak(Box::new([parameter]));
            }
        }
        let descriptor = Box::leak(Box::new(descriptor));
        assert!(std::panic::catch_unwind(|| validate_descriptor_v1(descriptor)).is_ok());
        assert!(validate_descriptor_v1(descriptor).is_err());
    }

    let initial = [
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 1.0,
        },
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Right,
            value: 1.0,
        },
    ];
    let metadata = expected_prepared_metadata(
        &DUAL_ACCUMULATOR_DELAY_DESCRIPTOR,
        PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: DUAL_ACCUMULATOR_DELAY_DESCRIPTOR.ports[1].id,
                    required: false,
                },
            },
            initial_values: &initial,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1024,
                maximum_scratch_bytes: 1024,
                maximum_automation_spans_per_block: 8,
            },
        },
    )
    .unwrap();
    for seed in 0..10_000_u32 {
        let mut span = PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: 1.0,
            end_value: 1.0,
        };
        match seed % 5 {
            0 => span.parameter_index = u32::MAX,
            1 => span.channel = ParameterChannel::Both,
            2 => span.start_sample = 128,
            3 => span.end_value = f32::NAN,
            _ => span.end_sample = 1,
        }
        assert!(
            std::panic::catch_unwind(|| validate_automation_block(&[span], metadata, 0, 128))
                .is_ok()
        );
        assert!(validate_automation_block(&[span], metadata, 0, 128).is_err());
    }
}
