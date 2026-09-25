# Bind partial compressor cohorts as banks with exact identity lanes

## Product outcome

Same as the EQ issue, for the compressor: a cohort with fewer than W compressor tracks runs each compressor scalar per node. Let the compressor accept exact identity lanes so a partial cohort binds as one bank. Banking must not move a bit.

## Root evidence

- `crates/graph-compiler/src/banks.rs:214-222` (cohort rejection), `crates/effect-runtime/src/bank.rs:272` `identity_coef`, `crates/compressor/src/lib.rs:751` `bind_homogeneous_bank`.
- The compressor's per-lane state (detector, gain smoother) must remain at its exact rest value on an identity lane so the lane's output is its input; the silent-block admission path (`block_is_positive_zero`, `crates/effect-runtime/src/bank.rs:130`, used near `crates/compressor/src/lib.rs:513-527`) shows the rest-state reasoning to reuse.

## Smallest closable slice

Authorized paths: `crates/graph-compiler/src/banks.rs` (only if the EQ issue left compressor-specific gating), `crates/compressor/src/lib.rs` (bank binding only), `crates/compressor/src/kernel.rs` (only if an identity lane needs a guard that costs nothing on active lanes), their tests, and this spec.

## Non-goals

No kernel arithmetic change on active lanes. No ramp-path change.

## Objective gates

1. New test: sessions with 1..W-1 compressor tracks bind one bank with identity lanes.
2. New end-to-end test: rendered PCM bit-identical to scalar per-node rendering on random input and the compressor fixtures, including blocks where active lanes ramp.
3. `scripts/check-graph-determinism.sh`, `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` pass; `cargo test -p compressor -p graph-compiler -p graph` green.

## Dependencies

"Bind partial parametric EQ cohorts as banks with exact identity lanes" (reuse its planner change and test shape).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
