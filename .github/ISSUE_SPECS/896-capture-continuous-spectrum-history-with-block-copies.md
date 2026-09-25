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
2. `scripts/check-host-core-policy.sh`, `scripts/check-realtime-policy.sh`; `cargo test -p host-core` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
