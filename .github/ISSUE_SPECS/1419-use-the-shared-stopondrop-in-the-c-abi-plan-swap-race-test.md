# Use the shared StopOnDrop in the C ABI plan-swap race test

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the verdict of *Make concurrent tests fail instead of hanging when a thread panics* (#1251), NIT-4
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1251-attempt1.md`). Test code only.

## Problem (verified on `main` at `0a1176b3b`)

#1251 made `bench_support::producer::StopOnDrop` public
(`tools/bench-support/src/producer.rs:26-39`): a guard that sets an `AtomicBool` with `Release`
when dropped, so a peer thread stops however the owning thread's loop ends. Two crates use it
(`crates/capi/src/ffi.rs:2512`, `crates/capi/src/runtime/tests.rs:3013` and `:3043`,
`tools/audit/src/builtins_graph.rs:8`).

`crates/capi/tests/plan_swap_race.rs` keeps a private copy with the same body (`:209-217`), used
once (`:496`, dropped at `:525`). That file already links `bench_support` and uses its allocator and
producer (`:4-6`, `:15-16`), so nothing stops it from using the shared guard.

Two other private copies are justified, and stay:
- `crates/capi/tests/resource_lifecycle.rs:2031-2042`: naming anything from `bench_support` links
  its `#[global_allocator]`, which conflicts with that file's counting allocator (#1251 D4). Its
  doc comment ends "The plan-swap race keeps its own copy in `plan_swap_race.rs` (#1273)", which
  becomes false.
- `crates/engine/tests/observation_transport.rs:110` and `crates/engine/src/realtime/spsc.rs:536`:
  `bench-support` depends on `engine`.

## Decisions

- **D1.** Delete the private `StopOnDrop` in `plan_swap_race.rs` and import
  `bench_support::producer::StopOnDrop`. The use at `:496` and the `drop` at `:525` stay as they
  are. Keep the comment's reason (stops the reader threads however the render loop leaves their
  scope) at the use site.
- **D2.** In `resource_lifecycle.rs`, replace the last sentence of the `StopOnDrop` doc comment
  with one that says `plan_swap_race.rs` uses the shared guard. The rest of that comment stays.

## Authorized paths

- `crates/capi/tests/plan_swap_race.rs` (D1 only)
- `crates/capi/tests/resource_lifecycle.rs` (D2's one sentence only)
- This spec

## Non-goals

- The two justified copies. The race tests' timing, contention and overlap checks: *Make the C ABI
  plan-swap race's overlap check hold on one CPU* (#1405) owns them.

## Hazards

- **#1405 edits `plan_swap_race.rs`.** Either order works; the slice that lands second rebases.
  This one changes only the guard's definition and import.
- The shared guard's field is public (`StopOnDrop(pub &AtomicBool)`); the tuple constructor call at
  `:496` compiles unchanged.

## Objective gates

1. `cargo test --locked -p capi --test plan_swap_race` passes.
2. **A failure still ends the test (PR evidence).** Plant a failing assertion in the render loop's
   closure of the reader-thread test (the scope that holds `stop_readers`). The test binary fails
   with that assertion and exits within its deadline; it does not hang. Revert.
3. `cargo test --locked -p capi --test resource_lifecycle` passes.
4. `cargo fmt --all -- --check`;
   `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`;
   `bash scripts/check-workspace-policy.sh`.

*Test value.* No new test. Gate 2 shows that the shared guard keeps the behaviour #1251 gave the
private copy.

## Evidence

- Gate 2's planted failure: the message and the wall time.
- Each gate command and its exit status at the PR head.

## Dependencies

- None. *Make concurrent tests fail instead of hanging when a thread panics* (#1251) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
