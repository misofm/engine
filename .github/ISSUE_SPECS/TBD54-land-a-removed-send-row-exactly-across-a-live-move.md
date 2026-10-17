# Land a removed send row exactly across a live move

Placeholder from batch 19 (#1654 U2 review, root ruling B19-R7; controls coordinator, 2026-10-17).
Phase 2. Root files it. It follows the "split a blocking gate early" practice: #1654 closes on its
passing gates with a conservative flag, and the exact case moves here.

## Product outcome

When a rebuild removes a send's route-row automation (or its source strip's mute automation) while the
send stays a seven-word route and a live send move (a hand gain offset or an un-automated row edit) is
still ramping on that column, the move keeps its own length and the column then lands on the new
static value over the removed row's own length. The revision's watermark reports `OUTCOME_EXACT`. This
issue deletes the conservative flag of B19-R7.

## Context

- Batch 19 (#1654 after B19-R7): a carried seven-word following route that loses a route row or its
  source-mute curve lands the column at once over the removed row's `J`. That can cut a live send move
  in flight, so the control plane marks such a rebuild `CandidateOutcome::TransitionFallback` while a
  live send move may still ramp (send-only, conservative; the doc text sits beside #1713's).
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
- G2. The B19-R7 flag and its doc text are deleted; its gates are replaced by G1.
- G3. Allocation-free render; no new panic path.

## Dependencies

- #1654 (batch 19), #1713 (the seven-to-six flag, separate).
