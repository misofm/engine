# Bind partial parametric EQ cohorts as banks with exact identity lanes

## Product outcome

The graph compiler banks only cohorts that are full and whose every member carries every slot; anything else renders scalar per node, so a five-track session on an eight-lane host banks no EQ at all. Let the EQ accept identity lanes so a partial cohort binds as one bank. Banking must not move a bit.

## Root evidence

- `crates/graph-compiler/src/banks.rs:214-222`: `if !group.is_full() { continue; }` and the `active_slots` all-lanes check; the comment cites the closed #96 F7 "no per-lane bypass mask".
- `crates/effect-runtime/src/bank.rs:261` `BankKernel` already declares `identity_coef()` (`:272`); `HomogeneousBank` (`:245-371`) has no production users.
- `crates/parametric-eq/src/lib.rs:2390` `bind_homogeneous_bank` is the EQ's bank entry; the rack already carries an `active_lanes` mask and `PreparedSlot::lane_active`.

## Smallest closable slice

Authorized paths: `crates/graph-compiler/src/banks.rs`, `crates/parametric-eq/src/lib.rs` (bank binding only), `crates/effect-runtime/src/bank.rs` (only if a shared identity-lane helper is needed), their tests, and this spec.

Allow a cohort with fewer than W members, or with members lacking the EQ slot, to bind as a bank whose missing lanes carry the EQ's exact identity coefficients and whose scatter skips them (relying on "Tiled gather and scatter for partial banks"). Scalar per-node rendering remains for cohorts the planner still rejects.

## Non-goals

No other effect (compressor is a separate issue). No kernel change. No change to the identity coefficient definition.

## Objective gates

1. New graph-compiler test: sessions with 1..W-1 EQ tracks bind one bank with identity lanes (assert on the plan).
2. New end-to-end test: for those sessions, rendered PCM is bit-identical to the same session rendered with banking disabled (scalar per node), on random input and on the EQ fixtures.
3. `scripts/check-graph-determinism.sh`, `scripts/check-parametric-eq-render-contract.sh`, `scripts/check-effect-runtime-policy.sh`, `scripts/check-realtime-policy.sh` pass; `cargo test -p graph-compiler -p parametric-eq -p graph` green.

## Dependencies

"Tiled gather and scatter for partial banks".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
