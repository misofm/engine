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
/// elision variants [`identity_chain_block`] and [`mixed_chain_block`], which duplicate the
/// sanitise prologue. A copy left on the select-form would still be correct — that is the point of
/// the equivalence — but the four are kept identical so the frozen order above has one reading.
///
/// The **floor accounting does not move**: this was one mask-and-value operation before and is one
/// now, so the sanitise inventory in `docs/rulings/effect-floor-accounting.md` stays at 7
/// lane-ops. The change is an instruction-selection one, gated by
/// `crates/miso-engine-lane/tests/sanitise_counter.rs`.
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
/// 3. `current = select(done, target, current + step)`
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
        current = L::select(done, r.target, current.add(r.step));
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
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
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
/// — never per call. A `true` entry can never go stale: the six coefficient words are
/// `PreparedOnly` in the builtin parameter ABI, and an elided section's integrators are never
/// written by the render path, so the only way out of the identity is an explicit state write,
/// which recomputes the plan.
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
        [[true, true], [true, true]] => identity_chain_block(left, right, frames, c),
        _ => mixed_chain_block(left, right, frames, c, s, plan),
    }
}

/// The chain of a bank whose four sections are all the identity: no recurrence, no state.
///
/// Frozen operation order, per frame and per channel `ch`: steps 1-3 of [`input_chain_block`],
/// then `v = v + 0.0` for the run of two identity sections, then its steps 5 and 6.
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
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = [zero; 2];
    let mut nonfinite = [no_lanes::<L>(); 2];
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
            count[channel] = count[channel].add(one.andnot(L::mask_not(bad)));
            let mut v = x.andnot(bad).mul(c.trim[channel]);
            let mut run = false;
            for section in 0..2 {
                if plan.elided[channel][section] {
                    run = true;
                    continue;
                }
                if run {
                    v = v.add(zero);
                    run = false;
                }
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
            if run {
                v = v.add(zero);
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

// ---------------------------------------------------------------------------------------------
// The mono-collapse one-plane variants.
//
// A collapsed track computes one channel and the strip duplicates it at the fader/matrix seam, so
// these are the same three shapes as above with the channel loop peeled to the one live channel.
// The rule that makes them class A is stated once here and holds for every one of them: **each is
// the dual body's channel-`0` arm, character for character, with the channel index frozen at `0`
// and the `1` arm deleted.** No operation is reassociated, no compare is hoisted, no accumulation
// changes order -- the two channels were already independent per-frame arithmetic in one loop, so
// deleting one of them cannot move the other's bits.
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
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut state = s.section[0];
    let mut nc1 = [zero; 2];
    for (section, coefficient) in c.section[0].iter().enumerate() {
        nc1[section] = coefficient.c1.neg();
    }

    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(c.trim[0]);
        for section in 0..2 {
            let coefficient = &c.section[0][section];
            let v0 = v;
            let (v1, v2) = svf_step(
                v0,
                nc1[section],
                coefficient.a2,
                coefficient.a3,
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
    let limit = L::splat(NONFINITE_LIMIT);
    let one = L::splat(1.0);
    let zero = L::zero();

    let mut count = zero;
    let mut nonfinite = no_lanes::<L>();
    let mut state = s.section[0];
    let mut nc1 = [zero; 2];
    for (section, coefficient) in c.section[0].iter().enumerate() {
        nc1[section] = coefficient.c1.neg();
    }

    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let bad = L::mask_not(x.abs().lt(limit));
        count = count.add(one.andnot(L::mask_not(bad)));
        let mut v = x.andnot(bad).mul(c.trim[0]);
        let mut run = false;
        for section in 0..2 {
            if plan.elided[0][section] {
                run = true;
                continue;
            }
            if run {
                v = v.add(zero);
                run = false;
            }
            let coefficient = &c.section[0][section];
            let v0 = v;
            let (v1, v2) = svf_step(
                v0,
                nc1[section],
                coefficient.a2,
                coefficient.a3,
                &mut state[section],
            );
            v = coefficient
                .m2
                .fma(v2, coefficient.m1.fma(v1, coefficient.m0.mul(v0)));
        }
        if run {
            v = v.add(zero);
        }
        nonfinite = L::mask_or(nonfinite, L::mask_not(v.abs().lt(limit)));
        v.store(frame);
    }

    s.section[0] = state;
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
        [true, true] => identity_chain_block_mono(io, frames, c),
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
