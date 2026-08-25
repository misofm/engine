//! Four-section dual-mono parametric EQ, realised as a cascade of TPT state-variable sections.
//!
//! The spec transfer is the RBJ Audio EQ Cookbook's, unchanged. The *realization* is Simper's
//! trapezoidal state-variable filter (decision D2 of the #83 master plan), designed in `f64` on the
//! control plane and rounded once into six `f32` words per section. Issue #87 replaced the shipped
//! "endpoint-conditioned delta" recurrence — a direct-form-I biquad with a re-labelled denominator,
//! four `f32` histories and a division per sample — because it did not realize the transfer its own
//! grid test certified (483 of 1,488 frozen rows failed, worst 12.4859 dB) and because its recovery
//! predicate counted a correctly decaying subnormal tail as a fault.
//!
//! # What is pinned
//!
//! * **Storage.** `c1 = t / (1 + t)` with `t = g * (g + k)`, never `a1 = 1 / (1 + t)`: at 10 Hz,
//!   Q = 18 and 88.2 kHz, `t` is about 4.7e-6, so `a1` rounded to `f32` carries about 0.6 % relative
//!   error in the pole damping while `c1` carries about 6e-8 (master plan §4.2 amendment A1).
//! * **The recurrence.** `miso_engine_lane::kernels::svf_block`, one generic body instantiated at
//!   `WIDTH` 1, 4 and 8, so lane identity and native↔wasm identity are properties of the code.
//! * **Smoothing.** Decision D11: an automation point starts a 64-sample linear ramp of the six
//!   **words**, with the per-sample increment precomputed as a multiply by `2^-6` and an exact
//!   assignment of the target on the final sample. There is no per-sample redesign and no division
//!   anywhere on the render path.
//! * **Denormals and faults.** Decision D7: the two integrator words are flushed inside the kernel;
//!   output finiteness is checked once per block per channel through
//!   `miso_engine_effect_runtime::bank`. A subnormal sample is a legal signal value, not a fault.
//!
//! # State layout
//!
//! Version 2. Per lane, per band, 19 little-endian 32-bit words (76 words, 304 bytes per lane); the
//! common section is the shared codec's two-word header — the layout version and the data word
//! count — and nothing else, because the two channels share no state. The header makes a payload
//! self-describing, so a stale or truncated restore is rejected on the payload's own evidence and
//! not only on the caller's out-of-band `state_layout_version`. A version-1 payload is rejected with
//! `effect.state.version`; there is no silent migration.

use miso_engine_core::{SampleRateHz, is_launch_sample_rate};
use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality as Quality,
    EnumChoiceV1, InitialParameterValue, LatencySamples, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport,
    QualityDescriptorV1, ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, TailSamples, expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{check_block, nonfinite_lane_mask};
use miso_engine_effect_runtime::params::{
    ParameterSpec, normalize_zero, parameter_value_valid as domain_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload as payload;
use miso_engine_lane::kernels::{SvfCoef, SvfCoefStep, SvfState, svf_block, svf_block_ramped};
use miso_engine_lane::{Backend, Lane, Simd4, Simd8};

/// Fixed cascade length in V1.
pub const EQ_SECTION_COUNT_V1: usize = 4;

/// State payload layout version. Version 1 was the delta-word layout of issue #42.
const STATE_LAYOUT_VERSION: u32 = 2;
/// Words one band occupies in a lane section of the payload.
const STATE_WORDS_PER_BAND: usize = 19;
/// Effect-owned words in each channel section.
const STATE_LANE_WORDS: usize = EQ_SECTION_COUNT_V1 * STATE_WORDS_PER_BAND;
/// The payload shape, stamped into the common section by the shared codec.
///
/// W2-D2's rule for a crate that has to bump its layout anyway: adopt the runtime header **inside**
/// that bump, so the layout is never versioned twice. The header is two words — the version and the
/// data word count — which is what makes a payload self-describing: a stale or truncated restore is
/// rejected on the payload's own evidence rather than on the caller's word alone.
const STATE_LAYOUT: payload::StateLayout = payload::StateLayout {
    version: STATE_LAYOUT_VERSION,
    common_words: 0,
    lane_words: STATE_LANE_WORDS as u32,
};

/// Byte lengths the descriptor advertises, derived from the layout rather than written out.
const STATE_SIZES: payload::StatePayloadSizes = payload::expected_sizes(&STATE_LAYOUT);

/// Samples an automation point takes to reach its target (`SmoothingRule::Linear`, D11).
const RAMP_SAMPLES: u32 = 64;
/// `1 / RAMP_SAMPLES` as an exact power of two: the ramp multiplies, it never divides.
const RAMP_SCALE: f32 = 1.0 / RAMP_SAMPLES as f32;
/// Widest bank this crate binds; sizes the small fixed per-lane scratch arrays.
const MAX_LANES: usize = 8;

/// Frozen V1 section filter families.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqBandKindV1 {
    /// Peaking bell.
    Bell = 1,
    /// Low shelving.
    LowShelf = 2,
    /// High shelving.
    HighShelf = 3,
    /// Second-order low pass.
    LowPass = 4,
    /// Second-order high pass.
    HighPass = 5,
    /// Second-order notch.
    Notch = 6,
}

impl EqBandKindV1 {
    /// Decodes the enumeration parameter's frozen numeric encoding.
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
    /// Band index, `0..EQ_SECTION_COUNT_V1`.
    pub index: u8,
    /// Position in the cascade; equal to `index` in V1.
    pub cascade_order: u8,
    /// Boolean enable.
    pub enabled: ParameterId,
    /// Filter family, one of [`EqBandKindV1`].
    pub kind: ParameterId,
    /// Centre or corner frequency in Hz.
    pub frequency_hz: ParameterId,
    /// Bell or shelf gain in dB.
    pub gain_db: ParameterId,
    /// Quality factor.
    pub q: ParameterId,
    /// Shelf slope `S`.
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

/// The first stable parameter ID of band `band`; bands are spaced sixteen apart.
const fn band_base(band: usize) -> u32 {
    band as u32 * 16 + 1
}

/// Four static cascade positions in increasing order.
pub const EQ_BAND_DESCRIPTORS_V1: [EqBandDescriptorV1; EQ_SECTION_COUNT_V1] = {
    let mut bands = [EqBandDescriptorV1 {
        index: 0,
        cascade_order: 0,
        enabled: parameter_id(1),
        kind: parameter_id(2),
        frequency_hz: parameter_id(3),
        gain_db: parameter_id(4),
        q: parameter_id(5),
        shelf_slope: parameter_id(6),
    }; EQ_SECTION_COUNT_V1];
    let mut band = 0;
    while band < EQ_SECTION_COUNT_V1 {
        let base = band_base(band);
        bands[band] = EqBandDescriptorV1 {
            index: band as u8,
            cascade_order: band as u8,
            enabled: parameter_id(base),
            kind: parameter_id(base + 1),
            frequency_hz: parameter_id(base + 2),
            gain_db: parameter_id(base + 3),
            q: parameter_id(base + 4),
            shelf_slope: parameter_id(base + 5),
        };
        band += 1;
    }
    bands
};

const KIND_CHOICES: [EnumChoiceV1; 6] = [
    EnumChoiceV1 {
        value: 1.0,
        label: "bell",
    },
    EnumChoiceV1 {
        value: 2.0,
        label: "low-shelf",
    },
    EnumChoiceV1 {
        value: 3.0,
        label: "high-shelf",
    },
    EnumChoiceV1 {
        value: 4.0,
        label: "low-pass",
    },
    EnumChoiceV1 {
        value: 5.0,
        label: "high-pass",
    },
    EnumChoiceV1 {
        value: 6.0,
        label: "notch",
    },
];

/// Display names, band major, in descriptor order. The table below is generated from them, so the
/// twenty-four descriptors cannot drift apart in a field no reader is comparing.
const PARAMETER_NAMES: [&str; EQ_SECTION_COUNT_V1 * 6] = [
    "band-1-enabled",
    "band-1-kind",
    "band-1-frequency",
    "band-1-gain",
    "band-1-q",
    "band-1-shelf-slope",
    "band-2-enabled",
    "band-2-kind",
    "band-2-frequency",
    "band-2-gain",
    "band-2-q",
    "band-2-shelf-slope",
    "band-3-enabled",
    "band-3-kind",
    "band-3-frequency",
    "band-3-gain",
    "band-3-q",
    "band-3-shelf-slope",
    "band-4-enabled",
    "band-4-kind",
    "band-4-frequency",
    "band-4-gain",
    "band-4-q",
    "band-4-shelf-slope",
];

/// Default centre frequency of each band, in Hz.
const FREQUENCY_DEFAULTS: [f32; EQ_SECTION_COUNT_V1] = [80.0, 400.0, 2_000.0, 10_000.0];

/// Lowest and highest admitted frequency, gain, Q and shelf slope, in field order 2..6.
const NUMERIC_SPECS: [ParameterSpec; 4] = [
    ParameterSpec::logarithmic(10.0, 20_000.0, 80.0),
    ParameterSpec::continuous(-24.0, 24.0, 0.0),
    ParameterSpec::logarithmic(0.1, 18.0, core::f32::consts::FRAC_1_SQRT_2),
    ParameterSpec::continuous(0.1, 1.0, 1.0),
];

/// Per-field descriptor columns, in field order: enabled, kind, frequency, gain, Q, shelf slope.
/// Only the frequency default varies by band ([`FREQUENCY_DEFAULTS`]); everything else is shared,
/// so the twenty-four descriptors are generated from these six columns and cannot drift apart in a
/// field no reader is comparing.
const DISPLAY_UNITS: [&str; 6] = ["on/off", "type", "Hz", "dB", "Q", "S"];
const UNITS: [ParameterUnit; 6] = [
    ParameterUnit::Linear,
    ParameterUnit::Linear,
    ParameterUnit::Hz,
    ParameterUnit::Db,
    ParameterUnit::Ratio,
    ParameterUnit::Ratio,
];
const DOMAINS: [ParameterDomain; 6] = [
    ParameterDomain::Boolean,
    ParameterDomain::Enumeration,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
    ParameterDomain::Continuous,
];
const MINIMA: [Option<f32>; 6] = [None, None, Some(10.0), Some(-24.0), Some(0.1), Some(0.1)];
const MAXIMA: [Option<f32>; 6] = [
    None,
    None,
    Some(20_000.0),
    Some(24.0),
    Some(18.0),
    Some(1.0),
];
const DEFAULTS: [f32; 6] = [0.0, 1.0, 0.0, 0.0, core::f32::consts::FRAC_1_SQRT_2, 1.0];
const MAPPINGS: [ParameterMapping; 6] = [
    ParameterMapping::Stepped,
    ParameterMapping::Stepped,
    ParameterMapping::Logarithmic,
    ParameterMapping::Linear,
    ParameterMapping::Logarithmic,
    ParameterMapping::Linear,
];

/// One descriptor of one field of one band.
const fn parameter(band: usize, field: usize) -> ParameterDescriptorV1 {
    let automatable = field >= 2;
    ParameterDescriptorV1 {
        id: parameter_id(band_base(band) + field as u32),
        display_name: PARAMETER_NAMES[band * 6 + field],
        display_unit: DISPLAY_UNITS[field],
        unit: UNITS[field],
        domain: DOMAINS[field],
        minimum: MINIMA[field],
        maximum: MAXIMA[field],
        default_value: if field == 2 {
            FREQUENCY_DEFAULTS[band]
        } else {
            DEFAULTS[field]
        },
        mapping: MAPPINGS[field],
        automation_rate: if automatable {
            AutomationRate::Block
        } else {
            AutomationRate::None
        },
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: if automatable {
            SmoothingRule::Linear
        } else {
            SmoothingRule::None
        },
        smoothing_samples: if automatable { RAMP_SAMPLES } else { 0 },
        readable: true,
        automatable,
        enum_choices: if field == 1 { &KIND_CHOICES } else { &[] },
    }
}

const EQ_PARAMETERS: [ParameterDescriptorV1; EQ_SECTION_COUNT_V1 * 6] = {
    let mut table = [parameter(0, 0); EQ_SECTION_COUNT_V1 * 6];
    let mut band = 0;
    while band < EQ_SECTION_COUNT_V1 {
        let mut field = 0;
        while field < 6 {
            table[band * 6 + field] = parameter(band, field);
            field += 1;
        }
        band += 1;
    }
    table
};

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

const QUALITIES: [QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

const fn quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: Quality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: STATE_SIZES.common as u32,
            left_bytes: STATE_SIZES.left as u32,
            right_bytes: STATE_SIZES.right as u32,
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
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::DUAL_MONO,
    parameters: &EQ_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

/// The six retained `f32` words of one TPT state-variable section.
///
/// Designed in `f64` and rounded exactly once, which is what makes the words a target-independent
/// function of the parameters (the `f64` design uses `miso-engine-math`, never the platform libm).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqSvfWordsV1 {
    /// `t / (1 + t)` with `t = g * (g + k)` — the pole's distance from `z = 1` (amendment A1).
    pub c1: f32,
    /// `g * (1 - c1)`.
    pub a2: f32,
    /// `g * a2`.
    pub a3: f32,
    /// Direct output mix.
    pub m0: f32,
    /// Band output mix.
    pub m1: f32,
    /// Low output mix.
    pub m2: f32,
}

impl EqSvfWordsV1 {
    /// The exact identity section: `y = x` bit for bit, with no state growth and no mask.
    pub const IDENTITY: Self = Self {
        c1: 0.0,
        a2: 0.0,
        a3: 0.0,
        m0: 1.0,
        m1: 0.0,
        m2: 0.0,
    };

    /// The words in the pinned order `c1, a2, a3, m0, m1, m2`.
    #[must_use]
    pub const fn to_array(self) -> [f32; 6] {
        [self.c1, self.a2, self.a3, self.m0, self.m1, self.m2]
    }

    /// Rebuilds a set from [`EqSvfWordsV1::to_array`] order.
    #[must_use]
    pub const fn from_array(words: [f32; 6]) -> Self {
        Self {
            c1: words[0],
            a2: words[1],
            a3: words[2],
            m0: words[3],
            m1: words[4],
            m2: words[5],
        }
    }
}

/// Why a section could not be designed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqDesignError {
    /// A parameter, the sample rate, or their combination is outside the frozen domain.
    InvalidInput,
    /// The rounded words are not a stable, finite realization.
    Coefficients,
}

/// The `f64` mapping, before the single rounding. Frozen operation order.
///
/// Simper 2013: `g = tan(pi f0 / fs)` prewarped per kind, `k` the damping word, and the output mix
/// `(m0, m1, m2)` that selects the response. The shelf damping is RBJ's `1/Q_S`, which is what makes
/// `alpha_S` and `alpha_Q` the same quantity and the shelf transfer the cookbook's.
///
/// Returned in the pinned order `c1, a2, a3, m0, m1, m2`. Public because it is the quantity an
/// oracle compares against: [`design_svf_v1`] is exactly this followed by one rounding.
#[must_use]
pub fn design_svf_words_f64(
    kind: EqBandKindV1,
    frequency_hz: f64,
    gain_db: f64,
    q: f64,
    shelf_slope: f64,
    sample_rate_hz: f64,
) -> [f64; 6] {
    let amplitude = miso_engine_math::pow(10.0, gain_db / 40.0);
    let warped = miso_engine_math::tan(core::f64::consts::PI * frequency_hz / sample_rate_hz);
    let shelf_k = ((amplitude + 1.0 / amplitude) * (1.0 / shelf_slope - 1.0) + 2.0).sqrt();
    let (g, k, m0, m1, m2) = match kind {
        EqBandKindV1::LowPass => (warped, 1.0 / q, 0.0, 0.0, 1.0),
        EqBandKindV1::HighPass => (warped, 1.0 / q, 1.0, -(1.0 / q), -1.0),
        EqBandKindV1::Notch => (warped, 1.0 / q, 1.0, -(1.0 / q), 0.0),
        EqBandKindV1::Bell => {
            let k = 1.0 / (q * amplitude);
            (warped, k, 1.0, k * (amplitude * amplitude - 1.0), 0.0)
        }
        EqBandKindV1::LowShelf => (
            warped / amplitude.sqrt(),
            shelf_k,
            1.0,
            shelf_k * (amplitude - 1.0),
            amplitude * amplitude - 1.0,
        ),
        EqBandKindV1::HighShelf => (
            warped * amplitude.sqrt(),
            shelf_k,
            amplitude * amplitude,
            shelf_k * (1.0 - amplitude) * amplitude,
            1.0 - amplitude * amplitude,
        ),
    };
    let t = g * (g + k);
    let c1 = t / (1.0 + t);
    let a1 = 1.0 - c1;
    let a2 = g * a1;
    let a3 = g * a2;
    [c1, a2, a3, m0, m1, m2]
}

/// Spectral norm of the zero-input state transition `M = [[1-2c1, -2a2], [2a2, 1-2a3]]`, in `f64`.
///
/// A linear ramp between two word triples is safe exactly when every point of it is contractive,
/// and `‖M‖₂` is convex in the words, so checking the endpoints checks the whole ramp. This is the
/// stability predicate that replaces the delta realization's Jury test: it is a statement about the
/// matrix the kernel actually iterates, not about a polynomial nothing evaluates.
#[must_use]
pub fn word_spectral_norm(words: EqSvfWordsV1) -> f64 {
    let a00 = 1.0 - 2.0 * f64::from(words.c1);
    let a01 = -2.0 * f64::from(words.a2);
    let a10 = 2.0 * f64::from(words.a2);
    let a11 = 1.0 - 2.0 * f64::from(words.a3);
    let first = a00 * a00 + a10 * a10;
    let second = a01 * a01 + a11 * a11;
    let cross = a00 * a01 + a10 * a11;
    let difference = first - second;
    let largest = 0.5 * (first + second + (difference * difference + 4.0 * cross * cross).sqrt());
    largest.sqrt()
}

/// Largest spectral norm accepted from a rounded word set: one `f32` rounding above contractive.
const NORM_TOLERANCE: f64 = 1.0 + 1.0 / 4_194_304.0;

/// Designs one section's six `f32` words from the frozen RBJ parameter domain.
///
/// # Errors
///
/// [`EqDesignError::InvalidInput`] if the sample rate is not a launch rate, a parameter is outside
/// its domain, or the centre frequency is at or above Nyquist. [`EqDesignError::Coefficients`] if
/// the rounded words are not finite or not contractive.
pub fn design_svf_v1(
    kind: EqBandKindV1,
    frequency_hz: f32,
    gain_db: f32,
    q: f32,
    shelf_slope: f32,
    sample_rate: SampleRateHz,
) -> Result<EqSvfWordsV1, EqDesignError> {
    if !is_launch_sample_rate(sample_rate)
        || !numeric_value_valid(0, frequency_hz)
        || !numeric_value_valid(1, gain_db)
        || !numeric_value_valid(2, q)
        || !numeric_value_valid(3, shelf_slope)
        || frequency_hz >= sample_rate.0 as f32 * 0.5
    {
        return Err(EqDesignError::InvalidInput);
    }
    let exact = design_svf_words_f64(
        kind,
        f64::from(frequency_hz),
        f64::from(gain_db),
        f64::from(q),
        f64::from(shelf_slope),
        f64::from(sample_rate.0),
    );
    // The single rounding. `-0.0` is normalised away so that adding a zero ramp increment to a word
    // is bit-preserving on every lane, which is what makes an idle lane of a ramping bank identical
    // to the same lane of a settled one.
    let words = EqSvfWordsV1::from_array(exact.map(|value| {
        let rounded = value as f32;
        if rounded == 0.0 { 0.0 } else { rounded }
    }));
    if !words.to_array().into_iter().all(f32::is_finite)
        || !(0.0..1.0).contains(&words.c1)
        || words.a2 <= 0.0
        || words.a3 < 0.0
        || word_spectral_norm(words) > NORM_TOLERANCE
    {
        return Err(EqDesignError::Coefficients);
    }
    Ok(words)
}

/// `true` if `value` is inside the domain of numeric field `field` (0 frequency, 1 gain, 2 Q,
/// 3 shelf slope), using the shared descriptor validator rather than a fourth copy of the ranges.
fn numeric_value_valid(field: usize, value: f32) -> bool {
    domain_valid(&NUMERIC_SPECS[field], value)
}

/// The control-plane parameter state of one band of one track.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BandTarget {
    enabled: bool,
    kind: EqBandKindV1,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
}

impl BandTarget {
    /// The words this band settles at. A disabled band is the exact identity at every parameter.
    fn words(&self, sample_rate: SampleRateHz) -> Result<EqSvfWordsV1, EqDesignError> {
        if !self.enabled {
            return Ok(EqSvfWordsV1::IDENTITY);
        }
        design_svf_v1(
            self.kind,
            self.frequency,
            self.gain,
            self.q,
            self.slope,
            sample_rate,
        )
    }

    /// The four automatable fields, in payload order.
    const fn numeric(&self) -> [f32; 4] {
        [self.frequency, self.gain, self.q, self.slope]
    }

    /// Writes automatable field `field`.
    fn set_numeric(&mut self, field: usize, value: f32) {
        match field {
            0 => self.frequency = value,
            1 => self.gain = value,
            2 => self.q = value,
            _ => self.slope = value,
        }
    }
}

/// One cascade section of one channel, across `L::WIDTH` tracks.
#[derive(Clone, Copy)]
struct Section<L: Lane> {
    /// Words the most recently processed frame used.
    coef: SvfCoef<L>,
    /// Per-sample increment; exactly zero on every settled lane.
    step: SvfCoefStep<L>,
    /// Words the ramp is heading for, assigned exactly on its final sample.
    target: SvfCoef<L>,
    /// The two integrator words.
    state: SvfState<L>,
}

/// Reads word `index` of a coefficient set in the pinned order.
fn coef_word<L: Lane>(coefficients: &SvfCoef<L>, index: usize) -> L {
    match index {
        0 => coefficients.c1,
        1 => coefficients.a2,
        2 => coefficients.a3,
        3 => coefficients.m0,
        4 => coefficients.m1,
        _ => coefficients.m2,
    }
}

/// Writes word `index` of a coefficient set in the pinned order.
fn coef_word_mut<L: Lane>(coefficients: &mut SvfCoef<L>, index: usize) -> &mut L {
    match index {
        0 => &mut coefficients.c1,
        1 => &mut coefficients.a2,
        2 => &mut coefficients.a3,
        3 => &mut coefficients.m0,
        4 => &mut coefficients.m1,
        _ => &mut coefficients.m2,
    }
}

/// Writes increment `index` of a step set in the pinned order.
fn step_word_mut<L: Lane>(step: &mut SvfCoefStep<L>, index: usize) -> &mut L {
    match index {
        0 => &mut step.c1,
        1 => &mut step.a2,
        2 => &mut step.a3,
        3 => &mut step.m0,
        4 => &mut step.m1,
        _ => &mut step.m2,
    }
}

/// Reads lane `lane` of a vector. Control plane only: it leaves the vector domain.
fn lane_get<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words = [0.0_f32; MAX_LANES];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

/// Writes lane `lane` of a vector. Control plane only.
fn lane_set<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words = [0.0_f32; MAX_LANES];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// One channel — left or right — of a cascade over `W = L::WIDTH` tracks.
///
/// This is the only realization in the crate: the scalar effect is the `L = f32`, `W = 1`
/// instantiation of the same body, because a planar block is already a one-lane AoSoA block. There
/// is no second copy of the recurrence, of the ramp law, or of the reset rules to keep in step.
struct Channel<L: Lane, const W: usize> {
    sections: [Section<L>; EQ_SECTION_COUNT_V1],
    /// Samples still to be produced before each lane's ramp is at its target.
    remaining: [[u32; W]; EQ_SECTION_COUNT_V1],
    /// Live parameter targets, per track.
    targets: [[BandTarget; EQ_SECTION_COUNT_V1]; W],
}

impl<L: Lane, const W: usize> Channel<L, W> {
    /// Builds a settled channel from per-track band parameters.
    fn new(
        targets: [[BandTarget; EQ_SECTION_COUNT_V1]; W],
        sample_rate: SampleRateHz,
    ) -> Result<Self, EqDesignError> {
        let identity = SvfCoef {
            c1: L::zero(),
            a2: L::zero(),
            a3: L::zero(),
            m0: L::splat(1.0),
            m1: L::zero(),
            m2: L::zero(),
        };
        let mut channel = Self {
            sections: [Section {
                coef: identity,
                step: SvfCoefStep::default(),
                target: identity,
                state: SvfState::default(),
            }; EQ_SECTION_COUNT_V1],
            remaining: [[0; W]; EQ_SECTION_COUNT_V1],
            targets,
        };
        for (track, bands) in targets.iter().enumerate() {
            for (section, band) in bands.iter().enumerate() {
                let words = band.words(sample_rate)?;
                channel.settle(section, track, words);
            }
        }
        Ok(channel)
    }

    /// Places lane `track` of `section` at `words` with no ramp in flight.
    fn settle(&mut self, section: usize, track: usize, words: EqSvfWordsV1) {
        let slot = &mut self.sections[section];
        for (index, word) in words.to_array().into_iter().enumerate() {
            lane_set(coef_word_mut(&mut slot.coef, index), track, word);
            lane_set(coef_word_mut(&mut slot.target, index), track, word);
            lane_set(step_word_mut(&mut slot.step, index), track, 0.0);
        }
        self.remaining[section][track] = 0;
    }

    /// Starts a [`RAMP_SAMPLES`]-sample word ramp on lane `track` of `section` (D11).
    ///
    /// The increment is one multiply by an exact power of two, computed once here; a ramp that is
    /// re-targeted mid-flight starts from the words in force now, not from the old target.
    ///
    /// # The stationary hoist (issue #144 item 6)
    ///
    /// When all six words this lane is being sent to are already the six words it holds, every
    /// increment is `(word - word) * RAMP_SCALE`, which is exactly `+0.0` for finite words. The
    /// ramp would then spend [`RAMP_SAMPLES`] samples adding zero to a lane that is already where
    /// it is being sent. That costs far more than the lane: `process_section` takes its ramping
    /// decision across **all** `W` lanes of the section, so one lane's no-op window drags the
    /// whole bank onto `svf_block_ramped` -- six vector additions and a negate per frame -- for
    /// sixty-four samples. A console that re-sends a band it did not move (an automation refresh,
    /// a touched-but-unmoved control) pays that on every refresh.
    ///
    /// [`LinearRamp::stationary_at`] decides it by bit compare. The lane is settled instead, which
    /// is bit-identical because a zero increment is bit-preserving on every word -- exactly the
    /// property `design_svf_v1` normalises `-0.0` away to guarantee, and the same property that
    /// makes an idle lane of a ramping bank identical to the same lane of a settled one.
    fn start_ramp(&mut self, section: usize, track: usize, words: EqSvfWordsV1) {
        if self.stationary_at(section, track, words) {
            self.settle(section, track, words);
            return;
        }
        let slot = &mut self.sections[section];
        for (index, word) in words.to_array().into_iter().enumerate() {
            let current = lane_get(coef_word(&slot.coef, index), track);
            lane_set(coef_word_mut(&mut slot.target, index), track, word);
            lane_set(
                step_word_mut(&mut slot.step, index),
                track,
                (word - current) * RAMP_SCALE,
            );
        }
        self.remaining[section][track] = RAMP_SAMPLES;
    }

    /// `true` when lane `track` of `section` already holds exactly `words`.
    ///
    /// All six words must agree bitwise, and each must pass [`LinearRamp::stationary_at`], which
    /// is where the `-0.0` and non-finite exclusions live. A partial match is not a hoist: five
    /// settled words and one moving word is a ramp.
    fn stationary_at(&self, section: usize, track: usize, words: EqSvfWordsV1) -> bool {
        let slot = &self.sections[section];
        words
            .to_array()
            .into_iter()
            .enumerate()
            .all(|(index, word)| {
                LinearRamp::stationary_at(lane_get(coef_word(&slot.coef, index), track), word)
            })
    }

    /// Assigns the target exactly and stops lane `track`'s ramp — the D11 snap.
    fn snap(&mut self, section: usize, track: usize) {
        let slot = &mut self.sections[section];
        for index in 0..6 {
            let target = lane_get(coef_word(&slot.target, index), track);
            lane_set(coef_word_mut(&mut slot.coef, index), track, target);
            lane_set(step_word_mut(&mut slot.step, index), track, 0.0);
        }
        self.remaining[section][track] = 0;
    }

    /// Runs the four sections over one block in place.
    fn process_block(&mut self, io: &mut [f32], frames: usize) {
        for section in 0..EQ_SECTION_COUNT_V1 {
            self.process_section(section, io, frames);
        }
    }

    /// Runs one section over one block, splitting it where a lane's ramp ends.
    ///
    /// The block is cut at every distinct ramp end, so within a segment every ramping lane steps on
    /// every frame and every settled lane has a zero increment. The cut is control-plane work done
    /// once per section per block; the frames themselves never branch.
    fn process_section(&mut self, section: usize, io: &mut [f32], frames: usize) {
        let mut position = 0;
        while position < frames {
            for track in 0..W {
                if self.remaining[section][track] == 1 {
                    self.snap(section, track);
                }
            }
            let mut length = frames - position;
            let mut ramping = false;
            for track in 0..W {
                let remaining = self.remaining[section][track];
                if remaining > 0 {
                    ramping = true;
                    length = length.min(remaining as usize - 1);
                }
            }
            debug_assert!(length > 0);
            let slot = &mut self.sections[section];
            let block = &mut io[position * W..(position + length) * W];
            if ramping {
                advance_words(&mut slot.coef, &slot.step);
                svf_block_ramped::<L>(
                    block,
                    length,
                    &mut slot.coef,
                    &slot.step,
                    length - 1,
                    &mut slot.state,
                );
            } else {
                svf_block::<L>(block, length, &slot.coef, &mut slot.state);
            }
            for track in 0..W {
                let remaining = &mut self.remaining[section][track];
                *remaining = remaining.saturating_sub(length as u32);
            }
            position += length;
        }
    }

    /// Clears every integrator word, leaving coefficients and ramps alone.
    fn reset_states(&mut self) {
        for section in &mut self.sections {
            section.state = SvfState::default();
        }
    }

    /// Ends every ramp at its target and clears the integrators (a seek or a transport stop).
    fn discontinuity_reset(&mut self) {
        self.reset_states();
        for section in 0..EQ_SECTION_COUNT_V1 {
            for track in 0..W {
                self.snap(section, track);
            }
        }
    }
}

/// `coefficients += step`, one exact vector addition per word.
fn advance_words<L: Lane>(coefficients: &mut SvfCoef<L>, step: &SvfCoefStep<L>) {
    coefficients.c1 = coefficients.c1.add(step.c1);
    coefficients.a2 = coefficients.a2.add(step.a2);
    coefficients.a3 = coefficients.a3.add(step.a3);
    coefficients.m0 = coefficients.m0.add(step.m0);
    coefficients.m1 = coefficients.m1.add(step.m1);
    coefficients.m2 = coefficients.m2.add(step.m2);
}

/// Reads increment `index` of a step set in the pinned order.
fn step_word<L: Lane>(step: &SvfCoefStep<L>, index: usize) -> L {
    match index {
        0 => step.c1,
        1 => step.a2,
        2 => step.a3,
        3 => step.m0,
        4 => step.m1,
        _ => step.m2,
    }
}

/// One band's decoded payload words, held until every band of the lane has validated.
#[derive(Clone, Copy)]
struct RestoredBand {
    integrators: [f32; 2],
    coefficients: [f32; 6],
    step: [f32; 6],
    remaining: u32,
    target: BandTarget,
}

impl<L: Lane, const W: usize> Channel<L, W> {
    /// Writes lane `track`'s state words in the version-2 order.
    fn snapshot_track(&self, track: usize, out: &mut [u32; STATE_LANE_WORDS]) {
        for section in 0..EQ_SECTION_COUNT_V1 {
            let base = section * STATE_WORDS_PER_BAND;
            let slot = &self.sections[section];
            out[base] = lane_get(slot.state.ic1, track).to_bits();
            out[base + 1] = lane_get(slot.state.ic2, track).to_bits();
            for index in 0..6 {
                out[base + 2 + index] = lane_get(coef_word(&slot.coef, index), track).to_bits();
                out[base + 8 + index] = lane_get(step_word(&slot.step, index), track).to_bits();
            }
            out[base + 14] = self.remaining[section][track];
            for (index, value) in self.targets[track][section]
                .numeric()
                .into_iter()
                .enumerate()
            {
                out[base + 15 + index] = value.to_bits();
            }
        }
    }

    /// Validates lane `track`'s state words and applies them all or none.
    ///
    /// `configuration` supplies the two parameters that are not automatable and therefore not in
    /// the payload — the band's enable and family. The ramp target words are **recomputed** from
    /// the stored parameters rather than stored, so a payload cannot carry words that disagree with
    /// the parameters it also carries.
    fn restore_track(
        &mut self,
        track: usize,
        words: &[u32; STATE_LANE_WORDS],
        configuration: &[BandTarget; EQ_SECTION_COUNT_V1],
        sample_rate: SampleRateHz,
    ) -> Result<(), StatePayloadError> {
        let invalid = StatePayloadError {
            code: "effect.state.payload",
        };
        let mut decoded = [RestoredBand {
            integrators: [0.0; 2],
            coefficients: [0.0; 6],
            step: [0.0; 6],
            remaining: 0,
            target: configuration[0],
        }; EQ_SECTION_COUNT_V1];
        for section in 0..EQ_SECTION_COUNT_V1 {
            let base = section * STATE_WORDS_PER_BAND;
            let read = |offset: usize| f32::from_bits(words[base + offset]);
            let band = RestoredBand {
                integrators: [read(0), read(1)],
                coefficients: core::array::from_fn(|index| read(2 + index)),
                step: core::array::from_fn(|index| read(8 + index)),
                remaining: words[base + 14],
                target: {
                    let mut target = configuration[section];
                    for index in 0..4 {
                        target.set_numeric(index, normalize_zero(read(15 + index)));
                    }
                    target
                },
            };
            let numeric = band.target.numeric();
            if !band.integrators.into_iter().all(f32::is_finite)
                || !band.coefficients.into_iter().all(f32::is_finite)
                || !band.step.into_iter().all(f32::is_finite)
                || band.remaining > RAMP_SAMPLES
                || !(0..4).all(|index| numeric_value_valid(index, numeric[index]))
            {
                return Err(invalid);
            }
            let target_words = band.target.words(sample_rate).map_err(|_| invalid)?;
            if band.remaining == 0
                && band
                    .coefficients
                    .iter()
                    .zip(target_words.to_array())
                    .any(|(stored, expected)| stored.to_bits() != expected.to_bits())
            {
                return Err(invalid);
            }
            decoded[section] = band;
        }
        for (section, band) in decoded.into_iter().enumerate() {
            let target_words = band
                .target
                .words(sample_rate)
                .unwrap_or(EqSvfWordsV1::IDENTITY);
            let slot = &mut self.sections[section];
            lane_set(&mut slot.state.ic1, track, band.integrators[0]);
            lane_set(&mut slot.state.ic2, track, band.integrators[1]);
            for index in 0..6 {
                lane_set(
                    coef_word_mut(&mut slot.coef, index),
                    track,
                    band.coefficients[index],
                );
                lane_set(
                    step_word_mut(&mut slot.step, index),
                    track,
                    band.step[index],
                );
                lane_set(
                    coef_word_mut(&mut slot.target, index),
                    track,
                    target_words.to_array()[index],
                );
            }
            self.remaining[section][track] = band.remaining;
            self.targets[track][section] = band.target;
        }
        Ok(())
    }
}

/// Stateless native factory for prepared parametric EQs.
#[derive(Clone, Copy, Debug, Default)]
pub struct ParametricEqFactory;

/// A prepared EQ over `W = L::WIDTH` tracks: one body for the scalar effect and for every bank.
struct PreparedParametricEq<L: Lane, const W: usize> {
    metadata: PreparedEffectMetadata,
    bank: PreparedBankMetadata,
    initial: [[[BandTarget; EQ_SECTION_COUNT_V1]; 2]; W],
    left: Channel<L, W>,
    right: Channel<L, W>,
}

impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    /// The channel's sample rate, as the design functions take it.
    fn sample_rate(&self) -> SampleRateHz {
        SampleRateHz(self.metadata.sample_rate)
    }

    /// Applies one track's automation spans to both channels.
    ///
    /// The rules are unchanged from the frozen contract: a span must be a `Point` at exactly this
    /// block's first sample with identical endpoints and an in-domain value, spans must arrive in
    /// non-decreasing `(sample, parameter, channel)` order, and one slot may be written once. Every
    /// malformed span is counted once and none of them discards a valid point.
    fn automate(
        &mut self,
        spans: &[PreparedAutomationSpan],
        first_sample: u64,
        track: usize,
        invalid_spans: &mut u64,
    ) {
        if spans.len() > self.metadata.automation_capacity as usize {
            *invalid_spans = invalid_spans.saturating_add(spans.len() as u64);
            return;
        }
        let mut pending = [None; EQ_SECTION_COUNT_V1 * 4 * 2];
        let mut prior_sort_key = None;
        for span in spans {
            let sort_key = (span.start_sample, span.parameter_index, span.channel);
            let slot = numeric_parameter(span.parameter_index as usize)
                .zip(lane_index(span.channel))
                .map(|((section, field), channel)| (section * 4 + field) * 2 + channel);
            let Some(slot) = slot else {
                *invalid_spans = invalid_spans.saturating_add(1);
                continue;
            };
            if prior_sort_key.is_some_and(|prior| sort_key < prior)
                || pending[slot].is_some()
                || span.kind != AutomationSpanKind::Point
                || span.start_sample != first_sample
                || span.end_sample != first_sample
                || span.start_value.to_bits() != span.end_value.to_bits()
                || !numeric_value_valid(slot / 2 % 4, span.start_value)
            {
                *invalid_spans = invalid_spans.saturating_add(1);
                continue;
            }
            // Master plan §6 / 83c decision 3: `-0.0` is a way of writing zero, not an error. It is
            // normalised here, on the way in, so no coefficient design and no payload ever sees it.
            pending[slot] = Some(normalize_zero(span.start_value));
            prior_sort_key = Some(sort_key);
        }
        let sample_rate = self.sample_rate();
        for section in 0..EQ_SECTION_COUNT_V1 {
            for channel in 0..2 {
                let updates: [Option<f32>; 4] =
                    core::array::from_fn(|field| pending[(section * 4 + field) * 2 + channel]);
                if updates.iter().all(Option::is_none) {
                    continue;
                }
                let target = if channel == 0 {
                    &mut self.left.targets[track][section]
                } else {
                    &mut self.right.targets[track][section]
                };
                let mut candidate = *target;
                for (field, value) in updates.into_iter().enumerate() {
                    if let Some(value) = value {
                        candidate.set_numeric(field, value);
                    }
                }
                match candidate.words(sample_rate) {
                    Ok(words) => {
                        *target = candidate;
                        if channel == 0 {
                            self.left.start_ramp(section, track, words);
                        } else {
                            self.right.start_ramp(section, track, words);
                        }
                    }
                    Err(_) => {
                        // Unreachable for in-domain values; the branch keeps the validator total.
                        *invalid_spans = invalid_spans.saturating_add(1);
                    }
                }
            }
        }
    }

    /// Runs both channels over one block and applies the master plan §4.4 boundary check.
    ///
    /// The check is one vector scan per channel per block, from `miso-engine-effect-runtime`. A
    /// rejected block is zeroed and the channel's integrators are cleared; coefficients and ramps
    /// survive, because a non-finite block is a fault report, not an automation event. The two
    /// channels are judged independently: they carry independent state and independent counters,
    /// which is what dual-mono means here.
    fn render(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) -> [[bool; MAX_LANES]; 2] {
        let mut failures = [[false; MAX_LANES]; 2];
        self.left.process_block(left, frames);
        self.right.process_block(right, frames);
        for (index, (channel, block)) in
            [(&mut self.left, &mut *left), (&mut self.right, &mut *right)]
                .into_iter()
                .enumerate()
        {
            if check_block::<L>(block) {
                continue;
            }
            let mask = nonfinite_lane_mask::<L>(block);
            block.fill(0.0);
            channel.reset_states();
            for (lane, failed) in failures[index].iter_mut().enumerate().take(W) {
                *failed = mask & (1 << lane) != 0;
            }
        }
        failures
    }

    /// Restores every band of both channels to the parameters preparation was given.
    fn reset_to_defaults(&mut self) -> Result<(), EqDesignError> {
        let sample_rate = self.sample_rate();
        let left = core::array::from_fn(|track| self.initial[track][0]);
        let right = core::array::from_fn(|track| self.initial[track][1]);
        self.left = Channel::new(left, sample_rate)?;
        self.right = Channel::new(right, sample_rate)?;
        Ok(())
    }
}

/// Returns the cascade section and the automatable field index (0 frequency .. 3 shelf slope).
fn numeric_parameter(parameter_index: usize) -> Option<(usize, usize)> {
    let section = parameter_index / 6;
    if section >= EQ_SECTION_COUNT_V1 || parameter_index % 6 < 2 {
        return None;
    }
    Some((section, parameter_index % 6 - 2))
}

/// Channel index of a per-lane parameter span; `Both` is not a per-lane address.
fn lane_index(channel: ParameterChannel) -> Option<usize> {
    match channel {
        ParameterChannel::Left => Some(0),
        ParameterChannel::Right => Some(1),
        ParameterChannel::Both => None,
    }
}

/// Reads one track's four bands out of a validated initial-value list.
///
/// The contract validates that the list is exactly one value per parameter per channel, in
/// descriptor order, before this runs, so the index arithmetic needs no search and no allocation.
fn band_targets(
    values: &[InitialParameterValue],
    channel: usize,
    sample_rate: SampleRateHz,
) -> Result<[BandTarget; EQ_SECTION_COUNT_V1], EffectPrepareError> {
    let mut bands = [BandTarget {
        enabled: false,
        kind: EqBandKindV1::Bell,
        frequency: 0.0,
        gain: 0.0,
        q: 0.0,
        slope: 0.0,
    }; EQ_SECTION_COUNT_V1];
    for (section, band) in bands.iter_mut().enumerate() {
        let field = |index: usize| values[(section * 6 + index) * 2 + channel].value;
        *band = BandTarget {
            enabled: field(0).to_bits() == 1.0_f32.to_bits(),
            kind: EqBandKindV1::from_value(field(1)).ok_or(EffectPrepareError {
                code: "effect.parameter.initial",
            })?,
            frequency: field(2),
            gain: field(3),
            q: field(4),
            slope: field(5),
        };
        band.words(sample_rate).map_err(|_| EffectPrepareError {
            code: "effect.eq.coefficients",
        })?;
    }
    Ok(bands)
}

/// Builds a prepared EQ of width `W` from one request per track.
fn prepare_width<L: Lane, const W: usize>(
    metadata: PreparedEffectMetadata,
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
) -> Result<PreparedParametricEq<L, W>, EffectPrepareError> {
    debug_assert_eq!(L::WIDTH, W);
    let sample_rate = SampleRateHz(metadata.sample_rate);
    let mut initial = [[[BandTarget {
        enabled: false,
        kind: EqBandKindV1::Bell,
        frequency: 0.0,
        gain: 0.0,
        q: 0.0,
        slope: 0.0,
    }; EQ_SECTION_COUNT_V1]; 2]; W];
    for (track, request) in requests.iter().enumerate() {
        initial[track][0] = band_targets(request.initial_values, 0, sample_rate)?;
        initial[track][1] = band_targets(request.initial_values, 1, sample_rate)?;
    }
    let coefficients = |_| EffectPrepareError {
        code: "effect.eq.coefficients",
    };
    Ok(PreparedParametricEq {
        metadata,
        bank: PreparedBankMetadata {
            width,
            program_key: metadata.program_key(),
        },
        initial,
        left: Channel::new(core::array::from_fn(|track| initial[track][0]), sample_rate)
            .map_err(coefficients)?,
        right: Channel::new(core::array::from_fn(|track| initial[track][1]), sample_rate)
            .map_err(coefficients)?,
    })
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
        Ok(Box::new(prepare_width::<f32, 1>(
            metadata,
            BankWidth::Four,
            &[request],
        )?))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        // Issue #95: a self-contradicting shape is a contract violation and a typed error; a
        // width this build does not execute is a capability gap and a legal `Ok(None)`. This
        // crate used to answer `Ok(None)` to both, which was the other half of the wave-2
        // divergence (`NativeEffectFactory::bind_homogeneous_bank` states the frozen rule).
        request.validate_shape()?;
        let lanes = request.width.lanes() as usize;
        if lanes != Backend::current().width() {
            return Ok(None);
        }
        let first = request
            .requests
            .first()
            .copied()
            .ok_or(EffectPrepareError {
                code: "effect.bank.requests",
            })?;
        let metadata = expected_prepared_metadata(self.descriptor(), first)?;
        for item in request.requests.iter().copied() {
            let candidate = expected_prepared_metadata(self.descriptor(), item)?;
            if candidate.program_key() != metadata.program_key() {
                return Ok(None);
            }
        }
        Ok(Some(match request.width {
            BankWidth::Four => Box::new(prepare_width::<Simd4, 4>(
                metadata,
                request.width,
                request.requests,
            )?) as Box<dyn PreparedNativeEffectBank>,
            BankWidth::Eight => Box::new(prepare_width::<Simd8, 8>(
                metadata,
                request.width,
                request.requests,
            )?) as Box<dyn PreparedNativeEffectBank>,
        }))
    }
}

/// Maps the shared codec's error onto the contract's.
fn runtime_state_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

/// Reads a whole payload — header, then both lane sections — rejecting before anything is decoded.
fn read_payload(
    input: StatePayloadInput<'_>,
) -> Result<([u32; STATE_LANE_WORDS], [u32; STATE_LANE_WORDS]), StatePayloadError> {
    let mut left = [0_u32; STATE_LANE_WORDS];
    let mut right = [0_u32; STATE_LANE_WORDS];
    payload::restore(
        &STATE_LAYOUT,
        &payload::StatePayloadInput {
            common: input.common,
            left: input.left,
            right: input.right,
        },
        &mut payload::StateWordsMut {
            common: &mut [],
            left: &mut left,
            right: &mut right,
        },
    )
    .map_err(runtime_state_error)?;
    Ok((left, right))
}

/// Writes the header and one lane pair into a payload.
fn write_payload(
    output: StatePayloadOutput<'_>,
    left: &[u32; STATE_LANE_WORDS],
    right: &[u32; STATE_LANE_WORDS],
) -> Result<(), StatePayloadError> {
    payload::snapshot(
        &STATE_LAYOUT,
        &payload::StateWords {
            common: &[],
            left,
            right,
        },
        &mut payload::StatePayloadOutput {
            common: output.common,
            left: output.left,
            right: output.right,
        },
    )
    .map_err(runtime_state_error)
}

impl<L: Lane, const W: usize> PreparedParametricEq<L, W> {
    /// Snapshots one track, shared by the scalar and bank surfaces.
    fn snapshot_track(
        &self,
        track: usize,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let mut left = [0_u32; STATE_LANE_WORDS];
        let mut right = [0_u32; STATE_LANE_WORDS];
        self.left.snapshot_track(track, &mut left);
        self.right.snapshot_track(track, &mut right);
        write_payload(output, &left, &right)
    }

    /// Restores one track, all or none across both channels.
    fn restore_track(
        &mut self,
        track: usize,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if version != STATE_LAYOUT_VERSION {
            return Err(StatePayloadError {
                code: payload::STATE_VERSION_CODE,
            });
        }
        let (left, right) = read_payload(input)?;
        let sample_rate = self.sample_rate();
        let mut candidate_left = Channel::<L, W>::new(
            core::array::from_fn(|index| self.left.targets[index]),
            sample_rate,
        )
        .map_err(|_| StatePayloadError {
            code: "effect.state.payload",
        })?;
        candidate_left.restore_track(track, &left, &self.initial[track][0], sample_rate)?;
        let mut candidate_right = Channel::<L, W>::new(
            core::array::from_fn(|index| self.right.targets[index]),
            sample_rate,
        )
        .map_err(|_| StatePayloadError {
            code: "effect.state.payload",
        })?;
        candidate_right.restore_track(track, &right, &self.initial[track][1], sample_rate)?;
        self.left
            .restore_track(track, &left, &self.initial[track][0], sample_rate)?;
        self.right
            .restore_track(track, &right, &self.initial[track][1], sample_rate)?;
        Ok(())
    }
}

impl PreparedNativeEffect for PreparedParametricEq<f32, 1> {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                let _ = self.reset_to_defaults();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        self.automate(
            block.automation,
            block.first_sample,
            0,
            &mut report.invalid_spans,
        );
        if self.metadata.bypass {
            return report;
        }
        let frames = block.left.len();
        let failures = self.render(block.left, block.right, frames);
        report.nonfinite_left_blocks = u64::from(failures[0][0]);
        report.nonfinite_right_blocks = u64::from(failures[1][0]);
        report
    }

    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_track(0, output)
    }

    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore_track(0, version, input)
    }
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedParametricEq<L, W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.bank.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        match kind {
            ResetKind::FullToDefaults => {
                let _ = self.reset_to_defaults();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.discontinuity_reset();
                self.right.discontinuity_reset();
            }
        }
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.bank.width);
        if !bank_block_matches(&block, self.bank.width, self.metadata.quantum)
            || self.bank.width.lanes() as usize != W
        {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            self.automate(
                &block.automation[start..end],
                block.first_sample,
                track,
                &mut report.reports[track].invalid_spans,
            );
        }
        if self.metadata.bypass {
            return report;
        }
        let frames = block.frames as usize;
        let failures = self.render(block.left, block.right, frames);
        for (track, entry) in report.reports.iter_mut().enumerate().take(W) {
            entry.nonfinite_left_blocks = u64::from(failures[0][track]);
            entry.nonfinite_right_blocks = u64::from(failures[1][track]);
        }
        report
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_track(bank_track_index(track_index, W)?, output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore_track(bank_track_index(track_index, W)?, version, input)
    }
}

/// Rejects a track index outside the bank.
fn bank_track_index(track_index: u32, lanes: usize) -> Result<usize, StatePayloadError> {
    let track = track_index as usize;
    if track >= lanes {
        return Err(StatePayloadError {
            code: "effect.bank.track",
        });
    }
    Ok(track)
}

/// The once-per-block bank entry guard: shapes, offsets and sample arithmetic.
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
            .checked_add(u64::from(block.frames))
            .is_some()
        && block.automation_offsets.len() == lanes + 1
        && block.automation_offsets.first() == Some(&0)
        && block.automation_offsets.last().copied() == Some(block.automation.len() as u32)
        && !block
            .automation_offsets
            .windows(2)
            .any(|pair| pair[0] > pair[1])
}

pub mod corpus;
