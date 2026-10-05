VERDICT: FAIL

# #1302 attempt 1: verdict (stream J, commit e3375bc1d, base ab9620a07)

Reviewed in the export `/tmp/claude-1002/v1302/src` (git archive of e3375bc1d). The worktree was not touched.
The diff touches only the three authorized paths: `scripts/check-realtime-policy.sh`,
`scripts/test-realtime-policy.sh` and the spec's attempt record. No Rust source changed. Floors are unchanged
(25 files, 89 regions), and every new fixture line sits inside an existing region.

## Findings

### MAJOR

**J1-1. A pop in the body of a loop whose header rustfmt wraps is classified "no loop" and passes.**
`scripts/check-realtime-policy.sh:120-127` (`enclosing()`; the spec's D2.5 sketch has the same hole).
When a `while`/`for` header is too long, rustfmt puts its `{` alone on a line, at the opener's indentation.
The walk-back meets that lone `{` first, because its indentation is lower than the pop's, and sets `m` to
it. The real opener line has the same indentation (`ind >= m`), so the walk skips it. The loop never
enters the chain, and the pop counts as being in no loop. Reproduced on rustfmt-formatted input (edition
2024, max_width 100) under gawk, mawk and busybox awk:

```rust
        while self
            .control_lane_consumer_for_this_strip
            .has_pending_records_for_the_current_block()
            && self.enabled
        {
            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {
                break;
            };
```

This is an unbounded drain, and it PASSES. These also pass:
- the same layout with `for _ in 0..self.<long>.capacity_of_the_ring_in_records_total()`, which has a
  non-entry bound (adv `a02`);
- a wrapped-header outer `while` around a drain that re-reads `available_at_entry` (adv `a17`). Deviation 2's
  enclosing-loop check is blind to it for the same reason.

The gate's comment (`:82-96`, the D6 claim) says that every `try_pop` in a loop is checked against its
innermost loop, "reading rustfmt's layout". It also mentions a wrapped header's `{`. The issue exists to
stop rustfmt layout from defeating the rule (D1: "a rule that rustfmt alone can defeat"). This is the
sibling of the verdict's N1 case 1, moved from the header into the body.

*Fix:* in `enclosing()`, when the lower-indented line's trimmed code is exactly `{`, resolve it to its
opener. That opener is the nearest earlier non-blank line at the same indentation, skipping a fn's `where`
line. Record the opener in the chain. Also correct the comment: under rustfmt, a wrapped control-flow
header's `{` stands alone on its own line at the opener's indentation. Add two `expect_failure` cases in
existing regions:
1. a pop in the body of a wrapped-header `while`;
2. a bounded drain inside a wrapped-header outer `while`.

A 10-line prototype of this fix (`/tmp/claude-1002/v1302/mut/FIX_lonebrace.sh`) refuses a01, a02 and a17.
Under the prototype, the real tree still prints `realtime policy: ok (89 marked regions in 25 files)` under
all three awks, and every existing drain case stays green.

### MINOR

**J1-2. A plain (non-renaming) import escapes D3.** `scripts/check-realtime-policy.sh:102-103`.
Given `use core::iter::from_fn;` at file top (outside the region), `from_fn(|| q.try_pop().ok()).for_each(apply);`
in a marked body passes (adv `a05`). The comment (`:100`) and the spec's non-goal name only "a `use` that
*renames*".

*Fix:* also refuse a bare call, `(^|[^:[:alnum:]_])(from_fn|repeat_with|successors)[[:space:]]*\(`.
No marked body has one today; every `from_fn` there is `core::array::from_fn`. Alternatively, say "a `use`
that imports or renames" in the comment.

**J1-3. The stated limits omit indirection.** `scripts/check-realtime-policy.sh:99-101`. These pass, and
the comment does not list them:
- a pop in a helper method that a loop calls: `fn next_record(&mut self) -> Option<Record> { self.control.try_pop().ok() }`
  plus `while let Some(record) = self.next_record() {` (adv `a08`);
- a pop in a local closure that a loop calls: `let mut pop = || control.try_pop().ok(); while let Some(r) = pop() {`
  (adv `b02`);
- an enclosing loop outside the marked region. The rule is per region.

These are fair limits of a lexical rule. D6 requires the claim to live in the gate, so add them to the
"stays outside it" sentence.

### NIT

- **J1-4.** `available_at_entry` is matched as a substring, not a word (`:153`, `:161`). So
  `let not_available_at_entry_cap = c.capacity(); for _ in 0..not_available_at_entry_cap {` passes (adv
  `a14`). *Fix:* use `word("available_at_entry")` in both places.
- **J1-5.** A pattern rebinding between the entry binding and the loop is not seen. `let Some(available) = limit else { return };`
  passes (adv `a13`), because `counted()` recognises only `let [mut] x` and `assigns()` ignores patterns.
  *Fix:* treat a `let` whose pattern names `x` as a word as the nearest binding, which then fails the
  entry test.
- **J1-6.** Deviation 5 (`[^{}]*` in place of `.*`) has no defending case. Reverting it reddens nothing
  (mutant M14). Under rustfmt, a non-empty loop body never sits on its header line, so the shape it guards
  is unreachable. Either add a case or drop the claim from the attempt record.
- **J1-7.** Residual escapes covered by the "closure handed to a repeating adapter" non-goal, all passing:
  - `core::iter::repeat(()).map_while(|()| q.try_pop().ok())` (adv `a06`);
  - `(0..).map_while(|_| q.try_pop().ok())` (adv `a07`);
  - an outer `for _ in (0..) {` (adv `b03`). Only a header ending in `.. {` counts as open.

  Adding `iter::repeat` and a parenthesised open range is cheap if the rule is reworked anyway. This is
  not required by the spec.

## Judgement of the six deviations from the D2 sketch

All six are inside the authorized paths, and none changes Rust source. Each was checked against the real
tree, which passes unchanged.

1. **Loop keyword anywhere on the line; only a line-leading opener (after an optional label) can be bounded.**
   A correct strengthening, and in fact required. Under the sketch's line-start opener (mutant M10), the
   unchanged `marked-unbounded-try-pop-drain` case passes, which contradicts D4. The false-positive risk is
   a string literal containing `while`/`loop`/`for .. in` on a pop line or an enclosing line. That fails
   safe, and no real site is affected.
2. **Every loop enclosing the innermost one, up to the fn, must be a `for .. in` that is not an open range.**
   Covered by the gate's stated claim: "pops at most the records present at block entry". A `while`/`loop`
   around a bounded drain re-drains. It would refuse a sub-block `while cursor < frames` around a drain that
   re-reads the entry count (adv `a11`), and that is consistent with the claim. It is blind to wrapped outer
   headers (J1-1).
3. **Nearest in-scope binding, no reassignment between binding and loop.** A correct strengthening. The
   sketch's earliest-binding search admits a shadowing `usize::MAX` binding and another function's binding
   (mutants M7a and M7b red the new cases). The reassignment scan ignores scope, which is conservative.
   Pattern rebinding is a residual gap (J1-5).
4. **A `while <count>` body must start with `<count> -= 1;`.** A correct strengthening: without it, a
   non-decrementing `while remaining != 0` drains until the queue is empty. The real `live.rs:364-365`
   conforms.
5. **`[^{}]*` header matching.** Harmless. Nothing tests it (J1-6).
6. **The `fn try_pop` exemption is dropped.** Correct. Re-adding the exemption (mutant M13) reddens
   nothing, and the definition passes as "no loop". The valid fixture's `pub fn try_pop` still defends
   "a pop outside a loop passes" (M12c).

The `body-read:awk` matcher change in the self-test is the Hazards-sanctioned "distinguishing matcher". It
is necessary: with the change reverted in a scratch copy, the self-test fails with
`realtime counter-mutant did not reach intended assertion: per-file-read`.

## Gates run (in the export)

- **Gate 1:** `bash scripts/check-realtime-policy.sh` prints `realtime policy: ok (89 marked regions in 25 files)`
  under gawk 5.2.1, mawk 1.3.4 and busybox awk 1.36.1 (each forced via a PATH shim).
- **Real sites (debug copy of the pass):**
  - `builtins-compiler/src/lib.rs:1076` → `1075 for _ in 0..available {`;
  - `:1120` → `1119`;
  - `effect-contract/src/live.rs:366` → `364 while remaining != 0 {`;
  - `plan_exchange.rs:377` → no loop;
  - `graph/src/runtime.rs:900` → `899`;
  - `spsc.rs:440` → no loop.

  This is identical under all three awks and matches the attempt record. There are exactly six `try_pop`
  words in the marked bodies, as the spec says.
- **Gate 2:** `bash scripts/test-realtime-policy.sh` prints `realtime policy mutation tests: ok` under gawk,
  mawk and busybox awk. The fixture shapes and every mutated fixture are rustfmt-clean
  (`rustfmt --check`, edition 2024, max_width 100). The existing `marked-unbounded-try-pop-drain` case is
  unchanged.
- **Gate 3 (scratch mutants):**
  - Outermost-loop walk-back (M1): reds the valid fixture. Case 7's own shape passes under M1, which means
    red, reproduced in isolation.
  - Old regex restored, D3 and the pass deleted (M2): all 17 drain cases red, including cases 1-3 and 6-8.
- **Gate 4:** `bash scripts/check-workspace-policy.sh && bash scripts/test-workspace-policy.sh` ends with
  `workspace policy mutation tests: ok`, rc 0. `shellcheck` is not installed on this host: not run.

## Test value (each reproduced: red on the named scratch mutant, green unmutated)

- `drain-wrapped-while-let`, `drain-let-else-loop`, `drain-path-pop`: red if the rule reverts to matching
  one `while let .. try_pop(` spelling on one line (M2). The existing case stays green there.
- `drain-constant-bound`: red if any `for .. in 0..` counts as bounded (M4).
- `drain-bound-not-at-entry`: red if any identifier counts as an entry count (M5).
- `drain-second-loop-in-bounded-region`: red under a region-level "`available_at_entry` present" rule (M3).
- `drain-loop-inside-bounded-for`: red if the outermost loop is checked instead of the innermost (M1,
  isolated shape) or under the region-level rule (M3).
- `drain-from-fn`, `drain-repeat-with`: red if the D3 scan is dropped (M11).
- `drain-bounded-drain-inside-outer-loop`: red if enclosing loops go unchecked (M6). This is its only catch.
- `drain-count-from-another-function`: red if the binding search ignores indentation scope and takes the
  nearest binding anywhere (M7a). This is its only catch.
- `drain-shadowed-count`: red if the earliest in-scope binding is taken instead of the nearest (M7b).
  This is its only catch.
- `drain-reassigned-count`: red without the assignment check between binding and loop (M8).
- `drain-while-without-decrement`: red without the first-decrement check (M9).
- `drain-while-decrement-not-first`: red if a decrement anywhere in the body suffices (M9b). This is its
  only catch.
- `drain-labelled-loop`, `drain-loop-expression`: red if openers are recognised only at the start of the
  trimmed line, as in the sketch's D2.3 (M10).
- Valid-fixture additions (spsc definition, plan_exchange single pop, `live.rs` alias drain, per-lane fader
  drain): red if a pop outside any loop is refused (M12c), the alias step is lost (M12b), or the outermost
  loop is checked (M1).
- `drain-bound-0` / `drain-bound-1`: red if the pass's awk status is swallowed (`drain_hits=""`) or ignored
  (`else :`). Both mutants were reproduced.
- Counter-mutant `drain-bound`: red if the drain-bound injection stops distinguishing a status-swallowing
  gate from the hardened one (for example, a non-selective matcher that fails at another step).

## Adversarial shapes (`/tmp/claude-1002/v1302/adv/`, rustfmt-formatted; base result)

**Refused correctly:**
- a03 `while let Some(x) = q.try_pop().ok()`;
- a04 `loop { match q.try_pop() { .. } }`;
- a11 sub-block `while` re-draining;
- a15 wrapped `while let` with the pop in the header.

**Passed correctly:**
- a09 comments with braces and loop words;
- a10 multi-line signature with a bounded drain;
- a16 a long bounded drain;
- b01 labelled bounded `for` in a `where` fn.

**False negatives:**
- a01, a02, a17 (J1-1, MAJOR);
- a05 (J1-2);
- a08, b02 (J1-3);
- a13 (J1-5);
- a14 (J1-4);
- a06, a07, b03 (J1-7);
- a12, an inflated count inside the entry statement. This one is acknowledged in the comment.
