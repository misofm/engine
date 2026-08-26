//! Two lane-generic pieces that belong in `miso-engine-effect-runtime` and are not there yet.
//!
//! Wave-2 decision **W2-D3** on issue #83 gives ownership of the stereo-coupled dynamics
//! scaffolding to **#88**: that job adds one shared form, and #89 and #94 consume it after it
//! merges rather than landing parallel variants. Both functions below are therefore **shims**,
//! written here so this crate can land, and they are the first thing to delete when #88's version
//! is on `main`. Both are one expression; adapting the two call sites is a one-line change each.
//!
//! What they must keep when they move:
//!
//! * The **direction** of the smoother's coefficient select. More gain reduction is a *lower* dB
//!   value, so the attack coefficient belongs to `target < y`. The 83c survey found the gate crate
//!   using the opposite compare under the opposite sign convention — correct in both, and exactly
//!   the thing a shared helper must not paper over. It is gated in `tests/dynamics_shims.rs`.
//! * The **exact identity at `c = 0`**. `fma(c, y - target, target)` returns `target` exactly
//!   there, unfused as much as fused (`0 * d + target == target` for finite `d`);
//!   the `c * y + (1 - c) * target` form the audit found copied into two crates rounds three
//!   times and does not.
//! * The link's `max` is the **D8 select form**, never `f32::max`. The copies relied on their
//!   inputs having been sanitised first, and under D7 nothing sanitises per value any more.

use miso_engine_lane::Lane;

/// Dual mono: each channel keeps its own detector level.
pub(crate) const LINK_DUAL_MONO: u8 = 0;

/// Both channels use the larger of the two detector levels.
pub(crate) const LINK_MAXIMUM: u8 = 1;

/// Both channels use the mean of the two detector levels.
pub(crate) const LINK_AVERAGE: u8 = 2;

/// One sample of a log-domain branching smoother.
///
/// Dimitrios Giannoulis, Michael Massberg and Joshua D. Reiss, *Digital Dynamic Range Compressor
/// Design*, JAES 60(6), 2012, figure 7 (the "smooth decoupled" placement).
///
/// `y` is the previous smoothed gain **in dB** and `target` is the static curve's output for this
/// sample. `attack` and `release` are *retention* coefficients
/// (`miso_engine_effect_runtime::envelope::retention_coefficient`): `1.0` freezes the smoother,
/// `+0.0` makes it instantaneous.
///
/// Frozen operation order:
/// 1. `c = select(target < y, attack, release)` — ordered, so a NaN target takes the release
///    coefficient rather than an unspecified one
/// 2. `d = y - target`
/// 3. `fma(c, d, target)` — **one** rounding
///
/// The caller applies `flush` to the result: this is a recurrence, so D7 applies to it, but the
/// flush belongs with the state word the caller stores.
#[inline(always)]
pub(crate) fn branching_smooth<L: Lane>(y: L, target: L, attack: L, release: L) -> L {
    let c = L::select(target.lt(y), attack, release);
    let d = y.sub(target);
    c.fma(d, target)
}

/// Combines a stereo pair of detector inputs into the pair of levels the two channels ride.
///
/// The link mode is a **compile-time** parameter: it is fixed when the effect is prepared and
/// never changes for the life of the prepared instance, so a per-sample branch on it would be a
/// branch whose outcome is known at preparation. An unknown `MODE` is dual mono.
///
/// Frozen operation order:
/// 1. `l = |left|`, `r = |right|` — sign-bit clears
/// 2. [`LINK_MAXIMUM`]: `l.max(r)`, the D8 select form
/// 3. [`LINK_AVERAGE`]: `0.5 * l + 0.5 * r` — halving is exact, so this cannot overflow the way
///    `0.5 * (l + r)` can
#[inline(always)]
pub(crate) fn link_levels<L: Lane, const MODE: u8>(left: L, right: L) -> (L, L) {
    let (left, right) = (left.abs(), right.abs());
    match MODE {
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

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_lane::{Simd4, Simd8};

    /// The smoother is `target + c * (y - target)`, and the attack coefficient is the one selected
    /// when the target asks for more reduction.
    ///
    /// The oracle restates that operation order through `f64`, taking each `f32` rounding
    /// separately (issue #163 phase 2 made `Lane::fma` unfused). `d` is an `f32` subtraction
    /// inside the function, so the oracle takes the same rounded difference.
    ///
    /// The `c * y + (1 - c) * target` rearrangement is checked to be *different* on more than a
    /// thousand steps, so this is not vacuous. That form is a different algebra, not a different
    /// rounding contract: it stays wrong for the same reason it was wrong before the phase.
    ///
    /// Red mutations: `select(target.gt(y), ..)`; `c * y + (1 - c) * target`; restoring the fused
    /// oracle (one `f64` expression narrowed once).
    #[test]
    fn the_smoother_picks_the_attack_direction_and_matches_the_unfused_restatement() {
        let mut differing = 0usize;
        let mut state = 0.0f32;
        for step in 0..20_000i32 {
            let target = if (step / 137) % 2 == 0 {
                -18.0 + (step % 61) as f32 * 0.25
            } else {
                -0.5 - (step % 29) as f32 * 0.03
            };
            let attack = 0.987_654_3f32;
            let release = 0.999_012_3f32;
            let expected_coefficient = if target < state { attack } else { release };
            let d = state - target;
            let oracle =
                miso_engine_lane::softfma::unfused_mul_add_via_f64(expected_coefficient, d, target);
            let rearranged = expected_coefficient * state + (1.0 - expected_coefficient) * target;
            if rearranged.to_bits() != oracle.to_bits() {
                differing += 1;
            }
            let actual = branching_smooth::<f32>(state, target, attack, release);
            assert_eq!(
                actual.to_bits(),
                oracle.to_bits(),
                "step {step}: state {state}, target {target}"
            );
            state = actual;
        }
        assert!(
            differing > 1_000,
            "the rearranged form must differ often enough for this to be a gate: {differing}"
        );
    }

    /// Both ends are exact: a zero coefficient arrives, a unit coefficient freezes.
    #[test]
    fn the_smoother_has_exact_ends() {
        for target in [-42.5f32, 0.0, -0.0, -100.0] {
            assert_eq!(
                branching_smooth::<f32>(-7.5, target, 0.0, 0.0).to_bits(),
                target.to_bits(),
                "a zero coefficient must arrive exactly"
            );
            assert_eq!(
                branching_smooth::<f32>(-7.5, target, 1.0, 1.0),
                -7.5,
                "a unit coefficient must freeze"
            );
        }
    }

    /// The three link modes, including the `±0.0`, overflow and NaN corners.
    ///
    /// Red mutations: maximum becomes minimum; the `abs` is dropped; the average halves the sum.
    #[test]
    fn the_link_combines_the_pair_per_mode() {
        for (left, right) in [
            (0.25f32, -0.75f32),
            (-0.75, 0.25),
            (0.0, -0.0),
            (-0.0, 0.0),
            (1.0, 1.0),
            (-3.5, 3.5),
        ] {
            let (near, far) = link_levels::<f32, LINK_DUAL_MONO>(left, right);
            assert_eq!(near.to_bits(), left.abs().to_bits());
            assert_eq!(far.to_bits(), right.abs().to_bits());

            let (near, far) = link_levels::<f32, LINK_MAXIMUM>(left, right);
            assert_eq!(near.to_bits(), far.to_bits());
            let larger = if left.abs() > right.abs() {
                left.abs()
            } else {
                right.abs()
            };
            assert_eq!(
                near.to_bits(),
                larger.to_bits(),
                "maximum must be the larger absolute level"
            );

            let (near, far) = link_levels::<f32, LINK_AVERAGE>(left, right);
            assert_eq!(near.to_bits(), far.to_bits());
            assert_eq!(
                near.to_bits(),
                (0.5 * left.abs() + 0.5 * right.abs()).to_bits()
            );
        }

        // Halving each operand rather than the sum is what keeps the average finite at the top of
        // the range: `0.5 * (MAX + MAX)` overflows to infinity, `0.5 * MAX + 0.5 * MAX` does not.
        let (near, _) = link_levels::<f32, LINK_AVERAGE>(f32::MAX, -f32::MAX);
        assert_eq!(
            near.to_bits(),
            f32::MAX.to_bits(),
            "the average must not overflow"
        );

        // D8: `max(a, b)` returns `b` when the two are unordered, so a NaN on the right survives
        // and a NaN on the left does not.
        let (near, _) = link_levels::<f32, LINK_MAXIMUM>(f32::NAN, 0.5);
        assert_eq!(near, 0.5);
        let (near, _) = link_levels::<f32, LINK_MAXIMUM>(0.5, f32::NAN);
        assert!(near.is_nan());
    }

    /// Both shims produce the same bits at every width.
    ///
    /// Red mutation: give either a `if L::WIDTH == 1` shortcut.
    #[test]
    fn the_shims_are_width_independent() {
        fn smoothed<L: Lane>(y: &[f32], target: &[f32], out: &mut [u32]) {
            for first in (0..8).step_by(L::WIDTH) {
                branching_smooth::<L>(
                    L::load(&y[first..]),
                    L::load(&target[first..]),
                    L::splat(0.987_654_3),
                    L::splat(0.999_012_3),
                )
                .store_bits(&mut out[first..]);
            }
        }

        fn linked<L: Lane, const MODE: u8>(left: &[f32], right: &[f32], out: &mut [u32]) {
            for first in (0..8).step_by(L::WIDTH) {
                let (near, far) =
                    link_levels::<L, MODE>(L::load(&left[first..]), L::load(&right[first..]));
                near.add(far.mul(L::splat(3.0)))
                    .store_bits(&mut out[first..]);
            }
        }

        let mut y = [0.0f32; 8];
        let mut target = [0.0f32; 8];
        for lane in 0..8 {
            y[lane] = -0.9 + lane as f32 * 0.27;
            target[lane] = 0.6 - lane as f32 * 0.19;
        }

        let (mut scalar, mut four, mut eight) = ([0u32; 8], [0u32; 8], [0u32; 8]);
        smoothed::<f32>(&y, &target, &mut scalar);
        smoothed::<Simd4>(&y, &target, &mut four);
        smoothed::<Simd8>(&y, &target, &mut eight);
        assert_eq!(scalar, four, "branching_smooth at W=4");
        assert_eq!(scalar, eight, "branching_smooth at W=8");

        for mode in [LINK_DUAL_MONO, LINK_MAXIMUM, LINK_AVERAGE] {
            let (mut scalar, mut four, mut eight) = ([0u32; 8], [0u32; 8], [0u32; 8]);
            match mode {
                LINK_MAXIMUM => {
                    linked::<f32, LINK_MAXIMUM>(&y, &target, &mut scalar);
                    linked::<Simd4, LINK_MAXIMUM>(&y, &target, &mut four);
                    linked::<Simd8, LINK_MAXIMUM>(&y, &target, &mut eight);
                }
                LINK_AVERAGE => {
                    linked::<f32, LINK_AVERAGE>(&y, &target, &mut scalar);
                    linked::<Simd4, LINK_AVERAGE>(&y, &target, &mut four);
                    linked::<Simd8, LINK_AVERAGE>(&y, &target, &mut eight);
                }
                _ => {
                    linked::<f32, LINK_DUAL_MONO>(&y, &target, &mut scalar);
                    linked::<Simd4, LINK_DUAL_MONO>(&y, &target, &mut four);
                    linked::<Simd8, LINK_DUAL_MONO>(&y, &target, &mut eight);
                }
            }
            assert_eq!(scalar, four, "link mode {mode} at W=4");
            assert_eq!(scalar, eight, "link mode {mode} at W=8");
        }
    }
}
