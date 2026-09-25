# Keep the direct scatter eligible when a later tap is observed

## Product outcome

`scatter_target` lets a full bank scatter straight into its consumers' buffers (delivered by #202/#399), but declines when the producer or any later tap has an observer, so metered sessions fall back to scatter-then-copy. Let observers that only read the final lane words keep the direct scatter. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:4494` `scatter_target` declines when the producer or any later tap is observed (`observed`, `:4764`); the fallback path scatters into `staging_*` and copies per lane (`crates/rack/src/lib.rs` `scatter_tiled` near `:2666-2791`).
- An observer of the post-matrix boundary can read `BankChain::final_output_lane` (`crates/rack/src/lib.rs:2061`) before or after the direct scatter with identical words.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/rack/src/lib.rs` (accessors only), their tests, and this spec.

Treat an observer whose tap is the chain's final output as not blocking `scatter_target`, and serve it from the resident lane. Observers at intermediate taps keep the current behaviour. No kernel change.

## Non-goals

No change to the fold (separate issue), to partial-bank scatter, or to observation cadence.

## Objective gates

1. New test: a metered full bank binds with direct-scatter targets (assert on the prepared plan).
2. New test: master output and meter snapshots bit-identical to the same plan with direct scatter forcibly disabled.
3. `cargo test -p graph -p rack` green; `scripts/check-graph-determinism.sh`, `scripts/check-graph-policy.sh`, `scripts/check-rack-policy.sh`, `scripts/check-realtime-policy.sh` pass.

## Dependencies

Do after "Keep the route fold eligible when the folded lane is observed"; both touch `observed` call sites and the second should reuse the first's resident-lane observer path.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
