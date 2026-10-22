# Add a console benchmark row that rides a fader and a pan

Found in #890 (2026-10-22). Qualification work only.

## Problem

`console_automation` (`tools/bench/src/console.rs`, near `:129-140` and `:2021-2022`) automates one compressor threshold. Every fader, mute and matrix stays settled in its timed region, so no benchmark row reaches the ramping output section. #890's fused ramping pass (`fader_matrix_ramp_block`) therefore has no row that can show its cost, and neither will later work on that path.

## Smallest closable slice

One native row, `sixty_four_track_console_fader_ride`, on the standing 64-track console: per block, a live fader move on one track per bank and a pan (matrix) move on one other track per bank, sent as ordinary control values so each opens a ramp. Inside the timed region at least one lane of every bank has a fader or matrix ramp in flight.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/` (fixtures or helpers it needs)
- `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh` (only to accept the new row)
- this spec

## Objective gates

1. The bench preflight passes with the new row and does not launch the timed workload.
2. In-run checks: a test-support witness or counter shows the ramping output path ran in every timed block; the row's digest differs from `sixty_four_track_console`'s; push counts are exact.
3. One descriptive run (one warmup, two measured rounds), attached. No pass threshold.

## Non-goals

No engine change. No V8 row (a later issue may add one).

## Dependencies

#890 merged, so the in-run witness can name the fused ramping path.

## Evidence and decision record

- 2026-10-22, attempt 1 (implementer), stopped before code: #890's fused ramping pass is reachable only from `FaderMatrixBankProcessor::process` (`crates/builtins-compiler/src/lib.rs`, the one caller of `try_process_ramping_with_matrix(_and_operations)`). That pair exists only when `make_fader_matrix` sees both banks at `BuiltinControlDelivery::BetweenRenderCalls`; it declines `Concurrent`. No production host prepares `BetweenRenderCalls`: the C ABI calls `prepare_host_runtime_with_live_lanes(_successor)`, which passes `between_render_calls = false`, and the browser's `prepare_web_host` calls no `_between_render_calls` entry since #1400b (13fd9b491). Only tests call the host-core `_between_render_calls` entries. So a row whose gate 2 needs the fused ramping witness must prepare a delivery no product runs, and a row on the production (`Concurrent`) delivery takes the split fader/matrix ramp path and cannot see #890. The two conflict with "benchmark real paths only"; the choice is root's. The metered row's doc (`tools/bench/src/console.rs` near `:103-118`) still calls `BetweenRenderCalls` the browser's delivery, which is stale for the same reason.
