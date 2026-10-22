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
