FAIL

# #1440 (B1a) attempt 1: adversarial verdict

Range `394a85c6d..3f80ae102` on `codex/d15-stream-j3`. The commit was exported to `/tmp/claude-1002/v1440/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1440/target`. The worktree was not changed. The probe evidence is kept in `/tmp/claude-1002/v1440/probe/`: `probes.rs`, `a.rs`, `d.rs`, `mut.py`, `mut.log`, `parity.sh` and `parity.tsv`.

The port of D1-D8 is faithful and well defended. The real tree, the 57 ported cases and every spec-named mutation that I re-ran agree with the attempt record. The attempt fails on two compilable false passes in D8b condition 6. Both come from gaps in the spec's own closed list, and both are fixed by small fail-closed restrictions inside the authorized paths. Root must amend condition 6. See "Spec problems for root".

## MAJOR

**MAJOR-1. D8b condition 6 accepts a counter write from a local `macro_rules!` that is defined before the set loop.**
- Where: `tools/realtime-policy/src/drain/per_queue/window.rs:45-50`. It reads only the uses in `set_end..outer_end`.
- Probe `probe-a` (root `ready: &mut Ready`) and probe `probe-a2` (root `self`), in `probe/probes.rs`:
  - `macro_rules! refill { () => { for left in ready.remaining.iter_mut() { *left = 8; } }; }` is the first statement of the function;
  - then a correct set loop, `for _ in 0..4 { <correct pass loop>; refill!(); }`.
- Both compile with `rustc --edition 2024` (`probe/a.rs`). The tool returns `findings: []` for both. Each pass refills every counter to 8, so the queues pop records that were not counted at entry. This breaks D8b's claim: "its counter starts at its own count at entry and only falls".
- Cause:
  - The macro body's mention of the root is before the window.
  - The `refill!()` call in the window names no root.
  - Condition 6 reads only the mentions in the window.
- The plain count form has no such hole. A macro that is defined before `let count` cannot name `count`, because rustc refuses it (`probe/d.rs`, E0425). D8b roots are `self` or a parameter, so they are in scope from the start of the function.
- B2a does not close this. It refuses pops in macros and handles `debug_assert` shadowing, but it does not handle counter writes.
- Fail-closed fix: in the window, refuse every macro call other than a `debug_assert!` whose arguments parse. D8b's own check of the meter poll says "with no macro in the window", so the real poll stays accepted. Add the probe as a refused case.

**MAJOR-2. The item-5 terminal call accepts a tail that holds `continue` (or a `break` to an inner target).**
- Where: `window.rs:210-227`. `Banned` (`:355-399`) refuses loops, pops, closures and macros, but not `continue` or `break`.
- Probe `probe-b`:

  ```rust
  if ready.lost > 9 { ready.refill_counters(); if flag { continue; } return 0; }
  ```

  This sits after the pass loop, inside `for _ in 0..4`. It compiles (`a.rs`, `poll_b`), and the tool accepts it.
- The `continue` takes the refilled counters into the next pass. A `break` out of an intermediate loop or out of a labelled block does the same thing.
- The spec text of item 5 ("no loop, no pop, no closure and no macro call ... last statement is `return ..;` or a `break` out of the form's outermost loop") does not exclude these. The intent of "terminal" plainly does.
- Fail-closed fix: refuse a tail that holds a `continue`, or a `break` other than its last statement. Add the probe as a refused case.

Both false passes stay hidden today, because the awk gate refuses every D8b form. But no later slice (B1b-D1, B2a, B2b) revisits condition 6, so both would reach `main` with C2. The spec's Hazards say "a false pass is not [acceptable]", and the owner principle forbids leaving them for a later fix.

## MINOR

**MINOR-1. No single test defends condition 5's "only pop in the pass body" guard.**
- Where: `per_queue/mod.rs:323-332`.
- With that check alone removed (mutation M14), all 113 tests stay green. The record says this ("either alone leaves it refused").
- Gate 4 asks that the named case go red under the single mutation. It does not: `per_queue_two_pops_one_decrement`'s second pop also names `<r'>`, so the queue-mention clause refuses it first.
- The guard is not dead weight. Probe `probe-c` is a second pop of a disjoint field queue (`self.other`) in a nested counted drain inside the pass body. The tool refuses it, and accepts it under M14.
- Fix: commit that case.

**MINOR-2. The `ctx.region.is_tokens()` guard is dead.**
- Where: `drain/mod.rs:135`.
- A tokens-only region gets no nodes (`boundary.rs:84-99`), so `top` is `None` and the pop is refused at `:139-141` anyway.
- With the guard removed (M13), no test goes red, as deviation 5 says.
- Fix: remove it (`tokens_only` is then unused). After that, the macro-call region case defends the missing-top guard alone.

## NIT

**NIT-1. Make deviation 4 part of the decisions.**
- Deviation 4 reads a D7 "mention" as a use that D2 resolves to the count. This is allowed, and it is not a false pass:
  - D6 says "`<count>` is the D2 binding", so a use that resolves to a shadow in a closed block is not a mention of `<count>`;
  - a `macro_rules!` body that is defined between the binding and the loop still resolves to the count, and is refused;
  - one that is defined before the binding cannot name it (rustc E0425).
- The header stays a token count, so `for available in 0..available` is still refused.
- M17 (mentions matched by name) turns `form-shadow-in-a-closed-block` red.
- Suggestion: state this reading in D7's decision text, not only in the attempt record, because B1b and B2b-2 extend the resolver.

## Deviations judged

1. **Region nodes (coordinator gap).** `Boundaries::region_nodes` takes the children of every non-macro container whose extent (after outer attributes) lies wholly between the markers, and keeps the maximal ones. A2-D1 puts markers only at child boundaries, so the maximal nodes between them are exactly container children. This matches A2-D1, including the `impl`-spanning case (`a_region_spanning_the_end_of_an_impl_reads_its_nodes`). A tokens-only region has none, as A2-D1 says.
2. **The allow-list is applied from the file down.** Accepted. It is stricter (it refuses regions inside `async` or `const` blocks), fail closed, and the real tree passes.
3. **Innermost loop = the innermost loop whose body holds the pop, with header pops refused separately.** Equivalent to D8. Accepted.
4. See NIT-1. Accepted.
5. **Three overlapping macro-region guards.** One of them is dead (MINOR-2). The unreached-pop guard is defended alone by `drain-in-async-block` and `allow-list-const-block`.
6. **14 deferred cases, two committed here.** Verified (see Parity). The list matches the expected list in D10 except for the two committed cases (`loop_inside_bounded_for` and `second_loop_in_bounded_region`, refused here), and B1b-D8 commits exactly "B1a's deferred list". `count_from_another_function` keeps the awk's claim: a `let available` in another function of the same region, and a parameter count in the tail.
7. **Spec-named mutations that stay green alone.** I checked each overlapping guard:
   - `two_pops` (MINOR-1) and `is_tokens` (MINOR-2) are the only undefended ones;
   - "patterns of `let`" and "non-`let` binders" are each defended by their own rows;
   - each D8b condition-5 and condition-6 mutation that I re-ran goes red on its named case alone.
8. **`per_queue_param_root`.** Accepted. It pins parameter roots, and condition-1 resolution goes through it.

## Gates run

1. **Real tree.** `cargo run --locked --release -q -p realtime-policy` and `bash scripts/check-realtime-policy.sh` both print `realtime policy: ok (93 marked regions in 26 files)`. The marked pops are `builtins-compiler/src/lib.rs:1084` and `:1128`, `effect-contract/src/live.rs:366` and `graph/src/runtime.rs:900`. `spsc.rs:441` is the `fn try_pop` definition. The claim is confirmed.
2. **Tests.** `cargo test --locked -p realtime-policy`: 113 passed.
3. **Parity, drain cases only.** In an instrumented copy of `scripts/test-realtime-policy.sh`, every awk drain case ran through both gates:
   - the awk refuses all 57;
   - the tool refuses 43: 41 with the drain class, plus `open-tail-last-file` and `region-ends-in-comment` without it (the two D10 class changes);
   - the tool passes exactly the 14 listed as deferred.

   I did not re-run the full 327-tree parity.
4. **Mutations, each alone, then reverted (`mut.log`).** The real tree stays ok under each.

   | Mutation | Red case(s) |
   |---|---|
   | M1 `r#` not stripped | `raw-identifier-pop` |
   | M2 count binding outside the region | `binding-above-begin` |
   | M3 nested `fn` as a new scope | `pop-in-a-nested-function-in-an-unbounded-loop` |
   | M4 `Expr::Async` walked | `drain-in-async-block` |
   | M5 roots compared by spelling | `per_queue_root_rebound` |
   | M6 window ends at the pass loop | `count_reread` + closure, reborrow and macro cases |
   | M7 item 1 checks `<K>` only | `per_queue_collection_swapped` |
   | M8 terminal tail unchecked | `per_queue_counter_written_by_call` |
   | M9 terminal call allowed in a closure | `per_queue_counters_refilled_in_closure` |
   | M10 header pops ignored | `marked-unbounded-try-pop-drain`, `wrapped-while-let`, `path-pop`, `header-past-closed-body` |
   | M11 expression binders ignored | `count-rebound-by-for-pattern` + binder rows + the D8b twins |
   | M12 D8b removed | the three accepted D8b cases |
   | M13 `is_tokens` removed | none (MINOR-2) |
   | M14 pop-count check removed | none (MINOR-1) |
   | M15 closure parameter not a binder | `binder-closure-parameter` |
   | M16 `> 0` refused | `form-while-greater-than-zero` |
   | M17 mentions matched by name | `form-shadow-in-a-closed-block` |
5. **Run time.** Release binary on the real tree: 1.04, 1.06 and 1.05 s at load average 7. A2's figure is 1.02-1.04 s.
6. **Shipped artifacts.** `git diff --stat 394a85c6d 3f80ae102` outside `tools/realtime-policy` and `.github/ISSUE_SPECS` is empty, and no `Cargo.toml` or `Cargo.lock` changed. I did not rebuild the capi digests.
7. **Other checks.** `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `check-workspace-policy.sh` and `test-realtime-policy.sh` all exit 0.

## Test value (one sentence per new test)

- `the_ported_drain_cases_are_refused`: red if the visitor misses a pop, binding, read or innermost loop in a ported position (M1, M10 and M11 confirmed). The lexer-era rows are a regression corpus by root's ruling.
- `the_forms_that_d6_and_d8_state_pass`: red if a stated D6 or D8 form (such as `> 0`, or a shadow in a closed block) is refused (M16 and M17 confirmed).
- `a_count_that_the_rule_cannot_prove_is_refused`: red if a non-comparison mention, a non-`debug_assert` macro mention, a method cap, an inclusive range, or a bad `while` decrement is accepted.
- `a_loop_with_no_count_does_not_bound_a_pop`: red if a collection loop with no count bounds a pop.
- `every_kind_of_binder_takes_the_count`: red if a binder kind is not resolved (M11 and M15).
- `a_pop_below_a_node_outside_the_allow_list_is_refused`: red if `Expr::Const`, an item `const` or a `macro_rules!` item is walked.
- `a_counted_drain_in_an_async_block_is_refused_twice`: red if `Expr::Async` is walked (M4).
- `a_raw_identifier_pop_is_a_pop`: red on an `r#` comparison (M1).
- `a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it`: red if macro pops are ignored or a closure hides its loop. B2a deletes it.
- `a_pop_in_a_nested_function_is_judged_by_the_loops_outside_it`: red if a nested `fn` starts a loop scope (M3).
- `a_count_binding_above_the_region_is_not_read`: red if a node outside the region is read (M2).
- `a_loop_that_encloses_the_region_is_refused`: red if a loop above the region's top is ignored.
- `a_pop_in_a_region_inside_a_macro_body_is_refused` and `..._macro_call_is_refused`: red only when the overlapping guards are removed together (see MINOR-2).
- `a_region_spanning_the_end_of_an_impl_reads_its_nodes`: red if the region nodes come from one container.
- `per_queue_*` refusals: each is red on its named defect alone (M5-M9 re-run), except `two_pops_one_decrement` (MINOR-1).
- `per_queue_meter_poll`, `per_queue_single_pass`, `per_queue_param_root`: red if the port refuses the meter poll's shape, the single pass, or a parameter root (M12).

## Acked-batch question

Not applicable. This slice adds no queue; the tool reads source. The question that D8b answers instead is whether a pop can exceed its count at entry. MAJOR-1 and MAJOR-2 are two ways in which it can.

## Spec problems for root

1. **D8b condition 6 is written over the mentions in the window only.** A local `macro_rules!` that is defined before the set loop and names a root writes the counters with no mention in the window (MAJOR-1). Amend it: "any macro call in the window other than a `debug_assert!` whose arguments parse refuses the form". The meter poll has none.
2. **D8b condition 6 item 5's tail list omits control flow.** Amend it: "the tail holds no `continue`, and no `break` other than its last statement" (MAJOR-2).
3. **Gate 4's `per_queue_two_pops_one_decrement` cannot defend the pop-count guard alone,** because its second pop must name `<r'>`. Name an additional case: a second pop of a disjoint field queue in a nested counted drain in the pass body (MINOR-1).
