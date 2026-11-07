# Rack: the bypass_skip mock bank declares lane defaults but states no payload verdict, so main's cross-chain swap test fails

## Mission

Make `main` green again. On `900c3d24c`, `cargo test -p rack --test bypass_skip` panics in `settle_edges_follow_live_switches_and_lane_moves_not_the_sessions_own_bypass` at `crates/rack/src/lib.rs:5243` with "a checked slot swaps its lanes". The qualification run `42374267842` on main fails, and so does every PR based on it, for example #2347.

## Cause

Two merged changes interact; neither branch alone fails.

- **#2326 (PR #2342)** added the cross-chain swap rows to this test. They use the file's mock bank `Recorder`, which declares `lane_default_bytes() == Some(0)`.
- **#2324 (PR #2341)** made `swap_bank_lanes` validate both lanes' payloads before its first write. A bank that keeps the trait default `validate_track_state_payload` (`effect.state.unchecked`) therefore refuses every swap. The trait docs say that a bank declaring its lane defaults must implement the validation. #2324's review flagged that this rule is held only by a comment and recorded a type-level pairing as a follow-up.

`Recorder` is the only bank in the tree that declares defaults without a validation: every launch effect and the `lane_swap` and `lane_rest` mocks have one.

## Fix

`Recorder` states its verdict: its restore takes every payload, so the validation returns `Ok(())`, as the `lane_rest` mock does. The change is test-only, so no product code or rendered bit moves.

## Gates

- `cargo test --locked -p rack --test bypass_skip` is green. On `900c3d24c` it is red at the line above.
- `cargo test --locked -p rack`, clippy `-D warnings` for rack (`--all-targets --all-features`), `cargo fmt --check`, and `scripts/check-workspace-policy.sh`.

## Follow-up

The type-level pairing of `lane_default_bytes` with its validation (#2324's spec, follow-up b) would have made this a compile error. That follow-up is recorded on tracker #2258.
