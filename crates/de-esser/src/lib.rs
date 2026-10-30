//! Launch de-esser (issue #15 S1): a dual-mono, zero-latency band-limited dynamics processor.
//!
//! One generic kernel body owns the per-sample graph ([`kernel`]); this module is its control
//! plane: the frozen descriptor, preparation, the live-word designs, the state payload and the
//! once-per-block boundary policy. The algorithm, its limits and its sources are
//! `dsp-research/de-esser.md`.
//!
//! # Shapes
//!
//! A prepared instance is `DeEsser<L, CONNECTED>`. The scalar instance is `L = f32`, the
//! `WIDTH = 1` instantiation of the same kernel, so it is the bit oracle of the four- and
//! eight-lane banks. `CONNECTED` selects the connected sidechain as the detector's source.
//!
//! # Live values
//!
//! All ten parameters are live block-rate values, each a 64-sample ramp of the word the kernel
//! reads: threshold and range in dB, the slope `1/R - 1` (so the render never divides), the
//! attack and release one-pole coefficients, the SVF words `c1, a2, a3` and the damping `k`
//! (frequency and Q), and the three switch words (mode, detector, listen), which crossfade their
//! two arms. A record designs its targets once on the block's control side (`targets`);
//! the render only adds.
//!
//! # Realtime rules
//!
//! Preparation allocates the instance. `process` and `process_bank` allocate nothing, take no lock
//! and make no syscall; a record's design runs the vendored `math` `tan` and `exp`.
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
use lane::kernels::{silence_advance, silence_skip_block, svf_state_held};
use lane::{Backend, Lane, Simd4, silence_armable_holding};

use kernel::{
    Args, C1, Chan, Coef, DETECTOR, LINK_COUNT, MAX_WIDTH, MODE, RAMPS, SHELF_C1, segment,
};

const PARAMETER_COUNT: usize = 10;

/// Samples every live word takes to reach a new target (decision 15: every live value ramps).
const RAMP_SAMPLES: u32 = 64;

/// The leading signal words of one channel ([`Chan`]): the main and sidechain SVFs' four
/// integrators, two silence clocks and the gain word.
const SIGNAL_WORDS: usize = 7;

/// Word index of the shelf SVF's two integrators (#1984), after each ramp's `current`, `target`,
/// `step` and `remaining`. They follow the ramps so that every earlier word keeps its index.
const SHELF_WORD: usize = SIGNAL_WORDS + RAMPS * 4;

/// The state words of one channel the kernel carries ([`Chan`]): the signal words, the ramps,
/// then the shelf integrators.
const CHAN_WORDS: usize = SHELF_WORD + 2;

/// The ramp words of [`CHAN_WORDS`].
const RAMP_WORDS: core::ops::Range<usize> = SIGNAL_WORDS..SHELF_WORD;

/// One channel section of the payload: the seven signal words, the ten committed values, the ramp
/// words and the shelf integrators, then the two link weight quadruples (issue #1969, #1370 D6). The lane's link mode is
/// the weights' targets (`effect_contract::link_glide_mode`), so it takes no word of its own.
const STATE_LANE_WORDS: usize = STATE_LINK_WORD + LINK_COUNT * 4;

/// Word index of the first link weight quadruple (`w_link`, then `w_avg`) in a channel section.
const STATE_LINK_WORD: usize = CHAN_WORDS + PARAMETER_COUNT;

/// The saturated silence clock (`lane::kernels::silence_advance`).
const RUN_MAX: f32 = 16_777_216.0;

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

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    name: &'static str,
    display_name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    domain: ParameterDomain,
    range: Option<(f32, f32)>,
    default_value: f32,
    mapping: ParameterMapping,
    enum_choices: &'static [EnumChoice],
) -> ParameterDescriptor {
    let (minimum, maximum) = match range {
        Some((low, high)) => (Some(low), Some(high)),
        None => (None, None),
    };
    ParameterDescriptor {
        id: match ParameterId::new(id) {
            Some(id) => id,
            None => panic!("nonzero static parameter identifier"),
        },
        name,
        display_name,
        display_unit,
        unit,
        domain,
        minimum,
        maximum,
        default_value,
        mapping,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices,
        lattice: effect_contract::default_parameter_lattice(unit, domain, mapping),
        prepared_reason: None,
    }
}

const MODE_CHOICES: [EnumChoice; 2] = [
    EnumChoice {
        value: 1.0,
        label: "wideband",
    },
    EnumChoice {
        value: 2.0,
        label: "split",
    },
];

/// The detector choices. The selected tap is also what the listen arm plays. The split arm cuts
/// the band with the band detector, and with the high-pass detector it is a dynamic high shelf
/// that never boosts (#1984): `g x + (1 - g) v2s`, `v2s` a Butterworth low-pass at the band's
/// frequency (`kernel`), unity at DC and the held gain toward Nyquist. There the frequency is
/// where the cut begins (the shelf's corner, about -3 to -5 dB at it, the held gain about an
/// octave above it), and Q shapes only the detector.
const DETECTOR_CHOICES: [EnumChoice; 2] = [
    EnumChoice {
        value: 1.0,
        label: "band",
    },
    EnumChoice {
        value: 2.0,
        label: "high",
    },
];

/// The value both enumerations use for their second choice (split, high).
const SECOND_CHOICE: f32 = 2.0;

use ParameterDomain::{Boolean, Continuous, Enumeration};
use ParameterMapping::{Linear, Logarithmic, Stepped};

/// Frozen de-esser parameters. Descriptor position and stable numeric ID agree.
pub const DE_ESSER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "frequency_hz",
        "frequency",
        "Hz",
        ParameterUnit::Hz,
        Continuous,
        Some((1_000.0, 16_000.0)),
        6_500.0,
        Logarithmic,
        &[],
    ),
    parameter(
        2,
        "q",
        "Q",
        "Q",
        ParameterUnit::Ratio,
        Continuous,
        Some((0.5, 8.0)),
        2.0,
        Logarithmic,
        &[],
    ),
    parameter(
        3,
        "threshold_db",
        "threshold",
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((-60.0, 0.0)),
        -20.0,
        Linear,
        &[],
    ),
    parameter(
        4,
        "ratio",
        "ratio",
        "ratio",
        ParameterUnit::Ratio,
        Continuous,
        Some((1.0, 20.0)),
        4.0,
        Logarithmic,
        &[],
    ),
    parameter(
        5,
        "range_db",
        "range",
        "dB",
        ParameterUnit::Db,
        Continuous,
        Some((0.0, 40.0)),
        20.0,
        Linear,
        &[],
    ),
    parameter(
        6,
        "attack_ms",
        "attack",
        "ms",
        ParameterUnit::Milliseconds,
        Continuous,
        Some((0.1, 50.0)),
        1.0,
        Logarithmic,
        &[],
    ),
    parameter(
        7,
        "release_ms",
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        Continuous,
        Some((5.0, 500.0)),
        50.0,
        Logarithmic,
        &[],
    ),
    parameter(
        8,
        "mode",
        "mode",
        "type",
        ParameterUnit::Linear,
        Enumeration,
        None,
        2.0,
        Stepped,
        &MODE_CHOICES,
    ),
    parameter(
        9,
        "detector",
        "detector",
        "type",
        ParameterUnit::Linear,
        Enumeration,
        None,
        1.0,
        Stepped,
        &DETECTOR_CHOICES,
    ),
    parameter(
        10,
        "listen",
        "listen",
        "on/off",
        ParameterUnit::Linear,
        Boolean,
        None,
        0.0,
        Stepped,
        &[],
    ),
];

/// The two enumerations' values, in the runtime's vocabulary.
const CHOICE_VALUES: [f32; 2] = [1.0, SECOND_CHOICE];

/// One parameter's domain in `effect_runtime::params`'s vocabulary, read from the descriptor so
/// the two cannot drift. The mapping is a control-surface concern no render path reads.
const fn spec(index: usize) -> ParameterSpec {
    let d = &DE_ESSER_PARAMETERS[index];
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

/// No certified bound yet: the split and listen arms carry the SVF's ring-out, whose live
/// (frequency- and Q-ramped) tail and exact-rest bound is the same `math::tail` derivation the
/// parametric EQ's #1372 makes, so S1 states the conservative [`NodeTailBound::UNBOUNDED`], as the
/// EQ does today. The silent fast path does not need a bound: `lanes_at_rest` reads state.
fn tail_and_rest(_sample_rate: u32, _quality: EffectQuality) -> NodeTailBound {
    NodeTailBound::UNBOUNDED
}

/// State payload layout version.
pub const STATE_LAYOUT_VERSION: u32 = 1;

/// The one declared observation tap: the smoother's own gain word, negative for reduction.
pub const DE_ESSER_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
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
    bypass_observation: Some(0.0),
}];

/// Immutable descriptor for the launch de-esser contract.
pub const DE_ESSER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.de-esser"),
    display_name: "De-esser",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: LinkModeSet::ALL,
    lane_link: true,
    parameters: &DE_ESSER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &DE_ESSER_OBSERVATIONS,
};

/// The interval each live word stays inside, for a restore's path check: a parameter's domain,
/// the slope's, every rate design over the time domains at every launch rate (the 500 ms release
/// at 96 kHz, `2.08e-5`, to the 0.1 ms attack at 44.1 kHz, `0.2029`), the SVF words' `(0, 1)` and
/// `k = 1/Q`'s `[1/8, 2]`, the switches' `[0, 1]` and the shelf SVF words' `(0, 1)`.
const WORD_BOUNDS: [(f32, f32); RAMPS] = [
    (-60.0, 0.0),
    (1.0 / 20.0 - 1.0, 0.0),
    (0.0, 40.0),
    (2.0e-5, 0.203),
    (2.0e-5, 0.203),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.125, 2.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
];

/// The fifteen words the committed `values` design, in [`kernel`]'s ramp order.
///
/// The SVF words are the parametric EQ's `f64` design (`parametric_eq::design_svf_words_f64`, a
/// high-pass row at 0 dB, whose `c1, a2, a3` depend on `g = tan(pi f / fs)` and `k = 1/Q` only),
/// rounded once; `k` is the negated band mix word of that row. The shelf SVF's words (#1984) are
/// the same design at the same frequency with Butterworth damping, `Q = 1/sqrt 2`, whatever the
/// band's Q. Out of line, so the scalar `tan` and `exp` stay out of `process_bank`.
#[inline(never)]
fn targets(values: &[f32; PARAMETER_COUNT], sample_rate: u32) -> [f32; RAMPS] {
    let words = parametric_eq::design_svf_words_f64(
        parametric_eq::EqBandKind::HighPass,
        f64::from(values[0]),
        0.0,
        f64::from(values[1]),
        1.0,
        f64::from(sample_rate),
    );
    let shelf = parametric_eq::design_svf_words_f64(
        parametric_eq::EqBandKind::HighPass,
        f64::from(values[0]),
        0.0,
        core::f64::consts::FRAC_1_SQRT_2,
        1.0,
        f64::from(sample_rate),
    );
    let on = |value: f32| if value == SECOND_CHOICE { 1.0 } else { 0.0 };
    [
        values[2],
        1.0 / values[3] - 1.0,
        values[4],
        attack_release_coefficient(values[5], sample_rate),
        attack_release_coefficient(values[6], sample_rate),
        words[0] as f32,
        words[1] as f32,
        words[2] as f32,
        (-words[4]) as f32,
        on(values[7]),
        on(values[8]),
        values[9],
        shelf[0] as f32,
        shelf[1] as f32,
        shelf[2] as f32,
    ]
}

/// Factory for the launch de-esser.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeEsserFactory;

/// A prepared de-esser at one lane width. It keeps only the prepared values its own methods read.
struct DeEsser<L: Lane, const CONNECTED: bool> {
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

/// Every word of one channel's state, in payload order ([`CHAN_WORDS`]).
fn chan_words<L: Lane>(s: &Chan<L>) -> [L; CHAN_WORDS] {
    let mut words = [L::zero(); CHAN_WORDS];
    words[..SIGNAL_WORDS].copy_from_slice(&[
        s.main.ic1, s.main.ic2, s.side.ic1, s.side.ic2, s.run, s.side_run, s.gain_db,
    ]);
    for (chunk, ramp) in words[RAMP_WORDS]
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(&s.ramps)
    {
        chunk.copy_from_slice(&[ramp.current, ramp.target, ramp.step, ramp.remaining]);
    }
    words[SHELF_WORD..].copy_from_slice(&[s.shelf.ic1, s.shelf.ic2]);
    words
}

/// The inverse of [`chan_words`].
fn chan_from_words<L: Lane>(words: &[L; CHAN_WORDS]) -> Chan<L> {
    let mut s = Chan::default();
    [s.main.ic1, s.main.ic2, s.side.ic1, s.side.ic2] = [words[0], words[1], words[2], words[3]];
    [s.run, s.side_run, s.gain_db] = [words[4], words[5], words[6]];
    for (ramp, chunk) in s
        .ramps
        .iter_mut()
        .zip(words[RAMP_WORDS].as_chunks::<4>().0.iter())
    {
        [ramp.current, ramp.target, ramp.step, ramp.remaining] =
            [chunk[0], chunk[1], chunk[2], chunk[3]];
    }
    [s.shelf.ic1, s.shelf.ic2] = [words[SHELF_WORD], words[SHELF_WORD + 1]];
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

impl<L: Lane, const CONNECTED: bool> DeEsser<L, CONNECTED> {
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
    /// D1-D3).
    ///
    /// The lane's link words become `mode`'s at once, so a glide that arrives lands on `mode`'s
    /// own law; the weights, retargeted from the words in force, carry the transition, and the
    /// bank's ramp clock covers it. The mode is the one a full reset keeps and a lane swap moves.
    /// `false`, changing nothing, for a lane past the width or padded, a mode the descriptor does
    /// not support or a glide longer than `LINK_GLIDE_MAX_SAMPLES`.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= L::WIDTH
            || !self.is_active(lane)
            || !DE_ESSER_DESCRIPTOR.lane_link
            || !DE_ESSER_DESCRIPTOR.supported_link_modes.contains(mode)
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
        for (chunk, design) in words[RAMP_WORDS]
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
        words[SHELF_WORD..].fill(0.0);
        for chunk in words[RAMP_WORDS].as_chunks_mut::<4>().0.iter_mut() {
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
    /// other lane. The bank-level `ramp_frames_left` stays: it only chooses where `run_block`
    /// splits a block, and its invariant (`>=` every lane's `remaining`) still holds once this
    /// lane's ramps rest. `collapsed` stays as the whole-bank reset leaves it. A lane past the
    /// width changes nothing. Bounded by one lane's words.
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
    /// The three SVF words `c1, a2, a3` move as one triple: when any of their designs moves, all
    /// three restart together from their current values, so every triple the kernel loads is a
    /// convex combination of the current point and the new design, which is contractive
    /// (`effect_runtime::svf::transition_norm` is convex in the words). Skipping a word whose
    /// design bits did not move would leave it on its old schedule and the triple off that chord
    /// (the #1372a finding: a one-ulp frequency nudge can keep `a2`'s bits and move `c1`'s or
    /// `a3`'s).
    #[inline(never)]
    fn retarget(&mut self, channel: usize, lane: usize) {
        let designs = targets(&self.values[lane][channel], self.sample_rate);
        let mut words = lane_words(&self.state[channel], lane);
        let svf = C1..C1 + 3;
        let svf_moved = svf
            .clone()
            .any(|ramp| words[SIGNAL_WORDS + ramp * 4 + 1].to_bits() != designs[ramp].to_bits());
        let shelf = SHELF_C1..SHELF_C1 + 3;
        let shelf_moved = shelf
            .clone()
            .any(|ramp| words[SIGNAL_WORDS + ramp * 4 + 1].to_bits() != designs[ramp].to_bits());
        for (ramp, (chunk, design)) in words[RAMP_WORDS]
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(designs)
            .enumerate()
        {
            let restart =
                (svf_moved && svf.contains(&ramp)) || (shelf_moved && shelf.contains(&ramp));
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
        // Issue #1969: only the ramping prefix of a block with a glide in flight takes the blend.
        let glide = self.gliding::<MONO>();
        let shelf = self.shelf_can_be_live::<MONO>();
        let (first, rest) = self.state.split_at_mut(1);
        let mut run = |ramping: bool, range: core::ops::Range<usize>| {
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
            let arms = |run: L, held: <L as Lane>::Mask| {
                silence_armable_holding(run, frames, armed_after, held)
            };
            let channels = if MONO { 1 } else { 2 };
            let armable = states[..channels].iter().any(|s| {
                arms(s.run, svf_state_held([s.main, s.shelf]))
                    || (CONNECTED && arms(s.side_run, svf_state_held([s.side])))
            });
            let [state_left, state_right] = states;
            if !armable {
                let planes: [&[f32]; 2] = [left, right];
                let sides = sidechain.map_or([&[][..]; 2], |(l, r)| [l, r]);
                for (channel, s) in [&mut *state_left, &mut *state_right]
                    .into_iter()
                    .enumerate()
                    .take(channels)
                {
                    silence_skip_block(planes[channel], frames, &mut s.run);
                    if CONNECTED {
                        silence_skip_block(sides[channel], frames, &mut s.side_run);
                    }
                }
            }
            let args = Args {
                left,
                right,
                sidechain,
                armed_after,
                coef: (&self.coef[0], &self.coef[1]),
                state: (state_left, state_right),
            };
            match (ramping, armable, ramping && glide, shelf) {
                (true, true, true, false) => {
                    segment::<L, CONNECTED, true, MONO, true, true, false>(args);
                }
                (true, false, true, false) => {
                    segment::<L, CONNECTED, true, MONO, false, true, false>(args);
                }
                (true, true, false, false) => {
                    segment::<L, CONNECTED, true, MONO, true, false, false>(args);
                }
                (true, false, false, false) => {
                    segment::<L, CONNECTED, true, MONO, false, false, false>(args);
                }
                (false, true, _, false) => {
                    segment::<L, CONNECTED, false, MONO, true, false, false>(args);
                }
                (false, false, _, false) => {
                    segment::<L, CONNECTED, false, MONO, false, false, false>(args);
                }
                (true, true, true, true) => {
                    segment::<L, CONNECTED, true, MONO, true, true, true>(args);
                }
                (true, false, true, true) => {
                    segment::<L, CONNECTED, true, MONO, false, true, true>(args);
                }
                (true, true, false, true) => {
                    segment::<L, CONNECTED, true, MONO, true, false, true>(args);
                }
                (true, false, false, true) => {
                    segment::<L, CONNECTED, true, MONO, false, false, true>(args);
                }
                (false, true, _, true) => {
                    segment::<L, CONNECTED, false, MONO, true, false, true>(args);
                }
                (false, false, _, true) => {
                    segment::<L, CONNECTED, false, MONO, false, false, true>(args);
                }
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

    /// Whether any lane can be live on the shelf path this block (#1984): its mode and detector
    /// words are each nonzero or still ramping. Read at the block's start, after its automation;
    /// a lane that cannot be live already holds a `+0.0` shelf state (its last rendered frame was
    /// not live, or it was reset or restored so), so skipping the shelf step moves no bit.
    fn shelf_can_be_live<const MONO: bool>(&self) -> bool {
        let zero = L::zero();
        let path = |ramp: &WordRamp<L>| L::mask_or(ramp.current.gt(zero), ramp.remaining.gt(zero));
        let channels = if MONO { 1 } else { 2 };
        self.state[..channels]
            .iter()
            .any(|s| L::mask_any(L::mask_and(path(&s.ramps[MODE]), path(&s.ramps[DETECTOR]))))
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

    /// The lanes of one channel at exact rest: the three SVFs' integrators and the gain word `+0.0`
    /// and every ramp settled, the link weights included (issue #1969: a glide in flight moves
    /// its detector every frame, so it is never at rest and never skipped). The silence clocks are clocks, not state (a `+0.0` block moves
    /// them to `min(run + frames, 2^24)` whatever the signal was).
    ///
    /// Why that is a fixed point of a `+0.0` frame that writes `+0.0`: with `+0.0` input and
    /// integrators, `svf_step` gives `v1 = v2 = +0.0` and keeps them (armed or not, the flush
    /// keeps `+0.0`), so both taps, the shelf's low-pass and the detector tap are `+0.0`; the level floors at
    /// -160 dB, under every threshold, so the curve's target is `+0.0` (`gain_delta_db`'s identity
    /// arm, then `max(-range)` and `min(range)`, which give `+-0.0`); the rate step's rest snap
    /// (`effect_runtime::dynamic_band::CHANGE_REST_DB`) then writes `+0.0`, so the gain word stays; a zero gain word selects the dry `+0.0` on the
    /// wideband and split arms, the listen arm is the `+0.0` tap and bypass the dry word. A
    /// settled ramp is an exact no-op of both segment variants.
    fn channel_rest_lanes(&self, channel: usize) -> u32 {
        let s = &self.state[channel];
        [
            s.main.ic1,
            s.main.ic2,
            s.side.ic1,
            s.side.ic2,
            s.shelf.ic1,
            s.shelf.ic2,
            s.gain_db,
        ]
        .into_iter()
        .chain(s.ramps.iter().chain(&s.links).map(|ramp| ramp.remaining))
        .fold(all_lanes(L::WIDTH), |mask, word| {
            mask & positive_zero_lanes(word)
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
    /// words are scanned beside the output: a NaN gain or integrator need not reach the output
    /// (the wideband arm never reads the tap, and the level clamp swallows NaN). A failed lane is
    /// zeroed, re-seeded and, when it carries a member, reported. The collapsed body decides on
    /// the left plane and state and recovers and counts both channels, as a dual block on `L == R`
    /// would.
    fn finish_block<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        reports: &mut [ProcessReport; MAX_WIDTH],
    ) {
        let width = L::WIDTH;
        let channels = if MONO { 1 } else { 2 };
        for (channel, block) in [left, right].into_iter().enumerate().take(channels) {
            let s = &self.state[channel];
            let state = [
                s.gain_db,
                s.main.ic1,
                s.main.ic2,
                s.side.ic1,
                s.side.ic2,
                s.shelf.ic1,
                s.shelf.ic2,
            ];
            let mut words = [0.0_f32; 7 * MAX_WIDTH];
            for (index, word) in state.into_iter().enumerate() {
                word.store(&mut words[index * width..(index + 1) * width]);
            }
            let words = &words[..state.len() * width];
            if check_block::<L>(block) && check_block::<L>(words) {
                continue;
            }
            let output_failed = nonfinite_lane_mask::<L>(block);
            let failed = output_failed | nonfinite_lane_mask::<L>(words);
            for lane in (0..width).filter(|lane| failed & (1 << lane) != 0) {
                // A prepared-bypass lane emits its dry input, which is inside the D7 domain, so a
                // failure of the wet state alone recovers the state and keeps that output: the
                // lane renders what the graph's bypass shunt renders (#15 S2,
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
    /// three [`Coef`] words and every ramp's four fields, the link weights' included (#1969); the
    /// integrators, clocks and gain word are running state, and `defaults` and `values` are
    /// control-side words the kernel never loads.
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
            && designed(l)[RAMP_WORDS]
                .iter()
                .zip(&designed(r)[RAMP_WORDS])
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
    /// domains, finite integrators, whole clocks, a gain word in `[-40, 0]` dB, and each ramp
    /// aimed bit for bit at the design of the committed values, its path inside the word's
    /// interval ([`WORD_BOUNDS`]), settled at its target, and the SVF words contractive.
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
        let gain = words[6];
        if !check_block::<f32>(&words[..4])
            || !check_block::<f32>(&words[SHELF_WORD..])
            || !integral_within(words[4], RUN_MAX)
            || !integral_within(words[5], RUN_MAX)
            || !gain.is_finite()
            || !(-40.0..=0.0).contains(&gain)
            || is_negative_zero(gain)
        {
            return Err(state_error("effect.state.signal"));
        }
        let designs = targets(&values, self.sample_rate);
        for (index, chunk) in words[RAMP_WORDS].as_chunks::<4>().0.iter().enumerate() {
            let ramp = LinearRamp {
                current: chunk[0],
                target: chunk[1],
                step: chunk[2],
                remaining: chunk[3] as u32,
            };
            let valid = chunk[1].to_bits() == designs[index].to_bits()
                && integral_within(chunk[3], RAMP_SAMPLES as f32)
                && (chunk[3] != 0.0 || chunk[0].to_bits() == chunk[1].to_bits())
                && payload::ramp_path_inside(ramp, WORD_BOUNDS[index], RAMP_SAMPLES);
            if !valid {
                return Err(state_error("effect.state.parameter"));
            }
        }
        let current = |index: usize| words[SIGNAL_WORDS + index * 4];
        if [C1, SHELF_C1].into_iter().any(|c1| {
            transition_norm(current(c1), current(c1 + 1), current(c1 + 2))
                > RAMP_PATH_NORM_TOLERANCE
        }) {
            return Err(state_error("effect.state.parameter"));
        }
        // #1984: a lane that cannot be live on the shelf path holds an exact `+0.0` shelf state,
        // which a block that skips the shelf step relies on.
        let path = |ramp: usize| {
            let at = SIGNAL_WORDS + ramp * 4;
            words[at] != 0.0 || words[at + 3] != 0.0
        };
        if !(path(MODE) && path(DETECTOR)) && words[SHELF_WORD..].iter().any(|w| w.to_bits() != 0) {
            return Err(state_error("effect.state.signal"));
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
            .filter(|mode| DE_ESSER_DESCRIPTOR.supported_link_modes.contains(*mode))
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
            if restore.words[RAMP_WORDS]
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
}

/// One channel section of a parsed, validated payload.
struct LaneRestore {
    words: [f32; CHAN_WORDS],
    values: [f32; PARAMETER_COUNT],
    links: [[f32; 4]; LINK_COUNT],
    /// The mode the weights glide to, decoded from their targets.
    mode: LinkMode,
}

impl<const CONNECTED: bool> PreparedNativeEffect for DeEsser<f32, CONNECTED> {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        DeEsser::reset(self, kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut reports = [ProcessReport::default(); MAX_WIDTH];
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
        track == 0 && DeEsser::retarget_link(self, 0, mode, samples)
    }

    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = self.state[0].gain_db;
        out.right = self.state[1].gain_db;
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

impl<L: Lane> DeEsser<L, false> {
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
        impl PreparedNativeEffectBank for DeEsser<$lane, false> {
            fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
                DeEsser::reset_lane(self, lane, kind);
            }
            /// Every field is inline: this instance owns no heap (#1074).
            fn retained_bytes(&self) -> u64 {
                0
            }

            fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
                if tap_index != 0 || out.len() != <$lane as Lane>::WIDTH {
                    return false;
                }
                for (lane, sample) in out.iter_mut().enumerate() {
                    sample.left = lane_get(self.state[0].gain_db, lane);
                    sample.right = lane_get(self.state[1].gain_db, lane);
                }
                true
            }

            fn reset(&mut self, kind: ResetKind) {
                DeEsser::reset(self, kind);
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

            /// Issue #1969: `DeEsser::retarget_link`; a padded lane takes no glide.
            fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
                DeEsser::retarget_link(self, track, mode, samples)
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

/// The scalar instance [`DeEsserFactory::prepare`] boxes: keyed by a connected sidechain or by its own
/// input.
enum Scalar {
    Keyed(DeEsser<f32, true>),
    Internal(DeEsser<f32, false>),
}

/// The scalar instance [`DeEsserFactory::prepare`] boxes, and its metadata.
fn prepare_scalar(
    request: PrepareEffectRequest<'_>,
) -> Result<(Scalar, PreparedEffectMetadata), EffectPrepareError> {
    let metadata = expected_prepared_metadata(&DE_ESSER_DESCRIPTOR, request)?;
    let defaults = [initial_defaults(request.initial_values)?; MAX_WIDTH];
    let invalid = EffectPrepareError {
        code: "effect.parameter.initial",
    };
    let scalar = match metadata.ports.sidechain {
        PreparedSidechainPort::Connected { .. } => {
            Scalar::Keyed(DeEsser::<f32, true>::new(&metadata, None, defaults).ok_or(invalid)?)
        }
        _ => {
            Scalar::Internal(DeEsser::<f32, false>::new(&metadata, None, defaults).ok_or(invalid)?)
        }
    };
    Ok((scalar, metadata))
}

impl NativeEffectFactory for DeEsserFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &DE_ESSER_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        (size_of::<DeEsser<f32, true>>()).max(size_of::<DeEsser<f32, false>>()) as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let (scalar, metadata) = prepare_scalar(request)?;
        let processor: Box<dyn PreparedNativeEffect> = match scalar {
            Scalar::Keyed(instance) => Box::new(instance),
            Scalar::Internal(instance) => Box::new(instance),
        };
        Ok(PreparedEffect {
            processor,
            metadata,
        })
    }

    /// Issue #2199: the scalar instance `prepare` boxes, built inline and asked. It holds no heap,
    /// so the answer allocates nothing.
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        prepare_scalar(request).map(|(scalar, _)| match scalar {
            Scalar::Keyed(instance) => instance.channel_symmetry(),
            Scalar::Internal(instance) => instance.channel_symmetry(),
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
    let metadata = expected_prepared_metadata(&DE_ESSER_DESCRIPTOR, first)?;
    let mut defaults = [initial_defaults(first.initial_values)?; MAX_WIDTH];
    let mut links = [metadata.link_mode; MAX_WIDTH];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(&DE_ESSER_DESCRIPTOR, item)?;
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
        let mut bank = DeEsser::<L, false>::new(&metadata, Some(request.width), defaults)
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
