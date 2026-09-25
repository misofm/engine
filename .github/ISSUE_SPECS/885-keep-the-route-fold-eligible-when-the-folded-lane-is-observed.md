# Keep the route fold eligible when the folded lane is observed

## Product outcome

The fused route fold (delivered by #218/#419) removes a whole `mix2x2` pass and a whole master-reduction pass per lane, but `foldable_lane` declines whenever the chain's last slot or the route is observed. With a meter on every track the fold never fires. Let a post-matrix observer read the lane's resident output and keep the fold armed. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:4826` `foldable_lane` returns `None` when `observed(...)` (`:4764`) is true for the chain's last slot or the route; the resident-observe branch in `observe_unit` (`:1915`, conditions near `:1930-1943`) and `observe_active_entry` (near `:2117-2133`) require `fold.is_empty() && chain.fold_lanes().is_empty()`.
- The fold consumes the scatter staging tile in `fold_plane`/`fold_cohort` (`:1218-1305`); the resident AoSoA scratch is untouched by it, and `BankChain::final_output_lane` (`crates/rack/src/lib.rs:2061`) already exposes the lane's final words.
- Bit-identity gate already exists: `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` (`runtime.rs:7040`).

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/observation_activation.rs` (only if the activation entry needs a resident-lane variant), `crates/rack/src/lib.rs` (only `final_output_lane` or a sibling accessor), their tests, and this spec.

Allow `foldable_lane` to return a fold when the only observers on the last slot / route read the post-matrix boundary, and make those observers read `final_output_lane` (the words the fold will mix) instead of the member buffer. Observers at other taps keep the current behaviour. Do not change the fold kernel.

## Non-goals

No change to scatter redirect eligibility (separate issue), to meter arithmetic, or to observation cadence.

## Objective gates

1. New test: a plan with a `PostMatrix` meter on every track of a full bank binds with the fold armed (assert on the prepared plan's fold lanes).
2. New test: for random input, the master output and every published meter snapshot of that plan are bit-identical to the same plan rendered with meters bound and the fold forcibly disabled (the current path).
3. `a_folded_epilogue_is_the_route_and_the_reduction_bit_for_bit` and the rest of `cargo test -p graph` pass; `scripts/check-graph-determinism.sh`, `scripts/check-graph-policy.sh`, `scripts/check-realtime-policy.sh` pass.
4. Descriptive metered console row before/after if the row exists.

## Dependencies

Measurement only: "Add a metered live-console row to the console benchmark". Independent of the web binding issue.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
