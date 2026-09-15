//! Native selected-observation identity and work accounting.
//!
//! This module owns the native selected-meter preparation boundary and its checked work records.
//! The graph endpoint and meter consumers stay private to the owner; later admission tranches add
//! the bounded mutation and read operations without changing that ownership seam.

use core::{
    alloc::Layout,
    mem::size_of,
    num::{NonZeroU32, NonZeroU64},
    sync::atomic::{AtomicU64, Ordering},
};

use builtins::{MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap};
use builtins_compiler::MeterConsumer;
use graph::{
    GraphObservationActivationConfig, GraphObservationActivationResources,
    GraphObservationAdmissionError, GraphObservationController,
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
    /// Spectrum is reserved for a later tranche. An empty collection is normalized to absent.
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

/// One preallocated complete-set selection array with a live prefix.
struct MeterSelection {
    entries: Box<[MeterSelectionEntry]>,
    len: usize,
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

impl MeterSelection {
    fn empty() -> Self {
        Self {
            entries: Box::new([]),
            len: 0,
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
        })
    }

    fn entries(&self) -> &[MeterSelectionEntry] {
        &self.entries[..self.len]
    }

    fn entries_mut(&mut self) -> &mut [MeterSelectionEntry] {
        &mut self.entries[..self.len]
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
/// that no raw observer endpoint can bypass owner identity or its future ledger.
pub struct HostObservationController {
    owner: ObservationOwnerId,
    graph: Option<GraphObservationController>,
    meters: Box<[PreparedHostMeter]>,
    meter_consumers: Vec<MeterConsumer>,
    budget: ObservationBudget,
    accepted: MeterSelection,
    applied: MeterSelection,
    candidate: MeterSelection,
    pending: [PendingApplication; 2],
    candidate_handles: Box<[u64]>,
    terminal_closed: bool,
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
    /// Maximum simultaneously active spectrum captures. Spectrum is not admitted by this native
    /// selected-meter preparation tranche.
    pub maximum_active_spectrum_captures: u64,
    /// Maximum selected spectrum input samples charged per block. Spectrum is not admitted by
    /// this native selected-meter preparation tranche.
    pub maximum_capture_input_samples_per_block: u64,
    /// Maximum selected spectrum copy samples charged per block. Spectrum is not admitted by this
    /// native selected-meter preparation tranche.
    pub maximum_capture_copy_samples_per_block: u64,
    /// Maximum spectrum publication attempts charged per block. Spectrum is not admitted by this
    /// native selected-meter preparation tranche.
    pub maximum_capture_publications_per_block: u64,
    /// Maximum spectrum payload bytes charged per second. Spectrum is not admitted by this native
    /// selected-meter preparation tranche.
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
    /// Active spectrum captures. Native selected-meter preparation keeps this zero.
    pub active_spectrum_captures: u64,
    /// Selected spectrum input samples per block. Native selected-meter preparation keeps this
    /// zero.
    pub capture_input_samples_per_block: u64,
    /// Selected spectrum copy samples per block. Native selected-meter preparation keeps this zero.
    pub capture_copy_samples_per_block: u64,
    /// Spectrum publication attempts per block. Native selected-meter preparation keeps this zero.
    pub capture_publications_per_block: u64,
    /// Spectrum payload bytes per second. Native selected-meter preparation keeps this zero.
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

/// Return whether a cost contains no spectrum demand.
fn is_zero_spectrum_cost(cost: ObservationWorkCost) -> bool {
    cost.active_spectrum_captures == 0
        && cost.capture_input_samples_per_block == 0
        && cost.capture_copy_samples_per_block == 0
        && cost.capture_publications_per_block == 0
        && cost.capture_bytes_per_second == 0
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
        if !is_zero_spectrum_cost(spectrum_cost) {
            return Err(invalid_request());
        }
        let meter_count = u64::try_from(meter_count).map_err(|_| arithmetic_overflow())?;
        let projected = project_meter_work(meter_count, self.quantum_frames, self.fixed_cost)?;
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
    pub(crate) fn prepare(
        owner: ObservationOwnerId,
        graph: Option<GraphObservationController>,
        requests: &[crate::prepare::HostMeterRequest],
        period_frames: Option<NonZeroU32>,
        meter_consumers: Vec<MeterConsumer>,
        budget: ObservationBudget,
        capacity: usize,
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

        let accepted = MeterSelection::with_capacity(capacity)?;
        let applied = MeterSelection::with_capacity(capacity)?;
        let candidate = MeterSelection::with_capacity(capacity)?;
        let pending_ordinary = PendingApplication {
            accepted: None,
            selection: MeterSelection::with_capacity(capacity)?,
        };
        let pending_removal = PendingApplication {
            accepted: None,
            selection: MeterSelection::with_capacity(capacity)?,
        };
        let mut candidate_handles = Vec::new();
        candidate_handles
            .try_reserve_exact(capacity)
            .map_err(|_| allocation_failure())?;
        candidate_handles.resize(capacity, 0);

        Ok(Self {
            owner,
            graph,
            meters: meters.into_boxed_slice(),
            meter_consumers,
            budget,
            accepted,
            applied,
            candidate,
            pending: [pending_ordinary, pending_removal],
            candidate_handles: candidate_handles.into_boxed_slice(),
            terminal_closed: false,
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
        self.publish_meter_candidate(meters, PublicationKind::Ordinary)
    }

    /// Remove meters to the complete selected subset using the reserved removal publication.
    ///
    /// A removal may pass one outstanding ordinary publication, but cannot overlap another
    /// removal. It is validated against the latest accepted set rather than the last applied set,
    /// so the two graph credits remain useful for a bounded ordinary-then-removal sequence.
    #[allow(clippy::result_large_err)]
    pub fn remove_meters_to(
        &mut self,
        remaining: &[HostMeterId],
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        self.publish_meter_candidate(remaining, PublicationKind::Removal)
    }

    #[allow(clippy::result_large_err)]
    fn publish_meter_candidate(
        &mut self,
        requested: &[HostMeterId],
        kind: PublicationKind,
    ) -> Result<ObservationAccepted, ObservationRefusal> {
        // An empty prepared catalog has no graph endpoint. Keep this distinction from a terminal
        // endpoint: the inert owner is live, but there is nothing to publish into.
        if self.graph.is_none() {
            return Err(not_prepared());
        }
        if self.is_closed() {
            return Err(closed());
        }

        // Check owner identity before handle lookup. A numerically matching handle from another
        // prepared owner must never address this owner's consumer.
        if requested.iter().any(|meter| meter.owner != self.owner) {
            return Err(wrong_owner());
        }
        if requested.len() > self.candidate.entries.len() {
            return Err(capacity());
        }

        // Resolve each handle into the preallocated candidate scratch and reject duplicates. The
        // immutable prepared catalog is scanned on the control side; no lookup map is retained by
        // the owner.
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
        self.candidate.len = requested.len();

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

        let work = self
            .budget
            .preflight_graph_demand(self.candidate.len, ObservationWorkCost::ZERO)?;

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

        self.candidate
            .entries_mut()
            .sort_unstable_by_key(|entry| self.meters[entry.prepared_index].id.handle.0.get());
        for (index, entry) in self.candidate.entries().iter().enumerate() {
            self.candidate_handles[index] = self.meters[entry.prepared_index].id.handle.0.get();
        }

        // `graph.replace`/`remove_to` is deliberately the last fallible operation. The candidate
        // storage and handle slice were both prepared above, so graph acceptance leaves only
        // infallible generation filling, bounded copies and budget assignment.
        let graph_accepted = {
            let graph = self.graph.as_mut().expect("graph checked above");
            match kind {
                PublicationKind::Ordinary => {
                    graph.replace(&self.candidate_handles[..self.candidate.len])
                }
                PublicationKind::Removal => {
                    graph.remove_to(&self.candidate_handles[..self.candidate.len])
                }
            }
        }
        .map_err(map_graph_refusal)?;

        let revision = graph_accepted.revision;
        for entry in self.candidate.entries_mut() {
            if entry.generation == 0 {
                entry.generation = revision;
            }
        }
        let accepted = ObservationAccepted {
            owner: self.owner,
            revision,
            work,
        };
        let candidate_len = self.candidate.len;
        let pending = &mut self.pending[pending_index];
        pending.selection.entries[..candidate_len]
            .copy_from_slice(&self.candidate.entries[..candidate_len]);
        pending.selection.len = candidate_len;
        pending.accepted = Some(accepted);
        self.accepted.entries[..candidate_len]
            .copy_from_slice(&self.candidate.entries[..candidate_len]);
        self.accepted.len = candidate_len;
        self.budget.commit_graph_demand(work);
        Ok(accepted)
    }
}

/// Derive the new host-owned metadata heap rows from the concrete owner layout.
pub(crate) fn observation_metadata_resources(
    catalog_len: usize,
    track_id_lengths: impl IntoIterator<Item = usize>,
    capacity: usize,
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
        add(layout_bytes::<MeterSelectionEntry>(capacity)?)?;
    }
    add(layout_bytes::<u64>(capacity)?)?;
    Ok((total, largest))
}

/// Derive the additive host rows and narrow retained reservation for a prepared owner.
pub(crate) fn observation_resources(
    catalog_len: usize,
    track_id_lengths: impl IntoIterator<Item = usize>,
    capacity: usize,
    graph_activation: Option<GraphObservationActivationResources>,
    builtin_retained_bytes: u64,
) -> Result<HostObservationResources, ObservationRefusal> {
    let (metadata_heap_bytes, metadata_largest_allocation_bytes) =
        observation_metadata_resources(catalog_len, track_id_lengths, capacity)?;
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
            .checked_add(activation.retained_bytes)
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
            maximum_active_spectrum_captures: 0,
            maximum_capture_input_samples_per_block: 0,
            maximum_capture_copy_samples_per_block: 0,
            maximum_capture_publications_per_block: 0,
            maximum_capture_bytes_per_second: 0,
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
    fn b1_does_not_accept_nonzero_spectrum_cost() {
        let budget = ObservationBudget::new(
            limits_with(u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX),
            NonZeroU32::new(48).expect("nonzero quantum"),
            ObservationWorkCost::ZERO,
        );
        let spectrum = ObservationWorkCost {
            active_spectrum_captures: 1,
            ..ObservationWorkCost::ZERO
        };
        let refusal = budget
            .preflight_graph_demand(0, spectrum)
            .expect_err("spectrum is outside B1");
        assert_eq!(refusal.reason, ObservationRefusalReason::InvalidRequest);
    }
}
