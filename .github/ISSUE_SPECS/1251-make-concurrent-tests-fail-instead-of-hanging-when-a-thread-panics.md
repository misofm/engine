# Make concurrent tests fail instead of hanging when a thread panics

Tooling issue, the follow-up that *Make the live-route allocation gates wait on the queue and fail
instead of hanging* (#1250, closed by PR #1249) left out of scope. Its verdict (Sol, MINOR-1)
listed the sites below. No production code changes.

## Problem

In each test below, one thread waits on another without a bound, and the thread it waits on
releases it only on its normal path. When that thread's assertion fails or it panics, the waiter
never wakes. `std::thread::scope` and `JoinHandle::join` then wait forever, and libtest, having
captured the panic message, prints only "has been running for over 60 seconds" until the CI job's
timeout cancels it. A red test becomes a hung job with no message.

None of these is a scheduling race that fails a correct build (#1250's other half); each hangs
only after something else has already gone wrong. But that is exactly when the message matters.

## The pattern to follow

`bench_support::producer::render_while_producing` (`tools/bench-support/src/producer.rs`, #1250):

- a drop guard (`StopOnDrop`, `:27-34`) sets the peer's stop flag **however** the loop ends, so a
  panic releases the peer before `scope` joins it;
- every wait on a peer is bounded by a deadline (`QUEUED_DEADLINE`, `:25`) and panics, naming
  what it waited for, when it expires;
- a panic on the peer is observed, not ignored (a poisoned lock is an `expect`).

A channel whose sender is dropped when its thread unwinds (`recv_timeout` returning
`Disconnected`) is an equally good release for a lockstep handshake, except inside an armed audit
scope (see Hazards).

## Sites (verified on `main` at `1cb677a76`)

1. **`crates/capi/src/ffi.rs:2295` `plan_queries_are_pure_and_concurrent_with_render`.** The scoped
   render thread runs `assert_eq!` in a loop and clears `rendering` only after it; the query thread
   spins on `while rendering || queries == 0`. A render failure hangs the scope. This is #1250's
   exact shape.
2. **`tools/audit/src/builtins_graph.rs`, `run_audit` (`:161`, scope at `:183`).** Many assertions
   run inside `std::thread::scope` while the retirement worker spins until `stop`, which is set
   only at the end; and the main thread spins without a bound on
   `reclaimed_epoch_plus_one == 0` (`:242`) if the worker dies. Any failed audit assertion hangs the
   audit binary. The same two shapes are in the test
   `issue070_retirement_worker_is_ready_quiescent_and_owns_only_a` (`:775`; scope at `:797`, spin
   at `:835`).
3. **`crates/engine/src/realtime/spsc.rs:535` `concurrent_spsc_stress`.** A consumer `assert_eq!`
   panic leaves the producer spinning on a full queue forever, and `producer_thread.join()` (joined
   first) blocks.
4. **`crates/engine/tests/observation_transport.rs:105`
   `a_million_windows_are_read_whole_and_in_order`.** The main thread spins on `done` without a
   bound; a writer panic never sets it.
5. **`crates/compressor/tests/conformance.rs:25`
   `scoped_allocator_attribution_controls_are_live_and_isolated`.** A three-`Barrier` lockstep
   (`:60-80`): a worker assertion failure leaves the main thread blocked in `done.wait()`, inside
   `audit::in_render_scope`.
6. **`crates/capi/src/runtime/tests.rs:2001`
   `barrier_schedule_separates_one_source_producer_from_exclusive_render`.** A two-`Barrier`
   lockstep (`:2012-2013`) between a source producer and a renderer: either side's failed
   assertion leaves the other blocked at a barrier.

**NIT (#1250 verdict, NIT-1).** `crates/capi/tests/resource_lifecycle.rs:1505` defines its own
`StopOnDrop`, identical to the helper's private one (`tools/bench-support/src/producer.rs:28`).

## Decisions

- **D1. Every site fails, never hangs.** In each site, a thread that leaves its loop by any path
  (normal end, failed assertion, panic) releases every peer that waits on it, and every wait on a
  peer is bounded by a deadline of 10 s (or the site's own existing bound, if larger) and panics
  with a message naming the site and the wait. A peer's panic still fails the test with that
  panic, or with the deadline's message.
- **D2. Claims and measured windows are unchanged.** Each test keeps every assertion, count and
  iteration number it has, and no lock, allocation or syscall enters a measured or armed region
  that did not have one.
- **D3. One `StopOnDrop` where a dependency already exists.** `bench_support::producer::StopOnDrop`
  becomes `pub` (documented). Sites in crates that already depend on `bench-support` (`tools/audit`,
  `crates/compressor`) use it. `crates/engine` cannot (`bench-support` depends on `engine`) and fixes
  its two sites locally.
- **D4. The capi copy (NIT).** `crates/capi` does not depend on `bench-support` today, and
  `bench-support` installs its audited `#[global_allocator]` (`tools/bench-support/src/alloc.rs:169`)
  in any binary that links it. Add `bench-support` as a capi **dev-dependency** and use its
  `StopOnDrop` in `resource_lifecycle.rs` and in sites 1 and 6 only if `cargo test --locked -p capi`
  stays green with it; otherwise keep capi's own copy, use it for sites 1 and 6 within the same test
  target where possible, and record in the evidence why the dedupe was declined.

## Authorized paths

- `crates/capi/src/ffi.rs` (site 1's test only), `crates/capi/src/runtime/tests.rs` (site 6's test
  only), `crates/capi/tests/resource_lifecycle.rs` (its `StopOnDrop` only), `crates/capi/Cargo.toml`
  (the `[dev-dependencies]` entry of D4 only)
- `tools/audit/src/builtins_graph.rs` (site 2's thread handling only)
- `crates/engine/src/realtime/spsc.rs` (site 3's test only), `crates/engine/tests/observation_transport.rs`
  (site 4's test only)
- `crates/compressor/tests/conformance.rs` (site 5's test only)
- `tools/bench-support/src/producer.rs` (make `StopOnDrop` public)
- this spec

## Non-goals

- Production render, graph, engine or host code.
- Any change to what a test asserts, how many iterations it runs, or what it measures.
- Threaded tests not listed here (#1250's scan found no other producer-progress wait; a new site
  found while working is recorded in the evidence, not fixed here).
- Converting sites to `render_while_producing` itself: its queue-predicate wait is for a render
  loop against a control producer, which none of these sites is.

## Hazards

- **Armed scopes.** Site 5 waits inside `audit::in_render_scope`, which audits this thread's
  allocations. A replacement wait there must not allocate: an atomic flag with a deadline does not;
  a `std::sync::mpsc` channel's first use on a thread may.
- **The audit's syscall trace.** `tools/audit`'s `builtins-graph` run is traced by
  `scripts/trace-builtins-graph-audit.sh` and its probes by `scripts/test-realtime-audit-probes.sh`;
  a new wait or guard must not add a syscall inside a traced interval. `scripts/test-builtins-fixtures.sh`
  copies `builtins_graph.rs` and edits its `ACCEPTED_MANIFEST_SHA256` line by `sed`; leave that line
  as it is.
- **A deadline is not a timeout on correct runs.** Size it so that a correct run under CI load
  never reaches it (each wait today completes in microseconds to milliseconds).

## Objective gates

1. **Fail, not hang (PR evidence, not committed).** For each site, plant a panic in the thread
   that releases the waiter (for site 2, in an assertion inside the scope and, separately, in the
   worker). Before the fix the run is killed by `timeout -s KILL 120` (exit 137); after it the test
   or binary exits with a failure well inside the bound and prints the planted panic or the
   deadline message. Record both exit codes and the message per site.
2. **Nothing else moved.**
   - `cargo test --locked -p capi`, `cargo test --locked -p engine`,
     `cargo test --locked -p compressor --test conformance`, `cargo test --locked -p audit` and
     `cargo test --locked -p bench-support` pass;
   - `cargo build --locked --release -p audit`, then
     `bash scripts/trace-builtins-graph-audit.sh target/release/audit`,
     `bash scripts/test-realtime-audit-probes.sh builtins-graph target/release/audit` and
     `bash scripts/test-builtins-fixtures.sh` pass.
3. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-conformance-boundaries.sh`

*Test value.* No new test. Each rewritten test keeps its own claim and catches the defect it
caught before; what changes is that the failure now reports instead of hanging, which gate 1's
planted panics show once, as PR evidence.

## Evidence

- Gate 1's table: site, planted panic, exit code and message before and after.
- D4's outcome: deduped, or declined and why.

## Dependencies

- None. *Make the live-route allocation gates wait on the queue and fail instead of hanging*
  (#1250) is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
