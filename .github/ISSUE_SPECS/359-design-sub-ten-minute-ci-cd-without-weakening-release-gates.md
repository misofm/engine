# Design sub-ten-minute CI/CD without weakening release gates

## One-line summary

Ask Fable 5.1 to design the optimal GitHub Actions architecture for fast daily
iteration while preserving every correctness, realtime, artifact, portability,
and release assertion that can fail a merge.

## Problem

Routine development cannot tolerate 20–30 minute pull-request feedback. The
current repository has path detection, but its `full` route is too coarse and
the principal host job is too monolithic. A single late or flaky failure wastes
the entire critical path and a failed-workflow rerun may repeat already-green
prerequisites.

PR #358 provides a concrete incident rather than a hypothetical concern:

- `release-build.yml` passed in **21m31s** (run `33841397089`);
- SDK qualification passed in **7m23s** (run `33841397131`);
- browser qualification took up to **7m43s** (run `33841397119`);
- the first `host quality and native shell` job failed after **11m35s** in run
  `33841397115`, during `cargo test --locked --workspace --all-targets`;
- its allocation-accounting failure passed immediately in isolation on both
  the candidate and unchanged old worktree, but rerunning the failed workflow
  repeated several green jobs and again entered the long host test; and
- the host job contains roughly 68 sequential steps after setup, including the
  workspace suite, policy/mutation suites, release-mode DSP gates, realtime
  audits, target checks, fixture checks, and native smoke tests.

The result is correct but operationally hostile: a developer can wait most of
an hour across one late failure and rerun even when the edited capability is
small.

## Current topology Fable must inspect

- `.github/workflows/ci.yml`: `route`, one very large `host` job, `x86-probes`,
  `wasm`, `wasm-gates`, and required aggregate `qualification`.
- `.github/workflows/sdk.yml`: route plus SDK package/headless qualification;
  it still builds the pinned Engine AudioWorklet because the Wasm is part of
  the npm archive.
- `.github/workflows/browser-qualification.yml`: builds the shipped artifact
  once, then runs Chromium, Firefox, and WebKit qualification in parallel.
- `.github/workflows/release-build.yml`: path-filtered workspace release build.
- `.github/workflows/fuzz.yml`: path-filtered bounded PR fuzzing.
- `.github/workflows/nightly.yml`: non-blocking descriptive/deep work.
- `scripts/ci-path-router.py`, `scripts/check-ci-path-routing.py`, and
  `scripts/test-ci-path-routing.py`: fail-safe `evidence` / `sdk` / `full`
  classification, including rename/copy/base handling.

Pure `sdk/**` changes already skip the Rust host/x86/Wasm test matrix and run
the SDK workflow. Evidence-only changes skip heavy work. Unknown, mixed, Cargo,
Rust, and workflow changes route to `full`. One known omission is
`scripts/test-sdk-artifact-builder-output-contract.sh`, which is SDK-owned but
is not in the narrow SDK path set.

## Consultation objective

Fable 5.1 should recommend one coherent target architecture—not merely a list
of possible optimizations—and support it with expected critical-path timing,
failure behavior, security boundaries, and a staged migration plan.

The design should make the fastest correct route the default. It should answer:

1. What measurable P50/P95 budgets should apply to first signal, routine PR
   completion, full-impact PR completion, merge/main, and release qualification?
2. What is the right unit of change classification: path families, Cargo
   dependency closures, generated-artifact ownership, job inputs, or a hybrid?
3. How should the monolithic host job be sharded so independent gates run in
   parallel without hiding failures or introducing nondeterministic shared
   state?
4. Which checks must run on every relevant PR, which may run after merge, which
   belong only in nightly/manual qualification, and why?
5. How can Rust, npm, browser, toolchain, and built-artifact caching reduce cold
   setup while remaining correct and resistant to cache poisoning from
   untrusted pull requests?
6. When may a previously built Engine Wasm or compilation output be reused, and
   which final release artifacts must always be rebuilt and byte-attested?
7. How should required aggregate checks behave when path-filtered workflows or
   jobs are skipped, so branch protection never waits forever or accepts an
   unknown state?
8. How should a failed shard be rerun without repeating unrelated green shards?
9. How should flaky/global-state tests—especially allocation tracking—be
   isolated, serialized, or redesigned without weakening their assertion?
10. What observability should record queue time, setup time, cache hit rate,
    execution time, critical path, reruns, and flake rate so the latency budget
    cannot silently regress?

## Non-negotiable correctness and security constraints

- Do not delete, skip, or weaken a merge-relevant gate merely to improve time.
  If a gate is redundant, prove which stronger gate subsumes it before removal.
- Realtime allocation/syscall/lock rules, deterministic PCM/DSP fixtures,
  scalar/SIMD parity, PDC, browser targets, native targets, and package mutation
  tests remain real assertions.
- The sealed Engine AudioWorklet digest and npm provenance must describe the
  exact built release artifact. A dependency cache is not release evidence.
- Unknown paths, malformed diffs, missing bases, unexpected rename/copy status,
  and manual dispatch must fail safe to the broad route.
- Pull requests from untrusted forks must not gain secrets or poison caches used
  by trusted release jobs. Cache restore/save permissions and key provenance
  must be explicit.
- A check that can fail a merge belongs in a required PR workflow. Descriptive
  benchmarks and deep fuzzing stay outside blocking CI.
- GitHub `paths:` filters operate at workflow scope. Before a workflow stops
  reporting on any PR, its status context must be removed or replaced in branch
  protection so skipped checks never remain permanently pending.
- Feature-branch pushes should not duplicate pull-request CI. `main` and manual
  release behavior must remain explicit, with superseded runs cancelled by ref.
- No compiled `MAX_TRACKS`, no weakening of supported-target coverage, and no
  change to engine/runtime semantics is authorized by this design issue.

## Required Fable 5.1 deliverable

Post a design comment containing:

1. a diagram of proposed workflows, jobs, dependencies, required aggregates,
   artifacts, and cache boundaries;
2. a path/input ownership matrix showing the exact route for SDK-only, docs,
   Rust crate, root Cargo/lock/toolchain, browser host, workflow/policy,
   fixture, and mixed changes;
3. the proposed sharding of every current blocking gate, including why each
   shard is safe to parallelize and its estimated duration;
4. exact required-check and GitHub Actions skip/failure/rerun semantics;
5. cache keys, restore/save rules, invalidation inputs, fork protections, and
   which artifacts are never reused as release evidence;
6. latency and cost estimates derived from recent run evidence, with explicit
   P50/P95 acceptance budgets;
7. a staged migration that keeps `main` protected at every step, including the
   branch-protection transition order and rollback plan;
8. a validation plan that intentionally mutates path routing, cache inputs,
   job failures, skipped jobs, and aggregate results; and
9. a short list of bounded implementation issues in dependency order.

Fable should call out any current gate whose purpose is unclear rather than
guessing that it is redundant. The answer must distinguish queue/setup time
from execution time and distinguish routine PR checks from release
qualification.

## Acceptance gates

1. The comment recommends one architecture and accounts for every current
   blocking workflow/job family.
2. Every proposed skip has a concrete input/dependency argument and a fail-safe
   fallback.
3. Required status checks always resolve to an explicit success or failure for
   every pull request shape.
4. The design includes a credible route to sub-ten-minute routine PR completion
   and defines a separate bounded target for genuinely full-impact changes.
5. Release artifact/provenance integrity and untrusted-PR isolation are not
   weakened.
6. Migration can be split into small issue-first checkpoints without a period
   in which `main` is less protected.
7. Sol adversarially reviews Fable's proposal before any implementation issue
   is authorized.

## Scope

This issue authorizes design and review only. It does not authorize workflow,
branch-protection, cache, test, or runtime changes. After Fable 5.1 comments,
Sol and the owner will accept, amend, or reject the proposal and create bounded
implementation issues.
