//! Bounded single-producer/single-consumer rings, and the two-cell plan mailbox.
//!
//! Queue storage is shared by two non-cloneable endpoints. Reference-count operations occur only
//! when endpoints are created, moved, or destroyed; `try_push` and `try_pop` touch only the fixed
//! slots and cursor atomics. The plan mailbox ([`plan_mailbox`], #1343) lives here too, because
//! this file is the realtime root's approved owner of `unsafe` slot access.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell, marker::PhantomData, mem::MaybeUninit, num::NonZeroUsize};
use sync::{Arc, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering, UnsafeCell};

/// The concurrency primitives the ring is built from.
///
/// Under `--cfg loom` the real ring and the real plan mailbox are instantiated on loom's `Arc`,
/// `AtomicUsize`, `AtomicU64` and `UnsafeCell`, so `spsc_loom` explores *this* code rather than a hand-written model of it
/// (#84 phase B). `cfg(loom)` is only a supported configuration for `cargo test`; a non-test
/// loom build is not a shape this crate promises to link.
#[cfg(not(loom))]
mod sync {
    pub(super) use core::cell::UnsafeCell;
    pub(super) use core::sync::atomic::{AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
    pub(super) use std::sync::Arc;

    /// Exclusive access to a slot's storage, spelled the way loom's `UnsafeCell` spells it.
    #[inline(always)]
    pub(super) fn with_mut<T, R>(cell: &UnsafeCell<T>, body: impl FnOnce(*mut T) -> R) -> R {
        body(cell.get())
    }

    /// Read a cursor with exclusive access, without an atomic operation.
    #[inline(always)]
    pub(super) fn load_exclusive(cursor: &mut AtomicUsize) -> usize {
        *cursor.get_mut()
    }
}
#[cfg(loom)]
mod sync {
    pub(super) use loom::cell::UnsafeCell;
    pub(super) use loom::sync::Arc;
    pub(super) use loom::sync::atomic::{AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};

    pub(super) fn with_mut<T, R>(cell: &UnsafeCell<T>, body: impl FnOnce(*mut T) -> R) -> R {
        cell.with_mut(body)
    }

    /// Loom's `AtomicUsize` has no `get_mut`; `unsync_load` is its exclusive-access read.
    pub(super) fn load_exclusive(cursor: &mut AtomicUsize) -> usize {
        // SAFETY: `&mut` proves no other thread can be touching this cursor.
        unsafe { cursor.unsync_load() }
    }
}

/// Wrap a ring cursor by comparison instead of by `%`.
///
/// `slot_count` is `capacity + 1` and is never a power of two, so the remainder operator compiles
/// to an integer division (20-40 cycles) on the hottest line of the queue. The compare is taken
/// once per `slot_count` operations and is perfectly predicted in between (#84 F6).
#[inline(always)]
const fn wrap_increment(cursor: usize, slot_count: usize) -> usize {
    let next = cursor + 1;
    if next == slot_count { 0 } else { next }
}

/// One cache line to itself, so a `Release` store by one endpoint cannot invalidate the line the
/// other endpoint reads on its fast path (#84 F6).
#[repr(align(64))]
struct CachePadded<T>(T);

/// Immutable queue generation selected by the owning control plane.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct QueueGeneration(pub u64);
/// Queue creation failed before allocating storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpscError {
    /// `capacity + 1` cannot be represented by `usize`.
    CapacityOverflow,
}
/// Exact engine-owned retained payload layouts for one bounded SPSC queue.
///
/// This deliberately excludes allocator headers and page rounding. The ring header and slot
/// backing allocation are the two payload allocations retained by the queue itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpscRetainedPayload {
    /// Requested logical capacity plus the one sentinel slot.
    pub slot_count: usize,
    /// Engine-owned ring header layout size.
    pub ring_header_bytes: usize,
    /// Engine-owned ring header layout alignment.
    pub ring_header_align: usize,
    /// Engine-owned backing slot layout size including element alignment padding.
    pub slot_payload_bytes: usize,
    /// Engine-owned backing slot layout alignment.
    pub slot_payload_align: usize,
}

impl SpscRetainedPayload {
    /// Total retained queue payload bytes.
    #[must_use]
    pub const fn total_bytes(self) -> Option<usize> {
        self.ring_header_bytes.checked_add(self.slot_payload_bytes)
    }

    /// Largest requested engine-owned allocation for this queue.
    #[must_use]
    pub const fn largest_allocation_bytes(self) -> usize {
        if self.ring_header_bytes > self.slot_payload_bytes {
            self.ring_header_bytes
        } else {
            self.slot_payload_bytes
        }
    }
}
/// A full result preserving the item for caller-owned retry/defer policy.
#[must_use]
#[derive(Debug)]
pub struct QueueFull<T> {
    /// Original value, still owned by the caller.
    pub value: T,
    /// Immutable identity of this queue lifetime.
    pub generation: QueueGeneration,
    /// Producer-local saturating full/overflow count.
    pub full_count: u64,
}
/// An empty result carrying its owner-local saturating counter.
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueEmpty {
    /// Immutable identity of this queue lifetime.
    pub generation: QueueGeneration,
    /// Consumer-local saturating empty/underrun count.
    pub empty_count: u64,
}

struct Ring<T> {
    // Read-mostly header. Both endpoints read these on every operation and neither ever writes
    // them, so they share one line without generating coherence traffic.
    slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
    slots_len: usize,
    logical_capacity: usize,
    generation: QueueGeneration,
    // One cache line per cursor, after the header, so `SharedRingAllocation` keeps mirroring
    // `ArcInner`'s `{ strong, weak, data }` order (#84 phase B).
    producer: CachePadded<AtomicUsize>,
    consumer: CachePadded<AtomicUsize>,
}

// `bounded_spsc_internal` retains this ring behind an `Arc`.  The resource helper exposes the
// complete engine-owned allocation layout, including the two atomic reference counts rather than
// pretending that the shared-owner header is allocator-private metadata. This mirrors the
// standard library's `ArcInner<T>` payload boundary: strong count, weak count, then `T`.
#[repr(C)]
struct SharedRingAllocation<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    ring: Ring<T>,
}

/// Compute the exact retained engine-owned queue layouts used by [`bounded_spsc`].
pub fn bounded_spsc_retained_payload<T>(
    capacity: NonZeroUsize,
) -> Result<SpscRetainedPayload, SpscError> {
    let slot_count = capacity
        .get()
        .checked_add(1)
        .ok_or(SpscError::CapacityOverflow)?;
    let slot_payload = Layout::array::<UnsafeCell<MaybeUninit<T>>>(slot_count)
        .map_err(|_| SpscError::CapacityOverflow)?;
    let ring_header = Layout::new::<SharedRingAllocation<T>>();
    Ok(SpscRetainedPayload {
        slot_count,
        ring_header_bytes: ring_header.size(),
        ring_header_align: ring_header.align(),
        slot_payload_bytes: slot_payload.size(),
        slot_payload_align: slot_payload.align(),
    })
}
// SAFETY: the only shared operations are atomics and slot access governed by SPSC cursor order.
unsafe impl<T: Send> Sync for Ring<T> {}

impl<T> Drop for Ring<T> {
    fn drop(&mut self) {
        let mut cursor = sync::load_exclusive(&mut self.consumer.0);
        let producer = sync::load_exclusive(&mut self.producer.0);
        let slots_len = self.slots_len;
        while cursor != producer {
            // SAFETY: endpoint destruction is complete before the final `Arc<Ring<T>>` drop, and
            // every cursor position in the half-open consumer..producer range is initialized
            // exactly once. No producer or consumer can access a slot concurrently here.
            sync::with_mut(&self.slots[cursor], |slot| unsafe {
                (*slot).assume_init_drop();
            });
            cursor = wrap_increment(cursor, slots_len);
        }
    }
}

/// Native producer endpoint. `Cell` intentionally makes it `!Sync`; ownership transfer is `Send`.
pub struct Producer<T: Send + 'static> {
    ring: Arc<Ring<T>>,
    local: usize,
    /// Last consumer cursor this producer observed.
    ///
    /// A peer cursor only advances and never passes its partner, so the true consumer cursor
    /// always lies on the ring arc `[cached_consumer, local]`. "Full" means `next == consumer`
    /// with `next = local + 1`, and `next` is on that arc only when `cached_consumer == next`.
    /// So "not full by the cached value" implies "not full by the true value", and the shared
    /// line is read only when the queue *looks* full (#84 F6). Visibility is unchanged: the
    /// `Acquire` reload below is the same load the old code did unconditionally.
    cached_consumer: usize,
    successes: u64,
    full: u64,
    _not_sync: PhantomData<Cell<()>>,
}
/// Native consumer endpoint and sole allocation owner. It is `!Sync` and must outlive producer.
pub struct Consumer<T: Send + 'static> {
    ring: Arc<Ring<T>>,
    local: usize,
    /// Last producer cursor this consumer observed; the mirror of
    /// [`Producer::cached_consumer`]. The true producer cursor lies on `[cached_producer, local)`,
    /// which excludes `local` unless `cached_producer == local`, so "not empty by the cached
    /// value" implies "not empty by the true value". Every slot in `[local, cached_producer)` was
    /// published by a `Release` store that happened-before the `Acquire` load that produced
    /// `cached_producer`, so the cache never weakens publication.
    cached_producer: usize,
    successes: u64,
    empty: u64,
    _not_sync: PhantomData<Cell<()>>,
}
/// Internal one-slot reservation. It prevents a plan swap until retirement storage is guaranteed.
pub(crate) struct PushPermit<'a, T: Send + 'static> {
    producer: &'a mut Producer<T>,
    next: usize,
}
/// Create a native queue with exact logical capacity and `capacity + 1` physical slots.
pub fn bounded_spsc<T: Copy + Send + 'static>(
    capacity: NonZeroUsize,
    generation: QueueGeneration,
) -> Result<(Producer<T>, Consumer<T>), SpscError> {
    bounded_spsc_internal(capacity, generation)
}
/// Create a native bounded SPSC queue that transfers move-only values.
///
/// The queue has the same acquire/release publication protocol and endpoint ownership invariant
/// as [`bounded_spsc`].  Unlike that convenience constructor, its values need not be `Copy`:
/// a full push returns the original value to its sole producer, and a successful pop transfers
/// the initialized slot to its sole consumer exactly once.  This is the only public move-only
/// extension of the core SPSC boundary; scheduler workers use it for prepared parcels.
pub fn bounded_spsc_move<T: Send + 'static>(
    capacity: NonZeroUsize,
    generation: QueueGeneration,
) -> Result<(Producer<T>, Consumer<T>), SpscError> {
    bounded_spsc_internal(capacity, generation)
}
/// Internal move-only variant for plan ownership; it is not public API.
pub(crate) fn bounded_spsc_internal<T: Send + 'static>(
    capacity: NonZeroUsize,
    generation: QueueGeneration,
) -> Result<(Producer<T>, Consumer<T>), SpscError> {
    let slots_len = capacity
        .get()
        .checked_add(1)
        .ok_or(SpscError::CapacityOverflow)?;
    let mut slots = Vec::with_capacity(slots_len);
    slots.resize_with(slots_len, || UnsafeCell::new(MaybeUninit::uninit()));
    #[allow(clippy::redundant_closure_for_method_calls)]
    let ring = Arc::new(Ring {
        slots: slots.into_boxed_slice(),
        slots_len,
        logical_capacity: capacity.get(),
        generation,
        producer: CachePadded(AtomicUsize::new(0)),
        consumer: CachePadded(AtomicUsize::new(0)),
    });
    let producer = Producer {
        ring: Arc::clone(&ring),
        local: 0,
        cached_consumer: 0,
        successes: 0,
        full: 0,
        _not_sync: PhantomData,
    };
    Ok((
        producer,
        Consumer {
            ring,
            local: 0,
            cached_producer: 0,
            successes: 0,
            empty: 0,
            _not_sync: PhantomData,
        },
    ))
}
// REALTIME_POLICY_BEGIN
impl<T: Send + 'static> Producer<T> {
    fn ring(&self) -> &Ring<T> {
        &self.ring
    }
    /// Exact usable queue capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.ring().logical_capacity
    }
    /// Immutable queue generation.
    #[must_use]
    pub fn generation(&self) -> QueueGeneration {
        self.ring().generation
    }
    /// Producer-local saturating successful-push count.
    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.successes
    }
    /// Producer-local saturating full/overflow count.
    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.full
    }
    /// Snapshots the number of queue slots that can accept a complete producer prefix.
    ///
    /// The consumer cursor is loaded exactly once with `Acquire`; this observation does not
    /// mutate the producer cache, cursors, counters, or queue contents. Since the consumer is the
    /// sole owner that advances its cursor, a later consumer pop can only increase the returned
    /// capacity. Callers that need all-or-nothing publication can therefore check a complete
    /// prefix before issuing its individual pushes.
    #[must_use]
    pub fn available_capacity(&self) -> usize {
        let consumer = self.ring.consumer.0.load(Ordering::Acquire);
        let producer = self.local;
        let occupied = if producer >= consumer {
            producer - consumer
        } else {
            self.ring.slots_len - consumer + producer
        };
        self.ring.logical_capacity - occupied
    }
    /// Whether `next` is the consumer's cursor, reloading the shared line only if it looks so.
    ///
    /// See [`Producer::cached_consumer`] for why one reload settles it.
    fn is_full_at(&mut self, next: usize) -> bool {
        if next != self.cached_consumer {
            return false;
        }
        self.cached_consumer = self.ring.consumer.0.load(Ordering::Acquire);
        next == self.cached_consumer
    }
    /// Try one bounded push; it never retries or blocks.
    pub fn try_push(&mut self, value: T) -> Result<(), QueueFull<T>> {
        match self.try_reserve() {
            Some(permit) => {
                permit.commit(value);
                Ok(())
            }
            None => Err(QueueFull {
                value,
                generation: self.ring.generation,
                full_count: self.full,
            }),
        }
    }
    /// Reserve one slot without publishing it. Only realtime exchange uses this transactionally.
    pub(crate) fn try_reserve(&mut self) -> Option<PushPermit<'_, T>> {
        let next = wrap_increment(self.local, self.ring.slots_len);
        if self.is_full_at(next) {
            self.full = self.full.saturating_add(1);
            None
        } else {
            Some(PushPermit {
                producer: self,
                next,
            })
        }
    }
}
impl<T: Send + 'static> PushPermit<'_, T> {
    /// Write and publish the reserved item exactly once.
    pub(crate) fn commit(self, value: T) {
        let local = self.producer.local;
        // SAFETY: `try_reserve` acquired free capacity and only the unique producer owns this slot.
        sync::with_mut(&self.producer.ring.slots[local], |slot| unsafe {
            (*slot).write(value);
        });
        self.producer
            .ring
            .producer
            .0
            .store(self.next, Ordering::Release);
        self.producer.local = self.next;
        self.producer.successes = self.producer.successes.saturating_add(1);
    }
}
impl<T: Send + 'static> Consumer<T> {
    fn ring(&self) -> &Ring<T> {
        &self.ring
    }
    /// Exact usable queue capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.ring().logical_capacity
    }
    /// Consumer-local saturating successful-pop count.
    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.successes
    }
    /// Consumer-local saturating empty/underrun count.
    #[must_use]
    pub const fn empty_count(&self) -> u64 {
        self.empty
    }
    /// Snapshots the number of records available at this drain entry.
    ///
    /// The producer cursor is loaded once with `Acquire`, then compared with this consumer's
    /// local cursor using the ring's modular cursor space. The returned count is bounded by the
    /// logical capacity under the SPSC ownership invariant. This method does not mutate cursors,
    /// caches, or counters, so later producer publication cannot enlarge the already-returned
    /// count; callers can use it to freeze a bounded drain before popping.
    #[must_use]
    pub fn available_at_entry(&self) -> usize {
        let producer = self.ring().producer.0.load(Ordering::Acquire);
        let consumer = self.local;
        if producer >= consumer {
            producer - consumer
        } else {
            self.ring().slots_len - consumer + producer
        }
    }
    /// Whether the queue is empty, reloading the shared line only if it looks so.
    ///
    /// See [`Consumer::cached_producer`] for why one reload settles it.
    fn is_drained(&mut self) -> bool {
        if self.local != self.cached_producer {
            return false;
        }
        self.cached_producer = self.ring.producer.0.load(Ordering::Acquire);
        self.local == self.cached_producer
    }
    /// Try one bounded pop; it never retries or blocks.
    pub fn try_pop(&mut self) -> Result<T, QueueEmpty> {
        if self.is_drained() {
            self.empty = self.empty.saturating_add(1);
            return Err(QueueEmpty {
                generation: self.ring.generation,
                empty_count: self.empty,
            });
        }
        // SAFETY: producer release + this acquire publishes initialization; only consumer reads it.
        let value = sync::with_mut(&self.ring.slots[self.local], |slot| unsafe {
            (*slot).assume_init_read()
        });
        let next = wrap_increment(self.local, self.ring.slots_len);
        self.ring.consumer.0.store(next, Ordering::Release);
        self.local = next;
        self.successes = self.successes.saturating_add(1);
        Ok(value)
    }
}
// REALTIME_POLICY_END

// ---------------------------------------------------------------------------------------------
// The two-cell plan mailbox (#1343, decision 15 D15-9).
//
// One `AtomicU64` word holds a 60-bit generation counter (bits 4..64) and, per cell, a 2-bit
// state (cell 0 in bits 0..2, cell 1 in bits 2..4): `Empty`, `Full` or `Active`.
//
// Invariants, under every interleaving:
// - I1. Exactly one cell is `Active`. It stands for the value the reader runs; the reader moved
//   its payload out when it claimed it, so an `Active` cell's payload is `None`. Cell 0 starts
//   `Active` for the value the reader is created with.
// - I2. At most one cell is `Full`, so the other cell is `Empty` or `Active`.
// - I3. Only the writer changes a cell `Empty -> Full` (publish) and `Full -> Empty` (withdraw).
//   Only the reader changes the word otherwise: one compare-and-swap that makes the `Full` cell
//   `Active` and the previously `Active` cell `Empty` (claim). Every successful transition adds
//   one to the generation, so a compare-and-swap with a stale word always fails.
// - I4. Payload ownership follows the state: the writer owns an `Empty` cell's payload, nobody
//   touches a `Full` cell's payload until a compare-and-swap moves it out of `Full` (the winner
//   then owns it), and the reader owns the `Active` cell's payload.
// - I5. The reader changes the word only when a cell is `Full`, and the writer publishes only when
//   none is. So the writer's publication compare-and-swap cannot fail (a failure is a broken
//   invariant, returned as [`MailboxInvariantBroken`] and never retried), and a failed withdrawal
//   compare-and-swap means the reader claimed the cell, which it does at most once per
//   publication.
// - I6. The reader makes at most one compare-and-swap per claim and never retries: a lost claim
//   means the writer withdrew the candidate after the reader's load.
// - I7 (#1314 D1). Each cell has a revision word beside its payload: the newest committed revision
//   whose content that cell's value carries. Only the writer stores it: into the `Empty` cell
//   before the publication that marks it `Full`, into the `Full` cell while it is published, and
//   into the `Active` cell only while no cell is `Full`. A concurrent claim does not change which
//   cell is right: the `Full` cell becomes the `Active` one. The reader loads only the `Active`
//   cell's word, whose index it learned from its own last claim. A withdrawal takes the cell's
//   word with its payload, so a cell the writer emptied never reports a revision.
// - I8 (#1311 D1, D2). Each cell has an adoption schedule beside its payload: a kind, a due sample
//   and a prime lead, three atomics outside the payload. Only the writer stores them, into the
//   `Empty` cell before the `Release` that marks it `Full`. The reader loads a `Full` cell's
//   schedule after its `Acquire` load of the word and before it claims, and claims only if the
//   schedule admits the block ([`PlanAdoption::admits`]). A schedule load that belongs to another
//   publication (the writer withdrew and republished in between) goes with a word of an older
//   generation, so the claim fails (I3, I6): a stale schedule or a stale readiness answer can
//   only make render skip a block, never claim a candidate that is not due or not ready.
// ---------------------------------------------------------------------------------------------

const CELL_EMPTY: u64 = 0;
const CELL_FULL: u64 = 1;
const CELL_ACTIVE: u64 = 2;
const CELL_STATE_MASK: u64 = 0b11;
const CELL_STATE_BITS: u32 = 2;
const MAILBOX_GENERATION_ONE: u64 = 1 << (2 * CELL_STATE_BITS);
/// Cell 0 `Active`, cell 1 `Empty`, generation 0.
const MAILBOX_INITIAL_WORD: u64 = CELL_ACTIVE;

const ADOPTION_NEXT: u8 = 0;
const ADOPTION_NO_EARLIER_THAN: u8 = 1;
const ADOPTION_PRIMED: u8 = 2;

/// When render may adopt a published candidate (#1311 D1), fixed when the control thread
/// reserves it and kept across withdrawal and republication.
///
/// **The claim rule (D2, D5).** At each block entry render loads the mailbox word, then the
/// `Full` cell's schedule, and claims the candidate only if the schedule admits the block that is
/// about to render (`PlanAdoption::admits`); a claimed candidate is adopted in that same block, before it renders (D4).
/// A candidate that is not admitted stays `Full` and withdrawable; render looks again at the next
/// block and never waits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanAdoption {
    /// Adopt at the first block after publication.
    Next,
    /// Adopt at the first block whose start is at or past this absolute sample. Any sample is
    /// accepted; one between block starts adopts at the next block start after it.
    NoEarlierThan(u64),
    /// Adopt at the first block whose start is at or past `not_before` and at which the running
    /// plan reports itself ready for a prime of `lead_blocks` blocks
    /// ([`super::PreparedPlanExecutor::prime_ready`]); a plan that cannot answer never adopts it.
    Primed {
        /// The earliest block start that may adopt the candidate.
        not_before: u64,
        /// The prime the adopting block runs, in blocks.
        lead_blocks: u32,
    },
}

// REALTIME_POLICY_BEGIN
impl PlanAdoption {
    /// Whether a block starting at `block_start` may claim a candidate with this schedule.
    ///
    /// `prime_ready` is asked only for a `Primed` candidate whose `not_before` the block has
    /// reached, with the block start and the lead; it is the running plan's readiness hook.
    #[inline]
    pub(crate) fn admits(
        self,
        block_start: u64,
        prime_ready: impl FnOnce(u64, u32) -> bool,
    ) -> bool {
        match self {
            Self::Next => true,
            Self::NoEarlierThan(due) => block_start >= due,
            Self::Primed {
                not_before,
                lead_blocks,
            } => block_start >= not_before && prime_ready(block_start, lead_blocks),
        }
    }

    const fn encode(self) -> (u8, u64, u32) {
        match self {
            Self::Next => (ADOPTION_NEXT, 0, 0),
            Self::NoEarlierThan(due) => (ADOPTION_NO_EARLIER_THAN, due, 0),
            Self::Primed {
                not_before,
                lead_blocks,
            } => (ADOPTION_PRIMED, not_before, lead_blocks),
        }
    }

    /// The writer stores only the three kinds `encode` produces.
    const fn decode(kind: u8, due: u64, lead_blocks: u32) -> Self {
        match kind {
            ADOPTION_NEXT => Self::Next,
            ADOPTION_NO_EARLIER_THAN => Self::NoEarlierThan(due),
            ADOPTION_PRIMED => Self::Primed {
                not_before: due,
                lead_blocks,
            },
            _ => {
                debug_assert!(
                    false,
                    "plan mailbox invariant broken: unknown adoption kind"
                );
                // Unreachable: only `encode` writes the kind. A schedule render cannot read never
                // admits a block, so the candidate stays published and withdrawable.
                Self::NoEarlierThan(u64::MAX)
            }
        }
    }
}
// REALTIME_POLICY_END

/// One snapshot of the mailbox state word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MailboxWord(u64);

impl MailboxWord {
    #[inline(always)]
    const fn shift(cell: usize) -> u32 {
        if cell == 0 { 0 } else { CELL_STATE_BITS }
    }

    #[inline(always)]
    const fn state(self, cell: usize) -> u64 {
        (self.0 >> Self::shift(cell)) & CELL_STATE_MASK
    }

    #[inline(always)]
    const fn with(self, cell: usize, state: u64) -> Self {
        let shift = Self::shift(cell);
        Self((self.0 & !(CELL_STATE_MASK << shift)) | (state << shift))
    }

    /// The same states one generation later; the generation wraps in its 60 bits.
    #[inline(always)]
    const fn advanced(self) -> Self {
        Self(self.0.wrapping_add(MAILBOX_GENERATION_ONE))
    }

    /// The cell in `state`, preferring cell 0; `None` if neither cell is in it.
    #[inline(always)]
    const fn cell_in(self, state: u64) -> Option<usize> {
        if self.state(0) == state {
            Some(0)
        } else if self.state(1) == state {
            Some(1)
        } else {
            None
        }
    }
}

/// The state of one mailbox cell, for tests that assert where a publication landed.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MailboxCellState {
    /// The writer owns the cell and may publish into it.
    Empty,
    /// A published candidate that the reader has not claimed and the writer has not withdrawn.
    Full,
    /// The cell standing for the value the reader runs.
    Active,
}

struct Mailbox<T> {
    word: AtomicU64,
    cells: [UnsafeCell<Option<T>>; 2],
    /// Each cell's revision word (I7), indexed as `cells`.
    revisions: [AtomicU64; 2],
    /// Each cell's adoption schedule (I8), indexed as `cells`: the kind, the due sample
    /// (`NoEarlierThan`'s sample or `Primed`'s `not_before`) and `Primed`'s lead.
    adoption_kinds: [AtomicU8; 2],
    due_samples: [AtomicU64; 2],
    lead_blocks: [AtomicU32; 2],
}

// REALTIME_POLICY_BEGIN
impl<T> Mailbox<T> {
    /// Store `cell`'s schedule (I8). `Relaxed`: the writer's next `Release` on the word publishes
    /// it.
    fn store_adoption(&self, cell: usize, adoption: PlanAdoption) {
        let (kind, due, lead) = adoption.encode();
        self.adoption_kinds[cell].store(kind, Ordering::Relaxed);
        self.due_samples[cell].store(due, Ordering::Relaxed);
        self.lead_blocks[cell].store(lead, Ordering::Relaxed);
    }

    /// Load `cell`'s schedule (I8), `Relaxed`: the caller's `Acquire` load of the word that found
    /// the cell `Full` makes that publication's stores visible, and a later store can only belong
    /// to a publication of a newer generation, which a claim built on that word cannot win.
    fn load_adoption(&self, cell: usize) -> PlanAdoption {
        PlanAdoption::decode(
            self.adoption_kinds[cell].load(Ordering::Relaxed),
            self.due_samples[cell].load(Ordering::Relaxed),
            self.lead_blocks[cell].load(Ordering::Relaxed),
        )
    }
}
// REALTIME_POLICY_END

// SAFETY: the cells are accessed only under the ownership rules I1-I6 above, which the state
// word's acquire/release transitions enforce; `T: Send` lets a payload cross threads.
unsafe impl<T: Send> Sync for Mailbox<T> {}

// The mailbox is retained behind an `Arc`, so its engine-owned allocation is `ArcInner`'s
// `{ strong, weak, data }`, mirrored here as `SharedRingAllocation` mirrors it for the ring.
#[repr(C)]
struct SharedMailboxAllocation<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    mailbox: Mailbox<T>,
}

/// Exact engine-owned bytes [`plan_mailbox`] retains: the state word and the two cells, with the
/// shared-owner header. It is one allocation.
pub(crate) fn plan_mailbox_retained_bytes<T>() -> usize {
    Layout::new::<SharedMailboxAllocation<T>>().size()
}

/// Writer endpoint: publishes into the `Empty` cell and withdraws a `Full` one. `!Sync`.
pub(crate) struct MailboxWriter<T: Send + 'static> {
    shared: Arc<Mailbox<T>>,
    /// The cell of the last publication whose fate the writer has not learned yet.
    published: Option<usize>,
    _not_sync: PhantomData<Cell<()>>,
}

/// Reader endpoint: claims a `Full` cell with one compare-and-swap. `!Sync`.
pub(crate) struct MailboxReader<T: Send + 'static> {
    shared: Arc<Mailbox<T>>,
    /// The `Active` cell: 0 at creation, then the cell of the reader's last successful claim.
    active: usize,
    /// Claim compare-and-swaps made, so the loom model can hold I6 to account.
    #[cfg(all(test, loom))]
    claim_attempts: u64,
    _not_sync: PhantomData<Cell<()>>,
}

/// The writer's hold on the `Empty` cell while no cell is `Full`. Committing it publishes.
pub(crate) struct MailboxPermit<'a, T: Send + 'static> {
    writer: &'a mut MailboxWriter<T>,
    cell: usize,
    observed: MailboxWord,
}

/// The publication compare-and-swap failed: invariant I5 is broken. The value comes back whole
/// and nothing was published. Never retried.
pub(crate) struct MailboxInvariantBroken<T>(pub(crate) T);

/// The result of [`MailboxWriter::withdraw`].
pub(crate) enum MailboxWithdrawal<T> {
    /// The cell was `Full`; the writer marked it `Empty` and moved the value out, with the cell's
    /// revision word (I7) and its adoption schedule (I8).
    Withdrawn(T, u64, PlanAdoption),
    /// The reader's claim won: the value is the reader's, and that cannot change.
    Taken,
    /// Nothing was published since the last claim the writer learned of or the last withdrawal.
    Nothing,
}

/// A reader's load that found a `Full` cell; [`MailboxReader::claim`] consumes it.
#[derive(Clone, Copy)]
pub(crate) struct MailboxObservation {
    word: MailboxWord,
    full: usize,
}

/// Create a mailbox whose cell 0 stands for the value the reader already runs.
pub(crate) fn plan_mailbox<T: Send + 'static>() -> (MailboxWriter<T>, MailboxReader<T>) {
    let shared = Arc::new(Mailbox {
        word: AtomicU64::new(MAILBOX_INITIAL_WORD),
        cells: [UnsafeCell::new(None), UnsafeCell::new(None)],
        revisions: [AtomicU64::new(0), AtomicU64::new(0)],
        adoption_kinds: [AtomicU8::new(ADOPTION_NEXT), AtomicU8::new(ADOPTION_NEXT)],
        due_samples: [AtomicU64::new(0), AtomicU64::new(0)],
        lead_blocks: [AtomicU32::new(0), AtomicU32::new(0)],
    });
    (
        MailboxWriter {
            shared: Arc::clone(&shared),
            published: None,
            _not_sync: PhantomData,
        },
        MailboxReader {
            shared,
            active: 0,
            #[cfg(all(test, loom))]
            claim_attempts: 0,
            _not_sync: PhantomData,
        },
    )
}

impl<T: Send + 'static> MailboxWriter<T> {
    /// Hold the `Empty` cell for one publication, or `None` while a cell is `Full`.
    ///
    /// The `Acquire` load synchronizes with the reader's claim that made the cell `Empty`, so the
    /// reader's last access to that cell happens before the writer's write into it.
    pub(crate) fn try_reserve(&mut self) -> Option<MailboxPermit<'_, T>> {
        let observed = MailboxWord(self.shared.word.load(Ordering::Acquire));
        if observed.cell_in(CELL_FULL).is_some() {
            return None;
        }
        let cell = observed.cell_in(CELL_EMPTY)?;
        Some(MailboxPermit {
            writer: self,
            cell,
            observed,
        })
    }

    /// Take back the last publication if the reader has not claimed it.
    ///
    /// One compare-and-swap marks the `Full` cell `Empty`. If it fails, the reader changed the
    /// word, which it does only by claiming the `Full` cell (I5); the failed compare-and-swap's
    /// current value is the one reload, and the result is reported from it. Never retried.
    pub(crate) fn withdraw(&mut self) -> MailboxWithdrawal<T> {
        let Some(cell) = self.published.take() else {
            return MailboxWithdrawal::Nothing;
        };
        let observed = MailboxWord(self.shared.word.load(Ordering::Acquire));
        if observed.state(cell) != CELL_FULL {
            return MailboxWithdrawal::Taken;
        }
        let next = observed.with(cell, CELL_EMPTY).advanced();
        match self.shared.word.compare_exchange(
            observed.0,
            next.0,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                // SAFETY: the compare-and-swap moved the cell from `Full` to `Empty`, so the writer
                // owns its payload (I4); the reader touches a cell's payload only after its own
                // claim of that cell won, and it lost this one.
                let value =
                    sync::with_mut(&self.shared.cells[cell], |slot| unsafe { (*slot).take() });
                // I7: only this writer stores a cell's word, so its own last store is the value.
                let revision = self.shared.revisions[cell].load(Ordering::Relaxed);
                // I8: likewise, only this writer stores the schedule.
                let adoption = self.shared.load_adoption(cell);
                match value {
                    Some(value) => MailboxWithdrawal::Withdrawn(value, revision, adoption),
                    // A `Full` cell always holds its payload: the writer stores it before the
                    // `Release` that marks the cell `Full`.
                    None => panic!("plan mailbox invariant broken: a Full cell held no payload"),
                }
            }
            Err(current) => {
                debug_assert_eq!(MailboxWord(current).state(cell), CELL_ACTIVE);
                MailboxWithdrawal::Taken
            }
        }
    }

    /// Store `revision` into the `Full` cell if there is one, else into the `Active` cell (I7).
    /// Returns `true` if it wrote the `Full` cell.
    ///
    /// The `Release` store publishes everything the writer wrote before it to the reader's
    /// `Acquire` load of the same word. A claim racing this store makes the `Full` cell the
    /// `Active` one, so the word lands in the right cell either way; and while no cell is `Full`
    /// the reader cannot change the word (I5), so the `Active` cell the load found stays `Active`.
    pub(crate) fn store_revision(&mut self, revision: u64) -> bool {
        let observed = MailboxWord(self.shared.word.load(Ordering::Acquire));
        let (cell, full) = match observed.cell_in(CELL_FULL) {
            Some(cell) => (cell, true),
            None => match observed.cell_in(CELL_ACTIVE) {
                Some(cell) => (cell, false),
                None => panic!("plan mailbox invariant broken: no cell is Active"),
            },
        };
        self.shared.revisions[cell].store(revision, Ordering::Release);
        full
    }

    /// The states of both cells, for tests.
    #[cfg(test)]
    pub(crate) fn cell_states(&self) -> [MailboxCellState; 2] {
        let word = MailboxWord(self.shared.word.load(Ordering::Acquire));
        [0, 1].map(|cell| match word.state(cell) {
            CELL_EMPTY => MailboxCellState::Empty,
            CELL_FULL => MailboxCellState::Full,
            _ => MailboxCellState::Active,
        })
    }
}

impl<T: Send + 'static> MailboxPermit<'_, T> {
    /// Write `value`, its revision word (I7) and its adoption schedule (I8) into the held
    /// `Empty` cell, then mark it `Full` with one `Release` compare-and-swap.
    ///
    /// The revision and the schedule are arguments, not optional prior stores, so every
    /// publication writes both: a cell never carries an earlier publication's revision or
    /// schedule into a new one. The reader loads a cell's revision only while it is `Active` and
    /// its schedule only after a load of the word that found it `Full`, and this compare-and-swap's
    /// `Release` publishes the stores with the payload.
    ///
    /// No cell was `Full` at reservation and only this writer publishes, so the reader cannot
    /// have changed the word since (I5) and the compare-and-swap cannot fail. A failure is a
    /// broken invariant: the value is taken back and returned, never retried.
    pub(crate) fn commit(
        self,
        value: T,
        revision: u64,
        adoption: PlanAdoption,
    ) -> Result<(), MailboxInvariantBroken<T>> {
        let shared = &*self.writer.shared;
        shared.revisions[self.cell].store(revision, Ordering::Relaxed);
        shared.store_adoption(self.cell, adoption);
        // SAFETY: the cell is `Empty`, so the writer owns its payload (I4); the reader touches a
        // cell's payload only after its claim of a `Full` cell, and this cell is not `Full`.
        let previous = sync::with_mut(&shared.cells[self.cell], |slot| unsafe {
            (*slot).replace(value)
        });
        debug_assert!(previous.is_none(), "an Empty cell holds no payload");
        drop(previous);
        let next = self.observed.with(self.cell, CELL_FULL).advanced();
        match shared.word.compare_exchange(
            self.observed.0,
            next.0,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                self.writer.published = Some(self.cell);
                Ok(())
            }
            Err(_) => {
                // SAFETY: the cell was never marked `Full`, so no reader can own it; the writer
                // still does (I4).
                let value =
                    sync::with_mut(&shared.cells[self.cell], |slot| unsafe { (*slot).take() });
                match value {
                    Some(value) => Err(MailboxInvariantBroken(value)),
                    None => {
                        panic!("plan mailbox invariant broken: a written cell lost its payload")
                    }
                }
            }
        }
    }
}

// REALTIME_POLICY_BEGIN
impl<T: Send + 'static> MailboxReader<T> {
    /// Claim compare-and-swaps this reader has made.
    #[cfg(all(test, loom))]
    #[must_use]
    pub(crate) const fn claim_attempts(&self) -> u64 {
        self.claim_attempts
    }

    /// One `Acquire` load of the `Active` cell's revision word (I7). It synchronizes with the
    /// writer's `Release` store of that word, so everything the writer wrote before the store is
    /// visible after this load.
    #[inline]
    pub(crate) fn active_revision(&self) -> u64 {
        self.shared.revisions[self.active].load(Ordering::Acquire)
    }

    /// One `Acquire` load: the `Full` cell, if any. It synchronizes with the writer's `Release`
    /// that marked the cell `Full`.
    #[inline]
    pub(crate) fn observe(&self) -> Option<MailboxObservation> {
        let word = MailboxWord(self.shared.word.load(Ordering::Acquire));
        let full = word.cell_in(CELL_FULL)?;
        Some(MailboxObservation { word, full })
    }

    /// The observed `Full` cell's adoption schedule (I8): three `Relaxed` loads, after the
    /// observation's `Acquire` and before any claim. It may belong to a later publication than the
    /// observation's if the writer withdrew and republished in between; the claim built on the
    /// observation then fails on the generation, so the reader acts on it only through a claim.
    #[inline]
    pub(crate) fn scheduled(&self, observed: MailboxObservation) -> PlanAdoption {
        self.shared.load_adoption(observed.full)
    }

    /// Claim the observed `Full` cell with exactly one compare-and-swap of the whole word: the
    /// `Full` cell becomes `Active` and the previously `Active` cell `Empty`. That transition is
    /// the adoption decision. On success the value is moved out of its cell; on failure (the
    /// writer withdrew it after the observation) this returns `None` and does nothing else. It
    /// never retries or spins.
    #[inline]
    pub(crate) fn claim(&mut self, observed: MailboxObservation) -> Option<T> {
        #[cfg(all(test, loom))]
        {
            self.claim_attempts += 1;
        }
        // I1 and I2: with one cell `Full`, the other is the `Active` one.
        let active = 1 - observed.full;
        let next = observed
            .word
            .with(observed.full, CELL_ACTIVE)
            .with(active, CELL_EMPTY)
            .advanced();
        // `Acquire` sees the writer's payload; `Release` hands the old `Active` cell, whose payload
        // this reader moved out at its own claim, to the writer.
        if self
            .shared
            .word
            .compare_exchange(observed.word.0, next.0, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return None;
        }
        self.active = observed.full;
        // SAFETY: the compare-and-swap made this cell `Active`, so the reader owns its payload
        // (I4); the writer never touches an `Active` cell.
        sync::with_mut(&self.shared.cells[observed.full], |slot| unsafe {
            (*slot).take()
        })
    }
}
// REALTIME_POLICY_END

#[cfg(all(test, loom))]
mod loom_tests {
    use super::{
        MailboxReader, MailboxWithdrawal, MailboxWriter, PlanAdoption, QueueGeneration,
        bounded_spsc, plan_mailbox,
    };
    use core::num::NonZeroUsize;
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    /// #84 phase B: the model is the **real** ring, instantiated on loom's atomics and cells by
    /// the `sync` shim at the top of this module. Three items through a two-slot queue force one
    /// wrap, one cached-full reload and one cached-empty reload under every interleaving loom
    /// explores, which is exactly what the cursor caches of `try_push`/`try_pop` have to survive.
    #[test]
    fn spsc_loom_real_ring_fifo_with_cached_cursors() {
        loom::model(|| {
            let (mut producer, mut consumer) =
                bounded_spsc::<usize>(NonZeroUsize::new(2).expect("two"), QueueGeneration(1))
                    .expect("ring");
            let writer = loom::thread::spawn(move || {
                for value in 0..3 {
                    while producer.try_push(value).is_err() {
                        loom::thread::yield_now();
                    }
                }
            });
            let reader = loom::thread::spawn(move || {
                for expected in 0..3 {
                    loop {
                        match consumer.try_pop() {
                            Ok(got) => {
                                assert_eq!(got, expected);
                                break;
                            }
                            Err(_) => loom::thread::yield_now(),
                        }
                    }
                }
            });
            writer.join().expect("producer");
            reader.join().expect("consumer");
        });
    }

    /// A mailbox payload that counts its drops, so every model can prove that each published
    /// value ends exactly once: adopted by render, held by control, or dropped with the mailbox.
    /// Its drop stands for the return of the candidate's retirement credit.
    struct Candidate {
        id: usize,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for Candidate {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn candidate(id: usize, drops: &Arc<AtomicUsize>) -> Candidate {
        Candidate {
            id,
            drops: Arc::clone(drops),
        }
    }

    fn publish(writer: &mut MailboxWriter<Candidate>, value: Candidate) {
        let permit = writer
            .try_reserve()
            .expect("an Empty cell while none is Full");
        assert!(
            permit.commit(value, 0, PlanAdoption::Next).is_ok(),
            "publication cannot fail (I5)"
        );
    }

    /// Render's blocks: one load per block and, only if a cell is `Full`, one claim. Returns the
    /// adopted IDs in adoption order. Each block makes at most one compare-and-swap (I6).
    fn render_blocks(reader: &mut MailboxReader<Candidate>, blocks: usize) -> Vec<usize> {
        let mut adopted = Vec::new();
        for _ in 0..blocks {
            let before = reader.claim_attempts();
            if let Some(observed) = reader.observe() {
                if let Some(value) = reader.claim(observed) {
                    adopted.push(value.id);
                }
            }
            assert!(reader.claim_attempts() - before <= 1, "one CAS per block");
        }
        adopted
    }

    /// #1343 gate 2: render claims while control withdraws the same candidate. Exactly one side
    /// wins: render adopts it and control reads `Taken`, or control holds it whole and render
    /// adopted nothing. Loom's cells fail the model if render reads the cell control writes.
    /// #1314 D3 (verifier F4): after its block render reads the revision of the cell it now
    /// renders from, the candidate's 7 if it won the claim and the initial 1 if it lost it.
    #[test]
    fn spsc_loom_plan_mailbox_claim_races_withdrawal() {
        loom::model(|| {
            let drops = Arc::new(AtomicUsize::new(0));
            let (mut writer, mut reader) = plan_mailbox::<Candidate>();
            assert!(
                !writer.store_revision(1),
                "the initial plan's cell is Active"
            );
            let render = loom::thread::spawn(move || {
                let adopted = render_blocks(&mut reader, 1);
                let revision = reader.active_revision();
                (adopted, revision, reader)
            });
            let permit = writer.try_reserve().expect("an Empty cell");
            assert!(
                permit
                    .commit(candidate(1, &drops), 7, PlanAdoption::Next)
                    .is_ok()
            );
            let outcome = writer.withdraw();
            let (adopted, revision, reader) = render.join().expect("render");
            match outcome {
                MailboxWithdrawal::Withdrawn(value, _, _) => {
                    assert_eq!(value.id, 1);
                    assert!(adopted.is_empty(), "adopted and withdrawn both");
                    // #1314 D3: a lost claim leaves render on the initial plan's cell.
                    assert_eq!(revision, 1, "a lost claim moved the Active cell");
                }
                MailboxWithdrawal::Taken => {
                    assert_eq!(adopted, [1]);
                    assert_eq!(revision, 7);
                }
                MailboxWithdrawal::Nothing => panic!("a publication is never Nothing"),
            }
            drop((writer, reader));
            assert_eq!(
                drops.load(Ordering::Relaxed),
                1,
                "each value ends exactly once"
            );
        });
    }

    /// #1343 gate 2: control publishes A, withdraws it, republishes it if it came back, then
    /// tries B; render runs two blocks. A is adopted at most once and never both adopted and
    /// withdrawn-and-kept, B only after A, B lands only while no cell is `Full`, and every value
    /// ends exactly once.
    #[test]
    fn spsc_loom_plan_mailbox_withdraw_republish_and_next_publication() {
        loom::model(|| {
            let drops = Arc::new(AtomicUsize::new(0));
            let (mut writer, mut reader) = plan_mailbox::<Candidate>();
            let render = loom::thread::spawn(move || {
                let adopted = render_blocks(&mut reader, 2);
                (adopted, reader)
            });
            publish(&mut writer, candidate(1, &drops));
            let first = writer.withdraw();
            let withdrawn_first = matches!(first, MailboxWithdrawal::Withdrawn(..));
            if let MailboxWithdrawal::Withdrawn(value, _, _) = first {
                assert_eq!(value.id, 1);
                publish(&mut writer, value);
            }
            let mut created = 1;
            let published_second = match writer.try_reserve() {
                Some(permit) => {
                    created += 1;
                    assert!(
                        permit
                            .commit(candidate(2, &drops), 0, PlanAdoption::Next)
                            .is_ok()
                    );
                    true
                }
                None => false,
            };
            let last = writer.withdraw();
            let (adopted, reader) = render.join().expect("render");

            assert!(adopted.len() <= 2);
            assert!(adopted.iter().filter(|id| **id == 1).count() <= 1);
            if adopted.contains(&2) {
                assert_eq!(adopted, [1, 2], "B adopts only after A");
            }
            match last {
                MailboxWithdrawal::Withdrawn(value, _, _) => {
                    assert!(!adopted.contains(&value.id), "adopted and withdrawn both");
                    let expected = if published_second { 2 } else { 1 };
                    assert_eq!(value.id, expected);
                }
                MailboxWithdrawal::Taken => {
                    let expected = if published_second { 2 } else { 1 };
                    assert_eq!(adopted.last(), Some(&expected));
                }
                MailboxWithdrawal::Nothing => panic!("a publication is never Nothing"),
            }
            if !withdrawn_first {
                assert_eq!(adopted.first(), Some(&1), "Taken means render adopted A");
            }
            drop((writer, reader));
            assert_eq!(
                drops.load(Ordering::Relaxed),
                created,
                "each value ends exactly once"
            );
        });
    }

    /// #1343 gate 2, cell reuse: render claims until it has adopted two candidates while control
    /// publishes three, each into a cell render emptied at an earlier claim. The claim's
    /// `Release` and `try_reserve`'s `Acquire` order render's move out of a cell before control's
    /// next write into it; loom's cells fail the model if either edge is missing. Both sides
    /// wait with `yield_now`, so loom schedules claims between the publications.
    #[test]
    fn spsc_loom_plan_mailbox_reuses_the_cell_render_emptied() {
        loom::model(|| {
            let drops = Arc::new(AtomicUsize::new(0));
            let (mut writer, mut reader) = plan_mailbox::<Candidate>();
            let render = loom::thread::spawn(move || {
                let mut adopted = Vec::new();
                while adopted.len() < 2 {
                    if let Some(observed) = reader.observe() {
                        if let Some(value) = reader.claim(observed) {
                            adopted.push(value.id);
                        }
                    }
                    loom::thread::yield_now();
                }
                (adopted, reader)
            });
            for id in 1..=3 {
                loop {
                    if let Some(permit) = writer.try_reserve() {
                        assert!(
                            permit
                                .commit(candidate(id, &drops), 0, PlanAdoption::Next)
                                .is_ok()
                        );
                        break;
                    }
                    loom::thread::yield_now();
                }
            }
            let (adopted, reader) = render.join().expect("render");
            assert_eq!(adopted, [1, 2]);
            drop((writer, reader));
            assert_eq!(
                drops.load(Ordering::Relaxed),
                3,
                "each value ends exactly once"
            );
        });
    }

    /// #1314 D2, D3 (I7): control publishes A at revision 7, then commits a live edit -- a
    /// `Relaxed` record store, then `store_revision(8)` as the commit's last write -- while render
    /// runs a block: claim if a cell is `Full`, then load the `Active` cell's revision. Under every
    /// interleaving render reads the initial 1 until it claims A, never 8 on the plan that does not
    /// carry it, and the record whenever it reads 8; the next block reads 8.
    #[test]
    fn spsc_loom_plan_mailbox_revision_follows_the_pending_candidate() {
        loom::model(|| {
            let drops = Arc::new(AtomicUsize::new(0));
            let record = Arc::new(AtomicUsize::new(0));
            let (mut writer, mut reader) = plan_mailbox::<Candidate>();
            assert!(
                !writer.store_revision(1),
                "the initial plan's cell is Active"
            );
            let render_record = Arc::clone(&record);
            let render = loom::thread::spawn(move || {
                let adopted = render_blocks(&mut reader, 1);
                let revision = reader.active_revision();
                let seen = render_record.load(Ordering::Relaxed);
                (adopted, revision, seen, reader)
            });
            let permit = writer.try_reserve().expect("an Empty cell");
            assert!(
                permit
                    .commit(candidate(1, &drops), 7, PlanAdoption::Next)
                    .is_ok()
            );
            record.store(1, Ordering::Relaxed);
            let _target = writer.store_revision(8);
            let (adopted, revision, seen, mut reader) = render.join().expect("render");
            if adopted.is_empty() {
                assert_eq!(
                    revision, 1,
                    "the running plan never reports the candidate's revision"
                );
            } else {
                assert!(revision == 7 || revision == 8, "read {revision}");
                assert!(
                    revision != 8 || seen == 1,
                    "revision 8 read without its record"
                );
            }
            let _ = render_blocks(&mut reader, 1);
            assert_eq!(reader.active_revision(), 8);
            drop((writer, reader));
            assert_eq!(drops.load(Ordering::Relaxed), 1);
        });
    }

    /// One #1311 race: control publishes A with its schedule and readiness flag, withdraws it,
    /// and if it came back publishes B into the same cell with another schedule and flag, then
    /// withdraws again. `*_claimable` says whether render's blocks (at 0 and 2) may ever claim it.
    struct ScheduleRace {
        first: PlanAdoption,
        first_ready: bool,
        first_claimable: bool,
        second: PlanAdoption,
        second_ready: bool,
        second_claimable: bool,
    }

    /// Render's scheduled blocks: per block one load of the word, the `Full` cell's schedule
    /// (I8), the readiness flag only through [`PlanAdoption::admits`] (a `Primed` candidate whose
    /// `not_before` the block reached), and a claim only if admitted. At most one CAS per block.
    fn render_scheduled_blocks(
        reader: &mut MailboxReader<Candidate>,
        starts: &[u64],
        ready: &AtomicBool,
    ) -> Vec<usize> {
        let mut adopted = Vec::new();
        for &start in starts {
            let before = reader.claim_attempts();
            if let Some(observed) = reader.observe() {
                let admitted = reader
                    .scheduled(observed)
                    .admits(start, |_, _| ready.load(Ordering::Relaxed));
                if admitted {
                    if let Some(value) = reader.claim(observed) {
                        adopted.push(value.id);
                    }
                }
            }
            assert!(reader.claim_attempts() - before <= 1, "one CAS per block");
        }
        adopted
    }

    fn run_schedule_race(race: &ScheduleRace) {
        let drops = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));
        let (mut writer, mut reader) = plan_mailbox::<Candidate>();
        let render_ready = Arc::clone(&ready);
        let render = loom::thread::spawn(move || {
            let adopted = render_scheduled_blocks(&mut reader, &[0, 2], &render_ready);
            (adopted, reader)
        });
        // The flag stands for #1355's per-ring `prime_required` atomics: stored before the
        // `Release` that publishes the candidate it belongs to.
        ready.store(race.first_ready, Ordering::Relaxed);
        let permit = writer.try_reserve().expect("an Empty cell");
        assert!(permit.commit(candidate(1, &drops), 0, race.first).is_ok());
        let mut created = 1;
        let first = writer.withdraw();
        let first_taken = matches!(first, MailboxWithdrawal::Taken);
        let mut last = None;
        if let MailboxWithdrawal::Withdrawn(value, _, adoption) = first {
            assert_eq!((value.id, adoption), (1, race.first), "A came back whole");
            drop(value);
            ready.store(race.second_ready, Ordering::Relaxed);
            let permit = writer.try_reserve().expect("the cell A left");
            assert!(permit.commit(candidate(2, &drops), 0, race.second).is_ok());
            created += 1;
            last = Some(writer.withdraw());
        }
        let (adopted, reader) = render.join().expect("render");

        assert!(adopted.len() <= 1, "adopted {adopted:?}");
        assert!(race.first_claimable || !adopted.contains(&1), "claimed A");
        assert!(race.second_claimable || !adopted.contains(&2), "claimed B");
        assert_eq!(
            first_taken,
            adopted == [1],
            "A is Taken exactly when adopted"
        );
        match last {
            Some(MailboxWithdrawal::Withdrawn(value, _, adoption)) => {
                assert_eq!((value.id, adoption), (2, race.second), "B came back whole");
                assert!(!adopted.contains(&2), "B adopted and withdrawn both");
            }
            Some(MailboxWithdrawal::Taken) => assert_eq!(adopted, [2]),
            Some(MailboxWithdrawal::Nothing) => panic!("a publication is never Nothing"),
            None => {}
        }
        drop((writer, reader));
        assert_eq!(
            drops.load(Ordering::Relaxed),
            created,
            "each value ends exactly once"
        );
    }

    /// #1311 gate 2, readiness: A is `Primed` and ready, B is `Primed` and not ready. Render may
    /// read A's word and A's `true` flag, then lose the claim to control's withdrawal and B's
    /// publication; a flag read for A never lets render claim B.
    #[test]
    fn spsc_loom_plan_mailbox_readiness_of_one_publication_never_claims_another() {
        loom::model(|| {
            run_schedule_race(&ScheduleRace {
                first: PlanAdoption::Primed {
                    not_before: 0,
                    lead_blocks: 1,
                },
                first_ready: true,
                first_claimable: true,
                second: PlanAdoption::Primed {
                    not_before: 0,
                    lead_blocks: 2,
                },
                second_ready: false,
                second_claimable: false,
            });
        });
    }

    /// #1311 gate 2, due sample: A is `Next`, B is due at 100, past render's blocks. Render may
    /// read A's kind, then lose the claim to B's publication; B is never claimed early.
    #[test]
    fn spsc_loom_plan_mailbox_due_check_of_one_publication_never_claims_another() {
        loom::model(|| {
            run_schedule_race(&ScheduleRace {
                first: PlanAdoption::Next,
                first_ready: false,
                first_claimable: true,
                second: PlanAdoption::NoEarlierThan(100),
                second_ready: false,
                second_claimable: false,
            });
        });
    }

    /// #1311 gate 2, the reverse: A is `Primed` and unready, then B is `Primed`, ready and due.
    /// A is never claimed, so control always takes it back, and B is adopted at most once.
    #[test]
    fn spsc_loom_plan_mailbox_an_unready_candidate_stays_withdrawable() {
        loom::model(|| {
            run_schedule_race(&ScheduleRace {
                first: PlanAdoption::Primed {
                    not_before: 0,
                    lead_blocks: 1,
                },
                first_ready: false,
                first_claimable: false,
                second: PlanAdoption::Primed {
                    not_before: 2,
                    lead_blocks: 1,
                },
                second_ready: true,
                second_claimable: true,
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{QueueGeneration, bounded_spsc, bounded_spsc_internal};
    use core::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    struct DropProbe(Arc<AtomicUsize>);

    impl Drop for DropProbe {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn endpoint_drop_order_is_safe_and_queued_values_drop_once() {
        let drops = Arc::new(AtomicUsize::new(0));
        let (mut producer, consumer) =
            bounded_spsc_internal(NonZeroUsize::new(2).expect("two"), QueueGeneration(3))
                .expect("queue");
        producer
            .try_push(DropProbe(Arc::clone(&drops)))
            .unwrap_or_else(|_| panic!("first probe"));
        drop(consumer);
        producer
            .try_push(DropProbe(Arc::clone(&drops)))
            .unwrap_or_else(|_| panic!("second probe"));
        drop(producer);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }

    /// Sets its flag when dropped, so a peer learns that this thread left its loop however it left
    /// it. `engine` cannot use `bench_support::producer::StopOnDrop`: `bench-support` depends on
    /// `engine` (#1251 D3).
    struct EndedOnDrop(Arc<AtomicBool>);

    impl Drop for EndedOnDrop {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }

    /// How long one side of the stress waits on a full or empty queue before it fails. The peer
    /// moves the queue within microseconds whenever it is running.
    const STALL_DEADLINE: Duration = Duration::from_secs(10);

    /// Fails the side that waits on the queue once its peer has ended, or after
    /// [`STALL_DEADLINE`] without progress. `peer_ended` must have been read before the failed
    /// queue operation: a peer that ended published its last operation first, so a queue still
    /// full or empty after it ended stays that way.
    fn fail_if_stalled(
        peer_ended: bool,
        stalled_since: &mut Option<Instant>,
        side: &str,
        peer: &str,
        position: u64,
    ) {
        assert!(
            !peer_ended,
            "spsc stress: the {peer} ended while the {side} waited at item {position}"
        );
        let since = *stalled_since.get_or_insert_with(Instant::now);
        assert!(
            since.elapsed() < STALL_DEADLINE,
            "spsc stress: the {side} made no progress at item {position} within {STALL_DEADLINE:?}"
        );
    }

    #[test]
    fn concurrent_spsc_stress() {
        const ITEMS: u64 = 1_000_000;
        let (mut producer, mut consumer) = bounded_spsc(
            NonZeroUsize::new(127).expect("capacity"),
            QueueGeneration(7),
        )
        .expect("queue");
        // Each thread marks its end however it leaves its loop, so a failed assertion on one side
        // fails the other's wait instead of leaving it spinning on a full or empty queue (#1251).
        let producer_ended = Arc::new(AtomicBool::new(false));
        let consumer_ended = Arc::new(AtomicBool::new(false));
        let (producer_end, consumer_peer) =
            (Arc::clone(&producer_ended), Arc::clone(&consumer_ended));
        let producer_thread = std::thread::spawn(move || {
            let _ended = EndedOnDrop(producer_end);
            let mut next = 0;
            let mut stalled_since = None;
            while next < ITEMS {
                let peer_ended = consumer_peer.load(Ordering::Acquire);
                match producer.try_push(next) {
                    Ok(()) => {
                        next += 1;
                        stalled_since = None;
                    }
                    Err(full) => {
                        assert_eq!(full.value, next);
                        fail_if_stalled(
                            peer_ended,
                            &mut stalled_since,
                            "producer",
                            "consumer",
                            next,
                        );
                        std::thread::yield_now();
                    }
                }
            }
            producer
        });
        let consumer_thread = std::thread::spawn(move || {
            let _ended = EndedOnDrop(consumer_ended);
            let mut expected = 0;
            let mut checksum = 0_u128;
            let mut stalled_since = None;
            while expected < ITEMS {
                let peer_ended = producer_ended.load(Ordering::Acquire);
                match consumer.try_pop() {
                    Ok(value) => {
                        assert_eq!(value, expected);
                        checksum = checksum.wrapping_add(u128::from(value));
                        expected += 1;
                        stalled_since = None;
                    }
                    Err(_) => {
                        fail_if_stalled(
                            peer_ended,
                            &mut stalled_since,
                            "consumer",
                            "producer",
                            expected,
                        );
                        std::thread::yield_now();
                    }
                }
            }
            (consumer, checksum)
        });
        let producer = producer_thread.join().expect("producer thread");
        let (consumer, checksum) = consumer_thread.join().expect("consumer thread");
        assert_eq!(producer.success_count(), ITEMS);
        assert_eq!(consumer.success_count(), ITEMS);
        assert_eq!(checksum, u128::from(ITEMS) * u128::from(ITEMS - 1) / 2);
    }

    /// The queue layout retains cursor cache-line alignment without pinning resource bytes.
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn ring_header_retains_cache_line_alignment() {
        let payload =
            super::bounded_spsc_retained_payload::<u64>(NonZeroUsize::new(1).expect("one"))
                .expect("layout");
        assert_eq!(payload.slot_count, 2);
        assert_eq!(payload.ring_header_align, 64);
    }

    /// The compare-wrap is the only wrap law in this module, and it agrees with `%` everywhere.
    #[test]
    fn wrap_increment_agrees_with_remainder() {
        for slot_count in 1..=9_usize {
            for cursor in 0..slot_count {
                assert_eq!(
                    super::wrap_increment(cursor, slot_count),
                    (cursor + 1) % slot_count,
                    "cursor {cursor} of {slot_count}"
                );
            }
        }
    }

    #[test]
    fn available_at_entry_is_bounded_and_handles_wrapped_cursors() {
        let (mut producer, mut consumer) =
            bounded_spsc(NonZeroUsize::new(3).expect("capacity"), QueueGeneration(11))
                .expect("queue");

        assert_eq!(consumer.available_at_entry(), 0);
        for value in 0..3 {
            producer.try_push(value).expect("space");
        }
        assert_eq!(consumer.available_at_entry(), 3);

        assert_eq!(consumer.try_pop(), Ok(0));
        assert_eq!(consumer.try_pop(), Ok(1));
        assert_eq!(consumer.available_at_entry(), 1);
        producer.try_push(3).expect("space after pop");
        producer.try_push(4).expect("space after pop");
        assert_eq!(consumer.available_at_entry(), 3);

        assert_eq!(consumer.try_pop(), Ok(2));
        assert_eq!(consumer.try_pop(), Ok(3));
        assert_eq!(consumer.available_at_entry(), 1);
    }

    #[test]
    fn producer_available_capacity_is_acquired_once_and_wrap_safe() {
        let (mut producer, mut consumer) =
            bounded_spsc(NonZeroUsize::new(3).expect("capacity"), QueueGeneration(13))
                .expect("queue");
        assert_eq!(producer.available_capacity(), 3);
        producer.try_push(1).expect("first");
        producer.try_push(2).expect("second");
        assert_eq!(producer.available_capacity(), 1);
        let successes = producer.success_count();
        let full = producer.full_count();
        assert_eq!(producer.available_capacity(), 1);
        assert_eq!(
            (producer.success_count(), producer.full_count()),
            (successes, full)
        );
        assert_eq!(consumer.try_pop(), Ok(1));
        assert_eq!(producer.available_capacity(), 2);
        producer.try_push(3).expect("wrapped slot");
        producer.try_push(4).expect("wrapped slot");
        assert_eq!(producer.available_capacity(), 0);
        let refused = producer.try_push(5).expect_err("full");
        assert_eq!(refused.value, 5);
        assert_eq!(producer.available_capacity(), 0);
        assert_eq!(producer.full_count(), full + 1);
        assert_eq!(consumer.try_pop(), Ok(2));
        assert_eq!(producer.available_capacity(), 1);
    }

    /// #1343 D2/D4: a claim built from a load taken before control withdrew and republished the
    /// candidate fails, although the word's cell states match again; only the generation tells
    /// the two words apart. A fresh load then claims the republished value.
    #[test]
    fn stale_claim_after_withdraw_and_republish_fails_on_the_generation() {
        use super::{MailboxWithdrawal, PlanAdoption, plan_mailbox};
        let (mut writer, mut reader) = plan_mailbox::<u32>();
        assert!(
            writer
                .try_reserve()
                .expect("empty")
                .commit(1, 0, PlanAdoption::Next)
                .is_ok()
        );
        let observed = reader.observe().expect("full");
        let MailboxWithdrawal::Withdrawn(value, _, _) = writer.withdraw() else {
            panic!("an unclaimed candidate is withdrawn")
        };
        assert!(
            writer
                .try_reserve()
                .expect("empty")
                .commit(value, 0, PlanAdoption::Next)
                .is_ok()
        );
        assert!(reader.claim(observed).is_none(), "a stale claim must fail");
        let observed = reader.observe().expect("full again");
        assert_eq!(reader.claim(observed), Some(1));
    }

    /// #1311 N4: a publication carries its own revision word and schedule. Candidate A is
    /// published `Primed` with revision 5 and withdrawn; candidate B then reuses the same cell with
    /// `Next` and revision 0 (a host that numbers no revisions). Render must see B's schedule and,
    /// after claiming it, B's revision, never the values A left in the cell.
    #[test]
    fn republication_into_a_reused_cell_never_inherits_its_schedule_or_revision() {
        use super::{MailboxWithdrawal, PlanAdoption, plan_mailbox};
        let primed = PlanAdoption::Primed {
            not_before: 96,
            lead_blocks: 2,
        };
        let (mut writer, mut reader) = plan_mailbox::<u32>();
        assert!(
            writer
                .try_reserve()
                .expect("empty")
                .commit(1, 5, primed)
                .is_ok()
        );
        let MailboxWithdrawal::Withdrawn(value, revision, adoption) = writer.withdraw() else {
            panic!("an unclaimed candidate is withdrawn")
        };
        assert_eq!((value, revision, adoption), (1, 5, primed));
        assert!(
            writer
                .try_reserve()
                .expect("the cell A left")
                .commit(2, 0, PlanAdoption::Next)
                .is_ok()
        );
        let observed = reader.observe().expect("B is Full");
        assert_eq!(reader.scheduled(observed), PlanAdoption::Next);
        assert_eq!(reader.claim(observed), Some(2));
        assert_eq!(reader.active_revision(), 0);
    }
}
