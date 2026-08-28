#![allow(missing_docs)]

use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptor, EffectId, EffectQuality, EnumChoice, LatencySamples,
    LinkModeSet, ObservationCadence, ObservationChannels, ObservationCost,
    ObservationDescriptor, ObservationFold, ObservationKind, ObservationTapId,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, QualityDescriptor,
    SmoothingRule, StatePayloadSizes, TailSamples, validate_descriptor,
};
use miso_engine_effect_package::{
    EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE, EffectArtifactAuthoring, EffectArtifactKind,
    EffectDescriptorWireDiagnosticCode as Code, EffectPackageAuthoring, EffectPackageLimits,
    effect_descriptor_identity, effect_descriptor_wire_required_size, effect_package_cid,
    effect_package_required_size, encode_effect_descriptor_wire, encode_effect_package,
    verify_effect_descriptor_wire, verify_effect_package,
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

static CHOICES: [EnumChoice; 3] = [
    EnumChoice {
        value: -1.0,
        label: "Low",
    },
    EnumChoice {
        value: 0.0,
        label: "Mid",
    },
    EnumChoice {
        value: 1.0,
        label: "High",
    },
];

static PARAMETERS: [ParameterDescriptor; 6] = [
    ParameterDescriptor {
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
    },
    ParameterDescriptor {
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
    },
    ParameterDescriptor {
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
    },
    ParameterDescriptor {
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
    },
    ParameterDescriptor {
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
    },
    ParameterDescriptor {
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
    },
];

const MAIN_IN: PortDescriptor = PortDescriptor {
    id: port_id("main-in"),
    role: PortRole::MainInput,
    required: true,
    layout: PortLayout::DualMonoPlanar,
};
const MAIN_OUT: PortDescriptor = PortDescriptor {
    id: port_id("main-out"),
    role: PortRole::MainOutput,
    required: true,
    layout: PortLayout::DualMonoPlanar,
};
const SIDECHAIN: PortDescriptor = PortDescriptor {
    id: port_id("sidechain"),
    role: PortRole::SidechainInput,
    required: false,
    layout: PortLayout::DualMonoPlanar,
};
static PORTS_UNSORTED: [PortDescriptor; 3] = [SIDECHAIN, MAIN_OUT, MAIN_IN];
static PORTS_PERMUTED: [PortDescriptor; 3] = [MAIN_IN, SIDECHAIN, MAIN_OUT];
static PORTS_MAIN: [PortDescriptor; 2] = [MAIN_OUT, MAIN_IN];

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
) -> QualityDescriptor {
    QualityDescriptor {
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

static QUALITIES_A: [QualityDescriptor; 12] = [
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

static QUALITIES_B: [QualityDescriptor; 8] = [
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

static DESCRIPTOR_A: EffectDescriptor = EffectDescriptor {
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
static OBSERVATIONS_C: [ObservationDescriptor; 2] = [
    ObservationDescriptor {
        id: ObservationTapId(1),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakMagnitude,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    },
    ObservationDescriptor {
        id: ObservationTapId(7),
        display_name: "Reduction Envelope",
        display_unit: "dB",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Computed,
        cadence: ObservationCadence::PerWindow,
        fold: ObservationFold::Latest,
        channels: ObservationChannels::Shared,
        minimum: 0.0,
        maximum: 60.0,
    },
];
static DESCRIPTOR_C: EffectDescriptor = EffectDescriptor {
    id: effect_id("fixture.comprehensive-c"),
    display_name: "Comprehensive C",
    observations: &OBSERVATIONS_C,
    ..DESCRIPTOR_A
};
static DESCRIPTOR_A_PERMUTED: EffectDescriptor = EffectDescriptor {
    ports: &PORTS_PERMUTED,
    ..DESCRIPTOR_A
};
static DESCRIPTOR_B: EffectDescriptor = EffectDescriptor {
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

fn encoded(descriptor: &'static EffectDescriptor) -> Vec<u8> {
    validate_descriptor(descriptor).unwrap();
    let required = effect_descriptor_wire_required_size(descriptor, 1 << 20).unwrap();
    let mut guarded = vec![0x5a; required as usize + 16];
    assert_eq!(
        encode_effect_descriptor_wire(descriptor, 1 << 20, &mut guarded),
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
    ] {
        let wire = encoded(descriptor);
        assert_eq!(wire, hex_bytes(wire_name));
        assert_eq!(
            verify_effect_descriptor_wire(&wire, 1 << 20)
                .unwrap()
                .as_bytes(),
            wire
        );
        assert_eq!(
            effect_descriptor_identity(&wire, 1 << 20)
                .unwrap()
                .as_bytes(),
            hex_bytes(identity_name).as_slice()
        );
        let mut short = vec![0xa5; wire.len() - 1];
        let before = short.clone();
        let error = encode_effect_descriptor_wire(descriptor, 1 << 20, &mut short).unwrap_err();
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
        &miso_engine_parametric_eq::PARAMETRIC_EQ_DESCRIPTOR,
        &miso_engine_soft_clip::SOFT_CLIP_DESCRIPTOR_V1,
        &miso_engine_transient_shaper::TRANSIENT_SHAPER_DESCRIPTOR_V1,
        &miso_engine_true_peak_limiter::TRUE_PEAK_LIMITER_DESCRIPTOR_V1,
    ];
    for descriptor in descriptors {
        let wire = encoded(descriptor);
        verify_effect_descriptor_wire(&wire, 1 << 20).unwrap();
        effect_descriptor_identity(&wire, 1 << 20).unwrap();
        let artifacts = [EffectArtifactAuthoring {
            kind: EffectArtifactKind::Source,
            path: "src/lib.rs",
            target: "",
            features: "",
            content: b"production descriptor package coverage",
        }];
        let authoring = EffectPackageAuthoring {
            descriptor: &wire,
            artifacts: &artifacts,
        };
        let mut package = vec![
            0;
            effect_package_required_size(&authoring, EffectPackageLimits::default()).unwrap()
                as usize
        ];
        encode_effect_package(&authoring, EffectPackageLimits::default(), &mut package)
            .unwrap();
        verify_effect_package(&package, EffectPackageLimits::default()).unwrap();
        effect_package_cid(&package, EffectPackageLimits::default()).unwrap();
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
    let before = *effect_descriptor_identity(original, 1 << 20)
        .unwrap()
        .as_bytes();
    let mut changed = original.to_vec();
    mutate(&mut changed);
    verify_effect_descriptor_wire(&changed, 1 << 20)
        .unwrap_or_else(|error| panic!("{name}: legal semantic mutation rejected: {error:?}"));
    let after = effect_descriptor_identity(&changed, 1 << 20).unwrap();
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
        effect_descriptor_identity(&original, 1 << 20)
            .unwrap()
            .as_bytes(),
        effect_descriptor_identity(&alternate, 1 << 20)
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
        (28, 0, Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE),
        (28, 8, Code::Enum, 28, EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE),
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
        let error = verify_effect_descriptor_wire(&mutated, 1 << 20).unwrap_err();
        assert_eq!(
            (error.code, error.byte_offset, error.record_index),
            (code, offset as u32, index)
        );
    }
    for field in [48, 56, 64, 72, 80] {
        let mut mutated = original.clone();
        put_u32(&mut mutated, field, u32::MAX);
        let error = verify_effect_descriptor_wire(&mutated, u32::MAX).unwrap_err();
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

    let verified = verify_effect_descriptor_wire(&tap_bearing, 1 << 20).unwrap();
    assert_eq!(verified.observation_count(), 2);
    assert_eq!(
        verify_effect_descriptor_wire(&zero_tap, 1 << 20)
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
    let error = verify_effect_descriptor_wire(&mutated, 1 << 20).unwrap_err();
    assert_eq!(error.code, Code::Reserved);
    assert_eq!(error.byte_offset, 92);
}
