//! Conflating single-writer observation cells (issue #143 D4).
//!
//! # Why this is not a queue
//!
//! An observation is a *level*, not an *event*. A meter that missed three windows does not want to
//! replay them; it wants the newest one. A queue would make the render thread's cost depend on
//! whether a control thread happened to drain it, and a full queue would force the render side to
//! choose between blocking, allocating and dropping -- three answers a render thread may not give.
//! A conflating cell has exactly one answer: the writer overwrites, always, in bounded time, and
//! the reader takes whatever is there. Nothing is queued, so nothing can back up.
//!
//! # The seqlock, spelled in safe Rust
//!
//! Every published field is its own atomic word and the whole window is fenced by an odd/even
//! sequence counter: the writer makes it odd, stores the fields, makes it even; a reader that sees
//! an odd counter, or a different counter before and after, retries. There is no `UnsafeCell` and
//! no `unsafe` block, which matters because this module is inside the realtime root that
//! `scripts/check-realtime-policy.sh` holds to an approved-unsafe list of exactly two files.
//!
//! Under `wasm32-unknown-unknown` without the `atomics` target feature -- the browser-local
//! fallback the AudioWorklet ships -- every operation below lowers to a plain load or store, which
//! is what `scripts/check-wasm-realtime-atomics.sh` inspects and requires.
//!
//! # Read-reset without a second writer
//!
//! The reader owns one word of its own, `consumed_sequence`, and is its only writer. It is not a
//! back channel that gates publication: the writer never waits for it and never changes what it
//! publishes because of it. It exists so that a stalled reader can *see* what it missed --
//! `sequence - consumed_sequence - 1` is the exact number of windows that were overwritten -- which
//! turns "the meter froze" from an invisible failure into a counted one.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering, fence};
use std::sync::Arc;

/// One closed observation window: what a consumer reads, and the only thing that crosses.
///
/// `first_sample` and `end_sample` are absolute render-clock samples and are half-open
/// (`[first_sample, end_sample)`), so consecutive windows tile with no gap and no overlap. That is
/// what lets a consumer correlate a window against the `applied_at_sample` a command was
/// acknowledged with, rather than against a wall clock nobody shares.
///
/// `sequence` starts at `1` for the first published window and increases by one per window, so a
/// default-valued (`sequence == 0`) read means "nothing has been published yet" and is reported as
/// absent rather than as a window of zeros.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObservationWindow {
    /// Absolute sample the window opened at, inclusive.
    pub first_sample: u64,
    /// Absolute sample the window closed at, exclusive.
    pub end_sample: u64,
    /// Monotonic window number for this slot; `0` only before anything was published.
    pub sequence: u64,
    /// Render blocks folded into the window.
    pub blocks: u32,
    /// Left lane, already folded by the tap's declared rule.
    pub left: f32,
    /// Right lane, already folded by the tap's declared rule.
    pub right: f32,
}

/// Bounded retries before a read gives up and reports absence.
///
/// In production a writer publishes once per closed window -- once per `window_blocks` render
/// blocks -- so a retry needs the read to land inside one seven-word store and is vanishingly
/// rare. The number is set for the *stress* case instead: `E11` runs a writer that publishes
/// continuously in a tight loop, and at four attempts that measured a 50% give-up rate, because
/// four six-word reads are comparable in length to the store they are racing. Sixty-four takes the
/// measured give-up rate to zero there while keeping the loop provably finite, which is what the
/// bound is actually for. A reader is never on the render thread, so a retry costs a render block
/// nothing.
const MAXIMUM_READ_ATTEMPTS: usize = 64;

/// The shared conflating cell. Not constructed directly: [`observation_slot`] makes the pair.
#[derive(Debug, Default)]
pub struct ObservationSlot {
    /// Odd while a publication is in flight, even between them.
    sequence_lock: AtomicU32,
    first_sample: AtomicU64,
    end_sample: AtomicU64,
    sequence: AtomicU64,
    blocks: AtomicU32,
    /// `f32::to_bits`, so the published word is exact and `-0.0` survives.
    left: AtomicU32,
    right: AtomicU32,
    /// The reader's own single-writer word.
    consumed: AtomicU64,
}

/// Exact engine-owned retained payload of one observation slot.
///
/// The cell is one allocation shared by the two endpoints; the endpoints themselves are one
/// pointer each and are counted by whatever owns them.
#[must_use]
pub const fn observation_slot_retained_bytes() -> usize {
    core::mem::size_of::<ObservationSlot>()
}

/// The render-side half. Wait-free, single writer, never blocks and never allocates.
#[derive(Debug)]
pub struct ObservationPublisher {
    slot: Arc<ObservationSlot>,
}

/// The control-side half. Reads off the render thread and may retry.
#[derive(Debug)]
pub struct ObservationReader {
    slot: Arc<ObservationSlot>,
}

/// Build one conflating slot and its two endpoints. The only allocation this module makes.
#[must_use]
pub fn observation_slot() -> (ObservationPublisher, ObservationReader) {
    let slot = Arc::new(ObservationSlot::default());
    let reader = Arc::clone(&slot);
    (
        ObservationPublisher { slot },
        ObservationReader { slot: reader },
    )
}

// REALTIME_POLICY_BEGIN
impl ObservationPublisher {
    /// Overwrite the cell with `window`. Wait-free: seven stores and two counter stores.
    ///
    /// Latest wins, always. There is no full, no backpressure and no return value, because there
    /// is no outcome a render thread could act on.
    pub fn publish(&self, window: ObservationWindow) {
        let opening = self
            .slot
            .sequence_lock
            .load(Ordering::Relaxed)
            .wrapping_add(1);
        self.slot.sequence_lock.store(opening, Ordering::Relaxed);
        // Keeps the field stores below from being observed before the counter went odd.
        fence(Ordering::Release);
        self.slot
            .first_sample
            .store(window.first_sample, Ordering::Relaxed);
        self.slot
            .end_sample
            .store(window.end_sample, Ordering::Relaxed);
        self.slot.sequence.store(window.sequence, Ordering::Relaxed);
        self.slot.blocks.store(window.blocks, Ordering::Relaxed);
        self.slot
            .left
            .store(window.left.to_bits(), Ordering::Relaxed);
        self.slot
            .right
            .store(window.right.to_bits(), Ordering::Relaxed);
        self.slot
            .sequence_lock
            .store(opening.wrapping_add(1), Ordering::Release);
    }

    /// The newest window sequence the reader says it has consumed. One load.
    #[must_use]
    pub fn consumed_sequence(&self) -> u64 {
        self.slot.consumed.load(Ordering::Acquire)
    }
}
// REALTIME_POLICY_END

impl ObservationReader {
    /// The newest whole window, or `None` if nothing has been published or the read tore.
    ///
    /// Never a partial window: the fields are taken between two equal even counter reads, so what
    /// comes back is exactly one publication.
    #[must_use]
    pub fn read(&self) -> Option<ObservationWindow> {
        for _ in 0..MAXIMUM_READ_ATTEMPTS {
            let before = self.slot.sequence_lock.load(Ordering::Acquire);
            if !before.is_multiple_of(2) {
                continue;
            }
            let window = ObservationWindow {
                first_sample: self.slot.first_sample.load(Ordering::Relaxed),
                end_sample: self.slot.end_sample.load(Ordering::Relaxed),
                sequence: self.slot.sequence.load(Ordering::Relaxed),
                blocks: self.slot.blocks.load(Ordering::Relaxed),
                left: f32::from_bits(self.slot.left.load(Ordering::Relaxed)),
                right: f32::from_bits(self.slot.right.load(Ordering::Relaxed)),
            };
            fence(Ordering::Acquire);
            if self.slot.sequence_lock.load(Ordering::Relaxed) == before {
                return (window.sequence != 0).then_some(window);
            }
        }
        None
    }

    /// Record that everything up to and including `sequence` has been consumed.
    ///
    /// Monotonic by construction on a single reader; a caller that hands back an older sequence is
    /// ignored rather than allowed to move the word backwards.
    pub fn acknowledge(&self, sequence: u64) {
        if sequence > self.slot.consumed.load(Ordering::Relaxed) {
            self.slot.consumed.store(sequence, Ordering::Release);
        }
    }

    /// The last sequence this reader acknowledged.
    #[must_use]
    pub fn consumed_sequence(&self) -> u64 {
        self.slot.consumed.load(Ordering::Relaxed)
    }

    /// Windows published and overwritten without ever being read, for the sequence just read.
    ///
    /// `0` for a reader that is keeping up. This is the whole reason `consumed_sequence` exists:
    /// a conflating cell drops by design, and a drop nobody can count is indistinguishable from a
    /// meter that stopped moving.
    #[must_use]
    pub fn missed_windows(&self, sequence: u64) -> u64 {
        sequence
            .saturating_sub(self.consumed_sequence())
            .saturating_sub(1)
    }
}
