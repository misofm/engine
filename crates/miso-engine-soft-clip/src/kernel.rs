//! The soft-clip block kernel: one generic body, instantiated at every width.
//!
//! This is the whole of the frozen graph of `.github/ISSUE_SPECS/BRIEFS/019` — two-times
//! oversampling through a 63-tap Blackman half-band, a cubic shaper clamped to `±2/3`, a dry path
//! delayed 31 samples and a dry/wet mix — written once, generic over
//! [`Lane`](miso_engine_lane::Lane) (D10). The scalar tail is the same function at `L = f32`,
//! because a planar slice is already a `WIDTH = 1` AoSoA block.
//!
//! # What changed against the five hand-written copies, and what did not
//!
//! The audit of issue #91 found this graph implemented five times (an effect-crate scalar lane and
//! four `core/arch` kernels) and running at 296 ns per track-channel-sample: 258 per-operation
//! store/`is_finite`/reload wrappers, 992 scalar gathers with a `% 63`, two 63-word coefficient
//! scans per frame, and four full 31-tap convolutions where the half-band structure needs the work
//! of two. All four are gone. **The arithmetic is not changed**: the surviving products are the
//! same products in the same ascending tap order, so the output is bit-identical to the 63-tap
//! form (`tests/polyphase_identity.rs`, one million samples, zero mismatches).
//!
//! # Frozen operation order, per frame
//!
//! 1. `drive += drive_step`, `output += output_step`, `mix += mix_step` — D11: the increment was
//!    computed once at event time, and the ramp advances *before* the sample uses it.
//! 2. `xin = load(frame)`; push `xin` into the dry history (unflushed — see below).
//! 3. `X = flush((drive + drive) * xin)` — `drive + drive` is exact, and is the `2 * drive` of the
//!    brief; push `X` into the interpolation history.
//! 4. `u = halfband2x_interp_even(X history)` — the even-phase high-rate sample.
//! 5. `e = flush(cubic(u))`; push `e` into the decimation history.
//! 6. `odd = cubic(0.5 * X[n-31])` — the odd-phase high-rate sample, recomputed from the input
//!    history rather than stored, because it is the only one the kept decimation phase reads.
//! 7. `wet = halfband2x_decim_even(e history, odd)`.
//! 8. `d = dry[n-31]`; `a = 1 - mix`; `b = a * d`; `c = mix * wet`; `s = b + c`; `y = output * s`.
//! 9. `store(frame, select(bypass or (mix == 0 and output == 1), d, y))`.
//!
//! There is no `fma` anywhere: the frozen graph has none, and adding one would change every pinned
//! bit (D3 permits fusion only where `Lane::fma` is written).
//!
//! # Denormals and finiteness (D7)
//!
//! `flush` is applied to the two values that enter a history and are then multiplied by a filter
//! tap — `X` and `e`. The dry history is deliberately **not** flushed: it is selected or multiplied
//! once and never accumulated, and the identity path has to reproduce a `-0.0` input exactly.
//! Flushing `X` cannot change a rendered bit even when it fires on `-0.0`, because every product a
//! `±0.0` makes is added to an accumulator that is never `-0.0`, and `cubic(±0.0)` is `+0.0` for
//! either sign. Output finiteness is checked once per block by
//! `miso_engine_effect_runtime::bank::finish_block`; there is no per-value check anywhere.

use miso_engine_lane::kernels::halfband::{
    HALFBAND63_BASE, HALFBAND63_ROWS, halfband2x_decim_even, halfband2x_interp_even,
    history_advance, history_push, history_row,
};
use miso_engine_lane::{Lane, flush};

/// Samples of dry delay, which is also the effect's latency and the deepest history age read.
pub const DRY_DELAY: usize = 31;

/// Per-block, per-lane coefficients: the D11 ramp increments and the bypass mask.
///
/// Every field is block-constant. A ramp that ends inside a block is handled by splitting the
/// block, not by branching per sample (see `Segments` in `lib.rs`), so a step never changes while
/// the kernel is running.
#[derive(Clone, Copy)]
pub struct SoftClipCoef<L: Lane> {
    /// Per-sample increment of the linear drive gain, `+0.0` for a lane that is not ramping.
    pub drive_step: L,
    /// Per-sample increment of the linear output gain.
    pub output_step: L,
    /// Per-sample increment of the dry/wet mix.
    pub mix_step: L,
    /// All-ones on lanes whose effect instance is bypassed. Block-uniform: bypass is part of the
    /// program key, so a cohort is either bypassed or not for its whole life.
    pub bypass: L::Mask,
}

/// The ramp values the kernel carries across a block.
///
/// These are the parameter values of record: `LinearRamp` keeps the target, the step and the
/// countdown, but the *current* value is whatever the kernel's iterated additions produced, and a
/// snapshot reads it from here. Deriving it as `start + n * step` instead would break partition
/// invariance.
#[derive(Clone, Copy)]
pub struct SoftClipState<L: Lane> {
    /// Linear drive gain `g`; the kernel applies `2 * g`.
    pub drive: L,
    /// Linear output gain.
    pub output: L,
    /// Dry/wet mix in `[0, 1]`.
    pub mix: L,
}

impl<L: Lane> SoftClipState<L> {
    /// A state with every ramp resting at the given per-lane values.
    #[must_use]
    pub fn from_lanes(drive: L, output: L, mix: L) -> Self {
        Self { drive, output, mix }
    }
}

/// The three AoSoA histories of one channel, and the one cursor all lanes share.
///
/// Each history is [`HALFBAND63_ROWS`] rows of `width` floats, double-written so the 31-row window
/// the taps read is always contiguous. There is **one** `pos` for the whole bank: the per-lane
/// cursors the audit found (F3) existed only because a faulted lane skipped a sample, and under D7
/// no per-sample fault path exists. Every lane-local operation — a track reset, a track restore —
/// is expressed relative to the shared `pos`, so no lane can ever drift.
pub struct SoftClipHistory {
    /// Write position in `0..32`.
    pub pos: u32,
    /// `X[n] = 2 * g * x[n]`, the high-rate interpolator input.
    pub x: Box<[f32]>,
    /// `e[n] = cubic(u[2n])`, the shaped even-phase high-rate sample.
    pub e: Box<[f32]>,
    /// The raw input, for the delayed dry path and the identity select.
    pub dry: Box<[f32]>,
}

impl SoftClipHistory {
    /// Allocates zeroed histories for `width` lanes. Control plane; the only allocation here.
    #[must_use]
    pub fn new(width: usize) -> Self {
        let rows = HALFBAND63_ROWS * width;
        Self {
            pos: 0,
            x: vec![0.0; rows].into_boxed_slice(),
            e: vec![0.0; rows].into_boxed_slice(),
            dry: vec![0.0; rows].into_boxed_slice(),
        }
    }

    /// Zeroes every history and returns the cursor to zero.
    pub fn clear(&mut self) {
        self.pos = 0;
        self.x.fill(0.0);
        self.e.fill(0.0);
        self.dry.fill(0.0);
    }
}

/// The cubic shaper `c(u) = u - u^3 / 3`, clamped to `±2/3`, branch-free.
///
/// Frozen operation order: `p0 = u * u`, `p1 = p0 * u`, `p2 = p1 / 3`, `poly = u - p2`, then
/// `select(u >= 1, +2/3, select(u <= -1, -2/3, poly))`. The division is an IEEE division on every
/// target, so it is deterministic; replacing it with a reciprocal multiply is a class-B change the
/// owner has not taken (issue #91 F8, question 1).
///
/// NaN takes the polynomial arm, because both comparisons are ordered and therefore false — which
/// is what the branching form did too, so NaN still propagates to the block boundary check.
#[inline(always)]
#[must_use]
pub fn cubic<L: Lane>(u: L) -> L {
    let below = u.le(L::splat(-1.0));
    let above = u.ge(L::splat(1.0));
    let p0 = u.mul(u);
    let p1 = p0.mul(u);
    let p2 = p1.div(L::splat(3.0));
    let poly = u.sub(p2);
    let clamped = L::select(below, L::splat(-2.0 / 3.0), poly);
    L::select(above, L::splat(2.0 / 3.0), clamped)
}

/// Renders one AoSoA block in place.
///
/// `io` is `frames * L::WIDTH` floats, frame-major. Shapes are validated once at plan preparation;
/// this is a render-path call with `debug_assert!` only, no `Result` and no allocation.
///
/// # Panics
///
/// Panics in debug builds if `io` or any history has the wrong length.
#[inline(always)]
pub fn soft_clip_block<L: Lane>(
    io: &mut [f32],
    frames: usize,
    c: &SoftClipCoef<L>,
    s: &mut SoftClipState<L>,
    h: &mut SoftClipHistory,
) {
    let width = L::WIDTH;
    debug_assert_eq!(io.len(), frames * width);
    debug_assert_eq!(h.x.len(), HALFBAND63_ROWS * width);
    debug_assert_eq!(h.e.len(), HALFBAND63_ROWS * width);
    debug_assert_eq!(h.dry.len(), HALFBAND63_ROWS * width);

    let one = L::splat(1.0);
    let half = L::splat(0.5);
    let zero = L::zero();
    let (mut drive, mut output, mut mix) = (s.drive, s.output, s.mix);
    let mut pos = h.pos as usize;

    for frame in io.chunks_exact_mut(width) {
        drive = drive.add(c.drive_step);
        output = output.add(c.output_step);
        mix = mix.add(c.mix_step);

        let xin = L::load(frame);
        history_push::<L>(&mut h.dry, pos, xin);
        history_push::<L>(&mut h.x, pos, flush(drive.add(drive).mul(xin)));

        let base = pos + HALFBAND63_BASE;
        let u = halfband2x_interp_even::<L>(&h.x, base);
        history_push::<L>(&mut h.e, pos, flush(cubic(u)));

        let odd = cubic(half.mul(L::load(history_row::<L>(&h.x, base - DRY_DELAY))));
        let wet = halfband2x_decim_even::<L>(&h.e, base, odd);

        let dry = L::load(history_row::<L>(&h.dry, base - DRY_DELAY));
        let a = one.sub(mix);
        let b = a.mul(dry);
        let w = mix.mul(wet);
        let summed = b.add(w);
        let y = output.mul(summed);

        let identity = L::mask_or(c.bypass, L::mask_and(mix.eq(zero), output.eq(one)));
        L::select(identity, dry, y).store(frame);

        pos = history_advance(pos);
    }

    s.drive = drive;
    s.output = output;
    s.mix = mix;
    h.pos = pos as u32;
}
