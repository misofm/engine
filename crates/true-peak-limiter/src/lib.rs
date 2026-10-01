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
//! `f64` by `math` at event time and ramped in the **linear** domain, so a coefficient
//! is never a transcendental of a per-sample value. No `%` on a cursor (#90 F6): rings are advanced
//! with a compare and a wrap. No per-value `is_finite`/`is_subnormal`/`Option` plumbing and no
//! per-sample recovery (#90 F5, decision D7): the only flush is on the single recursive word `d`,
//! and the only failure path is the once-per-block boundary check of `effect-runtime`. Its
//! recovery is attributed to the lanes that failed, so a bank-mate's bits never depend on another
//! lane's failure (#1091, decision 12). No second copy of the ramp, the payload codec or the
//! parameter validator (#90 F9): they come from `effect-runtime`.
#![allow(missing_docs)]

#[cfg(test)]
use core::cell::Cell;

pub mod corpus;

use effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory, ObservationCadence,
    ObservationChannels, ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind,
    ObservationSample, ObservationTapId, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule, StatePayloadError,
    StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use effect_runtime::bank::{
    NonFiniteReport, block_is_positive_zero, check_block, nonfinite_lane_mask,
};
use effect_runtime::params::{
    ParameterSpec, is_negative_zero, normalize_zero, parameter_value_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::{
    HEADER_WORDS, StateLayout, read_f32, read_header, read_u32, validate_lengths, write_f32,
    write_header, write_u32,
};
use lane::{Backend, Lane, flush};

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
        lattice: effect_contract::default_parameter_lattice(
            unit,
            ParameterDomain::Continuous,
            mapping,
        ),
    }
}

/// Frozen descriptor rows. Descriptor position and stable numeric ID agree.
pub const TRUE_PEAK_LIMITER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
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
/// version/length header `effect-runtime` stamps into every payload, which is why
/// `common_bytes` is eight and no longer zero. The latency column does not move.
const fn quality(rate: u32) -> effect_contract::QualityDescriptor {
    let lookahead_maximum = rate / 100;
    let lane_words = 3 * lookahead_maximum + 35;
    let lane_bytes = lane_words * 4;
    effect_contract::QualityDescriptor {
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

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
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
pub const TRUE_PEAK_LIMITER_OBSERVATIONS: [ObservationDescriptor; 1] = [ObservationDescriptor {
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
pub const TRUE_PEAK_LIMITER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
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
    parameters: &TRUE_PEAK_LIMITER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &TRUE_PEAK_LIMITER_OBSERVATIONS,
};

/// The state layout this crate reads and writes.
///
/// Bumped from 1 by #90: the wave-2 gain law needs a minimum-filter phase and prefix, an exact box
/// sum, a box ring and a precomputed ramp step, none of which layout 1 can hold, and the payload
/// gained the runtime's two-word header. This is the one contract fixture the issue authorises
/// moving; the latency, the parameter table, the port table, the link set, the Annex-2 coefficients
/// and `scratch_fixed_bytes` are unchanged.
pub const STATE_LAYOUT_VERSION: u32 = 1;

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
    /// `crates/true-peak-limiter/tests/mono_collapse.rs` fails if any of `history`,
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
            self.seed_lane_defaults(lane, shape, values, rate);
        }
        self.clear_runtime(shape);
    }

    /// Writes one lane's designed words from its prepared defaults: the per-lane half of
    /// [`reset_to_defaults`](Self::reset_to_defaults), which the per-lane D7 recovery shares
    /// (issue #1091).
    fn seed_lane_defaults(
        &mut self,
        lane: usize,
        shape: &Shape,
        values: &[f32; PARAMETER_COUNT],
        rate: u32,
    ) {
        self.lookahead_ms[lane] = values[2];
        self.lane[lane] = LaneShape::new(lookahead_samples(values[2], rate, shape.n), shape);
        self.limit[lane] = LinearRamp::fixed(limit_coefficient(values[0]));
        self.release[lane] = LinearRamp::fixed(release_coefficient(values[1], rate));
    }

    /// [`reset_to_defaults`](Self::reset_to_defaults) for lane `lane` alone: the D7 recovery of
    /// one lane of a bank (issue #1091). Every other lane's words are untouched.
    fn reset_lane_to_defaults(
        &mut self,
        lane: usize,
        shape: &Shape,
        values: &[f32; PARAMETER_COUNT],
        rate: u32,
    ) {
        self.seed_lane_defaults(lane, shape, values, rate);
        self.clear_lane_runtime(lane, shape);
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
    ///
    /// Out of line (#1091): it runs at preparation, at a reset and on a failed block, never in the
    /// frame loop, and each of its three `1.0` fills lowers to a `memset_pattern16` call on Apple
    /// targets (#1018). One copy keeps that count where it was when the per-lane recovery moved
    /// the call sites.
    #[inline(never)]
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

    /// [`clear_runtime`](Self::clear_runtime) for lane `lane` alone, word for word: the same list,
    /// written at that lane's stride of every ring (issue #1091).
    ///
    /// The shared cursors are not reset, and need not be. Every word this writes is uniform along
    /// its ring (`+0.0` in the history and the main line, `1.0` in both gain rings), and the kernel
    /// addresses a ring only at an offset from a cursor, so a cleared lane reads what a cleared lane
    /// at cursor zero reads. That is the rotation [`commit_lane`] performs for a payload, and it is
    /// why the lane then renders what a scalar instance renders after its own §4.4 reset.
    /// `a_lane_reset_is_the_whole_reset_at_one_lanes_stride` keeps the list equal to
    /// [`clear_runtime`](Self::clear_runtime)'s.
    fn clear_lane_runtime(&mut self, lane: usize, shape: &Shape) {
        let width = self.width;
        debug_assert!(lane < width);
        debug_assert_eq!(self.main_ring.len(), shape.main * width);
        for word in self.history.iter_mut().skip(lane).step_by(width) {
            *word = 0.0;
        }
        for word in self.main_ring.iter_mut().skip(lane).step_by(width) {
            *word = 0.0;
        }
        for word in self.required_ring.iter_mut().skip(lane).step_by(width) {
            *word = 1.0;
        }
        for word in self.box_ring.iter_mut().skip(lane).step_by(width) {
            *word = 1.0;
        }
        self.reduction[lane] = 0.0;
        self.prefix[lane] = 1.0;
        self.phase[lane] = 0;
        self.box_sum[lane] = self.lane[lane].window as f32;
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

/// The lane mask naming every lane of a `width`-lane instance or bank.
fn every_lane(width: usize) -> u32 {
    debug_assert!((1..=MAXIMUM_WIDTH).contains(&width));
    (1_u32 << width) - 1
}

/// Zeroes the words of every lane named in `lanes`, in every frame of an AoSoA block.
///
/// The per-lane half of the §4.4 recovery (issue #1091), so it runs on the failing path only,
/// never on a block that passed the check.
fn zero_lanes<L: Lane>(io: &mut [f32], lanes: u32) {
    for frame in io.chunks_exact_mut(L::WIDTH) {
        for (lane, word) in frame.iter_mut().enumerate() {
            if lanes & (1 << lane) != 0 {
                *word = 0.0;
            }
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

/// `true` when two ramps hold the same four words, by bit pattern.
///
/// All four, because [`RampLanes::advance`] reads all four: two ramps that agree on `current` and
/// disagree on `step` produce the same value on this sample and different ones on the next.
fn ramp_words_agree(left: &LinearRamp, right: &LinearRamp) -> bool {
    left.current.to_bits() == right.current.to_bits()
        && left.target.to_bits() == right.target.to_bits()
        && left.step.to_bits() == right.step.to_bits()
        && left.remaining == right.remaining
}

/// Issue #990 contract 1: `true` when, on every lane, the two channels carry the same designed
/// words the gain path reads.
///
/// Those are the window shape (`LaneShape`: the van Herk window, the ring distance to its newest
/// sample and the box term leaving the running sum) and all four words of each of the two ramps
/// (the `limit` and `release` coefficients). Nothing else designed reaches steps 1-6 of
/// [`channel_frame_uniform`]: `lookahead_ms` is what `LaneShape` was derived from and is never read
/// by a rendered block, and the link and bypass booleans are one per bank.
///
/// Bit compares throughout, never tolerances: the licence this grants is "the two gain paths run
/// the same operations on the same operands", which a tolerance cannot state. About ninety word
/// compares at `W = 8`, once per block, on the control side of the frame loop.
fn designed_gain_agree(left: &ChannelState, right: &ChannelState) -> bool {
    left.lane.iter().zip(right.lane.iter()).all(|(a, b)| a == b)
        && left
            .limit
            .iter()
            .zip(right.limit.iter())
            .all(|(a, b)| ramp_words_agree(a, b))
        && left
            .release
            .iter()
            .zip(right.release.iter())
            .all(|(a, b)| ramp_words_agree(a, b))
}

/// `true` when every lane's window shape agrees across the two channels.
///
/// After [`ChannelState::clear_runtime`] this is exactly "the two channels' gain words agree":
/// `clear_runtime` writes `1.0` over both gain rings and the prefix, `+0.0` over the reduction word,
/// `0` over the phase, and `Wb` over the box sum. The last is the only word that depends on the
/// lane, and it depends on nothing but the lane's window, so the words agree if and only if every
/// lane's `LaneShape` does. That is what lets a reset re-establish the linked-agreement record in
/// `W` compares instead of a full comparison.
fn lane_shapes_agree(left: &ChannelState, right: &ChannelState) -> bool {
    left.lane.iter().zip(right.lane.iter()).all(|(a, b)| a == b)
}

/// `true` when the two channels' gain words are bit-equal on every lane.
///
/// The gain words are the running state of steps 1-6 of [`channel_frame_uniform`]: both gain rings
/// (every slot, not only the live window -- a slot outside it is still serialised), the van Herk
/// `prefix` and `phase`, the running box sum and the recursive reduction word. The detector history
/// and the main delay ring are per channel by design and are not in the list.
///
/// A full comparison, so it runs off the frame loop only: after a restore, where the payload's two
/// sections are arbitrary, and after a per-lane §4.4 recovery (#1091), which runs only on a block
/// that failed the boundary check. Bounded by the rings and allocation-free either way.
fn gain_state_agrees(left: &ChannelState, right: &ChannelState) -> bool {
    let words_agree = |a: &[f32], b: &[f32]| {
        a.iter()
            .zip(b.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits())
    };
    words_agree(&left.required_ring, &right.required_ring)
        && words_agree(&left.box_ring, &right.box_ring)
        && words_agree(&left.prefix, &right.prefix)
        && words_agree(&left.box_sum, &right.box_sum)
        && words_agree(&left.reduction, &right.reduction)
        && left.phase == right.phase
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StationaryDispatch {
    Runtime,
    Stationary,
    Ramping,
}

const DISPATCH_RUNTIME: u8 = 0;
const DISPATCH_STATIONARY: u8 = 1;
const DISPATCH_RAMPING: u8 = 2;

#[inline(always)]
fn dual_stationary(left: &ChannelState, right: &ChannelState) -> bool {
    ramps_are_stationary(&left.limit)
        && ramps_are_stationary(&left.release)
        && ramps_are_stationary(&right.limit)
        && ramps_are_stationary(&right.release)
}

#[inline(always)]
fn mono_stationary(left: &ChannelState) -> bool {
    ramps_are_stationary(&left.limit) && ramps_are_stationary(&left.release)
}

#[inline(always)]
fn ramp_values<const DISPATCH: u8, L: Lane>(
    stationary: bool,
    limit: &mut RampLanes<L>,
    release: &mut RampLanes<L>,
) -> (L, L) {
    match DISPATCH {
        DISPATCH_RUNTIME => {
            if stationary {
                (limit.resting_value(), release.resting_value())
            } else {
                (limit.advance(), release.advance())
            }
        }
        DISPATCH_STATIONARY => (limit.resting_value(), release.resting_value()),
        DISPATCH_RAMPING => (limit.advance(), release.advance()),
        _ => unreachable!("invalid limiter dispatch"),
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DispatchRoute {
    Unset,
    DualPerLane,
    DualUniform,
    MonoPerLane,
    MonoUniform,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DispatchObservation {
    route: DispatchRoute,
    mode: StationaryDispatch,
}

#[cfg(test)]
thread_local! {
    static DISPATCH_OBSERVATION: Cell<DispatchObservation> = const {
        Cell::new(DispatchObservation {
            route: DispatchRoute::Unset,
            mode: StationaryDispatch::Runtime,
        })
    };
}

#[cfg(test)]
fn clear_dispatch_observation() {
    DISPATCH_OBSERVATION.with(|observation| {
        observation.set(DispatchObservation {
            route: DispatchRoute::Unset,
            mode: StationaryDispatch::Runtime,
        });
    });
}

#[cfg(test)]
fn observe_dispatch<const DISPATCH: u8>(route: DispatchRoute) {
    DISPATCH_OBSERVATION.with(|observation| {
        observation.set(DispatchObservation {
            route,
            mode: match DISPATCH {
                DISPATCH_RUNTIME => StationaryDispatch::Runtime,
                DISPATCH_STATIONARY => StationaryDispatch::Stationary,
                DISPATCH_RAMPING => StationaryDispatch::Ramping,
                _ => unreachable!("invalid limiter dispatch"),
            },
        });
    });
}

#[cfg(test)]
fn dispatch_observation() -> DispatchObservation {
    DISPATCH_OBSERVATION.with(Cell::get)
}

#[cfg(test)]
thread_local! {
    /// Issue #990 gate 2: uniform blocks that rendered a linked pair's gain path once, on this
    /// thread. Instrumentation, not render state.
    static LINKED_ENGAGEMENTS: Cell<u32> = const { Cell::new(0) };
    /// Issue #990 gate 1: route [`limiter_block_linkable`] to the unmodified reference kernel the
    /// test module keeps, for the oracle arm of the identity harness.
    static REFERENCE_KERNEL: Cell<bool> = const { Cell::new(false) };
}

/// Issue #1014 gate 2: what the stationary walk's segments were, on this thread.
///
/// Instrumentation for the identity tests' coverage checks, not render state. Counted at segment
/// entry, from the words the walk decides on.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SegmentCensus {
    /// Segments the stationary walk rendered.
    segments: u64,
    /// Segments whose last frame completes a van Herk block (either channel's, in the dual body).
    completions: u64,
    /// ... of a one-frame segment: the completion is on the segment's first frame.
    completion_first_frame: u64,
    /// ... of a segment of two or more frames: the completion is on its last frame.
    completion_last_frame: u64,
    /// ... that also ends the chunk (or the block).
    completion_chunk_boundary: u64,
    /// ... that also ends where a ring index wraps.
    completion_ring_wrap: u64,
    /// Dual segments cut by the right channel's completion while the left's is later.
    right_cut: u64,
    /// Dual segments whose two channels have different windows.
    asymmetric: u64,
    /// Linked segments by steady-frame count: 0, 1, 2, 3 and 31.
    linked_steady: [u64; 5],
    /// Linked runs that took `linked_steady_streams`.
    streams: u64,
    /// ... whose newest stream wrapped (`e < c`).
    streams_newest_wrapped: u64,
    /// ... whose expiring stream wrapped (`x < c`).
    streams_expiring_wrapped: u64,
    /// ... with neither wrapped.
    streams_unwrapped: u64,
    /// Linked runs of one or more steady frames that took the checked loop instead.
    fallback: u64,
    /// ... with two or more steady frames, refused by `Wb == R` or `steady > R - Wb`.
    fallback_bound: u64,
}

#[cfg(test)]
thread_local! {
    static SEGMENT_CENSUS: Cell<SegmentCensus> = const {
        Cell::new(SegmentCensus {
            segments: 0,
            completions: 0,
            completion_first_frame: 0,
            completion_last_frame: 0,
            completion_chunk_boundary: 0,
            completion_ring_wrap: 0,
            right_cut: 0,
            asymmetric: 0,
            linked_steady: [0; 5],
            streams: 0,
            streams_newest_wrapped: 0,
            streams_expiring_wrapped: 0,
            streams_unwrapped: 0,
            fallback: 0,
            fallback_bound: 0,
        })
    };
}

#[cfg(test)]
fn update_census(update: impl FnOnce(&mut SegmentCensus)) {
    SEGMENT_CENSUS.with(|census| {
        let mut value = census.get();
        update(&mut value);
        census.set(value);
    });
}

/// This thread's census so far.
#[cfg(test)]
fn peek_census() -> SegmentCensus {
    SEGMENT_CENSUS.with(Cell::get)
}

/// Returns this thread's census and clears it.
#[cfg(test)]
fn take_census() -> SegmentCensus {
    SEGMENT_CENSUS.with(|census| census.replace(SegmentCensus::default()))
}

/// One segment of the stationary walk, at its entry. `wrap_run` is [`segment`]'s run and
/// `remaining` the frames left in the chunk; `(phase, window)` per channel are read before the
/// segment advances them.
#[cfg(test)]
fn census_segment(
    linked: bool,
    run: usize,
    wrap_run: usize,
    remaining: usize,
    steady: usize,
    left: (usize, usize),
    right: (usize, usize),
) {
    let left_completes = left.0 + run == left.1;
    let right_completes = !linked && right.0 + run == right.1;
    let completes = left_completes || right_completes;
    update_census(|census| {
        census.segments += 1;
        if completes {
            census.completions += 1;
            if run == 1 {
                census.completion_first_frame += 1;
            } else {
                census.completion_last_frame += 1;
            }
            if run == remaining {
                census.completion_chunk_boundary += 1;
            }
            if run == wrap_run && wrap_run < remaining {
                census.completion_ring_wrap += 1;
            }
        }
        if right_completes && !left_completes {
            census.right_cut += 1;
        }
        if !linked && left.1 != right.1 {
            census.asymmetric += 1;
        }
        if let Some(bucket) = [0, 1, 2, 3, 31]
            .iter()
            .position(|count| linked && *count == steady)
        {
            census.linked_steady[bucket] += 1;
        }
    });
}

/// One linked run's pass-1 choice: the streams, or the checked loop.
#[cfg(test)]
fn census_streams(streams: bool, steady: usize, slots: FrameSlots) {
    update_census(|census| {
        if streams {
            census.streams += 1;
            let cursor = slots.ring_cursor;
            if slots.end < cursor {
                census.streams_newest_wrapped += 1;
            }
            if slots.expiring < cursor {
                census.streams_expiring_wrapped += 1;
            }
            if slots.end > cursor && slots.expiring > cursor {
                census.streams_unwrapped += 1;
            }
        } else if steady >= 1 {
            census.fallback += 1;
            if steady >= 2 {
                census.fallback_bound += 1;
            }
        }
    });
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
/// this job produces and belongs to #49 (#90 F7). Evaluated through `math` so the bits
/// are the same on every target (D6); the render path never sees a transcendental.
fn limit_coefficient(ceiling_db: f32) -> f32 {
    math::db_to_gain(f64::from(ceiling_db) - 1.0) as f32
}

/// `1 - exp(-1 / (0.001 * release_ms * Fs))`, designed in `f64` and rounded once to `f32`.
///
/// The rate form, not the pole: `d += c * (target - d)` moves toward the target at this rate.
fn release_coefficient(release_ms: f32, sample_rate: u32) -> f32 {
    (1.0 - math::exp(-1.0 / (0.001 * f64::from(release_ms) * f64::from(sample_rate)))) as f32
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
/// the same table rows in the same tap-major order with the same separately rounded
/// `add(mul(..))` steps per accumulator (twelve then; eleven since #1013 dropped the `+0.0` seed).
/// The native target reaches the same code either way once SROA has promoted the array, which is
/// why this is a wasm change with a native no-op attached.
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
/// The FIR is tap-major and lockstep across lanes: for each phase the accumulator starts at its
/// first product and takes the eleven remaining separately rounded `add(mul(...))` steps in
/// increasing tap order. That is the frozen order of the 016 brief without its leading `+0.0 +`,
/// and every peak word it produces is the one the brief's order produces (#90 F4 for the order,
/// #1013 for the seed; the proof is on [`annex2_phases`]). No fusion, no reassociation, no
/// horizontal work.
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
/// Each accumulator starts at its first product, `fir[0][p] * t0`, and takes the eleven remaining
/// separately rounded `add(mul(..))` steps in increasing tap order. Walking taps on the outside and
/// phases on the inside reads the table in its stored order and keeps each lane's summation order
/// the one the 016 brief froze, which is why the tap-major reorder is bit-preserving (#90 F4).
///
/// The steps are written out rather than iterated (round 2 R2). The order is the loop's, tap for
/// tap and phase for phase; what the unrolling buys is that each table row is read as a single-use
/// load feeding its multiply — the wasm backend sinks such a load into its consumer, where a
/// hoisted row would have had to be kept live — and that the four accumulators are four values
/// rather than an array a backend might decide to spill.
///
/// # No `+0.0` seed (#1013)
///
/// The brief starts each accumulator at `+0.0` and adds all twelve products; that form is kept as
/// the test oracle `annex2_phases_seeded`. Starting at the first product instead takes one add
/// off each of the four dependent chains a frame carries, and it is class A. Write `a_k` for the
/// `k`-th product of one lane and phase, `S_k` for the seeded accumulator after tap `k` and `T_k`
/// for this one, rounding to nearest throughout.
///
/// * **Tap 0.** `S_0 = +0.0 + a_0` and `T_0 = a_0`. For every `a_0` but `-0.0` they are the same
///   word, or both NaN when `a_0` is (`+0.0 + a` is `a` exactly for a nonzero `a`, `+0.0 + +0.0`
///   is `+0.0`). For `a_0 = -0.0`, `S_0` is `+0.0` and `T_0` is `-0.0`.
/// * **Taps 1 to 11, by induction.** If `S_{k-1}` and `T_{k-1}` are the same word, or both NaN,
///   so are `S_k` and `T_k`: the same add of the same operands, or an add of a NaN. If they are
///   zeros of opposite sign, then `x + a_k` is the same word for either zero `x` when `a_k` is
///   nonzero (it is `a_k`) or NaN, and a zero for either when `a_k` is a zero. So each phase is
///   the seeded phase, or that phase with the sign of a zero flipped, or NaN where it is NaN.
/// * **The one reader.** [`detector_peak`] reads a phase only as `peak.max(phase.abs())`. `abs`
///   erases a zero's sign, and `max` selects on an ordered compare, which reads a NaN of any
///   payload as unordered. So every peak word is the seeded form's, except that a NaN peak may
///   carry another payload.
/// * **A NaN peak is inert.** A peak goes only to the stack scratch, through the link's `max`
///   (again a compare-and-select), to `select(p > l, l / p, 1)`: a NaN `p` fails the ordered
///   compare and gives exactly `1.0`, whatever its payload. The base form never fixed the payload
///   either: `fadd` commutes, so the backend may emit either operand order, x86 returns the first
///   NaN operand's payload, and wasm leaves NaN payloads nondeterministic outright.
///
/// Hence every output word, state payload, report and observation is unchanged at every width and
/// on every target; only a phase value, which is not state, may differ in the sign of a zero. Seeding
/// with `-0.0`, the additive identity, is the same computation as this one; seeding with `+0.0` is
/// not, and that is the whole of the difference.
#[inline(always)]
fn annex2_phases<L: Lane>(history: &History<L>, fir: &[[L; 4]; HISTORY_WORDS]) -> [L; 4] {
    let row = &fir[0];
    let sample = history.t0;
    let mut phase0 = row[0].mul(sample);
    let mut phase1 = row[1].mul(sample);
    let mut phase2 = row[2].mul(sample);
    let mut phase3 = row[3].mul(sample);
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

/// The brief's order as it stood before #1013: each accumulator starts at exactly `+0.0` and takes
/// all twelve `add(mul(..))` steps.
///
/// Test-only. It is the statement of the frozen order that E1 checks against the scalar loop typed
/// from the brief, and the oracle E1b checks [`annex2_phases`]'s peaks against.
#[cfg(test)]
#[inline(always)]
fn annex2_phases_seeded<L: Lane>(history: &History<L>, fir: &[[L; 4]; HISTORY_WORDS]) -> [L; 4] {
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

/// Issue #990 contract 2: [`sliding_minimum_uniform`] with every write of its backward pass also
/// made to `mirror`, at the same slot.
///
/// The body is [`sliding_minimum_uniform`]'s, line for line and operand for operand; the only
/// addition is the second `store_ring_lane`. Every read comes from `required_ring`. It is a
/// separate function, not a parameter of the first, so the dual body keeps the code it had.
///
/// The mirror is the right channel of a linked pair whose required-gain rings are bit-equal on
/// entry (the linked-agreement invariant). The dual body would have run this same pass on the
/// right ring and written the same words to the same slots, so writing them here leaves the right
/// ring exactly as a dual block would: the saving is the recomputation, never the store.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn sliding_minimum_uniform_mirrored<L: Lane>(
    required_ring: &mut [f32],
    mirror: &mut [f32],
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
            store_ring_lane::<L>(mirror, slot, suffix);
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

/// Issue #990 contract 3: one frame of a linked pair, steps 1-6 once and step 7 on each channel.
///
/// # When this is the frame the dual body renders
///
/// Under `LinkMode::Maximum` both channels feed the same linked peak `max(p_R, p_L)` to their gain
/// computers. When, lane by lane, the two channels also carry the same designed words
/// ([`designed_gain_agree`]) and the same gain words (the `gain_linked` record on
/// [`LimiterCore`]), the right channel's steps 1-6 of [`channel_frame_uniform`] are the left's:
/// the same operations, in the same order, on the same operands. So they produce the same words,
/// including a `-0.0`, a subnormal and a NaN payload, and computing them once is class A.
///
/// Steps 1-6 run here on the left channel's hot words in [`channel_frame_uniform`]'s order. Every
/// ring write they make is made to the right channel as well, at the same slot: the required gain
/// at the cursor, each suffix minimum of the backward pass ([`sliding_minimum_uniform_mirrored`])
/// and the quantised box term. The right channel therefore ends every frame holding exactly the
/// ring words a dual frame would have left in it. Its four register words (`prefix`, `phase`, the
/// box sum and the reduction word) are set from the left's once, when the block ends.
///
/// Step 7 is per channel and runs twice, left then right, each on its own delay ring and its own
/// input, with the one gain `g = 1 - d`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn linked_frame_uniform<L: Lane>(
    left_frame: &mut [f32],
    right_frame: &mut [f32],
    x_left: L,
    x_right: L,
    peak: L,
    limit: L,
    release: L,
    hot: &mut HotChannel<L>,
    left: &mut UniformHot<'_, L>,
    right: &mut UniformHot<'_, L>,
    ring: usize,
    slots: FrameSlots,
    bypass: <L as Lane>::Mask,
) {
    let one = L::splat(1.0);

    let required = L::select(peak.gt(limit), limit.div(peak), one);
    store_ring_lane::<L>(left.required_ring, slots.ring_cursor, required);
    store_ring_lane::<L>(right.required_ring, slots.ring_cursor, required);

    let minimum = sliding_minimum_uniform_mirrored::<L>(
        left.required_ring,
        right.required_ring,
        left.offsets.window,
        ring,
        slots.end,
        slots.start,
        &mut left.prefix,
        &mut left.phase,
    );
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));

    let expired = ring_lane::<L>(left.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(left.box_ring, slots.ring_cursor, quantised);
    store_ring_lane::<L>(right.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);

    let target = one.sub(smoothed);
    let released = release.fma(target.sub(hot.reduction), hot.reduction);
    hot.reduction = flush(target.max(released));

    let gain = one.sub(hot.reduction);
    let delayed = ring_lane::<L>(left.main_ring, slots.main_cursor);
    store_ring_lane::<L>(left.main_ring, slots.main_cursor, x_left);
    L::select(bypass, delayed, delayed.mul(gain)).store(left_frame);
    let delayed = ring_lane::<L>(right.main_ring, slots.main_cursor);
    store_ring_lane::<L>(right.main_ring, slots.main_cursor, x_right);
    L::select(bypass, delayed, delayed.mul(gain)).store(right_frame);
}

// ---------------------------------------------------------------------------------------------
// Issue #1014: the stationary walk.
//
// In the stationary dispatch, [`limiter_block_uniform`] cuts each wrap-free segment also where the
// van Herk block completes, and renders the segment in two passes instead of one fused frame loop.
// Pass 1 runs steps 1-5 of the frozen order and the target `t = 1 - s` for every frame of the
// segment, storing `t` into a stack scratch; pass 2 runs step 6 (the release recursion) and step 7
// (the delay line and the output). Every frame before a segment's last is a *steady* frame: it
// cannot complete, so it needs no `position == 0` branch, no completion test and no backward pass.
// The functions below are the pieces; the four proofs are on `limiter_block_uniform`.
//
// [`channel_frame_uniform`], [`linked_frame_uniform`], [`sliding_minimum_uniform`] (and its
// mirrored twin) and [`segment`] keep their bodies: the ramping dispatch and #990's test oracle
// still run them.
// ---------------------------------------------------------------------------------------------

/// Issue #1014 pass 1, a segment's last frame: [`channel_frame_uniform`]'s steps 1-5, then the
/// release target `1 - s`.
///
/// Token for token the first half of [`channel_frame_uniform`], through
/// [`sliding_minimum_uniform`], so the completing frame and its backward pass are today's.
#[inline(always)]
fn channel_target_uniform<L: Lane>(
    peak: L,
    limit: L,
    hot: &mut HotChannel<L>,
    uniform: &mut UniformHot<'_, L>,
    ring: usize,
    slots: FrameSlots,
) -> L {
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
    let expired = ring_lane::<L>(uniform.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(uniform.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);
    one.sub(smoothed)
}

/// Issue #1014 pass 1, a linked segment's last frame: [`linked_frame_uniform`]'s steps 1-5, with
/// its mirrored ring stores, then the release target `1 - s`.
///
/// Token for token the first half of [`linked_frame_uniform`]: the required gain and the box term
/// are stored into both channels' rings, and the backward pass runs through
/// [`sliding_minimum_uniform_mirrored`].
#[inline(always)]
fn linked_target_uniform<L: Lane>(
    peak: L,
    limit: L,
    hot: &mut HotChannel<L>,
    left: &mut UniformHot<'_, L>,
    right: &mut UniformHot<'_, L>,
    ring: usize,
    slots: FrameSlots,
) -> L {
    let one = L::splat(1.0);
    let required = L::select(peak.gt(limit), limit.div(peak), one);
    store_ring_lane::<L>(left.required_ring, slots.ring_cursor, required);
    store_ring_lane::<L>(right.required_ring, slots.ring_cursor, required);
    let minimum = sliding_minimum_uniform_mirrored::<L>(
        left.required_ring,
        right.required_ring,
        left.offsets.window,
        ring,
        slots.end,
        slots.start,
        &mut left.prefix,
        &mut left.phase,
    );
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));
    let expired = ring_lane::<L>(left.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(left.box_ring, slots.ring_cursor, quantised);
    store_ring_lane::<L>(right.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);
    one.sub(smoothed)
}

/// Issue #1014 pass 1, a steady frame: [`channel_target_uniform`] for a frame that cannot complete
/// its van Herk block.
///
/// [`sliding_minimum_uniform`] with its two branches decided: `position != 0` (the caller presets
/// `prefix` to `+inf` at phase 0, so `prefix.min(newest)` is `newest` there) and not complete (the
/// caller cut the segment at the completion). `prefix.min(newest)` and `oldest.min(running)` keep
/// the operand order of the function they replace.
#[inline(always)]
fn channel_target_steady<L: Lane>(
    peak: L,
    limit: L,
    hot: &mut HotChannel<L>,
    uniform: &mut UniformHot<'_, L>,
    slots: FrameSlots,
) -> L {
    let one = L::splat(1.0);
    let required = L::select(peak.gt(limit), limit.div(peak), one);
    store_ring_lane::<L>(uniform.required_ring, slots.ring_cursor, required);
    let newest = ring_lane::<L>(uniform.required_ring, slots.end);
    let running = uniform.prefix.min(newest);
    uniform.prefix = running;
    let minimum = ring_lane::<L>(uniform.required_ring, slots.start).min(running);
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));
    let expired = ring_lane::<L>(uniform.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(uniform.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);
    one.sub(smoothed)
}

/// Issue #1014 pass 1, a linked steady frame: [`channel_target_steady`] on the left words, with
/// the required gain and the box term also stored into the right channel's rings, as
/// [`linked_frame_uniform`] stores them.
///
/// The checked form, kept for the runs the streams of [`linked_steady_streams`] cannot take.
#[inline(always)]
fn linked_target_steady<L: Lane>(
    peak: L,
    limit: L,
    hot: &mut HotChannel<L>,
    left: &mut UniformHot<'_, L>,
    right: &mut UniformHot<'_, L>,
    slots: FrameSlots,
) -> L {
    let one = L::splat(1.0);
    let required = L::select(peak.gt(limit), limit.div(peak), one);
    store_ring_lane::<L>(left.required_ring, slots.ring_cursor, required);
    store_ring_lane::<L>(right.required_ring, slots.ring_cursor, required);
    let newest = ring_lane::<L>(left.required_ring, slots.end);
    let running = left.prefix.min(newest);
    left.prefix = running;
    let minimum = ring_lane::<L>(left.required_ring, slots.start).min(running);
    let quantised = minimum
        .mul(L::splat(BOX_GRID))
        .floor()
        .mul(L::splat(1.0 / BOX_GRID));
    let expired = ring_lane::<L>(left.box_ring, slots.expiring);
    hot.box_sum = hot.box_sum.add(quantised).sub(expired);
    store_ring_lane::<L>(left.box_ring, slots.ring_cursor, quantised);
    store_ring_lane::<L>(right.box_ring, slots.ring_cursor, quantised);
    let smoothed = hot.box_sum.div(hot.window);
    one.sub(smoothed)
}

/// Issue #1014 pass 2, step 6: the release recursion from a stored target, returning the gain.
///
/// [`channel_frame_uniform`]'s `fma`, `max` and `flush`, then `1 - d`, on the same operands: the
/// target is the word pass 1 stored, and `d` is the previous frame's, in a register.
#[inline(always)]
fn release_step<L: Lane>(target: L, release: L, reduction: &mut L) -> L {
    let one = L::splat(1.0);
    let released = release.fma(target.sub(*reduction), *reduction);
    *reduction = flush(target.max(released));
    one.sub(*reduction)
}

/// Issue #1014 pass 2, step 7: the delay line and the output, for one channel-frame.
///
/// Read before write on `main_ring`, then `select(bypass, z, z * g)`, as in
/// [`channel_frame_uniform`]. `x` is loaded here, in pass 2: pass 1 never touches the block, whose
/// chunk the detector has already read.
#[inline(always)]
fn output_step<L: Lane>(
    io_frame: &mut [f32],
    main_ring: &mut [f32],
    main_cursor: usize,
    gain: L,
    bypass: <L as Lane>::Mask,
) {
    let x = L::load(io_frame);
    let delayed = ring_lane::<L>(main_ring, main_cursor);
    store_ring_lane::<L>(main_ring, main_cursor, x);
    L::select(bypass, delayed, delayed.mul(gain)).store(io_frame);
}

/// One ramp's value for this frame: [`ramp_values`] for a single ramp.
///
/// Issue #1014 advances the limit ramp in pass 1 and the release ramp in pass 2, each once per
/// frame in frame order. `RampLanes::advance` reads only its own four words, so splitting the pair
/// between the passes moves no value. (Under `DISPATCH_STATIONARY` both are resting anyway; the
/// test-only `DISPATCH_RUNTIME` oracle is where the split is exercised while ramping.)
#[inline(always)]
fn ramp_value<const DISPATCH: u8, L: Lane>(stationary: bool, ramp: &mut RampLanes<L>) -> L {
    match DISPATCH {
        DISPATCH_RUNTIME => {
            if stationary {
                ramp.resting_value()
            } else {
                ramp.advance()
            }
        }
        DISPATCH_STATIONARY => ramp.resting_value(),
        DISPATCH_RAMPING => ramp.advance(),
        _ => unreachable!("invalid limiter dispatch"),
    }
}

/// Issue #1014 pass 1 of a linked pair's steady frames, over bounds-check-free ring streams.
///
/// [`linked_target_steady`] for frames `0 .. steady` of a segment, with every ring access an
/// element of an exact-length `chunks_exact(_mut)` view instead of an indexed, range-checked slot.
/// Safe Rust, no copy of ring data, and one induction variable for the whole loop.
///
/// # Preconditions (checked by the caller)
///
/// `Wb < R`, `2 <= steady <= R - Wb`, and the `steady + 1` frames from `slots` are one wrap-free
/// segment whose only completing frame, if any, is the last (which the caller runs afterwards).
///
/// # The stream facts
///
/// Write `c` for the cursor, `e = c + Wb` for the newest slot, `c + 1` for the oldest and
/// `x = c + (R - Wb)` for the expiring box slot, each mod `R`, all as `slots` holds them.
///
/// 1. **The cursor block `C = [c, c + steady]` (`steady + 1` slots) fits in the ring.** The segment
///    has `steady + 1` frames and is wrap-free, so [`segment`] bounded its run by `R - (c + 1)`.
/// 2. **The newest stream `E = [e, e + steady)` is disjoint from `C`.** `steady <= 31` (a segment
///    lies inside one 32-frame chunk) and `Wb >= 32`, so `steady + 1 <= Wb`.
///    * Unwrapped: `e = c + Wb >= c + steady + 1`, past `C`'s end.
///    * Wrapped: `e = c + Wb - R`, and `e + steady <= c` because `steady <= R - Wb`.
///
///    So no frame of the run reads a newest slot the run has written. That is why `Wb == R`, where
///    `e == c`, is excluded.
/// 3. **The oldest slot of frame `s` is `c + 1 + s`, the next slot of `C`.** Frame `s` reads it and
///    frame `s + 1` overwrites it. A walk that visits each slot of `C` once, reading its old word
///    before writing the new one, reads frame `s`'s oldest as the old word of slot `c + 1 + s`.
/// 4. **The expiring stream `X = [x, x + steady)` of the box ring is disjoint from the box cursor
///    stream `B = [c, c + steady)`.**
///    * Unwrapped: `x - c = R - Wb >= steady`.
///    * Wrapped: `x = c - Wb`, and `x + steady <= c` because `steady < 32 <= Wb`.
///
///    `Wb == R`, where `x == c`, is excluded.
/// 5. **The right channel's rings are separate allocations.** Its mirrored stores go to
///    `[c, c + steady)` of each.
///
/// So `E` and `C` are two disjoint borrows of one ring (`split_at_mut` at whichever of `e` and `c`
/// is larger), and so are `X` and `B`.
///
/// # The skewed walk
///
/// The prologue starts frame 0: required gain into `C[0]` and its mirror, then
/// `running = prefix.min(E[0])`. Iteration `j` of the body finishes frame `j - 1` (its oldest is
/// the old word of `C[j]`, fact 3; quantise; `box_sum = (box_sum + q) - X[j - 1]`; `q` into
/// `B[j - 1]` and its mirror; the target `1 - box_sum / Wb`) before it starts frame `j` (ramp,
/// peak, required gain into `C[j]` and its mirror, `running = prefix.min(E[j])`). The epilogue
/// finishes frame `steady - 1` from the old word of `C[steady]`. Every frame's operations and
/// operand order are [`linked_target_steady`]'s: `limit.div(peak)` inside the same select,
/// `prefix.min(newest)`, `oldest.min(running)`, `box_sum.add(q).sub(expired)`. Only the program
/// interleaving across frames changes, and the finishing half of frame `j - 1` reads nothing the
/// starting half of frame `j` writes but the old word of `C[j]`, which it reads first.
///
/// `prefix`, `box_sum`, `phase` (advanced by the caller) and the limit ramp leave as
/// [`linked_target_steady`]'s loop leaves them.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn linked_steady_streams<const DISPATCH: u8, L: Lane>(
    left_peaks: &[f32],
    right_peaks: &[f32],
    targets: &mut [f32],
    stationary: bool,
    hot: &mut HotChannel<L>,
    left: &mut UniformHot<'_, L>,
    right: &mut UniformHot<'_, L>,
    slots: FrameSlots,
    steady: usize,
) {
    let width = L::WIDTH;
    let one = L::splat(1.0);
    let grid = L::splat(BOX_GRID);
    let step = L::splat(1.0 / BOX_GRID);
    let c = slots.ring_cursor;
    let e = slots.end;
    let x = slots.expiring;
    let (cursor_block, newest): (&mut [f32], &[f32]) = if e > c {
        let (lo, hi) = left.required_ring.split_at_mut(e * width);
        (
            &mut lo[c * width..(c + steady + 1) * width],
            &hi[..steady * width],
        )
    } else {
        let (lo, hi) = left.required_ring.split_at_mut(c * width);
        (
            &mut hi[..(steady + 1) * width],
            &lo[e * width..(e + steady) * width],
        )
    };
    let (box_block, expiring): (&mut [f32], &[f32]) = if x > c {
        let (lo, hi) = left.box_ring.split_at_mut(x * width);
        (
            &mut lo[c * width..(c + steady) * width],
            &hi[..steady * width],
        )
    } else {
        let (lo, hi) = left.box_ring.split_at_mut(c * width);
        (
            &mut hi[..steady * width],
            &lo[x * width..(x + steady) * width],
        )
    };
    let mirror_required = &mut right.required_ring[c * width..(c + steady) * width];
    let mirror_box = &mut right.box_ring[c * width..(c + steady) * width];
    let targets = &mut targets[..steady * width];
    let left_peaks = &left_peaks[..steady * width];
    let right_peaks = &right_peaks[..steady * width];
    let window = hot.window;
    let mut prefix = left.prefix;
    let mut box_sum = hot.box_sum;

    // Prologue: start frame 0.
    let limit = ramp_value::<DISPATCH, L>(stationary, &mut hot.limit);
    let peak = L::load(&right_peaks[..width]).max(L::load(&left_peaks[..width]));
    let required = L::select(peak.gt(limit), limit.div(peak), one);
    required.store(&mut cursor_block[..width]);
    required.store(&mut mirror_required[..width]);
    let mut running = prefix.min(L::load(&newest[..width]));
    prefix = running;

    let last = steady - 1;
    for (
        (((((((cursor, newest), left_peak), right_peak), mirror_r), expired), box_slot), mirror_b),
        target,
    ) in cursor_block[width..steady * width]
        .chunks_exact_mut(width)
        .zip(newest[width..].chunks_exact(width))
        .zip(left_peaks[width..].chunks_exact(width))
        .zip(right_peaks[width..].chunks_exact(width))
        .zip(mirror_required[width..].chunks_exact_mut(width))
        .zip(expiring[..last * width].chunks_exact(width))
        .zip(box_block[..last * width].chunks_exact_mut(width))
        .zip(mirror_box[..last * width].chunks_exact_mut(width))
        .zip(targets[..last * width].chunks_exact_mut(width))
    {
        // Finish frame j - 1: its oldest slot is this cursor slot's old word.
        let minimum = L::load(cursor).min(running);
        let quantised = minimum.mul(grid).floor().mul(step);
        box_sum = box_sum.add(quantised).sub(L::load(expired));
        quantised.store(box_slot);
        quantised.store(mirror_b);
        one.sub(box_sum.div(window)).store(target);
        // Start frame j.
        let limit = ramp_value::<DISPATCH, L>(stationary, &mut hot.limit);
        let peak = L::load(right_peak).max(L::load(left_peak));
        let required = L::select(peak.gt(limit), limit.div(peak), one);
        required.store(cursor);
        required.store(mirror_r);
        running = prefix.min(L::load(newest));
        prefix = running;
    }
    // Epilogue: finish frame steady - 1.
    let minimum = L::load(&cursor_block[steady * width..(steady + 1) * width]).min(running);
    let quantised = minimum.mul(grid).floor().mul(step);
    box_sum = box_sum
        .add(quantised)
        .sub(L::load(&expiring[last * width..steady * width]));
    quantised.store(&mut box_block[last * width..steady * width]);
    quantised.store(&mut mirror_box[last * width..steady * width]);
    one.sub(box_sum.div(window))
        .store(&mut targets[last * width..steady * width]);
    left.prefix = prefix;
    hot.box_sum = box_sum;
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
///
/// This entry never links a pair (issue #990): it carries no linked-agreement record, so it renders
/// both channels' gain paths, which is always correct. It is the corpus's entry; the shipped path
/// is [`limiter_block_linkable`], which [`LimiterCore::process_block`] calls with the record.
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
    limiter_block_linkable::<L>(
        left_io, right_io, frames, coef, shape, left, right, cursors, false,
    );
}

/// [`limiter_block`] with the linked-pair decision of issue #990.
///
/// `linked` is the caller's statement that the pair is linked (`LinkMode::Maximum`), that its
/// designed words agree on every lane ([`designed_gain_agree`]) and that its gain words agree (the
/// `gain_linked` record). A uniform cohort then renders its gain path once
/// ([`linked_frame_uniform`]); a cohort that is not uniform renders the per-lane body, which needs
/// no such branch: it computes both channels' gain words from equal operands and leaves them equal.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn limiter_block_linkable<L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
    linked: bool,
) {
    // Issue #990 gate 1: the oracle arm of the identity harness renders the unmodified kernel,
    // which the test module keeps as a verbatim copy. Test builds only.
    #[cfg(test)]
    if REFERENCE_KERNEL.with(Cell::get) {
        tests::reference_block::<L>(left_io, right_io, frames, coef, shape, left, right, cursors);
        return;
    }
    let stationary = dual_stationary(left, right);
    // Issue #182 S1: one whole-bank branch, taken here and nowhere else. Both channels must be
    // uniform, because both run the same body; a bank with a mixed left channel and a uniform
    // right one takes the per-lane path on both, which is the conservative direction and keeps the
    // decision one branch rather than two.
    if lanes_uniform(left) && lanes_uniform(right) {
        if stationary {
            limiter_block_uniform::<DISPATCH_STATIONARY, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, true, linked,
            );
        } else {
            limiter_block_uniform::<DISPATCH_RAMPING, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, false, linked,
            );
        }
    } else {
        if stationary {
            limiter_block_per_lane::<DISPATCH_STATIONARY, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, true,
            );
        } else {
            limiter_block_per_lane::<DISPATCH_RAMPING, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, false,
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn limiter_block_runtime_oracle<L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let stationary = dual_stationary(left, right);
    if lanes_uniform(left) && lanes_uniform(right) {
        limiter_block_uniform::<DISPATCH_RUNTIME, L>(
            left_io, right_io, frames, coef, shape, left, right, cursors, stationary, false,
        );
    } else {
        limiter_block_per_lane::<DISPATCH_RUNTIME, L>(
            left_io, right_io, frames, coef, shape, left, right, cursors, stationary,
        );
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
    fir: &[[L; 4]; HISTORY_WORDS],
    peaks: &mut [f32],
) {
    let width = L::WIDTH;
    debug_assert_eq!(io.len(), peaks.len());
    debug_assert_eq!(io.len() % width, 0);
    let mut history = *taps;
    for (input, output) in io.chunks_exact(width).zip(peaks.chunks_exact_mut(width)) {
        let x = L::load(input);
        detector_peak(&mut history, x, fir).store(output);
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
fn limiter_block_per_lane<const DISPATCH: u8, L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
    stationary: bool,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(right.width, width);
    debug_assert_eq!(left_io.len(), frames * width);
    debug_assert_eq!(right_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let mut hot_right = HotChannel::<L>::load(right);
    #[cfg(test)]
    observe_dispatch::<DISPATCH>(DispatchRoute::DualPerLane);
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
        let active_base = chunk * width;
        let active_words = span * width;
        detector_chunk::<L>(
            &mut hot_left.history,
            &left_io[active_base..active_base + active_words],
            &coef.fir,
            &mut peaks_left[..active_words],
        );
        detector_chunk::<L>(
            &mut hot_right.history,
            &right_io[active_base..active_base + active_words],
            &coef.fir,
            &mut peaks_right[..active_words],
        );

        for frame in 0..span {
            let base = (chunk + frame) * width;
            let (limit_left, release_left) =
                ramp_values::<DISPATCH, L>(stationary, &mut hot_left.limit, &mut hot_left.release);
            let (limit_right, release_right) = ramp_values::<DISPATCH, L>(
                stationary,
                &mut hot_right.limit,
                &mut hot_right.release,
            );

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
///
/// # The linked pair (issue #990 contract 4)
///
/// `linked` is a whole-block decision, taken once by the caller and never per frame or per lane.
/// When it holds, every segment runs [`linked_frame_uniform`] instead of the two
/// [`channel_frame_uniform`] calls, and only the left ramps are advanced per frame. The segment walk,
/// the detector passes and the write-back are the dual body's.
///
/// When the block ends, the right channel's four register words -- `prefix`, `phase`, the box sum
/// and the reduction word -- are set from the left's, and so are its two ramps. Each is the word the
/// dual body would have computed for the right channel, because it would have computed it from
/// equal operands: the ramps entered the block bit-equal ([`designed_gain_agree`]) and advance by a
/// function of their own four words, so copying them at block end in the ramping dispatch is the
/// same as advancing them frame by frame. The right channel's ring words were mirrored frame by
/// frame. So the right channel leaves the block in exactly the state a dual block leaves it in:
/// `snapshot_track`, the resident reduction tap, the silent-rest test and the mono collapse's
/// disengage copy all read the same words.
///
/// # The stationary walk (issue #1014)
///
/// Under `DISPATCH_STATIONARY` (and the test-only `DISPATCH_RUNTIME`) each segment is also cut
/// where the van Herk block completes: `run = min(walk.run, Wb_L - phase_L[, Wb_R - phase_R])`,
/// the bracketed term in the dual body only. The segment is then rendered in two passes. Pass 1
/// runs steps 1-5 and the target `t = 1 - s` for the `run - 1` steady frames
/// ([`channel_target_steady`], [`linked_target_steady`] or [`linked_steady_streams`]) and then for
/// the last frame through today's steps ([`channel_target_uniform`], [`linked_target_uniform`]),
/// storing `t` into a stack scratch. Pass 2 runs step 6 ([`release_step`]) and step 7
/// ([`output_step`]) for every frame, with `d` in a register. Under `DISPATCH_RAMPING` the segment
/// loop is the fused one above, token for token: the walk is a stationary-block change only.
///
/// 1. **The `+inf` preset.** D8's `min(a, b)` is `select(a < b, a, b)`. With `a = +inf`, `a < b`
///    is false for every `b` -- nothing exceeds `+inf`, and a NaN compares false -- so
///    `min(+inf, newest)` is `newest` bit for bit, `-0.0`, subnormals and every NaN payload
///    included. That is [`sliding_minimum_uniform`]'s `position == 0` result. The preset is
///    overwritten by the first frame's `prefix = running`, and a segment has at least one frame
///    ([`segment`] asserts `run >= 1`), so `+inf` never reaches the arena.
/// 2. **Steady frames never complete.** Frame `s` of a segment is at position `phase + s`, and it
///    completes iff `phase + s + 1 == Wb`. With `run <= Wb - phase` that is possible only for
///    `s == run - 1`, the last frame, which is not a steady frame. In the dual body both channels'
///    `Wb - phase` bound `run`, because the channels' windows may differ.
/// 3. **The passes commute.** Within a segment, pass 1 of frame `s` reads and writes only what
///    pass 1 of the frames before `s` left: the required-gain and box rings, `prefix`, `phase`,
///    the box sum, the limit ramp and the peak scratch. It never reads the block, the main ring,
///    `d` or the release ramp. Pass 2 of frame `s` reads `t_s`, `d_{s-1}`, the release ramp, the
///    main ring and the block. Neither pass reads anything the other writes except `t`, and the
///    detector read the block's chunk before the segment began. Each ramp advances once per frame
///    in frame order, the limit ramp in pass 1 and the release ramp in pass 2, and
///    `RampLanes::advance` reads only its own words. So every value is computed by the same
///    operations on the same operands as in the fused frame.
/// 4. **The linked pair** mirrors in pass 1 exactly the writes [`linked_frame_uniform`] mirrors:
///    the required gain and the box term at the cursor, and every suffix minimum of the backward
///    pass, which stays in the last frame through [`sliding_minimum_uniform_mirrored`]. The block-end
///    copy of the right channel's register words and ramps is unchanged, and so is #990's record.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn limiter_block_uniform<const DISPATCH: u8, L: Lane>(
    left_io: &mut [f32],
    right_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    right: &mut ChannelState,
    cursors: &mut Cursors,
    stationary: bool,
    linked: bool,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(right.width, width);
    debug_assert_eq!(left_io.len(), frames * width);
    debug_assert_eq!(right_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let mut hot_right = HotChannel::<L>::load(right);
    #[cfg(test)]
    observe_dispatch::<DISPATCH>(DispatchRoute::DualUniform);
    #[cfg(test)]
    if linked {
        LINKED_ENGAGEMENTS.with(|count| count.set(count.get().saturating_add(1)));
    }
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
    let mut targets_left = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];
    let mut targets_right = [0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH];

    // The ring views borrow the two channels for the whole walk, so the two van Herk words come
    // back out of the scope and are written to the arena below, once.
    let (left_prefix, left_phase, right_prefix, right_phase) = {
        let mut uniform_left = UniformHot::<L>::new(left, shape);
        let mut uniform_right = UniformHot::<L>::new(right, shape);

        // The chunking of the detector is `limiter_block_per_lane`'s, for its reason: only one
        // channel's twelve history words are live at a time.
        for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
            let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
            let active_base = chunk * width;
            let active_words = span * width;
            detector_chunk::<L>(
                &mut hot_left.history,
                &left_io[active_base..active_base + active_words],
                &coef.fir,
                &mut peaks_left[..active_words],
            );
            detector_chunk::<L>(
                &mut hot_right.history,
                &right_io[active_base..active_base + active_words],
                &coef.fir,
                &mut peaks_right[..active_words],
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
                // Issue #1014 A2: the new walk runs in the stationary dispatch (and the test-only
                // runtime oracle). Under `DISPATCH_RAMPING` this arm is the fused loop as it was,
                // token for token.
                let run = if DISPATCH == DISPATCH_RAMPING {
                    let run = walk.run;

                    let base = (chunk + frame) * width;
                    let words = run * width;
                    let left_segment = &mut left_io[base..base + words];
                    let right_segment = &mut right_io[base..base + words];
                    let left_peaks = &peaks_left[frame * width..(frame + run) * width];
                    let right_peaks = &peaks_right[frame * width..(frame + run) * width];

                    if linked {
                        for (step, (((left_frame, right_frame), left_peak), right_peak)) in
                            left_segment
                                .chunks_exact_mut(width)
                                .zip(right_segment.chunks_exact_mut(width))
                                .zip(left_peaks.chunks_exact(width))
                                .zip(right_peaks.chunks_exact(width))
                                .enumerate()
                        {
                            let (limit, release) = ramp_values::<DISPATCH, L>(
                                stationary,
                                &mut hot_left.limit,
                                &mut hot_left.release,
                            );

                            // The dual body's `select(link, linked, peak)` with `link` all-true, which
                            // is `linked` bit for bit: a linked pair is `LinkMode::Maximum` by the
                            // caller's decision.
                            let peak = L::load(right_peak).max(L::load(left_peak));

                            let x_left = L::load(left_frame);
                            let x_right = L::load(right_frame);

                            linked_frame_uniform::<L>(
                                left_frame,
                                right_frame,
                                x_left,
                                x_right,
                                peak,
                                limit,
                                release,
                                &mut hot_left,
                                &mut uniform_left,
                                &mut uniform_right,
                                ring,
                                walk.left.advanced(step),
                                bypass,
                            );
                        }
                        frame += run;
                        ring_cursor = wrapped(ring_cursor + run, ring);
                        main_cursor = wrapped(main_cursor + run, main);
                        continue;
                    }

                    for (step, (((left_frame, right_frame), left_peak), right_peak)) in left_segment
                        .chunks_exact_mut(width)
                        .zip(right_segment.chunks_exact_mut(width))
                        .zip(left_peaks.chunks_exact(width))
                        .zip(right_peaks.chunks_exact(width))
                        .enumerate()
                    {
                        let (limit_left, release_left) = ramp_values::<DISPATCH, L>(
                            stationary,
                            &mut hot_left.limit,
                            &mut hot_left.release,
                        );
                        let (limit_right, release_right) = ramp_values::<DISPATCH, L>(
                            stationary,
                            &mut hot_right.limit,
                            &mut hot_right.release,
                        );

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

                    run
                } else {
                    // Proof 2: also cut the segment where the van Herk block completes, so only
                    // its last frame can. The dual body's two windows may differ (`LaneShape` is
                    // per channel), so it takes both channels' distances; a linked pair has one.
                    let mut run = walk
                        .run
                        .min(uniform_left.offsets.window - uniform_left.phase as usize);
                    if !linked {
                        run = run.min(uniform_right.offsets.window - uniform_right.phase as usize);
                    }
                    let steady = run - 1;
                    #[cfg(test)]
                    census_segment(
                        linked,
                        run,
                        walk.run,
                        span - frame,
                        steady,
                        (uniform_left.phase as usize, uniform_left.offsets.window),
                        (uniform_right.phase as usize, uniform_right.offsets.window),
                    );

                    let base = (chunk + frame) * width;
                    let words = run * width;
                    let left_segment = &mut left_io[base..base + words];
                    let right_segment = &mut right_io[base..base + words];
                    let left_peaks = &peaks_left[frame * width..(frame + run) * width];
                    let right_peaks = &peaks_right[frame * width..(frame + run) * width];
                    let main_base = walk.left.main_cursor;

                    if linked {
                        // Proof 4: one target scratch; pass 1 mirrors the required gain and the
                        // box term into the right channel's rings, and the backward pass (the last
                        // frame's, through the mirrored minimum) as `linked_frame_uniform` does.
                        let targets = &mut targets_left[..words];
                        // Proof 1: `min(+inf, newest)` is `newest`, bit for bit.
                        if uniform_left.phase == 0 {
                            uniform_left.prefix = L::splat(f32::INFINITY);
                        }
                        // The stream facts of `linked_steady_streams` hold exactly here.
                        let streams = uniform_left.offsets.window < ring
                            && steady >= 2
                            && steady <= ring - uniform_left.offsets.window;
                        #[cfg(test)]
                        census_streams(streams, steady, walk.left);
                        if streams {
                            linked_steady_streams::<DISPATCH, L>(
                                left_peaks,
                                right_peaks,
                                targets,
                                stationary,
                                &mut hot_left,
                                &mut uniform_left,
                                &mut uniform_right,
                                walk.left,
                                steady,
                            );
                        } else {
                            for (step, ((left_peak, right_peak), target)) in left_peaks
                                .chunks_exact(width)
                                .zip(right_peaks.chunks_exact(width))
                                .zip(targets.chunks_exact_mut(width))
                                .take(steady)
                                .enumerate()
                            {
                                let limit =
                                    ramp_value::<DISPATCH, L>(stationary, &mut hot_left.limit);
                                let peak = L::load(right_peak).max(L::load(left_peak));
                                linked_target_steady::<L>(
                                    peak,
                                    limit,
                                    &mut hot_left,
                                    &mut uniform_left,
                                    &mut uniform_right,
                                    walk.left.advanced(step),
                                )
                                .store(target);
                            }
                        }
                        // The steady frames advanced the van Herk position by one each; the last
                        // frame's `sliding_minimum_uniform_mirrored` reads and advances it.
                        uniform_left.phase += steady as u32;
                        let limit = ramp_value::<DISPATCH, L>(stationary, &mut hot_left.limit);
                        let peak = L::load(&right_peaks[steady * width..])
                            .max(L::load(&left_peaks[steady * width..]));
                        linked_target_uniform::<L>(
                            peak,
                            limit,
                            &mut hot_left,
                            &mut uniform_left,
                            &mut uniform_right,
                            ring,
                            walk.left.advanced(steady),
                        )
                        .store(&mut targets[steady * width..]);
                        // Pass 2 (proof 3): the recursion and the delay line, one gain for both
                        // channels, with `d` in a register.
                        for (step, ((left_frame, right_frame), target)) in left_segment
                            .chunks_exact_mut(width)
                            .zip(right_segment.chunks_exact_mut(width))
                            .zip(targets.chunks_exact(width))
                            .enumerate()
                        {
                            let release =
                                ramp_value::<DISPATCH, L>(stationary, &mut hot_left.release);
                            let gain =
                                release_step(L::load(target), release, &mut hot_left.reduction);
                            output_step::<L>(
                                left_frame,
                                uniform_left.main_ring,
                                main_base + step,
                                gain,
                                bypass,
                            );
                            output_step::<L>(
                                right_frame,
                                uniform_right.main_ring,
                                main_base + step,
                                gain,
                                bypass,
                            );
                        }
                        frame += run;
                        ring_cursor = wrapped(ring_cursor + run, ring);
                        main_cursor = wrapped(main_cursor + run, main);
                        continue;
                    }

                    // The dual body: a target scratch per channel.
                    let targets_l = &mut targets_left[..words];
                    let targets_r = &mut targets_right[..words];
                    // Proof 1, per channel.
                    if uniform_left.phase == 0 {
                        uniform_left.prefix = L::splat(f32::INFINITY);
                    }
                    if uniform_right.phase == 0 {
                        uniform_right.prefix = L::splat(f32::INFINITY);
                    }
                    for (step, (((left_peak, right_peak), target_l), target_r)) in left_peaks
                        .chunks_exact(width)
                        .zip(right_peaks.chunks_exact(width))
                        .zip(targets_l.chunks_exact_mut(width))
                        .zip(targets_r.chunks_exact_mut(width))
                        .take(steady)
                        .enumerate()
                    {
                        let limit_left = ramp_value::<DISPATCH, L>(stationary, &mut hot_left.limit);
                        let limit_right =
                            ramp_value::<DISPATCH, L>(stationary, &mut hot_right.limit);
                        let peak_left = L::load(left_peak);
                        let peak_right = L::load(right_peak);
                        let linked_peak = peak_right.max(peak_left);
                        let peak_left = L::select(link, linked_peak, peak_left);
                        let peak_right = L::select(link, linked_peak, peak_right);
                        channel_target_steady::<L>(
                            peak_left,
                            limit_left,
                            &mut hot_left,
                            &mut uniform_left,
                            walk.left.advanced(step),
                        )
                        .store(target_l);
                        channel_target_steady::<L>(
                            peak_right,
                            limit_right,
                            &mut hot_right,
                            &mut uniform_right,
                            walk.right.advanced(step),
                        )
                        .store(target_r);
                    }
                    uniform_left.phase += steady as u32;
                    uniform_right.phase += steady as u32;
                    // The last frame, completing or not, through today's steps 1-5.
                    let limit_left = ramp_value::<DISPATCH, L>(stationary, &mut hot_left.limit);
                    let limit_right = ramp_value::<DISPATCH, L>(stationary, &mut hot_right.limit);
                    let peak_left = L::load(&left_peaks[steady * width..]);
                    let peak_right = L::load(&right_peaks[steady * width..]);
                    let linked_peak = peak_right.max(peak_left);
                    let peak_left = L::select(link, linked_peak, peak_left);
                    let peak_right = L::select(link, linked_peak, peak_right);
                    channel_target_uniform::<L>(
                        peak_left,
                        limit_left,
                        &mut hot_left,
                        &mut uniform_left,
                        ring,
                        walk.left.advanced(steady),
                    )
                    .store(&mut targets_l[steady * width..]);
                    channel_target_uniform::<L>(
                        peak_right,
                        limit_right,
                        &mut hot_right,
                        &mut uniform_right,
                        ring,
                        walk.right.advanced(steady),
                    )
                    .store(&mut targets_r[steady * width..]);
                    // Pass 2 (proof 3).
                    for (step, (((left_frame, right_frame), target_l), target_r)) in left_segment
                        .chunks_exact_mut(width)
                        .zip(right_segment.chunks_exact_mut(width))
                        .zip(targets_l.chunks_exact(width))
                        .zip(targets_r.chunks_exact(width))
                        .enumerate()
                    {
                        let release_left =
                            ramp_value::<DISPATCH, L>(stationary, &mut hot_left.release);
                        let release_right =
                            ramp_value::<DISPATCH, L>(stationary, &mut hot_right.release);
                        let gain_left =
                            release_step(L::load(target_l), release_left, &mut hot_left.reduction);
                        let gain_right = release_step(
                            L::load(target_r),
                            release_right,
                            &mut hot_right.reduction,
                        );
                        output_step::<L>(
                            left_frame,
                            uniform_left.main_ring,
                            main_base + step,
                            gain_left,
                            bypass,
                        );
                        output_step::<L>(
                            right_frame,
                            uniform_right.main_ring,
                            main_base + step,
                            gain_right,
                            bypass,
                        );
                    }

                    run
                };
                frame += run;
                ring_cursor = wrapped(ring_cursor + run, ring);
                main_cursor = wrapped(main_cursor + run, main);
            }
        }

        if linked {
            // Issue #990: the right channel's van Herk words are the left's.
            (
                uniform_left.prefix,
                uniform_left.phase,
                uniform_left.prefix,
                uniform_left.phase,
            )
        } else {
            (
                uniform_left.prefix,
                uniform_left.phase,
                uniform_right.prefix,
                uniform_right.phase,
            )
        }
    };

    // R1(a)'s write-back. One store of each van Herk word per block, holding what the last frame
    // of the block computed; `phase` is filled across the cohort because every lane of it shares
    // the one position `lanes_uniform` established.
    left_prefix.store(&mut left.prefix);
    left.phase.fill(left_phase);
    right_prefix.store(&mut right.prefix);
    right.phase.fill(right_phase);

    if linked {
        // Issue #990: the right channel's recursive words and ramps are the left's, as argued in
        // the doc comment above. Its detector history is its own and is stored as computed.
        hot_right.box_sum = hot_left.box_sum;
        hot_right.reduction = hot_left.reduction;
        hot_right.limit = hot_left.limit;
        hot_right.release = hot_left.release;
    }
    hot_left.store(left);
    hot_right.store(right);
    cursors.main = main_cursor as u32;
    cursors.ring = ring_cursor as u32;
}

/// The parameter domains, in descriptor order, for the runtime validator.
///
/// The same three rows as [`TRUE_PEAK_LIMITER_PARAMETERS`]; the descriptor keeps the identity,
/// unit, automation rate and smoothing rule, and `effect-runtime` owns the validation
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
    /// Issue #1091 (console strip P2d): the lanes that carry a member, bit `l` for lane `l`.
    ///
    /// Every lane of a scalar instance and of a full bank. A padded bank's other lanes carry a
    /// clone of a member's request, are fed `+0.0` and have their output discarded
    /// (`effect_contract::PrepareEffectBankRequest`). This word masks the §4.4 report, routes no
    /// automation to a padded lane and refuses a padded lane's state payload, so nothing is charged
    /// to one. The kernel never reads it: a padded lane is rendered exactly as a member is.
    active: u32,
    /// Issue #182 S2: the previous block proved this instance is at a silent fixed point.
    ///
    /// Earned only by observation in [`process_block`](Self::process_block), never assumed. This
    /// kernel's rest state is not all zeros: two of this crate's three retained windows rest at
    /// exactly `1.0`, not `+0.0`, and the argument transfers because what it needs is that the
    /// windows rest **uniform**, so a read from any phase returns the value a slow path would have
    /// read. The fixed point is a property of this limiter's state, including its phase.
    silent_fixed_point: bool,
    /// The bypass flag in force when the claim above was earned. Bypass selects a different arm of
    /// the output `select`, so a claim earned on one side of it says nothing about the other.
    silent_bypass: bool,
    /// Issue #990: the linked-agreement record.
    ///
    /// **Invariant:** when this is `true`, then on every lane the two channels' gain words are
    /// bit-equal -- both gain rings, `prefix`, `phase`, `box_sum` and `reduction`
    /// ([`gain_state_agrees`]). It licenses [`limiter_block_uniform`] to compute a linked pair's gain
    /// path once, together with the per-block legs that the prepared link is `Maximum`, that both
    /// channels are uniform and that the designed words agree ([`designed_gain_agree`]).
    ///
    /// * **Established** at construction and by [`reset`](Self::reset) (including the whole §4.4
    ///   reset of a non-finite block) iff every lane's window shape agrees ([`lane_shapes_agree`],
    ///   which is exact after `clear_runtime`); by [`desymmetrize`](Self::desymmetrize), which
    ///   copies the left channel over the right; and by [`restore_track`](Self::restore_track) and
    ///   a dual block's per-lane §4.4 recovery (#1091), from a full comparison of every lane's gain
    ///   words.
    /// * **Kept** by a dual block that renders under `Maximum` with the designed words agreeing:
    ///   linked, it mirrors every write; dual (the per-lane body), it computes both channels' words
    ///   from equal operands. The silent fast path keeps it too: it advances each channel's phase
    ///   by a function of that phase and the lane's window, and at silent rest the box sum *is* the
    ///   window, so equal box sums mean equal windows.
    /// * **Cleared** by a dual block under `DualMono` or with a designed word apart on any lane, by
    ///   every collapsed block (only the left channel advances), and by a restore whose comparison
    ///   fails. Once cleared it stays cleared until one of the three establishing events: a pair
    ///   whose designed words come back together has no proof its gain words did.
    gain_linked: bool,
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
        let left = ChannelState::new(width, &shape, &left_defaults, rate);
        let right = ChannelState::new(width, &shape, &right_defaults, rate);
        let gain_linked = lane_shapes_agree(&left, &right);
        Some(Self {
            coefficients: LimiterCoef::new(
                matches!(metadata.link_mode, LinkMode::Maximum),
                metadata.bypass,
            ),
            left,
            right,
            cursors: Cursors::default(),
            report: NonFiniteReport::new(),
            active: every_lane(width),
            silent_fixed_point: false,
            silent_bypass: metadata.bypass,
            gain_linked,
            #[cfg(test)]
            silent_engagements: 0,
            metadata,
            shape,
            left_defaults,
            right_defaults,
        })
    }

    /// Binds a padded bank's active mask (issue #1091). `active` names at least one lane and no
    /// lane past `L::WIDTH`; the caller has validated the mask with the request.
    fn with_active_lanes(mut self, active: u32) -> Self {
        debug_assert!(active != 0 && active & !every_lane(L::WIDTH) == 0);
        self.active = active;
        self
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
        // #990: both channels now hold `clear_runtime`'s words.
        self.gain_linked = lane_shapes_agree(&self.left, &self.right);
    }

    /// Runs one block and applies the master plan §4.4 boundary check (decision D7).
    ///
    /// A block whose output is NaN or at least `1e30` in magnitude is recovered lane by lane
    /// ([`reset_failed_lanes`](Self::reset_failed_lanes)): each lane that failed is zeroed on both
    /// channels and reset to its defaults, and the counter is incremented for the members among
    /// them. There is still no per-value check anywhere on this path: a signal that leaves the
    /// representable range is a bug report, not a signal-processing feature. What is per lane is
    /// only who pays for it.
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
            // This makes the skipped block leave the instance
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
        // Issue #990: the whole-block linked-pair decision, and the record it leaves. A block that
        // renders under `Maximum` with every designed word agreeing leaves agreeing gain words
        // behind whichever body runs it, so the record after the block is exactly the decision.
        let linked = self.coefficients.link_max
            && self.gain_linked
            && designed_gain_agree(&self.left, &self.right);
        self.gain_linked = linked;
        limiter_block_linkable::<L>(
            left_io,
            right_io,
            frames,
            &self.coefficients,
            &self.shape,
            &mut self.left,
            &mut self.right,
            &mut self.cursors,
            linked,
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
        // The §4.4 boundary check: one vector scan per channel, exactly `effect-runtime`'s
        // `finish_block` test. The two channels fail together per lane, as they do there, because
        // a lane's pair shares its reset.
        if check_block::<L>(left_io) && check_block::<L>(right_io) {
            return;
        }
        let failed = nonfinite_lane_mask::<L>(left_io) | nonfinite_lane_mask::<L>(right_io);
        if self.reset_failed_lanes(failed) {
            left_io.fill(0.0);
            right_io.fill(0.0);
            // #990: the whole §4.4 reset re-establishes the record exactly as `reset` does.
            self.gain_linked = lane_shapes_agree(&self.left, &self.right);
        } else {
            zero_lanes::<L>(left_io, failed);
            zero_lanes::<L>(right_io, failed);
            // #990: the lanes that did not fail keep gain words the record may have been cleared
            // for, and the reset ones hold `clear_runtime`'s, so the record is read back from the
            // words, every lane, as `restore_track` reads it. No cheaper statement is exact after a
            // partial reset, and this runs only on a block that failed the check.
            self.gain_linked = gain_state_agrees(&self.left, &self.right);
        }
    }

    /// The D7 recovery of a failed block, attributed per lane (issue #1091; decision 12, "Coupling
    /// rule" and "Padding contract").
    ///
    /// `failed` is the lane mask of the block's out-of-bounds words. Returns `true` when the whole
    /// instance was reset, which the caller answers by zeroing the whole block, and `false` when
    /// only the lanes of `failed` were, which it answers by zeroing their words alone.
    ///
    /// * **Every active lane failed** (always so for a scalar instance, and for a full bank whose
    ///   lanes all failed): today's whole reset. Both channels return to their defaults and the
    ///   cursors to zero. A padded lane is reset with them; it is at rest, fed `+0.0`, and its
    ///   output is discarded, so that moves nothing anyone reads, and it brings every lane back to
    ///   one van Herk phase.
    /// * **Otherwise** only the failed lanes are reset
    ///   ([`ChannelState::reset_lane_to_defaults`]), on both channels. A bank-mate that did not
    ///   fail keeps every word, so its bits are the bits it renders per node: banking couples the
    ///   lanes' cost, never their bits. The cursors are shared and stay where they are, which is
    ///   bit-neutral for the reset lane (see [`ChannelState::clear_lane_runtime`]). The reset lane
    ///   restarts at phase zero, so the bank renders the per-lane body until the next whole reset;
    ///   that is cost, and only after a bug report.
    ///
    /// The #990 record is the caller's to restate, because the dual and the collapsed body state
    /// it differently.
    ///
    /// The report is masked by [`active`](Self::active): a padded lane that failed is still
    /// recovered, because it must go on answering `+0.0` with `+0.0`, but it is neither reported
    /// nor counted.
    fn reset_failed_lanes(&mut self, failed: u32) -> bool {
        debug_assert!(failed != 0 && failed & !every_lane(L::WIDTH) == 0);
        let charged = failed & self.active;
        if charged != 0 {
            self.report.nonfinite_lanes = charged;
            self.report.nonfinite_blocks = self.report.nonfinite_blocks.saturating_add(1);
        }
        let shape = self.shape;
        let rate = self.metadata.sample_rate;
        if charged == self.active {
            self.left
                .reset_to_defaults(&shape, &self.left_defaults, rate);
            self.right
                .reset_to_defaults(&shape, &self.right_defaults, rate);
            self.cursors = Cursors::default();
            return true;
        }
        for lane in (0..L::WIDTH).filter(|lane| failed & (1 << lane) != 0) {
            self.left
                .reset_lane_to_defaults(lane, &shape, &self.left_defaults[lane], rate);
            self.right
                .reset_lane_to_defaults(lane, &shape, &self.right_defaults[lane], rate);
        }
        false
    }

    #[cfg(test)]
    fn process_block_runtime_oracle(
        &mut self,
        left_io: &mut [f32],
        right_io: &mut [f32],
        frames: usize,
    ) {
        // The oracle never links, and keeps the record honest for a core it drives.
        self.gain_linked = self.coefficients.link_max
            && self.gain_linked
            && designed_gain_agree(&self.left, &self.right);
        limiter_block_runtime_oracle::<L>(
            left_io,
            right_io,
            frames,
            &self.coefficients,
            &self.shape,
            &mut self.left,
            &mut self.right,
            &mut self.cursors,
        );
    }

    /// [`process_block`](Self::process_block) over one plane: the collapsed track's live channel.
    ///
    /// Every leg of the `quiet` admission reads the left channel, which on a collapse-eligible
    /// cohort is the same predicate the dual body evaluates -- see the module note above
    /// [`limiter_block_mono`] for why the two channels' ramps and window shapes agree.
    ///
    /// The §4.4 boundary check scans the one live plane. Its lane mask is the dual check's
    /// `mask(left) | mask(right)` with `right` equal to `left`, so it is the same mask; the
    /// recovery it triggers is the dual one ([`reset_failed_lanes`](Self::reset_failed_lanes)),
    /// which restores **both** channels of each failed lane.
    fn process_block_mono(&mut self, left_io: &mut [f32], frames: usize) {
        // #990: only the left channel advances from here, the silent path included, so the right
        // channel's gain words go stale until `desymmetrize` copies the left over them.
        self.gain_linked = false;
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
        let failed = nonfinite_lane_mask::<L>(left_io);
        if self.reset_failed_lanes(failed) {
            left_io.fill(0.0);
        } else {
            zero_lanes::<L>(left_io, failed);
        }
        // #990: a collapsed block leaves the record cleared, its recovery included. Only the left
        // channel advanced, and `desymmetrize` re-establishes the record at the disengage boundary.
    }

    #[cfg(test)]
    fn process_block_mono_runtime_oracle(&mut self, left_io: &mut [f32], frames: usize) {
        self.gain_linked = false;
        limiter_block_mono_runtime_oracle::<L>(
            left_io,
            frames,
            &self.coefficients,
            &self.shape,
            &mut self.left,
            &mut self.cursors,
        );
    }

    /// Copies the left channel's whole state onto the right (the collapse's disengage boundary).
    ///
    /// See [`ChannelState::copy_state_from`] for the word-by-word list and why it is exhaustive.
    fn desymmetrize(&mut self) {
        self.right.copy_state_from(&self.left);
        // #990: every word of the right channel is now the left's, the gain words included.
        self.gain_linked = true;
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
        // #990: a payload's two sections are arbitrary, so the record is re-derived from the words
        // themselves, every lane of the bank, on this control-path call. A rejected restore returns
        // above without writing anything and leaves the record as true as it was.
        self.gain_linked = gain_state_agrees(&self.left, &self.right);
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
        &TRUE_PEAK_LIMITER_DESCRIPTOR
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
        // Issue #1091 (console strip P2d): this effect accepts padding. A padded lane's request is
        // a clone of a member's, validated by the loop above like every member's, so it seeds the
        // lane with that member's defaults, program key and window shape: the bank takes the body
        // a full bank of the same members would. The active mask masks the §4.4 report and routes
        // no automation to a padded lane; nothing else reads it (`LimiterCore::active`).
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
        // `validate_shape` admits members-first masks only (#1088, verdict L3), so the members
        // are lanes `0..members`.
        let active = every_lane(request.active_lanes());
        let bank: Box<dyn PreparedNativeEffectBank> =
            effect_contract::match_bank_width!(request.width, |L| {
                Box::new(PreparedTruePeakLimiterBank::<L> {
                    metadata: bank_metadata,
                    core: LimiterCore::<L>::new(metadata, left_defaults, right_defaults)
                        .ok_or(EffectPrepareError {
                            code: "effect.parameter.initial",
                        })?
                        .with_active_lanes(active),
                })
            });
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
        self.left.lane[lane] == self.right.lane[lane]
            && self.left.lookahead_ms[lane].to_bits() == self.right.lookahead_ms[lane].to_bits()
            && ramp_words_agree(&self.left.limit[lane], &self.right.limit[lane])
            && ramp_words_agree(&self.left.release[lane], &self.right.release[lane])
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
        let track = self.checked_member(track_index)?;
        self.core.snapshot_track(track, &mut output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = self.checked_member(track_index)?;
        self.core.restore_track(track, state_layout_version, &input)
    }
}

impl<L: Lane> PreparedTruePeakLimiterBank<L> {
    /// A state payload's track index, which must name a lane that carries a member.
    ///
    /// Issue #1091: a padded lane carries no track, so it has no state to save and none may be
    /// restored into it. A restored payload would also leave it off the rest state it must hold to
    /// answer `+0.0` with `+0.0`.
    fn checked_member(&self, track_index: u32) -> Result<usize, StatePayloadError> {
        let track = checked_track(track_index, L::WIDTH)?;
        if self.core.active & (1 << track) == 0 {
            return Err(state_error("effect.state.track"));
        }
        Ok(track)
    }

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
            // Issue #1091: a padded lane carries no track. The caller routes it no automation, and
            // none is applied or charged to it, so its report entry stays empty and its clone
            // parameters stay the member's.
            if self.core.active & (1 << track) == 0 {
                continue;
            }
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
    let stationary = mono_stationary(left);
    if lanes_uniform(left) {
        if stationary {
            limiter_block_uniform_mono::<DISPATCH_STATIONARY, L>(
                left_io, frames, coef, shape, left, cursors, true,
            );
        } else {
            limiter_block_uniform_mono::<DISPATCH_RAMPING, L>(
                left_io, frames, coef, shape, left, cursors, false,
            );
        }
    } else {
        if stationary {
            limiter_block_per_lane_mono::<DISPATCH_STATIONARY, L>(
                left_io, frames, coef, shape, left, cursors, true,
            );
        } else {
            limiter_block_per_lane_mono::<DISPATCH_RAMPING, L>(
                left_io, frames, coef, shape, left, cursors, false,
            );
        }
    }
}

#[cfg(test)]
#[inline(always)]
fn limiter_block_mono_runtime_oracle<L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
) {
    let stationary = mono_stationary(left);
    if lanes_uniform(left) {
        limiter_block_uniform_mono::<DISPATCH_RUNTIME, L>(
            left_io, frames, coef, shape, left, cursors, stationary,
        );
    } else {
        limiter_block_per_lane_mono::<DISPATCH_RUNTIME, L>(
            left_io, frames, coef, shape, left, cursors, stationary,
        );
    }
}

/// [`limiter_block_per_lane`] over one plane.
#[inline(always)]
fn limiter_block_per_lane_mono<const DISPATCH: u8, L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
    stationary: bool,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(left_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    #[cfg(test)]
    observe_dispatch::<DISPATCH>(DispatchRoute::MonoPerLane);
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
        let active_base = chunk * width;
        let active_words = span * width;
        detector_chunk::<L>(
            &mut hot_left.history,
            &left_io[active_base..active_base + active_words],
            &coef.fir,
            &mut peaks_left[..active_words],
        );

        for frame in 0..span {
            let base = (chunk + frame) * width;
            let (limit_left, release_left) =
                ramp_values::<DISPATCH, L>(stationary, &mut hot_left.limit, &mut hot_left.release);

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
fn limiter_block_uniform_mono<const DISPATCH: u8, L: Lane>(
    left_io: &mut [f32],
    frames: usize,
    coef: &LimiterCoef<L>,
    shape: &Shape,
    left: &mut ChannelState,
    cursors: &mut Cursors,
    stationary: bool,
) {
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(left_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    #[cfg(test)]
    observe_dispatch::<DISPATCH>(DispatchRoute::MonoUniform);
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
            let active_base = chunk * width;
            let active_words = span * width;
            detector_chunk::<L>(
                &mut hot_left.history,
                &left_io[active_base..active_base + active_words],
                &coef.fir,
                &mut peaks_left[..active_words],
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
                    let (limit_left, release_left) = ramp_values::<DISPATCH, L>(
                        stationary,
                        &mut hot_left.limit,
                        &mut hot_left.release,
                    );

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
    use dsp_reference::reference_annex2_phases;
    use effect_contract::BankWidth;
    use effect_contract::{
        PrepareEffectLimits, PreparedPorts, PreparedSidechainPort, validate_descriptor,
    };
    use lane::Simd4;

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
            value: TRUE_PEAK_LIMITER_PARAMETERS[index / 2].default_value,
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
                active_mask: width.full_mask(),
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
        validate_descriptor(&TRUE_PEAK_LIMITER_DESCRIPTOR).expect("descriptor");
        assert_eq!(
            TRUE_PEAK_LIMITER_DESCRIPTOR.id.as_str(),
            "miso.true-peak-limiter"
        );
        assert_eq!(TRUE_PEAK_LIMITER_DESCRIPTOR.supported_link_modes.bits(), 3);
        assert_eq!(TRUE_PEAK_LIMITER_DESCRIPTOR.state_layout_version, 1);
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
    ///
    /// Since #1013 this is a statement about `annex2_phases_seeded`, the brief's order as written
    /// (`+0.0` accumulator). The render path's seedless form is held to it by E1b.
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
            let produced = annex2_phases_seeded(&kernel_history, &coefficients.fir);
            for (phase, value) in produced.iter().enumerate() {
                assert_eq!(
                    value.to_bits(),
                    expected[phase].to_bits(),
                    "phase {phase} bits"
                );
            }
        }
    }

    /// The Annex-2 table phase-major: `annex2_columns()[phase][tap]`.
    fn annex2_columns() -> [[f32; HISTORY_WORDS]; 4] {
        core::array::from_fn(|phase| core::array::from_fn(|tap| ANNEX2_FIR[tap][phase]))
    }

    /// E1b's inputs: named sample streams. Each lane of a bank runs one stream from a zero
    /// history, so every window of twelve consecutive samples of a stream is a history both
    /// detectors see.
    fn seedless_streams() -> Vec<(&'static str, Vec<f32>)> {
        // SplitMix64, for the drawn bit patterns.
        let mut state = 0x1013_E1B0_0001_u64;
        let mut word = move || {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut mixed = state;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            mixed ^ (mixed >> 31)
        };
        let mut noise = Noise(0x1013_E1B0_0002);
        let mut loud =
            move |count: usize| -> Vec<f32> { (0..count).map(|_| noise.next() * 3.0).collect() };
        let mut streams = Vec::new();

        // E1's noise, as E1 draws it.
        let mut e1 = Noise(0x5150_0090_0001);
        streams.push(("E1 noise", (0..4096).map(|_| e1.next() * 3.0).collect()));

        // A single impulse walking through all twelve taps, on a background of either zero.
        for background in [0.0_f32, -0.0] {
            for impulse in [0.0_f32, -0.0, 1.0, -1.0] {
                let mut stream = vec![background; HISTORY_WORDS];
                stream.push(impulse);
                stream.extend([background; HISTORY_WORDS]);
                streams.push(("impulse", stream));
            }
        }

        // For each phase, the one signed-zero history whose twelve products are all `-0.0`: tap `k`
        // is the zero of the sign opposite to `fir[k][phase]`. Only there does the seeded
        // accumulator end `+0.0` against the seedless one's `-0.0`; a random mix of signed zeros
        // gets there once in 4096 windows per phase.
        for column in annex2_columns() {
            let stream: Vec<f32> = column
                .iter()
                .rev()
                .map(|coefficient| if *coefficient > 0.0 { -0.0 } else { 0.0 })
                .collect();
            streams.push(("all products -0.0", stream));
        }

        // Histories of mixed signed zeros.
        streams.push((
            "signed zeros",
            (0..4096)
                .map(|_| f32::from_bits(((word() >> 17) as u32 & 1) << 31))
                .collect(),
        ));

        // Random subnormals, sign and mantissa drawn, exponent field zero.
        streams.push((
            "subnormals",
            (0..4096)
                .map(|_| f32::from_bits(word() as u32 & 0x807F_FFFF))
                .collect(),
        ));

        // One NaN tap in noise, with each of two payloads: a positive quiet NaN and a negative
        // signalling one.
        for payload in [0x7FC0_1234_u32, 0xFFA0_5A5A] {
            let mut stream = loud(2 * HISTORY_WORDS);
            stream.push(f32::from_bits(payload));
            stream.extend(loud(2 * HISTORY_WORDS));
            streams.push(("NaN tap", stream));
        }

        // One infinite tap in noise, of either sign.
        for infinity in [f32::INFINITY, f32::NEG_INFINITY] {
            let mut stream = loud(2 * HISTORY_WORDS);
            stream.push(infinity);
            stream.extend(loud(2 * HISTORY_WORDS));
            streams.push(("infinite tap", stream));
        }

        // `+inf` and `-inf` in one window, at every spacing and in either order: a phase whose
        // products take both signs of infinity makes its NaN inside the chain, at an add, rather
        // than receiving one from a tap.
        for (first, second) in [
            (f32::INFINITY, f32::NEG_INFINITY),
            (f32::NEG_INFINITY, f32::INFINITY),
        ] {
            for spacing in 1..HISTORY_WORDS {
                let mut stream = loud(2 * HISTORY_WORDS);
                stream.push(first);
                stream.extend(loud(spacing - 1));
                stream.push(second);
                stream.extend(loud(2 * HISTORY_WORDS));
                streams.push(("NaN made in a chain", stream));
            }
        }
        streams
    }

    /// The pre-#1013 detector, E1b's oracle: [`detector_peak`] with `annex2_phases_seeded`.
    fn detector_peak_seeded<L: Lane>(
        history: &mut History<L>,
        x: L,
        fir: &[[L; 4]; HISTORY_WORDS],
    ) -> L {
        history.shift(x);
        let mut peak = history.t6.abs();
        for phase in annex2_phases_seeded(history, fir) {
            peak = peak.max(phase.abs());
        }
        peak
    }

    /// What one width of E1b saw, for the non-vacuity checks.
    #[derive(Default)]
    struct SeedlessCounts {
        frames: usize,
        signed_zero_flips: usize,
        nan_peaks: usize,
    }

    /// One width of E1b. Streams are packed `L::WIDTH` to a bank, one per lane, and a lane whose
    /// stream has ended is fed `+0.0`.
    fn seedless_peaks_match_the_seeded_order<L: Lane>() -> SeedlessCounts {
        let fir = LimiterCoef::<L>::new(false, false).fir;
        let streams = seedless_streams();
        let mut counts = SeedlessCounts::default();
        for (group, bank) in streams.chunks(L::WIDTH).enumerate() {
            let mut candidate = History::<L>::zero();
            let mut oracle = History::<L>::zero();
            let length = bank
                .iter()
                .map(|(_, stream)| stream.len())
                .max()
                .expect("streams");
            for frame in 0..length {
                let mut input = [0.0_f32; MAXIMUM_WIDTH];
                for (lane, (_, stream)) in bank.iter().enumerate() {
                    input[lane] = stream.get(frame).copied().unwrap_or(0.0);
                }
                let x = L::load(&input);
                let mut produced = [0.0_f32; MAXIMUM_WIDTH];
                let mut expected = [0.0_f32; MAXIMUM_WIDTH];
                detector_peak(&mut candidate, x, &fir).store(&mut produced);
                detector_peak_seeded(&mut oracle, x, &fir).store(&mut expected);

                // The proof's induction, checked phase by phase: the same class-A word (every NaN
                // one value, #1065), or zeros of opposite sign.
                let seedless = annex2_phases(&candidate, &fir);
                let seeded = annex2_phases_seeded(&candidate, &fir);
                for (phase, (seedless, seeded)) in seedless.iter().zip(seeded.iter()).enumerate() {
                    let mut new = [0.0_f32; MAXIMUM_WIDTH];
                    let mut old = [0.0_f32; MAXIMUM_WIDTH];
                    seedless.store(&mut new);
                    seeded.store(&mut old);
                    for lane in 0..bank.len() {
                        let (new, old) = (new[lane], old[lane]);
                        if dsp_reference::class_a::same(new, old) {
                            continue;
                        }
                        assert!(
                            new == 0.0 && old == 0.0,
                            "W{} group {group} lane {lane} ({}) frame {frame} phase {phase}: \
                             {:#010x} against {:#010x}",
                            L::WIDTH,
                            bank[lane].0,
                            new.to_bits(),
                            old.to_bits()
                        );
                        counts.signed_zero_flips += 1;
                    }
                }

                for lane in 0..bank.len() {
                    let (new, old) = (produced[lane], expected[lane]);
                    if old.is_nan() {
                        assert!(
                            new.is_nan(),
                            "W{} group {group} lane {lane} ({}) frame {frame}: {:#010x} for NaN",
                            L::WIDTH,
                            bank[lane].0,
                            new.to_bits()
                        );
                        counts.nan_peaks += 1;
                    } else {
                        assert_eq!(
                            new.to_bits(),
                            old.to_bits(),
                            "W{} group {group} lane {lane} ({}) frame {frame}: peak bits",
                            L::WIDTH,
                            bank[lane].0
                        );
                    }
                    counts.frames += 1;
                }
            }
        }
        counts
    }

    /// E1b (#1013): the seedless detector's peaks are the `+0.0`-seeded detector's, bit for bit
    /// (NaN peaks as "both NaN"), at every width.
    ///
    /// E1 cannot see the seed: its noise never makes a signed-zero first product. The signed-zero
    /// rows here are the discriminating ones, and the non-vacuity checks below say that they did
    /// discriminate: phases did come out as zeros of opposite sign, peaks did come out NaN, and a
    /// NaN was made inside a chain from two infinite products, not only received from a tap.
    #[test]
    fn seedless_peaks_match_the_seeded_order_at_every_width() {
        let samples: usize = seedless_streams()
            .iter()
            .map(|(_, stream)| stream.len())
            .sum();
        let mut every_width = Vec::new();
        lane::each_lane!(|L| every_width.push(seedless_peaks_match_the_seeded_order::<L>()));
        for counts in every_width {
            assert!(
                counts.frames >= samples,
                "{} of {samples} frames",
                counts.frames
            );
            // At least the four designed histories, one per phase.
            assert!(
                counts.signed_zero_flips >= 4,
                "{} phases flipped a zero's sign",
                counts.signed_zero_flips
            );
            assert!(counts.nan_peaks > 0, "no NaN peak");
        }

        // A NaN made inside a chain: every tap and every product of the window is non-NaN, and
        // the seeded accumulator still ends NaN. Checked once, in scalar arithmetic, over the
        // streams' windows.
        let mut made = 0;
        for (_, stream) in seedless_streams() {
            for end in HISTORY_WORDS..=stream.len() {
                let window: Vec<f32> = stream[end - HISTORY_WORDS..end]
                    .iter()
                    .rev()
                    .copied()
                    .collect();
                for column in annex2_columns() {
                    let products: Vec<f32> = column
                        .iter()
                        .zip(&window)
                        .map(|(coefficient, tap)| coefficient * tap)
                        .collect();
                    if products.iter().any(|product| product.is_nan()) {
                        continue;
                    }
                    let sum = products.iter().fold(0.0_f32, |sum, product| sum + *product);
                    if sum.is_nan() {
                        made += 1;
                    }
                }
            }
        }
        assert!(made > 0, "no NaN was made inside a chain");
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

    /// The pre-change frame-at-a-time detector shape, retained only as a private test oracle.
    /// Keeping the full input and absolute chunk offset here makes a wrong active window produce
    /// a different result instead of letting the candidate compare against its own slice.
    fn detector_chunk_old_shape<L: Lane>(
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

    fn detector_history_bits<L: Lane>(history: &History<L>) -> Vec<u32> {
        let words = [
            history.t0,
            history.t1,
            history.t2,
            history.t3,
            history.t4,
            history.t5,
            history.t6,
            history.t7,
            history.t8,
            history.t9,
            history.t10,
            history.t11,
        ];
        let mut bits = Vec::with_capacity(HISTORY_WORDS * L::WIDTH);
        for word in words {
            let mut lanes = [0.0_f32; MAXIMUM_WIDTH];
            word.store(&mut lanes);
            bits.extend(lanes[..L::WIDTH].iter().map(|value| value.to_bits()));
        }
        bits
    }

    fn detector_peak_bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    fn detector_history_seed<L: Lane>() -> History<L> {
        let word = |tap: usize| {
            let lanes: [f32; MAXIMUM_WIDTH] =
                core::array::from_fn(|lane| (tap * MAXIMUM_WIDTH + lane + 1) as f32 * 0.03125);
            L::load(&lanes)
        };
        History {
            t0: word(0),
            t1: word(1),
            t2: word(2),
            t3: word(3),
            t4: word(4),
            t5: word(5),
            t6: word(6),
            t7: word(7),
            t8: word(8),
            t9: word(9),
            t10: word(10),
            t11: word(11),
        }
    }

    /// The candidate must match the old shape for an offset full chunk and an offset short tail.
    /// The sentinel proves that only the active peak prefix is written; the shifted-window arm is
    /// a wrong-result control for both the peak prefix and all twelve history words.
    fn detector_chunk_active_window_matches_old_shape<L: Lane>() {
        const TOTAL_FRAMES: usize = 64;
        const CHUNK_OFFSET: usize = 5;
        const SHORT_OFFSET: usize = DETECTOR_CHUNK + 3;
        const SHORT_SPAN: usize = 7;
        const SENTINEL: f32 = -12_345.25;

        let coefficients = LimiterCoef::<L>::new(false, false);
        let mut noise = Noise(0x6210_6100 ^ L::WIDTH as u64);
        let input: Vec<f32> = (0..TOTAL_FRAMES * L::WIDTH)
            .map(|_| noise.next() * 0.75)
            .collect();

        for (chunk, span) in [(CHUNK_OFFSET, DETECTOR_CHUNK), (SHORT_OFFSET, SHORT_SPAN)] {
            let words = span * L::WIDTH;
            let mut candidate_history = detector_history_seed::<L>();
            let mut oracle_history = candidate_history;
            let mut candidate_peaks = vec![SENTINEL; DETECTOR_CHUNK * MAXIMUM_WIDTH];
            let mut oracle_peaks = [SENTINEL; DETECTOR_CHUNK * MAXIMUM_WIDTH];
            let active_base = chunk * L::WIDTH;

            detector_chunk::<L>(
                &mut candidate_history,
                &input[active_base..active_base + words],
                &coefficients.fir,
                &mut candidate_peaks[..words],
            );
            detector_chunk_old_shape::<L>(
                &mut oracle_history,
                &input,
                chunk,
                span,
                &coefficients.fir,
                &mut oracle_peaks,
            );

            assert_eq!(
                detector_peak_bits(&candidate_peaks[..words]),
                detector_peak_bits(&oracle_peaks[..words]),
                "active peak prefix for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
            assert_eq!(
                detector_history_bits(&candidate_history),
                detector_history_bits(&oracle_history),
                "all twelve history words for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
            assert!(
                candidate_peaks[..words]
                    .iter()
                    .all(|value| value.to_bits() != SENTINEL.to_bits()),
                "active peak prefix was not populated for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
            assert!(
                candidate_peaks[words..]
                    .iter()
                    .all(|value| value.to_bits() == SENTINEL.to_bits()),
                "candidate wrote beyond active peak prefix for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
            assert!(
                oracle_peaks[words..]
                    .iter()
                    .all(|value| value.to_bits() == SENTINEL.to_bits()),
                "oracle wrote beyond active peak prefix for width {} chunk {chunk} span {span}",
                L::WIDTH
            );

            let wrong_base = (chunk + 1) * L::WIDTH;
            let mut wrong_history = detector_history_seed::<L>();
            let mut wrong_peaks = vec![SENTINEL; DETECTOR_CHUNK * MAXIMUM_WIDTH];
            detector_chunk::<L>(
                &mut wrong_history,
                &input[wrong_base..wrong_base + words],
                &coefficients.fir,
                &mut wrong_peaks[..words],
            );
            assert_ne!(
                detector_peak_bits(&candidate_peaks[..words]),
                detector_peak_bits(&wrong_peaks[..words]),
                "shifted active input must change peak prefix for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
            assert_ne!(
                detector_history_bits(&candidate_history),
                detector_history_bits(&wrong_history),
                "shifted active input must change history for width {} chunk {chunk} span {span}",
                L::WIDTH
            );
        }
    }

    #[test]
    fn detector_chunk_active_window_matches_old_shape_scalar() {
        detector_chunk_active_window_matches_old_shape::<f32>();
    }

    #[test]
    fn detector_chunk_active_window_matches_old_shape_w4() {
        detector_chunk_active_window_matches_old_shape::<Simd4>();
    }

    /// The 8-lane (AVX2) twin of `detector_chunk_active_window_matches_old_shape_w4`.
    #[cfg(target_feature = "avx2")]
    #[test]
    fn detector_chunk_active_window_matches_old_shape_w8() {
        detector_chunk_active_window_matches_old_shape::<lane::Simd8>();
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
        lane::each_lane!(|L| check::<L>());
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
        lane::each_lane!(|L| check::<L>());
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

            // Every width this build binds: a bank wider than the backend declines, so a 4-lane
            // (NEON/simd128) build checks W4 and the 8-lane (AVX2) build checks W4 and W8.
            for (width, backend, lanes) in BankWidth::ALL
                .iter()
                .map(|&width| (width, width.backend(), width.lanes() as usize))
                .filter(|&(_, _, lanes)| lanes <= Backend::current().width())
            {
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

    /// This build's native bank: eight lanes on x86-64-v3, four on AArch64 NEON (#1017). A bank
    /// wider than the backend does not bind, so the cohort tests below run at this width.
    fn native_bank() -> (BankWidth, Backend, usize) {
        let backend = Backend::current();
        let width = BankWidth::for_backend(backend).expect("every product target has a bank width");
        (width, backend, width.lanes() as usize)
    }

    /// Renders `tracks` through one bank of the native width over `blocks` blocks of 128 frames.
    ///
    /// The bank arm of [`cohort_run`] on its own, over the same per-lane signal, for the
    /// comparisons whose oracle is another *bank* rather than a scalar twin. A scalar instance is
    /// `W = 1` and therefore uniform by construction, so it is not a usable oracle for anything
    /// the uniform body does to a whole channel.
    fn bank_planes(
        tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        blocks: usize,
    ) -> (Vec<f32>, Vec<f32>) {
        let (width, backend, lanes) = native_bank();
        assert_eq!(tracks.len(), lanes);
        let frames = blocks * 128;
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        for lane in 0..lanes {
            let mut noise = Noise(0x5182_0000 + lane as u64);
            let lane_left: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            let lane_right: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            for frame in 0..frames {
                left[frame * lanes + lane] = lane_left[frame];
                right[frame * lanes + lane] = lane_right[frame];
            }
        }
        let mut bank = bank_for(tracks, LinkMode::DualMono, width, backend);
        for block in 0..blocks {
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
        (left, right)
    }

    /// Renders `tracks` through scalar instances and one bank of the native width over `blocks`
    /// blocks of 128.
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
        let (width, backend, lanes) = native_bank();
        assert_eq!(tracks.len(), lanes);
        let frames = blocks * 128;
        let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
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

        let mut bank = bank_for(tracks, LinkMode::DualMono, width, backend);
        let mut bank_left = vec![0.0_f32; frames * lanes];
        let mut bank_right = vec![0.0_f32; frames * lanes];
        for frame in 0..frames {
            for lane in 0..lanes {
                bank_left[frame * lanes + lane] = inputs[lane].0[frame];
                bank_right[frame * lanes + lane] = inputs[lane].1[frame];
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
                &mut bank_left[start * lanes..(start + 128) * lanes],
                &mut bank_right[start * lanes..(start + 128) * lanes],
                width,
                128,
                start as u64,
            );
        }

        CohortRun {
            scalar,
            bank_left,
            bank_right,
            frames,
            lanes,
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
        let tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..native_bank().2)
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
        let mut tracks: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..native_bank().2)
            .map(|_| values_with(-6.0, 100.0, 5.0))
            .collect();
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
    /// the crossed line. Both arms below are banks of the native width (W8 on x86-64-v3, W4 on
    /// AArch64) over the same signal; the oracle arm's last lane carries a third, different *left*
    /// lookahead, which makes `lanes_uniform(left)` false and sends the whole bank down the
    /// fallback. Every other lane is prepared identically in the two arms, so their rendered
    /// samples must agree to the bit.
    #[test]
    fn the_two_channels_of_a_uniform_cohort_keep_their_own_phases() {
        const BLOCKS: usize = 16;
        let lanes = native_bank().2;
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
        let subject: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..lanes)
            .map(|_| values_split(-6.0, 100.0, 5.0, 1.0))
            .collect();
        // Oracle: the last lane's *left* lookahead differs, so `lanes_uniform(left)` is false and
        // both channels take the per-lane body. Every other lane is byte-identical to the subject's.
        let mut oracle = subject.clone();
        oracle[lanes - 1] = values_split(-6.0, 100.0, 3.0, 1.0);

        let (subject_left, subject_right) = bank_planes(&subject, BLOCKS);
        let (oracle_left, oracle_right) = bank_planes(&oracle, BLOCKS);

        for lane in 0..lanes - 1 {
            for frame in 0..BLOCKS * 128 {
                let index = frame * lanes + lane;
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
            (0..native_bank().2).map(|_| values).collect();

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
    // This limiter's rest state is not all zeros: two of its three retained windows rest at exactly
    // `1.0` rather than `+0.0`, and the argument transfers because what it needs is that resting
    // storage is **uniform**. A read from any phase then returns the value a slow path would have
    // read, so a block that writes only the resting value back leaves it bit-identical at every
    // phase.
    //
    // The fixed point is also exactly reachable here.
    // The compressor's `gain_reduction_db` envelope approaches `0` dB geometrically; this limiter's
    // box terms live on the `2^-14` grid with `BOX_GRID * R` below `2^24` at every launch rate,
    // so the running sum arrives exactly at `Wb`, and the D7 flush terminates the release at
    // exactly `+0.0` rather than near it. The resulting proof is independent of the phase at which
    // the block begins.
    // ---------------------------------------------------------------------------------------

    /// One block of signal, or a block filled with `quiet` — an exact `+0.0` or an exact `-0.0`.
    ///
    /// The signal is [`Noise`], the crate's SplitMix64, seeded from the block index so that two
    /// arms of the same comparison see the same samples and a corpus stays a seed rather than a
    /// file. It is deliberately *not* a sine: `f32::sin` is a platform transcendental, which
    /// decision D6 and `clippy.toml`'s `disallowed-methods` (formerly `scripts/check-math-policy.sh`)
    /// forbid anywhere in `src/` — including in a test module, since the lint scans the file and
    /// not the `cfg`.
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
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
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
    /// fast path's cursor and phase advances load-bearing. Comparing this state between the two arms, and
    /// not only rendered samples, proves that the skipped block leaves every retained word at the
    /// same bits; `phase` is included for that reason. The state proof is independent of the
    /// rendered sample comparison.
    fn state_bits<L: Lane>(core: &LimiterCore<L>) -> Vec<u32> {
        let mut words = vec![core.cursors.main, core.cursors.ring];
        for channel in [&core.left, &core.right] {
            words.extend(channel_runtime_bits(channel));
        }
        words
    }

    fn channel_runtime_bits(channel: &ChannelState) -> Vec<u32> {
        let mut words = Vec::new();
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
        words
    }

    /// Builds the two route shapes this witness needs without changing the production factory.
    fn dispatch_witness_core<L: Lane>(uniform: bool) -> LimiterCore<L> {
        let mut values = vec![values_with(-6.0, 100.0, 5.0); L::WIDTH];
        if !uniform {
            for (lane, values) in values.iter_mut().enumerate().skip(1) {
                *values = values_with(-6.0, 100.0, if lane.is_multiple_of(2) { 1.0 } else { 5.0 });
            }
        }
        let mut preparation = request(&values[0]);
        preparation.link_mode = LinkMode::DualMono;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
            .expect("metadata");
        let mut left_defaults = Vec::with_capacity(L::WIDTH);
        let mut right_defaults = Vec::with_capacity(L::WIDTH);
        for values in &values {
            let (left, right) = initial_defaults(values).expect("defaults");
            left_defaults.push(left);
            right_defaults.push(right);
        }
        LimiterCore::<L>::new(
            metadata,
            left_defaults.into_boxed_slice(),
            right_defaults.into_boxed_slice(),
        )
        .expect("core")
    }

    /// Opens every dual ramp so the first populated block crosses an endpoint internally.
    fn open_dispatch_witness_ramps<L: Lane>(core: &mut LimiterCore<L>) {
        for channel in [&mut core.left, &mut core.right] {
            for ramp in channel.limit.iter_mut() {
                ramp.set_target(limit_coefficient(-3.0), RAMP_UPDATES);
            }
            for ramp in channel.release.iter_mut() {
                ramp.set_target(release_coefficient(200.0, 48_000), RAMP_UPDATES);
            }
        }
    }

    fn compare_dispatch_witness_case<L: Lane>(
        label: &str,
        uniform: bool,
        mono: bool,
        route: DispatchRoute,
    ) {
        const FRAMES: usize = 512;
        let mut candidate = dispatch_witness_core::<L>(uniform);
        let mut oracle = dispatch_witness_core::<L>(uniform);
        open_dispatch_witness_ramps(&mut candidate);
        open_dispatch_witness_ramps(&mut oracle);
        let right_before = mono.then(|| channel_runtime_bits(&candidate.right));
        let mut populated = false;

        for block in 0..2 {
            let mut candidate_left = silence_plane(block, FRAMES, L::WIDTH, None, 0.4, false);
            let mut oracle_left = candidate_left.clone();
            let mut candidate_right = silence_plane(block, FRAMES, L::WIDTH, None, 0.4, true);
            let mut oracle_right = candidate_right.clone();
            clear_dispatch_observation();
            if mono {
                candidate.process_block_mono(&mut candidate_left, FRAMES);
            } else {
                candidate.process_block(&mut candidate_left, &mut candidate_right, FRAMES);
            }
            let candidate_observation = dispatch_observation();
            if mono {
                oracle.process_block_mono_runtime_oracle(&mut oracle_left, FRAMES);
            } else {
                oracle.process_block_runtime_oracle(&mut oracle_left, &mut oracle_right, FRAMES);
            }
            let oracle_observation = dispatch_observation();

            // Identity is checked before the positive specialization assertion. A later
            // fallback-only control must keep these comparisons green and fail the same mechanism
            // assertion below, rather than being allowed to pass through classification intent.
            assert_eq!(
                candidate_left
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                oracle_left
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>(),
                "{label}: left PCM block {block}"
            );
            if !mono {
                assert_eq!(
                    candidate_right
                        .iter()
                        .map(|sample| sample.to_bits())
                        .collect::<Vec<_>>(),
                    oracle_right
                        .iter()
                        .map(|sample| sample.to_bits())
                        .collect::<Vec<_>>(),
                    "{label}: right PCM block {block}"
                );
            }
            assert_eq!(
                state_bits(&candidate),
                state_bits(&oracle),
                "{label}: complete state block {block}"
            );
            assert_eq!(
                candidate.silent_fixed_point, oracle.silent_fixed_point,
                "{label}: silent state block {block}"
            );
            if let Some(right_before) = &right_before {
                assert_eq!(
                    channel_runtime_bits(&candidate.right),
                    *right_before,
                    "{label}: mono candidate accessed the right plane at block {block}"
                );
            }

            assert_eq!(oracle_observation.mode, StationaryDispatch::Runtime);
            assert_eq!(oracle_observation.route, route);
            assert_eq!(candidate_observation.route, route);
            assert_eq!(
                candidate_observation.mode,
                if block == 0 {
                    StationaryDispatch::Ramping
                } else {
                    StationaryDispatch::Stationary
                },
                "{label}: selected specialization block {block}"
            );
            populated |= candidate_left.iter().any(|sample| sample.to_bits() != 0);
            if !mono {
                populated |= candidate_right.iter().any(|sample| sample.to_bits() != 0);
            }
        }

        assert!(populated, "{label}: witness PCM never became populated");
    }

    /// The candidate and the shared runtime oracle agree on populated PCM and complete state while
    /// the actual selected body proves all four vector routes, at the build's own width (W8 in the
    /// 8-lane (AVX2) build, W4 in a 4-lane (NEON/simd128) one; #1112), and scalar dual/uniform
    /// specialization.
    #[test]
    fn stationary_dispatch_matches_runtime_oracle_and_observes_selected_body() {
        compare_dispatch_witness_case::<lane::Native>(
            "native dual per-lane",
            false,
            false,
            DispatchRoute::DualPerLane,
        );
        compare_dispatch_witness_case::<lane::Native>(
            "native dual uniform",
            true,
            false,
            DispatchRoute::DualUniform,
        );
        compare_dispatch_witness_case::<lane::Native>(
            "native mono per-lane",
            false,
            true,
            DispatchRoute::MonoPerLane,
        );
        compare_dispatch_witness_case::<lane::Native>(
            "native mono uniform",
            true,
            true,
            DispatchRoute::MonoUniform,
        );
        compare_dispatch_witness_case::<f32>(
            "scalar dual uniform",
            true,
            false,
            DispatchRoute::DualUniform,
        );
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
    /// Issue #182 S2, the headline gate, at every width the build has. The tone is well under the guarded
    /// ceiling (`limit = 10^(-7/20) = 0.447` against an amplitude of `0.05`), so `r = 1` at every
    /// frame and the recursive word never leaves `+0.0`. That is deliberate, and it is the same
    /// choice this limiter's tests make explicit: they isolate the retained windows as the state
    /// that has to drain, and they make the fixed point reachable inside a test-sized run.
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
        let mut arms = Vec::new();
        lane::each_lane!(|L, W| arms.push((
            W,
            compare_silence_arms::<L>(&width_label(W), &plan, -6.0, 100.0, 0.05)
        )));
        // The widest arm is the reference: W8 in the 8-lane (AVX2) build, W4 in a 4-lane
        // (NEON/simd128) one (issue #1112).
        let (lanes, widest) = arms.last().expect("every build has a vector width");

        // Engagement is a property of the signal and the shape, not of the width.
        for (width, arm) in &arms {
            assert_eq!(
                arm.engagements,
                widest.engagements,
                "engagement rate depends on the lane width ({})",
                width_label(*width)
            );
        }
        assert_eq!(
            widest.engagements, 35,
            "the fast path engaged on {} of {SILENT_BLOCKS} silent blocks, not the 35 the 486-sample \
             delay line allows",
            widest.engagements
        );

        // Anti-vacuity: the trailing tone is really rendered, so the comparison above had
        // something other than silence to compare. Without it the test would pass on a fast path
        // that simply stopped rendering.
        let per_block = 128 * lanes * 2;
        assert!(
            widest.rendered[widest.rendered.len() - per_block..]
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
        let mut arms = Vec::new();
        lane::each_lane!(|L, W| {
            let label = width_label(W);
            let arm = compare_silence_arms::<L>(&label, &plan, -6.0, 2_000.0, 3.0);
            arms.push((label, arm));
        });
        for (label, arm) in arms {
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

        // The anti-vacuity at the build's own width (issue #1112).
        let arm = compare_silence_arms::<lane::Native>("native", &plan, -6.0, 100.0, 0.05);
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
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
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

        // At the build's own width (issue #1112).
        let free = arm::<lane::Native>(false);
        let forced = arm::<lane::Native>(true);
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
            let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
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
    /// A restore refills the limiter's retained windows from the other side: withdrawal then
    /// invalidates the fixed-point claim before the next block can be skipped, even when the signal
    /// itself is quiet.
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
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
            .expect("metadata");
        // At the build's own bank width (issue #1112).
        type L = lane::Native;
        let width = BankWidth::for_lanes(L::WIDTH).expect("a bank width");
        let mut bank = PreparedTruePeakLimiterBank::<L> {
            metadata: PreparedBankMetadata {
                width,
                program_key: metadata.program_key(),
            },
            core: silent_core::<L>(-6.0, 100.0, 5.0),
        };

        let lanes = L::WIDTH;
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
                    width,
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
                width,
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
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
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

    /// E11: the current layout round-trips, and every corruption is rejected without mutating the peer.
    #[test]
    fn state_round_trips_and_rejects_corruption() {
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
            1,
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
                1,
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
                Box::new(|bytes: &mut Vec<u8>| write_u32(bytes, 0, 0)),
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
                0,
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
        // At this build's native width (#1017): a bank wider than the backend does not bind.
        let (width, backend, lanes) = native_bank();
        let values = initial_values();
        let members: Vec<_> = (0..lanes).map(|_| values).collect();
        let bank = bank_for(&members, LinkMode::DualMono, width, backend);
        let key = bank.metadata().program_key;
        assert_eq!(key.state_layout_version, 1);
        assert_eq!(key.state_sizes.left_bytes, 5_900);
        assert_eq!(key.state_sizes.common_bytes, 8);
        assert_eq!(key.state_sizes.total(), Some(11_808));
        assert_eq!(bank.metadata().width, width);

        // Mismatched backend and width are rejected before anything is prepared: the native width
        // with every other width's backend, `Simd4` at `Eight` in the 8-lane (AVX2) build. A 4-lane
        // (NEON/simd128) build has no second width (issue #1112).
        let requests: Vec<_> = members.iter().map(|values| request(values)).collect();
        for &other in BankWidth::ALL.iter().filter(|&&other| other != width) {
            let mismatched =
                TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: other.backend(),
                    width,
                    requests: &requests,
                    active_mask: width.full_mask(),
                });
            assert_eq!(
                mismatched.err().map(|error| error.code),
                Some("effect.bank.requests")
            );
        }

        // Issue #95 unification: a heterogeneous cohort is a cohort this artifact cannot bank,
        // not a malformed request. Every member is still validated first — the `Ok` proves the
        // decline happened after validation, not instead of it — and the answer is the same
        // `Ok(None)` every other effect gives, so the tracks render as scalar instances instead
        // of failing the session compile.
        let mut heterogeneous: Vec<PrepareEffectRequest<'_>> = requests.clone();
        heterogeneous[3].link_mode = LinkMode::Maximum;
        let heterogeneous =
            TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &heterogeneous,
                active_mask: width.full_mask(),
            });
        assert!(
            heterogeneous
                .expect("a heterogeneous cohort is declined, never an error")
                .is_none()
        );

        // A member that would fail `prepare` on its own is still a typed error, and it is the
        // diagnostic `prepare` would have returned — an absent capability must never hide it.
        let mut malformed: Vec<PrepareEffectRequest<'_>> = requests.clone();
        malformed[lanes - 1].quality = EffectQuality::Draft;
        assert_eq!(
            TruePeakLimiterFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &malformed,
                    active_mask: width.full_mask(),
                })
                .err()
                .map(|error| error.code),
            Some("effect.quality.unsupported")
        );
    }

    /// The bank widths this build binds: `Four` and `Eight` in the 8-lane (AVX2) build, `Four` in
    /// a 4-lane (NEON/simd128) one (#1017, #1112). A bank wider than the backend declines, so a
    /// padded bank is tested at every width that binds rather than returning early on the other.
    fn bank_widths() -> Vec<(BankWidth, Backend)> {
        let widths: Vec<(BankWidth, Backend)> = BankWidth::ALL
            .iter()
            .map(|&width| (width, width.backend()))
            .filter(|(width, _)| width.lanes() as usize <= Backend::current().width())
            .collect();
        assert!(
            widths.contains(&(native_bank().0, native_bank().1)),
            "the native width binds"
        );
        widths
    }

    /// `request` with the link mode of the bank under test.
    fn linked_request(
        values: &[InitialParameterValue],
        link: LinkMode,
    ) -> PrepareEffectRequest<'_> {
        let mut request = request(values);
        request.link_mode = link;
        request
    }

    /// One [`linked_request`] per lane.
    fn linked_requests(
        values: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        link: LinkMode,
    ) -> Vec<PrepareEffectRequest<'_>> {
        values
            .iter()
            .map(|values| linked_request(values, link))
            .collect()
    }

    /// Issue #1091 (console strip P2d): the limiter accepts padding. A padded request binds at
    /// every active count, and every lane, member or clone, is still validated before the bank is
    /// built or declined.
    ///
    /// Each lane is malformed in turn, the first included, so a validation that stops early or a
    /// decline hoisted above the member loop is red: a malformed lane after the first must be
    /// refused with `prepare`'s own code, even when a foreign clone would also make the cohort one
    /// this artifact declines. Red too if the #1088 decline comes back (no padded request binds).
    #[test]
    fn a_padded_request_binds_after_every_lane_is_validated() {
        let values = initial_values();
        for (width, backend) in bank_widths() {
            let lanes = width.lanes() as usize;
            let requests = vec![request(&values); lanes];
            let bind = |requests: &[PrepareEffectRequest<'_>], mask: &[bool]| {
                TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests,
                    active_mask: mask,
                })
            };
            for members in 1..lanes {
                let mask: Vec<bool> = (0..lanes).map(|lane| lane < members).collect();
                let label = format!("{width:?}, {members} of {lanes} lanes active");
                let bank = bind(&requests, &mask)
                    .expect("a padded request is well formed")
                    .unwrap_or_else(|| panic!("{label}: a padded bank binds"));
                assert_eq!(bank.metadata().width, width, "{label}");

                // A padded lane of another program is not a clone, and a cohort of two programs
                // is one this artifact declines rather than binds.
                let mut foreign = requests.clone();
                foreign[lanes - 1].link_mode = LinkMode::Maximum;
                assert!(
                    bind(&foreign, &mask)
                        .expect("a foreign clone is well formed")
                        .is_none(),
                    "{label}: a foreign clone declines"
                );
                for malformed_lane in 0..lanes {
                    let role = if malformed_lane < members {
                        "member"
                    } else {
                        "clone"
                    };
                    for base in [&requests, &foreign] {
                        let mut malformed = base.clone();
                        malformed[malformed_lane].quality = EffectQuality::Draft;
                        assert_eq!(
                            bind(&malformed, &mask).err().map(|error| error.code),
                            Some("effect.quality.unsupported"),
                            "{label}: a malformed {role} on lane {malformed_lane} is refused"
                        );
                    }
                }
            }
            // The factory validates the shape first: a member after a padded lane is refused.
            let mut mask = vec![true; lanes];
            mask[0] = false;
            assert_eq!(
                bind(&requests, &mask).err().map(|error| error.code),
                Some("effect.bank.mask_not_prefix"),
                "{width:?}: members come first"
            );
        }
    }

    /// Issue #1091: a padded lane carries no track, so a state payload never addresses one.
    ///
    /// Red if `checked_member` goes: the padded lane's words are snapshotted as a track's, and a
    /// restored payload leaves it off the rest state it must hold to answer `+0.0` with `+0.0`.
    #[test]
    fn a_padded_lane_has_no_state_payload() {
        let values = values_with(-6.0, 100.0, 5.0);
        for (width, backend) in bank_widths() {
            let lanes = width.lanes() as usize;
            let requests = vec![request(&values); lanes];
            let mask: Vec<bool> = (0..lanes).map(|lane| lane + 1 < lanes).collect();
            let mut bank = TruePeakLimiterFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                    active_mask: &mask,
                })
                .expect("well formed")
                .expect("binds");
            let payload = snapshot_track(bank.as_ref(), 0);
            let sizes = bank.metadata().program_key.state_sizes;
            let padded = (lanes - 1) as u32;
            let mut common = vec![0; sizes.common_bytes as usize];
            let mut left = vec![0; sizes.left_bytes as usize];
            let mut right = vec![0; sizes.right_bytes as usize];
            assert_eq!(
                bank.snapshot_track_state_payload(
                    padded,
                    StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)
                        .expect("sizes"),
                )
                .err()
                .map(|error| error.code),
                Some("effect.state.track"),
                "{width:?}: a padded lane has no snapshot"
            );
            let input = || {
                StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes")
            };
            assert_eq!(
                bank.restore_track_state_payload(padded, STATE_LAYOUT_VERSION, input())
                    .err()
                    .map(|error| error.code),
                Some("effect.state.track"),
                "{width:?}: nothing is restored into a padded lane"
            );
            bank.restore_track_state_payload(0, STATE_LAYOUT_VERSION, input())
                .expect("a member still restores");
        }
    }

    /// Issue #1091: automation routed to a padded lane is neither applied nor charged to it.
    ///
    /// The planner routes a padded lane none, so this is the effect's half of "nothing is ever
    /// charged to a padded lane": its `BankProcessReport` entry stays empty whatever arrives. Red
    /// if the padded-lane skip in `process_bank_inner` goes, when the invalid span is counted
    /// against the padded lane.
    #[test]
    fn automation_routed_to_a_padded_lane_is_not_charged() {
        let values = initial_values();
        for (width, backend) in bank_widths() {
            let lanes = width.lanes() as usize;
            let requests = vec![request(&values); lanes];
            let mask: Vec<bool> = (0..lanes).map(|lane| lane == 0).collect();
            let mut bank = TruePeakLimiterFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &requests,
                    active_mask: &mask,
                })
                .expect("well formed")
                .expect("binds");
            // One invalid span (`Both` is never a limiter channel) on the member and one on the
            // last, padded, lane.
            let span = point_span(0, 0, ParameterChannel::Both, -3.0);
            let spans = [span, span];
            let offsets: Vec<u32> = (0..=lanes)
                .map(|lane| match lane {
                    0 => 0,
                    lane if lane == lanes => 2,
                    _ => 1,
                })
                .collect();
            let mut left = vec![0.0_f32; 128 * lanes];
            let mut right = vec![0.0_f32; 128 * lanes];
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left, &mut right, None, 128, width, 0, &spans, &offsets, 128,
                )
                .expect("bank block"),
            );
            assert_eq!(
                report.reports[0].invalid_spans, 1,
                "{width:?}: the member's"
            );
            for lane in 1..lanes {
                assert_eq!(
                    report.reports[lane],
                    ProcessReport::default(),
                    "{width:?}: padded lane {lane}"
                );
            }
        }
    }

    /// One dual block and, on a second bank, one collapsed block of a bank bound from `requests`
    /// and `mask`, and the body each took. Active lanes carry loud noise; padded lanes `+0.0`.
    fn padded_routes(
        width: BankWidth,
        backend: Backend,
        requests: &[PrepareEffectRequest<'_>],
        mask: &[bool],
    ) -> Result<(DispatchRoute, DispatchRoute), EffectPrepareError> {
        let lanes = width.lanes() as usize;
        let mut routes = [DispatchRoute::Unset; 2];
        for (mono, route) in [false, true].into_iter().zip(routes.iter_mut()) {
            let mut bank = TruePeakLimiterFactory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests,
                    active_mask: mask,
                })?
                .expect("a padded bank of one program binds");
            let mut noise = Noise(0x1091_0002);
            let mut left = vec![0.0_f32; 128 * lanes];
            let mut right = vec![0.0_f32; 128 * lanes];
            for (word, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
                if mask[word % lanes] {
                    *left = noise.next() * 3.0;
                    *right = noise.next() * 3.0;
                }
            }
            let offsets = vec![0_u32; lanes + 1];
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                128,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank block");
            clear_dispatch_observation();
            if mono {
                bank.process_bank_mono(block);
            } else {
                bank.process_bank(block);
            }
            *route = dispatch_observation().route;
        }
        Ok((routes[0], routes[1]))
    }

    /// Issue #1091 gate 2: a padded bank of members that share one window shape and one phase
    /// takes the uniform body, dual and collapsed, exactly as a full bank of them does.
    ///
    /// The members carry a 3 ms lookahead, off the 5 ms default, so the arms discriminate. A padded
    /// lane seeded from the descriptor defaults (5 ms) drags the whole bank onto the per-lane body:
    /// that is what this test turns red on if the binding stops seeding a padded lane from its
    /// clone, and the arm below shows the observation can see it. A padded lane of zeros is outside
    /// the release domain and is refused at bind, so it takes no body at all.
    #[test]
    fn a_padded_bank_of_uniform_members_takes_the_uniform_body() {
        use DispatchRoute::{DualPerLane, DualUniform, MonoPerLane, MonoUniform};
        let defaults = initial_values();
        let zeros = values_with(0.0, 0.0, 0.0);
        for (width, backend) in bank_widths() {
            let lanes = width.lanes() as usize;
            let members: Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> = (0..lanes)
                .map(|track| {
                    values_with(-3.0 - 0.5 * track as f32, 50.0 + 10.0 * track as f32, 3.0)
                })
                .collect();
            for link in [LinkMode::DualMono, LinkMode::Maximum] {
                let full = linked_requests(&members, link);
                assert_eq!(
                    padded_routes(width, backend, &full, width.full_mask()).expect("full"),
                    (DualUniform, MonoUniform),
                    "{width:?} {link:?}: the full bank, the control"
                );
                for active in 1..lanes {
                    let mask: Vec<bool> = (0..lanes).map(|lane| lane < active).collect();
                    let label = format!("{width:?} {link:?}, {active} of {lanes} active");
                    let padded = |padding: &[InitialParameterValue; PARAMETER_COUNT * 2]| {
                        (0..lanes)
                            .map(|lane| {
                                if lane < active {
                                    members[lane]
                                } else {
                                    *padding
                                }
                            })
                            .collect::<Vec<_>>()
                    };
                    let clone = padded(&members[0]);
                    assert_eq!(
                        padded_routes(width, backend, &linked_requests(&clone, link), &mask)
                            .expect("clone"),
                        (DualUniform, MonoUniform),
                        "{label}: padded with a clone"
                    );
                    let default_lanes = padded(&defaults);
                    assert_eq!(
                        padded_routes(
                            width,
                            backend,
                            &linked_requests(&default_lanes, link),
                            &mask
                        )
                        .expect("defaults"),
                        (DualPerLane, MonoPerLane),
                        "{label}: padded with the descriptor defaults"
                    );
                    let zero_lanes = padded(&zeros);
                    assert!(
                        padded_routes(width, backend, &linked_requests(&zero_lanes, link), &mask)
                            .is_err(),
                        "{label}: padded with zeros"
                    );
                }
            }
        }
    }

    /// Plants a NaN in `lane`'s recursive word on both channels: non-finite state, which the next
    /// block carries to the output and the §4.4 check catches.
    fn plant_nonfinite<L: Lane>(core: &mut LimiterCore<L>, lane: usize) {
        core.left.reduction[lane] = f32::NAN;
        core.right.reduction[lane] = f32::NAN;
    }

    /// A padded core of `L::WIDTH` lanes and the scalar twin of each member, driven block by block.
    ///
    /// Members sit on lanes `0..active` (the planner's layout) and every padded lane carries a
    /// clone of member 0. The bank's planes are its resident block: a padded lane starts at `+0.0`
    /// and is fed, each block, whatever the bank left in it, as `rack::BankChain` does.
    struct PaddedRun<L: Lane> {
        bank: LimiterCore<L>,
        twins: Vec<LimiterCore<f32>>,
        left: Vec<f32>,
        right: Vec<f32>,
        draw: Draw,
        block: usize,
        label: String,
    }

    impl<L: Lane> PaddedRun<L> {
        fn new(
            members: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
            active: usize,
            link: LinkMode,
            label: String,
        ) -> Self {
            let lanes = L::WIDTH;
            let tracks: Vec<_> = (0..lanes)
                .map(|lane| members[if lane < active { lane } else { 0 }])
                .collect();
            let bank = linked_core::<L>(&tracks, link, false, 48_000)
                .with_active_lanes(every_lane(active));
            let twins = members[..active]
                .iter()
                .map(|values| {
                    linked_core::<f32>(core::slice::from_ref(values), link, false, 48_000)
                })
                .collect();
            Self {
                bank,
                twins,
                left: vec![0.0; 128 * lanes],
                right: vec![0.0; 128 * lanes],
                draw: Draw(0x1091_0004 + active as u64),
                block: 0,
                label,
            }
        }

        /// One block of loud noise on every member, dual or collapsed (a collapsed block's two
        /// channels carry one signal). Every member must render its twin's bits and every padded
        /// lane `+0.0`.
        fn render(&mut self, mono: bool, what: &str) {
            let lanes = L::WIDTH;
            let mut expected = Vec::with_capacity(self.twins.len());
            for (member, twin) in self.twins.iter_mut().enumerate() {
                let mut left = vec![0.0_f32; 128];
                let mut right = vec![0.0_f32; 128];
                for frame in 0..128 {
                    left[frame] = self.draw.unit() * 3.0;
                    right[frame] = if mono {
                        left[frame]
                    } else {
                        self.draw.unit() * 3.0
                    };
                    self.left[frame * lanes + member] = left[frame];
                    self.right[frame * lanes + member] = right[frame];
                }
                twin.process_block(&mut left, &mut right, 128);
                expected.push((left, right));
            }
            if mono {
                self.bank.process_block_mono(&mut self.left, 128);
            } else {
                self.bank
                    .process_block(&mut self.left, &mut self.right, 128);
            }
            let label = format!("{}: block {} ({what})", self.label, self.block);
            // #990: a collapsed block leaves the linked record cleared, a recovery included, since
            // only its left channel advanced.
            assert!(
                !(mono && self.bank.gain_linked),
                "{label}: collapsed but linked"
            );
            let lane_of = |plane: &[f32], lane: usize| -> Vec<f32> {
                (0..128).map(|frame| plane[frame * lanes + lane]).collect()
            };
            for (member, (left, right)) in expected.iter().enumerate() {
                assert_same_words(
                    &lane_of(&self.left, member),
                    left,
                    &format!("{label}: member {member} left"),
                );
                if !mono {
                    assert_same_words(
                        &lane_of(&self.right, member),
                        right,
                        &format!("{label}: member {member} right"),
                    );
                }
            }
            for lane in self.twins.len()..lanes {
                for plane in [&self.left, &self.right] {
                    assert!(
                        lane_of(plane, lane).iter().all(|word| word.to_bits() == 0),
                        "{label}: padded lane {lane} is not +0.0"
                    );
                }
                for (name, state) in [("left", &self.bank.left), ("right", &self.bank.right)] {
                    assert!(
                        lane_at_rest(state, lane),
                        "{label}: padded lane {lane} {name} is off its rest state"
                    );
                }
            }
            self.block += 1;
        }
    }

    /// `true` when lane `lane` of `state` holds exactly the words `clear_runtime` writes, the van
    /// Herk phase aside: [`ChannelState::is_at_silent_rest`] for one lane. Every word is finite.
    fn lane_at_rest(state: &ChannelState, lane: usize) -> bool {
        let width = state.width;
        let all = |plane: &[f32], value: f32| {
            plane
                .iter()
                .skip(lane)
                .step_by(width)
                .all(|word| word.to_bits() == value.to_bits())
        };
        all(&state.history, 0.0)
            && all(&state.main_ring, 0.0)
            && all(&state.required_ring, 1.0)
            && all(&state.box_ring, 1.0)
            && state.reduction[lane].to_bits() == 0
            && state.prefix[lane].to_bits() == 1.0_f32.to_bits()
            && state.box_sum[lane].to_bits() == (state.lane[lane].window as f32).to_bits()
    }

    /// Issue #1091 gate 3, the P2a verdict's clause (L4): a padded lane fed `+0.0` answers exactly
    /// `+0.0` (never `-0.0`, never a subnormal) and keeps its state finite and at rest, block after
    /// block and through the `N + 6` lookahead, while its bank-mates limit hard under either link,
    /// dual and collapsed.
    ///
    /// [`PaddedRun::render`] checks both after every block, and the members against their twins.
    /// Red if a padded lane is seeded off its rest state, if a whole-bank body writes a padded lane
    /// from a member's words, or if the recovery leaves a signed zero in a line.
    #[test]
    fn a_padded_lane_stays_at_rest_through_the_lookahead() {
        fn at<L: Lane>() {
            let lanes = L::WIDTH;
            let members = fixture_tracks(lanes);
            for link in [LinkMode::DualMono, LinkMode::Maximum] {
                for mono in [false, true] {
                    for active in 1..lanes {
                        let mut run = PaddedRun::<L>::new(
                            &members,
                            active,
                            link,
                            format!("W{lanes} {link:?} mono {mono}, {active} active"),
                        );
                        for _ in 0..12 {
                            run.render(mono, "limiting");
                        }
                    }
                }
            }
        }
        lane::each_vector_lane!(|L| at::<L>());
    }

    /// Issue #1091 gate 4 at one width: a non-finite state planted in one active lane is
    /// recovered and reported for that lane alone, dual and collapsed, with the bank-mates'
    /// bits and the padded lanes' `+0.0` intact.
    ///
    /// The oracle is each member's scalar twin, planted the same way. A twin's recovery is today's
    /// whole reset at cursor zero, so the failed lane's blocks after the failure also prove that a
    /// lane reset under the bank's running cursors renders what a fresh twin renders.
    fn a_failed_lane_is_recovered_and_reported_alone_at<L: Lane>() {
        let lanes = L::WIDTH;
        let members = fixture_tracks(lanes);
        let shape = Shape::new(48_000).expect("shape");
        for link in [LinkMode::DualMono, LinkMode::Maximum] {
            for mono in [false, true] {
                for active in 1..=lanes {
                    let label = |case: &str| {
                        format!("W{lanes} {link:?} mono {mono}, {active} active: {case}")
                    };
                    let mut targets = vec![0, active - 1];
                    targets.dedup();
                    for target in targets {
                        let mut run = PaddedRun::<L>::new(
                            &members,
                            active,
                            link,
                            label(&format!("member {target} fails")),
                        );
                        for _ in 0..4 {
                            run.render(mono, "before");
                        }
                        let mut running = run.bank.cursors;
                        running.advance(128, &shape);
                        plant_nonfinite(&mut run.bank, target);
                        plant_nonfinite(&mut run.twins[target], 0);
                        run.render(mono, "failing");
                        assert_eq!(
                            run.bank.nonfinite_report(),
                            NonFiniteReport {
                                nonfinite_blocks: 1,
                                nonfinite_lanes: 1 << target,
                            },
                            "{}: reported alone",
                            run.label
                        );
                        for (member, twin) in run.twins.iter().enumerate() {
                            assert_eq!(
                                twin.nonfinite_report().nonfinite_blocks,
                                u64::from(member == target),
                                "{}: twin {member}",
                                run.label
                            );
                        }
                        // A bank of one member failed on every active lane: the whole reset.
                        // Otherwise the shared cursors run on.
                        let cursors = if active == 1 {
                            Cursors::default()
                        } else {
                            running
                        };
                        assert_eq!(run.bank.cursors, cursors, "{}: cursors", run.label);
                        for _ in 0..6 {
                            run.render(mono, "after");
                        }
                    }
                    if active < lanes {
                        let mut run = PaddedRun::<L>::new(
                            &members,
                            active,
                            link,
                            label("a padded lane fails"),
                        );
                        for _ in 0..4 {
                            run.render(mono, "before");
                        }
                        plant_nonfinite(&mut run.bank, lanes - 1);
                        for _ in 0..4 {
                            run.render(mono, "after");
                        }
                        assert_eq!(
                            run.bank.nonfinite_report(),
                            NonFiniteReport::new(),
                            "{}: a padded lane is neither reported nor charged",
                            run.label
                        );
                        assert_eq!(
                            run.bank.left.reduction[lanes - 1].to_bits(),
                            0,
                            "{}: the padded lane was recovered",
                            run.label
                        );
                    }
                    let mut run =
                        PaddedRun::<L>::new(&members, active, link, label("every member fails"));
                    for _ in 0..4 {
                        run.render(mono, "before");
                    }
                    for member in 0..active {
                        plant_nonfinite(&mut run.bank, member);
                        plant_nonfinite(&mut run.twins[member], 0);
                    }
                    run.render(mono, "failing");
                    assert_eq!(
                        run.bank.nonfinite_report(),
                        NonFiniteReport {
                            nonfinite_blocks: 1,
                            nonfinite_lanes: every_lane(active),
                        },
                        "{}: reported for every member",
                        run.label
                    );
                    assert_eq!(
                        run.bank.cursors,
                        Cursors::default(),
                        "{}: the whole reset",
                        run.label
                    );
                    for _ in 0..6 {
                        run.render(mono, "after");
                    }
                }
            }
        }
    }

    /// Issue #1091 gate 4: see [`a_failed_lane_is_recovered_and_reported_alone_at`].
    ///
    /// Red mutations: the old whole-bank recovery (`reset_failed_lanes` always takes its first
    /// branch) zeroes and resets the bank-mates; an unmasked report charges the padded lane; a
    /// recovery that skips padded lanes leaves one answering NaN; resetting the shared cursors on
    /// a partial recovery moves every bank-mate's ring reads.
    #[test]
    fn a_failed_lane_is_recovered_and_reported_alone() {
        lane::each_vector_lane!(|L| a_failed_lane_is_recovered_and_reported_alone_at::<L>());
    }

    /// Every word lane `lane` of `state` holds, designed and running, by bit pattern.
    fn lane_words(state: &ChannelState, lane: usize) -> Vec<u32> {
        let width = state.width;
        let strided = |plane: &[f32]| -> Vec<u32> {
            plane
                .iter()
                .skip(lane)
                .step_by(width)
                .map(|word| word.to_bits())
                .collect()
        };
        let mut words = Vec::new();
        for plane in [
            &state.history,
            &state.main_ring,
            &state.required_ring,
            &state.box_ring,
        ] {
            words.extend(strided(plane));
        }
        words.extend([
            state.reduction[lane].to_bits(),
            state.prefix[lane].to_bits(),
            state.box_sum[lane].to_bits(),
            state.phase[lane],
            state.lookahead_ms[lane].to_bits(),
            state.lane[lane].window,
            state.lane[lane].end_offset,
            state.lane[lane].box_offset,
        ]);
        for ramp in [state.limit[lane], state.release[lane]] {
            words.extend([
                ramp.current.to_bits(),
                ramp.target.to_bits(),
                ramp.step.to_bits(),
                ramp.remaining,
            ]);
        }
        words
    }

    /// Issue #1091: a lane reset is the whole reset at one lane's stride, and touches no other lane.
    ///
    /// Each lane carries its own ceiling, release and split lookahead, is rendered off its rest
    /// state on loud noise and has a ramp in flight, so every word the whole reset writes differs
    /// before it. Red if `clear_lane_runtime` or `seed_lane_defaults` drops a word
    /// `clear_runtime` or `reset_to_defaults` writes (`prefix` left unreset, say), or writes
    /// another lane's.
    #[test]
    fn a_lane_reset_is_the_whole_reset_at_one_lanes_stride() {
        fn at<L: Lane>() {
            let lanes = L::WIDTH;
            let tracks: Vec<_> = (0..lanes)
                .map(|lane| {
                    values_split(
                        -2.0 - lane as f32,
                        30.0 + 7.0 * lane as f32,
                        1.0 + lane as f32,
                        9.0 - lane as f32,
                    )
                })
                .collect();
            let shape = Shape::new(48_000).expect("shape");
            let dirty = || {
                let mut core = linked_core::<L>(&tracks, LinkMode::DualMono, false, 48_000);
                let mut noise = Noise(0x1091_0005);
                for block in 0..3 {
                    let mut left: Vec<f32> = (0..128 * lanes).map(|_| noise.next() * 3.0).collect();
                    let mut right: Vec<f32> =
                        (0..128 * lanes).map(|_| noise.next() * 3.0).collect();
                    let first = (block * 128) as u64;
                    let spans: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
                        .map(|_| {
                            vec![
                                point_span(first, 0, ParameterChannel::Left, -18.0),
                                point_span(first, 1, ParameterChannel::Right, 900.0),
                            ]
                        })
                        .collect();
                    drive_dual(&mut core, &mut left, &mut right, &spans, first);
                }
                core
            };
            let rate = 48_000;
            let mut whole = dirty();
            whole
                .left
                .reset_to_defaults(&shape, &whole.left_defaults, rate);
            whole
                .right
                .reset_to_defaults(&shape, &whole.right_defaults, rate);
            let untouched = dirty();
            for target in 0..lanes {
                let mut one = dirty();
                one.left
                    .reset_lane_to_defaults(target, &shape, &one.left_defaults[target], rate);
                one.right
                    .reset_lane_to_defaults(target, &shape, &one.right_defaults[target], rate);
                for lane in 0..lanes {
                    let channels = [
                        ("left", &one.left, &whole.left, &untouched.left),
                        ("right", &one.right, &whole.right, &untouched.right),
                    ];
                    for (name, one, whole, untouched) in channels {
                        let expected = if lane == target {
                            lane_words(whole, lane)
                        } else {
                            assert_ne!(
                                lane_words(untouched, lane),
                                lane_words(whole, lane),
                                "W{lanes} lane {lane} {name}: rendered off its rest state"
                            );
                            lane_words(untouched, lane)
                        };
                        assert_eq!(
                            lane_words(one, lane),
                            expected,
                            "W{lanes}: lane {target} reset, lane {lane} {name}"
                        );
                    }
                }
            }
        }
        lane::each_lane!(|L| at::<L>());
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

    // ---------------------------------------------------------------------------------------
    // Issue #990: the linked gain computer.
    //
    // Gate 1 is an identity harness. Every block runs through the shipped kernel and through
    // `reference_block`, a verbatim copy of the kernel as it stood before #990, kept here in test
    // code; everything a host or a snapshot can observe is compared after every block, and the
    // linked-agreement invariant is read straight off the shipped core. Gate 2 is the engagement
    // witness: `LINKED_ENGAGEMENTS` counts the uniform blocks that actually took the linked body.
    // ---------------------------------------------------------------------------------------

    /// The kernel's block entry before issue #990 (`limiter_block`), verbatim.
    ///
    /// The oracle arm of gate 1, reached through `REFERENCE_KERNEL`. It lives in test code so that
    /// the oracle is the kernel that shipped before the change rather than a switch inside the code
    /// under test: [`reference_block_uniform`] is the pre-#990 `limiter_block_uniform` token for
    /// token, and a ragged cohort goes to the per-lane body, which #990 does not touch.
    /// `#[inline(never)]` keeps the copy's frame out of `process_block`'s in the dev profile.
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn reference_block<L: Lane>(
        left_io: &mut [f32],
        right_io: &mut [f32],
        frames: usize,
        coef: &LimiterCoef<L>,
        shape: &Shape,
        left: &mut ChannelState,
        right: &mut ChannelState,
        cursors: &mut Cursors,
    ) {
        let stationary = dual_stationary(left, right);
        if lanes_uniform(left) && lanes_uniform(right) {
            if stationary {
                reference_block_uniform::<DISPATCH_STATIONARY, L>(
                    left_io, right_io, frames, coef, shape, left, right, cursors, true,
                );
            } else {
                reference_block_uniform::<DISPATCH_RAMPING, L>(
                    left_io, right_io, frames, coef, shape, left, right, cursors, false,
                );
            }
        } else if stationary {
            limiter_block_per_lane::<DISPATCH_STATIONARY, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, true,
            );
        } else {
            limiter_block_per_lane::<DISPATCH_RAMPING, L>(
                left_io, right_io, frames, coef, shape, left, right, cursors, false,
            );
        }
    }

    /// `limiter_block_uniform` before issue #990, verbatim but for its name and its inlining.
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    fn reference_block_uniform<const DISPATCH: u8, L: Lane>(
        left_io: &mut [f32],
        right_io: &mut [f32],
        frames: usize,
        coef: &LimiterCoef<L>,
        shape: &Shape,
        left: &mut ChannelState,
        right: &mut ChannelState,
        cursors: &mut Cursors,
        stationary: bool,
    ) {
        let width = L::WIDTH;
        debug_assert!(width <= MAXIMUM_WIDTH);
        debug_assert_eq!(left.width, width);
        debug_assert_eq!(right.width, width);
        debug_assert_eq!(left_io.len(), frames * width);
        debug_assert_eq!(right_io.len(), frames * width);

        let mut hot_left = HotChannel::<L>::load(left);
        let mut hot_right = HotChannel::<L>::load(right);
        #[cfg(test)]
        observe_dispatch::<DISPATCH>(DispatchRoute::DualUniform);
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
                let active_base = chunk * width;
                let active_words = span * width;
                detector_chunk::<L>(
                    &mut hot_left.history,
                    &left_io[active_base..active_base + active_words],
                    &coef.fir,
                    &mut peaks_left[..active_words],
                );
                detector_chunk::<L>(
                    &mut hot_right.history,
                    &right_io[active_base..active_base + active_words],
                    &coef.fir,
                    &mut peaks_right[..active_words],
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
                        let (limit_left, release_left) = ramp_values::<DISPATCH, L>(
                            stationary,
                            &mut hot_left.limit,
                            &mut hot_left.release,
                        );
                        let (limit_right, release_right) = ramp_values::<DISPATCH, L>(
                            stationary,
                            &mut hot_right.limit,
                            &mut hot_right.release,
                        );

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

    /// SplitMix64 draws for the #990 scenarios, so a scenario is a seed and never a file.
    struct Draw(u64);

    impl Draw {
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut mixed = self.0;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            mixed ^ (mixed >> 31)
        }

        fn below(&mut self, bound: usize) -> usize {
            (self.next_u64() % bound as u64) as usize
        }

        fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
            self.below(denominator) < numerator
        }

        fn pick<T: Copy>(&mut self, items: &[T]) -> T {
            items[self.below(items.len())]
        }

        /// Uniform in `[-1, 1)`, on the 2^-23 grid.
        fn unit(&mut self) -> f32 {
            ((self.next_u64() >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0
        }
    }

    /// The gate-1 block lengths: around the detector chunk (32), the cohort width and one frame.
    const LINKED_BLOCK_LENGTHS: [usize; 11] = [1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128];

    /// +3 dBFS peak: every lane of every fixture ceiling limits on this.
    const HOT: f32 = 1.412_537_5;

    /// What one block of the #990 harness carries on its two planes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum LinkedSignal {
        /// Noise at 0.2 peak: below every ceiling the harness prepares.
        Quiet,
        /// Noise at +3 dBFS on both channels.
        Hot,
        /// One channel hot, the other at -34 dBFS; which one is drawn per block.
        Asymmetric,
        /// Exact `+0.0`.
        Silence,
        /// Exact `-0.0`.
        NegativeZero,
        /// Random subnormals of either sign.
        Subnormal,
        /// Each lane's own limit, and the largest `f32` below it, with random signs.
        Threshold,
        /// Silence but for one loud sample in one lane of one channel.
        Spike,
        /// Magnitudes log-uniform over `2^-24 ..= 1`.
        LogUniform,
        /// Hot noise with one NaN, infinity or `1e30` in it.
        NonFinite,
    }

    const LINKED_SIGNALS: [LinkedSignal; 10] = [
        LinkedSignal::Quiet,
        LinkedSignal::Hot,
        LinkedSignal::Asymmetric,
        LinkedSignal::Silence,
        LinkedSignal::NegativeZero,
        LinkedSignal::Subnormal,
        LinkedSignal::Threshold,
        LinkedSignal::Spike,
        LinkedSignal::LogUniform,
        LinkedSignal::NonFinite,
    ];

    /// Both planes of one block of `frames` frames, lane-interleaved at `L::WIDTH`.
    fn linked_planes<L: Lane>(
        signal: LinkedSignal,
        draw: &mut Draw,
        frames: usize,
        core: &LimiterCore<L>,
    ) -> (Vec<f32>, Vec<f32>) {
        let width = L::WIDTH;
        let words = frames * width;
        let loud_left = draw.chance(1, 2);
        let mut planes = [vec![0.0_f32; words], vec![0.0_f32; words]];
        for (channel, plane) in planes.iter_mut().enumerate() {
            for (word, sample) in plane.iter_mut().enumerate() {
                let lane = word % width;
                *sample = match signal {
                    LinkedSignal::Quiet => draw.unit() * 0.2,
                    LinkedSignal::Hot | LinkedSignal::NonFinite => draw.unit() * HOT,
                    LinkedSignal::Asymmetric => {
                        if (channel == 0) == loud_left {
                            draw.unit() * HOT
                        } else {
                            draw.unit() * 0.02
                        }
                    }
                    LinkedSignal::Silence => 0.0,
                    LinkedSignal::NegativeZero => -0.0,
                    LinkedSignal::Subnormal => {
                        let bits = draw.next_u64() as u32;
                        f32::from_bits(bits & 0x807F_FFFF)
                    }
                    LinkedSignal::Threshold => {
                        let limit = [&core.left, &core.right][channel].limit[lane].current;
                        let below = f32::from_bits(limit.to_bits().wrapping_sub(1));
                        let magnitude = if draw.chance(1, 2) { limit } else { below };
                        if draw.chance(1, 2) {
                            magnitude
                        } else {
                            -magnitude
                        }
                    }
                    LinkedSignal::Spike => 0.0,
                    LinkedSignal::LogUniform => {
                        let exponent = draw.unit().abs() * -24.0;
                        let magnitude = f32::from_bits(((127.0 + exponent) * 8_388_608.0) as u32);
                        if draw.chance(1, 2) {
                            magnitude
                        } else {
                            -magnitude
                        }
                    }
                };
            }
        }
        match signal {
            LinkedSignal::Spike => {
                let channel = draw.below(2);
                let word = draw.below(words);
                planes[channel][word] = if draw.chance(1, 2) { 3.0 } else { -3.0 };
            }
            LinkedSignal::NonFinite => {
                let channel = draw.below(2);
                let word = draw.below(words);
                planes[channel][word] =
                    draw.pick(&[f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 1.0e30]);
            }
            _ => {}
        }
        let [left, right] = planes;
        (left, right)
    }

    /// One track's six initial values with a separate left and right `[ceiling, release,
    /// lookahead]`.
    fn linked_values(
        left: [f32; PARAMETER_COUNT],
        right: [f32; PARAMETER_COUNT],
    ) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
        let mut values = values_with(left[0], left[1], left[2]);
        values[1].value = right[0];
        values[3].value = right[1];
        values[5].value = right[2];
        values
    }

    /// The console fixture's limiter shape, `channel: "both"`, one track per lane.
    fn fixture_tracks(width: usize) -> Vec<[InitialParameterValue; PARAMETER_COUNT * 2]> {
        (0..width)
            .map(|track| {
                let values = [
                    -0.5 - 0.031_25 * track as f32,
                    60.0 + 1.25 * track as f32,
                    5.0,
                ];
                linked_values(values, values)
            })
            .collect()
    }

    fn linked_core<L: Lane>(
        tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        link: LinkMode,
        bypass: bool,
        rate: u32,
    ) -> LimiterCore<L> {
        assert_eq!(tracks.len(), L::WIDTH);
        let mut preparation = request_at_rate(&tracks[0], rate);
        preparation.link_mode = link;
        preparation.bypass = bypass;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
            .expect("metadata");
        let mut left_defaults = Vec::with_capacity(L::WIDTH);
        let mut right_defaults = Vec::with_capacity(L::WIDTH);
        for values in tracks {
            let (left, right) = initial_defaults(values).expect("defaults");
            left_defaults.push(left);
            right_defaults.push(right);
        }
        LimiterCore::<L>::new(
            metadata,
            left_defaults.into_boxed_slice(),
            right_defaults.into_boxed_slice(),
        )
        .expect("core")
    }

    /// `process_bank_inner`'s dual arm (and `PreparedTruePeakLimiter::process`), over a bare core.
    fn drive_dual<L: Lane>(
        core: &mut LimiterCore<L>,
        left: &mut [f32],
        right: &mut [f32],
        spans: &[Vec<PreparedAutomationSpan>],
        first_sample: u64,
    ) -> Vec<ProcessReport> {
        if spans.iter().any(|lane| !lane.is_empty()) {
            core.silent_fixed_point = false;
        }
        let mut reports = vec![ProcessReport::default(); L::WIDTH];
        for (lane, lane_spans) in spans.iter().enumerate() {
            apply_automation(
                lane_spans,
                &core.metadata,
                first_sample,
                &mut core.left,
                &mut core.right,
                lane,
                &mut reports[lane],
            );
        }
        let frames = left.len() / L::WIDTH;
        core.process_block(left, right, frames);
        reports
    }

    fn core_snapshot<L: Lane>(core: &LimiterCore<L>, track: usize) -> LanePayload {
        let sizes = core.metadata.state_sizes;
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        core.snapshot_track(
            track,
            &mut StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("snapshot");
        (common, left, right)
    }

    fn core_restore<L: Lane>(
        core: &mut LimiterCore<L>,
        track: usize,
        payload: &LanePayload,
    ) -> Result<(), StatePayloadError> {
        let sizes = core.metadata.state_sizes;
        core.restore_track(
            track,
            STATE_LAYOUT_VERSION,
            &StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes"),
        )
    }

    /// Class-A identity: the same bits, with every NaN one value (#1065; the §4.4 check zeroes a
    /// non-finite block before a host sees it, so this is the harness being general rather than a
    /// case it expects).
    fn assert_same_words(shipped: &[f32], oracle: &[f32], what: &str) {
        assert_eq!(shipped.len(), oracle.len(), "{what}: length");
        for (index, (a, b)) in shipped.iter().zip(oracle.iter()).enumerate() {
            assert!(
                dsp_reference::class_a::same(*a, *b),
                "{what}: word {index}: shipped {:#010x}, reference {:#010x}",
                a.to_bits(),
                b.to_bits()
            );
        }
    }

    /// The two arms of gate 1: the shipped kernel and the reference kernel, one core each, fed the
    /// same blocks, the same automation and the same control calls.
    struct LinkedPair<L: Lane> {
        shipped: LimiterCore<L>,
        oracle: LimiterCore<L>,
        first_sample: u64,
        /// Uniform dual blocks the shipped core rendered linked.
        engaged: u32,
        /// Dual blocks the shipped core rendered.
        rendered: u32,
        label: String,
    }

    impl<L: Lane> LinkedPair<L> {
        fn new(
            tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
            link: LinkMode,
            bypass: bool,
            rate: u32,
            label: String,
        ) -> Self {
            let pair = Self {
                shipped: linked_core::<L>(tracks, link, bypass, rate),
                oracle: linked_core::<L>(tracks, link, bypass, rate),
                first_sample: 0,
                engaged: 0,
                rendered: 0,
                label,
            };
            pair.compare("prepared");
            pair
        }

        /// Renders one dual block through both arms and compares everything. Returns whether the
        /// shipped core took the linked body.
        fn dual(
            &mut self,
            left: &[f32],
            right: &[f32],
            spans: &[Vec<PreparedAutomationSpan>],
            at: &str,
        ) -> bool {
            let (mut shipped_left, mut shipped_right) = (left.to_vec(), right.to_vec());
            let (mut oracle_left, mut oracle_right) = (left.to_vec(), right.to_vec());
            let before = LINKED_ENGAGEMENTS.with(Cell::get);
            let shipped_reports = drive_dual(
                &mut self.shipped,
                &mut shipped_left,
                &mut shipped_right,
                spans,
                self.first_sample,
            );
            let after = LINKED_ENGAGEMENTS.with(Cell::get);
            REFERENCE_KERNEL.with(|reference| reference.set(true));
            let oracle_reports = drive_dual(
                &mut self.oracle,
                &mut oracle_left,
                &mut oracle_right,
                spans,
                self.first_sample,
            );
            REFERENCE_KERNEL.with(|reference| reference.set(false));
            assert_eq!(
                LINKED_ENGAGEMENTS.with(Cell::get),
                after,
                "{} {at}: the reference kernel linked a pair",
                self.label
            );
            let label = format!("{} {at}", self.label);
            assert_same_words(
                &shipped_left,
                &oracle_left,
                &format!("{label}: left output"),
            );
            assert_same_words(
                &shipped_right,
                &oracle_right,
                &format!("{label}: right output"),
            );
            assert_eq!(shipped_reports, oracle_reports, "{label}: process reports");
            self.first_sample += (left.len() / L::WIDTH) as u64;
            let engaged = after != before;
            self.engaged += u32::from(engaged);
            self.rendered += 1;
            self.compare(at);
            engaged
        }

        /// Renders one collapsed block (the left plane) through both arms, which share the mono
        /// body: #990 does not touch it.
        fn mono(&mut self, left: &[f32], at: &str) {
            let frames = left.len() / L::WIDTH;
            let mut shipped_left = left.to_vec();
            let mut oracle_left = left.to_vec();
            let before = LINKED_ENGAGEMENTS.with(Cell::get);
            self.shipped.process_block_mono(&mut shipped_left, frames);
            self.oracle.process_block_mono(&mut oracle_left, frames);
            assert_eq!(
                LINKED_ENGAGEMENTS.with(Cell::get),
                before,
                "{} {at}: a collapsed block linked",
                self.label
            );
            assert!(
                !self.shipped.gain_linked,
                "{} {at}: collapsed but linked",
                self.label
            );
            assert_same_words(
                &shipped_left,
                &oracle_left,
                &format!("{} {at}: collapsed output", self.label),
            );
            self.first_sample += frames as u64;
            self.compare(at);
        }

        fn desymmetrize(&mut self) {
            self.shipped.desymmetrize();
            self.oracle.desymmetrize();
            self.compare("desymmetrized");
        }

        fn reset(&mut self, kind: ResetKind) {
            self.shipped.reset(kind);
            self.oracle.reset(kind);
            self.compare("reset");
        }

        fn restore(&mut self, track: usize, payload: &LanePayload) -> bool {
            let shipped = core_restore(&mut self.shipped, track, payload);
            let oracle = core_restore(&mut self.oracle, track, payload);
            assert_eq!(shipped, oracle, "{}: restore verdicts", self.label);
            self.compare("restored");
            shipped.is_ok()
        }

        /// Everything observable, and the #990 invariant itself.
        fn compare(&self, at: &str) {
            let label = format!("{} {at}", self.label);
            let (shipped, oracle) = (&self.shipped, &self.oracle);
            // Cursors, every arena word of both channels (the reduction word the resident tap
            // reads included), every phase and all four words of every ramp.
            assert_eq!(
                state_bits(shipped),
                state_bits(oracle),
                "{label}: complete state"
            );
            for (a, b) in [
                (&shipped.left, &oracle.left),
                (&shipped.right, &oracle.right),
            ] {
                assert_eq!(a.lane, b.lane, "{label}: window shapes");
                assert_eq!(
                    a.lookahead_ms
                        .iter()
                        .map(|v| v.to_bits())
                        .collect::<Vec<_>>(),
                    b.lookahead_ms
                        .iter()
                        .map(|v| v.to_bits())
                        .collect::<Vec<_>>(),
                    "{label}: lookahead"
                );
            }
            for track in 0..L::WIDTH {
                assert_eq!(
                    core_snapshot(shipped, track),
                    core_snapshot(oracle, track),
                    "{label}: track {track} payload"
                );
            }
            assert_eq!(
                shipped.silent_fixed_point, oracle.silent_fixed_point,
                "{label}: silent claim"
            );
            assert_eq!(
                shipped.nonfinite_report(),
                oracle.nonfinite_report(),
                "{label}: non-finite report"
            );
            if shipped.gain_linked {
                assert!(
                    gain_state_agrees(&shipped.left, &shipped.right),
                    "{label}: the linked-agreement record is set over disagreeing gain words"
                );
            }
        }
    }

    fn no_spans(width: usize) -> Vec<Vec<PreparedAutomationSpan>> {
        vec![Vec::new(); width]
    }

    fn point_span(
        first_sample: u64,
        parameter: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> PreparedAutomationSpan {
        PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index: parameter,
            start_sample: first_sample,
            end_sample: first_sample,
            start_value: value,
            end_value: value,
        }
    }

    /// Gate 1's matrix at one width: both links, bypass on and off, every block length, and the
    /// quiet, limiting and asymmetric signals, on the fixture-shaped bank.
    ///
    /// Each run first renders 512 frames of its signal in 128-frame blocks, so the delay line
    /// (`B = 486` at 48 kHz) is full of rendered samples and every output word below is a gained
    /// input word rather than the line's initial zeros, then 48 frames at the length under test.
    fn linked_identity_matrix<L: Lane>(label: &str) {
        let tracks = fixture_tracks(L::WIDTH);
        for link in [LinkMode::Maximum, LinkMode::DualMono] {
            for bypass in [false, true] {
                for (length_index, &length) in LINKED_BLOCK_LENGTHS.iter().enumerate() {
                    for signal in [
                        LinkedSignal::Quiet,
                        LinkedSignal::Hot,
                        LinkedSignal::Asymmetric,
                    ] {
                        let run =
                            format!("{label} {link:?} bypass {bypass} length {length} {signal:?}");
                        let mut draw = Draw(0x0990_0001 ^ ((length_index as u64) << 8));
                        let mut pair =
                            LinkedPair::<L>::new(&tracks, link, bypass, 48_000, run.clone());
                        let spans = no_spans(L::WIDTH);
                        let mut blocks = 0;
                        for block in 0..4 {
                            let (left, right) =
                                linked_planes(signal, &mut draw, 128, &pair.shipped);
                            pair.dual(&left, &right, &spans, &format!("pre-roll {block}"));
                            blocks += 1;
                        }
                        for block in 0..48_usize.div_ceil(length) {
                            let (left, right) =
                                linked_planes(signal, &mut draw, length, &pair.shipped);
                            pair.dual(&left, &right, &spans, &format!("block {block}"));
                            blocks += 1;
                        }
                        // Gate 2 on the same runs: the fixture-shaped bank links every block under
                        // `Maximum`, and nothing links under `DualMono`.
                        let expected = if link == LinkMode::Maximum { blocks } else { 0 };
                        assert_eq!(pair.engaged, expected, "{run}: engagements");
                    }
                }
            }
        }
    }

    /// **Gate 1: the linked body renders exactly what the unmodified kernel renders.**
    ///
    /// Issue #990. Every output word, every track's payload, the complete state (the resident
    /// reduction tap's word included), the silent claim and the non-finite report, after every
    /// block, against the pre-#990 kernel kept in this module. Red mutations: skip the mirrored
    /// backward pass (M1), skip the mirrored box store (M2), engage under `DualMono` (M3).
    #[test]
    fn the_linked_body_renders_exactly_the_unmodified_kernel() {
        lane::each_lane!(|L, W| linked_identity_matrix::<L>(&width_label(W)));
    }

    /// One randomized scenario: a random bank, then random blocks, signals, automation and control
    /// calls, every one of them through both arms.
    fn linked_scenario<L: Lane>(seed: u64, label: &str) -> (u32, u32) {
        let mut draw = Draw(seed);
        let rate = if draw.chance(3, 4) {
            48_000
        } else {
            draw.pick(&[44_100, 88_200, 96_000])
        };
        let link = if draw.chance(3, 4) {
            LinkMode::Maximum
        } else {
            LinkMode::DualMono
        };
        let bypass = draw.chance(1, 5);
        let ragged = draw.chance(1, 7);
        let asymmetric = draw.chance(1, 5);
        let common_lookahead = draw.pick(&[0.0, 1.0, 2.0, 5.0, 10.0]);
        let tracks: Vec<_> = (0..L::WIDTH)
            .map(|_| {
                let left = [
                    draw.pick(&[-0.5, -1.0, -3.0, -6.0, -12.0, -24.0]),
                    draw.pick(&[10.0, 60.0, 100.0, 500.0, 2000.0]),
                    if ragged {
                        draw.pick(&[0.0, 1.0, 2.0, 5.0, 10.0])
                    } else {
                        common_lookahead
                    },
                ];
                let mut right = left;
                if asymmetric && draw.chance(1, 2) {
                    let parameter = draw.below(PARAMETER_COUNT);
                    right[parameter] = match parameter {
                        0 => draw.pick(&[-0.5, -2.0, -9.0]),
                        1 => draw.pick(&[20.0, 300.0]),
                        _ => draw.pick(&[0.5, 3.0, 7.0]),
                    };
                }
                linked_values(left, right)
            })
            .collect();
        let run = format!(
            "{label} seed {seed:#x} {rate} Hz {link:?} bypass {bypass} ragged {ragged} \
             asymmetric {asymmetric}"
        );
        let mut pair = LinkedPair::<L>::new(&tracks, link, bypass, rate, run);
        let mut stash: Vec<LanePayload> = Vec::new();
        let blocks = 24 + draw.below(16);
        for block in 0..blocks {
            let at = format!("block {block}");
            match draw.below(100) {
                0..=2 => pair.reset(if draw.chance(1, 2) {
                    ResetKind::FullToDefaults
                } else {
                    ResetKind::DiscontinuityKeepParameters
                }),
                3..=6 if !stash.is_empty() => {
                    let mut payload = stash[draw.below(stash.len())].clone();
                    if draw.chance(1, 4) {
                        // Another stashed track's right section: two sections that disagree.
                        payload.2 = stash[draw.below(stash.len())].2.clone();
                    }
                    if draw.chance(1, 5) {
                        // An in-flight limit ramp that `read_lane` accepts and that no retarget
                        // produces: it walks the limit to zero, below it, or to infinity.
                        let current = read_f32(&payload.1, words::LIMIT_RAMP);
                        let step = draw.pick(&[-current, -2.0 * current, f32::MAX, -0.5 * current]);
                        let remaining = 2 + draw.below(63) as u32;
                        let sections = if draw.chance(1, 2) { 2 } else { 1 };
                        for section in [&mut payload.1, &mut payload.2].into_iter().take(sections) {
                            write_f32(section, words::LIMIT_RAMP, current);
                            write_f32(section, words::LIMIT_RAMP + 1, current);
                            write_f32(section, words::LIMIT_RAMP + 2, step);
                            write_u32(section, words::LIMIT_RAMP + 3, remaining);
                        }
                    }
                    pair.restore(draw.below(L::WIDTH), &payload);
                }
                7..=9 => {
                    for step in 0..1 + draw.below(3) {
                        let frames = draw.pick(&LINKED_BLOCK_LENGTHS);
                        let signal = draw.pick(&[LinkedSignal::Hot, LinkedSignal::Quiet]);
                        let (left, _) = linked_planes(signal, &mut draw, frames, &pair.shipped);
                        pair.mono(&left, &format!("{at} collapsed {step}"));
                    }
                    // The contract desymmetrizes before any dual block. One run in four does not,
                    // which is how the defensive clear on collapse is reached from here.
                    if draw.chance(3, 4) {
                        pair.desymmetrize();
                    }
                }
                _ => {}
            }
            let mut spans = no_spans(L::WIDTH);
            if draw.chance(1, 6) {
                let lane = draw.below(L::WIDTH);
                let parameter = draw.below(RAMP_COUNT) as u32;
                let value = if parameter == 0 {
                    draw.pick(&[-0.5, -1.0, -3.0, -6.0])
                } else {
                    draw.pick(&[10.0, 60.0, 100.0, 500.0])
                };
                let first = pair.first_sample;
                spans[lane] = match draw.below(4) {
                    0 => vec![point_span(first, parameter, ParameterChannel::Left, value)],
                    1 => vec![point_span(first, parameter, ParameterChannel::Right, value)],
                    2 => vec![
                        point_span(first, parameter, ParameterChannel::Left, value),
                        point_span(first, parameter, ParameterChannel::Right, value),
                    ],
                    _ => vec![point_span(first, parameter, ParameterChannel::Both, value)],
                };
            }
            let frames = draw.pick(&LINKED_BLOCK_LENGTHS);
            let signal = if draw.chance(1, 2) {
                draw.pick(&[LinkedSignal::Hot, LinkedSignal::Asymmetric])
            } else {
                draw.pick(&LINKED_SIGNALS)
            };
            let (left, right) = linked_planes(signal, &mut draw, frames, &pair.shipped);
            pair.dual(&left, &right, &spans, &at);
            if draw.chance(1, 3) {
                stash.push(core_snapshot(&pair.shipped, draw.below(L::WIDTH)));
            }
        }
        (pair.engaged, pair.rendered)
    }

    /// **Gate 1, randomized: the linked body is the unmodified kernel under hostile events.**
    ///
    /// Issue #990. Random banks (every launch rate, both links, bypass, ragged and asymmetric
    /// cohorts), random block lengths and signals (including `-0.0`, subnormals, the exact
    /// threshold and non-finite words), one- and two-channel retargets, same-value retargets,
    /// rejected `Both` spans, both resets, restores of current, cross-track and hostile-ramp
    /// payloads, and collapse runs with and without `desymmetrize`. Every block is compared as in
    /// the matrix. The linked body must actually have run.
    #[test]
    fn randomized_scenarios_render_exactly_the_unmodified_kernel() {
        // Issue #1051's seed discipline: `MISO_ENGINE_RANDOMIZED_SCALE` multiplies the debug share
        // (the nightly job), `MISO_ENGINE_RANDOMIZED_SEED` replays one scenario.
        let scenarios = if cfg!(debug_assertions) || dsp_reference::randomized::overridden() {
            24
        } else {
            1000
        };
        let mut runs: Vec<(String, LinkedScenario)> = Vec::new();
        lane::each_lane!(|L, W| runs.push((width_label(W), linked_scenario::<L>)));
        for (label, run) in runs {
            let label = label.as_str();
            let mut engaged = 0;
            let mut rendered = 0;
            dsp_reference::randomized::run_seeds(
                &format!("randomized_scenarios_render_exactly_the_unmodified_kernel {label}"),
                "cargo test -p true-peak-limiter --lib -- --exact \
                 tests::randomized_scenarios_render_exactly_the_unmodified_kernel",
                scenarios,
                |scenario| {
                    let (linked, blocks) = run(0x0990_5EED_0000 + scenario, label);
                    engaged += linked;
                    rendered += blocks;
                },
            );
            println!("{label}: {engaged} of {rendered} dual blocks linked");
            if dsp_reference::randomized::replaying() {
                continue;
            }
            assert!(
                engaged > rendered / 8,
                "{label}: the linked body ran on {engaged} of {rendered} blocks"
            );
        }
    }

    // ---------------------------------------------------------------------------------------
    // Issue #1014 gate 2: the stationary walk against the unmodified kernel, where it is fragile.
    //
    // The oracle is #990's: `reference_block`, the kernel as it stood before #990, token for token,
    // reached through `LinkedPair`. The generator aims at what the walk turns on -- windows at and
    // around its thresholds, asymmetric channel windows, every block length up to 256, completions
    // on every kind of frame, the ring streams' three wrap cases and their refusals, ramping
    // blocks, and restores that put the van Herk phase at 0 and at `Wb - 1` -- and the census the
    // kernel keeps in test builds says whether it got there.
    // ---------------------------------------------------------------------------------------

    /// Lookaheads whose 48 kHz windows are the ones the walk turns on. `Wb = L + 1` for `L >= 31`:
    /// 0 ms is `Wb = 32` (the floor), 32 samples `33`, 5 ms `241`, 469, 475 and 479 samples `470`,
    /// `476` and `480` (`R - Wb` = 11, 5 and 1), and 10 ms `481`, which is `R`.
    const SEGMENT_LOOKAHEADS: [f32; 7] = [
        0.0,
        32.0 / 48.0,
        5.0,
        469.0 / 48.0,
        475.0 / 48.0,
        479.0 / 48.0,
        10.0,
    ];

    /// The windows [`SEGMENT_LOOKAHEADS`] must produce at 48 kHz.
    const SEGMENT_WINDOWS: [u32; 7] = [32, 33, 241, 470, 476, 480, 481];

    /// Block lengths for gate 2: around the 32-frame chunk, the 128-frame quantum and the
    /// 256-frame one the cores are prepared with.
    const SEGMENT_BLOCK_LENGTHS: [usize; 17] = [
        1, 2, 3, 4, 5, 31, 32, 33, 63, 64, 65, 127, 128, 129, 200, 255, 256,
    ];

    /// [`linked_core`] at a 256-frame quantum.
    fn segment_core<L: Lane>(
        tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]],
        link: LinkMode,
        rate: u32,
    ) -> LimiterCore<L> {
        assert_eq!(tracks.len(), L::WIDTH);
        let mut preparation = request_at(&tracks[0], rate, 256);
        preparation.link_mode = link;
        let metadata = expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, preparation)
            .expect("metadata");
        let mut left_defaults = Vec::with_capacity(L::WIDTH);
        let mut right_defaults = Vec::with_capacity(L::WIDTH);
        for values in tracks {
            let (left, right) = initial_defaults(values).expect("defaults");
            left_defaults.push(left);
            right_defaults.push(right);
        }
        LimiterCore::<L>::new(
            metadata,
            left_defaults.into_boxed_slice(),
            right_defaults.into_boxed_slice(),
        )
        .expect("core")
    }

    /// What one width of gate 2 did, beside the kernel's census.
    #[derive(Default)]
    struct SegmentRun {
        blocks: u32,
        linked: u32,
        /// Uniform dual blocks in the ramping dispatch that ran no segment of the stationary walk:
        /// the fused loop, which A2 keeps there.
        ramping_fused: u32,
        /// Restores of the whole bank at phase 0 and at `Wb - 1`.
        phase_zero: u32,
        phase_last: u32,
    }

    /// One gate-2 scenario. Scenario `index` takes lookahead `index % 7` on the left, and
    /// `(index / 7) % 4` picks linked-symmetric, dual-mono-symmetric, maximum-asymmetric or
    /// dual-mono-asymmetric, so every width's first 28 scenarios cover every pairing.
    fn segment_scenario<L: Lane>(index: u64, label: &str, totals: &mut SegmentRun) {
        let mut draw = Draw(0x1014_5EED_0000 ^ index);
        let rate = if index >= 28 && draw.chance(1, 8) {
            draw.pick(&[44_100, 88_200, 96_000])
        } else {
            48_000
        };
        let pattern = (index / 7) % 4;
        let link = if pattern.is_multiple_of(2) {
            LinkMode::Maximum
        } else {
            LinkMode::DualMono
        };
        let left_index = (index % 7) as usize;
        let right_index = if pattern < 2 {
            left_index
        } else {
            (left_index + 1 + draw.below(6)) % 7
        };
        let (left_lookahead, right_lookahead) = (
            SEGMENT_LOOKAHEADS[left_index],
            SEGMENT_LOOKAHEADS[right_index],
        );
        let tracks: Vec<_> = (0..L::WIDTH)
            .map(|_| {
                let ceiling = draw.pick(&[-0.5, -1.0, -3.0, -6.0, -12.0]);
                let release = draw.pick(&[10.0, 60.0, 100.0, 500.0, 2000.0]);
                linked_values(
                    [ceiling, release, left_lookahead],
                    [ceiling, release, right_lookahead],
                )
            })
            .collect();
        let run = format!(
            "{label} scenario {index} {rate} Hz {link:?} lookahead {left_lookahead}/{right_lookahead}"
        );
        let mut pair = LinkedPair::<L> {
            shipped: segment_core::<L>(&tracks, link, rate),
            oracle: segment_core::<L>(&tracks, link, rate),
            first_sample: 0,
            engaged: 0,
            rendered: 0,
            label: run,
        };
        pair.compare("prepared");
        if rate == 48_000 {
            assert_eq!(
                (
                    pair.shipped.left.lane[0].window,
                    pair.shipped.right.lane[0].window
                ),
                (SEGMENT_WINDOWS[left_index], SEGMENT_WINDOWS[right_index]),
                "{}: windows",
                pair.label
            );
        }
        let blocks = 32 + draw.below(16);
        for block in 0..blocks {
            let at = format!("block {block}");
            match draw.below(100) {
                0..=2 => pair.reset(if draw.chance(1, 2) {
                    ResetKind::FullToDefaults
                } else {
                    ResetKind::DiscontinuityKeepParameters
                }),
                3..=9 => {
                    // The whole bank at one van Herk phase, so the cohort stays uniform: 0, or the
                    // last position of each channel's window, whose next frame completes.
                    let last = draw.chance(1, 2);
                    let windows = [
                        pair.shipped.left.lane[0].window,
                        pair.shipped.right.lane[0].window,
                    ];
                    let mut restored = true;
                    for track in 0..L::WIDTH {
                        let mut payload = core_snapshot(&pair.shipped, track);
                        for (section, window) in
                            [&mut payload.1, &mut payload.2].into_iter().zip(windows)
                        {
                            write_u32(section, words::PHASE, if last { window - 1 } else { 0 });
                        }
                        restored &= pair.restore(track, &payload);
                    }
                    // A payload whose main ring holds a non-finite word the §4.4 check has not
                    // reached yet is refused, by both arms alike (`restore` compares verdicts).
                    if restored && last {
                        totals.phase_last += 1;
                    } else if restored {
                        totals.phase_zero += 1;
                    }
                }
                _ => {}
            }
            let mut spans = no_spans(L::WIDTH);
            if draw.chance(1, 5) {
                // A retarget: of both channels (a left span and a right span of one value, so a
                // linked pair stays linked through the ramp) or of one.
                let parameter = draw.below(RAMP_COUNT) as u32;
                let value = if parameter == 0 {
                    draw.pick(&[-0.5, -1.0, -3.0, -6.0])
                } else {
                    draw.pick(&[10.0, 60.0, 100.0, 500.0])
                };
                let first = pair.first_sample;
                let both = draw.chance(3, 4);
                let lanes: Vec<usize> = if draw.chance(1, 2) {
                    (0..L::WIDTH).collect()
                } else {
                    vec![draw.below(L::WIDTH)]
                };
                for lane in lanes {
                    spans[lane] = if both {
                        vec![
                            point_span(first, parameter, ParameterChannel::Left, value),
                            point_span(first, parameter, ParameterChannel::Right, value),
                        ]
                    } else if draw.chance(1, 2) {
                        vec![point_span(first, parameter, ParameterChannel::Left, value)]
                    } else {
                        vec![point_span(first, parameter, ParameterChannel::Right, value)]
                    };
                }
            }
            let frames = if draw.chance(1, 2) {
                draw.pick(&SEGMENT_BLOCK_LENGTHS)
            } else {
                1 + draw.below(256)
            };
            let signal = if draw.chance(3, 4) {
                LinkedSignal::Hot
            } else {
                draw.pick(&LINKED_SIGNALS)
            };
            let (left, right) = linked_planes(signal, &mut draw, frames, &pair.shipped);
            clear_dispatch_observation();
            let walked = peek_census().segments;
            let linked = pair.dual(&left, &right, &spans, &at);
            let observed = dispatch_observation();
            totals.blocks += 1;
            totals.linked += u32::from(linked);
            if observed.route == DispatchRoute::DualUniform
                && observed.mode == StationaryDispatch::Ramping
                && peek_census().segments == walked
            {
                totals.ramping_fused += 1;
            }
        }
    }

    /// **Issue #1014 gate 2: the stationary walk renders exactly the unmodified kernel.**
    ///
    /// Randomized scenarios at every width (24 per width in dev, 1,000 in release) against #990's
    /// oracle, comparing every output word (NaN as "both NaN"), every track's payload and the
    /// complete state after every block. Every counter below must be nonzero: segments ending at a
    /// completion on their first frame, on their last frame, on a chunk boundary and on a ring
    /// wrap; dual segments cut by the right channel and asymmetric ones; linked runs of 0, 1, 2, 3
    /// and 31 steady frames; the ring streams with the newest stream wrapped, the expiring stream
    /// wrapped and neither; the checked fallback, and the fallback refused by `Wb == R` or
    /// `steady > R - Wb`; ramping blocks that took the fused loop; and restores at phase 0 and at
    /// `Wb - 1`.
    #[test]
    fn the_stationary_walk_renders_exactly_the_unmodified_kernel() {
        let scenarios = if cfg!(debug_assertions) { 24 } else { 1000 };
        let mut runs: Vec<(String, SegmentScenario)> = Vec::new();
        lane::each_lane!(|L, W| runs.push((width_label(W), segment_scenario::<L>)));
        for (label, run) in runs {
            let label = label.as_str();
            let _ = take_census();
            let mut totals = SegmentRun::default();
            for scenario in 0..scenarios {
                run(scenario, label, &mut totals);
            }
            let census = take_census();
            println!(
                "{label}: {scenarios} scenarios, {} blocks ({} linked, {} ramping on the fused \
                 loop), phase restores {}/{}; {census:?}",
                totals.blocks,
                totals.linked,
                totals.ramping_fused,
                totals.phase_zero,
                totals.phase_last
            );
            for (name, count) in [
                ("segments", census.segments),
                ("completions", census.completions),
                (
                    "completions on a first frame",
                    census.completion_first_frame,
                ),
                ("completions on a last frame", census.completion_last_frame),
                (
                    "completions on a chunk boundary",
                    census.completion_chunk_boundary,
                ),
                ("completions on a ring wrap", census.completion_ring_wrap),
                ("right-channel cuts", census.right_cut),
                ("asymmetric dual segments", census.asymmetric),
                ("linked runs of 0 steady frames", census.linked_steady[0]),
                ("linked runs of 1 steady frame", census.linked_steady[1]),
                ("linked runs of 2 steady frames", census.linked_steady[2]),
                ("linked runs of 3 steady frames", census.linked_steady[3]),
                ("linked runs of 31 steady frames", census.linked_steady[4]),
                ("stream runs", census.streams),
                ("stream runs, newest wrapped", census.streams_newest_wrapped),
                (
                    "stream runs, expiring wrapped",
                    census.streams_expiring_wrapped,
                ),
                ("stream runs, unwrapped", census.streams_unwrapped),
                ("checked fallback runs", census.fallback),
                ("refused stream runs", census.fallback_bound),
                (
                    "ramping blocks on the fused loop",
                    u64::from(totals.ramping_fused),
                ),
                ("linked blocks", u64::from(totals.linked)),
                ("phase-0 restores", u64::from(totals.phase_zero)),
                ("phase Wb - 1 restores", u64::from(totals.phase_last)),
            ] {
                assert!(count > 0, "{label}: no {name}");
            }
        }
    }

    /// Mutation M4's gate: a collapsed block unlinks the pair even when no `desymmetrize` follows.
    ///
    /// The contract desymmetrizes before any dual block (`effect-contract`), and `desymmetrize`
    /// re-establishes the record, so clearing it on a collapsed block is defensive: under the
    /// contract the clear is unobservable. This test leaves the contract on purpose to reach it.
    /// The right channel keeps its pre-collapse words, so a pair still marked linked would render
    /// its right channel from the left's gain path; the reference kernel renders it from its own.
    /// Red mutation: drop the clear in `process_block_mono` (M4).
    #[test]
    fn a_collapsed_block_unlinks_the_pair_without_desymmetrize() {
        fn run<L: Lane>(label: &str) {
            let mut pair = LinkedPair::<L>::new(
                &fixture_tracks(L::WIDTH),
                LinkMode::Maximum,
                false,
                48_000,
                label.to_owned(),
            );
            let mut draw = Draw(0x0990_0004);
            let spans = no_spans(L::WIDTH);
            for block in 0..4 {
                let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
                assert!(pair.dual(&left, &right, &spans, &format!("dual {block}")));
            }
            for block in 0..3 {
                let (left, _) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
                pair.mono(&left, &format!("collapsed {block}"));
            }
            for block in 0..4 {
                let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
                assert!(
                    !pair.dual(&left, &right, &spans, &format!("after {block}")),
                    "{label}: linked after a collapse with no desymmetrize"
                );
            }
        }
        lane::each_lane!(|L, W| run::<L>(&width_label(W)));
    }

    /// Renders `blocks` hot dual blocks of 128 frames with no automation and returns the engaged
    /// count.
    fn hot_blocks<L: Lane>(
        pair: &mut LinkedPair<L>,
        draw: &mut Draw,
        blocks: usize,
        at: &str,
    ) -> u32 {
        let spans = no_spans(L::WIDTH);
        let mut engaged = 0;
        for block in 0..blocks {
            let (left, right) = linked_planes(LinkedSignal::Hot, draw, 128, &pair.shipped);
            engaged += u32::from(pair.dual(&left, &right, &spans, &format!("{at} {block}")));
        }
        engaged
    }

    /// Gate 2 at one width. Every run is also a gate-1 identity run.
    fn linked_engagement_witness<L: Lane>(label: &str) {
        let fixture = fixture_tracks(L::WIDTH);
        let fresh =
            |link: LinkMode, tracks: &[[InitialParameterValue; PARAMETER_COUNT * 2]], tag: &str| {
                LinkedPair::<L>::new(tracks, link, false, 48_000, format!("{label} {tag}"))
            };
        let mut draw = Draw(0x0990_0002);

        // Engages on the fixture-shaped bank, every block.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "fixture");
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 6, "fixture"),
            6,
            "{label}: fixture"
        );

        // Never under `DualMono`.
        let mut pair = fresh(LinkMode::DualMono, &fixture, "dual mono");
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 6, "dual mono"),
            0,
            "{label}: DualMono"
        );

        // Never with one lane's left ceiling apart; desymmetrize copies the left ceiling over the
        // right and the pair links.
        let mut ceiling_apart = fixture.clone();
        let last = L::WIDTH - 1;
        ceiling_apart[last][0].value = -2.0;
        let mut pair = fresh(LinkMode::Maximum, &ceiling_apart, "ceiling apart");
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 4, "apart"),
            0,
            "{label}: ceiling apart"
        );
        pair.desymmetrize();
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "copied"),
            3,
            "{label}: desymmetrized"
        );

        // A one-channel retarget: not on the block it lands, not while it ramps, and not after
        // the other channel is retargeted to the same value and both ramps have settled with
        // bit-equal designed words. A reset re-engages.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "one channel");
        assert_eq!(hot_blocks(&mut pair, &mut draw, 3, "before"), 3);
        let mut spans = no_spans(L::WIDTH);
        spans[last] = vec![point_span(
            pair.first_sample,
            1,
            ParameterChannel::Left,
            250.0,
        )];
        let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
        assert!(
            !pair.dual(&left, &right, &spans, "lands"),
            "{label}: on the landing block"
        );
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "ramping"),
            0,
            "{label}: while apart"
        );
        spans[last] = vec![point_span(
            pair.first_sample,
            1,
            ParameterChannel::Right,
            250.0,
        )];
        let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
        assert!(
            !pair.dual(&left, &right, &spans, "matched"),
            "{label}: on the matching block"
        );
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 4, "re-equal"),
            0,
            "{label}: re-equal"
        );
        assert!(
            designed_gain_agree(&pair.shipped.left, &pair.shipped.right),
            "{label}: the re-equal case must have equal designed words"
        );
        pair.reset(ResetKind::DiscontinuityKeepParameters);
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "reset"),
            3,
            "{label}: after reset"
        );

        // A two-channel retarget (a left span and a right span, equal values, one block) stays
        // linked through the ramping dispatch.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "both channels");
        assert_eq!(hot_blocks(&mut pair, &mut draw, 2, "before"), 2);
        let mut spans = no_spans(L::WIDTH);
        for (lane, lane_spans) in spans.iter_mut().enumerate() {
            let value = -3.0 - lane as f32;
            *lane_spans = vec![
                point_span(pair.first_sample, 0, ParameterChannel::Left, value),
                point_span(pair.first_sample, 0, ParameterChannel::Right, value),
            ];
        }
        let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 32, &pair.shipped);
        assert!(
            pair.dual(&left, &right, &spans, "retarget"),
            "{label}: two-channel retarget"
        );
        assert!(!dual_stationary(&pair.shipped.left, &pair.shipped.right));
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "ramping"),
            3,
            "{label}: ramping"
        );

        // Restores of the whole bank, so every lane lands on one van Herk phase and the cohort
        // stays uniform: payloads whose left and right sections disagree in their gain words
        // (and agree in their designed words) unlink the pair; payloads whose sections agree
        // link it again. A one-track restore would desynchronise the phase and send the cohort
        // to the per-lane body, which would say nothing about the comparison.
        let mut donor = linked_core::<L>(&fixture, LinkMode::DualMono, false, 48_000);
        for _ in 0..3 {
            let (mut left, _) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &donor);
            let mut right = vec![0.0; left.len()];
            drive_dual(&mut donor, &mut left, &mut right, &no_spans(L::WIDTH), 0);
        }
        let apart: Vec<_> = (0..L::WIDTH)
            .map(|track| core_snapshot(&donor, track))
            .collect();
        assert!(
            apart.iter().all(|payload| payload.1 != payload.2),
            "the donor's sections must differ"
        );
        let mut pair = fresh(LinkMode::Maximum, &fixture, "restore");
        assert_eq!(hot_blocks(&mut pair, &mut draw, 2, "before"), 2);
        let together: Vec<_> = (0..L::WIDTH)
            .map(|track| core_snapshot(&pair.shipped, track))
            .collect();
        for (track, payload) in apart.iter().enumerate() {
            assert!(pair.restore(track, payload));
        }
        assert!(designed_gain_agree(&pair.shipped.left, &pair.shipped.right));
        assert!(lanes_uniform(&pair.shipped.left) && lanes_uniform(&pair.shipped.right));
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "apart"),
            0,
            "{label}: restored apart"
        );
        for (track, payload) in together.iter().enumerate() {
            assert!(pair.restore(track, payload));
        }
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "together"),
            3,
            "{label}: restored"
        );

        // A collapsed block never links; the record stays clear until `desymmetrize`.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "collapse");
        assert_eq!(hot_blocks(&mut pair, &mut draw, 2, "before"), 2);
        let (left, _) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
        pair.mono(&left, "collapsed");
        pair.desymmetrize();
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "after"),
            3,
            "{label}: desymmetrized"
        );

        // The silent fast path keeps the record: a fresh pair earns the silence claim, skips blocks
        // on it, and links on the first loud block after.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "silence");
        let silent = vec![0.0_f32; 128 * L::WIDTH];
        for block in 0..4 {
            pair.dual(
                &silent,
                &silent,
                &no_spans(L::WIDTH),
                &format!("silent {block}"),
            );
        }
        assert!(
            pair.shipped.silent_engagements() > 0,
            "{label}: the fast path never ran"
        );
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 2, "after"),
            2,
            "{label}: after silence"
        );

        // The §4.4 reset of a non-finite block re-establishes the record, as `reset` does: a pair
        // unlinked by a one-channel retarget links again once the reset has put both channels
        // back on their (symmetric) defaults. Every lane is poisoned, so every lane fails and the
        // recovery is the whole reset (#1091); the recovery of one lane of a bank follows.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "non-finite");
        assert_eq!(hot_blocks(&mut pair, &mut draw, 1, "before"), 1);
        let mut spans = no_spans(L::WIDTH);
        spans[0] = vec![point_span(
            pair.first_sample,
            1,
            ParameterChannel::Left,
            250.0,
        )];
        let (mut left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
        left[..L::WIDTH].fill(f32::NAN);
        assert!(!pair.dual(&left, &right, &spans, "poisoned"));
        let mut blocks = 0;
        while pair.shipped.nonfinite_report().nonfinite_blocks == 0 {
            assert!(
                blocks < 8,
                "{label}: the NaN never reached the boundary check"
            );
            hot_blocks(&mut pair, &mut draw, 1, "draining");
            blocks += 1;
        }
        assert_eq!(
            pair.shipped.nonfinite_report().nonfinite_lanes,
            every_lane(L::WIDTH),
            "{label}: every lane failed"
        );
        assert_eq!(
            hot_blocks(&mut pair, &mut draw, 3, "after"),
            3,
            "{label}: after §4.4"
        );

        // #1091: the recovery of the one failed lane of a bank re-derives the record from the
        // words, which agree again once the retargeted lane is back on its defaults. The linked
        // body itself waits: the recovered lane restarts its van Herk phase, so the bank renders
        // the per-lane body, which never links, until a whole reset brings the phases together.
        // At `W = 1` the one lane is every lane, which is the case above.
        if L::WIDTH > 1 {
            let mut pair = fresh(LinkMode::Maximum, &fixture, "one lane non-finite");
            assert_eq!(hot_blocks(&mut pair, &mut draw, 1, "before"), 1);
            let mut spans = no_spans(L::WIDTH);
            spans[0] = vec![point_span(
                pair.first_sample,
                1,
                ParameterChannel::Left,
                250.0,
            )];
            let (mut left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
            left[0] = f32::NAN;
            assert!(!pair.dual(&left, &right, &spans, "poisoned"));
            let mut blocks = 0;
            while pair.shipped.nonfinite_report().nonfinite_blocks == 0 {
                assert!(
                    blocks < 8,
                    "{label}: the NaN never reached the boundary check"
                );
                hot_blocks(&mut pair, &mut draw, 1, "draining");
                blocks += 1;
            }
            assert_eq!(pair.shipped.nonfinite_report().nonfinite_lanes, 1);
            assert!(
                pair.shipped.gain_linked,
                "{label}: one lane's recovery re-derives the record"
            );
            assert!(
                !lanes_uniform(&pair.shipped.left),
                "{label}: the recovered lane restarted its phase"
            );
            // Still the reference kernel's bits, block by block, on the per-lane body.
            hot_blocks(&mut pair, &mut draw, 3, "after one lane");
        }

        // ceiling retarget at 30, unlinked from the left-only release retarget at 60, linked
        // again from the reset at 90.
        let mut pair = fresh(LinkMode::Maximum, &fixture, "gate 3");
        for block in 0..128 {
            if block == 90 {
                pair.reset(ResetKind::FullToDefaults);
            }
            let mut spans = no_spans(L::WIDTH);
            let first = pair.first_sample;
            if block == 30 {
                for lane_spans in &mut spans {
                    *lane_spans = vec![
                        point_span(first, 0, ParameterChannel::Left, -3.0),
                        point_span(first, 0, ParameterChannel::Right, -3.0),
                    ];
                }
            }
            if block == 60 {
                spans[0] = vec![point_span(first, 1, ParameterChannel::Left, 250.0)];
            }
            let (left, right) = linked_planes(LinkedSignal::Hot, &mut draw, 128, &pair.shipped);
            let engaged = pair.dual(&left, &right, &spans, &format!("block {block}"));
            assert_eq!(
                engaged,
                !(60..90).contains(&block),
                "{label}: gate 3 block {block}"
            );
        }
    }

    /// **Gate 2: the linked body engages exactly where the linked-agreement record allows.**
    ///
    /// Issue #990. Red mutation: compare only `current` of the ramps in `designed_gain_agree`
    /// (M5) -- the one-channel retarget's landing block then links.
    #[test]
    fn the_linked_body_engages_exactly_where_the_record_allows() {
        lane::each_lane!(|L, W| linked_engagement_witness::<L>(&width_label(W)));
    }

    /// One width's `linked_scenario`.
    type LinkedScenario = fn(u64, &str) -> (u32, u32);

    /// One width's `segment_scenario`.
    type SegmentScenario = fn(u64, &str, &mut SegmentRun);

    /// The label this module gives a lane width: `scalar`, `W4` or `W8`.
    fn width_label(lanes: usize) -> String {
        if lanes == 1 {
            "scalar".to_owned()
        } else {
            format!("W{lanes}")
        }
    }
}
