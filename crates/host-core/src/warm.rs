//! Publish a warm successor `Primed` from the control plane (issue #1403 D3, D4; #1397 D1).
//!
//! A latency-growing structural edit is prepared warm (#1354) and published
//! [`PlanAdoption::Primed`]: render adopts it at the first block at or past `not_before` at which
//! the running plan reports itself ready for the prime (#1311, #1355). Readiness asks the
//! `prime_required` flag of each ring the running plan renders (#1320 D4), so the publication
//! writes those flags first. [`publish_primed`] is that step, and [`PrimedCandidate`] is the record
//! the control plane keeps of the candidate until it is adopted, superseded or withdrawn.

use engine::realtime::{PlanAdoption, PlanReplacementReservation};

use crate::source::SourceControlSet;

/// The record of the one warm candidate published [`PlanAdoption::Primed`] and not yet adopted,
/// superseded or withdrawn (#1403 D4). Every step that reads it compares [`Self::epoch`] with the
/// candidate it acts on, so a step never acts on a candidate the record does not name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimedCandidate {
    /// The candidate's plan epoch.
    pub epoch: u64,
    /// The earliest block start that may adopt it: the render clock read at publication, or the
    /// duck's `S` when the transaction ducks strips ([`Self::hold_until`]).
    pub not_before: u64,
    /// The prime its adopting block runs, in blocks: `P / q`.
    pub lead_blocks: u32,
}

impl PrimedCandidate {
    /// Hold the candidate until the block that starts at `not_before` too (#1397 D1): a warm
    /// candidate whose transaction ducks strips is adopted at the first ready block at or after
    /// the duck's `S` (#1325 D3), so every ducked strip and every route line out of it holds only
    /// `+0.0` when render adopts it. Raises [`Self::not_before`] to `not_before` (never lowers it)
    /// and sets the reservation's adoption to `Primed` with the result. Call it after
    /// [`publish_primed`], on the same reservation, before the commit. Control thread only;
    /// allocation-free.
    pub fn hold_until(
        &mut self,
        reservation: &mut PlanReplacementReservation<'_>,
        not_before: u64,
    ) {
        debug_assert_eq!(
            reservation.epoch().0,
            self.epoch,
            "the record names this candidate"
        );
        self.not_before = self.not_before.max(not_before);
        reservation.set_adoption(PlanAdoption::Primed {
            not_before: self.not_before,
            lead_blocks: self.lead_blocks,
        });
    }
}

/// Write the `prime_required` flags of a warm candidate W and schedule its reservation
/// [`PlanAdoption::Primed`] (#1403 D3, amended by the scope note's R1: the publication goes
/// through the rebuild's own reservation, which the caller then commits).
///
/// Call it after the transaction's last fallible step and **before** W's set takes any producer
/// ([`SourceControlSet::adopt_persisting`]) or donated ring: at that point an entry W holds vacant
/// is exactly a ring W carries from the plan it displaces, and each ring's producer is still in
/// the set of the plan that holds it.
///
/// 1. Every ring whose producer `running_sources` holds (the plan W displaces, which render runs)
///    reads `true` if W carries it (W holds the source vacant) and `false` otherwise.
/// 2. Every ring whose producer `withdrawn_sources` holds (a candidate W supersedes, #1310) reads
///    the same rule: a ring it carried from the running plan and W carries on reads `true`; one W
///    removes, and every ring it created, reads `false`, so no `true` written for it outlives it.
/// 3. Every ring W created reads `false`.
///
/// It then reads `not_before` from the render clock and sets the reservation's adoption to
/// `Primed { not_before, lead_blocks }`. The caller publishes only into an `Empty` cell (#1343 D3),
/// so no `Primed` candidate is `Full` while the flags are written, and render reads them only
/// for a `Full` `Primed` cell after its `Acquire` of the mailbox, which the commit's `Release`
/// orders after these stores. Control thread only; allocation-free.
pub fn publish_primed(
    reservation: &mut PlanReplacementReservation<'_>,
    running_sources: &SourceControlSet,
    candidate_sources: &SourceControlSet,
    withdrawn_sources: Option<&SourceControlSet>,
    lead_blocks: u32,
) -> PrimedCandidate {
    let carried = |id: &[u8]| candidate_sources.holds_vacant(id);
    running_sources.set_prime_flags(carried);
    if let Some(withdrawn) = withdrawn_sources {
        withdrawn.set_prime_flags(carried);
    }
    candidate_sources.set_prime_flags(|_| false);
    let not_before = reservation.render_clock();
    reservation.set_adoption(PlanAdoption::Primed {
        not_before,
        lead_blocks,
    });
    PrimedCandidate {
        epoch: reservation.epoch().0,
        not_before,
        lead_blocks,
    }
}
