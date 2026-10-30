//! Fixed causal dual-envelope transient shaper.
//!
//! This crate is the parameter table, the frozen follower coefficients and one generic block
//! kernel. Banking, ramps, the state-payload codec, parameter domains and the once-per-block
//! output boundary check are `effect-runtime`; the lane vocabulary and the dry/wet mix
//! law are `lane`; the per-sample dB conversions are `math` (master plan
//! for issue #83, §6 and D6).
//!
//! # The law
//!
//! Two switched attack/release one-pole followers — fast at 0.5 ms / 20 ms and slow at
//! 10 ms / 100 ms — track the linked detector magnitude. Their ratio is the *contrast*
//! `c = fast_level_db(max(fast, FLOOR) / max(slow, FLOOR))` in dB, clamped to ±24 dB; the
//! shape is `A * max(c, 0) + S * max(-c, 0)` clamped to ±18 dB; the gain is
//! `fast_gain_from_db(shape)` — which is `10^(shape / 20)` — and the output is the dry/wet mix
//! of `lane::kernels::gain_mix_step`.
//!
//! The ratio form keeps the detector to one level conversion instead of separate conversions of
//! both envelopes. Approved crossings X7/X8 use the fast dB tier; exhaustive MA-5 evidence records
//! a maximum applied-gain delta of `1.654115e-5` dB from the exact tier (issue #880).
//!
//! # Determinism
//!
//! Every render-path operation is a `Lane` operation or `Lane::fma`, so the rendered block is
//! bit-identical across `Scalar`/`Simd4`/`Simd8`, across `x86_64`/`aarch64`/`wasm32` and across
//! block partitions (D5). X7/X8 trade exact-tier accuracy for the measured fast-tier bounds; they
//! do not change the cross-target identity contract.
//!
//! # State
//!
//! Fourteen words per lane per channel at #1278 — `fast`, `slow`, then `(current, target, step, remaining)`
//! for each of the three parameters — at `state_layout_version` 1 (#1278 D2a). The layout before
//! it carried eleven words and re-derived each ramp's step on restore as
//! `(target - current) / remaining`, which resumes a mid-flight ramp on another path than the
//! continued one walks; carrying the step makes a restored lane continue bit for bit. The version
//! stays V1, the sole prelaunch identity (AGENTS.md): nothing persists a payload (R6b), and the
//! section lengths refuse an eleven-word one. Restore validates both channels in place and commits
//! from the bytes: no heap value is created or dropped, and the payload calls are render-safe.
//!
//! Issue #1370 D6 adds the two detector link glide weights, `w_link` then `w_avg`, one ramp
//! quadruple each, after the parameter ramps: twenty-two words, still at layout 1 (the section
//! length refuses a fourteen-word payload). The lane's link mode is the weights' targets
//! (`effect_contract::link_glide_mode`), so it takes no word of its own.
//!
//! # The link glide (issue #1370)
//!
//! [`PreparedNativeEffect::retarget_link`] moves one lane's detector link to another mode over a
//! ramp: the lane's mode, and with it its masks, takes the target at once, and the two weights of
//! each channel glide from the words in force to the target mode's rest weights. While a weight
//! moves, the lane's detector is `combined = max + w_avg * (avg - max)`,
//! `d = own + w_link * (combined - own)`, separate multiplies and adds; from the frame the glide
//! arrives it is the target mode's law, bit for bit. The glide rides the ramp prefix: only a
//! prefix with a glide in flight takes the blended body (`Shaper::run_glide`), and every other
//! frame renders the body it rendered before.

use effect_contract::{
    AutomationRate, BankProcessReport, BankWidth, EffectBankProcessBlock, EffectDescriptor,
    EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue, LatencySamples,
    LinkMode, LinkModeSet, NativeEffectFactory, ParameterChannel, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping, ParameterUnit,
    PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedBankMetadata, PreparedEffect, PreparedEffectBank,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport,
    ResetKind, SmoothingRule, StatePayloadError, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes, TailSamples, expected_prepared_metadata,
};
use effect_runtime::bank::{
    NonFiniteReport, block_is_positive_zero, check_block, nonfinite_lane_mask, positive_zero_lanes,
};
use effect_runtime::envelope::{ArCoef, ar_one_pole_step};
use effect_runtime::params::{
    ParameterSpec, normalize_zero, parameter_value_valid as spec_value_valid,
};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::{
    RAMP_WORDS, ramp_path_inside, read_f32, read_ramp, write_f32, write_ramp,
};
use lane::kernels::{gain_mix_step, ramp_toward};
use lane::{Backend, Lane};
use math::fast_db::{fast_gain_from_db, fast_level_db};

pub mod corpus;

const PARAMETER_COUNT: usize = 3;
/// The detector link glide weights of one channel (issue #1370 D2): `w_link`, then `w_avg`.
const LINK_WEIGHTS: usize = 2;
const RAMP_SAMPLES: u32 = 64;
/// Word index of the first link weight ramp in a channel section (issue #1370 D6): the parameter
/// ramps run straight into it.
const LINK_WORD: usize = 2 + PARAMETER_COUNT * RAMP_WORDS;
const STATE_WORDS: usize = LINK_WORD + LINK_WEIGHTS * RAMP_WORDS;
const LANE_STATE_BYTES: u32 = (STATE_WORDS * 4) as u32;

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
        None => panic!("nonzero static parameter identifier"),
    }
}

const fn parameter(
    id: u32,
    name: &'static str,
    display_name: &'static str,
    display_unit: &'static str,
    minimum: f32,
    maximum: f32,
    default_value: f32,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
        name,
        display_name,
        display_unit,
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(minimum),
        maximum: Some(maximum),
        default_value,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: RAMP_SAMPLES,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice: effect_contract::default_parameter_lattice(
            ParameterUnit::Linear,
            ParameterDomain::Continuous,
            ParameterMapping::Linear,
        ),
        prepared_reason: None,
    }
}

/// Frozen V1 parameter rows in stable numeric-ID order.
pub const TRANSIENT_SHAPER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(1, "attack_amount", "attack amount", "%", -1.0, 1.0, 0.0),
    parameter(2, "sustain_amount", "sustain amount", "%", -1.0, 1.0, 0.0),
    parameter(3, "mix", "mix", "linear", 0.0, 1.0, 1.0),
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

const fn quality(sample_rate: u32) -> effect_contract::QualityDescriptor {
    effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        maximum_state: StatePayloadSizes {
            common_bytes: 0,
            left_bytes: LANE_STATE_BYTES,
            right_bytes: LANE_STATE_BYTES,
        },
        // Re-accounting this row (the audit's F9: the 24 bytes are the in-struct parameter
        // defaults, which are state and not scratch) moves the canonical descriptor bytes and
        // therefore the Issue-082 effect identity, so it belongs to #95 and not here.
        scratch_fixed_bytes: 24,
        scratch_bytes_per_frame: 0,
    }
}

/// The fast tier's gain error, in dB: `fast_gain_from_db` is within this of `10^(g/20)` for every
/// `g` in `[-160, 24]` dB, the bound `crates/math/tests/f1_fast_db_bounds.rs` proves by an
/// exhaustive sweep (at most `7.431e-6` dB on `[-160, 0]` and `2.183e-6` dB on `[0, 24]`). The
/// applied gain's argument, the clamped shape, lies in `[-18, 18]` dB. The same value as the
/// compressor's, declared here because `math::fast_db` is sealed (#1376).
const FAST_GAIN_ERROR_DB: f64 = 1.0e-5;

/// `(1 + u)^count`, the relative growth of `count` rounded operations.
fn rounding_factor(count: u32) -> f64 {
    (0..count).fold(1.0, |factor, _| {
        factor * (1.0 + effect_runtime::tail::ROUNDOFF)
    })
}

/// The largest value either envelope word can hold for a detector of at most `peak`, as an `f32`
/// rounded up (#1376 D2).
///
/// With `U` the smallest `f32` at or above `peak`, every link mode's detector is at most `U`
/// (`abs`, `max`, and `fl(0.5 |l| + 0.5 |r|)`, whose halvings are exact and whose sum is at most
/// `fl(U) = U`). A follower step is `e' = fl(fl(c e) + fl(k u))` with `c + k = 1` exactly (each `k`
/// is `1 - c`, exact for `c` in `[1/2, 1]`), so for `e <= M` and `u <= U`,
/// `e' <= (c M + k U) (1 + u)^2 = (M - k (M - U)) (1 + u)^2`. With `M = U (1 + 3 u / k_min)`,
/// `k_min = 1 - c` of the slowest of the four coefficients, the right side is at most `M` whenever
/// `k_min >= 6 u (1 + u)`, which every launch rate meets by four decades. The words start at
/// `+0.0`, so `M` bounds them over every history. (A subnormal product adds at most `2^-150`
/// absolutely, far under `k_min (M - U)`.)
fn follower_distance_max(row: [f32; 4], peak: f64) -> Option<f32> {
    let k_min = row
        .iter()
        .map(|c| 1.0 - f64::from(*c))
        .fold(f64::INFINITY, f64::min);
    let u = effect_runtime::tail::ROUNDOFF;
    if k_min < 6.0 * u * (1.0 + u) {
        return None;
    }
    let bound = peak * (1.0 + 3.0 * u / k_min);
    // Round up to `f32`, then one `f32` ulp more: `U` exceeds `peak` by less than one ulp, and
    // the `f64` product's own rounding is far smaller.
    let mut word = bound as f32;
    if f64::from(word) < bound {
        word = f32::from_bits(word.to_bits() + 1);
    }
    let word = f32::from_bits(word.to_bits() + 1);
    word.is_normal().then_some(word)
}

/// The exact-rest bound `R` at `sample_rate` for every input of peak at most `peak` (#1376 D2),
/// or `None` when it does not exist (D3).
///
/// **Quantifier.** `R` holds when both input planes are exactly zero from the first silent sample
/// `M` on. With a zero detector both followers take the release arm (`0 > e` is false for
/// `e >= +0.0`), so each word is the pole form `e' = snap(fl(c e))` at its release retention, which
/// [`effect_runtime::tail::pole_follower_rest_samples`] bounds from [`follower_distance_max`]. The
/// D3 condition holds: the slowest retention, the 100 ms slow release at 96 kHz, is
/// `1 - 1.04e-4`, far below `1 - 2^-24`. The rest state `Z` is the reset state (both followers
/// `+0.0`); ramp words are parameter state, not signal state, and the latency is zero, so `R` is
/// the larger of the two words' bounds. The coefficients are frozen per rate, so no ramp or
/// parameter changes them.
fn rest_samples(sample_rate: u32, peak: f64) -> Option<u64> {
    let row = coefficient_row(sample_rate)?;
    let distance = follower_distance_max(row, peak)?;
    let snap = effect_runtime::tail::RestSnap::Below(FOLLOWER_REST);
    // Pin order: fast attack, fast release, slow attack, slow release.
    let fast = effect_runtime::tail::pole_follower_rest_samples(row[1], distance, snap)?;
    let slow = effect_runtime::tail::pole_follower_rest_samples(row[3], distance, snap)?;
    Some(fast.max(slow))
}

/// `G_p = G_t` in millibels (#1376 D4): the output law is `fma(mix, x g - x, x)` on the shaped arm,
/// or `x` on the identity arms, with `0 < g = fast_gain_from_db(shape)`, `shape` clamped to
/// `+-SHAPE_LIMIT_DB` and `mix` in `[0, 1]`. Through the four rounded operations (multiply,
/// subtract, and `Lane::fma`'s separate multiply and add), `|y| <= max(1, g) |x| (1 + u)^4`, and
/// `g <= 10^((SHAPE_LIMIT_DB + FAST_GAIN_ERROR_DB) / 20)`. After silence the output is the same
/// law of the current input, so `G_t = G_p`.
fn peak_gain_millibels() -> i32 {
    let gain = math::db_to_gain(f64::from(SHAPE_LIMIT_DB) + FAST_GAIN_ERROR_DB);
    math::tail::ceil_millibels(gain * rounding_factor(4), math::tail::Rounding::Computed)
}

/// `sigma_p = sigma_t` in millibels (#1376 D4, #1379 H3): the underflow allowance of the output's
/// chain, one [`math::tail::UNDERFLOW`] per rounded operation (four), each carried by at most
/// `(1 + u)^3` through the operations after it (the mix is at most `1`).
fn flush_stall_millibels() -> i32 {
    math::tail::ceil_millibels(
        4.0 * math::tail::UNDERFLOW * rounding_factor(3),
        math::tail::Rounding::Computed,
    )
}

/// This effect's tail, tail over every peak, exact-rest bound and composition values, the one place
/// they are stated (decision 15 D15-4(b), #1377 D1, #1376).
///
/// * **Tail `Finite(0)`, decay `0`** (#1376 D1): every output arm is the current input `x`, or
///   `gain_mix_step(x, gain, mix)` of it with a finite gain: the shaper holds no audio history and
///   has zero latency, so the output is exactly `+-0.0` from the first zero input sample.
/// * **Rest** [`rest_samples`] at the two peak cases, `10^(24/20)` and every sanitized input
///   (`|x| < 1e30`); **tail over every peak** the larger, an upper bound of `max(0, R(P*))`.
/// * **Composition** [`peak_gain_millibels`] and [`flush_stall_millibels`].
///
/// A rate the bound cannot cover states no bound.
fn tail_and_rest(sample_rate: u32, _quality: EffectQuality) -> effect_contract::NodeTailBound {
    let rest = (
        rest_samples(sample_rate, math::db_to_gain(24.0)),
        rest_samples(sample_rate, f64::from(1.0e30_f32)),
    );
    let (Some(peak_plus_24_dbfs), Some(any_sanitized_input)) = rest else {
        return effect_contract::NodeTailBound::UNBOUNDED;
    };
    let gain = effect_contract::PeakGain::Millibels(peak_gain_millibels());
    let stall = effect_contract::FlushStall::Level(flush_stall_millibels());
    effect_contract::NodeTailBound {
        tail: TailSamples::Finite(0),
        tail_every_peak: TailSamples::Finite(peak_plus_24_dbfs.max(any_sanitized_input)),
        rest: effect_contract::RestBound::Bounded(effect_contract::RestSamples {
            peak_plus_24_dbfs,
            any_sanitized_input,
        }),
        composition: effect_contract::CompositionBound::Stated {
            decay: effect_contract::TailDecay(0),
            peak_gain: gain,
            tail_gain: gain,
            peak_stall: stall,
            tail_stall: stall,
        },
    }
}

const QUALITIES: [effect_contract::QualityDescriptor; 4] = [
    quality(44_100),
    quality(48_000),
    quality(88_200),
    quality(96_000),
];

/// Immutable descriptor for the frozen causal transient-shaper V1 contract.
pub const TRANSIENT_SHAPER_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
    id: effect_id("miso.transient-shaper"),
    display_name: "Transient Shaper",
    contract_major: 1,
    contract_minor: 0,
    // #1278 D2a grew the layout to fourteen words without a bump: V1 is the sole prelaunch
    // identity (AGENTS.md), and nothing persists a payload (R6b).
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    lane_link: true,
    parameters: &TRANSIENT_SHAPER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    tail_and_rest,
    observations: &[],
};

/// Fast-attack, fast-release, slow-attack, slow-release coefficient bits by launch-rate row.
///
/// Each is `exp(-1 / (tau * fs))` — the *retention* coefficient of
/// `effect_runtime::envelope::retention_coefficient` — rounded once to `f32`, for the
/// frozen time constants 0.5 ms, 20 ms, 10 ms and 100 ms. Freezing the bits rather than designing
/// them at prepare is what keeps the coefficient off every transcendental path; the tests check
/// each of the sixteen against both the `f64` oracle and the runtime's coefficient design.
pub const TRANSIENT_SHAPER_COEFFICIENT_BITS: [[u32; 4]; 4] = [
    [0x3f74_a63c, 0x3f7f_b5bd, 0x3f7f_6b90, 0x3f7f_f124],
    [0x3f75_8d71, 0x3f7f_bbc5, 0x3f7f_779c, 0x3f7f_f259],
    [0x3f7a_42a5, 0x3f7f_dadc, 0x3f7f_b5bd, 0x3f7f_f892],
    [0x3f7a_b8ca, 0x3f7f_dde0, 0x3f7f_bbc5, 0x3f7f_f92c],
];

/// The frozen time constants of the four coefficients, in milliseconds and pin order.
pub const TRANSIENT_SHAPER_TIME_CONSTANTS_MS: [f32; 4] = [0.5, 20.0, 10.0, 100.0];

/// Envelope floor: the level both followers are raised to before the ratio is taken.
///
/// `1.0e-8`, applied to **both** operands before the division, exactly as the pre-audit crate
/// floored both logarithms. Flooring the ratio instead would change the silent case: two floored
/// envelopes divide to exactly `1`, so contrast is exactly zero and the effect is the identity,
/// which is the contract's "zero audio input produces zero output".
const FLOOR: f32 = f32::from_bits(0x322b_cc77);

/// Contrast is clamped to ±24 dB before the shape law.
const CONTRAST_LIMIT_DB: f32 = 24.0;

/// Shape is clamped to ±18 dB before the gain conversion.
const SHAPE_LIMIT_DB: f32 = 18.0;

#[cfg(test)]
thread_local! {
    /// Blocks the silent fast path skipped (issue #1746), counted once per skipped block.
    static SILENT_ADMISSIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// While set, the silent fast path is refused and every block runs the kernel: the control
    /// arm of the fast-path differential and the rest probe's fork.
    static SILENT_FORBIDDEN: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Blocks rendered by the mixed-link select arm (issue #1368 D4), counted once per block: a
    /// bank whose lanes share one mode must never take it.
    static MIXED_LINK_BLOCKS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// Whether the silent fast path may skip this block. Production has no switch: always `true`.
#[inline(always)]
fn silent_allowed() -> bool {
    #[cfg(test)]
    {
        !SILENT_FORBIDDEN.with(core::cell::Cell::get)
    }
    #[cfg(not(test))]
    {
        true
    }
}

const LINK_DUAL_MONO: u8 = 0;
const LINK_MAXIMUM: u8 = 1;
const LINK_AVERAGE: u8 = 2;
/// The bank's lanes do not share one mode (issue #1368 D4): each lane selects its own law.
const LINK_MIXED: u8 = 3;

/// Each lane's detector link mode as select masks (issue #1368 D3), and the arm the lanes pick.
///
/// `linked` is set on a lane at `Maximum` or `Average`, `averaged` on a lane at `Average`. `arm`
/// is the const arm of the lanes' one shared mode, or [`LINK_MIXED`] when they differ; it is
/// derived from the masks' modes only, never from data.
#[derive(Clone, Copy)]
struct LinkLanes<L: Lane> {
    linked: L::Mask,
    averaged: L::Mask,
    arm: u8,
}

impl<L: Lane> LinkLanes<L> {
    /// `links[lane]` is lane `lane`'s mode. Allocation-free and panic-free, so a render-time lane
    /// swap may rebuild it (issue #1747).
    fn new(links: &[LinkMode]) -> Self {
        debug_assert_eq!(links.len(), L::WIDTH);
        let mut linked = [0.0_f32; 8];
        let mut averaged = [0.0_f32; 8];
        for (lane, link) in links.iter().enumerate().take(L::WIDTH) {
            linked[lane] = f32::from(u8::from(*link != LinkMode::DualMono));
            averaged[lane] = f32::from(u8::from(*link == LinkMode::Average));
        }
        let arm = match links.first() {
            Some(first) if links.iter().all(|link| link == first) => match first {
                LinkMode::DualMono => LINK_DUAL_MONO,
                LinkMode::Maximum => LINK_MAXIMUM,
                LinkMode::Average => LINK_AVERAGE,
            },
            _ => LINK_MIXED,
        };
        let zero = L::zero();
        Self {
            linked: L::load(&linked).gt(zero),
            averaged: L::load(&averaged).gt(zero),
            arm,
        }
    }
}

/// Scalar factory entry point for the transient shaper.
#[derive(Clone, Copy, Debug, Default)]
pub struct TransientShaperFactory;

/// Per-bank follower coefficients, splatted once at prepare (master plan §4.2).
#[derive(Clone, Copy, Debug)]
struct Coef<L: Lane> {
    fast: ArCoef<L>,
    slow: ArCoef<L>,
}

fn coefficient_row(sample_rate: u32) -> Option<[f32; 4]> {
    let row = match sample_rate {
        44_100 => TRANSIENT_SHAPER_COEFFICIENT_BITS[0],
        48_000 => TRANSIENT_SHAPER_COEFFICIENT_BITS[1],
        88_200 => TRANSIENT_SHAPER_COEFFICIENT_BITS[2],
        96_000 => TRANSIENT_SHAPER_COEFFICIENT_BITS[3],
        _ => return None,
    };
    Some(row.map(f32::from_bits))
}

impl<L: Lane> Coef<L> {
    fn new(row: [f32; 4]) -> Self {
        Self {
            fast: ArCoef::splat(row[0], row[1]),
            slow: ArCoef::splat(row[2], row[3]),
        }
    }
}

/// The two recursive envelope words of one channel of one lane group.
#[derive(Clone, Copy, Debug)]
struct Env<L: Lane> {
    fast: L,
    slow: L,
}

impl<L: Lane> Default for Env<L> {
    fn default() -> Self {
        Self {
            fast: L::zero(),
            slow: L::zero(),
        }
    }
}

impl<L: Lane> Env<L> {
    // REALTIME_POLICY_BEGIN
    /// Lane `lane`'s two words back to `+0.0`, every other lane bit-unchanged: a reset's and the
    /// D7 recovery's envelope action for one lane.
    fn clear_lane(&mut self, lane: usize) {
        self.fast = replace_lane(self.fast, lane, 0.0);
        self.slow = replace_lane(self.slow, lane, 0.0);
    }
    // REALTIME_POLICY_END
}

/// The three ramped parameters of one channel, packed across lanes for one frame.
#[derive(Clone, Copy)]
struct Params<L: Lane> {
    attack: L,
    sustain: L,
    mix: L,
}

/// Detector linking, monomorphised over the link mode so the loop body is straight line.
///
/// Frozen operation order: rectify both channels, then `Maximum` is the D8 select `max` and
/// `Average` is `0.5 * l + 0.5 * r` — two products and a sum. Keep this order even where scaling
/// by one half makes `0.5 * (l + r)` produce the same bits; overflow and subnormal rounding can
/// distinguish the expressions.
///
/// [`LINK_MIXED`] (issue #1368 D4) computes both linked words in that same order and selects,
/// per lane, the word of the lane's own mode from `lanes`, so each lane's bits are the const arm's
/// of its mode. The const arms do not read `lanes`.
#[inline(always)]
fn link<L: Lane, const LINK: u8>(left: L, right: L, lanes: &LinkLanes<L>) -> (L, L) {
    let left = left.abs();
    let right = right.abs();
    match LINK {
        LINK_MIXED => {
            let maximum = left.max(right);
            let half = L::splat(0.5);
            let average = half.mul(left).add(half.mul(right));
            let value = L::select(lanes.averaged, average, maximum);
            (
                L::select(lanes.linked, value, left),
                L::select(lanes.linked, value, right),
            )
        }
        LINK_MAXIMUM => {
            let value = left.max(right);
            (value, value)
        }
        LINK_AVERAGE => {
            let half = L::splat(0.5);
            let value = half.mul(left).add(half.mul(right));
            (value, value)
        }
        _ => (left, right),
    }
}

/// One link weight of one channel across the lanes (issue #1370 D2), for a prefix with a glide in
/// flight: gathered from the lanes' [`LinearRamp`]s at the prefix's start and scattered back at
/// its end, so the weights move as vectors and never as per-lane scalars.
#[derive(Clone, Copy)]
struct LinkRamp<L: Lane> {
    current: L,
    target: L,
    step: L,
    remaining: L,
}

impl<L: Lane> LinkRamp<L> {
    /// Lane `l` of the result is `ramps[l][weight]`. A countdown of at most
    /// `LINK_GLIDE_MAX_SAMPLES` is exact in `f32`.
    fn gather<const W: usize>(ramps: &[[LinearRamp; LINK_WEIGHTS]; W], weight: usize) -> Self {
        let mut words = [[0.0_f32; 8]; 4];
        for (lane, ramps) in ramps.iter().enumerate() {
            let ramp = ramps[weight];
            words[0][lane] = ramp.current;
            words[1][lane] = ramp.target;
            words[2][lane] = ramp.step;
            words[3][lane] = ramp.remaining as f32;
        }
        Self {
            current: L::load(&words[0]),
            target: L::load(&words[1]),
            step: L::load(&words[2]),
            remaining: L::load(&words[3]),
        }
    }

    fn scatter<const W: usize>(self, ramps: &mut [[LinearRamp; LINK_WEIGHTS]; W], weight: usize) {
        let mut words = [[0.0_f32; 8]; 3];
        self.current.store(&mut words[0]);
        self.step.store(&mut words[1]);
        self.remaining.store(&mut words[2]);
        for (lane, ramps) in ramps.iter_mut().enumerate() {
            let ramp = &mut ramps[weight];
            ramp.current = words[0][lane];
            ramp.step = words[1][lane];
            ramp.remaining = words[2][lane] as u32;
        }
    }

    /// One frame of `LinearRamp::next_value`, lane-wide: `remaining = max(remaining - 1, 0)`,
    /// then the clamped step while it is still positive and the exact target once it is not (on
    /// a settled lane, `current` is already its target bit for bit, so the update is the
    /// identity).
    #[inline(always)]
    fn advance(&mut self) {
        let zero = L::zero();
        self.remaining = self.remaining.sub(L::splat(1.0)).max(zero);
        let stepping = self.remaining.gt(zero);
        self.current = L::select(
            stepping,
            ramp_toward(self.current, self.step, self.target),
            self.target,
        );
        self.step = L::select(stepping, self.step, zero);
    }
}

/// One channel's detector under a glide (issue #1370 D3): both weights advance one frame, and a
/// lane still in flight after it takes `combined = max + w_avg * (avg - max)`,
/// `d = own + w_link * (combined - own)`, each a separate multiply and add (base Wasm SIMD has no
/// fused multiply-add). A lane at rest, or arriving on this frame, takes `prepared`, the word its
/// own masks select ([`link`]'s [`LINK_MIXED`] arm), so its bits are its mode's law: a blend at
/// rest weights would equal it on finite words but not on non-finite ones (`0 * inf`).
#[inline(always)]
fn glide_level<L: Lane>(
    own: L,
    maximum: L,
    average: L,
    prepared: L,
    weights: &mut [LinkRamp<L>; LINK_WEIGHTS],
) -> L {
    for weight in weights.iter_mut() {
        weight.advance();
    }
    let zero = L::zero();
    let gliding = L::mask_or(weights[0].remaining.gt(zero), weights[1].remaining.gt(zero));
    let combined = maximum.add(weights[1].current.mul(average.sub(maximum)));
    let blend = own.add(weights[0].current.mul(combined.sub(own)));
    L::select(gliding, blend, prepared)
}

/// [`link`] under a glide: the prepared law of each lane's own mode ([`LINK_MIXED`]), and on each
/// channel the blend of [`glide_level`] for the lanes still in flight. `MONO` links the left word
/// with itself and advances the left weights alone, as [`step`]'s collapsed body does, so a
/// collapsed lane's detector is the dual one on `L == R`.
#[inline(always)]
fn glide_link<L: Lane, const MONO: bool>(
    left: L,
    right: L,
    lanes: &LinkLanes<L>,
    weights_left: &mut [LinkRamp<L>; LINK_WEIGHTS],
    weights_right: &mut [LinkRamp<L>; LINK_WEIGHTS],
) -> (L, L) {
    let (prepared_left, prepared_right) = link::<L, LINK_MIXED>(left, right, lanes);
    let own_left = left.abs();
    let own_right = right.abs();
    let maximum = own_left.max(own_right);
    let half = L::splat(0.5);
    let average = half.mul(own_left).add(half.mul(own_right));
    let detector_left = glide_level(own_left, maximum, average, prepared_left, weights_left);
    if MONO {
        return (detector_left, detector_left);
    }
    let detector_right = glide_level(own_right, maximum, average, prepared_right, weights_right);
    (detector_left, detector_right)
}

/// Magnitude below which a follower word snaps to `+0.0` (issue #1784, owner ruling B2 of
/// 2026-10-23): `1e-12`, 240 dB under full scale and four decades under [`FLOOR`].
///
/// Passed to `ar_one_pole_step` in place of the D7 floor `FLUSH_EPS` (`1e-20`) for the two
/// envelope words, so a follower released from full scale reaches its exact rest about 129,000
/// samples after the signal stops at 48 kHz instead of 217,000. A word below `FLOOR` does not reach
/// the output while the input is silent; it does reach the next onset through `c * e + k * u`.
/// Measured right after the snap, that onset changes by at most `1.3e-5` dB from -100 dBFS up and
/// not at all from -80 dBFS up, but by up to `7.3e-5` dB at -120 dBFS and `7.0e-4` dB at -140 dBFS
/// (the #1784 decision record, D5).
const FOLLOWER_REST: f32 = 1.0e-12;

/// One frame of one channel. Frozen operation order.
///
/// 1. both followers advance (`ar_one_pole_step`: two products, one sum, one rest snap each, at
///    [`FOLLOWER_REST`])
/// 2. `ratio = max(fast, FLOOR) / max(slow, FLOOR)` — one IEEE division
/// 3. `contrast = fast_level_db(ratio)`, clamped `min` then `max` to ±24 dB (X7)
/// 4. `shape = attack * max(contrast, 0) + sustain * max(-contrast, 0)`, clamped to ±18 dB
/// 5. `gain = fast_gain_from_db(shape)` — `fast_gain_from_db(+0.0)` is exactly `1` (X8)
/// 6. `wet = gain_mix_step(x, gain, mix)` = `fma(mix, x * gain - x, x)`; `Lane::fma`
///    uses separate multiply/add rounding on every target
/// 7. `select(bypass or mix == 0 or shape == 0, x, wet)` — the signed-zero identity contract:
///    `fma(mix, +0.0, -0.0)` is `+0.0`, so the dry value has to be selected, not computed
///
/// There is no per-value finiteness or subnormal classification (D7): the two envelope words are
/// snapped inside the follower (at [`FOLLOWER_REST`], which subsumes the D7 flush) and the output
/// block is checked once, by the caller.
#[inline(always)]
fn frame<L: Lane>(x: L, u: L, c: &Coef<L>, e: &mut Env<L>, p: &Params<L>, bypass: L::Mask) -> L {
    e.fast = ar_one_pole_step(e.fast, u, &c.fast, FOLLOWER_REST);
    e.slow = ar_one_pole_step(e.slow, u, &c.slow, FOLLOWER_REST);
    let floor = L::splat(FLOOR);
    let ratio = e.fast.max(floor).div(e.slow.max(floor));
    // FAST-DB-CROSSING X7: detector contrast; MA-5 proves [1e-8, 16] and its clamp.
    #[expect(
        clippy::disallowed_methods,
        reason = "FAST-DB-CROSSING X7: transient-shaper detector contrast in [-24, 24] dB"
    )]
    let contrast = fast_level_db(ratio);
    let contrast = contrast
        .min(L::splat(CONTRAST_LIMIT_DB))
        .max(L::splat(-CONTRAST_LIMIT_DB));
    let zero = L::zero();
    let shape = p
        .attack
        .mul(contrast.max(zero))
        .add(p.sustain.mul(contrast.neg().max(zero)));
    let shape = shape
        .min(L::splat(SHAPE_LIMIT_DB))
        .max(L::splat(-SHAPE_LIMIT_DB));
    // FAST-DB-CROSSING X8: applied gain; the shaped argument is clamped to [-18, 18] dB.
    #[expect(
        clippy::disallowed_methods,
        reason = "FAST-DB-CROSSING X8: transient-shaper applied gain in [-18, 18] dB"
    )]
    let gain = fast_gain_from_db(shape);
    let wet = gain_mix_step(x, gain, p.mix);
    let identity = L::mask_or(bypass, L::mask_or(p.mix.eq(zero), shape.eq(zero)));
    L::select(identity, x, wet)
}

/// One frame of both channels: link, then [`frame`] per channel.
///
/// `MONO` (issue #1873) is the collapsed track: only `fl` is loaded, linked with **itself** and
/// rendered on the left channel's words; `fr`, `er` and `pr` are not read or written. A
/// collapsed lane's link partner is therefore its own word: every arm computes
/// `link(x, x, lanes)`, the dual arm's expression on `L == R` in the same operation order, and
/// the mixed arm selects per lane the law of that lane's own mode over the same pair. So each
/// lane's detector, and with it every bit of the left channel, is the dual block's on `L == R`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn step<L: Lane, const LINK: u8, const MONO: bool>(
    fl: &mut [f32],
    fr: &mut [f32],
    c: &Coef<L>,
    el: &mut Env<L>,
    er: &mut Env<L>,
    pl: &Params<L>,
    pr: &Params<L>,
    bypass: L::Mask,
    lanes: &LinkLanes<L>,
) {
    if MONO {
        let xl = L::load(fl);
        let (ul, _) = link::<L, LINK>(xl, xl, lanes);
        frame(xl, ul, c, el, pl, bypass).store(fl);
        return;
    }
    let (xl, xr) = (L::load(fl), L::load(fr));
    let (ul, ur) = link::<L, LINK>(xl, xr, lanes);
    frame(xl, ul, c, el, pl, bypass).store(fl);
    frame(xr, ur, c, er, pr, bypass).store(fr);
}

/// [`step`] for a frame with a link glide in flight (issue #1370 D3): the detectors are
/// [`glide_link`]'s, and everything after them is [`frame`], unchanged.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn glide_step<L: Lane, const MONO: bool>(
    fl: &mut [f32],
    fr: &mut [f32],
    c: &Coef<L>,
    el: &mut Env<L>,
    er: &mut Env<L>,
    pl: &Params<L>,
    pr: &Params<L>,
    bypass: L::Mask,
    lanes: &LinkLanes<L>,
    weights: &mut [[LinkRamp<L>; LINK_WEIGHTS]; 2],
) {
    let [weights_left, weights_right] = weights;
    if MONO {
        let xl = L::load(fl);
        let (ul, _) = glide_link::<L, true>(xl, xl, lanes, weights_left, weights_right);
        frame(xl, ul, c, el, pl, bypass).store(fl);
        return;
    }
    let (xl, xr) = (L::load(fl), L::load(fr));
    let (ul, ur) = glide_link::<L, false>(xl, xr, lanes, weights_left, weights_right);
    frame(xl, ul, c, el, pl, bypass).store(fl);
    frame(xr, ur, c, er, pr, bypass).store(fr);
}

/// The parameter ramps of one channel of `W` lanes, plus the defaults a full reset restores, and
/// the lanes' detector link glide weights (issue #1370 D2).
#[derive(Clone, Copy, Debug)]
struct Ramps<const W: usize> {
    ramps: [[LinearRamp; PARAMETER_COUNT]; W],
    defaults: [[f32; PARAMETER_COUNT]; W],
    /// Each lane's two link weights, `w_link` then `w_avg`: at rest each is exactly
    /// `effect_contract::link_glide_weights` of the lane's mode. Only a glide in flight is read,
    /// by [`Shaper::run_glide`], which advances them; [`Ramps::advance`] does not.
    links: [[LinearRamp; LINK_WEIGHTS]; W],
}

impl<const W: usize> Ramps<W> {
    /// The ramps at rest at `defaults`, and the weights at rest at dual mono: the shaper seats
    /// them at each lane's mode ([`Shaper::rest_links`]).
    fn new(defaults: [[f32; PARAMETER_COUNT]; W]) -> Self {
        Self {
            ramps: defaults.map(|lane| lane.map(LinearRamp::fixed)),
            defaults,
            links: [[LinearRamp::fixed(0.0); LINK_WEIGHTS]; W],
        }
    }

    /// Frames still to be ramped by any lane, parameter or link weight: zero in steady state
    /// (finding F6). A glide rides this prefix (issue #1370).
    fn prefix(&self) -> usize {
        self.ramps
            .iter()
            .flatten()
            .chain(self.links.iter().flatten())
            .map(|ramp| ramp.remaining as usize)
            .max()
            .unwrap_or(0)
    }

    /// Whether any lane's link weight is in flight (issue #1370).
    fn gliding(&self) -> bool {
        self.links.iter().flatten().any(LinearRamp::is_ramping)
    }

    /// The next sample's values, packed across lanes. Each lane runs the runtime's scalar
    /// `LinearRamp::next_value`, so a bank ramp and a scalar ramp are the same law by construction.
    #[inline(always)]
    fn advance<L: Lane>(&mut self) -> Params<L> {
        debug_assert_eq!(W, L::WIDTH);
        let mut packed = [[0.0_f32; 8]; PARAMETER_COUNT];
        for (lane, ramps) in self.ramps.iter_mut().enumerate() {
            for (index, ramp) in ramps.iter_mut().enumerate() {
                packed[index][lane] = ramp.next_value();
            }
        }
        Params {
            attack: L::load(&packed[0]),
            sustain: L::load(&packed[1]),
            mix: L::load(&packed[2]),
        }
    }

    /// The resting values, packed across lanes: the block-constant suffix after the ramp prefix.
    #[inline(always)]
    fn current<L: Lane>(&self) -> Params<L> {
        debug_assert_eq!(W, L::WIDTH);
        let mut packed = [[0.0_f32; 8]; PARAMETER_COUNT];
        for (lane, ramps) in self.ramps.iter().enumerate() {
            for (index, ramp) in ramps.iter().enumerate() {
                packed[index][lane] = ramp.current;
            }
        }
        Params {
            attack: L::load(&packed[0]),
            sustain: L::load(&packed[1]),
            mix: L::load(&packed[2]),
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Lane `lane`'s parameter ramps re-seeded from its defaults, at rest. The link weights are
    /// the shaper's to re-seat at the lane's mode ([`Shaper::reset_lane`]).
    fn full_reset_lane(&mut self, lane: usize) {
        self.ramps[lane] = self.defaults[lane].map(LinearRamp::fixed);
    }
    // REALTIME_POLICY_END

    /// Lane `lane`'s ramps snapped to their targets, a link glide included: it lands on the
    /// lane's mode.
    fn snap_lane(&mut self, lane: usize) {
        for ramp in self.ramps[lane].iter_mut().chain(&mut self.links[lane]) {
            ramp.snap();
        }
    }
}

/// One prepared transient shaper over `W` lanes: `W = 1` is the scalar product, `W = 4`/`8` a bank.
///
/// It keeps no copy of its `PreparedEffectMetadata` (issue #1461): the prepare result carries that
/// record on the control side, and the shaper keeps only the prepared values its own code reads.
struct Shaper<L: Lane, const W: usize> {
    coefficients: Coef<L>,
    left_env: Env<L>,
    right_env: Env<L>,
    left: Ramps<W>,
    right: Ramps<W>,
    /// The session's bypass, read by `run`.
    bypass: bool,
    /// Each lane's prepared detector-link mode (issue #1368), read by the lane swap.
    links: [LinkMode; W],
    /// `links` as masks and the arm they pick, read by `process_block`.
    link_lanes: LinkLanes<L>,
    /// The prepared automation span capacity, read by `apply_automation`.
    automation_capacity: u32,
    /// The prepared state payload sizes, read by the snapshot and restore length checks.
    state_sizes: StatePayloadSizes,
    nonfinite: NonFiniteReport,
    /// Bit `l` is set when lane `l` carries a member (issue #1092). A padded lane's bit is clear:
    /// it takes no automation, and D7 recovers it without charging it to `nonfinite` (the padding
    /// contract on `PrepareEffectBankRequest`). Every lane of a scalar instance and of a full bank
    /// is active.
    active: u32,
    /// Whether a collapsed block (`process_block::<true>`) has rendered since the last disengage
    /// copy ([`desymmetrize`](Self::desymmetrize)), which alone clears it (#1894). While it is set
    /// the right channel is frozen and stale, so [`lanes_at_rest`](Self::lanes_at_rest) reads the
    /// left channel alone: the effect contract's collapsed-bank rule. The shaper has no clocks, so
    /// nothing else reads it.
    collapsed: bool,
}

impl<L: Lane, const W: usize> Shaper<L, W> {
    /// A shaper keeping, of `metadata`, only the values its own code reads.
    fn new(
        metadata: &PreparedEffectMetadata,
        row: [f32; 4],
        left: [[f32; PARAMETER_COUNT]; W],
        right: [[f32; PARAMETER_COUNT]; W],
    ) -> Self {
        Self {
            coefficients: Coef::new(row),
            left_env: Env::default(),
            right_env: Env::default(),
            left: Ramps::new(left),
            right: Ramps::new(right),
            bypass: metadata.bypass,
            links: [metadata.link_mode; W],
            link_lanes: LinkLanes::new(&[metadata.link_mode; W]),
            automation_capacity: metadata.automation_capacity,
            state_sizes: metadata.state_sizes,
            nonfinite: NonFiniteReport::new(),
            active: (1 << W) - 1,
            collapsed: false,
        }
        .with_links_at_rest()
    }

    /// Seats every lane's link weights at rest at its mode (issue #1370 D2).
    fn with_links_at_rest(mut self) -> Self {
        for lane in 0..W {
            self.rest_links(lane);
        }
        self
    }

    /// Seats lane `lane`'s link weights at rest at its mode on both channels (issue #1370 D2):
    /// preparation, a full reset, a padded lane that follows a new first member, and a lane whose
    /// mode a lane swap replaced. The D7 recovery leaves a glide in flight alone.
    fn rest_links(&mut self, lane: usize) {
        let weights = effect_contract::link_glide_weights(self.links[lane]);
        for ramps in [&mut self.left, &mut self.right] {
            ramps.links[lane] = weights.map(LinearRamp::fixed);
        }
    }

    // REALTIME_POLICY_BEGIN: #1370, a link glide is retargeted at a block boundary.
    /// Glides lane `lane`'s detector link to `mode` over `samples` samples (issue #1370 D1-D3).
    ///
    /// The lane's mode, and with it its masks and the bank's arm, becomes `mode` at once, so a
    /// glide that arrives lands on `mode`'s own law; the weights of both channels, retargeted
    /// from the words in force, carry the transition. The mode is the one a full reset keeps and
    /// a lane swap moves. `false`, changing nothing, for a lane past the width or padded, a mode
    /// the descriptor does not support or a glide longer than `LINK_GLIDE_MAX_SAMPLES`.
    /// Allocation-free and bounded by the lane count.
    fn retarget_link(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        if lane >= W
            || self.active & (1 << lane) == 0
            || !TRANSIENT_SHAPER_DESCRIPTOR
                .supported_link_modes
                .contains(mode)
            || samples > effect_contract::LINK_GLIDE_MAX_SAMPLES
        {
            return false;
        }
        self.set_lane_link(lane, mode);
        let weights = effect_contract::link_glide_weights(mode);
        for ramps in [&mut self.left, &mut self.right] {
            for (ramp, &weight) in ramps.links[lane].iter_mut().zip(&weights) {
                ramp.set_target(weight, samples);
            }
        }
        true
    }
    // REALTIME_POLICY_END

    /// Sets lane `lane`'s link mode and re-derives the masks and the arm (issue #1368).
    /// Allocation-free, so the render-time lane swap may call it.
    ///
    /// D5 holds after the swap too: every padded lane re-takes the first active lane's mode, so a
    /// bank whose members became uniform leaves the mixed arm. A padded lane's output is
    /// discarded, so this moves no member's bit.
    fn set_lane_link(&mut self, lane: usize, link: LinkMode) {
        self.links[lane] = link;
        let first = self.active.trailing_zeros() as usize;
        if first < W {
            let mode = self.links[first];
            for padded in 0..W {
                if self.active & (1 << padded) == 0 && self.links[padded] != mode {
                    self.links[padded] = mode;
                    // A padded lane follows at rest (issue #1370): its output is discarded.
                    self.rest_links(padded);
                }
            }
        }
        self.link_lanes = LinkLanes::new(&self.links);
    }

    fn reset(&mut self, kind: ResetKind) {
        for lane in 0..W {
            self.reset_lane(lane, kind);
        }
    }

    // REALTIME_POLICY_BEGIN
    /// `PreparedNativeEffectBank::reset_lane` (issue #2131): lane `lane`'s part of
    /// [`reset`](Self::reset) on both channels, and nothing of any other lane. Its envelope words
    /// clear; a full reset reseeds its ramps from its defaults and seats its link weights at rest
    /// at its mode, a discontinuity snaps its ramps and link weights. The shaper has no clocks
    /// and no cached bank flag (the rest claim is read from state), and `collapsed` stays as the
    /// whole-bank reset leaves it. A lane past the width changes nothing. Bounded by one lane.
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        if lane >= W {
            return;
        }
        self.left_env.clear_lane(lane);
        self.right_env.clear_lane(lane);
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset_lane(lane);
                self.right.full_reset_lane(lane);
                // Issue #1370: the lane keeps its mode, its weights at rest.
                self.rest_links(lane);
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.snap_lane(lane);
                self.right.snap_lane(lane);
            }
        }
    }
    // REALTIME_POLICY_END

    /// Renders one block in place and applies the master plan §4.4 boundary check.
    ///
    /// # The silent fast path (issue #1746)
    ///
    /// Called after the block's automation was applied. When both input planes are exact `+0.0`
    /// and [`lanes_at_rest`](Self::lanes_at_rest) has every lane set (padded lanes included), the
    /// kernel is skipped: by the rest claim it would leave every state word bit-identical and write
    /// the `+0.0` the buffers already hold, so the boundary check would pass and the D7 record is
    /// untouched. The shaper has no clocks and no bank-level counter a clean block moves, so there
    /// is nothing to advance. The claim is decided from state at every quiet block; nothing is
    /// cached, so no state change can leave a stale claim behind. The quiet input is tested first,
    /// so a block with signal pays one early-exiting bit scan. Bit tests and integer compares
    /// only: the predicate adds no `f32` arithmetic to the kernel. The scalar instance (`W = 1`)
    /// renders through here as well and takes the same path.
    ///
    /// Returns, per channel (`[left, right]`), the bitmask of the active lanes whose own output on
    /// that channel failed the D7 check: each is one nonfinite **block** on that channel (#1073).
    ///
    /// # The collapsed body (issue #1873)
    ///
    /// `MONO` renders the left channel alone, on the left plane alone: `right` is never read or
    /// written (a collapsed block passes it empty), and the right channel's envelopes and ramps
    /// are left exactly as they were, for [`desymmetrize`](Self::desymmetrize) to replace. The
    /// silent fast path reads the left plane and the left channel's rest only, because a
    /// collapsed bank's counterfactual right channel is its left one. A lane that fails the D7
    /// check on the left plane is recovered on **both** channels and reported on both, as the
    /// dual block on `L == R` fails both planes in the same lanes.
    ///
    /// `#[inline(always)]`, so the dual and the collapsed body stay two symbols, `process_bank`
    /// and `process_bank_mono`, each with the kernel inlined (the wasm kernel roster names them).
    #[inline(always)]
    fn process_block<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) -> [u32; 2] {
        debug_assert_eq!(W, L::WIDTH);
        if MONO {
            // From here until the disengage copy the right channel is frozen and stale.
            self.collapsed = true;
        }
        let silent = if MONO {
            block_is_positive_zero(left)
                && silent_allowed()
                && channel_rest_lanes(&self.left_env, &self.left) == all_lanes::<W>()
        } else {
            block_is_positive_zero(left)
                && block_is_positive_zero(right)
                && silent_allowed()
                && self.lanes_at_rest() == all_lanes::<W>()
        };
        if silent {
            #[cfg(test)]
            SILENT_ADMISSIONS.with(|blocks| blocks.set(blocks.get() + 1));
            return [0, 0];
        }
        // Issue #1370: a glide in flight renders the block's ramp prefix through the glide body;
        // the rest of the block, and every block with no glide, renders through the arm below.
        let gliding = self.left.gliding() || (!MONO && self.right.gliding());
        let head = if gliding {
            self.run_glide::<MONO>(left, right, frames)
        } else {
            0
        };
        if head < frames {
            let split = head * L::WIDTH;
            let (left_rest, right_rest) = if MONO {
                (&mut left[split..], &mut [][..])
            } else {
                (&mut left[split..], &mut right[split..])
            };
            let rest = frames - head;
            match self.link_lanes.arm {
                LINK_DUAL_MONO => self.run::<LINK_DUAL_MONO, MONO>(left_rest, right_rest, rest),
                LINK_MAXIMUM => self.run::<LINK_MAXIMUM, MONO>(left_rest, right_rest, rest),
                LINK_AVERAGE => self.run::<LINK_AVERAGE, MONO>(left_rest, right_rest, rest),
                _ => {
                    #[cfg(test)]
                    MIXED_LINK_BLOCKS.with(|blocks| blocks.set(blocks.get() + 1));
                    self.run::<LINK_MIXED, MONO>(left_rest, right_rest, rest);
                }
            }
        }
        if MONO {
            if check_block::<L>(left) {
                return [0, 0];
            }
            let failed = nonfinite_lane_mask::<L>(left);
            let charged = self.recover_lanes(left, &mut [], failed);
            return [failed & charged; 2];
        }
        if check_block::<L>(left) && check_block::<L>(right) {
            return [0, 0];
        }
        let failed = [
            nonfinite_lane_mask::<L>(left),
            nonfinite_lane_mask::<L>(right),
        ];
        let charged = self.recover_lanes(left, right, failed[0] | failed[1]);
        [failed[0] & charged, failed[1] & charged]
    }

    /// The lanes at their **exact rest** on both channels (`PreparedNativeEffectBank::lanes_at_rest`,
    /// issue #1746), as a bitmask over `W` lanes: from this state, one block of exact `+0.0` input
    /// leaves every state word of the lane bit-identical and writes `+0.0` on both planes.
    ///
    /// A lane's state words are, per channel, the two envelope words `fast` and `slow` and the
    /// four fields of each of the three parameter ramps and the two link weights (#1370): the
    /// twenty-two-word payload. `Ramps::defaults`
    /// is a reset value the kernel never reads, and the coefficients, bypass and link mode are
    /// prepared constants. Pinned, per lane and per channel:
    ///
    /// * `fast` and `slow` to `+0.0` by raw bits. Fed `+0.0`, every link mode's detector is `+0.0`
    ///   (`abs`, `max` of two `+0.0`, `0.5 * +0 + 0.5 * +0`); the follower is not rising
    ///   (`+0 > +0` is false), takes the release pair, and `c * +0 + k * +0` with positive `c`, `k`
    ///   is `+0.0`, which the [`FOLLOWER_REST`] snap keeps. So `+0.0` is a fixed point. A follower decaying to a
    ///   non-zero word is refused, even one no `+0.0` block would move.
    /// * every ramp settled (`remaining == 0`): `next_value` then returns `current` and changes no
    ///   field, and `current` is a read, so all four fields stay put whatever the bank's prefix is.
    ///   A ramp in flight moves its `current` and `remaining`, so it is refused.
    ///
    /// The output is then `+0.0`: both followers floor, `ratio = FLOOR / FLOOR` is exactly `1`,
    /// `fast_level_db(1.0)` is exactly `+0.0` (`math::fast_db`), so the contrast is `+0.0`, the
    /// shape `attack * +0 + sustain * +0` is a signed zero for every legal amount, the clamps keep
    /// it a zero, and `shape == 0` selects the input word, `+0.0`. Bypass and `mix == 0` select the
    /// same input word, and the followers run under bypass too, so neither changes the answer.
    /// That holds for every rate's coefficients and every parameter value in the domain.
    ///
    /// The list is complete: [`run`](Self::run) writes `left_env`, `right_env` and the ramps and
    /// nothing else; the boundary check passes on a `+0.0` block, so [`recover_lanes`](Self::recover_lanes) and the D7
    /// record are not reached. The shaper has **no clocks**.
    ///
    /// While [`collapsed`](Self::collapsed) the left channel answers alone (#1894).
    ///
    /// No allocation, bounded time, `&self`: four bit masks and `6 * W` integer compares.
    fn lanes_at_rest(&self) -> u32 {
        let left = channel_rest_lanes(&self.left_env, &self.left);
        if self.collapsed {
            left
        } else {
            left & channel_rest_lanes(&self.right_env, &self.right)
        }
    }

    /// The D7 recovery, one lane at a time (issue #1092; decision 12's coupling rule).
    ///
    /// A lane that failed the boundary check on either channel has both of its channels zeroed and
    /// both of its envelopes reset, which is exactly what the scalar instance does to its one lane;
    /// every other lane is left bit-unchanged. The shared `effect_runtime::bank::finish_block`
    /// zeroes and resets the whole bank instead, so one hot track -- a bypassed one whose wet path
    /// still runs (#1087) -- would silence its bank-mates. The ramps are untouched, as before: a
    /// reset clears history, not automation.
    ///
    /// A padded lane is recovered like any other, so it stays at rest, but only active lanes are
    /// charged to `nonfinite`. Returns the charged lanes. A collapsed block passes `right` empty;
    /// both channels' envelopes are still reset.
    fn recover_lanes(&mut self, left: &mut [f32], right: &mut [f32], failed: u32) -> u32 {
        for lane in 0..W {
            if failed & (1 << lane) == 0 {
                continue;
            }
            for word in left.iter_mut().skip(lane).step_by(W) {
                *word = 0.0;
            }
            for word in right.iter_mut().skip(lane).step_by(W) {
                *word = 0.0;
            }
            self.left_env.clear_lane(lane);
            self.right_env.clear_lane(lane);
        }
        let charged = failed & self.active;
        if charged != 0 {
            self.nonfinite.nonfinite_lanes = charged;
            self.nonfinite.nonfinite_blocks = self.nonfinite.nonfinite_blocks.saturating_add(1);
        }
        charged
    }

    /// The frame loop. `MONO` (issue #1873) walks the left plane, the left envelopes and the left
    /// ramps only, with the ramp prefix of the left ramps: the split is partition-invariant (a
    /// settled ramp's `advance` returns its `current`), and on `L == R` it is the dual one.
    #[inline(always)]
    fn run<const LINK: u8, const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) {
        let bypass = L::splat(if self.bypass { 1.0 } else { 0.0 }).gt(L::zero());
        let lanes = self.link_lanes;
        let prefix = if MONO {
            frames.min(self.left.prefix())
        } else {
            frames.min(self.left.prefix().max(self.right.prefix()))
        };
        let split = prefix * L::WIDTH;
        let (head_left, tail_left) = left.split_at_mut(split);
        // The envelopes live in locals across the whole block, not behind a `&mut` per frame.
        let (mut el, mut er) = (self.left_env, self.right_env);
        if MONO {
            for fl in head_left.chunks_exact_mut(L::WIDTH) {
                let pl = self.left.advance::<L>();
                step::<L, LINK, true>(
                    fl,
                    &mut [],
                    &self.coefficients,
                    &mut el,
                    &mut er,
                    &pl,
                    &pl,
                    bypass,
                    &lanes,
                );
            }
            let pl = self.left.current::<L>();
            for fl in tail_left.chunks_exact_mut(L::WIDTH) {
                step::<L, LINK, true>(
                    fl,
                    &mut [],
                    &self.coefficients,
                    &mut el,
                    &mut er,
                    &pl,
                    &pl,
                    bypass,
                    &lanes,
                );
            }
            self.left_env = el;
            return;
        }
        let (head_right, tail_right) = right.split_at_mut(split);
        for (fl, fr) in head_left
            .chunks_exact_mut(L::WIDTH)
            .zip(head_right.chunks_exact_mut(L::WIDTH))
        {
            let pl = self.left.advance::<L>();
            let pr = self.right.advance::<L>();
            step::<L, LINK, false>(
                fl,
                fr,
                &self.coefficients,
                &mut el,
                &mut er,
                &pl,
                &pr,
                bypass,
                &lanes,
            );
        }
        let pl = self.left.current::<L>();
        let pr = self.right.current::<L>();
        for (fl, fr) in tail_left
            .chunks_exact_mut(L::WIDTH)
            .zip(tail_right.chunks_exact_mut(L::WIDTH))
        {
            step::<L, LINK, false>(
                fl,
                fr,
                &self.coefficients,
                &mut el,
                &mut er,
                &pl,
                &pr,
                bypass,
                &lanes,
            );
        }
        self.left_env = el;
        self.right_env = er;
    }

    /// The ramp prefix of a block with a link glide in flight (issue #1370 D3): the first
    /// `min(frames, prefix)` frames, every ramp of the rendered channels advanced per frame and
    /// each detector [`glide_link`]'s. Returns the frames rendered. After them every ramp of the
    /// rendered channels has settled, or the block has ended, so [`run`](Self::run) renders the
    /// rest with no prefix. Rendering the prefix here and the rest there is partition-invariant:
    /// each is the frame loop over the same state.
    ///
    /// Out of the per-arm [`run`](Self::run): the glide body does not depend on the block's arm
    /// (a resting lane takes its own mode's law from its masks), so it is one body, not four.
    #[inline(always)]
    fn run_glide<const MONO: bool>(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
    ) -> usize {
        let bypass = L::splat(if self.bypass { 1.0 } else { 0.0 }).gt(L::zero());
        let lanes = self.link_lanes;
        let head = if MONO {
            frames.min(self.left.prefix())
        } else {
            frames.min(self.left.prefix().max(self.right.prefix()))
        };
        let split = head * L::WIDTH;
        let gather = |ramps: &Ramps<W>| {
            [
                LinkRamp::<L>::gather(&ramps.links, 0),
                LinkRamp::<L>::gather(&ramps.links, 1),
            ]
        };
        let mut weights = [gather(&self.left), gather(&self.right)];
        let (mut el, mut er) = (self.left_env, self.right_env);
        if MONO {
            for fl in left[..split].chunks_exact_mut(L::WIDTH) {
                let pl = self.left.advance::<L>();
                glide_step::<L, true>(
                    fl,
                    &mut [],
                    &self.coefficients,
                    &mut el,
                    &mut er,
                    &pl,
                    &pl,
                    bypass,
                    &lanes,
                    &mut weights,
                );
            }
        } else {
            for (fl, fr) in left[..split]
                .chunks_exact_mut(L::WIDTH)
                .zip(right[..split].chunks_exact_mut(L::WIDTH))
            {
                let pl = self.left.advance::<L>();
                let pr = self.right.advance::<L>();
                glide_step::<L, false>(
                    fl,
                    fr,
                    &self.coefficients,
                    &mut el,
                    &mut er,
                    &pl,
                    &pr,
                    bypass,
                    &lanes,
                    &mut weights,
                );
            }
        }
        let [weights_left, weights_right] = weights;
        for (weight, ramp) in weights_left.into_iter().enumerate() {
            ramp.scatter(&mut self.left.links, weight);
        }
        self.left_env = el;
        if !MONO {
            for (weight, ramp) in weights_right.into_iter().enumerate() {
                ramp.scatter(&mut self.right.links, weight);
            }
            self.right_env = er;
        }
        head
    }

    /// The collapse's disengage copy (issue #1873): the left channel's whole render state onto
    /// the right, every lane. That is every per-channel word [`run`](Self::run) reads: the two
    /// envelope words (`fast`, `slow`) and the three ramps, every field. Both are `Copy` values,
    /// so the copy allocates nothing.
    ///
    /// `Ramps::defaults` is not copied: each channel's own prepared values, which the kernel never
    /// reads (a full reset, preparation-side, re-seeds from them, and a lane swap exchanges them
    /// per channel). The link modes and masks, the coefficients and the bypass are whole-lane or
    /// whole-instance: one word serves both channels.
    fn desymmetrize(&mut self) {
        self.right_env = self.left_env;
        self.right.ramps = self.left.ramps;
        // Issue #1370: the link weights are per-channel render state too.
        self.right.links = self.left.links;
        self.collapsed = false;
    }

    /// `PreparedNativeEffectBank::channels_agree` (issue #1873): every word
    /// [`desymmetrize`](Self::desymmetrize) copies compares bit-equal between the channels, on
    /// every lane: the two envelope words, the twelve parameter ramp words and the eight link
    /// weight words (#1370), twenty-two per lane, compared outright.
    fn channels_bit_equal(&self) -> bool {
        same_lane_bits(self.left_env.fast, self.right_env.fast, W)
            && same_lane_bits(self.left_env.slow, self.right_env.slow, W)
            && self
                .left
                .ramps
                .iter()
                .flatten()
                .chain(self.left.links.iter().flatten())
                .zip(
                    self.right
                        .ramps
                        .iter()
                        .flatten()
                        .chain(self.right.links.iter().flatten()),
                )
                .all(|(a, b)| same_ramp_bits(a, b))
    }

    /// The `DESIGNED` term of the channel-symmetry witness for `lane` (issue #1873): whether every
    /// per-channel word [`run`](Self::run) reads that is not running state compares bit-equal
    /// between the two channels.
    ///
    /// # The word list, and why it is exactly this
    ///
    /// Per channel the kernel reads the lane's three ramps (`ramps[lane]`, all four fields each:
    /// `remaining` decides the ramp prefix and whether a ramp moves, and `current`, `target` and
    /// `step` are what it moves) and the two envelope words. Deliberately excluded:
    ///
    /// * `fast` and `slow`: running state, which the induction the witness states carries.
    /// * `Ramps::defaults`: prepared values the kernel never loads.
    /// * `links` and `link_lanes`: the lane's link mode is one word for both channels, so it
    ///   cannot differ between them; `coefficients` and `bypass` are whole-instance.
    ///
    /// Raw bits, so `+0.0` against `-0.0` declines. A lane past the width declines.
    fn designed_channel_symmetry(&self, lane: usize) -> bool {
        lane < W
            && self.left.ramps[lane]
                .iter()
                .chain(&self.left.links[lane])
                .zip(self.right.ramps[lane].iter().chain(&self.right.links[lane]))
                .all(|(a, b)| same_ramp_bits(a, b))
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload codec runs in the plan-swap block.
    fn snapshot(
        &self,
        lane: usize,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.state_sizes,
        )?;
        write_lane(output.left, &self.left_env, &self.left, lane);
        write_lane(output.right, &self.right_env, &self.right, lane);
        Ok(())
    }

    fn restore(
        &mut self,
        lane: usize,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if state_layout_version != TRANSIENT_SHAPER_DESCRIPTOR.state_layout_version {
            return Err(state_error("effect.state.version"));
        }
        validate_state_lengths(
            input.common.len(),
            input.left.len(),
            input.right.len(),
            self.state_sizes,
        )?;
        let left = read_lane(input.left)?;
        let right = read_lane(input.right)?;
        if left.mode != right.mode {
            return Err(state_error("effect.state.link"));
        }
        self.left_env.fast = replace_lane(self.left_env.fast, lane, left.fast);
        self.left_env.slow = replace_lane(self.left_env.slow, lane, left.slow);
        self.right_env.fast = replace_lane(self.right_env.fast, lane, right.fast);
        self.right_env.slow = replace_lane(self.right_env.slow, lane, right.slow);
        self.left.ramps[lane] = left.ramps;
        self.right.ramps[lane] = right.ramps;
        // Issue #1370 D6: the weights land only under the mode they glide to. A payload taken
        // under another mode (the plan-swap carry into a successor prepared with a new mode)
        // leaves the lane's own mode at rest, as before the glide existed.
        if left.mode == self.links[lane] {
            self.left.links[lane] = left.links;
            self.right.links[lane] = right.links;
        } else {
            self.rest_links(lane);
        }
        Ok(())
    }
}

/// The lanes of one channel at exact rest (issue #1746): both envelope words `+0.0` by raw bits
/// and every ramp settled. See [`Shaper::lanes_at_rest`] for why this is the claim.
fn channel_rest_lanes<L: Lane, const W: usize>(env: &Env<L>, ramps: &Ramps<W>) -> u32 {
    let mut mask = positive_zero_lanes(env.fast) & positive_zero_lanes(env.slow);
    for (lane, (ramps, links)) in ramps.ramps.iter().zip(&ramps.links).enumerate() {
        // Issue #1370: a link glide in flight moves its weights every frame.
        if ramps.iter().chain(links).any(|ramp| ramp.remaining != 0) {
            mask &= !(1 << lane);
        }
    }
    mask
}

/// Whether the first `lanes` lanes of two packed words hold the same raw bits (issue #1873).
fn same_lane_bits<L: Lane>(a: L, b: L, lanes: usize) -> bool {
    let (mut left, mut right) = ([0.0_f32; 8], [0.0_f32; 8]);
    a.store(&mut left);
    b.store(&mut right);
    left[..lanes]
        .iter()
        .zip(&right[..lanes])
        .all(|(a, b)| a.to_bits() == b.to_bits())
}

/// Whether two ramps hold the same four words, raw bits (issue #1873).
fn same_ramp_bits(a: &LinearRamp, b: &LinearRamp) -> bool {
    a.current.to_bits() == b.current.to_bits()
        && a.target.to_bits() == b.target.to_bits()
        && a.step.to_bits() == b.step.to_bits()
        && a.remaining == b.remaining
}

/// Writes one lane's twenty-two state words into a payload section.
fn write_lane<L: Lane, const W: usize>(
    bytes: &mut [u8],
    env: &Env<L>,
    ramps: &Ramps<W>,
    lane: usize,
) {
    let mut fast = [0.0_f32; 8];
    let mut slow = [0.0_f32; 8];
    env.fast.store(&mut fast);
    env.slow.store(&mut slow);
    write_f32(bytes, 0, fast[lane]);
    write_f32(bytes, 1, slow[lane]);
    for (index, ramp) in ramps.ramps[lane].iter().enumerate() {
        write_ramp(bytes, 2 + index * RAMP_WORDS, *ramp);
    }
    for (index, ramp) in ramps.links[lane].iter().enumerate() {
        write_ramp(bytes, LINK_WORD + index * RAMP_WORDS, *ramp);
    }
}

/// The twenty-two words of one restored lane: the two envelope words, the three parameter ramps
/// and the two link weights, with the mode the weights glide to (issue #1370 D6).
struct LaneWords {
    fast: f32,
    slow: f32,
    ramps: [LinearRamp; PARAMETER_COUNT],
    links: [LinearRamp; LINK_WEIGHTS],
    mode: LinkMode,
}

/// Validates and decodes one lane's twenty-two state words, reading the payload bytes in place.
///
/// Each ramp is read whole, its step included (#1278 D2a), and its every remaining value is
/// checked against the parameter's strict domain (`ramp_path_inside`): the clamped law keeps every
/// word between the ramp's start and its target (issue #1409 D2), so a word past the domain is one
/// the engine never holds (issue #1411 D1). The returned value is a few words on the stack:
/// nothing here allocates.
fn read_lane(bytes: &[u8]) -> Result<LaneWords, StatePayloadError> {
    let fast = read_f32(bytes, 0);
    let slow = read_f32(bytes, 1);
    if !valid_envelope(fast) || !valid_envelope(slow) {
        return Err(state_error("effect.state.envelope"));
    }
    let mut ramps = [LinearRamp::fixed(0.0); PARAMETER_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let read = read_ramp(bytes, 2 + index * RAMP_WORDS);
        let parameter = &TRANSIENT_SHAPER_PARAMETERS[index];
        let (low, high) = parameter
            .minimum
            .zip(parameter.maximum)
            .ok_or(state_error("effect.state.parameter"))?;
        // The target and the current, moving or settled, are held to the strict domain.
        if !value_valid(parameter, read.current)
            || !value_valid(parameter, read.target)
            || !ramp_path_inside(read, (low, high), RAMP_SAMPLES)
        {
            return Err(state_error("effect.state.parameter"));
        }
        *ramp = LinearRamp {
            current: normalize_zero(read.current),
            target: normalize_zero(read.target),
            ..read
        };
    }
    // Issue #1370 D6: each weight lies in `[0, 1]` on its whole path, the engine never holds
    // `-0.0` there, a glide in flight moves (its step is `(t - c) / n` with `t` in `{0, 1}`, `c` a
    // word of an earlier glide and `n <= 2^24`, never `0`), a settled weight holds its target,
    // and the two targets are exactly one supported mode's rest weights.
    let mut links = [LinearRamp::fixed(0.0); LINK_WEIGHTS];
    for (index, link) in links.iter_mut().enumerate() {
        let read = read_ramp(bytes, LINK_WORD + index * RAMP_WORDS);
        let valid = read.current.to_bits() != (-0.0_f32).to_bits()
            && (read.remaining == 0 || read.step != 0.0)
            && (read.remaining != 0 || read.current.to_bits() == read.target.to_bits())
            && ramp_path_inside(read, (0.0, 1.0), effect_contract::LINK_GLIDE_MAX_SAMPLES);
        if !valid {
            return Err(state_error("effect.state.link"));
        }
        *link = read;
    }
    let mode = effect_contract::link_glide_mode([links[0].target, links[1].target])
        .filter(|mode| {
            TRANSIENT_SHAPER_DESCRIPTOR
                .supported_link_modes
                .contains(*mode)
        })
        .ok_or(state_error("effect.state.link"))?;
    Ok(LaneWords {
        fast,
        slow,
        ramps,
        links,
        mode,
    })
}

fn valid_envelope(value: f32) -> bool {
    (value.is_normal() && value.is_sign_positive()) || value.to_bits() == 0.0_f32.to_bits()
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}
// REALTIME_POLICY_END

/// The runtime parameter domain of one contract parameter row.
fn domain(parameter: &ParameterDescriptor) -> Option<ParameterSpec> {
    let (minimum, maximum) = parameter.minimum.zip(parameter.maximum)?;
    Some(ParameterSpec::continuous(
        minimum,
        maximum,
        parameter.default_value,
    ))
}

fn value_valid(parameter: &ParameterDescriptor, value: f32) -> bool {
    domain(parameter).is_some_and(|spec| spec_value_valid(&spec, value))
}

/// Extracts the per-channel initial values. The contract's `validate_initial_values` has already
/// checked count, order, channel, domain and `-0.0` inside `expected_prepared_metadata`.
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
    for index in 0..PARAMETER_COUNT {
        left[index] = normalize_zero(values[index * 2].value);
        right[index] = normalize_zero(values[index * 2 + 1].value);
    }
    Ok((left, right))
}

/// Applies one block's automation spans to the ramps of one lane.
///
/// Block-rate `Point` spans at `first_sample` only, in ascending `(parameter, channel)` order, at
/// most one per parameter per channel, within the block's automation capacity — every other span is
/// counted as invalid and dropped. Retargeting is the D11 `set_target`: one division, at event time.
fn apply_automation<const W: usize>(
    spans: &[PreparedAutomationSpan],
    automation_capacity: u32,
    first_sample: u64,
    lane: usize,
    left: &mut Ramps<W>,
    right: &mut Ramps<W>,
    report: &mut ProcessReport,
) {
    let mut pending = [[None; PARAMETER_COUNT]; 2];
    let mut last_order = None;
    for (span_index, span) in spans.iter().enumerate() {
        let lane_index = match span.channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => {
                report.invalid_spans = report.invalid_spans.saturating_add(1);
                continue;
            }
        };
        let parameter_index = span.parameter_index as usize;
        let Some(order) = span
            .parameter_index
            .checked_mul(2)
            .and_then(|value| value.checked_add(lane_index as u32))
        else {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        };
        let valid = span_index < automation_capacity as usize
            && parameter_index < PARAMETER_COUNT
            && span.sample == first_sample
            && value_valid(&TRANSIENT_SHAPER_PARAMETERS[parameter_index], span.value)
            && last_order.is_none_or(|previous| order > previous)
            && pending[lane_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[lane_index][parameter_index] = Some(normalize_zero(span.value));
    }
    for (parameter_index, (left_pending, right_pending)) in
        pending[0].iter().zip(pending[1].iter()).enumerate()
    {
        if let Some(value) = *left_pending {
            left.ramps[lane][parameter_index].set_target(value, RAMP_SAMPLES);
        }
        if let Some(value) = *right_pending {
            right.ramps[lane][parameter_index].set_target(value, RAMP_SAMPLES);
        }
    }
}

fn validate_state_lengths(
    common_bytes: usize,
    left_bytes: usize,
    right_bytes: usize,
    sizes: StatePayloadSizes,
) -> Result<(), StatePayloadError> {
    if common_bytes != sizes.common_bytes as usize
        || left_bytes != sizes.left_bytes as usize
        || right_bytes != sizes.right_bytes as usize
    {
        return Err(state_error("effect.state.length"));
    }
    Ok(())
}

fn checked_track(track_index: u32, width: usize) -> Result<usize, StatePayloadError> {
    let track = usize::try_from(track_index).map_err(|_| state_error("effect.state.track"))?;
    if track >= width {
        return Err(state_error("effect.state.track"));
    }
    Ok(track)
}

/// The mask with every one of `W` lanes set: the bank-wide fast path's requirement.
const fn all_lanes<const W: usize>() -> u32 {
    (1 << W) - 1
}

/// Replaces one lane of a packed word, leaving the others bit-exact.
fn replace_lane<L: Lane>(value: L, lane: usize, replacement: f32) -> L {
    let mut words = [0.0_f32; 8];
    value.store(&mut words);
    words[lane] = replacement;
    L::load(&words)
}

/// The scalar (`W = 1`) product.
struct PreparedTransientShaper(Shaper<f32, 1>);

/// A bank of `W` tracks rendered as one vector.
///
/// Its `PreparedBankMetadata` is returned beside it (issue #1461); it keeps only the two bank
/// values `process_bank` reads.
struct PreparedTransientShaperBank<L: Lane, const W: usize> {
    shaper: Shaper<L, W>,
    /// The bound bank width, read by `process_bank`'s block check and report.
    width: BankWidth,
    /// The prepared quantum, read by `process_bank`'s block check.
    quantum: u32,
}

/// The scalar instance [`TransientShaperFactory::prepare`] boxes, and its metadata.
fn prepare_scalar(
    request: PrepareEffectRequest<'_>,
) -> Result<(PreparedTransientShaper, PreparedEffectMetadata), EffectPrepareError> {
    let metadata = expected_prepared_metadata(&TRANSIENT_SHAPER_DESCRIPTOR, request)?;
    let row = coefficient_row(metadata.sample_rate).ok_or(EffectPrepareError {
        code: "effect.quality.unsupported",
    })?;
    let (left, right) = initial_defaults(request.initial_values)?;
    Ok((
        PreparedTransientShaper(Shaper::new(&metadata, row, [left], [right])),
        metadata,
    ))
}

impl NativeEffectFactory for TransientShaperFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &TRANSIENT_SHAPER_DESCRIPTOR
    }

    fn prepared_inline_bytes(&self) -> u64 {
        size_of::<PreparedTransientShaper>() as u64
    }

    fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
        0
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<PreparedEffect, EffectPrepareError> {
        let (processor, metadata) = prepare_scalar(request)?;
        Ok(PreparedEffect {
            processor: Box::new(processor),
            metadata,
        })
    }

    /// Issue #2199: the scalar instance `prepare` boxes, built inline and asked. It holds no heap,
    /// so the answer allocates nothing.
    fn designed_channel_symmetry(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<bool, EffectPrepareError> {
        prepare_scalar(request).map(|(processor, _)| processor.channel_symmetry())
    }

    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
        bind_bank::<true>(self, request)
    }
}

/// `bind_homogeneous_bank`, with the D4 width check as a switch: `NATIVE_ONLY` declines a width
/// this artifact does not execute, which is what production asks for. The unit tests pass `false`
/// to bind the other width's bank on this host through the same code.
fn bind_bank<const NATIVE_ONLY: bool>(
    factory: &TransientShaperFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
    request.validate_shape()?;
    Ok(effect_contract::match_bank_width!(request.width, |L, W| {
        boxed::<L, W, NATIVE_ONLY>(bind::<_, W, NATIVE_ONLY>(factory, request)?)
    }))
}

/// Boxes a bank bound at `W` lanes.
///
/// The D4 check of [`bind`] is repeated here, where it is a compile-time constant ahead of the
/// only reference to the bank's vtable, so a production build never links the render code of a
/// width it does not execute. `bind` cannot promise that alone: it returns the bank by value, and
/// the eight-lane shaper would otherwise stay in the four-lane browser artifact.
fn boxed<L: Lane, const W: usize, const NATIVE_ONLY: bool>(
    bank: Option<(PreparedTransientShaperBank<L, W>, PreparedBankMetadata)>,
) -> Option<PreparedEffectBank> {
    if NATIVE_ONLY && W != Backend::current().width() {
        return None;
    }
    bank.map(|(bank, metadata)| PreparedEffectBank {
        processor: Box::new(bank) as Box<dyn PreparedNativeEffectBank>,
        metadata,
    })
}

/// Binds one bank of `W` lanes, returned beside its `PreparedBankMetadata`.
///
/// # Padding (issue #1092; decision 12)
///
/// The shaper accepts a padded request. Each lane, active or padded, is prepared from its own
/// request, which for a padded lane is the caller's clone of an active member's, so every lane is
/// validated. A padded lane takes no automation, and D7 recovers it without charging it
/// ([`Shaper::recover_lanes`]). Fed `+0.0` it writes `+0.0`: both envelopes floor, the contrast
/// is exactly zero and the shape-zero identity selects the input word. The shaper never reads
/// across lanes, and its one whole-bank decision, the ramp prefix, is partition-invariant, so an
/// active lane's bits depend neither on the padded lanes nor on which member they clone.
fn bind<L: Lane, const W: usize, const NATIVE_ONLY: bool>(
    factory: &TransientShaperFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<(PreparedTransientShaperBank<L, W>, PreparedBankMetadata)>, EffectPrepareError> {
    let first = request
        .requests
        .first()
        .copied()
        .ok_or(EffectPrepareError {
            code: "effect.bank.requests",
        })?;
    let metadata = expected_prepared_metadata(factory.descriptor(), first)?;
    let row = coefficient_row(metadata.sample_rate).ok_or(EffectPrepareError {
        code: "effect.quality.unsupported",
    })?;
    let (first_left, first_right) = initial_defaults(first.initial_values)?;
    let mut left = [first_left; W];
    let mut right = [first_right; W];
    // Issue #1368: the key carries no link mode (`lane_link`), so each lane takes its own
    // member's mode, and a padded lane takes the first member's (D5), so padding never forces the
    // mixed arm.
    let mut links = [metadata.link_mode; W];
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate().skip(1) {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        if request.active_mask[track] {
            links[track] = candidate.link_mode;
        }
        let (item_left, item_right) = initial_defaults(item.initial_values)?;
        left[track] = item_left;
        right[track] = item_right;
    }
    // There is no runtime SIMD dispatch (D4): this build has exactly one production width, and a
    // plan asking for another one is refused as unavailable rather than quietly served by it.
    // `has_matching_backend_width` has already tied the request's backend to its width `W`, a
    // compile-time constant here, so a production build never links the other width's bank.
    if !same_program || (NATIVE_ONLY && W != Backend::current().width()) {
        return Ok(None);
    }
    let mut shaper = Shaper::new(&metadata, row, left, right);
    shaper.links = links;
    shaper.link_lanes = LinkLanes::new(&links);
    let mut shaper = shaper.with_links_at_rest();
    shaper.active = request
        .active_mask
        .iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0, |bits, (lane, _)| bits | (1 << lane));
    Ok(Some((
        PreparedTransientShaperBank::<L, W> {
            shaper,
            width: request.width,
            quantum: metadata.quantum,
        },
        PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
    )))
}

impl PreparedNativeEffect for PreparedTransientShaper {
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    // REALTIME_POLICY_BEGIN
    fn reset(&mut self, kind: ResetKind) {
        self.0.reset(kind);
    }
    // REALTIME_POLICY_END

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.0.automation_capacity,
            block.first_sample,
            0,
            &mut self.0.left,
            &mut self.0.right,
            &mut report,
        );
        let frames = block.frames();
        let [left, right] = self
            .0
            .process_block::<false>(block.left, block.right, frames);
        report.nonfinite_left_blocks = u64::from(left & 1);
        report.nonfinite_right_blocks = u64::from(right & 1);
        report
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.0.snapshot(0, output)
    }

    fn restore_state_payload(
        &mut self,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.0.restore(0, state_layout_version, input)
    }
    // REALTIME_POLICY_END

    /// Issue #1873: [`Shaper::designed_channel_symmetry`] on lane 0.
    fn channel_symmetry(&self) -> bool {
        self.0.designed_channel_symmetry(0)
    }

    /// Issue #1370: [`Shaper::retarget_link`] on the one lane.
    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        track == 0 && self.0.retarget_link(0, mode, samples)
    }
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedTransientShaperBank<L, W> {
    // REALTIME_POLICY_BEGIN
    fn reset_lane(&mut self, lane: usize, kind: ResetKind) {
        self.shaper.reset_lane(lane, kind);
    }
    // REALTIME_POLICY_END
    /// Every field is inline: this instance owns no heap (#1074).
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn reset(&mut self, kind: ResetKind) {
        self.shaper.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<false>(block)
    }

    /// Issue #1873: [`Shaper::designed_channel_symmetry`].
    fn lane_channel_symmetry(&self, lane: usize) -> bool {
        self.shaper.designed_channel_symmetry(lane)
    }

    fn supports_mono_collapse(&self) -> bool {
        true
    }

    fn process_bank_mono(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        self.process_bank_inner::<true>(block)
    }

    fn desymmetrize_channels(&mut self) {
        self.shaper.desymmetrize();
    }

    fn channels_agree(&self) -> bool {
        self.shaper.channels_bit_equal()
    }

    fn lanes_at_rest(&self) -> u32 {
        self.shaper.lanes_at_rest()
    }

    // REALTIME_POLICY_BEGIN: #1278 D3, the payload calls run in the plan-swap block.
    fn snapshot_track_state_payload(
        &self,
        track_index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.shaper.snapshot(checked_track(track_index, W)?, output)
    }

    fn restore_track_state_payload(
        &mut self,
        track_index: u32,
        state_layout_version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        let track = checked_track(track_index, W)?;
        self.shaper.restore(track, state_layout_version, input)
    }
    // REALTIME_POLICY_END

    fn lane_default_bytes(&self) -> Option<usize> {
        Some(LANE_DEFAULT_BYTES)
    }

    fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        self.exchange_defaults(lane, bytes)
    }

    /// Issue #1370: [`Shaper::retarget_link`]; a padded lane takes no glide.
    fn retarget_link(&mut self, track: usize, mode: LinkMode, samples: u32) -> bool {
        self.shaper.retarget_link(track, mode, samples)
    }
}

impl<L: Lane, const W: usize> PreparedTransientShaperBank<L, W> {
    /// The one bank body, dual or collapsed (issue #1873).
    ///
    /// `MONO` chooses the render body and nothing else: the guards, the automation drain (both
    /// channels' spans, as a dual block takes them) and the report are the same statements in the
    /// same order. A collapsed block hands the kernel no right plane, so the right plane is never
    /// read or written. Inlined into `process_bank` and `process_bank_mono`, each with the kernel
    /// inlined.
    #[inline(always)]
    fn process_bank_inner<const MONO: bool>(
        &mut self,
        block: EffectBankProcessBlock<'_>,
    ) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.width);
        if block.width != self.width || block.frames > self.quantum || block.sidechain.is_some() {
            return report;
        }
        for track in 0..W {
            // A padded lane carries no track, so no span is its to apply (issue #1092).
            if self.shaper.active & (1 << track) == 0 {
                continue;
            }
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.shaper.automation_capacity,
                block.first_sample,
                track,
                &mut self.shaper.left,
                &mut self.shaper.right,
                &mut report.reports[track],
            );
        }
        let right = if MONO { &mut [][..] } else { block.right };
        let [left, right] =
            self.shaper
                .process_block::<MONO>(block.left, right, block.frames as usize);
        for (track, entry) in report.reports.iter_mut().enumerate().take(W) {
            entry.nonfinite_left_blocks = u64::from((left >> track) & 1);
            entry.nonfinite_right_blocks = u64::from((right >> track) & 1);
        }
        report
    }
}

/// One lane's prepared defaults in bytes: every parameter of both channels, one `f32` each, then
/// the lane's link mode (issue #1368).
const LANE_DEFAULT_BYTES: usize = 2 * PARAMETER_COUNT * 4 + 4;

// REALTIME_POLICY_BEGIN: #1747, the prepared defaults move with a render-time lane swap.
impl<L: Lane, const W: usize> PreparedTransientShaperBank<L, W> {
    /// [`PreparedNativeEffectBank::exchange_lane_defaults`]: lane `lane`'s left
    /// `Ramps::defaults` row then its right one, each parameter in descriptor order as
    /// little-endian `f32` bits, then the lane's link mode as one word (issue #1368,
    /// `effect_contract::exchange_lane_link_word`). They are the shaper's only prepared per-lane
    /// values: a full reset rebuilds the lane's ramps from the rows, and the link masks and the
    /// arm are re-derived from the modes. The D7 recovery clears envelopes and reads no default.
    fn exchange_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
        if lane >= W || bytes.len() != LANE_DEFAULT_BYTES {
            return false;
        }
        let shaper = &mut self.shaper;
        let (values_bytes, link_bytes) = bytes.split_at_mut(2 * PARAMETER_COUNT * 4);
        let link = effect_contract::exchange_lane_link_word(shaper.links[lane], link_bytes);
        shaper.set_lane_link(lane, link);
        // The weights rest at the incoming mode; the swap's payload restore then brings a glide
        // in flight with it (issue #1370 D6).
        shaper.rest_links(lane);
        let words = shaper.left.defaults[lane]
            .iter_mut()
            .chain(shaper.right.defaults[lane].iter_mut());
        for (value, chunk) in words.zip(values_bytes.as_chunks_mut::<4>().0.iter_mut()) {
            let mut incoming = [0_u8; 4];
            incoming.copy_from_slice(chunk);
            chunk.copy_from_slice(&value.to_bits().to_le_bytes());
            *value = f32::from_bits(u32::from_le_bytes(incoming));
        }
        true
    }
}
// REALTIME_POLICY_END

#[cfg(test)]
mod glide_tests;
#[cfg(test)]
mod padding_tests;
#[cfg(test)]
mod rest_tests;
