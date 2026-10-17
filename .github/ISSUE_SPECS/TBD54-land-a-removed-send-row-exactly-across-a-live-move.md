# Land a removed send row exactly across a live move

Placeholder from batch 19 (#1654 U2 review, root ruling B19-R7; controls coordinator, 2026-10-17).
Phase 2. Root files it. It follows the "split a blocking gate early" practice: #1654 closes on its
passing gates with a conservative flag, and the exact case moves here.

## Product outcome

When a rebuild removes a send's route-row automation (or its source strip's mute automation) while the
send stays a seven-word route and a live send move (a hand gain offset or an un-automated row edit) is
still ramping on that column, the move keeps its own length and the column then lands on the new
static value over the removed row's own length. The revision's watermark reports `OUTCOME_EXACT`. This
issue deletes the conservative flag of B19-R7 and B19-R8.

Any rebuild that carries an automated following send must also keep a live send move on it: the move
keeps its own length, whatever else the rebuild changes.

## Context

- Batch 19 (#1654 after B19-R7): a carried seven-word following route that loses a route row or its
  source-mute curve lands the column at once over the removed row's `J`. That can cut a live send move
  in flight, so the control plane marks such a rebuild `CandidateOutcome::TransitionFallback` while a
  live send move may still ramp (send-only, conservative; the doc text sits beside #1713's).
- MAJOR-1 of the B19-R7 split verification (root ruling B19-R8): any rebuild, an unrelated one too
  (a rebuild that edits only the direct route `eq0-main`, an added muted send), cuts a live send move
  on an automated following send to `J`. The cut is the at-once branch of `place_carried` in
  `crates/graph/src/runtime.rs` (`has_dropped || !any_placed`): a carried column that composes with no
  placed value lands at once, so a column with a held ride and a hand move in flight lands over `J`
  at the swap. B19-R8 widens the control-plane flag to every transaction rebuild that carries a
  following route in both plans while a live send move may still ramp, the transaction's own join
  records with a ramp included (MAJOR-2). The exact fix here removes both the cut and that flag.
- Two exact attempts failed review (U2 folds 1 and 2): an "owed landing" held per cursor leaked its
  length to later jumps; held per column, it was dropped by a further rebuild (`adopt` reset it,
  MAJOR-C) and broken by a seek (its time was a node time computed from a render-time remaining, and a
  seek dropped it, leaving a stale value: MAJOR-D). Probes: the batch 19 verifier's
  `scratchpad/u2f2v/probes*.rs` (session scratchpad; summarised here).
- The design this issue must hold: an owed landing kept in render terms (samples left on the column's
  ramp, counted down by `frames`), carried by every carry (merged with a new one by the longer length,
  R19-4), never dropped by a seek unless the seek's landing composes that column, and never leaving a
  value that differs from a fresh plan's after its landing.

## Objective gates

- G1. The batch 19 MAJOR-A, MAJOR-C (two rebuilds) and MAJOR-D (seek inside, forward and back) setups:
  bit-equal to the uninterrupted engine until the move's end, then to a fresh plan; `OUTCOME_EXACT`;
  both hosts; host-core at Simd8 and Simd4 and the scalar oracle.
- G1b. MAJOR-1: on a riding send with a live hand move (gain offset or `matrix_lr`) in flight, an
  unrelated rebuild keeps the move bit-equal to the never-rebuilt twin until its end, then a fresh
  plan; `OUTCOME_EXACT`; both hosts. The same for MAJOR-2 (one transaction: a `gain_db` move over
  4,000, an upsert of one matrix row and the removal of another).
- G2. The B19-R7/B19-R8 flag (`PlanStateInventory::carries_a_following_route_into_a_following_route`,
  `SendMoveWindow`) and its doc text on the five surfaces are deleted; its gates
  (`an_unrelated_rebuild_across_a_live_send_move_on_a_riding_send_is_flagged_on_the_c_abi`,
  `a_send_move_and_a_row_removal_in_one_transaction_are_flagged_on_the_c_abi`,
  `a_rebuild_carrying_a_following_send_across_a_live_send_move_is_flagged_alike_on_both_hosts`, and
  the flagged expectations of the B19-R7 gates) are replaced by G1 and G1b.
- G3. Allocation-free render; no new panic path.

## Dependencies

- #1654 (batch 19), #1713 (the seven-to-six flag, separate).

## Related

- A route column and a single fader cell hold values differently: on a route column a value held by an
  earlier column jump follows the column's free time and never moves earlier (R19-3 (d), #1654 T3),
  while in a fader cell a later step's jump shortens the hold. Unifying the two rules is phase 2 (root,
  batch 19); it is not part of this issue's gates.
