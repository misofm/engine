# Decide how adoption treats a rounding-only table difference

Placeholder from batch 19 (#1711 F2 and F3/F4 decision records; controls coordinator, 2026-10-17).
Phase 2. Root files it.

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
