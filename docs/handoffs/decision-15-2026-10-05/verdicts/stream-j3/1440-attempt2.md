PASS

# #1440 (B1a) attempt 2: adversarial verdict

Range `e33614d95..1e03edf19` on `codex/d15-stream-j3` (`cfca7f991` spec-only root Amendment 1, `82a67bc09` code and tests, `db77a46f2` redundant guard dropped, `1e03edf19` attempt record). For the whole slice I also read `git diff 394a85c6d 1e03edf19 -- tools/realtime-policy`. I exported `1e03edf19` to `/tmp/claude-1002/v1440b/tree` and built it with `CARGO_TARGET_DIR=/tmp/claude-1002/v1440b/target`. I did not change the worktree. Its HEAD has since moved to `993b85745` (#1441), and `boundary.rs` has an uncommitted change. Both are outside this review.

Evidence that I kept is in `/tmp/claude-1002/v1440b/probe/`:
- `probes_v2.rs`: 60 probes and the poll_meters copy;
- `mut.py`, `mut.log` and `mut2.log`: the mutation sweep;
- `shadow.rs`: rustc proof for MINOR-1;
- `parity.sh`, `parity.tsv` and `parity-attempt1.tsv`.

Both attempt-1 MAJORs are closed. Every review class R1-R9 is refused. Of about 45 fresh bypass probes, none is a false pass. Every claim in the attempt record that I re-ran is true. One undefended guard is left (MINOR-1).

## Spec text against root's ruling

The spec carries these texts verbatim (string compare against `r1440-review.md`):
- item 5 "The terminal call";
- the "Macros and attributes" paragraph;
- the mention-list replacement ("and any mention in a macro token tree.");
- the meter-poll sentence.

No `debug_assert!` exception remains in D8b. Line 132 is D7's count form, which is a different rule. The a.3 Deref residual is in Hazards.

Gate 4 has the seven named cases and their mutations. R8b and R9 are PR evidence. The old macro row is rewritten as probe-a.

Adding the new case definitions to D11 is correct. D11 listed `per_queue_counters_written_by_macro`, and the ruling says to remove or rewrite that row in the same PR. Leaving D11 stale would contradict Gate 4.

## BLOCKER / MAJOR

None.

## MINOR

**MINOR-1. No test defends the labelled-block refusal of item 5's ancestor list. Removing it opens a compilable false pass.**
- Where: `tools/realtime-policy/src/drain/per_queue/window.rs:282`, `block.label.is_none() && via == Via::Body`.
- Attempt 2 removed the labelled-block arm of `breaks_out_of_outer`. The attempt record says: "the ancestors admit no labelled block, so a label can only name a loop". That makes `:282` the only guard.
- Mutation K drops `block.label.is_none()`. All 121 tests stay green, and the real tree stays ok.
- Under K, this probe is accepted. It compiles with `rustc --edition 2024` (warnings only; `probe/shadow.rs`):

  ```rust
  'l: for _ in 0..4 { <pass loop>; 'l: { if ready.lost > 9 { ready.refill_counters(); break 'l; } } }
  ```

  `break 'l` leaves the inner labelled block, not the loop. The next pass then starts with refilled counters.
- Today's code refuses the probe (`F-shadow-label-block`).
- Fix: commit a refused case, for example `per_queue_terminal_break_leaves_a_labelled_block`, with the shadowing label. Defect: "a labelled block admitted as an ancestor of the terminal call".
- This is the same class as attempt-1 MINOR-1: the code is right, but one guard has no test.

## NIT

**NIT-1. `breaks_out_of_outer` refuses more than the spec text.**
- Where: `window.rs:260`, `return index == self.outer`.
- The spec says "that `break` leaves the form's outermost loop".
- A `break 'e` to a labelled loop that encloses the set loop also leaves it. That loop is above the set loop's block, so the set loop runs again on re-entry. The tool still refuses the form (probe `F-break-enclosing-labelled`).
- This over-refuses only, and no production form uses it.
- Fix: record it as a deviation, or leave it.

**NIT-2. Item 5 does not hold the terminal call's own arguments to the allow-list.**
- The spec text does not require it either.
- `ready.refill_counters(if flag { continue } else { 0 }); return 0;` is accepted (`F-continue-in-args`).
- This is not a false pass: the arguments run before the call, so `continue` skips the refill.

**NIT-3. Change the open item "`Expr::Verbatim` ... not read by this rule ... recorded, not ruled" to "closed by A1".**
- `syntax::parse` (`src/syntax.rs:11-26`) runs `VerbatimFinder` on every file that holds a marker. The drain runs only on such files.
- Any non-empty `Verbatim` node therefore fails the gate on its own.
- Probes `F-verbatim-expr` (`builtin # offset_of`) and `F-verbatim-item` (`unsafe extern { safe static .. }`) in the D8b window give `{Verbatim}`.
- The drain rule does not see inside the node, but the tool as a whole refuses it. This is not a live false pass.
- An empty `Expr::Verbatim` is exempt, and it holds no tokens.

**NIT-4. Two D11 case texts differ from the committed tests in spelling only.**
- D11's `per_queue_second_pop_in_counted_inner_drain` binds `let n`, and the test binds `extra`.
- The probe-a and R1 texts say `ready.` where the tests use `R.` (they render as both roots).

## Probes (exported tree, no assertion, classes printed)

**Controls, accepted:**
- the passing form with an assignment in its tail;
- a terminal `break`;
- `break 'o` with a label on the outer loop;
- the terminal call in a match-arm body;
- the terminal call in a nested loop with `return`;
- the terminal call before the outer loop;
- `return (0);`;
- a tail of calls, index, tuple, array, struct, cast and unary;
- a macro after the outer loop, and a macro or attribute before the set loop (outside the span by design).

**Attempt 1 and the review: every one refused** (`{UnboundedTryPop}`, R7 `{ForbiddenBody, UnboundedTryPop}`): probe-a, probe-b, probe-c, R1, R2, R3, R5, R6, R7, R8b, R9.

**Fresh probes: every one refused.**
- Macros:
  - a path macro `crate::refill!()`;
  - a macro in type position and one in pattern position;
  - a nested macro;
  - a macro in the set loop;
  - a macro between the set loop and the outer loop.
- Control flow in the tail:
  - `?` in the tail, and `?` in the `return` operand;
  - `.await` in the tail;
  - a `return` in the middle of the tail;
  - `[0; { continue; }]` and a range in the tail;
  - a tail `return` without its `;`;
  - an assignment to the counters in the tail.
- Labels:
  - a labelled block, as a statement, with a value, and in a `let`;
  - `break 'o { if flag { continue; } 7 }`;
  - a shadowed loop label;
  - a shadowed block label (MINOR-1).
- Positions of the terminal call:
  - a match guard;
  - an `unsafe` block (also `UnsafeOutsideOwnership`);
  - an `if` condition and a `while` condition;
  - `let .. else`;
  - a closure with `return`.
- Attributes:
  - a doc comment;
  - an inner attribute;
  - attributes on an arm, a struct-literal field and a closure parameter.

**Production shape.**
- The faithful copy of `poll_meters` (`hosts/host-web/src/lib.rs:3446-3788` on `1e03edf19`, wrapped in `impl Host`, in the second region of the base tree) is accepted with no finding.
- Its mutants are refused:
  - `#[cfg(any())]` on its terminal `return 0;`;
  - `if flag() { continue; }` in its reset tail.
- This confirms the record's claim and root's production statement.
- Its one `#[cfg(test)]` (function line 253) is after the `passes` loop.

## Mutations (each applied alone, full test run, real tree, then reverted; the tree compared byte-equal to the export afterwards)

| Mutation | Red tests | Real tree |
|---|---|---|
| A: macros not recorded | `written_by_macro_defined_before`, `debug_assert_shadowed` | ok |
| B: `debug_assert!` exempt | `debug_assert_shadowed` only | ok |
| C: tail deny-list (only closure, loop and macro refused) | `continue_in_terminal_tail`, `continue_in_terminal_return` | ok |
| D: last statement's operand unchecked | `continue_in_terminal_return` only | ok |
| D2: other tail statements unchecked | `continue_in_terminal_tail` only | ok |
| E: span starts at `set_end` | `set_loop_under_cfg` only | ok |
| F: attributes on `return` not recorded | `terminal_return_under_cfg` only | ok |
| F2: attributes not recorded at all | `set_loop_under_cfg`, `terminal_return_under_cfg`; R5, R6, R8b, R9 and the attribute probes are accepted | ok |
| M14: only-pop check dropped | `second_pop_in_counted_inner_drain` only (attempt-1 MINOR-1 closed) | ok |
| G: tail unchecked | `counter_written_by_call`, `continue_in_terminal_tail`, `continue_in_terminal_return`, `terminal_break_leaves_an_inner_loop` | ok |
| M9: ancestor list open | `counters_refilled_in_closure` only | ok |
| M9a: closure admitted alone / M9b: `let` init admitted alone | none, as the record says (overlapping arms; M9 opens both) | ok |
| H: terminal `break` refused | `terminal_break` only | ok |
| I: any `break` ends the form | `terminal_break_leaves_an_inner_loop` only | ok |
| J: assignment refused in the tail | `meter_poll`, `param_root` | ok |
| K: labelled block admitted | **none** (MINOR-1) | ok |
| L: match guard admitted | none. Not a false pass: `return` and `break` in a guard keep their meaning. | ok |
| N: missing-top guard opened (MINOR-2 follow-up) | `a_pop_in_a_region_inside_a_macro_call_is_refused` only | ok |

## Test value (one sentence per new test)

- `per_queue_counters_written_by_macro_defined_before`: red if a macro in the window is judged by the roots in its own tokens, or if only `debug_assert!` and macros defined in the window are refused (A). It shares A with the next test, and root's ruling names it.
- `per_queue_debug_assert_shadowed`: red if `debug_assert!` is exempt from the macro rule (B, alone).
- `per_queue_continue_in_terminal_tail`: red if the tail's statements before the last are not held to the allow-list (D2, alone).
- `per_queue_continue_in_terminal_return`: red if the last statement's operand is not held to the allow-list (D, alone).
- `per_queue_set_loop_under_cfg`: red if the attribute span starts at the end of the set loop (E, alone).
- `per_queue_terminal_return_under_cfg`: red if attributes on a nested `return` are not read (F, alone).
- `per_queue_second_pop_in_counted_inner_drain`: red if the pass body's one-pop check is dropped (M14, alone).
- `per_queue_terminal_break`: red if a tail ending in a `break` out of the outermost loop is refused (H, alone).
- `per_queue_terminal_break_leaves_an_inner_loop`: red if any `break` ends the tail (I, alone).
- `per_queue_meter_poll` and `per_queue_param_root` (twin tail now has `R.lost = 1;`): red if an assignment in the tail is refused, which would refuse the real poll (J).

`per_queue_counters_written_by_macro` was deleted in the same change, as the ruling says. The attempt-1 MINOR-2 is closed: `is_tokens` and `tokens_only` are removed, and mutation N shows that the missing-top guard is now defended alone.

## Gates run

1. **Real tree.** `cargo run --locked --release -q -p realtime-policy` and `bash scripts/check-realtime-policy.sh` both print `realtime policy: ok (93 marked regions in 26 files)`. With cargo overhead, at load 7-9, the runs took 1.18, 1.37 and 1.14 s.
2. **Tests.** `cargo test --locked -p realtime-policy`: 121 passed.
3. **Parity.** I ran the attempt-1 verifier harness (paths moved to `v1440b`) on all 96 `expect_failure` trees. `parity.tsv` is byte-identical to attempt 1's:
   - 41 trees: the awk fails and the tool fails with the drain class;
   - 41 trees: both fail without the drain class;
   - 14 trees: the awk fails and the tool passes. These are D10's deferred list, unchanged.

   The harness exit 1 and its `mutant-unsafe-status` log lines are the same harness artifact as in attempt 1.
4. **Gate 7.** These commands exit 0:
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`;
   - `check-realtime-policy.sh`;
   - `test-realtime-policy.sh` ("realtime policy mutation tests: ok");
   - `check-workspace-policy.sh`;
   - `check-bench-policy.sh`.
5. **Gate 6.** `git diff --stat 394a85c6d 1e03edf19` outside `tools/realtime-policy` and `.github/ISSUE_SPECS` is empty. No `Cargo.toml` or `Cargo.lock` changed.

## Acked-batch question

Not applicable. No queue is added: the tool reads source.

## Spec note for root (does not block)

The a.3 residual is stated for `Deref`/`DerefMut` only. The class is wider: the tool reads no types, so any user trait impl on the counters' or queues' types is outside it. Examples:
- a `SubAssign` that does not decrement;
- a `PartialEq<{integer}>` that lies;
- a user trait method that shadows `iter`, `zip`, `iter_mut` or `available_at_entry`.

Each needs a user type in place of the plain integers and SPSC consumers that the real poll uses. Root may want the Hazards line to say "user trait impls (Deref, operator and method traits)" so that a later slice does not read it as closed.
