//! Launch feed-forward peak compressor descriptor and factory scaffold.
//!
//! Scalar processing follows in its own bounded implementation edit.
#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, EffectPrepareError, EffectQuality, LatencySamples,
    LinkModeSet, NativeEffectFactory, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterId, ParameterMapping, ParameterUnit, PortDescriptorV1, PortId,
    PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest, PreparedNativeEffect,
    PreparedNativeEffectBank, SmoothingRule, StatePayloadSizes, TailSamples,
};

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port id"),
    }
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter id"),
    }
}

const fn parameter(
    id: u32,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: parameter_id(id),
        display_name,
        display_unit,
        unit,
        domain: ParameterDomain::Continuous,
        minimum: Some(minimum),
        maximum: Some(maximum),
        default_value,
        mapping,
        automation_rate,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing,
        smoothing_samples,
        readable: true,
        automatable: match automation_rate {
            AutomationRate::None => false,
            _ => true,
        },
        enum_choices: &[],
    }
}

/// Frozen V1 parameter descriptors. Parameter positions and stable IDs are identical.
pub const COMPRESSOR_PARAMETERS_V1: [ParameterDescriptorV1; 8] = [
    parameter(
        1,
        "threshold",
        "dB",
        ParameterUnit::Db,
        -80.0,
        0.0,
        -18.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        2,
        "ratio",
        "ratio",
        ParameterUnit::Ratio,
        1.0,
        20.0,
        4.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        3,
        "knee",
        "dB",
        ParameterUnit::Db,
        0.0,
        24.0,
        6.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        4,
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        0.1,
        200.0,
        10.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        5,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        5.0,
        5000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        6,
        "makeup",
        "dB",
        ParameterUnit::Db,
        -24.0,
        24.0,
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        7,
        "mix",
        "linear",
        ParameterUnit::Linear,
        0.0,
        1.0,
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        8,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        20.0,
        5.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
];

const PORTS: [PortDescriptorV1; 3] = [
    PortDescriptorV1 {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("sidechain-in"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

const fn quality(
    sample_rate: u32,
    latency: u64,
) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let ring_length = latency as u32 + 1;
    let per_lane = (24 + 2 * ring_length) * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(latency),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: per_lane,
            right_bytes: per_lane,
        },
        scratch_fixed_bytes: 64,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100, 882),
    quality(48_000, 960),
    quality(88_200, 1764),
    quality(96_000, 1920),
];

/// Immutable launch compressor descriptor.
pub const COMPRESSOR_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.compressor"),
    display_name: "Compressor",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &COMPRESSOR_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Factory entry point for the V1 compressor implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompressorFactory;

impl NativeEffectFactory for CompressorFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &COMPRESSOR_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        _request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        Err(EffectPrepareError {
            code: "effect.compressor.scalar.pending",
        })
    }

    fn bind_homogeneous_bank(
        &self,
        _request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_effect_contract::validate_descriptor_v1;

    #[test]
    fn descriptor_rows_and_resource_envelope_are_frozen() {
        validate_descriptor_v1(&COMPRESSOR_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(COMPRESSOR_DESCRIPTOR_V1.id.as_str(), "miso.compressor");
        assert_eq!(COMPRESSOR_PARAMETERS_V1.len(), 8);
        assert_eq!(QUALITIES[0].maximum_state.left_bytes, 7_160);
        assert_eq!(QUALITIES[1].latency, LatencySamples(960));
        assert_eq!(QUALITIES[1].maximum_state.left_bytes, 7_784);
        assert_eq!(QUALITIES[3].maximum_state.left_bytes, 15_464);
        assert_eq!(QUALITIES[3].maximum_state.total(), Some(30_928));
        assert_eq!(QUALITIES[1].scratch_fixed_bytes, 64);
    }
}
