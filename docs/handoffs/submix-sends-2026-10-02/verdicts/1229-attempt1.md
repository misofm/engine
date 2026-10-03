# #1229 attempt 1 verdict (Sol): `d1d738e55` + `8808ff5a2` (measured commit `244a52a0c`), branch `codex/batch-bench`

**PASS.** No BLOCKER, MAJOR or MINOR finding. There are four NITs, all about how the work is
described or which steps were captured. None of them changes a number or the D4 reading.

Summary of what I checked:

- The protocol in D1 and D2 was followed. I confirmed this from the raw captures and from file
  timestamps.
- The committed records are the runners' own output.
- Both validators accept the committed files.
- Every number in `REPORT.md`, in the spec's Attempt 1 record and in the #1196 evidence section
  matches its source exactly.
- The shares use one denominator, and the D4 reading is correctly derived from DESIGN 6.3.
- Only authorized paths changed.
- All the non-timed gates pass when I rerun them.
- I timed nothing. I ran no benchmark round and did not run the profile example. I did not modify
  the worktree, which is clean at `8808ff5a2`. Nothing was committed or pushed, and nothing on
  GitHub was edited.

## Protocol (D1, D2): verified

### The measured commit (D1)

- `244a52a0c` descends from `main` (`1cb677a76`). It holds BM1 (`e7bd95f88`) and BM2
  (`79d11ea49`), plus their follow-ups, and both have Sol PASS verdicts.
- `git ls-tree 244a52a0c tools/console-workload/examples/` lists only
  `mixing_automation_controls.rs`, so the profile example is not in the measured commit.
- The V8 harness at that commit asserts
  `miso_engine_web_v1_live_control_route_count == submixRoutes` (`.mjs:464`). So the V8 sends
  document provably ran live routes, which closes #1228's MINOR-1.

### Tree and order (`/tmp/claude-1002/kb-1229/`)

- The detached worktree was created at 18:44 (`01-worktree-add.txt`). It no longer exists, and
  `.git/worktrees/` lists only `wt-bench`.
- The fresh WORKDIR `web-YMInqj` was outside it.
- The steps ran in the order D1 requires:

  | Step | Capture | Time | Outcome |
  |---|---|---|---|
  | V8 `prepare` | `02` | 18:46 | exit 0 |
  | V8 `preflight` | `03` | 18:46 | exit 0 |
  | native preflight | `04` | 18:49 | PASS, workload launches 0 |
  | native run | `05` | 19:04:13-19:05:38 | exit 0 |
  | V8 run | `06` | 19:14:09-19:14:20 | exit 0 |
  | profile | `07` | 19:14:31-19:14:37 | (D2, below) |

- Both `Aborted (core dumped)` lines in the preflight capture come from
  `preflight-console-benchmark.sh:56-61`. They are its own refusal probes, as the report says.

### One invocation per path

- **Native.** The disposition shows `runner_invocations` 1, `workload_process_launches` 3 (one
  warmup and two measured rounds) and `measured_rounds_completed` 2.
  - Its `raw_sha256` and `accepted_sha256` both equal `6395e20e...0ab4`, which is the `sha256sum`
    of each committed file. `cmp` confirms the two files are identical.
  - Its `stderr_sha256` (`042f18e1...`, 88 B) is the hash of the stderr log kept in scratch.
- **V8.** The record holds one file with rounds 1 and 2, and there is no refused file.

### Waits, lock and waiver

- `timed.sh` implements S4's wait: below 0.45, capped at 900 s, with `/proc/loadavg` recorded
  before and after.
- **Native.** The wait hit the cap (`wait capped at 900 s`, `waited 901 s`), and the record honestly
  says `uncontrolled`. Only `loadavg_above_ceiling` was waived, at loadavg 1.57 after the cooldown,
  with cpu 31, its SMT sibling idle and a 60 s cooldown.
- **V8.** It waited 501 s and is recorded `controlled` (0.44 against a 0.50 ceiling) in both
  rounds.
- The waiver is proven by the record's `background_load_note`. The lock is not, because the
  wrapper does not log its own command (NIT-3).

### Profile run (D2)

The profile ran exactly once, on the D2 build, built before timing:

- Rebuilding D2's command with `--message-format=json` reports `fresh: true`. It relinks
  `target/release/examples/bus_send_profile` to `bus_send_profile-1a7fd1bb7b4b81d6`, which is the
  38.5 MB build without LTO, with mtime 18:44:01.
- The example's source mtime is 18:43:52, so the binary was built from the committed source,
  before the D1 worktree existed and before any timed run.
- That binary's atime is 19:14:31.05. On a `relatime` mount that is its first access since it was
  built, and it is the profile's start time. So nothing ran it earlier, such as a smoke run.
- The later `c6ab84d76a8829ea` build (19:15:41, fat LTO) came from gate 3's
  `cargo test --release`. It is not what produced `bus-send-profile.txt`, whose mtime is
  19:14:37.
- The profile was pinned to cpu 31, the records' `cpu_affinity`. Its stderr was empty.

## Records against the report: every number matches

I checked every number against its record field:

- **Native, round 1 / round 2.**
  - Sends: 160926/167158/171116 and 160084/167098/170825.
  - Standing console: 100421/105871/107234 and 100151/105601/108175.
  - `output_sha256` is the same in both rounds: `17fe04df...098c4` for sends and `6bae1dd0...f243`
    for the standing console.
  - Errors are 0 and forbidden operations are 0.
- **V8 `documents[]`.**
  - Sends: 336606/349942/356335 and 345253/373797/496082, with `39b9d7a0...29278` in both rounds.
  - Standing console: 231845/243076/249459 and 239279/262023/333721.
  - App-shape p50: 145770 and 149958.
  - `quiet_p50_ns`: 150359 and 159066.
- **Host metadata.** EPYC 7313P, schedutil, Simd8, `runtime-avx2,fma;baseline`, rustc 1.97.1,
  LLVM 22.1.6, node v22.23.2, V8 12.4.254.21-node.56 and `--no-liftoff` all match.
- **Module.** `module_sha256 30d075d3...aeff4` and `module_matches_pin` false match.
- **Loadavg values and timestamps** match the wait captures.
- **Facts line.** It matches `00-facts-test.txt` byte for byte.

The records carry percentiles, not samples, so p50/p95/p99 can only be checked as field equality,
not recomputed. `REPORT.md` states no throughput ratios. I computed the sends/standing ratio myself
only as a sanity check, and it is not a finding: 1.60x natively and 1.45x on V8 in round 1. The
standing console's native p50 of about 100.3k ns sits within the run-to-run spread of the earlier
`console-strip-base` (99.5k) and `final-0400` (100.0k) records. So the waived load did not visibly
inflate the reference row.

## Profile and attribution (D3): sound

### Recomputed shares, all of `net_total` = 165067.4 ns

| Share | Arithmetic | Result |
|---|---|---|
| (a) `route` | 13338.7 / 165067.4 | **8.081 %** |
| (b) `output` + `identity-copy` + `other` | (281.4 + 0 + 6033.8) = 6315.2, / 165067.4 | **3.826 %** |
| (a) + (b) | 19653.9 / 165067.4 | 11.907 % |
| (c) `fold` | 0 entries | 0 |

### One denominator

- The example divides every line by the single `net_total` (`bus_send_profile.rs`, the `row`
  closure).
- That number is the median over the three repeats of each repeat's sum of graph-phase nets
  (NIT-1).

### Probe correction

- The correction is the same method as `gain_pan_profile.rs`:
  `net = probed - entries x in-situ cost`, and the bank phase also subtracts its 183 nested chain
  reads.
- Recomputed, it holds:
  - route: 13471.2 - 4 x 33.1 = 13338.8;
  - bank: 148147.1 - 188 x 33.1 = 141924.3, against the printed 141918.9 (per-line medians);
  - the per-repeat probe costs (171790 - 165008)/201 = 33.7, 29.9 and 33.1, with median 33.1.
- The reading does not depend on the correction. Uncorrected, route's probed share is
  13471.2 / 171693.2 = 7.85 %, still at or above 5 %.

### Census consistency

- `runs()` uses 15 of 16 entries, and the 18 graph probes per block equal 15 + 3, so nothing was
  truncated.
- The unit runs add up to 308 = 64 bound + 29 bank + 202 route + 12 other + 1 output, which is the
  census's 308 units (29 bank chains and 279 single ops).
- The 202 route units match the facts line's `route_ops_per_block`.
- There are 183 chain reads per block, which is 67 stages + 4 x 29 chains.

### Upper bound on (b)

- 12 `OTHER_OP` units (1 + 1 + 8 + 2).
- There are 11 reduction nodes in all. `main-out`'s is the `OUTPUT` unit, and `identity-copy` and
  `identity-alias` have no runs. So 10 of the 12 are submix reductions: 8 buses plus `fx-a` and
  `fx-b`.
- The other 2 are per-node inserts. The report states (b) as an upper bound for exactly that
  reason.

### Fold

- `bank_route_folds = 0` in both the facts line and the census.
- The `fold` line has 0 entries.

## Example (D2)

It prints everything D2 lists:

- `bank_shape`;
- `slot_names` for all 8 positions, including `(not reached)`;
- every phase and sub-phase, including the zero rows (`identity-*`, `exit`, `slot 7`, `seam`,
  `fold`, `aux`), each as a per-line median with its share;
- the clock-read cost, the in-situ probe cost, the probes per block, the `runs()` list with its
  usage, and the D5 census.

It also stays within what D2 allows:

- It drops `slots == 3 * chains` and the gain/pan labels.
- Where the harness asserts the run's closing counts, it prints a NOTE instead, which is right for
  an instrument that must not panic after printing.
- It asserts nothing about time.
- It is not a `#[test]`, so it claims no test value. No test was added or changed, so there is no
  test-value question to answer.

## D4 and D5

### D4 reading

- DESIGN 6.3 (`DESIGN.md:888-893`) says: route ops plus route-input reductions below about 5 % put
  fusion's best case inside the 2 % allowance. "At or above it, O1 is a candidate for a brief."
- The report applies D4's second reading: "(a) alone at or above about 5 %". That is correct,
  because (a) is a lower bound on (a) + (b).
- It defers the decision to the weekly pass. It states no saving figure, and no fusion spec or
  optimisation issue was filed. The newest GitHub issue is #1251, and #1196 and #1229 have no new
  comments.

### D5 side reports

- **R9.** It reports 0 static folds beside both rows' p50, and says R9 stays unevaluated.
- **O5.** It lists the 5 submix chains:
  - `fx-a` x2 and `fx-b` x2, at 1 active lane and 7 pad lanes each;
  - `bus-0..7`, at 8 active lanes and 0 pad lanes;
  - so 28 pad lanes in all at width 8.

  `PlanUnitEligibility::lane_tracks` holds one entry per active lane, so the pad arithmetic is
  right. The report says O5's trigger stays unevaluated.
- **C ABI shape.** It is static:
  - at `244a52a0c`, `prepare_runtime` calls `prepare_host_runtime`, which passes
    `HostLiveControlRequest::default()` (`compile.rs:412-421`, `prepare.rs:563-574`);
  - #1053 and #1225 are open, both on GitHub and as specs at that commit.

## Scope and artifacts

- **Changed paths.** The diff `244a52a0c..8808ff5a2` touches only the authorized paths:
  - the example;
  - `artifacts/steps/bus-send-base/`, holding the S0 file set plus the profile and the report;
  - the appended #1196 evidence section, since the #1196 spec is still present;
  - this spec, where the Attempt 1 record is append-only.
- **Not committed.** No stderr log is committed. Both logs are quoted in the report: the native
  one's 88 B, and the V8 one, which is empty.
- **No `perf`.** There is no `perf.data` (`perf_event_paranoid` is 4).
- **Unchanged code.** Nothing under `tools/bench/`, `tools/console-workload/src/`, the runners or
  the validators changed.
- **`REPORT.md`, written via shell.**
  - It is exactly the required deliverable, at its authorized path, and it is clean UTF-8.
  - There are no heredoc artifacts: no stray `EOF`, escapes or `$(`.
  - All its truncated digests match their full values.
  - Nothing else in the repository or the main checkout was written after 18:40. The only files
    newer than that are `244a52a0c`'s own (mtime 18:40:35, unchanged since) and this attempt's
    files. Scratch is confined to `/tmp/claude-1002/kb-1229/`.

## Gates rerun (non-timed)

All of these ran in `wt-bench` at `8808ff5a2`, and the tree was clean afterwards:

| Gate | Result |
|---|---|
| Native validator on the accepted file | exit 0 (also exit 0 on the raw file) |
| V8 validator | exit 0 |
| D2 build | fresh, exit 0 |
| `cargo test --locked --release -p audit -p bench -p console-workload` | 113 passed, 0 failed, 2 ignored |
| `cargo fmt --all -- --check` | 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 0 |
| `check-bench-policy.sh` and `test-bench-policy.sh` | ok; `timed_subjects` unchanged, nothing under `tools/bench/` |
| `check-workspace-policy.sh` and `test-workspace-policy.sh` | ok |
| `test-console-benchmark.sh` | PASS (0/0/0 real invocations) |

## NITs

- **NIT-1: `net_total` is described slightly wrongly.** `REPORT.md` calls `net_total` "the sum of
  every graph phase's net time".
  - It is actually the median over repeats of each repeat's sum.
  - So the printed per-line medians add up to 165066.3, not 165067.4.
  - The denominator is still one number, and no share moves at two decimals. One clause would fix
    the wording.
- **NIT-2: an uncited cause for the load.** The report says the wait hit its cap because "another
  project's process held a core". No capture supports that. It explains the load but is not
  evidence of its cause. Drop it or cite it.
- **NIT-3: the timed commands' argv was not captured.**
  - `timed.sh` does not log the command it runs, so the `flock` on both timed runs and on the
    profile, and `taskset -c 31` on the profile, rest on the report's word.
  - Indirect evidence agrees: the record's waiver string, `cpu_affinity` 31 and the binary's
    single access.
  - A future wrapper should `printf '%q ' "$@"` into the wait file.
- **NIT-4: one probe is never subtracted.** The probe correction subtracts 17 graph entries plus
  183 chain reads, but there are 201 reads per block. The finish probe, about 33 ns or 0.02 % of
  the block, is never subtracted. The model harness has the same gap, and it is negligible here.
