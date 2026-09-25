# Silent block admission for the gate/expander and transient shaper

## Product outcome

Same as the multiband issue for the two remaining zero-latency dynamics effects without a silent fast path. Soft-clip and delay are excluded here because their oversampling and tap rings need their own fixed-point proofs.

## Root evidence

- `crates/gate-expander/src/lib.rs:920` and `crates/transient-shaper/src/lib.rs:923` `process_bank` have no `block_is_positive_zero` check; the template and `-0.0` argument are in `crates/effect-runtime/src/bank.rs:96-141` and `crates/true-peak-limiter/src/lib.rs:643`.
- The gate has hysteresis and hold state; the transient shaper has fast/slow envelope followers. Each must be shown to sit at an exact fixed point on `+0.0` input before admission is allowed.

## Smallest closable slice

Authorized paths: `crates/gate-expander/src/{lib.rs,kernel.rs}`, `crates/transient-shaper/src/lib.rs`, their tests, and this spec. One checkpoint per effect.

## Non-goals

Soft-clip, delay, active-block arithmetic, bypass.

## Objective gates

1. Per effect, the two tests from the multiband issue (fixed-point identity over 64 silent blocks after a loud passage; `-0.0`/subnormal not admitted).
2. Existing fixtures pass; `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh`; `cargo test -p gate-expander -p transient-shaper` green.

## Dependencies

"Silent block admission for the multiband compressor" (reuse its test shape).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
