# Capture continuous spectrum history with block copies

## Product outcome

Continuous spectrum capture appends one sample at a time with a checked add, a ring-wrap branch, two channel-selection branches, a validity byte write and a window-end recompute per sample, preceded by a separate finiteness pass. Do it as at most two slice copies per plane per block plus one publish check. Class A: the captured windows are the same words.

## Root evidence

- `crates/host-core/src/spectrum.rs:2685` `continuous_capture` -> `continuous_append_sample` (`:2521`); `selected_is_finite` (`:2797`) is a separate full pass; the resident variant near `:2728-2790` does checked arithmetic per frame.

## Smallest closable slice

Authorized paths: `crates/host-core/src/spectrum.rs`, its tests, and this spec.

Per block: compute the frames until the next window end once; copy that many frames into the history ring with one or two `copy_from_slice` per plane (wrap), fill the validity bytes for that span, update the filled counter, then run the publish check; repeat for the remainder of the block if a window boundary was crossed. Fold the finiteness check into the copy loop or keep it as one vectorisable pass.

## Non-goals

No change to publish (separate issue), window law, hop, smoothing or the SDK surface.

## Objective gates

1. New test: for random block sizes, hops and selections, the ring contents, validity bytes, counters and every published window are bit-identical to the current implementation across 1 000 blocks including wraps and non-finite inputs.
2. `scripts/check-host-core-policy.sh`, `tools/realtime-policy`; `cargo test -p host-core` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Decision and evidence record (2026-10-24, attempt 1)

- Scope note (root): both entries share one span path. `continuous_append_block` cuts a block at each window end, and `continuous_append_span` writes a span with at most two runs per selected plane and two validity fills. A `ContinuousPlane` enum selects planar `copy_from_slice` or the resident lane's strided scalar copy (R1: one lane of `lanes` is data movement). The resident entry is a thin adapter over `continuous_capture_lane` (plain words), so a unit test drives the lane path without a `rack` dependency. The finiteness pre-pass stays before `continuous_begin`, so a non-finite word still fails the block before any window it completes is published. It is now a branch-free fold. Probes stay in sample units. On a failed block, `validation_samples` counts the whole plane, not the prefix up to the first bad word (only finite inputs assert it).
- R3: `REALTIME_POLICY_END` moved from after the one-shot functions to after `continuous_fail`, so the `continuous_*` functions are covered.
- Gate 1 is a one-time PR comparison against the deleted per-sample code, not a committed test (AGENTS.md). Random differential: 48 runs x 25 blocks = 1 200 blocks, random quantum 1-3000, odd blocks to 4 200, all hops and masks, planar and 4/8-lane resident, NaN/inf injections in selected, unselected and neighbour-lane words, underrun and generation flags, gaps, and skipped reads (drops). It hashes every published window, the ring planes, the validity bytes, every state field and the shared counters after each block. Base and branch digests: `d866542cd9608452` (339 windows, 44 failures), identical.
- Committed claim: the eight continuous oracle tests, unchanged. One assertion was added to `continuous_q192_crosses_a_nonaligned_completion_boundary`: the resident arm (4 and 8 lanes, last lane, non-finite neighbours).
