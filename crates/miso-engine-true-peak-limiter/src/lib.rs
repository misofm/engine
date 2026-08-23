//! Fixed-four-phase true-peak safety limiter.
//!
//! The audible path stays at the host sample rate; the frozen BS.1770-5 Annex-2 FIR is
//! detector-only. One generic block kernel, [`limiter_block`], owns the frame loop for every
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
    EffectDescriptorV1, EffectPrepareError, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory,
    ParameterChannel, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain, ParameterId,
    ParameterMapping, ParameterUnit, PortDescriptorV1, PortId, PortLayout, PortRole,
    PrepareEffectBankRequest, PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport,
    ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples, expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{NonFiniteReport, finish_block};
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
) -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
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
pub const TRUE_PEAK_LIMITER_PARAMETERS_V1: [ParameterDescriptorV1; PARAMETER_COUNT] = [
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

/// The state-layout-2 resource row of one launch rate.
///
/// `lane_words = 27 + B + 2R = 3N + 35`: twenty-seven scalar words, the `B = N + 6` main-delay
/// ring, and the two `R = N + 1` gain rings the minimum filter and the box ramp need (layout 1 had
/// no box ring and no minimum-filter words, hence the re-pin). The common section is the two-word
/// version/length header `miso-engine-effect-runtime` stamps into every payload, which is why
/// `common_bytes` is eight and no longer zero. The latency column does not move.
const fn quality(rate: u32) -> miso_engine_effect_contract::QualityDescriptorV1 {
    let lookahead_maximum = rate / 100;
    let lane_words = 3 * lookahead_maximum + 35;
    let lane_bytes = lane_words * 4;
    miso_engine_effect_contract::QualityDescriptorV1 {
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

const QUALITIES: [miso_engine_effect_contract::QualityDescriptorV1; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable launch true-peak limiter descriptor.
pub const TRUE_PEAK_LIMITER_DESCRIPTOR_V1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("miso.true-peak-limiter"),
    display_name: "True-Peak Limiter",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: STATE_LAYOUT_VERSION,
    supported_link_modes: match LinkModeSet::new(3) {
        Some(value) => value,
        None => panic!("frozen link bits"),
    },
    parameters: &TRUE_PEAK_LIMITER_PARAMETERS_V1,
    ports: &PORTS,
    qualities: &QUALITIES,
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
fn detector_peak<L: Lane>(
    history: &mut [L; HISTORY_WORDS],
    x: L,
    fir: &[[L; 4]; HISTORY_WORDS],
) -> L {
    let mut tap = HISTORY_WORDS - 1;
    while tap > 0 {
        history[tap] = history[tap - 1];
        tap -= 1;
    }
    history[0] = x;
    let mut peak = history[FIR_ALIGNMENT_SAMPLES].abs();
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
#[inline(always)]
fn annex2_phases<L: Lane>(history: &[L; HISTORY_WORDS], fir: &[[L; 4]; HISTORY_WORDS]) -> [L; 4] {
    let mut phases = [L::zero(); 4];
    for (row, sample) in fir.iter().zip(history.iter()) {
        for (accumulator, coefficient) in phases.iter_mut().zip(row.iter()) {
            *accumulator = accumulator.add(coefficient.mul(*sample));
        }
    }
    phases
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
    history: [L; HISTORY_WORDS],
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
        let mut history = [L::zero(); HISTORY_WORDS];
        for (tap, word) in history.iter_mut().enumerate() {
            *word = L::load(&state.history[tap * state.width..]);
        }
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
        let width = state.width;
        for (tap, word) in self.history.iter().enumerate() {
            word.store(&mut state.history[tap * width..]);
        }
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

    for (lane, expiring) in scratch.iter_mut().enumerate().take(width) {
        let mut slot = ring_cursor + state.lane[lane].box_offset as usize;
        if slot >= ring {
            slot -= ring;
        }
        *expiring = state.box_ring[slot * width + lane];
    }
    let expired = L::load(scratch);
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
    let width = L::WIDTH;
    debug_assert!(width <= MAXIMUM_WIDTH);
    debug_assert_eq!(left.width, width);
    debug_assert_eq!(right.width, width);
    debug_assert_eq!(left_io.len(), frames * width);
    debug_assert_eq!(right_io.len(), frames * width);

    let mut hot_left = HotChannel::<L>::load(left);
    let mut hot_right = HotChannel::<L>::load(right);
    let all = L::zero().eq(L::zero());
    let none = L::mask_not(all);
    let link = if coef.link_max { all } else { none };
    let bypass = if coef.bypass { all } else { none };
    let mut main_cursor = cursors.main as usize;
    let mut ring_cursor = cursors.ring as usize;
    let mut scratch = [0.0_f32; MAXIMUM_WIDTH];
    let mut peaks = [[0.0_f32; DETECTOR_CHUNK * MAXIMUM_WIDTH]; 2];

    // The block is walked in chunks so that only one channel's twelve history words are live at a
    // time. Both channels' histories together are twenty-four vector registers, which is more than
    // any of the three backends has; splitting the detector into two passes over a short chunk
    // costs twelve loads and twelve stores per chunk and removes the spill from the inner loop.
    // Nothing about the per-lane operation order changes, so the block is bit-identical to the
    // single-pass form (the E12 digests are the proof).
    for chunk in (0..frames).step_by(DETECTOR_CHUNK) {
        let span = core::cmp::min(DETECTOR_CHUNK, frames - chunk);
        for (channel, io) in [&*left_io, &*right_io].into_iter().enumerate() {
            let hot = if channel == 0 {
                &mut hot_left
            } else {
                &mut hot_right
            };
            let mut history = hot.history;
            for frame in 0..span {
                let base = (chunk + frame) * width;
                let x = L::load(&io[base..]);
                detector_peak(&mut history, x, &coef.fir)
                    .store(&mut peaks[channel][frame * width..]);
            }
            hot.history = history;
        }

        for frame in 0..span {
            let base = (chunk + frame) * width;
            let limit_left = hot_left.limit.advance();
            let release_left = hot_left.release.advance();
            let limit_right = hot_right.limit.advance();
            let release_right = hot_right.release.advance();

            let peak_left = L::load(&peaks[0][frame * width..]);
            let peak_right = L::load(&peaks[1][frame * width..]);
            let linked = peak_right.max(peak_left);
            let peak_left = L::select(link, linked, peak_left);
            let peak_right = L::select(link, linked, peak_right);

            channel_frame(
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
            channel_frame(
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
            metadata,
            shape,
            left_defaults,
            right_defaults,
        })
    }

    /// The two resets, one implementation (#90 F9).
    fn reset(&mut self, kind: ResetKind) {
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

    /// The boundary-check record, for the gates. Wiring it into `ProcessReport` belongs to #95.
    #[cfg(test)]
    const fn nonfinite_report(&self) -> NonFiniteReport {
        self.report
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
/// block, so `process` runs the same [`limiter_block`] body a W8 bank runs, and lane identity is a
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
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
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
        if !request.has_matching_backend_width()
            || request.requests.len() != request.width.lanes() as usize
        {
            return Err(EffectPrepareError {
                code: "effect.bank.requests",
            });
        }
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
        for member in request.requests.iter().copied() {
            let candidate = expected_prepared_metadata(self.descriptor(), member)?;
            if candidate.program_key() != metadata.program_key() {
                return Err(EffectPrepareError {
                    code: "effect.bank.program",
                });
            }
            let (left, right) = initial_defaults(member.initial_values)?;
            left_defaults.push(left);
            right_defaults.push(right);
        }
        // Decision D4: the backend is a compile-time constant, so "unavailable" means this artifact
        // was built for a narrower width than the cohort asks for. Every member has already been
        // validated, so the fallback is transactional.
        if Backend::current().width() < request.width.lanes() as usize {
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

impl PreparedNativeEffect for PreparedTruePeakLimiter {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.core.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        self.core.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
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
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        debug_assert_eq!(block.width, self.metadata.width);
        debug_assert_eq!(block.width.lanes() as usize, L::WIDTH);
        debug_assert!(block.frames <= self.core.metadata.quantum);
        debug_assert!(block.sidechain.is_none());
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
        self.core
            .process_block(block.left, block.right, block.frames as usize);
        report
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

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_core::KernelBackendV1;
    use miso_engine_dsp_reference::reference_annex2_phases;
    use miso_engine_effect_contract::{
        PrepareEffectLimits, PreparedPortsV1, PreparedSidechainPort, validate_descriptor_v1,
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
            ports: PreparedPortsV1 {
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
        backend: KernelBackendV1,
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
        validate_descriptor_v1(&TRUE_PEAK_LIMITER_DESCRIPTOR_V1).expect("descriptor");
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
        let mut kernel_history = [0.0_f32; HISTORY_WORDS];
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
            let mut history = [0.0_f32; HISTORY_WORDS];
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
    /// `z * 1.0` exactly `z`, signed zero included. Without the flush the identity would be
    /// approached and never reached. At a 10 ms release the decay crosses `FLUSH_EPS` after about
    /// 22 000 samples, which is why the silence is that long.
    #[test]
    fn silence_restores_exact_identity_including_signed_zero() {
        let values = values_with(-6.0, 10.0, 5.0);
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
                "left identity at {index}"
            );
            assert_eq!(
                right[index].to_bits(),
                expected_right[index - latency].to_bits(),
                "right identity at {index}"
            );
        }
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
                (BankWidth::Four, KernelBackendV1::Aarch64Neon, 4_usize),
                (BankWidth::Eight, KernelBackendV1::X86Avx2Fma, 8),
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
            KernelBackendV1::X86Avx2Fma,
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
            backend: KernelBackendV1::Aarch64Neon,
            width: BankWidth::Eight,
            requests: &requests,
        });
        assert_eq!(
            mismatched.err().map(|error| error.code),
            Some("effect.bank.requests")
        );

        // A heterogeneous cohort is a planner bug, not a capability gap: it rejects rather than
        // silently falling back to scalar tails.
        let mut heterogeneous: Vec<PrepareEffectRequest<'_>> = requests.clone();
        heterogeneous[3].link_mode = LinkMode::Maximum;
        let heterogeneous =
            TruePeakLimiterFactory.bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: KernelBackendV1::X86Avx2Fma,
                width: BankWidth::Eight,
                requests: &heterogeneous,
            });
        assert_eq!(
            heterogeneous.err().map(|error| error.code),
            Some("effect.bank.program")
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
