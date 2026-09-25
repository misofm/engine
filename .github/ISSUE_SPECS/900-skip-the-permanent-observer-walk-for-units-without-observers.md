# Skip the permanent observer walk for units without observers

## Product outcome

On the permanent observation path the executor calls `observe_unit` for every unit every block; for a bank it evaluates an O(W) eligibility predicate and iterates every member before discovering that no member has an observer. Record at bind whether a unit has any observer and skip the walk otherwise. Class A.

## Root evidence

- `crates/graph/src/lib.rs:2251` calls `runtime.observe_unit` per unit when no activation is bound; `crates/graph/src/runtime.rs:1915` `observe_unit` walks members and `member.observers`.
- The controlled path (`observe_active_unit`, near `:1848`, delivered by #816) already visits only active entries.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, their tests, and this spec.

## Non-goals

No change to what an observer sees or when.

## Objective gates

1. New test: observation output for a plan with observers on some units is identical before/after; a plan with no observers performs zero `observe` calls (counter).
2. `cargo test -p graph` green; `scripts/check-graph-policy.sh`, `scripts/check-realtime-policy.sh`.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
