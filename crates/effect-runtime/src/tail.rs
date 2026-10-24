//! Certified exact-rest arithmetic for native effects (decision 15
//! D15-4(b); issues #1375 and #1376).
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
//! # The pole-form follower
//!
//! The transient shaper's two envelope words are the two-product form
//! [`crate::envelope::ar_one_pole_step`], which with a zero detector is `e' = snap(fl(c e))`, `c`
//! the release *retention* (issue #1376). [`pole_follower_rest_samples`] bounds it with the same
//! run-grouped integer walk, the decrement weight `1 - c` exact instead of inflated.
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

/// An exact coefficient `P / 2^Q` of an integer walk: the decrement weight of one step.
#[derive(Clone, Copy, Debug)]
struct Weight {
    p: u128,
    q: u32,
}

impl Weight {
    /// `max(0, ceil(P n / 2^Q - half / 2))`: the decrement of one step of the integer walk at `n`
    /// when the step's difference is rounded on the unit grid with an allowance of `half / 2`
    /// unit (`half = 1`: round to nearest, ties up; `half = 2`: a whole unit).
    fn decrement(self, n: u128, half: u128) -> u128 {
        (2 * self.p * n)
            .checked_sub(half << self.q)
            .map_or(0, |a| ceil_shift(a, self.q + 1))
    }

    /// The smallest `n` with [`decrement`](Self::decrement)`(n, half) == m`, for `m >= 1`: every
    /// `n' >= first(m)` up to the next run's first has the same decrement, so a run of equal
    /// decrements is one division.
    fn first(self, m: u128, half: u128) -> u128 {
        (((2 * m - 2 + half) << self.q) / (2 * self.p)) + 1
    }

    /// Whether the walk's step is at least one unit from `n = 2^23` on (the binade floor): the
    /// scale-free D3 condition `P 2^23 / 2^Q > 1/2`.
    fn moves_at_binade_floor(self) -> bool {
        self.p << 24 > 1_u128 << self.q
    }
}

/// The shared run-grouped walk of [`rate_follower_rest_samples`] and
/// [`pole_follower_rest_samples`]: a word of magnitude `n 2^x` steps to
/// `round_ties_up(n - (P / 2^Q) n)` on its own grid (or the grid one binade down when the
/// difference leaves the binade), and the walk counts steps until `snap` writes `+0.0`.
fn magnitude_walk(weight: Weight, distance_max: f32, snap: RestSnap) -> Option<u64> {
    let floor = match snap {
        RestSnap::AtOrBelow(floor) | RestSnap::Below(floor) => floor,
    };
    if !floor.is_normal() || floor < 0.0 {
        return None;
    }
    if distance_max == 0.0 {
        return Some(0);
    }
    if !distance_max.is_normal() || distance_max < 0.0 || !weight.moves_at_binade_floor() {
        return None;
    }
    let (p, q) = (weight.p, weight.q);
    let low = 1_u128 << 23;
    let (nf, xf) = decompose(floor);
    let snapped = |n: u128, x: i32| match snap {
        RestSnap::AtOrBelow(_) => (x, n) <= (xf, nf),
        RestSnap::Below(_) => (x, n) < (xf, nf),
    };
    let (mut n, mut x) = decompose(distance_max);
    if snapped(n, x) {
        return Some(1);
    }
    let mut steps = 0_u64;
    loop {
        let m = weight.decrement(n, 1);
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
        let lo = weight.first(m, 1).max(low + m);
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
    if !rate_min.is_normal() || rate_min < 0.0 {
        return None;
    }
    let (cm, cx) = decompose(rate_min.min(0.125));
    // c' = cm (2^24 - 1) 2^(cx - 24) = P / 2^Q.
    let weight = Weight {
        p: cm * ((1_u128 << 24) - 1),
        q: u32::try_from(24 - cx).ok()?,
    };
    magnitude_walk(weight, distance_max, snap)
}

/// Zero-input steps after which a two-product one-pole word `e' = snap(fl(c e))` has been
/// written `+0.0`, for every start `|e| <= distance_max` and every retention `c <= retention_max`;
/// `Some(0)` for `distance_max == 0`. `None` when the bound does not exist: a retention at or
/// above `1 - 2^-24`, where `fl(c e) = e` is a fixed point above every floor (the D3 stall check of
/// #1375 for the two-product form), or an argument that is not a positive normal `f32` below `1`.
///
/// This is [`crate::envelope::ar_one_pole_step`] with a zero detector: the attack arm needs
/// `u > e`, so the release arm runs, and `fl(fl(c e) + fl(k 0)) = fl(c e)` (`k 0 = +0.0`, and
/// adding `+0.0` is exact). One rounding per step.
///
/// # Derivation
///
/// `fl(c e)` does not decrease with `e` or with `c` (one monotone rounding), and the snap is
/// monotone. For `c` in `[1/2, 1)`, `1 - c` is exact (Sterbenz), so the step is exactly
/// `fl(e - (1 - c) e)` with `(1 - c) = P / 2^24`, `P` an integer: the same integer walk as
/// [`rate_follower_rest_samples`] with `c' = 1 - c` exactly, without its `(1 - u)` inflation, and
/// with the same ties-up rounding as a majorant of ties-to-even. The retention is raised to at
/// least `7/8` (so `c' <= 1/8`): by monotonicity in `c` a larger retention only lengthens the
/// walk, so the bound stays a bound.
#[must_use]
pub fn pole_follower_rest_samples(
    retention_max: f32,
    distance_max: f32,
    snap: RestSnap,
) -> Option<u64> {
    if !retention_max.is_normal() || !(0.0..1.0).contains(&retention_max) {
        return None;
    }
    let (cm, cx) = decompose(retention_max.max(0.875));
    debug_assert_eq!(cx, -24, "a retention in [1/2, 1)");
    let weight = Weight {
        p: (1_u128 << 24) - cm,
        q: 24,
    };
    magnitude_walk(weight, distance_max, snap)
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

    /// **The bound is the real recurrence's count, from above and within `max(2, ceil(B / 100))`,**
    /// at faster coefficients than the effects' slowest (which their own gate 4 checks, in
    /// `tests/tail_contract.rs`), up to the `1/8` clamp, and at both snap rules.
    #[test]
    fn the_bound_is_the_recurrence_count_from_above() {
        let rate = |ms: f64, fs: f64| (1.0 - math::exp(-1.0 / (0.001 * ms * fs))) as f32;
        let compressor = RestSnap::AtOrBelow(f32::from_bits(0x3510_7e90));
        let limiter = RestSnap::Below(lane::FLUSH_EPS);
        let cases = [
            (rate(100.0, 48_000.0), 37.5, compressor),
            (rate(5.0, 44_100.0), 100.0, compressor),
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

    /// The real pole word: `snap(fl(c e))` at `f32`, counted from `e` until it is `+0.0`.
    fn pole_brute_force(c: f32, e: f32, floor: f32) -> u64 {
        let mut e = e;
        let mut steps = 0;
        while e != 0.0 {
            let next = c * e;
            e = if next.abs() < floor { 0.0 } else { next };
            steps += 1;
        }
        steps
    }

    /// **The pole bound is the real recurrence's count, from above and within
    /// `max(2, ceil(B / 100))`,** at retentions faster than the transient shaper's slowest (which
    /// its gate 4 checks), down to the `7/8` clamp, at the shaper's `1e-12` floor and at
    /// `FLUSH_EPS`.
    #[test]
    fn the_pole_bound_is_the_recurrence_count_from_above() {
        let retention = |ms: f64, fs: f64| math::exp(-1.0 / (0.001 * ms * fs)) as f32;
        let cases = [
            (retention(20.0, 48_000.0), 16.0, 1.0e-12),
            (retention(5.0, 44_100.0), 1.0e29, 1.0e-12),
            (retention(10.0, 96_000.0), 1.0, lane::FLUSH_EPS),
            (0.875, 3.0, 1.0e-12),
        ];
        for (c, e, floor) in cases {
            let bound = pole_follower_rest_samples(c, e, RestSnap::Below(floor)).expect("a bound");
            let count = pole_brute_force(c, e, floor);
            std::eprintln!("c = {c}, e = {e:e}: brute force {count}, bound {bound}");
            assert!(
                count <= bound && bound <= count + 2.max(count.div_ceil(100)),
                "c = {c}, e = {e:e}: brute force {count}, bound {bound}"
            );
        }
    }

    /// D3 for the pole form: at `1 - 2^-24` the word is a fixed point, and no bound is stated.
    #[test]
    fn a_retention_that_can_stall_states_no_bound() {
        let stall = f32::from_bits(1.0_f32.to_bits() - 1); // 1 - 2^-24
        let snap = RestSnap::Below(1.0e-12);
        assert_eq!(pole_follower_rest_samples(stall, 1.0, snap), None);
        let below = f32::from_bits(stall.to_bits() - 1);
        assert!(pole_follower_rest_samples(below, 1.0, snap).is_some());
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
