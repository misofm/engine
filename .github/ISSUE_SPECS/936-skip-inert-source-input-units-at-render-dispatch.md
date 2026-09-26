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
