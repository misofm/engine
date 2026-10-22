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

## Decision record (attempt 1)

- **Which documents.** The `:144-149` anchor (also cited by #1107) is the boot-cost mode's
  `REBUILD_DOCUMENTS`, which times `miso_engine_web_v1_boot`. #1107's gate 3 needs render cost,
  so the row joins the render mode's console-strip `documents` (today three: the standing console,
  the app shape, the sends console), appended fourth and timed in the same per-observation
  alternation. The boot-cost mode is unchanged.
- **How the silence is fed.** The native row feeds each track its own block in code. The browser
  feeds sources, and the standing fixture (`console-sixty-four-track-intended.json`) binds all
  sixty-four tracks to its one source. So the harness derives the row's document in code (as the
  native row is "derived", `synthetic_fixture: true`): a second source of the same shape,
  `fixture-source-silent`, and every odd track's `source_id` rebound to it. The harness feeds it
  exact `+0.0` on both planes every block, at the tone source's frame. No strip, fader, mute, route
  or parameter changes; the harness asserts that the derived document, its odd tracks bound back
  and the second source dropped, deep-equals the standing one. The cost the row adds over the
  standing console is one more source per block: its submit is outside the clock, but its
  consumer step (`begin_block`: one ring pop and recycle, no PCM copy) is inside the clock. That
  constant counts against the sparse row, and the native row (fed without rings) does not have it.
- **For #1107 (W=4 reading).** Read the gain as sparse minus standing, with that constant included,
  and with the standing V8 difference from native (every V8 document feeds one tone phase; the
  native row gives each track its own phase). The harness writes exact `+0.0`; nothing checks the
  sign, so a feed change that wrote `-0.0` would hide #1107's skip without moving a digest.
- **Sol attempt 1 verdict: PASS.** The verifier re-ran the four harness mutants and the two new
  validator cases (each red on its revert); the three existing documents' preflight digests are
  unchanged against main's harness.
- **Gate 2 without an idle row.** The V8 harness has no idle row. In preflight and in the run the
  sparse row is asserted audible (some output word nonzero, so not silence, which is what the
  native idle row renders) and distinct from every other document, the standing console included.
  Because an underrun also renders `+0.0`, and no digest can tell it from fed silence, both of the
  row's sources are also asserted to report zero cumulative underrun frames (seek report, #1293 D6).
- **Validator.** `scripts/web-mixing-automation-lib.jq` now requires four documents, the fourth
  `sixty_four_track_console_sparse` on the standing console's fixture and facts with
  `input_signal == "odd_tracks_silent"`; the first three stay `tone`.
- **Cross-set edit needed.** `scripts/test-console-benchmark.sh` (#1750's file set) builds a
  three-document synthetic web record (`web_round`, near `:1490`) that this validator now refuses.
  It needs a fourth document entry appended, plus two cases (no sparse document; a sparse
  document claiming `tone`). Root authorized this edit and declared the order: #1744 edits the
  file and merges first, and #1750 takes `main` after that. The edit is applied on this branch;
  the suite passes, and each new case went red on its claim's revert (checked in a scratch
  worktree).

## Evidence (attempt 1)

- Harness mutants (preflight, each restored): no submit on the silent source -> "underran";
  the tone on the silent source -> "rendered the same bits" as the standing console; zeros on
  the tone source -> "silent"; no track rebound -> "binding". The three existing documents'
  preflight digests are unchanged from `main`'s harness.
- Descriptive run, commit `8670f98d0`, module `e7567184...` (not the release pin), one warmup
  and two measured launches, cpu 31, uncontrolled (loadavg 2.19, waived), exclusive lock:
  `artifacts/steps/1744-v8-sparse/web-mixing-automation.jsonl`, validator accepted. Per-block
  p50 / p95 / p99 in microseconds, rounds 1 and 2:

  | document | round 1 | round 2 |
  |---|---|---|
  | sixty_four_track_console | 242.0 / 254.2 / 262.8 | 247.0 / 290.1 / 295.0 |
  | sixty_four_track_console_sparse | 247.0 / 258.4 / 261.8 | 250.9 / 294.6 / 299.9 |

  Today the sparse row costs no less than the all-active row: the browser module does no
  per-lane silent work skipping (that is #1107).
