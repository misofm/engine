# Make the live-route allocation gates wait on the queue and fail instead of hanging

## Problem

The AArch64 debug leg (`scripts/run-aarch64-tests.sh debug`, job "AArch64 product crate debug
tests (NEON Simd4, FPCR)") was cancelled at its 30-minute timeout twice, each time on a test
that renders against a concurrent control producer:

- main's push run 37116067383 (K3 merge `6f1788f3a`, job 111183138223): graph-compiler's
  `live_routes_render_without_allocating` (#1220 gate 6) "has been running for over 60 seconds",
  then nothing until the cancellation.
- PR #1249's run 37133022847 (head `c3b9357cc`, job 111231872759): host-core's
  `live_sends_render_without_allocating` (#1221 gate 7), the same.

K3's own PR run 37115329466 (head `8c6268967`, job 111182637667) ran both gates on the same
arm64 runner type and passed them, gate 7 in about 0.07 s. `8c6268967` and `6f1788f3a` have the
same tree (`0758c094`), so the hang is intermittent, not a deterministic AArch64 or 4-lane defect.

## Root cause

Both gates are test-harness races. No render code is involved. Each gate spawns a producer that
pushes records in a loop until a `done` flag is set, and renders on the test thread:

```rust
while pushed.load(Acquire) == seen { yield_now() }
seen = pushed.load(Acquire);
render();                        // drains every queue
assert!(after > before, "block {block} applied a record");
```

1. **The wait reads the producer's progress, not the queue.** `pushed` counts successful pushes,
   and `seen` is sampled *before* the render. A push that lands between that sample and the
   render's drain is drained by that render, but it still makes `pushed != seen` at the next
   block. The next render can then start with every queue empty and fail "applied a record". An
   instrumented run on one CPU showed it every time: all 8 pushes were made and counted before
   block 0's render returned, and that render drained them all. Block 1 still started at once,
   because block 0 had sampled `seen` before the last of those pushes, and it found nothing
   queued.
2. **The failed assertion hangs.** The test panics inside `std::thread::scope`. `scope` joins every
   spawned thread before it propagates the panic, and the producer stops only on `done`, which the
   test sets after its last block. The scope waits forever. libtest has captured the panic's
   message, so the log shows only "has been running for over 60 seconds".

Reproduced on x86-64 (AVX2, `Simd8`), which also shows the race does not depend on the lane width:

- `taskset -c 3` on gate 7's debug binary: 7 of 20 runs hung (killed by `timeout 20`), and 4 of
  10 more under `--nocapture` printed `panicked at crates/host-core/tests/live_routes.rs:985:13:
  block 1 applied a record` and then hung.
- Gate 6 on two CPUs shared with three busy loops: 7 of 30 runs hung, one printing `block 7
  applied a record`.

On a lightly loaded machine the producer refills the queues before the next render reaches its
drain, so the race rarely loses. A busy arm64 runner, where other tests of the same binary still
run for about a second after the gate starts, makes it lose. This is likely, not proven.

## Fix

One shared helper, `bench_support::producer::render_while_producing`. Both gates now call it:

- The producers sit behind a mutex. The producer thread locks it only while it pushes, so it
  still pushes while the test thread renders.
- Before each block, the test thread waits until the caller's `queued` predicate holds over the
  locked producers: some queue holds a record (`free() < DEPTH`). Only a render drains a queue,
  so that record is still there when the render starts, whatever the schedule.
- A drop guard sets the stop flag however the render loop ends, so a render's panic ends the
  run with that panic.
- A panic in the producer poisons the mutex, and the next wait reports it. A wait that sees
  nothing queued within 10 s panics.

The producer's sweep treats `Full` as fine: a full queue still holds records for the next block.
Any other refusal panics. Each gate's assertions are unchanged: every block applies at least one
record, and no block after the first allocates or frees on the render thread.

## Authorized paths

- `tools/bench-support/src/producer.rs` (new) and `tools/bench-support/src/lib.rs`.
- `crates/graph-compiler/tests/live_routes.rs` (gate 6 only).
- `crates/host-core/tests/live_routes.rs` (gate 7 only).
- This spec.

## Non-goals

- Production render, graph or lane code: nothing there is implicated.
- Other threaded tests. A scan found no other test that waits on a producer's progress. The
  barrier-lockstep render tests in `crates/capi` would still hang if one side panicked. That is
  a separate follow-up.

## Objective gates

1. Both gates pass on every run under the schedules that hung them before:
   `taskset -c 3` (one CPU), and two CPUs shared with three busy loops.
2. The helper's own tests in `bench-support` pass. Each one goes red under a planted revert of
   the defect it defends (PR evidence, not committed):
   - `every_render_starts_with_a_record_queued`: red if a render starts on the producer's progress
     (a count of `produce` calls) instead of on `queued`.
   - `a_render_panic_ends_the_run`: red, after its 30 s bound and without hanging, if the stop
     flag is set only after the last block.
   - `a_producer_panic_ends_the_run`: red if the wait ignores a poisoned lock.
3. test-debug-a's workspace command, `cargo fmt --all -- --check`,
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, the bench,
   workspace, host-core, graph and realtime policy pairs, and `scripts/check-cross-targets.sh`
   are green.
4. The AArch64 debug leg passes on the PR.

## Evidence

Recorded on `codex/batch-vca` at the attempt-1 commit.

- Gate 1: after the fix, gate 7 under `taskset -c 3` hung or failed in 0 of 40 runs (before: 7 of
  20). Gate 6 under `taskset -c 3` failed in 0 of 40 runs. Under the busy-loop contention, gate 6
  failed in 0 of 40 runs (before: 7 of 30) and gate 7 in 0 of 40.
- Gate 2, planted reverts:
  - Progress-counter wait: `every_render_starts_with_a_record_queued` red with `block 0 started
    with nothing queued` (the first 20 blocks drained nothing).
  - Stop flag set only after the loop: `a_render_panic_ends_the_run` red with `"the run hung"`
    after 30 s.
  - Poisoned lock ignored: `a_producer_panic_ends_the_run` red with `block 0: nothing was queued
    within 10s`.
- Gate 3: test-debug-a's command passed: 1294 tests passed, 0 failed. fmt passed, and so did
  the workspace clippy with `-D warnings`, `cargo doc -p bench-support` with `-D warnings`, the
  bench, workspace, host-core, graph and realtime policy pairs, `check-test-support-ci.py` and
  `check-cross-targets.sh`. No render code changed, so the worklet chain is not a gate here.
- Gate 4: pending the PR's CI run. No arm64 host was available locally.
- Test value, which plausible defect turns each new test red that no existing test catches:
  - `every_render_starts_with_a_record_queued`: a render started on the producer's progress
    rather than on the queue state.
  - `a_render_panic_ends_the_run`: a producer that outlives a render loop's panic, which hangs
    `std::thread::scope`.
  - `a_producer_panic_ends_the_run`: a wait that spins on a queue whose producer has died.
  - The two rewritten gates keep their own test-value answers.

## Dependencies

- None. It lands on the VCA batch branch (PR #1249), whose AArch64 leg it unblocks.
