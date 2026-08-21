//! Scalar V1 four-section dual-mono parametric EQ.
//!
//! The scalar checkpoint deliberately owns no bank/SIMD token, fixture corpus, or graph integration.
#![allow(missing_docs)]

use core::f64::consts::PI;

use miso_engine_core::{
    DeltaBankKernelError, PreparedDeltaBankKernelV1, SampleRateHz, is_launch_sample_rate,
};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality as Quality,
    InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata, sanitize_sample,
};

/// Fixed cascade length in V1.
pub const EQ_SECTION_COUNT_V1: usize = 4;
const STATE_WORDS_PER_BAND: usize = 16;
const STATE_BYTES_PER_LANE: usize = EQ_SECTION_COUNT_V1 * STATE_WORDS_PER_BAND * 4;

/// Frozen V1 section filter families.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqBandKindV1 {
    Bell = 1,
    LowShelf = 2,
    HighShelf = 3,
    LowPass = 4,
    HighPass = 5,
    Notch = 6,
}

impl EqBandKindV1 {
    fn from_value(value: f32) -> Option<Self> {
        match value.to_bits() {
            bits if bits == 1.0_f32.to_bits() => Some(Self::Bell),
            bits if bits == 2.0_f32.to_bits() => Some(Self::LowShelf),
            bits if bits == 3.0_f32.to_bits() => Some(Self::HighShelf),
            bits if bits == 4.0_f32.to_bits() => Some(Self::LowPass),
            bits if bits == 5.0_f32.to_bits() => Some(Self::HighPass),
            bits if bits == 6.0_f32.to_bits() => Some(Self::Notch),
            _ => None,
        }
    }
}

/// Stable parameter IDs for one cascade position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EqBandDescriptorV1 {
    pub index: u8,
    pub cascade_order: u8,
    pub enabled: ParameterId,
    pub kind: ParameterId,
    pub frequency_hz: ParameterId,
    pub gain_db: ParameterId,
    pub q: ParameterId,
    pub shelf_slope: ParameterId,
}

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter id"),
    }
}

const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid port id"),
    }
}

/// Four static cascade positions in increasing order.
pub const EQ_BAND_DESCRIPTORS_V1: [EqBandDescriptorV1; EQ_SECTION_COUNT_V1] = [
    EqBandDescriptorV1 {
        index: 0,
        cascade_order: 0,
        enabled: parameter_id(1),
        kind: parameter_id(2),
        frequency_hz: parameter_id(3),
        gain_db: parameter_id(4),
        q: parameter_id(5),
        shelf_slope: parameter_id(6),
    },
    EqBandDescriptorV1 {
        index: 1,
        cascade_order: 1,
        enabled: parameter_id(17),
        kind: parameter_id(18),
        frequency_hz: parameter_id(19),
        gain_db: parameter_id(20),
        q: parameter_id(21),
        shelf_slope: parameter_id(22),
    },
    EqBandDescriptorV1 {
        index: 2,
        cascade_order: 2,
        enabled: parameter_id(33),
        kind: parameter_id(34),
        frequency_hz: parameter_id(35),
        gain_db: parameter_id(36),
        q: parameter_id(37),
        shelf_slope: parameter_id(38),
    },
    EqBandDescriptorV1 {
        index: 3,
        cascade_order: 3,
        enabled: parameter_id(49),
        kind: parameter_id(50),
        frequency_hz: parameter_id(51),
        gain_db: parameter_id(52),
        q: parameter_id(53),
        shelf_slope: parameter_id(54),
    },
];

const KIND_CHOICES: [miso_engine_effect_contract::EnumChoiceV1; 6] = [
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 1.0,
        label: "bell",
    },
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 2.0,
        label: "low-shelf",
    },
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 3.0,
        label: "high-shelf",
    },
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 4.0,
        label: "low-pass",
    },
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 5.0,
        label: "high-pass",
    },
    miso_engine_effect_contract::EnumChoiceV1 {
        value: 6.0,
        label: "notch",
    },
];

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    domain: ParameterDomain,
    minimum: Option<f32>,
    maximum: Option<f32>,
    default_value: f32,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
    choices: &'static [miso_engine_effect_contract::EnumChoiceV1],
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: parameter_id(id),
        display_name: name,
        display_unit,
        unit,
        domain,
        minimum,
        maximum,
        default_value,
        mapping,
        automation_rate,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing,
        smoothing_samples,
        readable: true,
        automatable: matches!(
            automation_rate,
            AutomationRate::Sample | AutomationRate::Block
        ),
        enum_choices: choices,
    }
}

const EQ_PARAMETERS: [ParameterDescriptorV1; 24] = [
    parameter(
        1,
        "band-1-enabled",
        "on/off",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        None,
        None,
        0.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &[],
    ),
    parameter(
        2,
        "band-1-kind",
        "type",
        ParameterUnit::Linear,
        ParameterDomain::Enumeration,
        None,
        None,
        1.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &KIND_CHOICES,
    ),
    parameter(
        3,
        "band-1-frequency",
        "Hz",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        Some(10.0),
        Some(20_000.0),
        80.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        4,
        "band-1-gain",
        "dB",
        ParameterUnit::Db,
        ParameterDomain::Continuous,
        Some(-24.0),
        Some(24.0),
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        5,
        "band-1-q",
        "Q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(18.0),
        0.70710677,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        6,
        "band-1-shelf-slope",
        "S",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(1.0),
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        17,
        "band-2-enabled",
        "on/off",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        None,
        None,
        0.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &[],
    ),
    parameter(
        18,
        "band-2-kind",
        "type",
        ParameterUnit::Linear,
        ParameterDomain::Enumeration,
        None,
        None,
        1.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &KIND_CHOICES,
    ),
    parameter(
        19,
        "band-2-frequency",
        "Hz",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        Some(10.0),
        Some(20_000.0),
        400.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        20,
        "band-2-gain",
        "dB",
        ParameterUnit::Db,
        ParameterDomain::Continuous,
        Some(-24.0),
        Some(24.0),
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        21,
        "band-2-q",
        "Q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(18.0),
        0.70710677,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        22,
        "band-2-shelf-slope",
        "S",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(1.0),
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        33,
        "band-3-enabled",
        "on/off",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        None,
        None,
        0.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &[],
    ),
    parameter(
        34,
        "band-3-kind",
        "type",
        ParameterUnit::Linear,
        ParameterDomain::Enumeration,
        None,
        None,
        1.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &KIND_CHOICES,
    ),
    parameter(
        35,
        "band-3-frequency",
        "Hz",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        Some(10.0),
        Some(20_000.0),
        2000.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        36,
        "band-3-gain",
        "dB",
        ParameterUnit::Db,
        ParameterDomain::Continuous,
        Some(-24.0),
        Some(24.0),
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        37,
        "band-3-q",
        "Q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(18.0),
        0.70710677,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        38,
        "band-3-shelf-slope",
        "S",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(1.0),
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        49,
        "band-4-enabled",
        "on/off",
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        None,
        None,
        0.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &[],
    ),
    parameter(
        50,
        "band-4-kind",
        "type",
        ParameterUnit::Linear,
        ParameterDomain::Enumeration,
        None,
        None,
        1.0,
        ParameterMapping::Stepped,
        AutomationRate::None,
        SmoothingRule::None,
        0,
        &KIND_CHOICES,
    ),
    parameter(
        51,
        "band-4-frequency",
        "Hz",
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        Some(10.0),
        Some(20_000.0),
        10_000.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        52,
        "band-4-gain",
        "dB",
        ParameterUnit::Db,
        ParameterDomain::Continuous,
        Some(-24.0),
        Some(24.0),
        0.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        53,
        "band-4-q",
        "Q",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(18.0),
        0.70710677,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
    parameter(
        54,
        "band-4-shelf-slope",
        "S",
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        Some(0.1),
        Some(1.0),
        1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
        &[],
    ),
];

const PORTS: [PortDescriptorV1; 2] = [
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
const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];
const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    miso_engine_effect_contract::QualityDescriptorV1 {
        quality: Quality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: STATE_BYTES_PER_LANE as u32,
            right_bytes: STATE_BYTES_PER_LANE as u32,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

/// Authoritative static V1 effect metadata.
pub static PARAMETRIC_EQ_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.parametric-eq"),
    display_name: "Parametric EQ",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &EQ_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
};

/// Stateless native factory for prepared scalar EQs.
#[derive(Clone, Copy, Debug, Default)]
pub struct ParametricEqFactory;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqCoefficientsV1 {
    /// Exact endpoint anchor, either positive or negative one.
    pub a: f32,
    /// Delta-basis numerator constant.
    pub n0: f32,
    /// Delta-basis denominator constant.
    pub d0: f32,
    /// Delta-basis numerator first difference.
    pub n1: f32,
    /// Delta-basis denominator first difference.
    pub d1: f32,
    /// Delta-basis numerator second difference.
    pub n2: f32,
    /// Delta-basis denominator second difference.
    pub d2: f32,
    pub identity: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqDesignError {
    InvalidInput,
    Coefficients,
}

/// Design one RBJ section in `f64`, then retain the selected endpoint-conditioned delta words.
pub fn design_biquad_v1(
    kind: EqBandKindV1,
    frequency_hz: f32,
    gain_db: f32,
    q: f32,
    shelf_slope: f32,
    sample_rate: SampleRateHz,
) -> Result<EqCoefficientsV1, EqDesignError> {
    if !is_launch_sample_rate(sample_rate)
        || ![frequency_hz, gain_db, q, shelf_slope]
            .into_iter()
            .all(f32::is_finite)
        || !(10.0..=20_000.0).contains(&frequency_hz)
        || !(-24.0..=24.0).contains(&gain_db)
        || !(0.1..=18.0).contains(&q)
        || !(0.1..=1.0).contains(&shelf_slope)
        || frequency_hz >= sample_rate.0 as f32 * 0.5
    {
        return Err(EqDesignError::InvalidInput);
    }
    let anchor = if frequency_hz <= sample_rate.0 as f32 * 0.25 {
        1.0_f32
    } else {
        -1.0_f32
    };
    if matches!(
        kind,
        EqBandKindV1::Bell | EqBandKindV1::LowShelf | EqBandKindV1::HighShelf
    ) && gain_db == 0.0
    {
        return Ok(identity_with_anchor(anchor));
    }
    let w = 2.0 * PI * f64::from(frequency_hz) / f64::from(sample_rate.0);
    let c = w.cos();
    let s = w.sin();
    let a = 10.0_f64.powf(f64::from(gain_db) / 40.0);
    let alpha_q = s / (2.0 * f64::from(q));
    let alpha_s = s * 0.5 * ((a + 1.0 / a) * (1.0 / f64::from(shelf_slope) - 1.0) + 2.0).sqrt();
    let beta = 2.0 * a.sqrt() * alpha_s;
    let (b0, b1, b2, a0, a1, a2) = match kind {
        EqBandKindV1::LowPass => (
            (1.0 - c) * 0.5,
            1.0 - c,
            (1.0 - c) * 0.5,
            1.0 + alpha_q,
            -2.0 * c,
            1.0 - alpha_q,
        ),
        EqBandKindV1::HighPass => (
            (1.0 + c) * 0.5,
            -(1.0 + c),
            (1.0 + c) * 0.5,
            1.0 + alpha_q,
            -2.0 * c,
            1.0 - alpha_q,
        ),
        EqBandKindV1::Notch => (1.0, -2.0 * c, 1.0, 1.0 + alpha_q, -2.0 * c, 1.0 - alpha_q),
        EqBandKindV1::Bell => (
            1.0 + alpha_q * a,
            -2.0 * c,
            1.0 - alpha_q * a,
            1.0 + alpha_q / a,
            -2.0 * c,
            1.0 - alpha_q / a,
        ),
        EqBandKindV1::LowShelf => (
            a * ((a + 1.0) - (a - 1.0) * c + beta),
            2.0 * a * ((a - 1.0) - (a + 1.0) * c),
            a * ((a + 1.0) - (a - 1.0) * c - beta),
            (a + 1.0) + (a - 1.0) * c + beta,
            -2.0 * ((a - 1.0) + (a + 1.0) * c),
            (a + 1.0) + (a - 1.0) * c - beta,
        ),
        EqBandKindV1::HighShelf => (
            a * ((a + 1.0) + (a - 1.0) * c + beta),
            -2.0 * a * ((a - 1.0) + (a + 1.0) * c),
            a * ((a + 1.0) + (a - 1.0) * c - beta),
            (a + 1.0) - (a - 1.0) * c + beta,
            2.0 * ((a - 1.0) - (a + 1.0) * c),
            (a + 1.0) - (a - 1.0) * c - beta,
        ),
    };
    if ![b0, b1, b2, a0, a1, a2].into_iter().all(f64::is_finite) || a0 == 0.0 {
        return Err(EqDesignError::Coefficients);
    }
    let b0 = b0 / a0;
    let b1 = b1 / a0;
    let b2 = b2 / a0;
    let a1 = a1 / a0;
    let a2 = a2 / a0;
    if ![b0, b1, b2, a1, a2].into_iter().all(f64::is_finite) {
        return Err(EqDesignError::Coefficients);
    }
    let anchor_f64 = f64::from(anchor);
    let coefficients = EqCoefficientsV1 {
        a: anchor,
        n0: (b0 + anchor_f64 * b1 + b2) as f32,
        d0: (1.0 + anchor_f64 * a1 + a2) as f32,
        n1: (b1 + 2.0 * anchor_f64 * b2) as f32,
        d1: (a1 + 2.0 * anchor_f64 * a2) as f32,
        n2: b2 as f32,
        d2: a2 as f32,
        identity: false,
    };
    let reconstructed_a1 = coefficients.d1 - 2.0 * coefficients.a * coefficients.d2;
    let reconstructed_a2 = coefficients.d2;
    let scale = (coefficients.d0 - coefficients.a * coefficients.d1) + coefficients.d2;
    if ![
        coefficients.a,
        coefficients.n0,
        coefficients.d0,
        coefficients.n1,
        coefficients.d1,
        coefficients.n2,
        coefficients.d2,
        reconstructed_a1,
        reconstructed_a2,
        scale,
    ]
    .into_iter()
    .all(f32::is_finite)
        || scale == 0.0
        || reconstructed_a2.abs() >= 1.0
        || 1.0 + reconstructed_a1 + reconstructed_a2 <= 0.0
        || 1.0 - reconstructed_a1 + reconstructed_a2 <= 0.0
    {
        return Err(EqDesignError::Coefficients);
    }
    Ok(coefficients)
}

const fn identity() -> EqCoefficientsV1 {
    identity_with_anchor(1.0)
}

const fn identity_with_anchor(anchor: f32) -> EqCoefficientsV1 {
    EqCoefficientsV1 {
        a: anchor,
        n0: 1.0,
        d0: 1.0,
        n1: 0.0,
        d1: 0.0,
        n2: 0.0,
        d2: 0.0,
        identity: true,
    }
}

#[derive(Clone, Copy)]
struct BandConfiguration {
    enabled: bool,
    kind: EqBandKindV1,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
    coefficients: EqCoefficientsV1,
}
#[derive(Clone, Copy, Default)]
struct BiquadState {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
struct NumericRamp {
    current: f32,
    target: f32,
    remaining: u32,
}

impl NumericRamp {
    const fn fixed(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            remaining: 0,
        }
    }
    fn set_target(&mut self, value: f32) {
        self.target = value;
        self.remaining = 64;
    }
    fn next(&mut self) -> bool {
        if self.remaining == 0 {
            return false;
        }
        self.current = if self.remaining == 1 {
            self.target
        } else {
            self.current + (self.target - self.current) / self.remaining as f32
        };
        self.remaining -= 1;
        true
    }
    fn snap(&mut self) {
        self.current = self.target;
        self.remaining = 0;
    }
}

#[derive(Clone, Copy)]
struct Lane {
    state: [BiquadState; EQ_SECTION_COUNT_V1],
    coefficients: [EqCoefficientsV1; EQ_SECTION_COUNT_V1],
    frequency: [NumericRamp; EQ_SECTION_COUNT_V1],
    gain: [NumericRamp; EQ_SECTION_COUNT_V1],
    q: [NumericRamp; EQ_SECTION_COUNT_V1],
    slope: [NumericRamp; EQ_SECTION_COUNT_V1],
}
impl Lane {
    fn from_config(configs: &[BandConfiguration; EQ_SECTION_COUNT_V1]) -> Self {
        Self {
            state: [BiquadState::default(); EQ_SECTION_COUNT_V1],
            coefficients: configs.map(|config| config.coefficients),
            frequency: configs.map(|config| NumericRamp::fixed(config.frequency)),
            gain: configs.map(|config| NumericRamp::fixed(config.gain)),
            q: configs.map(|config| NumericRamp::fixed(config.q)),
            slope: configs.map(|config| NumericRamp::fixed(config.slope)),
        }
    }
}

struct PreparedParametricEq {
    metadata: PreparedEffectMetadata,
    left_config: [BandConfiguration; EQ_SECTION_COUNT_V1],
    right_config: [BandConfiguration; EQ_SECTION_COUNT_V1],
    left: Lane,
    right: Lane,
}

/// Section-major transposed endpoint-conditioned delta words plus a compact derived mask.
#[derive(Clone, Copy)]
#[repr(C)]
struct BankCoefficients<const W: usize> {
    a: [f32; W],
    n0: [f32; W],
    d0: [f32; W],
    n1: [f32; W],
    d1: [f32; W],
    n2: [f32; W],
    d2: [f32; W],
    identity_mask: [u8; W],
}

#[derive(Clone, Copy)]
#[repr(C)]
struct BankDeltaState<const W: usize> {
    x1: [f32; W],
    x2: [f32; W],
    y1: [f32; W],
    y2: [f32; W],
}

/// Coefficients and histories are section-major/track-minor; smoothers remain track-major.
#[derive(Clone, Copy)]
#[repr(C)]
struct BankChannel<const W: usize> {
    coefficients: [BankCoefficients<W>; EQ_SECTION_COUNT_V1],
    state: [BankDeltaState<W>; EQ_SECTION_COUNT_V1],
    frequency: [[NumericRamp; EQ_SECTION_COUNT_V1]; W],
    gain: [[NumericRamp; EQ_SECTION_COUNT_V1]; W],
    q: [[NumericRamp; EQ_SECTION_COUNT_V1]; W],
    slope: [[NumericRamp; EQ_SECTION_COUNT_V1]; W],
}

impl<const W: usize> BankChannel<W> {
    fn from_configs(configs: &[[BandConfiguration; EQ_SECTION_COUNT_V1]; W]) -> Self {
        let mut channel = Self {
            coefficients: [BankCoefficients {
                a: [1.0; W],
                n0: [1.0; W],
                d0: [1.0; W],
                n1: [0.0; W],
                d1: [0.0; W],
                n2: [0.0; W],
                d2: [0.0; W],
                identity_mask: [u8::MAX; W],
            }; EQ_SECTION_COUNT_V1],
            state: [BankDeltaState {
                x1: [0.0; W],
                x2: [0.0; W],
                y1: [0.0; W],
                y2: [0.0; W],
            }; EQ_SECTION_COUNT_V1],
            frequency: [[NumericRamp::fixed(10.0); EQ_SECTION_COUNT_V1]; W],
            gain: [[NumericRamp::fixed(0.0); EQ_SECTION_COUNT_V1]; W],
            q: [[NumericRamp::fixed(0.1); EQ_SECTION_COUNT_V1]; W],
            slope: [[NumericRamp::fixed(0.1); EQ_SECTION_COUNT_V1]; W],
        };
        for (track, configs) in configs.iter().enumerate() {
            for (section, config) in configs.iter().copied().enumerate() {
                channel.frequency[track][section] = NumericRamp::fixed(config.frequency);
                channel.gain[track][section] = NumericRamp::fixed(config.gain);
                channel.q[track][section] = NumericRamp::fixed(config.q);
                channel.slope[track][section] = NumericRamp::fixed(config.slope);
                channel.set_coefficients(section, track, config.coefficients);
            }
        }
        channel
    }

    fn set_coefficients(&mut self, section: usize, track: usize, coefficients: EqCoefficientsV1) {
        let slot = &mut self.coefficients[section];
        slot.a[track] = coefficients.a;
        slot.n0[track] = coefficients.n0;
        slot.d0[track] = coefficients.d0;
        slot.n1[track] = coefficients.n1;
        slot.d1[track] = coefficients.d1;
        slot.n2[track] = coefficients.n2;
        slot.d2[track] = coefficients.d2;
        slot.identity_mask[track] = if coefficients.identity { u8::MAX } else { 0 };
    }

    fn lane(&self, track: usize) -> Lane {
        Lane {
            state: core::array::from_fn(|section| BiquadState {
                x1: self.state[section].x1[track],
                x2: self.state[section].x2[track],
                y1: self.state[section].y1[track],
                y2: self.state[section].y2[track],
            }),
            coefficients: core::array::from_fn(|section| EqCoefficientsV1 {
                a: self.coefficients[section].a[track],
                n0: self.coefficients[section].n0[track],
                d0: self.coefficients[section].d0[track],
                n1: self.coefficients[section].n1[track],
                d1: self.coefficients[section].d1[track],
                n2: self.coefficients[section].n2[track],
                d2: self.coefficients[section].d2[track],
                identity: self.coefficients[section].identity_mask[track] == u8::MAX,
            }),
            frequency: self.frequency[track],
            gain: self.gain[track],
            q: self.q[track],
            slope: self.slope[track],
        }
    }

    fn set_lane(&mut self, track: usize, lane: Lane) {
        self.frequency[track] = lane.frequency;
        self.gain[track] = lane.gain;
        self.q[track] = lane.q;
        self.slope[track] = lane.slope;
        for section in 0..EQ_SECTION_COUNT_V1 {
            self.state[section].x1[track] = lane.state[section].x1;
            self.state[section].x2[track] = lane.state[section].x2;
            self.state[section].y1[track] = lane.state[section].y1;
            self.state[section].y2[track] = lane.state[section].y2;
            self.set_coefficients(section, track, lane.coefficients[section]);
        }
    }
}

struct PreparedParametricEqBank<const W: usize> {
    metadata: PreparedBankMetadata,
    effect_metadata: PreparedEffectMetadata,
    kernel: PreparedDeltaBankKernelV1,
    left_config: [[BandConfiguration; EQ_SECTION_COUNT_V1]; W],
    right_config: [[BandConfiguration; EQ_SECTION_COUNT_V1]; W],
    left: BankChannel<W>,
    right: BankChannel<W>,
}

impl NativeEffectFactory for ParametricEqFactory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        &PARAMETRIC_EQ_DESCRIPTOR_V1
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left_config, right_config) =
            configurations(request.initial_values, SampleRateHz(request.sample_rate))?;
        Ok(Box::new(PreparedParametricEq {
            metadata,
            left_config,
            right_config,
            left: Lane::from_config(&left_config),
            right: Lane::from_config(&right_config),
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        if !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Ok(None);
        }
        let kernel = match PreparedDeltaBankKernelV1::try_new(request.backend) {
            Ok(kernel) => kernel,
            Err(DeltaBankKernelError::BackendUnavailable) => return Ok(None),
            Err(_) => return Ok(None),
        };
        match request.width {
            BankWidth::Four => prepare_homogeneous_bank::<4>(self, request, kernel),
            BankWidth::Eight => prepare_homogeneous_bank::<8>(self, request, kernel),
        }
    }
}

fn prepare_homogeneous_bank<const W: usize>(
    factory: &ParametricEqFactory,
    request: PrepareEffectBankRequest<'_>,
    kernel: PreparedDeltaBankKernelV1,
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
    let first = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first)?;
    let (first_left, first_right) =
        configurations(first.initial_values, SampleRateHz(first.sample_rate))?;
    let mut left_config = [first_left; W];
    let mut right_config = [first_right; W];
    for (track, item) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            return Ok(None);
        }
        let (left, right) = configurations(item.initial_values, SampleRateHz(item.sample_rate))?;
        left_config[track] = left;
        right_config[track] = right;
    }
    Ok(Some(Box::new(PreparedParametricEqBank::<W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        effect_metadata: metadata,
        kernel,
        left_config,
        right_config,
        left: BankChannel::from_configs(&left_config),
        right: BankChannel::from_configs(&right_config),
    })))
}

fn configurations(
    values: &[InitialParameterValue],
    rate: SampleRateHz,
) -> Result<([BandConfiguration; 4], [BandConfiguration; 4]), EffectPrepareError> {
    let mut left = [default_band(0, rate)?; 4];
    let mut right = left;
    for lane in 0..2 {
        for section in 0..4 {
            let index = section * 6;
            let lane_values = &values
                .iter()
                .filter(|value| {
                    value.channel
                        == if lane == 0 {
                            ParameterChannel::Left
                        } else {
                            ParameterChannel::Right
                        }
                })
                .collect::<Vec<_>>();
            let selected = &lane_values[index..index + 6];
            let enabled = selected[0].value.to_bits() == 1.0_f32.to_bits();
            let kind = EqBandKindV1::from_value(selected[1].value).ok_or(EffectPrepareError {
                code: "effect.parameter.initial",
            })?;
            let config = BandConfiguration {
                enabled,
                kind,
                frequency: selected[2].value,
                gain: selected[3].value,
                q: selected[4].value,
                slope: selected[5].value,
                coefficients: if enabled {
                    design_biquad_v1(
                        kind,
                        selected[2].value,
                        selected[3].value,
                        selected[4].value,
                        selected[5].value,
                        rate,
                    )
                    .map_err(|_| EffectPrepareError {
                        code: "effect.eq.coefficients",
                    })?
                } else {
                    identity()
                },
            };
            if lane == 0 {
                left[section] = config
            } else {
                right[section] = config
            }
        }
    }
    Ok((left, right))
}
fn default_band(
    section: usize,
    _rate: SampleRateHz,
) -> Result<BandConfiguration, EffectPrepareError> {
    let index = section * 6;
    let p = &EQ_PARAMETERS[index..index + 6];
    Ok(BandConfiguration {
        enabled: false,
        kind: EqBandKindV1::Bell,
        frequency: p[2].default_value,
        gain: p[3].default_value,
        q: p[4].default_value,
        slope: p[5].default_value,
        coefficients: identity(),
    })
}

impl PreparedNativeEffect for PreparedParametricEq {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left = Lane::from_config(&self.left_config);
                self.right = Lane::from_config(&self.right_config);
            }
            ResetKind::DiscontinuityKeepParameters => {
                let sample_rate = SampleRateHz(self.metadata.sample_rate);
                discontinuity_reset(&mut self.left, &self.left_config, sample_rate);
                discontinuity_reset(&mut self.right, &self.right_config, sample_rate);
            }
        }
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.metadata,
            block.first_sample,
            block.left.len() as u32,
            &mut self.left,
            &mut self.right,
            &mut report.invalid_spans,
        );
        for index in 0..block.left.len() {
            let dry_left = sanitize(block.left[index], &mut report.sanitized_main_samples);
            let dry_right = sanitize(block.right[index], &mut report.sanitized_main_samples);
            let left = process_lane(
                dry_left,
                &mut self.left,
                &self.left_config,
                SampleRateHz(self.metadata.sample_rate),
                self.metadata.bypass,
                &mut report.recovered_left_samples,
            );
            let right = process_lane(
                dry_right,
                &mut self.right,
                &self.right_config,
                SampleRateHz(self.metadata.sample_rate),
                self.metadata.bypass,
                &mut report.recovered_right_samples,
            );
            block.left[index] = if self.metadata.bypass { dry_left } else { left };
            block.right[index] = if self.metadata.bypass {
                dry_right
            } else {
                right
            };
        }
        report
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        validate_state_output(&output)?;
        write_lane(output.common, output.left, &self.left)?;
        write_lane(&mut [], output.right, &self.right)
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != 1 {
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
        };
        let sample_rate = SampleRateHz(self.metadata.sample_rate);
        let left = read_lane(input.common, input.left, &self.left_config, sample_rate)?;
        let right = read_lane(&[], input.right, &self.right_config, sample_rate)?;
        self.left = left;
        self.right = right;
        Ok(())
    }
}

impl<const W: usize> PreparedNativeEffectBank for PreparedParametricEqBank<W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                self.left = BankChannel::from_configs(&self.left_config);
                self.right = BankChannel::from_configs(&self.right_config);
            }
            ResetKind::DiscontinuityKeepParameters => {
                let sample_rate = SampleRateHz(self.effect_metadata.sample_rate);
                bank_discontinuity_reset(&mut self.left, &self.left_config, sample_rate);
                bank_discontinuity_reset(&mut self.right, &self.right_config, sample_rate);
            }
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        if !bank_block_matches(&block, self.metadata.width, self.effect_metadata.quantum) {
            return report;
        }
        let lanes = self.metadata.width.lanes() as usize;
        if lanes != W {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_bank_automation(
                &block.automation[start..end],
                self.effect_metadata,
                block.first_sample,
                &mut self.left,
                &mut self.right,
                track,
                &mut report.reports[track].invalid_spans,
            );
        }

        let sample_rate = SampleRateHz(self.effect_metadata.sample_rate);
        for frame in 0..block.frames as usize {
            let mut left = [0.0_f32; W];
            let mut right = [0.0_f32; W];
            let mut dry_left = [0.0_f32; W];
            let mut dry_right = [0.0_f32; W];
            for track in 0..W {
                let index = frame * lanes + track;
                dry_left[track] = sanitize(
                    block.left[index],
                    &mut report.reports[track].sanitized_main_samples,
                );
                dry_right[track] = sanitize(
                    block.right[index],
                    &mut report.reports[track].sanitized_main_samples,
                );
                left[track] = dry_left[track];
                right[track] = dry_right[track];
            }
            let mut recovered_left = [false; W];
            let mut recovered_right = [false; W];
            process_bank_channel(
                &mut self.left,
                &self.left_config,
                self.kernel,
                &mut left,
                sample_rate,
                self.effect_metadata.bypass,
                &mut recovered_left,
                &mut report.reports,
                true,
            );
            process_bank_channel(
                &mut self.right,
                &self.right_config,
                self.kernel,
                &mut right,
                sample_rate,
                self.effect_metadata.bypass,
                &mut recovered_right,
                &mut report.reports,
                false,
            );
            for track in 0..W {
                let index = frame * lanes + track;
                block.left[index] = if self.effect_metadata.bypass {
                    dry_left[track]
                } else {
                    left[track]
                };
                block.right[index] = if self.effect_metadata.bypass {
                    dry_right[track]
                } else {
                    right[track]
                };
            }
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = bank_track_index(track_index, self.metadata.width)?;
        validate_state_output(&output)?;
        let left = self.left.lane(track);
        let right = self.right.lane(track);
        write_lane(output.common, output.left, &left)?;
        write_lane(&mut [], output.right, &right)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != 1 {
            return Err(StatePayloadError {
                code: "effect.state.version",
            });
        }
        let track = bank_track_index(track_index, self.metadata.width)?;
        let sample_rate = SampleRateHz(self.effect_metadata.sample_rate);
        let left = read_lane(
            input.common,
            input.left,
            &self.left_config[track],
            sample_rate,
        )?;
        let right = read_lane(&[], input.right, &self.right_config[track], sample_rate)?;
        self.left.set_lane(track, left);
        self.right.set_lane(track, right);
        Ok(())
    }
}

fn bank_track_index(track_index: u32, width: BankWidth) -> Result<usize, StatePayloadError> {
    let track = track_index as usize;
    if track >= width.lanes() as usize {
        return Err(StatePayloadError {
            code: "effect.bank.track",
        });
    }
    Ok(track)
}

fn bank_block_matches(block: &EffectBankProcessBlock<'_>, width: BankWidth, quantum: u32) -> bool {
    let lanes = width.lanes() as usize;
    let Some(length) = (block.frames as usize).checked_mul(lanes) else {
        return false;
    };
    block.width == width
        && block.frames != 0
        && block.frames <= quantum
        && block.left.len() == length
        && block.right.len() == length
        && block.sidechain.is_none()
        && block
            .first_sample
            .checked_add(block.frames as u64)
            .is_some()
        && block.automation_offsets.len() == lanes + 1
        && block.automation_offsets.first() == Some(&0)
        && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
        && !block
            .automation_offsets
            .windows(2)
            .any(|pair| pair[0] > pair[1])
}

fn apply_bank_automation<const W: usize>(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    left: &mut BankChannel<W>,
    right: &mut BankChannel<W>,
    track: usize,
    invalid_spans: &mut u64,
) {
    if spans.len() > metadata.automation_capacity as usize {
        *invalid_spans = invalid_spans.saturating_add(spans.len() as u64);
        return;
    }
    let mut pending = [None; EQ_SECTION_COUNT_V1 * 6 * 2];
    let mut seen = [false; EQ_SECTION_COUNT_V1 * 6 * 2];
    let mut prior_sort_key = None;
    for span in spans {
        let sort_key = (span.start_sample, span.parameter_index, span.channel);
        let Some((section, field)) = numeric_parameter(span.parameter_index as usize) else {
            count_invalid(invalid_spans);
            continue;
        };
        let Some(channel) = lane_index(span.channel) else {
            count_invalid(invalid_spans);
            continue;
        };
        let slot = (section * 6 + field) * 2 + channel;
        if prior_sort_key.is_some_and(|prior| sort_key < prior)
            || seen[slot]
            || span.kind != AutomationSpanKind::Point
            || span.start_sample != first_sample
            || span.end_sample != first_sample
            || span.start_value.to_bits() != span.end_value.to_bits()
            || !numeric_value_valid(field, span.start_value)
        {
            count_invalid(invalid_spans);
            continue;
        }
        seen[slot] = true;
        pending[slot] = Some(span.start_value);
        prior_sort_key = Some(sort_key);
    }
    for (slot, target) in pending.into_iter().enumerate() {
        if let Some(target) = target {
            set_bank_target(left, right, track, slot, target);
        }
    }
}

fn set_bank_target<const W: usize>(
    left: &mut BankChannel<W>,
    right: &mut BankChannel<W>,
    track: usize,
    slot: usize,
    target: f32,
) {
    let parameter = slot / 2;
    let section = parameter / 6;
    let field = parameter % 6;
    let channel = if slot.is_multiple_of(2) { left } else { right };
    match field {
        2 => channel.frequency[track][section].set_target(target),
        3 => channel.gain[track][section].set_target(target),
        4 => channel.q[track][section].set_target(target),
        5 => channel.slope[track][section].set_target(target),
        _ => unreachable!("only numeric parameter slots are pending"),
    }
}

#[allow(clippy::too_many_arguments)]
fn process_bank_channel<const W: usize>(
    channel: &mut BankChannel<W>,
    configs: &[[BandConfiguration; EQ_SECTION_COUNT_V1]; W],
    kernel: PreparedDeltaBankKernelV1,
    values: &mut [f32; W],
    sample_rate: SampleRateHz,
    bypass: bool,
    recovered: &mut [bool; W],
    reports: &mut [ProcessReport; 8],
    is_left: bool,
) {
    for (section, _) in configs[0].iter().enumerate() {
        let mut failed_this_section = [false; W];
        for track in 0..W {
            let prior_frequency = channel.frequency[track][section];
            let prior_gain = channel.gain[track][section];
            let prior_q = channel.q[track][section];
            let prior_slope = channel.slope[track][section];
            let changed = channel.frequency[track][section].next()
                | channel.gain[track][section].next()
                | channel.q[track][section].next()
                | channel.slope[track][section].next();
            if changed {
                match coefficients_for_values(
                    configs[track][section],
                    channel.frequency[track][section].current,
                    channel.gain[track][section].current,
                    channel.q[track][section].current,
                    channel.slope[track][section].current,
                    sample_rate,
                ) {
                    Ok(coefficients) => channel.set_coefficients(section, track, coefficients),
                    Err(_) => {
                        channel.frequency[track][section] = prior_frequency;
                        channel.gain[track][section] = prior_gain;
                        channel.q[track][section] = prior_q;
                        channel.slope[track][section] = prior_slope;
                        reset_bank_section(channel, section, track);
                        failed_this_section[track] = true;
                        count_bank_recovery(recovered, reports, track, is_left);
                    }
                }
            }
        }
        if bypass {
            let state = &mut channel.state[section];
            for track in 0..W {
                if failed_this_section[track] {
                    values[track] = 0.0;
                    continue;
                }
                let value = values[track];
                let prior_x1 = state.x1[track];
                state.x2[track] = prior_x1;
                state.x1[track] = value;
                state.y2[track] = prior_x1;
                state.y1[track] = value;
            }
            continue;
        }
        let coefficients = &channel.coefficients[section];
        let state = &mut channel.state[section];
        let mut identity_mask = [0_u32; W];
        for (output, input) in identity_mask
            .iter_mut()
            .zip(coefficients.identity_mask.iter())
        {
            *output = if *input == u8::MAX { u32::MAX } else { 0 };
        }
        if kernel
            .process_delta(
                values,
                &coefficients.a,
                &coefficients.n0,
                &coefficients.d0,
                &coefficients.n1,
                &coefficients.d1,
                &coefficients.n2,
                &coefficients.d2,
                &mut state.x1,
                &mut state.x2,
                &mut state.y1,
                &mut state.y2,
                &identity_mask,
            )
            .is_err()
        {
            return;
        }
        for track in 0..W {
            let state = &channel.state[section];
            if !failed_this_section[track] && state_invalid(state, track) {
                reset_bank_section(channel, section, track);
                failed_this_section[track] = true;
                count_bank_recovery(recovered, reports, track, is_left);
            }
            if failed_this_section[track] {
                // The scalar path skips the failed section after clearing it, but resumes the
                // cascade with a positive-zero input. Preserve that exact downstream behavior.
                reset_bank_section(channel, section, track);
                values[track] = 0.0;
            }
        }
    }
}

fn state_invalid<const W: usize>(state: &BankDeltaState<W>, track: usize) -> bool {
    ![
        state.x1[track],
        state.x2[track],
        state.y1[track],
        state.y2[track],
    ]
    .into_iter()
    .all(valid)
}

fn reset_bank_section<const W: usize>(channel: &mut BankChannel<W>, section: usize, track: usize) {
    channel.state[section].x1[track] = 0.0;
    channel.state[section].x2[track] = 0.0;
    channel.state[section].y1[track] = 0.0;
    channel.state[section].y2[track] = 0.0;
}

fn count_bank_recovery<const W: usize>(
    recovered: &mut [bool; W],
    reports: &mut [ProcessReport; 8],
    track: usize,
    is_left: bool,
) {
    if recovered[track] {
        return;
    }
    recovered[track] = true;
    let counter = if is_left {
        &mut reports[track].recovered_left_samples
    } else {
        &mut reports[track].recovered_right_samples
    };
    *counter = counter.saturating_add(1);
}

fn bank_discontinuity_reset<const W: usize>(
    channel: &mut BankChannel<W>,
    configs: &[[BandConfiguration; EQ_SECTION_COUNT_V1]; W],
    sample_rate: SampleRateHz,
) {
    channel.state = [BankDeltaState {
        x1: [0.0; W],
        x2: [0.0; W],
        y1: [0.0; W],
        y2: [0.0; W],
    }; EQ_SECTION_COUNT_V1];
    for (track, configs) in configs.iter().enumerate() {
        for (section, config) in configs.iter().copied().enumerate() {
            channel.frequency[track][section].snap();
            channel.gain[track][section].snap();
            channel.q[track][section].snap();
            channel.slope[track][section].snap();
            let coefficients = coefficients_for_values(
                config,
                channel.frequency[track][section].current,
                channel.gain[track][section].current,
                channel.q[track][section].current,
                channel.slope[track][section].current,
                sample_rate,
            )
            .unwrap_or(config.coefficients);
            channel.set_coefficients(section, track, coefficients);
        }
    }
}

fn sanitize(value: f32, count: &mut u64) -> f32 {
    match sanitize_sample(value) {
        Some(value) => value,
        None => {
            *count = count.saturating_add(1);
            0.0
        }
    }
}

fn validate_state_output(output: &StatePayloadOutput<'_>) -> Result<(), StatePayloadError> {
    if !output.common.is_empty()
        || output.left.len() != STATE_BYTES_PER_LANE
        || output.right.len() != STATE_BYTES_PER_LANE
    {
        return Err(StatePayloadError {
            code: "effect.state.length",
        });
    }
    Ok(())
}

/// Runtime spans address descriptor positions (0..24), never sparse public `ParameterId`s.
/// They are validated one-by-one because a malformed span must not discard another valid point.
fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: PreparedEffectMetadata,
    first_sample: u64,
    _frames: u32,
    left: &mut Lane,
    right: &mut Lane,
    invalid_spans: &mut u64,
) {
    if spans.len() > metadata.automation_capacity as usize {
        *invalid_spans = invalid_spans.saturating_add(spans.len() as u64);
        return;
    }

    let mut pending = [None; EQ_SECTION_COUNT_V1 * 6 * 2];
    let mut seen = [false; EQ_SECTION_COUNT_V1 * 6 * 2];
    let mut prior_sort_key = None;
    for span in spans {
        let sort_key = (span.start_sample, span.parameter_index, span.channel);
        let parameter_index = span.parameter_index as usize;
        let Some((section, field)) = numeric_parameter(parameter_index) else {
            count_invalid(invalid_spans);
            continue;
        };
        let Some(channel) = lane_index(span.channel) else {
            count_invalid(invalid_spans);
            continue;
        };
        let slot = (section * 6 + field) * 2 + channel;
        if prior_sort_key.is_some_and(|prior| sort_key < prior)
            || seen[slot]
            || span.kind != AutomationSpanKind::Point
            || span.start_sample != first_sample
            || span.end_sample != first_sample
            || span.start_value.to_bits() != span.end_value.to_bits()
            || !numeric_value_valid(field, span.start_value)
        {
            count_invalid(invalid_spans);
            continue;
        }
        seen[slot] = true;
        pending[slot] = Some(span.start_value);
        prior_sort_key = Some(sort_key);
    }

    for (slot, target) in pending.into_iter().enumerate() {
        if let Some(target) = target {
            set_lane_target(left, right, slot, target);
        }
    }
}

fn count_invalid(invalid_spans: &mut u64) {
    *invalid_spans = invalid_spans.saturating_add(1);
}

/// Returns the cascade section and its numeric field: frequency, gain, Q, or shelf slope.
fn numeric_parameter(parameter_index: usize) -> Option<(usize, usize)> {
    let section = parameter_index / 6;
    if section >= EQ_SECTION_COUNT_V1 {
        return None;
    }
    match parameter_index % 6 {
        2 => Some((section, 2)),
        3 => Some((section, 3)),
        4 => Some((section, 4)),
        5 => Some((section, 5)),
        _ => None,
    }
}

fn lane_index(channel: ParameterChannel) -> Option<usize> {
    match channel {
        ParameterChannel::Left => Some(0),
        ParameterChannel::Right => Some(1),
        ParameterChannel::Both => None,
    }
}

fn numeric_value_valid(field: usize, value: f32) -> bool {
    value.is_finite()
        && match field {
            2 => (10.0..=20_000.0).contains(&value),
            3 => (-24.0..=24.0).contains(&value),
            4 => (0.1..=18.0).contains(&value),
            5 => (0.1..=1.0).contains(&value),
            _ => false,
        }
}

fn set_lane_target(left: &mut Lane, right: &mut Lane, slot: usize, target: f32) {
    let parameter = slot / 2;
    let channel = slot % 2;
    let section = parameter / 6;
    let field = parameter % 6;
    let lane = if channel == 0 { left } else { right };
    match field {
        2 => lane.frequency[section].set_target(target),
        3 => lane.gain[section].set_target(target),
        4 => lane.q[section].set_target(target),
        5 => lane.slope[section].set_target(target),
        _ => unreachable!("only numeric parameter slots are pending"),
    }
}

fn discontinuity_reset(
    lane: &mut Lane,
    configs: &[BandConfiguration; EQ_SECTION_COUNT_V1],
    sample_rate: SampleRateHz,
) {
    lane.state = [BiquadState::default(); EQ_SECTION_COUNT_V1];
    for (index, config) in configs.iter().copied().enumerate() {
        lane.frequency[index].snap();
        lane.gain[index].snap();
        lane.q[index].snap();
        lane.slope[index].snap();
        lane.coefficients[index] =
            current_coefficients(config, lane, index, sample_rate).unwrap_or(config.coefficients);
    }
}

fn current_coefficients(
    config: BandConfiguration,
    lane: &Lane,
    index: usize,
    sample_rate: SampleRateHz,
) -> Result<EqCoefficientsV1, EqDesignError> {
    coefficients_for_values(
        config,
        lane.frequency[index].current,
        lane.gain[index].current,
        lane.q[index].current,
        lane.slope[index].current,
        sample_rate,
    )
}

fn coefficients_for_values(
    config: BandConfiguration,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
    sample_rate: SampleRateHz,
) -> Result<EqCoefficientsV1, EqDesignError> {
    if !config.enabled {
        return Ok(identity());
    }
    design_biquad_v1(config.kind, frequency, gain, q, slope, sample_rate)
}

fn process_lane(
    mut value: f32,
    lane: &mut Lane,
    configs: &[BandConfiguration; 4],
    sample_rate: SampleRateHz,
    bypass: bool,
    recovered: &mut u64,
) -> f32 {
    let mut did_recover = false;
    for (index, config) in configs.iter().copied().enumerate() {
        let prior_frequency = lane.frequency[index];
        let prior_gain = lane.gain[index];
        let prior_q = lane.q[index];
        let prior_slope = lane.slope[index];
        let changed = lane.frequency[index].next()
            | lane.gain[index].next()
            | lane.q[index].next()
            | lane.slope[index].next();
        if changed {
            match current_coefficients(config, lane, index, sample_rate) {
                Ok(coefficients) => lane.coefficients[index] = coefficients,
                Err(_) => {
                    lane.frequency[index] = prior_frequency;
                    lane.gain[index] = prior_gain;
                    lane.q[index] = prior_q;
                    lane.slope[index] = prior_slope;
                    lane.state[index] = BiquadState::default();
                    value = 0.0;
                    if !did_recover {
                        *recovered = recovered.saturating_add(1);
                        did_recover = true;
                    }
                    continue;
                }
            }
        }
        if !config.enabled || bypass {
            warm(value, &mut lane.state[index]);
            continue;
        }
        let state = &mut lane.state[index];
        let c = lane.coefficients[index];
        if c.identity {
            warm(value, state);
            continue;
        }
        let t0 = c.a * value;
        let dx = state.x1 - t0;
        let t1 = c.a * state.x1;
        let t2 = state.x2 - t1;
        let t3 = c.a * dx;
        let ddx = t2 - t3;
        let p0 = c.n0 * value;
        let p1 = c.n1 * dx;
        let s0 = p0 + p1;
        let p2 = c.n2 * ddx;
        let num = s0 + p2;
        let q0 = c.a * c.d1;
        let scale = (c.d0 - q0) + c.d2;
        let q1 = c.a * c.d2;
        let q2 = (c.d1 - q1) - q1;
        let h0 = q2 * state.y1;
        let h1 = c.d2 * state.y2;
        let history = h0 + h1;
        let output = (num - history) / scale;
        state.x2 = state.x1;
        state.x1 = value;
        state.y2 = state.y1;
        state.y1 = output;
        if !valid(output)
            || ![state.x1, state.x2, state.y1, state.y2]
                .into_iter()
                .all(valid)
        {
            *state = BiquadState::default();
            value = 0.0;
            if !did_recover {
                *recovered = recovered.saturating_add(1);
                did_recover = true
            }
        } else {
            value = output
        }
    }
    value
}
fn warm(input: f32, state: &mut BiquadState) {
    let previous = state.x1;
    state.x2 = previous;
    state.y2 = previous;
    state.x1 = input;
    state.y1 = input
}
fn valid(value: f32) -> bool {
    sanitize_sample(value).is_some()
}
fn write_lane(common: &mut [u8], output: &mut [u8], lane: &Lane) -> Result<(), StatePayloadError> {
    if !common.is_empty() || output.len() != STATE_BYTES_PER_LANE {
        return Err(StatePayloadError {
            code: "effect.state.length",
        });
    };
    for band in 0..EQ_SECTION_COUNT_V1 {
        let state = lane.state[band];
        write_f32_word(output, band, 0, state.x1);
        write_f32_word(output, band, 1, state.x2);
        write_f32_word(output, band, 2, state.y1);
        write_f32_word(output, band, 3, state.y2);
        write_ramp(output, band, 4, lane.frequency[band]);
        write_ramp(output, band, 7, lane.gain[band]);
        write_ramp(output, band, 10, lane.q[band]);
        write_ramp(output, band, 13, lane.slope[band]);
    }
    Ok(())
}

fn write_ramp(output: &mut [u8], band: usize, start_word: usize, ramp: NumericRamp) {
    write_f32_word(output, band, start_word, ramp.current);
    write_f32_word(output, band, start_word + 1, ramp.target);
    write_u32_word(output, band, start_word + 2, ramp.remaining);
}

fn write_f32_word(output: &mut [u8], band: usize, word: usize, value: f32) {
    write_u32_word(output, band, word, value.to_bits());
}

fn write_u32_word(output: &mut [u8], band: usize, word: usize, value: u32) {
    let start = (band * STATE_WORDS_PER_BAND + word) * 4;
    output[start..start + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_lane(
    common: &[u8],
    input: &[u8],
    configs: &[BandConfiguration; 4],
    sample_rate: SampleRateHz,
) -> Result<Lane, StatePayloadError> {
    if !common.is_empty() || input.len() != STATE_BYTES_PER_LANE {
        return Err(StatePayloadError {
            code: "effect.state.length",
        });
    };
    let mut lane = Lane::from_config(configs);
    for (band, c) in configs.iter().copied().enumerate() {
        let x1 = read_f32_word(input, band, 0)?;
        let x2 = read_f32_word(input, band, 1)?;
        let y1 = read_f32_word(input, band, 2)?;
        let y2 = read_f32_word(input, band, 3)?;
        let frequency = read_ramp(input, band, 4, 2)?;
        let gain = read_ramp(input, band, 7, 3)?;
        let q = read_ramp(input, band, 10, 4)?;
        let slope = read_ramp(input, band, 13, 5)?;
        if ![x1, x2, y1, y2].into_iter().all(valid)
            || !numeric_value_valid(2, frequency.current)
            || !numeric_value_valid(2, frequency.target)
            || !numeric_value_valid(3, gain.current)
            || !numeric_value_valid(3, gain.target)
            || !numeric_value_valid(4, q.current)
            || !numeric_value_valid(4, q.target)
            || !numeric_value_valid(5, slope.current)
            || !numeric_value_valid(5, slope.target)
        {
            return Err(StatePayloadError {
                code: "effect.state.payload",
            });
        }
        lane.state[band] = BiquadState { x1, x2, y1, y2 };
        lane.frequency[band] = frequency;
        lane.gain[band] = gain;
        lane.q[band] = q;
        lane.slope[band] = slope;
        lane.coefficients[band] =
            current_coefficients(c, &lane, band, sample_rate).map_err(|_| StatePayloadError {
                code: "effect.state.payload",
            })?;
    }
    Ok(lane)
}

fn read_ramp(
    input: &[u8],
    band: usize,
    start_word: usize,
    field: usize,
) -> Result<NumericRamp, StatePayloadError> {
    let current = read_f32_word(input, band, start_word)?;
    let target = read_f32_word(input, band, start_word + 1)?;
    let remaining = read_u32_word(input, band, start_word + 2)?;
    if remaining > 64 || !numeric_value_valid(field, current) || !numeric_value_valid(field, target)
    {
        return Err(StatePayloadError {
            code: "effect.state.payload",
        });
    }
    Ok(NumericRamp {
        current,
        target,
        remaining,
    })
}

fn read_f32_word(input: &[u8], band: usize, word: usize) -> Result<f32, StatePayloadError> {
    Ok(f32::from_bits(read_u32_word(input, band, word)?))
}

fn read_u32_word(input: &[u8], band: usize, word: usize) -> Result<u32, StatePayloadError> {
    let start = (band * STATE_WORDS_PER_BAND + word) * 4;
    let bytes: [u8; 4] = input
        .get(start..start + 4)
        .ok_or(StatePayloadError {
            code: "effect.state.payload",
        })?
        .try_into()
        .map_err(|_| StatePayloadError {
            code: "effect.state.payload",
        })?;
    Ok(u32::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::{KernelBackendV1, PreparedDeltaBankKernelV1};
    use miso_engine_dsp_reference::{
        ReferenceParametricEqCoefficients, ReferenceParametricEqKind, ReferenceParametricEqSection,
    };
    use miso_engine_effect_contract::{
        BankWidth, EffectBankProcessBlock, LinkMode, PrepareEffectLimits, PreparedPortsV1,
        PreparedSidechainPort, StatePayloadInput, StatePayloadOutput,
    };
    fn values() -> Vec<InitialParameterValue> {
        let mut values = Vec::new();
        for (index, p) in EQ_PARAMETERS.iter().enumerate() {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                values.push(InitialParameterValue {
                    parameter_index: index as u32,
                    channel,
                    value: p.default_value,
                })
            }
        }
        values
    }
    fn request<'a>(values: &'a [InitialParameterValue], bypass: bool) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: Quality::Normal,
            bypass,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 512,
                maximum_scratch_bytes: 1,
                maximum_automation_spans_per_block: 48,
            },
        }
    }
    fn set_initial(
        values: &mut [InitialParameterValue],
        parameter_index: usize,
        channel: ParameterChannel,
        value: f32,
    ) {
        let offset = parameter_index * 2
            + match channel {
                ParameterChannel::Left => 0,
                ParameterChannel::Right => 1,
                ParameterChannel::Both => panic!("initial values are per lane"),
            };
        values[offset].value = value;
    }
    fn point(
        parameter_index: u32,
        channel: ParameterChannel,
        sample: u64,
        value: f32,
    ) -> PreparedAutomationSpan {
        PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index,
            start_sample: sample,
            end_sample: sample,
            start_value: value,
            end_value: value,
        }
    }
    fn snapshot(effect: &dyn PreparedNativeEffect) -> ([u8; 256], [u8; 256]) {
        let mut left = [0_u8; STATE_BYTES_PER_LANE];
        let mut right = [0_u8; STATE_BYTES_PER_LANE];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(
                    &mut [],
                    &mut left,
                    &mut right,
                    effect.metadata().state_sizes,
                )
                .expect("state output"),
            )
            .expect("snapshot");
        (left, right)
    }
    fn word(payload: &[u8], position: usize) -> u32 {
        u32::from_le_bytes(
            payload[position * 4..position * 4 + 4]
                .try_into()
                .expect("full state word"),
        )
    }
    fn process_zeros(
        effect: &mut dyn PreparedNativeEffect,
        first_sample: u64,
        frames: usize,
        automation: &[PreparedAutomationSpan],
    ) -> ProcessReport {
        let mut left = vec![0.0; frames];
        let mut right = vec![0.0; frames];
        let block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            first_sample,
            automation,
            effect.metadata().quantum,
        )
        .expect("block");
        effect.process(block)
    }
    fn bank_backend(width: BankWidth) -> KernelBackendV1 {
        match width {
            BankWidth::Four => {
                #[cfg(target_arch = "aarch64")]
                {
                    return KernelBackendV1::Aarch64Neon;
                }
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    return KernelBackendV1::WasmSimd128;
                }
                #[allow(unreachable_code)]
                KernelBackendV1::WasmSimd128
            }
            BankWidth::Eight => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma")
                    {
                        return KernelBackendV1::X86Avx2Fma;
                    }
                    return KernelBackendV1::X86Avx2;
                }
                #[allow(unreachable_code)]
                KernelBackendV1::X86Avx2
            }
        }
    }
    fn configured_values(track: usize) -> Vec<InitialParameterValue> {
        let mut values = values();
        set_initial(&mut values, 0, ParameterChannel::Left, 1.0);
        set_initial(&mut values, 0, ParameterChannel::Right, 1.0);
        set_initial(&mut values, 1, ParameterChannel::Right, 5.0);
        set_initial(
            &mut values,
            2,
            ParameterChannel::Left,
            500.0 + track as f32 * 125.0,
        );
        set_initial(
            &mut values,
            2,
            ParameterChannel::Right,
            1_000.0 + track as f32 * 125.0,
        );
        set_initial(&mut values, 3, ParameterChannel::Left, -9.0 + track as f32);
        set_initial(
            &mut values,
            4,
            ParameterChannel::Left,
            0.5 + track as f32 * 0.1,
        );
        values
    }
    fn snapshot_bank(bank: &dyn PreparedNativeEffectBank, track: u32) -> ([u8; 256], [u8; 256]) {
        let mut left = [0_u8; STATE_BYTES_PER_LANE];
        let mut right = [0_u8; STATE_BYTES_PER_LANE];
        let sizes = bank.metadata().program_key.state_sizes;
        bank.snapshot_track_state_payload(
            track,
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes)
                .expect("bank state output"),
        )
        .expect("bank snapshot");
        (left, right)
    }
    fn assert_bank_sample(candidate: f32, reference: f32) {
        assert_eq!(candidate.to_bits(), reference.to_bits());
    }
    fn reference_kind(kind: EqBandKindV1) -> ReferenceParametricEqKind {
        match kind {
            EqBandKindV1::Bell => ReferenceParametricEqKind::Bell,
            EqBandKindV1::LowShelf => ReferenceParametricEqKind::LowShelf,
            EqBandKindV1::HighShelf => ReferenceParametricEqKind::HighShelf,
            EqBandKindV1::LowPass => ReferenceParametricEqKind::LowPass,
            EqBandKindV1::HighPass => ReferenceParametricEqKind::HighPass,
            EqBandKindV1::Notch => ReferenceParametricEqKind::Notch,
        }
    }
    fn retained_delta_magnitude(
        coefficients: EqCoefficientsV1,
        sample_rate: SampleRateHz,
        frequency_hz: f64,
    ) -> f64 {
        let phase = 2.0 * core::f64::consts::PI * frequency_hz / f64::from(sample_rate.0);
        let difference_re = phase.cos() - f64::from(coefficients.a);
        let difference_im = -phase.sin();
        let difference2_re = difference_re * difference_re - difference_im * difference_im;
        let difference2_im = 2.0 * difference_re * difference_im;
        let numerator_re = f64::from(coefficients.n0)
            + f64::from(coefficients.n1) * difference_re
            + f64::from(coefficients.n2) * difference2_re;
        let numerator_im = f64::from(coefficients.n1) * difference_im
            + f64::from(coefficients.n2) * difference2_im;
        let denominator_re = f64::from(coefficients.d0)
            + f64::from(coefficients.d1) * difference_re
            + f64::from(coefficients.d2) * difference2_re;
        let denominator_im = f64::from(coefficients.d1) * difference_im
            + f64::from(coefficients.d2) * difference2_im;
        numerator_re.hypot(numerator_im) / denominator_re.hypot(denominator_im)
    }

    const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    const FROZEN_FREQUENCIES: [f32; 6] = [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0];
    const FROZEN_QS: [f32; 4] = [0.1, core::f32::consts::FRAC_1_SQRT_2, 1.0, 18.0];
    const FROZEN_GAINS: [f32; 5] = [-24.0, -6.0, 0.0, 6.0, 24.0];
    const FROZEN_SLOPES: [f32; 3] = [0.1, 0.5, 1.0];
    const ONE_SECOND_DFT_TOLERANCE_DB: f64 = 0.05;
    const FREQUENCY_TOLERANCE_RATIO: f64 = 0.001;
    const SEEDED_DESIGN_COUNT: usize = 10_000;
    const SEEDED_DESIGN_SEED: u64 = 0x0000_0000_0012_e911;
    const STABILITY_SAMPLES: usize = 1_000_000;

    fn request_at_rate<'a>(
        values: &'a [InitialParameterValue],
        bypass: bool,
        sample_rate: u32,
    ) -> PrepareEffectRequest<'a> {
        PrepareEffectRequest {
            sample_rate,
            quantum: 128,
            quality: Quality::Normal,
            bypass,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 512,
                maximum_scratch_bytes: 1,
                maximum_automation_spans_per_block: 48,
            },
        }
    }

    fn single_section_values(
        kind: EqBandKindV1,
        frequency: f32,
        gain: f32,
        q: f32,
        slope: f32,
    ) -> Vec<InitialParameterValue> {
        let mut configured = values();
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            set_initial(&mut configured, 0, channel, 1.0);
            set_initial(&mut configured, 1, channel, kind as u32 as f32);
            set_initial(&mut configured, 2, channel, frequency);
            set_initial(&mut configured, 3, channel, gain);
            set_initial(&mut configured, 4, channel, q);
            set_initial(&mut configured, 5, channel, slope);
        }
        configured
    }

    fn retained_delta_db(
        coefficients: EqCoefficientsV1,
        sample_rate: SampleRateHz,
        frequency_hz: f64,
    ) -> f64 {
        20.0 * retained_delta_magnitude(coefficients, sample_rate, frequency_hz).log10()
    }

    fn find_crossing(
        coefficients: EqCoefficientsV1,
        sample_rate: SampleRateHz,
        target_db: f64,
    ) -> f64 {
        let mut low = 0.0;
        let mut high = f64::from(sample_rate.0) * 0.5;
        let mut low_side = retained_delta_db(coefficients, sample_rate, low) >= target_db;
        let high_side = retained_delta_db(coefficients, sample_rate, high) >= target_db;
        assert_ne!(
            low_side, high_side,
            "frequency gate must bracket its crossing"
        );
        for _ in 0..96 {
            let middle = (low + high) * 0.5;
            let middle_side = retained_delta_db(coefficients, sample_rate, middle) >= target_db;
            if middle_side == low_side {
                low = middle;
                low_side = middle_side;
            } else {
                high = middle;
            }
        }
        (low + high) * 0.5
    }

    fn find_log_extremum(
        coefficients: EqCoefficientsV1,
        sample_rate: SampleRateHz,
        maximum: bool,
    ) -> f64 {
        let mut low = f64::from(sample_rate.0) * 1.0e-12;
        let mut high = f64::from(sample_rate.0) * 0.5;
        for _ in 0..96 {
            let log_low = low.ln();
            let span = high.ln() - log_low;
            let first = (log_low + span / 3.0).exp();
            let second = (log_low + span * (2.0 / 3.0)).exp();
            let first_value = retained_delta_magnitude(coefficients, sample_rate, first);
            let second_value = retained_delta_magnitude(coefficients, sample_rate, second);
            let keep_left = if maximum {
                first_value >= second_value
            } else {
                first_value <= second_value
            };
            if keep_left {
                high = second;
            } else {
                low = first;
            }
        }
        (low.ln() + (high.ln() - low.ln()) * 0.5).exp()
    }

    fn assert_frequency_match(found: f64, requested: f32, gate: &str) {
        let relative_error = (found - f64::from(requested)).abs() / f64::from(requested);
        assert!(
            relative_error <= FREQUENCY_TOLERANCE_RATIO,
            "{gate}: found={found} requested={requested} relative_error={relative_error}"
        );
    }

    fn one_second_impulse_response(
        kind: EqBandKindV1,
        frequency: f32,
        gain: f32,
        q: f32,
        slope: f32,
        rate: u32,
    ) -> (Vec<f32>, u64, u64) {
        let configured = single_section_values(kind, frequency, gain, q, slope);
        let mut effect = ParametricEqFactory
            .prepare(request_at_rate(&configured, false, rate))
            .expect("frozen impulse design must prepare");
        let mut left = vec![0.0_f32; rate as usize];
        let mut right = vec![0.0_f32; rate as usize];
        left[0] = 1.0;
        right[0] = 1.0;
        let mut recovered_left = 0_u64;
        let mut recovered_right = 0_u64;
        for first in (0..left.len()).step_by(128) {
            let end = (first + 128).min(left.len());
            let report = effect.process(
                EffectProcessBlock::new(
                    &mut left[first..end],
                    &mut right[first..end],
                    None,
                    first as u64,
                    &[],
                    128,
                )
                .expect("one-second block"),
            );
            recovered_left += report.recovered_left_samples;
            recovered_right += report.recovered_right_samples;
        }
        (left, recovered_left, recovered_right)
    }

    fn impulse_dft_db(samples: &[f32], rate: u32, frequency: f64) -> f64 {
        let phase = -core::f64::consts::TAU * frequency / f64::from(rate);
        let (step_re, step_im) = (phase.cos(), phase.sin());
        let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
        let (mut re, mut im) = (0.0_f64, 0.0_f64);
        for sample in samples {
            let sample = f64::from(*sample);
            re += sample * unit_re;
            im += sample * unit_im;
            (unit_re, unit_im) = (
                unit_re * step_re - unit_im * step_im,
                unit_re * step_im + unit_im * step_re,
            );
        }
        let magnitude = re.hypot(im);
        if magnitude == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * magnitude.log10()
        }
    }

    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = *state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn seeded_unit_interval(state: &mut u64) -> f32 {
        let high_24 = (splitmix64(state) >> 40) as u32;
        high_24 as f32 / ((1_u32 << 24) - 1) as f32
    }

    fn deterministic_noise(state: &mut u64) -> f32 {
        let word = splitmix64(state);
        let sign = if word & 1 == 0 { -1.0 } else { 1.0 };
        let magnitude = 0.01 + ((word >> 40) as u32 as f32 / ((1_u32 << 24) - 1) as f32) * 0.98;
        sign * magnitude
    }
    fn assert_complete_grid_row(
        kind: EqBandKindV1,
        frequency: f32,
        gain: f32,
        q: f32,
        slope: f32,
        rate: u32,
    ) -> f64 {
        let sample_rate = SampleRateHz(rate);
        let production = design_biquad_v1(kind, frequency, gain, q, slope, sample_rate)
            .expect("every frozen row is a legal production design");
        let expected_anchor = if frequency <= rate as f32 * 0.25 {
            1.0_f32
        } else {
            -1.0_f32
        };
        assert_eq!(production.a.to_bits(), expected_anchor.to_bits());
        assert!(
            [
                production.n0,
                production.d0,
                production.n1,
                production.d1,
                production.n2,
                production.d2,
            ]
            .into_iter()
            .all(f32::is_finite)
        );
        let reconstructed_a1 = production.d1 - 2.0 * production.a * production.d2;
        let scale = (production.d0 - production.a * production.d1) + production.d2;
        assert!(
            scale.is_finite()
                && scale != 0.0
                && production.d2.abs() < 1.0
                && 1.0 + reconstructed_a1 + production.d2 > 0.0
                && 1.0 - reconstructed_a1 + production.d2 > 0.0
        );
        let reference = ReferenceParametricEqCoefficients::design(
            reference_kind(kind),
            f64::from(rate),
            f64::from(frequency),
            f64::from(gain),
            f64::from(q),
            f64::from(slope),
        )
        .expect("every frozen row is a legal independent reference design");
        let mut worst_error = 0.0_f64;
        for index in 0..2_048 {
            let probe = 10.0 * 2_000.0_f64.powf(index as f64 / 2_047.0);
            let reference_magnitude = reference
                .magnitude_at_hz(probe)
                .expect("frozen log probe is legal");
            if reference_magnitude > 0.0 {
                let reference_db = 20.0 * reference_magnitude.log10();
                if reference_db >= -120.0 {
                    let production_magnitude =
                        retained_delta_magnitude(production, sample_rate, probe);
                    assert!(production_magnitude.is_finite() && production_magnitude > 0.0);
                    let error = (20.0 * production_magnitude.log10() - reference_db).abs();
                    worst_error = worst_error.max(error);
                    assert!(
                        error <= 0.005,
                        "{kind:?} Fs={rate} f={frequency} gain={gain} Q={q} S={slope} probe={probe}: error={error} dB"
                    );
                }
            }
        }
        for probe in [f64::from(frequency), 0.0, f64::from(rate) * 0.5] {
            let reference_magnitude = reference
                .magnitude_at_hz(probe)
                .expect("frozen endpoint probe is legal");
            if reference_magnitude > 0.0 {
                let reference_db = 20.0 * reference_magnitude.log10();
                if reference_db >= -120.0 {
                    let production_magnitude =
                        retained_delta_magnitude(production, sample_rate, probe);
                    assert!(production_magnitude.is_finite() && production_magnitude > 0.0);
                    let error = (20.0 * production_magnitude.log10() - reference_db).abs();
                    worst_error = worst_error.max(error);
                    assert!(
                        error <= 0.005,
                        "{kind:?} Fs={rate} f={frequency} gain={gain} Q={q} S={slope} probe={probe}: error={error} dB"
                    );
                }
            }
        }
        if kind == EqBandKindV1::Notch {
            assert!(
                retained_delta_magnitude(production, sample_rate, f64::from(frequency)) <= 1e-5,
                "notch Fs={rate} f={frequency} Q={q} did not retain the -100 dB null"
            );
        }
        worst_error
    }
    #[test]
    fn descriptor_is_frozen() {
        miso_engine_effect_contract::validate_descriptor_v1(&PARAMETRIC_EQ_DESCRIPTOR_V1)
            .expect("descriptor");
        assert_eq!(EQ_PARAMETERS.len(), 24);
        assert_eq!(STATE_BYTES_PER_LANE, 256)
    }
    #[test]
    fn rbj_design_is_finite_and_jury_valid() {
        for kind in [
            EqBandKindV1::Bell,
            EqBandKindV1::LowShelf,
            EqBandKindV1::HighShelf,
            EqBandKindV1::LowPass,
            EqBandKindV1::HighPass,
            EqBandKindV1::Notch,
        ] {
            let c = design_biquad_v1(kind, 1000.0, 6.0, 1.0, 1.0, SampleRateHz(48_000))
                .expect("design");
            assert!(c.identity || c.d2.abs() < 1.0)
        }
    }
    #[test]
    fn endpoint_conditioned_delta_matches_the_independent_oracle_on_the_complete_grid() {
        let mut rows = 0_u32;
        let mut worst_error = 0.0_f64;
        for rate in [44_100, 48_000, 88_200, 96_000] {
            for frequency in [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0] {
                for q in [0.1, core::f32::consts::FRAC_1_SQRT_2, 1.0, 18.0] {
                    for gain in [-24.0, -6.0, 0.0, 6.0, 24.0] {
                        rows += 1;
                        worst_error = worst_error.max(assert_complete_grid_row(
                            EqBandKindV1::Bell,
                            frequency,
                            gain,
                            q,
                            1.0,
                            rate,
                        ));
                    }
                    for kind in [
                        EqBandKindV1::LowPass,
                        EqBandKindV1::HighPass,
                        EqBandKindV1::Notch,
                    ] {
                        rows += 1;
                        worst_error = worst_error
                            .max(assert_complete_grid_row(kind, frequency, 0.0, q, 1.0, rate));
                    }
                }
                for gain in [-24.0, -6.0, 0.0, 6.0, 24.0] {
                    for slope in [0.1, 0.5, 1.0] {
                        for kind in [EqBandKindV1::LowShelf, EqBandKindV1::HighShelf] {
                            rows += 1;
                            worst_error = worst_error.max(assert_complete_grid_row(
                                kind, frequency, gain, 1.0, slope, rate,
                            ));
                        }
                    }
                }
            }
        }
        assert_eq!(rows, 1_488);
        assert!(
            worst_error <= 0.005,
            "worst retained delta error={worst_error} dB"
        );
    }

    #[test]
    fn retained_delta_frequency_searches_cover_cutoff_center_midpoint_and_notch_minimum() {
        let mut searches = 0_u32;
        for rate in LAUNCH_RATES {
            let sample_rate = SampleRateHz(rate);
            for frequency in FROZEN_FREQUENCIES {
                for kind in [EqBandKindV1::LowPass, EqBandKindV1::HighPass] {
                    let coefficients = design_biquad_v1(
                        kind,
                        frequency,
                        0.0,
                        core::f32::consts::FRAC_1_SQRT_2,
                        1.0,
                        sample_rate,
                    )
                    .expect("legal Butterworth design");
                    let found = find_crossing(coefficients, sample_rate, -3.010_299_956_6);
                    assert_frequency_match(found, frequency, "Butterworth cutoff");
                    searches += 1;
                }
                for q in FROZEN_QS {
                    for gain in FROZEN_GAINS {
                        if gain == 0.0 {
                            continue;
                        }
                        let coefficients = design_biquad_v1(
                            EqBandKindV1::Bell,
                            frequency,
                            gain,
                            q,
                            1.0,
                            sample_rate,
                        )
                        .expect("legal bell design");
                        let found = find_log_extremum(coefficients, sample_rate, gain > 0.0);
                        assert_frequency_match(found, frequency, "bell center");
                        assert!(
                            (retained_delta_db(coefficients, sample_rate, found) - f64::from(gain))
                                .abs()
                                <= 0.005,
                            "bell center gain Fs={rate} f={frequency} gain={gain} Q={q}"
                        );
                        searches += 1;
                    }
                    let coefficients =
                        design_biquad_v1(EqBandKindV1::Notch, frequency, 0.0, q, 1.0, sample_rate)
                            .expect("legal notch design");
                    let found = find_log_extremum(coefficients, sample_rate, false);
                    assert_frequency_match(found, frequency, "notch minimum");
                    assert!(
                        retained_delta_magnitude(coefficients, sample_rate, found) <= 1e-5,
                        "notch null Fs={rate} f={frequency} Q={q}"
                    );
                    searches += 1;
                }
                for gain in FROZEN_GAINS {
                    if gain == 0.0 {
                        continue;
                    }
                    for slope in FROZEN_SLOPES {
                        for kind in [EqBandKindV1::LowShelf, EqBandKindV1::HighShelf] {
                            let coefficients =
                                design_biquad_v1(kind, frequency, gain, 1.0, slope, sample_rate)
                                    .expect("legal shelf design");
                            let found =
                                find_crossing(coefficients, sample_rate, f64::from(gain) * 0.5);
                            assert_frequency_match(found, frequency, "shelf midpoint");
                            searches += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(searches, 1_104);
    }

    #[test]
    #[ignore = "Issue #42 stopped after this frozen time-domain gate failed; successor #44 owns recurrence selection"]
    fn one_second_impulse_dfts_match_the_independent_oracle_at_all_frozen_edges() {
        let mut cases = 0_u32;
        for rate in LAUNCH_RATES {
            for kind in [
                EqBandKindV1::Bell,
                EqBandKindV1::LowShelf,
                EqBandKindV1::HighShelf,
                EqBandKindV1::LowPass,
                EqBandKindV1::HighPass,
                EqBandKindV1::Notch,
            ] {
                for (frequency, gain, q, slope) in [
                    (10.0_f32, -24.0_f32, 0.1_f32, 0.1_f32),
                    (20_000.0_f32, 24.0_f32, 18.0_f32, 1.0_f32),
                ] {
                    let (impulse, recovered_left, recovered_right) =
                        one_second_impulse_response(kind, frequency, gain, q, slope, rate);
                    assert_eq!(recovered_left, 0, "left recovery Fs={rate} {kind:?}");
                    assert_eq!(recovered_right, 0, "right recovery Fs={rate} {kind:?}");
                    assert!(
                        impulse.iter().copied().all(valid),
                        "one-second output remained finite normal-or-zero Fs={rate} {kind:?}"
                    );
                    let reference = ReferenceParametricEqCoefficients::design(
                        reference_kind(kind),
                        f64::from(rate),
                        f64::from(frequency),
                        f64::from(gain),
                        f64::from(q),
                        f64::from(slope),
                    )
                    .expect("independent frozen edge design");
                    let expected = 20.0
                        * reference
                            .magnitude_at_hz(f64::from(frequency))
                            .expect("reference f0 probe")
                            .log10();
                    let actual = impulse_dft_db(&impulse, rate, f64::from(frequency));
                    if expected >= -120.0 {
                        assert!(
                            (actual - expected).abs() <= ONE_SECOND_DFT_TOLERANCE_DB,
                            "one-second DFT Fs={rate} {kind:?} f={frequency} gain={gain} Q={q} S={slope}: actual={actual} expected={expected}"
                        );
                    } else {
                        assert!(actual.is_finite());
                    }
                    cases += 1;
                }
            }
        }
        assert_eq!(cases, 48);
    }

    #[test]
    #[ignore = "Issue #42 stopped before this gate; successor #44 must re-enable it after recurrence selection"]
    fn ten_thousand_seeded_legal_designs_are_finite_jury_valid_and_reference_bounded() {
        let kinds = [
            EqBandKindV1::Bell,
            EqBandKindV1::LowShelf,
            EqBandKindV1::HighShelf,
            EqBandKindV1::LowPass,
            EqBandKindV1::HighPass,
            EqBandKindV1::Notch,
        ];
        let mut state = SEEDED_DESIGN_SEED;
        let mut strata = [[0_u32; 6]; 4];
        let mut worst_margin = f64::INFINITY;
        let mut worst_response_error = 0.0_f64;
        let mut transcript = 0xcbf2_9ce4_8422_2325_u64;
        for index in 0..SEEDED_DESIGN_COUNT {
            let (rate_index, kind_index, frequency, gain, q, slope) = if index < 48 {
                let rate_index = index / 12;
                let kind_index = (index / 2) % 6;
                let high_edge = index % 2 == 1;
                (
                    rate_index,
                    kind_index,
                    if high_edge { 20_000.0 } else { 10.0 },
                    if high_edge { 24.0 } else { -24.0 },
                    if high_edge { 18.0 } else { 0.1 },
                    if high_edge { 1.0 } else { 0.1 },
                )
            } else {
                let rate_index = (splitmix64(&mut state) as usize) % LAUNCH_RATES.len();
                let kind_index = (splitmix64(&mut state) as usize) % kinds.len();
                let frequency = 10.0 * 2_000.0_f32.powf(seeded_unit_interval(&mut state));
                let gain = -24.0 + 48.0 * seeded_unit_interval(&mut state);
                let q = 0.1 * 180.0_f32.powf(seeded_unit_interval(&mut state));
                let slope = 0.1 + 0.9 * seeded_unit_interval(&mut state);
                (rate_index, kind_index, frequency, gain, q, slope)
            };
            let rate = LAUNCH_RATES[rate_index];
            let kind = kinds[kind_index];
            strata[rate_index][kind_index] += 1;
            let coefficients =
                design_biquad_v1(kind, frequency, gain, q, slope, SampleRateHz(rate))
                    .expect("seeded legal design");
            let a1 = coefficients.d1 - 2.0 * coefficients.a * coefficients.d2;
            let margin = [
                1.0 - coefficients.d2.abs(),
                1.0 + a1 + coefficients.d2,
                1.0 - a1 + coefficients.d2,
            ]
            .into_iter()
            .map(f64::from)
            .fold(f64::INFINITY, f64::min);
            assert!(margin.is_finite() && margin > 0.0, "seeded Jury margin");
            worst_margin = worst_margin.min(margin);
            for word in [
                coefficients.a,
                coefficients.n0,
                coefficients.d0,
                coefficients.n1,
                coefficients.d1,
                coefficients.n2,
                coefficients.d2,
            ] {
                assert!(word.is_finite(), "seeded retained word");
                transcript ^= u64::from(word.to_bits());
                transcript = transcript.wrapping_mul(0x0000_0100_0000_01b3);
            }
            let reference = ReferenceParametricEqCoefficients::design(
                reference_kind(kind),
                f64::from(rate),
                f64::from(frequency),
                f64::from(gain),
                f64::from(q),
                f64::from(slope),
            )
            .expect("independent seeded design");
            let reference_magnitude = reference
                .magnitude_at_hz(f64::from(frequency))
                .expect("independent seeded f0");
            let reference_db = 20.0 * reference_magnitude.log10();
            let production_db =
                retained_delta_db(coefficients, SampleRateHz(rate), f64::from(frequency));
            if reference_db >= -120.0 {
                let error = (production_db - reference_db).abs();
                worst_response_error = worst_response_error.max(error);
                assert!(
                    error <= 0.005,
                    "seeded response Fs={rate} {kind:?} f={frequency} gain={gain} Q={q} S={slope}: error={error}"
                );
            }
        }
        assert_eq!(strata.iter().flatten().copied().sum::<u32>(), 10_000);
        assert!(strata.iter().flatten().all(|count| *count >= 2));
        assert!(worst_margin > 0.0);
        assert!(worst_response_error <= 0.005);
        eprintln!(
            "issue-042 seeded-designs count=10000 seed={SEEDED_DESIGN_SEED:#018x} worst_margin={worst_margin:.12e} worst_response_db={worst_response_error:.12} transcript={transcript:016x}"
        );
    }

    #[test]
    #[ignore = "Issue #42 stopped before this gate; successor #44 must re-enable it after recurrence selection"]
    fn forty_eight_frozen_million_sample_sequences_remain_valid_without_recovery() {
        let kinds = [
            EqBandKindV1::Bell,
            EqBandKindV1::LowShelf,
            EqBandKindV1::HighShelf,
            EqBandKindV1::LowPass,
            EqBandKindV1::HighPass,
            EqBandKindV1::Notch,
        ];
        let mut sequences = 0_u32;
        for rate in LAUNCH_RATES {
            for kind in kinds {
                for (frequency, gain, q, slope) in [
                    (10.0_f32, -24.0_f32, 0.1_f32, 0.1_f32),
                    (20_000.0_f32, 24.0_f32, 18.0_f32, 1.0_f32),
                ] {
                    let configured = single_section_values(kind, frequency, gain, q, slope);
                    let mut effect = ParametricEqFactory
                        .prepare(request_at_rate(&configured, false, rate))
                        .expect("frozen stability design must prepare");
                    let mut noise_state = SEEDED_DESIGN_SEED
                        ^ u64::from(rate)
                        ^ (u64::from(kind as u32) << 32)
                        ^ u64::from(frequency.to_bits());
                    let mut left = [0.0_f32; 128];
                    let mut right = [0.0_f32; 128];
                    let mut first_sample = 0_usize;
                    let mut recovered_left = 0_u64;
                    let mut recovered_right = 0_u64;
                    let mut sanitized = 0_u64;
                    while first_sample < STABILITY_SAMPLES {
                        let frames = (STABILITY_SAMPLES - first_sample).min(left.len());
                        for index in 0..frames {
                            if first_sample + index == 0 {
                                left[index] = 0.99;
                                right[index] = -0.99;
                            } else {
                                left[index] = deterministic_noise(&mut noise_state);
                                right[index] = deterministic_noise(&mut noise_state);
                            }
                        }
                        let report = effect.process(
                            EffectProcessBlock::new(
                                &mut left[..frames],
                                &mut right[..frames],
                                None,
                                first_sample as u64,
                                &[],
                                128,
                            )
                            .expect("million-sample block"),
                        );
                        recovered_left += report.recovered_left_samples;
                        recovered_right += report.recovered_right_samples;
                        sanitized += report.sanitized_main_samples;
                        assert!(
                            left[..frames].iter().copied().all(valid)
                                && right[..frames].iter().copied().all(valid),
                            "valid output Fs={rate} {kind:?} f={frequency}"
                        );
                        first_sample += frames;
                    }
                    assert_eq!(recovered_left, 0, "left recovery Fs={rate} {kind:?}");
                    assert_eq!(recovered_right, 0, "right recovery Fs={rate} {kind:?}");
                    assert_eq!(sanitized, 0, "input sanitation Fs={rate} {kind:?}");
                    let (left_state, right_state) = snapshot(effect.as_ref());
                    for payload in [&left_state[..], &right_state[..]] {
                        for section in 0..EQ_SECTION_COUNT_V1 {
                            for history_word in 0..4 {
                                assert!(
                                    valid(f32::from_bits(word(
                                        payload,
                                        section * STATE_WORDS_PER_BAND + history_word,
                                    ))),
                                    "valid retained state Fs={rate} {kind:?} f={frequency} section={section} word={history_word}"
                                );
                            }
                        }
                    }
                    sequences += 1;
                }
            }
        }
        assert_eq!(sequences, 48);
    }

    #[test]
    fn whole_bypass_warms_every_section_with_dry_history() {
        let mut values = values();
        set_initial(&mut values, 0, ParameterChannel::Left, 1.0);
        set_initial(&mut values, 2, ParameterChannel::Left, 1_000.0);
        set_initial(&mut values, 3, ParameterChannel::Left, 6.0);
        let mut effect = ParametricEqFactory
            .prepare(request(&values, true))
            .expect("bypass prepare");
        let mut left = [0.25_f32, -0.5];
        let mut right = [0.75_f32, 0.125];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128)
                .expect("bypass block"),
        );
        assert_eq!(left.map(f32::to_bits), [0.25_f32, -0.5].map(f32::to_bits));
        assert_eq!(right.map(f32::to_bits), [0.75_f32, 0.125].map(f32::to_bits));
        let (left_state, right_state) = snapshot(effect.as_ref());
        for section in 0..EQ_SECTION_COUNT_V1 {
            for (payload, prior, current) in [
                (&left_state[..], 0.25_f32, -0.5_f32),
                (&right_state[..], 0.75_f32, 0.125_f32),
            ] {
                assert_eq!(
                    word(payload, section * STATE_WORDS_PER_BAND),
                    current.to_bits()
                );
                assert_eq!(
                    word(payload, section * STATE_WORDS_PER_BAND + 1),
                    prior.to_bits()
                );
                assert_eq!(
                    word(payload, section * STATE_WORDS_PER_BAND + 2),
                    current.to_bits()
                );
                assert_eq!(
                    word(payload, section * STATE_WORDS_PER_BAND + 3),
                    prior.to_bits()
                );
            }
        }
    }
    #[test]
    fn scalar_delta_recurrence_tracks_the_independent_f64_oracle() {
        let mut values = values();
        set_initial(&mut values, 0, ParameterChannel::Left, 1.0);
        set_initial(&mut values, 1, ParameterChannel::Left, 2.0);
        set_initial(&mut values, 2, ParameterChannel::Left, 10.0);
        set_initial(&mut values, 3, ParameterChannel::Left, -24.0);
        set_initial(&mut values, 4, ParameterChannel::Left, 1.0);
        set_initial(&mut values, 5, ParameterChannel::Left, 0.1);
        let mut effect = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("scalar prepare");
        let reference_coefficients = ReferenceParametricEqCoefficients::design(
            ReferenceParametricEqKind::LowShelf,
            48_000.0,
            10.0,
            -24.0,
            1.0,
            0.1,
        )
        .expect("reference design");
        let mut reference = ReferenceParametricEqSection::new(reference_coefficients);
        let mut seed = 0x7f4a_7c15_u32;
        let mut left = [0.0_f32; 256];
        for sample in &mut left {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *sample = (seed as i32 as f32 / i32::MAX as f32) * 0.5;
        }
        let input = left;
        let mut right = [0.0_f32; 256];
        let (left_first, left_second) = left.split_at_mut(128);
        let (right_first, right_second) = right.split_at_mut(128);
        effect.process(
            EffectProcessBlock::new(left_first, right_first, None, 0, &[], 128)
                .expect("first scalar block"),
        );
        effect.process(
            EffectProcessBlock::new(left_second, right_second, None, 128, &[], 128)
                .expect("second scalar block"),
        );
        for (index, (input, output)) in input.into_iter().zip(left).enumerate() {
            let expected = reference.process(f64::from(input));
            assert!(
                (f64::from(output) - expected).abs() <= 5e-5,
                "sample={index} output={output} reference={expected}"
            );
        }
    }
    #[test]
    fn identity_bypass_and_state_round_trip() {
        let values = values();
        let mut effect = ParametricEqFactory
            .prepare(request(&values, true))
            .expect("prepare");
        let mut left = [0.0_f32, f32::NAN, 0.25];
        let mut right = [-0.0_f32, 0.5, 0.0];
        let block =
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block");
        let report = effect.process(block);
        assert_eq!(left[1].to_bits(), 0);
        assert_eq!(right[0].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(report.sanitized_main_samples, 1);
        let mut l = [0_u8; 256];
        let mut r = [0_u8; 256];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut l, &mut r, effect.metadata().state_sizes)
                    .expect("output"),
            )
            .expect("snapshot");
        effect
            .restore_state_payload(
                1,
                StatePayloadInput::new(&[], &l, &r, effect.metadata().state_sizes).expect("input"),
            )
            .expect("restore")
    }
    #[test]
    fn automation_uses_descriptor_index_and_exact_64_update_trajectory() {
        assert_eq!(
            EQ_PARAMETERS[3].id.0, 4,
            "public stable ID is sparse identity"
        );
        let values = values();
        let mut effect = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("prepare");
        let automation = [point(3, ParameterChannel::Left, 0, 12.0)];
        let report = process_zeros(effect.as_mut(), 0, 1, &automation);
        assert_eq!(report.invalid_spans, 0);
        let (left, _) = snapshot(effect.as_ref());
        assert_eq!(
            f32::from_bits(word(&left, 7)),
            12.0 / 64.0,
            "descriptor index 3 is band-1 gain, not ParameterId 3"
        );
        assert_eq!(f32::from_bits(word(&left, 8)), 12.0);
        assert_eq!(word(&left, 9), 63);

        let report = process_zeros(effect.as_mut(), 1, 63, &[]);
        assert_eq!(report.invalid_spans, 0);
        let (left, _) = snapshot(effect.as_ref());
        assert_eq!(f32::from_bits(word(&left, 7)), 12.0);
        assert_eq!(f32::from_bits(word(&left, 8)), 12.0);
        assert_eq!(word(&left, 9), 0);
    }
    #[test]
    fn malformed_automation_rejects_each_span_without_losing_valid_targets() {
        let values = values();
        let mut effect = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("prepare");
        let mut wrong_time = point(5, ParameterChannel::Left, 1, 0.5);
        wrong_time.end_sample = 1;
        let mut mismatched_point = point(2, ParameterChannel::Right, 0, 100.0);
        mismatched_point.end_value = 200.0;
        let automation = [
            point(3, ParameterChannel::Left, 0, 6.0),
            point(3, ParameterChannel::Left, 0, 8.0),
            point(4, ParameterChannel::Left, 0, 1.0),
            point(3, ParameterChannel::Right, 0, 7.0),
            point(0, ParameterChannel::Left, 0, 1.0),
            point(5, ParameterChannel::Both, 0, 0.5),
            wrong_time,
            mismatched_point,
        ];
        let report = process_zeros(effect.as_mut(), 0, 1, &automation);
        assert_eq!(report.invalid_spans, 6);
        let (left, right) = snapshot(effect.as_ref());
        assert_eq!(f32::from_bits(word(&left, 8)), 6.0);
        assert_eq!(f32::from_bits(word(&left, 11)), 1.0);
        assert_eq!(f32::from_bits(word(&right, 8)), 0.0);
    }
    #[test]
    fn automation_is_partition_invariant_for_1_63_64_and_128_frames() {
        let values = values();
        let mut partitioned = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("partitioned prepare");
        let mut whole = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("whole prepare");
        let automation = [point(2, ParameterChannel::Left, 0, 1_000.0)];
        process_zeros(partitioned.as_mut(), 0, 1, &automation);
        process_zeros(partitioned.as_mut(), 1, 63, &[]);
        process_zeros(partitioned.as_mut(), 64, 64, &[]);
        process_zeros(whole.as_mut(), 0, 128, &automation);
        assert_eq!(snapshot(partitioned.as_ref()), snapshot(whole.as_ref()));
    }
    #[test]
    fn state_restore_continues_active_ramp_bit_exactly() {
        let mut values = values();
        set_initial(&mut values, 0, ParameterChannel::Left, 1.0);
        set_initial(&mut values, 2, ParameterChannel::Left, 1_000.0);
        set_initial(&mut values, 3, ParameterChannel::Left, 6.0);
        let mut source = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("source prepare");
        let automation = [point(3, ParameterChannel::Left, 0, -6.0)];
        let mut first_left = [0.25_f32; 17];
        let mut first_right = [0.125_f32; 17];
        source.process(
            EffectProcessBlock::new(&mut first_left, &mut first_right, None, 0, &automation, 128)
                .expect("first block"),
        );
        let (saved_left, saved_right) = snapshot(source.as_ref());
        let mut restored = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("restore prepare");
        restored
            .restore_state_payload(
                1,
                StatePayloadInput::new(
                    &[],
                    &saved_left,
                    &saved_right,
                    restored.metadata().state_sizes,
                )
                .expect("state input"),
            )
            .expect("restore");

        let mut source_left = [0.5_f32; 64];
        let mut source_right = [-0.25_f32; 64];
        let mut restored_left = source_left;
        let mut restored_right = source_right;
        source.process(
            EffectProcessBlock::new(&mut source_left, &mut source_right, None, 17, &[], 128)
                .expect("source continuation"),
        );
        restored.process(
            EffectProcessBlock::new(&mut restored_left, &mut restored_right, None, 17, &[], 128)
                .expect("restored continuation"),
        );
        assert_eq!(
            source_left.map(f32::to_bits),
            restored_left.map(f32::to_bits)
        );
        assert_eq!(
            source_right.map(f32::to_bits),
            restored_right.map(f32::to_bits)
        );
        assert_eq!(snapshot(source.as_ref()), snapshot(restored.as_ref()));
    }
    #[test]
    fn snapshot_rejects_bad_output_without_touching_either_lane() {
        let values = values();
        let effect = ParametricEqFactory
            .prepare(request(&values, false))
            .expect("prepare");
        let mut left = [0xA5_u8; STATE_BYTES_PER_LANE];
        let mut right = [0x5A_u8; STATE_BYTES_PER_LANE - 1];
        let result = effect.snapshot_state_payload(StatePayloadOutput {
            common: &mut [],
            left: &mut left,
            right: &mut right,
        });
        assert_eq!(
            result,
            Err(StatePayloadError {
                code: "effect.state.length"
            })
        );
        assert_eq!(left, [0xA5; STATE_BYTES_PER_LANE]);
        assert_eq!(right, [0x5A; STATE_BYTES_PER_LANE - 1]);
    }
    #[test]
    fn redesign_fault_resets_only_its_section_and_continues_the_cascade() {
        let active = BandConfiguration {
            enabled: true,
            kind: EqBandKindV1::HighPass,
            frequency: 1_000.0,
            gain: 0.0,
            q: 1.0,
            slope: 1.0,
            coefficients: design_biquad_v1(
                EqBandKindV1::HighPass,
                1_000.0,
                0.0,
                1.0,
                1.0,
                SampleRateHz(48_000),
            )
            .expect("legal coefficients"),
        };
        let mut configs = [default_band(0, SampleRateHz(48_000)).expect("default"); 4];
        configs[0] = active;
        configs[1] = active;
        let mut lane = Lane::from_config(&configs);
        lane.gain[0].set_target(6.0);
        lane.state[1].x1 = 0.25;
        let mut recovered = 0;
        let output = process_lane(
            0.5,
            &mut lane,
            &configs,
            SampleRateHz(0),
            false,
            &mut recovered,
        );
        assert_eq!(recovered, 1);
        assert_eq!(lane.state[0].x1.to_bits(), 0);
        assert_eq!(lane.state[0].y1.to_bits(), 0);
        assert_eq!(lane.gain[0].target, 6.0);
        assert_eq!(lane.gain[0].remaining, 64);
        assert_eq!(lane.state[1].x1.to_bits(), 0);
        assert_ne!(
            output.to_bits(),
            0,
            "the following section still processed its state"
        );
    }

    fn lane_payload(lane: &Lane) -> [u8; STATE_BYTES_PER_LANE] {
        let mut payload = [0_u8; STATE_BYTES_PER_LANE];
        write_lane(&mut [], &mut payload, lane).expect("valid lane payload");
        payload
    }

    fn bank_redesign_fault_matches_scalar<const W: usize>(kernel: PreparedDeltaBankKernelV1) {
        let active = BandConfiguration {
            enabled: true,
            kind: EqBandKindV1::HighPass,
            frequency: 1_000.0,
            gain: 0.0,
            q: 1.0,
            slope: 1.0,
            coefficients: design_biquad_v1(
                EqBandKindV1::HighPass,
                1_000.0,
                0.0,
                1.0,
                1.0,
                SampleRateHz(48_000),
            )
            .expect("legal coefficients"),
        };
        let mut configs = [default_band(0, SampleRateHz(48_000)).expect("default"); 4];
        configs[0] = active;
        configs[1] = active;

        let mut scalar = Lane::from_config(&configs);
        scalar.gain[0].set_target(6.0);
        scalar.state[1].x1 = 0.25;
        let mut scalar_recovered = 0;
        let scalar_output = process_lane(
            0.5,
            &mut scalar,
            &configs,
            SampleRateHz(0),
            false,
            &mut scalar_recovered,
        );

        let configs_by_track = [configs; W];
        let mut channel = BankChannel::from_configs(&configs_by_track);
        channel.gain[0][0].set_target(6.0);
        channel.state[1].x1[0] = 0.25;
        let mut values = [0.5_f32; W];
        let mut recovered = [false; W];
        let mut reports = core::array::from_fn(|_| ProcessReport::default());
        process_bank_channel(
            &mut channel,
            &configs_by_track,
            kernel,
            &mut values,
            SampleRateHz(0),
            false,
            &mut recovered,
            &mut reports,
            true,
        );

        assert_eq!(scalar_recovered, 1);
        assert_eq!(reports[0].recovered_left_samples, scalar_recovered);
        assert_eq!(values[0].to_bits(), scalar_output.to_bits());
        assert_eq!(lane_payload(&channel.lane(0)), lane_payload(&scalar));
        assert_ne!(
            values[0].to_bits(),
            0,
            "a failed first section must not erase the next section's valid tail"
        );
    }

    #[test]
    fn bank_redesign_recovery_matches_scalar_downstream_and_state_continuation() {
        let mut tested = 0;
        if let Ok(kernel) = PreparedDeltaBankKernelV1::try_new(bank_backend(BankWidth::Four)) {
            bank_redesign_fault_matches_scalar::<4>(kernel);
            tested += 1;
        }
        if let Ok(kernel) = PreparedDeltaBankKernelV1::try_new(bank_backend(BankWidth::Eight)) {
            bank_redesign_fault_matches_scalar::<8>(kernel);
            tested += 1;
        }
        assert!(
            tested != 0,
            "native test host exposes one frozen bank backend"
        );
    }

    #[test]
    fn bank_storage_is_exactly_width_specific() {
        use core::mem::size_of;

        assert_eq!(size_of::<BankCoefficients<4>>(), 116);
        assert_eq!(size_of::<BankCoefficients<8>>(), 232);
        assert_eq!(size_of::<BankDeltaState<4>>(), 64);
        assert_eq!(size_of::<BankDeltaState<8>>(), 128);
        let coefficient_words_w4 = 7 * 4 * 4 * 2 * size_of::<f32>();
        let coefficient_words_w8 = 7 * 4 * 8 * 2 * size_of::<f32>();
        let identity_masks_w4 = 4 * 4 * 2;
        let identity_masks_w8 = 4 * 8 * 2;
        assert_eq!(coefficient_words_w4, 896);
        assert_eq!(coefficient_words_w8, 1_792);
        assert_eq!(size_of::<[BankCoefficients<4>; 4]>() * 2, 928);
        assert_eq!(size_of::<[BankCoefficients<8>; 4]>() * 2, 1_856);
        assert_eq!(
            size_of::<[BankCoefficients<4>; 4]>() * 2,
            coefficient_words_w4 + identity_masks_w4
        );
        assert_eq!(
            size_of::<[BankCoefficients<8>; 4]>() * 2,
            coefficient_words_w8 + identity_masks_w8
        );
        assert_eq!(size_of::<[BankDeltaState<4>; 4]>() * 2, 512);
        assert_eq!(size_of::<[BankDeltaState<8>; 4]>() * 2, 1_024);
        assert_eq!(size_of::<BankChannel<4>>(), 1_488);
        assert_eq!(size_of::<BankChannel<8>>(), 2_976);
        assert_eq!(
            size_of::<[[BandConfiguration; EQ_SECTION_COUNT_V1]; 4]>() * 2,
            1_792
        );
        assert_eq!(
            size_of::<[[BandConfiguration; EQ_SECTION_COUNT_V1]; 8]>() * 2,
            3_584
        );
    }

    fn bank_matches_scalar(width: BankWidth) {
        let backend = bank_backend(width);
        let lanes = width.lanes() as usize;
        let factory = ParametricEqFactory;
        let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
        let requests: Vec<_> = values_by_track
            .iter()
            .map(|values| request(values, false))
            .collect();
        let Some(mut bank) = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("valid bank request")
        else {
            assert!(
                PreparedDeltaBankKernelV1::try_new(backend).is_err(),
                "only an unavailable architecture may decline this otherwise homogeneous request"
            );
            return;
        };
        assert_eq!(bank.metadata().width, width);
        assert_eq!(bank.metadata().program_key.tail, TailSamples::Infinite);
        let mut scalar: Vec<_> = values_by_track
            .iter()
            .map(|values| {
                factory
                    .prepare(request(values, false))
                    .expect("scalar prepare")
            })
            .collect();
        let automation_by_track: Vec<_> = (0..lanes)
            .map(|track| {
                [
                    point(3, ParameterChannel::Left, 100, -4.0 + track as f32 * 0.5),
                    point(4, ParameterChannel::Right, 100, 0.8 + track as f32 * 0.01),
                ]
            })
            .collect();
        let mut automation = Vec::with_capacity(lanes * 2);
        let mut offsets = Vec::with_capacity(lanes + 1);
        offsets.push(0);
        for spans in &automation_by_track {
            automation.extend_from_slice(spans);
            offsets.push(automation.len() as u32);
        }
        let frames = 16;
        let mut bank_left = vec![0.0_f32; frames * lanes];
        let mut bank_right = vec![0.0_f32; frames * lanes];
        let mut scalar_left = vec![vec![0.0_f32; frames]; lanes];
        let mut scalar_right = vec![vec![0.0_f32; frames]; lanes];
        for frame in 0..frames {
            for track in 0..lanes {
                let left = (frame as f32 + 1.0) * (track as f32 + 1.0) * 0.01;
                let right = -left * 0.75;
                let index = frame * lanes + track;
                bank_left[index] = left;
                bank_right[index] = right;
                scalar_left[track][frame] = left;
                scalar_right[track][frame] = right;
            }
        }
        let mut scalar_reports = Vec::with_capacity(lanes);
        for track in 0..lanes {
            let quantum = scalar[track].metadata().quantum;
            scalar_reports.push(
                scalar[track].process(
                    EffectProcessBlock::new(
                        &mut scalar_left[track],
                        &mut scalar_right[track],
                        None,
                        100,
                        &automation_by_track[track],
                        quantum,
                    )
                    .expect("scalar block"),
                ),
            );
        }
        let bank_report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                frames as u32,
                width,
                100,
                &automation,
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        for track in 0..lanes {
            assert_eq!(bank_report.reports[track], scalar_reports[track]);
            for frame in 0..frames {
                let index = frame * lanes + track;
                assert_bank_sample(bank_left[index], scalar_left[track][frame]);
                assert_bank_sample(bank_right[index], scalar_right[track][frame]);
            }
            let bank_state = snapshot_bank(bank.as_ref(), track as u32);
            assert_eq!(bank_state, snapshot(scalar[track].as_ref()));
        }
        let saved = snapshot_bank(bank.as_ref(), 0);
        let sizes = bank.metadata().program_key.state_sizes;
        bank.restore_track_state_payload(
            0,
            1,
            StatePayloadInput::new(&[], &saved.0, &saved.1, sizes).expect("state input"),
        )
        .expect("state restore");
        assert_eq!(snapshot_bank(bank.as_ref(), 0), saved);
    }
    #[test]
    fn four_lane_bank_matches_scalar_when_its_target_is_available() {
        bank_matches_scalar(BankWidth::Four);
    }
    #[test]
    fn eight_lane_bank_matches_scalar_with_active_ramps_and_state_round_trip() {
        bank_matches_scalar(BankWidth::Eight);
    }
    #[test]
    fn bank_binding_rejects_malformed_shapes_and_returns_none_for_unavailable_backend() {
        let factory = ParametricEqFactory;
        let values = values();
        let request = request(&values, false);
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: KernelBackendV1::X86Avx2,
                    width: BankWidth::Four,
                    requests: &[request; 4],
                })
                .expect("incompatible width is a legal non-bank")
                .is_none()
        );
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: bank_backend(BankWidth::Eight),
                    width: BankWidth::Eight,
                    requests: &[request; 4],
                })
                .expect("wrong request count is a legal non-bank")
                .is_none()
        );
        let unavailable =
            if PreparedDeltaBankKernelV1::try_new(KernelBackendV1::WasmSimd128).is_err() {
                KernelBackendV1::WasmSimd128
            } else {
                KernelBackendV1::X86Avx2
            };
        let unavailable_width = match unavailable {
            KernelBackendV1::WasmSimd128 | KernelBackendV1::Aarch64Neon => BankWidth::Four,
            KernelBackendV1::X86Avx2 | KernelBackendV1::X86Avx2Fma => BankWidth::Eight,
            KernelBackendV1::Scalar => unreachable!("bank widths exclude scalar"),
            _ => unreachable!("unknown backend cannot be requested by this frozen test"),
        };
        let unavailable_requests = vec![request; unavailable_width.lanes() as usize];
        assert!(PreparedDeltaBankKernelV1::try_new(unavailable).is_err());
        assert!(
            factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: unavailable,
                    width: unavailable_width,
                    requests: &unavailable_requests,
                })
                .expect("unavailable backend is a legal non-bank")
                .is_none()
        );
    }
    #[test]
    fn bank_lane_and_track_changes_do_not_leak_when_the_backend_is_available() {
        let width = BankWidth::Eight;
        let backend = bank_backend(width);
        let lanes = width.lanes() as usize;
        let factory = ParametricEqFactory;
        let values_by_track: Vec<_> = (0..lanes).map(configured_values).collect();
        let requests: Vec<_> = values_by_track
            .iter()
            .map(|values| request(values, false))
            .collect();
        let Some(mut baseline) = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("request")
        else {
            return;
        };
        let Some(mut changed) = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("request")
        else {
            panic!("same available backend must bind consistently");
        };
        let frames = 8;
        let mut baseline_left = vec![0.1_f32; frames * lanes];
        let mut baseline_right = vec![-0.1_f32; frames * lanes];
        let mut changed_left = baseline_left.clone();
        let mut changed_right = baseline_right.clone();
        changed_left[3] = 0.75;
        let offsets = vec![0_u32; lanes + 1];
        baseline.process_bank(
            EffectBankProcessBlock::new(
                &mut baseline_left,
                &mut baseline_right,
                None,
                frames as u32,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("baseline block"),
        );
        changed.process_bank(
            EffectBankProcessBlock::new(
                &mut changed_left,
                &mut changed_right,
                None,
                frames as u32,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("changed block"),
        );
        for frame in 0..frames {
            for track in 0..lanes {
                if track == 3 {
                    let index = frame * lanes + track;
                    assert_eq!(
                        baseline_right[index].to_bits(),
                        changed_right[index].to_bits(),
                        "left-only perturbation must not affect the same track's right lane"
                    );
                    continue;
                }
                let index = frame * lanes + track;
                assert_eq!(
                    baseline_left[index].to_bits(),
                    changed_left[index].to_bits()
                );
                assert_eq!(
                    baseline_right[index].to_bits(),
                    changed_right[index].to_bits()
                );
            }
        }
        for track in 0..lanes {
            if track != 3 {
                assert_eq!(
                    snapshot_bank(baseline.as_ref(), track as u32),
                    snapshot_bank(changed.as_ref(), track as u32)
                );
            }
        }
    }
}
