//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

#[cfg(any(test, feature = "test-support"))]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
use builtins::builtin_filter_cutoff_maximum_hz;

#[cfg(any(test, feature = "test-support"))]
use std::sync::Mutex;

use sha2::{Digest, Sha256};

// The whole-plan scalar oracle's per-node strip owners (issue #1059) are the only users of these.
#[cfg(any(test, feature = "test-support"))]
use builtins::DualMonoBlock;
#[cfg(any(test, feature = "test-support"))]
use graph::{GraphBindingBlock, GraphRuntimeProcessor};

use builtins::{
    BuiltinChain, BuiltinFaderBank, BuiltinInputBank, BuiltinLaneSelector, BuiltinMatrixBank,
    BuiltinParameterError, BuiltinParameters, ChannelParameters, FaderMuteBuiltins,
    FaderMuteRampBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins, MeterAccumulator, MeterConfig,
    MeterConfigError, MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap,
    PreparedInputFilterTarget, PreparedMeter, pan_matrix, validate_builtin_filter_cutoff,
};
use effect_contract::{
    BankWidth, ChannelSymmetryWitness, LiveControlRecord, NodeTailBound, ResponseAnalysisError,
    ResponseSnapshotRequest, ResponseSnapshotSummary, SeamSide, SymmetryEvent, TailSamples,
};
pub use engine::realtime::LiveValuesSuperseded;
use engine::realtime::{
    CellRead, CellReader, CellWriter, Consumer, DirtyReader, DirtyWord, LatestCell, LiveSnapshot,
    LiveStamp, PreparedRenderPlan, RenderEnvelope, RenderError, SharedAllocation,
    bounded_spsc_retained_payload,
};
use graph::{
    BuiltinStripOwner, BuiltinStripStage, DependencyLevel, GraphBlockTime,
    GraphBuiltinBankResourceEstimate, GraphNodeId, GraphNodeObserverBinding, GraphObservationBlock,
    GraphPreparedBuiltinBank, GraphPreparedBuiltinBankInfo, GraphPreparedBuiltinBankProcessor,
    GraphPreparedSourceSet, GraphRuntimeBindings, GraphRuntimeObserver, PreparedGraphPlan,
    StableGraphId, TimelineSpan, TrackStage,
};
use lane::Backend;
use rack::{AoSoaScratch, BankSlotKey, RackLocation, RackProgram};
use rack_compiler::{CohortCandidate, CohortLevel, CohortPoolClass, plan_bank_groups};
use session::{CompiledSession, MatrixOrPan, StripRef, Track};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_total_retained_payload_bytes: u64,
    pub maximum_total_meter_items: u64,
    pub maximum_total_meter_bytes: u64,
    pub maximum_single_allocation_bytes: u64,
    pub maximum_meter_streams: u64,
    pub maximum_period_frames: u32,
    pub maximum_peak_hold_frames: u32,
    pub maximum_smoothing_samples: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeterRequest {
    pub handle: MeterHandle,
    pub track_id: String,
    pub tap: MeterTap,
    pub config: MeterConfig,
}

/// A meter request with an explicit computation/presence contract.
///
/// Existing [`MeterRequest`] entry points retain their full-statistics behavior; callers use this
/// wrapper only when they intentionally select a subset.
#[derive(Clone, Debug, PartialEq)]
pub struct SelectedMeterRequest {
    pub request: MeterRequest,
    pub metrics: MeterMetricSet,
}

/// One live-control record for a track's smoothed 2x2 matrix/pan stage (issue #137 D1).
///
/// # Why the matrix stage, and what the sentence here used to say
///
/// `BUILTIN_PARAMETER_DESCRIPTORS` is the builtin parameter ABI, and it is explicit about which
/// builtin parameters may move after preparation. When #137 D1 wrote this channel, the four
/// `matrix_ll/lr/rl/rr` rows were the **only** ones declaring
/// `BuiltinParameterUpdateRate::BlockTarget` and this comment said so, listing every other row as
/// `PreparedOnly`. That list has been overtaken twice and is not maintained here any more: #140 B
/// made `fader_db` and `mute` live ([`TrackFaderRecord`]), and #210 phase 3 made `trim_db` and
/// `polarity_invert` live ([`TrackInputRecord`]), and tranche2 adds prepared filter targets to
/// that same queue. The row that remains `PreparedOnly` is `delay_samples`, and the descriptor
/// table is the authority rather than this paragraph.
///
/// What is still true of *this* record, and is the reason it is its own type: it addresses the
/// matrix stage and nothing else, because a bounded SPSC queue has exactly one consumer and the
/// matrix stage is a different graph node from the fader and the input chain.
///
/// The record is `Copy` and fixed-size. Since #1312 it is the value written to the strip's
/// latest-target matrix cell ([`MatrixCells::write`]), not an element of a queue: the cell holds
/// the newest value, the render-side drain allocates nothing, and a write has no room to want.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackControlRecord {
    /// New 2x2 target, already domain-checked by the producer.
    pub matrix: Matrix2x2,
    /// Ramp length in sample updates for this retarget.
    pub smoothing_samples: u32,
}

/// One live-control fader or mute record for a track's fader/mute stage (issue #140 B).
///
/// # Why this is a second record type and a second set of cells
///
/// The matrix stage and the fader stage are two different graph nodes, so one lane cannot serve
/// both -- a lane has exactly one reader, and that reader is the processor bound to the node.
/// `TrackControlRecord` therefore stays exactly the 20-byte matrix record #137 froze, and this
/// is written to its own four latest-target cells (fader left and right, mute left and right,
/// [`FaderCells::write`]) read by `TrackStage::PostFader` (#1312: it was a bounded queue before).
///
/// `fader_db` and `mute` declare `BuiltinParameterUpdateRate::BlockTarget` in
/// `BUILTIN_PARAMETER_DESCRIPTORS`. (This paragraph read "still declare `PreparedOnly`" until
/// the rows were flipped; the ABI table is the authority.) The *prepared* fader section,
/// `FaderMuteBuiltins`, genuinely has no post-preparation write path and is unchanged; what #140
/// adds is a distinct live section, `FaderMuteRampBuiltins`, bound only where live controls asked
/// for one, and the parameter-metadata `liveUpdatable` flag is what tells a caller which of the
/// two a session is running.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackFaderRecord {
    /// Retarget the addressed lanes' fader gain, in decibels, over an explicit ramp window.
    FaderDb {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// New fader value in decibels, already domain-checked by the producer.
        db: f32,
        /// Ramp length in sample updates for this retarget.
        smoothing_samples: u32,
    },
    /// Set or clear the addressed lanes' mute, as a retarget of the same gain to `0`.
    Mute {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// The new mute state.
        muted: bool,
        /// Ramp length in sample updates for the fade.
        smoothing_samples: u32,
    },
}

/// One live-control trim, polarity or prepared-filter record for a track's **input** stage.
///
/// # Why this is a third record type and a third set of cells
///
/// The same reason [`TrackFaderRecord`] is a second one, one stage earlier: a stage's cells have
/// exactly one reader, and that reader is whoever renders the addressed stage. The input section
/// is a different stage from the fader, on a different node and in a different bank, so it needs
/// its own cells. `TrackControlRecord` and `TrackFaderRecord` are unchanged. Since #1346 it is the
/// value written to the strip's latest-target input cells ([`InputCells::write`]), not an element
/// of a queue: a trim or polarity value lands in its channel's cell, a prepared filter target in
/// the cell of its section and channel, and render applies the newest value of each.
///
/// # What `Both` means, and how the witness reads it
///
/// This record type is **upstream of the seam**, so [`ChannelSymmetryWitness::admit`] reads it, and
/// a pair of per-lane edits that changed both channels identically at one block boundary must not
/// retire the track's mono collapse for the life of the plan. `BuiltinLaneSelector::Both` says
/// exactly that and is admitted as [`SymmetryEvent::Preserve`], which is what
/// `SymmetryEvent::Preserve`'s own documentation says a both-channel retarget is. A cell holds a
/// value per channel, so render rebuilds the record from the cells' final contents (#1346 D3): when
/// both channels' cells of a kind were applied in one block with equal words, the drain admits and
/// applies one `Both` record, and any other pair, or a lone channel, is admitted as
/// `Desymmetrize`. So `Both` written as one record and the two channels written as two equal
/// records read alike, and no write order changes what the witness sees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackInputRecord {
    /// Retarget the addressed lanes' input trim, in decibels, over an explicit ramp window.
    ///
    /// The lane's polarity is preserved: `trim_db` and `polarity_invert` are two parameters that
    /// share one coefficient, and a gain ride must not silently clear a flip.
    TrimDb {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// New trim value in decibels, domain-checked against `trim_db`'s own declared range by
        /// the producer and again by the bank.
        db: f32,
        /// Ramp length in sample updates for this retarget.
        smoothing_samples: u32,
    },
    /// Set or clear the addressed lanes' polarity, as a retarget of the same coefficient to its
    /// own negation.
    ///
    /// The declick is the trim ramp's: the linear ramp carries the coefficient through zero over
    /// the requested window. There is no second DSP path.
    PolarityInvert {
        /// Which lane(s) this record addresses.
        lanes: BuiltinLaneSelector,
        /// The new polarity state.
        inverted: bool,
        /// Ramp length in sample updates for the flip.
        smoothing_samples: u32,
    },
    /// Apply one already-prepared HPF or LPF target to the addressed lanes.
    ///
    /// The target carries the lane's final pair for response/readback association, while its
    /// section and six coefficients are the only words the render owner applies. Preparation
    /// validates the pair and designs these words off the render thread.
    PreparedFilter {
        /// One section target, addressed to one or both channels.
        target: PreparedInputFilterTarget,
    },
}

impl LiveControlRecord for TrackInputRecord {
    /// The input chain is the first stage of the strip, before the fader and the matrix: a
    /// collapsed track runs it **once**, so every record on this queue gates the collapse.
    const SEAM: SeamSide = SeamSide::UpstreamOfSeam;

    fn symmetry_event(&self) -> SymmetryEvent {
        // Exhaustive, with no wildcard arm on purpose: a new variant is a compile error here,
        // which is the structural half of the hook rule.
        let lanes = match *self {
            Self::TrimDb { lanes, .. } | Self::PolarityInvert { lanes, .. } => lanes,
            Self::PreparedFilter { target } => target.lanes,
        };
        match lanes {
            BuiltinLaneSelector::Both => SymmetryEvent::Preserve,
            BuiltinLaneSelector::Left | BuiltinLaneSelector::Right => SymmetryEvent::Desymmetrize,
        }
    }
}

/// One requested live-control channel, addressed by session track ID (issue #137 D1).
#[derive(Clone, Debug, PartialEq)]
pub struct TrackControlRequest {
    /// Session-stable track identity. It must name a track of the compiled session.
    pub track_id: String,
    /// The session's `live_values_superseded` count, which this strip's cell readers add to
    /// (#1312 D2). Every request of one session carries clones of one count.
    pub superseded: LiveValuesSuperseded,
    /// Whether the strip also gets the live input trim/polarity/filter lane (#1254 D1).
    ///
    /// `true` attaches all three lanes, as every request did before #1254. `false` attaches only
    /// the matrix and fader lanes: no input cells are created or charged, the producer's
    /// [`TrackControlProducer::input`] is `None`, and the strip keeps its chain's own builtin
    /// tail, because only a live input target can enable a filter the session left off. Every lane
    /// is a latest-target cell with no depth (#1312, #1346), so a request names no queue size.
    pub input_lane: bool,
}

/// Words of one fader or mute cell: the value (`fader_db` bits, or `0`/`1`) and the ramp window.
const FADER_CELL_WORDS: usize = 2;
/// Words of the matrix cell: `ll`, `lr`, `rl`, `rr` bits and the ramp window.
const MATRIX_CELL_WORDS: usize = 5;
/// Cells of one fader stage: fader left, fader right, mute left, mute right. The cell index is
/// its dirty bit, and the stage's canonical drain order (#1312 D4).
const FADER_CELLS: usize = 4;
const FADER_LEFT: usize = 0;
const MUTE_LEFT: usize = 2;

/// What one strip's fader/mute stage shares between its writer (control) and its reader (render):
/// four latest-target cells and their one dirty word (#1312 D3).
struct FaderLane {
    cells: [LatestCell<FADER_CELL_WORDS>; FADER_CELLS],
    dirty: DirtyWord,
}

impl FaderLane {
    fn new() -> Self {
        Self {
            cells: core::array::from_fn(|_| LatestCell::new()),
            dirty: DirtyWord::new(),
        }
    }
}

/// What one strip's matrix/pan stage shares between its writer and its reader: one cell and its
/// dirty word (#1312 D3).
struct MatrixLane {
    cell: LatestCell<MATRIX_CELL_WORDS>,
    dirty: DirtyWord,
}

impl MatrixLane {
    fn new() -> Self {
        Self {
            cell: LatestCell::new(),
            dirty: DirtyWord::new(),
        }
    }
}

/// Control's write end of one strip's fader/mute cells (#1312 D5). A write cannot fail and never
/// waits.
pub struct FaderCells {
    lane: Arc<FaderLane>,
    writers: [CellWriter<FADER_CELL_WORDS>; FADER_CELLS],
    /// The offsets cells' write ends of the strip's automated fader channels (#1649 D4),
    /// `[left, right]`; `None` (no allocation) on a strip with no stored fader automation.
    offsets: Option<Box<[Option<OffsetsWriter>; 2]>>,
}

/// Control's write end of one automated fader channel's offsets cell (#1649 D4).
struct OffsetsWriter {
    lane: Arc<OffsetsLane>,
    writer: CellWriter<OFFSETS_CELL_WORDS>,
}

/// One fader event lane's offsets-cell value (#1649 D4, #1655 D3): the lane's offset sum `S`
/// (`controls::EffectiveStrip::offsets_sum`), the length of the jump a change of it asks for (the
/// lane's fader length of the edit, `LevelRules::lengths`) and, on an event lane whose own fader
/// is not automated, the member's static fader dB `own_db`. A write stores them in one slot, so
/// render never reads the halves of two sums or a sum with another `own_db`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaderOffsetsRecord {
    /// `S`.
    pub offsets_sum: f64,
    /// The jump's length in samples; `0` is an exact set.
    pub smoothing_samples: u32,
    /// The member's static fader dB (#1655 D3), on a fader event lane whose own fader is not
    /// automated: its hand fader move, which the lane composes with `S` and the reaching VCA
    /// curves. `None` keeps the lane's `own_db`; a lane whose own fader is automated reads no
    /// `own_db` (its own value is its curve). A `Some` is finite.
    pub own_db: Option<f32>,
}

/// The `own_db` word of a record with no `own_db` (a quiet NaN: a `Some` is finite).
const NO_OWN_DB: u32 = 0x7fc0_0000;

impl FaderOffsetsRecord {
    /// The cell's words: `S`'s `f64` bits, low word first, the ramp word, then the `own_db` bits
    /// ([`NO_OWN_DB`] for `None`).
    const fn words(self) -> [u32; OFFSETS_CELL_WORDS] {
        let bits = self.offsets_sum.to_bits();
        let own = match self.own_db {
            Some(own) => own.to_bits(),
            None => NO_OWN_DB,
        };
        [
            bits as u32,
            (bits >> 32) as u32,
            self.smoothing_samples,
            own,
        ]
    }

    /// The record a slot's words hold.
    const fn from_words(words: [u32; OFFSETS_CELL_WORDS]) -> Self {
        let own = f32::from_bits(words[3]);
        Self {
            offsets_sum: f64::from_bits((words[0] as u64) | ((words[1] as u64) << 32)),
            smoothing_samples: words[2],
            own_db: if own.is_nan() { None } else { Some(own) },
        }
    }
}

impl FaderCells {
    /// Whether the strip's fader channel `channel` (`0` left, `1` right) has an offsets cell: its
    /// fader is automated in the plan these cells belong to (#1649). A host checks every
    /// [`Self::write_offsets`] of a transaction with it before its first write.
    #[must_use]
    pub fn has_offsets(&self, channel: usize) -> bool {
        self.offsets
            .as_deref()
            .and_then(|writers| writers.get(channel))
            .is_some_and(Option::is_some)
    }

    /// Write `record` as the newest value of channel `channel`'s offsets cell, the whole slot at
    /// once (#1312 D1, #1649 D4), stamped with the revision of the transaction that commits it. A
    /// write cannot fail; a channel with no offsets cell ([`Self::has_offsets`] is `false`) is a
    /// caller fault the caller checked before its first write, and writes nothing.
    pub fn write_offsets(
        &mut self,
        stamp: LiveStamp<'_>,
        channel: usize,
        record: FaderOffsetsRecord,
    ) {
        let writer = self
            .offsets
            .as_deref_mut()
            .and_then(|writers| writers.get_mut(channel))
            .and_then(Option::as_mut);
        debug_assert!(
            writer.is_some(),
            "an offsets write to a channel with no cell"
        );
        if let Some(OffsetsWriter { lane, writer }) = writer {
            writer.write(&lane.cell, &lane.dirty, 0, stamp, record.words());
        }
    }

    /// Give these cells the write ends of the strip's offsets cells (#1649 D4), `[left, right]`;
    /// nothing when no channel is automated. Control side: it allocates one box.
    fn attach_offsets(&mut self, cells: &[Option<PreparedFaderCell>; 2]) {
        if cells.iter().all(Option::is_none) {
            return;
        }
        self.offsets = Some(Box::new(core::array::from_fn(|channel| {
            cells[channel].as_ref().map(|cell| OffsetsWriter {
                lane: Arc::clone(&cell.offsets),
                writer: CellWriter::new(),
            })
        })));
    }

    /// Write `record` as the newest value of the cell(s) it addresses, stamped with the revision
    /// of the transaction that commits it. A `Both` record writes the left and the right cell.
    ///
    /// The caller publishes the stamp's revision on the plan's gate after every write of the
    /// transaction (#1312 D7); a value is not applied before that.
    pub fn write(&mut self, stamp: LiveStamp<'_>, record: TrackFaderRecord) {
        let (base, words, lanes) = match record {
            TrackFaderRecord::FaderDb {
                lanes,
                db,
                smoothing_samples,
            } => (FADER_LEFT, [db.to_bits(), smoothing_samples], lanes),
            TrackFaderRecord::Mute {
                lanes,
                muted,
                smoothing_samples,
            } => (MUTE_LEFT, [u32::from(muted), smoothing_samples], lanes),
        };
        let cells = match lanes {
            BuiltinLaneSelector::Left => base..=base,
            BuiltinLaneSelector::Right => base + 1..=base + 1,
            BuiltinLaneSelector::Both => base..=base + 1,
        };
        for index in cells {
            // `index` is below `FADER_CELLS`, so it is a valid dirty bit.
            self.writers[index].write(
                &self.lane.cells[index],
                &self.lane.dirty,
                index as u32,
                stamp,
                words,
            );
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FaderCells {
    /// Test-only: how many writes each cell has taken (fader left, fader right, mute left, mute
    /// right). Valid while no write is in flight, which a quiescent test thread guarantees.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_counts(&self) -> [u64; FADER_CELLS] {
        core::array::from_fn(|index| {
            CellReader::new()
                .peek_unread(&self.lane.cells[index])
                .map_or(0, |(_, sequence)| sequence)
        })
    }

    /// Test-only: how many writes each offsets cell has taken (#1649), `[left, right]`; `None`
    /// for a channel with no cell. Valid while no write is in flight.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_offsets_write_counts(&self) -> [Option<u64>; 2] {
        core::array::from_fn(|channel| {
            let writer = self.offsets.as_deref()?.get(channel)?.as_ref()?;
            Some(
                CellReader::new()
                    .peek_unread(&writer.lane.cell)
                    .map_or(0, |(_, sequence)| sequence),
            )
        })
    }
}

/// Control's write end of one strip's matrix/pan cell (#1312 D5).
pub struct MatrixCells {
    lane: Arc<MatrixLane>,
    writer: CellWriter<MATRIX_CELL_WORDS>,
}

impl MatrixCells {
    /// Write `record` as the newest matrix target, stamped with the revision of the transaction
    /// that commits it. A write cannot fail and never waits.
    pub fn write(&mut self, stamp: LiveStamp<'_>, record: TrackControlRecord) {
        let matrix = record.matrix;
        self.writer.write(
            &self.lane.cell,
            &self.lane.dirty,
            0,
            stamp,
            [
                matrix.ll.to_bits(),
                matrix.lr.to_bits(),
                matrix.rl.to_bits(),
                matrix.rr.to_bits(),
                record.smoothing_samples,
            ],
        );
    }
}

#[cfg(any(test, feature = "test-support"))]
impl MatrixCells {
    /// Test-only: how many writes the cell has taken. Valid while no write is in flight.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_count(&self) -> u64 {
        CellReader::new()
            .peek_unread(&self.lane.cell)
            .map_or(0, |(_, sequence)| sequence)
    }
}

/// Render's read end of one strip's fader/mute cells: the cells' readers, the stage's deferred
/// mask and the session counter (#1312 D3).
struct FaderReader {
    lane: Arc<FaderLane>,
    cells: [CellReader; FADER_CELLS],
    dirty: DirtyReader,
    superseded: LiveValuesSuperseded,
    /// The carry marks (#1630 f, D15-2); see [`CarryMarks`].
    marks: CarryMarks,
}

/// What one reader knows about values a plan swap carried (#1630 f, decision 15 D15-2). Bit `n`
/// is the cell with dirty bit `n`.
///
/// `applied` is the cells the reader's last drain applied. The swap's carry drain on the
/// predecessor sets it, and the successor's adopting lane takes it as `carried`: the cells whose
/// value the swap carried into lane state and no sample has rendered yet. The successor's first
/// drain, in the adopting block before any sample, counts each carried cell it applies a value of
/// its own to as one superseded value (the carried value was replaced unheard, as an effect lane's
/// carried write is), and clears `carried`, whether or not it applied anything: from the second
/// block on, the carried value has been heard.
#[derive(Clone, Copy, Default)]
struct CarryMarks {
    carried: u16,
    applied: u16,
}

// REALTIME_POLICY_BEGIN
impl CarryMarks {
    /// Start a drain: take the carried cells and forget the last drain's applied ones.
    fn begin(&mut self) -> u16 {
        self.applied = 0;
        core::mem::take(&mut self.carried)
    }

    /// The drain applied cell `bit`; a carried value of that cell was replaced unheard.
    fn applied(&mut self, bit: usize, carried: u16, superseded: &LiveValuesSuperseded) {
        let mask = 1_u16 << bit;
        self.applied |= mask;
        if carried & mask != 0 {
            superseded.add_carried(1);
        }
    }

    /// The successor's adopting lane takes the cells the predecessor's carry drain applied.
    fn adopt(&mut self, predecessor: &Self) {
        self.carried = predecessor.applied;
    }
}

/// An adopted lane's reader takes its predecessor reader's carry marks; a lane without a reader on
/// either side has none to take.
fn adopt_carry_marks<R>(
    into: Option<&mut Option<R>>,
    from: Option<&Option<R>>,
    marks_mut: impl FnOnce(&mut R) -> &mut CarryMarks,
    marks: impl FnOnce(&R) -> &CarryMarks,
) {
    if let (Some(Some(into)), Some(Some(from))) = (into, from) {
        marks_mut(into).adopt(marks(from));
    }
}
// REALTIME_POLICY_END

/// Which cell kind a drain applies.
#[derive(Clone, Copy)]
enum FaderKind {
    Fader,
    Mute,
}

// REALTIME_POLICY_BEGIN
impl FaderReader {
    /// Read the stage's pending cells under `snapshot` and apply them in the canonical order
    /// (#1312 D4): fader before mute, left before right. When both channel cells of a kind were
    /// applied in this call with equal words, `apply` is called once with `Both`; a cell that
    /// read `Unchanged` is not paired.
    ///
    /// Every applied value is applied even if an earlier one is refused; the first refusal is
    /// returned after the last. Each applied read adds the writes it replaced to the session
    /// counter (`read_counted`). It allocates nothing and takes no lock. `Ok(true)`: no cell was
    /// pending, so the read found nothing to do (issue #1778's quiet walk).
    fn apply_latest(
        &mut self,
        snapshot: LiveSnapshot,
        mut apply: impl FnMut(
            FaderKind,
            BuiltinLaneSelector,
            [u32; FADER_CELL_WORDS],
        ) -> Result<(), BuiltinParameterError>,
    ) -> Result<bool, BuiltinParameterError> {
        let carried = self.marks.begin();
        let pending = self.dirty.pending(&self.lane.dirty);
        if pending == 0 {
            return Ok(true);
        }
        let mut applied = [None; FADER_CELLS];
        for (index, slot) in applied.iter_mut().enumerate() {
            if pending & (1 << index) == 0 {
                continue;
            }
            if let CellRead::Applied { words, .. } = self.cells[index].read_counted(
                &self.lane.cells[index],
                snapshot,
                &mut self.dirty,
                index as u32,
                &self.superseded,
            ) {
                self.marks.applied(index, carried, &self.superseded);
                *slot = Some(words);
            }
        }
        let mut first_error = Ok(());
        for (kind, base) in [(FaderKind::Fader, FADER_LEFT), (FaderKind::Mute, MUTE_LEFT)] {
            let mut run = |selector, words| {
                // Every value is applied; the first refusal is the one returned.
                first_error = first_error.and(apply(kind, selector, words));
            };
            match (applied[base], applied[base + 1]) {
                (Some(left), Some(right)) if left == right => {
                    run(BuiltinLaneSelector::Both, left);
                }
                (left, right) => {
                    if let Some(left) = left {
                        run(BuiltinLaneSelector::Left, left);
                    }
                    if let Some(right) = right {
                        run(BuiltinLaneSelector::Right, right);
                    }
                }
            }
        }
        first_error.map(|()| false)
    }
}
// REALTIME_POLICY_END

/// Render's read end of one strip's matrix/pan cell.
struct MatrixReader {
    lane: Arc<MatrixLane>,
    cell: CellReader,
    dirty: DirtyReader,
    superseded: LiveValuesSuperseded,
    /// The carry marks (#1630 f, D15-2); see [`CarryMarks`].
    marks: CarryMarks,
}

// REALTIME_POLICY_BEGIN
impl MatrixReader {
    /// Read the stage's pending cell under `snapshot` and apply its newest value in range, if
    /// any. The first refusal is returned; the applied read adds the writes it replaced to the session counter.
    /// `Ok(true)`: the cell was not pending (issue #1778's quiet walk).
    fn apply_latest(
        &mut self,
        snapshot: LiveSnapshot,
        mut apply: impl FnMut(Matrix2x2, u32) -> Result<(), BuiltinParameterError>,
    ) -> Result<bool, BuiltinParameterError> {
        let carried = self.marks.begin();
        let pending = self.dirty.pending(&self.lane.dirty);
        if pending == 0 {
            return Ok(true);
        }
        let CellRead::Applied { words, .. } = self.cell.read_counted(
            &self.lane.cell,
            snapshot,
            &mut self.dirty,
            0,
            &self.superseded,
        ) else {
            return Ok(false);
        };
        self.marks.applied(0, carried, &self.superseded);
        apply(
            Matrix2x2 {
                ll: f32::from_bits(words[0]),
                lr: f32::from_bits(words[1]),
                rl: f32::from_bits(words[2]),
                rr: f32::from_bits(words[3]),
            },
            words[4],
        )
        .map(|()| false)
    }
}
// REALTIME_POLICY_END

/// One strip's fader/mute lane, as its two ends: control's writers and render's reader.
fn fader_lane(superseded: &LiveValuesSuperseded) -> (FaderCells, FaderReader) {
    let lane = Arc::new(FaderLane::new());
    (
        FaderCells {
            lane: Arc::clone(&lane),
            writers: core::array::from_fn(|_| CellWriter::new()),
            offsets: None,
        },
        FaderReader {
            lane,
            cells: core::array::from_fn(|_| CellReader::new()),
            dirty: DirtyReader::new(),
            superseded: superseded.clone(),
            marks: CarryMarks::default(),
        },
    )
}

/// One strip's matrix/pan lane, as its two ends.
fn matrix_lane(superseded: &LiveValuesSuperseded) -> (MatrixCells, MatrixReader) {
    let lane = Arc::new(MatrixLane::new());
    (
        MatrixCells {
            lane: Arc::clone(&lane),
            writer: CellWriter::new(),
        },
        MatrixReader {
            lane,
            cell: CellReader::new(),
            dirty: DirtyReader::new(),
            superseded: superseded.clone(),
            marks: CarryMarks::default(),
        },
    )
}

/// Words of a fader event lane's offsets cell (#1646 D1, written by #1649): the lane's offset sum
/// `S` as the low and high words of its `f64` bits, the ramp word (the jump length a VCA edit asks
/// for), then the `own_db` word (#1655 D3, batch 19 amendment: a slot of four words and one of
/// three are both 32 bytes, so every offsets cell takes it; unread on a lane whose own fader is
/// automated).
const OFFSETS_CELL_WORDS: usize = 4;

/// What one automated fader channel's offsets cell shares between its writer (control,
/// [`FaderCells::write_offsets`]) and its reader (render, [`LaneFaderAutomation::read_offsets`]):
/// one #1312 latest-target cell and its dirty word (#1649).
///
/// The cell is created unwritten at preparation; its reader holds the prepared words until its
/// first applied write (#1646 departure 5, as #1639's jump-length cell does).
struct OffsetsLane {
    cell: LatestCell<OFFSETS_CELL_WORDS>,
    dirty: DirtyWord,
}

impl OffsetsLane {
    fn new() -> Self {
        Self {
            cell: LatestCell::new(),
            dirty: DirtyWord::new(),
        }
    }
}

impl core::fmt::Debug for OffsetsLane {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OffsetsLane").finish_non_exhaustive()
    }
}

/// One automated fader channel as a fader owner holds it (#1646 D1): its cell in the plan's
/// program, its event state and its offsets cell with the words render holds until the first
/// write.
///
/// The event state lives here, beside the lane's ramp state, so the stage's carry moves it with
/// the lane (#1648 extends [`FaderBankProcessor::adopt_lane`]).
struct AutomatedFaderCell {
    /// The channel's curve: its own cell, or the composed curve of a fader event lane an automated
    /// VCA `offset_db` lane reaches (#1655 V2).
    curve: FaderCurve,
    /// The offsets cell (#1649), shared with the strip's [`FaderCells`] when the strip has a live
    /// control lane.
    offsets: Arc<OffsetsLane>,
    /// Render's read end of the offsets cell and its dirty word.
    offsets_reader: CellReader,
    offsets_dirty: DirtyReader,
    /// `S`: the prepared one until the offsets cell's first applied write, then the newest one
    /// render read; every event's gain is composed with it (#1646 D1, #1647 D1.4).
    offsets_sum: f64,
    /// The ramp word of the newest value read: the length of the jump the last change of `S`
    /// took (#1649 D3). Prepared as the session's fader length (`LiveRampRow::VcaOffset`, #1054).
    offsets_ramp: u32,
    /// The held-ramp obligation (#1647 D1.6, batch 13b amendment).
    obligation: HeldRampObligation,
    /// What the cell's first block after a plan swap that carried its lane owes (#1648 D2).
    adoption: Adoption,
    /// The newest offsets record the plan-swap carry drain read from this cell and no block has
    /// applied yet (#1648 D2, U3 fold): the successor's adopting cell takes it, and its first
    /// block applies it through [`Self::offsets_jump`] as the uninterrupted block would.
    carried_record: Option<FaderOffsetsRecord>,
    /// At an adopted cell until its first block's offsets reads: the `S` this plan was prepared
    /// with. The adopted cell holds the carried `S` (the one its stage heard), and the carried and
    /// the successor's own records move it. With no record of the cell's own (which is newer than
    /// the preparation), an `S` left unequal to this one takes the adoption rule
    /// ([`Adoption::Compare`]).
    adopted_sum: Option<f64>,
}

/// The curve of one fader event channel (#1646 D1, #1655 V2).
#[derive(Debug)]
enum FaderCurve {
    /// The channel's own fader cell alone: the lane is automated and no automated VCA `offset_db`
    /// lane reaches it. Its events are the cell's [`automation::CellEvents`], as before #1655, so
    /// no bit of a session without VCA automation moves.
    Cell {
        /// The cell's index in the plan's [`automation::Program`].
        cell: u32,
        /// Seeded with the prepared curve value, so the first grid event on a flat curve emits
        /// nothing.
        state: automation::CellState,
    },
    /// The lane's composed curve (R19-2 = (b)): one output over its own curve (when its fader is
    /// automated) and every reaching automated VCA `offset_db` curve.
    Composed(Box<FaderComposite>),
}

/// The composed curve of a fader event lane that automated VCA `offset_db` lanes reach (#1655 D2,
/// R19-2): one [`automation::Composed`] output whose cursors are the lane's own fader cell (when
/// its fader is automated) and then each reaching automated VCA lane's cell, in ascending VCA ID.
/// Its value is the lane's fader dB `vca_compose_db(own, S + c_1 + ... + c_m)` (the `f64` sum in
/// D2's order: `S` first, then the curves in cursor order), `own` the own curve's value or the
/// member's static `own_db` (D3). The output keeps one cell's hold rule
/// ([`automation::Composed::with_held_grid`]), so a step of one curve keeps its jump length while
/// another rides, and one cell's jump rule ([`automation::Composed::with_jump_release`], #1655
/// U3 fold 1), so a jump the lane emits replaces the hold and the deferral the lane put on every
/// cursor; every other rule of the single cell (the held-ramp obligation, the R17-10 release, the
/// adoption and its deferred jump, #1711) acts on it through the same driver, applied to every
/// cursor.
#[derive(Debug)]
struct FaderComposite {
    walk: automation::Composed<f32>,
    /// Cursor 0 is the lane's own fader cell.
    own_curve: bool,
    /// The member's static fader dB (D3), the lane's own value when `own_curve` is `false`: the
    /// prepared one until an offsets record carries another.
    own_db: f32,
    /// At an adopted channel until its first block's offsets reads: the `own_db` this plan was
    /// prepared with (the rule of [`AutomatedFaderCell::adopted_sum`]).
    adopted_own_db: Option<f32>,
    /// Every cursor's history belongs to the carried stage. A changed layout or a preparation
    /// backstop invalidates it until the first block applies the ordinary adoption rule.
    cursor_history_valid: bool,
}

/// The curve of one mute event channel (#1651 D1, #1655 V2).
#[derive(Debug)]
enum MuteCurve {
    /// The channel's own mute cell alone, as before #1655.
    Cell {
        /// The cell's index in the plan's [`automation::Program`].
        cell: u32,
        /// Seeded with the prepared curve value, so a fresh lane's first block emits nothing.
        state: automation::CellState,
    },
    /// The lane's composed curve: one output over its own mute cell (when its mute is automated)
    /// and every reaching automated VCA `mute` lane's cell, in ascending VCA ID; its value is
    /// `own || v_1 || ... || v_m` (D4). The stage mute is that value `|| terms`.
    Composed(Box<MuteComposite>),
}

/// The composed mute curve of a mute event lane that automated VCA `mute` lanes reach (#1655 D4).
#[derive(Debug)]
struct MuteComposite {
    walk: automation::Composed<bool>,
    /// Cursor 0 is the lane's own mute cell.
    own_curve: bool,
}

impl FaderCurve {
    /// The prepared channel: its own cell alone when no automated VCA lane reaches it, else the
    /// composed curve over its cells, seeded with the lane's prepared fader dB. Control side: a
    /// composed channel allocates its cursor table.
    fn prepared(
        program: &automation::Program,
        cell: &PreparedFaderCell,
    ) -> Result<Self, automation::ComposeError> {
        if let (Some(own), true) = (cell.cell, cell.vca.is_empty()) {
            let mut state = automation::CellState::new();
            state.seed_target(cell.value);
            return Ok(Self::Cell { cell: own, state });
        }
        let cells: Vec<(u32, automation::CursorKind)> = cell
            .cell
            .into_iter()
            .chain(cell.vca.iter().copied())
            .map(|cell| (cell, automation::CursorKind::Value))
            .collect();
        let mut walk =
            automation::Composed::new(program, &cells, controls::FADER_CELL_RULE.completion)?
                .with_held_grid()
                .with_jump_release();
        // The prepared value, composed once inside preparation's `CanonicalFpEnv` (D2, R17-7).
        walk.seed_target(cell.fader_db);
        Ok(Self::Composed(Box::new(FaderComposite {
            walk,
            own_curve: cell.cell.is_some(),
            own_db: cell.own_db,
            adopted_own_db: None,
            cursor_history_valid: true,
        })))
    }
}

impl MuteCurve {
    /// The prepared channel: its own cell alone when no automated VCA mute lane reaches it, else
    /// the composed curve over its cells, seeded with the prepared curve mute. Control side.
    fn prepared(
        program: &automation::Program,
        cell: &PreparedMuteCell,
    ) -> Result<Self, automation::ComposeError> {
        if let (Some(own), true) = (cell.cell, cell.vca.is_empty()) {
            let mut state = automation::CellState::new();
            state.seed_target(cell.value);
            return Ok(Self::Cell { cell: own, state });
        }
        let cells: Vec<(u32, automation::CursorKind)> = cell
            .cell
            .into_iter()
            .chain(cell.vca.iter().copied())
            .map(|cell| (cell, automation::CursorKind::Gate))
            .collect();
        let mut walk =
            automation::Composed::new(program, &cells, controls::MUTE_CELL_RULE.completion)?;
        walk.seed_target(cell.curve_muted);
        Ok(Self::Composed(Box::new(MuteComposite {
            walk,
            own_curve: cell.cell.is_some(),
        })))
    }
}

// REALTIME_POLICY_BEGIN
/// A composed lane's fader dB (#1655 D2): `vca_compose_db(own, S + c_1 + ... + c_m)`, the `f64`
/// sum in order, `S` first. Render runs it inside the render entry's `CanonicalFpEnv` (R17-7).
fn composed_db(own: f32, offsets_sum: f64, curves: impl Iterator<Item = f32>) -> f32 {
    let sum = curves.fold(offsets_sum, |sum, value| sum + f64::from(value));
    session::vca_compose_db(own, sum)
}

/// Copy the cursor states and heard values of `from` into `into` when both tables have one
/// layout (the same own cursor and as many VCA cursors: a continued channel's, whose tables
/// continue bit for bit); a repositioned channel places its cursors again.
fn carry_cursors<T: automation::Composable>(
    into: &mut automation::Composed<T>,
    from: &automation::Composed<T>,
) -> bool {
    if into.cursors().len() != from.cursors().len() {
        return false;
    }
    for (cursor, carried) in into.cursors_mut().iter_mut().zip(from.cursors()) {
        cursor.carry_from(carried);
    }
    true
}

impl FaderComposite {
    /// The composition of the heard values with `S = offsets_sum`.
    fn composition(&self, offsets_sum: f64) -> impl FnMut(automation::Heard<'_>) -> f32 + use<> {
        let (own_curve, own_db) = (self.own_curve, self.own_db);
        move |heard| {
            let first = usize::from(own_curve);
            let own = if own_curve { heard.value(0) } else { own_db };
            composed_db(
                own,
                offsets_sum,
                (first..heard.len()).map(|index| heard.value(index)),
            )
        }
    }

    /// The lane's value with every curve read by `read(cell)` (a cell the program does not hold
    /// reads its heard value), composed with `offsets_sum`.
    fn compose_with(
        &self,
        program: &automation::Program,
        offsets_sum: f64,
        mut read: impl FnMut(&automation::CellProgram<'_>) -> f32,
    ) -> f32 {
        let cursors = self.walk.cursors();
        let mut values = cursors.iter().map(|cursor| {
            program
                .cell(cursor.cell() as usize)
                .map_or(cursor.heard(), |cell| read(&cell))
        });
        let own = if self.own_curve {
            values.next().unwrap_or(self.own_db)
        } else {
            self.own_db
        };
        composed_db(own, offsets_sum, values)
    }

    /// The composed value toward a ramp started at `start` and completing at `at`: each curve by
    /// [`curve_toward`].
    fn toward(&self, program: &automation::Program, offsets_sum: f64, start: i64, at: i64) -> f32 {
        self.compose_with(program, offsets_sum, |cell| curve_toward(cell, start, at))
    }

    /// The composed value at node time `time`.
    fn value_at(&self, program: &automation::Program, offsets_sum: f64, time: i64) -> f32 {
        self.compose_with(program, offsets_sum, |cell| {
            automation::value_at(cell.segments(), time)
        })
    }

    /// The output's next event of the block begun on the walk, as a cell event whose value is
    /// the composed fader dB.
    fn next_event(
        &mut self,
        program: &automation::Program,
        offsets_sum: f64,
    ) -> Option<automation::Event> {
        let compose = self.composition(offsets_sum);
        self.cursor_history_valid = true;
        self.walk
            .next_event(program, compose)
            .map(|event| automation::Event {
                offset: event.offset,
                value: event.value,
                ramp: event.ramp,
                kind: event.kind,
                held: event.held,
            })
    }

    /// Apply `apply` to every cursor's state: the single cell's rule on each of the lane's curves.
    fn each_state(&mut self, mut apply: impl FnMut(&mut automation::CellState)) {
        for cursor in self.walk.cursors_mut() {
            apply(cursor.state_mut());
        }
    }

    /// Set each cursor's jump length for the block: the fader row's for the own cursor, the VCA
    /// `offset_db` row's for the VCA cursors (#1655, `block_jumps`).
    fn set_jumps(&mut self, fader: u32, vca: u32) {
        let own = usize::from(self.own_curve);
        for (index, cursor) in self.walk.cursors_mut().iter_mut().enumerate() {
            cursor.set_jump(if index < own { fader } else { vca });
        }
    }
}

impl MuteComposite {
    /// `own || v_1 || ... || v_m` of the heard values.
    fn composition(heard: automation::Heard<'_>) -> bool {
        (0..heard.len()).any(|index| curve_mutes(heard.value(index)))
    }

    /// Whether some curve mutes at node time `time`.
    fn mutes_at(&self, program: &automation::Program, time: i64) -> bool {
        self.walk.cursors().iter().any(|cursor| {
            program.cell(cursor.cell() as usize).map_or_else(
                || curve_mutes(cursor.heard()),
                |cell| curve_mutes(automation::value_at(cell.segments(), time)),
            )
        })
    }

    /// Set each cursor's jump length for the block: the mute row's for the own cursor, the VCA
    /// `mute` row's for the VCA cursors.
    fn set_jumps(&mut self, mute: u32, vca: u32) {
        let own = usize::from(self.own_curve);
        for (index, cursor) in self.walk.cursors_mut().iter_mut().enumerate() {
            cursor.set_jump(if index < own { mute } else { vca });
        }
    }
}

impl FaderCurve {
    /// #1648 D2, at a plan swap: take the carried channel's event state (its cell's, or a composed
    /// curve's output and, with one layout, its cursors). A composed channel carried from a lone
    /// cell (a gained VCA curve) takes the cell's ramp in flight and no target; a lone cell
    /// carried from a composed channel (a lost VCA curve) starts fresh. Every such change of
    /// shape repositions (the reaching VCA tables differ, so the join continues no cell).
    /// Allocation-free.
    fn carry(&mut self, from: &Self) {
        match (self, from) {
            (Self::Cell { state, .. }, Self::Cell { state: carried, .. }) => *state = *carried,
            (Self::Composed(curve), Self::Composed(carried)) => {
                curve.cursor_history_valid = curve.own_curve == carried.own_curve
                    && carry_cursors(&mut curve.walk, &carried.walk)
                    && carried.cursor_history_valid;
                curve
                    .walk
                    .set_output(*carried.walk.output_state(), carried.walk.target());
                // The `own_db` the stage heard; the successor's own records move it, and an
                // unequal one with none takes the adoption rule (as `S`, `read_offsets`).
                curve.adopted_own_db = Some(curve.own_db);
                curve.own_db = carried.own_db;
            }
            (Self::Composed(curve), Self::Cell { state: carried, .. }) => {
                curve.walk.set_output(*carried, None);
                curve.cursor_history_valid = false;
            }
            (Self::Cell { state, .. }, Self::Composed(_)) => {
                *state = automation::CellState::new();
            }
        }
    }
}

impl MuteCurve {
    /// #1651 T4, at a plan swap: take the carried channel's event state, by the rule of
    /// [`FaderCurve::carry`]. Allocation-free.
    fn carry(&mut self, from: &Self) {
        match (self, from) {
            (Self::Cell { state, .. }, Self::Cell { state: carried, .. }) => *state = *carried,
            (Self::Composed(curve), Self::Composed(carried)) => {
                if curve.own_curve == carried.own_curve {
                    carry_cursors(&mut curve.walk, &carried.walk);
                }
                curve
                    .walk
                    .set_output(*carried.walk.output_state(), carried.walk.target());
            }
            (Self::Composed(curve), Self::Cell { state: carried, .. }) => {
                curve
                    .walk
                    .set_output(*carried, carried.target().map(curve_mutes));
            }
            (Self::Cell { state, .. }, Self::Composed(carried)) => {
                *state = automation::CellState::new();
                if let Some(muted) = carried.walk.target() {
                    state.seed_target(f32::from(u8::from(muted)));
                }
            }
        }
    }

    /// The curve mute the stage has heard (the output's current target).
    fn heard(&self) -> bool {
        match self {
            Self::Cell { state, .. } => state.target().is_some_and(curve_mutes),
            Self::Composed(curve) => curve.walk.target() == Some(true),
        }
    }

    /// Whether the curve mutes at node time `time` (`None` when a cell is not the program's).
    fn mutes_at(&self, program: &automation::Program, time: i64) -> Option<bool> {
        match self {
            Self::Cell { cell, .. } => program
                .cell(*cell as usize)
                .map(|program| curve_mutes(automation::value_at(program.segments(), time))),
            Self::Composed(curve) => Some(curve.mutes_at(program, time)),
        }
    }
}
// REALTIME_POLICY_END

/// What an automated channel's first block after a plan swap owes (#1648 D2). Set by
/// [`LaneFaderAutomation::adopt`] at the swap block (and raised to `Compare` by the first block's
/// offsets reads), taken by the lane's next block, which knows the node time. Ordered by strength:
/// two adoptions owed at once keep the stronger one.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Adoption {
    /// Nothing: the cell has rendered a block since it was prepared or adopted, or it continues
    /// its carried state verbatim (the same table, `a(n)` and `S`: the uninterrupted events).
    None,
    /// The cell continues its carried table and cursor, but after the first block's offsets
    /// records (the carried one and the successor's own) its `S` still differs from the one this
    /// plan was prepared with: the adoption rule alone (the jump when the composed gain differs
    /// from the stage's). Reached when no record could bring the change: a plan with no live
    /// cells, whose join cannot write one.
    Compare,
    /// The cell's table is not the one its state was built on (a changed or gained curve, or a
    /// moved `a(n)`): it repositions its cursor in its own table, takes the stage's held ramp
    /// again, then applies the adoption rule.
    Reposition,
}

/// One automated channel's obligation to move to its curve where a held ramp ends with the gain
/// free (#1647 D1.6, batch 13b amendment, R17-9): while the stage holds the lane's gain (muted,
/// or in a mute or unmute ramp, #1638 D2) a gain event is only remembered, and on a curve flat
/// afterwards the cell emits nothing more, so without the move the lane would stay at the gain
/// the unmute ramp aimed for.
#[derive(Clone, Copy, Debug, Default)]
struct HeldRampObligation {
    /// A gain event the stage only remembered is outstanding. Kept through a ramp that ends
    /// muted (constraint 4); cleared by the move, by a seek `Set` exactly at the hold's end
    /// (R17-8), or when a live write freed the gain at a block entry (it discharges the
    /// obligation there, R17-10).
    owed: bool,
    /// The held ramp ends on the last sample of the block just rendered: the move is due at the
    /// next block's first sample, which is the first sample the stage reports the gain free.
    due_at_entry: bool,
    /// The stage held the gain through the last sample of the block just rendered. A next block
    /// that finds the gain free with no move due had its hold cleared by a live write at its
    /// entry, so the cell's hold of the ramp that write ended is released there, owed or not
    /// (R17-10). An offsets jump applied on the free lane at that entry clears it: the jump's own
    /// hold is newer than the ramp the write ended (U2 fold 3, F1).
    held_through: bool,
}

/// One automated mute channel as a fader owner holds it (#1651 D1, batch 18 amendment): its cell
/// in the plan's program, its event state and `terms`.
///
/// `terms` is the lane's mute without the curve (OQ1 = C: `controls::EffectiveStrip::mute`), so
/// the stage mute of the lane is `curve || terms`. Preparation seeds it; the strip's mute cell
/// (#1312) carries every later value of it, the `Mute` record the shared commit emits from the
/// effective mute, and the block drain gives that value to this cell
/// ([`LaneFaderAutomation::mute_terms`]) instead of to the stage.
#[derive(Debug)]
struct AutomatedMuteCell {
    /// The channel's curve: its own cell, or the composed curve of a mute event lane an automated
    /// VCA `mute` lane reaches (#1655 V2).
    curve: MuteCurve,
    /// The mute terms the session does not automate.
    terms: bool,
    /// The cell's first block after a plan swap that carried its lane repositions it (#1651 T4,
    /// [`LaneFaderAutomation::adopt`]): its table is not the one its state was built on (a
    /// changed or gained curve, or a moved `a(n)`). That block composes `curve || terms` at its
    /// node time and lands a `SetMute` over the block's mute length when it differs from the
    /// stage's mute ([`FaderEventDriver::new`]). Taken by the lane's next block with node time.
    reposition: bool,
}

/// One strip lane's stored fader and mute automation (#1646 D1, #1651 D1): the plan's program and
/// the lane's automated channels. The one event driver both fader owners hold, the fader bank per
/// lane and the test-only scalar oracle per strip (R17-4), so neither has a second copy of the
/// rule.
struct LaneFaderAutomation {
    program: Arc<automation::Program>,
    /// `[left, right]`; `None` for a channel no fader entry drives.
    channels: [Option<AutomatedFaderCell>; 2],
    /// `[left, right]`; `None` for a channel no mute entry drives (#1651).
    mute: [Option<AutomatedMuteCell>; 2],
    /// The lane's floored input arrival `a(n)` (#1647a, R17-2): the graph compiler's, given to a
    /// bank lane at the end of `compile_with_builtins`; `0` until then.
    arrival: u64,
    /// The node-time mapping of the block the graph last set (#1647a, R17-3):
    /// `timeline_block(r + ΣP - a(n))`, `None` in a plan with no timeline. The event driver builds
    /// the lane's `NodeSpan` from it.
    span: Option<TimelineSpan>,
    /// The fader row's jump length of that block, from the three lengths the graph holds (#1639
    /// D2); `None` in a plan with no jump-length cell. The event driver passes it to
    /// `CellEvents::new`.
    fader_jump: Option<u32>,
    /// The mute row's jump length of that block (`LiveRamps::for_row(Mute)`, #1651 D1), from the
    /// same three lengths; `None` exactly when [`Self::fader_jump`] is.
    mute_jump: Option<u32>,
    /// The VCA rows' jump lengths of that block, `[offset_db, mute]` (`LiveRamps::for_row` of
    /// `VcaOffset` and `VcaMute`, #1655): a composed channel's VCA cursors take them; `None`
    /// exactly when [`Self::fader_jump`] is.
    vca_jumps: Option<[u32; 2]>,
    /// The lane has rendered no block yet (#1647 D2): its first block takes a `Set` when the
    /// curve at that block's node time differs from the value it was seeded with.
    fresh: bool,
}

impl LaneFaderAutomation {
    /// The lane's automation, from its prepared cells, sharing each fader channel's offsets cell.
    /// Control side: a composed channel allocates its cursor table (#1655 V2).
    ///
    /// # Errors
    ///
    /// The [`automation::ComposeError`] of a composed channel's table: unreachable for a plan's
    /// own cells (each names a cell of `program`, and a composed channel has at least one), and
    /// [`automation::ComposeError::Allocation`] when the table cannot be allocated.
    fn new(
        program: &Arc<automation::Program>,
        strip: &PreparedStripAutomation,
    ) -> Result<Self, automation::ComposeError> {
        let mut mute = [None, None];
        for (slot, cell) in mute.iter_mut().zip(&strip.mute) {
            let Some(cell) = cell else { continue };
            *slot = Some(AutomatedMuteCell {
                curve: MuteCurve::prepared(program, cell)?,
                terms: cell.terms,
                reposition: false,
            });
        }
        let mut channels = [None, None];
        for (slot, cell) in channels.iter_mut().zip(&strip.fader) {
            let Some(cell) = cell else { continue };
            *slot = Some(AutomatedFaderCell {
                curve: FaderCurve::prepared(program, cell)?,
                offsets: Arc::clone(&cell.offsets),
                offsets_reader: CellReader::new(),
                offsets_dirty: DirtyReader::new(),
                offsets_sum: cell.offsets_sum,
                offsets_ramp: cell.offsets_ramp,
                obligation: HeldRampObligation::default(),
                adoption: Adoption::None,
                carried_record: None,
                adopted_sum: None,
            });
        }
        Ok(Self {
            program: Arc::clone(program),
            mute,
            channels,
            arrival: 0,
            span: None,
            fader_jump: None,
            mute_jump: None,
            vca_jumps: None,
            fresh: true,
        })
    }

    // REALTIME_POLICY_BEGIN
    /// #1648 D2: this lane's stage was carried from `predecessor`'s lane at the swap block (the
    /// predecessor lane's automation, `None` when it had none). Each automated channel takes the
    /// carried cell's event state, held-ramp obligation, heard `S` with its ramp word, and the
    /// offsets record the carry drain read and no block applied, by its stable address (the
    /// strip, row 5 and the channel; the strip move paired the strip). Its first block applies
    /// that record and the successor's own (the join's retarget, a later live write) from the
    /// heard `S`, as the uninterrupted block would, and then compares the `S` it holds with this
    /// plan's ([`AutomatedFaderCell::adopted_sum`]). It owes that block a reposition when its
    /// table does not continue bit for bit (`continued_cells`, from the join) or `a(n)` moved,
    /// else nothing; an adoption the carried cell still owed (two swaps with no block between)
    /// is kept when it is the stronger. A channel the predecessor did not automate (a gained
    /// cell) keeps its prepared state and `S` and repositions. The lane is not fresh: its stage
    /// holds the carried gain, so no `Set` lands (#1647 D2 is for a lane that starts at its
    /// prepared value). A channel the successor does not automate is released by the join's
    /// retarget record, not here.
    ///
    /// #1651 T4: each automated mute channel takes the carried cell's event state and `terms`
    /// (the mute terms its stage heard) by the same address (the strip, row 6 and the channel,
    /// `continued_cells` bits 2 and 3); the successor's own `Mute` records (the join's, a later
    /// live write) move `terms` from there, as they move a fader channel's `S`. It owes its first
    /// block a reposition when its table does not continue bit for bit or `a(n)` moved (or the
    /// carried cell still owed one); a gained channel keeps its prepared state and `terms` and
    /// repositions. A repositioned channel composes `curve || terms` at the block's node time and
    /// lands a `SetMute` over the mute length only when that differs from the stage's mute; a
    /// continued one emits only its own events. A mute channel the successor does not automate is
    /// released by the join's `Mute` record. Allocation-free: plain copies.
    fn adopt(&mut self, predecessor: Option<&Self>, continued_cells: u8) {
        self.fresh = false;
        let arrival = self.arrival;
        for (channel, cell) in self.mute.iter_mut().enumerate() {
            let Some(cell) = cell else {
                continue;
            };
            let carried = predecessor.and_then(|predecessor| {
                predecessor
                    .mute
                    .get(channel)?
                    .as_ref()
                    .map(|carried| (carried, predecessor.arrival))
            });
            cell.reposition = match carried {
                Some((carried, carried_arrival)) => {
                    cell.curve.carry(&carried.curve);
                    cell.terms = carried.terms;
                    continued_cells & (1 << (MUTE_CONTINUED_SHIFT + channel)) == 0
                        || carried_arrival != arrival
                        || carried.reposition
                }
                None => true,
            };
        }
        for (channel, cell) in self.channels.iter_mut().enumerate() {
            let Some(cell) = cell else {
                continue;
            };
            let carried = predecessor.and_then(|predecessor| {
                predecessor
                    .channels
                    .get(channel)?
                    .as_ref()
                    .map(|carried| (carried, predecessor.arrival))
            });
            cell.adoption = match carried {
                Some((carried, carried_arrival)) => {
                    cell.curve.carry(&carried.curve);
                    cell.obligation = carried.obligation;
                    if carried_arrival != arrival
                        && let FaderCurve::Composed(curve) = &mut cell.curve
                    {
                        // A moved node clock needs the adoption rule, even if the unshifted
                        // successor tables still agree at the old origins.
                        curve.cursor_history_valid = false;
                    }
                    cell.adopted_sum = Some(cell.offsets_sum);
                    cell.offsets_sum = carried.offsets_sum;
                    cell.offsets_ramp = carried.offsets_ramp;
                    cell.carried_record = carried.carried_record;
                    let own = if continued_cells & (1 << channel) == 0 || carried_arrival != arrival
                    {
                        Adoption::Reposition
                    } else {
                        Adoption::None
                    };
                    own.max(carried.adoption)
                }
                None => Adoption::Reposition,
            };
        }
    }

    /// #1648 D2 (U3 fold), the plan-swap carry drain: each automated channel whose offsets cell
    /// holds a write takes the newest with every revision ([`LiveSnapshot::ALL`]), as the block
    /// drain would at the next block, and keeps it for the successor's first block
    /// ([`AutomatedFaderCell::carried_record`]); the stage is not moved here, because the jump's
    /// target is the curve at the node time of that block. The read adds the writes it replaced
    /// to `superseded`, and a kept record it replaces unheard (two carries with no block between)
    /// adds one. Allocation-, lock- and syscall-free; one read per automated channel.
    fn carry_offsets(&mut self, superseded: &LiveValuesSuperseded) {
        for cell in self.channels.iter_mut().flatten() {
            if cell.offsets_dirty.pending(&cell.offsets.dirty) == 0 {
                continue;
            }
            if let CellRead::Applied { words, .. } = cell.offsets_reader.read_counted(
                &cell.offsets.cell,
                LiveSnapshot::ALL,
                &mut cell.offsets_dirty,
                0,
                superseded,
            ) && cell
                .carried_record
                .replace(FaderOffsetsRecord::from_words(words))
                .is_some()
            {
                superseded.add_carried(1);
            }
        }
    }

    /// Take the block's node-time mapping and the fader and mute rows' jump lengths (#1647a,
    /// R17-3; #1651 D1): the one path of both fader owners, the bank per lane and the scalar
    /// oracle per strip.
    fn take_block_time(&mut self, time: &GraphBlockTime<'_>, jumps: Option<BlockJumps>) {
        self.span = time.node_span(self.arrival);
        self.fader_jump = jumps.map(|jumps| jumps.fader);
        self.mute_jump = jumps.map(|jumps| jumps.mute);
        self.vca_jumps = jumps.map(|jumps| [jumps.vca_offset, jumps.vca_mute]);
    }

    /// #1651 D1, the drain branch: a mute value the block drain read for channel `channel`, whose
    /// mute this lane automates, is the lane's new `terms` (the shared commit's `Mute` record
    /// carries `controls::EffectiveStrip::mute`, OQ1 = C). The cell stores it and composes the
    /// stage mute `curve || terms` with the curve state the stage has heard (the cell's current
    /// target); when that differs from the stage's mute it applies `SetMute` at the block
    /// boundary over the record's ramp word `ramp`, as [`Self::read_offsets`] applies its jump.
    /// On a cell that owes a reposition (#1651 T4) the same record composes with the curve at the
    /// block's first node time instead, so it lands as committed and the driver's adoption
    /// landing pushes only a change the curve itself makes (U1 fold 1). The block's curve events
    /// then compose with the new `terms`. A channel with no mute cell
    /// is not this cell's (the caller applies its record to the stage). Allocation-, lock- and
    /// syscall-free: one compare and at most one operation.
    fn mute_terms(
        &mut self,
        channel: usize,
        terms: bool,
        ramp: u32,
        stage: &mut impl FaderJumpStage,
    ) {
        let Some(cell) = self.mute.get_mut(channel).and_then(Option::as_mut) else {
            return;
        };
        cell.terms = terms;
        let heard = cell.curve.heard() || terms;
        let stage_muted = stage.muted(channel);
        if heard == stage_muted {
            return;
        }
        // #1651 U1 fold 1 (MAJOR-2): a repositioned cell's target is not yet the curve at this
        // block's node time (the driver repositions it below). The record changes the stage mute
        // the curve the stage heard gives, so it lands here as committed, over its own ramp word,
        // composed with the curve at the block's first node time; the driver's landing then
        // compares with that stage mute and pushes only a change the curve itself makes, over the
        // mute length. So the swap block lands at most one `SetMute` per channel. A lane with no
        // node time this block renders no curve and keeps the cell owed: the carried target is
        // the curve it holds.
        let at_node = match (cell.reposition, self.mute_jump, self.span) {
            (true, Some(_), Some(span)) => node_time_at(span, 0)
                .and_then(|time| cell.curve.mutes_at(&self.program, time))
                .map(|muted| muted || terms),
            _ => None,
        };
        let muted = at_node.unwrap_or(heard);
        if stage_muted != muted {
            let applied = stage.apply(
                channel,
                builtins::FaderOperation::SetMute {
                    muted,
                    samples: ramp,
                },
            );
            // Unreachable refusal: the lane is a populated member and a mute operation carries
            // no gain. In release the stage keeps its mute, no panic.
            debug_assert!(applied, "a mute terms write is refused at render");
        }
    }

    /// Whether this lane automates the mute of channel `channel`.
    fn automates_mute(&self, channel: usize) -> bool {
        self.mute.get(channel).is_some_and(Option::is_some)
    }

    /// #1649 D3, at block entry under the block's `snapshot`: each automated channel whose
    /// offsets cell holds a newer write takes its `S` and ramp word, and the lane jumps to the
    /// composed gain at the jump's completion when that gain differs in bits from the lane's
    /// current gain target ([`AutomatedFaderCell::offsets_jump`]). An applied read adds the
    /// writes it replaced to `superseded`; `superseded` is `None` on a lane with no control lane,
    /// whose cells have no writer and are not read.
    ///
    /// #1648 D2 (U3 fold), at an adopted cell's first block: the record the carry drain kept is
    /// applied the same way, from the carried `S` the stage heard, unless the cell's own read
    /// applied a newer one, which replaces it unheard (one superseded value, as two writes before
    /// one uninterrupted block). With no record of its own, an `S` still unequal to the one this
    /// plan was prepared with takes it and owes the adoption rule ([`Adoption::Compare`]); a
    /// record of its own is newer than the preparation and is the committed `S`. Allocation-, lock- and
    /// syscall-free; at most one read per automated channel.
    fn read_offsets(
        &mut self,
        snapshot: LiveSnapshot,
        superseded: Option<&LiveValuesSuperseded>,
        stage: &mut impl FaderJumpStage,
    ) {
        let Self {
            program,
            channels,
            span,
            ..
        } = self;
        for (channel, cell) in channels.iter_mut().enumerate() {
            let Some(cell) = cell.as_mut() else {
                continue;
            };
            let carried = cell.carried_record.take();
            let own = superseded.and_then(|superseded| {
                if cell.offsets_dirty.pending(&cell.offsets.dirty) == 0 {
                    return None;
                }
                let CellRead::Applied { words, .. } = cell.offsets_reader.read_counted(
                    &cell.offsets.cell,
                    snapshot,
                    &mut cell.offsets_dirty,
                    0,
                    superseded,
                ) else {
                    return None;
                };
                if carried.is_some() {
                    superseded.add_carried(1);
                }
                Some(FaderOffsetsRecord::from_words(words))
            });
            let adopted_sum = cell.adopted_sum.take();
            if let Some(record) = own.or(carried) {
                let jumped = cell.offsets_jump(program, *span, channel, record, stage);
                // Unreachable refusal: the cell index is the program's own (prepared from it),
                // the curve is in the fader domain and `vca_compose_db` clamps (a NaN `S`, which
                // a validated record never carries, fails `checked_fader_gain`, so no NaN reaches
                // the stage), and the lane is a populated member. In release a refusal leaves the
                // stage's gain as it was: a hold, no panic.
                debug_assert!(jumped, "an offsets jump is refused at render");
            }
            // A record of the cell's own is newer than this plan's preparation (the join's
            // retarget, or a live write committed after it), so its `S` is the committed one; a
            // carried record is older than the preparation, whose `S` it must then match.
            if own.is_none()
                && let Some(sum) = adopted_sum
                && sum.to_bits() != cell.offsets_sum.to_bits()
            {
                // Unreachable on a lane with live cells (B17-R12): the join writes an offsets
                // record to the successor's cell whenever its `S` differs from the committed one
                // (`join_level_records`), and every C ABI and browser plan attaches live cells to
                // every strip, so only a plan prepared with no live controls (a host-core entry no
                // shipped host calls) takes this backstop. In release it ramps over `J`.
                debug_assert!(
                    superseded.is_none(),
                    "the adoption rule stands in for a live offsets record"
                );
                cell.offsets_sum = sum;
                if let FaderCurve::Composed(curve) = &mut cell.curve {
                    curve.cursor_history_valid = false;
                }
                cell.adoption = cell.adoption.max(Adoption::Compare);
            }
            // #1655 D3: a composed channel's `own_db` takes the same rule as its `S`.
            if let FaderCurve::Composed(curve) = &mut cell.curve
                && let Some(prepared) = curve.adopted_own_db.take()
                && own.is_none()
                && prepared.to_bits() != curve.own_db.to_bits()
            {
                debug_assert!(
                    superseded.is_none(),
                    "the adoption rule stands in for a live own_db record"
                );
                curve.own_db = prepared;
                curve.cursor_history_valid = false;
                cell.adoption = cell.adoption.max(Adoption::Compare);
            }
        }
    }
    // REALTIME_POLICY_END
}

/// The fader stage of one automated lane, as an offsets jump drives it (#1649 D3): a fader bank's
/// lane or the scalar oracle's fader.
trait FaderJumpStage {
    /// Whether `channel`'s gain is held (muted, or in a mute or unmute ramp): a gain operation
    /// then only remembers the gain (#1638 D2).
    fn gain_held(&self, channel: usize) -> bool;
    /// Whether `channel` is muted (settled, or in a ramp toward `0.0`) by the model, a live
    /// record or a curve. An armed channel whose fire opens it (issue #1288, #2080 R1) is not:
    /// its mute is the arm's, and the model holds it unmuted. An armed channel whose fire stays
    /// muted is muted in the model. A record that reaches the stage while the channel is armed
    /// only retargets the fire (#2080 R1).
    fn muted(&self, channel: usize) -> bool;
    /// Frames left in `channel`'s ramp; `0` when settled.
    fn ramp_remaining(&self, channel: usize) -> u32;
    /// Apply `operation` to `channel` now, at the block boundary; `false` when the stage refuses.
    fn apply(&mut self, channel: usize, operation: builtins::FaderOperation) -> bool;
}

/// One lane of a fader bank, as a [`FaderJumpStage`].
struct BankLaneStage<'a> {
    bank: &'a mut BuiltinFaderBank,
    lane: usize,
}

// REALTIME_POLICY_BEGIN
impl FaderJumpStage for BankLaneStage<'_> {
    fn gain_held(&self, channel: usize) -> bool {
        self.bank.gain_held(self.lane, channel) == Some(true)
    }
    fn muted(&self, channel: usize) -> bool {
        self.bank.is_muted(self.lane, channel) == Some(true)
            && self.bank.is_armed(self.lane, channel) != Some(true)
    }
    fn ramp_remaining(&self, channel: usize) -> u32 {
        self.bank.ramp_remaining(self.lane, channel).unwrap_or(0)
    }
    fn apply(&mut self, channel: usize, operation: builtins::FaderOperation) -> bool {
        self.bank
            .apply_fader_operation(self.lane, channel_selector(channel), operation)
            .is_ok()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FaderJumpStage for FaderMuteRampBuiltins {
    fn gain_held(&self, channel: usize) -> bool {
        Self::gain_held(self, channel) == Some(true)
    }
    fn muted(&self, channel: usize) -> bool {
        self.is_muted(channel) && self.is_armed(channel) != Some(true)
    }
    fn ramp_remaining(&self, channel: usize) -> u32 {
        Self::ramp_remaining(self, channel).unwrap_or(0)
    }
    fn apply(&mut self, channel: usize, operation: builtins::FaderOperation) -> bool {
        self.apply_fader_operation(channel_selector(channel), operation)
            .is_ok()
    }
}

/// The selector of one channel (`0` left, anything else right).
const fn channel_selector(channel: usize) -> BuiltinLaneSelector {
    if channel == 0 {
        BuiltinLaneSelector::Left
    } else {
        BuiltinLaneSelector::Right
    }
}

impl AutomatedFaderCell {
    /// #1649 D3: take `record` (a new `S` and its ramp word) and jump the lane to it.
    ///
    /// The jump's target is the curve at the node time of the jump's completion (`End` of a ramp
    /// of the ramp word started at the block's first sample, never past the next discontinuity,
    /// which jumps of its own), composed with the new `S`: `checked_fader_gain(vca_compose_db(v,
    /// S))`, as every event's gain is (#1646 D1). The lane's current gain target is the cell's
    /// target composed with the `S` the lane heard before. Equal gain bits take no jump (a `+0.0`
    /// `S` that turns a `-0.0` value into `+0.0` moves nothing). Otherwise the lane retargets over
    /// the ramp word (an exact set for `0`) at the block's first sample, and the cell commits the
    /// target and holds until the completion, so grid events inside the jump are held (README
    /// "Held events").
    ///
    /// On a lane whose gain is held (muted, or in a mute or unmute ramp) the stage only remembers
    /// the gain (#1638 D2), so the target its ramp aims for is no longer one the cell knows: the
    /// cell forgets its target (a fresh state, whose next free grid event always retargets) and
    /// holds until the held ramp's last sample (released at the entry of a block whose live write
    /// frees the gain early, R17-10). On a free lane the jump's own hold is never released by
    /// that rule: a live write that frees the gain at the same entry ended an older ramp. With no
    /// node time (a plan with no timeline) the cell's own target is the jump's value.
    ///
    /// `false` when the composed value has no fader gain, the program has no such cell, or the
    /// stage refuses the operation (unreachable: the curve is in the fader domain,
    /// `vca_compose_db` clamps, and the lane is a member).
    fn offsets_jump(
        &mut self,
        program: &automation::Program,
        span: Option<TimelineSpan>,
        channel: usize,
        record: FaderOffsetsRecord,
        stage: &mut impl FaderJumpStage,
    ) -> bool {
        let heard = core::mem::replace(&mut self.offsets_sum, record.offsets_sum);
        let ramp = record.smoothing_samples;
        self.offsets_ramp = ramp;
        let completion = controls::FADER_CELL_RULE.completion;
        let start = span.and_then(|span| node_time_at(span, 0));
        let compose =
            |value, sum| builtins::checked_fader_gain(session::vca_compose_db(value, sum)).ok();
        // `value` is the channel's new target (a cell's curve value, a composed channel's fader
        // dB), `gain` its gain, `current` the gain of the target the lane heard.
        let (value, gain, current) = match &mut self.curve {
            FaderCurve::Cell { cell, state } => {
                let value = match (start, program.cell(*cell as usize)) {
                    (Some(start), Some(cell)) => curve_toward(
                        &cell,
                        start,
                        automation::completion_of(completion, start, ramp),
                    ),
                    (Some(_), None) => return false,
                    (None, _) => match state.target() {
                        Some(value) => value,
                        None => return true,
                    },
                };
                let Some(gain) = compose(value, self.offsets_sum) else {
                    return false;
                };
                let current = state.target().and_then(|target| compose(target, heard));
                (value, gain, current)
            }
            FaderCurve::Composed(curve) => {
                // #1655 D3: a record of an event lane whose own fader is not automated carries
                // the member's `own_db`, composed with the new `S` and the curves.
                if let (false, Some(own_db)) = (curve.own_curve, record.own_db) {
                    curve.own_db = own_db;
                }
                let value = match start {
                    Some(start) => curve.toward(
                        program,
                        self.offsets_sum,
                        start,
                        automation::completion_of(completion, start, ramp),
                    ),
                    None => {
                        let compose = curve.composition(self.offsets_sum);
                        curve.walk.compose_heard(compose)
                    }
                };
                let Some(gain) = builtins::checked_fader_gain(value).ok() else {
                    return false;
                };
                let current = curve
                    .walk
                    .target()
                    .and_then(|target| builtins::checked_fader_gain(target).ok());
                (value, gain, current)
            }
        };
        if current.map(f32::to_bits) == Some(gain.to_bits()) {
            return true;
        }
        let operation = if ramp == 0 {
            builtins::FaderOperation::SetGain { gain }
        } else {
            builtins::FaderOperation::RetargetGain {
                gain,
                samples: ramp,
            }
        };
        if stage.gain_held(channel) {
            let remaining = stage.ramp_remaining(channel);
            let applied = stage.apply(channel, operation);
            let end = remaining
                .checked_sub(1)
                .zip(span)
                .and_then(|(last, span)| node_time_at(span, u64::from(last)));
            match &mut self.curve {
                FaderCurve::Cell { state, .. } => {
                    *state = automation::CellState::new();
                    if let Some(end) = end {
                        state.hold_until(end);
                    }
                }
                FaderCurve::Composed(curve) => {
                    curve.walk.clear_target();
                    curve.each_state(|state| {
                        *state = automation::CellState::new();
                        if let Some(end) = end {
                            state.hold_until(end);
                        }
                    });
                }
            }
            return applied;
        }
        if !stage.apply(channel, operation) {
            return false;
        }
        // This jump's hold is newer than the held ramp a live write ended at this entry, so the
        // block's R17-10 release must not cut it (#1647 U2 fold 3, F1). A successor's carried
        // record takes this same path at its first block, with the carried obligation.
        self.obligation.held_through = false;
        let hold = start.map(|start| automation::completion_of(completion, start, ramp));
        // A newer live move keeps its own length: a jump an adoption deferred to the end of the
        // ramp this one replaces is dropped (#1711 D1).
        match &mut self.curve {
            FaderCurve::Cell { state, .. } => {
                state.seed_target(value);
                state.clear_deferred_jump();
                if let Some(hold) = hold {
                    state.hold_until(hold);
                }
            }
            FaderCurve::Composed(curve) => {
                curve.walk.seed_target(value);
                curve.walk.clear_deferred_jump();
                if let Some(hold) = hold {
                    curve.each_state(|state| state.hold_until(hold));
                }
            }
        }
        true
    }
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// The value a ramp started at node time `start` and completing at `at` aims for: the curve at
/// `at`, but never past the next discontinuity after `start`, which jumps of its own (the rule of
/// the cell's own `target_at`).
fn curve_toward(cell: &automation::CellProgram<'_>, start: i64, at: i64) -> f32 {
    automation::value_at(cell.segments(), curve_target_time(cell, start, at))
}

/// The origin of `curve_toward`'s value, also retained when adoption seeds a cursor.
fn curve_target_time(cell: &automation::CellProgram<'_>, start: i64, at: i64) -> i64 {
    let ahead = cell.discontinuities();
    match ahead.get(ahead.partition_point(|&time| time <= start)) {
        Some(&next) if next <= at => next - 1,
        _ => at,
    }
}

/// The node time of sample `offset` of a node block mapped by `span` (`offset` may lie past the
/// block: the mapping extends forward), or `None` past the `i64` range. Checked: a span saturated
/// at the `i64` edges (R17-5) never wraps here.
fn node_time_at(span: TimelineSpan, offset: u64) -> Option<i64> {
    span_time(
        span.start,
        span.step.map(|step| (step.offset, step.timeline)),
        offset,
    )
}

/// [`node_time_at`] for a block whose offset 0 is node time `start`, with an optional step
/// `(offset, node time)`.
fn span_time(start: i64, step: Option<(u32, i64)>, offset: u64) -> Option<i64> {
    let (base, from) = match step {
        Some((at, timeline)) if offset >= u64::from(at) => (timeline, u64::from(at)),
        _ => (start, 0),
    };
    base.checked_add(i64::try_from(offset - from).ok()?)
}

/// The fader, mute and VCA rows' jump lengths of one block (#1639 D2, #1651 D1, #1655).
#[derive(Clone, Copy, Debug)]
struct BlockJumps {
    /// `LiveRamps::for_row(Fader)`.
    fader: u32,
    /// `LiveRamps::for_row(Mute)`.
    mute: u32,
    /// `LiveRamps::for_row(VcaOffset)`: a composed channel's VCA offset cursors (resolved by
    /// row, not assumed equal to the fader's).
    vca_offset: u32,
    /// `LiveRamps::for_row(VcaMute)`.
    vca_mute: u32,
}

/// The fader and mute rows' jump lengths of a block (#1639 D2, #1647a departure 7, #1651 D1),
/// resolved once from the three lengths the graph holds through `LiveRamps`, which owns the row
/// rule; `None` in a plan with no jump-length cell.
fn block_jumps(time: &GraphBlockTime<'_>) -> Option<BlockJumps> {
    time.jump_lengths().map(|lengths| {
        let ramps = controls::LiveRamps::from_lengths(
            lengths.fader_samples,
            lengths.mute_samples,
            lengths.pan_samples,
        );
        BlockJumps {
            fader: ramps.for_row(effect_contract::control::LiveRampRow::Fader),
            mute: ramps.for_row(effect_contract::control::LiveRampRow::Mute),
            vca_offset: ramps.for_row(effect_contract::control::LiveRampRow::VcaOffset),
            vca_mute: ramps.for_row(effect_contract::control::LiveRampRow::VcaMute),
        }
    })
}

/// The bit of `GraphStripMove::continued_cells` that holds a fader stage's left mute channel; the
/// right one is the next bit (#1651 T4). Bits 0 and 1 are the fader channels (#1648 D2).
const MUTE_CONTINUED_SHIFT: usize = 2;

/// The most cells one fader owner drives in a block: two channels on each of a bank's lanes.
const MAXIMUM_FADER_CELLS: usize = 2 * MAXIMUM_BANK_LANES;

/// One automated channel's stage state at a block's first sample, after the live drain (#1647
/// D1.6): what decides where the stage's hold of its gain ends in the block.
#[derive(Clone, Copy, Debug)]
struct ChannelHold {
    /// The stage holds the gain: muted, or in a mute or unmute ramp (#1638 D2).
    held: bool,
    /// The channel is muted: settled, or in a ramp toward `0.0`.
    muted: bool,
    /// Frames left in the channel's ramp.
    remaining: u32,
    /// The gain an adopted cell compares its curve with (#1648 D2): on a held stage, the fader
    /// gain it aims for (the one an unmute lands on); on a free stage, its ramp's target (#1711
    /// F0: the cell's held grid events only remember a gain inside a live move's ramp, so the
    /// fader gain follows the ride while the target stays where the move ends). `None` on the
    /// scalar oracle, which no plan swap carries (its fader owners are bound nodes, not bank
    /// lanes).
    stage_gain: Option<f32>,
}

/// One channel's block cut at the end `E` of a held ramp (#1647 D1.6, constraint 2): the piece
/// that holds `E` ends there, and the next starts at `E + 1`.
#[derive(Clone, Copy, Debug)]
struct HoldEnd {
    /// `E`: the first offset at which the stage reports the gain free (constraint 7).
    at: u32,
    /// The node time of `E`.
    node: i64,
}

/// One automated channel's events of one block, with the next one pulled (#1647 D1.3).
///
/// The block is rendered in pieces, each a span of the cell's events: a piece ends at the cut at
/// `E` (D1.6), at the offset of the channel's next mute event (#1651 T3b: a mute event changes
/// where the stage holds the gain, so the piece after it is cut where that hold ends), or at the
/// block's last sample. A piece's events are consumed to its end before the next starts, so the
/// cell's state advances only through events the stage was given (#1647 constraint 2).
struct FaderCellCursor<'a> {
    /// The owner's lane (the bank lane; `0` on the scalar oracle).
    lane: usize,
    /// `0` left, `1` right.
    channel: usize,
    /// `S`, the offset sum every event's value is composed with (#1646 D1).
    offsets_sum: f64,
    /// The block's jump length of a lone cell, to build the later pieces.
    jump: u32,
    /// The channel's events: a lone cell's or a composed curve's (#1655 V2).
    walk: ChannelEvents<'a>,
    /// The block offset of the current piece's first sample.
    base: u32,
    /// The block offset of the current piece's last sample.
    last: u32,
    /// The next event, its offset in the block.
    next: Option<automation::Event>,
    /// The channel's held-ramp obligation (D1.6).
    obligation: &'a mut HeldRampObligation,
    /// The first offset at which the stage's gain is free in this block: an event pushed below it
    /// is only remembered (`0`: free all block; the block length: held all block). A mute event
    /// pushed on the channel sets it again ([`Self::rehold`]).
    free_from: u32,
    /// The cut at the held ramp's end, while it is ahead.
    end: Option<HoldEnd>,
    /// The offset of the channel's next mute event, while one is ahead (#1651 T3b).
    mute_at: Option<u32>,
    /// The node time by which the cell's hold must end, set by an unmute the driver pushed
    /// ([`Self::rehold`]) and applied to the cell's state where the next piece opens
    /// ([`Self::open`]), shorten-only (#1651 U1 fold 1, MAJOR-1).
    release: Option<i64>,
    /// The block's node-time mapping: offset 0's node time and the seek step, and its length.
    start: i64,
    step: Option<(u32, i64)>,
    frames: u32,
}

/// One fader channel's events in a block (#1655 V2): a lone cell's, piece by piece, or a composed
/// curve's, whose walk is begun on each piece.
enum ChannelEvents<'a> {
    Cell {
        /// The cell's program.
        program: automation::CellProgram<'a>,
        /// The current piece's events, spent or not: it holds the cell's state for the next piece.
        /// `None` only after a refusal (unreachable).
        events: Option<automation::CellEvents<'a, 'a>>,
    },
    Composed {
        /// The plan's program.
        program: &'a automation::Program,
        /// The composed curve; its walk holds every cursor's state for the next piece.
        curve: &'a mut FaderComposite,
    },
}

impl FaderCellCursor<'_> {
    /// Pull the current piece's next event, rebased to the block.
    fn pull(&mut self) {
        let base = self.base;
        let sum = self.offsets_sum;
        let next = match &mut self.walk {
            ChannelEvents::Cell { events, .. } => events.as_mut().and_then(Iterator::next),
            ChannelEvents::Composed { program, curve } => curve.next_event(program, sum),
        };
        self.next = next.map(|event| automation::Event {
            offset: event.offset.saturating_add(base),
            ..event
        });
    }

    /// The gain of event value `value`: a lone cell's curve value composed with `S`, a composed
    /// curve's fader dB as it is; `None` when it has no fader gain (unreachable).
    fn gain(&self, value: f32) -> Option<f32> {
        match self.walk {
            ChannelEvents::Cell { .. } => FaderEventDriver::gain(value, self.offsets_sum),
            ChannelEvents::Composed { .. } => builtins::checked_fader_gain(value).ok(),
        }
    }

    /// The offset of the cursor's next operation: its next event, or the cut at `E` once the
    /// pieces before it are spent (a piece never lies past `E`, so no event is pulled past it).
    fn next_offset(&self) -> Option<u32> {
        self.next
            .map(|event| event.offset)
            .or_else(|| self.end.map(|end| end.at))
    }

    /// The last offset of a piece that starts at `from` ([`piece_last`]).
    fn piece_last(&self, from: u32) -> u32 {
        piece_last(self.end, self.mute_at, self.frames, from)
    }

    /// End the piece at the cut. With `moving`, the cell first lands at `E` as a seek in
    /// [`automation::SeekMode::Ramp`] does (R17-9): one `Jump` over the block's jump length `J`,
    /// aimed at the curve at the ramp's completion `E + J - 1` (never past the next
    /// discontinuity), which commits that target and holds the grid for the move's own ramp
    /// (constraint 3), as after the cell's own `Jump`. Returns that landing. The next piece is
    /// opened by the caller ([`Self::open`]). Bounded: one event of a one-sample span, a binary
    /// search of the discontinuities and one curve read.
    fn cut(&mut self, end: HoldEnd, moving: bool) -> Option<automation::Event> {
        self.end = None;
        if !moving {
            return None;
        }
        // Unreachable: `NodeSpan::new` errs only for a step offset at or past the span's length,
        // and 0 < 1. In release the cut lands nothing (the move is skipped and the lane holds its
        // gain), no panic.
        let Ok(span) = automation::NodeSpan::new(end.node, Some((0, end.node)), 1) else {
            debug_assert!(false, "a one-sample span with its step at 0 is valid");
            return None;
        };
        let sum = self.offsets_sum;
        match &mut self.walk {
            ChannelEvents::Cell { program, events } => {
                let state = events.take()?.into_state();
                let mut landing_events = automation::CellEvents::new(
                    *program,
                    state,
                    span,
                    automation::SeekMode::Ramp,
                    self.jump,
                );
                let landing = landing_events.next();
                *events = Some(landing_events);
                landing
            }
            ChannelEvents::Composed { program, curve } => {
                // Every cursor lands (R17-9 on each curve), and the output lands over the longest
                // of their jumps (R19-4).
                curve.walk.start(program, span, automation::SeekMode::Ramp);
                let landing = curve.next_event(program, sum);
                // The one-sample span holds no other offset: the walk ends here.
                while curve.next_event(program, sum).is_some() {}
                landing
            }
        }
    }

    /// Start the next piece at block offset `from` (the current piece is spent), through
    /// [`Self::piece_last`]. `false` when there is none: `from` is past the block, or a refusal
    /// (unreachable: the block's node times were checked against the `i64` edge, and a step kept
    /// inside the piece lies inside it), which leaves the cell's state where it was.
    ///
    /// A release [`Self::rehold`] recorded is applied to the cell's state first, also when no
    /// piece is left (an unmute on the block's last sample): the next block's events start from
    /// that state.
    fn open(&mut self, from: u32) -> bool {
        if from >= self.frames {
            // The block is spent: no later piece reads the events, and the cell's state lives
            // in its lane slot.
            if let Some(release) = self.release.take() {
                match &mut self.walk {
                    ChannelEvents::Cell { events, .. } => {
                        if let Some(events) = events.take() {
                            release_lane_hold(events.into_state(), release);
                        }
                    }
                    ChannelEvents::Composed { curve, .. } => {
                        curve.each_state(|state| release_lane_hold(state, release));
                    }
                }
            }
            return false;
        }
        let last = self.piece_last(from);
        let Some(span) = piece_span(self.start, self.step, from, last) else {
            debug_assert!(false, "a piece of a checked block is a valid span");
            return false;
        };
        let release = self.release.take();
        match &mut self.walk {
            ChannelEvents::Cell { program, events } => {
                let Some(spent) = events.take() else {
                    return false;
                };
                let state = spent.into_state();
                if let Some(release) = release {
                    release_lane_hold(state, release);
                }
                *events = Some(automation::CellEvents::new(
                    *program,
                    state,
                    span,
                    controls::FADER_CELL_RULE.seek,
                    self.jump,
                ));
            }
            ChannelEvents::Composed { program, curve } => {
                if let Some(release) = release {
                    curve.each_state(|state| release_lane_hold(state, release));
                }
                curve
                    .walk
                    .start(program, span, controls::FADER_CELL_RULE.seek);
            }
        }
        self.base = from;
        self.last = last;
        self.pull();
        true
    }

    /// #1651 T3b: the hold a `SetMute { muted, samples }` pushed at offset `at` gives the channel
    /// from `at + 1` on (the gain operation at `at` kept the old hold: gain before mute). Muted
    /// (settled or in a ramp toward `0.0`): held to the block's end, and through it. Unmuted over
    /// `M = samples`: held until `E = at + M`, the first offset at which the stage reports the
    /// gain free; an `E` inside the block is the cut where an owed move lands (D1.6), an `E` at
    /// the block's end makes the move due at the next block's first sample, and a later `E` is
    /// the next block's to find from the stage. Unmuted exactly (`M = 0`): the stage takes the
    /// remembered gain, the cell's committed target, at `at`, so the gain is free and nothing is
    /// owed. An older cut ahead is dropped: the stage's hold no longer ends there.
    ///
    /// #1651 U1 fold 1 (MAJOR-1): an unmute also ends a hold the cell holds of an older ramp (an
    /// offsets jump's on a lane held by a mute ramp: until that ramp's end at its block's entry),
    /// which the stage no longer holds: the cell's hold is shortened to end at `node(at)` for an
    /// exact unmute and at `node(at + M) - 1` otherwise, so its grid moves the lane from the first
    /// free sample instead of being only remembered on a free stage. Shorten-only: a hold of the
    /// cell's own that ends earlier is kept.
    fn rehold(&mut self, at: u32, muted: bool, samples: u32) {
        self.end = None;
        let frames = self.frames;
        let obligation = &mut *self.obligation;
        obligation.due_at_entry = false;
        if muted {
            self.free_from = frames;
            obligation.held_through = true;
            return;
        }
        // The M > 0 arm is defensive: a muted span is at least 64 samples, so a grid event before
        // E owes a move whose landing at E re-commits the cell anyway (#1651 U1 fold verification).
        self.release = match samples {
            0 => span_time(self.start, self.step, u64::from(at)),
            samples => span_time(self.start, self.step, u64::from(at) + u64::from(samples))
                .map(|end| end.saturating_sub(1)),
        };
        if samples == 0 {
            self.free_from = 0;
            obligation.owed = false;
            obligation.held_through = false;
            return;
        }
        match u32::try_from(u64::from(at) + u64::from(samples)) {
            Ok(end) if end < frames => {
                self.free_from = end;
                self.end = hold_end(self.start, self.step, end);
                // Unreachable: the block's node times were checked against the `i64` edge. In
                // release no cut is set: the lane holds the gain the unmute ramp aimed for.
                debug_assert!(
                    self.end.is_some(),
                    "an end inside a checked block has a node time"
                );
                obligation.held_through = false;
            }
            end => {
                self.free_from = frames;
                obligation.due_at_entry = end == Ok(frames);
                obligation.held_through = true;
            }
        }
    }
}

/// The last offset of a piece that starts at `from` in a block of `frames` samples: the cut at
/// `E` (`end`) or the channel's next mute event (`mute_at`), whichever comes first, else the
/// block's last sample.
fn piece_last(end: Option<HoldEnd>, mute_at: Option<u32>, frames: u32, from: u32) -> u32 {
    [end.map(|end| end.at), mute_at]
        .into_iter()
        .flatten()
        .filter(|&at| at >= from)
        .fold(frames.saturating_sub(1), u32::min)
}

/// A fader channel as the driver placed it at a block's entry, before its first piece.
enum Placed<'a> {
    Cell {
        program: automation::CellProgram<'a>,
        state: &'a mut automation::CellState,
    },
    Composed(&'a mut FaderComposite),
}

impl Placed<'_> {
    /// R17-10: release the hold of the ramp a live write ended at the block's entry (`release`,
    /// the node time before the block's first sample) and drop a jump deferred to its end: on a
    /// lone cell, or on every cursor of a composed curve.
    fn release_at_entry(&mut self, release: i64) {
        match self {
            Self::Cell { state, .. } => {
                state.hold_until(release);
                state.clear_deferred_jump();
            }
            Self::Composed(curve) => {
                curve.each_state(|state| state.hold_until(release));
                curve.walk.clear_deferred_jump();
            }
        }
    }
}

/// The span of the block offsets `[from, last]` of a block whose offset 0 is node time `start`,
/// with seek step `step` (kept, rebased, when it lies inside the piece); `None` past the `i64`
/// range (unreachable: the caller checked the block's span).
fn piece_span(
    start: i64,
    step: Option<(u32, i64)>,
    from: u32,
    last: u32,
) -> Option<automation::NodeSpan> {
    let frames = last.checked_sub(from)?.checked_add(1)?;
    let inside = step.filter(|&(offset, _)| offset >= from && offset <= last);
    let span = if from == 0 {
        automation::NodeSpan::new(start, inside, frames)
    } else {
        automation::NodeSpan::new(
            span_time(start, step, u64::from(from))?,
            inside.map(|(offset, timeline)| (offset - from, timeline)),
            frames,
        )
    };
    span.ok()
}

/// The one event driver of stored fader automation (#1647 D1.3-D1.6, R17-4): both fader owners,
/// the fader bank and the test-only scalar oracle, render a block's events through it.
///
/// It pulls each automated channel's events (#1637, [`automation::SeekMode::Exact`]) and merges
/// them by offset with one fixed cursor per channel, with no buffer: the stage asks for the
/// lowest pending offset and takes every channel's event at it ([`builtins::FaderOperationSource`]).
/// Each event's value is composed with the channel's `S` and converted to a fader gain as the
/// static path does (`checked_fader_gain(vca_compose_db(value, S))`, #1646 D1); a grid or jump
/// event becomes a `RetargetGain` over its ramp, a held event a `RememberGain` and a seek or
/// restart landing a `SetGain` (#1638 D1).
///
/// D1.6 (batch 13b amendment): a gain event pushed while the stage holds the channel's gain is
/// only remembered, so the channel owes a move. At the first sample `E` at which the stage
/// reports the gain free, the driver moves the channel with a ramped landing of its cell at `E`
/// (R17-9: aimed at the curve at `E + J - 1`, over the block's jump length `J`), in place of any
/// event the cell gives at `E`; a seek `Set` exactly at `E` is pushed instead and discharges the
/// obligation (R17-8). The channel's span is cut there, so no event past `E` is pulled before the
/// move. A live write that frees a gain the stage held through the previous block discharges the
/// obligation and releases the cell's hold there, owed or not (R17-10). An event render refuses (unreachable: the curve is in the fader
/// domain, `vca_compose_db` clamps and the lane is a member) keeps the stage's target and fails a
/// `debug_assert`. Fixed size, on the render stack; allocation-, lock- and syscall-free.
struct FaderEventDriver<'a> {
    cursors: [Option<FaderCellCursor<'a>>; MAXIMUM_FADER_CELLS],
    /// The automated mute channels' cursors (#1651 D1), one per lane and channel.
    mutes: [Option<MuteCellCursor<'a>>; MAXIMUM_FADER_CELLS],
    #[cfg(any(test, feature = "test-support"))]
    first_sample: u64,
}

/// One automated mute channel's events of one block (#1651 D1): its cell's jumps and seek
/// landings, and the stage mute the driver last composed.
struct MuteCellCursor<'a> {
    /// The owner's lane (the bank lane; `0` on the scalar oracle).
    lane: usize,
    /// `0` left, `1` right.
    channel: usize,
    /// The lane's `terms` (OQ1 = C), as the block's drain left it.
    terms: bool,
    /// The stage mute: the stage's at the block's first sample (after the drain), then each
    /// composition the driver pushed.
    muted: bool,
    events: MuteEvents<'a>,
    /// The next event, its offset in the block.
    next: Option<automation::Event>,
    /// The `SetMute { muted, samples }` pushed at the offset being applied, if any: the lane's
    /// fader channel takes its hold from it (#1651 T3b).
    pushed: Option<(bool, u32)>,
}

/// One mute channel's events in a block (#1655 V2): a lone cell's or a composed curve's, whose
/// value is `1.0` when the curves mute.
enum MuteEvents<'a> {
    Cell(automation::CellEvents<'a, 'a>),
    Composed {
        program: &'a automation::Program,
        curve: &'a mut MuteComposite,
    },
}

impl MuteCellCursor<'_> {
    /// Pull the cell's next event.
    fn pull(&mut self) {
        self.next = match &mut self.events {
            MuteEvents::Cell(events) => events.next(),
            MuteEvents::Composed { program, curve } => curve
                .walk
                .next_event(program, MuteComposite::composition)
                .map(|event| automation::Event {
                    offset: event.offset,
                    value: f32::from(u8::from(event.value)),
                    ramp: event.ramp,
                    kind: event.kind,
                    held: event.held,
                }),
        };
    }
}

impl<'a> FaderEventDriver<'a> {
    /// The block of `frames` samples of every automated lane in `lanes`, `(owner lane, lane's
    /// automation)`; `hold(lane, channel)` is the stage's state of that channel after the block's
    /// live drain. A lane with no node time this block (no timeline, no jump length, or a span
    /// saturated at the `i64` edge) holds its value and emits nothing.
    ///
    /// D2, departure 4: a lane's first block, with no seek reaching it, lands a `Set` at offset
    /// 0 when the curve at the block's node time differs from the value the lane was seeded
    /// with, so a fresh plan (seeded with the curve at `-a(n)`) emits none and a successor's
    /// restarted lane is set to its curve. A seek step in that first block lands instead: the
    /// samples before it precede the seek on the node.
    fn new(
        lanes: impl Iterator<Item = (usize, &'a mut LaneFaderAutomation)>,
        frames: u32,
        first_sample: u64,
        hold: impl Fn(usize, usize) -> Option<ChannelHold>,
    ) -> Self {
        let _ = first_sample;
        let mut driver = Self {
            cursors: core::array::from_fn(|_| None),
            mutes: core::array::from_fn(|_| None),
            #[cfg(any(test, feature = "test-support"))]
            first_sample,
        };
        if frames == 0 {
            return driver;
        }
        let mut used = 0_usize;
        let mut mutes_used = 0_usize;
        for (lane, automation) in lanes {
            let LaneFaderAutomation {
                program,
                channels,
                mute,
                span,
                fader_jump,
                mute_jump,
                vca_jumps,
                fresh,
                ..
            } = automation;
            let first_block = core::mem::replace(fresh, false);
            let (Some(span), Some(jump), Some(mute_jump), Some([vca_offset_jump, vca_mute_jump])) =
                (*span, *fader_jump, *mute_jump, *vca_jumps)
            else {
                continue;
            };
            let length = i64::from(frames);
            if span.start.checked_add(length).is_none()
                || span
                    .step
                    .is_some_and(|step| step.timeline.checked_add(length).is_none())
            {
                continue;
            }
            let program: &'a automation::Program = program;
            // #1651 D1: the lane's mute cells, before its fader cells, so each fader channel's
            // first piece is cut at its first mute event (T3b). A mute cell gives no grid event
            // (a step curve is constant between its discontinuities), only its jumps over the
            // block's mute length and a seek's exact landing; its first block lands a `Set` when
            // the curve at the block's node time differs from its seed, as a fader cell's does
            // (D2, departure 4).
            let lane_mutes = mutes_used;
            for (channel, cell) in mute.iter_mut().enumerate() {
                let Some(AutomatedMuteCell {
                    curve,
                    terms,
                    reposition,
                }) = cell.as_mut()
                else {
                    continue;
                };
                let refused =
                    || debug_assert!(false, "an automated mute cell is refused at render");
                // #1651 T4: a repositioned cell (a carried lane whose table changed or was
                // gained) is placed at the block's node time in its own table, and the block
                // opens with a ramped landing over the mute length, which pushes `SetMute` only
                // when `curve || terms` differs from the stage's mute. A seek step in the block
                // lands instead (the seek sets the lane exactly where it applies). A composed
                // curve (#1655 D4) places each cursor in its own cell.
                let adopted = core::mem::replace(reposition, false);
                let mut mode = controls::MUTE_CELL_RULE.seek;
                let mut step_with = |differs: bool| {
                    span.step
                        .map(|step| (step.offset, step.timeline))
                        .or_else(|| {
                            if adopted {
                                mode = automation::SeekMode::Ramp;
                                return Some((0, span.start));
                            }
                            (first_block && differs).then_some((0, span.start))
                        })
                };
                let muted = hold(lane, channel).is_some_and(|hold| hold.muted);
                let events = match curve {
                    MuteCurve::Cell { cell, state } => {
                        // Unreachable: the cell index was prepared from this same program. In
                        // release the channel is skipped: it emits nothing and the lane keeps its
                        // mute, no panic.
                        let Some(cell_program) = program.cell(*cell as usize) else {
                            refused();
                            continue;
                        };
                        if adopted {
                            state.reposition(&cell_program, span.start);
                        }
                        let value = automation::value_at(cell_program.segments(), span.start);
                        let step =
                            step_with(state.target().map(f32::to_bits) != Some(value.to_bits()));
                        let Ok(node) = automation::NodeSpan::new(span.start, step, frames) else {
                            refused();
                            continue;
                        };
                        MuteEvents::Cell(automation::CellEvents::new(
                            cell_program,
                            state,
                            node,
                            mode,
                            mute_jump,
                        ))
                    }
                    MuteCurve::Composed(curve) => {
                        let curve: &'a mut MuteComposite = curve;
                        if adopted {
                            for cursor in curve.walk.cursors_mut() {
                                if let Some(cell) = program.cell(cursor.cell() as usize) {
                                    cursor.state_mut().reposition(&cell, span.start);
                                }
                            }
                        }
                        curve.set_jumps(mute_jump, vca_mute_jump);
                        let value = curve.mutes_at(program, span.start);
                        let step = step_with(curve.walk.target() != Some(value));
                        let Ok(node) = automation::NodeSpan::new(span.start, step, frames) else {
                            refused();
                            continue;
                        };
                        curve.walk.start(program, node, mode);
                        MuteEvents::Composed { program, curve }
                    }
                };
                // Unreachable refusal: the graph's step lies inside the block, and at most
                // `MAXIMUM_FADER_CELLS` mute cursors are used (two channels per lane). In release
                // the channel is skipped: it emits nothing and the lane keeps its mute.
                let Some(slot) = driver.mutes.get_mut(mutes_used) else {
                    refused();
                    continue;
                };
                let mut cursor = MuteCellCursor {
                    lane,
                    channel,
                    terms: *terms,
                    muted,
                    events,
                    next: None,
                    pushed: None,
                };
                cursor.pull();
                *slot = Some(cursor);
                mutes_used += 1;
            }
            for (channel, cell) in channels.iter_mut().enumerate() {
                let Some(AutomatedFaderCell {
                    curve,
                    offsets_sum,
                    obligation,
                    adoption,
                    ..
                }) = cell.as_mut()
                else {
                    continue;
                };
                let refused =
                    || debug_assert!(false, "an automated fader cell is refused at render");
                let hold_now = hold(lane, channel);
                let owed = core::mem::replace(adoption, Adoption::None);
                let first_step = |differs: bool| {
                    span.step
                        .map(|step| (step.offset, step.timeline))
                        .or_else(|| (first_block && differs).then_some((0, span.start)))
                };
                // The channel's first-block step and the adoption rule (#1648 D2, #1711): one
                // rule on a lone cell and on a composed curve (#1655 V2).
                let (mut step, landing, mut placed) = match curve {
                    FaderCurve::Cell { cell, state } => {
                        // Unreachable: the cell index was prepared from this same program. In
                        // release the channel is skipped: it emits nothing and the lane holds its
                        // gain, no panic.
                        let Some(cell_program) = program.cell(*cell as usize) else {
                            refused();
                            continue;
                        };
                        let value = automation::value_at(cell_program.segments(), span.start);
                        let step =
                            first_step(state.target().map(f32::to_bits) != Some(value.to_bits()));
                        let landing = settle_adoption(
                            owed,
                            span,
                            hold_now,
                            &mut CellAdoption {
                                program: &cell_program,
                                state: &mut *state,
                                offsets_sum: *offsets_sum,
                            },
                        );
                        (
                            step,
                            landing,
                            Placed::Cell {
                                program: cell_program,
                                state,
                            },
                        )
                    }
                    FaderCurve::Composed(curve) => {
                        let curve: &'a mut FaderComposite = curve;
                        curve.set_jumps(jump, vca_offset_jump);
                        let value = curve.value_at(program, *offsets_sum, span.start);
                        let step = first_step(
                            curve.walk.target().map(f32::to_bits) != Some(value.to_bits()),
                        );
                        let landing = settle_adoption(
                            owed,
                            span,
                            hold_now,
                            &mut ComposedAdoption {
                                program,
                                curve: &mut *curve,
                                offsets_sum: *offsets_sum,
                            },
                        );
                        (step, landing, Placed::Composed(curve))
                    }
                };
                let mut mode = controls::FADER_CELL_RULE.seek;
                if landing && step.is_none() {
                    // The adoption jump (#1648 D2): a ramped landing at the block's first sample
                    // over the block's jump length, to the curve at the jump's completion.
                    step = Some((0, span.start));
                    mode = automation::SeekMode::Ramp;
                }
                let (free_from, at, release) = obligation.block(hold_now, frames);
                if release && let Some(first) = span_time(span.start, step, 0) {
                    // R17-10: a live write freed the gain the stage held through the previous
                    // block, so the cell's hold of the ramp it ended is released here, owed or
                    // not: its grid moves the lane from this block's first sample on (and a jump
                    // deferred to that ramp's end is dropped with it, #1711 D1).
                    placed.release_at_entry(first.saturating_sub(1));
                }
                let end = at.map(|at| hold_end(span.start, step, at));
                // The channel's first mute event (#1651 T3b): the first piece ends there.
                let mute_at = driver
                    .mutes
                    .get(lane_mutes..mutes_used)
                    .unwrap_or(&[])
                    .iter()
                    .flatten()
                    .find(|mute| mute.channel == channel)
                    .and_then(|mute| mute.next.map(|event| event.offset));
                // Unreachable refusal: at most `MAXIMUM_FADER_CELLS` cursors are used (two
                // channels per lane, and the const assert bounds every bank width by
                // `MAXIMUM_BANK_LANES`); and `hold_end` is `Some` because the block's node times
                // were checked against the `i64` edge above. In release the channel is skipped:
                // it emits nothing and the lane holds its gain.
                let (Some(slot), false) = (driver.cursors.get_mut(used), matches!(end, Some(None)))
                else {
                    refused();
                    continue;
                };
                let end = end.flatten();
                let last = piece_last(end, mute_at, frames, 0);
                // Unreachable: a step kept inside the piece lies inside it. In release
                // the channel is skipped: it emits nothing and the lane holds its gain.
                let Some(node) = piece_span(span.start, step, 0, last) else {
                    refused();
                    continue;
                };
                let walk = match placed {
                    Placed::Cell { program, state } => ChannelEvents::Cell {
                        program,
                        events: Some(automation::CellEvents::new(
                            program, state, node, mode, jump,
                        )),
                    },
                    Placed::Composed(curve) => {
                        curve.walk.start(program, node, mode);
                        ChannelEvents::Composed { program, curve }
                    }
                };
                let mut cursor = FaderCellCursor {
                    lane,
                    channel,
                    offsets_sum: *offsets_sum,
                    jump,
                    walk,
                    base: 0,
                    last,
                    next: None,
                    obligation,
                    free_from,
                    end,
                    mute_at,
                    release: None,
                    start: span.start,
                    step,
                    frames,
                };
                cursor.pull();
                *slot = Some(cursor);
                used += 1;
            }
        }
        driver
    }

    /// The fader gain of curve value `value` on a channel whose offset sum is `offsets_sum`, or
    /// `None` when the composed value has no fader gain (unreachable: #1645 bounds the curve to
    /// the fader domain and `vca_compose_db` clamps, #1646 D6).
    fn gain(value: f32, offsets_sum: f64) -> Option<f32> {
        builtins::checked_fader_gain(session::vca_compose_db(value, offsets_sum)).ok()
    }

    /// The fader operation of `event` on `cursor`'s channel, or `None` when its value has no fader
    /// gain ([`FaderCellCursor::gain`]).
    fn operation(
        cursor: &FaderCellCursor<'_>,
        event: automation::Event,
    ) -> Option<builtins::FaderOperation> {
        let gain = cursor.gain(event.value)?;
        Some(if event.held {
            builtins::FaderOperation::RememberGain { gain }
        } else {
            match event.kind {
                automation::EventKind::Set => builtins::FaderOperation::SetGain { gain },
                automation::EventKind::Grid | automation::EventKind::Jump => {
                    builtins::FaderOperation::RetargetGain {
                        gain,
                        samples: event.ramp,
                    }
                }
            }
        })
    }

    /// Consume every event the stage did not pull (none, unless a stage stopped pulling, which
    /// fails its own `debug_assert`: the cell's state must still advance through its block). A
    /// cut the stage did not reach starts its second piece with no move.
    fn finish(mut self) {
        for cursor in self.cursors.iter_mut().flatten() {
            cursor.mute_at = None;
            loop {
                while cursor.next.is_some() {
                    cursor.pull();
                }
                if let Some(end) = cursor.end.filter(|end| end.at <= cursor.last) {
                    let _ = cursor.cut(end, false);
                }
                if !cursor.open(cursor.last.saturating_add(1)) {
                    break;
                }
            }
        }
        for cursor in self.mutes.iter_mut().flatten() {
            while cursor.next.is_some() {
                cursor.pull();
            }
        }
    }
}

impl HeldRampObligation {
    /// The block's hold of one channel from its stage state `hold` after the live drain: the
    /// first offset at which the gain is free, `E` when the hold ends with the gain free in this
    /// block (a ramp that ends exactly at the block's end makes `E` the next block's first sample,
    /// when a move is owed), and whether the caller releases the cell's hold at the block's first
    /// sample (R17-10). A channel the stage held through the end of the previous block and that
    /// is free at this block's entry with no move due had its hold cleared by a live write there:
    /// the write discharges an owed obligation (constraint 4), and the cell's hold of the ramp it
    /// ended is released whether or not a move was owed. A hold that ended by itself (a ramp that
    /// ended inside the previous block, or on its last sample) needs no release: the cell's hold
    /// of it ended at the same sample.
    fn block(&mut self, hold: Option<ChannelHold>, frames: u32) -> (u32, Option<u32>, bool) {
        let due = core::mem::take(&mut self.due_at_entry);
        let held_through = core::mem::take(&mut self.held_through);
        match hold {
            Some(ChannelHold {
                held: true,
                muted: false,
                remaining,
                ..
            }) if remaining > 0 => {
                if remaining < frames {
                    (remaining, Some(remaining), false)
                } else {
                    self.due_at_entry = remaining == frames;
                    self.held_through = true;
                    (frames, None, false)
                }
            }
            Some(ChannelHold { held: true, .. }) => {
                self.held_through = true;
                (frames, None, false)
            }
            Some(_) if due && self.owed => (0, Some(0), false),
            Some(_) => {
                // A live write that freed a held stage at this entry (an exact unmute) discharges
                // the move the hold owed; a stale `owed` would land a move over `J` at a later
                // hold's end with nothing remembered there (#1711 G4,
                // `an_exact_unmute_that_frees_a_held_lane_discharges_the_owed_move`).
                self.owed = false;
                (0, None, held_through && !due)
            }
            None => (0, None, false),
        }
    }
}

/// The curve of one adopted fader channel, as the adoption rule reads and moves it
/// ([`settle_adoption`]): a lone cell ([`CellAdoption`]) or a composed curve
/// ([`ComposedAdoption`], #1655 V2), so both take one rule.
trait AdoptedCurve {
    /// The old curve's last recorded ramp completion, before a reposition clears its state.
    fn ramp_completion(&self) -> Option<i64>;
    /// Place the curve at node time `t` in its own table(s) (a reposition, #1637 D4).
    fn reposition(&mut self, t: i64);
    /// Report a ramp of the stage that completes at node time `end` (the grid holds inside it).
    fn hold_until(&mut self, end: i64);
    /// Whether a curve has a discontinuity (a step of its own) at node time `t`.
    fn steps_at(&self, t: i64) -> bool;
    /// Keep a running composed entry event's ordinary ordering when the successor agrees at
    /// every carried component value's origin. A lone cell retains its entry landing.
    fn continue_entry_step(&mut self, _start: i64) -> bool {
        false
    }
    /// The channel's value toward a ramp started at `start` and completing at `at`
    /// ([`curve_toward`]; a composed curve composes each).
    fn toward(&self, start: i64, at: i64) -> f32;
    /// The fader gain of the channel's value `value`.
    fn gain(&self, value: f32) -> Option<f32>;
    /// A repositioned curve whose comparison did not differ takes `value`, the channel's value
    /// toward `start..at`, as its current target (the stage already holds its gain).
    fn seed(&mut self, start: i64, at: i64, value: f32);
    /// Defer the curve's jump to node time `at` ([`automation::CellState::defer_jump`]).
    fn defer_jump(&mut self, at: i64);
}

/// A lone cell's curve under the adoption rule.
struct CellAdoption<'p, 'a, 's> {
    program: &'p automation::CellProgram<'a>,
    state: &'s mut automation::CellState,
    offsets_sum: f64,
}

/// A composed curve under the adoption rule (#1655 V2): each of the cell rule's moves acts on
/// every cursor.
struct ComposedAdoption<'p, 's> {
    program: &'p automation::Program,
    curve: &'s mut FaderComposite,
    offsets_sum: f64,
}

impl AdoptedCurve for CellAdoption<'_, '_, '_> {
    fn ramp_completion(&self) -> Option<i64> {
        self.state.ramp_completion()
    }
    fn reposition(&mut self, t: i64) {
        self.state.reposition(self.program, t);
    }
    fn hold_until(&mut self, end: i64) {
        self.state.hold_until(end);
    }
    fn steps_at(&self, t: i64) -> bool {
        self.program.discontinuities().binary_search(&t).is_ok()
    }
    fn toward(&self, start: i64, at: i64) -> f32 {
        curve_toward(self.program, start, at)
    }
    fn gain(&self, value: f32) -> Option<f32> {
        FaderEventDriver::gain(value, self.offsets_sum)
    }
    fn seed(&mut self, _start: i64, _at: i64, value: f32) {
        self.state.seed_target(value);
    }
    fn defer_jump(&mut self, at: i64) {
        self.state.defer_jump(at);
    }
}

impl AdoptedCurve for ComposedAdoption<'_, '_> {
    fn ramp_completion(&self) -> Option<i64> {
        self.curve.walk.output_state().ramp_completion()
    }
    /// Each cursor is placed afresh in its own cell at `t` and hears its curve there: with no
    /// target of its own, a deferred jump fires and a landing lands on every curve, so the
    /// composed value is the successor's tables' wherever the comparison differs (the cursors'
    /// carried targets belong to the tables they were built on).
    fn reposition(&mut self, t: i64) {
        let program = self.program;
        for cursor in self.curve.walk.cursors_mut() {
            let Some(cell) = program.cell(cursor.cell() as usize) else {
                continue;
            };
            let state = cursor.state_mut();
            *state = automation::CellState::new();
            state.reposition(&cell, t);
            cursor.set_heard_at(automation::value_at(cell.segments(), t), t);
        }
    }
    fn hold_until(&mut self, end: i64) {
        self.curve.each_state(|state| state.hold_until(end));
    }
    fn steps_at(&self, t: i64) -> bool {
        self.curve.walk.cursors().iter().any(|cursor| {
            self.program
                .cell(cursor.cell() as usize)
                .is_some_and(|cell| cell.discontinuities().binary_search(&t).is_ok())
        })
    }
    fn continue_entry_step(&mut self, start: i64) -> bool {
        if !self.curve.cursor_history_valid
            || !self.curve.walk.cursors().iter().all(|cursor| {
                self.program
                    .cell(cursor.cell() as usize)
                    .is_some_and(|cell| cursor.values_match(&cell))
            })
        {
            return false;
        }
        for cursor in self.curve.walk.cursors_mut() {
            if let Some(cell) = self.program.cell(cursor.cell() as usize) {
                cursor.state_mut().reindex(&cell, start);
            }
        }
        true
    }
    fn toward(&self, start: i64, at: i64) -> f32 {
        self.curve.toward(self.program, self.offsets_sum, start, at)
    }
    fn gain(&self, value: f32) -> Option<f32> {
        builtins::checked_fader_gain(value).ok()
    }
    fn seed(&mut self, start: i64, at: i64, value: f32) {
        let program = self.program;
        for cursor in self.curve.walk.cursors_mut() {
            let Some(cell) = program.cell(cursor.cell() as usize) else {
                continue;
            };
            let origin = curve_target_time(&cell, start, at);
            let heard = automation::value_at(cell.segments(), origin);
            cursor.seed_at(heard, origin);
        }
        self.curve.walk.seed_target(value);
    }
    fn defer_jump(&mut self, at: i64) {
        self.curve.walk.defer_jump(at);
    }
}

/// #1648 D2, at an adopted channel's first block: settle what [`LaneFaderAutomation::adopt`] left
/// owed; `true` when the block must open with the adoption jump (a ramped landing at its first
/// sample over the block's jump length `J`).
///
/// A [`Adoption::Reposition`] places the cursor at the block's node time in the cell's own table
/// (#1637 D4) and reports the stage's ramp again (batch 13b: `reposition` drops the one the
/// carried state held). On a held stage that report changes no sample: a held stage only
/// remembers a gain operation of either kind, and the obligation is decided by where the gain is
/// free, not by the event's flag (batch 17 U3 fold, MINOR 8: kept as bookkeeping, with no audible
/// gate). On a free stage still in a ramp of its own (a live level move over its own length,
/// applied at this block's entry or heard before the swap, or a jump), the ramp is not retargeted
/// (#1711 D1, R19-1): the cell compares the curve where the ramp ends, where the stage's target
/// was taken (U3 fold 2, D15-7). An unmoved target keeps the ramp; a moved one defers the jump to
/// the ramp's first free sample ([`automation::CellState::defer_jump`]), where the cell jumps over
/// `J` as at a discontinuity of its table, so the ramp ends when it was due.
///
/// Then the curve is compared: for a reposition whose stage runs no ramp at all, held or free,
/// at the old cell's last ramp completion (#2096, extending #1716 fix D's amendment of #1648
/// D2): the node time from which the stage's target was taken, even off the grid or while held.
/// With no recorded completion it keeps fix D's comparison at the sample before the block.
/// For a running free composed channel with a step at entry, matching values at every carried
/// cursor's heard and non-held target origins preserve its event state and ordinary merged
/// entry walk (#2158), even inside an earlier jump. Otherwise, for a held stage in a mute or
/// unmute ramp, a step at entry (B17-R13), or [`Adoption::Compare`], compare at entry.
/// Composed with this plan's `S` (`checked_fader_gain(vca_compose_db(curve, S))`, #1648
/// departure 4), when it differs in bits from the gain the stage aims for (a free
/// stage's ramp target, #1711 F0; a held stage's fader gain) the block opens with the adoption
/// jump. A repositioned cell whose comparison does not differ takes the curve's value as its
/// current target (`seed_target`: the stage already holds its gain), and a continued one keeps
/// its carried state. A block with a seek step lands the seek instead of the adoption jump (the
/// caller's step wins), and a seek landing drops a deferred jump.
fn settle_adoption(
    adoption: Adoption,
    span: TimelineSpan,
    hold: Option<ChannelHold>,
    curve: &mut impl AdoptedCurve,
) -> bool {
    if adoption == Adoption::None {
        return false;
    }
    // Compare where the stage's target was taken: its cell's last completion when no stage
    // ramp runs (#2096), or the end of the ramp a free stage still runs (U3 fold 2). A step
    // of the curve at this block's entry keeps the entry-time comparison (B17-R13).
    let mut at = span.start;
    // The free stage's ramp's first free sample, where a moved target's jump is deferred to.
    let mut free = None;
    let mut carried_completion = None;
    if adoption == Adoption::Reposition {
        if hold.is_some_and(|hold| !hold.held && hold.remaining > 0 && hold.stage_gain.is_some())
            && curve.steps_at(span.start)
            && curve.continue_entry_step(span.start)
        {
            // #2158: each cursor still hears the same successor value at the origin from which
            // it was taken. Keep its held/deferred state and reindex only its discontinuities;
            // the ordinary merged walk processes the real entry step once. A new landing would
            // advance other riding cursors to this step's completion and change their arithmetic.
            return false;
        }
        let completion = curve.ramp_completion();
        curve.reposition(span.start);
        if let Some(ChannelHold { remaining: 0, .. }) = hold
            && !curve.steps_at(span.start)
            && let Some(previous) = completion.or_else(|| span.start.checked_sub(1))
        {
            // Read before reposition clears the old cell's completion (#2096). Off the grid
            // a held stage may remember the curve at a completion still ahead of this block;
            // span.start - 1 is the target's time only at a grid-aligned block. A free stage
            // with no ramp takes the same comparison. A fresh/exact-set cell has no recorded
            // ramp, so it keeps fix D's previous-sample fallback. A step at span.start still
            // jumps there (B17-R13).
            at = previous;
            carried_completion = completion;
        } else if let Some(ChannelHold {
            held, remaining, ..
        }) = hold
            && let Some(last) = remaining.checked_sub(1)
            && let Some(end) = node_time_at(span, u64::from(last))
        {
            // A held stage: the held ramp again (batch 13b). A free stage: the ramp it runs is the
            // cell's own and `reposition` dropped its hold, so the cell holds it again.
            curve.hold_until(end);
            // A step of the cell's own at the block's first sample jumps over `J` there, as the
            // cell does at any discontinuity (and as a plan prepared with this table does), so
            // the curve is compared at the block's first sample (B17-R13): `curve_toward` looks
            // only past `start` and would skip it.
            if !held && !curve.steps_at(span.start) {
                at = end;
                free = node_time_at(span, u64::from(remaining));
            }
        }
    }
    let value = curve.toward(span.start, at);
    let differs = curve
        .gain(value)
        .zip(hold.and_then(|hold| hold.stage_gain))
        .is_some_and(|(gain, stage)| gain.to_bits() != stage.to_bits());
    if !differs {
        if adoption == Adoption::Reposition {
            curve.seed(span.start, at, value);
            if let Some(completion) = carried_completion {
                // The target still belongs to that completion. Keep its history (and grid
                // hold), so a second no-op adoption before the next grid compares there too.
                curve.hold_until(completion);
            }
        }
        return false;
    }
    match free {
        Some(free) => {
            curve.defer_jump(free);
            false
        }
        None => true,
    }
}

/// Release the hold of a cell whose stage stopped holding the lane at node time `release` (an
/// early unmute, #1651 T3b): the ramp a deferred jump waits for ended with it, so the deferral is
/// dropped too (#1711 D1; a held grid would otherwise freeze the lane, #1651 U1 MAJOR-1).
fn release_lane_hold(state: &mut automation::CellState, release: i64) {
    state.release_hold_by(release);
    state.clear_deferred_jump();
}

/// The cut at offset `at` of a block whose offset 0 is node time `start`, with seek step `step`;
/// `None` past the `i64` range (unreachable: the caller checked the block's span).
fn hold_end(start: i64, step: Option<(u32, i64)>, at: u32) -> Option<HoldEnd> {
    let node = span_time(start, step, u64::from(at))?;
    Some(HoldEnd { at, node })
}

impl builtins::FaderOperationSource for FaderEventDriver<'_> {
    fn next_offset(&mut self) -> Option<u32> {
        let mutes = self
            .mutes
            .iter()
            .flatten()
            .filter_map(|cursor| cursor.next.map(|event| event.offset));
        self.cursors
            .iter()
            .flatten()
            .filter_map(FaderCellCursor::next_offset)
            .chain(mutes)
            .min()
    }

    fn apply(&mut self, offset: u32, sink: &mut builtins::FaderOperationSink) {
        #[cfg(any(test, feature = "test-support"))]
        let first_sample = self.first_sample;
        for cursor in self.cursors.iter_mut().flatten() {
            let event = cursor.next.filter(|event| event.offset == offset);
            let end = cursor.end.filter(|end| end.at == offset);
            if event.is_none() && end.is_none() {
                continue;
            }
            let selector = channel_selector(cursor.channel);
            // R17-8: a seek `Set` that lands exactly at `E` sets the lane there (the gain is
            // free from `E`) and discharges the obligation (D3 wins over constraint 7).
            let set_at_end = end.is_some()
                && event.is_some_and(|event| event.kind == automation::EventKind::Set);
            // Constraint 7: otherwise the move at `E` replaces any gain operation the cell gives
            // at `E`.
            let moving = end.is_some() && cursor.obligation.owed && !set_at_end;
            let mut push = |cursor: &FaderCellCursor<'_>, event: automation::Event| {
                let pushed = Self::operation(cursor, event)
                    .is_some_and(|operation| sink.push(cursor.lane, selector, operation).is_ok());
                // Unreachable refusal: one cursor per lane and channel, and constraint 7 with
                // R17-8 push one gain operation per cursor and offset (no `Duplicate`); the lane
                // is a populated member; `vca_compose_db` clamps to the fader domain, so the
                // conversion gives a finite gain. In release the event is dropped: the stage
                // keeps its gain target (a hold, no NaN), no panic.
                debug_assert!(pushed, "a stored fader event is refused at render");
                #[cfg(any(test, feature = "test-support"))]
                test_only_record_fader_event(TestOnlyFaderEvent {
                    lane: cursor.lane,
                    channel: cursor.channel,
                    sample: first_sample.saturating_add(u64::from(offset)),
                    kind: event.kind,
                    held: event.held,
                    value: event.value,
                    ramp: event.ramp,
                    mute: false,
                });
            };
            if let Some(event) = event {
                if !moving {
                    push(cursor, event);
                    // The stage only remembers a gain while it holds the lane.
                    if offset < cursor.free_from {
                        cursor.obligation.owed = true;
                    }
                }
                cursor.pull();
            }
            if let Some(end) = end {
                if set_at_end {
                    cursor.obligation.owed = false;
                }
                let landing = cursor.cut(end, moving);
                if moving {
                    cursor.obligation.owed = false;
                    match landing {
                        Some(landing) => push(cursor, landing),
                        // Unreachable: a cursor's events are `Some` but after a refusal, the
                        // one-sample span is valid, and a `Ramp` seek step always lands (`land`
                        // returns an event). In release no move is pushed and the lane holds the
                        // gain the held ramp aimed for, no panic.
                        None => debug_assert!(false, "a moving cut lands its cell"),
                    }
                }
            }
            // The piece ends here: the next starts at `offset + 1`, unless the channel's mute
            // event at `offset` sets its hold first (below).
            if cursor.last == offset && cursor.mute_at != Some(offset) {
                let _ = cursor.open(offset.saturating_add(1));
            }
        }
        // #1651 D1: each mute cell's event at `offset` composes the stage mute `curve || terms`,
        // and only a change pushes `SetMute`: over the event's ramp (the block's mute jump
        // length) for a jump, exactly (`0`) for a seek landing. The sink applies it after the
        // gain operations of the offset (gain first, #1638 D4).
        for cursor in self.mutes.iter_mut().flatten() {
            let Some(event) = cursor.next.filter(|event| event.offset == offset) else {
                continue;
            };
            cursor.pull();
            cursor.pushed = None;
            let muted = curve_mutes(event.value) || cursor.terms;
            if muted == cursor.muted {
                continue;
            }
            cursor.muted = muted;
            let samples = match event.kind {
                automation::EventKind::Set => 0,
                automation::EventKind::Grid | automation::EventKind::Jump => event.ramp,
            };
            let pushed = sink
                .push(
                    cursor.lane,
                    channel_selector(cursor.channel),
                    builtins::FaderOperation::SetMute { muted, samples },
                )
                .is_ok();
            // Unreachable refusal: one mute cursor per lane and channel (no `Duplicate`), the lane
            // is a populated member, and a mute operation carries no gain. In release the stage
            // keeps its mute, no panic.
            debug_assert!(pushed, "a stored mute event is refused at render");
            if pushed {
                cursor.pushed = Some((muted, samples));
            }
            #[cfg(any(test, feature = "test-support"))]
            test_only_record_fader_event(TestOnlyFaderEvent {
                lane: cursor.lane,
                channel: cursor.channel,
                sample: self.first_sample.saturating_add(u64::from(offset)),
                kind: event.kind,
                held: false,
                value: event.value,
                ramp: samples,
                mute: true,
            });
        }
        // #1651 T3b: a fader channel whose piece ended at its mute event takes the hold the
        // pushed `SetMute` gives (an unpushed one, which changed nothing, keeps the old hold),
        // and its next piece starts at `offset + 1`, cut at the new `E` or at the channel's next
        // mute event. Bounded: one search of at most `MAXIMUM_FADER_CELLS` mute cursors per
        // fader channel with a mute event at `offset`.
        for cursor in self.cursors.iter_mut().flatten() {
            if cursor.mute_at != Some(offset) {
                continue;
            }
            let mute = self
                .mutes
                .iter()
                .flatten()
                .find(|mute| mute.lane == cursor.lane && mute.channel == cursor.channel);
            cursor.mute_at = mute.and_then(|mute| mute.next.map(|event| event.offset));
            if let Some((muted, samples)) = mute.and_then(|mute| mute.pushed) {
                cursor.rehold(offset, muted, samples);
            }
            let _ = cursor.open(offset.saturating_add(1));
        }
    }
}
// REALTIME_POLICY_END

/// Test-only: one stored fader event the driver gave the stage (#1647b, gate 3's event probe).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[doc(hidden)]
pub struct TestOnlyFaderEvent {
    /// The owner lane: the bank lane, `0` on the scalar oracle.
    pub lane: usize,
    /// `0` left, `1` right.
    pub channel: usize,
    /// The render sample the event applies at: the block's first sample plus its offset.
    pub sample: u64,
    /// What produced it.
    pub kind: automation::EventKind,
    /// A held grid event (a `RememberGain`).
    pub held: bool,
    /// The curve value, before `S`; on a composed channel (#1655 V2) the composed fader dB, `S`
    /// and the VCA curves included.
    pub value: f32,
    /// The ramp length.
    pub ramp: u32,
    /// A mute event (#1651): a `SetMute` the driver pushed, `value` the curve value (`0` or
    /// `1`) and `ramp` the mute window; else a fader event.
    pub mute: bool,
}

#[cfg(any(test, feature = "test-support"))]
const FADER_EVENT_CAPACITY: usize = 64;

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy)]
struct TestOnlyFaderEvents {
    len: usize,
    overflow: bool,
    entries: [TestOnlyFaderEvent; FADER_EVENT_CAPACITY],
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static FADER_EVENTS: core::cell::Cell<TestOnlyFaderEvents> = const {
        core::cell::Cell::new(TestOnlyFaderEvents {
            len: 0,
            overflow: false,
            entries: [TestOnlyFaderEvent {
                lane: 0,
                channel: 0,
                sample: 0,
                kind: automation::EventKind::Grid,
                held: false,
                value: 0.0,
                ramp: 0,
                mute: false,
            }; FADER_EVENT_CAPACITY],
        })
    };
}

/// Append one event to this thread's probe. Allocation-free: a copy in and out of a fixed array.
#[cfg(any(test, feature = "test-support"))]
fn test_only_record_fader_event(event: TestOnlyFaderEvent) {
    FADER_EVENTS.with(|cell| {
        let mut events = cell.get();
        match events.entries.get_mut(events.len) {
            Some(free) => {
                *free = event;
                events.len += 1;
            }
            None => events.overflow = true,
        }
        cell.set(events);
    });
}

/// Test-only (#1647b): take the stored fader events this thread's fader owners gave their stages
/// since the last take, in order, and whether the probe's capacity overflowed.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_take_fader_events() -> (Vec<TestOnlyFaderEvent>, bool) {
    FADER_EVENTS.with(|cell| {
        let events = cell.get();
        cell.set(TestOnlyFaderEvents {
            len: 0,
            overflow: false,
            ..events
        });
        (events.entries[..events.len].to_vec(), events.overflow)
    })
}

/// One VCA's cells in the plan's program (#1655 D1), `[row][lane]`: row 0 `offset_db`, row 1
/// `mute`; lane 0 left, 1 right; `None` for a lane no entry drives.
pub type VcaCells = [[Option<u32>; 2]; 2];

/// A fader owner's lane automation at bind ([`LaneFaderAutomation::new`]). Control side.
///
/// # Panics
///
/// When a composed channel's cursor table cannot be allocated: the failure of any allocation of
/// the bind (as a `Box::new` aborts). The plan's cells are its own program's, and a composed
/// channel has at least one, so the table's other refusals are unreachable.
fn bound_lane_automation(
    program: &Arc<automation::Program>,
    strip: &PreparedStripAutomation,
) -> LaneFaderAutomation {
    match LaneFaderAutomation::new(program, strip) {
        Ok(automation) => automation,
        Err(error) => panic!("a prepared lane binds its automation: {error:?}"),
    }
}

/// One automated fader channel as preparation computed it (#1646 D1, D2), with its offsets cell
/// (#1649), which the strip's [`FaderCells`] and its fader owner share.
#[derive(Clone, Debug)]
struct PreparedFaderCell {
    /// The index in the program of the lane's own fader cell; `None` on a fader event lane whose
    /// own fader is not automated (#1655 D3).
    cell: Option<u32>,
    /// The cells of the automated VCA `offset_db` lanes that reach the lane, in ascending VCA ID
    /// (#1655 D1); empty when none does (the lane renders its own cell alone, as before #1655).
    vca: Box<[u32]>,
    /// The prepared own value: the own entry's first `start_value`, the curve at node time
    /// `-a(n)` for every `a(n)` (D2, the hold rule); `own_db` with no own cell.
    value: f32,
    /// The member's static fader dB (#1655 D3), the lane's own value with no own cell.
    own_db: f32,
    /// The lane's prepared fader dB, `vca_compose_db(value, S)` (#1655 D2: `S` and then each
    /// reaching VCA curve's first `start_value`), composed once inside
    /// [`lane::fpenv::CanonicalFpEnv`] (D2). It is the lane's prepared fader parameter, so the
    /// lowering bakes it through the same `checked_fader_gain` path a static fader takes.
    fader_db: f32,
    /// `S` (`controls::EffectiveStrip::offsets_sum`).
    offsets_sum: f64,
    /// The offsets cell's ramp word.
    offsets_ramp: u32,
    /// The offsets cell, created unwritten (#1646 departure 5).
    offsets: Arc<OffsetsLane>,
}

/// One automated mute channel as preparation computed it (#1651 D1, batch 18 amendment).
#[derive(Clone, Debug)]
struct PreparedMuteCell {
    /// The index in the program of the lane's own mute cell, after every fader cell; `None` on a
    /// mute event lane whose own mute is not automated (#1655 D4).
    cell: Option<u32>,
    /// The cells of the automated VCA `mute` lanes that reach the lane, in ascending VCA ID.
    vca: Box<[u32]>,
    /// The prepared own curve value: the entry's first `start_value` (`0` or `1`), the curve at
    /// node time `-a(n)` for every `a(n)` (the hold rule, as #1646 D2 for the fader); `0` with no
    /// own cell.
    value: f32,
    /// The prepared curve mute: `own || v_1 || ... || v_m` at the curves' first `start_value`s.
    curve_muted: bool,
    /// `terms`: the lane's mute without the curve, `controls::EffectiveStrip::mute` (OQ1 = C: the
    /// strip's own static mute, any reaching VCA's, or the solo term).
    terms: bool,
}

/// Whether a mute curve value mutes: a step curve holds `0` or `1` (#1645 D1); `-0.0` is `0`.
const fn curve_mutes(value: f32) -> bool {
    value != 0.0
}

/// One automated strip's prepared cells (#1646, #1651): its fader channels and its mute channels,
/// `[left, right]` each; `None` for a channel no entry drives.
#[derive(Clone, Debug)]
struct PreparedStripAutomation {
    fader: [Option<PreparedFaderCell>; 2],
    mute: [Option<PreparedMuteCell>; 2],
}

/// The stored fader and mute automation of one preparation (#1646, #1651): the program every
/// automated lane reads, its byte report, and each automated strip's channels.
struct FaderAutomationPlan {
    program: Arc<automation::Program>,
    /// [`automation::ProgramBuilder::bytes`] of the program.
    program_bytes: u64,
    /// Each strip that has an automated fader or mute lane, by track ID.
    strips: BTreeMap<Box<str>, PreparedStripAutomation>,
    /// Each VCA with an automated `offset_db` or `mute` lane (#1655 D1), by VCA ID: the program's
    /// cell index of each `[row][lane]` (row 0 `offset_db`, row 1 `mute`), `None` for a lane no
    /// entry drives.
    vcas: BTreeMap<Box<str>, VcaCells>,
}

impl FaderAutomationPlan {
    /// The prepared cells of strip `track`, if any fader or mute channel is automated.
    fn strip(&self, track: &str) -> Option<&PreparedStripAutomation> {
        self.strips.get(track)
    }

    /// The prepared dB of each lane of strip `track`: the cell's composed `fader_db` on an
    /// automated lane (D2), `effective` elsewhere.
    fn fader_db(&self, track: &str, effective: [f32; 2]) -> [f32; 2] {
        let Some(cells) = self.strip(track) else {
            return effective;
        };
        [0, 1].map(|lane| {
            cells.fader[lane]
                .as_ref()
                .map_or(effective[lane], |cell| cell.fader_db)
        })
    }

    /// The prepared stage mute of each lane of strip `track` (#1651 D1): `curve || terms` on an
    /// automated mute lane, with the curve at its prepared value, `effective` elsewhere.
    fn mute(&self, track: &str, effective: [bool; 2]) -> [bool; 2] {
        let Some(cells) = self.strip(track) else {
            return effective;
        };
        [0, 1].map(|lane| {
            cells.mute[lane]
                .as_ref()
                .map_or(effective[lane], |cell| cell.curve_muted || cell.terms)
        })
    }

    /// The bytes the program retains: its shared allocation and the builder's report.
    fn retained_program_bytes(&self) -> Option<u64> {
        fader_automation_sizes()
            .program_overhead_bytes
            .checked_add(self.program_bytes)
    }

    /// The composed channels' bytes among `members` (#1655 R19-6): each composed fader or mute
    /// channel's boxed curve and its cursor table (one cursor per curve: the reach pairs), and the
    /// largest of those single allocations.
    fn composed_bytes<'a>(&self, members: impl Iterator<Item = &'a str>) -> Option<(u64, u64)> {
        let sizes = fader_automation_sizes();
        let mut total = 0_u64;
        let mut largest = 0_u64;
        for cells in members.filter_map(|track| self.strip(track)) {
            let faders = cells
                .fader
                .iter()
                .flatten()
                .filter(|cell| cell.cell.is_none() || !cell.vca.is_empty())
                .map(|cell| {
                    (
                        sizes.fader_composite_bytes,
                        usize::from(cell.cell.is_some()) + cell.vca.len(),
                    )
                });
            let mutes = cells
                .mute
                .iter()
                .flatten()
                .filter(|cell| cell.cell.is_none() || !cell.vca.is_empty())
                .map(|cell| {
                    (
                        sizes.mute_composite_bytes,
                        usize::from(cell.cell.is_some()) + cell.vca.len(),
                    )
                });
            for (curve, cursors) in faders.chain(mutes) {
                let table = sizes
                    .cursor_bytes
                    .checked_mul(u64::try_from(cursors).ok()?)?;
                total = total.checked_add(curve)?.checked_add(table)?;
                largest = largest.max(curve).max(table);
            }
        }
        Some((total, largest))
    }

    /// The automated fader channels among `members`: one offsets cell each (#1649).
    fn automated_channels<'a>(&self, members: impl Iterator<Item = &'a str>) -> Option<u64> {
        members
            .filter_map(|track| self.strip(track))
            .map(|cells| cells.fader.iter().filter(|cell| cell.is_some()).count())
            .try_fold(0_u64, |total, count| {
                total.checked_add(u64::try_from(count).ok()?)
            })
    }
}

/// The fixed retained sizes of stored fader automation (#1646 D4), measured on the build that
/// runs, never written as literals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FaderAutomationSizes {
    /// The shared allocation that holds a plan's program, beyond the builder's byte report
    /// ([`automation::ProgramBuilder::bytes`]), which is charged with it once per plan.
    pub program_overhead_bytes: u64,
    /// One automated channel's offsets cell: its shared allocation (the three-word
    /// `LatestCell<3>` and its dirty word).
    pub offsets_cell_bytes: u64,
    /// One lane's automation slot on a fader owner (the event state, `S`, the ramp word and the
    /// program handle of both channels). A fader bank with an automated member holds one per bank
    /// lane.
    pub lane_slot_bytes: u64,
    /// One composed fader channel's boxed curve (#1655 V2): its output state and `own_db`, beside
    /// its cursor table.
    pub fader_composite_bytes: u64,
    /// One composed mute channel's boxed curve (#1655 V2), beside its cursor table.
    pub mute_composite_bytes: u64,
    /// One cursor of a composed channel's table: one (member channel, curve) pair's event state
    /// (R19-6), `automation::Composed::bytes(1)`.
    pub cursor_bytes: u64,
}

/// The retained sizes stored fader automation charges (#1646 D4): a plan with automation charges
/// its program once, each fader bank with an automated member one slot per bank lane, and each
/// automated channel one offsets cell, as builtin bank payload.
#[must_use]
pub fn fader_automation_sizes() -> FaderAutomationSizes {
    let bytes = |size: usize| u64::try_from(size).unwrap_or(u64::MAX);
    FaderAutomationSizes {
        program_overhead_bytes: bytes(core::mem::size_of::<SharedAllocation<automation::Program>>()),
        offsets_cell_bytes: bytes(core::mem::size_of::<SharedAllocation<OffsetsLane>>()),
        lane_slot_bytes: bytes(core::mem::size_of::<Option<LaneFaderAutomation>>()),
        fader_composite_bytes: bytes(core::mem::size_of::<FaderComposite>()),
        mute_composite_bytes: bytes(core::mem::size_of::<MuteComposite>()),
        cursor_bytes: bytes(automation::Composed::<f32>::bytes(1)),
    }
}

/// Compile the session's stored fader and mute automation (#1646 D1, D2; #1651 D1): one cell per
/// automated fader lane (a `both` entry gives two), in strip order and then channel order, built
/// once with the fader row's cell rule (`controls::FADER_CELL_RULE`), then one cell per automated
/// mute lane in the same order, with the mute row's rule (`controls::MUTE_CELL_RULE`). The mute
/// cells follow every fader cell, so a session with no mute automation builds the program it built
/// before #1651, cell for cell. The VCA cells (#1655 D1) follow every mute cell. `None` when no
/// row that renders is automated, so a session without stored automation prepares exactly as
/// before; a session with mute (or VCA) automation only builds those cells only.
///
/// Each automated mute lane is seeded with its entry's first `start_value` and with `terms`, the
/// strip's [`controls::EffectiveStrip::mute`] (OQ1 = C); its prepared stage mute is
/// `curve || terms` ([`FaderAutomationPlan::mute`]).
///
/// Each lane's prepared value is its entry's first `start_value` and its `S` the strip's
/// [`controls::EffectiveStrip::offsets_sum`]; the composition `vca_compose_db(value, S)` runs
/// here, and its gain is baked by the lowering, both inside the [`lane::fpenv::CanonicalFpEnv`]
/// the caller holds, as render's are (D2).
///
/// # Errors
///
/// `host.automation.fader` at the entry when the entry has no segment, a lane's composed value
/// has no fader gain or the program builder refuses its cell, and at `$.automation` when the
/// program does not fit or cannot be allocated. The refusals at an entry are defensive: the
/// session validator refuses an empty segment list, a `start_value` outside the fader domain
/// (`vca_compose_db` then clamps), and every segment the builder would refuse (a time at or past
/// `2^53` samples, an order or spacing fault) before preparation, so a validated session reaches
/// none of them. They stay typed refusals so a broken invariant never panics in preparation.
fn prepare_fader_automation(
    session: &CompiledSession,
    effective: &controls::Effective,
) -> Result<Option<FaderAutomationPlan>, BuiltinDiagnostic> {
    let model = session.normalized_model();
    if !model
        .automation
        .iter()
        .any(|entry| controls::automation_row_renders(&entry.target))
    {
        return Ok(None);
    }
    let ramp = controls::LiveRamps::for_session(model).for_row(controls::LiveRampRow::VcaOffset);
    // #1844: one pass over the table; every strip and VCA looks its driving entries up.
    let index = controls::AutomationIndex::of(model);
    let refuse = |code: &'static str, entry: &session::Automation| {
        diag(code, &format!("$.automation[id={}]", entry.id.as_str()))
    };
    // One segment table per cell, in program order, with its entry, its row's refusal code and its
    // row's rule; transient.
    let mut tables: Vec<(
        &session::Automation,
        Vec<automation::Segment>,
        controls::CellRule,
        &'static str,
    )> = Vec::new();
    let mut strips: BTreeMap<Box<str>, PreparedStripAutomation> = BTreeMap::new();
    // #1655: the VCA reach, read only when an entry drives a VCA row (a session without VCA
    // automation reads its strips' own entries alone, as before). Indices into `model.vcas`, in
    // ascending VCA ID.
    let vca_rows = model.automation.iter().any(|entry| {
        controls::vca_offset_row(&entry.target) || controls::vca_mute_row(&entry.target)
    });
    let reach: Vec<Vec<usize>> = if vca_rows {
        model.vca_reach()
    } else {
        Vec::new()
    };
    let reach_of = |index: usize| reach.get(index).map_or(&[][..], Vec::as_slice);
    // The fader and mute event lanes that automated VCA lanes reach, `(strip, lane, VCAs)`: their
    // VCA cells follow every mute cell, so they are filled in once those exist. Transient.
    let mut composed_faders: Vec<(Box<str>, usize, Vec<usize>)> = Vec::new();
    let mut composed_mutes: Vec<(Box<str>, usize, Vec<usize>)> = Vec::new();
    // The reaching VCAs (indices) whose `mute` (else `offset_db`) row drives lane `lane`.
    let reaching = |strip: usize, lane: usize, mute: bool| -> Vec<usize> {
        reach_of(strip)
            .iter()
            .copied()
            .filter(|&vca| {
                model.vcas.get(vca).is_some_and(|vca| {
                    let entries = if mute {
                        index.vca_mute(&vca.id)
                    } else {
                        index.vca_offset(&vca.id)
                    };
                    entries[lane].is_some()
                })
            })
            .collect()
    };
    for (strip_index, (strip, values)) in model.strips().zip(effective.strips.iter()).enumerate() {
        let entries = index.fader(strip.id);
        let events = if vca_rows {
            index.fader_event_lanes(strip.id, reach_of(strip_index))
        } else {
            entries.map(|entry| entry.is_some())
        };
        if !events.contains(&true) {
            continue;
        }
        let mut cells = [None, None];
        for (lane, entry) in entries.into_iter().enumerate() {
            if !events.get(lane).copied().unwrap_or(false) {
                continue;
            }
            let own_db = [strip.fader.left_db, strip.fader.right_db][lane];
            let (cell, value) = match entry {
                Some(entry) => {
                    let segments = controls::stored_segments(entry);
                    // Defensive: the session validator refuses an entry with no segment.
                    let value = segments
                        .first()
                        .ok_or_else(|| refuse("host.automation.fader", entry))?
                        .start_value;
                    let cell = u32::try_from(tables.len())
                        .map_err(|_| diag("host.automation.fader", "$.automation"))?;
                    tables.push((
                        entry,
                        segments,
                        controls::FADER_CELL_RULE,
                        "host.automation.fader",
                    ));
                    (Some(cell), value)
                }
                None => (None, own_db),
            };
            let offsets_sum = values.offsets_sum[lane];
            let vcas = if vca_rows {
                reaching(strip_index, lane, false)
            } else {
                Vec::new()
            };
            // The caller's body holds `CanonicalFpEnv` across this composition and the lowering
            // that bakes its gain (D2). A lane reached by VCA curves composes once its VCA cells
            // exist (below).
            let fader_db = session::vca_compose_db(value, offsets_sum);
            if vcas.is_empty() {
                // Defensive: the session validator bounds every `start_value` to the fader
                // domain (#1645) and `vca_compose_db` clamps to it, so a validated session never
                // fails this domain check. It refuses with a typed diagnostic rather than reach
                // the lowering's `expect("preflighted parameters")`.
                if let Some(entry) = entry {
                    builtins::checked_fader_gain(fader_db)
                        .map_err(|_| refuse("host.automation.fader", entry))?;
                }
            } else {
                composed_faders.push((Box::<str>::from(strip.id.as_str()), lane, vcas));
            }
            cells[lane] = Some(PreparedFaderCell {
                cell,
                vca: Box::default(),
                value,
                own_db,
                fader_db,
                offsets_sum,
                offsets_ramp: ramp,
                offsets: Arc::new(OffsetsLane::new()),
            });
        }
        strips.insert(
            Box::<str>::from(strip.id.as_str()),
            PreparedStripAutomation {
                fader: cells,
                mute: [None, None],
            },
        );
    }
    // #1651 D1: the mute cells, after every fader cell.
    for (strip_index, (strip, values)) in model.strips().zip(effective.strips.iter()).enumerate() {
        let entries = index.mute(strip.id);
        let events = if vca_rows {
            index.mute_event_lanes(strip.id, reach_of(strip_index))
        } else {
            entries.map(|entry| entry.is_some())
        };
        if !events.contains(&true) {
            continue;
        }
        let mut cells = [None, None];
        for (lane, entry) in entries.into_iter().enumerate() {
            if !events.get(lane).copied().unwrap_or(false) {
                continue;
            }
            let (cell, value) = match entry {
                Some(entry) => {
                    let segments = controls::stored_segments(entry);
                    // Defensive: the session validator refuses an entry with no segment.
                    let value = segments
                        .first()
                        .ok_or_else(|| refuse("host.automation.mute", entry))?
                        .start_value;
                    let cell = u32::try_from(tables.len())
                        .map_err(|_| diag("host.automation.mute", "$.automation"))?;
                    tables.push((
                        entry,
                        segments,
                        controls::MUTE_CELL_RULE,
                        "host.automation.mute",
                    ));
                    (Some(cell), value)
                }
                None => (None, 0.0),
            };
            if vca_rows {
                let vcas = reaching(strip_index, lane, true);
                if !vcas.is_empty() {
                    composed_mutes.push((Box::<str>::from(strip.id.as_str()), lane, vcas));
                }
            }
            cells[lane] = Some(PreparedMuteCell {
                cell,
                vca: Box::default(),
                value,
                curve_muted: curve_mutes(value),
                terms: values.mute[lane],
            });
        }
        strips
            .entry(Box::<str>::from(strip.id.as_str()))
            .or_insert(PreparedStripAutomation {
                fader: [None, None],
                mute: [None, None],
            })
            .mute = cells;
    }
    // #1655 D1: the VCA cells, after every mute cell, in ascending VCA ID, then row (`offset_db`
    // with the fader's rule, `mute` with the mute rule), then lane: one cell per automated VCA
    // lane, shared by every member it reaches. A session without VCA automation builds the program
    // it built before, cell for cell.
    let mut vcas: BTreeMap<Box<str>, VcaCells> = BTreeMap::new();
    if vca_rows {
        let mut order: Vec<usize> = (0..model.vcas.len()).collect();
        order.sort_by(|&one, &two| model.vcas[one].id.as_str().cmp(model.vcas[two].id.as_str()));
        for at in order {
            let vca = &model.vcas[at];
            let rows = [
                (index.vca_offset(&vca.id), controls::FADER_CELL_RULE),
                (index.vca_mute(&vca.id), controls::MUTE_CELL_RULE),
            ];
            if rows
                .iter()
                .all(|(entries, _)| entries.iter().all(Option::is_none))
            {
                continue;
            }
            let mut cells = [[None; 2]; 2];
            for (row, (entries, rule)) in rows.into_iter().enumerate() {
                for (lane, entry) in entries.into_iter().enumerate() {
                    let Some(entry) = entry else { continue };
                    let segments = controls::stored_segments(entry);
                    // Defensive: the session validator refuses an entry with no segment.
                    if segments.is_empty() {
                        return Err(refuse("host.automation.vca", entry));
                    }
                    cells[row][lane] = Some(
                        u32::try_from(tables.len())
                            .map_err(|_| diag("host.automation.vca", "$.automation"))?,
                    );
                    tables.push((entry, segments, rule, "host.automation.vca"));
                }
            }
            vcas.insert(Box::<str>::from(vca.id.as_str()), cells);
        }
    }
    // Each composed lane takes its reaching VCA cells, in ascending VCA ID, and its prepared
    // value: the own value with `S` and then each VCA curve's first `start_value` (D2), composed
    // inside the caller's `CanonicalFpEnv`.
    let vca_cell = |vca: usize, row: usize, lane: usize| {
        let cells = vcas.get(model.vcas.get(vca)?.id.as_str())?;
        cells.get(row)?.get(lane).copied().flatten()
    };
    let first_value = |cell: u32| {
        tables
            .get(cell as usize)
            .and_then(|(_, segments, _, _)| segments.first())
            .map(|first| first.start_value)
    };
    for (track, lane, reaching) in composed_faders {
        let Some(prepared) = strips
            .get_mut(&track)
            .and_then(|strip| strip.fader.get_mut(lane))
            .and_then(Option::as_mut)
        else {
            continue;
        };
        let cells: Vec<u32> = reaching
            .iter()
            .filter_map(|&vca| vca_cell(vca, 0, lane))
            .collect();
        let seeds = cells
            .iter()
            .map(|&cell| first_value(cell))
            .collect::<Option<Vec<f32>>>()
            .ok_or_else(|| diag("host.automation.vca", "$.automation"))?;
        prepared.fader_db = composed_db(prepared.value, prepared.offsets_sum, seeds.into_iter());
        // Defensive, as the lone cell's check: `vca_compose_db` clamps to the fader domain.
        builtins::checked_fader_gain(prepared.fader_db)
            .map_err(|_| diag("host.automation.vca", "$.automation"))?;
        prepared.vca = cells.into_boxed_slice();
    }
    for (track, lane, reaching) in composed_mutes {
        let Some(prepared) = strips
            .get_mut(&track)
            .and_then(|strip| strip.mute.get_mut(lane))
            .and_then(Option::as_mut)
        else {
            continue;
        };
        let cells: Vec<u32> = reaching
            .iter()
            .filter_map(|&vca| vca_cell(vca, 1, lane))
            .collect();
        for &cell in &cells {
            let seed =
                first_value(cell).ok_or_else(|| diag("host.automation.vca", "$.automation"))?;
            prepared.curve_muted |= curve_mutes(seed);
        }
        prepared.vca = cells.into_boxed_slice();
    }
    let descriptions: Vec<automation::CellDescription<'_>> = tables
        .iter()
        .map(|(_, segments, rule, _)| rule.describe(segments))
        .collect();
    let program_bytes = automation::ProgramBuilder::bytes(&descriptions);
    // A refusal at a cell is defensive: the session validator refuses every segment the builder
    // refuses (#1646b departure 2, batch 17 follow-up), so only `TooLarge` and `Allocation`, the
    // program's own size and memory, can reach a validated session.
    let program = automation::ProgramBuilder::build(&descriptions).map_err(|error| {
        let cell = match error {
            automation::BuildError::NoSegments { cell }
            | automation::BuildError::Segment { cell, .. }
            | automation::BuildError::Order { cell, .. }
            | automation::BuildError::NonPositiveExponential { cell, .. }
            | automation::BuildError::Grid { cell }
            | automation::BuildError::Spacing { cell, .. } => Some(cell),
            automation::BuildError::TooLarge | automation::BuildError::Allocation => None,
        };
        cell.and_then(|cell| tables.get(cell)).map_or_else(
            || diag("host.automation.fader", "$.automation"),
            |&(entry, _, _, code)| refuse(code, entry),
        )
    })?;
    Ok(Some(FaderAutomationPlan {
        program: Arc::new(program),
        program_bytes,
        strips,
        vcas,
    }))
}

/// Words of one input trim cell: the `trim_db` bits and the ramp window.
const TRIM_CELL_WORDS: usize = 2;
/// Words of one input polarity cell: `0` or `1` and the ramp window.
const POLARITY_CELL_WORDS: usize = 2;
/// Words of one input filter cell: the final pair's two values, then the section's six
/// coefficients (`[c1, a2, a3, m0, m1, m2]`), all as bits.
const FILTER_CELL_WORDS: usize = 8;
/// The input section's two filter sections: the HPF (`0`) and the LPF (`1`).
const FILTER_SECTIONS: usize = 2;
/// Cells of one input stage: trim left and right, polarity left and right, then the filter cells
/// of section 0 and section 1, left before right. The cell index is its dirty bit, and the stage's
/// canonical drain order (#1346 D2).
const INPUT_CELLS: usize = 4 + 2 * FILTER_SECTIONS;
const TRIM_FIRST: usize = 0;
const POLARITY_FIRST: usize = 2;
const FILTER_FIRST: usize = 4;
const _: () = assert!(
    INPUT_CELLS <= u64::BITS as usize,
    "a cell index is a dirty bit"
);

/// What one strip's input stage shares between its writer (control) and its reader (render): the
/// trim, polarity and filter cells and their one dirty word (#1346 D1).
struct InputLane {
    trim: [LatestCell<TRIM_CELL_WORDS>; 2],
    polarity: [LatestCell<POLARITY_CELL_WORDS>; 2],
    filter: [LatestCell<FILTER_CELL_WORDS>; 2 * FILTER_SECTIONS],
    dirty: DirtyWord,
}

impl InputLane {
    fn new() -> Self {
        Self {
            trim: core::array::from_fn(|_| LatestCell::new()),
            polarity: core::array::from_fn(|_| LatestCell::new()),
            filter: core::array::from_fn(|_| LatestCell::new()),
            dirty: DirtyWord::new(),
        }
    }
}

/// The channels a lane selector addresses, as cell indices (left `0`, right `1`).
fn selected_channels(lanes: BuiltinLaneSelector) -> core::ops::RangeInclusive<usize> {
    match lanes {
        BuiltinLaneSelector::Left => 0..=0,
        BuiltinLaneSelector::Right => 1..=1,
        BuiltinLaneSelector::Both => 0..=1,
    }
}

/// Control's write end of one strip's input trim, polarity and filter cells (#1346 D6). A write
/// cannot fail on a value and never waits.
pub struct InputCells {
    lane: Arc<InputLane>,
    trim: [CellWriter<TRIM_CELL_WORDS>; 2],
    polarity: [CellWriter<POLARITY_CELL_WORDS>; 2],
    filter: [CellWriter<FILTER_CELL_WORDS>; 2 * FILTER_SECTIONS],
}

impl InputCells {
    /// Write `record` as the newest value of the cell(s) it addresses, stamped with the revision of
    /// the transaction that commits it. A `Both` record writes the left and the right cell; a
    /// filter target writes the cells of its section.
    ///
    /// The caller publishes the stamp's revision on the plan's gate after every write of the
    /// transaction (#1346 D7); a value is not applied before that.
    ///
    /// # Panics
    ///
    /// When a filter target names a section other than `0` or `1`. The caller has validated the
    /// target ([`builtins::validate_prepared_input_filter_target`]) before its first write of the
    /// transaction, as every fallible check precedes the first write; a value is never dropped
    /// for want of a cell.
    pub fn write(&mut self, stamp: LiveStamp<'_>, record: TrackInputRecord) {
        match record {
            TrackInputRecord::TrimDb {
                lanes,
                db,
                smoothing_samples,
            } => {
                let words = [db.to_bits(), smoothing_samples];
                for channel in selected_channels(lanes) {
                    self.trim[channel].write(
                        &self.lane.trim[channel],
                        &self.lane.dirty,
                        (TRIM_FIRST + channel) as u32,
                        stamp,
                        words,
                    );
                }
            }
            TrackInputRecord::PolarityInvert {
                lanes,
                inverted,
                smoothing_samples,
            } => {
                let words = [u32::from(inverted), smoothing_samples];
                for channel in selected_channels(lanes) {
                    self.polarity[channel].write(
                        &self.lane.polarity[channel],
                        &self.lane.dirty,
                        (POLARITY_FIRST + channel) as u32,
                        stamp,
                        words,
                    );
                }
            }
            TrackInputRecord::PreparedFilter { target } => {
                let section = usize::try_from(target.section)
                    .ok()
                    .filter(|section| *section < FILTER_SECTIONS)
                    .expect("a prepared filter target names section 0 or 1");
                let c = target.coefficients;
                let words = [
                    target.pair[0].to_bits(),
                    target.pair[1].to_bits(),
                    c[0].to_bits(),
                    c[1].to_bits(),
                    c[2].to_bits(),
                    c[3].to_bits(),
                    c[4].to_bits(),
                    c[5].to_bits(),
                ];
                for channel in selected_channels(target.lanes) {
                    let index = 2 * section + channel;
                    self.filter[index].write(
                        &self.lane.filter[index],
                        &self.lane.dirty,
                        (FILTER_FIRST + index) as u32,
                        stamp,
                        words,
                    );
                }
            }
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl InputCells {
    /// Test-only: how many writes each cell has taken (trim left and right, polarity left and
    /// right, then the filter cells section 0 left and right, section 1 left and right). Valid
    /// while no write is in flight, which a quiescent test thread guarantees.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_counts(&self) -> [u64; INPUT_CELLS] {
        fn count<const N: usize>(peek: Option<([u32; N], u64)>) -> u64 {
            peek.map_or(0, |(_, sequence)| sequence)
        }
        let reader = CellReader::new();
        core::array::from_fn(|index| match index {
            0..=1 => count(reader.peek_unread(&self.lane.trim[index - TRIM_FIRST])),
            2..=3 => count(reader.peek_unread(&self.lane.polarity[index - POLARITY_FIRST])),
            _ => count(reader.peek_unread(&self.lane.filter[index - FILTER_FIRST])),
        })
    }
}

/// Render's read end of one strip's input cells: the cells' readers, the stage's deferred mask and
/// the session counter (#1346 D1).
struct InputReader {
    lane: Arc<InputLane>,
    trim: [CellReader; 2],
    polarity: [CellReader; 2],
    filter: [CellReader; 2 * FILTER_SECTIONS],
    dirty: DirtyReader,
    superseded: LiveValuesSuperseded,
    /// The carry marks (#1630 f, D15-2); see [`CarryMarks`].
    marks: CarryMarks,
}

/// The value(s) a pair of channel cells applied in one drain: one `Both` value when the two words
/// are equal, else one value per applied channel (#1346 D2). A cell that read `Unchanged` is not
/// paired.
// REALTIME_POLICY_BEGIN
fn paired_channels<const N: usize>(
    left: Option<[u32; N]>,
    right: Option<[u32; N]>,
) -> [Option<(BuiltinLaneSelector, [u32; N])>; 2] {
    match (left, right) {
        (Some(left), Some(right)) if left == right => {
            [Some((BuiltinLaneSelector::Both, left)), None]
        }
        (left, right) => [
            left.map(|words| (BuiltinLaneSelector::Left, words)),
            right.map(|words| (BuiltinLaneSelector::Right, words)),
        ],
    }
}

/// Read one pending cell under `snapshot`: its newest value in range, if it was not applied
/// before. The replaced count joins `superseded`, and an applied cell marks `marks` (a carried
/// value it replaces joins the count too, #1630 f).
#[allow(clippy::too_many_arguments)]
fn read_input_cell<const N: usize>(
    pending: u64,
    bit: usize,
    cell: &LatestCell<N>,
    reader: &mut CellReader,
    snapshot: LiveSnapshot,
    dirty: &mut DirtyReader,
    superseded: &LiveValuesSuperseded,
    (marks, carried): (&mut CarryMarks, u16),
) -> Option<[u32; N]> {
    if pending & (1 << bit) == 0 {
        return None;
    }
    match reader.read_counted(cell, snapshot, dirty, bit as u32, superseded) {
        CellRead::Applied { words, .. } => {
            marks.applied(bit, carried, superseded);
            Some(words)
        }
        CellRead::Unchanged => None,
    }
}

impl InputReader {
    /// Read the stage's pending cells under `snapshot` and apply them in the canonical order
    /// (#1346 D2): trim, then polarity, then filter section 0, then section 1, left before right.
    /// When both channel cells of a kind were applied in this call with equal words, `apply` is
    /// called once with a `Both` record.
    ///
    /// Each record is admitted to `witness` before it is applied, from the cells' final contents
    /// (#1346 D3): an equal pair is a `Both` record (`Preserve`), any other pair or a lone channel
    /// is per-lane records (`Desymmetrize`), through the one `admit` hook, so a record kind still
    /// cannot reach render state without declaring its witness effect.
    ///
    /// Every applied value is applied even if an earlier one is refused; the first refusal is
    /// returned after the last. Each applied read adds the writes it replaced to the session
    /// counter (`read_counted`). It allocates nothing and takes no lock. `Ok(true)`: no cell was
    /// pending, so the read found nothing to do (issue #1778's quiet walk).
    fn apply_latest(
        &mut self,
        snapshot: LiveSnapshot,
        witness: &mut ChannelSymmetryWitness,
        mut apply: impl FnMut(TrackInputRecord) -> Result<(), BuiltinParameterError>,
    ) -> Result<bool, BuiltinParameterError> {
        let carried = self.marks.begin();
        let pending = self.dirty.pending(&self.lane.dirty);
        if pending == 0 {
            return Ok(true);
        }
        let Self {
            lane,
            trim,
            polarity,
            filter,
            dirty,
            superseded,
            marks,
        } = self;
        let trim_words: [Option<[u32; TRIM_CELL_WORDS]>; 2] = core::array::from_fn(|channel| {
            read_input_cell(
                pending,
                TRIM_FIRST + channel,
                &lane.trim[channel],
                &mut trim[channel],
                snapshot,
                dirty,
                superseded,
                (&mut *marks, carried),
            )
        });
        let polarity_words: [Option<[u32; POLARITY_CELL_WORDS]>; 2] =
            core::array::from_fn(|channel| {
                read_input_cell(
                    pending,
                    POLARITY_FIRST + channel,
                    &lane.polarity[channel],
                    &mut polarity[channel],
                    snapshot,
                    dirty,
                    superseded,
                    (&mut *marks, carried),
                )
            });
        let filter_words: [Option<[u32; FILTER_CELL_WORDS]>; 2 * FILTER_SECTIONS] =
            core::array::from_fn(|index| {
                read_input_cell(
                    pending,
                    FILTER_FIRST + index,
                    &lane.filter[index],
                    &mut filter[index],
                    snapshot,
                    dirty,
                    superseded,
                    (&mut *marks, carried),
                )
            });
        let mut first_error = Ok(());
        let mut run = |record: TrackInputRecord| {
            // The one hook. `admit` takes the record by trait, not by kind, so a record type
            // added to these cells later cannot reach the render state without declaring what it
            // does to the witness (`effect_contract::symmetry::LiveControlRecord`).
            witness.admit(&record);
            // Every value is applied; the first refusal is the one returned.
            first_error = first_error.and(apply(record));
        };
        for (lanes, words) in paired_channels(trim_words[0], trim_words[1])
            .into_iter()
            .flatten()
        {
            run(TrackInputRecord::TrimDb {
                lanes,
                db: f32::from_bits(words[0]),
                smoothing_samples: words[1],
            });
        }
        for (lanes, words) in paired_channels(polarity_words[0], polarity_words[1])
            .into_iter()
            .flatten()
        {
            run(TrackInputRecord::PolarityInvert {
                lanes,
                inverted: words[0] != 0,
                smoothing_samples: words[1],
            });
        }
        for section in 0..FILTER_SECTIONS {
            for (lanes, words) in
                paired_channels(filter_words[2 * section], filter_words[2 * section + 1])
                    .into_iter()
                    .flatten()
            {
                run(TrackInputRecord::PreparedFilter {
                    target: PreparedInputFilterTarget {
                        lanes,
                        section: section as u32,
                        pair: [f32::from_bits(words[0]), f32::from_bits(words[1])],
                        coefficients: [
                            f32::from_bits(words[2]),
                            f32::from_bits(words[3]),
                            f32::from_bits(words[4]),
                            f32::from_bits(words[5]),
                            f32::from_bits(words[6]),
                            f32::from_bits(words[7]),
                        ],
                    },
                });
            }
        }
        first_error.map(|()| false)
    }
}
// REALTIME_POLICY_END

/// One strip's input lane, as its two ends: control's writers and render's reader.
fn input_lane_cells(superseded: &LiveValuesSuperseded) -> (InputCells, InputReader) {
    let lane = Arc::new(InputLane::new());
    (
        InputCells {
            lane: Arc::clone(&lane),
            trim: core::array::from_fn(|_| CellWriter::new()),
            polarity: core::array::from_fn(|_| CellWriter::new()),
            filter: core::array::from_fn(|_| CellWriter::new()),
        },
        InputReader {
            lane,
            trim: core::array::from_fn(|_| CellReader::new()),
            polarity: core::array::from_fn(|_| CellReader::new()),
            filter: core::array::from_fn(|_| CellReader::new()),
            dirty: DirtyReader::new(),
            superseded: superseded.clone(),
            marks: CarryMarks::default(),
        },
    )
}

/// The control-side producer half of one prepared live-control channel.
///
/// Every half is a latest-target cell set (#1312, #1346): a write cannot fail, and render applies
/// the last committed value of each cell. Each is an `Arc` shared by the [`TrackControlProducer`] and the reader
/// half, so either may drop first: the last owner frees the storage, on the control thread, since the
/// render thread never drops a plan.
pub struct TrackControlProducer {
    /// Session-stable track identity this channel addresses.
    pub track_id: Box<str>,
    /// The matrix/pan stage's cell writer (#137 D1, #1312 D5).
    pub producer: MatrixCells,
    /// The fader/mute stage's cell writers (issue #140 B, #1312 D5).
    ///
    /// It is a field of the same struct rather than a second vector so that "one track, one
    /// live-control channel" stays one object with one lifetime: all three halves are created
    /// together, handed to the caller together, and dropped together.
    pub fader: FaderCells,
    /// The input trim/polarity/filter stage's cell writers (#210 phase 3, #1346 D6). `None` when
    /// the request set [`TrackControlRequest::input_lane`] to `false` (#1254 D1).
    pub input: Option<InputCells>,
}

impl TrackControlProducer {
    /// Bytes of the heap allocations this producer owns beyond its table slot and its `track_id`
    /// (#1916): today only the boxed offsets-cell writers of an automated strip's fader channels
    /// (`FaderCells`' offsets box, #1649 D4), which a strip with no fader event lane does not
    /// allocate. The cells the writers share with the plan are not here; builtins charges them.
    ///
    /// A host's strip-table charge (`host_core::strip_control_table_bytes`) sums this over the
    /// built producers, so a new heap owner of a producer belongs here.
    #[must_use]
    pub fn retained_heap_bytes(&self) -> u64 {
        if self.fader.offsets.is_some() {
            core::mem::size_of::<[Option<OffsetsWriter>; 2]>() as u64
        } else {
            0
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinDiagnostic {
    pub code: &'static str,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinDiagnosticSet(pub Vec<BuiltinDiagnostic>);

impl BuiltinDiagnosticSet {
    pub fn sorted(mut values: Vec<BuiltinDiagnostic>) -> Self {
        values.sort();
        values.dedup();
        Self(values)
    }
}

pub struct MeterConsumer {
    pub handle: MeterHandle,
    pub track_id: Box<str>,
    pub tap: MeterTap,
    pub consumer: Consumer<MeterSnapshot>,
}

/// The live controls' three consumers for one track.
///
/// They travel together because they are leased together (`TrackControlProducer` holds all three
/// producers) and because whichever owner ends up rendering the track's fader also ends up
/// rendering its matrix and its input section -- a per-node triple, or three slots of one cohort
/// chain. Keeping them in one value is what makes "one track, one live-control channel" hold on the
/// consumer side too.
struct StripControlConsumers {
    /// `None` once a strip bank has claimed this side. The three sides are claimed together --
    /// `planned_strip_banks` plans all three stages over one track list -- so a partly claimed
    /// strip is unreachable, and `strip_bindings` says so rather than papering over it. The input
    /// side is also `None` from the start when the request had `input_lane: false` (#1254 D1);
    /// that strip's input section is then the plain one, banked or per node.
    input: Option<InputReader>,
    fader: Option<FaderReader>,
    matrix: Option<MatrixReader>,
}

/// Everything one track's fader and matrix stages need, before their binding form is decided.
///
/// Both shapes are built from this: the per-node processors [`PreparedBuiltinsSession::\
/// strip_bindings`] makes, and the bank lanes `into_graph_artifact_with_banks` gathers. The
/// prepared `fader`/`matrix` sections are the scalar fallback and `parameters` is what a bank
/// lane is built from -- prepared independently, exactly as `bank_inputs` is prepared beside the
/// scalar `InputBuiltins`, so selecting one never mutates the other.
///
/// Only the scalar oracle's `strip_bindings` reads `graph_id` and the three scalar sections, and
/// it is compiled only for tests and `test-support` (issue #1059). The fields stay in every build
/// anyway: this struct's size is charged by `resource_plan` (`builtinRetainedBytes`), and a
/// feature that moved an estimate would make a test build and the shipped module report
/// different resources for the same session. Shedding the dead storage is a layout change with its
/// own re-pins, not part of gating the oracle.
struct StripPreparation {
    track_id: Box<str>,
    #[cfg_attr(
        not(any(test, feature = "test-support")),
        allow(dead_code, reason = "read only by the scalar oracle (#1059)")
    )]
    graph_id: StableGraphId,
    parameters: BuiltinParameters,
    /// The scalar input section, held here rather than bound eagerly since #210 phase 3, for the
    /// same reason the fader and the matrix are: whether a track's input is a per-node processor
    /// or one lane of a strip bank is a *lowering* decision, and the live-control consumer has to
    /// move to whichever owner wins. A track the input bank claims keeps this value as dead storage
    /// the bank never renders, exactly as a partly claimed fader does.
    #[cfg_attr(
        not(any(test, feature = "test-support")),
        allow(dead_code, reason = "read only by the scalar oracle (#1059)")
    )]
    input: InputBuiltins,
    #[cfg_attr(
        not(any(test, feature = "test-support")),
        allow(dead_code, reason = "read only by the scalar oracle (#1059)")
    )]
    fader: FaderMuteBuiltins,
    #[cfg_attr(
        not(any(test, feature = "test-support")),
        allow(dead_code, reason = "read only by the scalar oracle (#1059)")
    )]
    matrix: MatrixBuiltins,
    control: Option<StripControlConsumers>,
}

/// Opaque, sealed builtin payload. It can only be lowered into a graph once.
pub struct PreparedBuiltinsSession {
    seal: BuiltinSessionSeal,
    /// Post-input builtin bindings, one per track.
    ///
    /// The fader and matrix bindings are deliberately **not** here. Whether a track's fader is a
    /// per-node processor or one lane of a strip bank is a *lowering* decision -- it needs the
    /// selected dispatch backend and the graph's dependency levels, neither of which exists when
    /// a session is prepared -- so both stages are held as [`StripPreparation`] until then and
    /// bound by [`PreparedBuiltinsSession::strip_bindings`] (issue #212). The seal is unchanged:
    /// it still records three stages per track, and `processors_match` still proves all three are
    /// owned, counting the strip preparations alongside these bindings.
    processors: Vec<graph::GraphNodeBinding>,
    /// The fader and matrix section of every track, in normalized track order, with the live
    /// controls' consumers where they drive the track.
    ///
    /// Declared after `processors` and before `track_controls` so a dropped preparation releases
    /// the producers before the consumers that own the ring storage.
    strips: Vec<StripPreparation>,
    bank_inputs: Vec<(Box<str>, InputBuiltins)>,
    observers: Vec<GraphNodeObserverBinding>,
    meter_consumers: Vec<MeterConsumer>,
    /// Issue #137 D1: control-side producers, sealed alongside the consumers bound into the
    /// matrix processors. Declared after `processors` so a dropped preparation releases the
    /// producers before the consumers that own the ring storage.
    track_controls: Vec<TrackControlProducer>,
    /// Every strip's input-section bounds as reported (#1329 D7, Amendment 4 R5), sorted by track
    /// ID: `T_decay` (the tail graph lowering reads), `T_rest` and `RestSamples` (silence skipping,
    /// #1107). Control-side: nothing render owns carries them. A strip with a live input lane
    /// carries the rate's live bound ([`builtins::input_section_live_bound_table`]). Any other
    /// strip carries its own design's bound while the preparation's budget covers its design, and
    /// the live bound once it does not (#1457 D3, `builtins::INPUT_BOUND_BUDGET_FRAMES`, charged
    /// in strip order): a non-live strip's bound is certified for its design either way, but it
    /// is not always the design's own.
    tails: Vec<(Box<str>, NodeTailBound)>,
    requests: Vec<MeterRequestSeal>,
    resources: BuiltinResourceEstimate,
    /// The session's stored fader automation (#1646), installed on each automated lane's fader
    /// owner at lowering; `None` when no fader lane is automated. Its program and every slot and
    /// cell lowering builds for it are charged as builtin bank payload (D4).
    fader_automation: Option<FaderAutomationPlan>,
    /// Every route's effective follow gate per source lane, in `model.routes` order, as the
    /// preparation was given it (decision 17, C7; `controls::Effective::routes`). Control-side data
    /// like `tails`: graph lowering reads it for the routes' follow gates, so the gate is composed
    /// in one place and nowhere else.
    route_follow: Box<[[bool; 2]]>,
}

/// The witness of a stage that is **seam-side by design**: fader, mute, pan and matrix.
///
/// Every term holds, unconditionally and without looking at a single word. That is not an
/// optimism: a collapsed track duplicates its one plane *into* these stages, so their per-channel
/// words are free to differ and must never gate the collapse. The measured evidence for the seam
/// sitting exactly here is a session with per-track asymmetric faders and non-identity pans whose
/// collapsed and dual renders are byte-identical.
const SEAM_SIDE_WITNESS: ChannelSymmetryWitness = ChannelSymmetryWitness::SYMMETRIC;

/// One strip input bank and the live-control channels of its member lanes (#210 phase 3).
///
/// # The third drain, and why it is at bank level
///
/// The input trim, polarity and filter values ride latest-target cells (#1346) with exactly one
/// reader, and that reader is whoever renders the track's input section. The input builtins are
/// **banked** -- `BuiltinInputBank` over eight member tracks -- so the reader is this processor,
/// draining its members' cells in one loop, and not a per-track node sibling. The single-reader
/// property is preserved structurally, exactly as it is for the fader and the matrix:
/// `into_graph_artifact_with_banks` moves each [`InputReader`] out of its `StripPreparation` once,
/// so a track's input is either a bank lane or a per-node [`LiveControlInputProcessor`] and never
/// both.
///
/// # Why the drain is `begin_block` and not the first paragraph of `process`
///
/// This is the difference from [`FaderBankProcessor`], and it is load-bearing. The fader and the
/// matrix are **seam-side**: a collapsed chain duplicates its one plane *into* them, so a value
/// that writes one of their channels changes nothing the collapse dispatch needs to know, and
/// draining inside `process` is fine. The input section is **upstream** of the seam. A collapsed
/// block publishes the left plane on both channels, so a `Left` trim retarget applied at block
/// `N` must be visible to the collapse dispatch *before* block `N` decides whether to run one
/// plane -- or the right channel would receive a retarget addressed to the left one, and the bits
/// would be wrong on the one block nobody would think to look at.
/// `rack::BankStage::begin_block` states the rule; `BankChain::run` drains every slot,
/// then reads the witness, then gathers. This drain is on the correct side of that ordering.
///
/// A cell read is a few loads and a retarget performs at most one division per channel; neither
/// allocates, locks, nor drops, which is what keeps the shipped artifact's render call-graph gate
/// green.
struct BuiltinBankProcessor {
    bank: BuiltinInputBank,
    /// One reader per bank lane; `None` for a lane no live control addresses. Always `lanes` long,
    /// so the lane index is the array index and no lane map is stored.
    controls: Box<[Option<InputReader>]>,
    /// Each lane's **live** channel-symmetry terms, retained across blocks for the reason
    /// `EffectControlLane::symmetry` gives: the terms describe what the drained values did, so
    /// they have to survive the block that drained them.
    ///
    /// Inline and fixed at the widest bank rather than boxed: it is one byte per lane, and a
    /// second heap block per bank would cost more to retain than the array does.
    ///
    /// Only `LIVE` moves here. It is cleared by an admitted per-lane record and, like every other
    /// `LIVE` term in the tree, **never restored within a plan** -- see
    /// [`Self::lane_symmetry`] for why that is the conservative half of the M3 re-engage rule
    /// rather than a missing feature.
    live: [ChannelSymmetryWitness; MAXIMUM_BANK_LANES],
    /// The snapshot publication of the last block drain that walked every lane and found no cell
    /// pending, or `None` (issue #1778): see [`quiet_walk_skips`]. Every method that changes a
    /// lane's reader outside the block drain clears it.
    quiet_at: Option<u64>,
    process_calls: u64,
    frames_processed: u64,
}

/// Widest bank any backend selects (`BankWidth::Eight`), as a plain array bound.
const MAXIMUM_BANK_LANES: usize = 8;

// Every bank width this build has fits the arrays bound by `MAXIMUM_BANK_LANES` (the live
// witnesses, the lane masks and `MAXIMUM_FADER_CELLS`' cursors): a wider width fails the build.
const _: () = {
    let mut index = 0;
    while index < BankWidth::ALL.len() {
        assert!(BankWidth::ALL[index].lanes() as usize <= MAXIMUM_BANK_LANES);
        index += 1;
    }
};

/// The lane mask of a drain that visits every lane of a bank: the block drain's.
const EVERY_LANE: u8 = u8::MAX;

// REALTIME_POLICY_BEGIN
/// Whether a builtin bank's block drain under `snapshot` skips its lane walk (issue #1778): the
/// last block drain walked every lane under the same publication and found no cell pending
/// (`quiet_at`, set by [`quiet_mark`]).
///
/// The skip is exact, by the argument #1763 makes for the effect rack's quiet slot: every write of
/// a publication at or below `S` is stored and marked before `S` is published, so the walk under
/// `S` that set the mark would have seen its dirty bit; it saw none, and no later write can carry
/// a publication at or below `S`. A write above `S` waits in its dirty word, or in the deferred
/// mask, for the first drain under a newer publication, which walks. The skipped walk's
/// `CarryMarks::begin` would change nothing: the marking walk already took `carried` and cleared
/// `applied`, and applied nothing. [`LiveSnapshot::ALL`] is not a publication and never skips.
fn quiet_walk_skips(quiet_at: Option<u64>, snapshot: LiveSnapshot) -> bool {
    snapshot != LiveSnapshot::ALL && quiet_at == Some(snapshot.publication())
}

/// The quiet mark a block drain under `snapshot` leaves (issue #1778): `snapshot`'s publication
/// when the drain walked every lane and each found no cell pending (`every_lane_quiet`), else
/// `None`.
fn quiet_mark(snapshot: LiveSnapshot, every_lane_quiet: bool) -> Option<u64> {
    every_lane_quiet.then_some(snapshot.publication())
}

/// Test-only: which builtin bank skipped a lane walk (issue #1778).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy)]
enum QuietBank {
    Input,
    Fader,
    Matrix,
}

/// Test-only: count one skipped lane walk of `bank` on this thread. Allocation-free: a copy in
/// and out of a fixed array. Not compiled into a release artifact.
#[cfg(any(test, feature = "test-support"))]
fn test_only_count_quiet_skip(bank: QuietBank) {
    QUIET_WALK_SKIPS.with(|cell| {
        let mut counts = cell.get();
        counts[bank as usize] = counts[bank as usize].saturating_add(1);
        cell.set(counts);
    });
}

/// Whether bit `lane` of `lanes` is set. A bank has at most [`MAXIMUM_BANK_LANES`] lanes.
const fn lane_selected(lanes: u8, lane: usize) -> bool {
    debug_assert!(lane < MAXIMUM_BANK_LANES);
    (lanes >> (lane % MAXIMUM_BANK_LANES)) & 1 != 0
}

/// Whether [`exchange_lane_slots`] would exchange slot `lane` of `own` with slot `peer_lane` of
/// `peer` (`None`: of `own`): both are in range.
fn lane_slots_fit<T>(own: &[T], lane: usize, peer: Option<&[T]>, peer_lane: usize) -> bool {
    lane < own.len() && peer_lane < peer.unwrap_or(own).len()
}

/// Exchange slot `lane` of `own` with slot `peer_lane` of `peer` (`None`: of `own`), issue
/// #1765. Returns `false`, changing nothing, when [`lane_slots_fit`] does not hold. Moves the two
/// values; allocation-free.
fn exchange_lane_slots<T>(
    own: &mut [T],
    lane: usize,
    peer: Option<&mut [T]>,
    peer_lane: usize,
) -> bool {
    match peer {
        None if lane == peer_lane => lane < own.len(),
        None => {
            let (low, high) = (lane.min(peer_lane), lane.max(peer_lane));
            let Some((head, tail)) = own.split_at_mut_checked(high) else {
                return false;
            };
            let (Some(first), Some(second)) = (head.get_mut(low), tail.first_mut()) else {
                return false;
            };
            core::mem::swap(first, second);
            true
        }
        Some(peer) => match (own.get_mut(lane), peer.get_mut(peer_lane)) {
            (Some(first), Some(second)) => {
                core::mem::swap(first, second);
                true
            }
            _ => false,
        },
    }
}
// REALTIME_POLICY_END

/// The mask [`GraphPreparedBuiltinBankProcessor::live_lanes`] reports: bit `n` is set when lane
/// `n` of a bank holds a live reader. A bank has at most [`MAXIMUM_BANK_LANES`] lanes.
fn live_lane_mask(present: impl Iterator<Item = bool>) -> u8 {
    present.enumerate().fold(0, |mask, (lane, live)| {
        debug_assert!(lane < MAXIMUM_BANK_LANES);
        mask | (u8::from(live) << lane)
    })
}

impl BuiltinBankProcessor {
    /// Apply every pending value of each lane's input cells readable under `snapshot` to lane
    /// state, in the canonical order, folding the lane's witness from what this drain applied
    /// (#1346 D2, D3): the body of [`GraphPreparedBuiltinBankProcessor::begin_block`], and of the
    /// plan-swap drain.
    ///
    /// `lanes` names the lanes that drain (bit `n` is lane `n`): every lane for the block drain,
    /// the carried lanes for the plan-swap drain.
    ///
    /// `carry` picks what a refused apply does. The block drain (`false`) applies everything it
    /// read and returns the first refusal after the last lane, as the fader's drain does. The
    /// carry drain (`true`) counts each refusal and goes on, because this bank never drains again:
    /// the result is the refused count.
    // REALTIME_POLICY_BEGIN
    fn drain_controls(
        &mut self,
        snapshot: LiveSnapshot,
        lanes: u8,
        carry: bool,
    ) -> Result<usize, BuiltinParameterError> {
        let Self {
            bank,
            controls,
            live,
            quiet_at,
            ..
        } = self;
        let mut refused = 0_usize;
        let mut first_error = Ok(());
        let mut quiet = lanes == EVERY_LANE;
        for (lane, control) in controls.iter_mut().enumerate() {
            if !lane_selected(lanes, lane) {
                continue;
            }
            let Some(control) = control.as_mut() else {
                continue;
            };
            // `live` holds `MAXIMUM_BANK_LANES` terms and a bank never has more lanes, so every
            // lane that holds a reader has its term. The `get_mut` only keeps a slice index (a
            // panic edge) out of the render call graph; a lane without a term would skip its whole
            // drain, acknowledged values and all, so a debug build refuses it outright.
            debug_assert!(
                lane < MAXIMUM_BANK_LANES,
                "a bank lane with a reader has a symmetry term"
            );
            let Some(witness) = live.get_mut(lane) else {
                continue;
            };
            let drained = control.apply_latest(snapshot, witness, |record| {
                let applied = match record {
                    TrackInputRecord::TrimDb {
                        lanes,
                        db,
                        smoothing_samples,
                    } => bank.set_trim_db(lane, lanes, db, smoothing_samples),
                    TrackInputRecord::PolarityInvert {
                        lanes,
                        inverted,
                        smoothing_samples,
                    } => bank.set_polarity_invert(lane, lanes, inverted, smoothing_samples),
                    TrackInputRecord::PreparedFilter { target } => {
                        bank.apply_prepared_filter(lane, target)
                    }
                };
                match applied {
                    Err(_) if carry => {
                        refused = refused.saturating_add(1);
                        Ok(())
                    }
                    other => other,
                }
            });
            quiet &= matches!(drained, Ok(true));
            first_error = first_error.and(drained.map(|_| ()));
        }
        // A carry drain leaves no mark: it changed lanes outside the block drain.
        *quiet_at = quiet_mark(snapshot, quiet && !carry);
        first_error.map(|()| refused)
    }

    /// Whether [`GraphPreparedBuiltinBankProcessor::swap_builtin_lanes`] would exchange lane
    /// `lane` with `peer`'s lane `peer_lane` (`None`: this bank's), issue #1765: the banks' own
    /// check, and both lanes hold a reader slot and a symmetry term.
    fn can_swap_with(&self, lane: usize, peer: Option<&Self>, peer_lane: usize) -> bool {
        self.bank
            .can_swap_lanes(lane, peer.map(|peer| &peer.bank), peer_lane)
            && lane_slots_fit(
                &self.controls,
                lane,
                peer.map(|peer| &*peer.controls),
                peer_lane,
            )
            && lane_slots_fit(&self.live, lane, peer.map(|peer| &peer.live[..]), peer_lane)
    }
    // REALTIME_POLICY_END
}

impl GraphPreparedBuiltinBankProcessor for BuiltinBankProcessor {
    fn retained_bytes(&self) -> u64 {
        reader_table_bytes(&self.controls, InputReader::retained_bytes)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
    fn response_snapshot_declared(&self) -> bool {
        true
    }
    fn response_snapshot_native_id(&self) -> Option<&'static str> {
        Some("miso.builtin.input-filters")
    }
    /// The third drain. Runs before the collapse dispatch reads the witness -- see the type's
    /// documentation for why that ordering is the whole reason this is not folded into `process`.
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        if quiet_walk_skips(self.quiet_at, snapshot) {
            #[cfg(any(test, feature = "test-support"))]
            test_only_count_quiet_skip(QuietBank::Input);
            return Ok(());
        }
        self.drain_controls(snapshot, EVERY_LANE, false)
            .map(|_| ())
            .map_err(render_error)
    }

    /// The lanes whose input cells this bank drains (#1346; #1504 D3's slot guard reads it).
    fn live_lanes(&self) -> u8 {
        live_lane_mask(self.controls.iter().map(Option::is_some))
    }

    // REALTIME_POLICY_BEGIN
    /// The input bank's own rest mask (issue #1781, [`BuiltinInputBank::lanes_at_rest`]). The
    /// drain applies every value readable under the block's snapshot to lane state before this
    /// is asked, so the bank's words are the block's; a write the snapshot does not cover waits
    /// in its cell for a later block's drain, which runs whether or not that block is skipped,
    /// and has nothing to do in this one. So no reader term is needed.
    fn builtin_lanes_at_rest(&self) -> u32 {
        self.bank.lanes_at_rest()
    }

    /// The input's silence counters ([`BuiltinInputBank::advance_clocks`]).
    fn advance_builtin_clocks(&mut self, lanes_mask: u32, frames: u32) {
        self.bank.advance_clocks(lanes_mask, frames);
    }

    fn carried_input_lanes(&self) -> Option<usize> {
        Some(self.bank.active_lanes())
    }

    fn can_adopt_input_lane(
        &self,
        lane: usize,
        predecessor: &dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        predecessor
            .as_any()
            .downcast_ref::<Self>()
            .is_some_and(|predecessor| {
                predecessor.bank.width() == self.bank.width()
                    && lane < self.bank.active_lanes()
                    && predecessor_lane < predecessor.bank.active_lanes()
            })
    }

    /// Issue #1276 D2: the lane's whole input-section state and its live symmetry terms. The
    /// cell reader stays: every value this lane's predecessor had written was applied to lane
    /// state by [`Self::drain_for_carry`], and the successor's own cells take every later one.
    fn adopt_input_lane(
        &mut self,
        lane: usize,
        predecessor: &mut dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        let Some(predecessor) = predecessor
            .as_any_mut()
            .and_then(|any| any.downcast_mut::<Self>())
        else {
            return false;
        };
        if predecessor.bank.width() != self.bank.width() {
            return false;
        }
        let (Some(state), Some(live)) = (
            predecessor.bank.export_lane(predecessor_lane),
            predecessor.live.get(predecessor_lane).copied(),
        ) else {
            return false;
        };
        let Some(slot) = self.live.get_mut(lane) else {
            return false;
        };
        if self.bank.import_lane(lane, &state).is_err() {
            return false;
        }
        *slot = live;
        self.quiet_at = None;
        adopt_carry_marks(
            self.controls.get_mut(lane),
            predecessor.controls.get(predecessor_lane),
            |reader| &mut reader.marks,
            |reader| &reader.marks,
        );
        true
    }

    /// Issue #1276 D4, #1346 D5. A value reached these cells only after validation, so applying it
    /// cannot fail; were it to, that value is counted and every other pending value is still
    /// applied, so the carry loses none of them. It reads with [`LiveSnapshot::ALL`], which on the
    /// quiescent predecessor equals the adoption block's snapshot (#1312 D6): every value the
    /// predecessor holds was written before the successor's publication.
    fn drain_for_carry(&mut self, lanes: u8) -> usize {
        // The carry drain counts a refused apply instead of returning it, so `Err` is not
        // produced; were it, it is reported as one refused value, never as none.
        self.drain_controls(LiveSnapshot::ALL, lanes, true)
            .unwrap_or(1)
    }

    fn can_swap_builtin_lanes(
        &self,
        lane: usize,
        peer: Option<&dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        match peer {
            None => self.can_swap_with(lane, None, peer_lane),
            Some(peer) => peer
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|peer| self.can_swap_with(lane, Some(peer), peer_lane)),
        }
    }

    /// Issue #1765: the lane's input-section state and prepared endpoint
    /// ([`BuiltinInputBank::swap_lanes`]), its cell reader (cells, dirty bits, carry marks and
    /// superseded counter: the strip's live controls move with the strip) and its live symmetry
    /// terms. The block counters are the bank's and stay.
    fn swap_builtin_lanes(
        &mut self,
        lane: usize,
        peer: Option<&mut dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        let other = match peer {
            None => None,
            Some(peer) => match peer.as_any_mut().and_then(|any| any.downcast_mut::<Self>()) {
                Some(other) => Some(other),
                None => return false,
            },
        };
        if !self.can_swap_with(lane, other.as_deref(), peer_lane) {
            return false;
        }
        // One path for both peers: `None` names this bank's own field, so no field can be
        // exchanged for one kind of peer and left behind for the other.
        self.quiet_at = None;
        let (bank, controls, live) = match other {
            None => (None, None, None),
            Some(other) => {
                other.quiet_at = None;
                (
                    Some(&mut other.bank),
                    Some(&mut *other.controls),
                    Some(&mut other.live[..]),
                )
            }
        };
        let swapped = self.bank.swap_lanes(lane, bank, peer_lane).is_ok()
            && exchange_lane_slots(&mut self.controls, lane, controls, peer_lane)
            && exchange_lane_slots(&mut self.live, lane, live, peer_lane);
        debug_assert!(swapped, "a checked input lane swaps");
        swapped
    }
    // REALTIME_POLICY_END

    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn copy_response_snapshot_lane(
        &self,
        lane: usize,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.bank
            .copy_response_snapshot_lane(lane, sample_rate_hz, request)
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// The prepared designed-word comparison, conjoined with this lane's live terms.
    ///
    /// # Two independent guards on the same fact, and why both are kept
    ///
    /// `BuiltinInputBank::lane_symmetry` compares every word the input kernel loads per channel,
    /// and since #210 phase 3 that includes the **live** trim ramp record -- current, target, step
    /// and countdown -- so an asymmetric retarget is visible in the words on the very block it is
    /// admitted. The `LIVE` term this conjoins says the same thing from the other end: a record
    /// that addressed one lane was drained, whatever the words now say.
    ///
    /// They are kept side by side because they fail differently. The word comparison would call a
    /// no-op asymmetric retarget symmetric (retargeting the left channel to the value it already
    /// holds, with the right channel's window) and let the block collapse -- which is *correct*,
    /// but correct by an argument about arithmetic rather than by the structural rule the witness
    /// exists to enforce. The `LIVE` latch would allow a collapse the words forbid only if a
    /// designed word moved without a record, which cannot happen. A wrong collapse therefore needs
    /// both to be wrong at once.
    ///
    /// # What this means for re-engage (M3)
    ///
    /// `LIVE` is a latch: cleared by the drain, never set again within a plan. That is the same
    /// law `EffectControlLane` has carried since the witness existed, and it makes this bank
    /// **strictly stronger** than the M3 re-engage rule requires -- a track whose channels were
    /// driven apart by a per-lane trim ride does not come back even after the two words are made
    /// equal again and even if the integrators would prove agreement. Re-equal parameter words
    /// alone must not re-engage a collapse (M3's rule); here they cannot re-engage one at all.
    /// Making `LIVE` recoverable would be a change to the M-series machinery -- the proof would
    /// have to be consulted before the witness rather than after it -- and is deliberately not one
    /// this phase makes.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let live = self
            .live
            .get(lane)
            .copied()
            .unwrap_or(ChannelSymmetryWitness::DECLINED);
        self.bank.lane_symmetry(lane).and(live)
    }

    fn supports_mono_collapse(&self) -> bool {
        self.bank.supports_mono_collapse()
    }

    fn process_mono(
        &mut self,
        left: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.bank.process_mono(left, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn desymmetrize(&mut self) {
        self.bank.desymmetrize();
    }

    /// Whether this bank can prove its two channels' state bit-equal (M3's way back).
    ///
    /// The walk covers the four integrators per channel and the whole trim-ramp record, which is
    /// this kernel's entire per-channel state -- exactly the words `desymmetrize` copies. It is
    /// asked only inside a recovery window, so a session that never disagrees never calls it.
    fn channels_agree(&self) -> bool {
        self.bank.channels_agree()
    }
}

/// One strip fader bank and the live-control channels of its member lanes (issue #212).
///
/// # The drain contract, at bank level
///
/// `TrackFaderRecord` rides a bounded SPSC queue with exactly one consumer, and that consumer is
/// whoever renders the track's fader. Banking moves the renderer from a per-track node to one lane
/// of this bank, so the consumer moves with it -- the queue, its depth, its record semantics and
/// its slot in the host's frozen queue-slot layout are all untouched. The single-consumer property
/// is preserved because it is preserved *structurally*: `into_graph_artifact_with_banks` moves
/// each `Consumer` out of its `StripPreparation` exactly once, so a track is either a bank lane or
/// a per-node processor and never both.
///
/// This is the bank-level wiring the plan calls for, and not a per-track node sibling: there is no
/// second drainer, and the drain is one loop over member lanes at the top of the block, before a
/// single sample is touched. An admitted move therefore takes effect at exactly the block boundary
/// the control side was acknowledged with, which is the same rule `LiveControlFaderProcessor`
/// followed.
///
/// `try_pop` moves one `Copy` record and a retarget performs at most one division per channel;
/// neither allocates, locks, nor drops, which is what keeps the shipped artifact's render
/// call-graph gate green.
struct FaderBankProcessor {
    bank: BuiltinFaderBank,
    /// One channel per bank lane; `None` for a lane no live control addresses. Always `lanes` long,
    /// so the lane index is the array index and no lane map is stored.
    controls: Box<[Option<FaderReader>]>,
    /// The stored fader automation of the bank's lanes (#1646 D1): one slot per bank lane when a
    /// member's fader is automated, `None` on a lane that is not; empty (no allocation) when no
    /// member is.
    automation: Box<[Option<LaneFaderAutomation>]>,
    /// The snapshot publication of the last block drain that walked every lane and found no cell
    /// pending, or `None` (issue #1778): see [`quiet_walk_skips`]. Every method that changes a
    /// lane's reader outside the block drain clears it.
    quiet_at: Option<u64>,
    process_calls: u64,
    frames_processed: u64,
}

/// Test-only: one automated fader lane's node time for one block, as the bank took it (#1647a).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestOnlyFaderLaneTime {
    /// The bank lane.
    pub lane: usize,
    /// The lane's `a(n)`.
    pub arrival: u64,
    /// The block's node-time mapping, `None` with no timeline.
    pub span: Option<TimelineSpan>,
    /// The fader row's jump length, `None` with no jump-length cell.
    pub fader_jump: Option<u32>,
}

#[cfg(any(test, feature = "test-support"))]
const FADER_LANE_TIME_CAPACITY: usize = 16;

/// Test-only: the automated fader lanes' node times the banks took since the last take, in the
/// order they took them, and whether more than the capacity arrived.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy)]
struct TestOnlyFaderLaneTimes {
    len: usize,
    overflow: bool,
    entries: [TestOnlyFaderLaneTime; FADER_LANE_TIME_CAPACITY],
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static FADER_LANE_TIMES: core::cell::Cell<TestOnlyFaderLaneTimes> =
        const {
            core::cell::Cell::new(TestOnlyFaderLaneTimes {
                len: 0,
                overflow: false,
                entries: [TestOnlyFaderLaneTime {
                    lane: 0,
                    arrival: 0,
                    span: None,
                    fader_jump: None,
                }; FADER_LANE_TIME_CAPACITY],
            })
        };
}

/// Append each automated lane's node time to this thread's probe. Allocation-free: a copy in and
/// out of a fixed array.
#[cfg(any(test, feature = "test-support"))]
fn test_only_record_fader_lane_times(automation: &[Option<LaneFaderAutomation>]) {
    FADER_LANE_TIMES.with(|cell| {
        let mut times = cell.get();
        for (lane, slot) in automation.iter().enumerate() {
            let Some(slot) = slot else {
                continue;
            };
            let entry = TestOnlyFaderLaneTime {
                lane,
                arrival: slot.arrival,
                span: slot.span,
                fader_jump: slot.fader_jump,
            };
            match times.entries.get_mut(times.len) {
                Some(free) => {
                    *free = entry;
                    times.len += 1;
                }
                None => times.overflow = true,
            }
        }
        cell.set(times);
    });
}

/// Test-only (#1647a): take the automated fader lanes' node times this thread's banks took since
/// the last take, in order, and whether the probe's capacity overflowed. Each block's
/// `set_block_time` adds one entry per automated lane of every fader bank.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_take_fader_lane_times() -> (Vec<TestOnlyFaderLaneTime>, bool) {
    FADER_LANE_TIMES.with(|cell| {
        let times = cell.get();
        cell.set(TestOnlyFaderLaneTimes {
            len: 0,
            overflow: false,
            ..times
        });
        (times.entries[..times.len].to_vec(), times.overflow)
    })
}

impl FaderBankProcessor {
    // REALTIME_POLICY_BEGIN
    /// One block: the body of `process` and, with `from_left`, of `process_from_left` (issue
    /// #975), whose right input is the left plane and whose `right` is output only. Returns
    /// whether the bank wrote the right plane from the left one; otherwise, with `from_left`, the
    /// left plane is copied into the right one and the block renders as `process` renders it.
    fn render(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
        from_left: bool,
    ) -> bool {
        let Self {
            bank, automation, ..
        } = self;
        let took = if automation.is_empty() {
            let took = from_left && bank.process_from_left(left, right, frames);
            if !took {
                if from_left {
                    right.copy_from_slice(left);
                }
                bank.process(left, right, frames);
            }
            took
        } else {
            // #1647 D1.3-D1.5: the automated lanes' events, through the stage's timed path.
            let mut driver = FaderEventDriver::new(
                automated_lanes(automation),
                frames,
                first_sample,
                bank_hold(bank),
            );
            // A block with no operation inside it renders exactly `process` (`process_with`'s
            // contract), so only such a block takes the from-left body.
            let took = from_left
                && builtins::FaderOperationSource::next_offset(&mut driver)
                    .is_none_or(|offset| offset >= frames)
                && bank.process_from_left(left, right, frames);
            if !took {
                if from_left {
                    right.copy_from_slice(left);
                }
                // A stop is unreachable: after `apply(offset)` every cursor has consumed its event
                // and cut at `offset`, a piece's events rise strictly and a second piece starts at
                // `E + 1`, so the next offset is above it. A stop (a broken source contract) fails
                // the stage's own `debug_assert`; in release the block's rest renders with the
                // gains held and `finish` advances every cell through the block, no panic.
                let _ = bank.process_with(left, right, frames, &mut driver);
            }
            driver.finish();
            took
        };
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        took
    }

    /// The block drain under `snapshot`: the body of `begin_block`.
    fn drain(&mut self, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        let Self {
            bank,
            controls,
            automation,
            quiet_at,
            ..
        } = self;
        let result = if quiet_walk_skips(*quiet_at, snapshot) {
            #[cfg(any(test, feature = "test-support"))]
            test_only_count_quiet_skip(QuietBank::Fader);
            Ok(())
        } else {
            let drained = drain_fader_controls(bank, controls, automation, snapshot, EVERY_LANE);
            *quiet_at = quiet_mark(snapshot, matches!(drained, Ok(true)));
            drained.map(|_| ())
        };
        // The offsets cells are not the walk's: they are read every block (automated lanes only).
        read_fader_offsets(bank, controls, automation, snapshot);
        result
    }

    /// Apply every pending value of the cells of the lanes in `lanes`, whatever its revision: the
    /// block drain with [`LiveSnapshot::ALL`], the same code (#1312 D6). The plan-swap carry calls
    /// it on a quiescent predecessor, for the lanes it exports (#1277); no production render path
    /// takes `ALL`.
    ///
    /// Each carried automated lane's offsets cells are read the same way (#1648 D2, U3 fold):
    /// the newest record is kept for the successor's first block, which applies it
    /// ([`LaneFaderAutomation::carry_offsets`]).
    fn apply_pending(&mut self, lanes: u8) -> Result<(), RenderError> {
        let Self {
            bank,
            controls,
            automation,
            quiet_at,
            ..
        } = self;
        *quiet_at = None;
        let result =
            drain_fader_controls(bank, controls, automation, LiveSnapshot::ALL, lanes).map(|_| ());
        for (lane, slot) in automated_lanes(automation) {
            if !lane_selected(lanes, lane) {
                continue;
            }
            if let Some(Some(reader)) = controls.get(lane) {
                slot.carry_offsets(&reader.superseded);
            }
        }
        result
    }

    /// Whether `predecessor` is a fader bank of this bank's width with lane `predecessor_lane`
    /// populated, and `lane` is populated here (#1277 D1).
    fn can_adopt_lane(&self, lane: usize, predecessor: &Self, predecessor_lane: usize) -> bool {
        predecessor.bank.width() == self.bank.width()
            && lane < self.bank.active_lanes()
            && predecessor_lane < predecessor.bank.active_lanes()
    }

    /// Copy `predecessor`'s lane `predecessor_lane` into lane `lane` (#1277 D7, D8). The cell
    /// reader stays: the successor's own cells take every later value, and it takes the carry
    /// marks of the values the carry drain applied (#1630 f). The lane's stored automation takes
    /// the predecessor lane's event state (#1648 D2, [`LaneFaderAutomation::adopt`]).
    fn adopt_lane(
        &mut self,
        lane: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        continued_cells: u8,
    ) -> bool {
        let adopted = predecessor
            .bank
            .export_lane(predecessor_lane)
            .is_some_and(|state| self.bank.import_lane(lane, &state).is_ok());
        if adopted {
            self.quiet_at = None;
            adopt_carry_marks(
                self.controls.get_mut(lane),
                predecessor.controls.get(predecessor_lane),
                |reader| &mut reader.marks,
                |reader| &reader.marks,
            );
            if let Some(Some(automation)) = self.automation.get_mut(lane) {
                automation.adopt(
                    predecessor
                        .automation
                        .get(predecessor_lane)
                        .and_then(Option::as_ref),
                    continued_cells,
                );
            }
        }
        adopted
    }

    /// Whether [`GraphPreparedBuiltinBankProcessor::swap_builtin_lanes`] would exchange lane
    /// `lane` with `peer`'s lane `peer_lane` (`None`: this bank's), issue #1765: the banks' own
    /// check, both lanes hold a reader slot, and the stored automation fits
    /// ([`Self::automation_swap_fits`]).
    fn can_swap_with(&self, lane: usize, peer: Option<&Self>, peer_lane: usize) -> bool {
        self.bank
            .can_swap_lanes(lane, peer.map(|peer| &peer.bank), peer_lane)
            && lane_slots_fit(
                &self.controls,
                lane,
                peer.map(|peer| &*peer.controls),
                peer_lane,
            )
            && Self::automation_swap_fits(
                &self.automation,
                lane,
                peer.map_or(&self.automation, |peer| &peer.automation),
                peer_lane,
            )
    }

    /// Whether two lanes' stored automation slots can trade places. A bank none of whose members
    /// is automated holds no slots at all (`automation` is empty, #1646 D1), so a lane can move
    /// into it only when it carries no automation: the automated lane would have no slot to land
    /// in. Two banks that both hold slots, or neither, always fit.
    fn automation_swap_fits(
        own: &[Option<LaneFaderAutomation>],
        lane: usize,
        theirs: &[Option<LaneFaderAutomation>],
        peer_lane: usize,
    ) -> bool {
        match (own.is_empty(), theirs.is_empty()) {
            (false, false) => lane < own.len() && peer_lane < theirs.len(),
            (true, true) => true,
            (false, true) => own.get(lane).is_some_and(Option::is_none),
            (true, false) => theirs.get(peer_lane).is_some_and(Option::is_none),
        }
    }
    // REALTIME_POLICY_END
}

impl GraphPreparedBuiltinBankProcessor for FaderBankProcessor {
    fn retained_bytes(&self) -> u64 {
        reader_table_bytes(&self.controls, FaderReader::retained_bytes)
            + reader_table_bytes(&self.automation, LaneFaderAutomation::retained_bytes)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
    /// #1647a (R17-2): each automated lane keeps its member's `a(n)`.
    fn set_input_arrivals(&mut self, arrivals: &[u64]) {
        for (slot, &arrival) in self.automation.iter_mut().zip(arrivals) {
            if let Some(slot) = slot {
                slot.arrival = arrival;
            }
        }
    }
    // REALTIME_POLICY_BEGIN
    /// #1647a (R17-3): each automated lane takes the block's node-time mapping
    /// `timeline_block(r + ΣP - a(n))` and the fader row's jump length, resolved once from the
    /// three lengths the graph holds through `LiveRamps`, which owns the row rule. A bank with no
    /// automated lane keeps nothing and reads no timeline. Allocation-free; one timeline read per
    /// automated lane.
    fn set_block_time(&mut self, time: &GraphBlockTime<'_>) {
        if self.automation.is_empty() {
            return;
        }
        let jumps = block_jumps(time);
        for slot in self.automation.iter_mut().flatten() {
            slot.take_block_time(time, jumps);
        }
        #[cfg(any(test, feature = "test-support"))]
        test_only_record_fader_lane_times(&self.automation);
    }
    // REALTIME_POLICY_END
    // REALTIME_POLICY_BEGIN
    fn carried_strip_lanes(&self) -> Option<(BuiltinStripStage, usize)> {
        Some((BuiltinStripStage::Fader, self.bank.active_lanes()))
    }

    fn can_adopt_strip_lane(
        &self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        owner == BuiltinStripOwner::Fader
            && predecessor
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|predecessor| self.can_adopt_lane(lane, predecessor, predecessor_lane))
    }

    /// Issue #1277 D8. The cell reader stays: every value the predecessor's cells held was
    /// applied to lane state by [`Self::drain_strip_for_carry`], and the successor's own cells take
    /// every later one.
    fn adopt_strip_lane(
        &mut self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &mut dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
        continued_cells: u8,
    ) -> bool {
        owner == BuiltinStripOwner::Fader
            && predecessor
                .as_any_mut()
                .and_then(|any| any.downcast_mut::<Self>())
                .is_some_and(|predecessor| {
                    self.adopt_lane(lane, predecessor, predecessor_lane, continued_cells)
                })
    }

    /// Issue #1277 D8: apply every pending cell value of the carried lanes with every revision,
    /// as the block drain would at the next block. A value the apply refuses is counted, and the
    /// rest still apply. A lane that restarts is not read, so its replaced writes are not counted.
    fn drain_strip_for_carry(&mut self, owner: BuiltinStripOwner, lanes: u8) -> usize {
        usize::from(owner == BuiltinStripOwner::Fader && self.apply_pending(lanes).is_err())
    }

    fn can_swap_builtin_lanes(
        &self,
        lane: usize,
        peer: Option<&dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        match peer {
            None => self.can_swap_with(lane, None, peer_lane),
            Some(peer) => peer
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|peer| self.can_swap_with(lane, Some(peer), peer_lane)),
        }
    }

    /// Issue #1765: the lane's fader and mute state, ramps in flight included
    /// ([`BuiltinFaderBank::swap_lanes`]), its cell reader (cells, dirty bits, carry marks,
    /// superseded counter) and its stored automation (event cursors, held-ramp obligations, offsets
    /// cells and `a(n)`; the block's node time is set again by the next block). The block counters
    /// are the bank's and stay.
    fn swap_builtin_lanes(
        &mut self,
        lane: usize,
        peer: Option<&mut dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        let other = match peer {
            None => None,
            Some(peer) => match peer.as_any_mut().and_then(|any| any.downcast_mut::<Self>()) {
                Some(other) => Some(other),
                None => return false,
            },
        };
        if !self.can_swap_with(lane, other.as_deref(), peer_lane) {
            return false;
        }
        // One path for both peers, as the input bank's.
        self.quiet_at = None;
        let (bank, controls, automation) = match other {
            None => (None, None, None),
            Some(other) => {
                other.quiet_at = None;
                (
                    Some(&mut other.bank),
                    Some(&mut *other.controls),
                    Some(&mut *other.automation),
                )
            }
        };
        // A bank without slots trades only a lane without automation (`automation_swap_fits`):
        // both lanes' slots are `None`, and nothing moves.
        let slotless = self.automation.is_empty()
            || automation
                .as_deref()
                .is_some_and(<[Option<LaneFaderAutomation>]>::is_empty);
        let swapped = self.bank.swap_lanes(lane, bank, peer_lane).is_ok()
            && exchange_lane_slots(&mut self.controls, lane, controls, peer_lane)
            && (slotless || exchange_lane_slots(&mut self.automation, lane, automation, peer_lane));
        debug_assert!(swapped, "a checked fader lane swaps");
        swapped
    }
    // REALTIME_POLICY_END
    /// The fader and mute cells' drain, before any lane of the block is dispatched (#1312 D12).
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.drain(snapshot)
    }
    fn live_lanes(&self) -> u8 {
        live_lane_mask(self.controls.iter().map(Option::is_some))
    }
    /// Issue #1288 D2: [`BuiltinFaderBank::arm_fade_in`]; `false` for a lane past the bank.
    fn arm_fade_in(&mut self, lane: usize) -> bool {
        self.bank.arm_fade_in(lane).unwrap_or(false)
    }
    // REALTIME_POLICY_BEGIN
    /// Issue #1288 D3: [`BuiltinFaderBank::fire_fade_in`], at the block boundary before this
    /// block's drain.
    fn fire_fade_in(&mut self, lane: usize, ramp_samples: u32) {
        self.bank.fire_fade_in(lane, ramp_samples);
    }
    // REALTIME_POLICY_END
    // REALTIME_POLICY_BEGIN
    /// The fader bank's own rest mask ([`BuiltinFaderBank::lanes_at_rest`]) minus every lane
    /// that holds stored automation (issue #1781). The drain applies every live value readable
    /// under the block's snapshot first, so the bank's words are the block's, and a write the
    /// snapshot does not cover waits for a later block's drain. An automated lane is declined
    /// whatever its curve: its block runs the timed path ([`FaderEventDriver`]), which may land an
    /// operation inside the block and advances the lane's event cursors and held-ramp obligations
    /// by the block's node time on every rendered block (`FaderEventDriver::finish`), and no clock
    /// advance here reproduces that. The bank has no clock of its own, so the default no-op
    /// `advance_builtin_clocks` is exact for every lane this reports.
    fn builtin_lanes_at_rest(&self) -> u32 {
        let automated = self
            .automation
            .iter()
            .enumerate()
            .fold(0_u32, |mask, (lane, slot)| {
                mask | (u32::from(slot.is_some()) << lane)
            });
        self.bank.lanes_at_rest() & !automated
    }
    // REALTIME_POLICY_END
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        self.render(left, right, frames, first_sample, false);
        Ok(())
    }

    /// The seam on a collapsed block (issue #975): the bank writes the right plane from the left
    /// one on a block with no right-channel ramp and no timed operation inside it.
    fn process_from_left(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<bool, RenderError> {
        Ok(self.render(left, right, frames, first_sample, true))
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// Seam-side: see [`SEAM_SIDE_WITNESS`]. `TrackFaderRecord` is a seam-side record type, so
    /// its drain above deliberately folds nothing into a witness -- and could not, because
    /// `LiveControlRecord::SEAM` compiles the seam-side arm away.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        SEAM_SIDE_WITNESS
    }

    /// The fader is the first stage that reads the duplicated plane, so the seam is immediately
    /// before it. It never runs one-plane: its two channels' gains and mutes are free to differ,
    /// and on the standing mono fixture 49 of 64 tracks' faders do.
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
}

// REALTIME_POLICY_BEGIN
/// A fader bank's automated lanes, `(bank lane, automation)`, for [`FaderEventDriver::new`].
fn automated_lanes(
    automation: &mut [Option<LaneFaderAutomation>],
) -> impl Iterator<Item = (usize, &mut LaneFaderAutomation)> {
    automation
        .iter_mut()
        .enumerate()
        .filter_map(|(lane, slot)| slot.as_mut().map(|slot| (lane, slot)))
}

/// The stage state of a fader bank's lane and channel after the block's live drain, for
/// [`FaderEventDriver::new`] (#1647 D1.6).
fn bank_hold(bank: &BuiltinFaderBank) -> impl Fn(usize, usize) -> Option<ChannelHold> + '_ {
    |lane, channel| {
        let held = bank.gain_held(lane, channel)?;
        Some(ChannelHold {
            held,
            // Issue #1288, #2080 R1: an armed channel whose fire opens it is unmuted in the model
            // (see `FaderJumpStage::muted`); one whose fire stays muted is muted. A curve's
            // record that reaches the stage only retargets the fire.
            muted: bank.is_muted(lane, channel)? && bank.is_armed(lane, channel) != Some(true),
            remaining: bank.ramp_remaining(lane, channel)?,
            stage_gain: if held {
                bank.fader_gain(lane, channel)
            } else {
                bank.ramp_target(lane, channel)
            },
        })
    }
}

/// #1649 D3: each automated lane reads its offsets cells under the block's `snapshot` and jumps
/// ([`LaneFaderAutomation::read_offsets`]). A lane with no control lane has no writer, so its
/// cells stay as prepared; an adopted lane still settles its carried `S` (#1648 D2).
fn read_fader_offsets(
    bank: &mut BuiltinFaderBank,
    controls: &[Option<FaderReader>],
    automation: &mut [Option<LaneFaderAutomation>],
    snapshot: LiveSnapshot,
) {
    for (lane, slot) in automated_lanes(automation) {
        let superseded = controls
            .get(lane)
            .and_then(Option::as_ref)
            .map(|reader| &reader.superseded);
        slot.read_offsets(
            snapshot,
            superseded,
            &mut BankLaneStage {
                bank: &mut *bank,
                lane,
            },
        );
    }
}
// REALTIME_POLICY_END

/// One strip matrix/pan bank and the live-control channels of its member lanes (issue #212).
///
/// The mirror of [`FaderBankProcessor`], on `TrackControlRecord`, and it carries the same drain
/// contract for the same reason -- see that type for the argument.
struct MatrixBankProcessor {
    bank: BuiltinMatrixBank,
    /// One channel per bank lane; `None` for a lane no live control addresses.
    controls: Box<[Option<MatrixReader>]>,
    /// The snapshot publication of the last block drain that walked every lane and found no cell
    /// pending, or `None` (issue #1778): see [`quiet_walk_skips`]. Every method that changes a
    /// lane's reader outside the block drain clears it.
    quiet_at: Option<u64>,
    process_calls: u64,
    frames_processed: u64,
}

impl MatrixBankProcessor {
    // REALTIME_POLICY_BEGIN
    /// The block drain under `snapshot`: the body of `begin_block`.
    fn drain(&mut self, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        if quiet_walk_skips(self.quiet_at, snapshot) {
            #[cfg(any(test, feature = "test-support"))]
            test_only_count_quiet_skip(QuietBank::Matrix);
            return Ok(());
        }
        let Self {
            bank,
            controls,
            quiet_at,
            ..
        } = self;
        let drained = drain_matrix_controls(bank, controls, snapshot, EVERY_LANE);
        *quiet_at = quiet_mark(snapshot, matches!(drained, Ok(true)));
        drained.map(|_| ())
    }

    /// The matrix cells' counterpart of [`FaderBankProcessor::apply_pending`].
    fn apply_pending(&mut self, lanes: u8) -> Result<(), RenderError> {
        let Self {
            bank,
            controls,
            quiet_at,
            ..
        } = self;
        *quiet_at = None;
        drain_matrix_controls(bank, controls, LiveSnapshot::ALL, lanes).map(|_| ())
    }

    /// The matrix bank's counterpart of [`FaderBankProcessor::can_adopt_lane`].
    fn can_adopt_lane(&self, lane: usize, predecessor: &Self, predecessor_lane: usize) -> bool {
        predecessor.bank.width() == self.bank.width()
            && lane < self.bank.active_lanes()
            && predecessor_lane < predecessor.bank.active_lanes()
    }

    /// The matrix bank's counterpart of [`FaderBankProcessor::adopt_lane`].
    fn adopt_lane(&mut self, lane: usize, predecessor: &Self, predecessor_lane: usize) -> bool {
        let adopted = predecessor
            .bank
            .export_lane(predecessor_lane)
            .is_some_and(|state| self.bank.import_lane(lane, &state).is_ok());
        if adopted {
            self.quiet_at = None;
            adopt_carry_marks(
                self.controls.get_mut(lane),
                predecessor.controls.get(predecessor_lane),
                |reader| &mut reader.marks,
                |reader| &reader.marks,
            );
        }
        adopted
    }

    /// The matrix bank's counterpart of [`FaderBankProcessor::can_swap_with`], without stored
    /// automation.
    fn can_swap_with(&self, lane: usize, peer: Option<&Self>, peer_lane: usize) -> bool {
        self.bank
            .can_swap_lanes(lane, peer.map(|peer| &peer.bank), peer_lane)
            && lane_slots_fit(
                &self.controls,
                lane,
                peer.map(|peer| &*peer.controls),
                peer_lane,
            )
    }
    // REALTIME_POLICY_END
}

impl GraphPreparedBuiltinBankProcessor for MatrixBankProcessor {
    fn retained_bytes(&self) -> u64 {
        reader_table_bytes(&self.controls, MatrixReader::retained_bytes)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
    // REALTIME_POLICY_BEGIN
    fn carried_strip_lanes(&self) -> Option<(BuiltinStripStage, usize)> {
        Some((BuiltinStripStage::Matrix, self.bank.active_lanes()))
    }

    fn can_adopt_strip_lane(
        &self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        owner == BuiltinStripOwner::Matrix
            && predecessor
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|predecessor| self.can_adopt_lane(lane, predecessor, predecessor_lane))
    }

    /// Issue #1277 D8: the fader bank's counterpart, on the matrix cell.
    fn adopt_strip_lane(
        &mut self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &mut dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
        continued_cells: u8,
    ) -> bool {
        let _ = continued_cells;
        owner == BuiltinStripOwner::Matrix
            && predecessor
                .as_any_mut()
                .and_then(|any| any.downcast_mut::<Self>())
                .is_some_and(|predecessor| self.adopt_lane(lane, predecessor, predecessor_lane))
    }

    /// Issue #1277 D8: the fader bank's counterpart, on the matrix cell.
    fn drain_strip_for_carry(&mut self, owner: BuiltinStripOwner, lanes: u8) -> usize {
        usize::from(owner == BuiltinStripOwner::Matrix && self.apply_pending(lanes).is_err())
    }

    fn can_swap_builtin_lanes(
        &self,
        lane: usize,
        peer: Option<&dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        match peer {
            None => self.can_swap_with(lane, None, peer_lane),
            Some(peer) => peer
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|peer| self.can_swap_with(lane, Some(peer), peer_lane)),
        }
    }

    /// Issue #1765: the lane's matrix state, ramp in flight included
    /// ([`BuiltinMatrixBank::swap_lanes`]), and its cell reader. The block counters stay.
    fn swap_builtin_lanes(
        &mut self,
        lane: usize,
        peer: Option<&mut dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        let other = match peer {
            None => None,
            Some(peer) => match peer.as_any_mut().and_then(|any| any.downcast_mut::<Self>()) {
                Some(other) => Some(other),
                None => return false,
            },
        };
        if !self.can_swap_with(lane, other.as_deref(), peer_lane) {
            return false;
        }
        // One path for both peers, as the input bank's.
        self.quiet_at = None;
        let (bank, controls) = match other {
            None => (None, None),
            Some(other) => {
                other.quiet_at = None;
                (Some(&mut other.bank), Some(&mut *other.controls))
            }
        };
        let swapped = self.bank.swap_lanes(lane, bank, peer_lane).is_ok()
            && exchange_lane_slots(&mut self.controls, lane, controls, peer_lane);
        debug_assert!(swapped, "a checked matrix lane swaps");
        swapped
    }
    // REALTIME_POLICY_END
    /// The matrix cell's drain, before any lane of the block is dispatched (#1312 D12).
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.drain(snapshot)
    }
    fn live_lanes(&self) -> u8 {
        live_lane_mask(self.controls.iter().map(Option::is_some))
    }
    // REALTIME_POLICY_BEGIN
    /// The matrix bank's own rest mask ([`BuiltinMatrixBank::lanes_at_rest`], issue #1781). The
    /// drain applies every value readable under the block's snapshot first, so the bank's words
    /// are the block's; the bank has no clock, so the default no-op `advance_builtin_clocks` is
    /// exact.
    fn builtin_lanes_at_rest(&self) -> u32 {
        self.bank.lanes_at_rest()
    }
    // REALTIME_POLICY_END
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        let Self { bank, .. } = self;
        bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }

    /// Seam-side: see [`SEAM_SIDE_WITNESS`]. The 2x2 matrix **is** the seam -- it is the earliest
    /// genuinely cross-channel operation in the strip -- so it is the one stage that is
    /// structurally guaranteed to sit on the free side of it.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        SEAM_SIDE_WITNESS
    }

    /// Irreducibly cross-plane: `yl = ll*l + lr*r` reads both. See [`Self::lane_symmetry`].
    fn seam_side(&self) -> SeamSide {
        SeamSide::SeamSide
    }
}

/// Test-only source of stamps: a gate that stamps the next revision for a write and publishes it
/// when the write is done, as a transaction's commit does (#1312 D7).
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub trait TestOnlyWriteGate {
    /// Stamp the next revision, run `write` with the stamp, then publish that revision.
    fn write_published<R>(&mut self, write: impl FnOnce(LiveStamp<'_>) -> R) -> R;
}

#[cfg(any(test, feature = "test-support"))]
impl TestOnlyWriteGate for engine::realtime::PlanRevisionGate {
    fn write_published<R>(&mut self, write: impl FnOnce(LiveStamp<'_>) -> R) -> R {
        let revision = self.last_published() + 1;
        let stamp = self.stamp(revision).expect("a legal test revision");
        let result = write(stamp);
        self.publish(revision);
        result
    }
}

/// A gate with no plan, for tests that render a bound graph directly under
/// [`LiveSnapshot::ALL`]: every write stamps the next revision and publishes it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub struct TestOnlyGate {
    gate: engine::realtime::RevisionGate,
    pin: engine::realtime::GatePin,
}

#[cfg(any(test, feature = "test-support"))]
impl Default for TestOnlyGate {
    fn default() -> Self {
        let (gate, pin) = engine::realtime::RevisionGate::pair(0);
        Self { gate, pin }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl TestOnlyGate {
    /// A gate at revision 0.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The snapshot a block rendered now would take.
    #[must_use]
    pub fn snapshot(&self) -> LiveSnapshot {
        self.gate.snapshot()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl TestOnlyWriteGate for TestOnlyGate {
    fn write_published<R>(&mut self, write: impl FnOnce(LiveStamp<'_>) -> R) -> R {
        let revision = self.pin.last_published() + 1;
        let stamp = self.pin.stamp(revision).expect("a legal test revision");
        let result = write(stamp);
        self.pin.publish(&self.gate, revision);
        result
    }
}

// Render-local witnesses of the fader and matrix drains. These are deliberately thread-local:
// a host test can reset and read the counters around one real render without a process-global
// diagnostic surface or interference from another test thread.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static FADER_MATRIX_LIVE_WITNESS: std::cell::Cell<[u64; 2]> = const { std::cell::Cell::new([0; 2]) };
    /// Issue #1778: the lane walks this thread's input, fader and matrix banks skipped.
    static QUIET_WALK_SKIPS: std::cell::Cell<[u64; 3]> = const { std::cell::Cell::new([0; 3]) };
}

/// Test-only (#1778): take the lane walks this thread's input, fader and matrix banks skipped
/// since the last take, in that order.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_take_quiet_walk_skips() -> [u64; 3] {
    QUIET_WALK_SKIPS.with(|cell| cell.replace([0; 3]))
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestOnlyFaderMatrixWitness {
    pub fader_values_applied: u64,
    pub matrix_values_applied: u64,
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_fader_matrix_witness() {
    FADER_MATRIX_LIVE_WITNESS.with(|value| value.set([0; 2]));
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[doc(hidden)]
pub fn test_only_fader_matrix_witness() -> TestOnlyFaderMatrixWitness {
    let values = FADER_MATRIX_LIVE_WITNESS.with(std::cell::Cell::get);
    TestOnlyFaderMatrixWitness {
        fader_values_applied: values[0],
        matrix_values_applied: values[1],
    }
}

// REALTIME_POLICY_BEGIN
/// Drain the fader and mute cells of the lanes in `lanes` into `bank` under the block's `snapshot`
/// (#1312 D4, D12): every lane for the block drain, the carried lanes for the plan-swap drain.
/// Each lane reads and applies in the canonical order, an applied value is never lost to a
/// refusal of another, and the first refusal is returned after the last lane. `Ok(true)`: `lanes`
/// is every lane and none had a cell pending ([`quiet_mark`], issue #1778).
///
/// #1651 D1: a mute value of a channel whose mute the lane's stored automation drives is that
/// lane's `terms` ([`LaneFaderAutomation::mute_terms`]), not the stage's mute.
fn drain_fader_controls(
    bank: &mut BuiltinFaderBank,
    controls: &mut [Option<FaderReader>],
    automation: &mut [Option<LaneFaderAutomation>],
    snapshot: LiveSnapshot,
    lanes: u8,
) -> Result<bool, RenderError> {
    let mut result = Ok(());
    let mut quiet = lanes == EVERY_LANE;
    for (lane, control) in controls.iter_mut().enumerate() {
        if !lane_selected(lanes, lane) {
            continue;
        }
        let Some(control) = control.as_mut() else {
            continue;
        };
        let mut slot = automation.get_mut(lane).and_then(Option::as_mut);
        let drained = control.apply_latest(snapshot, |kind, lanes, words| {
            #[cfg(any(test, feature = "test-support"))]
            FADER_MATRIX_LIVE_WITNESS.with(|value| {
                let mut counters = value.get();
                counters[0] = counters[0].saturating_add(1);
                value.set(counters);
            });
            match kind {
                FaderKind::Fader => {
                    bank.set_fader_db(lane, lanes, f32::from_bits(words[0]), words[1])
                }
                FaderKind::Mute => match slot.as_deref_mut() {
                    Some(slot) => apply_mute_value(
                        slot,
                        &mut BankLaneStage {
                            bank: &mut *bank,
                            lane,
                        },
                        lanes,
                        words,
                        |stage, selector| {
                            stage.bank.set_mute(lane, selector, words[0] != 0, words[1])
                        },
                    ),
                    None => bank.set_mute(lane, lanes, words[0] != 0, words[1]),
                },
            }
        });
        quiet &= matches!(drained, Ok(true));
        result = result.and(drained.map(|_| ()).map_err(render_error));
    }
    result.map(|()| quiet)
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// One drained mute value `words` (`0`/`1`, then the ramp word) of the channels `lanes` covers on
/// a lane with stored automation (#1651 D1): a channel whose mute the lane automates takes it as
/// its `terms` ([`LaneFaderAutomation::mute_terms`]); every other channel's goes to the stage, as
/// on a lane with no automation (`set_mute(stage, selector)`). The first refusal is returned.
fn apply_mute_value<S: FaderJumpStage>(
    slot: &mut LaneFaderAutomation,
    stage: &mut S,
    lanes: BuiltinLaneSelector,
    words: [u32; FADER_CELL_WORDS],
    mut set_mute: impl FnMut(&mut S, BuiltinLaneSelector) -> Result<(), BuiltinParameterError>,
) -> Result<(), BuiltinParameterError> {
    if !(0..2).any(|channel| slot.automates_mute(channel)) {
        return set_mute(stage, lanes);
    }
    let mut result = Ok(());
    for channel in selected_channels(lanes) {
        if slot.automates_mute(channel) {
            slot.mute_terms(channel, words[0] != 0, words[1], stage);
        } else {
            result = result.and(set_mute(stage, channel_selector(channel)));
        }
    }
    result
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// The matrix counterpart of [`drain_fader_controls`].
fn drain_matrix_controls(
    bank: &mut BuiltinMatrixBank,
    controls: &mut [Option<MatrixReader>],
    snapshot: LiveSnapshot,
    lanes: u8,
) -> Result<bool, RenderError> {
    let mut result = Ok(());
    let mut quiet = lanes == EVERY_LANE;
    for (lane, control) in controls.iter_mut().enumerate() {
        if !lane_selected(lanes, lane) {
            continue;
        }
        let Some(control) = control.as_mut() else {
            continue;
        };
        let drained = control.apply_latest(snapshot, |target, smoothing_samples| {
            #[cfg(any(test, feature = "test-support"))]
            FADER_MATRIX_LIVE_WITNESS.with(|value| {
                let mut counters = value.get();
                counters[1] = counters[1].saturating_add(1);
                value.set(counters);
            });
            bank.set_target_smoothed(lane, target, smoothing_samples)
        });
        quiet &= matches!(drained, Ok(true));
        result = result.and(drained.map(|_| ()).map_err(render_error));
    }
    result.map(|()| quiet)
}
// REALTIME_POLICY_END

/// Retained bytes of one strip bank's per-lane live-control consumer array.
///
/// Charged whether or not live controls are attached: the array is `lanes` long either way, and a
/// lane with no channel holds `None`. That is deliberate -- it is what makes a banked session's
/// retained payload independent of whether the host leased live controls, rather than merely equal
/// to what it was before live controls existed.
fn strip_control_bytes<T>(width: effect_contract::BankWidth) -> Option<u64> {
    u64::try_from(core::mem::size_of::<Option<T>>())
        .ok()?
        .checked_mul(u64::from(width.lanes()))
}

/// The whole per-bank processor cost of one bankable stage: the struct plus the heap it owns.
fn strip_processor_bytes(stage: TrackStage, width: effect_contract::BankWidth) -> Option<u64> {
    let inline = match stage {
        TrackStage::PostInputBuiltins => core::mem::size_of::<BuiltinBankProcessor>(),
        TrackStage::PostFader => core::mem::size_of::<FaderBankProcessor>(),
        TrackStage::PostMatrix => core::mem::size_of::<MatrixBankProcessor>(),
        _ => return None,
    };
    let inline = u64::try_from(inline).ok()?;
    match stage {
        TrackStage::PostInputBuiltins => {
            inline.checked_add(strip_control_bytes::<InputReader>(width)?)
        }
        TrackStage::PostFader => inline.checked_add(strip_control_bytes::<FaderReader>(width)?),
        _ => inline.checked_add(strip_control_bytes::<MatrixReader>(width)?),
    }
}

/// Folds one kind's estimate into a running total over every builtin bank kind.
fn add_bank_resource(
    total: GraphBuiltinBankResourceEstimate,
    add: GraphBuiltinBankResourceEstimate,
) -> Option<GraphBuiltinBankResourceEstimate> {
    Some(GraphBuiltinBankResourceEstimate {
        bank_count: total.bank_count.checked_add(add.bank_count)?,
        payload_bytes: total.payload_bytes.checked_add(add.payload_bytes)?,
        scratch_bytes: total.scratch_bytes.checked_add(add.scratch_bytes)?,
        scratch_samples: total.scratch_samples.checked_add(add.scratch_samples)?,
        metadata_bytes: total.metadata_bytes.checked_add(add.metadata_bytes)?,
        maximum_mask_bytes: total.maximum_mask_bytes.max(add.maximum_mask_bytes),
        // A ceiling on the single largest allocation, so the maximum is taken and never summed.
        largest_allocation_bytes: total
            .largest_allocation_bytes
            .max(add.largest_allocation_bytes),
    })
}

/// The whole strip's bank plan: one group list per bankable stage, in render order.
///
/// The three stages are planned over the *same* track list with the *same* planner and the *same*
/// pool classes, so lane `i` of a cohort's fader bank is the same track as lane `i` of its
/// builtins bank wherever the dependency levels line up. Nothing relies on that: `runtime::chains_into` proves the lane-wise
/// relation on the lowered program and simply declines the merge where the orders disagree. What
/// planning them alike buys is that on a homogeneous session they *do* agree, and the strip fuses
/// into one chain per cohort.
/// The fader and the matrix are **seam-side** stages and their pool class is not about them: a
/// collapsed track duplicates its one plane *into* them, so their own per-channel words never gate
/// anything ([`SEAM_SIDE_WITNESS`]). They are pooled by the class anyway, and deliberately -- the
/// class here is the *track's*, not the stage's, so partitioning all three stages alike is exactly
/// what keeps a cohort's builtins, fader and matrix banks covering the same lanes in the same
/// order, which is the condition `runtime::chains_into` needs to fuse them into one chain. Pooling
/// only the upstream stage would have split the strip's chain on every mixed session.
fn planned_strip_banks(
    tracks: &[Box<str>],
    dispatch: Backend,
    levels: &[DependencyLevel],
    classes: &SessionPoolClasses,
) -> [(TrackStage, Vec<Box<[GraphNodeId]>>); 3] {
    [
        TrackStage::PostInputBuiltins,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
    .map(|stage| {
        (
            stage,
            planned_builtin_bank_members(tracks, stage, dispatch, levels, classes),
        )
    })
}

/// The cohort key of the fixed post-input builtin stage.
///
/// It carries no fields on purpose: the stage is not selectable, its backend and width are fixed
/// for the whole artifact, and its rate and quantum come from the session envelope. Every
/// post-input node is therefore co-bankable with every other one at its dependency level, which
/// is exactly the cohort the planner forms. If a per-track variant is ever added (a quality, a
/// second section order), it becomes a field here and the planner splits the cohorts for free.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BuiltinStageKey;

impl BankSlotKey for BuiltinStageKey {}

/// Groups every node of one bankable track stage, per dependency level, into `ceil(n / W)` banks.
///
/// The last bank of a level is short: it holds `1..=W` members and the bank pads the remaining
/// lanes with identity lanes.  Every post-input node on a vector host is therefore a bank member,
/// and scalar post-input bindings survive only when the backend has no bank width at all.
///
/// The grouping itself is **not implemented here**: it delegates to
/// [`rack_compiler::plan_bank_groups`], the workspace's single cohort planner (#96 F1).
/// Padding, level partitioning and the trailing-`None` lane order are that planner's rules, so
/// this function only turns each planned group back into the member list the graph attaches.
fn planned_builtin_bank_members(
    tracks: &[Box<str>],
    stage: TrackStage,
    dispatch: Backend,
    levels: &[DependencyLevel],
    classes: &SessionPoolClasses,
) -> Vec<Box<[GraphNodeId]>> {
    let Some(width) = BankWidth::for_backend(dispatch) else {
        return Vec::new();
    };
    let level_by_node: BTreeMap<_, _> = levels
        .iter()
        .flat_map(|level| level.nodes.iter().map(move |node| (node, level.level)))
        .collect();
    let mut by_level = BTreeMap::<u64, Vec<CohortCandidate<GraphNodeId, BuiltinStageKey>>>::new();
    for track in tracks {
        let node = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(track).expect("prepared stable ID"),
            stage,
        };
        if let Some(level) = level_by_node.get(&node).copied() {
            by_level.entry(level).or_default().push(CohortCandidate {
                id: node,
                program: RackProgram::new(RackLocation::Simd1, vec![BuiltinStageKey]),
                class: classes.class_of(track),
            });
        }
    }
    let levels_in: Vec<_> = by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let plan = plan_bank_groups(&levels_in, width)
        .expect("one node per track per stage, so ids are unique");
    // Every strip node is bankable, so the planner's scalar list is empty; if a future stage key
    // ever blocks banking, those tracks simply keep their scalar bindings.
    debug_assert!(plan.scalar.is_empty());
    plan.groups
        .into_iter()
        .map(|group| group.members.into_vec().into_iter().flatten().collect())
        .collect()
}

/// Builds one padded bank from `inputs.len()` tracks in member order.
///
/// Lanes `inputs.len()..width.lanes()` become the bank's identity lanes: the builtins crate owns
/// that contract (`BuiltinInputBank::new` accepts `1..=W` inputs and pads the rest), and this
/// is the single call site, so the compiler never builds a mask of its own.
fn build_input_bank(
    dispatch: Backend,
    width: effect_contract::BankWidth,
    inputs: Vec<InputBuiltins>,
) -> BuiltinInputBank {
    BuiltinInputBank::new(dispatch, width, inputs)
        .expect("planner emits 1..=W members at the width the selected backend chose")
}

/// Exact retained storage of one *kind* of builtin bank.
///
/// `processor_bytes` is the whole per-bank processor cost of that kind: the inline struct plus any
/// heap it owns. The three strip kinds differ only there -- an input bank owns nothing beyond its
/// kernel, while a fader or matrix bank owns a `lanes`-long array of optional live-control
/// consumers -- so the rest of the accounting is shared rather than restated three times.
///
/// The sample charge is the bank chain's whole `f32` storage: two AoSoA scratch planes, the two
/// lane-major staging planes every `rack::BankChain` owns for its tiled scatter, and, for a
/// padded bank, the chain's `quantum`-word `+0.0` plane its tiled gather reads for an inactive
/// lane (issue #887).
fn builtin_bank_resource(
    groups: &[Box<[GraphNodeId]>],
    width: effect_contract::BankWidth,
    quantum: u32,
    processor_bytes: u64,
) -> Option<GraphBuiltinBankResourceEstimate> {
    let bank_count = u64::try_from(groups.len()).ok()?;
    let lanes = u64::from(width.lanes());
    if groups
        .iter()
        .any(|members| members.is_empty() || members.len() as u64 > lanes)
    {
        return None;
    }
    let node_bytes = u64::try_from(core::mem::size_of::<GraphNodeId>()).ok()?;
    // Two planes: `AoSoaScratch` has no sidechain surface at all (#96 F9 deleted it).
    let scratch_plane_samples = u64::from(quantum).checked_mul(lanes)?;
    let scratch_plane_bytes = scratch_plane_samples.checked_mul(4)?;
    // Scratch and staging: four planes. The zero plane is charged per padded bank below.
    let scratch_samples_per_bank = scratch_plane_samples.checked_mul(4)?;
    let mut padded_banks = 0_u64;
    let mut member_string_bytes = 0_u64;
    let mut largest_member_string = 0_u64;
    let mut payload_bytes = 0_u64;
    let mut largest_member_array = 0_u64;
    for members in groups {
        // A padded bank owns exactly the ids it holds, not a full-width array.
        let member_array_bytes = node_bytes.checked_mul(u64::try_from(members.len()).ok()?)?;
        largest_member_array = largest_member_array.max(member_array_bytes);
        padded_banks += u64::from((members.len() as u64) < lanes);
        payload_bytes = payload_bytes
            .checked_add(member_array_bytes)?
            .checked_add(processor_bytes)?;
        for member in members.iter() {
            let GraphNodeId::TrackStage { track_id, .. } = member else {
                return None;
            };
            let bytes = u64::try_from(track_id.as_str().len()).ok()?;
            member_string_bytes = member_string_bytes.checked_add(bytes)?;
            largest_member_string = largest_member_string.max(bytes);
        }
    }
    let payload_bytes = payload_bytes.checked_add(member_string_bytes)?;
    let scratch_samples = scratch_samples_per_bank
        .checked_mul(bank_count)?
        .checked_add(u64::from(quantum).checked_mul(padded_banks)?)?;
    let scratch_bytes = scratch_samples.checked_mul(4)?;
    let metadata_bytes = u64::try_from(core::mem::size_of::<GraphPreparedBuiltinBank>())
        .ok()?
        .checked_mul(bank_count)?;
    Some(GraphBuiltinBankResourceEstimate {
        bank_count,
        payload_bytes,
        scratch_bytes,
        scratch_samples,
        metadata_bytes,
        maximum_mask_bytes: if bank_count == 0 {
            0
        } else {
            u64::from(width.lanes())
                .checked_mul(u64::try_from(core::mem::size_of::<bool>()).ok()?)?
        },
        largest_allocation_bytes: largest_member_array
            .max(processor_bytes)
            .max(largest_member_string)
            .max(scratch_plane_bytes)
            .max(metadata_bytes),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BuiltinSessionSeal {
    session_sha256: [u8; 32],
    sample_rate: u32,
    quantum: u32,
    tracks: Vec<Box<str>>,
    processors: Vec<(Box<str>, TrackStage)>,
    tails: Vec<(Box<str>, NodeTailBound)>,
    /// The live input bound, computed once per preparation when any strip's input lane is live
    /// (#1329 Amendment 4, R7); the seal check reuses it.
    live_bound: Option<NodeTailBound>,
    requests: Vec<MeterRequestSeal>,
    observers: Vec<(Box<str>, TrackStage, u64)>,
    consumers: Vec<(u64, Box<str>, MeterTap)>,
    /// Issue #137 D1: `(track_id, input_lane)` per live-control channel, sorted (#1254 D1 added
    /// the input-lane flag; the cells have no depth to seal, #1312, #1346).
    controls: Vec<(Box<str>, bool)>,
    resources: BuiltinResourceEstimate,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MeterRequestSeal {
    handle: u64,
    track_id: Box<str>,
    tap: MeterTap,
    reset_generation: u64,
    period_frames: u32,
    peak_hold_frames: u32,
    peak_decay_bits: u32,
    queue_capacity: usize,
    metrics: MeterMetricSet,
}

type ObserverSeal = (Box<str>, TrackStage, u64);
type ConsumerSeal = (u64, Box<str>, MeterTap);

/// Test-only phase-two allocation accounting.  The production resource report deliberately
/// remains a layout calculation; this probe independently observes the allocator requests made
/// after phase-one validation has accepted the artifact.
#[cfg(any(test, feature = "test-support"))]
static TEST_PHASE_TWO_ACTIVE: AtomicBool = AtomicBool::new(false);
#[cfg(any(test, feature = "test-support"))]
static TEST_PHASE_TWO_LAYOUTS: Mutex<TestPhaseTwoLayoutTable> =
    Mutex::new(TestPhaseTwoLayoutTable::new());

#[cfg(any(test, feature = "test-support"))]
struct TestPhaseTwoLayoutTable {
    values: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
    len: usize,
    overflowed: bool,
    deallocation_count: u64,
    deallocations: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
    deallocation_len: usize,
}

#[cfg(any(test, feature = "test-support"))]
impl TestPhaseTwoLayoutTable {
    const fn new() -> Self {
        Self {
            values: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
            len: 0,
            overflowed: false,
            deallocation_count: 0,
            deallocations: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
            deallocation_len: 0,
        }
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn record(&mut self, layout: core::alloc::Layout) {
        let Ok(size_bytes) = u64::try_from(layout.size()) else {
            self.overflowed = true;
            return;
        };
        let Ok(align_bytes) = u64::try_from(layout.align()) else {
            self.overflowed = true;
            return;
        };
        if let Some(value) = self.values[..self.len]
            .iter_mut()
            .find(|value| value.size_bytes == size_bytes && value.align_bytes == align_bytes)
        {
            let Some(count) = value.allocation_count.checked_add(1) else {
                self.overflowed = true;
                return;
            };
            value.allocation_count = count;
            return;
        }
        let Some(slot) = self.values.get_mut(self.len) else {
            self.overflowed = true;
            return;
        };
        *slot = BuiltinRetainedLayout {
            size_bytes,
            align_bytes,
            allocation_count: 1,
        };
        self.len += 1;
    }

    fn record_deallocation(&mut self, layout: core::alloc::Layout) {
        match self.deallocation_count.checked_add(1) {
            Some(count) => self.deallocation_count = count,
            None => self.overflowed = true,
        }
        let size_bytes = layout.size() as u64;
        let align_bytes = layout.align() as u64;
        if let Some(value) = self.deallocations[..self.deallocation_len]
            .iter_mut()
            .find(|value| value.size_bytes == size_bytes && value.align_bytes == align_bytes)
        {
            value.allocation_count = value.allocation_count.saturating_add(1);
        } else if let Some(slot) = self.deallocations.get_mut(self.deallocation_len) {
            *slot = BuiltinRetainedLayout {
                size_bytes,
                align_bytes,
                allocation_count: 1,
            };
            self.deallocation_len += 1;
        } else {
            self.overflowed = true;
        }
    }
}

/// Independent test-only phase-two allocation observation.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestPhaseTwoAllocationSnapshot {
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
    pub allocation_count: u64,
    pub layouts: Vec<BuiltinRetainedLayout>,
    pub overflowed: bool,
    pub deallocation_count: u64,
    pub deallocation_layouts: Vec<BuiltinRetainedLayout>,
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_phase_two_allocation_tracker() {
    TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.clear();
    }
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_record_phase_two_allocation(layout: core::alloc::Layout) {
    if !TEST_PHASE_TWO_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.record(layout);
    }
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_record_phase_two_deallocation(layout: core::alloc::Layout) {
    if !TEST_PHASE_TWO_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.record_deallocation(layout);
    }
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_phase_two_allocation_snapshot() -> TestPhaseTwoAllocationSnapshot {
    let table = TEST_PHASE_TWO_LAYOUTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut layouts = table.values[..table.len].to_vec();
    layouts.sort();
    let mut total_bytes = 0_u64;
    let mut allocation_count = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    let mut overflowed = table.overflowed;
    for layout in &layouts {
        let Some(bytes) = layout.size_bytes.checked_mul(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        let Some(total) = total_bytes.checked_add(bytes) else {
            overflowed = true;
            continue;
        };
        total_bytes = total;
        let Some(count) = allocation_count.checked_add(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        allocation_count = count;
        largest_allocation_bytes = largest_allocation_bytes.max(layout.size_bytes);
    }
    TestPhaseTwoAllocationSnapshot {
        total_bytes,
        largest_allocation_bytes,
        allocation_count,
        layouts,
        overflowed,
        deallocation_count: table.deallocation_count,
        deallocation_layouts: table.deallocations[..table.deallocation_len].to_vec(),
    }
}

#[cfg(any(test, feature = "test-support"))]
struct TestPhaseTwoAllocationGuard;
#[cfg(any(test, feature = "test-support"))]
impl TestPhaseTwoAllocationGuard {
    fn begin() -> Self {
        TEST_PHASE_TWO_ACTIVE.store(true, Ordering::SeqCst);
        Self
    }
}
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_begin_phase_two_allocation_observation() -> impl Drop {
    TestPhaseTwoAllocationGuard::begin()
}

#[cfg(any(test, feature = "test-support"))]
impl Drop for TestPhaseTwoAllocationGuard {
    fn drop(&mut self) {
        TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    }
}

/// A graph plus genuine compiler-owned builtin bindings with no public parts-extraction seam.
///
/// `R` is caller-owned immutable graph-report metadata. All provenance-bearing fields stay
/// private to this crate, including the unbound graph and the concrete processor/observer parts.
pub struct PreparedBuiltinsGraphArtifact<R> {
    graph: PreparedGraphPlan,
    builtin_processors: Vec<graph::GraphNodeBinding>,
    builtin_observers: Vec<GraphNodeObserverBinding>,
    report: R,
    /// Issue #137 D1: control producers travel with the artifact so the one-way binding hands
    /// them to the caller together with the plan that owns their consumers.
    track_controls: Vec<TrackControlProducer>,
    meter_consumers: Vec<MeterConsumer>,
    /// The session's one fader and mute automation program, `None` without one. Bind hands it
    /// to the plan's graph executor, which charges it once in the plan's retained bytes (#1074).
    automation_program: Option<Arc<automation::Program>>,
}

/// The one-way result of consuming and binding a sealed builtin graph artifact.
///
/// Field order is drop order: `track_controls` producers are released before the `plan` that owns
/// their consumer endpoints, and `meter_consumers` outlive the plan that owns their producers.
pub struct PreparedBuiltinsGraphBound {
    pub track_controls: Vec<TrackControlProducer>,
    pub plan: PreparedRenderPlan,
    pub meter_consumers: Vec<MeterConsumer>,
    /// The plan's gate handle, taken by the first [`Self::test_only_write`].
    #[cfg(any(test, feature = "test-support"))]
    test_gate: Option<engine::realtime::PlanRevisionGate>,
}

#[cfg(any(test, feature = "test-support"))]
impl PreparedBuiltinsGraphBound {
    /// Test-only: write the strips' cells as one transaction, stamped from the plan's own gate,
    /// then publish its revision on that gate, as a commit does (#1312 D7). The plan's next block
    /// takes a snapshot that covers it.
    #[doc(hidden)]
    pub fn test_only_write<R>(
        &mut self,
        write: impl FnOnce(&mut [TrackControlProducer], LiveStamp<'_>) -> R,
    ) -> R {
        let gate = self.test_gate.get_or_insert_with(|| {
            self.plan
                .take_revision_gate()
                .expect("the bound plan's gate handle is untaken")
        });
        let controls = &mut self.track_controls;
        gate.write_published(|stamp| write(controls, stamp))
    }
}

/// A rejected external binding preserves the opaque artifact and caller-owned bindings.
pub struct PreparedBuiltinsGraphBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub code: &'static str,
}

/// A rejected source-set binding preserves the opaque artifact and every caller-owned input.
pub struct PreparedBuiltinsGraphSourceBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub source_set: GraphPreparedSourceSet,
    pub code: &'static str,
}

/// Deliberate seal corruption available only to the graph compiler's adversarial tests.
///
/// This is not a wire format and is compiled out of production artifacts.  Keeping each seal
/// field independently reachable lets the graph boundary prove that it rejects the exact
/// corrupted tuple before it consumes either prepared input.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruption {
    /// The prepared session identity does not match the graph session.
    SessionIdentity,
    /// The sealed track set does not match the prepared bindings.
    Tracks,
    /// The sealed processor set does not match the prepared bindings.
    Processors,
    /// The retained tail records do not match their seal.
    Tails,
    /// The sealed meter request records do not match their seal.
    Requests,
    /// The sealed observer records do not match their bindings.
    Observers,
    /// The sealed consumer records do not match their queues.
    Consumers,
    /// The sealed resource report does not match the retained report.
    Resources,
}

/// Frozen corruption subcases within the eight seal categories.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruptionCase {
    SessionHash,
    SessionRate,
    SessionQuantum,
    TrackMissing,
    TrackExtra,
    TrackDuplicate,
    ProcessorMissing,
    ProcessorExtra,
    ProcessorChangedStage,
    TailMissing,
    TailExtra,
    TailChanged,
    RequestMissing,
    RequestExtra,
    RequestDuplicate,
    ObserverMissing,
    ObserverExtra,
    ObserverChangedNode,
    ConsumerMissing,
    ConsumerExtra,
    ConsumerChangedMetadata,
    ConsumerDuplicateHandle,
    ResourceReport,
}

#[cfg(feature = "test-support")]
impl PreparedBuiltinsCorruptionCase {
    #[must_use]
    pub const fn category(self) -> PreparedBuiltinsCorruption {
        match self {
            Self::SessionHash | Self::SessionRate | Self::SessionQuantum => {
                PreparedBuiltinsCorruption::SessionIdentity
            }
            Self::TrackMissing | Self::TrackExtra | Self::TrackDuplicate => {
                PreparedBuiltinsCorruption::Tracks
            }
            Self::ProcessorMissing | Self::ProcessorExtra | Self::ProcessorChangedStage => {
                PreparedBuiltinsCorruption::Processors
            }
            Self::TailMissing | Self::TailExtra | Self::TailChanged => {
                PreparedBuiltinsCorruption::Tails
            }
            Self::RequestMissing | Self::RequestExtra | Self::RequestDuplicate => {
                PreparedBuiltinsCorruption::Requests
            }
            Self::ObserverMissing | Self::ObserverExtra | Self::ObserverChangedNode => {
                PreparedBuiltinsCorruption::Observers
            }
            Self::ConsumerMissing
            | Self::ConsumerExtra
            | Self::ConsumerChangedMetadata
            | Self::ConsumerDuplicateHandle => PreparedBuiltinsCorruption::Consumers,
            Self::ResourceReport => PreparedBuiltinsCorruption::Resources,
        }
    }
}

/// The stable-ID grammar admits at most 127 distinct string allocation sizes. The remaining
/// entries cover every fixed processor, vector, endpoint and queue layout class without imposing
/// any limit on track, meter or allocation counts.
pub const BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY: usize = 160;

/// One exact `(size, alignment)` class in the retained allocation multiset.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinRetainedLayout {
    pub size_bytes: u64,
    pub align_bytes: u64,
    pub allocation_count: u64,
}

impl BuiltinRetainedLayout {
    const ZERO: Self = Self {
        size_bytes: 0,
        align_bytes: 0,
        allocation_count: 0,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinResourceEstimate {
    /// Exact engine-owned processor, seal and binding payload bytes retained by this artifact.
    pub engine_owned_processor_payload_bytes: u64,
    /// Exact engine-owned meter and queue payload bytes retained by this artifact.
    pub engine_owned_meter_payload_bytes: u64,
    /// Exact total of all engine-owned retained payload bytes in this artifact.
    pub engine_owned_retained_payload_bytes: u64,
    pub meter_items: u64,
    /// Largest requested engine-owned payload allocation retained by this artifact.
    pub maximum_single_allocation_bytes: u64,
    /// Count of retained engine-owned payload allocations represented by this report.
    pub retained_allocation_count: u64,
    /// Number of populated entries in [`Self::retained_layouts`].
    pub retained_layout_class_count: u16,
    /// Exact ordered multiset classes for all retained allocation requests.
    pub retained_layouts: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for BuiltinResourceEstimate {
    fn default() -> Self {
        Self {
            engine_owned_processor_payload_bytes: 0,
            engine_owned_meter_payload_bytes: 0,
            engine_owned_retained_payload_bytes: 0,
            meter_items: 0,
            maximum_single_allocation_bytes: 0,
            retained_allocation_count: 0,
            retained_layout_class_count: 0,
            retained_layouts: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl BuiltinResourceEstimate {
    /// Populated exact retained layout classes in deterministic `(size, align)` order.
    #[must_use]
    pub fn retained_layouts(&self) -> &[BuiltinRetainedLayout] {
        &self.retained_layouts[..usize::from(self.retained_layout_class_count)]
    }
}

/// Versioned public name for the exact engine-owned retained-payload report.
///
/// This is deliberately an alias rather than a duplicate accounting type: a caller cannot
/// accidentally read one report while the compiler validates another.
pub type BuiltinResourceReport = BuiltinResourceEstimate;

#[derive(Clone, Copy, Debug)]
struct ResourceAccumulator {
    total: u64,
    largest: u64,
    allocations: u64,
    layout_class_count: u16,
    layouts: [BuiltinRetainedLayout; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for ResourceAccumulator {
    fn default() -> Self {
        Self {
            total: 0,
            largest: 0,
            allocations: 0,
            layout_class_count: 0,
            layouts: [BuiltinRetainedLayout::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl ResourceAccumulator {
    fn add_layout(&mut self, layout: core::alloc::Layout) -> Option<()> {
        let size_bytes = u64::try_from(layout.size()).ok()?;
        let align_bytes = u64::try_from(layout.align()).ok()?;
        self.add_layout_count(size_bytes, align_bytes, 1)?;
        Some(())
    }

    fn add_layout_count(
        &mut self,
        size_bytes: u64,
        align_bytes: u64,
        allocation_count: u64,
    ) -> Option<()> {
        let retained_bytes = size_bytes.checked_mul(allocation_count)?;
        self.total = self.total.checked_add(retained_bytes)?;
        self.largest = self.largest.max(size_bytes);
        self.allocations = self.allocations.checked_add(allocation_count)?;
        let populated = usize::from(self.layout_class_count);
        if let Some(layout) = self.layouts[..populated]
            .iter_mut()
            .find(|layout| layout.size_bytes == size_bytes && layout.align_bytes == align_bytes)
        {
            layout.allocation_count = layout.allocation_count.checked_add(allocation_count)?;
            return Some(());
        }
        let slot = self.layouts.get_mut(populated)?;
        *slot = BuiltinRetainedLayout {
            size_bytes,
            align_bytes,
            allocation_count,
        };
        self.layout_class_count = self.layout_class_count.checked_add(1)?;
        Some(())
    }

    fn merge(&mut self, other: Self) -> Option<()> {
        for layout in &other.layouts[..usize::from(other.layout_class_count)] {
            self.add_layout_count(
                layout.size_bytes,
                layout.align_bytes,
                layout.allocation_count,
            )?;
        }
        Some(())
    }

    fn sorted_layouts(mut self) -> Self {
        self.layouts[..usize::from(self.layout_class_count)].sort();
        self
    }

    fn add_bytes(&mut self, bytes: usize) -> Option<()> {
        self.add_layout(core::alloc::Layout::from_size_align(bytes, 1).ok()?)
    }
}

#[derive(Clone, Copy, Debug)]
struct BuiltinResourcePlan {
    report: BuiltinResourceEstimate,
}

impl PreparedBuiltinsSession {
    /// Read-only retained-payload resource report.
    #[must_use]
    pub const fn resource_report(&self) -> BuiltinResourceEstimate {
        self.resources
    }

    /// Number of sealed builtin processor bindings.
    ///
    /// Three per track, as it has always been -- but all three are now held as a
    /// `StripPreparation` until their binding form is chosen (issue #212 for the fader and the
    /// matrix, #210 phase 3 for the input) and are counted here in the shape they will take. The
    /// `processors` term is what a lowering has already built, and is zero for a session
    /// preparation has only just returned.
    #[must_use]
    pub fn processor_count(&self) -> usize {
        self.processors.len() + self.strips.len() * 3
    }

    /// Number of sealed builtin tails.
    #[must_use]
    pub fn tail_count(&self) -> usize {
        self.tails.len()
    }

    /// Number of sealed meter observer bindings.
    #[must_use]
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Number of sealed meter consumer endpoints.
    #[must_use]
    pub fn meter_consumer_count(&self) -> usize {
        self.meter_consumers.len()
    }

    /// Number of sealed live-control channels (issue #137 D1).
    #[must_use]
    pub fn track_control_count(&self) -> usize {
        self.track_controls.len()
    }

    /// The heap the sealed live-control producers own beyond their table: each one's
    /// [`TrackControlProducer::retained_heap_bytes`], summed (issue #2199), as the control plane
    /// charges its strip producer table from the producers it keeps.
    #[must_use]
    pub fn track_control_heap_bytes(&self) -> u64 {
        self.track_controls
            .iter()
            .map(TrackControlProducer::retained_heap_bytes)
            .fold(0, u64::saturating_add)
    }

    /// Strip `track_id`'s stored mute automation (#1651 D1), for the route lanes of #1652 that
    /// follow it: the plan's one automation program (the same `Arc` every fader owner of the plan
    /// reads) and the program's mute cell index of each lane, `[left, right]`, `None` for a lane
    /// no mute entry drives. `None` when no lane of the strip automates its mute (or the session
    /// has no stored automation that renders). Control side; it clones one `Arc`.
    #[must_use]
    pub fn strip_mute_cells(
        &self,
        track_id: &str,
    ) -> Option<(Arc<automation::Program>, [Option<u32>; 2])> {
        let plan = self.fader_automation.as_ref()?;
        let mute = &plan.strip(track_id)?.mute;
        let cells = [0, 1].map(|lane| mute[lane].as_ref().and_then(|cell| cell.cell));
        cells
            .iter()
            .any(Option::is_some)
            .then(|| (Arc::clone(&plan.program), cells))
    }

    /// VCA `vca_id`'s stored automation (#1655 D1), for the members' records and the following
    /// routes of the host (V3, V4): the plan's one automation program (the `Arc`
    /// [`Self::strip_mute_cells`] hands out) and the program's cell index of each `[row][lane]`
    /// (row 0 `offset_db`, row 1 `mute`; lane 0 left, 1 right), `None` for a lane no entry drives.
    /// `None` when the VCA automates neither row (or the session has no stored automation that
    /// renders). Control side; it clones one `Arc`.
    #[must_use]
    pub fn vca_cells(&self, vca_id: &str) -> Option<(Arc<automation::Program>, VcaCells)> {
        let plan = self.fader_automation.as_ref()?;
        let cells = *plan.vcas.get(vca_id)?;
        Some((Arc::clone(&plan.program), cells))
    }

    /// The session's one automation program, cloned for the bound plan (#1074). Control side.
    fn automation_program(&self) -> Option<Arc<automation::Program>> {
        self.fader_automation
            .as_ref()
            .map(|plan| Arc::clone(&plan.program))
    }

    /// Every route's effective follow gate, `[left, right]` per source lane, in `model.routes`
    /// order, as it was prepared with (decision 17, C7): `follows_mute` on a muted lane of the
    /// route's source strip, composed once by `controls::effective`. Control-side data like
    /// [`Self::tails`]: graph lowering reads it for each route's gate and composes nothing.
    #[must_use]
    pub fn effective_route_follow(&self) -> &[[bool; 2]] {
        &self.route_follow
    }

    /// Read-only builtin tails used by graph lowering: each strip's `T_decay` (#1329 G1).
    pub fn tails(&self) -> impl Iterator<Item = (&str, TailSamples)> {
        self.tails
            .iter()
            .map(|(track, bound)| (track.as_ref(), bound.tail))
    }

    /// Every strip's input-section bounds as reported, by track ID (#1329 D7, Amendment 4 R5):
    /// `T_decay`, the tail over every peak `T_rest`, and D2's exact-rest bound, which silence
    /// skipping reads (#1107). Computed once per preparation on the control thread.
    pub fn input_bounds(&self) -> impl Iterator<Item = (&str, NodeTailBound)> {
        self.tails
            .iter()
            .map(|(track, bound)| (track.as_ref(), *bound))
    }

    /// Validate the immutable payload against the exact effect-prepared session.
    pub fn validate_for_session(&self, session: &CompiledSession) -> BuiltinDiagnosticSet {
        let mut diagnostics = Vec::new();
        if self.seal.session_sha256 != session_identity(session)
            || self.seal.sample_rate != session.sample_rate().0
            || self.seal.quantum != session.quantum().0
        {
            diagnostics.push(diag("builtin.session.mismatch", "$.session"));
        }
        let expected_tracks: Vec<&str> = session
            .normalized_model()
            .strips()
            .map(|strip| strip.id.as_str())
            .collect();
        if !self
            .seal
            .tracks
            .iter()
            .map(Box::as_ref)
            .eq(expected_tracks.iter().copied())
        {
            diagnostics.push(diag("builtin.prepared.track_set", "$.builtins.tracks"));
        }
        let expected_processors = processor_seal(&expected_tracks);
        if !self
            .seal
            .processors
            .iter()
            .map(|(track, stage)| (track.as_ref(), *stage))
            .eq(expected_processors.iter().copied())
            || !processors_match(&self.processors, &self.strips, &expected_processors)
        {
            diagnostics.push(diag(
                "builtin.prepared.processor_set",
                "$.builtins.processors",
            ));
        }
        let mut actual_controls: Vec<(&str, bool)> = self
            .track_controls
            .iter()
            .map(|control| (control.track_id.as_ref(), control.input.is_some()))
            .collect();
        actual_controls.sort_unstable();
        let expected_tails = match expected_tails(
            session,
            &actual_controls,
            &self.seal.tails,
            self.seal.live_bound,
        ) {
            Ok(value) => value,
            Err(()) => {
                diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
                Vec::new()
            }
        };
        if self.seal.tails != expected_tails || self.tails != expected_tails {
            diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
        }
        let (actual_observers, actual_consumers) =
            actual_meter_seals(&self.observers, &self.meter_consumers);
        if self.seal.requests != self.requests {
            diagnostics.push(diag(
                "builtin.prepared.request_set",
                "$.builtins.meter_requests",
            ));
        }
        if self.seal.observers != actual_observers {
            diagnostics.push(diag(
                "builtin.prepared.observer_set",
                "$.builtins.observers",
            ));
        }
        if self.seal.consumers != actual_consumers {
            diagnostics.push(diag(
                "builtin.prepared.consumer_set",
                "$.builtins.meter_consumers",
            ));
        }
        // Issue #137 D1: the control seal is the producer set, so a lost, duplicated, or
        // retargeted control channel is a prepared-set mismatch exactly like a lost meter.
        if !self
            .seal
            .controls
            .iter()
            .map(|(track, input_lane)| (track.as_ref(), *input_lane))
            .eq(actual_controls)
        {
            diagnostics.push(diag(
                "builtin.prepared.control_set",
                "$.builtins.track_controls",
            ));
        }
        if self.seal.resources != self.resources {
            diagnostics.push(diag(
                "builtin.prepared.resource_report",
                "$.builtins.resources",
            ));
        }
        BuiltinDiagnosticSet::sorted(diagnostics)
    }

    /// Seal an already session-validated graph around these genuine compiler-owned bindings.
    ///
    /// This is deliberately a one-way conversion: callers may carry and bind the resulting
    /// artifact, but cannot extract, replace, or clone its provenance-bearing parts.
    ///
    /// Every track is bound per node here, so this is the scalar oracle's lowering and exists only
    /// for tests and `test-support` (issue #1059); [`Self::into_graph_artifact_with_banks`] is the
    /// lowering every shipped build takes.
    #[cfg(any(test, feature = "test-support"))]
    pub fn into_graph_artifact<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
    ) -> PreparedBuiltinsGraphArtifact<R> {
        // #1646 D2: the scalar oracle rebuilds the ramped fader from the prepared parameters here.
        // Defensive for direct oracle callers; `into_graph_artifact_with_banks` already holds it.
        let _canonical = lane::CanonicalFpEnv::enter();
        let automation_program = self.automation_program();
        let mut processors = core::mem::take(&mut self.processors);
        processors.append(&mut self.strip_bindings());
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: processors,
            builtin_observers: self.observers,
            report,
            track_controls: self.track_controls,
            meter_consumers: self.meter_consumers,
            automation_program,
        }
    }

    /// The per-node input, fader and matrix bindings of every track still holding a
    /// [`StripPreparation`].
    ///
    /// A track a strip bank claimed had its parts moved out first, so it is simply not here. This
    /// is the *only* place the per-node shapes are built, which is what keeps "a track is a bank
    /// lane or a per-node processor, never both" a structural fact rather than a rule.
    ///
    /// The input stage joined this list in #210 phase 3, for the reason the stage comment on
    /// [`StripPreparation::input`] gives: once the input section has a live-control channel, which
    /// owner drains it is a lowering decision, and a binding built at preparation could not hand
    /// its consumer to a bank.
    ///
    /// The scalar oracle's half of lowering (issue #1059): a vector dispatch leaves no strip
    /// behind, so shipped builds bind through [`Self::unbanked_strip_bindings`] instead.
    #[cfg(any(test, feature = "test-support"))]
    fn strip_bindings(&mut self) -> Vec<graph::GraphNodeBinding> {
        let mut bindings = Vec::with_capacity(self.strips.len() * 3);
        let fader_automation = self.fader_automation.take();
        for strip in core::mem::take(&mut self.strips) {
            // #1646 R17-4: the oracle installs the same lane automation as a fader bank lane.
            let automation = fader_automation.as_ref().and_then(|plan| {
                plan.strip(&strip.track_id)
                    .map(|cells| Box::new(bound_lane_automation(&plan.program, cells)))
            });
            let StripPreparation {
                graph_id,
                parameters,
                input,
                fader,
                matrix,
                control,
                ..
            } = strip;
            let (input_processor, fader_processor, matrix_processor): (
                Box<dyn GraphRuntimeProcessor>,
                Box<dyn GraphRuntimeProcessor>,
                Box<dyn GraphRuntimeProcessor>,
            ) = match control {
                None => (
                    Box::new(InputProcessor(input)),
                    match automation {
                        // #1647b (R17-4): an automated strip binds the ramped fader, which takes
                        // the events' timed operations.
                        Some(automation) => Box::new(AutomatedFaderProcessor {
                            fader: FaderMuteRampBuiltins::new(parameters)
                                .expect("preparation validated the ramped fader's gain domain"),
                            automation,
                        }),
                        None => Box::new(FaderProcessor(fader)),
                    },
                    Box::new(MatrixProcessor(matrix)),
                ),
                Some(control) => {
                    // Issue #140 B: the live fader is a *separate* section, built from the same
                    // prepared parameters, and preparation already proved the domain.
                    let ramped = FaderMuteRampBuiltins::new(parameters)
                        .expect("preparation validated the ramped fader's gain domain");
                    // #1254 D1: a fader-and-matrix channel has no input consumer, so its strip
                    // binds the plain input section a strip without controls gets.
                    let input_processor: Box<dyn GraphRuntimeProcessor> = match control.input {
                        Some(input_control) => Box::new(LiveControlInputProcessor {
                            input,
                            control: input_control,
                            live: ChannelSymmetryWitness::SYMMETRIC,
                        }),
                        None => Box::new(InputProcessor(input)),
                    };
                    (
                        input_processor,
                        Box::new(LiveControlFaderProcessor {
                            fader: ramped,
                            control: control
                                .fader
                                .expect("a strip is banked on all three stages or on none"),
                            automation,
                        }),
                        Box::new(LiveControlMatrixProcessor {
                            matrix,
                            control: control
                                .matrix
                                .expect("a strip is banked on all three stages or on none"),
                        }),
                    )
                }
            };
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id.clone(), TrackStage::PostInputBuiltins),
                input_processor,
            ));
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id.clone(), TrackStage::PostFader),
                fader_processor,
            ));
            bindings.push(graph::GraphNodeBinding::new(
                stage_node(graph_id, TrackStage::PostMatrix),
                matrix_processor,
            ));
        }
        bindings
    }

    /// Every track's prepared **input-builtins** channel-symmetry witness, in track order.
    ///
    /// The upstream-of-seam half of this crate's contribution to a track's pool class: the input
    /// section is the one stage here that a collapse would run once, so its designed-word
    /// comparison gates the class. The fader and the matrix are seam-side (`SEAM_SIDE_WITNESS`)
    /// and are deliberately absent -- a collapsed track duplicates its plane *into* them, so
    /// folding their vacuously-symmetric witness in here would add a term that can never be false
    /// and read, to a later caller, as if the seam had been checked.
    ///
    /// Read from the **bank** copy of each track's input section, which is the one a banked plan
    /// renders. It is prepared from the same parameters as the scalar fallback by the same
    /// `BuiltinChain::new` call shape, so the two cannot design different words.
    pub fn input_channel_symmetry(&self) -> impl Iterator<Item = (&str, ChannelSymmetryWitness)> {
        self.bank_inputs
            .iter()
            .map(|(track, input)| (track.as_ref(), input.channel_symmetry()))
    }

    /// Exact retained resource addition for the selected production bank layout.
    ///
    /// This is a read-only transactional preflight: graph/session caps can reject the final
    /// artifact while both prepared inputs are still owned by their caller.
    pub fn graph_builtin_bank_resource(
        &self,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> Option<GraphBuiltinBankResourceEstimate> {
        let Some(width) = BankWidth::for_backend(dispatch) else {
            return Some(GraphBuiltinBankResourceEstimate::default());
        };
        let plan = planned_strip_banks(&self.seal.tracks, dispatch, levels, classes);
        self.strip_bank_resource(&plan, width)
    }

    /// The retained bytes of the strip banks of `plan` at `width`: each bank kind's processors,
    /// members and scratch, and the stored fader automation lowering installs on the fader banks
    /// (#1646 D4). The one sum the preflight and the lowering both take.
    fn strip_bank_resource(
        &self,
        plan: &[(TrackStage, Vec<Box<[GraphNodeId]>>); 3],
        width: BankWidth,
    ) -> Option<GraphBuiltinBankResourceEstimate> {
        let mut total = GraphBuiltinBankResourceEstimate::default();
        for (stage, groups) in plan {
            let processor_bytes = strip_processor_bytes(*stage, width)?;
            let kind = builtin_bank_resource(groups, width, self.seal.quantum, processor_bytes)?;
            total = add_bank_resource(total, kind)?;
        }
        let fader_groups = plan
            .iter()
            .find(|(stage, _)| *stage == TrackStage::PostFader)
            .map_or(&[][..], |(_, groups)| groups.as_slice());
        add_bank_resource(
            total,
            self.fader_automation_bank_resource(fader_groups, width)?,
        )
    }

    /// The stored fader automation the fader banks of `groups` retain (#1646 D4), as bank payload:
    /// the program once (its shared allocation and the builder's report), one lane slot per bank
    /// lane of each bank with an automated member, and one offsets cell per automated channel.
    /// Nothing when no member is automated.
    fn fader_automation_bank_resource(
        &self,
        groups: &[Box<[GraphNodeId]>],
        width: BankWidth,
    ) -> Option<GraphBuiltinBankResourceEstimate> {
        let none = GraphBuiltinBankResourceEstimate::default();
        let Some(automation) = &self.fader_automation else {
            return Some(none);
        };
        let sizes = fader_automation_sizes();
        let slots_per_bank = sizes
            .lane_slot_bytes
            .checked_mul(u64::from(width.lanes()))?;
        let mut payload_bytes = 0_u64;
        for members in groups {
            let channels =
                automation.automated_channels(members.iter().filter_map(
                    |member| match member {
                        GraphNodeId::TrackStage { track_id, .. } => Some(track_id.as_str()),
                        _ => None,
                    },
                ))?;
            // #1651: a bank whose members automate only their mute holds the slots too.
            if channels == 0
                && !members.iter().any(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => {
                        automation.strip(track_id.as_str()).is_some()
                    }
                    _ => false,
                })
            {
                continue;
            }
            // #1655 R19-6: each composed channel's curve and cursor table (the reach pairs), as
            // bank payload under the existing caps.
            let (composed, _) =
                automation.composed_bytes(members.iter().filter_map(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => Some(track_id.as_str()),
                    _ => None,
                }))?;
            payload_bytes = payload_bytes
                .checked_add(slots_per_bank)?
                .checked_add(sizes.offsets_cell_bytes.checked_mul(channels)?)?
                .checked_add(composed)?;
        }
        if payload_bytes == 0 {
            return Some(none);
        }
        let program_bytes = automation.retained_program_bytes()?;
        let (_, largest_composed) = automation.composed_bytes(
            groups
                .iter()
                .flat_map(|members| members.iter())
                .filter_map(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => Some(track_id.as_str()),
                    _ => None,
                }),
        )?;
        Some(GraphBuiltinBankResourceEstimate {
            payload_bytes: payload_bytes.checked_add(program_bytes)?,
            // A ceiling on the largest single allocation: the program's arrays are each within its
            // report.
            largest_allocation_bytes: slots_per_bank
                .max(sizes.offsets_cell_bytes)
                .max(sizes.program_overhead_bytes)
                .max(automation.program_bytes)
                .max(largest_composed),
            ..none
        })
    }

    /// Exact pre-bind charge for live scalar owners. Vector backends consume every strip into
    /// banks, so only the scalar backend retains these concrete per-node boxes.
    #[cfg(any(test, feature = "test-support"))]
    pub fn graph_scalar_owner_resource(
        &self,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> Option<graph::GraphScalarOwnerResourceEstimate> {
        let fader = u64::try_from(core::mem::size_of::<LiveControlFaderProcessor>()).ok()?;
        let matrix = u64::try_from(core::mem::size_of::<LiveControlMatrixProcessor>()).ok()?;
        let banked: std::collections::BTreeSet<_> =
            planned_strip_banks(&self.seal.tracks, dispatch, levels, classes)
                .into_iter()
                .find(|(stage, _)| *stage == TrackStage::PostFader)
                .map(|(_, groups)| {
                    groups
                        .into_iter()
                        .flat_map(|group| group.into_vec())
                        .filter_map(|node| match node {
                            GraphNodeId::TrackStage { track_id, .. } => {
                                Some(Box::<str>::from(track_id.as_str()))
                            }
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
        let count = u64::try_from(
            self.strips
                .iter()
                .filter(|strip| strip.control.is_some() && !banked.contains(&strip.track_id))
                .count(),
        )
        .ok()?;
        let owners = fader.checked_add(matrix)?.checked_mul(count)?;
        // #1646 D4 (R17-4): the stored fader automation an unbanked strip's fader owner holds,
        // boxed, with its offsets cells, and the program once.
        let (automation_bytes, automation_largest) = match &self.fader_automation {
            None => (0, 0),
            Some(plan) => {
                let sizes = fader_automation_sizes();
                let lane = u64::try_from(core::mem::size_of::<LaneFaderAutomation>()).ok()?;
                let unbanked = self
                    .strips
                    .iter()
                    .filter(|strip| !banked.contains(&strip.track_id))
                    .map(|strip| strip.track_id.as_ref());
                let strips = u64::try_from(
                    unbanked
                        .clone()
                        .filter(|track| plan.strip(track).is_some())
                        .count(),
                )
                .ok()?;
                let channels = plan.automated_channels(unbanked.clone())?;
                // #1655 R19-6: each composed channel's curve and cursor table.
                let (composed, largest_composed) = plan.composed_bytes(unbanked)?;
                if strips == 0 {
                    (0, 0)
                } else {
                    (
                        lane.checked_mul(strips)?
                            .checked_add(sizes.offsets_cell_bytes.checked_mul(channels)?)?
                            .checked_add(composed)?
                            .checked_add(plan.retained_program_bytes()?)?,
                        lane.max(sizes.offsets_cell_bytes)
                            .max(sizes.program_overhead_bytes)
                            .max(plan.program_bytes)
                            .max(largest_composed),
                    )
                }
            }
        };
        let total_bytes = owners.checked_add(automation_bytes)?;
        Some(graph::GraphScalarOwnerResourceEstimate {
            total_bytes,
            largest_allocation_bytes: if count == 0 {
                automation_largest
            } else {
                fader.max(matrix).max(automation_largest)
            },
        })
    }

    /// The pre-bind charge for live scalar owners in a build without the scalar oracle: none.
    ///
    /// The per-node strip owners exist only for tests and `test-support` (issue #1059). Every
    /// dispatch a shipped build can name has a bank width, and at a bank width the planner banks
    /// every strip, so the oracle form above would count zero owners.
    #[cfg(not(any(test, feature = "test-support")))]
    pub fn graph_scalar_owner_resource(
        &self,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> Option<graph::GraphScalarOwnerResourceEstimate> {
        let _ = (dispatch, levels, classes);
        Some(graph::GraphScalarOwnerResourceEstimate::default())
    }

    /// The per-node bindings of every strip no bank claimed.
    ///
    /// With the scalar oracle compiled in (tests and `test-support`) these are
    /// [`Self::strip_bindings`]. Without it there are none to make: every dispatch a shipped build
    /// can name has a bank width, and the planner banks every strip at a bank width, so a strip
    /// left over here would be a planner defect, and it is refused rather than dropped (#1059).
    fn unbanked_strip_bindings(&mut self) -> Vec<graph::GraphNodeBinding> {
        #[cfg(any(test, feature = "test-support"))]
        {
            self.strip_bindings()
        }
        #[cfg(not(any(test, feature = "test-support")))]
        {
            assert!(
                self.strips.is_empty(),
                "a bank-width dispatch banks every strip; the per-node strip owners are compiled \
                 only with builtins-compiler/test-support (#1059)"
            );
            Vec::new()
        }
    }

    /// Materialize post-input builtin banks using the already-selected host dispatch.
    ///
    /// Every post-input node in a level with a vector backend is banked; the last bank of a
    /// level is padded with identity lanes.  Scalar `InputProcessor` bindings remain only when
    /// `BankWidth::for_backend(dispatch)` is `None`, which only the test-only `Backend::Scalar`
    /// gives (issue #1059).
    ///
    /// Lowering is infallible after `graph_builtin_bank_resource`: `with_builtin_banks` consumes
    /// the plan on error, so the read-only preflight is what makes the attach transactional and
    /// the `expect`s here are this crate's own planner invariants.
    ///
    /// Kept out of line: `scripts/check-scalar-oracle-absent.py` reads this symbol in the shipped
    /// wasm and C ABI artifacts as its positive control (#1765 moved its body into
    /// `lower_strip_banks`, which let the optimizer inline it).
    #[inline(never)]
    pub fn into_graph_artifact_with_banks<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
        dispatch: Backend,
        levels: &[DependencyLevel],
        classes: &SessionPoolClasses,
    ) -> PreparedBuiltinsGraphArtifact<R> {
        // #1646 D2: the banks bake each lane's prepared fader gain and input coefficients here, so
        // lowering runs in the canonical floating-point environment, as preparation does.
        let _canonical = lane::CanonicalFpEnv::enter();
        let Some(width) = BankWidth::for_backend(dispatch) else {
            #[cfg(any(test, feature = "test-support"))]
            return self.into_graph_artifact(graph, report);
            #[cfg(not(any(test, feature = "test-support")))]
            panic!(
                "the whole-plan scalar lowering is compiled only with builtins-compiler/test-support \
                 (#1059)"
            );
        };
        let automation_program = self.automation_program();
        let plan = planned_strip_banks(&self.seal.tracks, dispatch, levels, classes);
        if plan.iter().all(|(_, groups)| groups.is_empty()) {
            let mut processors = core::mem::take(&mut self.processors);
            processors.append(&mut self.unbanked_strip_bindings());
            return PreparedBuiltinsGraphArtifact {
                graph,
                builtin_processors: processors,
                builtin_observers: self.observers,
                report,
                track_controls: self.track_controls,
                meter_consumers: self.meter_consumers,
                automation_program,
            };
        }
        let resource = self
            .strip_bank_resource(&plan, width)
            .expect("preflighted builtin-bank resource");
        let (graph_banks, selected) = self.lower_strip_banks(plan, dispatch, width);
        let mut processors = core::mem::take(&mut self.processors);
        processors.retain(|binding| !selected.contains(&binding.node));
        processors.append(&mut self.unbanked_strip_bindings());

        let graph = graph
            .with_builtin_banks(graph_banks, resource)
            .expect("validated fixed builtin member shape");
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: processors,
            builtin_observers: self.observers,
            report,
            track_controls: self.track_controls,
            meter_consumers: self.meter_consumers,
            automation_program,
        }
    }

    /// Lower `plan`'s strip groups into the builtin bank processors that render them (issue #212),
    /// moving each member's prepared input, live readers and stored fader automation into its
    /// bank lane: the body of [`Self::into_graph_artifact_with_banks`]. Returns the banks and the
    /// member nodes they claim; a strip whose three stages were all claimed leaves `self.strips`.
    fn lower_strip_banks(
        &mut self,
        plan: [(TrackStage, Vec<Box<[GraphNodeId]>>); 3],
        dispatch: Backend,
        width: BankWidth,
    ) -> (Vec<GraphPreparedBuiltinBank>, BTreeSet<GraphNodeId>) {
        // Borrowed: a strip no bank claims keeps its automation for the scalar oracle's bindings.
        let fader_automation = &self.fader_automation;

        let mut bank_inputs: BTreeMap<Box<str>, InputBuiltins> =
            core::mem::take(&mut self.bank_inputs).into_iter().collect();
        // The strip preparations are indexed by track and *consumed* by whichever bank claims
        // them, so a track's live-control consumers can only ever reach one owner. Whatever is left
        // afterwards keeps its per-node bindings.
        let mut strips: BTreeMap<Box<str>, StripPreparation> = core::mem::take(&mut self.strips)
            .into_iter()
            .map(|strip| (strip.track_id.clone(), strip))
            .collect();
        let lanes = width.lanes() as usize;
        let mut selected = BTreeSet::new();
        let mut claimed_input: BTreeSet<Box<str>> = BTreeSet::new();
        let mut claimed_fader: BTreeSet<Box<str>> = BTreeSet::new();
        let mut claimed_matrix: BTreeSet<Box<str>> = BTreeSet::new();
        let mut graph_banks = Vec::new();

        for (stage, groups) in plan {
            for members in groups {
                fn track_of(member: &GraphNodeId) -> &str {
                    let GraphNodeId::TrackStage { track_id, .. } = member else {
                        unreachable!("prepared strip member shape")
                    };
                    track_id.as_str()
                }
                let processor: Box<dyn GraphPreparedBuiltinBankProcessor> = match stage {
                    TrackStage::PostInputBuiltins => {
                        let mut controls: Vec<Option<InputReader>> =
                            (0..lanes).map(|_| None).collect();
                        let mut inputs = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            inputs.push(
                                bank_inputs
                                    .remove(track_of(member))
                                    .expect("planner members are owned prepared builtin tracks"),
                            );
                            let strip = strips
                                .get_mut(track_of(member))
                                .expect("planner members are owned prepared strip tracks");
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.input.take());
                            claimed_input.insert(strip.track_id.clone());
                        }
                        Box::new(BuiltinBankProcessor {
                            bank: build_input_bank(dispatch, width, inputs),
                            controls: controls.into_boxed_slice(),
                            live: [ChannelSymmetryWitness::SYMMETRIC; MAXIMUM_BANK_LANES],
                            quiet_at: None,
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    TrackStage::PostFader => {
                        let mut controls: Vec<Option<FaderReader>> =
                            (0..lanes).map(|_| None).collect();
                        let mut parameters = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            let strip = strips
                                .get_mut(track_of(member))
                                .expect("planner members are owned prepared strip tracks");
                            parameters.push(strip.parameters);
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.fader.take());
                            claimed_fader.insert(strip.track_id.clone());
                        }
                        // #1646 D1: one slot per bank lane when a member's fader is automated,
                        // charged by `fader_automation_bank_resource`.
                        let automation: Box<[Option<LaneFaderAutomation>]> =
                            match fader_automation.as_ref() {
                                Some(plan)
                                    if members
                                        .iter()
                                        .any(|member| plan.strip(track_of(member)).is_some()) =>
                                {
                                    (0..lanes)
                                        .map(|lane| {
                                            let track = members.get(lane).map(track_of)?;
                                            let cells = plan.strip(track)?;
                                            Some(bound_lane_automation(&plan.program, cells))
                                        })
                                        .collect()
                                }
                                _ => Box::default(),
                            };
                        Box::new(FaderBankProcessor {
                            bank: BuiltinFaderBank::new(dispatch, width, parameters)
                                .expect("planner emits 1..=W members at the selected width"),
                            controls: controls.into_boxed_slice(),
                            automation,
                            quiet_at: None,
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    TrackStage::PostMatrix => {
                        let mut controls: Vec<Option<MatrixReader>> =
                            (0..lanes).map(|_| None).collect();
                        let mut targets = Vec::with_capacity(members.len());
                        for (lane, member) in members.iter().enumerate() {
                            let strip = strips
                                .get_mut(track_of(member))
                                .expect("planner members are owned prepared strip tracks");
                            targets.push((
                                strip.parameters.matrix,
                                strip.parameters.smoothing_samples,
                            ));
                            controls[lane] = strip
                                .control
                                .as_mut()
                                .and_then(|control| control.matrix.take());
                            claimed_matrix.insert(strip.track_id.clone());
                        }
                        Box::new(MatrixBankProcessor {
                            bank: BuiltinMatrixBank::new(dispatch, width, targets)
                                .expect("preparation validated every matrix coefficient"),
                            controls: controls.into_boxed_slice(),
                            quiet_at: None,
                            process_calls: 0,
                            frames_processed: 0,
                        })
                    }
                    _ => unreachable!("planned_strip_banks yields only bankable stages"),
                };
                selected.extend(members.iter().cloned());
                graph_banks.push(GraphPreparedBuiltinBank {
                    backend: dispatch,
                    members,
                    processor,
                    scratch: AoSoaScratch::new(width, self.seal.quantum)
                        .expect("prepared nonzero graph quantum"),
                });
            }
        }

        // A track whose input, fader and matrix all went into banks has nothing left to bind; one
        // whose stages were only partly claimed keeps all three per-node bindings, and the parts
        // that were moved out are dead storage the banks never render. That case cannot arise from
        // `planned_strip_banks` -- it plans all three stages over one track list -- and the drain
        // ownership is still single, because a `take`n consumer is `None` on the side that lost it.
        // `planned_strip_banks` plans all three stages over one track list, so the three claimed
        // sets are the same set. Stated as an assertion rather than assumed: the retain below is
        // written for the partly-claimed case because that case must not silently bind a track
        // twice, and this is what says the case is unreachable rather than merely unhandled.
        debug_assert_eq!(
            claimed_input, claimed_fader,
            "the planner claims stages together"
        );
        debug_assert_eq!(
            claimed_input, claimed_matrix,
            "the planner claims stages together"
        );
        strips.retain(|track, _| {
            !(claimed_input.contains(track)
                && claimed_fader.contains(track)
                && claimed_matrix.contains(track))
        });
        self.strips = strips.into_values().collect();
        (graph_banks, selected)
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_corrupt_for_compiler_test(
        &mut self,
        corruption: PreparedBuiltinsCorruptionCase,
    ) {
        match corruption {
            PreparedBuiltinsCorruptionCase::SessionHash => self.seal.session_sha256[0] ^= 1,
            PreparedBuiltinsCorruptionCase::SessionRate => {
                self.seal.sample_rate = self.seal.sample_rate.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::SessionQuantum => {
                self.seal.quantum = self.seal.quantum.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::TrackMissing => self.seal.tracks.clear(),
            PreparedBuiltinsCorruptionCase::TrackExtra => {
                self.seal.tracks.push("forged-track".into());
            }
            PreparedBuiltinsCorruptionCase::TrackDuplicate => {
                if let Some(track) = self.seal.tracks.first().cloned() {
                    self.seal.tracks.push(track);
                }
            }
            // The seal records three stages per track and preparation owns them in two places
            // since #212 -- one binding plus one `StripPreparation` -- so dropping either has to
            // be caught. `strips.pop()` removes a track's fader *and* matrix stages at once,
            // which is the shape a lost strip actually takes.
            PreparedBuiltinsCorruptionCase::ProcessorMissing => {
                if self.processors.pop().is_none() {
                    self.strips.pop();
                }
            }
            PreparedBuiltinsCorruptionCase::ProcessorExtra => self
                .seal
                .processors
                .push(("forged-processor".into(), TrackStage::PostMatrix)),
            PreparedBuiltinsCorruptionCase::ProcessorChangedStage => {
                if let Some((_, stage)) = self.seal.processors.first_mut() {
                    *stage = TrackStage::Input;
                }
            }
            PreparedBuiltinsCorruptionCase::TailMissing => {
                self.tails.pop();
            }
            PreparedBuiltinsCorruptionCase::TailExtra => self
                .seal
                .tails
                .push(("forged-tail".into(), NodeTailBound::ZERO)),
            PreparedBuiltinsCorruptionCase::TailChanged => {
                if let Some((_, bound)) = self.tails.first_mut() {
                    bound.tail = match bound.tail {
                        TailSamples::Finite(samples) => TailSamples::Finite(samples + 1),
                        TailSamples::Infinite => TailSamples::Finite(0),
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::RequestMissing => {
                self.requests.pop();
            }
            PreparedBuiltinsCorruptionCase::RequestExtra => {
                self.seal.requests.push(forged_request_seal());
            }
            PreparedBuiltinsCorruptionCase::RequestDuplicate => {
                if let Some(request) = self.requests.first().cloned() {
                    self.requests.push(request);
                }
            }
            PreparedBuiltinsCorruptionCase::ObserverMissing => {
                self.observers.pop();
            }
            PreparedBuiltinsCorruptionCase::ObserverExtra => {
                self.seal
                    .observers
                    .push(("forged-observer".into(), TrackStage::Input, u64::MAX))
            }
            PreparedBuiltinsCorruptionCase::ObserverChangedNode => {
                if let Some(observer) = self.observers.first_mut() {
                    observer.node = GraphNodeId::Output {
                        output_id: StableGraphId::parse("forged-output").expect("stable test ID"),
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerMissing => {
                self.meter_consumers.pop();
            }
            PreparedBuiltinsCorruptionCase::ConsumerExtra => {
                self.seal
                    .consumers
                    .push((u64::MAX, "forged-consumer".into(), MeterTap::Input))
            }
            PreparedBuiltinsCorruptionCase::ConsumerChangedMetadata => {
                if let Some(consumer) = self.meter_consumers.first_mut() {
                    consumer.track_id = "forged-consumer".into();
                    consumer.tap = MeterTap::PostMatrix;
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerDuplicateHandle => {
                if self.meter_consumers.len() >= 2 {
                    self.meter_consumers[1].handle = self.meter_consumers[0].handle;
                }
            }
            PreparedBuiltinsCorruptionCase::ResourceReport => {
                self.resources.engine_owned_retained_payload_bytes = self
                    .resources
                    .engine_owned_retained_payload_bytes
                    .checked_add(1)
                    .unwrap_or(0);
            }
        }
    }
}

#[cfg(feature = "test-support")]
fn forged_request_seal() -> MeterRequestSeal {
    MeterRequestSeal {
        handle: u64::MAX,
        track_id: "forged-request".into(),
        tap: MeterTap::Input,
        reset_generation: 0,
        period_frames: 1,
        peak_hold_frames: 0,
        peak_decay_bits: 0,
        queue_capacity: 1,
        metrics: MeterMetricSet::ALL,
    }
}

struct PreparedBuiltinsBindingValidation {
    builtin_nodes: BTreeSet<GraphNodeId>,
    builtin_observer_pairs: BTreeSet<(GraphNodeId, u64)>,
}

/// Split a graph bind failure back into the sealed compiler-owned artifact and the caller-owned
/// bindings. The graph may reject after the wrapper has appended its private bindings, so the
/// partition is deliberately shared by every bind policy and source-set variant.
fn restore_graph_bind_failure<R>(
    plan: Box<PreparedGraphPlan>,
    bindings: GraphRuntimeBindings,
    validation: &PreparedBuiltinsBindingValidation,
    report: R,
    track_controls: Vec<TrackControlProducer>,
    meter_consumers: Vec<MeterConsumer>,
    automation_program: Option<Arc<automation::Program>>,
) -> (PreparedBuiltinsGraphArtifact<R>, GraphRuntimeBindings) {
    let envelope = bindings.envelope;
    let mut builtin_processors = Vec::new();
    let mut external_processors = Vec::new();
    for binding in bindings.nodes {
        if validation.builtin_nodes.contains(&binding.node) {
            builtin_processors.push(binding);
        } else {
            external_processors.push(binding);
        }
    }
    let mut builtin_observers = Vec::new();
    let mut external_observers = Vec::new();
    for observer in bindings.observers {
        if validation
            .builtin_observer_pairs
            .contains(&(observer.node.clone(), observer.handle))
        {
            builtin_observers.push(observer);
        } else {
            external_observers.push(observer);
        }
    }
    (
        PreparedBuiltinsGraphArtifact {
            graph: *plan,
            builtin_processors,
            builtin_observers,
            report,
            track_controls,
            meter_consumers,
            automation_program,
        },
        GraphRuntimeBindings {
            envelope,
            nodes: external_processors,
            observers: external_observers,
        },
    )
}

impl<R> PreparedBuiltinsGraphArtifact<R> {
    /// Give each builtin bank its members' floored input arrivals `a(n)` (#1647a, R17-2), from
    /// `GraphCompileReport::input_arrivals`. The graph compiler's one call at the end of
    /// `compile_with_builtins`; it changes nothing but what the banks hold of `a(n)`. Off the
    /// render thread.
    #[doc(hidden)]
    pub fn install_builtin_input_arrivals(
        &mut self,
        arrivals: &[(GraphNodeId, effect_contract::LatencySamples)],
    ) {
        self.graph.install_builtin_input_arrivals(arrivals);
        // #1647b (R17-4): the scalar oracle's bound fader owners take their node's `a(n)` by the
        // same lookup.
        for binding in &mut self.builtin_processors {
            let arrival = graph::floored_input_arrival(arrivals, &binding.node);
            binding.set_input_arrival(arrival);
        }
    }
    /// Immutable caller-owned graph report.
    #[must_use]
    /// The sealed graph, by shared reference.
    ///
    /// Read-only, and deliberately so: #99 F5 stopped `GraphCompileReport` from carrying its own
    /// copy of the plan's vectors, so the callers that used to read them from the report read
    /// them here instead. The seal's compile-fail doctests still hold -- a `&` cannot extract,
    /// clone or mutate the artifact's provenance.
    pub const fn graph(&self) -> &PreparedGraphPlan {
        &self.graph
    }
    pub const fn report(&self) -> &R {
        &self.report
    }

    /// Envelope required by the still-unbound graph.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.graph.envelope
    }

    /// Number of sealed production post-input builtin banks retained by this artifact.
    #[must_use]
    pub const fn prepared_builtin_bank_count(&self) -> usize {
        self.graph.prepared_builtin_bank_count()
    }

    /// Address-free backend, width, member and active-mask metadata for qualification.
    pub fn prepared_builtin_banks(&self) -> impl Iterator<Item = GraphPreparedBuiltinBankInfo<'_>> {
        self.graph.builtin_bank_info()
    }

    /// Exact graph-owned storage after retained builtin-bank attachment.
    #[must_use]
    pub const fn graph_resource_estimate(&self) -> &graph::GraphResourceEstimate {
        &self.graph.estimate
    }

    /// Give every route into a submix input a live lane before bind (issue #1220 D7, #1347),
    /// returning one producer per lane in canonical route-ID order. Every lane adds its replaced
    /// writes to `superseded`, the session's count. Delegates to
    /// [`PreparedGraphPlan::attach_route_controls`]: the sealed graph is attached in place and
    /// never handed out. A second call is refused
    /// ([`graph::GraphRouteControlError::AlreadyAttached`]).
    pub fn attach_route_live_controls(
        &mut self,
        superseded: &LiveValuesSuperseded,
    ) -> Result<Vec<graph::GraphRouteControlProducer>, graph::GraphRouteControlError> {
        self.graph.attach_route_controls(superseded)
    }

    /// [`Self::attach_route_live_controls`] with following routes (#1652c): delegates to
    /// [`PreparedGraphPlan::attach_route_controls_following`], which gives every route into a
    /// submix that `following` names a seven-word lane, and the others a six-word lane when
    /// `six_word` is set, and binds each route of `transition` that gets no lane as a transition
    /// route (#1363 D2). Returns the six-word and the seven-word producers, each in canonical
    /// route-ID order, and the transition routes' charge.
    #[allow(clippy::type_complexity)]
    pub fn attach_route_live_controls_following(
        &mut self,
        superseded: &LiveValuesSuperseded,
        six_word: bool,
        transition: &[graph::StableGraphId],
        following: impl FnMut(&graph::StableGraphId) -> Option<graph::GraphFollowRouteSource>,
    ) -> Result<
        (
            Vec<graph::GraphRouteControlProducer>,
            Vec<graph::GraphFollowRouteControlProducer>,
            graph::GraphTransitionRoutes,
        ),
        graph::GraphRouteControlError,
    > {
        self.graph
            .attach_route_controls_following(superseded, six_word, transition, following)
    }

    /// Ordinary external nodes required in addition to compiler-owned builtin processors.
    pub fn external_binding_nodes(&self) -> impl Iterator<Item = &GraphNodeId> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| &binding.node)
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        self.graph
            .required_bindings
            .iter()
            .filter(move |node| !builtin_nodes.contains(node) && !bank_nodes.contains(node))
    }

    fn prepare_binding_validation(
        &self,
        bindings: &GraphRuntimeBindings,
        source_set: Option<&GraphPreparedSourceSet>,
    ) -> Result<PreparedBuiltinsBindingValidation, &'static str> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        let expected_external_nodes: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let builtin_observer_pairs: BTreeSet<_> = self
            .builtin_observers
            .iter()
            .map(|observer| (observer.node.clone(), observer.handle))
            .collect();
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(
                    observer.node,
                    GraphNodeId::TrackStage { .. } | GraphNodeId::Output { .. }
                ) && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        let coverage_matches = if let Some(source_set) = source_set {
            let source_nodes: BTreeSet<_> = source_set
                .claims()
                .iter()
                .map(|claim| claim.node.clone())
                .collect();
            let mut all_supplied = supplied.clone();
            all_supplied.extend(source_nodes);
            all_supplied == expected_external_nodes
        } else {
            supplied == expected_external_nodes
        };
        if !valid_observers {
            return Err("graph.plan.observer");
        }
        if bindings.envelope != self.graph.envelope {
            return Err("graph.plan.envelope_mismatch");
        }
        if source_set.is_some() {
            if duplicate_nodes || overlaps_builtin {
                return Err("graph.plan.binding");
            }
            if !coverage_matches {
                return Err("source.graph.binding_mismatch");
            }
        } else if duplicate_nodes || overlaps_builtin || !coverage_matches {
            return Err("graph.plan.binding");
        }
        Ok(PreparedBuiltinsBindingValidation {
            builtin_nodes,
            builtin_observer_pairs,
        })
    }

    fn append_private_bindings(&mut self, bindings: &mut GraphRuntimeBindings) {
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
    }

    fn bound_from_plan(
        mut plan: PreparedRenderPlan,
        track_controls: Vec<TrackControlProducer>,
        meter_consumers: Vec<MeterConsumer>,
        automation_program: Option<Arc<automation::Program>>,
    ) -> PreparedBuiltinsGraphBound {
        // No error path is left here: `bind` has consumed the artifact. Neither refusal can occur:
        // `plan` is the graph plan `bind` just built (not `NotAGraphPlan`) and nothing has attached
        // a program to it yet (not `AlreadyAttached`).
        if let Some(program) = automation_program {
            graph::attach_automation_program(&mut plan, program)
                .expect("a freshly bound graph plan is a graph plan with no program attached");
        }
        PreparedBuiltinsGraphBound {
            track_controls,
            plan,
            meter_consumers,
            #[cfg(any(test, feature = "test-support"))]
            test_gate: None,
        }
    }

    /// Consume the sealed wrapper and attach its private builtin bindings exactly once.
    #[allow(clippy::result_large_err)]
    pub fn into_bound(
        mut self,
        mut bindings: GraphRuntimeBindings,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphBindFailure<R>> {
        let validation = match self.prepare_binding_validation(&bindings, None) {
            Ok(validation) => validation,
            Err(code) => {
                return Err(PreparedBuiltinsGraphBindFailure {
                    artifact: self,
                    bindings,
                    code,
                });
            }
        };
        self.append_private_bindings(&mut bindings);
        let PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: _,
            builtin_observers: _,
            report,
            track_controls,
            meter_consumers,
            automation_program,
        } = self;
        match graph.bind(bindings) {
            Ok(plan) => Ok(Self::bound_from_plan(
                plan,
                track_controls,
                meter_consumers,
                automation_program,
            )),
            Err(failure) => {
                let (artifact, bindings) = restore_graph_bind_failure(
                    failure.plan,
                    failure.bindings,
                    &validation,
                    report,
                    track_controls,
                    meter_consumers,
                    automation_program,
                );
                Err(PreparedBuiltinsGraphBindFailure {
                    artifact,
                    bindings,
                    code: failure.code,
                })
            }
        }
    }

    /// Consume the sealed wrapper and bind one coordinator-owned source set.
    ///
    /// The wrapper first applies the same builtin-node and observer prevalidation as
    /// [`Self::into_bound`]. It then appends only its genuine private bindings and delegates the
    /// source claims to the graph's transactional source-set bind. Every rejection returns the
    /// opaque artifact, caller bindings, and source set without cloning or exposing sealed parts.
    #[allow(clippy::result_large_err)]
    pub fn into_bound_with_source_set(
        mut self,
        mut bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphSourceBindFailure<R>> {
        let validation = match self.prepare_binding_validation(&bindings, Some(&source_set)) {
            Ok(validation) => validation,
            Err(code) => {
                return Err(PreparedBuiltinsGraphSourceBindFailure {
                    artifact: self,
                    bindings,
                    source_set,
                    code,
                });
            }
        };
        self.append_private_bindings(&mut bindings);
        let PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: _,
            builtin_observers: _,
            report,
            track_controls,
            meter_consumers,
            automation_program,
        } = self;
        match graph.bind_with_source_set(bindings, source_set) {
            Ok(plan) => Ok(Self::bound_from_plan(
                plan,
                track_controls,
                meter_consumers,
                automation_program,
            )),
            Err(failure) => {
                let (artifact, bindings) = restore_graph_bind_failure(
                    failure.plan,
                    failure.bindings,
                    &validation,
                    report,
                    track_controls,
                    meter_consumers,
                    automation_program,
                );
                Err(PreparedBuiltinsGraphSourceBindFailure {
                    artifact,
                    bindings,
                    source_set: failure.source_set,
                    code: failure.code,
                })
            }
        }
    }
}

fn session_identity(session: &CompiledSession) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(session.canonical_json().as_bytes());
    hash.update(session.sample_rate().0.to_le_bytes());
    hash.update(session.quantum().0.to_le_bytes());
    hash.finalize().into()
}

/// Each strip's three sealed builtin stages by strip ID: input in `strips()` order, output sorted.
fn processor_seal<T: Clone + Ord>(strips: &[T]) -> Vec<(T, TrackStage)> {
    let capacity = strips
        .len()
        .checked_mul(3)
        .expect("session preparation preflighted processor count");
    let mut values = Vec::with_capacity(capacity);
    for strip in strips {
        for stage in [
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ] {
            values.push((strip.clone(), stage));
        }
    }
    values.sort_unstable();
    values
}

/// Whether a prepared session owns exactly the stages its seal records.
///
/// A prepared session holds all three of a track's stages as one [`StripPreparation`] (issue #212
/// for the fader and the matrix, #210 phase 3 for the input), so ownership is proven over the
/// strips and over whatever bindings a lowering has already produced. The seal itself is unchanged
/// -- three stages per track -- and so is what this refuses: a missing stage, a duplicated one, or
/// a binding on a node that is not a track stage at all.
fn processors_match(
    processors: &[graph::GraphNodeBinding],
    strips: &[StripPreparation],
    expected: &[(&str, TrackStage)],
) -> bool {
    let mut actual: Vec<_> = processors
        .iter()
        .filter_map(|binding| match &binding.node {
            GraphNodeId::TrackStage { track_id, stage } => Some((track_id.as_str(), *stage)),
            _ => None,
        })
        .collect();
    let bindings = actual.len();
    for strip in strips {
        actual.push((strip.track_id.as_ref(), TrackStage::PostInputBuiltins));
        actual.push((strip.track_id.as_ref(), TrackStage::PostFader));
        actual.push((strip.track_id.as_ref(), TrackStage::PostMatrix));
    }
    actual.sort();
    bindings == processors.len() && actual == expected
}

/// The bound a session with a live input lane gives each of its live strips (#1329 D5, D7): the
/// live bound, which holds for every history of live trim, polarity and filter targets, read from
/// its per-rate table (#1457 D2). Off the launch rates no bound is stated (no rate there has a
/// cutoff domain, and preparation refuses the session before this is reached).
fn live_input_bound(sample_rate: u32) -> NodeTailBound {
    builtins::input_section_live_bound_table(sample_rate).unwrap_or(NodeTailBound::UNBOUNDED)
}

/// The strip bounds the seal check expects, sorted by track ID: the sealed live bound for a strip
/// whose input lane is live, and the strip's own sealed design bound otherwise.
///
/// The bounds are the preparation's own, computed once per distinct design and sealed with the
/// session's identity (#1329 Amendment 4, R7): the check reuses them rather than computing every
/// design again, and checks the rule that chooses between them, the track set, and that the
/// payload's copy agrees with the seal's.
fn expected_tails(
    session: &CompiledSession,
    sorted_controls: &[(&str, bool)],
    sealed: &[(Box<str>, NodeTailBound)],
    live_bound: Option<NodeTailBound>,
) -> Result<Vec<(Box<str>, NodeTailBound)>, ()> {
    let model = session.normalized_model();
    let mut values: Vec<(Box<str>, NodeTailBound)> = Vec::with_capacity(model.strips().count());
    for strip in model.strips() {
        let id = strip.id.as_str();
        let live = sorted_controls
            .binary_search_by(|(control, _)| control.cmp(&id))
            .is_ok_and(|index| sorted_controls[index].1);
        let bound = if live {
            live_bound.ok_or(())?
        } else {
            sealed
                .binary_search_by(|(track, _)| track.as_ref().cmp(id))
                .map(|index| sealed[index].1)
                .map_err(|_| ())?
        };
        values.push((id.into(), bound));
    }
    values.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    Ok(values)
}

fn actual_meter_seals(
    observers: &[GraphNodeObserverBinding],
    consumers: &[MeterConsumer],
) -> (Vec<ObserverSeal>, Vec<ConsumerSeal>) {
    let mut observer_values = Vec::with_capacity(observers.len());
    for observer in observers {
        if let GraphNodeId::TrackStage { track_id, stage } = &observer.node {
            observer_values.push((Box::<str>::from(track_id.as_str()), *stage, observer.handle));
        }
    }
    observer_values.sort_unstable();
    let mut consumer_values = Vec::with_capacity(consumers.len());
    for consumer in consumers {
        consumer_values.push((
            consumer.handle.0.get(),
            Box::<str>::from(&*consumer.track_id),
            consumer.tap,
        ));
    }
    consumer_values.sort_unstable();
    (observer_values, consumer_values)
}

pub fn prepare_session_builtins(
    session: &CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    // #1646 D2: the effective values are composed in the canonical floating-point environment.
    let effective = {
        let _canonical = lane::CanonicalFpEnv::enter();
        controls::effective(
            session.normalized_model(),
            &controls::MonitorOverlay::default(),
        )
    };
    prepare_session_builtins_with_live_controls(session, &effective, requests, &[], caps)
}

/// Prepare builtins with live-control channels attached (issue #137 D1).
///
/// `effective` is every effective value of the session (decision 17, C7): preparation bakes
/// `effective.strips` as each strip's fader and mute instead of composing the VCAs itself, and
/// keeps `effective.routes`' follow gates for the graph compiler. It must be `controls::effective`
/// of the session's normalized model (under whatever monitor overlay the host holds); a value of
/// another strip or route count is a preparation diagnostic. Every entry below takes it the same way.
///
/// `controls` requests one bounded control channel per named track; the consumer half is bound
/// into that track's matrix processor and the producer half is returned in the prepared session
/// for the host to drive off render. A track named twice, or a track the session does not
/// declare, is a preparation diagnostic -- never a silently ignored request.
///
/// [`prepare_session_builtins`] is exactly this call with no control channels, so a host that does
/// not want live controls pays nothing: no queue is allocated and the matrix processors carry
/// `None`.
pub fn prepare_session_builtins_with_live_controls(
    session: &CompiledSession,
    effective: &controls::Effective,
    requests: &[MeterRequest],
    controls: &[TrackControlRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    prepare_session_builtins_with_live_controls_and_policy(
        session, effective, requests, None, controls, caps,
    )
}

/// Prepare explicitly selected meter observers for hosts that deliver control concurrently with
/// rendering. Selected requests retain the ordinary permanent observer binding policy.
pub fn prepare_selected_session_builtins_with_live_controls(
    session: &CompiledSession,
    effective: &controls::Effective,
    requests: &[SelectedMeterRequest],
    controls: &[TrackControlRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    let (plain, metrics) = split_selected_requests(requests);
    prepare_session_builtins_with_live_controls_and_policy(
        session,
        effective,
        &plain,
        Some(&metrics),
        controls,
        caps,
    )
}

fn split_selected_requests(
    requests: &[SelectedMeterRequest],
) -> (Vec<MeterRequest>, Vec<MeterMetricSet>) {
    let mut plain = Vec::with_capacity(requests.len());
    let mut metrics = Vec::with_capacity(requests.len());
    for request in requests {
        plain.push(request.request.clone());
        metrics.push(request.metrics);
    }
    (plain, metrics)
}

fn prepare_session_builtins_with_live_controls_and_policy(
    session: &CompiledSession,
    effective: &controls::Effective,
    requests: &[MeterRequest],
    selected_metrics: Option<&[MeterMetricSet]>,
    controls: &[TrackControlRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    // #1646 D2: every curve evaluation and conversion of preparation runs in the canonical
    // floating-point environment, as render's do. The whole body is held so a strip's prepared
    // fader (automated or static) and its input coefficients are computed under one environment,
    // whatever the caller's thread has set: a flat automated lane bakes the bits a static fader
    // bakes by construction. The lowerings hold their own (`into_graph_artifact_with_banks`,
    // `into_graph_artifact`).
    let _canonical = lane::CanonicalFpEnv::enter();
    let mut diagnostics = Vec::new();
    if [
        caps.maximum_total_state_bytes,
        caps.maximum_total_retained_payload_bytes,
        caps.maximum_total_meter_items,
        caps.maximum_total_meter_bytes,
        caps.maximum_single_allocation_bytes,
        caps.maximum_meter_streams,
    ]
    .into_iter()
    .any(|value| value == 0)
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }
    match u64::try_from(requests.len()) {
        Ok(count) if count > caps.maximum_meter_streams => {
            diagnostics.push(diag("builtin.resource.limit", "$.meter_requests"));
        }
        Err(_) => diagnostics.push(diag(
            "builtin.resource.arithmetic_overflow",
            "$.meter_requests",
        )),
        Ok(_) => {}
    }
    let mut request_keys = BTreeSet::new();
    let mut request_handles = BTreeSet::new();
    for (index, request) in requests.iter().enumerate() {
        if !request_handles.insert(request.handle) {
            diagnostics.push(diag("builtin.meter.duplicate_handle", &meter_path(request)));
        }
        let key = (request.track_id.as_str(), request.tap);
        if !request_keys.insert(key) {
            diagnostics.push(diag("builtin.meter.duplicate", &meter_path(request)));
        }
        if request.config.period_frames.get() > caps.maximum_period_frames
            || request.config.peak_hold_frames > caps.maximum_peak_hold_frames
        {
            diagnostics.push(diag("builtin.resource.limit", &meter_path(request)));
        }
        if !request.config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&request.config.peak_decay_db_per_second)
        {
            diagnostics.push(diag("builtin.meter.config", &meter_path(request)));
        }
        let metrics = selected_metrics.map_or(MeterMetricSet::ALL, |selections| selections[index]);
        if !metrics.is_valid() {
            diagnostics.push(diag("builtin.meter.metrics", &meter_path(request)));
        }
    }
    if effective.strips.len() != session.normalized_model().strips().count()
        || effective.routes.len() != session.normalized_model().routes.len()
    {
        diagnostics.push(diag("builtin.effective.shape", "$.effective"));
    }
    let known_strips: BTreeSet<_> = session
        .normalized_model()
        .strips()
        .map(|strip| strip.id.as_str())
        .collect();
    for request in requests {
        if !known_strips.contains(request.track_id.as_str()) {
            diagnostics.push(diag("builtin.meter.unknown_track", &meter_path(request)));
        }
    }
    let mut control_tracks = BTreeSet::new();
    for control in controls {
        if !control_tracks.insert(control.track_id.as_str()) {
            diagnostics.push(diag("builtin.control.duplicate", &control_path(control)));
        }
        if !known_strips.contains(control.track_id.as_str()) {
            diagnostics.push(diag(
                "builtin.control.unknown_track",
                &control_path(control),
            ));
        }
    }
    // #1242 D2: the domain preflight checks each strip's own fader values, so a VCA's clamp
    // never turns an out-of-domain member fader into an acceptance.
    for strip in session.normalized_model().strips() {
        match strip_parameters(&strip, own_fader(&strip), caps.maximum_smoothing_samples)
            .and_then(|parameters| BuiltinChain::new(session.sample_rate().0, parameters))
        {
            Ok(_) => {}
            Err(error) => {
                diagnostics.push(parameter_diagnostic(&strip, error, session.sample_rate().0))
            }
        }
    }
    let resource_plan = match resource_plan(session, requests, controls) {
        Ok(value) => Some(value),
        Err(error) => {
            diagnostics.push(error);
            None
        }
    };
    if let Some(plan) = resource_plan {
        let report = plan.report;
        if report.engine_owned_processor_payload_bytes > caps.maximum_total_state_bytes
            || report.engine_owned_retained_payload_bytes
                > caps.maximum_total_retained_payload_bytes
            || report.meter_items > caps.maximum_total_meter_items
            || report.engine_owned_meter_payload_bytes > caps.maximum_total_meter_bytes
            || report.maximum_single_allocation_bytes > caps.maximum_single_allocation_bytes
        {
            diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
        }
    }
    if !diagnostics.is_empty() {
        return Err(BuiltinDiagnosticSet::sorted(diagnostics));
    }
    // #1646 D1, D2: every form compiles the stored fader automation here, before phase two (its
    // program and cells are graph plan payload, charged by the bank estimate, D4).
    let fader_automation = prepare_fader_automation(session, effective)
        .map_err(|diagnostic| BuiltinDiagnosticSet::sorted(vec![diagnostic]))?;
    // The fader each strip prepares: its effective fader, and on an automated lane the lane's
    // prepared value composed with its `S` (D2); its effective mute, and on an automated mute
    // lane the prepared `curve || terms` (#1651 D1).
    let prepared_fader = |strip: &StripRef<'_>, fader: &controls::EffectiveStrip| {
        fader_automation
            .as_ref()
            .map_or((fader.fader_db, fader.mute), |plan| {
                (
                    plan.fader_db(strip.id.as_str(), fader.fader_db),
                    plan.mute(strip.id.as_str(), fader.mute),
                )
            })
    };
    let resources = resource_plan.expect("validated resource plan").report;
    // #1242 D2, decision 17 C7: lowering bakes each strip's effective fader and mute, which the
    // caller computed (strip `i` is entry `i`).
    let effective_faders = &*effective.strips;
    // The request index is a transient lookup, not retained storage, so it is built before the
    // phase-two observation that compares retained allocations with the report (#1254 gate 3).
    let mut control_lanes: BTreeMap<&str, (bool, &LiveValuesSuperseded)> = BTreeMap::new();
    for control in controls {
        control_lanes.insert(
            control.track_id.as_str(),
            (control.input_lane, &control.superseded),
        );
    }
    // #1329 D7, Amendment 4 R7: each distinct input design's certified tail and rest bound is
    // computed here, at most once per preparation, before the phase-two observation: the
    // computation allocates transient storage, which phase two's retained-allocation account must
    // not see. A live input lane's bound depends only on the rate, so it is read from the table
    // (#1457 D2) and computed nowhere.
    // Amendment 5 (MJ1): a strip whose input lane is live reports the live bound, so no design
    // bound is computed for it, so a browser boot with live controls computes none. An audio-only
    // browser boot (the SDK default) and the C ABI attach no input lane and bound every distinct
    // design (#1329 follow-up C). #1457: within the preparation budget
    // `builtins::INPUT_BOUND_BUDGET_FRAMES`, charged in strip order; a design past it reports the
    // live bound (D3).
    let input_lane_live = |track_id: &str| {
        control_lanes
            .get(track_id)
            .is_some_and(|(input_lane, _)| *input_lane)
    };
    let design_bounds = builtins::input_section_bounds(
        session.sample_rate().0,
        session
            .normalized_model()
            .strips()
            .zip(effective_faders)
            .filter(|(strip, _)| !input_lane_live(strip.id.as_str()))
            .map(|(strip, fader)| {
                strip_parameters(
                    &strip,
                    prepared_fader(&strip, fader),
                    caps.maximum_smoothing_samples,
                )
                .expect("preflighted parameters")
            }),
    )
    .expect("preflighted coefficients");
    let mut design_bounds = design_bounds.into_iter();
    let live_bound = control_lanes
        .values()
        .any(|(input_lane, _)| *input_lane)
        .then(|| live_input_bound(session.sample_rate().0));
    #[cfg(feature = "test-support")]
    let _phase_two_tracker = TestPhaseTwoAllocationGuard::begin();
    let track_count = session.normalized_model().strips().count();
    // Empty, and it stays empty: every track stage is held as a `StripPreparation` until lowering
    // (#212 for the fader and the matrix, #210 phase 3 for the input). The vector exists so that
    // `into_graph_artifact` has somewhere to put the bindings it builds, and so that
    // `processors_match` keeps refusing a binding on a node that is not a track stage.
    let mut processors: Vec<graph::GraphNodeBinding> = Vec::new();
    let _ = &mut processors;
    let mut strips = Vec::with_capacity(track_count);
    let mut bank_inputs = Vec::with_capacity(track_count);
    let mut tails = Vec::with_capacity(track_count);
    let mut track_controls = Vec::with_capacity(controls.len());
    let mut control_seal: Vec<(Box<str>, bool)> = Vec::with_capacity(controls.len());
    for (strip, fader) in session.normalized_model().strips().zip(effective_faders) {
        // An automated lane's prepared value is in the fader domain: `vca_compose_db` clamps and
        // `prepare_fader_automation` checked its gain.
        let parameters = strip_parameters(
            &strip,
            prepared_fader(&strip, fader),
            caps.maximum_smoothing_samples,
        )
        .expect("preflighted parameters");
        let chain = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted coefficients");
        let (input, fader, matrix) = chain.into_sections();
        // The bank candidate is prepared independently from the scalar fallback.  The selected
        // full-bank artifact consumes this copy and removes the corresponding scalar binding;
        // the scalar input remains the transactional fallback until that point.
        let bank_input = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted bank coefficients")
            .into_input_builtins();
        // #1329 D7: the same rule as the seal check's `expected_tails`. The design bounds are the
        // non-live strips' own, in strip order.
        let bound = if input_lane_live(strip.id.as_str()) {
            live_bound.expect("a live input lane computed the live bound")
        } else {
            design_bounds
                .next()
                .expect("one design bound per non-live strip")
        };
        tails.push((Box::<str>::from(strip.id.as_str()), bound));
        bank_inputs.push((Box::<str>::from(strip.id.as_str()), bank_input));
        let graph_id = StableGraphId::parse(strip.id.as_str()).expect("preflighted stable ID");
        let control = match control_lanes.get(strip.id.as_str()) {
            None => None,
            Some(&(input_lane, superseded)) => {
                let (producer, control) = matrix_lane(superseded);
                let (mut fader_producer, fader_control) = fader_lane(superseded);
                // #1649 D4: the strip's offsets cells' write ends, charged by `resource_plan`.
                if let Some(cells) = fader_automation
                    .as_ref()
                    .and_then(|plan| plan.strip(strip.id.as_str()))
                {
                    fader_producer.attach_offsets(&cells.fader);
                }
                let (input_producer, input_control) = if input_lane {
                    let (cells, reader) = input_lane_cells(superseded);
                    (Some(cells), Some(reader))
                } else {
                    (None, None)
                };
                // The ramped fader is validated here rather than at binding, so a declared
                // `fader_db` outside the live domain is a preparation diagnostic on the track that
                // carries it -- the same failure, at the same place, as before the strip banked.
                FaderMuteRampBuiltins::new(parameters).map_err(|_| {
                    BuiltinDiagnosticSet::sorted(vec![parameter_diagnostic(
                        &strip,
                        BuiltinParameterError::GainDomain,
                        session.sample_rate().0,
                    )])
                })?;
                track_controls.push(TrackControlProducer {
                    track_id: Box::<str>::from(strip.id.as_str()),
                    producer,
                    fader: fader_producer,
                    input: input_producer,
                });
                control_seal.push((Box::<str>::from(strip.id.as_str()), input_lane));
                Some(StripControlConsumers {
                    input: input_control,
                    fader: Some(fader_control),
                    matrix: Some(control),
                })
            }
        };
        strips.push(StripPreparation {
            track_id: Box::<str>::from(strip.id.as_str()),
            graph_id,
            parameters,
            input,
            fader,
            matrix,
            control,
        });
    }
    control_seal.sort_unstable();
    let mut observers = Vec::with_capacity(requests.len());
    let mut meter_consumers = Vec::with_capacity(requests.len());
    let mut request_seals = Vec::with_capacity(requests.len());
    for (index, request) in requests.iter().enumerate() {
        let handle = request.handle;
        let metrics = selected_metrics.map_or(MeterMetricSet::ALL, |selections| selections[index]);
        let PreparedMeter {
            accumulator,
            consumer,
        } = MeterAccumulator::prepare_selected(
            handle,
            request.config,
            session.sample_rate().0,
            metrics,
        )
        .map_err(|error| BuiltinDiagnosticSet::sorted(vec![meter_diagnostic(request, error)]))?;
        let graph_id = StableGraphId::parse(&request.track_id).expect("known accepted session ID");
        let observer = GraphNodeObserverBinding::new(
            stage_node(graph_id, stage(request.tap)),
            handle.0.get(),
            Box::new(MeterObserver(accumulator)),
        );
        observers.push(observer);
        meter_consumers.push(MeterConsumer {
            handle,
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            consumer,
        });
        request_seals.push(MeterRequestSeal {
            handle: handle.0.get(),
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            reset_generation: request.config.reset_generation,
            period_frames: request.config.period_frames.get(),
            peak_hold_frames: request.config.peak_hold_frames,
            peak_decay_bits: request.config.peak_decay_db_per_second.to_bits(),
            queue_capacity: request.config.queue_capacity.get(),
            metrics,
        });
    }
    tails.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    request_seals.sort_unstable();
    let tracks: Vec<Box<str>> = session
        .normalized_model()
        .strips()
        .map(|strip| strip.id.as_str().into())
        .collect();
    let processor_seal = processor_seal(&tracks);
    let (observer_seal, consumer_seal) = actual_meter_seals(&observers, &meter_consumers);
    Ok(PreparedBuiltinsSession {
        seal: BuiltinSessionSeal {
            session_sha256: session_identity(session),
            sample_rate: session.sample_rate().0,
            quantum: session.quantum().0,
            tracks,
            processors: processor_seal,
            tails: tails.clone(),
            live_bound,
            requests: request_seals.clone(),
            observers: observer_seal,
            consumers: consumer_seal,
            controls: control_seal,
            resources,
        },
        processors,
        strips,
        bank_inputs,
        observers,
        meter_consumers,
        track_controls,
        tails,
        requests: request_seals,
        resources,
        route_follow: effective.routes.iter().map(|route| route.follow).collect(),
        fader_automation,
    })
}

fn control_path(request: &TrackControlRequest) -> String {
    format!("$.controls[track_id={}]", request.track_id)
}

fn resource_plan(
    session: &CompiledSession,
    requests: &[MeterRequest],
    controls: &[TrackControlRequest],
) -> Result<BuiltinResourcePlan, BuiltinDiagnostic> {
    let track_count = session.normalized_model().strips().count();
    // The seal still records three stages per track even though preparation binds one and defers
    // two, so the seal vector below is charged at the seal's own count and not at the bindings'.
    let sealed_stage_count = track_count
        .checked_mul(3)
        .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
    let request_count = requests.len();
    let mut processor = ResourceAccumulator::default();
    let mut meter = ResourceAccumulator::default();
    // **No** bindings per track: all three stages are held as `StripPreparation` until their
    // binding form is chosen at lowering (#212 for the fader and the matrix, #210 phase 3 for the
    // input), so preparation allocates the strip vector and no boxed processors at all. The
    // binding vector is empty at preparation and charges nothing; the vector it will be at
    // lowering is charged where lowering's own storage is, which is the graph's estimate and not
    // this one.
    add_vector_layout::<StripPreparation>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, InputBuiltins)>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, NodeTailBound)>(&mut processor, track_count)?;
    add_vector_layout::<Box<str>>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, TrackStage)>(&mut processor, sealed_stage_count)?;
    add_vector_layout::<(Box<str>, NodeTailBound)>(&mut processor, track_count)?;
    // The routes' effective follow gates, retained for graph lowering (decision 17, C7).
    add_vector_layout::<[bool; 2]>(&mut processor, session.normalized_model().routes.len())?;
    for strip in session.normalized_model().strips() {
        let bytes = strip.id.as_str().len();
        // Nine independently retained copies of the track's ID: the strip's own ID and its graph
        // ID, the retained tail, the compact track seal, the three processor-seal rows, the seal's
        // cloned tail ID, and the independently retained bank-input candidate ID. The tenth was
        // the post-input binding's node ID, and it is gone: #210 phase 3 defers that binding to
        // lowering with the other two.
        for _ in 0..9 {
            processor
                .add_bytes(bytes)
                .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
        }
        // Nothing is boxed here any more. Issue #140 B's rule -- a controlled track holds the
        // ramped section, an uncontrolled one the prepared section -- still holds, and all three
        // sections now live *inline* in the strip vector charged above rather than behind a `Box`,
        // so they are charged by that vector's layout and not per track. `controlled` therefore
        // changes no byte of the *section* storage, and the only live-control-dependent rows left
        // are the input, fader and matrix cells' `Arc`s charged below.
    }
    add_vector_layout::<GraphNodeObserverBinding>(&mut meter, request_count)?;
    add_vector_layout::<MeterConsumer>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<(Box<str>, TrackStage, u64)>(&mut meter, request_count)?;
    add_vector_layout::<(u64, Box<str>, MeterTap)>(&mut meter, request_count)?;
    // Issue #137 D1: the live-control channels are charged to the processor accumulator,
    // because they are per-track processor storage rather than meter storage: the producer vector,
    // its seal, and one bounded ring per requested track.
    add_vector_layout::<TrackControlProducer>(&mut processor, controls.len())?;
    add_vector_layout::<(Box<str>, bool)>(&mut processor, controls.len())?;
    // #1649 D4: the strips whose fader automation gives their producer offsets-cell writers (the
    // rule `prepare_fader_automation` and `FaderCells::attach_offsets` follow). Transient.
    let model = session.normalized_model();
    // #1655: a fader event lane an automated VCA `offset_db` lane reaches has an offsets cell
    // too, also when its own fader is not automated.
    let automated: BTreeSet<&str> = if model.automation.is_empty() {
        BTreeSet::new()
    } else {
        let vca_rows = model
            .automation
            .iter()
            .any(|entry| controls::vca_offset_row(&entry.target));
        let reach = if vca_rows {
            model.vca_reach()
        } else {
            Vec::new()
        };
        // #1844: one pass over the table; every strip looks its entries up.
        let automation = controls::AutomationIndex::of(model);
        model
            .strips()
            .enumerate()
            .filter(|(index, strip)| {
                if vca_rows {
                    let reach = reach.get(*index).map_or(&[][..], Vec::as_slice);
                    automation
                        .fader_event_lanes(strip.id, reach)
                        .contains(&true)
                } else {
                    automation.fader(strip.id).iter().any(Option::is_some)
                }
            })
            .map(|(_, strip)| strip.id.as_str())
            .collect()
    };
    for control in controls {
        // #1312 D5, #1346 D6: every lane is a latest-target cell set, one `Arc` allocation each,
        // charged by the allocation's own layout. The input lane's is charged when requested
        // (#1254 D1). A strip with stored fader automation also holds its offsets cells' write
        // ends, one box (#1649 D4); the cells themselves are bank payload (#1646 D4).
        for (present, layout) in [
            (
                automated.contains(control.track_id.as_str()),
                core::alloc::Layout::new::<[Option<OffsetsWriter>; 2]>(),
            ),
            (
                true,
                core::alloc::Layout::new::<SharedAllocation<MatrixLane>>(),
            ),
            (
                true,
                core::alloc::Layout::new::<SharedAllocation<FaderLane>>(),
            ),
            (
                control.input_lane,
                core::alloc::Layout::new::<SharedAllocation<InputLane>>(),
            ),
        ] {
            if present {
                processor.add_layout(layout).ok_or_else(|| {
                    diag(
                        "builtin.resource.arithmetic_overflow",
                        &control_path(control),
                    )
                })?;
            }
        }
        // The public producer identity and the sealed control identity are retained separately.
        for _ in 0..2 {
            processor.add_bytes(control.track_id.len()).ok_or_else(|| {
                diag(
                    "builtin.resource.arithmetic_overflow",
                    &control_path(control),
                )
            })?;
        }
    }
    let mut meter_items = 0_u64;
    for request in requests {
        let queue =
            bounded_spsc_retained_payload::<MeterSnapshot>(request.config.queue_capacity)
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter_items = meter_items
            .checked_add(
                u64::try_from(queue.slot_count).map_err(|_| {
                    diag("builtin.resource.arithmetic_overflow", &meter_path(request))
                })?,
            )
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter
            .add_layout(
                core::alloc::Layout::from_size_align(
                    queue.ring_header_bytes,
                    queue.ring_header_align,
                )
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?,
            )
            .and_then(|_| {
                core::alloc::Layout::from_size_align(
                    queue.slot_payload_bytes,
                    queue.slot_payload_align,
                )
                .ok()
                .and_then(|layout| meter.add_layout(layout))
            })
            .and_then(|_| meter.add_layout(core::alloc::Layout::new::<MeterObserver>()))
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        let bytes = request.track_id.len();
        // Observer graph ID, public consumer ID, retained request, and three seal identities.
        for _ in 0..6 {
            meter.add_bytes(bytes).ok_or_else(|| {
                diag("builtin.resource.arithmetic_overflow", &meter_path(request))
            })?;
        }
    }
    let total = processor.total.checked_add(meter.total).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let allocations = processor
        .allocations
        .checked_add(meter.allocations)
        .ok_or_else(|| {
            diag(
                "builtin.resource.arithmetic_overflow",
                "$.builtin_compile_caps",
            )
        })?;
    let mut retained = processor;
    retained.merge(meter).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let retained = retained.sorted_layouts();
    Ok(BuiltinResourcePlan {
        report: BuiltinResourceEstimate {
            engine_owned_processor_payload_bytes: processor.total,
            engine_owned_meter_payload_bytes: meter.total,
            engine_owned_retained_payload_bytes: total,
            meter_items,
            maximum_single_allocation_bytes: processor.largest.max(meter.largest),
            retained_allocation_count: allocations,
            retained_layout_class_count: retained.layout_class_count,
            retained_layouts: retained.layouts,
        },
    })
}

fn add_vector_layout<T>(
    accumulator: &mut ResourceAccumulator,
    items: usize,
) -> Result<(), BuiltinDiagnostic> {
    if items == 0 {
        return Ok(());
    }
    let layout = core::alloc::Layout::array::<T>(items).map_err(|_| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    accumulator.add_layout(layout).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })
}

/// The `SOURCE` term of one track's channel-symmetry witness.
///
/// True exactly when the track's two dual-mono lanes read the **same** source channel, so the two
/// planes are filled with identical samples. That is the structural precondition of a mono
/// collapse: a collapsed track computes one plane, so the two planes have to have been fed
/// identically in the first place.
///
/// # Why one condition and not two
///
/// The mono-collapse design states the predicate as "`left_source_channel ==
/// right_source_channel`, **or** a one-channel source". The second is not a second case: session
/// validation refuses any channel index that is not below the source's `channel_count`
/// (`SourceChannelIndexOutOfRange`, `$.tracks[i].left_source_channel`), so a one-channel source can
/// only ever be mapped as `0, 0` -- which the first condition already admits. A separate branch
/// for it would be code no validated session can reach, and the honest form of an unreachable
/// branch is not writing it.
///
/// `session` is taken so this stays the one predicate both bank planners call even after the rule
/// grows a term the track alone cannot answer.
// Source semantics: tracks only.
#[must_use]
pub fn track_mono_source(session: &CompiledSession, track: &Track) -> bool {
    let _ = session;
    track.left_source_channel == track.right_source_channel
}

/// The `DESIGNED` term a track's **input-side delay** contributes (#210 phase 2).
///
/// True exactly when the two lanes declare the same `delay_samples`. A collapsed track computes one
/// plane and duplicates it at the seam; two lanes with different delays turn one source channel
/// into two genuinely different signals upstream of that seam, so duplicating either one is wrong
/// audio, not a saved multiply.
///
/// # Why this term is decided here and not by the input stage's word list
///
/// `InputStage::lane_channel_symmetry` compares the words its **kernel** reads -- `trim_signed` and
/// the two SVF sections' six coefficients each. `delay_samples` is not one of them: the delay is
/// applied by a graph node at `TrackStage::Input`, upstream of the bank, and the bank never sees
/// it. Adding it to that list would be claiming the kernel reads a word it does not read. It is a
/// prepared-only session word, so the honest owner of the verdict is the prepare-time structural
/// witness -- here -- which is also the only one that runs before the chain is armed at all.
/// Named without a `_v1` suffix on purpose, ahead of its two neighbours: issue #215's owner ruling
/// is that pre-launch internal implementation names are born unversioned, and that issue has since
/// taken the suffix off the neighbours too.
#[must_use]
pub fn track_input_delay_symmetric(track: &Track) -> bool {
    track.builtins.left.delay_samples == track.builtins.right.delay_samples
}

/// Every track's structural channel-symmetry witness, in normalized track order.
///
/// # Why this is a function over the compiled session and not a field of the prepared plan
///
/// The structural term is what the **planner** needs: the mono-collapse design pools cohorts by
/// class -- "mono-symmetric at prepare" against "stereo" -- and that decision is taken while the
/// plan is being built, from the compiled session, before any prepared object exists. Both bank
/// planners must derive the class from the same predicate or the lane-alignment check declines
/// the chain merges silently, so the predicate is written once, here, and called rather than
/// copied.
///
/// The prepared side is deliberately source-agnostic: the SOURCE term lives only in this
/// bit into each track's input stage, so the runtime witness carries it too. This function is what
/// a caller with a `CompiledSession` and no plan asks.
// Source semantics: tracks only.
// This set is the mono-collapse guard, and a strip with no source must never enter it.
#[must_use]
pub fn session_structural_symmetry(
    session: &CompiledSession,
) -> Vec<(Box<str>, ChannelSymmetryWitness)> {
    session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| {
            let mut witness = ChannelSymmetryWitness::SYMMETRIC;
            witness.set(
                ChannelSymmetryWitness::SOURCE,
                track_mono_source(session, track),
            );
            // Issue #210 phase 2. Prepared-only state, so the whole verdict is available here and
            // needs no per-block maintenance: an asymmetric delay declines this track's collapse
            // for the life of the plan, and a symmetric one -- including the overwhelmingly common
            // zero -- leaves the witness exactly as it was before the feature existed.
            witness.set(
                ChannelSymmetryWitness::DESIGNED,
                track_input_delay_symmetric(track),
            );
            (Box::<str>::from(track.id.as_str()), witness)
        })
        .collect()
}

/// Every track's pool class for one compile, derived **once** and handed to both bank planners.
///
/// # Why this exists as an object rather than as a predicate each planner calls
///
/// The obligation stated on [`CohortPoolClass::of_prepare_witness`] is that the builtin-stage
/// planner and the rack-chain planner must classify every track *identically*: issue #208's chain
/// merges are proved lane by lane on the lowered program, and two planners whose pools hold
/// different track sets produce banks whose lane sets slide out of step, so every
/// `builtins -> EQ -> compressor -> limiter` merge declines **silently** -- the same rendered
/// bits, no diagnostic, one planar/AoSoA round-trip per stage where the strip used to pay one per
/// cohort.
///
/// A predicate each planner calls would make that agreement a property of two call sites staying
/// in step. This makes it a property of there being one value: `GraphCompiler` derives this once
/// per compile, `bind_rack_banks` reads it, and `into_graph_artifact_with_banks` reads the same
/// object. `the_two_planners_agree_on_every_track_class` is the gate on the *observable*
/// consequence -- the two planners' lane sets line up track for track -- because that, and not
/// the map, is what the merge actually needs.
///
/// # What a track's class is derived from
///
/// The conjunction of every prepare-time witness that speaks for the track, then
/// [`CohortPoolClass::of_prepare_witness`]:
///
/// * `SOURCE` from [`track_mono_source`], through [`session_structural_symmetry`]. This is
///   the term the M0 phase built and the only one the compiled session can answer alone.
/// * `DESIGNED` from each prepared upstream-of-seam stage the compile prepared: the track's
///   input builtins ([`InputBuiltins::channel_symmetry`]) and each of its prepared native effects.
///   Every compile prepares the input builtins, so that term is always present; the builtins-less
///   compile entry, which had no input stage to contribute it, was deleted by issue #959.
///
/// Absent terms are never assumed: an unknown track answers [`CohortPoolClass::Stereo`], which
/// is the class that cannot over-claim.
///
/// # The one class that is not the witness's: the stranded mono remainder (issue #971)
///
/// An effect bank binds only a **full** group (#96 F7), so a mono pool whose size is not a
/// multiple of the lane width strands its tail: those tracks' effects render per node. The rack
/// planner therefore runs a trial plan and, where every effect group a mono track sits in is a
/// partial mono group, moves the track to the stereo pool with
/// [`SessionPoolClasses::pool_as_stereo`], where it renders dual inside a bank. It binds both
/// plans and keeps the move only when the factories bound more effect banks for the new plan
/// than for the trial: a planned full group is not a bank, because a factory may decline it (the
/// delay never banks). The move is all or nothing, one decision for every stranded track. That is a
/// **pooling** decision, not a symmetry fact, so it is kept beside the witness rather than
/// written into it: the track's witness still says its source is mono, and the host's
/// structural arming join (which never reads this map) is untouched. It happens before either
/// planner forms the plan it keeps, and both read it through [`SessionPoolClasses::class_of`],
/// so the two planners' lane sets still agree track for track.
#[derive(Clone, Debug, Default)]
pub struct SessionPoolClasses {
    by_track: BTreeMap<Box<str>, ChannelSymmetryWitness>,
    /// Mono-class tracks pooled as stereo because their mono groups would strand (issue #971).
    pooled_as_stereo: BTreeSet<Box<str>>,
}

impl SessionPoolClasses {
    /// Seeds every track's witness with its structural (`SOURCE`) term, and every submix as
    /// [`CohortPoolClass::Stereo`], explicitly (#1200 D4).
    ///
    /// A submix has no source, so it has no `SOURCE` term to claim: its witness is seeded with that
    /// term cleared, which no later conjunction can set. A bus is never mono-collapsed. The
    /// collapse's own guard is the host's eligibility set, [`session_structural_symmetry`], which
    /// stays tracks-only; this keeps the pool class from ever disagreeing with it.
    // Source semantics: tracks only.
    #[must_use]
    pub fn from_session(session: &CompiledSession) -> Self {
        let mut by_track: BTreeMap<Box<str>, ChannelSymmetryWitness> =
            session_structural_symmetry(session).into_iter().collect();
        for submix in &session.normalized_model().submixes {
            by_track.insert(
                Box::<str>::from(submix.id.as_str()),
                ChannelSymmetryWitness::symmetric_except(ChannelSymmetryWitness::SOURCE),
            );
        }
        Self {
            by_track,
            pooled_as_stereo: BTreeSet::new(),
        }
    }

    /// Conjoins one prepared stage's witness into its track's.
    ///
    /// Conjunction, never assignment: a track has several upstream stages and a single declining
    /// one declines the track, which is the same rule `ChannelSymmetryWitness::and` states for
    /// every other composition of these witnesses. A stage naming a track the session does not
    /// have is ignored rather than inserted, so a mismatched caller cannot invent a class.
    pub fn conjoin(&mut self, track_id: &str, witness: ChannelSymmetryWitness) {
        if let Some(existing) = self.by_track.get_mut(track_id) {
            *existing = existing.and(witness);
        }
    }

    /// Moves one mono-class track into the stereo pool (issue #971).
    ///
    /// Pooling only: the track's witness is left as it is, because its source is still mono and
    /// nothing but the pool it competes in changes. The caller is the rack planner, which moves a
    /// track only when every effect group it sits in is a partial mono group; see the type's
    /// documentation. A track the session does not have is ignored rather than inserted, as in
    /// [`SessionPoolClasses::conjoin`], and a stereo-class track stays where it is.
    pub fn pool_as_stereo(&mut self, track_id: &str) {
        if self.class_of(track_id) == CohortPoolClass::MonoSymmetricAtPrepare
            && let Some((track, _)) = self.by_track.get_key_value(track_id)
        {
            self.pooled_as_stereo.insert(track.clone());
        }
    }

    /// This track's pool class. An unknown track is [`CohortPoolClass::Stereo`], and so is a
    /// track [`SessionPoolClasses::pool_as_stereo`] moved.
    #[must_use]
    pub fn class_of(&self, track_id: &str) -> CohortPoolClass {
        if self.pooled_as_stereo.contains(track_id) {
            return CohortPoolClass::Stereo;
        }
        self.by_track
            .get(track_id)
            .copied()
            .map_or(CohortPoolClass::Stereo, |witness| {
                CohortPoolClass::of_prepare_witness(witness)
            })
    }
}

// #1074: the heap each builtins plan owner retains, exclusive of itself. A lane's cell `Arc` is
// charged in full (its writer is control-side and drops before the plan). Two things are never
// charged here: the host-owned `LiveValuesSuperseded` counter, which outlives the plan, and the
// session's one `Arc<automation::Program>`, which the graph executor charges once for the plan.
impl InputReader {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.lane)
    }
}

impl FaderReader {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.lane)
    }
}

impl MatrixReader {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.lane)
    }
}

impl AutomatedFaderCell {
    fn retained_bytes(&self) -> u64 {
        let curve = match &self.curve {
            FaderCurve::Cell { .. } => 0,
            FaderCurve::Composed(composite) => {
                effect_contract::retained::boxed(&**composite) + composite.walk.retained_bytes()
            }
        };
        effect_contract::retained::arc(&self.offsets) + curve
    }
}

impl AutomatedMuteCell {
    fn retained_bytes(&self) -> u64 {
        match &self.curve {
            MuteCurve::Cell { .. } => 0,
            MuteCurve::Composed(composite) => {
                effect_contract::retained::boxed(&**composite) + composite.walk.retained_bytes()
            }
        }
    }
}

impl LaneFaderAutomation {
    /// Its cells' heap; the graph executor charges the shared program once (see above).
    fn retained_bytes(&self) -> u64 {
        self.channels
            .iter()
            .flatten()
            .map(AutomatedFaderCell::retained_bytes)
            .chain(
                self.mute
                    .iter()
                    .flatten()
                    .map(AutomatedMuteCell::retained_bytes),
            )
            .sum()
    }
}

/// A boxed lane automation: the box and its own heap.
#[cfg(any(test, feature = "test-support"))]
fn boxed_automation_bytes(automation: &LaneFaderAutomation) -> u64 {
    effect_contract::retained::boxed(automation) + automation.retained_bytes()
}

/// A per-lane control table: the table and each present reader's heap.
fn reader_table_bytes<T>(table: &[Option<T>], reader: impl Fn(&T) -> u64) -> u64 {
    effect_contract::retained::slice(table) + table.iter().flatten().map(reader).sum::<u64>()
}

// The whole-plan scalar path (issue #1059). A track's strip renders through the per-node owners
// below only when its plan was compiled at `Backend::Scalar`, which binds no strip bank; at every
// vector width the planner banks every strip (`planned_builtin_bank_members`). The scalar backend
// is the test-only oracle vector banking is compared against (owner ruling 2026-09-28, decision 3),
// so these owners and `strip_bindings` compile only for tests and this
// crate's `test-support` feature, and no shipped artifact contains them
// (`scripts/check-scalar-oracle-absent.py`).
#[cfg(any(test, feature = "test-support"))]
struct InputProcessor(InputBuiltins);
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for InputProcessor {
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        self.0.channel_symmetry()
    }
    fn copy_response_snapshot(
        &self,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.0.copy_response_snapshot(sample_rate_hz, request)
    }
    fn response_snapshot_declared(&self) -> bool {
        true
    }
    fn response_snapshot_native_id(&self) -> Option<&'static str> {
        Some("miso.builtin.input-filters")
    }
}
#[cfg(any(test, feature = "test-support"))]
struct FaderProcessor(FaderMuteBuiltins);
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for FaderProcessor {
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        SEAM_SIDE_WITNESS
    }
}

/// The scalar oracle's fader of a strip with stored fader automation and no live control
/// (#1647b, R17-4, #1646b departure 5): the ramped fader, which takes timed operations, driven by
/// the one event driver the fader bank uses. [`FaderProcessor`] wraps the unramped fader, which
/// takes none, so an automated strip binds this instead.
#[cfg(any(test, feature = "test-support"))]
struct AutomatedFaderProcessor {
    fader: FaderMuteRampBuiltins,
    /// The strip's stored fader automation (#1646 D1), boxed as on the live-control owner.
    automation: Box<LaneFaderAutomation>,
}
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for AutomatedFaderProcessor {
    fn retained_bytes(&self) -> u64 {
        boxed_automation_bytes(&self.automation)
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        render_oracle_fader(
            &mut self.fader,
            Some(&mut self.automation),
            block.left,
            block.right,
            block.first_sample,
        )
    }
    fn set_input_arrival(&mut self, arrival: u64) {
        self.automation.arrival = arrival;
    }
    fn set_block_time(&mut self, time: &GraphBlockTime<'_>) {
        self.automation.take_block_time(time, block_jumps(time));
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        SEAM_SIDE_WITNESS
    }
}

// REALTIME_POLICY_BEGIN
/// The scalar oracle's fader render (#1647b, R17-4): with stored automation, the block's events
/// through the one event driver into the ramped fader's timed path, exactly as a fader bank lane
/// renders them; without, the plain block.
#[cfg(any(test, feature = "test-support"))]
fn render_oracle_fader(
    fader: &mut FaderMuteRampBuiltins,
    automation: Option<&mut LaneFaderAutomation>,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
) -> Result<(), RenderError> {
    let frames = u32::try_from(left.len()).unwrap_or(u32::MAX);
    let block = DualMonoBlock::new(left, right, first_sample).map_err(render_error)?;
    match automation {
        None => {
            fader.process(block);
        }
        Some(automation) => {
            let mut driver = FaderEventDriver::new(
                core::iter::once((0, &mut *automation)),
                frames,
                first_sample,
                |_, channel| {
                    Some(ChannelHold {
                        held: fader.gain_held(channel)?,
                        muted: fader.is_muted(channel) && fader.is_armed(channel) != Some(true),
                        remaining: fader.ramp_remaining(channel)?,
                        stage_gain: None,
                    })
                },
            );
            // A stop is unreachable, and safe in release (see `FaderBankProcessor`).
            let _ = fader.process_with(block, &mut driver);
            driver.finish();
        }
    }
    Ok(())
}
// REALTIME_POLICY_END
#[cfg(any(test, feature = "test-support"))]
struct MatrixProcessor(MatrixBuiltins);
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for MatrixProcessor {
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        SEAM_SIDE_WITNESS
    }
}

/// The input trim/polarity stage of one track that live controls drive (#210 phase 3).
///
/// The per-node sibling of [`BuiltinBankProcessor`]'s drain, for the tracks a strip bank did not
/// claim -- a host on a scalar backend, or a plan whose planner emitted no input bank. It is a
/// separate type from [`InputProcessor`] for the reason [`LiveControlMatrixProcessor`] gives: a
/// session prepared without live controls keeps `InputProcessor` and therefore keeps its exact
/// processor storage and its exact rendered bits.
///
/// # The witness, and why this one has to carry it
///
/// The input section is upstream of the fader/matrix seam, so an admitted per-lane record clears
/// the `LIVE` term. A per-node processor is not a bank slot and the collapse dispatch never asks
/// it anything -- `GraphRuntimeProcessor::channel_symmetry` is the scalar-tail surface, read by
/// the prepare-time pool classification -- but the term is folded anyway, by the same
/// [`InputReader::apply_latest`] the banked drain makes, so the two shapes cannot disagree about what a
/// value meant. If a later phase gives the scalar tail a collapse, the answer is already correct
/// here.
#[cfg(any(test, feature = "test-support"))]
struct LiveControlInputProcessor {
    input: InputBuiltins,
    control: InputReader,
    /// This track's live channel-symmetry terms, retained across blocks exactly as
    /// `EffectControlLane::symmetry` is and for the same reason.
    live: ChannelSymmetryWitness,
}
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for LiveControlInputProcessor {
    fn retained_bytes(&self) -> u64 {
        self.control.retained_bytes()
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.drain_controls(block.live_snapshot)?;
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.input.process(block);
        Ok(())
    }
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        self.input.channel_symmetry().and(self.live)
    }
    fn copy_response_snapshot(
        &self,
        sample_rate_hz: u32,
        request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        self.input.copy_response_snapshot(sample_rate_hz, request)
    }
    fn response_snapshot_declared(&self) -> bool {
        true
    }
    fn response_snapshot_native_id(&self) -> Option<&'static str> {
        Some("miso.builtin.input-filters")
    }
}
#[cfg(any(test, feature = "test-support"))]
impl LiveControlInputProcessor {
    /// The input cells' drain under the block's `snapshot` (#1346 D2, D3).
    fn drain_controls(&mut self, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        let input = &mut self.input;
        self.control
            .apply_latest(snapshot, &mut self.live, |record| match record {
                TrackInputRecord::TrimDb {
                    lanes,
                    db,
                    smoothing_samples,
                } => input.set_trim_db(lanes, db, smoothing_samples),
                TrackInputRecord::PolarityInvert {
                    lanes,
                    inverted,
                    smoothing_samples,
                } => {
                    input.set_polarity_invert(lanes, inverted, smoothing_samples);
                    Ok(())
                }
                TrackInputRecord::PreparedFilter { target } => input.apply_prepared_filter(target),
            })
            .map(|_| ())
            .map_err(render_error)
    }
}

/// The matrix/pan stage of one track that live controls drive (issue #137 D1).
///
/// It is a separate type from [`MatrixProcessor`] on purpose: a session prepared without live
/// controls keeps the exact processor storage, and therefore the exact
/// `engine_owned_processor_payload_bytes` row, it had before this channel existed. "Metering and
/// control off cost nothing" is a byte identity here, not a figure of speech.
///
/// The drain runs at the top of the block, before any audio is touched, so an admitted retarget
/// takes effect at exactly the block boundary the control side was told it would: every sample of
/// the block starting at `block.first_sample` is rendered by the post-command ramp. `try_pop`
/// moves one `Copy` record and `set_target_smoothed` performs four divisions; neither allocates,
/// locks, nor drops, which is what keeps the shipped artifact's render call-graph gate green.
#[cfg(any(test, feature = "test-support"))]
struct LiveControlMatrixProcessor {
    matrix: MatrixBuiltins,
    control: MatrixReader,
}
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for LiveControlMatrixProcessor {
    fn retained_bytes(&self) -> u64 {
        self.control.retained_bytes()
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.drain_controls(block.live_snapshot)?;
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.matrix.process(block);
        Ok(())
    }
}
#[cfg(any(test, feature = "test-support"))]
impl LiveControlMatrixProcessor {
    /// The matrix cell's drain under the block's `snapshot` (#1312 D12).
    fn drain_controls(&mut self, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        let matrix = &mut self.matrix;
        self.control
            .apply_latest(snapshot, |target, smoothing_samples| {
                #[cfg(any(test, feature = "test-support"))]
                FADER_MATRIX_LIVE_WITNESS.with(|value| {
                    let mut counters = value.get();
                    counters[1] = counters[1].saturating_add(1);
                    value.set(counters);
                });
                matrix.set_target_smoothed(target, smoothing_samples)
            })
            .map(|_| ())
            .map_err(render_error)
    }

    /// Apply every pending value, whatever its revision (#1312 D6): the block drain with
    /// [`LiveSnapshot::ALL`].
    #[allow(dead_code, reason = "called by the swap carry of #1277")]
    fn apply_pending(&mut self) -> Result<(), RenderError> {
        self.drain_controls(LiveSnapshot::ALL)
    }
}
/// The fader/mute stage of one track that live controls drive (issue #140 B).
///
/// The same shape as [`LiveControlMatrixProcessor`], for the same reason: a session prepared
/// without live controls keeps `FaderProcessor` and therefore keeps its exact processor storage and
/// its exact rendered bits. The drain runs at the top of the block, before any audio is touched, so
/// an admitted fader move or mute takes effect at exactly the block boundary the control side was
/// acknowledged with.
#[cfg(any(test, feature = "test-support"))]
struct LiveControlFaderProcessor {
    fader: FaderMuteRampBuiltins,
    control: FaderReader,
    /// The strip's stored fader automation (#1646 D1, R17-4): the bank's event driver, per strip.
    automation: Option<Box<LaneFaderAutomation>>,
}
#[cfg(any(test, feature = "test-support"))]
impl GraphRuntimeProcessor for LiveControlFaderProcessor {
    fn retained_bytes(&self) -> u64 {
        self.control.retained_bytes() + self.automation.as_deref().map_or(0, boxed_automation_bytes)
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.drain_controls(block.live_snapshot)?;
        render_oracle_fader(
            &mut self.fader,
            self.automation.as_deref_mut(),
            block.left,
            block.right,
            block.first_sample,
        )?;
        Ok(())
    }
    fn set_input_arrival(&mut self, arrival: u64) {
        if let Some(automation) = self.automation.as_deref_mut() {
            automation.arrival = arrival;
        }
    }
    fn set_block_time(&mut self, time: &GraphBlockTime<'_>) {
        if let Some(automation) = self.automation.as_deref_mut() {
            automation.take_block_time(time, block_jumps(time));
        }
    }
}
#[cfg(any(test, feature = "test-support"))]
impl LiveControlFaderProcessor {
    /// The fader and mute cells' drain under the block's `snapshot` (#1312 D12).
    fn drain_controls(&mut self, snapshot: LiveSnapshot) -> Result<(), RenderError> {
        let fader = &mut self.fader;
        let mut slot = self.automation.as_deref_mut();
        let result = self
            .control
            .apply_latest(snapshot, |kind, lanes, words| {
                #[cfg(any(test, feature = "test-support"))]
                FADER_MATRIX_LIVE_WITNESS.with(|value| {
                    let mut counters = value.get();
                    counters[0] = counters[0].saturating_add(1);
                    value.set(counters);
                });
                match kind {
                    FaderKind::Fader => {
                        fader.set_fader_db(lanes, f32::from_bits(words[0]), words[1])
                    }
                    // #1651 D1: an automated mute channel's value is its lane's `terms`.
                    FaderKind::Mute => match slot.as_deref_mut() {
                        Some(slot) => {
                            apply_mute_value(slot, &mut *fader, lanes, words, |fader, selector| {
                                fader.set_mute(selector, words[0] != 0, words[1]);
                                Ok(())
                            })
                        }
                        None => {
                            fader.set_mute(lanes, words[0] != 0, words[1]);
                            Ok(())
                        }
                    },
                }
            })
            .map(|_| ())
            .map_err(render_error);
        if let Some(automation) = self.automation.as_deref_mut() {
            // #1649 D3: the offsets cells, after the drain, as the fader bank reads them.
            automation.read_offsets(snapshot, Some(&self.control.superseded), &mut self.fader);
        }
        result
    }

    /// Apply every pending value, whatever its revision (#1312 D6): the block drain with
    /// [`LiveSnapshot::ALL`].
    #[allow(dead_code, reason = "called by the swap carry of #1277")]
    fn apply_pending(&mut self) -> Result<(), RenderError> {
        self.drain_controls(LiveSnapshot::ALL)
    }
}

struct MeterObserver(MeterAccumulator);
impl GraphRuntimeObserver for MeterObserver {
    fn retained_bytes(&self) -> u64 {
        self.0.retained_bytes()
    }

    /// Issue #943: a meter that selects exactly `SAMPLE_PEAK` merges the bank's block peak instead
    /// of reading its samples, so it asks the bank to compute one. Any other selection still needs
    /// the samples and never makes a bank run the pass.
    fn accepts_sample_peak(&self) -> bool {
        self.0.metrics() == builtins::MeterMetricSet::SAMPLE_PEAK
    }

    /// Issues #950/#951: a meter asks the bank for its full meter pass, including the separately
    /// seeded held recurrence, unless it selects exactly `SAMPLE_PEAK`, which
    /// issue #943's cheaper peak pass already serves.
    fn accepts_banked_meter(&self) -> bool {
        self.0.banked_eligible() && self.0.metrics() != builtins::MeterMetricSet::SAMPLE_PEAK
    }

    /// Issue #950: the energy this meter's own loop would start the block from, read without
    /// changing anything.
    fn banked_meter_seed(&self, first_sample: u64, frames: u32) -> Option<[f64; 2]> {
        self.0
            .banked_seed(first_sample, usize::try_from(frames).ok()?)
    }

    fn banked_held_seed(
        &self,
        first_sample: u64,
        frames: u32,
    ) -> Option<[lane::kernels::builtins::MeterHeldSeed; 2]> {
        self.0
            .banked_held_seed(first_sample, usize::try_from(frames).ok()?)
    }

    /// A meter's window, cumulative counters and producer are state a swap carries (#1327 D1).
    fn carry_kind(&self) -> Option<engine::realtime::ObserverKind> {
        Some(engine::realtime::ObserverKind::Meter)
    }

    /// The state moved into this binding is named by this binding's handle, as the snapshots it
    /// publishes show (#1327 D2). Binding handles of meters are meter handles, never zero.
    fn adopt_binding_handle(&mut self, handle: u64) {
        if let Some(handle) = core::num::NonZeroU64::new(handle) {
            self.0.rebind_handle(MeterHandle(handle));
        }
    }

    fn window_position(&self) -> Option<graph::ObserverWindow> {
        let position = self.0.window_position();
        Some(graph::ObserverWindow {
            start: position.start,
            frames: position.frames,
            sequence: position.sequence,
        })
    }

    fn join_window(&mut self, position: graph::ObserverWindow) -> bool {
        self.0.join_window(builtins::MeterWindowPosition {
            start: position.start,
            frames: position.frames,
            sequence: position.sequence,
        })
    }

    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.0
            .observe(block.left, block.right, block.first_sample)
            .map_err(|error| match error {
                builtins::MeterObservationError::SampleTimeOverflow => RenderError::TimeOverflow,
                builtins::MeterObservationError::LaneLength => RenderError::InvalidEnvelope,
            })
    }

    fn observe_resident(
        &mut self,
        block: graph::GraphResidentObservationBlock<'_>,
    ) -> Option<Result<(), RenderError>> {
        // Issue #950: the full pass is this meter's only if it accepted it. A `SAMPLE_PEAK` meter
        // on a unit whose full pass ran merges that pass's peak through issue #943's call.
        let banked = block
            .meter
            .filter(|_| self.accepts_banked_meter())
            .map(|meter| builtins::MeterBankedBlock {
                sample_peak: meter.sample_peak,
                clipped: meter.clipped.map(u64::from),
                sanitized: meter.sanitized.map(u64::from),
                energy_seed: meter.energy_seed,
                energy: meter.energy,
                held: meter.held,
            });
        Some(
            builtins::MeterInput::strided(
                block.lane.left(),
                block.lane.right(),
                block.lane.frames() as usize,
                block.lane.width().lanes() as usize,
                block.lane.lane(),
            )
            .and_then(|input| match banked {
                Some(banked) => {
                    self.0
                        .observe_input_banked(input, block.first_sample, Some(banked))
                }
                None => self.0.observe_input_with_block_peak(
                    input,
                    block.first_sample,
                    block.sample_peak,
                ),
            })
            .map_err(|error| match error {
                builtins::MeterObservationError::SampleTimeOverflow => RenderError::TimeOverflow,
                builtins::MeterObservationError::LaneLength => RenderError::InvalidEnvelope,
            }),
        )
    }
}

fn render_error(error: BuiltinParameterError) -> RenderError {
    match error {
        BuiltinParameterError::SampleTimeOverflow => RenderError::TimeOverflow,
        _ => RenderError::InvalidEnvelope,
    }
}

/// The strip's own `[left, right]` fader dB and mutes, as the session declares them. The domain
/// preflight checks these (#1242 D2), so a VCA's clamp never hides an out-of-domain member fader.
fn own_fader(strip: &StripRef<'_>) -> ([f32; 2], [bool; 2]) {
    (
        [strip.fader.left_db, strip.fader.right_db],
        [strip.fader.left_mute, strip.fader.right_mute],
    )
}

/// Lowers one strip's declared pan or matrix to the 2x2 target the matrix stage holds, and its
/// smoothing window.
///
/// The one matrix-domain authority between the session and the stage (issue #1255 D3):
/// preparation calls it from `strip_parameters`, and host-core's live-delta classifier calls it to
/// decide whether a committed pan or matrix is live and what record it becomes. A pan lowers
/// through [`pan_matrix`] and a matrix through [`Matrix2x2::checked`], so the result is exactly the
/// target preparation bakes, bit for bit. The caller's `maximum_smoothing` cap is not checked here;
/// it is a preparation capacity, not a domain.
///
/// # Errors
///
/// [`BuiltinParameterError::MatrixCoefficient`] when a pan position or a coefficient is outside
/// `[-1, 1]` or is not finite.
pub fn lower_matrix_or_pan(value: &MatrixOrPan) -> Result<(Matrix2x2, u32), BuiltinParameterError> {
    match *value {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => Ok((pan_matrix(left, right)?, smoothing_samples)),
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => Ok((Matrix2x2 { ll, lr, rl, rr }.checked()?, smoothing_samples)),
    }
}

/// The strip's lowered parameters, baking `fader_db` and `muted` (`[left, right]`) as its fader:
/// the strip's own values for the domain preflight, its effective values (`controls::effective`,
/// #1242 D2, decision 17 C7) for lowering and the tail seal.
fn strip_parameters(
    strip: &StripRef<'_>,
    (fader_db, muted): ([f32; 2], [bool; 2]),
    maximum_smoothing: u32,
) -> Result<BuiltinParameters, BuiltinParameterError> {
    let left = ChannelParameters {
        polarity_invert: strip.builtins.left.polarity_invert,
        trim_db: strip.builtins.left.trim_db,
        hpf_hz: strip.builtins.left.hpf_hz,
        lpf_hz: strip.builtins.left.lpf_hz,
        fader_db: fader_db[0],
        muted: muted[0],
    };
    let right = ChannelParameters {
        polarity_invert: strip.builtins.right.polarity_invert,
        trim_db: strip.builtins.right.trim_db,
        hpf_hz: strip.builtins.right.hpf_hz,
        lpf_hz: strip.builtins.right.lpf_hz,
        fader_db: fader_db[1],
        muted: muted[1],
    };
    let (matrix, smoothing_samples) = lower_matrix_or_pan(strip.matrix_or_pan)?;
    if smoothing_samples > maximum_smoothing {
        return Err(BuiltinParameterError::MatrixSmoothing);
    }
    Ok(BuiltinParameters {
        left,
        right,
        matrix,
        smoothing_samples,
    })
}

fn stage(tap: MeterTap) -> TrackStage {
    match tap {
        MeterTap::Input => TrackStage::Input,
        MeterTap::PostInputBuiltins => TrackStage::PostInputBuiltins,
        MeterTap::PostSimd1 => TrackStage::PostSimd1,
        MeterTap::PostDynamic => TrackStage::PostDynamic,
        MeterTap::PostSimd2PreFader => TrackStage::PostSimd2PreFader,
        MeterTap::PostFader => TrackStage::PostFader,
        MeterTap::PostMatrix => TrackStage::PostMatrix,
    }
}
fn stage_node(track_id: StableGraphId, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage { track_id, stage }
}
fn diag(code: &'static str, path: &str) -> BuiltinDiagnostic {
    BuiltinDiagnostic {
        code,
        path: path.to_owned(),
    }
}
fn meter_path(request: &MeterRequest) -> String {
    format!(
        "$.meters[track_id={},tap={:?}]",
        request.track_id, request.tap
    )
}
fn parameter_diagnostic(
    strip: &StripRef<'_>,
    error: BuiltinParameterError,
    sample_rate: u32,
) -> BuiltinDiagnostic {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        _ => "builtin.resource.arithmetic_overflow",
    };
    let track_path = strip.path_prefix();
    let path = match error {
        BuiltinParameterError::GainDomain => gain_path(strip, &track_path),
        BuiltinParameterError::FilterCutoff => cutoff_path(strip, &track_path, sample_rate),
        BuiltinParameterError::FilterOrder => filter_order_path(strip, &track_path),
        BuiltinParameterError::MatrixCoefficient => matrix_path(strip, &track_path),
        BuiltinParameterError::MatrixSmoothing => {
            format!("{track_path}.matrix_or_pan.smoothing_samples")
        }
        _ => format!("{track_path}.builtins"),
    };
    diag(code, &path)
}

fn gain_path(strip: &StripRef<'_>, track_path: &str) -> String {
    for (lane, builtins, fader) in [
        ("left", &strip.builtins.left, strip.fader.left_db),
        ("right", &strip.builtins.right, strip.fader.right_db),
    ] {
        if builtins::checked_trim_gain(builtins.trim_db).is_err() {
            return format!("{track_path}.builtins.{lane}.trim_db");
        }
        if builtins::checked_fader_gain(fader).is_err() {
            return format!("{track_path}.fader.{lane}_db");
        }
    }
    format!("{track_path}.builtins")
}

fn cutoff_path(strip: &StripRef<'_>, track_path: &str, sample_rate: u32) -> String {
    for (lane, builtins) in [
        ("left", &strip.builtins.left),
        ("right", &strip.builtins.right),
    ] {
        if invalid_cutoff(builtins.hpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.hpf_hz");
        }
        if invalid_cutoff(builtins.lpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.lpf_hz");
        }
    }
    format!("{track_path}.builtins")
}

fn filter_order_path(strip: &StripRef<'_>, track_path: &str) -> String {
    if strip.builtins.left.hpf_hz > 0.0
        && strip.builtins.left.lpf_hz > 0.0
        && strip.builtins.left.hpf_hz >= strip.builtins.left.lpf_hz
    {
        format!("{track_path}.builtins.left.lpf_hz")
    } else {
        format!("{track_path}.builtins.right.lpf_hz")
    }
}

fn invalid_cutoff(value: f32, sample_rate: u32) -> bool {
    validate_builtin_filter_cutoff(value, sample_rate, 0.0, 10.0).is_err()
}

fn matrix_path(strip: &StripRef<'_>, track_path: &str) -> String {
    match *strip.matrix_or_pan {
        MatrixOrPan::Pan { left, .. } if !left.is_finite() || !(-1.0..=1.0).contains(&left) => {
            format!("{track_path}.matrix_or_pan.left")
        }
        MatrixOrPan::Pan { .. } => format!("{track_path}.matrix_or_pan.right"),
        MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
            for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
                    return format!("{track_path}.matrix_or_pan.{field}");
                }
            }
            format!("{track_path}.matrix_or_pan")
        }
    }
}
fn meter_diagnostic(request: &MeterRequest, error: MeterConfigError) -> BuiltinDiagnostic {
    diag(
        match error {
            MeterConfigError::DecayDomain => "builtin.meter.config",
            MeterConfigError::Metrics => "builtin.meter.metrics",
            MeterConfigError::Queue => "builtin.resource.arithmetic_overflow",
        },
        &meter_path(request),
    )
}

/// The track count of [`test_only_prepared_unpaired_graph`]'s own-width graph: one full bank and a
/// tail.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub const TEST_ONLY_PAIR_GRAPH_TRACKS: usize = Backend::current().width() + 1;

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use tests::{
    test_only_builtin_lane_swap, test_only_prepared_unpaired_graph,
    test_only_prepared_unpaired_graph_with_held_meter,
};

#[cfg(any(test, feature = "test-support"))]
#[cfg_attr(not(test), allow(dead_code))]
mod tests {
    use super::*;
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use core::sync::atomic::{AtomicUsize, Ordering};
    #[cfg(test)]
    use engine::realtime::LiveSnapshot;
    use engine::{QuantumFrames, SampleRateHz};
    use graph::{
        GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding, GraphPortId, GraphPortKind,
        GraphPreparedSourceSetDriver, GraphResourceEstimate, GraphSourceInputClaim,
        GraphSourceSetResourceReport, PreparedGraphPlanParts, PreparedRoute, RouteGate,
        RouteTransform,
    };

    /// The compiler always emits `spec.nodes` sorted by id; hand-built fixtures list them in
    /// reading order, so they sort here (`program::lower` interns ids by binary search).
    fn sorted_nodes(mut nodes: Vec<GraphNode>) -> Vec<GraphNode> {
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        nodes
    }
    use session::{CompileCaps, compile_session, parse_session_json};
    use std::sync::Arc;

    static PAIR_WITNESS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn session() -> CompiledSession {
        let document = include_str!("../../../fixtures/session/v1/canonical.json");
        compile_session(
            &parse_session_json(document).expect("parse"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile")
    }
    /// The effective values of `session` with no solo: what a host without a monitor overlay passes.
    fn effective_of(session: &CompiledSession) -> controls::Effective {
        controls::effective(
            session.normalized_model(),
            &controls::MonitorOverlay::default(),
        )
    }

    fn caps() -> BuiltinCompileCaps {
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        }
    }

    struct SourceSetDriver {
        claim_count: usize,
        marker: u64,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for SourceSetDriver {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphPreparedSourceSetDriver for SourceSetDriver {
        fn retained_bytes(&self) -> u64 {
            effect_contract::retained::arc(&self.drops)
        }

        fn claim_count(&self) -> usize {
            self.claim_count
        }

        fn begin_block(&mut self, _first_sample: u64, _frames: u32) -> Result<(), RenderError> {
            Ok(())
        }

        fn copy_track_input(
            &mut self,
            _claim_index: usize,
            left: &mut [f32],
            right: &mut [f32],
        ) -> Result<(), RenderError> {
            left.fill(0.0);
            right.fill(0.0);
            Ok(())
        }

        fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
            if let Some(first) = output.first_mut() {
                *first = self.marker;
                1
            } else {
                0
            }
        }
    }

    struct DropProcessor(Arc<AtomicUsize>);

    impl Drop for DropProcessor {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphRuntimeProcessor for DropProcessor {
        fn retained_bytes(&self) -> u64 {
            effect_contract::retained::arc(&self.0)
        }

        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct NoopObserver;

    impl GraphRuntimeObserver for NoopObserver {
        fn retained_bytes(&self) -> u64 {
            0
        }

        fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct SourceBindFixture {
        artifact: PreparedBuiltinsGraphArtifact<u64>,
        bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
        input: GraphNodeId,
        builtin: GraphNodeId,
        output: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    struct SourceBindOwnership {
        input: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    impl SourceBindFixture {
        fn ownership(&self) -> SourceBindOwnership {
            SourceBindOwnership {
                input: self.input.clone(),
                builtin_drops: Arc::clone(&self.builtin_drops),
                external_drops: Arc::clone(&self.external_drops),
                source_drops: Arc::clone(&self.source_drops),
            }
        }
    }

    fn zero_graph_estimate() -> GraphResourceEstimate {
        GraphResourceEstimate {
            logical_nodes: 0,
            materialized_nodes: 0,
            edges: 0,
            schedule_items: 0,
            dependency_levels: 0,
            reductions: 0,
            routes: 0,
            effects: 0,
            audio_buffer_samples: 0,
            total_delay_samples: 0,
            delay_bytes: 0,
            graph_metadata_bytes: 0,
            declared_effect_bytes: 0,
            effect_bank_count: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        }
    }

    fn source_bind_fixture() -> SourceBindFixture {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let track_id = StableGraphId::parse("source-track").expect("test ID");
        let input = GraphNodeId::TrackStage {
            track_id: track_id.clone(),
            stage: TrackStage::Input,
        };
        let builtin = GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::PostInputBuiltins,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-output").expect("test ID"),
        };
        let edge = |source: GraphNodeId, target: GraphNodeId| GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: target.clone(),
            },
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: target,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.source-bind-test".to_owned(),
        };
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 22,
            spec: graph::GraphSpec {
                nodes: sorted_nodes(
                    [input.clone(), builtin.clone(), output.clone()]
                        .into_iter()
                        .map(|id| GraphNode {
                            id,
                            latency: effect_contract::LatencySamples(0),
                            bound: effect_contract::NodeTailBound::ZERO,
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges: vec![
                    edge(input.clone(), builtin.clone()),
                    edge(builtin.clone(), output.clone()),
                ],
            },
            sequential_schedule: vec![input.clone(), builtin.clone(), output.clone()],
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![builtin.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: vec![input.clone(), builtin.clone(), output.clone()],
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
            effect_observations: Vec::new(),
        });
        let builtin_drops = Arc::new(AtomicUsize::new(0));
        let external_drops = Arc::new(AtomicUsize::new(0));
        let source_drops = Arc::new(AtomicUsize::new(0));
        let builtin_observer =
            GraphNodeObserverBinding::new(builtin.clone(), 0x22_73, Box::new(NoopObserver));
        let artifact = PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: vec![GraphNodeBinding::new(
                builtin.clone(),
                Box::new(DropProcessor(Arc::clone(&builtin_drops))),
            )],
            builtin_observers: vec![builtin_observer],
            report: 0x22_73,
            track_controls: Vec::new(),
            meter_consumers: Vec::new(),
            automation_program: None,
        };
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![GraphNodeBinding::new(
                output.clone(),
                Box::new(DropProcessor(Arc::clone(&external_drops))),
            )],
            observers: Vec::new(),
        };
        let source_set = GraphPreparedSourceSet::new(
            envelope,
            vec![GraphSourceInputClaim {
                node: input.clone(),
            }],
            GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(SourceSetDriver {
                claim_count: 1,
                marker: 0x22_73,
                drops: Arc::clone(&source_drops),
            }),
        );
        SourceBindFixture {
            artifact,
            bindings,
            source_set,
            input,
            builtin,
            output,
            builtin_drops,
            external_drops,
            source_drops,
        }
    }

    fn assert_source_bind_failure_ownership(
        failure: &PreparedBuiltinsGraphSourceBindFailure<u64>,
        ownership: &SourceBindOwnership,
        expected_binding_nodes: &[GraphNodeId],
    ) {
        assert_eq!(*failure.artifact.report(), 0x22_73);
        assert_eq!(
            failure
                .bindings
                .nodes
                .iter()
                .map(|binding| binding.node.clone())
                .collect::<Vec<_>>(),
            expected_binding_nodes
        );
        assert_eq!(
            failure.source_set.claims(),
            [GraphSourceInputClaim {
                node: ownership.input.clone()
            }]
        );
        let mut telemetry = [0];
        assert_eq!(
            failure
                .source_set
                .copy_after_disarm_telemetry(&mut telemetry),
            1
        );
        assert_eq!(telemetry, [0x22_73]);
        assert_eq!(ownership.builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.source_drops.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn source_set_bind_succeeds_with_private_builtin_and_external_ownership() {
        let fixture = source_bind_fixture();
        let SourceBindFixture {
            artifact,
            bindings,
            source_set,
            builtin_drops,
            external_drops,
            source_drops,
            ..
        } = fixture;
        let bound = artifact
            .into_bound_with_source_set(bindings, source_set)
            .unwrap_or_else(|failure| panic!("source-set bind rejected: {}", failure.code));
        assert!(bound.meter_consumers.is_empty());
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(source_drops.load(Ordering::SeqCst), 0);
        drop(bound);
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 1);
        assert_eq!(external_drops.load(Ordering::SeqCst), 1);
        assert_eq!(source_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn source_set_bind_prevalidation_returns_all_ownership_for_each_code() {
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.envelope.quantum = QuantumFrames(8);
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("envelope mismatch must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.envelope_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.push(GraphNodeBinding::new(
                fixture.output.clone(),
                Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
            ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone(), fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("duplicate external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes[0].node = fixture.builtin.clone();
            let ownership = fixture.ownership();
            let expected = [fixture.builtin.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("external builtin overlap must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.clear();
            fixture.external_drops.store(0, Ordering::SeqCst);
            let ownership = fixture.ownership();
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("missing external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "source.graph.binding_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &[]);
        }
        {
            let mut fixture = source_bind_fixture();
            let invalid_node = GraphNodeId::CompensationDelay {
                edge_id: Box::new(GraphEdgeId::TrackMain {
                    target: fixture.output.clone(),
                }),
            };
            fixture
                .bindings
                .observers
                .push(GraphNodeObserverBinding::new(
                    invalid_node.clone(),
                    1,
                    Box::new(NoopObserver),
                ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("invalid observer node must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.observer");
            assert_eq!(failure.bindings.observers.len(), 1);
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
    }

    #[test]
    fn delegated_source_rejection_restores_private_and_external_ownership() {
        let mut fixture = source_bind_fixture();
        fixture.bindings.nodes.push(GraphNodeBinding::new(
            fixture.input.clone(),
            Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
        ));
        let ownership = fixture.ownership();
        let expected = [fixture.output.clone(), fixture.input.clone()];
        let expected_builtin = fixture.builtin.clone();
        let failure = match fixture
            .artifact
            .into_bound_with_source_set(fixture.bindings, fixture.source_set)
        {
            Ok(_) => panic!("source/external overlap must reject in graph bind"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "source.graph.binding_mismatch");
        assert_eq!(failure.artifact.builtin_processors.len(), 1);
        assert_eq!(
            failure.artifact.builtin_processors[0].node,
            expected_builtin
        );
        assert_eq!(failure.artifact.builtin_observers.len(), 1);
        assert_eq!(failure.artifact.builtin_observers[0].handle, 0x22_73);
        assert_source_bind_failure_ownership(&failure, &ownership, &expected);
    }

    #[test]
    fn builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back() {
        let inputs: Vec<Box<str>> = (0..17)
            .map(|index| Box::<str>::from(format!("bank{index}")))
            .collect();
        let node = |index| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let levels = vec![
            DependencyLevel {
                level: 0,
                nodes: vec![node(0)],
            },
            DependencyLevel {
                level: 1,
                nodes: (1..10).map(node).collect(),
            },
            DependencyLevel {
                level: 2,
                nodes: (10..17).map(node).collect(),
            },
        ];
        // The 1 / 9 / 7 layout, banked per level with the last bank of each level padded:
        // W4 -> 1 | 4 4 1 | 4 3 and W8 -> 1 | 8 1 | 7.  Hand counts of `n.div_ceil(W)` banks per
        // level; every post-input node is a member, so no level contributes a scalar tail.
        for (dispatch, expected_sizes) in [
            (Backend::Simd4, &[1, 4, 4, 1, 4, 3][..]),
            #[cfg(target_feature = "avx2")]
            (Backend::Simd8, &[1, 8, 1, 7][..]),
            // D4: the scalar backend has no bank width at all -- one arithmetic graph everywhere
            // means fusion is written, not inferred, so those tracks stay on the scalar `Lane`.
            (Backend::Scalar, &[][..]),
        ] {
            let groups = planned_builtin_bank_members(
                &inputs,
                TrackStage::PostInputBuiltins,
                dispatch,
                &levels,
                &SessionPoolClasses::default(),
            );
            let sizes: Vec<_> = groups.iter().map(|members| members.len()).collect();
            assert_eq!(sizes, expected_sizes, "{:?}", dispatch);
            assert_eq!(
                sizes.iter().sum::<usize>(),
                if expected_sizes.is_empty() { 0 } else { 17 },
                "every post-input node is banked once"
            );
            let mut group_levels = Vec::new();
            assert!(groups.iter().all(|members| {
                let member_levels: BTreeSet<_> = members
                    .iter()
                    .map(|member| {
                        levels
                            .iter()
                            .find(|level| level.nodes.contains(member))
                            .expect("member level")
                            .level
                    })
                    .collect();
                group_levels.extend(member_levels.iter().copied());
                member_levels.len() == 1 && members.windows(2).all(|pair| pair[0] < pair[1])
            }));
            assert!(
                group_levels.windows(2).all(|pair| pair[0] <= pair[1]),
                "banks are emitted in dependency-level order"
            );
        }
        let scalar = Backend::Scalar;
        assert!(
            planned_builtin_bank_members(
                &inputs,
                TrackStage::PostInputBuiltins,
                scalar,
                &levels,
                &SessionPoolClasses::default(),
            )
            .is_empty()
        );
    }
    /// F4/F11: a bank is charged for the two main planes it actually owns, its chain's staging
    /// and (padded) zero plane (issue #887), and the member ids it actually holds -- a padded bank
    /// is not charged a full-width id array, and no bank is charged the two sidechain planes a
    /// fixed stage can never reach.
    #[test]
    fn builtin_bank_resource_charges_two_planes_and_actual_members() {
        let node = |index: usize| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let quantum = 64_u32;
        // A full and a padded bank at four lanes; a full-plus-one and a padded bank at eight.
        let cases = effect_contract::BankWidth::ALL.iter().flat_map(|&width| {
            if width.lanes() == 4 {
                vec![(width, &[4, 3][..])]
            } else {
                vec![(width, &[8, 1][..]), (width, &[5][..])]
            }
        });
        for (width, sizes) in cases {
            let mut next = 0_usize;
            let groups: Vec<Box<[GraphNodeId]>> = sizes
                .iter()
                .map(|size| {
                    (0..*size)
                        .map(|_| {
                            next += 1;
                            node(next - 1)
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect();
            let processor_bytes =
                strip_processor_bytes(TrackStage::PostInputBuiltins, width).expect("bankable");
            let resource = builtin_bank_resource(&groups, width, quantum, processor_bytes)
                .expect("padded layout is chargeable");

            // Hand formula, written from `size_of` rather than from the function under test.
            let lanes = u64::from(width.lanes());
            let banks = sizes.len() as u64;
            let node_bytes = core::mem::size_of::<GraphNodeId>() as u64;
            // The struct **plus** the per-lane live-control reader array it owns. #210 phase 3
            // gave the input bank the same shape the fader and matrix banks already had: one
            // `Option<InputReader>` per lane (a queue consumer until #1346), allocated whether or
            // not live controls are attached,
            // so that a banked session's retained payload does not depend on whether the host
            // leased one. Written out here rather than read off `strip_processor_bytes`, which is
            // the function under test.
            let processor_bytes = core::mem::size_of::<BuiltinBankProcessor>() as u64
                + core::mem::size_of::<Option<InputReader>>() as u64 * lanes;
            let plane_bytes = u64::from(quantum) * lanes * 4;
            let string_lengths: Vec<u64> = groups
                .iter()
                .flat_map(|members| members.iter())
                .map(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().len() as u64,
                    _ => unreachable!("member kind"),
                })
                .collect();
            let strings: u64 = string_lengths.iter().sum();
            let largest_string = string_lengths.iter().copied().max().expect("a member");
            assert_eq!(resource.bank_count, banks);
            // Scratch and staging, four planes per bank, plus the `+0.0` plane of each padded
            // bank (issue #887): exactly what one `rack::BankChain` over the bank retains.
            let padded = sizes.iter().filter(|size| (**size as u64) < lanes).count() as u64;
            assert_eq!(
                resource.scratch_samples,
                banks * u64::from(quantum) * lanes * 4 + padded * u64::from(quantum)
            );
            assert_eq!(
                resource.scratch_bytes,
                banks * plane_bytes * 4 + padded * u64::from(quantum) * 4
            );
            assert_eq!(
                resource.payload_bytes,
                sizes
                    .iter()
                    .map(|size| node_bytes * *size as u64 + processor_bytes)
                    .sum::<u64>()
                    + strings
            );
            assert_eq!(
                resource.metadata_bytes,
                banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64
            );
            assert_eq!(
                resource.largest_allocation_bytes,
                [
                    node_bytes * *sizes.iter().max().expect("a bank") as u64,
                    processor_bytes,
                    largest_string,
                    plane_bytes,
                    banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64,
                ]
                .into_iter()
                .max()
                .expect("a term")
            );
        }
        // Only an empty or oversized group is unchargeable now.
        assert!(
            builtin_bank_resource(
                &[Vec::new().into_boxed_slice()],
                effect_contract::BankWidth::Four,
                64,
                0
            )
            .is_none()
        );
        assert!(
            builtin_bank_resource(
                &[(0..5).map(node).collect::<Vec<_>>().into_boxed_slice()],
                effect_contract::BankWidth::Four,
                64,
                0
            )
            .is_none()
        );
    }

    // ---------------------------------------------------------------------------------------
    // #86 A7: the bit-identity harness.
    //
    // These tests render through the production path -- `prepare_session_builtins` ->
    // `into_graph_artifact_with_banks` -> `into_bound` -> `PreparedRenderPlan::render` -- and
    // compare `to_bits()` of the post-input-builtins output, per track, per channel, per block.
    // The oracle is the same generic kernel body at `L = f32` reached through `Scalar` dispatch,
    // which is independent of the vector wrapper, the AoSoA transposes and the padding.
    // ---------------------------------------------------------------------------------------

    /// One session of `n` tracks with deliberately distinct per-track builtins.
    fn n_track_session(n: usize) -> CompiledSession {
        n_track_session_with_symmetry(n, false)
    }

    fn n_track_session_with_symmetry(n: usize, symmetric_input: bool) -> CompiledSession {
        let mut model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("fixture parse");
        let mut template = model.tracks[0].clone();
        template.console.clear();
        template.inserts.effects.clear();
        model.automation.clear();
        model.tracks.clear();
        for index in 0..n {
            let mut track = template.clone();
            track.id = session::StableId::parse(&track_name(index)).expect("generated stable ID");
            let scale = index as f32;
            track.builtins.left.hpf_hz = 20.0 + 5.0 * scale;
            track.builtins.left.lpf_hz = 18_000.0 - 100.0 * scale;
            track.builtins.left.trim_db = -3.0 + 0.5 * scale;
            track.builtins.left.polarity_invert = index % 4 == 3;
            track.builtins.right.hpf_hz = if index % 2 == 0 {
                0.0
            } else {
                30.0 + 3.0 * scale
            };
            track.builtins.right.lpf_hz = if index % 3 == 0 {
                0.0
            } else {
                17_000.0 - 250.0 * scale
            };
            track.builtins.right.polarity_invert = index % 2 == 1;
            track.builtins.right.trim_db = 1.0 - 0.25 * scale;
            if symmetric_input {
                track.builtins.right = track.builtins.left.clone();
            }
            track.fader.left_db = -1.0 + 0.125 * scale;
            track.fader.right_db = 0.5 - 0.125 * scale;
            track.matrix_or_pan = session::MatrixOrPan::Matrix {
                ll: 0.75,
                lr: 0.25,
                rl: -0.125,
                rr: 0.625,
                smoothing_samples: 0,
            };
            model.tracks.push(track);
        }
        model.routes.truncate(1);
        model.routes[0].source = session::RouteSource::Track {
            track_id: session::StableId::parse(&track_name(0)).expect("route track"),
            tap: session::SendTap::PostPan,
        };
        compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("harness session compile")
    }

    fn track_name(index: usize) -> String {
        format!("t{index:02}")
    }

    const HARNESS_QUANTUM: u32 = 64;
    const HARNESS_BLOCKS: u64 = 3;

    /// A deterministic per-track input signal: an LCG seeded by `(track, first_sample)`.
    ///
    /// It is a plain `GraphRuntimeProcessor` bound at the `Input` stage, so every dispatch under
    /// test sees byte-identical input.
    struct SeededInput {
        seed: u64,
        symmetric: bool,
        nonfinite: bool,
    }

    thread_local! {
        static ALIAS_OBSERVATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        static ALIAS_CAPTURE: std::cell::RefCell<Vec<u32>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    struct AliasObserver;
    impl GraphRuntimeObserver for AliasObserver {
        fn retained_bytes(&self) -> u64 {
            0
        }

        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            ALIAS_OBSERVATIONS.with(|count| count.set(count.get() + 1));
            ALIAS_CAPTURE.with(|capture| {
                let mut capture = capture.borrow_mut();
                capture.extend(block.left.iter().map(|sample| sample.to_bits()));
                capture.extend(block.right.iter().map(|sample| sample.to_bits()));
            });
            Ok(())
        }
    }

    impl GraphRuntimeProcessor for SeededInput {
        fn retained_bytes(&self) -> u64 {
            0
        }

        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let mut state = self.seed ^ block.first_sample.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut next = || {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let unit = ((state >> 40) as f32) / ((1_u32 << 24) as f32);
                (unit * 2.0 - 1.0) * 0.8
            };
            for (index, (left, right)) in block
                .left
                .iter_mut()
                .zip(block.right.iter_mut())
                .enumerate()
            {
                *left = next();
                *right = if self.symmetric { *left } else { next() };
                if self.nonfinite && index == 0 {
                    *left = f32::NAN;
                    *right = f32::INFINITY;
                }
            }
            self.nonfinite = false;
            Ok(())
        }
    }

    /// Records `to_bits()` of both channels after its node completes.
    struct Capture(Arc<std::sync::Mutex<Vec<u32>>>);

    impl GraphRuntimeObserver for Capture {
        fn retained_bytes(&self) -> u64 {
            effect_contract::retained::arc(&self.0)
                + self
                    .0
                    .lock()
                    .map_or(0, |v| effect_contract::retained::vec(&v))
        }

        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let mut sink = self.0.lock().expect("harness capture");
            sink.extend(block.left.iter().map(|sample| sample.to_bits()));
            sink.extend(block.right.iter().map(|sample| sample.to_bits()));
            Ok(())
        }
    }

    /// Builds the five-level graph the harness renders: one level per track stage.
    ///
    /// `Output` is fed by track 0's `PostMatrix` only: `GraphEdgeId::TrackMain { target }` is not
    /// unique for fan-in, and a reduction is not what this harness measures.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum BoundaryVariant {
        Plain,
        SelectedSend,
        AliasObserved,
    }

    fn track_graph(n: usize) -> (PreparedGraphPlan, Vec<DependencyLevel>) {
        track_graph_variant(n, BoundaryVariant::Plain)
    }

    fn track_graph_variant(
        n: usize,
        variant: BoundaryVariant,
    ) -> (PreparedGraphPlan, Vec<DependencyLevel>) {
        track_graph_variant_with_route_transform(
            n,
            variant,
            RouteTransform {
                gain: 0.5,
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
        )
    }

    fn track_graph_variant_with_route_transform(
        n: usize,
        variant: BoundaryVariant,
        route_transform: RouteTransform,
    ) -> (PreparedGraphPlan, Vec<DependencyLevel>) {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(HARNESS_QUANTUM),
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let stage = |index: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
            stage,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-out").expect("harness ID"),
        };
        let route = GraphNodeId::Route {
            route_id: StableGraphId::parse("proof-send").expect("harness ID"),
        };
        // The synthetic alias and the selected send read the trailing track (issue #916: the
        // session Output is dedicated storage, so no track's post-matrix buffer is the graph
        // output).
        let alias_track = n - 1;
        let alias = stage(alias_track, TrackStage::PostDynamic);
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for index in 0..n {
            for pair in stages.windows(2) {
                let source = stage(index, pair[0]);
                let target = stage(index, pair[1]);
                edges.push(GraphEdge {
                    id: GraphEdgeId::TrackMain {
                        target: target.clone(),
                    },
                    source: GraphPortId {
                        node: source,
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: target,
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: format!("$.tracks[{index}].chain"),
                });
            }
        }
        let main_target = output.clone();
        // Issue #1420: every variant takes track 0; the `Output` is dedicated storage (#916).
        let output_track = 0;
        edges.push(GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: main_target.clone(),
            },
            source: GraphPortId {
                node: stage(output_track, TrackStage::PostMatrix),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: main_target,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.routes[0]".to_owned(),
        });
        if variant == BoundaryVariant::SelectedSend {
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("proof-send").expect("route"),
                },
                source: GraphPortId {
                    node: stage(alias_track, TrackStage::PostFader),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: route.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.routes[proof-send].source".to_owned(),
            });
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteDestination {
                    route_id: StableGraphId::parse("proof-send").expect("route"),
                },
                source: GraphPortId {
                    node: route.clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.routes[proof-send].destination".to_owned(),
            });
        }
        if variant == BoundaryVariant::AliasObserved {
            edges.push(GraphEdge {
                id: GraphEdgeId::TrackMain {
                    target: alias.clone(),
                },
                source: GraphPortId {
                    node: stage(alias_track, TrackStage::PostFader),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: alias.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: format!("$.tracks[{alias_track}].alias"),
            });
            let edge = edges.last().expect("alias edge");
            assert_eq!(
                edge.source.node,
                stage(n - 1, TrackStage::PostFader),
                "the proof alias reads the trailing track"
            );
            assert_eq!(
                edge.destination.node, alias,
                "the proof edge targets the alias"
            );
        }
        let mut levels = Vec::new();
        for (level, kind) in stages.iter().enumerate() {
            let level_nodes: Vec<_> = (0..n).map(|index| stage(index, *kind)).collect();
            nodes.extend(level_nodes.iter().cloned());
            levels.push(DependencyLevel {
                level: level as u64,
                nodes: level_nodes,
            });
        }
        nodes.push(output.clone());
        if variant == BoundaryVariant::SelectedSend {
            nodes.push(route.clone());
        }
        if variant == BoundaryVariant::AliasObserved {
            nodes.push(alias.clone());
        }
        levels.push(DependencyLevel {
            level: stages.len() as u64,
            nodes: vec![output.clone()],
        });
        if matches!(
            variant,
            BoundaryVariant::SelectedSend | BoundaryVariant::AliasObserved
        ) {
            let boundary_node = if variant == BoundaryVariant::SelectedSend {
                route.clone()
            } else {
                alias.clone()
            };
            levels.insert(
                stages.len(),
                DependencyLevel {
                    level: stages.len() as u64,
                    nodes: vec![boundary_node],
                },
            );
            for (index, level) in levels.iter_mut().enumerate() {
                level.level = index as u64;
            }
        }
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 8_600 + n as u64,
            spec: graph::GraphSpec {
                nodes: sorted_nodes(
                    nodes
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: effect_contract::LatencySamples(0),
                            bound: effect_contract::NodeTailBound::ZERO,
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels.clone(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: nodes
                .iter()
                .filter(|node| {
                    !matches!(node, GraphNodeId::Route { .. } | GraphNodeId::Effect(_))
                        && *node != &alias
                })
                .cloned()
                .collect(),
            routes: if variant == BoundaryVariant::SelectedSend {
                vec![PreparedRoute {
                    node: route,
                    transform: route_transform,
                    gate: RouteGate::OPEN,
                }]
            } else {
                Vec::new()
            },
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: if matches!(variant, BoundaryVariant::AliasObserved) {
                vec![GraphNodeObserverBinding::new(
                    alias,
                    0x86ff,
                    Box::new(AliasObserver),
                )]
            } else {
                Vec::new()
            },
            effect_observations: Vec::new(),
        });
        (graph, levels)
    }

    /// Renders `HARNESS_BLOCKS` blocks and returns post-matrix output bits per track.
    fn render_post_input_bits_with_variant(
        n: usize,
        dispatch: Backend,
        post_fader_barrier: bool,
        variant: BoundaryVariant,
    ) -> (Vec<Vec<u32>>, usize, Vec<MeterSnapshot>, Vec<u32>) {
        let compiled = n_track_session(n);
        let meter_requests = post_fader_barrier.then(|| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(0x430).expect("handle")),
            track_id: track_name(0),
            tap: MeterTap::PostFader,
            config: MeterConfig {
                period_frames: NonZeroU32::new(HARNESS_QUANTUM).expect("period"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: NonZeroUsize::new(4).expect("queue"),
                reset_generation: 0,
            },
        });
        let meter_requests = meter_requests.as_slice();
        let builtins =
            prepare_session_builtins(&compiled, meter_requests, caps()).expect("harness builtins");
        let (graph, levels) = track_graph_variant(n, variant);
        let classes = SessionPoolClasses::from_session(&compiled);
        let mut artifact =
            builtins.into_graph_artifact_with_banks(graph, (), dispatch, &levels, &classes);
        let bank_count = artifact.graph.prepared_builtin_bank_count();
        let captures: Vec<_> = (0..n)
            .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
            .collect();
        for (index, capture) in captures.iter().enumerate() {
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::PostMatrix,
                    },
                    0x8600 + index as u64,
                    Box::new(Capture(Arc::clone(capture))),
                ));
        }
        let envelope = artifact.graph.envelope;
        let mut nodes: Vec<_> = (0..n)
            .map(|index| {
                GraphNodeBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::Input,
                    },
                    Box::new(SeededInput {
                        seed: 0x5eed_0000 ^ index as u64,
                        symmetric: false,
                        nonfinite: false,
                    }) as Box<dyn GraphRuntimeProcessor>,
                )
            })
            .collect();
        nodes.push(GraphNodeBinding::new(
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main-out").expect("harness ID"),
            },
            Box::new(HarnessSink) as Box<dyn GraphRuntimeProcessor>,
        ));
        let mut bound = match artifact.into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound,
            Err(failure) => panic!("harness bind: {}", failure.code),
        };
        let mut plan = bound.plan;
        if matches!(variant, BoundaryVariant::AliasObserved) {
            ALIAS_CAPTURE.with(|capture| capture.borrow_mut().clear());
        }
        let frames = HARNESS_QUANTUM as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        for block in 0..HARNESS_BLOCKS {
            plan.render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(&mut pcm, 2, frames, frames)
                        .expect("harness output"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: block * HARNESS_QUANTUM as u64,
                },
            )
            .expect("harness render");
        }
        if post_fader_barrier {
            let meter = bound.meter_consumers.first_mut().expect("post-fader meter");
            let mut windows = Vec::new();
            while let Ok(snapshot) = meter.consumer.try_pop() {
                windows.push(snapshot);
            }
            assert!(
                !windows.is_empty(),
                "post-fader meter published nonempty windows"
            );
            let meter_snapshots = windows;
            let bits = captures
                .into_iter()
                .map(|capture| {
                    let taken = capture.lock().expect("harness capture").clone();
                    assert_eq!(taken.len(), frames * 2 * HARNESS_BLOCKS as usize);
                    taken
                })
                .collect();
            let output = pcm.iter().map(|sample| sample.to_bits()).collect();
            return (bits, bank_count, meter_snapshots, output);
        }
        let bits = captures
            .into_iter()
            .map(|capture| {
                let taken = capture.lock().expect("harness capture").clone();
                assert_eq!(taken.len(), frames * 2 * HARNESS_BLOCKS as usize);
                taken
            })
            .collect();
        let output = pcm.iter().map(|sample| sample.to_bits()).collect();
        (bits, bank_count, Vec::new(), output)
    }

    /// The live graph's own-width fixture (issue #1112): the build's vector backend, and one full
    /// bank of it plus a one-track tail -- `Simd8` and nine tracks on an 8-lane (AVX2) build,
    /// `Simd4` and five on a 4-lane (NEON/simd128) one.
    const PAIR_GRAPH_BACKEND: Backend = Backend::current();

    /// The live graph's one-lane tail track.
    fn pair_graph_tail_id() -> String {
        format!("t{:02}", TEST_ONLY_PAIR_GRAPH_TRACKS - 1)
    }

    /// The queued own-width graph with live controls, for the installed allocator's render-only
    /// audit: separate fader and matrix banks. Construction, binding and queue ownership stay off
    /// render.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_only_prepared_unpaired_graph() -> PreparedBuiltinsGraphBound {
        prepared_live_graph_fixture(
            false,
            false,
            false,
            None,
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
        )
    }

    /// The same full-cohort-and-tail fixture, with one final resident hold/decay meter, for the
    /// existing isolated full-render allocation owner (issue #951).
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_only_prepared_unpaired_graph_with_held_meter() -> PreparedBuiltinsGraphBound {
        prepared_live_graph_fixture_with_ballistics(
            Some(TEST_ONLY_PAIR_GRAPH_TRACKS - 1),
            false,
            false,
            None,
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
            (7, 12.0),
        )
    }

    /// A queued graph of `n` live-control tracks at the vector `backend`, prepared, lowered and
    /// bound: separate fader and matrix banks. `post_fader_observed` adds a post-fader meter on
    /// track 0; `post_matrix_capture` captures the last track's post-matrix words.
    fn prepared_live_graph_fixture(
        post_fader_observed: bool,
        symmetric_input: bool,
        nonfinite_input: bool,
        post_matrix_capture: Option<Arc<std::sync::Mutex<Vec<u32>>>>,
        backend: Backend,
        n: usize,
    ) -> PreparedBuiltinsGraphBound {
        prepared_live_graph_fixture_with_ballistics(
            post_fader_observed.then_some(0),
            symmetric_input,
            nonfinite_input,
            post_matrix_capture,
            backend,
            n,
            (0, 0.0),
        )
    }

    fn prepared_live_graph_fixture_with_ballistics(
        post_fader_track: Option<usize>,
        symmetric_input: bool,
        nonfinite_input: bool,
        post_matrix_capture: Option<Arc<std::sync::Mutex<Vec<u32>>>>,
        backend: Backend,
        n: usize,
        ballistics: (u32, f32),
    ) -> PreparedBuiltinsGraphBound {
        let compiled = n_track_session_with_symmetry(n, symmetric_input);
        let controls = (0..n)
            .map(|index| TrackControlRequest {
                track_id: track_name(index),
                superseded: LiveValuesSuperseded::default(),
                input_lane: true,
            })
            .collect::<Vec<_>>();
        let meter = post_fader_track.map(|track| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(0x459).expect("handle")),
            track_id: track_name(track),
            tap: MeterTap::PostFader,
            config: MeterConfig {
                period_frames: NonZeroU32::new(HARNESS_QUANTUM).expect("period"),
                peak_hold_frames: ballistics.0,
                peak_decay_db_per_second: ballistics.1,
                queue_capacity: NonZeroUsize::new(8).expect("queue"),
                reset_generation: 0,
            },
        });
        let builtins = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            meter.as_slice(),
            &controls,
            caps(),
        )
        .expect("prepared builtins");
        let (mut graph, levels) = track_graph(n);
        let classes = SessionPoolClasses::from_session(&compiled);
        let scalar_resource = builtins
            .graph_scalar_owner_resource(backend, &levels, &classes)
            .expect("checked scalar owner resource");
        graph
            .estimate
            .checked_add_scalar_owners(scalar_resource)
            .expect("fixture scalar owner estimate");
        let emitted_op_count = levels.iter().fold(0_u64, |total, level| {
            total
                .checked_add(u64::try_from(level.nodes.len()).expect("fixture level fits u64"))
                .expect("fixture runtime metadata count")
        });
        let runtime_resource =
            graph::GraphRuntimeMetadataResourceEstimate::checked_for(emitted_op_count)
                .expect("fixture runtime metadata estimate");
        graph
            .estimate
            .checked_add_runtime_metadata(runtime_resource)
            .expect("fixture runtime metadata estimate fold");
        let mut artifact =
            builtins.into_graph_artifact_with_banks(graph, (), backend, &levels, &classes);
        if let Some(capture) = post_matrix_capture {
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(n - 1)).expect("selected tail"),
                        stage: TrackStage::PostMatrix,
                    },
                    0x4598,
                    Box::new(Capture(capture)),
                ));
        }
        let envelope = artifact.graph.envelope;
        let mut nodes = (0..n)
            .map(|index| {
                GraphNodeBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("track"),
                        stage: TrackStage::Input,
                    },
                    Box::new(SeededInput {
                        seed: 0x4590_0000 ^ index as u64,
                        symmetric: symmetric_input,
                        nonfinite: nonfinite_input,
                    }) as Box<dyn GraphRuntimeProcessor>,
                )
            })
            .collect::<Vec<_>>();
        nodes.push(GraphNodeBinding::new(
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main-out").expect("output"),
            },
            Box::new(HarnessSink) as Box<dyn GraphRuntimeProcessor>,
        ));
        artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("fixture bind: {}", failure.code))
    }

    fn render_bound(bound: &mut PreparedBuiltinsGraphBound, sample: u64) -> Vec<u32> {
        let mut pcm = vec![0.0_f32; HARNESS_QUANTUM as usize * 2];
        render_bound_result(bound, sample, &mut pcm).expect("fixture render");
        pcm.into_iter().map(f32::to_bits).collect()
    }

    fn render_bound_result(
        bound: &mut PreparedBuiltinsGraphBound,
        sample: u64,
        pcm: &mut [f32],
    ) -> Result<(), RenderError> {
        bound
            .plan
            .render(
                engine::realtime::RenderIo {
                    output: engine::realtime::PlanarBufferMut::try_new(
                        pcm,
                        2,
                        HARNESS_QUANTUM as usize,
                        HARNESS_QUANTUM as usize,
                    )
                    .expect("output"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: sample,
                },
            )
            .map(|_| ())
    }

    /// A two-track graph whose strips carry fader-and-matrix channels only (`input_lane: false`,
    /// #1254 D1), prepared, lowered and bound at `backend`.
    fn input_free_pair_graph(backend: Backend) -> PreparedBuiltinsGraphBound {
        let n = 2;
        let compiled = n_track_session_with_symmetry(n, false);
        let controls = (0..n)
            .map(|index| TrackControlRequest {
                track_id: track_name(index),
                superseded: LiveValuesSuperseded::default(),
                input_lane: false,
            })
            .collect::<Vec<_>>();
        let builtins = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            &[],
            &controls,
            caps(),
        )
        .expect("prepared builtins");
        assert!(builtins.validate_for_session(&compiled).0.is_empty());
        let (mut graph, levels) = track_graph(n);
        let classes = SessionPoolClasses::from_session(&compiled);
        let scalar_resource = builtins
            .graph_scalar_owner_resource(backend, &levels, &classes)
            .expect("checked scalar owner resource");
        graph
            .estimate
            .checked_add_scalar_owners(scalar_resource)
            .expect("fixture scalar owner estimate");
        let emitted_op_count = levels.iter().map(|level| level.nodes.len() as u64).sum();
        graph
            .estimate
            .checked_add_runtime_metadata(
                graph::GraphRuntimeMetadataResourceEstimate::checked_for(emitted_op_count)
                    .expect("fixture runtime metadata estimate"),
            )
            .expect("fixture runtime metadata estimate fold");
        let artifact =
            builtins.into_graph_artifact_with_banks(graph, (), backend, &levels, &classes);
        let envelope = artifact.graph.envelope;
        let mut nodes = (0..n)
            .map(|index| {
                GraphNodeBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("track"),
                        stage: TrackStage::Input,
                    },
                    Box::new(SeededInput {
                        seed: 0x1254_0000 ^ index as u64,
                        symmetric: false,
                        nonfinite: false,
                    }) as Box<dyn GraphRuntimeProcessor>,
                )
            })
            .collect::<Vec<_>>();
        nodes.push(GraphNodeBinding::new(
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main-out").expect("output"),
            },
            Box::new(HarnessSink) as Box<dyn GraphRuntimeProcessor>,
        ));
        artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("fixture bind: {}", failure.code))
    }

    /// Issue #1254 gate 1(d): a fader-and-matrix strip prepares under the scalar oracle (which
    /// binds the plain input section) and under the native banks, and the two render the same
    /// live mute and matrix bit for bit.
    #[test]
    fn input_free_live_strips_match_between_the_scalar_oracle_and_the_banks() {
        let mut outputs = Vec::new();
        for backend in [Backend::Scalar, PAIR_GRAPH_BACKEND] {
            let mut bound = input_free_pair_graph(backend);
            assert_eq!(bound.track_controls.len(), 2);
            assert!(
                bound
                    .track_controls
                    .iter()
                    .all(|control| control.input.is_none())
            );
            bound.test_only_write(|controls, stamp| {
                controls[0].fader.write(
                    stamp,
                    TrackFaderRecord::Mute {
                        lanes: BuiltinLaneSelector::Left,
                        muted: true,
                        smoothing_samples: 0,
                    },
                )
            });
            bound.test_only_write(|controls, stamp| {
                controls[1].producer.write(
                    stamp,
                    TrackControlRecord {
                        matrix: Matrix2x2 {
                            ll: 0.5,
                            lr: 0.25,
                            rl: -0.5,
                            rr: 0.75,
                        },
                        smoothing_samples: 0,
                    },
                )
            });
            let mut bits = Vec::new();
            for block in 0..4_u64 {
                bits.extend(render_bound(&mut bound, block * HARNESS_QUANTUM as u64));
            }
            outputs.push(bits);
        }
        assert_eq!(outputs[0], outputs[1]);
        let mut quiet = input_free_pair_graph(PAIR_GRAPH_BACKEND);
        let quiet_bits: Vec<u32> = (0..4_u64)
            .flat_map(|block| render_bound(&mut quiet, block * HARNESS_QUANTUM as u64))
            .collect();
        assert_ne!(
            outputs[1], quiet_bits,
            "the records reached the rendered output"
        );
    }

    #[test]
    fn actual_graph_mono_collapse_disengages_on_input_command_and_recovers_nonfinite_input() {
        let _guard = PAIR_WITNESS_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let collapsed_capture = Arc::new(std::sync::Mutex::new(Vec::new()));
        let separate_capture = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut collapsed = prepared_live_graph_fixture(
            false,
            true,
            false,
            Some(Arc::clone(&collapsed_capture)),
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
        );
        let mut separate = prepared_live_graph_fixture(
            false,
            true,
            false,
            Some(Arc::clone(&separate_capture)),
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
        );
        collapsed.plan.arm_mono_collapse(&|_| true);
        separate.plan.force_mono_collapse_off(true);

        let collapsed_host = render_bound(&mut collapsed, 0);
        let separate_host = render_bound(&mut separate, 0);
        let collapsed_initial = std::mem::take(&mut *collapsed_capture.lock().unwrap());
        let separate_initial = std::mem::take(&mut *separate_capture.lock().unwrap());
        assert_eq!(collapsed_initial, separate_initial);
        assert_eq!(
            collapsed_host, separate_host,
            "the host planes, track 0's output, are the uncollapsed plan's"
        );
        assert!(
            collapsed.plan.bank_collapse_counters()[0] > 0,
            "real input bank collapsed"
        );
        assert_eq!(separate.plan.bank_collapse_counters()[0], 0);

        let prepared = builtins::prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
        let command = TrackInputRecord::PreparedFilter {
            target: builtins::PreparedInputFilterTarget {
                lanes: BuiltinLaneSelector::Left,
                ..prepared.targets[0]
            },
        };
        for bound in [&mut collapsed, &mut separate] {
            bound.test_only_write(|controls, stamp| {
                controls
                    .iter_mut()
                    .find(|control| control.track_id.as_ref() == pair_graph_tail_id())
                    .expect("selected tail controls")
                    .input
                    .as_mut()
                    .expect("input lane requested")
                    .write(stamp, command);
            });
        }
        let collapsed_host = render_bound(&mut collapsed, HARNESS_QUANTUM as u64);
        let collapsed_asymmetric = std::mem::take(&mut *collapsed_capture.lock().unwrap());
        let separate_host = render_bound(&mut separate, HARNESS_QUANTUM as u64);
        let separate_asymmetric = std::mem::take(&mut *separate_capture.lock().unwrap());
        assert_eq!(collapsed_asymmetric, separate_asymmetric);
        assert_eq!(collapsed_host, separate_host);
        assert!(
            collapsed.plan.bank_collapse_transitions()[0] > 0,
            "input command disengages"
        );
        let frames = HARNESS_QUANTUM as usize;
        assert_ne!(
            &collapsed_asymmetric[..frames],
            &collapsed_asymmetric[frames..],
            "asymmetric left input filter reaches real output"
        );

        let recovered_capture = Arc::new(std::sync::Mutex::new(Vec::new()));
        let reference_capture = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut recovered = prepared_live_graph_fixture(
            false,
            true,
            true,
            Some(Arc::clone(&recovered_capture)),
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
        );
        let mut recovery_reference = prepared_live_graph_fixture(
            false,
            true,
            true,
            Some(Arc::clone(&reference_capture)),
            PAIR_GRAPH_BACKEND,
            TEST_ONLY_PAIR_GRAPH_TRACKS,
        );
        recovered.plan.arm_mono_collapse(&|_| true);
        recovery_reference.plan.force_mono_collapse_off(true);
        let _ = render_bound(&mut recovered, 0);
        let _ = render_bound(&mut recovery_reference, 0);
        let hostile = std::mem::take(&mut *recovered_capture.lock().unwrap());
        let hostile_reference = std::mem::take(&mut *reference_capture.lock().unwrap());
        assert_eq!(hostile, hostile_reference);
        assert!(
            hostile.iter().all(|word| f32::from_bits(*word).is_finite()),
            "the real input bank sanitizes representative NaN/infinity"
        );
        let _ = render_bound(&mut recovered, HARNESS_QUANTUM as u64);
        let _ = render_bound(&mut recovery_reference, HARNESS_QUANTUM as u64);
        let following = std::mem::take(&mut *recovered_capture.lock().unwrap());
        let following_reference = std::mem::take(&mut *reference_capture.lock().unwrap());
        assert_eq!(
            following, following_reference,
            "clean recovery matches the uncollapsed plan"
        );
        assert!(
            following
                .iter()
                .all(|word| f32::from_bits(*word).is_finite())
        );
        assert!(
            following
                .iter()
                .any(|word| *word != 0 && *word != 0x8000_0000)
        );
    }

    fn render_post_input_bits(n: usize, dispatch: Backend) -> (Vec<Vec<u32>>, usize) {
        let (bits, banks, _, _) =
            render_post_input_bits_with_variant(n, dispatch, false, BoundaryVariant::Plain);
        (bits, banks)
    }

    /// Two identical fader and matrix bank owners with their own cells, written together.
    struct TwinFixture {
        pending_fader: FaderBankProcessor,
        pending_matrix: MatrixBankProcessor,
        separate_fader: FaderBankProcessor,
        separate_matrix: MatrixBankProcessor,
        pending_fader_tx: FaderCells,
        pending_matrix_tx: MatrixCells,
        separate_fader_tx: FaderCells,
        separate_matrix_tx: MatrixCells,
        gate: TestOnlyGate,
    }

    impl TwinFixture {
        /// The block's snapshot, taken from the fixture's gate as render takes it.
        fn snapshot(&self) -> LiveSnapshot {
            self.gate.snapshot()
        }

        /// One transaction: the same fader record into both owners' cells, published as one
        /// revision.
        fn write_fader(&mut self, record: TrackFaderRecord) {
            let Self {
                gate,
                pending_fader_tx,
                separate_fader_tx,
                ..
            } = self;
            gate.write_published(|stamp| {
                pending_fader_tx.write(stamp, record);
                separate_fader_tx.write(stamp, record);
            });
        }

        /// The matrix counterpart of [`Self::write_fader`].
        fn write_matrix(&mut self, record: TrackControlRecord) {
            let Self {
                gate,
                pending_matrix_tx,
                separate_matrix_tx,
                ..
            } = self;
            gate.write_published(|stamp| {
                pending_matrix_tx.write(stamp, record);
                separate_matrix_tx.write(stamp, record);
            });
        }
    }

    fn twin_fixture(backend: Backend, members: usize) -> TwinFixture {
        let width = BankWidth::for_backend(backend).expect("vector backend");
        let params = (0..members)
            .map(|lane| BuiltinParameters {
                left: builtins::ChannelParameters {
                    fader_db: -1.0 - lane as f32,
                    muted: lane == 1,
                    ..Default::default()
                },
                right: builtins::ChannelParameters {
                    fader_db: -3.0 + lane as f32 * 0.25,
                    ..Default::default()
                },
                matrix: Matrix2x2 {
                    ll: 0.75,
                    lr: -0.125,
                    rl: 0.25,
                    rr: 0.5,
                },
                smoothing_samples: 0,
            })
            .collect::<Vec<_>>();
        let make = || {
            let counter = LiveValuesSuperseded::default();
            let (fader_tx, fader_rx) = fader_lane(&counter);
            let (matrix_tx, matrix_rx) = matrix_lane(&counter);
            (
                FaderBankProcessor {
                    bank: BuiltinFaderBank::new(backend, width, params.clone()).expect("fader"),
                    controls: core::iter::once(Some(fader_rx))
                        .chain((1..width.lanes()).map(|_| None))
                        .collect(),
                    automation: Box::default(),
                    quiet_at: None,
                    process_calls: 0,
                    frames_processed: 0,
                },
                MatrixBankProcessor {
                    bank: BuiltinMatrixBank::new(
                        backend,
                        width,
                        params.iter().map(|p| (p.matrix, 0)).collect(),
                    )
                    .expect("matrix"),
                    controls: core::iter::once(Some(matrix_rx))
                        .chain((1..width.lanes()).map(|_| None))
                        .collect(),
                    quiet_at: None,
                    process_calls: 0,
                    frames_processed: 0,
                },
                fader_tx,
                matrix_tx,
            )
        };
        let (pending_fader, pending_matrix, pending_fader_tx, pending_matrix_tx) = make();
        let (separate_fader, separate_matrix, separate_fader_tx, separate_matrix_tx) = make();
        TwinFixture {
            pending_fader,
            pending_matrix,
            separate_fader,
            separate_matrix,
            pending_fader_tx,
            pending_matrix_tx,
            separate_fader_tx,
            separate_matrix_tx,
            gate: TestOnlyGate::new(),
        }
    }

    /// `apply_pending` is the block drain with `ALL`, the same code (#1312 D6): fader and matrix
    /// banks whose pending cells were applied by it hold the bank words of banks whose block drain
    /// applied them. The writes mix a fader move, a mute and a ramping matrix on one lane, so a
    /// different application order or a skipped kind moves a word.
    #[test]
    fn apply_pending_equals_the_block_drain() {
        let _guard = PAIR_WITNESS_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for &backend in Backend::VECTOR {
            let members = if backend.width() == 4 { 4 } else { 5 };
            let mut fixture = twin_fixture(backend, members);
            fixture.write_fader(TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Right,
                muted: true,
                smoothing_samples: 5,
            });
            fixture.write_fader(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: -6.0,
                smoothing_samples: 3,
            });
            fixture.write_fader(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: -9.0,
                smoothing_samples: 7,
            });
            fixture.write_matrix(TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.5,
                    lr: 0.25,
                    rl: -0.5,
                    rr: 0.75,
                },
                smoothing_samples: 6,
            });
            let fader_before =
                builtins::test_support::fader_bank_lane_words(&fixture.pending_fader.bank, 0);
            let matrix_before =
                builtins::test_support::matrix_bank_lane_words(&fixture.pending_matrix.bank, 0);
            fixture.pending_fader.apply_pending(EVERY_LANE).unwrap();
            fixture.pending_matrix.apply_pending(EVERY_LANE).unwrap();
            // `apply_pending` alone has moved one pair of banks to where the other pair's block
            // drain moves theirs, in every bank word.
            let snapshot = fixture.snapshot();
            fixture.separate_fader.begin_block(0, snapshot).unwrap();
            fixture.separate_matrix.begin_block(0, snapshot).unwrap();
            let fader_after =
                builtins::test_support::fader_bank_lane_words(&fixture.pending_fader.bank, 0);
            let matrix_after =
                builtins::test_support::matrix_bank_lane_words(&fixture.pending_matrix.bank, 0);
            assert_ne!(fader_after, fader_before, "the fader writes were applied");
            assert_ne!(matrix_after, matrix_before, "the matrix write was applied");
            assert_eq!(
                fader_after,
                builtins::test_support::fader_bank_lane_words(&fixture.separate_fader.bank, 0)
            );
            assert_eq!(
                matrix_after,
                builtins::test_support::matrix_bank_lane_words(&fixture.separate_matrix.bank, 0)
            );
        }
    }

    /// The drain applies a strip's fader cells before its mute cells (#1312 D4), whatever order the
    /// writes came in: they share one gain ramp, and a fader move after an unmute would cut the
    /// unmute's ramp short. (Since #1638 a fader move on a muted lane only remembers the gain, so
    /// the case starts muted and unmutes: a mute write would leave the two orders alike.) The
    /// oracle applies the same two values directly in D4's order; the reverse order is shown to
    /// differ, so the comparison can tell them apart.
    #[test]
    fn the_drain_applies_the_fader_before_the_mute_whatever_the_write_order() {
        let mut muted = BuiltinParameters::default();
        muted.left.muted = true;
        let new_bank = || {
            BuiltinFaderBank::new(Backend::Simd4, BankWidth::Four, vec![muted; 4]).expect("bank")
        };
        let words =
            |bank: &BuiltinFaderBank| builtins::test_support::fader_bank_lane_words(bank, 0);
        let (fader_db, mute_ramp, fader_ramp) = (-6.0_f32, 5_u32, 9_u32);

        let mut canonical = new_bank();
        canonical
            .set_fader_db(0, BuiltinLaneSelector::Left, fader_db, fader_ramp)
            .unwrap();
        canonical
            .set_mute(0, BuiltinLaneSelector::Left, false, mute_ramp)
            .unwrap();
        let mut reversed = new_bank();
        reversed
            .set_mute(0, BuiltinLaneSelector::Left, false, mute_ramp)
            .unwrap();
        reversed
            .set_fader_db(0, BuiltinLaneSelector::Left, fader_db, fader_ramp)
            .unwrap();
        assert_ne!(
            words(&canonical),
            words(&reversed),
            "the order is observable in the ramp state"
        );

        for mute_first in [true, false] {
            let counter = LiveValuesSuperseded::default();
            let (mut cells, reader) = fader_lane(&counter);
            let mut processor = FaderBankProcessor {
                bank: new_bank(),
                controls: core::iter::once(Some(reader))
                    .chain((1..4).map(|_| None))
                    .collect(),
                automation: Box::default(),
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            };
            let mut gate = TestOnlyGate::new();
            let mute = TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Left,
                muted: false,
                smoothing_samples: mute_ramp,
            };
            let fader = TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: fader_db,
                smoothing_samples: fader_ramp,
            };
            let order = if mute_first {
                [mute, fader]
            } else {
                [fader, mute]
            };
            gate.write_published(|stamp| {
                for record in order {
                    cells.write(stamp, record);
                }
            });
            processor.begin_block(0, gate.snapshot()).unwrap();
            assert_eq!(
                words(&processor.bank),
                words(&canonical),
                "mute written first: {mute_first}"
            );
        }
    }

    /// A drain applies only what its block's snapshot covers: a value written for a later
    /// revision stays pending and is applied by the block whose snapshot reaches it, and not
    /// before (#1312 D1, D12). Forty `Both` writes of one cell count `39` replaced writes per
    /// channel cell (D2), and the fader bank applies the two equal cells once.
    #[test]
    fn a_drain_applies_the_newest_value_its_snapshot_covers_and_counts_the_rest() {
        let _guard = PAIR_WITNESS_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let counter = LiveValuesSuperseded::default();
        let (mut cells, reader) = fader_lane(&counter);
        let width = BankWidth::Four;
        let mut processor = FaderBankProcessor {
            bank: BuiltinFaderBank::new(
                Backend::Simd4,
                width,
                vec![BuiltinParameters::default(); 4],
            )
            .expect("bank"),
            controls: core::iter::once(Some(reader))
                .chain((1..width.lanes()).map(|_| None))
                .collect(),
            automation: Box::default(),
            quiet_at: None,
            process_calls: 0,
            frames_processed: 0,
        };
        let mut gate = TestOnlyGate::new();
        let initial = builtins::test_support::fader_bank_lane_words(&processor.bank, 0);
        for index in 0..40_u32 {
            gate.write_published(|stamp| {
                cells.write(
                    stamp,
                    TrackFaderRecord::FaderDb {
                        lanes: BuiltinLaneSelector::Both,
                        db: -(index as f32) * 0.25,
                        smoothing_samples: 0,
                    },
                );
            });
        }
        test_only_reset_fader_matrix_witness();
        // The block's snapshot covers the 40th write; the 39 before it were replaced.
        let covers_forty = gate.snapshot();
        // A 41st write is published after the snapshot: this block must not apply it.
        gate.write_published(|stamp| {
            cells.write(
                stamp,
                TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -30.0,
                    smoothing_samples: 0,
                },
            );
        });
        processor.begin_block(0, covers_forty).unwrap();
        assert_eq!(counter.get(), 78, "39 replaced writes in each channel cell");
        assert_eq!(
            test_only_fader_matrix_witness().fader_values_applied,
            1,
            "two channel cells with equal words are one `Both` apply"
        );
        let after_forty = builtins::test_support::fader_bank_lane_words(&processor.bank, 0);
        assert_ne!(after_forty, initial);
        // The next block's snapshot reaches the 41st write, which replaced nothing.
        processor.begin_block(0, gate.snapshot()).unwrap();
        assert_eq!(counter.get(), 78);
        assert_ne!(
            builtins::test_support::fader_bank_lane_words(&processor.bank, 0),
            after_forty,
            "the later write applied in the block whose snapshot covers it"
        );
        // Nothing is pending now.
        let settled = builtins::test_support::fader_bank_lane_words(&processor.bank, 0);
        let _ = test_only_take_quiet_walk_skips();
        processor.begin_block(0, gate.snapshot()).unwrap();
        assert_eq!(
            builtins::test_support::fader_bank_lane_words(&processor.bank, 0),
            settled
        );
        // Issue #1778: that walk found nothing pending, so a second block under the same
        // publication skips it. A monitor write is a new publication under the same revision
        // (#1603), and the first block under it applies it; a `LiveSnapshot::ALL` block (the
        // carry's) never skips, so a write published between two of them is applied by the second.
        // Red if the quiet mark is compared by revision, or honoured under `ALL`, or if the
        // second block walks (the skip never fires).
        processor.begin_block(0, gate.snapshot()).unwrap();
        assert_eq!(
            test_only_take_quiet_walk_skips(),
            [0, 1, 0],
            "the second block under one publication skipped the fader walk"
        );
        let revision = gate.snapshot().revision();
        let key = {
            let stamp = gate.pin.stamp_monitor(revision).expect("a monitor stamp");
            cells.write(
                stamp,
                TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -3.0,
                    smoothing_samples: 0,
                },
            );
            stamp.key()
        };
        gate.pin.publish_key(&gate.gate, key);
        assert_eq!(gate.snapshot().revision(), revision);
        processor.begin_block(0, gate.snapshot()).unwrap();
        let monitored = builtins::test_support::fader_bank_lane_words(&processor.bank, 0);
        processor.begin_block(0, LiveSnapshot::ALL).unwrap();
        gate.write_published(|stamp| {
            cells.write(
                stamp,
                TrackFaderRecord::FaderDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -9.0,
                    smoothing_samples: 0,
                },
            );
        });
        processor.begin_block(0, LiveSnapshot::ALL).unwrap();
        let after_all = builtins::test_support::fader_bank_lane_words(&processor.bank, 0);
        assert!(
            monitored != settled && after_all != monitored,
            "the monitor write applied under its publication and the write under `ALL`"
        );
        assert_eq!(
            test_only_take_quiet_walk_skips(),
            [0, 0, 0],
            "`ALL` never skips"
        );
    }

    /// The matrix counterpart of the fader drain test above (#1312 D2): the matrix/pan cell counts
    /// the writes a later write replaced, in the same session counter, and applies only what its
    /// block's snapshot covers. Thirty writes of the one matrix cell count `29` replaced writes.
    #[test]
    fn a_matrix_drain_counts_the_replaced_writes_and_applies_only_what_its_snapshot_covers() {
        let counter = LiveValuesSuperseded::default();
        let (mut cells, reader) = matrix_lane(&counter);
        let width = BankWidth::Four;
        let mut processor = MatrixBankProcessor {
            bank: BuiltinMatrixBank::new(
                Backend::Simd4,
                width,
                vec![(Matrix2x2::IDENTITY, 0); width.lanes() as usize],
            )
            .expect("bank"),
            controls: core::iter::once(Some(reader))
                .chain((1..width.lanes()).map(|_| None))
                .collect(),
            quiet_at: None,
            process_calls: 0,
            frames_processed: 0,
        };
        let record = |index: u32| TrackControlRecord {
            matrix: Matrix2x2 {
                ll: 1.0 - index as f32 * 0.01,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
            smoothing_samples: 0,
        };
        let mut gate = TestOnlyGate::new();
        let initial = builtins::test_support::matrix_bank_lane_words(&processor.bank, 0);
        for index in 0..30_u32 {
            gate.write_published(|stamp| cells.write(stamp, record(index)));
        }
        // The block's snapshot covers the 30th write; the 29 before it were replaced.
        let covers_thirty = gate.snapshot();
        // A 31st write is published after the snapshot: this block must not apply it.
        gate.write_published(|stamp| cells.write(stamp, record(30)));
        processor.begin_block(0, covers_thirty).unwrap();
        assert_eq!(counter.get(), 29, "29 replaced writes in the matrix cell");
        let after_thirty = builtins::test_support::matrix_bank_lane_words(&processor.bank, 0);
        assert_ne!(after_thirty, initial);
        // The next block's snapshot reaches the 31st write, which replaced nothing.
        processor.begin_block(0, gate.snapshot()).unwrap();
        assert_eq!(counter.get(), 29);
        assert_ne!(
            builtins::test_support::matrix_bank_lane_words(&processor.bank, 0),
            after_thirty,
            "the later write applied in the block whose snapshot covers it"
        );
        // Issue #1778: a block that finds nothing pending marks the publication, and the next
        // block under it skips the walk.
        let _ = test_only_take_quiet_walk_skips();
        processor.begin_block(0, gate.snapshot()).unwrap();
        processor.begin_block(0, gate.snapshot()).unwrap();
        assert_eq!(test_only_take_quiet_walk_skips(), [0, 0, 1]);
    }

    /// #1651 D1: the mute cells follow every fader cell in the plan's one program, so the fader
    /// cells keep their indices and tables, and [`PreparedBuiltinsSession::strip_mute_cells`]
    /// (the export #1652's route lanes read) names each mute lane's cell in that same program; a
    /// session with no mute entry exports none, and one with a mute entry only builds mute cells
    /// only. The prepared stage mute of the automated lane is `curve || terms`.
    ///
    /// Test value: red if a mute cell is placed before a fader cell (the fader table moves and a
    /// fader-only session no longer builds the program it did), if the export names another
    /// program or cell, or if a mute-only session builds no program.
    #[test]
    fn the_mute_cells_follow_the_fader_cells_in_the_one_program() {
        let plain = n_track_session(3);
        let entry = |name: &str, track: usize, control: u32, lane, segment| session::Automation {
            id: session::StableId::parse(name).expect("id"),
            target: session::ControlAddress {
                owner: session::Owner::Strip(
                    session::StableId::parse(&track_name(track)).expect("track"),
                ),
                section: session::Section::Builtins,
                control,
                lane,
            },
            segments: vec![segment],
            offset: session::AutomationOffset::default(),
        };
        let fader = entry(
            "fader-t01",
            1,
            5,
            session::ParameterChannel::Left,
            session::AutomationSegment {
                shape: session::AutomationShape::Linear,
                start_sample: 0,
                end_sample: 4_096,
                start_value: -6.5,
                end_value: -2.0,
                unit: session::ParameterUnit::Db,
            },
        );
        let mute = entry(
            "mute-t00",
            0,
            6,
            session::ParameterChannel::Both,
            session::AutomationSegment {
                shape: session::AutomationShape::Step,
                start_sample: 0,
                end_sample: 1_000,
                start_value: 1.0,
                end_value: 0.0,
                unit: session::ParameterUnit::Linear,
            },
        );
        let caps_all = CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        };
        let prepare = |automation: Vec<session::Automation>| {
            let mut model = plain.normalized_model().clone();
            model.automation = automation;
            let compiled = compile_session(&model, caps_all).expect("the session compiles");
            prepare_session_builtins(&compiled, &[], caps()).expect("prepared")
        };
        let fader_only = prepare(vec![fader.clone()]);
        let both = prepare(vec![mute.clone(), fader.clone()]);
        let mute_only = prepare(vec![mute.clone()]);
        let fader_segments = controls::stored_segments(&fader);
        let mute_segments = controls::stored_segments(&mute);

        let only = fader_only.fader_automation.as_ref().expect("fader cells");
        assert_eq!(
            only.program_bytes,
            automation::ProgramBuilder::bytes(&[
                controls::FADER_CELL_RULE.describe(&fader_segments)
            ])
        );
        assert!((0..3).all(|track| fader_only.strip_mute_cells(&track_name(track)).is_none()));

        let plan = both.fader_automation.as_ref().expect("cells");
        assert_eq!(
            plan.program_bytes,
            automation::ProgramBuilder::bytes(&[
                controls::FADER_CELL_RULE.describe(&fader_segments),
                controls::MUTE_CELL_RULE.describe(&mute_segments),
                controls::MUTE_CELL_RULE.describe(&mute_segments),
            ]),
            "the fader cell first, then the mute cells"
        );
        let fader_cell = plan.strip(&track_name(1)).expect("track 1").fader[0]
            .as_ref()
            .expect("left fader cell")
            .cell;
        assert_eq!(fader_cell, Some(0), "the fader cell keeps its index");
        let (program, cells) = both
            .strip_mute_cells(&track_name(0))
            .expect("track 0 automates its mute");
        assert!(
            Arc::ptr_eq(&program, &plan.program),
            "the plan's one program"
        );
        assert_eq!(cells, [Some(1), Some(2)]);
        for cell in cells.into_iter().flatten() {
            let segments = program.cell(cell as usize).expect("cell").segments();
            assert_eq!(segments.len(), mute_segments.len());
            assert_eq!(segments[0].start_value.to_bits(), 1.0_f32.to_bits());
        }
        assert!(both.strip_mute_cells(&track_name(1)).is_none());
        assert_eq!(
            plan.mute(&track_name(0), [false, false]),
            [true, true],
            "a curve that starts at 1 prepares the lane muted"
        );
        assert_eq!(plan.mute(&track_name(1), [false, true]), [false, true]);

        let (program, cells) = mute_only
            .strip_mute_cells(&track_name(0))
            .expect("a mute-only session builds its mute cells");
        assert_eq!(cells, [Some(0), Some(1)]);
        assert!(program.cell(2).is_none(), "mute cells only");
    }

    struct HarnessSink;

    impl GraphRuntimeProcessor for HarnessSink {
        fn retained_bytes(&self) -> u64 {
            0
        }

        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn resident_meter_prepared_plans_match_planar_with_aliases_mixed_observers_and_delayed_sends() {
        struct FailingObserver;
        impl GraphRuntimeObserver for FailingObserver {
            fn retained_bytes(&self) -> u64 {
                0
            }

            fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
                Err(RenderError::InvalidEnvelope)
            }
            fn observe_resident(
                &mut self,
                _block: graph::GraphResidentObservationBlock<'_>,
            ) -> Option<Result<(), RenderError>> {
                Some(Err(RenderError::InvalidEnvelope))
            }
        }
        struct CountSink(Arc<AtomicUsize>);
        impl GraphRuntimeProcessor for CountSink {
            fn retained_bytes(&self) -> u64 {
                effect_contract::retained::arc(&self.0)
            }

            fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                self.0.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        }
        let run = |backend, n, variant, disabled, fail| {
            let compiled = n_track_session(n);
            let config = MeterConfig {
                period_frames: NonZeroU32::new(17).unwrap(),
                peak_hold_frames: 3,
                peak_decay_db_per_second: 24.0,
                queue_capacity: NonZeroUsize::new(2).unwrap(),
                reset_generation: 714,
            };
            let selected = n - 1;
            let mut requests = vec![MeterRequest {
                handle: handle(20),
                track_id: track_name(selected),
                tap: MeterTap::PostFader,
                config,
            }];
            if variant == BoundaryVariant::AliasObserved {
                requests.push(MeterRequest {
                    handle: handle(1),
                    track_id: track_name(selected),
                    tap: MeterTap::PostDynamic,
                    config,
                });
            }
            let builtins = prepare_session_builtins(&compiled, &requests, caps()).unwrap();
            let (mut graph, levels) = track_graph_variant_with_route_transform(
                n,
                variant,
                RouteTransform {
                    gain: 0.625,
                    ll: 0.75,
                    lr: -0.25,
                    rl: 0.125,
                    rr: 0.5,
                },
            );
            if variant == BoundaryVariant::SelectedSend {
                let edge_id = GraphEdgeId::RouteDestination {
                    route_id: StableGraphId::parse("proof-send").unwrap(),
                };
                graph.inserted_delays.push(graph::InsertedDelay {
                    node: GraphNodeId::CompensationDelay {
                        edge_id: Box::new(edge_id.clone()),
                    },
                    edge_id,
                    samples: effect_contract::LatencySamples(7),
                });
            }
            let classes = SessionPoolClasses::from_session(&compiled);
            let mut artifact =
                builtins.into_graph_artifact_with_banks(graph, (), backend, &levels, &classes);
            let node = GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(&track_name(selected)).unwrap(),
                stage: TrackStage::PostFader,
            };
            let extra = MeterAccumulator::prepare(handle(30), config, 48_000).unwrap();
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    node.clone(),
                    30,
                    Box::new(MeterObserver(extra.accumulator)),
                ));
            artifact.meter_consumers.push(MeterConsumer {
                handle: handle(30),
                track_id: track_name(selected).into_boxed_str(),
                tap: MeterTap::PostFader,
                consumer: extra.consumer,
            });
            if fail {
                artifact
                    .builtin_observers
                    .push(GraphNodeObserverBinding::new(
                        node.clone(),
                        22,
                        Box::new(FailingObserver),
                    ));
            }
            let capture = Arc::new(std::sync::Mutex::new(Vec::new()));
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    node,
                    25,
                    Box::new(Capture(Arc::clone(&capture))),
                ));
            let envelope = artifact.graph.envelope;
            let mut nodes: Vec<_> = (0..n)
                .map(|index| {
                    GraphNodeBinding::new(
                        GraphNodeId::TrackStage {
                            track_id: StableGraphId::parse(&track_name(index)).unwrap(),
                            stage: TrackStage::Input,
                        },
                        Box::new(SeededInput {
                            seed: 714 + index as u64,
                            symmetric: false,
                            nonfinite: true,
                        }),
                    )
                })
                .collect();
            let sink_calls = Arc::new(AtomicUsize::new(0));
            nodes.push(GraphNodeBinding::new(
                GraphNodeId::Output {
                    output_id: StableGraphId::parse("main-out").unwrap(),
                },
                Box::new(CountSink(Arc::clone(&sink_calls))),
            ));
            let mut bound = artifact
                .into_bound(GraphRuntimeBindings {
                    envelope,
                    nodes,
                    observers: Vec::new(),
                })
                .unwrap_or_else(|error| panic!("resident meter bind: {}", error.code));
            graph::test_only_meter_input_reset(disabled);
            let mut pcm = Vec::new();
            let mut snapshots = Vec::new();
            let mut reports = Vec::new();
            for block in 0..if fail { 1 } else { 5 } {
                let mut words = vec![0.0; HARNESS_QUANTUM as usize * 2];
                let report = bound.plan.render(
                    engine::realtime::RenderIo {
                        output: engine::realtime::PlanarBufferMut::try_new(
                            &mut words,
                            2,
                            HARNESS_QUANTUM as usize,
                            HARNESS_QUANTUM as usize,
                        )
                        .unwrap(),
                    },
                    engine::realtime::RenderTime {
                        absolute_sample: block * u64::from(HARNESS_QUANTUM),
                    },
                );
                assert_eq!(report.is_err(), fail);
                reports.push(report);
                pcm.extend(words.into_iter().map(f32::to_bits));
                for meter in &mut bound.meter_consumers {
                    while let Ok(snapshot) = meter.consumer.try_pop() {
                        snapshots.push(snapshot);
                    }
                }
            }
            let counts = graph::test_only_meter_input_counts();
            graph::test_only_meter_input_reset(false);
            let observed = capture.lock().unwrap().clone();
            if fail {
                assert!(observed.is_empty(), "later custom observer must not run");
                assert!(!snapshots.is_empty());
                assert!(
                    snapshots
                        .iter()
                        .all(|snapshot| snapshot.handle == handle(20)),
                    "later direct and alias meters must publish nothing"
                );
                assert_eq!(
                    sink_calls.load(Ordering::Relaxed),
                    0,
                    "successor must not execute"
                );
            } else {
                assert!(observed.iter().any(|word| *word != 0));
                assert!(
                    snapshots
                        .iter()
                        .any(|snapshot| snapshot.cumulative_dropped_snapshots > 0)
                );
                assert_eq!(sink_calls.load(Ordering::Relaxed), 5);
            }
            (pcm, observed, snapshots, counts, reports)
        };
        for backend in [Backend::Scalar]
            .into_iter()
            .chain(Backend::VECTOR.iter().copied())
        {
            for n in [3, 4, 5, 8, 9] {
                for variant in [
                    BoundaryVariant::Plain,
                    BoundaryVariant::AliasObserved,
                    BoundaryVariant::SelectedSend,
                ] {
                    let a = run(backend, n, variant, false, false);
                    let b = run(backend, n, variant, true, false);
                    assert_eq!(a.0, b.0, "output: {backend:?}/{n}/{variant:?}");
                    assert_eq!(a.1, b.1);
                    assert_eq!(a.4, b.4);
                    assert_eq!(a.2, b.2);
                    for (a, b) in a.2.iter().zip(&b.2) {
                        for (a, b) in [(a.left, b.left), (a.right, b.right)] {
                            assert_eq!(a.sample_peak.to_bits(), b.sample_peak.to_bits());
                            assert_eq!(a.rms.to_bits(), b.rms.to_bits());
                            assert_eq!(a.energy.to_bits(), b.energy.to_bits());
                            assert_eq!(a.held_peak.to_bits(), b.held_peak.to_bits());
                        }
                    }
                    assert_eq!(b.3[1..], [0, 0]);
                    if backend == Backend::Scalar {
                        assert_eq!(a.3[1..], [0, 0]);
                    } else {
                        assert!(
                            a.3[2] >= 10,
                            "both production meters must accept resident input: {backend:?}/{n}/{variant:?}: {:?}",
                            a.3
                        );
                        assert!(a.3[1] > a.3[2], "the mixed custom observer declines");
                        // Mixed declines share one planar acquisition at this final member.
                        assert_eq!(a.3[0], b.3[0]);
                    }
                }
            }
            let a = run(backend, 3, BoundaryVariant::AliasObserved, false, true);
            let b = run(backend, 3, BoundaryVariant::AliasObserved, true, true);
            assert_eq!((&a.0, &a.1, &a.2, &a.4), (&b.0, &b.1, &b.2, &b.4));
            assert_eq!(
                a.3,
                if backend == Backend::Scalar {
                    [1, 0, 0]
                } else {
                    [0, 2, 2]
                }
            );
            assert_eq!(b.3, [1, 0, 0]);
        }
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn resident_meter_accepted_plan_avoids_planar_acquisition_and_matches_decline() {
        for &backend in Backend::VECTOR {
            // The combined case keeps L/R distinct so swapped held-state metadata cannot pass;
            // the other cases retain the collapsed/forced-dual execution rows.
            for (ballistics, symmetric) in [
                ((0, 0.0), true),
                ((7, 0.0), true),
                ((0, 12.0), true),
                ((7, 12.0), false),
            ] {
                let mut candidate = prepared_live_graph_fixture_with_ballistics(
                    Some(8),
                    symmetric,
                    false,
                    None,
                    backend,
                    9,
                    ballistics,
                );
                let mut reference = prepared_live_graph_fixture_with_ballistics(
                    Some(8),
                    symmetric,
                    false,
                    None,
                    backend,
                    9,
                    ballistics,
                );
                candidate.plan.arm_mono_collapse(&|_| true);
                reference.plan.arm_mono_collapse(&|_| true);
                let mut a = vec![0.0; HARNESS_QUANTUM as usize * 2];
                let mut b = a.clone();
                // Warm both plans before checking steady-state dispatch and parity.
                graph::test_only_meter_input_reset(false);
                render_bound_result(&mut candidate, 0, &mut a).unwrap();
                graph::test_only_meter_input_reset(true);
                render_bound_result(&mut reference, 0, &mut b).unwrap();
                candidate.meter_consumers[0].consumer.try_pop().unwrap();
                reference.meter_consumers[0].consumer.try_pop().unwrap();
                for (block, forced) in [false, true, false].into_iter().enumerate() {
                    candidate.plan.force_mono_collapse_off(forced);
                    reference.plan.force_mono_collapse_off(forced);
                    graph::test_only_meter_input_reset(false);
                    builtins::test_only_reset_banked_meter_commits();
                    let result = render_bound_result(
                        &mut candidate,
                        (block as u64 + 1) * u64::from(HARNESS_QUANTUM),
                        &mut a,
                    );
                    result.unwrap();
                    assert_eq!(graph::test_only_meter_input_counts(), [0, 1, 1]);
                    assert_eq!(
                        builtins::test_only_banked_meter_commits(),
                        1,
                        "{backend:?} {ballistics:?}: the resident meter commits the banked recurrence"
                    );
                    graph::test_only_meter_input_reset(true);
                    render_bound_result(
                        &mut reference,
                        (block as u64 + 1) * u64::from(HARNESS_QUANTUM),
                        &mut b,
                    )
                    .unwrap();
                    assert_eq!(graph::test_only_meter_input_counts(), [1, 0, 0]);
                    assert_eq!(
                        a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        b.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                    assert_eq!(
                        candidate.meter_consumers[0].consumer.try_pop(),
                        reference.meter_consumers[0].consumer.try_pop()
                    );
                    graph::test_only_meter_input_reset(false);
                }
            }
        }
    }

    fn host_dispatch() -> Backend {
        Backend::current()
    }

    fn scalar_dispatch() -> Backend {
        Backend::Scalar
    }

    /// E2 (#86 F2 proof, F3): a track's bits do not depend on whether it renders in a bank.
    ///
    /// `n = 1..=9` covers the short first bank, the exactly-full bank and the padded second bank
    /// at both widths. The oracle is the `Scalar` dispatch, which banks nothing at all.
    #[test]
    fn banked_tracks_are_bit_identical_to_their_scalar_tails() {
        let host = host_dispatch();
        for n in 1..=9 {
            let (banked, bank_count) = render_post_input_bits(n, host);
            let (scalar, scalar_banks) = render_post_input_bits(n, scalar_dispatch());
            assert_eq!(scalar_banks, 0, "the scalar oracle banks nothing");
            match BankWidth::for_backend(host) {
                // Three bankable stages per track since #212 -- post-input builtins, fader,
                // matrix -- each grouping the same `n` tracks into the same padded banks.
                Some(width) => assert_eq!(
                    bank_count,
                    3 * n.div_ceil(width.lanes() as usize),
                    "padded bank count for {n} tracks over three bankable stages"
                ),
                None => assert_eq!(bank_count, 0),
            }
            // Distinctness guard: a harness whose tracks all render silence, or render the same
            // thing, would pass every identity assertion below without testing anything.
            assert!(
                banked
                    .iter()
                    .all(|bits| bits.iter().any(|bits| *bits != 0 && *bits != 0x8000_0000)),
                "every track must carry signal"
            );
            let distinct: BTreeSet<_> = banked.iter().collect();
            assert_eq!(distinct.len(), n, "every track must render differently");
            for (track, (banked, scalar)) in banked.iter().zip(&scalar).enumerate() {
                assert_eq!(
                    banked, scalar,
                    "track {track} of {n} differs between the bank and the scalar tail"
                );
            }
        }
    }

    /// E3 (#86 F2 partition invariance): adding a track never moves an existing track's bits.
    ///
    /// 7, 8 and 9 tracks straddle the W8 bank boundary in both directions, and 3/4/5 straddle W4.
    #[test]
    fn track_bits_do_not_depend_on_session_track_count() {
        let host = host_dispatch();
        let renders: BTreeMap<usize, Vec<Vec<u32>>> = [3, 4, 5, 7, 8, 9]
            .into_iter()
            .map(|n| (n, render_post_input_bits(n, host).0))
            .collect();
        let reference = render_post_input_bits(7, scalar_dispatch()).0;
        for (n, bits) in &renders {
            for (track, (bits, reference)) in bits.iter().zip(&reference).enumerate() {
                assert_eq!(
                    bits, reference,
                    "track {track} moved between a {n}-track session and the 7-track scalar oracle"
                );
            }
        }
        for smaller in [3, 4, 5, 7, 8] {
            for larger in [4, 5, 7, 8, 9] {
                if larger <= smaller {
                    continue;
                }
                for (track, (small, large)) in
                    renders[&smaller].iter().zip(&renders[&larger]).enumerate()
                {
                    assert_eq!(
                        small, large,
                        "track {track} moved from a {smaller}-track to a {larger}-track session"
                    );
                }
            }
        }
    }

    fn handle(value: u64) -> MeterHandle {
        MeterHandle(NonZeroU64::new(value).expect("nonzero test meter handle"))
    }
    #[test]
    fn prepares_three_sections_and_each_named_meter_tap() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 8,
            peak_decay_db_per_second: 12.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 3,
        };
        let requests: Vec<_> = [
            MeterTap::Input,
            MeterTap::PostInputBuiltins,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostSimd2PreFader,
            MeterTap::PostFader,
            MeterTap::PostMatrix,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: handle(u64::try_from(index).expect("bounded") + 1),
            track_id: "vocal".to_owned(),
            tap,
            config,
        })
        .collect();
        let prepared = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        // All three of the track's stages are deferred strip stages now (#212 for the fader and
        // the matrix, #210 phase 3 for the input), so preparation holds no bindings at all.
        assert_eq!(prepared.processor_count(), 3);
        assert_eq!(prepared.processors.len(), 0);
        assert_eq!(prepared.strips.len(), 1);
        assert_eq!(prepared.observers.len(), 7);
        assert_eq!(prepared.meter_consumers.len(), 7);
        assert_eq!(prepared.resources.meter_items, 35);
        assert!(prepared.resources.engine_owned_processor_payload_bytes > 0);
        assert!(
            prepared.resources.engine_owned_meter_payload_bytes
                > 35 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert!(
            prepared.resources.maximum_single_allocation_bytes
                >= 5 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        // #1329 D4, D7: a strip without a live input lane reports its prepared design's own
        // certified tail; the fixture's input HPF makes it finite and non-zero.
        let session = session();
        let model = session.normalized_model();
        let effective = controls::effective(model, &controls::MonitorOverlay::default());
        let (strip, fader) = model
            .strips()
            .zip(effective.strips.iter())
            .next()
            .expect("one strip");
        let parameters =
            strip_parameters(&strip, (fader.fader_db, fader.mute), u32::MAX).expect("parameters");
        let own =
            builtins::input_section_bound(session.sample_rate().0, parameters).expect("bound");
        assert!(matches!(own.tail, TailSamples::Finite(samples) if samples > 0));
        assert_eq!(
            prepared.tails().collect::<Vec<_>>(),
            vec![("vocal", own.tail)]
        );
        assert_eq!(
            prepared.input_bounds().collect::<Vec<_>>(),
            vec![("vocal", own)]
        );
    }

    #[test]
    fn live_input_lane_reports_the_live_bound_and_plain_input_its_own() {
        let mut model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("parse");
        for track in &mut model.tracks {
            track.builtins.left.hpf_hz = 0.0;
            track.builtins.left.lpf_hz = 0.0;
            track.builtins.right.hpf_hz = 0.0;
            track.builtins.right.lpf_hz = 0.0;
        }
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile");
        // Disabled filters: trim and polarity are memoryless (#1329 D4). The strip reports its
        // own bound: a zero tail with its trim-only composition stated (#1465, root's named
        // exception of 2026-10-08).
        let plain = prepare_session_builtins(&compiled, &[], caps()).expect("plain");
        assert_eq!(
            plain.tails().collect::<Vec<_>>(),
            vec![("vocal", TailSamples::Finite(0))]
        );
        let normalized = compiled.normalized_model();
        let effective = controls::effective(normalized, &controls::MonitorOverlay::default());
        let (strip, fader) = normalized
            .strips()
            .zip(effective.strips.iter())
            .next()
            .expect("one strip");
        let parameters =
            strip_parameters(&strip, (fader.fader_db, fader.mute), u32::MAX).expect("parameters");
        let own =
            builtins::input_section_bound(compiled.sample_rate().0, parameters).expect("bound");
        assert_eq!(own.tail, TailSamples::Finite(0));
        assert_eq!(
            plain.input_bounds().collect::<Vec<_>>(),
            vec![("vocal", own)]
        );

        let live = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            &[],
            &[TrackControlRequest {
                track_id: "vocal".to_owned(),
                superseded: LiveValuesSuperseded::default(),
                input_lane: true,
            }],
            caps(),
        )
        .expect("live");
        // A live input lane can enable either filter anywhere in the domain, so the strip reports
        // the live bound (#1329 D5, D7), whatever its prepared design.
        let live_bound =
            builtins::input_section_live_bound(compiled.sample_rate().0).expect("launch rate");
        assert!(matches!(live_bound.tail, TailSamples::Finite(samples) if samples > 0));
        assert_eq!(
            live.tails().collect::<Vec<_>>(),
            vec![("vocal", live_bound.tail)]
        );
        assert_eq!(
            live.input_bounds().collect::<Vec<_>>(),
            vec![("vocal", live_bound)]
        );
        assert!(live.validate_for_session(&compiled).0.is_empty());
        // #1329 Amendment 5 (m1): the seal check's live rule. A live strip that carries its own
        // design bound (here `ZERO`: both filters disabled) in the payload and in the seal alike
        // agrees with itself, so only the rule that a live strip reports the live bound refuses it.
        let mut forged = live;
        forged.tails[0].1 = NodeTailBound::ZERO;
        forged.seal.tails[0].1 = NodeTailBound::ZERO;
        let diagnostics = forged.validate_for_session(&compiled);
        assert!(
            diagnostics
                .0
                .iter()
                .any(|diagnostic| diagnostic.code == "builtin.prepared.tail_set"),
            "a live strip carrying its design bound passed the seal check: {diagnostics:?}"
        );
    }

    /// #1329 Amendment 5 (MJ1): preparation computes a design bound only for a strip without a
    /// live input lane, once per distinct design.
    #[cfg(feature = "test-support")]
    #[test]
    fn design_bounds_are_computed_only_for_strips_without_a_live_input_lane() {
        let mut model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("parse");
        // Six distinct designs over nine strips: eq0-eq2 keep the fixture's 20 Hz / 20 kHz pair,
        // eq3 moves its HPF, eq4 its LPF, eq5 its trim, eq6 only its right channel's HPF, and
        // eq7-eq8 disable both filters. The last is memoryless: its zero bound walks nothing and
        // is not computed (#1457), so five designs are bounded.
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let builtins = &mut track.builtins;
            match index {
                3 => {
                    builtins.left.hpf_hz = 40.0;
                    builtins.right.hpf_hz = 40.0;
                }
                4 => {
                    builtins.left.lpf_hz = 10_000.0;
                    builtins.right.lpf_hz = 10_000.0;
                }
                5 => {
                    builtins.left.trim_db = 6.0;
                    builtins.right.trim_db = 6.0;
                }
                6 => builtins.right.hpf_hz = 80.0,
                7 | 8 => {
                    for channel in [&mut builtins.left, &mut builtins.right] {
                        channel.hpf_hz = 0.0;
                        channel.lpf_hz = 0.0;
                    }
                }
                _ => {}
            }
        }
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile");
        let ids: Vec<String> = model
            .tracks
            .iter()
            .map(|track| track.id.to_string())
            .collect();
        let controls = |live: &[usize]| -> Vec<TrackControlRequest> {
            ids.iter()
                .enumerate()
                .map(|(index, id)| TrackControlRequest {
                    track_id: id.clone(),
                    superseded: LiveValuesSuperseded::default(),
                    input_lane: live.contains(&index),
                })
                .collect()
        };
        let computed = |live: &[usize]| {
            let before = builtins::test_support::fixed_input_bounds_computed();
            let prepared = prepare_session_builtins_with_live_controls(
                &compiled,
                &effective_of(&compiled),
                &[],
                &controls(live),
                caps(),
            )
            .expect("prepare");
            let after = builtins::test_support::fixed_input_bounds_computed();
            assert!(prepared.validate_for_session(&compiled).0.is_empty());
            after - before
        };
        assert_eq!(
            computed(&[]),
            5,
            "no strip live: one bound per distinct design with an enabled filter"
        );
        assert_eq!(
            computed(&(0..9).collect::<Vec<_>>()),
            0,
            "every strip live (the browser): no design bound is computed"
        );
        // eq3 and eq4 are the only strips of their designs.
        assert_eq!(computed(&[3, 4]), 3, "a live strip's design is not bounded");
    }

    /// #1457 D1's purpose: typical sessions bound every distinct design exactly. Each 64-track
    /// console document (`console-sixty-four-track.json`, `-app`, `-intended`, `-mono` and
    /// `-sends`, 64 distinct input designs each) prepares with no strip on the live bound at every
    /// launch rate, its charge (frames walked plus the fixed charges) inside the budget.
    #[cfg(feature = "test-support")]
    #[test]
    fn every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate() {
        let documents = [
            (
                "console-sixty-four-track.json",
                include_str!("../../../fixtures/session/v1/console-sixty-four-track.json"),
            ),
            (
                "console-sixty-four-track-app.json",
                include_str!("../../../fixtures/session/v1/console-sixty-four-track-app.json"),
            ),
            (
                "console-sixty-four-track-intended.json",
                include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json"),
            ),
            (
                "console-sixty-four-track-mono.json",
                include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json"),
            ),
            (
                "console-sixty-four-track-sends.json",
                include_str!("../../../fixtures/session/v1/console-sixty-four-track-sends.json"),
            ),
        ];
        for (name, document) in documents {
            let model = parse_session_json(document).expect("parse");
            for rate in [44_100, 48_000, 88_200, 96_000] {
                let mut model = model.clone();
                model.sample_rate_hz = rate;
                let compiled = compile_session(
                    &model,
                    CompileCaps {
                        max_compiled_model_bytes: u64::MAX,
                        max_requested_runtime_bytes: u64::MAX,
                        max_single_allocation_bytes: u64::MAX,
                        max_queue_items: u64::MAX,
                        max_source_ring_frames: u64::MAX,
                        max_source_ring_bytes: u64::MAX,
                    },
                )
                .expect("compile");
                let frames = builtins::test_support::fixed_input_frames_walked();
                let charged = builtins::test_support::fixed_input_charged();
                let prepared = prepare_session_builtins_with_live_controls(
                    &compiled,
                    &effective_of(&compiled),
                    &[],
                    &[],
                    caps(),
                )
                .expect("prepare");
                let walked = builtins::test_support::fixed_input_frames_walked() - frames;
                let charged = builtins::test_support::fixed_input_charged() - charged;
                let live = builtins::input_section_live_bound_table(rate).expect("launch rate");
                let on_live: Vec<&str> = prepared
                    .input_bounds()
                    .filter(|(_, bound)| *bound == live)
                    .map(|(id, _)| id)
                    .collect();
                assert!(
                    on_live.is_empty() && charged <= builtins::INPUT_BOUND_BUDGET_FRAMES,
                    "{name} at {rate} Hz: charged {charged} ({walked} frames walked) of a budget \
                     of {}, on the live bound: {on_live:?}",
                    builtins::INPUT_BOUND_BUDGET_FRAMES
                );
                println!(
                    "{name} at {rate} Hz: charged {charged} ({walked} frames walked), margin {}",
                    builtins::INPUT_BOUND_BUDGET_FRAMES - charged
                );
            }
        }
    }

    /// #1457 D1: a preparation walks at most the budget. The first three strips carry distinct
    /// near-top designs, more than the budget covers: a preparation walks exactly the budget less
    /// the three designs' section charges (the design that crosses it is stopped there, and every
    /// later one walks nothing) and reports the live bound for the stopped design.
    #[cfg(feature = "test-support")]
    #[test]
    fn a_preparation_walks_at_most_the_budget() {
        let mut model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("parse");
        let rate = model.sample_rate_hz;
        let maximum = builtin_filter_cutoff_maximum_hz(rate).expect("launch rate");
        let below = |steps: u32| f32::from_bits(maximum.to_bits() - steps);
        let designs = [
            (below(1), maximum),
            (below(2), maximum),
            (below(3), maximum),
        ];
        for (track, &(hpf, lpf)) in model.tracks.iter_mut().zip(&designs) {
            for channel in [&mut track.builtins.left, &mut track.builtins.right] {
                channel.hpf_hz = hpf;
                channel.lpf_hz = lpf;
                channel.trim_db = 24.0;
            }
        }
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile");
        let prepare = || {
            let bounds = builtins::test_support::fixed_input_bounds_computed();
            let frames = builtins::test_support::fixed_input_frames_walked();
            let prepared = prepare_session_builtins_with_live_controls(
                &compiled,
                &effective_of(&compiled),
                &[],
                &[],
                caps(),
            )
            .expect("prepare");
            assert!(prepared.validate_for_session(&compiled).0.is_empty());
            let values: Vec<(String, NodeTailBound)> = prepared
                .input_bounds()
                .map(|(id, bound)| (id.to_owned(), bound))
                .collect();
            (
                values,
                builtins::test_support::fixed_input_bounds_computed() - bounds,
                builtins::test_support::fixed_input_frames_walked() - frames,
            )
        };
        // Each design reserves its sections' charge before it walks: two sections on one cascade.
        let fixed = 3 * 2 * builtins::INPUT_BOUND_SECTION_CHARGE;
        let spent = builtins::INPUT_BOUND_BUDGET_FRAMES - fixed;
        let (none, _, walked) = prepare();
        assert_eq!(walked, spent, "walked");
        let live = builtins::input_section_live_bound_table(rate).expect("launch rate");
        assert!(none.iter().any(|(_, bound)| *bound == live), "{none:?}");
    }

    #[test]
    fn rejects_duplicate_and_unknown_meter_transactionally() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let result = prepare_session_builtins(
            &session(),
            &[
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
            ],
            caps(),
        );
        let Err(error) = result else {
            panic!("must reject");
        };
        assert_eq!(
            error.0.iter().map(|item| item.code).collect::<Vec<_>>(),
            vec![
                "builtin.meter.duplicate",
                "builtin.meter.duplicate_handle",
                "builtin.meter.unknown_track"
            ]
        );
    }

    #[test]
    fn cutoff_boundaries_match_compiler_diagnostics_at_every_launch_rate_and_section() {
        let document = include_str!("../../../fixtures/session/v1/canonical.json");
        let base_model = parse_session_json(document).expect("parse boundary session");
        for (rate, maximum_bits) in [
            (44_100, 0x46ac_42f7),
            (48_000, 0x46bb_7ede),
            (88_200, 0x472c_42f7),
            (96_000, 0x473b_7ede),
        ] {
            for (high_pass, path) in [
                (true, "$.tracks[id=vocal].builtins.left.hpf_hz"),
                (false, "$.tracks[id=vocal].builtins.left.lpf_hz"),
            ] {
                let prepare = |cutoff: f32| {
                    let mut model = base_model.clone();
                    model.sample_rate_hz = rate;
                    for track in &mut model.tracks {
                        track.builtins.left.hpf_hz = 0.0;
                        track.builtins.left.lpf_hz = 0.0;
                        track.builtins.right.hpf_hz = 0.0;
                        track.builtins.right.lpf_hz = 0.0;
                    }
                    if high_pass {
                        model.tracks[0].builtins.left.hpf_hz = cutoff;
                    } else {
                        model.tracks[0].builtins.left.lpf_hz = cutoff;
                    }
                    let compiled = compile_session(
                        &model,
                        CompileCaps {
                            max_compiled_model_bytes: u64::MAX,
                            max_requested_runtime_bytes: u64::MAX,
                            max_single_allocation_bytes: u64::MAX,
                            max_queue_items: u64::MAX,
                            max_source_ring_frames: u64::MAX,
                            max_source_ring_bytes: u64::MAX,
                        },
                    )
                    .expect("launch-rate boundary session compiles");
                    prepare_session_builtins(&compiled, &[], caps())
                };
                prepare(f32::from_bits(maximum_bits)).unwrap_or_else(|error| {
                    panic!(
                        "maximum must prepare: rate={rate}, high_pass={high_pass}, error={error:?}"
                    )
                });
                let Err(successor_error) = prepare(f32::from_bits(maximum_bits + 1)) else {
                    panic!("the immediate successor must reject before coefficient preparation");
                };
                assert_eq!(
                    successor_error,
                    BuiltinDiagnosticSet(vec![diag("builtin.filter.cutoff", path)]),
                    "rate={rate}, high_pass={high_pass}"
                );
            }
        }
    }

    /// Frozen issue-034 compiler-mutation seed. This exercises complete preparation requests and
    /// their prepared block/target contract, never a timed workload.
    const BUILTIN_COMPILER_MUTATION_SEED: u64 = 0x34_007_c10_u64;
    const BUILTIN_COMPILER_MUTATION_CLASSES: usize = 49;

    #[test]
    fn deterministic_builtin_compiler_mutations_cover_domains_and_preparation() {
        let mut base_model =
            parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
                .expect("parse baseline mutation session");
        base_model.tracks[0].inserts.effects.clear();
        base_model.automation.clear();
        let base_config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 0,
        };
        let baseline_request = MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config: base_config,
        };
        let accepted = compile_session(
            &base_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile baseline mutation session");
        let baseline =
            prepare_session_builtins(&accepted, std::slice::from_ref(&baseline_request), caps())
                .expect("baseline preparation");
        let report = baseline.resource_report();
        let mut state = BUILTIN_COMPILER_MUTATION_SEED;
        let mut seen_taps = BTreeSet::new();
        let mut seen_rates = BTreeSet::new();
        let mut seen_quanta = BTreeSet::new();
        let mut seen_smoothing = BTreeSet::new();
        let mut classes = [false; BUILTIN_COMPILER_MUTATION_CLASSES];
        for case in 0_u32..10_000 {
            // Keep the deterministic generator and its input order.
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let value = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            let class =
                usize::try_from(case).expect("u32 fits usize") % BUILTIN_COMPILER_MUTATION_CLASSES;
            classes[class] = true;
            let taps = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ];
            let tap = taps[usize::try_from(case % 7).expect("bounded tap index")];
            seen_taps.insert(tap);
            let rate = [44_100, 48_000, 88_200, 96_000]
                [usize::try_from(case % 4).expect("bounded rate index")];
            let quantum = [1, 127, 128, 255, 1_024]
                [usize::try_from(case % 5).expect("bounded quantum index")];
            let smoothing = [0, 1, 2, 127, 128, u32::MAX]
                [usize::try_from(case % 6).expect("bounded smoothing index")];
            seen_rates.insert(rate);
            seen_quanta.insert(quantum);
            seen_smoothing.insert(smoothing);

            if class == 0 {
                let invalid = include_str!("../../../fixtures/session/v1/canonical.json").replacen(
                    "\"polarity_invert\": false",
                    "\"polarity_invert\": 0.5",
                    1,
                );
                let diagnostics = parse_session_json(&invalid)
                    .expect_err("numeric boolean encoding must reject before preparation");
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(
                    observed,
                    vec![(
                        "schema.wrong_type",
                        "$.tracks[0].builtins.left.polarity_invert".to_owned()
                    )]
                );
                continue;
            }

            let mut model = base_model.clone();
            model.sample_rate_hz = rate;
            model.quantum_frames = quantum;
            model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples: smoothing,
            };
            let mut requests = vec![MeterRequest {
                handle: handle(1),
                track_id: "vocal".to_owned(),
                tap,
                config: base_config,
            }];
            let mut mutation_caps = caps();
            let mut expected = Vec::new();
            let mut expected_session = Vec::new();
            let mut target = Matrix2x2::IDENTITY;
            let mut block_probe = 0_u8;
            match class {
                1 => model.tracks[0].builtins.left.polarity_invert = value & 1 != 0,
                2 => model.tracks[0].builtins.left.trim_db = -144.0,
                3 => model.tracks[0].builtins.right.trim_db = 24.0,
                4 => {
                    model.tracks[0].builtins.left.trim_db = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.left.trim_db".to_owned(),
                    ));
                }
                5 => {
                    model.tracks[0].fader.right_db = 24.001;
                    expected.push(diag(
                        "builtin.gain.domain",
                        "$.tracks[id=vocal].fader.right_db",
                    ));
                }
                6 => model.tracks[0].builtins.left.hpf_hz = 0.0,
                7 => model.tracks[0].builtins.left.hpf_hz = 10.0,
                8 => {
                    model.tracks[0].builtins.left.hpf_hz = rate as f32 / 2.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    expected.push(diag(
                        "builtin.filter.cutoff",
                        "$.tracks[id=vocal].builtins.left.hpf_hz",
                    ));
                }
                9 => {
                    model.tracks[0].builtins.left.hpf_hz = 1_000.0;
                    model.tracks[0].builtins.left.lpf_hz = 100.0;
                    expected.push(diag(
                        "builtin.filter.order",
                        "$.tracks[id=vocal].builtins.left.lpf_hz",
                    ));
                }
                10 => {
                    if let MatrixOrPan::Matrix { ll, .. } = &mut model.tracks[0].matrix_or_pan {
                        *ll = -1.0;
                    }
                }
                11 => {
                    if let MatrixOrPan::Matrix { rr, .. } = &mut model.tracks[0].matrix_or_pan {
                        *rr = 1.001;
                    }
                    expected.push(diag(
                        "builtin.matrix.coefficient",
                        "$.tracks[id=vocal].matrix_or_pan.rr",
                    ));
                }
                12 => {
                    requests.push(MeterRequest {
                        handle: handle(1),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    expected.push(diag(
                        "builtin.meter.duplicate_handle",
                        &meter_path(&requests[1]),
                    ));
                }
                13 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        ..requests[0].clone()
                    });
                    expected.push(diag("builtin.meter.duplicate", &meter_path(&requests[1])));
                }
                14 => {
                    requests[0].track_id = "unknown-track".to_owned();
                    expected.push(diag(
                        "builtin.meter.unknown_track",
                        &meter_path(&requests[0]),
                    ));
                }
                15 => requests[0].config.period_frames = NonZeroU32::new(1).expect("constant"),
                16 => {
                    requests[0].config.period_frames = NonZeroU32::new(u32::MAX).expect("constant")
                }
                17 => {
                    requests[0].config.period_frames = NonZeroU32::new(128).expect("constant");
                    mutation_caps.maximum_period_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                18 => requests[0].config.peak_hold_frames = u32::MAX,
                19 => {
                    requests[0].config.peak_hold_frames = 128;
                    mutation_caps.maximum_peak_hold_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                20 => requests[0].config.peak_decay_db_per_second = 120.0,
                21 => {
                    requests[0].config.peak_decay_db_per_second = f32::NAN;
                    expected.push(diag("builtin.meter.config", &meter_path(&requests[0])));
                }
                22 => requests[0].config.reset_generation = u64::MAX,
                23 => {
                    requests[0].config.queue_capacity =
                        NonZeroUsize::new(usize::MAX).expect("constant");
                    expected.push(diag(
                        "builtin.resource.arithmetic_overflow",
                        &meter_path(&requests[0]),
                    ));
                }
                24 => {
                    target = Matrix2x2 {
                        ll: -1.0,
                        lr: 1.0,
                        rl: 1.0,
                        rr: -1.0,
                    }
                }
                25 => target.ll = f32::NAN,
                26 => target.lr = f32::INFINITY,
                27 => target.rr = 1.001,
                28 => block_probe = 1,
                29 => block_probe = 2,
                30 => block_probe = 3,
                31 => block_probe = 4,
                32 => {
                    mutation_caps.maximum_total_state_bytes =
                        report.engine_owned_processor_payload_bytes
                }
                33 => {
                    mutation_caps.maximum_total_state_bytes = report
                        .engine_owned_processor_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                34 => {
                    mutation_caps.maximum_total_retained_payload_bytes =
                        report.engine_owned_retained_payload_bytes
                }
                35 => {
                    mutation_caps.maximum_total_retained_payload_bytes = report
                        .engine_owned_retained_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                36 => {
                    mutation_caps.maximum_total_meter_bytes =
                        report.engine_owned_meter_payload_bytes
                }
                37 => {
                    mutation_caps.maximum_total_meter_bytes = report
                        .engine_owned_meter_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                38 => {
                    mutation_caps.maximum_single_allocation_bytes =
                        report.maximum_single_allocation_bytes
                }
                39 => {
                    mutation_caps.maximum_single_allocation_bytes = report
                        .maximum_single_allocation_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                40 => mutation_caps.maximum_total_meter_items = report.meter_items,
                41 => {
                    mutation_caps.maximum_total_meter_items =
                        report.meter_items.checked_sub(1).expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                42 => mutation_caps.maximum_meter_streams = 1,
                43 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    mutation_caps.maximum_meter_streams = 1;
                    expected.push(diag("builtin.resource.limit", "$.meter_requests"));
                }
                44 => {
                    let limited = if smoothing == 0 { 0 } else { smoothing - 1 };
                    mutation_caps.maximum_smoothing_samples = limited;
                    if smoothing > 0 {
                        expected.push(diag(
                            "builtin.matrix.smoothing",
                            "$.tracks[id=vocal].matrix_or_pan.smoothing_samples",
                        ));
                    }
                }
                45 => {
                    let coefficient = (u32::try_from(value >> 32).expect("masked") as f32
                        / u32::MAX as f32)
                        * 2.0
                        - 1.0;
                    if let MatrixOrPan::Matrix { lr, rl, .. } = &mut model.tracks[0].matrix_or_pan {
                        *lr = coefficient;
                        *rl = -coefficient;
                    }
                }
                46 => {
                    let maximum = builtin_filter_cutoff_maximum_hz(rate)
                        .expect("matrix only uses launch rates");
                    let successor = f32::from_bits(maximum.to_bits() + 1);
                    model.tracks[0].builtins.left.hpf_hz = 0.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    match case % 4 {
                        0 => model.tracks[0].builtins.left.hpf_hz = maximum,
                        1 => model.tracks[0].builtins.left.lpf_hz = maximum,
                        2 => {
                            model.tracks[0].builtins.left.hpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.hpf_hz",
                            ));
                        }
                        3 => {
                            model.tracks[0].builtins.left.lpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.lpf_hz",
                            ));
                        }
                        _ => unreachable!("case remainder is bounded"),
                    }
                }
                47 => target.rl = f32::NEG_INFINITY,
                48 => {
                    model.tracks[0].builtins.right.lpf_hz = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.right.lpf_hz".to_owned(),
                    ));
                }
                _ => unreachable!("frozen class range"),
            }

            let compiled_result = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            );
            if !expected_session.is_empty() {
                let Err(diagnostics) = compiled_result else {
                    panic!("invalid complete session must reject: case={case}, class={class}");
                };
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(observed, expected_session, "case={case}, class={class}");
                continue;
            }
            let compiled = compiled_result
                .expect("complete generated session compiles before builtin preparation");
            let expected = BuiltinDiagnosticSet::sorted(expected);
            let result = prepare_session_builtins(&compiled, &requests, mutation_caps);
            if !expected.0.is_empty() {
                let Err(observed) = result else {
                    panic!("frozen invalid class must reject: case={case}, class={class}");
                };
                assert_eq!(observed, expected, "case={case}, class={class}");
            } else {
                let prepared = match result {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        panic!(
                            "frozen valid preparation class failed: case={case}, class={class}, error={error:?}"
                        )
                    }
                };
                let accepted_report = prepared.resource_report();
                assert!(
                    accepted_report.engine_owned_processor_payload_bytes
                        <= mutation_caps.maximum_total_state_bytes
                );
                assert!(
                    accepted_report.engine_owned_retained_payload_bytes
                        <= mutation_caps.maximum_total_retained_payload_bytes
                );
                assert!(
                    accepted_report.engine_owned_meter_payload_bytes
                        <= mutation_caps.maximum_total_meter_bytes
                );
                assert!(
                    accepted_report.maximum_single_allocation_bytes
                        <= mutation_caps.maximum_single_allocation_bytes
                );
                assert_eq!(
                    accepted_report
                        .retained_layouts()
                        .iter()
                        .try_fold(0_u64, |total, layout| {
                            total.checked_add(
                                layout.size_bytes.checked_mul(layout.allocation_count)?,
                            )
                        }),
                    Some(accepted_report.engine_owned_retained_payload_bytes)
                );

                let strip = model.strips().next().unwrap();
                let parameters = strip_parameters(&strip, own_fader(&strip), u32::MAX)
                    .expect("accepted class parameters");
                let mut chain = BuiltinChain::new(rate, parameters).expect("accepted chain");
                let target_result = chain.set_matrix_target(target);
                if matches!(class, 25 | 26 | 27 | 47) {
                    assert_eq!(target_result, Err(BuiltinParameterError::MatrixCoefficient));
                } else {
                    target_result.expect("valid target");
                }
                let frames = usize::try_from(quantum).expect("supported quantum fits usize");
                match block_probe {
                    1 => {
                        let mut left = Vec::<f32>::new();
                        let mut right = Vec::<f32>::new();
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::EmptyBlock)
                        ));
                    }
                    2 => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames.checked_sub(1).expect("nonzero")];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::LaneLength)
                        ));
                    }
                    3 => {
                        let mut left = [0.0_f32];
                        let mut right = [0.0_f32];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, u64::MAX),
                            Err(BuiltinParameterError::SampleTimeOverflow)
                        ));
                    }
                    4 => {
                        let PreparedMeter {
                            mut accumulator, ..
                        } = MeterAccumulator::prepare(handle(99), base_config, rate)
                            .expect("valid discontinuity meter");
                        accumulator.observe(&[0.0], &[0.0], 0).expect("first block");
                        accumulator
                            .observe(&[0.0], &[0.0], 100)
                            .expect("discontinuous block is bounded and accepted");
                    }
                    _ => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames];
                        let block = DualMonoBlock::new(&mut left, &mut right, u64::from(case))
                            .expect("valid generated block");
                        chain.process_dual_mono(block);
                    }
                }
            }
        }
        assert_eq!(seen_taps.len(), 7);
        assert_eq!(seen_rates, BTreeSet::from([44_100, 48_000, 88_200, 96_000]));
        assert_eq!(seen_quanta, BTreeSet::from([1, 127, 128, 255, 1_024]));
        assert_eq!(
            seen_smoothing,
            BTreeSet::from([0, 1, 2, 127, 128, u32::MAX])
        );
        assert!(classes.into_iter().all(core::convert::identity));
    }

    /// The plan-swap drain applies every pending value past a refused one (issue #1276 D4, #1346
    /// D5), and so does the block drain, which returns the first refusal after the last value. The
    /// bank never drains again after the carry, so a value left in a cell is an acknowledged edit
    /// lost with the retired plan.
    ///
    /// Lane 0 holds a NaN trim (the apply is refused) and an inversion, which the canonical order
    /// applies after it; lane 1 holds a trim. Admission would refuse a NaN trim; it stands in for
    /// any apply that fails.
    ///
    /// Red mutation: let the reader's `drain` stop applying after the first refused apply (apply
    /// only while `first_error` is still `Ok`) -> the block arm leaves lane 0's inversion and
    /// lane 1's trim unapplied. `drain_for_carry` applying nothing turns the carry arm red. The
    /// carry arm's `Err(_) if carry` count-and-continue is not what this test defends: replacing
    /// it with a return stays green, because the drain applies every value either way and
    /// `drain_for_carry` reports a returned error as one refusal, which is the count here.
    #[test]
    fn the_input_drains_apply_every_value_past_a_refused_one() {
        // A `Vec`, so a target without AVX2 iterates one width without a single-element loop.
        let backends = vec![
            #[cfg(target_feature = "avx2")]
            Backend::Simd8,
            Backend::Simd4,
        ];
        for backend in backends {
            for carry in [true, false] {
                input_drain_applies_every_value_past_a_refused_one(backend, carry);
            }
        }
    }

    fn input_drain_applies_every_value_past_a_refused_one(backend: Backend, carry: bool) {
        let width = BankWidth::for_backend(backend).expect("a banking backend");
        let input = || {
            BuiltinChain::new(48_000, BuiltinParameters::default())
                .expect("chain")
                .into_input_builtins()
        };
        let superseded = LiveValuesSuperseded::new();
        let (mut first, first_reader) = input_lane_cells(&superseded);
        let (mut second, second_reader) = input_lane_cells(&superseded);
        let mut gate = TestOnlyGate::new();
        gate.write_published(|stamp| {
            first.write(
                stamp,
                TrackInputRecord::TrimDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: f32::NAN,
                    smoothing_samples: 0,
                },
            );
            first.write(
                stamp,
                TrackInputRecord::PolarityInvert {
                    lanes: BuiltinLaneSelector::Both,
                    inverted: true,
                    smoothing_samples: 0,
                },
            );
            second.write(
                stamp,
                TrackInputRecord::TrimDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -144.0,
                    smoothing_samples: 0,
                },
            );
        });
        let mut controls: Vec<Option<InputReader>> = (0..width.lanes()).map(|_| None).collect();
        controls[0] = Some(first_reader);
        controls[1] = Some(second_reader);
        let mut processor = BuiltinBankProcessor {
            bank: BuiltinInputBank::new(backend, width, vec![input(), input()]).expect("bank"),
            controls: controls.into_boxed_slice(),
            live: [ChannelSymmetryWitness::SYMMETRIC; MAXIMUM_BANK_LANES],
            quiet_at: None,
            process_calls: 0,
            frames_processed: 0,
        };

        if carry {
            assert_eq!(
                processor.drain_for_carry(EVERY_LANE),
                1,
                "the NaN trim is refused and counted"
            );
        } else {
            assert_eq!(
                processor.drain_controls(gate.snapshot(), EVERY_LANE, false),
                Err(BuiltinParameterError::GainDomain),
                "the block drain returns the first refusal"
            );
        }
        for channel in 0..2 {
            assert_eq!(
                processor.bank.trim_signed(0, channel),
                -1.0,
                "lane 0 channel {channel}: the inversion after the refused trim was applied"
            );
            let trim = processor.bank.trim_signed(1, channel);
            assert!(
                trim.abs() < 1.0e-6,
                "lane 1 channel {channel}: the trim after lane 0's refusal was applied: {trim}"
            );
        }
    }

    /// One input-bank processor with lane 0 live, and the writer and gate that feed it.
    fn live_input_processor(
        backend: Backend,
        superseded: &LiveValuesSuperseded,
    ) -> (BuiltinBankProcessor, InputCells, TestOnlyGate) {
        let width = BankWidth::for_backend(backend).expect("a banking backend");
        let input = BuiltinChain::new(48_000, BuiltinParameters::default())
            .expect("chain")
            .into_input_builtins();
        let (cells, reader) = input_lane_cells(superseded);
        let mut controls: Vec<Option<InputReader>> = (0..width.lanes()).map(|_| None).collect();
        controls[0] = Some(reader);
        (
            BuiltinBankProcessor {
                bank: BuiltinInputBank::new(backend, width, vec![input]).expect("bank"),
                controls: controls.into_boxed_slice(),
                live: [ChannelSymmetryWitness::SYMMETRIC; MAXIMUM_BANK_LANES],
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            },
            cells,
            TestOnlyGate::new(),
        )
    }

    /// Both planes of one block of a fixed signal, as bits, through a bank's `process`.
    fn input_bank_bits(bank: &mut BuiltinInputBank) -> Vec<u32> {
        const FRAMES: usize = 192;
        let width = bank.width().lanes() as usize;
        let signal = |plane: f32| -> Vec<f32> {
            (0..FRAMES * width)
                .map(|index| ((index % 23) as f32 - 11.0) * 0.07 + plane)
                .collect()
        };
        let (mut left, mut right) = (signal(0.0), signal(1.0));
        bank.process(&mut left, &mut right, FRAMES as u32);
        left.iter()
            .chain(&right)
            .map(|value| value.to_bits())
            .collect()
    }

    fn input_test_backends() -> Vec<Backend> {
        // A `Vec`, so a target without AVX2 iterates one width without a single-element loop.
        vec![
            #[cfg(target_feature = "avx2")]
            Backend::Simd8,
            Backend::Simd4,
        ]
    }

    /// The input cells drain trim, then polarity, then filter section 0, then section 1, whatever
    /// order they were written in (#1346 D2).
    ///
    /// The oracle is the bank itself, driven by direct calls in that order. Trim and polarity
    /// share one coefficient and the last call's window wins, so with a 0-sample trim window and a
    /// 64-sample polarity window the order moves bits: the reversed order is rendered too, to show
    /// the comparison can see it.
    ///
    /// Red mutation: drain polarity before trim in `InputReader::apply_latest` -> the drained bank
    /// renders the reversed twin's bits, not the canonical twin's.
    #[test]
    fn input_cells_drain_in_the_canonical_order_whatever_the_write_order() {
        let prepared = builtins::prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
        let (low, high) = (prepared.targets[0], prepared.targets[1]);
        for backend in input_test_backends() {
            let superseded = LiveValuesSuperseded::new();
            let (mut processor, mut cells, mut gate) = live_input_processor(backend, &superseded);
            // The reverse of the canonical order, in one transaction.
            gate.write_published(|stamp| {
                cells.write(stamp, TrackInputRecord::PreparedFilter { target: high });
                cells.write(stamp, TrackInputRecord::PreparedFilter { target: low });
                cells.write(
                    stamp,
                    TrackInputRecord::PolarityInvert {
                        lanes: BuiltinLaneSelector::Both,
                        inverted: true,
                        smoothing_samples: 64,
                    },
                );
                cells.write(
                    stamp,
                    TrackInputRecord::TrimDb {
                        lanes: BuiltinLaneSelector::Both,
                        db: -12.0,
                        smoothing_samples: 0,
                    },
                );
            });
            processor
                .begin_block(0, gate.snapshot())
                .expect("the drain");

            let twin = |polarity_first: bool| {
                let width = BankWidth::for_backend(backend).expect("a banking backend");
                let input = BuiltinChain::new(48_000, BuiltinParameters::default())
                    .expect("chain")
                    .into_input_builtins();
                let mut bank = BuiltinInputBank::new(backend, width, vec![input]).expect("bank");
                let trim = |bank: &mut BuiltinInputBank| {
                    bank.set_trim_db(0, BuiltinLaneSelector::Both, -12.0, 0)
                        .expect("trim");
                };
                let polarity = |bank: &mut BuiltinInputBank| {
                    bank.set_polarity_invert(0, BuiltinLaneSelector::Both, true, 64)
                        .expect("polarity");
                };
                if polarity_first {
                    polarity(&mut bank);
                    trim(&mut bank);
                } else {
                    trim(&mut bank);
                    polarity(&mut bank);
                }
                bank.apply_prepared_filter(0, low).expect("section 0");
                bank.apply_prepared_filter(0, high).expect("section 1");
                input_bank_bits(&mut bank)
            };
            let canonical = twin(false);
            assert_ne!(
                canonical,
                twin(true),
                "{backend:?}: the order must be visible in the bits, or the comparison proves nothing"
            );
            assert_eq!(
                input_bank_bits(&mut processor.bank),
                canonical,
                "{backend:?}: the drain applied the cells in the canonical order"
            );
        }
    }

    /// The witness folds from the cells' final contents (#1346 D3): equal channel words are one
    /// `Both` record and keep the lane's `LIVE` term, however they were written; unequal words, a
    /// lone channel, and a differing ramp window clear it.
    ///
    /// Red mutation: admit a fixed `Desymmetrize` for every drained value -> the first two cases
    /// (a `Both` write, and the two channels written separately to equal words) clear `LIVE` and
    /// retire the mono collapse. Admit a fixed `Preserve` -> the last three cases keep `LIVE`.
    #[test]
    fn the_input_witness_folds_from_the_final_cell_contents() {
        fn trim(lanes: BuiltinLaneSelector, db: f32, smoothing_samples: u32) -> TrackInputRecord {
            TrackInputRecord::TrimDb {
                lanes,
                db,
                smoothing_samples,
            }
        }
        type Writes = fn(&mut InputCells, LiveStamp<'_>);
        let cases: [(&str, bool, Writes); 5] = [
            ("a Both write", true, |cells, stamp| {
                cells.write(stamp, trim(BuiltinLaneSelector::Both, -6.0, 32));
            }),
            (
                "the channels written separately to equal words",
                true,
                |cells, stamp| {
                    cells.write(stamp, trim(BuiltinLaneSelector::Left, -6.0, 32));
                    cells.write(stamp, trim(BuiltinLaneSelector::Right, -6.0, 32));
                },
            ),
            ("unequal values", false, |cells, stamp| {
                cells.write(stamp, trim(BuiltinLaneSelector::Left, -6.0, 32));
                cells.write(stamp, trim(BuiltinLaneSelector::Right, -9.0, 32));
            }),
            (
                "equal values over different windows",
                false,
                |cells, stamp| {
                    cells.write(stamp, trim(BuiltinLaneSelector::Left, -6.0, 32));
                    cells.write(stamp, trim(BuiltinLaneSelector::Right, -6.0, 64));
                },
            ),
            ("a lone channel", false, |cells, stamp| {
                cells.write(stamp, trim(BuiltinLaneSelector::Right, -6.0, 32));
            }),
        ];
        for backend in input_test_backends() {
            for (label, preserved, writes) in cases {
                let superseded = LiveValuesSuperseded::new();
                let (mut processor, mut cells, mut gate) =
                    live_input_processor(backend, &superseded);
                gate.write_published(|stamp| writes(&mut cells, stamp));
                processor
                    .begin_block(0, gate.snapshot())
                    .expect("the drain");
                assert_eq!(
                    processor.live[0].holds(ChannelSymmetryWitness::LIVE),
                    preserved,
                    "{backend:?}: {label}"
                );
            }
        }
    }

    /// Any number of edits between two blocks is accepted, the block applies the last of each
    /// cell, and the session count records every replaced value (#1346 gate 1, at the bank).
    ///
    /// 30 `Both` trim edits and then 30 `Both` HPF edits, each its own transaction, then one
    /// block. A twin that wrote only the last of each renders the same bits. Each `Both` edit
    /// writes two cells, so each kind adds `2 x 29` replaced writes.
    ///
    /// Red mutation: count the replaced writes of the left cell only -> 58 instead of 116. Apply
    /// the oldest value instead of the newest -> the bits differ from the twin's.
    #[test]
    fn many_input_edits_before_one_block_apply_the_last_and_count_each_replaced_value() {
        let hpf = |index: usize| {
            let pair =
                builtins::prepare_input_filter_pair(48_000, 100.0 + 10.0 * index as f32, 8_000.0)
                    .expect("pair");
            TrackInputRecord::PreparedFilter {
                target: pair.targets[0],
            }
        };
        let trim = |index: usize| TrackInputRecord::TrimDb {
            lanes: BuiltinLaneSelector::Both,
            db: -1.0 - index as f32,
            smoothing_samples: 16,
        };
        for backend in input_test_backends() {
            let many = LiveValuesSuperseded::new();
            let (mut processor, mut cells, mut gate) = live_input_processor(backend, &many);
            for index in 0..30 {
                gate.write_published(|stamp| cells.write(stamp, trim(index)));
            }
            for index in 0..30 {
                gate.write_published(|stamp| cells.write(stamp, hpf(index)));
            }
            processor
                .begin_block(0, gate.snapshot())
                .expect("the drain");

            let single = LiveValuesSuperseded::new();
            let (mut twin, mut twin_cells, mut twin_gate) = live_input_processor(backend, &single);
            twin_gate.write_published(|stamp| {
                twin_cells.write(stamp, trim(29));
                twin_cells.write(stamp, hpf(29));
            });
            twin.begin_block(0, twin_gate.snapshot())
                .expect("the drain");

            assert_eq!(
                many.get(),
                116,
                "{backend:?}: 2 x 29 for each of the two kinds"
            );
            assert_eq!(single.get(), 0, "{backend:?}: nothing was replaced");
            assert_eq!(
                input_bank_bits(&mut processor.bank),
                input_bank_bits(&mut twin.bank),
                "{backend:?}: the block applied the last value of each cell"
            );
        }
    }

    /// A block reads each cell under its own snapshot (#1346 D2): a value stamped above it waits
    /// for the block whose snapshot covers it, and the earlier one applies now.
    ///
    /// Red mutation: drain with `LiveSnapshot::ALL` in `begin_block` -> the first block applies
    /// the second revision's trim too.
    #[test]
    fn the_input_drain_reads_under_the_blocks_snapshot() {
        for backend in input_test_backends() {
            let superseded = LiveValuesSuperseded::new();
            let (mut processor, mut cells, mut gate) = live_input_processor(backend, &superseded);
            let trim = |db| TrackInputRecord::TrimDb {
                lanes: BuiltinLaneSelector::Both,
                db,
                smoothing_samples: 0,
            };
            gate.write_published(|stamp| cells.write(stamp, trim(-6.0)));
            let first = gate.snapshot();
            gate.write_published(|stamp| cells.write(stamp, trim(-12.0)));
            let second = gate.snapshot();

            // The coefficient a trim of `db` settles to, from a bank that was told it directly.
            let settled = |db: f32| {
                let width = BankWidth::for_backend(backend).expect("a banking backend");
                let input = BuiltinChain::new(48_000, BuiltinParameters::default())
                    .expect("chain")
                    .into_input_builtins();
                let mut bank = BuiltinInputBank::new(backend, width, vec![input]).expect("bank");
                bank.set_trim_db(0, BuiltinLaneSelector::Both, db, 0)
                    .expect("trim");
                bank.trim_signed(0, 0).to_bits()
            };
            processor.begin_block(0, first).expect("the first block");
            let after_first = processor.bank.trim_signed(0, 0).to_bits();
            processor.begin_block(64, second).expect("the second block");
            let after_second = processor.bank.trim_signed(0, 0).to_bits();
            assert_eq!(
                after_first,
                settled(-6.0),
                "{backend:?}: the first block applied revision 1"
            );
            assert_eq!(
                after_second,
                settled(-12.0),
                "{backend:?}: the second block applied revision 2"
            );
            assert_eq!(superseded.get(), 0, "no value was replaced unread");
            // Issue #1778: a block that finds nothing pending marks the publication, and the
            // next block under it skips the walk.
            let _ = test_only_take_quiet_walk_skips();
            processor.begin_block(128, second).expect("a quiet block");
            processor.begin_block(192, second).expect("a skipped block");
            assert_eq!(test_only_take_quiet_walk_skips(), [1, 0, 0], "{backend:?}");
        }
    }

    /// Each builtin bank reports exactly the lanes that hold a reader (#1346; the lane mask
    /// `rack`'s `BankChain::new` slot guard reads, #1504 D3). The guard's first disjunct is
    /// `live_lanes() == 0`, so a bank that reports 0 for a lane that does drain makes it vacuous for
    /// that bank, and nothing else in the tree notices: no reachable builtin configuration can
    /// make the guard fire.
    ///
    /// Three shapes: the input bank, the fader bank and the matrix bank, each with readers on lanes
    /// 0 and 2. A bank with no reader reports 0 (an `input_lane: false` strip).
    ///
    /// Red mutation: make any one of the three `live_lanes` return 0 -> that shape's assertion
    /// fails; report every lane instead of the readers' lanes -> the exact masks fail.
    #[test]
    fn every_builtin_bank_reports_exactly_the_lanes_that_hold_a_reader() {
        fn sparse<T>(lanes: usize, readers: Vec<(usize, T)>) -> Box<[Option<T>]> {
            let mut controls: Vec<Option<T>> = (0..lanes).map(|_| None).collect();
            for (lane, reader) in readers {
                controls[lane] = Some(reader);
            }
            controls.into_boxed_slice()
        }
        for backend in input_test_backends() {
            let width = BankWidth::for_backend(backend).expect("a banking backend");
            let lanes = width.lanes() as usize;
            let superseded = LiveValuesSuperseded::new();
            let parameters = vec![BuiltinParameters::default(); 4];
            let input = |controls: Box<[Option<InputReader>]>| BuiltinBankProcessor {
                bank: BuiltinInputBank::new(
                    backend,
                    width,
                    (0..4)
                        .map(|_| {
                            BuiltinChain::new(48_000, BuiltinParameters::default())
                                .expect("chain")
                                .into_input_builtins()
                        })
                        .collect(),
                )
                .expect("input bank"),
                controls,
                live: [ChannelSymmetryWitness::SYMMETRIC; MAXIMUM_BANK_LANES],
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            };
            let fader = |controls: Box<[Option<FaderReader>]>| FaderBankProcessor {
                bank: BuiltinFaderBank::new(backend, width, parameters.clone())
                    .expect("fader bank"),
                controls,
                automation: Box::default(),
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            };
            let matrix = |controls: Box<[Option<MatrixReader>]>| MatrixBankProcessor {
                bank: BuiltinMatrixBank::new(
                    backend,
                    width,
                    parameters.iter().map(|p| (p.matrix, 0)).collect(),
                )
                .expect("matrix bank"),
                controls,
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            };

            let input_reader = || input_lane_cells(&superseded).1;
            let fader_reader = || fader_lane(&superseded).1;
            let matrix_reader = || matrix_lane(&superseded).1;

            assert_eq!(
                input(sparse(
                    lanes,
                    vec![(0, input_reader()), (2, input_reader())]
                ))
                .live_lanes(),
                0b101,
                "{backend:?}: the input bank"
            );
            assert_eq!(
                fader(sparse(
                    lanes,
                    vec![(0, fader_reader()), (2, fader_reader())]
                ))
                .live_lanes(),
                0b101,
                "{backend:?}: the fader bank"
            );
            assert_eq!(
                matrix(sparse(
                    lanes,
                    vec![(0, matrix_reader()), (2, matrix_reader())]
                ))
                .live_lanes(),
                0b101,
                "{backend:?}: the matrix bank"
            );
            // A strip prepared without its live lane holds no reader: nothing to drain.
            assert_eq!(input(sparse::<InputReader>(lanes, vec![])).live_lanes(), 0);
            assert_eq!(fader(sparse::<FaderReader>(lanes, vec![])).live_lanes(), 0);
            assert_eq!(
                matrix(sparse::<MatrixReader>(lanes, vec![])).live_lanes(),
                0
            );
        }
    }

    /// The per-node live-control input processor: the arm a scalar-backend host binds (#210 phase
    /// 3).
    ///
    /// `Backend::current()` is a compile-time constant, so on every architecture the workspace's
    /// tests run on the strip is banked and `LiveControlInputProcessor` is unreachable from an
    /// end-to-end fixture. It is not unreachable in *production* -- a target with no SIMD binds
    /// it, and so does any lowering the planner leaves unbanked -- so it is driven directly here,
    /// through the same `GraphRuntimeProcessor::process` the runtime calls.
    ///
    /// The two facts it owes, and the two the banked drain owes for the same reason: the record
    /// reaches the section, and the witness records what the record did.
    ///
    /// Red mutation: make the drain loop `continue` past every record without applying it -> the
    /// coefficient never moves. Red mutation: drop `witness.admit(&record)` from
    /// `InputReader::apply_latest` -> the per-lane arm keeps claiming symmetry.
    #[test]
    fn the_per_node_live_control_input_processor_drains_and_folds_its_witness() {
        let parameters = BuiltinParameters {
            left: ChannelParameters {
                trim_db: 0.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                trim_db: 0.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        };
        let build = || {
            let input = BuiltinChain::new(48_000, parameters)
                .expect("chain")
                .into_input_builtins();
            let (cells, control) = input_lane_cells(&LiveValuesSuperseded::new());
            (
                (cells, TestOnlyGate::new()),
                LiveControlInputProcessor {
                    input,
                    control,
                    live: ChannelSymmetryWitness::SYMMETRIC,
                },
            )
        };

        // A `Both` retarget moves the coefficient and preserves every term.
        let ((mut cells, mut gate), mut processor) = build();
        gate.write_published(|stamp| {
            cells.write(
                stamp,
                TrackInputRecord::TrimDb {
                    lanes: BuiltinLaneSelector::Both,
                    db: -144.0,
                    smoothing_samples: 0,
                },
            );
        });
        let mut left = [1.0_f32; 4];
        let mut right = [1.0_f32; 4];
        processor
            .process(GraphBindingBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
                live_snapshot: LiveSnapshot::ALL,
            })
            .expect("render");
        assert!(
            left.iter().all(|value| *value < 1.0e-6) && right.iter().all(|value| *value < 1.0e-6),
            "the record reached the section: {left:?} {right:?}"
        );
        assert!(
            processor.channel_symmetry().eligible(),
            "a `Both` retarget is symmetry-preserving on the per-node arm too"
        );

        // A prepared filter target uses the same cells and preserves the same witness
        // rule. The first sample is the old current coefficient; sample A+64 reaches the target.
        let ((mut cells, mut gate), mut processor) = build();
        let prepared = builtins::prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
        gate.write_published(|stamp| {
            cells.write(
                stamp,
                TrackInputRecord::PreparedFilter {
                    target: builtins::PreparedInputFilterTarget {
                        lanes: BuiltinLaneSelector::Both,
                        ..prepared.targets[0]
                    },
                },
            );
        });
        let mut left = [1.0_f32; 65];
        let mut right = [1.0_f32; 65];
        processor
            .process(GraphBindingBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
                live_snapshot: LiveSnapshot::ALL,
            })
            .expect("render");
        assert_eq!(left[0].to_bits(), 1.0_f32.to_bits());
        assert_ne!(left[64].to_bits(), left[0].to_bits());
        assert_eq!(
            left.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
            right
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        assert!(
            processor.channel_symmetry().eligible(),
            "a Both prepared filter target preserves the scalar witness"
        );

        // A per-lane retarget declines the `LIVE` term and moves one plane only.
        let ((mut cells, mut gate), mut processor) = build();
        gate.write_published(|stamp| {
            cells.write(
                stamp,
                TrackInputRecord::PolarityInvert {
                    lanes: BuiltinLaneSelector::Right,
                    inverted: true,
                    smoothing_samples: 0,
                },
            );
        });
        let mut left = [1.0_f32; 4];
        let mut right = [1.0_f32; 4];
        processor
            .process(GraphBindingBlock {
                left: &mut left,
                right: &mut right,
                first_sample: 0,
                live_snapshot: LiveSnapshot::ALL,
            })
            .expect("render");
        assert!(
            left.iter().all(|value| *value > 0.0),
            "the left plane is untouched"
        );
        assert!(
            right.iter().all(|value| *value < 0.0),
            "the right plane is inverted"
        );
        assert!(
            !processor
                .channel_symmetry()
                .holds(ChannelSymmetryWitness::LIVE),
            "a one-channel write clears the LIVE term on the per-node arm"
        );
    }

    /// Issue #212: a prepared session that lost a *strip* is caught, not only one that lost a
    /// binding.
    ///
    /// Preparation owns three stages per track in two places -- one `GraphNodeBinding` for the
    /// post-input stage, and one `StripPreparation` carrying the fader and the matrix -- while the
    /// seal still records three stages per track flat. So `processors_match` has to count both,
    /// and this is the half that did not exist before the strip's binding form became a lowering
    /// decision. Without the strip term, a session missing a whole track's fader and matrix would
    /// validate clean.
    #[test]
    fn a_prepared_session_that_lost_a_strip_fails_validation() {
        let session = session();
        let mut prepared =
            prepare_session_builtins(&session, &[], caps()).expect("prepared builtins");
        assert!(
            prepared.validate_for_session(&session).0.is_empty(),
            "the untouched preparation validates"
        );
        assert!(prepared.strips.pop().is_some(), "the fixture has a strip");
        let diagnostics = prepared.validate_for_session(&session);
        assert!(
            diagnostics
                .0
                .iter()
                .any(|diagnostic| diagnostic.code == "builtin.prepared.processor_set"),
            "a lost strip must be diagnosed as a processor-set mismatch, got {diagnostics:?}"
        );
    }

    /// Issue #137 D1: a live-control request is validated like a meter request, sealed
    /// like a meter consumer, and charges only the tracks that asked for one.
    ///
    /// Red mutation: delete the `control_tracks.insert` / `known_tracks.contains` legs in
    /// `prepare_session_builtins_with_live_controls` -> the duplicate and unknown-track requests
    /// are accepted, and the assertions below on `builtin.control.duplicate` /
    /// `builtin.control.unknown_track` fail with an `Ok` preparation.
    #[test]
    fn live_control_requests_are_validated_sealed_and_charged_per_track() {
        let compiled = session();
        let track = compiled.normalized_model().tracks[0].id.as_str().to_owned();

        let baseline = prepare_session_builtins(&compiled, &[], caps()).expect("baseline");
        let attached = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            &[],
            &[TrackControlRequest {
                track_id: track.clone(),
                superseded: LiveValuesSuperseded::default(),
                input_lane: true,
            }],
            caps(),
        )
        .expect("attached live controls");
        assert_eq!(baseline.track_control_count(), 0);
        assert_eq!(attached.track_control_count(), 1);
        assert!(
            attached
                .resource_report()
                .engine_owned_processor_payload_bytes
                > baseline
                    .resource_report()
                    .engine_owned_processor_payload_bytes,
            "an attached channel is charged"
        );
        assert!(
            attached.validate_for_session(&compiled).0.is_empty(),
            "the control seal matches the producers it was built from"
        );
        assert_eq!(
            attached.processor_count(),
            baseline.processor_count(),
            "live controls change no processor count, only one processor's type"
        );

        let duplicate = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            &[],
            &[
                TrackControlRequest {
                    track_id: track.clone(),
                    superseded: LiveValuesSuperseded::default(),
                    input_lane: true,
                },
                TrackControlRequest {
                    track_id: track.clone(),
                    superseded: LiveValuesSuperseded::default(),
                    input_lane: true,
                },
            ],
            caps(),
        )
        .map(|_| ())
        .expect_err("a track may hold one control channel");
        assert!(
            duplicate
                .0
                .iter()
                .any(|value| value.code == "builtin.control.duplicate"),
            "{duplicate:?}"
        );

        let unknown = prepare_session_builtins_with_live_controls(
            &compiled,
            &effective_of(&compiled),
            &[],
            &[TrackControlRequest {
                track_id: "no-such-track".to_owned(),
                superseded: LiveValuesSuperseded::default(),
                input_lane: true,
            }],
            caps(),
        )
        .map(|_| ())
        .expect_err("an undeclared track has no channel");
        assert!(
            unknown
                .0
                .iter()
                .any(|value| value.code == "builtin.control.unknown_track"),
            "{unknown:?}"
        );
    }

    /// Decision 17, C7: preparation bakes the `Effective` it is given, not a composition of its
    /// own. A solo overlay mutes the vocal in the baked fader and the stored mutes, and an
    /// `Effective` of another strip or route count is refused at `$.effective`.
    ///
    /// Red if preparation recomputes the fader from the model (the solo would be lost), if
    /// `effective_route_follow` is not the follow gate it was given, or if a mismatched shape is truncated
    /// by the zip instead of refused.
    #[test]
    fn preparation_bakes_the_effective_value_it_is_given() {
        let compiled = session();
        let soloed_elsewhere =
            controls::MonitorOverlay::with_soloed([
                session::StableId::parse("not-a-strip").expect("stable ID")
            ]);
        // A solo on an unknown ID is no solo; a hand-made effective value is baked as given.
        let mut effective = controls::effective(compiled.normalized_model(), &soloed_elsewhere);
        effective.strips[0].mute = [true, false];
        effective.strips[0].fader_db = [-12.5, -3.0];
        assert!(!effective.routes.is_empty(), "the fixture declares a route");
        effective.routes[0].follow = [true, false];
        let prepared =
            prepare_session_builtins_with_live_controls(&compiled, &effective, &[], &[], caps())
                .expect("prepare");
        assert_eq!(
            prepared.effective_route_follow(),
            effective
                .routes
                .iter()
                .map(|route| route.follow)
                .collect::<Vec<_>>()
        );
        let parameters = prepared.strips[0].parameters;
        assert_eq!(
            (parameters.left.fader_db, parameters.right.fader_db),
            (-12.5, -3.0)
        );

        let strips = effective.strips.len();
        let routes = effective.routes.len();
        for wrong in [
            controls::Effective::with_shape(strips + 1, routes),
            controls::Effective::with_shape(strips, routes + 1),
        ] {
            let refused =
                prepare_session_builtins_with_live_controls(&compiled, &wrong, &[], &[], caps())
                    .err()
                    .expect("refused");
            assert!(refused.0.iter().any(
                |value| value.code == "builtin.effective.shape" && value.path == "$.effective"
            ));
        }
    }

    /// #1242 gate 7: a member fader outside the fader domain still refuses at its own path. Track
    /// `vocal` sits at `fader.left_db = 24.5` in VCA `drums` at -6 dB, whose effective value
    /// (18.5) is in domain; preparation refuses `builtin.gain.domain` at
    /// `$.tracks[id=vocal].fader.left_db`, as it does with no VCA.
    ///
    /// Red if the domain preflight checks the clamped effective value, which would accept it.
    #[test]
    fn a_vca_member_fader_outside_its_domain_refuses_at_its_own_path() {
        let document = include_str!("../../../fixtures/session/v1/canonical.json");
        let mut model = parse_session_json(document).expect("parse");
        model.tracks[0].fader.left_db = 24.5;
        model.vcas = vec![session::Vca {
            id: session::StableId::parse("drums").expect("stable ID"),
            fader: session::DualMonoFader {
                left_db: -6.0,
                right_db: -6.0,
                left_mute: false,
                right_mute: false,
            },
            members: vec![model.tracks[0].id.clone()],
        }];
        assert_eq!(
            controls::effective(&model, &controls::MonitorOverlay::default()).strips[0].fader_db[0],
            18.5
        );
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("the session compiles");
        let refused = prepare_session_builtins(&compiled, &[], caps())
            .map(|_| ())
            .expect_err("an out-of-domain member fader refuses");
        assert_eq!(
            refused,
            BuiltinDiagnosticSet::sorted(vec![diag(
                "builtin.gain.domain",
                "$.tracks[id=vocal].fader.left_db",
            )])
        );
    }

    /// #1655 V2: one session's fader banks over its strips, stored automation bound as a fader
    /// bank binds it, rendered from timeline 0 with every input sample `1.0` (so each output
    /// sample is the lane's stage gain) and `a(n) = 0`. The jump lengths `[fader, mute,
    /// vca_offset, vca_mute]` are set per block as `set_block_time` sets them.
    /// Per block, per strip, its left and right gains as bits.
    type RigBlocks = Vec<Vec<[Vec<u32>; 2]>>;

    struct VcaRig {
        processor: FaderBankProcessor,
        plan: Option<FaderAutomationPlan>,
        lanes: usize,
        jumps: [u32; 4],
        first: u64,
    }

    fn vca_caps() -> CompileCaps {
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        }
    }

    fn vca_rig(model: &session::SessionModel, backend: Backend, jumps: [u32; 4]) -> VcaRig {
        let compiled = compile_session(model, vca_caps()).expect("the session compiles");
        let effective = effective_of(&compiled);
        let _canonical = lane::CanonicalFpEnv::enter();
        let plan = prepare_fader_automation(&compiled, &effective).expect("prepared");
        let width = BankWidth::for_backend(backend).expect("a bank width");
        let model = compiled.normalized_model();
        let mut parameters = Vec::new();
        let mut automation = Vec::new();
        for (strip, values) in model.strips().zip(effective.strips.iter()) {
            let id = strip.id.as_str();
            let prepared = plan
                .as_ref()
                .map_or((values.fader_db, values.mute), |plan| {
                    (
                        plan.fader_db(id, values.fader_db),
                        plan.mute(id, values.mute),
                    )
                });
            parameters.push(
                strip_parameters(&strip, prepared, caps().maximum_smoothing_samples)
                    .expect("parameters"),
            );
            automation.push(plan.as_ref().and_then(|plan| {
                plan.strip(id).map(|cells| {
                    LaneFaderAutomation::new(&plan.program, cells).expect("the lane binds")
                })
            }));
        }
        let lanes = parameters.len();
        assert!(
            lanes <= width.lanes() as usize,
            "one bank holds the session"
        );
        automation.resize_with(width.lanes() as usize, || None);
        let automated = automation.iter().any(Option::is_some);
        VcaRig {
            processor: FaderBankProcessor {
                bank: BuiltinFaderBank::new(backend, width, parameters).expect("bank"),
                controls: (0..width.lanes()).map(|_| None).collect(),
                automation: if automated {
                    automation.into_boxed_slice()
                } else {
                    Box::default()
                },
                quiet_at: None,
                process_calls: 0,
                frames_processed: 0,
            },
            plan,
            lanes,
            jumps,
            first: 0,
        }
    }

    impl VcaRig {
        /// One block of `frames`: per strip, its left and right gains, as bits.
        fn block(&mut self, frames: u32) -> Vec<[Vec<u32>; 2]> {
            let [fader, mute, vca_offset, vca_mute] = self.jumps;
            for slot in self.processor.automation.iter_mut().flatten() {
                slot.span = Some(TimelineSpan {
                    start: i64::try_from(self.first).expect("time"),
                    step: None,
                });
                slot.fader_jump = Some(fader);
                slot.mute_jump = Some(mute);
                slot.vca_jumps = Some([vca_offset, vca_mute]);
            }
            let width = self.processor.bank.width().lanes() as usize;
            let samples = width * frames as usize;
            let (mut left, mut right) = (vec![1.0_f32; samples], vec![1.0_f32; samples]);
            {
                let _canonical = lane::CanonicalFpEnv::enter();
                self.processor
                    .process(&mut left, &mut right, frames, self.first)
                    .expect("process");
            }
            self.first += u64::from(frames);
            (0..self.lanes)
                .map(|lane| {
                    [&left, &right].map(|plane| {
                        (0..frames as usize)
                            .map(|sample| plane[sample * width + lane].to_bits())
                            .collect()
                    })
                })
                .collect()
        }

        /// `blocks` blocks of `frames`, and every stored event the driver gave the stage.
        fn run(&mut self, blocks: u32, frames: u32) -> (RigBlocks, Vec<TestOnlyFaderEvent>) {
            let _ = test_only_take_fader_events();
            let mut out = Vec::new();
            let mut events = Vec::new();
            for _ in 0..blocks {
                out.push(self.block(frames));
                let (taken, overflow) = test_only_take_fader_events();
                assert!(!overflow, "the event probe holds one block");
                events.extend(taken);
            }
            (out, events)
        }
    }

    fn vca_model(tracks: usize) -> session::SessionModel {
        let mut model = n_track_session(tracks).normalized_model().clone();
        model.vcas.clear();
        model.automation.clear();
        model
    }

    fn stable(id: &str) -> session::StableId {
        session::StableId::parse(id).expect("stable ID")
    }

    fn vca_of(id: &str, offset_db: f32, members: &[&str]) -> session::Vca {
        session::Vca {
            id: stable(id),
            fader: session::DualMonoFader {
                left_db: offset_db,
                right_db: offset_db,
                left_mute: false,
                right_mute: false,
            },
            members: members.iter().map(|member| stable(member)).collect(),
        }
    }

    fn vca_segment(
        shape: session::AutomationShape,
        span: (u64, u64),
        values: (f32, f32),
        unit: session::ParameterUnit,
    ) -> session::AutomationSegment {
        session::AutomationSegment {
            shape,
            start_sample: span.0,
            end_sample: span.1,
            start_value: values.0,
            end_value: values.1,
            unit,
        }
    }

    /// A `dB` or `0`/`1` step table: `before` until `at`, then `after` to 100,000.
    fn step_table(
        at: u64,
        before: f32,
        after: f32,
        unit: session::ParameterUnit,
    ) -> Vec<session::AutomationSegment> {
        vec![
            vca_segment(
                session::AutomationShape::Step,
                (0, at),
                (before, before),
                unit,
            ),
            vca_segment(
                session::AutomationShape::Step,
                (at, 100_000),
                (after, after),
                unit,
            ),
        ]
    }

    /// A linear `dB` ride from `from` to `to` over `[0, end)`.
    fn ride_table(end: u64, from: f32, to: f32) -> Vec<session::AutomationSegment> {
        vec![vca_segment(
            session::AutomationShape::Linear,
            (0, end),
            (from, to),
            session::ParameterUnit::Db,
        )]
    }

    fn stored(
        name: &str,
        owner: session::Owner,
        control: u32,
        lane: session::ParameterChannel,
        segments: Vec<session::AutomationSegment>,
    ) -> session::Automation {
        let section = match owner {
            session::Owner::Vca(_) => session::Section::Own,
            _ => session::Section::Builtins,
        };
        session::Automation {
            id: stable(name),
            target: session::ControlAddress {
                owner,
                section,
                control,
                lane,
            },
            segments,
            offset: session::AutomationOffset::default(),
        }
    }

    fn vca_owner(id: &str) -> session::Owner {
        session::Owner::Vca(stable(id))
    }

    fn strip_owner(index: usize) -> session::Owner {
        session::Owner::Strip(stable(&track_name(index)))
    }

    /// #1655 gate 1 at the fader stage (V2): a flat VCA `offset_db` ride at `-6` dB and a flat VCA
    /// mute at `0` render, bit for bit, the session with those static values, for a member with
    /// its own fader ride (`t01` left) and for lanes with none (`t00`, `t01` right). The ridden
    /// VCA's static offset (`+3`, its fallback) is in no sum.
    ///
    /// Red if a composed channel adds the VCA curve with other arithmetic than the static path
    /// (another order, an `f32` sum, the fallback in `S`), if its prepared gain differs, or if a
    /// flat curve emits a move.
    #[test]
    fn a_flat_vca_ride_renders_the_static_vca_offset_bits() {
        for backend in input_test_backends() {
            let own = stored(
                "own-ride",
                strip_owner(1),
                5,
                session::ParameterChannel::Left,
                ride_table(4_096, -6.5, -2.0),
            );
            let mut fixed = vca_model(2);
            fixed.vcas = vec![vca_of("v", -6.0, &["t00", "t01"])];
            fixed.automation = vec![own.clone()];
            let mut ridden = fixed.clone();
            ridden.vcas = vec![vca_of("v", 3.0, &["t00", "t01"])];
            ridden.automation = vec![
                own,
                stored(
                    "vca-ride",
                    vca_owner("v"),
                    1,
                    session::ParameterChannel::Both,
                    ride_table(100_000, -6.0, -6.0),
                ),
                stored(
                    "vca-mute",
                    vca_owner("v"),
                    2,
                    session::ParameterChannel::Both,
                    step_table(50_000, 0.0, 0.0, session::ParameterUnit::Linear),
                ),
            ];
            let mut fixed = vca_rig(&fixed, backend, [480; 4]);
            let mut ridden = vca_rig(&ridden, backend, [480; 4]);
            let plan = ridden.plan.as_ref().expect("cells");
            let t00 = plan.strip(&track_name(0)).expect("t00 is an event lane");
            assert!(
                t00.fader
                    .iter()
                    .flatten()
                    .all(|cell| cell.cell.is_none() && cell.vca.len() == 1)
            );
            assert!(
                t00.mute
                    .iter()
                    .flatten()
                    .all(|cell| cell.cell.is_none() && cell.vca.len() == 1)
            );
            let (fixed_blocks, _) = fixed.run(40, 128);
            // #1781: the fader processor's rest mask keeps the settled unautomated lane (`t00`)
            // and declines the automated one (`t01`), whatever its curve does.
            assert_eq!(
                fixed.processor.builtin_lanes_at_rest() & 0b11,
                0b01,
                "an automated fader lane is never at rest: {backend:?}"
            );
            let (ridden, events) = ridden.run(40, 128);
            assert_eq!(fixed_blocks, ridden, "{backend:?}");
            assert!(
                events
                    .iter()
                    .all(|event| event.lane == 1 && event.channel == 0 && !event.mute),
                "only the member's own ride moves: {backend:?}"
            );
        }
    }

    /// #1655 D2 (V2): every grid event of a member lane a linear VCA ride reaches is
    /// `vca_compose_db(own, S + c)` at the grid ramp's completion, with `own` the member's own
    /// ride (`t01` left) or its static fader dB (`t00`, `t01` right), and `S` its offset sum.
    ///
    /// Red if a composed channel reads the VCA curve at another time than its own, composes with
    /// another own value, or drops a grid event.
    #[test]
    fn a_vca_ride_composes_with_each_members_own_value_at_every_grid_event() {
        for backend in input_test_backends() {
            let own_table = ride_table(4_096, -6.5, -2.0);
            let vca_table = ride_table(8_192, -12.0, 0.0);
            let mut model = vca_model(2);
            model.vcas = vec![vca_of("v", 0.0, &["t00", "t01"])];
            model.automation = vec![
                stored(
                    "own-ride",
                    strip_owner(1),
                    5,
                    session::ParameterChannel::Left,
                    own_table.clone(),
                ),
                stored(
                    "vca-ride",
                    vca_owner("v"),
                    1,
                    session::ParameterChannel::Both,
                    vca_table.clone(),
                ),
            ];
            let stored_segments = |table: &[session::AutomationSegment]| {
                table
                    .iter()
                    .map(|segment| automation::Segment {
                        start: segment.start_sample,
                        end: segment.end_sample,
                        start_value: segment.start_value,
                        end_value: segment.end_value,
                        shape: automation::Shape::Linear,
                    })
                    .collect::<Vec<_>>()
            };
            let (own_curve, vca_curve) = (stored_segments(&own_table), stored_segments(&vca_table));
            let mut rig = vca_rig(&model, backend, [480; 4]);
            let static_db = |track: usize, channel: usize| {
                let strip = &model.tracks[track].fader;
                [strip.left_db, strip.right_db][channel]
            };
            let (_, events) = rig.run(24, 128);
            let mut checked = [[0_usize; 2]; 2];
            for event in events.iter().filter(|event| !event.mute && !event.held) {
                assert_eq!(event.kind, automation::EventKind::Grid, "{event:?}");
                let at = i64::try_from(event.sample).expect("time") + 63;
                if at >= 4_096 {
                    continue;
                }
                let own = if (event.lane, event.channel) == (1, 0) {
                    automation::value_at(&own_curve, at)
                } else {
                    static_db(event.lane, event.channel)
                };
                let expected = composed_db(
                    own,
                    0.0,
                    core::iter::once(automation::value_at(&vca_curve, at)),
                );
                assert_eq!(
                    event.value.to_bits(),
                    expected.to_bits(),
                    "{event:?} {backend:?}"
                );
                checked[event.lane][event.channel] += 1;
            }
            assert!(
                checked.iter().flatten().all(|&count| count >= 40),
                "every member lane moves on the grid: {checked:?}"
            );
        }
    }

    /// The non-held fader events of lane 0's left channel from sample 1,000 on, as `(sample,
    /// kind, ramp)`.
    fn moves_after_step(events: &[TestOnlyFaderEvent]) -> Vec<(u64, automation::EventKind, u32)> {
        events
            .iter()
            .filter(|event| {
                (event.lane, event.channel) == (0, 0)
                    && !event.mute
                    && !event.held
                    && event.sample >= 1_000
            })
            .map(|event| (event.sample, event.kind, event.ramp))
            .collect()
    }

    /// #1655 R19-2 (V2): a VCA step inside a member's own grid ride keeps its jump length `J`,
    /// and a member's own step inside a VCA ride keeps its `J`: the step at 1,000 jumps over
    /// 480, the riding curve's grid points whose ramps would end inside it are held, and the
    /// first one whose ramp ends after it (1,472) moves the lane again.
    ///
    /// Red if a grid event of one curve retargets the composed lane inside another curve's jump
    /// (the step would end after 24 samples, at the grid point 1,024).
    #[test]
    fn a_step_inside_another_curves_ride_keeps_its_jump_length() {
        for backend in input_test_backends() {
            for vca_steps in [true, false] {
                let (own, vca) = if vca_steps {
                    (
                        ride_table(8_000, 0.0, -24.0),
                        step_table(1_000, -6.0, -12.0, session::ParameterUnit::Db),
                    )
                } else {
                    (
                        step_table(1_000, -6.0, -12.0, session::ParameterUnit::Db),
                        ride_table(8_000, 0.0, -24.0),
                    )
                };
                let mut model = vca_model(2);
                model.vcas = vec![vca_of("v", 0.0, &["t00"])];
                model.automation = vec![
                    stored(
                        "own",
                        strip_owner(0),
                        5,
                        session::ParameterChannel::Left,
                        own,
                    ),
                    stored(
                        "vca",
                        vca_owner("v"),
                        1,
                        session::ParameterChannel::Left,
                        vca,
                    ),
                ];
                let mut rig = vca_rig(&model, backend, [480; 4]);
                let (_, events) = rig.run(16, 128);
                assert_eq!(
                    moves_after_step(&events).get(..2),
                    Some(
                        &[
                            (1_000, automation::EventKind::Jump, 480),
                            (1_472, automation::EventKind::Grid, 64),
                        ][..]
                    ),
                    "vca steps {vca_steps}, {backend:?}"
                );
            }
        }
    }

    /// #1655 R19-4 (V2): a member's own step and a VCA step at one sample are one jump over the
    /// longer of their rows' lengths, both ways round; an own unmute and a VCA unmute at one
    /// sample are one `SetMute` over the longer of the mute and VCA mute lengths.
    ///
    /// Red if coincident events emit twice, or the composed jump takes the shorter or the first
    /// curve's length.
    #[test]
    fn coincident_member_and_vca_events_take_the_longest_jump() {
        for backend in input_test_backends() {
            for (own_jump, vca_jump) in [(480, 960), (960, 480)] {
                let db = session::ParameterUnit::Db;
                let toggle = session::ParameterUnit::Linear;
                let mut model = vca_model(2);
                model.vcas = vec![vca_of("v", 0.0, &["t00"])];
                model.automation = vec![
                    stored(
                        "own",
                        strip_owner(0),
                        5,
                        session::ParameterChannel::Left,
                        step_table(1_000, -6.0, -12.0, db),
                    ),
                    stored(
                        "own-mute",
                        strip_owner(0),
                        6,
                        session::ParameterChannel::Left,
                        step_table(1_000, 1.0, 0.0, toggle),
                    ),
                    stored(
                        "vca",
                        vca_owner("v"),
                        1,
                        session::ParameterChannel::Left,
                        step_table(1_000, -3.0, -1.0, db),
                    ),
                    stored(
                        "vca-mute",
                        vca_owner("v"),
                        2,
                        session::ParameterChannel::Left,
                        step_table(1_000, 1.0, 0.0, toggle),
                    ),
                ];
                let mut rig = vca_rig(&model, backend, [own_jump, own_jump, vca_jump, vca_jump]);
                let (_, events) = rig.run(12, 128);
                let at_step = events
                    .iter()
                    .filter(|event| (event.lane, event.channel, event.sample) == (0, 0, 1_000))
                    .map(|event| (event.mute, event.kind, event.ramp, event.held))
                    .collect::<Vec<_>>();
                let longest = own_jump.max(vca_jump);
                assert_eq!(
                    at_step,
                    [
                        (false, automation::EventKind::Jump, longest, false),
                        (true, automation::EventKind::Jump, longest, false),
                    ],
                    "{own_jump}/{vca_jump} {backend:?}"
                );
            }
        }
    }

    /// #1655 D2 (V2): a session without VCA automation (static VCAs over automated members) and
    /// one whose only VCA automation is a mute keep every fader channel a lone cell over its own
    /// table, as before #1655, and build no VCA cell for the offset; so no fader event of such a
    /// session goes through the composed walk and no bit of it can move.
    ///
    /// Red if a VCA without automation, or a VCA mute curve, turns a member's fader channel into a
    /// composed one.
    #[test]
    fn a_session_without_vca_offset_automation_keeps_its_lone_fader_cells() {
        let db = session::ParameterUnit::Db;
        let toggle = session::ParameterUnit::Linear;
        let mut model = vca_model(2);
        model.vcas = vec![vca_of("v", -6.0, &["t00", "t01"])];
        model.automation = vec![
            stored(
                "own",
                strip_owner(0),
                5,
                session::ParameterChannel::Both,
                ride_table(4_096, -6.5, -2.0),
            ),
            stored(
                "own-mute",
                strip_owner(1),
                6,
                session::ParameterChannel::Left,
                step_table(1_000, 1.0, 0.0, toggle),
            ),
        ];
        let rig = vca_rig(&model, Backend::Simd4, [480; 4]);
        let plan = rig.plan.as_ref().expect("cells");
        assert!(plan.vcas.is_empty(), "no VCA cell");
        for slot in rig.processor.automation.iter().flatten() {
            assert!(
                slot.channels
                    .iter()
                    .flatten()
                    .all(|cell| matches!(cell.curve, FaderCurve::Cell { .. }))
            );
            assert!(
                slot.mute
                    .iter()
                    .flatten()
                    .all(|cell| matches!(cell.curve, MuteCurve::Cell { .. }))
            );
        }
        model.automation.push(stored(
            "vca-mute",
            vca_owner("v"),
            2,
            session::ParameterChannel::Both,
            step_table(1_000, 0.0, 1.0, toggle),
        ));
        let _ = db;
        let rig = vca_rig(&model, Backend::Simd4, [480; 4]);
        let plan = rig.plan.as_ref().expect("cells");
        assert_eq!(
            plan.vcas.get("v").copied(),
            Some([[None, None], [Some(3), Some(4)]]),
            "the VCA mute cells follow every strip cell, the offset row has none"
        );
        for slot in rig.processor.automation.iter().flatten() {
            assert!(
                slot.channels
                    .iter()
                    .flatten()
                    .all(|cell| matches!(cell.curve, FaderCurve::Cell { .. }))
            );
            assert!(
                slot.mute
                    .iter()
                    .flatten()
                    .all(|cell| matches!(cell.curve, MuteCurve::Composed(_)))
            );
        }
    }

    /// #1648 D2 and #1655 D6 on composed and lone channels: a plan swap at sample 1,100 that carries
    /// every lane renders, bit for bit, the uninterrupted bank, whether the join continues the
    /// composed cells verbatim (`continued_cells` set) or repositions them (clear), over a VCA
    /// ride that reaches a member with its own ride and one with none (the swap inside a grid
    /// ramp), and continued across a VCA step's jump (1,000 over 480: the own ride's grid stays
    /// held to its end). The same swap at 1,152 with 128-frame blocks starts on the grid, where
    /// the last grid ramp has just completed (#1716 fix D). At 1,100, a settled muted stage
    /// remembers the grid's target at 1,151 with no ramp, adopts again one frame later before
    /// the next grid, then unmutes at 1,120, so a spurious adoption jump changes audible output
    /// (#2096). The exact-set row leaves both runs' free stages at
    /// their carried targets with no ramp. A table step at adoption still jumps there (B17-R13),
    /// including a VCA step coincident with a running own ride (#2158): the merged walk keeps
    /// that ride's heard target instead of landing every cursor at the jump's completion.
    ///
    /// Red if a composed channel's carry loses its cursors' or its output's state (a continued
    /// walk restarts), if its reposition compares at another time than the ramp's end (an
    /// unchanged curve would jump at the swap), or if a reposition with no stage ramp compares at
    /// the block's first sample or the sample before it instead of the old completion: an
    /// unchanged curve would jump on a grid-aligned block or an off-grid no-ramp adoption.
    #[test]
    fn a_carried_automated_lane_renders_the_uninterrupted_bits() {
        for backend in input_test_backends() {
            for composed in [true, false] {
                for (case, continued, vca_table, initially_muted, settle) in [
                    (
                        "continued-ride",
                        0b1111_u8,
                        ride_table(8_192, -12.0, 0.0),
                        false,
                        false,
                    ),
                    (
                        "repositioned-ride",
                        0,
                        ride_table(8_192, -12.0, 0.0),
                        false,
                        false,
                    ),
                    (
                        "settled-ride",
                        0,
                        ride_table(8_192, -12.0, 0.0),
                        false,
                        true,
                    ),
                    ("muted-ride", 0, ride_table(8_192, -12.0, 0.0), true, false),
                    (
                        "entry-step",
                        0,
                        step_table(1_100, -6.0, -12.0, session::ParameterUnit::Db),
                        false,
                        false,
                    ),
                    (
                        "entry-step-running",
                        0,
                        step_table(1_100, -6.0, -12.0, session::ParameterUnit::Db),
                        false,
                        false,
                    ),
                    (
                        "entry-step-running-prior-jump",
                        0,
                        vec![
                            vca_segment(
                                session::AutomationShape::Step,
                                (0, 900),
                                (-3.0, -3.0),
                                session::ParameterUnit::Db,
                            ),
                            vca_segment(
                                session::AutomationShape::Step,
                                (900, 1_100),
                                (-6.0, -6.0),
                                session::ParameterUnit::Db,
                            ),
                            vca_segment(
                                session::AutomationShape::Step,
                                (1_100, 100_000),
                                (-12.0, -12.0),
                                session::ParameterUnit::Db,
                            ),
                        ],
                        false,
                        false,
                    ),
                    (
                        "continued-step",
                        0b1111,
                        step_table(1_000, -6.0, -12.0, session::ParameterUnit::Db),
                        false,
                        false,
                    ),
                ] {
                    let mut model = vca_model(2);
                    model.vcas = vec![vca_of("v", 0.0, &["t00", "t01"])];
                    model.automation = vec![
                        stored(
                            "own-ride",
                            strip_owner(1),
                            5,
                            session::ParameterChannel::Left,
                            ride_table(4_096, -6.5, if case == "entry-step" { -6.5 } else { -2.0 }),
                        ),
                        stored(
                            "vca-ride",
                            vca_owner("v"),
                            1,
                            session::ParameterChannel::Both,
                            vca_table,
                        ),
                        stored(
                            "vca-mute",
                            vca_owner("v"),
                            2,
                            session::ParameterChannel::Both,
                            step_table(
                                if initially_muted { 1_120 } else { 1_500 },
                                if initially_muted { 1.0 } else { 0.0 },
                                if initially_muted { 0.0 } else { 1.0 },
                                session::ParameterUnit::Linear,
                            ),
                        ),
                    ];
                    if !composed {
                        model
                            .automation
                            .retain(|table| table.id.as_str() != "vca-ride");
                    }
                    // (block frames, blocks before the swap, blocks in all): the swap at 1,100, off
                    // the grid, and at 1,152, on it (#1716 fix D).
                    for (frames, swap, blocks) in [(100, 11, 24), (128, 9, 19)] {
                        if case.starts_with("entry-step-running") && frames != 100 {
                            // #2158 exercises the swap coincident with the step, at 1,100.
                            continue;
                        }
                        let mut uninterrupted = vca_rig(&model, backend, [480; 4]);
                        let mut predecessor = vca_rig(&model, backend, [480; 4]);
                        let (expected_before, _) = uninterrupted.run(swap, frames);
                        let (before, _) = predecessor.run(swap, frames);
                        assert_eq!(before, expected_before);
                        if settle {
                            // The same exact stage operation on both runs leaves a free stage with
                            // no ramp, still at the old cell's target (its recorded completion).
                            for rig in [&mut uninterrupted, &mut predecessor] {
                                for lane in 0..rig.lanes {
                                    for (channel, selector) in
                                        [BuiltinLaneSelector::Left, BuiltinLaneSelector::Right]
                                            .into_iter()
                                            .enumerate()
                                    {
                                        let gain = rig
                                            .processor
                                            .bank
                                            .ramp_target(lane, channel)
                                            .expect("populated lane");
                                        rig.processor
                                            .bank
                                            .apply_fader_operation(
                                                lane,
                                                selector,
                                                builtins::FaderOperation::SetGain { gain },
                                            )
                                            .expect("the carried target is a valid gain");
                                        assert_eq!(
                                            rig.processor.bank.ramp_remaining(lane, channel),
                                            Some(0)
                                        );
                                        assert_eq!(
                                            rig.processor.bank.gain_held(lane, channel),
                                            Some(false)
                                        );
                                    }
                                }
                            }
                        }
                        let mut successor = vca_rig(&model, backend, [480; 4]);
                        for lane in 0..successor.lanes {
                            assert!(successor.processor.adopt_lane(
                                lane,
                                &predecessor.processor,
                                lane,
                                continued
                            ));
                        }
                        successor.first = predecessor.first;
                        if case == "entry-step-running" && composed && frames == 100 {
                            // A genuinely changed own curve must still land at entry. The
                            // no-op comparison must not preserve the old curve's heard target.
                            let mut changed_model = model.clone();
                            changed_model.automation[0].segments = ride_table(4_096, -8.5, -4.0);
                            let mut changed = vca_rig(&changed_model, backend, [480; 4]);
                            for lane in 0..changed.lanes {
                                assert!(changed.processor.adopt_lane(
                                    lane,
                                    &predecessor.processor,
                                    lane,
                                    0
                                ));
                            }
                            changed.first = predecessor.first;
                            let (_, events) = changed.run(1, frames);
                            let entry = events
                                .iter()
                                .filter(|event| {
                                    (event.lane, event.channel, event.sample) == (1, 0, 1_100)
                                        && !event.mute
                                        && !event.held
                                })
                                .collect::<Vec<_>>();
                            assert_eq!(entry.len(), 1, "one changed-table entry landing");
                            assert_eq!(entry[0].kind, automation::EventKind::Jump);
                            assert_eq!(entry[0].ramp, 480);
                            let plan = changed.plan.as_ref().expect("automation");
                            let own = plan.strip("t01").expect("member").fader[0]
                                .as_ref()
                                .expect("fader")
                                .cell
                                .expect("own curve");
                            let cell = plan.program.cell(own as usize).expect("own cell");
                            let expected = composed_db(
                                automation::value_at(cell.segments(), 1_579),
                                0.0,
                                core::iter::once(-12.0),
                            );
                            assert_eq!(
                                entry[0].value.to_bits(),
                                expected.to_bits(),
                                "a changed table retains its entry landing target: {backend:?}"
                            );
                        }
                        if initially_muted && frames == 100 {
                            let (expected, _) = uninterrupted.run(1, 1);
                            let (actual, _) = successor.run(1, 1);
                            assert_eq!(actual, expected);
                            assert_eq!(
                                successor.processor.bank.fader_gain(1, 0).map(f32::to_bits),
                                uninterrupted
                                    .processor
                                    .bank
                                    .fader_gain(1, 0)
                                    .map(f32::to_bits),
                                "the first no-ramp swap preserves the held target: composed {composed}, {backend:?}, {frames}-frame blocks"
                            );
                            // Adopt once more before the next grid. In a lone cell, reposition
                            // cleared the completion unless the unchanged comparison restored it.
                            let mut again = vca_rig(&model, backend, [480; 4]);
                            for lane in 0..again.lanes {
                                assert!(again.processor.adopt_lane(
                                    lane,
                                    &successor.processor,
                                    lane,
                                    0
                                ));
                            }
                            again.first = successor.first;
                            successor = again;
                        }
                        let (expected_after, _) = uninterrupted.run(blocks - swap, frames);
                        let (after, _) = successor.run(blocks - swap, frames);
                        if initially_muted {
                            assert!(
                                expected_after.last().expect("post-swap blocks")[1][0]
                                    .iter()
                                    .any(|&bits| f32::from_bits(bits) > 0.0),
                                "the unmute must make the bit comparison audible"
                            );
                            assert_eq!(
                                successor.processor.bank.fader_gain(1, 0).map(f32::to_bits),
                                uninterrupted
                                    .processor
                                    .bank
                                    .fader_gain(1, 0)
                                    .map(f32::to_bits),
                                "the repeated swap preserves the held target"
                            );
                        }
                        assert_eq!(
                            after[..],
                            expected_after[..],
                            "{case}, composed {composed}, continued {continued:#b}, muted {initially_muted}, settled {settle}, {backend:?}, {frames}-frame blocks"
                        );
                    }
                }
            }
        }
    }

    /// SplitMix64, for [`test_only_builtin_lane_swap`].
    #[cfg(feature = "test-support")]
    struct LaneSwapRng(u64);

    #[cfg(feature = "test-support")]
    impl LaneSwapRng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        }

        fn below(&mut self, bound: u64) -> u64 {
            self.next() % bound
        }

        /// A value in `[low, high)`.
        fn within(&mut self, low: f32, high: f32) -> f32 {
            let unit = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
            low + (high - low) * unit
        }

        fn selector(&mut self) -> BuiltinLaneSelector {
            [
                BuiltinLaneSelector::Left,
                BuiltinLaneSelector::Right,
                BuiltinLaneSelector::Both,
            ][self.below(3) as usize]
        }
    }

    /// One live write to one strip's cells.
    #[cfg(feature = "test-support")]
    #[derive(Clone, Copy)]
    enum LaneSwapWrite {
        Input(TrackInputRecord),
        Fader(TrackFaderRecord),
        Matrix(TrackControlRecord),
        Offsets(usize, FaderOffsetsRecord),
    }

    /// Per lane of one chain, a strip's planes.
    #[cfg(feature = "test-support")]
    struct LaneSwapPlanes {
        left: Vec<Vec<f32>>,
        right: Vec<Vec<f32>>,
    }

    #[cfg(feature = "test-support")]
    impl rack::BankMembers for LaneSwapPlanes {
        fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
            (&self.left[lane], &self.right[lane])
        }
        fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
            (&mut self.left[lane], &mut self.right[lane])
        }
    }

    /// A session's strips in builtin bank chains `[input, fader, matrix]`, one per cohort, as the
    /// graph binds them, with every strip's live controls and the strip -> `(chain, lane)` map.
    #[cfg(feature = "test-support")]
    struct LaneSwapRig {
        chains: Vec<rack::BankChain>,
        position: Vec<(usize, usize)>,
        controls: Vec<TrackControlProducer>,
        gate: TestOnlyGate,
        width: usize,
        frames: u32,
        first: u64,
    }

    /// `tracks` strips with live controls and stored fader and mute automation on `t01`, `t05` and
    /// `t09`, each with an event after the swaps below. `mono`: one source channel and equal
    /// input sections on every track. `quantum`: the session's render quantum.
    #[cfg(feature = "test-support")]
    fn lane_swap_session(tracks: usize, mono: bool, quantum: u32) -> CompiledSession {
        let mut model = n_track_session_with_symmetry(tracks, mono)
            .normalized_model()
            .clone();
        model.quantum_frames = quantum;
        if mono {
            for track in &mut model.tracks {
                track.right_source_channel = track.left_source_channel;
            }
        }
        let db = session::ParameterUnit::Db;
        let toggle = session::ParameterUnit::Linear;
        let (left, right, both) = (
            session::ParameterChannel::Left,
            session::ParameterChannel::Right,
            session::ParameterChannel::Both,
        );
        model.vcas.clear();
        model.automation = vec![
            stored(
                "a1f",
                strip_owner(1),
                5,
                left,
                step_table(800, -6.0, -14.0, db),
            ),
            stored(
                "a1m",
                strip_owner(1),
                6,
                right,
                step_table(900, 0.0, 1.0, toggle),
            ),
            stored(
                "a5f",
                strip_owner(5),
                5,
                both,
                ride_table(1_400, -3.0, -18.0),
            ),
            stored(
                "a5m",
                strip_owner(5),
                6,
                left,
                step_table(1_100, 1.0, 0.0, toggle),
            ),
            stored(
                "a9f",
                strip_owner(9),
                5,
                right,
                step_table(1_100, -2.0, -9.0, db),
            ),
        ];
        compile_session(&model, vca_caps()).expect("the session compiles")
    }

    #[cfg(feature = "test-support")]
    fn lane_swap_rig(compiled: &CompiledSession, backend: Backend, armed: bool) -> LaneSwapRig {
        let tracks = compiled.normalized_model().tracks.len();
        let requests = (0..tracks)
            .map(|index| TrackControlRequest {
                track_id: track_name(index),
                superseded: LiveValuesSuperseded::default(),
                input_lane: true,
            })
            .collect::<Vec<_>>();
        let mut prepared = prepare_session_builtins_with_live_controls(
            compiled,
            &effective_of(compiled),
            &[],
            &requests,
            caps(),
        )
        .expect("prepared builtins");
        let (_, levels) = track_graph(tracks);
        let classes = SessionPoolClasses::from_session(compiled);
        let width = BankWidth::for_backend(backend).expect("a bank width");
        let plan = planned_strip_banks(&prepared.seal.tracks, backend, &levels, &classes);
        let (banks, _) = {
            let _canonical = lane::CanonicalFpEnv::enter();
            prepared.lower_strip_banks(plan, backend, width)
        };
        let strip_of = |member: &GraphNodeId| {
            let GraphNodeId::TrackStage { track_id, stage } = member else {
                panic!("a strip bank member is a track stage")
            };
            let strip: usize = track_id.as_str()[1..]
                .parse()
                .expect("a generated track ID");
            (strip, *stage)
        };
        type Cohort = (
            Vec<usize>,
            [Option<Box<dyn GraphPreparedBuiltinBankProcessor>>; 3],
            Option<AoSoaScratch>,
        );
        let mut cohorts: Vec<Cohort> = Vec::new();
        for bank in banks {
            let members = bank.members.iter().map(strip_of).collect::<Vec<_>>();
            let stage = members[0].1;
            assert!(members.iter().all(|member| member.1 == stage));
            let strips = members.iter().map(|member| member.0).collect::<Vec<_>>();
            let slot = match stage {
                TrackStage::PostInputBuiltins => 0,
                TrackStage::PostFader => 1,
                TrackStage::PostMatrix => 2,
                _ => unreachable!("a strip bank stage"),
            };
            let index = cohorts
                .iter()
                .position(|cohort| cohort.0 == strips)
                .unwrap_or_else(|| {
                    cohorts.push((strips, [None, None, None], None));
                    cohorts.len() - 1
                });
            let cohort = &mut cohorts[index];
            assert!(cohort.1[slot].is_none(), "one bank per stage and cohort");
            cohort.1[slot] = Some(bank.processor);
            if slot == 0 {
                cohort.2 = Some(bank.scratch);
            }
        }
        let mut position = vec![None; tracks];
        let mut chains = Vec::new();
        for (chain, (strips, processors, scratch)) in cohorts.into_iter().enumerate() {
            for (lane, strip) in strips.iter().enumerate() {
                assert!(position[*strip].replace((chain, lane)).is_none());
            }
            let active = (0..width.lanes() as usize)
                .map(|lane| lane < strips.len())
                .collect::<Box<[bool]>>();
            let slots = processors
                .into_iter()
                .map(|processor| rack::BankSlot {
                    stage: graph::test_only_builtin_stage(
                        processor.expect("the input, fader and matrix banks share one member list"),
                    ),
                    active_lanes: active.clone(),
                })
                .collect();
            let mut built = rack::BankChain::new(
                scratch.expect("the cohort has an input bank"),
                active,
                slots,
            )
            .expect("a builtin chain");
            built.arm_mono_collapse(armed);
            chains.push(built);
        }
        let mut controls = (0..tracks).map(|_| None).collect::<Vec<_>>();
        for control in core::mem::take(&mut prepared.track_controls) {
            let strip: usize = control.track_id[1..].parse().expect("a generated track ID");
            controls[strip] = Some(control);
        }
        LaneSwapRig {
            chains,
            position: position
                .into_iter()
                .map(|at| at.expect("every strip is banked"))
                .collect(),
            controls: controls
                .into_iter()
                .map(|control| control.expect("every strip has live controls"))
                .collect(),
            gate: TestOnlyGate::new(),
            width: width.lanes() as usize,
            frames: prepared.seal.quantum,
            first: 0,
        }
    }

    #[cfg(feature = "test-support")]
    impl LaneSwapRig {
        fn write(&mut self, writes: &[(usize, LaneSwapWrite)]) {
            let controls = &mut self.controls;
            self.gate.write_published(|stamp| {
                for &(strip, write) in writes {
                    let control = &mut controls[strip];
                    match write {
                        LaneSwapWrite::Input(record) => control
                            .input
                            .as_mut()
                            .expect("an input lane")
                            .write(stamp, record),
                        LaneSwapWrite::Fader(record) => control.fader.write(stamp, record),
                        LaneSwapWrite::Matrix(record) => control.producer.write(stamp, record),
                        LaneSwapWrite::Offsets(channel, record) => {
                            control.fader.write_offsets(stamp, channel, record);
                        }
                    }
                }
            });
        }

        /// The automated fader lanes of chain `chain`.
        fn automated_lanes(&mut self, chain: usize) -> Vec<usize> {
            self.fader(chain)
                .automation
                .iter()
                .enumerate()
                .filter_map(|(lane, slot)| slot.as_ref().map(|_| lane))
                .collect()
        }

        /// Chain `chain`'s fader bank processor (slot 1).
        fn fader(&mut self, chain: usize) -> &mut FaderBankProcessor {
            graph::test_only_builtin_processor_mut(&mut self.chains[chain], 1)
                .and_then(GraphPreparedBuiltinBankProcessor::as_any_mut)
                .and_then(|any| any.downcast_mut::<FaderBankProcessor>())
                .expect("slot 1 is the fader bank")
        }

        /// One block of `inputs[strip]`; per strip, its output bits.
        fn block(&mut self, inputs: &[[Vec<f32>; 2]]) -> Vec<[Vec<u32>; 2]> {
            let frames = self.frames as usize;
            let mut planes = self
                .chains
                .iter()
                .map(|_| LaneSwapPlanes {
                    left: vec![vec![0.0; frames]; self.width],
                    right: vec![vec![0.0; frames]; self.width],
                })
                .collect::<Vec<_>>();
            for (strip, [left, right]) in inputs.iter().enumerate() {
                let (chain, lane) = self.position[strip];
                planes[chain].left[lane].clone_from(left);
                planes[chain].right[lane].clone_from(right);
            }
            let snapshot = self.gate.snapshot();
            {
                let _canonical = lane::CanonicalFpEnv::enter();
                for (chain, planes) in self.chains.iter_mut().zip(planes.iter_mut()) {
                    // The block's node time as `set_block_time` sets it, with `a(n) = 0`.
                    let fader = graph::test_only_builtin_processor_mut(chain, 1)
                        .and_then(GraphPreparedBuiltinBankProcessor::as_any_mut)
                        .and_then(|any| any.downcast_mut::<FaderBankProcessor>())
                        .expect("slot 1 is the fader bank");
                    for slot in fader.automation.iter_mut().flatten() {
                        slot.span = Some(TimelineSpan {
                            start: i64::try_from(self.first).expect("time"),
                            step: None,
                        });
                        slot.fader_jump = Some(96);
                        slot.mute_jump = Some(64);
                        slot.vca_jumps = Some([32, 32]);
                    }
                    chain
                        .run(planes, self.frames, self.first, snapshot)
                        .expect("the chain renders");
                }
            }
            self.first += u64::from(self.frames);
            self.position
                .iter()
                .map(|&(chain, lane)| {
                    [&planes[chain].left[lane], &planes[chain].right[lane]]
                        .map(|plane| plane.iter().map(|sample| sample.to_bits()).collect())
                })
                .collect()
        }

        /// Swap `at` with `other` (`at.0 <= other.0`) inside `measure`'s callback, which must
        /// call it exactly once, and move the strips' positions.
        fn swap(
            &mut self,
            at: (usize, usize),
            other: (usize, usize),
            measure: &mut dyn FnMut(&mut dyn FnMut()),
        ) {
            assert!(at.0 <= other.0);
            let bytes = self.chains[at.0]
                .lane_swap_scratch_bytes()
                .max(self.chains[other.0].lane_swap_scratch_bytes());
            let mut scratch = vec![0_u8; bytes];
            let mut swapped = 0_u32;
            let chains = &mut self.chains;
            measure(&mut || {
                let result = if at.0 == other.0 {
                    chains[at.0].swap_lanes(at.1, rack::ChainLanePeer::Same(other.1), &mut scratch)
                } else {
                    let (head, tail) = chains.split_at_mut(other.0);
                    head[at.0].swap_lanes(
                        at.1,
                        rack::ChainLanePeer::Other(&mut tail[0], other.1),
                        &mut scratch,
                    )
                };
                result.expect("a builtin lane swap");
                swapped += 1;
            });
            assert_eq!(swapped, 1, "the measure callback runs the swap once");
            self.moved(at, other);
        }

        /// A cross-chain swap the chains must refuse; the refusal, and nothing moved.
        fn refused_swap(
            &mut self,
            at: (usize, usize),
            other: (usize, usize),
        ) -> Result<(), rack::ChainLaneSwapRefusal> {
            assert!(at.0 < other.0);
            let mut scratch = vec![
                0_u8;
                self.chains[at.0]
                    .lane_swap_scratch_bytes()
                    .max(self.chains[other.0].lane_swap_scratch_bytes())
            ];
            let (head, tail) = self.chains.split_at_mut(other.0);
            let result = head[at.0].swap_lanes(
                at.1,
                rack::ChainLanePeer::Other(&mut tail[0], other.1),
                &mut scratch,
            );
            if result.is_ok() {
                self.moved(at, other);
            }
            result
        }

        /// The strips at `at` and `other` trade positions.
        fn moved(&mut self, at: (usize, usize), other: (usize, usize)) {
            let strip_at = |position: &[(usize, usize)], at| {
                position
                    .iter()
                    .position(|here| *here == at)
                    .expect("a strip sits there")
            };
            let (a, b) = (
                strip_at(&self.position, at),
                strip_at(&self.position, other),
            );
            self.position.swap(a, b);
        }
    }

    /// One block's random live writes: per strip, up to three input, fader, mute, matrix or
    /// offsets writes, ramped over windows up to 600 samples (a block is 128). `ramp_all` also
    /// starts a long trim, fader and matrix ramp on every strip, so ramps are in flight at a swap.
    /// A `mono` strip's input writes address both channels.
    #[cfg(feature = "test-support")]
    fn lane_swap_writes(
        rng: &mut LaneSwapRng,
        offsets: &[[bool; 2]],
        filters: &[builtins::PreparedInputFilterTarget],
        mono: bool,
        ramp_all: bool,
        quiet: Option<usize>,
    ) -> Vec<(usize, LaneSwapWrite)> {
        let mut writes = Vec::new();
        for (strip, offsets) in offsets.iter().enumerate() {
            let input_lanes = |rng: &mut LaneSwapRng| {
                if mono {
                    BuiltinLaneSelector::Both
                } else {
                    rng.selector()
                }
            };
            for _ in 0..rng.below(4) {
                let smoothing_samples = rng.below(600) as u32;
                let write = match rng.below(7) {
                    0 => LaneSwapWrite::Input(TrackInputRecord::TrimDb {
                        lanes: input_lanes(rng),
                        db: rng.within(-12.0, 6.0),
                        smoothing_samples,
                    }),
                    1 => LaneSwapWrite::Input(TrackInputRecord::PolarityInvert {
                        lanes: input_lanes(rng),
                        inverted: rng.below(2) == 1,
                        smoothing_samples,
                    }),
                    2 => LaneSwapWrite::Input(TrackInputRecord::PreparedFilter {
                        target: builtins::PreparedInputFilterTarget {
                            lanes: input_lanes(rng),
                            ..filters[rng.below(filters.len() as u64) as usize]
                        },
                    }),
                    3 => LaneSwapWrite::Fader(TrackFaderRecord::FaderDb {
                        lanes: rng.selector(),
                        db: rng.within(-30.0, 6.0),
                        smoothing_samples,
                    }),
                    4 => LaneSwapWrite::Fader(TrackFaderRecord::Mute {
                        lanes: rng.selector(),
                        muted: rng.below(3) == 0,
                        smoothing_samples,
                    }),
                    5 => LaneSwapWrite::Matrix(TrackControlRecord {
                        matrix: Matrix2x2 {
                            ll: rng.within(-1.0, 1.0),
                            lr: rng.within(-1.0, 1.0),
                            rl: rng.within(-1.0, 1.0),
                            rr: rng.within(-1.0, 1.0),
                        },
                        smoothing_samples,
                    }),
                    _ => {
                        let channel = rng.below(2) as usize;
                        if !offsets[channel] {
                            continue;
                        }
                        LaneSwapWrite::Offsets(
                            channel,
                            FaderOffsetsRecord {
                                offsets_sum: f64::from(rng.within(-6.0, 0.0)),
                                smoothing_samples,
                                own_db: None,
                            },
                        )
                    }
                };
                if quiet == Some(strip) && matches!(write, LaneSwapWrite::Input(_)) {
                    continue;
                }
                writes.push((strip, write));
            }
            if ramp_all {
                let smoothing_samples = 300 + rng.below(300) as u32;
                let trim = LaneSwapWrite::Input(TrackInputRecord::TrimDb {
                    lanes: input_lanes(rng),
                    db: rng.within(-12.0, 6.0),
                    smoothing_samples,
                });
                if quiet != Some(strip) {
                    writes.push((strip, trim));
                }
                writes.push((
                    strip,
                    LaneSwapWrite::Fader(TrackFaderRecord::FaderDb {
                        lanes: BuiltinLaneSelector::Both,
                        db: rng.within(-30.0, 6.0),
                        smoothing_samples,
                    }),
                ));
                writes.push((
                    strip,
                    LaneSwapWrite::Matrix(TrackControlRecord {
                        matrix: Matrix2x2 {
                            ll: rng.within(-1.0, 1.0),
                            lr: rng.within(-1.0, 1.0),
                            rl: rng.within(-1.0, 1.0),
                            rr: rng.within(-1.0, 1.0),
                        },
                        smoothing_samples,
                    }),
                ));
            }
        }
        writes
    }

    /// Test-only (#1765 gates 1 and 2): a render-time swap of two strips' builtin lanes -- inside
    /// one chain and across two chains -- moves each strip's whole input, fader and matrix state
    /// with it. Every `BankChain::swap_lanes` call runs inside `measure`'s callback, which must
    /// call it once, so a caller with a counting allocator gates the swap's allocations (the
    /// `lane_swap_alloc` test). Returns the number of swaps measured.
    ///
    /// A real session's strips are prepared with live controls and stored fader and mute
    /// automation, lowered into the builtin banks, and bound as the graph binds them: one
    /// `[input, fader, matrix]` chain per cohort, every slot the graph's own builtin adapter. Two
    /// identical preparations render the same random inputs and the same random live writes
    /// (trim, polarity, filter target, fader, mute, matrix and offsets, ramped over windows longer
    /// than a block), strip by strip. Before block 5 the test rig swaps two lanes of chain 0 (an
    /// automated strip and a plain one) after that block's writes are published, so dirty cells,
    /// ramps in flight and a stored step at sample 800 all lie ahead of the swap; before block 8 it
    /// swaps a lane of chain 0 with an automated lane of chain 1, whose stored events at 1,100 are
    /// ahead. Every strip's left and right output, every block, is bit-equal to the twin's. The
    /// mono pass arms every chain for mono sources with equal channels and both-channel input
    /// writes: the in-chain swap lands on a
    /// chain that rendered its last block collapsed (R7), and the cross-chain swap takes a collapsed
    /// chain's lane and a strip whose left-only trim (of its own value) cleared its live symmetry
    /// term, so that strip's new chain must render dual and its old one collapse again. Before block
    /// 10, where a chain has no automated strip (`t12` alone at four lanes), an automated lane is
    /// refused a move into its slotless fader bank (`Slot`), and the twin check proves nothing
    /// moved. At both widths this host builds, at a 128-frame quantum and at a 32-frame one: before
    /// each swap every fourth strip from `t01` retargets both filter sections (#1777), and at 32
    /// frames that 64-sample filter ramp is half done at the swap; the strip it trades with has no
    /// new ramp in the mono passes and may hold a random one in the stereo passes. At 32 frames
    /// the 14 blocks end before the first stored event, so only the 128-frame passes render
    /// stored automation.
    ///
    /// Red if a swap leaves behind any per-lane field the next blocks read: the stage state or a
    /// ramp (the input filter's countdown among them), a cell reader (with its dirty bits), the
    /// live symmetry term, the stored automation,
    /// or the disengage before an in-chain swap of a collapsed chain (through the input stage's
    /// debug assertion); or if a builtin stage refuses the swap. Not covered: the input stage's
    /// prepared filter endpoint (`filter_initial`), which only a `FullToPrepared` reset reads and
    /// no chain path reaches.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_builtin_lane_swap(measure: &mut dyn FnMut(&mut dyn FnMut())) -> usize {
        const TRACKS: usize = 13;
        const BLOCKS: u64 = 14;
        const SAME_AT: u64 = 5;
        const OTHER_AT: u64 = 8;
        /// The block before which an automated lane is refused a move into a fader bank with no
        /// automation slots.
        const REFUSED_AT: u64 = 10;
        /// The strip the cross-chain swap moves out of chain 0.
        const QUIET: usize = 2;
        let mut swaps = 0;
        let mut refusals = 0;
        let pair = |hpf, lpf| {
            builtins::prepare_input_filter_pair(48_000, hpf, lpf)
                .expect("a filter pair")
                .targets
        };
        let filters = [(40.0, 16_000.0), (120.0, 8_000.0), (300.0, 12_000.0)]
            .into_iter()
            .flat_map(|(hpf, lpf)| pair(hpf, lpf))
            .collect::<Vec<_>>();
        // The swaps' filter retargets: pairs no random write and no prepared strip holds, one
        // per swap, so each starts a ramp.
        let swap_filters = [pair(60.0, 10_000.0), pair(70.0, 9_000.0)];
        for backend in input_test_backends() {
            for (quantum, mono) in [(128, false), (128, true), (32, false), (32, true)] {
                let what = format!("{backend:?}, quantum {quantum}, mono {mono}");
                let compiled = lane_swap_session(TRACKS, mono, quantum);
                let mut rig = lane_swap_rig(&compiled, backend, mono);
                let mut twin = lane_swap_rig(&compiled, backend, mono);
                assert!(rig.chains.len() >= 2, "{what}: two chains");
                // The swaps below move automated lanes: `t01` inside chain 0, and lane 1 of
                // chain 1 (`t09` at eight lanes, `t05` at four) into chain 0.
                assert!(rig.automated_lanes(0).contains(&1), "{what}");
                assert!(rig.automated_lanes(1).contains(&1), "{what}");
                let offsets = rig
                    .controls
                    .iter()
                    .map(|control| {
                        core::array::from_fn(|channel| control.fader.has_offsets(channel))
                    })
                    .collect::<Vec<[bool; 2]>>();
                let mut rng = LaneSwapRng(0x1765 ^ ((rig.width as u64) << 8) ^ u64::from(mono));
                for block in 0..BLOCKS {
                    let ramp_all = block == SAME_AT - 1 || block == OTHER_AT - 1;
                    let mut writes = lane_swap_writes(
                        &mut rng,
                        &offsets,
                        &filters,
                        mono,
                        ramp_all,
                        mono.then_some(QUIET),
                    );
                    // The mono pass's quiet strip: one both-channel trim, then before the
                    // cross-chain swap the same trim on its left channel alone. Its channels
                    // stay equal, but its live symmetry term is cleared for the plan: its chain
                    // renders dual from then on, wherever it sits.
                    let quiet_trim = |lanes| {
                        LaneSwapWrite::Input(TrackInputRecord::TrimDb {
                            lanes,
                            db: -4.0,
                            smoothing_samples: 0,
                        })
                    };
                    if mono && block == 0 {
                        writes.push((QUIET, quiet_trim(BuiltinLaneSelector::Both)));
                    }
                    if mono && block == OTHER_AT - 1 {
                        writes.push((QUIET, quiet_trim(BuiltinLaneSelector::Left)));
                    }
                    if ramp_all {
                        let targets = swap_filters[usize::from(block == OTHER_AT - 1)];
                        for strip in (1..TRACKS).step_by(4) {
                            for target in targets {
                                writes.push((
                                    strip,
                                    LaneSwapWrite::Input(TrackInputRecord::PreparedFilter {
                                        target,
                                    }),
                                ));
                            }
                        }
                    }
                    rig.write(&writes);
                    twin.write(&writes);
                    if block == SAME_AT {
                        assert!(!mono || rig.chains[0].is_collapsed(), "{what}: R7");
                        // A swap, its inverse and the swap again: each is counted, and the first
                        // two cancel.
                        for _ in 0..3 {
                            rig.swap((0, 1), (0, 3), measure);
                        }
                        swaps += 3;
                    }
                    if block == OTHER_AT {
                        assert_eq!(rig.position[QUIET], (0, 2), "{what}");
                        assert!(
                            !mono
                                || (!rig.chains[0].is_collapsed() && rig.chains[1].is_collapsed()),
                            "{what}: R7 across chains, the quiet strip's chain dual"
                        );
                        rig.swap((0, 2), (1, 1), measure);
                        swaps += 1;
                    }
                    if block == REFUSED_AT {
                        // A chain none of whose strips is automated has a fader bank without
                        // automation slots (#1646 D1): an automated lane has nowhere to land, so
                        // the swap is refused before any write, and the twin check of the blocks
                        // after it proves that nothing moved. At four lanes `t12` is such a chain.
                        let slotless = (1..rig.chains.len())
                            .find(|&chain| rig.fader(chain).automation.is_empty());
                        assert!(
                            backend != Backend::Simd4 || slotless.is_some(),
                            "{what}: a chain without automation"
                        );
                        if let Some(slotless) = slotless {
                            let automated = rig.automated_lanes(0)[0];
                            assert_eq!(
                                rig.refused_swap((0, automated), (slotless, 0)),
                                Err(rack::ChainLaneSwapRefusal::Slot),
                                "{what}: an automated lane into a slotless fader bank"
                            );
                            refusals += 1;
                        }
                    }
                    let inputs = (0..TRACKS)
                        .map(|_| {
                            let left = (0..rig.frames)
                                .map(|_| rng.within(-0.8, 0.8))
                                .collect::<Vec<_>>();
                            let right = if mono {
                                left.clone()
                            } else {
                                (0..rig.frames).map(|_| rng.within(-0.8, 0.8)).collect()
                            };
                            [left, right]
                        })
                        .collect::<Vec<_>>();
                    let got = rig.block(&inputs);
                    let expected = twin.block(&inputs);
                    for (strip, (got, expected)) in got.iter().zip(&expected).enumerate() {
                        assert!(
                            got == expected,
                            "{what}: strip t{strip:02}, block {block}, left/right equal: {}/{}",
                            got[0] == expected[0],
                            got[1] == expected[1]
                        );
                    }
                }
                // The quiet strip's cleared term moved with it: its new chain renders dual, and its
                // old chain collapses again.
                if mono {
                    assert!(
                        rig.chains[0].is_collapsed() && !rig.chains[1].is_collapsed(),
                        "{what}: the live symmetry term moves with the strip"
                    );
                }
            }
        }
        assert!(
            !input_test_backends().contains(&Backend::Simd4) || refusals == 4,
            "the slotless refusal ran in all four Simd4 passes"
        );
        swaps
    }

    /// Issue #975: on a collapsed chain the real fader bank takes the seam -- it writes the right
    /// plane from the left one, so the chain copies nothing -- on every settled block, an
    /// automated bank's with no event in the block included, and takes the copy on a block where
    /// a right-channel lane ramps or a stored event lands; every strip's output is the unarmed
    /// (dual) twin's either way.
    ///
    /// Red if the fader stops taking the seam (`BuiltinStage`'s forward, the processor's
    /// override, the bank's or the stage's from-left body dropped: the copies come back), or takes
    /// it on a ramping block.
    #[cfg(feature = "test-support")]
    #[test]
    fn a_settled_fader_takes_the_seam_and_a_ramping_one_copies() {
        const TRACKS: usize = 13;
        for backend in input_test_backends() {
            let compiled = lane_swap_session(TRACKS, true, 128);
            let mut rig = lane_swap_rig(&compiled, backend, true);
            let mut twin = lane_swap_rig(&compiled, backend, false);
            assert!(
                rig.automated_lanes(0).contains(&1),
                "{backend:?}: an automated bank"
            );
            let mut rng = LaneSwapRng(0x975);
            for block in 0..4 {
                // Block 2: one right-channel fader ramp on strip 0's chain.
                if block == 2 {
                    let ramp = [(
                        0,
                        LaneSwapWrite::Fader(TrackFaderRecord::FaderDb {
                            lanes: BuiltinLaneSelector::Right,
                            db: -6.0,
                            smoothing_samples: 64,
                        }),
                    )];
                    rig.write(&ramp);
                    twin.write(&ramp);
                }
                let inputs = (0..TRACKS)
                    .map(|_| {
                        let left = (0..rig.frames)
                            .map(|_| rng.within(-0.8, 0.8))
                            .collect::<Vec<_>>();
                        [left.clone(), left]
                    })
                    .collect::<Vec<_>>();
                assert!(
                    rig.block(&inputs) == twin.block(&inputs),
                    "{backend:?}: block {block}"
                );
                // Strip 5's stored fader ride moves inside every block: its chain copies on each.
                let (ramped, riding) = (rig.position[0].0, rig.position[5].0);
                for (index, chain) in rig.chains.iter().enumerate() {
                    assert!(chain.is_collapsed(), "{backend:?}: chain {index} collapsed");
                    let copies = if index == riding {
                        block + 1
                    } else {
                        u64::from(block >= 2 && index == ramped)
                    };
                    assert_eq!(
                        chain.seam_copies(),
                        copies,
                        "{backend:?}: block {block}, chain {index}"
                    );
                }
            }
        }
    }
}
