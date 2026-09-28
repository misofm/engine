//! The dynamics gain computer: a soft-knee static curve in the dB domain, branchless.
//!
//! # The curve
//!
//! Dimitrios Giannoulis, Michael Massberg and Joshua D. Reiss, *Digital Dynamic Range Compressor
//! Design — A Tutorial and Analysis*, Journal of the Audio Engineering Society 60(6), 2012,
//! equation 4. With input level `x` in dB, threshold `T` in dB, ratio `R` and knee width `W` in
//! dB, the output level `y` in dB is
//!
//! ```text
//!            / x                                        if 2 (x - T) < -W
//!       y = <  x + (1/R - 1) (x - T + W/2)^2 / (2 W)     if |2 (x - T)| <= W
//!            \ T + (x - T) / R                           if 2 (x - T) > W
//! ```
//!
//! The middle arm is the quadratic knee that joins the two straight arms with a continuous first
//! derivative: at `x = T - W/2` its value and slope are `x` and `1`, at `x = T + W/2` they are the
//! above-threshold line and `1/R`. `W = 0` collapses it to a hard knee at `T`.
//!
//! # A knee too narrow to design is a hard knee
//!
//! The knee arm is scaled by `1 / (2 W)`, designed once per parameter change. For a positive knee
//! narrower than [`MIN_SOFT_KNEE_DB`] (`2^-129 + 2^-149` dB, about `1.47e-39`) that reciprocal
//! overflows `f32` to `+inf`, and every level inside the knee then evaluates `(v * v) * inf` with
//! `v * v` underflowed to zero: a NaN exactly where the curve should be at its most ordinary
//! (issue #994). Such a knee is therefore designed as the hard knee, which is the limit the
//! soft curve converges to: the two differ by at most `|1/R - 1| W / 8` dB, reached at `x = T`,
//! which for `W <= 2^-129` and any ratio of at least 1 is below `2^-132` dB, far below one ulp of
//! any gain it becomes. Every knee whose reciprocal is finite is designed exactly as before, word
//! for word; see [`knee_coefficients`] for the derivation of the bound and of the finiteness of
//! the knee arm.
//!
//! # How it is evaluated
//!
//! All three arms are evaluated and one is selected per lane (D10: no data-dependent branch on a
//! render path). The comparisons are against `W/2`, not `2 (x - T)` against `W`: halving is exact
//! in binary floating point, so the two are the same predicate, and the form with fewer operations
//! is used.
//!
//! The dB conversions use the lane-wide `exp2`/`log2` of `math` (D6), which are built
//! from `Lane` basic operations only and are therefore bit-identical on every target and at every
//! width.

use lane::Lane;
use math::{exp2_lane, log2_lane};

/// `20 * log10(2)`: dB per octave of amplitude. Rounded once, from the `f64` constant.
const DB_PER_LOG2: f32 = (20.0_f64 * core::f64::consts::LOG10_2) as f32;

/// `log2(10) / 20`: the inverse of [`DB_PER_LOG2`]. Rounded once, from the `f64` constant.
const LOG2_PER_DB: f32 = (core::f64::consts::LOG2_10 / 20.0) as f32;

/// The narrowest knee width, in dB, that [`knee_coefficients`] designs as a soft knee.
///
/// `2^-129 + 2^-149`, the subnormal `f32::from_bits(0x0010_0001)`, about `1.469e-39`. Every
/// positive width below it — exactly the widths in `(0, 2^-129]` — has an `f32` reciprocal
/// `1 / (2 W)` that overflows to `+inf`, and is designed as a hard knee instead. The derivation is
/// on [`knee_coefficients`]; `tests/dynamics.rs` pins it on both sides.
///
/// This is documentation of a property of `f32` division, not a second guard: the design decides
/// from the reciprocal it actually computed, so the rule holds under any floating-point
/// environment, including one that flushes subnormal results to zero.
pub const MIN_SOFT_KNEE_DB: f32 = f32::from_bits(0x0010_0001);

/// The knee words `(W/2, 1/(2 W))` of a knee width `W` in dB, or `(+0.0, +0.0)` — the hard knee.
///
/// The hard knee is designed for `knee_db <= 0`, for a NaN, **and for a positive width whose
/// reciprocal `1 / (2 W)` is not finite** (issue #994). Every other width is designed exactly as
/// `(0.5 * W, 1.0 / (2.0 * W))`, so for every knee whose reciprocal is finite the words, the curve
/// and every output bit are what they were before the overflow rule existed.
///
/// # Which widths overflow
///
/// `2 W` is exact for every finite `W <= 24` (doubling does not round in binary floating point,
/// subnormals included). Under round-to-nearest-even the quotient `1 / (2 W)` rounds to `+inf`
/// exactly when it is at least `f32::MAX` plus half its ulp, `2^128 - 2^103`: the tie goes to the
/// even significand, which is the overflow. So the reciprocal overflows exactly when
/// `2 W <= 2^-128 / (1 - 2^-25)`. That bound lies strictly between `2^-128` and the next `f32`
/// above it, `2^-128 + 2^-149`, so for an `f32` width the condition is `2 W <= 2^-128`, that is
/// `W <= 2^-129`. The smallest width with a finite reciprocal is the next `f32`,
/// `2^-129 + 2^-149` ([`MIN_SOFT_KNEE_DB`]). The decision below tests the computed reciprocal
/// rather than comparing against that constant, which is the same predicate under IEEE
/// arithmetic and stays right in a flush-to-zero environment where `2 W` itself could flush.
///
/// # Why a finite reciprocal is enough
///
/// [`gain_delta_db`] selects the knee arm only for `-(W/2) < d <= W/2`, where `v = d + W/2` is
/// positive and at most `2 * (W/2)`. With a finite `1 / (2 W)` the arm `(v * v) * (1 / (2 W))` is
/// then at most `W/2` to within three roundings, plus at most `2^-150 / (2 W) < 2^-22` when `v * v`
/// rounds in the subnormal range: finite for every width up to 24 dB, whatever the level. It is
/// scaled by `1/R - 1`, which is finite for every positive finite ratio. The two straight arms do
/// not involve `W` beyond the exact comparison against `W/2`.
#[must_use]
pub const fn knee_coefficients(knee_db: f32) -> (f32, f32) {
    if knee_db > 0.0 {
        let inv_two_knee = 1.0 / (2.0 * knee_db);
        if inv_two_knee.is_finite() {
            return (0.5 * knee_db, inv_two_knee);
        }
    }
    (0.0, 0.0)
}

/// Static-curve coefficients, one set per lane.
///
/// Built on the control plane by [`GainComputerCoef::new`] from threshold, ratio and knee width;
/// the render path never sees `R` or `W` themselves.
#[derive(Clone, Copy)]
pub struct GainComputerCoef<L: Lane> {
    /// Threshold `T` in dB.
    pub threshold_db: L,
    /// `1/R - 1`, the slope change above the threshold. Zero for `R = 1` (no compression),
    /// negative for a compressor, positive for an upward expander.
    pub inv_ratio_minus_one: L,
    /// `W/2`, the half knee width in dB. Exact: halving does not round.
    pub half_knee_db: L,
    /// `1 / (2 W)`, the quadratic knee's scale — **`+0.0` for a hard knee**, so that the hard-knee
    /// case cannot produce `0 * inf` at exactly the threshold. A hard knee is `W <= 0` or a width
    /// narrower than [`MIN_SOFT_KNEE_DB`], whose reciprocal would overflow ([`knee_coefficients`]).
    pub inv_two_knee: L,
}

impl<L: Lane> GainComputerCoef<L> {
    /// Designs the curve from a threshold in dB, a ratio and a knee width in dB.
    ///
    /// One scalar design shared by every lane. `knee_db <= 0` gives a hard knee: `half_knee_db`
    /// and `inv_two_knee` are both `+0.0`, the knee arm reduces to `x` at exactly `x = T` and the
    /// two straight arms meet there. So does a positive knee narrower than [`MIN_SOFT_KNEE_DB`],
    /// whose `1 / (2 W)` overflows: the knee words are [`knee_coefficients`]'s.
    ///
    /// `1 / (2 W)` is the only division, and it happens here, at parameter-change time.
    #[must_use]
    pub fn new(threshold_db: f32, ratio: f32, knee_db: f32) -> Self {
        let (half_knee, inv_two_knee) = knee_coefficients(knee_db);
        Self {
            threshold_db: L::splat(threshold_db),
            inv_ratio_minus_one: L::splat(1.0 / ratio - 1.0),
            half_knee_db: L::splat(half_knee),
            inv_two_knee: L::splat(inv_two_knee),
        }
    }
}

/// The static curve of equation 4: input level in dB to output level in dB.
///
/// Frozen operation order:
/// 1. `d = x_db - T`
/// 2. `over = d > W/2`, `under = d <= -(W/2)` — `-(W/2)` is a sign-bit flip, exact. The paper
///    writes the identity arm as the strict `2 (x - T) < -W`; at equality both arms are `x`
///    analytically, and the non-strict form is used so that the lower knee edge is the **exact**
///    identity rather than `x + (-0.0)`. `over` stays strict, which is what makes a hard knee
///    (`W = 0`) give exactly `x` at `x = T`, as the paper requires.
/// 3. `above = d * (1/R - 1)` — the above-threshold arm, as a *delta*: `T + d/R = x + d (1/R - 1)`
/// 4. `v = d + W/2`; `knee = ((v * v) * inv_two_knee) * (1/R - 1)` — the knee arm, also a delta
/// 5. `delta = select(under, 0, select(over, above, knee))` — both knee branches evaluated
/// 6. `y = x_db + delta`
///
/// Writing both arms as a delta added to `x_db` rather than as absolute levels is what makes the
/// below-threshold arm exactly `x_db` and the knee exactly continuous at `x = T - W/2`, where the
/// delta is exactly `+0.0` regardless of the ratio.
///
/// A NaN input fails both ordered compares, takes the knee arm and stays NaN — it is never
/// silently turned into a level. The once-per-block boundary check of [`crate::bank`] is what
/// catches it.
#[inline(always)]
pub fn gain_computer_db<L: Lane>(x_db: L, c: &GainComputerCoef<L>) -> L {
    x_db.add(gain_delta_db(x_db, c))
}

/// The static curve expressed as the change it applies: `gain_computer_db(x, c) - x`.
///
/// This is what a compressor's smoother actually tracks — the gain reduction in dB, `<= 0` for a
/// downward compressor — so it is exposed directly rather than recovered by a subtraction that
/// would round a second time. Same frozen operation order as [`gain_computer_db`], steps 1 to 5.
#[inline(always)]
pub fn gain_delta_db<L: Lane>(x_db: L, c: &GainComputerCoef<L>) -> L {
    let d = x_db.sub(c.threshold_db);
    let over = d.gt(c.half_knee_db);
    let under = d.le(c.half_knee_db.neg());
    let above = d.mul(c.inv_ratio_minus_one);
    let v = d.add(c.half_knee_db);
    let knee = v.mul(v).mul(c.inv_two_knee).mul(c.inv_ratio_minus_one);
    L::select(under, L::zero(), L::select(over, above, knee))
}

/// Amplitude to level: `20 * log10(|x|)`, lane-wide.
///
/// `log2_lane` clamps its input up to `f32::MIN_POSITIVE` before the range reduction, so silence
/// gives the finite floor `-126 * 20 * log10(2)`, about `-758.6` dB, rather than `-inf`. Callers
/// pass an already-non-negative detector level.
///
/// Frozen operation order: `log2_lane(x_abs) * DB_PER_LOG2`.
#[inline(always)]
pub fn level_db<L: Lane>(x_abs: L) -> L {
    log2_lane(x_abs).mul(L::splat(DB_PER_LOG2))
}

/// Level to amplitude: `10^(db / 20)`, lane-wide.
///
/// Frozen operation order: `exp2_lane(db * LOG2_PER_DB)`. `exp2_lane` clamps its argument to
/// `[-126, 127]`, so the result is always a finite positive gain and never a NaN payload (D5).
///
/// `gain_from_db(0.0)` is exactly `1.0`: `0 * LOG2_PER_DB` is `+0.0` and `exp2_lane(0)` is pinned
/// exact by gate M1, so a bypassed or unity stage is a true identity.
#[inline(always)]
pub fn gain_from_db<L: Lane>(db: L) -> L {
    exp2_lane(db.mul(L::splat(LOG2_PER_DB)))
}
