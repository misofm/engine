#![no_main]

use libfuzzer_sys::fuzz_target;
use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, EffectId, EffectQuality, LatencySamples, LinkModeSet,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole, QualityDescriptorV1,
    SmoothingRule, StatePayloadSizes, TailSamples,
};
use miso_engine_effect_package::{
    bind_effect_descriptor_wire_v1, effect_descriptor_wire_v1_required_size,
    encode_effect_descriptor_wire_v1, verify_effect_state_v1, EffectStateLimitsV1,
};
use std::sync::LazyLock;

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("port id"),
    }
}

static PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
    id: ParameterId(1),
    display_name: "Value",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(-1.0),
    maximum: Some(1.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::Shared,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
}];
static PORTS: [PortDescriptorV1; 2] = [
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
];
const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: 1,
            right_bytes: 1,
        },
        scratch_fixed_bytes: 1,
        scratch_bytes_per_frame: 0,
    }
}
static QUALITIES: [QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];
static DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("fuzz.state"),
    display_name: "Fuzz state",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
};
static DESCRIPTOR_WIRE: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let required = effect_descriptor_wire_v1_required_size(&DESCRIPTOR, 1 << 20).unwrap();
    let mut wire = vec![0; required as usize];
    encode_effect_descriptor_wire_v1(&DESCRIPTOR, 1 << 20, &mut wire).unwrap();
    wire
});

fuzz_target!(|data: &[u8]| {
    let bound = bind_effect_descriptor_wire_v1(&DESCRIPTOR, &DESCRIPTOR_WIRE, 1 << 20).unwrap();
    let _ = verify_effect_state_v1(bound, data, EffectStateLimitsV1::default());
});
