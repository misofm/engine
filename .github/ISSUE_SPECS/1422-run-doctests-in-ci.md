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
- **A `compile_fail` doctest can pass for the wrong reason.** A typo or a renamed item in the
  snippet keeps it green while the guarantee is gone. An error code on the fence does not help on
  the pinned stable toolchain: rustdoc checks `compile_fail,E....` codes only on nightly (*The
  rustdoc book*, "Unstable features", section "Error numbers for compile-fail doctests",
  <https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests>).
  Attempt 1's probe confirmed it on 1.97.1: with the fake code `E0000` on all 12 fences, all 12
  passed; under `RUSTC_BOOTSTRAP=1` all 12 failed. One snippet
  (`crates/graph-compiler/src/lib.rs:61`, a struct literal with private fields) gets an error with
  no E-number at all.
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
- **D2. Every `compile_fail` doctest has a passing twin (Amendment 1).** Beside each of the 12
  that run, a plain doctest that is identical except for exactly the one forbidden construct: the
  twin reaches the same items by the allowed path, so a rename, a typo or an unrelated error in the
  shared code turns the twin red on stable. `crates/graph-compiler/src/lib.rs:61` gets the same
  treatment. Where an existing plain doctest already is such a twin (for example
  `crates/host-core/src/prepare.rs:521` for `:514`, if it differs from it in the forbidden construct
  only), reshape it into the twin rather than adding another. The fence stays `compile_fail`
  without an error code. A comment beside each fence names the code rustc reports today for the
  reason the comment states (or says rustc gives that error no code), as documentation only, and
  says that stable rustdoc does not check it. If rustc reports a different reason than the comment
  (for example a missing import), fix the snippet so it fails for the stated reason, and record it.
  No `RUSTC_BOOTSTRAP` anywhere: nightly behaviour on the stable toolchain is refused.
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
- The 12 `compile_fail` doctests named above (their fence line, their error-code comment, their
  snippet only where D2 finds a wrong reason) and their twins (D2), in the same doc comments.
  `crates/engine/src/realtime/plan.rs:539` is in stream B's area: do its pair last, after root
  confirms stream B's batch 1 is on `main` (Amendment 1).
- This spec

## Non-goals

- Doctests in release jobs, in `nightly.yml` or in `fuzz.yml`.
- New doctests other than D2's twins (#1416 and #1423 add theirs, with twins by the same rule).
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
   and their output lists every `compile_fail` doctest and its twin (record the count).
   `bash scripts/run-aarch64-tests.sh debug` on an AArch64 host, if one is available, or the PR's `aarch64-debug` job, passes with the doctest run.
2. **A broken guarantee is red (PR evidence).** Make one guarantee false: add
   `unsafe impl Sync for PreparedHost {}` in `crates/host-core/src/prepare.rs`. The
   `test-debug-a` doctest step command exits non-zero, naming that doctest. Revert. Repeat for `lane`'s `CanonicalFpEnv` with the `test-debug-b` command.
3. **A wrong reason is red (evidence per pair).** For each `compile_fail`/twin pair: rename an
   item in the shared part of both snippets (or introduce the same typo in both): the twin turns
   red. Revert. Delete the forbidden construct from the `compile_fail` snippet so it compiles: the
   `compile_fail` doctest turns red. Revert. Record both runs per pair.
4. **The CI checkers.** `python3 -B scripts/check-test-support-ci.py`,
   `python3 -B scripts/test-test-support-ci.py`, `python3 -B scripts/check-ci-path-routing.py` and
   its self-test, `bash scripts/check-workspace-policy.sh` exit 0.
5. **The PR run.** On the batch's pull request, `test-debug-a`, `test-debug-b` and `aarch64-debug`
   show the doctest steps, each passing; the `qualification` verdict is green.

*Test value.* No new test. The new steps make the 12 existing `compile_fail` doctests, and the
ones #1416 and #1423 add, able to fail a merge: each is red if the type-level guarantee it states
stops holding, which nothing in CI checks today. D2's twins catch the wrong-reason defect (a
renamed item or a typo in the shared code keeps a lone `compile_fail` green), which nothing else
catches on the stable toolchain.

## Evidence

- Gate 1's doctest lists per leg, gate 2's and gate 3's runs, and the new steps' wall times.
- Every D2 pair (the one construct that differs), its documented error code, and any snippet D2
  or D3 had to fix or delete, with its reason.

## Amendment 1 (root, 2026-10-05)

Attempt 1 stopped before implementation: D2's pinned error codes and gate 3 could not work on the
pinned stable toolchain (Problem, third bullet). Root ruled option (a): D2 is now the twin rule
above, gate 3 is the per-pair mutation above, error codes are comments only, and the "no new
doctests" non-goal is lifted for the twins only. Option (b), `RUSTC_BOOTSTRAP=1` on the doctest
steps, was refused. D1, D3 and D4 are unchanged. The `crates/engine/src/realtime/plan.rs:539`
pair is sequenced after stream B's batch 1 lands on `main`. #1416 and #1423 carry the same twin
rule. Attempt 1 consumed no verdict; the first implementation under this amendment is attempt 1
of the three-attempt budget.

## Dependencies

- None.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt record

### Attempt 1 under Amendment 1 (implementer, 2026-10-05)

Status: implemented for 11 of the 12 pairs; the `crates/engine/src/realtime/plan.rs:539` pair is
**pending** (stream B's area, sequenced after stream B's batch 1 is on `main`). Its
`compile_fail` already runs under the new `test-debug-a` step; only its twin and its comment wait.

**D1.** `test-debug-a` gains "Workspace doctests", `test-debug-b` gains "DSP crates doctests", each
the job's whole-package command with `--all-targets` replaced by `--doc` (same `--exclude`/`-p`
lists and `--features`). `scripts/run-aarch64-tests.sh` debug leg runs
`cargo test --locked --doc "${packages[@]}" --features "$features"` after its main run. No new
job: router and verdict table unchanged.

**D2 pairs** (fence without code; the comment names rustc's error today and says stable rustdoc does
not check it). Reasons were read by flipping every fence to a plain doctest once: each failed with
exactly the stated error and no other, so no snippet needed a fix.

| `compile_fail` | Forbidden construct | Twin | rustc today |
|---|---|---|---|
| graph-compiler `PreparedGraphBuiltinsArtifact` construct | body `PreparedGraphBuiltinsArtifact {}` | body `unimplemented!()` | no code (private fields) |
| mutate | `artifact.graph = panic!(..)` | `let _ = artifact.graph();` | E0616 |
| extract | pattern `{ graph, .. }` | pattern `{ .. }` | E0451 |
| clone_back | `artifact.clone()` | `artifact` (move) | E0599 |
| back_convert | owned `artifact.into()` to `graph::PreparedGraphPlan` | `&graph::PreparedGraphPlan = artifact.graph()` | E0277 |
| generic_internal_attachment | `.attach_internal_bindings(..)` | `let _ = plan;` | E0599 (the method exists nowhere: the guarantee is its absence) |
| host-core `PreparedHost` | bound `T: Sync` | bound `T: Send` (the existing plain doctest, reshaped: shared fn name `requires`) | E0277 |
| host-core `StartedRenderSession` Send | bound `T: Send` | one shared twin, no bound | E0277 |
| host-core `StartedRenderSession` Sync | bound `T: Sync` | same shared twin | E0277 |
| lane `CanonicalFpEnv` Send | bound `T: Send` | one shared twin, no bound | E0277 |
| lane `CanonicalFpEnv` Sync | bound `T: Sync` | same shared twin | E0277 |

The Send and Sync fences of one type share one twin: each differs from it in its one bound only,
so a second identical twin would add a doctest binary and no catch. The graph-compiler fences
previously each used a different fn name; the pairs keep the original names. The wasm32-only
fences (`fpenv.rs`, the portable `CanonicalFpEnv`) got one comment saying they are documentation
only and never compiled by a native run.

**D3.** No doctest failed on a new leg locally (x86-64; `test-debug-b`'s feature set ran lane's
three). AArch64 runs only in CI; its result is gate 5's.

**D4.** `check-test-support-ci.py` already treats `--doc` as narrowing; unchanged.
`test-test-support-ci.py` needed its anchors scoped: the feature lists now occur twice (whole-package
step and doctest step), so mutations target the whole-package step by its command line
(`in_a`/`in_b`). New cases: "test-debug-a narrowed to --doc" and "test-debug-b narrowed to --doc"
must leave the packages uncovered. Mutation: with `--doc` removed from `NARROWING_TARGET_FLAGS`
the self-test is red ("mutation accepted: host-web/test-support removed from test-debug-a",
because the doctest step then counted); reverted, green. `check-ci-path-routing.py` accepted the
new steps unchanged.

**Gate 1.** `test-debug-a` doctest command: exit 0, 18 doctests (plan.rs 1 `compile_fail`;
graph-compiler 6 `compile_fail` + 6 twins; host-core 3 `compile_fail` + 2 twins), about 22 s wall on
a warm `target/`. `test-debug-b` doctest command: exit 0, 3 doctests (lane 2 `compile_fail` + 1
twin), 2.8 s warm. AArch64: pending the PR's `aarch64-debug` job.

**Gate 2.** `#[allow(unsafe_code)] unsafe impl Sync for PreparedHost {}` appended to
`prepare.rs`: `test-debug-a` doctest command exit 101, `prepare::PreparedHost (line 515) - compile
fail ... FAILED`. (The gate's literal line without the `allow` does not compile: the workspace
denies `unsafe_code`, so the lib fails before any doctest.) Reverted. `unsafe impl Sync for
CanonicalFpEnv {}` (x86-64/aarch64 cfg) in `fpenv.rs`: `test-debug-b` doctest command exit 101,
`fpenv::CanonicalFpEnv (line 285) - compile fail ... FAILED`. Reverted.

**Gate 3** (per pair; each run is the job's doctest command filtered to the type; every run exit
101 and reverted). Rename: a typo in the type path shared by the pair's snippets (only those two;
for shared twins, the one `compile_fail` and the twin). Delete: the forbidden construct replaced as
in the table's twin column (or removed).

| Pair | Rename: red | Delete: red |
|---|---|---|
| GC construct (`PreparedGraphBuiltinArtifact` in `use`) | twin line 79 | `compile_fail` line 71 |
| GC mutate | twin line 96 | `compile_fail` line 90 |
| GC extract | twin line 110 | `compile_fail` line 104 |
| GC clone_back | twin line 124 | `compile_fail` line 118 |
| GC back_convert | twin line 139 | `compile_fail` line 133 |
| GC attach (`graph::PreparedGraphPlam`) | twin line 154 | `compile_fail` line 148 |
| PreparedHost Sync | twin line 524 | `compile_fail` line 515 |
| StartedRenderSession Send | twin line 63 | `compile_fail` line 50 |
| StartedRenderSession Sync | twin line 63 | `compile_fail` line 55 |
| CanonicalFpEnv Send | twin line 293 | `compile_fail` line 280 |
| CanonicalFpEnv Sync | twin line 293 | `compile_fail` line 285 |

In every rename run the `compile_fail` itself stayed green, which is the wrong-reason defect the
twin exists to catch. `plan.rs:539`: pending.

**Gate 4.** `check-test-support-ci.py`, `test-test-support-ci.py`, `check-ci-path-routing.py`,
`test-ci-path-routing.py`, `check-workspace-policy.sh`: all exit 0. Also `cargo fmt --all --check`
and the lint job's `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: exit 0.
`shellcheck` is not installed here; `bash -n` passes.

**Gate 5.** Pending the batch's single PR run (wall times of the new steps to be recorded there).
