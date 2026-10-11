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
