# Meters read known silence

Draft, slice S12 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` section 4.4.

## Product outcome

Every browser track carries a post-matrix sample-peak meter (`sixty_four_track_console_metered`). A
latched chain's resident block holds its sealed `+0.0` rest output, so the banked meter pass (#943,
#950) over it is correct but reads 2 x `frames * lanes` words that are known to be `+0.0`. When the
chain skipped this block, the unit's observers receive the fact and the banked meter commits a
constant block (peak `0.0`, energy `0.0`, the frame count) without scanning; the window still
advances and emits on schedule.

## Authorized paths

`crates/graph/src/runtime.rs` (the bank meter dispatch), `crates/builtins/src/lib.rs`
(`MeterAccumulator`'s banked commit), tests, this spec.

## Objective gates

1. Every meter snapshot (values, sequence, timing) of the metered row and of a metered sparse variant
   is bit-identical with and without the shortcut, across window boundaries and resets.
2. Red mutation: commit the constant without advancing the frame count.
3. Realtime and rule 3.

## Console benchmark rows

`sixty_four_track_console_metered` on silence, and a metered sparse variant.

## Dependencies

S4.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "bit-identical" or "unchanged" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
