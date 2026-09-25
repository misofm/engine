# Hoist arena slice derivation out of the master reduction loop

## Product outcome

`reduce_many` re-derives each input's arena slice (bounds check, offset multiply, raw-parts) inside the vector loop because `lease.write` takes `&mut self`, and indexes with `&source[index..]`. Derive every slice once and iterate with `chunks_exact`. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:291` `reduce_many`; `ArenaLease::write_read` (`crates/engine/src/realtime/disjoint.rs:309`) and `read` near `:198-213`; the pattern to copy is `ordered_accumulate_block` (`crates/lane/src/kernels.rs:644`), which takes slices once.

## Smallest closable slice

Authorized paths: `crates/engine/src/realtime/disjoint.rs` (a `write_read_many::<N>` returning one output and up to eight input slices with the same disjointness proof as `write_read`), `crates/graph/src/runtime.rs` (`reduce_many`), their tests, and this spec.

## Non-goals

No change to summation order or to single-input reduction.

## Objective gates

1. Existing reduction bit-identity tests and `scripts/check-graph-determinism.sh` pass; new test: random N-input reductions bit-identical before/after.
2. `scripts/check-realtime-policy.sh`; `cargo test -p engine -p graph` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
