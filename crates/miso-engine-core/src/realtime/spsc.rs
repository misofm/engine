//! Bounded single-producer/single-consumer rings.
//!
//! Queue storage is shared by two non-cloneable endpoints. Reference-count operations occur only
//! when endpoints are created, moved, or destroyed; `try_push` and `try_pop` touch only the fixed
//! slots and cursor atomics.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell, marker::PhantomData, mem::MaybeUninit, num::NonZeroUsize};
use sync::{Arc, AtomicUsize, Ordering, UnsafeCell};

/// The concurrency primitives the ring is built from.
///
/// Under `--cfg loom` the real ring is instantiated on loom's `Arc`, `AtomicUsize` and
/// `UnsafeCell`, so `spsc_loom` explores *this* code rather than a hand-written model of it
/// (#84 phase B). `cfg(loom)` is only a supported configuration for `cargo test`; a non-test
/// loom build is not a shape this crate promises to link.
#[cfg(not(loom))]
mod sync {
    pub(super) use core::cell::UnsafeCell;
    pub(super) use core::sync::atomic::{AtomicUsize, Ordering};
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
    pub(super) use loom::sync::atomic::{AtomicUsize, Ordering};

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
    // `ArcInner`'s `{ strong, weak, data }` order (#84 phase B; pinned by
    // `ring_header_is_arc_counts_plus_three_cache_lines`).
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
    /// Alias naming a full result as producer overflow.
    #[must_use]
    pub const fn overflow_count(&self) -> u64 {
        self.full
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
        let next = wrap_increment(self.local, self.ring.slots_len);
        if self.is_full_at(next) {
            self.full = self.full.saturating_add(1);
            return Err(QueueFull {
                value,
                generation: self.ring.generation,
                full_count: self.full,
            });
        }
        // SAFETY: only this producer writes its local slot after acquiring consumer's cursor.
        sync::with_mut(&self.ring.slots[self.local], |slot| unsafe {
            (*slot).write(value);
        });
        self.ring.producer.0.store(next, Ordering::Release);
        self.local = next;
        self.successes = self.successes.saturating_add(1);
        Ok(())
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
    /// Immutable queue generation.
    #[must_use]
    pub fn generation(&self) -> QueueGeneration {
        self.ring().generation
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
    /// Alias naming an empty result as consumer underrun.
    #[must_use]
    pub const fn underrun_count(&self) -> u64 {
        self.empty
    }
    /// Whether the queue currently holds nothing for this consumer.
    ///
    /// This is the bounded, counter-free observation the scheduler uses to decide whether a
    /// worker is idle; it never pops and never touches the empty/underrun counters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.local == self.ring().producer.0.load(Ordering::Acquire)
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

/// Browser/single-owner fallback ring. It contains no atomics and makes no shared-memory claim.
pub struct LocalRing<T: Copy> {
    /// `MaybeUninit<T>` rather than `Option<T>`: the discriminant was a per-slot byte the browser
    /// render ring paid for on every push, and `Option::take().expect(..)` put a panic path inside
    /// a `REALTIME_POLICY` region (#84 F12). `head`/`tail` already carry the occupancy the
    /// discriminant duplicated.
    slots: Box<[MaybeUninit<T>]>,
    capacity: usize,
    head: usize,
    tail: usize,
    generation: QueueGeneration,
    full: u64,
    empty: u64,
}
impl<T: Copy> LocalRing<T> {
    /// Prepare a local ring with exact capacity.
    pub fn new(capacity: NonZeroUsize, generation: QueueGeneration) -> Result<Self, SpscError> {
        let slots = capacity
            .get()
            .checked_add(1)
            .ok_or(SpscError::CapacityOverflow)?;
        let mut storage = Vec::with_capacity(slots);
        storage.resize_with(slots, MaybeUninit::uninit);
        Ok(Self {
            slots: storage.into_boxed_slice(),
            capacity: capacity.get(),
            head: 0,
            tail: 0,
            generation,
            full: 0,
            empty: 0,
        })
    }
    // REALTIME_POLICY_BEGIN
    /// Push without retry.
    pub fn try_push(&mut self, value: T) -> Result<(), QueueFull<T>> {
        let next = wrap_increment(self.head, self.slots.len());
        if next == self.tail {
            self.full = self.full.saturating_add(1);
            Err(QueueFull {
                value,
                generation: self.generation,
                full_count: self.full,
            })
        } else {
            self.slots[self.head].write(value);
            self.head = next;
            Ok(())
        }
    }
    /// Pop without retry.
    pub fn try_pop(&mut self) -> Result<T, QueueEmpty> {
        if self.tail == self.head {
            self.empty = self.empty.saturating_add(1);
            Err(QueueEmpty {
                generation: self.generation,
                empty_count: self.empty,
            })
        } else {
            // SAFETY: `head` and `tail` are the single owner's cursors, so every slot in the
            // half-open `tail..head` range was written by `try_push` and not yet read. `T: Copy`,
            // so reading a slot twice could not double-drop even if the cursors were wrong.
            let value = unsafe { self.slots[self.tail].assume_init() };
            self.tail = wrap_increment(self.tail, self.slots.len());
            Ok(value)
        }
    }
    /// Exact usable queue capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
    /// Immutable queue generation.
    #[must_use]
    pub const fn generation(&self) -> QueueGeneration {
        self.generation
    }
    /// Producer-local saturating full/overflow count.
    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.full
    }
    /// Alias naming a full result as producer overflow.
    #[must_use]
    pub const fn overflow_count(&self) -> u64 {
        self.full
    }
    /// Consumer-local saturating empty/underrun count.
    #[must_use]
    pub const fn empty_count(&self) -> u64 {
        self.empty
    }
    /// Alias naming an empty result as consumer underrun.
    #[must_use]
    pub const fn underrun_count(&self) -> u64 {
        self.empty
    }
    // REALTIME_POLICY_END
}

#[cfg(all(test, loom))]
mod loom_tests {
    use super::{QueueGeneration, bounded_spsc};
    use core::num::NonZeroUsize;

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
}

#[cfg(test)]
mod tests {
    use super::{QueueGeneration, bounded_spsc, bounded_spsc_internal};
    use core::num::NonZeroUsize;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    #[test]
    fn concurrent_spsc_stress() {
        const ITEMS: u64 = 1_000_000;
        let (mut producer, mut consumer) = bounded_spsc(
            NonZeroUsize::new(127).expect("capacity"),
            QueueGeneration(7),
        )
        .expect("queue");
        let producer_thread = std::thread::spawn(move || {
            let mut next = 0;
            while next < ITEMS {
                match producer.try_push(next) {
                    Ok(()) => next += 1,
                    Err(full) => {
                        assert_eq!(full.value, next);
                        std::thread::yield_now();
                    }
                }
            }
            producer
        });
        let consumer_thread = std::thread::spawn(move || {
            let mut expected = 0;
            let mut checksum = 0_u128;
            while expected < ITEMS {
                match consumer.try_pop() {
                    Ok(value) => {
                        assert_eq!(value, expected);
                        checksum = checksum.wrapping_add(u128::from(value));
                        expected += 1;
                    }
                    Err(_) => std::thread::yield_now(),
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

    /// #84 phase B, eval B-3: the ring header is `Arc`'s two counts followed by three cache
    /// lines -- the read-mostly header, the producer cursor and the consumer cursor. The oracle is
    /// `core::alloc::Layout` (the compiler), never a copied run output; the fixture pins that
    /// number so the resource oracles cannot drift from it silently.
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn ring_header_is_arc_counts_plus_three_cache_lines() {
        let payload =
            super::bounded_spsc_retained_payload::<u64>(NonZeroUsize::new(1).expect("one"))
                .expect("layout");
        assert_eq!(
            (
                payload.slot_count,
                payload.ring_header_bytes,
                payload.ring_header_align
            ),
            (2, 256, 64)
        );
        assert_eq!(core::mem::size_of::<super::Ring<u64>>(), 192);
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
}
