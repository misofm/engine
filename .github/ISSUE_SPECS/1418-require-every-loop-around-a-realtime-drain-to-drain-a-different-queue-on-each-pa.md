# Require every loop around a realtime drain to drain a different queue on each pass

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Found by the stream-J batch verdict, MINOR-3 and NIT-2
(`/home/bl/misofm/submix-verdicts/stream-j-batch-verdict.md`, "MINOR-3 (design gap ...)"), on
*Make the realtime-policy drain rule structural instead of one regex line* (#1302; its attempt-3
verdict's J3-2 and J3-4 are the same class:
`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1302-attempt3.md`).
No production code changes.

## Problem (verified on `main` at `0a1176b3b`)

**The rule.** `scripts/check-realtime-policy.sh` reads each marked realtime region as tokens and
proves that every render-thread `try_pop` runs at most once per record counted at block entry by
`available_at_entry`, per pass of the loops around its drain (the comment at `:79-135`; the loop
classifier `classify`, `:270-305`). AGENTS.md forbids data-dependent unbounded calls in render, and
the C ABI prepares its queues with `Concurrent` delivery, so a drain that pops until the queue is
empty pops whatever a producer publishes meanwhile.

**The gap.** The rule bounds each pass, not the number of passes. A loop around the drain is
accepted when it is merely finite: `for P in 0..<bound>` with `<bound>` an integer literal or any
path (`self.lanes`, `LANES`, `self.controls.len()`, and also `usize::MAX` or `control.capacity()`),
`&<fields>`, `&mut <fields>`, `.iter()`/`.iter_mut()`, a slice or array parameter, or an
`array::from_fn` closure. The comment says so: "Finite is not small: ... `0..usize::MAX` or an
`array::from_fn` into `[(); usize::MAX]` around a drain that re-reads its count passes"
(`:110-112`). These pass the gate today, and each pops until the queue is empty:

```rust
fn drain_matrix_controls(control: &mut Consumer<Record>) {
    for _ in 0..usize::MAX {                   // or: for _ in 0..control.capacity() {
        let available = control.available_at_entry();
        if available == 0 {
            break;
        }
        for _ in 0..available {
            let Ok(record) = control.try_pop() else {
                break;
            };
            apply(record);
        }
    }
}
```

The same holds for `fn drain(rounds: [(); usize::MAX], control: ..) { for () in rounds { .. } }`
(batch NIT-2) and for `let mut rounds = 0_u32..; for _ in &mut rounds { .. }` (#1302 J3-2, an
infinite iterator behind `&mut`). The batch verifier reproduced `0..usize::MAX` and
`0..control.capacity()` against the batch tip `ef3b8835a`; the merged fold `116521b57` took its
other findings and left this one documented.

**The fixture holds a same-queue case as valid.** `drain_lane_pairs`
(`scripts/test-realtime-policy.sh:251`) drains one queue, the parameter `control`, in each of an
`array::from_fn`'s two closure calls, so it can pop up to twice the records counted at entry.

**The real outer-loop drain is not scanned.** The render drain of the builtin input bank,
`BuiltinBankProcessor::drain_controls` (`crates/builtins-compiler/src/lib.rs:460-505`, the body of
`begin_block` at `:527-530`), loops over lanes:
`for (lane, control) in controls.iter_mut().enumerate()`, then
`let Some(control) = control.as_mut() else { continue; };`, then the counted drain of `control`.
Each pass drains a different queue. It sits outside every marked region (the file's regions are
`:532-595`, `:1063-1105`, `:1107-1135` and `:1514-1570`), and a pop in a helper is a documented
limit of the per-region rule, so the gate never sees it. The marked drains it does see
(`crates/builtins-compiler/src/lib.rs:1074` and `:1118`, `crates/graph/src/runtime.rs:897`) have no
outer loop. The gate passes the real tree as `realtime policy: ok (89 marked regions in 25 files)`.

## Decisions

- **D1. Each enclosing loop must select the queue.** For a pop whose innermost loop is a bounded
  drain, every other loop around it (inside the region) is accepted only if the queue drained
  changes with that loop's pass. Walk outward from the count's receiver (`<path>` in
  `<path>.available_at_entry()`):
  - the receiver's root identifier is bound by the loop's pattern (`for control in ..`,
    `for (lane, control) in ..`) or by an `array::from_fn` closure's parameter; or
  - the receiver is indexed by exactly one identifier the loop's pattern binds
    (`controls[lane]`, `self.controls[lane]`; not `controls[lane % 2]` and not `controls[0]`); or
  - the receiver's root is bound inside the loop by `let`, `let .. else` or `if let`, from
    `&mut` or `&` of an indexed form above (`let control = &mut self.controls[lane];`), or from a
    pattern-bound identifier, alone or followed by `.as_mut()` or `.as_ref()`
    (`let Some(control) = control.as_mut() else { .. };`), and that binding is the one in scope.
  A loop between the drain and an outer loop is walked the same way: its iterable must name the
  outer loop's pattern identifier by one of these forms (for example
  `for bank in &mut self.banks { for control in &mut bank.controls { .. } }`).
- **D2. The iterable stays finite.** D1 is in addition to today's finite-iterable forms, not in
  place of them. A loop whose pattern binds nothing (`_`, `()`) can never pass D1.
- **D3. Collections are taken at their word.** As today, the gate does not prove that a collection
  holds each queue once. Iterating `&mut <fields>` yields distinct elements, and an SPSC consumer
  is not `Clone`, so each element is a different queue. Say this in the comment.
- **D4. The fixture.** Move `drain_lane_pairs` from the valid fixture to an expected failure, and
  add an `array::from_fn` case over `controls[lane]` to the valid fixture in its place. Add a valid
  two-level case (D1's last example) and a valid `let Some(control) = control.as_mut() else`
  case in the shape of `drain_controls`. Add expected failures for: `0..usize::MAX` (above),
  `0..control.capacity()`, the `[(); usize::MAX]` parameter, `&mut rounds` over `0_u32..`,
  `controls[lane % 2]` inside `for lane in 0..2`, and a counted outer loop
  (`for _ in 0..outer_available`) around a re-read drain.
- **D5. Mark the real drain.** Put `drain_controls` (`crates/builtins-compiler/src/lib.rs:460-505`)
  in its own marked region (`// REALTIME_POLICY_BEGIN` / `// REALTIME_POLICY_END`, as the file's
  other regions are written), so the rule checks the one outer-loop drain the engine has. The
  real-tree count becomes 90 marked regions in 25 files. If the region fails any other rule of the
  gate, stop and report the rule and the line; do not change the function.
- **D6. The comment.** Replace "Finite is not small ..." (`:110-112`) and the matching sentence in
  `classify`'s comment (`:270-275`) with D1's rule. The headline (`:79-80`) then states the bound
  the rule proves: per queue, at most the records counted at block entry.

## Authorized paths

- `scripts/check-realtime-policy.sh` (the drain rule and its comment)
- `scripts/test-realtime-policy.sh` (the valid fixture and the expected-failure cases)
- `crates/builtins-compiler/src/lib.rs` (D5's two marker comments around `drain_controls` only)
- This spec

## Non-goals

- The rule's other documented limits: pops in helpers, closures or macros; recursion; a loop around
  the drain outside the marked region.
- Any Rust source other than D5's two comments. If D1 refuses a real drain, stop and report it; do
  not restructure it here.
- Other unmarked render helpers. If D5 shows that a render-path drain sat outside the gate, list
  any other unmarked render-thread `try_pop` the implementer meets in the Evidence for root; do not
  mark them here.

## Hazards

- **Portability.** The gate is awk. It must give the same result under gawk, mawk and busybox awk,
  as #1302's verdicts checked; CI's runner has its own `awk`.
- **D5 changes a hot file.** `crates/builtins-compiler/src/lib.rs` is merged in the order
  `STREAMS.md` gives; two comment lines rebase trivially.
- **Over-refusal is acceptable, a false pass is not.** Where D1's walk is unsure, refuse.
- **Two passes over one queue.** A drain that legitimately reads one queue twice per block, if one
  is ever needed, must be written as one counted drain, not as an outer loop.

## Objective gates

1. **The real tree passes.** `bash scripts/check-realtime-policy.sh` prints
   `realtime policy: ok (90 marked regions in 25 files)` with `awk` resolving in turn to gawk, mawk
   and busybox awk (a `PATH` shim directory).
2. **The fixture.** `bash scripts/test-realtime-policy.sh` prints
   `realtime policy mutation tests: ok` under the same three awks.
3. **Red on revert (PR evidence).** With `main`'s `check-realtime-policy.sh` and the new
   `test-realtime-policy.sh`, the self-test fails on every D4 expected-failure case (each passes the
   old gate). Record the failing case names.
4. **Mutations (PR evidence).** Each applied alone to the new gate makes the self-test fail:
   - accept any identifier inside the index, not only a pattern identifier: the
     `controls[lane % 2]` case;
   - accept a receiver bound before the loop (drop the "bound by the pattern" requirement): the
     `0..usize::MAX` and `0..control.capacity()` cases;
   - drop the `let` / `let .. else` step: the valid `as_mut()` case, and gate 1 on the real tree
     fails on `drain_controls`.
5. `bash scripts/check-workspace-policy.sh` exits 0.

*Test value.*
- The `0..usize::MAX`, `0..control.capacity()`, `[(); usize::MAX]` and `&mut rounds` cases are red
  if an outer loop is again accepted for being finite alone, which lets a pop-until-empty drain
  through; no other case reaches it.
- `controls[lane % 2]` is red if any index that names the pattern counts as selecting the queue.
- `drain_lane_pairs` (now a failure) is red if a closure parameter that the receiver never uses
  counts as selecting the queue.
- The valid two-level and `as_mut()` cases are red if D1's walk refuses the shapes real code uses;
  the marked `drain_controls` (D5) is red on the same defect in the real tree.

## Evidence

- Gates 1 and 2 under each awk, gate 3's failing case names and gate 4's mutation runs.

## Dependencies

- None. *Make the realtime-policy drain rule structural instead of one regex line* (#1302) is on
  `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The self-test runs the gate on synthetic regions,
  which is the gate's own input, not a grep of the repository.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**What changed.**
- `scripts/check-realtime-policy.sh`: D1 to D3 and D6. `classify` now keeps each loop's pattern
  (`lpat`) and the collection its iterable names (`itrecv`). `counted` records the count's
  receiver and where it is read. A new walk (`selwalk`, `selects`, `resolve`, `binder`,
  `macro_named`) marks `sel[drain, loop]` for each enclosing loop that selects the queue, and the
  per-pop judgement accepts an outer loop only when it is finite **and** marked. A name the walk
  cannot read is refused: a `mut` binding, a struct or tuple `let` pattern (read from the raw
  text, because braces collapse to one `{}` token), a closure, a `match` arm or another header
  that names it (a pure `if` condition does not), a macro call other than `debug_assert!`, and a
  `static`/`const`/`use` that can name it. A method call ends the receiver forms, so shared
  handles behind a method are refused. The comment headline, the finite-is-not-small paragraph,
  the `&mut rounds` note and `classify`'s comment now state the D1 rule. The region floor is now
  90, with the floor comment updated.
- `crates/builtins-compiler/src/lib.rs`: D5. There is one `// REALTIME_POLICY_BEGIN` /
  `// REALTIME_POLICY_END` pair around `drain_controls` (inside `impl BuiltinBankProcessor`).
  Nothing else changed. The region passes every other rule of the gate.
- `scripts/test-realtime-policy.sh`: D4. `drain_lane_pairs` now drains `controls[lane]`. The
  valid fixture adds `BankSet::drain_banks` (two levels) and a third builtins-compiler region in
  the shape of `drain_controls` (`let Self {..} = self`,
  `for (lane, control) in controls.iter_mut().enumerate()`,
  `let Some(control) = control.as_mut() else`). That region also raises the fixture to 90
  regions. Seven new expected failures: `outer_usize_max`, `outer_capacity`,
  `outer_unit_array_parameter`, `outer_borrowed_open_range`, `outer_index_expression` (written
  as `let control = &mut controls[lane % 2];`, because a direct `controls[lane % 2]` receiver is
  already outside the count's receiver forms and the old gate refuses it), `outer_counted_loop`
  and `same_queue_lane_pairs` (the old `drain_lane_pairs`). The region-floor message is now
  "ninety".

**Gates.**
1. `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (90 marked regions in 25 files)`
   under gawk, mawk and busybox awk (a `PATH` shim directory for each).
2. `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok` under the same
   three awks.
3. Red on revert: with `HEAD`'s `check-realtime-policy.sh` and the new test script (the
   unexpected-pass exit turned into a listing), exactly these cases pass the old gate:
   `drain-outer-usize-max`, `drain-outer-capacity`, `drain-outer-unit-array-parameter`,
   `drain-outer-borrowed-open-range`, `drain-outer-index-expression`,
   `drain-outer-counted-loop` and `drain-same-queue-lane-pairs`. All seven are D4 cases, and no
   other case passes. The old gate also accepts the new valid fixture.
4. Mutations, each applied alone to the new gate:
   - The index form accepts `[ <pattern ident> <anything> ]` (`[[] " ident "[^]]* []]`). The
     self-test fails with `unexpectedly passed: drain-outer-index-expression`. The real tree
     still passes.
   - A receiver with no binding in the loop is accepted (`return patbound(x, kg)` becomes
     `return 1`). The self-test fails at `drain-outer-usize-max`. A listing run shows that
     `drain-outer-capacity`, `-unit-array-parameter`, `-borrowed-open-range`, `-counted-loop`
     and `drain-same-queue-lane-pairs` then pass as well.
   - The `let` / `let .. else` step is dropped (`if (r ~ /^L/) return 0`). The valid fixture
     fails at `crates/builtins/src/lib.rs:44` (`drain_lane_fields`). Run alone on the fixture,
     it also fails at builtins-compiler `:10` (`drain_fader_records`) and `:39` (the
     `drain_controls`-shaped `as_mut()` case). Gate 1 on the real tree fails at
     `crates/builtins-compiler/src/lib.rs:475` (`drain_controls`), `:1078` and `:1122`.
5. `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

Extra probes, run by hand and not committed. The gate refuses these drains in a pattern loop:
the receiver rebound by a struct `let`, by a closure parameter, by a `match` arm, by
`let control = &mut *fixed`, by `for mut control` with reassignment, by `rebind!(control)`, by
`controls[0]`, and through a base rebound inside the loop. The gate accepts
`if let Some(control) = control.as_mut()`, `let control = &mut controls[lane]` and a pure
`if control.ready()` around the drain.

**Notes for root.**
- A correction to the spec's Problem text: the marked fader and matrix drains
  (`crates/builtins-compiler/src/lib.rs:1070` and `:1114`) do have an outer loop, the same
  `for (lane, control) in controls.iter_mut().enumerate()` with `let Some(control) = control.as_mut() else`.
  They pass D1 through that shape.
- An open item that existed before this change and is outside this spec: the gate does not
  check that the pop's receiver is the count's receiver. A drain such as
  `let available = a.available_at_entry(); for _ in 0..available { b.try_pop() }` is still
  bounded by a's count. Inside a selecting outer loop, it is bounded by the sum of the selected
  queues' counts, but not "per queue".
- During development, one bug sent awk into unbounded recursion: an `if let` header was found
  again at its own offset. It is fixed (`binder` only considers headers that start before
  `pos`). The spec's fixture would not have caught this bug, because no valid `if let` case
  exists.
- I did not meet any other unmarked render-thread `try_pop`. I did not survey for them.
