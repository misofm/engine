//! Bounded block-boundary publication and off-render retirement.

use super::spsc::bounded_spsc_internal;
use super::{Consumer, PlanEpoch, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError};
use super::{PreparedRenderPlan, RenderError, RenderIo, RenderReport, RenderTime};
use core::{cell::Cell, num::NonZeroUsize};

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
}
/// Control-side retirement owner. Reclamation happens only by popping here.
pub struct PlanRetirer {
    queue: Consumer<RetiredPlan>,
}
/// Realtime plan ownership, with at most one pending candidate.
pub struct RealtimePlanOwner {
    active: (PlanEpoch, PreparedRenderPlan),
    pending: Option<(PlanEpoch, PreparedRenderPlan)>,
    publication: Consumer<PublishedPlan>,
    retirement: Producer<RetiredPlan>,
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

/// Prepare bounded publication and retirement queues for plans with one exact envelope.
pub fn plan_exchange(
    initial: PreparedRenderPlan,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), SpscError> {
    let envelope = initial.envelope();
    let (publication_producer, publication_consumer) =
        bounded_spsc_internal(config.publication_capacity, QueueGeneration(1))?;
    let (retirement_producer, retirement_consumer) =
        bounded_spsc_internal(config.retirement_capacity, QueueGeneration(2))?;
    Ok((
        PlanPublisher {
            queue: publication_producer,
            next_epoch: 1,
            envelope,
        },
        RealtimePlanOwner {
            active: (PlanEpoch(0), initial),
            pending: None,
            publication: publication_consumer,
            retirement: retirement_producer,
            deferred: 0,
            _not_sync: Cell::new(()),
        },
        PlanRetirer {
            queue: retirement_consumer,
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
        };
        match self.queue.try_push(item) {
            Ok(()) => {
                self.next_epoch = epoch + 1;
                Ok(PlanEpoch(epoch))
            }
            Err(QueueFull { value, .. }) => Err(PublishError::Full(value.plan)),
        }
    }
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
            self.pending = Some((candidate.epoch, candidate.plan));
        }
        let Some(_) = self.pending.as_ref() else {
            return SwapOutcome::None;
        };
        let old_epoch = self.active.0;
        // Queue capacity is checked before moving the active plan; no old plan can drop here.
        let placeholder = match self.retirement.try_reserve() {
            Some(permit) => permit,
            None => {
                self.deferred = self.deferred.saturating_add(1);
                return SwapOutcome::DeferredRetirementFull;
            }
        };
        let Some(candidate) = self.pending.take() else {
            return SwapOutcome::None;
        };
        let old = core::mem::replace(&mut self.active, candidate);
        placeholder.commit(RetiredPlan {
            epoch: old_epoch,
            plan: old.1,
        });
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
impl PlanRetirer {
    /// Reclaim one displaced plan on the control/retirement owner.
    pub fn try_reclaim(&mut self) -> Result<(PlanEpoch, PreparedRenderPlan), QueueEmpty> {
        self.queue.try_pop().map(|item| (item.epoch, item.plan))
    }
}
