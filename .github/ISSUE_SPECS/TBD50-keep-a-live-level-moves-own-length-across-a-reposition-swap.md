# Keep a live level move's own length across a Reposition swap

Placeholder from batch 17 (#1648's split, root ruling B17-R13; controls coordinator, 2026-10-16).
It follows the "split a blocking gate early" practice: #1648 closes on its passing gates, and the
in-flight length case moves here.

## Product outcome

A live level move (a VCA move over an automated fader, or a hand move stored as the ride's offset)
that is still ramping when a `Reposition` swap adopts (a rebuild whose only change on that lane is
the ride's table) ends when it was due, with its own length, for every `remaining` (the frames
left in its ramp at the swap), including `remaining < J` (`J`, the fader jump length, 960 in the
gate's session). The revision's watermark then reports `OUTCOME_EXACT`. This issue replaces the
conservative fallback flag of B17-R14 with the exact behaviour and deletes that flag path.

## Context

- Since #1648 attempt 3 (U3 fold 2), the adoption rule (`settle_adoption`,
  `crates/builtins-compiler/src/lib.rs`) holds a free stage's running ramp again, compares the
  curve where that ramp ends with the stage's target, and lands a moved target over
  `max(J, remaining)` (a one-sample landing piece, then the rest of the block over `J`).
- The defect (verifier, batch 17, at 0fac00ee9): when `remaining < J` the landing is stretched to
  `J`. In the gate's setup (a 4,000-sample move from block 4, `a_level_move_across_a_table_change_
  keeps_its_length` in `crates/capi/src/runtime/live_tests.rs`) a move heard 24, 26 and 30 blocks
  has 928, 672 and 160 samples left; the rebuilt rig differs from the uninterrupted engine by
  4.8e-4, 3.1e-3 and 2.1e-3 (both table changes, VCA and hand moves).
- Current behaviour (B17-R13, and B17-R14 once landed): stretched to `J`; the rebuild's candidate is marked
  `CandidateOutcome::TransitionFallback` on the control thread when it Reposition-adopts an
  automated lane with a live level move that may still ramp, so the watermark reports
  `OUTCOME_TRANSITION_FALLBACK`, conservatively (it can be set when the move was not stretched).
- Found while landing B17-R13 (measured with a temporary probe): for every heard move on the moving
  ride the stage's `fader_gain` at the swap is not the composed curve at the ramp's end (0.398 at
  12 blocks heard and 0.335 at 24, against 0.3018), so the adoption rule's comparison always
  differs and every heard move lands at the swap (over `remaining`, or `J`). The 4.6e-6 at 12
  blocks heard comes from that landing recomputing the stage's ramp step. Find why the stage's
  target moves during the move before designing the comparison.
- Also from B17-R13 (MINOR 4, not gated): the comparison point `at = end` in `settle_adoption` has
  no gate, because of the finding above (the comparison differs at either point). A gate needs a
  heard move whose stage target equals the curve at its ramp's end.

## Decision to make at filing

- D1. The deferred jump. At a `Reposition` adoption, a free stage's live ramp is not retargeted:
  it runs to its end as the uninterrupted engine runs it. The cell makes the D2 comparison and, if
  the target moved, the `J` jump at the first free sample after the ramp (the ramp's end), not at
  the block's first sample. A small API in `crates/automation` gives the cell's deferred jump
  (the node time at which it is due and the comparison it makes there), so the driver needs no
  second landing path; the one-sample landing piece and `max(J, remaining)` are deleted.
- D2. The conservative fallback flag of B17-R14 (control-time `TransitionFallback` on such a
  rebuild) is deleted; the C header comment, `docs/C_ABI_V1_QUALIFICATION.md` and the d.ts text
  that B17-R14 appended are removed.
- D3. A step of the cell's own at the swap block's first sample keeps the B17-R13 rule: the curve
  is compared there and the landing is over `J` (gate
  `a_step_at_a_reposition_swap_jumps_over_the_fader_length`).

## Objective gates

- G1. VCA and hand moves of 4,000 samples, heard 1, 12, 24, 26 and 30 blocks before a table-only
  rebuild (the ride split at 10,000, and a change in its last 2,000 samples): every block after
  the swap is bit-equal to the uninterrupted engine's (or, where the table's own values differ,
  to a fresh plan prepared with the new table and given the same move), allocation-free, adopted,
  and the rebuild's watermark advance reports `OUTCOME_EXACT`.
- G2. The comparison point: a heard move whose stage target equals the curve at its ramp's end
  keeps its own length with less than `J` left (red if the curve is compared at the block's first
  sample).
- G3. The B17-R13 step gate and the #1648 gates stay green; realtime policy count unchanged or
  explained.

## Dependencies

- #1648 (closed on its gates), B17-R13 and B17-R14 (the conservative flag this issue removes).
