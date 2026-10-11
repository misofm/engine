//! The revision gate and the revision-bounded latest-target cell (decision 15, D15-2 and D15-17;
//! issue #1432).
//!
//! # What this module guarantees
//!
//! One control thread writes live values into [`LatestCell`]s and then publishes the revision of
//! the transaction that wrote them through a [`RevisionGate`]. Render takes one
//! [`LiveSnapshot`] `S` of the gate per block, and every read of that block applies, per cell, the
//! newest value whose revision is `<= S`. It never applies a value newer than `S`, never a torn
//! value, never a value twice, and never skips one. So every value of one revision takes effect
//! in one block. A write cannot fail and never waits, however far control laps render.
//!
//! # The two operations that carry the proof
//!
//! The gate word is `ANNOUNCED | revision`: bit 63 is render's announcement, bits 0-62 are the
//! revision. Render's `snapshot` is one `fetch_or(ANNOUNCED, AcqRel)`: the same atomic operation
//! reads `S` and records that a reader holds it. Control's `publish` is one `swap(revision,
//! AcqRel)`: it clears the announcement and, if the old word carried one, learns the snapshot
//! (`pinned`) render may still be reading. A cell write then protects two of its three slots from
//! reuse: `A`, the newest published value, and `B`, the newest value at or below the pin. Those
//! two slots hold every value a reader can still need, so the third slot is always free to
//! overwrite. The writer needs no knowledge of render's progress beyond the pin, and render needs
//! no retry.
//!
//! **Every write to a gate word is a read-modify-write**, and every read of it masks
//! `ANNOUNCED`. A plain store would end the release sequences the visibility argument uses. The
//! same holds for the dirty word: it is written only by `fetch_or` and `swap`.
//!
//! # Why the loom models here do not lean on loom's imprecision
//!
//! Loom 0.7.2 lets a plain store that races a read-modify-write on one location be reordered
//! before the read-modify-write's write although the read-modify-write read an older value; C++20
//! forbids that. On a location written only by read-modify-writes loom is exact. So every model in
//! this file keeps each atomic location in one of two classes: written only by read-modify-writes
//! (the gate word, the dirty word, the model's mailbox word, all initialised by construction) or
//! written only by plain stores (a slot's words, revision and sequence). No model races a plain
//! store against a read-modify-write on one location.
//!
//! # Realtime
//!
//! Safe Rust, atomics only. Construction allocates and runs on the control thread.
//! `snapshot`, `publish`, `stamp`, a write, a read and a peek never allocate, lock or make a
//! syscall. Render pays one read-modify-write per plan per block (the snapshot), one `Relaxed`
//! load per idle stage, and for each pending cell three revision loads, one sequence load and `N`
//! word loads. On AArch64 without LSE (ARMv8.0) a read-modify-write is a load-linked or
//! store-conditional loop, which `docs/REALTIME_DEPENDENCY_POLICY.md` states.

#[cfg(not(loom))]
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
#[cfg(loom)]
use loom::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Bit 63 of the gate word: render has taken a snapshot since control's last `publish`.
const ANNOUNCED: u64 = 1 << 63;

/// The largest revision a gate holds (bits 0-62). The session store's revision ceiling is this
/// value (#1503).
pub const GATE_REVISION_MAX: u64 = ANNOUNCED - 1;

/// A slot's revision before its first write. It is above every legal revision and above every
/// snapshot, so a reader tests for it before it compares.
const UNWRITTEN: u64 = u64::MAX;

/// Every cell has exactly three slots: the newest published value `A`, the value the pin
/// protects `B`, and one slot control may overwrite.
const SLOTS: usize = 3;

/// The revision render reads in one block, taken by [`RevisionGate::snapshot`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveSnapshot(u64);

impl LiveSnapshot {
    /// Every revision a cell can hold. Its only user is a drain of a quiescent predecessor plan
    /// (#1312 D6): no production render path takes it.
    pub const ALL: Self = Self(GATE_REVISION_MAX);

    /// The revision this snapshot covers: reads apply values at or below it.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.0
    }
}

/// The revision a transaction commits as, and the snapshot render may still hold, as of the time
/// control stamps it. Only [`GatePin::stamp`] makes one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveStamp {
    revision: u64,
    pinned: Option<u64>,
}

impl LiveStamp {
    /// The revision of the transaction that writes with this stamp.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// The snapshot render may still be reading, learned from the gate at the last publication
    /// that saw an announcement. `None` when render has not announced since the gate was built.
    #[must_use]
    pub const fn pinned(self) -> Option<u64> {
        self.pinned
    }
}

/// One plan's revision gate: the word render snapshots and control publishes to.
///
/// A gate belongs to one plan for the plan's life (#1502), so it has no re-arm. It is embeddable:
/// it owns no `Arc`.
pub struct RevisionGate {
    word: AtomicU64,
}

impl RevisionGate {
    /// A gate at `initial`, with no announcement.
    ///
    /// # Panics
    ///
    /// Panics when `initial` is above [`GATE_REVISION_MAX`]. Construction is a control-thread
    /// operation.
    #[must_use]
    pub fn new(initial: u64) -> Self {
        assert!(
            initial <= GATE_REVISION_MAX,
            "gate revision {initial} is above the ceiling {GATE_REVISION_MAX}"
        );
        Self {
            word: AtomicU64::new(initial),
        }
    }

    /// The newest published revision, with the announcement masked.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.word.load(Ordering::Acquire) & GATE_REVISION_MAX
    }
}

// REALTIME_POLICY_BEGIN
impl RevisionGate {
    /// Render, once per block: read `S` and announce it in one read-modify-write.
    ///
    /// One `fetch_or(ANNOUNCED, AcqRel)`. The acquire half synchronizes with every earlier
    /// control `publish`, so every cell store of a revision `<= S` is visible. The release half
    /// ends the previous section's reads for the next `publish`. A load here would lose the
    /// announcement, and a `Relaxed` operation would lose both edges.
    #[must_use]
    pub fn snapshot(&self) -> LiveSnapshot {
        LiveSnapshot(self.word.fetch_or(ANNOUNCED, Ordering::AcqRel) & GATE_REVISION_MAX)
    }
}
// REALTIME_POLICY_END

/// Control's private half of one gate: the snapshot render may still hold and the last revision
/// published. One per gate; it moves between threads only by ownership transfer.
#[derive(Debug)]
pub struct GatePin {
    pinned: Option<u64>,
    last_published: u64,
}

impl GatePin {
    /// The pin of a gate built by [`RevisionGate::new`] with the same `initial`.
    ///
    /// # Panics
    ///
    /// Panics when `initial` is above [`GATE_REVISION_MAX`].
    #[must_use]
    pub fn new(initial: u64) -> Self {
        assert!(
            initial <= GATE_REVISION_MAX,
            "gate revision {initial} is above the ceiling {GATE_REVISION_MAX}"
        );
        Self {
            pinned: None,
            last_published: initial,
        }
    }

    /// The last revision published on the gate, or the gate's initial revision.
    #[must_use]
    pub const fn last_published(&self) -> u64 {
        self.last_published
    }

    /// Control, the last write of a commit: publish `revision` on `gate`.
    ///
    /// One `swap(revision, AcqRel)`. If the old word carried the announcement, the pin becomes
    /// the old revision (render's snapshot); otherwise it is unchanged, because render has
    /// announced nothing newer.
    ///
    /// In debug builds, a revision at or below the last published one, or above
    /// [`GATE_REVISION_MAX`], panics: it would end the ordering that protects render's slots.
    pub fn publish(&mut self, gate: &RevisionGate, revision: u64) {
        debug_assert!(
            revision <= GATE_REVISION_MAX,
            "published revision {revision} is above the ceiling {GATE_REVISION_MAX}"
        );
        debug_assert!(
            revision > self.last_published,
            "published revision {revision} is not above the last published {}",
            self.last_published
        );
        let old = gate.word.swap(revision, Ordering::AcqRel);
        if old & ANNOUNCED != 0 {
            self.pinned = Some(old & GATE_REVISION_MAX);
        }
        self.last_published = revision;
    }

    /// The stamp for a transaction that commits as `revision`.
    ///
    /// In debug builds, a revision at or below the last published one, or above
    /// [`GATE_REVISION_MAX`], panics: the in-place rule would rewrite a slot render may be
    /// reading.
    #[must_use]
    pub fn stamp(&self, revision: u64) -> LiveStamp {
        debug_assert!(
            revision <= GATE_REVISION_MAX,
            "stamped revision {revision} is above the ceiling {GATE_REVISION_MAX}"
        );
        debug_assert!(
            revision > self.last_published,
            "stamped revision {revision} is not above the last published {}",
            self.last_published
        );
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

/// One live value of `N` words (a target and its ramp are one unit), held in three slots.
///
/// Shared by one [`CellWriter`] (control) and one [`CellReader`] (render). Allocated on the
/// control thread.
pub struct LatestCell<const N: usize> {
    slots: [Slot<N>; SLOTS],
}

impl<const N: usize> LatestCell<N> {
    /// A cell with no written slot.
    #[must_use]
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

impl<const N: usize> Default for LatestCell<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// The word a stage shares among its cells: one bit per cell, set by a write and taken by the
/// stage's block drain.
pub struct DirtyWord {
    bits: AtomicU64,
}

impl DirtyWord {
    /// A word with no bit set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bits: AtomicU64::new(0),
        }
    }

    /// Control, after a write's slot stores.
    fn mark(&self, bit: u32) {
        self.bits.fetch_or(1 << bit, Ordering::Release);
    }
}

impl Default for DirtyWord {
    fn default() -> Self {
        Self::new()
    }
}

// REALTIME_POLICY_BEGIN
impl DirtyWord {
    /// Render: the bits set since the last take, or 0. An idle stage pays one `Relaxed` load.
    fn take(&self) -> u64 {
        if self.bits.load(Ordering::Relaxed) == 0 {
            return 0;
        }
        self.bits.swap(0, Ordering::Acquire)
    }
}

/// Render's private half of a stage's dirty word: the cells whose newest write was above the
/// snapshot that last read them.
#[derive(Debug, Default)]
pub struct DirtyReader {
    deferred: u64,
}

impl DirtyReader {
    /// A reader with no deferred cell.
    #[must_use]
    pub const fn new() -> Self {
        Self { deferred: 0 }
    }

    /// The cells to read this block: the deferred mask plus the bits set since the last take.
    ///
    /// Call it after the block's [`RevisionGate::snapshot`]. A take that came first would leave a
    /// write at or below `S`, whose bit was set between the take and the snapshot, waiting a
    /// block. The deferred mask is cleared: [`CellReader::read`] defers a cell again if its
    /// newest write is still above `S`.
    #[must_use]
    pub fn pending(&mut self, dirty: &DirtyWord) -> u64 {
        core::mem::take(&mut self.deferred) | dirty.take()
    }
}
// REALTIME_POLICY_END

/// Control's private half of one cell: a mirror of the three slot revisions and the next
/// sequence number. The mirror is exact, because control is the cell's only writer.
#[derive(Debug)]
pub struct CellWriter<const N: usize> {
    revisions: [u64; SLOTS],
    next_sequence: u64,
}

impl<const N: usize> CellWriter<N> {
    /// A writer for a cell with no written slot. Sequences start at 1.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            revisions: [UNWRITTEN; SLOTS],
            next_sequence: 1,
        }
    }

    /// The slot with the greatest mirrored revision at or below `bound`.
    fn newest_at_or_below(&self, bound: u64) -> Option<usize> {
        let mut best: Option<usize> = None;
        for (index, &revision) in self.revisions.iter().enumerate() {
            if revision == UNWRITTEN || revision > bound {
                continue;
            }
            match best {
                Some(held) if self.revisions[held] >= revision => {}
                _ => best = Some(index),
            }
        }
        best
    }

    /// The slot a write stamped `stamp` stores into.
    fn target(&self, stamp: LiveStamp) -> usize {
        // One revision occupies at most one slot, so a second write of it rewrites that slot.
        // Render reads only revisions at or below its snapshot, and this one is not published.
        if let Some(index) = self.revisions.iter().position(|&r| r == stamp.revision) {
            return index;
        }
        // A: the newest value of an earlier, hence published, revision.
        let a = stamp
            .revision
            .checked_sub(1)
            .and_then(|bound| self.newest_at_or_below(bound));
        // B: the newest value at or below the snapshot render may still hold.
        let b = stamp
            .pinned
            .and_then(|pinned| self.newest_at_or_below(pinned));
        (0..SLOTS)
            .find(|&index| Some(index) != a && Some(index) != b)
            .expect("three slots leave one free of A and B")
    }

    /// Control: store `words` as this transaction's value of the cell and mark its dirty bit.
    ///
    /// It cannot fail and never waits. The slot is chosen as D1 states: the slot already holding
    /// `stamp.revision`, else a slot that is neither `A` nor `B`. The stores are `Relaxed` and in
    /// any order; the dirty bit follows them with a `Release` `fetch_or`, and the revision's
    /// publication on the gate, which the caller makes after all of the transaction's writes,
    /// carries the visibility.
    ///
    /// In debug builds, a stamp whose revision is below the cell's newest written revision
    /// panics, and so does a `bit` of 64 or more.
    pub fn write(
        &mut self,
        cell: &LatestCell<N>,
        dirty: &DirtyWord,
        bit: u32,
        stamp: LiveStamp,
        words: [u32; N],
    ) {
        debug_assert!(bit < u64::BITS, "dirty bit {bit} is outside the word");
        debug_assert!(
            stamp.revision <= GATE_REVISION_MAX,
            "stamped revision {} is above the ceiling {GATE_REVISION_MAX}",
            stamp.revision
        );
        debug_assert!(
            self.revisions
                .iter()
                .all(|&r| r == UNWRITTEN || r <= stamp.revision),
            "stamp revision {} is below a revision already written to this cell",
            stamp.revision
        );
        let index = self.target(stamp);
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let slot = &cell.slots[index];
        for (word, value) in slot.words.iter().zip(words) {
            word.store(value, Ordering::Relaxed);
        }
        slot.sequence.store(sequence, Ordering::Relaxed);
        slot.revision.store(stamp.revision, Ordering::Relaxed);
        self.revisions[index] = stamp.revision;
        dirty.mark(bit);
    }
}

impl<const N: usize> Default for CellWriter<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// What one [`CellReader::read`] found.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellRead<const N: usize> {
    /// The newest write at or below the snapshot was applied already, or there is none.
    Unchanged,
    /// The newest write at or below the snapshot, not applied before.
    Applied {
        /// The value's words.
        words: [u32; N],
        /// The write's per-cell sequence number.
        sequence: u64,
        /// The writes this read replaced before any snapshot covered them: `sequence - previous
        /// applied sequence - 1`, saturating.
        superseded: u64,
    },
}

// REALTIME_POLICY_BEGIN
/// Render's private half of one cell: the sequence of the last applied write, 0 for none.
#[derive(Debug, Default)]
pub struct CellReader {
    last_applied: u64,
}

impl CellReader {
    /// A reader that has applied nothing.
    #[must_use]
    pub const fn new() -> Self {
        Self { last_applied: 0 }
    }

    /// The sequence of the last applied write, 0 for none.
    #[must_use]
    pub const fn last_applied(&self) -> u64 {
        self.last_applied
    }

    /// Render, after the block's snapshot and for each cell the stage's [`DirtyReader`] reports
    /// pending, in the stage's canonical order: apply the newest write at or below `snapshot`.
    ///
    /// Of the three slots it picks the greatest revision at or below `snapshot`, skipping
    /// unwritten ones, and loads the words of that slot only. A written slot above `snapshot`
    /// keeps the cell in `stage`'s deferred mask (as `bit`), so the next block reads it again.
    /// There is no retry and no spin.
    pub fn read<const N: usize>(
        &mut self,
        cell: &LatestCell<N>,
        snapshot: LiveSnapshot,
        stage: &mut DirtyReader,
        bit: u32,
    ) -> CellRead<N> {
        debug_assert!(bit < u64::BITS, "dirty bit {bit} is outside the word");
        let mut chosen: Option<(&Slot<N>, u64)> = None;
        let mut newer = false;
        for slot in &cell.slots {
            let revision = slot.revision.load(Ordering::Relaxed);
            if revision == UNWRITTEN {
                continue;
            }
            if revision > snapshot.0 {
                newer = true;
            } else if chosen.is_none_or(|(_, best)| revision > best) {
                chosen = Some((slot, revision));
            }
        }
        if newer {
            stage.deferred |= 1 << bit;
        }
        let Some((slot, _)) = chosen else {
            return CellRead::Unchanged;
        };
        let sequence = slot.sequence.load(Ordering::Relaxed);
        if sequence <= self.last_applied {
            return CellRead::Unchanged;
        }
        let mut words = [0; N];
        for (out, word) in words.iter_mut().zip(&slot.words) {
            *out = word.load(Ordering::Relaxed);
        }
        let superseded = sequence.saturating_sub(self.last_applied).saturating_sub(1);
        self.last_applied = sequence;
        CellRead::Applied {
            words,
            sequence,
            superseded,
        }
    }

    /// The plan-swap carry only: the written slot with the greatest sequence, if that sequence is
    /// above the last applied one. It changes nothing: no read-modify-write, and the last applied
    /// sequence stays as it was, as do the dirty word and the deferred mask.
    ///
    /// It is valid only once the writer is quiescent: no write to this cell can start before the
    /// peek ends (#1280 D2).
    #[must_use]
    pub fn peek_unread<const N: usize>(&self, cell: &LatestCell<N>) -> Option<([u32; N], u64)> {
        let mut chosen: Option<(&Slot<N>, u64)> = None;
        for slot in &cell.slots {
            if slot.revision.load(Ordering::Relaxed) == UNWRITTEN {
                continue;
            }
            let sequence = slot.sequence.load(Ordering::Relaxed);
            if chosen.is_none_or(|(_, best)| sequence > best) {
                chosen = Some((slot, sequence));
            }
        }
        let (slot, sequence) = chosen?;
        if sequence <= self.last_applied {
            return None;
        }
        let mut words = [0; N];
        for (out, word) in words.iter_mut().zip(&slot.words) {
            *out = word.load(Ordering::Relaxed);
        }
        Some((words, sequence))
    }
}
// REALTIME_POLICY_END

#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;

    /// The words a model writes for revision `r`, value `k`: a torn or misattributed read fails to
    /// decode.
    fn value_words(revision: u64, k: u32) -> [u32; 2] {
        let tag = u32::try_from(revision).expect("small") << 8 | k;
        [tag, !tag]
    }

    struct Fixture {
        cell: LatestCell<2>,
        dirty: DirtyWord,
        gate: RevisionGate,
        pin: GatePin,
        writer: CellWriter<2>,
        reader: CellReader,
        stage: DirtyReader,
    }

    impl Fixture {
        fn new() -> Self {
            Self {
                cell: LatestCell::new(),
                dirty: DirtyWord::new(),
                gate: RevisionGate::new(0),
                pin: GatePin::new(0),
                writer: CellWriter::new(),
                reader: CellReader::new(),
                stage: DirtyReader::new(),
            }
        }

        fn write(&mut self, revision: u64, k: u32) {
            self.writer.write(
                &self.cell,
                &self.dirty,
                0,
                self.pin.stamp(revision),
                value_words(revision, k),
            );
        }

        fn publish(&mut self, revision: u64) {
            self.pin.publish(&self.gate, revision);
        }

        /// One block: snapshot, then the stage's pending take, then the cell's read.
        fn block(&mut self) -> (LiveSnapshot, CellRead<2>) {
            let snapshot = self.gate.snapshot();
            let pending = self.stage.pending(&self.dirty);
            let read = if pending & 1 == 0 {
                CellRead::Unchanged
            } else {
                self.reader.read(&self.cell, snapshot, &mut self.stage, 0)
            };
            (snapshot, read)
        }
    }

    #[test]
    fn peek_changes_nothing_and_a_read_counts_the_replaced_writes() {
        let mut f = Fixture::new();
        for revision in 1..=3 {
            f.write(revision, 0);
            f.publish(revision);
        }
        let first = f.reader.peek_unread(&f.cell).expect("unread");
        assert_eq!(first, (value_words(3, 0), 3));
        assert_eq!(
            f.reader.peek_unread(&f.cell),
            Some(first),
            "a peek consumes nothing"
        );
        assert_eq!(f.reader.last_applied(), 0);
        let (snapshot, read) = f.block();
        assert_eq!(snapshot.revision(), 3);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(3, 0),
                sequence: 3,
                superseded: 2
            }
        );
        assert_eq!(f.reader.peek_unread(&f.cell), None);
    }

    #[test]
    fn two_writes_in_one_revision_apply_the_second() {
        let mut f = Fixture::new();
        f.write(1, 0);
        f.write(1, 1);
        f.publish(1);
        let (_, read) = f.block();
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(1, 1),
                sequence: 2,
                superseded: 1
            }
        );
    }

    #[test]
    fn a_lapped_reader_applies_the_last_commit_and_no_write_fails() {
        let mut f = Fixture::new();
        f.write(1, 0);
        f.publish(1);
        let (_, read) = f.block();
        assert!(matches!(read, CellRead::Applied { sequence: 1, .. }));
        for revision in 2..=10_000 {
            f.write(revision, 0);
            f.publish(revision);
        }
        let (snapshot, read) = f.block();
        assert_eq!(snapshot.revision(), 10_000);
        assert_eq!(
            read,
            CellRead::Applied {
                words: [10_000 << 8, !(10_000 << 8)],
                sequence: 10_000,
                superseded: 9_998
            }
        );
    }

    #[test]
    fn a_read_applies_only_what_the_snapshot_covers_and_defers_the_rest() {
        let mut f = Fixture::new();
        f.write(1, 0);
        f.publish(1);
        f.write(2, 0);
        // Revision 2 is written and its bit is set, but it is not published: the snapshot is 1.
        let (snapshot, read) = f.block();
        assert_eq!(snapshot.revision(), 1);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(1, 0),
                sequence: 1,
                superseded: 0
            }
        );
        let (_, read) = f.block();
        assert_eq!(
            read,
            CellRead::Unchanged,
            "revision 2 waits for its publication"
        );
        f.publish(2);
        let (snapshot, read) = f.block();
        assert_eq!(snapshot.revision(), 2);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(2, 0),
                sequence: 2,
                superseded: 0
            }
        );
    }

    #[test]
    fn the_all_snapshot_applies_the_newest_write() {
        let mut f = Fixture::new();
        f.write(1, 0);
        f.write(5, 0);
        let read = f.reader.read(&f.cell, LiveSnapshot::ALL, &mut f.stage, 0);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(5, 0),
                sequence: 2,
                superseded: 1
            }
        );
    }

    #[test]
    fn the_pin_follows_the_announcement_and_survives_unannounced_publications() {
        let mut f = Fixture::new();
        f.publish(1);
        assert_eq!(f.pin.stamp(2).pinned(), None, "no announcement yet");
        assert_eq!(f.gate.snapshot().revision(), 1);
        f.publish(2);
        assert_eq!(f.pin.stamp(3).pinned(), Some(1), "render holds snapshot 1");
        f.publish(3);
        f.publish(4);
        assert_eq!(
            f.pin.stamp(5).pinned(),
            Some(1),
            "no newer announcement, so the pin stays"
        );
        assert_eq!(f.gate.snapshot().revision(), 4);
        f.publish(5);
        assert_eq!(f.pin.stamp(6).pinned(), Some(4));
    }

    #[test]
    fn the_announcement_never_leaks_into_a_revision() {
        let f = Fixture::new();
        assert_eq!(f.gate.snapshot().revision(), 0);
        assert_eq!(
            f.gate.snapshot().revision(),
            0,
            "a second snapshot reads the same revision"
        );
        assert_eq!(f.gate.revision(), 0, "the gate read masks the announcement");
    }

    #[test]
    fn the_ceiling_is_the_largest_revision_a_gate_holds() {
        let mut pin = GatePin::new(0);
        let gate = RevisionGate::new(0);
        assert_eq!(gate.snapshot().revision(), 0);
        pin.publish(&gate, 0x7fff_ffff_ffff_ffff);
        assert_eq!(pin.pinned, Some(0), "the announcement was set");
        assert_eq!(gate.snapshot(), LiveSnapshot::ALL);
        assert_eq!(gate.revision(), 0x7fff_ffff_ffff_ffff);
        assert_eq!(GATE_REVISION_MAX, 0x7fff_ffff_ffff_ffff);
    }

    #[test]
    #[should_panic(expected = "above the ceiling")]
    fn a_gate_above_the_ceiling_is_refused() {
        let _ = RevisionGate::new(GATE_REVISION_MAX + 1);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "is not above the last published")]
    fn a_stamp_at_the_last_published_revision_panics() {
        let mut f = Fixture::new();
        f.publish(4);
        let _ = f.pin.stamp(4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "is not above the last published")]
    fn a_publication_at_the_last_published_revision_panics() {
        let mut f = Fixture::new();
        f.publish(4);
        f.publish(4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "is below a revision already written")]
    fn a_write_below_a_written_revision_panics() {
        let mut f = Fixture::new();
        f.write(3, 0);
        let stamp = f.pin.stamp(2);
        f.writer
            .write(&f.cell, &f.dirty, 0, stamp, value_words(2, 0));
    }
}

#[cfg(all(test, loom))]
mod loom_tests {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;

    /// The words a model writes for revision `r`, value `k`: a torn or misattributed read fails to
    /// decode.
    fn value_words(revision: u64, k: u32) -> [u32; 2] {
        let tag = u32::try_from(revision).expect("small") << 8 | k;
        [tag, !tag]
    }

    fn decode(words: [u32; 2]) -> (u64, u32) {
        assert_eq!(words[0], !words[1], "torn words {words:?}");
        (u64::from(words[0] >> 8), words[0] & 0xff)
    }

    /// One plan's shared state: its gate, one stage's dirty word and its cells. Every location is
    /// written by read-modify-writes only (gate, dirty word) or by plain stores only (a slot).
    struct Shared<const CELLS: usize> {
        gate: RevisionGate,
        dirty: DirtyWord,
        cells: [LatestCell<2>; CELLS],
    }

    impl<const CELLS: usize> Shared<CELLS> {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                gate: RevisionGate::new(0),
                dirty: DirtyWord::new(),
                cells: core::array::from_fn(|_| LatestCell::new()),
            })
        }
    }

    /// Render's side of one plan: a stage over `CELLS` cells, and what each cell holds.
    struct Render<const CELLS: usize> {
        stage: DirtyReader,
        readers: [CellReader; CELLS],
        applied: [Option<(u64, u32)>; CELLS],
        superseded: u64,
        applications: u64,
    }

    impl<const CELLS: usize> Render<CELLS> {
        fn new() -> Self {
            Self {
                stage: DirtyReader::new(),
                readers: core::array::from_fn(|_| CellReader::new()),
                applied: [None; CELLS],
                superseded: 0,
                applications: 0,
            }
        }

        /// One block of `shared`: the snapshot, then the take, then each pending cell in order.
        /// It returns `S`, after asserting nothing newer than `S` was applied.
        fn block(&mut self, shared: &Shared<CELLS>) -> u64 {
            let snapshot = shared.gate.snapshot();
            let pending = self.stage.pending(&shared.dirty);
            self.drain(shared, snapshot, pending);
            snapshot.revision()
        }

        fn drain(&mut self, shared: &Shared<CELLS>, snapshot: LiveSnapshot, pending: u64) {
            for cell in 0..CELLS {
                if pending & (1 << cell) == 0 {
                    continue;
                }
                let bit = u32::try_from(cell).expect("small");
                let read =
                    self.readers[cell].read(&shared.cells[cell], snapshot, &mut self.stage, bit);
                if let CellRead::Applied {
                    words, superseded, ..
                } = read
                {
                    let decoded = decode(words);
                    assert!(
                        decoded.0 <= snapshot.revision(),
                        "applied revision {} above S {}",
                        decoded.0,
                        snapshot.revision()
                    );
                    self.applied[cell] = Some(decoded);
                    self.superseded += superseded;
                    self.applications += 1;
                }
            }
        }
    }

    /// L1: one block never applies part of a transaction. Two cells, two transactions that each
    /// write both, racing one block.
    #[test]
    fn spsc_loom_cells_one_block_never_tears() {
        loom::model(|| {
            let shared = Shared::<2>::new();
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut pin = GatePin::new(0);
                let mut writers = [CellWriter::<2>::new(), CellWriter::new()];
                for revision in 1..=2 {
                    for (cell, writer) in writers.iter_mut().enumerate() {
                        writer.write(
                            &control_shared.cells[cell],
                            &control_shared.dirty,
                            u32::try_from(cell).expect("small"),
                            pin.stamp(revision),
                            value_words(revision, 0),
                        );
                    }
                    pin.publish(&control_shared.gate, revision);
                }
            });
            let mut render = Render::<2>::new();
            let s = render.block(&shared);
            let expected = (s != 0).then_some((s, 0));
            assert_eq!(render.applied, [expected, expected], "S = {s}");
            control.join().expect("control");
            let s = render.block(&shared);
            assert_eq!(s, 2);
            assert_eq!(render.applied, [Some((2, 0)), Some((2, 0))]);
            assert_eq!(
                render.superseded + render.applications,
                4,
                "every write resolved once"
            );
        });
    }

    /// L2: render pinned at an old snapshot while control laps it three times still reads exactly
    /// that snapshot's value. Control never fails or waits.
    #[test]
    fn spsc_loom_cells_a_pinned_reader_survives_a_lapping_writer() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            let shared = Shared::<1>::new();
            let mut pin = GatePin::new(0);
            let mut writer = CellWriter::<2>::new();
            writer.write(
                &shared.cells[0],
                &shared.dirty,
                0,
                pin.stamp(1),
                value_words(1, 0),
            );
            pin.publish(&shared.gate, 1);
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                for revision in 2..=4 {
                    writer.write(
                        &control_shared.cells[0],
                        &control_shared.dirty,
                        0,
                        pin.stamp(revision),
                        value_words(revision, 0),
                    );
                    pin.publish(&control_shared.gate, revision);
                }
            });
            let mut render = Render::<1>::new();
            let s = render.block(&shared);
            assert_eq!(
                render.applied[0],
                Some((s, 0)),
                "the newest value at or below S applies"
            );
            control.join().expect("control");
            render.block(&shared);
            assert_eq!(render.applied[0], Some((4, 0)));
            assert_eq!(render.superseded + render.applications, 4);
        });
    }

    /// L3: a write committed while a block drains is applied by a later block: its dirty bit
    /// either stays set, or the block that took it saw its revision above `S` and deferred it.
    #[test]
    fn spsc_loom_cells_a_dirty_bit_is_never_lost() {
        loom::model(|| {
            let shared = Shared::<1>::new();
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut pin = GatePin::new(0);
                let mut writer = CellWriter::<2>::new();
                for revision in 1..=2 {
                    writer.write(
                        &control_shared.cells[0],
                        &control_shared.dirty,
                        0,
                        pin.stamp(revision),
                        value_words(revision, 0),
                    );
                    pin.publish(&control_shared.gate, revision);
                }
            });
            let mut render = Render::<1>::new();
            for _ in 0..2 {
                let s = render.block(&shared);
                assert_eq!(render.applied[0], (s != 0).then_some((s, 0)));
            }
            control.join().expect("control");
            render.block(&shared);
            assert_eq!(
                render.applied[0],
                Some((2, 0)),
                "the last committed value applies"
            );
            assert_eq!(render.superseded + render.applications, 2);
        });
    }

    /// R2: two writes per revision race two blocks. A block applies the second write of revision
    /// `S`, or revision 1's only write.
    #[test]
    fn spsc_loom_cells_an_in_place_rewrite_races_reads() {
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            let shared = Shared::<1>::new();
            let mut pin = GatePin::new(0);
            let mut writer = CellWriter::<2>::new();
            writer.write(
                &shared.cells[0],
                &shared.dirty,
                0,
                pin.stamp(1),
                value_words(1, 0),
            );
            pin.publish(&shared.gate, 1);
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                for revision in 2..=3 {
                    for k in 0..2 {
                        writer.write(
                            &control_shared.cells[0],
                            &control_shared.dirty,
                            0,
                            pin.stamp(revision),
                            value_words(revision, k),
                        );
                    }
                    pin.publish(&control_shared.gate, revision);
                }
            });
            let mut render = Render::<1>::new();
            let expected = |s: u64| if s == 1 { (1, 0) } else { (s, 1) };
            for _ in 0..2 {
                let s = render.block(&shared);
                assert_eq!(render.applied[0], Some(expected(s)), "S = {s}");
            }
            control.join().expect("control");
            render.block(&shared);
            assert_eq!(render.applied[0], Some((3, 1)));
            assert_eq!(render.superseded + render.applications, 5);
        });
    }

    /// L4 (the review's R1): render claims a candidate while control stamps and publishes
    /// transactions to the candidate's own gate, whose pin render has never announced.
    ///
    /// Plan P runs on gate 0 and holds a write at revision 1. Candidate C's cell holds a write at
    /// revision 2, published on C's gate before C is published by a `Release` compare-and-swap of
    /// the mailbox word. Control then commits 3, 4 and 5 to C. Render claims C with an `AcqRel`
    /// compare-and-swap, takes C's snapshot and drains. At least two commits follow C's
    /// publication, so C's `B` protection is exercised.
    #[test]
    fn spsc_loom_cells_a_claim_races_stamped_writes() {
        struct Plan {
            gate: RevisionGate,
            dirty: DirtyWord,
            cells: [LatestCell<2>; 1],
        }
        struct Both {
            /// 0 while P runs, 1 once C is published, 2 once render claimed C. Every write to it
            /// is a compare-and-swap.
            mailbox: AtomicU64,
            plans: [Plan; 2],
        }
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            let plan = || Plan {
                gate: RevisionGate::new(0),
                dirty: DirtyWord::new(),
                cells: [LatestCell::new()],
            };
            let both = Arc::new(Both {
                mailbox: AtomicU64::new(0),
                plans: [plan(), plan()],
            });
            // P holds X at revision 1, published before render starts.
            let mut pin_p = GatePin::new(0);
            let mut writer_p = CellWriter::<2>::new();
            writer_p.write(
                &both.plans[0].cells[0],
                &both.plans[0].dirty,
                0,
                pin_p.stamp(1),
                value_words(1, 0),
            );
            pin_p.publish(&both.plans[0].gate, 1);
            // C holds a retarget at revision 2, published on C's gate before C is published.
            let mut pin_c = GatePin::new(0);
            let mut writer_c = CellWriter::<2>::new();
            writer_c.write(
                &both.plans[1].cells[0],
                &both.plans[1].dirty,
                0,
                pin_c.stamp(2),
                value_words(2, 0),
            );
            pin_c.publish(&both.plans[1].gate, 2);
            let control_both = Arc::clone(&both);
            let control = thread::spawn(move || {
                let both = &control_both;
                both.mailbox
                    .compare_exchange(0, 1, Ordering::Release, Ordering::Relaxed)
                    .expect("C is published once");
                for revision in 3..=5 {
                    writer_c.write(
                        &both.plans[1].cells[0],
                        &both.plans[1].dirty,
                        0,
                        pin_c.stamp(revision),
                        value_words(revision, 0),
                    );
                    pin_c.publish(&both.plans[1].gate, revision);
                }
            });
            let mut readers = [CellReader::new(), CellReader::new()];
            let mut stages = [DirtyReader::new(), DirtyReader::new()];
            let mut applied = [None, None];
            let mut resolved = [0u64; 2];
            let mut active = 0usize;
            let mut block = |active: &mut usize| -> u64 {
                if *active == 0
                    && both.mailbox.load(Ordering::Acquire) == 1
                    && both
                        .mailbox
                        .compare_exchange(1, 2, Ordering::AcqRel, Ordering::Relaxed)
                        .is_ok()
                {
                    *active = 1;
                }
                let plan = &both.plans[*active];
                let snapshot = plan.gate.snapshot();
                let pending = stages[*active].pending(&plan.dirty);
                if pending & 1 != 0 {
                    let read =
                        readers[*active].read(&plan.cells[0], snapshot, &mut stages[*active], 0);
                    if let CellRead::Applied {
                        words, superseded, ..
                    } = read
                    {
                        applied[*active] = Some(decode(words).0);
                        resolved[*active] += superseded + 1;
                    }
                }
                if *active == 0 {
                    assert_eq!((snapshot.revision(), applied[0]), (1, Some(1)));
                } else {
                    assert_eq!(
                        applied[1],
                        Some(snapshot.revision()),
                        "C applied {:?} at S {}",
                        applied[1],
                        snapshot.revision()
                    );
                }
                snapshot.revision()
            };
            block(&mut active);
            block(&mut active);
            control.join().expect("control");
            let s = block(&mut active);
            assert_eq!((active, s), (1, 5));
            assert_eq!(
                resolved[1], 4,
                "C's writes at 2 to 5 are resolved once each"
            );
        });
    }
}
