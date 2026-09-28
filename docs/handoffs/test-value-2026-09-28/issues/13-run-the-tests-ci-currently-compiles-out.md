# Run the test code CI currently compiles out, and scope realtime counters to one thread

Draft, not a GitHub issue. A **gap**, not a cut. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §6 items 3 and 5). Base `a9414c0c`. No ruling
needed.

## Problem

**1. Assertions behind `test-support` features that no CI job enables.**

Features are unified only per `cargo test` invocation.
- `test-debug-b` passes only `--features math/lane` (`.github/workflows/qualification.yml:513-517`).
  So `builtins/test-support` and `parametric-eq/test-support` are off when those crates' own tests
  run.
- `test-debug-a` passes `builtins-compiler/test-support,source/test-support,graph/test-support,engine/realtime-audit`
  (`:487-495`). So `host-web/test-support` and `host-core/test-support` are off. No workspace
  dev-dependency turns them on: checked with `grep test-support` over every `Cargo.toml`.

Compiled out as a result:
- **builtins:** the counter halves of `crates/builtins/tests/meter.rs:924`, `:1403`, `:1511`
  (`#[cfg(feature = "test-support")]` at `:899`, `:909`, `:1024`, `:1034`). The recorded mutations
  K-2, S-1L/R and A-2 would pass CI today.
- **parametric-eq:** the counter legs of the pinned base-bits scenarios in
  `crates/parametric-eq/tests/bank.rs` (`:913`, `:919`, `:1344`, `:1350`).
- **host-web:** the gated helpers at `hosts/host-web/src/tests.rs:2391`, `:2504`, `:2549`, `:3179`
  and `:3267`, and the gated tests that use them. These include **host-web's only zero-allocation check
  on prepared-EQ admission and render**,
  `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation` (`#[cfg(feature =
  "test-support")]` at `:3535`).
- **host-core:** `crates/host-core/tests/observation_demand.rs:23`, `:524` and the asserts at
  `:1511`, `:1970`, `:2040`.

**2. Realtime counters that count other threads.** From 2026-09-01 to 09-28, **7 red CI jobs, 4 of
them on `main`**, were allocation or ownership counters that also counted lazily-initialised statics
or the test harness's own thread:
- capi `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`, 3 reds, fixed
  by `87926988`;
- compressor `uniform_and_ragged_render_paths_allocate_and_free_nothing`, fixed by `ce88bf12`;
- host-core `scalar_point_endpoint` `preparation_resources_and_success_path_are_bounded`, 3 reds,
  fixed by `bc53500a` and `6edf0992`.

Each was fixed per test (`../data/ci-red-jobs.tsv`). None was a real realtime violation.

## Outcome

- **Enable the features.** `test-debug-b` adds `builtins/test-support,parametric-eq/test-support`;
  `test-debug-a` adds `host-web/test-support,host-core/test-support`. Alternatively make those
  counters unconditional in test builds. Either way, every `#[cfg(feature = "test-support")]`
  assertion executes in some required job.
- **One thread-scoped counter.** `bench_support::alloc` gains a thread-scoped allocation scope,
  counting only the render thread between two marks. Every test that asserts "allocates nothing"
  uses it. The per-test workarounds from the four fixes above are replaced by it.

## Scope

Authorized paths:
- `.github/workflows/qualification.yml` (two `--features` lists);
- `tools/bench-support/src/alloc.rs`;
- the allocation-asserting tests in capi, compressor and host-core named above;
- this issue's spec.

## Gates

1. **Nothing is left compiled out.** Across the two debug jobs' feature sets, no `#[test]` body, and
   no assertion inside one, is excluded by `cfg(feature = "test-support")`. Show a
   `cargo test -- --list` diff before and after: the gated tests appear.
2. **The re-enabled assertions discriminate.** Re-apply builtins mutations K-2, S-1L/R and A-2 from
   `crates/builtins/tests/MUTATIONS.md`, and one host-web prepared-EQ allocation (a `Vec::push` in
   admission), in a scratch branch. Each turns a now-running test red in CI's configuration.
3. **The counter is thread-scoped.** An allocation on a *second* thread during a counted render
   window does not fail the scope. An allocation on the render thread inside the window does.
4. **Historical bugs.** `../tools/revert.py` for all four bugs. The red set is a superset of
   today's.
5. **Cost.** Record the job durations. The re-enabled code adds only counters.

## Saving and risk

- **Saving:** none; this closes a gap. It should also end the class of false red on `main`.
- **Risk:** enabling features can surface currently-hidden failures. That is the point: fix them or
  record them.
