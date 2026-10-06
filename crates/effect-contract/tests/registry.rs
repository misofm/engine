#![allow(missing_docs)]

//! `NativeEffectRegistry::new` is the one release-build descriptor validation point (issue #1330):
//! `validate_prepare_request` only debug-asserts it, so a registry that admitted an invalid main
//! descriptor would let it prepare in release.

use effect_contract::*;

const EFFECT_ID: EffectId = match EffectId::new("registry-test") {
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
fn tail_and_rest(_: u32, _: EffectQuality) -> EffectTailBound {
    EffectTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
    }
}
const fn descriptor(contract_major: u16) -> EffectDescriptor {
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
static VALID: EffectDescriptor = descriptor(1);
static WRONG_CONTRACT_MAJOR: EffectDescriptor = descriptor(2);

struct Factory(&'static EffectDescriptor);

impl NativeEffectFactory for Factory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        self.0
    }

    fn prepare(
        &self,
        _request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        Err(EffectPrepareError {
            code: "fixture.prepare.unsupported",
        })
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}

#[test]
fn registry_refuses_an_invalid_main_descriptor() {
    // The fixture differs from an admissible one only in `contract_major`, so the refusal below is
    // the registry's main-descriptor check and not some unrelated fixture defect.
    assert_eq!(validate_descriptor(&VALID), Ok(()));
    assert!(validate_descriptor(&WRONG_CONTRACT_MAJOR).is_err());
    let registry =
        NativeEffectRegistry::new([Box::new(Factory(&VALID)) as Box<dyn NativeEffectFactory>])
            .expect("the valid twin enters the registry");
    assert!(registry.get(EFFECT_ID).is_some());

    let error = match NativeEffectRegistry::new([
        Box::new(Factory(&WRONG_CONTRACT_MAJOR)) as Box<dyn NativeEffectFactory>
    ]) {
        Ok(_) => panic!("a descriptor with contract_major 2 must not enter the registry"),
        Err(error) => error,
    };
    assert_eq!(
        error,
        RegistryError {
            code: "effect.descriptor.invalid",
            id: Some(EFFECT_ID),
        }
    );
}
