# Decide how adoption treats a rounding-only table difference

Filed by root from batch 19 (#1711 F2 and F3/F4 decision records; controls coordinator, 2026-10-17).
Phase 2.

## Product outcome

A table-only rebuild that changes no value a user can hear (for example a ride split at a sample, so
the same line is stored as two segments) leaves the rendered bits as they were, or the difference is a
documented, measured choice.

## Context

- At a Reposition adoption the fader cell compares the stage's target with the new curve (#1711 F0)
  and, if they differ, jumps over the fader length `J` (#1648 D2; after #1711, at the move's first free
  sample).
- A split table evaluates the same line in `f32` from different segment ends, so its values can differ
  from the old table's in the last bits. The comparison then sees a moved target and the cell jumps over
  `J`, where a fresh plan with the new table never jumps.
- Measured on batch 19 (`a_level_move_across_a_table_change_keeps_its_length` setup, C ABI): a
  table-only rebuild with no live move differs from the uninterrupted engine by about 1.7e-4 three
  blocks after the swap (1.4e-4 after a second rebuild); with a 500-sample move, 7.7e-5. The watermark
  reports `OUTCOME_EXACT`, which is honest under D2 (the jump is the specified adoption behaviour).
- Because of this, the G1 case "two table-only rebuilds 5 blocks apart" is gated on its outcome only.

## Decision to make at filing

- Option A: keep D2 as it is and document that a rounding-only difference is landed over `J`.
- Option B: compare with a tolerance tied to the curve's own `f32` evaluation error, or compare the
  segment parameters, so a rounding-only difference does not jump.
- Option C: make the table builder canonical (no split of a straight line), so the values are equal.

## Objective gates

- G1. The two-rebuild case of #1711 G1 is bit-equal to the uninterrupted engine (B or C), or a gate
  pins the documented jump (A).
- G2. A real table change still lands over `J` (the #1711 gates stay green).

## Dependencies

- #1711, #1648 D2.

## Step 0 record (diagnosis, 2026-10-24; no rendered bit moved)

- Red-first row: `a_table_only_rebuild_with_no_move_is_bit_equal` (`crates/capi/src/runtime/live_tests.rs`,
  `#[ignore]` until the fix): the riding fader, no live move, the ride split at 10,000 exactly on its line,
  then the single ride again 5 blocks later; bits against the uninterrupted engine. Red: first difference
  at block 4, sample 0 (the swap block); per-block maximum 6.9e-5, 1.3e-4, 1.5e-4, 1.7e-4, 1.6e-4
  (blocks 4-8), 1.4e-4, 9.4e-5, 3.4e-5 (blocks 9-11), equal from block 12 (sample 1,536).
- Branch (scratch instrumentation, removed): first rebuild, `Reposition` at `span.start` 512, free
  stage, `remaining == 0`, so `at = span.start` and `free = None`; the compare differs and the block opens
  with the adoption jump over `J` = 960 (the final `None => true`). New curve at 512: -0.5 dB, gain bits
  1064414713; `stage_gain` (the completed grid ramp's target) bits 1064416494, which equal the new curve's
  gain at 511 (-0.49902344 dB) exactly. Second rebuild: `Reposition` at 1,152 inside that jump
  (`remaining` 320), compared at 1,471 equal to the stage target, no new jump; the deviation is the first
  chord's.
- The stored tables do not differ: `value_at` of the split and the single ride agree in `f32` bits at
  every sample 0..=12,288 (an `f64` sweep of the same formula), and in the engine at 511. The filing's
  diagnosis ("the split line evaluates differently in `f32`") is wrong; option C has no trigger here.
- Cause: a one-sample compare-time mismatch. `G` divides the 128-sample block, so at a block start the
  last grid ramp has just completed and the stage target is the old curve at `start - 1`, while D2
  compares the new curve at `start`. On any moving ride these differ, so every no-move `Reposition`
  opens with a 960-sample chord where the uninterrupted engine runs `G` ramps.
- Supported option: D (compare the new curve with the old one at the same node time; with no stage ramp
  that is `start - 1`, where the stage's target was taken), which needs a root amendment of #1648 D2.
  The fix must keep a step of the new table at `span.start` jumping (B17-R13). Class B: the bits of an
  adopted lane move only for a `Reposition` with no stage ramp on a curve that moves between `start - 1`
  and `start` and whose new table equals the old one at `start - 1` (today a chord of up to about 1.7e-4
  on this ride, gone after the fix); flat rides, moves in flight and real table changes are unchanged.

## Decision record (fix D, attempt 1, 2026-10-25)

- Owner approval: the owner approved fix D on 2026-10-25 as a class-B change (root's brief). Options A,
  B and C are closed: step 0 found no table-bit difference (C has no trigger), and a tolerance (B)
  would mask the real cause.
- #1648 D2 amendment (root, 2026-10-25): at a `Reposition` the adoption compares the new curve with
  the old curve at the same node time. With no stage ramp (`remaining == 0`, held or free) that is
  `span.start - 1`, where the stage's target was taken; inside a free stage's ramp it stays the ramp's
  end (U3 fold 2); a held stage in a mute or unmute ramp and `Adoption::Compare` still compare at
  `span.start`. A step of the new table at `span.start` is still compared at `span.start` and jumps
  (B17-R13). No tracked #1648 ruling or derivation text exists (its spec has left
  `.github/ISSUE_SPECS/`); `settle_adoption`'s doc comment carries the amended rule.
- Change: `settle_adoption` (`crates/builtins-compiler/src/lib.rs`) sets `at = span.start - 1` for a
  `Reposition` whose stage runs no ramp and whose table has no step at `span.start`; the compare, the
  seed and the jump decision are otherwise unchanged. The step-0 row is un-ignored and takes the
  outcome check (`OUTCOME_EXACT`, adopted) of the no-move block that
  `a_level_move_across_a_table_change_keeps_its_length` held; that block is deleted.
- G1: the row is green after the fix and red before it (first difference block 4, sample 0, as
  step 0). G2: every existing adoption and automation row stays green (`builtins-compiler`, `capi`,
  `host-core` `stored_fader_automation`, `host-web` with `test-support`, `control-plane`).
- Bit-move evidence (PR evidence, scratch worktree and target dir, removed): an allocation-free probe
  in `settle_adoption` computed the old decision beside the new one over those five packages' full
  test runs. The new branch fired 176 times; the jump decision or seed value changed in 4 of them,
  all in the new row (two channels times two rebuilds, at 512 and 1,152), each a free stage with no
  ramp on a moving ride whose new table equals the stage's target at `start - 1` (old: jump; new: no
  jump). The other 172 jumped on both compares (a real table change), so no other rendered bit moved.
  No test reaches a held stage with no ramp (a settled mute) at a `Reposition`.
- Held stages keep D (review, 2026-10-25): a settled mute runs no ramp, its grid events are only
  remembered, so on a grid-aligned block its fader gain is the old curve at `start - 1` too, and the
  same-node-time compare is the right one.
- Off-grid limit: D compares at `start - 1` because, when a block starts on the grid, the last grid
  ramp has just completed there. Off the grid (any C ABI quantum, a seek landing on any frame) a free
  stage is still in its ramp (the U3 branch), and a held or free stage with no ramp whose target was
  taken earlier still jumps, as before D: no regression. Root files a successor for the exact rule
  (compare at the old cell's last ramp completion).
- Test placement (review MAJOR): the claim is owned by builtins-compiler.
  `a_carried_composed_lane_renders_the_uninterrupted_bits` runs its swap also with 128-frame blocks
  (swap at 1,152, on the grid) beside the 100-frame case (1,100, off the grid, U3). The capi row stays
  as the C ABI's one parity row (the rebuild's `Reposition` classification and `OUTCOME_EXACT`).
