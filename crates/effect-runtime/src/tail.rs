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

/// One phase of a fixed-unit integer walk: from `b`, `b' = b - D(b)` with
/// `D(b) = `[`Weight::decrement`]`(b, half)`, until `b <= stop` or `D(b) = 0` (a fixed point).
/// Returns the steps taken and the last `b`; `None` only if the count overflows.
fn fixed_walk(weight: Weight, start: u128, half: u128, stop: u128) -> Option<(u64, u128)> {
    let mut b = start;
    let mut steps = 0_u64;
    while b > stop {
        let m = weight.decrement(b, half);
        if m == 0 {
            break;
        }
        // Every `n` in `[first(m), b]` has the decrement `m` (it does not decrease with `n`).
        let run = (b - weight.first(m, half)) / m + 1;
        let need = (b - stop).div_ceil(m);
        if need <= run {
            return Some((steps.checked_add(u64::try_from(need).ok()?)?, b - need * m));
        }
        b -= run * m;
        steps = steps.checked_add(u64::try_from(run).ok()?)?;
    }
    Some((steps, b))
}

/// A walk of two phases: the whole-unit allowance (`half = 2`) until `b <= switch`, then the
/// half-unit one (`half = 1`) until `b <= stop` or a fixed point. `None` if the first phase stops
/// above `switch`.
fn phased_walk(weight: Weight, start: u128, switch: u128, stop: u128) -> Option<(u64, u128)> {
    let (first, b) = fixed_walk(weight, start, 2, switch)?;
    if b > switch {
        return None;
    }
    let (second, b) = fixed_walk(weight, b, 1, stop)?;
    Some((first.checked_add(second)?, b))
}

/// `c (1 - u)^2` of a positive normal `f32` as an exact weight: the least decrement weight of a
/// rate-form step `fl(c fl(t - e))` relative to the distance `t - e`.
fn inflated_weight(c: f32) -> Option<Weight> {
    let (cm, cx) = decompose(c);
    let s = (1_u128 << 24) - 1;
    Some(Weight {
        p: cm * s * s,
        q: u32::try_from(48 - cx).ok()?,
    })
}

/// The largest integer distance `d`, in units of the word's grid, with `c d (1 + u)^2 < 1/2` for
/// every coefficient `c <= hi`: a word that close to its target cannot move (its step is under half
/// a unit, so the sum rounds back to the word).
fn sure_stall_distance(hi: f32) -> Option<u128> {
    let (hm, hx) = decompose(hi);
    let s = (1_u128 << 24) + 1;
    let shift = u32::try_from(47 - hx).ok()?;
    if shift >= 127 {
        return None;
    }
    Some(((1_u128 << shift) - 1) / (hm * s * s))
}

/// The bound of one coefficient bracket `[lo, hi]` of [`rate_follower_stall_samples`]: the
/// maximum over the three target classes, B, M and A, of the derivation.
fn stall_bracket(lo: f32, hi: f32) -> Option<u64> {
    const BINADE: u128 = 1 << 23;
    const TOP: u128 = 3 << 21;
    let weight = inflated_weight(lo)?;
    let sure = sure_stall_distance(hi)?;
    let after = |b: u128| u64::try_from(b.saturating_sub(sure)).ok();
    // B: L <= sure, in units of the half grid.
    let (steps, b) = phased_walk(weight, 2 * BINADE + 2 * sure, BINADE, 0)?;
    let mut worst = steps.checked_add(after(b)?)?.checked_add(1)?;
    // A: L >= TOP, in units of the grid of `t`.
    let (steps, b) = phased_walk(weight, 2 * BINADE - 1, TOP, 0)?;
    worst = worst.max(steps.checked_add(after(b)?)?);
    // M: L in [low, high], ranges covering (sure, TOP). A range whose bound is under the maximum so
    // far is done; a wider one is split at its geometric middle, down to ranges of 1 %.
    let middle = |low: u128, high: u128| -> Option<u64> {
        let (fine, b) = phased_walk(weight, 2 * BINADE + 2 * high, BINADE + 2 * low, 2 * low)?;
        if b > 2 * low {
            return None;
        }
        let (coarse, b) = fixed_walk(weight, high, 1, 0)?;
        fine.checked_add(coarse)?.checked_add(after(b)?)
    };
    let mut ranges = [(0_u128, 0_u128); 64];
    let mut pending = 0;
    if sure + 1 < TOP {
        ranges[0] = (sure + 1, TOP - 1);
        pending = 1;
    }
    while pending > 0 {
        pending -= 1;
        let (low, high) = ranges[pending];
        let bound = middle(low, high)?;
        if bound <= worst || high - low <= low / 100 || pending + 2 > ranges.len() {
            worst = worst.max(bound);
            continue;
        }
        let split = math::sqrt((low as f64) * (high as f64)) as u128;
        let split = split.clamp(low, high - 1);
        ranges[pending] = (low, split);
        ranges[pending + 1] = (split + 1, high);
        pending += 2;
    }
    Some(worst)
}

/// Steps after which a rate-form word `e' = flush(fma(c, t - e, e))` walking toward a fixed
/// non-zero target `t` from above (`t <= e <= 0` or its mirror) has stopped moving, for every target
/// `t` and every coefficient `c` in `[rate_min, 1/8]`; `None` when the derivation does not cover
/// `rate_min` (below `2^-20` or above `1/8`, or not a normal `f32`).
///
/// This is the gate's gain word after its hold expires (issue #1376 D2). It never reaches `t`: it
/// stops at the first word where `fl(c fl(t - e))` is under half a grid unit, the **stall word**
/// `w*`. With the walk measured from `t` in units of a grid, everything below is scale-free, so the
/// bound holds for every `t` whose grid products stay normal (and a target so small that the first
/// step flushes to `+0.0` stalls at once).
///
/// # The walk
///
/// Let `2^k <= |t| < 2^(k+1)`, `g = ulp(t)`, `h = g / 2`, and `L = (|t| - 2^k) / g`, an integer in
/// `[0, 2^23)`. Words with `|e| >= 2^k` (distance `d = |t| - |e| <= L g`) are *coarse*, on the
/// grid `g`; words in the binade below are *fine*, on `h`; lower ones are finer still. A step
/// moves `e` toward `t` and never past it, and `|e|` only grows, so the walk visits coarser grids
/// as it goes. One step: `d' = round(d - |p|)` on the grid of the new word, with
/// `|p| = |fl(c fl(t - e))| >= c'' d`, `c'' = c (1 - u)^2`. In units `U` of a grid at least as fine
/// as the new word's, with `x = d (1 - c'')`:
///
/// * if the new word is on the grid `U`, `d' <= floor(x + 1/2)` (round to nearest, ties up: a
///   majorant of ties-to-even);
/// * on any grid of at most `2 U`, `d' <= floor(x + 1)` (a multiple of a grid unit `q <= 2 U`
///   within `q / 2` of a value at most `x`).
///
/// Both right sides are `b - D(b)` with `D(b) = ceil(c'' b - half / 2)`, nondecreasing in `b`, so
/// a majorant `b` that takes the rule valid for every grid the real word can be on dominates the
/// distance step by step (`d_j <= b_j`); the walk is evaluated in runs of equal `D`.
///
/// When the majorant reaches its fixed point `b*`, the real distance is at most `b*` but the word
/// may still move. A word at distance `d <= S`, `S` the largest integer with
/// `c d (1 + u)^2 < 1/2` for every `c` of the bracket, cannot move. Each
/// moving step moves at least one unit, so at most `b* - S` more steps follow.
///
/// # The three target classes, per coefficient bracket `[lo, hi]`
///
/// The walk uses `lo` for the positions and `hi` for `S`, so it covers every `c` in the bracket.
///
/// * **B, `L <= S`.** Every coarse word is a stall. In units of `h` the start is
///   `2^24 + 2 L <= 2^24 + 2 S`; the whole-unit rule holds while the word may be finer than `h`
///   (`d > 2^23` units), then the half-unit rule while it is fine. A fine-to-coarse step is not
///   dominated, but it lands on a stall: one step more. Bound: walk + `(b* - S)` + 1.
/// * **M, `S < L < 3 2^21`, in ranges `[La, Lb]`.** While the word is fine it is
///   dominated in units of `h` from `2^24 + 2 Lb` (whole-unit rule above `2^23 + 2 La`, half-unit
///   below); a fine word has `d > 2 L >= 2 La`, so it is coarse by the first step `J` with
///   `b_J <= 2 La`. From there it is coarse at `d <= Lb` in units of `g`, dominated by the
///   half-unit walk from `Lb` to its fixed point. Bound: `J` + coarse walk + `(b* - S)`. A
///   range costs about `ln(Lb / La) / c`; one whose bound is under B's and A's is done, a wider one
///   is split (M's walks start lower than A's, so they need little splitting).
/// * **A, `L >= 3 2^21`.** In units of `g` the start is `|t| / g < 2^24`; the whole-unit rule above
///   `3 2^21`, below which every word the walk can hold is coarse. Bound: walk + `(b* - S)`.
///
/// Brackets of ratio `1.02` start at `rate_min`. A coarse bound covers every `c >= lo` at once:
/// the whole-unit walk at `lo` from `2^24` units of `g` to its fixed point `b*`, then at most
/// `2 b* + 1` moving steps, one per word within `b* g` of `t` (those words are on the grids `g` and
/// `h`). It falls about as `1 / c`, so the brackets stop at the first `lo` whose coarse bound is
/// under their maximum (about `1.16 rate_min`), or at `2 rate_min` with the coarse bound included.
///
/// The ranges and brackets cost their width: a few thousand samples above the worst real walk at
/// the gate's slowest release. The class that attains the maximum is B at the slowest bracket (a
/// target just above a power of two, whose walk ends on the fine grid), with A within about 0.2 %.
///
/// # Uniqueness of the stall word
///
/// Every walk from a start in `[w*, 0]` toward the same `t` with the same `c` ends on the same
/// word. The successor of a word depends on the word alone. In the last stretch, where the step
/// is `|p| < 3/2` units of the word's grid (`d < 3 / (2c)` units), a moving word goes to the
/// adjacent float toward `t` (at the fine-to-coarse boundary too: the next float is one fine unit
/// away), so a walk there visits consecutive words and stops at the first word that cannot move.
/// A step from farther out (`|p| >= 3/2` units) lands no closer than
/// `d (1 - c (1 + u)^2) - 1/2 >= 3 / (2c) - 2` units, still inside the last stretch's moving words
/// (a word moves while `c d (1 - u)^2 >= 1/2`, `d >= 1 / (2c)` about). Every walk therefore
/// enters the last stretch above its first stall word and walks it word by word: they all stop at
/// the same one, and it is the walk from `+0.0`'s end, `w*`. A start between `w*` and `t` (only a
/// parameter edit leaves one there) is past that word and stops at the first fixed point below it.
#[must_use]
pub fn rate_follower_stall_samples(rate_min: f32) -> Option<u64> {
    const FLOOR: f32 = 1.0 / 1_048_576.0;
    if !rate_min.is_normal() || !(FLOOR..=0.125).contains(&rate_min) {
        return None;
    }
    let ceiling = 2.0 * f64::from(rate_min);
    let mut lo = rate_min;
    let mut worst = 0_u64;
    loop {
        // The coarse bound holds for every coefficient at or above `lo`: once it is under the
        // brackets' maximum, no faster coefficient can raise it.
        let crude = coarse_stall_bound(lo)?;
        if crude <= worst {
            return Some(worst);
        }
        if f64::from(lo) >= ceiling {
            return Some(worst.max(crude));
        }
        let target = f64::from(lo) * 1.02;
        let mut hi = target as f32;
        if f64::from(hi) < target {
            hi = f32::from_bits(hi.to_bits() + 1);
        }
        worst = worst.max(stall_bracket(lo, hi)?);
        lo = hi;
    }
}

/// The coarse bound of [`rate_follower_stall_samples`] for every coefficient `c >= lo`: the
/// whole-unit walk at `lo` from `2^24` units of `g` to its fixed point `b*`, then one step per word
/// within `b* g` of the target, at most `2 b* + 1`.
fn coarse_stall_bound(lo: f32) -> Option<u64> {
    let (steps, b) = fixed_walk(inflated_weight(lo)?, 1 << 24, 2, 0)?;
    steps.checked_add(u64::try_from(2 * b + 1).ok()?)
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

    /// The real gate word toward a non-zero target: `flush(fma(c, t - e, e))` at `f32` (two
    /// roundings), from `e` until a step leaves it unchanged. Returns the steps and the word.
    fn stall_brute_force(c: f32, t: f32, e: f32) -> (u64, f32) {
        let mut e = e;
        let mut steps = 0;
        loop {
            let next = c * (t - e) + e;
            let next = if next.abs() < lane::FLUSH_EPS {
                0.0
            } else {
                next
            };
            if next == e {
                return (steps, e);
            }
            e = next;
            steps += 1;
        }
    }

    /// **The stall bound covers every target and every start ends on one word**, at coefficients
    /// faster than the gate's slowest (its gate 4 checks that one): over targets
    /// `t = -(2^k + L ulp(t))` for `L` across the three classes of the derivation (a target on a
    /// power of two, just above one by fewer or more than the sure-stall distance, mid-binade and
    /// just below the next power), the walk from `+0.0` stops within the bound, the largest walk
    /// is within `max(2, ceil(B / 100))` of it, and walks from two other starts above the stall
    /// word stop on the same word.
    #[test]
    fn the_stall_bound_covers_every_target_and_the_stall_word_is_unique() {
        for c in [1.0e-3_f32, 2.0e-4] {
            let bound = rate_follower_stall_samples(c).expect("a bound");
            let sure = sure_stall_distance(c).expect("a distance");
            let mut worst = 0;
            let mut ls: std::vec::Vec<u32> = std::vec![0, 1, 2, 3, (1 << 23) - 1, 1 << 22, 3 << 21];
            for offset in 0..40 {
                ls.push(sure as u32 / 2 + offset * (sure as u32) / 40);
                ls.push(sure as u32 + offset * 7);
                ls.push((2 * sure) as u32 + offset * 13);
            }
            for exponent in [-3_i32, 6] {
                let base = f32::from_bits(((127 + exponent) as u32) << 23);
                for &l in &ls {
                    let t = -f32::from_bits(base.to_bits() + l);
                    let (steps, word) = stall_brute_force(c, t, 0.0);
                    assert!(
                        steps <= bound,
                        "c = {c:e}, t = {t:e} (L = {l}): {steps} steps, bound {bound}"
                    );
                    worst = worst.max(steps);
                    for start in [t / 3.0, f32::from_bits(word.to_bits() - 7)] {
                        let (_, other) = stall_brute_force(c, t, start);
                        assert_eq!(other.to_bits(), word.to_bits(), "c = {c:e}, t = {t:e}");
                    }
                }
            }
            std::eprintln!("c = {c:e}: largest walk {worst}, bound {bound}");
            assert!(
                bound <= worst + 2.max(worst.div_ceil(100)),
                "c = {c:e}: largest walk {worst}, bound {bound}"
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
