# Publish spectrum records by reserve and commit

## Product outcome

Each spectrum publication reconstructs a 2 048-frame window from the ring with a scalar per-frame wrap branch into a staging buffer, copies it into a by-value `SpectrumWindow` on the stack, then moves the ~16 KB record into the queue slot: about 48 KB of memmove and a 16 KB stack frame inside the audio callback per hop. Write once, directly into the queue slot. Class A.

## Root evidence

- `crates/host-core/src/spectrum.rs:2581` `continuous_publish`; `SpectrumWindow` (`:438`, two `[f32; 2048]`); the captured record type near `:465-471`; `try_push` on the bounded SPSC (`crates/engine/src/realtime/spsc.rs`).
- A reserve/commit producer pattern already exists for the retirement queue in `crates/engine/src/realtime/plan_exchange.rs`.

## Smallest closable slice

Authorized paths: `crates/engine/src/realtime/spsc.rs` (a `try_reserve`/`commit` or `try_push_with(|slot| ...)` API, documented safety unchanged), `crates/host-core/src/spectrum.rs`, their tests, and this spec.

Reconstruct the window with two `copy_from_slice` per plane straight into the reserved slot; commit; no stack temporary.

## Non-goals

No change to record contents, queue depth, drop accounting or the consumer side.

## Objective gates

1. New test: published records bit-identical to the current implementation for random ring states including every wrap offset.
2. New SPSC test: reserve/commit preserves the existing ordering guarantees and drop counting; a reserve without commit leaves the queue unchanged.
3. `scripts/check-host-core-policy.sh`, `scripts/check-realtime-policy.sh`, `scripts/check-realtime-audit-leak.sh`; `cargo test -p engine -p host-core` green.

## Dependencies

None; may follow "Capture continuous spectrum history with block copies".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
