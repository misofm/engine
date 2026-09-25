# Tiled gather and scatter for partial banks

## Product outcome

Whenever a bank's active lane count is not the full width (a single-track session, or any cohort whose size is not a multiple of W), `BankChain` gathers and scatters with a scalar strided loop, one 4-byte move per lane-sample per plane, while the kernel still processes all W lanes. Run the tiled register transpose for partial banks too. Class A: transposes are exact permutations.

## Root evidence

- `crates/rack/src/lib.rs`: `full_bank` computed at `:1749`; `gather` branches on it at `:2535` and `scatter` at `:2600`; the scalar fallbacks are `gather_lane` (`:180`) and `scatter_lane` (`:222`); the tiled path is `gather_tiled` (`:2553`) and `scatter_tiled` (near `:2666`), built on `tile_gather`/`tile_scatter` (`:252-337`) and `transpose_tile_4/8` in `crates/effect-contract/src/lib.rs` near line 289.
- Inactive lanes can be fed from `ARENA_SILENCE_BUFFER` (`crates/engine/src/realtime/disjoint.rs:33`); `write_stereo_many` (`disjoint.rs:262`) requires distinct writable buffers, so an inactive lane needs a chain-owned dump buffer reserved at bind, never a render-time allocation.
- Existing gate: `full_bank_gather_scatter_round_trip_is_bit_exact` (`crates/rack/src/lib.rs:4408`).

## Smallest closable slice

Authorized paths: `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (only to reserve one dump buffer per chain at bind if the arena must own it), their tests, and this spec.

When `full_bank` is false: gather with the tiled path using the silence buffer for inactive lanes; scatter with the tiled path into the member buffers for active lanes and into the dump buffer for inactive lanes (or into staging and copy only active lanes out, whichever keeps `write_stereo_many` satisfied). Keep the ragged frame tail on the existing per-lane code. Do not change kernels.

## Non-goals

No change to which cohorts get banked (that is the identity-lane issue), no kernel change, no fold change.

## Objective gates

1. New test: for every active count 1..W-1 and random active-lane masks, the gathered scratch and the scattered outputs are bit-identical to the current per-lane path on random input (extend `full_bank_gather_scatter_round_trip_is_bit_exact`).
2. Allocation test: preparing and rendering a partial bank allocates nothing on render (use the crate's existing allocation-counter test pattern).
3. `cargo test -p rack` green; `scripts/check-rack-policy.sh`, `scripts/check-realtime-policy.sh` pass.
4. One descriptive `scripts/run-rack-benchmark.sh` run before/after, attached.

## Dependencies

None. Unblocks the identity-lane issues.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
