//! Native selected-observation identity and work accounting.
//!
//! This module is the control-side foundation for the selected-meter owner.  The first
//! implementation tranche deliberately contains no graph binding or render integration: it
//! only gives later preparation and admission code one checked owner identity, one set of public
//! records, and one pure meter-work projection.

use core::{
    mem::size_of,
    num::{NonZeroU32, NonZeroU64},
    sync::atomic::{AtomicU64, Ordering},
};

use builtins::{MeterHandle, MeterMetricSet, MeterSnapshot, MeterTap};

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
    /// Maximum simultaneously active spectrum captures.  Spectrum is not admitted in B1.
    pub maximum_active_spectrum_captures: u64,
    /// Maximum selected spectrum input samples charged per block.  Spectrum is not admitted in
    /// B1.
    pub maximum_capture_input_samples_per_block: u64,
    /// Maximum selected spectrum copy samples charged per block.  Spectrum is not admitted in
    /// B1.
    pub maximum_capture_copy_samples_per_block: u64,
    /// Maximum spectrum publication attempts charged per block.  Spectrum is not admitted in B1.
    pub maximum_capture_publications_per_block: u64,
    /// Maximum spectrum payload bytes charged per second.  Spectrum is not admitted in B1.
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
    /// Active spectrum captures.  B1 keeps this zero.
    pub active_spectrum_captures: u64,
    /// Selected spectrum input samples per block.  B1 keeps this zero.
    pub capture_input_samples_per_block: u64,
    /// Selected spectrum copy samples per block.  B1 keeps this zero.
    pub capture_copy_samples_per_block: u64,
    /// Spectrum publication attempts per block.  B1 keeps this zero.
    pub capture_publications_per_block: u64,
    /// Spectrum payload bytes per second.  B1 keeps this zero.
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

fn work_budget(limit: &'static str, requested: u64, maximum: u64) -> ObservationRefusal {
    ObservationRefusal {
        reason: ObservationRefusalReason::WorkBudget,
        limit: Some(limit),
        requested: Some(requested),
        maximum: Some(maximum),
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

/// Private control-side work owner used by later preparation and admission tranches.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ObservationBudget {
    limits: ObservationWorkLimits,
    quantum_frames: u64,
    fixed_cost: ObservationWorkCost,
    accepted: ObservationWorkCost,
}

impl ObservationBudget {
    fn new(
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
