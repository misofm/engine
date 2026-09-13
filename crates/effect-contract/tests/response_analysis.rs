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

static VALID_SECTIONS: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 1,
        name: "first",
    },
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
];
static DESCENDING_SECTIONS: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
    ResponseSectionDescriptor {
        id: 1,
        name: "first",
    },
];
static DUPLICATE_SECTIONS: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 1,
        name: "first",
    },
    ResponseSectionDescriptor {
        id: 1,
        name: "second",
    },
];
static ZERO_SECTION_ID: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 0,
        name: "first",
    },
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
];
static EMPTY_SECTIONS: [ResponseSectionDescriptor; 0] = [];
static MANY_SECTIONS: [ResponseSectionDescriptor; 65] = [ResponseSectionDescriptor {
    id: 1,
    name: "section",
}; 65];
static LONG_NAME: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
static INVALID_SECTION_NAME: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor { id: 1, name: "" },
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
];
static LONG_SECTION_NAME: [ResponseSectionDescriptor; 2] = [
    ResponseSectionDescriptor {
        id: 1,
        name: LONG_NAME,
    },
    ResponseSectionDescriptor {
        id: 2,
        name: "second",
    },
];
static VALID_RESPONSE: ResponseAnalysisDescriptor = ResponseAnalysisDescriptor {
    id: 1,
    name: "Valid Response",
    sections: &VALID_SECTIONS,
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

struct BadResponseFactory {
    response: &'static ResponseAnalysisDescriptor,
}

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
        self.response
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
fn valid_companion_response_is_admissible() {
    assert_eq!(
        validate_response_analysis_descriptor(&VALID_RESPONSE),
        Ok(())
    );
    assert!(
        NativeEffectRegistry::new([Box::new(BadResponseFactory {
            response: &VALID_RESPONSE,
        }) as Box<dyn NativeEffectFactory>])
        .is_ok()
    );
}

fn assert_registry_rejects(response: &'static ResponseAnalysisDescriptor) {
    let error = match NativeEffectRegistry::new([
        Box::new(BadResponseFactory { response }) as Box<dyn NativeEffectFactory>
    ]) {
        Ok(_) => panic!("malformed companion must not enter the registry"),
        Err(error) => error,
    };
    assert_eq!(error.code, "effect.response.descriptor.invalid");
    assert_eq!(error.id, Some(EFFECT_ID));
}

#[test]
fn companion_descriptor_mutations_are_independently_rejected() {
    let mut zero_id = VALID_RESPONSE;
    zero_id.id = 0;
    let mut empty_name = VALID_RESPONSE;
    empty_name.name = "";
    let mut long_name = VALID_RESPONSE;
    long_name.name = LONG_NAME;
    let mut empty_sections = VALID_RESPONSE;
    empty_sections.sections = &EMPTY_SECTIONS;
    let mut many_sections = VALID_RESPONSE;
    many_sections.sections = &MANY_SECTIONS;
    let mut zero_section_id = VALID_RESPONSE;
    zero_section_id.sections = &ZERO_SECTION_ID;
    let mut duplicate_sections = VALID_RESPONSE;
    duplicate_sections.sections = &DUPLICATE_SECTIONS;
    let mut descending_sections = VALID_RESPONSE;
    descending_sections.sections = &DESCENDING_SECTIONS;
    let mut invalid_section_name = VALID_RESPONSE;
    invalid_section_name.sections = &INVALID_SECTION_NAME;
    let mut long_section_name = VALID_RESPONSE;
    long_section_name.sections = &LONG_SECTION_NAME;
    let mut nan_floor = VALID_RESPONSE;
    nan_floor.floor_db = f32::NAN;
    let mut zero_floor = VALID_RESPONSE;
    zero_floor.floor_db = 0.0;
    let mut wrong_axis = VALID_RESPONSE;
    wrong_axis.axis_unit = ParameterUnit::Db;
    let mut wrong_unit = VALID_RESPONSE;
    wrong_unit.unit = ParameterUnit::Hz;
    let mut wrong_cost = VALID_RESPONSE;
    wrong_cost.cost = ObservationCost::Resident;
    let mut eq_without_bypass = VALID_RESPONSE;
    eq_without_bypass.bypass = ResponseBypassSemantics::NoEffectBypass;
    let mut builtin_with_bypass = VALID_RESPONSE;
    builtin_with_bypass.total_scope = ResponseTotalScope::BuiltinInputFilterSubtotal;
    builtin_with_bypass.bypass = ResponseBypassSemantics::EffectWideIdentityWithSections;
    let mut builtin_scope = VALID_RESPONSE;
    builtin_scope.total_scope = ResponseTotalScope::BuiltinInputFilterSubtotal;
    builtin_scope.bypass = ResponseBypassSemantics::NoEffectBypass;

    let cases = [
        (zero_id, ResponseDescriptorError::ZeroId),
        (empty_name, ResponseDescriptorError::InvalidName),
        (long_name, ResponseDescriptorError::InvalidName),
        (empty_sections, ResponseDescriptorError::SectionCount),
        (many_sections, ResponseDescriptorError::SectionCount),
        (zero_section_id, ResponseDescriptorError::SectionOrder),
        (duplicate_sections, ResponseDescriptorError::SectionOrder),
        (descending_sections, ResponseDescriptorError::SectionOrder),
        (invalid_section_name, ResponseDescriptorError::InvalidName),
        (long_section_name, ResponseDescriptorError::InvalidName),
        (nan_floor, ResponseDescriptorError::Floor),
        (zero_floor, ResponseDescriptorError::Floor),
        (wrong_axis, ResponseDescriptorError::Semantics),
        (wrong_unit, ResponseDescriptorError::Semantics),
        (wrong_cost, ResponseDescriptorError::Semantics),
        (eq_without_bypass, ResponseDescriptorError::Semantics),
        (builtin_with_bypass, ResponseDescriptorError::Semantics),
    ];
    for (descriptor, expected) in cases {
        assert_eq!(
            validate_response_analysis_descriptor(&descriptor),
            Err(expected)
        );
        let descriptor: &'static ResponseAnalysisDescriptor = Box::leak(Box::new(descriptor));
        assert_registry_rejects(descriptor);
    }
    assert_eq!(
        validate_response_analysis_descriptor(&builtin_scope),
        Ok(())
    );
}
