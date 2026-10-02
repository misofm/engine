# Record the bus-and-send baseline and its route-work profile

Successor performance issue BM3, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is evidence only: no engine change, no tuning,
and no decision filed.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

One baseline per benchmark path records what buses and sends cost as they ship:

- native AVX2, the static plan (the C ABI fan-playback shape, until the C ABI attaches live
  controls);
- the shipped module under V8, with live controls (the producer-mixer shape).

One untimed phase profile of the native row says how much of its block time the route work takes.
The numbers, with `DESIGN.md` 6.3's guidance, go to the weekly performance pass, which decides
whether route fusion (deferred item O1) earns a brief. Nothing is optimised or filed here (the
owner's rule: a fold or fusion is justified by a measurement on a real host path, and systematic
optimisation is the weekly pass's).

## Context (verified on `fe8ac679`)

- **The rows.** Native `sixty_four_track_console_sends`, from *Add a bus-and-send row to the native
  console benchmark* (#1227, BM1), and the V8 sends document, from *Add the bus-and-send session to the
  browser mixing benchmark* (#1228, BM2); both render
  `fixtures/session/v1/console-sixty-four-track-sends.json`. The standing rows run in the same
  invocations and give the reference.
- **The runners.**
  - Native: `bash scripts/operator/preflight-console-benchmark.sh --step NAME` checks everything
    that can fail without launching the workload (`scripts/operator/run-console-benchmark.sh:23-24`
    says to run it first). Then `bash scripts/operator/run-console-benchmark.sh --step NAME` pins to
    the highest online CPU (`bench_highest_cpu`, `:176`), runs one warmup and two rounds, validates,
    and writes `console-benchmark.{raw,accepted}.jsonl`, `.stderr.log`, `.disposition.json` and
    `.core-clock.csv` under `artifacts/steps/NAME/` (`:50-61`). It refuses to overwrite any of them.
  - V8: `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then
    `preflight WORKDIR`, then `run WORKDIR --step NAME`, which writes
    `artifacts/steps/NAME/web-mixing-automation.jsonl`, or keeps a refused pair as
    `web-mixing-automation.refused.jsonl` (`scripts/run-web-mixing-automation-benchmark.sh:13-22`).
    `prepare` refuses a non-empty WORKDIR and a dirty tree.
  - The precedent is `artifacts/steps/console-strip-base/` (S0, #1086), which holds
    `console-benchmark.{accepted,raw}.jsonl`, `console-benchmark.disposition.json` and
    `web-mixing-automation.jsonl`.
- **`perf` cannot run on the bench host.** `/proc/sys/kernel/perf_event_paranoid` is 4, and the repo
  has recorded it twice (`docs/handoffs/plumbing-floor-2026-09-26/PLAN.md:25`,
  `docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md:56`). A profile that cannot run would fix
  any decision before anything is measured.
- **The supported attribution instrument** is `graph::test_only_phase_profile`
  (`crates/graph/src/lib.rs:35-206`), a test-support-only module absent from production builds:
  - phases (`:60-83`): `ENTER`, `SOURCE`, `BOUND`, `ROUTE` (plain units whose op is a route),
    `OUTPUT` (the session Output op), `IDENTITY_COPY` (a one-input identity not in place:
    `reduce_plane`'s copy arm), `IDENTITY_ALIAS`, `OTHER_OP` (any other plain unit, which includes a
    bus `Input`'s multi-input reduction), `BANK` and `EXIT`;
  - `bank` re-exports `rack::test_only_bank_phase_profile` (`crates/rack/src/lib.rs:39`), whose
    sub-phases are `PROLOGUE`, `GATHER`, `SLOT`.., `SEAM`, `SCATTER`, `FOLD` (the route and master
    accumulation the members perform) and `AUX`;
  - `enable`, `reset`, `snapshot()` and `runs()` drive and read it.
  - `tools/console-workload/tests/gain_pan_profile.rs` (#960) is the model harness: its
    `phase_profile` test (`:212`) builds `SessionRuntime::build(ROW, PlanConfig::BASELINE)`, warms
    up, profiles 4000 blocks three times, prints each phase and sub-phase in derived cycles, and
    asserts nothing about time. It is ignored by default and run pinned:

    ```
    CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test gain_pan_profile --no-run
    taskset -c <bench cpu> target/release/deps/gain_pan_profile-<hash> --ignored --nocapture \
        --test-threads 1 phase_profile
    ```

    `console-workload`'s dev-dependency on `graph` enables `test-support`
    (`tools/console-workload/Cargo.toml`, `[dev-dependencies]`), so the bank sub-phases exist in its
    tests. `tools/bench` does not link `console-workload`'s tests, so a new test file does not change
    the timed subject.
- **Plan facts** (route ops per block, delayed route edges, folds) are stated by BM1's facts test;
  this issue adds no census.
- **What fusion would remove** (`DESIGN.md` 10, O1): each undelayed route's buffer write and the
  reducer's re-read of it, plus the route op's dispatch. The mix arithmetic and the add stay. So the
  route work's share is an upper bound on fusion's saving.
- **The live-controlled fold loss** (`DESIGN.md` R9): in a live-controlled plan a route into a submix
  is a bound live route and never folds, so the V8 document's folds are the static plan's folds of
  routes into the output.
- **Which shape the C ABI runs** depends on #1053 and *Deliver value-only send and submix-strip edits
  to the running C ABI plan* (#1225): before both land, the C ABI prepares with no live controls
  (`crates/capi/src/runtime/compile.rs:408-410`), the native row's shape; after, it prepares with live
  controls, whose sends are live routes like the V8 document's.
- **Benchmark rules** (`AGENTS.md:172-179`): freeze the workload and validator before timing; exactly
  one invocation per path, one warmup and two measured rounds; no tuning and no retry; if
  post-workload tooling fails, preserve the raw output, record the failure and move the repair to a
  tooling issue; do not optimise merely to improve a descriptive number.

## Decisions frozen for this issue

- **D1. Timed runs first, on a recorded commit.** On the commit that has BM1, BM2 and batch K3 on
  `main`, with nothing
  else changed and **before** this issue's profile test file is committed:
  1. `bash scripts/operator/preflight-console-benchmark.sh --step bus-send-base`;
  2. exactly one `bash scripts/operator/run-console-benchmark.sh --step bus-send-base`;
  3. exactly one V8 sequence: `prepare <WORKDIR>`, `preflight <WORKDIR>`, then
     `run <WORKDIR> --step bus-send-base`.

  Record that commit's hash in the report.
- **D2. The profile, untimed and once.** A new test `tools/console-workload/tests/bus_send_profile.rs`,
  modelled on `gain_pan_profile.rs`'s `phase_profile`, ignored by default, profiles the native sends
  row (`SessionRuntime::build(Workload::SixtyFourTrackConsoleSends, PlanConfig::BASELINE)`) and
  prints, per phase and per bank sub-phase, nanoseconds and derived cycles per block and their share
  of the profiled block time. It asserts nothing about time. It is run once, pinned to the bench CPU,
  after the timed runs; its output is saved as `artifacts/steps/bus-send-base/bus-send-profile.txt`.
  No `perf`, and no `perf.data` anywhere.
- **D3. The attribution.** The report states these shares, each read from a profile line:
  - (a) route ops: the `ROUTE` phase;
  - (b) route-input reductions: `OUTPUT`, `OTHER_OP` and `IDENTITY_COPY` (on this row every reduction's
    inputs are route buffers: the bus inputs, the return inputs and the output);
  - (c) the bank `FOLD` sub-phase (folded route lanes);
  - the console-bank effect slots' share, for deferred item O5.

  The profile is never compared with the timed records, and it is never re-run for a "better" share.
- **D4. No decision here.** The report applies `DESIGN.md` 6.3's guidance descriptively: whether
  (a) + (b) is below or at least about 5 % of the native row's profiled block time. It files no fusion
  spec and no optimisation issue. The handoff to the weekly performance pass is one evidence
  paragraph in the umbrella spec of *Submix strips and live aux sends* (or, if that spec has left
  `.github/ISSUE_SPECS/`, BM3's own spec and an issue comment on the umbrella) naming `REPORT.md` as
  the pass's input (`AGENTS.md:177-179`: the weekly pass decides whether to open an optimisation issue).
- **D5. Side reports**, descriptive only: for O5, the pad-lane fraction per console slot times that
  slot's share; for R9, the static and live-plan fold counts beside the V8 and native numbers; and
  which shape the C ABI ran at measurement time (Context).

## Deliverables

1. The native and V8 records under `artifacts/steps/bus-send-base/`, as `console-strip-base/` holds
   them, plus the stderr log and core-clock file the native runner writes.
2. `tools/console-workload/tests/bus_send_profile.rs` (D2) and its saved output.
3. `artifacts/steps/bus-send-base/REPORT.md`. Every number cites a record field, a facts-test value or
   a profile line. It contains: the commit of D1; per row, p50 and the existing percentiles; the
   facts; the shares (a), (b), (c) and the console banks; D4's descriptive reading; and D5.
4. The umbrella spec's evidence paragraph (D4).

## Authorized paths

- `tools/console-workload/tests/bus_send_profile.rs` (new)
- `artifacts/steps/bus-send-base/` (new)
- the umbrella spec of *Submix strips and live aux sends* in `.github/ISSUE_SPECS/` (its evidence
  section only)
- this spec

## Non-goals

- No engine change, no tuning, and no change to `tools/console-workload/src/` or `tools/bench/`.
- No second timed invocation of either path, for any reason.
- No new benchmark row and no diagnostic row.
- No fusion spec, optimisation issue or class-B order change; Q5 stays an owner question.
- No projected saving: shares are measured, never extrapolated.
- No `perf`, and no binary profile committed.

## Hazards

- **The profile is not a timing.** It must not be used to argue a p50, and must not be repeated.
- **Probe placement.** The phase profile takes a probe only where the unit kind changes; read its
  probe count and clock cost (printed, as `gain_pan_profile.rs` prints them) beside the shares.
- **Order.** Committing the profile test before the timed runs would put a changed tree under the
  records. D1 runs first.
- **A tooling failure after the timed rounds** (validator or profile) keeps the raw output. Record
  the failure and move the repair to a tooling issue; never re-run.

## Objective gates

1. **The records are accepted.** The native runner's validation (`console-benchmark-validator.jq`)
   and the V8 runner's (`web-mixing-automation-validator.jq`) accepted their records, and the raw
   output is preserved beside them.
2. **One timed invocation per path.** The step directory holds exactly one native record set and one
   V8 record; no refused file was replaced and nothing was overwritten.
3. **The profile test builds and is untimed.** `cargo test --locked --release -p console-workload
   --test bus_send_profile --no-run` builds; the default run of
   `cargo test --locked --release -p audit -p bench -p console-workload` skips it (ignored) and
   passes.
   *Test value: none claimed; the profile is a descriptive instrument, ignored by default, that turns
   no gate red. It is evidence, not a test (`AGENTS.md` test-value rule).*
4. **The report is traceable.** Every number in `REPORT.md` cites a record field, a facts-test value
   or a profile line, and the report names the D1 commit and the C ABI shape.
5. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh` (`timed_subjects`
     unchanged)

## Evidence

- The step directory and `REPORT.md`.
- The machine metadata the records already carry.
- The D1 commit hash.

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
- Commit on its own branch from synchronized `main`, after the timed runs.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
