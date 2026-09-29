# Add the console-strip benchmark rows

Slice B0 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's verification
`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

## Problem

The console strip changes which tracks bank and what a bypassed or silent lane costs. The
baseline has to be frozen before any engine slice lands, and the standing console benchmark does
not yet have the shapes that move:

- Remainders. Today only the 64-track (eight full banks) and the ragged 9-track (a remainder of one)
  rows exist. Padding (P2a-P2e, S2) changes the cost of every remainder, so the remainder sizes that
  matter need rows.
- The app. The app compiles EQ -> compressor onto every track and marks unselected tracks `bypass`
  (Sol's M8). Under the console strip a bypassed lane stays in its bank and runs the wet path
  (decision 12, "Bypass"), so the app shape must be measured before and after.
- Silence. The silent fast path is bank-wide, so one active track keeps its bank-mates processing
  (M4). The owner accepted that trade-off on condition that a sparse-activity row measures it.
- The validator pins `strip_layout` in rack tokens (`simd1:eq+compressor,simd2:limiter`,
  `scripts/console-benchmark-record-lib.jq`), which S1a retires.

## Smallest closable slice

Add rows to the existing native console benchmark and the existing V8 benchmark. Add no new timed
subject: the `timed_subjects` ratchet counts files (`scripts/check-bench-policy.sh:193`).

Authorized paths:
- `tools/console-workload/**`;
- `tools/bench/src/console.rs`;
- `scripts/console-benchmark-record-lib.jq`, `scripts/console-benchmark-record-validator.jq`,
  `scripts/console-benchmark-validator.jq` and `scripts/test-console-benchmark.sh`;
- `scripts/operator/preflight-console-benchmark.sh` and `scripts/operator/run-console-benchmark.sh`,
  only where the record count or row list is spelled;
- `scripts/web-mixing-automation-benchmark.mjs` and `scripts/run-web-mixing-automation-benchmark.sh`,
  for the two V8 documents;
- this spec.

Rows (native, Simd8, 48 kHz, 128-frame quantum, like the existing console rows):

1. **Strip at N tracks.** EQ -> compressor -> limiter (today's `console-sixty-four-track-intended`
   layout) at N in {9, 10, 13, 16, 64}. 9 (the ragged row, remainder 1) and 64 exist. Add 10
   (remainder 2), 13 (remainder 5) and 16 (two full banks). Derive them in `tools/console-workload`
   from the committed 64-track intended fixture by taking the first N tracks, as the ragged row
   does. Commit no new fixture document.
2. **App shape.** N = 64. Every track carries EQ -> compressor in `dynamic`. The EQ and the
   compressor are bypassed on every track whose index is 2 mod 3 (21 tracks, about one third).
   The pattern is fixed and deterministic.
3. **Sparse activity.** N = 64, the full strip, every second track fed silence. No bank is then
   wholly silent, which is the case the bank-wide skip cannot serve. The report reads it against
   the all-active row and the existing `sixty_four_track_idle` row.

`strip_layout` becomes layout-neutral now, so S4 needs no validator rewrite. It names the chain in
the console vocabulary through decision 12's lowering (`simd1` -> `pre_insert`, `dynamic` ->
`inserts`, `simd2` -> `post_insert`). Today's intended strip is therefore
`pre_insert:eq+compressor,post_insert:limiter`, and the app shape is `inserts:eq+compressor` with a
separate field recording the bypass pattern. The validator accepts, for the app-shape kind, both
`inserts:eq+compressor` (before) and `pre_insert:eq+compressor` (after, once the app shape migrates
to the console). Existing rows' layouts are rewritten by the same mapping.

V8: add the N = 64 strip and the app shape as documents of the web mixing benchmark on the shipped
artifact. Add no per-N V8 rows, because that would be a second framework.

## Dependencies

None beyond decision 12 and the AGENTS.md amendment, which landed with R0.

Merge order: B0 must merge before any engine slice (P1, P2a, S1a), so that S0 can time B0's commit.

## Objective gates

1. `scripts/operator/preflight-console-benchmark.sh --step console-strip-base` passes without
   launching the timed workload.
2. `scripts/test-console-benchmark.sh` passes, with its record counts updated for the new rows.
   Its mutation cases cover the new `strip_layout` spellings and the app-shape bypass field.
3. `scripts/check-bench-policy.sh` passes, and `timed_subjects` is unchanged.
4. Each new row renders deterministic output across the two rounds. The row's `output_sha256` is
   checked by the existing validator rule, not by a new pinned digest.
5. No timed run happens in this slice.

## Non-goals

No engine change. No tuning of existing rows. No isolates for per-node against padded cost; H5's
figures stay arithmetic until S4 reports.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint. Commit on the batch branch in
  CI-conscious batch mode.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value").
