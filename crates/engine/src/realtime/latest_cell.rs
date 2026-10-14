//! The revision gate and the revision-bounded latest-target cell (decision 15, D15-2 and D15-17;
//! issues #1432 and #1603).
//!
//! # What this module guarantees
//!
//! One control thread writes live values into [`LatestCell`]s and then publishes the transaction
//! that wrote them through a [`RevisionGate`]. Render takes one [`LiveSnapshot`] `S` of the gate
//! per block, and every read of that block applies, per cell, the newest value whose publication
//! is `<= S`. It never applies a value newer than `S`, never a torn value, never a value twice,
//! and never skips one. So every value of one publication takes effect in one block. A write
//! cannot fail and never waits, however far control laps render.
//!
//! # Publications and revisions
//!
//! The gate orders by a **publication number**: a `u64` that counts the gate's publications, one
//! per publish, starting at 1 (0 is the gate's birth state). It is internal: no wire, session or
//! status field carries it. Every publication also carries the **session revision** it belongs
//! to. A model commit of revision `r` is a new publication that carries `r`; a monitor write
//! (#1603) is a new publication that carries the current revision `r` again and spends no
//! revision. So revisions stay dense while monitor writes are ordered with the model commits.
//! The applied-revision watermark reports the revision of the last applied publication, which
//! is never above the committed revision.
//!
//! Render learns the revision of its snapshot without a lock and without a retry. Control
//! writes each publication's `(number, revision)` pair into a three-slot record before it
//! publishes the number, with the same slot rule the cells use (below), and render reads the
//! slot with the greatest number at or below `S`. The pair is therefore never torn and render's
//! cost is a fixed number of loads.
//!
//! # The two operations that carry the proof
//!
//! The gate word is `ANNOUNCED | publication number`: bit 63 is render's announcement, bits 0-62
//! are the number. Render's `snapshot` is one `fetch_or(ANNOUNCED, AcqRel)`: the same atomic
//! operation reads `S` and records that a reader holds it. Control's `publish_key` is one
//! `swap(number, AcqRel)`: it clears the announcement and, if the old word carried one, learns the
//! snapshot (`pinned`) render may still be reading. A cell write (or the gate's own record write)
//! then protects two of its three slots from reuse: `A`, the newest value at or below the number
//! the gate had published when the write was stamped, and `B`, the newest value at or below the
//! pin. Those two slots hold every value a reader can still need, so the third slot is always
//! free to overwrite. The writer needs no knowledge of render's progress beyond the pin, and
//! render needs no retry.
//!
//! A gate and its [`GatePin`] are born together ([`RevisionGate::pair`]), so a pin cannot start
//! below, or apart from, its gate's initial revision. Control keeps one rule: it publishes each
//! publication it stamped before it stamps another. `stamp`, `stamp_monitor` and `publish_key`
//! debug-assert it. A stamp also refuses, in every build, a model revision at or below the last
//! published one or above the ceiling, a monitor stamp under any revision but the last published
//! one, and a gate that has used every publication number ([`StampRefused`]); a refusal changes
//! nothing. A [`LiveStamp`] borrows its pin, so a stamp cannot outlive its own publication: the
//! compiler, not a check, refuses a write with a stamp made stale by a later `publish`.
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
//! written only by plain stores (a slot's words, publication number and sequence, and the gate's
//! record slots). No model races a plain store against a read-modify-write on one location.
//!
//! # Realtime
//!
//! Safe Rust, atomics only. Construction allocates and runs on the control thread.
//! `snapshot`, `publish`, `stamp`, a write, a read and a peek never allocate, lock or make a
//! syscall. Render pays one read-modify-write per plan per block (the snapshot), three number
//! loads and one revision load for the snapshot's record, one `Relaxed` load per idle stage, and
//! for each pending cell three number loads, one sequence load and `N` word loads. On AArch64
//! without LSE (ARMv8.0) a read-modify-write is a load-linked or store-conditional loop, which
//! `docs/REALTIME_DEPENDENCY_POLICY.md` states.

use core::marker::PhantomData;
#[cfg(not(loom))]
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
#[cfg(loom)]
use loom::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Bit 63 of the gate word: render has taken a snapshot since control's last `publish`.
const ANNOUNCED: u64 = 1 << 63;

/// The largest revision a gate holds (bits 0-62). The session store's revision ceiling is this
/// value (#1503).
pub const GATE_REVISION_MAX: u64 = ANNOUNCED - 1;

/// The largest publication number a gate holds (bits 0-62).
const PUBLICATION_MAX: u64 = ANNOUNCED - 1;

/// A slot's number before its first write. It is above every legal number and above every
/// snapshot, so a reader tests for it before it compares.
const UNWRITTEN: u64 = u64::MAX;

/// Every cell, and the gate's record, has exactly three slots: the newest published value `A`,
/// the value the pin protects `B`, and one slot control may overwrite.
const SLOTS: usize = 3;

/// The identity of one publication: its internal number, which orders it among its gate's
/// publications, and the session revision it carries (#1603). A model commit and the monitor
/// writes after it carry the same revision and consecutive numbers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GateKey {
    publication: u64,
    revision: u64,
}

impl GateKey {
    /// The internal publication number: 1 for a gate's first publication.
    #[must_use]
    pub const fn publication(self) -> u64 {
        self.publication
    }

    /// The session revision the publication carries.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }
}

/// The publication render reads in one block, taken by [`RevisionGate::snapshot`], and the
/// session revision it carries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveSnapshot {
    publication: u64,
    revision: u64,
}

impl LiveSnapshot {
    /// Every publication a cell can hold. Its only user is a drain of a quiescent predecessor plan
    /// (#1312 D6): no production render path takes it.
    pub const ALL: Self = Self {
        publication: PUBLICATION_MAX,
        revision: GATE_REVISION_MAX,
    };

    /// The session revision of the snapshot's publication: the revision the block applied. The
    /// applied watermark reports this.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// The internal publication number: reads apply values at or below it.
    #[must_use]
    pub const fn publication(self) -> u64 {
        self.publication
    }
}

/// Why a stamp was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StampRefusal {
    /// A model stamp whose revision is at or below the last published revision.
    NotAbovePublished,
    /// A revision above [`GATE_REVISION_MAX`].
    AboveCeiling,
    /// A monitor stamp under a revision other than the one the gate published last.
    NotPublishedRevision,
    /// The gate has published every number it can hold (2^63 - 1 publications).
    PublicationsExhausted,
}

/// A refused [`GatePin::stamp`] or [`GatePin::stamp_monitor`]. The pin is unchanged. It borrows
/// nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StampRefused {
    /// The refused revision.
    pub revision: u64,
    /// The revision of the pin's last publication.
    pub last_published: u64,
    /// Why it was refused.
    pub reason: StampRefusal,
}

/// The publication a transaction makes, and the snapshot render may still hold, as of the time
/// control stamps it. Only [`GatePin::stamp`] and [`GatePin::stamp_monitor`] make one.
///
/// A stamp borrows the pin that made it (`'p`), so [`GatePin::publish`] cannot run while a stamp
/// is alive: a stamp cannot outlive its own publication, and a write cannot use a stamp whose
/// facts (`pinned`, `published`) a later publication has made stale. The borrow is a type-level
/// fact with no runtime cost. One stamp serves every cell write of one transaction:
///
/// ```
/// use engine::realtime::{CellWriter, DirtyWord, LatestCell, RevisionGate};
///
/// let (gate, mut pin) = RevisionGate::pair(0);
/// let (cells, dirty) = ([LatestCell::<1>::new(), LatestCell::<1>::new()], DirtyWord::new());
/// let mut writers = [CellWriter::<1>::new(), CellWriter::<1>::new()];
/// let stamp = pin.stamp(1).expect("a legal revision");
/// for (bit, (writer, cell)) in writers.iter_mut().zip(&cells).enumerate() {
///     writer.write(cell, &dirty, bit as u32, stamp, [7]);
/// }
/// let key = stamp.key();
/// pin.publish_key(&gate, key);
/// ```
///
/// A stamp used after its publication does not compile, because the publication needs the pin
/// the stamp still borrows:
///
/// ```compile_fail,E0499
/// use engine::realtime::{CellWriter, DirtyWord, LatestCell, RevisionGate};
///
/// let (gate, mut pin) = RevisionGate::pair(0);
/// let (cell, dirty, mut writer) = (LatestCell::<1>::new(), DirtyWord::new(), CellWriter::<1>::new());
/// let stamp = pin.stamp(1).expect("a legal revision");
/// writer.write(&cell, &dirty, 0, stamp, [7]);
/// pin.publish(&gate, 1);
/// // The stamp's `published` and `pinned` are now stale; this write would target a published slot.
/// writer.write(&cell, &dirty, 0, stamp, [8]);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveStamp<'p> {
    /// The publication number the transaction publishes as.
    key: u64,
    /// The session revision it carries.
    revision: u64,
    pinned: Option<u64>,
    /// The gate's published number when the stamp was made: `A` is the newest value at or below
    /// it.
    published: u64,
    /// The borrow of the pin that made the stamp.
    pin: PhantomData<&'p mut GatePin>,
}

impl LiveStamp<'_> {
    /// The session revision the transaction that writes with this stamp carries.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// The publication the transaction makes. A caller reads it into a local and publishes it
    /// with [`GatePin::publish_key`].
    #[must_use]
    pub const fn key(self) -> GateKey {
        GateKey {
            publication: self.key,
            revision: self.revision,
        }
    }

    /// The publication number of the snapshot render may still be reading, learned from the gate
    /// at the last publication that saw an announcement. `None` when render has not announced
    /// since the gate was built.
    #[must_use]
    pub const fn pinned(self) -> Option<u64> {
        self.pinned
    }
}

/// One slot of a gate's record: a publication number and the revision it carries. Both are
/// written by plain stores only, and a reader reads the slot only when the slot rule protects it.
struct RecordSlot {
    publication: AtomicU64,
    revision: AtomicU64,
}

/// One plan's revision gate: the word render snapshots and control publishes to, and the record
/// of the revision each recent publication carries.
///
/// A gate belongs to one plan for the plan's life (#1502), so it has no re-arm. It is embeddable:
/// it owns no `Arc`.
pub struct RevisionGate {
    word: AtomicU64,
    record: [RecordSlot; SLOTS],
}

impl RevisionGate {
    /// A gate at `initial` with no announcement, and the one [`GatePin`] that publishes to it.
    ///
    /// The gate starts at publication number 0, which carries `initial`, and the pin starts there
    /// too, so the two cannot disagree. A gate built at `initial` takes model stamps above
    /// `initial` only (and monitor stamps under it), and a candidate whose first writes are
    /// stamped at revision `r` needs a gate built below `r`.
    ///
    /// # Panics
    ///
    /// Panics when `initial` is above [`GATE_REVISION_MAX`]. Construction is a control-thread
    /// operation.
    #[must_use]
    pub fn pair(initial: u64) -> (Self, GatePin) {
        assert!(
            initial <= GATE_REVISION_MAX,
            "gate revision {initial} is above the ceiling {GATE_REVISION_MAX}"
        );
        let record = core::array::from_fn(|index| RecordSlot {
            publication: AtomicU64::new(if index == 0 { 0 } else { UNWRITTEN }),
            revision: AtomicU64::new(if index == 0 { initial } else { 0 }),
        });
        (
            Self {
                word: AtomicU64::new(0),
                record,
            },
            GatePin {
                pinned: None,
                last_publication: 0,
                last_revision: initial,
                record_numbers: [0, UNWRITTEN, UNWRITTEN],
                open: None,
            },
        )
    }

    /// The revision carried by the newest publication at or below `bound`, or 0 when no slot
    /// holds one (the slot rule keeps the snapshot's publication readable, so 0 is unreachable;
    /// it is the value that can never be early).
    fn revision_at_or_below(&self, bound: u64) -> u64 {
        let mut best: Option<(&RecordSlot, u64)> = None;
        for slot in &self.record {
            let number = slot.publication.load(Ordering::Relaxed);
            if number == UNWRITTEN || number > bound {
                continue;
            }
            if best.is_none_or(|(_, held)| number > held) {
                best = Some((slot, number));
            }
        }
        best.map_or(0, |(slot, _)| slot.revision.load(Ordering::Relaxed))
    }

    /// The revision carried by the newest published publication, with the announcement masked.
    /// Control reads it on its own gate, or anyone reads it once control is quiescent: the record
    /// is not protected against a concurrent publish.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision_at_or_below(self.word.load(Ordering::Acquire) & PUBLICATION_MAX)
    }

    /// The newest published publication number, with the announcement masked.
    #[must_use]
    pub fn publication(&self) -> u64 {
        self.word.load(Ordering::Acquire) & PUBLICATION_MAX
    }
}

// REALTIME_POLICY_BEGIN
impl RevisionGate {
    /// Render, once per block: read `S` and announce it in one read-modify-write, then read the
    /// revision `S` carries.
    ///
    /// One `fetch_or(ANNOUNCED, AcqRel)`. The acquire half synchronizes with every earlier
    /// control `publish`, so every cell store and record store of a publication `<= S` is visible.
    /// The release half ends the previous section's reads for the next `publish`. A load here
    /// would lose the announcement, and a `Relaxed` operation would lose both edges. The record
    /// read that follows is three number loads and one revision load: the slot with the greatest
    /// number at or below `S` is protected by the slot rule, so it is never overwritten while
    /// render reads it, and there is no retry.
    #[must_use]
    pub fn snapshot(&self) -> LiveSnapshot {
        let publication = self.word.fetch_or(ANNOUNCED, Ordering::AcqRel) & PUBLICATION_MAX;
        LiveSnapshot {
            publication,
            revision: self.revision_at_or_below(publication),
        }
    }
}
// REALTIME_POLICY_END

/// Control's private half of one gate: the snapshot render may still hold, the last publication
/// made and the publication stamped but not yet made. One per gate, born with it by
/// [`RevisionGate::pair`]; it moves between threads only by ownership transfer.
#[derive(Debug)]
pub struct GatePin {
    pinned: Option<u64>,
    last_publication: u64,
    last_revision: u64,
    /// A mirror of the three record slots' numbers. Exact: control is the record's only writer.
    record_numbers: [u64; SLOTS],
    open: Option<GateKey>,
}

impl GatePin {
    /// The revision of the last publication on the gate, or the gate's initial revision.
    #[must_use]
    pub const fn last_published(&self) -> u64 {
        self.last_revision
    }

    /// The last publication on the gate: number 0 and the initial revision for a gate that has
    /// published nothing.
    #[must_use]
    pub const fn last_published_key(&self) -> GateKey {
        GateKey {
            publication: self.last_publication,
            revision: self.last_revision,
        }
    }

    /// The record slot with the greatest mirrored number at or below `bound`.
    fn record_newest_at_or_below(&self, bound: u64) -> Option<usize> {
        let mut best: Option<usize> = None;
        for (index, &number) in self.record_numbers.iter().enumerate() {
            if number == UNWRITTEN || number > bound {
                continue;
            }
            if best.is_none_or(|held| number > self.record_numbers[held]) {
                best = Some(index);
            }
        }
        best
    }

    /// Control, the last write of a model commit: publish a new publication that carries
    /// `revision` on `gate`, the gate this pin was born with. It is [`Self::publish_key`] of the
    /// stamped key when `revision` is the stamped one, else of the next number carrying
    /// `revision`.
    ///
    /// # Panics
    ///
    /// In every build, when `revision` is above [`GATE_REVISION_MAX`]. In debug builds,
    /// [`Self::publish_key`]'s panics, and a `revision` at or below the last published one.
    pub fn publish(&mut self, gate: &RevisionGate, revision: u64) {
        assert!(
            revision <= GATE_REVISION_MAX,
            "published revision {revision} is above the ceiling {GATE_REVISION_MAX}"
        );
        debug_assert!(
            revision > self.last_revision,
            "published revision {revision} is not above the last published {}",
            self.last_revision
        );
        let key = self
            .open
            .filter(|open| open.revision == revision)
            .unwrap_or(GateKey {
                publication: self.last_publication + 1,
                revision,
            });
        self.publish_key(gate, key);
    }

    /// Control, the last write of a commit: publish `key` on `gate`, the gate this pin was born
    /// with.
    ///
    /// It records `(number, revision)` in a record slot that is neither `A` nor `B`, then does one
    /// `swap(number, AcqRel)`. If the old word carried the announcement, the pin becomes the old
    /// number (render's snapshot); otherwise it is unchanged, because render has announced
    /// nothing newer.
    ///
    /// In debug builds, these panic: a number at or below the last published one (it would end
    /// the ordering that protects render's slots); a key other than the one stamped and not yet
    /// published (that stamp's writes would be abandoned or would leak into another revision);
    /// a revision below the last published one; and a `gate` whose old number is not the last
    /// published one (the pin was born with another gate). The last check reads the swap's result
    /// and costs nothing.
    pub fn publish_key(&mut self, gate: &RevisionGate, key: GateKey) {
        debug_assert!(
            key.revision <= GATE_REVISION_MAX,
            "published revision {} is above the ceiling {GATE_REVISION_MAX}",
            key.revision
        );
        debug_assert!(
            key.publication > self.last_publication && key.publication <= PUBLICATION_MAX,
            "published number {} is not above the last published {}",
            key.publication,
            self.last_publication
        );
        debug_assert!(
            key.revision >= self.last_revision,
            "published revision {} is below the last published {}",
            key.revision,
            self.last_revision
        );
        debug_assert!(
            self.open.is_none_or(|open| open == key),
            "published key {key:?} is not the stamped, unpublished key {:?}",
            self.open
        );
        // The record slot: neither `A`, the newest publication, nor `B`, the pinned one.
        let a = self.record_newest_at_or_below(self.last_publication);
        let b = self
            .pinned
            .and_then(|pinned| self.record_newest_at_or_below(pinned));
        let index = (0..SLOTS)
            .find(|&index| Some(index) != a && Some(index) != b)
            .expect("three slots leave one free of A and B");
        let slot = &gate.record[index];
        slot.revision.store(key.revision, Ordering::Relaxed);
        slot.publication.store(key.publication, Ordering::Relaxed);
        self.record_numbers[index] = key.publication;
        let old = gate.word.swap(key.publication, Ordering::AcqRel);
        debug_assert_eq!(
            old & PUBLICATION_MAX,
            self.last_publication,
            "this pin was born with another gate"
        );
        if old & ANNOUNCED != 0 {
            self.pinned = Some(old & PUBLICATION_MAX);
        }
        self.last_publication = key.publication;
        self.last_revision = key.revision;
        self.open = None;
    }

    /// The stamp for a model transaction that commits as `revision`: the next publication number,
    /// carrying `revision`.
    ///
    /// The stamp borrows the pin, so a publication cannot run while the stamp is alive: take the
    /// stamp, write every cell of the transaction with it, then publish.
    ///
    /// **Rule:** control publishes every stamp it took before it takes another. A transaction
    /// that stamps writes all its cells and then publishes that key; no other stamp is taken
    /// between. Cell writes choose their slot on this rule.
    ///
    /// # Errors
    ///
    /// Refuses, in every build, a `revision` above [`GATE_REVISION_MAX`]
    /// ([`StampRefusal::AboveCeiling`]), a `revision` at or below the last published one
    /// ([`StampRefusal::NotAbovePublished`]: the in-place rule would rewrite a slot render may be
    /// reading, and render could report a revision twice) and a gate with no publication number
    /// left ([`StampRefusal::PublicationsExhausted`]). A refusal changes nothing in the pin. The
    /// cost is integer compares on the control thread. A caller publishes the key its stamp
    /// returned, read into a local, so this check covers the publication too.
    ///
    /// In debug builds a second key stamped while another is stamped and not yet published panics
    /// (the rule above). That rule stays debug-only: breaking it leaks an abandoned value, never a
    /// torn one.
    pub fn stamp(&mut self, revision: u64) -> Result<LiveStamp<'_>, StampRefused> {
        if revision > GATE_REVISION_MAX {
            return Err(self.refuse(revision, StampRefusal::AboveCeiling));
        }
        if revision <= self.last_revision {
            return Err(self.refuse(revision, StampRefusal::NotAbovePublished));
        }
        self.open_stamp(revision)
    }

    /// The stamp for a monitor write under the committed `revision`: the next publication number,
    /// carrying `revision` again (#1603). A monitor write spends no revision.
    ///
    /// # Errors
    ///
    /// Refuses, in every build and with the pin unchanged: a `revision` other than the last
    /// published one ([`StampRefusal::NotPublishedRevision`]; a monitor write sits under the
    /// revision the gate published last, which is the committed one) and a gate with no
    /// publication number left ([`StampRefusal::PublicationsExhausted`]).
    pub fn stamp_monitor(&mut self, revision: u64) -> Result<LiveStamp<'_>, StampRefused> {
        if revision != self.last_revision {
            return Err(self.refuse(revision, StampRefusal::NotPublishedRevision));
        }
        self.open_stamp(revision)
    }

    fn refuse(&self, revision: u64, reason: StampRefusal) -> StampRefused {
        StampRefused {
            revision,
            last_published: self.last_revision,
            reason,
        }
    }

    fn open_stamp(&mut self, revision: u64) -> Result<LiveStamp<'_>, StampRefused> {
        if self.last_publication >= PUBLICATION_MAX {
            return Err(self.refuse(revision, StampRefusal::PublicationsExhausted));
        }
        let key = GateKey {
            publication: self.last_publication + 1,
            revision,
        };
        debug_assert!(
            self.open.is_none_or(|open| open == key),
            "stamped key {key:?} while key {:?} is stamped and not yet published",
            self.open
        );
        self.open = Some(key);
        Ok(LiveStamp {
            key: key.publication,
            revision,
            pinned: self.pinned,
            published: self.last_publication,
            pin: PhantomData,
        })
    }
}

struct Slot<const N: usize> {
    words: [AtomicU32; N],
    key: AtomicU64,
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
                key: AtomicU64::new(UNWRITTEN),
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

/// Control's private half of one cell: a mirror of the three slot keys and the next
/// sequence number. The mirror is exact, because control is the cell's only writer.
#[derive(Debug)]
pub struct CellWriter<const N: usize> {
    keys: [u64; SLOTS],
    next_sequence: u64,
}

impl<const N: usize> CellWriter<N> {
    /// A writer for a cell with no written slot. Sequences start at 1.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            keys: [UNWRITTEN; SLOTS],
            next_sequence: 1,
        }
    }

    /// The slot with the greatest mirrored key at or below `bound`.
    fn newest_at_or_below(&self, bound: u64) -> Option<usize> {
        let mut best: Option<usize> = None;
        for (index, &key) in self.keys.iter().enumerate() {
            if key == UNWRITTEN || key > bound {
                continue;
            }
            match best {
                Some(held) if self.keys[held] >= key => {}
                _ => best = Some(index),
            }
        }
        best
    }

    /// The slot a write stamped `stamp` stores into.
    fn target(&self, stamp: LiveStamp<'_>) -> usize {
        // One key occupies at most one slot, so a second write of it rewrites that slot. Render
        // reads only keys at or below its snapshot, and this one is not published.
        if let Some(index) = self.keys.iter().position(|&k| k == stamp.key) {
            return index;
        }
        // A: the newest published value. A value above the published key is never `A`, so a key
        // that was stamped and abandoned cannot shield a slot render needs.
        let a = self.newest_at_or_below(stamp.published);
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
    /// `stamp.key`, else a slot that is neither `A` nor `B`. The stores are `Relaxed` and in
    /// any order; the dirty bit follows them with a `Release` `fetch_or`, and the key's
    /// publication on the gate, which the caller makes after all of the transaction's writes,
    /// carries the visibility.
    ///
    /// **Rule:** every key written to a cell is published (by [`GatePin::publish_key`]) before a
    /// higher key is stamped; [`GatePin::stamp`] states it and debug-asserts it. A key written and
    /// then abandoned leaves its value in a slot that a later write reuses; that is the caller's
    /// contract, and the slot of a published value is never taken.
    ///
    /// In debug builds, a stamp whose publication is below the cell's newest written one panics (a stamp
    /// of this pin cannot be stale, because it borrows the pin; the check catches a stamp from
    /// another pin only when its key is below one already written), and so does a `bit` of 64 or
    /// more.
    pub fn write(
        &mut self,
        cell: &LatestCell<N>,
        dirty: &DirtyWord,
        bit: u32,
        stamp: LiveStamp<'_>,
        words: [u32; N],
    ) {
        debug_assert!(bit < u64::BITS, "dirty bit {bit} is outside the word");
        debug_assert!(
            stamp.key <= PUBLICATION_MAX,
            "stamped publication {} is above the ceiling {PUBLICATION_MAX}",
            stamp.key
        );
        debug_assert!(
            self.keys.iter().all(|&k| k == UNWRITTEN || k <= stamp.key),
            "stamp publication {} is below a publication already written to this cell",
            stamp.key
        );
        let index = self.target(stamp);
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let slot = &cell.slots[index];
        for (word, value) in slot.words.iter().zip(words) {
            word.store(value, Ordering::Relaxed);
        }
        slot.sequence.store(sequence, Ordering::Relaxed);
        slot.key.store(stamp.key, Ordering::Relaxed);
        self.keys[index] = stamp.key;
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
    /// Of the three slots it picks the greatest key at or below `snapshot`, skipping
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
            let key = slot.key.load(Ordering::Relaxed);
            if key == UNWRITTEN {
                continue;
            }
            if key > snapshot.publication {
                newer = true;
            } else if chosen.is_none_or(|(_, best)| key > best) {
                chosen = Some((slot, key));
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

    /// [`Self::read`], and when it applies a write, the writes it replaced join `superseded`
    /// (#1312 D2). This is the only way to write a [`LiveValuesSuperseded`], so the count's
    /// single-writer rule is the reader's own: render, the cell's one reader.
    pub fn read_counted<const N: usize>(
        &mut self,
        cell: &LatestCell<N>,
        snapshot: LiveSnapshot,
        stage: &mut DirtyReader,
        bit: u32,
        superseded: &LiveValuesSuperseded,
    ) -> CellRead<N> {
        let read = self.read(cell, snapshot, stage, bit);
        if let CellRead::Applied {
            superseded: replaced,
            ..
        } = read
        {
            superseded.add(replaced);
        }
        read
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
            if slot.key.load(Ordering::Relaxed) == UNWRITTEN {
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

/// The layout of an `Arc<T>` allocation: the two counts, then `T` (the allocation `Arc::new`
/// makes). A resource plan charges each shared lane or count by `Layout::new::<SharedAllocation<T>>()`.
#[repr(C)]
pub struct SharedAllocation<T> {
    strong: core::sync::atomic::AtomicUsize,
    weak: core::sync::atomic::AtomicUsize,
    value: T,
}

/// The session's count of live values a later write replaced before render read them (D15-2,
/// #1312 D2): `live_values_superseded`.
///
/// One per session, shared by every plan the session prepares and by every kind of lane that holds
/// cells (strips #1312 and #1346, routes #1347, effects #1345). Render is its only writer, with a load, a
/// saturating add and a store (never a read-modify-write: a plan swap hands render from one plan
/// to the next on the same single render thread, so two writers never overlap); control reads it.
/// A clone shares the count.
#[derive(Clone, Debug, Default)]
pub struct LiveValuesSuperseded(std::sync::Arc<core::sync::atomic::AtomicU64>);

impl LiveValuesSuperseded {
    /// A count at zero.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Exact bytes of the one allocation a count retains: the word with the shared-owner header.
    /// A host that makes a count per plan charges it to that plan's row; a host that makes one per
    /// session charges it once (#1312 D2).
    #[must_use]
    pub fn retained_bytes() -> u64 {
        u64::try_from(
            core::alloc::Layout::new::<SharedAllocation<core::sync::atomic::AtomicU64>>().size(),
        )
        .expect("a count allocation's size fits u64")
    }
}

// REALTIME_POLICY_BEGIN
impl LiveValuesSuperseded {
    /// The writes counted so far: a relaxed load. Control reads it for the counters, and render
    /// reads it after each block to publish it in the browser's status (#1399).
    #[must_use]
    pub fn get(&self) -> u64 {
        self.0.load(core::sync::atomic::Ordering::Relaxed)
    }

    /// Render: add `count` replaced writes. Saturating. A load and a store, not a read-modify-write,
    /// because render is the count's only writer. The method is private to this module, so the only
    /// caller is [`CellReader::read_counted`], which render's drains call: only a cell read writes
    /// the count. `read_counted` is public, so a reader built over a stamped cell could call it
    /// elsewhere; the rule rests on who holds the readers, not on a type barrier (#1347 attempt 2,
    /// n1).
    fn add(&self, count: u64) {
        if count != 0 {
            let now = self.0.load(core::sync::atomic::Ordering::Relaxed);
            self.0.store(
                now.saturating_add(count),
                core::sync::atomic::Ordering::Relaxed,
            );
        }
    }
}
// REALTIME_POLICY_END

impl PartialEq for LiveValuesSuperseded {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for LiveValuesSuperseded {}

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
            let (gate, pin) = RevisionGate::pair(0);
            Self {
                cell: LatestCell::new(),
                dirty: DirtyWord::new(),
                gate,
                pin,
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
                self.pin.stamp(revision).expect("a legal revision"),
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
        f.publish(1);
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
        assert_eq!(
            f.pin.stamp(2).expect("a legal revision").pinned(),
            None,
            "no announcement yet"
        );
        assert_eq!(f.gate.snapshot().revision(), 1);
        f.publish(2);
        assert_eq!(
            f.pin.stamp(3).expect("a legal revision").pinned(),
            Some(1),
            "render holds snapshot 1"
        );
        f.publish(3);
        f.publish(4);
        assert_eq!(
            f.pin.stamp(5).expect("a legal revision").pinned(),
            Some(1),
            "no newer announcement, so the pin stays"
        );
        assert_eq!(f.gate.snapshot().revision(), 4);
        f.publish(5);
        assert_eq!(f.pin.stamp(6).expect("a legal revision").pinned(), Some(4));
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
        let (gate, mut pin) = RevisionGate::pair(0);
        assert_eq!(gate.snapshot().revision(), 0);
        pin.publish(&gate, 0x7fff_ffff_ffff_ffff);
        assert_eq!(pin.pinned, Some(0), "the announcement was set");
        let snapshot = gate.snapshot();
        assert_eq!(snapshot.revision(), 0x7fff_ffff_ffff_ffff);
        assert_eq!(snapshot.publication(), 1);
        assert_eq!(gate.revision(), 0x7fff_ffff_ffff_ffff);
        assert_eq!(GATE_REVISION_MAX, 0x7fff_ffff_ffff_ffff);
        assert_eq!(LiveSnapshot::ALL.revision(), GATE_REVISION_MAX);
    }

    #[test]
    #[should_panic(expected = "above the ceiling")]
    fn a_gate_above_the_ceiling_is_refused() {
        let _ = RevisionGate::pair(GATE_REVISION_MAX + 1);
    }

    /// Root ruling n3 (#1432 D1): the stamp's revision bound is on in every build and a refusal
    /// changes nothing. In a release build an unchecked stamp at the published revision would
    /// make the in-place rule rewrite the slot render may be reading.
    #[test]
    fn a_stamp_at_or_below_the_published_revision_or_above_the_ceiling_is_refused_and_changes_nothing()
     {
        let (gate, mut pin) = RevisionGate::pair(5);
        let (cell, dirty) = (LatestCell::<2>::new(), DirtyWord::new());
        let (mut writer, mut reader, mut stage) = (
            CellWriter::<2>::new(),
            CellReader::new(),
            DirtyReader::new(),
        );
        for (revision, reason) in [
            (5, StampRefusal::NotAbovePublished),
            (3, StampRefusal::NotAbovePublished),
            (GATE_REVISION_MAX + 1, StampRefusal::AboveCeiling),
            (u64::MAX, StampRefusal::AboveCeiling),
        ] {
            assert_eq!(
                pin.stamp(revision).map(|_| ()),
                Err(StampRefused {
                    revision,
                    last_published: 5,
                    reason
                })
            );
        }
        assert_eq!(pin.last_published(), 5);
        // In a debug build this stamp would panic if a refusal had set `open`.
        let stamp = pin.stamp(7).expect("a revision above the published one");
        writer.write(&cell, &dirty, 0, stamp, value_words(7, 0));
        pin.publish(&gate, 7);
        let snapshot = gate.snapshot();
        assert_eq!(stage.pending(&dirty), 1);
        let CellRead::Applied { words, .. } = reader.read(&cell, snapshot, &mut stage, 0) else {
            panic!("the value applies in the first block");
        };
        assert_eq!(words, value_words(7, 0));
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
    #[should_panic(expected = "is stamped and not yet published")]
    fn a_second_revision_is_not_stamped_before_the_first_is_published() {
        let mut f = Fixture::new();
        let _ = f.pin.stamp(3).expect("a legal revision");
        let _ = f.pin.stamp(4).expect("a legal revision");
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "is not the stamped, unpublished key")]
    fn a_publication_below_the_stamped_revision_panics() {
        let mut f = Fixture::new();
        let _ = f.pin.stamp(5).expect("a legal revision");
        f.publish(4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "born with another gate")]
    fn a_pin_publishes_only_to_its_own_gate() {
        let (other_gate, mut other_pin) = RevisionGate::pair(5);
        other_pin.publish(&other_gate, 6);
        let (_, mut pin) = RevisionGate::pair(5);
        pin.publish(&other_gate, 6);
    }

    #[test]
    fn a_pair_starts_the_pin_at_the_gates_revision() {
        let (gate, pin) = RevisionGate::pair(7);
        assert_eq!(gate.revision(), 7);
        assert_eq!(pin.last_published(), 7);
    }

    #[test]
    fn an_abandoned_revision_never_shields_the_slot_render_holds() {
        let mut f = Fixture::new();
        f.write(1, 0);
        f.publish(1);
        f.write(2, 0);
        f.publish(2);
        // Render snapshots S = 2 and takes its pending set. Control then stamps revision 3 and
        // abandons it. A caller that breaks the publish-before-a-higher-stamp rule stamps 4 by
        // hand, as the published revision is still 2.
        let snapshot = f.gate.snapshot();
        assert_eq!(f.stage.pending(&f.dirty) & 1, 1);
        f.write(3, 0);
        let stamp = LiveStamp {
            key: 4,
            revision: 4,
            pinned: f.pin.pinned,
            published: f.pin.last_publication,
            pin: PhantomData,
        };
        f.writer
            .write(&f.cell, &f.dirty, 0, stamp, value_words(4, 0));
        let read = f.reader.read(&f.cell, snapshot, &mut f.stage, 0);
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(2, 0),
                sequence: 2,
                superseded: 1
            },
            "render's snapshot 2 still reads revision 2's value"
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "is below a publication already written")]
    fn a_write_below_a_written_revision_panics() {
        // A stamp of this pin cannot be kept across a publication (it borrows the pin), so the
        // stale stamp is forged, as a stamp of another pin would be.
        let mut f = Fixture::new();
        f.write(3, 0);
        f.publish(3);
        f.write(4, 0);
        let old = LiveStamp {
            key: 1,
            revision: 3,
            pinned: None,
            published: 0,
            pin: PhantomData,
        };
        f.writer.write(&f.cell, &f.dirty, 0, old, value_words(3, 0));
    }

    /// Stamp, write the fixture's cell with the gate's publication number as the value index,
    /// publish; the key.
    fn write_and_publish(f: &mut Fixture, revision: u64, monitor: bool) -> GateKey {
        let stamp = if monitor {
            f.pin.stamp_monitor(revision)
        } else {
            f.pin.stamp(revision)
        }
        .expect("a legal stamp");
        let published = stamp.key();
        f.writer.write(
            &f.cell,
            &f.dirty,
            0,
            stamp,
            value_words(
                revision,
                u32::try_from(published.publication()).expect("small"),
            ),
        );
        f.pin.publish_key(&f.gate, published);
        published
    }

    /// Gate 1 (#1603): a monitor stamp takes the next publication under the published revision
    /// and spends no revision; a model stamp carries its own revision; a refusal moves nothing.
    #[test]
    fn monitor_stamps_take_publications_under_a_revision_and_spend_none() {
        let (gate, mut pin) = RevisionGate::pair(0);
        let publish = |pin: &mut GatePin, monitor: bool, revision: u64| {
            let stamp = if monitor {
                pin.stamp_monitor(revision)
            } else {
                pin.stamp(revision)
            }
            .expect("a legal stamp");
            let published = stamp.key();
            pin.publish_key(&gate, published);
            (published.publication(), published.revision())
        };
        assert_eq!(publish(&mut pin, false, 1), (1, 1));
        for number in 2..=4 {
            assert_eq!(publish(&mut pin, true, 1), (number, 1));
        }
        assert_eq!(pin.last_published(), 1, "monitor writes spend no revision");
        assert_eq!(publish(&mut pin, false, 2), (5, 2));
        for revision in [1, 3] {
            assert_eq!(
                pin.stamp_monitor(revision).map(|_| ()),
                Err(StampRefused {
                    revision,
                    last_published: 2,
                    reason: StampRefusal::NotPublishedRevision
                })
            );
        }
        assert_eq!(publish(&mut pin, true, 2), (6, 2));
        assert_eq!(
            pin.stamp(2).map(|_| ()),
            Err(StampRefused {
                revision: 2,
                last_published: 2,
                reason: StampRefusal::NotAbovePublished
            }),
            "a model stamp at the published revision is refused after monitor writes"
        );
        assert_eq!(gate.publication(), 6, "a refusal leaves the gate word");
        assert_eq!(gate.revision(), 2);
        assert_eq!(pin.last_published_key().publication(), 6);
        assert_eq!(publish(&mut pin, true, 2), (7, 2));
        assert_eq!(publish(&mut pin, false, 3), (8, 3));
    }

    /// Gate 1: a gate that used every publication number refuses both stamps, changes nothing,
    /// and `publish` of a stamped key at the last number still works.
    #[test]
    fn an_exhausted_gate_refuses_both_stamps_and_changes_nothing() {
        let (gate, mut pin) = RevisionGate::pair(4);
        gate.word.store(PUBLICATION_MAX - 1, Ordering::Relaxed);
        pin.last_publication = PUBLICATION_MAX - 1;
        pin.record_numbers = [PUBLICATION_MAX - 1, UNWRITTEN, UNWRITTEN];
        let stamp = pin.stamp_monitor(4).expect("the last number");
        let last = stamp.key();
        assert_eq!(last.publication(), PUBLICATION_MAX);
        pin.publish_key(&gate, last);
        assert_eq!(gate.revision(), 4);
        let reason = StampRefusal::PublicationsExhausted;
        assert_eq!(
            pin.stamp_monitor(4).map(|_| ()),
            Err(StampRefused {
                revision: 4,
                last_published: 4,
                reason
            })
        );
        assert_eq!(
            pin.stamp(5).map(|_| ()),
            Err(StampRefused {
                revision: 5,
                last_published: 4,
                reason
            })
        );
        assert_eq!(gate.publication(), PUBLICATION_MAX);
        assert!(pin.open.is_none(), "a refusal opens nothing");
    }

    /// Gate 1: a snapshot reads the revision its own publication carries, however many
    /// publications follow it: the pinned record slot is never overwritten.
    #[test]
    fn a_pinned_snapshot_keeps_its_revision_across_laps() {
        let mut f = Fixture::new();
        write_and_publish(&mut f, 1, false);
        write_and_publish(&mut f, 1, true);
        let snapshot = f.gate.snapshot();
        assert_eq!((snapshot.publication(), snapshot.revision()), (2, 1));
        let mut revision = 1;
        for step in 0..40_u32 {
            if step % 7 == 6 {
                revision += 1;
                write_and_publish(&mut f, revision, false);
            } else {
                write_and_publish(&mut f, revision, true);
            }
            assert_eq!(
                f.gate.revision_at_or_below(snapshot.publication()),
                1,
                "step {step}: the pinned publication still names revision 1"
            );
            assert_eq!(f.gate.revision(), revision);
        }
    }

    /// Gate 2: a cell read compares publications, so a monitor write applies, the snapshot bounds
    /// monitor writes too, and `superseded` counts the writes a read skipped. The snapshot's
    /// revision is the revision of the last publication.
    #[test]
    fn a_cell_applies_the_newest_publication_at_or_below_the_snapshot() {
        let mut f = Fixture::new();
        let mut applied = Vec::new();
        for (revision, monitor) in [(1, false), (1, true), (1, true), (2, false)] {
            let published = write_and_publish(&mut f, revision, monitor);
            let (snapshot, read) = f.block();
            assert_eq!(snapshot.publication(), published.publication());
            applied.push((snapshot.revision(), read));
        }
        for (index, (revision, read)) in applied.into_iter().enumerate() {
            assert_eq!(revision, [1, 1, 1, 2][index]);
            let CellRead::Applied {
                sequence,
                superseded,
                ..
            } = read
            else {
                panic!("every publication applies in its own block");
            };
            assert_eq!((sequence, superseded), (index as u64 + 1, 0));
        }
        // Three monitor writes published between two blocks: the read applies the newest and
        // counts two.
        for _ in 0..3 {
            write_and_publish(&mut f, 2, true);
        }
        let (snapshot, read) = f.block();
        assert_eq!((snapshot.publication(), snapshot.revision()), (7, 2));
        assert_eq!(
            read,
            CellRead::Applied {
                words: value_words(2, 7),
                sequence: 7,
                superseded: 2
            }
        );
        // A monitor write stamped and written but not yet published waits for its publication.
        let stamp = f.pin.stamp_monitor(2).expect("publication 8");
        let key = stamp.key();
        f.writer
            .write(&f.cell, &f.dirty, 0, stamp, value_words(2, 8));
        let (snapshot, read) = f.block();
        assert_eq!(snapshot.publication(), 7);
        assert_eq!(
            read,
            CellRead::Unchanged,
            "publication 8 is above the snapshot"
        );
        f.pin.publish_key(&f.gate, key);
        let (snapshot, read) = f.block();
        assert_eq!((snapshot.publication(), snapshot.revision()), (8, 2));
        assert!(matches!(read, CellRead::Applied { sequence: 8, .. }));
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
        /// The shared state, and the gate's pin for control.
        fn new() -> (Arc<Self>, GatePin) {
            let (gate, pin) = RevisionGate::pair(0);
            let shared = Arc::new(Self {
                gate,
                dirty: DirtyWord::new(),
                cells: core::array::from_fn(|_| LatestCell::new()),
            });
            (shared, pin)
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
            let (shared, mut pin) = Shared::<2>::new();
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut writers = [CellWriter::<2>::new(), CellWriter::new()];
                for revision in 1..=2 {
                    for (cell, writer) in writers.iter_mut().enumerate() {
                        writer.write(
                            &control_shared.cells[cell],
                            &control_shared.dirty,
                            u32::try_from(cell).expect("small"),
                            pin.stamp(revision).expect("a legal revision"),
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
            let (shared, mut pin) = Shared::<1>::new();
            let mut writer = CellWriter::<2>::new();
            writer.write(
                &shared.cells[0],
                &shared.dirty,
                0,
                pin.stamp(1).expect("a legal revision"),
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
                        pin.stamp(revision).expect("a legal revision"),
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
            let (shared, mut pin) = Shared::<1>::new();
            let control_shared = Arc::clone(&shared);
            let control = thread::spawn(move || {
                let mut writer = CellWriter::<2>::new();
                for revision in 1..=2 {
                    writer.write(
                        &control_shared.cells[0],
                        &control_shared.dirty,
                        0,
                        pin.stamp(revision).expect("a legal revision"),
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
            let (shared, mut pin) = Shared::<1>::new();
            let mut writer = CellWriter::<2>::new();
            writer.write(
                &shared.cells[0],
                &shared.dirty,
                0,
                pin.stamp(1).expect("a legal revision"),
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
                            pin.stamp(revision).expect("a legal revision"),
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
            let plan = || {
                let (gate, pin) = RevisionGate::pair(0);
                let plan = Plan {
                    gate,
                    dirty: DirtyWord::new(),
                    cells: [LatestCell::new()],
                };
                (plan, pin)
            };
            let ((plan_p, mut pin_p), (plan_c, mut pin_c)) = (plan(), plan());
            let both = Arc::new(Both {
                mailbox: AtomicU64::new(0),
                plans: [plan_p, plan_c],
            });
            // P holds X at revision 1, published before render starts.
            let mut writer_p = CellWriter::<2>::new();
            writer_p.write(
                &both.plans[0].cells[0],
                &both.plans[0].dirty,
                0,
                pin_p.stamp(1).expect("a legal revision"),
                value_words(1, 0),
            );
            pin_p.publish(&both.plans[0].gate, 1);
            // C holds a retarget at revision 2, published on C's gate before C is published.
            let mut writer_c = CellWriter::<2>::new();
            writer_c.write(
                &both.plans[1].cells[0],
                &both.plans[1].dirty,
                0,
                pin_c.stamp(2).expect("a legal revision"),
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
                        pin_c.stamp(revision).expect("a legal revision"),
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
                resolved[0],
                u64::from(applied[0].is_some()),
                "P's one write is resolved once if P ran a block, and never otherwise"
            );
            assert_eq!(
                resolved[1], 4,
                "C's writes at 2 to 5 are resolved once each"
            );
        });
    }

    /// L5 (#1603 gate 3): monitor writes race reads. Control publishes a model commit, two
    /// monitor writes and the next model commit on one two-word cell, storing the revision it is
    /// about to publish in a read-modify-write mailbox first. Render takes snapshots and reads.
    /// Every read is whole, is the newest written publication at or below its snapshot, the
    /// snapshot's revision is the revision of its own publication, and it never exceeds the
    /// newest model revision control had begun to publish.
    #[test]
    fn spsc_loom_cells_monitor_writes_race_reads() {
        // `(revision, monitor)` in publication order: publication `i + 1` is `WRITES[i]`.
        const WRITES: [(u64, bool); 4] = [(1, false), (1, true), (1, true), (2, false)];
        let mut builder = loom::model::Builder::new();
        builder.preemption_bound = Some(3);
        builder.check(|| {
            let (shared, mut pin) = Shared::<1>::new();
            let announced = Arc::new(AtomicU64::new(0));
            let control_shared = Arc::clone(&shared);
            let control_announced = Arc::clone(&announced);
            let control = thread::spawn(move || {
                let mut writer = CellWriter::<2>::new();
                for (index, (revision, monitor)) in WRITES.into_iter().enumerate() {
                    if !monitor {
                        control_announced.fetch_max(revision, Ordering::Relaxed);
                    }
                    let stamp = if monitor {
                        pin.stamp_monitor(revision)
                    } else {
                        pin.stamp(revision)
                    }
                    .expect("a legal stamp");
                    let published = stamp.key();
                    assert_eq!(published.publication(), index as u64 + 1);
                    writer.write(
                        &control_shared.cells[0],
                        &control_shared.dirty,
                        0,
                        stamp,
                        value_words(revision, u32::try_from(index).expect("small")),
                    );
                    pin.publish_key(&control_shared.gate, published);
                }
            });
            let mut render = Render::<1>::new();
            let block = |render: &mut Render<1>| {
                let snapshot = shared.gate.snapshot();
                let number = snapshot.publication();
                let expected = usize::try_from(number)
                    .expect("small")
                    .checked_sub(1)
                    .map(|index| (WRITES[index].0, u32::try_from(index).expect("small")));
                assert_eq!(
                    snapshot.revision(),
                    expected.map_or(0, |(revision, _)| revision),
                    "publication {number} names its own revision"
                );
                assert!(
                    snapshot.revision() <= announced.load(Ordering::Relaxed),
                    "snapshot revision {} above the announced model revision",
                    snapshot.revision()
                );
                let pending = render.stage.pending(&shared.dirty);
                render.drain(&shared, snapshot, pending);
                assert_eq!(render.applied[0], expected, "publication {number}");
            };
            block(&mut render);
            block(&mut render);
            control.join().expect("control");
            block(&mut render);
            assert_eq!(render.applied[0], Some((2, 3)));
            assert_eq!(render.superseded + render.applications, 4);
        });
    }
}
