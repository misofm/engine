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

## Decision record (attempt 1, 2026-10-24)

- Scope note: `scratchpad/opt/scope/1851.md`; its paths and decisions govern. Paths narrowed to
  `src/drain/{per_queue/mod.rs,select.rs,mod.rs}`, `tests/policy/drain/{per_queue,ported,receiver,outer}.rs`
  and the inventoried probes.
- Root R1 (accepted): `select::pass_index` strips `Expr::Paren` in its range branch, so
  `for lane in (0..) { controls[lane] }` selects and only `outer::finite` refuses the open range.
- Root R2 (accepted): a per-queue form's pass count loop stays select-free; only the loops in
  `steps[top..block_at]` (those that enclose the set loop) must select `<Q>`.
- Implementation: `select::selects` takes the start receiver and an end step instead of a
  `Counted`. `per_queue::Pass::holds` runs it from the pass header's `<Q>` (the receiver of
  `.iter_mut()`) over `top..block_at`, inside the per-reading `any`, so a reading whose outer loops
  do not select reads nothing (D10 rewind). `bounds` stays `bool` (the scope note's `Option<Form>`
  sketch was not needed). Threat model unchanged: each new refusal is a direct written form.
- Item 2 inventory. Rewritten to a selecting shape (`controls[lane]` under a range or
  `.enumerate()` index, or an element pattern over a slice parameter): ported
  `paren-open-range-outer`, `outer-repeat`, `outer-over-iterator-parameter`,
  `outer-over-rebound-slice-parameter`, `outer-zip-then-cycle`,
  `outer-over-slice-parameter-shadowed-by-block-const`; probes `a3-t/u20`, `rt-shapes/e09`,
  `a2-n/n08`, `a2-n/n02`, `rt-shapes/e12`. Deleted: probes `adv/b03` and `a2-n/n01` (the same text
  as the ported `paren-open-range-outer` and `outer-repeat`); ported `count-rebound-by-for-pattern`
  and probe `a2-n/n04` (no selecting loop can enclose a count read before it, so D1 always refuses
  them too; `forms.rs`'s `if let` / `match` / `while let` rebinding rows hold the count-scope claim
  alone); `receiver.rs` `pop-receiver-swapped-in-outer-loop` (scope note (b)). Left as is (c): the
  `while` / `loop` outers (`bounded-drain-inside-outer-loop`, `wrapped-outer-while`, `adv/a11`,
  `adv/a17`, `rt-shapes/e10`, `per_queue_inside_an_unbounded_outer_loop`).
- Not done (optional in the scope note): the `outermost` window of `pops_the_counted_queue` now
  has no row of its own; it was not proved dead in this PR.

## Evidence (attempt 1)

- Item 1 rows: `per_queue_inside_a_finite_outer_loop` (accept) became
  `per_queue_inside_a_non_selecting_outer_loop` (refuse). Red with the select call removed from
  `Pass::holds` (the base tool's behaviour; the same text was an accept row on 38324eb3e).
  New accept row `per_queue_inside_a_loop_over_banks` (`for bank in &mut ready.banks`): red under
  a mutant that refuses every loop in `top..block_at`.
- Mutants on the rewritten rows (each restored with `git checkout` + `touch`):
  - `outer::finite` returns true: red on all six ported rows and the five probes.
  - `slice_parameter`'s type check off: red on `outer-over-iterator-parameter` (and
    `outer-vec-parameter-by-value`).
  - `shadowed_by_an_item` returns false: red on `outer-over-slice-parameter-shadowed-by-block-const`
    and `rt-shapes/e12`.
  - R1 reverted together with `outer::finite` true: `paren-open-range-outer` stays refused (select
    refuses it), so R1 is what leaves that row to its own rule. R1 reverted alone: all green.
