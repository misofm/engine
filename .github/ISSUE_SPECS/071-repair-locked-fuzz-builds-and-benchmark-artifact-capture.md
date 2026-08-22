# 071 Repair locked fuzz builds and benchmark artifact capture

## Outcome

Repair the two independently verified CI plumbing failures from run `32546875919`: make the
standalone fuzz workspace lock current for pinned Rust 1.97.1 `--locked` builds, and guarantee that
every benchmark `tee target/...` pipeline has a writable artifact directory before any workload
can start.

## Context

The failed main-branch run at candidate `8d874a89eb949f34f180f78154486e105203914e` established
two tooling defects, not product defects:

- both fuzz build jobs stopped at `cargo check --locked --manifest-path fuzz/Cargo.toml --bins`
  because Cargo would update `fuzz/Cargo.lock`; the manifest contains the path dependency
  `miso-engine-effect-package`, while the stale lock does not contain that package or list it under
  `miso-engine-session-fuzz`; and
- the first benchmark step evaluated
  `bash scripts/run-conformance-benchmark.sh 2 | tee target/conformance-benchmark.jsonl` before
  `target/` existed. `tee` failed, but the producer still executed and printed both measured
  rounds. The artifact was not retained and later benchmark steps were skipped.

That pre-issue CI event is recorded as
`historical_unrecorded_benchmark_pipeline_invocations=1`. It is not an Issue-058 builtin benchmark
invocation and grants no retry authority. Both fuzz run counts were zero because their build gate
failed first.

This issue has exactly one Terra implementation/review attempt followed by Sol verification, with
no correction retry. Any required-gate failure is final STOP/RESCOPE. Issue-071 counters begin and
must remain `fuzz_run_invocations=0`, `benchmark_workload_invocations=0`, and
`timed_benchmark_invocations=0`.

## Scope

Change only `.github/workflows/ci.yml` and generated `fuzz/Cargo.lock`, then append concise evidence
to this spec/brief. Regenerate the standalone lock with the pinned stable toolchain; do not edit it
by hand or change `fuzz/Cargo.toml`. In each of the three workflow steps containing a benchmark
pipeline to `tee target/...`, place `set -o pipefail` and `mkdir -p target` before the pipeline,
then require the named artifact to be a nonempty regular file after success.

## Required interfaces/contracts

The existing commands, workload arguments, rounds, job triggers, runner class and artifact upload
paths remain unchanged. Directory creation is local to every benchmark pipeline block, so failure
to create `target/` stops before the producer starts. `pipefail` makes producer or `tee` failure
fail the step. The workflow must contain exactly the existing three benchmark `tee target/...`
pipelines, each dominated in its own shell block by directory creation and pipefail setup.

`fuzz/Cargo.lock` remains Cargo-generated version 4 and must resolve the unchanged standalone
manifest under Rust/Cargo 1.97.1. The repository root `Cargo.lock` is immutable in this issue.

## Deliverables

- regenerated, pinned-toolchain-valid `fuzz/Cargo.lock`;
- three guarded benchmark pipeline blocks in `.github/workflows/ci.yml`; and
- exact zero-workload static/synthetic/local validation evidence and final verdict.

## Explicit non-goals

Running any fuzz target; running a benchmark, benchmark runner, benchmark preflight, audit, trace,
target matrix or timing command; changing fuzz targets/dependencies/manifests; changing benchmark
scripts, workloads, rounds, validators, schedules or artifacts; changing product code; recovering
the lost CI stdout as an accepted artifact; retrying/re-running CI run `32546875919`; manually
dispatching CI; or touching Issue 058.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix

## Acceptance gates with objective measurements

1. `fuzz/Cargo.toml`, root `Cargo.lock`, `rust-toolchain.toml` and every path outside the four
   authorized issue/workflow/lock docs paths remain byte-identical. Generate the fuzz lock with
   `cargo +1.97.1 generate-lockfile --manifest-path fuzz/Cargo.toml`; the generated graph includes
   every unchanged path dependency, including `miso-engine-effect-package`.
2. `cargo +1.97.1 metadata --locked --manifest-path fuzz/Cargo.toml --format-version 1 --no-deps`
   and `cargo +1.97.1 check --locked --manifest-path fuzz/Cargo.toml --bins` pass. A second
   pinned-toolchain generation produces no lock diff/hash change. These are build/lock checks, not
   fuzz runs.
3. Static inspection finds exactly three `| tee target/` benchmark pipelines and proves every one
   has `set -o pipefail` and `mkdir -p target` earlier in the same `run: |` block, plus a nonempty
   regular-file assertion for its named output. Directory creation precedes producer launch.
4. A scratch-only counted producer proves: absent `target/` is created before the pipeline; success
   preserves exact stdout bytes; producer status 73 is propagated under pipefail while partial
   bytes remain; and a directory-creation failure produces zero stub launches. The scratch command
   must not name or execute a repository benchmark or fuzz binary.
5. Workflow/document syntax, title/dependency checks, `git diff --check`, exact changed-path scan
   and static no-fuzz/no-benchmark/no-timing scans pass. Do not use `gh run rerun`,
   `workflow_dispatch`, a benchmark job, a fuzz run or a CI execution as an Issue-071 gate.

## Target matrix

Pinned Cargo/Rust 1.97.1 on the native development host for lock generation and `--locked` build
checking; GNU Bash pipeline semantics matching the Ubuntu CI runner. No fuzz execution, sanitizer,
mobile, Wasm, DSP or timing target is exercised.

## Required evidence

Run/head/job IDs and exact failure text; before/after workflow, fuzz manifest/lock, root lock and
toolchain hashes; generated lock package/dependency proof; pinned metadata/check results; exact
three guarded pipeline rows; scratch producer launch/status/artifact matrix; changed-path and diff
checks; strict verdict; `historical_unrecorded_benchmark_pipeline_invocations=1`;
`fuzz_run_invocations=0`; `benchmark_workload_invocations=0`;
`timed_benchmark_invocations=0`; and confirmation that no CI rerun was requested.
