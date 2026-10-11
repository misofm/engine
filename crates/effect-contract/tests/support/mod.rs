//! A minimal admissible effect for the registry's integration tests: one gain parameter, a main
//! input and output, and a `Normal` quality row at every launch rate.
//!
//! Each test binary declares its own `static` descriptors from [`descriptor`], choosing the
//! contract major and the `tail_and_rest` statement it needs, and admits them with [`Factory`].
#![allow(dead_code)]

use effect_contract::*;

pub(crate) const EFFECT_ID: EffectId = match EffectId::new("registry-test") {
    Ok(value) => value,
    Err(_) => panic!("valid test effect ID"),
};
const MAIN_IN: PortId = match PortId::new("main-in") {
    Ok(value) => value,
    Err(_) => panic!("valid test port ID"),
};
const MAIN_OUT: PortId = match PortId::new("main-out") {
    Ok(value) => value,
    Err(_) => panic!("valid test port ID"),
};

const PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
    id: ParameterId(1),
    display_name: "Gain",
    display_unit: "dB",
    unit: ParameterUnit::Db,
    domain: ParameterDomain::Continuous,
    minimum: Some(-24.0),
    maximum: Some(24.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Sample,
    channel_policy: ParameterChannelPolicy::Shared,
    smoothing: SmoothingRule::Linear,
    smoothing_samples: 8,
    readable: true,
    automatable: true,
    enum_choices: &[],
    lattice: ParameterLattice::arithmetic(0.1, 1),
}];
const PORTS: [PortDescriptor; 2] = [
    PortDescriptor {
        id: MAIN_IN,
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: MAIN_OUT,
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const fn quality(sample_rate: u32) -> QualityDescriptor {
    QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 0,
            right_bytes: 0,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}
const QUALITIES: [QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// The launch effects' statement today: a finite tail, no exact-rest bound.
pub(crate) fn unstated(_: u32, _: EffectQuality) -> NodeTailBound {
    NodeTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
        composition: CompositionBound::Unstated,
    }
}

pub(crate) const fn descriptor(
    contract_major: u16,
    tail_and_rest: fn(u32, EffectQuality) -> NodeTailBound,
) -> EffectDescriptor {
    EffectDescriptor {
        id: EFFECT_ID,
        display_name: "Registry Test",
        contract_major,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        parameters: &PARAMETERS,
        ports: &PORTS,
        qualities: &QUALITIES,
        tail_and_rest,
        observations: &[],
    }
}

pub(crate) struct Factory(pub(crate) &'static EffectDescriptor);

impl NativeEffectFactory for Factory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        self.0
    }

    fn prepare(
        &self,
        _request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        Err(EffectPrepareError {
            code: "fixture.prepare.unsupported",
        })
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        Ok(None)
    }
}

/// `NativeEffectRegistry::new` over the one factory.
///
/// # Errors
///
/// The registry's refusal.
pub(crate) fn registry(
    descriptor: &'static EffectDescriptor,
) -> Result<NativeEffectRegistry, RegistryError> {
    NativeEffectRegistry::new([Box::new(Factory(descriptor)) as Box<dyn NativeEffectFactory>])
}
