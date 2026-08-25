//! The homogeneous-bank driver and the once-per-block output boundary check.
//!
//! A bank is `WIDTH` tracks processed as one vector: a chain of slots, each slot one kernel with
//! one coefficient set and one state set covering every lane. Every effect in the workspace had
//! its own copy of the same three things — allocate the arrays at prepare, make an absent lane an
//! identity, check the output once per block — with the third one written per *value* instead of
//! per block. This module is the single copy, and the check is the D7 / master plan §4.4 form.
//!
//! # Boundary check, not sanitisation
//!
//! Decision D7: there is no per-value `is_finite`, `sanitize` or `recover` anywhere on a render
//! path. Denormals are handled by `miso_engine_lane::flush` inside the recurrence, and the output
//! of a bank is inspected **once per block** with a vector compare. A block that fails is zeroed,
//! the bank's state is reset, and a counter is incremented; the failure is a bug report, not a
//! signal-processing feature.
//!
//! The check is `!(|x| < 1e30)`, which is exactly `|x| >= 1e30 or x is NaN` because an ordered
//! compare against NaN is false. One `abs`, one compare and one mask-and per frame, and a single
//! `mask_any` for the whole block — no horizontal reduction inside the loop.

use alloc::vec;
use alloc::vec::Vec;

use miso_engine_lane::Lane;

/// Magnitude at or above which a block is rejected as non-finite (master plan §4.4).
///
/// `1e30` is about 60 dB above the largest level any correct signal path can reach and about
/// `3.4e8` times below `f32::MAX`, so it catches a diverging recurrence long before it reaches
/// infinity while leaving every legitimate value — including the loudest possible intermediate of
/// a well-conditioned filter — untouched.
pub const BLOCK_LIMIT: f32 = 1.0e30;

/// Running record of the boundary check for one bank.
///
/// `nonfinite_blocks` is the counter the master plan requires; it is cumulative for the life of
/// the prepared plan and is read off the render thread. `nonfinite_lanes` is the lane bitmask of
/// the most recent failure, which is what tells an operator *which track* diverged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NonFiniteReport {
    /// Number of blocks rejected since preparation.
    pub nonfinite_blocks: u64,
    /// Bit `l` is set if lane `l` was out of bounds in the most recently rejected block. `0` while
    /// no block has been rejected.
    pub nonfinite_lanes: u32,
}

impl NonFiniteReport {
    /// A report with no failures.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nonfinite_blocks: 0,
            nonfinite_lanes: 0,
        }
    }

    /// `true` if any block has ever been rejected.
    #[must_use]
    pub const fn tripped(&self) -> bool {
        self.nonfinite_blocks != 0
    }
}

/// `true` if every value in `io` is finite and below [`BLOCK_LIMIT`] in magnitude.
///
/// One vector scan of the block (master plan §4.4). Frozen operation order, per frame:
/// 1. `x = load(frame)`
/// 2. `ok = ok AND (|x| < BLOCK_LIMIT)` — the ordered compare is false for NaN, so the NaN case
///    needs no separate `x == x` term
///
/// and once for the whole block: `mask_any(NOT ok)`. `mask_any` is the only operation that leaves
/// the vector domain and it happens once per block, never per sample.
///
/// # Panics
///
/// Panics in debug builds if `io` is not a whole number of `L::WIDTH` frames.
#[inline(always)]
#[must_use]
pub fn check_block<L: Lane>(io: &[f32]) -> bool {
    debug_assert_eq!(io.len() % L::WIDTH, 0);
    let limit = L::splat(BLOCK_LIMIT);
    let mut ok = L::zero().eq(L::zero());
    for frame in io.chunks_exact(L::WIDTH) {
        let x = L::load(frame);
        ok = L::mask_and(ok, x.abs().lt(limit));
    }
    !L::mask_any(L::mask_not(ok))
}

/// `true` when every word of `io` is **exactly** `+0.0` (bit pattern zero).
///
/// # Why bits and not `== 0.0` (issue #163 phase 4 item 1)
///
/// This is the admission test for the silent fast path, and the fast path's whole claim is that
/// leaving a buffer untouched is bit-identical to what the kernel would have written into it. A
/// float compare cannot carry that claim: `-0.0 == 0.0` is `true`, so a block of negative zeros
/// would pass while the bits differ from what an untouched `+0.0` buffer holds. Reducing the raw
/// `u32` patterns instead makes the test exactly as strict as the claim -- `+0.0` is the only
/// `f32` whose bits are zero, so `bits == 0` is "every word is `+0.0`" and nothing else.
///
/// # Why `-0.0` is excluded on the **input** side too (#163 phase 4, adversarial pass)
///
/// The paragraph above is about state. The input side needs its own argument, because "this block
/// is all zeros, so skipping is free" is *false* for negative zero, and it is false in the
/// direction that loses bits.
///
/// Measured, not assumed: a settled parametric-EQ bank handed one block of all `-0.0` writes back
/// all `+0.0`. The SVF's `x - ic2` at `ic2 = +0.0` does give `-0.0`, but the output sum
/// `m0*x + m1*v1 + m2*v2` mixes signed zeros and `(-0.0) + (+0.0)` is `+0.0` under
/// round-to-nearest. The compressor reaches the same place by a different route: a `-0.0` block is
/// written into its lookahead ring and emerges some seven blocks later, so skipping it changes a
/// sample that is not even in this block.
///
/// So a sign-blind predicate would let a claim earned on `+0.0` engage on a `-0.0` block, and the
/// skip would leave the buffer holding `-0.0` where the kernel writes `+0.0` — different bit
/// patterns for the same block, which is precisely the class-A promise this fast path makes. The
/// strict test declines such a block and renders it through the real kernel like any other input.
///
/// Both directions are pinned by `a_negative_zero_input_block_is_not_treated_as_silence`, in
/// `miso-engine-parametric-eq` and `miso-engine-compressor`. Masking the sign bit here
/// (`bits |= value.to_bits() & 0x7fff_ffff`) leaves every other test in the workspace green and
/// reddens exactly those two.
///
/// Chunked so the **active** path stays cheap. A block carrying signal returns on its first
/// chunk, so a rendering console pays 32 words rather than a whole extra pass over
/// `frames * lanes`. The reduction is a plain `|` fold, which vectorises.
#[inline]
#[must_use]
pub fn block_is_positive_zero(io: &[f32]) -> bool {
    for chunk in io.chunks(32) {
        let mut bits = 0_u32;
        for value in chunk {
            bits |= value.to_bits();
        }
        if bits != 0 {
            return false;
        }
    }
    true
}

/// `true` when every lane of `value` is **exactly** `+0.0`, by the same bit rule as
/// [`block_is_positive_zero`].
///
/// Used on state words rather than audio: a recursive kernel's fixed point is only usable by the
/// fast path if it is the *exact* word the slow path would have left behind.
#[inline]
#[must_use]
pub fn lane_is_positive_zero<L: Lane>(value: L) -> bool {
    debug_assert!(L::WIDTH <= 32);
    let mut words = [0_u32; 32];
    value.store_bits(&mut words[..L::WIDTH]);
    let mut bits = 0_u32;
    for word in words.iter().take(L::WIDTH) {
        bits |= *word;
    }
    bits == 0
}

/// The lane bitmask of the values in `io` that are out of bounds.
///
/// Only called on the failing path, once, to attribute a rejected block to a track. Bit `l` is set
/// if any frame of lane `l` was NaN or at least [`BLOCK_LIMIT`] in magnitude.
///
/// # Panics
///
/// Panics in debug builds if `io` is not a whole number of `L::WIDTH` frames, or if `L::WIDTH`
/// exceeds 32 (no such width exists: the three backends are 1, 4 and 8).
#[inline]
#[must_use]
pub fn nonfinite_lane_mask<L: Lane>(io: &[f32]) -> u32 {
    debug_assert_eq!(io.len() % L::WIDTH, 0);
    debug_assert!(L::WIDTH <= 32);
    let limit = L::splat(BLOCK_LIMIT);
    let mut ok = L::zero().eq(L::zero());
    for frame in io.chunks_exact(L::WIDTH) {
        let x = L::load(frame);
        ok = L::mask_and(ok, x.abs().lt(limit));
    }
    let mut words = [0u32; 32];
    L::select(ok, L::zero(), L::splat(1.0)).store_bits(&mut words);
    let mut mask = 0u32;
    for (lane, word) in words.iter().take(L::WIDTH).enumerate() {
        if *word != 0 {
            mask |= 1 << lane;
        }
    }
    mask
}

/// Applies the master plan §4.4 policy to one bank's output block.
///
/// Returns `true` if the block was accepted. On rejection: both output blocks are zeroed, `reset`
/// is called so the bank can restore its kernel states to their defaults, the failing lane mask is
/// recorded and `nonfinite_blocks` is incremented. The counter counts *blocks*, so a bank that
/// diverges for a second at 48 kHz with a quantum of 128 adds 375, not 48 000.
///
/// Left and right are checked and zeroed together: a bank's two channels share their coefficients
/// and their reset, so accepting one while rejecting the other would leave the pair inconsistent.
#[inline]
pub fn finish_block<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    report: &mut NonFiniteReport,
    reset: impl FnOnce(),
) -> bool {
    if check_block::<L>(left) && check_block::<L>(right) {
        return true;
    }
    report.nonfinite_lanes = nonfinite_lane_mask::<L>(left) | nonfinite_lane_mask::<L>(right);
    report.nonfinite_blocks = report.nonfinite_blocks.saturating_add(1);
    left.fill(0.0);
    right.fill(0.0);
    reset();
    false
}

/// Applies the master plan section 4.4 policy to **one channel** of one block.
///
/// Returns `0` when the block is clean. Otherwise the block is zeroed, `reset` is called, and the
/// bitmask of the lanes that were out of bounds is returned so that the caller can attribute the
/// failure to a track — which is what an effect with one report per lane needs and what a single
/// `bool` cannot carry.
///
/// # Why this exists next to [`finish_block`]
///
/// [`finish_block`] checks and zeroes the two channels **together**, because a
/// [`HomogeneousBank`] slot shares one coefficient set and one reset between them: accepting one
/// and rejecting the other would leave the pair inconsistent.
///
/// A stereo effect whose channels carry *separate* state — separate rings, separate cursors,
/// separate recurrences, which is every dynamics processor running `LinkMode::DualMono` — has the
/// opposite requirement. Its left channel is exactly correct when its right diverged, and zeroing
/// it would destroy evidence rather than protect anything. Under a linked detector the two fail
/// together anyway, because the level that diverged reaches both, so the coupled case is not lost.
///
/// The check itself is identical in both: [`check_block`] once per channel, one `mask_any` for the
/// whole block, no horizontal reduction inside the loop.
///
/// Two shipped crates already open-code exactly this — `miso-engine-parametric-eq` and
/// `miso-engine-gate-expander` both write `if check_block(block) { … }` followed by
/// `nonfinite_lane_mask(block)` per channel — which is the divergence this module exists to stop.
/// They are not rewritten here: each is another issue's file. `miso-engine-compressor` is the
/// first caller.
#[inline]
pub fn finish_channel<L: Lane>(io: &mut [f32], reset: impl FnOnce()) -> u32 {
    if check_block::<L>(io) {
        return 0;
    }
    let mask = nonfinite_lane_mask::<L>(io);
    io.fill(0.0);
    reset();
    mask
}

/// One slot type of a homogeneous bank: a kernel plus the shapes it carries.
///
/// Implemented once per effect, generic over [`Lane`]. Everything the driver needs to allocate,
/// reset and run a bank is here, so [`HomogeneousBank`] itself has no effect-specific code at all.
pub trait BankKernel<L: Lane> {
    /// Per-lane coefficients of one slot.
    type Coef: Copy;
    /// Per-lane state of one slot.
    type State: Copy + Default;

    /// The coefficients that make the slot an exact identity on every lane.
    ///
    /// An absent slot in a cohort is not skipped — that would make the bank's shape depend on its
    /// membership — it is run with these. "Exact" is the requirement: the output must be the input
    /// bit for bit, not within a rounding error of it.
    fn identity_coef() -> Self::Coef;

    /// Processes one block of one slot in place.
    ///
    /// `io.len()` is `frames * L::WIDTH` and is validated at prepare time; this is a render-path
    /// call and must not branch on shape, allocate or return a `Result`.
    fn process_block(io: &mut [f32], frames: usize, coef: &Self::Coef, state: &mut Self::State);
}

/// A prepared homogeneous bank: `slots` kernels over `L::WIDTH` lanes, for two channels.
///
/// All allocation happens in [`HomogeneousBank::prepare`]. [`HomogeneousBank::process_block`]
/// touches nothing but the preallocated arrays and the caller's blocks.
pub struct HomogeneousBank<L: Lane, K: BankKernel<L>> {
    coefficients: Vec<K::Coef>,
    left: Vec<K::State>,
    right: Vec<K::State>,
    report: NonFiniteReport,
    lane: core::marker::PhantomData<fn() -> L>,
}

impl<L: Lane, K: BankKernel<L>> HomogeneousBank<L, K> {
    /// Allocates a bank of `slots` slots, every slot an identity, every state at its default.
    ///
    /// The only allocating function in this crate. It runs on the control plane, at plan
    /// preparation; after it returns, the bank never allocates again.
    #[must_use]
    pub fn prepare(slots: usize) -> Self {
        Self {
            coefficients: vec![K::identity_coef(); slots],
            left: vec![K::State::default(); slots],
            right: vec![K::State::default(); slots],
            report: NonFiniteReport::new(),
            lane: core::marker::PhantomData,
        }
    }

    /// Number of slots in the bank.
    #[must_use]
    pub fn slots(&self) -> usize {
        self.coefficients.len()
    }

    /// The bank's boundary-check record.
    #[must_use]
    pub fn report(&self) -> NonFiniteReport {
        self.report
    }

    /// The coefficient set of one slot, for a control-plane update.
    ///
    /// # Panics
    ///
    /// Panics if `slot` is out of range. Slot indices come from the prepared plan.
    pub fn coefficients_mut(&mut self, slot: usize) -> &mut K::Coef {
        &mut self.coefficients[slot]
    }

    /// Restores every slot of both channels to its default state.
    ///
    /// This is the one discontinuity reset: a seek, a transport stop, a plan swap and a rejected
    /// block all use it, so a reset cannot mean two different things in two effects. Coefficients
    /// are deliberately left alone — a reset clears history, it does not undo automation.
    pub fn reset(&mut self) {
        for state in &mut self.left {
            *state = K::State::default();
        }
        for state in &mut self.right {
            *state = K::State::default();
        }
    }

    /// Runs every slot over both channels and applies the §4.4 boundary check.
    ///
    /// Returns `true` if the block was accepted. Zero allocation, no locks, no syscalls; the only
    /// branch is the once-per-block check itself.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if either block is not `frames * L::WIDTH` long.
    #[inline]
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32], frames: usize) -> bool {
        debug_assert_eq!(left.len(), frames * L::WIDTH);
        debug_assert_eq!(right.len(), frames * L::WIDTH);
        for (slot, coefficients) in self.coefficients.iter().enumerate() {
            K::process_block(left, frames, coefficients, &mut self.left[slot]);
            K::process_block(right, frames, coefficients, &mut self.right[slot]);
        }
        let states = (&mut self.left, &mut self.right);
        let report = &mut self.report;
        finish_block::<L>(left, right, report, || {
            for state in states.0.iter_mut() {
                *state = K::State::default();
            }
            for state in states.1.iter_mut() {
                *state = K::State::default();
            }
        })
    }
}
