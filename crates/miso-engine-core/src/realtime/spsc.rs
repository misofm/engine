//! Bounded single-producer/single-consumer rings.
//!
//! Queue storage is shared by two non-cloneable endpoints. Reference-count operations occur only
//! when endpoints are created, moved, or destroyed; `try_push` and `try_pop` touch only the fixed
//! slots and cursor atomics.

#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    mem::MaybeUninit,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Arc;

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
    slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
    slots_len: usize,
    logical_capacity: usize,
    generation: QueueGeneration,
    producer: AtomicUsize,
    consumer: AtomicUsize,
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
        let mut cursor = *self.consumer.get_mut();
        let producer = *self.producer.get_mut();
        while cursor != producer {
            // SAFETY: endpoint destruction is complete before the final `Arc<Ring<T>>` drop, and
            // every cursor position in the half-open consumer..producer range is initialized
            // exactly once. No producer or consumer can access a slot concurrently here.
            unsafe {
                self.slots[cursor].get_mut().assume_init_drop();
            }
            cursor = (cursor + 1) % self.slots_len;
        }
    }
}

/// Native producer endpoint. `Cell` intentionally makes it `!Sync`; ownership transfer is `Send`.
pub struct Producer<T: Send + 'static> {
    ring: Arc<Ring<T>>,
    local: usize,
    successes: u64,
    full: u64,
    _not_sync: PhantomData<Cell<()>>,
}
/// Native consumer endpoint and sole allocation owner. It is `!Sync` and must outlive producer.
pub struct Consumer<T: Send + 'static> {
    ring: Arc<Ring<T>>,
    local: usize,
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
    let ring = Arc::new(Ring {
        slots: slots.into_boxed_slice(),
        slots_len,
        logical_capacity: capacity.get(),
        generation,
        producer: AtomicUsize::new(0),
        consumer: AtomicUsize::new(0),
    });
    let producer = Producer {
        ring: Arc::clone(&ring),
        local: 0,
        successes: 0,
        full: 0,
        _not_sync: PhantomData,
    };
    Ok((
        producer,
        Consumer {
            ring,
            local: 0,
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
    /// Try one bounded push; it never retries or blocks.
    pub fn try_push(&mut self, value: T) -> Result<(), QueueFull<T>> {
        let local = self.local;
        let (next, full, generation) = {
            let ring = self.ring();
            (
                (local + 1) % ring.slots_len,
                (local + 1) % ring.slots_len == ring.consumer.load(Ordering::Acquire),
                ring.generation,
            )
        };
        if full {
            self.full = self.full.saturating_add(1);
            return Err(QueueFull {
                value,
                generation,
                full_count: self.full,
            });
        }
        let ring = self.ring();
        // SAFETY: only this producer writes its local slot after acquiring consumer's cursor.
        unsafe {
            (*ring.slots[self.local].get()).write(value);
        }
        ring.producer.store(next, Ordering::Release);
        self.local = next;
        self.successes = self.successes.saturating_add(1);
        Ok(())
    }
    /// Reserve one slot without publishing it. Only realtime exchange uses this transactionally.
    pub(crate) fn try_reserve(&mut self) -> Option<PushPermit<'_, T>> {
        let local = self.local;
        let (next, full) = {
            let ring = self.ring();
            (
                (local + 1) % ring.slots_len,
                (local + 1) % ring.slots_len == ring.consumer.load(Ordering::Acquire),
            )
        };
        if full {
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
        let ring = self.producer.ring();
        let local = self.producer.local;
        // SAFETY: `try_reserve` acquired free capacity and only the unique producer owns this slot.
        unsafe {
            (*ring.slots[local].get()).write(value);
        }
        ring.producer.store(self.next, Ordering::Release);
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
    /// Try one bounded pop; it never retries or blocks.
    pub fn try_pop(&mut self) -> Result<T, QueueEmpty> {
        let local = self.local;
        let (is_empty, generation) = {
            let ring = self.ring();
            (
                local == ring.producer.load(Ordering::Acquire),
                ring.generation,
            )
        };
        if is_empty {
            self.empty = self.empty.saturating_add(1);
            return Err(QueueEmpty {
                generation,
                empty_count: self.empty,
            });
        }
        let ring = self.ring();
        // SAFETY: producer release + this acquire publishes initialization; only consumer reads it.
        let value = unsafe { (*ring.slots[self.local].get()).assume_init_read() };
        let next = (local + 1) % ring.slots_len;
        ring.consumer.store(next, Ordering::Release);
        self.local = next;
        self.successes = self.successes.saturating_add(1);
        Ok(value)
    }
}
// REALTIME_POLICY_END

/// Browser/single-owner fallback ring. It contains no atomics and makes no shared-memory claim.
pub struct LocalRing<T: Copy> {
    slots: Box<[Option<T>]>,
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
        Ok(Self {
            slots: vec![None; slots].into_boxed_slice(),
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
        let next = (self.head + 1) % self.slots.len();
        if next == self.tail {
            self.full = self.full.saturating_add(1);
            Err(QueueFull {
                value,
                generation: self.generation,
                full_count: self.full,
            })
        } else {
            self.slots[self.head] = Some(value);
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
            let value = self.slots[self.tail]
                .take()
                .expect("prepared local ring slot");
            self.tail = (self.tail + 1) % self.slots.len();
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
    use loom::cell::UnsafeCell;
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::thread;

    struct ModelRing {
        slots: [UnsafeCell<usize>; 2],
        producer: AtomicUsize,
        consumer: AtomicUsize,
    }

    #[test]
    fn spsc_loom_publication_visibility_and_reuse() {
        loom::model(|| {
            let ring = Arc::new(ModelRing {
                slots: [UnsafeCell::new(0), UnsafeCell::new(0)],
                producer: AtomicUsize::new(0),
                consumer: AtomicUsize::new(0),
            });
            let producer_ring = Arc::clone(&ring);
            let producer = thread::spawn(move || {
                let local = 0;
                let next = 1;
                assert_ne!(
                    next,
                    producer_ring.consumer.load(Ordering::Acquire),
                    "one-slot model unexpectedly full"
                );
                producer_ring.slots[local].with_mut(|slot| {
                    // SAFETY: the model has one producer, and the acquired consumer cursor proves
                    // this slot is not being read.
                    unsafe { *slot = 0x5a5a }
                });
                producer_ring.producer.store(next, Ordering::Release);
            });
            let consumer = thread::spawn(move || {
                while ring.producer.load(Ordering::Acquire) == 0 {
                    thread::yield_now();
                }
                let value = ring.slots[0].with(|slot| {
                    // SAFETY: acquiring producer cursor 1 proves slot zero was initialized.
                    unsafe { *slot }
                });
                assert_eq!(value, 0x5a5a);
                ring.consumer.store(1, Ordering::Release);
            });
            producer.join().expect("producer");
            consumer.join().expect("consumer");
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
}
