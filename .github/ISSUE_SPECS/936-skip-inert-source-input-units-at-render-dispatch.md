# Skip inert source-input units at render dispatch

**Ruled** (coordinator, 2026-09-26): change 1 of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md`, with the amendments of its adversarial
verification (`DIAGNOSIS-2-VERIFY.md` in the same folder). Class A.

## Product outcome

A source-fed session binds one `SourceInput` op per claimed input. For a plain claim that op does
nothing at render: `execute_op` returns before touching memory (`crates/graph/src/runtime.rs:3361`)
and `observe_unit` returns because the unit is not observed (`runtime.rs:3033`). The render loop
still dispatches all of them. On `sixty_four_track_plumbing_ring` that is 64 of the block's 65
units and about 1,630-1,850 cycles per block, measured in the shipped release profile. Build a
bind-time list of the units that do work and dispatch only those. Every source-fed session with N
plain claims saves N dispatches per block. Class A: an inert unit computes nothing, so skipping it
moves no bit.

## Root evidence

- `crates/graph/src/lib.rs:2518`: the render loop is `for unit in 0..runtime.units.len() {`.
- `runtime.rs:2807-2940` `Runtime::execute`: for a plain op its only effect is `execute_op`.
- `runtime.rs:3338` `execute_op`; it returns at `:3361` for `NodeKind::SourceInput`.
  `TrackDelay` is a separate kind handled before that line, so a PDC-delayed claim is **not** inert.
- `runtime.rs:3023` `observe_unit` returns on `!identity.observed` (`:3033`).
- Selective observation: activation entries are emitted once per observer binding
  (`runtime.rs:4692-4704`) and resolved through `op.observers` (`:3207-3217`). A unit with no
  observers has no cursor entry, so skipping it keeps the cursor in step. Meters and alias taps are
  folded into `op.observers` at bind.
- Resident input uses `before.last()` by index and needs a `Bank` predecessor (`runtime.rs:2830-2840`);
  skipping an op cannot change that adjacency. `complete_pending` and invalidation walk every unit
  and are not changed.
- Measured (verification, production shape, four runtimes, aligned and unaligned claims): change
  alone is -1,630 to -1,850 cycles on the ring row; the bound row does not move.

## Smallest closable slice

Authorized paths: `crates/graph/src/lib.rs` (the executor struct, its two layout mirrors, the
render loop, bind, the metadata resource estimate), `crates/graph/src/runtime.rs` (a new
`Runtime::unit_inert` predicate and the `rt9` source-pin test only), their tests,
`tools/console-workload/src/lib.rs` and `tests/` (gates only), and this spec.

1. **The predicate.** `Runtime::unit_inert(index) -> bool` is true exactly when all of these hold:
   - the unit is a plain `RuntimeUnit::Op` whose kind is `NodeKind::SourceInput`;
   - `op.observers` is empty;
   - `identity[index].observed` is false;
   - it is not the Output op.
   Anything else, including `TrackDelay`, banks, split pairs and every other kind, is active.
2. **The table.** At bind, build `active_units: Box<[u32]>`, the ascending indices of the units that
   are not inert. Units stay in `runtime.units`, so the census and `unit_eligibility()` do not change.
   The table is a bind-time allocation, never touched structurally at render.
3. **The loop.** The render loop iterates `active_units` in order instead of `0..units.len()`. Every
   other part of the loop body is unchanged.
4. **Resource accounting.** `GraphRuntimeMetadataResourceEstimate` (`lib.rs:425`) charges only layout
   deltas today, and the sibling table `source_input_buffers` (`lib.rs:2239`) is uncharged. Charge
   both the new table and `source_input_buffers` by their byte length, and add the new field to
   both layout mirrors (the structs at `lib.rs:2250` and `:2261`) so the observation-state and
   split-owner deltas do not shift.
5. **The rt9 source pin.** `rt9_resident_entry_has_one_guarded_production_caller_and_control`
   (`runtime.rs:7913`) splits the render source on the literal loop header (`:7946`). Re-pin it to
   the new header and keep every assertion and control it has. Do not weaken it.

## Non-goals

No change to what an op computes, to the Output kernel, to bound-unit dispatch (DIAGNOSIS-2 change 3)
or to any public trait. No batching of driver calls (change 4 was dropped by the verification).

## Objective gates

1. **Dispatch counter.** A `test-support`-only counter of dispatched units per block reads 1 on
   `sixty_four_track_plumbing_ring` and 65 on `sixty_four_track_plumbing_only`.
2. **Observed input stays active.** A ring-fed plan with an observer (a meter) bound at one claim's
   Input boundary keeps that unit in `active_units`, and the observation reports the same values as
   on the base commit over 16 blocks.
3. **Delayed claim stays active.** A ring-fed plan where one track carries a PDC delay renders
   bit-identical output to the base commit over 16 blocks and dispatches that track's `TrackDelay`.
4. **Digests.** Every console workload's 64-block digest is unchanged, including `BASE_DIGEST` in
   `tools/console-workload/tests/chain_shape.rs` and
   `the_driver_fed_plumbing_row_renders_the_bound_rows_bits`.
5. **Resource report.** A test pins that the metadata estimate grows by exactly the two tables' byte
   lengths on the ring row, and the existing resource-report tests pass.
6. **Allocation and policy.** `crates/graph/tests/rt10_source_in_place_alloc.rs`, rt1 and rt9 alloc
   tests, the re-pinned rt9 source test, `scripts/check-realtime-policy.sh`,
   `check-graph-determinism.sh`, `check-graph-policy.sh`.
7. **Red mutations** recorded in `crates/graph/tests/MUTATIONS.md`: a predicate that ignores
   `observed` fails gate 2; a predicate that treats `TrackDelay` as inert fails gate 3; iterating
   `active_units` in reverse fails gate 4.
8. `cargo test -p graph` with and without `test-support`, `-p graph-compiler`, `-p console-workload`,
   `-p host-core`; `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets
   --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

## Console benchmark rows

Can move: `sixty_four_track_plumbing_ring` and any other source-fed row. Bound-fed rows must not move
a bit.

## Dependencies

None. Independent of "Resolve Output inputs in groups of eight and tighten the pair kernel"; the two
compose.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit. Every "bit-identical" or "unchanged" gate is a
  hard stop.
- Render paths stay allocation-free, lock-free and syscall-free. `crates/graph` stays free of
  `unsafe`. Public docs must not link to private items (rustdoc runs with `-D warnings`).
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not quote a projected saving; the
  coordinator runs the console benchmark once after the batch.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.

## What the implementer will hit

- `runtime.rs:12210` has a second `for unit in 0..runtime.units.len()` loop; read it and decide
  whether it is a test helper or production before touching it. Do not change it unless it is the
  render loop.
- The `#[cfg(test)]` skip-disabled override affects only tests that dispatch source inputs; the
  #900 test binds bound inputs.
- The wasm call-graph gate cannot see the graph executor (it sits behind
  `Box<dyn PreparedPlanExecutor>`, `crates/engine/src/realtime/plan.rs:557`), so allocation-freedom
  rests on the native counting-allocator tests in gate 6.

## Attempt 1 evidence

Implementer: attempt 1, branch `codex/936-skip-inert-source-inputs`. The implementation commit is
`3172be42`, on `64b155d0`. Nothing was pushed, and no timed benchmark was run.

### Design

- **The predicate.** `Runtime::unit_inert(index)` (`crates/graph/src/runtime.rs`, beside
  `source_in_place`, outside the realtime region) holds exactly when all four of the brief's
  clauses hold: a plain `RuntimeUnit::Op` of kind `NodeKind::SourceInput`, empty `op.observers`,
  `!identity[index].observed`, and not `output_unit`. An index past the units is active.
  `TrackDelay`, banks and every other kind are active.
- **The table.** `GraphExecutor::new` builds `active_units: Box<[u32]>` after the runtime and the
  copy list: the ascending indices of the units that are not inert. The units stay in
  `runtime.units`, so the census, `unit_eligibility`, `complete_pending`, invalidation and every
  other walk over the units are unchanged. The field is on `GraphExecutor` and on both layout
  witnesses (`GraphExecutorWithoutSplitPairTable`, `GraphExecutorWithoutObservationActivation`).
- **The loop.** The header is now `for unit in active_units.iter().map(|&unit| unit as usize) {`.
  The body is unchanged apart from one `test-support`-only register increment. The count is added
  to a thread-local once per completed block (`test_only_count_unit_dispatches`), not once per
  unit, so a `test-support` build (the bench still builds `graph` with it) pays one TLS add per
  block. The second `0..runtime.units.len()` loop (`runtime.rs`, `render_arena_oracle`) is the
  #916 test oracle inside `mod tests` and was not touched.
- **Resource accounting.** `GraphRuntimeMetadataResourceEstimate` gains `active_unit_table_bytes`
  and `source_input_table_bytes`. Both are added to `total_bytes` and to the largest-allocation
  maximum. See deviation 1 for how they are sized.
- **Test-support surface** (`#[doc(hidden)]`, absent without the feature):
  - `test_only_unit_dispatch_reset` and `test_only_unit_dispatches`: the gate-1 counter.
  - `test_only_executor_table_bytes`: the byte lengths of the two tables of the executor most
    recently bound on this thread, recorded at bind.
- **rt9 source pin.** `rt9_resident_entry_has_one_guarded_production_caller_and_control` now
  splits the render source on a `RENDER_LOOP_HEADER` constant holding the new header. Every
  expected statement, every count and every control it had is kept. It adds two controls: the
  header occurs exactly once in `lib.rs`, and a source whose loop goes back to
  `0..runtime.units.len()` is refused.

### Gates

| # | gate | command | result |
|---|---|---|---|
| 1 | dispatch counter: 1 per block on `sixty_four_track_plumbing_ring`, 65 on `sixty_four_track_plumbing_only`, over 64 blocks, census 65 on both | `cargo test --locked -p console-workload --lib the_driver_fed_plumbing_row_dispatches_only_its_output_unit` | PASS. The graph-level twin `an_unobserved_source_input_is_not_dispatched_and_moves_no_bit` (#927 ring plan, 64 x `Input -> Route -> Output`, frames 1/7/16/128, 16 blocks) finds the table to be `[output]`, 1 dispatch per block, and the base digest. |
| 2 | observed input stays active, meters the base values over 16 blocks | `cargo test --locked -p graph --lib an_observed_source_input_stays_dispatched_and_meters_the_base_values` | PASS. Covers `RingShape::ObservedInput` (a meter on K's `Input`) and `ObservedAlias` (a meter on K's elided `PostFader`, folded into the input op). The table is `[K, output]`, with 2 dispatches per block. The K meter publishes every block. Every host word and meter frame matches the digest recorded on `64b155d0`: `0x1f149a39d035ceea`, combined over frames 1/7/16/128. |
| 3 | delayed claim stays active and renders the base bits over 16 blocks | `cargo test --locked -p graph --lib a_delayed_claim_stays_dispatched_and_renders_the_base_bits` | PASS. `RingShape::TrackDelayed` (K delayed (3, 5)). K's unit is its `NodeKind::TrackDelay` op, it is in the table, and there are 2 dispatches per block. The digest matches the base: `0xe8b0890ea031f463`. |
| 4 | every console workload's 64-block digest unchanged | `cargo test --locked -p console-workload` (lib 5, chain_shape 24, automation 4, placement 3: all pass, `BASE_DIGEST` and `the_driver_fed_plumbing_row_renders_the_bound_rows_bits` included); plus a throwaway test (not committed) printing the 64-block SHA-256 of all 17 rows (`WORKLOADS` + `DRIVER_FED_WORKLOADS`, `PlanConfig::BASELINE`), run on this tree and again on `64b155d0` with the change stashed | PASS. All 17 digests are identical. Both plumbing rows are `57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800`, and `sixty_four_track_console` is `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de`. |
| 5 | metadata estimate grows by exactly the two tables on the ring row; existing resource tests pass | `cargo test --locked -p console-workload --lib the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables`; `cargo test --locked -p graph --lib runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow the_metadata_charge_covers_the_ring_plans_executor_tables` | PASS. On the ring row, 513 emitted ops: `graph_metadata_bytes` goes from 322,999 on `64b155d0` to 333,259, which is +10,260 = 513 x (4 + 16). The runtime metadata goes from 15,840 to 26,100. The bound tables are 4 B (one dispatched unit) and 0 B (no copied claim). The ring plan bound declined has a 1,024 B copy table, which fits its 2,064 B charge. The metadata test adds the mirror invariants: the owner-level split and observation deltas equal the runtime-level ones. The graph-compiler, builtins-compiler and host-core resource tests pass unchanged. |
| 6 | allocation and policy | `cargo test --locked -p graph --features test-support` (rt10 2, rt1 1, rt9 8); `cargo test --locked -p graph` (rt10 2, rt1 1, rt9 1); `bash scripts/check-realtime-policy.sh`; `bash scripts/check-graph-determinism.sh`; `bash scripts/check-graph-policy.sh` | PASS. `realtime policy: ok (54 marked regions in 16 files)`; `graph fresh-process determinism: PASS (100/100)`; `graph policy: PASS`. The re-pinned rt9 source test passes. No `unsafe` was added to `crates/graph`. |
| 7 | red mutations | `crates/graph/tests/MUTATIONS.md`, issue #936 (rows 936-1 to 936-7) | The brief's three are RED: 936-1 (gate 2, and bits-only), 936-2 (gate 3, and bits-only), and 936-3 (gate 4: `BASE_DIGEST` and the driver-fed equality). The extras are RED too: the table ignored (counters), each mirror omission, each charge omission, and a per-block `Vec` in the loop (the alloc tests). |
| 8 | suites and lints | `cargo test --locked -p graph` with and without `--features test-support` (113 lib + integration), `-p graph-compiler` (73 lib, 1 fixture bin, route_gain 3, scale 1, track_delay 8, doctests 6), `-p console-workload`, `-p host-core` (73 lib + 14 test binaries); `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS on all of them. |

### Deviations and disclosures

1. **The tables are charged at their bound, not at their bound length.** The runtime metadata
   estimate is computed and admitted at compile time (`graph-compiler` `compile.rs`, from
   `emitted_op_count`). The two tables are sized at bind, after admission, and their lengths
   depend on the source set and on the in-place decisions, which the compiler never sees. So each
   table is charged at one entry per emitted op. A dispatched unit is a unit, and a unit holds at
   least one op. A copied claim names a distinct node (claims are strictly ascending), and a node
   lowers to at most one op. This is the same emitted-op bound the estimate already uses for its
   unit and op containing allocations. The charge is deliberately loose: 20 B per op on 64-bit
   targets, 10,260 B on the ring row's 914 KB plan, for 4 B actually retained. A tighter,
   claim-count bound would need the compiler's call signatures to change, and they are outside
   this brief's paths. Gate 5 therefore pins three things:
   - the growth equals the two charged table lengths;
   - both charges cover what the ring row actually binds;
   - the untouched terms did not shift, through the mirror invariants.

   It does not pin a literal pre-change byte count. That literal would re-pin on every unrelated
   layout change, and the invariants catch the mirror hazard the verification named (936-5a and
   936-5b).
2. **"As on the base commit" for gates 2 and 3.** The oracle is a digest (`INERT_PRE_CHANGE`)
   recorded by the committed fixture's own bind, poison and render sequence, as a throwaway test
   against the unmodified `64b155d0` tree. The practice is #927's `RING_PRE_CHANGE`. No
   test-support switch was added to restore the old loop.
3. **Mutation 1 as the brief words it is equivalent.** Dropping only `!identity.observed` (936-1a)
   or only `op.observers.is_empty()` (936-1b) stays green. `new_with_observation_activation` sets
   `observed` from `has_observers()`, which for a plain op is the same fact. The recorded red
   mutation (936-1) drops both observation clauses.
4. **Policy-script gap.** `check-realtime-policy.sh` misses `.collect::<...>()` (turbofish) in a
   marked region (936-7). The counting-allocator tests catch it. The gap is not fixed here because
   the script is outside the brief's paths. It is a candidate for a tooling issue.
5. **Already-stale unchecked fixture** (corrected in attempt 2, after Sol's finding 3).
   `fixtures/graph/v1/direct-route.resources.json` was stale before this issue: it records
   `graph_metadata_bytes: 5336`, while the generator gave 5,868 on `64b155d0`. This change moves
   the generated value by a further 180 B, to 6,048 (9 emitted ops x 20 B). Nothing gates that file:
   `check-graph-determinism.sh` compares only the fingerprint, and `graph_fixture --check` runs
   nowhere. Per Sol's ruling it is not regenerated here; it is tracked as #947.
6. **Public surface.** `GraphRuntimeMetadataResourceEstimate`, a public struct with public fields,
   gains two fields. No workspace code builds it with a literal (the workspace clippy passes). No
   public trait changed. The new test-only functions are `#[doc(hidden)]` and exist only with
   `test-support`.

## Sol attempt 1 verdict: FAIL

Reviewer: Sol, attempt 1, on `cde3345e` (implementation `3172be42`, base `64b155d0`). Nothing was
pushed, and no timed benchmark was run. Every mutation and scratch probe below was applied in
place and reverted with `git checkout`. The tree is left exactly at `cde3345e` plus this section.

The render change is correct and class A. The attempt fails for one reason: it turns a required CI
test red, and that test is outside the brief's authorized paths.

### Findings, most severe first

1. **Blocking: a required resource-report test is red.**
   `crates/capi/tests/resource_lifecycle.rs:2689`
   (`external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`) fails on HEAD. The
   live report gives `graph_metadata_bytes` 56,403 against the pinned 54,763, and
   `graph_session_plus_plan_bytes` and `graph_incremental_plan_bytes` 237,168 against 235,528.
   - **Cause:** the difference, +1,640, is exactly that fixture's 82 emitted ops x 20 B, the new
     `active_unit_table_bytes` + `source_input_table_bytes` charge (`crates/graph/src/lib.rs:501-502`).
   - **Attribution:** with `crates/graph/src/{lib,runtime}.rs` put back to `64b155d0`, the same
     test passes 4 of 4.
   - **Scope of the failure:** I ran the required `qualification` job `test-debug-a` command
     (`.github/workflows/qualification.yml:476`, with `--no-fail-fast`) on HEAD. This is its only
     failure; the other 107 test binaries pass.
   - **Gate 5 is not met:** it says "the existing resource-report tests pass". The attempt-1
     evidence did not run `-p capi` and does not mention this test.
   - **Needed for attempt 2:** re-pin the three single-plan graph literals
     (`resource_lifecycle.rs:555-572`) and the double-live `oracle.graph` (`:2644`, +3,280). Add the
     two tables as independent primitive rows in `graph_owners` (`:1766`), following that file's
     own convention (#470, #779, #816). That file is outside the brief's authorized paths, so the
     coordinator must amend the scope first.
2. **Batch-boundary item, not charged to this attempt.** The browser pin
   `hosts/host-web/tests/browser-v1/expected.json:70-72` will move: `graphSessionPlusPlanBytes`,
   `graphIncrementalPlanBytes` and `graphMetadataBytes`. It is gated by
   `scripts/check-browser-expected-resources.py` (`qualification.yml:216`). The shift is emitted
   ops x 12 B, because on wasm32 the entries are `u32` (4 B) and `(usize, u32)` (8 B). The brief
   defers browser re-pins to the batch boundary, and these rows must be on that list.
3. **Low: deviation 5 is inaccurate, but my ruling is not to regenerate here.**
   `fixtures/graph/v1/direct-route.resources.json` was stale before #936. On HEAD the generator
   (`graph_fixture --emit`) gives `graph_metadata_bytes` 6,048 (base 5,868) against the checked-in
   5,336, and `session_plus_plan_bytes` 16,292 against 27,868. `direct-route.report.json`'s
   `graph_sha256` differs too, and `graph_fixture --check` fails with `manifest mismatch`, as #685
   recorded on its own base.
   - **Nothing consumes the file.** `check-graph-determinism.sh` compares only the fingerprint, and
     `docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:156` records that `--check`
     runs nowhere.
   - **Ruling:** regenerating one row of an already-stale corpus is not in scope and would hide
     that corpus's real state. File a stateless tooling successor that either regenerates
     `fixtures/graph/v1` and wires `graph_fixture --check` into CI, or deletes the checker.
     Deviation 5 should say "already stale; moves by a further 180 B".
4. **Low: a gap in the policy script (tooling successor).** Mutation 936-7 shows
   `scripts/check-realtime-policy.sh` does not match the turbofish form `.collect::<`. The
   counting-allocator tests catch it. This needs its own issue.
5. **Accepted deviation (gate 5 wording).** Gate 5 pins growth by the charged bound (emitted ops x
   20 B), not by the bound tables' actual lengths. That is the only possible form: the estimate is
   admitted at compile time, and the lengths are decided at bind. The bound is a true upper bound
   for every plan:
   - **Dispatched-unit table:** `active_units.len()` <= units <= ops <= spec nodes, which equals
     `emitted_op_count`. `has_valid_structural_layout` (`lib.rs:1158-1180`) requires the
     dependency levels to partition the spec nodes exactly.
   - **Copied-claim table:** `source_input_buffers.len()` <= claims <= spec nodes. Claims must be
     strictly ascending (`lib.rs:1903`), and each one is resolved by `node_index`.

   Neither bound depends on how many units are active, so a plan where every unit is dispatched
   is covered too.

### What holds (re-run or checked by me)

- **Class A.**
  - *What the predicate skips:* `unit_inert` (`runtime.rs:2713`) names only a plain
    `RuntimeUnit::Op` of kind `SourceInput`. For that kind, `execute_op` returns before any
    staging, reduction or split-pair code, and `execute`'s resident lookup only matters for a
    `Bank`.
  - *No hidden observers:* `observed` is set only in `new_with_observation_activation`, from
    `has_observers()`, the same `op.observers` slice. Activation entries are emitted from the same
    `observer_nodes(node, taps)` rows that `build_op` folds, alias taps included. So an inert unit
    has no cursor entry.
  - *No later rebinding:* `GraphExecutor::new` is the only constructor, and `units`,
    `identity`, `output_unit` and `op.observers` are never reassigned after bind. A plan swap
    builds a new executor, so the table is rebuilt from the rows render reads.
  - *Walks that ignore the table:* `complete_pending`, invalidation and the census still walk
    every unit.
  - *Selective observation, tested:* a scratch test (not committed) bound the ring plan with every
    observer controlled, over `Plain`, `ObservedInput`, `ObservedAlias`, `TrackDelayed` and
    `SendTap`, at frames 1 and 16. It activated every handle, removed K's meter at block 4 and
    re-added it at block 7, over 12 blocks. The digests of every host word and meter frame are
    identical with the table loop and with a dispatch-every-unit loop, and K's meter published 9
    frames both ways.
  - *The base-commit oracle:* with the loop dispatching every unit and only the per-block count
    check relaxed, gates 1-3 match `INERT_PRE_CHANGE`.
- **Order.** The table is `0..units.len()` filtered, so it is ascending. The Output op and every
  `Bank` are never inert. A resident predecessor `units[index - 1]` that is a `Bank` is therefore
  dispatched immediately before its successor.
- **Tests.** The rt9 source pin keeps every expected statement, count and control, and adds two
  more. The three required mutations are RED as recorded:
  - 936-1: graph 5 of 113 red; bits-only, gate 2's digest is red (it reads the Plain digest).
  - 936-2: 3 of 113 red; bits-only, gate 3's digest is red.
  - 936-3: `BASE_DIGEST`, `the_folded_master_is_the_reductions_own_bits` and the driver-fed
    equality are red.

  The mutants 936-1a and 936-1b stay green (113 of 113), and they are truly equivalent: for a
  plain op, `observed` is always `!op.observers.is_empty()`, because that is how it is set.
- **Gates re-run on HEAD, all passing:**
  - `cargo test -p graph` with and without `test-support` (rt10, rt1 and rt9 alloc included);
    `-p console-workload`, `-p graph-compiler`, `-p host-core`, `-p audit`, `-p bench`.
  - `check-realtime-policy.sh` (54 regions), `check-graph-policy.sh`, and
    `check-graph-determinism.sh` (100/100).
  - `cargo fmt --all --check`, workspace `clippy -D warnings`, and `RUSTDOCFLAGS='-D warnings'
    cargo doc`.

  No `unsafe` was added to `crates/graph`.
- **Scope.** The branch changes only authorized paths.

### For attempt 2

The coordinator authorizes `crates/capi/tests/resource_lifecycle.rs`. The implementer re-pins it
as described in finding 1, corrects deviation 5, and re-runs the `test-debug-a` command in full.
No production change is needed.

## Attempt 2 evidence

Implementer: attempt 2, on `0eee137d` (Sol's attempt 1 verdict). The scope was amended by the
coordinator's issue comment of 2026-09-26: `crates/capi/tests/resource_lifecycle.rs` is
authorized. No production change was made. Nothing was pushed, and no timed benchmark was run.

### The blocking finding: the capi resource re-pin

`crates/capi/tests/resource_lifecycle.rs` restates the two #936 tables as independent primitives,
following the file's own convention (#470, #779, #816):

- **`executor_table_rows()`:** `bytes::<u32>(82)` and `bytes::<(usize, u32)>(82)`, asserted to be
  `(328, 1_312)`. That is the fixture's 82 emitted ops, the same count #470's reservation uses, at
  one entry per op.
- **`frozen_scratch_report`:** each of the three single-plan graph figures
  (`graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes`, `graph_metadata_bytes`) adds
  `executor_table_bytes` = 1,640. A comment derives it: 82 x 4 + 82 x 16. The largest allocation
  does not move, because 1,312 is below the 49,167 graph metadata row.
- **`graph_owners()`:** two appended rows, the executor dispatched-unit table reservation and the
  executor copied-claim table reservation. They are appended so the positional
  `graph_rows[5..13]` oracle stands. `primitive_replacement_oracle`'s effective-owner authority
  takes `+ 2 * executor_table_bytes`, so omitting either row, or miscounting it by one byte, is
  refused.
- **The double-live pin:** `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`
  asserts `executor_table_bytes == 1_640`, and `oracle.graph` takes `+ 2 * executor_table_bytes` =
  +3,280.

**Discrimination.** With `crates/graph/src/{lib,runtime}.rs` put back to `64b155d0` and the new
pins kept, the test goes red on exactly those rows. The live report gives 54,763 / 235,528 /
235,528 against the pinned 56,403 / 237,168 / 237,168: exactly 1,640 on each of the three graph
figures, with every other field equal. On the branch it is green, 4 of 4.

### Other findings

- **Finding 3:** deviation 5 of the attempt-1 evidence is corrected in place. The fixture was
  already stale, and this issue moves it by a further 180 B. It is not regenerated (#947).
- **Findings 2 and 4:** the browser expected-resources rows are left to the batch boundary, and
  `hosts/host-web/tests/browser-v1/expected.json` is untouched. The policy-script gap is a tooling
  successor.

### Gates re-run on this tree

| check | command | result |
|---|---|---|
| CI `test-debug-a` | the job's exact `cargo test --locked --workspace --all-targets --exclude ... --features builtins-compiler/test-support,source/test-support,graph/test-support,engine/realtime-audit` (`qualification.yml:476`), plus `--no-fail-fast`; then `cargo run --locked -p host-native` | PASS: exit 0, 108 test binaries and doctests, 1,471 passed, 0 failed, 10 ignored; the host smoke passes |
| focused suites | `cargo test --locked` for `-p graph`, `-p graph --features test-support`, `-p graph-compiler`, `-p console-workload`, `-p host-core` and `-p capi` | PASS: 117, 124, 92, 36 (2 ignored), 174 (2 ignored) and 36 passed; none failed |
| lints | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| policy | `scripts/check-realtime-policy.sh`, `scripts/check-graph-policy.sh`, `scripts/check-graph-determinism.sh` | PASS: 54 regions; PASS; 100/100 |
