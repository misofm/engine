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
use miso_engine_effect_runtime::bank::{
    BLOCK_LIMIT, block_is_positive_zero, check_block, lane_is_positive_zero, nonfinite_lane_mask,
};
use miso_engine_effect_runtime::params::{
    ParameterSpec, normalize_zero, parameter_value_valid as domain_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload as payload;
use miso_engine_lane::kernels::{
    SvfCoef, SvfCoefStep, SvfState, svf_block, svf_block_ramped, svf_cascade_interleaved,
};
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

    /// `true` when `other` is this band bit for bit.
    ///
    /// Issue #144 item 6. Derived `PartialEq` would compare the four `f32` fields with `==`, which
    /// makes `-0.0` equal `0.0` and makes a `NaN` band unequal to itself. Neither can reach a
    /// stored `BandTarget` today -- values are normalised and domain-checked on the way in -- but
    /// the hoist this feeds decides whether an `f64` coefficient design is skipped, and a decision
    /// of that weight is made on bits rather than on a coincidence of the current admission rules.
    fn same_bits(&self, other: &Self) -> bool {
        self.enabled == other.enabled
            && self.kind == other.kind
            && self
                .numeric()
                .iter()
                .zip(other.numeric().iter())
                .all(|(left, right)| left.to_bits() == right.to_bits())
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
/// The words lane `track` of `section` is currently heading for.
///
/// These are exactly the words `BandTarget::words` last returned for this lane's stored band, so
/// reading them back is the same value the design would recompute -- see `Channel::target_words`.
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

/// The `-0.0` bit pattern.
///
/// It is the one input value an *elided* identity section would not reproduce, and the one state
/// word that can defeat the sign argument the elision gate rests on. See [`cascade_sections`].
const NEGATIVE_ZERO_BITS: u32 = 0x8000_0000;

/// Sign-clearing mask, so a magnitude comparison is one integer `and` and one integer compare.
const MAGNITUDE_MASK: u32 = 0x7fff_ffff;

/// Magnitude bits of [`BLOCK_LIMIT`], the ceiling the elision gate applies to its input planes.
///
/// `f32` magnitude bit patterns are monotone in magnitude, so `bits & MAGNITUDE_MASK > this` is
/// exactly `|x| > BLOCK_LIMIT` — and because every infinity and every NaN has magnitude bits at or
/// above `0x7f80_0000`, that single compare also refuses both. The gate needs all three refusals:
/// a non-finite value does not survive an identity section unchanged (`0.0 * inf` is `NaN`), and
/// the bound is what keeps a live section from overflowing into one mid-cascade.
const ELISION_MAGNITUDE_CEILING: u32 = BLOCK_LIMIT.to_bits();

/// The six [`EqSvfWordsV1::IDENTITY`] words as raw bits, in the pinned order.
///
/// Bits rather than floats, for the same reason [`Channel::state_bits`] uses them: the flag this
/// feeds claims a section is the *exact* identity, and `-0.0 == 0.0` would let a section that is
/// not claim that it is.
const IDENTITY_WORD_BITS: [u32; 6] = {
    let words = EqSvfWordsV1::IDENTITY.to_array();
    [
        words[0].to_bits(),
        words[1].to_bits(),
        words[2].to_bits(),
        words[3].to_bits(),
        words[4].to_bits(),
        words[5].to_bits(),
    ]
};

/// `true` when every lane of `value` holds exactly the bit pattern `bits`.
fn lane_bits_all<L: Lane>(value: L, bits: u32) -> bool {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].iter().all(|word| *word == bits)
}

/// `true` when no lane of `value` is `-0.0`.
fn lane_has_no_negative_zero<L: Lane>(value: L) -> bool {
    debug_assert!(L::WIDTH <= MAX_LANES);
    let mut words = [0_u32; MAX_LANES];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH]
        .iter()
        .all(|word| *word != NEGATIVE_ZERO_BITS)
}

/// `true` when both integrator words of `section` are exactly `+0.0` on every lane.
fn section_state_is_positive_zero<L: Lane>(section: &Section<L>) -> bool {
    lane_is_positive_zero::<L>(section.state.ic1) && lane_is_positive_zero::<L>(section.state.ic2)
}

/// `true` when no integrator word of `section` is `-0.0` on any lane.
fn section_state_has_no_negative_zero<L: Lane>(section: &Section<L>) -> bool {
    lane_has_no_negative_zero::<L>(section.state.ic1)
        && lane_has_no_negative_zero::<L>(section.state.ic2)
}

/// `true` when no word of `io` is `-0.0` and every word is finite and inside [`BLOCK_LIMIT`].
///
/// One branchless integer scan of the block, in the same shape as
/// [`miso_engine_effect_runtime::bank::check_block`]: two compares and two `or`s per word, folded
/// into one accumulator that is inspected once at the end, so the loop vectorises and nothing
/// leaves the integer domain until the block is finished.
///
/// It is deliberately **not** chunked and short-circuiting the way
/// [`block_is_positive_zero`] is. That predicate's common answer is "no" on the first chunk; this
/// one's common answer is "yes", which is only knowable from the whole block.
#[inline]
#[must_use]
fn block_admits_elision(io: &[f32]) -> bool {
    let mut rejected = 0_u32;
    for value in io {
        let bits = value.to_bits();
        rejected |= u32::from(bits == NEGATIVE_ZERO_BITS);
        rejected |= u32::from((bits & MAGNITUDE_MASK) > ELISION_MAGNITUDE_CEILING);
    }
    rejected == 0
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
    /// Per section: this section's coefficient words are [`EqSvfWordsV1::IDENTITY`] to the bit on
    /// **every** lane of this channel.
    ///
    /// Maintained only where coefficients change -- [`settle`](Self::settle),
    /// [`start_ramp`](Self::start_ramp), [`snap`](Self::snap),
    /// [`restore_track`](Self::restore_track) -- so a rendered block pays nothing to keep it. A
    /// ramp moves `coef` per segment through `advance_words` without passing through any of those,
    /// which would leave the flag stale; it cannot be read while that is true, because a lane with
    /// a ramp in flight makes the whole bank non-stationary and the elision this feeds is on the
    /// stationary path only. Every ramp ends in [`snap`](Self::snap), which refreshes it.
    ///
    /// [`identity_flags_agree`](Self::identity_flags_agree) re-derives the whole array from the
    /// words and is asserted in debug builds on every stationary block, so a coefficient-change
    /// site added later without a refresh is a test failure rather than a silent wrong render.
    identity: [bool; EQ_SECTION_COUNT_V1],
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
            // Every `(section, track)` pair is settled below, and `settle` refreshes the flag.
            identity: [false; EQ_SECTION_COUNT_V1],
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
        self.refresh_identity(section);
    }

    /// Re-derives `identity[section]` from the section's coefficient words.
    ///
    /// Control plane only: it runs where a coefficient changes, never per block and never per
    /// frame. Six lane compares, and it reads the words rather than tracking which lane was
    /// written, so it cannot drift out of step with a partially updated section.
    fn refresh_identity(&mut self, section: usize) {
        let identity = {
            let coef = &self.sections[section].coef;
            (0..6)
                .all(|index| lane_bits_all::<L>(coef_word(coef, index), IDENTITY_WORD_BITS[index]))
        };
        self.identity[section] = identity;
    }

    /// `true` when every `identity` flag equals what the coefficient words say right now.
    ///
    /// Asserted in debug builds on every stationary block. It is the standing check that the list
    /// of coefficient-change sites is complete.
    fn identity_flags_agree(&self) -> bool {
        (0..EQ_SECTION_COUNT_V1).all(|section| {
            let coef = &self.sections[section].coef;
            let observed = (0..6)
                .all(|index| lane_bits_all::<L>(coef_word(coef, index), IDENTITY_WORD_BITS[index]));
            observed == self.identity[section]
        })
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
    /// `#[inline(never)]`: this is coefficient *design*, not render arithmetic. It runs only when
    /// an admitted automation span retargets a band, it is per lane and scalar by nature (six
    /// words, one subtract and one multiply each), and the shipped wasm artifact is gated on the
    /// EQ's render kernel carrying no scalar arithmetic budget it does not need
    /// (`check-web-audioworklet.sh`, `KERNEL_ROSTER`). Before #163 phase 3 the inliner kept it out
    /// of `process_bank` on its own; phase 3 made that function small enough that it stopped, so
    /// the shape is pinned here rather than left to a heuristic.
    #[inline(never)]
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
        // `coef` did not move here, so this cannot change the flag. It is refreshed anyway because
        // "every coefficient-change site refreshes" is a rule that is cheap to keep total and
        // expensive to keep partial.
        self.refresh_identity(section);
    }

    /// The words lane `track` of `section` is heading for, read back out of the lane words.
    ///
    /// Issue #144 item 6, the re-preparation half. `BandTarget::words` is a pure function of the
    /// band and the sample rate, and `Section::target` holds exactly what it returned the last
    /// time this lane's band changed. So when an automation point restates a band it has already
    /// been given, the design does not have to be recomputed -- it can be read. That matters far
    /// more than the ramp arithmetic: the design is an `f64` `design_svf_v1` per lane per event,
    /// and a console that refreshes its automation is paying it on every refresh for every band it
    /// did not move.
    ///
    /// This is bit-identical rather than approximately equal, by determinism: same band, same
    /// rate, same function, same words.
    fn target_words(&self, section: usize, track: usize) -> EqSvfWordsV1 {
        let slot = &self.sections[section];
        EqSvfWordsV1::from_array(core::array::from_fn(|index| {
            lane_get(coef_word(&slot.target, index), track)
        }))
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
    ///
    /// **The caller refreshes** `identity[section]`. Every call site is a loop over the lanes of
    /// one section, and [`refresh_identity`](Self::refresh_identity) re-derives the whole section
    /// from its coefficient words, so running it inside this function re-derived the same six lane
    /// compares `W` times for one bank-wide ramp end. Hoisting it is exact rather than merely
    /// cheaper: the flag is read only on a *stationary* block ([`cascade_sections`]), no lane's
    /// ramp can start mid-block, and a section's last snap is the one whose value survives either
    /// way. `identity_flags_agree` is the standing oracle that no refresh site was lost.
    fn snap(&mut self, section: usize, track: usize) {
        let slot = &mut self.sections[section];
        for index in 0..6 {
            let target = lane_get(coef_word(&slot.target, index), track);
            lane_set(coef_word_mut(&mut slot.coef, index), track, target);
            lane_set(step_word_mut(&mut slot.step, index), track, 0.0);
        }
        self.remaining[section][track] = 0;
    }

    /// Snaps **every** lane of `section` at once: six vector copies and one zeroed step set.
    ///
    /// Bit-identical to `for track in 0..W { self.snap(section, track) }`, and it is the shape a
    /// bank-wide ramp end actually has -- the console moves a band on all `W` lanes of a bank
    /// together. The per-lane form pays a `lane_get`/`lane_set` pair per word per lane, and each of
    /// those is a full `store`/`load` round trip out of and back into the vector domain: `12 * W`
    /// of them per section, to write words the target already holds in exactly the right lanes.
    ///
    /// The caller refreshes `identity[section]`, as with [`snap`](Self::snap).
    fn snap_section(&mut self, section: usize) {
        let slot = &mut self.sections[section];
        slot.coef = slot.target;
        slot.step = SvfCoefStep::default();
        self.remaining[section] = [0; W];
    }

    /// Runs the four sections over one block in place.
    ///
    /// `#[inline(always)]`, and [`process_section`](Self::process_section) with it, so that the
    /// whole render path of a bank stays inside one wasm function. `check-web-audioworklet.sh`
    /// asserts that each shipped effect has **exactly one** arithmetic-carrying kernel in the
    /// artifact -- that is how a de-vectorisation is caught. #163 phase 3 made `process_bank`
    /// small enough that the inliner began outlining this body into a second one, and a second
    /// kernel reads to that gate as a kernel that moved, not as the ramp fallback it is.
    #[inline(always)]
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
    #[inline(always)]
    fn process_section(&mut self, section: usize, io: &mut [f32], frames: usize) {
        let mut position = 0;
        let mut snapped = false;
        while position < frames {
            // A ramp that ends on every lane at once is the common case -- a console automates a
            // band across a whole bank -- and it is the one that can skip the lane loop entirely.
            if self.remaining[section]
                .iter()
                .all(|remaining| *remaining == 1)
            {
                self.snap_section(section);
                snapped = true;
            } else {
                for track in 0..W {
                    if self.remaining[section][track] == 1 {
                        self.snap(section, track);
                        snapped = true;
                    }
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
        // Once for the section, not once per snapped lane per segment. Nothing between the snaps
        // above and here reads `identity`, and the value this leaves is the value the last snap
        // would have left: see the note on `snap`.
        if snapped {
            self.refresh_identity(section);
        }
    }

    /// `true` when no lane of any section has a ramp in flight.
    ///
    /// A ramp means the coefficients move within the block, so the "same coefficients" leg of the
    /// fixed-point induction does not hold and the fast path must not engage.
    fn no_ramp_in_flight(&self) -> bool {
        self.remaining
            .iter()
            .all(|section| section.iter().all(|remaining| *remaining == 0))
    }

    /// The raw bit patterns of every integrator word, for an exact before/after comparison.
    ///
    /// Bits rather than floats because the comparison has to distinguish `+0.0` from `-0.0`: an
    /// integrator that settles at `-0.0` is still settled, but a float compare would also call a
    /// pair that moved between the two zeros "unchanged", which is exactly the bit the fast path
    /// promises not to move.
    fn state_bits(&self, out: &mut [u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES]) {
        for (index, section) in self.sections.iter().enumerate() {
            let base = index * 2 * L::WIDTH;
            section
                .state
                .ic1
                .store_bits(&mut out[base..base + L::WIDTH]);
            section
                .state
                .ic2
                .store_bits(&mut out[base + L::WIDTH..base + 2 * L::WIDTH]);
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
            // Every lane, so this is the whole-section snap by construction; one refresh per
            // section rather than one per lane.
            self.snap_section(section);
            self.refresh_identity(section);
        }
    }
}

/// Runs both channels' four-section cascades over one block.
///
/// Issue #163 phase 3. `stationary` means no lane of either channel has a ramp in flight, so
/// every section would run [`svf_block`] with fixed coefficients over the whole block --
/// exactly `EQ_SECTION_COUNT_V1 * 2` serial passes, each one a recurrence whose next frame
/// cannot start until this one's integrators are written. That shape is latency-bound: it
/// leaves the vector units idle for most of every frame, and it is why the EQ took the same
/// wall time at `Simd4` as at `Simd8`.
///
/// The interleaved form runs the two channels in one frame loop, [`Lane::SVF_CASCADE_DEPTH`]
/// sections deep, so several independent recurrences are always in flight. It is the *same*
/// operation order per chain -- see [`svf_cascade_interleaved`] -- so this is a schedule
/// change, not a numeric one, and the fixture pins are untouched.
///
/// A ramping block falls back to the per-section path, which owns the block-splitting rule
/// that a moving coefficient needs. Ramps run for at most a smoothing window after a
/// parameter change; a console rendering audio is stationary on essentially every block.
#[inline(always)]
fn process_channels<L: Lane, const W: usize>(
    channels: (&mut Channel<L, W>, &mut Channel<L, W>),
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    stationary: bool,
) {
    if !stationary {
        channels.0.process_block(left, frames);
        channels.1.process_block(right, frames);
        return;
    }
    debug_assert!(channels.0.identity_flags_agree());
    debug_assert!(channels.1.identity_flags_agree());
    let sections = cascade_sections::<L, W>(channels.0, channels.1, left, right, frames);
    // `SVF_CASCADE_DEPTH` is a constant, so exactly one arm survives monomorphisation and the
    // match costs nothing. Every arm is bit-identical; a depth that did not divide the cascade
    // would silently drop sections, so only the divisors of `EQ_SECTION_COUNT_V1` are reached
    // and anything else falls back to the always-valid depth of one.
    match L::SVF_CASCADE_DEPTH {
        4 => interleave::<L, W, 4>(channels, left, right, frames, sections),
        2 => interleave::<L, W, 2>(channels, left, right, frames, sections),
        _ => interleave::<L, W, 1>(channels, left, right, frames, sections),
    }
}

/// The cascade positions this stationary block will actually run, in cascade order.
///
/// All four, unless identity-section elision is admissible — in which case the sections that are
/// the exact identity on every lane of *both* channels are dropped from the list, and the cascade
/// runs shorter.
///
/// # Why a section can be dropped at all
///
/// `BandTarget::words` maps `enabled = false` to [`EqSvfWordsV1::IDENTITY`] at every other
/// parameter, so a disabled band is not "nearly" a pass-through, it is `c1 = a2 = a3 = m1 = m2 =
/// +0.0, m0 = 1.0`. A console that ships four bands and uses two spends half the cascade there.
/// The bank runs both channels through [`svf_cascade_interleaved`] at a fixed
/// [`Lane::SVF_CASCADE_DEPTH`], so dropping sections buys whole passes.
///
/// The flag is per section **across both channels** because that is the granularity the kernel
/// has: one pass carries section `k` of the left channel and section `k` of the right, and a
/// section that is identity on the left but live on the right must still run.
///
/// # The proof obligation
///
/// The claim is exact bit identity of the rendered audio **and** of every section's integrator
/// words, elided sections included — not approximate agreement. Take an identity section whose
/// two integrator words are exactly `+0.0`, and one input word `v0`.
///
/// * `v3 = v0 - ic2 = v0 - (+0.0) = v0`, bit for bit, including `v0 = -0.0` (IEEE-754 gives
///   `(-0) - (+0) = -0` under round-to-nearest).
/// * `d1 = nc1 * ic1 + a2 * v3`. `nc1 = -c1 = -0.0`, so `nc1 * ic1 = -0.0`; `a2 * v3 = +0.0 * v0`,
///   which is `+0.0` for `v0 >= +0.0` and `-0.0` for `v0 < 0` or `v0 = -0.0`. Either way `d1` is a
///   zero, and `v1 = ic1 + d1 = (+0.0) + (±0.0) = +0.0`.
/// * `d2 = a3 * v3 + a2 * ic1 = (±0.0) + (+0.0) = +0.0`, and `v2 = ic2 + d2 = +0.0`.
/// * `ic1' = flush(ic1 + (d1 + d1)) = flush(+0.0) = +0.0` and likewise `ic2' = +0.0`. **The state
///   stays exactly `+0.0`, by induction over the block.** (`flush` maps every zero to `+0.0`.)
/// * `y = m2 * v2 + (m1 * v1 + m0 * v0) = (+0.0) + ((+0.0) + v0)`. `m0 = 1.0`, so `m0 * v0 = v0`
///   exactly; `(+0.0) + v0` is `v0` for every `v0` **except** `v0 = -0.0`, where it is `+0.0`.
///
/// So an identity section at `+0.0` state is the exact identity on a finite input, with exactly
/// one exception: it rewrites `-0.0` to `+0.0`. Eliding it is therefore bit-exact **iff no `-0.0`
/// ever reaches it**, and the state it keeps is `+0.0` either way — which is what gate (b) below
/// asserts it already holds.
///
/// That leaves: can a `-0.0` reach an elided section? Its input is either the block input or a
/// live section's output, so both have to be closed.
///
/// * **The block input.** Gate (a) refuses any block containing the `-0.0` bit pattern.
/// * **A live section's output.** `y = m2 * v2 + (m1 * v1 + m0 * v0)`, two `f32` additions.
///   An `f32` addition yields `-0.0` **only** when both addends are `-0.0`: a nonzero exact sum in
///   the subnormal range is representable, so addition never underflows to a zero of either sign,
///   and exact cancellation `a + (-a)` gives `+0.0` under round-to-nearest. So `y = -0.0` requires
///   all three of `m2 * v2`, `m1 * v1` and `m0 * v0` to be `-0.0`. `design_svf_v1` normalises
///   `-0.0` out of every designed word, and the `m0` column of `design_svf_words_f64` is `+0.0`
///   for a low-pass and strictly positive for every other kind, so `m0 >= +0.0` always. Two cases:
///   - `m0 > 0`. Then `m0 * v0 = -0.0` needs either `v0 = -0.0` — excluded by induction, this
///     section's input carries no `-0.0` — or a multiplication that underflows to `-0.0`, which
///     needs `|m0 * v0| <= 2^-150`. For `m0 = 1.0` (high-pass, notch, bell, low shelf) that forces
///     `v0 = ±0.0` and contradicts. The one remaining `m0` is the high shelf's `A^2`, with
///     `A = 10^(gain/40)` and `gain` domain-limited to `[-24, 24]`, so `A^2 >= 0.063`; underflow
///     then needs `|v0| <= 2^-150 / A^2` **and** `A^2 <= 0.5`, hence `m2 = 1 - A^2 >= 0.5`, hence
///     `m2 * v2 = -0.0` needs `v2 = -2^-149` exactly. `v2 = ic2 + d2` and `v1 = ic1 + d1`; every
///     `a2` this design produces is `g / (1 + g * (g + k)) <= 1 / (2 + k) < 0.5` because `k > 0`
///     on every kind, so `a2 * v3` at `|v3| = 2^-149` underflows to a zero, `d1` is a zero, and
///     `v1 = ic1 + d1` is either `+0.0` (giving `m1 * v1 = +0.0`, since a cut high shelf has
///     `m1 = shelf_k * (1 - A) * A > 0`) or has `|v1| >= FLUSH_EPS * 2^-24`, far too large to
///     underflow. Either way `m1 * v1 != -0.0` and the conjunction fails.
///   - `m0 = +0.0`, i.e. a low pass, whose other words are `m1 = +0.0` and `m2 = 1.0`. Then
///     `m2 * v2 = v2`, so `y = -0.0` needs `v2 = -0.0`, which by the addition rule needs
///     `ic2 = -0.0`. `flush` maps every zero to `+0.0`, so no integrator word the kernel writes is
///     ever `-0.0`; the only way in is a restored state payload, and gate (c) refuses that.
///
///   So no live section emits `-0.0` when its own input carries none, and the induction closes.
///
/// **Finiteness is part of the claim, not an aside.** `0.0 * inf` is `NaN`, so an identity section
/// handed an infinity writes `NaN` where elision would pass the infinity through. Gate (a)'s
/// magnitude ceiling refuses non-finite input, and by refusing anything above [`BLOCK_LIMIT`] —
/// `1e30`, about `3.4e8` below `f32::MAX`, against a four-section cascade whose per-section output
/// mix is bounded by `|m0| + |m1| + |m2| < 2^6` — it also leaves the cascade unable to reach an
/// infinity from a block it admitted.
///
/// # Correctness never depends on the gate
///
/// Every leg is a refusal: any doubt returns all four sections and the block renders exactly as it
/// did before this function existed. The gate is a performance predicate, and the only thing a
/// wrong *engagement* rule could cost is speed.
#[inline(always)]
fn cascade_sections<L: Lane, const W: usize>(
    left_channel: &Channel<L, W>,
    right_channel: &Channel<L, W>,
    left: &[f32],
    right: &[f32],
    frames: usize,
) -> ([usize; EQ_SECTION_COUNT_V1], usize) {
    let all = (core::array::from_fn(|section| section), EQ_SECTION_COUNT_V1);
    let dead = |section: usize| left_channel.identity[section] && right_channel.identity[section];
    let depth = L::SVF_CASCADE_DEPTH.clamp(1, EQ_SECTION_COUNT_V1);
    let live = (0..EQ_SECTION_COUNT_V1)
        .filter(|section| !dead(*section))
        .count();
    // The kernel runs whole passes of `DEPTH` sections, so the list has to divide by the depth.
    // Rounding the live count up and paying for the difference in identity sections keeps the
    // *one* instantiation of `svf_cascade_interleaved` this backend already ships: a shorter final
    // pass would need a second `DEPTH`, and a second arithmetic-carrying EQ kernel in the wasm
    // artifact reads to `KERNEL_ROSTER` as a kernel that moved.
    let kept = live.div_ceil(depth) * depth;
    if kept >= EQ_SECTION_COUNT_V1 {
        return all;
    }
    // (a) Neither input plane carries `-0.0`, an infinity, a NaN, or a magnitude above the §4.4
    // bound. See the proof above; this is the leg that stops a `-0.0` entering the cascade.
    let words = frames * W;
    if !block_admits_elision(&left[..words]) || !block_admits_elision(&right[..words]) {
        return all;
    }
    for section in 0..EQ_SECTION_COUNT_V1 {
        let admissible = if dead(section) {
            // (b) An elided section must already be at the `+0.0` state the proof's induction
            // starts from -- otherwise its state would move in the full cascade and not here.
            section_state_is_positive_zero(&left_channel.sections[section])
                && section_state_is_positive_zero(&right_channel.sections[section])
        } else {
            // (c) A live section must carry no `-0.0` integrator word. Nothing the kernel writes
            // is ever `-0.0`, but a restored state payload is admitted on finiteness alone, and a
            // low pass with `ic2 = -0.0` is the one shape that can emit `-0.0` into a later
            // elided section.
            section_state_has_no_negative_zero(&left_channel.sections[section])
                && section_state_has_no_negative_zero(&right_channel.sections[section])
        };
        if !admissible {
            return all;
        }
    }
    let mut padding = kept - live;
    let mut list = [0_usize; EQ_SECTION_COUNT_V1];
    let mut length = 0;
    for section in 0..EQ_SECTION_COUNT_V1 {
        let keep = if dead(section) {
            let take = padding > 0;
            padding -= usize::from(take);
            take
        } else {
            true
        };
        if keep {
            list[length] = section;
            length += 1;
        }
    }
    debug_assert_eq!(length, kept);
    (list, length)
}

/// One stationary block, both channels, `DEPTH` cascade sections fused per pass.
///
/// `sections` is the list [`cascade_sections`] chose and its length, which is a whole number of
/// passes by construction. Positions are read out of it rather than counted from a base, so an
/// elided cascade runs the same kernel over a shorter list — the operation order per surviving
/// chain is untouched, which is what keeps this a schedule change.
#[inline(always)]
fn interleave<L: Lane, const W: usize, const DEPTH: usize>(
    channels: (&mut Channel<L, W>, &mut Channel<L, W>),
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    sections: ([usize; EQ_SECTION_COUNT_V1], usize),
) {
    debug_assert_eq!(EQ_SECTION_COUNT_V1 % DEPTH, 0);
    let (list, length) = sections;
    debug_assert_eq!(length % DEPTH, 0);
    for pass in 0..length / DEPTH {
        let base = pass * DEPTH;
        let at: [usize; DEPTH] = core::array::from_fn(|k| list[base + k]);
        let coefficients: [[SvfCoef<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| channels.0.sections[at[k]].coef),
            core::array::from_fn(|k| channels.1.sections[at[k]].coef),
        ];
        let mut state: [[SvfState<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| channels.0.sections[at[k]].state),
            core::array::from_fn(|k| channels.1.sections[at[k]].state),
        ];
        svf_cascade_interleaved::<L, 2, DEPTH>(
            [&mut *left, &mut *right],
            frames,
            &coefficients,
            &mut state,
        );
        let [left_state, right_state] = state;
        for (k, word) in left_state.into_iter().enumerate() {
            channels.0.sections[at[k]].state = word;
        }
        for (k, word) in right_state.into_iter().enumerate() {
            channels.1.sections[at[k]].state = word;
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
        for section in 0..EQ_SECTION_COUNT_V1 {
            self.refresh_identity(section);
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
    /// Issue #163 phase 4 item 1: the previous block proved this bank is at a silent fixed point.
    ///
    /// Set only by [`render`](Self::render), and only after it has *observed* -- not assumed --
    /// all three facts on a block it actually ran: the input was exactly `+0.0` everywhere, the
    /// output it produced was exactly `+0.0` everywhere, and every integrator word came out of the
    /// block bit-identical to the word it went in with. Cleared by anything that can move a
    /// coefficient or a state word out from under that observation.
    ///
    /// The induction it licenses: a bank kernel is a pure function of (input, state,
    /// coefficients). If those three are bit-identical to a block that provably produced all
    /// `+0.0` output and left its state unmoved, the next block produces the same output and
    /// leaves the state unmoved again -- so writing nothing is bit-identical to running it. This
    /// is why the flag is *earned* by observation rather than derived from a theory about where an
    /// SVF settles: the fixed point is not always `+0.0` (a negative coefficient drives an
    /// integrator to `-0.0` and it stays there), and a theory that assumed it was would either
    /// engage wrongly or never engage at all.
    silent_fixed_point: bool,
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
                // The stored band is read by value, so the borrow of the channel ends here and
                // the cached-design read below can borrow it again.
                let stored = if channel == 0 {
                    self.left.targets[track][section]
                } else {
                    self.right.targets[track][section]
                };
                let mut candidate = stored;
                for (field, value) in updates.into_iter().enumerate() {
                    if let Some(value) = value {
                        candidate.set_numeric(field, value);
                    }
                }
                // Issue #144 item 6: a band restated at the values it already holds designs to
                // the words it is already heading for, so the `f64` design is read from the lane
                // rather than recomputed. `words` is deterministic in (band, rate) and
                // `Section::target` is what it last returned, so the two are the same bits.
                let designed = if candidate.same_bits(&stored) {
                    Ok(if channel == 0 {
                        self.left.target_words(section, track)
                    } else {
                        self.right.target_words(section, track)
                    })
                } else {
                    candidate.words(sample_rate)
                };
                match designed {
                    Ok(words) => {
                        if channel == 0 {
                            self.left.targets[track][section] = candidate;
                            self.left.start_ramp(section, track, words);
                        } else {
                            self.right.targets[track][section] = candidate;
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
        let words = frames * W;
        // Issue #163 phase 4 item 1. `quiet` is the whole precondition, evaluated whole-bank:
        // no ramp in flight on either channel (so the coefficients are the same words the
        // observed block used) and both input planes exactly `+0.0`. The block test short-circuits on
        // its first chunk, so a console rendering music pays 32 words per channel for this and
        // nothing else.
        // #163 phase 3 reuses this leg. "Stationary" -- no lane of either channel has a ramp in
        // flight -- is exactly the condition under which every section runs `svf_block` with fixed
        // coefficients over the whole block, which is the shape the interleaved cascade replaces.
        let stationary = self.left.no_ramp_in_flight() && self.right.no_ramp_in_flight();
        let quiet = stationary
            && block_is_positive_zero(&left[..words])
            && block_is_positive_zero(&right[..words]);
        if quiet && self.silent_fixed_point {
            // The buffers already hold exactly `+0.0`, which is exactly what the four sections
            // would have written; the integrators are at a fixed point the previous block
            // measured. Writing nothing is bit-identical, and an all-`+0.0` block is trivially
            // inside the §4.4 bound, so no lane failed.
            return failures;
        }
        let mut before_left = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
        let mut before_right = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
        if quiet {
            self.left.state_bits(&mut before_left);
            self.right.state_bits(&mut before_right);
        }
        process_channels(
            (&mut self.left, &mut self.right),
            left,
            right,
            frames,
            stationary,
        );
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
        // Earn (or lose) the flag from what this block actually did. All three legs are required:
        // the input was `+0.0` and the coefficients were still (`quiet`), the integrators came out
        // exactly as they went in, and the output the sections wrote was `+0.0` to the bit. Only
        // then is "write nothing" a faithful replay of "run the kernel".
        self.silent_fixed_point = quiet && {
            let mut after_left = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
            let mut after_right = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
            self.left.state_bits(&mut after_left);
            self.right.state_bits(&mut after_right);
            after_left == before_left
                && after_right == before_right
                && block_is_positive_zero(&left[..words])
                && block_is_positive_zero(&right[..words])
        };
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
        // Nothing has been observed yet, so nothing is claimed.
        silent_fixed_point: false,
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
        // #163 phase 4 item 1: a reset moves state, and `FullToDefaults` moves coefficients too.
        // The flag is a claim about a block that has now been overwritten, so it is withdrawn.
        self.silent_fixed_point = false;
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
        // #163 phase 4 item 1, as in `process_bank`: automation can snap a coefficient without
        // ever raising `remaining`, so the claim is withdrawn whenever a span arrives.
        if !block.automation.is_empty() {
            self.silent_fixed_point = false;
        }
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
        // #163 phase 4 item 1: see `restore_track_state_payload`.
        self.silent_fixed_point = false;
        self.restore_track(0, version, input)
    }
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedParametricEq<L, W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.bank.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        // #163 phase 4 item 1: a reset moves state, and `FullToDefaults` moves coefficients too.
        // The flag is a claim about a block that has now been overwritten, so it is withdrawn.
        self.silent_fixed_point = false;
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
        // #163 phase 4 item 1: an admitted span retargets a band, and a span with no smoothing
        // snaps the coefficient words outright while leaving `remaining` at zero -- so
        // `no_ramp_in_flight` alone would not notice it. Withdraw the claim whenever this block
        // carries automation at all; the next silent block re-earns it.
        if !block.automation.is_empty() {
            self.silent_fixed_point = false;
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
        // #163 phase 4 item 1: a restore writes integrators and coefficients this bank never
        // rendered, so any standing fixed-point claim is void.
        self.silent_fixed_point = false;
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

/// Issue #163 phase 3: the interleaved cascade is the per-section path, bit for bit.
///
/// The frozen E9 corpus (`corpus.rs`, `tests/determinism.rs`) drives [`Channel::process_block`]
/// directly, because its whole job is to describe *arithmetic* independently of layout -- so it
/// runs one channel at a time and never reaches [`process_channels`]. That leaves the phase-3 fast
/// path outside the pinned digests, which is exactly where a schedule change could hide. This
/// module closes that: it drives both arms of [`process_channels`] over the same corpus signals and
/// asserts equality of the rendered audio **and** of every integrator word left behind, at all
/// three widths.
///
/// The two arms are the *same entry point* with its stationary flag flipped, so the sequential arm
/// is literally the code the EQ ran before phase 3 rather than a re-transcription of it, and no
/// runtime tuning knob is added to reach it.
#[cfg(test)]
mod interleave_identity {
    use super::{BandTarget, Channel, EQ_SECTION_COUNT_V1, MAX_LANES, corpus, process_channels};
    use miso_engine_lane::{Lane, Simd4, Simd8};

    /// Frames per case: the corpus length, several blocks' worth of settling.
    const FRAMES: usize = corpus::FRAMES;

    /// One channel of `W` tracks from the corpus band table, offset so the two channels of a case
    /// never carry the same configuration.
    fn channel<L: Lane, const W: usize>(offset: usize) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT_V1]; W] =
            core::array::from_fn(|lane| corpus::bands((offset + lane) % corpus::LANES));
        Channel::new(targets, corpus::CORPUS_RATE).expect("every corpus row is a legal design")
    }

    /// One AoSoA block of corpus audio for `W` tracks.
    fn block<const W: usize>(case: usize, offset: usize) -> Vec<f32> {
        let mut lanes = vec![[0.0_f32; FRAMES]; W];
        for (index, lane) in lanes.iter_mut().enumerate() {
            corpus::fill(case, (offset + index) % corpus::LANES, lane);
        }
        let mut out = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for (index, lane) in lanes.iter().enumerate() {
                out[frame * W + index] = lane[frame];
            }
        }
        out
    }

    /// Every integrator word of a channel pair, as raw bits.
    fn integrators<L: Lane, const W: usize>(
        left: &Channel<L, W>,
        right: &Channel<L, W>,
    ) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
        let mut out = Vec::new();
        left.state_bits(&mut words);
        out.extend_from_slice(&words);
        right.state_bits(&mut words);
        out.extend_from_slice(&words);
        out
    }

    /// Raw bits of a block.
    fn bits(samples: &[f32]) -> Vec<u32> {
        samples.iter().map(|sample| sample.to_bits()).collect()
    }

    /// Runs one corpus case at one width down both arms and asserts they are the same bits.
    ///
    /// `seed_state` puts a non-zero, subnormal-adjacent state into every section first, so the
    /// comparison covers the D7 flush inside the interleaved body and not only a cold start.
    fn compare<L: Lane, const W: usize>(width: &str, case: usize, seed_state: bool) {
        let mut arms = Vec::new();
        for stationary in [true, false] {
            let mut left_channel = channel::<L, W>(0);
            let mut right_channel = channel::<L, W>(3);
            if seed_state {
                for section in 0..EQ_SECTION_COUNT_V1 {
                    left_channel.sections[section].state.ic1 = L::splat(1.0e-40);
                    left_channel.sections[section].state.ic2 = L::splat(-1.0e-41);
                    right_channel.sections[section].state.ic1 = L::splat(-3.5e-7);
                    right_channel.sections[section].state.ic2 = L::splat(9.0e-8);
                }
            }
            let mut left = block::<W>(case, 0);
            let mut right = block::<W>(case, 3);
            process_channels(
                (&mut left_channel, &mut right_channel),
                &mut left,
                &mut right,
                FRAMES,
                stationary,
            );
            arms.push((left, right, integrators(&left_channel, &right_channel)));
        }
        let (interleaved, sequential) = (&arms[0], &arms[1]);
        assert_eq!(
            bits(&interleaved.0),
            bits(&sequential.0),
            "#163 phase 3: interleaved left channel differs at {width}, case {case}, \
             seeded={seed_state}"
        );
        assert_eq!(
            bits(&interleaved.1),
            bits(&sequential.1),
            "#163 phase 3: interleaved right channel differs at {width}, case {case}, \
             seeded={seed_state}"
        );
        assert_eq!(
            interleaved.2, sequential.2,
            "#163 phase 3: interleaved integrators differ at {width}, case {case}, \
             seeded={seed_state}"
        );
        // Non-vacuity: the arms must have filtered something, the channels must differ from each
        // other, and nothing may have left the finite range.
        assert_ne!(
            bits(&interleaved.0),
            bits(&block::<W>(case, 0)),
            "#163 phase 3: the case must actually filter at {width}"
        );
        assert_ne!(
            bits(&interleaved.0),
            bits(&interleaved.1),
            "#163 phase 3: the two channels must carry different audio at {width}"
        );
        assert!(
            interleaved.0.iter().all(|sample| sample.is_finite()),
            "#163 phase 3: a corpus case must stay finite at {width}"
        );
    }

    #[test]
    fn the_interleaved_cascade_renders_the_per_section_path_bit_for_bit() {
        // Case 1 of the corpus is the ramped one, and a ramp never reaches the interleaved arm --
        // its caller passes `stationary = false` -- so the stationary cases are 0 and 2.
        for case in [0_usize, 2] {
            for seeded in [false, true] {
                compare::<f32, 1>("Scalar", case, seeded);
                compare::<Simd4, 4>("Simd4", case, seeded);
                compare::<Simd8, 8>("Simd8", case, seeded);
            }
        }
    }

    /// The depth this gate exercises really is per backend, and really is a divisor of the cascade.
    #[test]
    fn the_tuned_depth_is_per_backend_and_divides_the_cascade() {
        for depth in [
            <f32 as Lane>::SVF_CASCADE_DEPTH,
            <Simd4 as Lane>::SVF_CASCADE_DEPTH,
            <Simd8 as Lane>::SVF_CASCADE_DEPTH,
        ] {
            assert_eq!(
                EQ_SECTION_COUNT_V1 % depth,
                0,
                "#163 phase 3: a cascade depth that does not divide {EQ_SECTION_COUNT_V1} \
                 sections would silently drop sections"
            );
        }
        assert_ne!(
            <f32 as Lane>::SVF_CASCADE_DEPTH,
            <Simd8 as Lane>::SVF_CASCADE_DEPTH,
            "#163 phase 3: the constant is tuned per backend, not shared"
        );
    }
}

/// Identity-section elision: the shortened cascade is the full cascade, bit for bit.
///
/// The oracle is the one [`interleave_identity`] already uses and for the same reason: the
/// `stationary = false` arm of [`process_channels`] is [`Channel::process_block`], which runs all
/// four sections through the per-section path and knows nothing about elision. So an equality
/// against it is an equality against the code the EQ ran before this existed, not against a
/// re-transcription of it -- and no runtime knob is added to reach either arm.
///
/// What is covered: every live/dead subset of the four sections, both channels agreeing and
/// disagreeing, at all three widths, cold and with seeded subnormal-adjacent state; the three
/// refusal legs of the gate (`-0.0` input, a non-`+0.0` state in an elided section, a `-0.0`
/// integrator in a live one) and the magnitude ceiling; per-lane disagreement inside one section;
/// and enable/disable transitions arriving mid-session through the ramp path.
#[cfg(test)]
mod elision {
    use super::{
        BandTarget, Channel, EQ_SECTION_COUNT_V1, EqSvfWordsV1, MAX_LANES, cascade_sections,
        corpus, process_channels,
    };
    use miso_engine_lane::{Lane, Simd4, Simd8};

    const FRAMES: usize = corpus::FRAMES;
    /// Every subset of the four cascade positions, as a bitmask of *live* sections.
    const MASKS: core::ops::Range<u8> = 0..(1 << EQ_SECTION_COUNT_V1);

    /// A channel whose section `s` is a real corpus band when `live` has bit `s` set, and the
    /// exact identity when it does not.
    ///
    /// Disabling is expressed through `BandTarget::enabled`, which is the production route: it is
    /// what `BandTarget::words` maps to [`EqSvfWordsV1::IDENTITY`], so the test is exercising the
    /// same words a session with a disabled band prepares.
    fn channel<L: Lane, const W: usize>(offset: usize, live: u8) -> Channel<L, W> {
        let targets: [[BandTarget; EQ_SECTION_COUNT_V1]; W] = core::array::from_fn(|lane| {
            let mut bands = corpus::bands((offset + lane) % corpus::LANES);
            for (section, band) in bands.iter_mut().enumerate() {
                band.enabled = live & (1 << section) != 0;
            }
            bands
        });
        Channel::new(targets, corpus::CORPUS_RATE).expect("every corpus row is a legal design")
    }

    fn block<const W: usize>(case: usize, offset: usize) -> Vec<f32> {
        let mut lanes = vec![[0.0_f32; FRAMES]; W];
        for (index, lane) in lanes.iter_mut().enumerate() {
            corpus::fill(case, (offset + index) % corpus::LANES, lane);
        }
        let mut out = vec![0.0_f32; FRAMES * W];
        for frame in 0..FRAMES {
            for (index, lane) in lanes.iter().enumerate() {
                out[frame * W + index] = lane[frame];
            }
        }
        out
    }

    fn integrators<L: Lane, const W: usize>(
        left: &Channel<L, W>,
        right: &Channel<L, W>,
    ) -> Vec<u32> {
        let mut words = [0_u32; EQ_SECTION_COUNT_V1 * 2 * MAX_LANES];
        let mut out = Vec::new();
        left.state_bits(&mut words);
        out.extend_from_slice(&words);
        right.state_bits(&mut words);
        out.extend_from_slice(&words);
        out
    }

    fn bits(samples: &[f32]) -> Vec<u32> {
        samples.iter().map(|sample| sample.to_bits()).collect()
    }

    /// How many sections the elision gate would run for this pair over this block.
    fn kept<L: Lane, const W: usize>(
        left_channel: &Channel<L, W>,
        right_channel: &Channel<L, W>,
        left: &[f32],
        right: &[f32],
    ) -> usize {
        cascade_sections::<L, W>(left_channel, right_channel, left, right, FRAMES).1
    }

    /// Runs one case down both arms with the given live masks and asserts bit equality of the
    /// rendered audio and of every integrator word, elided sections included.
    ///
    /// Returns the number of sections the elided arm actually ran, so a caller can assert that a
    /// configuration engaged rather than quietly falling back.
    fn compare<L: Lane, const W: usize>(
        width: &str,
        case: usize,
        left_live: u8,
        right_live: u8,
        seed_state: bool,
    ) -> usize {
        let mut arms = Vec::new();
        let mut ran = EQ_SECTION_COUNT_V1;
        for stationary in [true, false] {
            let mut left_channel = channel::<L, W>(0, left_live);
            let mut right_channel = channel::<L, W>(3, right_live);
            if seed_state {
                for section in 0..EQ_SECTION_COUNT_V1 {
                    // Only *live* sections are seeded: a non-`+0.0` state in a dead section is a
                    // refusal leg with its own test, and seeding it here would silently disable
                    // the very engagement this function is asserting.
                    if left_live & (1 << section) != 0 {
                        left_channel.sections[section].state.ic1 = L::splat(1.0e-40);
                        left_channel.sections[section].state.ic2 = L::splat(-1.0e-41);
                    }
                    if right_live & (1 << section) != 0 {
                        right_channel.sections[section].state.ic1 = L::splat(-3.5e-7);
                        right_channel.sections[section].state.ic2 = L::splat(9.0e-8);
                    }
                }
            }
            let mut left = block::<W>(case, 0);
            let mut right = block::<W>(case, 3);
            if stationary {
                ran = kept(&left_channel, &right_channel, &left, &right);
            }
            process_channels(
                (&mut left_channel, &mut right_channel),
                &mut left,
                &mut right,
                FRAMES,
                stationary,
            );
            arms.push((left, right, integrators(&left_channel, &right_channel)));
        }
        let (elided, full) = (&arms[0], &arms[1]);
        let label = format!(
            "{width}, case {case}, live {left_live:04b}/{right_live:04b}, seeded={seed_state}"
        );
        assert_eq!(
            bits(&elided.0),
            bits(&full.0),
            "elided left channel differs from the full cascade at {label}"
        );
        assert_eq!(
            bits(&elided.1),
            bits(&full.1),
            "elided right channel differs from the full cascade at {label}"
        );
        assert_eq!(
            elided.2, full.2,
            "elided integrators differ from the full cascade at {label}"
        );
        assert!(
            elided
                .0
                .iter()
                .chain(elided.1.iter())
                .all(|s| s.is_finite()),
            "an elided case must stay finite at {label}"
        );
        ran
    }

    /// Every live/dead subset, both channels, all three widths, cold and seeded.
    #[test]
    fn an_elided_cascade_is_the_full_cascade_bit_for_bit() {
        for case in [0_usize, 2] {
            for seeded in [false, true] {
                for live in MASKS {
                    compare::<f32, 1>("Scalar", case, live, live, seeded);
                    compare::<Simd4, 4>("Simd4", case, live, live, seeded);
                    compare::<Simd8, 8>("Simd8", case, live, live, seeded);
                }
            }
        }
    }

    /// The two channels are allowed to disagree about which sections are live, and a section is
    /// only elidable when it is identity on **both**.
    #[test]
    fn the_two_channels_are_judged_together() {
        for left_live in MASKS {
            for right_live in MASKS {
                let ran = compare::<Simd4, 4>("Simd4", 0, left_live, right_live, false);
                let live = (left_live | right_live).count_ones() as usize;
                let depth = <Simd4 as Lane>::SVF_CASCADE_DEPTH;
                let expected = live.div_ceil(depth) * depth;
                assert_eq!(
                    ran,
                    expected.min(EQ_SECTION_COUNT_V1),
                    "a section is elidable only when it is identity on both channels \
                     ({left_live:04b}/{right_live:04b})"
                );
            }
        }
    }

    /// Non-vacuity: the shapes this optimisation exists for really do shorten the cascade.
    ///
    /// One live band of four is the shipped console fixture's shape (see the intended
    /// sixty-four-track session), and it is the row the standing measurement moves.
    #[test]
    fn the_shipped_shape_actually_elides() {
        for (live, expected) in [(0b0001_u8, 2_usize), (0b0011, 2), (0b0000, 0)] {
            let left_channel = channel::<Simd8, 8>(0, live);
            let right_channel = channel::<Simd8, 8>(3, live);
            let left = block::<8>(0, 0);
            let right = block::<8>(0, 3);
            assert_eq!(
                kept(&left_channel, &right_channel, &left, &right),
                expected,
                "a bank with live mask {live:04b} should run {expected} of \
                 {EQ_SECTION_COUNT_V1} sections"
            );
        }
        // Three live sections cannot be shortened at depth two: the list has to divide by the
        // depth, and paying one identity section is what keeps a single kernel instantiation.
        let left_channel = channel::<Simd8, 8>(0, 0b0111);
        let right_channel = channel::<Simd8, 8>(3, 0b0111);
        assert_eq!(
            kept(
                &left_channel,
                &right_channel,
                &block::<8>(0, 0),
                &block::<8>(0, 3)
            ),
            EQ_SECTION_COUNT_V1,
            "three live sections round up to the whole cascade at depth two"
        );
    }

    /// A section that is identity on some lanes and live on others is not elidable.
    #[test]
    fn a_section_live_on_one_lane_is_not_elided() {
        let mut targets: [[BandTarget; EQ_SECTION_COUNT_V1]; 8] =
            core::array::from_fn(corpus::bands);
        for bands in &mut targets {
            for band in bands.iter_mut() {
                band.enabled = false;
            }
        }
        // Lane 5 alone keeps section 2.
        targets[5][2].enabled = true;
        let left_channel =
            Channel::<Simd8, 8>::new(targets, corpus::CORPUS_RATE).expect("legal design");
        let right_channel = channel::<Simd8, 8>(3, 0b0000);
        let left = block::<8>(0, 0);
        let right = block::<8>(0, 3);
        assert_eq!(
            kept(&left_channel, &right_channel, &left, &right),
            2,
            "one live lane keeps its whole section, and the depth rounds one live section to two"
        );
        // And it renders the same bits as the full cascade.
        let ran = compare::<Simd8, 8>("Simd8", 0, 0b0100, 0b0000, false);
        assert_eq!(ran, 2, "the mixed-lane case must still engage");
    }

    /// A `-0.0` anywhere in either input plane refuses the elision.
    ///
    /// This is the leg the proof rests on: an elided identity section would rewrite that `-0.0` to
    /// `+0.0`, which is a moved bit.
    #[test]
    fn a_negative_zero_input_refuses_elision() {
        for plane in 0..2 {
            for position in [0_usize, 1, FRAMES * 8 - 1] {
                let left_channel = channel::<Simd8, 8>(0, 0b0001);
                let right_channel = channel::<Simd8, 8>(3, 0b0001);
                let mut left = block::<8>(0, 0);
                let mut right = block::<8>(0, 3);
                if plane == 0 {
                    left[position] = -0.0;
                } else {
                    right[position] = -0.0;
                }
                assert_eq!(
                    kept(&left_channel, &right_channel, &left, &right),
                    EQ_SECTION_COUNT_V1,
                    "a -0.0 at word {position} of plane {plane} must refuse elision"
                );
                // A `+0.0` in the same place must not.
                let mut left = block::<8>(0, 0);
                let mut right = block::<8>(0, 3);
                if plane == 0 {
                    left[position] = 0.0;
                } else {
                    right[position] = 0.0;
                }
                assert_eq!(
                    kept(&left_channel, &right_channel, &left, &right),
                    2,
                    "a +0.0 at word {position} of plane {plane} is an ordinary sample"
                );
            }
        }
    }

    /// Non-finite input, and input above the §4.4 magnitude bound, refuse the elision.
    #[test]
    fn a_non_finite_or_oversized_input_refuses_elision() {
        for sample in [
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            1.0e31,
            -1.0e31,
            f32::MAX,
        ] {
            let left_channel = channel::<Simd8, 8>(0, 0b0001);
            let right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut left = block::<8>(0, 0);
            let right = block::<8>(0, 3);
            left[17] = sample;
            assert_eq!(
                kept(&left_channel, &right_channel, &left, &right),
                EQ_SECTION_COUNT_V1,
                "{sample} must refuse elision"
            );
        }
        // The bound is inclusive on the legal side: 1e29 is an ordinary, if absurd, sample.
        let left_channel = channel::<Simd8, 8>(0, 0b0001);
        let right_channel = channel::<Simd8, 8>(3, 0b0001);
        let mut left = block::<8>(0, 0);
        let right = block::<8>(0, 3);
        left[17] = 1.0e29;
        assert_eq!(
            kept(&left_channel, &right_channel, &left, &right),
            2,
            "a large but admissible sample does not refuse elision"
        );
    }

    /// A restored, non-`+0.0` state in a section that *would* be elided refuses the elision.
    ///
    /// The full cascade would move that state; eliding the section would not, and the state words
    /// are part of the bit-identity claim.
    #[test]
    fn a_non_zero_state_in_a_dead_section_refuses_elision() {
        for word in [1.0e-30_f32, -1.0e-30, 1.0, -0.0] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0001);
            let right_channel = channel::<Simd8, 8>(3, 0b0001);
            left_channel.sections[3].state.ic2 = Simd8::splat(word);
            let left = block::<8>(0, 0);
            let right = block::<8>(0, 3);
            assert_eq!(
                kept(&left_channel, &right_channel, &left, &right),
                EQ_SECTION_COUNT_V1,
                "a dead section holding {word} must refuse elision"
            );
        }
    }

    /// A `-0.0` integrator in a *live* section refuses the elision.
    ///
    /// Nothing the kernel writes is ever `-0.0` -- `flush` maps every zero to `+0.0` -- but a
    /// restored state payload is admitted on finiteness alone, and a low-pass section carrying
    /// `ic2 = -0.0` is the one shape that can emit `-0.0` into a later elided section.
    #[test]
    fn a_negative_zero_state_in_a_live_section_refuses_elision() {
        for word in [0_usize, 1] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0001);
            let right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut lanes = [0.0_f32; 8];
            lanes[6] = -0.0;
            let seeded = Simd8::load(&lanes);
            if word == 0 {
                left_channel.sections[0].state.ic1 = seeded;
            } else {
                left_channel.sections[0].state.ic2 = seeded;
            }
            assert_eq!(
                kept(
                    &left_channel,
                    &right_channel,
                    &block::<8>(0, 0),
                    &block::<8>(0, 3)
                ),
                EQ_SECTION_COUNT_V1,
                "a live section holding -0.0 in integrator {word} must refuse elision"
            );
        }
    }

    /// The `-0.0` refusal is load-bearing: forced past it, the bits really do move.
    ///
    /// Without this the gate could be decorative -- a refusal that never mattered would leave
    /// every other test in this module green. Here the elided cascade is run *directly*, with the
    /// section list the gate would have produced had it not refused, and the result is asserted to
    /// **differ** from the full cascade. That difference is exactly the `-0.0` an identity section
    /// rewrites to `+0.0`.
    #[test]
    fn the_negative_zero_refusal_is_load_bearing() {
        let mut moved = 0_usize;
        for stationary in [true, false] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0000);
            let mut right_channel = channel::<Simd8, 8>(3, 0b0000);
            let mut left = vec![-0.0_f32; FRAMES * 8];
            let mut right = vec![-0.0_f32; FRAMES * 8];
            if stationary {
                // The list the gate refuses to hand out: an all-identity cascade elides to zero
                // sections, so the elided arm writes nothing at all.
                super::interleave::<Simd8, 8, 2>(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    ([0, 1, 2, 3], 0),
                );
            } else {
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    false,
                );
            }
            moved += usize::from(left[0].to_bits() == 0x8000_0000);
        }
        assert_eq!(
            moved, 1,
            "the elided arm must keep the -0.0 the full cascade rewrites to +0.0; if both arms \
             agree the gate is not protecting anything"
        );
        // And the gate does refuse this block.
        let left_channel = channel::<Simd8, 8>(0, 0b0000);
        let right_channel = channel::<Simd8, 8>(3, 0b0000);
        let negative = vec![-0.0_f32; FRAMES * 8];
        assert_eq!(
            kept(&left_channel, &right_channel, &negative, &negative),
            EQ_SECTION_COUNT_V1,
            "an all -0.0 block is refused"
        );
    }

    /// Enabling and disabling a band mid-session keeps the flag honest, and every block renders
    /// exactly what the full cascade renders.
    ///
    /// The transition arrives the way automation delivers it -- through `start_ramp`, which ramps
    /// the six words over the smoothing window -- so the sequence covers the non-stationary blocks
    /// during the ramp, the snap that ends it, and the stationary blocks on either side.
    #[test]
    fn a_mid_session_enable_or_disable_stays_bit_exact() {
        const BLOCKS: usize = 8;
        let mut engaged = 0_usize;
        let mut arms = Vec::new();
        for elide in [true, false] {
            let mut left_channel = channel::<Simd8, 8>(0, 0b0001);
            let mut right_channel = channel::<Simd8, 8>(3, 0b0001);
            let mut rendered: Vec<u32> = Vec::new();
            for step in 0..BLOCKS {
                if step == 2 {
                    // Band 3 comes on, on every lane of both channels.
                    for lane in 0..8 {
                        let mut band = corpus::bands(lane % corpus::LANES)[2];
                        band.enabled = true;
                        let words = band.words(corpus::CORPUS_RATE).expect("legal design");
                        left_channel.start_ramp(2, lane, words);
                        right_channel.start_ramp(2, lane, words);
                    }
                }
                if step == 5 {
                    // And goes off again.
                    for lane in 0..8 {
                        left_channel.start_ramp(2, lane, EqSvfWordsV1::IDENTITY);
                        right_channel.start_ramp(2, lane, EqSvfWordsV1::IDENTITY);
                    }
                }
                let mut left = block::<8>(0, step % corpus::LANES);
                let mut right = block::<8>(0, (step + 3) % corpus::LANES);
                let stationary =
                    left_channel.no_ramp_in_flight() && right_channel.no_ramp_in_flight();
                // The two arms differ only in whether the stationary path may shorten the
                // cascade; `elide = false` forces the per-section path, which never does.
                let stationary = stationary && elide;
                if stationary
                    && kept(&left_channel, &right_channel, &left, &right) < EQ_SECTION_COUNT_V1
                {
                    engaged += 1;
                }
                process_channels(
                    (&mut left_channel, &mut right_channel),
                    &mut left,
                    &mut right,
                    FRAMES,
                    stationary,
                );
                rendered.extend(bits(&left));
                rendered.extend(bits(&right));
            }
            rendered.extend(integrators(&left_channel, &right_channel));
            arms.push(rendered);
        }
        assert_eq!(
            arms[0], arms[1],
            "an enable/disable transition must render the full cascade's bits at every block"
        );
        assert!(
            engaged >= 4,
            "the transition sequence must actually engage the elision on its stationary blocks, \
             engaged on {engaged} of {BLOCKS}"
        );
    }
}
