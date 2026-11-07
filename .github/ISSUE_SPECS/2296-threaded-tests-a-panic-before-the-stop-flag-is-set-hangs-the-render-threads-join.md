# Threaded tests: a panic before the stop flag is set hangs the render thread's join

A test-harness defect found while reviewing #2272, filed under the implicit-state-machine tracker
#2258: an implicit stop flag whose release a panic skips. It is the hang-on-panic shape #1251
removed elsewhere (`bench_support::producer::StopOnDrop`, `tools/bench-support/src/producer.rs`).

## Problem
Verified on `71124afcc`.

`crates/control-plane/tests/spectrum_monitor.rs`,
`alternating_configures_against_a_rendering_thread_allocate_nothing_on_render` (:1649), hangs
forever under CPU load, on base as well:
- the render thread loops `while !done` (:1665) inside `std::thread::scope`;
- the control thread sets `done` (:1768) only after its last statement, `assert!(ready > 0, "some
  reads analyzed a window")` (:1767);
- under load, render fills too few blocks between captures, no read comes back `Ready`, the
  assertion panics, `done` is never set, and the scope joins the render thread forever. libtest
  captures the panic message, so the hang is silent.

Any other panic on the control thread (a configure reply, a capture `INTERNAL`, a service step)
skips the same store. The reverse direction hangs too: if the render thread panics (its
allocation assertion, a render error), the configure loop retries `Backpressure` (:1700) with no
deadline.

The `ready > 0` count was also weaker than it looked. In every odd (collection) configure the
first `Select` (:1714 on) reaches the monitor before the configure's plan is adopted and is
refused `spectrum.select.single`; the state machine answers every error with `Cancel` then `Arm`
(:1750), and a collection with no selection refuses every arm `spectrum.busy`, so the 20
collection configures never armed. Every `Ready` came from the 20 single configures.

The same shape, found by searching the control-plane, host-core and capi tests:
`crates/capi/tests/latency_growth.rs`, `a_growth_with_sources_queued_ahead_is_adopted_exact`
(:389). The render worker waits for the feed with `assert!(!done...)` (:401); the control thread
sets `done` (:423) only after its loop, so a failed control assertion (`feed_to`, `apply`,
`service` all assert) leaves the worker spinning and the scope's join waiting. A worker that
panics leaves the control loop (:410-421), which ends only at `blocks >= TOTAL`, spinning forever.

## Contract
- In both tests the flag that releases the peer is set by `bench_support::producer::StopOnDrop`,
  bound right after the spawn, so every exit from the control thread's part of the scope sets it
  before the join (`drop(stop)` on the normal path). Both crates already dev-depend on
  `bench-support`, and neither file has its own global allocator.
- `spectrum_monitor`: the control thread checks `render.is_finished()` wherever it waits on render
  (a backpressured configure, the wait for `Ready`) and panics "the render thread ended before it
  was released"; the backpressured configure has a 30 s `PROGRESS_DEADLINE`.
- `spectrum_monitor`: `ready > 0` is replaced by a bounded wait. The last configure keeps
  capturing past its 25 operations until a read is `Ready`, and panics "no SPECTRUM_CAPTURE read
  came back Ready within 30s ..." after `PROGRESS_DEADLINE`. A slow machine is slower, not red.
- `spectrum_monitor`: in a collection configure, an error answers with a fresh `Select` instead of
  `Cancel`, so a select refused before the plan is adopted is retried. Without it the last
  (collection) configure can never arm, and the bounded wait would always time out.
- `latency_growth`: the control loop also ends when the worker has finished, and the worker is
  joined before `revision` is unwrapped, so the join reports the worker's panic.
- The claims are unchanged: no render call allocates or frees after the first two blocks, every
  configure answers, no capture answers `INTERNAL`; the growth is adopted `EXACT` and continues
  the predecessor bit for bit.

## Non-goals
- No product code changes. No render-path code changes, so no no-bit-moved comparison applies.
- The listed instances below that do not hang on a control panic are not changed here.
- No committed hang test: a hang cannot be a test. The evidence is the injected-failure run below.

## Other instances (searched, not changed)
Searched: every `while !<flag>.load(...)` and `AtomicBool` stop flag in the tests of
`crates/control-plane`, `crates/host-core` and `crates/capi`.
- Already guarded by `StopOnDrop` (#1251): `capi/tests/resource_lifecycle.rs` (#1258 live-edit
  race, its local copy at :2465, bound at :3533), `capi/tests/plan_swap_race.rs` (:550, :675),
  `capi/src/ffi.rs` (:3422), `capi/src/runtime/tests.rs` (:4624, :4654).
- `host-core/tests/session_seek.rs` `run` (render `while !stop` :139, store :241): the render
  thread is a `thread::spawn`, not scoped and not joined on a control panic, so a failed seek
  assertion fails the test without hanging (the thread spins until the binary exits). The reverse
  direction hangs: a render panic leaves `session_thread` spinning on `position` (:192) and on
  `landed` (:207) with no deadline. Follow-up candidate.
- `host-core/src/spectrum.rs`
  `a_cancel_racing_completion_never_returns_the_cancelled_window_over_1000_seeds` (render
  `while !stop` :4623, store :4714): also `thread::spawn`, not joined on a panic, so no hang. Its
  window wait is bounded by a spin count (:4689); its first wait, `progress < 2` (:4647), is not,
  so a render panic in the first two blocks hangs it. Follow-up candidate.
- `host-core/tests/launch_registry_once.rs`: a `Barrier`, no stop flag.

## Gates
- Injected failures, base vs fix (not committed): a control-thread panic before the release and a
  render-thread panic, in each test. Base hangs; the fix fails promptly with the named panic.
- The original hang reproduced under load on base, and the fix passing under the same load.
- Mutations of the fix: remove each guard / liveness check; the injected run hangs or waits out
  the deadline again.
- `cargo test --locked` for control-plane (with `test-support`) and capi; clippy `-D warnings`;
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc`; `scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)
**Natural hang on base (under load).** The test binary and four busy loops pinned to one CPU
(`taskset -c 0`), `--nocapture`, 90 s timeout:
```
test alternating_configures_against_a_rendering_thread_allocate_nothing_on_render ...
thread '...' panicked at crates/control-plane/tests/spectrum_monitor.rs:1767:9:
some reads analyzed a window
exit=124 elapsed=90.05s            (killed by the timeout: the scope never joined)
```
Pinned with no busy loops, base passes in 3.96 s. The fix under the same load passes: 9.08 s with
four busy loops, 20.18 s with eight (the last configure waits for its `Ready` read).

**Injected failures, base vs fix (not committed).** A temporary script inserted two env-gated
panics into each test on both sources: `INJECT_CONTROL`, a control-thread panic just before the
release (base: before `assert!(ready > 0`; `latency_growth`: right after `apply`), and
`INJECT_RENDER`, a render-thread panic part way through (frame 64 quanta; block `BEFORE + 2`).
The env is read before the threads start. No load, 60 s timeout:
```
spectrum_monitor base INJECT_CONTROL exit=124 60.01s | injected control failure
spectrum_monitor base INJECT_RENDER  exit=124 60.01s | injected render failure
spectrum_monitor fix  INJECT_CONTROL exit=101  2.87s | injected control failure
spectrum_monitor fix  INJECT_RENDER  exit=101  0.27s | injected render failure; the render thread ended before it was released
latency_growth   base INJECT_CONTROL exit=124 60.01s | injected control failure
latency_growth   base INJECT_RENDER  exit=124 60.01s | injected render failure
latency_growth   fix  INJECT_CONTROL exit=101  0.49s | injected control failure; the feed stopped
latency_growth   fix  INJECT_RENDER  exit=101  0.55s | injected render failure; render: Any { .. }
```
Every base case hangs; every fixed case fails promptly with the named panic.

**The collection configures never armed (base state machine).** Instrumented replies on the base
state machine, unloaded: every odd configure's first `Select` answers `ValidationFailed
spectrum.select.single`, then `Cancel` `Done`, then `Arm` `Backpressure spectrum.busy` for the
rest of the configure. With the re-select, two unloaded runs counted `Ready` reads of
single/collection 13/10 and 4/1.

**Mutations of the fix (each reverted; injected binaries, no load, 60 s timeout).**
- M1, `spectrum_monitor` guard removed (`done` stored after the loop, as on base):
  `INJECT_CONTROL` hangs (exit 124, 60.01 s).
- M2, `spectrum_monitor` `render_alive` disabled: `INJECT_RENDER` fails only at the deadline,
  30.33 s, "configure 1: still backpressured after 30s" (so a dead render thread does wedge the
  configure loop; the deadline alone bounds it).
- M3, the re-select reverted (the first version of this fix): under four busy loops the bounded
  wait times out, 42.88 s, "no SPECTRUM_CAPTURE read came back Ready within 30s after the last
  configure's 25 captures": the last configure is a collection and never arms.
- M4, `latency_growth` guard removed: `INJECT_CONTROL` hangs (exit 124, 60.02 s).
- M5, `latency_growth` `worker.is_finished()` removed: `INJECT_RENDER` hangs (exit 124, 60.02 s).

**The allocation claim is intact.** A temporary `vec![0_u8; 8]` between the render thread's mark
and delta at frame 40 quanta: the test fails in 0.79 s with `frame 5120`, `left: (1, 1)`,
`right: (0, 0)`, followed by "the render thread ended before it was released". (On base the same
render panic hangs: the `INJECT_RENDER` row above.)

**Test value.** No new test: the change is the harness of two existing tests, so a failure in
either one reports instead of hanging CI. The re-select is the one change to what
`alternating_configures...` exercises: its collection configures now arm and read, which its doc
claimed and base never did.

**Gates (head).**
- `cargo test --locked -p control-plane --features test-support,host-core/test-support`: 11
  binaries, 101 tests, all pass.
- `cargo test --locked -p capi --features control-plane/test-support,host-core/test-support,protocol/test-support,engine/test-support,session/test-support,graph/test-support`:
  11 binaries, 234 tests, all pass.
- `cargo clippy --locked -p control-plane -p capi --all-targets` with those features, and `-p capi`
  with none, `-- -D warnings`: clean. `-p control-plane --all-targets` with no features fails on
  base already (`compile_children_warm` is never used in the lib test, a file this issue does not
  touch); the CI clippy step enables `control-plane/test-support`.
- `cargo fmt --check`: clean. `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p
  control-plane -p capi`: clean. `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- Realtime policy tool: not run; no render-path code changed (test files only).

**Follow-ups (not in this issue).**
- `host-core/tests/session_seek.rs` `run`: a render panic leaves `session_thread` spinning on
  `position` and `landed` with no deadline.
- `host-core/src/spectrum.rs` cancel-race test: its first `progress < 2` wait has no bound.
- Both detach their render thread with `thread::spawn`, so a control panic leaves it spinning
  until the binary exits; a scoped thread with `StopOnDrop` would end it.
