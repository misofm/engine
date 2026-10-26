//! The certified tail and exact-rest bound of `miso.reverb` (#2114 S3, decision 15 D15-4(b)).
//!
//! The derivation is `docs/derivations/2114-reverb-tail-and-rest.md`; its names are used here.
//! Everything runs on the control thread, once per launch rate when the registry builds its
//! table: a bisection of about 60 steps, 64 candidate diffuser rates and two short integer walks.
//!
//! * **Before the settle point `t0 = N + 256`** no linear bound is used. The once-per-block
//!   finiteness check is the invariant: every signal word a later sample can read was written in a
//!   block whose words are all below `BLOCK_LIMIT = 1e30`, or the history was dropped. That holds
//!   under every admitted control history and every input.
//! * **From `t0`** every coefficient and tap is fixed. A weighted `l2` storage `Phi` (ring words
//!   weighted by `r^(-2 (D_i - k))`, filter states by their scaled TPT storages) contracts by
//!   `r = 1 - delta` per sample up to counted rounding; `delta` is certified by bisection. The
//!   diffusers and the draining pre-delay feed it through a geometric `l-inf` majorant.
//! * **Rest** follows once the ring writes fall below `FLUSH_EPS`; the filter states then walk
//!   to `+0.0` (`effect_runtime::tail::rate_follower_rest_samples`).
//!
//! The bound does not depend on the input peak, so no composition value is stated
//! (`CompositionBound::Unstated`, the derivation's OQ1).

use effect_contract::{CompositionBound, NodeTailBound, RestBound, RestSamples, TailSamples};
use effect_runtime::tail::{RestSnap, rate_follower_rest_samples};

use crate::{
    BASE, DAMPING, DIFFUSION, LOW_CUT, RAMP_SAMPLES, REVERB_PARAMETERS, RING_SLACK, SIZE,
    TRANSITION_SAMPLES, const_nearest_prime, damping_coefficient, diffusion_coefficient, geometry,
    low_cut_coefficient,
};

/// The `f32` unit roundoff `u = 2^-24`.
const U: f64 = effect_runtime::tail::ROUNDOFF;
/// `2^-126`, one operation's absolute underflow (gradual or flush-to-zero).
const UNDERFLOW: f64 = math::tail::UNDERFLOW;
/// The relative allowance by which every `f64` evaluation is rounded toward its bound (`2^-24`),
/// far above the few `2^-53` units a short expression loses. No rounded expression cancels.
const EVAL: f64 = 1.0 / 16_777_216.0;
/// The block check's magnitude limit, `effect_runtime::bank::BLOCK_LIMIT`.
const LIMIT: f64 = effect_runtime::bank::BLOCK_LIMIT as f64;
/// `lane::FLUSH_EPS`.
const FLUSH: f64 = lane::FLUSH_EPS as f64;

/// Samples after the last control event by which every ramp has landed and every crossfade (one
/// in flight and one queued) has ended.
pub(crate) const SETTLE_SAMPLES: u64 = if RAMP_SAMPLES > 2 * TRANSITION_SAMPLES {
    RAMP_SAMPLES as u64
} else {
    2 * TRANSITION_SAMPLES as u64
};

/// `x (1 + EVAL)`: an upper bound of the exact value that a positive `x` approximates.
fn up(x: f64) -> f64 {
    x * (1.0 + EVAL)
}

/// `x (1 - EVAL)`: a lower bound of the exact value that a positive `x` approximates.
fn down(x: f64) -> f64 {
    x * (1.0 - EVAL)
}

/// `1 + k u`.
fn grow(k: f64) -> f64 {
    1.0 + k * U
}

/// `(1 + u)^n`, multiplied out.
fn grow_pow(n: u32) -> f64 {
    (0..n).fold(1.0, |product, _| product * grow(1.0))
}

/// `ceil(x)` of a non-negative finite `x`; `None` past `u64`.
fn ceil_u64(x: f64) -> Option<u64> {
    let ceiling = -math::floor(-x);
    (ceiling.is_finite() && (0.0..1.8e19).contains(&ceiling)).then_some(ceiling as u64)
}

/// The smallest `n >= 0` with `x e^(-n nu) < y` (`x, y, nu > 0`): `0` when `x < y`, otherwise
/// `ceil(ln(x / y) / nu)` with the logarithm rounded up.
fn decay_samples(x: f64, y: f64, nu: f64) -> Option<u64> {
    if !(x.is_finite() && y > 0.0 && nu > 0.0) {
        return None;
    }
    if x < y {
        return Some(0);
    }
    ceil_u64(up(up(math::log(up(x / y))) / nu))
}

/// An upper bound of `-ln(1 - delta)` for `0 <= delta <= 1/2`: `delta + delta^2 / (1 - delta)`.
fn nu_plus(delta: f64) -> f64 {
    up(delta + delta * delta / down(1.0 - delta))
}

/// An upper bound of `e^x` for any finite `x`.
fn exp_up(x: f64) -> f64 {
    up(math::exp(x + x.abs() * EVAL))
}

/// The largest `f32` at or below a positive finite `x`.
fn f32_at_or_below(x: f64) -> f32 {
    let value = x as f32;
    if f64::from(value) > x {
        value.next_down()
    } else {
        value
    }
}

/// The smallest `f32` at or above a positive finite `x`.
fn f32_at_or_above(x: f64) -> f32 {
    let value = x as f32;
    if f64::from(value) < x {
        value.next_up()
    } else {
        value
    }
}

/// One TPT section's rounding, as the derivation's "Rounding of a section": the computed step is
/// `r`-dissipative with `Gamma_c = (1 + lambda) Gamma(delta + lambda)` and an additive `abs U`
/// inside the storage norm.
#[derive(Clone, Copy, Debug)]
struct SectionRounding {
    lambda: f64,
    abs: f64,
}

/// [`SectionRounding`] for every coefficient up to `g_hi` and every `delta <= delta_cap`, with the
/// storage weight in `[theta_min, theta_max]`. `high_pass` selects the output `f = fl(x - y)`
/// (R6) instead of `y` (R5). Every coefficient below is increasing in `g`, so `g_hi` bounds them.
fn section_rounding(
    g: f64,
    theta_min: f64,
    theta_max: f64,
    high_pass: bool,
    delta_cap: f64,
) -> SectionRounding {
    let u = U;
    let k2 = 2.0 * u + u * u;
    // |y - y*| <= alpha_y |s| + beta_y |x| + nu_y U.
    let alpha_y = (1.0 + u) * g * k2 + u * (1.0 - g);
    let beta_y = (1.0 + u) * g * k2 + u * g;
    let nu_y = (1.0 + u) * (1.0 + g * (1.0 + u)) + 1.0;
    // |s' - s'*| before the flush.
    let alpha_s = (1.0 + u) * (alpha_y + g * k2) + u * (1.0 - 2.0 * g);
    let beta_s = (1.0 + u) * (beta_y + g * k2) + 2.0 * u * g;
    let nu_s = (1.0 + u) * (nu_y + 1.0 + g * (1.0 + u)) + 1.0;
    let (alpha_o, beta_o, nu_o) = if high_pass {
        (
            (1.0 + u) * alpha_y + u * (1.0 - g),
            (1.0 + u) * beta_y + u * (1.0 - g),
            (1.0 + u) * nu_y + 1.0,
        )
    } else {
        (alpha_y, beta_y, nu_y)
    };
    let inv_rho = up(1.0 / down(1.0 - delta_cap));
    let root_min = down(theta_min.sqrt());
    let root_max = up(theta_max.sqrt());
    let a_term = up(alpha_o / root_min + alpha_s * inv_rho);
    let b_term = up(beta_o + root_max * beta_s * inv_rho);
    SectionRounding {
        lambda: up(up(a_term * a_term + b_term * b_term).sqrt()),
        abs: up(nu_o + root_max * nu_s * inv_rho),
    }
}

/// The diffuser stage's majorant coefficients at `k`: a ring write before its flush is at most
/// `v_d |d| + v_a |a| + v_u U`, a stage output at most `c_d |d| + c_a |a| + c_u U` (`d` the word
/// `M` samples old, `a` the stage input).
#[derive(Clone, Copy, Debug)]
struct Stage {
    v_d: f64,
    v_a: f64,
    v_u: f64,
    c_d: f64,
    c_a: f64,
    c_u: f64,
}

impl Stage {
    fn new(k: f64) -> Self {
        let u = U;
        let e_d = u * (2.0 + u) * k;
        let e_a = u;
        let e_u = 2.0 + u;
        Self {
            v_d: up((1.0 + u) * (1.0 + u) * k),
            v_a: up(1.0 + u),
            v_u: up(2.0 + u),
            c_d: up((1.0 - k * k) * (1.0 + u) + (1.0 + u) * (k * e_d + u * k * (k + e_d))),
            c_a: up(k + (1.0 + u) * (k * e_a + u * k * (1.0 + e_a)) + u * k),
            c_u: up((1.0 + u) * (k * e_u + u * k * e_u + 1.0) + 1.0),
        }
    }
}

/// The diffuser majorant's rates tried: `delta_d = k / 64` of the largest feasible rate.
const DIFFUSER_RATE_STEPS: u32 = 64;

/// Sample counts that do not depend on the diffuser majorant's rate.
#[derive(Clone, Copy, Debug)]
struct Fixed {
    /// `floor(fs / 4)`: the pre-delay drains by `N + pre_max`.
    pre_max: u64,
    /// Pre-delay ring words: every one holds an input word from `N` on by `N + pre_len`.
    pre_len: u64,
    /// The longest diffuser ring.
    diffuser_ring: u64,
    /// The longest line ring.
    line_ring: u64,
    /// `D_max` plus the low-pass and high-pass walks.
    walks: u64,
}

/// Phase B of the derivation (S5-S7): from `t_p = N + pre_max` the pre-delay feeds zeros, the
/// diffusers decay geometrically at `1 - delta_d`, and the loop at `1 - delta`.
#[derive(Clone, Copy, Debug)]
struct PhaseB {
    stage: Stage,
    /// Each stage index's longest ring.
    stage_len: [f64; 4],
    /// Phase A's invariants: ring words and outputs of each stage.
    words_a: [f64; 4],
    outputs_a: [f64; 4],
    /// `sqrt(Phi)` at `t_p`.
    start: f64,
    /// `C_q`: the loop's gain to one injected word per line.
    injection: f64,
    /// `C_U`: the loop's absolute term per sample.
    loop_abs: f64,
    delta: f64,
    output_gain: f64,
    output_abs: f64,
}

impl PhaseB {
    /// The tail and rest at diffuser rate `1 - delta_d`; `None` where that rate is infeasible or
    /// too close to the loop's.
    fn candidate(&self, delta_d: f64, fixed: Fixed) -> Option<Candidate> {
        let stage = self.stage;
        let delta = self.delta;
        let gap = (delta_d - delta).abs();
        if gap <= delta.min(delta_d) / 1024.0 {
            return None;
        }
        let per_sample = exp_up(nu_plus(delta_d));
        // hat: the input's geometric coefficient, floor: its absolute part (stage 0: zero).
        let (mut hat, mut floor) = (0.0_f64, 0.0_f64);
        let (mut word_hat, mut word_floor) = (0.0_f64, 0.0_f64);
        let mut output = (0.0_f64, 0.0_f64);
        for s in 0..4 {
            let back = exp_up(self.stage_len[s] * nu_plus(delta_d));
            let contraction = up(stage.v_d * back);
            if contraction >= 1.0 {
                return None;
            }
            let words = self.words_a[s].max(up(stage.v_a * hat / down(1.0 - contraction)));
            let words_floor =
                up((stage.v_a * floor + stage.v_u * UNDERFLOW) / down(1.0 - stage.v_d));
            output = (
                up(stage.c_d * words * back + stage.c_a * hat),
                up(stage.c_d * words_floor + stage.c_a * floor + stage.c_u * UNDERFLOW),
            );
            word_hat = word_hat.max(words);
            word_floor = word_floor.max(words_floor);
            hat = self.outputs_a[s].max(up(output.0 * per_sample));
            floor = output.1;
        }
        let injection_hat = up(0.5 * output.0);
        let injection_floor = up(0.5 * output.1 + UNDERFLOW);
        let start = up(self.start + self.injection * injection_hat / down(gap));
        let nu = delta.min(delta_d);
        let loop_floor = up((self.injection * injection_floor + self.loop_abs) / down(delta));
        let writes = decay_samples(start, down(FLUSH - loop_floor), nu)?;
        let diffusers = decay_samples(word_hat, down(FLUSH - word_floor), delta_d)?;
        let floor_out = up(self.output_abs + self.output_gain * loop_floor);
        let audible = decay_samples(up(self.output_gain * start), floor_out, nu)?;
        let rest = (fixed.pre_max)
            .checked_add(diffusers)?
            .checked_add(fixed.diffuser_ring + 4)?
            .max(
                fixed
                    .pre_max
                    .checked_add(writes)?
                    .checked_add(fixed.line_ring.max(fixed.walks))?,
            )
            .max(fixed.pre_len);
        let tail = fixed.pre_max.checked_add(audible)?.min(rest);
        Some(Candidate { tail, rest })
    }
}

/// `miso.reverb`'s tail, tail over every peak and exact-rest bound at `sample_rate`, for every
/// design whose decay is at most `decay_max_ms` and whose other controls lie in the descriptor's
/// domains (decision 15 D15-4(b), #2114 S3). [`crate::REVERB_DESCRIPTOR`]'s `tail_and_rest` is this
/// at the descriptor's longest decay; the tail contract's rest gate evaluates it at a short one.
///
/// The derivation is `docs/derivations/2114-reverb-tail-and-rest.md`. The bound holds under every
/// admitted control history before `N` and for every input, because the block check bounds every
/// readable word by `1e30` at the settle point; so it does not depend on the input peak, and the
/// tail is the rest bound whenever that is the smaller. No composition value is stated.
///
/// `UNBOUNDED` where the derivation does not cover the rate (a damping coefficient above `1/2`) or
/// a value overflows.
#[must_use]
pub fn tail_bound_at(sample_rate: u32, decay_max_ms: f32) -> NodeTailBound {
    stated_bound(sample_rate, decay_max_ms).unwrap_or(NodeTailBound::UNBOUNDED)
}

/// The rest and tail of one choice of the diffuser majorant's rate.
#[derive(Clone, Copy, Debug)]
struct Candidate {
    tail: u64,
    rest: u64,
}

fn stated_bound(sample_rate: u32, decay_max_ms: f32) -> Option<NodeTailBound> {
    if !(decay_max_ms.is_finite() && decay_max_ms > 0.0) {
        return None;
    }
    let g = geometry(sample_rate);
    let fs = f64::from(sample_rate);
    let parameter = |index: usize| &REVERB_PARAMETERS[index];

    // Geometry: the shortest tap over the size domain, the longest tap, rings and pre-delay.
    let size_min = parameter(SIZE).minimum?;
    let scaled = math::floor(f64::from(BASE[0]) * f64::from(size_min) * fs / 48_000.0 + 0.5);
    if !(1.0..=1.0e9).contains(&scaled) {
        return None;
    }
    let d_min = f64::from(const_nearest_prime(scaled as u32));
    let line_len_max = u64::from(*g.line_len.iter().max()?);
    let d_max = f64::from(*g.line_len.iter().max()? - RING_SLACK);
    let m_max = u64::from(*g.diffuser_len.iter().max()?);
    let pre_max = u64::from(g.pre_max);
    let pre_len = u64::from(g.pre_len);
    if pre_max < SETTLE_SAMPLES {
        return None;
    }

    // Coefficient domains, padded by 2^-20 for any ulp the f64 design or its f32 rounding moves.
    let pad = 1.0 / 1_048_576.0;
    let g1_lo = f64::from(damping_coefficient(
        parameter(DAMPING).minimum?,
        sample_rate,
    )) * (1.0 - pad);
    let g1_design_max = damping_coefficient(parameter(DAMPING).maximum?, sample_rate);
    if !(g1_design_max <= 0.5 && g1_lo > 0.0) {
        return None;
    }
    let g1_hi = 0.5;
    let g2_lo = f64::from(low_cut_coefficient(
        parameter(LOW_CUT).minimum?,
        sample_rate,
    )) * (1.0 - pad);
    let g2_hi = f64::from(low_cut_coefficient(
        parameter(LOW_CUT).maximum?,
        sample_rate,
    )) * (1.0 + pad);
    let k_max = f64::from(diffusion_coefficient(parameter(DIFFUSION).maximum?));
    if !(g2_lo > 0.0 && g2_hi < 0.25 && (0.0..1.0).contains(&k_max)) {
        return None;
    }
    // Every r-scaled section is exactly dissipative for delta up to the cap (derivation, S2).
    let delta_cap = g2_lo / 2.0;

    // The scaled storage weights theta over the coefficient range and delta in [0, delta_cap].
    let lp_theta_min = down((1.0 - g1_hi) * (2.0 - delta_cap) * (1.0 - delta_cap) / (4.0 * g1_hi));
    let lp_theta_max = up((1.0 - g1_lo) / down(2.0 * g1_lo - delta_cap));
    let hp_theta_min =
        down((1.0 - g2_hi) * (1.0 - g2_hi) * (2.0 - delta_cap) * (1.0 - delta_cap) / (4.0 * g2_hi));
    let hp_theta_max =
        up((1.0 - g2_lo) * (1.0 - g2_lo) / down(g2_lo * (2.0 - delta_cap - 2.0 * g2_hi)));
    let lp = section_rounding(g1_hi, lp_theta_min, lp_theta_max, false, delta_cap);
    let hp = section_rounding(g2_hi, hp_theta_min, hp_theta_max, true, delta_cap);

    // a = 3 ln(10) / (fs T): every design's line gain is at most e^(-a D) (1 + u) (1 + EVAL).
    let a_lo = down(3.0 * core::f64::consts::LN_10 / (fs * f64::from(decay_max_ms) / 1000.0));

    // The certificate (derivation, S3): gamma_c Gamma_1c Gamma_2c <= 1 at r = 1 - delta.
    let factors = |delta: f64| -> Option<(f64, f64, f64)> {
        let d1 = delta + lp.lambda;
        let d2 = delta + hp.lambda;
        if d1 > delta_cap || d2 > delta_cap {
            return None;
        }
        let gamma1 = up(up(g1_lo * (2.0 - d1) / down(2.0 * g1_lo - d1)) * (1.0 + lp.lambda));
        let gamma2 =
            up(up((1.0 - g2_hi) * (2.0 - d2) / down(2.0 - 2.0 * g2_hi - d2)) * (1.0 + hp.lambda));
        let exponent = d_min * (nu_plus(delta) - a_lo);
        if exponent > 0.0 {
            return None;
        }
        let gamma = up(grow_pow(7) * (1.0 + EVAL) * exp_up(exponent));
        Some((gamma, gamma1, gamma2))
    };
    let certified = |delta: f64| {
        factors(delta).is_some_and(|(gamma, gamma1, gamma2)| up(gamma * gamma1 * gamma2) <= 1.0)
    };
    if !certified(0.0) {
        return None;
    }
    let (mut lo, mut hi) = (0.0_f64, a_lo.min(delta_cap));
    if certified(hi) {
        lo = hi;
    } else {
        loop {
            let mid = lo + (hi - lo) / 2.0;
            if mid <= lo || mid >= hi {
                break;
            }
            if certified(mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
    }
    let delta = lo;
    if delta <= 0.0 {
        return None;
    }
    let (gamma_c, _, gamma2_c) = factors(delta)?;
    let nup = nu_plus(delta);

    // The loop's absolute terms and its gain from the injection (derivation, S4).
    // sqrt(sum_i r^(-2 (D_i - 1))), the weighted norm of one injected word per line, and the
    // pending words' weight sum_i sum_(j < D_i) r^(-2 j), at the longest taps (size 2).
    let taps_max = g.line_len.map(|len| f64::from(len - RING_SLACK));
    let weight = up(taps_max
        .iter()
        .map(|&d| exp_up(2.0 * (d - 1.0) * nup))
        .sum::<f64>()
        .sqrt());
    let pending = up(taps_max
        .iter()
        .map(|&d| up(math::expm1(up(2.0 * d * nup))) / down(delta * (2.0 - delta)))
        .sum::<f64>());
    let root2 = up(core::f64::consts::SQRT_2);
    let butterfly =
        up(4.0
            * (1.0 + root2 * grow(1.0) + 2.0 * grow(1.0) * grow(1.0) + 2.0 * root2 * grow_pow(3)));
    let slot_abs = up(gamma2_c * lp.abs + hp.abs);
    let write_abs = up(grow(1.0) * grow(1.0) * gamma_c * butterfly / 4.0 + (2.0 + U) * weight);
    let loop_abs = up(4.0 * gamma_c * slot_abs + write_abs) * UNDERFLOW;
    let injection = up(grow(1.0) * weight);

    // Phi at t0: readable ring words and filter states from the block invariant (S1).
    let tap0 = up(grow(6.0) * LIMIT + 3.0 * UNDERFLOW);
    let state = down(1.0 - 7.0 * U);
    let lp0 = up((tap0 + 4.0 * UNDERFLOW) / state);
    let lp_out0 = up(grow(3.0) * lp0 + 2.0 * UNDERFLOW);
    let hp0 = up((lp_out0 + 4.0 * UNDERFLOW) / state);
    let lp_theta_c = up(lp_theta_max * (1.0 + lp.lambda) * (1.0 + lp.lambda));
    let hp_theta_c = up(hp_theta_max * (1.0 + hp.lambda) * (1.0 + hp.lambda));
    let ring0 = up(LIMIT * up(pending.sqrt()));
    let filters0 =
        up(4.0 * up(gamma2_c * gamma2_c * lp_theta_c * lp0 * lp0 + hp_theta_c * hp0 * hp0).sqrt());
    let phi0 = up(ring0 + filters0);

    // The diffusers while the pre-delay drains (phase A, S5): l-inf invariants at k_max.
    let stage = Stage::new(k_max);
    let pipe0 = up(grow(1.0) * (LIMIT + grow(1.0) * k_max * LIMIT + UNDERFLOW) + UNDERFLOW);
    let mut words_a = [0.0_f64; 4];
    let mut outputs_a = [0.0_f64; 4];
    let mut input = LIMIT;
    for s in 0..4 {
        let words =
            up((stage.v_a * input + stage.v_u * UNDERFLOW) / down(1.0 - stage.v_d)).max(LIMIT);
        let output = up(stage.c_d * words + stage.c_a * input + stage.c_u * UNDERFLOW);
        words_a[s] = words;
        outputs_a[s] = output;
        input = output.max(pipe0);
    }
    let injection_a = up(0.5 * outputs_a[3] + UNDERFLOW);
    let drain = (pre_max - SETTLE_SAMPLES) as f64;
    let phi_p = up(phi0 + drain * up(injection * injection_a + loop_abs));

    // Phase B: a geometric majorant at rate 1 - delta_d; the best of a few rates is kept.
    let output_gain = up(2.0 * grow(9.0) / gamma_c);
    let output_abs = up(
        2.0 * grow(5.0) * (grow(4.0) * 4.0 * slot_abs + butterfly / 4.0 + 1.0)
            + 6.0 * grow(1.0)
            + 1.0,
    ) * UNDERFLOW;
    let lp_start = up(FLUSH / down(gamma_c * down(lp_theta_min.sqrt())));
    let hp_bound = up(FLUSH / down(gamma_c * down(hp_theta_min.sqrt())));
    let lp_walk = rate_follower_rest_samples(
        f32_at_or_below(down(2.0 * g1_lo - 3.0 * U)),
        f32_at_or_above(lp_start),
        RestSnap::Below(lane::FLUSH_EPS),
    )?;
    let hp_start = hp_bound.max(up((grow(3.0) * lp_start + 6.0 * UNDERFLOW) / state));
    let hp_walk = rate_follower_rest_samples(
        f32_at_or_below(down(2.0 * g2_lo - 3.0 * U)),
        f32_at_or_above(hp_start),
        RestSnap::Below(lane::FLUSH_EPS),
    )?;
    let walks = (d_max as u64).checked_add(lp_walk)?.checked_add(hp_walk)?;

    // Each stage index's longest ring over the two channels (lanes s and 4 + s).
    let stage_len: [f64; 4] =
        core::array::from_fn(|s| f64::from(g.diffuser_len[s].max(g.diffuser_len[s + 4])));
    let phase_b = PhaseB {
        stage,
        stage_len,
        words_a,
        outputs_a,
        start: phi_p,
        injection,
        loop_abs,
        delta,
        output_gain,
        output_abs,
    };
    let fixed = Fixed {
        pre_max,
        pre_len,
        diffuser_ring: m_max,
        line_ring: line_len_max,
        walks,
    };
    // The geometric rate must keep `v_d (1 - delta_d)^(-M)` below 1 on the longest stage.
    let delta_d_max = -math::log(stage.v_d) / up(m_max as f64);
    let mut best: Option<Candidate> = None;
    for step in 1..DIFFUSER_RATE_STEPS {
        let delta_d = delta_d_max * f64::from(step) / f64::from(DIFFUSER_RATE_STEPS);
        if let Some(candidate) = phase_b.candidate(delta_d, fixed) {
            best = Some(match best {
                Some(kept) => Candidate {
                    tail: kept.tail.min(candidate.tail),
                    rest: kept.rest.min(candidate.rest),
                },
                None => candidate,
            });
        }
    }
    let Candidate { tail, rest } = best?;
    let tail = tail.min(rest);
    Some(NodeTailBound {
        tail: TailSamples::Finite(tail),
        tail_every_peak: TailSamples::Finite(rest),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: rest,
            any_sanitized_input: rest,
        }),
        composition: CompositionBound::Unstated,
    })
}
