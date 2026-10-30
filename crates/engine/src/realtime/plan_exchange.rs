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
//!
//! Every candidate also carries its structural revision (#1482 R1), its `superseded` count and
//! its outcome (#1314 D1) in its payload. Its revisions live on the candidate's own revision gate
//! (#1502 D1), which travels inside the plan: the
//! control thread publishes each committed revision on the gate of the plan that carries its
//! content (`PlanRevisionGate::publish`), and render takes each block's live snapshot from the
//! gate of the plan it runs and advances the applied-revision watermark (`watermark.rs`) to it
//! (D2, D3). The gate orders publications by an internal number that each carries its session
//! revision (#1603); the watermark advances to the snapshot's revision, so a monitor write (a
//! publication that spends no revision) never moves it and it is never above the committed
//! revision. A withdrawn candidate takes its revision along by construction.
//! The mailbox carries no revision.
//!
//! Every candidate is reserved with a [`PlanAdoption`] (#1311): adopt at the next block, no
//! earlier than a scheduled sample, or no earlier than a sample and only once the running plan
//! is ready for a prime. Render decides before it claims: it loads the mailbox word, then the
//! `Full` cell's schedule, asks the running plan's readiness hook for a due `Primed` candidate,
//! and claims only if the block is admitted. A candidate is therefore either unclaimed and
//! withdrawable, or claimed and adopted in the same block; nothing is handed back from render.

use super::spsc::{
    MailboxPermit, MailboxReader, MailboxWithdrawal, MailboxWriter, PlanAdoption,
    bounded_spsc_internal, bounded_spsc_retained_payload, plan_mailbox,
    plan_mailbox_retained_bytes,
};
use super::watermark::{
    CandidateOutcome, PlanWatermark, PlanWatermarkReader, WatermarkWriter, plan_watermark,
    watermark_retained_bytes,
};
use super::{Consumer, GATE_REVISION_MAX, Producer, QueueEmpty, QueueGeneration, SpscError};

/// Epoch assigned to a successfully published render plan.
///
/// #84 phase C: this moved here from the deleted parameter-event store; publication epochs are
/// plan-exchange vocabulary, not parameter vocabulary.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlanEpoch(pub u64);
use super::{
    CarryOutcome, PlanRevisionGate, PreparedRenderPlan, RenderError, RenderIo, RenderReport,
    RenderTime,
};
use core::{
    alloc::Layout,
    cell::Cell,
    fmt,
    num::NonZeroUsize,
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
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
    /// Sum of the mailbox (its state word and two cells with their adoption schedules), the
    /// retirement SPSC header and backing, the shared credit-counter allocation, the
    /// applied-revision watermark record and the shared clock record (#1396 D4).
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

/// The two clocks render publishes after each rendered block (#1396 D4): the active plan's next
/// render sample, and that sample plus the plan's source-read offset. Each is one word, stored
/// `Release` by render and loaded `Acquire` by the control thread, so neither read can tear.
/// The two loads are not a snapshot: render stores the source-read clock and then the render
/// clock, so a reader that needs both together would need them published as one record through
/// `seqlock.rs`; no reader needs that today.
struct PlanClocks {
    render: AtomicU64,
    source_read: AtomicU64,
}

impl PlanClocks {
    /// The clocks of `plan` as it stands: its next render sample and its source-read sample.
    fn of(plan: &PreparedRenderPlan) -> (u64, u64) {
        let render = plan.next_absolute_sample();
        (render, render.saturating_add(plan.source_read_offset()))
    }
}

#[repr(C)]
struct SharedClockAllocation {
    strong: AtomicUsize,
    weak: AtomicUsize,
    clocks: PlanClocks,
}
/// Result of one render-entry swap decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapOutcome {
    /// No candidate was claimed at this boundary: none was published, the published one's
    /// [`PlanAdoption`] does not admit this block yet, or control withdrew it after render's load
    /// (render looks again at the next block).
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

/// A published candidate's payload. `revision`, `superseded` and `outcome` are written only before
/// publication, so they travel in the payload; its revisions are on the plan's own gate, and
/// `revision` is the structural one control stamped there, which the watermark's claim settle
/// reads (#1482 R1).
struct PublishedPlan {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    credit: RetirementCredit,
    revision: u64,
    superseded: u64,
    outcome: CandidateOutcome,
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
    watermark: PlanWatermarkReader,
    clocks: Arc<PlanClocks>,
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
    /// Blocks the executor failed after the live snapshot (#1502 D7), saturating.
    live_block_errors: u64,
    /// Blocks rendered in which a source underran (#1537 D1), saturating, over every plan.
    source_underrun_blocks: u64,
    watermark: WatermarkWriter,
    /// The clock record render stores after each rendered block (#1396 D4).
    clocks: Arc<PlanClocks>,
    /// The strip meter level every block's active plan renders with (#882), `true` by default.
    /// Kept on the owner, not the plan, so an adopted successor inherits it.
    strip_meters: bool,
    /// The strip meter level of a block that adopts a successor which carried no strip meter
    /// (#882), `true` by default: such a swap restarts the host's meter grid at its adoption
    /// block, so the host's level for that block can differ from [`Self::strip_meters`].
    strip_meters_at_restart: bool,
    /// Whether a render call reached the block-boundary adoption decision (#1597): `false`
    /// until the first [`Self::render`] or [`Self::render_contiguous`] call, and so the window in
    /// which [`Self::adopt_before_first_block`] adopts in place.
    entered: bool,
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
    /// The watermark this block published: `Some` exactly on a block that advanced it (#1314 D3),
    /// with the values render just wrote; `None` on every other block (#1349 D1). A host copies
    /// from it and never reads back the record render writes.
    pub watermark: Option<PlanWatermark>,
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
    superseded: u64,
    outcome: CandidateOutcome,
    adoption: PlanAdoption,
    /// The exchange's clocks, read by [`PlanReplacementReservation::render_clock`] (#1325 D2).
    clocks: &'a PlanClocks,
}

/// A published candidate the control thread took back before render claimed it: whole,
/// unrendered, with its epoch, its retirement credit and its [`PlanAdoption`]. Republish it with
/// [`PlanPublisher::republish`], or drop it on the control thread, which returns its credit.
#[must_use]
pub struct UnadoptedCandidate {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    credit: RetirementCredit,
    revision: u64,
    superseded: u64,
    outcome: CandidateOutcome,
    adoption: PlanAdoption,
}

impl UnadoptedCandidate {
    /// Revisions this candidate folds in from candidates it replaced.
    #[must_use]
    pub const fn superseded(&self) -> u64 {
        self.superseded
    }

    /// The outcome the candidate's own revisions complete with.
    #[must_use]
    pub const fn outcome(&self) -> CandidateOutcome {
        self.outcome
    }

    /// Set the outcome the candidate's own revisions complete with (#1314 D1). Its only writer of
    /// another value than `Exact` is the transition fallback (#1397).
    pub fn set_outcome(&mut self, outcome: CandidateOutcome) {
        self.outcome = outcome;
    }

    /// The epoch the candidate was published with; republication keeps it.
    #[must_use]
    pub const fn epoch(&self) -> PlanEpoch {
        self.epoch
    }

    /// When the candidate may be adopted; republication keeps it (#1311 D1).
    #[must_use]
    pub const fn adoption(&self) -> PlanAdoption {
        self.adoption
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
            .field("adoption", &self.adoption)
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
        watermark_retained_bytes(),
        Layout::new::<SharedClockAllocation>().size(),
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
/// envelope, for a host that numbers no revisions: the initial plan's gate stays at revision 0,
/// nothing publishes on it, and the watermark stays `(0, 0, EXACT)`.
pub fn plan_exchange(
    initial: PreparedRenderPlan,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), SpscError> {
    plan_exchange_inner(initial, 0, config)
}

/// Why [`plan_exchange_at_revision`] refused to prepare an exchange. Nothing was allocated for the
/// exchange or published, and `initial` is dropped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanExchangeError {
    /// The mailbox or the retirement queue could not be sized.
    Resources(SpscError),
    /// The initial revision is above [`GATE_REVISION_MAX`], the largest revision a plan's gate can
    /// hold (#1432 D1: bit 63 is render's announcement).
    RevisionAboveCeiling {
        /// The refused revision.
        revision: u64,
    },
}

impl From<SpscError> for PlanExchangeError {
    fn from(error: SpscError) -> Self {
        Self::Resources(error)
    }
}

/// Prepare the exchange with `initial` at committed revision `initial_revision` (#1314 D1, #1502
/// D5): `gate` is the initial plan's handle ([`PreparedRenderPlan::take_revision_gate`]), the
/// revision is published on it when it is above 0, and the watermark starts at
/// `(initial_revision, 0, EXACT)` with every counter 0. The caller keeps the handle: it publishes
/// the plan's later revisions.
///
/// # Panics
///
/// If `gate` is not `initial`'s handle (checked by pointer identity). In a debug build it also
/// panics when `initial_revision` is not above the gate's published revision and is above 0 (the
/// check is `GatePin::publish`'s debug assertion).
///
/// # Errors
///
/// [`PlanExchangeError::RevisionAboveCeiling`] when `initial_revision` is above
/// [`GATE_REVISION_MAX`], checked before the exchange allocates anything (#1503 D3);
/// [`PlanExchangeError::Resources`] when the queue cannot be sized.
pub fn plan_exchange_at_revision(
    initial: PreparedRenderPlan,
    gate: &mut PlanRevisionGate,
    initial_revision: u64,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), PlanExchangeError> {
    assert!(
        initial.owns_revision_gate(gate),
        "the initial revision is published on the initial plan's own gate"
    );
    if initial_revision > GATE_REVISION_MAX {
        return Err(PlanExchangeError::RevisionAboveCeiling {
            revision: initial_revision,
        });
    }
    let exchange = plan_exchange_inner(initial, initial_revision, config)?;
    // Render has not run, so the first snapshot of the gate reads the revision.
    if initial_revision > 0 {
        gate.publish(initial_revision);
    }
    Ok(exchange)
}

fn plan_exchange_inner(
    initial: PreparedRenderPlan,
    initial_revision: u64,
    config: PlanExchangeConfig,
) -> Result<(PlanPublisher, RealtimePlanOwner, PlanRetirer), SpscError> {
    let _resources = plan_exchange_resource_report(config)?;
    let envelope = initial.envelope();
    let retirement_credits = Arc::new(AtomicUsize::new(config.retirement_capacity.get()));
    let (retirement_producer, retirement_consumer) =
        bounded_spsc_internal(config.retirement_capacity, QueueGeneration(2))?;
    let (mailbox_writer, mailbox_reader) = plan_mailbox();
    let (watermark_writer, watermark_reader) = plan_watermark(initial_revision);
    // #1396 D4: before the first block both clocks read the initial plan's values.
    let (render, source_read) = PlanClocks::of(&initial);
    let clocks = Arc::new(PlanClocks {
        render: AtomicU64::new(render),
        source_read: AtomicU64::new(source_read),
    });
    Ok((
        PlanPublisher {
            mailbox: mailbox_writer,
            next_epoch: 1,
            envelope,
            retirement_credits,
            watermark: watermark_reader,
            clocks: Arc::clone(&clocks),
        },
        RealtimePlanOwner {
            active: (PlanEpoch(0), initial),
            publication: mailbox_reader,
            retirement: retirement_producer,
            carried: 0,
            carry_mismatched: 0,
            live_block_errors: 0,
            source_underrun_blocks: 0,
            watermark: watermark_writer,
            clocks,
            strip_meters: true,
            strip_meters_at_restart: true,
            entered: false,
            _not_sync: Cell::new(()),
        },
        PlanRetirer {
            queue: retirement_consumer,
        },
    ))
}

/// Publish into the held `Empty` cell, with `adoption` in its schedule. Its compare-and-swap
/// cannot fail (the mailbox's I5): the permit proves no cell was `Full`, and only this publisher
/// publishes.
fn publish_into(
    permit: MailboxPermit<'_, PublishedPlan>,
    item: PublishedPlan,
    adoption: PlanAdoption,
) {
    if permit.commit(item, adoption).is_err() {
        panic!("plan mailbox invariant broken: render changed the word while no cell was Full");
    }
}

impl PlanPublisher {
    /// Reserve, then commit, an exact-envelope candidate for adoption at the next block
    /// ([`PlanAdoption::Next`]); the epoch is consumed only on success. It fails `Full` while a
    /// published candidate is unclaimed or no retirement credit is free. `revision` is the
    /// candidate's own structural revision, as [`PlanReplacementReservation::commit`] takes it.
    #[allow(clippy::result_large_err)] // Ownership-preserving backpressure is the public contract.
    pub fn publish(
        &mut self,
        plan: PreparedRenderPlan,
        revision: u64,
    ) -> Result<PlanEpoch, PublishError> {
        match self.reserve_replacement(plan, PlanAdoption::Next) {
            Ok(reservation) => Ok(reservation.commit(revision)),
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
    ///
    /// `adoption` fixes when render may adopt the candidate (#1311 D1): render claims it at the
    /// first block that [`PlanAdoption`] admits, and until then it stays published and
    /// withdrawable.
    #[allow(clippy::result_large_err)] // Every failure returns the complete candidate.
    pub fn reserve_replacement(
        &mut self,
        plan: PreparedRenderPlan,
        adoption: PlanAdoption,
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
            superseded: 0,
            outcome: CandidateOutcome::Exact,
            adoption,
            clocks: &self.clocks,
        })
    }

    /// A reader of the applied-revision watermark render publishes (#1314 D4).
    #[must_use]
    pub fn watermark_reader(&self) -> PlanWatermarkReader {
        self.watermark.clone()
    }

    /// The running plan's render clock (#1396 D4): the absolute sample its next block renders
    /// from, as render stored it after its last rendered block, or the initial plan's before the
    /// first. One `Acquire` load of one word. The warm-successor deadline reads this clock.
    #[must_use]
    pub fn render_clock(&self) -> u64 {
        self.clocks.render.load(Ordering::Acquire)
    }

    /// The running plan's source-read clock (#1396 D4): the render clock plus the plan's
    /// source-read offset ([`PreparedRenderPlan::source_read_offset`]), as render stored it after
    /// its last rendered block, or the initial plan's before the first. One `Acquire` load of one
    /// word. A host anchors a seek on this clock.
    #[must_use]
    pub fn source_read_clock(&self) -> u64 {
        self.clocks.source_read.load(Ordering::Acquire)
    }

    /// Take back the last published candidate if render has not claimed it.
    ///
    /// One compare-and-swap marks its cell `Empty`. If it fails, render's claim won; render
    /// claims a candidate at most once, so the one reload settles it. Never retried, never waits.
    pub fn withdraw(&mut self) -> Withdrawal {
        match self.mailbox.withdraw() {
            MailboxWithdrawal::Withdrawn(
                PublishedPlan {
                    epoch,
                    plan,
                    credit,
                    revision,
                    superseded,
                    outcome,
                },
                adoption,
            ) => Withdrawal::Withdrawn(UnadoptedCandidate {
                epoch,
                plan,
                credit,
                revision,
                superseded,
                outcome,
                adoption,
            }),
            MailboxWithdrawal::Taken => Withdrawal::Taken,
            MailboxWithdrawal::Nothing => Withdrawal::Nothing,
        }
    }

    /// Publish a withdrawn candidate again, with its own epoch, retirement credit and
    /// [`PlanAdoption`] (#1311 D1).
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
                revision: candidate.revision,
                superseded: candidate.superseded,
                outcome: candidate.outcome,
            },
            candidate.adoption,
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

    /// When the candidate may be adopted, fixed at reservation (#1311 D1).
    #[must_use]
    pub const fn adoption(&self) -> PlanAdoption {
        self.adoption
    }

    /// The bound candidate, unpublished (#1310 D4): a control-side check reads it before the
    /// commit that decides whether it is published.
    #[must_use]
    pub const fn plan(&self) -> &PreparedRenderPlan {
        &self.plan
    }

    /// The bound candidate, unpublished and owned by this control thread (#1310 D5): a source
    /// donation moves rings into it after the protocol commit and before [`Self::commit`]
    /// publishes it. Nothing here changes its envelope, which the reservation checked.
    pub fn plan_mut(&mut self) -> &mut PreparedRenderPlan {
        &mut self.plan
    }

    /// Set how many revisions the candidate folds in from candidates it replaced (#1314 D1,
    /// #1310): its adoption completes them as `superseded`.
    pub fn set_superseded(&mut self, superseded: u64) {
        self.superseded = superseded;
    }

    /// Set the outcome the candidate's own revisions complete with (#1314 D1). Its only writer of
    /// another value than `Exact` is the transition fallback (#1397).
    pub fn set_outcome(&mut self, outcome: CandidateOutcome) {
        self.outcome = outcome;
    }

    /// Replace the schedule given at reservation (#1325 D3): a strip transition reads the render
    /// clock only after its phase-1 writes, which follow the reservation, so its `S` is known
    /// last. Control thread only, before [`Self::commit`], which publishes it with the candidate.
    pub fn set_adoption(&mut self, adoption: PlanAdoption) {
        self.adoption = adoption;
    }

    /// [`PlanPublisher::render_clock`], read while the reservation borrows the publisher: a strip
    /// transition reads `p` after its phase-1 writes, which follow the reservation (#1325 D2).
    #[must_use]
    pub fn render_clock(&self) -> u64 {
        self.clocks.render.load(Ordering::Acquire)
    }

    /// Publish the bound complete candidate. All fallible checks and the credit were taken by the
    /// reservation, so this is a bounded move into the `Empty` cell plus one `Release`
    /// compare-and-swap that cannot fail.
    ///
    /// `revision` is the candidate's own structural revision, the one the control plane
    /// publishes on its gate (#1482 R1). It travels in the payload: when render claims a newer
    /// candidate before the watermark advances past this one, the revisions up to it complete
    /// with this candidate's outcome.
    pub fn commit(self, revision: u64) -> PlanEpoch {
        let Self {
            publication,
            next_epoch,
            epoch,
            plan,
            credit,
            superseded,
            outcome,
            adoption,
            clocks: _,
        } = self;
        *next_epoch = epoch.0 + 1;
        publish_into(
            publication,
            PublishedPlan {
                epoch,
                plan,
                credit,
                revision,
                superseded,
                outcome,
            },
            adoption,
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
    /// The active plan's host attachment as a `T` (issue #1381 D3), or `None` when the plan
    /// carries none or carries another type.
    ///
    /// It is a downcast of the attachment [`PreparedRenderPlan::set_host_attachment`] stored before
    /// publication: no allocation, no lock and no drop, so a render-side reader may call it. The
    /// attachment becomes the active one at the same block as its plan, because it is part of
    /// the plan; a block whose swap decision returned [`SwapOutcome::Applied`] therefore reads the
    /// successor's attachment.
    pub fn active_attachment_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.active.1.host_attachment_mut::<T>()
    }
    /// The currently active complete plan, for the plan's exclusive owner between render calls
    /// (#1642): a test reads the plan's own state, such as its timeline, through it. Nothing here
    /// adopts, swaps or renders; the borrow ends before the next render call. Test support only
    /// (root ruling B16-R8): no production caller needs `&mut` access to the active plan.
    #[cfg(feature = "test-support")]
    pub fn active_plan_mut(&mut self) -> &mut PreparedRenderPlan {
        &mut self.active.1
    }
    /// Whether the active plan carried a strip meter from its predecessor at its swap block
    /// (#882): the fact the restart level ([`Self::set_strip_meter_level_at_restart`]) reads, for
    /// a host that keeps its own copy to check against. `false` for a plan that never swapped in.
    #[must_use]
    pub const fn active_strip_meters_carried(&self) -> bool {
        self.active.1.strip_meters_carried()
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
    /// Saturating count of blocks the executor failed after the block's live snapshot (#1502 D7).
    ///
    /// Such a block may have applied live values and rendered no output, so it is not atomic. No
    /// valid plan reaches it (`InvalidEnvelope` and `Buffer` are invariant failures); it is a
    /// counted defect path with no recovery. The watermark does not advance on such a block.
    #[must_use]
    pub const fn live_block_errors(&self) -> u64 {
        self.live_block_errors
    }
    /// Saturating count of blocks rendered successfully in which a source played zeros because
    /// the host had not delivered PCM it owed (#1537 D1): graph-wide, one per block, never reset.
    /// A failed block, a held block and a block past the end of a region add nothing; a plan swap
    /// does not reset it.
    #[must_use]
    pub const fn source_underrun_blocks(&self) -> u64 {
        self.source_underrun_blocks
    }
    /// Set the strip meter level the next blocks render with (#882): the active plan receives it
    /// at each block after the adoption decision, so a successor adopted at a later block renders
    /// with it too. `false` skips every strip meter observer
    /// ([`PreparedRenderPlan::set_strip_meter_level`]). One store; called between blocks by the
    /// plan's owner.
    pub fn set_strip_meter_level(&mut self, on: bool) {
        self.strip_meters = on;
    }
    /// Set the strip meter level of a block that applies a swap whose successor carried no strip
    /// meter (no [`super::ObserverKind::Meter`] entry in its
    /// [`PreparedRenderPlan::observer_carries`], #882). Such a successor's meters open their first
    /// window at the adoption block, and a host restarts its meter grid there, so that block
    /// follows this level instead of [`Self::set_strip_meter_level`]'s. One store; called between
    /// blocks by the plan's owner.
    pub fn set_strip_meter_level_at_restart(&mut self, on: bool) {
        self.strip_meters_at_restart = on;
    }
    /// The strip meter level of the block that just entered with `swap`: the restart level when
    /// the swap applied and its successor carried no strip meter, the steady level otherwise.
    fn strip_meter_level(&self, swap: SwapOutcome) -> bool {
        if swap == SwapOutcome::Applied && !self.active.1.strip_meters_carried() {
            self.strip_meters_at_restart
        } else {
            self.strip_meters
        }
    }
    /// The block-boundary adoption decision for the block that starts at `block_start`: one
    /// `Acquire` load and, only if a cell is `Full`, the claim rule (#1311 D2), then one
    /// compare-and-swap that claims it.
    ///
    /// Render decides before it claims: it loads the `Full` cell's [`PlanAdoption`], and for a
    /// `Primed` candidate whose `not_before` the block has reached asks the **running** plan's
    /// readiness hook. It claims only if the schedule admits the block, so a candidate that is
    /// not due or not ready stays published and withdrawable. A claimed candidate is adopted here,
    /// before the block renders (D4). A lost claim (control withdrew the candidate after the load,
    /// perhaps republishing another one whose schedule or readiness render read) does nothing;
    /// render looks again at the next block and never retries here.
    fn enter_block(&mut self, block_start: u64) -> (SwapOutcome, CarryOutcome) {
        let Some(observed) = self.publication.observe() else {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        };
        let running = &self.active.1;
        let admitted = self
            .publication
            .scheduled(observed)
            .admits(block_start, |start, lead_blocks| {
                running.prime_ready(start, lead_blocks)
            });
        if !admitted {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        }
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
            revision,
            superseded,
            outcome,
        }) = self.publication.claim(observed)
        else {
            return (SwapOutcome::None, CarryOutcome::NotRequested);
        };
        self.watermark.note_claim(revision, superseded, outcome);
        let old_epoch = self.active.0;
        let continuing = self.active.1.next_absolute_sample();
        let mut old = core::mem::replace(&mut self.active, (epoch, plan));
        // The timeline belongs to the host, not to any one plan: a plan that takes over mid-stream
        // continues the outgoing plan's clock instead of restarting at zero.
        self.active.1.adopt_absolute_sample(continuing);
        // The incoming plan's one chance to take state from the outgoing one: after the clock
        // adoption, before it renders, and before the outgoing plan leaves for retirement.
        let carry = self.active.1.carry_from(&mut old.1, block_start);
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
    /// Render the block through the active plan, after the adoption decision. The plan takes the
    /// block's live snapshot `S` from its own gate and reports it; the watermark advances to `S`
    /// only once the block rendered (#1502 D2, D3). A block the executor failed after the snapshot
    /// is counted and publishes nothing (D7).
    fn render_active(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
        swap: SwapOutcome,
    ) -> Result<(RenderReport, Option<PlanWatermark>), RenderError> {
        let strip_meters = self.strip_meter_level(swap);
        self.active.1.set_strip_meter_level(strip_meters);
        match self.active.1.render_inner(io, time) {
            Ok(render) => {
                let watermark = self
                    .watermark
                    .advance(render.live_snapshot.revision(), time.absolute_sample);
                // #1396 D4: the plan that rendered the block, after it rendered: two stores.
                let (render_clock, source_read_clock) = PlanClocks::of(&self.active.1);
                self.clocks
                    .source_read
                    .store(source_read_clock, Ordering::Release);
                self.clocks.render.store(render_clock, Ordering::Release);
                if render.source_underrun {
                    self.source_underrun_blocks = self.source_underrun_blocks.saturating_add(1);
                }
                Ok((render, watermark))
            }
            Err(failure) => {
                if failure.after_snapshot {
                    self.live_block_errors = self.live_block_errors.saturating_add(1);
                }
                Err(failure.error)
            }
        }
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
            // The block can only start where the running plan's clock stands, which an adopted
            // plan continues. A call at another sample renders nothing, so the clock, not the
            // argument, decides the adoption; a candidate due by the clock is still adopted here,
            // and its first rendered block starts at the clock.
            self.entered = true;
            let block_start = self.active.1.next_absolute_sample();
            let (swap, carry) = self.enter_block(block_start);
            let active_epoch = self.active.0;
            let expected = self.active.1.next_absolute_sample();
            if absolute_sample != expected {
                return Err(RenderError::TimeDiscontinuity { expected });
            }
            let (render, watermark) =
                self.render_active(io, RenderTime { absolute_sample }, swap)?;
            Ok(RealtimeRenderReport {
                swap,
                carry,
                active_epoch,
                render,
                watermark,
            })
        })
    }
    /// [`PreparedRenderPlan::copy_response_snapshot`] on the active plan, naming that plan.
    ///
    /// Valid between blocks, on the thread that renders (#1577 calls it after a block): it
    /// allocates nothing itself (the sink owns all storage) and neither renders, advances the
    /// clock nor adopts a successor. The
    /// returned `active_epoch` is the epoch of the plan whose retained values were copied.
    ///
    /// # Errors
    ///
    /// The active plan's typed refusal.
    pub fn copy_response_snapshot(
        &mut self,
        request: super::ResponseSnapshotRequest<'_>,
    ) -> Result<RealtimeResponseSnapshot, super::ResponseSnapshotError> {
        let capture = self.active.1.copy_response_snapshot(request)?;
        Ok(RealtimeResponseSnapshot {
            active_epoch: self.active.0,
            captured_sample: capture.captured_sample,
            owners: capture.owners,
        })
    }
    /// Attempt one block-boundary publication and render through exactly one complete plan.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RealtimeRenderReport, RenderError> {
        super::audit::in_render_scope(|| {
            // The host's explicit time, not the plan's own clock, is the block's start.
            self.entered = true;
            let (swap, carry) = self.enter_block(time.absolute_sample);
            let active_epoch = self.active.0;
            let (render, watermark) = self.render_active(io, time, swap)?;
            Ok(RealtimeRenderReport {
                swap,
                carry,
                active_epoch,
                render,
                watermark,
            })
        })
    }
}
// REALTIME_POLICY_END
/// The active plan's exclusive-owner operations between blocks (issue #1381a).
///
/// A host that renders on one thread, and runs its control work on that thread between blocks,
/// calls these on the owner of the plan: they are the plan's own operations, forwarded to the plan
/// the owner holds. Each one does what the same-named method of [`PreparedRenderPlan`] documents.
/// None renders, advances the clock or changes the plan's topology, and none is called inside a
/// render call.
impl RealtimePlanOwner {
    /// Adopt the published candidate in place, before any block (#1597): the adoption decision
    /// the first render call would make at the plan's clock, made now, on the owner's thread, by
    /// the host that holds the owner and has not rendered through it. The candidate's state is
    /// carried from the never-rendered plan it replaces, which goes to the retirement queue, so
    /// the first block renders the candidate with no swap: the outcome the first plan would have
    /// had, had it been prepared as the candidate was.
    ///
    /// Returns [`SwapOutcome::Applied`] when it adopted, and [`SwapOutcome::None`] with nothing
    /// changed when a render call has already reached the owner, no candidate is published, or
    /// the candidate's schedule does not admit the plan's clock. It allocates nothing.
    pub fn adopt_before_first_block(&mut self) -> SwapOutcome {
        if self.entered {
            return SwapOutcome::None;
        }
        let block_start = self.active.1.next_absolute_sample();
        self.enter_block(block_start).0
    }

    /// [`PreparedRenderPlan::can_prepare_source_seek`] on the active plan.
    #[must_use]
    pub fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.active.1.can_prepare_source_seek(source_index)
    }

    /// [`PreparedRenderPlan::prepare_source_seek`] on the active plan.
    pub fn prepare_source_seek(
        &mut self,
        source_index: usize,
        generation: u64,
        frame: u64,
    ) -> bool {
        self.active
            .1
            .prepare_source_seek(source_index, generation, frame)
    }

    /// [`PreparedRenderPlan::prepare_admitted_seeks`] on the active plan (#2213). A published
    /// candidate is not prepared: its consumers apply their seeks at the block that adopts it.
    pub fn prepare_admitted_seeks(&mut self) -> usize {
        self.active.1.prepare_admitted_seeks()
    }
}
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
