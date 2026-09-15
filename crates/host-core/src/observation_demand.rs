//! Native selected-observation identity and work accounting.
//!
//! This module owns the native selected-meter preparation boundary and its checked work records.
//! The graph endpoint and meter consumers stay private to the owner, which admits bounded
//! mutations, reconciles application receipts, and filters reads by the applied generation.

use core::{
    alloc::Layout,
    mem::size_of,
    num::{NonZeroU32, NonZeroU64},
    sync::atomic::{AtomicU64, Ordering},
};

use builtins::{MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::MeterConsumer;
use engine::realtime::Consumer;
use graph::{
    GraphObservationActivationConfig, GraphObservationActivationResources,
    GraphObservationAdmissionError, GraphObservationController,
};

use crate::spectrum::{
    ControlledSpectrumCaptureCollection, ControlledSpectrumDescriptor, SPECTRUM_WINDOW_FRAMES,
    SpectrumCadence, SpectrumCapturedRecord, SpectrumChannels, SpectrumContinuousWindow,
    SpectrumTarget,
};

/// The process-local identity of one prepared observation owner.
///
/// The constructor is intentionally private.  Hosts obtain an identity from preparation, and
/// every identity is allocated once from the checked monotonic process-local counter.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObservationOwnerId(NonZeroU64);

impl ObservationOwnerId {
    /// Return the numeric owner identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// A prepared meter's owner-scoped stable identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HostMeterId {
    /// The prepared owner that created this handle.
    pub owner: ObservationOwnerId,
    /// The meter handle within that owner.
    pub handle: MeterHandle,
}

/// The capture mode selected for one host-owned spectrum demand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostSpectrumMode {
    /// Reserved for protected one-shot capture; current host admission refuses this mode.
    OneShot,
    /// Continue publishing fixed-size windows at the prepared cadence.
    Continuous,
}

/// One exact prepared spectrum entry requested by a host-owned observation controller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostSpectrumDemand {
    /// The prepared graph boundary to observe.
    pub target: SpectrumTarget,
    /// The channel planes the observer copies and publishes.
    pub channels: SpectrumChannels,
    /// The capture mode. Protected host admission currently accepts continuous mode only.
    pub mode: HostSpectrumMode,
}

/// One caller-owned continuous spectrum window with its observation generation.
#[derive(Clone, Copy, Debug)]
pub struct ObservedContinuousSpectrumWindow {
    /// The observation owner that produced the window.
    pub owner: ObservationOwnerId,
    /// The graph observation generation that produced the window.
    pub observation_generation: u64,
    /// The public target-selection epoch for this stream.
    pub selection_epoch: u64,
    /// The bounded captured window and its DSP stream metadata.
    pub window: SpectrumContinuousWindow,
}

/// One meter in the prepared, immutable observation catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedHostMeter {
    /// The owner-scoped identity of this prepared meter.
    pub id: HostMeterId,
    /// Stable compiled track identity.
    pub track_id: Box<str>,
    /// Signal boundary observed by this meter.
    pub tap: MeterTap,
    /// Numeric groups present in each emitted snapshot.
    pub metrics: MeterMetricSet,
    /// The fixed meter publication window in session frames.
    pub period_frames: NonZeroU32,
}

/// Host-owned retained storage facts for the selected observation owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostObservationResources {
    /// Exact graph activation storage, when a nonempty controlled meter catalog was bound.
    pub graph_activation: Option<GraphObservationActivationResources>,
    /// Inline bytes occupied by the containing host observation owner after the graph controller's
    /// inline bytes are excluded. The graph activation report already charges that controller.
    pub owner_inline_bytes: u64,
    /// New host-owned catalog, identity-string and fixed selection-array heap bytes.
    pub metadata_heap_bytes: u64,
    /// Largest individual new host-owned metadata allocation, including an identity string.
    pub metadata_largest_allocation_bytes: u64,
    /// Narrow fixed retained reservation for this prepared owner.
    pub reserved_bytes: u64,
}

impl HostObservationResources {
    /// No observation demand was prepared.
    pub const ZERO: Self = Self {
        graph_activation: None,
        owner_inline_bytes: 0,
        metadata_heap_bytes: 0,
        metadata_largest_allocation_bytes: 0,
        reserved_bytes: 0,
    };
}

impl Default for HostObservationResources {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Explicit native observation demand supplied during host preparation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostObservationPreparation<'a> {
    /// The complete prepared meter catalog, in caller request order.
    pub meters: &'a [crate::prepare::HostMeterRequest],
    /// Optional controlled fixed-spectrum catalog. An empty collection is normalized to absent.
    pub spectrum: Option<&'a crate::spectrum::SpectrumCaptureCollectionRequest>,
    /// Inclusive per-owner work and retained-byte limits.
    pub work_limits: ObservationWorkLimits,
    /// Graph activation population and retained-byte configuration.
    pub activation: GraphObservationActivationConfig,
}

/// One selected meter's private generation identity in a complete-set selection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MeterSelectionEntry {
    prepared_index: usize,
    generation: u64,
}

/// One optional controlled-spectrum selection carried alongside a complete meter set.
///
/// The descriptor identifies the private paired slot. Generation and selection epoch are kept as
/// separate scalar identities so later admission/read code can fence graph application and public
/// target changes independently.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpectrumSelectionEntry {
    descriptor: ControlledSpectrumDescriptor,
    channels: SpectrumChannels,
    generation: u64,
    selection_epoch: u64,
}

/// One preallocated complete-set selection array with a live prefix.
struct MeterSelection {
    entries: Box<[MeterSelectionEntry]>,
    len: usize,
    spectrum: Option<SpectrumSelectionEntry>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublicationKind {
    Ordinary,
    Removal,
}

impl PublicationKind {
    const fn pending_index(self) -> usize {
        match self {
            Self::Ordinary => 0,
            Self::Removal => 1,
        }
    }
}

enum MeterSelectionSource<'a> {
    Explicit(&'a [HostMeterId]),
    Accepted,
}

#[derive(Clone, Copy)]
enum SpectrumSelectionUpdate {
    Keep,
    Remove,
    Select {
        entry_index: usize,
        channels: SpectrumChannels,
        mode: HostSpectrumMode,
        selection_epoch: u64,
    },
}

impl MeterSelection {
    fn empty() -> Self {
        Self {
            entries: Box::new([]),
            len: 0,
            spectrum: None,
        }
    }

    fn with_capacity(capacity: usize) -> Result<Self, ObservationRefusal> {
        if capacity == 0 {
            return Ok(Self::empty());
        }
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(capacity)
            .map_err(|_| allocation_failure())?;
        entries.resize(capacity, MeterSelectionEntry::default());
        Ok(Self {
            entries: entries.into_boxed_slice(),
            len: 0,
            spectrum: None,
        })
    }

    fn entries(&self) -> &[MeterSelectionEntry] {
        &self.entries[..self.len]
    }

    fn entries_mut(&mut self) -> &mut [MeterSelectionEntry] {
        &mut self.entries[..self.len]
    }

    fn total_len(&self) -> usize {
        self.len + usize::from(self.spectrum.is_some())
    }

    fn copy_from(&mut self, source: &Self) {
        self.entries[..source.len].copy_from_slice(&source.entries[..source.len]);
        self.len = source.len;
        self.spectrum = source.spectrum;
    }
}

/// One of the two reserved graph application obligations.
struct PendingApplication {
    accepted: Option<ObservationAccepted>,
    selection: MeterSelection,
}

/// Private owner of one prepared native selected-meter catalog and its graph activation transport.
///
/// Keeping the graph controller and meter consumers private from the preparation boundary ensures
/// that no raw observer endpoint can bypass owner identity or its work ledger.
pub struct HostObservationController {
    owner: ObservationOwnerId,
    graph: Option<GraphObservationController>,
    meters: Box<[PreparedHostMeter]>,
    meter_consumers: Vec<MeterConsumer>,
    spectrum: Option<ControlledSpectrumCaptureCollection>,
    pub(crate) spectrum_cadence: Option<SpectrumCadence>,
    /// Last successfully selected public spectrum entry. The value persists through stop so a
    /// same-entry re-selection/restart keeps its selection epoch.
    spectrum_selection_entry: Option<usize>,
    /// Checked public target-selection identity, independent of graph generation and DSP epoch.
    spectrum_selection_epoch: u64,
    budget: ObservationBudget,
    accepted: MeterSelection,
    applied: MeterSelection,
    candidate: MeterSelection,
    pending: [PendingApplication; 2],
    candidate_handles: Box<[u64]>,
    terminal_closed: bool,
    /// Number of queued snapshots discarded because their observation generation is no longer
    /// live for the applied meter selection.
    ///
    /// This is control-side delivery diagnostics. The producer's lifetime drop counter remains
    /// part of [`MeterSnapshot`] and is intentionally not folded into this value.
    stale_generation_discards: u64,
    /// Number of matching snapshots superseded by a newer matching snapshot in one bounded read.
    superseded_snapshots: u64,
    /// Number of queued records discarded while retiring an applied meter selection.
    ///
    /// This is deliberately private and informational. Producer-side drop counts remain owned
    /// by the meter accumulator; this counter accounts only for control-side cleanup.
    retirement_discards: u64,
}

/// Inclusive limits for one prepared observation owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationWorkLimits {
    /// Maximum active dual-mono meter channels charged per block.
    pub maximum_active_meter_channels: u64,
    /// Maximum meter sample visits charged per block.
    pub maximum_meter_samples_per_block: u64,
    /// Maximum meter publication attempts charged per block.
    pub maximum_meter_publications_per_block: u64,
    /// Maximum meter snapshot payload bytes charged per block.
    pub maximum_meter_publication_bytes_per_block: u64,
    /// Maximum simultaneously active spectrum captures. V1 admits at most one active capture.
    pub maximum_active_spectrum_captures: u64,
    /// Maximum selected spectrum input samples charged per block.
    pub maximum_capture_input_samples_per_block: u64,
    /// Maximum selected spectrum copy samples charged per block.
    pub maximum_capture_copy_samples_per_block: u64,
    /// Maximum spectrum publication attempts charged per block.
    pub maximum_capture_publications_per_block: u64,
    /// Maximum spectrum payload bytes charged per second.
    pub maximum_capture_bytes_per_second: u64,
    /// Maximum graph transition entry visits charged per block.
    pub maximum_transition_entry_visits_per_block: u64,
    /// Maximum fixed retained observation reservation.
    pub maximum_retained_bytes: u64,
}

/// Checked worst-case work charged by one complete observation selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationWorkCost {
    /// Active dual-mono meter channels.
    pub active_meter_channels: u64,
    /// Meter sample visits per block.
    pub meter_samples_per_block: u64,
    /// Meter publication attempts per block.
    pub meter_publications_per_block: u64,
    /// Meter snapshot payload bytes per block.
    pub meter_publication_bytes_per_block: u64,
    /// Active spectrum captures. V1 admits at most one active capture.
    pub active_spectrum_captures: u64,
    /// Selected spectrum input samples per block.
    pub capture_input_samples_per_block: u64,
    /// Selected spectrum copy samples per block.
    pub capture_copy_samples_per_block: u64,
    /// Spectrum publication attempts per block.
    pub capture_publications_per_block: u64,
    /// Spectrum payload bytes per second.
    pub capture_bytes_per_second: u64,
    /// Graph transition entry visits per block.
    pub transition_entry_visits_per_block: u64,
    /// Fixed retained observation reservation.
    pub retained_bytes: u64,
}

impl ObservationWorkCost {
    /// An observation selection with no meter, spectrum, transition, or retained cost.
    pub const ZERO: Self = Self {
        active_meter_channels: 0,
        meter_samples_per_block: 0,
        meter_publications_per_block: 0,
        meter_publication_bytes_per_block: 0,
        active_spectrum_captures: 0,
        capture_input_samples_per_block: 0,
        capture_copy_samples_per_block: 0,
        capture_publications_per_block: 0,
        capture_bytes_per_second: 0,
        transition_entry_visits_per_block: 0,
        retained_bytes: 0,
    };
}

impl Default for ObservationWorkCost {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Why a checked observation owner operation was refused.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationRefusalReason {
    /// No prepared observation owner exists for the operation.
    NotPrepared,
    /// A handle belongs to another observation owner.
    WrongOwner,
    /// The requested selection exceeds a configured capacity.
    Capacity,
    /// The requested selection exceeds an inclusive work or retained-byte limit.
    WorkBudget,
    /// A bounded publication or application credit is occupied.
    Backpressure,
    /// The requested removal is not a subset of the accepted selection.
    Conflict,
    /// The owner has reached terminal closure.
    Closed,
    /// The request is not valid for this owner or tranche.
    InvalidRequest,
    /// Checked identity, cost, or allocation arithmetic overflowed.
    ArithmeticOverflow,
    /// No further observation revision can be represented.
    RevisionExhausted,
}

/// A refusal that preserves the owner's last accepted work cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationRefusal {
    /// The stable refusal category.
    pub reason: ObservationRefusalReason,
    /// The public maximum field that rejected a work request, when applicable.
    pub limit: Option<&'static str>,
    /// The checked requested value, when applicable.
    pub requested: Option<u64>,
    /// The inclusive configured maximum, when applicable.
    pub maximum: Option<u64>,
}

/// The control-side acknowledgement stored after a graph publication is accepted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationAccepted {
    /// The owner that accepted the selection.
    pub owner: ObservationOwnerId,
    /// The checked graph observation revision.
    pub revision: u64,
    /// The complete accepted selection's charged work.
    pub work: ObservationWorkCost,
}

/// The real application-boundary acknowledgement for one accepted publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationApplied {
    /// The owner whose selection became active.
    pub owner: ObservationOwnerId,
    /// The accepted graph observation revision that became active.
    pub revision: u64,
    /// The first sample of the block where it became active.
    pub first_sample: u64,
}

/// Why a meter read could not be served from the owner-scoped active selection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationReadError {
    /// The supplied meter belongs to another owner.
    WrongOwner,
    /// The owner has no prepared catalog.
    NotPrepared,
    /// The meter is not in the accepted selection.
    Inactive,
    /// The meter is accepted but its new generation is not applied yet.
    PendingApplication,
    /// The owner has reached terminal closure.
    Closed,
}

/// Why a protected continuous spectrum read could not be served from the owner-scoped active
/// selection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HostSpectrumReadError {
    /// No spectrum demand is currently accepted by this owner.
    Inactive,
    /// A new spectrum demand was accepted but has not reached the render application boundary.
    PendingApplication,
    /// The active stream has not observed a complete window yet.
    Warming,
    /// The active stream has no completed window queued.
    Pending,
    /// The render endpoint reached terminal closure.
    Closed,
    /// The active stream reported a discontinuity or failed render.
    Failed {
        /// The owner that owns the failed stream.
        owner: ObservationOwnerId,
        /// The applied graph observation generation for the failed stream.
        observation_generation: u64,
        /// The native continuous history epoch reported after the failure.
        stream_epoch: u64,
    },
    /// The bounded native result queue dropped one or more completed windows.
    Gap {
        /// The owner that owns the gapped stream.
        owner: ObservationOwnerId,
        /// The applied graph observation generation for the gapped stream.
        observation_generation: u64,
        /// The native continuous stream epoch that dropped windows.
        stream_epoch: u64,
        /// Cumulative windows dropped in that native stream epoch.
        dropped_captures: u64,
    },
}

/// A meter snapshot paired with its owner-scoped meter identity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservedMeterSnapshot {
    /// The owner-scoped meter that produced the snapshot.
    pub meter: HostMeterId,
    /// The bounded meter snapshot payload.
    pub snapshot: MeterSnapshot,
}

/// The result of asking an owner to stop all selected observation work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationStop {
    /// No graph publication was needed because the owner was already quiescent.
    Quiescent,
    /// A removal publication was accepted and awaits application.
    Pending(ObservationAccepted),
}

static LAST_OWNER_ID: AtomicU64 = AtomicU64::new(0);

/// Allocate the next owner identity for host-core preparation.
///
/// This is crate-visible so preparation can allocate exactly once per successful owner.  Tests
/// use [`allocate_owner_id_from`] with an isolated counter and never reset the production
/// counter.
pub(crate) fn allocate_owner_id() -> Result<ObservationOwnerId, ObservationRefusal> {
    allocate_owner_id_from(&LAST_OWNER_ID)
}

fn allocate_owner_id_from(counter: &AtomicU64) -> Result<ObservationOwnerId, ObservationRefusal> {
    let previous = counter
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |last| {
            last.checked_add(1)
        })
        .map_err(|_| arithmetic_overflow())?;
    let issued = previous
        .checked_add(1)
        .and_then(NonZeroU64::new)
        .ok_or_else(arithmetic_overflow)?;
    Ok(ObservationOwnerId(issued))
}

fn arithmetic_overflow() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::ArithmeticOverflow,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn invalid_request() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::InvalidRequest,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn not_prepared() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::NotPrepared,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn wrong_owner() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::WrongOwner,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn capacity() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::Capacity,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn backpressure() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::Backpressure,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn conflict() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::Conflict,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn closed() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::Closed,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn allocation_failure() -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::Capacity,
        limit: None,
        requested: None,
        maximum: None,
    }
}

fn work_budget(limit: &'static str, requested: u64, maximum: u64) -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::WorkBudget,
        limit: Some(limit),
        requested: Some(requested),
        maximum: Some(maximum),
    }
}

fn map_graph_refusal(error: GraphObservationAdmissionError) -> ObservationRefusal {
    match error {
        GraphObservationAdmissionError::UnknownHandle => not_prepared(),
        GraphObservationAdmissionError::DuplicateHandle => invalid_request(),
        GraphObservationAdmissionError::ActiveCapacity
        | GraphObservationAdmissionError::RetainedBytes => capacity(),
        GraphObservationAdmissionError::InvalidRemoval => conflict(),
        GraphObservationAdmissionError::Backpressure => backpressure(),
        GraphObservationAdmissionError::RevisionExhausted => ObservationRefusal {
            reason: ObservationRefusalReason::RevisionExhausted,
            limit: None,
            requested: None,
            maximum: None,
        },
        GraphObservationAdmissionError::OwnerClosed => closed(),
    }
}

fn map_spectrum_selection_refusal(
    error: crate::spectrum::SpectrumCaptureCollectionSelectionError,
) -> ObservationRefusal {
    match error {
        crate::spectrum::SpectrumCaptureCollectionSelectionError::UnknownEntry => not_prepared(),
        crate::spectrum::SpectrumCaptureCollectionSelectionError::Busy => backpressure(),
        crate::spectrum::SpectrumCaptureCollectionSelectionError::EpochOverflow => {
            arithmetic_overflow()
        }
    }
}

/// Project checked meter work and add the fixed preparation reservation.
fn project_meter_work(
    meter_count: u64,
    quantum_frames: u64,
    fixed_cost: ObservationWorkCost,
) -> Result<ObservationWorkCost, ObservationRefusal> {
    let active_meter_channels = meter_count.checked_mul(2).ok_or_else(arithmetic_overflow)?;
    let meter_samples_per_block = active_meter_channels
        .checked_mul(quantum_frames)
        .ok_or_else(arithmetic_overflow)?;
    let meter_snapshot_bytes =
        u64::try_from(size_of::<MeterSnapshot>()).map_err(|_| arithmetic_overflow())?;
    let meter_publication_bytes_per_block = meter_count
        .checked_mul(meter_snapshot_bytes)
        .ok_or_else(arithmetic_overflow)?;

    let projected = ObservationWorkCost {
        active_meter_channels,
        meter_samples_per_block,
        meter_publications_per_block: meter_count,
        meter_publication_bytes_per_block,
        transition_entry_visits_per_block: fixed_cost.transition_entry_visits_per_block,
        retained_bytes: fixed_cost.retained_bytes,
        ..ObservationWorkCost::ZERO
    };
    Ok(projected)
}

/// Derive the checked work charged by one active fixed-spectrum capture.
///
/// The prepared cadence fixes the launch rate and block quantum. The projection deliberately
/// counts one active producer and one publication attempt per block; paired dormant slots are
/// storage only and never multiply these fields.
pub(crate) fn project_spectrum_work(
    cadence: SpectrumCadence,
    channels: SpectrumChannels,
) -> Result<ObservationWorkCost, ObservationRefusal> {
    let quantum = u64::from(cadence.quantum_frames());
    let sample_rate = u64::from(cadence.sample_rate_hz());
    if quantum == 0 {
        return Err(arithmetic_overflow());
    }
    let window = u64::try_from(SPECTRUM_WINDOW_FRAMES).map_err(|_| arithmetic_overflow())?;
    let channel_count: u64 = match channels {
        SpectrumChannels::Left | SpectrumChannels::Right => 1,
        SpectrumChannels::Stereo => 2,
    };
    let capture_input_samples_per_block = channel_count
        .checked_mul(quantum)
        .ok_or_else(arithmetic_overflow)?;
    let capture_copy_samples_per_block = channel_count
        .checked_mul(quantum.min(window))
        .and_then(|samples| samples.checked_add(window.checked_mul(4)?))
        .ok_or_else(arithmetic_overflow)?;
    let quanta = window
        .checked_add(quantum - 1)
        .ok_or_else(arithmetic_overflow)?
        / quantum;
    let full_hop = quanta
        .checked_mul(quantum)
        .ok_or_else(arithmetic_overflow)?;
    let publications_per_second = sample_rate
        .checked_add(full_hop - 1)
        .ok_or_else(arithmetic_overflow)?
        / full_hop;
    let record_bytes =
        u64::try_from(size_of::<SpectrumCapturedRecord>()).map_err(|_| arithmetic_overflow())?;
    let capture_bytes_per_second = publications_per_second
        .checked_add(1)
        .and_then(|count| count.checked_mul(record_bytes))
        .ok_or_else(arithmetic_overflow)?;
    Ok(ObservationWorkCost {
        active_spectrum_captures: 1,
        capture_input_samples_per_block,
        capture_copy_samples_per_block,
        capture_publications_per_block: 1,
        capture_bytes_per_second,
        ..ObservationWorkCost::ZERO
    })
}

/// Private control-side work owner shared by host preparation and later admission tranches.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ObservationBudget {
    limits: ObservationWorkLimits,
    quantum_frames: u64,
    fixed_cost: ObservationWorkCost,
    accepted: ObservationWorkCost,
}

impl ObservationBudget {
    pub(crate) fn new(
        limits: ObservationWorkLimits,
        quantum_frames: NonZeroU32,
        fixed_cost: ObservationWorkCost,
    ) -> Self {
        Self {
            limits,
            quantum_frames: u64::from(quantum_frames.get()),
            fixed_cost,
            accepted: ObservationWorkCost::ZERO,
        }
    }

    fn preflight_graph_demand(
        &self,
        meter_count: usize,
        spectrum_cost: ObservationWorkCost,
    ) -> Result<ObservationWorkCost, ObservationRefusal> {
        let meter_count = u64::try_from(meter_count).map_err(|_| arithmetic_overflow())?;
        let meter = project_meter_work(meter_count, self.quantum_frames, self.fixed_cost)?;
        // Only the five spectrum work fields are admission demand. Fixed retained bytes and
        // transition visits belong to the common preparation reservation and are charged once by
        // `fixed_cost`; copying either field from a caller-supplied spectrum value would double
        // charge the owner.
        let projected = ObservationWorkCost {
            active_meter_channels: meter.active_meter_channels,
            meter_samples_per_block: meter.meter_samples_per_block,
            meter_publications_per_block: meter.meter_publications_per_block,
            meter_publication_bytes_per_block: meter.meter_publication_bytes_per_block,
            active_spectrum_captures: meter
                .active_spectrum_captures
                .checked_add(spectrum_cost.active_spectrum_captures)
                .ok_or_else(arithmetic_overflow)?,
            capture_input_samples_per_block: meter
                .capture_input_samples_per_block
                .checked_add(spectrum_cost.capture_input_samples_per_block)
                .ok_or_else(arithmetic_overflow)?,
            capture_copy_samples_per_block: meter
                .capture_copy_samples_per_block
                .checked_add(spectrum_cost.capture_copy_samples_per_block)
                .ok_or_else(arithmetic_overflow)?,
            capture_publications_per_block: meter
                .capture_publications_per_block
                .checked_add(spectrum_cost.capture_publications_per_block)
                .ok_or_else(arithmetic_overflow)?,
            capture_bytes_per_second: meter
                .capture_bytes_per_second
                .checked_add(spectrum_cost.capture_bytes_per_second)
                .ok_or_else(arithmetic_overflow)?,
            transition_entry_visits_per_block: meter.transition_entry_visits_per_block,
            retained_bytes: meter.retained_bytes,
        };
        self.check_limits(projected)
    }

    fn check_limits(
        &self,
        projected: ObservationWorkCost,
    ) -> Result<ObservationWorkCost, ObservationRefusal> {
        let checks = [
            (
                projected.active_meter_channels,
                self.limits.maximum_active_meter_channels,
                "maximum_active_meter_channels",
            ),
            (
                projected.meter_samples_per_block,
                self.limits.maximum_meter_samples_per_block,
                "maximum_meter_samples_per_block",
            ),
            (
                projected.meter_publications_per_block,
                self.limits.maximum_meter_publications_per_block,
                "maximum_meter_publications_per_block",
            ),
            (
                projected.meter_publication_bytes_per_block,
                self.limits.maximum_meter_publication_bytes_per_block,
                "maximum_meter_publication_bytes_per_block",
            ),
            (
                projected.active_spectrum_captures,
                self.limits.maximum_active_spectrum_captures,
                "maximum_active_spectrum_captures",
            ),
            (
                projected.capture_input_samples_per_block,
                self.limits.maximum_capture_input_samples_per_block,
                "maximum_capture_input_samples_per_block",
            ),
            (
                projected.capture_copy_samples_per_block,
                self.limits.maximum_capture_copy_samples_per_block,
                "maximum_capture_copy_samples_per_block",
            ),
            (
                projected.capture_publications_per_block,
                self.limits.maximum_capture_publications_per_block,
                "maximum_capture_publications_per_block",
            ),
            (
                projected.capture_bytes_per_second,
                self.limits.maximum_capture_bytes_per_second,
                "maximum_capture_bytes_per_second",
            ),
            (
                projected.transition_entry_visits_per_block,
                self.limits.maximum_transition_entry_visits_per_block,
                "maximum_transition_entry_visits_per_block",
            ),
            (
                projected.retained_bytes,
                self.limits.maximum_retained_bytes,
                "maximum_retained_bytes",
            ),
        ];
        for (requested, maximum, limit) in checks {
            if requested > maximum {
                return Err(work_budget(limit, requested, maximum));
            }
        }
        Ok(projected)
    }

    /// Commit a cost already checked by [`Self::preflight_graph_demand`].
    ///
    /// Publication is the fallible boundary.  Once the graph publication succeeds, assigning
    /// the prevalidated complete-set cost is infallible and does not allocate.
    fn commit_graph_demand(&mut self, cost: ObservationWorkCost) {
        debug_assert!(self.check_limits(cost).is_ok());
        self.accepted = cost;
    }

    fn accepted(&self) -> ObservationWorkCost {
        self.accepted
    }
}

impl HostObservationController {
    /// Construct the owner after all graph, work and allocation caps have been checked.
    // Preparation transfers independently owned, prevalidated resources without another wrapper.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prepare(
        owner: ObservationOwnerId,
        graph: Option<GraphObservationController>,
        requests: &[crate::prepare::HostMeterRequest],
        period_frames: Option<NonZeroU32>,
        meter_consumers: Vec<MeterConsumer>,
        spectrum: Option<ControlledSpectrumCaptureCollection>,
        spectrum_cadence: Option<SpectrumCadence>,
        budget: ObservationBudget,
        meter_capacity: usize,
        handle_capacity: usize,
    ) -> Result<Self, ObservationRefusal> {
        if !requests.is_empty() && period_frames.is_none() {
            return Err(invalid_request());
        }
        let mut budget = budget;
        let initial = budget.preflight_graph_demand(0, ObservationWorkCost::ZERO)?;
        budget.commit_graph_demand(initial);

        let mut meters = Vec::new();
        meters
            .try_reserve_exact(requests.len())
            .map_err(|_| allocation_failure())?;
        for (index, request) in requests.iter().enumerate() {
            let handle = u64::try_from(index)
                .ok()
                .and_then(|index| index.checked_add(1))
                .and_then(NonZeroU64::new)
                .ok_or_else(arithmetic_overflow)?;
            meters.push(PreparedHostMeter {
                id: HostMeterId {
                    owner,
                    handle: MeterHandle(handle),
                },
                track_id: request.track_id.clone(),
                tap: request.tap,
                metrics: request.metrics,
                period_frames: period_frames.ok_or_else(invalid_request)?,
            });
        }

        let accepted = MeterSelection::with_capacity(meter_capacity)?;
        let applied = MeterSelection::with_capacity(meter_capacity)?;
        let candidate = MeterSelection::with_capacity(meter_capacity)?;
        let pending_ordinary = PendingApplication {
            accepted: None,
            selection: MeterSelection::with_capacity(meter_capacity)?,
        };
        let pending_removal = PendingApplication {
            accepted: None,
            selection: MeterSelection::with_capacity(meter_capacity)?,
        };
        let mut candidate_handles = Vec::new();
        candidate_handles
            .try_reserve_exact(handle_capacity)
            .map_err(|_| allocation_failure())?;
        candidate_handles.resize(handle_capacity, 0);

        Ok(Self {
            owner,
            graph,
            meters: meters.into_boxed_slice(),
            meter_consumers,
            spectrum,
            spectrum_cadence,
            spectrum_selection_entry: None,
            spectrum_selection_epoch: 0,
            budget,
            accepted,
            applied,
            candidate,
            pending: [pending_ordinary, pending_removal],
            candidate_handles: candidate_handles.into_boxed_slice(),
            terminal_closed: false,
            stale_generation_discards: 0,
            superseded_snapshots: 0,
            retirement_discards: 0,
        })
    }

    /// Return this owner's process-local identity.
    #[must_use]
    pub const fn owner(&self) -> ObservationOwnerId {
        self.owner
    }

    /// Return the immutable prepared meter catalog in caller request order.
    #[must_use]
    pub fn meters(&self) -> &[PreparedHostMeter] {
        &self.meters
    }

    /// Return the currently accepted observation work cost.
    #[must_use]
    pub fn work(&self) -> ObservationWorkCost {
        self.budget.accepted()
    }

    /// Whether the graph endpoint has reached terminal closure.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.terminal_closed
            || self
                .graph
                .as_ref()
                .is_some_and(GraphObservationController::is_closed)
    }

    /// Replace the complete selected-meter set with an ordinary graph publication.
    ///
    /// The candidate is built entirely in the preallocated scratch storage. The graph endpoint is
    /// the final fallible operation; after it accepts, all owner-side assignments are bounded
    /// copies into storage reserved during preparation.
    #[allow(clippy::result_large_err)]
    pub fn replace_meters(
        &mut self,
        meters: &[HostMeterId],
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        self.publish_candidate(
            MeterSelectionSource::Explicit(meters),
            PublicationKind::Ordinary,
            SpectrumSelectionUpdate::Keep,
        )
    }

    /// Remove meters to the complete selected subset using the reserved removal publication.
    ///
    /// A removal may pass one outstanding ordinary publication, but cannot overlap another
    /// removal. It is validated against the latest accepted set rather than the last applied set,
    /// so the two graph credits remain useful for a bounded ordinary-then-removal sequence. The
    /// accepted spectrum selection, when present, is retained.
    #[allow(clippy::result_large_err)]
    pub fn remove_meters_to(
        &mut self,
        remaining: &[HostMeterId],
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        self.publish_candidate(
            MeterSelectionSource::Explicit(remaining),
            PublicationKind::Removal,
            SpectrumSelectionUpdate::Keep,
        )
    }

    /// Replace the protected continuous spectrum selection while retaining the accepted meters.
    ///
    /// One-shot demands are deliberately refused after the terminal-owner check and before any
    /// target lookup, staging or graph publication. This keeps the protected one-shot successor
    /// out of the continuous-first slice without changing the legacy permanent capture API.
    #[allow(clippy::result_large_err)]
    pub fn replace_spectrum(
        &mut self,
        demand: &HostSpectrumDemand,
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        if self.observe_terminal_closure() {
            return Err(closed());
        }
        if demand.mode == HostSpectrumMode::OneShot {
            return Err(invalid_request());
        }
        if self.graph.is_none() {
            return Err(not_prepared());
        }
        let spectrum = self.spectrum.as_ref().ok_or_else(not_prepared)?;
        let entry_index = spectrum
            .prepared_entry_index(&demand.target, demand.channels)
            .map_err(map_spectrum_selection_refusal)?;
        let selection_epoch = self.reserve_selection_epoch(entry_index)?;
        // The target lookup, complete-set union, work projection and publication-credit checks
        // all happen in `publish_candidate` before its Free alternate slot is staged.
        self.publish_candidate(
            MeterSelectionSource::Accepted,
            PublicationKind::Ordinary,
            SpectrumSelectionUpdate::Select {
                entry_index,
                channels: demand.channels,
                mode: demand.mode,
                selection_epoch,
            },
        )
    }

    /// Restart the accepted continuous spectrum target on a fresh graph generation.
    #[allow(clippy::result_large_err)]
    pub fn restart_spectrum(&mut self) -> Result<ObservationAccepted, ObservationRefusal> {
        self.ensure_graph_live()?;
        let accepted = self.accepted.spectrum.ok_or_else(conflict)?;
        if accepted.descriptor.mode != HostSpectrumMode::Continuous {
            return Err(invalid_request());
        }
        let channels = self
            .spectrum
            .as_ref()
            .and_then(|spectrum| spectrum.entry_channels(accepted.descriptor.entry_index))
            .ok_or_else(not_prepared)?;
        self.publish_candidate(
            MeterSelectionSource::Accepted,
            PublicationKind::Ordinary,
            SpectrumSelectionUpdate::Select {
                entry_index: accepted.descriptor.entry_index,
                channels,
                mode: HostSpectrumMode::Continuous,
                selection_epoch: accepted.selection_epoch,
            },
        )
    }

    /// Stop only the spectrum family, retaining the accepted meter set.
    ///
    /// This family-level stop does not wait for unrelated meter-only receipts. If a spectrum stop
    /// is already pending, its exact acknowledgement is returned without spending a revision.
    #[allow(clippy::result_large_err)]
    pub fn stop_spectrum(&mut self) -> Result<ObservationStop, ObservationRefusal> {
        if self.graph.is_none() {
            return Ok(ObservationStop::Quiescent);
        }
        if self.observe_terminal_closure() {
            return Err(closed());
        }

        if self.accepted.spectrum.is_none() {
            let applied_has_spectrum = self.applied.spectrum.is_some();
            let pending_has_spectrum = self
                .pending
                .iter()
                .any(|pending| pending.accepted.is_some() && pending.selection.spectrum.is_some());
            if !applied_has_spectrum && !pending_has_spectrum {
                return Ok(ObservationStop::Quiescent);
            }
            if let Some(accepted) = self.pending_without_spectrum() {
                return Ok(ObservationStop::Pending(accepted));
            }
            // An accepted/applied spectrum with no pending complete removal is not expected after
            // a successful admission. Preserve the refusal rather than fabricating a receipt.
            return Err(backpressure());
        }

        if let Some(accepted) = self.pending_without_spectrum() {
            return Ok(ObservationStop::Pending(accepted));
        }

        self.publish_candidate(
            MeterSelectionSource::Accepted,
            PublicationKind::Removal,
            SpectrumSelectionUpdate::Remove,
        )
        .map(ObservationStop::Pending)
    }

    /// Reconcile at most one real graph application receipt.
    ///
    /// The graph controller is the sole source of application boundaries. A receipt is matched
    /// to exactly one of the two preallocated pending records; no receipt is synthesized or
    /// coalesced when ordinary and removal publications apply at the same sample.
    pub fn try_applied(&mut self) -> Option<ObservationApplied> {
        if self.observe_terminal_closure() {
            return None;
        }
        let receipt = self.graph.as_mut()?.try_applied()?;
        let pending_index = self
            .pending_index_for_revision(receipt.revision)
            .expect("graph receipt must match one pending observation record");
        self.pending[pending_index]
            .accepted
            .take()
            .expect("matching pending observation record must retain its acknowledgement");

        if let Some(spectrum) = self.spectrum.as_mut() {
            debug_assert!(
                self.spectrum_cadence
                    .is_some_and(|cadence| cadence.quantum_frames() != 0),
                "controlled spectrum owners retain a valid prepared cadence"
            );
            spectrum.reconcile_applied(receipt.revision);
        }
        self.cleanup_retired_readers(pending_index);
        core::mem::swap(
            &mut self.applied,
            &mut self.pending[pending_index].selection,
        );
        self.pending[pending_index].selection.len = 0;
        self.pending[pending_index].selection.spectrum = None;
        self.candidate.spectrum = None;

        Some(ObservationApplied {
            owner: self.owner,
            revision: receipt.revision,
            first_sample: receipt.first_sample,
        })
    }

    /// Read the newest available snapshot for one applied live meter generation.
    ///
    /// State validation is deliberately complete before the consumer is borrowed: a refused read
    /// cannot drain a queue. Once the meter is known to be live, the queue's availability is
    /// frozen exactly once and capped by its configured capacity. A producer may publish later,
    /// but those later records belong to a subsequent read. Matching records are coalesced to the
    /// newest bounded record; records from an older or otherwise different activation generation
    /// are discarded and counted separately.
    pub fn try_read_meter(
        &mut self,
        meter: HostMeterId,
    ) -> Result<Option<ObservedMeterSnapshot>, ObservationReadError> {
        // An inert owner has no graph endpoint. Check this before closure so that its explicit
        // no-op lifetime remains distinguishable from a renderer that actually closed.
        if self.graph.is_none() {
            return Err(ObservationReadError::NotPrepared);
        }
        if self.observe_terminal_closure() {
            return Err(ObservationReadError::Closed);
        }
        if meter.owner != self.owner {
            return Err(ObservationReadError::WrongOwner);
        }
        let Some(prepared_index) = self
            .meters
            .iter()
            .position(|prepared| prepared.id.handle == meter.handle)
        else {
            return Err(ObservationReadError::NotPrepared);
        };

        let Some(accepted) = self
            .accepted
            .entries()
            .iter()
            .find(|entry| entry.prepared_index == prepared_index)
        else {
            return Err(ObservationReadError::Inactive);
        };
        let Some(applied) = self
            .applied
            .entries()
            .iter()
            .find(|entry| entry.prepared_index == prepared_index)
        else {
            return Err(ObservationReadError::PendingApplication);
        };
        if applied.generation != accepted.generation {
            return Err(ObservationReadError::PendingApplication);
        }

        let (newest, stale_count, superseded_count) = read_newest_meter_generation(
            &mut self.meter_consumers[prepared_index].consumer,
            applied.generation,
        );
        self.stale_generation_discards = self.stale_generation_discards.saturating_add(stale_count);
        self.superseded_snapshots = self.superseded_snapshots.saturating_add(superseded_count);
        Ok(newest.map(|snapshot| ObservedMeterSnapshot { meter, snapshot }))
    }

    /// Read one generation-fenced continuous spectrum window from the applied selection.
    ///
    /// The owner validates terminal, accepted and applied state before borrowing the private
    /// capture queue. Once that state is live, the queue availability is frozen at the read entry
    /// and capped at one native record. A stale record is rejected before it is projected into the
    /// public window, and it consumes the same single pop budget without chasing a producer refill.
    /// Reads only advance the private capture consumer; they never publish a graph mutation or
    /// consume an application receipt.
    pub fn try_read_continuous_spectrum(
        &mut self,
    ) -> Result<ObservedContinuousSpectrumWindow, HostSpectrumReadError> {
        if self.observe_terminal_closure() {
            return Err(HostSpectrumReadError::Closed);
        }
        let Some(accepted) = self.accepted.spectrum else {
            return Err(HostSpectrumReadError::Inactive);
        };
        if accepted.descriptor.mode != HostSpectrumMode::Continuous {
            return Err(HostSpectrumReadError::Inactive);
        }
        let Some(applied) = self.applied.spectrum else {
            return Err(HostSpectrumReadError::PendingApplication);
        };
        if applied.descriptor != accepted.descriptor || applied.generation != accepted.generation {
            return Err(HostSpectrumReadError::PendingApplication);
        }

        let record = {
            let spectrum = self
                .spectrum
                .as_mut()
                .ok_or(HostSpectrumReadError::Inactive)?;
            let available = spectrum
                .continuous_available_at_entry(accepted.descriptor)
                .ok_or(HostSpectrumReadError::Inactive)?;
            spectrum
                .try_read_continuous_record(accepted.descriptor, Some(available))
                .map_err(|error| match error {
                    crate::spectrum::SpectrumContinuousReadError::NotActive => {
                        HostSpectrumReadError::Inactive
                    }
                    crate::spectrum::SpectrumContinuousReadError::Warming => {
                        HostSpectrumReadError::Warming
                    }
                    crate::spectrum::SpectrumContinuousReadError::Pending => {
                        HostSpectrumReadError::Pending
                    }
                    crate::spectrum::SpectrumContinuousReadError::Failed { stream_epoch } => {
                        HostSpectrumReadError::Failed {
                            owner: self.owner,
                            observation_generation: applied.generation,
                            stream_epoch,
                        }
                    }
                    crate::spectrum::SpectrumContinuousReadError::Gap {
                        stream_epoch,
                        dropped_captures,
                    } => HostSpectrumReadError::Gap {
                        owner: self.owner,
                        observation_generation: applied.generation,
                        stream_epoch,
                        dropped_captures,
                    },
                })?
        };

        // Keep the generation check ahead of `continuous_window`: a record from a retired graph
        // activation must never be projected as if it belonged to the current public stream.
        if record.observation_generation() != applied.generation {
            return Err(HostSpectrumReadError::Pending);
        }
        Ok(ObservedContinuousSpectrumWindow {
            owner: self.owner,
            observation_generation: applied.generation,
            selection_epoch: accepted.selection_epoch,
            window: record.continuous_window(),
        })
    }

    /// Request removal of every selected meter and spectrum family, or report quiescence.
    ///
    /// Stop never polls the graph receipt queue. If an empty publication is already accepted,
    /// the same stored acknowledgement is returned so repeating stop cannot spend a revision.
    #[allow(clippy::result_large_err)]
    pub fn stop_all(&mut self) -> Result<ObservationStop, ObservationRefusal> {
        if self.observe_terminal_closure() {
            return Err(closed());
        }
        if self.graph.is_none() {
            return Ok(ObservationStop::Quiescent);
        }

        if self.accepted.total_len() == 0 {
            if let Some(accepted) = self.pending_empty_selection() {
                return Ok(ObservationStop::Pending(accepted));
            }
            if self.applied.total_len() == 0
                && self
                    .pending
                    .iter()
                    .all(|pending| pending.accepted.is_none())
            {
                return Ok(ObservationStop::Quiescent);
            }
        }

        self.publish_candidate(
            MeterSelectionSource::Explicit(&[]),
            PublicationKind::Removal,
            SpectrumSelectionUpdate::Remove,
        )
        .map(ObservationStop::Pending)
    }

    #[allow(clippy::result_large_err)]
    fn publish_candidate(
        &mut self,
        source: MeterSelectionSource<'_>,
        kind: PublicationKind,
        spectrum_update: SpectrumSelectionUpdate,
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        self.ensure_graph_live()?;

        let requested_len = match source {
            MeterSelectionSource::Explicit(requested) => {
                if requested.iter().any(|meter| meter.owner != self.owner) {
                    return Err(wrong_owner());
                }
                if requested.len() > self.candidate.entries.len() {
                    return Err(capacity());
                }
                for (index, meter) in requested.iter().enumerate() {
                    let prepared_index = self
                        .meters
                        .iter()
                        .position(|prepared| prepared.id.handle == meter.handle)
                        .ok_or_else(not_prepared)?;
                    if requested[..index]
                        .iter()
                        .any(|previous| previous.handle == meter.handle)
                    {
                        return Err(invalid_request());
                    }
                    self.candidate.entries[index] = MeterSelectionEntry {
                        prepared_index,
                        generation: self
                            .accepted
                            .entries()
                            .iter()
                            .find(|entry| entry.prepared_index == prepared_index)
                            .map_or(0, |entry| entry.generation),
                    };
                }
                requested.len()
            }
            MeterSelectionSource::Accepted => {
                let count = self.accepted.len;
                if count > self.candidate.entries.len() {
                    return Err(capacity());
                }
                self.candidate.entries[..count].copy_from_slice(self.accepted.entries());
                count
            }
        };
        self.candidate.len = requested_len;

        if matches!(kind, PublicationKind::Removal)
            && self.candidate.entries().iter().any(|entry| {
                !self
                    .accepted
                    .entries()
                    .iter()
                    .any(|accepted| accepted.prepared_index == entry.prepared_index)
            })
        {
            return Err(conflict());
        }

        let mut staged_candidate = None;
        let spectrum_cost = match spectrum_update {
            SpectrumSelectionUpdate::Keep => {
                self.candidate.spectrum = self.accepted.spectrum;
                self.candidate
                    .spectrum
                    .map_or(Ok(ObservationWorkCost::ZERO), |selection| {
                        self.spectrum_cost(selection.channels)
                    })?
            }
            SpectrumSelectionUpdate::Remove => {
                self.candidate.spectrum = None;
                ObservationWorkCost::ZERO
            }
            SpectrumSelectionUpdate::Select { channels, .. } => {
                self.candidate.spectrum = None;
                // This checked projection proves the complete union and work budget before the
                // Free alternate slot is staged below.
                self.spectrum_cost(channels)?
            }
        };
        let spectrum_present = !matches!(spectrum_update, SpectrumSelectionUpdate::Remove)
            && (matches!(spectrum_update, SpectrumSelectionUpdate::Select { .. })
                || self.candidate.spectrum.is_some());
        let candidate_handle_len = self
            .candidate
            .len
            .checked_add(usize::from(spectrum_present))
            .ok_or_else(arithmetic_overflow)?;
        if candidate_handle_len > self.candidate_handles.len() {
            return Err(capacity());
        }
        let work = self
            .budget
            .preflight_graph_demand(self.candidate.len, spectrum_cost)?;

        // Ordinary demand consumes the ordinary credit and is serialized against the reserved
        // removal record. Removal demand consumes only the removal credit, allowing it to follow
        // one ordinary publication before either has reached an application boundary.
        let pending_index = kind.pending_index();
        if self.pending[pending_index].accepted.is_some()
            || (matches!(kind, PublicationKind::Ordinary)
                && self.pending[PublicationKind::Removal.pending_index()]
                    .accepted
                    .is_some())
        {
            return Err(backpressure());
        }

        if let SpectrumSelectionUpdate::Select {
            entry_index,
            channels,
            mode,
            selection_epoch,
        } = spectrum_update
        {
            let cadence = self.spectrum_cadence.ok_or_else(not_prepared)?;
            let spectrum = self.spectrum.as_mut().ok_or_else(not_prepared)?;
            let staged = spectrum
                .stage_prepared_entry(entry_index, mode, cadence)
                .map_err(map_spectrum_selection_refusal)?;
            let descriptor = staged.descriptor();
            self.candidate.spectrum = Some(SpectrumSelectionEntry {
                descriptor,
                generation: 0,
                selection_epoch,
                channels,
            });
            staged_candidate = Some(staged);
        }

        self.candidate
            .entries_mut()
            .sort_unstable_by_key(|entry| self.meters[entry.prepared_index].id.handle.0.get());
        for (index, entry) in self.candidate.entries().iter().enumerate() {
            self.candidate_handles[index] = self.meters[entry.prepared_index].id.handle.0.get();
        }
        if let Some(spectrum) = self.candidate.spectrum {
            self.candidate_handles[self.candidate.len] = spectrum.descriptor.observer_handle;
        }

        // `graph.replace`/`remove_to` is deliberately the last fallible operation. The candidate
        // storage and handle slice were both prepared above. A graph refusal returns only the
        // newly staged Free slot; all accepted/applied state remains untouched.
        let graph_result = {
            let graph = self.graph.as_mut().expect("graph checked above");
            match kind {
                PublicationKind::Ordinary => {
                    graph.replace(&self.candidate_handles[..self.candidate.total_len()])
                }
                PublicationKind::Removal => {
                    graph.remove_to(&self.candidate_handles[..self.candidate.total_len()])
                }
            }
        };
        let graph_accepted = match graph_result {
            Ok(accepted) => accepted,
            Err(error) => {
                if let Some(staged) = staged_candidate {
                    self.spectrum
                        .as_mut()
                        .expect("staged spectrum has a collection")
                        .cancel_candidate(staged);
                }
                return Err(map_graph_refusal(error));
            }
        };

        let revision = graph_accepted.revision;
        for entry in self.candidate.entries_mut() {
            if entry.generation == 0 {
                entry.generation = revision;
            }
        }
        if let Some(spectrum_selection) = self.candidate.spectrum.as_mut()
            && spectrum_selection.generation == 0
        {
            spectrum_selection.generation = revision;
        }
        if let Some(staged) = staged_candidate {
            self.spectrum
                .as_mut()
                .expect("staged spectrum has a collection")
                .commit_candidate(staged, revision);
            self.spectrum_selection_entry = self
                .candidate
                .spectrum
                .map(|selection| selection.descriptor.entry_index);
            self.spectrum_selection_epoch = self
                .candidate
                .spectrum
                .map_or(self.spectrum_selection_epoch, |selection| {
                    selection.selection_epoch
                });
        } else if matches!(spectrum_update, SpectrumSelectionUpdate::Remove)
            && self.accepted.spectrum.is_some()
        {
            self.spectrum
                .as_mut()
                .expect("accepted spectrum has a collection")
                .commit_removal(revision);
        }
        let accepted = ObservationAccepted {
            owner: self.owner,
            revision,
            work,
        };
        let pending = &mut self.pending[pending_index];
        pending.selection.copy_from(&self.candidate);
        pending.accepted = Some(accepted);
        self.accepted.copy_from(&self.candidate);
        self.budget.commit_graph_demand(work);
        Ok(accepted)
    }

    fn ensure_graph_live(&mut self) -> Result<(), ObservationRefusal> {
        // An empty prepared catalog has no graph endpoint. Keep this distinction from a terminal
        // endpoint: an inert owner is live but has nothing to publish into.
        if self.graph.is_none() {
            return Err(not_prepared());
        }
        if self.observe_terminal_closure() {
            return Err(closed());
        }
        Ok(())
    }

    fn reserve_selection_epoch(&self, entry_index: usize) -> Result<u64, ObservationRefusal> {
        if self.spectrum_selection_entry == Some(entry_index) {
            Ok(self.spectrum_selection_epoch)
        } else {
            self.spectrum_selection_epoch
                .checked_add(1)
                .ok_or_else(arithmetic_overflow)
        }
    }

    fn spectrum_cost(
        &self,
        channels: SpectrumChannels,
    ) -> Result<ObservationWorkCost, ObservationRefusal> {
        let cadence = self.spectrum_cadence.ok_or_else(not_prepared)?;
        project_spectrum_work(cadence, channels)
    }

    fn pending_without_spectrum(&self) -> Option<ObservationAccepted> {
        self.pending
            .iter()
            .filter(|pending| pending.selection.spectrum.is_none())
            .filter_map(|pending| pending.accepted)
            .max_by_key(|accepted| accepted.revision)
    }

    fn pending_empty_selection(&self) -> Option<ObservationAccepted> {
        self.pending
            .iter()
            .filter(|pending| pending.selection.total_len() == 0)
            .filter_map(|pending| pending.accepted)
            .max_by_key(|accepted| accepted.revision)
    }

    fn observe_terminal_closure(&mut self) -> bool {
        if self.terminal_closed {
            return true;
        }
        let closed = self
            .graph
            .as_ref()
            .is_some_and(GraphObservationController::is_closed);
        if closed {
            self.terminal_closed = true;
            for pending in &mut self.pending {
                pending.accepted = None;
                pending.selection.len = 0;
                pending.selection.spectrum = None;
            }
        }
        closed
    }

    fn pending_index_for_revision(&self, revision: u64) -> Option<usize> {
        let mut found = None;
        for (index, pending) in self.pending.iter().enumerate() {
            if pending
                .accepted
                .is_some_and(|accepted| accepted.revision == revision)
            {
                if found.is_some() {
                    debug_assert!(false, "one graph revision matched two pending records");
                    return None;
                }
                found = Some(index);
            }
        }
        found
    }

    fn cleanup_retired_readers(&mut self, incoming_index: usize) {
        let incoming = self.pending[incoming_index].selection.entries();
        let old = self.applied.entries();
        for old_entry in old {
            if incoming
                .iter()
                .any(|entry| entry.prepared_index == old_entry.prepared_index)
            {
                continue;
            }
            let consumer = &mut self.meter_consumers[old_entry.prepared_index];
            let available = consumer
                .consumer
                .available_at_entry()
                .min(consumer.consumer.capacity());
            for _ in 0..available {
                if consumer.consumer.try_pop().is_ok() {
                    self.retirement_discards = self.retirement_discards.saturating_add(1);
                }
            }
        }
    }
}

/// Consume only the queue records visible at one read entry and keep the newest matching
/// generation. This is shared with the owner read path so the bounded/stale rules can be checked
/// with a real [`MeterAccumulator`](builtins::MeterAccumulator) producer in a private unit test.
fn read_newest_meter_generation(
    consumer: &mut Consumer<MeterSnapshot>,
    generation: u64,
) -> (Option<MeterSnapshot>, u64, u64) {
    // Freeze this once. `try_pop` is bounded by the count visible at entry, so a producer
    // replenishing the queue cannot turn one read into an unbounded drain.
    let available = consumer.available_at_entry().min(consumer.capacity());
    let mut newest = None;
    let mut stale_count = 0_u64;
    let mut superseded_count = 0_u64;
    for _ in 0..available {
        let Ok(snapshot) = consumer.try_pop() else {
            break;
        };
        if snapshot.observation_generation != generation {
            stale_count = stale_count.saturating_add(1);
            continue;
        }
        if newest.is_some() {
            superseded_count = superseded_count.saturating_add(1);
        }
        newest = Some(snapshot);
    }
    (newest, stale_count, superseded_count)
}

/// Derive the new host-owned metadata heap rows from the concrete owner layout.
pub(crate) fn observation_metadata_resources(
    catalog_len: usize,
    track_id_lengths: impl IntoIterator<Item = usize>,
    meter_capacity: usize,
    handle_capacity: usize,
) -> Result<(u64, u64), ObservationRefusal> {
    let mut total = 0_u64;
    let mut largest = 0_u64;
    let mut add = |bytes: u64| -> Result<(), ObservationRefusal> {
        total = total.checked_add(bytes).ok_or_else(arithmetic_overflow)?;
        largest = largest.max(bytes);
        Ok(())
    };

    add(layout_bytes::<PreparedHostMeter>(catalog_len)?)?;
    for length in track_id_lengths {
        add(layout_bytes::<u8>(length)?)?;
    }
    for _ in 0..5 {
        add(layout_bytes::<MeterSelectionEntry>(meter_capacity)?)?;
    }
    add(layout_bytes::<u64>(handle_capacity)?)?;
    Ok((total, largest))
}

/// Derive the additive host rows and narrow retained reservation for a prepared owner.
pub(crate) fn observation_resources(
    catalog_len: usize,
    track_id_lengths: impl IntoIterator<Item = usize>,
    meter_capacity: usize,
    handle_capacity: usize,
    graph_activation: Option<GraphObservationActivationResources>,
    builtin_retained_bytes: u64,
    spectrum_capture_retained_bytes: u64,
) -> Result<HostObservationResources, ObservationRefusal> {
    let (metadata_heap_bytes, metadata_largest_allocation_bytes) = observation_metadata_resources(
        catalog_len,
        track_id_lengths,
        meter_capacity,
        handle_capacity,
    )?;
    let owner_inline_bytes = u64::try_from(size_of::<HostObservationController>())
        .map_err(|_| arithmetic_overflow())?
        .checked_sub(if graph_activation.is_some() {
            u64::try_from(size_of::<GraphObservationController>())
                .map_err(|_| arithmetic_overflow())?
        } else {
            0
        })
        .ok_or_else(arithmetic_overflow)?;
    let reserved_bytes = match graph_activation {
        None => 0,
        Some(activation) => builtin_retained_bytes
            .checked_add(spectrum_capture_retained_bytes)
            .and_then(|bytes| bytes.checked_add(activation.retained_bytes))
            .and_then(|bytes| bytes.checked_add(owner_inline_bytes))
            .and_then(|bytes| bytes.checked_add(metadata_heap_bytes))
            .ok_or_else(arithmetic_overflow)?,
    };
    Ok(HostObservationResources {
        graph_activation,
        owner_inline_bytes,
        metadata_heap_bytes,
        metadata_largest_allocation_bytes,
        reserved_bytes,
    })
}

/// Return the additive bytes charged to the common graph/model admission expression.
pub(crate) fn observation_graph_model_addition(
    resources: HostObservationResources,
) -> Result<u64, ObservationRefusal> {
    let activation_without_runtime = match resources.graph_activation {
        None => 0,
        Some(activation) => activation
            .retained_bytes
            .checked_sub(activation.runtime_state_bytes)
            .ok_or_else(arithmetic_overflow)?,
    };
    resources
        .owner_inline_bytes
        .checked_add(resources.metadata_heap_bytes)
        .and_then(|bytes| bytes.checked_add(activation_without_runtime))
        .ok_or_else(arithmetic_overflow)
}

/// Compute a checked standalone layout size in address-free bytes.
fn layout_bytes<T>(length: usize) -> Result<u64, ObservationRefusal> {
    let layout = Layout::array::<T>(length).map_err(|_| arithmetic_overflow())?;
    u64::try_from(layout.size()).map_err(|_| arithmetic_overflow())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits_with(
        active_meter_channels: u64,
        meter_samples_per_block: u64,
        meter_publications_per_block: u64,
        meter_publication_bytes_per_block: u64,
        transition_entry_visits_per_block: u64,
        retained_bytes: u64,
    ) -> ObservationWorkLimits {
        ObservationWorkLimits {
            maximum_active_meter_channels: active_meter_channels,
            maximum_meter_samples_per_block: meter_samples_per_block,
            maximum_meter_publications_per_block: meter_publications_per_block,
            maximum_meter_publication_bytes_per_block: meter_publication_bytes_per_block,
            maximum_active_spectrum_captures: u64::MAX,
            maximum_capture_input_samples_per_block: u64::MAX,
            maximum_capture_copy_samples_per_block: u64::MAX,
            maximum_capture_publications_per_block: u64::MAX,
            maximum_capture_bytes_per_second: u64::MAX,
            maximum_transition_entry_visits_per_block: transition_entry_visits_per_block,
            maximum_retained_bytes: retained_bytes,
        }
    }

    #[test]
    fn owner_ids_are_unique_and_checked_monotonic() {
        let counter = AtomicU64::new(0);
        let first = allocate_owner_id_from(&counter).expect("first owner");
        let second = allocate_owner_id_from(&counter).expect("second owner");
        let third = allocate_owner_id_from(&counter).expect("third owner");
        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        assert_eq!(third.get(), 3);
        assert!(first < second && second < third);
    }

    #[test]
    fn owner_id_exhaustion_does_not_reuse_the_last_valid_id() {
        let counter = AtomicU64::new(u64::MAX - 1);
        let last = allocate_owner_id_from(&counter).expect("maximum nonzero owner");
        assert_eq!(last.get(), u64::MAX);

        let refusal = allocate_owner_id_from(&counter).expect_err("owner counter exhausted");
        assert_eq!(refusal.reason, ObservationRefusalReason::ArithmeticOverflow);
        assert_eq!(counter.load(Ordering::Relaxed), u64::MAX);
        assert_eq!(
            allocate_owner_id_from(&counter)
                .expect_err("exhaustion is sticky")
                .reason,
            ObservationRefusalReason::ArithmeticOverflow
        );
    }

    #[test]
    fn empty_selection_has_zero_meter_work() {
        let projected =
            project_meter_work(0, u64::MAX, ObservationWorkCost::ZERO).expect("empty projection");
        assert_eq!(projected, ObservationWorkCost::ZERO);
    }

    #[test]
    fn fixed_transition_and_retained_reservation_survive_meter_projection() {
        let fixed = ObservationWorkCost {
            transition_entry_visits_per_block: 17,
            retained_bytes: 4096,
            ..ObservationWorkCost::ZERO
        };
        let projected = project_meter_work(0, 64, fixed).expect("fixed projection");
        assert_eq!(projected, fixed);
    }

    #[test]
    fn exact_limits_accept_the_meter_and_fixed_cost() {
        let meter_count = 3;
        let fixed = ObservationWorkCost {
            transition_entry_visits_per_block: 17,
            retained_bytes: 4096,
            ..ObservationWorkCost::ZERO
        };
        let expected = project_meter_work(3, 48, fixed).expect("expected projection");
        let limits = limits_with(
            expected.active_meter_channels,
            expected.meter_samples_per_block,
            expected.meter_publications_per_block,
            expected.meter_publication_bytes_per_block,
            expected.transition_entry_visits_per_block,
            expected.retained_bytes,
        );
        let budget =
            ObservationBudget::new(limits, NonZeroU32::new(48).expect("nonzero quantum"), fixed);
        assert_eq!(
            budget
                .preflight_graph_demand(meter_count, ObservationWorkCost::ZERO)
                .expect("inclusive limits"),
            expected
        );
    }

    #[test]
    fn one_below_each_meter_or_fixed_limit_refuses_with_the_public_field_name() {
        let meter_count = 3;
        let fixed = ObservationWorkCost {
            transition_entry_visits_per_block: 17,
            retained_bytes: 4096,
            ..ObservationWorkCost::ZERO
        };
        let expected = project_meter_work(3, 48, fixed).expect("expected projection");
        let fields = [
            (
                "maximum_active_meter_channels",
                expected.active_meter_channels,
            ),
            (
                "maximum_meter_samples_per_block",
                expected.meter_samples_per_block,
            ),
            (
                "maximum_meter_publications_per_block",
                expected.meter_publications_per_block,
            ),
            (
                "maximum_meter_publication_bytes_per_block",
                expected.meter_publication_bytes_per_block,
            ),
            (
                "maximum_transition_entry_visits_per_block",
                expected.transition_entry_visits_per_block,
            ),
            ("maximum_retained_bytes", expected.retained_bytes),
        ];
        for (field, value) in fields {
            let mut limits = limits_with(
                expected.active_meter_channels,
                expected.meter_samples_per_block,
                expected.meter_publications_per_block,
                expected.meter_publication_bytes_per_block,
                expected.transition_entry_visits_per_block,
                expected.retained_bytes,
            );
            match field {
                "maximum_active_meter_channels" => limits.maximum_active_meter_channels = value - 1,
                "maximum_meter_samples_per_block" => {
                    limits.maximum_meter_samples_per_block = value - 1
                }
                "maximum_meter_publications_per_block" => {
                    limits.maximum_meter_publications_per_block = value - 1;
                }
                "maximum_meter_publication_bytes_per_block" => {
                    limits.maximum_meter_publication_bytes_per_block = value - 1;
                }
                "maximum_transition_entry_visits_per_block" => {
                    limits.maximum_transition_entry_visits_per_block = value - 1;
                }
                "maximum_retained_bytes" => limits.maximum_retained_bytes = value - 1,
                _ => unreachable!("field list is exhaustive"),
            }
            let budget = ObservationBudget::new(
                limits,
                NonZeroU32::new(48).expect("nonzero quantum"),
                fixed,
            );
            let refusal = budget
                .preflight_graph_demand(meter_count, ObservationWorkCost::ZERO)
                .expect_err("one below must refuse");
            assert_eq!(refusal.reason, ObservationRefusalReason::WorkBudget);
            assert_eq!(refusal.limit, Some(field));
            assert_eq!(refusal.requested, Some(value));
            assert_eq!(refusal.maximum, Some(value - 1));
        }
    }

    #[test]
    fn arithmetic_overflow_is_checked_before_budget_comparison() {
        let overflow = project_meter_work(u64::MAX, u64::MAX, ObservationWorkCost::ZERO)
            .expect_err("channel multiplication overflows");
        assert_eq!(
            overflow.reason,
            ObservationRefusalReason::ArithmeticOverflow
        );

        let overflow = project_meter_work(u64::MAX / 2, 2, ObservationWorkCost::ZERO)
            .expect_err("sample multiplication overflows");
        assert_eq!(
            overflow.reason,
            ObservationRefusalReason::ArithmeticOverflow
        );

        let meter_snapshot_bytes = u64::try_from(size_of::<MeterSnapshot>()).expect("size fits");
        let overflow = project_meter_work(
            u64::MAX / meter_snapshot_bytes + 1,
            1,
            ObservationWorkCost::ZERO,
        )
        .expect_err("publication byte multiplication overflows");
        assert_eq!(
            overflow.reason,
            ObservationRefusalReason::ArithmeticOverflow
        );
    }

    #[test]
    fn failed_projection_preserves_the_last_committed_cost() {
        let fixed = ObservationWorkCost {
            transition_entry_visits_per_block: 17,
            retained_bytes: 4096,
            ..ObservationWorkCost::ZERO
        };
        let accepted = project_meter_work(1, 48, fixed).expect("accepted projection");
        let limits = limits_with(
            accepted.active_meter_channels,
            accepted.meter_samples_per_block,
            accepted.meter_publications_per_block,
            accepted.meter_publication_bytes_per_block,
            accepted.transition_entry_visits_per_block,
            accepted.retained_bytes,
        );
        let mut budget =
            ObservationBudget::new(limits, NonZeroU32::new(48).expect("nonzero quantum"), fixed);
        let committed = budget
            .preflight_graph_demand(1, ObservationWorkCost::ZERO)
            .expect("first projection");
        budget.commit_graph_demand(committed);
        let before = budget.accepted();

        let refusal = budget
            .preflight_graph_demand(2, ObservationWorkCost::ZERO)
            .expect_err("second meter exceeds every exact limit");
        assert_eq!(refusal.reason, ObservationRefusalReason::WorkBudget);
        assert_eq!(budget.accepted(), before);
    }

    #[test]
    fn spectrum_work_composes_with_meter_and_fixed_cost() {
        let fixed = ObservationWorkCost {
            transition_entry_visits_per_block: 17,
            retained_bytes: 4096,
            ..ObservationWorkCost::ZERO
        };
        let budget = ObservationBudget::new(
            limits_with(u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX),
            NonZeroU32::new(48).expect("nonzero quantum"),
            fixed,
        );
        let spectrum = ObservationWorkCost {
            active_spectrum_captures: 1,
            capture_input_samples_per_block: 2,
            capture_copy_samples_per_block: 3,
            capture_publications_per_block: 1,
            capture_bytes_per_second: 4,
            transition_entry_visits_per_block: 999,
            retained_bytes: 888,
            ..ObservationWorkCost::ZERO
        };
        let expected = ObservationWorkCost {
            active_meter_channels: 4,
            meter_samples_per_block: 4 * 48,
            meter_publications_per_block: 2,
            meter_publication_bytes_per_block: 2 * u64::try_from(size_of::<MeterSnapshot>())
                .expect("snapshot size"),
            active_spectrum_captures: 1,
            capture_input_samples_per_block: 2,
            capture_copy_samples_per_block: 3,
            capture_publications_per_block: 1,
            capture_bytes_per_second: 4,
            transition_entry_visits_per_block: fixed.transition_entry_visits_per_block,
            retained_bytes: fixed.retained_bytes,
        };
        assert_eq!(
            budget
                .preflight_graph_demand(2, spectrum)
                .expect("spectrum work is admitted by the checked composition"),
            expected
        );
    }

    #[test]
    fn spectrum_work_uses_the_frozen_fixed_cadence_formula() {
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let projected =
            project_spectrum_work(cadence, SpectrumChannels::Stereo).expect("spectrum projection");
        let record_bytes = u64::try_from(size_of::<SpectrumCapturedRecord>()).expect("record size");
        assert_eq!(projected.active_spectrum_captures, 1);
        assert_eq!(projected.capture_input_samples_per_block, 2 * 128);
        assert_eq!(projected.capture_copy_samples_per_block, 2 * 128 + 4 * 2048);
        assert_eq!(projected.capture_publications_per_block, 1);
        assert_eq!(projected.capture_bytes_per_second, (24 + 1) * record_bytes);
    }

    #[test]
    fn graph_refusal_cancels_staged_restart_and_preserves_queued_window() {
        use crate::{
            HostConsoleRequest, HostObservationPreparation, HostPrepareCaps, HostShapePolicy,
            HostSpectrumDemand, HostSpectrumMode, HostSpectrumReadError, SourceSubmission,
            SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumChannels,
            SpectrumTarget, compile_host_session, prepare_host_runtime_with_observation_demand,
        };

        const SESSION: &str =
            include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
        let caps = HostPrepareCaps {
            shape: HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 1_024,
            maximum_source_channels: None,
            maximum_automation_spans_per_block: 128,
            maximum_tracks: 100,
            maximum_sources: 100,
            maximum_routes: 100,
            maximum_effects: 100,
            maximum_graph_session_plus_plan_bytes: 100_000_000,
            maximum_source_total_bytes: 10_000_000,
            maximum_source_overhead_bytes: 10_000_000,
            maximum_effect_state_bytes: 100_000_000,
            maximum_effect_scratch_bytes: 100_000_000,
            maximum_builtin_retained_bytes: 100_000_000,
            maximum_named_allocation_bytes: 100_000_000,
            maximum_meter_streams: 100,
            maximum_meter_items: 1_000,
            maximum_meter_bytes: 10_000_000,
        };
        let compiled = compile_host_session(SESSION, &caps).expect("fixture session");
        let request = SpectrumCaptureCollectionRequest {
            entries: vec![SpectrumCaptureCollectionEntry {
                target: SpectrumTarget::Output("main-out".into()),
                channels: SpectrumChannels::Stereo,
            }],
            maximum_capture_bytes: u64::MAX,
        };
        let observations = HostObservationPreparation {
            meters: &[],
            spectrum: Some(&request),
            work_limits: ObservationWorkLimits {
                maximum_active_meter_channels: u64::MAX,
                maximum_meter_samples_per_block: u64::MAX,
                maximum_meter_publications_per_block: u64::MAX,
                maximum_meter_publication_bytes_per_block: u64::MAX,
                maximum_active_spectrum_captures: u64::MAX,
                maximum_capture_input_samples_per_block: u64::MAX,
                maximum_capture_copy_samples_per_block: u64::MAX,
                maximum_capture_publications_per_block: u64::MAX,
                maximum_capture_bytes_per_second: u64::MAX,
                maximum_transition_entry_visits_per_block: u64::MAX,
                maximum_retained_bytes: u64::MAX,
            },
            activation: GraphObservationActivationConfig {
                maximum_active_observers: 3,
                maximum_retained_bytes: u64::MAX,
            },
        };
        let console = HostConsoleRequest::default();
        let (host, _, mut owner) =
            prepare_host_runtime_with_observation_demand(&compiled, &caps, &console, &observations)
                .expect("spectrum owner");
        let demand = HostSpectrumDemand {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
            mode: HostSpectrumMode::Continuous,
        };
        let accepted = owner.replace_spectrum(&demand).expect("initial admission");
        let (mut render, mut sources, _) = host.start_render_session().expect("render session");
        for block in 0..16 {
            let left = [0.25_f32; 128];
            let right = [-0.25_f32; 128];
            sources
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: (block * 128) as u64,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: 128,
                        end_of_region: false,
                    },
                )
                .expect("source block");
            let mut output = [0.0_f32; 128 * 2];
            render
                .render_planar(&mut output, 2, 128, 128, (block * 128) as u64)
                .expect("render block");
        }
        assert_eq!(owner.try_applied().expect("initial receipt").revision, 1);

        // This is intentionally a private, module-local fault seam. It consumes the underlying
        // ordinary graph credit without creating a host receipt, so restart must cancel its staged
        // candidate and leave the accepted capture and its queued window untouched.
        owner
            .graph
            .as_mut()
            .expect("prepared graph")
            .replace(&[])
            .expect("injected ordinary graph publication");
        assert_eq!(
            owner
                .restart_spectrum()
                .expect_err("ordinary credit is occupied")
                .reason,
            ObservationRefusalReason::Backpressure
        );
        let preserved = owner
            .try_read_continuous_spectrum()
            .expect("queued window survives refusal");
        assert_eq!(preserved.owner, owner.owner());
        assert_eq!(preserved.observation_generation, accepted.revision);
        assert_eq!(preserved.selection_epoch, 1);
        assert_eq!(preserved.window.stream_epoch, 1);
        assert_eq!(preserved.window.sequence, 0);
        assert_eq!(preserved.window.first_sample, 0);

        drop(render);
        assert_eq!(
            owner.try_read_continuous_spectrum().err(),
            Some(HostSpectrumReadError::Closed)
        );
    }

    #[test]
    fn one_below_each_spectrum_work_limit_refuses_with_the_public_field_name() {
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let expected =
            project_spectrum_work(cadence, SpectrumChannels::Stereo).expect("spectrum projection");
        let fields = [
            (
                "maximum_active_spectrum_captures",
                expected.active_spectrum_captures,
            ),
            (
                "maximum_capture_input_samples_per_block",
                expected.capture_input_samples_per_block,
            ),
            (
                "maximum_capture_copy_samples_per_block",
                expected.capture_copy_samples_per_block,
            ),
            (
                "maximum_capture_publications_per_block",
                expected.capture_publications_per_block,
            ),
            (
                "maximum_capture_bytes_per_second",
                expected.capture_bytes_per_second,
            ),
        ];
        for (field, value) in fields {
            let mut limits =
                limits_with(u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
            match field {
                "maximum_active_spectrum_captures" => {
                    limits.maximum_active_spectrum_captures = value - 1
                }
                "maximum_capture_input_samples_per_block" => {
                    limits.maximum_capture_input_samples_per_block = value - 1
                }
                "maximum_capture_copy_samples_per_block" => {
                    limits.maximum_capture_copy_samples_per_block = value - 1
                }
                "maximum_capture_publications_per_block" => {
                    limits.maximum_capture_publications_per_block = value - 1
                }
                "maximum_capture_bytes_per_second" => {
                    limits.maximum_capture_bytes_per_second = value - 1
                }
                _ => unreachable!("field list is exhaustive"),
            }
            let budget = ObservationBudget::new(
                limits,
                NonZeroU32::new(128).expect("nonzero quantum"),
                ObservationWorkCost::ZERO,
            );
            let refusal = budget
                .preflight_graph_demand(0, expected)
                .expect_err("one below must refuse");
            assert_eq!(refusal.reason, ObservationRefusalReason::WorkBudget);
            assert_eq!(refusal.limit, Some(field));
            assert_eq!(refusal.requested, Some(value));
            assert_eq!(refusal.maximum, Some(value - 1));
        }
    }

    #[test]
    fn generation_filter_uses_actual_accumulator_records_in_one_queue() {
        use core::num::NonZeroUsize;

        let handle = MeterHandle(NonZeroU64::new(1).expect("handle"));
        let config = builtins::MeterConfig {
            period_frames: NonZeroU32::new(2).expect("period"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(4).expect("queue"),
            reset_generation: 1,
        };
        let mut prepared = builtins::MeterAccumulator::prepare_selected(
            handle,
            config,
            48_000,
            MeterMetricSet::SAMPLE_PEAK,
        )
        .expect("meter accumulator");
        let left = [0.25_f32; 2];
        let right = [-0.25_f32; 2];

        prepared.accumulator.restart_observation(7);
        prepared
            .accumulator
            .observe(&left, &right, 0)
            .expect("old generation");
        prepared.accumulator.restart_observation(8);
        prepared
            .accumulator
            .observe(&left, &right, 2)
            .expect("first live generation");
        prepared.accumulator.restart_observation(8);
        prepared
            .accumulator
            .observe(&left, &right, 4)
            .expect("newest live generation");

        let (newest, stale, superseded) = read_newest_meter_generation(&mut prepared.consumer, 8);
        let newest = newest.expect("matching snapshot");
        assert_eq!(newest.observation_generation, 8);
        assert_eq!(newest.start_sample, 4);
        assert_eq!(stale, 1);
        assert_eq!(superseded, 1);
        assert_eq!(prepared.consumer.available_at_entry(), 0);
    }
}
