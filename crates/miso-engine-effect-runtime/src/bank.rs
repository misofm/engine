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
