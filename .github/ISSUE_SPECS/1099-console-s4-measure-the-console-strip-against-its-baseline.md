# Measure the console strip against its baseline

Slice S4 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H2, H5 and M4 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Decision 12 accepted known costs on condition that they are measured:
- a padded one- or two-member remainder costs more than per-node rendering at W=8 (H5);
- a bypassed lane runs the wet path;
- a bank's silent fast path needs every lane silent (M4);
- `post_insert` groups split by level (H2).

S0 recorded the baseline on B0's rows. S4 records the same rows on the console shape and reports
the difference.

## Smallest closable slice

On the batch C4 commit where S2 has landed, with C3 (through S1d) already pushed:

- Native: `bash scripts/operator/preflight-console-benchmark.sh --step console-strip-after`, then
  `bash scripts/operator/run-console-benchmark.sh --step console-strip-after`.
- V8: `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
  WORKDIR`, then `run WORKDIR --step console-strip-after`, on the module built from the same commit.

Each run is one invocation, with one warmup and two measured rounds. S4 changes no code. Every row
it needs is already on the new shape, and each has an owner:

| Row | S0 (before) | S4 (after) | Moved by |
|---|---|---|---|
| Strip, N in {9, 10, 13, 16, 64}, and sparse activity (native) | `simd1`/`simd2` racks of the intended fixture | `pre_insert` and `post_insert` | S1a: fixture migration and the `tools/console-workload` builders |
| App shape (native) | `dynamic`: EQ -> compressor, 2-mod-3 bypass | `pre_insert`: EQ -> compressor, same bypass | S1a: the builder, under the `dynamic` clause of its migration rule |
| App shape and N = 64 strip (V8), and the existing mono automation row | old-shape documents, rack bytes `0`/`1`/`2` | console documents, rack bytes `3`/`1` | S1a: documents and the harness's fixture lookup. S1c: the rack codes |

The validator reads the layout-neutral `strip_layout` B0 introduced, so no validator change is
needed. If a row does not run, or needs a validator change, stop and report it as a defect of the
slice named above (B0, S1a or S1c). S4 does not repair it.

Report a before/after table: p50 µs per block for both rounds of S0 and S4, per row. Name:

- the remainder rows' motion against H5's arithmetic (about 9.3 µs per padded bank of the three
  slots against about 4.2 µs per per-node track);
- the app-shape motion (bypassed lanes now run the wet path in shared banks);
- the sparse-activity row against the all-active and idle rows, stating whether a per-lane silence
  skip is warranted. If it is, draft that successor issue. Do not implement it.

State `measurement_control` as recorded and name the operator's lock.

Authorized paths: `artifacts/steps/console-strip-after/**`, the V8 record directory, and this
spec's evidence section.

## Dependencies

This is batch C4, after S2.

- *Bind every console slot banked for every track count* (S2).
- *Ship the session console and inserts in the SDK* (S1d).
- *Record the console-strip baseline benchmark* (S0).

## Objective gates

1. Both preflights pass. Both runners accept and promote their records, and the validators pass
   unchanged.
2. There is one invocation per runner: no tuning, no retry, and no projected savings.
3. The before/after table covers every B0 row, with both rounds of both runs.
4. The report names each accepted cost from decision 12 with its measured motion. Where the
   evidence supports one, it gives a recommendation: a per-lane silence skip, an ALAP `post_insert`
   alignment, or #892. It changes no code.
5. Each row's `output_sha256` equals S0's. The design is class A end to end (P1, P2a-P2e, S1a and
   S2 each keep every bit), so a difference is a class-A defect. Report the row and the difference
   instead of explaining it away. This covers the native records' `output_sha256` and the V8
   records' output digest fields (name them from the V8 record schema). The one allowance: a
   difference traced to an unrelated merged change between C1 and C4 is re-baselined by rerunning
   S0's command on that change's commit, and the evidence names the commit.

## S4 evidence

Terra, one invocation per runner, on 2026-09-30. The timed commit is batch C4's head on
`codex/batch-console-4`, `f7ba70a8bccf5111f398a80030f7762aad402bb0` (S2 merged at `1434cdaf`, C3
through S1d pushed), on a clean detached worktree. The baseline is S0's records in
`artifacts/steps/console-strip-base/`, recorded on B0's commit `58f8c77b08e1ef344a3ad5d18c013fb64d555b8a`.
S4 changed no code. The records name `f7ba70a8` and nothing else.

### Commands

These are S0's commands, with only the step name and the commit changed. Each script's argument
form was checked against its usage line at `f7ba70a8` first, and all of them matched.

```bash
LOCK=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock
HEAD_C4=f7ba70a8
BATCH=/home/bl/misofm/worktrees/engine-console
WT=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/s4-wt
git -C /home/bl/misofm/engine worktree add --detach "$WT" "$HEAD_C4" && cd "$WT"
W=$(mktemp -d -p /tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad)   # .../tmp.XQbMdl5ZVr
bash scripts/run-web-mixing-automation-benchmark.sh prepare "$W"
bash scripts/run-web-mixing-automation-benchmark.sh preflight "$W"
bash scripts/operator/preflight-console-benchmark.sh --step console-strip-after
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/operator/run-console-benchmark.sh --step console-strip-after
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/run-web-mixing-automation-benchmark.sh run "$W" --step console-strip-after
cp -r artifacts/steps/console-strip-after "$BATCH/artifacts/steps/"
```

Each timed command ran inside a small wrapper script. The wrapper did three things:
- It waited for the host to go quiet, on S0's condition fixed in advance: a 1-minute loadavg below
  0.45, capped at 900 s.
- It recorded `/proc/loadavg` and a `ps` snapshot immediately before and after the timed command.
- It ran the timed command exactly once.

Nothing was launched or timed during either wait, so neither wait is a retry.
- Before the native run, the loadavg reached 0.41 after 210 s. Most of the load it waited out was
  left over from the untimed preflight builds.
- Before the V8 run, the loadavg reached 0.43 after 76 s.

The operator's lock is `.../scratchpad/timing.lock`, taken with `flock -w 7200`. `fuser` showed no
holder before either run, and both runs started when their wrapper's wait ended.

### Untimed preflights

- V8 `prepare`: exit 0.
  - `host_web.wasm` is `ac3a9353e8107fa409398c16fb27de7cc9846b3c5b0cecf8f864535ad302efc9`,
    built at `f7ba70a8`. It is not the release pin, `6c952a2c...`.
  - `controls.json` is `a32cb879b83fba4cc99a0de694237205dcc2f8f6f96e5c89392c90223d8097c6`. It
    carries rack byte `3` (console) for all three slots (`eq`, `comp`, `limiter`), which is S1c's
    code.
- V8 `preflight`: exit 0.
  - All seven arm digests equal S0's preflight.
  - The two documents' preflight output digests are `d913ad96...` for the console and
    `3dd8b2ff...` for the app shape, and they also equal S0's. They are rendered-output digests over
    the preflight blocks, not file digests: the app fixture itself moved to `console.pre_insert`
    under S1a.
- Native preflight: PASS, with `workload_launches` 0, `records_required` 60 and candidate
  `f7ba70a8`.
  - It built the binary `c2b4c3c6...` under the default release profile.
  - The log shows two `Aborted (core dumped)` lines. They are the preflight's own refusal probes
    (an empty round marker and an extra argument), and S0's log has the same two.
  - `record_validator_sha256` is unchanged from S0 (`bfadf1b4...`).
  - `validator_library_sha256` moved (`55652efe...` to `d56cd318...`). The only change to
    `scripts/console-benchmark-record-lib.jq` since `58f8c77b` is one word in a comment (#1095,
    `b07c32f0`). No validator `.jq` file changed.

### Timed runs, load and control

All times are UTC, on an AMD EPYC 7313P (32 CPUs) with the `schedutil` governor. Both runners
pinned CPU 31.

| Run | Before | Admissibility read | After | `measurement_control` |
|---|---|---|---|---|
| native (16:01:05 to 16:02:29) | `0.41 1.78 3.16` | `1.02 1.79 3.07` | `1.09 1.77 3.04` | `uncontrolled` |
| V8 (16:04:04 to 16:04:13) | `0.43 1.38 2.78` | `0.43 1.38 2.78` | `0.51 1.36 2.76` | `controlled` |

- Native: `uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling`,
  against a ceiling of 0.50. No other precondition was waived.
  - The SMT sibling, cpu15, was 0.00% busy.
  - The cooldown was 60 s, and the runner waited all of it, because it rebuilt `bench` under its
    frozen release profile. The rebuild produced binary `3e57989c...`, not the preflight's
    `c2b4c3c6...`. This matches S0, where the same happened.
  - The loadavg rose from 0.41 to 1.02 over that rebuild and cooldown, before the admissibility
    read.
  - No `perf` counter is usable, so the records carry no cycle columns.
- V8: `controlled; loadavg 0.43 1.38 2.78 2/1798 4184419; ceiling 0.50; affinity cpu 31`.
  - The allow-uncontrolled variable was set, as in S0, but the admissibility read was under the
    ceiling, so nothing was waived.
  - S0's V8 run was `uncontrolled` at about 1.06, so the two V8 runs ran under different load.
  - Per round, loadavg at start and end: round 1 went from `0.47` to `0.51`, and round 2 from
    `0.51` to `0.51`.
- Background load, from `ps` just before and after each run. `ps` reports each process's lifetime
  average `%CPU`.
  - `blast-cetus-pipeline`, PID 3975593, ran at about 0.55 to 0.6 of a core. It was started at
    about 15:25, and the brief measured it at about 0.8 of a core at launch.
  - `clickhouse-server`, PID 929971, ran at about 0.28 of a core.
  - Smaller: `herdr` at about 0.09 and `tailscaled` at about 0.04, plus idle agent CLIs at a few
    percent.
  - No `cargo` or `rustc` process was running when either timed run started.

### Identities and gates

- Native disposition: `PASS complete`, `runner_invocations` 1, `workload_process_launches` 3
  (1 warmup and 2 measured), and 60 records.
  - `raw_sha256` and `accepted_sha256` are both
    `fa3466281f6ba6c56e7c65ad84d1b9ef29c2819cc93bfdc922efb570dabfb1bc`.
  - `binary_sha256` is `3e57989c6f2a25f3129366db9e2255a7329a19c368169eb616cb69a4ab724132`.
- V8: `web-mixing-automation.jsonl` is
  `76609d74464111d0b3721045cf5215d05de232e3cac68744f1dfea81ace4e532`, with two rounds.
  - `prepared_commit` equals `candidate_commit`, which is `f7ba70a8`, and `module_matches_pin` is
    false.
  - The runtime is Node v22.23.2 with V8 12.4.254.21-node.56, run with `--no-liftoff`. Each round
    has 1000 observations and 64 preroll blocks.
- The stderr logs match `.gitignore`'s `*.log` and are not committed.
  - The native log is byte-identical to S0's (88 bytes, `042f18e1...`).
  - The V8 log is empty.

Gates:
1. PASS. Both preflights passed, and both runners accepted and promoted their records. Re-running
   `console-benchmark-validator.jq` and `web-mixing-automation-validator.jq` on the copied records
   in the batch worktree passes both, and S0's records still pass the C4 validator. Every row ran,
   and none needed a validator change. The app-shape row's `strip_layout` is now
   `pre_insert:eq+compressor`, with the same `index_mod_3_is_2` pattern (21 tracks) that S0
   observed.
2. PASS. There was one invocation per runner, with no tuning and no retry. No post-workload tooling
   failed, and this report quotes no projected saving.
3. PASS. The tables below cover all 22 native session rows and all 21 native arm rows (60 records),
   plus five V8 rows, with both rounds of both runs.
4. See the analysis. It names each accepted cost from decision 12 with its measured motion, and it
   recommends one successor: a per-lane silence skip, drafted below. It makes no ALAP or #892
   recommendation, because the evidence supports neither.
5. PASS, and no re-baseline was needed.
   - Native: every `*_output_sha256` field in all 60 records equals S0's. That is 86 fields across
     14 field names, including each row's `output_sha256`. The mixing-automation record's
     `preflight_output_sha256` object is also equal.
   - V8: `quiet_output_sha256`, `restated_output_sha256`, `automated_output_sha256`, both
     `documents[].output_sha256` and all seven `preflight_output_sha256.*` keys equal S0's in both
     rounds.

### Tables

Units are µs per block at 48 kHz and a 128-frame quantum. Percentiles are nearest-rank over 1000
observations per round. r1 and r2 are the two measured rounds, and "Δ mean" is the mean of S4's two
rounds minus the mean of S0's. Every digest is identical across the rounds of each run. The hoist
arms' records carry no p95 for the quiet arm.

#### Native, Simd8, p50: the console-strip rows, with idle

| Row | Tracks | S0 r1 | S0 r2 | S4 r1 | S4 r2 | Δ mean | Δ % |
|---|---|---|---|---|---|---|---|
| strip N=9 (`nine_track_ragged_strip`) | 9 | 19.948 | 20.189 | 24.276 | 24.787 | +4.463 | +22.2% |
| strip N=10 (`ten_track_ragged_strip`) | 10 | 24.708 | 25.068 | 24.136 | 24.346 | -0.647 | -2.6% |
| strip N=13 (`thirteen_track_ragged_strip`) | 13 | 39.385 | 39.745 | 25.108 | 25.218 | -14.402 | -36.4% |
| strip N=16 (`sixteen_track_strip`) | 16 | 23.875 | 23.846 | 24.106 | 24.206 | +0.295 | +1.2% |
| strip N=64 (`sixty_four_track_console`) | 64 | 99.480 | 98.999 | 100.592 | 100.421 | +1.267 | +1.3% |
| app shape (`sixty_four_track_app_shape`) | 64 | 75.254 | 75.384 | 68.571 | 68.561 | -6.753 | -9.0% |
| sparse activity (`sixty_four_track_console_sparse`) | 64 | 98.547 | 98.918 | 99.890 | 100.331 | +1.378 | +1.4% |
| idle (`sixty_four_track_idle`) | 64 | 30.678 | 30.888 | 31.230 | 31.080 | +0.372 | +1.2% |

#### Native, Simd8, p50: the other session rows

| Row | Tracks | S0 r1 | S0 r2 | S4 r1 | S4 r2 | Δ mean | Δ % |
|---|---|---|---|---|---|---|---|
| `nine_track_baseline` | 9 | 7.835 | 7.725 | 8.005 | 7.795 | +0.120 | +1.5% |
| `one_twenty_eight_track_stretch` | 128 | 199.008 | 198.258 | 200.592 | 201.063 | +2.194 | +1.1% |
| `sixty_four_track_eq_only` | 64 | 35.428 | 34.987 | 36.179 | 36.450 | +1.107 | +3.1% |
| `sixty_four_track_compressor_only` | 64 | 48.472 | 48.572 | 49.495 | 49.554 | +1.002 | +2.1% |
| `sixty_four_track_builtins_only` | 64 | 24.828 | 24.897 | 25.498 | 25.258 | +0.516 | +2.1% |
| `sixty_four_track_dispatch_only` | 64 | 11.371 | 11.432 | 10.691 | 11.201 | -0.455 | -4.0% |
| `sixty_four_track_console_legacy` | 64 | 59.313 | 59.143 | 59.383 | 59.744 | +0.336 | +0.6% |
| `sixty_four_track_eq_comp_simd1` | 64 | 59.844 | 58.642 | 59.804 | 60.225 | +0.771 | +1.3% |
| `sixty_four_track_gain_pan_only` | 64 | 10.580 | 10.901 | 10.790 | 10.931 | +0.120 | +1.1% |
| `sixty_four_track_console_mono` | 64 | 63.271 | 63.601 | 64.583 | 64.904 | +1.307 | +2.1% |
| `sixty_four_track_console_mono_dual` | 64 | 99.900 | 99.870 | 100.692 | 100.822 | +0.872 | +0.9% |
| `sixty_four_track_console_half_mono` | 64 | 85.753 | 85.703 | 86.244 | 86.785 | +0.786 | +0.9% |
| `sixty_four_track_gain_pan_ring` | 64 | 8.837 | 9.167 | 8.927 | 9.017 | -0.030 | -0.3% |
| `sixty_four_track_console_metered` | 64 | 103.557 | 103.637 | 105.330 | 105.561 | +1.849 | +1.8% |

#### Native, Simd8, p50: the arm records

| Row | Tracks | S0 r1 | S0 r2 | S4 r1 | S4 r2 | Δ mean | Δ % |
|---|---|---|---|---|---|---|---|
| `console_hoist` `nine_track_ragged_strip`, quiet | 9 | 6.492 | 10.951 | 6.533 | 6.603 | -2.154 | -24.7% |
| `console_hoist` `nine_track_ragged_strip`, restated | 9 | 7.885 | 12.333 | 7.955 | 8.016 | -2.123 | -21.0% |
| `console_hoist` `nine_track_ragged_strip`, moving | 9 | 18.104 | 18.124 | 18.145 | 18.164 | +0.041 | +0.2% |
| `console_hoist` `sixty_four_track_console`, quiet | 64 | 15.199 | 19.277 | 14.859 | 15.449 | -2.084 | -12.1% |
| `console_hoist` `sixty_four_track_console`, restated | 64 | 21.030 | 25.449 | 21.150 | 21.511 | -1.909 | -8.2% |
| `console_hoist` `sixty_four_track_console`, moving | 64 | 68.331 | 68.330 | 68.500 | 68.420 | +0.130 | +0.2% |
| `console_meters` `sixty_four_track_console`, meters_off | 64 | 98.869 | 98.838 | 100.331 | 100.041 | +1.333 | +1.3% |
| `console_meters` `sixty_four_track_console`, meters_on | 64 | 109.418 | 108.867 | 110.540 | 110.901 | +1.578 | +1.4% |
| `console_observation` `sixty_four_track_console`, absent | 64 | 106.823 | 106.653 | 107.685 | 107.645 | +0.927 | +0.9% |
| `console_observation` `sixty_four_track_console`, unarmed | 64 | 106.462 | 107.284 | 107.645 | 108.046 | +0.972 | +0.9% |
| `console_observation` `sixty_four_track_console`, armed | 64 | 107.976 | 108.456 | 108.857 | 109.258 | +0.841 | +0.8% |
| `console_placement` `sixty_four_track_placement`, split_chains | 64 | 60.415 | 61.217 | 61.647 | 61.146 | +0.581 | +1.0% |
| `console_placement` `sixty_four_track_placement`, merged_chain | 64 | 60.455 | 61.277 | 60.916 | 60.796 | -0.010 | -0.0% |
| `console_automation` `sixty_four_track_compressor_automation`, quiet | 64 | 51.308 | 52.701 | 52.079 | 51.929 | -0.000 | -0.0% |
| `console_automation` `sixty_four_track_compressor_automation`, restated | 64 | 52.109 | 52.950 | 51.798 | 51.628 | -0.816 | -1.6% |
| `console_automation` `sixty_four_track_compressor_automation`, automated | 64 | 52.721 | 53.943 | 52.269 | 52.440 | -0.977 | -1.8% |
| `console_mono` `sixty_four_track_mono_pair`, collapse_eligible | 64 | 64.442 | 65.114 | 65.354 | 65.685 | +0.742 | +1.1% |
| `console_mono` `sixty_four_track_mono_pair`, collapse_forced_off | 64 | 100.651 | 101.663 | 101.413 | 101.523 | +0.311 | +0.3% |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, quiet | 64 | 71.386 | 71.586 | 73.129 | 72.338 | +1.248 | +1.7% |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, restated | 64 | 72.708 | 73.279 | 73.630 | 73.219 | +0.431 | +0.6% |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, automated | 64 | 75.684 | 76.185 | 76.004 | 75.414 | -0.225 | -0.3% |

#### Native, Simd8, p95

| Row | S0 r1 | S0 r2 | S4 r1 | S4 r2 |
|---|---|---|---|---|
| strip N=9 (`nine_track_ragged_strip`) | 20.449 | 20.549 | 24.837 | 25.328 |
| strip N=10 (`ten_track_ragged_strip`) | 25.448 | 25.739 | 25.018 | 24.887 |
| strip N=13 (`thirteen_track_ragged_strip`) | 40.867 | 40.988 | 26.049 | 26.220 |
| strip N=16 (`sixteen_track_strip`) | 24.466 | 24.777 | 24.537 | 24.597 |
| strip N=64 (`sixty_four_track_console`) | 104.960 | 104.569 | 107.655 | 105.922 |
| app shape (`sixty_four_track_app_shape`) | 80.614 | 80.734 | 73.660 | 74.201 |
| sparse activity (`sixty_four_track_console_sparse`) | 103.818 | 104.780 | 105.230 | 105.521 |
| idle (`sixty_four_track_idle`) | 31.129 | 31.701 | 34.846 | 32.082 |
| `nine_track_baseline` | 7.925 | 7.805 | 8.085 | 7.875 |
| `one_twenty_eight_track_stretch` | 206.964 | 205.191 | 209.249 | 208.077 |
| `sixty_four_track_eq_only` | 36.389 | 35.989 | 38.704 | 38.573 |
| `sixty_four_track_compressor_only` | 52.981 | 52.500 | 53.112 | 53.522 |
| `sixty_four_track_builtins_only` | 25.469 | 25.158 | 26.350 | 26.210 |
| `sixty_four_track_dispatch_only` | 11.522 | 11.613 | 10.841 | 11.532 |
| `sixty_four_track_console_legacy` | 64.233 | 63.912 | 64.362 | 64.153 |
| `sixty_four_track_eq_comp_simd1` | 64.603 | 63.171 | 64.563 | 65.265 |
| `sixty_four_track_gain_pan_only` | 10.701 | 11.061 | 10.981 | 11.191 |
| `sixty_four_track_console_mono` | 67.929 | 67.759 | 68.781 | 70.054 |
| `sixty_four_track_console_mono_dual` | 105.531 | 105.691 | 106.383 | 106.082 |
| `sixty_four_track_console_half_mono` | 91.284 | 90.944 | 91.675 | 92.276 |
| `sixty_four_track_gain_pan_ring` | 8.937 | 9.248 | 9.067 | 9.127 |
| `sixty_four_track_console_metered` | 109.218 | 109.238 | 111.162 | 113.206 |
| `console_hoist` `nine_track_ragged_strip`, quiet | n/a | n/a | n/a | n/a |
| `console_hoist` `nine_track_ragged_strip`, restated | 7.975 | 14.026 | 8.055 | 8.126 |
| `console_hoist` `nine_track_ragged_strip`, moving | 18.235 | 18.234 | 18.264 | 18.285 |
| `console_hoist` `sixty_four_track_console`, quiet | n/a | n/a | n/a | n/a |
| `console_hoist` `sixty_four_track_console`, restated | 21.320 | 27.111 | 21.772 | 22.263 |
| `console_hoist` `sixty_four_track_console`, moving | 73.149 | 73.099 | 73.379 | 73.270 |
| `console_meters` `sixty_four_track_console`, meters_off | 103.968 | 104.138 | 105.711 | 105.481 |
| `console_meters` `sixty_four_track_console`, meters_on | 115.210 | 114.979 | 116.582 | 116.732 |
| `console_observation` `sixty_four_track_console`, absent | 112.985 | 112.534 | 113.606 | 114.669 |
| `console_observation` `sixty_four_track_console`, unarmed | 112.223 | 112.875 | 113.285 | 113.947 |
| `console_observation` `sixty_four_track_console`, armed | 114.278 | 114.248 | 114.839 | 115.590 |
| `console_placement` `sixty_four_track_placement`, split_chains | 65.385 | 68.310 | 66.497 | 66.547 |
| `console_placement` `sixty_four_track_placement`, merged_chain | 65.435 | 68.030 | 65.916 | 65.605 |
| `console_automation` `sixty_four_track_compressor_automation`, quiet | 56.217 | 58.021 | 56.959 | 57.048 |
| `console_automation` `sixty_four_track_compressor_automation`, restated | 56.127 | 57.920 | 56.798 | 56.718 |
| `console_automation` `sixty_four_track_compressor_automation`, automated | 57.469 | 58.712 | 57.038 | 57.360 |
| `console_mono` `sixty_four_track_mono_pair`, collapse_eligible | 69.352 | 70.004 | 70.354 | 70.805 |
| `console_mono` `sixty_four_track_mono_pair`, collapse_forced_off | 106.613 | 107.124 | 106.813 | 107.194 |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, quiet | 76.305 | 75.524 | 78.369 | 77.518 |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, restated | 77.988 | 76.867 | 78.780 | 78.429 |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, automated | 81.204 | 81.195 | 81.516 | 80.133 |

#### Native: `output_sha256` equality (gate 5)

| Row | `output_sha256` (S0 r1, S0 r2, S4 r1, S4 r2) | All four equal |
|---|---|---|
| strip N=9 (`nine_track_ragged_strip`) | `cd98694d647e73d1dc2ffbe3891d16a29e11031b083e48cd044f622ca77f6814` | yes |
| strip N=10 (`ten_track_ragged_strip`) | `45eefb82824053281761cd5799edaeddfa542097bb1362457c2623fbe04dac2b` | yes |
| strip N=13 (`thirteen_track_ragged_strip`) | `64015ad3c502898e8b53486637ea1233f642c36af6a97a51fd835364abe75a23` | yes |
| strip N=16 (`sixteen_track_strip`) | `8957e9904258732df6575cd205b51aab282b80c87808aa8caad424d6db5cba7b` | yes |
| strip N=64 (`sixty_four_track_console`) | `6bae1dd0eacc770044e64de2699e35eac2fb8aa192f98e8180bc036fe7edf243` | yes |
| app shape (`sixty_four_track_app_shape`) | `3ce17e3e31bbbe23f7a446ef04b54adc4c605b25dba0fdc8a58d7221a84055b8` | yes |
| sparse activity (`sixty_four_track_console_sparse`) | `8076e45021ca08a99b28e59d97f4d04cd13c7a1f7bee6daf021ce9ce6485cccb` | yes |
| idle (`sixty_four_track_idle`) | `7b331c02e313c7599d5a90212e17e6d3cb729bd2e1c9b873c302a63c95a2f9bf` | yes |
| `nine_track_baseline` | `d5df5ebe109d28b4ed96eb2f703a2301448e9df6a9c5f10f7d0af58732a58533` | yes |
| `one_twenty_eight_track_stretch` | `1fd1755ec5adb86b08240580abff443128ef2bdd187501ddf8b83fefd87176ab` | yes |
| `sixty_four_track_eq_only` | `83a4b205c38320f4c48eb038a811ccb826230a47f9687d05674c7ae3dbac5a3d` | yes |
| `sixty_four_track_compressor_only` | `2d8712e612474b55ae70ad18a0242f41bf9b8b20fbc70e62f937c64c3d0b41ad` | yes |
| `sixty_four_track_builtins_only` | `5bf3c3772d4cb400e3cbe703cd750363a1b8ba7e612b2e3f15292c1cda5e6d26` | yes |
| `sixty_four_track_dispatch_only` | `2b015145fd33d19a8a84bc6bbe0caaa65a50bd129a0f2512fe44a9cdb21b1ad7` | yes |
| `sixty_four_track_console_legacy` | `f41ad6354fc25d4e4eed04d1f8032fe8068c267cbe9dc7afcc0ee595fffd9cfd` | yes |
| `sixty_four_track_eq_comp_simd1` | `f41ad6354fc25d4e4eed04d1f8032fe8068c267cbe9dc7afcc0ee595fffd9cfd` | yes |
| `sixty_four_track_gain_pan_only` | `2410d7237d2173526f967b897835b52eb515196c655920b09ca93acb111bb7b1` | yes |
| `sixty_four_track_console_mono` | `fa1eb6bff3b0780c9b26cbbb75c1f9071f04769eb2edaf0029970ffb3f6ca049` | yes |
| `sixty_four_track_console_mono_dual` | `fa1eb6bff3b0780c9b26cbbb75c1f9071f04769eb2edaf0029970ffb3f6ca049` | yes |
| `sixty_four_track_console_half_mono` | `50f6e28443ac921fb3ae0508ea9ec218f81bbadd9158cd0a2d0c31d350032732` | yes |
| `sixty_four_track_gain_pan_ring` | `2410d7237d2173526f967b897835b52eb515196c655920b09ca93acb111bb7b1` | yes |
| `sixty_four_track_console_metered` | `6bae1dd0eacc770044e64de2699e35eac2fb8aa192f98e8180bc036fe7edf243` | yes |
| `console_hoist` `nine_track_ragged_strip`, quiet | `bac4f68c477c1699fed23306df14a6a380963d30208a526d81908de6e9cc9d93` | yes |
| `console_hoist` `nine_track_ragged_strip`, restated | `bac4f68c477c1699fed23306df14a6a380963d30208a526d81908de6e9cc9d93` | yes |
| `console_hoist` `nine_track_ragged_strip`, moving | `3f8ea404e4ff7ab66f98ae6cebb14fecca16e8ffae721562e3efe9f6f3b4cdad` | yes |
| `console_hoist` `sixty_four_track_console`, quiet | `f0d738e2d3bee09ebadb01624e7dc32175aa54d573da8b95e0a0bf9e51de7be8` | yes |
| `console_hoist` `sixty_four_track_console`, restated | `f0d738e2d3bee09ebadb01624e7dc32175aa54d573da8b95e0a0bf9e51de7be8` | yes |
| `console_hoist` `sixty_four_track_console`, moving | `0065d7bc52ccd7d3356c515bb825d70d0133be7a88d1efda18745cbc50db0c4a` | yes |
| `console_meters` `sixty_four_track_console`, meters_off | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` | yes |
| `console_meters` `sixty_four_track_console`, meters_on | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` | yes |
| `console_observation` `sixty_four_track_console`, absent | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` | yes |
| `console_observation` `sixty_four_track_console`, unarmed | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` | yes |
| `console_observation` `sixty_four_track_console`, armed | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` | yes |
| `console_placement` `sixty_four_track_placement`, split_chains | `c4967abce12441e255348ce1a7f4bad2bdc6f5a454e2147de3d86161f52372ea` | yes |
| `console_placement` `sixty_four_track_placement`, merged_chain | `c4967abce12441e255348ce1a7f4bad2bdc6f5a454e2147de3d86161f52372ea` | yes |
| `console_automation` `sixty_four_track_compressor_automation`, quiet | `fd0d8210c88727b0851eb96fb21a70d5d9498f400bed446932b11a4f594f282a` | yes |
| `console_automation` `sixty_four_track_compressor_automation`, restated | `fd0d8210c88727b0851eb96fb21a70d5d9498f400bed446932b11a4f594f282a` | yes |
| `console_automation` `sixty_four_track_compressor_automation`, automated | `013979516036bf5c954cae32b1d4263b77918b6358b031fee4209da5b9c4b301` | yes |
| `console_mono` `sixty_four_track_mono_pair`, collapse_eligible | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` | yes |
| `console_mono` `sixty_four_track_mono_pair`, collapse_forced_off | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` | yes |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, quiet | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` | yes |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, restated | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` | yes |
| `console_mixing_automation` `sixty_four_track_console_mono_mixing_automation`, automated | `7b20684b09bfe4c99e0a54563d4d702d099ffb98cc23c83e5289ce8cf744c7e9` | yes |

#### V8, `host_web.wasm` `ac3a9353...` against S0's `885aa117...`

| Row | S0 p50 r1 | S0 p50 r2 | S4 p50 r1 | S4 p50 r2 | Δ mean p50 | Δ % | S0 p95 r1 | S0 p95 r2 | S4 p95 r1 | S4 p95 r2 |
|---|---|---|---|---|---|---|---|---|---|---|
| `sixty_four_track_console` document | 232.473 | 232.653 | 234.436 | 235.618 | +2.464 | +1.1% | 242.823 | 242.221 | 243.404 | 246.760 |
| `sixty_four_track_app_shape` document | 163.591 | 162.930 | 147.340 | 146.199 | -16.491 | -10.1% | 169.994 | 169.071 | 154.023 | 153.202 |
| `sixty_four_track_console_mono_mixing_automation`, quiet | 152.050 | 152.901 | 154.314 | 153.984 | +1.673 | +1.1% | 159.875 | 161.257 | 163.501 | 162.360 |
| `sixty_four_track_console_mono_mixing_automation`, restated | 154.373 | 154.774 | 156.779 | 158.191 | +2.912 | +1.9% | 163.621 | 164.073 | 165.766 | 166.097 |
| `sixty_four_track_console_mono_mixing_automation`, automated | 158.362 | 158.181 | 160.356 | 159.974 | +1.893 | +1.2% | 166.748 | 167.759 | 168.811 | 169.233 |

#### V8: output digest equality (gate 5)

| V8 field | Value (S0 r1, S0 r2, S4 r1, S4 r2) | All four equal |
|---|---|---|
| `quiet_output_sha256` | `9b1ed9ff7669774086a5b38575d5e8f3b8a68ee49952f259d54c7eb47cf47733` | yes |
| `restated_output_sha256` | `9b1ed9ff7669774086a5b38575d5e8f3b8a68ee49952f259d54c7eb47cf47733` | yes |
| `automated_output_sha256` | `c5fb4e46b0088de271e7f04fbd1fb973d01efbb35522ff7240cb567707f50b81` | yes |
| `documents[0].output_sha256` (`sixty_four_track_console`) | `f69277c581165885756495842c2f750bf9a513d0bdc2617a761efe94f9e8c34a` | yes |
| `documents[1].output_sha256` (`sixty_four_track_app_shape`) | `b076ab14f81cb8ec9e7a0901c19590ff912d5e691adb9f3d35d35aad11fe538c` | yes |
| `preflight_output_sha256.quiet` | `014e5f5b190e0966d8cd83c872e8d2fff5d0d8de14d5568d3277f53ed6b02dd8` | yes |
| `preflight_output_sha256.restated` | `014e5f5b190e0966d8cd83c872e8d2fff5d0d8de14d5568d3277f53ed6b02dd8` | yes |
| `preflight_output_sha256.automated` | `e7025b5cbcea96cbfe8a2f2ac1d0834273d3fde28ee2f4c4021913c498c0ecd3` | yes |
| `preflight_output_sha256.automated_eq_only` | `2540aff4259b362371b341488c43deecf503a7276f854df9b0dbc6805b7f2698` | yes |
| `preflight_output_sha256.automated_compressor_only` | `c29d12a7b758fb885dd4c5d4034e0fa827aa8751548d365545c9fe2a738b144c` | yes |
| `preflight_output_sha256.automated_limiter_only` | `8db18991bed9035e6cc4b4f42c9e6ee0067e1a87a3fce9e3426f180b0d6e6f15` | yes |
| `preflight_output_sha256.restated_eq_only` | `014e5f5b190e0966d8cd83c872e8d2fff5d0d8de14d5568d3277f53ed6b02dd8` | yes |

### Analysis

All figures are means of the two measured p50 rounds, from the tables above.

**Common-mode motion.** Some rows' compiled plans no console slice changes: every row at 16, 64
or 128 tracks with no bypass. Their banks were full before and after, and S1a's lowering is class
A. These rows moved between -4.0 % (`dispatch_only`) and +3.1 % (`eq_only`), and 14 of the 17
moved between +0.6 % and +2.1 %. This includes `builtins_only`, which has no console slot at all
(+2.1 %). This band separates the two runs without any structural cause, and S4 did not isolate
its source. Motion inside it is not attributed to the console design.

The same holds on V8, where the console document and the three mixing-automation arms moved +1.1 %
to +1.9 %, even though S4's V8 run was the less loaded of the two.

The hoist arms' mean motion of about -2 µs comes from S0's round 2, which was an outlier (quiet
6.49 against 10.95 µs at N=9). S4's two rounds agree with S0's round 1.

**H5, the padded remainder (decision 12, "Banking").** At S0 a console remainder rendered per node
(S2's problem statement). At S4 it is one padded bank per slot.

What the rows give:

| Figure | S0 | S4 |
|---|---|---|
| A full bank of the three slots, (N=64 - `builtins_only`) / 8 | 9.30 | 9.39 |
| One per-node track: N=10 - N=9, and (N=13 - N=9) / 4 | 4.82, 4.87 | (none) |
| N=9, 10, 13, 16 | 20.07, 24.89, 39.57, 23.86 | 24.53, 24.24, 25.16, 24.16 |

- At S4 the four ragged-strip rows cost the same to within 1 µs, because a padded bank costs a
  full bank whatever its member count.
- H5's bank figure (9.3 µs) holds. Its per-node figure (4.2 µs) measures about 4.8 µs here.
- The motion is set against H5's arithmetic (padded bank 9.3 against per-node 4.2 µs per member):

  | Row | Members in the remainder | H5 arithmetic | Measured |
  |---|---|---|---|
  | N=9 | 1 | +5.1 | +4.46 (+22.2 %) |
  | N=10 | 2 | +0.9 | -0.65 (-2.6 %) |
  | N=13 | 5 | -11.7 | -14.40 (-36.4 %) |

  With the per-node cost measured here (about 4.8 µs) in place of 4.2, the same arithmetic lands
  within 0.4 µs of each measured motion.
- The break-even at W=8 is therefore two members, not between two and three. Per node wins only
  for a one-member remainder (N = 8k + 1), by about 4.5 µs per block for the three-slot strip.
  Padding is level at two members and wins from three.
- The single-EQ nine-track row (`nine_track_baseline`, a one-member remainder of one EQ slot)
  moved +0.12 µs, inside the band.
- W=4 (wasm simd128 and NEON) is unmeasured. The V8 runner has no ragged row.

Recommendation: none. The owner accepted this cost knowingly. It is confined to one-member
remainders and measured at +4.46 µs on the nine-track strip, while the five-member remainder gained
14.40 µs.

**Bypass: a bypassed lane runs the wet path (decision 12, "Bypass").** The app shape moved
-6.75 µs natively (75.32 to 68.57, -9.0 %) and -16.49 µs on V8 (163.26 to 146.77, -10.1 %).

- At B0 the bypass was part of the bank key. The builder's own doc at `58f8c77b` says "the
  bypassed tracks bank apart from the rest and skip the wet path".
- The 43 active and 21 bypassed tracks therefore formed separate cohorts (43 = 5 x 8 + 3, and
  21 = 2 x 8 + 5), each with a partial remainder. The records carry no bank census, so this is the
  rule applied to the fixture, not a measurement.
- At S4 all 64 lanes form eight full `pre_insert` banks, and the 21 bypassed lanes run the wet path
  under the shunt.
- The wet-path charge is visible against `sixty_four_track_eq_comp_simd1`. That row has the same
  EQ -> compressor slots on the standing fixture, with no limiter and no track bypassed, which is
  what the app fixture derives from.
  - At S4 the app shape costs +8.55 µs more (68.57 against 60.01, +14 %). That is the measured cost
    of bypassing 21 of 64 lanes in shared banks.
  - At S0 the same difference was +16.08 µs, when bypassed lanes skipped the wet path but split the
    cohort.
- The accepted cost is real, but smaller than the cohort split it replaced, and the row got faster
  on both targets.

Recommendation: none. #892 does not apply here, because the row carries no latent slot.

**Silence: the fast path is bank-wide (decision 12, "Silence"; M4).** The sparse row feeds tone to
even tracks and exact zeros to odd ones, so every bank is half silent. The plan is the standing
console's.

| Row | S0 | S4 |
|---|---|---|
| all active (`sixty_four_track_console`) | 99.24 | 100.51 |
| sparse, half of every bank silent | 98.73 | 100.11 |
| idle, every lane silent | 30.78 | 31.16 |
| sparse as a share of all-active | 99.5 % | 99.6 % |
| all-active minus sparse, as a share of all-active minus idle | 0.51 of 68.46 µs (0.74 %) | 0.40 of 69.35 µs (0.57 %) |

- The console slices did not move this: the standing console's banks were full before and after,
  and the sparse row moved +1.4 %, inside the band.
- With half of every bank silent, the bank-wide path recovers under 1 % of the effect cost that the
  idle row shows silence can remove.

A per-lane silence skip is warranted. The only silence path a console bank has does nothing unless
all of its lanes are silent. Decision 12 now puts every console slot in a bank for every track
count. The owner's standing priority is that silence costs nothing at every stage. The draft
successor issue follows. It is not filed, and it quotes no saving: the sparse row measures only
what the bank-wide path fails to recover.

**`post_insert` splits by level (decision 12, "Banking"; H2).** Not measured.
- Every B0 row with a `post_insert` slot has empty inserts on every track. Each such slot therefore
  sits at one level and forms `ceil(N / 8)` groups.
- The app shape has no `post_insert`.
- The placement arms (`split_chains` against `merged_chain`) move EQ and compressor between racks,
  not across levels, and record 8 transposes per block in both runs.

S4 therefore can neither confirm nor size H2's split, or its chain-fusion round trip. It makes no
ALAP recommendation. Sizing it needs a row with mixed insert counts ahead of a `post_insert` slot,
which would be qualification work for its own issue.

**Latency is always paid, and the latent limiter's dry line (decision 12, "Bypass"; #892).** Not
moved.
- #892 is the limiter's dry-line feed when nothing is bypassed. No slice changed it.
- The limiter rows without bypass moved inside the band: N=64 +1.3 %, 128 tracks +1.1 %.
- The one row with bypass, the app shape, has no limiter.

Recommendation: none from this evidence.

**#971 under padding (S2's amendment-10 decision; M3).** This is not an accepted cost, but S2 was
told not to demote mono tracks silently. The mono pool's saving against the forced-dual row was
36.5 % at S0 (63.44 against 99.89) and 35.7 % at S4 (64.74 against 100.76). The collapse arms read
the same: `collapse_eligible` against `collapse_forced_off` is 64.78 against 101.16 at S0, and
65.52 against 101.47 at S4.

S2's L5 is the cost of retiring the demotion: one padded one-plane bank per slot, wherever a
mono remainder fits the stereo pool's padding. It is not measured here. No B0 row has a partial
mono group: the mono rows carry either 64 mono tracks or 32 mono and 32 stereo, alternating.

**A stale doc comment, not repaired.** In `tools/console-workload/src/lib.rs` at `f7ba70a8`, the
`SixtyFourTrackAppShape` doc still describes B0's shape. It says the effects sit "in its `dynamic`
rack" and that "Today a bypassed effect is part of its bank key". The row's record correctly states
`pre_insert:eq+compressor`. This is S1a's or P1's doc, and S4 changes no code.

#### Draft successor (not filed): skip silent lanes' work in console banks

```markdown
# Skip silent lanes' work in console banks

## Problem

Console slots always bank (decision 12). A bank's silent fast path is bank-wide:
`block_is_positive_zero` runs over frames x lanes. One active lane therefore keeps its bank-mates
processing. S4 (#1099) measured this at `f7ba70a8` (p50 µs per block, the mean of two rounds):
- `sixty_four_track_console` (all active): 100.51;
- `sixty_four_track_console_sparse` (odd tracks exact zeros): 100.11;
- `sixty_four_track_idle`: 31.16.

With half of every bank silent, the fast path recovers 0.40 of the 69.35 µs that silence removes
when whole banks are silent. That is against the owner's standing priority that no stage processes
silence. Masking a lane inside a vector removes no arithmetic, because the lanes run in lockstep.
The work can fall only if silent lanes stop occupying bank vectors.

## Smallest closable slice

Decide, prototype and measure one mechanism by which a console slot does no vector work for a lane
whose input block is exact positive zero and whose state is at the kernel's exact fixed point.
For example: per block, pack a slot group's active lanes into as few banks as they fill, and
leave the silent lanes untouched.

- Bits: banking may couple lanes' cost, never their bits.
- Latent slots: a silent lane of a latent slot still feeds its latency line (amendment 11).
- Masking: D7 recovery and reports stay masked by active lanes.

Name what the mechanism does at W=4 and at W=8. If it needs a new benchmark row (for example one
active lane per bank), split that row into a qualification issue first.

## Objective gates

1. Class A: a committed randomized differential renders sessions with random per-lane silence
   runs, with the skip and without it, on Simd8 and Simd4. Every lane is bit-identical, with NaNs
   folded.
2. Render allocates nothing: `allocations == 0`, measured on the render thread after warm-up.
3. One native run and one V8 run of the console benchmark. The sparse row is reported against S4's.
   The all-active console row is not more than 2 % slower than S4's.

## Dependencies

- *Measure the console strip against its baseline* (S4, #1099).
```
