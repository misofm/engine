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

## Terra attempt 1 evidence — PASS (2026-08-22)

Candidate `81c1013` preserves the frozen fuzz manifest
`8b25d5c05a7f5c86b9fab83cde70e3e3c362b2aeb978f037d03ecd21b973c55b`, root lock
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`, and toolchain file
`f6a2b4a1eb8d7d2cad50aa25f028c86a20ca90191f2ff066e8cc34896e94ffd3`. Pinned Cargo 1.97.1
generated `fuzz/Cargo.lock` from stale `0be0fbe18be5635a5bc40d12395dd50a4ea358ed822722c426a9559443a0873b`
to `bf56130a8ea92bae516074ee60c40eb7740c04fe91b78a16b95f6a014d12e9f6`; it remains version 4,
contains `miso-engine-effect-package` and `miso-engine-effect-contract`, and lists the three
unchanged path dependencies under `miso-engine-session-fuzz`. Pinned `metadata --locked --no-deps`
and `check --locked --bins` passed; one repeated generation preserved that exact lock hash.

Workflow SHA-256 changed from frozen incident value
`8f85718ec907dcabd808a8d9b5a1a0d8d4b6152a1ed60c8f8ced967b61fcab0a` to
`8f69e862876d0bdda977c96be46f945d1826199bde9a096abcc4206b7ab064b7`. Static validation found
exactly three `| tee target/` rows. Each has, in its own literal shell block, `set -o pipefail`,
`mkdir -p target`, the unchanged pipeline/arguments, and a nonempty regular-file assertion; the
native-effect benchmark step remains unchanged. Scratch-only proof `miso-engine-issue071.7frWbg`
captured exact success bytes, propagated producer status 73 while retaining partial bytes, and
proved a synthetic mkdir failure made zero additional launches (two total stub launches from the
success and status-73 cases only).

Final changed-path/static seal and `git diff --check` passed. Terra verdict: **PASS**.
`historical_unrecorded_benchmark_pipeline_invocations=1`;
`issue071_fuzz_run_invocations=0`; `issue071_benchmark_workload_invocations=0`;
`issue071_timed_benchmark_invocations=0`; `issue071_ci_rerun_requests=0`.

## Sol final verification — PASS (2026-08-22)

Sol adversarially verified repair commit `81c10130ed06b03fe3879966e486f5106dc51c60`, Terra evidence
commit `b6db0360648ad0ebd5481d0f099d25cfcc6d23d2`, and the unchanged Issue-071 files at committed
candidate `265109f300f58e005ac7a68a56298d167c5ae809`. The exact Issue-071 range changes only
the four authorized workflow, fuzz-lock and evidence paths, and `git diff --check` passes. Frozen
pre-change workflow and fuzz-lock hashes reproduce as
`8f85718ec907dcabd808a8d9b5a1a0d8d4b6152a1ed60c8f8ced967b61fcab0a` and
`0be0fbe18be5635a5bc40d12395dd50a4ea358ed822722c426a9559443a0873b`; current workflow,
fuzz-lock and immutable root-lock hashes are respectively
`8f69e862876d0bdda977c96be46f945d1826199bde9a096abcc4206b7ab064b7`,
`bf56130a8ea92bae516074ee60c40eb7740c04fe91b78a16b95f6a014d12e9f6`, and
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`.

Static verification found exactly three benchmark `tee target/` pipelines and exactly three
corresponding ordered `pipefail`, `mkdir -p target`, and nonempty regular-file guards. The
generated version-4 fuzz lock contains the effect-contract/effect-package closure and the fuzz
package's three unchanged path dependencies. Without regenerating the lock, pinned Rust/Cargo
1.97.1 `metadata --locked --no-deps` and `check --locked --bins` both passed. Terra's accepted
scratch-only producer matrix supplies the bounded pipeline behavioral proof; Sol launched no CI,
fuzz target or benchmark.

Final Sol verdict: **PASS**. `historical_unrecorded_benchmark_pipeline_invocations=1`;
`issue071_fuzz_run_invocations=0`; `issue071_benchmark_workload_invocations=0`;
`issue071_timed_benchmark_invocations=0`; `issue071_ci_rerun_requests=0`.
