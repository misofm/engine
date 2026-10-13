FAIL

# #1505 attempt 1 verdict (commit 710f77df7 on codex/d15-stream-b3)

Reviewer: fresh adversarial verifier. Reviewed `git diff 710f77df7^ 710f77df7` against the spec (with the
Attempt record), AGENTS.md (test-value rule), decision 15 (D15-2, D15-17) and the #1312/#1314 context.
Built from an export of 710f77df7 (`/tmp/claude-1002/v1505/tree`, a second export for mutations).

The test is correct and it does defend the one-block claim. A mutation I added (MX, below) shows
that the first assertion (the mixed block) can be reached, and MX is a defect that no other workspace
test catches. The attempt fails on one MAJOR: on a 4-CPU machine, the new test makes an existing test
in the same file fail much more often in release. The fix is small and inside the authorized file.

## MAJOR

**M-1. The new test makes `control_calls_racing_plan_swapping_renders_never_wedge_replacement` fail
much more often in release on a 4-CPU machine (crates/capi/tests/plan_swap_race.rs:590, :599, :1183).**
The plan_swap_race binary runs its four tests in parallel. With the binary held to CPUs 0-3 by
`taskset` (the size of a GitHub ubuntu-24.04 runner for a public repo), the existing test's vacuity
guard ("vacuous: no control call overlapped a render call") fails as follows:

| Release binary, 4 CPUs | Failures |
| --- | --- |
| Head, whole file (three sets of runs) | 9/10, 13/20, 6/20: **28/50** |
| Parent 710f77df7^, whole file | 3/10, 4/20: 7/30 |
| Head with `--skip a_two_strip` | 3/20 |
| Head with `--test-threads=1` | **0/20** |

In debug, which is CI's profile (test-debug-a), the difference is small: head 4/50, parent 1/50.
The two interleaved runs of 20 were both 0/20. The new test is never the test that fails.
The cause: the new test adds two more spinning threads, the parked render and the timed control,
next to the other race tests, and those tests' overlap guards expect cores that nothing else uses.
The flake existed before this commit, but this commit makes it more than twice as common. It also
makes the spec's own gate 2 (`cargo test --locked --release -p capi --test plan_swap_race`) red most
of the time on a 4-core machine. On this 32-core machine the file is 20/20 green in each profile.
The fix is in scope: hold one file-level `static` lock in each timing-sensitive race test in
plan_swap_race.rs (the two live races and the two plan-swap races), so that they never run at the
same time. `--test-threads=1` has the same effect and gave 0/20. After the fix, measure again on 4
CPUs, in both profiles.

## MINOR

**m-1. The Attempt record gives the wrong unique catches, and it says the first assertion cannot be
reached (spec Attempt record, lines 138-148).** The record says "M1 is the defect only the new test
catches" and calls M5 unique in release. Those claims hold only against the one-way test. I ran the
test-debug-a package set (`--no-fail-fast`, CI features) with each mutation applied:
- M1 (`begin_live_block` stores `LiveSnapshot::ALL`) is also red in
  `crates/host-core/tests/live_transaction.rs` (`one_transaction_is_applied_whole_in_the_block_its_revision_is_published_for`,
  #1312 gate 8), in `crates/graph/tests/live_snapshot_drains.rs` and in
  `crates/graph-compiler/tests/live_routes.rs`.
- M5 (watermark one block late) is also red in host-core live_transaction, in six
  `engine::realtime::tests::watermark` tests and in three host-core runtime tests.
- **MX is the unique catch that I found.** In `drain_fader_controls` (crates/builtins-compiler/src/lib.rs:1886),
  bank lane 1 (eq1) reads its cells under `LiveSnapshot::ALL` and lane 0 reads under the block's `S`.
  Result: the new test is red 6/6 in debug and 6/6 in release, and every failure comes from the
  **first** assertion ("block N mixes the two strips' states"). The one-way test is green 3/3 in each
  profile. In the whole test-debug-a set, the new test is the only failure that MX causes. (The
  abi_layout failure is a separate defect that was already present; see "Outside this diff".)
  This is the spec's gate-1 defect in its exact words: one strip's drain reads outside `S`, and the
  result is a mixed block.
- The first assertion fires under M1 too, but only after the "late" assertion has stopped the run.
  When I changed the asserts to counters, M1 runs gave mix counts of 0 to 7.
The record must give MX as the gate-1 evidence and correct the uniqueness claims.
The sentence "which no available mutation produces without a hook in the product" is wrong.

**m-2. The D3 guard does not see whether the thin sweep was covered (plan_swap_race.rs:1211, :1293).**
For Q3, I changed the asserts to counters, so that a run counts every catch and does not stop at
the first one:

| Mutation | Machine | Debug catches per run | Release catches per run |
| --- | --- | --- | --- |
| M1 | 32 CPUs | 13-25 | 67-75 |
| M1 | 4 CPUs, whole file | 5-10 | 81-126 |
| M1 | 2 CPUs, whole file | **1-8** | about 100 |
| MX | 32 CPUs | 15-25 | -- |
| MX | 4 CPUs, whole file | **6-10** | 59-67 |

Almost every debug catch comes from the `(-2, 6)` sweep. The sweep is already a fraction of the
measured render time. The defect's window is the time from the snapshot to the bank drains, which is
a few microseconds of absolute time after the call starts, and the catches come from jitter around
that point. The guard counts any revision store that lands inside a render call (more than a tenth).
A run on a busy runner can therefore pass the guard and miss the window: at 2 CPUs one debug run had
1 M1 catch with `overlapped` = 116. At the rates measured on 4 CPUs (6 or more per run) a silent miss
is unlikely, but nothing makes it fail loudly. The change I recommend is in this file and needs no
spec decision: count the stores of the thin sweep that land inside the first 1/16 of their render
call, measured with the gate's `now()` (store time minus render start), and refuse the run below a
floor. If root wants D3 to state this guard, that is a spec amendment, but it is not a blocker.

## NIT

- plan_swap_race.rs:972-976: in the `windows` doc, "edit `n`" is the 0-based `committed` count. Two
  lines above, in the `command` doc, "edit `n`" counts from 1.
- Q4 (one-way sweep): `(-16, 64)` gives `target = ((5*phase - 16) * render) / 64`, which is the old
  `floor(5*phase*render/64) - floor(render/4)`. Integer rounding can differ by up to 1 ns, which has no
  effect. Phases, span and the guard are unchanged, so the one-way test sweeps as it did before.
- The record's M5 row says the one-way test was red in debug (3/3). My M5 (the previous block's `S`,
  kept in a field of `RealtimePlanOwner`) left the one-way test green in debug 3/3 and in release 4/4.
  The row depends on how M5 is written. Say how it was written.

## Outside this diff (for root; not counted against #1505)

**Batch BLOCKER: test-debug-a is red on the branch head and on 710f77df7^.**
`tools/parameter-metadata/tests/abi_layout.rs::the_checked_in_self_test_fixture_is_current` fails
because `scripts/fixtures/abi-layout-v1-self-test.json` is stale. The generator now emits
`liveValuesSuperseded` at offset 48 and `reserved u64[3]`, but the fixture still has `reserved u64[4]`.
The cause is 731c8be30 (#1399, "Report live_values_superseded in the browser status"), which did not
regenerate the fixture. To fix it, run `cargo run -q -p parameter-metadata -- --print-abi-layout >
scripts/fixtures/abi-layout-v1-self-test.json`, inside #1399's scope.

The existing 4-CPU release flake of `control_calls_racing_plan_swapping_renders_never_wedge_replacement`
(7/30 at the parent) predates this commit. The lock in M-1 also fixes it.

## Test value

`a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it`: a strip drain that reads
its live cells outside the block's snapshot `S` (MX: one bank lane under `LiveSnapshot::ALL`) lets a
commit that races the block apply to one strip in this block and to the other strip in the next. The
new test is red on this ("mixes the two strips' states") in each profile, and no other test in the
workspace catches it. This includes the single-threaded #1312 gate 8, which cannot put a write
between the snapshot and the drains on the C ABI path.

## Gates run (export of 710f77df7, CARGO_TARGET_DIR=/tmp/claude-1002/v1505/target, export git-initialised)

- **lint job (qualification.yml), every step: all ok.** This includes fmt, workspace clippy
  `-D warnings`, rustdoc, every policy check and its mutation tests, check-rack-policy and
  test-rack-policy, test-support CI coverage and its mutations, script reachability, the builtins
  fixture manifest mutation tests, and the sub-v3 lane-guard probes.
- **test-release job, every step: all ok.** This includes the loom SPSC leg, the M1 and F1 exhaustive
  sweeps, and the wasm-gates G5/G6 digests.
- **test-debug-a job: FAIL**, only on the abi_layout stale fixture above, which predates this commit.
  A `--no-fail-fast` rerun showed no other failures (131 suites ok). plan_swap_race passed with CI
  features, 4/4.
- **plan_swap_race, head, this machine:** release 20/20 green and debug 20/20 green.
- **Mutations:**
  - M1: new test red 8/8 in each profile. The one-way test is green 4/4 in each profile.
  - M5: new test red 4/4 in each profile, every time at block 1. The one-way test is green.
  - MX: as given in m-1.
  - MY (lane 1 skips every second drain): red in `pair_levels` ("the ramps end within a block"),
    the single-threaded preflight.
- **4-CPU and 2-CPU runs** (`taskset`) and the counted-catch runs: see M-1 and m-2.
