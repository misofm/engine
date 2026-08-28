//! Fixed causal dual-envelope transient shaper.
//!
//! This crate is the parameter table, the frozen follower coefficients and one generic block
//! kernel. Banking, ramps, the state-payload codec, parameter domains and the once-per-block
//! output boundary check are `miso-engine-effect-runtime`; the lane vocabulary and the dry/wet mix
//! law are `miso-engine-lane`; the per-sample dB conversions are `miso-engine-math` (master plan
//! for issue #83, §6 and D6).
//!
//! # The law
//!
//! Two switched attack/release one-pole followers — fast at 0.5 ms / 20 ms and slow at
//! 10 ms / 100 ms — track the linked detector magnitude. Their ratio is the *contrast*
//! `c = DB_PER_OCTAVE * log2(max(fast, FLOOR) / max(slow, FLOOR))` in dB, clamped to ±24 dB; the
//! shape is `A * max(c, 0) + S * max(-c, 0)` clamped to ±18 dB; the gain is
//! `exp2(OCTAVES_PER_DB * shape)` — which is `10^(shape / 20)` — and the output is the dry/wet mix
//! of `miso_engine_lane::kernels::gain_mix_step`.
//!
//! One `log2` of the ratio replaces the `20 log10(fast) - 20 log10(slow)` of the pre-audit crate:
//! the two are algebraically identical, and the ratio form both halves the transcendental error
//! and costs one polynomial instead of two.
//!
//! # Determinism
//!
//! Every render-path operation is an IEEE basic operation or `Lane::fma`, so the rendered block is
//! bit-identical across `Scalar`/`Simd4`/`Simd8`, across `x86_64`/`aarch64`/`wasm32` and across
//! block partitions (D5). The pre-audit crate called `f32::log10` and `f32::powf` three times per
//! lane-sample, which made its output the platform libm's; those bits were never portable.
//!
//! # State
//!
//! Eleven words per lane per channel — `fast`, `slow`, then `(current, target, remaining)` for each
//! of the three parameters — unchanged, at `state_layout_version` 1. The D11 ramp's precomputed
//! `step` is *derived* on restore (`(target - current) / remaining`) rather than persisted, which
//! is what keeps the layout a contract fixture.

use miso_engine_effect_contract::{
    AutomationRate, AutomationSpanKind, BankProcessReport, BankWidth, EffectBankProcessBlock,
    EffectDescriptor, EffectPrepareError, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LatencySamples, LinkMode, LinkModeSet, NativeEffectFactory, ParameterChannel,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterId, ParameterMapping,
    ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedBankMetadata, PreparedEffectMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, SmoothingRule,
    StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
    expected_prepared_metadata,
};
use miso_engine_effect_runtime::bank::{NonFiniteReport, finish_block};
use miso_engine_effect_runtime::envelope::{ArCoef, ar_one_pole_step};
use miso_engine_effect_runtime::params::{
    ParameterSpec, normalize_zero, parameter_value_valid as spec_value_valid,
};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};
use miso_engine_lane::kernels::gain_mix_step;
use miso_engine_lane::{Backend, Lane, Simd4, Simd8};
use miso_engine_math::{exp2_lane, log2_lane};

pub mod corpus;

const PARAMETER_COUNT: usize = 3;
const RAMP_SAMPLES: u32 = 64;
const STATE_WORDS: usize = 11;
const LANE_STATE_BYTES: u32 = (STATE_WORDS * 4) as u32;

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
        None => panic!("nonzero static parameter identifier"),
    }
}

const fn parameter(
    id: u32,
    display_name: &'static str,
    display_unit: &'static str,
    minimum: f32,
    maximum: f32,
    default_value: f32,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: parameter_id(id),
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
    }
}

/// Frozen V1 parameter rows in stable numeric-ID order.
pub const TRANSIENT_SHAPER_PARAMETERS: [ParameterDescriptor; PARAMETER_COUNT] = [
    parameter(1, "attack amount", "%", -1.0, 1.0, 0.0),
    parameter(2, "sustain amount", "%", -1.0, 1.0, 0.0),
    parameter(3, "mix", "linear", 0.0, 1.0, 1.0),
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

const fn quality(sample_rate: u32) -> miso_engine_effect_contract::QualityDescriptor {
    miso_engine_effect_contract::QualityDescriptor {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
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

const QUALITIES: [miso_engine_effect_contract::QualityDescriptor; 4] = [
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
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &TRANSIENT_SHAPER_PARAMETERS,
    ports: &PORTS,
    qualities: &QUALITIES,
    observations: &[],
};

/// Fast-attack, fast-release, slow-attack, slow-release coefficient bits by launch-rate row.
///
/// Each is `exp(-1 / (tau * fs))` — the *retention* coefficient of
/// `miso_engine_effect_runtime::envelope::retention_coefficient` — rounded once to `f32`, for the
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

/// `20 * log10(2)`: decibels per octave, the scale from `log2` to the dB contrast.
const DB_PER_OCTAVE: f32 = f32::from_bits(0x40c0_a8c1);

/// `log2(10) / 20`: octaves per decibel, the scale from the dB shape to the `exp2` argument.
const OCTAVES_PER_DB: f32 = f32::from_bits(0x3e2a_152d);

/// Contrast is clamped to ±24 dB before the shape law.
const CONTRAST_LIMIT_DB: f32 = 24.0;

/// Shape is clamped to ±18 dB before the gain conversion.
const SHAPE_LIMIT_DB: f32 = 18.0;

const LINK_DUAL_MONO: u8 = 0;
const LINK_MAXIMUM: u8 = 1;
const LINK_AVERAGE: u8 = 2;

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
/// `Average` is `0.5 * l + 0.5 * r` — two products and a sum, never `0.5 * (l + r)`, which has one
/// fewer rounding and different bits.
#[inline(always)]
fn link<L: Lane, const LINK: u8>(left: L, right: L) -> (L, L) {
    let left = left.abs();
    let right = right.abs();
    match LINK {
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

/// One frame of one channel. Frozen operation order.
///
/// 1. both followers advance (`ar_one_pole_step`: two products, one sum, one D7 flush each)
/// 2. `ratio = max(fast, FLOOR) / max(slow, FLOOR)` — one IEEE division
/// 3. `contrast = log2_lane(ratio) * DB_PER_OCTAVE`, clamped `min` then `max` to ±24 dB
/// 4. `shape = attack * max(contrast, 0) + sustain * max(-contrast, 0)`, clamped to ±18 dB
/// 5. `gain = exp2_lane(shape * OCTAVES_PER_DB)` — `exp2_lane(0)` is exactly `1`
/// 6. `wet = gain_mix_step(x, gain, mix)` = `fma(mix, x * gain - x, x)`
/// 7. `select(bypass or mix == 0 or shape == 0, x, wet)` — the signed-zero identity contract:
///    `fma(mix, +0.0, -0.0)` is `+0.0`, so the dry value has to be selected, not computed
///
/// There is no per-value finiteness or subnormal classification (D7): the two envelope words are
/// flushed inside the follower and the output block is checked once, by the caller.
#[inline(always)]
fn frame<L: Lane>(x: L, u: L, c: &Coef<L>, e: &mut Env<L>, p: &Params<L>, bypass: L::Mask) -> L {
    e.fast = ar_one_pole_step(e.fast, u, &c.fast);
    e.slow = ar_one_pole_step(e.slow, u, &c.slow);
    let floor = L::splat(FLOOR);
    let ratio = e.fast.max(floor).div(e.slow.max(floor));
    let contrast = log2_lane(ratio).mul(L::splat(DB_PER_OCTAVE));
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
    let gain = exp2_lane(shape.mul(L::splat(OCTAVES_PER_DB)));
    let wet = gain_mix_step(x, gain, p.mix);
    let identity = L::mask_or(bypass, L::mask_or(p.mix.eq(zero), shape.eq(zero)));
    L::select(identity, x, wet)
}

/// One frame of both channels: link, then [`frame`] per channel.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn step<L: Lane, const LINK: u8>(
    fl: &mut [f32],
    fr: &mut [f32],
    c: &Coef<L>,
    el: &mut Env<L>,
    er: &mut Env<L>,
    pl: &Params<L>,
    pr: &Params<L>,
    bypass: L::Mask,
) {
    let (xl, xr) = (L::load(fl), L::load(fr));
    let (ul, ur) = link::<L, LINK>(xl, xr);
    frame(xl, ul, c, el, pl, bypass).store(fl);
    frame(xr, ur, c, er, pr, bypass).store(fr);
}

/// The parameter ramps of one channel of `W` lanes, plus the defaults a full reset restores.
#[derive(Clone, Copy, Debug)]
struct Ramps<const W: usize> {
    ramps: [[LinearRamp; PARAMETER_COUNT]; W],
    defaults: [[f32; PARAMETER_COUNT]; W],
}

impl<const W: usize> Ramps<W> {
    fn new(defaults: [[f32; PARAMETER_COUNT]; W]) -> Self {
        Self {
            ramps: defaults.map(|lane| lane.map(LinearRamp::fixed)),
            defaults,
        }
    }

    /// Frames still to be ramped by any lane or parameter: zero in steady state (finding F6).
    fn prefix(&self) -> usize {
        self.ramps
            .iter()
            .flatten()
            .map(|ramp| ramp.remaining as usize)
            .max()
            .unwrap_or(0)
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

    fn full_reset(&mut self) {
        *self = Self::new(self.defaults);
    }

    fn snap(&mut self) {
        for ramp in self.ramps.iter_mut().flatten() {
            ramp.snap();
        }
    }
}

/// One prepared transient shaper over `W` lanes: `W = 1` is the scalar product, `W = 4`/`8` a bank.
struct Shaper<L: Lane, const W: usize> {
    coefficients: Coef<L>,
    left_env: Env<L>,
    right_env: Env<L>,
    left: Ramps<W>,
    right: Ramps<W>,
    metadata: PreparedEffectMetadata,
    nonfinite: NonFiniteReport,
}

impl<L: Lane, const W: usize> Shaper<L, W> {
    fn new(
        metadata: PreparedEffectMetadata,
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
            metadata,
            nonfinite: NonFiniteReport::new(),
        }
    }

    fn reset(&mut self, kind: ResetKind) {
        self.left_env = Env::default();
        self.right_env = Env::default();
        match kind {
            ResetKind::FullToDefaults => {
                self.left.full_reset();
                self.right.full_reset();
            }
            ResetKind::DiscontinuityKeepParameters => {
                self.left.snap();
                self.right.snap();
            }
        }
    }

    /// Renders one block in place and applies the master plan §4.4 boundary check.
    fn process_block(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) {
        debug_assert_eq!(W, L::WIDTH);
        match self.metadata.link_mode {
            LinkMode::DualMono => self.run::<LINK_DUAL_MONO>(left, right, frames),
            LinkMode::Maximum => self.run::<LINK_MAXIMUM>(left, right, frames),
            LinkMode::Average => self.run::<LINK_AVERAGE>(left, right, frames),
        }
        let (left_env, right_env) = (&mut self.left_env, &mut self.right_env);
        let report = &mut self.nonfinite;
        finish_block::<L>(left, right, report, || {
            *left_env = Env::default();
            *right_env = Env::default();
        });
    }

    #[inline(always)]
    fn run<const LINK: u8>(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) {
        let bypass = L::splat(if self.metadata.bypass { 1.0 } else { 0.0 }).gt(L::zero());
        let prefix = frames.min(self.left.prefix().max(self.right.prefix()));
        let split = prefix * L::WIDTH;
        let (head_left, tail_left) = left.split_at_mut(split);
        let (head_right, tail_right) = right.split_at_mut(split);
        // The envelopes live in locals across the whole block, not behind a `&mut` per frame.
        let (mut el, mut er) = (self.left_env, self.right_env);
        for (fl, fr) in head_left
            .chunks_exact_mut(L::WIDTH)
            .zip(head_right.chunks_exact_mut(L::WIDTH))
        {
            let pl = self.left.advance::<L>();
            let pr = self.right.advance::<L>();
            step::<L, LINK>(
                fl,
                fr,
                &self.coefficients,
                &mut el,
                &mut er,
                &pl,
                &pr,
                bypass,
            );
        }
        let pl = self.left.current::<L>();
        let pr = self.right.current::<L>();
        for (fl, fr) in tail_left
            .chunks_exact_mut(L::WIDTH)
            .zip(tail_right.chunks_exact_mut(L::WIDTH))
        {
            step::<L, LINK>(
                fl,
                fr,
                &self.coefficients,
                &mut el,
                &mut er,
                &pl,
                &pr,
                bypass,
            );
        }
        self.left_env = el;
        self.right_env = er;
    }
    fn snapshot(
        &self,
        lane: usize,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        validate_state_lengths(
            output.common.len(),
            output.left.len(),
            output.right.len(),
            self.metadata.state_sizes,
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
            self.metadata.state_sizes,
        )?;
        let left = read_lane(input.left)?;
        let right = read_lane(input.right)?;
        self.left_env.fast = replace_lane(self.left_env.fast, lane, left.fast);
        self.left_env.slow = replace_lane(self.left_env.slow, lane, left.slow);
        self.right_env.fast = replace_lane(self.right_env.fast, lane, right.fast);
        self.right_env.slow = replace_lane(self.right_env.slow, lane, right.slow);
        self.left.ramps[lane] = left.ramps;
        self.right.ramps[lane] = right.ramps;
        Ok(())
    }
}

/// Reads one lane's eleven state words out of a payload section.
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
        let word = 2 + index * 3;
        write_f32(bytes, word, ramp.current);
        write_f32(bytes, word + 1, ramp.target);
        write_u32(bytes, word + 2, ramp.remaining);
    }
}

/// The eleven words of one restored lane: the two envelope words and the three ramps.
struct LaneWords {
    fast: f32,
    slow: f32,
    ramps: [LinearRamp; PARAMETER_COUNT],
}

/// Validates and decodes one lane's eleven state words.
///
/// The D11 `step` is **derived**, not persisted: `(target - current) / remaining`, which reproduces
/// the pre-audit per-sample division exactly for the pinned continuation row and resumes within one
/// ulp of the original increment in general. That is what keeps the layout eleven words.
fn read_lane(bytes: &[u8]) -> Result<LaneWords, StatePayloadError> {
    let fast = read_f32(bytes, 0);
    let slow = read_f32(bytes, 1);
    if !valid_envelope(fast) || !valid_envelope(slow) {
        return Err(state_error("effect.state.envelope"));
    }
    let mut ramps = [LinearRamp::fixed(0.0); PARAMETER_COUNT];
    for (index, ramp) in ramps.iter_mut().enumerate() {
        let word = 2 + index * 3;
        let current = read_f32(bytes, word);
        let target = read_f32(bytes, word + 1);
        let remaining = read_u32(bytes, word + 2);
        let parameter = &TRANSIENT_SHAPER_PARAMETERS[index];
        if !value_valid(parameter, current)
            || !value_valid(parameter, target)
            || remaining > RAMP_SAMPLES
        {
            return Err(state_error("effect.state.parameter"));
        }
        let current = normalize_zero(current);
        let target = normalize_zero(target);
        *ramp = LinearRamp {
            current,
            target,
            step: if remaining == 0 {
                0.0
            } else {
                (target - current) / remaining as f32
            },
            remaining,
        };
    }
    Ok(LaneWords { fast, slow, ramps })
}

fn valid_envelope(value: f32) -> bool {
    (value.is_normal() && value.is_sign_positive()) || value.to_bits() == 0.0_f32.to_bits()
}

const fn state_error(code: &'static str) -> StatePayloadError {
    StatePayloadError { code }
}

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
    metadata: PreparedEffectMetadata,
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
        let valid = span_index < metadata.automation_capacity as usize
            && parameter_index < PARAMETER_COUNT
            && span.kind == AutomationSpanKind::Point
            && span.start_sample == first_sample
            && span.end_sample == first_sample
            && span.start_value.to_bits() == span.end_value.to_bits()
            && value_valid(
                &TRANSIENT_SHAPER_PARAMETERS[parameter_index],
                span.start_value,
            )
            && last_order.is_none_or(|previous| order > previous)
            && pending[lane_index][parameter_index].is_none();
        if !valid {
            report.invalid_spans = report.invalid_spans.saturating_add(1);
            continue;
        }
        last_order = Some(order);
        pending[lane_index][parameter_index] = Some(normalize_zero(span.start_value));
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
struct PreparedTransientShaperBank<L: Lane, const W: usize> {
    shaper: Shaper<L, W>,
    metadata: PreparedBankMetadata,
}

impl NativeEffectFactory for TransientShaperFactory {
    fn descriptor(&self) -> &'static EffectDescriptor {
        &TRANSIENT_SHAPER_DESCRIPTOR
    }

    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        let metadata = expected_prepared_metadata(self.descriptor(), request)?;
        let row = coefficient_row(metadata.sample_rate).ok_or(EffectPrepareError {
            code: "effect.quality.unsupported",
        })?;
        let (left, right) = initial_defaults(request.initial_values)?;
        Ok(Box::new(PreparedTransientShaper(Shaper::new(
            metadata,
            row,
            [left],
            [right],
        ))))
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
        match request.width {
            BankWidth::Four => bind::<Simd4, 4>(self, request),
            BankWidth::Eight => bind::<Simd8, 8>(self, request),
        }
    }
}

fn bind<L: Lane, const W: usize>(
    factory: &TransientShaperFactory,
    request: PrepareEffectBankRequest<'_>,
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
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
    let mut same_program = true;
    for (track, item) in request.requests.iter().copied().enumerate() {
        let candidate = expected_prepared_metadata(factory.descriptor(), item)?;
        if candidate.program_key() != metadata.program_key() {
            same_program = false;
        }
        let (item_left, item_right) = initial_defaults(item.initial_values)?;
        left[track] = item_left;
        right[track] = item_right;
    }
    // There is no runtime SIMD dispatch (D4): this build has exactly one production width, and a
    // plan asking for another one is refused as unavailable rather than quietly served by it.
    // `has_matching_backend_width` has already tied the request's backend to its width.
    if !same_program || request.width.lanes() as usize != Backend::current().width() {
        return Ok(None);
    }
    Ok(Some(Box::new(PreparedTransientShaperBank::<L, W> {
        metadata: PreparedBankMetadata {
            width: request.width,
            program_key: metadata.program_key(),
        },
        shaper: Shaper::new(metadata, row, left, right),
    })))
}

impl PreparedNativeEffect for PreparedTransientShaper {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.0.metadata
    }

    fn reset(&mut self, kind: ResetKind) {
        self.0.reset(kind);
    }

    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        let mut report = ProcessReport::default();
        apply_automation(
            block.automation,
            self.0.metadata,
            block.first_sample,
            0,
            &mut self.0.left,
            &mut self.0.right,
            &mut report,
        );
        let frames = block.frames();
        self.0.process_block(block.left, block.right, frames);
        report
    }

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
}

impl<L: Lane, const W: usize> PreparedNativeEffectBank for PreparedTransientShaperBank<L, W> {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }

    fn reset(&mut self, kind: ResetKind) {
        self.shaper.reset(kind);
    }

    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        let mut report = BankProcessReport::empty(self.metadata.width);
        if block.width != self.metadata.width
            || block.frames > self.shaper.metadata.quantum
            || block.sidechain.is_some()
        {
            return report;
        }
        for track in 0..W {
            let start = block.automation_offsets[track] as usize;
            let end = block.automation_offsets[track + 1] as usize;
            apply_automation(
                &block.automation[start..end],
                self.shaper.metadata,
                block.first_sample,
                track,
                &mut self.shaper.left,
                &mut self.shaper.right,
                &mut report.reports[track],
            );
        }
        self.shaper
            .process_block(block.left, block.right, block.frames as usize);
        report
    }

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
}
