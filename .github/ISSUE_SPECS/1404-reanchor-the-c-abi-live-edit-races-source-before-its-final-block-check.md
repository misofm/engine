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
| Split with 6 busy loops on the control thread's CPU (independent verifier) | `main` | **3 / 3** |
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

- **D1. Restart the source right after the settle's first block.** A new
  `ConstantFeed::restart` seeks the source to frame 0 under the next generation and resets the
  feed's position, as a host re-anchors a source that fell behind. The settle's first fed block runs
  before it, so a candidate still pending swaps in and takes any seek the feed queued for its fresh
  ring. Plan swaps from capi reserve retirement, so they are never deferred. The seek must return
  `RESULT_OK`. The source's command queue has one slot, every rendered block empties it, and the
  feed seeks only inside `fill`, which does not run between that block and the restart.
- **D2. Why the final block then plays fed PCM.** The ring never queues more blocks than one render
  pops (eight), and the old generation's blocks are queued ahead of the restarted feed's. So the
  first block after the restart drops every old block, and it plays the restarted feed's first
  quantum if the ring had room for it. Every later block plays fed PCM, because each block is fed
  before it renders. `settle` is at least two, so the final block is at least the third after the
  restart.
- **D3. Nothing else changes.** Same race, same edits, same counts and asserts, same reference plan,
  same bit-exact comparison. The settle renders the same number of blocks as before, so the final
  block is still about one ramp after the race, and the test still bounds how long a live ramp may
  run (gate 2).

**Rejected: render many blocks unfed before the restart.** Attempt 1 (`c977cae67`) did this
(`RACE_SOURCE_LAG_BLOCKS = 64`), so that removing the restart would fail every run with no
contention. The verifier showed the cost. It moved the final block about 67 blocks past the race,
and a live pan ramp 128 times too long then passed the race test, which fails it on `main`. The
deterministic unit test `live_fader_mute_and_pan_edits_change_the_running_plan_bit_exactly` still
catches that mutant, but the race test should not lose it. The verifier also built this
restart-only variant: it failed 0 of 5 under the 6-loop split and 0 of 8 under the 3-loop split.

**Rejected: pace the render thread on PCM readiness**, as `plan_swap_race.rs` does. Each structural
edit here restarts the source in the candidate's fresh ring, so the retiring plan's ring starves
until the swap. A render thread waiting for that PCM would never reach the swap.

**Rejected: more settle blocks.** The lag the race can leave has no bound, so no fixed count is
enough, and every extra block weakens the ramp bound D3 keeps.

## Authorized paths

- `crates/capi/tests/resource_lifecycle.rs`: `ConstantFeed::restart`, the settle in
  `race_live_edits` and its doc comment.
- This spec.

## Non-goals

- Production code. The source's underrun and late-PCM rules are the shipped contract.
- C ABI telemetry for source underruns. The C ABI exposes no consumer counter, so the test cannot
  observe an underrun directly. #1318 (*Report held source blocks apart from underruns*) already
  records that gap.
- `plan_swap_race.rs`: see #1405.

## Hazards

- **The restart must not hide a lost live edit.** It only seeks the source. It prepares nothing,
  so a live value lost to a retiring plan stays lost. Gate 2 checks this with a mutation.
- **The seek must be accepted.** Should it ever be refused, the run fails with the result and the
  diagnostic. It never retries.

## Objective gates

1. **Red on revert.** Without the restart, the settle is `main`'s, and `main` fails under the split
   contention of the evidence table. This is PR evidence, not a committed test.
2. **The oracle keeps its teeth.** Each mutant below fails the fixed test. This is PR evidence.
   - In `commit_live` (`crates/capi/src/runtime/control.rs:1087-1091`), resolve producers from
     `self.providers` instead of the newest pending epoch. A live edit committed behind a pending
     candidate then reaches the retiring plan.
   - In `classify_live_delta` (`crates/host-core/src/live_delta.rs:308-313`), make the matrix
     record's `smoothing_samples` 128 times longer. A live pan ramp then outlasts the settle.
3. **No intermittent failure under contention.**
   - The whole `resource_lifecycle` binary, which includes the race test, at least 200 invocations
     with `taskset` on 1 CPU shared with 2 busy loops: 0 failures.
   - The race test, at least 200 invocations under the 3-loop split contention: 0 failures.
   - The whole `capi` test suite (its unit tests, `resource_lifecycle` and `plan_swap_race`), 200
     iterations with `taskset` on 1 CPU shared with 1 busy loop. The unit tests and
     `resource_lifecycle` must have 0 failures. `plan_swap_race` fails its own non-vacuity guard
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
as before (gate 2): a live edit that races a plan swap and reaches the retiring plan or is lost,
and a live ramp that outlasts the settle. It no longer turns red when the harness feeds the source
too late.

## Gate results

These are debug binaries. The contention lanes ran on a host already loaded by other work.
Attempt 1 (`c977cae67`, with the unfed lag) received an adversarial PASS with two MINORs and three
NITs. This revision applies the verifier's preferred fix for MINOR 1, and the results below are
for this revision unless they say otherwise.

1. Red on revert: `main` failed 3 of 3 under the 3-loop split, and 3 of 3 under the verifier's
   6-loop split, each at its first failing run with "[0.0, 0.0]".
2. Mutants, 3 of 3 invocations each, failing at run 0:
   - the `commit_live` mutant: "[-0.08495355, 0.033467546] against ... [-0.1212493,
     0.039765462]";
   - the ramp mutant: "[-0.11219247, 0.03819576]" (one run "[-0.10992405, 0.037801996]") against
     the same reference.
3. Contention:
   - Whole `resource_lifecycle` binary, 1 CPU with 2 busy loops: 200 of 200 invocations passed
     (8 lanes of 25). On attempt 1's binary, the race test alone passed 200 of 200 the same way.
     One attempt-1 invocation with `--nocapture` counted 3146 control calls that overlapped a
     render call.
   - Race test under the 3-loop split: 200 of 200 passed (4 lanes of 50). Attempt 1 also passed
     200 of 200; `main` failed 3 of 3.
   - Whole `capi` suite, 1 CPU with 1 busy loop, 200 iterations (8 lanes of 25, attempt 1's
     `resource_lifecycle`): the unit tests and `resource_lifecycle` failed 0 times. `plan_swap_race`
     failed in 89 iterations, all at `plan_swap_race.rs:572` (#1405). The 24-swap test failed 83
     times and the 200-swap test 28. One earlier lane was voided and rerun: the disk filled up,
     and that broke its log writes, not its tests.
4. Workspace gates:
   - On this revision: `test-debug-a` exits 0 (120 test binaries, the race test `ok`); fmt clean;
     clippy clean; realtime policy ok (89 marked regions in 25 files) and its mutation tests ok;
     workspace policy ok and its mutation tests ok.
   - On attempt 1, unchanged by this test-only revision: `audit capi` passes (100000 calls; 0
     allocations, locks and syscalls; 0 total violations), and `check-cross-targets.sh` PASS.

## Dependencies

- None. #1053 and #1269 phase 1 are on `main` at `6fb211594`.
