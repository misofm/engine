# Measure what product live lanes cost a console that nobody edits

Found in #1750 (2026-10-22).

## Problem

In #1750's descriptive run, the `quiet` arm of `sixty_four_track_console_fader_ride` (the standing 64-track console prepared with the lanes a product host attaches, `controls::live_lanes(model)`: fader/matrix, strip input and effects; no writes) took a p50 of 112.1 / 112.3 us per block. `sixty_four_track_console` (the same session with no live lanes) took 102.4 / 102.1 us in the same run. The #1750 verifier proved the two plans render bit-identical output (1018 blocks, every lane subset), so the gap is work or cache, not signal.

Two causes are possible, and the run cannot separate them:

1. the live lanes add render work on every block (input-lane and effect-lane cell reads on every strip and effect), which every browser and C ABI session pays;
2. the ride row alternates two whole plans in its timed loop (quiet, then riding), while the standing row renders one plan back to back, so cache residency differs.

If cause 1 holds, about 10 % of a 64-track console's render is spent reading controls that did not change.

## Smallest closable slice

Measure first, change nothing in the engine.

- One native row, `sixty_four_track_console_product_lanes`: the standing console prepared exactly as the ride row's quiet arm (product lanes, no writes), rendered back to back like `sixty_four_track_console`. One descriptive run.
- Report its p50 against `sixty_four_track_console` in the same run, and a per-stage split if the bench's existing tools give one (or a `perf` profile where `perf record` runs).
- If the gap is at least 2 % of the standing row: file a successor that names the stage and a mechanism (for example skipping an unchanged control snapshot per bank), with the measurement attached. If it is below 2 %, record it and close.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs`
- `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh`, the runner's record-count pin in `scripts/operator/run-console-benchmark.sh` and `scripts/operator/preflight-console-benchmark.sh`
- this spec

## Objective gates

1. The bench preflight passes with the new row and launches no workload.
2. In-run checks: the row's output digest equals `sixty_four_track_console`'s over the same block window (the plans must render the same bits); 0 forbidden operations.
3. One descriptive run (one warmup, two measured rounds), attached.

## Dependencies

#1750 merged (it adds `SessionRuntime::build_with_product_lanes`).
