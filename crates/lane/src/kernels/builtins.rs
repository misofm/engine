//! Block kernels for the builtin track chain: input sanitisation, trim, fader/mute and the
//! smoothed 2x2 channel matrix (issue #85).
//!
//! These live here, next to [`svf_block`](super::svf_block), for the same reason every other
//! kernel does: one generic body, instantiated at every width, is what makes lane identity a
//! property of the code rather than of a corpus (master plan §1, §4.2). The builtin chain is
//! `sanitize_gain_block` -> `svf_block` (HPF) -> `svf_block` (LPF) -> the D7 per-lane non-finite
//! check -> `gain_mute_block` -> `matrix2x2_block`, with one AoSoA transpose pair per chain per
//! block.
//!
//! # Operation order is frozen
//!
//! As in [`super`], each kernel's doc comment lists its operations line by line, and that order is
//! the numeric contract. None of these kernels uses [`Lane::fma`]: they are feed-forward, and the
//! unfused order below is what keeps the gain and matrix fixtures bit-identical to the scalar
//! chain they replace (master plan D3: fusion exists only where `fma` is written).

use crate::Lane;
use crate::kernels::{SvfCoef, SvfState, silence_skip_block, svf_state_held, svf_step_when};
use crate::{REST_EPS, silence_armable_holding, silence_step_hoisted};

/// Magnitude at or above which a sample is treated as non-finite by the D7 boundary policy.
///
/// `!(|x| < 1e30)` is exactly "NaN or `|x| >= 1e30`", because an ordered compare against NaN is
/// false; the NaN case therefore needs no separate `x == x` term. This is the same threshold and
/// the same one-compare form as `effect_runtime::bank::check_block` (master plan §4.4).
pub const NONFINITE_LIMIT: f32 = 1.0e30;

/// A mask with no lane set, built from an ordered compare that is false in every lane.
#[inline(always)]
pub fn no_lanes<L: Lane>() -> L::Mask {
    L::zero().lt(L::zero())
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
/// 3. `count = count + (1.0 & bad)` — the and-form; see below
/// 4. `y = andnot(x, bad) * gain` — one multiply, no fusion
/// 5. `store(frame, y)`
///
/// The count is an exact `f32` integer: a block never has more frames than `2^24`, so the
/// accumulation is exact and the caller reads it back with `store`.
///
/// # The and-form, and why it is not a numeric change
///
/// Step 3 was `count + select(bad, 1.0, 0.0)` and is now `count + one.andnot(mask_not(bad))`,
/// which is `1.0 & bad`. The two are the *same bits*, not merely the same value, and the reason is
/// the canonical-mask contract on [`Lane::Mask`]: a comparison result is per lane either all zero
/// bits or all one bits, and nothing else. `bad` is `mask_not` of a single ordered compare at every
/// site, so it is canonical; `mask_not` of a canonical mask is canonical; and on a canonical mask
/// `select(m, a, b)` is by definition `(m & a) | (!m & b)`, which with `b = +0.0` — all zero bits —
/// is exactly `m & a`. So the and-form is a spelling of step 3, not a rounding of it.
///
/// It is used at **every** copy of this frame body: here, in [`input_chain_block`], and in the two
/// elision variants `identity_chain_block` and `mixed_chain_block`, which duplicate the
/// sanitise prologue. A copy left on the select-form would still be correct — that is the point of
/// the equivalence — but the four are kept identical so the frozen order above has one reading.
///
/// The **floor accounting does not move**: this was one mask-and-value operation before and is one
/// now, so the sanitise inventory in `docs/rulings/effect-floor-accounting.md` stays at 7
/// lane-ops. The change is an instruction-selection one, gated by
/// `crates/lane/tests/sanitise_counter.rs`.
#[inline(always)]
pub fn sanitize_gain_block<L: Lane>(io: &mut [f32], frames: usize, gain: L) -> L {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let mut count = L::zero();
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        x.andnot(bad).mul(gain).store(frame);
    }
    count
}

/// Clears every frame of the lanes selected by `m` to exactly `+0.0`.
///
/// The rare arm of the D7 block-boundary check: lanes that are not selected keep their bits
/// exactly.
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

/// One D11 ramp update that never passes its target (issue #1408).
///
/// Returns `current + step`, held inside `[min(current, target), max(current, target)]`. Every
/// builtin ramp (trim and polarity, fader and mute, and the four matrix words) advances its word
/// with this and nothing else, as step 3 of each ramping kernel's frozen order:
/// `current = select(done, target, ramp_toward(current, step, target))`.
///
/// # Why it is exact and needs no stored start
///
/// The step is `(target - start) / n`, so it has the sign of `target - start`. Round-to-nearest
/// is monotone and `current` is representable, so `fl(current + step)` never lands on the far
/// side of `current` from the direction of `step`. By induction from the event, every word lies
/// between `start` and `target`; the lower bound is therefore never the active side, and the clamp
/// only holds a word that would pass `target` at `target`. Once there, both bounds are `target`
/// and the word stays. Without the clamp, a long ramp's accumulated rounding is bounded only by
/// `max(|start|, |2 target - start|)`.
///
/// # Operand order is part of the contract
///
/// [`Lane::max`] and [`Lane::min`] keep their *second* operand unless the first is strictly
/// beyond it. `L::max(low, next)` and `L::min(high, x)` therefore replace the word only when it is
/// strictly outside an endpoint: an in-range word keeps the unclamped law's exact bits, including
/// a `+0.0` next to a `-0.0` endpoint and the reverse, and a NaN `next` passes through rather than
/// being hidden. Do not write `L::max(next, low)` or a `clamp`. The trait forms are the D8
/// specification at every width (`check-lane-policy.sh`).
///
/// Four lane operations beyond the add, one generic body at every width.
#[inline(always)]
pub fn ramp_toward<L: Lane>(current: L, step: L, target: L) -> L {
    let next = current.add(step);
    let low = L::min(current, target);
    let high = L::max(current, target);
    L::min(high, L::max(low, next))
}

/// State of a ramping fader/mute, one set per lane (issue #212, the banked strip fader).
///
/// One channel of one bank: every array is `[lane]`, and a dual-mono stage carries two of these.
#[derive(Clone, Copy)]
pub struct GainMuteRamp<L: Lane> {
    /// The gain applied to the current frame.
    pub current: L,
    /// The gain assigned exactly on the last ramping frame (D11: the snap is an assignment).
    pub target: L,
    /// Per-sample increment, `(target - start) / n`, computed once per event.
    pub step: L,
    /// Frames left in this lane's ramp, as an exact `f32` integer (the caller clamps to `2^24`).
    pub remaining: L,
    /// Muted lanes. A muted lane is cleared from the frame its ramp settles on, never after.
    pub mute: L::Mask,
}

/// Applies a ramping fader and mute (D11) to one AoSoA plane.
///
/// This is the banked form of the per-track scalar ramp, and it is the *only* form: the scalar
/// track is this kernel at `L = f32`, so lane identity is a property of the code and not of two
/// implementations agreeing (the same rule `input_chain_block` follows).
///
/// Frozen operation order, per frame:
/// 1. `remaining = remaining - 1`
/// 2. `done = remaining <= 0`
/// 3. `current = select(done, target, ramp_toward(current, step, target))` -- the word never
///    passes its target ([`ramp_toward`])
/// 4. `store(frame, andnot(load(frame) * current, done & mute))`
///
/// # Why the clear is gated on `done` and not on the block
///
/// A settled mute must be exactly `+0.0`, including for a negative input, which is what
/// [`gain_mute_block`]'s `andnot` gives the prepared path. Step 3 assigns the target exactly on
/// the frame the ramp settles on, so that frame's product already has magnitude zero and only its
/// sign is in question; clearing from exactly that frame -- not the one after -- is what makes
/// "every sample of a completed mute is `+0.0`" true of the settling sample too. An unmuted lane
/// has an all-zero mask and is one multiply, so gain `1.0` still preserves signed zero.
///
/// `remaining` and `current` are advanced in place, so a lane's ramp carries across block
/// boundaries and evolves by its own additions regardless of block size or of its neighbours:
/// partition and cohort invariance hold by construction, exactly as they do for
/// [`matrix2x2_ramp_block`].
#[inline(always)]
pub fn gain_mute_ramp_block<L: Lane>(io: &mut [f32], frames: usize, r: &mut GainMuteRamp<L>) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut remaining = r.remaining;
    let mut current = r.current;
    for frame in io.chunks_exact_mut(L::WIDTH) {
        remaining = remaining.sub(one);
        let done = remaining.le(zero);
        current = L::select(done, r.target, ramp_toward(current, r.step, r.target));
        L::load(frame)
            .mul(current)
            .andnot(L::mask_and(done, r.mute))
            .store(frame);
    }
    r.remaining = remaining;
    r.current = current;
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

/// [`matrix2x2_block`] for a coefficient set with **no** identity lane: its second arm alone.
///
/// Why it exists: on `x86-64-v3`, LLVM folds [`matrix2x2_block`]'s left output -- a store of a
/// select whose first arm is the word just loaded from the same address -- into one `vmaskmovps`
/// masked store under `!identity`. On Zen 2 to Zen 4 that store is about 42 uops with a 12-cycle
/// reciprocal throughput, so a settled pan matrix costs several times its arithmetic (issue #944).
/// No standing session has an identity lane in a full bank: the equal-power pan law never yields
/// one, because `cos(pi / 2)` rounds to `6.1e-17` in `f32`.
///
/// Frozen operation order, per frame -- [`matrix2x2_block`]'s second arm verbatim:
/// 1. `l = load(left)`, `r = load(right)`
/// 2. `yl = ll * l + lr * r` -- multiply, multiply, add; no fusion
/// 3. `yr = rl * l + rr * r`
/// 4. `store(left, yl)`, `store(right, yr)`
///
/// There is no `L::select` anywhere in the body, not even one on a constant mask: a select is
/// exactly what LLVM rebuilds the masked store from.
///
/// Precondition: `!L::mask_any(c.identity)` over all `L::WIDTH` lanes, padding lanes included,
/// checked by a `debug_assert!`. The caller tests it once per call, never per frame.
///
/// Class A: with no identity lane, [`matrix2x2_block`]'s per-lane select returns its second arm,
/// so the two kernels compute the same products and sums on every lane of every word, padding
/// included. Every non-NaN word is bit-identical. A NaN word stays a NaN: LLVM may commute the
/// commutative `fadd` differently in the two bodies, and when both products are NaN with different
/// payloads x86 keeps the first operand's. Rendered plans never reach that case -- the input stage
/// sanitises every non-finite sample before the matrix -- and the payload is not part of the
/// class-A statement.
#[inline(always)]
pub fn matrix2x2_block_without_identity<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &Matrix2x2Coef<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    debug_assert!(
        !L::mask_any(c.identity),
        "the select-free matrix arm requires a coefficient set with no identity lane"
    );
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        let l = L::load(left_frame);
        let r = L::load(right_frame);
        let yl = c.ll.mul(l).add(c.lr.mul(r));
        let yr = c.rl.mul(l).add(c.rr.mul(r));
        yl.store(left_frame);
        yr.store(right_frame);
    }
}

/// Applies the settled fader/mute and 2x2 matrix in one frame traversal.
///
/// Frozen operation order, per frame:
/// 1. `l = load(left) * gain_left`, `r = load(right) * gain_right`, then clear each muted lane
/// 2. `yl = select(identity, l, ll * l + lr * r)` and `yr = select(identity, r, rl * l + rr * r)`
/// 3. store both planes
///
/// Both input planes are loaded before either is written. The fader products retain sample before
/// gain operand order; coefficient-before-sample applies to the matrix products. Both matrix arms
/// are evaluated before the identity select.
/// This is the settled equivalent of [`gain_mute_block`] followed by [`matrix2x2_block`].
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fader_matrix_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    gain_left: L,
    mute_left: L::Mask,
    gain_right: L,
    mute_right: L::Mask,
    matrix: &Matrix2x2Coef<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        let left_input = L::load(left_frame);
        let right_input = L::load(right_frame);
        let l = left_input.mul(gain_left).andnot(mute_left);
        let r = right_input.mul(gain_right).andnot(mute_right);
        let yl = L::select(matrix.identity, l, matrix.ll.mul(l).add(matrix.lr.mul(r)));
        let yr = L::select(matrix.identity, r, matrix.rl.mul(l).add(matrix.rr.mul(r)));
        yl.store(left_frame);
        yr.store(right_frame);
    }
}

/// [`fader_matrix_block`] for a coefficient set with **no** identity lane: its second arm alone.
///
/// Why it exists: [`fader_matrix_block`] evaluates the per-lane identity select on both outputs of
/// every frame. With no identity lane both selects return their second arm, so the select is pure
/// cost: on `x86-64-v3` two `vblendvps` per frame that compete with the six multiplies for the same
/// ports, and on wasm `simd128` two `v128.bitselect` per frame, about 1.1 us of a 64-track
/// gain/pan block under V8 (issue #954). No standing session has an identity lane in a full bank:
/// the equal-power pan law never yields one, because `cos(pi / 2)` rounds to `6.1e-17` in `f32`.
///
/// Frozen operation order, per frame -- [`fader_matrix_block`]'s second arm verbatim:
/// 1. `l = load(left) * gain_left`, `r = load(right) * gain_right`, then clear each muted lane
/// 2. `yl = ll * l + lr * r` and `yr = rl * l + rr * r` -- multiply, multiply, add; no fusion
/// 3. store both planes
///
/// Both input planes are loaded before either is written; the operand orders are
/// [`fader_matrix_block`]'s. There is no `L::select` anywhere in the body, not even one on a
/// constant mask: a select is what brings the blend back.
///
/// Precondition: `!L::mask_any(matrix.identity)` over all `L::WIDTH` lanes, padding lanes included,
/// checked by a `debug_assert!`. The caller tests it once per call, never per frame.
///
/// Class A: with no identity lane, [`fader_matrix_block`]'s per-lane select returns its second arm,
/// so the two kernels compute the same products and sums on every lane of every word, padding
/// included, and so does [`gain_mute_block`] on each plane followed by
/// [`matrix2x2_block_without_identity`]. Every non-NaN word is bit-identical. A NaN word stays a
/// NaN: LLVM may commute the commutative `fadd` differently in the two bodies, and when both
/// products are NaN with different payloads x86 keeps the first operand's. Rendered plans never
/// reach that case -- the input stage sanitises every non-finite sample before the fader -- and the
/// payload is not part of the class-A statement.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fader_matrix_block_without_identity<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    gain_left: L,
    mute_left: L::Mask,
    gain_right: L,
    mute_right: L::Mask,
    matrix: &Matrix2x2Coef<L>,
) {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    debug_assert!(
        !L::mask_any(matrix.identity),
        "the select-free fused arm requires a coefficient set with no identity lane"
    );
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        let left_input = L::load(left_frame);
        let right_input = L::load(right_frame);
        let l = left_input.mul(gain_left).andnot(mute_left);
        let r = right_input.mul(gain_right).andnot(mute_right);
        let yl = matrix.ll.mul(l).add(matrix.lr.mul(r));
        let yr = matrix.rl.mul(l).add(matrix.rr.mul(r));
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
/// 3. `current[i] = select(done, target[i], ramp_toward(current[i], step[i], target[i]))` for
///    `i` in `0..4` -- no word passes its target ([`ramp_toward`])
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
            *current = L::select(done, *target, ramp_toward(*current, *step, *target));
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

/// `true` when one channel of an input chain needs the armed form of its body for a block of
/// `frames` (issue #1328): some lane whose silence counter `run` can arm within the block holds a
/// non-zero integrator word in either section ([`silence_armable_holding`], which proves the
/// unarmed form gives the same bits otherwise). A padding lane, or a track whose input has been
/// silent since its tail rested, therefore leaves the bank on the unarmed form.
#[inline(always)]
fn channel_arms<L: Lane>(run: L, frames: usize, armed_after: L, state: &[SvfState<L>; 2]) -> bool {
    silence_armable_holding(run, frames, armed_after, svf_state_held(*state))
}

/// The prepared coefficients of one dual-mono input chain, for [`input_chain_block`].
#[derive(Clone, Copy)]
pub struct InputChainCoef<L: Lane> {
    /// Trim with the polarity inversion folded in, `[left, right]`.
    pub trim: [L; 2],
    /// `[channel][section]`, section `0` applied first.
    pub section: [[SvfCoef<L>; 2]; 2],
    /// `N_SILENCE` at the chain's rate on every lane ([`crate::silence_frames`], issue #1328
    /// amendment A9): the input's run of zero frames that arms both sections' joint flush.
    pub silence: L,
    /// The bodies' vector constants, carried as words ([`InputChainConstants`]).
    pub constants: InputChainConstants<L>,
}

/// The vector constants of the input chain bodies: [`NONFINITE_LIMIT`], `1.0` (the sanitise and
/// silence counters' step), [`crate::FLUSH_EPS`] and [`crate::REST_EPS`] on every lane.
///
/// Carried in the prepared coefficients and loaded once per block rather than splatted in the
/// bodies, as the filter-ramp countdown is ([`INPUT_FILTER_LEADING_UPDATES`]): on Apple targets
/// LLVM stores a splatted constant through a `memset_pattern16` call inside the render function
/// (known defect #1018, whose per-crate ceilings `check-cross-targets.sh` holds), and each chain
/// body runs in two copies, one for blocks in which some lane's silence counter can arm the joint
/// flush and one for every other block (issue #1328, amendment A9). The words are the constants;
/// [`InputChainConstants::new`] is the only constructor a caller should use.
#[derive(Clone, Copy)]
pub struct InputChainConstants<L: Lane> {
    /// [`NONFINITE_LIMIT`].
    pub limit: L,
    /// `1.0`.
    pub one: L,
    /// [`crate::FLUSH_EPS`].
    pub flush_eps: L,
    /// [`crate::REST_EPS`].
    pub rest_eps: L,
}

impl<L: Lane> InputChainConstants<L> {
    /// The four constants on every lane.
    #[must_use]
    pub fn new() -> Self {
        Self {
            limit: L::splat(NONFINITE_LIMIT),
            one: L::splat(1.0),
            flush_eps: L::splat(crate::FLUSH_EPS),
            rest_eps: L::splat(REST_EPS),
        }
    }
}

impl<L: Lane> Default for InputChainConstants<L> {
    fn default() -> Self {
        Self::new()
    }
}

/// The retained state of one dual-mono input chain: the integrators, indexed like
/// [`InputChainCoef::section`], and the input's silence counter per channel.
#[derive(Clone, Copy)]
pub struct InputChainState<L: Lane> {
    /// `[channel][section]`.
    pub section: [[SvfState<L>; 2]; 2],
    /// `[channel]`: the run of exactly-zero input frames per lane, [`crate::silence_step`]'s
    /// counter (issue #1328, amendment A9). The chain's input -- the sample each body loads,
    /// before sanitising and trim -- is the effect input both sections' joint flush is armed by.
    /// Every body advances it, the all-identity ones included, so it is the same word whichever
    /// shape the elision plan chose.
    pub silence: [L; 2],
}

impl<L: Lane> Default for InputChainState<L> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            section: [[SvfState::default(); 2]; 2],
            silence: [L::zero(); 2],
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
/// the D7 per-lane non-finite check (one ordered compare and one mask OR per frame) for step 4.
/// The intermediate value that used to be stored and reloaded between passes is now kept in a
/// register, which is exact, and the counter and mask accumulations keep their per-frame order.
/// This is a scheduling change, not a numeric one (master plan §8 class A).
///
/// Frozen operation order, per frame and per channel `ch`:
/// 1. `x = load(frame)`; `rest = silence_step(x, silence[ch], c.silence)` -- the chain input's silence
///    counter and the frame's rest threshold (issue #1328, amendment A9)
/// 2. `bad = !(|x| < NONFINITE_LIMIT)`; `sanitized[ch] = sanitized[ch] + select(bad, 1.0, 0.0)`
/// 3. `v = andnot(x, bad) * trim[ch]`
/// 4. `v = svf_step(v, section[ch][0], rest)`, then `v = svf_step(v, section[ch][1], rest)`
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
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0])
        || channel_arms(s.silence[1], frames, c.silence, &s.section[1])
    {
        input_chain_block_body::<L>(true, left, right, frames, c, s)
    } else {
        silence_skip_block(left, frames, &mut s.silence[0]);
        silence_skip_block(right, frames, &mut s.silence[1]);
        input_chain_block_body::<L>(false, left, right, frames, c, s)
    }
}

/// The body of [`input_chain_block`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_block_body<L: Lane>(
    armable: bool,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;

    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    // The four integrator pairs and the four negated damping coefficients live in registers for the
    // whole block; `svf_step` documents that its state must be a local copy for exactly this
    // reason, or it would be reloaded from memory every frame (D10).
    let mut state = s.section;
    let mut silence = s.silence;
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
            let rest = if armable {
                silence_step_hoisted(x, &mut silence[channel], armed_after, one, rest_on)
            } else {
                zero
            };
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let mut v = x.andnot(bad).mul(c.trim[channel]);
            for section in 0..2 {
                let coefficient = &c.section[channel][section];
                let v0 = v;
                let (v1, v2) = svf_step_when::<L>(
                    armable,
                    flush_eps,
                    v0,
                    nc1[channel][section],
                    coefficient.a2,
                    coefficient.a3,
                    rest,
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
    s.silence = silence;
    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}

/// State of a ramping input trim, one set per lane and channel (#210 phase 3).
///
/// Every array is `[channel]`, and each element carries one lane word: `current[0]` is the left
/// channel's per-lane trim applied to the frame being rendered. The shape is
/// [`GainMuteRamp`]'s, minus the mute mask -- the trim has no mute endpoint -- and the arithmetic
/// is the same D11 form, which is what makes a trim ramp obey the same smoother law the fader and
/// the matrix already obey.
#[derive(Clone, Copy)]
pub struct InputTrimRamp<L: Lane> {
    /// The trim applied to the current frame, `[channel]`.
    pub current: [L; 2],
    /// The trim assigned exactly on the last ramping frame (D11: the snap is an assignment).
    pub target: [L; 2],
    /// Per-sample increment, `(target - start) / n`, computed once per event.
    pub step: [L; 2],
    /// Frames left in this lane's ramp, as an exact `f32` integer (the caller clamps to `2^24`).
    pub remaining: [L; 2],
}

/// [`input_chain_block`] with the trim stepping per sample under the D11 linear law.
///
/// # Why the ramping body ignores the elision plan, and why that is class A
///
/// This is [`input_chain_block`] -- the *unelided* body -- with step 3's constant trim replaced by
/// a ramp word. It is deliberately not given the three-shape dispatch
/// [`input_chain_block_elided`] has, and the reason is [`section_is_identity`]'s own derivation:
/// an elided section is elided exactly when its six coefficient words are the identity and both
/// its integrators are `+0.0`, and over such a section the unelided body computes `v |-> v + 0.0`
/// and writes `+0.0` back into both integrators. That is the same map, the same output bits and
/// the same retained words the elided form produces, which is what
/// `crates/lane/tests/input_chain_elision.rs` proves at every width and every section
/// pattern. So a ramping block renders the elision-planned bits whether or not it takes the
/// elision-planned *path*, and the plan it would have consulted is left untouched and still valid
/// for the settled blocks on either side of the ramp.
///
/// The cost of not eliding is paid only while a ramp is in flight -- a transient of at most one
/// smoothing window per admitted command -- and it buys three fewer kernel bodies to keep
/// bit-identical to three others.
///
/// Frozen operation order, per frame and per channel `ch`:
/// 1. `remaining[ch] = remaining[ch] - 1`
/// 2. `done = remaining[ch] <= 0`
/// 3. `trim = select(done, target[ch], ramp_toward(current[ch], step[ch], target[ch]))`;
///    `current[ch] = trim` -- the trim never passes its target ([`ramp_toward`])
/// 4. `x = load(frame)`; `rest = silence_step(x, silence[ch], c.silence)`
/// 5. `bad = !(|x| < NONFINITE_LIMIT)`; `sanitized[ch] = sanitized[ch] + (1.0 & bad)`
/// 6. `v = andnot(x, bad) * trim`
/// 7. `v = svf_step(v, section[ch][0], rest)`, then `v = svf_step(v, section[ch][1], rest)`
/// 8. `nonfinite[ch] = nonfinite[ch] | !(|v| < NONFINITE_LIMIT)`
/// 9. `store(frame, v)`
///
/// Steps 4-9 are [`input_chain_block`]'s steps 1-6 character for character, with its `c.trim[ch]`
/// replaced by the ramp word step 3 produced. Steps 1-3 are [`gain_mute_ramp_block`]'s steps 1-3.
/// `remaining` and `current` are advanced in place, so a lane's ramp evolves by its own additions
/// regardless of block size or of its neighbours: partition and cohort invariance hold by
/// construction, exactly as they do for [`matrix2x2_ramp_block`].
///
/// The left channel's frame is evaluated before the right channel's, as in [`input_chain_block`].
#[inline(always)]
pub fn input_chain_ramp_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0])
        || channel_arms(s.silence[1], frames, c.silence, &s.section[1])
    {
        input_chain_ramp_block_body::<L>(true, left, right, frames, c, s, r)
    } else {
        silence_skip_block(left, frames, &mut s.silence[0]);
        silence_skip_block(right, frames, &mut s.silence[1]);
        input_chain_ramp_block_body::<L>(false, left, right, frames, c, s, r)
    }
}

/// The body of [`input_chain_ramp_block`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_ramp_block_body<L: Lane>(
    armable: bool,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;

    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    let mut state = s.section;
    let mut silence = s.silence;
    let mut remaining = r.remaining;
    let mut current = r.current;
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
            remaining[channel] = remaining[channel].sub(one);
            let done = remaining[channel].le(zero);
            let trim = L::select(
                done,
                r.target[channel],
                ramp_toward(current[channel], r.step[channel], r.target[channel]),
            );
            current[channel] = trim;
            let x = L::load(frame);
            let rest = if armable {
                silence_step_hoisted(x, &mut silence[channel], armed_after, one, rest_on)
            } else {
                zero
            };
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let mut v = x.andnot(bad).mul(trim);
            for section in 0..2 {
                let coefficient = &c.section[channel][section];
                let v0 = v;
                let (v1, v2) = svf_step_when::<L>(
                    armable,
                    flush_eps,
                    v0,
                    nc1[channel][section],
                    coefficient.a2,
                    coefficient.a3,
                    rest,
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
    s.silence = silence;
    r.remaining = remaining;
    r.current = current;
    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}

/// [`input_chain_ramp_block`] over one plane: the collapsed track's live channel.
///
/// The dual body's channel-`0` arm, character for character, with the channel index frozen at `0`
/// and the `1` arm deleted -- the rule stated above the one-plane settled variants, applied to the
/// ramping one. The right channel's ramp words are not advanced here and are not read: the caller
/// duplicates the left channel's whole per-channel state onto them after the block, exactly as it
/// duplicates the report.
#[inline(always)]
pub fn input_chain_ramp_block_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0]) {
        input_chain_ramp_block_mono_body::<L>(true, io, frames, c, s, r)
    } else {
        silence_skip_block(io, frames, &mut s.silence[0]);
        input_chain_ramp_block_mono_body::<L>(false, io, frames, c, s, r)
    }
}

/// The body of [`input_chain_ramp_block_mono`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_ramp_block_mono_body<L: Lane>(
    armable: bool,
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;

    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut state = s.section[0];
    let mut silence = s.silence[0];
    let mut remaining = r.remaining[0];
    let mut current = r.current[0];
    let mut nc1 = [zero; 2];
    for (section, coefficient) in c.section[0].iter().enumerate() {
        nc1[section] = coefficient.c1.neg();
    }

    for frame in io.chunks_exact_mut(L::WIDTH) {
        remaining = remaining.sub(one);
        let done = remaining.le(zero);
        let trim = L::select(
            done,
            r.target[0],
            ramp_toward(current, r.step[0], r.target[0]),
        );
        current = trim;
        let x = L::load(frame);
        let rest = if armable {
            silence_step_hoisted(x, &mut silence, armed_after, one, rest_on)
        } else {
            zero
        };
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(trim);
        for section in 0..2 {
            let coefficient = &c.section[0][section];
            let v0 = v;
            let (v1, v2) = svf_step_when::<L>(
                armable,
                flush_eps,
                v0,
                nc1[section],
                coefficient.a2,
                coefficient.a3,
                rest,
                &mut state[section],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        }
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }

    s.section[0] = state;
    s.silence[0] = silence;
    r.remaining[0] = remaining;
    r.current[0] = current;
    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// A fixed 64-update filter ramp combined with the existing trim ramp.  Filter coefficients are
/// consumed for the current frame and advanced afterwards, so frame A uses the old words and
/// frame A+64 uses the exact target words.  `filter_remaining` is indexed `[channel][section]`;
/// zero-count lanes are harmless and let one bank run its bounded prefix without a per-lane
/// dispatch.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn input_chain_ramp_block_filter<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &mut InputChainCoef<L>,
    s: &mut InputChainState<L>,
    trim: &mut InputTrimRamp<L>,
    trim_ramping: bool,
    filter_target: &[[SvfCoef<L>; 2]; 2],
    filter_step: &[[SvfCoef<L>; 2]; 2],
    filter_remaining: &mut [[L; 2]; 2],
    filter_leading: [[L; 2]; 2],
) -> InputChainReport<L> {
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0])
        || channel_arms(s.silence[1], frames, c.silence, &s.section[1])
    {
        input_chain_ramp_block_filter_body::<L>(
            true,
            left,
            right,
            frames,
            c,
            s,
            trim,
            trim_ramping,
            filter_target,
            filter_step,
            filter_remaining,
            filter_leading,
        )
    } else {
        silence_skip_block(left, frames, &mut s.silence[0]);
        silence_skip_block(right, frames, &mut s.silence[1]);
        input_chain_ramp_block_filter_body::<L>(
            false,
            left,
            right,
            frames,
            c,
            s,
            trim,
            trim_ramping,
            filter_target,
            filter_step,
            filter_remaining,
            filter_leading,
        )
    }
}

/// The body of [`input_chain_ramp_block_filter`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_ramp_block_filter_body<L: Lane>(
    armable: bool,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &mut InputChainCoef<L>,
    s: &mut InputChainState<L>,
    trim: &mut InputTrimRamp<L>,
    trim_ramping: bool,
    filter_target: &[[SvfCoef<L>; 2]; 2],
    filter_step: &[[SvfCoef<L>; 2]; 2],
    filter_remaining: &mut [[L; 2]; 2],
    mut filter_leading: [[L; 2]; 2],
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;
    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    let mut state = s.section;
    let mut silence = s.silence;
    let mut coefficients = c.section;
    let mut trim_current = trim.current;
    let mut trim_remaining = trim.remaining;
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        for (channel, frame) in [left_frame, right_frame].into_iter().enumerate() {
            let trim_value = if trim_ramping {
                trim_remaining[channel] = trim_remaining[channel].sub(one);
                let done = trim_remaining[channel].le(zero);
                let value = L::select(
                    done,
                    trim.target[channel],
                    ramp_toward(
                        trim_current[channel],
                        trim.step[channel],
                        trim.target[channel],
                    ),
                );
                trim_current[channel] = value;
                value
            } else {
                c.trim[channel]
            };
            let x = L::load(frame);
            let rest = if armable {
                silence_step_hoisted(x, &mut silence[channel], armed_after, one, rest_on)
            } else {
                zero
            };
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let mut v = x.andnot(bad).mul(trim_value);
            for section in 0..2 {
                let coefficient = &coefficients[channel][section];
                let v0 = v;
                let (v1, v2) = svf_step_when::<L>(
                    armable,
                    flush_eps,
                    v0,
                    coefficient.c1.neg(),
                    coefficient.a2,
                    coefficient.a3,
                    rest,
                    &mut state[channel][section],
                );
                v = coefficient
                    .m2
                    .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
            }
            nonfinite[channel] = L::mask_or(nonfinite[channel], L::mask_not(v.abs().lt(limit)));
            v.store(frame);

            // Current-then-advance is the coefficient-ramp contract.  Clearing a disabled
            // endpoint occurs after its last old-word sample and before the first identity one.
            for section in 0..2 {
                let remaining = filter_remaining[channel][section].sub(one);
                let done = remaining.le(zero);
                filter_remaining[channel][section] = remaining;
                let lead = filter_leading[channel][section].sub(one);
                filter_leading[channel][section] = lead;
                let leading = zero.le(lead);
                let target = &filter_target[channel][section];
                filter_ramp_words(
                    &mut coefficients[channel][section],
                    target,
                    &filter_step[channel][section],
                    remaining,
                    done,
                    leading,
                    zero,
                );
                let identity = L::mask_and(
                    L::mask_and(current_target_identity(target), done),
                    L::mask_not(no_lanes::<L>()),
                );
                state[channel][section].ic1 = state[channel][section].ic1.andnot(identity);
                state[channel][section].ic2 = state[channel][section].ic2.andnot(identity);
            }
        }
    }
    c.section = coefficients;
    s.section = state;
    s.silence = silence;
    trim.current = trim_current;
    trim.remaining = trim_remaining;
    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}

#[inline(always)]
fn current_target_identity<L: Lane>(target: &SvfCoef<L>) -> L::Mask {
    L::mask_and(
        L::mask_and(target.m0.eq(L::splat(1.0)), target.m1.eq(L::zero())),
        L::mask_and(
            target.m2.eq(L::zero()),
            L::mask_and(
                target.c1.eq(L::zero()),
                L::mask_and(target.a2.eq(L::zero()), target.a3.eq(L::zero())),
            ),
        ),
    )
}

/// Advances one section's six filter ramp words after a frame (#1407).
///
/// The first [`INPUT_FILTER_LEADING_UPDATES`] words of a ramp step from the current word,
/// `current + step`; every later word is computed from the target, the step and the countdown left
/// after this frame, `target - step * remaining`, never from the previous word. So a ramp
/// accumulates at most four additions, and every word is within a fixed rounding of the line from
/// the word the ramp started at to its target.
///
/// Why the split, and why at four: a word computed from the target carries a rounding of the
/// whole remaining distance `step * remaining`, which near the start of a ramp is nearly the whole
/// `target - start`; a word stepped from the start carries one rounding of the word per step
/// taken. A host that restarts a ramp every `q` frames compounds the error of its restart word
/// `64 / q` times (the contraction of the old start's error by `1 - q / 64` per restart), and
/// stepping the first four words is what keeps that compounded error at its floor, `64` half-ulps
/// of the word, at every block size: past four steps the target-relative word is the smaller
/// one. The bound and its proof are in `docs/rulings/builtins-input-liveness-d2.md`.
///
/// A recursion word (`c1`, `a2`, `a3`) whose step is zero holds its current word until the
/// completion snap. That is how rule 2 of the live retarget law (a disable) freezes the recursion
/// at its current words while only the mix ramps: the owner writes a `+0.0` step, and the
/// target-relative form alone would jump the word to the identity's zero. Every other zero step
/// belongs to a word already equal to its target (rule 3's jumped recursion, or a word a rule-4
/// retarget does not move), so the hold changes nothing for it. The mix words never freeze, so
/// they take no hold.
///
/// Partition-invariant by construction: the word depends only on the countdown, which the owner
/// reloads exactly from its integer countdown at the top of every ramping block, and on the
/// current word, which the owner keeps across blocks.
#[inline(always)]
fn filter_ramp_words<L: Lane>(
    current: &mut SvfCoef<L>,
    target: &SvfCoef<L>,
    step: &SvfCoef<L>,
    remaining: L,
    done: L::Mask,
    leading: L::Mask,
    zero: L,
) {
    let word = |current: L, target: L, step: L| {
        L::select(
            done,
            target,
            L::select(leading, current.add(step), target.sub(step.mul(remaining))),
        )
    };
    let held = |current: L, target: L, step: L| {
        let hold = L::mask_and(step.eq(zero), L::mask_not(done));
        L::select(hold, current, word(current, target, step))
    };
    current.c1 = held(current.c1, target.c1, step.c1);
    current.a2 = held(current.a2, target.a2, step.a2);
    current.a3 = held(current.a3, target.a3, step.a3);
    current.m0 = word(current.m0, target.m0, step.m0);
    current.m1 = word(current.m1, target.m1, step.m1);
    current.m2 = word(current.m2, target.m2, step.m2);
}

/// How many of a filter ramp's 64 updates step from the current word (#1407).
///
/// The owner hands the filter-ramp bodies each lane's leading countdown, `remaining - 60` before
/// the block (`remaining` its integer ramp countdown), and a frame steps from the current word
/// while that countdown, decremented with the ramp's, is still at least zero: the frames that leave
/// a ramp countdown of `63` down to `60`. Carried as a per-lane word rather than compared against a
/// splatted `60.0` so the bodies hold no new vector constant: on Apple targets each one is a
/// `memset_pattern16` call in the render function (known defect #1018).
pub const INPUT_FILTER_LEADING_UPDATES: u32 = 4;

/// Mono-collapse form of [`input_chain_ramp_block_filter`].  Only channel zero advances; the
/// owner mirrors its complete filter and trim records onto channel one after the block.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn input_chain_ramp_block_filter_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &mut InputChainCoef<L>,
    s: &mut InputChainState<L>,
    trim: &mut InputTrimRamp<L>,
    trim_ramping: bool,
    filter_target: &[[SvfCoef<L>; 2]; 2],
    filter_step: &[[SvfCoef<L>; 2]; 2],
    filter_remaining: &mut [[L; 2]; 2],
    filter_leading: [[L; 2]; 2],
) -> InputChainReport<L> {
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0]) {
        input_chain_ramp_block_filter_mono_body::<L>(
            true,
            io,
            frames,
            c,
            s,
            trim,
            trim_ramping,
            filter_target,
            filter_step,
            filter_remaining,
            filter_leading,
        )
    } else {
        silence_skip_block(io, frames, &mut s.silence[0]);
        input_chain_ramp_block_filter_mono_body::<L>(
            false,
            io,
            frames,
            c,
            s,
            trim,
            trim_ramping,
            filter_target,
            filter_step,
            filter_remaining,
            filter_leading,
        )
    }
}

/// The body of [`input_chain_ramp_block_filter_mono`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_ramp_block_filter_mono_body<L: Lane>(
    armable: bool,
    io: &mut [f32],
    frames: usize,
    c: &mut InputChainCoef<L>,
    s: &mut InputChainState<L>,
    trim: &mut InputTrimRamp<L>,
    trim_ramping: bool,
    filter_target: &[[SvfCoef<L>; 2]; 2],
    filter_step: &[[SvfCoef<L>; 2]; 2],
    filter_remaining: &mut [[L; 2]; 2],
    mut filter_leading: [[L; 2]; 2],
) -> InputChainReport<L> {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;
    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut state = s.section[0];
    let mut silence = s.silence[0];
    let mut coefficients = c.section[0];
    let mut trim_current = trim.current[0];
    let mut trim_remaining = trim.remaining[0];
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let trim_value = if trim_ramping {
            trim_remaining = trim_remaining.sub(one);
            let done = trim_remaining.le(zero);
            let value = L::select(
                done,
                trim.target[0],
                ramp_toward(trim_current, trim.step[0], trim.target[0]),
            );
            trim_current = value;
            value
        } else {
            c.trim[0]
        };
        let x = L::load(frame);
        let rest = if armable {
            silence_step_hoisted(x, &mut silence, armed_after, one, rest_on)
        } else {
            zero
        };
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(trim_value);
        for section in 0..2 {
            let coefficient = &coefficients[section];
            let v0 = v;
            let (v1, v2) = svf_step_when::<L>(
                armable,
                flush_eps,
                v0,
                coefficient.c1.neg(),
                coefficient.a2,
                coefficient.a3,
                rest,
                &mut state[section],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        }
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
        for section in 0..2 {
            let remaining = filter_remaining[0][section].sub(one);
            let done = remaining.le(zero);
            filter_remaining[0][section] = remaining;
            let lead = filter_leading[0][section].sub(one);
            filter_leading[0][section] = lead;
            let leading = zero.le(lead);
            let target = &filter_target[0][section];
            filter_ramp_words(
                &mut coefficients[section],
                target,
                &filter_step[0][section],
                remaining,
                done,
                leading,
                zero,
            );
            let identity = L::mask_and(current_target_identity(target), done);
            state[section].ic1 = state[section].ic1.andnot(identity);
            state[section].ic2 = state[section].ic2.andnot(identity);
        }
    }
    c.section[0] = coefficients;
    s.section[0] = state;
    s.silence[0] = silence;
    trim.current[0] = trim_current;
    trim.remaining[0] = trim_remaining;
    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// Dispatch a trim-only ramp through the all-identity body when the settled plan proves that no
/// SVF recurrence is needed.  The wrapper performs its one shape choice before the frame loop.
#[inline(always)]
pub fn input_chain_ramp_block_elided<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    if plan.elided == [[true, true], [true, true]] {
        // No section runs: the input's silence counter advances without a rest plane.
        silence_skip_block(left, frames, &mut s.silence[0]);
        silence_skip_block(right, frames, &mut s.silence[1]);
        identity_chain_ramp_block(left, right, frames, r)
    } else {
        input_chain_ramp_block(left, right, frames, c, s, r)
    }
}

#[inline(always)]
fn identity_chain_ramp_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    #[cfg(test)]
    IDENTITY_RAMP_DISPATCHES.with(|count| count.set(count.get() + 1));
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    let mut current = r.current;
    let mut remaining = r.remaining;
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        for (channel, frame) in [left_frame, right_frame].into_iter().enumerate() {
            remaining[channel] = remaining[channel].sub(one);
            let done = remaining[channel].le(zero);
            let trim = L::select(
                done,
                r.target[channel],
                ramp_toward(current[channel], r.step[channel], r.target[channel]),
            );
            current[channel] = trim;
            let x = L::load(frame);
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let v = x.andnot(bad).mul(trim).add(zero);
            nonfinite[channel] = L::mask_or(nonfinite[channel], L::mask_not(v.abs().lt(limit)));
            v.store(frame);
        }
    }
    r.current = current;
    r.remaining = remaining;
    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}

/// Mono-collapse wrapper for the all-identity trim-ramp path.
#[inline(always)]
pub fn input_chain_ramp_block_mono_elided<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    r: &mut InputTrimRamp<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    if plan.elided[0] == [true, true] {
        silence_skip_block(io, frames, &mut s.silence[0]);
        identity_chain_ramp_block_mono(io, frames, r)
    } else {
        input_chain_ramp_block_mono(io, frames, c, s, r)
    }
}

#[inline(always)]
fn identity_chain_ramp_block_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    r: &mut InputTrimRamp<L>,
) -> InputChainReport<L> {
    #[cfg(test)]
    IDENTITY_RAMP_DISPATCHES.with(|count| count.set(count.get() + 1));
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut current = r.current[0];
    let mut remaining = r.remaining[0];
    for frame in io.chunks_exact_mut(L::WIDTH) {
        remaining = remaining.sub(one);
        let done = remaining.le(zero);
        let trim = L::select(
            done,
            r.target[0],
            ramp_toward(current, r.step[0], r.target[0]),
        );
        current = trim;
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let v = x.andnot(bad).mul(trim).add(zero);
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }
    r.current[0] = current;
    r.remaining[0] = remaining;
    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// The bit pattern of `+1.0`, the direct-mix word of a disabled builtin section.
const IDENTITY_M0_BITS: u32 = 0x3F80_0000;
/// The bit pattern of `+0.0`, which every other identity word and both identity state words carry.
const IDENTITY_ZERO_BITS: u32 = 0x0000_0000;

/// True when every lane of `value` carries exactly `pattern`.
///
/// **Bit patterns, not `==`.** A float compare would call `-0.0` equal to `+0.0`, and the whole
/// point of [`section_is_identity`] is that a `-0.0` retained word is *not* inert: it makes the
/// section emit `-0.0` where the elided form emits `+0.0`. The comparison is therefore on
/// [`Lane::store_bits`] words, and the answer is a control-plane `bool`.
#[inline]
fn every_lane_is<L: Lane>(value: L, pattern: u32) -> bool {
    let mut words = [0_u32; 64];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].iter().all(|word| *word == pattern)
}

/// True when one section of a prepared chain is the arithmetic identity in **every** lane, state
/// included, and its whole contribution is therefore exactly one `add(+0.0)`.
///
/// # The map, and why the state words are part of the test
///
/// With `m0 = +1.0`, `m1 = m2 = c1 = a2 = a3 = +0.0` and both integrators at `+0.0`,
/// [`super::svf_step`] holds both integrators at `+0.0` for every input, and the section's output
/// mix collapses to `v |-> v + 0.0` — the map that sends `-0.0` to `+0.0` and fixes every other
/// value. Nonfinites cannot reach it: [`sanitize_gain_block`]'s clear runs first and the trim
/// domain is bounded.
///
/// One intermediate is *not* `+0.0`, and the derivation must not claim otherwise: `nc1 = neg(+0.0)`
/// is `-0.0`, so `d1 = fma(-0.0, +0.0, +0.0 * v3)` is `-0.0` whenever `v3` is negative or `-0.0`.
/// The conclusion survives it — `v1 = +0.0 + (-0.0) = +0.0` and `ic1' = flush(+0.0 + (-0.0)) =
/// +0.0` — because `+0` absorbs `-0` under round-to-nearest.
///
/// The state words are checked for the same reason they are checked *bitwise*: identity
/// coefficients over a `-0.0` integrator emit `-0.0`, which the elided form would have washed to
/// `+0.0`. That is an observable divergence, and `-0.0 == 0.0` would have admitted it.
///
/// All lanes or none: the kernel is a vector body with no per-lane branch, so a bank elides a
/// section only when every lane of it — padding lanes included — carries the identity.
#[inline]
pub fn section_is_identity<L: Lane>(c: &SvfCoef<L>, s: &SvfState<L>) -> bool {
    every_lane_is::<L>(c.m0, IDENTITY_M0_BITS)
        && every_lane_is::<L>(c.m1, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(c.m2, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(c.c1, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(c.a2, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(c.a3, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(s.ic1, IDENTITY_ZERO_BITS)
        && every_lane_is::<L>(s.ic2, IDENTITY_ZERO_BITS)
}

/// Which sections of a prepared input chain [`input_chain_block_elided`] may skip.
///
/// Decided once, by [`input_chain_plan`], at the point the coefficient and state words are written
/// — never per call.
///
/// # Why a `true` entry cannot go stale
///
/// The predicate reads **exactly eight words per section**: the six `SvfCoef` words and the two
/// `SvfState` integrators ([`section_is_identity`]). Nothing else. So the only parameters that can
/// invalidate a decision are the ones that design those eight, and those are `hpf_hz` and
/// `lpf_hz`, which are `PreparedOnly` in the builtin parameter ABI: the coefficients are written
/// once, at preparation. An elided section's integrators are never written by the render path, so
/// the only way out of the identity is an explicit state write, and both of those
/// (`InputStage::reset`, `InputStage::set_lane_state_words`) recompute the plan.
///
/// `trim` is deliberately **not** in that list, and since #210 phase 3 it is a live word --
/// `trim_db` and `polarity_invert` retarget it after preparation. The decision is unaffected,
/// because a section is the arithmetic identity or it is not regardless of what the chain
/// multiplies its input by: the trim word is consumed one step earlier, at
/// [`input_chain_block`]'s step 3, and is not one of the eight this predicate loads. The elision
/// plan and the live trim are orthogonal, and the ramping kernel
/// [`input_chain_ramp_block`] leaves the plan untouched for exactly that reason.
///
/// **The proviso.** If `hpf_hz` or `lpf_hz` ever become live, the premise above fails at the word
/// list this comment names, and that liveness *requires* a command-driven invalidation: the
/// retarget must recompute the plan, the way the two state writes already do. That is the
/// obligation the D2 ruling records
/// (`docs/rulings/builtins-input-liveness-d2.md`), and it is a hook, not a comment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputChainPlan {
    /// `[channel][section]`, indexed like [`InputChainCoef::section`]; `true` means elided.
    pub elided: [[bool; 2]; 2],
}

impl InputChainPlan {
    /// The plan that elides nothing, which is what every prepared chain that carries a real filter
    /// gets.
    pub const NONE: Self = Self {
        elided: [[false; 2]; 2],
    };
}

/// Decides [`InputChainPlan`] for one prepared chain from its coefficient and state words.
#[inline]
pub fn input_chain_plan<L: Lane>(c: &InputChainCoef<L>, s: &InputChainState<L>) -> InputChainPlan {
    let mut plan = InputChainPlan::NONE;
    for (channel, elided) in plan.elided.iter_mut().enumerate() {
        for (section, elided) in elided.iter_mut().enumerate() {
            *elided = section_is_identity::<L>(
                &c.section[channel][section],
                &s.section[channel][section],
            );
        }
    }
    plan
}

/// [`input_chain_block`] with the sections its `plan` marks identity replaced by the one
/// `add(+0.0)` they compose to.
///
/// # Why this is class A
///
/// A run of `N` consecutive identity sections is the map `v |-> v + 0.0` composed `N` times, and
/// that map is idempotent, so the run is exactly one `add(+0.0)` — see [`section_is_identity`] for
/// the derivation and for why the state words are part of the test. The add is emitted **at the
/// run's position** in the chain, because the washing of `-0.0` does not commute with a real
/// section: an identity high-pass followed by a real low-pass must feed the low-pass `v + 0.0`,
/// not `v`.
///
/// # The three shapes
///
/// * nothing elided — the call is [`input_chain_block`] itself, unchanged, so a chain that carries
///   a real filter pays nothing for this feature;
/// * every section elided — the whole chain is sanitise, trim, one add and the boundary scan, with
///   no recurrence and no state traffic at all;
/// * mixed — the frame body of [`input_chain_block`] with the run collapsed at its position.
///
/// The retained state of an elided section is not written: it is `+0.0` by the plan's own test and
/// the section that would have written it is gone, so it stays `+0.0`.
#[inline(always)]
pub fn input_chain_block_elided<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    match plan.elided {
        [[false, false], [false, false]] => input_chain_block(left, right, frames, c, s),
        [[true, true], [true, true]] => {
            // No section runs: the input's silence counter advances without a rest plane.
            silence_skip_block(left, frames, &mut s.silence[0]);
            silence_skip_block(right, frames, &mut s.silence[1]);
            identity_chain_block(left, right, frames, c)
        }
        _ => mixed_chain_block(left, right, frames, c, s, plan),
    }
}

/// The chain of a bank whose four sections are all the identity: no recurrence, no state.
///
/// Frozen operation order, per frame and per channel `ch`: steps 1-3 of [`input_chain_block`]
/// (step 1's silence counter included: no section consumes the threshold here, but the counter is
/// the chain input's and advances whichever body runs), then `v = v + 0.0` for the run of two
/// identity sections, then its steps 5 and 6.
#[inline(always)]
fn identity_chain_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
    for (left_frame, right_frame) in left
        .chunks_exact_mut(L::WIDTH)
        .zip(right.chunks_exact_mut(L::WIDTH))
    {
        for (channel, frame) in [left_frame, right_frame].into_iter().enumerate() {
            let x = L::load(frame);
            let bad = L::mask_not(x.abs().lt(limit));
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let v = x.andnot(bad).mul(c.trim[channel]).add(zero);
            nonfinite[channel] = L::mask_or(nonfinite[channel], L::mask_not(v.abs().lt(limit)));
            v.store(frame);
        }
    }

    InputChainReport {
        sanitized: count,
        nonfinite,
    }
}

/// The chain of a bank where some sections are the identity and some are not.
///
/// [`input_chain_block`]'s frame body, with each maximal run of elided sections replaced by one
/// `add(+0.0)` at the run's position: before the real section that follows it, or after the real
/// section that precedes it when the run ends the chain.
#[inline(always)]
fn mixed_chain_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    debug_assert_eq!(left.len(), frames * L::WIDTH);
    debug_assert_eq!(right.len(), frames * L::WIDTH);
    // Each channel owns disjoint buffers, coefficients, integrators and report accumulators.
    // Lane arithmetic and svf_step have no fallible step or observable side effect. Completing
    // one channel first therefore preserves every per-channel operation; caller state is still
    // published once, after both channels finish, exactly as in the interleaved mixed body.
    let mut state = s.section;
    let mut silence = s.silence;
    let (left_count, left_nonfinite) = dispatch_mixed_channel(
        left,
        c.trim[0],
        &c.section[0],
        &mut state[0],
        &mut silence[0],
        c.silence,
        &c.constants,
        plan.elided[0],
    );
    let (right_count, right_nonfinite) = dispatch_mixed_channel(
        right,
        c.trim[1],
        &c.section[1],
        &mut state[1],
        &mut silence[1],
        c.silence,
        &c.constants,
        plan.elided[1],
    );
    s.section = state;
    s.silence = silence;
    InputChainReport {
        sanitized: [left_count, right_count],
        nonfinite: [left_nonfinite, right_nonfinite],
    }
}

/// Select one of the four immutable two-section shapes before its channel's frame loop.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn dispatch_mixed_channel<L: Lane>(
    io: &mut [f32],
    trim: L,
    coefficients: &[SvfCoef<L>; 2],
    state: &mut [SvfState<L>; 2],
    silence: &mut L,
    armed_after: L,
    constants: &InputChainConstants<L>,
    shape: [bool; 2],
) -> (L, L::Mask) {
    #[cfg(test)]
    MIXED_PLAN_SELECTIONS.with(|count| count.set(count.get() + 1));
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    let frames = io.len() / L::WIDTH;
    if channel_arms(*silence, frames, armed_after, state) {
        dispatch_mixed_shape::<L>(
            true,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
            shape,
        )
    } else {
        silence_skip_block(io, frames, silence);
        dispatch_mixed_shape::<L>(
            false,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
            shape,
        )
    }
}

/// [`dispatch_mixed_channel`]'s shape choice, at a known `ARMABLE`.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn dispatch_mixed_shape<L: Lane>(
    armable: bool,
    io: &mut [f32],
    trim: L,
    coefficients: &[SvfCoef<L>; 2],
    state: &mut [SvfState<L>; 2],
    silence: &mut L,
    armed_after: L,
    constants: &InputChainConstants<L>,
    shape: [bool; 2],
) -> (L, L::Mask) {
    match shape {
        [false, false] => mixed_channel_block::<L, false, false>(
            armable,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
        ),
        [true, false] => mixed_channel_block::<L, true, false>(
            armable,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
        ),
        [false, true] => mixed_channel_block::<L, false, true>(
            armable,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
        ),
        [true, true] => mixed_channel_block::<L, true, true>(
            armable,
            io,
            trim,
            coefficients,
            state,
            silence,
            armed_after,
            constants,
        ),
    }
}

/// No runtime plan reaches this body. The const arms place one add at an identity run's
/// original position; the both-elided instantiation adds once and touches no integrator.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn mixed_channel_block<L: Lane, const ELIDE_HPF: bool, const ELIDE_LPF: bool>(
    armable: bool,
    io: &mut [f32],
    trim: L,
    coefficients: &[SvfCoef<L>; 2],
    state: &mut [SvfState<L>; 2],
    silence: &mut L,
    armed_after: L,
    constants: &InputChainConstants<L>,
) -> (L, L::Mask) {
    let limit = constants.limit;
    let one = constants.one;
    let rest_on = constants.rest_eps;
    let flush_eps = constants.flush_eps;
    let zero = L::zero();
    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let nc1 = [coefficients[0].c1.neg(), coefficients[1].c1.neg()];
    let mut counted = *silence;
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let rest = if armable {
            silence_step_hoisted(x, &mut counted, armed_after, one, rest_on)
        } else {
            zero
        };
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(trim);
        if ELIDE_HPF {
            v = v.add(zero);
        } else {
            let coefficient = &coefficients[0];
            let v0 = v;
            let (v1, v2) = svf_step_when::<L>(
                armable,
                flush_eps,
                v0,
                nc1[0],
                coefficient.a2,
                coefficient.a3,
                rest,
                &mut state[0],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        }
        if !ELIDE_LPF {
            let coefficient = &coefficients[1];
            let v0 = v;
            let (v1, v2) = svf_step_when::<L>(
                armable,
                flush_eps,
                v0,
                nc1[1],
                coefficient.a2,
                coefficient.a3,
                rest,
                &mut state[1],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        } else if !ELIDE_HPF {
            v = v.add(zero);
        }
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }
    *silence = counted;
    (count, nonfinite)
}

// ---------------------------------------------------------------------------------------------
// The mono-collapse one-plane variants.
//
// A collapsed track computes one channel and the strip duplicates it at the fader/matrix seam, so
// these are the same three shapes as above with only channel 0 processed. The mixed variant
// shares the specialized per-channel body with its dual counterpart; the other two retain their
// peeled frame bodies. No operation is reassociated and no per-channel accumulation changes
// order: the two channels have independent arithmetic, so omitting channel 1 cannot move channel
// 0's bits.
//
// The report is filled for **both** channels, because the collapsed track's right plane is the
// duplicated left one and its accounting is therefore the left one's: a caller that read
// `sanitized[1]` off a collapsed block would otherwise see a zero where the dual run counted.
// ---------------------------------------------------------------------------------------------

/// [`input_chain_block`] over one plane: the collapsed track's live channel.
///
/// Frozen operation order: [`input_chain_block`]'s, with `ch = 0` and no second channel.
#[inline(always)]
pub fn input_chain_block_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> InputChainReport<L> {
    // Issue #1328 amendment A9: the joint rule's arithmetic runs only on a block it can act on
    // ([`channel_arms`]).
    if channel_arms(s.silence[0], frames, c.silence, &s.section[0]) {
        input_chain_block_mono_body::<L>(true, io, frames, c, s)
    } else {
        silence_skip_block(io, frames, &mut s.silence[0]);
        input_chain_block_mono_body::<L>(false, io, frames, c, s)
    }
}

/// The body of [`input_chain_block_mono`]; `ARMABLE = false` runs no silence counter and the per-word
/// flush ([`super::svf_step_armable`]), for a block in which no lane's counter can arm.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn input_chain_block_mono_body<L: Lane>(
    armable: bool,
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = c.constants.limit;
    let one = c.constants.one;
    let zero = L::zero();
    let armed_after = c.silence;
    let rest_on = c.constants.rest_eps;
    let flush_eps = c.constants.flush_eps;

    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut state = s.section[0];
    let mut silence = s.silence[0];
    let mut nc1 = [zero; 2];
    for (section, coefficient) in c.section[0].iter().enumerate() {
        nc1[section] = coefficient.c1.neg();
    }

    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let rest = if armable {
            silence_step_hoisted(x, &mut silence, armed_after, one, rest_on)
        } else {
            zero
        };
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(c.trim[0]);
        for section in 0..2 {
            let coefficient = &c.section[0][section];
            let v0 = v;
            let (v1, v2) = svf_step_when::<L>(
                armable,
                flush_eps,
                v0,
                nc1[section],
                coefficient.a2,
                coefficient.a3,
                rest,
                &mut state[section],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        }
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }

    s.section[0] = state;
    s.silence[0] = silence;
    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// [`identity_chain_block`] over one plane.
#[inline(always)]
fn identity_chain_block_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
) -> InputChainReport<L> {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let v = x.andnot(bad).mul(c.trim[0]).add(zero);
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }

    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// [`mixed_chain_block`] over one plane.
#[inline(always)]
fn mixed_chain_block_mono<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let mut state = s.section[0];
    let mut silence = s.silence[0];
    let (count, nonfinite) = dispatch_mixed_channel(
        io,
        c.trim[0],
        &c.section[0],
        &mut state,
        &mut silence,
        c.silence,
        &c.constants,
        plan.elided[0],
    );
    s.section[0] = state;
    s.silence[0] = silence;
    InputChainReport {
        sanitized: [count; 2],
        nonfinite: [nonfinite; 2],
    }
}

/// [`input_chain_block_elided`] over one plane: the collapsed track's whole input chain.
///
/// The plan is read at **channel `0` only**, which is the collapsed channel's own plan and
/// therefore exactly the plan the dual body would have taken for it. The two channels' plans agree
/// whenever the chain is collapse-eligible at all -- the elision test is a function of the
/// coefficient words the `DESIGNED` term compares plus a state that starts `+0.0` in both -- and
/// [`plan_is_channel_symmetric`] is the bit a caller gates the collapse on so that the one case
/// where they could disagree declines instead of guessing.
#[inline(always)]
pub fn input_chain_block_mono_elided<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &InputChainCoef<L>,
    s: &mut InputChainState<L>,
    plan: &InputChainPlan,
) -> InputChainReport<L> {
    match plan.elided[0] {
        [false, false] => input_chain_block_mono(io, frames, c, s),
        [true, true] => {
            silence_skip_block(io, frames, &mut s.silence[0]);
            identity_chain_block_mono(io, frames, c)
        }
        _ => mixed_chain_block_mono(io, frames, c, s, plan),
    }
}

/// Whether the two channels of a chain elide the same sections.
///
/// The collapse's Job-1 interaction, and the reason it is a query rather than an assertion: an
/// elided identity section is `v |-> v + 0.0` and an *unelided* one with identity coefficients is
/// `v |-> fma(0, 0, fma(0, 0, 1.0 * v))`, which is `v`. The two agree everywhere except at `-0.0`.
/// So a chain whose channels disagree about elision is one whose dual run can produce `-0.0` on
/// one plane and `+0.0` on the other, and a collapse would claim they agree. It declines instead.
#[must_use]
pub const fn plan_is_channel_symmetric(plan: &InputChainPlan) -> bool {
    plan.elided[0][0] == plan.elided[1][0] && plan.elided[0][1] == plan.elided[1][1]
}

/// The sample-peak meter's partial over one resident AoSoA block (issue #943).
///
/// Returns, per lane, the maximum of `peak` and every **sanitized magnitude** in the block's
/// `frames` frames. A meter that seeds `peak` with `L::zero()` gets the block's partial peak and
/// merges it into its window with the same D8 select form; that merge is an exact reassociation of
/// the scalar meter's sample-serial `if a > p { a } else { p }` (`builtins::MeterAccumulator`),
/// because every operand lies in `{+0.0} ∪ [f32::MIN_POSITIVE, f32::MAX]`, where the select form
/// is commutative and associative, has one bit pattern per value and has `+0.0` as its identity.
///
/// The validity test is the meter's own `normal_or_zero` -- a finite, non-subnormal sample is
/// kept and anything else becomes `+0.0` -- and deliberately **not** [`NONFINITE_LIMIT`], the D7
/// input rule, which admits subnormals and rejects `[1e30, inf)`. Tested on the magnitude, it is
/// the two ordered compares below: NaN fails both, so it becomes `+0.0` before the `max` and D8's
/// asymmetric NaN rule is never exercised.
///
/// Frozen operation order, per frame:
/// 1. `a = |load(frame)|`
/// 2. `c = select(a >= MIN_POSITIVE & a < INFINITY, a, +0.0)`
/// 3. `peak = max(c, peak)`, the D8 `select(c > peak, c, peak)`
///
/// Branch free per frame: one load, two compares, one mask `and`, one select and one `max`.
#[inline(always)]
pub fn meter_sample_peak_block<L: Lane>(words: &[f32], frames: usize, peak: L) -> L {
    debug_assert!(words.len() >= frames * L::WIDTH);
    let low = L::splat(f32::MIN_POSITIVE);
    let high = L::splat(f32::INFINITY);
    let mut peak = peak;
    for frame in words[..frames * L::WIDTH].chunks_exact(L::WIDTH) {
        let a = L::load(frame).abs();
        let c = L::select(L::mask_and(a.ge(low), a.lt(high)), a, L::zero());
        peak = L::max(c, peak);
    }
    peak
}

/// The full meter's partials over one resident AoSoA block, per lane (issue #950): what
/// [`meter_block`] returns.
///
/// `peak`, `clipped` and `sanitized` are order-free block partials that the meter merges into its
/// window. `energy` is not a partial: it is the meter's own running sum carried through this block
/// sample by sample, from the seed the caller passed in.
#[derive(Clone, Copy, Debug)]
pub struct MeterBlock<L: crate::Widen> {
    /// Per lane, the maximum of `+0.0` and every sanitized magnitude of the block: bit for bit
    /// [`meter_sample_peak_block`] seeded with `L::zero()`.
    pub peak: L,
    /// Per lane, how many sanitized magnitudes are `>= 1.0`, as an exact `f32` integer.
    pub clipped: L,
    /// Per lane, how many words the sanitization replaced (NaN, `±inf` and nonzero subnormals), as
    /// an exact `f32` integer.
    pub sanitized: L,
    /// Per lane, the seed plus the square of every sanitized sample, added one sample at a time in
    /// frame order.
    pub energy: L::F64,
}

/// The full meter's block pass over one resident AoSoA block (issue #950): the sample peak, the
/// clipped and sanitized counts, and the energy sum, for every lane at once.
///
/// This is the builtin meter's scalar per-sample loop (`builtins::MeterAccumulator`, the `ALL`
/// selection) run across lanes: each lane reads its own words, in frame order, and nothing moves
/// between lanes. Four facts make every result bit-identical to that loop:
///
/// * `peak` equals [`meter_sample_peak_block`]`(words, frames, L::zero())` bit for bit: steps 1 to
///   3 below are that kernel's, in its order.
/// * The counts are exact integers in `f32`: each adds `+0.0` or `1.0` per frame, and a block never
///   has more than `2^24` frames (as [`sanitize_gain_block`] states), so no sum rounds.
/// * `w * w` is exact in binary64 for every widened `f32` `w`: its significand has at most 24 bits,
///   so the square has at most 48, and its exponent stays inside binary64's normal range. The
///   scalar loop's `f64::from(s) * f64::from(s)` is the same exact value, because the sanitized
///   sample `s` and its magnitude `c` have the same square.
/// * The energy's only rounding is therefore its add, and each lane performs the scalar loop's adds
///   in the scalar loop's order from the scalar loop's seed. No partial sum, pairwise sum or
///   reassociation exists here: that form moves the published `energy`/`rms` bits (issue #950,
///   R3), and is refused.
///
/// The validity test is the meter's `normal_or_zero` on the magnitude, as in
/// [`meter_sample_peak_block`]. `sanitized` counts a word exactly when its magnitude was replaced:
/// `a != c` holds for NaN, `±inf` and a nonzero subnormal, and fails for `±0.0` and every normal.
///
/// Frozen operation order, per frame:
/// 1. `a = |load(frame)|`
/// 2. `c = select(a >= MIN_POSITIVE & a < INFINITY, a, +0.0)`
/// 3. `peak = max(c, peak)`, the D8 `select(c > peak, c, peak)`
/// 4. `sanitized = sanitized + (1.0 & !(a == c))`
/// 5. `clipped = clipped + (1.0 & (c >= 1.0))`
/// 6. `w = widen(c)`, then `energy = energy + w * w`: two roundings' worth of operations, never
///    fused, of which only the add rounds
///
/// Branch free per frame. The caller must keep `frames <= 2^24`.
#[inline(always)]
pub fn meter_block<L: crate::Widen>(words: &[f32], frames: usize, energy: L::F64) -> MeterBlock<L> {
    use crate::LaneF64;
    debug_assert!(words.len() >= frames * L::WIDTH);
    let low = L::splat(f32::MIN_POSITIVE);
    let high = L::splat(f32::INFINITY);
    let one = L::splat(1.0);
    let mut peak = L::zero();
    let mut clipped = L::zero();
    let mut sanitized = L::zero();
    let mut energy = energy;
    for frame in words[..frames * L::WIDTH].chunks_exact(L::WIDTH) {
        let a = L::load(frame).abs();
        let c = L::select(L::mask_and(a.ge(low), a.lt(high)), a, L::zero());
        peak = L::max(c, peak);
        sanitized = sanitized.add(one.andnot(a.eq(c)));
        clipped = clipped.add(one.andnot(L::mask_not(c.ge(one))));
        let w = c.widen();
        energy = energy.add(w.mul(w));
    }
    MeterBlock {
        peak,
        clipped,
        sanitized,
        energy,
    }
}

#[cfg(test)]
std::thread_local! {
    static MIXED_PLAN_SELECTIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static IDENTITY_RAMP_DISPATCHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
mod mixed_elision_tests {
    use super::*;

    fn check<L: Lane>() {
        let zero = L::zero();
        let coefficient = SvfCoef {
            c1: zero,
            a2: zero,
            a3: zero,
            m0: L::splat(1.0),
            m1: zero,
            m2: zero,
        };
        let c = InputChainCoef {
            trim: [L::splat(1.0); 2],
            section: [[coefficient; 2]; 2],
            silence: L::splat(crate::silence_frames(48_000) as f32),
            constants: InputChainConstants::new(),
        };
        for frames in [0, 1, 17] {
            for pairing in 0..16 {
                let plan = InputChainPlan {
                    elided: [
                        [pairing & 1 != 0, pairing & 2 != 0],
                        [pairing & 4 != 0, pairing & 8 != 0],
                    ],
                };
                let mut left = std::vec![-0.0; frames * L::WIDTH];
                let mut right = std::vec![1.0; frames * L::WIDTH];
                let mut state = InputChainState::default();
                MIXED_PLAN_SELECTIONS.with(|count| count.set(0));
                let _ = mixed_chain_block(&mut left, &mut right, frames, &c, &mut state, &plan);
                assert_eq!(MIXED_PLAN_SELECTIONS.with(std::cell::Cell::get), 2);
                MIXED_PLAN_SELECTIONS.with(|count| count.set(0));
                let _ = mixed_chain_block_mono(&mut left, frames, &c, &mut state, &plan);
                assert_eq!(MIXED_PLAN_SELECTIONS.with(std::cell::Cell::get), 1);
            }
        }
    }

    #[test]
    fn mixed_elision_selects_once_per_channel_independent_of_frames() {
        crate::each_lane!(|L| check::<L>());
    }

    #[test]
    fn identity_trim_ramp_wrappers_enter_the_identity_body() {
        let zero = f32::zero();
        let identity = SvfCoef {
            c1: zero,
            a2: zero,
            a3: zero,
            m0: f32::splat(1.0),
            m1: zero,
            m2: zero,
        };
        let c = InputChainCoef {
            trim: [f32::splat(1.0); 2],
            section: [[identity; 2]; 2],
            silence: crate::silence_frames(48_000) as f32,
            constants: InputChainConstants::new(),
        };
        let plan = InputChainPlan {
            elided: [[true, true], [true, true]],
        };
        let mut ramp = InputTrimRamp {
            current: [f32::splat(1.0); 2],
            target: [f32::splat(2.0); 2],
            step: [f32::splat(0.25); 2],
            remaining: [f32::splat(4.0); 2],
        };
        let mut state = InputChainState::default();
        let mut left = std::vec![1.0; 4];
        let mut right = left.clone();
        IDENTITY_RAMP_DISPATCHES.with(|count| count.set(0));
        let _ = input_chain_ramp_block_elided(
            &mut left, &mut right, 4, &c, &mut state, &mut ramp, &plan,
        );
        assert_eq!(IDENTITY_RAMP_DISPATCHES.with(std::cell::Cell::get), 1);
        IDENTITY_RAMP_DISPATCHES.with(|count| count.set(0));
        let _ = input_chain_ramp_block_mono_elided(&mut left, 4, &c, &mut state, &mut ramp, &plan);
        assert_eq!(IDENTITY_RAMP_DISPATCHES.with(std::cell::Cell::get), 1);
    }
}
