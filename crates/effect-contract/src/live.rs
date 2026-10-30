//! The live-control seam: committed live values in, prepared automation spans out.
//!
//! # Why this lives in the contract crate
//!
//! Issue #137 shipped the browser's first live-command ABI (deleted since: the value commands by
//! #1565, the observation records by #1589c; every live value is now a `SET_CONTROL` edit) and
//! discovered that no live parameter path reaches a running plan: `graph::runtime` handed every
//! effect a hardcoded empty automation slice and `rack::EffectBankStage` handed every bank an empty
//! one with zero offsets. #140 closes that gap by feeding the admitted live values in as
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
//! - one **bypass** cell (two words: the bypass and the crossfade ramp that carries it, #1341);
//! - one **link** cell for an effect whose descriptor has `lane_link` (two words: the detector link
//!   mode's code and the glide that carries it, #1371 D1). The stage that owns the processor hands
//!   an applied word to `retarget_link`;
//! - one **parameter** cell per `(parameter_index, channel)` of every `Block`-rate parameter: a
//!   `Shared` parameter has one `Both` cell, a `PerLane` one a `Left` and a `Right` cell (one
//!   word, the value's bits). A lane whose effect owns prepared targets (the parametric EQ) has
//!   none: its parameters ride its targets, and a bare parameter write is refused;
//! - one **target** cell per `(target slot, channel)` of a prepared-target owner
//!   ([`PREPARED_EFFECT_TARGET_WORDS`] words; a `Both` target writes both channel cells).
//!
//! The cell's index is its dirty bit and its place in the canonical drain order (D3): bypass, then
//! the link cell (#1371 D2), then the parameter cells in `(parameter_index, channel)` order, then the target cells in `(slot,
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
//! a single sample is touched, and emits [`PreparedAutomationSpan`]s whose `sample` is that block's
//! `first_sample`. Every launch effect accepts precisely that shape
//! (a span at `first_sample`) and rejects anything else into
//! `ProcessReport::invalid_spans`, so the block boundary is proven by the effect contract rather
//! than asserted by the host.
//!
//! # The window holds every live cell (D4)
//!
//! A block stages one span for every parameter cell it reads, in the block that reads it: there is
//! no deferral and no drop. So the staging window is at least [`EffectControlLane::parameter_cells`]
//! spans, and every effect accepts that many: preparation raises an instance's
//! `automation_capacity` to at least [`EffectLaneShape::live_parameter_cells`] of its descriptor,
//! the window is the lane's cells, never past that capacity (the pairing rule of issue #1012, #1621),
//! and a window shorter than the lane needs is refused at bind ([`EffectControlLane::check_window`]).
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
use lane::Lane;
use lane::kernels::{
    FRAME_INDEX_OFFSETS, INDEXED_RAMP_LENGTH_MAXIMUM, IndexedRamp, pdc_delay_block,
};
use std::sync::Arc;

use crate::{
    AutomationRate, ChannelSymmetryWitness, EffectDescriptor, LINK_GLIDE_MAX_SAMPLES, LinkMode,
    LinkModeSet, ObservationDescriptor, ObservationFold, ObservationSample,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, ParameterChannelPolicy, PreparedAutomationSpan,
    PreparedEffectTarget, PreparedNativeEffectBank, StatePayloadInput, StatePayloadOutput,
    StatePayloadSizes,
};

/// The native effects whose link mode is a live update (decision 15 E2, #1371): each holds its
/// mode per lane (`lane_link`) and glides a lane's detector to another mode
/// ([`PreparedNativeEffectBank::retarget_link`], #1370 and #1969). Only their lanes hold a link
/// cell ([`EffectLaneShape::from_descriptor`]), and only their insert and console-entry link rows
/// are `Live` (`controls`), so the list is the row's class and the cell's presence, never a
/// runtime answer: one on it whose processor refused the glide would be an acknowledged edit
/// render never hears (decision 15 E4), which the controls test of this list refuses. The
/// multiband compressor joined it with its glide (#1367b).
pub const LIVE_LINK_EFFECTS: [&str; 7] = [
    "miso.compressor",
    "miso.gate-expander",
    "miso.transient-shaper",
    "miso.true-peak-limiter",
    "miso.de-esser",
    "miso.dynamic-eq",
    "miso.multiband-compressor",
];

/// One admitted live control event for one prepared effect instance.
///
/// It is `Copy` and fixed-size. Addressing is *not* carried: a lane belongs to exactly one effect
/// instance (or, in a bank, to exactly one lane of one slot), so the record only says what to
/// change. [`EffectCellWriter::write`] puts a [`Parameter`](Self::Parameter),
/// [`PreparedTarget`](Self::PreparedTarget), [`Bypass`](Self::Bypass) or [`Link`](Self::Link)
/// value into its cell, and
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
        /// The tag of the edit this record carries out, which render publishes in the tap's slot
        /// with its arm state (#1585 D1, D3); `0` means no tag.
        arm_tag: u64,
    },
    /// Set this instance's live bypass, with the crossfade ramp that carries it.
    ///
    /// Bypass is not a parameter and has no span: it is applied by the rack's latency-preserving
    /// shunt (see [`BypassShunt`]), never by the effect, because
    /// [`PreparedEffectMetadata::bypass`](crate::PreparedEffectMetadata) is baked into the
    /// program key and a bank's lanes must be able to disagree about it.
    ///
    /// The record is the whole bypass cell, two words: the bypass and its ramp
    /// ([`BypassSwitch`]), written and drained as one unit. A host fills the ramp from the
    /// session's mute ramp (`LiveRamps::mute_samples`); a bare step is
    /// [`BypassSwitch::step`]. A ramp longer than [`INDEXED_RAMP_LENGTH_MAXIMUM`] is refused as
    /// [`EffectWriteRefusal::Ramp`] by [`EffectCellWriter::check`] and
    /// [`EffectCellWriter::write`], with nothing written.
    Bypass(BypassSwitch),
    /// Glide this instance's detector link to another mode (#1371).
    ///
    /// The record is the whole link cell, two words: the mode and the glide's length
    /// ([`LinkSwitch`]), written and drained as one unit. The stage that owns the processor hands
    /// an applied word to `retarget_link` at the block boundary. A lane whose effect has no
    /// `lane_link` holds no link cell and refuses it as [`EffectWriteRefusal::Link`], as it does a
    /// mode the effect does not support; a glide longer than [`LINK_GLIDE_MAX_SAMPLES`] is
    /// refused as [`EffectWriteRefusal::Ramp`]. Nothing is written on a refusal.
    Link(LinkSwitch),
}

/// A detector link mode and the glide that carries it: the two words of the link cell, written and
/// drained as one unit (#1371 D1).
///
/// The lane's detector glides from the weights in force to `mode`'s rest weights over
/// `ramp_samples` frames (`retarget_link`, #1370); `0` is a step. A host fills the length from the
/// edit's own ramp, else the session's `fader_ms` (`LiveRamps::link_samples`, #1054 D3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LinkSwitch {
    /// The mode the lane's detector reaches when the glide ends.
    pub mode: LinkMode,
    /// The glide's length in frames, at most [`LINK_GLIDE_MAX_SAMPLES`]; `0` is a step.
    pub ramp_samples: u32,
}

impl LinkSwitch {
    /// The mode whose code (`LinkMode as u32`) is `code`, if any.
    fn mode_of(code: u32) -> Option<LinkMode> {
        [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
            .into_iter()
            .find(|mode| *mode as u32 == code)
    }
}

/// A live bypass and the crossfade ramp that carries it: the two words of the bypass cell, written
/// and drained as one unit (decision 15, D15-2; #1341 D1).
///
/// The render lane crossfades linearly between the effect's wet output and its latency-matched dry
/// signal over `ramp_samples` frames, on the [`IndexedRamp`] law; `0` is a step, which renders the
/// whole-block select of an un-ramped bypass. A host fills it from the session's mute ramp
/// (`LiveRamps::mute_samples`); the bypass is the one live row with no per-edit length.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BypassSwitch {
    /// The bypass the lane reaches when the ramp ends.
    pub bypassed: bool,
    /// The ramp's length in frames, at most [`INDEXED_RAMP_LENGTH_MAXIMUM`]; `0` is a step.
    pub ramp_samples: u32,
}

impl BypassSwitch {
    /// A bypass change that takes effect in one step.
    #[must_use]
    pub const fn step(bypassed: bool) -> Self {
        Self {
            bypassed,
            ramp_samples: 0,
        }
    }

    /// Whether a cell may hold this switch: its ramp fits the indexed ramp law.
    ///
    /// # Errors
    ///
    /// [`EffectWriteRefusal::Ramp`] when `ramp_samples` exceeds [`INDEXED_RAMP_LENGTH_MAXIMUM`].
    pub const fn check(&self) -> Result<(), EffectWriteRefusal> {
        if self.ramp_samples > INDEXED_RAMP_LENGTH_MAXIMUM {
            Err(EffectWriteRefusal::Ramp)
        } else {
            Ok(())
        }
    }
}

/// One drain unit of the parameter cells: a `Shared` parameter's one `Both` cell, or a `PerLane`
/// parameter's `Left` cell with its `Right` twin at `first + 1`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParameterGroup {
    /// The descriptor's parameter index, strictly increasing along the shape's groups.
    parameter_index: u32,
    /// The group's first parameter cell.
    first: u32,
    pair: bool,
}

/// The cells one lane holds: which parameters are live, how many target slots there are, and the
/// cell index of each. Built once, at preparation, from the effect's descriptor.
///
/// Cell indices run bypass (`0`), the parameter cells (`1..=P`), then the target cells (`1 + P ..`,
/// slot-major, `Left` before `Right`).
///
/// The shape is sized by the effect's live parameters alone (#1611 D2): one 12-byte group per live
/// parameter, found by binary search on the parameter index, with no table as long as the last live
/// index and no record per cell. The lookup runs on the control thread; render walks the groups.
#[derive(Debug, Eq, PartialEq)]
pub struct EffectLaneShape {
    groups: Box<[ParameterGroup]>,
    /// The modes the lane's link cell may hold, or `None` for a lane without one: its effect has
    /// no `lane_link` (#1371 D1).
    link: Option<LinkModeSet>,
    /// The parameter cells of all groups: one per `Shared` group, two per `PerLane` one.
    parameter_cells: u32,
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
        let mut groups = Vec::new();
        let mut previous: Option<u32> = None;
        let mut parameter_cells = 0_u32;
        for (parameter_index, policy) in live {
            assert!(
                previous.is_none_or(|previous| parameter_index > previous),
                "live parameters are strictly increasing in the parameter index"
            );
            previous = Some(parameter_index);
            let pair = policy == ParameterChannelPolicy::PerLane;
            groups.push(ParameterGroup {
                parameter_index,
                first: parameter_cells,
                pair,
            });
            parameter_cells = parameter_cells
                .checked_add(if pair { 2 } else { 1 })
                .expect("a lane's parameter cells fit u32");
        }
        Self {
            groups: groups.into_boxed_slice(),
            link: None,
            parameter_cells,
            target_slots,
        }
    }

    /// This shape with a link cell that holds the modes in `modes` (#1371 D1).
    #[must_use]
    pub fn with_link(self, modes: LinkModeSet) -> Self {
        Self {
            link: Some(modes),
            ..self
        }
    }

    /// Whether the lane holds a link cell.
    #[must_use]
    pub const fn has_link(&self) -> bool {
        self.link.is_some()
    }

    /// The shape of `descriptor`'s lane: one cell group per `Block`-rate parameter when the effect
    /// owns no prepared targets (`target_slots == 0`), and none when it does, because a
    /// prepared-target owner's parameters ride its targets (D1); and a link cell, holding its
    /// supported modes, when the effect is on [`LIVE_LINK_EFFECTS`] (#1371 D1), so its processor
    /// takes every switch the writer stores. A `lane_link` effect off the list holds none, and its
    /// writer refuses a switch.
    #[must_use]
    pub fn from_descriptor(descriptor: &EffectDescriptor, target_slots: usize) -> Self {
        let shape = if target_slots != 0 {
            Self::from_live_parameters(core::iter::empty(), target_slots)
        } else {
            Self::from_live_parameters(live_parameters(descriptor), 0)
        };
        if descriptor.lane_link && LIVE_LINK_EFFECTS.contains(&descriptor.id.as_str()) {
            shape.with_link(descriptor.supported_link_modes)
        } else {
            shape
        }
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
        self.parameter_cells as usize
    }

    /// The target slots this lane holds cells for (`0` for an effect without prepared targets).
    #[must_use]
    pub const fn target_slots(&self) -> usize {
        self.target_slots
    }

    /// Every cell of the lane: bypass, link, parameters and targets. Also the dirty bit count.
    fn cell_count(&self) -> usize {
        self.parameter_base() + self.parameter_cells() + 2 * self.target_slots
    }

    /// The dirty bit of the link cell, which follows the bypass cell's (#1371 D2).
    const LINK_BIT: usize = 1;

    /// The dirty bit of the first parameter cell: after the bypass cell and the link cell.
    fn parameter_base(&self) -> usize {
        1 + usize::from(self.link.is_some())
    }

    /// The dirty bit (and cell index) of target cell `index`.
    fn target_bit(&self, index: usize) -> usize {
        self.parameter_base() + self.parameter_cells() + index
    }

    /// The parameter cell of `(parameter_index, channel)`, or why there is none.
    fn locate(
        &self,
        parameter_index: u32,
        channel: ParameterChannel,
    ) -> Result<usize, EffectWriteRefusal> {
        let group = self
            .groups
            .binary_search_by_key(&parameter_index, |group| group.parameter_index)
            .map(|position| self.groups[position])
            .map_err(|_| EffectWriteRefusal::NotLive)?;
        let first = group.first as usize;
        match (group.pair, channel) {
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
    /// A bypass switch's ramp is longer than [`INDEXED_RAMP_LENGTH_MAXIMUM`], or a link switch's
    /// glide is longer than [`LINK_GLIDE_MAX_SAMPLES`].
    Ramp,
    /// The lane holds no link cell (its effect has no `lane_link`), or the effect does not
    /// support the switch's mode.
    Link,
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
    bypass: LatestCell<2>,
    /// Written and read only when the shape has a link cell.
    link: LatestCell<2>,
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
            link: LatestCell::new(),
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
    /// The bypass cell and its two words: the bypass and the ramp.
    Bypass(BypassSwitch),
    /// The link cell and its two words: the mode's code and the glide's length.
    Link(LinkSwitch),
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
    bypass: CellWriter<2>,
    link: CellWriter<2>,
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
            EffectControlRecord::Bypass(switch) => {
                switch.check()?;
                Ok(Address::Bypass(switch))
            }
            EffectControlRecord::Link(switch) => {
                if !self
                    .cells
                    .shape
                    .link
                    .is_some_and(|modes| modes.contains(switch.mode))
                {
                    return Err(EffectWriteRefusal::Link);
                }
                if switch.ramp_samples > LINK_GLIDE_MAX_SAMPLES {
                    return Err(EffectWriteRefusal::Ramp);
                }
                Ok(Address::Link(switch))
            }
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
            Address::Bypass(switch) => {
                self.bypass.write(
                    &cells.bypass,
                    &cells.dirty[0],
                    0,
                    stamp,
                    [u32::from(switch.bypassed), switch.ramp_samples],
                );
            }
            Address::Link(switch) => {
                let bit = EffectLaneShape::LINK_BIT;
                self.link.write(
                    &cells.link,
                    &cells.dirty[bit / 64],
                    (bit % 64) as u32,
                    stamp,
                    [switch.mode as u32, switch.ramp_samples],
                );
            }
            Address::Parameter(index, bits) => {
                let bit = cells.shape.parameter_base() + index;
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
    ///
    /// `arm_tag` is the tag the control plane issued for the edit (#1585 D4), `0` for none: render
    /// publishes it with the tap's arm state when it drains the record.
    pub fn try_push_observe(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        arm_tag: u64,
    ) -> Result<(), ObservePushError> {
        let queue = self.observe.as_mut().ok_or(ObservePushError::Unbound)?;
        queue
            .try_push(EffectControlRecord::Observe {
                tap_index,
                armed,
                window_blocks,
                arm_tag,
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

    /// Test-only: how many writes the bypass cell has taken, then the link cell (on a lane that
    /// holds one), then each parameter cell, then each target cell, in cell order. Valid while no write is in flight, which a quiescent test
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
                    .shape
                    .has_link()
                    .then(|| count(reader.peek_unread(&cells.link))),
            )
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
    /// The link mode the instance is prepared with, for a lane whose shape holds a link cell
    /// (#1371 D6): the mode the lane's detector rests at before any link write applies. `None`
    /// when the lane has no link cell, or the builder does not know the mode; such a lane's
    /// plan-swap carry seats no mode before a restore.
    pub link: Option<LinkMode>,
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
        link,
    } = spec;
    let link = link.filter(|_| shape.has_link());
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
        link: CellWriter::new(),
        parameters: (0..parameter_cells).map(|_| CellWriter::new()).collect(),
        targets: (0..target_cells).map(|_| CellWriter::new()).collect(),
        observe: producer,
    };
    let lane = EffectControlLane {
        cells: Some(cells),
        bypass_reader: CellReader::new(),
        link_reader: CellReader::new(),
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
        mix: settled_mix(bypass),
        mix_position: 0,
        hold: 0,
        rest_pending: false,
        symmetry,
        carried_bypass: CarriedCell::EMPTY,
        carried_link: CarriedCell::EMPTY,
        carried_parameters: Box::default(),
        carried_targets: Box::default(),
        carried_pending: false,
        link,
        link_refusals: 0,
    };
    Some((writer, lane))
}

/// What a live-control lane presents to the graph's resource estimate (issue #2199): the heap it
/// retains ([`EffectControlLane::retained_resources`]), its live parameter cells, and whether it
/// has a channel, is bypassed and holds an observation queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectLaneLayout {
    pub resources: EffectLaneResources,
    pub parameter_cells: usize,
    pub has_channel: bool,
    pub bypassed: bool,
    pub has_observe_queue: bool,
    /// What [`EffectControlLane::install_carry_slots`] would allocate on this lane
    /// ([`EffectControlLane::carry_slot_bytes`]).
    pub carry_slot_bytes: u64,
}

/// The lane and writer [`effect_control_lane`] builds from a spec, laid out before either exists
/// (issue #2199): the lane's [`EffectLaneLayout`] and the writer's
/// [`EffectCellWriter::retained_resources`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectLaneProjection {
    pub lane: EffectLaneLayout,
    pub writer: EffectLaneResources,
}

impl EffectLaneSpec {
    /// What [`effect_control_lane`] would build from this spec, without building it (issue #2199):
    /// the allocations it makes, counted as the built lane's and writer's `retained_resources`
    /// count them. `None` exactly when `effect_control_lane` would return `None` (an observation
    /// queue that cannot be built). Allocation-free.
    #[must_use]
    pub fn project(&self) -> Option<EffectLaneProjection> {
        let shape = &self.shape;
        let parameter_cells = shape.parameter_cells();
        let target_cells = 2 * shape.target_slots();
        let dirty_words = shape.cell_count().div_ceil(64);
        let mut lane = EffectLaneResources::default();
        lane.add(Layout::new::<SharedAllocation<EffectCells>>().size());
        lane.add_array::<ParameterGroup>(shape.groups.len());
        lane.add_array::<LatestCell<1>>(parameter_cells);
        lane.add_array::<LatestCell<PREPARED_EFFECT_TARGET_WORDS>>(target_cells);
        lane.add_array::<DirtyWord>(dirty_words);
        lane.add_array::<CellReader>(parameter_cells);
        lane.add_array::<CellReader>(target_cells);
        lane.add_array::<DirtyReader>(dirty_words);
        lane.add_array::<u64>(dirty_words);
        lane.add_array::<PreparedEffectTarget>(target_cells);
        if let Some(depth) = NonZeroUsize::new(self.observation_depth) {
            let payload = bounded_spsc_retained_payload::<EffectControlRecord>(depth).ok()?;
            lane.add(payload.ring_header_bytes);
            lane.add(payload.slot_payload_bytes);
        }
        let mut writer = EffectLaneResources::default();
        writer.add_array::<CellWriter<1>>(parameter_cells);
        writer.add_array::<CellWriter<PREPARED_EFFECT_TARGET_WORDS>>(target_cells);
        let carry_slot_bytes = parameter_cells
            .saturating_mul(core::mem::size_of::<CarriedCell<1>>())
            .saturating_add(target_cells.saturating_mul(core::mem::size_of::<
                CarriedCell<PREPARED_EFFECT_TARGET_WORDS>,
            >()));
        Some(EffectLaneProjection {
            lane: EffectLaneLayout {
                resources: lane,
                parameter_cells,
                has_channel: true,
                bypassed: self.bypass,
                has_observe_queue: self.observation_depth != 0,
                carry_slot_bytes: u64::try_from(carry_slot_bytes).unwrap_or(u64::MAX),
            },
            writer,
        })
    }
}

/// One prepared live-control lane for one effect instance, or one lane of one bank slot: render's
/// end of the cells and the observation queue.
///
/// The shared cells are an `Arc`, so either end may drop first and the last owner frees them; the
/// C ABI drops a reclaimed plan before its writers, on the control thread.
///
/// # A lane without cells (issue #1087)
///
/// A session `bypass` is per-lane state, not a prepared program, for every effect (issues #1087,
/// #1339, #1340): preparation lowers it to a prepared `bypass = false` plus this lane's initial bypass, so a bypassed track
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
    link_reader: CellReader,
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
    /// Live bypass target, retained across blocks so a rendered block always knows it. While a
    /// crossfade runs, the lane is moving toward this state.
    bypass: bool,
    /// The wet mix `m` (`1` wet, `0` dry) as an indexed ramp that uses word 0: settled at the
    /// bypass target, or running from the mix the last record found toward it (#1341 D2).
    mix: IndexedRamp,
    /// Ramp frames rendered since the ramp started, saturated at the ramp's length. The ramp is
    /// settled exactly when it has reached the length.
    mix_position: u32,
    /// Frames the lane still holds the dry signal before its crossfade runs (issue #2129 D3, as
    /// amended): set by [`Self::begin_hold`] when the lane leaves settled bypass, consumed by
    /// [`Self::advance_bypass_mix`] before the ramp advances, and `0` otherwise. A lane with a
    /// hold reads dry and counts as ramping. Per-lane state: it moves with the lane in a lane
    /// swap and a plan swap's carry.
    hold: u32,
    /// Set by the [`Self::stage`] that applies a switch out of **settled** bypass (issue #2129 D2,
    /// as amended): the lane's effect state is stale and the caller resets the lane, keeping
    /// parameters, and starts its hold before the block renders ([`Self::take_rest`]). A switch
    /// that reverses a crossfade toward bypass before it settled does not set it.
    rest_pending: bool,
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
    /// The predecessor's unread bypass write this lane took at a plan swap (#1280 D2).
    carried_bypass: CarriedCell<2>,
    /// The predecessor's unread link write this lane took at a plan swap (#1371 D6).
    carried_link: CarriedCell<2>,
    /// The predecessor's unread parameter writes, one per parameter cell. Empty until
    /// [`EffectControlLane::install_carry_slots`] runs: a lane no successor carries holds none
    /// (#1633 D1).
    carried_parameters: Box<[CarriedCell<1>]>,
    /// The predecessor's unread target writes, one per target cell. Installed with
    /// [`Self::carried_parameters`].
    carried_targets: Box<[CarriedCell<PREPARED_EFFECT_TARGET_WORDS>]>,
    /// Whether any carried slot holds a write: the next [`Self::stage`] resolves them before it
    /// decides it has nothing to read.
    carried_pending: bool,
    /// The link mode the processor's lane is heading for or rests at (#1371 D6): the prepared mode
    /// until a link write applies, then the mode of the last switch `retarget_link` took
    /// ([`Self::settle_link`]). It moves with the lane in a lane swap and a plan swap, so a
    /// carry knows which mode the payload it restores was taken under. `None` for a lane without
    /// a link cell, or one built without its prepared mode.
    link: Option<LinkMode>,
    /// Link switches the processor refused (`retarget_link` returned `false`). The writer refuses
    /// every switch the effect would, so this stays `0`.
    link_refusals: u64,
}

/// A write the retiring plan's cell held unread at a plan swap, which its successor's lane applies
/// at its first rendered block (#1280 D2): the newest unread write's words, and how many unread
/// writes the predecessor's cell held (`0` for none).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CarriedCell<const N: usize> {
    words: [u32; N],
    unread: u64,
}

impl<const N: usize> CarriedCell<N> {
    const EMPTY: Self = Self {
        words: [0; N],
        unread: 0,
    };

    /// The slot a successor lane takes for one predecessor cell: the predecessor's newest unread
    /// write, with the writes it still holds from an earlier swap that had not rendered yet.
    fn of(reader: &CellReader, cell: &LatestCell<N>, earlier: Self) -> Self {
        match reader.peek_unread(cell) {
            Some((words, sequence)) => Self {
                words,
                unread: sequence
                    .saturating_sub(reader.last_applied())
                    .saturating_add(earlier.unread),
            },
            None => earlier,
        }
    }
}

/// The mix of a lane settled at bypass state `bypass`: `0` (dry) when bypassed, else `1` (wet).
const fn settled_mix(bypass: bool) -> IndexedRamp {
    IndexedRamp::settled([if bypass { 0.0 } else { 1.0 }, 0.0, 0.0, 0.0])
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
            link_reader: CellReader::new(),
            parameter_readers: Box::default(),
            target_readers: Box::default(),
            dirty: Box::default(),
            pending: Box::default(),
            superseded: None,
            targets: Box::default(),
            staged_targets: 0,
            observe: None,
            bypass,
            mix: settled_mix(bypass),
            mix_position: 0,
            hold: 0,
            rest_pending: false,
            symmetry,
            carried_bypass: CarriedCell::EMPTY,
            carried_link: CarriedCell::EMPTY,
            carried_parameters: Box::default(),
            carried_targets: Box::default(),
            carried_pending: false,
            link: None,
            link_refusals: 0,
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
            resources.add_array::<ParameterGroup>(cells.shape.groups.len());
            resources.add_array::<LatestCell<1>>(cells.parameters.len());
            resources.add_array::<LatestCell<PREPARED_EFFECT_TARGET_WORDS>>(cells.targets.len());
            resources.add_array::<DirtyWord>(cells.dirty.len());
        }
        resources.add_array::<CellReader>(self.parameter_readers.len());
        resources.add_array::<CellReader>(self.target_readers.len());
        resources.add_array::<DirtyReader>(self.dirty.len());
        resources.add_array::<u64>(self.pending.len());
        resources.add_array::<PreparedEffectTarget>(self.targets.len());
        resources.add_array::<CarriedCell<1>>(self.carried_parameters.len());
        resources
            .add_array::<CarriedCell<PREPARED_EFFECT_TARGET_WORDS>>(self.carried_targets.len());
        if let Some(queue) = self.observe.as_ref()
            && let Some(capacity) = NonZeroUsize::new(queue.capacity())
            && let Ok(payload) = bounded_spsc_retained_payload::<EffectControlRecord>(capacity)
        {
            resources.add(payload.ring_header_bytes);
            resources.add(payload.slot_payload_bytes);
        }
        resources
    }

    /// The heap this lane retains, exclusive of the lane itself (#1074): exactly
    /// [`Self::retained_resources`]' total, so the shared cells' `Arc` allocation is charged in
    /// full whoever else holds it. The `LiveValuesSuperseded` counter is not charged: it is one
    /// host-owned allocation cloned into every lane, made before the plan and outliving it, so the
    /// plan never frees it. Control plane only.
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        self.retained_resources().total_bytes
    }

    /// This lane as the graph's resource estimate reads it (issue #2199): what
    /// [`EffectLaneSpec::project`] laid out for it, while no carry slot is installed.
    #[must_use]
    pub fn layout(&self) -> EffectLaneLayout {
        EffectLaneLayout {
            resources: self.retained_resources(),
            parameter_cells: self.parameter_cells(),
            has_channel: self.has_channel(),
            bypassed: self.bypassed(),
            has_observe_queue: self.has_observe_queue(),
            carry_slot_bytes: self.carry_slot_bytes(),
        }
    }

    /// The bytes [`Self::install_carry_slots`] allocates for this lane, and the lane holds once it
    /// has: none for a lane without cells. The one place the slot charge is spelled, so
    /// preparation's admission and the retained-bytes row cannot disagree.
    #[must_use]
    pub fn carry_slot_bytes(&self) -> u64 {
        if self.cells.is_none() {
            return 0;
        }
        let parameters = self
            .parameter_readers
            .len()
            .saturating_mul(core::mem::size_of::<CarriedCell<1>>());
        let targets = self
            .target_readers
            .len()
            .saturating_mul(core::mem::size_of::<
                CarriedCell<PREPARED_EFFECT_TARGET_WORDS>,
            >());
        u64::try_from(parameters.saturating_add(targets)).unwrap_or(u64::MAX)
    }

    /// Allocate the carried slots a plan swap fills, one per parameter and target cell (#1633
    /// D1). Off the render thread: preparation calls it for each successor lane a swap carries
    /// into, and for no other, so a plan that is never a successor holds none. A lane without
    /// cells has nothing to carry, and a lane whose slots are installed keeps them.
    pub fn install_carry_slots(&mut self) {
        if self.cells.is_none() || self.carry_slots_installed() {
            return;
        }
        self.carried_parameters =
            vec![CarriedCell::EMPTY; self.parameter_readers.len()].into_boxed_slice();
        self.carried_targets =
            vec![CarriedCell::EMPTY; self.target_readers.len()].into_boxed_slice();
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

    /// Whether this instance's bypass **target** is set as of the last [`stage`](Self::stage): the
    /// state its crossfade is heading for or has reached. Use [`Self::needs_dry`] to ask whether a
    /// block must capture the dry signal.
    #[must_use]
    pub const fn bypassed(&self) -> bool {
        self.bypass
    }

    /// Whether a bypass crossfade is running: the mix has not reached its target, or the lane
    /// still holds dry before its crossfade starts ([`Self::begin_hold`]).
    #[must_use]
    pub const fn bypass_ramping(&self) -> bool {
        self.mix_position < self.mix.length || self.hold != 0
    }

    /// Whether this lane is settled at bypass: its target is bypassed and no crossfade runs, so it
    /// emits exactly the latency-matched dry signal (issue #2129 D1).
    #[must_use]
    pub const fn settled_bypassed(&self) -> bool {
        self.bypass && !self.bypass_ramping()
    }

    /// Whether the last [`Self::stage`] took this lane out of settled bypass, clearing the flag
    /// (issue #2129 D2, as amended). When it returns `true` the caller resets the lane's effect
    /// state, keeping parameters (a bank's `reset_lane`, an instance's `reset`), and calls
    /// [`Self::begin_hold`] with the effect's warm-up, before anything of the block reaches the
    /// effect. Every un-bypass from settled bypass resumes from rest, so a lane's bits never
    /// depend on whether its bank mates kept its wet path running.
    pub fn take_rest(&mut self) -> bool {
        core::mem::take(&mut self.rest_pending)
    }

    /// Hold the dry signal for `frames` frames before this lane's crossfade runs (issue #2129
    /// D3, as amended): the lane's state was reset as it left settled bypass, and its output
    /// reads `frames` of post-reset history before it is the output of a lane that never paused. The ramp starts when the hold ends, on the [`IndexedRamp`] law from its first
    /// frame, so the frame `frames` into the hold takes `k = 1`. Clamped to
    /// [`INDEXED_RAMP_LENGTH_MAXIMUM`], which keeps every crossfade index exact in `f32`. A lane
    /// that is bypassed, or a `frames` of `0`, takes no hold. Render thread, at a block boundary.
    pub fn begin_hold(&mut self, frames: u32) {
        if !self.bypass {
            self.hold = frames.min(INDEXED_RAMP_LENGTH_MAXIMUM);
        }
    }

    /// Whether this block reads the dry signal: the instance is bypassed, or its crossfade is
    /// running (#1341 D5). The caller captures the dry block before the effect runs exactly when
    /// this is true or the shunt feeds a latency line.
    #[must_use]
    pub const fn needs_dry(&self) -> bool {
        self.bypass || self.bypass_ramping()
    }

    /// The wet mix `m` the next rendered frame continues from: `1` for a lane settled wet, `0`
    /// settled dry, and the last rendered frame's `m(k)` while a ramp runs.
    #[must_use]
    pub fn bypass_mix(&self) -> f32 {
        self.mix.coefficients_at(self.mix_position)[0]
    }

    /// This lane's crossfade for the next rendered block, as one lane of a bank's
    /// [`BankCrossfade`] (#1341 D4).
    #[must_use]
    pub fn bypass_crossfade_lane(&self) -> BankCrossfadeLane {
        let ramping = self.mix_position < self.mix.length;
        BankCrossfadeLane {
            step: self.mix.step[0],
            start: self.mix.start[0],
            // Exact: a position is at most the ramp's length and a hold at most
            // `INDEXED_RAMP_LENGTH_MAXIMUM`, both at most `2^22`. A held lane's position is
            // negative, so its first `hold` frames take an index below `1` and read dry (#2129).
            position: self.mix_position as f32 - self.hold as f32,
            // A settled lane has length `0`, so no frame index (all at least `1`) is below it.
            length: if ramping { self.mix.length as f32 } else { 0.0 },
            settled_dry: self.bypass,
        }
    }

    /// Mix this block: replace the wet block in `left`/`right` with its bypass crossfade against
    /// `shunt`'s dry block, then advance the crossfade by the block (#1341 D3).
    ///
    /// * Settled wet: nothing, the wet words stay untouched.
    /// * Settled dry: the dry words are copied, as an un-ramped bypass always did.
    /// * Ramping: `out = dry + (wet - dry) * m(k)`, `m(k)` the [`IndexedRamp`] law at ramp frame
    ///   `k = position + f + 1`, a separate subtract, multiply and add (no fused multiply-add),
    ///   vectorised over frames at the caller's width. One `m` serves both channels. The frames of
    ///   the block past the ramp's end are settled and take the settled rule.
    ///
    /// `shunt` must have captured this block's dry signal ([`Self::needs_dry`]).
    pub fn apply_bypass<L: Lane>(
        &mut self,
        shunt: &BypassShunt,
        left: &mut [f32],
        right: &mut [f32],
    ) {
        if self.hold != 0 {
            // #2129 D3: the held frames are dry, then the ramp runs from its first frame.
            shunt.crossfade_held::<L>(
                left,
                right,
                &self.mix,
                self.mix_position,
                self.hold,
                self.bypass,
            );
            let frames = left.len().min(right.len());
            self.advance_bypass_mix(u32::try_from(frames).unwrap_or(u32::MAX));
        } else if self.bypass_ramping() {
            shunt.crossfade::<L>(left, right, &self.mix, self.mix_position, self.bypass);
            let frames = left.len().min(right.len());
            self.advance_bypass_mix(u32::try_from(frames).unwrap_or(u32::MAX));
        } else if self.bypass {
            shunt.apply(left, right);
        }
    }

    /// Advance the crossfade by `frames` rendered frames, and, when that settles it at wet, restore
    /// the witness' `UNBYPASSED` term: a ramping lane counts as bypassed until then (#1341 D6).
    ///
    /// [`Self::apply_bypass`] calls it; a bank mixes through [`BypassShunt::crossfade_bank`] and
    /// then calls it once per block, so each lane's state stays in step with the timeline.
    pub fn advance_bypass_mix(&mut self, frames: u32) {
        if !self.bypass_ramping() {
            return;
        }
        // A hold is spent before the ramp moves (#2129 D3): the frames past its end are the
        // ramp's first.
        let held = self.hold.min(frames);
        self.hold -= held;
        self.mix_position = self
            .mix_position
            .saturating_add(frames - held)
            .min(self.mix.length);
        if !self.bypass_ramping() && !self.bypass {
            self.symmetry
                .admit(&EffectControlRecord::Bypass(BypassSwitch::step(false)));
        }
    }

    /// Whether this lane and `other` have no cells, or cells of one shape (the same parameter
    /// groups and target slots), so every cell of one has a cell of the other. Allocation-free and
    /// bounded by the group count.
    #[must_use]
    pub fn same_cell_shape(&self, other: &Self) -> bool {
        match (self.cells.as_deref(), other.cells.as_deref()) {
            (None, None) => true,
            (Some(own), Some(before)) => own.shape == before.shape,
            _ => false,
        }
    }

    /// Whether [`Self::carry_from`] would take `predecessor`: both lanes have no cells, or both
    /// have cells of one shape, so every cell of the predecessor has a carried slot here, and
    /// this lane's slots are installed ([`Self::install_carry_slots`], #1633 D2). The
    /// predecessor's own slots may be missing: it reads them as empty. Allocation-free and
    /// bounded by the group count.
    #[must_use]
    pub fn can_carry_from(&self, predecessor: &Self) -> bool {
        self.same_cell_shape(predecessor) && self.carry_slots_installed()
    }

    /// Whether this lane holds the carried slots a successor writes into: always true for a lane
    /// without cells, which has nothing to carry.
    #[must_use]
    pub fn carry_slots_installed(&self) -> bool {
        self.carried_parameters.len() == self.parameter_readers.len()
            && self.carried_targets.len() == self.target_readers.len()
    }

    /// Take the live state of `predecessor`, the lane of the plan this lane's plan displaces, at
    /// the swap block (#1280 D2). The predecessor never renders again, so nothing here consumes
    /// from it and nothing writes a cell: control stays each cell's only writer.
    ///
    /// * The applied live bypass flag, the bypass crossfade (its mix and position, so a swap inside
    ///   a ramp continues it, #1341 D7) and the live symmetry terms are copied, and so is the link
    ///   mode the predecessor's processor lane holds, when it is known (#1371 D6): the stage that
    ///   moves the processor's state has seated that mode first ([`Self::link_to_seat`]).
    /// * Each predecessor cell's newest unread write ([`CellReader::peek_unread`]) goes to this
    ///   lane's carried slot of the cell, with the count of unread writes. The slot resolves at
    ///   this lane's first rendered block, before that block stages anything ([`Self::stage`]):
    ///   the successor's own newer write wins, otherwise the carried write applies, and the
    ///   writes replaced join `live_values_superseded` exactly once. A slot the predecessor
    ///   itself had not resolved yet stays pending, with its writes added.
    ///
    /// The caller has checked [`Self::can_carry_from`]; a lane that fails it takes nothing and
    /// reports `false`. Allocation-free, lock-free and bounded by the cell count.
    pub fn carry_from(&mut self, predecessor: &Self) -> bool {
        if !self.can_carry_from(predecessor) {
            return false;
        }
        self.bypass = predecessor.bypass;
        self.mix = predecessor.mix;
        self.mix_position = predecessor.mix_position;
        self.hold = predecessor.hold;
        self.rest_pending = predecessor.rest_pending;
        self.symmetry = predecessor.symmetry;
        if predecessor.link.is_some() {
            self.link = predecessor.link;
        }
        let Some(before) = predecessor.cells.as_deref() else {
            return true;
        };
        let mut pending = false;
        self.carried_bypass = CarriedCell::of(
            &predecessor.bypass_reader,
            &before.bypass,
            predecessor.carried_bypass,
        );
        pending |= self.carried_bypass.unread != 0;
        if before.shape.has_link() {
            self.carried_link = CarriedCell::of(
                &predecessor.link_reader,
                &before.link,
                predecessor.carried_link,
            );
            pending |= self.carried_link.unread != 0;
        }
        // A predecessor that was never carried into holds no slots: it has no earlier writes.
        for (index, ((slot, cell), reader)) in self
            .carried_parameters
            .iter_mut()
            .zip(&before.parameters)
            .zip(&predecessor.parameter_readers)
            .enumerate()
        {
            let earlier = predecessor
                .carried_parameters
                .get(index)
                .copied()
                .unwrap_or(CarriedCell::EMPTY);
            *slot = CarriedCell::of(reader, cell, earlier);
            pending |= slot.unread != 0;
        }
        for (index, ((slot, cell), reader)) in self
            .carried_targets
            .iter_mut()
            .zip(&before.targets)
            .zip(&predecessor.target_readers)
            .enumerate()
        {
            let earlier = predecessor
                .carried_targets
                .get(index)
                .copied()
                .unwrap_or(CarriedCell::EMPTY);
            *slot = CarriedCell::of(reader, cell, earlier);
            pending |= slot.unread != 0;
        }
        self.carried_pending = pending;
        true
    }

    /// The link mode this lane's processor holds, as far as the lane knows it (#1371 D6): the
    /// prepared mode until a switch applies, then the last one [`Self::settle_link`] recorded.
    #[must_use]
    pub const fn link_mode(&self) -> Option<LinkMode> {
        self.link
    }

    /// The mode the stage that carries `predecessor` into this lane seats on this lane's processor
    /// before it restores the predecessor's state payload (#1371 D6): the predecessor's mode, when
    /// both lanes know theirs and they differ. A restore keeps a payload's glide words only under
    /// the mode they glide to, so the seat lets the glide in flight continue bit for bit; the
    /// link write of the committed mode (the successor's own, or the predecessor's carried one)
    /// then glides the lane on from there, with no step. `None` when there is nothing to seat.
    #[must_use]
    pub fn link_to_seat(&self, predecessor: &Self) -> Option<LinkMode> {
        match (self.link, predecessor.link) {
            (Some(own), Some(before)) if own != before => Some(before),
            _ => None,
        }
    }

    /// Record what the processor did with the link switch to `mode` that the last
    /// [`Self::stage`] returned (#1371 D2): `taken` is `retarget_link`'s answer. A taken switch
    /// becomes the lane's mode; a refused one is counted in [`Self::link_refusals`] and changes
    /// nothing.
    pub fn settle_link(&mut self, mode: LinkMode, taken: bool) {
        if taken {
            self.link = Some(mode);
        } else {
            self.link_refusals = self.link_refusals.saturating_add(1);
        }
    }

    /// Link switches the processor refused ([`Self::settle_link`]). The writer refuses every switch
    /// an effect would, so this is `0`.
    #[must_use]
    pub const fn link_refusals(&self) -> u64 {
        self.link_refusals
    }

    /// The targets applied by the most recent [`stage`](Self::stage), in `(slot, channel)` order.
    #[must_use]
    pub fn prepared_targets(&self) -> &[PreparedEffectTarget] {
        &self.targets[..self.staged_targets]
    }

    /// Read every pending cell under the block's snapshot `snapshot` and stage what they hold: the
    /// bypass first, then the link switch ([`Staged::link`], which the caller hands to the
    /// processor's `retarget_link` and settles with [`Self::settle_link`]), then the parameter spans into `staging` in canonical `(parameter_index,
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
    /// [`Self::parameter_cells`] spans long (checked at bind with [`Self::check_window`]) and no
    /// longer than the effect's `automation_capacity`, which the pairing rule depends on
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
            link_reader,
            parameter_readers,
            target_readers,
            dirty,
            pending,
            superseded,
            targets,
            staged_targets,
            bypass,
            mix,
            mix_position,
            hold,
            rest_pending,
            symmetry,
            carried_bypass,
            carried_link,
            carried_parameters,
            carried_targets,
            carried_pending,
            link,
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
        // A lane that took a plan swap's carried writes resolves them in its first block, whether or
        // not its own cells hold anything (#1280 D2).
        if any == 0 && !*carried_pending {
            return Staged::default();
        }
        *carried_pending = false;
        let mut reads = Reads {
            pending,
            dirty,
            snapshot,
            superseded,
        };
        if let Some([word, ramp_samples]) =
            reads.read(0, &cells.bypass, bypass_reader, Some(carried_bypass))
        {
            let value = word != 0;
            // A record whose target is the lane's current target changes nothing, whether the
            // lane is settled there or still on its way (#1341 D2). Otherwise the crossfade starts
            // from the mix the lane is at now, so a reversal mid-ramp continues without a jump.
            if value != *bypass {
                // #2129 D2: a lane leaving settled bypass resumes from rest.
                *rest_pending = !value && *mix_position >= mix.length && *hold == 0;
                let now = mix.coefficients_at(*mix_position)[0];
                let target = if value { 0.0 } else { 1.0 };
                // The writer refuses a longer ramp, so the cell holds only checked words.
                *mix = IndexedRamp::new(
                    [now, 0.0, 0.0, 0.0],
                    [target, 0.0, 0.0, 0.0],
                    ramp_samples.min(INDEXED_RAMP_LENGTH_MAXIMUM),
                );
                *mix_position = 0;
                // A switch during a hold starts from the mix the hold kept, the dry `0`, and ends
                // the hold: the new ramp is the lane's whole law from here (#2129 D3).
                *hold = 0;
                *bypass = value;
                // A ramping lane counts as bypassed until it settles at wet (#1341 D6).
                symmetry.admit(&EffectControlRecord::Bypass(BypassSwitch::step(
                    value || mix.length != 0,
                )));
            }
        }
        // A switch whose mode is the one the lane already heads for changes nothing (the newest
        // write of the cell decides, so a change and its reversal before a read leave the lane be).
        let mut switch = None;
        if cells.shape.has_link()
            && let Some([code, ramp_samples]) = reads.read(
                EffectLaneShape::LINK_BIT,
                &cells.link,
                link_reader,
                Some(carried_link),
            )
            // The writer stores only checked switches, so every code names a mode.
            && let Some(mode) = LinkSwitch::mode_of(code)
            && *link != Some(mode)
        {
            let record = LinkSwitch { mode, ramp_samples };
            symmetry.admit(&EffectControlRecord::Link(record));
            switch = Some(record);
        }
        let base = cells.shape.parameter_base();
        let mut staged = 0_usize;
        for group in cells.shape.groups.iter() {
            let first = group.first as usize;
            let parameter_index = group.parameter_index;
            let mut applied = [None; 2];
            for (offset, slot) in applied
                .iter_mut()
                .enumerate()
                .take(1 + usize::from(group.pair))
            {
                *slot = reads
                    .read(
                        base + first + offset,
                        &cells.parameters[first + offset],
                        &mut parameter_readers[first + offset],
                        carried_parameters.get_mut(first + offset),
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
                    channel,
                    parameter_index,
                    sample: first_sample,
                    value,
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
                    carried_targets.get_mut(index),
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
            link: switch,
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
    ///
    /// `carried` is the write the retiring plan's cell held unread at a plan swap (#1280 D2),
    /// resolved here in #1345 D3's order, and emptied. When the cell's own newer write applies,
    /// it wins and every carried write was replaced: the count gains `unread`. Otherwise the
    /// carried write applies and the older ones it replaced join the count: `unread - 1`. The
    /// predecessor never reads again, so this lane counts for it.
    fn read<const N: usize>(
        &mut self,
        bit: usize,
        cell: &LatestCell<N>,
        reader: &mut CellReader,
        carried: Option<&mut CarriedCell<N>>,
    ) -> Option<[u32; N]> {
        let own = if self.pending[bit / 64] & (1 << (bit % 64)) == 0 {
            None
        } else {
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
        };
        // A lane whose slots are not installed was never carried into: nothing waits in them.
        let Some(carried) = carried.filter(|carried| carried.unread != 0) else {
            return own;
        };
        let unread = core::mem::replace(&mut carried.unread, 0);
        if own.is_some() {
            self.superseded.add_carried(unread);
            own
        } else {
            self.superseded.add_carried(unread - 1);
            Some(carried.words)
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
        arm_tag,
    } = record
    else {
        return false;
    };
    observation.is_some_and(|lane| lane.arm(tap_index, armed, window_blocks, first_sample, arm_tag))
}
// REALTIME_POLICY_END

/// What one [`EffectControlLane::stage`] call produced.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Staged {
    /// Spans written to the front of the staging window, in canonical order.
    pub staged: usize,
    /// Prepared targets retained in the lane-owned prefix.
    pub staged_targets: usize,
    /// The link switch this block applies, for the caller to hand to the processor's
    /// `retarget_link` before the block renders and to settle on the lane
    /// ([`EffectControlLane::settle_link`]); `None` when the lane's mode does not change.
    pub link: Option<LinkSwitch>,
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
    /// The declared bypass value ([`ObservationDescriptor::bypass_observation`], issue #2129 D5),
    /// copied once at bind: what the tap folds for a block its lane emitted settled-bypassed.
    bypass: Option<f32>,
    armed: bool,
    /// Armed at preparation ([`ObservationLane::arm_prepared`]) and not yet accumulated: the
    /// window's first sample is the first sample of the first block that folds into it, which is
    /// not known before the plan renders. Cleared by that block, by [`ObservationLane::arm`] and by
    /// [`ObservationLane::disarm_all`]. Part of the tap's state, so [`ObservationLane::swap_state`]
    /// moves it with the rest and a carried tap's own state replaces it whole.
    start_pending: bool,
    /// Armed at preparation and not yet published in the slot (#1585 R2): the block drain that
    /// precedes the plan's first block publishes it ([`ObservationLane::publish_prepared_arms`]),
    /// before any silence skip or resident read can pass the tap by. Cleared by that drain, by
    /// [`ObservationLane::arm`] and by [`ObservationLane::disarm_all`].
    arm_unpublished: bool,
    /// The first sample of the block that applied the tap's last arm or disarm, as its slot
    /// publishes it (#1585 R2). A prepared arm's is set by the plan's first block drain.
    arm_sample: u64,
    /// The tag of the record that last armed or disarmed the tap, `0` for none (#1585 R2).
    arm_tag: u64,
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
    /// a tap *does*; this is only a redundant count of it, and `set_arm` (behind `arm` and
    /// `arm_prepared`) and `disarm_all` are the only two writers of either.
    armed_count: u32,
    /// How many armed taps of this lane declare no bypass value (issue #2129 D5): an armed tap that
    /// needs its wet path computed. Maintained beside [`Self::armed_count`] by the same writers,
    /// so [`any_armed_computed`](Self::any_armed_computed) is one load.
    armed_computed: u32,
    /// Whether a tap of this lane may hold a prepared arm its slot has not published (#1585 R2),
    /// so [`publish_prepared_arms`](Self::publish_prepared_arms) is one load on every block after
    /// the first. Set by `arm_prepared`, cleared by that publication and by `disarm_all`.
    arms_unpublished: bool,
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
                bypass: descriptor.bypass_observation,
                armed: false,
                start_pending: false,
                arm_unpublished: false,
                arm_sample: 0,
                arm_tag: 0,
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
            // Every tap is built unarmed, so the counts start where they do.
            armed_count: 0,
            armed_computed: 0,
            arms_unpublished: false,
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
    ///
    /// The tap's slot publishes the new arm state, `armed` at `first_sample` with `arm_tag`, in one
    /// publication (#1585 D3): render is the one writer of a tap's arm state, and this is where it
    /// writes it. The window the slot holds is kept.
    pub fn arm(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        first_sample: u64,
        arm_tag: u64,
    ) -> bool {
        let Some(tap) = self.set_arm(tap_index, armed, window_blocks, first_sample, arm_tag) else {
            return false;
        };
        tap.publisher.publish_arm(armed, first_sample, arm_tag);
        true
    }

    /// Set tap `tap_index`'s arm state and restart its window at `first_sample`, publishing
    /// nothing; `None` for an index this lane has no tap for.
    fn set_arm(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        first_sample: u64,
        arm_tag: u64,
    ) -> Option<&mut ObservationTap> {
        let default = self.default_window_blocks;
        let tap = usize::try_from(tap_index)
            .ok()
            .and_then(|index| self.taps.get_mut(index))?;
        // Maintained across the transition, not recomputed: a re-arm of an already-armed tap is
        // idempotent here exactly as it is for `armed` itself.
        let computed = u32::from(tap.bypass.is_none());
        match (tap.armed, armed) {
            (false, true) => {
                self.armed_count = self.armed_count.saturating_add(1);
                self.armed_computed = self.armed_computed.saturating_add(computed);
            }
            (true, false) => {
                self.armed_count = self.armed_count.saturating_sub(1);
                self.armed_computed = self.armed_computed.saturating_sub(computed);
            }
            _ => {}
        }
        tap.armed = armed;
        tap.start_pending = false;
        tap.arm_unpublished = false;
        tap.arm_sample = first_sample;
        tap.arm_tag = arm_tag;
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
        Some(tap)
    }

    /// Arm `tap_index` at preparation, before the plan has rendered a block (#1584 D6a).
    ///
    /// A successor plan is prepared from the monitor state the host acknowledged, so a subscription
    /// survives a rebuild that restarts its effect. The adoption sample is not known here, so the
    /// tap's first window opens at the first sample of the first block that accumulates into it
    /// (`accumulate` takes it), exactly as if a record had armed it at that block. No record is
    /// queued: a queued record would re-arm a *carried* tap at its first block and restart its
    /// window. A carried tap exchanges its whole state with the predecessor's at the swap
    /// ([`Self::swap_state`]), so this arm never wins over a carried tap's own.
    ///
    /// A zero `window_blocks` means the lane's default, as in [`Self::arm`]. Returns `false` for an
    /// index this lane has no tap for. Off the render thread, before the plan is published.
    ///
    /// The slot publishes nothing here: render is the one writer of a tap's arm state (#1585 D3),
    /// so the drain at the top of the plan's first block publishes it armed at that block's first
    /// sample with `arm_tag`, the tag of the binding it was prepared from
    /// ([`Self::publish_prepared_arms`]). Until that block the slot holds no arm, and a reader that
    /// knows the tag it expects reports the tap pending.
    pub fn arm_prepared(&mut self, tap_index: u32, window_blocks: u32, arm_tag: u64) -> bool {
        let Some(tap) = self.set_arm(tap_index, true, window_blocks, 0, arm_tag) else {
            return false;
        };
        tap.start_pending = true;
        tap.arm_unpublished = true;
        self.arms_unpublished = true;
        true
    }

    /// Release every subscription without touching the published windows (issue #143 D7).
    ///
    /// A plan replacement drops subscriptions because the plan they addressed is gone; the app
    /// re-subscribes against the new plan. Disarming here is the same operation an explicit
    /// unsubscribe performs, so there is one code path and one meaning.
    ///
    /// The slots publish nothing (#1585 R2): only render writes a tap's arm state, and no
    /// production path calls this; only tests do, directly or through
    /// `rack::BankSlot::disarm_observations`.
    pub fn disarm_all(&mut self) {
        self.armed_count = 0;
        self.armed_computed = 0;
        self.arms_unpublished = false;
        for tap in self.taps.iter_mut() {
            tap.armed = false;
            tap.start_pending = false;
            tap.arm_unpublished = false;
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

    /// Whether an armed tap of this lane declares no bypass value, so its reading needs the wet
    /// path to run (issue #2129 D5). One load, from the count the arm writers maintain.
    #[must_use]
    pub const fn any_armed_computed(&self) -> bool {
        self.armed_computed != 0
    }

    /// Tap `tap_index`'s declared bypass value, or `None` for a tap that declares none or an index
    /// this lane has no tap for (issue #2129 D5).
    #[must_use]
    pub fn bypass_value(&self, tap_index: usize) -> Option<f32> {
        self.taps.get(tap_index).and_then(|tap| tap.bypass)
    }

    /// Fold every armed tap's declared bypass value into its window, for a block the lane emitted
    /// settled-bypassed and no wet path rendered (issue #2129 D5): each tap folds it exactly as
    /// [`Self::accumulate`] folds a computed reading, so window peaks, cadence and the window's
    /// samples are unchanged in shape. An armed tap that declares no value folds nothing; a caller
    /// skips a block only when no such tap is armed ([`Self::any_armed_computed`]).
    pub fn accumulate_bypass(&mut self, first_sample: u64, frames: u64) {
        if !self.any_armed() {
            return;
        }
        for tap in 0..self.taps.len() {
            if let Some(value) = self.taps[tap].bypass {
                self.accumulate(
                    tap,
                    ObservationSample {
                        left: value,
                        right: value,
                    },
                    first_sample,
                    frames,
                );
            }
        }
    }

    /// Whether [`Self::swap_state`] would exchange this lane's taps with `other`'s: both declare
    /// the same number of taps, with the same fold at each index (the same effect's menu, #1327
    /// D1). One load and one compare per tap; allocation-free.
    #[must_use]
    pub fn same_taps(&self, other: &Self) -> bool {
        self.taps.len() == other.taps.len()
            && self
                .taps
                .iter()
                .zip(other.taps.iter())
                .all(|(own, theirs)| own.fold == theirs.fold)
    }

    /// Exchange every tap's whole state with `other`'s at a plan swap (#1327 D2): arm state, open
    /// window, sequence, accumulated peaks and the publisher, so a reader that holds the carried
    /// tap's slot stays connected to the tap that renders on. The two lanes' default window
    /// lengths stay where they are: each plan sized its own, and a carried tap keeps the length
    /// it was armed with.
    ///
    /// Returns `false` and exchanges nothing unless [`Self::same_taps`]. Two pointer exchanges and
    /// a scalar one: no allocation, free, lock or syscall, and constant time.
    pub fn swap_state(&mut self, other: &mut Self) -> bool {
        if !self.same_taps(other) {
            return false;
        }
        core::mem::swap(&mut self.taps, &mut other.taps);
        core::mem::swap(&mut self.armed_count, &mut other.armed_count);
        core::mem::swap(&mut self.armed_computed, &mut other.armed_computed);
        core::mem::swap(&mut self.arms_unpublished, &mut other.arms_unpublished);
        true
    }

    /// Publish every prepared arm this lane's slots do not hold yet (#1585 R2): armed at
    /// `first_sample`, the first sample of the block whose drain calls it, with the tag the arm was
    /// prepared with, as the drain of a record at that block would. The block drain calls it before
    /// it applies any record, so a record of the same block overwrites it as a later record does.
    /// It runs in the drain, which no silence skip or resident read bypasses. One load on every
    /// block after the first; a bounded walk over the taps once.
    pub fn publish_prepared_arms(&mut self, first_sample: u64) {
        if !core::mem::take(&mut self.arms_unpublished) {
            return;
        }
        for tap in self.taps.iter_mut() {
            if core::mem::take(&mut tap.arm_unpublished) {
                tap.arm_sample = first_sample;
                tap.publisher
                    .publish_arm(tap.armed, first_sample, tap.arm_tag);
            }
        }
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
        // A tap armed at preparation opens its first window here, at the first block it folds
        // (#1584 D6a). Every other window's start is set below.
        if tap.start_pending {
            tap.start_pending = false;
            tap.first_sample = first_sample;
        }
        // `first_sample` is otherwise **not** taken from the block. It was set when the tap was
        // armed and it is set again to `end_sample` when a window closes, so consecutive windows
        // tile with no gap as a property of this type rather than of whatever the caller happens
        // to pass.
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
            // The value of largest magnitude, sign kept, so a boost and a cut of the same size
            // stay distinguishable. The strict `>` keeps the earlier of two equal magnitudes, and
            // the window's `0.0` reset is beaten by any nonzero reading.
            ObservationFold::PeakSigned => {
                if sample.left.abs() > tap.peak_left.abs() {
                    tap.peak_left = sample.left;
                }
                if sample.right.abs() > tap.peak_right.abs() {
                    tap.peak_right = sample.right;
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
/// * **A settled lane is selected, never computed.** A lane settled dry copies the dry block
///   (`copy_from_slice`) and a lane settled wet keeps the wet words untouched, so after a ramp the
///   output is bit-identical to a whole-block select, and `-0.0` survives it. A session whose
///   `control_smoothing.mute_ms` is 0 gives every bypass record a ramp of zero samples, which
///   renders exactly that select.
/// * **A switch is a linear crossfade over the session ramp (#1341).** A bypass record applies at
///   its block boundary (#140's rule) and starts a crossfade over `ramp_samples` frames:
///   `out = dry + (wet - dry) * m(k)`, where `m` runs from the mix the lane is at (`1` wet, `0`
///   dry) to the target on the [`IndexedRamp`] law, a pure function of the frame index `k`. It is a
///   separate subtract, multiply and add -- no fused multiply-add, which base Wasm SIMD does not
///   guarantee -- and the same `m` serves both channels, so the switch never makes them differ.
///   The law is linear in amplitude, the engine's one ramp family: dry and wet are the same
///   signal, latency-matched, and are highly correlated, so an equal-gain crossfade keeps the
///   level where an equal-power one would bulge by up to 3 dB at the midpoint (#1055,
///   `FINDINGS.md` section 3). The effect's latency does not change: the dry side stays delayed by
///   exactly the declared latency throughout.
///
/// # When a shunt exists (issue #1087)
///
/// A session `bypass` is per-lane shunt state, not a prepared program, for every effect (issues
/// #1087, #1339, #1340): preparation lowers it to a prepared
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
/// every stage after it zeroes a block that leaves that range. The delay, which never banks
/// (`effect_compiler::NEVER_BANKED_EFFECTS`), lowers its bypass to a per-node shunt too (issue
/// #1339), and its D7 check also reads its ring and damping state: on a block where that state
/// trips it, the shunt passes the dry block where its prepared bypass zeroed it (decision 15 E4).
/// The multiband's D7 check reads its crossover and smoother state the same way, and recovers
/// per lane (issue #1340), so the shunt passes a tripped bypassed lane's dry block and its
/// bank-mates are untouched. `graph-compiler`'s
/// `bypass_shunt_identity` and `bypass_cohorts` tests are the gates.
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
    /// The heap this shunt retains, exclusive of the shunt itself (#1074): its four planes, from
    /// their lengths. Control plane only.
    #[must_use]
    pub const fn retained_bytes(&self) -> u64 {
        use crate::retained::slice;
        slice(&self.dry_left)
            + slice(&self.dry_right)
            + slice(&self.line_left)
            + slice(&self.line_right)
    }

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
    /// bypassed may skip it. At nonzero latency the line must still be fed, or the first bypassed
    /// block would emit stale samples; such a block feeds it with [`feed_line`](Self::feed_line),
    /// which stores the line's input without the dry staging (issue #1763). This predicate is the
    /// exact boundary between those two cases, and it is fixed at preparation rather than per
    /// block.
    #[must_use]
    pub const fn feeds_line(&self) -> bool {
        !self.line_left.is_empty()
    }

    /// Whether `other` was allocated for the same block and the same latency, so a plan swap can
    /// exchange the two shunts whole (#1282 D2): every buffer of one has the length of the
    /// matching buffer of the other. Allocation-free and constant time.
    #[must_use]
    pub fn same_shape(&self, other: &Self) -> bool {
        self.dry_left.len() == other.dry_left.len()
            && self.dry_right.len() == other.dry_right.len()
            && self.line_left.len() == other.line_left.len()
            && self.line_right.len() == other.line_right.len()
    }

    /// Capture this block's input and advance the latency line, returning nothing.
    ///
    /// Called on every block whenever [`feeds_line`](Self::feeds_line) is true, bypassed or not:
    /// the line has to stay fed so that enabling bypass mid-stream produces the correctly delayed
    /// dry signal on its very first block rather than `latency` samples of stale zeros. A caller
    /// that is not bypassed this block may feed the line with [`feed_line`](Self::feed_line)
    /// instead, which leaves the line and its cursor exactly as this does (issue #1763). When
    /// `feeds_line` is false there is no line, and such a caller may skip the call entirely (#163
    /// phase 4 item 4).
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

    // REALTIME_POLICY_BEGIN
    /// Advance the latency line by this block's input without staging the dry block: the line
    /// half of [`capture`](Self::capture), for a block no lane reads dry (issue #1763).
    ///
    /// The exchange in `capture` is a swap, so the words it leaves in the line are the input
    /// words and nothing else: input word `j` lands at `(cursor + j) % length`, and a block longer
    /// than the line writes a position more than once, so the newest `length` words survive. This
    /// stores exactly those words at exactly those positions and leaves the cursor where
    /// `capture` leaves it, so the line and the cursor equal `capture`'s bit for bit. It does not
    /// fill `dry_*` with the line's old words, and nothing reads them: `dry_*` is per-block
    /// scratch, read only by [`apply`](Self::apply), [`dry`](Self::dry) and the crossfades later in
    /// the same block, and only for a lane that is bypassed or ramping; a block with such a lane
    /// calls `capture`. A shunt without a line has nothing to feed.
    pub fn feed_line(&mut self, left: &[f32], right: &[f32]) {
        let length = self.line_left.len();
        if length == 0 {
            return;
        }
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        let skipped = frames.saturating_sub(length);
        let words = frames - skipped;
        let start = (self.cursor + skipped) % length;
        let first = core::cmp::min(words, length - start);
        let (left, right) = (&left[skipped..frames], &right[skipped..frames]);
        self.line_left[start..start + first].copy_from_slice(&left[..first]);
        self.line_right[start..start + first].copy_from_slice(&right[..first]);
        let rest = words - first;
        self.line_left[..rest].copy_from_slice(&left[first..]);
        self.line_right[..rest].copy_from_slice(&right[first..]);
        let advanced = start + words;
        self.cursor = if advanced >= length {
            advanced - length
        } else {
            advanced
        };
    }

    /// Replace this block with the latency-matched dry block in place, for a block no wet path
    /// renders (issue #2129 D1): the latency line half of [`capture`](Self::capture) run on the
    /// planes themselves, so each plane leaves holding the words `capture` followed by
    /// [`apply`](Self::apply) leaves there, and the line and its cursor end as `capture` leaves
    /// them. Every word moves by a copy, so `-0.0` survives. The dry staging is not written: it is
    /// per-block scratch, and a block no wet path renders has no mix to read it. A shunt without a
    /// line writes nothing, because the planes already hold the dry words.
    pub fn passthrough(&mut self, left: &mut [f32], right: &mut [f32]) {
        if self.line_left.is_empty() {
            return;
        }
        let frames = left.len().min(self.dry_left.len()).min(right.len());
        let length = self.line_left.len();
        let mut offset = 0;
        while offset < frames {
            let take = core::cmp::min(length, frames - offset);
            let cursor = self.cursor;
            let mut left_cursor = cursor;
            pdc_delay_block(
                &mut self.line_left,
                &mut left_cursor,
                &mut left[offset..offset + take],
            );
            let mut right_cursor = cursor;
            pdc_delay_block(
                &mut self.line_right,
                &mut right_cursor,
                &mut right[offset..offset + take],
            );
            debug_assert_eq!(left_cursor, right_cursor);
            self.cursor = left_cursor;
            offset += take;
        }
    }

    /// Copy lane `predecessor_lane` of `predecessor`'s interleaved latency line into lane `lane`
    /// of this one, at a plan swap (#1280 D5). Both shunts are the lines of an AoSoA bank, of
    /// `predecessor_lanes` and `lanes` lanes, and one lane's samples are every `lanes`-th word of
    /// its line. The line is a ring whose shared cursor is its oldest word, and the block it
    /// receives is a whole number of frames of `lanes` words, so the word `t * lanes + lane`
    /// after the cursor is lane `lane`'s sample of age `latency - t`. The copy walks both rings
    /// from their own cursors, so a successor whose cursor sits elsewhere, or whose bank has
    /// another width, keeps the lane's delayed dry signal exactly.
    ///
    /// Returns `false`, copying nothing, when either line is not a whole number of frames of its
    /// lanes, the lane indexes are out of range, or the two lanes' latencies differ; two shunts
    /// both without a line (zero latency) copy nothing and return `true`. Allocation-free and
    /// bounded by the latency.
    pub fn carry_line_lane_from(
        &mut self,
        lane: usize,
        lanes: usize,
        predecessor: &Self,
        predecessor_lane: usize,
        predecessor_lanes: usize,
    ) -> bool {
        let (length, before_length) = (self.line_left.len(), predecessor.line_left.len());
        if length == 0 && before_length == 0 {
            return true;
        }
        if lanes == 0
            || predecessor_lanes == 0
            || lane >= lanes
            || predecessor_lane >= predecessor_lanes
            || self.line_right.len() != length
            || predecessor.line_right.len() != before_length
            || length % lanes != 0
            || before_length % predecessor_lanes != 0
            || length / lanes != before_length / predecessor_lanes
        {
            return false;
        }
        let latency = length / lanes;
        // Each ring's position of the lane's sample of age rank `t`, advanced by one frame of
        // words per step and wrapped at the ring's end.
        let mut to = self.cursor + lane;
        let mut from = predecessor.cursor + predecessor_lane;
        for _ in 0..latency {
            if to >= length {
                to -= length;
            }
            if from >= before_length {
                from -= before_length;
            }
            self.line_left[to] = predecessor.line_left[from];
            self.line_right[to] = predecessor.line_right[from];
            to += lanes;
            from += predecessor_lanes;
        }
        true
    }

    /// Exchange lane `lane`'s words of this shunt's interleaved latency line with lane
    /// `other_lane`'s words of `other`'s line, or of this same line when `other` is `None`: a
    /// render-time lane swap (issue #1747). Both lines are the lines of AoSoA banks of `lanes`
    /// and `other_lanes` lanes (equal when `other` is `None`), laid out as
    /// [`Self::carry_line_lane_from`] reads them, and each lane is walked from its own line's
    /// shared cursor, so the sample of age `latency - t` of one lane trades places with the sample
    /// of the same age of the other. The shared cursors never move: they are per line, not per
    /// lane, and the dry staging (`dry_*`) is per-block scratch that never crosses a block.
    ///
    /// Returns `false`, exchanging nothing, when either line is not a whole number of frames of
    /// its lanes, a lane index is out of range, the two lanes' latencies differ, or `other` is
    /// `None` and `other_lanes` is not `lanes`. Two lines both empty (zero latency), or a lane
    /// swapped with itself, exchange nothing and return `true`. Allocation-free and bounded by the
    /// latency.
    pub fn swap_line_lanes(
        &mut self,
        lane: usize,
        lanes: usize,
        other: Option<&mut Self>,
        other_lane: usize,
        other_lanes: usize,
    ) -> bool {
        let length = self.line_left.len();
        let (other_length, other_right) = match other.as_deref() {
            Some(other) => (other.line_left.len(), other.line_right.len()),
            None => (length, self.line_right.len()),
        };
        if length == 0 && other_length == 0 {
            return true;
        }
        if lanes == 0
            || other_lanes == 0
            || lane >= lanes
            || other_lane >= other_lanes
            || self.line_right.len() != length
            || other_right != other_length
            || !length.is_multiple_of(lanes)
            || !other_length.is_multiple_of(other_lanes)
            || length / lanes != other_length / other_lanes
            || (other.is_none() && other_lanes != lanes)
        {
            return false;
        }
        let latency = length / lanes;
        let mut own = self.cursor + lane;
        match other {
            Some(other) => {
                let mut theirs = other.cursor + other_lane;
                for _ in 0..latency {
                    if own >= length {
                        own -= length;
                    }
                    if theirs >= other_length {
                        theirs -= other_length;
                    }
                    core::mem::swap(&mut self.line_left[own], &mut other.line_left[theirs]);
                    core::mem::swap(&mut self.line_right[own], &mut other.line_right[theirs]);
                    own += lanes;
                    theirs += other_lanes;
                }
            }
            None => {
                if lane == other_lane {
                    return true;
                }
                let mut theirs = self.cursor + other_lane;
                for _ in 0..latency {
                    if own >= length {
                        own -= length;
                    }
                    if theirs >= length {
                        theirs -= length;
                    }
                    self.line_left.swap(own, theirs);
                    self.line_right.swap(own, theirs);
                    own += lanes;
                    theirs += lanes;
                }
            }
        }
        true
    }
    // REALTIME_POLICY_END

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

    // REALTIME_POLICY_BEGIN
    /// Crossfade the wet block in `left`/`right` against the captured dry block, in place, along
    /// `ramp`'s word 0 (#1341 D3, D4).
    ///
    /// Frame `f` of the block takes ramp index `k = position + f + 1`, the convention of
    /// [`route_mix_ramp_block`](lane::kernels::route_mix_ramp_block): a ramp's first frame is
    /// already one step along it. The block splits by frame count, decided from `position` and
    /// `ramp.length` before any loop:
    ///
    /// 1. the frames with `k < length` run the ramp body, vectorised over frames: a frame-index
    ///    vector `k0 + [1, ..., L::WIDTH]` (exact: every value is an integer below `2^24`) yields
    ///    `m = fma(k, step, start)` (two roundings), and the frame is
    ///    `dry + (wet - dry) * m` as a separate subtract, multiply and add;
    /// 2. the ramp frames that do not fill a vector run the same arithmetic in a non-generic,
    ///    outlined `f32` function (the browser's kernel-shape rule);
    /// 3. the frames with `k >= length` are settled and never computed: `settled_dry` copies the
    ///    dry words, and a settled-wet lane leaves the wet words as they are.
    ///
    /// `m(k)` is a pure function of `k`, so the result does not depend on the width.
    #[inline(never)]
    pub fn crossfade<L: Lane>(
        &self,
        left: &mut [f32],
        right: &mut [f32],
        ramp: &IndexedRamp,
        position: u32,
        settled_dry: bool,
    ) {
        crossfade_planes::<L>(
            left,
            right,
            &self.dry_left,
            &self.dry_right,
            ramp,
            position,
            settled_dry,
        );
    }

    /// [`Self::crossfade`] for a lane that still holds dry for `hold` frames (issue #2129 D3):
    /// the first `hold` frames of the block (all of them, for a longer hold) take the dry words by
    /// a copy, and the frames after them crossfade from ramp position `position`, so the first
    /// frame past the hold takes `k = position + 1`. Bit for bit what a bank lane renders with the
    /// hold folded into its position ([`BypassShunt::crossfade_bank`]'s `k < 1` rule).
    pub fn crossfade_held<L: Lane>(
        &self,
        left: &mut [f32],
        right: &mut [f32],
        ramp: &IndexedRamp,
        position: u32,
        hold: u32,
        settled_dry: bool,
    ) {
        let count = core::cmp::min(
            core::cmp::min(left.len(), right.len()),
            core::cmp::min(self.dry_left.len(), self.dry_right.len()),
        );
        let held = core::cmp::min(count, hold as usize);
        left[..held].copy_from_slice(&self.dry_left[..held]);
        right[..held].copy_from_slice(&self.dry_right[..held]);
        crossfade_planes::<L>(
            &mut left[held..count],
            &mut right[held..count],
            &self.dry_left[held..count],
            &self.dry_right[held..count],
            ramp,
            position,
            settled_dry,
        );
    }
    // REALTIME_POLICY_END
}
// REALTIME_POLICY_BEGIN

/// The body of [`BypassShunt::crossfade`] over planes and their aligned dry words.
#[inline(never)]
fn crossfade_planes<L: Lane>(
    left: &mut [f32],
    right: &mut [f32],
    dry_left: &[f32],
    dry_right: &[f32],
    ramp: &IndexedRamp,
    position: u32,
    settled_dry: bool,
) {
    {
        // All four planes are cut to the shortest, so no index below can fail: the kernel is never
        // inlined into the render loop, and a bounds check here would be its own trap owner.
        let count = core::cmp::min(
            core::cmp::min(left.len(), right.len()),
            core::cmp::min(dry_left.len(), dry_right.len()),
        );
        let (left, right) = (&mut left[..count], &mut right[..count]);
        let (dry_left, dry_right) = (&dry_left[..count], &dry_right[..count]);
        // `k = position + f + 1 < length` holds for the first `length - position - 1` frames.
        let ramping = ramp.length.saturating_sub(position).saturating_sub(1);
        let ramping = core::cmp::min(count, ramping as usize);
        let vectored = ramping - ramping % L::WIDTH;
        let (left_ramp, left_settled) = left.split_at_mut(ramping);
        let (right_ramp, right_settled) = right.split_at_mut(ramping);
        let (dry_left_ramp, dry_left_settled) = dry_left.split_at(ramping);
        let (dry_right_ramp, dry_right_settled) = dry_right.split_at(ramping);
        let (left_vectors, left_tail) = left_ramp.split_at_mut(vectored);
        let (right_vectors, right_tail) = right_ramp.split_at_mut(vectored);
        let (dry_left_vectors, dry_left_tail) = dry_left_ramp.split_at(vectored);
        let (dry_right_vectors, dry_right_tail) = dry_right_ramp.split_at(vectored);
        let step = L::splat(ramp.step[0]);
        let start = L::splat(ramp.start[0]);
        let advance = L::splat(L::WIDTH as f32);
        // One vector add per chunk advances every frame's index. Exact: each index is an integer
        // below `position + vectored < length <= 2^22` whenever a frame ramps.
        let mut index = L::splat(position as f32).add(L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]));
        for (((left, right), dry_left), dry_right) in left_vectors
            .chunks_exact_mut(L::WIDTH)
            .zip(right_vectors.chunks_exact_mut(L::WIDTH))
            .zip(dry_left_vectors.chunks_exact(L::WIDTH))
            .zip(dry_right_vectors.chunks_exact(L::WIDTH))
        {
            let mix = index.fma(step, start);
            let dry = L::load(dry_left);
            let wet = L::load(left);
            dry.add(wet.sub(dry).mul(mix)).store(left);
            let dry = L::load(dry_right);
            let wet = L::load(right);
            dry.add(wet.sub(dry).mul(mix)).store(right);
            index = index.add(advance);
        }
        if !left_tail.is_empty() {
            // `vectored < ramping < length`, so the sum stays below `2^22`.
            crossfade_tail(
                left_tail,
                right_tail,
                dry_left_tail,
                dry_right_tail,
                ramp,
                position + vectored as u32 + 1,
            );
        }
        if settled_dry {
            left_settled.copy_from_slice(dry_left_settled);
            right_settled.copy_from_slice(dry_right_settled);
        }
    }
}
// REALTIME_POLICY_END

impl BypassShunt {
    // REALTIME_POLICY_BEGIN
    /// Crossfade a bank's interleaved wet block against the captured dry block, in place, one
    /// vector per frame and channel, each lane along its own ramp (#1341 D4).
    ///
    /// The block holds `L::WIDTH` contiguous words per frame (one per bank lane), so `L` is the
    /// bank's lane type. For frame `f` the lane vector of ramp indices is
    /// `position + f + 1`, exactly [`Self::crossfade`]'s `k`, and per lane:
    ///
    /// * `k < length`: the lane ramps, `m = fma(k, step, start)` and
    ///   `out = dry + (wet - dry) * m`, the same separate subtract, multiply and add;
    /// * otherwise the lane is settled: `settled_dry` takes the dry word and a settled-wet lane
    ///   keeps the wet word. Both are a bit select, so `-0.0`, NaN payloads and subnormals pass
    ///   unchanged, and a lane's wet word never reaches the arithmetic of a settled neighbour.
    /// * `k < 1`: the lane holds dry before its ramp starts (issue #2129 D3; a held lane's
    ///   position is minus its hold), a bit select of the dry word, whatever the other two rules
    ///   say. Every lane without a hold has `k >= 1`, so the rule never selects for it.
    ///
    /// Every lane's bits equal the per-node [`Self::crossfade`] of the same lane at any width:
    /// `m(k)` is a pure function of the lane's own `k`, and the mix is the same operation order.
    /// A bank with no ramping lane skips the mix and only selects.
    ///
    /// `frames` is cut to what the four planes hold. `L::WIDTH` must not exceed
    /// [`BANK_CROSSFADE_LANES`].
    #[inline(never)]
    pub fn crossfade_bank<L: Lane>(
        &self,
        left: &mut [f32],
        right: &mut [f32],
        frames: usize,
        lanes: &BankCrossfade,
    ) {
        debug_assert!(L::WIDTH <= BANK_CROSSFADE_LANES);
        let words = core::cmp::min(
            core::cmp::min(left.len(), right.len()),
            core::cmp::min(self.dry_left.len(), self.dry_right.len()),
        );
        let words = core::cmp::min(words, frames.saturating_mul(L::WIDTH));
        let words = words - words % L::WIDTH;
        let (left, right) = (&mut left[..words], &mut right[..words]);
        let (dry_left, dry_right) = (&self.dry_left[..words], &self.dry_right[..words]);
        let zero = L::zero();
        let settled_dry = L::load(&lanes.settled_dry[..L::WIDTH]).gt(zero);
        let planes = left
            .chunks_exact_mut(L::WIDTH)
            .zip(right.chunks_exact_mut(L::WIDTH))
            .zip(dry_left.chunks_exact(L::WIDTH))
            .zip(dry_right.chunks_exact(L::WIDTH));
        if !lanes.ramping {
            for (((left, right), dry_left), dry_right) in planes {
                L::select(settled_dry, L::load(dry_left), L::load(left)).store(left);
                L::select(settled_dry, L::load(dry_right), L::load(right)).store(right);
            }
            return;
        }
        let step = L::load(&lanes.step[..L::WIDTH]);
        let start = L::load(&lanes.start[..L::WIDTH]);
        let length = L::load(&lanes.length[..L::WIDTH]);
        let one = L::splat(1.0);
        // Exact: each index is an integer below `position + frames`, far below `2^24`; and an
        // index past a lane's length only has to compare at or above it.
        let mut index = L::load(&lanes.position[..L::WIDTH]).add(one);
        for (((left, right), dry_left), dry_right) in planes {
            // A held lane's frames before its ramp take an index below `1` and read dry (#2129
            // D3); every other lane's index is at least `1`, so the select passes it through.
            let held = index.lt(one);
            let ramping = index.lt(length);
            let mix = index.fma(step, start);
            let (dry, wet) = (L::load(dry_left), L::load(left));
            let ramped = dry.add(wet.sub(dry).mul(mix));
            let settled = L::select(settled_dry, dry, wet);
            L::select(held, dry, L::select(ramping, ramped, settled)).store(left);
            let (dry, wet) = (L::load(dry_right), L::load(right));
            let ramped = dry.add(wet.sub(dry).mul(mix));
            let settled = L::select(settled_dry, dry, wet);
            L::select(held, dry, L::select(ramping, ramped, settled)).store(right);
            index = index.add(one);
        }
    }
    // REALTIME_POLICY_END
}

/// The most lanes a bank's [`BankCrossfade`] holds: the widest bank, eight lanes on AVX2.
pub const BANK_CROSSFADE_LANES: usize = 8;

/// One bank lane's bypass crossfade for a block: what [`EffectControlLane::bypass_crossfade_lane`]
/// reads and [`BankCrossfade::set`] stores.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BankCrossfadeLane {
    step: f32,
    start: f32,
    position: f32,
    length: f32,
    settled_dry: bool,
}

impl BankCrossfadeLane {
    /// A lane that only passes the wet block: no ramp, not bypassed. What a bank lane without a
    /// control lane is. Its `step` and `start` are never read (its length is zero), and all-zero
    /// words keep [`BankCrossfade::new`] a plain zero fill.
    pub const WET: Self = Self {
        step: 0.0,
        start: 0.0,
        position: 0.0,
        length: 0.0,
        settled_dry: false,
    };
}

/// The per-lane crossfade state of one bank for one block, in the vectors
/// [`BypassShunt::crossfade_bank`] loads. A plain inline value: the stage that owns it allocates
/// nothing to fill it, block after block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BankCrossfade {
    step: [f32; BANK_CROSSFADE_LANES],
    start: [f32; BANK_CROSSFADE_LANES],
    position: [f32; BANK_CROSSFADE_LANES],
    length: [f32; BANK_CROSSFADE_LANES],
    /// `1.0` for a lane settled dry (bypassed), else `0.0`: loaded and compared once per block.
    settled_dry: [f32; BANK_CROSSFADE_LANES],
    ramping: bool,
}

impl BankCrossfade {
    /// Every lane passes the wet block.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            step: [0.0; BANK_CROSSFADE_LANES],
            start: [0.0; BANK_CROSSFADE_LANES],
            position: [0.0; BANK_CROSSFADE_LANES],
            length: [0.0; BANK_CROSSFADE_LANES],
            settled_dry: [0.0; BANK_CROSSFADE_LANES],
            ramping: false,
        }
    }

    /// Store `lane`'s crossfade for this block. `lane` must be below [`BANK_CROSSFADE_LANES`].
    pub fn set(&mut self, lane: usize, state: BankCrossfadeLane) {
        self.step[lane] = state.step;
        self.start[lane] = state.start;
        self.position[lane] = state.position;
        self.length[lane] = state.length;
        self.settled_dry[lane] = if state.settled_dry { 1.0 } else { 0.0 };
        // A held lane (negative position, #2129 D3) needs the per-frame body as a ramping one does.
        self.ramping = self.length.iter().any(|length| *length > 0.0)
            || self.position.iter().any(|position| *position < 0.0);
    }
}

impl Default for BankCrossfade {
    fn default() -> Self {
        Self::new()
    }
}

/// The ramp frames of [`BypassShunt::crossfade`] that do not fill a vector: the same arithmetic at
/// `f32`, frame `f` at ramp index `first + f`.
///
/// Non-generic and never inlined, so the browser's four-lane kernel instantiation carries no
/// scalar ramp body.
// REALTIME_POLICY_BEGIN
#[inline(never)]
fn crossfade_tail(
    left: &mut [f32],
    right: &mut [f32],
    dry_left: &[f32],
    dry_right: &[f32],
    ramp: &IndexedRamp,
    first: u32,
) {
    let (step, start) = (ramp.step[0], ramp.start[0]);
    for ((((left, right), dry_left), dry_right), k) in left
        .iter_mut()
        .zip(right.iter_mut())
        .zip(dry_left)
        .zip(dry_right)
        .zip(first..)
    {
        // Exact: `k < length <= 2^22`.
        let mix = <f32 as Lane>::fma(k as f32, step, start);
        let (dry, wet) = (*dry_left, *left);
        *left = <f32 as Lane>::add(dry, <f32 as Lane>::mul(<f32 as Lane>::sub(wet, dry), mix));
        let (dry, wet) = (*dry_right, *right);
        *right = <f32 as Lane>::add(dry, <f32 as Lane>::mul(<f32 as Lane>::sub(wet, dry), mix));
    }
}
// REALTIME_POLICY_END

/// Why [`swap_bank_lanes`] exchanged nothing. Every refusal leaves both lanes as they were.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaneSwapRefusal {
    /// The scratch is shorter than [`swap_bank_lanes_scratch_bytes`].
    Scratch,
    /// A lane index does not fit the payload calls' `u32` track index.
    Lane,
    /// A bank has not declared its prepared per-lane defaults
    /// ([`PreparedNativeEffectBank::lane_default_bytes`]), or the two banks declare different
    /// sizes.
    Defaults,
    /// A bank refused to snapshot its lane (an unsupported effect, or a lane out of range).
    Snapshot,
    /// A bank refused a restore after the checks passed: a defect, since both payloads are the
    /// banks' own, of one layout, under the lanes' own defaults. Nothing was left half-swapped:
    /// the swap was undone (see [`swap_bank_lanes`]).
    Restore,
}

// REALTIME_POLICY_BEGIN
/// The scratch bytes [`swap_bank_lanes`] needs to swap two lanes of `bank` (or of `bank` and
/// another bank of its layout) whose payloads have `sizes`: two whole payloads and one lane's
/// prepared defaults. `None` when the bank has not declared its defaults, or on overflow. The
/// caller allocates it once, at preparation.
#[must_use]
pub fn swap_bank_lanes_scratch_bytes(
    bank: &dyn PreparedNativeEffectBank,
    sizes: StatePayloadSizes,
) -> Option<usize> {
    let payloads = usize::try_from(sizes.total()?.checked_mul(2)?).ok()?;
    payloads.checked_add(bank.lane_default_bytes()?)
}

/// One payload's three sections in `scratch`, cut at `sizes`.
fn swap_payload_sections(
    scratch: &mut [u8],
    sizes: StatePayloadSizes,
) -> (&mut [u8], &mut [u8], &mut [u8]) {
    let (common, rest) = scratch.split_at_mut(sizes.common_bytes as usize);
    let (left, right) = rest.split_at_mut(sizes.left_bytes as usize);
    (common, left, &mut right[..sizes.right_bytes as usize])
}

/// One payload of `scratch` as a restore input, cut at `sizes`.
fn swap_payload_input(
    payload: &[u8],
    sizes: StatePayloadSizes,
) -> Result<StatePayloadInput<'_>, crate::StatePayloadError> {
    let (common, rest) = payload.split_at(sizes.common_bytes as usize);
    let (left, right) = rest.split_at(sizes.left_bytes as usize);
    StatePayloadInput::new(common, left, &right[..sizes.right_bytes as usize], sizes)
}

/// Exchange two lanes' prepared defaults through `spare`: three in-place exchanges, which swap
/// the two lanes' bytes whatever `spare` held. `spare` comes back holding what it held as the
/// first lane's exchange hands it back, which a bank may canonicalise, so it is not promised to
/// be byte-for-byte as it was; nothing reads it afterwards. Calling it twice is the identity on
/// the two lanes. Returns whether every exchange took; the caller has checked the lanes and the
/// length, so each exchange takes.
fn rotate_lane_defaults<'one, 'two>(
    first: &mut (dyn PreparedNativeEffectBank + 'one),
    first_lane: usize,
    second: Option<&mut (dyn PreparedNativeEffectBank + 'two)>,
    second_lane: usize,
    spare: &mut [u8],
) -> bool {
    // first = X, spare = A.
    let mut taken = first.exchange_lane_defaults(first_lane, spare);
    // second = A, spare = B.
    taken &= match second {
        Some(bank) => bank.exchange_lane_defaults(second_lane, spare),
        None => first.exchange_lane_defaults(second_lane, spare),
    };
    // first = B, spare = X.
    taken &= first.exchange_lane_defaults(first_lane, spare);
    taken
}

/// Exchange the whole state of lane `first_lane` of `first` with lane `second_lane` of `second`,
/// or of `first` itself when `second` is `None` (issue #1747): a render-time lane swap.
///
/// # What moves
///
/// * The lane's state payload, the plan-swap carry's one complete record of a lane's running
///   state (#1279 D3): filter and envelope state, designed coefficients, every ramp in flight
///   and the lane's clocks.
/// * The lane's prepared per-lane defaults
///   ([`PreparedNativeEffectBank::exchange_lane_defaults`]), which a reset, a D7 recovery or a
///   restore's validation reads and which the carry deliberately leaves with the successor.
///
/// Anything a bank keeps per *bank* rather than per lane -- a shared cursor -- stays. Two banks
/// of one slot need not hold equal cursors (a whole-bank D7 reset moves one bank's limiter
/// cursors and not its neighbour's), and the swap does not need them to: each payload is written
/// relative to its own bank's cursor and restored relative to the destination bank's (the
/// restore re-ages the lane's rings to it), so the exchange is exact either way. Control lanes,
/// taps and dry shunt lines are not effect state; the stage that owns them swaps them (`rack`).
///
/// # All or nothing
///
/// Every refusal is decided before the first write to either lane: the scratch size, the lane
/// indices, the declared defaults sizes ([`LaneSwapRefusal::Defaults`]) and both snapshots, which
/// write only `scratch`. Then:
///
/// 1. The defaults rotate. Precondition: both lanes exist (their snapshots took) and `spare` is
///    exactly the declared length (checked), the only two reasons an exchange refuses.
/// 2. The first lane restores the second's payload, now under the second lane's defaults. A
///    refused restore changes nothing (a restore validates the whole payload before it commits).
/// 3. The second lane restores the first's payload, under the first lane's defaults.
///
/// Steps 2 and 3 restore a bank's own payload of one layout under the defaults it was written
/// with, which a bank never refuses (the carry's own-payload rule, #1279 D6), so a refusal there
/// is a defect, and it is still undone: a refused step 3 restores the first lane from its own
/// payload (step 2's precondition again, after the rotation is undone), and the rotation undoes
/// itself (step 1's precondition is unchanged). Both lanes are then as they were, and the call
/// returns [`LaneSwapRefusal::Restore`]. Both banks must run one layout (`state_layout_version`
/// and `sizes`), and a caller swaps only populated lanes: a padded lane never swaps.
///
/// A collapsed bank holds a stale right channel; the caller disengages its chain first, as the
/// plan-swap carry does.
///
/// # Cost
///
/// Two snapshots and two restores: each lane's state is read twice and written twice, plus two
/// payloads of `scratch`, plus three passes over one lane's defaults. Allocation-free, lock-free
/// and bounded by the prepared state size.
///
/// # Errors
///
/// [`LaneSwapRefusal`], with both lanes unchanged.
pub fn swap_bank_lanes(
    first: &mut dyn PreparedNativeEffectBank,
    first_lane: usize,
    mut second: Option<&mut dyn PreparedNativeEffectBank>,
    second_lane: usize,
    state_layout_version: u32,
    sizes: StatePayloadSizes,
    scratch: &mut [u8],
) -> Result<(), LaneSwapRefusal> {
    let defaults = first.lane_default_bytes();
    if defaults.is_none()
        || second
            .as_deref()
            .is_some_and(|bank| bank.lane_default_bytes() != defaults)
    {
        return Err(LaneSwapRefusal::Defaults);
    }
    let Some(needed) = swap_bank_lanes_scratch_bytes(&*first, sizes) else {
        return Err(LaneSwapRefusal::Scratch);
    };
    if scratch.len() < needed {
        return Err(LaneSwapRefusal::Scratch);
    }
    let (Ok(first_track), Ok(second_track)) =
        (u32::try_from(first_lane), u32::try_from(second_lane))
    else {
        return Err(LaneSwapRefusal::Lane);
    };
    if second.is_none() && first_lane == second_lane {
        return Ok(());
    }
    let payload = usize::try_from(sizes.total().unwrap_or(0)).unwrap_or(0);
    let (first_payload, rest) = scratch.split_at_mut(payload);
    let (second_payload, rest) = rest.split_at_mut(payload);
    let spare = &mut rest[..needed - 2 * payload];
    {
        let (common, left, right) = swap_payload_sections(first_payload, sizes);
        let output = StatePayloadOutput::new(common, left, right, sizes)
            .map_err(|_| LaneSwapRefusal::Scratch)?;
        first
            .snapshot_track_state_payload(first_track, output)
            .map_err(|_| LaneSwapRefusal::Snapshot)?;
    }
    {
        let (common, left, right) = swap_payload_sections(second_payload, sizes);
        let output = StatePayloadOutput::new(common, left, right, sizes)
            .map_err(|_| LaneSwapRefusal::Scratch)?;
        match second.as_deref() {
            Some(bank) => bank.snapshot_track_state_payload(second_track, output),
            None => first.snapshot_track_state_payload(second_track, output),
        }
        .map_err(|_| LaneSwapRefusal::Snapshot)?;
    }
    let first_payload: &[u8] = first_payload;
    let second_payload: &[u8] = second_payload;
    let (Ok(from_first), Ok(from_second)) = (
        swap_payload_input(first_payload, sizes),
        swap_payload_input(second_payload, sizes),
    ) else {
        return Err(LaneSwapRefusal::Scratch);
    };
    // Step 1. Precondition: both lanes snapshotted, so both exist; `spare` has the declared length.
    let rotated =
        rotate_lane_defaults(first, first_lane, second.as_deref_mut(), second_lane, spare);
    debug_assert!(rotated, "a checked lane's defaults exchange");
    // Step 2. Precondition: the second lane's own payload, under the second lane's defaults.
    if first
        .restore_track_state_payload(first_track, state_layout_version, from_second)
        .is_err()
    {
        let undone =
            rotate_lane_defaults(first, first_lane, second.as_deref_mut(), second_lane, spare);
        debug_assert!(undone, "a checked lane's defaults exchange");
        return Err(LaneSwapRefusal::Restore);
    }
    // Step 3. Precondition: the first lane's own payload, under the first lane's defaults.
    let restored = match second.as_deref_mut() {
        Some(bank) => {
            bank.restore_track_state_payload(second_track, state_layout_version, from_first)
        }
        None => first.restore_track_state_payload(second_track, state_layout_version, from_first),
    };
    if restored.is_err() {
        let undone = rotate_lane_defaults(first, first_lane, second, second_lane, spare);
        debug_assert!(undone, "a checked lane's defaults exchange");
        // Precondition: the first lane's own payload, under its own defaults again.
        let own = swap_payload_input(first_payload, sizes);
        let reverted = own.is_ok_and(|own| {
            first
                .restore_track_state_payload(first_track, state_layout_version, own)
                .is_ok()
        });
        debug_assert!(reverted, "a bank never refuses its own lane's payload");
        return Err(LaneSwapRefusal::Restore);
    }
    Ok(())
}
// REALTIME_POLICY_END

#[cfg(test)]
mod carry_tests {
    use super::BypassShunt;

    /// One frame of an AoSoA block of `lanes` lanes: lane `l` of frame `f` holds `10 f + l`, and
    /// the right plane its negation.
    fn frame(f: usize, lanes: usize) -> (Vec<f32>, Vec<f32>) {
        let left: Vec<f32> = (0..lanes).map(|l| (10 * f + l) as f32).collect();
        let right = left.iter().map(|value| -value).collect();
        (left, right)
    }

    /// #1280 D5: a lane's samples keep their age order when they move to another lane of a bank
    /// of another width whose shared cursor sits elsewhere. A copy that ignores either cursor, or
    /// that indexes the lane by the wrong width, hands the lane other samples.
    #[test]
    fn a_shunt_lane_moves_to_a_bank_of_another_width_and_cursor() {
        let latency = 3;
        let mut from = BypassShunt::new(4, latency * 4);
        // Five frames in: the line holds frames 2, 3 and 4, and its cursor is eight words in.
        for f in 0..5 {
            let (left, right) = frame(f, 4);
            from.capture(&left, &right);
        }
        let mut into = BypassShunt::new(8, latency * 8);
        // One frame of another content moves this line's cursor off zero.
        let (left, right) = frame(9, 8);
        into.capture(&left, &right);
        assert!(into.carry_line_lane_from(5, 8, &from, 2, 4));
        // The next three frames read the carried lane's samples back, oldest first.
        for expected in [2, 3, 4] {
            let zero = vec![0.0_f32; 8];
            into.capture(&zero, &zero);
            let (left, right) = into.dry();
            assert_eq!(left[5], (10 * expected + 2) as f32);
            assert_eq!(right[5], -((10 * expected + 2) as f32));
        }
        // A line of another latency, or a lane out of range, takes nothing.
        let mut other = BypassShunt::new(8, (latency + 1) * 8);
        assert!(!other.carry_line_lane_from(5, 8, &from, 2, 4));
        assert!(!into.carry_line_lane_from(8, 8, &from, 2, 4));
        // Neither shunt has a line: nothing to move.
        let mut empty = BypassShunt::new(8, 0);
        assert!(empty.carry_line_lane_from(0, 8, &BypassShunt::new(4, 0), 0, 4));
    }

    /// The words every lane of a `lanes`-lane shunt emits over its next `latency` zero frames,
    /// oldest first, left then right, per lane. Advances the shunt.
    fn drain(shunt: &mut BypassShunt, lanes: usize, latency: usize) -> Vec<Vec<u32>> {
        let zero = vec![0.0_f32; lanes];
        let mut words = vec![Vec::new(); lanes];
        for _ in 0..latency {
            shunt.capture(&zero, &zero);
            let (left, right) = shunt.dry();
            for (lane, words) in words.iter_mut().enumerate() {
                words.push(left[lane].to_bits());
                words.push(right[lane].to_bits());
            }
        }
        words
    }

    /// #1747: a swap of two lanes' latency words, across two lines of other cursors and within
    /// one line, hands each lane the other's delayed samples in age order and leaves every other
    /// lane and the cursors alone; a shape mismatch exchanges nothing. The rack swaps only
    /// between banks of one width, so no case crosses widths.
    ///
    /// Test value: a swap that walks one line from the other's cursor, moves a cursor, or
    /// half-exchanges on a refusal.
    #[test]
    fn swapped_line_lanes_trade_their_delayed_samples() {
        const LATENCY: usize = 3;
        let fill = |lanes: usize, frames: usize, seed: usize| {
            let mut shunt = BypassShunt::new(lanes, LATENCY * lanes);
            for f in 0..frames {
                let (left, right) = frame(f + seed, lanes);
                shunt.capture(&left, &right);
            }
            shunt
        };
        // Five frames into one four-lane line (cursor 8) and six into another (cursor 0).
        let (mut a, mut b) = (fill(4, 5, 0), fill(4, 6, 20));
        let (before_a, before_b) = (
            drain(&mut fill(4, 5, 0), 4, LATENCY),
            drain(&mut fill(4, 6, 20), 4, LATENCY),
        );
        assert!(a.swap_line_lanes(2, 4, Some(&mut b), 1, 4));
        let (after_a, after_b) = (drain(&mut a, 4, LATENCY), drain(&mut b, 4, LATENCY));
        let mut expected_a = before_a.clone();
        let mut expected_b = before_b.clone();
        expected_a[2].clone_from(&before_b[1]);
        expected_b[1].clone_from(&before_a[2]);
        assert_eq!((after_a, after_b), (expected_a, expected_b));
        // Within one line.
        let mut a = fill(4, 5, 0);
        assert!(a.swap_line_lanes(0, 4, None, 3, 4));
        let mut expected = before_a.clone();
        expected.swap(0, 3);
        assert_eq!(drain(&mut a, 4, LATENCY), expected);
        // Refusals change nothing.
        let mut a = fill(4, 5, 0);
        let mut longer = BypassShunt::new(4, (LATENCY + 1) * 4);
        assert!(!a.swap_line_lanes(0, 4, Some(&mut longer), 0, 4), "latency");
        assert!(!a.swap_line_lanes(4, 4, None, 0, 4), "lane index");
        assert!(
            !a.swap_line_lanes(0, 4, None, 1, 8),
            "one line has one width"
        );
        assert_eq!(drain(&mut a, 4, LATENCY), before_a);
        assert!(BypassShunt::new(4, 0).swap_line_lanes(
            0,
            4,
            Some(&mut BypassShunt::new(4, 0)),
            0,
            4
        ));
    }
}

#[cfg(test)]
mod lane_swap_tests {
    use super::{LaneSwapRefusal, swap_bank_lanes, swap_bank_lanes_scratch_bytes};
    use crate::{
        BankProcessReport, BankWidth, EffectBankProcessBlock, PreparedNativeEffectBank, ResetKind,
        StatePayloadError, StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    };

    const SIZES: StatePayloadSizes = StatePayloadSizes {
        common_bytes: 4,
        left_bytes: 4,
        right_bytes: 4,
    };
    /// The prepared defaults bytes per lane of [`words`].
    const DEFAULTS: usize = 3;

    /// Four lanes of three words (common, left, right), and per lane `defaults` prepared default
    /// bytes, distinct per lane and per seed (`None`: undeclared). `refuse_restore_call` names
    /// the restore call (counted from 1) that refuses; every other restore commits.
    struct Words {
        lanes: [[u32; 3]; 4],
        defaults: Option<[Vec<u8>; 4]>,
        restores: u32,
        refuse_restore_call: Option<u32>,
    }

    fn words(seed: u32, refuse_restore_call: Option<u32>) -> Words {
        with_defaults(seed, refuse_restore_call, Some(DEFAULTS))
    }

    fn with_defaults(seed: u32, refuse_restore_call: Option<u32>, bytes: Option<usize>) -> Words {
        Words {
            lanes: core::array::from_fn(|lane| {
                let base = seed + 10 * lane as u32;
                [base, base + 1, base + 2]
            }),
            defaults: bytes.map(|bytes| {
                core::array::from_fn(|lane| {
                    (0..bytes)
                        .map(|byte| (seed as u8).wrapping_add((16 * lane + 5 * byte + 200) as u8))
                        .collect()
                })
            }),
            restores: 0,
            refuse_restore_call,
        }
    }

    impl PreparedNativeEffectBank for Words {
        fn reset(&mut self, _kind: ResetKind) {}
        fn reset_lane(&mut self, _lane: usize, _kind: ResetKind) {}

        fn process_bank(&mut self, _block: EffectBankProcessBlock<'_>) -> BankProcessReport {
            BankProcessReport::empty(BankWidth::Four)
        }

        fn retained_bytes(&self) -> u64 {
            self.defaults
                .as_ref()
                .map_or(0, |d| d.iter().map(crate::retained::vec).sum())
        }

        fn snapshot_track_state_payload(
            &self,
            track_index: u32,
            output: StatePayloadOutput<'_>,
        ) -> Result<(), StatePayloadError> {
            let lane = self
                .lanes
                .get(track_index as usize)
                .ok_or(StatePayloadError {
                    code: "effect.state.lane",
                })?;
            output.common.copy_from_slice(&lane[0].to_le_bytes());
            output.left.copy_from_slice(&lane[1].to_le_bytes());
            output.right.copy_from_slice(&lane[2].to_le_bytes());
            Ok(())
        }

        fn restore_track_state_payload(
            &mut self,
            track_index: u32,
            _state_layout_version: u32,
            input: StatePayloadInput<'_>,
        ) -> Result<(), StatePayloadError> {
            self.restores += 1;
            if self.refuse_restore_call == Some(self.restores) {
                return Err(StatePayloadError {
                    code: "effect.state.refused",
                });
            }
            let lane = self
                .lanes
                .get_mut(track_index as usize)
                .ok_or(StatePayloadError {
                    code: "effect.state.lane",
                })?;
            let word = |bytes: &[u8]| u32::from_le_bytes(bytes.try_into().expect("four bytes"));
            *lane = [word(input.common), word(input.left), word(input.right)];
            Ok(())
        }

        fn lane_default_bytes(&self) -> Option<usize> {
            self.defaults.as_ref().map(|defaults| defaults[0].len())
        }

        fn exchange_lane_defaults(&mut self, lane: usize, bytes: &mut [u8]) -> bool {
            let Some(own) = self.defaults.as_mut().and_then(|all| all.get_mut(lane)) else {
                return false;
            };
            if own.len() != bytes.len() {
                return false;
            }
            own.swap_with_slice(bytes);
            true
        }
    }

    /// One bank's lanes: payload words and defaults bytes.
    type Lanes = ([[u32; 3]; 4], Option<[Vec<u8>; 4]>);

    fn lanes(bank: &Words) -> Lanes {
        (bank.lanes, bank.defaults.clone())
    }

    /// #1747: every refusal of [`swap_bank_lanes`] leaves both lanes' payloads and prepared
    /// defaults as they were, including the restore refusals after the defaults rotated, which
    /// the undo must put back; and an accepted swap exchanges the payloads and defaults exactly.
    ///
    /// Test value: a swap that restores before it has both snapshots, accepts a bank with
    /// undeclared or mismatched defaults, sizes its scratch without the defaults, skips the
    /// payload or rotation undo after a restore refusal, leaves the defaults behind on an
    /// accepted swap, or names the wrong refusal.
    #[test]
    fn every_lane_swap_refusal_leaves_both_lanes_unchanged() {
        let scratch_bytes =
            swap_bank_lanes_scratch_bytes(&words(1, None), SIZES).expect("declared defaults");
        assert_eq!(scratch_bytes, 24 + DEFAULTS);
        assert_eq!(
            swap_bank_lanes_scratch_bytes(&with_defaults(1, None, None), SIZES),
            None
        );
        type Case = (Words, Option<Words>, usize, usize, usize, LaneSwapRefusal);
        let cases: [Case; 10] = [
            // The defaults are checked before the scratch, so a short scratch does not mask them.
            (
                with_defaults(1, None, None),
                Some(words(100, None)),
                1,
                2,
                0,
                LaneSwapRefusal::Defaults,
            ),
            (
                words(1, None),
                Some(with_defaults(100, None, None)),
                1,
                2,
                scratch_bytes,
                LaneSwapRefusal::Defaults,
            ),
            (
                with_defaults(1, None, None),
                None,
                1,
                2,
                scratch_bytes,
                LaneSwapRefusal::Defaults,
            ),
            (
                words(1, None),
                Some(with_defaults(100, None, Some(DEFAULTS + 1))),
                1,
                2,
                64,
                LaneSwapRefusal::Defaults,
            ),
            (
                words(1, None),
                Some(words(100, None)),
                1,
                2,
                scratch_bytes - 1,
                LaneSwapRefusal::Scratch,
            ),
            (
                words(1, None),
                Some(words(100, None)),
                usize::MAX,
                2,
                scratch_bytes,
                LaneSwapRefusal::Lane,
            ),
            (
                words(1, None),
                Some(words(100, None)),
                1,
                4,
                scratch_bytes,
                LaneSwapRefusal::Snapshot,
            ),
            // Step 2 refuses after the rotation: the rotation is undone.
            (
                words(1, Some(1)),
                Some(words(100, None)),
                1,
                2,
                scratch_bytes,
                LaneSwapRefusal::Restore,
            ),
            // Step 3 refuses: the first bank's lane and the rotation are undone.
            (
                words(1, None),
                Some(words(100, Some(1))),
                1,
                2,
                scratch_bytes,
                LaneSwapRefusal::Restore,
            ),
            // One bank whose second restore (of three: two plus the undo) refuses.
            (
                words(1, Some(2)),
                None,
                1,
                2,
                scratch_bytes,
                LaneSwapRefusal::Restore,
            ),
        ];
        for (index, (mut first, mut second, first_lane, second_lane, scratch, refusal)) in
            cases.into_iter().enumerate()
        {
            let (first_before, second_before) = (lanes(&first), second.as_ref().map(lanes));
            let mut scratch = vec![0_u8; scratch];
            let result = swap_bank_lanes(
                &mut first,
                first_lane,
                second
                    .as_mut()
                    .map(|bank| bank as &mut dyn PreparedNativeEffectBank),
                second_lane,
                1,
                SIZES,
                &mut scratch,
            );
            assert_eq!(result, Err(refusal), "case {index}");
            assert_eq!(lanes(&first), first_before, "case {index}: first bank");
            assert_eq!(
                second.as_ref().map(lanes),
                second_before,
                "case {index}: second bank"
            );
        }
        // Accepted, across two banks and within one: payloads and defaults trade exactly, and
        // nothing else moves.
        let (mut first, mut second) = (words(1, None), words(100, None));
        let mut scratch = vec![0_u8; scratch_bytes];
        assert_eq!(
            swap_bank_lanes(&mut first, 1, Some(&mut second), 2, 1, SIZES, &mut scratch),
            Ok(())
        );
        let (mut expected_first, mut expected_second) =
            (lanes(&words(1, None)), lanes(&words(100, None)));
        let (from_first, from_second) = (expected_first.clone(), expected_second.clone());
        let (Some(first_defaults), Some(second_defaults)) =
            (expected_first.1.as_mut(), expected_second.1.as_mut())
        else {
            unreachable!("declared defaults");
        };
        expected_first.0[1] = from_second.0[2];
        expected_second.0[2] = from_first.0[1];
        first_defaults[1].clone_from(&from_second.1.as_ref().expect("declared")[2]);
        second_defaults[2].clone_from(&from_first.1.as_ref().expect("declared")[1]);
        assert_eq!(
            (lanes(&first), lanes(&second)),
            (expected_first, expected_second)
        );
        let mut one = words(1, None);
        assert_eq!(
            swap_bank_lanes(&mut one, 0, None, 3, 1, SIZES, &mut scratch),
            Ok(())
        );
        let mut expected = lanes(&words(1, None));
        expected.0.swap(0, 3);
        expected.1.as_mut().expect("declared").swap(0, 3);
        assert_eq!(lanes(&one), expected);
    }
}
