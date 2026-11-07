//! Publish a warm successor `Primed` from the control plane (issue #1403 D3, D4; #1397 D1), and
//! fall back to the transition when it is not ready by its deadline (#1358 D1, D3).
//!
//! A latency-growing structural edit is prepared warm (#1354) and published
//! [`PlanAdoption::Primed`]: render adopts it at the first block at or past `not_before` at which
//! the running plan reports itself ready for the prime (#1311, #1355). Readiness asks the
//! `prime_required` flag of each ring the running plan renders (#1320 D4), so the publication
//! writes those flags first. [`publish_primed`] is that step, [`hold_primed_until`] raises its
//! `not_before`, and [`PrimedCandidate`] is the record the control plane keeps of the candidate,
//! read from its reservation, until it is adopted, superseded or withdrawn.
//! [`check_prime_deadline`] decides, per control call, whether that record's deadline
//! ([`prime_deadline_samples`]) has passed.

use engine::realtime::{PlanAdoption, PlanPublisher, PlanReplacementReservation};

use session::SessionModel;

use crate::prepare::{
    PlanStateInventory, SOURCE_STALL_TOLERANCE_MS, SuccessorBase, WarmConfig, p_max_samples,
};
use crate::source::SourceControlSet;

/// The largest prime a warm successor may run in its adoption block, in bytes (#1360 D1): the
/// bound [`WarmConfig::prime_bytes_max`] of every product session.
///
/// The value is *Record the swap block's cost on the 64-track console* (#1286) D3 item 3, record
/// `artifacts/steps/swap-carry-base/swap-benchmark.accepted.jsonl`, round 1, the smaller of its two
/// rounds (root ruling). Inputs: the ordinary block `o` 177,157 ns, the adoption block of the
/// `ΣP = P_MAX` growth `a_P` 625,831 ns with its prime `b_P` 2,072,576 bytes, so `β = (a_P − o) /
/// b_P` 216.481 ps per byte, and the margin `m` 0.7 of the 96 kHz, 128-frame quantum `D`. The
/// native row binds: `floor((m · D − o) / β)`, below the browser row's 7,012,421.
pub const PRIME_BYTES_MAX: u64 = 3_493_032;

/// The warm-successor bounds of every product session at `sample_rate_hz` and `quantum_frames`
/// (#1360 D1): `P_MAX` ([`p_max_samples`], #1406 D1) and [`PRIME_BYTES_MAX`]. Both hosts get it
/// through the control plane's public session compile.
#[must_use]
pub const fn production_config(sample_rate_hz: u32, quantum_frames: u32) -> WarmConfig {
    WarmConfig {
        p_max: p_max_samples(sample_rate_hz, quantum_frames) as u64,
        prime_bytes_max: PRIME_BYTES_MAX,
    }
}

/// The record of the one warm candidate published [`PlanAdoption::Primed`] and not yet adopted,
/// superseded or withdrawn (#1403 D4).
///
/// It is read from the candidate's own reservation ([`Self::of`]) once its schedule is final, and
/// nothing else makes one, so a record always names the epoch and the schedule its reservation
/// publishes (#2258 PR-7). The control plane keeps it on the candidate's pending epoch, which
/// drops it when render adopts the candidate or a supersession drops the candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimedCandidate {
    epoch: u64,
    not_before: u64,
    lead_blocks: u32,
}

impl PrimedCandidate {
    /// The record of the candidate `reservation` publishes, or `None` unless its adoption is
    /// [`PlanAdoption::Primed`]. Read it after the last schedule write ([`publish_primed`], then
    /// [`hold_primed_until`]) and before the commit, so it is what the commit publishes. Control
    /// thread only; allocation-free.
    #[must_use]
    pub const fn of(reservation: &PlanReplacementReservation<'_>) -> Option<Self> {
        match reservation.adoption() {
            PlanAdoption::Primed {
                not_before,
                lead_blocks,
            } => Some(Self {
                epoch: reservation.epoch().0,
                not_before,
                lead_blocks,
            }),
            _ => None,
        }
    }

    /// The candidate's plan epoch.
    #[must_use]
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }

    /// The earliest block start that may adopt it: the render clock read at publication, or the
    /// duck's `S` when the transaction ducks strips ([`hold_primed_until`]).
    #[must_use]
    pub const fn not_before(&self) -> u64 {
        self.not_before
    }

    /// The prime its adopting block runs, in blocks: `P / q`.
    #[must_use]
    pub const fn lead_blocks(&self) -> u32 {
        self.lead_blocks
    }
}

/// Hold the warm candidate `reservation` publishes until the block that starts at `not_before` too
/// (#1397 D1): a warm candidate whose transaction ducks strips is adopted at the first ready block
/// at or after the duck's `S` (#1325 D3), so every ducked strip and every route line out of it
/// holds only `+0.0` when render adopts it. Raises the `not_before` of the reservation's
/// [`PlanAdoption::Primed`] adoption to `not_before` (never lowers it) and keeps its lead. It acts
/// on the reservation alone, so it cannot meet another candidate's record (#2258 PR-7); a
/// reservation that is not `Primed` is not a warm candidate and is left as it is. Call it after
/// [`publish_primed`], on the same reservation, before the commit. Control thread only;
/// allocation-free.
pub fn hold_primed_until(reservation: &mut PlanReplacementReservation<'_>, not_before: u64) {
    if let PlanAdoption::Primed {
        not_before: published,
        lead_blocks,
    } = reservation.adoption()
    {
        reservation.set_adoption(PlanAdoption::Primed {
            not_before: published.max(not_before),
            lead_blocks,
        });
    }
}

/// Write the `prime_required` flags of a warm candidate W and schedule its reservation
/// [`PlanAdoption::Primed`] (#1403 D3, amended by the scope note's R1: the publication goes
/// through the rebuild's own reservation, which the caller then commits).
///
/// Call it after the transaction's last fallible step. Every ring reads `true` exactly when W
/// carries it ([`SourceControlSet::carries`], fixed when W was prepared), wherever its producer is,
/// so the flags are the same whether W's set has taken its producers
/// ([`SourceControlSet::adopt_persisting`]) or a donated ring yet or not (#2273). Each ring's
/// producer is in one of the three sets:
///
/// 1. `running_sources`, the plan W displaces, which render runs: `true` if W carries the ring.
/// 2. `withdrawn_sources`, a candidate W supersedes (#1310): a ring it carried from the running
///    plan and W carries on reads `true`; one W removes, and every ring it created, reads `false`,
///    so no `true` written for it outlives it.
/// 3. `candidate_sources`, W's own: a ring W carries, once its producer has moved in, reads
///    `true`; every ring W created reads `false`.
///
/// It then reads `not_before` from the render clock and sets the reservation's adoption to
/// `Primed { not_before, lead_blocks }`, which [`PrimedCandidate::of`] reads back. The caller
/// publishes only into an `Empty` cell (#1343 D3), so no `Primed` candidate is `Full` while the
/// flags are written, and render reads them only for a `Full` `Primed` cell after its `Acquire`
/// of the mailbox, which the commit's `Release` orders after these stores. Control thread only;
/// allocation-free.
pub fn publish_primed(
    reservation: &mut PlanReplacementReservation<'_>,
    running_sources: &SourceControlSet,
    candidate_sources: &SourceControlSet,
    withdrawn_sources: Option<&SourceControlSet>,
    lead_blocks: u32,
) {
    let carried = |id: &[u8]| candidate_sources.carries(id);
    running_sources.set_prime_flags(carried);
    if let Some(withdrawn) = withdrawn_sources {
        withdrawn.set_prime_flags(carried);
    }
    candidate_sources.set_prime_flags(carried);
    let not_before = reservation.render_clock();
    reservation.set_adoption(PlanAdoption::Primed {
        not_before,
        lead_blocks,
    });
}

/// The transition fallback's re-preparation (#1397 D2 step 1): the one host-core entry a host
/// runs when a latency growth cannot be adopted warm, at submit (`warm_lead` answered
/// `Unavailable`) or after a deadline step withdrew the `Primed` candidate (the donor, #1358 D3).
///
/// `ordinary` is the committed model `next` prepared against `base` as an ordinary successor
/// (#1285 floors, no lead); `inventory` reads its inventory. The duck set is
/// [`crate::grown_strips`] over that compile, computed before `ordinary` drops and before any
/// carry join reads it; `prepare` then prepares `next` against `base` with the set as
/// [`SuccessorBase::forced_restart`], so each strip in it is restarted whole and armed and listed
/// in the result's `restarted_strips()`, which is the set the transition ducks. Across a
/// withdrawn donor, `base` is the base of a successor of the plan the donor succeeds (#1344 D2:
/// its inventory across the withdrawn candidate, and the running epoch's kept model with its duck
/// overlay), and the host applies the donation only after this returns `Ok` (D2 step 2).
///
/// The host publishes the result as a duck-swap with `S` and writes
/// [`engine::realtime::CandidateOutcome::TransitionFallback`] on its reservation before the
/// commit (D2 step 5, D5): its publication is the host's one rebuild path. Control thread only.
///
/// # Errors
///
/// `prepare`'s error, with nothing published: at submit the edit is refused before its commit;
/// across a donor the host keeps the donor whole (D2 step 3).
pub fn fall_back_to_transition<T, E>(
    base: SuccessorBase<'_>,
    next: &SessionModel,
    ordinary: T,
    inventory: fn(&T) -> &PlanStateInventory,
    prepare: impl FnOnce(SuccessorBase<'_>) -> Result<T, E>,
) -> Result<T, E> {
    let grown = crate::grown_strips(base.inventory, base.committed, next, inventory(&ordinary));
    drop(ordinary);
    prepare(SuccessorBase {
        forced_restart: &grown,
        ..base
    })
}

/// How long render may run past a warm candidate's `not_before` without adopting it before the
/// control plane withdraws it and takes the transition, in render samples (#1358 D1):
/// `T(fs, q) + p_max_samples(fs, q) + q`, with `T(fs, q) = ceil_q(fs · SOURCE_STALL_TOLERANCE_MS /
/// 1000)`, one stall tolerance in whole quanta (not [`crate::stall_ring_frames`], which adds the
/// consumer's and the recycle path's quanta). 7,040 at 48 kHz and quantum 128.
///
/// A host that keeps its rings full holds `P_MAX + q` frames past each consumer through any
/// producer stall up to the tolerance (#1406 D2), so only a held seek or command inside the prime
/// window delays readiness: it applies within `P + q <= P_MAX + q` samples, and the host then has
/// one stall tolerance to queue the new generation. A zero quantum returns zero; callers validate
/// the document first.
#[must_use]
pub const fn prime_deadline_samples(sample_rate_hz: u32, quantum_frames: u32) -> u64 {
    if quantum_frames == 0 {
        return 0;
    }
    let quantum = quantum_frames as u64;
    let tolerance = (sample_rate_hz as u64 * SOURCE_STALL_TOLERANCE_MS as u64) / 1_000;
    tolerance.div_ceil(quantum) * quantum
        + p_max_samples(sample_rate_hz, quantum_frames) as u64
        + quantum
}

/// What [`check_prime_deadline`] found for a warm candidate's record (#1358 D3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeadlineStep {
    /// The render clock has not reached `not_before + prime_deadline_samples`: do nothing. A
    /// render clock that does not move (a paused host) never passes it.
    Pending,
    /// The deadline has passed: withdraw the candidate once and take the transition if render
    /// has not adopted it.
    Due,
}

/// The deadline step's decision for the record `pending` (#1358 D3): [`DeadlineStep::Pending`]
/// while `publisher.render_clock()` is below `pending.not_before() +
/// prime_deadline_samples(sample_rate_hz, quantum_frames)`, else [`DeadlineStep::Due`].
///
/// The caller passes the record its newest pending candidate holds, so the record always names
/// the candidate a `Due` step withdraws (#2258 PR-7: the record lives on that candidate's epoch,
/// and a record of any other epoch cannot be held).
///
/// It reads one clock and acts on nothing: the caller runs the one withdrawal of a `Due` step
/// through its own withdrawal path, which also promotes a candidate render took (`Taken`), and
/// hands a withdrawn candidate of the record's epoch to the transition fallback as its donor
/// ([`fall_back_to_transition`]). Control thread only; allocation-free; never waits on render.
#[must_use]
pub fn check_prime_deadline(
    publisher: &PlanPublisher,
    pending: &PrimedCandidate,
    sample_rate_hz: u32,
    quantum_frames: u32,
) -> DeadlineStep {
    let deadline = pending
        .not_before
        .saturating_add(prime_deadline_samples(sample_rate_hz, quantum_frames));
    if publisher.render_clock() < deadline {
        DeadlineStep::Pending
    } else {
        DeadlineStep::Due
    }
}
