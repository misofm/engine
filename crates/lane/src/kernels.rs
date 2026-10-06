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

/// The one D11 ramp update every engine ramp uses (issues #1408, #1409); see
/// [`builtins::ramp_toward`]. Re-exported here so the effect ramps and the contract's smoother call
/// the same body at `L = f32` as the bank kernels do at every width.
pub use builtins::ramp_toward;

use crate::{Lane, flush, flush_pair, silence_step};

/// Where an SVF kernel reads its per-frame rest thresholds from (issue #1328): an effect channel's
/// **rest plane** (`&[f32]`, written by [`silence_block`]), or none ([`UnarmedRest`]).
///
/// With a plane a kernel runs [`svf_step`]: one threshold load per frame (per stream-frame in the
/// cascades) and [`crate::flush_pair`]'s joint term. Without one it runs [`svf_step_when`]`(false,
/// ..)`: the two per-word [`flush`]es, no load and no joint term. The two forms are the same bits
/// on a block whose thresholds are all `+0.0`, because `flush_pair(n1, n2, +0.0)` is
/// `(flush(n1), flush(n2))`: no magnitude and no NaN compares below `+0.0`. The caller chooses the
/// form once per block ([`crate::silence_armable_holding`]), so a block of live audio runs the
/// per-word law's frame loop with no threshold and no joint term; the rule's remaining cost there
/// is the caller's armability test and [`silence_skip_block`]'s last-frame check, once per block
/// per channel, plus a whole-block zero scan when a silent or padding lane shares the bank.
///
/// The choice is a type, so each instantiation of a kernel holds one form's frame loop and nothing
/// else; a caller that runs both forms instantiates its block body twice.
pub trait RestThresholds<L: Lane>: Copy {
    /// The rest plane, or `None` when every threshold is `+0.0` and none is read.
    fn plane(&self) -> Option<&[f32]>;
    /// The thresholds from word `base` on: a segment of a block that starts at frame
    /// `base / L::WIDTH`.
    #[must_use]
    fn skip(self, base: usize) -> Self;
}

impl<L: Lane> RestThresholds<L> for &[f32] {
    #[inline(always)]
    fn plane(&self) -> Option<&[f32]> {
        Some(self)
    }

    #[inline(always)]
    fn skip(self, base: usize) -> Self {
        &self[base..]
    }
}

/// The rest thresholds of a block in which no lane's joint flush can act (issue #1328): every
/// threshold is `+0.0`, nothing is loaded, and each SVF step runs the per-word law alone.
#[derive(Clone, Copy)]
pub struct UnarmedRest;

impl<L: Lane> RestThresholds<L> for UnarmedRest {
    #[inline(always)]
    fn plane(&self) -> Option<&[f32]> {
        None
    }

    #[inline(always)]
    fn skip(self, _base: usize) -> Self {
        self
    }
}

/// The rest planes of `S` streams for one kernel call: all armed or all unarmed (the caller arms
/// a whole block).
#[inline(always)]
fn stream_planes<L: Lane, R: RestThresholds<L>, const S: usize>(
    rest: &[R; S],
) -> Option<[&[f32]; S]> {
    let first = rest[0].plane()?;
    debug_assert!(rest.iter().all(|plane| plane.plane().is_some()));
    Some(core::array::from_fn(|stream| {
        rest[stream].plane().unwrap_or(first)
    }))
}

/// One frame's thresholds in a loop of form `ARMED`: loaded from the plane when armed, `+0.0`
/// (never read by the unarmed step) otherwise.
#[inline(always)]
fn threshold_at<L: Lane, const ARMED: bool>(plane: &[f32], base: usize) -> L {
    if ARMED {
        L::load(&plane[base..base + L::WIDTH])
    } else {
        L::zero()
    }
}

/// The lanes on which some word of `states` is neither `+0.0` nor `-0.0`; a NaN word counts
/// (issue #1328, [`crate::silence_armable_holding`]'s `holding`).
///
/// The unsigned maximum of the magnitudes' bits ([`Lane::max_u32`]) is zero exactly when every
/// magnitude is `+0.0`, and a NaN magnitude sorts above every other, so one compare at the end
/// decides the lane: two `abs` and two `max_u32` per state, then one `eq` and one `not`.
#[inline(always)]
pub fn svf_state_held<L: Lane>(states: impl IntoIterator<Item = SvfState<L>>) -> L::Mask {
    let mut held = L::zero();
    for state in states {
        held = held.max_u32(state.ic1.abs()).max_u32(state.ic2.abs());
    }
    L::mask_not(held.eq(L::zero()))
}

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
/// 1. `v0 = load(frame)`; `rest = load(rest plane, frame)`
/// 2. `v3 = v0 - ic2`
/// 3. `d1 = fma(-c1, ic1, a2 * v3)` — two multiplies, then an add; `fma` is unfused (#163)
/// 4. `v1 = ic1 + d1`
/// 5. `d2 = fma(a3, v3, a2 * ic1)` — `ic1` is still the old value here
/// 6. `v2 = ic2 + d2`
/// 7. `(ic1, ic2) = flush_pair(ic1 + (d1 + d1), ic2 + (d2 + d2), rest)` — `d1 + d1` and `d2 + d2`
///    are exact; the joint flush of [`crate::flush_pair`] (issue #1328), armed only on a lane whose
///    effect input has been exactly zero for `N_SILENCE` frames ([`crate::silence_frames`],
///    amendment A9)
/// 8. `y = fma(m2, v2, fma(m1, v1, m0 * v0))`
/// 9. `store(frame, y)`
///
/// `rest` is the **rest plane**: one threshold word per lane per frame, written by
/// [`silence_block`] from the effect's own input before any section of the effect runs, because a
/// section in a cascade sees its upstream section's output, not the effect input the silence law
/// is about. A block is `frames * L::WIDTH` words of it, frame-major like `io`.
///
/// `-c1` is computed once per block as a sign-bit flip, which is exact. Steps 2 to 7 are
/// [`svf_step`], which is the only copy of them: this kernel, [`svf_block_ramped`] and the fused
/// chain kernels of [`builtins`] all call it, so the numeric contract has one home.
#[inline(always)]
pub fn svf_block<L: Lane, R: RestThresholds<L>>(
    io: &mut [f32],
    frames: usize,
    c: &SvfCoef<L>,
    s: &mut SvfState<L>,
    rest: R,
) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    match rest.plane() {
        Some(plane) => svf_block_form::<L, true>(io, c, s, plane),
        None => svf_block_form::<L, false>(io, c, s, &[]),
    }
}

/// [`svf_block`]'s frame loop in one form ([`RestThresholds`]): `ARMED` selects the joint flush
/// and the threshold load, so each form is its own loop with no per-frame branch.
#[inline(always)]
fn svf_block_form<L: Lane, const ARMED: bool>(
    io: &mut [f32],
    c: &SvfCoef<L>,
    s: &mut SvfState<L>,
    rest: &[f32],
) {
    let rest = if ARMED { &rest[..io.len()] } else { rest };
    let mut state = *s;
    let nc1 = c.c1.neg();
    for (index, frame) in io.chunks_exact_mut(L::WIDTH).enumerate() {
        let v0 = L::load(frame);
        let threshold = threshold_at::<L, ARMED>(rest, index * L::WIDTH);
        let (v1, v2) = svf_step_when(ARMED, v0, nc1, c.a2, c.a3, threshold, &mut state);
        let y = c.m2.fma(v2, c.m1.fma(v1, c.m0.mul(v0)));
        y.store(frame);
    }
    *s = state;
}

/// The rest plane of one effect input channel over one block (issue #1328, amendment A9): advances
/// the channel's silence counter `run` frame by frame on `input`, and writes each frame's rest
/// threshold to `rest`.
///
/// Frozen operation order, per frame: `x = load(input, frame)`, then
/// `store(rest, frame, silence_step(x, run, armed_after))` ([`crate::silence_step`]). `input` is
/// the effect's input before any of its sections runs; `rest` is then read by every SVF section of
/// the effect channel through [`svf_block`], the ramped blocks and the cascades. Both are
/// `frames * L::WIDTH` words, frame-major. `armed_after` is `N_SILENCE` at the effect's rate
/// ([`crate::silence_frames`]).
#[inline(always)]
pub fn silence_block<L: Lane>(
    input: &[f32],
    frames: usize,
    run: &mut L,
    rest: &mut [f32],
    armed_after: L,
) {
    let span = frames * L::WIDTH;
    debug_assert!(input.len() >= span && rest.len() >= span);
    let mut counted = *run;
    for (frame, threshold) in input[..span]
        .chunks_exact(L::WIDTH)
        .zip(rest[..span].chunks_exact_mut(L::WIDTH))
    {
        silence_step(L::load(frame), &mut counted, armed_after).store(threshold);
    }
    *run = counted;
}

/// Advances a silence counter over one block of `input` without writing a rest plane, for a block
/// whose thresholds no section needs: one no lane's counter can arm ([`crate::silence_armable`]),
/// or one no section runs (issue #1328, amendment A9).
///
/// The counter is left exactly where [`silence_block`] would leave it, in one of three forms:
///
/// 1. **Live.** The block's last frame is non-zero on every lane -- any block of live audio: every
///    counter ends at `+0.0`, read off one frame with one `mask_any`.
/// 2. **Settled.** Otherwise a whole-block zero scan (a load, an `eq` and a `mask_and` per frame,
///    no counter and no loop-carried add) finds the lanes whose input is `±0.0` on every frame of
///    the block. When every lane is either such a lane or non-zero on its last frame -- a bank
///    with a silent or padding lane beside live ones, or a block of silence -- the counter is
///    `min(run + frames, 2^24)` ([`silence_advance`]) on the silent lanes and `+0.0` on the
///    others, which is what the frame loop computes: `run + 1` per zero frame is exact below
///    `2^24` and saturates there, and a lane non-zero on its last frame ends at `+0.0`.
/// 3. **Transition.** Some lane is zero on its last frame but not on every frame (its silence
///    starts inside the block): the frame loop runs [`crate::silence_step`]'s counter, three
///    operations per frame, with no threshold and no store.
///
/// The live form is inlined into each caller; the other two are one outlined function per lane
/// width ([`silence_skip_settle`]), so their constants are materialised once per width, not at
/// every call site (on `aarch64-apple-ios` a constant vector is a `memset_pattern16` call, #1018).
#[inline(always)]
pub fn silence_skip_block<L: Lane>(input: &[f32], frames: usize, run: &mut L) {
    if frames == 0 {
        return;
    }
    let span = frames * L::WIDTH;
    debug_assert!(input.len() >= span);
    let input = &input[..span];
    let ends_silent = L::load(&input[span - L::WIDTH..]).eq(L::zero());
    if !L::mask_any(ends_silent) {
        *run = L::zero();
        return;
    }
    silence_skip_settle(input, ends_silent, run);
}

/// [`silence_skip_block`]'s settled and transition forms over `input`, a whole block of
/// `L::WIDTH`-word frames whose last frame is zero on the lanes of `ends_silent`, some lane at
/// least.
#[inline(never)]
fn silence_skip_settle<L: Lane>(input: &[f32], ends_silent: L::Mask, run: &mut L) {
    let frames = input.len() / L::WIDTH;
    // Four independent `mask_and` chains, so the scan runs at the loads' throughput instead of
    // one `mask_and` latency per frame; `mask_and` is exact in any grouping.
    let mut silent = [ends_silent; 4];
    let mut quads = input.chunks_exact(4 * L::WIDTH);
    for quad in &mut quads {
        for (chain, frame) in silent.iter_mut().zip(quad.chunks_exact(L::WIDTH)) {
            *chain = L::mask_and(*chain, L::load(frame).eq(L::zero()));
        }
    }
    for frame in quads.remainder().chunks_exact(L::WIDTH) {
        silent[0] = L::mask_and(silent[0], L::load(frame).eq(L::zero()));
    }
    let silent = L::mask_and(
        L::mask_and(silent[0], silent[1]),
        L::mask_and(silent[2], silent[3]),
    );
    if !L::mask_any(L::mask_and(ends_silent, L::mask_not(silent))) {
        let mut advanced = *run;
        silence_advance(&mut advanced, frames);
        *run = L::select(silent, advanced, L::zero());
        return;
    }
    let one = L::splat(1.0);
    let mut counted = *run;
    for frame in input.chunks_exact(L::WIDTH) {
        counted = L::select(L::load(frame).eq(L::zero()), counted.add(one), L::zero());
    }
    *run = counted;
}

/// Advances a silence counter over `frames` frames of exactly-zero input without a frame loop:
/// `run = min(run + frames, 2^24)`, which is what [`silence_block`] leaves after such a block
/// (issue #1328, amendment A9).
///
/// For the paths that skip a silent block instead of rendering it (the EQ's silent fixed point).
/// Exact: below `2^24` the sum is an exact `f32` integer, and a rounded sum at or above `2^24`
/// becomes `2^24`, the value the frame-by-frame counter saturates at.
#[inline(always)]
pub fn silence_advance<L: Lane>(run: &mut L, frames: usize) {
    const SATURATED: f32 = 16_777_216.0;
    *run = L::min(run.add(L::splat(frames as f32)), L::splat(SATURATED));
}

/// `S` independent cascades of `D` [`svf_block`] sections each, run in one shared frame loop
/// (issue #163 phase 3).
///
/// # Why this exists
///
/// The TPT recurrence is first-order: frame `n`'s `ic1`/`ic2` are frame `n + 1`'s inputs. Within
/// one filter the block loop is therefore a serial dependency chain whose period is the *latency*
/// of `sub -> fma -> add -> add -> flush_pair`, while the frame body issues about a dozen vector
/// operations that the FMA ports could retire in a third of that. A lone chain leaves the vector
/// units idle most of the window -- which is why [`svf_block`] at `Simd4` and at `Simd8` take the
/// same wall time per chain-frame on the bench host. The kernel is latency-bound, not width-bound,
/// and the only cure is to have more independent recurrences in flight.
///
/// This kernel supplies them from the two places a bank has them: `S` **independent streams**
/// (a bank's left and right channels, which carry independent state by definition of dual-mono)
/// and `D` **cascade sections**, whose integrators are independent of each other even though their
/// audio is not -- section `k`'s state at frame `n + 1` depends on section `k`'s state at frame
/// `n`, never on section `k - 1`'s. Section `k - 1`'s *output* at frame `n` feeds section `k` at
/// the same frame, so the cascade is a forward chain inside the frame body and not a second
/// loop-carried dependency. `S * D` recurrences are therefore live at once.
///
/// `D` is [`Lane::SVF_CASCADE_DEPTH`], a per-backend constant fixed by measurement, not a runtime
/// knob: past the register file the compiler spills and the win reverses (measured; the table is
/// on that constant).
///
/// # What it does not change
///
/// Each `(stream, section)` chain runs [`svf_block`]'s frozen operation order, in that order, on
/// its own values. This function *is* `S * D` copies of that loop body with the loops merged and
/// the intermediate section outputs kept in registers instead of round-tripping through `io` --
/// and an `f32` store followed by an `f32` load of the same slot is the identity, so even that is
/// bit-preserving rather than merely close. Nothing is reassociated and no value crosses between
/// streams. Gate G2 pins it as an identity against a chain of [`svf_block`] calls
/// (`tests/g2_kernel_identity.rs`).
///
/// # Contract
///
/// The `S` blocks are distinct buffers of `frames * L::WIDTH` samples. `c[t][k]` and `s[t][k]` are
/// the coefficients and integrators of section `k` of stream `t`, in cascade order. `rest[t]` is
/// stream `t`'s rest plane ([`silence_block`]), as long as its block: every section of stream `t`
/// reads frame `f`'s threshold from it, the effect input's, whatever its position in the cascade.
/// Aliasing is unrepresentable: the blocks arrive as `&mut`, and the coefficient and state sets
/// are owned arrays.
#[inline(always)]
pub fn svf_cascade_interleaved<L: Lane, R: RestThresholds<L>, const S: usize, const D: usize>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [R; S],
) {
    svf_cascade_interleaved_impl(io, frames, c, s, rest, UnmaskedOutput, &mut Unobserved);
}

/// [`svf_cascade_interleaved`] with a bitwise per-lane dry-output selection for each section.
///
/// The recurrence and its state updates are exactly the same as the unmasked kernel. For each
/// frame, `dry_masks[stream][section]` selects the section input `x` when set and the ordinary SVF
/// output when clear. Selection happens at the section boundary, before the selected value feeds
/// the next section, so the dry input reaches that section with its bits intact. Downstream
/// processing then applies normally. Both arms are evaluated; an executed dry section therefore
/// retains the existing state evolution, including its `flush` behavior.
///
/// Masks must be canonical [`Lane::Mask`] values produced by the trait comparisons and mask
/// combinators. The array shape is fixed by the prepared cascade, and this function adds no
/// render-time allocation or scratch storage.
#[inline(always)]
pub fn svf_cascade_interleaved_with_dry_masks<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    dry_masks: &[[L::Mask; D]; S],
    rest: [R; S],
) {
    svf_cascade_interleaved_impl(
        io,
        frames,
        c,
        s,
        rest,
        MaskedCascadeOutput { masks: dry_masks },
        &mut Unobserved,
    );
}

/// [`svf_cascade_interleaved`] scheduled as a software pipeline: in iteration `i`, section `k` of
/// every stream runs frame `i - k`. Same chains, same order per chain, same bits.
///
/// # Why this exists (issue #978)
///
/// [`svf_cascade_interleaved`] runs section `0` then section `1` of each stream within one frame,
/// and section `1` consumes section `0`'s output of that same frame. Each frame's work is
/// therefore a dependency chain `D` sections long, and the out-of-order window does not overlap
/// enough of it across frames: at `D = 2` the frame body waits on two recurrence latencies back
/// to back. Skewing the pass by one frame per section breaks that chain. In iteration `i`,
/// section `k` takes its input from section `k - 1`'s output of the *previous* iteration (frame
/// `i - k`, held in a register), so the sections of one iteration are independent of each other
/// and only the recurrences themselves remain loop-carried.
///
/// # What it does not change
///
/// Every `(stream, section)` chain runs [`svf_step`] and the output mix
/// `m2.fma(v2, m1.fma(v1, m0.mul(x)))` on exactly the inputs, and in exactly the frame order, that
/// [`svf_cascade_interleaved`] gives it: section `k` of frame `f` still reads section `k - 1`'s
/// output of frame `f` (now from the carry rather than from a register of the same iteration) and
/// its own state after frame `f - 1`. Only the interleaving of independent chains moves, so the
/// output words and the state words are identical, `-0.0`, subnormals and NaN payloads included.
/// Gate G2 pins it against [`svf_cascade_interleaved`] (`tests/g2_kernel_identity.rs`).
///
/// # Schedule
///
/// * **Prologue**, iterations `0..D - 1`: section `k` runs only once frame `i - k` exists
///   (`k <= i`).
/// * **Steady state**, iterations `D - 1..frames`, branch-free: every section runs, visited from
///   `D - 1` down to `0` so that each consumes the previous iteration's carry before the section
///   ahead of it overwrites that carry.
/// * **Epilogue**, iterations `frames..frames + D - 1`: section `k` runs only while frame `i - k`
///   exists (`i - k < frames`), draining the pipeline.
///
/// Section `0` reads frame `i` from `io`, and section `D - 1` writes frame `i - (D - 1)` back, so
/// every frame is read before any section writes it (the frame written in iteration `i` was read
/// `D - 1` iterations earlier). With `frames < D` there is no steady state, and the call falls back
/// to [`svf_cascade_interleaved`] itself.
///
/// # Contract
///
/// As [`svf_cascade_interleaved`]: `S` distinct blocks of `frames * L::WIDTH` samples, `c[t][k]`
/// and `s[t][k]` the coefficients and integrators of section `k` of stream `t` in cascade order.
/// No allocation, no scratch beyond `S * D` carried vectors, no `unsafe`.
#[inline(always)]
pub fn svf_cascade_skewed<L: Lane, R: RestThresholds<L>, const S: usize, const D: usize>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [R; S],
) {
    svf_cascade_skewed_impl(io, frames, c, s, rest, UnmaskedOutput);
}

/// [`svf_cascade_skewed`] with [`svf_cascade_interleaved_with_dry_masks`]'s per-section dry
/// selection.
///
/// `dry_masks[stream][section]` selects the section input `x` where set and the wet output where
/// clear, at the same section boundary and on the same `x` as
/// [`svf_cascade_interleaved_with_dry_masks`]; the recurrence and its state updates are the
/// unmasked kernel's, so a dry lane keeps its state evolution. Same bits as that kernel.
#[inline(always)]
pub fn svf_cascade_skewed_with_dry_masks<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    dry_masks: &[[L::Mask; D]; S],
    rest: [R; S],
) {
    svf_cascade_skewed_impl(
        io,
        frames,
        c,
        s,
        rest,
        MaskedCascadeOutput { masks: dry_masks },
    );
}

/// [`svf_cascade_interleaved`] that also judges, per stream, every word it stores against
/// `limit` (issue #999).
///
/// Returns `verdict[t] = true` exactly when every word the kernel stored into `io[t]` has
/// `|y| < limit`. The compare is ordered, so a NaN word fails it, as does an infinity for any
/// finite `limit`. The rendered words and the state words are [`svf_cascade_interleaved`]'s, bit
/// for bit: the fold reads the output word and writes nothing but its own accumulator.
///
/// # Why it exists
///
/// A caller that applies a block-limit rule to the kernel's output -- the effects' master plan
/// §4.4 check, `effect_runtime::bank::check_block` -- would otherwise re-read both planes after
/// the cascade has written them. The cascade already holds every output word in a register when
/// it stores it, and its frame loop is latency-bound (see [`svf_cascade_interleaved`]), so the
/// fold's few independent operations per stored vector (an `abs`, a compare, a mask reduction and
/// an `or`) ride in the recurrence's shadow, where the separate scan costs a load, the same
/// predicate and a loop of its own.
///
/// # Frozen operation order
///
/// Per stream, before the first frame: `failed = false`. Per stored vector `y`, after the kernel
/// computes it and before it is stored: `failed = failed OR mask_any(NOT (|y| < limit))`. Once,
/// after the last frame: `verdict = NOT failed`. `check_block` folds the same per-word predicate
/// into a vector mask and reduces once; a disjunction of per-word failures does not depend on
/// where the reduction happens or in what order, so the verdict is the scan's on every block, NaN
/// payloads and signed zeros included.
///
/// # Contract
///
/// As [`svf_cascade_interleaved`]. With `frames = 0` nothing is stored and every verdict is
/// `true`, which is the scan's answer on an empty block.
#[inline(always)]
#[must_use]
pub fn svf_cascade_interleaved_bounded<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [R; S],
    limit: f32,
) -> [bool; S] {
    let mut bound = StoreBound::<L, S>::new(limit);
    svf_cascade_interleaved_impl(io, frames, c, s, rest, UnmaskedOutput, &mut bound);
    bound.verdict()
}

/// [`svf_cascade_interleaved_with_dry_masks`] with [`svf_cascade_interleaved_bounded`]'s verdict.
///
/// The judged word is the one stored: after the last section's dry selection.
#[inline(always)]
#[must_use]
pub fn svf_cascade_interleaved_with_dry_masks_bounded<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    dry_masks: &[[L::Mask; D]; S],
    rest: [R; S],
    limit: f32,
) -> [bool; S] {
    let mut bound = StoreBound::<L, S>::new(limit);
    svf_cascade_interleaved_impl(
        io,
        frames,
        c,
        s,
        rest,
        MaskedCascadeOutput { masks: dry_masks },
        &mut bound,
    );
    bound.verdict()
}

/// Shared skewed body. `M` is monomorphized exactly as in [`svf_cascade_interleaved_impl`].
#[inline(always)]
fn svf_cascade_skewed_impl<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
    M: SvfOutput<L>,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [R; S],
    output: M,
) {
    if frames < D {
        svf_cascade_interleaved_impl(io, frames, c, s, rest, output, &mut Unobserved);
        return;
    }
    match stream_planes(&rest) {
        None => {
            svf_cascade_skewed_form::<L, S, D, M, false>(io, frames, c, s, [&[]; S], output);
        }
        Some(planes) => {
            svf_cascade_skewed_form::<L, S, D, M, true>(io, frames, c, s, planes, output);
        }
    }
}

/// [`svf_cascade_skewed_impl`]'s schedule in one form ([`RestThresholds`]).
#[inline(always)]
fn svf_cascade_skewed_form<
    L: Lane,
    const S: usize,
    const D: usize,
    M: SvfOutput<L>,
    const ARMED: bool,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [&[f32]; S],
    output: M,
) {
    let width = L::WIDTH;
    let span = frames * width;
    debug_assert!(io.iter().all(|block| block.len() == span));
    // Truncating once, outside the loops, as the interleaved body does.
    let io = io.map(|block| &mut block[..span]);
    let rest = rest.map(|plane| if ARMED { &plane[..span] } else { plane });
    let mut state = *s;
    let nc1: [[L; D]; S] =
        core::array::from_fn(|stream| core::array::from_fn(|section| c[stream][section].c1.neg()));
    // `carry[t][k]`: section `k`'s output of the previous iteration, section `k + 1`'s input of
    // this one. The last section's slot is never read.
    let mut carry: [[L; D]; S] = [[L::zero(); D]; S];
    // One `(stream, section)` step of iteration `i`, on frame `i - section`: the interleaved
    // body's operations, in its order, on the input the skew hands it.
    macro_rules! step {
        ($i:expr, $stream:expr, $section:expr) => {{
            let (stream, section) = ($stream, $section);
            let base = ($i - section) * width;
            let x = if section == 0 {
                L::load(&io[stream][base..base + width])
            } else {
                carry[stream][section - 1]
            };
            let coefficients = &c[stream][section];
            // Frame `i - section`'s threshold: the effect input's, as every section of that frame
            // reads it in the interleaved body.
            let (v1, v2) = svf_step_when(
                ARMED,
                x,
                nc1[stream][section],
                coefficients.a2,
                coefficients.a3,
                threshold_at::<L, ARMED>(rest[stream], base),
                &mut state[stream][section],
            );
            let wet = coefficients
                .m2
                .fma(v2, coefficients.m1.fma(v1, coefficients.m0.mul(x)));
            let y = output.choose(x, wet, stream, section);
            if section == D - 1 {
                y.store(&mut io[stream][base..base + width]);
            } else {
                carry[stream][section] = y;
            }
        }};
    }
    for i in 0..D - 1 {
        for stream in 0..S {
            for section in (0..=i).rev() {
                step!(i, stream, section);
            }
        }
    }
    for i in D - 1..frames {
        for stream in 0..S {
            for section in (0..D).rev() {
                step!(i, stream, section);
            }
        }
    }
    for i in frames..frames + D - 1 {
        for stream in 0..S {
            for section in (i + 1 - frames..D).rev() {
                step!(i, stream, section);
            }
        }
    }
    *s = state;
}

/// The output policy is a zero-cost static choice: `UnmaskedOutput` is the original arithmetic,
/// while the two masked output policies add only their required bitwise selection.
trait SvfOutput<L: Lane> {
    fn choose(&self, dry: L, wet: L, stream: usize, section: usize) -> L;
}

struct UnmaskedOutput;

impl<L: Lane> SvfOutput<L> for UnmaskedOutput {
    #[inline(always)]
    fn choose(&self, _dry: L, wet: L, _stream: usize, _section: usize) -> L {
        wet
    }
}

struct MaskedCascadeOutput<'a, L: Lane, const S: usize, const D: usize> {
    masks: &'a [[L::Mask; D]; S],
}

impl<L: Lane, const S: usize, const D: usize> SvfOutput<L> for MaskedCascadeOutput<'_, L, S, D> {
    #[inline(always)]
    fn choose(&self, dry: L, wet: L, stream: usize, section: usize) -> L {
        L::select(self.masks[stream][section], dry, wet)
    }
}

struct MaskedBlockOutput<L: Lane> {
    mask: L::Mask,
}

impl<L: Lane> SvfOutput<L> for MaskedBlockOutput<L> {
    #[inline(always)]
    fn choose(&self, dry: L, wet: L, _stream: usize, _section: usize) -> L {
        L::select(self.mask, dry, wet)
    }
}

/// What a cascade kernel does with each output word besides storing it (issue #999).
///
/// A zero-cost static choice, like [`SvfOutput`]: [`Unobserved`] does nothing, so the entry points
/// that return no verdict keep their arithmetic and their loop, and [`StoreBound`] adds only its
/// fold.
trait StoreObserver<L: Lane> {
    fn observe(&mut self, stream: usize, y: L);
}

struct Unobserved;

impl<L: Lane> StoreObserver<L> for Unobserved {
    #[inline(always)]
    fn observe(&mut self, _stream: usize, _y: L) {}
}

/// The block-limit fold of [`svf_cascade_interleaved_bounded`]: one `bool` per stream.
///
/// # Why a `bool`, not a vector mask (issue #999, attempt 2)
///
/// The first form kept one vector mask per stream and folded `ok = ok AND (|y| < limit)` into it.
/// In the shipped `simd128` artifact V8 carried both masks through stack slots across the one-band
/// dual tail's back edge (stored at the loop top, reloaded before the `and`), a loop-carried value
/// through memory that the V8 spill gate (issue #1000) rightly refuses. Reducing each stored
/// vector's verdict to a scalar at once -- `failed |= mask_any(NOT (|y| < limit))` -- leaves the
/// carried value in a general-purpose register, of which the kernel has plenty, and takes no vector
/// register from the recurrence. It is the same predicate on the same words, and a disjunction of
/// per-vector verdicts is the scan's per-block one.
struct StoreBound<L: Lane, const S: usize> {
    limit: L,
    failed: [bool; S],
}

impl<L: Lane, const S: usize> StoreBound<L, S> {
    #[inline(always)]
    fn new(limit: f32) -> Self {
        Self {
            limit: L::splat(limit),
            failed: [false; S],
        }
    }

    #[inline(always)]
    fn verdict(&self) -> [bool; S] {
        core::array::from_fn(|stream| !self.failed[stream])
    }
}

impl<L: Lane, const S: usize> StoreObserver<L> for StoreBound<L, S> {
    #[inline(always)]
    fn observe(&mut self, stream: usize, y: L) {
        // Non-short-circuiting `|`: the reduction runs on every stored vector, branch-free.
        self.failed[stream] |= L::mask_any(L::mask_not(y.abs().lt(self.limit)));
    }
}

/// Shared cascade body. `M` is monomorphized, so old callers retain the original arithmetic
/// without a select and the additive entrypoint gets one boundary select per section. `O` is
/// monomorphized the same way: [`Unobserved`] adds nothing, and [`StoreBound`] folds each stored
/// word into its stream's verdict (issue #999).
#[inline(always)]
fn svf_cascade_interleaved_impl<
    L: Lane,
    R: RestThresholds<L>,
    const S: usize,
    const D: usize,
    M: SvfOutput<L>,
    O: StoreObserver<L>,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [R; S],
    output: M,
    observer: &mut O,
) {
    match stream_planes(&rest) {
        None => svf_cascade_interleaved_form::<L, S, D, M, O, false>(
            io,
            frames,
            c,
            s,
            [&[]; S],
            output,
            observer,
        ),
        Some(planes) => svf_cascade_interleaved_form::<L, S, D, M, O, true>(
            io, frames, c, s, planes, output, observer,
        ),
    }
}

/// [`svf_cascade_interleaved_impl`]'s frame loop in one form ([`RestThresholds`]).
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn svf_cascade_interleaved_form<
    L: Lane,
    const S: usize,
    const D: usize,
    M: SvfOutput<L>,
    O: StoreObserver<L>,
    const ARMED: bool,
>(
    io: [&mut [f32]; S],
    frames: usize,
    c: &[[SvfCoef<L>; D]; S],
    s: &mut [[SvfState<L>; D]; S],
    rest: [&[f32]; S],
    output: M,
    observer: &mut O,
) {
    let width = L::WIDTH;
    let span = frames * width;
    debug_assert!(io.iter().all(|block| block.len() == span));
    // Truncating once, outside the loop, is what lets the frame indexing below carry no per-frame
    // bounds branch: every stream then has exactly `span` samples, and so has every armed plane.
    let io = io.map(|block| &mut block[..span]);
    let rest = rest.map(|plane| if ARMED { &plane[..span] } else { plane });
    let mut state = *s;
    let nc1: [[L; D]; S] =
        core::array::from_fn(|stream| core::array::from_fn(|section| c[stream][section].c1.neg()));
    for frame in 0..frames {
        let base = frame * width;
        for stream in 0..S {
            let slot = &mut io[stream][base..base + width];
            let mut x = L::load(slot);
            // One threshold per stream and frame, read by every section of the frame.
            let threshold = threshold_at::<L, ARMED>(rest[stream], base);
            for section in 0..D {
                let coefficients = &c[stream][section];
                let (v1, v2) = svf_step_when(
                    ARMED,
                    x,
                    nc1[stream][section],
                    coefficients.a2,
                    coefficients.a3,
                    threshold,
                    &mut state[stream][section],
                );
                let wet = coefficients
                    .m2
                    .fma(v2, coefficients.m1.fma(v1, coefficients.m0.mul(x)));
                // The output policy is statically monomorphized. Masked policies select after
                // the recurrence, even for a dry lane, to retain its state updates.
                x = output.choose(x, wet, stream, section);
            }
            observer.observe(stream, x);
            x.store(slot);
        }
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
/// Steps 2 to 7 of [`svf_block`]'s frozen order, in that order — everything except the load, the
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
/// 6. `(ic1, ic2) = flush_pair(ic1 + (d1 + d1), ic2 + (d2 + d2), rest)` — `d1 + d1` and `d2 + d2`
///    are exact; the joint flush of [`crate::flush_pair`] (issue #1328), armed by `rest`, the
///    threshold [`crate::silence_step`] gave the **effect input's** frame (amendment A9) -- never
///    this section's own input `v0`, which inside a cascade is an upstream section's output
///
/// `rest` is `REST_EPS` on a lane whose effect input has been exactly zero for
/// `N_SILENCE` frames ([`crate::silence_frames`]) and `+0.0` on every other lane, where the pair
/// follows the per-word law bit for bit.
#[inline(always)]
pub fn svf_step<L: Lane>(v0: L, nc1: L, a2: L, a3: L, rest: L, s: &mut SvfState<L>) -> (L, L) {
    svf_step_when(true, v0, nc1, a2, a3, rest, s)
}

/// [`svf_step`] for a caller that knows, per block, whether any lane's rest threshold can be armed
/// (issue #1328, amendment A9).
///
/// `armable = true` is [`svf_step`] bit for bit. `armable = false` is for a block in which no lane's
/// silence counter can reach `N_SILENCE` ([`crate::silence_armable`]), so every threshold is
/// `+0.0`, or in which every lane that can arm is at rest ([`crate::silence_armable_holding`]):
/// step 6 is then two per-word [`flush`]es and `rest` is not read, which is the same bits, because
/// `flush_pair(n1, n2, +0.0)` is `(flush(n1), flush(n2))` (no magnitude and no NaN compares below
/// `+0.0`) and a lane at rest has nothing for the joint term to zero, and four lane-ops fewer. A frame loop calls it with a block-constant `armable`, so
/// the compiler unswitches the loop into the two forms.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn svf_step_when<L: Lane>(
    armable: bool,
    v0: L,
    nc1: L,
    a2: L,
    a3: L,
    rest: L,
    s: &mut SvfState<L>,
) -> (L, L) {
    let v3 = v0.sub(s.ic2);
    let d1 = nc1.fma(s.ic1, a2.mul(v3));
    let v1 = s.ic1.add(d1);
    let d2 = a3.fma(v3, a2.mul(s.ic1));
    let v2 = s.ic2.add(d2);
    let (n1, n2) = (s.ic1.add(d1.add(d1)), s.ic2.add(d2.add(d2)));
    (s.ic1, s.ic2) = if armable {
        flush_pair(n1, n2, rest)
    } else {
        (flush(n1), flush(n2))
    };
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
pub fn svf_block_ramped<L: Lane, R: RestThresholds<L>>(
    io: &mut [f32],
    frames: usize,
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    ramp_frames: usize,
    s: &mut SvfState<L>,
    rest: R,
) {
    svf_block_ramped_impl(io, frames, c, step, ramp_frames, s, rest, UnmaskedOutput);
}

/// [`svf_block_ramped`] with a bitwise per-lane dry-output selection.
///
/// The existing recurrence and current-then-advance ramp rule are unchanged. On every frame the
/// selected dry lane stores its input bits, while its state still follows [`svf_step`]. The mask is
/// constant for this block/segment; callers that split a ramp can provide a newly derived mask for
/// each segment. `ramp_frames = 0` is the stationary masked section case.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn svf_block_ramped_with_dry_mask<L: Lane, R: RestThresholds<L>>(
    io: &mut [f32],
    frames: usize,
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    ramp_frames: usize,
    s: &mut SvfState<L>,
    rest: R,
    dry_mask: L::Mask,
) {
    svf_block_ramped_impl(
        io,
        frames,
        c,
        step,
        ramp_frames,
        s,
        rest,
        MaskedBlockOutput { mask: dry_mask },
    );
}

/// Shared ramp body. `M` is monomorphized, so the original entrypoint retains its unmasked
/// arithmetic and the additive entrypoint adds only one output selection per frame.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn svf_block_ramped_impl<L: Lane, R: RestThresholds<L>, M: SvfOutput<L>>(
    io: &mut [f32],
    frames: usize,
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    ramp_frames: usize,
    s: &mut SvfState<L>,
    rest: R,
    output: M,
) {
    debug_assert_eq!(io.len(), frames * L::WIDTH);
    match rest.plane() {
        Some(plane) => {
            svf_block_ramped_form::<L, M, true>(io, c, step, ramp_frames, s, plane, output)
        }
        None => svf_block_ramped_form::<L, M, false>(io, c, step, ramp_frames, s, &[], output),
    }
}

/// [`svf_block_ramped_impl`]'s frame loop in one form ([`RestThresholds`]).
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn svf_block_ramped_form<L: Lane, M: SvfOutput<L>, const ARMED: bool>(
    io: &mut [f32],
    c: &mut SvfCoef<L>,
    step: &SvfCoefStep<L>,
    ramp_frames: usize,
    s: &mut SvfState<L>,
    rest: &[f32],
    output: M,
) {
    let rest = if ARMED { &rest[..io.len()] } else { rest };
    let mut state = *s;
    for (index, frame) in io.chunks_exact_mut(L::WIDTH).enumerate() {
        let nc1 = c.c1.neg();
        let v0 = L::load(frame);
        let threshold = threshold_at::<L, ARMED>(rest, index * L::WIDTH);
        let (v1, v2) = svf_step_when(ARMED, v0, nc1, c.a2, c.a3, threshold, &mut state);
        let wet = c.m2.fma(v2, c.m1.fma(v1, c.m0.mul(v0)));
        let y = output.choose(v0, wet, 0, 0);
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

/// One-pole smoother over one block: `y += c * (x - y)`.
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
/// 3. `y = fma(mix, d, x)` — a multiply then an add, two roundings (#163 phase 2)
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
/// 5. `g = ramp_toward(g, step, target)` -- `g + step` held inside `[min(g, target),
///    max(g, target)]`, so the gain never passes its target ([`ramp_toward`], issue #1409)
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
        g = ramp_toward(g, seg.step, seg.target);
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
    let a = &a[..count];
    let b = &b[..count];
    let (out_vectors, out_tail) = out.split_at_mut(vectored);
    let (a_vectors, a_tail) = a.split_at(vectored);
    let (b_vectors, b_tail) = b.split_at(vectored);
    for ((out, a), b) in out_vectors
        .chunks_exact_mut(L::WIDTH)
        .zip(a_vectors.chunks_exact(L::WIDTH))
        .zip(b_vectors.chunks_exact(L::WIDTH))
    {
        L::load(a).add(L::load(b)).store(out);
    }
    for ((out, a), b) in out_tail.iter_mut().zip(a_tail).zip(b_tail) {
        *out = <f32 as Lane>::add(*a, *b);
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
    let x = &x[..count];
    let (acc_vectors, acc_tail) = acc.split_at_mut(vectored);
    let (x_vectors, x_tail) = x.split_at(vectored);
    for (acc, x) in acc_vectors
        .chunks_exact_mut(L::WIDTH)
        .zip(x_vectors.chunks_exact(L::WIDTH))
    {
        L::load(acc).add(L::load(x)).store(acc);
    }
    for (acc, x) in acc_tail.iter_mut().zip(x_tail) {
        *acc = <f32 as Lane>::add(*acc, *x);
    }
}

/// Ordered accumulation of one to eight contributors into one planar block.
///
/// When `initial_store` is true, the first contributor is copied before the remaining contributors
/// are added. Otherwise the existing output is the prior master and every contributor is added in
/// order. All shape checks happen before the first write, and the bounded contributor count keeps
/// the render path free of unbounded work.
#[inline(always)]
pub fn ordered_accumulate_block<L: Lane>(
    out: &mut [f32],
    contributors: &[&[f32]],
    initial_store: bool,
) -> bool {
    if contributors.is_empty() || contributors.len() > 8 {
        return false;
    }
    if contributors.iter().any(|input| input.len() != out.len()) {
        return false;
    }
    let vectored = out.len() - out.len() % L::WIDTH;
    let first = usize::from(initial_store);
    let mut index = 0;
    while index < vectored {
        let mut value = if initial_store {
            L::load(&contributors[0][index..])
        } else {
            L::load(&out[index..])
        };
        for input in &contributors[first..] {
            value = value.add(L::load(&input[index..]));
        }
        value.store(&mut out[index..]);
        index += L::WIDTH;
    }
    while index < out.len() {
        let mut value = if initial_store {
            <f32 as Lane>::load(&contributors[0][index..])
        } else {
            <f32 as Lane>::load(&out[index..])
        };
        for input in &contributors[first..] {
            value = value.add(<f32 as Lane>::load(&input[index..]));
        }
        value.store(&mut out[index..]);
        index += 1;
    }
    true
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

/// The 2x2 route/pan matrix over one planar stereo block, in place (D3).
///
/// `c` is `[ll, lr, rl, rr]`, already carrying the route's linear gain: the compiler hands the
/// graph a separate `gain`, and the graph folds it into each coefficient **once, at bind**
/// (`ll' = gain * ll`, ...), so render never multiplies by the gain again. The op order is frozen:
///
/// ```text
/// l' = fma(lr', r, ll' * l)
/// r' = fma(rr', r, rl' * l)
/// ```
///
/// two multiplies and one add per output word, four roundings per frame instead of
/// the five the unfolded `gain * (ll * l + lr * r)` form spends. Both outputs are computed from
/// the frame's *original* `l` and `r`, so the in-place write is safe.
///
/// Frames are independent: the block is vectorised over frames and finished by the same body at
/// `L = f32`, so the result does not depend on the width.
#[inline(always)]
pub fn mix2x2_block<L: Lane>(left: &mut [f32], right: &mut [f32], c: [f32; 4]) {
    debug_assert_eq!(left.len(), right.len());
    let count = left.len();
    let vectored = count - count % L::WIDTH;
    let right = &mut right[..count];
    let (ll, lr, rl, rr) = (
        L::splat(c[0]),
        L::splat(c[1]),
        L::splat(c[2]),
        L::splat(c[3]),
    );
    let (left_vectors, left_tail) = left.split_at_mut(vectored);
    let (right_vectors, right_tail) = right.split_at_mut(vectored);
    for (left, right) in left_vectors
        .chunks_exact_mut(L::WIDTH)
        .zip(right_vectors.chunks_exact_mut(L::WIDTH))
    {
        let old_left = L::load(left);
        let old_right = L::load(right);
        lr.fma(old_right, ll.mul(old_left)).store(left);
        rr.fma(old_right, rl.mul(old_left)).store(right);
    }
    let (ll, lr, rl, rr) = (c[0], c[1], c[2], c[3]);
    for (left, right) in left_tail.iter_mut().zip(right_tail) {
        let old_left = *left;
        let old_right = *right;
        *left = <f32 as Lane>::fma(lr, old_right, <f32 as Lane>::mul(ll, old_left));
        *right = <f32 as Lane>::fma(rr, old_right, <f32 as Lane>::mul(rl, old_left));
    }
}

/// The longest [`IndexedRamp`] in frames: `2^22`.
///
/// Two facts rest on it. A frame index `k <= 2^22` converts to `f32` exactly, so `c(k)` is a
/// function of the integer index and not of a rounded one; and, while the step is a normal number,
/// the indexed law cannot pass its target before the snap while `(length - 1) * 3u < 1`,
/// `u = 2^-24` the unit roundoff, which holds up to about `2^22.4` (the submix design's VERIFY-2).
/// That bound assumes relative rounding. When `|target - start| < length * 2^-126` the step is
/// subnormal (or exactly `2^-126`) and its rounding error is absolute, up to `2^-150`: `k * step`
/// can then exceed `target - start` by less than `(length - 1) * 2^-150`, and the sum's rounding
/// onto the target's grid can double that, so `c(k)` can pass the target by less than
/// `(length - 1) * 2^-149` and never by more than `2^-128` (reached at `length = 2^22`). That is
/// inaudible; `c(k)` stays monotone for `k < length`, and the snap assigns `target` exactly,
/// stepping back by the overshoot. A caller refuses a longer length before a ramp exists.
pub const INDEXED_RAMP_LENGTH_MAXIMUM: u32 = 1 << 22;

/// The **indexed ramp** of a 2x2 route mix's four coefficients `[ll, lr, rl, rr]`.
///
/// The law, per coefficient, for the frame at ramp index `k`:
///
/// ```text
/// c(0) = start
/// c(k) = round(round(k * step) + start)    for 1 <= k < length,  as Lane::fma(k, step, start)
/// c(k) = target                            for k >= length,      assigned, never computed
/// step = round(round(target - start) / length)                   once, in `new`
/// ```
///
/// `k` is an integer `<= 2^22` ([`INDEXED_RAMP_LENGTH_MAXIMUM`]), converted to `f32` exactly, and
/// `Lane::fma` rounds twice on every target. `length == 0` is a step: every `k` yields `target`.
///
/// # Why it is not D11
///
/// D11, the law of the faders, matrices and effects ([`ramp_block`] and
/// `builtins::gain_mute_ramp_block`), carries `current = ramp_toward(current, step, target)` from
/// frame to frame, so a D11 coefficient's bits depend on the frame's history. Here `c(k)` is a pure
/// function of `k`: the ramp vectorises over frames, any later fused or folded traversal can compute a frame's
/// coefficients in any order, the snap frame is decided by `k` rather than by a running value, and
/// a settled ramp is `target` exactly, so a route that has ramped and settled mixes the bits a
/// freshly prepared plan mixes. D11 is unchanged and stays the law everywhere else.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexedRamp {
    /// `c(0)`: the coefficients the ramp starts from.
    pub start: [f32; 4],
    /// The per-frame increment, `round(round(target - start) / length)`; zero for a step.
    pub step: [f32; 4],
    /// The coefficients from `k = length` onward, exactly.
    pub target: [f32; 4],
    /// The ramp's length in frames, at most [`INDEXED_RAMP_LENGTH_MAXIMUM`]; zero for a step.
    pub length: u32,
}

impl IndexedRamp {
    /// A settled ramp: every index yields `target`.
    #[must_use]
    pub const fn settled(target: [f32; 4]) -> Self {
        Self {
            start: target,
            step: [0.0; 4],
            target,
            length: 0,
        }
    }

    /// A ramp from `start` to `target` over `length` frames.
    ///
    /// `length == 0` is a step to `target`. So is a ramp whose `round(target - start)` is not
    /// finite for any of the four coefficients: the decision is taken here, once per ramp, never
    /// in the per-frame body, so no `step` is ever NaN or infinite.
    ///
    /// `length` must not exceed [`INDEXED_RAMP_LENGTH_MAXIMUM`]; the caller refuses a longer one
    /// before a ramp exists (debug-asserted here).
    #[must_use]
    pub fn new(start: [f32; 4], target: [f32; 4], length: u32) -> Self {
        debug_assert!(length <= INDEXED_RAMP_LENGTH_MAXIMUM);
        if length == 0 {
            return Self::settled(target);
        }
        let mut difference = [0.0; 4];
        for ((difference, start), target) in difference.iter_mut().zip(start).zip(target) {
            *difference = <f32 as Lane>::sub(target, start);
        }
        if !difference.iter().all(|difference| difference.is_finite()) {
            return Self::settled(target);
        }
        // Exact: `length <= 2^22 < 2^24`.
        let frames = length as f32;
        Self {
            start,
            step: difference.map(|difference| <f32 as Lane>::div(difference, frames)),
            target,
            length,
        }
    }

    /// `c(k)`, exactly as [`route_mix_ramp_block`] computes it for frame index `k`.
    #[must_use]
    pub fn coefficients_at(&self, k: u32) -> [f32; 4] {
        if k >= self.length {
            return self.target;
        }
        if k == 0 {
            return self.start;
        }
        // Exact: `k < length <= 2^22`.
        let index = k as f32;
        let mut coefficients = [0.0; 4];
        for ((coefficient, step), start) in coefficients.iter_mut().zip(self.step).zip(self.start) {
            *coefficient = <f32 as Lane>::fma(index, step, start);
        }
        coefficients
    }
}

/// `1, 2, ..., 16`: the frame-index offsets of one vector's frames, `L::WIDTH` of them.
///
/// Sixteen covers every lane width the crate may grow; the kernel reads the first `L::WIDTH`.
const FRAME_INDEX_OFFSETS: [f32; 16] = [
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
];

/// The 2x2 route mix of [`mix2x2_block`] over one planar stereo block, in place, with the four
/// coefficients ramping by the [`IndexedRamp`] law.
///
/// Frame `f` (0-based) of the block takes ramp index `k = position + f + 1`. The block splits by
/// frame count, decided from `position` and `ramp.length` before any loop:
///
/// 1. the frames with `k < length` run the ramp body, vectorised over frames: a frame-index vector
///    `k0 + [1, ..., L::WIDTH]` (exact: every value is an integer below `2^24`) yields each frame's
///    coefficients with one `fma` each, and the frame is mixed in [`mix2x2_block`]'s frozen order:
///
///    ```text
///    c   = fma(k, step, start)            per coefficient
///    l'  = fma(c_lr, r, c_ll * l)
///    r'  = fma(c_rr, r, c_rl * l)
///    ```
///
/// 2. the ramp frames that do not fill a whole vector run the same arithmetic in a non-generic,
///    outlined `f32` function, so no scalar body is unrolled into a vector instantiation (the
///    browser's kernel-shape rule; the #926 lesson);
/// 3. the frames with `k >= length` are [`mix2x2_block`] with `ramp.target`, unchanged: settled
///    coefficients are assigned, never computed. Its whole vectors run at `L` and its last frames
///    in an outlined `f32` call, which is what `mix2x2_block::<L>` does inline.
///
/// `c(k)` is a pure function of `k`, so the result does not depend on the width. The caller
/// advances `position` by the block's length and saturates it at `ramp.length`; a `position` at or
/// past `length` renders [`mix2x2_block`] with the target.
///
/// `#[inline(never)]` keeps every instantiation a named function.
#[inline(never)]
pub fn route_mix_ramp_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    ramp: &IndexedRamp,
    position: u32,
) {
    debug_assert_eq!(left.len(), right.len());
    debug_assert!(ramp.length <= INDEXED_RAMP_LENGTH_MAXIMUM);
    // Both planes are cut to the shorter one, so no index below can fail: the kernel is never
    // inlined into `render_inner`, and a bounds check here would be its own trap owner, which the
    // browser's render-closure gate refuses.
    let count = core::cmp::min(left.len(), right.len());
    let (left, right) = (&mut left[..count], &mut right[..count]);
    // `k = position + f + 1 < length` holds for the first `length - position - 1` frames.
    let ramping = ramp.length.saturating_sub(position).saturating_sub(1);
    let ramping = core::cmp::min(count, ramping as usize);
    let vectored = ramping - ramping % L::WIDTH;
    let (left_ramp, left_settled) = left.split_at_mut(ramping);
    let (right_ramp, right_settled) = right.split_at_mut(ramping);
    let (left_vectors, left_tail) = left_ramp.split_at_mut(vectored);
    let (right_vectors, right_tail) = right_ramp.split_at_mut(vectored);

    let step = ramp.step.map(L::splat);
    let start = ramp.start.map(L::splat);
    let offsets = L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]);
    // Each chunk's index vector is built from a `u32` counter, not advanced by a splatted
    // `L::WIDTH`: that splat is a constant LLVM stores through `memset_pattern16` on iOS, a libc
    // call inside a render kernel (#1018's ratchet; the #1220 amendment). Exact either way:
    // `position + vectored < length <= 2^22` whenever a frame ramps.
    let mut first = position;
    for (left, right) in left_vectors
        .chunks_exact_mut(L::WIDTH)
        .zip(right_vectors.chunks_exact_mut(L::WIDTH))
    {
        let index = L::splat(first as f32).add(offsets);
        first += L::WIDTH as u32;
        let ll = index.fma(step[0], start[0]);
        let lr = index.fma(step[1], start[1]);
        let rl = index.fma(step[2], start[2]);
        let rr = index.fma(step[3], start[3]);
        let old_left = L::load(left);
        let old_right = L::load(right);
        lr.fma(old_right, ll.mul(old_left)).store(left);
        rr.fma(old_right, rl.mul(old_left)).store(right);
    }
    if !left_tail.is_empty() {
        // `vectored < ramping < length`, so the sum stays below `2^22`.
        route_mix_ramp_tail(left_tail, right_tail, ramp, position + vectored as u32 + 1);
    }
    let settled = left_settled.len();
    let settled_vectored = settled - settled % L::WIDTH;
    let (left_vectors, left_tail) = left_settled.split_at_mut(settled_vectored);
    let (right_vectors, right_tail) = right_settled.split_at_mut(settled_vectored);
    mix2x2_block::<L>(left_vectors, right_vectors, ramp.target);
    if !left_tail.is_empty() {
        route_mix_settled_tail(left_tail, right_tail, ramp.target);
    }
}

/// The settled frames of [`route_mix_ramp_block`] that do not fill a vector: [`mix2x2_block`] at
/// `f32`, which is exactly the tail `mix2x2_block::<L>` finishes with.
///
/// Outlined for the same reason as [`route_mix_ramp_tail`]: inlined, `mix2x2_block`'s `f32` tail
/// unrolls into the four-lane instantiation as 18 scalar operations beside its 21 vector ones
/// (measured on a `wasm32` `simd128` build), one edit away from the browser's kernel-shape rule.
#[inline(never)]
fn route_mix_settled_tail(left: &mut [f32], right: &mut [f32], c: [f32; 4]) {
    // Cut both planes here too: nothing inside this outlined function proves their lengths equal,
    // so `mix2x2_block`'s own `&mut right[..count]` would otherwise keep a trap.
    let count = core::cmp::min(left.len(), right.len());
    mix2x2_block::<f32>(&mut left[..count], &mut right[..count], c);
}

/// The ramp frames of [`route_mix_ramp_block`] that do not fill a vector: the same arithmetic at
/// `f32`, frame `f` at ramp index `first + f`.
///
/// Non-generic and never inlined, so the browser's four-lane kernel instantiation carries no
/// scalar ramp body (the #926 lesson).
#[inline(never)]
fn route_mix_ramp_tail(left: &mut [f32], right: &mut [f32], ramp: &IndexedRamp, first: u32) {
    let [step_ll, step_lr, step_rl, step_rr] = ramp.step;
    let [start_ll, start_lr, start_rl, start_rr] = ramp.start;
    for ((left, right), k) in left.iter_mut().zip(right.iter_mut()).zip(first..) {
        // Exact: `k < length <= 2^22`.
        let index = k as f32;
        let ll = <f32 as Lane>::fma(index, step_ll, start_ll);
        let lr = <f32 as Lane>::fma(index, step_lr, start_lr);
        let rl = <f32 as Lane>::fma(index, step_rl, start_rl);
        let rr = <f32 as Lane>::fma(index, step_rr, start_rr);
        let old_left = *left;
        let old_right = *right;
        *left = <f32 as Lane>::fma(lr, old_right, <f32 as Lane>::mul(ll, old_left));
        *right = <f32 as Lane>::fma(rr, old_right, <f32 as Lane>::mul(rl, old_left));
    }
}
