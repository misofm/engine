#![allow(missing_docs)]

//! Decision 17, C3 and D8: a parameter's stable machine name is `[a-z][a-z0-9_]*`, unique within
//! its effect, and ends with the suffix of its unit.

mod support;

use effect_contract::*;
use support::{descriptor, unstated};

const fn parameter(id: u32, name: &'static str) -> ParameterDescriptor {
    with_unit(id, name, ParameterUnit::Db)
}

const fn with_unit(id: u32, name: &'static str, unit: ParameterUnit) -> ParameterDescriptor {
    ParameterDescriptor {
        id: ParameterId(id),
        name,
        display_name: "Gain",
        display_unit: "dB",
        unit,
        domain: ParameterDomain::Continuous,
        minimum: Some(-24.0),
        maximum: Some(24.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: 8,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice: ParameterLattice::arithmetic(0.1, 1),
        prepared_reason: None,
    }
}

const VALID: [ParameterDescriptor; 7] = [
    parameter(1, "gain_db"),
    with_unit(2, "band_1_q", ParameterUnit::Ratio),
    with_unit(3, "cutoff_hz", ParameterUnit::Hz),
    with_unit(4, "attack_ms", ParameterUnit::Milliseconds),
    with_unit(5, "window_samples", ParameterUnit::Samples),
    with_unit(6, "mix", ParameterUnit::Linear),
    with_unit(7, "low_ratio", ParameterUnit::Ratio),
];
const MISSING_SUFFIX: [ParameterDescriptor; 1] = [parameter(1, "gain")];
const WRONG_SUFFIX: [ParameterDescriptor; 1] = [parameter(1, "gain_hz")];
const SUFFIX_ON_LINEAR: [ParameterDescriptor; 1] = [with_unit(1, "mix_db", ParameterUnit::Linear)];
const SUFFIX_ON_RATIO: [ParameterDescriptor; 1] = [with_unit(1, "q_ms", ParameterUnit::Ratio)];
const UPPERCASE: [ParameterDescriptor; 1] = [parameter(1, "Gain_db")];
const LEADING_DIGIT: [ParameterDescriptor; 1] = [parameter(1, "1_gain_db")];
const HYPHEN: [ParameterDescriptor; 1] = [parameter(1, "band-1_db")];
const SPACE: [ParameterDescriptor; 1] = [parameter(1, "delay time_db")];
const EMPTY: [ParameterDescriptor; 1] = [parameter(1, "")];
const DUPLICATE: [ParameterDescriptor; 2] = [parameter(1, "gain_db"), parameter(2, "gain_db")];

macro_rules! effect {
    ($name:ident, $parameters:expr) => {
        static $name: EffectDescriptor = EffectDescriptor {
            parameters: &$parameters,
            ..descriptor(1, unstated)
        };
    };
}
effect!(VALID_EFFECT, VALID);
effect!(UPPERCASE_EFFECT, UPPERCASE);
effect!(LEADING_DIGIT_EFFECT, LEADING_DIGIT);
effect!(HYPHEN_EFFECT, HYPHEN);
effect!(SPACE_EFFECT, SPACE);
effect!(EMPTY_EFFECT, EMPTY);
effect!(DUPLICATE_EFFECT, DUPLICATE);
effect!(MISSING_SUFFIX_EFFECT, MISSING_SUFFIX);
effect!(WRONG_SUFFIX_EFFECT, WRONG_SUFFIX);
effect!(SUFFIX_ON_LINEAR_EFFECT, SUFFIX_ON_LINEAR);
effect!(SUFFIX_ON_RATIO_EFFECT, SUFFIX_ON_RATIO);

fn codes(descriptor: &'static EffectDescriptor) -> Vec<DescriptorDiagnosticCode> {
    match validate_descriptor(descriptor) {
        Ok(()) => Vec::new(),
        Err(set) => set.errors().iter().map(|error| error.code).collect(),
    }
}

/// Red if a machine name that a frontend or the generated SDK could not key by (uppercase, a
/// leading digit, a hyphen or space, empty), a name repeated inside one effect, a name without
/// the suffix of its unit (dB, Hz, ms, samples), the wrong suffix, or a unit
/// suffix on a unitless or ratio parameter passes the validator. The valid effect carries one
/// parameter of every unit; each refused descriptor differs from a valid one in the name alone.
#[test]
fn a_machine_name_is_snake_case_and_unique_within_its_effect() {
    assert_eq!(codes(&VALID_EFFECT), []);
    for refused in [
        &UPPERCASE_EFFECT,
        &LEADING_DIGIT_EFFECT,
        &HYPHEN_EFFECT,
        &SPACE_EFFECT,
        &EMPTY_EFFECT,
        &DUPLICATE_EFFECT,
        &MISSING_SUFFIX_EFFECT,
        &WRONG_SUFFIX_EFFECT,
        &SUFFIX_ON_LINEAR_EFFECT,
        &SUFFIX_ON_RATIO_EFFECT,
    ] {
        assert_eq!(codes(refused), [DescriptorDiagnosticCode::ParameterName]);
    }
}

/// Red if `machine_name_unit_suffix` and the validator disagree about a unit's suffix.
#[test]
fn each_unit_maps_to_one_suffix() {
    for (unit, suffix) in [
        (ParameterUnit::Db, Some("_db")),
        (ParameterUnit::Hz, Some("_hz")),
        (ParameterUnit::Milliseconds, Some("_ms")),
        (ParameterUnit::Samples, Some("_samples")),
        (ParameterUnit::Linear, None),
        (ParameterUnit::Ratio, None),
    ] {
        assert_eq!(machine_name_unit_suffix(unit), suffix);
    }
}
