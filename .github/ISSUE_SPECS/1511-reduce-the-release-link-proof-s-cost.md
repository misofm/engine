# Reduce the release link proof's cost

## Product outcome

The nightly "workspace release link proof" stays well inside its timeout as the workspace grows, and its
time limit is re-derived from measurements on the runner, not raised again by guesswork.

## Context

- The link proof builds about 306 release test executables under `[profile.release]` (`lto = "fat"`,
  `codegen-units = 1`, `debug = 1`). Its build grew from about 28 to about 40 minutes in nine days
  (2026-09-29 to 2026-10-09): #1051's randomized step (about +5 min), about 48 more release test
  executables, and runner variance (the same commit, 1b55de30a, built in 39.6 min on 10-09 and 26.2 min
  on 10-10). On 10-09 it hit its 45-minute timeout.
- The predecessor issue raised the timeout to 60 minutes as hang protection, recorded runner hardware
  (`nproc`, `lscpu` model), and corrected two false comments. This issue owns the cost.
- The randomized step recompiles for 2-3.5 minutes each night although its comment said "nothing is
  rebuilt". `MISO_ENGINE_RANDOMIZED_SCALE` and `PROBE_966_COUNT` are read only at run time and no crate
  uses `option_env!` or `rerun-if-env-changed`, so the cause is unconfirmed.
- `docs/REALTIME_DEPENDENCY_POLICY.md:460-463`: the link proof's claim is a compile-time failure
  (the clobber); its build settings are D12's release profile.

## Decisions to make in this issue

- D1. Measure: run `cargo build --timings` for the link proof on the runner and upload the HTML; split
  compile time from link time per executable.
- D2. Decide, with a recorded reason, whether the link proof's claim needs fat LTO, `codegen-units = 1`
  and `debug = 1` for every test executable. Any override of D12's release profile for this job is a
  recorded decision, not a silent change.
- D3. Make the randomized step reuse the build: diagnose the rebuild with
  `CARGO_LOG=cargo::core::compiler::fingerprint=info` and remove its cause.
- D4. Re-derive `timeout-minutes` from the new runner measurements (slowest observed runner times a
  stated margin).

## Trigger

If any night's link proof exceeds 50 minutes before this issue lands, it blocks the next batch.

## Gates

- G1. A dispatched nightly records, against the old and new settings, build and link minutes, the
  randomized step's rebuild (zero after D3) and the runner CPU model.
- G2. The link proof still fails red on its own seeded clobber (the claim it exists for).

## Authorized paths

`.github/workflows/nightly.yml` (the link-proof job), `scripts/run-release-workspace-tests.sh`,
`Cargo.toml` `[profile.*]` only through a recorded D2 decision, this spec.

## Decision and evidence record (attempt 1, 2026-10-24)

Scope note D1-D3 accepted by root. Root also authorized one paragraph of
`docs/REALTIME_DEPENDENCY_POLICY.md` (after the `release-link-proof` paragraph) and the header of
`scripts/run-release-workspace-tests.sh`. `Cargo.toml` is not changed.

- Trigger fired: build step 53, 56, 37, 60, 57, 58, 55 min on 10-17..10-23 (5 of 7 nights red).
- D2 (root): job-level env on `release-link-proof` only: `CARGO_PROFILE_RELEASE_LTO=thin`,
  `CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16`, `CARGO_PROFILE_RELEASE_DEBUG=0`. Argument: the clobber
  needs (a) two panic variants of one unit in one `cargo` run, which comes from `[profile.release]`'s
  `panic = "abort"` plus Cargo's unwind override for test harnesses under `--workspace --all-targets`,
  and (b) a `cdylib`/`staticlib` crate-type, whose output filename is not hashed. The three
  overridden keys change neither the variant count nor any filename. `thin` is still a Cargo-level
  LTO setting, so Cargo's per-unit LTO modes (and so the unit graph) keep the shape fat LTO gives
  them; `lto = false` would not. The `cfg(debug_assertions)` residual depends on `debug-assertions`,
  which is not overridden. The script's command and its `CARGO_PROFILE_RELEASE_PANIC=unwind` are
  unchanged, so G2 holds by this argument plus `scripts/check-release-shape.py`'s crate-type pin
  (scope note recommendation; root decides whether a verifier runs the red build). Residual: the
  100x randomized differentials now run under thin LTO; their per-PR seeds still run under fat LTO
  in `qualification.yml`'s `-p` legs, so a defect visible only under fat-LTO inlining at 100x seeds
  would be missed.
- D1: the build step passes `--timings`; `target/cargo-timings/*.html` is uploaded (`if: always()`)
  as artifact `release-link-proof-timings`.
- D3 (one bounded diagnosis, local): after the job's build, `cargo test --release --workspace
  --test randomized --no-run` recompiled 72 units in 28 s; `--lib` 0 units; `--test bank_levels` 1
  unit (8 s). `CARGO_LOG=cargo::core::compiler::fingerprint=info` shows new unit hashes (no stale
  fingerprint): engine's two fingerprints have equal features and different `profile` hashes. The
  rlibs differ in content: `libcfg_if-e5f1142678c3b138.rlib` (randomized run) holds LLVM bitcode
  only, `libcfg_if-42a4392d5ee6d5ca.rlib` (all-targets build) an ELF object. Cause: Cargo computes
  each unit's LTO mode from the invocation's root targets; test-only roots give bitcode-only deps.
  Not a one-line fix; residual recorded in the job comment and stopped there.
- D4: `timeout-minutes` stays at a provisional 60. Root sets the final value after one dispatched
  run (slowest observed new build x 1.5, rounded up to 5 min). Not dispatched by the implementer.
- Local proof (one run, exclusive lock, load 7.56 at start, AMD EPYC 7313P, 32 jobs): the job's
  build command under the job's env passed in 8 m 58 s wall (cargo timings 537.6 s, 724 units;
  slowest units `wasm-gates` bin 207 s, `capi` lib test 152 s). No old-settings local run (cost).
