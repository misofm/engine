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

use engine::realtime::{observation_slot, ObservationReader, ObservationWindow};

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

#[derive(Default)]
struct ReadAccounting {
    reads: u64,
    torn: u64,
    regressions: u64,
    advances: u64,
    newest: u64,
    missed_total: u64,
}

fn account_read(
    reader: &ObservationReader,
    observed: ObservationWindow,
    accounting: &mut ReadAccounting,
) {
    accounting.reads += 1;
    if !consistent(observed) {
        accounting.torn += 1;
    }
    if observed.sequence < accounting.newest {
        accounting.regressions += 1;
    }
    if observed.sequence > accounting.newest {
        accounting.missed_total += reader.missed_windows(observed.sequence);
        accounting.advances += 1;
        accounting.newest = observed.sequence;
        reader.acknowledge(observed.sequence);
    }
}

#[test]
fn repeat_reads_and_final_gap_have_exact_accounting() {
    let (publisher, reader) = observation_slot();
    let mut accounting = ReadAccounting::default();

    publisher.publish(window(1));
    for _ in 0..3 {
        account_read(
            &reader,
            reader.read().expect("repeated first window"),
            &mut accounting,
        );
    }
    assert_eq!(accounting.advances, 1);
    assert_eq!(accounting.missed_total, 0);

    publisher.publish(window(4));
    account_read(
        &reader,
        reader.read().expect("final gapped window"),
        &mut accounting,
    );
    assert_eq!(accounting.advances, 2);
    assert_eq!(accounting.missed_total, 2);
    assert_eq!(
        accounting.advances + accounting.missed_total,
        accounting.newest
    );
    assert_eq!(accounting.newest, 4);
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
    let mut accounting = ReadAccounting::default();
    while !done.load(Ordering::Acquire) {
        if let Some(observed) = reader.read() {
            account_read(&reader, observed, &mut accounting);
        }
    }
    if let Some(observed) = reader.read() {
        account_read(&reader, observed, &mut accounting);
    }
    let writer_view = writer.join().expect("writer");

    assert_eq!(
        accounting.torn, 0,
        "{} of {} reads were torn",
        accounting.torn, accounting.reads
    );
    assert_eq!(
        accounting.regressions, 0,
        "a conflating cell never goes backwards"
    );
    assert!(accounting.reads > 0, "the reader observed nothing at all");
    assert_eq!(
        accounting.newest, WINDOWS,
        "the reader ends on the newest window, not on a queued backlog"
    );
    assert!(
        accounting.advances + accounting.missed_total == accounting.newest,
        "every skipped publication is counted before acknowledgment"
    );
    assert!(
        writer_view <= WINDOWS,
        "the writer's view of the reader is bounded by what was published"
    );
    // No bound on `absent` here (issue #359 WP-2, §10): how often a read lands inside a
    // publication depends on how the scheduler interleaves the writer and reader threads on this
    // machine under this load, which is host/scheduler behavior, not a property of the seqlock.
    // A run with `33598887208` (docs-only main push) failed a "coarse livelock" ratio bound on
    // `absent` though the tested code had not changed. The seqlock's correctness claim -- whole
    // windows only, non-decreasing sequences with a counted gap, a wait-free writer -- does not
    // depend on how often the reader wins the race, and every assertion above already states that
    // claim exactly; `absent` itself is not part of it.
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
