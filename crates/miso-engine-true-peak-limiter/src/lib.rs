//! Fixed-four-phase true-peak safety limiter.
//!
//! The audible path stays at the host sample rate; the frozen BS.1770-5 Annex-2 FIR is
//! detector-only. One generic block kernel, `limiter_block`, owns the frame loop for every
//! width: a scalar instance is the same body at `L = f32`, a W4/W8 bank is the same body at
//! `Simd4`/`Simd8` over an AoSoA arena. Nothing in this crate is per-sample scalar any more, and
//! nothing here names an intrinsic, a vector library or `unsafe`.
//!
//! # The gain law (issue #90, wave 2)
//!
//! With `N = Fs/100`, `T = N + 6` (the immutable declared latency), `R = N + 1`, lookahead
//! `L = round(lookahead_ms * Fs / 1000)` clamped to `0..=N`, and ramp window
//! `Wb = clamp(L + 1, 32, R)`:
//!
//! ```text
//! P[n]   = max(|h[6]|, |v0|, |v1|, |v2|, |v3|)          // Annex-2 four-phase estimate
//! r[n]   = if P[n] > limit { limit / P[n] } else { 1 }  // limit = 10^((ceiling - 1) / 20)
//! m[n]   = min(r[n-N ..= n-N+Wb-1])                     // sliding minimum, van Herk / Gil-Werman
//! m_q[n] = floor(m[n] * 16384) / 16384                  // exact 2^-14 grid
//! s[n]   = (m_q[n] + ... + m_q[n-Wb+1]) / Wb            // box ramp, exact running sum
//! d[n]   = max(1 - s[n], fma(c, (1 - s[n]) - d[n-1], d[n-1]))
//! g[n]   = 1 - d[n]
//! y[n]   = x[n-T] * g[n]
//! ```
//!
//! `g[n] <= r[n-N]` holds **by algebra**, not by corpus: every box term `m_q[n-j]`, `j < Wb`, is a
//! minimum over a window that contains `n-N`, so their average is at most `r[n-N]`; `d >= 1 - s`
//! forces `g <= s`. That is what replaces the old instantaneous step attack, whose full-bandwidth
//! gain discontinuity was the crate's sound-quality defect (#90 F2).
//!
//! # What is not here any more
//!
//! No `powf`/`exp` on the render path (#90 F1): `limit` and the release coefficient are designed in
//! `f64` by `miso-engine-math` at event time and ramped in the **linear** domain, so a coefficient
//! is never a transcendental of a per-sample value. No `%` on a cursor (#90 F6): rings are advanced
//! with a compare and a wrap. No per-value `is_finite`/`is_subnormal`/`Option` plumbing and no
//! per-lane recovery (#90 F5, decision D7): the only flush is on the single recursive word `d`, and
//! the only failure path is the once-per-block boundary check of `miso-engine-effect-runtime`. No
//! second copy of the ramp, the payload codec or the parameter validator (#90 F9): they come from
//! `miso-engine-effect-runtime`.
#![allow(missing_docs)]

pub mod corpus;

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
    ObservationFold, ObservationKind, ObservationSample, ObservationTapId, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{
    NonFiniteReport, block_is_positive_zero, check_block, finish_block, nonfinite_lane_mask,
};
use miso_engine_effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload::{
    HEADER_WORDS, StateLayout, read_f32, read_header, read_u32, validate_lengths, write_f32,
    write_header, write_u32,
};
use miso_engine_lane::{Backend, Lane, Simd4, Simd8, flush};

/// Parameters in the frozen descriptor.
const PARAMETER_COUNT: usize = 3;
/// Ramped parameters: ceiling and release. Lookahead is preparation-only.
const RAMP_COUNT: usize = 2;
/// Detector history words.
const HISTORY_WORDS: usize = 12;
/// Discrete alignment of the 23.5-high-rate-sample FIR group delay, in base-rate samples.
const FIR_ALIGNMENT_SAMPLES: usize = 6;
/// Widest backend, so per-frame scratch is a fixed-size array and never an allocation.
const MAXIMUM_WIDTH: usize = 8;
/// Frames of one detector pass; the peak scratch is `2 * DETECTOR_CHUNK * MAXIMUM_WIDTH` on the
/// stack, two kilobytes, and never an allocation.
const DETECTOR_CHUNK: usize = 32;
/// Updates in the frozen `SmoothingRule::Linear` de-zipper window.
const RAMP_UPDATES: u32 = 64;
/// Lane words before the three rings.
const LANE_HEADER_WORDS: usize = 27;

/// Quantisation grid of the box-ramp terms, `2^14`.
///
/// Every `m_q` is an integer multiple of `2^-14` in `[0, 1]`, so a running sum of at most
/// `R <= 961` of them is an integer multiple of `2^-14` strictly below `2^24`. Every partial sum is
/// therefore exactly representable in `f32` and every add and subtract of the sliding window is
/// exact: the box sum cannot drift, needs no periodic resynchronisation, and is partition
/// invariant. It is also exactly `Wb` when nothing is limiting, so `S / Wb` is exactly `1.0` and
/// the identity path stays bit-exact.
const BOX_GRID: f32 = 16_384.0;

/// Shortest box-ramp window, in samples (`W_MIN`).
///
/// A ramp shorter than the twelve-tap detector span re-creates the inter-sample overshoot the
/// detector has already measured. Thirty-two samples is 0.33 ms at 96 kHz and 0.73 ms at 44.1 kHz,
/// below any attack-time audibility threshold, and halves the ramp-rate modulation term against a
/// sixteen-sample floor. A lookahead of 0 ms therefore means "fastest ramp", never "step".
const MINIMUM_RAMP_WINDOW: u32 = 32;
const fn effect_id(value: &'static str) -> miso_engine_effect_contract::EffectId {
    match miso_engine_effect_contract::EffectId::new(value) {
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

const fn parameter_id(value: u32) -> ParameterId {
    match ParameterId::new(value) {
        Some(value) => value,
        None => panic!("nonzero parameter identifier"),
    }
}

#[allow(clippy::too_many_arguments)]
const fn parameter(
    id: u32,
    name: &'static str,
    display_unit: &'static str,
    unit: ParameterUnit,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    mapping: ParameterMapping,
    automation_rate: AutomationRate,
    smoothing: SmoothingRule,
    smoothing_samples: u32,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
        display_name: name,
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
        automatable: !matches!(automation_rate, AutomationRate::None),
        enum_choices: &[],
    }
}

/// Frozen descriptor rows. Descriptor position and stable numeric ID agree.
pub const TRUE_PEAK_LIMITER_PARAMETERS_V1: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(
        1,
        "ceiling",
        "dBTP-est",
        ParameterUnit::Db,
        -24.0,
        0.0,
        -1.0,
        ParameterMapping::Linear,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        2,
        "release",
        "ms",
        ParameterUnit::Milliseconds,
        10.0,
        2000.0,
        100.0,
        ParameterMapping::Logarithmic,
        AutomationRate::Block,
        SmoothingRule::Linear,
        64,
    ),
    parameter(
        3,
        "lookahead",
        "ms",
        ParameterUnit::Milliseconds,
        0.0,
        10.0,
        5.0,
        ParameterMapping::Linear,
        AutomationRate::None,
        SmoothingRule::None,
        0,
    ),
];

const PORTS: [PortDescriptor; 2] = [
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
];

/// The state-layout-2 resource row of one launch rate.
///
/// `lane_words = 27 + B + 2R = 3N + 35`: twenty-seven scalar words, the `B = N + 6` main-delay
/// ring, and the two `R = N + 1` gain rings the minimum filter and the box ramp need (layout 1 had
/// no box ring and no minimum-filter words, hence the re-pin). The common section is the two-word
/// version/length header `miso-engine-effect-runtime` stamps into every payload, which is why
/// `common_bytes` is eight and no longer zero. The latency column does not move.
const fn quality(rate: u32) -> miso_engine_effect_contract::QualityDescriptor {
    let lookahead_maximum = rate / 100;
    let lane_words = 3 * lookahead_maximum + 35;
    let lane_bytes = lane_words * 4;
    miso_engine_effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples((lookahead_maximum + 6) as u64),
        tail: TailSamples::Infinite,
        maximum_state: StatePayloadSizes {
            common_bytes: HEADER_WORDS * 4,
            left_bytes: lane_bytes,
            right_bytes: lane_bytes,
        },
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

const QUALITIES: [miso_engine_effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// The one declared observation tap: the recursive reduction word, **linear** (issue #143 R4).
///
/// `ChannelState::reduction` is `d` in the kernel's `gain = 1 - d` recursion, a linear magnitude in
/// `[0, 1]`. The tap declares `unit: Linear` and publishes exactly that word: converting it to
/// decibels would put a `log` on the render thread, and "resident = copy out" would stop being
/// literally true. The host converts once per closed window, on the control plane.
///
/// `display_unit`, `minimum` and `maximum` describe the value a **consumer** reads, after the
/// declared fold and after that one unit conversion -- decibels of reduction, `0 .. 100`. `unit`
/// describes what crosses the transport. They differ here and only here, and the difference is the
/// whole point of declaring the transport unit separately.
pub const TRUE_PEAK_LIMITER_OBSERVATIONS_V1: [ObservationDescriptor; 1] =
    [ObservationDescriptor {
        id: ObservationTapId(1),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Linear,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakMagnitude,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    }];

/// Immutable launch true-peak limiter descriptor.
pub const TRUE_PEAK_LIMITER_DESCRIPTOR_V1: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.true-peak-limiter"),
    display_name: "True-Peak Limiter",
    contract_major: 1,
    // Issue #143 P1: declaring the first tap is a `contract_minor` bump and a derived identity
    // re-pin of exactly `32 + len("Gain Reduction") + len("dB")` = 48 bytes.
    // `state_layout_version` does not move: the tap reads state that was already there.
    contract_minor: 1,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: match LinkModeSet::new(3) {
        Some(value) => value,
        None => panic!("frozen link bits"),
    },
    parameters: &TRUE_PEAK_LIMITER_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &TRUE_PEAK_LIMITER_OBSERVATIONS_V1,
};

/// The state layout this crate reads and writes.
///
/// Bumped from 1 by #90: the wave-2 gain law needs a minimum-filter phase and prefix, an exact box
/// sum, a box ring and a precomputed ramp step, none of which layout 1 can hold, and the payload
/// gained the runtime's two-word header. This is the one contract fixture the issue authorises
/// moving; the latency, the parameter table, the port table, the link set, the Annex-2 coefficients
/// and `scratch_fixed_bytes` are unchanged.
pub const STATE_LAYOUT_VERSION: u32 = 2;

/// Factory for the fixed-latency scalar limiter.
#[derive(Clone, Copy, Debug, Default)]
pub struct TruePeakLimiterFactory;

/// The exact Annex-2 four-phase detector table, indexed by history tap then phase.
const ANNEX2_FIR: [[f32; 4]; HISTORY_WORDS] = [
    [
        0.001_708_984_4,
        -0.029_174_805,
        -0.018_920_898,
        -0.008_300_781,
    ],
    [0.010_986_328, 0.029_296_875, 0.033_081_055, 0.014_892_578],
    [-0.019_653_32, -0.051_757_812, -0.058_227_54, -0.026_611_328],
    [0.033_203_125, 0.089_111_33, 0.101_562_5, 0.047_607_422],
    [-0.059_448_242, -0.166_503_9, -0.200_317_38, -0.102_294_92],
    [0.137_329_1, 0.465_087_9, 0.779_785_16, 0.972_167_97],
    [0.972_167_97, 0.779_785_16, 0.465_087_9, 0.137_329_1],
    [-0.102_294_92, -0.200_317_38, -0.166_503_9, -0.059_448_242],
    [0.047_607_422, 0.101_562_5, 0.089_111_33, 0.033_203_125],
    [-0.026_611_328, -0.058_227_54, -0.051_757_812, -0.019_653_32],
    [0.014_892_578, 0.033_081_055, 0.029_296_875, 0.010_986_328],
    [
        -0.008_300_781,
        -0.018_920_898,
        -0.029_174_805,
        0.001_708_984_4,
    ],
];

/// Per-rate ring shapes, fixed at preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Shape {
    /// `N = Fs/100`, the maximum lookahead in samples and the required-gain delay.
    n: usize,
    /// `R = N + 1`, the slot count of the required-gain and box rings.
    ring: usize,
    /// `B = N + 6`, the slot count of the main-delay ring; with read-before-write the delay is `B`.
    main: usize,
}

impl Shape {
    /// The shape of one launch rate, or `None` if the rate cannot produce one.
    fn new(sample_rate: u32) -> Option<Self> {
        let n = usize::try_from(sample_rate / 100).ok()?;
        if n == 0 {
            return None;
        }
        Some(Self {
            n,
            ring: n.checked_add(1)?,
            main: n.checked_add(FIR_ALIGNMENT_SAMPLES)?,
        })
    }

    /// Data words of one channel of one track.
    const fn lane_words(&self) -> usize {
        LANE_HEADER_WORDS + self.main + 2 * self.ring
    }

    /// The payload layout of one track at this rate.
    const fn layout(&self) -> StateLayout {
        StateLayout {
            version: STATE_LAYOUT_VERSION,
            common_words: 0,
            lane_words: self.lane_words() as u32,
        }
    }
}

/// The per-lane window offsets a lookahead produces.
///
/// `window` is `Wb`; `end_offset` is `Wb`, the ring distance from the write cursor to the newest
/// sample of the minimum window; `box_offset` is `R - Wb`, the ring distance to the box term
/// leaving the running sum. Both are stored so the render path adds and compares instead of
/// dividing (#90 F6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LaneShape {
    window: u32,
    end_offset: u32,
    box_offset: u32,
}

impl LaneShape {
    /// The window shape of `lookahead` samples at `shape`.
    fn new(lookahead: usize, shape: &Shape) -> Self {
        let window = (lookahead + 1).clamp(MINIMUM_RAMP_WINDOW as usize, shape.ring);
        Self {
            window: window as u32,
            end_offset: window as u32,
            box_offset: (shape.ring - window) as u32,
        }
    }
}

/// Lane-wide coefficients of one prepared instance or bank.
struct LimiterCoef<L: Lane> {
    /// The Annex-2 table, tap-major then phase, splatted once at preparation.
    fir: [[L; 4]; HISTORY_WORDS],
    /// `true` when the prepared link mode is [`LinkMode::Maximum`].
    link_max: bool,
    /// `true` when the whole effect is bypassed; the delay and every ring still advance.
    bypass: bool,
}

impl<L: Lane> LimiterCoef<L> {
    /// Splats the frozen table and records the two prepared booleans.
    fn new(link_max: bool, bypass: bool) -> Self {
        let mut fir = [[L::zero(); 4]; HISTORY_WORDS];
        for (tap, row) in fir.iter_mut().enumerate() {
            for (phase, value) in row.iter_mut().enumerate() {
                *value = L::splat(ANNEX2_FIR[tap][phase]);
            }
        }
        Self {
            fir,
            link_max,
            bypass,
        }
    }
}

/// The one cursor pair of a whole bank.
///
/// Every lane and both channels advance in lockstep and always have (#90 F3): keeping one pair
/// instead of `2 * W` removes the redundant state layout 1 carried per lane, and makes the ring
/// slot of a frame a single uniform index that a vector store can use.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Cursors {
    main: u32,
    ring: u32,
}

impl Cursors {
    /// Advances both cursors by a whole block, as `frames` per-sample steps would (#182 S2).
    ///
    /// The `%` here is not the one #90 F6 removed. F6 is about the *render path*: a cursor must not
    /// cost a division per sample, which is why [`limiter_block_body`] advances with a compare and
    /// a wrap. This runs once per block on a path that renders nothing at all, in the same position
    /// and for the same reason as the rotation arithmetic in [`commit_lane`].
    fn advance(&mut self, frames: usize, shape: &Shape) {
        self.main = ((self.main as usize + frames % shape.main) % shape.main) as u32;
        self.ring = ((self.ring as usize + frames % shape.ring) % shape.ring) as u32;
    }
}

/// One channel of one instance or bank: the AoSoA arena plus the planar small state.
///
/// Every ring is `slots * width` with lane `l` of slot `s` at `s * width + l`, so a frame of `W`
/// tracks is one contiguous vector load or store. Allocation happens here and only here, at
/// preparation.
#[derive(Debug)]
struct ChannelState {
    width: usize,
    /// `HISTORY_WORDS * width`, tap-major.
    history: Box<[f32]>,
    /// `B * width` main-delay ring.
    main_ring: Box<[f32]>,
    /// `R * width` required-gain ring; the van Herk suffix minima overwrite expired raw values.
    required_ring: Box<[f32]>,
    /// `R * width` box ring of quantised minima.
    box_ring: Box<[f32]>,
    /// The recursive reduction word, one per lane. The only word the D7 flush applies to.
    reduction: Box<[f32]>,
    /// Running minimum of the current van Herk block, one per lane.
    prefix: Box<[f32]>,
    /// Exact running box sum, one per lane.
    box_sum: Box<[f32]>,
    /// Position inside the current van Herk block, one per lane.
    phase: Box<[u32]>,
    /// Linear-domain limit ramp, one per lane.
    limit: Box<[LinearRamp]>,
    /// Linear-domain release-coefficient ramp, one per lane.
    release: Box<[LinearRamp]>,
    /// The prepared lookahead of each lane, in milliseconds; serialised, never ramped.
    lookahead_ms: Box<[f32]>,
    /// The window offsets each lane's lookahead produced.
    lane: Box<[LaneShape]>,
}

impl ChannelState {
    /// Allocates one channel of `width` lanes and seeds it with each lane's defaults.
    fn new(width: usize, shape: &Shape, defaults: &[[f32; PARAMETER_COUNT]], rate: u32) -> Self {
        debug_assert_eq!(defaults.len(), width);
        let mut state = Self {
            width,
            history: vec![0.0; HISTORY_WORDS * width].into_boxed_slice(),
            main_ring: vec![0.0; shape.main * width].into_boxed_slice(),
            required_ring: vec![1.0; shape.ring * width].into_boxed_slice(),
            box_ring: vec![1.0; shape.ring * width].into_boxed_slice(),
            reduction: vec![0.0; width].into_boxed_slice(),
            prefix: vec![1.0; width].into_boxed_slice(),
            box_sum: vec![0.0; width].into_boxed_slice(),
            phase: vec![0; width].into_boxed_slice(),
            limit: vec![LinearRamp::fixed(0.0); width].into_boxed_slice(),
            release: vec![LinearRamp::fixed(0.0); width].into_boxed_slice(),
            lookahead_ms: vec![0.0; width].into_boxed_slice(),
            lane: vec![LaneShape::new(0, shape); width].into_boxed_slice(),
        };
        state.reset_to_defaults(shape, defaults, rate);
        state
    }

    /// Copies every word `source` carries onto this channel: the collapse's disengage boundary.
    ///
    /// # The copy list, term by term
    ///
    /// This kernel's per-channel state is **all** of it, and the reason the list is exhaustive
    /// rather than selective is that a collapsed block touches nearly every word: the detector
    /// writes `history`, `channel_frame` writes `main_ring`, `required_ring`, `box_ring` and
    /// `reduction`, and the uniform body additionally writes `prefix`, `box_sum` and `phase`. The
    /// two ramps are advanced per frame off the hot copy and stored back, and `lane`/`lookahead_ms`
    /// are the window shape the segment walk reads.
    ///
    /// | word | why it is here |
    /// |---|---|
    /// | `history` | the twelve oversampling taps -- the detector's whole cross-block state. |
    /// | `main_ring` | the `B`-sample delay line the output is read out of. A partial copy would emit pre-collapse samples `N + 6` frames later. |
    /// | `required_ring`, `box_ring` | the van Herk sliding-minimum rings. |
    /// | `reduction` | the recursive release word, the only one the D7 flush applies to. |
    /// | `prefix`, `box_sum`, `phase` | the uniform body's three van Herk registers, written back once per block. |
    /// | `limit`, `release` | all four fields of each ramp: only the collapsed channel's were advanced. |
    /// | `lookahead_ms`, `lane` | the window shape `segment` and `LaneShape` read. Not moved by a rendered block, and copied anyway, because "whole per-channel state" is the rule that survives a later field being added. |
    ///
    /// `width` is a preparation shape both channels share and is asserted rather than copied.
    /// # Which entries are individually gated, and which are here by the rule
    ///
    /// `crates/miso-engine-true-peak-limiter/tests/mono_collapse.rs` fails if any of `history`,
    /// `main_ring`, `required_ring`, `box_ring`, `reduction`, `box_sum`, `limit` or `release` is
    /// dropped.
    ///
    /// Four entries are **not** individually red, and the two groups are not the same kind of
    /// thing:
    ///
    /// * `lookahead_ms` and `lane` are the prepared window shape. No rendered block writes them --
    ///   they move only at prepare, restore and a full reset, none of which is reachable on a bound
    ///   bank -- so nothing can make them diverge today.
    /// * `prefix` and `phase` are the uniform body's two van Herk registers, and they *are* running
    ///   state: `UniformHot::new` loads them out of this arena and the block write-back stores them
    ///   into it. A collapsed block advances only the left channel's. Dropping either IS red --
    ///   via the whole-strip transition oracle
    ///   (`chain_shape::a_run_that_stops_collapsing_renders_what_a_never_collapsed_run_renders`),
    ///   not this crate's local corpus -- which is why an earlier draft mislabelled them ungated.
    ///   (A yet-earlier draft asserted re-derivation at the next block boundary, which is not true
    ///   of either word.)
    ///
    /// All four are copied because the rule is *whole per-channel state*, which is the rule
    /// precisely so that a word nobody has a divergence for is still carried.
    fn copy_state_from(&mut self, source: &Self) {
        debug_assert_eq!(self.width, source.width);
        debug_assert_eq!(self.main_ring.len(), source.main_ring.len());
        self.history.copy_from_slice(&source.history);
        self.main_ring.copy_from_slice(&source.main_ring);
        self.required_ring.copy_from_slice(&source.required_ring);
        self.box_ring.copy_from_slice(&source.box_ring);
        self.reduction.copy_from_slice(&source.reduction);
        self.prefix.copy_from_slice(&source.prefix);
        self.box_sum.copy_from_slice(&source.box_sum);
        self.phase.copy_from_slice(&source.phase);
        self.limit.copy_from_slice(&source.limit);
        self.release.copy_from_slice(&source.release);
        self.lookahead_ms.copy_from_slice(&source.lookahead_ms);
        self.lane.copy_from_slice(&source.lane);
    }

    /// `FullToDefaults`: every runtime word cleared and every ramp snapped to the prepared value.
    fn reset_to_defaults(&mut self, shape: &Shape, defaults: &[[f32; PARAMETER_COUNT]], rate: u32) {
        for (lane, values) in defaults.iter().enumerate() {
            self.lookahead_ms[lane] = values[2];
            self.lane[lane] = LaneShape::new(lookahead_samples(values[2], rate, shape.n), shape);
            self.limit[lane] = LinearRamp::fixed(limit_coefficient(values[0]));
            self.release[lane] = LinearRamp::fixed(release_coefficient(values[1], rate));
        }
        self.clear_runtime(shape);
    }

    /// `DiscontinuityKeepParameters`: the same runtime words, ramps snapped to their targets.
    fn reset_keeping_parameters(&mut self, shape: &Shape) {
        for (limit, release) in self.limit.iter_mut().zip(self.release.iter_mut()) {
            limit.snap();
            release.snap();
        }
        self.clear_runtime(shape);
    }

    /// Clears history, rings and the recursive word to the state a silent lane rests in.
    ///
    /// The box ring rests at `1.0` and the box sum at `Wb`, which is the only pair consistent with
    /// "nothing has ever been limited": `S / Wb` is then exactly `1.0`, `d` is exactly `+0.0` and
    /// the first output sample is the delayed input bit for bit.
    fn clear_runtime(&mut self, shape: &Shape) {
        debug_assert_eq!(self.main_ring.len(), shape.main * self.width);
        self.history.fill(0.0);
        self.main_ring.fill(0.0);
        self.required_ring.fill(1.0);
        self.box_ring.fill(1.0);
        self.reduction.fill(0.0);
        self.prefix.fill(1.0);
        self.phase.fill(0);
        for (sum, shape) in self.box_sum.iter_mut().zip(self.lane.iter()) {
            *sum = shape.window as f32;
        }
    }

    /// `true` when every runtime word of this channel is exactly what [`clear_runtime`] writes.
    ///
    /// Issue #182 S2, the observation half of the silent fixed point. The list is
    /// [`clear_runtime`]'s own, word for word, and that is the point: the rest state is not a
    /// property this function invents, it is the state the crate already documents a silent lane
    /// rests in, read back rather than assumed.
    ///
    /// The one member of [`clear_runtime`]'s list that is deliberately **absent** is `phase`. A
    /// resting channel is at whatever van Herk position its history left it at, and the position is
    /// unobservable while the rest of the state holds: with `required_ring` and `prefix` entirely
    /// `1.0`, [`sliding_minimum`] returns `1.0` from every position and its backward pass writes
    /// `1.0` over `1.0`. Requiring `phase == 0` would refuse the claim for all but one frame in
    /// `Wb`, which is a correctness-free way to never engage.
    ///
    /// `history` is *not* absent, and it is the one entry that is not obvious. A channel can reach
    /// `required_ring == 1.0` everywhere with a non-zero detector history — that only needs the
    /// twelve stale taps to estimate a peak at or under the ceiling, which quiet material does all
    /// the time. Freezing the history there would mean the first eleven frames of the tone that
    /// ends the silence are estimated against samples from before it, so the fast path has to wait
    /// for the history to drain like everything else.
    ///
    /// Bits, not values, at every word. `-0.0 == 0.0` and `1.0 - 2^-24 != 1.0`, but the fast path
    /// promises not to move a bit, so the test that licenses it has to be a bit test.
    ///
    /// [`clear_runtime`]: ChannelState::clear_runtime
    fn is_at_silent_rest(&self) -> bool {
        block_is_positive_zero(&self.history)
            && block_is_positive_zero(&self.main_ring)
            && block_is_positive_zero(&self.reduction)
            && all_exactly_one(&self.required_ring)
            && all_exactly_one(&self.box_ring)
            && all_exactly_one(&self.prefix)
            && self
                .box_sum
                .iter()
                .zip(self.lane.iter())
                .all(|(sum, shape)| sum.to_bits() == (shape.window as f32).to_bits())
    }

    /// Advances each lane's van Herk phase by a whole block, as the frame loop would (#182 S2).
    ///
    /// Per lane, because `window` is a per-lane preparation parameter — this is the control-plane
    /// mirror of the same fact [`lanes_uniform`] gates on. The phase cycles `0 .. Wb - 1` one step
    /// per frame, so `frames` steps land on `(phase + frames) mod Wb`.
    fn advance_rest_phase(&mut self, frames: usize) {
        for (phase, shape) in self.phase.iter_mut().zip(self.lane.iter()) {
            let window = shape.window as usize;
            *phase = ((*phase as usize + frames % window) % window) as u32;
        }
    }
}

/// `true` when every word of `values` is **exactly** the `f32` `1.0`, by bit pattern.
///
/// The identity element of this kernel's three multiplicative rings, and the counterpart of
/// [`block_is_positive_zero`] for them: a ring of exact `1.0` returns `1.0` from any cursor
/// position, which is what lets a skipped block leave it untouched and still be bit-identical.
fn all_exactly_one(values: &[f32]) -> bool {
    const ONE: u32 = 1.0_f32.to_bits();
    values.iter().all(|value| value.to_bits() == ONE)
}

/// `true` when no lane of `ramps` has a window open and every lane holds exactly its target.
///
/// Issue #144 item 6. Both halves are needed and both are exact: `remaining == 0` is the D11
/// statement that no ramp is in flight, and the bit compare is the statement that the value in
/// force *is* the target, so that reading `current` for the whole block reproduces what
/// [`RampLanes::advance`] would have produced sample by sample. Tolerances are not used anywhere
/// in this decision -- an epsilon here would make the optimisation a re-tuning.
fn ramps_are_stationary(ramps: &[LinearRamp]) -> bool {
    ramps
        .iter()
        .all(|ramp| ramp.remaining == 0 && ramp.current.to_bits() == ramp.target.to_bits())
}

/// A linear ramp of one coefficient, held as lanes for the block loop.
#[derive(Clone, Copy)]
struct RampLanes<L: Lane> {
    current: L,
    target: L,
    step: L,
    remaining: L,
}

impl<L: Lane> RampLanes<L> {
    /// Gathers `width` scalar ramps into one lane-wide ramp.
    #[inline]
    fn gather(ramps: &[LinearRamp]) -> Self {
        let mut current = [0.0_f32; MAXIMUM_WIDTH];
        let mut target = [0.0_f32; MAXIMUM_WIDTH];
        let mut step = [0.0_f32; MAXIMUM_WIDTH];
        let mut remaining = [0.0_f32; MAXIMUM_WIDTH];
        for (lane, ramp) in ramps.iter().enumerate() {
            current[lane] = ramp.current;
            target[lane] = ramp.target;
            step[lane] = ramp.step;
            remaining[lane] = f32::from(ramp.remaining as u16);
        }
        Self {
            current: L::load(&current),
            target: L::load(&target),
            step: L::load(&step),
            remaining: L::load(&remaining),
        }
    }

    /// Writes the lane-wide ramp back into `width` scalar ramps.
    #[inline]
    fn scatter(self, ramps: &mut [LinearRamp]) {
        let mut current = [0.0_f32; MAXIMUM_WIDTH];
        let mut remaining = [0.0_f32; MAXIMUM_WIDTH];
        let mut step = [0.0_f32; MAXIMUM_WIDTH];
        self.current.store(&mut current);
        self.remaining.store(&mut remaining);
        self.step.store(&mut step);
        for (lane, ramp) in ramps.iter_mut().enumerate() {
            ramp.current = current[lane];
            ramp.step = step[lane];
            ramp.remaining = remaining[lane] as u32;
        }
    }

    /// Produces this sample's value and advances the ramp (decision D11).
    ///
    /// `remaining = max(remaining - 1, 0)` then `current = select(remaining > 0, current + step,
    /// target)`: the last ramping sample is an assignment of the target, never an addition, which
    /// is exactly `LinearRamp::next_value` and is why a block boundary is not observable. `step` is
    /// cleared on the snap so a resting ramp cannot drift.
    #[inline(always)]
    fn advance(&mut self) -> L {
        let zero = L::zero();
        self.remaining = self.remaining.sub(L::splat(1.0)).max(zero);
        let stepping = self.remaining.gt(zero);
        self.current = L::select(stepping, self.current.add(self.step), self.target);
        self.step = L::select(stepping, self.step, zero);
        self.current
    }

    /// This sample's value when the ramp is known to be stationary, advancing nothing.
    ///
    /// Issue #144 item 6. Unlike the compressor and the gate, this effect had no ramping split at
    /// all: [`RampLanes::advance`] ran four times per frame, every frame, whether or not anything
    /// was moving -- and outside a sixty-four-sample window after a ceiling or release change,
    /// nothing ever is. At rest `advance` computes `remaining = max(0 - 1, 0) = 0`, `stepping =
    /// false`, `current = select(false, .., target) = target` and `step = select(false, step, 0)
    /// = 0`; with the rest invariant `current == target` bitwise and `step == 0` already true,
    /// every one of those is the identity. Returning `current` is therefore the same value and
    /// the same state, which is what makes the skip class A rather than a re-tuning.
    #[inline(always)]
    const fn resting_value(&self) -> L {
        self.current
    }
}

/// `10^((ceiling_db - 1) / 20)`, designed in `f64` and rounded once to `f32`.
///
/// The `-1.0` is the frozen internal estimator guard: it covers the 4x Annex-2 estimator's
/// worst-case under-read and the residual modulation of the ramp. Reducing it needs the measurement
/// this job produces and belongs to #49 (#90 F7). Evaluated through `miso-engine-math` so the bits
/// are the same on every target (D6); the render path never sees a transcendental.
fn limit_coefficient(ceiling_db: f32) -> f32 {
    miso_engine_math::db_to_gain(f64::from(ceiling_db) - 1.0) as f32
}

/// `1 - exp(-1 / (0.001 * release_ms * Fs))`, designed in `f64` and rounded once to `f32`.
///
/// The rate form, not the pole: `d += c * (target - d)` moves toward the target at this rate.
fn release_coefficient(release_ms: f32, sample_rate: u32) -> f32 {
    (1.0 - miso_engine_math::exp(-1.0 / (0.001 * f64::from(release_ms) * f64::from(sample_rate))))
        as f32
}

/// `round(lookahead_ms * Fs / 1000)` clamped to `0..=maximum`. Preparation only.
fn lookahead_samples(value: f32, sample_rate: u32, maximum: usize) -> usize {
    let samples = (f64::from(value) * f64::from(sample_rate) / 1000.0 + 0.5).floor();
    if !samples.is_finite() || samples < 0.0 {
        return 0;
    }
    (samples as usize).min(maximum)
}

/// `select(a < b, a, b)` for scalars: decision D8, so the minimum filter and the lane `min` agree.
#[inline(always)]
fn scalar_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// The twelve detector taps of one channel, one named field per tap.
///
/// Round 2 R2, a **data-residence** change and nothing else. The taps were a `[L; 12]`, and on the
/// wasm target that array is where the frame loop went wrong: LLVM idiom-recognises the twelve-word
/// shift of an array as a block move, so each frame emitted a 192-byte `memory.copy` and then
/// twelve `v128.load`s to read the taps it had just copied. Linear memory is not a register file,
/// and the detector is latency-bound, so a store-to-load round trip per tap per frame lands
/// directly on the critical path.
///
/// Twelve named fields cannot be memmoved. The shift becomes twelve local-to-local moves that
/// register allocation coalesces away, and every tap read is a live value rather than a load.
///
/// Nothing about the arithmetic moves: [`History::shift`] writes exactly the assignments the
/// `while tap > 0` loop wrote, in the same order, and [`annex2_phases`] walks the same taps against
/// the same table rows in the same tap-major order with the same twelve separately rounded
/// `add(mul(..))` steps per accumulator. The native target reaches the same code either way once
/// SROA has promoted the array, which is why this is a wasm change with a native no-op attached.
#[derive(Clone, Copy)]
struct History<L: Lane> {
    t0: L,
    t1: L,
    t2: L,
    t3: L,
    t4: L,
    t5: L,
    t6: L,
    t7: L,
    t8: L,
    t9: L,
    t10: L,
    t11: L,
}

/// The struct has one field per `HISTORY_WORDS` tap, and `t6` is the alignment sample.
const _: () = assert!(HISTORY_WORDS == 12 && FIR_ALIGNMENT_SAMPLES == 6);

impl<L: Lane> History<L> {
    /// Reads the twelve tap-major words of `state` into locals, once per block.
    #[inline]
    fn load(state: &ChannelState) -> Self {
        let width = state.width;
        let tap = |index: usize| L::load(&state.history[index * width..]);
        Self {
            t0: tap(0),
            t1: tap(1),
            t2: tap(2),
            t3: tap(3),
            t4: tap(4),
            t5: tap(5),
            t6: tap(6),
            t7: tap(7),
            t8: tap(8),
            t9: tap(9),
            t10: tap(10),
            t11: tap(11),
        }
    }

    /// Writes the twelve locals back into `state`, once per block.
    #[inline]
    fn store(self, state: &mut ChannelState) {
        let width = state.width;
        let mut tap = |index: usize, word: L| word.store(&mut state.history[index * width..]);
        tap(0, self.t0);
        tap(1, self.t1);
        tap(2, self.t2);
        tap(3, self.t3);
        tap(4, self.t4);
        tap(5, self.t5);
        tap(6, self.t6);
        tap(7, self.t7);
        tap(8, self.t8);
        tap(9, self.t9);
        tap(10, self.t10);
        tap(11, self.t11);
    }

    /// All twelve taps `+0.0`, the rest state of a silent lane.
    ///
    /// Test-only: the render path reaches the rest state through [`History::load`] of an arena
    /// `clear_runtime` has already zeroed, so a second constructor on it would be dead code.
    #[cfg(test)]
    #[inline(always)]
    fn zero() -> Self {
        let zero = L::zero();
        Self {
            t0: zero,
            t1: zero,
            t2: zero,
            t3: zero,
            t4: zero,
            t5: zero,
            t6: zero,
            t7: zero,
            t8: zero,
            t9: zero,
            t10: zero,
            t11: zero,
        }
    }

    /// Every tap moves up one and `x` becomes tap 0.
    ///
    /// Written out because the point is that it is *not* a block move: these are the same twelve
    /// assignments the array form made, oldest first, so no tap can read a value the shift has
    /// already overwritten.
    #[inline(always)]
    fn shift(&mut self, x: L) {
        self.t11 = self.t10;
        self.t10 = self.t9;
        self.t9 = self.t8;
        self.t8 = self.t7;
        self.t7 = self.t6;
        self.t6 = self.t5;
        self.t5 = self.t4;
        self.t4 = self.t3;
        self.t3 = self.t2;
        self.t2 = self.t1;
        self.t1 = self.t0;
        self.t0 = x;
    }
}

/// Shifts the history and returns `P[n] = max(|h[6]|, |v0|, |v1|, |v2|, |v3|)`.
///
/// The FIR is tap-major and lockstep across lanes: for each phase the accumulator starts at exactly
/// `+0.0` and takes twelve separately rounded `add(mul(...))` steps in increasing tap order, which
/// is the frozen order of the 016 brief and therefore bit-identical to the scalar detector this
/// replaces (#90 F4). No fusion, no reassociation, no horizontal work.
///
/// The sample term is `|h[6]|`, not `|h[0]|`: `h[6]` is the input sample the four phases are
/// centred on, so the estimate and the phases now describe the same instant. Layout 1 compared the
/// phases against a sample six frames in the future, which is the sole reason its gain law needed a
/// six-sample hold.
#[inline(always)]
fn detector_peak<L: Lane>(history: &mut History<L>, x: L, fir: &[[L; 4]; HISTORY_WORDS]) -> L {
    history.shift(x);
    let mut peak = history.t6.abs();
    for phase in annex2_phases(history, fir) {
        peak = peak.max(phase.abs());
    }
    peak
}

/// The four Annex-2 phase outputs of a history, tap-major and lockstep across lanes.
///
/// Each accumulator starts at exactly `+0.0` and takes twelve separately rounded `add(mul(..))`
/// steps in increasing tap order. Walking taps on the outside and phases on the inside reads the
/// table in its stored order and keeps each lane's summation order exactly the one the 016 brief
/// froze, which is why the reorder is bit-preserving (#90 F4).
///
/// The twelve steps are written out rather than iterated (round 2 R2). The order is the loop's,
/// tap for tap and phase for phase; what the unrolling buys is that each table row is read as a
/// single-use load feeding its multiply — the wasm backend sinks such a load into its consumer,
/// where a hoisted row would have had to be kept live — and that the four accumulators are four
/// values rather than an array a backend might decide to spill.
#[inline(always)]
fn annex2_phases<L: Lane>(history: &History<L>, fir: &[[L; 4]; HISTORY_WORDS]) -> [L; 4] {
    let mut phase0 = L::zero();
    let mut phase1 = L::zero();
    let mut phase2 = L::zero();
    let mut phase3 = L::zero();
    macro_rules! tap {
        ($index:literal, $sample:expr) => {{
            let row = &fir[$index];
            let sample = $sample;
            phase0 = phase0.add(row[0].mul(sample));
            phase1 = phase1.add(row[1].mul(sample));
            phase2 = phase2.add(row[2].mul(sample));
            phase3 = phase3.add(row[3].mul(sample));
        }};
    }
    tap!(0, history.t0);
    tap!(1, history.t1);
    tap!(2, history.t2);
    tap!(3, history.t3);
    tap!(4, history.t4);
    tap!(5, history.t5);
    tap!(6, history.t6);
    tap!(7, history.t7);
    tap!(8, history.t8);
    tap!(9, history.t9);
    tap!(10, history.t10);
    tap!(11, history.t11);
    [phase0, phase1, phase2, phase3]
}

/// `true` when every lane of `state` shares one window shape **and** one van Herk phase.
///
/// Issue #182 S1, the uniform-cohort gate. Lookahead is a per-lane preparation parameter, so
/// [`sliding_minimum`] and the box-expiry gather of [`channel_frame`] address the arena lane by
/// lane: two lanes of one bank can hold different `window`, `end_offset` and `box_offset`, and can
/// therefore be at different points of their van Herk blocks. Those two steps are the only scalar
/// work left in the kernel, and they are the majority of it.
///
/// In practice a cohort is uniform: `bind_homogeneous_bank` only admits members that share a
/// program key, and a lookahead difference is the ordinary reason two tracks are in the same
/// cohort with different windows. So the kernel takes **one** whole-bank branch, exactly as the
/// `stationary` hoist of #144 item 6 does, and runs the lane-wide form under it; anything else
/// falls back to the per-lane body, which is unchanged. Per-lane branching stays forbidden: one
/// track's arithmetic must never depend on which lane of which cohort it landed in.
///
/// Both legs are bit compares of integers, never tolerances. The shape leg is a derived-value
/// compare on [`LaneShape`], which is `Eq`. The **phase** leg is not redundant with it: lanes that
/// share a window advance their phase in lockstep for as long as they only render, but
/// [`commit_lane`] writes one lane's `phase` from a payload, so restoring a single track of a bank
/// can leave a cohort with one shape and several phases. Reading `state.phase[0]` for the whole
/// bank there would render the other lanes at the wrong window position, which is why the phase is
/// in the gate and why `a_restore_that_desyncs_the_phase_falls_back` exists.
fn lanes_uniform(state: &ChannelState) -> bool {
    state.lane.iter().all(|shape| *shape == state.lane[0])
        && state.phase.iter().all(|phase| *phase == state.phase[0])
}

/// The `W` contiguous words of ring slot `slot`, as one lane.
///
/// Round 2 R1(c). The uniform path addresses every ring with the **constant** `L::WIDTH` rather
/// than the runtime `ChannelState::width` the per-lane body must use. The two are equal — the
/// arena is allocated at `L::WIDTH` and [`limiter_block_uniform`] debug-asserts it — but only the
/// constant is a constant: with it a slot stride is a shift instead of an `imul`, and the
/// sub-slice handed to [`Lane::load`] has a statically known length, so the width check inside
/// `load` folds away and one of the two bounds checks per access disappears.
#[inline(always)]
fn ring_lane<L: Lane>(ring: &[f32], slot: usize) -> L {
    let base = slot * L::WIDTH;
    L::load(&ring[base..base + L::WIDTH])
}

/// [`ring_lane`]'s counterpart: writes one lane over the `W` contiguous words of ring slot `slot`.
#[inline(always)]
fn store_ring_lane<L: Lane>(ring: &mut [f32], slot: usize, value: L) {
    let base = slot * L::WIDTH;
    value.store(&mut ring[base..base + L::WIDTH]);
}

/// `value - ring` once `value` has passed the ring's end.
///
/// The render path's only form of the modulo (#90 F6). Every call site holds `value < 2 * ring`,
/// which is what makes one conditional subtraction exact; the uniform block loop calls it once per
/// *segment* rather than once per frame (R1 d).
#[inline(always)]
const fn wrapped(value: usize, ring: usize) -> usize {
    if value >= ring { value - ring } else { value }
}

/// [`sliding_minimum`] for a bank whose lanes are known uniform by [`lanes_uniform`].
///
/// Same algorithm, same operation order, one lane-wide instance of it instead of `W` scalar ones.
/// The window offsets and the phase are read once from lane 0 — which the gate has established is
/// every lane's — so every index this computes is the index the scalar body computes for *each*
/// lane, and the AoSoA layout makes the `W` words at that index one contiguous vector.
///
/// Bit identity is structural rather than empirical. [`Lane::min`] is defined as
/// `select(self < b, self, b)` (decision D8) and [`scalar_min`] is `if a < b { a } else { b }`, so
/// a lane of `a.min(b)` *is* `scalar_min(a, b)` — including on the two zeros and on NaN, where the
/// definition is deliberately asymmetric. Argument order therefore has to be preserved exactly,
/// and each of the three `min` sites below keeps the operand order its scalar original has.
/// `L::load`/`L::store` move the same words the indexed reads and writes move.
///
/// The amortised backward suffix pass is included: it is `Wb` loads, mins and stores once per
/// completed block, and it is the single largest scalar cost in the kernel.
///
/// # Residency (round 2, R1 a and b)
///
/// Three round trips through memory are gone from the frame and nothing else is.
///
/// * `prefix` and the van Herk `phase` are `&mut` locals of [`limiter_block_uniform`] instead of
///   arena words, written back once when the block ends.
/// * The window minimum is **returned** as an `L` instead of being stored into a `[f32; 8]`
///   scratch for the caller to load straight back out of.
/// * `end` and `start` arrive already resolved, because the caller walks the block in wrap-free
///   segments and therefore knows both indices are linear inside one (R1 d). They are the same two
///   indices the `+ offset` / `- ring` pair computed here before.
///
/// None of the three moves a *value*, and the first is not even a new idea: [`HotChannel`] already
/// holds the recursive reduction word, the box sum, the twelve detector taps and all four ramp
/// words in registers for a whole block and writes them back once at the end. `prefix` and `phase`
/// join that set; they were only ever left in the arena because the scalar body had to address
/// them lane by lane. The word this frame would have left in `state.prefix` is the word the local
/// now holds, and the block-end write-back leaves exactly what the last frame's store would have
/// left; `state.phase` is filled from the local for the same reason, every lane
/// of a uniform cohort holding one phase being precisely what [`lanes_uniform`] established. The
/// state words move in *when* they are written, never in what is written, and nothing observes
/// them between two frames of one block — `snapshot_track` and `is_at_silent_rest` read the arena
/// between blocks. That is the same licence the #182 S2 cursor note relies on when it advances the
/// cursors and the rest phase of a block it skipped instead of running it: mid-block state is not
/// observable, so only the state a block *ends* on has to match.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn sliding_minimum_uniform<L: Lane>(
    required_ring: &mut [f32],
    window: usize,
    ring: usize,
    end: usize,
    start: usize,
    prefix: &mut L,
    phase: &mut u32,
) -> L {
    let newest = ring_lane::<L>(required_ring, end);
    let position = *phase as usize;
    let running = if position == 0 {
        newest
    } else {
        (*prefix).min(newest)
    };
    *prefix = running;
    let complete = position + 1 == window;
    let minimum = if complete {
        running
    } else {
        ring_lane::<L>(required_ring, start).min(running)
    };
    if complete {
        let mut suffix = ring_lane::<L>(required_ring, end);
        let mut slot = end;
        for _ in 0..window {
            suffix = suffix.min(ring_lane::<L>(required_ring, slot));
            store_ring_lane::<L>(required_ring, slot, suffix);
            if slot == 0 {
                slot = ring;
            }
            slot -= 1;
        }
        *phase = 0;
    } else {
        *phase = (position + 1) as u32;
    }
    minimum
}

/// The streaming van Herk / Gil-Werman sliding minimum over each lane's window.
///
/// M. van Herk, *A fast algorithm for local minimum and maximum filters on rectangular and
/// octagonal kernels*, Pattern Recognition Letters 13(7), 1992; J. Gil and M. Werman, *Computing
/// 2-D min, median, and max filters*, IEEE PAMI 15(5), 1993.
///
/// Three compares per element amortised, data-independent control flow, and no memory beyond the
/// ring: the suffix minima of a completed block overwrite the raw values in the required-gain ring,
/// which are never read again. `prefix` accumulates the minimum from the current block's start to
/// the newest window sample; the remainder of the window is the suffix minimum a previous block
/// left at the window's oldest slot. When the block completes, one backward pass of `Wb` writes the
/// suffix minima and the phase restarts.
///
/// Ordering matters twice: this runs **after** the frame's required gain has been written at the
/// cursor (with `Wb == R` the window's newest slot *is* the cursor), and the backward pass runs
/// **after** this frame's minimum has been taken.
#[inline(always)]
fn sliding_minimum(
    state: &mut ChannelState,
    ring: usize,
    cursor: usize,
    out: &mut [f32; MAXIMUM_WIDTH],
) {
    let width = state.width;
    // The window offsets are per lane (lookahead is a per-lane preparation parameter), so this one
    // step is scalar inside an otherwise lane-wide body. Everything else runs in lockstep.
    for (lane, minimum) in out.iter_mut().enumerate().take(width) {
        let shape = state.lane[lane];
        let window = shape.window as usize;
        let mut end = cursor + shape.end_offset as usize;
        if end >= ring {
            end -= ring;
        }
        let mut start = cursor + 1;
        if start >= ring {
            start -= ring;
        }
        let newest = state.required_ring[end * width + lane];
        let position = state.phase[lane] as usize;
        let running = if position == 0 {
            newest
        } else {
            scalar_min(state.prefix[lane], newest)
        };
        state.prefix[lane] = running;
        let complete = position + 1 == window;
        *minimum = if complete {
            running
        } else {
            scalar_min(state.required_ring[start * width + lane], running)
        };
        if complete {
            let mut suffix = state.required_ring[end * width + lane];
            let mut slot = end;
            for _ in 0..window {
                suffix = scalar_min(suffix, state.required_ring[slot * width + lane]);
                state.required_ring[slot * width + lane] = suffix;
                if slot == 0 {
                    slot = ring;
                }
                slot -= 1;
            }
            state.phase[lane] = 0;
        } else {
            state.phase[lane] = (position + 1) as u32;
        }
    }
}

/// The lane-wide state one channel carries across a block.
struct HotChannel<L: Lane> {
    history: History<L>,
    reduction: L,
    box_sum: L,
    window: L,
    limit: RampLanes<L>,
    release: RampLanes<L>,
}

impl<L: Lane> HotChannel<L> {
    /// Loads every lane-wide word of `state` into registers for the block loop.
    #[inline]
    fn load(state: &ChannelState) -> Self {
        let history = History::<L>::load(state);
        let mut window = [0.0_f32; MAXIMUM_WIDTH];
        for (lane, shape) in state.lane.iter().enumerate() {
            window[lane] = shape.window as f32;
        }
        Self {
            history,
            reduction: L::load(&state.reduction),
            box_sum: L::load(&state.box_sum),
            window: L::load(&window),
            limit: RampLanes::gather(&state.limit),
            release: RampLanes::gather(&state.release),
        }
    }

    /// Writes every lane-wide word back into `state` at the end of the block.
    #[inline]
    fn store(self, state: &mut ChannelState) {
        self.history.store(state);
        self.reduction.store(&mut state.reduction);
        self.box_sum.store(&mut state.box_sum);
        self.limit.scatter(&mut state.limit);
        self.release.scatter(&mut state.release);
    }
}

/// Everything one channel does with one frame, after the peak has been linked.
///
/// Frozen operation order; `A` marks a step that is bit-preserving against layout 1 and `B` a step
/// of the new law.
///
/// 1. **required (A)** `r = select(P > limit, limit / P, 1)`. `P > limit >= 10^(-25/20)` implies
///    `P > 0`, so the divide is always defined; it is a divide and not a reciprocal because the
///    unlimited case must be exactly `1.0`.
/// 2. **write (A)** `r` is stored at the write cursor, before the minimum filter reads the window.
/// 3. **minimum (B)** the van Herk window minimum of each lane.
/// 4. **quantise (B)** `m_q = floor(m * 2^14) * 2^-14`; both scalings are exact.
/// 5. **box (B)** `S += m_q - m_q[n-Wb]`, read before write so `Wb == R` reads the slot it is about
///    to overwrite; `s = S / Wb`.
/// 6. **release (B)** `d = max(1 - s, fma(c, (1 - s) - d, d))`, then the D7 flush. This is the only
///    `fma` and the only recursive word in the crate. Working in the reduction domain is what makes
///    the decay terminate at exactly `+0.0`, and therefore `g` at exactly `1.0`.
/// 7. **output (A)** read-before-write on the main ring gives a delay of exactly `B = N + 6`;
///    `y = select(bypass, z, z * g)` keeps the bypass path bit-exact including signed zero.
/// # The uniform-cohort form
///
/// [`channel_frame_uniform`] is the same seven steps for a cohort [`lanes_uniform`] has accepted
/// (#182 S1): steps 3 and 5 run lane-wide there instead of lane by lane. It is a separate function
/// rather than a const parameter on this one so that neither form carries the other's branch —
/// and, since round 2, so that the uniform form can be handed pre-split ring views and pre-resolved
/// slot indices that this one, addressing the arena lane by lane, cannot use. The whole-bank
/// decision is taken once, in [`limiter_block`].
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn channel_frame<L: Lane>(
    io: &mut [f32],
    base: usize,
    x: L,
    peak: L,
    limit: L,
    release: L,
    hot: &mut HotChannel<L>,
    state: &mut ChannelState,
    ring: usize,
    ring_cursor: usize,
    main_cursor: usize,
    bypass: <L as Lane>::Mask,
    scratch: &mut [f32; MAXIMUM_WIDTH],
) {
    let width = state.width;
    let one = L::splat(1.0);

    let required = L::select(peak.gt(limit), limit.div(peak), one);
    required.store(&mut state.required_ring[ring_cursor * width..]);

    sliding_minimum(state, ring, ring_cursor, scratch);
    let minimum = L::load(scratch);
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));

    let expired = {
        for (lane, expiring) in scratch.iter_mut().enumerate().take(width) {
            let mut slot = ring_cursor + state.lane[lane].box_offset as usize;
            if slot >= ring {
                slot -= ring;
            }
            *expiring = state.box_ring[slot * width + lane];
        }
        L::load(scratch)
    };
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    quantised.store(&mut state.box_ring[ring_cursor * width..]);
    let smoothed = hot.box_sum.div(hot.window);

    let target = one.sub(smoothed);
    let released = release.fma(target.sub(hot.reduction), hot.reduction);
    hot.reduction = flush(target.max(released));

    let gain = one.sub(hot.reduction);
    let delayed = L::load(&state.main_ring[main_cursor * width..]);
    x.store(&mut state.main_ring[main_cursor * width..]);
    L::select(bypass, delayed, delayed.mul(gain)).store(&mut io[base..]);
}

/// The three ring views and the two van Herk words one uniform channel carries across a block.
///
/// Round 2 R1 (a) and (c). `ChannelState` is behind a `&mut` that the frame body used to hold for
/// the whole frame, so every read of a ring base pointer, of `width`, or of `lane[0]` had to be
/// re-loaded after each store the compiler could not prove disjoint from it — about two hundred
/// scalar instructions per frame of pure bookkeeping. Taking the three views and the three window
/// offsets **once per block** removes the aliasing question entirely: the views are `&mut [f32]`
/// locals of known length, and the offsets are integers in registers.
///
/// The views are cut to exactly `slots * L::WIDTH` words, which is their whole length. The slice
/// is not a narrowing, it is a *statement*: it gives the block loop's bounds checks a length the
/// compiler can relate to the slot indices, which is what lets them be hoisted to segment entry.
struct UniformHot<'a, L: Lane> {
    /// Running minimum of the current van Herk block, in a register for the whole block.
    prefix: L,
    /// Position inside the current van Herk block, in a register for the whole block.
    phase: u32,
    /// The cohort's one window shape, read once from lane 0.
    offsets: WindowOffsets,
    /// `R * W` words of required gain; the van Herk suffix minima overwrite expired raw values.
    required_ring: &'a mut [f32],
    /// `R * W` words of quantised minima.
    box_ring: &'a mut [f32],
    /// `B * W` words of main delay.
    main_ring: &'a mut [f32],
}

impl<'a, L: Lane> UniformHot<'a, L> {
    /// Splits one channel's arena into the views the block loop holds, and reads the two van Herk
    /// words out of it.
    ///
    /// Lane 0 speaks for the cohort at every one of the three offsets, which is exactly what
    /// [`lanes_uniform`] has just established — the same read `sliding_minimum_uniform` and the
    /// box gather each made for themselves, once per frame, before.
    #[inline]
    fn new(state: &'a mut ChannelState, shape: &Shape) -> Self {
        let ring_words = shape.ring * L::WIDTH;
        let main_words = shape.main * L::WIDTH;
        let lane = state.lane[0];
        Self {
            prefix: L::load(&state.prefix),
            phase: state.phase[0],
            offsets: WindowOffsets::new(lane),
            required_ring: &mut state.required_ring[..ring_words],
            box_ring: &mut state.box_ring[..ring_words],
            main_ring: &mut state.main_ring[..main_words],
        }
    }
}

/// The five ring slots one frame of one uniform channel touches, resolved at segment entry.
///
/// Round 2 R1(d). Inside a wrap-free segment every one of the five advances by exactly one per
/// frame, so the segment resolves them once and the frame loop adds its step index. The values are
/// the same indices the per-frame `+ offset` / `- ring` pairs produced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FrameSlots {
    /// The write cursor of the required-gain and box rings.
    ring_cursor: usize,
    /// The read-before-write cursor of the main delay ring.
    main_cursor: usize,
    /// The window's newest sample, `ring_cursor + Wb` around the ring.
    end: usize,
    /// The window's oldest sample, `ring_cursor + 1` around the ring.
    start: usize,
    /// The box term leaving the running sum, `ring_cursor + (R - Wb)` around the ring.
    expiring: usize,
}

impl FrameSlots {
    /// These slots `step` frames into the segment they begin.
    ///
    /// One addition each, and no compare: that every one of the five stays below its ring's slot
    /// count for the whole run is what [`segment`] computes the run *from*.
    #[inline(always)]
    const fn advanced(self, step: usize) -> Self {
        Self {
            ring_cursor: self.ring_cursor + step,
            main_cursor: self.main_cursor + step,
            end: self.end + step,
            start: self.start + step,
            expiring: self.expiring + step,
        }
    }
}

/// The window shape of a uniform cohort, as ring distances from the write cursor.
///
/// [`LaneShape`]'s three fields as `usize`, read once per block from lane 0 — which
/// [`lanes_uniform`] has established is every lane's.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WindowOffsets {
    /// `Wb`, the window length.
    window: usize,
    /// `Wb`, the ring distance to the window's newest sample.
    end_offset: usize,
    /// `R - Wb`, the ring distance to the box term leaving the running sum.
    box_offset: usize,
}

impl WindowOffsets {
    #[inline]
    const fn new(shape: LaneShape) -> Self {
        Self {
            window: shape.window as usize,
            end_offset: shape.end_offset as usize,
            box_offset: shape.box_offset as usize,
        }
    }
}

/// One wrap-free segment of the frame loop: where each channel's five slots start, and for how
/// many frames all of them advance by one without wrapping.
///
/// Round 2 R1(d).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Segment {
    left: FrameSlots,
    right: FrameSlots,
    run: usize,
}

/// Resolves the segment that begins at `ring_cursor` / `main_cursor`.
///
/// Seven indices advance by one per frame — the two cursors, each channel's window end and box
/// slot, and the window start both channels share — and each wraps at its own point of its ring.
/// The run is the distance to the first of those wraps, capped by `remaining`, so inside a segment
/// the frame loop adds its step index to five integers and does nothing else.
///
/// # Identity
///
/// For an offset `o` and a segment whose entry cursor is `c`, the frame-at-a-time body computes
/// `(c + step + o) mod R` and this form computes `((c + o) mod R) + step`. The two agree exactly
/// while `((c + o) mod R) + step < R`, which is precisely what `run <= R - ((c + o) mod R)` says;
/// the same argument with `B` covers the main cursor.
/// `the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits` is that statement as a test,
/// against a `%` oracle rather than against this function's own conditional subtraction.
///
/// This is **per-block** control flow, not per-sample: a segment's length is a function of the
/// cursors and of the prepared window shape and of nothing the signal does, so the Lane doc's ban
/// on data-dependent branching inside a per-sample loop is untouched.
#[inline(always)]
fn segment(
    shape: &Shape,
    ring_cursor: usize,
    main_cursor: usize,
    remaining: usize,
    left: WindowOffsets,
    right: WindowOffsets,
) -> Segment {
    let ring = shape.ring;
    let main = shape.main;
    let start = wrapped(ring_cursor + 1, ring);
    let left_end = wrapped(ring_cursor + left.end_offset, ring);
    let right_end = wrapped(ring_cursor + right.end_offset, ring);
    let left_expiring = wrapped(ring_cursor + left.box_offset, ring);
    let right_expiring = wrapped(ring_cursor + right.box_offset, ring);
    let run = remaining
        .min(ring - ring_cursor)
        .min(main - main_cursor)
        .min(ring - start)
        .min(ring - left_end)
        .min(ring - right_end)
        .min(ring - left_expiring)
        .min(ring - right_expiring);
    // Every term is at least one -- `remaining` because the caller's loop guard says so, and each
    // `ring - index` / `main - main_cursor` because the index it subtracts is a slot of that ring.
    // A zero would not be a slow segment walk, it would be a frame loop that never advances, so it
    // is asserted rather than assumed.
    debug_assert!(run >= 1);
    Segment {
        left: FrameSlots {
            ring_cursor,
            main_cursor,
            end: left_end,
            start,
            expiring: left_expiring,
        },
        right: FrameSlots {
            ring_cursor,
            main_cursor,
            end: right_end,
            start,
            expiring: right_expiring,
        },
        run,
    }
}

/// [`channel_frame`] for a uniform cohort: the same seven steps, no per-lane scalar work.
///
/// The operation order is [`channel_frame`]'s, step for step and operand for operand. What differs
/// is where the operands live: the rings arrive as views and the slots as integers, so the body is
/// seven lane-wide operations and no bookkeeping.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn channel_frame_uniform<L: Lane>(
    io_frame: &mut [f32],
    x: L,
    peak: L,
    limit: L,
    release: L,
    hot: &mut HotChannel<L>,
    uniform: &mut UniformHot<'_, L>,
    ring: usize,
    slots: FrameSlots,
    bypass: <L as Lane>::Mask,
) {
    let one = L::splat(1.0);

    let required = L::select(peak.gt(limit), limit.div(peak), one);
    store_ring_lane::<L>(uniform.required_ring, slots.ring_cursor, required);

    let minimum = sliding_minimum_uniform::<L>(
        uniform.required_ring,
        uniform.offsets.window,
        ring,
        slots.end,
        slots.start,
        &mut uniform.prefix,
        &mut uniform.phase,
    );
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));

    // One slot for the whole bank, so the `W` expiring terms are one contiguous vector load.
    let expired = ring_lane::<L>(uniform.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(uniform.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);

    let target = one.sub(smoothed);
    let released = release.fma(target.sub(hot.reduction), hot.reduction);
    hot.reduction = flush(target.max(released));

    let gain = one.sub(hot.reduction);
    let delayed = ring_lane::<L>(uniform.main_ring, slots.main_cursor);
    store_ring_lane::<L>(uniform.main_ring, slots.main_cursor, x);
    L::select(bypass, delayed, delayed.mul(gain)).store(io_frame);
}

/// The one block kernel: `frames` frames of `L::WIDTH` tracks, both channels, one pass.
///
/// Decision D10. The frame loop lives here and nothing per-sample crosses a call boundary: the
/// twelve-word history, the reduction word, the box sum and both coefficient ramps stay in
/// registers for the whole block, and the ring cursors are one pair for the whole bank.
///
/// # Panics
///
/// Panics in debug builds if either block is not `frames * L::WIDTH` long, or if the arena was not
/// allocated for `L::WIDTH` lanes. Block shapes are validated once at preparation (#90 F8).
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn limiter_block<L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
) {
    // Issue #182 S1: one whole-bank branch, taken here and nowhere else. Both channels must be
    // uniform, because both run the same body; a bank with a mixed left channel and a uniform
    // right one takes the per-lane path on both, which is the conservative direction and keeps the
    // decision one branch rather than two.
    if lanes_uniform(left) && lanes_uniform(right) {
        limiter_block_uniform::<L>(left_io, right_io, frames, coef, shape, left, right, cursors);
    } else {
        limiter_block_per_lane::<L>(left_io, right_io, frames, coef, shape, left, right, cursors);
    }
}

/// One channel's detector pass over one chunk: `span` peaks from `span` input frames.
///
/// The twelve history words live in locals for the whole chunk and are written back to `taps` once,
/// which is the reason the block is walked in chunks at all (see [`limiter_block_per_lane`]).
/// Shared by both block bodies verbatim: the detector is the same computation whether or not the
/// cohort is uniform, and its operation order is frozen.
#[inline(always)]
fn detector_chunk<L: Lane>(
    taps: &mut History<L>,
    io: &[f32],
    chunk: usize,
    span: usize,
    fir: &[[L; 4]; HISTORY_WORDS],
    peaks: &mut [f32; DETECTOR_CHUNK * MAXIMUM_WIDTH],
) {
    let width = L::WIDTH;
    let mut history = *taps;
    for frame in 0..span {
        let base = (chunk + frame) * width;
        let x = L::load(&io[base..]);
        detector_peak(&mut history, x, fir).store(&mut peaks[frame * width..]);
    }
    *taps = history;
}

/// The body of [`limiter_block`] for a cohort whose lanes are **not** uniform.
///
/// The fallback of #182 S1, unchanged: every ring is addressed lane by lane because `LaneShape` is
/// a per-lane preparation parameter, and one track's arithmetic must not depend on which lane of
/// which cohort it landed in. Round 2 left this body's arithmetic and loop structure exactly as
/// they were; the only edit is that the detector pass it shares with the uniform body now lives in
/// [`detector_chunk`] and the peak scratch is two named arrays instead of one indexed pair.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn limiter_block_per_lane<L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(right.width, width);
    debug_assert_eq!(left_io.len(), frames * width);
    debug_assert_eq!(right_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let mut hot_right = HotChannel::<L>::load(right);
    // Issue #144 item 6, the stationary hoist, taken once per block at whole-bank granularity.
    //
    // Per-lane branching is forbidden here: one track's arithmetic must not depend on which
    // cohort it landed in. So the block takes the hoist only when *every* lane of *all four*
    // ramps is stationary, exactly as the compressor's `max_remaining` split and the matrix
    // stage's `if maximum == 0` already do.
    //
    // The test is a bit compare, never a tolerance: `remaining == 0` says no window is open, and
    // `current.to_bits() == target.to_bits()` says the value in force is exactly the value being
    // held. The scalar ramps are read rather than the gathered lanes because this is
    // control-plane bookkeeping done once, and because lanes at or above `width` are inert.
    let stationary = ramps_are_stationary(&left.limit)
        && ramps_are_stationary(&left.release)
        && ramps_are_stationary(&right.limit)
        && ramps_are_stationary(&right.release);
    let all = L::zero().eq(L::zero());
    let none = L::mask_not(all);
    let link = if coef.link_max { all } else { none };
    let bypass = if coef.bypass { all } else { none };
    let mut main_cursor = cursors.main as usize;
    let mut ring_cursor = cursors.ring as usize;
    let mut scratch = [0.0_f32; MAXIMUM_WIDTH];
    let mut peaks_left = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];
    let mut peaks_right = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];

    // The block is walked in chunks so that only one channel's twelve history words are live at a
    // time. Both channels' histories together are twenty-four vector registers, which is more than
    // any of the three backends has; splitting the detector into two passes over a short chunk
    // costs twelve loads and twelve stores per chunk and removes the spill from the inner loop.
    // Nothing about the per-lane operation order changes, so the block is bit-identical to the
    // single-pass form (the E12 digests are the proof).
    for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
        let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
        detector_chunk::<L>(
            &mut hot_left.history,
            left_io,
            chunk,
            span,
            &coef.fir,
            &mut peaks_left,
        );
        detector_chunk::<L>(
            &mut hot_right.history,
            right_io,
            chunk,
            span,
            &coef.fir,
            &mut peaks_right,
        );

        for frame in 0..span {
            let base = (chunk + frame) * width;
            let (limit_left, release_left, limit_right, release_right) = if stationary {
                (
                    hot_left.limit.resting_value(),
                    hot_left.release.resting_value(),
                    hot_right.limit.resting_value(),
                    hot_right.release.resting_value(),
                )
            } else {
                (
                    hot_left.limit.advance(),
                    hot_left.release.advance(),
                    hot_right.limit.advance(),
                    hot_right.release.advance(),
                )
            };

            let peak_left = L::load(&peaks_left[frame * width..]);
            let peak_right = L::load(&peaks_right[frame * width..]);
            let linked = peak_right.max(peak_left);
            let peak_left = L::select(link, linked, peak_left);
            let peak_right = L::select(link, linked, peak_right);

            channel_frame::<L>(
                left_io,
                base,
                L::load(&left_io[base..]),
                peak_left,
                limit_left,
                release_left,
                &mut hot_left,
                left,
                shape.ring,
                ring_cursor,
                main_cursor,
                bypass,
                &mut scratch,
            );
            channel_frame::<L>(
                right_io,
                base,
                L::load(&right_io[base..]),
                peak_right,
                limit_right,
                release_right,
                &mut hot_right,
                right,
                shape.ring,
                ring_cursor,
                main_cursor,
                bypass,
                &mut scratch,
            );

            main_cursor += 1;
            if main_cursor == shape.main {
                main_cursor = 0;
            }
            ring_cursor += 1;
            if ring_cursor == shape.ring {
                ring_cursor = 0;
            }
        }
    }

    hot_left.store(left);
    hot_right.store(right);
    cursors.main = main_cursor as u32;
    cursors.ring = ring_cursor as u32;
}

/// The body of [`limiter_block`] for a cohort [`lanes_uniform`] has accepted.
///
/// Every frame is the same seven lane-wide steps [`channel_frame_uniform`] lists, in the same
/// order, on the same words. What round 2 changed is the bookkeeping around them.
///
/// # The segment walk (R1 d)
///
/// The frame loop is split into **wrap-free segments**. Seven indices advance by one per frame —
/// the two cursors, and each channel's window end, box slot and the shared window start — and each
/// wraps at its own point of the ring. A segment runs until the first of them would wrap, so
/// inside a segment every one of the seven is `base + step` with no compare, no conditional
/// subtract, and a slot index the compiler can relate to the ring view's length. At the launch
/// rates this crate supports the rings are hundreds of slots and a block is at most a few hundred
/// frames, so each index wraps at most once in a block and the walk costs a handful of segment
/// entries — the console's 128-frame quantum against `R = 481` and `B = 486` takes at most six.
///
/// This is **per-block** control flow, not per-sample: the segment lengths are functions of the
/// cursors and the prepared window shape and of nothing the signal does, so the Lane doc's ban on
/// data-dependent branching inside a per-sample loop is untouched. Two cohorts with the same
/// cursors and the same shape take the same segments whatever they are rendering.
///
/// # Identity
///
/// Every index this produces is the index the frame-by-frame form produced. For an offset `o` and
/// a segment whose entry cursor is `c`, the frame-by-frame form computes `(c + step + o) mod R`
/// and this form computes `((c + o) mod R) + step`; the two agree exactly while
/// `((c + o) mod R) + step < R`, which is the condition the segment length is the minimum of. The
/// state words the frame loop keeps in registers are argued in [`sliding_minimum_uniform`].
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn limiter_block_uniform<L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(right.width, width);
    debug_assert_eq!(left_io.len(), frames * width);
    debug_assert_eq!(right_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let mut hot_right = HotChannel::<L>::load(right);
    // Issue #144 item 6, the stationary hoist, taken once per block at whole-bank granularity, on
    // the terms `limiter_block_per_lane` states.
    let stationary = ramps_are_stationary(&left.limit)
        && ramps_are_stationary(&left.release)
        && ramps_are_stationary(&right.limit)
        && ramps_are_stationary(&right.release);
    let all = L::zero().eq(L::zero());
    let none = L::mask_not(all);
    let link = if coef.link_max { all } else { none };
    let bypass = if coef.bypass { all } else { none };
    let ring = shape.ring;
    let main = shape.main;
    let mut main_cursor = cursors.main as usize;
    let mut ring_cursor = cursors.ring as usize;
    let mut peaks_left = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];
    let mut peaks_right = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];

    // The ring views borrow the two channels for the whole walk, so the two van Herk words come
    // back out of the scope and are written to the arena below, once.
    let (left_prefix, left_phase, right_prefix, right_phase) = {
        let mut uniform_left = UniformHot::<L>::new(left, shape);
        let mut uniform_right = UniformHot::<L>::new(right, shape);

        // The chunking of the detector is `limiter_block_per_lane`'s, for its reason: only one
        // channel's twelve history words are live at a time.
        for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
            let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
            detector_chunk::<L>(
                &mut hot_left.history,
                left_io,
                chunk,
                span,
                &coef.fir,
                &mut peaks_left,
            );
            detector_chunk::<L>(
                &mut hot_right.history,
                right_io,
                chunk,
                span,
                &coef.fir,
                &mut peaks_right,
            );

            let mut frame = 0;
            while frame < span {
                let walk = segment(
                    shape,
                    ring_cursor,
                    main_cursor,
                    span - frame,
                    uniform_left.offsets,
                    uniform_right.offsets,
                );
                let run = walk.run;

                let base = (chunk + frame) * width;
                let words = run * width;
                let left_segment = &mut left_io[base..base + words];
                let right_segment = &mut right_io[base..base + words];
                let left_peaks = &peaks_left[frame * width..(frame + run) * width];
                let right_peaks = &peaks_right[frame * width..(frame + run) * width];

                for (step, (((left_frame, right_frame), left_peak), right_peak)) in left_segment
                    .chunks_exact_mut(width)
                    .zip(right_segment.chunks_exact_mut(width))
                    .zip(left_peaks.chunks_exact(width))
                    .zip(right_peaks.chunks_exact(width))
                    .enumerate()
                {
                    let (limit_left, release_left, limit_right, release_right) = if stationary {
                        (
                            hot_left.limit.resting_value(),
                            hot_left.release.resting_value(),
                            hot_right.limit.resting_value(),
                            hot_right.release.resting_value(),
                        )
                    } else {
                        (
                            hot_left.limit.advance(),
                            hot_left.release.advance(),
                            hot_right.limit.advance(),
                            hot_right.release.advance(),
                        )
                    };

                    let peak_left = L::load(left_peak);
                    let peak_right = L::load(right_peak);
                    let linked = peak_right.max(peak_left);
                    let peak_left = L::select(link, linked, peak_left);
                    let peak_right = L::select(link, linked, peak_right);

                    let x_left = L::load(left_frame);
                    let x_right = L::load(right_frame);

                    channel_frame_uniform::<L>(
                        left_frame,
                        x_left,
                        peak_left,
                        limit_left,
                        release_left,
                        &mut hot_left,
                        &mut uniform_left,
                        ring,
                        walk.left.advanced(step),
                        bypass,
                    );
                    channel_frame_uniform::<L>(
                        right_frame,
                        x_right,
                        peak_right,
                        limit_right,
                        release_right,
                        &mut hot_right,
                        &mut uniform_right,
                        ring,
                        walk.right.advanced(step),
                        bypass,
                    );
                }

                frame += run;
                ring_cursor = wrapped(ring_cursor + run, ring);
                main_cursor = wrapped(main_cursor + run, main);
            }
        }

        (
            uniform_left.prefix,
            uniform_left.phase,
            uniform_right.prefix,
            uniform_right.phase,
        )
    };

    // R1(a)'s write-back. One store of each van Herk word per block, holding what the last frame
    // of the block computed; `phase` is filled across the cohort because every lane of it shares
    // the one position `lanes_uniform` established.
    left_prefix.store(&mut left.prefix);
    left.phase.fill(left_phase);
    right_prefix.store(&mut right.prefix);
    right.phase.fill(right_phase);

    hot_left.store(left);
    hot_right.store(right);
    cursors.main = main_cursor as u32;
    cursors.ring = ring_cursor as u32;
}

/// The parameter domains, in descriptor order, for the runtime validator.
///
/// The same three rows as [`TRUE_PEAK_LIMITER_PARAMETERS_V1`]; the descriptor keeps the identity,
/// unit, automation rate and smoothing rule, and `miso-engine-effect-runtime` owns the validation
/// so this crate no longer carries its own copy of `parameter_value_valid` (#90 F9).
const PARAMETER_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    ParameterSpec::continuous(-24.0, 0.0, -1.0),
    ParameterSpec::logarithmic(10.0, 2000.0, 100.0),
    ParameterSpec::continuous(0.0, 10.0, 5.0),
];

/// Everything one prepared instance or bank owns, at one width.
struct LimiterCore<L: Lane> {
    metadata: PreparedEffectMetadata,
    shape: Shape,
    coefficients: LimiterCoef<L>,
    left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    left: ChannelState,
    right: ChannelState,
    cursors: Cursors,
    report: NonFiniteReport,
    /// Issue #182 S2: the previous block proved this instance is at a silent fixed point.
    ///
    /// Earned only by observation in [`process_block`](Self::process_block), never assumed. The
    /// design is the compressor's `silent_fixed_point` (#163 phase 4 item 1) at a kernel whose rest
    /// state is not all zeros: two of this crate's three rings rest at exactly `1.0`, not `+0.0`,
    /// and the argument transfers because what it needs is that the rings rest **uniform**, so that
    /// a read from any cursor position returns the value a slow path would have read.
    silent_fixed_point: bool,
    /// The bypass flag in force when the claim above was earned. Bypass selects a different arm of
    /// the output `select`, so a claim earned on one side of it says nothing about the other.
    silent_bypass: bool,
    /// Blocks the fast path actually took, for the engagement-rate gate. Test-only, like
    /// [`nonfinite_report`](Self::nonfinite_report): instrumentation is not render state.
    #[cfg(test)]
    silent_engagements: u32,
}

impl<L: Lane> LimiterCore<L> {
    /// Allocates one core of `L::WIDTH` tracks. The only allocating function in the render crate.
    fn new(
        metadata: PreparedEffectMetadata,
        left_defaults: Box<[[f32; PARAMETER_COUNT]]>,
        right_defaults: Box<[[f32; PARAMETER_COUNT]]>,
    ) -> Option<Self> {
        let width = L::WIDTH;
        if left_defaults.len() != width || right_defaults.len() != width || width > MAXIMUM_WIDTH {
            return None;
        }
        let shape = Shape::new(metadata.sample_rate)?;
        let rate = metadata.sample_rate;
        Some(Self {
            coefficients: LimiterCoef::new(
                matches!(metadata.link_mode, LinkMode::Maximum),
                metadata.bypass,
            ),
            left: ChannelState::new(width, &shape, &left_defaults, rate),
            right: ChannelState::new(width, &shape, &right_defaults, rate),
            cursors: Cursors::default(),
            report: NonFiniteReport::new(),
            silent_fixed_point: false,
            silent_bypass: metadata.bypass,
            #[cfg(test)]
            silent_engagements: 0,
            metadata,
            shape,
            left_defaults,
            right_defaults,
        })
    }

    /// The two resets, one implementation (#90 F9).
    fn reset(&mut self, kind: ResetKind) {
        // #182 S2: a reset rewrites every ring, the recursive word and the cursors, so the claim
        // goes. It is withdrawn rather than re-earned here even though `clear_runtime` leaves
        // precisely the rest state the claim describes, because the claim is a statement about a
        // block that was *rendered and observed*, and a reset renders nothing.
        self.silent_fixed_point = false;
        let rate = self.metadata.sample_rate;
        match kind {
            ResetKind::FullToDefaults => {
                self.left
                    .reset_to_defaults(&self.shape, &self.left_defaults, rate);
                self.right
                    .reset_to_defaults(&self.shape, &self.right_defaults, rate);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.reset_keeping_parameters(&self.shape);
                self.right.reset_keeping_parameters(&self.shape);
            }
        }
        self.cursors = Cursors::default();
    }

    /// Runs one block and applies the master plan §4.4 boundary check (decision D7).
    ///
    /// A block whose output is NaN or at least `1e30` in magnitude is zeroed on both channels, the
    /// whole instance is reset to its defaults and the bank's counter is incremented. There is no
    /// per-lane recovery and no per-value check anywhere on this path: a signal that leaves the
    /// representable range is a bug report, not a signal-processing feature.
    fn process_block(&mut self, left_io: &mut [f32], right_io: &mut [f32], frames: usize) {
        let words = frames * L::WIDTH;
        // Issue #182 S2, the phase-4 admission test. Whole-bank, never per lane, and every leg is
        // cheap next to the block it can replace:
        //
        // * no ramp of either channel has a window open and each holds exactly its target, so the
        //   coefficient words in force are the ones the observed block used;
        // * the bypass flag is the one that was in force when the claim was earned, since bypass
        //   selects the other arm of this kernel's output `select`;
        // * both input planes are exactly `+0.0`, which short-circuits on the first thirty-two
        //   words for a block carrying signal.
        //
        // Strict `+0.0`, never `== 0.0`. This crate is the one where the distinction is audible
        // rather than academic: a `-0.0` input sample is written into `main_ring` and emerges `B =
        // N + 6` samples later, and `select(bypass, delayed, delayed * gain)` preserves its sign on
        // both arms (`-0.0 * 1.0` is `-0.0`). A fast path that counted `-0.0` as silence would
        // never write it into the line, and the sample that should have come out of the line four
        // blocks later would be `+0.0` instead. That is the compressor's input-side argument
        // (#163 phase 4, adversarial pass) at a kernel that also has a delay line to carry it.
        let quiet = self.silent_bypass == self.metadata.bypass
            && ramps_are_stationary(&self.left.limit)
            && ramps_are_stationary(&self.left.release)
            && ramps_are_stationary(&self.right.limit)
            && ramps_are_stationary(&self.right.release)
            && block_is_positive_zero(&left_io[..words])
            && block_is_positive_zero(&right_io[..words]);
        if quiet && self.silent_fixed_point {
            // Every ring is known uniform — `main_ring` all `+0.0`, `required_ring` and `box_ring`
            // all exactly `1.0` — the recursive word is at its fixed point, and the buffers already
            // hold the `+0.0` the kernel would have written over them. What the frame loop would
            // have done to the arena is the identity at every step: it writes `r = 1.0` over a
            // `1.0`, takes a window minimum of `1.0`s, quantises `1.0` to `1.0`, adds and subtracts
            // the same `1.0` from a box sum that is exactly `Wb` (and is therefore exact, being a
            // multiple of `2^-14` below `2^24`), lands on `d = flush(max(0, fma(c, 0, 0))) = +0.0`,
            // and stores the `+0.0` it just read out of the main ring.
            //
            // Only the cursors and the van Herk phase actually move, so only they are advanced.
            // As in the compressor's cursor note, this makes the skipped block leave the instance
            // **bit-identical** to the block that ran, rather than merely observationally
            // equivalent to it: the weaker invariant would have to be re-proved every time the ring
            // handling changed, and `snapshot_track` would expose the difference immediately.
            self.left.advance_rest_phase(frames);
            self.right.advance_rest_phase(frames);
            self.cursors.advance(frames, &self.shape);
            #[cfg(test)]
            {
                self.silent_engagements = self.silent_engagements.saturating_add(1);
            }
            return;
        }
        limiter_block::<L>(
            left_io,
            right_io,
            frames,
            &self.coefficients,
            &self.shape,
            &mut self.left,
            &mut self.right,
            &mut self.cursors,
        );
        // Earn or lose the claim from what this block actually did. `is_at_silent_rest` is
        // `clear_runtime`'s own word list read back, which is the state the crate documents a
        // silent lane rests in; the output test is what says the *caller* saw silence too.
        //
        // The rest state is exactly reachable here, which is the precondition #163 phase 4 states
        // for engaging at all. Every box term is an integer multiple of `2^-14` and `BOX_GRID * R`
        // is below `2^24` at every launch rate, so the running sum arrives at exactly `Wb` rather
        // than near it; and the D7 flush terminates the release at exactly `+0.0` rather than
        // asymptotically, which is the difference between this kernel and the compressor's
        // `gain_reduction_db`. Without both, the claim would be refused forever and the fast path
        // would be dead code.
        self.silent_fixed_point = quiet
            && self.left.is_at_silent_rest()
            && self.right.is_at_silent_rest()
            && block_is_positive_zero(&left_io[..words])
            && block_is_positive_zero(&right_io[..words]);
        self.silent_bypass = self.metadata.bypass;
        let shape = self.shape;
        let rate = self.metadata.sample_rate;
        let left = &mut self.left;
        let right = &mut self.right;
        let left_defaults = &self.left_defaults;
        let right_defaults = &self.right_defaults;
        let cursors = &mut self.cursors;
        finish_block::<L>(left_io, right_io, &mut self.report, || {
            left.reset_to_defaults(&shape, left_defaults, rate);
            right.reset_to_defaults(&shape, right_defaults, rate);
            *cursors = Cursors::default();
        });
    }

    /// [`process_block`](Self::process_block) over one plane: the collapsed track's live channel.
    ///
    /// Every leg of the `quiet` admission reads the left channel, which on a collapse-eligible
    /// cohort is the same predicate the dual body evaluates -- see the module note above
    /// [`limiter_block_mono`] for why the two channels' ramps and window shapes agree.
    ///
    /// The §4.4 boundary check scans the one live plane. Its lane mask is the dual check's
    /// `mask(left) | mask(right)` with `right` equal to `left`, so it is the same mask; the reset
    /// it triggers still restores **both** channels and the cursors, exactly as the dual one does.
    fn process_block_mono(&mut self, left_io: &mut [f32], frames: usize) {
        let words = frames * L::WIDTH;
        let quiet = self.silent_bypass == self.metadata.bypass
            && ramps_are_stationary(&self.left.limit)
            && ramps_are_stationary(&self.left.release)
            && block_is_positive_zero(&left_io[..words]);
        if quiet && self.silent_fixed_point {
            self.left.advance_rest_phase(frames);
            self.cursors.advance(frames, &self.shape);
            #[cfg(test)]
            {
                self.silent_engagements = self.silent_engagements.saturating_add(1);
            }
            return;
        }
        limiter_block_mono::<L>(
            left_io,
            frames,
            &self.coefficients,
            &self.shape,
            &mut self.left,
            &mut self.cursors,
        );
        self.silent_fixed_point =
            quiet && self.left.is_at_silent_rest() && block_is_positive_zero(&left_io[..words]);
        self.silent_bypass = self.metadata.bypass;
        if check_block::<L>(left_io) {
            return;
        }
        self.report.nonfinite_lanes = nonfinite_lane_mask::<L>(left_io);
        self.report.nonfinite_blocks = self.report.nonfinite_blocks.saturating_add(1);
        left_io.fill(0.0);
        let shape = self.shape;
        let rate = self.metadata.sample_rate;
        self.left
            .reset_to_defaults(&shape, &self.left_defaults, rate);
        self.right
            .reset_to_defaults(&shape, &self.right_defaults, rate);
        self.cursors = Cursors::default();
    }

    /// Copies the left channel's whole state onto the right (the collapse's disengage boundary).
    ///
    /// See [`ChannelState::copy_state_from`] for the word-by-word list and why it is exhaustive.
    fn desymmetrize(&mut self) {
        self.right.copy_state_from(&self.left);
    }

    /// The boundary-check record, for the gates. Wiring it into `ProcessReport` belongs to #95.
    #[cfg(test)]
    const fn nonfinite_report(&self) -> NonFiniteReport {
        self.report
    }

    /// Blocks the #182 S2 fast path took since preparation, for the engagement-rate gate.
    #[cfg(test)]
    const fn silent_engagements(&self) -> u32 {
        self.silent_engagements
    }
}

/// Applies the accepted automation of one track to its two channels.
///
/// The span validation is unchanged from layout 1 — canonical `Point` spans at `first_sample`, for
/// descriptor positions 0 and 1, on an explicit `Left` or `Right` channel, in strictly ascending
/// `(parameter, channel)` order, inside the prepared capacity, with no duplicate — because it is
/// the contract, not an implementation detail. What changed is what an accepted value does: it
/// retargets a ramp of the **linear** coefficient (`limit`, or the release rate), with the single
/// D11 division performed here, at event time, and never per sample.
fn apply_automation(
    spans: &[PreparedAutomationSpan],
    metadata: &PreparedEffectMetadata,
    first_sample: u64,
    left: &mut ChannelState,
    right: &mut ChannelState,
    lane: usize,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; RAMP_COUNT]; 2];
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let channel = match span.channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let parameter = span.parameter_index as usize;
        let Some(order) = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(channel as u32))
        else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < metadata.automation_capacity as usize
            && parameter < RAMP_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && parameter_value_valid(&PARAMETER_SPECS[parameter], span.start_value)
            && !is_negative_zero(span.start_value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[channel][parameter].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[channel][parameter] = Some(normalize_zero(span.start_value));
    }
    let rate = metadata.sample_rate;
    for (channel, state) in [left, right].into_iter().enumerate() {
        if let Some(value) = pending[channel][0] {
            state.limit[lane].set_target(limit_coefficient(value), RAMP_UPDATES);
        }
        if let Some(value) = pending[channel][1] {
            state.release[lane].set_target(release_coefficient(value, rate), RAMP_UPDATES);
        }
    }
}

/// Reads the ordered six-value initial table into per-channel defaults.
fn initial_defaults(
    values: &[InitialParameterValue],
) -> Result<([f32; PARAMETER_COUNT], [f32; PARAMETER_COUNT]), EffectPrepareError> {
    if values.len() != PARAMETER_COUNT * 2 {
        return Err(EffectPrepareError {
            code: "effect.parameter.initial",
        });
    }
    let mut left = [0.0; PARAMETER_COUNT];
    let mut right = [0.0; PARAMETER_COUNT];
    for (index, spec) in PARAMETER_SPECS.iter().enumerate() {
        let left_value = values[index * 2];
        let right_value = values[index * 2 + 1];
        if left_value.parameter_index != index as u32
            || right_value.parameter_index != index as u32
            || left_value.channel != ParameterChannel::Left
            || right_value.channel != ParameterChannel::Right
            || !parameter_value_valid(spec, left_value.value)
            || !parameter_value_valid(spec, right_value.value)
            || is_negative_zero(left_value.value)
            || is_negative_zero(right_value.value)
        {
            return Err(EffectPrepareError {
                code: "effect.parameter.initial",
            });
        }
        left[index] = normalize_zero(left_value.value);
        right[index] = normalize_zero(right_value.value);
    }
    Ok((left, right))
}

/// Word offsets inside one channel section of a state-layout-2 payload.
mod words {
    /// Bank main-delay cursor, written into every lane.
    pub(super) const MAIN_CURSOR: usize = 0;
    /// Bank gain-ring cursor, written into every lane.
    pub(super) const RING_CURSOR: usize = 1;
    /// Prepared lookahead in milliseconds.
    pub(super) const LOOKAHEAD: usize = 2;
    /// The recursive reduction word `d`.
    pub(super) const REDUCTION: usize = 3;
    /// Position inside the current van Herk block.
    pub(super) const PHASE: usize = 4;
    /// Running minimum of the current van Herk block.
    pub(super) const PREFIX: usize = 5;
    /// Exact running box sum.
    pub(super) const BOX_SUM: usize = 6;
    /// Limit ramp: current, target, step, remaining.
    pub(super) const LIMIT_RAMP: usize = 7;
    /// Release-coefficient ramp: current, target, step, remaining.
    pub(super) const RELEASE_RAMP: usize = 11;
    /// Detector history, newest first.
    pub(super) const HISTORY: usize = 15;
}

/// A parsed, not yet committed channel section.
#[derive(Debug)]
struct LaneRestore {
    main_cursor: u32,
    ring_cursor: u32,
    lookahead_ms: f32,
    lane: LaneShape,
    reduction: f32,
    phase: u32,
    prefix: f32,
    box_sum: f32,
    limit: LinearRamp,
    release: LinearRamp,
    history: Box<[f32]>,
    main_ring: Box<[f32]>,
    required_ring: Box<[f32]>,
    box_ring: Box<[f32]>,
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

/// The `[minimum, maximum]` a stored coefficient may occupy, with a four-ulp relaxation.
///
/// A ramped `current` lies mathematically between two in-domain coefficients, but the iterated
/// `current + step` of D11 can round a hair past an endpoint on its last step before the snap. The
/// relaxation is exactly that rounding budget; it is not a domain widening, and a value outside a
/// coefficient's real range by more than a few ulps is still rejected.
fn coefficient_bounds(low: f32, high: f32) -> (f32, f32) {
    let slack = 4.0 * f32::EPSILON;
    (low - low.abs() * slack, high + high.abs() * slack)
}

/// Writes one channel of one track into `bytes`, physical ring order.
fn snapshot_lane(
    bytes: &mut [u8],
    state: &ChannelState,
    lane: usize,
    cursors: Cursors,
    shape: &Shape,
) {
    let width = state.width;
    write_u32(bytes, words::MAIN_CURSOR, cursors.main);
    write_u32(bytes, words::RING_CURSOR, cursors.ring);
    write_f32(bytes, words::LOOKAHEAD, state.lookahead_ms[lane]);
    write_f32(bytes, words::REDUCTION, state.reduction[lane]);
    write_u32(bytes, words::PHASE, state.phase[lane]);
    write_f32(bytes, words::PREFIX, state.prefix[lane]);
    write_f32(bytes, words::BOX_SUM, state.box_sum[lane]);
    for (index, ramp) in [state.limit[lane], state.release[lane]]
        .into_iter()
        .enumerate()
    {
        let word = if index == 0 {
            words::LIMIT_RAMP
        } else {
            words::RELEASE_RAMP
        };
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_f32(bytes, word + 2, ramp.step);
        write_u32(bytes, word + 3, ramp.remaining);
    }
    for tap in 0..HISTORY_WORDS {
        write_f32(
            bytes,
            words::HISTORY + tap,
            state.history[tap * width + lane],
        );
    }
    let mut word = LANE_HEADER_WORDS;
    for slot in 0..shape.main {
        write_f32(bytes, word, state.main_ring[slot * width + lane]);
        word += 1;
    }
    for slot in 0..shape.ring {
        write_f32(bytes, word, state.required_ring[slot * width + lane]);
        word += 1;
    }
    for slot in 0..shape.ring {
        write_f32(bytes, word, state.box_ring[slot * width + lane]);
        word += 1;
    }
    debug_assert_eq!(word, shape.lane_words());
}

/// Parses and validates one channel section without touching any live state.
fn read_lane(
    bytes: &[u8],
    shape: &Shape,
    sample_rate: u32,
) -> Result<LaneRestore, StatePayloadError> {
    let main_cursor = read_u32(bytes, words::MAIN_CURSOR);
    let ring_cursor = read_u32(bytes, words::RING_CURSOR);
    if main_cursor as usize >= shape.main || ring_cursor as usize >= shape.ring {
        return Err(state_error("effect.state.cursor"));
    }

    let lookahead_ms = read_f32(bytes, words::LOOKAHEAD);
    if is_negative_zero(lookahead_ms) || !parameter_value_valid(&PARAMETER_SPECS[2], lookahead_ms) {
        return Err(state_error("effect.state.parameter"));
    }
    let lane = LaneShape::new(lookahead_samples(lookahead_ms, sample_rate, shape.n), shape);
    let window = lane.window as usize;

    let reduction = read_f32(bytes, words::REDUCTION);
    let phase = read_u32(bytes, words::PHASE);
    let prefix = read_f32(bytes, words::PREFIX);
    let box_sum = read_f32(bytes, words::BOX_SUM);
    if !(0.0..=1.0).contains(&reduction)
        || !(0.0..=1.0).contains(&prefix)
        || phase as usize >= window
    {
        return Err(state_error("effect.state.parameter"));
    }
    if !(0.0..=lane.window as f32).contains(&box_sum)
        || (box_sum * BOX_GRID).floor() != box_sum * BOX_GRID
    {
        return Err(state_error("effect.state.gain"));
    }

    let limit_bounds = coefficient_bounds(limit_coefficient(-24.0), limit_coefficient(0.0));
    let release_bounds = coefficient_bounds(
        release_coefficient(2000.0, sample_rate),
        release_coefficient(10.0, sample_rate),
    );
    let mut ramps = [LinearRamp::fixed(0.0); RAMP_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = if index == 0 {
            words::LIMIT_RAMP
        } else {
            words::RELEASE_RAMP
        };
        let (low, high) = if index == 0 {
            limit_bounds
        } else {
            release_bounds
        };
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let step = read_f32(bytes, word + 2);
        let remaining = read_u32(bytes, word + 3);
        if !(low..=high).contains(&current)
            || !(low..=high).contains(&target)
            || !step.is_finite()
            || remaining > RAMP_UPDATES
            || (remaining == 0 && current.to_bits() != target.to_bits())
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = LinearRamp {
            current,
            target,
            step,
            remaining,
        };
    }

    let mut history = vec![0.0_f32; HISTORY_WORDS].into_boxed_slice();
    for (tap, value) in history.iter_mut().enumerate() {
        *value = read_f32(bytes, words::HISTORY + tap);
        if !value.is_finite() {
            return Err(state_error("effect.state.history"));
        }
    }

    let mut word = LANE_HEADER_WORDS;
    let mut main_ring = vec![0.0_f32; shape.main].into_boxed_slice();
    for value in main_ring.iter_mut() {
        *value = read_f32(bytes, word);
        word += 1;
        if !value.is_finite() {
            return Err(state_error("effect.state.ring"));
        }
    }
    let mut required_ring = vec![0.0_f32; shape.ring].into_boxed_slice();
    for value in required_ring.iter_mut() {
        *value = read_f32(bytes, word);
        word += 1;
        if !(0.0..=1.0).contains(value) {
            return Err(state_error("effect.state.gain"));
        }
    }
    let mut box_ring = vec![0.0_f32; shape.ring].into_boxed_slice();
    for value in box_ring.iter_mut() {
        *value = read_f32(bytes, word);
        word += 1;
        if !(0.0..=1.0).contains(value) || (*value * BOX_GRID).floor() != *value * BOX_GRID {
            return Err(state_error("effect.state.gain"));
        }
    }
    debug_assert_eq!(word, shape.lane_words());

    // The box sum is recomputed from the ring rather than trusted: the two together are the state
    // of one running window, and a payload whose sum does not match its own terms would make the
    // gain law drift silently for the rest of the session.
    let mut recomputed = 0.0_f32;
    for age in 1..=window {
        let slot = (ring_cursor as usize + shape.ring - age) % shape.ring;
        recomputed += box_ring[slot];
    }
    if recomputed.to_bits() != box_sum.to_bits() {
        return Err(state_error("effect.state.gain"));
    }

    Ok(LaneRestore {
        main_cursor,
        ring_cursor,
        lookahead_ms,
        lane,
        reduction,
        phase,
        prefix,
        box_sum,
        limit: ramps[0],
        release: ramps[1],
        history,
        main_ring,
        required_ring,
        box_ring,
    })
}

/// Commits a parsed channel section into one lane of a live arena.
///
/// The payload's cursor is the frame its rings are written in. A bank shares one cursor pair across
/// `W` tracks (#90 F3/F6), so the rings are rotated from the payload's frame into the receiver's
/// while they are copied: logical age `a` of the payload lands at logical age `a` of the receiver.
/// The rotation is the identity whenever the two frames agree, which is the case for a scalar
/// instance restored from a scalar snapshot and for every track of a bank restored from that same
/// bank. Everything else in the section — the phase, the prefix, the sum, the ramps — is expressed
/// relative to the value stream and needs no adjustment.
fn commit_lane(
    state: &mut ChannelState,
    lane: usize,
    parsed: &LaneRestore,
    cursors: Cursors,
    shape: &Shape,
) {
    let width = state.width;
    state.lookahead_ms[lane] = parsed.lookahead_ms;
    state.lane[lane] = parsed.lane;
    state.reduction[lane] = parsed.reduction;
    state.phase[lane] = parsed.phase;
    state.prefix[lane] = parsed.prefix;
    state.box_sum[lane] = parsed.box_sum;
    state.limit[lane] = parsed.limit;
    state.release[lane] = parsed.release;
    for (tap, value) in parsed.history.iter().enumerate() {
        state.history[tap * width + lane] = *value;
    }
    for age in 0..shape.main {
        let source = (parsed.main_cursor as usize + age) % shape.main;
        let destination = (cursors.main as usize + age) % shape.main;
        state.main_ring[destination * width + lane] = parsed.main_ring[source];
    }
    for age in 0..shape.ring {
        let source = (parsed.ring_cursor as usize + age) % shape.ring;
        let destination = (cursors.ring as usize + age) % shape.ring;
        state.required_ring[destination * width + lane] = parsed.required_ring[source];
        state.box_ring[destination * width + lane] = parsed.box_ring[source];
    }
}

impl<L: Lane> LimiterCore<L> {
    /// Writes the payload of one track: the runtime header, then both channel sections.
    fn snapshot_track(
        &self,
        track: usize,
        output: &mut StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let layout = self.shape.layout();
        validate_lengths(
            &layout,
            (output.common.len(), output.left.len(), output.right.len()),
        )
        .map_err(|_| state_error("effect.state.length"))?;
        write_header(&layout, output.common);
        snapshot_lane(output.left, &self.left, track, self.cursors, &self.shape);
        snapshot_lane(output.right, &self.right, track, self.cursors, &self.shape);
        Ok(())
    }

    /// Parses both channel sections of one track, then commits them together or not at all.
    fn restore_track(
        &mut self,
        track: usize,
        state_layout_version: u32,
        input: &StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        // #182 S2: a restore writes rings, the recursive word, the phase and the coefficients of
        // one lane from a payload this instance never rendered, so any standing claim is void.
        // Withdrawn before the version check, so a rejected restore cannot leave a half-trusted
        // claim behind either.
        self.silent_fixed_point = false;
        if state_layout_version != STATE_LAYOUT_VERSION {
            return Err(state_error("effect.state.version"));
        }
        let layout = self.shape.layout();
        validate_lengths(
            &layout,
            (input.common.len(), input.left.len(), input.right.len()),
        )
        .map_err(|_| state_error("effect.state.length"))?;
        read_header(&layout, input.common).map_err(|error| state_error(error.code))?;
        let rate = self.metadata.sample_rate;
        let left = read_lane(input.left, &self.shape, rate)?;
        let right = read_lane(input.right, &self.shape, rate)?;
        commit_lane(&mut self.left, track, &left, self.cursors, &self.shape);
        commit_lane(&mut self.right, track, &right, self.cursors, &self.shape);
        Ok(())
    }
}

fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

/// A prepared scalar limiter instance: the block kernel at `L = f32`, `WIDTH = 1`.
///
/// There is no separate scalar code path any more (#90 F9). A planar block is a `W = 1` AoSoA
/// block, so `process` runs the same `limiter_block` body a W8 bank runs, and lane identity is a
/// property of the code rather than of a fixture.
pub struct PreparedTruePeakLimiter {
    core: LimiterCore<f32>,
}

/// A prepared homogeneous cohort of `L::WIDTH` tracks.
struct PreparedTruePeakLimiterBank<L: Lane> {
    metadata: PreparedBankMetadata,
    core: LimiterCore<L>,
}

impl NativeEffectFactory for TruePeakLimiterFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &TRUE_PEAK_LIMITER_DESCRIPTOR_V1
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let (left, right) = initial_defaults(request.initial_values)?;
        let core = LimiterCore::<f32>::new(
            metadata,
            vec![left].into_boxed_slice(),
            vec![right].into_boxed_slice(),
        )
        .ok_or(EffectPrepareError {
            code: "effect.parameter.initial",
        })?;
        Ok(Box::new(PreparedTruePeakLimiter { core }))
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        request.validate_shape()?;
        let first = request
            .requests
            .first()
            .copied()
            .ok_or(EffectPrepareError {
                code: "effect.bank.requests",
            })?;
        let metadata = expected_prepared_metadata(self.descriptor(), first)?;
        let mut left_defaults = Vec::with_capacity(request.requests.len());
        let mut right_defaults = Vec::with_capacity(request.requests.len());
        let mut same_program = true;
        for member in request.requests.iter().copied() {
            let candidate = expected_prepared_metadata(self.descriptor(), member)?;
            if candidate.program_key() != metadata.program_key() {
                same_program = false;
            }
            let (left, right) = initial_defaults(member.initial_values)?;
            left_defaults.push(left);
            right_defaults.push(right);
        }
        // Issue #95: a cohort whose members do not share one program key is a *cohort* this
        // artifact cannot bank, not a malformed request. It declines with `Ok(None)` and the
        // tracks render as scalar instances, which is the contract's frozen rule for every
        // effect (`NativeEffectFactory::bind_homogeneous_bank`). This crate used to be the one
        // that answered `Err("effect.bank.program")`, which would have cost the user the whole
        // session compile for a planner bug.
        //
        // Decision D4: the backend is a compile-time constant, so "unavailable" means this
        // artifact was built for a narrower width than the cohort asks for. Every member has
        // already been validated in both cases, so the fallback is transactional.
        if !same_program || Backend::current().width() < request.width.lanes() as usize {
            return Ok(None);
        }
        let bank_metadata = PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        };
        let left_defaults = left_defaults.into_boxed_slice();
        let right_defaults = right_defaults.into_boxed_slice();
        let bank: Box<dyn PreparedNativeEffectBank> =
            match request.width {
                BankWidth::Four => Box::new(PreparedTruePeakLimiterBank::<Simd4> {
                    metadata: bank_metadata,
                    core: LimiterCore::<Simd4>::new(metadata, left_defaults, right_defaults)
                        .ok_or(EffectPrepareError {
                            code: "effect.parameter.initial",
                        })?,
                }),
                BankWidth::Eight => Box::new(PreparedTruePeakLimiterBank::<Simd8> {
                    metadata: bank_metadata,
                    core: LimiterCore::<Simd8>::new(metadata, left_defaults, right_defaults)
                        .ok_or(EffectPrepareError {
                            code: "effect.parameter.initial",
                        })?,
                }),
            };
        Ok(Some(bank))
    }
}

/// The `DESIGNED` term of the channel-symmetry witness, over the limiter's own kernel read
/// surface.
///
/// # The word list, and why it is exactly this
///
/// The limiter's per-lane designed words are four `ChannelState` fields, and every one of them is
/// read by a body that runs every block:
///
/// * `lane[l]` (`LaneShape { window, end_offset, box_offset }`) -- the van Herk window geometry.
///   It is leg one of `lanes_uniform`, the gate that chooses the uniform body over the general
///   one, and `UniformHot::new` hoists lane 0 of it. Three integers; `LaneShape` is `Eq`.
/// * `limit[l]` and `release[l]` (`LinearRamp`) -- the two automatable coefficients, all four
///   fields each. `RampLanes::gather` reads `current`, `target`, `step` and `remaining` into
///   registers at the top of every block and `scatter` writes them back.
/// * `lookahead_ms[l]` -- what `lane[l]` was derived from, serialised and never ramped. The frame
///   loop does not read it, but `commit_lane` and `reset_to_defaults` do, so two channels that
///   agreed on the shape and disagreed here would diverge at the next restore or reset.
///
/// Deliberately excluded, each for its own reason:
///
/// * `history`, `main_ring`, `required_ring`, `box_ring`, `reduction`, `prefix`, `box_sum`,
///   `phase` -- running state (the crate's own `clear_runtime` is the authoritative list). `phase`
///   is leg two of `lanes_uniform`, which makes it a *gate* input; it is still running state and
///   still converges by induction rather than by comparison, and a restore that desynchronised it
///   is caught by the `RESTORED` term, not this one.
/// * `left_defaults` / `right_defaults` -- control-plane reset values the kernel never reads; a
///   `FullToDefaults` that made the channels disagree lands in the four words above.
/// * `Cursors` and `LimiterCoef` (the FIR table, `link_max`, `bypass`) -- one per **bank**, shared
///   by both channels, so they cannot be asymmetric. `link_max` in particular is the reason the
///   seam sits where it does: `linked = peak_right.max(peak_left)` on two identical words is
///   `max(p, p) = p` bit-exactly.
impl<L: Lane> LimiterCore<L> {
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        if lane >= self.left.width || lane >= self.right.width {
            return false;
        }
        let ramps_agree = |left: &LinearRamp, right: &LinearRamp| {
            left.current.to_bits() == right.current.to_bits()
                && left.target.to_bits() == right.target.to_bits()
                && left.step.to_bits() == right.step.to_bits()
                && left.remaining == right.remaining
        };
        self.left.lane[lane] == self.right.lane[lane]
            && self.left.lookahead_ms[lane].to_bits() == self.right.lookahead_ms[lane].to_bits()
            && ramps_agree(&self.left.limit[lane], &self.right.limit[lane])
            && ramps_agree(&self.left.release[lane], &self.right.release[lane])
    }
}

impl PreparedNativeEffect for PreparedTruePeakLimiter {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.core.metadata
    }

    fn channel_symmetry(&self) -> bool {
        self.core.designed_channel_symmetry(0)
    }

    /// Issue #143 D2 / R4: the recursive reduction word `d`, linear, read for lane 0.
    ///
    /// A plain indexed read of the planar word the block already wrote -- no release step, no
    /// logarithm, no second recursion. Freshening the state here would make two routes to one
    /// value diverge, which is exactly what E6's red mutation demonstrates.
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        if tap_index != 0 {
            return false;
        }
        out.left = self.core.left.reduction[0];
        out.right = self.core.right.reduction[0];
        true
    }

    fn reset(&mut self, kind: ResetKind) {
        self.core.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        // #182 S2, as in `process_bank`.
        if !block.automation.is_empty() {
            self.core.silent_fixed_point = false;
        }
        let mut report = ProcessReport::default();
        let frames = block.frames();
        apply_automation(
            block.automation,
            &self.core.metadata,
            block.first_sample,
            &mut self.core.left,
            &mut self.core.right,
            0,
            &mut report,
        );
        self.core.process_block(block.left, block.right, frames);
        report
    }

    fn snapshot_state_payload(
        &self,
        mut output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.core.snapshot_track(0, &mut output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.core.restore_track(0, state_layout_version, &input)
    }
}

impl<L: Lane> PreparedNativeEffectBank for PreparedTruePeakLimiterBank<L> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.core.designed_channel_symmetry(lane)
    }

    fn observe_resident_bank(&self, tap_index: u32, out: &mut [ObservationSample]) -> bool {
        let lanes = L::WIDTH;
        if tap_index != 0
            || out.len() != lanes
            || self.core.left.reduction.len() != lanes
            || self.core.right.reduction.len() != lanes
        {
            return false;
        }
        for (lane, sample) in out.iter_mut().enumerate() {
            sample.left = self.core.left.reduction[lane];
            sample.right = self.core.right.reduction[lane];
        }
        true
    }

    fn reset(&mut self, kind: ResetKind) {
        self.core.reset(kind);
    }

    /// Runs the cohort's block.
    ///
    /// The width, quantum and sidechain conditions layout 1 rechecked here are compiler invariants
    /// established by `EffectBankProcessBlock::new` and by bank binding, so they are
    /// `debug_assert!`s (#90 F8). The old guard returned the caller's buffers **untouched and
    /// undelayed**, which silently voided the declared `N + 6` latency for that block; nothing on
    /// this path can return without processing.
    fn supports_mono_collapse(&self) -> bool {
        true
    }

    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<true>(block)
    }

    fn desymmetrize_channels(&mut self) {
        self.core.desymmetrize();
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        mut output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH)?;
        self.core.snapshot_track(track, &mut output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH)?;
        self.core.restore_track(track, state_layout_version, &input)
    }
}

impl<L: Lane> PreparedTruePeakLimiterBank<L> {
    /// The one bank body, dual or collapsed. `MONO` chooses the render body and nothing else.
    ///
    /// A const generic rather than an argument, so the two monomorphise and the dual instantiation
    /// is the code that shipped before the collapse existed. See the parametric EQ's copy of this
    /// method for the measurement that settled it.
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        debug_assert_eq!(block.width, self.metadata.width);
        debug_assert_eq!(block.width.lanes() as usize, L::WIDTH);
        debug_assert!(block.frames <= self.core.metadata.quantum);
        debug_assert!(block.sidechain.is_none());
        // #182 S2: an admitted span retargets a linear coefficient, and one whose smoothing
        // window resolves to zero updates snaps it outright while leaving `remaining` at zero — so
        // the `ramps_are_stationary` leg alone would not notice it. Withdraw the claim whenever
        // this block carries automation at all, valid or not; the next settled silent block earns
        // it back. This is the compressor's rule and it is the same hole at both effects.
        if !block.automation.is_empty() {
            self.core.silent_fixed_point = false;
        }
        let mut report = BankProcessReport::empty(self.metadata.width);
        for track in 0..L::WIDTH {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                &self.core.metadata,
                block.first_sample,
                &mut self.core.left,
                &mut self.core.right,
                track,
                &mut report.reports[track],
            );
        }
        if MONO {
            self.core
                .process_block_mono(block.left, block.frames as usize);
        } else {
            self.core
                .process_block(block.left, block.right, block.frames as usize);
        }
        report
    }
}

// ---------------------------------------------------------------------------------------------
// The mono-collapse one-plane bodies.
//
// Each is the dual body with the right channel's arguments deleted and every remaining line left
// where it was. Three things are restatements rather than deletions:
//
// * the peak **link** is computed on the one plane read twice, in the original operation order:
//   `peak_left.max(peak_left)`. This crate's link is `Maximum` only, and `max(p, p)` is `p` to the
//   bit for every finite `p` -- so this one is provably a no-op, and it is written out rather than
//   folded away so that the collapsed body and the dual body read the same;
// * `stationary` is the four-ramp conjunction with the right channel's two conjuncts dropped. On a
//   collapse-eligible bank the two channels' `limit` and `release` ramps are the same words (a
//   one-channel retarget clears the witness' `LIVE` term), so the left pair's answer *is* the
//   conjunction's;
// * `lanes_uniform` is the whole-bank branch, and it takes the same reading for the same reason:
//   `LaneShape` is derived from `lookahead_ms`, which the `DESIGNED` comparison covers.
//
// The right channel's state is untouched, and is restored by `ChannelState::copy_state_from` at
// the disengage boundary before any dual block runs.
// ---------------------------------------------------------------------------------------------

/// [`limiter_block`] over one plane.
#[inline(always)]
fn limiter_block_mono<L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
) {
    if lanes_uniform(left) {
        limiter_block_uniform_mono::<L>(left_io, frames, coef, shape, left, cursors);
    } else {
        limiter_block_per_lane_mono::<L>(left_io, frames, coef, shape, left, cursors);
    }
}

/// [`limiter_block_per_lane`] over one plane.
#[inline(always)]
fn limiter_block_per_lane_mono<L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(left_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let stationary = ramps_are_stationary(&left.limit) && ramps_are_stationary(&left.release);
    let all = L::zero().eq(L::zero());
    let none = L::mask_not(all);
    let link = if coef.link_max { all } else { none };
    let bypass = if coef.bypass { all } else { none };
    let mut main_cursor = cursors.main as usize;
    let mut ring_cursor = cursors.ring as usize;
    let mut scratch = [0.0_f32; MAXIMUM_WIDTH];
    let mut peaks_left = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];

    for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
        let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
        detector_chunk::<L>(
            &mut hot_left.history,
            left_io,
            chunk,
            span,
            &coef.fir,
            &mut peaks_left,
        );

        for frame in 0..span {
            let base = (chunk + frame) * width;
            let (limit_left, release_left) = if stationary {
                (
                    hot_left.limit.resting_value(),
                    hot_left.release.resting_value(),
                )
            } else {
                (hot_left.limit.advance(), hot_left.release.advance())
            };

            let peak_left = L::load(&peaks_left[frame * width..]);
            let linked = peak_left.max(peak_left);
            let peak_left = L::select(link, linked, peak_left);

            channel_frame::<L>(
                left_io,
                base,
                L::load(&left_io[base..]),
                peak_left,
                limit_left,
                release_left,
                &mut hot_left,
                left,
                shape.ring,
                ring_cursor,
                main_cursor,
                bypass,
                &mut scratch,
            );

            main_cursor += 1;
            if main_cursor == shape.main {
                main_cursor = 0;
            }
            ring_cursor += 1;
            if ring_cursor == shape.ring {
                ring_cursor = 0;
            }
        }
    }

    hot_left.store(left);
    cursors.main = main_cursor as u32;
    cursors.ring = ring_cursor as u32;
}

/// [`limiter_block_uniform`] over one plane.
#[inline(always)]
fn limiter_block_uniform_mono<L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(left_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let stationary = ramps_are_stationary(&left.limit) && ramps_are_stationary(&left.release);
    let all = L::zero().eq(L::zero());
    let none = L::mask_not(all);
    let link = if coef.link_max { all } else { none };
    let bypass = if coef.bypass { all } else { none };
    let ring = shape.ring;
    let main = shape.main;
    let mut main_cursor = cursors.main as usize;
    let mut ring_cursor = cursors.ring as usize;
    let mut peaks_left = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];

    let (left_prefix, left_phase) = {
        let mut uniform_left = UniformHot::<L>::new(left, shape);

        for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
            let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
            detector_chunk::<L>(
                &mut hot_left.history,
                left_io,
                chunk,
                span,
                &coef.fir,
                &mut peaks_left,
            );

            let mut frame = 0;
            while frame < span {
                // The segment walk takes the one live channel's offsets for both of its window
                // arguments: on a collapsed cohort the two channels' `LaneShape`s are the same
                // words, so this is the same minimum the dual walk takes.
                let walk = segment(
                    shape,
                    ring_cursor,
                    main_cursor,
                    span - frame,
                    uniform_left.offsets,
                    uniform_left.offsets,
                );
                let run = walk.run;

                let base = (chunk + frame) * width;
                let words = run * width;
                let left_segment = &mut left_io[base..base + words];
                let left_peaks = &peaks_left[frame * width..(frame + run) * width];

                for (step, (left_frame, left_peak)) in left_segment
                    .chunks_exact_mut(width)
                    .zip(left_peaks.chunks_exact(width))
                    .enumerate()
                {
                    let (limit_left, release_left) = if stationary {
                        (
                            hot_left.limit.resting_value(),
                            hot_left.release.resting_value(),
                        )
                    } else {
                        (hot_left.limit.advance(), hot_left.release.advance())
                    };

                    let peak_left = L::load(left_peak);
                    let linked = peak_left.max(peak_left);
                    let peak_left = L::select(link, linked, peak_left);

                    let x_left = L::load(left_frame);

                    channel_frame_uniform::<L>(
                        left_frame,
                        x_left,
                        peak_left,
                        limit_left,
                        release_left,
                        &mut hot_left,
                        &mut uniform_left,
                        ring,
                        walk.left.advanced(step),
                        bypass,
                    );
                }

                frame += run;
                ring_cursor = wrapped(ring_cursor + run, ring);
                main_cursor = wrapped(main_cursor + run, main);
            }
        }

        (uniform_left.prefix, uniform_left.phase)
    };

    left_prefix.store(&mut left.prefix);
    left.phase.fill(left_phase);

    hot_left.store(left);
    cursors.main = main_cursor as u32;
    cursors.ring = ring_cursor as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_dsp_reference::reference_annex2_phases;
    use miso_engine_effect_contract::{
        PrepareEffectLimits, PreparedPorts, PreparedSidechainPort, validate_descriptor,
    };

    /// Deterministic SplitMix64 noise, so a corpus is a seed and never a file.
    struct Noise(u64);

    impl Noise {
        fn next(&mut self) -> f32 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut mixed = self.0;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            mixed ^= mixed >> 31;
            ((mixed >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0
        }
    }

    fn initial_values() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: TRUE_PEAK_LIMITER_PARAMETERS_V1[index / 2].default_value,
        })
    }

    fn values_with(
        ceiling: f32,
        release: f32,
        lookahead: f32,
    ) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        let mut values = initial_values();
        values[0].value = ceiling;
        values[1].value = ceiling;
        values[2].value = release;
        values[3].value = release;
        values[4].value = lookahead;
        values[5].value = lookahead;
        values
    }

    /// [`values_with`] with a different lookahead on each channel.
    ///
    /// Lookahead is the one parameter of this effect that is per channel *and* changes the shape
    /// of the window rather than a coefficient, so a left/right split is the only way to reach a
    /// cohort whose two channels sit at different van Herk positions while both are internally
    /// uniform. `values[4]` and `values[5]` are parameter 2's Left and Right entries, which is the
    /// order [`initial_defaults`] reads them in.
    fn values_split(
        ceiling: f32,
        release: f32,
        left_lookahead: f32,
        right_lookahead: f32,
    ) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        let mut values = values_with(ceiling, release, left_lookahead);
        values[5].value = right_lookahead;
        values
    }

    fn request_at_rate<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
    ) -> PrepareEffectRequest<'a> {
        request_at(values, sample_rate, 128)
    }

    fn request_at<'a>(
        values: &'a [InitialParameterValue],
        sample_rate: u32,
        quantum: u32,
    ) -> PrepareEffectRequest<'a> {
        let quality = QUALITIES
            .iter()
            .find(|quality| quality.sample_rate == sample_rate)
            .expect("launch rate");
        PrepareEffectRequest {
            sample_rate,
            quantum,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
                maximum_scratch_bytes: 24,
                maximum_automation_spans_per_block: 16,
            },
        }
    }

    fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
        request_at_rate(values, 48_000)
    }

    fn snapshot(effect: &dyn PreparedNativeEffect) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let sizes = effect.metadata().state_sizes;
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (common, left, right)
    }

    fn snapshot_track(
        bank: &dyn PreparedNativeEffectBank,
        track: u32,
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let sizes = bank.metadata().program_key.state_sizes;
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        bank.snapshot_track_state_payload(
            track,
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("snapshot");
        (common, left, right)
    }

    fn render(
        effect: &mut dyn PreparedNativeEffect,
        left: &mut [f32],
        right: &mut [f32],
        block: usize,
    ) -> ProcessReport {
        let quantum = effect.metadata().quantum;
        let mut report = ProcessReport::default();
        for (index, (left, right)) in left
            .chunks_mut(block)
            .zip(right.chunks_mut(block))
            .enumerate()
        {
            let next = effect.process(
                EffectProcessBlock::new(left, right, None, (index * block) as u64, &[], quantum)
                    .expect("block"),
            );
            report.invalid_spans = report.invalid_spans.saturating_add(next.invalid_spans);
        }
        report
    }

    fn bank_for(
        values: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        link_mode: LinkMode,
        width: BankWidth,
        backend: Backend,
    ) -> Box<dyn PreparedNativeEffectBank> {
        let requests: Vec<PrepareEffectRequest<'_>> = values
            .iter()
            .map(|values| {
                let mut request = request(values);
                request.link_mode = link_mode;
                request
            })
            .collect();
        TruePeakLimiterFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("bank binding")
            .expect("bank available")
    }

    fn process_bank(
        bank: &mut dyn PreparedNativeEffectBank,
        left: &mut [f32],
        right: &mut [f32],
        width: BankWidth,
        frames: u32,
        first_sample: u64,
    ) {
        let offsets = vec![0_u32; width.lanes() as usize + 1];
        bank.process_bank(
            EffectBankProcessBlock::new(
                left,
                right,
                None,
                frames,
                width,
                first_sample,
                &[],
                &offsets,
                128,
            )
            .expect("bank block"),
        );
    }

    #[test]
    fn descriptor_metadata_and_exact_resource_rows_are_frozen() {
        validate_descriptor(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1).expect("descriptor");
        assert_eq!(
            TRUE_PEAK_LIMITER_DESCRIPTOR_V1.id.as_str(),
            "miso.true-peak-limiter"
        );
        assert_eq!(
            TRUE_PEAK_LIMITER_DESCRIPTOR_V1.supported_link_modes.bits(),
            3
        );
        assert_eq!(TRUE_PEAK_LIMITER_DESCRIPTOR_V1.state_layout_version, 2);
        // Latency is a contract fixture and does not move; the state rows are the #90 re-pin
        // (3N + 35 lane words, plus the runtime's two-word common header).
        for (quality, expected) in QUALITIES.iter().zip([
            (44_100_u32, 447_u64, 5_432_u32, 10_872_u64),
            (48_000, 486, 5_900, 11_808),
            (88_200, 888, 10_724, 21_456),
            (96_000, 966, 11_660, 23_328),
        ]) {
            let n = expected.0 / 100;
            assert_eq!(quality.sample_rate, expected.0);
            assert_eq!(quality.latency, LatencySamples(u64::from(n) + 6));
            assert_eq!(quality.latency, LatencySamples(expected.1));
            assert_eq!(quality.maximum_state.common_bytes, 8);
            assert_eq!(quality.maximum_state.left_bytes, (3 * n + 35) * 4);
            assert_eq!(quality.maximum_state.left_bytes, expected.2);
            assert_eq!(quality.maximum_state.right_bytes, expected.2);
            assert_eq!(quality.maximum_state.total(), Some(expected.3));
            assert_eq!(quality.scratch_fixed_bytes, 24);
            assert_eq!(quality.scratch_bytes_per_frame, 0);
            assert_eq!(quality.tail, TailSamples::Infinite);
        }
    }

    /// E1: the tap-major reorder is bit-preserving against the frozen scalar order of the brief.
    #[test]
    fn phase_outputs_match_the_frozen_scalar_order() {
        let coefficients = LimiterCoef::<f32>::new(false, false);
        let mut history = [0.0_f32; HISTORY_WORDS];
        let mut kernel_history = History::<f32>::zero();
        let mut noise = Noise(0x5150_0090_0001);
        for _ in 0..4096 {
            let sample = noise.next() * 3.0;
            for tap in (1..HISTORY_WORDS).rev() {
                history[tap] = history[tap - 1];
            }
            history[0] = sample;
            // Typed from the brief: increasing tap order, `+0.0` accumulator, separately rounded
            // multiply then add. `#[allow]` because the brief's order is the assertion.
            #[allow(clippy::assign_op_pattern)]
            let expected = {
                let mut expected = [0.0_f32; 4];
                for (phase, output) in expected.iter_mut().enumerate() {
                    let mut accumulator = 0.0_f32;
                    for (tap, word) in history.iter().enumerate() {
                        accumulator = accumulator + ANNEX2_FIR[tap][phase] * *word;
                    }
                    *output = accumulator;
                }
                expected
            };
            let _ = detector_peak(&mut kernel_history, sample, &coefficients.fir);
            let produced = annex2_phases(&kernel_history, &coefficients.fir);
            for (phase, value) in produced.iter().enumerate() {
                assert_eq!(
                    value.to_bits(),
                    expected[phase].to_bits(),
                    "phase {phase} bits"
                );
            }
        }
    }

    /// E2: the frozen table and the phase outputs against the independent `f64` oracle.
    #[test]
    fn bs1770_annex2_conformance_is_unchanged() {
        let coefficients = LimiterCoef::<f32>::new(false, false);
        for tap in 0..HISTORY_WORDS {
            let mut unit = [0.0_f64; HISTORY_WORDS];
            unit[tap] = 1.0;
            let oracle = reference_annex2_phases(&unit);
            for phase in 0..4 {
                assert_eq!(
                    f64::from(ANNEX2_FIR[tap][phase]),
                    oracle[phase],
                    "table row {tap} phase {phase}"
                );
            }
        }
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let mut history = History::<f32>::zero();
            let mut oracle_history = [0.0_f64; HISTORY_WORDS];
            let mut noise = Noise(0x1770_0000 ^ u64::from(rate));
            for _ in 0..4096 {
                let sample = noise.next();
                let _ = detector_peak(&mut history, sample, &coefficients.fir);
                for tap in (1..HISTORY_WORDS).rev() {
                    oracle_history[tap] = oracle_history[tap - 1];
                }
                oracle_history[0] = f64::from(sample);
                let oracle = reference_annex2_phases(&oracle_history);
                let produced = annex2_phases(&history, &coefficients.fir);
                for phase in 0..4 {
                    assert!(
                        (f64::from(produced[phase]) - oracle[phase]).abs() <= 2.0e-6,
                        "rate {rate} phase {phase}"
                    );
                }
            }
        }
    }

    /// E3: the declared latency, the guarded ceiling and the bypass bits (contract, unchanged).
    #[test]
    fn fixed_latency_guarded_ceiling_and_bypass_bits_hold() {
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let latency = (rate / 100 + 6) as usize;
            // `P[6] = max(|h[6]| = 1, |v_p|) = 1`, so `r[6]` is exactly the guarded limit and
            // `g[T] <= r[T-N] = r[6]`: the impulse emerges at or below the guarded ceiling.
            let guard = limit_coefficient(-6.0);
            for lookahead in [0.0_f32, 5.0, 10.0] {
                let values = values_with(-6.0, 100.0, lookahead);
                let mut effect = TruePeakLimiterFactory
                    .prepare(request_at_rate(&values, rate))
                    .expect("prepare");
                assert_eq!(effect.metadata().latency, LatencySamples(latency as u64));
                let mut left = vec![0.0; latency + 1];
                let mut right = vec![0.0; latency + 1];
                left[0] = 1.0;
                right[0] = 0.5;
                render(effect.as_mut(), &mut left, &mut right, 128);
                assert!(
                    left[..latency].iter().all(|sample| sample.to_bits() == 0),
                    "rate {rate} lookahead {lookahead}: output before latency"
                );
                assert!(
                    left[latency].abs() <= guard,
                    "rate {rate} lookahead {lookahead}: {} > {guard}",
                    left[latency].abs()
                );
                assert!(right[latency].abs() <= guard);
            }
        }

        let values = values_with(-6.0, 100.0, 10.0);
        let mut bypass_request = request(&values);
        bypass_request.bypass = true;
        let mut bypass = TruePeakLimiterFactory
            .prepare(bypass_request)
            .expect("bypass prepare");
        let mut left = vec![0.0; 487];
        let mut right = vec![0.0; 487];
        left[0] = -0.0;
        right[0] = 0.25;
        render(bypass.as_mut(), &mut left, &mut right, 128);
        assert_eq!(left[486].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(right[486].to_bits(), 0.25_f32.to_bits());
    }

    /// E6: the gain ramp reaches the requirement without a step.
    ///
    /// A step in level from a transparent 0.25 to an overloading 4.0. Because the dry signal is
    /// never zero, the applied gain is observable at every sample, and the assertion is the shape
    /// the law promises: monotone descent, no single-sample fall larger than one box term, and the
    /// requirement met by the time the loud sample reaches the output.
    #[test]
    fn the_gain_ramp_falls_gradually_and_arrives_at_the_requirement() {
        let values = values_with(-12.0, 100.0, 5.0);
        let latency = 486_usize;
        let window = 241_usize;
        let mut effect = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let frames = 1024_usize;
        // The step is late enough that the ramp starts after the delay line has filled, so the
        // whole descent is observable at the output.
        let step = 400_usize;
        let source: Vec<f32> = (0..frames)
            .map(|frame| if frame >= step { 4.0 } else { 0.1 })
            .collect();
        let mut left = source.clone();
        let mut right = source.clone();
        render(effect.as_mut(), &mut left, &mut right, 128);

        let mut previous = 1.0_f32;
        let mut falls = 0_usize;
        for frame in latency..frames {
            let dry = source[frame - latency];
            let gain = left[frame] / dry;
            assert!(gain <= previous + 1.0e-6, "gain rose at {frame}");
            let fall = previous - gain;
            assert!(
                fall <= 1.0 / window as f32 + 1.0e-6,
                "gain fell by {fall} in one sample at {frame}"
            );
            if fall > 0.0 {
                falls += 1;
            }
            previous = gain;
        }
        assert!(falls > 16, "the descent took {falls} steps, not a ramp");
        // The requirement is met exactly when the loud sample arrives, not after it.
        let arrival = step + latency;
        assert!(
            left[arrival].abs() <= limit_coefficient(-12.0),
            "arrival sample {} exceeds the guarded limit",
            left[arrival].abs()
        );
    }

    /// E7: after silence the reduction returns to exactly `+0.0`, so the path is bit-transparent.
    ///
    /// The release is a one-pole on the reduction `d`; the D7 flush is what turns its asymptotic
    /// decay into an exact `+0.0`, and an exact `+0.0` is what makes `g` exactly `1.0` and
    /// `z * 1.0` exactly `z`, signed zero included. Without the flush the decay would pass under
    /// `f32`'s normal range and stay there for ever. At a 10 ms release it crosses `FLUSH_EPS`
    /// after about 22 000 samples, which is why the silence is that long.
    ///
    /// The sweep includes a 2 ms lookahead deliberately: that is `Wb = 97`, and 97 is one of the
    /// few window lengths for which `97 * (1 / 97)` is not exactly `1.0` in `f32`. The box average
    /// is a division precisely so that an unlimited block is the exact identity at every window.
    #[test]
    fn silence_restores_exact_identity_including_signed_zero() {
        for lookahead in [0.0_f32, 2.0, 5.0, 10.0] {
            let values = values_with(-6.0, 10.0, lookahead);
            let mut effect = TruePeakLimiterFactory
                .prepare(request(&values))
                .expect("prepare");
            let mut noise = Noise(0x9001);
            let mut left = vec![0.0_f32; 32_768];
            let mut right = vec![0.0_f32; 32_768];
            for index in 0..1024 {
                left[index] = noise.next() * 4.0;
                right[index] = noise.next() * 4.0;
            }
            render(effect.as_mut(), &mut left, &mut right, 128);

            // The recursive word itself, not just its effect on the output.
            let payload = snapshot(effect.as_ref());
            assert_eq!(
                read_f32(&payload.1, words::REDUCTION).to_bits(),
                0.0_f32.to_bits(),
                "lookahead {lookahead}: the reduction never reached +0.0"
            );
            assert_eq!(
                read_f32(&payload.2, words::REDUCTION).to_bits(),
                0.0_f32.to_bits()
            );

            let mut left = vec![0.0_f32; 1024];
            let mut right = vec![0.0_f32; 1024];
            left[100] = -0.0;
            left[200] = 0.25;
            right[100] = 0.25;
            let expected_left = left.clone();
            let expected_right = right.clone();
            render(effect.as_mut(), &mut left, &mut right, 128);
            let latency = 486;
            for index in latency..1024 {
                assert_eq!(
                    left[index].to_bits(),
                    expected_left[index - latency].to_bits(),
                    "lookahead {lookahead}: left identity at {index}"
                );
                assert_eq!(
                    right[index].to_bits(),
                    expected_right[index - latency].to_bits(),
                    "lookahead {lookahead}: right identity at {index}"
                );
            }
        }
    }

    /// Issue #144 item 6: the stationary hoist reads the value `advance` would have produced.
    ///
    /// This effect had no ramping split at all -- four `RampLanes::advance` per frame, every
    /// frame. The hoist skips them when nothing is moving, so the gate is that `resting_value`
    /// and `advance` agree bitwise at rest, *and* that `advance` leaves the state untouched
    /// there. If either half stopped holding, the skip would be a re-tuning rather than a
    /// no-op, which is exactly the failure the class-A bar exists to catch.
    #[test]
    fn the_stationary_hoist_reads_what_advancing_would_have_produced() {
        fn check<L: Lane>() {
            let values = [0.0_f32, -1.0, 0.25, 100.0, -0.5, 3.0, 1.0e-7, 7.0];
            let mut scalar = [LinearRamp::fixed(0.0); MAXIMUM_WIDTH];
            for (lane, ramp) in scalar.iter_mut().enumerate().take(L::WIDTH) {
                *ramp = LinearRamp::fixed(values[lane]);
            }
            assert!(
                ramps_are_stationary(&scalar[..L::WIDTH]),
                "width {}: ramps built at rest must read as stationary",
                L::WIDTH
            );

            let mut lanes = RampLanes::<L>::gather(&scalar[..L::WIDTH]);
            let mut rested = [0.0_f32; MAXIMUM_WIDTH];
            let mut advanced = [0.0_f32; MAXIMUM_WIDTH];
            // Several frames, because the hoist skips the whole block, not one sample.
            for frame in 0..8 {
                lanes.resting_value().store(&mut rested);
                let before = lanes;
                lanes.advance().store(&mut advanced);
                for lane in 0..L::WIDTH {
                    assert_eq!(
                        rested[lane].to_bits(),
                        advanced[lane].to_bits(),
                        "width {} lane {lane} frame {frame}: resting value diverged",
                        L::WIDTH
                    );
                }
                let mut before_state = [0.0_f32; MAXIMUM_WIDTH];
                let mut after_state = [0.0_f32; MAXIMUM_WIDTH];
                before.current.store(&mut before_state);
                lanes.current.store(&mut after_state);
                for lane in 0..L::WIDTH {
                    assert_eq!(
                        before_state[lane].to_bits(),
                        after_state[lane].to_bits(),
                        "width {} lane {lane} frame {frame}: advancing at rest moved the state",
                        L::WIDTH
                    );
                }
            }
        }
        check::<f32>();
        check::<miso_engine_lane::Simd4>();
        check::<miso_engine_lane::Simd8>();
    }

    /// A ramp with a window open is never stationary, however small the move.
    #[test]
    fn an_open_window_is_never_stationary() {
        let mut ramps = [LinearRamp::fixed(0.5); 4];
        assert!(ramps_are_stationary(&ramps));
        // One ULP: the smallest real move the bit compare must still refuse to hoist.
        ramps[2].set_target(f32::from_bits(0.5_f32.to_bits() + 1), RAMP_UPDATES);
        assert!(
            !ramps_are_stationary(&ramps),
            "a one-ULP retarget must open a window"
        );
        // A redundant retarget is hoisted by `LinearRamp::set_target` itself, so it stays at rest.
        let mut redundant = [LinearRamp::fixed(0.5); 4];
        redundant[1].set_target(0.5, RAMP_UPDATES);
        assert!(
            ramps_are_stationary(&redundant),
            "a redundant retarget must not open a window"
        );
    }

    /// E13: the lane-wide coefficient ramp is `LinearRamp::next_value`, snap included.
    ///
    /// `RampLanes::advance` is a second implementation of the runtime's D11 law, in the vector
    /// domain, so it needs its own gate: `remaining = max(remaining - 1, 0)` then
    /// `current = select(remaining > 0, current + step, target)`. The scalar ramp is the oracle and
    /// the comparison is `to_bits`, at every width and across the snap.
    #[test]
    fn the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit() {
        fn check<L: Lane>() {
            let mut scalar = [LinearRamp::fixed(0.0); MAXIMUM_WIDTH];
            let starts = [0.0_f32, -1.0, 0.25, 100.0, -0.5, 3.0, 0.0, 7.0];
            let targets = [1.0_f32, 2.0, 0.25, -100.0, 0.5, -3.0, 1.0e-4, 0.0];
            for (lane, ramp) in scalar.iter_mut().enumerate().take(L::WIDTH) {
                *ramp = LinearRamp::fixed(starts[lane]);
                ramp.set_target(targets[lane], RAMP_UPDATES);
            }
            let mut lanes = RampLanes::<L>::gather(&scalar[..L::WIDTH]);
            let mut expected = scalar;
            let mut produced = [0.0_f32; MAXIMUM_WIDTH];
            for update in 0..RAMP_UPDATES + 4 {
                lanes.advance().store(&mut produced);
                for lane in 0..L::WIDTH {
                    assert_eq!(
                        produced[lane].to_bits(),
                        expected[lane].next_value().to_bits(),
                        "width {} lane {lane} update {update}",
                        L::WIDTH
                    );
                }
            }
            // The scattered state is the scalar state, so a block boundary is not observable.
            let mut scattered = scalar;
            lanes.scatter(&mut scattered[..L::WIDTH]);
            for lane in 0..L::WIDTH {
                assert_eq!(
                    scattered[lane].current.to_bits(),
                    expected[lane].current.to_bits()
                );
                assert_eq!(scattered[lane].remaining, expected[lane].remaining);
            }
        }
        check::<f32>();
        check::<Simd4>();
        check::<Simd8>();
    }

    /// E8: one body, three widths; PCM, per-track payload bytes and reports agree by `to_bits`.
    #[test]
    fn lane_identity_holds_across_widths() {
        let lookaheads = [0.0_f32, 5.0, 10.0, 2.0, 7.0, 10.0, 0.0, 5.0];
        let ceilings = [-1.0_f32, -6.0, -12.0, -3.0, -1.0, -24.0, -6.0, -2.0];
        let tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..8)
            .map(|track| values_with(ceilings[track], 100.0, lookaheads[track]))
            .collect();
        let frames = 640_usize;
        let mut inputs = Vec::new();
        for track in 0..8 {
            let mut noise = Noise(0xBEEF_0000 + track as u64);
            let left: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            let right: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            inputs.push((left, right));
        }

        for link in [LinkMode::DualMono, LinkMode::Maximum] {
            let mut scalar_out = Vec::new();
            let mut scalar_state = Vec::new();
            for track in 0..8 {
                let mut preparation = request(&tracks[track]);
                preparation.link_mode = link;
                let mut effect = TruePeakLimiterFactory
                    .prepare(preparation)
                    .expect("prepare");
                let mut left = inputs[track].0.clone();
                let mut right = inputs[track].1.clone();
                render(effect.as_mut(), &mut left, &mut right, 128);
                scalar_state.push(snapshot(effect.as_ref()));
                scalar_out.push((left, right));
            }

            for (width, backend, lanes) in [
                (BankWidth::Four, Backend::Simd4, 4_usize),
                (BankWidth::Eight, Backend::Simd8, 8),
            ] {
                for group in 0..8 / lanes {
                    let members: Vec<_> = (0..lanes)
                        .map(|lane| tracks[group * lanes + lane])
                        .collect();
                    let mut bank = bank_for(&members, link, width, backend);
                    let mut left = vec![0.0_f32; frames * lanes];
                    let mut right = vec![0.0_f32; frames * lanes];
                    for frame in 0..frames {
                        for lane in 0..lanes {
                            left[frame * lanes + lane] = inputs[group * lanes + lane].0[frame];
                            right[frame * lanes + lane] = inputs[group * lanes + lane].1[frame];
                        }
                    }
                    for block in 0..frames / 128 {
                        let start = block * 128 * lanes;
                        let end = start + 128 * lanes;
                        process_bank(
                            bank.as_mut(),
                            &mut left[start..end],
                            &mut right[start..end],
                            width,
                            128,
                            (block * 128) as u64,
                        );
                    }
                    for lane in 0..lanes {
                        let track = group * lanes + lane;
                        for frame in 0..frames {
                            assert_eq!(
                                left[frame * lanes + lane].to_bits(),
                                scalar_out[track].0[frame].to_bits(),
                                "{link:?} W{lanes} left track {track} frame {frame}"
                            );
                            assert_eq!(
                                right[frame * lanes + lane].to_bits(),
                                scalar_out[track].1[frame].to_bits(),
                                "{link:?} W{lanes} right track {track} frame {frame}"
                            );
                        }
                        assert_eq!(
                            snapshot_track(bank.as_ref(), lane as u32),
                            scalar_state[track],
                            "{link:?} W{lanes} payload track {track}"
                        );
                    }
                }
            }
        }
    }

    /// The #182 S1 cohort helper: renders `lanes` tracks both as scalar instances and as one bank,
    /// with an optional payload swap partway through, and returns both output plans lane major.
    ///
    /// The two arms are the same tracks, the same samples and the same block boundaries, so any
    /// difference between them is the uniform-cohort gate deciding something the per-lane body
    /// would not have decided.
    struct CohortRun {
        scalar: Vec<(Vec<f32>, Vec<f32>)>,
        bank_left: Vec<f32>,
        bank_right: Vec<f32>,
        frames: usize,
        lanes: usize,
    }

    impl CohortRun {
        fn assert_lane_identity(&self, label: &str) {
            for lane in 0..self.lanes {
                for frame in 0..self.frames {
                    assert_eq!(
                        self.bank_left[frame * self.lanes + lane].to_bits(),
                        self.scalar[lane].0[frame].to_bits(),
                        "{label}: left lane {lane} frame {frame}"
                    );
                    assert_eq!(
                        self.bank_right[frame * self.lanes + lane].to_bits(),
                        self.scalar[lane].1[frame].to_bits(),
                        "{label}: right lane {lane} frame {frame}"
                    );
                }
            }
        }
    }

    /// One track's state payload: the common, left and right sections a snapshot produces.
    type LanePayload = (Vec<u8>, Vec<u8>, Vec<u8>);

    /// Renders `tracks` through one W8 bank over `blocks` blocks of 128 frames.
    ///
    /// The bank arm of [`cohort_run`] on its own, over the same per-lane signal, for the
    /// comparisons whose oracle is another *bank* rather than a scalar twin. A scalar instance is
    /// `W = 1` and therefore uniform by construction, so it is not a usable oracle for anything
    /// the uniform body does to a whole channel.
    fn bank_planes(
        tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        blocks: usize,
    ) -> (Vec<f32>, Vec<f32>) {
        const LANES: usize = 8;
        assert_eq!(tracks.len(), LANES);
        let frames = blocks * 128;
        let mut left = vec![0.0_f32; frames * LANES];
        let mut right = vec![0.0_f32; frames * LANES];
        for lane in 0..LANES {
            let mut noise = Noise(0x5182_0000 + lane as u64);
            let lane_left: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            let lane_right: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            for frame in 0..frames {
                left[frame * LANES + lane] = lane_left[frame];
                right[frame * LANES + lane] = lane_right[frame];
            }
        }
        let mut bank = bank_for(tracks, LinkMode::DualMono, BankWidth::Eight, Backend::Simd8);
        for block in 0..blocks {
            let start = block * 128 * LANES;
            let end = start + 128 * LANES;
            process_bank(
                bank.as_mut(),
                &mut left[start..end],
                &mut right[start..end],
                BankWidth::Eight,
                128,
                (block * 128) as u64,
            );
        }
        (left, right)
    }

    /// Renders `tracks` through eight scalar instances and one W8 bank over `blocks` blocks of 128.
    ///
    /// `swap_after` optionally restores `donor` into track 0 of both arms after that many blocks,
    /// which is how the phase leg of [`lanes_uniform`] is reached: a payload carries its own van
    /// Herk phase, and committing one lane of a bank is the only way a cohort that shares a window
    /// can end a block with lanes at different window positions.
    fn cohort_run(
        tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        blocks: usize,
        swap_after: Option<(usize, LanePayload)>,
    ) -> CohortRun {
        const LANES: usize = 8;
        assert_eq!(tracks.len(), LANES);
        let frames = blocks * 128;
        let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..LANES)
            .map(|lane| {
                let mut noise = Noise(0x5182_0000 + lane as u64);
                (
                    (0..frames).map(|_| noise.next() * 3.0).collect(),
                    (0..frames).map(|_| noise.next() * 3.0).collect(),
                )
            })
            .collect();

        let mut instances: Vec<Box<dyn PreparedNativeEffect>> = tracks
            .iter()
            .map(|values| {
                TruePeakLimiterFactory
                    .prepare(request(values))
                    .expect("prepare")
            })
            .collect();
        let mut scalar: Vec<(Vec<f32>, Vec<f32>)> = inputs
            .iter()
            .map(|(left, right)| (left.clone(), right.clone()))
            .collect();

        let mut bank = bank_for(tracks, LinkMode::DualMono, BankWidth::Eight, Backend::Simd8);
        let mut bank_left = vec![0.0_f32; frames * LANES];
        let mut bank_right = vec![0.0_f32; frames * LANES];
        for frame in 0..frames {
            for lane in 0..LANES {
                bank_left[frame * LANES + lane] = inputs[lane].0[frame];
                bank_right[frame * LANES + lane] = inputs[lane].1[frame];
            }
        }

        for block in 0..blocks {
            if let Some((after, payload)) = swap_after.as_ref()
                && block == *after
            {
                let sizes = instances[0].metadata().state_sizes;
                instances[0]
                    .restore_state_payload(
                        STATE_LAYOUT_VERSION,
                        StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                            .expect("sizes"),
                    )
                    .expect("scalar restore");
                bank.restore_track_state_payload(
                    0,
                    STATE_LAYOUT_VERSION,
                    StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                        .expect("sizes"),
                )
                .expect("bank restore");
            }
            let start = block * 128;
            for (lane, effect) in instances.iter_mut().enumerate() {
                let (left, right) = &mut scalar[lane];
                effect.process(
                    EffectProcessBlock::new(
                        &mut left[start..start + 128],
                        &mut right[start..start + 128],
                        None,
                        start as u64,
                        &[],
                        128,
                    )
                    .expect("block"),
                );
            }
            process_bank(
                bank.as_mut(),
                &mut bank_left[start * LANES..(start + 128) * LANES],
                &mut bank_right[start * LANES..(start + 128) * LANES],
                BankWidth::Eight,
                128,
                start as u64,
            );
        }

        CohortRun {
            scalar,
            bank_left,
            bank_right,
            frames,
            lanes: LANES,
        }
    }

    /// **A cohort every lane of which shares one window renders exactly the per-lane body.**
    ///
    /// Issue #182 S1. This is the arm the vectorised van Herk and the vectorised box-expiry gather
    /// actually run on: [`lanes_uniform`] accepts it, so `sliding_minimum_uniform` and the
    /// lane-wide `expired` load replace `W` scalar passes over the arena. A scalar instance is
    /// `L = f32`, `W = 1`, which is uniform by construction, so the comparison is the vectorised
    /// path against the same law one lane at a time.
    ///
    /// What this test is, precisely, is the **lane-identity** property *of the uniform path*: the
    /// scalar arm runs `sliding_minimum_uniform` at `W = 1` and the bank arm runs it at `W = 8`, so
    /// a mutation that treats the wide instantiation differently from the narrow one is red here
    /// and nowhere else. Its red mutation is the #182 analogue of row 7: guard the uniform suffix
    /// pass with `complete && width < 2` so it never runs at W4/W8. `lane_identity_holds_across_
    /// widths` and `a_mixed_lookahead_cohort_falls_back_bit_identically` both stay green under it,
    /// because every cohort they build falls back.
    ///
    /// Round 2 R1(d): the segment walk visits exactly the slots a frame-at-a-time walk visits.
    ///
    /// The whole of [`segment`]'s claim is an arithmetic one — that
    /// `((c + o) mod R) + step` is `(c + step + o) mod R` for every step of a run it sized — and
    /// this is that claim as a test rather than as prose. The oracle is written with `%`, not with
    /// [`wrapped`], so it is an independent formulation and not the same conditional subtraction
    /// compared with itself; and it walks *both* channels, because the two carry different window
    /// shapes and their wrap points interleave, which is the case a single-channel argument would
    /// miss.
    ///
    /// The sweep covers every launch rate the crate supports, the boundary lookaheads (zero, the
    /// clamp at `MINIMUM_RAMP_WINDOW`, and the maximum `N`, where `Wb == R` collapses the window
    /// end onto the write cursor and the box offset to zero), and cursor positions at both ends of
    /// both rings. It also asserts what the release build depends on and the `debug_assert`s
    /// state: every slot the walk produces is in range, and no run is empty.
    #[test]
    fn the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits() {
        fn oracle(
            shape: &Shape,
            ring_cursor: usize,
            main_cursor: usize,
            o: WindowOffsets,
        ) -> FrameSlots {
            FrameSlots {
                ring_cursor,
                main_cursor,
                end: (ring_cursor + o.end_offset) % shape.ring,
                start: (ring_cursor + 1) % shape.ring,
                expiring: (ring_cursor + o.box_offset) % shape.ring,
            }
        }
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let shape = Shape::new(rate).expect("shape");
            let lookaheads = [
                0,
                1,
                MINIMUM_RAMP_WINDOW as usize,
                240,
                shape.n - 1,
                shape.n,
            ];
            for left_lookahead in lookaheads {
                for right_lookahead in [0, 7, 240, shape.n] {
                    let left = WindowOffsets::new(LaneShape::new(left_lookahead, &shape));
                    let right = WindowOffsets::new(LaneShape::new(right_lookahead, &shape));
                    for ring_start in [0, 1, shape.ring / 2, shape.ring - 2, shape.ring - 1] {
                        for main_start in [0, 5, shape.main - 1] {
                            let frames = 3 * DETECTOR_CHUNK + 7;
                            let mut expected = Vec::with_capacity(frames);
                            let (mut ring_cursor, mut main_cursor) = (ring_start, main_start);
                            for _ in 0..frames {
                                expected.push((
                                    oracle(&shape, ring_cursor, main_cursor, left),
                                    oracle(&shape, ring_cursor, main_cursor, right),
                                ));
                                ring_cursor = (ring_cursor + 1) % shape.ring;
                                main_cursor = (main_cursor + 1) % shape.main;
                            }

                            let mut produced = Vec::with_capacity(frames);
                            let (mut ring_cursor, mut main_cursor) = (ring_start, main_start);
                            let mut done = 0;
                            let mut segments = 0;
                            while done < frames {
                                // The real loop never asks for more than one detector chunk at a
                                // time, because the frame loop lives inside the chunk loop.
                                let remaining = (frames - done).min(DETECTOR_CHUNK);
                                let walk = segment(
                                    &shape,
                                    ring_cursor,
                                    main_cursor,
                                    remaining,
                                    left,
                                    right,
                                );
                                assert!(walk.run >= 1, "empty segment at {ring_cursor}");
                                assert!(walk.run <= remaining);
                                for step in 0..walk.run {
                                    let slots =
                                        (walk.left.advanced(step), walk.right.advanced(step));
                                    for side in [slots.0, slots.1] {
                                        assert!(side.ring_cursor < shape.ring);
                                        assert!(side.end < shape.ring);
                                        assert!(side.start < shape.ring);
                                        assert!(side.expiring < shape.ring);
                                        assert!(side.main_cursor < shape.main);
                                    }
                                    produced.push(slots);
                                }
                                done += walk.run;
                                ring_cursor = wrapped(ring_cursor + walk.run, shape.ring);
                                main_cursor = wrapped(main_cursor + walk.run, shape.main);
                                segments += 1;
                            }
                            assert_eq!(
                                produced, expected,
                                "rate {rate} lookaheads {left_lookahead}/{right_lookahead} \
                                 cursors {ring_start}/{main_start}"
                            );
                            // The point of the split is that it is rare: a block of this length
                            // takes a handful of segments, not one per frame.
                            assert!(
                                segments <= frames / 8,
                                "{segments} segments for {frames} frames"
                            );
                        }
                    }
                }
            }
        }
    }

    /// It is deliberately *not* claimed to gate a mutation that applies at every width — the two
    /// arms would move together, since a scalar instance is `W = 1` and therefore uniform too.
    /// Those are gated by the frozen E12 pins (which a moved scalar digest breaks immediately) and
    /// by `a_mixed_lookahead_cohort_falls_back_bit_identically`, whose bank lanes run the per-lane
    /// body against scalar twins running this one.
    ///
    /// The E12 corpus already carries this path at the digest level — cases 2 and 3 give every lane
    /// the same lookahead, so they take it at W4 and W8 while cases 0, 1 and 4 take the fallback —
    /// but a pinned digest says *which* bits, not *why*, and this test names the why.
    #[test]
    fn a_uniform_cohort_renders_exactly_the_per_lane_path() {
        let tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..8)
            .map(|lane| values_with(-6.0 - lane as f32, 100.0, 5.0))
            .collect();
        cohort_run(&tracks, 6, None).assert_lane_identity("uniform cohort");
    }

    /// **A cohort with one differently prepared lookahead falls back, bit for bit.**
    ///
    /// Issue #182 S1, the shape leg of [`lanes_uniform`]. Seven lanes at 5 ms and one at 1 ms is
    /// the adversarial shape rather than eight distinct ones: a gate that compared only the first
    /// two lanes, or only `window` and not `box_offset`, would still reject eight distinct
    /// lookaheads and would wrongly accept this.
    ///
    /// Red mutation: drop the shape leg, `state.lane.iter().all(..)`, from `lanes_uniform`. The
    /// odd lane is then rendered with lane 0's 241-sample window instead of its own 49-sample one,
    /// and diverges from its scalar twin inside the first block. (The phase leg does not cover
    /// this: every lane starts a fresh cohort at phase 0, so the first block is admitted before
    /// the differing windows have had a chance to desync the phases.)
    ///
    /// It is also the crate's **cross-path** gate, which is worth stating because it is not
    /// obvious from the name. The bank arm falls back to the per-lane body for all eight lanes,
    /// while every scalar twin is `W = 1` and therefore runs `sliding_minimum_uniform`. So the
    /// seven lanes that are not the odd one out compare the uniform body against the per-lane body
    /// on the same samples, and mutations *inside* the uniform body are red here: `suffix.min(..)`
    /// → `suffix.max(..)`, `for _ in 0..window` → `0..window - 1`, `state.phase.fill(position + 1)`
    /// → `fill(position)`, and `state.lane[0].box_offset` → `end_offset` in the uniform gather.
    #[test]
    fn a_mixed_lookahead_cohort_falls_back_bit_identically() {
        let mut tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> =
            (0..8).map(|_| values_with(-6.0, 100.0, 5.0)).collect();
        tracks[3] = values_with(-6.0, 100.0, 1.0);
        cohort_run(&tracks, 6, None).assert_lane_identity("mixed lookahead cohort");
    }

    /// **The two channels of a uniform cohort keep their own van Herk phases.**
    ///
    /// Closes the adversarial verifier's M-A finding. Round 2 R1(a) moved `prefix` and `phase` out
    /// of the arena and into block locals, written back once at the end of
    /// [`limiter_block_uniform`]. The `round2-1` and `round2-2` mutation rows gate a **dropped**
    /// write-back; nothing gated a **crossed** one. Writing `right`'s phase into `left` at the
    /// block end survives every other test in this crate while moving rendered bits, and this is
    /// the test that does not.
    ///
    /// Red mutation (M-A): `left.phase.fill(left_phase)` → `left.phase.fill(right_phase)` at the
    /// block-end write-back of `limiter_block_uniform`.
    ///
    /// # Why the obvious gates cannot reach it
    ///
    /// The crossing is only observable when the two channels are at *different* window positions,
    /// which needs a per-channel lookahead split — every other test in the crate prepares both
    /// channels alike, and there the crossed value is the value being overwritten. And once the
    /// split exists, neither of the crate's two standing comparison shapes helps:
    ///
    /// * **Cross-width** (`lane_identity_holds_across_widths`, `assert_lane_identity`) compares a
    ///   bank against scalar twins, and a scalar instance is `W = 1` and therefore uniform by
    ///   construction — it runs the same crossed write-back. Both widths corrupt identically and
    ///   agree.
    /// * **Partition invariance** compares one long block against several short ones. `right`'s
    ///   phase is uncorrupted and advances one step per frame, so at any shared block boundary it
    ///   is `frames mod Wb_right` whatever the partition was; the corrupted `left` inherits it and
    ///   re-syncs. Both partitions corrupt identically and agree.
    ///
    /// So the oracle has to be a rendering of the same asymmetric configuration that does **not**
    /// run the uniform write-back. The per-lane fallback body is exactly that: it writes each
    /// lane's phase from that lane's own `sliding_minimum`, per channel, and shares no code with
    /// the crossed line. Both arms below are W8 banks over the same signal; lane 7 of the oracle
    /// arm carries a third, different *left* lookahead, which makes `lanes_uniform(left)` false
    /// and sends the whole bank down the fallback. Lanes 0 through 6 are prepared identically in
    /// the two arms, so their rendered samples must agree to the bit.
    #[test]
    fn the_two_channels_of_a_uniform_cohort_keep_their_own_phases() {
        const BLOCKS: usize = 16;
        const LANES: usize = 8;
        // The three windows this test needs to be distinct. Asserted rather than assumed: if the
        // clamp in `LaneShape::new` ever swallowed one of them, the comparison below would still
        // pass and would be gating nothing.
        let shape = Shape::new(48_000).expect("shape");
        let window = |milliseconds: f32| {
            LaneShape::new(lookahead_samples(milliseconds, 48_000, shape.n), &shape).window
        };
        assert_ne!(
            window(5.0),
            window(1.0),
            "the two channels must sit at different window lengths for a crossed phase to show"
        );
        assert_ne!(
            window(3.0),
            window(5.0),
            "the odd lane must differ from the cohort or the oracle arm stays uniform"
        );

        // Subject: every lane the same asymmetric program, so both channels are internally uniform
        // and the bank takes `limiter_block_uniform`.
        let subject: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..LANES)
            .map(|_| values_split(-6.0, 100.0, 5.0, 1.0))
            .collect();
        // Oracle: lane 7's *left* lookahead differs, so `lanes_uniform(left)` is false and both
        // channels take the per-lane body. Lanes 0..7 are byte-identical to the subject's.
        let mut oracle = subject.clone();
        oracle[7] = values_split(-6.0, 100.0, 3.0, 1.0);

        let (subject_left, subject_right) = bank_planes(&subject, BLOCKS);
        let (oracle_left, oracle_right) = bank_planes(&oracle, BLOCKS);

        for lane in 0..LANES - 1 {
            for frame in 0..BLOCKS * 128 {
                let index = frame * LANES + lane;
                assert_eq!(
                    subject_left[index].to_bits(),
                    oracle_left[index].to_bits(),
                    "left lane {lane} frame {frame}"
                );
                assert_eq!(
                    subject_right[index].to_bits(),
                    oracle_right[index].to_bits(),
                    "right lane {lane} frame {frame}"
                );
            }
        }
    }

    /// **Restoring one track of a uniform cohort desynchronises the phase, and that falls back.**
    ///
    /// Issue #182 S1, the phase leg of [`lanes_uniform`], and the reason the gate is two bit
    /// compares rather than one. Every lane here is prepared with the same 5 ms lookahead, so the
    /// shape leg holds for the whole run and never fires. What moves is the *position inside the
    /// van Herk block*: `commit_lane` writes one lane's `phase` straight out of a payload, and the
    /// donor below was snapshotted two blocks into its own timeline while the cohort is three
    /// blocks into its. 256 mod 241 is 15 and 384 mod 241 is 143, so track 0 lands at phase 15
    /// inside a cohort resting at 143.
    ///
    /// Red mutation: drop the phase leg, `state.phase.iter().all(..)`, from `lanes_uniform`. The
    /// bank then reads `state.phase[0]` — the restored 15 — for all eight lanes, so the seven
    /// lanes that were *not* restored take their window minimum at the wrong window position and
    /// run their suffix pass on the wrong block boundary. They diverge from their scalar twins,
    /// which is the whole-bank failure a per-lane parameter causes when it is assumed uniform.
    ///
    /// The scalar arm restores the same payload into track 0 at the same block, so the comparison
    /// is not "did the restore change anything" — it is "did the restore change anything *for the
    /// other seven lanes*", which is the only thing the gate is responsible for.
    #[test]
    fn a_restore_that_desyncs_the_phase_falls_back() {
        let values = values_with(-6.0, 100.0, 5.0);
        let tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> =
            (0..8).map(|_| values).collect();

        // The donor: the same program, two blocks into a different signal, so its phase is 15.
        let mut donor = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let mut noise = Noise(0x0D0D_0182);
        let mut left: Vec<f32> = (0..256).map(|_| noise.next() * 3.0).collect();
        let mut right: Vec<f32> = (0..256).map(|_| noise.next() * 3.0).collect();
        render(donor.as_mut(), &mut left, &mut right, 128);
        let payload = snapshot(donor.as_ref());
        assert_eq!(
            read_u32(&payload.1, words::PHASE),
            256 % 241,
            "the donor is not at the phase this test is about"
        );

        let run = cohort_run(&tracks, 8, Some((3, payload)));
        run.assert_lane_identity("phase-desynchronised cohort");
    }

    // ---------------------------------------------------------------------------------------
    // Issue #182 S2: the earned silence fixed point.
    //
    // The family is the compressor's (`miso-engine-compressor/tests/silent_fixed_point.rs`) at a
    // kernel whose rest state is not all zeros. Two of the three rings rest at exactly `1.0` rather
    // than `+0.0`, and the argument transfers unchanged because what it needs is that a resting
    // ring is **uniform**: a read from any cursor position then returns the value a slow path would
    // have read, so a block that writes only the resting value back leaves it bit-identical
    // whatever the cursor did.
    //
    // This crate is also the one where the fixed point is *exactly* reachable, which the compressor
    // records as the precondition for engaging at all and does not have: `gain_reduction_db`
    // approaches `0` dB geometrically, whereas the box terms here live on the `2^-14` grid with
    // `BOX_GRID * R` below `2^24` at every launch rate, so the running sum arrives at exactly `Wb`,
    // and the D7 flush terminates the release at exactly `+0.0` rather than near it.
    // ---------------------------------------------------------------------------------------

    /// One block of signal, or a block filled with `quiet` — an exact `+0.0` or an exact `-0.0`.
    ///
    /// The signal is [`Noise`], the crate's SplitMix64, seeded from the block index so that two
    /// arms of the same comparison see the same samples and a corpus stays a seed rather than a
    /// file. It is deliberately *not* a sine: `f32::sin` is a platform transcendental, which
    /// decision D6 and `scripts/check-math-policy.sh` forbid anywhere in `src/` — including in a
    /// test module, since the policy scans the file and not the `cfg`.
    fn silence_plane(
        block: usize,
        frames: usize,
        width: usize,
        quiet: Option<f32>,
        amplitude: f32,
        negate: bool,
    ) -> Vec<f32> {
        if let Some(fill) = quiet {
            return vec![fill; frames * width];
        }
        let mut noise = Noise(0x5182_0000 ^ (block as u64).wrapping_mul(0x9E37_79B9));
        (0..frames * width)
            .map(|_| {
                let value = noise.next() * amplitude;
                if negate { -value } else { value }
            })
            .collect()
    }

    /// A core of `L::WIDTH` identically prepared lanes, driven directly through `process_block`.
    ///
    /// The same construction `a_nonfinite_block_is_zeroed_reset_and_counted` uses, and for the same
    /// reason: `process_block` *is* the shipped path — `process` and `process_bank` both end in it
    /// — and reaching it directly is what lets the control arm suppress the fast path without
    /// having to change the signal or the parameters to do it.
    fn silent_core<L: Lane>(ceiling: f32, release: f32, lookahead: f32) -> LimiterCore<L> {
        let values = values_with(ceiling, release, lookahead);
        let mut preparation = request(&values);
        preparation.link_mode = LinkMode::DualMono;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1, preparation)
            .expect("metadata");
        let (left, right) = initial_defaults(&values).expect("defaults");
        LimiterCore::<L>::new(
            metadata,
            vec![left; L::WIDTH].into_boxed_slice(),
            vec![right; L::WIDTH].into_boxed_slice(),
        )
        .expect("core")
    }

    /// Every word of one core's state, as bits: both cursors, then both channels' whole arenas.
    ///
    /// Comparing *this* between the two arms, and not only the rendered samples, is what makes the
    /// fast path's cursor and phase advances load-bearing. The compressor's version of this test
    /// records honestly that deleting its cursor advance passes every test in its file, because a
    /// ring of exact `+0.0` reads the same from every position; reading the state back directly
    /// closes that gap rather than restating it. `phase` is in the list for the same reason.
    fn state_bits<L: Lane>(core: &LimiterCore<L>) -> Vec<u32> {
        let mut words = vec![core.cursors.main, core.cursors.ring];
        for channel in [&core.left, &core.right] {
            for plane in [
                &channel.history,
                &channel.main_ring,
                &channel.required_ring,
                &channel.box_ring,
                &channel.reduction,
                &channel.prefix,
                &channel.box_sum,
            ] {
                words.extend(plane.iter().map(|value| value.to_bits()));
            }
            words.extend(channel.phase.iter().copied());
            // The two coefficient ramps, which the fast path must not freeze. A skipped block
            // advances no ramp, so a claim admitted while a de-zipper window was still open would
            // strand `current` short of its target for as long as the silence lasted.
            for ramps in [&channel.limit, &channel.release] {
                for ramp in ramps.iter() {
                    words.push(ramp.current.to_bits());
                    words.push(ramp.target.to_bits());
                    words.push(ramp.step.to_bits());
                    words.push(ramp.remaining);
                }
            }
        }
        words
    }

    /// What one arm of a silence comparison produced.
    struct SilenceArm {
        /// Every rendered sample of every block, both planes.
        rendered: Vec<u32>,
        /// The whole instance's state, sampled at the end of every block.
        states: Vec<u32>,
        /// Left lane 0's recursive reduction word, at the end of every block.
        reduction: Vec<u32>,
        /// Blocks the fast path actually took.
        engagements: u32,
    }

    /// Renders `plan` through `core`: `None` is a tone block, `Some(fill)` a constant plane.
    ///
    /// `force_slow` withdraws the claim before every block, which is the control arm: it changes
    /// nothing about the signal, the parameters or the block boundaries, so any difference between
    /// the arms is the fast path and only the fast path.
    fn run_silence_arm<L: Lane>(
        core: &mut LimiterCore<L>,
        plan: &[Option<f32>],
        frames: usize,
        amplitude: f32,
        force_slow: bool,
    ) -> SilenceArm {
        let width = L::WIDTH;
        let mut arm = SilenceArm {
            rendered: Vec::new(),
            states: Vec::new(),
            reduction: Vec::new(),
            engagements: 0,
        };
        for (block, quiet) in plan.iter().enumerate() {
            let mut left = silence_plane(block, frames, width, *quiet, amplitude, false);
            let mut right = silence_plane(block, frames, width, *quiet, amplitude, true);
            if force_slow {
                core.silent_fixed_point = false;
            }
            core.process_block(&mut left, &mut right, frames);
            arm.rendered
                .extend(left.iter().map(|value| value.to_bits()));
            arm.rendered
                .extend(right.iter().map(|value| value.to_bits()));
            arm.states.extend(state_bits(core));
            arm.reduction.push(core.left.reduction[0].to_bits());
        }
        arm.engagements = core.silent_engagements();
        arm
    }

    /// Renders `plan` twice at one width, once free and once forced slow, and asserts they agree.
    fn compare_silence_arms<L: Lane>(
        label: &str,
        plan: &[Option<f32>],
        ceiling: f32,
        release: f32,
        amplitude: f32,
    ) -> SilenceArm {
        let mut fast = silent_core::<L>(ceiling, release, 5.0);
        let mut slow = silent_core::<L>(ceiling, release, 5.0);
        let free = run_silence_arm(&mut fast, plan, 128, amplitude, false);
        let forced = run_silence_arm(&mut slow, plan, 128, amplitude, true);
        assert_eq!(
            forced.engagements, 0,
            "{label}: the control arm took the fast path, so it is not a control"
        );
        assert_eq!(
            free.rendered, forced.rendered,
            "{label}: the silent fast path moved a rendered bit"
        );
        assert_eq!(
            free.states, forced.states,
            "{label}: the silent fast path moved a state word"
        );
        free
    }

    /// A plan of `before` tone blocks, `quiet` silent blocks and `after` tone blocks.
    fn silence_plan(before: usize, quiet: usize, after: usize) -> Vec<Option<f32>> {
        let mut plan = vec![None; before];
        plan.extend(vec![Some(0.0_f32); quiet]);
        plan.extend(vec![None; after]);
        plan
    }

    /// **A settled silent limiter renders exactly the limiter that is never allowed to skip.**
    ///
    /// Issue #182 S2, the headline gate, at all three widths. The tone is well under the guarded
    /// ceiling (`limit = 10^(-7/20) = 0.447` against an amplitude of `0.05`), so `r = 1` at every
    /// frame and the recursive word never leaves `+0.0`. That is deliberate, and it is the same
    /// choice the compressor's file explains at length: it isolates the *rings* as the thing that
    /// has to drain, and it makes the fixed point reachable inside a test-sized run.
    /// `a_limiter_still_releasing_through_the_silence_is_never_frozen` is the arm that makes the
    /// recursive word's own leg load-bearing.
    ///
    /// What has to drain here is the main delay line, `B = N + 6 = 486` samples at 48 kHz, which is
    /// 3.8 blocks of 128. The claim is therefore earned at the end of the block in which both the
    /// line has gone entirely `+0.0` *and* the output has, and the engagement count below is that
    /// arithmetic read back rather than asserted loosely.
    ///
    /// Red mutations: drop `block_is_positive_zero` on either input plane from the admission test;
    /// drop `is_at_silent_rest` on either channel from the claim; drop `all_exactly_one(&self
    /// .main_ring)`'s counterpart `block_is_positive_zero(&self.main_ring)`; drop the output test
    /// from the claim; and — caught by the state comparison rather than the sample comparison —
    /// delete `self.cursors.advance(..)` or either `advance_rest_phase(..)` from the fast path.
    #[test]
    fn a_settled_silent_limiter_renders_exactly_the_never_fast_path() {
        const SILENT_BLOCKS: usize = 40;
        let plan = silence_plan(1, SILENT_BLOCKS, 8);
        let scalar = compare_silence_arms::<f32>("scalar", &plan, -6.0, 100.0, 0.05);
        let four = compare_silence_arms::<Simd4>("W4", &plan, -6.0, 100.0, 0.05);
        let eight = compare_silence_arms::<Simd8>("W8", &plan, -6.0, 100.0, 0.05);

        // Engagement is a property of the signal and the shape, not of the width.
        assert_eq!(
            (scalar.engagements, four.engagements),
            (eight.engagements, eight.engagements),
            "engagement rate depends on the lane width"
        );
        assert_eq!(
            eight.engagements, 35,
            "the fast path engaged on {} of {SILENT_BLOCKS} silent blocks, not the 35 the 486-sample \
             delay line allows",
            eight.engagements
        );

        // Anti-vacuity: the trailing tone is really rendered, so the comparison above had
        // something other than silence to compare. Without it the test would pass on a fast path
        // that simply stopped rendering.
        let per_block = 128 * 8 * 2;
        assert!(
            eight.rendered[eight.rendered.len() - per_block..]
                .iter()
                .any(|word| *word != 0),
            "the block after the silence rendered nothing at all"
        );
    }

    /// **A limiter still releasing when the tone returns is never frozen by the fast path.**
    ///
    /// Issue #182 S2, the recursive-word and delay-line legs, and the refusal the brief for this
    /// work asks to be *proved* rather than assumed. The tone is far over the ceiling and the
    /// release is 2 000 ms, so `d` decays by a factor of `1 - 1.04e-5` per sample and needs some
    /// four million samples — about 34 000 blocks — to fall from its working value to `FLUSH_EPS`
    /// and snap to exactly `+0.0`. Twenty-four blocks of silence is nowhere near it.
    ///
    /// So the correct code refuses for the whole silence, and the assertion is that it refuses:
    /// `engagements == 0`. The `assert_ne!` on the recursive word is what keeps that from being
    /// vacuous — a run in which the release had already finished would refuse for the wrong
    /// reason and prove nothing.
    ///
    /// Red mutation: drop `block_is_positive_zero(&self.reduction)` from `is_at_silent_rest`. The
    /// claim is then earned on the first block whose output has drained to `+0.0`, `d` is frozen
    /// part-way through its release, and the returning tone is limited by a reduction that was
    /// never allowed to finish — the exact failure the compressor's file describes, at a kernel
    /// that reaches it through a delay line as well as through the gain.
    #[test]
    fn a_limiter_still_releasing_through_the_silence_is_never_frozen() {
        const SILENT_BLOCKS: usize = 24;
        let plan = silence_plan(1, SILENT_BLOCKS, 8);
        for (label, arm) in [
            (
                "scalar",
                compare_silence_arms::<f32>("scalar", &plan, -6.0, 2_000.0, 3.0),
            ),
            (
                "W4",
                compare_silence_arms::<Simd4>("W4", &plan, -6.0, 2_000.0, 3.0),
            ),
            (
                "W8",
                compare_silence_arms::<Simd8>("W8", &plan, -6.0, 2_000.0, 3.0),
            ),
        ] {
            assert_eq!(
                arm.engagements, 0,
                "{label}: the fast path engaged during a release that had not finished"
            );
            assert_ne!(
                arm.reduction[SILENT_BLOCKS], 0,
                "{label}: the release had already terminated, so the refusal proves nothing"
            );
        }
    }

    /// **A `-0.0` input block is not silence, and is not skipped.**
    ///
    /// Issue #182 S2, the input side of the signed-zero rule. Masking the sign bit in
    /// `block_is_positive_zero` makes a `-0.0` block count as silence, and a standing claim then
    /// engages on it. Every other test in this family stays green under that mutation.
    ///
    /// This crate's exposure is its delay line, as the compressor's is. A `-0.0` block the kernel
    /// actually renders is written into `main_ring` and emerges `B = 486` samples — a little under
    /// four blocks — later, with its sign intact: `select(bypass, delayed, delayed * gain)` gives
    /// `-0.0 * 1.0 = -0.0` on the limiting arm and `-0.0` on the bypass arm. A fast path that
    /// skipped the block never writes it, so the sample that should have emerged is `+0.0`.
    ///
    /// The `any(.. == 0x8000_0000)` is the anti-vacuity: it asserts a `-0.0` really did come out
    /// the far end, so the comparison had the divergence available to it.
    #[test]
    fn a_negative_zero_input_block_is_not_treated_as_silence() {
        let mut plan = vec![None];
        plan.extend(vec![Some(0.0_f32); 40]);
        plan.push(Some(-0.0_f32));
        plan.extend(vec![Some(0.0_f32); 16]);

        let arm = compare_silence_arms::<Simd8>("W8", &plan, -6.0, 100.0, 0.05);
        assert!(
            arm.rendered.contains(&0x8000_0000),
            "no -0.0 ever reached the output, so the comparison had nothing to catch"
        );
        compare_silence_arms::<f32>("scalar", &plan, -6.0, 100.0, 0.05);
        compare_silence_arms::<Simd4>("W4", &plan, -6.0, 100.0, 0.05);
    }

    /// **A block carrying automation is rendered, not skipped, and the resident tap keeps up.**
    ///
    /// Issue #182 S2 at the contract boundary rather than the kernel one: `process` and
    /// `process_bank` withdraw the claim whenever the block carries any automation at all, and the
    /// #143 D2/R4 gain-reduction tap reads the same word during a skipped block that it reads
    /// during a rendered one.
    ///
    /// The withdrawal is **not** load-bearing for correctness at this crate, and saying so is more
    /// useful than implying otherwise. `SmoothingRule::Linear` here resolves to a constant
    /// `RAMP_UPDATES = 64`, so an accepted span that actually moves a coefficient leaves
    /// `remaining == 64` and the `ramps_are_stationary` leg refuses the next block anyway; a span
    /// that restates the value it already holds snaps through `LinearRamp::stationary_at` and
    /// changes no state at all, so skipping would have been correct. It is kept for two reasons
    /// that are: the claim must not silently depend on `RAMP_UPDATES` being non-zero, which is a
    /// tuning constant and not a contract; and it is what makes a restated point a usable
    /// forced-slow control arm, which is how the compressor's family is built.
    ///
    /// Red mutations: delete either `if !block.automation.is_empty()` withdrawal (the engagement
    /// assertion goes red); make `observe_resident` freshen the word it reads, as #143's E6-c does
    /// (the tap comparison goes red).
    #[test]
    fn automation_withdraws_the_claim_and_the_resident_tap_keeps_up() {
        let values = values_with(-6.0, 100.0, 5.0);
        let mut preparation = request(&values);
        preparation.link_mode = LinkMode::DualMono;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1, preparation)
            .expect("metadata");
        let (left_defaults, right_defaults) = initial_defaults(&values).expect("defaults");
        let mut effect = PreparedTruePeakLimiter {
            core: LimiterCore::<f32>::new(
                metadata,
                vec![left_defaults].into_boxed_slice(),
                vec![right_defaults].into_boxed_slice(),
            )
            .expect("core"),
        };

        let mut taps = Vec::new();
        let mut tap = ObservationSample::default();
        // One tone block, then long enough for the 486-sample line to drain and the claim to be
        // earned and used.
        for block in 0..16_usize {
            let quiet = (block > 0).then_some(0.0_f32);
            let mut left = silence_plane(block, 128, 1, quiet, 0.05, false);
            let mut right = silence_plane(block, 128, 1, quiet, 0.05, true);
            effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * 128) as u64,
                    &[],
                    128,
                )
                .expect("block"),
            );
            assert!(effect.observe_resident(0, &mut tap), "the tap must answer");
            taps.push((tap.left.to_bits(), tap.right.to_bits()));
        }
        let engaged = effect.core.silent_engagements();
        assert!(
            engaged > 0,
            "the claim was never used, so the rest of this test proves nothing"
        );
        assert!(
            taps[15..]
                .iter()
                .all(|(left, right)| *left == 0 && *right == 0),
            "the resident tap did not read the resting +0.0 reduction through the skipped blocks"
        );

        // One more silent block, this time carrying a point that restates the ceiling it already
        // holds. It must be rendered rather than skipped.
        let restated = [PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 16 * 128,
            end_sample: 16 * 128,
            start_value: -6.0,
            end_value: -6.0,
        }];
        let mut left = vec![0.0_f32; 128];
        let mut right = vec![0.0_f32; 128];
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 16 * 128, &restated, 128)
                .expect("block"),
        );
        assert_eq!(
            report.invalid_spans, 0,
            "the restated point must be accepted"
        );
        assert_eq!(
            effect.core.silent_engagements(),
            engaged,
            "a block carrying automation took the silent fast path"
        );
    }

    /// **A stale detector history refuses the claim, even when every ring is already at rest.**
    ///
    /// Issue #182 S2, the `history` leg of `is_at_silent_rest` — the one entry on that list that is
    /// not obviously needed, and the one that is not reachable at a 128-frame quantum.
    ///
    /// At an ordinary quantum the leg is *implied*: `main_ring` holds the last `B = 486` input
    /// samples, so `block_is_positive_zero(&main_ring)` already says the last 486 inputs were
    /// `+0.0`, and the twelve detector taps are a suffix of those. Deleting the history test
    /// therefore turns nothing red at 128 frames, and recording that would be the end of it if the
    /// implication held generally. It does not: the render quantum is caller-supplied, and a block
    /// of fewer than `HISTORY_WORDS = 12` frames cannot flush the taps that a longer block would.
    ///
    /// So this test runs eight-frame blocks. The instance starts at the rest state with one word
    /// changed — a small stale value in every detector tap, small enough (`0.01` against a guarded
    /// limit of `0.447`) that the estimate it produces never crosses the ceiling and every ring
    /// therefore stays at exactly `1.0`. Every leg of the claim but `history` holds after the first
    /// eight-frame block. Correct code refuses until the taps have drained; code without the leg
    /// earns the claim there and freezes four stale taps for the rest of the silence, which the
    /// loud tone at the end reads as part of its own Annex-2 estimate.
    ///
    /// The same shape is what a crafted payload reaches at any quantum: `commit_lane` writes
    /// `history` and `main_ring` from independent regions of a section, so a restore can install a
    /// zeroed delay line behind a non-zero history. That path is closed by the restore withdrawal;
    /// this one is closed by the leg.
    ///
    /// Red mutation: drop `block_is_positive_zero(&self.history)` from `is_at_silent_rest`.
    #[test]
    fn a_stale_detector_history_refuses_the_claim() {
        fn arm<L: Lane>(force_slow: bool) -> SilenceArm {
            let mut core = silent_core::<L>(-6.0, 100.0, 5.0);
            core.left.history.fill(0.01);
            core.right.history.fill(-0.01);
            let mut plan = vec![Some(0.0_f32); 6];
            plan.extend(vec![None; 400]);
            run_silence_arm(&mut core, &plan, 8, 3.0, force_slow)
        }

        let free = arm::<Simd8>(false);
        let forced = arm::<Simd8>(true);
        assert_eq!(
            forced.engagements, 0,
            "the control arm took the fast path, so it is not a control"
        );
        assert!(
            free.engagements > 0,
            "the fast path never engaged at all, so the test would pass on any refusal"
        );
        assert_eq!(
            free.rendered, forced.rendered,
            "a stale detector tap survived into the returning tone"
        );
        assert_eq!(
            free.states, forced.states,
            "a stale detector tap survived into the instance state"
        );
    }

    /// **A de-zipper window still open across a block boundary refuses the claim.**
    ///
    /// Issue #182 S2, the `ramps_are_stationary` leg of the admission test — the leg that is not
    /// reachable at a 128-frame quantum, for the mirror of the reason the `history` leg is not.
    /// `SmoothingRule::Linear` here resolves to `RAMP_UPDATES = 64` updates, so at any quantum of
    /// 64 frames or more a retarget is fully consumed inside the very block that carried it and no
    /// later block ever *begins* with a window open. The render quantum is caller-supplied, so
    /// that is a coincidence of one configuration and not a property of the effect.
    ///
    /// At eight frames a retarget takes eight blocks to consume, seven of which carry no automation
    /// at all — so the automation withdrawal cannot cover them and only this leg can. The failure
    /// it prevents is not a wrong sample during the silence (`peak > limit` is false for a `+0.0`
    /// input whatever `limit` holds, so the rendered block really is `+0.0` either way); it is that
    /// a skipped block advances no ramp, and the ceiling would be stranded part-way to the value
    /// the automation asked for, for as long as the silence lasted. The tone that ends the silence
    /// is then limited to the wrong ceiling.
    ///
    /// That is why `state_bits` carries the ramp words. The rendered samples of the two arms agree
    /// under the mutation; the state does not.
    ///
    /// Red mutation: drop the four `ramps_are_stationary` legs from the admission test in
    /// `process_block`.
    #[test]
    fn a_de_zipper_window_open_across_a_block_boundary_refuses_the_claim() {
        const FRAMES: usize = 8;
        const BLOCKS: usize = 240;
        /// Well after the 486-sample line has drained and the claim is in use.
        const RETARGET_BLOCK: usize = 120;

        fn arm(force_slow: bool) -> (Vec<u32>, Vec<u32>, u32) {
            let values = values_with(-6.0, 100.0, 5.0);
            let mut preparation = request(&values);
            preparation.link_mode = LinkMode::DualMono;
            let metadata =
                expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1, preparation)
                    .expect("metadata");
            let (left_defaults, right_defaults) = initial_defaults(&values).expect("defaults");
            let mut effect = PreparedTruePeakLimiter {
                core: LimiterCore::<f32>::new(
                    metadata,
                    vec![left_defaults].into_boxed_slice(),
                    vec![right_defaults].into_boxed_slice(),
                )
                .expect("core"),
            };
            let mut rendered = Vec::new();
            let mut states = Vec::new();
            for block in 0..BLOCKS {
                // Eight tone blocks, a long silence, then tone again. The tone is under the
                // guarded ceiling on *both* sides of the retarget, so the recursive word never
                // leaves `+0.0` and the fixed point is reachable inside a test-sized run — the
                // same choice, for the same reason, as the settled-silence gate above.
                let quiet = (8..BLOCKS - 40).contains(&block).then_some(0.0_f32);
                let mut left = silence_plane(block, FRAMES, 1, quiet, 0.05, false);
                let mut right = silence_plane(block, FRAMES, 1, quiet, 0.05, true);
                let first_sample = (block * FRAMES) as u64;
                let retarget = [PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: 0,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: -3.0,
                    end_value: -3.0,
                }];
                let spans: &[PreparedAutomationSpan] = if block == RETARGET_BLOCK {
                    &retarget
                } else {
                    &[]
                };
                if force_slow {
                    effect.core.silent_fixed_point = false;
                }
                let report = effect.process(
                    EffectProcessBlock::new(&mut left, &mut right, None, first_sample, spans, 128)
                        .expect("block"),
                );
                assert_eq!(report.invalid_spans, 0, "the retarget must be accepted");
                rendered.extend(left.iter().map(|value| value.to_bits()));
                rendered.extend(right.iter().map(|value| value.to_bits()));
                states.extend(state_bits(&effect.core));
            }
            (rendered, states, effect.core.silent_engagements())
        }

        let (free_rendered, free_states, engagements) = arm(false);
        let (slow_rendered, slow_states, never) = arm(true);
        assert_eq!(never, 0, "the control arm took the fast path");
        assert!(
            engagements > 0,
            "the fast path never engaged, so the test would pass on any refusal"
        );
        assert_eq!(
            free_rendered, slow_rendered,
            "the silent fast path moved a rendered bit across an open de-zipper window"
        );
        assert_eq!(
            free_states, slow_states,
            "the silent fast path stranded a coefficient ramp part-way to its target"
        );
    }

    /// A scalar instance of `LimiterCore<f32>` behind the contract type, for the tests that need
    /// both the public entry points and the private engagement counter.
    fn silent_instance(ceiling: f32, release: f32, lookahead: f32) -> PreparedTruePeakLimiter {
        PreparedTruePeakLimiter {
            core: silent_core::<f32>(ceiling, release, lookahead),
        }
    }

    /// **A restore withdraws the claim, so the payload's delay line is drained and not skipped.**
    ///
    /// Issue #182 S2. This is the leg that makes the withdrawal in `restore_track` load-bearing
    /// rather than merely tidy. An instance that has earned the claim is, by construction, holding
    /// an all-`+0.0` delay line; `commit_lane` then fills that line with a payload's contents,
    /// which this instance never rendered and whose samples have not reached its output yet. Every
    /// other leg of the admission test still passes — the ramps are stationary, the bypass flag has
    /// not moved, and the caller's block is still exactly `+0.0` — so nothing but the withdrawal
    /// stands between the claim and a skipped block that drops the restored signal on the floor.
    ///
    /// It is the same window `silence_shorter_than_the_lookahead_line_still_drains_it` opens at the
    /// compressor, reached from the other side: there the rings fill because the *signal* has not
    /// drained, here because a *restore* refilled them.
    ///
    /// Red mutation: delete `self.silent_fixed_point = false;` from `LimiterCore::restore_track`.
    #[test]
    fn a_restore_withdraws_the_silence_claim() {
        // The donor's delay line is full of signal when it is snapshotted, but its signal is under
        // the guarded ceiling, so its recursive word is at `+0.0` and the *only* thing the
        // receiver has to drain is the line itself. A donor that had been limiting would refuse
        // the claim afterwards on the recursive word instead, which is a different leg's job.
        let values = values_with(-6.0, 100.0, 5.0);
        let mut donor = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let mut noise = Noise(0x0118_2000);
        let mut left: Vec<f32> = (0..1024).map(|_| noise.next() * 0.2).collect();
        let mut right: Vec<f32> = (0..1024).map(|_| noise.next() * 0.2).collect();
        render(donor.as_mut(), &mut left, &mut right, 128);
        let payload = snapshot(donor.as_ref());

        fn arm(payload: &LanePayload, force_slow: bool) -> (Vec<u32>, Vec<u32>, u32, u32) {
            let mut effect = silent_instance(-6.0, 100.0, 5.0);
            let mut rendered = Vec::new();
            let mut states = Vec::new();
            let mut engaged_before_restore = 0;
            // One quiet tone block, then enough silence for the claim to be earned and used.
            for block in 0..32_usize {
                if block == 16 {
                    engaged_before_restore = effect.core.silent_engagements();
                    let sizes = effect.metadata().state_sizes;
                    effect
                        .restore_state_payload(
                            STATE_LAYOUT_VERSION,
                            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes)
                                .expect("sizes"),
                        )
                        .expect("restore");
                }
                let quiet = (block > 0).then_some(0.0_f32);
                let mut left = silence_plane(block, 128, 1, quiet, 0.05, false);
                let mut right = silence_plane(block, 128, 1, quiet, 0.05, true);
                if force_slow {
                    effect.core.silent_fixed_point = false;
                }
                effect.process(
                    EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        (block * 128) as u64,
                        &[],
                        128,
                    )
                    .expect("block"),
                );
                rendered.extend(left.iter().map(|value| value.to_bits()));
                rendered.extend(right.iter().map(|value| value.to_bits()));
                states.extend(state_bits(&effect.core));
            }
            (
                rendered,
                states,
                engaged_before_restore,
                effect.core.silent_engagements(),
            )
        }

        let (free_rendered, free_states, engaged_before, engaged_after) = arm(&payload, false);
        let (slow_rendered, slow_states, _, never) = arm(&payload, true);
        assert_eq!(never, 0, "the control arm took the fast path");
        assert!(
            engaged_before > 0,
            "the claim was never earned before the restore, so the test proves nothing"
        );
        assert!(
            engaged_after > engaged_before,
            "the claim was never re-earned after the restore drained, so the arms could agree by \
             refusing everything"
        );
        assert_eq!(
            free_rendered, slow_rendered,
            "the restored payload's delay line was skipped instead of drained"
        );
        assert_eq!(
            free_states, slow_states,
            "the restored payload left the two arms in different states"
        );
    }

    /// **The bank entry point withdraws the claim on an automated block, exactly as `process` does.**
    ///
    /// Issue #182 S2. `process` and `process_bank` are separate functions carrying the same rule,
    /// which is precisely the shape the #90 audit found six crates diverging in, so the rule is
    /// gated at both. See `automation_withdraws_the_claim_and_the_resident_tap_keeps_up` for why
    /// the withdrawal is defence rather than a hole-plug at this crate.
    ///
    /// Red mutation: delete `if !block.automation.is_empty()` from `process_bank`.
    #[test]
    fn automation_withdraws_the_claim_on_the_bank_path_too() {
        let values = values_with(-6.0, 100.0, 5.0);
        let mut preparation = request(&values);
        preparation.link_mode = LinkMode::DualMono;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1, preparation)
            .expect("metadata");
        let mut bank = PreparedTruePeakLimiterBank::<Simd8> {
            metadata: PreparedBankMetadata {
                width: BankWidth::Eight,
                program_key: metadata.program_key(),
            },
            core: silent_core::<Simd8>(-6.0, 100.0, 5.0),
        };

        let lanes = 8_usize;
        let empty = vec![0_u32; lanes + 1];
        for block in 0..16_usize {
            let quiet = (block > 0).then_some(0.0_f32);
            let mut left = silence_plane(block, 128, lanes, quiet, 0.05, false);
            let mut right = silence_plane(block, 128, lanes, quiet, 0.05, true);
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    128,
                    BankWidth::Eight,
                    (block * 128) as u64,
                    &[],
                    &empty,
                    128,
                )
                .expect("bank block"),
            );
        }
        let engaged = bank.core.silent_engagements();
        assert!(
            engaged > 0,
            "the bank never used the claim, so the rest of this test proves nothing"
        );

        // One more silent block, carrying a point per track that restates the ceiling in force.
        let first_sample = 16 * 128_u64;
        let restated: Vec<PreparedAutomationSpan> = (0..lanes)
            .map(|_| PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: first_sample,
                end_sample: first_sample,
                start_value: -6.0,
                end_value: -6.0,
            })
            .collect();
        let offsets: Vec<u32> = (0..=lanes).map(|track| track as u32).collect();
        let mut left = vec![0.0_f32; 128 * lanes];
        let mut right = vec![0.0_f32; 128 * lanes];
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                128,
                BankWidth::Eight,
                first_sample,
                &restated,
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        assert!(
            report.reports.iter().all(|track| track.invalid_spans == 0),
            "the restated points must be accepted"
        );
        assert_eq!(
            bank.core.silent_engagements(),
            engaged,
            "a bank block carrying automation took the silent fast path"
        );
    }

    /// E9: partition invariance over the gate's block sizes (master plan P1).
    #[test]
    fn partition_invariance_holds_over_block_sizes() {
        let values = values_with(-6.0, 100.0, 5.0);
        let frames = 512_usize;
        let mut noise = Noise(0x7777);
        let source_left: Vec<f32> = (0..frames).map(|_| noise.next() * 4.0).collect();
        let source_right: Vec<f32> = (0..frames).map(|_| noise.next() * 4.0).collect();

        let mut reference = TruePeakLimiterFactory
            .prepare(request_at(&values, 48_000, 512))
            .expect("prepare");
        let mut left = source_left.clone();
        let mut right = source_right.clone();
        render(reference.as_mut(), &mut left, &mut right, 512);
        let reference_state = snapshot(reference.as_ref());

        for block in [1_usize, 7, 64, 128, 512] {
            let mut effect = TruePeakLimiterFactory
                .prepare(request_at(&values, 48_000, 512))
                .expect("prepare");
            let mut partitioned_left = source_left.clone();
            let mut partitioned_right = source_right.clone();
            render(
                effect.as_mut(),
                &mut partitioned_left,
                &mut partitioned_right,
                block,
            );
            for frame in 0..frames {
                assert_eq!(
                    partitioned_left[frame].to_bits(),
                    left[frame].to_bits(),
                    "block {block} left frame {frame}"
                );
                assert_eq!(
                    partitioned_right[frame].to_bits(),
                    right[frame].to_bits(),
                    "block {block} right frame {frame}"
                );
            }
            assert_eq!(snapshot(effect.as_ref()), reference_state, "block {block}");
        }
    }

    /// E10: the D7 boundary check replaces every per-value recovery path.
    #[test]
    fn a_nonfinite_block_is_zeroed_reset_and_counted() {
        let values = values_with(-6.0, 100.0, 5.0);
        let mut preparation = request(&values);
        preparation.link_mode = LinkMode::DualMono;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1, preparation)
            .expect("metadata");
        let (left_defaults, right_defaults) = initial_defaults(&values).expect("defaults");
        let mut core = LimiterCore::<f32>::new(
            metadata,
            vec![left_defaults].into_boxed_slice(),
            vec![right_defaults].into_boxed_slice(),
        )
        .expect("core");
        let mut left = vec![0.5_f32; 128];
        let mut right = vec![0.5_f32; 128];
        core.process_block(&mut left, &mut right, 128);
        assert_eq!(core.nonfinite_report().nonfinite_blocks, 0);

        core.left.reduction[0] = f32::NAN;
        let mut left = vec![0.5_f32; 128];
        let mut right = vec![0.5_f32; 128];
        core.process_block(&mut left, &mut right, 128);
        assert!(left.iter().all(|sample| sample.to_bits() == 0));
        assert!(right.iter().all(|sample| sample.to_bits() == 0));
        assert_eq!(core.nonfinite_report().nonfinite_blocks, 1);
        assert_eq!(core.nonfinite_report().nonfinite_lanes, 1);
        assert_eq!(core.cursors, Cursors::default());
        assert_eq!(core.left.reduction[0].to_bits(), 0);
        assert!(core.left.required_ring.iter().all(|value| *value == 1.0));
        assert_eq!(core.left.box_sum[0], core.left.lane[0].window as f32);
    }

    /// E11: layout 2 round-trips, and every corruption is rejected without mutating the peer.
    #[test]
    fn state_v2_round_trips_and_rejects_corruption() {
        let values = values_with(-6.0, 100.0, 5.0);
        let mut source = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let mut peer = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let mut noise = Noise(0x2222);
        let source_left: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        let source_right: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        let mut left = source_left.clone();
        let mut right = source_right.clone();
        render(source.as_mut(), &mut left, &mut right, 128);
        let mut left = source_left.clone();
        let mut right = source_right.clone();
        render(peer.as_mut(), &mut left, &mut right, 128);

        let payload = snapshot(source.as_ref());
        let before = snapshot(peer.as_ref());
        assert_eq!(payload, before);
        peer.restore_state_payload(
            2,
            StatePayloadInput::new(
                &payload.0,
                &payload.1,
                &payload.2,
                peer.metadata().state_sizes,
            )
            .expect("sizes"),
        )
        .expect("restore");
        assert_eq!(snapshot(peer.as_ref()), payload);

        // Restoring into a fresh instance reproduces the source's continuation bit for bit.
        let mut fresh = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        fresh
            .restore_state_payload(
                2,
                StatePayloadInput::new(
                    &payload.0,
                    &payload.1,
                    &payload.2,
                    fresh.metadata().state_sizes,
                )
                .expect("sizes"),
            )
            .expect("restore into fresh");
        let mut noise = Noise(0x3333);
        let tail_left: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        let tail_right: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        let mut source_out = (tail_left.clone(), tail_right.clone());
        render(source.as_mut(), &mut source_out.0, &mut source_out.1, 128);
        let mut fresh_out = (tail_left.clone(), tail_right.clone());
        render(fresh.as_mut(), &mut fresh_out.0, &mut fresh_out.1, 128);
        for frame in 0..512 {
            assert_eq!(
                fresh_out.0[frame].to_bits(),
                source_out.0[frame].to_bits(),
                "restored continuation at {frame}"
            );
        }

        let sizes = peer.metadata().state_sizes;
        let reference = snapshot(peer.as_ref());
        type Corruption = (&'static str, Box<dyn Fn(&mut Vec<u8>)>);
        let corruptions: [Corruption; 6] = [
            (
                "version",
                Box::new(|bytes: &mut Vec<u8>| write_u32(bytes, 0, 1)),
            ),
            (
                "box sum off the grid",
                Box::new(|bytes: &mut Vec<u8>| {
                    let sum = read_f32(bytes, words::BOX_SUM);
                    write_f32(bytes, words::BOX_SUM, sum + 1.0 / BOX_GRID);
                }),
            ),
            (
                "phase at the window length",
                Box::new(|bytes: &mut Vec<u8>| write_u32(bytes, words::PHASE, 241)),
            ),
            (
                "ring cursor out of range",
                Box::new(|bytes: &mut Vec<u8>| write_u32(bytes, words::RING_CURSOR, 481)),
            ),
            (
                "negative-zero lookahead",
                Box::new(|bytes: &mut Vec<u8>| {
                    write_f32(bytes, words::LOOKAHEAD, -0.0);
                }),
            ),
            (
                "ramp remaining past the window",
                Box::new(|bytes: &mut Vec<u8>| {
                    write_u32(bytes, words::LIMIT_RAMP + 3, 65);
                }),
            ),
        ];
        for (name, corrupt) in corruptions {
            let mut common = reference.0.clone();
            let mut left = reference.1.clone();
            let right = reference.2.clone();
            if name == "version" {
                corrupt(&mut common);
            } else {
                corrupt(&mut left);
            }
            let result = peer.restore_state_payload(
                STATE_LAYOUT_VERSION,
                StatePayloadInput::new(&common, &left, &right, sizes).expect("sizes"),
            );
            assert!(result.is_err(), "{name} was accepted");
            assert_eq!(
                snapshot(peer.as_ref()),
                reference,
                "{name} mutated the peer"
            );
        }

        // The declared version argument is checked before anything else.
        assert!(
            peer.restore_state_payload(
                1,
                StatePayloadInput::new(&reference.0, &reference.1, &reference.2, sizes)
                    .expect("sizes")
            )
            .is_err()
        );
        // A one-byte-short section is rejected.
        let short = vec![0_u8; sizes.left_bytes as usize - 4];
        assert!(StatePayloadInput::new(&reference.0, &short, &reference.2, sizes).is_err());
    }

    #[test]
    fn bank_binding_validates_before_fallback_and_retains_exact_width_bytes() {
        let values = initial_values();
        let members: Vec<_> = (0..8).map(|_| values).collect();
        let bank = bank_for(
            &members,
            LinkMode::DualMono,
            BankWidth::Eight,
            Backend::Simd8,
        );
        let key = bank.metadata().program_key;
        assert_eq!(key.state_layout_version, 2);
        assert_eq!(key.state_sizes.left_bytes, 5_900);
        assert_eq!(key.state_sizes.common_bytes, 8);
        assert_eq!(key.state_sizes.total(), Some(11_808));
        assert_eq!(bank.metadata().width, BankWidth::Eight);

        // Mismatched backend and width are rejected before anything is prepared.
        let requests: Vec<_> = members.iter().map(|values| request(values)).collect();
        let mismatched = TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd4,
            width: BankWidth::Eight,
            requests: &requests,
        });
        assert_eq!(
            mismatched.err().map(|error| error.code),
            Some("effect.bank.requests")
        );

        // Issue #95 unification: a heterogeneous cohort is a cohort this artifact cannot bank,
        // not a malformed request. Every member is still validated first — the `Ok` proves the
        // decline happened after validation, not instead of it — and the answer is the same
        // `Ok(None)` every other effect gives, so the tracks render as scalar instances instead
        // of failing the session compile.
        let mut heterogeneous: Vec<PrepareEffectRequest<'_>> = requests.clone();
        heterogeneous[3].link_mode = LinkMode::Maximum;
        let heterogeneous =
            TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::Simd8,
                width: BankWidth::Eight,
                requests: &heterogeneous,
            });
        assert!(
            heterogeneous
                .expect("a heterogeneous cohort is declined, never an error")
                .is_none()
        );

        // A member that would fail `prepare` on its own is still a typed error, and it is the
        // diagnostic `prepare` would have returned — an absent capability must never hide it.
        let mut malformed: Vec<PrepareEffectRequest<'_>> = requests.clone();
        malformed[5].quality = EffectQuality::Draft;
        assert_eq!(
            TruePeakLimiterFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: Backend::Simd8,
                    width: BankWidth::Eight,
                    requests: &malformed,
                })
                .err()
                .map(|error| error.code),
            Some("effect.quality.unsupported")
        );
    }

    #[test]
    fn automation_retargets_linear_coefficients_and_counts_invalid_spans() {
        let values = initial_values();
        let mut effect = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let spans = [
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 0,
                start_sample: 0,
                end_sample: 0,
                start_value: -6.0,
                end_value: -6.0,
            },
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Both,
                parameter_index: 1,
                start_sample: 0,
                end_sample: 0,
                start_value: 500.0,
                end_value: 500.0,
            },
            PreparedAutomationSpan {
                kind: AutomationSpanKind::Point,
                channel: ParameterChannel::Left,
                parameter_index: 2,
                start_sample: 0,
                end_sample: 0,
                start_value: 1.0,
                end_value: 1.0,
            },
        ];
        let mut left = vec![0.0_f32; 8];
        let mut right = vec![0.0_f32; 8];
        let report = effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &spans, 128).expect("block"),
        );
        assert_eq!(report.invalid_spans, 2);
        let payload = snapshot(effect.as_ref());
        // Eight of the sixty-four updates have been produced, so the ramp is in flight toward the
        // linear limit of -6 dB and the lookahead word is untouched.
        assert_eq!(read_u32(&payload.1, words::LIMIT_RAMP + 3), 64 - 8);
        assert_eq!(
            read_f32(&payload.1, words::LIMIT_RAMP + 1).to_bits(),
            limit_coefficient(-6.0).to_bits()
        );
        assert_eq!(read_f32(&payload.1, words::LOOKAHEAD), 5.0);
        assert_eq!(read_u32(&payload.2, words::LIMIT_RAMP + 3), 0);
    }

    #[test]
    fn both_resets_return_the_runtime_state_to_a_silent_lane() {
        let values = values_with(-6.0, 100.0, 5.0);
        let mut effect = TruePeakLimiterFactory
            .prepare(request(&values))
            .expect("prepare");
        let fresh = snapshot(effect.as_ref());
        let mut noise = Noise(0x4444);
        let mut left: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        let mut right: Vec<f32> = (0..512).map(|_| noise.next() * 4.0).collect();
        render(effect.as_mut(), &mut left, &mut right, 128);
        assert_ne!(snapshot(effect.as_ref()), fresh);
        effect.reset(ResetKind::FullToDefaults);
        assert_eq!(snapshot(effect.as_ref()), fresh);

        let spans = [PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel: ParameterChannel::Left,
            parameter_index: 0,
            start_sample: 0,
            end_sample: 0,
            start_value: -3.0,
            end_value: -3.0,
        }];
        let mut left = vec![0.0_f32; 8];
        let mut right = vec![0.0_f32; 8];
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &spans, 128).expect("block"),
        );
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        let payload = snapshot(effect.as_ref());
        assert_eq!(read_u32(&payload.1, words::LIMIT_RAMP + 3), 0);
        assert_eq!(
            read_f32(&payload.1, words::LIMIT_RAMP).to_bits(),
            limit_coefficient(-3.0).to_bits()
        );
        assert_eq!(read_u32(&payload.1, words::MAIN_CURSOR), 0);
    }
}
