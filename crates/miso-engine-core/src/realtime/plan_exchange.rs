//! Bounded block-boundary publication and off-render retirement.

use super::spsc::bounded_spsc_internal;
use super::{Consumer, PlanEpoch, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError};
use super::{PreparedRenderPlan, RenderError, RenderIo, RenderReport, RenderTime};
use core::{
    cell::Cell,
    fmt,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Arc;

/// Capacity choices for publication and retirement directions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanExchangeConfig {
    /// Number of validated replacement plans that control may queue.
    pub publication_capacity: NonZeroUsize,
    /// Number of displaced plans that may await off-render reclamation.
    pub retirement_capacity: NonZeroUsize,
}
/// Result of one render-entry swap attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapOutcome {
    /// No replacement plan was pending at this boundary.
    None,
    /// One complete replacement plan became active before rendering.
    Applied,
    /// The pending replacement remained pending because retirement was full.
    DeferredRetirementFull,
}
struct PublishedPlan {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    retirement_reserved: bool,
}
struct RetiredPlan {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
}
/// Control-side publisher. Failed publication retains the original plan.
pub struct PlanPublisher {
    queue: Producer<PublishedPlan>,
    next_epoch: u64,
    envelope: super::RenderEnvelope,
    retirement_credits: Arc<AtomicUsize>,
    legacy_outstanding: Arc<AtomicUsize>,
}
/// Control-side retirement owner. Reclamation happens only by popping here.
pub struct PlanRetirer {
    queue: Consumer<RetiredPlan>,
    retirement_credits: Arc<AtomicUsize>,
}
/// Realtime plan ownership, with at most one pending candidate.
pub struct RealtimePlanOwner {
    active: (PlanEpoch, PreparedRenderPlan),
    pending: Option<PublishedPlan>,
    publication: Consumer<PublishedPlan>,
    retirement: Producer<RetiredPlan>,
    retirement_credits: Arc<AtomicUsize>,
    legacy_outstanding: Arc<AtomicUsize>,
    deferred: u64,
    _not_sync: Cell<()>,
}
/// Fixed report proving which complete plan owned a rendered block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeRenderReport {
    /// Boundary publication decision made before processing the block.
    pub swap: SwapOutcome,
    /// Epoch of the plan that rendered the block.
    pub active_epoch: PlanEpoch,
    /// Inner prepared-plan render report.
    pub render: RenderReport,
}
/// Publication failure preserving candidate ownership.
#[must_use]
pub enum PublishError {
    /// Bounded publication storage is full.
    Full(PreparedRenderPlan),
    /// Candidate external envelope differs from the running exchange.
    Incompatible(PreparedRenderPlan),
    /// The next monotonically increasing publication epoch cannot be represented.
    EpochExhausted(PreparedRenderPlan),
}

/// A fully ownership-preserving replacement-reservation failure.
#[must_use]
pub enum PlanReplacementReservationError {
    /// The bounded publication queue has no free slot.
    PublicationFull(PreparedRenderPlan),
    /// Every eventual displaced-plan retirement credit is already owned.
    RetirementFull(PreparedRenderPlan),
    /// Candidate external envelope differs from the running exchange.
    Incompatible(PreparedRenderPlan),
    /// The next monotonically increasing publication epoch cannot be represented.
    EpochExhausted(PreparedRenderPlan),
}

impl fmt::Debug for PlanReplacementReservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PublicationFull(_) => "PublicationFull(..)",
            Self::RetirementFull(_) => "RetirementFull(..)",
            Self::Incompatible(_) => "Incompatible(..)",
            Self::EpochExhausted(_) => "EpochExhausted(..)",
        })
    }
}

/// An affine control-side reservation of one exact publication slot, epoch, and retirement
/// credit.  Its lifetime exclusively borrows the publisher, so it cannot be committed through a
/// different exchange or reordered with another publication.  Drop/cancel returns the credit.
pub struct PlanReplacementReservation<'a> {
    publication: Option<super::spsc::PushPermit<'a, PublishedPlan>>,
    next_epoch: &'a mut u64,
    epoch: PlanEpoch,
    plan: Option<PreparedRenderPlan>,
    retirement_credits: Arc<AtomicUsize>,
    credit_armed: bool,
}

/// Prepare bounded publication and retirement queues for plans with one exact envelope.
pub fn plan_exchange(
    initial: PreparedRenderPlan,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), SpscError> {
    let envelope = initial.envelope();
    let retirement_credits = Arc::new(AtomicUsize::new(config.retirement_capacity.get()));
    let legacy_outstanding = Arc::new(AtomicUsize::new(0));
    let (publication_producer, publication_consumer) =
        bounded_spsc_internal(config.publication_capacity, QueueGeneration(1))?;
    let (retirement_producer, retirement_consumer) =
        bounded_spsc_internal(config.retirement_capacity, QueueGeneration(2))?;
    Ok((
        PlanPublisher {
            queue: publication_producer,
            next_epoch: 1,
            envelope,
            retirement_credits: Arc::clone(&retirement_credits),
            legacy_outstanding: Arc::clone(&legacy_outstanding),
        },
        RealtimePlanOwner {
            active: (PlanEpoch(0), initial),
            pending: None,
            publication: publication_consumer,
            retirement: retirement_producer,
            retirement_credits: Arc::clone(&retirement_credits),
            legacy_outstanding,
            deferred: 0,
            _not_sync: Cell::new(()),
        },
        PlanRetirer {
            queue: retirement_consumer,
            retirement_credits,
        },
    ))
}
impl PlanPublisher {
    /// Publish an exact-envelope candidate; epoch is consumed only on success.
    #[allow(clippy::result_large_err)] // Ownership-preserving backpressure is the public contract.
    pub fn publish(&mut self, plan: PreparedRenderPlan) -> Result<PlanEpoch, PublishError> {
        if plan.envelope() != self.envelope {
            return Err(PublishError::Incompatible(plan));
        }
        if self.next_epoch == u64::MAX {
            return Err(PublishError::EpochExhausted(plan));
        }
        let epoch = self.next_epoch;
        let item = PublishedPlan {
            epoch: PlanEpoch(epoch),
            plan,
            retirement_reserved: false,
        };
        self.legacy_outstanding.fetch_add(1, Ordering::AcqRel);
        match self.queue.try_push(item) {
            Ok(()) => {
                self.next_epoch = epoch + 1;
                Ok(PlanEpoch(epoch))
            }
            Err(QueueFull { value, .. }) => {
                self.legacy_outstanding.fetch_sub(1, Ordering::AcqRel);
                Err(PublishError::Full(value.plan))
            }
        }
    }

    /// Reserve publication and the eventual displaced-plan retirement before any caller-owned
    /// state becomes visible.  A valid reservation's [`PlanReplacementReservation::commit`] is
    /// non-fallible and publishes exactly once.
    #[allow(clippy::result_large_err)] // Every failure returns the complete candidate.
    pub fn reserve_replacement(
        &mut self,
        plan: PreparedRenderPlan,
    ) -> Result<PlanReplacementReservation<'_>, PlanReplacementReservationError> {
        if plan.envelope() != self.envelope {
            return Err(PlanReplacementReservationError::Incompatible(plan));
        }
        if self.next_epoch == u64::MAX {
            return Err(PlanReplacementReservationError::EpochExhausted(plan));
        }
        let Some(publication) = self.queue.try_reserve() else {
            return Err(PlanReplacementReservationError::PublicationFull(plan));
        };
        if self.legacy_outstanding.load(Ordering::Acquire) != 0
            || !try_take_retirement_credit(&self.retirement_credits)
        {
            return Err(PlanReplacementReservationError::RetirementFull(plan));
        }
        let epoch = PlanEpoch(self.next_epoch);
        Ok(PlanReplacementReservation {
            publication: Some(publication),
            next_epoch: &mut self.next_epoch,
            epoch,
            plan: Some(plan),
            retirement_credits: Arc::clone(&self.retirement_credits),
            credit_armed: true,
        })
    }
}

impl PlanReplacementReservation<'_> {
    /// Exact epoch that will become visible if this reservation is committed.
    #[must_use]
    pub const fn epoch(&self) -> PlanEpoch {
        self.epoch
    }

    /// Publish the bound complete candidate. All fallible checks and credits were consumed by
    /// reservation, so this operation is a bounded move plus one release-store.
    pub fn commit(mut self) -> PlanEpoch {
        let epoch = self.epoch;
        *self.next_epoch = epoch.0 + 1;
        let plan = self.plan.take().expect("affine reservation owns plan");
        self.credit_armed = false;
        self.publication
            .take()
            .expect("affine reservation owns publication slot")
            .commit(PublishedPlan {
                epoch,
                plan,
                retirement_reserved: true,
            });
        epoch
    }

    /// Cancel without publication and return the complete candidate to the caller.
    pub fn cancel(mut self) -> PreparedRenderPlan {
        self.plan.take().expect("affine reservation owns plan")
    }
}

impl Drop for PlanReplacementReservation<'_> {
    fn drop(&mut self) {
        if self.credit_armed {
            self.retirement_credits.fetch_add(1, Ordering::Release);
            self.credit_armed = false;
        }
    }
}

fn try_take_retirement_credit(credits: &AtomicUsize) -> bool {
    let available = credits.load(Ordering::Acquire);
    available != 0
        && credits
            .compare_exchange(
                available,
                available - 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
}
// REALTIME_POLICY_BEGIN
impl RealtimePlanOwner {
    /// Epoch of the currently active complete plan.
    #[must_use]
    pub fn active_epoch(&self) -> PlanEpoch {
        self.active.0
    }
    /// Control-plane ID of the currently active complete plan.
    #[must_use]
    pub fn active_plan_id(&self) -> u64 {
        self.active.1.program().plan_id()
    }
    /// Saturating count of swaps deferred by a full retirement queue.
    #[must_use]
    pub const fn deferred_count(&self) -> u64 {
        self.deferred
    }
    fn enter_block(&mut self) -> SwapOutcome {
        if self.pending.is_none()
            && let Ok(candidate) = self.publication.try_pop()
        {
            self.pending = Some(candidate);
        }
        let Some(_) = self.pending.as_ref() else {
            return SwapOutcome::None;
        };
        let old_epoch = self.active.0;
        let retirement_reserved = self
            .pending
            .as_ref()
            .is_some_and(|candidate| candidate.retirement_reserved);
        if !retirement_reserved && !try_take_retirement_credit(&self.retirement_credits) {
            self.deferred = self.deferred.saturating_add(1);
            return SwapOutcome::DeferredRetirementFull;
        }
        // A consumed credit proves a queue slot exists. A conservative failed CAS above leaves
        // the unreserved candidate pending; a reserved candidate never makes a new decision.
        let placeholder = match self.retirement.try_reserve() {
            Some(permit) => permit,
            None => {
                if !retirement_reserved {
                    self.retirement_credits.fetch_add(1, Ordering::Release);
                }
                self.deferred = self.deferred.saturating_add(1);
                return SwapOutcome::DeferredRetirementFull;
            }
        };
        let Some(candidate) = self.pending.take() else {
            return SwapOutcome::None;
        };
        let old = core::mem::replace(&mut self.active, (candidate.epoch, candidate.plan));
        placeholder.commit(RetiredPlan {
            epoch: old_epoch,
            plan: old.1,
        });
        if !candidate.retirement_reserved {
            self.legacy_outstanding.fetch_sub(1, Ordering::AcqRel);
        }
        SwapOutcome::Applied
    }
    /// Attempt one block-boundary publication and render through exactly one complete plan.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RealtimeRenderReport, RenderError> {
        super::audit::in_render_scope(|| {
            let swap = self.enter_block();
            let active_epoch = self.active.0;
            let render = self.active.1.render_inner(io, time)?;
            Ok(RealtimeRenderReport {
                swap,
                active_epoch,
                render,
            })
        })
    }
}
// REALTIME_POLICY_END
impl RealtimePlanOwner {
    /// Copy cumulative auxiliary-worker audit snapshots after callback rendering is disarmed.
    pub fn copy_worker_audit_snapshots(&self, output: &mut [super::audit::AuditSnapshot]) -> usize {
        self.active.1.copy_worker_audit_snapshots(output)
    }
}
impl Drop for RealtimePlanOwner {
    fn drop(&mut self) {
        if let Some(candidate) = self.pending.take() {
            if candidate.retirement_reserved {
                self.retirement_credits.fetch_add(1, Ordering::Release);
            } else {
                self.legacy_outstanding.fetch_sub(1, Ordering::AcqRel);
            }
        }
        while let Ok(candidate) = self.publication.try_pop() {
            if candidate.retirement_reserved {
                self.retirement_credits.fetch_add(1, Ordering::Release);
            } else {
                self.legacy_outstanding.fetch_sub(1, Ordering::AcqRel);
            }
        }
    }
}
impl PlanRetirer {
    /// Reclaim one displaced plan on the control/retirement owner.
    pub fn try_reclaim(&mut self) -> Result<(PlanEpoch, PreparedRenderPlan), QueueEmpty> {
        self.queue.try_pop().map(|item| {
            self.retirement_credits.fetch_add(1, Ordering::Release);
            (item.epoch, item.plan)
        })
    }
}
impl Drop for PlanRetirer {
    fn drop(&mut self) {
        while let Ok(_retired) = self.queue.try_pop() {
            self.retirement_credits.fetch_add(1, Ordering::Release);
        }
    }
}
