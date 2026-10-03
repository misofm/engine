//! A render loop on this thread against a control producer on another (issue #1250).
//!
//! The live-control allocation gates render block after block while a second thread keeps pushing
//! records, and each block must apply at least one record, so that the audited render really runs
//! the drain-and-apply path. Two defects in hand-rolled versions of that loop turned a scheduling
//! accident into a CI job hung until its timeout:
//!
//! * **The wait read the producer's progress, not the queue.** It compared a count of successful
//!   pushes with a sample taken before the previous render. A push that render drained still moved
//!   the count, so the next block could start with every queue empty and fail "applied a record".
//! * **A failed assertion hung.** The producer spun until a flag the render loop set only after
//!   its last block, and `std::thread::scope` joins every thread before it propagates a panic, so
//!   the panic waited forever on a producer that never stopped, and the test harness, which had
//!   captured the panic's message, printed nothing but "has been running for over 60 seconds".
//!
//! [`render_while_producing`] waits on the queue state itself and stops the producer however the
//! render loop ends.

use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// How long the render loop waits for a queued record before it fails instead of hanging. A
/// producer that is running queues one in microseconds.
pub const QUEUED_DEADLINE: Duration = Duration::from_secs(10);

/// Sets its flag when dropped, so the producer thread stops however the render loop ends.
struct StopOnDrop<'a>(&'a AtomicBool);

impl Drop for StopOnDrop<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

/// Renders blocks `0..blocks` on this thread while a scoped thread keeps feeding `producers`.
///
/// The producer thread calls `produce` over the locked producers, again and again, until the
/// render loop ends; it takes the lock only for `produce`, so it pushes while this thread renders.
/// Before each `render(block)`, this thread waits until `queued`, over the same locked producers,
/// is true: some queue holds a record. Only a render drains a queue, so that record is still there
/// when the render starts, whatever the two threads' schedule.
///
/// # Panics
///
/// With the panic of `render` or `produce`, and never later than the panic: a render's panic
/// stops the producer thread, and a panic in `produce` poisons the lock, which the next wait
/// reports. Also when nothing is queued within [`QUEUED_DEADLINE`].
pub fn render_while_producing<P: Send>(
    blocks: usize,
    producers: P,
    mut produce: impl FnMut(&mut P) + Send,
    queued: impl Fn(&P) -> bool,
    mut render: impl FnMut(usize),
) {
    let producers = Mutex::new(producers);
    let stop = AtomicBool::new(false);
    std::thread::scope(|scope| {
        let producers = &producers;
        let stop = &stop;
        scope.spawn(move || {
            while !stop.load(Ordering::Acquire) {
                // A poisoned lock means the render loop panicked inside `queued`: stop.
                let Ok(mut locked) = producers.lock() else {
                    return;
                };
                produce(&mut locked);
                drop(locked);
                std::thread::yield_now();
            }
        });
        let _stop = StopOnDrop(stop);
        for block in 0..blocks {
            let deadline = Instant::now() + QUEUED_DEADLINE;
            while !queued(&producers.lock().expect("the producer thread panicked")) {
                assert!(
                    Instant::now() < deadline,
                    "block {block}: nothing was queued within {QUEUED_DEADLINE:?}"
                );
                std::thread::yield_now();
            }
            render(block);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::render_while_producing;
    use core::num::NonZeroUsize;
    use engine::realtime::{QueueGeneration, bounded_spsc};
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::mpsc;
    use std::time::Duration;

    const DEPTH: usize = 4;

    /// Runs `run` on its own thread and returns its panic message, or `Err` if it neither returned
    /// nor panicked within 30 s. A hung run is reported, not waited on: its threads are leaked.
    fn panic_of(run: impl FnOnce() + Send + 'static) -> Result<Option<String>, &'static str> {
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let message = catch_unwind(AssertUnwindSafe(run)).err().map(|payload| {
                payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| {
                        payload
                            .downcast_ref::<&str>()
                            .map(|text| (*text).to_owned())
                    })
                    .unwrap_or_default()
            });
            let _sent = sender.send(message);
        });
        receiver
            .recv_timeout(Duration::from_secs(30))
            .map_err(|_| "the run hung")
    }

    /// A producer whose first `idle` calls queue nothing, then one record per call while the
    /// queue has room, and a render that drains the queue whole.
    fn run_with_idle_producer(blocks: usize, idle: usize) -> Vec<usize> {
        let (producer, mut consumer) =
            bounded_spsc::<u32>(NonZeroUsize::new(DEPTH).expect("depth"), QueueGeneration(1))
                .expect("queue");
        let mut calls = 0_usize;
        let mut drained = Vec::with_capacity(blocks);
        render_while_producing(
            blocks,
            producer,
            |producer| {
                calls += 1;
                if calls > idle {
                    let _full = producer.try_push(7);
                } else {
                    // A producer that is busy elsewhere: it returns without queueing anything.
                    std::thread::sleep(Duration::from_millis(1));
                }
            },
            |producer| producer.available_capacity() < DEPTH,
            |_| {
                let available = consumer.available_at_entry();
                for _ in 0..available {
                    consumer.try_pop().expect("an available record");
                }
                drained.push(available);
            },
        );
        drained
    }

    /// Every render starts with a record queued, even when the producer's calls return without
    /// queueing one.
    ///
    /// Test value: red if a render starts on the producer's progress rather than on the queue
    /// state, as the #1221 and #1220 allocation gates did (block 0 would follow the first, idle
    /// call and drain nothing).
    #[test]
    fn every_render_starts_with_a_record_queued() {
        let drained = run_with_idle_producer(64, 20);
        assert_eq!(drained.len(), 64);
        if let Some(block) = drained.iter().position(|&records| records == 0) {
            panic!("block {block} started with nothing queued: {drained:?}");
        }
    }

    /// A render's panic ends the run with that panic; the producer thread stops.
    ///
    /// Test value: red (by this test's 30 s bound, not a hang) if the producer is stopped only
    /// after the last block, which leaves `std::thread::scope` waiting on it forever.
    #[test]
    fn a_render_panic_ends_the_run() {
        let message = panic_of(|| {
            render_while_producing(
                8,
                0_u64,
                |pushed| *pushed += 1,
                |_| true,
                |block| assert_ne!(block, 2, "render failed at block 2"),
            );
        });
        let message = message
            .expect("a render panic ends the run")
            .expect("panicked");
        assert!(
            message.contains("render failed at block 2"),
            "the render's own panic: {message}"
        );
    }

    /// A producer's panic ends the run with a panic, instead of a render loop waiting forever on a
    /// queue nothing feeds.
    ///
    /// Test value: red if the wait ignores the producer thread's death: only the deadline would
    /// then end the run, later than this test allows and with another message.
    #[test]
    fn a_producer_panic_ends_the_run() {
        let started = std::time::Instant::now();
        let message = panic_of(|| {
            render_while_producing(
                8,
                0_u64,
                |calls| {
                    *calls += 1;
                    assert!(*calls < 3, "produce failed");
                },
                |_| false,
                |_| {},
            );
        });
        let message = message
            .expect("a producer panic ends the run")
            .expect("panicked");
        assert!(
            message.contains("the producer thread panicked"),
            "the wait reports the producer: {message}"
        );
        assert!(
            started.elapsed() < super::QUEUED_DEADLINE / 2,
            "the poisoned lock, not the deadline, ended the run"
        );
    }
}
