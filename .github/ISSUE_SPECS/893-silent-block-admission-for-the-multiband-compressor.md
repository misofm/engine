# Silent block admission for the multiband compressor

## Product outcome

The compressor, EQ and true-peak limiter skip a settled all-`+0.0` block; the multiband compressor has no such path and runs its crossovers and two band detectors on silence. Add the same admission. Class A once the rest state is proven.

## Root evidence

- Template: `crates/true-peak-limiter/src/lib.rs:643` `is_at_silent_rest` and its use near `:2204-2230`; compressor admission near `crates/compressor/src/lib.rs:513-527`; `block_is_positive_zero` (`crates/effect-runtime/src/bank.rs:130`) and the `-0.0` argument at `bank.rs:96-128` explain why only `+0.0` qualifies.
- `crates/multiband-compressor/src/lib.rs:1670` `process_bank` has no silent check.

## Smallest closable slice

Authorized paths: `crates/multiband-compressor/src/lib.rs`, `crates/multiband-compressor/src/kernel.rs` (or wherever the frame loop lives), its tests, and this spec.

Define the multiband's rest state (every crossover integrator, detector envelope and gain smoother at the exact value it reaches on infinite `+0.0` input; ramps settled). When the input block is all `+0.0`, no ramp is in flight, and the state is at rest, return without touching the planes and leave the state unchanged. Prove the rest state is a fixed point of the kernel on `+0.0` input.

## Non-goals

No change to active-block arithmetic; no `-0.0` admission; no change to bypass.

## Objective gates

1. New test: feed 64 blocks of `+0.0` after a loud passage; the outputs and the final state are bit-identical with the admission on and off (this proves the fixed point).
2. New test: a `-0.0` block and a block with one subnormal are not admitted.
3. Existing multiband fixtures pass; `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh`; `cargo test -p multiband-compressor` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
