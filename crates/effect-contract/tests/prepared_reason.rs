#![allow(missing_docs)]

//! Decision 17, D1: a parameter that is prepared (`AutomationRate::None`) states why in its own
//! descriptor row, and no other parameter carries a reason.

mod support;

use effect_contract::control::{PreparedReason, RebuildReason};
use effect_contract::*;
use support::{descriptor, unstated};

const REASON: PreparedReason = PreparedReason {
    reason: RebuildReason::Glitch,
    reopen: "a window transition that does not glitch",
};

/// A continuous parameter that differs from the fixture's `Gain` only in the fields under test.
const fn parameter(
    automation_rate: AutomationRate,
    prepared_reason: Option<PreparedReason>,
) -> ParameterDescriptor {
    let live = !matches!(automation_rate, AutomationRate::None);
    ParameterDescriptor {
        id: ParameterId(1),
        display_name: "Gain",
        name: "gain_db",
        display_unit: "dB",
        unit: ParameterUnit::Db,
        domain: ParameterDomain::Continuous,
        minimum: Some(-24.0),
        maximum: Some(24.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: if live {
            SmoothingRule::Linear
        } else {
            SmoothingRule::None
        },
        smoothing_samples: if live { 8 } else { 0 },
        readable: true,
        automatable: live,
        enum_choices: &[],
        lattice: ParameterLattice::arithmetic(0.1, 1),
        prepared_reason,
    }
}

const BLOCK: [ParameterDescriptor; 1] = [parameter(AutomationRate::Block, None)];
const PREPARED: [ParameterDescriptor; 1] = [parameter(AutomationRate::None, Some(REASON))];
const PREPARED_NO_REASON: [ParameterDescriptor; 1] = [parameter(AutomationRate::None, None)];
const BLOCK_WITH_REASON: [ParameterDescriptor; 1] =
    [parameter(AutomationRate::Block, Some(REASON))];
const PREPARED_EMPTY_REOPEN: [ParameterDescriptor; 1] = [parameter(
    AutomationRate::None,
    Some(PreparedReason {
        reason: RebuildReason::Glitch,
        reopen: "",
    }),
)];

static LIVE_OK: EffectDescriptor = EffectDescriptor {
    parameters: &BLOCK,
    ..descriptor(1, unstated)
};
static PREPARED_OK: EffectDescriptor = EffectDescriptor {
    parameters: &PREPARED,
    ..descriptor(1, unstated)
};
static NO_REASON: EffectDescriptor = EffectDescriptor {
    parameters: &PREPARED_NO_REASON,
    ..descriptor(1, unstated)
};
static REASON_ON_LIVE: EffectDescriptor = EffectDescriptor {
    parameters: &BLOCK_WITH_REASON,
    ..descriptor(1, unstated)
};
static EMPTY_REOPEN: EffectDescriptor = EffectDescriptor {
    parameters: &PREPARED_EMPTY_REOPEN,
    ..descriptor(1, unstated)
};

fn codes(descriptor: &'static EffectDescriptor) -> Vec<DescriptorDiagnosticCode> {
    match validate_descriptor(descriptor) {
        Ok(()) => Vec::new(),
        Err(set) => set.errors().iter().map(|error| error.code).collect(),
    }
}

/// Red if a prepared parameter can be declared without a recorded reason, a reason can sit on a
/// parameter that is not prepared, or a reason can have no reopening condition. The two admissible
/// twins differ from the refused descriptors in the reason field and the rate alone.
#[test]
fn a_prepared_parameter_states_its_reason_and_no_other_does() {
    assert_eq!(codes(&LIVE_OK), []);
    assert_eq!(codes(&PREPARED_OK), []);
    for refused in [&NO_REASON, &REASON_ON_LIVE, &EMPTY_REOPEN] {
        assert_eq!(codes(refused), [DescriptorDiagnosticCode::PreparedReason]);
    }
}
