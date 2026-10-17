# Keep a live level move's own length across a Reposition swap

Filed by root from batch 17 (#1648's split, root ruling B17-R13; controls coordinator, 2026-10-16).
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
- Current behaviour (B17-R13 and B17-R14): the move is stretched to `J`. The control plane marks a
  rebuild candidate `CandidateOutcome::TransitionFallback` before it publishes it when the
  rebuild is a transaction (not a declared discontinuity), the successor's model has at least one
  stored fader automation entry (`controls::automation_row_renders`), and the session's level
  move window may still ramp. The window (`LevelMoveWindow` in
  `crates/control-plane/src/control.rs`) folds every live commit's fader `FaderDb` and offsets
  records with a nonzero ramp, and every rebuild commit that changes some kept strip's effective
  fader dB or `S` (`controls::effective`, old model against new) with the ramp
  `max(fader_samples, the longest explicit ramp of the transaction)`, into one bound: the newest
  revision and the longest ramp. "May still ramp" is `watermark.revision < revision ||
  render_clock < watermark.first_sample + ramp` (a watermark read that gives up counts as "may").
  The watermark and the render clock are read before the reservation.
  So the watermark reports `OUTCOME_TRANSITION_FALLBACK`. This is conservative: it is also set
  when no move is stretched (any transaction rebuild of a session with fader automation while any
  level move may still ramp; see the #1648 decision record "the conservative fallback flag").
- First thing to find out, before the design work (root's note): why the stage target moves
  during a heard move (0.398 at 12 blocks, 0.335 at 24, against 0.3018). This may be a defect of
  its own. Measured while landing B17-R13 with a temporary probe: for every heard move on the
  moving ride, the stage's `fader_gain` at the swap is not the composed curve at the ramp's end.
  So the adoption rule's comparison always differs, and every heard move lands at the swap (over
  `remaining`, or `J`). The 4.6e-6 at 12 blocks heard comes from that landing recomputing the
  stage's ramp step.
- A second Reposition inside a stretched landing (B17-R13 tranche 5): a flagged rebuild also folds
  `(its revision, J)` into the window, so a second table-only rebuild inside the first one's
  landing over `J` is flagged too (gate
  `a_second_reposition_inside_a_stretched_landing_reports_a_fallback`). D2 deletes this with the
  rest of the flag.
- Further over-report: `may_ramp` reads the watermark's newest `first_sample`, so any later
  revision restarts the window (a non-level edit, or the flagged rebuild itself). A stream of live
  edits keeps it open after one level move.
- A cell's own jump of `J` that is still running at a Reposition swap and is landed over `J` again
  is specified D2 behaviour, not a defect: it has no user length, and a fresh plan with the new
  table also jumps over `J`. The exception is a stretched live move's landing, which is covered by
  the tranche-5 fold today and by D1 after this issue.
- MINOR 4 (from B17-R13, not gated): the comparison point `at = end` in `settle_adoption` has no
  gate. It is observable with a heard move on the flat ride (`S` moves, the curve does not), then
  a table whose step falls inside the remaining ramp. G2 can use that setup.

## Decision to make at filing

- D1. The deferred jump. At a `Reposition` adoption, a free stage's live ramp is not retargeted:
  it runs to its end as the uninterrupted engine runs it. The cell makes the D2 comparison and, if
  the target moved, the `J` jump at the first free sample after the ramp (the ramp's end), not at
  the block's first sample. A small API in `crates/automation` gives the cell's deferred jump
  (the node time at which it is due and the comparison it makes there), so the driver needs no
  second landing path; the one-sample landing piece and `max(J, remaining)` are deleted.
- D2. The conservative fallback flag of B17-R14 is deleted: `LevelMoveWindow`, `fold_level_move`,
  `rebuild_level_ramp`, `rebuild_moves_a_level` and the `set_outcome` call in
  `crates/control-plane/src/control.rs`, with `ObservedReservation::set_outcome` if nothing else
  uses it. The text that B17-R14 appended is removed from `crates/capi/src/abi.rs`, the C header
  comment, `docs/C_ABI_V1_QUALIFICATION.md` and the two d.ts files. Its gates
  (`a_reposition_swap_while_a_level_move_ramps_reports_a_fallback` and its host-web parity test)
  are replaced by G1.
- D3. A step of the cell's own at the swap block's first sample keeps the B17-R13 rule: the curve
  is compared there and the landing is over `J` (gate
  `a_step_at_a_reposition_swap_jumps_over_the_fader_length`).

## Objective gates

- G1. VCA and hand moves of 4,000 samples, heard 1, 12, 24, 26 and 30 blocks before a table-only
  rebuild (the ride split at 10,000, and a change in its last 2,000 samples): every block after
  the swap is bit-equal to the uninterrupted engine's (or, where the table's own values differ,
  to a fresh plan prepared with the new table and given the same move), allocation-free, adopted,
  and the rebuild's watermark advance reports `OUTCOME_EXACT`.
- G2. The comparison point (setup: a heard move on the flat ride, then a table whose step falls
  inside the remaining ramp): a heard move whose stage target equals the curve at its ramp's end
  keeps its own length with less than `J` left (red if the curve is compared at the block's first
  sample).
- G3. The B17-R13 step gate and the #1648 gates stay green; realtime policy count unchanged or
  explained.

## Dependencies

- #1648 (closed on its gates), B17-R13 and B17-R14 (the conservative flag this issue removes).

## Amendment (controls coordinator, batch 18, root ruling 2026-10-17)

- **Added item: gate or delete the owed discharge on a free fader write.** In
  `crates/builtins-compiler/src/lib.rs` (about `:1689` on `ad03780ee`), a free live write releases the
  hold and sets `self.owed = false`. Deleting that line leaves every test green (batch 18 T1 gate
  power check, `scratchpad/c18t1/`). Prove the discharge observable and gate it, or prove it
  unobservable and delete it. This slice reworks the same owed and hold machinery at a Reposition,
  so the item belongs here, not in a phase-2 placeholder.
