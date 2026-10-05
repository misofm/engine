# Run doctests in CI

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found
while filing *Let only host-core build the live route records that hosts push* (#1416), whose
guarantee is a `compile_fail` doctest. CI and test tooling only; no engine code changes.

## Problem (verified on `main` at `0a1176b3b`)

- **No required job runs a doctest.** Every `cargo test` in `.github/workflows/qualification.yml`
  either passes `--all-targets` (`test-debug-a`, `:609-620`; `test-debug-b`, `:641-646`), names
  test targets, or runs `--release` gate packages (`test-release`, `:667-684`; `audit-native`,
  `:710`). Cargo's `--all-targets` builds and runs lib, bin, test, bench and example targets, and
  not doctests. The AArch64 debug leg does the same (`scripts/run-aarch64-tests.sh:142-144`). No
  other workflow runs `--doc` either.
- **The doctests that exist are type-level guarantees.** On `main`, a local
  `cargo test --locked --workspace --doc` with the union of the two debug jobs' features runs 13
  doctests in 35 doctest binaries, all passing on x86-64. 12 of them are `compile_fail`, and each
  states a compile-time guarantee that nothing else tests:
  - `crates/engine/src/realtime/plan.rs:539` (`PreparedRenderPlan`);
  - `crates/graph-compiler/src/lib.rs:61`, `:69`, `:75`, `:81`, `:87`, `:93`
    (`PreparedGraphBuiltinsArtifact`);
  - `crates/host-core/src/prepare.rs:514` (`PreparedHost` is not `Sync`);
  - `crates/host-core/src/render_session.rs:48`, `:53` (`StartedRenderSession`);
  - `crates/lane/src/fpenv.rs:278`, `:283` (`CanonicalFpEnv`).
  The 13th is the plain doctest `crates/host-core/src/prepare.rs:521`. Two more `compile_fail`
  fences, `crates/lane/src/fpenv.rs:360` and `:365`, document the guard on targets without an FP
  control word (`cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))`, that is wasm32),
  so no native doctest run compiles them. A change that breaks any of these guarantees passes CI
  today.
- **A `compile_fail` doctest without an error code can pass for the wrong reason.** None of the
  14 names one, so a typo or a renamed item in the snippet keeps it green while the guarantee is
  gone.
- **Two filed issues rely on this.** #1416 and *Make live strip records valid by construction*
  (#1423) each prove their guarantee with `compile_fail` doctests.

## Decisions

- **D1. A doctest step in each debug job, same packages and features.**
  - `test-debug-a`: a new step after the workspace step, `cargo test --locked --workspace --doc`
    with the same `--exclude` list and the same `--features` as `:609-620`.
  - `test-debug-b`: a new step, `cargo test --locked --doc` over the same `-p` list and features as
    `:638-644`.
  - `scripts/run-aarch64-tests.sh` debug leg: after the main run, `cargo test --locked --doc` with
    the same packages and features (`:131-144`). `lane`'s `CanonicalFpEnv` guarantee is
    target-specific, so AArch64 must run it too.
  No new job, so the router (`scripts/ci-path-router.py`) and the verdict's expectation table
  (`qualification.yml`, job `verdict`, from `:1044`) do not change: the steps run exactly when
  their jobs do.
- **D2. Pin every `compile_fail` doctest's error code.** Each of the 12 that run becomes
  `compile_fail,E....` with the code rustc reports today for the reason its comment states. If
  rustc reports a different reason than the comment (for example a missing import), fix the
  snippet so it fails for the stated reason, and record it.
- **D3. A doctest that fails in a new place is fixed or deleted, each with its reason.** Any
  doctest that fails under D1 on a leg where it never ran (debug AArch64, or the `test-debug-b`
  feature set) is fixed if its guarantee holds there, or deleted with the reason if it never could;
  a doctest is not marked `ignore` to get green.
- **D4. The CI checkers accept the new steps.** `scripts/check-test-support-ci.py` treats `--doc`
  as a narrowing selector (`:44`): a `--doc` step does not count as a whole-package test step, and
  the existing `--all-targets` steps still do. `scripts/check-ci-path-routing.py` must pass on the
  new workflow; if it parses these jobs' steps and refuses the new one, extend it in the same
  change, with a self-test case.

## Authorized paths

- `.github/workflows/qualification.yml` (the two new steps only)
- `scripts/run-aarch64-tests.sh` (the debug leg's doctest run only)
- `scripts/check-test-support-ci.py`, `scripts/test-test-support-ci.py`,
  `scripts/check-ci-path-routing.py` and its self-test (only if D4 needs them)
- `crates/lane/src/fpenv.rs` (one comment beside the wasm32-only fences at `:360` and `:365`)
- The 12 `compile_fail` doctests named above (their fence line, and their snippet only where D2
  finds a wrong reason)
- This spec

## Non-goals

- Doctests in release jobs, in `nightly.yml` or in `fuzz.yml`.
- New doctests (#1416 and #1423 add theirs).
- The two wasm32-only fences in `crates/lane/src/fpenv.rs` (`:360`, `:365`). rustdoc cannot run a
  doctest on `wasm32-unknown-unknown` here; leave them as documentation and say so in a comment
  beside them. A wasm doctest runner is a separate issue if the owner wants one.
- Converting `text` or `ignore` fences.

## Hazards

- **Hot files.** `.github/workflows/*.yml` is in `STREAMS.md`'s hot-file table (#877, H #1334);
  this change adds two steps and touches neither the toolchain nor the browser artifact.
- **CI-conscious batching.** AGENTS.md: do not push to get a CI run. Gate 3's CI evidence comes from
  the batch's single pull-request run.
- **Time budgets.** `test-debug-a` and `test-debug-b` have a 15-minute timeout. Doctests reuse the
  jobs' compiled dependencies; record the steps' wall time from the PR run, and if either job would
  then exceed 80 % of its budget, stop and report rather than raising the timeout.

## Objective gates

1. **Every doctest runs and passes.** The exact new step commands, run locally on x86-64, exit 0,
   and their output lists all 13 doctests. `bash scripts/run-aarch64-tests.sh debug` on an AArch64
   host, if one is available, or the PR's `aarch64-debug` job, passes with the doctest run.
2. **A broken guarantee is red (PR evidence).** Make one guarantee false: add
   `unsafe impl Sync for PreparedHost {}` in `crates/host-core/src/prepare.rs`, or delete D2's error
   code and make the snippet compile. The `test-debug-a` doctest step command exits non-zero, naming
   that doctest. Revert. Repeat for `lane`'s `CanonicalFpEnv` with the `test-debug-b` command.
3. **A wrong reason is red (PR evidence).** Introduce a typo in one `compile_fail` snippet (an
   undefined name): with D2's error code the doctest fails; without it, it passes. Record both.
4. **The CI checkers.** `python3 -B scripts/check-test-support-ci.py`,
   `python3 -B scripts/test-test-support-ci.py`, `python3 -B scripts/check-ci-path-routing.py` and
   its self-test, `bash scripts/check-workspace-policy.sh` exit 0.
5. **The PR run.** On the batch's pull request, `test-debug-a`, `test-debug-b` and `aarch64-debug`
   show the doctest steps, each passing; the `qualification` verdict is green.

*Test value.* No new test. The new steps make the 12 existing `compile_fail` doctests, and the
ones #1416 and #1423 add, able to fail a merge: each is red if the type-level guarantee it states
stops holding, which nothing in CI checks today. D2's error codes make each red only for its own
reason.

## Evidence

- Gate 1's doctest lists per leg, gate 2's and gate 3's runs, and the new steps' wall times.
- Every D2 error code, and any snippet D2 or D3 had to fix or delete, with its reason.

## Dependencies

- None.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
