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


## Sol attempt 1 verdict: PASS

Sol, 2026-09-27. I judged `f216649a` merged onto the current batch head `53024efc` (#997 and a
bench record landed after the branch's base `9539c3d9`). The merge is clean (tree `bd133c08`,
scratch only, not kept) and every gate below ran on it. Host x86-64-v3, `CARGO_INCREMENTAL=0`.
The four-lane legs use the research `--cfg miso_native_simd4` lane hunk in their own scratch tree
and target. Nothing was timed. The worktree was left clean.

The change does what the brief asks, and nothing of the failed re-plan survives into the kept
plan. Both gates discriminate, and every claim in the evidence reproduces. The two findings are
Low and do not block.

### Reproduced

* `cargo fmt --all --check` passes, and so does `cargo clippy --locked --workspace --all-targets
  -- -D warnings`.
* `cargo test --locked -p graph-compiler` (dev): 118 passed (lib 88).
  `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler`: 118 passed.
  `cargo test --locked -p console-workload`: 39 passed, and `check-env-vocabulary.sh` is ok.
* On the four-lane build the lib gives 85 passed and 3 failed. The three failures are the known
  scratch-cfg artefacts (`launch_soft_clip_fixture_*`, `misaligned_lane_sets_decline_the_merge`,
  `frozen_issue_037_*`). Both #1001 gates and every #971 test pass there.
* **Mutations.** For each one I ran the whole lib at 8 lanes and at 4 lanes, then reverted it.
  The red sets below leave out the three artefacts.

  | mutation in `banks.rs` | red at 8 and at 4 lanes |
  |---|---|
  | M1: restore `?` on the re-plan bind | both gates (`test.bank.bind_refused`: Simd8 and Simd4 in gate 1, host width in gate 2) |
  | M2: on `Err`, apply the demoted map but keep the trial plan | both: gate 1 on shape (`[2,3]` against the unmoved plan), gate 2 on the mono post-input bank (`[8]` against `[1]`) |
  | M3: on `Err`, drop the trial's banks | gate 2 (`live` 0) |
  | M4: leak the partial re-plan's banks (`mem::forget` on the factory-error return) | gate 2 (`live` 2) |
  | M5: swallow a **trial** bind error (`unwrap_or_default`) | `mixed_twelve_track_plan_binds_…` (`lib.rs:4993`, "factory failure must reject transactionally") |
  | M6: on `Err`, report the re-plan's plan with the trial's banks | both gates (the mono track is missing from the mono pool) |

* **Standing rows, by construction.** I instrumented the re-plan to log each entry and to panic
  on a bind error. I then built all 17 `native_session_rows()` through `build_with_dispatch` at
  `Simd8` and `Simd4`, on both the 8-lane and the 4-lane builds: 68 compiles. **None of them
  reaches the re-plan.** A positive control shows the instrumentation does fire: the dogfood gate
  logs a re-plan and a kept move. The changed statement is therefore never executed on a
  standing row, so row identity follows without a digest comparison. It also agrees with the
  implementer's SHA-256 table.

### The questions asked

1. **Leakage into the kept plan: none.**
   * `classes` is written only in the success arm. The clone and the in-place class rewrite of
     `levels_in` are both locals that are never read after the `if`.
   * `plan_bank_groups` can only return `DuplicateId`. The re-plan uses the trial's ids, so its
     `?` cannot fire.
   * Bank-resource and scratch accounting run later in `compile.rs`, over the returned banks
     only.
   * `bind_homogeneous_bank` takes `&self`. The launch factories keep no shared state apart from
     `thread_local` test counters.
   * On an error, `bind_planned_banks` returns early and drops its local `Vec`. That frees every
     bank the re-plan bound, on the compile (control) thread; M4 shows gate 2 would catch a leak.
2. **Errors.** A trial bind error still fails the compile exactly as before (`bind(&plan)?` is
   untouched, and M5 is red). On the re-plan, **every** `Err` kind is swallowed. Under #95's
   three-outcome rule none of them means "this cohort cannot bank", because that answer is
   `Ok(None)` and never reaches this branch. What reaches it:
   * a factory contract violation: the factory's `Err`, or `graph.effect.bank_metadata`;
   * graph-compiler's own `graph.internal.invariant`, from `bindable_slot_members`;
   * a scratch overflow, which depends only on width and quantum and is harmless to drop here.
   
   The owner's #971 ruling chose to keep the unmoved plan, and the result is always the valid
   move-disabled plan. I accept it; see findings 1 and 2.
3. **Gates.** Both are discriminating (see the table). Gate 1's reference is the bank-free
   registry's compile. That is the move-disabled plan, because there both the trial and the
   re-plan bind 0 banks. In gate 2, `bound == 2` together with `live == 1` proves that the second
   group errored rather than declined: had it bound, `bound` would be 3 and the move kept.

### Findings, by severity

1. **Low (contract text).** `crates/effect-contract/src/lib.rs:1513`, the frozen #95 table, still
   says that in `graph-compiler` an `Err(code)` "fails the whole graph compile". The new comment
   (`banks.rs:264-268`, and the spec's product outcome) cites #95 as "a cohort a factory cannot
   bank never costs the user the compile". #95 says that about `Ok(None)`. It defines `Err` as a
   contract violation that must fail the compile.
   *Scenario:* an effect's bank path rejects a member that `prepare` accepts, which is a real
   defect under the table. If that member only ever completes a full group through the re-plan
   (a lone stranded mono track), the session compiles silently. The table promises a refusal, so
   the defect surfaces only when the member later lands in a trial group.
   *Fix (one sentence each, can ride the batch or #1002):* amend the table's `graph-compiler`
   cell to name the speculative-re-plan exception, and cite #95 correctly in the comment.
2. **Low (observability).** The swallow is total and leaves no trace. There is no counter or
   report field, and the compiler has no channel for non-fatal notes, as the evidence says. So
   an internal `graph.internal.invariant` raised by a planner regression that only affects
   demoted classes would not be seen. The #971 gates cover a *systematic* re-plan failure, which
   would show on the dogfood gate as banks 27 against 30. A session-specific one would render
   the move-disabled plan with no signal. I recommend no change now. If a notes channel ever
   exists, this is its first entry.
3. **Informational: release test builds (separate tooling issue).** Reproduced; see below. It is
   independent of this change.

### Release-test clobber: problem statement for a tooling issue

**Problem.** `[profile.release] panic = "abort"` plus Cargo's unwind-only test harnesses mean
that one `cargo test --release` invocation can build two panic variants of a lib. `effect-package`
(`rlib`+`cdylib`), `capi` (`rlib`+`staticlib`+`cdylib`) and `host-web` (`rlib`+`cdylib`) get
un-hashed output names, so the two variants write the same `target/release/deps/lib*.{rlib,so}`.
Cargo warns `output filename collision … (rust-lang/cargo#6313)`, and a dependent then fails.

**Reproduced** on batch head `53024efc`, toolchain 1.97.1, in a fresh target each time:

* `cargo test --locked --release -p graph-compiler`: ``error[E0463]: can't find crate for
  `effect_compiler` `` in the lib test and the `graph_fixture` test. A second fresh run failed in
  the abort `graph_fixture` bin instead, with E0463 for `graph_compiler` and `effect_compiler`.
* `-p session-validator`: ``error[E0460]: found possibly newer version of crate
  `effect_package` ``.
* `-p native-pcm-runner` (collides on `capi` and `effect-package`) and `-p parameter-metadata`
  (collides on `effect-package` and `host-web`): E0463 inside `host-core`.

**Scope.** A `--unit-graph` check of `cargo test --release -p <pkg>` for all 45 workspace
packages finds exactly these four with both panic variants of a cdylib or staticlib lib. The
other 41 have one variant.

**Doc contradiction.** `docs/REALTIME_DEPENDENCY_POLICY.md:280` says per-package invocations
"never put two panic variants of a clobbering lib unit in the same run". That is false for these
four packages.

**CI.** No release `cargo test` command in `qualification.yml` hits it. That covers
`test-release`'s lane/math/wasm-gates, `m3_determinism`, the loom leg and m1/f1, and
`audit-native`'s `-p audit -p bench -p console-workload`. Nightly's four `--ignored` release
tests do not hit it either: each builds one variant. So CI is green, but nothing runs these four
packages' tests in release. A later separate invocation heals itself: Cargo marks the unit dirty
("the profile configuration changed"), which I checked on a toy workspace. The failure is
therefore confined to one invocation.

**Workaround.** `CARGO_PROFILE_RELEASE_PANIC=unwind`.

**Gate for the issue.** Either the four packages' `cargo test --locked --release --no-run` builds
in a fresh target, or the policy doc names them and the supported invocation, and
`check-release-shape.py` pins that set. The owner's deferred `dist`-profile decision is the
structural option.
