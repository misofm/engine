# Record the console-strip baseline benchmark

Slice S0 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`).

## Problem

S4 reports the console strip against a baseline. The baseline has to be the engine before any
console slice changes banking, bypass or the schema, measured on the rows B0 froze.

## Smallest closable slice

Time B0's merge commit, once:

- native: `scripts/operator/run-console-benchmark.sh --step console-strip-base`, after
  `scripts/operator/preflight-console-benchmark.sh --step console-strip-base` passes;
- V8: `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
  WORKDIR`, then `run WORKDIR --step console-strip-base`, for the existing row and B0's two
  documents, on the module built from the same commit.

Each run is one invocation, with one warmup and two measured rounds. If a later console slice has
merged by then, time a clean detached worktree of B0's commit; the runner refuses a dirty tree.

Report the `measurement_control` field as the records carry it. The runner takes no lock, so name
the operator's `flock` (or its absence) and the host's background load in the disposition.

Authorized paths: `artifacts/steps/console-strip-base/**`, the V8 record directory the runner
writes, and this spec's evidence section.

## Dependencies

- *Add the console-strip benchmark rows* (B0, #1085).

This closes batch C1 (R0, B0, S0). Push C1 with S0's records before any engine slice lands, so the
baseline is recorded on the unchanged engine.

## Objective gates

1. Both runners accept and promote their records, and the validators pass.
2. The records name B0's commit as `candidate_commit`.
3. No tuning, no retry and no second invocation. If post-workload tooling fails, preserve the raw
   output, record the failure and open a tooling issue (AGENTS.md benchmark rules).
4. The evidence section tabulates, per row, p50 µs per block for both rounds and the
   sparse-activity row against the all-active and idle rows. Quote no projection.

## Non-goals

No interpretation beyond the table. S4 does the before-and-after comparison.

## S0 evidence

Terra, one invocation per runner. The timed commit is B0's merge on `codex/batch-console-1`,
`58f8c77b08e1ef344a3ad5d18c013fb64d555b8a`, on a clean detached worktree. That is the engine
before P1, P2 and S1a. During this session, while the preflights ran, the batch branch gained
`b7cae659`, a spec-only commit. The records name `58f8c77b` and nothing else.

### Commands

These are Sol's corrected block from #1085 (M1), run as written. Each script's argument form was
checked against its usage line first, and all of them matched.

```bash
LOCK=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock
B0=58f8c77b
BATCH=/home/bl/misofm/worktrees/engine-console
WT=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/s0-wt
git -C /home/bl/misofm/engine worktree add --detach "$WT" "$B0" && cd "$WT"
W=$(mktemp -d -p /tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad)   # .../tmp.tni2xhy8FT
bash scripts/run-web-mixing-automation-benchmark.sh prepare "$W"
bash scripts/run-web-mixing-automation-benchmark.sh preflight "$W"
bash scripts/operator/preflight-console-benchmark.sh --step console-strip-base
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/operator/run-console-benchmark.sh --step console-strip-base
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/run-web-mixing-automation-benchmark.sh run "$W" --step console-strip-base
cp -r artifacts/steps/console-strip-base "$BATCH/artifacts/steps/"
```

Before each timed run, the operator waited for the host to go quiet, with a condition fixed in
advance: a 1-minute loadavg below 0.45, capped at 900 s. Nothing was launched or timed during
either wait, so neither wait is a retry.
- Before the native run, the loadavg reached 0.43 after 360 s.
- Before the V8 run, the wait hit its 900 s cap with the loadavg still near 1.1.

The operator's lock was free (`fuser` showed no holder), and `flock` acquired it at once for both
runs.

### Untimed preflights

- V8 `prepare`: exit 0.
  - `host_web.wasm` is `885aa117ed0133f325a08992e0df0265e2984b1b63a163b8a8708722322c6545`,
    built at `58f8c77b`. It is not the release pin, `6c952a2c...`.
  - `controls.json` is `0c17c10eb03f2ae2323d050fdce763f5beb75e60daf1e8367821f552c3a9aa47`.
- V8 `preflight`: exit 0. The documents' digests, `d913ad96...` for the console and `3dd8b2ff...`
  for the app shape, match Sol's #1085 preflight.
- Native preflight: PASS, with `workload_launches` 0, `records_required` 60 and candidate
  `58f8c77b`. It built the binary `cc25e761...` under the default release profile.

### Timed runs, load and control

All times are UTC, on an AMD EPYC 7313P (32 CPUs) with the `schedutil` governor. Both runners
pinned CPU 31.

| Run | Before | Admissibility read | After | `measurement_control` |
|---|---|---|---|---|
| native (00:10:59 to 00:12:23) | `0.60 1.33 3.11` | `0.91 1.35 2.99` | `1.01 1.35 2.97` | `uncontrolled` |
| V8 (00:27:49 to 00:27:58) | `1.06 1.22 1.85` | `1.06 1.22 1.85` | `1.21 1.25 1.85` | `uncontrolled` |

- Both runs are `uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived
  loadavg_above_ceiling`, against a ceiling of 0.50. No other precondition was waived.
- Native: the SMT sibling, cpu15, was 0.00% busy. The cooldown was 60 s, and the runner waited all
  of it, because it rebuilt `bench` under its frozen release profile. That rebuild produced binary
  `31f1c6a1...`, not the preflight's `cc25e761...`. The loadavg rose from 0.60 to 0.91 over
  that rebuild and cooldown, before the admissibility read. The host has no usable `perf` counter, so no core-clock CSV
  was written and the records carry no cycle columns.
- V8 per round, loadavg at start and end: round 1 went from `1.06` to `1.14`, and round 2 from
  `1.14` to `1.21`.
- Background load: no `cargo` or `rustc` process was running when either timed run started. A
  service outside the engine, `blast-cetus-pipeline` (Docker), held about one core throughout. It
  was PID 4042097 at the session start and restarted at about 00:17 as PID 77309, at about 95% of a
  core. That is why the second wait never reached its target.

### Identities and gates

- Native disposition: `PASS complete`, `runner_invocations` 1, `workload_process_launches` 3
  (1 warmup and 2 measured), and 60 records.
  - `raw_sha256` and `accepted_sha256` are both
    `9d6e2ad220cff9a88474683fa833492034880926e4cdb0ffbd5e9b298cafeaff`.
  - `binary_sha256` is `31f1c6a111ce2f1176483d97fdaae295992529e081d5287116a5792001b56656`.
- V8: `web-mixing-automation.jsonl` is
  `f0484d33444993ab49976cd26b5b8bab76953fd8179d86af51e4fd9447b3471a`, with two rounds.
  - `prepared_commit` equals `candidate_commit`, and `module_matches_pin` is false.
  - The runtime is Node v22.23.2 with V8 12.4.254.21-node.56, run with `--no-liftoff`. Each round
    has 1000 observations and 64 preroll blocks.
- The stderr logs match `.gitignore`'s `*.log` and are not committed.
  - The native log is 88 bytes, `console fixture: ok (64 tracks, 8 full banks, EQ + compressor
    strip, 13 distinct trims)`, and the disposition pins it as `042f18e1...`.
  - The V8 log is empty.

Gates:
1. PASS. Both runners accepted and promoted their records. Re-running
   `console-benchmark-validator.jq` and `web-mixing-automation-validator.jq` on the copied records
   in the batch worktree passes both. The validators are unchanged since `58f8c77b`.
2. PASS. All 60 native records and both V8 rounds name `58f8c77b08e1ef344a3ad5d18c013fb64d555b8a`
   as `candidate_commit`.
3. PASS. There was one invocation per runner, with no tuning and no retry, and no post-workload
   tooling failed.
4. The tables follow.

### Tables

Units are µs per block at 48 kHz and a 128-frame quantum. Percentiles are nearest-rank over 1000
observations per round. r1 and r2 are the two measured rounds. Every digest is identical across
the two rounds.

The digests are not the 64-block pins from B0, which cover 64 blocks rather than this run's blocks.
V8 digests do not compare with native ones either, because the harness feeds a streamed continuous
tone, as its header states.

The hoist arms' records carry no p95 for the quiet arm.

#### Native, Simd8: the console-strip rows (µs per block)

| Row | Tracks | p50 r1 | p50 r2 | p95 r1 | p95 r2 | `output_sha256` |
|---|---|---|---|---|---|---|
| strip N=9 (`nine_track_ragged_strip`) | 9 | 19.948 | 20.189 | 20.449 | 20.549 | `cd98694d647e73d1dc2ffbe3891d16a29e11031b083e48cd044f622ca77f6814` |
| strip N=10 (`ten_track_ragged_strip`), new | 10 | 24.708 | 25.068 | 25.448 | 25.739 | `45eefb82824053281761cd5799edaeddfa542097bb1362457c2623fbe04dac2b` |
| strip N=13 (`thirteen_track_ragged_strip`), new | 13 | 39.385 | 39.745 | 40.867 | 40.988 | `64015ad3c502898e8b53486637ea1233f642c36af6a97a51fd835364abe75a23` |
| strip N=16 (`sixteen_track_strip`), new | 16 | 23.875 | 23.846 | 24.466 | 24.777 | `8957e9904258732df6575cd205b51aab282b80c87808aa8caad424d6db5cba7b` |
| strip N=64 (`sixty_four_track_console`) | 64 | 99.480 | 98.999 | 104.960 | 104.569 | `6bae1dd0eacc770044e64de2699e35eac2fb8aa192f98e8180bc036fe7edf243` |
| app shape (`sixty_four_track_app_shape`), new | 64 | 75.254 | 75.384 | 80.614 | 80.734 | `3ce17e3e31bbbe23f7a446ef04b54adc4c605b25dba0fdc8a58d7221a84055b8` |
| sparse activity (`sixty_four_track_console_sparse`), new | 64 | 98.547 | 98.918 | 103.818 | 104.780 | `8076e45021ca08a99b28e59d97f4d04cd13c7a1f7bee6daf021ce9ce6485cccb` |

#### Native, Simd8: sparse activity against all-active and idle (µs per block)

| Row | Tracks | p50 r1 | p50 r2 | p95 r1 | p95 r2 | `output_sha256` |
|---|---|---|---|---|---|---|
| `sixty_four_track_console_sparse` | 64 | 98.547 | 98.918 | 103.818 | 104.780 | `8076e45021ca08a99b28e59d97f4d04cd13c7a1f7bee6daf021ce9ce6485cccb` |
| `sixty_four_track_console` | 64 | 99.480 | 98.999 | 104.960 | 104.569 | `6bae1dd0eacc770044e64de2699e35eac2fb8aa192f98e8180bc036fe7edf243` |
| `sixty_four_track_idle` | 64 | 30.678 | 30.888 | 31.129 | 31.701 | `7b331c02e313c7599d5a90212e17e6d3cb729bd2e1c9b873c302a63c95a2f9bf` |

#### Native, Simd8: the other existing session rows (µs per block)

| Row | Tracks | p50 r1 | p50 r2 | p95 r1 | p95 r2 | `output_sha256` |
|---|---|---|---|---|---|---|
| `nine_track_baseline` | 9 | 7.835 | 7.725 | 7.925 | 7.805 | `d5df5ebe109d28b4ed96eb2f703a2301448e9df6a9c5f10f7d0af58732a58533` |
| `one_twenty_eight_track_stretch` | 128 | 199.008 | 198.258 | 206.964 | 205.191 | `1fd1755ec5adb86b08240580abff443128ef2bdd187501ddf8b83fefd87176ab` |
| `sixty_four_track_eq_only` | 64 | 35.428 | 34.987 | 36.389 | 35.989 | `83a4b205c38320f4c48eb038a811ccb826230a47f9687d05674c7ae3dbac5a3d` |
| `sixty_four_track_compressor_only` | 64 | 48.472 | 48.572 | 52.981 | 52.500 | `2d8712e612474b55ae70ad18a0242f41bf9b8b20fbc70e62f937c64c3d0b41ad` |
| `sixty_four_track_builtins_only` | 64 | 24.828 | 24.897 | 25.469 | 25.158 | `5bf3c3772d4cb400e3cbe703cd750363a1b8ba7e612b2e3f15292c1cda5e6d26` |
| `sixty_four_track_dispatch_only` | 64 | 11.371 | 11.432 | 11.522 | 11.613 | `2b015145fd33d19a8a84bc6bbe0caaa65a50bd129a0f2512fe44a9cdb21b1ad7` |
| `sixty_four_track_idle` | 64 | 30.678 | 30.888 | 31.129 | 31.701 | `7b331c02e313c7599d5a90212e17e6d3cb729bd2e1c9b873c302a63c95a2f9bf` |
| `sixty_four_track_console_legacy` | 64 | 59.313 | 59.143 | 64.233 | 63.912 | `f41ad6354fc25d4e4eed04d1f8032fe8068c267cbe9dc7afcc0ee595fffd9cfd` |
| `sixty_four_track_eq_comp_simd1` | 64 | 59.844 | 58.642 | 64.603 | 63.171 | `f41ad6354fc25d4e4eed04d1f8032fe8068c267cbe9dc7afcc0ee595fffd9cfd` |
| `sixty_four_track_gain_pan_only` | 64 | 10.580 | 10.901 | 10.701 | 11.061 | `2410d7237d2173526f967b897835b52eb515196c655920b09ca93acb111bb7b1` |
| `sixty_four_track_console_mono` | 64 | 63.271 | 63.601 | 67.929 | 67.759 | `fa1eb6bff3b0780c9b26cbbb75c1f9071f04769eb2edaf0029970ffb3f6ca049` |
| `sixty_four_track_console_mono_dual` | 64 | 99.900 | 99.870 | 105.531 | 105.691 | `fa1eb6bff3b0780c9b26cbbb75c1f9071f04769eb2edaf0029970ffb3f6ca049` |
| `sixty_four_track_console_half_mono` | 64 | 85.753 | 85.703 | 91.284 | 90.944 | `50f6e28443ac921fb3ae0508ea9ec218f81bbadd9158cd0a2d0c31d350032732` |
| `sixty_four_track_gain_pan_ring` | 64 | 8.837 | 9.167 | 8.937 | 9.248 | `2410d7237d2173526f967b897835b52eb515196c655920b09ca93acb111bb7b1` |
| `sixty_four_track_console_metered` | 64 | 103.557 | 103.637 | 109.218 | 109.238 | `6bae1dd0eacc770044e64de2699e35eac2fb8aa192f98e8180bc036fe7edf243` |

#### Native, Simd8: the existing arm records (µs per block)

| Record | Row | Arm | p50 r1 | p50 r2 | p95 r1 | p95 r2 | `output_sha256` |
|---|---|---|---|---|---|---|---|
| `console_hoist` | `nine_track_ragged_strip` | quiet | 6.492 | 10.951 | n/a | n/a | `bac4f68c477c1699fed23306df14a6a380963d30208a526d81908de6e9cc9d93` |
| `console_hoist` | `nine_track_ragged_strip` | restated | 7.885 | 12.333 | 7.975 | 14.026 | `bac4f68c477c1699fed23306df14a6a380963d30208a526d81908de6e9cc9d93` |
| `console_hoist` | `nine_track_ragged_strip` | moving | 18.104 | 18.124 | 18.235 | 18.234 | `3f8ea404e4ff7ab66f98ae6cebb14fecca16e8ffae721562e3efe9f6f3b4cdad` |
| `console_hoist` | `sixty_four_track_console` | quiet | 15.199 | 19.277 | n/a | n/a | `f0d738e2d3bee09ebadb01624e7dc32175aa54d573da8b95e0a0bf9e51de7be8` |
| `console_hoist` | `sixty_four_track_console` | restated | 21.030 | 25.449 | 21.320 | 27.111 | `f0d738e2d3bee09ebadb01624e7dc32175aa54d573da8b95e0a0bf9e51de7be8` |
| `console_hoist` | `sixty_four_track_console` | moving | 68.331 | 68.330 | 73.149 | 73.099 | `0065d7bc52ccd7d3356c515bb825d70d0133be7a88d1efda18745cbc50db0c4a` |
| `console_meters` | `sixty_four_track_console` | meters_off | 98.869 | 98.838 | 103.968 | 104.138 | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` |
| `console_meters` | `sixty_four_track_console` | meters_on | 109.418 | 108.867 | 115.210 | 114.979 | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` |
| `console_observation` | `sixty_four_track_console` | absent | 106.823 | 106.653 | 112.985 | 112.534 | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` |
| `console_observation` | `sixty_four_track_console` | unarmed | 106.462 | 107.284 | 112.223 | 112.875 | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` |
| `console_observation` | `sixty_four_track_console` | armed | 107.976 | 108.456 | 114.278 | 114.248 | `9ef997994e8e63816755557e6300ba38f2087d12e0b2ac111bd467983db772f3` |
| `console_placement` | `sixty_four_track_placement` | split_chains | 60.415 | 61.217 | 65.385 | 68.310 | `c4967abce12441e255348ce1a7f4bad2bdc6f5a454e2147de3d86161f52372ea` |
| `console_placement` | `sixty_four_track_placement` | merged_chain | 60.455 | 61.277 | 65.435 | 68.030 | `c4967abce12441e255348ce1a7f4bad2bdc6f5a454e2147de3d86161f52372ea` |
| `console_automation` | `sixty_four_track_compressor_automation` | quiet | 51.308 | 52.701 | 56.217 | 58.021 | `fd0d8210c88727b0851eb96fb21a70d5d9498f400bed446932b11a4f594f282a` |
| `console_automation` | `sixty_four_track_compressor_automation` | restated | 52.109 | 52.950 | 56.127 | 57.920 | `fd0d8210c88727b0851eb96fb21a70d5d9498f400bed446932b11a4f594f282a` |
| `console_automation` | `sixty_four_track_compressor_automation` | automated | 52.721 | 53.943 | 57.469 | 58.712 | `013979516036bf5c954cae32b1d4263b77918b6358b031fee4209da5b9c4b301` |
| `console_mono` | `sixty_four_track_mono_pair` | collapse_eligible | 64.442 | 65.114 | 69.352 | 70.004 | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` |
| `console_mono` | `sixty_four_track_mono_pair` | collapse_forced_off | 100.651 | 101.663 | 106.613 | 107.124 | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` |
| `console_mixing_automation` | `sixty_four_track_console_mono_mixing_automation` | quiet | 71.386 | 71.586 | 76.305 | 75.524 | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` |
| `console_mixing_automation` | `sixty_four_track_console_mono_mixing_automation` | restated | 72.708 | 73.279 | 77.988 | 76.867 | `ee0c17bfa8a9380d3cd871dd43b1fd225cac990f76e188168f37db381abf0102` |
| `console_mixing_automation` | `sixty_four_track_console_mono_mixing_automation` | automated | 75.684 | 76.185 | 81.204 | 81.195 | `7b20684b09bfe4c99e0a54563d4d702d099ffb98cc23c83e5289ce8cf744c7e9` |

#### V8, `host_web.wasm` `885aa117...` (µs per block)

| Row | Tracks | p50 r1 | p50 r2 | p95 r1 | p95 r2 | `output_sha256` |
|---|---|---|---|---|---|---|
| `sixty_four_track_console` (document, `pre_insert:eq+compressor,post_insert:limiter`, bypass `none` 0) | 64 | 232.473 | 232.653 | 242.823 | 242.221 | `f69277c581165885756495842c2f750bf9a513d0bdc2617a761efe94f9e8c34a` |
| `sixty_four_track_app_shape` (document, `inserts:eq+compressor`, bypass `index_mod_3_is_2` 21) | 64 | 163.591 | 162.930 | 169.994 | 169.071 | `b076ab14f81cb8ec9e7a0901c19590ff912d5e691adb9f3d35d35aad11fe538c` |
| `sixty_four_track_console_mono_mixing_automation`, quiet arm | 64 | 152.050 | 152.901 | 159.875 | 161.257 | `9b1ed9ff7669774086a5b38575d5e8f3b8a68ee49952f259d54c7eb47cf47733` |
| `sixty_four_track_console_mono_mixing_automation`, restated arm | 64 | 154.373 | 154.774 | 163.621 | 164.073 | `9b1ed9ff7669774086a5b38575d5e8f3b8a68ee49952f259d54c7eb47cf47733` |
| `sixty_four_track_console_mono_mixing_automation`, automated arm | 64 | 158.362 | 158.181 | 166.748 | 167.759 | `c5fb4e46b0088de271e7f04fbd1fb973d01efbb35522ff7240cb567707f50b81` |
