# Retire the used-up console benchmark arms

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, row B1. No
ruling is needed: each arm removed here can never run again.

## Context

`scripts/run-console-benchmark.sh` (682 lines) is the live native console benchmark. Its only
usable mode is `--step NAME`, which writes `artifacts/steps/NAME`.

**The dead arms.**

- Besides `--step`, the runner carries 48 named historical arms (`:283-330`): `--phase2`,
  `--issue163-phase0` … `--plumbing-floor-baseline`. It also has a no-argument default that
  writes `artifacts/issue149`.
- All 49 target folders exist. The audit checked all 48 named arms and `artifacts/issue149`.
- The runner refuses to overwrite a record (`:351-353`), so every one of those modes can only
  fail.
- Lines `:1-271` are header comments, mostly each arm's history.

**The preflight is out of sync.** `scripts/operator/preflight-console-benchmark.sh` (177 lines)
mirrors 40 of the 48 arms. It lacks these eight:

- `--copy-removal`, `--copy-removal-baseline`, `--copy-removal-without-920`;
- `--issue388-lane4-evidence`;
- `--plumbing-floor`, `--plumbing-floor-baseline`;
- `--pure-path`, `--pure-path-baseline`.

So the lists are not maintained. Yet they are still edited: #956 re-indexed arms that cannot run.

**Tests do not name arms.** `scripts/test-console-benchmark.sh`, `check-bench-policy.sh`,
`test-bench-policy.sh` and the console jq validators reference no historical arm by name (audit
grep). So removing the arms cannot weaken any validator.

**The wasm console runner is out of scope.** `scripts/operator/run-wasm-console-benchmark.sh` and
its preflight have 29 more arms of the same kind, but they belong to ruling `R9-…`.

## Smallest closable slice

1. **`run-console-benchmark.sh`.**
   - Keep only `--step NAME`. A call with no arguments, or with any other argument, is a usage
     error (exit 2).
   - Replace the 271-line header with a short description of the `--step` mode and its
     preconditions. The arms' histories stay in git and in their issue specs.
2. **`operator/preflight-console-benchmark.sh`.** The same change: `--step NAME` only.
3. **Docs.** Update every live doc (outside `docs/handoffs/`) that tells a reader to run a removed
   arm: `scripts/operator/README.md`, rulings that name an arm as a re-run recipe, and
   `docs/ENGINE_ENV_VOCABULARY.md`. Leave the records under `artifacts/` alone; `07-…` handles
   them.

## Objective gates

1. **Scripts.**
   - `bash scripts/test-console-benchmark.sh`, `bash scripts/check-bench-policy.sh`,
     `bash scripts/test-bench-policy.sh`, `bash scripts/check-bench-preconditions.sh` and
     `bash scripts/check-console-benchmark-fixture.sh` pass.
   - `bash scripts/run-console-benchmark.sh` and `bash scripts/run-console-benchmark.sh --strip4`
     exit 2 with the new usage.
   - `bash scripts/run-console-benchmark.sh --step base` refuses to overwrite the existing record
     and launches no workload.
2. **Native and wasm build:** unaffected, because no Rust changes. Both still pass:
   - `cargo check --locked --workspace --all-targets`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`.
3. **Console digests:** unaffected; no engine or workload source changes. State this in the
   evidence with `git diff --stat` showing only `scripts/` and `docs/`.
4. **Shipped artifact:** unaffected; no crate in its closure changes.
5. **CI routing.** No workflow step names a removed arm. `python3 -B scripts/check-ci-path-routing.py`
   and `python3 -B scripts/test-ci-path-routing.py` pass. The change routes to `full` because
   `scripts/` is not an evidence path.
6. **No live claim lost.** No test is removed, and every lint step that ran before still runs.

## Dependencies

None. It can land in the same CI-conscious batch as `04b-…` and `04c-…`.

## Standing rules for the implementer

- Do not run the benchmark. The gates need only argument and preflight paths, which launch no
  workload.
- Commit on `codex/<issue>-retire-console-arms`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`. The draft is sound. Reproduced: 48 named arms plus the `issue149`
default, all 49 folders hold records; the preflight has 40 named arms plus the default and lacks
exactly the eight listed; `--step base` and `--strip4` both print "refusing to overwrite" and exit
1 without launching anything; no test or validator names a historical arm.

1. Gate 1 expects `--strip4` to "exit 2 with the new usage" after the change; today it exits 1
   (refusal). Say explicitly that the exit code changes from 1 to 2 for removed arms.
2. **Records the rulings cite stay reproducible from git, not from the runner.** Rulings such as
   `docs/rulings/effect-floor-accounting.md` cite arms and records by folder. The runner's arm
   history leaves the header, so add a one-line pointer in the new header to the commit before this
   change (a permalink), so a reader can still find each arm's exact invocation.
3. Mobile scope: no effect.

## Attempt 1 evidence

Implementer attempt 1 (Terra), 2026-09-28. Branch `codex/1025-retire-console-arms`, base
`ed0556a9` (the `codex/batch-slim-1` head), code commit `03101cea`. Host x86_64, `CARGO_INCREMENTAL=0`.
No timed benchmark ran: every runner and preflight call below stops at argument parsing or at the
overwrite refusal, before any build or workload.

### The change

| file | before | after | +/- |
|---|---|---|---|
| `scripts/run-console-benchmark.sh` | 682 | 388 | +31 / -325 |
| `scripts/operator/preflight-console-benchmark.sh` | 177 | 98 | +12 / -91 |
| `docs/rulings/fast-db-tier-boundaries.md` | | | +2 / -1 |
| `docs/rulings/multiband-ramping-split-boundary.md` | | | +2 / -1 |
| `docs/rulings/stationary-smoother-hoist-boundary.md` | | | +2 / -1 |
| **total** | | | **+49 / -419 (net -370)** |

- **Runner.** Only `--step NAME` parses. No argument, a removed arm, or any other shape prints
  `usage: … --step NAME` and exits 2. The NAME regex and its exit 2 are unchanged. The 271-line
  arm history becomes a short header: the `--step` mode, the refusals in the order the script
  applies them, and a pointer to the preflight. The admissibility section is kept verbatim.
  Everything after argument parsing is unchanged except the variable name (`step_directory`) and
  three refusal messages that still said "Issue-149 qualification"; they now say "the console
  benchmark". The disposition's `"issue":149` schema field is untouched.
- **Amendment 1.** A removed arm now exits **2** (usage). Before, it exited **1** (overwrite
  refusal). See the table below.
- **Amendment 2.** The header points at
  `https://github.com/misofm/engine/blob/d3349b72dd9e48674d087d14722368b23c4bfc1b/scripts/run-console-benchmark.sh`.
  `d3349b72` (#1003) is the last commit to touch either script. It is an ancestor of `origin/main`,
  and its runner and preflight are byte-identical to the base's. So the permalink resolves upstream
  and shows all 48 arms and the `issue149` default.
- **Preflight.** It gets the same `--step NAME`-only parser and a matching short header.
  `issue-149 artifact already exists` became `console artifact already exists`. Every check after
  argument parsing is unchanged: fixture checks, validator mutation suite, preconditions
  self-test, bench tests, workspace clippy, release build, subject refusals, and the provenance
  JSON.
- **Docs.** Three rulings name a retired arm as their run command: `--phase2`, `--phase3`, and
  the no-argument `issue149` default. Each now says the arm is a one-shot retired by #1025, and
  that the runner's header links a version that still carries it. Their records and figures are
  unchanged. No other doc needed a change:
  - `scripts/operator/README.md` and `docs/ENGINE_ENV_VOCABULARY.md` name no removed arm (grep).
  - `docs/rulings/effect-floor-accounting.md` and `d7-check-block-fusion.md` cite the runner or
    record folders only. The permalink keeps those reproducible.
  - `crates/rack/tests/MUTATIONS.md:271` ("the sealed `--mono3` pair measured it") is history,
    not a recipe.
  - Closed issue specs are left to R10 (#1040), records under `artifacts/` to 07 (#1030), and the
    wasm runner and its preflight to R9 (#1039).

### Argument paths (no workload launched)

| invocation | runner before | runner after | preflight after |
|---|---|---|---|
| (none) | 1, refuses `artifacts/issue149` | 2, usage | 2, usage |
| `--strip4` | 1, refuses `artifacts/strip4` | 2, usage | 2, usage |
| `--phase2` | 1, refuses `artifacts/issue149-phase2` | 2, usage | 2, usage |
| `--plumbing-floor-baseline` | 1, refuses | 2, usage | 2, usage |
| `--step` | 2, usage | 2, usage | 2, usage |
| `--step Bad_Name` | 2, invalid name | 2, invalid name | 2, invalid name |
| `--step base extra` | 2, usage | 2, usage | 2, usage |
| `--step base` | 1, refuses `artifacts/steps/base` | 1, refuses `artifacts/steps/base` | 1, `console artifact already exists` |

`git status` stayed clean after every call.

### Gates

Every command passed (exit 0) on `03101cea` with a clean tree.

| gate | result |
|---|---|
| `bash scripts/test-console-benchmark.sh` | pass |
| `bash scripts/check-bench-policy.sh`, `bash scripts/test-bench-policy.sh` | pass |
| `bash scripts/check-bench-preconditions.sh`, `bash scripts/check-console-benchmark-fixture.sh`, `bash scripts/check-console-fixtures.sh` | pass |
| `bash scripts/check-env-vocabulary.sh`, `bash scripts/test-env-vocabulary.sh` | pass |
| `python3 -B scripts/check-ci-path-routing.py`, `python3 -B scripts/test-ci-path-routing.py` | pass |
| `python3 -B scripts/ci-path-router.py --event pull_request --base HEAD~1 --head HEAD --flags` | `route=full`, `math_closure=false`, `release_inputs=false` |
| `cargo fmt --all -- --check` | pass |
| `cargo check --locked --workspace --all-targets` (also with `--all-features`) | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` (also with `--all-features`) | pass |
| `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web` | pass |
| `cargo test --locked -p bench -p console-workload -p bench-support` | pass |
| The rest of the `lint` job's script steps, run locally in its order (see below) | pass |

The other lint steps, by script family:

- workspace and session policy;
- test-support CI;
- rack fixture;
- builtins fixture and manifest consumers;
- the wasm-console, rack, builtins-current and wasm-kernel-timing validator suites;
- host-core and protocol-control;
- realtime policy, audit leak and artifact-evidence leak;
- realtime trace;
- lane;
- unfused seal;
- rack, builtins and graph policy;
- effect runtime and its fixtures;
- effect interchange;
- the native PCM runner seals;
- conformance boundaries;
- step vocabulary;
- the parametric-EQ render contract;
- release-shape self-test;
- npm publish modes;
- stem store.

- **`timed_subjects` ratchet.** `scripts/check-bench-policy.sh:184-185` names Rust subjects
  (`tools/bench/src/rack.rs`, `tools/audit/src/fp_env.rs`, `tools/wasm-console/src/main.rs`), not
  runner arms. No retired arm appears there, so the ratchet is unchanged.
- **No test lost.** `cargo test --locked --workspace --all-targets --all-features -- --list` gave
  the same output before (on `ed0556a9`) and after (on `03101cea`): 2,556 tests in 274 test
  targets. The retired arms were shell dispatch entries with no tests of their own.
  - No shell suite or workflow step was edited.
  - No workflow names `run-console-benchmark.sh`, its preflight, or any of the 48 arms.
- **Gates 2 to 4.** `git diff --stat ed0556a9 03101cea` touches only `scripts/` (2 files) and
  `docs/rulings/` (3 files). So there are no Rust, workload, validator, fixture or workflow
  changes; the console digests and the shipped artifact's closure are unchanged by construction.
- **Mobile.** No effect.

## Sol attempt 1 verdict: PASS

Reviewer: Sol, 2026-09-28, on `d1b1b844` (work `03101cea`), and on a scratch merge of it onto the
`codex/batch-slim-1` head `b8bea8e1` (clean, no conflicts). No timed workload was launched.

**Nothing still used was removed.**
- After argument parsing, both scripts are byte-identical to `ed0556a9`, apart from the variable
  rename and the four refusal messages. The old runner never read the arm again after it chose the
  directory, so no arm had behaviour of its own that the `--step` path lost.
- All 49 target folders hold a record, so each of them could only refuse.
- Apart from the `--phase2` and `--phase3` rulings the evidence already lists, no live file names
  a retired arm or the no-argument default as a recipe. The search covered the rulings, `docs/`,
  `scripts/operator/README.md`, `docs/ENGINE_ENV_VOCABULARY.md`, `tools/`, `crates/` and both
  workflows. The other hits are the wasm runner's own arms (R9) and history in
  `crates/rack/tests/MUTATIONS.md`.
- `floor.rs` names no arm and no folder. The floor-accounting clause needs "a record produced by
  `scripts/run-console-benchmark.sh`", and the `--step` path still produces one.
- `test-console-benchmark.sh` and the policy scripts do not run either script.
  `check-step-vocabulary.py` is about the word "nudge". The only other reader of `artifacts/steps`
  is the web mixing-automation runner, which is untouched and uses the same NAME regex.
- The permalink is right: `d3349b72` is an ancestor of `origin/main` (`a9414c0c`), and its two
  scripts equal the base's and carry all 48 arms.

**Exit code 1 to 2.** No workflow, script or test runs either script. None of them matches the
changed messages or checks an exit status from them. The only records of the old exit 1 are
history: #1022's evidence and closed specs.

**Argument paths**, run from the scratchpad as the working directory. These exit 2 with the usage:
- no argument, `--strip4`, `--phase2`, `--issue163-phase0`, `--plumbing-floor-baseline`;
- `--step`, `--step=base`, `--STEP base`, `base --step`, `--step base extra`.

These exit 2 as an invalid name: `Bad_Name`, `""`, `-base`, and a 65-character name.

`--step base` and `--step after-1003` exit 1 at the overwrite refusal. `bash -x` shows that
neither script runs cargo, the binary, `mkdir` or any check before refusing.

**End to end, untimed, on the merge:**
- `preflight-console-benchmark.sh --step verify-1025-preflight` passed, with 0 workload launches,
  in 202 s. It wrote 22 provenance fields, none empty, and `runner_sha256` equals the merged
  runner's hash.
- The runner with a new NAME in a dirty tree reached "requires a clean committed candidate" and
  exited 1, before `mkdir`.

**Disclosure.** By mistake I also ran the runner with a valid 64-character NAME on the
implementer's clean worktree. It built `bench`, then refused at `precondition_loadavg_above_ceiling`
with `workload_process_launches: 0`. That incidentally exercised the full `--step` path through
the build to the admissibility refusal. I removed the `artifacts/steps/aaa…/` folder it created,
and that tree is clean again.

**Gates on the merge.** Every gate below exits 0:
- `cargo fmt --all -- --check`;
- `cargo check --locked --workspace --all-targets`, with and without `--all-features`;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, with and without
  `--all-features`;
- the wasm32 `host-web` check;
- `cargo test --locked -p bench -p console-workload -p bench-support`: 172 passed;
- `check-bench-policy.sh`, which reports "7 operator scripts rooted";
- `test-bench-policy.sh`, `check-bench-preconditions.sh`, `check-console-benchmark-fixture.sh` and
  `check-console-fixtures.sh`;
- `test-console-benchmark.sh`, which made 0/0/0 real invocations;
- `check-`/`test-env-vocabulary.sh`, `check-step-vocabulary.py`, and `check-`/`test-ci-path-routing.py`.

The router gives `route=full`. The test list is 2,556 tests in 274 targets, the same as the
implementer's count. The line counts and the net -370 match. `bash -n` is clean.

**Findings.** None blocks the verdict.
1. Merge order: #1027 moves `run-console-benchmark.sh` into `scripts/operator/`. It must move
   this version of the file, not re-add the old one. No sibling branch touches these files today.
