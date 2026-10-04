# Qualify live C ABI edits against a concurrently rendering plan

Core slice 6 of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is the qualification of #1257: the realtime and
acked-batch contracts with a render thread running concurrently, at the exact resource caps, and
under reliable-event backpressure. Anchors verified on `main` at `54b0a1bf8`; re-verify them after
#1257 lands.

## Product outcome

Evidence that a phone app can move faders, mutes and pans while the audio thread renders and while
other edits rebuild the plan, and that:

- the audio thread never allocates, frees, locks or makes a syscall;
- no call returns `INTERNAL`;
- no acked edit is lost;
- the live admission holds the caller's caps to the byte.

No product code changes unless a gate finds a defect. A fix is in scope only if it stays inside
capi's live arm (#1257's `commit_live`). Otherwise stop, record the evidence, and file a bounded
issue.

## Context (verified at `54b0a1bf8`)

All paths are in `crates/capi/` unless named.

- **The race harness.** `tests/resource_lifecycle.rs` already races control calls against
  plan-swapping renders: `race_plan_swaps` (`:1541`) and
  `control_calls_racing_plan_swapping_renders_never_wedge_replacement` (`:1770`). It counts
  allocations and frees with its own global allocator, which the realtime-policy scan exempts
  (`scripts/check-realtime-policy.sh:29`).
- **The exact-cap oracle.** `double_live_oracle_drives_exact_and_one_below_c_caps` (`:1263`)
  derives exact caps from two live reports and drives the C entry point at the cap and one byte
  below it.
- **The render audit.** `tools/audit/src/capi.rs` renders 100,000 calls of the nine-track EQ
  fixture, all inside one `audit::in_render_scope` (`:164-185`). CI validates its one JSON record
  (`.github/workflows/qualification.yml`, the "Validate Issue-544 runtime audit records" step, and
  `scripts/run-aarch64-tests.sh:166-185`): the key set, `calls`, and every forbidden counter are
  exact; `pcm_digest` is checked only for its form. So the record's shape must not change, and the
  digest may.
- **The live arm (#1257).** `commit_live` runs admission (`validate_live_peak`), resolve, room,
  `check_prepared_structural`, push, commit, respond. The live admission is #1053 D8.
- **The reliable lane** has two slots (`src/runtime/compile.rs:98`). Reliable-event backpressure is
  decided before a token exists (`crates/protocol/src/controller.rs:1734-1746`). It is a protocol
  response with status `Backpressure` under C result `RESULT_OK` (the pinned `EVENT_FULL` vector,
  `src/runtime/tests.rs:1903-1908`), not `RESULT_BACKPRESSURE`.
- **Two kinds of `RESULT_BACKPRESSURE`.** After #1257's D8, a full live lane sets the last error
  `control.live.backpressure`; a pending candidate or the epoch lag sets
  `control.plan.backpressure` (`src/ffi.rs:625-631`).

## Decisions

- **D1. The race session.** Use tracks with an empty console, no inserts, no input filter and zero
  `delay_samples`, fed a constant, distinct, non-zero value on each lane. Nothing then keeps state
  that depends on history, so the final comparison holds after any sequence of swaps. Keep every
  pan or matrix edit's `smoothing_samples` at most one quantum, or add the largest one to the
  settle window.
- **D2. Rebuild triggers.** Interleave structural edits that render identically, such as a changed
  content string on an unread source. Do not use `SetSessionId`: it becomes model-only in #1260.
- **D3. Audit edits.** `audit capi` submits live edits between render calls, never inside the
  audited render scope, and keeps its record shape. The one scope around all calls becomes one
  scope per render call, with each submission and each event dequeue outside it. After each edit
  the audit dequeues the reliable events; otherwise every third edit is refused with a protocol
  backpressure response and never reaches a drain.

## Authorized paths

- `crates/capi/tests/resource_lifecycle.rs`.
- `tools/audit/src/capi.rs`.
- `crates/capi/src/runtime/live_tests.rs`, for gate 4 only.
- `crates/capi/src/runtime/control.rs` and `compile.rs`, only to fix a defect that a gate here
  finds inside the live arm.
- This spec.

## Non-goals

- No new feature. No change to the classifier, host-core or the protocol.
- No benchmark.

## Objective gates

Run every command from the repository root.

1. **The race.** Add a two-thread test in `tests/resource_lifecycle.rs`, beside `race_plan_swaps`:
   - The render thread renders continuously through `miso_engine_v1_render_f32_planar`.
   - The control thread submits live fader, mute and pan edits, several per render period, and a
     structural edit every eighth edit (D2). It feeds the sources, seeks again after every swap,
     and drains the events.
   - Run it 20 times. Assert:
     - every render call shows `allocations == 0` and `frees == 0` in the file's counters, after
       warm-up;
     - no call returns `INTERNAL`;
     - every `RESULT_BACKPRESSURE` carries `control.live.backpressure` or
       `control.plan.backpressure`, and the test retries it with a new request ID, at most a
       bounded number of times per edit (as `WEDGE_BLOCKS` bounds it,
       `resource_lifecycle.rs:1546`);
     - after the last edit, render `latency_samples` plus one quantum; the final block is then
       bit-identical to that of a fresh plan compiled from the final committed snapshot and fed the
       same constant source.

   *Test value: it turns red if a drain allocates or frees on the render thread, if a live edit
   that races a swap reaches the retiring plan or is lost (the final block differs), or if epoch
   synchronization returns `INTERNAL` under live traffic.*
2. **The audit.** In `tools/audit/src/capi.rs`, every 64th call submits a live edit before the
   render call: a mute toggle on one track and a pan move on another, alternating. The record's
   shape stays as it is, and every forbidden counter stays 0:
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `./target/release/audit capi`
   - `cargo test --locked --release -p audit -p bench -p console-workload`

   *Test value: it turns red if draining a non-empty fader or matrix lane allocates, locks or makes
   a syscall on the render thread.*
3. **Exact live caps.** Add a test in `tests/resource_lifecycle.rs`, beside the double-live oracle.
   Use a fader edit whose canonical JSON grows the compiled model (for example `-6.0` to
   `-6.0123`):
   - With `maximum_graph_session_plus_plan_bytes` equal to the live peak of #1053 D8, the edit
     returns `OK`.
   - One byte below, it returns `COMPILE_REJECTED` with `graph.resource.limit`, and the revision
     and the queue room are unchanged.
   - Do the same for `maximum_capi_retained_bytes`, and once with a structural candidate pending.

   *Test value: it turns red if the live arm admits an edit whose peak is over the caller's cap, or
   refuses one within it.*
4. **Reliable-event backpressure.** Make two live edits without dequeuing. The third returns
   `RESULT_OK` with a protocol response of status `Backpressure` for the reliable-event queue; the
   revision is unchanged and no strip queue's room changes.
   *Test value: it turns red if a live push comes before the reliable-event capacity check.*
5. **Workspace and policy.**
   - `cargo test --locked -p capi`
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh release` (it runs `audit capi` on arm64) and
     `bash scripts/run-aarch64-tests.sh debug` are CI-only here (`aarch64-release`,
     `aarch64-debug`); record them as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The race's run count and its retry and `BACKPRESSURE` counts.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- Every wait on another thread has a deadline and fails with a message; never hang (#1251).
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
