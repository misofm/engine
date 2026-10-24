# Refuse a loop that encloses a per-queue drain's set loop

Filed by root from #1418's successor note (1418b; backlog sprint, 2026-10-24).

## Product outcome

#1418 (slice a) makes the realtime-policy tool refuse a loop around a counted drain unless the loop
drains a different queue on each pass. Two holes remain:

1. **A per-queue drain inside an outer loop.** `per_queue_inside_outer_loop`: a loop that encloses
   the per-queue form's set loop is today judged finite only, so it can drain the same set twice.
   D1 must refuse it. The tool needs `per_queue::bounds` (`tools/realtime-policy/src/drain/per_queue/mod.rs`)
   to hand back the set loop's block, so the select walk can run on the loops outside it.
2. **Doubly refused corpus rows.** Some rows in the ported awk corpus and in the refused probes
   (with `_` outer loops) are now refused by D1 as well as by the rule they were written for, so
   that rule lost its only catch. Rewrite them to selecting shapes where the rule matters.

## Smallest closable slice

Item 1, plus the rewrite of the item 2 rows. Optional rows (one each only if a distinct defect):
`controls[0]`, a struct `let`, a `match` arm, `while let`, `.rev()` and `chunks_mut(1)` over the
queue, `mem::take`, and the `(&mut it).enumerate()` index.

## Gates

- `cargo test --locked -p realtime-policy`; the release tool on the real tree stays ok.
- One refused row for item 1, red on #1418a's tool; each rewritten row is red when its own rule is
  disabled (mutant), not only under D1.

## Paths

`tools/realtime-policy/src/drain/{per_queue/mod.rs,select.rs,mod.rs}`,
`tools/realtime-policy/tests/policy/drain/**`, `tools/realtime-policy/tests/fixtures/**`, this spec.
