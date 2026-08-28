//! Semantic runtime and adversarial detector regression coverage.
//!
//! Issue #105 F2: this binary installs the workspace's one audited global allocator (#104 phase B,
//! registered inside `miso-engine-bench-support` itself) in count-and-continue mode, which is what
//! makes `run_effect_conformance`'s `process.allocation` verdict a real measurement. Without it
//! the harness stops with `harness.allocator_not_installed` rather than reporting a vacuous pass.
use miso_engine_bench_support::alloc as audited_allocator;
use miso_engine_lane::Backend;

/// Arm the counting allocator once for every test in this binary.
fn armed() {
    audited_allocator::assert_installed();
    audited_allocator::set_mode(audited_allocator::Mode::Count);
}

use miso_engine_conformance::{
    ConformanceConfig, DUAL_ACCUMULATOR_DELAY_DESCRIPTOR, DualAccumulatorDelayFactory, FaultKind,
    run_effect_conformance,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectDescriptor, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, ParameterId, ParameterMapping,
    ParameterSmoother, PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedPorts, PreparedSidechainPort, QualityDescriptor,
    SmoothingRule, automation_segment_value, expected_prepared_metadata, inverse_map_normalized,
    inverse_map_stepped_normalized, map_normalized, map_stepped_normalized,
    validate_automation_block, validate_descriptor,
};

#[test]
fn correct_factory_binds_distinguishable_four_lane_bank() {
    let values: Vec<_> = (0..4)
        .map(|lane| {
            [
                InitialParameterValue {
                    parameter_index: 0,
                    channel: ParameterChannel::Left,
                    value: 0.5 + lane as f32 * 0.1,
                },
                InitialParameterValue {
                    parameter_index: 0,
                    channel: ParameterChannel::Right,
                    value: 1.0 + lane as f32 * 0.1,
                },
            ]
        })
        .collect();
    let requests: Vec<_> = values
        .iter()
        .map(|initial_values| PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 8,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: miso_engine_conformance::DUAL_ACCUMULATOR_DELAY_DESCRIPTOR.ports[1].id,
                    required: false,
                },
            },
            initial_values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 8,
            },
        })
        .collect();
    let mut bank = DualAccumulatorDelayFactory::correct()
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd4,
            width: BankWidth::Four,
            requests: &requests,
        })
        .unwrap()
        .expect("positive bank");
    let mut left = vec![0.0; 4 * 4];
    let mut right = vec![0.0; 4 * 4];
    for lane in 0..4 {
        left[lane] = 1.0;
        right[lane] = 2.0;
    }
    let report = bank.process_bank(
        miso_engine_effect_contract::EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            4,
            BankWidth::Four,
            0,
            &[],
            &[0; 5],
            8,
        )
        .unwrap(),
    );
    assert_eq!(report.width, BankWidth::Four);
    for lane in 0..4 {
        assert_eq!(
            left[3 * 4 + lane].to_bits(),
            (0.5 + lane as f32 * 0.1).to_bits()
        );
        assert_eq!(
            right[3 * 4 + lane].to_bits(),
            (2.0 + lane as f32 * 0.2).to_bits()
        );
    }
}

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
    armed();
    let report = run_effect_conformance(
        &DualAccumulatorDelayFactory::correct(),
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(report.passed(), "{:?}", report.launch_gates.failures);
    assert!(report.extended_compatibility_probes.failures.is_empty());
    assert_eq!(report.launch_gates.prepared_configurations, 8);
    assert_eq!(
        report.extended_compatibility_probes.prepared_configurations,
        8
    );
    assert!(report.launch_gates.process_calls >= 800);
    assert!(report.extended_compatibility_probes.process_calls >= 800);
}

#[test]
fn every_faulty_mock_is_detected() {
    armed();
    // The second column names the failure string the fault must produce, where the fault has one.
    // `None` means "detected, string not pinned here" -- the hook faults all reach the harness as
    // a panic out of `audit::forbidden`, and pinning `process.panic` for them would only restate
    // the classification rule rather than gate the detector.
    for (fault, expected) in [
        (FaultKind::AllocationHook, None),
        (FaultKind::DeallocationHook, None),
        (FaultKind::LockHook, None),
        (FaultKind::IoHook, None),
        (FaultKind::NetworkHook, None),
        (FaultKind::LogHook, None),
        (FaultKind::SyscallHook, None),
        (FaultKind::SharedLaneState, None),
        (FaultKind::ChangingMetadata, None),
        (FaultKind::ChangingTail, None),
        (FaultKind::LatencyChangingBypass, None),
        (FaultKind::BadResources, None),
        (FaultKind::MalformedSpanAcceptance, None),
        (FaultKind::NonfinitePropagation, None),
        (FaultKind::NondeterministicSnapshot, None),
        (FaultKind::PartialSnapshot, None),
        (FaultKind::BadRestore, None),
        (FaultKind::Panic, Some("process.panic")),
        // Issue #105 phase 2, evals E10/E12/E13.
        (FaultKind::HeapAllocation, Some("process.allocation")),
        (
            FaultKind::PartitionDependent,
            Some("process.partition_invariance"),
        ),
        (FaultKind::StickyReset, Some("reset.snapshot_differs")),
        (FaultKind::BypassDelayMismatch, Some("latency.bypass_delay")),
    ] {
        let report = run_effect_conformance(
            &DualAccumulatorDelayFactory::faulty(fault),
            ConformanceConfig {
                quantum: 128,
                blocks: 1,
            },
        );
        assert!(!report.passed(), "fault {fault:?} escaped detection");
        if let Some(expected) = expected {
            assert!(
                report.launch_gates.failures.contains(&expected),
                "fault {fault:?} was detected as {:?}, not {expected:?}",
                report.launch_gates.failures
            );
        }
    }
}

#[test]
fn extended_rate_failures_are_reported_but_do_not_fail_launch_gates() {
    armed();
    let report = run_effect_conformance(
        &DualAccumulatorDelayFactory::faulty(FaultKind::ExtendedRatePreparation),
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(report.passed(), "{:?}", report.launch_gates.failures);
    assert!(report.launch_gates.failures.is_empty());
    assert_eq!(report.launch_gates.prepared_configurations, 8);
    assert_eq!(
        report.extended_compatibility_probes.failures,
        ["prepare.factory"]
    );
    assert_eq!(
        report.extended_compatibility_probes.prepared_configurations,
        0
    );
}

#[test]
fn ten_thousand_descriptor_and_span_mutations_reject_without_panic() {
    for seed in 0..10_000_u32 {
        let mut descriptor: EffectDescriptor = DUAL_ACCUMULATOR_DELAY_DESCRIPTOR;
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
        assert!(std::panic::catch_unwind(|| validate_descriptor(descriptor)).is_ok());
        assert!(validate_descriptor(descriptor).is_err());
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
            ports: PreparedPorts {
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

#[test]
fn descriptor_requires_launch_rows_and_accepts_optional_extended_rows() {
    let original = DUAL_ACCUMULATOR_DELAY_DESCRIPTOR;
    let launch = Box::leak(original.qualities[..4].to_vec().into_boxed_slice());
    let launch_only = Box::leak(Box::new(EffectDescriptor {
        qualities: launch,
        ..original
    }));
    assert!(validate_descriptor(launch_only).is_ok());

    for missing in 0..4 {
        let rows = original.qualities[..4]
            .iter()
            .enumerate()
            .filter_map(|(index, row)| (index != missing).then_some(*row))
            .collect::<Vec<_>>();
        let descriptor = Box::leak(Box::new(EffectDescriptor {
            qualities: Box::leak(rows.into_boxed_slice()),
            ..original
        }));
        assert!(validate_descriptor(descriptor).is_err());
    }

    for subset in 0_u8..16 {
        let rows = original.qualities[..4]
            .iter()
            .copied()
            .chain(
                original.qualities[4..]
                    .iter()
                    .enumerate()
                    .filter_map(|(index, row)| (subset & (1 << index) != 0).then_some(*row)),
            )
            .collect::<Vec<_>>();
        let descriptor = Box::leak(Box::new(EffectDescriptor {
            qualities: Box::leak(rows.into_boxed_slice()),
            ..original
        }));
        assert!(validate_descriptor(descriptor).is_ok());
    }

    let draft_launch = original.qualities[..4]
        .iter()
        .map(|row| QualityDescriptor {
            quality: EffectQuality::Draft,
            ..*row
        });
    let multiple_qualities = draft_launch
        .chain(original.qualities[..4].iter().copied())
        .collect::<Vec<_>>();
    let descriptor = Box::leak(Box::new(EffectDescriptor {
        qualities: Box::leak(multiple_qualities.into_boxed_slice()),
        ..original
    }));
    assert!(validate_descriptor(descriptor).is_ok());

    let incomplete_draft = original.qualities[..3]
        .iter()
        .map(|row| QualityDescriptor {
            quality: EffectQuality::Draft,
            ..*row
        })
        .chain(original.qualities[..4].iter().copied())
        .collect::<Vec<_>>();
    let descriptor = Box::leak(Box::new(EffectDescriptor {
        qualities: Box::leak(incomplete_draft.into_boxed_slice()),
        ..original
    }));
    assert!(validate_descriptor(descriptor).is_err());

    let mut duplicate = original.qualities[..4].to_vec();
    duplicate.push(original.qualities[3]);
    let descriptor = Box::leak(Box::new(EffectDescriptor {
        qualities: Box::leak(duplicate.into_boxed_slice()),
        ..original
    }));
    assert!(validate_descriptor(descriptor).is_err());

    let unordered = original.qualities[..4]
        .iter()
        .copied()
        .chain([original.qualities[5], original.qualities[4]])
        .collect::<Vec<_>>();
    let descriptor = Box::leak(Box::new(EffectDescriptor {
        qualities: Box::leak(unordered.into_boxed_slice()),
        ..original
    }));
    assert!(validate_descriptor(descriptor).is_err());

    let unsupported = QualityDescriptor {
        sample_rate: 192_001,
        ..original.qualities[3]
    };
    let rows = original.qualities[..4]
        .iter()
        .copied()
        .chain([unsupported])
        .collect::<Vec<_>>();
    let descriptor = Box::leak(Box::new(EffectDescriptor {
        qualities: Box::leak(rows.into_boxed_slice()),
        ..original
    }));
    assert!(validate_descriptor(descriptor).is_err());
}
