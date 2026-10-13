#![allow(missing_docs)]

//! Decision 17, C3: a parameter's stable machine name is `[a-z][a-z0-9_]*` and unique within its
//! effect.

mod support;

use effect_contract::*;
use support::{descriptor, unstated};

const fn parameter(id: u32, name: &'static str) -> ParameterDescriptor {
    ParameterDescriptor {
        id: ParameterId(id),
        name,
        display_name: "Gain",
        display_unit: "dB",
        unit: ParameterUnit::Db,
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

const VALID: [ParameterDescriptor; 2] = [parameter(1, "gain_db"), parameter(2, "band_1_q")];
const UPPERCASE: [ParameterDescriptor; 1] = [parameter(1, "Gain")];
const LEADING_DIGIT: [ParameterDescriptor; 1] = [parameter(1, "1_gain")];
const HYPHEN: [ParameterDescriptor; 1] = [parameter(1, "band-1")];
const SPACE: [ParameterDescriptor; 1] = [parameter(1, "delay time")];
const EMPTY: [ParameterDescriptor; 1] = [parameter(1, "")];
const DUPLICATE: [ParameterDescriptor; 2] = [parameter(1, "gain"), parameter(2, "gain")];

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

fn codes(descriptor: &'static EffectDescriptor) -> Vec<DescriptorDiagnosticCode> {
    match validate_descriptor(descriptor) {
        Ok(()) => Vec::new(),
        Err(set) => set.errors().iter().map(|error| error.code).collect(),
    }
}

/// Red if a machine name that a frontend or the generated SDK could not key by (uppercase, a
/// leading digit, a hyphen or space, empty) or a name repeated inside one effect passes the
/// validator. The valid twin differs from each refused descriptor in the name alone.
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
    ] {
        assert_eq!(codes(refused), [DescriptorDiagnosticCode::ParameterName]);
    }
}
