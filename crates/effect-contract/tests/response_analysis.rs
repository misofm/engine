#![allow(missing_docs)]

use effect_contract::*;

const EFFECT_ID: EffectId = match EffectId::new("response-test") {
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
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 0,
            right_bytes: 0,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}
const QUALITIES: [QualityDescriptor; 8] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
    quality(176_400),
    quality(192_000),
    quality(352_800),
    quality(384_000),
];
static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: EFFECT_ID,
    display_name: "Response Test",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

static BAD_SECTIONS: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
    ResponseSectionDescriptor {
        id: 1,
        name: "first",
    },
];
static BAD_RESPONSE: ResponseAnalysisDescriptor = ResponseAnalysisDescriptor {
    id: 0,
    name: "Bad Response",
    sections: &BAD_SECTIONS,
    axis_unit: ParameterUnit::Hz,
    unit: ParameterUnit::Db,
    amplitude_reference: ResponseAmplitudeReference::Unity,
    channels: ResponseChannelLayout::Independent,
    mode: ResponseAnalysisMode::RequestedConfiguration,
    cadence: ResponseQueryCadence::ExplicitQuery,
    section_output: ResponseSectionOutput::TotalAndOptionalSections,
    cost: ObservationCost::Computed,
    floor_db: -120.0,
    total_scope: ResponseTotalScope::ParametricEqCascade,
    bypass: ResponseBypassSemantics::EffectWideIdentityWithSections,
};

struct BadResponseFactory;

impl NativeEffectFactory for BadResponseFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DESCRIPTOR
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

    fn response_analysis(&self) -> Option<&dyn NativeEffectResponseFactory> {
        Some(self)
    }
}

impl NativeEffectResponseFactory for BadResponseFactory {
    fn analysis_descriptor(&self) -> &'static ResponseAnalysisDescriptor {
        &BAD_RESPONSE
    }

    fn prepare_response(
        &self,
        _request: PrepareEffectRequest<'_>,
        _limits: ResponsePrepareLimits,
    ) -> Result<Box<dyn PreparedResponseAnalysis>, ResponseAnalysisError> {
        Err(ResponseAnalysisError::UnsupportedCapability)
    }
}

#[test]
fn malformed_companion_response_prevents_registry_admission() {
    let error = match NativeEffectRegistry::new([
        Box::new(BadResponseFactory) as Box<dyn NativeEffectFactory>
    ]) {
        Ok(_) => panic!("malformed companion must not enter the registry"),
        Err(error) => error,
    };
    assert_eq!(error.code, "effect.response.descriptor.invalid");
    assert_eq!(error.id, Some(EFFECT_ID));
}

#[test]
fn response_descriptor_validation_is_allocation_free_and_typed() {
    assert_eq!(
        validate_response_analysis_descriptor(&BAD_RESPONSE),
        Err(ResponseDescriptorError::ZeroId)
    );
}
