# Add a sparse console row to the V8 mixing benchmark

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 3. Qualification work only.

## Problem

The native console benchmark has a sparse row, `sixty_four_track_console_sparse`: the standing
64-track console with every odd track fed exact zeros (`tools/bench/src/console.rs`, issue #1085).
The V8 harness has no such row. Its documents are `nine_track_eq`, `sixty_four_track_console`,
`_app_shape` and `_console_sends` (`scripts/web-mixing-automation-benchmark.mjs`, near `:144-149`).
#1107's gate 3 needs one V8 run of the sparse case, and W=4 is the width the browser ships.

## Smallest closable slice

Add one V8 row, `sixty_four_track_console_sparse`, with the same session and the same odd-track
silence as the native row. The silence comes from the PCM the harness feeds (exact `+0.0`), not
from mute or a session change, so the row times silent input through active strips.

## Authorized paths

- `scripts/web-mixing-automation-benchmark.mjs` and the fixtures or helpers it reads
- this spec

## Objective gates

1. The harness's own preflight (arguments, schema, output persistence, overwrite refusal) passes
   with the new row and does not launch the timed workload.
2. In-run check, as in the native row: the sparse row's output digest differs from both the
   all-active and the idle row's (a sparse row that renders either one's bits feeds the wrong
   input).
3. One descriptive run (one warmup, two measured rounds), attached. No pass threshold.

## Non-goals

No change to the engine, to the native rows, or to any other V8 row.

## Dependencies

None.
