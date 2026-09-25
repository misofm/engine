# Move PDC-staged inputs without the intermediate copy

## Product outcome

An op with plugin-delay compensation copies its producer into a staging buffer and then swaps the staging buffer through the delay ring, one extra full-block pass per delayed edge. Read the producer directly with a three-way move. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:2183` `execute_op`, PDC staging near `:2207-2218`; `pdc_delay_block` (`crates/lane/src/kernels.rs:697`) swaps `io` with the ring.

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels.rs` (a `pdc_delay_move_block(ring, cursor, source, out)`), `crates/lane/tests/`, `crates/graph/src/runtime.rs`, their tests, and this spec.

## Non-goals

No change to PDC sample counts or ring sizing.

## Objective gates

1. New lane test: for random rings, cursors and inputs the new kernel's output and ring state equal copy-then-`pdc_delay_block`.
2. Existing PDC sample-count fixtures and `scripts/check-graph-determinism.sh` pass; `scripts/check-lane-policy.sh`, `scripts/check-realtime-policy.sh`.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
