# Run the release reachable-word sweep of the live input filter in required CI

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-05 as the CI follow-up that *Retarget a live input filter only through its designs
and their mixtures* (#1407) deferred (its Amendment A4, NIT 9: "The release gate-4 sweep workflow
is a separate follow-up issue the coordinator files ... no workflow changes here"). CI only; no
engine code changes. Code anchors verified on `codex/d15-stream-g` at `098499891` (which contains
`main` at `68ef86651`).

## Product outcome

The full stability sweep that proves #1407's live input filter never leaves the hull of its
designs runs on every pull request that can change it, and can fail the merge. Today CI runs only
its debug subset (one rate, four quanta), so a regression that appears only at another launch rate
or at a quantum between 3 and 62 merges green. After this slice the release sweep (every launch
rate, quanta 1-63, 22 histories of 512 blocks) runs in the required `qualification` workflow.

## Context

- **The sweep.** `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs`
  (`crates/builtins/tests/filter_liveness.rs:760-761`) chooses its scale from
  `cfg!(debug_assertions)` (`:762-766`): debug runs 48 kHz at quanta `{1, 2, 7, 63}`; release runs
  `LAUNCH_RATES` (`:340`: 44.1, 48, 88.2 and 96 kHz) at quanta `1..=63`. Each quantum runs 22
  histories (`:767-770`: 16 log-uniform, 2 endpoint, 4 close-retarget) of 512 blocks. The release
  scale needs no environment knob: building the test in release selects it.
- **What CI runs today.** `test-debug-b` (`.github/workflows/qualification.yml:621-648`) runs
  `cargo test --locked --all-targets -p ... -p builtins ... --features ...builtins/test-support...`
  in debug, so it runs the debug subset only. `test-release` (`:650-684`) runs
  `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (`:667`) plus
  the M3 cfg check, the loom leg and the math sweeps; it runs no `builtins` integration test.
  `audit-native` (`:686-`) runs `audit`, `bench` and `console-workload` release tests. No job in
  `qualification.yml` or `nightly.yml` runs `filter_liveness` in release. #1407's own gate command
  was `cargo test --locked --release -p builtins --features builtins/test-support --test
  filter_liveness`, run by hand.
- **Measured cost (2026-10-05, this worktree, x86-64, 32 hardware threads; the existing target
  directory with the release dependencies already built).**
  - `cargo test --locked --release -p builtins --features builtins/test-support --test
    filter_liveness --no-run`: 11.8 s (the test crate and its fat-LTO link only).
  - The sweep alone (`--exact every_reachable_...`): 20.3 s wall, single-threaded inside the test.
  - The whole `filter_liveness` target (14 tests, run in parallel): 19.7 s wall; the sweep is its
    critical path.
- **The CI budget (`qualification` run `37329888826` on `main` at `68ef86651`, 2026-10-05).**
  `test-release` took 254 s (timeout 15 min). The longest jobs were `aarch64-debug` 524 s,
  `audit-native` 483 s and `aarch64-release` 397 s; the verdict waits for the slowest. A GitHub
  `ubuntu-24.04` runner has 4 vCPUs and is slower per thread than the measuring host, so the
  implementer measures the step on the batch's qualification run (gate 2).
- **The CI rules (AGENTS.md, "CI-conscious batch delivery").** `qualification.yml` is the only
  required check; a job that cannot fail a merge does not belong in it; it has no `paths:` filter
  on any trigger; every leaf job is gated by its router's `route` output, and the `verdict` job
  (`:1044-`) holds every job's result to a static expectation table. `nightly.yml` carries jobs that
  gate nothing.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator approved this follow-up of
  #1407: the release gate-4 sweep runs in CI. Rationale: the sweep is the only test that reaches
  every launch rate and every quantum below 64, where #1407's stability bound is tightest (block
  size 1 sets `P(1) = 1.419e-5`); a stability regression there is a merge-blocking defect, and a
  check that only runs by hand does not block anything. Stream G files and owns it.
- **D1. Where it runs: `qualification.yml`, job `test-release`, as one new step.** The sweep
  defends a launch-critical DSP contract (the live input filter's contraction), so it must be able
  to fail a merge; `nightly.yml` would only report a regression after it merged. It fits:
  measured locally at about 32 s (11.8 s build plus 20.3 s run), and even several times that on a
  CI runner keeps `test-release` (254 s) well below the slowest required job (524 s) and its
  15-minute timeout. The step runs the whole `filter_liveness` target, not only the sweep: it
  costs no more wall time (the sweep is the target's critical path) and runs #1407's gates 1-3
  in the release profile too. The step:

  ```yaml
      # #1407 gate 4: the release scale of the reachable-word sweep (every launch rate, quanta
      # 1-63, 22 histories x 512 blocks). test-debug-b runs only its debug subset (48 kHz, quanta
      # {1, 2, 7, 63}).
      - name: Live input filter reachable-word sweep (filter_liveness) in release
        run: cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness
  ```

  placed directly after the `Lane and math gates ... in release` step (`:666-667`), before the
  steps that set their own `CARGO_TARGET_DIR`. No new job, so no router or verdict change: the step
  inherits `test-release`'s `if: needs.route.outputs.route == 'full'`, and every change to
  `crates/builtins` or its dependencies routes `full` (`scripts/ci-path-router.py` routes
  `.github/ISSUE_SPECS/`, `docs/`, `README.md` and `dsp-research/*.md` to evidence and `sdk/` plus
  the SDK script set to `sdk`; none of these is an input of the sweep).
- **D2. The response if the step lengthens the critical path, decided now (amended by Root
  amendment R1 below).** The trigger is wall time: on the batch's qualification run (PR or `main`
  push), `test-release` takes longer than every other required job (it becomes the run's slowest
  job). When that is measured, the release sweeps move out of `test-release` into their own
  parallel **required** job in `qualification.yml`, routed like `test-release`
  (`needs: route`, `if: needs.route.outputs.route == 'full'`), with the `verdict` job's `needs`
  list and expectation table updated to hold it (`full_expected`), and this spec records the
  measurement and the move. The sweep never moves to `nightly.yml`: a check that guards a
  merge-blocking defect (D0) stays able to fail the merge. Neither case changes the test.
- **D3. No test change.** `filter_liveness.rs` already selects the release scale by
  `cfg!(debug_assertions)`, so no environment knob is added; the test file is not edited.

## Deliverables

1. The D1 step in `.github/workflows/qualification.yml` `test-release` (or, under D2, its own
   parallel required job in `qualification.yml` with the verdict's `needs` and expectation table
   updated).
2. Gate 2's measurement recorded in this spec.

## Authorized paths

- `.github/workflows/qualification.yml` (`test-release`'s steps and their comments; under D2, the
  new parallel job, its router `if:` and the `verdict` job's `needs` list and expectation table)
- This spec

## Non-goals

- Changing the sweep, its scale, its bound or any other test.
- The AArch64 legs (`scripts/run-aarch64-tests.sh`): its release leg runs `-p lane -p math` gates
  only; running the builtins sweep on NEON is a separate decision.
- Splitting `test-release` into a new job before D2's trigger is measured (adds a router and
  verdict row for no measured need).
- Rejected alternatives:
  - `nightly.yml`, first or as a fallback: reports a stability regression only after it is on
    `main`, against D0.
  - Scaling the debug subset up in `test-debug-b`: a debug run of the full sweep costs minutes and
    tests the unoptimized arithmetic, not the shipped release code.

## Hazards

- `--features builtins/test-support` builds `builtins` (and the crates that depend on it inside the
  test) a second time in `test-release`'s shared target directory, beside the `wasm-gates` build
  without the feature; the two builds have separate fingerprints and do not invalidate each other.
  `Swatinem/rust-cache` drops workspace crates from its cache, and the step compiles no third-party
  crate the lane/math step has not already built, so the step does not change the cache.
- `scripts/check-test-support-ci.py` (lint job) reads the workflow's `cargo test` steps; it must
  stay green.
- Hot file: `.github/workflows/qualification.yml` (STREAMS.md row `rust-toolchain.toml`,
  `.github/workflows/*.yml`: #877, H #1334, J #1422); this slice edits only
  `test-release`'s steps, so any order works and the later slice rebases.

## Objective gates

1. **The step runs the sweep.** Locally, the D1 command exits 0 and lists
   `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs ... ok` among 14 passed
   tests (15 after #1407's follow-up added gate 8; 14 after #1329 attempt 3 deleted
   `input_tail_is_infinite_while_a_filter_target_is_ramping` on purpose, its gate 4). On the batch's qualification run (PR or `main` push), the `test-release` job shows the
   new step passing.
2. **It fits (D1/D2).** On the batch's qualification run (PR or `main` push): the new step's wall
   time and `test-release`'s job time, against every other required job of that run, recorded
   here. If `test-release` is the slowest required job, D2 applies.
3. **The workflow checks.** `python3 -B scripts/check-ci-path-routing.py`,
   `python3 -B scripts/test-ci-path-routing.py`, `python3 -B scripts/check-test-support-ci.py`,
   `python3 -B scripts/test-test-support-ci.py` and `bash scripts/check-workspace-policy.sh` exit
   0. No job is added, so the verdict's expectation table is unchanged; under D2 the new required
   job is in the `verdict` job's `needs` list and expectation table, and these checks stay green.
4. **The batch's qualification run (PR or `main` push) has a green `qualification` verdict.**

## Test value

No new test. The step makes the existing release-scale sweep able to fail a merge: a change that
pushes a reachable recursion word past its history's designs plus the proven allowance at a launch
rate other than 48 kHz, or at a quantum outside `{1, 2, 7, 63}`, is red in `test-release`, which no
CI job catches today.

## Dependencies

- #1407 (*Retarget a live input filter only through its designs and their mixtures*): the sweep
  and its release scale are #1407's gate 4.

## Attempt record

### Attempt 1 (2026-10-06, implementer; on `codex/d15-stream-g` at `4e41b6296`)

- **Change.** D1 exactly: one new step in `qualification.yml`'s `test-release`, directly after
  `Lane and math gates ... in release` and before the loom step that sets its own
  `CARGO_TARGET_DIR`. No new job, so no router or verdict change. `nightly.yml` and
  `filter_liveness.rs` are not edited (D2 not triggered locally; D3).
- **Gate 1 (local).** The D1 command exits 0 with `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs ... ok`.
  The target now holds **15** tests, not 14: #1407's follow-up added gate 8
  (`every_lane_steps_exactly_four_words_from_its_own_countdown`) to the same binary; all 15 pass.
  The PR-run half of gate 1 is open until the batch push.
- **Gate 2 (local measurement; x86-64, 32 hardware threads, release dependencies already built).**
  - `--no-run` (the test crate and its fat-LTO link only): 13.3 s.
  - Whole `--test filter_liveness` target (15 tests in parallel): 19.8 s wall (test harness
    19.72 s).
  - The sweep alone (`--exact`): 19.9 s wall; gate 8 alone: 0.1 s. The sweep stays the target's
    critical path, so gate 8 adds no measurable wall time.
  - The local total (about 33 s) is well inside D1's budget. The PR-run step time,
    `test-release`'s job time and that run's slowest required job are open until the batch push;
    D2 applies if the step exceeds 240 s or `test-release` exceeds the slowest required job.
- **Gate 3.** `python3 -B scripts/check-ci-path-routing.py`, `python3 -B
  scripts/test-ci-path-routing.py`, `python3 -B scripts/check-test-support-ci.py`, `python3 -B
  scripts/test-test-support-ci.py` and `bash scripts/check-workspace-policy.sh` exit 0. Also
  green: `python3 -B scripts/test-script-reachability.py`, `bash
  scripts/check-artifact-evidence-leak.sh` and a YAML parse of the workflow (`actionlint` is not
  installed on the host). The verdict's expectation table is unchanged.
- **Gate 4.** Open until the batch push (CI cannot run locally).

## Root amendments

### R1 (2026-10-06): D2's trigger and response

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation, after attempt
1's verdict (`MINOR2`). The nightly fallback is dropped: it contradicted D0 (a merge-gating check
never moves to a non-blocking workflow). D2's trigger is now wall time only: `test-release` takes
longer than every other required job of the batch's qualification run (the old trigger "exceeds the
slowest required job" could never fire, because `test-release` is itself required, and the 240 s
limit had no reason of its own). The response is a separate parallel required job in
`qualification.yml`, routed like `test-release`, with the verdict's `needs` and expectation table
updated. Deliverables, Authorized paths, Non-goals and gates 2-4 are amended to match; the gates'
"PR run" is now "the batch's qualification run (PR or `main` push)".

## Follow-up record

### Batch follow-up (2026-10-06, stream G part A; on `codex/d15-stream-g`)

- **MINOR1 (comment).** The #1044 comment above `M3's leg is built with FMA` said "the step above
  runs M3"; after this slice and #1329's `tail_contract` step, the step above is no longer the
  lane/math step. The comment now names the `Lane and math gates ... in release` step. No step was
  moved.
- **NITs folded.** Gate 1 said 15 passed tests (now 14; see the batch note below); the hot-file note no longer names the closed
  J #1427; D1's routing sentence names every path class that routes away from `full`; the
  rust-cache hazard sentence is corrected; gates 2 and 4 name the batch's qualification run.
- **Other release steps now in `test-release` (relevant to D2's trigger).** Beside this slice's
  step, the job now also runs #1329's `tail_contract` step, #1409's `ramp_endpoint` step (seven
  effect crates and `effect-runtime`; local: 11.5 s build, 6.1 s run) and #1366's exhaustive
  `designer_total` sweep (local: 13.1 s including its build, 4.2 s run). Gate 2's measurement on
  the batch's qualification run covers the whole job; if `test-release` then exceeds every other
  required job, D2 moves the release sweeps into their own parallel required job.
- **D2 not triggered by recorded measurements.** No CI run with these steps exists yet; gate 2's
  measurement is pending-CI until the batch push. Locally the added steps cost well under a
  minute against the 254 s baseline job and the 524 s slowest job.

**Batch note (2026-10-06, stream G batch verdict).** The `filter_liveness` target holds 14 tests
at `fa967fe0b`, all passing in release: #1329 attempt 3 (`11f59fa42`) deleted
`input_tail_is_infinite_while_a_filter_target_is_ramping` by design (#1329 gate 4). Gate 1's count
is corrected to 14; the 15 above is the count when gate 1 was first run.
