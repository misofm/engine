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

## Evidence and decisions

- Attempt 1 (2026-10-23). The row is a record shape of its own, `console_product_lanes` (workload kind `sixty_four_track_console_product_lanes`), not a new `Workload` variant: a variant would have to enter `tools/bench/src/floor.rs`'s exhaustive floor table, which is outside this spec's paths. It is timed by the session rows' own loop (`SessionMeasurement::run_prepared`, split out of `run_for` so both rows run one loop), measured straight after the session rows, emitted last. The in-run assertion and the aggregate validator hold its digest equal to `sixty_four_track_console`'s over the same window (blocks 0..1000).
- The record count is 66, not 65: the runner counts records of both rounds, so one row adds two (64 -> 66 in `run-console-benchmark.sh` and `preflight-console-benchmark.sh`).
- Gate 1: `scripts/operator/preflight-console-benchmark.sh --step opt3-1758` PASS, workload launches 0, `records_required: 66`.
- Gate 3, descriptive (`run-console-benchmark.sh --step opt3-1758` at ddbdb68ab, one warmup and two rounds, exclusive lock, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` for load above the ceiling, loadavg about 7, cpu 31): `artifacts/steps/opt3-1758/`. All 66 records validated. Rounds 1 / 2, p50 per block: product-lane row 110.69 / 111.07 us, `sixty_four_track_console` 102.91 / 102.97 us, fader-ride quiet arm 112.07 / 111.70 us. Gap: +7.79 / +8.11 us per block, 7.6 % / 7.9 % of the standing row (min per block: 107.17 / 107.68 against 99.61 / 99.91 us). Digests equal (6bae1dd0...), 0 forbidden operations, 0 transactions published.
- Reading: cause 1 holds. Rendered back to back with no alternation, the lanes still cost about 7.7 % of the standing console's block; the ride row's alternation explains at most about 1 us of #1750's 10 us. The gap is at least 2 %, so a successor is due (root files it). No per-stage split: the bench has no stage timer for a lane plan, and `perf record` does not run on this host (`kernel.perf_event_paranoid` 4).
- #1750's note that the quiet arm's digest differs from the standing row's is a block-window difference (the ride row times blocks after its pre-roll); over the same window the bits are equal.
- Candidate stages, unmeasured (code read only): (1) an effect bank stage with a live channel on any lane gets one `BypassShunt` per bank (8 interleaved lanes and 8 banks per console slot on AVX2; 4 lanes and 16 banks on Wasm/NEON) (`crates/rack/src/lib.rs` near 1247-1258). The no-lane plan has a shunt only where an effect is bypassed at preparation. Only the true-peak limiter has latency (`LatencySamples(lookahead_maximum + 6)`, `crates/true-peak-limiter/src/lib.rs` near 263), so only its shunts run `capture` on every block, bypassed or not (`crates/rack/src/lib.rs` near 1944): two plane copies and a PDC delay-line exchange for the bank (`crates/effect-contract/src/live.rs`, `BypassShunt::capture`). The EQ and compressor shunts exist but skip the capture. (2) The live effect bank stage's per-lane drain, target and bypass-mix loops, and the strip input, fader and matrix banks' per-lane dirty-word loads; no bank skips an unchanged snapshot as a whole.
- Successor: #1763 (root, 2026-10-23), because the gap is at least 2 % of the standing row. Verifier attempt 2: PASS.
