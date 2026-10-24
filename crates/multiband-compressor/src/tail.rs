//! The multiband compressor's certified tail and exact-rest bounds (issue #1373, tranche b;
//! decision 15 D15-4(b)).
//!
//! Control-plane only. [`multiband_live_bound_table`] is what the descriptor's `tail_and_rest`
//! reads: the bound depends only on the rate, so it is pinned per launch rate and computed
//! nowhere on a preparation path. [`multiband_live_bound`] computes it (through `math::tail`, which
//! allocates); the gate `tests/tail_contract.rs` holds every pinned entry equal to it.
//!
//! The derivation is `docs/derivations/1373-multiband-compressor-tail-and-rest.md` (part 1: the
//! decomposition `|y| <= G_max (2 |lp2| + |ap1|)` and the first-stage term; part 2: this module's
//! envelope, gains, smoother rest and composition). The sums and the zone propagation are
//! `math::tail`'s ([`live_cascade`] for `lp2`, [`live_first_section`] for `ap1`); this module
//! supplies the crossover's own terms: the suprema over every word the iterated 64-sample word
//! ramps (#1338) can load across the 80 Hz to 8 kHz domain, the band gains' ceiling and the
//! smoother words' rest.
//!
//! Every bound is computed under the canonical floating-point environment
//! (`lane::CanonicalFpEnv`), as the builtin input section's is.

use effect_contract::{
    CompositionBound, FlushStall, NodeTailBound, PeakGain, RestBound, RestSamples, TailDecay,
    TailSamples,
};
use effect_runtime::envelope::retention_coefficient;
use effect_runtime::tail::{RestSnap, pole_follower_rest_samples};
use math::tail::{
    CascadeBound, EnvelopeSection, FirstSectionRow, FlushLaw, LiveCascade, LiveComposition,
    PoleDomain, Rounding, SvfWords, TAIL_FLOOR, TailBoundError, UNDERFLOW, ceil_millibels,
    live_cascade, live_cascade_composition, live_first_section, live_first_section_composition,
    output_rounding, state_rounding, v_dual_norm, v_norm, word_box_norm,
};

use crate::{MULTIBAND_COMPRESSOR_PARAMETERS, NEGATIVE_TWO_K, SMOOTHING_SAMPLES};

/// `2^-24`, the `f32` unit roundoff.
const U: f64 = 1.0 / 16_777_216.0;

/// `1 + 2^-30`: the inflation of one short, non-cancelling `f64` evaluation (`math::tail`'s
/// `SLACK`).
const SLACK: f64 = 1.0 + 1.0 / 1_073_741_824.0;

/// The fast tier's proven decibel error, rounded up: `math::fast_db`'s exhaustive F1 sweep bounds
/// `fast_gain_from_db` by `7.431e-6` dB on `[-160, 0]` and `2.183e-6` dB on `[0, 24]`, every `f32`
/// input (the gate-expander's and the transient shaper's constant).
const FAST_GAIN_ERROR_DB: f64 = 1.0e-5;

/// The crossover parameter's domain, in Hz (`crossover_hz`).
fn crossover_domain() -> (f64, f64) {
    let row = &MULTIBAND_COMPRESSOR_PARAMETERS[0];
    (
        f64::from(row.minimum.expect("a minimum")),
        f64::from(row.maximum.expect("a maximum")),
    )
}

/// The slowest release the descriptor admits, in ms (`low_release_ms` and `high_release_ms` share
/// the domain).
fn slowest_release_ms() -> f32 {
    MULTIBAND_COMPRESSOR_PARAMETERS[4]
        .maximum
        .expect("a maximum")
        .max(
            MULTIBAND_COMPRESSOR_PARAMETERS[9]
                .maximum
                .expect("a maximum"),
        )
}

/// The largest makeup the descriptor admits, in dB.
fn largest_makeup_db() -> f64 {
    f64::from(
        MULTIBAND_COMPRESSOR_PARAMETERS[5]
            .maximum
            .expect("a maximum")
            .max(
                MULTIBAND_COMPRESSOR_PARAMETERS[10]
                    .maximum
                    .expect("a maximum"),
            ),
    )
}

const fn launch_rate(sample_rate: u32) -> bool {
    matches!(sample_rate, 44_100 | 48_000 | 88_200 | 96_000)
}

/// The flush law of #1328 at `sample_rate`, from the engine's own constants.
fn flush_law(sample_rate: u32) -> FlushLaw {
    FlushLaw {
        flush_eps: f64::from(lane::FLUSH_EPS),
        rest_eps: f64::from(lane::REST_EPS),
        silence_frames: u64::from(lane::silence_frames(sample_rate)),
    }
}

/// The two peaks the contract states exact rest for: +24 dBFS (rounded up) and the input
/// sanitizer's limit.
fn rest_peaks() -> [f64; 2] {
    [
        math::db_to_gain(24.0) * SLACK,
        f64::from(lane::kernels::builtins::NONFINITE_LIMIT),
    ]
}

/// `G_max` of the derivation's part 2, with the output's three `f32` roundings folded in: every
/// band amplitude is `fast_gain_from_db(fl(smoothed + makeup))`, the smoothed reduction in
/// `[-100, 0]` (its target's clamp; a smoother step is a rounded convex combination of its word and
/// its target) and the makeup word in `[-24, 24]` (a ramp stays inside its endpoints, #1409), so
/// the argument lies in `[-124, 24]`, inside the F1 sweep's domain, and the amplitude is at most
/// `10^((24 + FAST_GAIN_ERROR_DB) / 20)`. The output `fl(fl(low g_lo) + fl(high g_hi))`, with
/// `high = fl(ap1 - lp2)`, is at most `(1 + u)^3 G (2 |lp2| + |ap1|)` (part 1, decision (a)).
pub fn output_gain() -> f64 {
    let word = math::db_to_gain(largest_makeup_db() + FAST_GAIN_ERROR_DB);
    word * (1.0 + U) * (1.0 + U) * (1.0 + U) * SLACK
}

/// The largest retention coefficient a band's release word can hold at `sample_rate`, or `None`
/// when the argument below does not apply. A release word is `retention_coefficient(t, fs)` of a
/// legal time `t <= 5000` ms, or a ramp word between two such (#1409); that is
/// `clamp(expf(x), 0, 1)` with `x = -1 / fl(fl(t 0.001) fs)`, which does not decrease with `t`, so
/// `x <= x_max`, the slowest release's. `expf` is not proven monotone, but it is within one ulp of
/// `exp` (`math`'s crate note: the vendored rust-lang/libm `expf`, FreeBSD's `e_expf.c`;
/// `2^-24` on `[1/2, 1)`, where every launch-rate value lies; `tests/tail_contract.rs` T3 checks
/// it exhaustively over every legal release argument at the launch rates). For
/// `x < x_max - delta`, `delta = 2^-21`: `expf(x) <= exp(x) + 2^-24 < exp(x_max) - 2^-22 + 2^-24
/// <= expf(x_max) - 2^-23` (`exp(x_max) >= 1/2`, so `exp(x_max) (1 - e^-delta) > 2^-22`). So the
/// largest value is `expf` of one of the `f32` arguments in `[x_max - delta, x_max]`, all of which
/// are scanned (about a million at 44.1 kHz).
pub fn smoother_retention_max(sample_rate: u32) -> Option<f32> {
    let slowest = slowest_release_ms();
    let x_max = -1.0 / (slowest * 0.001 * sample_rate as f32);
    let top = math::expf(x_max).clamp(0.0, 1.0);
    // The scan reads the same law as the effect's own design.
    if top != retention_coefficient(slowest, sample_rate) || !(0.5..1.0).contains(&top) {
        return None;
    }
    let lowest = f64::from(x_max) - 1.0 / 2_097_152.0;
    let mut largest = top;
    let mut x = x_max;
    while f64::from(x) >= lowest {
        largest = largest.max(math::expf(x).clamp(0.0, 1.0));
        // `x < 0`: the next `f32` below it has the next larger bit pattern.
        x = f32::from_bits(x.to_bits() + 1);
    }
    Some(largest)
}

/// Zero-input frames after which every band's smoother word is `+0.0`, from any word in
/// `[-100, 0]` dB, once both bands' signals are exactly zero: `None` when the bound does not exist
/// (the #1375 D3 stall check). With both bands `+-0` the detector floors at -160 dB, below every
/// threshold's knee, so the target is `+0.0`; for a word `y <= 0` the branching smoother takes the
/// release arm (`0 < y` is false) and its step is `flush(fl(fl(c y) + 0))`, the pole form
/// [`pole_follower_rest_samples`] bounds, at a retention of at most [`smoother_retention_max`].
pub fn smoother_rest_samples(sample_rate: u32) -> Option<u64> {
    pole_follower_rest_samples(
        smoother_retention_max(sample_rate)?,
        100.0,
        RestSnap::Below(lane::FLUSH_EPS),
    )
}

/// The exact design's words `(c1, a2, a3)` at prewarped frequency `g`.
fn exact_words(g: f64) -> [f64; 3] {
    let t = g * (g + core::f64::consts::SQRT_2);
    let c1 = t / (1.0 + t);
    [c1, g * (1.0 - c1), g * g * (1.0 - c1)]
}

/// The crossover's live envelope at `sample_rate`: the cascade both stages form (each a
/// Butterworth low-pass section, the second fed by the first's low-pass tap) and the all-pass
/// tap's row on the first stage. `None` off the launch rates.
///
/// * **Words.** Both stages load the same `(c1, a2, a3)`: an `f32` design of a crossover in
///   `[80 Hz, 8 kHz]` (within the half-ulp box `h = (u/2, u/4, u/2)` of the exact design, since
///   `c1, a3 < 1` and `a2 < 1/2` over the domain at every launch rate), or a word of the iterated
///   ramp (#1338: `s = fl(t - c) / 64`, exact since `64` is a power of two and the difference is
///   far from underflow; `w = clamp(fl(w + s))` toward the target; the target assigned on the
///   64th frame). After `j` frames an unclamped word lies within `j h + (j / 64) |t - c| u` of the
///   chord point `c + (j / 64) (t - c)`; the endpoint clamp only moves a word toward that point or
///   holds it where its step was below its rounding (`|s| <= h`), so the bound holds with `h`
///   inflated by `1 + 3u`. A retarget restarts from the word in force: the start's distance `E`
///   from the hull of the designs is carried with weight `1 - j/64`, so
///   `E = max_j (64 / j) rho_j = 64 h (1 + 3u) + u |t - c|` is a fixed point; `|t - c|` is at most
///   the largest word (every word is positive) plus `E`, and `2 u` times the largest word covers
///   both. A word is then within `E + h` of the convex hull of the exact designs.
/// * **Contraction.** `||A(w)||_V` is convex in the words, so over the hull it is largest at an
///   exact design; the pole radius `sqrt(1 + g^4) / (1 + sqrt(2) g + g^2)` falls on `g < 1` (the
///   whole domain), so 80 Hz is the slowest. The box adds [`word_box_norm`]; the state rounding
///   [`state_rounding`] increases in every word's magnitude, which is largest at 8 kHz (`c1`,
///   `a2 = g / (1 + sqrt(2) g + g^2)` and `a3` rise on `g < 1`).
/// * **Input column.** `||b||_V = 2 g / sqrt(1 + g (g + sqrt(2)))`, increasing in `g`, plus the
///   box's `2 (e2, e3)` and the input rounding.
/// * **Low-pass row** (`m = (0, 0, 1)`, both stages' outputs). `c = (a2, 1 - a3)`,
///   `||c||_V* = sqrt(2) / sqrt(1 + t)` at a design, largest at 80 Hz; the box adds the dual norm
///   of `(e2, -e3)`, and the output rounding [`output_rounding`] is taken at the largest words plus
///   at the corner where `c1` and `a3` are `-(E + h)` (its `|1 - c1|`, `|1 - a3|` terms), as the
///   builtin input section's. `|d| = a3`, at most the largest `a3` plus the box.
/// * **All-pass tap** (`m = (1, fl(-2k), 0)`, part 1). `||(m1, 0)||_V* = 2 |m1|`; its output
///   rounding the same way; `|d| = |1 + m1 a2| <= 1 + |m1| (E + h)` (`0 < 1 - 2k a2` on every
///   design).
/// * **Pole domain.** `Re p = (1 - g^2) / (1 + sqrt(2) g + g^2)` falls in `g`, so the designs' poles
///   lie in `[Re p(g_8k), Re p(g_80)]`, padded by `1e-12` (far above the `f64` error of `tan` here).
#[must_use]
pub fn crossover_live_envelope(sample_rate: u32) -> Option<(LiveCascade, FirstSectionRow)> {
    if !launch_rate(sample_rate) {
        return None;
    }
    let _environment = lane::CanonicalFpEnv::enter();
    let sqrt2 = core::f64::consts::SQRT_2;
    let rate = f64::from(sample_rate);
    let (low_hz, high_hz) = crossover_domain();
    let prewarp = |hz: f64| math::tan(core::f64::consts::PI * hz / rate);
    let (g_min, g_max) = (prewarp(low_hz), prewarp(high_hz));
    let largest = exact_words(g_max);
    let half_ulp = [U / 2.0, U / 4.0, U / 2.0];
    // The half-ulp box needs `c1, a3 < 1` and `a2 < 1/2`; checked rather than assumed.
    if !(largest[0] < 1.0 && largest[1] < 0.5 && largest[2] < 1.0 && g_max < 1.0) {
        return None;
    }
    let allowance: [f64; 3] =
        core::array::from_fn(|i| 64.0 * half_ulp[i] * (1.0 + 3.0 * U) + 2.0 * U * largest[i]);
    let off_design: [f64; 3] = core::array::from_fn(|i| allowance[i] + half_ulp[i]);

    let radius =
        math::sqrt(1.0 + g_min * g_min * g_min * g_min) / (1.0 + sqrt2 * g_min + g_min * g_min);
    let q_ramp = radius * SLACK + word_box_norm(off_design);
    let q_settled = radius * SLACK + word_box_norm(half_ulp);
    let t_max = g_max * (g_max + sqrt2);
    let beta = 2.0 * g_max / math::sqrt(1.0 + t_max) * SLACK;
    let rounding = |extra: [f64; 3], q: f64| {
        state_rounding(core::array::from_fn(|i| largest[i] + extra[i]), q, beta)
    };
    let (mu_ramp, mu_input) = rounding(off_design, q_ramp);
    let (mu_settled, _) = rounding(half_ulp, q_settled);
    let signs = |bit_a: u32, bit_b: u32, signs: u32| {
        let sign = |bit: u32| if signs & bit == 0 { 1.0 } else { -1.0 };
        (sign(bit_a), sign(bit_b))
    };
    let mut column = 0.0_f64;
    let mut row_box = 0.0_f64;
    for pattern in 0..4_u32 {
        let (first, second) = signs(1, 2, pattern);
        column = column.max(v_norm([
            2.0 * off_design[1] * first,
            2.0 * off_design[2] * second,
        ]));
        row_box = row_box.max(v_dual_norm([off_design[1] * first, off_design[2] * second]));
    }

    // The low-pass row of both stages.
    let t_min = g_min * (g_min + sqrt2);
    let gamma = sqrt2 / math::sqrt(1.0 + t_min) * SLACK;
    let top = |m0: f64, m1: f64, m2: f64| SvfWords {
        c1: largest[0] + off_design[0],
        a2: largest[1] + off_design[1],
        a3: largest[2] + off_design[2],
        m0,
        m1,
        m2,
    };
    let supremum = |words: SvfWords| {
        let corner = SvfWords {
            c1: -off_design[0],
            a3: -off_design[2],
            ..words
        };
        let (largest_state, input) = output_rounding(&words);
        (largest_state + output_rounding(&corner).0, input)
    };
    let (omega_low, omega_low_input) = supremum(top(0.0, 0.0, 1.0));
    let low_row = v_dual_norm([0.0, 1.0]) * SLACK;
    let envelope = EnvelopeSection {
        rho_ramp: q_ramp + mu_ramp,
        rho_settled: q_settled + mu_settled,
        input: beta + column + mu_input,
        output_state: gamma + row_box + omega_low,
        output_input: (largest[2] + off_design[2]) * SLACK + omega_low_input,
    };
    let real_pole = |g: f64| (1.0 - g * g) / (1.0 + sqrt2 * g + g * g);
    const POLE_PAD: f64 = 1.0e-12;
    let poles = PoleDomain {
        low: real_pole(g_max) - POLE_PAD,
        high: real_pole(g_min) + POLE_PAD,
        ramp_box: off_design,
        design_box: half_ulp,
        state_rounding: mu_ramp,
        settled_state_rounding: mu_settled,
        input_rounding: mu_input,
    };
    let live = LiveCascade {
        envelope,
        poles,
        first_mix_row: low_row,
        first_output_rounding: omega_low,
        second_mix_row: low_row,
        second_output_rounding: omega_low,
    };

    // The all-pass tap on the first stage.
    let m1 = f64::from(NEGATIVE_TWO_K);
    let (omega_tap, omega_tap_input) = supremum(top(1.0, m1, 0.0));
    let tap = FirstSectionRow {
        mix_row: v_dual_norm([m1, 0.0]) * SLACK,
        output_rounding: omega_tap,
        feedthrough: (1.0 + m1.abs() * off_design[1]) * SLACK + omega_tap_input,
    };
    Some((live, tap))
}

/// The raw values behind [`multiband_live_bound`] (evidence for the derivation and the gates).
#[derive(Clone, Debug, PartialEq)]
pub struct MultibandLiveBound {
    /// `lp2` at the scaled gain `4 G`: its `T` and `P*` enter the node's.
    pub low_tail: CascadeBound,
    /// `ap1` at the scaled gain `2 G`.
    pub tap_tail: CascadeBound,
    /// `lp2` at unit gain: its rests at the stated peaks.
    pub low_rest: CascadeBound,
    /// `ap1` at unit gain.
    pub tap_rest: CascadeBound,
    /// The crossover's rest at the flush floor `P*`, the larger of the two terms'.
    pub rest_at_flush_floor: u64,
    /// The node's `T`, `max(T_lp, T_ap)`.
    pub tail: u64,
    /// The node's `P*`, `G (2 P*_lp + P*_ap)` plus the output's underflow.
    pub flush_floor: f64,
    /// [`smoother_rest_samples`].
    pub smoother_rest: u64,
    /// The composition values, or `None` when they are not certified.
    pub composition: Option<LiveComposition>,
}

/// The four absolute underflow allowances of the output's chain (the subtraction, two products,
/// the sum), each carried through at most `(1 + u)^2` and the gain.
fn output_underflow(gain: f64) -> f64 {
    4.0 * UNDERFLOW * gain
}

/// The full computation behind [`multiband_live_bound`]. `Ok(None)` off the launch rates.
///
/// # Errors
///
/// `math::tail`'s, when a term is not bounded.
pub fn multiband_live_terms(
    sample_rate: u32,
) -> Result<Option<MultibandLiveBound>, TailBoundError> {
    let Some((live, tap)) = crossover_live_envelope(sample_rate) else {
        return Ok(None);
    };
    let Some(smoother_rest) = smoother_rest_samples(sample_rate) else {
        return Err(TailBoundError::StallAboveRest);
    };
    let _environment = lane::CanonicalFpEnv::enter();
    let law = flush_law(sample_rate);
    let ramp = u64::from(SMOOTHING_SAMPLES);
    let gain = output_gain();
    let peaks = rest_peaks();
    // `T`: `4 G r_lp < eps / 2` and `2 G r_ap < eps / 2`, so `G (2 r_lp + r_ap) < eps / 2`.
    let low_tail = live_cascade(&live, 4.0 * gain, &law, ramp, peaks)?;
    let tap_tail = live_first_section(&live, &tap, 2.0 * gain, &law, ramp, peaks)?;
    let low_rest = live_cascade(&live, 1.0, &law, ramp, peaks)?;
    let tap_rest = live_first_section(&live, &tap, 1.0, &law, ramp, peaks)?;
    let tail = low_tail.tail.max(tap_tail.tail);
    // Each term's flush part from its own `T` (at most the node's) is at most
    // `P*_z (eps / 2)`, so the output's is at most `P*_y (eps / 2)`.
    let flush_floor = (gain * (2.0 * low_tail.flush_floor + tap_tail.flush_floor)
        + output_underflow(gain) / (TAIL_FLOOR / 2.0))
        * SLACK;
    let at_floor = [flush_floor; 2];
    let rest_at_flush_floor = live_cascade(&live, 1.0, &law, ramp, at_floor)?
        .rest_peak
        .max(live_first_section(&live, &tap, 1.0, &law, ramp, at_floor)?.rest_peak);
    let composition = composition(&live, &tap, gain, &law, ramp)?;
    Ok(Some(MultibandLiveBound {
        low_tail,
        tap_tail,
        low_rest,
        tap_rest,
        rest_at_flush_floor,
        tail,
        flush_floor,
        smoother_rest,
        composition,
    }))
}

/// The node's composition values (#1379 Amendment 1 H1), part 2's rules: the gains and stalls
/// from the unit-gain terms, `G_y = G (2 G_lp + G_ap)` and `sigma_y = G (2 sigma_lp + sigma_ap)`
/// plus the output's underflow (each term's tail stall is from its own `T`, at most the node's,
/// and a flush part does not increase with its start), and `D` from the scaled terms,
/// `D_y = max(D_lp, D_ap)`: each states `c r_z < (eps / 2) 10^-k` from `T_z + k D_z`, at or before
/// `T_y + k D_y`. `Ok(None)` when any term's values are not certified.
fn composition(
    live: &LiveCascade,
    tap: &FirstSectionRow,
    gain: f64,
    law: &FlushLaw,
    ramp: u64,
) -> Result<Option<LiveComposition>, TailBoundError> {
    let Some(low) = live_cascade_composition(live, 1.0, law, ramp)? else {
        return Ok(None);
    };
    let Some(tap_unit) = live_first_section_composition(live, tap, 1.0, law, ramp)? else {
        return Ok(None);
    };
    let Some(low_scaled) = live_cascade_composition(live, 4.0 * gain, law, ramp)? else {
        return Ok(None);
    };
    let Some(tap_scaled) = live_first_section_composition(live, tap, 2.0 * gain, law, ramp)? else {
        return Ok(None);
    };
    let combine = |low: f64, tap: f64| gain * (2.0 * low + tap) * SLACK;
    let underflow = output_underflow(gain) * SLACK;
    let mut groups = low_scaled.groups;
    groups.extend(tap_scaled.groups);
    Ok(Some(LiveComposition {
        decay: low_scaled.decay.max(tap_scaled.decay),
        groups,
        peak_gain: combine(low.peak_gain, tap_unit.peak_gain),
        tail_gain: combine(low.tail_gain, tap_unit.tail_gain),
        peak_stall: combine(low.peak_stall, tap_unit.peak_stall) + underflow,
        tail_stall: combine(low.tail_stall, tap_unit.tail_stall) + underflow,
    }))
}

/// The statement of a live composition, as the builtin input section's (#1467 L-D7): `D`, then
/// `G_p`, `G_t`, `sigma_p`, `sigma_t` rounded up into millibels; `Unstated` when the tail gain or
/// stall would exceed its peak value (#1464, #1484 rule (g)).
fn stated_composition(composition: &LiveComposition) -> CompositionBound {
    let peak_gain = ceil_millibels(composition.peak_gain, Rounding::Computed);
    let tail_gain = ceil_millibels(composition.tail_gain, Rounding::Computed);
    let peak_stall = ceil_millibels(composition.peak_stall, Rounding::Computed);
    let tail_stall = ceil_millibels(composition.tail_stall, Rounding::Computed);
    if tail_gain > peak_gain || tail_stall > peak_stall {
        return CompositionBound::Unstated;
    }
    CompositionBound::Stated {
        decay: TailDecay(composition.decay),
        peak_gain: PeakGain::Millibels(peak_gain),
        tail_gain: PeakGain::Millibels(tail_gain),
        peak_stall: FlushStall::Level(peak_stall),
        tail_stall: FlushStall::Level(tail_stall),
    }
}

/// The multiband compressor's bound at `sample_rate`, over every crossover in the domain, every
/// history of 64-sample parameter and crossover-word ramps with no control event at or after the
/// silence's first frame, every link mode, curve and makeup, for both channels (the node's bound
/// is the same on each). `None` off the launch rates; [`NodeTailBound::UNBOUNDED`] when a term is
/// not bounded.
///
/// * `tail = T`, `tail_every_peak = max(T, R_x(P*))`: below `P*` the crossover's taps are exactly
///   zero from `R_x(P*)`, and then so is the output, whatever the gains.
/// * `rest`: `R(P) = max(R_x(P), 64) + R_s`, `R_x` the larger of the two terms' rests at the
///   peak (the cascade's includes the first stage's), `R_s` [`smoother_rest_samples`]: from
///   `N + R_x` both bands are `+-0`, the targets `+0.0` and every ramp settled, so each smoother
///   word reaches `+0.0` within `R_s` more frames. The rest state `Z` is the reset state: the four
///   crossover words and both smoother words `+0.0` (#893's `rest_lanes`), every ramp settled at
///   its target, and the silence counter the clock a reset instance fed the same zeros keeps.
#[must_use]
pub fn multiband_live_bound(sample_rate: u32) -> Option<NodeTailBound> {
    if !launch_rate(sample_rate) {
        return None;
    }
    let Ok(Some(terms)) = multiband_live_terms(sample_rate) else {
        return Some(NodeTailBound::UNBOUNDED);
    };
    let ramp = u64::from(SMOOTHING_SAMPLES);
    let rest = |crossover: u64| crossover.max(ramp) + terms.smoother_rest;
    Some(NodeTailBound {
        tail: TailSamples::Finite(terms.tail),
        tail_every_peak: TailSamples::Finite(terms.tail.max(terms.rest_at_flush_floor)),
        rest: RestBound::Bounded(RestSamples {
            peak_plus_24_dbfs: rest(terms.low_rest.rest_peak.max(terms.tap_rest.rest_peak)),
            any_sanitized_input: rest(terms.low_rest.rest_any.max(terms.tap_rest.rest_any)),
        }),
        composition: terms
            .composition
            .as_ref()
            .map_or(CompositionBound::Unstated, stated_composition),
    })
}

/// [`multiband_live_bound`] at each launch rate, pinned: the descriptor's `tail_and_rest` reads
/// it, so no preparation computes `math::tail`'s live bounds. `None` off the launch rates.
///
/// `tests/tail_contract.rs` holds every entry equal to [`multiband_live_bound`], so a change to
/// the derivation, the flush law, the ramp, the domains or `math::tail` turns it red until the
/// table is restated. Each entry is `[T, T_rest, R(+24 dBFS), R(any)]`, `D`, then `G_p`, `G_t`,
/// `sigma_p`, `sigma_t` in millibels.
#[must_use]
pub const fn multiband_live_bound_table(sample_rate: u32) -> Option<NodeTailBound> {
    const fn bound(
        [tail, every_peak, rest_peak, rest_any]: [u64; 4],
        decay: u64,
        [peak_gain, tail_gain, peak_stall, tail_stall]: [i32; 4],
    ) -> NodeTailBound {
        NodeTailBound {
            tail: TailSamples::Finite(tail),
            tail_every_peak: TailSamples::Finite(every_peak),
            rest: RestBound::Bounded(RestSamples {
                peak_plus_24_dbfs: rest_peak,
                any_sanitized_input: rest_any,
            }),
            composition: CompositionBound::Stated {
                decay: TailDecay(decay),
                peak_gain: PeakGain::Millibels(peak_gain),
                tail_gain: PeakGain::Millibels(tail_gain),
                peak_stall: FlushStall::Level(peak_stall),
                tail_stall: FlushStall::Level(tail_stall),
            },
        }
    }
    match sample_rate {
        44_100 => Some(bound(
            [3_505, 9_579, 11_195_358, 11_203_587],
            288,
            [6_834, 6_259, -27_515, -29_780],
        )),
        48_000 => Some(bound(
            [3_810, 10_434, 12_155_124, 12_164_081],
            314,
            [6_884, 6_261, -27_429, -29_719],
        )),
        88_200 => Some(bound(
            [6_990, 19_334, 22_394_952, 22_411_410],
            578,
            [7_341, 6_279, -26_815, -29_177],
        )),
        96_000 => Some(bound(
            [7_600, 21_063, 24_301_393, 24_319_307],
            627,
            [7_409, 6_281, -26_732, -29_116],
        )),
        _ => None,
    }
}
