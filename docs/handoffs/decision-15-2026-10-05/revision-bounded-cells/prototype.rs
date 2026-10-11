#![allow(unreachable_pub, dead_code, missing_docs, clippy::all, clippy::pedantic, unused)]
//! THROWAWAY PROTOTYPE (design check for the revision-bounded cell). Not for the repository.
//!
//! A revision gate (one word: bit 63 = ANNOUNCED, bits 0..63 = revision) and a three-slot
//! latest-target cell whose reader applies, per cell, the newest value with revision <= the
//! block's snapshot S.

#[cfg(not(loom))]
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
#[cfg(loom)]
use loom::sync::atomic::{AtomicU32, AtomicU64, Ordering};

fn mutation(name: &str) -> bool {
    option_env!("CELL_MUTATION") == Some(name)
}

pub const ANNOUNCED: u64 = 1 << 63;
pub const REVISION_MAX: u64 = ANNOUNCED - 1;
const UNWRITTEN: u64 = u64::MAX;
const SLOTS: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveSnapshot(pub u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveStamp {
    pub revision: u64,
    pub pinned: Option<u64>,
}

pub struct RevisionGate {
    word: AtomicU64,
}

#[derive(Debug, Default)]
pub struct GatePin {
    pub pinned: Option<u64>,
}

impl RevisionGate {
    pub fn new(initial: u64) -> Self {
        Self {
            word: AtomicU64::new(initial),
        }
    }
    /// Writer: the last write of a commit.
    pub fn publish(&self, pin: &mut GatePin, revision: u64) {
        assert!(revision <= REVISION_MAX);
        if mutation("writer_store_not_swap") {
            self.word.store(revision, Ordering::Release);
            return;
        }
        let order = if mutation("gate_relaxed") {
            Ordering::Relaxed
        } else {
            Ordering::AcqRel
        };
        let old = self.word.swap(revision, order);
        if old & ANNOUNCED != 0 {
            pin.pinned = Some(old & REVISION_MAX);
        } else if mutation("writer_forgets_pin") {
            pin.pinned = None;
        }
    }
    /// Reader: once per block.
    pub fn snapshot(&self) -> LiveSnapshot {
        if mutation("reader_load_not_rmw") {
            return LiveSnapshot(self.word.load(Ordering::Acquire) & REVISION_MAX);
        }
        let order = if mutation("gate_relaxed") {
            Ordering::Relaxed
        } else {
            Ordering::AcqRel
        };
        LiveSnapshot(self.word.fetch_or(ANNOUNCED, order) & REVISION_MAX)
    }
}

impl GatePin {
    pub fn stamp(&self, revision: u64) -> LiveStamp {
        LiveStamp {
            revision,
            pinned: self.pinned,
        }
    }
}

struct Slot<const N: usize> {
    words: [AtomicU32; N],
    revision: AtomicU64,
    sequence: AtomicU64,
}

pub struct LatestCell<const N: usize> {
    slots: [Slot<N>; SLOTS],
}

impl<const N: usize> LatestCell<N> {
    pub fn new() -> Self {
        Self {
            slots: core::array::from_fn(|_| Slot {
                words: core::array::from_fn(|_| AtomicU32::new(0)),
                revision: AtomicU64::new(UNWRITTEN),
                sequence: AtomicU64::new(0),
            }),
        }
    }
}

pub struct DirtyWord {
    bits: AtomicU64,
}

impl DirtyWord {
    pub fn new() -> Self {
        Self {
            bits: AtomicU64::new(0),
        }
    }
    pub fn mark(&self, bit: u32) {
        self.bits.fetch_or(1 << bit, Ordering::Release);
    }
    /// Reader: the bits set since the last take, or 0. One Relaxed load when idle.
    pub fn take(&self) -> u64 {
        if self.bits.load(Ordering::Relaxed) == 0 {
            return 0;
        }
        let order = if mutation("dirty_swap_relaxed") {
            Ordering::Relaxed
        } else {
            Ordering::Acquire
        };
        self.bits.swap(0, order)
    }
}

/// Control-thread half of one cell: a private mirror of the slot tags.
pub struct CellWriter {
    revisions: [u64; SLOTS],
    next_sequence: u64,
}

impl CellWriter {
    pub fn new() -> Self {
        Self {
            revisions: [UNWRITTEN; SLOTS],
            next_sequence: 1,
        }
    }

    fn newest_at_or_below(&self, bound: u64) -> Option<usize> {
        let mut best: Option<usize> = None;
        for slot in 0..SLOTS {
            let revision = self.revisions[slot];
            if revision != UNWRITTEN
                && revision <= bound
                && best.is_none_or(|b| revision > self.revisions[b])
            {
                best = Some(slot);
            }
        }
        best
    }

    fn target(&self, stamp: LiveStamp) -> usize {
        if !mutation("no_rewrite_in_place")
            && let Some(slot) = (0..SLOTS).find(|&s| self.revisions[s] == stamp.revision)
        {
            return slot;
        }
        // A: newest written in an earlier (hence published) revision.
        let a = stamp
            .revision
            .checked_sub(1)
            .and_then(|bound| self.newest_at_or_below(bound));
        // B: newest at or below the reader's possible older snapshot.
        let b = if mutation("protect_only_a") {
            None
        } else {
            stamp.pinned.and_then(|pin| self.newest_at_or_below(pin))
        };
        (0..SLOTS)
            .find(|&s| Some(s) != a && Some(s) != b && self.revisions[s] != stamp.revision)
            .expect("three slots leave one free")
    }

    pub fn write<const N: usize>(
        &mut self,
        cell: &LatestCell<N>,
        dirty: &DirtyWord,
        bit: u32,
        stamp: LiveStamp,
        words: [u32; N],
    ) {
        let slot = self.target(stamp);
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        if mutation("dirty_before_slot") {
            dirty.mark(bit);
        }
        let target = &cell.slots[slot];
        if mutation("tag_first") {
            target.revision.store(stamp.revision, Ordering::Relaxed);
            target.sequence.store(sequence, Ordering::Relaxed);
        }
        for (word, value) in target.words.iter().zip(words) {
            word.store(value, Ordering::Relaxed);
        }
        if !mutation("tag_first") {
            target.sequence.store(sequence, Ordering::Relaxed);
            target.revision.store(stamp.revision, Ordering::Relaxed);
        }
        self.revisions[slot] = stamp.revision;
        if !mutation("dirty_before_slot") {
            dirty.mark(bit);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellRead<const N: usize> {
    Unchanged,
    Applied { words: [u32; N], sequence: u64, superseded: u64 },
}

/// Render-thread half of one cell.
pub struct CellReader {
    last_applied: u64,
}

impl CellReader {
    pub fn new() -> Self {
        Self { last_applied: 0 }
    }
    pub fn last_applied(&self) -> u64 {
        self.last_applied
    }

    /// Apply the newest value with revision <= S. Returns the read and whether a newer value
    /// (revision > S) was seen, which keeps the cell pending.
    pub fn read<const N: usize>(
        &mut self,
        cell: &LatestCell<N>,
        snapshot: LiveSnapshot,
    ) -> (CellRead<N>, bool) {
        let mut chosen: Option<(usize, u64)> = None;
        let mut newer = false;
        for (index, slot) in cell.slots.iter().enumerate() {
            let revision = slot.revision.load(Ordering::Relaxed);
            if revision == UNWRITTEN {
                continue;
            }
            let eligible = mutation("reader_ignores_snapshot") || revision <= snapshot.0;
            if eligible {
                if chosen.is_none_or(|(_, best)| revision > best) {
                    chosen = Some((index, revision));
                }
            } else {
                newer = true;
            }
        }
        if mutation("reader_never_defers") {
            newer = false;
        }
        let Some((index, _)) = chosen else {
            return (CellRead::Unchanged, newer);
        };
        let slot = &cell.slots[index];
        let sequence = slot.sequence.load(Ordering::Relaxed);
        if sequence <= self.last_applied {
            return (CellRead::Unchanged, newer);
        }
        let words = core::array::from_fn(|i| slot.words[i].load(Ordering::Relaxed));
        let superseded = sequence.saturating_sub(self.last_applied).saturating_sub(1);
        self.last_applied = sequence;
        (
            CellRead::Applied {
                words,
                sequence,
                superseded,
            },
            newer,
        )
    }

    /// Carry only, writer quiescent: the newest written value if unread. Changes nothing.
    pub fn peek_unread<const N: usize>(&self, cell: &LatestCell<N>) -> Option<([u32; N], u64)> {
        let mut chosen: Option<(usize, u64)> = None;
        for (index, slot) in cell.slots.iter().enumerate() {
            let sequence = slot.sequence.load(Ordering::Relaxed);
            if slot.revision.load(Ordering::Relaxed) != UNWRITTEN
                && chosen.is_none_or(|(_, best)| sequence > best)
            {
                chosen = Some((index, sequence));
            }
        }
        let (index, sequence) = chosen?;
        (sequence > self.last_applied).then(|| {
            let slot = &cell.slots[index];
            (
                core::array::from_fn(|i| slot.words[i].load(Ordering::Relaxed)),
                sequence,
            )
        })
    }
}

/// A stage of render-side cells sharing one dirty word, with the render-local deferred mask.
pub struct StageReader {
    pub deferred: u64,
}

impl StageReader {
    pub fn pending(&mut self, dirty: &DirtyWord) -> u64 {
        core::mem::take(&mut self.deferred) | dirty.take()
    }
}

/// Value words for revision `r`, value index `k`: torn or misattributed reads are detectable.
pub fn value_words(revision: u64, k: u32) -> [u32; 2] {
    let tag = (revision as u32) << 8 | k;
    [tag, !tag]
}

pub fn decode(words: [u32; 2]) -> (u64, u32) {
    assert_eq!(words[0], !words[1], "torn words {words:?}");
    (u64::from(words[0] >> 8), words[0] & 0xff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_is_pure_and_read_counts() {
        let cell = LatestCell::<2>::new();
        let dirty = DirtyWord::new();
        let gate = RevisionGate::new(0);
        let mut pin = GatePin::default();
        let mut writer = CellWriter::new();
        let mut reader = CellReader::new();
        for r in 1..=3 {
            writer.write(&cell, &dirty, 0, pin.stamp(r), value_words(r, 0));
            gate.publish(&mut pin, r);
        }
        let first = reader.peek_unread(&cell).expect("unread");
        assert_eq!(first, (value_words(3, 0), 3));
        assert_eq!(reader.peek_unread(&cell), Some(first));
        let snapshot = gate.snapshot();
        assert_eq!(snapshot, LiveSnapshot(3));
        let (read, newer) = reader.read(&cell, snapshot);
        assert!(!newer);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(3, 0),
                sequence: 3,
                superseded: 2
            }
        );
        assert_eq!(reader.peek_unread(&cell), None);
    }

    #[test]
    fn rewrite_in_one_revision_applies_the_last() {
        let cell = LatestCell::<2>::new();
        let dirty = DirtyWord::new();
        let gate = RevisionGate::new(0);
        let mut pin = GatePin::default();
        let mut writer = CellWriter::new();
        let mut reader = CellReader::new();
        writer.write(&cell, &dirty, 0, pin.stamp(1), value_words(1, 0));
        writer.write(&cell, &dirty, 0, pin.stamp(1), value_words(1, 1));
        gate.publish(&mut pin, 1);
        let (read, _) = reader.read(&cell, gate.snapshot());
        match read {
            CellRead::Applied {
                words, superseded, ..
            } => {
                assert_eq!(decode(words), (1, 1), "the second write of revision 1 applies");
                assert_eq!(superseded, 1);
            }
            CellRead::Unchanged => panic!("nothing applied"),
        }
    }

    /// A paused reader: 10_000 commits, bounded memory, then one block applies the newest.
    #[test]
    fn a_paused_reader_is_lapped_without_failure() {
        let cell = LatestCell::<2>::new();
        let dirty = DirtyWord::new();
        let gate = RevisionGate::new(0);
        let mut pin = GatePin::default();
        let mut writer = CellWriter::new();
        let mut reader = CellReader::new();
        writer.write(&cell, &dirty, 0, pin.stamp(1), value_words(1, 0));
        gate.publish(&mut pin, 1);
        let s = gate.snapshot();
        let (read, _) = reader.read(&cell, s);
        assert!(matches!(read, CellRead::Applied { .. }));
        for r in 2..=10_000 {
            writer.write(&cell, &dirty, 0, pin.stamp(r), value_words(r, 0));
            gate.publish(&mut pin, r);
        }
        let mut stage = StageReader { deferred: 0 };
        assert_eq!(stage.pending(&dirty), 1);
        let (read, newer) = reader.read(&cell, gate.snapshot());
        assert!(!newer);
        match read {
            CellRead::Applied {
                words, superseded, ..
            } => {
                assert_eq!(decode(words), (10_000, 0));
                assert_eq!(superseded, 9_998);
            }
            CellRead::Unchanged => panic!(),
        }
    }
}

#[cfg(all(test, loom))]
mod loom_tests {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;

    struct Shared {
        gate: RevisionGate,
        dirty: DirtyWord,
        cells: [LatestCell<2>; 2],
    }

    fn shared() -> Arc<Shared> {
        Arc::new(Shared {
            gate: RevisionGate::new(0),
            dirty: DirtyWord::new(),
            cells: [LatestCell::new(), LatestCell::new()],
        })
    }

    struct Render {
        stage: StageReader,
        readers: [CellReader; 2],
        applied: [Option<u64>; 2],
        superseded: u64,
        applications: u64,
    }

    impl Render {
        fn new() -> Self {
            Self {
                stage: StageReader { deferred: 0 },
                readers: [CellReader::new(), CellReader::new()],
                applied: [None, None],
                superseded: 0,
                applications: 0,
            }
        }
        /// One block over `cells` cells: returns S and, per cell, the revision it holds after.
        fn block(&mut self, shared: &Shared, cells: usize) -> u64 {
            let (snapshot, pending) = if mutation("dirty_before_snapshot") {
                let pending = self.stage.pending(&shared.dirty);
                (shared.gate.snapshot(), pending)
            } else {
                let snapshot = shared.gate.snapshot();
                (snapshot, self.stage.pending(&shared.dirty))
            };
            for cell in 0..cells {
                if pending & (1 << cell) == 0 {
                    continue;
                }
                let (read, newer) = self.readers[cell].read(&shared.cells[cell], snapshot);
                if newer {
                    self.stage.deferred |= 1 << cell;
                }
                if let CellRead::Applied {
                    words, superseded, ..
                } = read
                {
                    let (revision, _) = decode(words);
                    assert!(revision <= snapshot.0, "applied {revision} above S {}", snapshot.0);
                    self.applied[cell] = Some(revision);
                    self.superseded += superseded;
                    self.applications += 1;
                }
            }
            snapshot.0
        }
    }

    /// L1: one block never applies part of a transaction. Two cells, two transactions that each
    /// write both, racing one block.
    #[test]
    fn spsc_loom_cells_one_block_never_tears() {
        loom::model(|| {
            let shared = shared();
            let writer_shared = Arc::clone(&shared);
            let writer = thread::spawn(move || {
                let mut pin = GatePin::default();
                let mut writers = [CellWriter::new(), CellWriter::new()];
                for revision in 1..=2 {
                    for (cell, writer) in writers.iter_mut().enumerate() {
                        writer.write(
                            &writer_shared.cells[cell],
                            &writer_shared.dirty,
                            cell as u32,
                            pin.stamp(revision),
                            value_words(revision, 0),
                        );
                    }
                    writer_shared.gate.publish(&mut pin, revision);
                }
            });
            let mut render = Render::new();
            let s = render.block(&shared, 2);
            let expected = (s != 0).then_some(s);
            assert_eq!(render.applied, [expected, expected], "S = {s}");
            writer.join().unwrap();
            let s = render.block(&shared, 2);
            assert_eq!(s, 2);
            assert_eq!(render.applied, [Some(2), Some(2)]);
            assert_eq!(render.superseded + render.applications, 4, "count is exact");
        });
    }

    /// L2: a reader pinned at an old snapshot while the writer laps it three times still reads
    /// exactly that snapshot's value; the writer never fails or waits.
    #[test]
    fn spsc_loom_cells_a_pinned_reader_survives_a_lapping_writer() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = option_env!("CELL_UNBOUNDED").map_or(Some(3), |_| None);
        builder.check(|| {
            let shared = shared();
            let mut pin = GatePin::default();
            let mut writer = CellWriter::new();
            writer.write(
                &shared.cells[0],
                &shared.dirty,
                0,
                pin.stamp(1),
                value_words(1, 0),
            );
            shared.gate.publish(&mut pin, 1);
            let writer_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                for revision in 2..=4 {
                    writer.write(
                        &writer_shared.cells[0],
                        &writer_shared.dirty,
                        0,
                        pin.stamp(revision),
                        value_words(revision, 0),
                    );
                    writer_shared.gate.publish(&mut pin, revision);
                }
            });
            let mut render = Render::new();
            let s = render.block(&shared, 1);
            assert_eq!(render.applied[0], Some(s), "the newest value at or below S applies");
            control.join().unwrap();
            render.block(&shared, 1);
            assert_eq!(render.applied[0], Some(4));
            assert_eq!(render.superseded + render.applications, 4);
        });
    }

    /// L3: a value committed while a block drains is never lost: its dirty bit either stays set
    /// or the block that took it sees its revision above S and defers it.
    #[test]
    fn spsc_loom_cells_a_dirty_bit_is_never_lost() {
        loom::model(|| {
            let shared = shared();
            let writer_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut pin = GatePin::default();
                let mut writer = CellWriter::new();
                for revision in 1..=2 {
                    writer.write(
                        &writer_shared.cells[0],
                        &writer_shared.dirty,
                        0,
                        pin.stamp(revision),
                        value_words(revision, 0),
                    );
                    writer_shared.gate.publish(&mut pin, revision);
                }
            });
            let mut render = Render::new();
            for _ in 0..2 {
                let s = render.block(&shared, 1);
                let expected = (s != 0).then_some(s);
                assert_eq!(render.applied[0], expected);
            }
            control.join().unwrap();
            render.block(&shared, 1);
            assert_eq!(render.applied[0], Some(2), "the last committed value applies");
            assert_eq!(render.superseded + render.applications, 2);
        });
    }
}

#[cfg(all(test, loom))]
mod loom_probe {
    use loom::sync::Arc;
    use loom::sync::atomic::{AtomicU64, Ordering};
    #[test]
    fn spsc_loom_probe_store_then_rmw() {
        loom::model(|| {
            let x = Arc::new(AtomicU64::new(0));
            let y = Arc::clone(&x);
            let t = loom::thread::spawn(move || {
                y.store(1, Ordering::Release);
                y.store(2, Ordering::Release);
            });
            let first = x.fetch_or(1 << 63, Ordering::AcqRel);
            t.join().unwrap();
            let last = x.fetch_or(1 << 63, Ordering::AcqRel);
            assert_eq!(last & !(1 << 63), 2, "first {first:#x} last {last:#x}");
        });
    }
}
