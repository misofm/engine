# Re-anchor the C ABI live-edit race's source before its final-block check

Test-defect fix found while verifying #1335 (attempt 1's verdict, section "Outside scope: a
load-sensitive race test"). On a loaded machine, `cargo test -p capi --test resource_lifecycle
live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` failed once with
"the raced plan's final block [0.0, 0.0] against a fresh plan of the final snapshot". It passed on
rerun. Under the owner's correctness-first rule, an intermittent failure is a defect to find, not
a flake to rerun. This issue finds the cause, classifies it, and fixes it. No production code
changes.

## Problem (verified on `main` at `6fb211594`)

**The test.** `race_live_edits` (`crates/capi/tests/resource_lifecycle.rs:2653`) is #1258 gate 1,
adapted in the #1269 merge. A scoped thread renders back-to-back blocks and never waits for PCM
(`:2675`). The control thread commits live and structural edits, and it feeds the source only
inside `RaceControl::await_block` (`:2526`), through `ConstantFeed::fill` (`:2263`). After the
join, the settle (`:2766`) renders one fed block, then `settle` fed blocks (two at latency 0), then
the final fed block, and compares the final block bit for bit with a fresh plan of the final
snapshot (`:2816`). Every edit leaves the right lanes unmuted, and every gain and pan is nonzero.
So only a source that plays nothing can make both lanes of the final block `+0.0`.

**The source stays on the render clock (by design).** A block with no PCM for the next frame is an
underrun: it renders `+0.0` and still advances the read position by one quantum
(`crates/source/src/lib.rs:1135`, underrun branch at `:1159`). PCM that arrives later for a frame
the position has passed is discarded (`acquire_current_block`, `:1389`, discard at `:1427`). This
is the shipped contract that underrun emits zero plus a counter, and
`underrun_is_positive_zero_and_eof_is_not_an_underrun` (`:2456`) pins it: the late frame-0 chunk is
dropped. Each render pops at most the ring's eight transfer blocks (`source_ring_frames` 1024 at a
quantum of 128), and each `fill` refills at most the ring. So a fed block catches a lagging source
up by at most seven blocks.

**The defect.** Nothing bounds how far the render thread outruns the feed. If the control thread
is preempted after its last `fill` while the render thread keeps running, the source ends the race
more blocks behind the render clock than the settle's four fed blocks can catch up (about 28).
Every settle block then discards a ringful of stale PCM and underruns, and so does the final block.
The test's oracle assumed the source plays in the final block. On a loaded machine that is false.

**Not a product defect.** Every structural edit in the race is `SetSourceContent`. It changes the
source's declaration, so preparation gives the successor a fresh ring
(`crates/host-core/src/prepare.rs:1215`), and the feed seeks it to frame 0. No ring is carried, and
no swap loses PCM. The silence is the source keeping time with the render clock while the harness
fed it too late.

## Evidence

All runs use the debug test binary at `6fb211594`. One invocation is one `--exact` run of the test,
which is 20 race runs.

| Contention | Binary | Invocations failing |
| --- | --- | --- |
| `taskset` 1 CPU, 2 busy loops on it | `main` | 0 / 30 |
| `taskset` 2 CPUs, no busy loops | `main` | 0 / 60 |
| Whole `resource_lifecycle` binary, 2 CPUs | instrumented copy | 0 / 40 |
| `taskset` 1 CPU, 4 busy loops on it | instrumented copy | 0 / 20 |
| Split: control thread's CPU shared with 3 busy loops; each render thread moved by `taskset -p` to an idle CPU | `main` | **3 / 3** |
| Same split | instrumented copy | **2 / 2** |
| Unloaded, 40 ms sleep injected after the last edit | instrumented copy | **1 / 1** |

Every failure printed the original message, `[0.0, 0.0]`. Uniform 1-CPU contention cannot
reproduce it: a preempted control thread there also stops the render thread, so the lag stays
small. The split setup is the loaded-machine case, where the render thread keeps running while the
control thread waits.

The instrumented copy, an exported tree that was not committed, adds atomic counters to the source
consumer and producer and prints them at each settle block. In a failing split run (run 0), at the
race end the consumer's next frame was 5632 and the producer's next write frame was 1920: 29 blocks
behind. Each settle block discarded 8 stale blocks and underran once. At the final block the
consumer was at 6144 and the producer at 6016, still one block behind, so the block was `+0.0`. The
40 ms injection left the source 47 blocks behind (7040 against 1024), with the same sequence. In
passing runs, the source ended the race at most a few blocks behind.

## Decisions

- **D1. Restart the source before the final block.** A new `ConstantFeed::restart` seeks the
  source to frame 0 under the next generation and resets the feed, as a host re-anchors a source
  that fell behind. The seek must return `RESULT_OK`: the settle has rendered a block since any
  earlier seek, which emptied the one-slot command queue. The settle's first fed block runs before
  the restart, so a candidate still pending swaps in and takes any seek the feed queued for its
  fresh ring. Plan swaps from capi reserve retirement, so they are never deferred.
- **D2. Start the restart from the worst state, every run.** Between the first settle block and
  the restart, render `RACE_SOURCE_LAG_BLOCKS = 64` blocks unfed (eight rings), then `fill`. That
  leaves the source far behind the render clock with its ring full of stale PCM, whatever the race
  left. After the seek, the first block may play nothing: it drops the stale ring, and the full
  ring left no room for the restarted feed. Every later block plays fed PCM: that first block freed
  the ring, and each block is fed before it renders. `settle` is at least two, so the final block
  is at least the third after the restart. With D2, removing D1 fails every invocation at run 0.
- **D3. Nothing else changes.** Same race, same edits, same counts and asserts, same reference
  plan, same bit-exact comparison. `fed_render_c` now calls a new `render_c`, which renders without
  feeding. The race's history-free session (`race_session`: no inserts, no console slots, no
  filters, no delay) means extra blocks change nothing in the final block except what the source
  feeds it.

**Rejected: pace the render thread on PCM readiness**, as `plan_swap_race.rs` does. Each
structural edit here restarts the source in the candidate's fresh ring, so the retiring plan's ring
starves until the swap. A render thread waiting for that PCM would never reach the swap.

**Rejected: more settle blocks.** The lag the race can leave has no bound, so no fixed count is
enough.

## Authorized paths

- `crates/capi/tests/resource_lifecycle.rs`: `ConstantFeed::restart`, `render_c` and its use in
  `fed_render_c`, `RACE_SOURCE_LAG_BLOCKS`, the settle in `race_live_edits` and its doc comment.
- This spec.

## Non-goals

- Production code. The source's underrun and late-PCM rules are the shipped contract.
- C ABI telemetry for source underruns. The C ABI exposes no consumer counters, so the test
  cannot observe an underrun directly. Whether a C host needs them is a product question for a
  separate issue.
- `plan_swap_race.rs`, which paces its render thread and checks no PCM.

## Hazards

- **The restart must not hide a lost live edit.** It only seeks the source. It prepares nothing,
  so a live value lost to a retiring plan stays lost. Gate 2 checks this with a mutation.
- **The seek must be accepted.** Should it ever be refused, the run fails with the result and the
  diagnostic. It never retries.

## Objective gates

1. **Red on revert, deterministically.** With D2 kept and the `restart` call removed, the test fails
   every invocation at run 0 with "the raced plan's final block [0.0, 0.0]". This is PR evidence,
   not a committed test.
2. **The oracle keeps its teeth.** In `commit_live` (`crates/capi/src/runtime/control.rs:1087-1091`),
   resolving producers from `self.providers` instead of the newest pending epoch sends a live edit
   committed behind a pending candidate to the retiring plan. That fails the fixed test. This is PR
   evidence.
3. **No intermittent failure under contention.**
   - The fixed test, 200 or more invocations with `taskset` on 1 CPU shared with 2 busy loops,
     0 failures.
   - The fixed test, 200 or more invocations under the split contention above, 0 failures.
   - The whole `capi` test suite (its unit tests, `resource_lifecycle` and `plan_swap_race`), 200
     iterations with `taskset` on 1 CPU shared with 1 busy loop. The unit tests and
     `resource_lifecycle` have 0 failures. `plan_swap_race` fails its own non-vacuity guard
     intermittently there. That is a separate, pre-existing defect, filed as #1405, and its counts
     are recorded below.
4. **Workspace gates.** The `test-debug-a` job's `cargo test` command
   (`.github/workflows/qualification.yml:609-620`); `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`;
   `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`;
   `./target/release/audit capi` after `cargo build --locked --release -p audit -p capi`;
   `bash scripts/check-cross-targets.sh`.

*Test value.* No new test. The race test's settle is rewritten. It turns red on the same defects
as before (gate 2): a live edit that races a plan swap and reaches the retiring plan, or is lost.
It no longer turns red when the harness feeds the source too late.

## Gate results

Debug binaries built from this change. The contention lanes ran on a host already loaded by other
work.

1. Red on revert: 6 of 6 invocations failed at run 0 with "the raced plan's final block
   [0.0, 0.0]".
2. Mutation: 3 of 3 invocations failed at run 0 with "[-0.08495355, 0.033467546] against ...
   [-0.1212493, 0.039765462]". A lost live edit is nonzero, but it is wrong.
3. Contention:
   - 1 CPU with 2 busy loops: 200 of 200 invocations passed (8 lanes of 25, about 48 s each). One
     invocation with `--nocapture` counted 3146 control calls that overlapped a render call.
   - Split: IN PROGRESS.
   - Whole suite, 1 CPU with 1 busy loop: IN PROGRESS.
4. Workspace gates: `test-debug-a` exits 0 (120 test binaries, the race test `ok`); fmt clean;
   clippy clean; realtime policy ok (89 marked regions in 25 files) and its mutation tests ok;
   workspace policy ok and its mutation tests ok; `audit capi` passes (100000 calls, 0
   allocations, locks, syscalls, total violations); `check-cross-targets.sh` PASS.

## Dependencies

- None. #1053 and #1269 phase 1 are on `main` at `6fb211594`.
