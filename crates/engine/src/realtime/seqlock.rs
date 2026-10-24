//! The engine's one single-writer seqlock: a fixed-width record of `u64` words (B15-R1).
//!
//! # What it is for
//!
//! A *level* that render publishes and any thread reads: the applied-revision watermark (#1314)
//! and a source ring's seek report (#1316) are both instances. Render overwrites the record in
//! bounded time and never waits; a reader takes the newest whole record, or reports
//! [`PublishedWordsBusy`] after a bounded number of torn attempts. Nothing is queued, so nothing
//! can back up or be dropped.
//!
//! The record is generic over its width and knows nothing of its words' meaning, so any owner,
//! a ring or not, can publish its own words through it.
//!
//! # The protocol, spelled in safe Rust
//!
//! Every published word is its own atomic, and an odd/even sequence counter fences the window.
//! The writer is the only one: [`PublishedWordsWriter`] is not `Clone`, and publishing takes
//! `&mut self`. A reader that sees an odd counter, or a different counter before and after its
//! loads, retries, at most [`MAXIMUM_READ_ATTEMPTS`] times. There is no `unsafe` here: this module
//! is inside the realtime root that `tools/realtime-policy` holds to its approved-unsafe list.
//!
//! An owner that embeds a [`PublishedWords`] in its own shared allocation (an observation slot,
//! #1482) calls the record's crate-private publish and read directly, and keeps the single-writer
//! discipline by ownership: its one publisher handle is not `Clone`.
//!
//! # The counter's width
//!
//! The counter is a `u64` (#1482 R3). At one publication per block it cannot wrap in a session's
//! lifetime, whereas a `u32` wraps in about 130 days at 48 kHz and 128-frame blocks, and a reader
//! stalled across exactly 2^32 publications would then accept a torn record. A `u64` costs the
//! writer the same two stores.

use core::sync::atomic::AtomicUsize;

// The ring's shim: core's atomics and `std`'s `Arc` in every shipped build, loom's under
// `--cfg loom`, so `seqlock_loom` explores this protocol itself (#1702).
use super::spsc::sync::{Arc, AtomicU64, Ordering, fence};

/// The bound on one read's attempts. It keeps a read provably finite while a writer publishing in
/// a tight loop still lets the reader through.
///
/// A production writer publishes at most once per render block (an observation slot once per
/// closed window), so a read retries only when it lands inside one record store, which is
/// vanishingly rare. The number is set for the *stress* case instead: #143's `E11` runs a writer
/// that publishes continuously in a tight loop, and at four attempts that measured a 50% give-up
/// rate, because four reads of a record are comparable in length to the store they race.
/// Sixty-four took the measured give-up rate to zero there while keeping the loop provably
/// finite, which is what the bound is for. A reader is never on the render thread, so a retry
/// costs a render block nothing.
pub const MAXIMUM_READ_ATTEMPTS: usize = 64;

/// The attempts one read takes. Under `--cfg loom` it is two, so `seqlock_loom` can explore every
/// interleaving of a read that retries once: each attempt's no-tear argument is independent of the
/// bound, and 64 spinning attempts are beyond loom's reach (#1702). Every shipped build takes
/// [`MAXIMUM_READ_ATTEMPTS`].
const READ_ATTEMPTS: usize = if cfg!(loom) { 2 } else { MAXIMUM_READ_ATTEMPTS };

/// A read gave up after [`MAXIMUM_READ_ATTEMPTS`] torn attempts; the caller retries later.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublishedWordsBusy;

/// The shared record: a sequence counter and `N` words. One allocation, made off render.
#[derive(Debug)]
pub struct PublishedWords<const N: usize> {
    /// Odd while a publication is in flight, even between them.
    sequence_lock: AtomicU64,
    words: [AtomicU64; N],
}

/// The writer's half: the record's only writer. Not `Clone`.
#[derive(Debug)]
pub struct PublishedWordsWriter<const N: usize> {
    record: Arc<PublishedWords<N>>,
}

/// Any-thread reader handle. Cloning it shares the one record.
#[derive(Clone, Debug)]
pub struct PublishedWordsReader<const N: usize> {
    record: Arc<PublishedWords<N>>,
}

/// Make one record holding `initial`, and split it into its writer and a reader. Allocates the
/// record once; call it on a control thread.
#[must_use]
pub fn published_words<const N: usize>(
    initial: [u64; N],
) -> (PublishedWordsWriter<N>, PublishedWordsReader<N>) {
    let record = Arc::new(PublishedWords::new(initial));
    let reader = PublishedWordsReader {
        record: Arc::clone(&record),
    };
    (PublishedWordsWriter { record }, reader)
}

/// Exact engine-owned bytes of one shared record of `N` words, with its shared-owner header.
#[must_use]
pub fn published_words_retained_bytes<const N: usize>() -> usize {
    // `Arc`'s allocation is `{ strong, weak, data }`, as the mailbox's and the ring's mirrors spell.
    #[repr(C)]
    struct SharedAllocation<const N: usize> {
        strong: AtomicUsize,
        weak: AtomicUsize,
        record: PublishedWords<N>,
    }
    core::alloc::Layout::new::<SharedAllocation<N>>().size()
}

impl<const N: usize> PublishedWords<N> {
    /// A record holding `initial` with an even counter, for an owner that embeds it in its own
    /// shared allocation. Call it on a control thread.
    pub(crate) fn new(initial: [u64; N]) -> Self {
        Self {
            sequence_lock: AtomicU64::new(0),
            words: initial.map(AtomicU64::new),
        }
    }
}

// REALTIME_POLICY_BEGIN
impl<const N: usize> PublishedWords<N> {
    /// Overwrite the record with `words`. Wait-free: two counter stores and `N` word stores.
    ///
    /// The caller is the record's only writer: [`PublishedWordsWriter`] by `&mut self`, an
    /// embedding owner by its one non-`Clone` publisher handle.
    #[inline]
    pub(crate) fn publish(&self, words: &[u64; N]) {
        let opening = self.sequence_lock.load(Ordering::Relaxed).wrapping_add(1);
        self.sequence_lock.store(opening, Ordering::Relaxed);
        // Keeps the word stores below from being observed before the counter went odd.
        fence(Ordering::Release);
        for (cell, word) in self.words.iter().zip(words) {
            cell.store(*word, Ordering::Relaxed);
        }
        self.sequence_lock
            .store(opening.wrapping_add(1), Ordering::Release);
    }
}

impl<const N: usize> PublishedWordsWriter<N> {
    /// Overwrite the record with `words`. Wait-free: two counter stores and `N` word stores.
    #[inline]
    pub fn publish(&mut self, words: &[u64; N]) {
        self.record.publish(words);
    }
}
// REALTIME_POLICY_END

impl<const N: usize> PublishedWords<N> {
    /// The newest whole record, or [`PublishedWordsBusy`] if every one of the bounded attempts
    /// landed inside a publication. Never a partial record: the words are taken between two equal
    /// even counter reads, so what comes back is exactly one publication.
    pub(crate) fn read(&self) -> Result<[u64; N], PublishedWordsBusy> {
        for _ in 0..READ_ATTEMPTS {
            let before = self.sequence_lock.load(Ordering::Acquire);
            if !before.is_multiple_of(2) {
                continue;
            }
            let mut words = [0_u64; N];
            for (word, cell) in words.iter_mut().zip(&self.words) {
                *word = cell.load(Ordering::Relaxed);
            }
            fence(Ordering::Acquire);
            if self.sequence_lock.load(Ordering::Relaxed) == before {
                return Ok(words);
            }
        }
        Err(PublishedWordsBusy)
    }
}

impl<const N: usize> PublishedWordsReader<N> {
    /// The newest whole record, or [`PublishedWordsBusy`] if every one of the bounded attempts
    /// landed inside a publication. Never a partial record: the words are taken between two equal
    /// even counter reads, so what comes back is exactly one publication.
    pub fn read(&self) -> Result<[u64; N], PublishedWordsBusy> {
        self.record.read()
    }
}

#[cfg(test)]
mod tests {
    use super::{PublishedWordsBusy, published_words};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{Duration, Instant};

    /// Every word derived from one counter, so a mixture of two publications is detectable.
    fn record(counter: u64) -> [u64; 6] {
        [
            counter,
            counter.wrapping_mul(128),
            counter % 7,
            counter.wrapping_mul(3),
            counter ^ 0x5555_5555_5555_5555,
            !counter,
        ]
    }

    struct DoneOnDrop(Arc<AtomicBool>);

    impl Drop for DoneOnDrop {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }

    /// #1314 gate 5, moved here with the protocol (B15-R1): a writer publishing in a tight loop
    /// never lets a reader see two publications' words in one record; a busy result is the only
    /// alternative to a whole one.
    #[test]
    fn a_read_is_one_whole_publication_or_busy() {
        const PUBLICATIONS: u64 = 1_000_000;
        const STALL_DEADLINE: Duration = Duration::from_secs(10);
        let (mut writer, reader) = published_words(record(0));
        let barrier = Arc::new(Barrier::new(2));
        let writer_barrier = Arc::clone(&barrier);
        let done = Arc::new(AtomicBool::new(false));
        let writer_done = Arc::clone(&done);
        let publisher = thread::spawn(move || {
            let _done = DoneOnDrop(writer_done);
            writer_barrier.wait();
            for counter in 1..=PUBLICATIONS {
                writer.publish(&record(counter));
            }
        });
        barrier.wait();
        let (mut whole, mut busy, mut torn, mut newest) = (0_u64, 0_u64, 0_u64, 0_u64);
        let started = Instant::now();
        while !done.load(Ordering::Acquire) {
            match reader.read() {
                Ok(observed) => {
                    whole += 1;
                    if observed != record(observed[0]) {
                        torn += 1;
                    }
                    newest = newest.max(observed[0]);
                }
                Err(PublishedWordsBusy) => busy += 1,
            }
            assert!(started.elapsed() < STALL_DEADLINE, "the writer stalled");
        }
        publisher.join().expect("writer");
        assert_eq!(torn, 0, "{torn} of {whole} reads mixed two publications");
        assert!(
            whole > 0,
            "the reader never got a whole record ({busy} busy)"
        );
        assert_eq!(
            reader.read().expect("a quiescent read is never busy"),
            record(PUBLICATIONS)
        );
        assert!(newest <= PUBLICATIONS);
    }
}

#[cfg(all(test, loom))]
mod loom_tests {
    use super::published_words;

    /// Words that are all equal within one publication and distinct across publications, so any
    /// mixture of two records is detectable.
    const RECORDS: [[u64; 2]; 3] = [[0, 0], [1, 1], [2, 2]];

    /// #1702: the real seqlock on loom's atomics and fences. One writer publishes two records
    /// while one reader reads; under every interleaving loom explores (including the stale loads
    /// its weak memory model allows), an `Ok` is one whole record -- the initial one or a
    /// published one. `Busy` is allowed. After the writer is joined, a read is the last record.
    #[test]
    fn seqlock_loom_a_read_is_one_whole_record_or_busy() {
        loom::model(|| {
            let (mut writer, reader) = published_words(RECORDS[0]);
            let publisher = loom::thread::spawn(move || {
                writer.publish(&RECORDS[1]);
                writer.publish(&RECORDS[2]);
            });
            if let Ok(observed) = reader.read() {
                assert!(
                    RECORDS.contains(&observed),
                    "a read returned a torn record {observed:?}"
                );
            }
            publisher.join().expect("writer");
            assert_eq!(reader.read(), Ok(RECORDS[2]));
        });
    }
}
