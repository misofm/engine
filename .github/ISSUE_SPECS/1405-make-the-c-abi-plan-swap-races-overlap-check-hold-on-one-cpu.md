# Make the C ABI plan-swap race's overlap check hold on one CPU

Test-defect follow-up found by the contention gate of *Re-anchor the C ABI live-edit race's source
before its final-block check* (#1404). That gate repeats the whole `capi` test suite on one CPU. Both
tests in `crates/capi/tests/plan_swap_race.rs` fail there, intermittently, on their non-vacuity
assertion. Under the owner's correctness-first rule, an intermittent failure is a defect to fix,
not a flake to rerun. No production code is implicated.

## Problem (verified on `main` at `6fb211594`; the file is unchanged by #1404)

**The assertion.** `race_plan_swaps` (`crates/capi/tests/plan_swap_race.rs:428`) ends with
`assert!(control.overlapped > 0, "vacuous: no control call overlapped a render call")` (`:572`).
`Control::step` counts an overlap when `in_render` is true both before and after one of its two
dequeue calls (`:318-329`). The render closure sets `in_render` around
`miso_engine_v1_render_f32_planar` (`:509-514`).

**Why it is scheduler-dependent.** Since #1273 (`1338b063c`), the race runs through
`bench_support::producer::render_while_producing`. Each block waits until the producer has queued
its PCM (`Control::ready`, `:401-404`), and `Control::step` feeds one block per call (`:314`;
`feed` at `:254`). So the two threads alternate. On a single CPU they never run at the same time. An overlap needs the
scheduler to preempt the render thread inside a render call and then run a dequeue call before the
render resumes. Nothing in the test makes that happen. The first test commits 24 swaps, so it
renders only about 50 to 60 blocks, and no preemption may land inside one of them.

The #1404 live-edit race uses the same counter and does not have this problem. Its render thread
renders back to back, unpaced. One invocation on one CPU with two busy loops counted 3146
overlaps.

## Evidence

From #1404's 1-CPU suite gate: debug binaries at `6fb211594` (this file is unchanged there), 200
suite iterations in 8 lanes. Each lane was pinned with `taskset` to one CPU shared with one busy
loop, on a host already loaded by other work.

- In 89 of the 200 iterations, `plan_swap_race` failed.
- `control_calls_racing_plan_swapping_renders_never_wedge_replacement` (24 swaps, no readers)
  failed 83 of 200.
- `resource_queries_racing_plan_swaps_always_find_the_published_row` (200 swaps, two readers)
  failed 28 of 200.

All 111 failures were this assertion, at `:572`. In the same iterations, the `capi` unit tests and
`resource_lifecycle` failed 0 times. Unpinned on an idle multi-core host, the binary passes.

## Decisions (for the brief to confirm)

- **D1. Do not weaken the guard.** It defends the #1042 F1 coverage: a control call that falls
  inside a plan-swapping render call. Deleting it, or skipping it below two CPUs, would let the
  race go vacuous silently.
- **D2. Candidate shapes, to be decided in the brief:**
  - (a) After the planned swaps, keep racing (more blocks, more control steps, and further swaps
    if needed) until `overlapped > 0`, within a stated bound. Fail only at that bound.
  - (b) Count overlaps from both sides: the render thread also records whether a control call was
    in flight at the start or end of its call.
  - (c) Force a deterministic interleaving inside the render call. The C ABI has no hook for this,
    so the brief must show one exists or reject this shape.

  Prefer a shape whose failure on correct code needs a scheduler that never preempts a render call
  within the whole bound, and state that probability argument in the test's doc comment.

## Authorized paths

- `crates/capi/tests/plan_swap_race.rs`.
- This spec.

## Non-goals

- Production code. `bench_support::producer` is changed only if the brief shows the guard needs it.
- The #1404 live-edit race.

## Objective gates

1. Both `plan_swap_race` tests, at least 200 invocations each with `taskset` on one CPU shared with
   one busy loop, 0 failures. Record the number of overlaps each invocation observed.
2. The guard still bites: with the render closure changed so that no control call can run during
   a render call (for example, holding the producers' lock across `render`), the test fails with
   the vacuity message within its bound. This is PR evidence.
3. `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`; `cargo run --locked --release -q -p realtime-policy` and `cargo test --locked -p realtime-policy`;
   `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`;
   `test-debug-a`.

*Test value.* There is no new test. The rewritten guard turns red on the same defect as before, a
race that no longer overlaps a control call with a render call (gate 2). It no longer turns red
because a single CPU was not preempted at the right moment.

## Dependencies

- None. It may land before or after #1404.

## Decision and evidence record (attempt 1, 2026-10-24)

- **D2 = (a)** (root, from the scope note). Separate "swaps admitted" (`Control::is_done`) from
  "race over" (`Control::race_over`). `done` is stored only when every swap is admitted and either
  `overlapped > 0` or `OVERLAP_BLOCKS = 4096` blocks have rendered since the last admission (or the
  race failed). Feeding, both dequeues and the query go on after the last swap. The change covers
  all three callers: `race_plan_swaps(24, 0, false)`, `(200, 2, false)` and `(64, 0, true)`.
- **Gate 1 count (root decision):** 50 invocations per test, not 200.
- **Gate 1 result: FAIL.** Debug binary, 150 invocations (50 per test), 6 lanes on CPUs 26-31,
  each pinned with `taskset` to one CPU shared with one busy loop.
  - 24 swaps: 31/50 pass.
  - 200 swaps, 2 readers: 50/50 pass.
  - 64 back to back: 3/50 pass.
  - Failures: 57 at the overlap assertion, after exactly 4096 more blocks with 0 overlaps. 9 at
    #1310's `withdrawn > 0 && taken > 0` assertion, a separate scheduler-dependent guard.
  - Passing runs: median 2 overlaps (24 swaps), 12 (200 swaps), 2 (back to back). The first
    overlap came at most 1336 blocks after the last admission.
- **Finding: the probability argument does not hold.** A vacuous run stays vacuous for 4096 more
  blocks, so the overlap chance is a property of the whole run, not an independent chance per
  block. A likely cause, not proven: under EEVDF (kernel 6.8) each `yield_now` pushes the yielding
  thread's virtual deadline forward. In the paced mode the control thread yields after every step,
  and the render thread renders back to back while PCM is queued. When a tick preempts the render
  thread inside a render call, the busy loop or the render thread is picked before the control
  thread. The control thread then runs only when the render thread yields, which is never inside
  a render call.
- **Diagnostic, not committed:** in the back-to-back loop, a 50 us `sleep` replaced `yield_now`
  after the last admission. Of 22 pinned runs, every run that reached the overlap assertion passed
  it (2 overlaps each). The failing runs failed #1310's `taken > 0` assertion instead. The paced
  mode's yield is in `bench_support::producer`, which is outside this issue's paths.
- **Gate 2 (PR evidence): PASS.** With `in_render.store(true, ..)` commented out, each test failed
  with "vacuous: no control call overlapped a render call" after 4096 blocks past its last
  admission (4146, 4510 and 4160 blocks, in 5.2 s, 9.0 s and 2.8 s). The line is restored.

## Root decisions for attempt 2 (2026-10-24)

- The attempt 1 evidence stands. The e^-65 argument was invalid, because a vacuous run stays
  vacuous. The likely reason is that under EEVDF each `yield_now` moves the yielding control
  thread's deadline later, so a tick that preempts the render thread mid-call does not hand the CPU
  to the control thread.
- **D2 = (a) plus a sleeping control thread.** `OVERLAP_BLOCKS` and `race_over` stay. After the
  last admission, the paced race stops rendering, and `race_unpaced` runs the control side
  unlocked: feed, dequeue, query, then `sleep(OVERLAP_SLEEP = 50 us)`, until the race is over.
  The back-to-back mode uses the same loop: `yield_now` while edits remain, the sleep after that.
  `tools/bench-support` is unchanged.
- **#1310's `withdrawn > 0 && taken > 0` guard (third caller) is folded into the race's end.**
  Implementer finding: these outcomes are counted only inside a command's withdrawal
  (`crates/control-plane/src/control.rs`, `withdraw_pending_candidate`), so they cannot change
  after the last admission. The fold is therefore in `is_done`: back to back, `swaps` is a floor.
  The race admits further edits, in pairs to end on the boot model, until both outcomes are seen
  or `edits_bound = 2 * swaps` (128) edits are admitted.

## Attempt 2 evidence

- **Gate 1: FAIL, 1 of 150.** Same harness as attempt 1: 6 lanes on CPUs 26-31, each pinned to
  one CPU with one busy loop.
  - 24 swaps: 50/50 pass.
  - 200 swaps, 2 readers: 50/50 pass.
  - 64 back to back: 49/50 pass.
  - The overlap guard passed in all 150 runs.
  - The one failure was the #1310 guard, after the full bound: "128 edits ... 71 withdrawn,
    0 taken". So `Taken`, too, is a per-run property on one CPU, and more edits do not rescue it.
  - Per-run overlap counts for passing runs were not captured: the lane script matched the
    attempt 1 print. The gate was not rerun.
- Gates 2 and 3 were not run for attempt 2: root's instruction was to stop if gate 1 fails.
