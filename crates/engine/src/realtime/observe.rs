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
//! # The seqlock
//!
//! A window travels as five `u64` words through the engine's one single-writer seqlock
//! (`seqlock.rs`, #1482 D3), embedded in the slot so the slot stays one allocation. The tap's arm
//! state (armed, the arm sample and the arm tag, #1585 D2) is three more words of the same record,
//! so a reader takes the window and the arm state of one publication. The publisher is the
//! record's only writer by ownership: it is not `Clone`. A read is bounded to
//! `seqlock::MAXIMUM_READ_ATTEMPTS` attempts and reports absence when every one tore.
//!
//! Under `wasm32-unknown-unknown` without the `atomics` target feature -- the browser-local
//! fallback the AudioWorklet ships -- every operation below lowers to a plain load or store (the
//! script that inspected this, `check-wasm-realtime-atomics.sh`, retired with #1062's legs).
//!
//! # Read-reset without a second writer
//!
//! The reader owns one word of its own, `consumed_sequence`, and is its only writer. It is not a
//! back channel that gates publication: the writer never waits for it and never changes what it
//! publishes because of it. It exists so that a stalled reader can *see* what it missed --
//! `sequence - consumed_sequence - 1` is the exact number of windows that were overwritten -- which
//! turns "the meter froze" from an invisible failure into a counted one.

use core::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::seqlock::PublishedWords;

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

/// The words of one window in the shared record: `first_sample`, `end_sample`, `sequence`,
/// `blocks`, and the two lanes' `f32::to_bits` (left high, right low), so every published value is
/// exact and `-0.0` survives.
const WINDOW_WORDS: usize = 5;

/// The words of one slot: the window's five, then the arm state's three (`armed` as `0` or `1`,
/// `arm_sample`, `arm_tag`), all in one sequence-locked record (#1585 R1).
const SLOT_WORDS: usize = WINDOW_WORDS + 3;

/// One tap's whole published state, taken in one locked read (#1585 D2): its latest window and its
/// arm state, from the same publication.
///
/// Render is the only writer of both. Before any arm the state is unarmed, with sample `0` and tag
/// `0`. `arm_sample` is the first sample of the block that applied the tap's last arm or disarm;
/// `arm_tag` is that record's tag, `0` for none.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObservationState {
    /// The newest window, or `None` before the first.
    pub window: Option<ObservationWindow>,
    /// Whether the tap is armed.
    pub armed: bool,
    /// The first sample of the block that applied the last arm or disarm (`0` before any).
    pub arm_sample: u64,
    /// The tag of the record that armed or disarmed the tap (`0` for none).
    pub arm_tag: u64,
}

impl ObservationWindow {
    const fn to_words(self) -> [u64; WINDOW_WORDS] {
        [
            self.first_sample,
            self.end_sample,
            self.sequence,
            self.blocks as u64,
            ((self.left.to_bits() as u64) << 32) | self.right.to_bits() as u64,
        ]
    }

    #[allow(clippy::cast_possible_truncation)] // `blocks` and each lane were published as 32 bits.
    const fn from_words(words: [u64; WINDOW_WORDS]) -> Self {
        let [first_sample, end_sample, sequence, blocks, lanes] = words;
        Self {
            first_sample,
            end_sample,
            sequence,
            blocks: blocks as u32,
            left: f32::from_bits((lanes >> 32) as u32),
            right: f32::from_bits(lanes as u32),
        }
    }
}

/// The shared conflating cell. Not constructed directly: [`observation_slot`] makes the pair.
#[derive(Debug)]
pub struct ObservationSlot {
    /// The published window, in `ObservationWindow::to_words` order, then the arm state.
    words: PublishedWords<SLOT_WORDS>,
    /// The reader's own single-writer word.
    consumed: AtomicU64,
}

/// Exact engine-owned retained bytes of one observation slot: its whole shared allocation.
///
/// The cell is one `Arc` allocation shared by the two endpoints, charged as every shared lane is
/// (`SharedAllocation`: the two reference counts, then the slot, #1584). The endpoints themselves
/// are one pointer each and are counted by whatever owns them. The slot is 80 bytes since #1585
/// (the seqlock's `u64` counter, five window words, three arm words and the reader's `consumed`
/// word; 56 before), so the allocation is 96; before #1584 the charge omitted the 16-byte header.
#[must_use]
pub const fn observation_slot_retained_bytes() -> usize {
    core::alloc::Layout::new::<super::SharedAllocation<ObservationSlot>>().size()
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

/// A read-only endpoint of a reader's slot (#1332 S6b, root ruling R-S6b): it reads the cell as
/// its [`ObservationReader`] does and has no `acknowledge`, so the cell keeps exactly one writer of
/// its `consumed` word, the reader. A host whose acknowledging reader lives on the render thread
/// gives its control side one of these, made from that reader by [`ObservationReader::view`]: a
/// reference-count increment, no allocation. It is one pointer.
#[derive(Debug)]
pub struct ObservationView {
    slot: Arc<ObservationSlot>,
}

/// Build one conflating slot and its two endpoints. The only allocation this module makes.
#[must_use]
pub fn observation_slot() -> (ObservationPublisher, ObservationReader) {
    let slot = Arc::new(ObservationSlot {
        words: PublishedWords::new([0; SLOT_WORDS]),
        consumed: AtomicU64::new(0),
    });
    let reader = Arc::clone(&slot);
    (
        ObservationPublisher { slot },
        ObservationReader { slot: reader },
    )
}

// REALTIME_POLICY_BEGIN
impl ObservationPublisher {
    /// Overwrite the cell's window with `window`, keeping the arm state. Wait-free: the writer's
    /// own eight loads, eight word stores and two counter stores.
    ///
    /// Latest wins, always. There is no full, no backpressure and no return value, because there
    /// is no outcome a render thread could act on.
    pub fn publish(&self, window: ObservationWindow) {
        let [.., armed, arm_sample, arm_tag] = self.slot.words.writer_words();
        let [first_sample, end_sample, sequence, blocks, lanes] = window.to_words();
        self.slot.words.publish(&[
            first_sample,
            end_sample,
            sequence,
            blocks,
            lanes,
            armed,
            arm_sample,
            arm_tag,
        ]);
    }

    /// Overwrite the cell's arm state, keeping the window (#1585 D2): one publication of the whole
    /// record, so no reader pairs a window with another publication's arm state. Wait-free, as
    /// [`Self::publish`].
    pub fn publish_arm(&self, armed: bool, arm_sample: u64, arm_tag: u64) {
        let [first_sample, end_sample, sequence, blocks, lanes, ..] =
            self.slot.words.writer_words();
        self.slot.words.publish(&[
            first_sample,
            end_sample,
            sequence,
            blocks,
            lanes,
            u64::from(armed),
            arm_sample,
            arm_tag,
        ]);
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
        self.read_state()?.window
    }

    /// The tap's newest window and arm state, from one publication, or `None` if the read tore
    /// (#1585 D2). Bounded as [`Self::read`] is.
    #[must_use]
    pub fn read_state(&self) -> Option<ObservationState> {
        read_slot_state(&self.slot)
    }

    /// A read-only endpoint of this reader's slot ([`ObservationView`]). Allocates nothing.
    #[must_use]
    pub fn view(&self) -> ObservationView {
        ObservationView {
            slot: Arc::clone(&self.slot),
        }
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

impl ObservationView {
    /// [`ObservationReader::read`], through this endpoint.
    #[must_use]
    pub fn read(&self) -> Option<ObservationWindow> {
        self.read_state()?.window
    }

    /// [`ObservationReader::read_state`], through this endpoint. Never acknowledges.
    #[must_use]
    pub fn read_state(&self) -> Option<ObservationState> {
        read_slot_state(&self.slot)
    }
}

/// One bounded read of a slot's record: the window and arm state of one publication, or `None`
/// if every attempt tore. Shared by both read endpoints; writes nothing.
fn read_slot_state(slot: &ObservationSlot) -> Option<ObservationState> {
    let [
        first_sample,
        end_sample,
        sequence,
        blocks,
        lanes,
        armed,
        arm_sample,
        arm_tag,
    ] = slot.words.read().ok()?;
    let window = ObservationWindow::from_words([first_sample, end_sample, sequence, blocks, lanes]);
    Some(ObservationState {
        window: (window.sequence != 0).then_some(window),
        armed: armed != 0,
        arm_sample,
        arm_tag,
    })
}
