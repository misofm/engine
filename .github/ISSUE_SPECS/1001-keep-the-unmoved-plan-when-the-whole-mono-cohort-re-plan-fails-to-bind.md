# Keep the unmoved plan when the whole-mono-cohort re-plan fails to bind

## Product outcome

#971 keeps the mono pool a whole number of cohorts by re-planning the stranded remainder and binding both plans to compare the banks they actually produce. A bank-bind error on the speculative re-plan fails the whole compile (`crates/graph-compiler/src/banks.rs`, `bind(&replan)?`). The #971 attempt-2 verification showed this is real: with a test factory whose bank bind always errors, a session of `ch00` mono and `ch01..` stereo carrying only `dynamic: [comp]` compiles with the move disabled (the unmoved plan has no full group and never calls bind), and is refused at head, at 8 and 4 lanes. No shipped factory can trigger it today (all eight launch factories' bank paths were read), but an optimisation's trial plan must never cost the user the compile. This matches #95's stance that a cohort a factory cannot bank must not fail the compile.

## Smallest closable slice

On a `bind(&replan)` error, keep the unmoved plan and its already-bound banks, and continue. No diagnostic is needed for correctness; if the compiler already records non-fatal planning notes, add one.

## Objective gates

1. The erroring-factory session above (a test-only factory) compiles at `Simd8` and `Simd4`, keeps `ch00` mono, and renders the bits of the move-disabled plan.
2. Mutation: restoring `?` on the re-plan bind turns gate 1 red.
3. Every #971 gate, the #971 discriminating pool test and every standing console digest are unchanged; `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` pass.

## Attempt 1 evidence

Implementer attempt 1, 2026-09-27. Branch `codex/1001-replan-error-keeps-plan`, base
`9539c3d9` (the batch head, carrying #971), code commit `f4e8c0ac`. Host x86_64
(`x86-64-v3`), `CARGO_INCREMENTAL=0`. The four-lane legs use the research `--cfg
miso_native_simd4` lane hunk in `target/simd4`, applied for measurement only and reverted. No
timing was run.

### The change

In `crates/graph-compiler/src/banks.rs`, `bind(&replan)?` became `if let Ok(..) =
bind(&replan) && replan_banks.len() > banks.len()`. On an error, the trial plan, its bound
banks, its report slots and the unmoved `classes` map are all kept. The re-plan's
`bind_planned_banks` returns early, so the banks it bound before the error are dropped with its
local vector. The demoted map was only ever a clone, and the builtin-stage planner reads the
unmoved one. The re-plan's `plan_bank_groups` keeps its `?`: it can only fail with
`DuplicateId`, which is unreachable once the trial succeeded on the same ids. The compiler
records no non-fatal planning notes, so no diagnostic is added.

### Gates (`crates/graph-compiler/src/lib.rs` tests)

The test-only `ErroringBankFactory` wraps the launch compressor, delegating `prepare`. Its bank
bind errors (`test.bank.bind_refused`) on every group, or only on a group whose members carry a
marker parameter. Any other group binds the real bank, wrapped in `CountedBank`: every trait
method delegates, and `Drop` counts the bank out. A new helper,
`try_compile_console_model_at`, compiles at an explicit dispatch and returns the diagnostics.

1. `a_replan_that_fails_to_bind_keeps_the_unmoved_plan` is Sol's session: `W` tracks, `ch00` mono,
   the rest stereo, `dynamic: [comp]` only, and a bind that always errors. It is compiled at
   `Simd8` **and** `Simd4` explicitly on any host, and both widths compile before anything is
   asserted. At each width:
   * it compiles;
   * `ch00` stays in the mono pool;
   * 0 effect banks, and the factory never produced one;
   * against the bank-free registry's compile of the same session, which is the move-disabled
     plan here (the re-plan binds nothing, so there is no move, and there is no effect bank): the
     same pools, the same `bank_shape`, the same collapse counters and the same PCM bits over 12
     armed blocks, and not silent.
2. `a_replan_that_fails_part_way_drops_its_banks` runs at the host width, because the real
   compressor banks only at the width the build executes. `ch00..ch{2W-2}` are stereo and
   `ch{2W-1}` is mono with the marker threshold. The trial binds the stereo cohort
   `ch00..ch{W-1}`. The re-plan binds a second bank over the same cohort, then errors on the
   group holding the marker. After the compile:
   * `bound == 2`, so the re-plan really did bind before failing;
   * `live == 1`, so only the trial's bank is alive;
   * the artifact's one effect bank is `ch00..ch{W-1}`;
   * the mono track is still mono in the rack plan, and its post-input builtin bank holds it
     alone, so the builtin planner read the unmoved map and nothing was half-applied;
   * after the artifact is dropped, `live == 0`, so nothing leaked.

**Mutation (restore `?` on the re-plan bind): both gates red at 8 and at 4 lanes.** Gate 1
reports both widths refused with `test.bank.bind_refused`. Gate 2 is refused at the host width,
which is `Simd8` natively and `Simd4` under the scratch cfg.

### Commands

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` (also with `--all-features`) | pass |
| `cargo test --locked -p graph-compiler` (dev) | 118 passed, 0 failed (lib 88) |
| `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler`, own target directory | 118 passed, 0 failed |
| the #971 gates (all in the graph-compiler lib, including the discriminating pool test) | pass, dev and release |
| 4 lanes (scratch cfg): the two #1001 gates, the eight #971 tests, `the_two_planners_agree_on_every_track_class` and `class_pooling_forfeits_the_route_fold_only_on_an_interleaved_session`; whole lib | 11 of 11 pass. The whole lib has 85 passed and 3 failed; the three failures (`launch_soft_clip_fixture_*`, `misaligned_lane_sets_decline_the_merge`, `frozen_issue_037_*`) are the scratch-cfg artefacts Sol recorded at base in #971 |
| `cargo test --locked -p console-workload` | 39 passed |
| `bash scripts/check-env-vocabulary.sh .` | ok |
| the 17 `native_session_rows()`, 64 blocks, at `Simd8` and `Simd4`, base `9539c3d9` against head | shape, transposes, collapse, folds, redirects and SHA-256 identical row for row at both widths |

### Deviation: release tests need `panic = "unwind"`

A plain `cargo test --release -p graph-compiler` does not build on this workspace, and that is
independent of this change. The release profile sets `panic = "abort"`, so a release test build
compiles each dependency twice: once with abort for the `graph_fixture` binary and once with
unwind for the test harness. `effect-package` declares `crate-type = ["rlib", "cdylib"]`, so
Cargo gives its rlib no hash suffix (`libeffect_package.rlib`), and the two variants overwrite
each other. `effect-compiler` then fails against the wrong one. The error varies from run to
run: `E0463` "can't find crate", or `E0277`/`E0308` inside `effect-compiler`. It reproduces with
`-j1`. The release leg above therefore overrides `panic` to unwind, which keeps opt-level 3 and
fat LTO. The collision may deserve its own issue.
