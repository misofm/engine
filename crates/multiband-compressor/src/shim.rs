//! The multiband compressor's retention smoother and per-band detector link.
//!
//! Both helpers keep their per-lane operation order at every width. The smoother uses retention
//! coefficients and selects attack for a lower gain-reduction target. The link clears input signs,
//! then applies D8 Maximum or two separately rounded half-products for Average.
//!
//! The inline tests own the unfused arithmetic, coefficient endpoints, link corners and width
//! agreement. The caller flushes the smoother's stored recurrence and checks output at the block
//! boundary.

use effect_contract::LinkMode;
use lane::Lane;
use lane::kernels::ramp_toward;

/// Dual mono: each channel keeps its own detector level.
pub(crate) const LINK_DUAL_MONO: u8 = 0;

/// Both channels use the larger of the two detector levels.
pub(crate) const LINK_MAXIMUM: u8 = 1;

/// Both channels use the mean of the two detector levels.
pub(crate) const LINK_AVERAGE: u8 = 2;

/// The bank's lanes do not share one mode (issue #1367 D2): each lane selects its own law.
pub(crate) const LINK_MIXED: u8 = 3;

/// Each lane's detector link mode as select masks (issue #1367 D1), and the arm the lanes pick.
///
/// `linked` is set on a lane at `Maximum` or `Average`, `averaged` on a lane at `Average`. `arm`
/// is the const arm of the lanes' one shared mode, or [`LINK_MIXED`] when they differ; it is
/// derived from the modes only, never from data, and only [`LINK_MIXED`] reads the masks.
#[derive(Clone, Copy)]
pub(crate) struct LinkLanes<L: Lane> {
    linked: L::Mask,
    averaged: L::Mask,
    pub(crate) arm: u8,
}

impl<L: Lane> LinkLanes<L> {
    /// `links[lane]` is lane `lane`'s mode. Allocation-free and panic-free.
    pub(crate) fn new(links: &[LinkMode]) -> Self {
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
            linked: L::load(&linked[..L::WIDTH]).gt(zero),
            averaged: L::load(&averaged[..L::WIDTH]).gt(zero),
            arm,
        }
    }
}

/// The detector link glide weights per channel and lane (issue #1367 D3, #1370 D2): `w_link`,
/// then `w_avg`.
pub(crate) const LINK_WEIGHTS: usize = 2;

/// One channel's link glide weights across the lanes for one segment (issue #1367 D3): gathered
/// from the lanes' ramps at the segment's start, advanced once a frame by [`advance`](Self::advance)
/// and written back at its end, like the seventeen parameter, rate and crossover ramps.
///
/// `gliding` is set on a lane whose either weight is in flight. It is constant over a segment: the
/// segment split snaps every ramp one sample from its target at a boundary and ends each segment
/// before the next arrival, so no weight arrives inside one.
#[derive(Clone, Copy)]
pub(crate) struct GlideWeights<L: Lane> {
    pub(crate) current: [L; LINK_WEIGHTS],
    pub(crate) step: [L; LINK_WEIGHTS],
    pub(crate) target: [L; LINK_WEIGHTS],
    pub(crate) gliding: L::Mask,
}

impl<L: Lane> GlideWeights<L> {
    /// No lane gliding and every word `+0.0`: the value a segment with no glide carries and never
    /// reads.
    #[inline(always)]
    pub(crate) fn idle() -> Self {
        let zero = L::zero();
        Self {
            current: [zero; LINK_WEIGHTS],
            step: [zero; LINK_WEIGHTS],
            target: [zero; LINK_WEIGHTS],
            gliding: zero.gt(zero),
        }
    }

    /// One frame of each weight: `ramp_toward(current, step, target)`, the step the other ramps
    /// take (issue #1409's clamped law). A resting weight's step is `+0.0` and its word `0.0` or
    /// `1.0`, so the update is the identity there.
    #[inline(always)]
    pub(crate) fn advance(&mut self) {
        for weight in 0..LINK_WEIGHTS {
            self.current[weight] =
                ramp_toward(self.current[weight], self.step[weight], self.target[weight]);
        }
    }
}

/// [`link_levels`] while a link glide is in flight (issue #1367 D3, #1370 D3).
///
/// Each lane's prepared law is the [`LINK_MIXED`] arm's word, its own mode's bits whatever its
/// bank-mates' modes. On each channel, a lane whose weights still glide takes instead the blend
/// `combined = max + w_avg * (avg - max)`, `d = own + w_link * (combined - own)` of the frame's
/// advanced weights, each a separate `mul` and `add` (base Wasm SIMD has no fused multiply-add),
/// with `own`, `max` and `avg` in [`link_levels`]' frozen order. A resting lane, the arriving one
/// included, keeps the prepared word: a blend at rest weights equals it on finite words, but not
/// on non-finite ones (`0 * inf`), so the `select` is load-bearing.
#[inline(always)]
pub(crate) fn glide_levels<L: Lane>(
    left: L,
    right: L,
    lanes: &LinkLanes<L>,
    near: &GlideWeights<L>,
    far: &GlideWeights<L>,
) -> (L, L) {
    let (prepared_left, prepared_right) = link_levels::<L, LINK_MIXED>(left, right, lanes);
    #[cfg(test)]
    let raw = (left, right);
    let (left, right) = (left.abs(), right.abs());
    let maximum = left.max(right);
    let half = L::splat(0.5);
    let average = half.mul(left).add(half.mul(right));
    let blend = |own: L, weights: &GlideWeights<L>| {
        let combined = maximum.add(weights.current[1].mul(average.sub(maximum)));
        own.add(weights.current[0].mul(combined.sub(own)))
    };
    let levels = (
        L::select(near.gliding, blend(left, near), prepared_left),
        L::select(far.gliding, blend(right, far), prepared_right),
    );
    #[cfg(test)]
    crate::glide_tests::record(raw, levels);
    levels
}

/// One sample of a log-domain branching smoother.
///
/// Dimitrios Giannoulis, Michael Massberg and Joshua D. Reiss, *Digital Dynamic Range Compressor
/// Design*, JAES 60(6), 2012, figure 7 (the "smooth decoupled" placement).
///
/// `y` is the previous smoothed gain **in dB** and `target` is the static curve's output for this
/// sample. `attack` and `release` are *retention* coefficients
/// (`effect_runtime::envelope::retention_coefficient`): `1.0` freezes the smoother,
/// `+0.0` makes it instantaneous.
///
/// Frozen operation order:
/// 1. `c = select(target < y, attack, release)` — ordered, so a NaN target takes the release
///    coefficient rather than an unspecified one
/// 2. `d = y - target`
/// 3. `fma(c, d, target)` — a separately rounded multiply followed by add (`Lane::fma`)
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
/// `MODE` is a **compile-time** arm, chosen once per block from the lanes' prepared modes
/// ([`LinkLanes::arm`]), so no per-sample branch reads it. An unknown `MODE` is dual mono.
///
/// Frozen operation order:
/// 1. `l = |left|`, `r = |right|` — sign-bit clears
/// 2. [`LINK_MAXIMUM`]: `l.max(r)`, the D8 select form
/// 3. [`LINK_AVERAGE`]: `0.5 * l + 0.5 * r` — two separately rounded half-products, which can
///    round subnormals but avoid the finite-input overflow possible in `0.5 * (l + r)`
///
/// [`LINK_MIXED`] (issue #1367 D2) computes both linked words in that same order and selects,
/// per lane, the word of the lane's own mode from `lanes`, so each lane's bits are the const arm's
/// of its mode and never depend on its bank-mates. The const arms do not read `lanes`.
#[inline(always)]
pub(crate) fn link_levels<L: Lane, const MODE: u8>(
    left: L,
    right: L,
    lanes: &LinkLanes<L>,
) -> (L, L) {
    let (left, right) = (left.abs(), right.abs());
    match MODE {
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

#[cfg(test)]
mod tests {
    use super::*;

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
                lane::softfma::unfused_multiply_add_via_f64(expected_coefficient, d, target);
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
        let dual = LinkLanes::<f32>::new(&[LinkMode::DualMono]);
        let maximum = LinkLanes::<f32>::new(&[LinkMode::Maximum]);
        let average = LinkLanes::<f32>::new(&[LinkMode::Average]);
        for (left, right) in [
            (0.25f32, -0.75f32),
            (-0.75, 0.25),
            (0.0, -0.0),
            (-0.0, 0.0),
            (1.0, 1.0),
            (-3.5, 3.5),
        ] {
            let (near, far) = link_levels::<f32, LINK_DUAL_MONO>(left, right, &dual);
            assert_eq!(near.to_bits(), left.abs().to_bits());
            assert_eq!(far.to_bits(), right.abs().to_bits());

            let (near, far) = link_levels::<f32, LINK_MAXIMUM>(left, right, &maximum);
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

            let (near, far) = link_levels::<f32, LINK_AVERAGE>(left, right, &average);
            assert_eq!(near.to_bits(), far.to_bits());
            assert_eq!(
                near.to_bits(),
                (0.5 * left.abs() + 0.5 * right.abs()).to_bits()
            );
        }

        // Halving each operand rather than the sum is what keeps the average finite at the top of
        // the range: `0.5 * (MAX + MAX)` overflows to infinity, `0.5 * MAX + 0.5 * MAX` does not.
        let (near, _) = link_levels::<f32, LINK_AVERAGE>(f32::MAX, -f32::MAX, &average);
        assert_eq!(
            near.to_bits(),
            f32::MAX.to_bits(),
            "the average must not overflow"
        );

        // D8: `max(a, b)` returns `b` when the two are unordered, so a NaN on the right survives
        // and a NaN on the left does not.
        let (near, _) = link_levels::<f32, LINK_MAXIMUM>(f32::NAN, 0.5, &maximum);
        assert_eq!(near, 0.5);
        let (near, _) = link_levels::<f32, LINK_MAXIMUM>(0.5, f32::NAN, &maximum);
        assert!(near.is_nan());
    }

    /// Both shims produce the same bits at every width, and the link's mixed arm gives each lane
    /// its own mode's bits.
    ///
    /// Red mutations: give either a `if L::WIDTH == 1` shortcut; the mixed arm links a DualMono
    /// lane (`linked` ignored) or averages a Maximum lane (`averaged` ignored).
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

        /// Each `L::WIDTH` chunk at the arm its lanes' modes pick (issue #1367 D2): a chunk whose
        /// lanes mix modes takes [`LINK_MIXED`], a scalar lane always its own mode's const arm.
        fn linked<L: Lane>(links: &[LinkMode; 8], left: &[f32], right: &[f32], out: &mut [u32]) {
            for first in (0..8).step_by(L::WIDTH) {
                let lanes = LinkLanes::<L>::new(&links[first..first + L::WIDTH]);
                let (l, r) = (L::load(&left[first..]), L::load(&right[first..]));
                let (near, far) = match lanes.arm {
                    LINK_DUAL_MONO => link_levels::<L, LINK_DUAL_MONO>(l, r, &lanes),
                    LINK_MAXIMUM => link_levels::<L, LINK_MAXIMUM>(l, r, &lanes),
                    LINK_AVERAGE => link_levels::<L, LINK_AVERAGE>(l, r, &lanes),
                    _ => link_levels::<L, LINK_MIXED>(l, r, &lanes),
                };
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

        let mut scalar = [0u32; 8];
        smoothed::<f32>(&y, &target, &mut scalar);
        lane::each_vector_lane!(|L, W| {
            let mut words = [0u32; 8];
            smoothed::<L>(&y, &target, &mut words);
            assert_eq!(scalar, words, "branching_smooth at W={W}");
        });

        // Every mode across the bank, then a bank whose lanes mix modes (issue #1367): each lane of
        // the mixed arm is its own mode's scalar bits, whatever its bank-mates' modes.
        let (d, m, a) = (LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average);
        for links in [
            [d; 8],
            [m; 8],
            [a; 8],
            [d, m, a, d, a, m, d, m],
            [m, a, a, d, d, d, m, a],
        ] {
            let mut scalar = [0u32; 8];
            linked::<f32>(&links, &y, &target, &mut scalar);
            lane::each_vector_lane!(|L, W| {
                let mut words = [0u32; 8];
                linked::<L>(&links, &y, &target, &mut words);
                assert_eq!(scalar, words, "link modes {links:?} at W={W}");
            });
        }
    }
}
