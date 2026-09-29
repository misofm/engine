# Measure the console strip against its baseline

Slice S4 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H2, H5 and M4 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Decision 12 accepted known costs on condition that they are measured:
- a padded one- or two-member remainder costs more than per-node rendering at W=8 (H5);
- a bypassed lane runs the wet path;
- a bank's silent fast path needs every lane silent (M4);
- `post_insert` groups split by level (H2).

S0 recorded the baseline on B0's rows. S4 records the same rows on the console shape and reports
the difference.

## Smallest closable slice

On the batch C4 commit where S2 has landed, with C3 (through S1d) already pushed:

- Native: `bash scripts/operator/preflight-console-benchmark.sh --step console-strip-after`, then
  `bash scripts/operator/run-console-benchmark.sh --step console-strip-after`.
- V8: `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
  WORKDIR`, then `run WORKDIR --step console-strip-after`, on the module built from the same commit.

Each run is one invocation, with one warmup and two measured rounds. S4 changes no code. Every row
it needs is already on the new shape, and each has an owner:

| Row | S0 (before) | S4 (after) | Moved by |
|---|---|---|---|
| Strip, N in {9, 10, 13, 16, 64}, and sparse activity (native) | `simd1`/`simd2` racks of the intended fixture | `pre_insert` and `post_insert` | S1a: fixture migration and the `tools/console-workload` builders |
| App shape (native) | `dynamic`: EQ -> compressor, 2-mod-3 bypass | `pre_insert`: EQ -> compressor, same bypass | S1a: the builder, under the `dynamic` clause of its migration rule |
| App shape and N = 64 strip (V8), and the existing mono automation row | old-shape documents, rack bytes `0`/`1`/`2` | console documents, rack bytes `3`/`1` | S1a: documents and the harness's fixture lookup. S1c: the rack codes |

The validator reads the layout-neutral `strip_layout` B0 introduced, so no validator change is
needed. If a row does not run, or needs a validator change, stop and report it as a defect of the
slice named above (B0, S1a or S1c). S4 does not repair it.

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

This is batch C4, after S2.

- *Bind every console slot banked for every track count* (S2).
- *Ship the session console and inserts in the SDK* (S1d).
- *Record the console-strip baseline benchmark* (S0).

## Objective gates

1. Both preflights pass. Both runners accept and promote their records, and the validators pass
   unchanged.
2. There is one invocation per runner: no tuning, no retry, and no projected savings.
3. The before/after table covers every B0 row, with both rounds of both runs.
4. The report names each accepted cost from decision 12 with its measured motion. Where the
   evidence supports one, it gives a recommendation: a per-lane silence skip, an ALAP `post_insert`
   alignment, or #892. It changes no code.
5. Each row's `output_sha256` equals S0's. The design is class A end to end (P1, P2a-P2e, S1a and
   S2 each keep every bit), so a difference is a class-A defect. Report the row and the difference
   instead of explaining it away.
