//! Certified tail, exact-rest and composition arithmetic for native effects (decision 15
//! D15-4(b); issue #1375).
//!
//! Control-plane only: every function here runs at registry build, once per launch rate, in
//! integer and `f64` arithmetic. Nothing here is called on the render thread.
//!
//! # The rate-form follower
//!
//! The compressor's gain-reduction word and the true-peak limiter's reduction word are both the
//! rate form `e' = snap(fma(c, t - e, e))` ([`crate::envelope::rms_follow`]), with
//! [`lane::Lane::fma`] two roundings. With a zero target the step is
//! `e' = snap(fl(e - fl(c e)))`: `0 - e` is exact and `fl(c (-e)) = -fl(c e)`. The sign of `e` does
//! not matter (every operation is odd), so this module works with magnitudes.
//! [`rate_follower_rest_samples`] bounds how many such steps write `+0.0`.
//!
//! # Why the bound is computed, not a closed form
//!
//! The closed form `ln(E / floor) / (-ln(1 - c (1 - u)) - u)` charges every step the worst
//! rounding, `u = 2^-24` of the word. At a slow coefficient that is not small: at
//! `c = 1 / 480,000` (a 5,000 ms release at 96 kHz) `u / c` is 2.9 %, so the closed form states
//! about 2.9 % more samples than the word takes. The bound here walks the word's own grid
//! instead, grouped so that it costs a few thousand integer steps, and is within a few samples of
//! the real `f32` recurrence (gate 4 of #1375, `tests` below).

/// The `f32` unit roundoff, `2^-24`.
pub const ROUNDOFF: f64 = 1.0 / 16_777_216.0;

/// `2^-126`, the smallest normal `f32`: the absolute error one rounded `f32` operation can make by
/// underflowing (gradual underflow loses less, flush-to-zero at most this).
pub const UNDERFLOW: f64 = 1.0 / 85_070_591_730_234_615_865_843_651_857_942_052_864.0;

/// When the snap after a follower step writes `+0.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RestSnap {
    /// `|e'| <= floor` snaps: the compressor's `snap_reduction` (`le`).
    AtOrBelow(f32),
    /// `|e'| < floor` snaps: [`lane::flush`] at [`lane::FLUSH_EPS`] (`lt`).
    Below(f32),
}

/// A positive normal `f32` as `(n, x)` with value `n * 2^x` and `n` in `[2^23, 2^24)`.
fn decompose(value: f32) -> (u128, i32) {
    let bits = value.to_bits();
    let exponent = ((bits >> 23) & 0xff) as i32;
    (
        u128::from((bits & 0x007f_ffff) | 0x0080_0000),
        exponent - 150,
    )
}

/// `ceil(a / 2^shift)` for `a >= 0`.
fn ceil_shift(a: u128, shift: u32) -> u128 {
    (a + (1_u128 << shift) - 1) >> shift
}

/// Zero-target steps after which a rate-form follower word has been written `+0.0` by its snap,
/// for every start `|e| <= distance_max` and every coefficient sequence with `c >= rate_min`
/// (a ramp in flight included); `Some(0)` for `distance_max == 0`. `None` when the bound does not
/// exist: `rate_min` at or below `2^-24`, where the word can stall above its floor (the D3 stall
/// check of #1375), or an argument that is not a positive normal `f32`.
///
/// # Derivation
///
/// Write `f_c(e) = fl(e - fl(c e))` and `S` for the snap. Three facts:
///
/// 1. `f_c(e)` does not increase with `c`: `fl(c e)` is monotone in `c`.
/// 2. For `c <= 1/5`, `f_c(e)` does not decrease with `e`: for floats `e1 < e2`,
///    `fl(c e2) - fl(c e1) <= c (e2 - e1) + 2 u c e2 <= e2 - e1` (when `e2 <= 2 e1`,
///    `e2 - e1 >= ulp(e1) > u e2 / 2`; otherwise `e2 - e1 > e2 / 2`). `S` is monotone too.
/// 3. `fl(c e) >= c e (1 - u)`, so `f_c(e) <= F(e) = round_ties_up(e - c' e)`,
///    `c' = c (1 - u)`, rounded on the grid of `e` (or the next grid down when the difference
///    leaves the binade).
///
/// With `c = min(rate_min, 1/8)` and `b_0 = distance_max`, `b_{k+1} = S(F(b_k))`, induction on
/// 1-3 gives `|e_k| <= b_k` for every reachable start and coefficient sequence. The bound is the
/// first `k` with `b_k = 0`. `F` is evaluated exactly in integers: `c' = P / 2^Q`, and in a binade
/// `e = n 2^x` the step decrements `n` by `D(n) = ceil(c' n - 1/2)`. `D` is constant over runs of
/// `n`, so each run is one division. A run has `D >= 1` from `n = 2^23` on exactly when
/// `c' 2^23 > 1/2`, the scale-free stall condition.
#[must_use]
pub fn rate_follower_rest_samples(rate_min: f32, distance_max: f32, snap: RestSnap) -> Option<u64> {
    let floor = match snap {
        RestSnap::AtOrBelow(floor) | RestSnap::Below(floor) => floor,
    };
    if !rate_min.is_normal() || rate_min < 0.0 || !floor.is_normal() || floor < 0.0 {
        return None;
    }
    if distance_max == 0.0 {
        return Some(0);
    }
    if !distance_max.is_normal() || distance_max < 0.0 {
        return None;
    }
    let (cm, cx) = decompose(rate_min.min(0.125));
    // c' = cm (2^24 - 1) 2^(cx - 24) = P / 2^Q.
    let p = cm * ((1_u128 << 24) - 1);
    let q = u32::try_from(24 - cx).ok()?;
    let low = 1_u128 << 23;
    if p << 24 <= 1_u128 << q {
        return None;
    }
    let (nf, xf) = decompose(floor);
    let snapped = |n: u128, x: i32| match snap {
        RestSnap::AtOrBelow(_) => (x, n) <= (xf, nf),
        RestSnap::Below(_) => (x, n) < (xf, nf),
    };
    let (mut n, mut x) = decompose(distance_max);
    if snapped(n, x) {
        return Some(1);
    }
    let decrement = |n: u128| ceil_shift(2 * p * n - (1_u128 << q), q + 1);
    let mut steps = 0_u64;
    loop {
        let m = decrement(n);
        if n - m < low {
            // The difference leaves the binade: round it on the grid one binade down.
            n = 2 * n - ceil_shift(4 * p * n - (1_u128 << q), q + 1);
            x -= 1;
            steps += 1;
            if snapped(n, x) {
                return Some(steps);
            }
            continue;
        }
        // Every `n'` in `[lo, n]` has `D(n') = m` and stays in the binade.
        let first = (((2 * m - 1) << q) / (2 * p)) + 1;
        let lo = first.max(low + m);
        let run = (n - lo) / m + 1;
        if x == xf {
            let hit = match snap {
                RestSnap::AtOrBelow(_) => (n - nf).div_ceil(m),
                RestSnap::Below(_) => (n - nf) / m + 1,
            };
            if hit <= run {
                return Some(steps + u64::try_from(hit).ok()?);
            }
        }
        n -= run * m;
        steps += u64::try_from(run).ok()?;
    }
}

/// `ceil(2000 log10 g)`, the millibels of a positive finite linear value, rounded up.
///
/// `math::log` keeps every target on the same value. `x = 2000 ln(g) / ln 10` carries a few
/// `2^-53` relative errors, so the ceiling is taken of `x + |x| 2^-30`, plus `2^-30` mB when `g`
/// is itself the result of rounded operations (`computed`); an exact `g = 1` stays `0`. The same
/// rule as the builtin input section's (#1465 F-D3).
///
/// # Panics
///
/// When the millibels do not fit an `i32` (a `g` that is not positive and finite).
#[must_use]
pub fn ceil_millibels(g: f64, computed: bool) -> i32 {
    const MARGIN: f64 = 1.0 / 1_073_741_824.0;
    let x = 2000.0 * math::log(g) / core::f64::consts::LN_10;
    let absolute = if computed { MARGIN } else { 0.0 };
    let up = x + x.abs() * MARGIN + absolute;
    let ceiling = -math::floor(-up);
    assert!(
        ceiling.is_finite() && ceiling.abs() < f64::from(i32::MAX),
        "a millibel value of a finite positive gain"
    );
    ceiling as i32
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::envelope::rms_follow;

    /// The real word: `snap(fma(c, 0 - e, e))` at `f32`, counted from `e` until it is `+0.0`.
    fn brute_force(c: f32, e: f32, snap: RestSnap) -> u64 {
        let mut e = e;
        let mut steps = 0;
        while e != 0.0 {
            let next = rms_follow::<f32>(0.0, e, c);
            e = match snap {
                RestSnap::AtOrBelow(floor) if next.abs() <= floor => 0.0,
                RestSnap::Below(floor) if next.abs() < floor => 0.0,
                _ => next,
            };
            steps += 1;
        }
        steps
    }

    /// **#1375 gate 4: the bound is the real recurrence's count, from above and within
    /// `max(2, ceil(B / 100))`,** at both effects' slowest coefficients (the compressor's
    /// 5,000 ms and the limiter's 2,000 ms release at 44.1 and 96 kHz), at faster ones, and at
    /// both snap rules, from the start distance each effect states (`100` dB plus one ulp, `1.0`).
    #[test]
    fn the_bound_is_the_recurrence_count_from_above() {
        let rate = |ms: f64, fs: f64| (1.0 - math::exp(-1.0 / (0.001 * ms * fs))) as f32;
        let compressor = RestSnap::AtOrBelow(f32::from_bits(0x3510_7e90));
        let limiter = RestSnap::Below(lane::FLUSH_EPS);
        let cases = [
            (
                rate(5_000.0, 96_000.0),
                f32::from_bits(100.0_f32.to_bits() + 1),
                compressor,
            ),
            (
                rate(5_000.0, 44_100.0),
                f32::from_bits(100.0_f32.to_bits() + 1),
                compressor,
            ),
            (rate(100.0, 48_000.0), 37.5, compressor),
            (rate(5.0, 44_100.0), 100.0, compressor),
            (rate(2_000.0, 96_000.0), 1.0, limiter),
            (rate(2_000.0, 44_100.0), 1.0, limiter),
            (rate(10.0, 48_000.0), 1.0, limiter),
            (0.1, 1.0, limiter),
            (0.125, 3.0, compressor),
        ];
        for (c, e, snap) in cases {
            let bound = rate_follower_rest_samples(c, e, snap).expect("a bound");
            let count = brute_force(c, e, snap);
            std::eprintln!("c = {c:e}, e = {e}: brute force {count}, bound {bound}");
            assert!(
                count <= bound && bound <= count + 2.max(count.div_ceil(100)),
                "c = {c:e}, e = {e}: brute force {count}, bound {bound}"
            );
        }
    }

    /// D3: at and below `2^-24` the word can stall above its floor, and no bound is stated.
    #[test]
    fn a_coefficient_that_can_stall_states_no_bound() {
        let stall = f32::from_bits(0x3380_0000); // 2^-24
        let snap = RestSnap::Below(lane::FLUSH_EPS);
        assert_eq!(rate_follower_rest_samples(stall, 1.0, snap), None);
        let above = f32::from_bits(stall.to_bits() + 1);
        assert!(rate_follower_rest_samples(above, 1.0, snap).is_some());
    }
}
