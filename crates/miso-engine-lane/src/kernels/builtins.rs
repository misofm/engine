//! Block kernels for the builtin track chain: input sanitisation, trim, fader/mute and the
//! smoothed 2x2 channel matrix (issue #85).
//!
//! These live here, next to [`svf_block`](super::svf_block), for the same reason every other
//! kernel does: one generic body, instantiated at every width, is what makes lane identity a
//! property of the code rather than of a corpus (master plan §1, §4.2). The builtin chain is
//! `sanitize_gain_block` -> `svf_block` (HPF) -> `svf_block` (LPF) -> `nonfinite_lanes_block`
//! -> `gain_mute_block` -> `matrix2x2_block`, with one AoSoA transpose pair per chain per block.
//!
//! # Operation order is frozen
//!
//! As in [`super`], each kernel's doc comment lists its operations line by line, and that order is
//! the numeric contract. None of these kernels uses [`Lane::fma`]: they are feed-forward, and the
//! unfused order below is what keeps the gain and matrix fixtures bit-identical to the scalar
//! chain they replace (master plan D3: fusion exists only where `fma` is written).

use crate::Lane;
use crate::kernels::{SvfCoef, SvfState, svf_step};

/// Magnitude at or above which a sample is treated as non-finite by the D7 boundary policy.
///
/// `!(|x| < 1e30)` is exactly "NaN or `|x| >= 1e30`", because an ordered compare against NaN is
/// false; the NaN case therefore needs no separate `x == x` term. This is the same threshold and
/// the same one-compare form as `miso_engine_effect_runtime::bank::check_block` (master plan §4.4).
pub const NONFINITE_LIMIT: f32 = 1.0e30;

/// A mask with no lane set, built from an ordered compare that is false in every lane.
#[inline(always)]
pub fn no_lanes<L: Lane>() -> L::Mask {
    L::zero().lt(L::zero())
}

/// A mask with every lane set.
#[inline(always)]
pub fn all_lanes<L: Lane>() -> L::Mask {
    L::zero().eq(L::zero())
}

/// A mask set in the lanes whose index is below `count`.
///
/// Control-plane only: this is how a partially populated bank marks its padding lanes once, at
/// preparation. `count` may be any value in `0..=L::WIDTH`.
#[inline(always)]
pub fn lanes_below<L: Lane>(count: usize) -> L::Mask {
    debug_assert!(count <= L::WIDTH);
    let mut flags = [0.0_f32; 64];
    for flag in flags.iter_mut().take(count) {
        *flag = 1.0;
    }
    L::load(&flags[..L::WIDTH]).gt(L::zero())
}

/// A mask built from one flag per lane; a lane is set where its flag is non-zero.
///
/// Control-plane only, like [`lanes_below`]: masks are per-lane booleans decided at preparation
/// (mute, padding, matrix identity), and this is the one place they cross from scalar bookkeeping
/// into the vector domain.
///
/// # Panics
///
/// Panics if `flags` is shorter than `L::WIDTH`.
#[inline(always)]
pub fn mask_from_flags<L: Lane>(flags: &[f32]) -> L::Mask {
    L::load(flags).gt(L::zero())
}

/// Input sanitisation and trim in one pass; returns the per-lane count of sanitised samples.
///
/// This is the D7 input stage: sanitisation happens once per track per block here, not inside
/// every downstream kernel. A sanitised sample becomes exactly `+0.0` before the gain, so a
/// non-finite input can never enter the filter recurrence.
///
/// Frozen operation order, per frame:
/// 1. `x = load(frame)`
/// 2. `bad = !(|x| < NONFINITE_LIMIT)` — one ordered compare; NaN is included because the compare
///    is false for it
/// 3. `count = count + select(bad, 1.0, 0.0)`
/// 4. `y = andnot(x, bad) * gain` — one multiply, no fusion
/// 5. `store(frame, y)`
///
/// The count is an exact `f32` integer: a block never has more frames than `2^24`, so the
/// accumulation is exact and the caller reads it back with `store`.
#[inline(always)]
pub fn sanitize_gain_block<L: Lane>(io: &mut [f32], frames: usize, gain: L) -> L {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut count = L::zero();
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(L::select(bad, one, zero));
        x.andnot(bad).mul(gain).store(frame);
    }
    count
}

/// The D7 block boundary check (master plan §4.4), at per-lane granularity.
///
/// Returns the mask of lanes that produced a non-finite sample anywhere in the block: one ordered
/// compare and one mask OR per frame, and no `mask_any` inside the loop. The caller calls
/// [`Lane::mask_any`] once per block, and only on that rare path does it pay for
/// [`zero_lanes_block`] and a state reset.
///
/// Per-lane granularity is deliberate: one track's non-finite block must not change another
/// track's bits, or a track's output would depend on its cohort membership.
///
/// Frozen operation order, per frame: `bad = bad | !(|load(frame)| < NONFINITE_LIMIT)`.
#[inline(always)]
pub fn nonfinite_lanes_block<L: Lane>(io: &[f32], frames: usize) -> L::Mask {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let mut bad = no_lanes::<L>();
    for frame in io.chunks_exact(L::WIDTH) {
        bad = L::mask_or(bad, L::mask_not(L::load(frame).abs().lt(limit)));
    }
    bad
}

/// Clears every frame of the lanes selected by `m` to exactly `+0.0`.
///
/// The rare arm of [`nonfinite_lanes_block`]: lanes that are not selected keep their bits exactly.
///
/// Frozen operation order, per frame: `store(frame, andnot(load(frame), m))`.
#[inline(always)]
pub fn zero_lanes_block<L: Lane>(io: &mut [f32], frames: usize, m: L::Mask) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    for frame in io.chunks_exact_mut(L::WIDTH) {
        L::load(frame).andnot(m).store(frame);
    }
}

/// Fader gain with mute, one pass.
///
/// A muted lane becomes exactly `+0.0`, including for a negative input: `andnot` clears every bit,
/// where multiplying by zero would keep the sign. An unmuted lane is one multiply, so gain `1.0`
/// preserves signed zero.
///
/// Frozen operation order, per frame: `store(frame, andnot(load(frame) * gain, mute))`.
#[inline(always)]
pub fn gain_mute_block<L: Lane>(io: &mut [f32], frames: usize, gain: L, mute: L::Mask) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    for frame in io.chunks_exact_mut(L::WIDTH) {
        L::load(frame).mul(gain).andnot(mute).store(frame);
    }
}

/// Coefficients of a settled 2x2 channel matrix, one set per lane.
#[derive(Clone, Copy)]
pub struct Matrix2x2Coef<L: Lane> {
    /// Left output from the left input.
    pub ll: L,
    /// Left output from the right input.
    pub lr: L,
    /// Right output from the left input.
    pub rl: L,
    /// Right output from the right input.
    pub rr: L,
    /// Lanes whose coefficients are exactly the identity matrix.
    ///
    /// Computed by the caller once per settle, never per frame. An identity lane passes its
    /// samples through untouched, which is what preserves `-0.0` on a settled identity matrix.
    pub identity: L::Mask,
}

/// Applies a settled 2x2 channel matrix to a pair of AoSoA blocks.
///
/// Frozen operation order, per frame:
/// 1. `l = load(left)`, `r = load(right)`
/// 2. `yl = select(identity, l, ll * l + lr * r)` — multiply, multiply, add; no fusion
/// 3. `yr = select(identity, r, rl * l + rr * r)`
/// 4. `store(left, yl)`, `store(right, yr)`
///
/// Both arms are always evaluated and selected per lane: there is no branch and no `mask_any`.
#[inline(always)]
pub fn matrix2x2_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &Matrix2x2Coef<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        let l = L::load(left_frame);
        let r = L::load(right_frame);
        let yl = L::select(c.identity, l, c.ll.mul(l).add(c.lr.mul(r)));
        let yr = L::select(c.identity, r, c.rl.mul(l).add(c.rr.mul(r)));
        yl.store(left_frame);
        yr.store(right_frame);
    }
}

/// State of a ramping 2x2 channel matrix, one set per lane. Coefficient order is `[ll, lr, rl, rr]`.
#[derive(Clone, Copy)]
pub struct Matrix2x2Ramp<L: Lane> {
    /// The value applied to the current frame.
    pub current: [L; 4],
    /// The value assigned exactly on the last ramping frame (D11: the snap is an assignment).
    pub target: [L; 4],
    /// Per-sample increment, `(target - start) / n`, computed once per event.
    pub step: [L; 4],
    /// Frames left in this lane's ramp, as an exact `f32` integer (the caller clamps to `2^24`).
    pub remaining: L,
}

/// Applies a ramping 2x2 channel matrix (D11) to a pair of AoSoA blocks.
///
/// The identity select of [`matrix2x2_block`] is deliberately **not** applied while a lane is
/// ramping: a ramp that passes through the identity must not change its arithmetic mid-flight.
///
/// Frozen operation order, per frame:
/// 1. `remaining = remaining - 1`
/// 2. `done = remaining <= 0`
/// 3. `current[i] = select(done, target[i], current[i] + step[i])` for `i` in `0..4`
/// 4. `l = load(left)`, `r = load(right)`
/// 5. `yl = ll * l + lr * r`, `yr = rl * l + rr * r` — the [`matrix2x2_block`] arithmetic, in the
///    same operation order
/// 6. `store(left, yl)`, `store(right, yr)`
///
/// `remaining` and `current` are advanced in place, so a lane's ramp carries across block
/// boundaries and evolves by its own additions regardless of block size or of its neighbours:
/// partition and cohort invariance hold by construction.
#[inline(always)]
pub fn matrix2x2_ramp_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    r: &mut Matrix2x2Ramp<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut remaining = r.remaining;
    let mut current = r.current;
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        remaining = remaining.sub(one);
        let done = remaining.le(zero);
        for ((current, target), step) in current.iter_mut().zip(&r.target).zip(&r.step) {
            *current = L::select(done, *target, current.add(*step));
        }
        let l = L::load(left_frame);
        let right_sample = L::load(right_frame);
        let yl = current[0].mul(l).add(current[1].mul(right_sample));
        let yr = current[2].mul(l).add(current[3].mul(right_sample));
        yl.store(left_frame);
        yr.store(right_frame);
    }
    r.remaining = remaining;
    r.current = current;
}

/// The prepared coefficients of one dual-mono input chain, for [`input_chain_block`].
#[derive(Clone, Copy)]
pub struct InputChainCoef<L: Lane> {
    /// Trim with the polarity inversion folded in, `[left, right]`.
    pub trim: [L; 2],
    /// `[channel][section]`, section `0` applied first.
    pub section: [[SvfCoef<L>; 2]; 2],
}

/// The retained integrator state of one dual-mono input chain, indexed like
/// [`InputChainCoef::section`].
#[derive(Clone, Copy)]
pub struct InputChainState<L: Lane> {
    /// `[channel][section]`.
    pub section: [[SvfState<L>; 2]; 2],
}

impl<L: Lane> Default for InputChainState<L> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            section: [[SvfState::default(); 2]; 2],
        }
    }
}

/// What one [`input_chain_block`] call sanitised and what its output boundary check found.
pub struct InputChainReport<L: Lane> {
    /// Per-lane count of sanitised input samples, per channel, as an exact `f32` integer.
    pub sanitized: [L; 2],
    /// Per-channel mask of lanes whose output was non-finite anywhere in the block.
    pub nonfinite: [L::Mask; 2],
}

/// The whole builtin input chain — sanitise, trim, two cascaded sections, boundary scan — for
/// both channels, in **one** frame loop.
///
/// # Why one loop
///
/// The four recurrences (two sections, two channels) are independent, and each is a serial
/// dependency chain of about twenty-five cycles per sample. Run as four separate block passes they
/// serialise: the scalar chain costs the sum of four latencies. Interleaved in one frame body they
/// overlap, and the block is also read and written once instead of six times. The measured effect
/// at `WIDTH = 1` is better than two to one; at `WIDTH = 8` it is about one and a half to one.
///
/// # Why the bits do not move
///
/// Every operation, and the order of every operation, is the one the separate kernels use:
/// [`sanitize_gain_block`] for step 1, [`super::svf_step`] — the single copy of the recurrence,
/// shared with [`super::svf_block`] — plus that kernel's output mix for steps 2 and 3, and
/// [`nonfinite_lanes_block`] for step 4. The intermediate value that used
/// to be stored and reloaded between passes is now kept in a register, which is exact, and the
/// counter and mask accumulations keep their per-frame order. This is a scheduling change, not a
/// numeric one (master plan §8 class A).
///
/// Frozen operation order, per frame and per channel `ch`:
/// 1. `x = load(frame)`
/// 2. `bad = !(|x| < NONFINITE_LIMIT)`; `sanitized[ch] = sanitized[ch] + select(bad, 1.0, 0.0)`
/// 3. `v = andnot(x, bad) * trim[ch]`
/// 4. `v = svf_step(v, section[ch][0])`, then `v = svf_step(v, section[ch][1])`
/// 5. `nonfinite[ch] = nonfinite[ch] | !(|v| < NONFINITE_LIMIT)`
/// 6. `store(frame, v)`
///
/// The left channel's frame is evaluated before the right channel's, as it was when they were
/// separate passes.
#[inline(always)]
pub fn input_chain_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    // The four integrator pairs and the four negated damping coefficients live in registers for the
    // whole block; `svf_step` documents that its state must be a local copy for exactly this
    // reason, or it would be reloaded from memory every frame (D10).
    let mut state = s.section;
    let mut nc1 = [[zero; 2]; 2];
    for (channel, coefficients) in c.section.iter().enumerate() {
        for (section, coefficient) in coefficients.iter().enumerate() {
            nc1[channel][section] = coefficient.c1.neg();
        }
    }

    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        for (channel, frame) in [left_frame, right_frame].into_iter().enumerate() {
            let x = L::load(frame);
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(L::select(bad, one, zero));
            let mut v = x.andnot(bad).mul(c.trim[channel]);
            for section in 0..2 {
                let coefficient = &c.section[channel][section];
                let v0 = v;
                let (v1, v2) = svf_step(
                    v0,
                    nc1[channel][section],
                    coefficient.a2,
                    coefficient.a3,
                    &mut state[channel][section],
                );
                v = coefficient
                    .m2
                    .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
            }
            nonfinite[channel] = L::mask_or(nonfinite[channel], L::mask_not(v.abs().lt(limit)));
            v.store(frame);
        }
    }

    s.section = state;
    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}
