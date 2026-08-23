//! Frozen contract surface of the builtin parameters: descriptor table, rate-keyed domains and
//! the issue-036 representable-cutoff table.
//!
//! Nothing here is a numerics gate. These are the fixtures master plan §8.2 classifies as
//! *contract*: they do not move when a kernel changes, and #85 changed none of them.

use miso_engine_builtins::*;
use miso_engine_core::EXTENDED_COMPATIBILITY_SAMPLE_RATES;

fn parameters_with_cutoff(cutoff: f32, high_pass: bool) -> BuiltinParameters {
    let mut parameters = BuiltinParameters::default();
    if high_pass {
        parameters.left.hpf_hz = cutoff;
    } else {
        parameters.left.lpf_hz = cutoff;
    }
    parameters
}

#[test]
fn parameter_descriptors_have_complete_stable_contracts() {
    let descriptors = BUILTIN_PARAMETER_DESCRIPTORS_V1;
    assert_eq!(
        descriptors.map(|descriptor| descriptor.id),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.name),
        [
            "polarity_invert",
            "trim_db",
            "hpf_hz",
            "lpf_hz",
            "fader_db",
            "mute",
            "matrix_ll",
            "matrix_lr",
            "matrix_rl",
            "matrix_rr",
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.scope),
        [
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::PerLane,
            BuiltinParameterScope::MatrixShared,
            BuiltinParameterScope::MatrixShared,
            BuiltinParameterScope::MatrixShared,
            BuiltinParameterScope::MatrixShared,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.mapping),
        [
            BuiltinParameterMapping::Boolean,
            BuiltinParameterMapping::DecibelAmplitude,
            BuiltinParameterMapping::Hertz,
            BuiltinParameterMapping::Hertz,
            BuiltinParameterMapping::DecibelAmplitude,
            BuiltinParameterMapping::Boolean,
            BuiltinParameterMapping::Linear,
            BuiltinParameterMapping::Linear,
            BuiltinParameterMapping::Linear,
            BuiltinParameterMapping::Linear,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.default.to_bits()),
        [0, 0, 0, 0, 0, 0, 1.0_f32.to_bits(), 0, 0, 1.0_f32.to_bits()]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.update_rate),
        [
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::PreparedOnly,
            BuiltinParameterUpdateRate::BlockTarget,
            BuiltinParameterUpdateRate::BlockTarget,
            BuiltinParameterUpdateRate::BlockTarget,
            BuiltinParameterUpdateRate::BlockTarget,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.smoothing),
        [
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::None,
            BuiltinSmoothingPolicy::LinearNUpdates,
            BuiltinSmoothingPolicy::LinearNUpdates,
            BuiltinSmoothingPolicy::LinearNUpdates,
            BuiltinSmoothingPolicy::LinearNUpdates,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.reset),
        [
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::RestorePreparedValue,
            BuiltinParameterReset::KeepTargetResetCurrent,
            BuiltinParameterReset::KeepTargetResetCurrent,
            BuiltinParameterReset::KeepTargetResetCurrent,
            BuiltinParameterReset::KeepTargetResetCurrent,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.disabled_value),
        [
            None,
            None,
            Some(0.0),
            Some(0.0),
            None,
            None,
            None,
            None,
            None,
            None,
        ]
    );
    assert_eq!(
        descriptors.map(|descriptor| descriptor.domain),
        [
            BuiltinParameterDomain::BooleanExact,
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -144.0,
                maximum: 24.0,
            },
            BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
                disabled: 0.0,
                minimum_hz: 10.0,
            },
            BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 {
                disabled: 0.0,
                minimum_hz: 10.0,
            },
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -144.0,
                maximum: 24.0,
            },
            BuiltinParameterDomain::BooleanExact,
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -1.0,
                maximum: 1.0,
            },
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -1.0,
                maximum: 1.0,
            },
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -1.0,
                maximum: 1.0,
            },
            BuiltinParameterDomain::FiniteInclusive {
                minimum: -1.0,
                maximum: 1.0,
            },
        ]
    );
}

#[test]
fn descriptor_domains_are_exhaustive_at_launch_rates() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        for descriptor in BUILTIN_PARAMETER_DESCRIPTORS_V1 {
            assert!(descriptor.domain.contains(descriptor.default, rate));
            assert!(!descriptor.domain.contains(f32::NAN, rate));
            assert!(!descriptor.domain.contains(f32::INFINITY, rate));
            assert!(!descriptor.domain.contains(f32::NEG_INFINITY, rate));
        }
        for descriptor in [
            BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
            BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
        ] {
            let maximum = builtin_filter_cutoff_maximum_hz_v1(rate)
                .expect("launch rate has an exact cutoff maximum");
            let successor = f32::from_bits(maximum.to_bits() + 1);
            let nyquist = rate as f32 / 2.0;
            let just_below_maximum = f32::from_bits(maximum.to_bits() - 1);
            assert!(descriptor.domain.contains(0.0, rate));
            assert!(!descriptor.domain.contains(-0.0, rate));
            assert!(!descriptor.domain.contains(9.999, rate));
            assert!(descriptor.domain.contains(10.0, rate));
            assert!(descriptor.domain.contains(just_below_maximum, rate));
            assert!(descriptor.domain.contains(maximum, rate));
            assert!(!descriptor.domain.contains(successor, rate));
            assert!(!descriptor.domain.contains(nyquist, rate));
        }
    }
    for boolean in [
        BUILTIN_PARAMETER_DESCRIPTORS_V1[0],
        BUILTIN_PARAMETER_DESCRIPTORS_V1[5],
    ] {
        assert!(boolean.domain.contains(0.0, 48_000));
        assert!(boolean.domain.contains(1.0, 48_000));
        assert!(!boolean.domain.contains(-0.0, 48_000));
        assert!(!boolean.domain.contains(0.5, 48_000));
    }
    for decibels in [
        BUILTIN_PARAMETER_DESCRIPTORS_V1[1],
        BUILTIN_PARAMETER_DESCRIPTORS_V1[4],
    ] {
        assert!(decibels.domain.contains(-144.0, 48_000));
        assert!(decibels.domain.contains(24.0, 48_000));
        assert!(!decibels.domain.contains(-144.001, 48_000));
        assert!(!decibels.domain.contains(24.001, 48_000));
    }
    for matrix in &BUILTIN_PARAMETER_DESCRIPTORS_V1[6..] {
        assert!(matrix.domain.contains(-1.0, 48_000));
        assert!(matrix.domain.contains(1.0, 48_000));
        assert!(!matrix.domain.contains(-1.001, 48_000));
        assert!(!matrix.domain.contains(1.001, 48_000));
    }
}

#[test]
fn compatibility_fallback_is_limited_to_the_exact_extended_rate_tier() {
    for rate in EXTENDED_COMPATIBILITY_SAMPLE_RATES.map(|rate| rate.0) {
        assert_eq!(builtin_filter_cutoff_maximum_hz_v1(rate), None);
        for descriptor in [
            BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
            BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
        ] {
            assert!(descriptor.domain.contains(0.0, rate));
            assert!(descriptor.domain.contains(10.0, rate));
            assert!(descriptor.domain.contains(0.45 * rate as f32, rate));
        }
        assert!(BuiltinChain::new(rate, BuiltinParameters::default()).is_ok());
    }
    for rate in [0, 32_000, 192_001] {
        assert_eq!(builtin_filter_cutoff_maximum_hz_v1(rate), None);
        for descriptor in [
            BUILTIN_PARAMETER_DESCRIPTORS_V1[2],
            BUILTIN_PARAMETER_DESCRIPTORS_V1[3],
        ] {
            assert!(!descriptor.domain.contains(0.0, rate));
            assert!(!descriptor.domain.contains(10.0, rate));
        }
        assert!(matches!(
            BuiltinChain::new(rate, BuiltinParameters::default()),
            Err(BuiltinParameterError::FilterCutoff)
        ));
    }
}

#[test]
fn representable_cutoff_domain_is_shared_by_descriptors_and_preparation() {
    for (rate, maximum_bits) in [
        (44_100, 0x46ac_42f7),
        (48_000, 0x46bb_7ede),
        (88_200, 0x472c_42f7),
        (96_000, 0x473b_7ede),
    ] {
        let maximum = builtin_filter_cutoff_maximum_hz_v1(rate).expect("launch rate has maximum");
        assert_eq!(maximum.to_bits(), maximum_bits, "rate={rate}");
        let successor = f32::from_bits(maximum_bits + 1);
        let nyquist = rate as f32 * 0.5;
        let nyquist_predecessor = f32::from_bits(nyquist.to_bits() - 1);
        for (descriptor, high_pass) in [
            (BUILTIN_PARAMETER_DESCRIPTORS_V1[2], true),
            (BUILTIN_PARAMETER_DESCRIPTORS_V1[3], false),
        ] {
            for (cutoff, expected) in [
                (0.0, true),
                (10.0, true),
                (f32::from_bits(maximum_bits - 1), true),
                (maximum, true),
                (successor, false),
                (nyquist_predecessor, false),
                (nyquist, false),
                (9.999, false),
                (f32::NAN, false),
                (f32::INFINITY, false),
                (f32::NEG_INFINITY, false),
            ] {
                assert_eq!(
                    descriptor.domain.contains(cutoff, rate),
                    expected,
                    "descriptor rate={rate}, high_pass={high_pass}, cutoff={:08x}",
                    cutoff.to_bits()
                );
                assert_eq!(
                    BuiltinChain::new(rate, parameters_with_cutoff(cutoff, high_pass)).is_ok(),
                    expected,
                    "preparation rate={rate}, high_pass={high_pass}, cutoff={:08x}",
                    cutoff.to_bits()
                );
            }
        }
    }
}
