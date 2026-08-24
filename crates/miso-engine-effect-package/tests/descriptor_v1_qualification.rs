#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, EffectId, EffectQuality, EnumChoiceV1, LatencySamples,
    LinkModeSet, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    QualityDescriptorV1, SmoothingRule, StatePayloadSizes, TailSamples, validate_descriptor_v1,
};
use miso_engine_effect_package::{
    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE, EffectArtifactAuthoringV1, EffectArtifactKindV1,
    EffectDescriptorWireDiagnosticCodeV1 as Code, EffectPackageAuthoringV1, EffectPackageLimitsV1,
    effect_descriptor_identity_v1, effect_descriptor_wire_v1_required_size, effect_package_cid_v1,
    effect_package_v1_required_size, encode_effect_descriptor_wire_v1, encode_effect_package_v1,
    verify_effect_descriptor_wire_v1, verify_effect_package_v1,
};
use std::{fs, path::PathBuf};

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("fixture effect ID must be valid"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("fixture port ID must be valid"),
    }
}

static CHOICES: [EnumChoiceV1; 3] = [
    EnumChoiceV1 {
        value: -1.0,
        label: "Low",
    },
    EnumChoiceV1 {
        value: 0.0,
        label: "Mid",
    },
    EnumChoiceV1 {
        value: 1.0,
        label: "High",
    },
];

static PARAMETERS: [ParameterDescriptorV1; 6] = [
    ParameterDescriptorV1 {
        id: ParameterId(1),
        display_name: "Gain",
        display_unit: "dB",
        unit: ParameterUnit::Db,
        domain: ParameterDomain::Continuous,
        minimum: Some(-60.0),
        maximum: Some(12.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Sample,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: 64,
        readable: false,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(2),
        display_name: "Frequency",
        display_unit: "Hz",
        unit: ParameterUnit::Hz,
        domain: ParameterDomain::Continuous,
        minimum: Some(20.0),
        maximum: Some(20_000.0),
        default_value: 1_000.0,
        mapping: ParameterMapping::Logarithmic,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::OnePole99,
        smoothing_samples: 32,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(3),
        display_name: "Time",
        display_unit: "ms",
        unit: ParameterUnit::Milliseconds,
        domain: ParameterDomain::Continuous,
        minimum: Some(0.0),
        maximum: Some(1_000.0),
        default_value: 10.0,
        mapping: ParameterMapping::Exponential,
        automation_rate: AutomationRate::Sample,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(4),
        display_name: "Bypass",
        display_unit: "state",
        unit: ParameterUnit::Samples,
        domain: ParameterDomain::Boolean,
        minimum: None,
        maximum: None,
        default_value: 0.0,
        mapping: ParameterMapping::Stepped,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(5),
        display_name: "Mode",
        display_unit: "choice",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Enumeration,
        minimum: None,
        maximum: None,
        default_value: 0.0,
        mapping: ParameterMapping::Stepped,
        automation_rate: AutomationRate::None,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: false,
        nudge_ladder: None,
        enum_choices: &CHOICES,
    },
    ParameterDescriptorV1 {
        id: ParameterId(6),
        display_name: "Ratio",
        display_unit: ":1",
        unit: ParameterUnit::Ratio,
        domain: ParameterDomain::Continuous,
        minimum: Some(1.0),
        maximum: Some(20.0),
        default_value: 4.0,
        mapping: ParameterMapping::Exponential,
        automation_rate: AutomationRate::Sample,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::OnePole99,
        smoothing_samples: 16,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
];

const MAIN_IN: PortDescriptorV1 = PortDescriptorV1 {
    id: port_id("main-in"),
    role: PortRole::MainInput,
    required: true,
    layout: PortLayout::DualMonoPlanar,
};
const MAIN_OUT: PortDescriptorV1 = PortDescriptorV1 {
    id: port_id("main-out"),
    role: PortRole::MainOutput,
    required: true,
    layout: PortLayout::DualMonoPlanar,
};
const SIDECHAIN: PortDescriptorV1 = PortDescriptorV1 {
    id: port_id("sidechain"),
    role: PortRole::SidechainInput,
    required: false,
    layout: PortLayout::DualMonoPlanar,
};
static PORTS_UNSORTED: [PortDescriptorV1; 3] = [SIDECHAIN, MAIN_OUT, MAIN_IN];
static PORTS_PERMUTED: [PortDescriptorV1; 3] = [MAIN_IN, SIDECHAIN, MAIN_OUT];
static PORTS_MAIN: [PortDescriptorV1; 2] = [MAIN_OUT, MAIN_IN];

#[allow(clippy::too_many_arguments)]
const fn quality(
    quality: EffectQuality,
    sample_rate: u32,
    latency: u64,
    tail: TailSamples,
    common: u32,
    lane: u32,
    scratch: u64,
    per_frame: u64,
) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality,
        sample_rate,
        latency: LatencySamples(latency),
        tail,
        maximum_state: StatePayloadSizes {
            common_bytes: common,
            left_bytes: lane,
            right_bytes: lane,
        },
        scratch_fixed_bytes: scratch,
        scratch_bytes_per_frame: per_frame,
    }
}

static QUALITIES_A: [QualityDescriptorV1; 12] = [
    quality(
        EffectQuality::Draft,
        44_100,
        0,
        TailSamples::Finite(0),
        8,
        16,
        32,
        4,
    ),
    quality(
        EffectQuality::Draft,
        48_000,
        0,
        TailSamples::Finite(0),
        8,
        16,
        32,
        4,
    ),
    quality(
        EffectQuality::Draft,
        88_200,
        0,
        TailSamples::Finite(0),
        8,
        16,
        32,
        4,
    ),
    quality(
        EffectQuality::Draft,
        96_000,
        0,
        TailSamples::Finite(0),
        8,
        16,
        32,
        4,
    ),
    quality(
        EffectQuality::Normal,
        44_100,
        4,
        TailSamples::Finite(256),
        24,
        32,
        64,
        8,
    ),
    quality(
        EffectQuality::Normal,
        48_000,
        4,
        TailSamples::Finite(256),
        24,
        32,
        64,
        8,
    ),
    quality(
        EffectQuality::Normal,
        88_200,
        4,
        TailSamples::Finite(256),
        24,
        32,
        64,
        8,
    ),
    quality(
        EffectQuality::Normal,
        96_000,
        4,
        TailSamples::Finite(256),
        24,
        32,
        64,
        8,
    ),
    quality(
        EffectQuality::High,
        44_100,
        8,
        TailSamples::Infinite,
        40,
        64,
        128,
        16,
    ),
    quality(
        EffectQuality::High,
        48_000,
        8,
        TailSamples::Infinite,
        40,
        64,
        128,
        16,
    ),
    quality(
        EffectQuality::High,
        88_200,
        8,
        TailSamples::Infinite,
        40,
        64,
        128,
        16,
    ),
    quality(
        EffectQuality::High,
        96_000,
        8,
        TailSamples::Infinite,
        40,
        64,
        128,
        16,
    ),
];

static QUALITIES_B: [QualityDescriptorV1; 8] = [
    quality(
        EffectQuality::Normal,
        44_100,
        u64::MAX,
        TailSamples::Finite(u64::MAX),
        u32::MAX,
        u32::MAX,
        u64::MAX,
        u64::MAX,
    ),
    quality(
        EffectQuality::Normal,
        48_000,
        0,
        TailSamples::Finite(0),
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        88_200,
        0,
        TailSamples::Finite(0),
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        96_000,
        0,
        TailSamples::Finite(0),
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        176_400,
        0,
        TailSamples::Infinite,
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        192_000,
        0,
        TailSamples::Infinite,
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        352_800,
        0,
        TailSamples::Infinite,
        0,
        0,
        0,
        0,
    ),
    quality(
        EffectQuality::Normal,
        384_000,
        0,
        TailSamples::Infinite,
        0,
        0,
        0,
        0,
    ),
];

static DESCRIPTOR_A: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("fixture.comprehensive-a"),
    display_name: "Comprehensive A",
    contract_major: 1,
    contract_minor: 7,
    state_layout_version: 3,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &PARAMETERS,
    ports: &PORTS_UNSORTED,
    qualities: &QUALITIES_A,
};
static DESCRIPTOR_A_PERMUTED: EffectDescriptorV1 = EffectDescriptorV1 {
    ports: &PORTS_PERMUTED,
    ..DESCRIPTOR_A
};
static DESCRIPTOR_B: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("fixture.comprehensive-b"),
    display_name: "Unicode\u{2028}Boundary",
    contract_major: 1,
    contract_minor: u16::MAX,
    state_layout_version: u32::MAX,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &[],
    ports: &PORTS_MAIN,
    qualities: &QUALITIES_B,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/effect-descriptor/v1")
        .join(name)
}

fn hex_bytes(name: &str) -> Vec<u8> {
    let text = fs::read_to_string(fixture(name)).unwrap();
    let digits: Vec<_> = text
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    assert_eq!(digits.len() % 2, 0);
    digits
        .chunks_exact(2)
        .map(|pair| {
            let digit = |value: u8| match value {
                b'0'..=b'9' => value - b'0',
                b'a'..=b'f' => value - b'a' + 10,
                _ => panic!("fixture contains non-hex input"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn encoded(descriptor: &'static EffectDescriptorV1) -> Vec<u8> {
    validate_descriptor_v1(descriptor).unwrap();
    let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
    let mut guarded = vec![0x5a; required as usize + 16];
    assert_eq!(
        encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut guarded),
        Ok(required)
    );
    assert_eq!(&guarded[required as usize..], &[0x5a; 16]);
    guarded.truncate(required as usize);
    guarded
}

#[test]
fn checked_vectors_match_independent_wire_identity_and_port_permutation() {
    for (descriptor, wire_name, identity_name) in [
        (
            &DESCRIPTOR_A,
            "comprehensive-a.wire.hex",
            "comprehensive-a.identity.hex",
        ),
        (
            &DESCRIPTOR_B,
            "comprehensive-b.wire.hex",
            "comprehensive-b.identity.hex",
        ),
    ] {
        let wire = encoded(descriptor);
        assert_eq!(wire, hex_bytes(wire_name));
        assert_eq!(
            verify_effect_descriptor_wire_v1(&wire, 1 << 20)
                .unwrap()
                .as_bytes(),
            wire
        );
        assert_eq!(
            effect_descriptor_identity_v1(&wire, 1 << 20)
                .unwrap()
                .as_bytes(),
            hex_bytes(identity_name).as_slice()
        );
        let mut short = vec![0xa5; wire.len() - 1];
        let before = short.clone();
        let error = encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut short).unwrap_err();
        assert_eq!(
            (error.code, error.required_bytes),
            (Code::BufferTooSmall, wire.len() as u32)
        );
        assert_eq!(short, before);
    }
    assert_eq!(encoded(&DESCRIPTOR_A), encoded(&DESCRIPTOR_A_PERMUTED));
}

#[test]
fn every_current_production_descriptor_encodes_and_verifies() {
    let descriptors = [
        &miso_engine_compressor::COMPRESSOR_DESCRIPTOR_V1,
        &miso_engine_delay::DELAY_DESCRIPTOR_V1,
        &miso_engine_gate_expander::GATE_EXPANDER_DESCRIPTOR_V1,
        &miso_engine_multiband_compressor::MULTIBAND_COMPRESSOR_DESCRIPTOR_V1,
        &miso_engine_parametric_eq::PARAMETRIC_EQ_DESCRIPTOR_V1,
        &miso_engine_soft_clip::SOFT_CLIP_DESCRIPTOR_V1,
        &miso_engine_transient_shaper::TRANSIENT_SHAPER_DESCRIPTOR_V1,
        &miso_engine_true_peak_limiter::TRUE_PEAK_LIMITER_DESCRIPTOR_V1,
    ];
    for descriptor in descriptors {
        let wire = encoded(descriptor);
        verify_effect_descriptor_wire_v1(&wire, 1 << 20).unwrap();
        effect_descriptor_identity_v1(&wire, 1 << 20).unwrap();
        let artifacts = [EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: "src/lib.rs",
            target: "",
            features: "",
            content: b"production descriptor package coverage",
        }];
        let authoring = EffectPackageAuthoringV1 {
            descriptor: &wire,
            artifacts: &artifacts,
        };
        let mut package = vec![
            0;
            effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default()).unwrap()
                as usize
        ];
        encode_effect_package_v1(&authoring, EffectPackageLimitsV1::default(), &mut package)
            .unwrap();
        verify_effect_package_v1(&package, EffectPackageLimitsV1::default()).unwrap();
        effect_package_cid_v1(&package, EffectPackageLimitsV1::default()).unwrap();
    }
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn assert_valid_identity_change(original: &[u8], name: &str, mutate: impl FnOnce(&mut [u8])) {
    let before = *effect_descriptor_identity_v1(original, 1 << 20)
        .unwrap()
        .as_bytes();
    let mut changed = original.to_vec();
    mutate(&mut changed);
    verify_effect_descriptor_wire_v1(&changed, 1 << 20)
        .unwrap_or_else(|error| panic!("{name}: legal semantic mutation rejected: {error:?}"));
    let after = effect_descriptor_identity_v1(&changed, 1 << 20).unwrap();
    assert_ne!(&before, after.as_bytes(), "{name}: identity did not change");
}

#[test]
fn every_legally_mutable_semantic_field_class_changes_identity() {
    let original = encoded(&DESCRIPTOR_A);
    let parameter = u32::from_le_bytes(original[52..56].try_into().unwrap()) as usize;
    let port = u32::from_le_bytes(original[60..64].try_into().unwrap()) as usize;
    let quality = u32::from_le_bytes(original[68..72].try_into().unwrap()) as usize;
    let choice = u32::from_le_bytes(original[76..80].try_into().unwrap()) as usize;

    assert_valid_identity_change(&original, "effect ID", |bytes| {
        let offset = u32::from_le_bytes(bytes[32..36].try_into().unwrap()) as usize;
        bytes[offset + "fixture.comprehensive-a".len() - 1] = b'b';
    });
    assert_valid_identity_change(&original, "display name", |bytes| {
        let offset = u32::from_le_bytes(bytes[40..44].try_into().unwrap()) as usize;
        bytes[offset] = b'D';
    });
    assert_valid_identity_change(&original, "contract minor", |bytes| put_u16(bytes, 22, 8));
    assert_valid_identity_change(&original, "state layout", |bytes| put_u32(bytes, 24, 4));
    assert_valid_identity_change(&original, "link modes", |bytes| put_u32(bytes, 28, 1));
    assert_valid_identity_change(&original, "parameter ID", |bytes| {
        put_u32(bytes, parameter + 5 * 80, 7);
    });
    assert_valid_identity_change(&original, "parameter unit", |bytes| {
        put_u32(bytes, parameter + 4, ParameterUnit::Linear as u32);
    });
    assert_valid_identity_change(&original, "parameter domain", |bytes| {
        let record = parameter + 3 * 80;
        put_u32(bytes, record + 8, ParameterDomain::Continuous as u32);
        put_u32(bytes, record + 12, ParameterMapping::Linear as u32);
        put_u32(bytes, record + 32, 15);
        put_u32(bytes, record + 36, 0.0f32.to_bits());
        put_u32(bytes, record + 40, 1.0f32.to_bits());
    });
    assert_valid_identity_change(&original, "parameter mapping", |bytes| {
        put_u32(bytes, parameter + 12, ParameterMapping::Exponential as u32);
    });
    assert_valid_identity_change(&original, "automation rate", |bytes| {
        put_u32(bytes, parameter + 16, AutomationRate::Block as u32);
    });
    assert_valid_identity_change(&original, "channel policy", |bytes| {
        put_u32(bytes, parameter + 20, ParameterChannelPolicy::Shared as u32);
    });
    assert_valid_identity_change(&original, "smoothing rule", |bytes| {
        put_u32(bytes, parameter + 24, SmoothingRule::OnePole99 as u32);
    });
    assert_valid_identity_change(&original, "smoothing samples", |bytes| {
        put_u32(bytes, parameter + 28, 63);
    });
    assert_valid_identity_change(&original, "readable flag", |bytes| {
        put_u32(bytes, parameter + 32, 15);
    });
    assert_valid_identity_change(&original, "automatable flag", |bytes| {
        let record = parameter + 4 * 80;
        put_u32(bytes, record + 16, AutomationRate::Block as u32);
        put_u32(bytes, record + 32, 3);
    });
    assert_valid_identity_change(&original, "minimum", |bytes| {
        put_u32(bytes, parameter + 36, (-59.0f32).to_bits());
    });
    assert_valid_identity_change(&original, "maximum", |bytes| {
        put_u32(bytes, parameter + 40, 11.0f32.to_bits());
    });
    assert_valid_identity_change(&original, "default", |bytes| {
        put_u32(bytes, parameter + 44, 1.0f32.to_bits());
    });
    assert_valid_identity_change(&original, "enum value", |bytes| {
        put_u32(bytes, choice, (-2.0f32).to_bits());
    });
    assert_valid_identity_change(&original, "enum label", |bytes| {
        let offset = u32::from_le_bytes(bytes[choice + 4..choice + 8].try_into().unwrap()) as usize;
        bytes[offset..offset + 3].copy_from_slice(b"Lox");
    });
    assert_valid_identity_change(&original, "port ID", |bytes| {
        let record = port + 2 * 24;
        let offset = u32::from_le_bytes(bytes[record..record + 4].try_into().unwrap()) as usize;
        bytes[offset + 8] = b'j';
    });
    assert_valid_identity_change(&original, "port required", |bytes| {
        put_u32(bytes, port + 2 * 24 + 12, 1);
    });
    assert_valid_identity_change(&original, "latency", |bytes| {
        put_u64(bytes, quality + 8, 1);
    });
    assert_valid_identity_change(&original, "tail kind", |bytes| {
        put_u32(bytes, quality + 16, 2);
    });
    assert_valid_identity_change(&original, "tail samples", |bytes| {
        put_u64(bytes, quality + 24, 1);
    });
    assert_valid_identity_change(&original, "common state", |bytes| {
        put_u32(bytes, quality + 32, 9);
    });
    assert_valid_identity_change(&original, "lane state", |bytes| {
        put_u32(bytes, quality + 36, 17);
        put_u32(bytes, quality + 40, 17);
    });
    assert_valid_identity_change(&original, "fixed scratch", |bytes| {
        put_u64(bytes, quality + 48, 33);
    });
    assert_valid_identity_change(&original, "per-frame scratch", |bytes| {
        put_u64(bytes, quality + 56, 5);
    });

    let alternate = encoded(&DESCRIPTOR_B);
    assert_ne!(
        effect_descriptor_identity_v1(&original, 1 << 20)
            .unwrap()
            .as_bytes(),
        effect_descriptor_identity_v1(&alternate, 1 << 20)
            .unwrap()
            .as_bytes(),
        "parameter/port/quality table-shape classes must change identity"
    );
}

#[test]
fn raw_closed_values_and_field_overflows_have_exact_public_diagnostics() {
    let original = encoded(&DESCRIPTOR_A);
    let parameter = u32::from_le_bytes(original[52..56].try_into().unwrap()) as usize;
    let port = u32::from_le_bytes(original[60..64].try_into().unwrap()) as usize;
    let quality = u32::from_le_bytes(original[68..72].try_into().unwrap()) as usize;
    let cases = [
        (28, 0, Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE),
        (28, 8, Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE),
        (parameter + 4, 0, Code::Enum, parameter + 4, 0),
        (parameter + 8, 0, Code::Enum, parameter + 8, 0),
        (parameter + 12, 0, Code::Enum, parameter + 12, 0),
        (parameter + 16, 0, Code::Enum, parameter + 16, 0),
        (parameter + 20, 0, Code::Enum, parameter + 20, 0),
        (parameter + 24, 0, Code::Enum, parameter + 24, 0),
        (port + 8, 0, Code::Enum, port + 8, 0),
        (port + 12, 2, Code::Enum, port + 12, 0),
        (port + 16, 0, Code::Enum, port + 16, 0),
        (quality, 0, Code::Enum, quality, 0),
        (quality + 16, 0, Code::Enum, quality + 16, 0),
        (parameter + 32, 16, Code::Flags, parameter + 32, 0),
    ];
    for (field, value, code, offset, index) in cases {
        let mut mutated = original.clone();
        put_u32(&mut mutated, field, value);
        let error = verify_effect_descriptor_wire_v1(&mutated, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (code, offset as u32, index)
        );
    }
    for field in [48, 56, 64, 72, 80] {
        let mut mutated = original.clone();
        put_u32(&mut mutated, field, u32::MAX);
        let error = verify_effect_descriptor_wire_v1(&mutated, u32::MAX).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset),
            (Code::Overflow, field as u32)
        );
    }
}
