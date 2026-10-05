VERDICT: PASS

# #1251 attempt 1 -- adversarial verification

Commit `b3095ca2a` against base `3854c03bb`. Exported with `git archive` to
`/tmp/claude-1002/v1251/src`; built and tested only there, `CARGO_TARGET_DIR=/tmp/claude-1002/v1251/target`.
Gate-1 plants were applied to separate exports at the base (`/tmp/claude-1002/v1251/pb`) and the commit
(`/tmp/claude-1002/v1251/pa`), using my own env-gated plant script (`/tmp/claude-1002/v1251/plant.py`), not the
implementer's binaries. The worktree was not touched.

No BLOCKER, no MAJOR. Every site now fails instead of hanging, and I reproduced that for all six sites. No
assertion, count or iteration number changed: a line diff of every `assert`/`expect`/count/loop line shows
additions only. Gates 2 and 3 are green. The D4 outcome is correct and I reproduced it.

## Findings

### MINOR-1 -- the worker's realtime doc comment now documents `WORKER_DEADLINE` (`tools/audit/src/builtins_graph.rs:113-120`, `:146-149`)

`WORKER_DEADLINE` and `await_worker` were inserted between `run_retirement_worker`'s original doc block and
the function, so the original paragraph now opens the doc of the constant. Lines 113-120 are one rustdoc
block:

> Run only the audit-local, off-render retirement ownership handoff. Before control disarms the graph markers
> this loop can reach only the move-SPSC poll, atomic loads/stores, and a processor spin hint. ... / How long
> the control thread waits on the retirement worker ...

`run_retirement_worker` got a new doc (`:146-149`) that leaves out what the worker loop can reach while the
markers are armed. That is the audit's own realtime-reachability claim, and it is now attached to the wrong
item and missing from the function it describes.

**Fix.** Move `WORKER_DEADLINE` and `await_worker` above line 113 (or below the worker), and put the original
paragraph back on `run_retirement_worker`, followed by the new `ended`/`stop` paragraph. Add one clause to
"thread exit occur only after control sends its sole command": on a control-thread panic, control's
`StopOnDrop` now stops the worker inside the armed lifetime, and only on that failure path.

### NIT-1 -- site 1 reads the clock on almost every query (`crates/capi/src/ffi.rs:2540-2549`)

`progress.1.elapsed()` runs on every loop iteration that saw no new block, which is nearly every query. I
measured one sample each, under load average ~110: 9,110,142 queries in 5.5 s at the base and 7,839,803 in
5.7 s after the change, about 17 % fewer per second. The test's claim is that queries run concurrently with
render, so fewer racing queries slightly thins the interleavings. Site 4 already avoids this by reading the
clock once per 1,024 polls.

**Fix.** Use the same polling stride here.

### NIT-2 -- `format!` on the happy path of the site-6 lockstep (`crates/capi/src/runtime/tests.rs:3037`, `:3050`)

Each side allocates a `String` per block to build a message that is only used if the wait fails. Both calls
are outside the render call, so D2 holds. But the pattern would break D2 if someone copied it into an armed
wait.

**Fix.** Pass `block` and a `&'static str` to `await_lockstep`, and format only inside the panic.

### NIT-3 -- the D4 record leaves out a material side effect (spec, attempt record, "D4 outcome")

Sites 1 and 6 name `bench_support` in capi's lib unit tests. That links its `#[global_allocator]` into the
capi lib test binary, which used `System` before. `nm -C` on the planted binaries shows
`AuditedAllocator as GlobalAlloc`: 0 matches at the base and 4 after.

The allocator runs in `Mode::Abort`. All 70 capi lib unit tests now abort the whole binary (SIGABRT, no test
name) on any allocation inside an armed render scope, including the allocation a panic makes inside render.
That is green today and is what D4 conditioned on, but the record should say it plainly.

**Fix.** Add one sentence to the D4 outcome.

### NIT-4 -- the third private `StopOnDrop` (`crates/capi/tests/plan_swap_race.rs:211`)

The attempt record correctly lists this copy as found and not fixed, because the file is outside the
authorized paths. Root should file it as a follow-up so it does not live only in this record. That file
already links `bench_support`, so the dedupe is trivial there.

### Checked and not a finding

- **Render-scope waits stay allocation- and lock-free.**
  - Site 5's in-scope wait (`conformance.rs:85-88`, `await_flag`) is an atomic load, `Instant::now()` and
    `spin_loop`. The test's own final `assert_eq!(snapshot, default)` stays green, which is direct evidence
    that the wait allocated nothing in the armed scope. A mutation that allocates there turns that assertion
    red (see M5 below). The `Barrier` it replaced took a mutex and a futex inside the scope.
  - The audit's waits (`await_worker`) run before the first and after the last `RT_BEGIN`/`RT_END`
    interval. The all-TID strace gate passes: `intervals: 4`, `violations: 0`.
- **Deadline inside an armed scope.** `PLANT5H` hangs the worker. The 10 s deadline fires inside
  `in_render_scope` and still prints its message, because the test sets `Mode::Count`, so the panic's
  allocation is counted rather than aborted.
- **Miri.** `ffi.rs` keeps a `cfg!(miri)` branch, so I ran that test under `cargo +nightly-2026-08-20 miri`.
  Base and commit fail identically on x86_64 (`llvm.x86.sse.stmxcsr` unsupported), so whether the new
  per-block deadline holds under Miri cannot be assessed on this host. The commit introduces no regression
  there.
- **Unbounded waits that remain.** The site-2 worker still spins on `stop` and the site-4 start barrier
  still has no bound. In both cases the waiter is released on every exit of the peer: `stop` is the
  control thread's `StopOnDrop`, and the barrier sits before anything that can fail. A non-panicking hang of
  the control thread would hang the scope owner regardless, so a deadline there would add nothing.
- **Release `audit` binary.** It never hung: the workspace `[profile.release]` sets `panic = "abort"`
  (`Cargo.toml`).
- **Authorized paths.** Every changed file is on the list. `Cargo.toml` is unchanged: the base
  `crates/capi/Cargo.toml` already has `bench-support.workspace = true` under `[dev-dependencies]`. The
  `ACCEPTED_MANIFEST_SHA256` line is untouched.
- **`render_while_producing`.** No site uses it, which matches the spec's non-goal (none of the sites is a
  render loop against a control producer).

## Gate 1 -- fail, not hang (reproduced; `timeout -s KILL 120`)

Same env-gated plants at the base and at the commit. Times are wall-clock.

| Site | Plant | Before (3854c03bb) | After (b3095ca2a) |
|---|---|---|---|
| 1 | render thread, block 100 | 137 (120 s) | 101 in 0.3 s: `PLANTED site1 render` |
| 2 `run_audit`, debug binary | control, after the first render | 137 (plant printed, then hung) | 101 in 0.0 s: `PLANTED site2 main` |
| 2 `run_audit`, debug binary | worker, before readiness | 137 (plant printed, then hung) | 101 in 0.1 s: `PLANTED site2 worker before ready`, then `the retirement worker ended before readiness` |
| 2 test `issue070_...` | control, after publishing B | 137 | 101 in 0.0 s: `PLANTED site2 test main` |
| 2 test `issue070_...` | worker, on Reclaim | 137 | 101 in 0.0 s: `PLANTED site2 worker`, then `the retirement worker ended before a reclaimed plan` |
| 2 test `issue070_...` | worker, before readiness | 137 | 101 in 0.0 s: `... ended before readiness` |
| 3 | consumer, item 500,000 | 137 | 101 in 2.8 s: `PLANTED site3 consumer`, then `spsc stress: the consumer ended while the producer waited at item 500128` |
| 3 | producer, item 500,000 | 101 in 1.2 s (never a hang direction) | 101 in 0.7 s |
| 4 | writer, window 500,000 | 137 | 101 in 0.1 s: `PLANTED site4 writer` |
| 5 | worker, after its probe | 137 | 101 in 0.0 s: `PLANTED site5 worker` |
| 5 | worker hangs after its probe (deadline path) | 137 | 101 in 10.0 s: `allocation-control: the worker's end did not arrive within 10s` |
| 6 | renderer, block 2 | 137 | 101 in 0.1 s: `PLANTED site6 render`, then `lockstep: the peer ended before the renderer consumed block 2` |
| 6 | producer, block 2 | 137 | 101 in 0.1 s: `PLANTED site6 producer`, then `lockstep: the peer ended before the producer submitted block 2` |

The implementer's table matches mine row for row. Its "at once" for site 3 is about 1-3 s here, because the
stress has to reach item 500,000 first.

## Mutations (red on mutation, green on revert)

- **M3, production defect in `Consumer::try_pop`** (`spsc.rs`): the consumer index is not advanced at
  success 300,000, so one item is duplicated.
  - Before: 137, hung 120 s.
  - After: 101 in 0.2 s with `left: 300000 right: 300001`, then
    `spsc stress: the consumer ended while the producer waited at item 300128`.
  - Revert: green (gate 2).
- **M5, the armed wait allocates** (`conformance.rs`, one allocation after `await_flag(&ended)` inside
  `in_render_scope`, behind `MUT5`).
  - Mutated: 101 with `left: AuditSnapshot { allocations: 2, deallocations: 2, .. } right: AuditSnapshot { allocations: 0, .. }`.
  - Unmutated (same binary, `MUT5` unset): ok.
- **D4 conflict.** Adding `use bench_support::producer::StopOnDrop;` to `crates/capi/tests/resource_lifecycle.rs`
  makes `cargo check -p capi --test resource_lifecycle` fail with
  `the #[global_allocator] in this crate conflicts with global allocator in: bench_support`. Declining the
  dedupe was correct.

## Gates 2 and 3 (in the export)

- **Unit and integration tests.** `cargo test --locked`:
  - `-p capi`: 70 + 2 + 11 passed.
  - `-p engine`: 38 + 4 + doc tests passed.
  - `-p compressor --test conformance`: passed.
  - `-p audit`: 33 passed.
  - `-p bench-support`: 44 passed.
- **Release audit.** `cargo build --locked --release -p audit`: ok. Its release binary contains the new
  messages.
  - `trace-builtins-graph-audit.sh`: PASS, issue-070 all-TID trace, `{"intervals":4,"violations":0}`.
  - `test-realtime-audit-probes.sh builtins-graph`: ok (9 operations).
  - `test-builtins-fixtures.sh`: ok.
- **Formatting and clippy.** `cargo fmt --all -- --check`: ok. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`:
  ok, and I re-ran it on `bench-support` after touching `producer.rs`.
- **Policy scripts.** All ok:
  - `check-workspace-policy.sh` / `test-workspace-policy.sh`
  - `check-bench-policy.sh` / `test-bench-policy.sh`
  - `check-realtime-policy.sh` / `test-realtime-policy.sh`
  - `check-conformance-boundaries.sh`

## Test value

There is no new test. Each rewritten test keeps its claim, and the new part of each one is that its failure
now reports instead of hanging.

1. `plan_queries_are_pure_and_concurrent_with_render`: a plan query that reads a field render is mutating,
   or a render that returns non-OK while queries run, turns it red. A render failure now reports in 0.3 s
   instead of hanging (planted).
2. `issue070_retirement_worker_is_ready_quiescent_and_owns_only_a` and `run_audit`: a retirement handoff
   that destroys a plan on the wrong thread or reclaims the wrong epoch turns them red. A failure on the
   control thread or the worker now reports at once (planted on both sides).
3. `concurrent_spsc_stress`: an SPSC ring that loses, duplicates or reorders an item turns it red, now in
   0.2 s where the base hung (M3).
4. `a_million_windows_are_read_whole_and_in_order`: a torn or regressing seqlock read turns it red. A writer
   failure now ends the reader instead of hanging it (planted).
5. `scoped_allocator_attribution_controls_are_live_and_isolated`: an audit that charges another thread's
   allocation, or the handshake's own allocation, to the armed thread turns it red (M5). A worker failure or
   hang now reports (planted).
6. `barrier_schedule_separates_one_source_producer_from_exclusive_render`: a source-producer/render
   interaction that yields a non-OK render or non-finite PCM turns it red. A failure on either side now names
   the other side's wait (planted on both sides).
