# Record the bus-and-send baseline and its route-work profile

Successor performance issue BM3, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is evidence only: no engine change, no
tuning, and no decision filed.

The design record cited below (`DESIGN.md`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A baseline record of what buses and sends cost as they ship, on the two benchmark paths:

- native AVX2, the static plan (the C ABI's shape while it attaches no live controls);
- the shipped module under V8, with live controls (every route into a submix is a live route).

Beside it, one untimed phase profile of the native row says how much of its block time the route
work takes. The numbers and `DESIGN.md` 6.3's guidance go to the weekly performance pass, which
decides whether route fusion (deferred item O1) earns a brief. Nothing is optimised, tuned or
filed here (`AGENTS.md:175-182`; the owner's rule that a fold or fusion is justified by a
measurement on a real host path).

## Context (verified on `main` at `1cb677a76`)

- **The rows.** Native `sixty_four_track_console_sends`, from *Add a bus-and-send row to the native
  console benchmark* (#1227, BM1), and the V8 document of the same kind, from *Add the bus-and-send
  session to the browser mixing benchmark* (#1228, BM2). Both render
  `fixtures/session/v1/console-sixty-four-track-sends.json`. The standing rows run in the same
  invocations and are the reference (`sixty_four_track_console` natively, the standing console
  document in V8).
- **BM1's facts test** prints, on one line, `route_transforms`, `reduction_nodes`,
  `bank_route_folds`, `route_ops_per_block` and `delayed_route_edges` for the native row's plan.
  Run it with `cargo test --locked --release -p console-workload --lib -- --nocapture <its name>`.
  This issue adds no census of its own.
- **The native runner** (`scripts/operator/run-console-benchmark.sh`).
  `bash scripts/operator/preflight-console-benchmark.sh --step NAME` checks everything that can fail
  without launching the workload (the runner's header, `:23-24`, says to run it first). The runner
  then:
  - refuses an existing record, a non-x86-64 or non-AVX2 host, and a tree that is not clean
    **including untracked files** (`:60-68`). These refusals come before the step directory is made
    (`:71`) and before the exit trap is set (`:129`), so they write nothing;
  - builds `bench` with frozen profile overrides (`CARGO_PROFILE_RELEASE_OPT_LEVEL=3`,
    `..._LTO=false`, `..._CODEGEN_UNITS=16`, `:140-142`);
  - pins to the highest online CPU (`bench_highest_cpu`, `:176`), runs one warmup and two measured
    rounds, validates with `jq -s -e -L scripts -f scripts/console-benchmark-validator.jq` (`:383`),
    and writes `console-benchmark.{raw,accepted}.jsonl`, `.stderr.log` and `.disposition.json` under
    `artifacts/steps/NAME/` (`:51-59`), plus `.core-clock.csv` only when `perf` counts (it cannot
    here, so the records carry no cycle columns);
  - refuses an uncontrolled host (load above the ceiling, a busy SMT sibling, no affinity) unless
    `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, in which case every record says `uncontrolled` and
    names what it waived; a host that meets every precondition is recorded `controlled` either way
    (`:252-264`). A refusal after the trap writes a FAIL disposition (`on_exit`, `:116-124`) and
    prints its reason on the terminal (`:259-262`); the runner never overwrites a disposition.
  - Its disposition's `runner_invocations` is a literal 1 (`:109`), so it cannot show a repeat.
- **The V8 runner** (`scripts/run-web-mixing-automation-benchmark.sh:15-40`): `prepare WORKDIR`
  (empty WORKDIR; writes only there), `preflight WORKDIR` (writes nothing), then
  `run WORKDIR --step NAME`, which makes the step directory only after its checks (`:147`) and
  writes `artifacts/steps/NAME/web-mixing-automation.jsonl` (or keeps a refused pair as
  `web-mixing-automation.refused.jsonl`) and `web-mixing-automation.stderr.log`. It requires
  unmodified **tracked** files only, refuses to overwrite its own three files, requires the module
  to have been prepared at HEAD, and has the same load waiver. The run's one file holds two round
  records.
- **`*.log` is gitignored** (`.gitignore:8`): the runners' stderr logs are not committed; S0
  committed none.
- **The precedent protocol** is S0's (`62c958a95`, #1086) and S4's (the evidence of #1099 at
  `4c5d0961f`): on a clean detached worktree of the measured commit, V8 `prepare` and `preflight`,
  then the native preflight, then the native run and the V8 run, each timed command under the
  operator's timing lock with the waiver set:

  ```bash
  LOCK=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock
  flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
      bash scripts/operator/run-console-benchmark.sh --step NAME
  flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
      bash scripts/run-web-mixing-automation-benchmark.sh run "$W" --step NAME
  ```

  S4 also waited, untimed and launching nothing, for a 1-minute loadavg below 0.45 (capped at
  900 s) before each timed command, and recorded `/proc/loadavg` before and after it. S0 committed
  `console-benchmark.{accepted,raw}.jsonl`, `console-benchmark.disposition.json` and
  `web-mixing-automation.jsonl` (`artifacts/steps/console-strip-base/`).
- **`perf` cannot run on the bench host.** `/proc/sys/kernel/perf_event_paranoid` is 4, recorded in
  `docs/handoffs/plumbing-floor-2026-09-26/PLAN.md:25` and
  `docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md:56`.
- **The attribution instrument** is `graph::test_only_phase_profile` (`crates/graph/src/lib.rs:36-207`),
  test-support only and absent from production builds:
  - phases (`:60-83`): `ENTER`, `SOURCE`, `BOUND`, `ROUTE` (a plain unit whose op is a `Route` or
    `LiveRoute`), `OUTPUT` (the session Output op's unit), `IDENTITY_COPY` (a one-input identity
    not in place), `IDENTITY_ALIAS`, `OTHER_OP` (every other plain unit), `BANK` and `EXIT`. The
    classification is `test_only_unit_phase` (`crates/graph/src/runtime.rs:2892-2914`): a submix
    input reduction (a multi-input identity) **and** a per-node effect op are both `OTHER_OP`;
  - `bank` re-exports `rack::test_only_bank_phase_profile` (`crates/rack/src/lib.rs:39-125`):
    sub-phases `PROLOGUE`, `GATHER`, `SLOT`.. by chain **position** (`SLOTS = 8`; a position where
    chains run different stage types is named `MIXED`), `SEAM`, `SCATTER`, `FOLD` (the members'
    route and master accumulation) and `AUX`;
  - `enable`, `reset`, `snapshot()` and `runs()` (the first profiled block's unit-kind runs, at most
    16) drive and read it. A probe is taken only where the unit kind changes.
  - `console-workload`'s dev-dependency on `graph` enables `test-support`
    (`tools/console-workload/Cargo.toml`, `[dev-dependencies]`); dev-dependencies apply to the
    crate's tests **and examples**, and `cargo test` builds examples without running them.
    `tools/bench` links neither, so neither changes the timed subject.
- **The model harness** is `tools/console-workload/tests/gain_pan_profile.rs` (#960): `phase_profile`
  (`:212`) builds `SessionRuntime::build(ROW, PlanConfig::BASELINE)`, renders 512 warmup blocks,
  then three repeats of 4000 blocks with the probes off and on, measures the in-situ probe cost,
  derives each phase's entry count from `runs()` (`:296-304`), and prints each phase and
  sub-phase net of the probes (`Repeat::net_phase`, `net_sub_phase`) with a `share` column, the
  line's net time over `net_total`, the sum of every phase's net time; it skips zero rows and
  asserts nothing about time. Two parts do not carry over to this row: it asserts
  `slots == 3 * chains` (`:231-235`), a gain/pan-strip fact this row breaks, and it labels slots
  for that strip.
- **Planner probe (on `1cb677a76`, not committed; a shape, not a number to quote).** On a draft of
  BM1's fixture (reproduced by the verifier):
  - `bank_shape()` was `[29, 67]`, and `runs()` used 15 of its 16 entries: `BOUND` 64, `BANK` 16,
    `ROUTE` 64, `BANK` 8, `OTHER` 1, `ROUTE` 64, `BANK` 1, `OTHER` 1, `ROUTE` 64, `OTHER` 8,
    `BANK` 2, `OTHER` 2, `BANK` 2, `ROUTE` 10, `OUTPUT` 1; about 18 graph probes per block;
  - the 12 `OTHER_OP` units were the 10 submix input reductions plus two per-node inserts
    (`fx-a`'s delay, which has no bank kernel, `crates/delay/src/lib.rs:287`, and `fx-b`'s
    one-lane EQ);
  - the 29 chains were three per 8-track bank (the `pre_fader` and `post_fader` taps split each
    track chain), one 8-lane chain of the buses, and two one-lane chains each for `fx-a` and `fx-b`
    around their per-node insert;
  - bank positions 0-2 were `MIXED`, 3-4 `EffectBankStage` and 5-6 `BuiltinStage`, each position
    summing different chains' and different effects' stages, so no console slot's share is
    attributable on this row;
  - `bank_route_folds()` was 0, so the `FOLD` sub-phase had no entries.
- **What fusion would touch** (`DESIGN.md` 10, O1): each undelayed route's buffer write and the
  reducer's re-read of it, plus the route op's dispatch; the mix arithmetic and the add stay. The
  shares below bound the route-attributable work. Nobody has ruled on O1, so no saving figure for
  it is stated anywhere (`AGENTS.md`: no projected win may be quoted for a change nobody has
  ruled on).
- **The live-controlled fold loss** (`DESIGN.md` R9, `:1078-1080`): in a live-controlled plan a
  route into a submix is a live route and never folds, so the V8 plan can fold at most the static
  plan's folds of routes into the output. On this fixture the static plan folds nothing, so R9's
  loss cannot show here. O5's trigger (`DESIGN.md:1300-1301`) needs a padded-bank time share, which
  this instrument cannot attribute (above). Both stay unevaluated by this issue.
- **Which shape the C ABI runs.** While #1053 and #1225 are open, `prepare_runtime`
  (`crates/capi/src/runtime/compile.rs:412-421`) calls host-core's `prepare_host_runtime`, which
  attaches no live controls and no meters (`Concurrent` builtins): the native row's shape.

## Decisions frozen for this issue

- **D1. The timed runs, once, on a recorded commit, S0's way.**
  - **The commit.** A commit that contains BM1 and BM2 (each with a Sol PASS) and nothing of this
    issue: the profile example must not exist in it. Record its hash in the report.
  - **The tree.** A clean detached worktree of that commit, outside the batch worktree
    (`git -C /home/bl/misofm/engine worktree add --detach <WT> <commit>`), so no other file, tracked
    or untracked, is under the runners. A fresh empty WORKDIR from `mktemp -d` outside `<WT>`.
  - **The order, in `<WT>`** (S0's and S4's):
    1. `bash scripts/run-web-mixing-automation-benchmark.sh prepare "$W"`;
    2. `bash scripts/run-web-mixing-automation-benchmark.sh preflight "$W"`;
    3. `bash scripts/operator/preflight-console-benchmark.sh --step bus-send-base`;
    4. exactly one native run, under the lock with the waiver (Context);
    5. exactly one V8 run, under the lock with the waiver (Context).
  - **Around each timed command,** as S4: an untimed wait that launches nothing until the 1-minute
    loadavg is below 0.45, capped at 900 s; `/proc/loadavg` recorded before and after. Every
    command's terminal output (stdout and stderr) is captured to a file outside `<WT>`
    (`cmd 2>&1 | tee <scratch>/<n>-<name>.txt`), for the report.
  - **Afterwards,** copy `<WT>/artifacts/steps/bus-send-base/` into the issue branch, and remove
    `<WT>` once nothing in it is needed (`git worktree remove`).
  - **What counts as the one invocation.** The first run of a path that launches its workload is
    that path's one invocation, whatever its outcome; a path that has launched is never restarted.
  - **Refusals before a launch measured nothing.** Quote the captured terminal output (and, if the
    runner wrote them, its FAIL disposition and stderr log) in the report. Then:
    - steps 1-3 or an early native refusal (`:60-68`, which writes nothing): fix the cause and
      continue;
    - a native refusal after the trap whose disposition says `workload_process_launches: 0`: the
      step directory then holds only that disposition and `console-benchmark.stderr.log`; delete
      exactly that directory, fix the cause, and run step 4 again;
    - a V8 `run` refusal before its warmup: delete **nothing** (the step directory holds the native
      records); fix the cause and run step 5 again.
    - If a fix needs a code change: before step 4 has launched, stop and restart D1 on the commit
      that fixes it; after it, the native records stand, the V8 run happens once on the fixing
      commit, and the report names both commits.
- **D2. The profile, untimed and once.** A new example
  `tools/console-workload/examples/bus_send_profile.rs` (an example, not a `#[test]`: it is an
  instrument that gates nothing, so it claims no test value) drives `graph::test_only_phase_profile`
  as `gain_pan_profile.rs`'s `phase_profile` does, on
  `SessionRuntime::build(Workload::SixtyFourTrackConsoleSends, PlanConfig::BASELINE)`: 512 warmup
  blocks, then three repeats of 4000 blocks with the probes off and on, and the in-situ probe cost.
  It may copy `gain_pan_profile.rs`'s helpers, but:
  - it does **not** copy the `slots == 3 * chains` assertion or the gain/pan slot labels; it prints
    `bank_shape()` and `bank::slot_names()` instead;
  - it prints **every** phase and every bank sub-phase, zero rows included, each as net nanoseconds
    per block and its share of `net_total` (the "profiled block time" everywhere in this issue),
    as the per-line median of the three repeats;
  - it prints the clock-read cost, the in-situ probe cost, the probes per block, the full
    `profile::runs()` list and how many of its 16 entries were used, and the plan census of D5 (from
    `SessionRuntime::unit_eligibility()`, `bank_shape()` and `bank_route_folds()`);
  - it asserts nothing about time.

  Build it with the native runner's profile overrides, so it is the timed subject's code:
  `CARGO_PROFILE_RELEASE_OPT_LEVEL=3 CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_INCREMENTAL=0 cargo build --locked --release -p console-workload --example bus_send_profile`.
  Run it once, after the timed runs, under the same lock, pinned to the CPU the native records name
  (`cpu_affinity`; if that is `uncontrolled`, run it unpinned and say so):
  `flock -w 7200 "$LOCK" taskset -c <cpu> target/release/examples/bus_send_profile >artifacts/steps/bus-send-base/bus-send-profile.txt`.
  A run that fails before printing any measurement (a panic, a build error) measured nothing: quote
  the failure, fix the example, and run it once more. Once it has printed measurements it is never
  re-run. If all 16 `runs()` entries are used, the runs-derived entry counts, and so the net
  figures, may be truncated: the report says so and reads the probed (not net) figures. No `perf`,
  and no `perf.data` anywhere.
- **D3. The attribution.** The report states these shares of the profiled block time, each read
  from a printed line:
  - (a) route ops: `ROUTE`;
  - (b) route-input reductions: `OUTPUT` + `IDENTITY_COPY` + `OTHER_OP`, stated as an **upper
    bound**, because `OTHER_OP` also holds every insert that renders per node. Beside it, the
    `OTHER_OP` unit count from `runs()` and how many of them are reductions (BM1's facts: 11
    reduction nodes, of which `main-out`'s is the `OUTPUT` unit);
  - (c) the bank `FOLD` sub-phase (folded route lanes; BM1's facts give the fold count; a zero line
    is a valid reading).

  The profile is never compared with the timed records, and is never re-run for a "better" share.
- **D4. No decision here.** The report applies `DESIGN.md` 6.3's guidance descriptively, as one of:
  (a) + (b) below about 5 % of the profiled block time; (a) alone at or above about 5 %; or, when
  neither holds, inconclusive within (b)'s bound. It states no saving figure for O1, files no
  fusion spec and no optimisation issue. The handoff to the weekly pass is one evidence section
  appended to the umbrella spec of *Submix strips and live aux sends* (#1196), naming the report as
  the pass's input; if that spec has left `.github/ISSUE_SPECS/`, the section goes in this spec and
  a comment on #1196.
- **D5. Side reports**, descriptive only:
  - **R9:** the static plan's fold count (BM1's facts) beside both rows' p50, the bound it puts on
    the V8 plan's folds, and the sentence that R9 stays unevaluated on this fixture (Context).
  - **O5:** the submix bank census: for each bank chain whose lanes are submixes, its lanes, its
    active-lane count and the bank width (`lane::Backend::current()`), so the pad lanes; no time
    share, and the sentence that O5's trigger stays unevaluated (Context).
  - **The C ABI shape at measurement time:** static or live-controlled, from the state of #1053 and
    #1225 on the D1 commit.

## Deliverables

1. Under `artifacts/steps/bus-send-base/`: the files S0 committed
   (`console-benchmark.{accepted,raw}.jsonl`, `console-benchmark.disposition.json`,
   `web-mixing-automation.jsonl`), plus, if a runner refused after launching its workload, what it
   kept (a FAIL disposition, `web-mixing-automation.refused.jsonl`) as evidence. Stderr logs are
   gitignored: quote them in the report instead of force-adding them.
2. `tools/console-workload/examples/bus_send_profile.rs` (D2) and its saved output
   `artifacts/steps/bus-send-base/bus-send-profile.txt`.
3. `artifacts/steps/bus-send-base/REPORT.md`. Every number cites a record field, a facts-test value
   or a profile line. It holds:
   - the D1 commit (and the V8 commit, if D1 needed a second), and every runner invocation in
     order, refusals included, with its captured terminal output quoted or summarised;
   - the host, admissibility and loadavg the records and the waits state;
   - per row, p50, p95 and p99 for each measured round: the native sends and console rows, and the
     V8 sends and console documents;
   - BM1's facts line;
   - the shares (a), (b) and (c), with the probe cost, the probes per block and the `runs()` usage
     beside them;
   - D4's reading and D5.
4. The umbrella spec's evidence section (D4).

## Authorized paths

- `tools/console-workload/examples/bus_send_profile.rs` (new)
- `artifacts/steps/bus-send-base/` (new)
- `.github/ISSUE_SPECS/1196-submix-strips-and-live-aux-sends.md` (one appended evidence section)
- this spec

## Non-goals

- No engine change, no tuning, and no change to `tools/console-workload/src/`, `tools/bench/` or
  any runner or validator.
- No second timed invocation of a path after it launched, for any reason.
- No new benchmark row and no diagnostic row.
- No fusion spec, optimisation issue or class-B order change; Q5 stays an owner question.
- No projected saving: shares are measured, never extrapolated, and no saving figure is stated for
  O1.
- No `perf`, and no binary profile committed.

## Hazards

- **The profile is not a timing.** It is never used to argue a p50, and once it has printed
  measurements it is never repeated.
- **Probe placement.** A probe is taken where the unit kind changes, and this row changes kind
  often. Report the probe count and in-situ cost beside the shares, and use the net figures.
- **Order and tree.** The example must not exist in the measured commit or tree. Use the detached
  worktree: the batch worktree's own untracked files would refuse the native run (`:65`).
- **Shared host.** Other agents build on this host. Every timed command and the profile run take
  the operator's timing lock; the waiver keeps a busy moment from refusing the run, and the
  records say whether it was quiet.
- **A tooling failure after the timed rounds** (a validator refusal, a profile panic after it
  printed): keep the raw output, record the failure, and move the repair to a tooling issue.
  Never re-run a timed path.

## Objective gates

1. **The records are accepted.** The native disposition says `"status":"PASS"` and
   `"measured_rounds_completed":2`, and the accepted file equals the raw one; the V8 run wrote
   `web-mixing-automation.jsonl` and no refused file. Re-check both on the committed files:
   `jq -s -e -L scripts -f scripts/console-benchmark-validator.jq artifacts/steps/bus-send-base/console-benchmark.accepted.jsonl`
   and
   `jq -s -e -L scripts -f scripts/web-mixing-automation-validator.jq artifacts/steps/bus-send-base/web-mixing-automation.jsonl`.
2. **One timed invocation per path.** The report's invocation list shows one launched native run
   and one launched V8 run, and every other invocation as a refusal before launch; the step
   directory holds one native record set and one V8 file; nothing was overwritten.
3. **The profile example builds and is untimed.** D2's build command succeeds;
   `cargo test --locked --release -p audit -p bench -p console-workload` passes (it builds the
   example and runs none of it).
4. **The report is traceable.** Every number in `REPORT.md` cites a record field, a facts-test
   value or a profile line, and the report names the D1 commit and the C ABI shape.
5. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
     (`timed_subjects` unchanged)
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`

## Evidence

- The step directory and `REPORT.md`.
- The machine metadata the records already carry.
- The D1 commit hash.

### Attempt 1 record (Terra, records on `244a52a0c`)

`artifacts/steps/bus-send-base/REPORT.md` holds the full record; this is its summary.

- **D1 commit `244a52a0c7c9ac1c64064b94265079a017c0aab9`** (BM1 and BM2, both Sol PASS; no
  `bus_send_profile.rs`). Clean detached worktree `/home/bl/misofm/wt-1229-d1`, fresh WORKDIR outside
  it; removed after the copy. Invocations in order: V8 `prepare` (exit 0), V8 `preflight` (exit 0),
  native preflight (PASS, workload launches 0), **one native run** (PASS, 3 launches, 2 measured
  rounds), **one V8 run** (exit 0, no refused file). No refusal, nothing deleted or re-run.
- **Load.** Native pre-run wait hit the 900 s cap (loadavg `1.13` before, `1.74` after); records say
  `uncontrolled` (waived `loadavg_above_ceiling` only; affinity cpu 31, SMT sibling idle, 60 s
  cooldown). V8 waited 501 s (loadavg `0.44`); records say `controlled`.
- **Native p50/p95/p99 ns per block**, rounds 1 / 2: sends `160926/167158/171116` /
  `160084/167098/170825`; standing console `100421/105871/107234` / `100151/105601/108175`.
- **V8 documents p50/p95/p99 ns**, rounds 1 / 2: sends `336606/349942/356335` /
  `345253/373797/496082`; standing console `231845/243076/249459` / `239279/262023/333721`.
- **Facts:** `route_transforms=202 reduction_nodes=11 bank_route_folds=0 route_ops_per_block=202
  delayed_route_edges=0`.
- **Profile** (one run, pinned to cpu 31 under the lock, after the timed runs): probe 33.1 ns in
  situ, 201 clock reads/block (18 graph), `runs()` 15 of 16 entries (not truncated); profiled block
  time `net_total` 165067.4 ns. Shares: **(a) route 8.08 %**; **(b) <= 3.83 %** (`output` 0.17 % +
  `identity-copy` 0 + `other` 3.66 %; 12 `OTHER_OP` units, 10 of them reductions); **(c) fold
  0.00 %** (0 entries). D4 reading: (a) alone at or above about 5 %. No decision filed.
- **D5:** R9 and O5 unevaluated as the Context predicts (0 static folds; 5 submix bank chains
  carrying 28 pad lanes at width 8, no time attributable). C ABI shape: static.
- **Gates.** 1: native disposition PASS with 2 measured rounds, accepted == raw, both validators
  accept the committed files. 2: one launched invocation per path. 3: D2 build ok; `cargo test
  --locked --release -p audit -p bench -p console-workload` 113 passed, 0 failed, 2 ignored. 5:
  `cargo fmt --check`, workspace clippy `-D warnings`, `check-bench-policy.sh`,
  `test-bench-policy.sh` (`timed_subjects` unchanged: nothing under `tools/bench/`),
  `check-workspace-policy.sh`, `test-workspace-policy.sh` and `test-console-benchmark.sh` all
  pass.
- **Test value.** No test was added or changed. The example is an instrument that gates nothing
  (D2), so it claims no test value and has no mutation.
- **Deviations.** The profile run's stderr went to a scratch file (it was empty). The spec's
  command redirected only stdout. The facts test ran in the batch worktree before timing (the same
  library code as the D1 commit), not inside the D1 worktree.

### Verdict follow-up record (after the attempt 1 PASS verdict)

No number changed, and nothing was timed or re-run.

- **NIT-1.** `REPORT.md`'s denominator now says what `net_total` is: the median over the three
  repeats of each repeat's sum of graph-phase net times, so the printed per-line medians add to
  165066.3 ns, not 165067.4 ns. Every share is of the one figure and none moves.
- **NIT-2.** `REPORT.md` no longer names a cause for the native wait hitting its cap: the cause was
  not captured, and the one recorded fact is loadavg `1.13` at the 900 s cap.
- **NIT-3.** The `flock` on both timed runs and the profile, and the profile's `taskset -c 31`, are
  attested by the report plus indirect record evidence only (the waiver string, `cpu_affinity` 31,
  the profile binary's single access), because the wait wrapper did not log its own command.
- **NIT-4.** The probe correction leaves one probe per block uncorrected (the finish probe, about
  33 ns, about 0.02 % of the profiled block time), which moves no share.

## Verdict

- **Attempt 1** (`d1d738e55` + `8808ff5a2`, records on `244a52a0c`): Sol PASS. No BLOCKER, MAJOR
  or MINOR; four NITs, recorded above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1229-attempt1.md`.

## Dependencies

- *Add the bus-and-send session to the browser mixing benchmark* (#1228, BM2), which itself follows *Add a
  bus-and-send row to the native console benchmark* (#1227, BM1)

## Standing rules for the implementer

- Freeze everything before timing. Run each path exactly once, with one warmup and two measured
  rounds. Do not tune or retry.
- If post-workload tooling fails, preserve the raw output, record the failure, and move the repair to
  a tooling issue.
- Do not quote projected savings.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main` (or BM2's batch branch), after the timed runs.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
