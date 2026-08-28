//! Issue #143 E11: the conflating observation cell never tears and never backs up.
//!
//! Three properties, each stated as something that must hold for **every** observed window rather
//! than as an average over a run:
//!
//! 1. **Whole windows only.** A window is published as seven words behind an odd/even counter. A
//!    reader either sees one publication or retries; it never sees the left lane of window `n` with
//!    the right lane of window `n + 1`. The stress below makes every field a pure function of the
//!    sequence number, so a torn read is arithmetically detectable rather than merely unlikely.
//! 2. **Non-decreasing sequences, and a counted gap.** A conflating cell drops by design. What it
//!    may not do is go backwards, and what it must do is let the reader *count* what it missed.
//! 3. **A wait-free writer.** The writer's loop is bounded by the window count and nothing else:
//!    it never waits for the reader, so a reader that stops entirely cannot slow it down. The
//!    stalled-reader case measures exactly that.

#![allow(missing_docs)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use miso_engine_core::realtime::{ObservationWindow, observation_slot};

/// One million windows, which is the eval's number.
const WINDOWS: u64 = 1_000_000;

/// Every field derived from the sequence, so any cross-window mixture is detectable.
fn window(sequence: u64) -> ObservationWindow {
    ObservationWindow {
        first_sample: sequence.wrapping_mul(128),
        end_sample: sequence.wrapping_mul(128) + 128,
        sequence,
        blocks: (sequence % 251) as u32 + 1,
        left: sequence as f32,
        right: -(sequence as f32),
    }
}

fn consistent(observed: ObservationWindow) -> bool {
    observed == window(observed.sequence)
}

#[test]
fn a_million_windows_are_read_whole_and_in_order() {
    let (publisher, reader) = observation_slot();
    let barrier = Arc::new(Barrier::new(2));
    let writer_barrier = Arc::clone(&barrier);
    let done = Arc::new(AtomicBool::new(false));
    let writer_done = Arc::clone(&done);

    let writer = thread::spawn(move || {
        writer_barrier.wait();
        for sequence in 1..=WINDOWS {
            publisher.publish(window(sequence));
        }
        writer_done.store(true, Ordering::Release);
        publisher.consumed_sequence()
    });

    barrier.wait();
    let mut reads = 0_u64;
    let mut torn = 0_u64;
    let mut regressions = 0_u64;
    let mut absent = 0_u64;
    let mut newest = 0_u64;
    let mut missed_total = 0_u64;
    while !done.load(Ordering::Acquire) {
        match reader.read() {
            Some(observed) => {
                reads += 1;
                if !consistent(observed) {
                    torn += 1;
                }
                if observed.sequence < newest {
                    regressions += 1;
                }
                if observed.sequence > newest {
                    missed_total += reader.missed_windows(observed.sequence);
                    newest = observed.sequence;
                    reader.acknowledge(observed.sequence);
                }
            }
            None => absent += 1,
        }
    }
    if let Some(observed) = reader.read() {
        reads += 1;
        if !consistent(observed) {
            torn += 1;
        }
        if observed.sequence < newest {
            regressions += 1;
        }
        newest = newest.max(observed.sequence);
        reader.acknowledge(observed.sequence);
    }
    let writer_view = writer.join().expect("writer");

    assert_eq!(torn, 0, "{torn} of {reads} reads were torn");
    assert_eq!(regressions, 0, "a conflating cell never goes backwards");
    assert!(reads > 0, "the reader observed nothing at all");
    assert_eq!(
        newest, WINDOWS,
        "the reader ends on the newest window, not on a queued backlog"
    );
    assert!(
        reads <= WINDOWS,
        "a conflating cell cannot produce more reads than writes"
    );
    assert!(
        missed_total > 0 || reads == WINDOWS,
        "either the reader kept up exactly, or the windows it missed were counted"
    );
    assert!(
        writer_view <= WINDOWS,
        "the writer's view of the reader is bounded by what was published"
    );
    // A coarse livelock bound, not a jitter budget. How often a read lands inside a publication
    // depends on how the scheduler interleaves two threads on this machine under this load, so a
    // tight ratio here would be measuring the host rather than the primitive. What must not happen
    // is the reader spending most of its time giving up, which is what a broken retry loop looks
    // like; an order of magnitude either way is still decisively inside that.
    assert!(
        absent <= reads.saturating_mul(10).saturating_add(1_000),
        "reads gave up far more often than they succeeded: {absent} absent against {reads} whole"
    );
}

/// A reader that stops entirely resumes on the newest window plus an exact gap, never on a stale
/// queue head.
#[test]
fn a_stalled_reader_resumes_on_the_newest_window_with_a_counted_gap() {
    let (publisher, reader) = observation_slot();
    assert_eq!(reader.read(), None, "nothing published is absent, not zero");

    publisher.publish(window(1));
    let first = reader.read().expect("first window");
    assert_eq!(first.sequence, 1);
    assert_eq!(reader.missed_windows(1), 0);
    reader.acknowledge(1);
    assert_eq!(publisher.consumed_sequence(), 1);

    for sequence in 2..=1_001 {
        publisher.publish(window(sequence));
    }
    let resumed = reader.read().expect("resumed window");
    assert_eq!(resumed.sequence, 1_001, "latest wins, never a queue head");
    assert!(consistent(resumed));
    assert_eq!(
        reader.missed_windows(resumed.sequence),
        999,
        "windows 2..=1000 were overwritten, and the count says exactly that"
    );
    reader.acknowledge(resumed.sequence);
    assert_eq!(reader.missed_windows(resumed.sequence), 0);

    reader.acknowledge(5);
    assert_eq!(reader.consumed_sequence(), 1_001);
}

/// The published words are exact: `-0.0` survives, and so does a subnormal.
#[test]
fn published_words_are_bit_exact() {
    let (publisher, reader) = observation_slot();
    let exact = ObservationWindow {
        first_sample: u64::MAX - 1,
        end_sample: u64::MAX,
        sequence: 7,
        blocks: u32::MAX,
        left: -0.0,
        right: f32::from_bits(1),
    };
    publisher.publish(exact);
    let observed = reader.read().expect("window");
    assert_eq!(observed.left.to_bits(), (-0.0_f32).to_bits());
    assert_eq!(observed.right.to_bits(), 1);
    assert_eq!(observed.first_sample, u64::MAX - 1);
    assert_eq!(observed.blocks, u32::MAX);
}
