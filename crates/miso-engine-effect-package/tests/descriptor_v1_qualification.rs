#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, EffectId, EffectQuality, EnumChoiceV1, LatencySamples,
    LinkModeSet, NudgeLadderV1, ObservationCadenceV1, ObservationChannelsV1, ObservationCostV1,
    ObservationDescriptorV1, ObservationFoldV1, ObservationKindV1, ObservationTapId,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole, QualityDescriptorV1,
    SmoothingRule, StatePayloadSizes, TailSamples, validate_descriptor_v1,
};
use miso_engine_effect_package::{
    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE, EffectArtifactAuthoringV1, EffectArtifactKindV1,
    EffectDescriptorBindingErrorKindV1, EffectDescriptorWireDiagnosticCodeV1,
    EffectDescriptorWireDiagnosticCodeV1 as Code, EffectPackageAuthoringV1, EffectPackageLimitsV1,
    bind_effect_descriptor_wire_v1, effect_descriptor_identity_v1,
    effect_descriptor_wire_v1_required_size, effect_package_cid_v1,
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
        enum_choices: &[],
        nudge: None,
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
        enum_choices: &[],
        nudge: None,
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
        enum_choices: &[],
        nudge: None,
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
        enum_choices: &[],
        nudge: None,
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
        enum_choices: &CHOICES,
        nudge: None,
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
        enum_choices: &[],
        nudge: None,
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
    observations: &[],
};
/// Issue #143: `comprehensive-a` plus a declared observation menu and nothing else.
///
/// The id and the display name are the same byte lengths as A's, so
/// `total(C) - total(A)` is exactly the observation section plus its two strings per tap. That is
/// the formula E10 asserts, in-tree, on both encoders.
static OBSERVATIONS_C: [ObservationDescriptorV1; 2] = [
    ObservationDescriptorV1 {
        id: ObservationTapId(1),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKindV1::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCostV1::Resident,
        cadence: ObservationCadenceV1::PerBlock,
        fold: ObservationFoldV1::PeakMagnitude,
        channels: ObservationChannelsV1::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    },
    ObservationDescriptorV1 {
        id: ObservationTapId(7),
        display_name: "Reduction Envelope",
        display_unit: "dB",
        kind: ObservationKindV1::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCostV1::Computed,
        cadence: ObservationCadenceV1::PerWindow,
        fold: ObservationFoldV1::Latest,
        channels: ObservationChannelsV1::Shared,
        minimum: 0.0,
        maximum: 60.0,
    },
];
static DESCRIPTOR_C: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("fixture.comprehensive-c"),
    display_name: "Comprehensive C",
    observations: &OBSERVATIONS_C,
    ..DESCRIPTOR_A
};
/// Issue #127: `comprehensive-a` plus three declared nudge ladders and nothing else.
///
/// The id and the display name are the same byte lengths as A's, so `total(D)` must equal
/// `total(A)` exactly -- a ladder rides the eight bytes the parameter record already reserved and
/// costs none of its own. The three ladders cover the three step units a continuous or enumerated
/// parameter can declare, and the three parameters left ladder-free cover the three reasons a
/// parameter has none: an exponential mapping (`Time`, `Ratio`) and a boolean domain (`Bypass`).
static PARAMETERS_D: [ParameterDescriptorV1; 6] = {
    let mut parameters = PARAMETERS;
    parameters[0].nudge = Some(NudgeLadderV1::absolute(0.5));
    parameters[1].nudge = Some(NudgeLadderV1::cents(20.0));
    parameters[4].nudge = Some(NudgeLadderV1::steps(1));
    parameters
};
static DESCRIPTOR_D: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("fixture.comprehensive-d"),
    display_name: "Comprehensive D",
    parameters: &PARAMETERS_D,
    ..DESCRIPTOR_A
};
/// `comprehensive-d` with one ladder's magnitude changed and nothing else.
///
/// It exists so that "the wire declares a different ladder" is testable at the *magnitude*, not
/// only at the step unit: a comparison that checked the unit and the class but read the magnitude
/// back out of the wire would still bind this, and it must not.
static PARAMETERS_D_ALTERNATE: [ParameterDescriptorV1; 6] = {
    let mut parameters = PARAMETERS_D;
    parameters[0].nudge = Some(NudgeLadderV1::absolute(0.25));
    parameters
};
static DESCRIPTOR_D_ALTERNATE: EffectDescriptorV1 = EffectDescriptorV1 {
    parameters: &PARAMETERS_D_ALTERNATE,
    ..DESCRIPTOR_D
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
    observations: &[],
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
        (
            &DESCRIPTOR_C,
            "comprehensive-c.wire.hex",
            "comprehensive-c.identity.hex",
        ),
        (
            &DESCRIPTOR_D,
            "comprehensive-d.wire.hex",
            "comprehensive-d.identity.hex",
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

/// Issue #143 E10: the observation section is exactly additive, and a stale reader refuses it.
///
/// Three statements, each about bytes rather than about intent:
///
/// 1. **Zero taps move nothing.** Every production descriptor and both pre-#143 fixtures encode to
///    the identity they had before the section existed -- proven by the fixture comparison above,
///    and here by the header window staying eight zeros.
/// 2. **A tap-bearing total is the formula.** `total(C) - total(A)` is `32` per tap plus the two
///    declared strings, and nothing else.
/// 3. **A stale reader refuses rather than ignores.** The pre-#143 verifier's rule was "bytes
///    88..96 are reserved zero". A tap-bearing descriptor breaks it at byte 88, which is a refusal
///    with an exact offset, not a silently dropped menu.
#[test]
fn observation_section_is_additive_and_stale_readers_refuse_it() {
    let zero_tap = encoded(&DESCRIPTOR_A);
    let tap_bearing = encoded(&DESCRIPTOR_C);
    let expected_delta: usize = OBSERVATIONS_C
        .iter()
        .map(|observation| 32 + observation.display_name.len() + observation.display_unit.len())
        .sum();
    assert_eq!(tap_bearing.len() - zero_tap.len(), expected_delta);
    assert_eq!(expected_delta, 2 * 32 + 14 + 2 + 18 + 2);

    // The pre-#143 header rule, spelled out so the refusal is checked rather than assumed.
    let stale_reader_reserved_zero = |wire: &[u8]| wire[88..96].iter().position(|byte| *byte != 0);
    assert_eq!(stale_reader_reserved_zero(&zero_tap), None);
    assert_eq!(stale_reader_reserved_zero(&tap_bearing), Some(0));

    let verified = verify_effect_descriptor_wire_v1(&tap_bearing, 1 << 20).unwrap();
    assert_eq!(verified.observation_count(), 2);
    assert_eq!(
        verify_effect_descriptor_wire_v1(&zero_tap, 1 << 20)
            .unwrap()
            .observation_count(),
        0
    );

    // The E10 red mutation, run as a positive assertion: writing `string_offset` into the header's
    // observation-offset word for a zero-tap descriptor is refused, so it can never become the
    // encoder's habit and silently move every non-dynamics identity.
    let mut mutated = zero_tap.clone();
    let string_offset = u32::from_le_bytes(mutated[84..88].try_into().unwrap());
    mutated[92..96].copy_from_slice(&string_offset.to_le_bytes());
    let error = verify_effect_descriptor_wire_v1(&mutated, 1 << 20).unwrap_err();
    assert_eq!(error.code, Code::Reserved);
    assert_eq!(error.byte_offset, 92);
}

/// Issue #127 wire accounting: a declared ladder costs zero bytes and moves the identity.
///
/// `DESCRIPTOR_D` is `DESCRIPTOR_A` with three ladders and two renamed letters, so the two wires
/// must be the same length, must differ only inside the eight reserved bytes of the parameter
/// records that declare a ladder (plus those two letters), and must not share an identity. That is
/// the whole additivity claim, stated as bytes rather than as prose.
#[test]
fn a_declared_nudge_ladder_costs_no_bytes_and_moves_the_identity() {
    let ladder_free = encoded(&DESCRIPTOR_A);
    let ladder_bearing = encoded(&DESCRIPTOR_D);
    assert_eq!(
        ladder_free.len(),
        ladder_bearing.len(),
        "a nudge ladder rides reserved bytes and adds none"
    );
    let identity = |wire: &[u8]| {
        *effect_descriptor_identity_v1(wire, 1 << 20)
            .unwrap()
            .as_bytes()
    };
    assert_ne!(
        identity(&ladder_free),
        identity(&ladder_bearing),
        "a descriptor that declares a ladder describes something different"
    );

    let parameter_offset = 96usize;
    let string_offset = u32::from_le_bytes(ladder_free[84..88].try_into().unwrap()) as usize;
    let mut windows = std::collections::BTreeSet::new();
    for index in 0..DESCRIPTOR_A.parameters.len() {
        let record = parameter_offset + index * 80;
        windows.extend(record + 72..record + 80);
        assert_eq!(
            &ladder_free[record + 72..record + 80],
            &[0u8; 8],
            "a ladder-free parameter keeps the window reserved-zero"
        );
    }
    let moved: Vec<usize> = (0..ladder_free.len())
        .filter(|offset| ladder_free[*offset] != ladder_bearing[*offset])
        .collect();
    let renamed: Vec<usize> = moved
        .iter()
        .copied()
        .filter(|offset| *offset >= string_offset)
        .collect();
    assert_eq!(
        renamed.len(),
        2,
        "only the two renamed letters move the pool"
    );
    let ladder_bytes: Vec<usize> = moved
        .iter()
        .copied()
        .filter(|offset| *offset < string_offset)
        .collect();
    assert!(
        !ladder_bytes.is_empty(),
        "the ladders were actually written"
    );
    assert!(
        ladder_bytes.iter().all(|offset| windows.contains(offset)),
        "a ladder writes only inside the reserved windows"
    );

    // And the wire round-trips back to exactly this descriptor, ladders included: a wire that
    // declared a different ladder would not bind -- neither one that declares none, nor one whose
    // rung is a different size.
    assert!(bind_effect_descriptor_wire_v1(&DESCRIPTOR_D, &ladder_bearing, 1 << 20).is_ok());
    assert_eq!(
        bind_effect_descriptor_wire_v1(&DESCRIPTOR_A, &ladder_bearing, 1 << 20)
            .unwrap_err()
            .kind(),
        EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch
    );
    let alternate = encoded(&DESCRIPTOR_D_ALTERNATE);
    assert_eq!(
        alternate.len(),
        ladder_bearing.len(),
        "the two differ only in one rung's magnitude"
    );
    assert_ne!(alternate, ladder_bearing);
    assert_eq!(
        bind_effect_descriptor_wire_v1(&DESCRIPTOR_D, &alternate, 1 << 20)
            .unwrap_err()
            .kind(),
        EffectDescriptorBindingErrorKindV1::StaticDescriptorMismatch,
        "a wire whose xs rung is a different size is not this descriptor"
    );
}

/// Issue #127: a wire whose declared ladder breaks a rule is refused as semantically invalid.
///
/// The wire verifier runs the contract's own three rules through `check_nudge_ladder_parts_v1`, so
/// a hand-crafted wire that a legitimate encoder would never produce is still refused. Nothing
/// this crate encodes can reach this path, which is exactly why it needs a test that builds the
/// bytes by hand.
///
/// Red mutation: delete the ladder arm from `borrowed_semantic_errors`.
#[test]
fn a_wire_whose_ladder_breaks_a_rule_is_semantically_invalid() {
    let wire = encoded(&DESCRIPTOR_D);
    let record = 96_usize;
    // Parameter 0 is a dB level spanning 72 dB with a 0.5 dB xs rung. Three decibels times thirty
    // is ninety, which is more domain than the parameter has.
    let mut too_coarse = wire.clone();
    too_coarse[record + 72..record + 76].copy_from_slice(&3.0_f32.to_bits().to_le_bytes());
    let diagnostic = verify_effect_descriptor_wire_v1(&too_coarse, 1 << 20)
        .expect_err("an xl rung that crosses the domain is refused");
    assert_eq!(
        diagnostic.code,
        EffectDescriptorWireDiagnosticCodeV1::Semantic
    );
    assert_eq!(diagnostic.byte_offset as usize, record + 72);
    // A cents ladder on a linear mapping has no ratio to step by.
    let mut wrong_unit = wire.clone();
    wrong_unit[record + 76] = 2;
    assert_eq!(
        verify_effect_descriptor_wire_v1(&wrong_unit, 1 << 20)
            .expect_err("a cents rung on a linear mapping is refused")
            .code,
        EffectDescriptorWireDiagnosticCodeV1::Semantic
    );
    // And a whole-choice rung must be a whole number.
    let mode = record + 4 * 80;
    let mut fractional = wire.clone();
    fractional[mode + 72..mode + 76].copy_from_slice(&1.5_f32.to_bits().to_le_bytes());
    assert_eq!(
        verify_effect_descriptor_wire_v1(&fractional, 1 << 20)
            .expect_err("a fractional choice count is refused")
            .code,
        EffectDescriptorWireDiagnosticCodeV1::Semantic
    );
}

/// Issue #127: the eight bytes a ladder rides stay reserved, whether or not one is declared.
///
/// Red mutation: drop either leg of the reserved check in `parse_borrowed_wire`'s phase 5.
#[test]
fn the_nudge_window_is_reserved_whether_or_not_a_ladder_is_declared() {
    let wire = encoded(&DESCRIPTOR_D);
    let record = |index: usize| 96 + index * 80;
    // Parameter 2 (`Time`) declares no ladder. Every byte of its window except the presence bit
    // must stay zero, and the diagnostic names the exact byte that moved.
    for offset in (72..76).chain(77..80) {
        let mut mutated = wire.clone();
        mutated[record(2) + offset] = 1;
        let diagnostic = verify_effect_descriptor_wire_v1(&mutated, 1 << 20)
            .expect_err("a nonzero byte in a ladder-free window is refused");
        assert_eq!(
            diagnostic.code,
            EffectDescriptorWireDiagnosticCodeV1::Reserved,
            "byte {offset} of a ladder-free window"
        );
        assert_eq!(diagnostic.byte_offset as usize, record(2) + offset);
    }
    // Setting the presence bit alone claims a ladder with no ratio class, which is a closed
    // vocabulary refusing an unknown value rather than a reserved byte moving.
    let mut presence = wire.clone();
    presence[record(2) + 76] = 1;
    assert_eq!(
        verify_effect_descriptor_wire_v1(&presence, 1 << 20)
            .expect_err("a declared ladder needs a ratio class")
            .code,
        EffectDescriptorWireDiagnosticCodeV1::Enum
    );
    // Parameter 0 declares one: the two bytes past the class are still reserved.
    for offset in 78..80 {
        let mut mutated = wire.clone();
        mutated[record(0) + offset] = 1;
        assert_eq!(
            verify_effect_descriptor_wire_v1(&mutated, 1 << 20)
                .expect_err("the window's tail is reserved")
                .code,
            EffectDescriptorWireDiagnosticCodeV1::Reserved
        );
    }
    // And the two vocabularies are closed.
    for (offset, value) in [(76_usize, 5_u8), (77, 3)] {
        let mut mutated = wire.clone();
        mutated[record(0) + offset] = value;
        assert_eq!(
            verify_effect_descriptor_wire_v1(&mutated, 1 << 20)
                .expect_err("an unknown nudge vocabulary value is refused")
                .code,
            EffectDescriptorWireDiagnosticCodeV1::Enum
        );
    }
}
