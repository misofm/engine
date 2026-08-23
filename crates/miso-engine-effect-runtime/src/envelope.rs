//! Envelope followers and the gate's hysteresis, lane-generic and branchless.
//!
//! Every dynamics processor in the workspace tracks a level with a one-pole follower and switches
//! its coefficient between an attack and a release value. This module owns the two follower forms
//! and the coefficient design; the effects keep only their choice of detector and their parameter
//! table.
//!
//! # Coefficient
//!
//! [`attack_release_coefficient`] returns `1 - exp(-1 / (tau * fs))`, the per-sample coefficient
//! `c` of `y += c * (x - y)`, computed at **control rate** through `miso_engine_math::expf` (D6:
//! never the platform libm). It is a coefficient, not a sample-rate-independent time: it is
//! designed once per parameter change on the control plane and splatted into the lane.

use miso_engine_lane::Lane;

/// Per-sample one-pole *rate* coefficient for a time constant of `time_ms` at `sample_rate`.
///
/// `c = 1 - exp(-1 / (tau * fs))` with `tau = time_ms / 1000`, the standard one-pole step
/// response: after `tau` seconds a follower running `y += c * (x - y)` has covered `1 - 1/e` of
/// the distance to its input. This is the coefficient [`rms_follow`] and
/// `miso_engine_lane::kernels::one_pole_block` take; [`peak_follow`] takes its complement, which
/// is [`retention_coefficient`].
///
/// * `time_ms <= 0`, or a non-finite `time_ms`, gives `1.0` — an instantaneous follower, the only
///   continuous extension of the formula as `tau` goes to zero.
/// * `sample_rate == 0` gives `1.0` for the same reason.
/// * The result is clamped to `[0, 1]`, so no rounding of the exponential can make the follower
///   unstable or run backwards.
///
/// The exponential is evaluated by the vendored scalar layer of `miso-engine-math` (D6), so the
/// same coefficient bits are produced on every target.
#[must_use]
pub fn attack_release_coefficient(time_ms: f32, sample_rate: u32) -> f32 {
    1.0 - retention_coefficient(time_ms, sample_rate)
}

/// Per-sample one-pole *retention* coefficient: `exp(-1 / (tau * fs))`.
///
/// The fraction of the current envelope a follower keeps at each sample, and the complement of
/// [`attack_release_coefficient`]. `1.0` freezes the follower, `+0.0` makes it instantaneous.
///
/// The sign of the exponent is the whole content of this function: `exp(+1 / (tau * fs))` is
/// greater than one and turns every follower in the workspace into a divergent recurrence.
#[must_use]
pub fn retention_coefficient(time_ms: f32, sample_rate: u32) -> f32 {
    if time_ms.is_nan() || time_ms <= 0.0 || sample_rate == 0 {
        return 0.0;
    }
    let tau_samples = time_ms * 0.001 * sample_rate as f32;
    if tau_samples.is_nan() || tau_samples <= 0.0 {
        return 0.0;
    }
    // `expf` of a finite negative argument is in `(0, 1]`; the clamp is belt and braces so that
    // no rounding at the extremes can hand a follower a coefficient that makes it diverge.
    miso_engine_math::expf(-1.0 / tau_samples).clamp(0.0, 1.0)
}

/// One sample of a peak follower, in the single-rounding form.
///
/// `y' = max(|x|, fma(c, y - |x|, |x|))`
///
/// Frozen operation order:
/// 1. `d = y - x_abs`
/// 2. `t = fma(c, d, x_abs)` — **one** rounding: this is the release path
///    `x + c * (y - x)`, and writing it as `c * y + (1 - c) * x` would round twice and would not
///    return `x` exactly at `c = 0`
/// 3. `y' = x_abs.max(t)` — the D8 select form, `select(x_abs > t, x_abs, t)`
///
/// The `max` is what makes the follower attack instantaneously and release with `c`: a peak larger
/// than the decayed envelope is taken as is. `c` is the *retention* coefficient of
/// [`retention_coefficient`]: `c = 0` holds the envelope at the input and `c = 1` freezes it.
///
/// Operand order in the `max` is load-bearing under D8: `max(a, b)` returns `b` on equal and on
/// unordered lanes, so `x_abs.max(t)` propagates a NaN in `t` and returns `t` when the two are
/// equal. Swapping it is a bit change.
#[inline(always)]
pub fn peak_follow<L: Lane>(x_abs: L, y: L, c: L) -> L {
    let d = y.sub(x_abs);
    let t = c.fma(d, x_abs);
    x_abs.max(t)
}

/// Attack and release coefficients of a switched one-pole follower, with their complements.
///
/// The complements are stored because `1 - c` is coefficient-only arithmetic that has no business
/// in a per-sample loop, and because it is **exact**: every follower in the workspace runs with a
/// retention coefficient in `[0.5, 1]`, where Sterbenz's lemma makes the subtraction exact, so
/// precomputing it moves no bits.
#[derive(Clone, Copy, Debug)]
pub struct ArCoef<L: Lane> {
    /// Retention coefficient while the detector is rising.
    pub attack: L,
    /// Retention coefficient while the detector is at or below the envelope.
    pub release: L,
    /// `1 - attack`, the weight the rising path gives the detector.
    pub one_minus_attack: L,
    /// `1 - release`, the weight the falling path gives the detector.
    pub one_minus_release: L,
}

impl<L: Lane> ArCoef<L> {
    /// Builds a coefficient set from a pair of *retention* coefficients
    /// ([`retention_coefficient`]), computing both complements once.
    ///
    /// `1 - c` is exact for `c` in `[0.5, 1]` and is a well-defined `f32` everywhere else, so this
    /// is a control-plane convenience and never a source of error.
    #[must_use]
    pub fn new(attack: L, release: L) -> Self {
        let one = L::splat(1.0);
        Self {
            attack,
            release,
            one_minus_attack: one.sub(attack),
            one_minus_release: one.sub(release),
        }
    }

    /// Builds a coefficient set from one scalar pair, broadcast to every lane.
    #[must_use]
    pub fn splat(attack: f32, release: f32) -> Self {
        Self::new(L::splat(attack), L::splat(release))
    }
}

/// One sample of a switched attack/release one-pole on a rectified detector.
///
/// `e' = flush(c * e + k * u)` with `c` and `k` selected by the direction of travel — the
/// **two-product** form, deliberately, and the counterpart of [`peak_follow`]'s one-rounding form.
/// The two exist side by side because they are not the same filter: [`peak_follow`] attacks
/// instantaneously and only its release is filtered, while this one filters both directions with
/// two different coefficients, which is what a dual-envelope transient detector needs.
///
/// The two-product form is also the numerically correct one *here*. Writing the release as
/// `e + k * (u - e)` (one rounding) stalls in `f32` when `|u - e| < ulp(e) / (2k)`: at the 100 ms
/// coefficient of a 96 kHz slow follower that is about `2.7e-4` relative, roughly 0.002 dB, and a
/// stalled slow envelope is a permanent contrast offset. With `1 - c` exact (see [`ArCoef`]) the
/// only error left is the rounding of `c` itself.
///
/// Frozen operation order, one rounding per line:
/// 1. `rising = u > e` — **strict**: a detector exactly at the envelope releases, which is the
///    convention of Giannoulis, Massberg and Reiss (JAES 2012) and of every follower in the
///    workspace
/// 2. `c = select(rising, attack, release)`, `k = select(rising, one_minus_attack,
///    one_minus_release)` — branchless (D10)
/// 3. `e' = flush(c * e + k * u)` — two rounded products, one rounded sum, then the D7 flush,
///    because `e` is a recurrence
#[inline(always)]
#[must_use]
pub fn ar_one_pole_step<L: Lane>(e: L, u: L, coefficients: &ArCoef<L>) -> L {
    let rising = u.gt(e);
    let c = L::select(rising, coefficients.attack, coefficients.release);
    let k = L::select(
        rising,
        coefficients.one_minus_attack,
        coefficients.one_minus_release,
    );
    miso_engine_lane::flush(c.mul(e).add(k.mul(u)))
}

/// One sample of a mean-square follower: `y' = fma(c, x2 - y, y)`.
///
/// Frozen operation order: `d = x2 - y`, then `y' = fma(c, d, y)` — one rounding. `x2` is the
/// squared input; the square root that turns the result into an RMS level belongs to the caller
/// and is IEEE-exact, so it is not part of the recurrence.
///
/// `c` here is the *rate* coefficient of [`attack_release_coefficient`]: `c = 0` freezes the
/// follower and `c = 1` makes it follow the input exactly.
#[inline(always)]
pub fn rms_follow<L: Lane>(x2: L, y: L, c: L) -> L {
    let d = x2.sub(y);
    c.fma(d, y)
}

/// Thresholds and hold time of a gate's open/close hysteresis, one set per lane.
#[derive(Clone, Copy)]
pub struct HysteresisCoef<L: Lane> {
    /// Level in dB at or above which the gate opens.
    pub open_db: L,
    /// Level in dB below which the gate may close, once the hold has expired. Always at or below
    /// `open_db`; the difference between the two is the hysteresis that stops a level sitting on
    /// the threshold from chattering.
    pub close_db: L,
    /// Hold time in samples, reloaded on every opening trigger.
    pub hold_samples: L,
}

/// Gate hysteresis state, one set per lane.
#[derive(Clone, Copy)]
pub struct HysteresisState<L: Lane> {
    /// `1.0` while the gate is open, `+0.0` while it is closed. Kept as a lane word rather than a
    /// mask so it survives a `Copy` across block boundaries without naming a backend mask type.
    pub open: L,
    /// Samples left on the hold countdown.
    pub hold: L,
}

impl<L: Lane> Default for HysteresisState<L> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            open: L::zero(),
            hold: L::zero(),
        }
    }
}

/// One sample of gate hysteresis; returns `1.0` for an open lane and `+0.0` for a closed one.
///
/// Branchless (D10: no data-dependent branch on a render path). Frozen operation order:
/// 1. `above = level_db > open_db`
/// 2. `below = level_db < close_db`
/// 3. `expired = hold <= 0` — tested on the countdown **as it stands at the top of the sample**,
///    before it is decremented, so a hold of `n` keeps the gate open for exactly `n` samples
/// 4. `decayed = max(hold - 1, 0)` — the countdown, clamped so it cannot run negative and wrap
///    back into a re-trigger
/// 5. `hold' = select(above, hold_samples, decayed)` — an opening trigger reloads the hold
/// 6. `close = below AND expired` — the gate may only close once the hold has run out
/// 7. `open' = select(close, 0, select(above, 1, open))` — opening wins over closing on a lane
///    where both fire, which cannot happen while `close_db <= open_db` but is pinned anyway
///
/// A level between `close_db` and `open_db` leaves the open state alone, which is the hysteresis;
/// `hold_samples = 0` closes on the first sample below `close_db`.
#[inline(always)]
pub fn hysteresis_step<L: Lane>(
    level_db: L,
    c: &HysteresisCoef<L>,
    s: &mut HysteresisState<L>,
) -> L {
    let one = L::splat(1.0);
    let zero = L::zero();
    let above = level_db.gt(c.open_db);
    let below = level_db.lt(c.close_db);
    let expired = s.hold.le(zero);
    let decayed = s.hold.sub(one).max(zero);
    let hold = L::select(above, c.hold_samples, decayed);
    let close = L::mask_and(below, expired);
    let open = L::select(close, zero, L::select(above, one, s.open));
    s.hold = hold;
    s.open = open;
    open
}
