# Measure the console strip against its baseline

Slice S4 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H2, H5 and M4 in
`.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

## Problem

Decision 12 accepted known costs on condition that they are measured:
- a padded one- or two-member remainder costs more than per-node rendering at W=8 (H5);
- a bypassed lane runs the wet path;
- a bank's silent fast path needs every lane silent (M4);
- `post_insert` groups split by level (H2).

S0 recorded the baseline on B0's rows. S4 records the same rows on the console shape and reports
the difference.

## Smallest closable slice

On the batch commit where S2 and S1d have both landed:

- Native: `scripts/operator/preflight-console-benchmark.sh --step console-strip-after`, then
  `scripts/operator/run-console-benchmark.sh --step console-strip-after`.
- V8: the web mixing benchmark on the shipped artifact built from the same commit.

Each run is one invocation, with one warmup and two measured rounds. The rows are B0's: the strip
at N in {9, 10, 13, 16, 64}, the app shape and sparse activity. Their documents were migrated by
S1a, and the app shape now puts EQ -> compressor in `console.pre_insert` with the same bypass
pattern. The validator reads the layout-neutral `strip_layout` B0 introduced, so no validator change
is needed. If one is, stop and report it as a B0 defect.

Report a before/after table: p50 µs per block for both rounds of S0 and S4, per row. Name:

- the remainder rows' motion against H5's arithmetic (about 9.3 µs per padded bank of the three
  slots against about 4.2 µs per per-node track);
- the app-shape motion (bypassed lanes now run the wet path in shared banks);
- the sparse-activity row against the all-active and idle rows, stating whether a per-lane silence
  skip is warranted. If it is, draft that successor issue. Do not implement it.

State `measurement_control` as recorded and name the operator's lock.

Authorized paths: `artifacts/steps/console-strip-after/**`, the V8 record directory, and this
spec's evidence section.

## Dependencies

- *Bind every console slot banked for every track count* (S2).
- *Ship the session console and inserts in the SDK* (S1d).
- *Record the console-strip baseline benchmark* (S0).

## Objective gates

1. Both runners accept and promote their records, and the validators pass unchanged.
2. There is one invocation per runner: no tuning, no retry, and no projected savings.
3. The before/after table covers every B0 row, with both rounds of both runs.
4. The report names each accepted cost from decision 12 with its measured motion. Where the
   evidence supports one, it gives a recommendation: a per-lane silence skip, an ALAP `post_insert`
   alignment, or #892. It changes no code.
