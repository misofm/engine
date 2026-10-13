//! The live-control seam: committed live values in, prepared automation spans out.
//!
//! # Why this lives in the contract crate
//!
//! Issue #137 shipped the whole `miso.command.v1` ABI and discovered that no live parameter path
//! reaches a running plan: `graph::runtime` handed every effect a hardcoded empty
//! automation slice and `rack::EffectBankStage` handed every bank an empty one with
//! zero offsets. #140 closes that gap by feeding the admitted commands in as
//! [`PreparedAutomationSpan`]s -- which every effect's `process` already honours -- rather than by
//! inventing a second parameter path.
//!
//! Two render paths need exactly the same staging: the per-node dynamic rack (`graph`)
//! and the AoSoA SIMD racks (`rack`). Neither crate depends on the other's private
//! internals, and both already depend on this one, so the staging is written **once**, here, and
//! the two racks differ only in how many lanes they stage.
//!
//! # Latest-target cells (decision 15, D15-2; issue #1345)
//!
//! An effect's live parameter values, its bypass and a prepared-target owner's targets are held in
//! latest-target cells ([`engine::realtime::LatestCell`]), not in a queue. Control writes a cell
//! with [`EffectCellWriter::write`], and a write cannot fail on a value and never waits, however
//! many edits arrive between two render calls. Render reads each pending cell under the block's
//! live snapshot `S` and applies the newest write whose revision is at or below it; the writes it
//! replaced before any snapshot covered them are added to the session's `live_values_superseded`
//! (`CellReader::read_counted`). One lane holds
//!
//! - one **bypass** cell (one word);
//! - one **parameter** cell per `(parameter_index, channel)` of every `Block`-rate parameter: a
//!   `Shared` parameter has one `Both` cell, a `PerLane` one a `Left` and a `Right` cell (one
//!   word, the value's bits). A lane whose effect owns prepared targets (the parametric EQ) has
//!   none: its parameters ride its targets, and a bare parameter write is refused;
//! - one **target** cell per `(target slot, channel)` of a prepared-target owner
//!   ([`PREPARED_EFFECT_TARGET_WORDS`] words; a `Both` target writes both channel cells).
//!
//! The cell's index is its dirty bit and its place in the canonical drain order (D3): bypass, then
//! the parameter cells in `(parameter_index, channel)` order, then the target cells in `(slot,
//! channel)` order. Where both channel cells of one parameter or target were applied in a block
//! with equal words, the channels are a pair: the channel-symmetry witness keeps its `LIVE` term
//! and a target is applied once, as `Both`. Every other applied cell is a lone channel write and
//! clears `LIVE`. So the witness is folded from the cells' final contents, and the pairing rule of
//! issue #1004 is structural.
//!
//! # The frozen application rule
//!
//! A drained value takes effect at the **first sample of the next rendered block**, exactly as
//! #137's matrix retarget does: [`EffectControlLane::stage`] runs at the top of the block, before
//! a single sample is touched, and emits `AutomationSpanKind::Point` spans whose `start_sample` and
//! `end_sample` are that block's `first_sample`. Every launch effect accepts precisely that shape
//! (a `Point` at `first_sample` with bit-identical endpoints) and rejects anything else into
//! `ProcessReport::invalid_spans`, so the block boundary is proven by the effect contract rather
//! than asserted by the host.
//!
//! # The window holds every live cell (D4)
//!
//! A block stages one span for every parameter cell it reads, in the block that reads it: there is
//! no deferral and no drop. So the staging window is at least [`EffectControlLane::parameter_cells`]
//! spans, and every effect accepts that many: preparation raises an instance's
//! `automation_capacity` to at least [`EffectLaneShape::live_parameter_cells`] of its descriptor,
//! the window is exactly that capacity (the pairing rule of issue #1012), and a window shorter than
//! the lane needs is refused at bind ([`EffectControlLane::check_window`]).
//!
//! # Observation subscriptions stay a FIFO (D2)
//!
//! An [`EffectControlRecord::Observe`] arms or disarms a tap and is not a level: a later one never
//! replaces an earlier one. It rides a small bounded queue beside the cells, with one slot per
//! tap the effect declares, drained after the cells. Its writer reports a full queue as a typed
//! error and drops nothing.
//!
//! # Allocation-free by construction
//!
//! Every buffer here is sized at preparation. Reading cells and draining the queue move `Copy`
//! values into those buffers; there is no allocation, no lock, no drop and no unbounded loop.

use core::alloc::Layout;
use core::num::NonZeroUsize;
use engine::realtime::{
    CellRead, CellReader, CellWriter, Consumer, DirtyReader, DirtyWord, LatestCell, LiveSnapshot,
    LiveStamp, LiveValuesSuperseded, ObservationPublisher, ObservationWindow, Producer, QueueFull,
    QueueGeneration, SharedAllocation, bounded_spsc, bounded_spsc_retained_payload,
    observation_slot_retained_bytes,
};
use lane::kernels::pdc_delay_block;
use std::sync::Arc;

use crate::{
    AutomationRate, AutomationSpanKind, ChannelSymmetryWitness, EffectDescriptor,
    ObservationDescriptor, ObservationFold, ObservationSample, PREPARED_EFFECT_TARGET_WORDS,
    ParameterChannel, ParameterChannelPolicy, PreparedAutomationSpan, PreparedEffectTarget,
};

/// One admitted live control event for one prepared effect instance.
///
/// It is `Copy` and fixed-size. Addressing is *not* carried: a lane belongs to exactly one effect
/// instance (or, in a bank, to exactly one lane of one slot), so the record only says what to
/// change. [`EffectCellWriter::write`] puts a [`Parameter`](Self::Parameter),
/// [`PreparedTarget`](Self::PreparedTarget) or [`Bypass`](Self::Bypass) value into its cell, and
/// [`EffectCellWriter::try_push_observe`] queues an [`Observe`](Self::Observe).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EffectControlRecord {
    /// Retarget one declared parameter of one channel.
    ///
    /// `parameter_index` is the index into `EffectDescriptor::parameters`, never the wire
    /// `parameter_id`: the translation is an admission-time lookup, off the render thread.
    /// `channel` already obeys the parameter's
    /// [`ParameterChannelPolicy`](crate::ParameterChannelPolicy) -- a `Shared` parameter arrives as
    /// [`ParameterChannel::Both`] and a `PerLane` parameter arrives as one record per lane, because
    /// every launch effect counts a policy-violating span as invalid rather than applying it.
    Parameter {
        /// Index into the descriptor's parameter table.
        parameter_index: u32,
        /// Which lane this value addresses.
        channel: ParameterChannel,
        /// The new value, already domain-checked by the admitting host.
        value: f32,
    },
    /// Apply one off-render prepared effect target at this block boundary.
    PreparedTarget(PreparedEffectTarget),
    /// Arm or disarm one declared observation tap (issue #143 D3, level 2).
    ///
    /// It rides the lane's observation queue, in order, beside the cells. `tap_index` is the index
    /// into the descriptor's observation table, never the wire `tap_id`; `window_blocks` of `0`
    /// means "the plan's default".
    Observe {
        /// Index into the descriptor's observation table.
        tap_index: u32,
        /// Whether the tap is to be read at all after this block boundary.
        armed: bool,
        /// Render blocks per published window, or `0` for the plan's default.
        window_blocks: u32,
    },
    /// Set this instance's live bypass.
    ///
    /// Bypass is not a parameter and has no span: it is applied by the rack's latency-preserving
    /// shunt (see [`BypassShunt`]), never by the effect, because
    /// [`PreparedEffectMetadata::bypass`](crate::PreparedEffectMetadata) is baked into the
    /// program key and a bank's lanes must be able to disagree about it.
    Bypass(bool),
}

/// Marks a parameter that has no live cell in [`EffectLaneShape::first`].
const NOT_LIVE: u32 = u32::MAX;

/// One live parameter cell: the `(parameter_index, channel)` it holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParameterCell {
    parameter_index: u32,
    channel: ParameterChannel,
}

/// One drain unit of the parameter cells: a `Shared` parameter's one `Both` cell, or a `PerLane`
/// parameter's `Left` cell with its `Right` twin at `first + 1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParameterGroup {
    first: u32,
    pair: bool,
}

/// The cells one lane holds: which parameters are live, how many target slots there are, and the
/// cell index of each. Built once, at preparation, from the effect's descriptor.
///
/// Cell indices run bypass (`0`), the parameter cells (`1..=P`), then the target cells (`1 + P ..`,
/// slot-major, `Left` before `Right`).
#[derive(Debug, Eq, PartialEq)]
pub struct EffectLaneShape {
    cells: Box<[ParameterCell]>,
    groups: Box<[ParameterGroup]>,
    /// The first parameter cell of each descriptor parameter up to the last live one, or
    /// [`NOT_LIVE`].
    first: Box<[u32]>,
    target_slots: usize,
}

impl EffectLaneShape {
    /// A shape with one cell group per `(parameter index, channel policy)` of `live`, which must
    /// be strictly increasing in the index, and `target_slots` target slots (`0` for an effect
    /// that owns no prepared targets).
    ///
    /// # Panics
    ///
    /// When `live` is not strictly increasing in the parameter index. Construction is a
    /// control-plane operation and the table is a compiler product.
    #[must_use]
    pub fn from_live_parameters(
        live: impl IntoIterator<Item = (u32, ParameterChannelPolicy)>,
        target_slots: usize,
    ) -> Self {
        let mut cells = Vec::new();
        let mut groups = Vec::new();
        let mut firsts: Vec<(u32, u32)> = Vec::new();
        let mut previous: Option<u32> = None;
        for (parameter_index, policy) in live {
            assert!(
                previous.is_none_or(|previous| parameter_index > previous),
                "live parameters are strictly increasing in the parameter index"
            );
            previous = Some(parameter_index);
            let first = u32::try_from(cells.len()).expect("a lane's parameter cells fit u32");
            firsts.push((parameter_index, first));
            match policy {
                ParameterChannelPolicy::Shared => {
                    cells.push(ParameterCell {
                        parameter_index,
                        channel: ParameterChannel::Both,
                    });
                    groups.push(ParameterGroup { first, pair: false });
                }
                ParameterChannelPolicy::PerLane => {
                    cells.push(ParameterCell {
                        parameter_index,
                        channel: ParameterChannel::Left,
                    });
                    cells.push(ParameterCell {
                        parameter_index,
                        channel: ParameterChannel::Right,
                    });
                    groups.push(ParameterGroup { first, pair: true });
                }
            }
        }
        let table = firsts.last().map_or(0, |(index, _)| *index as usize + 1);
        let mut first = vec![NOT_LIVE; table];
        for (parameter_index, cell) in firsts {
            first[parameter_index as usize] = cell;
        }
        Self {
            cells: cells.into_boxed_slice(),
            groups: groups.into_boxed_slice(),
            first: first.into_boxed_slice(),
            target_slots,
        }
    }

    /// The shape of `descriptor`'s lane: one cell group per `Block`-rate parameter when the effect
    /// owns no prepared targets (`target_slots == 0`), and none when it does, because a
    /// prepared-target owner's parameters ride its targets (D1).
    #[must_use]
    pub fn from_descriptor(descriptor: &EffectDescriptor, target_slots: usize) -> Self {
        if target_slots != 0 {
            return Self::from_live_parameters(core::iter::empty(), target_slots);
        }
        Self::from_live_parameters(live_parameters(descriptor), 0)
    }

    /// The live parameter cell count of `descriptor`: one per `Shared` `Block`-rate parameter and
    /// two per `PerLane` one, whether or not the effect's lane holds them or its parameters ride
    /// targets. This is the least `automation_capacity` an instance of the effect is prepared with
    /// (D4). It depends on the descriptor alone, so every instance of one effect has one capacity,
    /// a live lane changes no program key, and no bank cohort splits.
    #[must_use]
    pub fn live_parameter_cells(descriptor: &EffectDescriptor) -> usize {
        live_parameters(descriptor)
            .map(|(_, policy)| match policy {
                ParameterChannelPolicy::Shared => 1,
                ParameterChannelPolicy::PerLane => 2,
            })
            .sum()
    }

    /// The parameter cells this lane holds: the most spans a block stages.
    #[must_use]
    pub fn parameter_cells(&self) -> usize {
        self.cells.len()
    }

    /// The target slots this lane holds cells for (`0` for an effect without prepared targets).
    #[must_use]
    pub const fn target_slots(&self) -> usize {
        self.target_slots
    }

    /// Every cell of the lane: bypass, parameters and targets. Also the dirty bit count.
    fn cell_count(&self) -> usize {
        1 + self.cells.len() + 2 * self.target_slots
    }

    /// The dirty bit (and cell index) of target cell `index`.
    fn target_bit(&self, index: usize) -> usize {
        1 + self.cells.len() + index
    }

    /// The parameter cell of `(parameter_index, channel)`, or why there is none.
    fn locate(
        &self,
        parameter_index: u32,
        channel: ParameterChannel,
    ) -> Result<usize, EffectWriteRefusal> {
        let first = *self
            .first
            .get(parameter_index as usize)
            .filter(|first| **first != NOT_LIVE)
            .ok_or(EffectWriteRefusal::NotLive)? as usize;
        let paired = self.cells[first].channel == ParameterChannel::Left;
        match (paired, channel) {
            (false, ParameterChannel::Both) | (true, ParameterChannel::Left) => Ok(first),
            (true, ParameterChannel::Right) => Ok(first + 1),
            _ => Err(EffectWriteRefusal::Channel),
        }
    }
}

/// The `Block`-rate parameters of `descriptor`, as `(index, channel policy)`.
fn live_parameters(
    descriptor: &EffectDescriptor,
) -> impl Iterator<Item = (u32, ParameterChannelPolicy)> + '_ {
    descriptor
        .parameters
        .iter()
        .enumerate()
        .filter(|(_, parameter)| {
            parameter.automatable && parameter.automation_rate == AutomationRate::Block
        })
        .map(|(index, parameter)| {
            (
                u32::try_from(index).expect("a descriptor's parameters fit u32"),
                parameter.channel_policy,
            )
        })
}

/// Why a record has no cell on a lane. Nothing was written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectWriteRefusal {
    /// The parameter has no live cell on this lane: it is not a `Block`-rate parameter, the index
    /// is outside the descriptor, or the lane's effect owns prepared targets (its parameters ride
    /// its targets).
    NotLive,
    /// The channel does not match the parameter's policy: a `Shared` parameter takes `Both` alone,
    /// a `PerLane` one `Left` or `Right`.
    Channel,
    /// The target's slot is outside the lane's target slots, or the lane holds no target cells.
    Target,
    /// The record is an observation subscription, which the queue takes
    /// ([`EffectCellWriter::try_push_observe`]).
    Observe,
}

/// Why an observation subscription was not queued. Nothing was queued.
#[derive(Debug)]
pub enum ObservePushError {
    /// The lane has no observation queue: its effect declares no tap, or the plan named no
    /// observation capacity.
    Unbound,
    /// The queue holds as many records as the effect has taps. The record is returned.
    Full(QueueFull<EffectControlRecord>),
}

/// What one lane shares between its writer (control) and its reader (render): the cells and their
/// dirty words.
struct EffectCells {
    shape: EffectLaneShape,
    bypass: LatestCell<1>,
    parameters: Box<[LatestCell<1>]>,
    targets: Box<[LatestCell<PREPARED_EFFECT_TARGET_WORDS>]>,
    /// One word per 64 cells.
    dirty: Box<[DirtyWord]>,
}

impl EffectCells {
    fn new(shape: EffectLaneShape) -> Self {
        let dirty_words = shape.cell_count().div_ceil(64);
        Self {
            parameters: (0..shape.parameter_cells())
                .map(|_| LatestCell::new())
                .collect(),
            targets: (0..2 * shape.target_slots())
                .map(|_| LatestCell::new())
                .collect(),
            dirty: (0..dirty_words).map(|_| DirtyWord::new()).collect(),
            bypass: LatestCell::new(),
            shape,
        }
    }
}

/// The heap an effect lane or writer retains: every allocation, summed, and the largest one (a
/// zero-length array allocates nothing and counts as nothing).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectLaneResources {
    /// Every allocation, summed.
    pub total_bytes: u64,
    /// The largest single allocation.
    pub largest_allocation_bytes: u64,
}

impl EffectLaneResources {
    fn add(&mut self, bytes: usize) {
        if bytes != 0 {
            let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
            self.total_bytes = self.total_bytes.saturating_add(bytes);
            self.largest_allocation_bytes = self.largest_allocation_bytes.max(bytes);
        }
    }

    fn add_array<T>(&mut self, length: usize) {
        self.add(length.saturating_mul(core::mem::size_of::<T>()));
    }
}

/// Where a record lands on a lane, with the words it writes there.
enum Address {
    Bypass(bool),
    /// A parameter cell and the value's bits.
    Parameter(usize, u32),
    /// The first and last target cell of a target (a `Both` target addresses two) and its words.
    Targets(usize, usize, [u32; PREPARED_EFFECT_TARGET_WORDS]),
}

/// Control's write end of one effect lane: the cell writers and the observation queue's producer.
///
/// A write cannot fail on a value and never waits. The caller publishes the stamp's revision on
/// the plan's gate after every write of its transaction (#1312 D7); a value is not applied before
/// that.
pub struct EffectCellWriter {
    cells: Arc<EffectCells>,
    bypass: CellWriter<1>,
    parameters: Box<[CellWriter<1>]>,
    targets: Box<[CellWriter<PREPARED_EFFECT_TARGET_WORDS>]>,
    observe: Option<Producer<EffectControlRecord>>,
}

impl EffectCellWriter {
    fn address(&self, record: &EffectControlRecord) -> Result<Address, EffectWriteRefusal> {
        match *record {
            EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value,
            } => self
                .cells
                .shape
                .locate(parameter_index, channel)
                .map(|index| Address::Parameter(index, value.to_bits())),
            EffectControlRecord::PreparedTarget(target) => {
                let slot = target.slot as usize;
                if slot >= self.cells.shape.target_slots {
                    return Err(EffectWriteRefusal::Target);
                }
                let (first, last) = match target.channel {
                    ParameterChannel::Left => (2 * slot, 2 * slot),
                    ParameterChannel::Right => (2 * slot + 1, 2 * slot + 1),
                    ParameterChannel::Both => (2 * slot, 2 * slot + 1),
                };
                Ok(Address::Targets(first, last, target.words))
            }
            EffectControlRecord::Bypass(bypass) => Ok(Address::Bypass(bypass)),
            EffectControlRecord::Observe { .. } => Err(EffectWriteRefusal::Observe),
        }
    }

    /// Why `record` has no cell on this lane, without writing: `Ok` when [`Self::write`] would
    /// write it. An [`EffectControlRecord::Observe`] is refused here, because it is not a cell.
    ///
    /// # Errors
    ///
    /// [`EffectWriteRefusal`].
    pub fn check(&self, record: &EffectControlRecord) -> Result<(), EffectWriteRefusal> {
        self.address(record).map(|_| ())
    }

    /// Write `record` as the newest value of the cell(s) it addresses, stamped with the revision of
    /// the transaction that commits it. A `Both` target writes the left and the right cell.
    ///
    /// # Errors
    ///
    /// [`EffectWriteRefusal`], with nothing written. The caller has checked the record
    /// ([`Self::check`]) before the first write of its transaction, so a value is never dropped
    /// for want of a cell.
    pub fn write(
        &mut self,
        stamp: LiveStamp<'_>,
        record: EffectControlRecord,
    ) -> Result<(), EffectWriteRefusal> {
        let cells = &*self.cells;
        match self.address(&record)? {
            Address::Bypass(bypass) => {
                self.bypass.write(
                    &cells.bypass,
                    &cells.dirty[0],
                    0,
                    stamp,
                    [u32::from(bypass)],
                );
            }
            Address::Parameter(index, bits) => {
                let bit = 1 + index;
                self.parameters[index].write(
                    &cells.parameters[index],
                    &cells.dirty[bit / 64],
                    (bit % 64) as u32,
                    stamp,
                    [bits],
                );
            }
            Address::Targets(first, last, words) => {
                for index in first..=last {
                    let bit = cells.shape.target_bit(index);
                    self.targets[index].write(
                        &cells.targets[index],
                        &cells.dirty[bit / 64],
                        (bit % 64) as u32,
                        stamp,
                        words,
                    );
                }
            }
        }
        Ok(())
    }

    /// Queue one observation subscription, in order, for the lane's next drain.
    ///
    /// # Errors
    ///
    /// [`ObservePushError::Unbound`] for a lane with no queue, [`ObservePushError::Full`] when the
    /// queue holds as many records as the effect has taps. Nothing is queued: the host checks the
    /// room of a whole submission before it writes anything, so an acked batch is never partly
    /// queued.
    pub fn try_push_observe(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
    ) -> Result<(), ObservePushError> {
        let queue = self.observe.as_mut().ok_or(ObservePushError::Unbound)?;
        queue
            .try_push(EffectControlRecord::Observe {
                tap_index,
                armed,
                window_blocks,
            })
            .map_err(ObservePushError::Full)
    }

    /// The observation queue's capacity, `0` for a lane without one.
    #[must_use]
    pub fn observe_capacity(&self) -> usize {
        self.observe.as_ref().map_or(0, Producer::capacity)
    }

    /// Free slots of the observation queue, `0` for a lane without one.
    #[must_use]
    pub fn observe_available_capacity(&self) -> usize {
        self.observe
            .as_ref()
            .map_or(0, Producer::available_capacity)
    }

    /// Producer-local count of observation records queued.
    #[must_use]
    pub fn observe_success_count(&self) -> u64 {
        self.observe.as_ref().map_or(0, Producer::success_count)
    }

    /// Producer-local count of observation records refused because the queue was full.
    #[must_use]
    pub fn observe_full_count(&self) -> u64 {
        self.observe.as_ref().map_or(0, Producer::full_count)
    }

    /// The lane's cell table.
    #[must_use]
    pub fn shape(&self) -> &EffectLaneShape {
        &self.cells.shape
    }

    /// Heap this writer owns beyond the lane's shared storage: its cell writers.
    #[must_use]
    pub fn retained_resources(&self) -> EffectLaneResources {
        let mut resources = EffectLaneResources::default();
        resources.add_array::<CellWriter<1>>(self.parameters.len());
        resources.add_array::<CellWriter<PREPARED_EFFECT_TARGET_WORDS>>(self.targets.len());
        resources
    }

    /// Test-only: how many writes the bypass cell has taken, then each parameter cell, then each
    /// target cell, in cell order. Valid while no write is in flight, which a quiescent test
    /// thread guarantees.
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_counts(&self) -> Vec<u64> {
        fn count<const N: usize>(peek: Option<([u32; N], u64)>) -> u64 {
            peek.map_or(0, |(_, sequence)| sequence)
        }
        let reader = CellReader::new();
        let cells = &*self.cells;
        core::iter::once(count(reader.peek_unread(&cells.bypass)))
            .chain(
                cells
                    .parameters
                    .iter()
                    .map(|cell| count(reader.peek_unread(cell))),
            )
            .chain(
                cells
                    .targets
                    .iter()
                    .map(|cell| count(reader.peek_unread(cell))),
            )
            .collect()
    }
}

/// What an effect lane is built from.
#[derive(Debug)]
pub struct EffectLaneSpec {
    /// The lane's cell table.
    pub shape: EffectLaneShape,
    /// The instance's bypass when the lane is built: the session's, for an effect that lowers it.
    pub bypass: bool,
    /// The observation queue's depth: the effect's declared tap count, or `0` for no queue (an
    /// effect without taps, or a plan with no observation capacity).
    pub observation_depth: usize,
}

/// The cells and observation queue of one effect instance, as its two ends: control's writer and
/// render's lane.
///
/// Off the render thread: this allocates the shared cells, both ends' arrays and the queue.
/// `superseded` is the session's `live_values_superseded` count the lane's reader adds the writes
/// it replaces to. `None` only if the observation queue cannot be built.
#[must_use]
pub fn effect_control_lane(
    spec: EffectLaneSpec,
    superseded: &LiveValuesSuperseded,
) -> Option<(EffectCellWriter, EffectControlLane)> {
    let EffectLaneSpec {
        shape,
        bypass,
        observation_depth,
    } = spec;
    let (producer, consumer) = match NonZeroUsize::new(observation_depth) {
        Some(depth) => {
            let (producer, consumer) =
                bounded_spsc::<EffectControlRecord>(depth, QueueGeneration(0)).ok()?;
            (Some(producer), Some(consumer))
        }
        None => (None, None),
    };
    let parameter_cells = shape.parameter_cells();
    let target_cells = 2 * shape.target_slots();
    let dirty_words = shape.cell_count().div_ceil(64);
    let cells = Arc::new(EffectCells::new(shape));
    let mut symmetry = ChannelSymmetryWitness::SYMMETRIC;
    symmetry.set(ChannelSymmetryWitness::UNBYPASSED, !bypass);
    let writer = EffectCellWriter {
        cells: Arc::clone(&cells),
        bypass: CellWriter::new(),
        parameters: (0..parameter_cells).map(|_| CellWriter::new()).collect(),
        targets: (0..target_cells).map(|_| CellWriter::new()).collect(),
        observe: producer,
    };
    let lane = EffectControlLane {
        cells: Some(cells),
        bypass_reader: CellReader::new(),
        parameter_readers: (0..parameter_cells).map(|_| CellReader::new()).collect(),
        target_readers: (0..target_cells).map(|_| CellReader::new()).collect(),
        dirty: (0..dirty_words).map(|_| DirtyReader::new()).collect(),
        pending: vec![0; dirty_words].into_boxed_slice(),
        superseded: Some(superseded.clone()),
        targets: vec![
            PreparedEffectTarget {
                slot: 0,
                channel: ParameterChannel::Both,
                words: [0; PREPARED_EFFECT_TARGET_WORDS],
            };
            target_cells
        ]
        .into_boxed_slice(),
        staged_targets: 0,
        observe: consumer,
        bypass,
        symmetry,
    };
    Some((writer, lane))
}

/// One prepared live-control lane for one effect instance, or one lane of one bank slot: render's
/// end of the cells and the observation queue.
///
/// The shared cells are an `Arc`, so either end may drop first and the last owner frees them; the
/// C ABI drops a reclaimed plan before its writers, on the control thread.
///
/// # A lane without cells (issue #1087)
///
/// A session `bypass` is per-lane state, not a prepared program, for every effect whose bypass
/// `effect_compiler::lowers_session_bypass` (all but the delay and the multiband): preparation
/// lowers it to a prepared `bypass = false` plus this lane's initial bypass, so a bypassed track
/// keeps its place in its effect bank and the rack's [`BypassShunt`] selects the dry signal for
/// exactly that lane. A session with no live controls still needs somewhere to hold that bit, so
/// [`without_channel`](Self::without_channel) builds a lane with no cells and no queue: nothing
/// can ever be written to it, its drain stages nothing, and its bypass is the prepared one for the
/// life of the plan. Live controls replace it with a lane built by [`effect_control_lane`], seeded
/// from the same bit.
pub struct EffectControlLane {
    /// `None` for a lane that only carries a prepared bypass ([`Self::without_channel`]).
    cells: Option<Arc<EffectCells>>,
    bypass_reader: CellReader,
    parameter_readers: Box<[CellReader]>,
    target_readers: Box<[CellReader]>,
    /// Render's half of each dirty word: the cells whose newest write was above the snapshot that
    /// last read them.
    dirty: Box<[DirtyReader]>,
    /// This block's pending cells, one word per 64 cells; scratch, rewritten every block.
    pending: Box<[u64]>,
    /// The session's count of replaced writes; `None` exactly when `cells` is.
    superseded: Option<LiveValuesSuperseded>,
    /// The targets applied by the last [`Self::stage`], at most one per target cell.
    targets: Box<[PreparedEffectTarget]>,
    /// Number of valid entries in the retained target prefix from the last stage call.
    staged_targets: usize,
    /// The observation queue, if the lane has one and a rack has not taken it
    /// ([`Self::take_observe_queue`]).
    observe: Option<Consumer<EffectControlRecord>>,
    /// Live bypass state, retained across blocks so a rendered block always knows it.
    bypass: bool,
    /// This instance's live channel-symmetry terms, retained across blocks exactly as `bypass`
    /// is, and for the same reason: the terms describe what the *drained* values did, so
    /// they have to survive the block that drained them.
    ///
    /// # Why the witness is maintained here and not at the host's admission call
    ///
    /// The cells **are** the admission boundary. A value is admitted into the rendered state at
    /// the drain, on the render thread, at the top of the block that first applies it -- so a
    /// witness folded in here cannot disagree with the state it describes, and it needs no
    /// atomic and no second channel to reach the collapse dispatch. A host-side bit could not:
    /// `PreparedRenderPlan` is `Send` and **not** `Sync`, so a value the control thread owns is
    /// unreadable from the render thread by construction.
    ///
    /// Only the two live terms move here. `SOURCE`, `DESIGNED` and `RESTORED` are decided off
    /// render, at preparation and at restore, and are conjoined by the stage that owns this lane.
    symmetry: ChannelSymmetryWitness,
}

impl EffectControlLane {
    /// A lane with no cells that carries only this instance's prepared bypass (issue #1087).
    ///
    /// Off the render thread, and it allocates nothing. It exists so that a session-bypassed
    /// instance reaches the rack's latency-preserving [`BypassShunt`] whether or not live
    /// controls are attached: the rack builds a shunt for an instance or a bank slot that holds any
    /// lane, and restores the dry signal into every lane whose [`bypassed`](Self::bypassed) is
    /// true. The witness' `UNBYPASSED` term is seeded from `bypass` exactly as
    /// [`effect_control_lane`] seeds it, so a statically bypassed lane declines the collapse from
    /// the plan's first block.
    #[must_use]
    pub fn without_channel(bypass: bool) -> Self {
        let mut symmetry = ChannelSymmetryWitness::SYMMETRIC;
        symmetry.set(ChannelSymmetryWitness::UNBYPASSED, !bypass);
        Self {
            cells: None,
            bypass_reader: CellReader::new(),
            parameter_readers: Box::default(),
            target_readers: Box::default(),
            dirty: Box::default(),
            pending: Box::default(),
            superseded: None,
            targets: Box::default(),
            staged_targets: 0,
            observe: None,
            bypass,
            symmetry,
        }
    }

    /// Whether this lane has cells a control plane can write.
    ///
    /// `false` only for a [`without_channel`](Self::without_channel) lane, whose bypass is fixed
    /// at preparation.
    #[must_use]
    pub const fn has_channel(&self) -> bool {
        self.cells.is_some()
    }

    /// The parameter cells of this lane: the least staging window it needs.
    #[must_use]
    pub fn parameter_cells(&self) -> usize {
        self.cells
            .as_ref()
            .map_or(0, |cells| cells.shape.parameter_cells())
    }

    /// Check, at bind, that a staging window of `window` spans holds every live parameter cell
    /// (D4): a block stages one span per parameter cell it reads, and none is ever deferred or
    /// dropped.
    ///
    /// # Errors
    ///
    /// The number of spans the lane needs, when `window` is shorter.
    pub fn check_window(&self, window: usize) -> Result<(), usize> {
        let needed = self.parameter_cells();
        if window >= needed {
            Ok(())
        } else {
            Err(needed)
        }
    }

    /// Whether this lane still holds an observation queue (a rack has not taken it).
    #[must_use]
    pub fn has_observe_queue(&self) -> bool {
        self.observe.is_some()
    }

    /// Move the observation queue's consumer out of the lane. The racks hold the consumers of all
    /// their lanes in one array and drain them in a loop of their own, so each pop sits in its own
    /// loop body (D8). Off the render thread.
    #[must_use]
    pub fn take_observe_queue(&mut self) -> Option<Consumer<EffectControlRecord>> {
        self.observe.take()
    }

    /// Exact heap this lane and its shared cells retain, each allocation counted once, including
    /// the observation queue if the lane still holds it. The lane's own struct is charged by its
    /// owner. The writer's arrays are [`EffectCellWriter::retained_resources`].
    ///
    /// This is a control-plane accounting query; it is never called from a realtime-marked method.
    #[must_use]
    pub fn retained_resources(&self) -> EffectLaneResources {
        let mut resources = EffectLaneResources::default();
        if let Some(cells) = self.cells.as_deref() {
            resources.add(Layout::new::<SharedAllocation<EffectCells>>().size());
            resources.add_array::<ParameterCell>(cells.shape.cells.len());
            resources.add_array::<ParameterGroup>(cells.shape.groups.len());
            resources.add_array::<u32>(cells.shape.first.len());
            resources.add_array::<LatestCell<1>>(cells.parameters.len());
            resources.add_array::<LatestCell<PREPARED_EFFECT_TARGET_WORDS>>(cells.targets.len());
            resources.add_array::<DirtyWord>(cells.dirty.len());
        }
        resources.add_array::<CellReader>(self.parameter_readers.len());
        resources.add_array::<CellReader>(self.target_readers.len());
        resources.add_array::<DirtyReader>(self.dirty.len());
        resources.add_array::<u64>(self.pending.len());
        resources.add_array::<PreparedEffectTarget>(self.targets.len());
        if let Some(queue) = self.observe.as_ref()
            && let Some(capacity) = NonZeroUsize::new(queue.capacity())
            && let Ok(payload) = bounded_spsc_retained_payload::<EffectControlRecord>(capacity)
        {
            resources.add(payload.ring_header_bytes);
            resources.add(payload.slot_payload_bytes);
        }
        resources
    }

    // REALTIME_POLICY_BEGIN

    /// This lane's live channel-symmetry terms as of the last [`stage`](Self::stage).
    ///
    /// `LIVE` and `UNBYPASSED` are the only terms this value speaks to; the other three are set,
    /// so conjoining it with the stage's prepared witness gives the whole answer and never
    /// over-claims.
    #[must_use]
    pub const fn symmetry(&self) -> ChannelSymmetryWitness {
        self.symmetry
    }

    /// Whether this instance is bypassed as of the last [`stage`](Self::stage).
    #[must_use]
    pub const fn bypassed(&self) -> bool {
        self.bypass
    }

    /// The targets applied by the most recent [`stage`](Self::stage), in `(slot, channel)` order.
    #[must_use]
    pub fn prepared_targets(&self) -> &[PreparedEffectTarget] {
        &self.targets[..self.staged_targets]
    }

    /// Read every pending cell under the block's snapshot `snapshot` and stage what they hold: the
    /// bypass first, then the parameter spans into `staging` in canonical `(parameter_index,
    /// channel)` order, then the prepared targets in `(slot, channel)` order (D3). It pops
    /// nothing: the observation queue is drained apart ([`drain_observations`](Self::drain_observations)
    /// per node, the rack's own loop per bank).
    ///
    /// Each applied cell adds the writes it replaced to the session count. When both channel
    /// cells of one parameter or target were applied with equal words, the channels are a pair and
    /// the witness keeps `LIVE` (a target is applied once, as `Both`); any other applied cell is a
    /// lone channel write and clears it. Every value is admitted into the witness through
    /// [`ChannelSymmetryWitness::admit`], so a record kind cannot reach render state without
    /// declaring what it does to the witness.
    ///
    /// `staging` is the caller's preallocated window for this lane, at least
    /// [`Self::parameter_cells`] spans long (checked at bind with [`Self::check_window`]) and
    /// exactly the effect's `automation_capacity`, which the pairing rule depends on
    /// ([`EffectProcessBlock::check_automation_window`](crate::EffectProcessBlock::check_automation_window)):
    /// a staged twin pair is never cut off by the effect's `span_index < automation_capacity`
    /// limit.
    pub fn stage(
        &mut self,
        staging: &mut [PreparedAutomationSpan],
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Staged {
        let Self {
            cells,
            bypass_reader,
            parameter_readers,
            target_readers,
            dirty,
            pending,
            superseded,
            targets,
            staged_targets,
            bypass,
            symmetry,
            ..
        } = self;
        *staged_targets = 0;
        // A lane without cells has nothing to read: the window, the target prefix and the
        // witness stay exactly as the last block left them.
        let (Some(cells), Some(superseded)) = (cells.as_deref(), superseded.as_ref()) else {
            return Staged::default();
        };
        // After the snapshot: a write at or below `S` whose bit was set between a take and the
        // snapshot would otherwise wait a block. The deferred mask of an earlier block joins.
        let mut any = 0_u64;
        for ((mask, reader), word) in pending.iter_mut().zip(dirty.iter_mut()).zip(&cells.dirty) {
            *mask = reader.pending(word);
            any |= *mask;
        }
        if any == 0 {
            return Staged::default();
        }
        let mut reads = Reads {
            pending,
            dirty,
            snapshot,
            superseded,
        };
        if let Some([word]) = reads.read(0, &cells.bypass, bypass_reader) {
            let value = word != 0;
            *bypass = value;
            symmetry.admit(&EffectControlRecord::Bypass(value));
        }
        let mut staged = 0_usize;
        for group in cells.shape.groups.iter() {
            let first = group.first as usize;
            let parameter_index = cells.shape.cells[first].parameter_index;
            let mut applied = [None; 2];
            for (offset, slot) in applied
                .iter_mut()
                .enumerate()
                .take(1 + usize::from(group.pair))
            {
                *slot = reads
                    .read(
                        1 + first + offset,
                        &cells.parameters[first + offset],
                        &mut parameter_readers[first + offset],
                    )
                    .map(|[bits]| bits);
            }
            // A pair whose final values are bit-equal is one both-channel write that arrived as
            // two cells (issue #1004): the channels stay where a `Both` would leave them.
            let paired = group.pair && applied[0].is_some() && applied[0] == applied[1];
            for (offset, bits) in applied.into_iter().enumerate() {
                let Some(bits) = bits else {
                    continue;
                };
                let channel = if group.pair {
                    [ParameterChannel::Left, ParameterChannel::Right][offset]
                } else {
                    ParameterChannel::Both
                };
                let value = f32::from_bits(bits);
                staging[staged] = PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: value,
                    end_value: value,
                };
                staged += 1;
                // The one hook: a record kind cannot reach the render state without declaring
                // what it does to the witness (`symmetry::LiveControlRecord`).
                symmetry.admit(&EffectControlRecord::Parameter {
                    parameter_index,
                    channel: if paired {
                        ParameterChannel::Both
                    } else {
                        channel
                    },
                    value,
                });
            }
        }
        for slot in 0..cells.shape.target_slots {
            let mut applied = [None; 2];
            for (channel, words) in applied.iter_mut().enumerate() {
                let index = 2 * slot + channel;
                *words = reads.read(
                    cells.shape.target_bit(index),
                    &cells.targets[index],
                    &mut target_readers[index],
                );
            }
            let slot = slot as u32;
            let target = |channel, words| PreparedEffectTarget {
                slot,
                channel,
                words,
            };
            let records = match applied {
                [Some(left), Some(right)] if left == right => {
                    [Some(target(ParameterChannel::Both, left)), None]
                }
                [left, right] => [
                    left.map(|words| target(ParameterChannel::Left, words)),
                    right.map(|words| target(ParameterChannel::Right, words)),
                ],
            };
            for record in records.into_iter().flatten() {
                symmetry.admit(&EffectControlRecord::PreparedTarget(record));
                targets[*staged_targets] = record;
                *staged_targets += 1;
            }
        }
        Staged {
            staged,
            staged_targets: *staged_targets,
        }
    }

    /// Drain this lane's observation queue and apply each subscription to `observation`, in order.
    /// Returns the subscriptions the plan had no capacity to apply (`observation` was `None`, or
    /// the tap is not declared), which the control plane refuses before they reach a queue: zero
    /// by construction.
    ///
    /// For a per-node effect, which calls it once per block, in no loop. The racks drain their
    /// lanes' queues in a loop of their own ([`take_observe_queue`](Self::take_observe_queue)).
    pub fn drain_observations(
        &mut self,
        first_sample: u64,
        mut observation: Option<&mut ObservationLane>,
    ) -> u32 {
        let mut unbound = 0_u32;
        if let Some(observe) = self.observe.as_mut() {
            let available = observe.available_at_entry();
            let mut remaining = available;
            while remaining != 0 {
                remaining -= 1;
                let Ok(record) = observe.try_pop() else {
                    // The entry snapshot and SPSC ownership guarantee this cannot happen. Keep the
                    // drain bounded if a malformed test double violates that invariant.
                    break;
                };
                if !apply_observe(record, first_sample, observation.as_deref_mut()) {
                    unbound = unbound.saturating_add(1);
                }
            }
        }
        unbound
    }
}

/// One block's reads of a lane's pending cells: the pending mask, render's half of each dirty
/// word, the block's snapshot and the session count.
struct Reads<'a> {
    pending: &'a [u64],
    dirty: &'a mut [DirtyReader],
    snapshot: LiveSnapshot,
    superseded: &'a LiveValuesSuperseded,
}

impl Reads<'_> {
    /// Read the cell at dirty bit `bit` if it is pending: the newest write at or below the
    /// snapshot, if it was not applied before. The writes it replaced join the session count.
    fn read<const N: usize>(
        &mut self,
        bit: usize,
        cell: &LatestCell<N>,
        reader: &mut CellReader,
    ) -> Option<[u32; N]> {
        if self.pending[bit / 64] & (1 << (bit % 64)) == 0 {
            return None;
        }
        match reader.read_counted(
            cell,
            self.snapshot,
            &mut self.dirty[bit / 64],
            (bit % 64) as u32,
            self.superseded,
        ) {
            CellRead::Applied { words, .. } => Some(words),
            CellRead::Unchanged => None,
        }
    }
}

/// Apply one drained [`EffectControlRecord::Observe`] to `observation`, the instance's tap state
/// when the plan is observation-capable and `None` otherwise. Returns `false` when it could not be
/// applied -- the plan has no observation capacity, or the tap is not declared -- which the
/// control plane turns into `ObservationUnbound` before a record reaches a queue.
///
/// An `Observe` record emits **no span**: it changes what is read after the block, never what the
/// block renders, so it does not touch the staging window. Any other record kind is not an
/// observation and is reported as not applied.
pub fn apply_observe(
    record: EffectControlRecord,
    first_sample: u64,
    observation: Option<&mut ObservationLane>,
) -> bool {
    let EffectControlRecord::Observe {
        tap_index,
        armed,
        window_blocks,
    } = record
    else {
        return false;
    };
    observation.is_some_and(|lane| lane.arm(tap_index, armed, window_blocks, first_sample))
}
// REALTIME_POLICY_END

/// What one [`EffectControlLane::stage`] call produced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Staged {
    /// Spans written to the front of the staging window, in canonical order.
    pub staged: usize,
    /// Prepared targets retained in the lane-owned prefix.
    pub staged_targets: usize,
}

/// One declared tap's arm state and open window, for one prepared instance or one bank lane.
///
/// Sixteen bytes of accumulator (`peak_left`, `peak_right`, `blocks`, `window_blocks`) plus the
/// window's own bookkeeping. All of it is allocated at preparation from the **declared menu**, so
/// arming allocates nothing and disarming frees nothing: subscribe is a flag, not a resource.
#[derive(Debug)]
struct ObservationTap {
    publisher: ObservationPublisher,
    /// The declared fold, copied once at bind so the render path never walks the descriptor.
    fold: ObservationFold,
    armed: bool,
    /// Blocks per published window; never zero once armed.
    window_blocks: u32,
    blocks: u32,
    first_sample: u64,
    end_sample: u64,
    sequence: u64,
    peak_left: f32,
    peak_right: f32,
}

/// Every declared tap of one prepared effect instance, or of one lane of one bank slot.
///
/// # The two-level zero (issue #143 D3)
///
/// Level 1 is that this type does not exist in a plan whose live-control request named no
/// observation capacity: there is no lane, no slot and no vector, and the render path is the
/// byte-identical one it always was. Level 2 is `ObservationTap::armed`: inside a capable plan, an
/// unarmed tap's effect state is never read, never folded and never stored, and the honest cost is
/// one predicted branch per driven effect per block.
#[derive(Debug)]
pub struct ObservationLane {
    taps: Box<[ObservationTap]>,
    default_window_blocks: u32,
    /// How many taps of this lane are armed, maintained by [`arm`](Self::arm) and
    /// [`disarm_all`](Self::disarm_all) (issue #163 phase 4 item 6).
    ///
    /// This exists so [`any_armed`](Self::any_armed) — which the doc has always described as "the
    /// one branch the block-top publish step takes" — is a single load rather than a walk over
    /// every tap. The publish gate runs on the render thread once per driven effect (or once per
    /// bank slot) per block, so a walk there would make the level-2 zero cost O(taps) exactly
    /// where #143 promises one predicted branch. `armed` on the tap stays the authority for what
    /// a tap *does*; this is only a redundant count of it, and `arm`/`disarm_all` are the only
    /// two writers of either.
    armed_count: u32,
}

impl ObservationLane {
    /// Bind one publisher per declared tap. Off the render thread, once, at preparation.
    ///
    /// `publishers` must be one per entry of `observations`, in declaration order.
    #[must_use]
    pub fn new(
        observations: &'static [ObservationDescriptor],
        publishers: Vec<ObservationPublisher>,
        default_window_blocks: u32,
    ) -> Option<Self> {
        if publishers.len() != observations.len() {
            return None;
        }
        let default_window_blocks = default_window_blocks.max(1);
        let taps: Vec<ObservationTap> = observations
            .iter()
            .zip(publishers)
            .map(|(descriptor, publisher)| ObservationTap {
                publisher,
                fold: descriptor.fold,
                armed: false,
                window_blocks: default_window_blocks,
                blocks: 0,
                first_sample: 0,
                end_sample: 0,
                sequence: 0,
                peak_left: 0.0,
                peak_right: 0.0,
            })
            .collect();
        Some(Self {
            taps: taps.into_boxed_slice(),
            default_window_blocks,
            // Every tap is built unarmed, so the count starts where they do.
            armed_count: 0,
        })
    }

    /// Declared taps this lane carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.taps.len()
    }

    /// Whether the lane carries no tap at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.taps.is_empty()
    }

    /// Exact engine-owned bytes this lane retains, including its slots.
    ///
    /// One tap is one accumulator row plus one shared conflating cell. Stated as a formula rather
    /// than a measured number so a row that grows has to move this line too (#143 R7).
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.taps.len()
            * (core::mem::size_of::<ObservationTap>() + observation_slot_retained_bytes())
    }

    /// Whether `tap_index` is armed right now. Off-thread introspection for the structural gates.
    #[must_use]
    pub fn is_armed(&self, tap_index: usize) -> bool {
        self.taps.get(tap_index).is_some_and(|tap| tap.armed)
    }

    /// The window length `tap_index` is armed or was last armed with, or `None` for an index this
    /// lane has no tap for. Off-thread introspection for the structural gates.
    #[must_use]
    pub fn window_blocks(&self, tap_index: usize) -> Option<u32> {
        self.taps.get(tap_index).map(|tap| tap.window_blocks)
    }

    /// The window length this lane would use for a request that names none.
    #[must_use]
    pub const fn default_window_blocks(&self) -> u32 {
        self.default_window_blocks
    }

    /// Apply one drained [`EffectControlRecord::Observe`]; `false` for an index this lane has
    /// no tap for.
    ///
    /// Arming opens a fresh window at `first_sample`, which is the first sample of the block that
    /// drained the record -- the exact `applied_at_sample` the subscription was acknowledged with.
    /// A re-arm is idempotent except that the newer `window_blocks` wins and the window restarts,
    /// so a consumer never folds two different window lengths into one reading.
    pub fn arm(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        first_sample: u64,
    ) -> bool {
        let default = self.default_window_blocks;
        let Some(tap) = usize::try_from(tap_index)
            .ok()
            .and_then(|index| self.taps.get_mut(index))
        else {
            return false;
        };
        // Maintained across the transition, not recomputed: a re-arm of an already-armed tap is
        // idempotent here exactly as it is for `armed` itself.
        match (tap.armed, armed) {
            (false, true) => self.armed_count = self.armed_count.saturating_add(1),
            (true, false) => self.armed_count = self.armed_count.saturating_sub(1),
            _ => {}
        }
        tap.armed = armed;
        tap.window_blocks = if window_blocks == 0 {
            default
        } else {
            window_blocks
        };
        tap.blocks = 0;
        tap.first_sample = first_sample;
        tap.end_sample = first_sample;
        tap.peak_left = 0.0;
        tap.peak_right = 0.0;
        true
    }

    /// Release every subscription without touching the published windows (issue #143 D7).
    ///
    /// A plan replacement drops subscriptions because the plan they addressed is gone; the app
    /// re-subscribes against the new plan. Disarming here is the same operation an explicit
    /// unsubscribe performs, so there is one code path and one meaning.
    pub fn disarm_all(&mut self) {
        self.armed_count = 0;
        for tap in self.taps.iter_mut() {
            tap.armed = false;
            tap.blocks = 0;
            tap.peak_left = 0.0;
            tap.peak_right = 0.0;
        }
    }
}

// REALTIME_POLICY_BEGIN
impl ObservationLane {
    /// Whether any tap is armed, which is the one branch the block-top publish step takes.
    ///
    /// One load and one compare, from the count [`arm`](Self::arm) and
    /// [`disarm_all`](Self::disarm_all) maintain. Wired into both publish sites by #163 phase 4
    /// item 6; before that it was `pub`, documented as the block-top gate, and called only from
    /// this crate's own tests, while the render path walked every tap instead.
    #[must_use]
    pub const fn any_armed(&self) -> bool {
        self.armed_count != 0
    }

    /// Whether tap `tap_index` should be read for this block at all.
    ///
    /// The whole of level-2 zero: one bounds check and one flag load. An unarmed tap's effect state
    /// is never touched, because this returns `false` before anything asks the effect for it.
    #[must_use]
    pub fn wants(&self, tap_index: usize) -> bool {
        match self.taps.get(tap_index) {
            Some(tap) => tap.armed,
            None => false,
        }
    }

    /// Fold one block's reading into tap `tap_index`, publishing the window if it closes here.
    ///
    /// `frames` is the block length, so `end_sample` advances by exactly what was rendered and
    /// consecutive windows tile with no gap. Called **after** `process` returns: the reading is the
    /// state at the end of the block, which is the only moment at which "resident" is true.
    pub fn accumulate(
        &mut self,
        tap_index: usize,
        sample: ObservationSample,
        first_sample: u64,
        frames: u64,
    ) {
        let Some(tap) = self.taps.get_mut(tap_index) else {
            return;
        };
        if !tap.armed {
            return;
        }
        // `first_sample` is **not** taken from the block. It was set when the tap was armed and it
        // is set again to `end_sample` when a window closes, so consecutive windows tile with no
        // gap as a property of this type rather than of whatever the caller happens to pass.
        tap.end_sample = first_sample.saturating_add(frames);
        match tap.fold {
            // `max(|x|)` is what turns an effect's own negative-for-reduction convention into the
            // non-negative magnitude a meter reads. It is one `abs` and one compare per lane per
            // block, and it is the whole reason the app's `Math.max(0, x)` is a no-op rather than
            // a silent zeroing.
            ObservationFold::PeakMagnitude => {
                let left = sample.left.abs();
                let right = sample.right.abs();
                if left > tap.peak_left {
                    tap.peak_left = left;
                }
                if right > tap.peak_right {
                    tap.peak_right = right;
                }
            }
            ObservationFold::Latest => {
                tap.peak_left = sample.left;
                tap.peak_right = sample.right;
            }
        }
        tap.blocks = tap.blocks.saturating_add(1);
        if tap.blocks < tap.window_blocks {
            return;
        }
        tap.sequence = tap.sequence.saturating_add(1);
        tap.publisher.publish(ObservationWindow {
            first_sample: tap.first_sample,
            end_sample: tap.end_sample,
            sequence: tap.sequence,
            blocks: tap.blocks,
            left: tap.peak_left,
            right: tap.peak_right,
        });
        tap.blocks = 0;
        tap.first_sample = tap.end_sample;
        tap.peak_left = 0.0;
        tap.peak_right = 0.0;
    }
}
// REALTIME_POLICY_END

/// The latency-preserving bypass shunt, applied **outside** the effect.
///
/// # Why outside
///
/// `PreparedEffectMetadata::bypass` is a *prepared* configuration: it is part of
/// [`EffectProgramKey`](crate::EffectProgramKey), it is byte 108 of the persisted state
/// envelope, and every effect's bank reads one flag for the whole bank. Moving it after
/// preparation would change a program key at render time -- which would re-cohort a bank -- and a
/// bank's lanes could not disagree about it anyway.
///
/// The shunt is the design the contract already documents on `EffectProgramKey`, minus the parts
/// that only a per-lane kernel needs:
///
/// * **The wet path always runs.** A bypassed instance still processes, so its state stays
///   continuous and un-bypassing does not click, and a bank never re-cohorts.
/// * **Latency is preserved exactly.** The dry signal is delayed by exactly
///   `PreparedEffectMetadata::latency` -- the same integer the enabled path reports and the same
///   integer `graph-compiler` derived every route timing from -- so a bypassed instance's impulse
///   lands on the sample an enabled instance's would. PDC is unchanged by construction.
/// * **Selection is whole-block, never per sample.** #140's application rule is the block
///   boundary, so a rendered block is entirely dry or entirely wet and the select is a
///   `copy_from_slice`, not an arithmetic blend. `-0.0` survives it.
///
/// # When a shunt exists (issue #1087)
///
/// A session `bypass` is per-lane shunt state, not a prepared program, for every effect whose
/// bypass `effect_compiler::lowers_session_bypass`: preparation lowers it to a prepared
/// `bypass = false` plus the lane's initial bypass, carried by an
/// [`EffectControlLane`] (a live channel, or [`EffectControlLane::without_channel`] when no live
/// controls are attached). A shunt is built for every instance, and every bank slot, that holds
/// such a lane: a live channel, or a lane bypassed at preparation. A session with neither allocates
/// none of this and renders the byte-identical path it always did.
///
/// A shunt-bypassed lane is bit-identical to the same instance prepared with `bypass = true`:
/// every launch effect that banks emits, under a prepared bypass, its input delayed by exactly its
/// declared latency from a line that starts at `+0.0`, and checks only that output at its block
/// boundary (D7), and this shunt emits the same words from a line that starts at `+0.0`, by
/// copies, so `-0.0` survives. The two differ only where that D7 check fires on the dry block
/// itself: a non-finite sample, or one at least `1e30` in magnitude. An effect never receives one
/// in a compiled plan, because the track's input stage sanitises every such sample to `+0.0` and
/// every stage after it zeroes a block that leaves that range. The delay, whose D7 check also
/// reads its state, never banks and keeps its prepared bypass
/// (`effect_compiler::NEVER_BANKED_EFFECTS`), and so does the multiband, whose D7 recovery is
/// whole-bank (`effect_compiler::PREPARED_BYPASS_EFFECTS`, issue #1100). `graph-compiler`'s
/// `bypass_shunt_identity` and
/// `bypass_cohorts` tests are the gates.
pub struct BypassShunt {
    /// Dry copy of this block's input, taken before the effect runs.
    dry_left: Box<[f32]>,
    dry_right: Box<[f32]>,
    /// `latency` words per channel; empty when the effect reports zero latency.
    line_left: Box<[f32]>,
    line_right: Box<[f32]>,
    cursor: usize,
}

impl BypassShunt {
    /// Allocates a shunt for `frames` of block and `latency` samples of declared latency.
    ///
    /// Off the render thread: this is the only allocation the live path makes, and it happens once
    /// at plan preparation.
    #[must_use]
    pub fn new(frames: usize, latency: usize) -> Self {
        Self {
            dry_left: vec![0.0; frames].into_boxed_slice(),
            dry_right: vec![0.0; frames].into_boxed_slice(),
            line_left: vec![0.0; latency].into_boxed_slice(),
            line_right: vec![0.0; latency].into_boxed_slice(),
            cursor: 0,
        }
    }

    /// The heap bytes [`Self::new`]`(frames, latency)` allocates, or `None` if they overflow.
    ///
    /// Two dry blocks of `frames` words and two latency lines of `latency` words, all `f32`. An
    /// AoSoA caller passes the interleaved sizes, `quantum * lanes` and `latency * lanes`, exactly
    /// as it constructs the shunt. This is what a resource estimate charges for a shunt before
    /// bind allocates it (issue #1100); the shunt's own struct is charged with its owner.
    #[must_use]
    pub const fn allocated_bytes(frames: usize, latency: usize) -> Option<usize> {
        let Some(words) = frames.checked_add(latency) else {
            return None;
        };
        let Some(words) = words.checked_mul(2) else {
            return None;
        };
        words.checked_mul(core::mem::size_of::<f32>())
    }

    /// The largest single allocation [`Self::new`]`(frames, latency)` makes: one channel's dry
    /// block or one channel's latency line, whichever is longer, or `None` if it overflows.
    #[must_use]
    pub const fn largest_allocation_bytes(frames: usize, latency: usize) -> Option<usize> {
        let words = if frames > latency { frames } else { latency };
        words.checked_mul(core::mem::size_of::<f32>())
    }

    /// Whether this shunt carries a latency line that has to be fed on every block.
    ///
    /// # Why a caller needs to ask (issue #163 phase 4 item 4)
    ///
    /// [`capture`](Self::capture) does two separable things: it stages the dry block into
    /// `dry_*`, and — only when `latency > 0` — exchanges that staging through the delay line.
    /// The second is stateful and must happen on every block, bypassed or not, for the reason
    /// `capture` documents. The first is **not**: `dry_*` is read only by
    /// [`apply`](Self::apply) and [`dry`](Self::dry), both of which run later in the *same* block
    /// and only for a bypassed instance or lane. Nothing carries `dry_*` across a block boundary.
    ///
    /// So at zero latency a non-bypassed block's capture is provably dead work — two whole-block
    /// `copy_from_slice`s whose result no reader can observe — and a caller that knows it is not
    /// bypassed may skip it. At nonzero latency it may not: the staging buffer *is* the line's
    /// input, so skipping the copy would starve the line and the first bypassed block would emit
    /// stale samples. This predicate is the exact boundary between those two cases, and it is
    /// fixed at preparation rather than per block.
    #[must_use]
    pub const fn feeds_line(&self) -> bool {
        !self.line_left.is_empty()
    }

    /// Capture this block's input and advance the latency line, returning nothing.
    ///
    /// Called on every block whenever [`feeds_line`](Self::feeds_line) is true, bypassed or not:
    /// the line has to stay fed so that enabling bypass mid-stream produces the correctly delayed
    /// dry signal on its very first block rather than `latency` samples of stale zeros. When
    /// `feeds_line` is false there is no line, and a caller that is not bypassed this block may
    /// skip the call entirely (#163 phase 4 item 4).
    pub fn capture(&mut self, left: &[f32], right: &[f32]) {
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        self.dry_left[..frames].copy_from_slice(&left[..frames]);
        self.dry_right[..frames].copy_from_slice(&right[..frames]);
        if self.line_left.is_empty() {
            return;
        }
        let mut offset = 0;
        let length = self.line_left.len();
        while offset < frames {
            let take = core::cmp::min(length, frames - offset);
            let cursor = self.cursor;
            let mut left_cursor = cursor;
            pdc_delay_block(
                &mut self.line_left,
                &mut left_cursor,
                &mut self.dry_left[offset..offset + take],
            );
            let mut right_cursor = cursor;
            pdc_delay_block(
                &mut self.line_right,
                &mut right_cursor,
                &mut self.dry_right[offset..offset + take],
            );
            debug_assert_eq!(left_cursor, right_cursor);
            self.cursor = left_cursor;
            offset += take;
        }
    }

    /// Replace the wet block with the latency-matched dry block.
    pub fn apply(&self, left: &mut [f32], right: &mut [f32]) {
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        left[..frames].copy_from_slice(&self.dry_left[..frames]);
        right[..frames].copy_from_slice(&self.dry_right[..frames]);
    }

    /// The delayed dry block, for a lane-selective caller (the AoSoA racks).
    #[must_use]
    pub fn dry(&self) -> (&[f32], &[f32]) {
        (&self.dry_left, &self.dry_right)
    }
}
