# Add the console-strip benchmark rows

Slice B0 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's verification
`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

## Problem

The console strip changes which tracks bank and what a bypassed or silent lane costs. The
baseline has to be frozen before any engine slice lands, and the standing console benchmark does
not yet have the shapes that move:

- Remainders. Today only the 64-track (eight full banks) and the ragged 9-track (a remainder of one)
  rows exist. Padding (P2a-P2e, S2) changes the cost of every remainder, so the remainder sizes that
  matter need rows.
- The app. The app compiles EQ -> compressor onto every track and marks unselected tracks `bypass`
  (Sol's M8). Under the console strip a bypassed lane stays in its bank and runs the wet path
  (decision 12, "Bypass"), so the app shape must be measured before and after.
- Silence. The silent fast path is bank-wide, so one active track keeps its bank-mates processing
  (M4). The owner accepted that trade-off on condition that a sparse-activity row measures it.
- The validator pins `strip_layout` in rack tokens (`simd1:eq+compressor,simd2:limiter`,
  `scripts/console-benchmark-record-lib.jq`), which S1a retires.

## Smallest closable slice

Add rows to the existing native console benchmark and the existing V8 benchmark. Add no new timed
subject: the `timed_subjects` ratchet counts files (`scripts/check-bench-policy.sh:193`).

Authorized paths:
- `tools/console-workload/**`;
- `tools/bench/src/console.rs`;
- `scripts/console-benchmark-record-lib.jq`, `scripts/console-benchmark-record-validator.jq`,
  `scripts/console-benchmark-validator.jq` and `scripts/test-console-benchmark.sh`;
- `scripts/operator/preflight-console-benchmark.sh` and `scripts/operator/run-console-benchmark.sh`,
  only where the record count or row list is spelled;
- `scripts/web-mixing-automation-benchmark.mjs` and `scripts/run-web-mixing-automation-benchmark.sh`,
  for the two V8 documents;
- this spec.

Rows (native, Simd8, 48 kHz, 128-frame quantum, like the existing console rows):

1. **Strip at N tracks.** EQ -> compressor -> limiter (today's `console-sixty-four-track-intended`
   layout) at N in {9, 10, 13, 16, 64}. 9 (the ragged row, remainder 1) and 64 exist. Add 10
   (remainder 2), 13 (remainder 5) and 16 (two full banks). Derive them in `tools/console-workload`
   from the committed 64-track intended fixture by taking the first N tracks, as the ragged row
   does. Commit no new fixture document.
2. **App shape.** N = 64. Every track carries EQ -> compressor in `dynamic`. The EQ and the
   compressor are bypassed on every track whose index is 2 mod 3 (21 tracks, about one third).
   The pattern is fixed and deterministic.
3. **Sparse activity.** N = 64, the full strip, every second track fed silence. No bank is then
   wholly silent, which is the case the bank-wide skip cannot serve. The report reads it against
   the all-active row and the existing `sixty_four_track_idle` row.

`strip_layout` becomes layout-neutral now, so S4 needs no validator rewrite. It names the chain in
the console vocabulary through decision 12's lowering (`simd1` -> `pre_insert`, `dynamic` ->
`inserts`, `simd2` -> `post_insert`). Today's intended strip is therefore
`pre_insert:eq+compressor,post_insert:limiter`, and the app shape is `inserts:eq+compressor` with a
separate field recording the bypass pattern. The validator accepts, for the app-shape kind, both
`inserts:eq+compressor` (before) and `pre_insert:eq+compressor` (after, once the app shape migrates
to the console). Existing rows' layouts are rewritten by the same mapping.

V8: add the N = 64 strip and the app shape as documents of the web mixing benchmark on the shipped
artifact. Add no per-N V8 rows, because that would be a second framework.

## Dependencies

None beyond decision 12 and the AGENTS.md amendment, which landed with R0.

Merge order: B0 must merge before any engine slice (P1, P2a, S1a), so that S0 can time B0's commit.

## Objective gates

1. `scripts/operator/preflight-console-benchmark.sh --step console-strip-base` passes without
   launching the timed workload.
2. `scripts/test-console-benchmark.sh` passes, with its record counts updated for the new rows.
   Its mutation cases cover the new `strip_layout` spellings and the app-shape bypass field.
3. `scripts/check-bench-policy.sh` passes, and `timed_subjects` is unchanged.
4. Each new row renders deterministic output across the two rounds. The row's `output_sha256` is
   checked by the existing validator rule, not by a new pinned digest.
5. No timed run happens in this slice.

## Non-goals

No engine change. No tuning of existing rows. No isolates for per-node against padded cost; H5's
figures stay arithmetic until S4 reports.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint. Commit on the batch branch in
  CI-conscious batch mode.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").

## Attempt 1 evidence

Terra, attempt 1, on `codex/1085-console-benchmark-rows` from `6fdf5db2`. Implementation commit
`e1b0fed3`. No timed run was made.

### What was added

Five native session rows, `CONSOLE_STRIP_WORKLOADS` in `tools/console-workload/src/lib.rs`, emitted
after the metered row (outside `WORKLOADS`, the wasm arm's address space):

| Kind | Tracks | Fixture | Synthetic | `strip_layout` | `input_signal` |
|---|---|---|---|---|---|
| `ten_track_ragged_strip` | 10 | intended | yes | `pre_insert:eq+compressor,post_insert:limiter` | `tone` |
| `thirteen_track_ragged_strip` | 13 | intended | yes | same | `tone` |
| `sixteen_track_strip` | 16 | intended | yes | same | `tone` |
| `sixty_four_track_app_shape` | 64 | app (new) | no | `inserts:eq+compressor` | `tone` |
| `sixty_four_track_console_sparse` | 64 | intended | yes | intended | `odd_tracks_silent` |

- The strip-at-N rows take the intended fixture's first N tracks through `synthesise_tracks`, as the
  ragged nine-track row does. With `nine_track_ragged_strip` and `sixty_four_track_console` they are
  N in {9, 10, 13, 16, 64}.
- The app shape is a committed document, `fixtures/session/v1/console-sixty-four-track-app.json`.
  It is derived by the new `scripts/derive-app-console-fixture.py` on the existing derive pattern:
  each track's exact EQ and compressor objects move from `simd1` to `dynamic`, the limiter is
  dropped, and both effects are bypassed on tracks with index 2 mod 3 (21 of 64).
  `scripts/check-console-fixtures.sh` regenerates it, compares it byte for byte and checks its
  witness. It is a document rather than a strip edit because the V8 arm must boot the bytes the
  native row renders. Its record adds a bypass group, `bypass_pattern: "index_mod_3_is_2"` and
  `bypassed_tracks: 21`. The group is read from the compiled session (`SessionRuntime::bypass_census`)
  and is pinned by `bypass_session_shape`.
- The sparse row feeds exact zeros to every odd track through a per-track `InputSignal`. A new test
  reads the plan's lane table and shows that every bank holds active and silent lanes. The run
  asserts, and the aggregate validator requires, that its digest differs from both the all-active
  row and the idle row.
- V8: the web mixing benchmark times two documents after its three arms, alternated per
  observation: the intended console and the app shape. The record states them under `documents`,
  with facts, percentiles and digest. Their facts come from the `mixing_automation_controls`
  example (native `Workload` methods and the bypass census), not from a transcription. The harness
  checks each fixture's per-track layout and bypass against those facts before it boots anything.
  `web-mixing-automation-lib.jq` pins the documents and requires both rounds to agree on them,
  timings aside. No per-N V8 rows.

**`strip_layout` is layout-neutral.** Every row was rewritten by decision 12's lowering
(`simd1` -> `pre_insert`, `dynamic` -> `inserts`, `simd2` -> `post_insert`). The placement,
automation and mixing records follow, because they print `Workload::strip_layout`. The rack-token
spellings are now refused. For the app-shape kind alone, native and V8 accept both
`inserts:eq+compressor` and `pre_insert:eq+compressor`.

**Floors.** The five rows are `not_derived`, with no control (`floor.rs`, `floor_pins`):
- a remainder's width factor depends on whether it renders per node or padded, and the nine-track
  factor holds only for a remainder of one;
- bypassed and silent lanes have no inventory;
- H5 stays arithmetic until S4.

**Counts.** 60 records (44 session). Updated in the aggregate validator, the mutation suite's
index map, `run-console-benchmark.sh` and the preflight's `records_required`. `timed_subjects` is
unchanged (1).

**Paths outside the brief's list, with reasons.**
- `tools/bench/src/floor.rs`: its `floor_row` match is exhaustive, and the Rust/jq floor-parity
  test iterates every row.
- The app fixture, its derive script and `check-console-fixtures.sh`: root's instruction for new
  fixtures.
- `scripts/web-mixing-automation-lib.jq`: the V8 record's validator.

### Digests

Existing rows are unchanged. All console-workload pins pass, including `chain_shape.rs`'s four and
the console row's `fe5bed9b...`. New 64-block pins are in
`the_console_strip_rows_render_their_pinned_bits`. They hold in both release and debug:

| Row | 64-block digest |
|---|---|
| `ten_track_ragged_strip` | `1eed6377a0b5c773fb35d979f76600d4ad2403796a4fdbc1ce481e11742a4fe3` |
| `thirteen_track_ragged_strip` | `17b2451b59f71b6504cb905cafb11742edcecd5be10dd1f86759541bb201bb12` |
| `sixteen_track_strip` | `5e1b97df9b46df231895344b4da03c02a8ff9e6cead10f356b001b9c2144c293` |
| `sixty_four_track_app_shape` | `c740fa2dd904d26dfc104ff4b202b81454757b8428c54e83092d93fe441bd4c3` |
| `sixty_four_track_console_sparse` | `1f18f156705313c098e2e8310a848f63eeb23085c971239991dffd2b4a8de334` |

Every console slice is class A per lane, so these pins are the bits S4's rows must render again.

### Gates

- `scripts/operator/preflight-console-benchmark.sh --step console-strip-base`: PASS,
  `workload_launches: 0`, `records_required: 60`. The two aborts it logs are its own
  refusal probes.
- `scripts/test-console-benchmark.sh`: PASS. Six validator mutations were each run and each turned
  the suite red. The count after each is the number of failing cases:
  - drop `bypass_session_shape`: 11;
  - accept the old intended spelling: 20;
  - drop the sparse aggregate rule: 1;
  - drop the documents claim: 23;
  - compare document timings across rounds: 2;
  - accept any app layout: 3.
- `bash scripts/check-bench-policy.sh` (1 timed subject) and `scripts/test-bench-policy.sh`: PASS.
- `scripts/check-console-fixtures.sh` (intended, mono and app regenerated byte for byte) and
  `check-console-benchmark-fixture.sh`: PASS.
- `cargo test --locked --release -p console-workload`: all pass (lib 14, chain_shape 23 and the
  rest). `cargo test --locked --release -p bench`: 13 pass, including
  `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`. `cargo test -p
  session-validator`: pass, so the new fixture is canonical.
- Each named red mutation in the new Rust tests was run and went red:
  - silencing tracks 32 and above instead of odd tracks fails the sparse bank test and the sparse
    pin;
  - a census that ignores partial bypass fails the census test;
  - taking the last N tracks fails the strip-at-N test and the pins.
- V8, untimed:
  - `run-web-mixing-automation-benchmark.sh prepare` built host_web.wasm `885aa117...` at
    `e1b0fed3`, which is not the release pin;
  - `preflight` passed: both documents booted, rendered audible bits, and gave distinct digests.
  - A schema check ran a scratch copy of the harness with `OBSERVATIONS = 8`. Its timings were
    discarded and the copy deleted. The record, with the counts restored to their frozen values,
    passed `web-mixing-automation-validator.jq` with no refusal reason.
- `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D
  warnings` and `cargo check --locked --workspace --all-targets`: clean.
- Policy scripts, all ok:
  - `check-session-policy`, `check-workspace-policy`, `check-env-vocabulary`,
    `check-artifact-evidence-leak`, `check-realtime-policy` and `check-bench-preconditions`;
  - with `python3 -B`: `check-script-reachability.py`, `check-ci-path-routing.py`,
    `check-test-support-ci.py`, `check-release-shape.py` and `check-sdk-deletions.py`.

### For S0 (#1086): the exact commands

Run on a clean detached worktree of B0's merge commit. The native runner refuses untracked files,
so run the native arm first. The V8 work directory must be outside the repository. Hold the
operator's lock around both timed runs, and name it in the disposition.

```bash
git worktree add --detach /path/to/s0 <B0-merge-commit> && cd /path/to/s0
LOCK=/path/to/operator/timing.lock   # name it in the disposition
scripts/operator/preflight-console-benchmark.sh --step console-strip-base
flock "$LOCK" scripts/operator/run-console-benchmark.sh --step console-strip-base
W=$(mktemp -d)
scripts/run-web-mixing-automation-benchmark.sh prepare "$W"
scripts/run-web-mixing-automation-benchmark.sh preflight "$W"
flock "$LOCK" scripts/run-web-mixing-automation-benchmark.sh run "$W" --step console-strip-base
```

Add `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` to a run only if its admissibility refuses the host,
and report the resulting `uncontrolled` records as they are.

The native records land in `artifacts/steps/console-strip-base/console-benchmark.*`. The V8 records
land in `artifacts/steps/console-strip-base/web-mixing-automation.jsonl`, under `documents` in each
round.

### Notes for later slices

- S1a has one more document to migrate (the app fixture). It must also port all three
  `derive-*-console-fixture.py` scripts and `console_model`'s strip edits, which read
  `simd1`/`dynamic`/`simd2`.
- S4 expects the app document's EQ and compressor in `console.pre_insert` after the migration. The
  validators already accept that spelling.

## Sol verdict, attempt 1

**PASS.** Sol verified `e1b0fed3` and `bb6000c0` from `6fdf5db2`. No timed run was made. The rows
measure what S0 and S4 need, the benchmark rules hold, the pins are deterministic, and every gate
passes. The S0 command block needs correcting before S0 runs (M1). That is an evidence defect, not
a code defect.

### Findings, by severity

**M1. The S0 command block cannot be run as written.**
- The lock is a placeholder (`/path/to/operator/timing.lock`). The operator's lock is
  `/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock`.
- The conditional waiver traps the step name. The native runner creates
  `artifacts/steps/console-strip-base/` and its stderr log before admissibility. On a precondition
  refusal it writes a FAIL `console-benchmark.disposition.json` (`run-console-benchmark.sh`, the
  `mkdir`/`: >"$stderr_log"` lines and `on_exit`). A second invocation with
  `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` then refuses to overwrite, and it refuses the untracked
  disposition as a dirty tree. With this host's load (1-minute loadavg about 9, against a 0.50
  ceiling), the first run would be refused.
- Setting the waiver up front costs nothing. Both runners record `controlled` when no precondition
  fails, and they write `uncontrolled` only when one does.
- The block does not copy the records from the detached worktree back to the batch branch.
- V8 `prepare`/`preflight` run after the native timed run. A V8 infrastructure failure would then
  leave half a baseline. Preflight both arms before timing either.

Corrected block, for S0 (#1086):

```bash
LOCK=/tmp/claude-1002/-home-bl-misofm-engine/43895396-e183-427f-b08f-a0b72f15ae5f/scratchpad/timing.lock
B0=<B0 merge commit on codex/batch-console-1>
BATCH=/home/bl/misofm/worktrees/engine-console
WT=<an unused scratch path>
git -C /home/bl/misofm/engine worktree add --detach "$WT" "$B0" && cd "$WT"
W=$(mktemp -d)
bash scripts/run-web-mixing-automation-benchmark.sh prepare "$W"
bash scripts/run-web-mixing-automation-benchmark.sh preflight "$W"
bash scripts/operator/preflight-console-benchmark.sh --step console-strip-base
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/operator/run-console-benchmark.sh --step console-strip-base
flock -w 7200 "$LOCK" env MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
    bash scripts/run-web-mixing-automation-benchmark.sh run "$W" --step console-strip-base
cp -r artifacts/steps/console-strip-base "$BATCH/artifacts/steps/"
```

Each runner makes one invocation, with one warmup and two measured rounds. Both runners enforce
this.

**L1. Gate 4 says "not by a new pinned digest".** B0 adds five 64-block pins
(`the_console_strip_rows_render_their_pinned_bits`).
- They are deterministic: two release runs and one debug run passed.
- They give P1 through S2 a per-slice class-A check, so keep them.
- The evidence should state the deviation. It should also say that an unrelated change which moves
  bits must repin them, under S4 gate 5's re-baseline allowance.

**L2. The S1a hand-off note misses some rack readers.** Besides the three derive scripts and
`console_model`, S1a must port:
- `BypassCensus::of` in `tools/console-workload/src/lib.rs`;
- the harness's new `SECTIONS`, `trackLayout` and `bypassCensus` in
  `scripts/web-mixing-automation-benchmark.mjs`;
- the app arm of `Workload::strip_layout`, which must flip to `pre_insert:eq+compressor`.

The native validator accepts both spellings for the app row, so it will not catch a missed flip.
Only the V8 harness's per-track check catches it, once ported.

**L3. The native preflight's provenance JSON omits the app fixture.** It hashes the standing and
mono fixtures and their generators, but not `console-sixty-four-track-app.json` or
`derive-app-console-fixture.py`. `candidate_commit` pins both files, so no provenance is lost.
Adding them is optional.

**L4. The V8 schema check ran the timing loop.** It used a modified copy of the harness in `run`
mode (`OBSERVATIONS = 8`, timings discarded).
- No number was kept, so gate 5 holds in substance.
- The stub-harness path in `scripts/test-console-benchmark.sh` is the untimed route for such checks.

**Info.**
- The W=4 remainders (1, 2, 1, 0, 0) are arithmetic only. The native rows run at Simd8, and by the
  spec's design V8 has no per-N rows.
- `check-step-vocabulary.py` was retired by #1050 (`dbf4875e`). Step names are governed by the
  runners' pattern, `^[a-z0-9][a-z0-9-]{0,63}$`, which `console-strip-base` and
  `console-strip-after` satisfy.
- The sealed records under `artifacts/steps/` keep the rack-token spellings, and the new validator
  refuses them. Nothing revalidates sealed records, so this is by design.

### What was verified

**The rows.** A temporary probe example, since deleted, read each row's compiled plan.
- The strip at 9, 10, 13, 16 and 64 tracks renders remainders of 1, 2, 5, 0 and 0 at W=8. The
  remainder tracks run each of the three effects per node: 3, 6, 15, 0 and 0 per-node effect ops.
- The app shape: every track carries EQ and compressor in `dynamic`, 21 of 64 are bypassed, and the
  census is `index_mod_3_is_2`. The plan has 23 bank chains, against 8 for the standing console,
  and 16 per-node effect ops. The 43 active tracks (5 banks of 8, plus 3) and the 21 bypassed tracks
  (2 banks of 8, plus 5) bank apart, which is today's split that P1 removes.
- The sparse row compiles the standing plan, and every bank holds both parities.

**For S4.**
- `strip_layout` is layout-neutral. The native and V8 validators pin only the console vocabulary,
  and they accept both app spellings. No session-record field names a bank shape. S4 therefore needs
  no validator rewrite, provided S1a ports the builders (L2).

**The rules.**
- `timed_subjects` still holds one subject, `tools/bench/src/console.rs`.
- Only the native host-core path and the shipped `host_web.wasm` path are timed.
- The workloads and validators are frozen in B0.
- The native preflight probes for existing step artifacts. V8 `prepare` refuses a non-empty
  WORKDIR, and V8 `run` refuses before launching when any of its three artifacts exists.

**The preflights.**
- `preflight-console-benchmark.sh --step console-strip-base`: PASS, `workload_launches` 0,
  `records_required` 60, candidate `bb6000c0`. It left no artifacts.
- V8 `prepare` built `host_web.wasm` `885aa117...`, which is not the release pin.
- V8 `preflight` ran twice and produced byte-identical output. The console document's digest is
  `d913ad96...` and the app shape's is `3dd8b2ff...`.

**The digests.**
- The diff removes or changes no 64-hex pin and touches nothing under `crates/` or `hosts/`.
- `cargo test --locked --release -p console-workload` passed twice (lib 14, chain_shape 23 and the
  other test files), with the five new pins.
- The pinned-bits test also passed in debug.
- `cargo test --locked --release -p bench` passed: 13 tests, including the console-strip record test.

**Mutations, each restored afterwards.** Each of these turned its check red:
- dropping `bypass_session_shape`;
- dropping the sparse aggregate rule (two named cases);
- `OddTracksSilent` becoming `track >= 32` (the sparse test and the pins).

**The gates.** Each passed:
- `cargo fmt --all --check`;
- `cargo check --locked --workspace --all-targets`;
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
- `scripts/test-console-benchmark.sh`;
- `check-bench-policy.sh` (1 subject) and `test-bench-policy.sh`;
- `check-console-fixtures.sh` (intended, mono and app) and `check-console-benchmark-fixture.sh`;
- `check-env-vocabulary.sh` and `test-env-vocabulary.sh`;
- with `python3 -B`, the reachability lint and its self-test (118 reached, 6 exempt);
- with `python3 -B`, the CI-routing check and its self-test. Every new path routes to `full`.

**The policy scripts.** Every `check-*-policy.sh` and `test-*-policy.sh` passed, as did
`check-artifact-evidence-leak.sh` and `check-bench-preconditions.sh`. With `python3 -B`, these also
passed:
- `check-test-support-ci.py`, `check-release-shape.py`, `check-sdk-deletions.py`;
- `check-command-kind-vocabulary.py`, `check-command-reason-vocabulary.py`;
- `check-session-map-shape.py`, `check-browser-expected-resources.py`;
- the self-tests of `check-scalar-oracle-absent.py` and `check-abi-layout-v1.py`.

**Trial merge onto `c4906414`.** The merge is clean, with no conflicts (`git merge-tree`, tree
`b2899df4`). Since `6fdf5db2` the batch has changed only specs and docs. On a scratch worktree of
the merged tree, since removed, these passed:
- the session, workspace, env-vocabulary and bench policies;
- the reachability lint and CI routing;
- the artifact evidence gate;
- `test-console-benchmark.sh`.
