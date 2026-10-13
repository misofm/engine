PASS

# #1505 attempt 2 verdict (commit 9d4ca9c9d on codex/d15-stream-b3)

Reviewer: fresh adversarial verifier. I reviewed `git diff 1d6e12f93 9d4ca9c9d` and `git diff 710f77df7^ 710f77df7`
against the spec and its Attempt record, the attempt-1 verdict, AGENTS.md (test-value rule) and the verifier rules.
I built from an export of 9d4ca9c9d (`/tmp/claude-1002/v1505b/tree`). I made a second export for the variants:
lock removed, MX, M1, and an instrumented build that adds timestamps to the product (evidence only, never
committed). The instrumentation patch is kept at `/tmp/claude-1002/v1505b/ev/instr3.patch`.

M-1 is fixed. The race tests are now serialized, and the 4-CPU flake does not occur on the head. MX and M1 are
red in each profile, and the record's uniqueness claims are now correct. The NITs are fixed. One MINOR stays
open: the thin-sweep guard (m-2) is not calibrated to the defect window. In debug, which is CI's profile, it
lets a run with zero stores in the defect window pass. In release, it refuses runs that did hit the window.
CI is not made flaky by it. The finding is MINOR because the attempt-1 verdict called m-2 non-blocking, and
the test still catches MX and M1 in every normal run. But the code comment and the record make a claim that
my measurements show to be false, and that must be corrected.

## MINOR

**m-2'. The thin-sweep guard counts a window that scales with the render call. The defect window scales with
the commit. (crates/capi/tests/plan_swap_race.rs:1078-1084, :1358-1367; spec Attempt 2, "m-2, the thin-sweep guard")**

The measurement has no ordering error that inflates the count. `stored` is taken when the submit returns,
which is after the revision store. `render_began` is loaded after that. A render call that started after the
store fails `stored >= call`, and a `render_began` that is not yet visible gives the previous call, so the
count goes down. Each error makes the count smaller, never larger.

The error is the scale of the window. I added timestamps to an exported copy of the product: the gate
`publish`, the snapshot `fetch_or`, the first and last cell write of each commit, and the start of bank
lane 1's fader drain. With these I measured where each thin-sweep store really landed. The defect window
for MX and M1 is "a cell written before the block's drain, with the publish after the block's snapshot".
All runs were whole runs of the two-strip test under `taskset -c 0-3`:

| Profile, load | Guard's `early` (floor 40) | Stores in the MX defect window | Result |
| --- | --- | --- | --- |
| release, outside load avg about 8 | 82-119 | 65-107 | pass (correct) |
| debug, same | 148-166 | 8-21 | pass (correct) |
| release, 4 spinners pinned to CPUs 0-3 | 3-15 | 6-18 | **refused, although the window was hit** |
| debug, 4 spinners | 46-63 | **0-4** (0 in 3 of 7 runs) | **passes a run that never hit the window** |

The reason: in debug, the first 1/16 of a render call is about 137 us (call 2.19 ms). The defect window is
about 15 us: the commit's cell writes come about 10.5 us before its publish, and the snapshot comes about
4 us before lane 1's drain. In release, the 1/16 is about 0.74 us. The window there is about 0.9 us (cell
writes to publish 0.65 us, snapshot to drain 0.25 us). The window follows the control commit's own cost,
not the render call's length.

Direct consequence: under 4 spinners, M1 in debug passed 1 run of 10 with the guard silent. MX in debug gave
9 catches and 1 refusal. Under 2 spinners, and with no spinners, the head was green in each profile (below).
So the guard does not make CI flaky. In debug it is nearly always satisfied, and it does not make a weak
debug run fail loudly, which is what m-2 asked for. In release, the implementer's 3 refusals under outside
load (load avg 13-22) are the same false refusals I measured with spinners: the window was still hit.

The code comment at :1358-1362 says the first 1/16 is "between the block's snapshot and the strips' drains".
The record says "a run at the floor is a run that is hardly racing". For debug, both statements are false.

Concrete change within this file (I measured it with the instrumented build): count a thin-sweep store as
early when its submit returned within `commit_nanos / 16` of the call's start, not within `render_nanos / 16`.
`commit_nanos` is the control commit's own measured duration, which the aim already uses. With that window:

| Profile, load | Count | Of which in the MX window |
| --- | --- | --- |
| release, quiet | 97-105 | 65-68 |
| release, 4 spinners | 20-25 | 16 |
| debug, quiet | 21-33 | 8-16 |
| debug, 4 spinners | 2-6 | 0-4 |

The count now follows the real window in both profiles. A floor of about 10 refuses the loaded debug runs
and passes the loaded release runs that still hit the window. The floor needs its own short calibration
before it is committed. A further option is to give the thin sweep's targets in commit-time units, so that
more debug stores land in the window. Now about one phase in 16 is aimed there, and the hits come from
jitter. If root does not want this change in #1505, correct the comment and the record so that they say what
the guard measures: that the sweep's aim reached the call's start, not that the run covered the defect window.

## NIT

- Record, M-1 table: "lock only ... 0/30" in debug, and "attempt 1 head, no lock ... 0/30" in debug. My
  lock-removed binary failed 1/30 in debug ("vacuous: no control call overlapped a render call"). This does
  not change the conclusion.

## Item checks

1. **M-1, the lock: fixed.** `RACES`/`one_race_at_a_time()` (plan_swap_race.rs:30-39) is taken first in
   `race_plan_swaps` (:457, which covers both plan-swap tests) and in both live races (:1112, :1243). These
   are the four tests in the binary. A poisoned lock is still taken. 4-CPU runs, interleaved in sets of 10,
   whole binary, outside load avg 0.6-6.2:
   - head release **50/50 green**; lock removed **14/50 green (36 failures)**. Of these, 34 were "vacuous: no
     control call overlapped a render call" and 2 were the new guard ("30 of 320", "32 of 320").
   - head debug **30/30 green**; lock removed 29/30 (1 failure, the overlap guard).
   - With 2 spinners pinned to CPUs 0-1: head release 20/20 and debug 10/10 green. With 4 spinners: release
     0/20 (each one the new guard, 5 to 27 of 320) and debug 10/10 green. See m-2'.
2. **m-2, the guard:** sound in its ordering, but mis-scaled. See m-2'.
3. **m-1, the record: fixed.** Re-run, 4 CPUs:
   - MX (`drain_fader_controls`, bank lane 1 drains under `LiveSnapshot::ALL`): the new test is red 10/10 in
     release and 10/10 in debug. Every failure is the first assertion ("block N mixes the two strips'
     states"). The one-way test is green 5/5 in each profile. In debug under 4 spinners: 9 catches and 1
     refusal.
   - M1 (`GraphExecutor::begin_live_block` keeps `LiveSnapshot::ALL`): the new test is red 10/10 in each
     profile ("shows a new state, but the watermark ... did not advance"). The one-way test is green 5/5 in
     each profile.
   - The corrected text (M1 and M5 unique only against the one-way test, and MX as the gate-1 evidence on the
     first assertion) agrees with these runs and with the attempt-1 workspace runs. I did not re-run M5. The
     record no longer claims that M5 is unique.
4. **NITs: fixed.** The `windows` doc counts from 0, the same as `committed`, and the `command` doc counts from
   1. This agrees with `(self.command)(self.committed + 1, ...)`. The M5 row says how M5 was written.

## Test value

`a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it`: if a strip drain reads its live cells
outside the block's snapshot `S` (MX: one bank lane under `LiveSnapshot::ALL`), a transaction that races the
block is applied to one strip in this block and to the other strip in the next block. The test is red on this
in each profile, 10/10 ("mixes the two strips' states"). No other workspace test catches it.

The change to the one-way test (the lock only) adds no assertion. The lock defends both plan-swap tests and
both live tests against starving each other's overlap on a 4-CPU runner: 36/50 release failures without the
lock, 0/50 with it.

## Gates run (export of 9d4ca9c9d, CARGO_TARGET_DIR=/tmp/claude-1002/v1505b/target, export git-initialized)

- **lint job, all 32 steps ok.** This includes fmt, workspace clippy `--all-features -D warnings`, rustdoc,
  workspace policy and its mutation tests, the rack/builtins/graph dependency-boundary policy (check-rack-policy)
  and its mutations, the test-support CI coverage policy and its mutations, script reachability, the standing
  builtins fixture manifest mutation tests, and the sub-v3 lane-guard probes.
- **test-release job, all 9 steps ok.** This includes the engine SPSC loom leg, the math M1 and F1 exhaustive
  sweeps, and the wasm-gates G5/G6 digests.
- **test-debug-a job, all 3 steps ok.** This includes plan_swap_race with CI features, and the workspace
  doctests. The abi_layout stale-fixture failure that attempt 1 found is no longer present.
- **test-debug-b job, all 3 steps ok.**
- Spec gate 2 release run (`plan_swap_race` in release): see item 1. 50/50 on 4 CPUs and 20/20 under 2 spinners.
- Machine load during the timing runs: load avg 0.6-6.2 for the M-1 sets, and about 5-17 for the mutation and
  instrumented sets (other agents' jobs, plus my spinners where I state them).
