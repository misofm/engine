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
5. **Stale unchecked fixture.** `fixtures/graph/v1/direct-route.resources.json` records
   `graph_metadata_bytes: 5336`. This change adds 180 B to that fixture's estimate (9 emitted ops x
   20 B). Nothing gates that file: `check-graph-determinism.sh` compares only the fingerprint, and
   #698 records that the checked-in `graph_fixture --check` corpus is already stale. It was not
   regenerated, because it is outside the brief's paths.
6. **Public surface.** `GraphRuntimeMetadataResourceEstimate`, a public struct with public fields,
   gains two fields. No workspace code builds it with a literal (the workspace clippy passes). No
   public trait changed. The new test-only functions are `#[doc(hidden)]` and exist only with
   `test-support`.
