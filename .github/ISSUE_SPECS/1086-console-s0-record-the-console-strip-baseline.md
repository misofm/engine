# Record the console-strip baseline benchmark

Slice S0 of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`).

## Problem

S4 reports the console strip against a baseline. The baseline has to be the engine before any
console slice changes banking, bypass or the schema, measured on the rows B0 froze.

## Smallest closable slice

Time B0's merge commit, once:

- native: `scripts/operator/run-console-benchmark.sh --step console-strip-base`, after
  `scripts/operator/preflight-console-benchmark.sh --step console-strip-base` passes;
- V8: `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
  WORKDIR`, then `run WORKDIR --step console-strip-base`, for the existing row and B0's two
  documents, on the module built from the same commit.

Each run is one invocation, with one warmup and two measured rounds. If a later console slice has
merged by then, time a clean detached worktree of B0's commit; the runner refuses a dirty tree.

Report the `measurement_control` field as the records carry it. The runner takes no lock, so name
the operator's `flock` (or its absence) and the host's background load in the disposition.

Authorized paths: `artifacts/steps/console-strip-base/**`, the V8 record directory the runner
writes, and this spec's evidence section.

## Dependencies

- *Add the console-strip benchmark rows* (B0, #1085).

This closes batch C1 (R0, B0, S0). Push C1 with S0's records before any engine slice lands, so the
baseline is recorded on the unchanged engine.

## Objective gates

1. Both runners accept and promote their records, and the validators pass.
2. The records name B0's commit as `candidate_commit`.
3. No tuning, no retry and no second invocation. If post-workload tooling fails, preserve the raw
   output, record the failure and open a tooling issue (AGENTS.md benchmark rules).
4. The evidence section tabulates, per row, p50 µs per block for both rounds and the
   sparse-activity row against the all-active and idle rows. Quote no projection.

## Non-goals

No interpretation beyond the table. S4 does the before-and-after comparison.
