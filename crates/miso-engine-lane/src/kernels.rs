//! Generic block kernels: one body per kernel, instantiated at every width.
//!
//! Every kernel takes a whole block, keeps its state in registers across the block, loads its
//! coefficients once, and is `#[inline(always)]` and generic over [`Lane`] (D10). There are no
//! per-sample entry points and no scalar copies: a scalar tail is the same body at `L = f32`,
//! because a planar slice is already a `WIDTH = 1` AoSoA block (master plan §4.1).
//!
//! # Layout
//!
//! An AoSoA block is a `&mut [f32]` of length `frames * L::WIDTH`, frame-major: sample `f` of lane
//! `l` lives at `f * WIDTH + l`. Left and right are separate blocks with separate state.
//!
//! # Validation
//!
//! Block length, width and coefficient shapes are fixed by the prepared plan and validated once at
//! prepare time. The kernels carry `debug_assert!` only: no `Result`, no branch, no panic path on
//! the render thread (master plan §4.3).
//!
//! # Operation order is frozen
//!
//! Each kernel's doc comment lists its operations line by line. That order *is* the numeric
//! contract: reassociating one of them changes the rendered bits, so a change here is a change to
//! every pinned fixture in the workspace and needs the fixture re-pin procedure of master plan §8.

pub mod builtins;
pub mod halfband;

use crate::{Lane, flush};

/// Coefficients of one TPT state-variable filter, one set per lane.
///
/// Amendment A1 (measured in #87): the stored damping coefficient is `c1 = t / (1 + t)` with
/// `t = g * (g + k)`, **not** `a1 = 1 / (1 + t)`. At 10 Hz, Q = 18, 88.2 kHz, `t` is about
/// 4.7e-6, so `a1` rounded to `f32` carries about 0.6 % relative error in the pole damping (127
/// grid failures, worst 0.0466 dB, 0.604 dB on an impulse) while `c1` carries about 6e-8 (no
/// failures, worst 6.8e-4 dB).
///
/// The control plane designs these in `f64` and rounds once: `g = tan(pi * f0 / fs)`, `k = 1 / Q`,
/// `t = g * (g + k)`, `c1 = t / (1 + t)`, `a1 = 1 - c1`, `a2 = g * a1`, `a3 = g * a2`; the output
/// mix `(m0, m1, m2)` selects the response (low `(0, 0, 1)`, high `(1, -k, -1)`, band `(0, 1, 0)`,
/// notch `(1, -k, 0)`, and Simper's published bell and shelf mappings).
#[derive(Clone, Copy)]
pub struct SvfCoef<L: Lane> {
    /// `t / (1 + t)`, the damping coefficient (A1).
    pub c1: L,
    /// `g * (1 - c1)`.
    pub a2: L,
    /// `g * a2`.
    pub a3: L,
    /// Direct output mix.
    pub m0: L,
    /// Band output mix.
    pub m1: L,
    /// Low output mix.
    pub m2: L,
}

/// The two integrator state words of a TPT state-variable filter, one set per lane.
#[derive(Clone, Copy)]
pub struct SvfState<L: Lane> {
    /// First integrator.
    pub ic1: L,
    /// Second integrator.
    pub ic2: L,
}

impl<L: Lane> Default for SvfState<L> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            ic1: L::zero(),
            ic2: L::zero(),
        }
    }
}

/// Per-sample coefficient increments for [`svf_block_ramped`], one set per lane.
///
/// All-zero increments with `ramp_frames = 0` make [`svf_block_ramped`] bit-identical to
/// [`svf_block`], which is a gate (G2), not a claim.
#[derive(Clone, Copy)]
pub struct SvfCoefStep<L: Lane> {
    /// Increment of [`SvfCoef::c1`].
    pub c1: L,
    /// Increment of [`SvfCoef::a2`].
    pub a2: L,
    /// Increment of [`SvfCoef::a3`].
    pub a3: L,
    /// Increment of [`SvfCoef::m0`].
    pub m0: L,
    /// Increment of [`SvfCoef::m1`].
    pub m1: L,
    /// Increment of [`SvfCoef::m2`].
    pub m2: L,
}

impl<L: Lane> Default for SvfCoefStep<L> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            c1: L::zero(),
            a2: L::zero(),
            a3: L::zero(),
            m0: L::zero(),
            m1: L::zero(),
            m2: L::zero(),
        }
    }
}

/// Topology-preserving-transform state-variable filter over one block (D2, master plan §4.2).
///
/// Algebraically this is Simper's `v1 = a1 * ic1 + a2 * v3`, `ic1' = 2 * v1 - ic1`; numerically the
/// `c1` / `ic + 2 * d` form below is the one that passes the frozen gates (#87 plan §3).
///
/// Frozen operation order, per frame:
/// 1. `v0 = load(frame)`
/// 2. `v3 = v0 - ic2`
/// 3. `d1 = fma(-c1, ic1, a2 * v3)` — one multiply, then one fused multiply-add
/// 4. `v1 = ic1 + d1`
/// 5. `d2 = fma(a3, v3, a2 * ic1)` — `ic1` is still the old value here
/// 6. `v2 = ic2 + d2`
/// 7. `ic1 = flush(ic1 + (d1 + d1))` — `d1 + d1` is exact
/// 8. `ic2 = flush(ic2 + (d2 + d2))`
/// 9. `y = fma(m2, v2, fma(m1, v1, m0 * v0))`
/// 10. `store(frame, y)`
///
/// `-c1` is computed once per block as a sign-bit flip, which is exact. Steps 2 to 9 are
/// [`svf_step`], which is the only copy of them: this kernel, [`svf_block_ramped`] and the fused
/// chain kernels of [`builtins`] all call it, so the numeric contract has one home.
#[inline(always)]
pub fn svf_block<L: Lane>(io: &mut [f32], frames: usize, c: &SvfCoef<L>, s: &mut SvfState<L>) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let mut state = *s;
    let nc1 = c.c1.neg();
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let v0 = L::load(frame);
        let (v1, v2) = svf_step(v0, nc1, c.a2, c.a3, &mut state);
        let y = c.m2.fma(v2, c.m1.fma(v1, c.m0.mul(v0)));
        y.store(frame);
    }
    *s = state;
}

/// One frame of [`svf_block`]'s recurrence, returning the band-pass and low-pass taps.
///
/// **It is a per-frame step helper for a caller that owns its own frame loop, and it duplicates
/// nothing.** [`svf_block_ramped`] is the *other* thing: a whole-block kernel whose coefficients
/// move per sample. The two are orthogonal — this one takes constant coefficients and hands back
/// both taps, that one takes moving coefficients and hands back one mixed output — and both are
/// the same recurrence, because `svf_block`, `svf_block_ramped` and every caller of `svf_step` run
/// the body written once below. A crossover embedded in a segment driver, where the filter output
/// feeds a ring in the same frame, cannot call either block kernel; it calls this.
///
/// Steps 2 to 8 of [`svf_block`]'s frozen order, in that order — everything except the load, the
/// output mix and the store. `nc1` is `-c1`, hoisted out of the caller's frame loop because a
/// sign-bit flip is exact and a filter whose coefficients do not move should not recompute it per
/// sample; [`svf_block_ramped`] recomputes it per frame because its coefficients do move.
///
/// This is **not** a per-sample entry point in the sense D10 forbids: it is `#[inline(always)]`,
/// generic, takes no slices, validates nothing and returns no `Result`, and it is the body
/// [`svf_block`] itself runs. What D10 deletes is the opposite thing — a validated, dynamically
/// dispatched, `#[inline(never)]` one-sample call across a crate boundary. Nor is it
/// [`svf_block_ramped`]'s job: that kernel exists for per-sample *coefficient* ramps, and a
/// crossover's coefficients never move.
///
/// This exists because a filter whose two taps are *both* wanted cannot go through
/// [`svf_block`]'s single output mix. The Linkwitz-Riley crossover of the multiband compressor is
/// the case that motivates it (audit #94 F4): one stage yields the low-pass tap that feeds the
/// second stage *and* the band-pass tap that forms the all-pass `x - 2k*v1`, from which the high
/// band is a subtraction. Writing a second SVF body for that would be exactly the duplication the
/// #83 audit is about, so [`svf_block`] is expressed through this function and there is one
/// recurrence in the workspace.
///
/// Expressing that with [`svf_block`] would need two passes with two mixes over two *separate*
/// state sets — which is the four-section crossover the audit proved redundant — plus a scratch
/// buffer per band, and it cannot express a frame-serial graph at all: the multiband's crossover
/// output feeds a ring, the ring feeds a per-track detector tap, and the detector's gain
/// multiplies the *delayed* band, all inside one frame.
///
/// The caller owns the frame loop, so `s` must be a local copy of the state for the duration of a
/// block — passing the caller's stored state straight in would reload it from memory every frame.
///
/// Frozen operation order, matching [`svf_block`] step for step:
/// 1. `v3 = v0 - ic2`
/// 2. `d1 = fma(nc1, ic1, a2 * v3)`
/// 3. `v1 = ic1 + d1`
/// 4. `d2 = fma(a3, v3, a2 * ic1)` — `ic1` is still the old value here
/// 5. `v2 = ic2 + d2`
/// 6. `ic1 = flush(ic1 + (d1 + d1))` — `d1 + d1` is exact
/// 7. `ic2 = flush(ic2 + (d2 + d2))`
#[inline(always)]
pub fn svf_step<L: Lane>(v0: L, nc1: L, a2: L, a3: L, s: &mut SvfState<L>) -> (L, L) {
    let v3 = v0.sub(s.ic2);
    let d1 = nc1.fma(s.ic1, a2.mul(v3));
    let v1 = s.ic1.add(d1);
    let d2 = a3.fma(v3, a2.mul(s.ic1));
    let v2 = s.ic2.add(d2);
    s.ic1 = flush(s.ic1.add(d1.add(d1)));
    s.ic2 = flush(s.ic2.add(d2.add(d2)));
    (v1, v2)
}

/// [`svf_block`] with per-lane, per-sample coefficient ramps (amendment A2).
///
/// This is the only legal home for a per-sample coefficient update under D10. The frame body is
/// [`svf_block`]'s, unchanged and in the same order; after each of the first `ramp_frames` frames
/// every coefficient advances by its increment (`c += step`, D11: no per-sample division). The
/// window is the smoothing window of master plan D11 and is at most 64 frames in production.
///
/// `c` is advanced in place, so the caller's coefficient set carries the ramp across block
/// boundaries; with `ramp_frames = 0` no addition happens at all and the result is bit-identical to
/// [`svf_block`].
#[inline(always)]
pub fn svf_block_ramped<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    ramp_frames: usize,
    s: &mut SvfState<L>,
) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let mut state = *s;
    for (index, frame) in io.chunks_exact_mut(L::WIDTH).enumerate() {
        let nc1 = c.c1.neg();
        let v0 = L::load(frame);
        let (v1, v2) = svf_step(v0, nc1, c.a2, c.a3, &mut state);
        let y = c.m2.fma(v2, c.m1.fma(v1, c.m0.mul(v0)));
        y.store(frame);
        if index < ramp_frames {
            c.c1 = c.c1.add(step.c1);
            c.a2 = c.a2.add(step.a2);
            c.a3 = c.a3.add(step.a3);
            c.m0 = c.m0.add(step.m0);
            c.m1 = c.m1.add(step.m1);
            c.m2 = c.m2.add(step.m2);
        }
    }
    *s = state;
}

/// Coefficient of a one-pole TPT smoother or envelope follower, one per lane.
#[derive(Clone, Copy)]
pub struct OnePoleCoef<L: Lane> {
    /// Per-sample coefficient, `1 - exp(-1 / (tau * fs))` for a time constant `tau`.
    pub c: L,
}

/// The single state word of a one-pole smoother, one per lane.
#[derive(Clone, Copy)]
pub struct OnePoleState<L: Lane> {
    /// Last output.
    pub y: L,
}

impl<L: Lane> Default for OnePoleState<L> {
    #[inline(always)]
    fn default() -> Self {
        Self { y: L::zero() }
    }
}

/// One-pole smoother over one block: `y += c * (x - y)`, one rounding.
///
/// Frozen operation order, per frame:
/// 1. `x = load(frame)`
/// 2. `d = x - y`
/// 3. `y = fma(c, d, y)`
/// 4. `y = flush(y)` — `y` is a recurrence, so D7 applies to it
/// 5. `store(frame, y)`
#[inline(always)]
pub fn one_pole_block<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &OnePoleCoef<L>,
    s: &mut OnePoleState<L>,
) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let mut y = s.y;
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        let d = x.sub(y);
        y = c.c.fma(d, y);
        y = flush(y);
        y.store(frame);
    }
    s.y = y;
}

/// Constant gain over one block: `y = x * g`.
///
/// Frozen operation order, per frame: `x = load(frame)`, `y = x * g`, `store(frame, y)`.
#[inline(always)]
pub fn gain_block<L: Lane>(io: &mut [f32], frames: usize, g: L) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    for frame in io.chunks_exact_mut(L::WIDTH) {
        let x = L::load(frame);
        x.mul(g).store(frame);
    }
}

/// Dry/wet mix of a gain over one block: `y = dry + mix * (wet - dry)` with `wet = x * g`.
///
/// Frozen operation order, per frame:
/// 1. `x = load(frame)`
/// 2. `w = x * g`
/// 3. `d = w - x`
/// 4. `y = fma(mix, d, x)`
/// 5. `store(frame, y)`
///
/// `mix = 0` returns `x` bit-for-bit (`fma(0, d, x) = x` for finite `d`), which is what makes a
/// bypassed slot an identity kernel rather than a near-identity one.
#[inline(always)]
pub fn gain_mix_block<L: Lane>(io: &mut [f32], frames: usize, g: L, mix: L) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    for frame in io.chunks_exact_mut(L::WIDTH) {
        gain_mix_step(L::load(frame), g, mix).store(frame);
    }
}

/// One frame of [`gain_mix_block`]: `y = dry + mix * (wet - dry)` with `wet = x * g`.
///
/// The body of [`gain_mix_block`], factored out so that an effect whose gain is recomputed per
/// frame — a dynamics processor, where `g` comes out of a detector rather than out of a prepared
/// coefficient — composes the *same* law rather than writing a second copy of it. The block kernel
/// is a loop over this function, so the two are bit-identical by construction.
///
/// Frozen operation order:
/// 1. `w = x * g`
/// 2. `d = w - x`
/// 3. `y = fma(mix, d, x)` — one rounding
///
/// `mix = 0` returns `x` bit-for-bit for a finite `d` (`fma(0, d, x) = x`). It does **not**
/// preserve the sign of a zero `x` when `d` is non-zero, which is why an effect with a signed-zero
/// identity contract selects the dry value with a mask instead of relying on `mix`.
#[inline(always)]
#[must_use]
pub fn gain_mix_step<L: Lane>(x: L, g: L, mix: L) -> L {
    let w = x.mul(g);
    let d = w.sub(x);
    mix.fma(d, x)
}

/// A linear gain ramp over one block (D11), one segment per lane.
#[derive(Clone, Copy)]
pub struct RampSegment<L: Lane> {
    /// Gain applied to the first frame of the block.
    pub start: L,
    /// Per-sample increment, precomputed once at event time — never a per-sample division.
    pub step: L,
    /// Gain applied from frame `ramp_frames` onward, exactly (the ramp snaps to it).
    pub target: L,
    /// Number of ramping frames in this block.
    pub ramp_frames: usize,
}

/// Applies a linear gain ramp over one block and returns the running gain after it.
///
/// Frozen operation order, per frame:
/// 1. `gain = index < ramp_frames ? g : target` — the snap of D11, exact
/// 2. `x = load(frame)`
/// 3. `y = x * gain`
/// 4. `store(frame, y)`
/// 5. `g = g + step`
///
/// The returned gain is the `g` a following block must start from, which is what makes the kernel
/// partition-invariant: `start + step` iterated is not `start + n * step` in `f32` (gate P1).
#[inline(always)]
pub fn ramp_block<L: Lane>(io: &mut [f32], frames: usize, seg: &RampSegment<L>) -> L {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    let mut g = seg.start;
    for (index, frame) in io.chunks_exact_mut(L::WIDTH).enumerate() {
        let gain = if index < seg.ramp_frames {
            g
        } else {
            seg.target
        };
        let x = L::load(frame);
        x.mul(gain).store(frame);
        g = g.add(seg.step);
    }
    g
}

/// Two-input sum: `out = a + b`, frame by frame (D9).
///
/// Frames are independent, so the sum is vectorised over frames and never across lanes: there is
/// no horizontal add anywhere in the engine. Whatever does not fill a whole vector is finished by
/// the same body at `L = f32`, so the result does not depend on the width.
#[inline(always)]
pub fn sum2_block<L: Lane>(out: &mut [f32], a: &[f32], b: &[f32]) {
    debug_assert_eq!(out.len(), a.len());
    debug_assert_eq!(out.len(), b.len());
    let count = out.len();
    let vectored = count - count % L::WIDTH;
    let mut index = 0;
    while index < vectored {
        let left = L::load(&a[index..]);
        let right = L::load(&b[index..]);
        left.add(right).store(&mut out[index..]);
        index += L::WIDTH;
    }
    while index < count {
        let left = <f32 as Lane>::load(&a[index..]);
        let right = <f32 as Lane>::load(&b[index..]);
        left.add(right).store(&mut out[index..]);
        index += 1;
    }
}

/// Accumulating sum: `acc += x`, frame by frame (D9).
///
/// Mixing is a left-to-right pairwise reduction in stable node-ID order; this is the step of that
/// reduction. The same order is used by the sequential and the parallel executor.
#[inline(always)]
pub fn sum_into_block<L: Lane>(acc: &mut [f32], x: &[f32]) {
    debug_assert_eq!(acc.len(), x.len());
    let count = acc.len();
    let vectored = count - count % L::WIDTH;
    let mut index = 0;
    while index < vectored {
        let sum = L::load(&acc[index..]).add(L::load(&x[index..]));
        sum.store(&mut acc[index..]);
        index += L::WIDTH;
    }
    while index < count {
        let sum = <f32 as Lane>::load(&acc[index..]).add(<f32 as Lane>::load(&x[index..]));
        sum.store(&mut acc[index..]);
        index += 1;
    }
}

/// Integer-sample plugin-delay compensation over one block: a two-segment slice exchange.
///
/// `ring` is a delay line of `ring.len()` sample words, `cursor` is its write position, and `io` is
/// exchanged with it in at most two contiguous segments. There is no per-sample work and no
/// floating-point operation at all, so this kernel is exact by construction and needs no width.
/// The delay is `ring.len()` words; for an AoSoA block of `WIDTH` lanes that is
/// `delay_frames * WIDTH`.
///
/// # Panics
///
/// Panics if `io` is longer than `ring`, or if `cursor` is not a position inside `ring`.
#[inline(always)]
pub fn pdc_delay_block(ring: &mut [f32], cursor: &mut usize, io: &mut [f32]) {
    debug_assert!(io.len() <= ring.len());
    debug_assert!(*cursor < ring.len());
    let length = ring.len();
    let count = io.len();
    let first = core::cmp::min(count, length - *cursor);
    ring[*cursor..*cursor + first].swap_with_slice(&mut io[..first]);
    let rest = count - first;
    if rest > 0 {
        ring[..rest].swap_with_slice(&mut io[first..count]);
    }
    let advanced = *cursor + count;
    *cursor = if advanced >= length {
        advanced - length
    } else {
        advanced
    };
}
