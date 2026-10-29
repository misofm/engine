//! Launch dynamic EQ (issue #17 S1): four dual-mono bell bands, each with its own detector and
//! gain law, zero latency.
//!
//! One generic kernel body owns the per-sample graph ([`kernel`]); this module is its control
//! plane: the frozen descriptor, preparation, the live-word designs, the state payload and the
//! once-per-block boundary policy. The algorithm, its limits and its sources are
//! `dsp-research/dynamic-eq.md`.
//!
//! # Shapes
//!
//! A prepared instance is `DynamicEq<L, CONNECTED>`. The scalar instance is `L = f32`, the
//! `WIDTH = 1` instantiation of the same kernel, so it is the bit oracle of the four- and
//! eight-lane banks. `CONNECTED` gives every band a detector SVF on the sidechain, which a band
//! with the external detector reads.
//!
//! # Bands and parameters
//!
//! Band `b` (0 to 3) owns parameters `b * 12 .. b * 12 + 12` in descriptor order and the stable
//! IDs `b * 16 + 1 ..= b * 16 + 12`: enabled, frequency, Q, gain (the static bell gain), threshold,
//! ratio, range, attack, release, direction, detector and knee. A band's pole words are the
//! parametric EQ's bell (`parametric_eq::design_svf`) at its static gain, rounded once; the
//! dynamics move only the band mix word, so with no change the band is #87's bell.
//!
//! # Live values
//!
//! All 48 parameters are live block-rate values, each a 64-sample ramp of the words the kernel
//! reads (`kernel::BAND_RAMPS` per band). A record designs its targets once on the block's control
//! side (`targets`); the render only adds. A disabled band ramps its six filter words to the
//! parametric EQ's identity section.
//!
//! # Realtime rules
//!
//! Preparation allocates the instance. `process` and `process_bank` allocate nothing, take no lock
//! and make no syscall; a record's design runs the vendored `math` `tan`, `pow` and `exp`.
#![allow(missing_docs)]

pub mod kernel;

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, EnumChoice, InitialParameterValue,
    LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory, NodeTailBound, ObservationCadence,
    ObservationChannels, ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind,
    ObservationSample, ObservationTapId, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffect, PreparedEffectBank,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, PreparedSidechainPort,
    ProcessReport, ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput,
    StatePayloadOutput, StatePayloadSizes, expected_prepared_metadata,
};
use effect_runtime::bank::{
    block_is_positive_zero, check_block, nonfinite_lane_mask, positive_zero_lanes,
};
use effect_runtime::dynamic_band::WordRamp;
use effect_runtime::envelope::attack_release_coefficient;
use effect_runtime::params::{
    ParameterKind, ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload as payload;
use effect_runtime::svf::{RAMP_PATH_NORM_TOLERANCE, transition_norm};
use lane::kernels::{SvfState, silence_advance, silence_skip_block, svf_state_held};
use lane::{Backend, Lane, Simd4, silence_armable_holding};

use kernel::{
    Args, BAND_RAMPS, BANDS, C1, Chan, Coef, DC1, DESIGN_WORDS, LINK_COUNT, MAX_WIDTH, RAMPS,
    refresh_knees, segment,
};

/// Parameters per band.
const BAND_PARAMETERS: usize = 12;

const PARAMETER_COUNT: usize = BANDS * BAND_PARAMETERS;

/// Samples every live word takes to reach a new target (decision 15: every live value ramps).
const RAMP_SAMPLES: u32 = 64;

/// Signal words per band in the payload: the bell, detector and sidechain SVF integrators, the gain
/// word.
const BAND_SIGNAL_WORDS: usize = 7;

/// Signal words per channel: every band's, then the two silence clocks.
const SIGNAL_WORDS: usize = BANDS * BAND_SIGNAL_WORDS + 2;

/// The state words of one channel the payload carries: the signal words, then each ramp's
/// `current`, `target`, `step` and `remaining`. The derived knee words are not carried.
const CHAN_WORDS: usize = SIGNAL_WORDS + RAMPS * 4;

/// One channel section of the payload: the signal words, the committed values, the ramp words,
/// then the two link weight quadruples (issue #1969, #1370 D6). The lane's link mode is the
/// weights' targets (`effect_contract::link_glide_mode`), so it takes no word of its own.
const STATE_LANE_WORDS: usize = STATE_LINK_WORD + LINK_COUNT * 4;

/// Word index of the first link weight quadruple (`w_link`, then `w_avg`) in a channel section.
const STATE_LINK_WORD: usize = CHAN_WORDS + PARAMETER_COUNT;

/// The saturated silence clock (`lane::kernels::silence_advance`).
const RUN_MAX: f32 = 16_777_216.0;

/// The largest range, which bounds every gain word.
const RANGE_MAX_DB: f32 = 24.0;

const fn effect_id(value: &'static str) -> effect_contract::EffectId {
    match effect_contract::EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static effect identifier"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("valid static port identifier"),
    }
}

/// The value both enumerations use for their second choice (up, external).
const SECOND_CHOICE: f32 = 2.0;

const DIRECTION_CHOICES: [EnumChoice; 2] = [
    EnumChoice {
        value: 1.0,
        label: "down",
    },
    EnumChoice {
        value: SECOND_CHOICE,
        label: "up",
    },
];

/// The detector choices. `external` reads the connected sidechain through the band's own filter;
/// with no sidechain connected it falls back to the band's own tap.
const DETECTOR_CHOICES: [EnumChoice; 2] = [
    EnumChoice {
        value: 1.0,
        label: "internal",
    },
    EnumChoice {
        value: SECOND_CHOICE,
        label: "external",
    },
];

use ParameterDomain::{Boolean, Continuous, Enumeration};
use ParameterMapping::{Linear, Logarithmic, Stepped};

/// One band field's frozen shape: name suffix, display name, unit, domain, bounds, mapping.
struct Field {
    display_unit: &'static str,
    unit: ParameterUnit,
    domain: ParameterDomain,
    range: Option<(f32, f32)>,
    default_value: f32,
    mapping: ParameterMapping,
    enum_choices: &'static [EnumChoice],
}

const fn field(
    display_unit: &'static str,
    unit: ParameterUnit,
    domain: ParameterDomain,
    range: Option<(f32, f32)>,
    default_value: f32,
    mapping: ParameterMapping,
    enum_choices: &'static [EnumChoice],
) -> Field {
    Field {
        display_unit,
        unit,
        domain,
        range,
        default_value,
        mapping,
        enum_choices,
    }
}

/// The twelve fields in descriptor order. The frequency default is per band
/// ([`FREQUENCY_DEFAULTS`]).
const FIELDS: [Field; BAND_PARAMETERS] = [
    field(
        "on/off",
        ParameterUnit::Linear,
        Boolean,
        None,
        1.0,
        Stepped,
        &[],
    ),
    field(
        "Hz",
        ParameterUnit::Hz,
        Continuous,
        Some((20.0, 20_000.0)),
        1_000.0,
        Logarithmic,
        &[],
    ),
    field(
        "Q",
        ParameterUnit::Ratio,
        Continuous,
        Some((0.1, 16.0)),
        1.0,
        Logarithmic,
        &[],
    ),
    field(
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((-24.0, 24.0)),
        0.0,
        Linear,
        &[],
    ),
    field(
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((-60.0, 0.0)),
        -20.0,
        Linear,
        &[],
    ),
    field(
        "ratio",
        ParameterUnit::Ratio,
        Continuous,
        Some((1.0, 20.0)),
        2.0,
        Logarithmic,
        &[],
    ),
    field(
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((0.0, RANGE_MAX_DB)),
        12.0,
        Linear,
        &[],
    ),
    field(
        "ms",
        ParameterUnit::Milliseconds,
        Continuous,
        Some((0.1, 200.0)),
        5.0,
        Logarithmic,
        &[],
    ),
    field(
        "ms",
        ParameterUnit::Milliseconds,
        Continuous,
        Some((5.0, 2_000.0)),
        100.0,
        Logarithmic,
        &[],
    ),
    field(
        "type",
        ParameterUnit::Linear,
        Enumeration,
        None,
        1.0,
        Stepped,
        &DIRECTION_CHOICES,
    ),
    field(
        "type",
        ParameterUnit::Linear,
        Enumeration,
        None,
        1.0,
        Stepped,
        &DETECTOR_CHOICES,
    ),
    field(
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((0.0, 12.0)),
        6.0,
        Linear,
        &[],
    ),
];

/// Field indices inside a band, in descriptor order.
const ENABLED: usize = 0;
const FREQUENCY: usize = 1;
const Q: usize = 2;
const GAIN: usize = 3;
const THRESHOLD: usize = 4;
const RATIO: usize = 5;
const RANGE: usize = 6;
const ATTACK: usize = 7;
const RELEASE: usize = 8;
const DIRECTION: usize = 9;
const DETECTOR: usize = 10;
const KNEE: usize = 11;

/// Default centre frequency of each band.
const FREQUENCY_DEFAULTS: [f32; BANDS] = [200.0, 1_000.0, 3_000.0, 8_000.0];

macro_rules! band_names {
    ($prefix:literal, $join:literal, [$($field:literal),* $(,)?]) => {
        [$(concat!($prefix, "1", $join, $field),)* $(concat!($prefix, "2", $join, $field),)*
         $(concat!($prefix, "3", $join, $field),)* $(concat!($prefix, "4", $join, $field),)*]
    };
}

/// The stable machine name of each parameter, in descriptor order.
const MACHINE_NAMES: [&str; PARAMETER_COUNT] = band_names!(
    "band_",
    "_",
    [
        "enabled",
        "frequency_hz",
        "q",
        "gain_db",
        "threshold_db",
        "ratio",
        "range_db",
        "attack_ms",
        "release_ms",
        "direction",
        "detector",
        "knee_db",
    ]
);

/// The display name of each parameter, in descriptor order.
const DISPLAY_NAMES: [&str; PARAMETER_COUNT] = band_names!(
    "band ",
    " ",
    [
        "enabled",
        "frequency",
        "Q",
        "gain",
        "threshold",
        "ratio",
        "range",
        "attack",
        "release",
        "direction",
        "detector",
        "knee",
    ]
);

const fn parameter(index: usize) -> ParameterDescriptor {
    let band = index / BAND_PARAMETERS;
    let slot = index % BAND_PARAMETERS;
    let f = &FIELDS[slot];
    let (minimum, maximum) = match f.range {
        Some((low, high)) => (Some(low), Some(high)),
        None => (None, None),
    };
    ParameterDescriptor {
        id: match ParameterId::new(band as u32 * 16 + slot as u32 + 1) {
            Some(id) => id,
            None => panic!("nonzero static parameter identifier"),
        },
        name: MACHINE_NAMES[index],
        display_name: DISPLAY_NAMES[index],
        display_unit: f.display_unit,
        unit: f.unit,
        domain: f.domain,
        minimum,
        maximum,
        default_value: if slot == FREQUENCY {
            FREQUENCY_DEFAULTS[band]
        } else {
            f.default_value
        },
        mapping: f.mapping,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices: f.enum_choices,
        lattice: effect_contract::default_parameter_lattice(f.unit, f.domain, f.mapping),
        prepared_reason: None,
    }
}

/// Frozen dynamic EQ parameters: band `b`'s twelve at positions `b * 12 ..`, IDs `b * 16 + 1 ..`.
pub const DYNAMIC_EQ_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = {
    let mut table = [parameter(0); PARAMETER_COUNT];
    let mut index = 1;
    while index < PARAMETER_COUNT {
        table[index] = parameter(index);
        index += 1;
    }
    table
};

/// The two enumerations' values, in the runtime's vocabulary.
const CHOICE_VALUES: [f32; 2] = [1.0, SECOND_CHOICE];

/// One parameter's domain in `effect_runtime::params`'s vocabulary, read from the descriptor so
/// the two cannot drift. The mapping is a control-surface concern no render path reads.
const fn spec(index: usize) -> ParameterSpec {
    let d = &DYNAMIC_EQ_PARAMETERS[index];
    let (kind, minimum, maximum) = match (d.domain, d.minimum, d.maximum) {
        (Continuous, Some(low), Some(high)) => (ParameterKind::Continuous, low, high),
        (Boolean, _, _) => (ParameterKind::Boolean, 0.0, 1.0),
        _ => (
            ParameterKind::Enumeration(&CHOICE_VALUES),
            1.0,
            SECOND_CHOICE,
        ),
    };
    ParameterSpec {
        kind,
        minimum,
        maximum,
        mapping: effect_runtime::params::ParameterMapping::Linear,
        default: d.default_value,
    }
}

const SPECS: [ParameterSpec; PARAMETER_COUNT] = {
    let mut specs = [spec(0); PARAMETER_COUNT];
    let mut index = 1;
    while index < PARAMETER_COUNT {
        specs[index] = spec(index);
        index += 1;
    }
    specs
};

const PORTS: [PortDescriptor; 3] = [
    PortDescriptor {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptor {
        id: port_id("sidechain-in"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];

/// The one quality: causal, zero latency, one channel section per lane and the common header.
const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    let per_lane = (STATE_LANE_WORDS * payload::WORD_BYTES) as u32;
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: payload::HEADER_WORDS * payload::WORD_BYTES as u32,
            left_bytes: per_lane,
            right_bytes: per_lane,
        },
        scratch_fixed_bytes: 0,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// No certified bound yet, as the de-esser (#15 S1) and the parametric EQ: every band carries a
/// live (frequency-, Q- and gain-ramped) SVF ring-out, whose exact-rest bound is the parametric
/// EQ's #1372 derivation, so S1 states the conservative [`NodeTailBound::UNBOUNDED`]. The silent
/// fast path does not need a bound: `lanes_at_rest` reads state.
fn tail_and_rest(_sample_rate: u32, _quality: EffectQuality) -> NodeTailBound {
    NodeTailBound::UNBOUNDED
}

/// State payload layout version.
pub const STATE_LAYOUT_VERSION: u32 = 1;

const fn observation(id: u32, display_name: &'static str) -> ObservationDescriptor {
    ObservationDescriptor {
        id: ObservationTapId(id),
        display_name,
        display_unit: "dB",
        kind: ObservationKind::GainChangeDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakSigned,
        channels: ObservationChannels::PerLane,
        minimum: -RANGE_MAX_DB,
        maximum: RANGE_MAX_DB,
        bypass_observation: Some(0.0),
    }
}

/// One tap per band: the smoother's own change word, negative for a cut, positive for a boost.
/// It is a signed gain change (#1918): the `PeakSigned` fold keeps the sign, so an upward band
/// reads as a boost, never as a reduction (#17 S2).
pub const DYNAMIC_EQ_OBSERVATIONS: [ObservationDescriptor; BANDS] = [
    observation(1, "Band 1 Gain Change"),
    observation(2, "Band 2 Gain Change"),
    observation(3, "Band 3 Gain Change"),
    observation(4, "Band 4 Gain Change"),
];

/// Immutable descriptor for the launch dynamic EQ contract.
pub const DYNAMIC_EQ_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.dynamic-eq"),
    display_name: "Dynamic EQ",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    lane_link: true,
    parameters: &DYNAMIC_EQ_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &DYNAMIC_EQ_OBSERVATIONS,
};

/// The interval each band word stays inside, for a restore's path check, in
/// [`kernel`]'s ramp order: the threshold's domain, the slope's, the knee's and the range's, every
/// rate design over the time domains at every launch rate (the 2 s release at 96 kHz, `5.2e-6`,
/// to the 0.1 ms attack at 44.1 kHz, `0.2029`), the SVF words' `(0, 1)`, `k A^2 = A/Q` inside
/// `[0, 40)` and `m1 = k (A^2 - 1)` inside `(-40, 40)` over the Q and gain domains (`A = 10^(g/40)`
/// in `[0.251, 3.99]`, `Q` in `[0.1, 16]`; `0` is the disabled band), the switches' `[0, 1]`, and
/// the detector's SVF words in `(0, 1)` and `1/Q` in `[0, 10]`.
const WORD_BOUNDS: [(f32, f32); BAND_RAMPS] = [
    (-60.0, 0.0),
    (1.0 / 20.0 - 1.0, 0.0),
    (0.0, 12.0),
    (0.0, RANGE_MAX_DB),
    (5.0e-6, 0.203),
    (5.0e-6, 0.203),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (-40.0, 40.0),
    (0.0, 40.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 10.0),
];

/// The nine filter words of one band, in [`DESIGN_WORDS`] order: the parametric EQ's bell design
/// at the static gain (`c1, a2, a3, m1` exactly as `design_svf` rounds them) and `k A^2 = A/Q` in
/// `f64` from the same amplitude, rounded once; then the detector SVF at the band's own quality
/// `Q` whatever the static gain (`parametric_eq::design_svf_words_f64`'s 0 dB high-pass row, whose
/// `c1, a2, a3` depend on `g = tan(pi f / fs)` and `k = 1/Q` only, and `k` its negated band mix),
/// rounded once. At a static 0 dB the detector words are the bell's own, bit for bit. A disabled
/// band is the EQ's identity section and a detector of zero words.
fn band_design(values: &[f32], sample_rate: u32) -> [f32; 9] {
    if values[ENABLED] == 0.0 {
        return [0.0; 9];
    }
    let (frequency, gain, q) = (values[FREQUENCY], values[GAIN], values[Q]);
    // Every committed value is inside its descriptor domain, which is inside the EQ's own
    // (frequency 10 Hz to 20 kHz, below every launch rate's Nyquist; gain +-24 dB; Q 0.1 to 18),
    // so the design cannot fail; the identity is the defensive answer, never reached.
    let words = parametric_eq::design_svf(
        parametric_eq::EqBandKind::Bell,
        frequency,
        gain,
        q,
        1.0,
        engine::SampleRateHz(sample_rate),
    );
    debug_assert!(words.is_ok(), "an in-domain bell design");
    let Ok(words) = words else {
        return [0.0; 9];
    };
    let detector = parametric_eq::design_svf_words_f64(
        parametric_eq::EqBandKind::HighPass,
        f64::from(frequency),
        0.0,
        f64::from(q),
        1.0,
        f64::from(sample_rate),
    );
    let amplitude = math::pow(10.0, f64::from(gain) / 40.0);
    [
        words.c1,
        words.a2,
        words.a3,
        words.m1,
        (amplitude / f64::from(q)) as f32,
        detector[0] as f32,
        detector[1] as f32,
        detector[2] as f32,
        (-detector[4]) as f32,
    ]
}

/// The words the committed `values` design, in [`kernel`]'s ramp order. Out of line, so the
/// scalar `tan`, `pow` and `exp` stay out of `process_bank`.
#[inline(never)]
fn targets(values: &[f32; PARAMETER_COUNT], sample_rate: u32) -> [f32; RAMPS] {
    let on = |value: f32| if value == SECOND_CHOICE { 1.0 } else { 0.0 };
    let mut words = [0.0_f32; RAMPS];
    for (band, out) in words.as_chunks_mut::<BAND_RAMPS>().0.iter_mut().enumerate() {
        let v = &values[band * BAND_PARAMETERS..(band + 1) * BAND_PARAMETERS];
        let design = band_design(v, sample_rate);
        out[kernel::THRESHOLD] = v[THRESHOLD];
        out[kernel::SLOPE] = 1.0 / v[RATIO] - 1.0;
        out[kernel::KNEE] = v[KNEE];
        out[kernel::RANGE] = v[RANGE];
        out[kernel::ATTACK] = attack_release_coefficient(v[ATTACK], sample_rate);
        out[kernel::RELEASE] = attack_release_coefficient(v[RELEASE], sample_rate);
        for (&index, word) in DESIGN_WORDS.iter().zip(design) {
            out[index] = word;
        }
        out[kernel::UPWARD] = on(v[DIRECTION]);
        out[kernel::EXTERNAL] = on(v[DETECTOR]);
    }
    words
}

/// Factory for the launch dynamic EQ.
#[derive(Clone, Copy, Debug, Default)]
pub struct DynamicEqFactory;

/// A prepared dynamic EQ at one lane width. It keeps only the prepared values its own methods read.
struct DynamicEq<L: Lane, const CONNECTED: bool> {
    sample_rate: u32,
    automation_capacity: u32,
    bank_width: Option<BankWidth>,
    /// Each lane's prepared values, the seed of a full reset and of the D7 recovery.
    defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    /// Each lane's committed values, which design the ramp targets.
    values: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    coef: [Coef<L>; 2],
    state: [Chan<L>; 2],
    /// At least every lane's `remaining`: where a block's ramping segment ends.
    ramp_frames_left: u32,
    /// Bit `l` set when lane `l` carries a member (issue #1092).
    active: u32,
    /// Whether a collapsed block (`run_block::<true>`) has rendered since the last disengage copy
    /// (`desymmetrize_channels`), which alone clears it (#1894). While it is set the right channel
    /// is frozen and stale, so [`rest_lanes`](Self::rest_lanes) reads the left channel alone and a
    /// skipped block advances the left channel's silence clocks alone, as the collapsed silent
    /// path does: the effect contract's collapsed-bank rule.
    collapsed: bool,
}

fn lane_get<L: Lane>(value: L, lane: usize) -> f32 {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane]
}

fn lane_set<L: Lane>(value: &mut L, lane: usize, sample: f32) {
    let mut words = [0.0_f32; MAX_WIDTH];
    value.store(&mut words[..L::WIDTH]);
    words[lane] = sample;
    *value = L::load(&words[..L::WIDTH]);
}

/// Every payload word of one channel's state, in payload order ([`CHAN_WORDS`]).
fn chan_words<L: Lane>(s: &Chan<L>) -> [L; CHAN_WORDS] {
    let mut words = [L::zero(); CHAN_WORDS];
    for (chunk, band) in words[..BANDS * BAND_SIGNAL_WORDS]
        .as_chunks_mut::<BAND_SIGNAL_WORDS>()
        .0
        .iter_mut()
        .zip(&s.bands)
    {
        chunk.copy_from_slice(&[
            band.main.ic1,
            band.main.ic2,
            band.detector.ic1,
            band.detector.ic2,
            band.side.ic1,
            band.side.ic2,
            band.gain_db,
        ]);
    }
    words[SIGNAL_WORDS - 2] = s.run;
    words[SIGNAL_WORDS - 1] = s.side_run;
    for (chunk, ramp) in words[SIGNAL_WORDS..]
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(&s.ramps)
    {
        chunk.copy_from_slice(&[ramp.current, ramp.target, ramp.step, ramp.remaining]);
    }
    words
}

/// The inverse of [`chan_words`], with the derived knee words designed from the knee ramps.
fn chan_from_words<L: Lane>(words: &[L; CHAN_WORDS]) -> Chan<L> {
    let mut s = Chan::default();
    for (band, chunk) in s.bands.iter_mut().zip(
        words[..BANDS * BAND_SIGNAL_WORDS]
            .as_chunks::<BAND_SIGNAL_WORDS>()
            .0
            .iter(),
    ) {
        [band.main.ic1, band.main.ic2] = [chunk[0], chunk[1]];
        [band.detector.ic1, band.detector.ic2] = [chunk[2], chunk[3]];
        [band.side.ic1, band.side.ic2] = [chunk[4], chunk[5]];
        band.gain_db = chunk[6];
    }
    [s.run, s.side_run] = [words[SIGNAL_WORDS - 2], words[SIGNAL_WORDS - 1]];
    for (ramp, chunk) in s
        .ramps
        .iter_mut()
        .zip(words[SIGNAL_WORDS..].as_chunks::<4>().0.iter())
    {
        [ramp.current, ramp.target, ramp.step, ramp.remaining] =
            [chunk[0], chunk[1], chunk[2], chunk[3]];
    }
    refresh_knees(&mut s);
    s
}

/// One lane of one channel's state words.
fn lane_words<L: Lane>(s: &Chan<L>, lane: usize) -> [f32; CHAN_WORDS] {
    chan_words(s).map(|word| lane_get(word, lane))
}

/// Writes one lane of one channel's state words. The link weights are not among them and stay.
fn set_lane_words<L: Lane>(s: &mut Chan<L>, lane: usize, values: &[f32; CHAN_WORDS]) {
    let mut words = chan_words(s);
    for (word, &value) in words.iter_mut().zip(values) {
        lane_set(word, lane, value);
    }
    let links = s.links;
    *s = chan_from_words(&words);
    s.links = links;
}

/// One lane of one link weight: `current`, `target`, `step`, `remaining`.
fn link_lane<L: Lane>(ramp: &WordRamp<L>, lane: usize) -> [f32; 4] {
    [ramp.current, ramp.target, ramp.step, ramp.remaining].map(|word| lane_get(word, lane))
}

/// Writes one lane of one link weight.
fn set_link_lane<L: Lane>(ramp: &mut WordRamp<L>, lane: usize, words: [f32; 4]) {
    lane_set(&mut ramp.current, lane, words[0]);
    lane_set(&mut ramp.target, lane, words[1]);
    lane_set(&mut ramp.step, lane, words[2]);
    lane_set(&mut ramp.remaining, lane, words[3]);
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

fn codec_error(error: payload::StatePayloadError) -> StatePayloadError {
    StatePayloadError { code: error.code }
}

/// `true` for a finite whole number in `[0, limit]`.
fn integral_within(value: f32, limit: f32) -> bool {
    value.is_finite() && (0.0..=limit).contains(&value) && value == value.floor()
}

const fn all_lanes(width: usize) -> u32 {
    (1 << width) - 1
}

/// The lanes on which band `band` of `s` holds the identity section, settled: its six filter
/// words are `+0.0` and none of them is ramping. Such a band's SVF integrates nothing (its
/// integrators are frozen), its tap and mix word are zero and its output is its input.
fn identity_lanes<L: Lane>(s: &Chan<L>, band: usize) -> u32 {
    DESIGN_WORDS
        .iter()
        .map(|&index| &s.ramps[band * BAND_RAMPS + index])
        .fold(all_lanes(L::WIDTH), |mask, ramp| {
            mask & positive_zero_lanes(ramp.current) & positive_zero_lanes(ramp.remaining)
        })
}

/// The lanes on which band `band`'s three SVFs and gain word are `+0.0`.
fn band_rest_lanes<L: Lane>(s: &Chan<L>, band: usize) -> u32 {
    let b = &s.bands[band];
    [
        b.main.ic1,
        b.main.ic2,
        b.detector.ic1,
        b.detector.ic2,
        b.side.ic1,
        b.side.ic2,
        b.gain_db,
    ]
    .into_iter()
    .fold(all_lanes(L::WIDTH), |mask, word| {
        mask & positive_zero_lanes(word)
    })
}

impl<L: Lane, const CONNECTED: bool> DynamicEq<L, CONNECTED> {
    fn layout(&self) -> payload::StateLayout {
        payload::StateLayout {
            version: STATE_LAYOUT_VERSION,
            common_words: 0,
            lane_words: STATE_LANE_WORDS as u32,
        }
    }

    fn new(
        metadata: &PreparedEffectMetadata,
        bank_width: Option<BankWidth>,
        defaults: [[[f32; PARAMETER_COUNT]; 2]; MAX_WIDTH],
    ) -> Option<Self> {
        let connected = matches!(
            metadata.ports.sidechain,
            PreparedSidechainPort::Connected { .. }
        );
        if connected != CONNECTED {
            return None;
        }
        let word = |on: bool| L::splat(f32::from(u8::from(on)));
        let mut effect = Self {
            sample_rate: metadata.sample_rate,
            automation_capacity: metadata.automation_capacity,
            bank_width,
            defaults,
            values: defaults,
            coef: [Coef {
                bypass: word(metadata.bypass),
                link_max: word(metadata.link_mode == LinkMode::Maximum),
                link_avg: word(metadata.link_mode == LinkMode::Average),
            }; 2],
            state: [Chan::default(); 2],
            ramp_frames_left: 0,
            active: all_lanes(L::WIDTH),
            collapsed: false,
        };
        for channel in 0..2 {
            for lane in 0..L::WIDTH {
                effect.reset_lane_full(channel, lane);
            }
        }
        for lane in 0..L::WIDTH {
            effect.rest_link(lane);
        }
        Some(effect)
    }

    /// Sets each lane's link words from its own request's mode (issue #1368).
    fn with_lane_links(mut self, links: &[LinkMode]) -> Self {
        for (lane, &link) in links.iter().enumerate().take(L::WIDTH) {
            self.set_lane_link(lane, link);
            self.rest_link(lane);
        }
        self
    }

    /// Seats lane `lane`'s link weights at rest at its mode on both channels (issue #1969, #1370
    /// D2): preparation, a full reset and a lane whose mode a lane swap replaced. The D7 recovery
    /// (`reset_lane_full` of one channel) leaves a glide in flight alone.
    fn rest_link(&mut self, lane: usize) {
        let weights = effect_contract::link_glide_weights(self.lane_link(lane));
        for state in &mut self.state {
            for (ramp, &value) in state.links.iter_mut().zip(&weights) {
                set_link_lane(ramp, lane, [value, value, 0.0, 0.0]);
            }
        }
    }

    // REALTIME_POLICY_BEGIN: #1969, a link glide is retargeted at a block boundary.
    /// Glides lane `lane`'s detector link to `mode` over `samples` samples (issue #1969, #1370
    /// D1-D3), on every band at once.
    ///
    /// The lane's link words become `mode`'s at once, so a glide that arrives lands on `mode`'s
    /// own law; the weights, retargeted from the words in force, carry the transition, and the
    /// bank's ramp clock covers it. The mode is the one a full reset keeps and a lane swap moves.
    /// `false`, changing nothing, for a lane past the width or padded, a mode the descriptor does
    /// not support or a glide longer than `LINK_GLIDE_MAX_SAMPLES`.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= L::WIDTH
            || !self.is_active(lane)
            || !DYNAMIC_EQ_DESCRIPTOR.lane_link
            || !DYNAMIC_EQ_DESCRIPTOR.supported_link_modes.contains(mode)
            || samples > effect_contract::LINK_GLIDE_MAX_SAMPLES
        {
            return false;
        }
        self.set_lane_link(lane, mode);
        let weights = effect_contract::link_glide_weights(mode);
        for state in &mut self.state {
            for (slot, &weight) in state.links.iter_mut().zip(&weights) {
                let [current, target, step, remaining] = link_lane(slot, lane);
                let mut ramp = LinearRamp {
                    current,
                    target,
                    step,
                    remaining: remaining as u32,
                };
                ramp.set_target(weight, samples);
                set_link_lane(
                    slot,
                    lane,
                    [ramp.current, ramp.target, ramp.step, ramp.remaining as f32],
                );
                self.ramp_frames_left = self.ramp_frames_left.max(ramp.remaining);
            }
        }
        true
    }

    /// Whether a link glide is in flight on any lane of the channels the block renders: the
    /// left alone on a collapsed block, whose right state is frozen.
    fn gliding<const MONO: bool>(&self) -> bool {
        let channels = if MONO { 1 } else { 2 };
        self.state[..channels].iter().any(|state| {
            state
                .links
                .iter()
                .any(|ramp| positive_zero_lanes(ramp.remaining) != all_lanes(L::WIDTH))
        })
    }
    // REALTIME_POLICY_END

    fn set_lane_link(&mut self, lane: usize, link: LinkMode) {
        for coef in &mut self.coef {
            lane_set(
                &mut coef.link_max,
                lane,
                f32::from(u8::from(link == LinkMode::Maximum)),
            );
            lane_set(
                &mut coef.link_avg,
                lane,
                f32::from(u8::from(link == LinkMode::Average)),
            );
        }
    }

    fn lane_link(&self, lane: usize) -> LinkMode {
        if lane_get(self.coef[0].link_max, lane) > 0.0 {
            LinkMode::Maximum
        } else if lane_get(self.coef[0].link_avg, lane) > 0.0 {
            LinkMode::Average
        } else {
            LinkMode::DualMono
        }
    }

    const fn is_active(&self, lane: usize) -> bool {
        self.active & (1 << lane) != 0
    }

    // REALTIME_POLICY_BEGIN
    /// `ResetKind::FullToDefaults` for one lane and channel, and the D7 recovery: the prepared
    /// values are committed again, every signal word is `+0.0` and every ramp rests at its design.
    fn reset_lane_full(&mut self, channel: usize, lane: usize) {
        self.values[lane][channel] = self.defaults[lane][channel];
        let mut words = [0.0_f32; CHAN_WORDS];
        let designs = targets(&self.values[lane][channel], self.sample_rate);
        for (chunk, design) in words[SIGNAL_WORDS..]
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(designs)
        {
            chunk[..2].copy_from_slice(&[design, design]);
        }
        set_lane_words(&mut self.state[channel], lane, &words);
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// `ResetKind::DiscontinuityKeepParameters` for one lane and channel: the signal words clear,
    /// every ramp snaps to its target, the committed values stay.
    fn reset_lane_discontinuity(&mut self, channel: usize, lane: usize) {
        let mut words = lane_words(&self.state[channel], lane);
        words[..SIGNAL_WORDS].fill(0.0);
        for chunk in words[SIGNAL_WORDS..].as_chunks_mut::<4>().0.iter_mut() {
            chunk[0] = chunk[1];
            chunk[2..].fill(0.0);
        }
        set_lane_words(&mut self.state[channel], lane, &words);
    }
    // REALTIME_POLICY_END

    fn reset(&mut self, kind: ResetKind) {
        for lane in 0..L::WIDTH {
            self.reset_lane(lane, kind);
        }
        self.ramp_frames_left = 0;
    }

    // REALTIME_POLICY_BEGIN
    /// `PreparedNativeEffectBank::reset_lane` (issue #2131): lane `lane`'s part of
    /// [`reset`](Self::reset) on both channels, its silence clocks included, and nothing of any
    /// other lane. The bank-level `ramp_frames_left` stays: it only chooses where a block's
    /// ramping segment ends, and its invariant (`>=` every lane's `remaining`) still holds once
    /// this lane's ramps rest. `collapsed` stays as the whole-bank reset leaves it. A lane past
    /// the width changes nothing. Bounded by one lane's words.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= L::WIDTH {
            return;
        }
        for channel in 0..2 {
            match kind {
                ResetKind::FullToDefaults => self.reset_lane_full(channel, lane),
                ResetKind::DiscontinuityKeepParameters => {
                    self.reset_lane_discontinuity(channel, lane);
                    // Issue #1969: a glide in flight snaps to its target, the lane's mode.
                    for ramp in &mut self.state[channel].links {
                        let target = lane_get(ramp.target, lane);
                        set_link_lane(ramp, lane, [target, target, 0.0, 0.0]);
                    }
                }
            }
        }
        if kind == ResetKind::FullToDefaults {
            self.rest_link(lane);
        }
    }
    // REALTIME_POLICY_END

    /// Clears the frozen integrators of every settled identity band on the first `channels`
    /// channels, once per block before its automation.
    ///
    /// A disabled band's words reach the identity section, through which `svf_step` integrates
    /// nothing, so its integrators keep the state of a signal long gone. That state is
    /// unobservable while the band stays disabled (its tap is `0 v1` and its mix word `0`, which
    /// selects the input), so clearing it moves no output bit, and it is what lets such a lane
    /// reach rest and a re-enabled band start from the state of a band prepared disabled (the
    /// parametric EQ's #1337 D4 rule, applied here at the block boundary). Per lane, from that
    /// lane's own words, so a bank and its scalar instances agree.
    fn clear_identity_bands(&mut self, channels: usize) {
        for s in &mut self.state[..channels] {
            for band in 0..BANDS {
                let frozen = identity_lanes(s, band) & !band_rest_lanes(s, band);
                for lane in (0..L::WIDTH).filter(|lane| frozen & (1 << lane) != 0) {
                    let b = &mut s.bands[band];
                    for word in [
                        &mut b.main.ic1,
                        &mut b.main.ic2,
                        &mut b.detector.ic1,
                        &mut b.detector.ic2,
                        &mut b.side.ic1,
                        &mut b.side.ic2,
                    ] {
                        lane_set(word, lane, 0.0);
                    }
                }
            }
        }
    }

    /// Validates one block's spans for one lane, commits the accepted values and retargets the
    /// ramps whose design moved. The one division D11 permits happens in `LinearRamp::set_target`.
    fn apply_automation(
        &mut self,
        spans: &[PreparedAutomationSpan],
        first_sample: u64,
        lane: usize,
        report: &mut ProcessReport,
    ) {
        let mut pending = [[None; PARAMETER_COUNT]; 2];
        let mut last_order = None;
        for (index, span) in spans.iter().enumerate() {
            let channel = match span.channel {
                ParameterChannel::Left => 0,
                ParameterChannel::Right => 1,
                ParameterChannel::Both => 2,
            };
            let parameter = span.parameter_index as usize;
            let order = parameter.saturating_mul(2).saturating_add(channel);
            let valid = channel < 2
                && index < self.automation_capacity as usize
                && parameter < PARAMETER_COUNT
                && span.sample == first_sample
                && parameter_value_valid(&SPECS[parameter], span.value)
                && last_order.is_none_or(|previous| order > previous)
                && pending[channel][parameter].is_none();
            if !valid {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
            last_order = Some(order);
            pending[channel][parameter] = Some(normalize_zero(span.value));
        }
        for (channel, values) in pending.iter().enumerate() {
            if values.iter().all(Option::is_none) {
                continue;
            }
            for (slot, value) in self.values[lane][channel].iter_mut().zip(values) {
                if let Some(value) = *value {
                    *slot = value;
                }
            }
            self.retarget(channel, lane);
        }
    }

    /// Points every ramp of one lane and channel whose design moved at its new design (#1409: the
    /// clamped walk keeps every word between its start and its target).
    ///
    /// A band's nine filter words ([`DESIGN_WORDS`]: the bell's `c1, a2, a3, m1, k A^2` and the
    /// detector's `c1, a2, a3, 1/Q`) move as one design: when any of them moves, all nine restart
    /// together from their current values, so every SVF triple the kernel loads is a convex
    /// combination of the current point and the new design, which is contractive
    /// (`effect_runtime::svf::transition_norm` is convex in the words). Skipping a word whose
    /// design bits did not move would leave it on its old schedule and the triple off that chord
    /// (#1928, the #1372a finding: a one-ulp frequency nudge can keep `a2`'s bits and move `c1`'s
    /// or `a3`'s).
    #[inline(never)]
    fn retarget(&mut self, channel: usize, lane: usize) {
        let designs = targets(&self.values[lane][channel], self.sample_rate);
        let mut words = lane_words(&self.state[channel], lane);
        let target = |words: &[f32; CHAN_WORDS], ramp: usize| words[SIGNAL_WORDS + ramp * 4 + 1];
        let design_moved: [bool; BANDS] = core::array::from_fn(|band| {
            DESIGN_WORDS.iter().any(|&index| {
                let ramp = band * BAND_RAMPS + index;
                target(&words, ramp).to_bits() != designs[ramp].to_bits()
            })
        });
        for (ramp, (chunk, design)) in words[SIGNAL_WORDS..]
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(designs)
            .enumerate()
        {
            let restart =
                design_moved[ramp / BAND_RAMPS] && DESIGN_WORDS.contains(&(ramp % BAND_RAMPS));
            if !restart && chunk[1].to_bits() == design.to_bits() {
                continue;
            }
            let mut ramp = LinearRamp {
                current: chunk[0],
                target: chunk[1],
                step: chunk[2],
                remaining: chunk[3] as u32,
            };
            ramp.set_target(design, RAMP_SAMPLES);
            chunk.copy_from_slice(&[ramp.current, ramp.target, ramp.step, ramp.remaining as f32]);
            // Issue #1969: never below a link glide's remaining frames, which may be longer.
            self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
        }
        set_lane_words(&mut self.state[channel], lane, &words);
    }

    /// The bands a segment must run on the first `channels` channels: a band is skipped only
    /// when, on every lane, it is a settled identity section whose SVFs and gain word are `+0.0`.
    /// Running such a band is an exact no-op for finite inputs (its `+0.0` integrators stay `+0.0` through zero
    /// words, its tap is `+0.0`, so the level floors at -160 dB below every threshold and knee and
    /// the gain word stays `+0.0`, and its zero mix word selects the input), so skipping it moves
    /// no bit, whichever lanes a bank or a scalar instance decides on.
    fn live_bands(&self, channels: usize) -> [bool; BANDS] {
        core::array::from_fn(|band| {
            self.state[..channels]
                .iter()
                .any(|s| identity_lanes(s, band) & band_rest_lanes(s, band) != all_lanes(L::WIDTH))
        })
    }

    /// Runs one block: the silent fast path, the ramping and the settled segment, the D7 check.
    ///
    /// The block splits where the last ramp settles, so the ramp prologue is a const-generic
    /// variant, not a per-sample branch; the settled variant omits updates that are exact no-ops,
    /// which is what makes the render partition-invariant. `MONO` (issue #973) renders the
    /// collapsed track from the left plane and the left state only.
    fn run_block<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        sidechain: Option<(&[f32], &[f32])>,
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        if MONO {
            // From here until the disengage copy the right channel is frozen and stale.
            self.collapsed = true;
        }
        let smoothed = (self.ramp_frames_left as usize).min(frames);
        self.ramp_frames_left -= smoothed as u32;
        if self.silent_block::<MONO>(left, right, sidechain) {
            // Every lane rests and every input word is `+0.0`: the kernel would write the `+0.0`
            // the planes hold and move only the clocks, which advance here as it would.
            let channels = if MONO { 1 } else { 2 };
            self.advance_clocks_masked(all_lanes(L::WIDTH), frames, channels);
            return;
        }
        let (split, total) = (smoothed * L::WIDTH, left.len());
        let armed_after = L::splat(lane::silence_frames(self.sample_rate) as f32);
        let channels = if MONO { 1 } else { 2 };
        // Issue #1969: only the ramping prefix of a block with a glide in flight takes the blend.
        let glide = self.gliding::<MONO>();
        let mut run = |ramping: bool, range: core::ops::Range<usize>| {
            let live = self.live_bands(channels);
            let (first, rest) = self.state.split_at_mut(1);
            let (left, right) = (&mut left[range.clone()], &mut right[..]);
            let right = if MONO {
                &mut [][..]
            } else {
                &mut right[range.clone()]
            };
            let sidechain = sidechain.map(|(l, r)| (&l[range.clone()], &r[range.clone()]));
            let states = [&mut first[0], &mut rest[0]];
            let frames = range.len() / L::WIDTH;
            // Issue #1328 A9: the armed form only where a lane can arm while holding state.
            let arms = |run: L, svfs: &[SvfState<L>]| {
                silence_armable_holding(
                    run,
                    frames,
                    armed_after,
                    svf_state_held(svfs.iter().copied()),
                )
            };
            let armable = states[..channels].iter().any(|s| {
                let main: [SvfState<L>; 2 * BANDS] = core::array::from_fn(|index| {
                    let band = &s.bands[index / 2];
                    if index % 2 == 0 {
                        band.main
                    } else {
                        band.detector
                    }
                });
                arms(s.run, &main) || (CONNECTED && arms(s.side_run, &s.bands.map(|b| b.side)))
            });
            let [state_left, state_right] = states;
            if !armable {
                let planes: [&[f32]; 2] = [left, right];
                let sides = sidechain.map(|(l, r)| [l, r]);
                for (channel, s) in [&mut *state_left, &mut *state_right]
                    .into_iter()
                    .enumerate()
                    .take(channels)
                {
                    silence_skip_block(planes[channel], frames, &mut s.run);
                    if CONNECTED {
                        match sides {
                            Some(sides) => {
                                silence_skip_block(sides[channel], frames, &mut s.side_run)
                            }
                            None => silence_advance(&mut s.side_run, frames),
                        }
                    }
                }
            }
            let args = Args {
                left,
                right,
                sidechain,
                armed_after,
                live,
                coef: (&self.coef[0], &self.coef[1]),
                state: (state_left, state_right),
            };
            match (ramping, armable, ramping && glide) {
                (true, true, true) => segment::<L, CONNECTED, true, MONO, true, true>(args),
                (true, false, true) => segment::<L, CONNECTED, true, MONO, false, true>(args),
                (true, true, false) => segment::<L, CONNECTED, true, MONO, true, false>(args),
                (true, false, false) => segment::<L, CONNECTED, true, MONO, false, false>(args),
                (false, true, _) => segment::<L, CONNECTED, false, MONO, true, false>(args),
                (false, false, _) => segment::<L, CONNECTED, false, MONO, false, false>(args),
            }
        };
        if split > 0 {
            run(true, 0..split);
        }
        if split < total {
            run(false, split..total);
        }
        self.finish_block::<MONO>(left, right, frames, reports);
    }

    /// Whether the block may skip the kernel (issue #1746): every input plane is exact `+0.0` and
    /// every lane rests. Out of line: no vector arithmetic of the kernel's.
    #[inline(never)]
    fn silent_block<const MONO: bool>(
        &self,
        left: &[f32],
        right: &[f32],
        sidechain: Option<(&[f32], &[f32])>,
    ) -> bool {
        let all = all_lanes(L::WIDTH);
        if MONO {
            return block_is_positive_zero(left) && self.channel_rest_lanes(0) == all;
        }
        block_is_positive_zero(left)
            && block_is_positive_zero(right)
            && sidechain.is_none_or(|(l, r)| block_is_positive_zero(l) && block_is_positive_zero(r))
            && self.rest_lanes() == all
    }

    /// The lanes at exact rest on both channels (`PreparedNativeEffectBank::lanes_at_rest`), or
    /// on the left channel alone while [`collapsed`](Self::collapsed) (#1894).
    fn rest_lanes(&self) -> u32 {
        if self.collapsed {
            self.channel_rest_lanes(0)
        } else {
            self.channel_rest_lanes(0) & self.channel_rest_lanes(1)
        }
    }

    /// The lanes of one channel at exact rest: every band's SVF integrators and gain word `+0.0`
    /// and every ramp settled, the link weights included (issue #1969: a glide in flight moves
    /// its detectors every frame, so it is never at rest and never skipped). The silence clocks
    /// are clocks, not state.
    ///
    /// Why that is a fixed point of a `+0.0` frame that writes `+0.0`: with `+0.0` input and
    /// integrators, `svf_step` gives `v1 = v2 = +0.0` and keeps them, so every tap is `+0.0`; the
    /// level floors at -160 dB, under every threshold less half the widest knee, so the curve's
    /// target is `+-0.0` and the rate step's rest snap
    /// (`effect_runtime::dynamic_band::CHANGE_REST_DB`) writes `+0.0`; the mix word is then the
    /// static `m1`, and `m1 v1 + 0` is `+0.0` (or a zero `m1` selects the `+0.0` input), so band
    /// by band the cascade carries `+0.0`. A settled ramp is an exact no-op of both segment
    /// variants.
    fn channel_rest_lanes(&self, channel: usize) -> u32 {
        let s = &self.state[channel];
        (0..BANDS).fold(all_lanes(L::WIDTH), |mask, band| {
            mask & band_rest_lanes(s, band)
        }) & s
            .ramps
            .iter()
            .chain(&s.links)
            .fold(all_lanes(L::WIDTH), |mask, ramp| {
                mask & positive_zero_lanes(ramp.remaining)
            })
    }

    /// The clocks a `+0.0` block of `frames` frames moves, on the lanes of `mask` and the first
    /// `channels` channels: the silence clocks (the sidechain's only while connected) and the
    /// bank's ramp countdown, which only says where a block splits.
    fn advance_clocks_masked(&mut self, mask: u32, frames: usize, channels: usize) {
        for s in &mut self.state[..channels] {
            for clock in [&mut s.run, &mut s.side_run]
                .into_iter()
                .take(1 + usize::from(CONNECTED))
            {
                let mut advanced = *clock;
                silence_advance(&mut advanced, frames);
                for lane in (0..L::WIDTH).filter(|lane| mask & (1 << lane) != 0) {
                    lane_set(clock, lane, lane_get(advanced, lane));
                }
            }
        }
    }

    /// The D7 boundary check, once per block per channel, and the lane-local recovery. The state
    /// words are scanned beside the output: a NaN integrator or gain word need not reach the
    /// output (the level clamp swallows NaN). A failed lane is zeroed, re-seeded and, when it
    /// carries a member, reported. The collapsed body decides on the left plane and state and
    /// recovers and counts both channels, as a dual block on `L == R` would.
    fn finish_block<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        const WORDS: usize = BANDS * BAND_SIGNAL_WORDS;
        let width = L::WIDTH;
        let channels = if MONO { 1 } else { 2 };
        for (channel, block) in [left, right].into_iter().enumerate().take(channels) {
            let all = chan_words(&self.state[channel]);
            let mut words = [0.0_f32; WORDS * MAX_WIDTH];
            for (index, word) in all[..WORDS].iter().enumerate() {
                word.store(&mut words[index * width..(index + 1) * width]);
            }
            let words = &words[..WORDS * width];
            if check_block::<L>(block) && check_block::<L>(words) {
                continue;
            }
            let output_failed = nonfinite_lane_mask::<L>(block);
            let failed = output_failed | nonfinite_lane_mask::<L>(words);
            for lane in (0..width).filter(|lane| failed & (1 << lane) != 0) {
                // A prepared-bypass lane emits its dry input, which is inside the D7 domain, so a
                // failure of the wet state alone recovers the state and keeps that output: the
                // lane renders what the graph's bypass shunt renders (#17 S2,
                // `bypass_shunt_identity`).
                let bypassed = lane_get(self.coef[channel].bypass, lane) > 0.0;
                if output_failed & (1 << lane) != 0 || !bypassed {
                    for frame in 0..frames {
                        block[frame * width + lane] = 0.0;
                    }
                }
                for recovered in if MONO { 0..2 } else { channel..channel + 1 } {
                    self.reset_lane_full(recovered, lane);
                    if !self.is_active(lane) {
                        continue;
                    }
                    let report = &mut reports[lane];
                    let counter = if recovered == 0 {
                        &mut report.nonfinite_left_blocks
                    } else {
                        &mut report.nonfinite_right_blocks
                    };
                    *counter = counter.saturating_add(1);
                }
            }
        }
    }

    /// The `DESIGNED` term of the channel-symmetry witness for `lane` (issue #973): every
    /// per-channel word the kernel reads that is not running state, compared by bits. That is the
    /// three [`Coef`] words and every ramp's four fields (the derived knee words follow the knee
    /// ramp), the link weights' included (#1969); the integrators, clocks and gain words are
    /// running state, and `defaults` and `values` are control-side words the kernel never loads.
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= L::WIDTH {
            return false;
        }
        let coef = |c: &Coef<L>| [c.bypass, c.link_max, c.link_avg];
        let (l, r) = (&self.state[0], &self.state[1]);
        let designed = |s: &Chan<L>| lane_words(s, lane);
        coef(&self.coef[0])
            .iter()
            .zip(&coef(&self.coef[1]))
            .all(|(a, b)| lane_get(*a, lane).to_bits() == lane_get(*b, lane).to_bits())
            && designed(l)[SIGNAL_WORDS..]
                .iter()
                .zip(&designed(r)[SIGNAL_WORDS..])
                .all(|(a, b)| a.to_bits() == b.to_bits())
            && l.links.iter().zip(&r.links).all(|(a, b)| {
                link_lane(a, lane)
                    .iter()
                    .zip(&link_lane(b, lane))
                    .all(|(a, b)| a.to_bits() == b.to_bits())
            })
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload codec runs in the plan-swap block.
    fn snapshot_lane(
        &self,
        lane: usize,
        output: &mut StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let layout = self.layout();
        let lengths = (output.common.len(), output.left.len(), output.right.len());
        payload::validate_lengths(&layout, lengths).map_err(codec_error)?;
        payload::write_header(&layout, output.common);
        for (channel, bytes) in [&mut *output.left, &mut *output.right]
            .into_iter()
            .enumerate()
        {
            let words = lane_words(&self.state[channel], lane);
            let values = &self.values[lane][channel];
            let all = words[..SIGNAL_WORDS]
                .iter()
                .chain(values)
                .chain(&words[SIGNAL_WORDS..]);
            for (index, &word) in all.enumerate() {
                payload::write_f32(bytes, index, word);
            }
            for (index, ramp) in self.state[channel].links.iter().enumerate() {
                for (offset, word) in link_lane(ramp, lane).into_iter().enumerate() {
                    payload::write_f32(bytes, STATE_LINK_WORD + index * 4 + offset, word);
                }
            }
        }
        Ok(())
    }

    /// Parses and validates one channel section without touching any state, so a restore is
    /// all-or-none. Every word must be one the engine can hold: the committed values in their
    /// domains, finite integrators, whole clocks, gain words in `[-24, 24]` dB, and each ramp
    /// aimed bit for bit at the design of the committed values, its path inside the word's
    /// interval ([`WORD_BOUNDS`]), settled at its target, and every band's SVF words contractive.
    fn parse_lane(&self, bytes: &[u8]) -> Result<LaneRestore, StatePayloadError> {
        let read = |index: usize| payload::read_f32(bytes, index);
        let mut values = [0.0_f32; PARAMETER_COUNT];
        for (index, value) in values.iter_mut().enumerate() {
            *value = read(SIGNAL_WORDS + index);
            if is_negative_zero(*value) || !parameter_value_valid(&SPECS[index], *value) {
                return Err(state_error("effect.state.parameter"));
            }
        }
        let mut words = [0.0_f32; CHAN_WORDS];
        for (index, word) in words.iter_mut().enumerate() {
            *word = read(if index < SIGNAL_WORDS {
                index
            } else {
                index + PARAMETER_COUNT
            });
        }
        let signal_ok = words[..BANDS * BAND_SIGNAL_WORDS]
            .as_chunks::<BAND_SIGNAL_WORDS>()
            .0
            .iter()
            .all(|band| {
                let gain = band[6];
                check_block::<f32>(&band[..6])
                    && gain.is_finite()
                    && (-RANGE_MAX_DB..=RANGE_MAX_DB).contains(&gain)
                    && !is_negative_zero(gain)
            })
            && integral_within(words[SIGNAL_WORDS - 2], RUN_MAX)
            && integral_within(words[SIGNAL_WORDS - 1], RUN_MAX);
        if !signal_ok {
            return Err(state_error("effect.state.signal"));
        }
        let designs = targets(&values, self.sample_rate);
        for (index, chunk) in words[SIGNAL_WORDS..].as_chunks::<4>().0.iter().enumerate() {
            let ramp = LinearRamp {
                current: chunk[0],
                target: chunk[1],
                step: chunk[2],
                remaining: chunk[3] as u32,
            };
            let valid = chunk[1].to_bits() == designs[index].to_bits()
                && integral_within(chunk[3], RAMP_SAMPLES as f32)
                && (chunk[3] != 0.0 || chunk[0].to_bits() == chunk[1].to_bits())
                && payload::ramp_path_inside(ramp, WORD_BOUNDS[index % BAND_RAMPS], RAMP_SAMPLES);
            if !valid {
                return Err(state_error("effect.state.parameter"));
            }
        }
        let current = |index: usize| words[SIGNAL_WORDS + index * 4];
        let contractive = (0..BANDS).all(|band| {
            [C1, DC1].into_iter().all(|first| {
                let c1 = band * BAND_RAMPS + first;
                transition_norm(current(c1), current(c1 + 1), current(c1 + 2))
                    <= RAMP_PATH_NORM_TOLERANCE
            })
        });
        if !contractive {
            return Err(state_error("effect.state.parameter"));
        }
        // Issue #1969 (#1370 D6): each weight lies in `[0, 1]` on its whole path, the engine never
        // holds `-0.0` there, a glide in flight moves (its step is `(t - c) / n` with `t` in
        // `{0, 1}`, `c` a word of an earlier glide and `n <= 2^24`, never `0`), a settled weight
        // holds its target, and the two targets are exactly one supported mode's rest weights.
        let mut links = [[0.0_f32; 4]; LINK_COUNT];
        for (index, link) in links.iter_mut().enumerate() {
            let word = |offset: usize| read(STATE_LINK_WORD + index * 4 + offset);
            let [current, target, step, remaining] = [word(0), word(1), word(2), word(3)];
            let maximum = effect_contract::LINK_GLIDE_MAX_SAMPLES;
            let valid = integral_within(remaining, maximum as f32)
                && !is_negative_zero(current)
                && (remaining == 0.0 || step != 0.0)
                && (remaining != 0.0 || current.to_bits() == target.to_bits())
                && payload::ramp_path_inside(
                    LinearRamp {
                        current,
                        target,
                        step,
                        remaining: remaining as u32,
                    },
                    (0.0, 1.0),
                    maximum,
                );
            if !valid {
                return Err(state_error("effect.state.link"));
            }
            *link = [current, target, step, remaining];
        }
        let mode = effect_contract::link_glide_mode([links[0][1], links[1][1]])
            .filter(|mode| DYNAMIC_EQ_DESCRIPTOR.supported_link_modes.contains(*mode))
            .ok_or(state_error("effect.state.link"))?;
        Ok(LaneRestore {
            words,
            values,
            links,
            mode,
        })
    }

    fn restore_lane(
        &mut self,
        lane: usize,
        state_layout_version: u32,
        input: &StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != STATE_LAYOUT_VERSION {
            return Err(state_error("effect.state.version"));
        }
        let layout = self.layout();
        let lengths = (input.common.len(), input.left.len(), input.right.len());
        payload::validate_lengths(&layout, lengths).map_err(codec_error)?;
        payload::read_header(&layout, input.common).map_err(codec_error)?;
        let parsed = [self.parse_lane(input.left)?, self.parse_lane(input.right)?];
        if parsed[0].mode != parsed[1].mode {
            return Err(state_error("effect.state.link"));
        }
        // Issue #1969 (#1370 D6): the weights land only under the mode they glide to. A payload
        // taken under another mode (the plan-swap carry into a successor prepared with a new
        // mode) leaves the lane's own mode at rest, as before the glide existed.
        let adopt = parsed[0].mode == self.lane_link(lane);
        if !adopt {
            self.rest_link(lane);
        }
        for (channel, restore) in parsed.iter().enumerate() {
            set_lane_words(&mut self.state[channel], lane, &restore.words);
            self.values[lane][channel] = restore.values;
            if restore.words[SIGNAL_WORDS..]
                .as_chunks::<4>()
                .0
                .iter()
                .any(|chunk| chunk[3] != 0.0)
            {
                self.ramp_frames_left = self.ramp_frames_left.max(RAMP_SAMPLES);
            }
            if adopt {
                for (ramp, words) in self.state[channel].links.iter_mut().zip(restore.links) {
                    set_link_lane(ramp, lane, words);
                    self.ramp_frames_left = self.ramp_frames_left.max(words[3] as u32);
                }
            }
        }
        Ok(())
    }
    // REALTIME_POLICY_END

    /// Band `tap`'s smoothed change on lane `lane`, both channels.
    fn observe_lane(&self, tap: usize, lane: usize) -> (f32, f32) {
        (
            lane_get(self.state[0].bands[tap].gain_db, lane),
            lane_get(self.state[1].bands[tap].gain_db, lane),
        )
    }
}

/// One channel section of a parsed, validated payload.
struct LaneRestore {
    words: [f32; CHAN_WORDS],
    values: [f32; PARAMETER_COUNT],
    links: [[f32; 4]; LINK_COUNT],
    /// The mode the weights glide to, decoded from their targets.
    mode: LinkMode,
}

impl<const CONNECTED: bool> PreparedNativeEffect for DynamicEq<f32, CONNECTED> {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        DynamicEq::reset(self, kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        self.clear_identity_bands(2);
        self.apply_automation(block.automation, block.first_sample, 0, &mut reports[0]);
        let frames = block.frames();
        let sidechain = if CONNECTED { block.sidechain } else { None };
        self.run_block::<false>(block.left, block.right, sidechain, frames, &mut reports);
        reports[0]
    }

    fn channel_symmetry(&self) -> bool {
        self.designed_channel_symmetry(0)
    }

    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track == 0 && DynamicEq::retarget_link(self, 0, mode, samples)
    }

    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        let tap = tap_index as usize;
        if tap >= BANDS {
            return false;
        }
        (out.left, out.right) = self.observe_lane(tap, 0);
        true
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_state_payload(
        &self,
        mut output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.snapshot_lane(0, &mut output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.restore_lane(0, state_layout_version, &input)
    }
    // REALTIME_POLICY_END
}

/// Rejects a track index the bank does not have.
fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    usize::try_from(track_index)
        .ok()
        .filter(|&track| track < width)
        .ok_or(state_error("effect.state.track"))
}

/// One lane's prepared defaults in bytes: every parameter of both channels, then the link mode.
const LANE_DEFAULT_BYTES: usize = 2 * PARAMETER_COUNT * 4 + 4;

impl<L: Lane> DynamicEq<L, false> {
    /// The one bank body, dual or collapsed (issue #973): `MONO` chooses the render body only.
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let width = self.bank_width.expect("a bank is prepared with a width");
        let mut report = BankProcessReport::empty(width);
        if block.width != width || block.sidechain.is_some() {
            return report;
        }
        self.clear_identity_bands(if MONO { 1 } else { 2 });
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
        for (track, lane_report) in reports.iter_mut().enumerate().take(L::WIDTH) {
            if !self.is_active(track) {
                continue;
            }
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            let spans = &block.automation[start..end];
            self.apply_automation(spans, block.first_sample, track, lane_report);
        }
        let frames = block.frames as usize;
        self.run_block::<MONO>(block.left, block.right, None, frames, &mut reports);
        report.reports[..L::WIDTH].copy_from_slice(&reports[..L::WIDTH]);
        report
    }

    /// Every per-channel word a collapsed block leaves stale on the right, compared by bits.
    fn channels_bit_equal(&self) -> bool {
        let words = |channel: usize| {
            let (c, s) = (&self.coef[channel], &self.state[channel]);
            let mut all = [L::zero(); CHAN_WORDS + 3 + LINK_COUNT * 4];
            all[..CHAN_WORDS].copy_from_slice(&chan_words(s));
            all[CHAN_WORDS..CHAN_WORDS + 3].copy_from_slice(&[c.bypass, c.link_max, c.link_avg]);
            for (chunk, ramp) in all[CHAN_WORDS + 3..]
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(&s.links)
            {
                chunk.copy_from_slice(&[ramp.current, ramp.target, ramp.step, ramp.remaining]);
            }
            all
        };
        words(0).iter().zip(&words(1)).all(|(l, r)| {
            (0..L::WIDTH).all(|lane| lane_get(*l, lane).to_bits() == lane_get(*r, lane).to_bits())
        })
    }

    // REALTIME_POLICY_BEGIN: #1747, the prepared defaults move with a render-time lane swap.
    /// Lane `lane`'s defaults row, left then right, as little-endian `f32` bits, then its link
    /// mode (`effect_contract::exchange_lane_link_word`).
    fn exchange_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        if lane >= L::WIDTH || bytes.len() != LANE_DEFAULT_BYTES {
            return false;
        }
        let (values, link) = bytes.split_at_mut(2 * PARAMETER_COUNT * 4);
        let link = effect_contract::exchange_lane_link_word(self.lane_link(lane), link);
        self.set_lane_link(lane, link);
        // The weights rest at the incoming mode; the swap's payload restore then brings a glide
        // in flight with it (issue #1969, #1370 D6).
        self.rest_link(lane);
        for (value, chunk) in self.defaults[lane]
            .iter_mut()
            .flatten()
            .zip(values.as_chunks_mut::<4>().0.iter_mut())
        {
            let incoming = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            chunk.copy_from_slice(&value.to_le_bytes());
            *value = incoming;
        }
        true
    }
    // REALTIME_POLICY_END
}

macro_rules! bank_impl {
    ($lane:ty) => {
        impl PreparedNativeEffectBank for DynamicEq<$lane, false> {
            fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
                DynamicEq::reset_lane(self, lane, kind);
            }
            /// Every field is inline: this instance owns no heap (#1074).
            fn retained_bytes(&self) -> u64 {
                0
            }

            fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
                let tap = tap_index as usize;
                if tap >= BANDS || out.len() != <$lane as Lane>::WIDTH {
                    return false;
                }
                for (lane, sample) in out.iter_mut().enumerate() {
                    (sample.left, sample.right) = self.observe_lane(tap, lane);
                }
                true
            }

            fn reset(&mut self, kind: ResetKind) {
                DynamicEq::reset(self, kind);
            }

            fn lanes_at_rest(&self) -> u32 {
                self.rest_lanes()
            }

            fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
                self.process_bank_inner::<false>(block)
            }

            fn lane_channel_symmetry(&self, lane: usize) -> bool {
                self.designed_channel_symmetry(lane)
            }

            /// Issue #1969: `DynamicEq::retarget_link`; a padded lane takes no glide.
            fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
                DynamicEq::retarget_link(self, track, mode, samples)
            }

            fn supports_mono_collapse(&self) -> bool {
                true
            }

            fn process_bank_mono(
                &mut self,
                block: EffectBankProcessBlock<'_>,
            ) -> BankProcessReport {
                self.process_bank_inner::<true>(block)
            }

            /// The left channel's whole per-channel render state onto the right, every lane.
            fn desymmetrize_channels(&mut self) {
                self.state[1] = self.state[0];
                self.coef[1] = self.coef[0];
                self.collapsed = false;
            }

            fn channels_agree(&self) -> bool {
                self.channels_bit_equal()
            }

            // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
            fn snapshot_track_state_payload(
                &self,
                track_index: u32,
                mut output: StatePayloadOutput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane as Lane>::WIDTH)?;
                self.snapshot_lane(track, &mut output)
            }

            fn restore_track_state_payload(
                &mut self,
                track_index: u32,
                state_layout_version: u32,
                input: StatePayloadInput<'_>,
            ) -> Result<(), StatePayloadError> {
                let track = checked_track(track_index, <$lane as Lane>::WIDTH)?;
                self.restore_lane(track, state_layout_version, &input)
            }
            // REALTIME_POLICY_END

            // REALTIME_POLICY_BEGIN: #1747, the clock advance of a skipped block.
            fn advance_clocks(&mut self, lanes_mask: u32, frames: u32) {
                debug_assert_eq!(lanes_mask & !self.rest_lanes(), 0, "a lane not at rest");
                self.ramp_frames_left = self.ramp_frames_left.saturating_sub(frames);
                // A collapsed bank's right clocks are frozen, as the collapsed block leaves them.
                let channels = if self.collapsed { 1 } else { 2 };
                self.advance_clocks_masked(lanes_mask, frames as usize, channels);
            }
            // REALTIME_POLICY_END

            fn lane_default_bytes(&self) -> Option<usize> {
                Some(LANE_DEFAULT_BYTES)
            }

            fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
                self.exchange_defaults(lane, bytes)
            }
        }
    };
}

bank_impl!(Simd4);
// Issues #1110 and #1112: an eight-lane type only where `avx2` is enabled (`BankWidth`).
#[cfg(target_feature = "avx2")]
bank_impl!(lane::Simd8);

/// Validates one prepare request's initial values against the frozen domains.
fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<[[f32; PARAMETER_COUNT]; 2], EffectPrepareError> {
    let invalid = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(invalid);
    }
    let mut defaults = [[0.0; PARAMETER_COUNT]; 2];
    for (index, pair) in values.as_chunks::<2>().0.iter().enumerate() {
        for (channel, (value, expected)) in pair
            .iter()
            .zip([ParameterChannel::Left, ParameterChannel::Right])
            .enumerate()
        {
            if value.parameter_index != index as u32
                || value.channel != expected
                || is_negative_zero(value.value)
                || !parameter_value_valid(&SPECS[index], value.value)
            {
                return Err(invalid);
            }
            defaults[channel][index] = normalize_zero(value.value);
        }
    }
    Ok(defaults)
}

impl NativeEffectFactory for DynamicEqFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DYNAMIC_EQ_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        (size_of::<DynamicEq<f32, true>>()).max(size_of::<DynamicEq<f32, false>>()) as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let defaults = [initial_defaults(request.initial_values)?; MAX_WIDTH];
        let invalid = EffectPrepareError {
            code: "effect.parameter.initial",
        };
        let processor: Box<dyn PreparedNativeEffect> = match metadata.ports.sidechain {
            PreparedSidechainPort::Connected { .. } => {
                Box::new(DynamicEq::<f32, true>::new(&metadata, None, defaults).ok_or(invalid)?)
            }
            _ => Box::new(DynamicEq::<f32, false>::new(&metadata, None, defaults).ok_or(invalid)?),
        };
        Ok(PreparedEffect {
            processor,
            metadata,
        })
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        bind_bank::<true>(request)
    }
}

/// Binds a bank at a width this artifact executes (no runtime SIMD dispatch, master plan #83 D4),
/// or declines. Each lane is prepared from its own request, so a padded lane (issue #1092) is a
/// clone of a member's validated values; it takes no automation and is recovered unreported.
/// `NATIVE_ONLY` is the width check as a switch: production passes `true`; the unit tests pass
/// `false` to bind the other width's bank on this host through the same code.
fn bind_bank<const NATIVE_ONLY: bool>(
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
    request.validate_shape()?;
    let first = request.requests[0];
    let metadata = expected_prepared_metadata(&DYNAMIC_EQ_DESCRIPTOR, first)?;
    let mut defaults = [initial_defaults(first.initial_values)?; MAX_WIDTH];
    let mut links = [metadata.link_mode; MAX_WIDTH];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(&DYNAMIC_EQ_DESCRIPTOR, item)?;
        same_program &= candidate.program_key() == metadata.program_key();
        defaults[track] = initial_defaults(item.initial_values)?;
        if request.active_mask[track] {
            links[track] = candidate.link_mode;
        }
    }
    let unconnected = matches!(
        metadata.ports.sidechain,
        PreparedSidechainPort::Unconnected { id, required: false } if id == port_id("sidechain-in")
    );
    if !same_program || !unconnected {
        return Ok(None);
    }
    let invalid = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    let active = request
        .active_mask
        .iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0, |bits, (lane, _)| bits | (1 << lane));
    let processor = effect_contract::match_bank_width!(request.width, |L| {
        if NATIVE_ONLY && L::WIDTH != Backend::current().width() {
            return Ok(None);
        }
        let mut bank = DynamicEq::<L, false>::new(&metadata, Some(request.width), defaults)
            .ok_or(invalid)?
            .with_lane_links(&links[..request.requests.len()]);
        bank.active = active & all_lanes(L::WIDTH);
        Box::new(bank) as Box<dyn PreparedNativeEffectBank>
    });
    Ok(Some(PreparedEffectBank {
        processor,
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
    }))
}

#[cfg(test)]
mod glide_tests;
