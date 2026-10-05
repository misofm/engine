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

From #1404's 1-CPU suite gate: debug binaries, each lane pinned with `taskset` to one CPU shared
with one busy loop, on a host already loaded by other work. Each failure was this assertion. Both
tests failed it: `control_calls_racing_plan_swapping_renders_never_wedge_replacement` (24 swaps, no
readers) and `resource_queries_racing_plan_swaps_always_find_the_published_row` (200 swaps, two
readers). The final counts are in #1404's gate results. In the first 28 suite iterations, 10 runs
of `plan_swap_race` failed. Unpinned on an idle multi-core host, the binary passes.

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
   -D warnings`; `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`;
   `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`;
   `test-debug-a`.

*Test value.* There is no new test. The rewritten guard turns red on the same defect as before, a
race that no longer overlaps a control call with a render call (gate 2). It no longer turns red
because a single CPU was not preempted at the right moment.

## Dependencies

- None. It may land before or after #1404.
