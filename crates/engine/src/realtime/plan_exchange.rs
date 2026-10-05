//! Block-boundary publication through a two-cell mailbox, and off-render retirement.
//!
//! Publication is the plan mailbox of `spsc.rs` (#1343): the control thread publishes one
//! candidate into the mailbox's `Empty` cell, can take it back whole while render has not claimed
//! it ([`PlanPublisher::withdraw`]) and can publish it again ([`PlanPublisher::republish`]).
//! Render claims a candidate with one compare-and-swap at block entry and adopts it in the same
//! block; it never waits on the control thread or retries, and the control thread never waits on
//! render.
//!
//! Every candidate carries one retirement credit, taken when it is reserved. The credit travels
//! with the candidate into the retirement queue when render adopts it (as the credit of the plan
//! it displaces) and returns when the retirer reclaims that plan, or when the control thread drops
//! a reservation or an [`UnadoptedCandidate`]. So a claimed candidate always finds retirement room,
//! and render never defers a swap.

use super::spsc::{
    MailboxPermit, MailboxReader, MailboxWithdrawal, MailboxWriter, bounded_spsc_internal,
    bounded_spsc_retained_payload, plan_mailbox, plan_mailbox_retained_bytes,
};
use super::{Consumer, Producer, QueueEmpty, QueueGeneration, SpscError};

/// Epoch assigned to a successfully published render plan.
///
/// #84 phase C: this moved here from the deleted parameter-event store; publication epochs are
/// plan-exchange vocabulary, not parameter vocabulary.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlanEpoch(pub u64);
use super::{CarryOutcome, PreparedRenderPlan, RenderError, RenderIo, RenderReport, RenderTime};
use core::{
    alloc::Layout,
    cell::Cell,
    fmt,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};
use std::sync::Arc;

/// Capacity choice for the retirement direction. Publication needs none: the mailbox holds one
/// published candidate by construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanExchangeConfig {
    /// Number of displaced plans that may await off-render reclamation, and so the number of
    /// retirement credits: reserved candidates, unadopted candidates and retired plans together.
    pub retirement_capacity: NonZeroUsize,
}

/// Exact engine-owned heap payload budget for one prepared plan exchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanExchangeResourceReport {
    /// Sum of the mailbox (its state word and two cells), the retirement SPSC header and backing,
    /// and the shared credit-counter allocation.
    pub retained_payload_bytes: u64,
    /// Largest single requested heap payload allocation among those rows.
    pub largest_allocation_bytes: u64,
}

#[repr(C)]
struct SharedCounterAllocation {
    strong: AtomicUsize,
    weak: AtomicUsize,
    value: AtomicUsize,
}
/// Result of one render-entry swap decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapOutcome {
    /// No candidate was claimed at this boundary: none was published, or control withdrew it
    /// after render's load (render looks again at the next block).
    None,
    /// One complete replacement plan became active before rendering.
    Applied,
}

/// One retirement credit. Dropping it returns the credit; render only ever moves it.
struct RetirementCredit {
    credits: Arc<AtomicUsize>,
}

impl RetirementCredit {
    fn try_take(credits: &Arc<AtomicUsize>) -> Option<Self> {
        let available = credits.load(Ordering::Acquire);
        let taken = available != 0
            && credits
                .compare_exchange(
                    available,
                    available - 1,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok();
        taken.then(|| Self {
            credits: Arc::clone(credits),
        })
    }
}

impl Drop for RetirementCredit {
    fn drop(&mut self) {
        self.credits.fetch_add(1, Ordering::Release);
    }
}

struct PublishedPlan {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    credit: RetirementCredit,
}
struct RetiredPlan {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    /// The credit of the candidate that displaced this plan; it returns at reclamation.
    credit: RetirementCredit,
}
/// Control-side publisher. Failed publication retains the original plan.
pub struct PlanPublisher {
    mailbox: MailboxWriter<PublishedPlan>,
    next_epoch: u64,
    envelope: super::RenderEnvelope,
    retirement_credits: Arc<AtomicUsize>,
}
/// Control-side retirement owner. Reclamation happens only by popping here.
pub struct PlanRetirer {
    queue: Consumer<RetiredPlan>,
}
/// Realtime plan ownership. A published candidate stays in the mailbox until render claims it.
pub struct RealtimePlanOwner {
    active: (PlanEpoch, PreparedRenderPlan),
    publication: MailboxReader<PublishedPlan>,
    retirement: Producer<RetiredPlan>,
    carried: u64,
    carry_mismatched: u64,
    _not_sync: Cell<()>,
}
/// Fixed report proving which complete plan owned a rendered block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeRenderReport {
    /// Boundary publication decision made before processing the block.
    pub swap: SwapOutcome,
    /// What the incoming plan took from its predecessor; `NotRequested` unless `swap` is `Applied`.
    pub carry: CarryOutcome,
    /// Epoch of the plan that rendered the block.
    pub active_epoch: PlanEpoch,
    /// Inner prepared-plan render report.
    pub render: RenderReport,
}

/// Boundary metadata for a response capture made from the active realtime plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealtimeResponseSnapshot {
    /// The complete plan that owned the capture.
    pub active_epoch: PlanEpoch,
    /// The active plan's next sample at capture.
    pub captured_sample: u64,
    /// Number of response-capable owners copied in graph order.
    pub owners: u32,
}
/// Publication failure preserving candidate ownership.
#[must_use]
pub enum PublishError {
    /// The mailbox already holds a published candidate, or no retirement credit is free.
    Full(PreparedRenderPlan),
    /// Candidate external envelope differs from the running exchange.
    Incompatible(PreparedRenderPlan),
    /// The next monotonically increasing publication epoch cannot be represented.
    EpochExhausted(PreparedRenderPlan),
}

/// A fully ownership-preserving replacement-reservation failure.
#[must_use]
pub enum PlanReplacementReservationError {
    /// The mailbox already holds a published candidate that render has not claimed.
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

/// An affine control-side reservation of the mailbox's `Empty` cell, one epoch and one
/// retirement credit.  Its lifetime exclusively borrows the publisher, so it cannot be committed
/// through a different exchange or reordered with another publication.  Drop/cancel returns the
/// credit and consumes no epoch.
pub struct PlanReplacementReservation<'a> {
    publication: MailboxPermit<'a, PublishedPlan>,
    next_epoch: &'a mut u64,
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    credit: RetirementCredit,
}

/// A published candidate the control thread took back before render claimed it: whole,
/// unrendered, with its epoch and its retirement credit. Republish it with
/// [`PlanPublisher::republish`], or drop it on the control thread, which returns its credit.
#[must_use]
pub struct UnadoptedCandidate {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    credit: RetirementCredit,
}

impl UnadoptedCandidate {
    /// The epoch the candidate was published with; republication keeps it.
    #[must_use]
    pub const fn epoch(&self) -> PlanEpoch {
        self.epoch
    }

    /// Control-plane ID of the candidate plan.
    #[must_use]
    pub fn plan_id(&self) -> u64 {
        self.plan.program().plan_id()
    }

    /// The candidate plan, unrendered.
    #[must_use]
    pub const fn plan(&self) -> &PreparedRenderPlan {
        &self.plan
    }

    /// Give up the candidate: return its retirement credit and hand back the plan.
    #[must_use]
    pub fn into_plan(self) -> PreparedRenderPlan {
        let Self { plan, credit, .. } = self;
        drop(credit);
        plan
    }
}

impl fmt::Debug for UnadoptedCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UnadoptedCandidate")
            .field("epoch", &self.epoch)
            .field("plan_id", &self.plan_id())
            .finish_non_exhaustive()
    }
}

/// The result of [`PlanPublisher::withdraw`].
#[must_use]
#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // The candidate comes back whole, by value, never boxed.
pub enum Withdrawal {
    /// The candidate was still published; it is back on the control thread, unrendered.
    Withdrawn(UnadoptedCandidate),
    /// Render's claim won: the candidate is adopted, and that cannot change.
    Taken,
    /// No candidate was published since the last claim or withdrawal.
    Nothing,
}

/// Project the exact heap payloads that [`plan_exchange`] requests for this configuration.
pub fn plan_exchange_resource_report(
    config: PlanExchangeConfig,
) -> Result<PlanExchangeResourceReport, SpscError> {
    let retirement = bounded_spsc_retained_payload::<RetiredPlan>(config.retirement_capacity)?;
    let rows = [
        plan_mailbox_retained_bytes::<PublishedPlan>(),
        retirement.ring_header_bytes,
        retirement.slot_payload_bytes,
        Layout::new::<SharedCounterAllocation>().size(),
    ];
    let retained = rows.iter().try_fold(0_u64, |total, row| {
        total
            .checked_add(u64::try_from(*row).map_err(|_| SpscError::CapacityOverflow)?)
            .ok_or(SpscError::CapacityOverflow)
    })?;
    let largest = rows
        .into_iter()
        .max()
        .and_then(|value| u64::try_from(value).ok())
        .ok_or(SpscError::CapacityOverflow)?;
    Ok(PlanExchangeResourceReport {
        retained_payload_bytes: retained,
        largest_allocation_bytes: largest,
    })
}

/// Prepare the publication mailbox and the bounded retirement queue for plans with one exact
/// envelope.
pub fn plan_exchange(
    initial: PreparedRenderPlan,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), SpscError> {
    let _resources = plan_exchange_resource_report(config)?;
    let envelope = initial.envelope();
    let retirement_credits = Arc::new(AtomicUsize::new(config.retirement_capacity.get()));
    let (retirement_producer, retirement_consumer) =
        bounded_spsc_internal(config.retirement_capacity, QueueGeneration(2))?;
    let (mailbox_writer, mailbox_reader) = plan_mailbox();
    Ok((
        PlanPublisher {
            mailbox: mailbox_writer,
            next_epoch: 1,
            envelope,
            retirement_credits,
        },
        RealtimePlanOwner {
            active: (PlanEpoch(0), initial),
            publication: mailbox_reader,
            retirement: retirement_producer,
            carried: 0,
            carry_mismatched: 0,
            _not_sync: Cell::new(()),
        },
        PlanRetirer {
            queue: retirement_consumer,
        },
    ))
}

/// Publish into the held `Empty` cell. Its compare-and-swap cannot fail (the mailbox's I5): the
/// permit proves no cell was `Full`, and only this publisher publishes.
fn publish_into(permit: MailboxPermit<'_, PublishedPlan>, item: PublishedPlan) {
    if permit.commit(item).is_err() {
        panic!("plan mailbox invariant broken: render changed the word while no cell was Full");
    }
}

impl PlanPublisher {
    /// Reserve, then commit, an exact-envelope candidate; the epoch is consumed only on success.
    /// It fails `Full` while a published candidate is unclaimed or no retirement credit is free.
    #[allow(clippy::result_large_err)] // Ownership-preserving backpressure is the public contract.
    pub fn publish(&mut self, plan: PreparedRenderPlan) -> Result<PlanEpoch, PublishError> {
        match self.reserve_replacement(plan) {
            Ok(reservation) => Ok(reservation.commit()),
            Err(
                PlanReplacementReservationError::PublicationFull(plan)
                | PlanReplacementReservationError::RetirementFull(plan),
            ) => Err(PublishError::Full(plan)),
            Err(PlanReplacementReservationError::Incompatible(plan)) => {
                Err(PublishError::Incompatible(plan))
            }
            Err(PlanReplacementReservationError::EpochExhausted(plan)) => {
                Err(PublishError::EpochExhausted(plan))
            }
        }
    }

    /// Reserve the mailbox's `Empty` cell and the eventual displaced-plan retirement before any
    /// caller-owned state becomes visible.  A valid reservation's
    /// [`PlanReplacementReservation::commit`] is non-fallible and publishes exactly once.
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
        let Some(publication) = self.mailbox.try_reserve() else {
            return Err(PlanReplacementReservationError::PublicationFull(plan));
        };
        let Some(credit) = RetirementCredit::try_take(&self.retirement_credits) else {
            return Err(PlanReplacementReservationError::RetirementFull(plan));
        };
        let epoch = PlanEpoch(self.next_epoch);
        Ok(PlanReplacementReservation {
            publication,
            next_epoch: &mut self.next_epoch,
            epoch,
            plan,
            credit,
        })
    }

    /// Take back the last published candidate if render has not claimed it.
    ///
    /// One compare-and-swap marks its cell `Empty`. If it fails, render's claim won; render
    /// claims a candidate at most once, so the one reload settles it. Never retried, never waits.
    pub fn withdraw(&mut self) -> Withdrawal {
        match self.mailbox.withdraw() {
            MailboxWithdrawal::Withdrawn(PublishedPlan {
                epoch,
                plan,
                credit,
            }) => Withdrawal::Withdrawn(UnadoptedCandidate {
                epoch,
                plan,
                credit,
            }),
            MailboxWithdrawal::Taken => Withdrawal::Taken,
            MailboxWithdrawal::Nothing => Withdrawal::Nothing,
        }
    }

    /// Publish a withdrawn candidate again, with its own epoch and retirement credit.
    ///
    /// It cannot fail: the candidate holds the newest epoch this publisher issued, so nothing was
    /// published after its withdrawal and the mailbox has an `Empty` cell.
    ///
    /// # Panics
    ///
    /// If `candidate` was withdrawn from another exchange, or a newer epoch was committed after
    /// it was withdrawn: republishing it then would publish over, or regress past, a newer plan.
    pub fn republish(&mut self, candidate: UnadoptedCandidate) -> PlanEpoch {
        assert!(
            Arc::ptr_eq(&candidate.credit.credits, &self.retirement_credits),
            "an unadopted candidate is republished only through the exchange it left"
        );
        assert_eq!(
            candidate.epoch.0.checked_add(1),
            Some(self.next_epoch),
            "only the newest candidate is republished; a newer epoch was committed after it"
        );
        let Some(permit) = self.mailbox.try_reserve() else {
            panic!("the mailbox holds a candidate while the newest one is withdrawn");
        };
        let epoch = candidate.epoch;
        publish_into(
            permit,
            PublishedPlan {
                epoch,
                plan: candidate.plan,
                credit: candidate.credit,
            },
        );
        epoch
    }

    /// The mailbox's cell states, for tests that assert where a publication landed.
    #[cfg(test)]
    pub(super) fn mailbox_cell_states(&self) -> [super::spsc::MailboxCellState; 2] {
        self.mailbox.cell_states()
    }

    /// Retirement credits not held by a reservation, a candidate or a retired plan.
    #[cfg(test)]
    pub(super) fn free_retirement_credits(&self) -> usize {
        self.retirement_credits.load(Ordering::Acquire)
    }
}

impl PlanReplacementReservation<'_> {
    /// Exact epoch that will become visible if this reservation is committed.
    #[must_use]
    pub const fn epoch(&self) -> PlanEpoch {
        self.epoch
    }

    /// Publish the bound complete candidate. All fallible checks and the credit were taken by the
    /// reservation, so this is a bounded move into the `Empty` cell plus one `Release`
    /// compare-and-swap that cannot fail.
    pub fn commit(self) -> PlanEpoch {
        let Self {
            publication,
            next_epoch,
            epoch,
            plan,
            credit,
        } = self;
        *next_epoch = epoch.0 + 1;
        publish_into(
            publication,
            PublishedPlan {
                epoch,
                plan,
                credit,
            },
        );
        epoch
    }

    /// Cancel without publication and return the complete candidate to the caller; the credit
    /// returns and no epoch is consumed.
    pub fn cancel(self) -> PreparedRenderPlan {
        self.plan
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
    /// Saturating count of applied swaps whose incoming plan took state from its predecessor.
    #[must_use]
    pub const fn carried_count(&self) -> u64 {
        self.carried
    }
    /// Saturating count of applied swaps whose incoming plan asked for state its predecessor's
    /// shape could not supply.
    #[must_use]
    pub const fn carry_mismatch_count(&self) -> u64 {
        self.carry_mismatched
    }
    /// The block-boundary adoption decision: one `Acquire` load, and only if a cell is `Full`,
    /// one compare-and-swap that claims it. A lost claim (control withdrew the candidate after
    /// the load) does nothing; render looks again at the next block and never retries here.
    fn enter_block(&mut self) -> (SwapOutcome, CarryOutcome) {
        let Some(observed) = self.publication.observe() else {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        };
        // Every published candidate holds a retirement credit, so the retirement queue has room
        // for the plan it displaces. The room is taken before the claim, so a claimed candidate
        // is always adopted; were the room ever missing, nothing is claimed and the candidate
        // stays published and withdrawable.
        let Some(placeholder) = self.retirement.try_reserve() else {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        };
        let Some(PublishedPlan {
            epoch,
            plan,
            credit,
        }) = self.publication.claim(observed)
        else {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        };
        let old_epoch = self.active.0;
        let continuing = self.active.1.next_absolute_sample();
        let mut old = core::mem::replace(&mut self.active, (epoch, plan));
        // The timeline belongs to the host, not to any one plan: a plan that takes over mid-stream
        // continues the outgoing plan's clock instead of restarting at zero.
        self.active.1.adopt_absolute_sample(continuing);
        // The incoming plan's one chance to take state from the outgoing one: after the clock
        // adoption, before it renders, and before the outgoing plan leaves for retirement.
        let carry = self.active.1.carry_from(&mut old.1);
        match carry {
            CarryOutcome::Carried => self.carried = self.carried.saturating_add(1),
            CarryOutcome::PredecessorMismatch => {
                self.carry_mismatched = self.carry_mismatched.saturating_add(1);
            }
            CarryOutcome::NotRequested => {}
        }
        placeholder.commit(RetiredPlan {
            epoch: old_epoch,
            plan: old.1,
            credit,
        });
        (SwapOutcome::Applied, carry)
    }
    /// The absolute sample the next contiguous block must start at.
    ///
    /// A plan swap does not reset it: the incoming plan adopts the outgoing plan's clock.
    #[must_use]
    pub fn next_absolute_sample(&self) -> u64 {
        self.active.1.next_absolute_sample()
    }

    /// Publish at the boundary, then render the block that must start at
    /// [`Self::next_absolute_sample`].
    pub fn render_contiguous(
        &mut self,
        io: RenderIo<'_>,
        absolute_sample: u64,
    ) -> Result<RealtimeRenderReport, RenderError> {
        super::audit::in_render_scope(|| {
            let (swap, carry) = self.enter_block();
            let active_epoch = self.active.0;
            let expected = self.active.1.next_absolute_sample();
            if absolute_sample != expected {
                return Err(RenderError::TimeDiscontinuity { expected });
            }
            let render = self
                .active
                .1
                .render_inner(io, RenderTime { absolute_sample })?;
            Ok(RealtimeRenderReport {
                swap,
                carry,
                active_epoch,
                render,
            })
        })
    }
    /// Attempt one block-boundary publication and render through exactly one complete plan.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RealtimeRenderReport, RenderError> {
        super::audit::in_render_scope(|| {
            let (swap, carry) = self.enter_block();
            let active_epoch = self.active.0;
            let render = self.active.1.render_inner(io, time)?;
            Ok(RealtimeRenderReport {
                swap,
                carry,
                active_epoch,
                render,
            })
        })
    }
}
// REALTIME_POLICY_END
impl PlanRetirer {
    /// Reclaim one displaced plan on the control/retirement owner. The plan's slot is free before
    /// its retirement credit returns.
    pub fn try_reclaim(&mut self) -> Result<(PlanEpoch, PreparedRenderPlan), QueueEmpty> {
        self.queue.try_pop().map(|item| {
            let RetiredPlan {
                epoch,
                plan,
                credit,
            } = item;
            drop(credit);
            (epoch, plan)
        })
    }
}
impl Drop for PlanRetirer {
    /// Retired plans that were never reclaimed are destroyed here, on the retirement owner's
    /// thread, not wherever the last queue endpoint happens to drop.
    fn drop(&mut self) {
        while let Ok(retired) = self.queue.try_pop() {
            drop(retired);
        }
    }
}
