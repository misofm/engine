//! The applied-revision watermark (#1314, decision 15 D15-17).
//!
//! # What it answers
//!
//! "Which committed revision is audible, and since which render sample?" The control thread
//! commits a revision and returns at once; render applies it at some later block that the control
//! thread cannot predict (a live record is drained by whatever render call comes next, and a
//! rebuild applies when render adopts its candidate, which a paused host never reaches). So
//! completion is *observed*, never awaited: render publishes `(revision, first sample in effect,
//! outcome flags)` here, with saturating per-outcome counters, and any thread reads it.
//!
//! The revision is the highest one that is in effect together with every revision before it.
//! Revisions travel with their plans (the mailbox cell words of `spsc.rs`), and render reads only
//! the cell of the plan it runs, so the watermark never reports a revision before the plan that
//! carries it renders.
//!
//! # Why this is a level, not a queue
//!
//! Revisions are monotone, so the newest one answers the host's question for every older one: a
//! watermark at `b` says every revision up to `b` is in effect. Nothing is queued, so nothing can
//! back up, be dropped or make render wait. Render overwrites the record in bounded time; a reader
//! that missed an advance loses nothing, because the counters accumulate and the revision only
//! grows. That is the acked-batch answer: no ack can precede a drop, because there is no queue to
//! drop from (#1314 D8).
//!
//! # The seqlock, spelled in safe Rust
//!
//! The pattern of `observe.rs`: every published word is its own atomic, and an odd/even sequence
//! counter fences the window. Render is the only writer. A reader that sees an odd counter, or a
//! different counter before and after its loads, retries, at most [`MAXIMUM_READ_ATTEMPTS`] times,
//! then reports [`WatermarkBusy`]. There is no `unsafe` here: this module is inside the realtime
//! root that `scripts/check-realtime-policy.sh` holds to its approved-unsafe list.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering, fence};
use std::sync::Arc;

/// Outcome flag: the covered revisions completed exactly as committed.
pub const OUTCOME_EXACT: u64 = 1;
/// Outcome flag: a warm successor could not adopt and its revisions completed through the
/// transition (#1397, #1358).
pub const OUTCOME_TRANSITION_FALLBACK: u64 = 2;
/// Outcome flag: the covered revisions include ones folded in from candidates a newer candidate
/// replaced before render adopted them (#1310).
pub const OUTCOME_SUPERSEDED: u64 = 4;

/// The same bound as `observe.rs`'s, for the same reason: it keeps a read provably finite while a
/// writer publishing in a tight loop still lets the reader through. Render publishes at most once
/// per block, so a production read retries only when it lands inside one seven-word store.
const MAXIMUM_READ_ATTEMPTS: usize = 64;

/// How a candidate's own revisions complete when render adopts it (#1314 D1, D5).
///
/// Written only while control owns the candidate, before publication. In this slice every
/// candidate is [`Self::Exact`]; the only writer of [`Self::TransitionFallback`] is the
/// transition fallback (#1397, #1358).
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CandidateOutcome {
    /// The candidate's revisions complete as [`OUTCOME_EXACT`].
    #[default]
    Exact = 0,
    /// The candidate is the transition a warm successor fell back to; its revisions complete as
    /// [`OUTCOME_TRANSITION_FALLBACK`].
    TransitionFallback = 1,
}

/// One whole watermark: what a reader gets, and the only thing that crosses.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanWatermark {
    /// The highest committed revision in effect together with every revision before it.
    pub revision: u64,
    /// The absolute render sample of the first block in which `revision` was in effect: a live
    /// value's ramp starts at it, and a rebuilt plan renders from it. `0` for the initial revision.
    pub first_sample: u64,
    /// The OR of the `OUTCOME_*` flags over the revisions the last advance covered.
    pub outcome_flags: u64,
    /// Saturating count of revisions completed as [`OUTCOME_EXACT`].
    pub exact: u64,
    /// Saturating count of revisions completed as [`OUTCOME_TRANSITION_FALLBACK`].
    pub transition_fallback: u64,
    /// Saturating count of revisions completed as [`OUTCOME_SUPERSEDED`].
    pub superseded: u64,
}

/// A read gave up after `MAXIMUM_READ_ATTEMPTS` torn attempts; the caller retries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatermarkBusy;

/// The shared record. One allocation, made when the plan exchange is prepared.
#[derive(Debug)]
pub(crate) struct WatermarkRecord {
    /// Odd while a publication is in flight, even between them.
    sequence_lock: AtomicU64,
    revision: AtomicU64,
    first_sample: AtomicU64,
    flags: AtomicU64,
    exact: AtomicU64,
    transition_fallback: AtomicU64,
    superseded: AtomicU64,
}

/// Exact engine-owned bytes of the shared watermark record, with its shared-owner header.
pub(crate) fn watermark_retained_bytes() -> usize {
    // `Arc`'s allocation is `{ strong, weak, data }`, as the mailbox's and the ring's mirrors spell.
    #[repr(C)]
    struct SharedWatermarkAllocation {
        strong: AtomicUsize,
        weak: AtomicUsize,
        record: WatermarkRecord,
    }
    core::alloc::Layout::new::<SharedWatermarkAllocation>().size()
}

/// Render's half: the only writer, with the render-local state an advance needs.
#[derive(Debug)]
pub(crate) struct WatermarkWriter {
    record: Arc<WatermarkRecord>,
    /// What the record holds; render is its only writer, so it never loads it back.
    current: PlanWatermark,
    /// `superseded` of the candidates claimed since the last advance (#1314 D5).
    pending_superseded: u64,
    /// `outcome` of the candidates claimed since the last advance (#1314 D5).
    pending_outcome: CandidateOutcome,
}

/// Any-thread reader handle. Cloning it shares the one record.
#[derive(Clone, Debug)]
pub struct PlanWatermarkReader {
    record: Arc<WatermarkRecord>,
}

/// Make the record with the initial watermark `(initial_revision, 0, EXACT)` and every counter 0.
pub(crate) fn plan_watermark(initial_revision: u64) -> (WatermarkWriter, PlanWatermarkReader) {
    let current = PlanWatermark {
        revision: initial_revision,
        first_sample: 0,
        outcome_flags: OUTCOME_EXACT,
        exact: 0,
        transition_fallback: 0,
        superseded: 0,
    };
    let record = Arc::new(WatermarkRecord {
        sequence_lock: AtomicU64::new(0),
        revision: AtomicU64::new(current.revision),
        first_sample: AtomicU64::new(current.first_sample),
        flags: AtomicU64::new(current.outcome_flags),
        exact: AtomicU64::new(current.exact),
        transition_fallback: AtomicU64::new(current.transition_fallback),
        superseded: AtomicU64::new(current.superseded),
    });
    let reader = PlanWatermarkReader {
        record: Arc::clone(&record),
    };
    (
        WatermarkWriter {
            record,
            current,
            pending_superseded: 0,
            pending_outcome: CandidateOutcome::Exact,
        },
        reader,
    )
}

// REALTIME_POLICY_BEGIN
impl WatermarkWriter {
    /// Render claimed a candidate: keep its `superseded` count and its `outcome` for the first
    /// advance after the claim (#1314 D5). Claims between two advances accumulate, so no
    /// revision's outcome is lost to a block that published nothing.
    #[inline]
    pub(crate) fn note_claim(&mut self, superseded: u64, outcome: CandidateOutcome) {
        self.pending_superseded = self.pending_superseded.saturating_add(superseded);
        if outcome == CandidateOutcome::TransitionFallback {
            self.pending_outcome = CandidateOutcome::TransitionFallback;
        }
    }

    /// Advance to `revision`, in effect from `first_sample`, if it is above the published one;
    /// otherwise write nothing.
    ///
    /// An advance from `a` to `b` completes `b - a` revisions. The first advance after a claim
    /// adds the claimed `superseded` count to its counter and sets [`OUTCOME_SUPERSEDED`]; the
    /// rest goes to `exact`, or to `transition_fallback` if the claimed outcome was the fallback,
    /// with its flag. Later advances on the same plan are [`OUTCOME_EXACT`].
    #[inline]
    pub(crate) fn advance(&mut self, revision: u64, first_sample: u64) {
        if revision <= self.current.revision {
            return;
        }
        let covered = revision - self.current.revision;
        let superseded = core::mem::take(&mut self.pending_superseded);
        let outcome = core::mem::take(&mut self.pending_outcome);
        let rest = covered.saturating_sub(superseded);
        let mut flags = 0;
        if superseded != 0 {
            flags |= OUTCOME_SUPERSEDED;
            self.current.superseded = self.current.superseded.saturating_add(superseded);
        }
        if rest != 0 {
            match outcome {
                CandidateOutcome::Exact => {
                    flags |= OUTCOME_EXACT;
                    self.current.exact = self.current.exact.saturating_add(rest);
                }
                CandidateOutcome::TransitionFallback => {
                    flags |= OUTCOME_TRANSITION_FALLBACK;
                    self.current.transition_fallback =
                        self.current.transition_fallback.saturating_add(rest);
                }
            }
        }
        self.current.revision = revision;
        self.current.first_sample = first_sample;
        self.current.outcome_flags = flags;
        self.store(self.current);
    }

    /// Overwrite the record with `watermark`. Wait-free: two counter stores and six word stores.
    #[inline]
    fn store(&self, watermark: PlanWatermark) {
        let record = &*self.record;
        let opening = record.sequence_lock.load(Ordering::Relaxed).wrapping_add(1);
        record.sequence_lock.store(opening, Ordering::Relaxed);
        // Keeps the word stores below from being observed before the counter went odd.
        fence(Ordering::Release);
        record.revision.store(watermark.revision, Ordering::Relaxed);
        record
            .first_sample
            .store(watermark.first_sample, Ordering::Relaxed);
        record
            .flags
            .store(watermark.outcome_flags, Ordering::Relaxed);
        record.exact.store(watermark.exact, Ordering::Relaxed);
        record
            .transition_fallback
            .store(watermark.transition_fallback, Ordering::Relaxed);
        record
            .superseded
            .store(watermark.superseded, Ordering::Relaxed);
        record
            .sequence_lock
            .store(opening.wrapping_add(1), Ordering::Release);
    }
}
// REALTIME_POLICY_END

impl PlanWatermarkReader {
    /// The newest whole watermark, or [`WatermarkBusy`] if every one of the bounded attempts
    /// landed inside a publication. Never a partial record: the words are taken between two equal
    /// even counter reads, so what comes back is exactly one publication.
    pub fn read(&self) -> Result<PlanWatermark, WatermarkBusy> {
        let record = &*self.record;
        for _ in 0..MAXIMUM_READ_ATTEMPTS {
            let before = record.sequence_lock.load(Ordering::Acquire);
            if !before.is_multiple_of(2) {
                continue;
            }
            let watermark = PlanWatermark {
                revision: record.revision.load(Ordering::Relaxed),
                first_sample: record.first_sample.load(Ordering::Relaxed),
                outcome_flags: record.flags.load(Ordering::Relaxed),
                exact: record.exact.load(Ordering::Relaxed),
                transition_fallback: record.transition_fallback.load(Ordering::Relaxed),
                superseded: record.superseded.load(Ordering::Relaxed),
            };
            fence(Ordering::Acquire);
            if record.sequence_lock.load(Ordering::Relaxed) == before {
                return Ok(watermark);
            }
        }
        Err(WatermarkBusy)
    }
}

#[cfg(test)]
mod tests {
    use super::{PlanWatermark, WatermarkBusy, plan_watermark};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{Duration, Instant};

    /// Every word derived from one counter, so a mixture of two publications is detectable.
    fn record(counter: u64) -> PlanWatermark {
        PlanWatermark {
            revision: counter,
            first_sample: counter.wrapping_mul(128),
            outcome_flags: counter % 7,
            exact: counter.wrapping_mul(3),
            transition_fallback: counter ^ 0x5555_5555_5555_5555,
            superseded: !counter,
        }
    }

    struct DoneOnDrop(Arc<AtomicBool>);

    impl Drop for DoneOnDrop {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }

    /// #1314 gate 5: a writer publishing in a tight loop never lets a reader see two
    /// publications' words in one record; a busy result is the only alternative to a whole one.
    #[test]
    fn a_watermark_read_is_one_whole_publication_or_busy() {
        const PUBLICATIONS: u64 = 1_000_000;
        const STALL_DEADLINE: Duration = Duration::from_secs(10);
        let (writer, reader) = plan_watermark(0);
        let barrier = Arc::new(Barrier::new(2));
        let writer_barrier = Arc::clone(&barrier);
        let done = Arc::new(AtomicBool::new(false));
        let writer_done = Arc::clone(&done);
        let publisher = thread::spawn(move || {
            let _done = DoneOnDrop(writer_done);
            writer_barrier.wait();
            for counter in 1..=PUBLICATIONS {
                writer.store(record(counter));
            }
        });
        barrier.wait();
        let (mut whole, mut busy, mut torn, mut newest) = (0_u64, 0_u64, 0_u64, 0_u64);
        let started = Instant::now();
        while !done.load(Ordering::Acquire) {
            match reader.read() {
                Ok(observed) => {
                    whole += 1;
                    if observed.revision != 0 && observed != record(observed.revision) {
                        torn += 1;
                    }
                    newest = newest.max(observed.revision);
                }
                Err(WatermarkBusy) => busy += 1,
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
