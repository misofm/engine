# Feed the bypass dry line without a copy and swap when nothing is bypassed

## Product outcome

A console-controlled effect slot with nonzero latency copies both planes into a dry buffer and swaps them through its delay line every block, bypassed or not, so that a future bypass can be latency-preserving. When nothing is bypassed the swapped-out samples are never read. Feed the line with one write per plane and materialise the dry copy only on bypassed blocks. Class A.

## Root evidence

- `crates/rack/src/lib.rs:1141-1143` call `shunt.capture` when `any_bypassed || shunt.feeds_line()`; `BypassShunt::feeds_line` (`crates/effect-contract/src/live.rs:739`) is true for any latency > 0; `capture` (`:750`) copies into `dry_*` then exchanges through the ring with `pdc_delay_block`. The restore loop near `crates/rack/src/lib.rs:1232-1250` is a strided scalar loop per bypassed lane.
- The scalar console path does the same in `crates/graph/src/runtime.rs` near `:2296-2340`.

## Smallest closable slice

Authorized paths: `crates/effect-contract/src/live.rs` (`BypassShunt`), `crates/rack/src/lib.rs` (call sites and restore), `crates/graph/src/runtime.rs` (scalar call site), their tests, and this spec.

Add a `feed` operation that writes the input frames into the ring and advances the cursor without producing the delayed output (one write pass per plane); use it when `feeds_line() && !any_bypassed`. Keep `capture` for bypassed blocks. Replace the strided restore with a per-frame lane select where a lane mask is available.

## Non-goals

No change to bypass semantics, latency, or the effect kernels.

## Objective gates

1. New test: for random input and a random bypass schedule that toggles at every block offset, the output of a latent slot is bit-identical before and after, including the first bypassed block after a long un-bypassed run and the first un-bypassed block after a bypass.
2. Existing bypass/latency fixtures pass (`scripts/check-effect-contract.sh`, `scripts/check-rack-policy.sh`, `scripts/check-realtime-policy.sh`); `cargo test -p effect-contract -p rack -p graph` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
