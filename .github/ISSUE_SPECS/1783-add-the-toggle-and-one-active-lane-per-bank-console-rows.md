# Add the toggle and one-active-lane-per-bank console rows

Split from #1107 (D5 item 4, 2026-10-23). Qualification for #1782 (packing).

## Problem

Packing (swap-to-pack) needs two workloads that no row measures: a lane whose activity toggles every few blocks (the hysteresis and swap-budget case, #1107 D4-1), and a console where every bank has exactly one active lane (the best case for packing).

## Smallest closable slice

- Native: two rows in the console benchmark on the standing 64-track console, `sixty_four_track_console_toggle` and `sixty_four_track_console_one_per_bank`, with frozen input patterns, through production paths only; record count and validator updated with only rules a plausible defect can break.
- V8: the same two documents in `scripts/web-mixing-automation-benchmark.mjs`.
- One descriptive run each, before #1782's timing.

## Authorized paths

- `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs`, the console-benchmark scripts
- `scripts/web-mixing-automation-benchmark.mjs`, `scripts/web-mixing-automation-lib.jq`
- this spec

## Objective gates

1. The preflights pass and launch no workload.
2. One native and one V8 descriptive run, attached.

## Test value

The bench unit test turns red when a row's record drifts from its validator; no other test.

## Dependencies

#1107 merged (so the rows read against the whole-bank skip).
