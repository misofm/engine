# Bus-and-send baseline and route-work profile (#1229, BM3)

Descriptive evidence only: no decision, no tuning, no projected saving. Sources, cited by these
short names:

- **native**: `console-benchmark.accepted.jsonl` (byte-identical to `console-benchmark.raw.jsonl`)
  and `console-benchmark.disposition.json`;
- **v8**: `web-mixing-automation.jsonl` (two round records);
- **profile**: `bus-send-profile.txt`, the one run of
  `tools/console-workload/examples/bus_send_profile.rs`;
- **facts**: the line BM1's facts test printed
  (`the_bus_send_rows_plan_carries_every_route_unmuted_and_uncompensated`).

## Measured commit and tree (D1)

- **D1 commit: `244a52a0c7c9ac1c64064b94265079a017c0aab9`**, which holds BM1 (#1227) and BM2 (#1228),
  each with a Sol PASS, and nothing of this issue (`tools/console-workload/examples/` held only
  `mixing_automation_controls.rs`). Both runners name it (native `candidate_commit`; v8
  `candidate_commit` and `prepared_commit`). One commit served both paths.
- **Tree:** a clean detached worktree, `git -C /home/bl/misofm/engine worktree add --detach
  /home/bl/misofm/wt-1229-d1 244a52a0c`; `git status --porcelain=v1 --untracked-files=all` was
  empty before the preflights. WORKDIR was a fresh `mktemp -d` (`/tmp/claude-1002/kb-1229/web-YMInqj`)
  outside it. The worktree was removed (`git worktree remove`) after the step directory was copied
  here.

## Every invocation, in order

| # | command (in the D1 worktree) | outcome |
|---|---|---|
| 1 | `run-web-mixing-automation-benchmark.sh prepare "$W"` | exit 0; printed `host_web.wasm 30d075d3…aeff4 at 244a52a0c… (release pin 6c952a2c…: not the released module)` |
| 2 | `run-web-mixing-automation-benchmark.sh preflight "$W"` | exit 0; premises line with all three document digests (`sixty_four_track_console_sends` `cf5aca93…`) |
| 3 | `operator/preflight-console-benchmark.sh --step bus-send-base` | exit 0, `console benchmark preflight: PASS (workload launches 0)` (its two `Aborted (core dumped)` lines are its own refusal probes: the empty round marker and the extra argument) |
| 4 | **native run** (lock + waiver) | **launched once**, exit 0; disposition `"status":"PASS"`, `"reason":"complete"`, `"workload_process_launches":3`, `"warmup_launches":1`, `"measured_rounds_completed":2`. Terminal output: the accepted file's path. `console-benchmark.stderr.log` (88 B, not committed): `console fixture: ok (64 tracks, 8 full banks, EQ + compressor strip, 13 distinct trims)` |
| 5 | **V8 run** (lock + waiver) | **launched once**, exit 0; wrote `web-mixing-automation.jsonl`, no refused file. Terminal output: the record's path. `web-mixing-automation.stderr.log`: empty (0 B, not committed) |
| 6 | profile example (after both timed runs) | one run, exit 0, stderr empty (D2; below) |

No refusal occurred, so nothing was deleted or re-run. Lock:
`flock -w 7200 /tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock`,
with `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` on both timed commands, as the spec's Context writes
them.

## Host, admissibility and load

- **Host** (native): `cpu_model` AMD EPYC 7313P 16-Core Processor, `governor_or_power_mode`
  schedutil, `backend` Simd8, `target_features` `runtime-avx2,fma;baseline`, `rust_version` rustc
  1.97.1, `llvm_version` 22.1.6, `cpu_affinity` 31. No `perf` (`perf_event_paranoid` 4), so no
  cycle columns.
- **Host** (v8): `node_version` v22.23.2, `v8_version` 12.4.254.21-node.56, `node_flags`
  `--no-liftoff`, `cpu_affinity` 31, `module_sha256` `30d075d3…aeff4`, `module_matches_pin` false
  (this commit's build, not the release pin).
- **Pre-run waits** (untimed, launching nothing, until the 1-minute loadavg < 0.45, cap 900 s):
  - native: the wait hit its **900 s cap** (901 s; the cause of the load was not captured: the
    one recorded fact is loadavg `1.13` at the cap);
    `/proc/loadavg` before the run `1.13 1.40 2.17` (19:04:13Z), after `1.74 1.58 2.17` (19:05:38Z);
  - V8: 501 s; before `0.44 1.01 1.66` (19:14:09Z), after `0.76 1.06 1.67` (19:14:20Z).
- **Admissibility as the records state it:**
  - native: `measurement_control` `uncontrolled`; `background_load_note`
    `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 1.57 1.54 2.16 …;
    affinity cpu 31; smt siblings 15 cpu15=0.00%; cooldown 60s waited 60s` (only the loadavg
    ceiling was waived);
  - v8: `measurement_control` `controlled; loadavg 0.44 1.01 1.66 …; ceiling 0.50; affinity cpu 31`
    in both rounds.

## Timed rows (per measured round, nanoseconds per 128-frame block)

Native, static plan (`record` `console_session`; `p50/p95/p99_ns_per_block`):

| row | round | p50 | p95 | p99 |
|---|---|---|---|---|
| `sixty_four_track_console_sends` | 1 | 160926 | 167158 | 171116 |
| `sixty_four_track_console_sends` | 2 | 160084 | 167098 | 170825 |
| `sixty_four_track_console` (standing) | 1 | 100421 | 105871 | 107234 |
| `sixty_four_track_console` (standing) | 2 | 100151 | 105601 | 108175 |

`output_sha256`: sends `17fe04df…098c4` and standing `6bae1dd0…f243`, each identical in both
rounds; `render_errors` 0, `render_total_forbidden_operations` 0.

V8, live controls (`documents[]`, `p50_ns`/`p95_ns`/`p99_ns`):

| document | round | p50 | p95 | p99 |
|---|---|---|---|---|
| `sixty_four_track_console_sends` | 1 | 336606 | 349942 | 356335 |
| `sixty_four_track_console_sends` | 2 | 345253 | 373797 | 496082 |
| `sixty_four_track_console` (standing) | 1 | 231845 | 243076 | 249459 |
| `sixty_four_track_console` (standing) | 2 | 239279 | 262023 | 333721 |

The sends document's `output_sha256` is `39b9d7a0…29278` in both rounds. (The record also carries
the app-shape document, p50 145770 / 149958, and the mixing-automation arms, e.g. `quiet_p50_ns`
150359 / 159066, which this issue does not read.)

## Plan facts (BM1)

Run once in the batch worktree before timing (its library code is the D1 commit's; this issue
changes nothing under `tools/console-workload/src/`):

```text
bus_send_plan_facts route_transforms=202 reduction_nodes=11 bank_route_folds=0 route_ops_per_block=202 delayed_route_edges=0
```

## Route-work profile (D2, D3)

- **Run:** after both timed runs, under the same lock, pinned to the records' CPU:
  `flock -w 7200 "$LOCK" taskset -c 31 target/release/examples/bus_send_profile`, built with D2's
  command (the runner's profile overrides). `/proc/loadavg` before `0.87 1.07 1.67`, after
  `0.88 1.07 1.67`. It printed measurements once and was not re-run.
- **Probes** (profile summary and repeat lines): clock read 20.0 ns; **in-situ probe cost 33.1 ns**
  (median of 33.7, 29.9, 33.1); **201.0 clock reads per block** (18.0 graph, 183.0 chain); probed
  total 171693.2 ns against net total 165067.4 ns.
- **`runs()` usage: 15 of 16 entries**, so not truncated: graph probes per block (18.0) equal
  start + source + 15 runs + finish (18), and the net figures are usable.
  `64 x bound, 16 x bank, 64 x route, 8 x bank, 1 x other, 64 x route, 1 x bank, 1 x other,
  64 x route, 8 x other, 2 x bank, 2 x other, 2 x bank, 10 x route, 1 x output`.
- **Denominator:** the **profiled block time** is `net_total` = **165067.4 ns** (the profile's
  `total` line: the median over the three repeats of each repeat's sum of graph-phase net
  times, so the printed per-line medians add to 165066.3 ns, not 165067.4 ns; no share moves at
  two decimals). Every share below and in the profile is of it. (The profile's own probes-off
  p50, 165097 ns, is the untimed instrument's figure and is never compared with the timed
  records.)

| share | lines read (profile, net ns) | net ns | of 165067.4 ns |
|---|---|---|---|
| **(a) route ops** | `route` (4 runs, 202 units) | 13338.7 | **8.08 %** |
| **(b) route-input reductions, upper bound** | `output` 281.4 + `identity-copy` 0.0 + `other` 6033.8 | 6315.2 | **≤ 3.83 %** |
| **(c) bank fold** | `fold` (0 entries) | 0.0 | **0.00 %** |
| (a) + (b), upper bound | | 19653.9 | ≤ 11.91 % |

- (b) is an upper bound: the `other` runs hold 1 + 1 + 8 + 2 = **12 `OTHER_OP` units** (`runs()`).
  The facts give 11 reduction nodes; `main-out`'s is the `OUTPUT` unit and `identity-copy` has no
  units, so **10 of the 12** `OTHER_OP` units are submix input reductions, and the other 2 are
  per-node inserts whose time is inside the 3.66 % `other` line and cannot be split out by this
  instrument.
- (c) is zero: `bank_route_folds` is 0 (facts and profile census), and the `fold` sub-phase had no
  entries. A zero line is a valid reading.
- For context only: `bank (whole unit)` is 85.98 %; its slot positions 0-2 are `(mixed)`, so no
  console slot's share is attributable on this row (Context). `bound` 2.11 %, `gather` 4.51 %,
  `scatter` 3.95 %. `enter` nets to -3.9 ns (within one probe cost of zero).

## Reading under DESIGN 6.3 (D4)

**(a) alone is at or above about 5 %** of the profiled block time (route ops 8.08 %; (a) + (b) is
not below 5 %, at most 11.91 %). Under 6.3's guidance this places O1 in the "candidate for a brief"
band; whether it earns one is the weekly performance pass's decision on these numbers. No saving
figure is stated for O1, and no fusion spec or optimisation issue is filed.

## Side reports (D5)

- **R9 (live-controlled fold loss).** The static plan folds **0** routes (facts `bank_route_folds=0`;
  profile census `bank_route_folds = 0`), so the V8 plan can fold at most 0 of its routes into the
  output. Sends-row p50 beside it: native 160926 / 160084 ns, V8 336606 / 345253 ns. **R9 stays
  unevaluated on this fixture**: its static plan folds nothing, so the loss cannot show.
- **O5 (bus bank alignment): submix bank census** (profile `census chain` lines, from
  `unit_eligibility()`; bank width 8 from `Backend::current()` = `Simd8`):

  | unit | lanes | active lanes | width | pad lanes |
  |---|---|---|---|---|
  | 217 | `fx-a` | 1 | 8 | 7 |
  | 291 | `fx-b` | 1 | 8 | 7 |
  | 292 | `bus-0` … `bus-7` | 8 | 8 | 0 |
  | 295 | `fx-a` | 1 | 8 | 7 |
  | 296 | `fx-b` | 1 | 8 | 7 |

  5 of the 29 bank chains (`bank_shape [29, 67]`) carry submix lanes, with 28 pad lanes in all; the
  other 24 are 8-active-lane track chains with no pad. This instrument attributes no time to pad
  lanes, so **O5's trigger (padded-bank cost above 5 % of the bus row) stays unevaluated**.
- **C ABI shape at measurement time: static.** At the D1 commit, #1053 and #1225 are open (their
  specs are in `.github/ISSUE_SPECS/`); `prepare_runtime` (`crates/capi/src/runtime/compile.rs:412-421`)
  calls `prepare_host_runtime` (`crates/host-core/src/prepare.rs:563-574`), which requests
  `HostLiveControlRequest::default()`: no live controls, no meters. That is the native row's shape.
