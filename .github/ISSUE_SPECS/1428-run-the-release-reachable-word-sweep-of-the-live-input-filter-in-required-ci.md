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
  implementer measures the step on the PR run (gate 2).
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
  `crates/builtins` or its dependencies routes `full` (`scripts/ci-path-router.py`: only
  `.github/ISSUE_SPECS/` and `docs/` paths route away from it).
- **D2. The fallback, decided now.** If gate 2 measures the new step above 240 s on the PR run, or
  `test-release` exceeds the slowest required job of that run, the step moves to `nightly.yml`
  instead (a new job `filter-liveness-release-sweep`, the same command, `needs` nothing, joining
  `failure-notice`'s `needs` list), and this spec records the measurement and the move. Neither
  case changes the test.
- **D3. No test change.** `filter_liveness.rs` already selects the release scale by
  `cfg!(debug_assertions)`, so no environment knob is added; the test file is not edited.

## Deliverables

1. The D1 step in `.github/workflows/qualification.yml` `test-release` (or, under D2, the
   `nightly.yml` job).
2. Gate 2's measurement recorded in this spec.

## Authorized paths

- `.github/workflows/qualification.yml` (`test-release`'s steps only; under D2,
  `.github/workflows/nightly.yml` instead)
- This spec

## Non-goals

- Changing the sweep, its scale, its bound or any other test.
- The AArch64 legs (`scripts/run-aarch64-tests.sh`): its release leg runs `-p lane -p math` gates
  only; running the builtins sweep on NEON is a separate decision.
- Splitting `test-release` into a new job (adds a router and verdict row for no measured need).
- Rejected alternatives:
  - `nightly.yml` first: reports a stability regression only after it is on `main`.
  - Scaling the debug subset up in `test-debug-b`: a debug run of the full sweep costs minutes and
    tests the unoptimized arithmetic, not the shipped release code.

## Hazards

- `--features builtins/test-support` builds `builtins` (and the crates that depend on it inside the
  test) a second time in `test-release`'s shared target directory, beside the `wasm-gates` build
  without the feature; the two builds have separate fingerprints and do not invalidate each other.
  The `Swatinem/rust-cache` key (`shared-key: test-release`) then caches both.
- `scripts/check-test-support-ci.py` (lint job) reads the workflow's `cargo test` steps; it must
  stay green.
- Hot file: `.github/workflows/qualification.yml` (STREAMS.md row `rust-toolchain.toml`,
  `.github/workflows/*.yml`: #877, H #1334, J #1422, J #1427); this slice edits only
  `test-release`'s steps, so any order works and the later slice rebases.

## Objective gates

1. **The step runs the sweep.** Locally, the D1 command exits 0 and lists
   `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs ... ok` among 14 passed
   tests. On the PR or batch run, the `test-release` job shows the new step passing.
2. **It fits (D1/D2).** The PR run's new step wall time and `test-release`'s job time, against
   that run's slowest required job, recorded here.
3. **The workflow checks.** `python3 -B scripts/check-ci-path-routing.py`,
   `python3 -B scripts/test-ci-path-routing.py`, `python3 -B scripts/check-test-support-ci.py`,
   `python3 -B scripts/test-test-support-ci.py` and `bash scripts/check-workspace-policy.sh` exit
   0. No job is added, so the verdict's expectation table is unchanged; under D2 the nightly job is
   added to `failure-notice`'s `needs`.
4. **The PR run's `qualification` verdict is green.**

## Test value

No new test. The step makes the existing release-scale sweep able to fail a merge: a change that
pushes a reachable recursion word past its history's designs plus the proven allowance at a launch
rate other than 48 kHz, or at a quantum outside `{1, 2, 7, 63}`, is red in `test-release`, which no
CI job catches today.

## Dependencies

- #1407 (*Retarget a live input filter only through its designs and their mixtures*): the sweep
  and its release scale are #1407's gate 4.

## Attempt record
