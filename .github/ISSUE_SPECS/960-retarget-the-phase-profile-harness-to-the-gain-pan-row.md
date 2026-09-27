# Retarget the phase-profile harness to the gain/pan row

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (option (a), owner-directed removal, 2026-09-27).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

1. **Authorized paths:** `tools/console-workload/tests/` (the new `gain_pan_profile.rs`),
   `crates/graph/src/lib.rs` and `crates/rack/src/lib.rs` (test-support probes only), and this spec.
2. Add sub-phase probes inside the bank chains (gather, each slot, fold, scatter), because the
   single `BANK` phase hides everything that matters on the gain/pan row.
3. Target the production-feed row `sixty_four_track_gain_pan_ring`. Land after #957.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Smallest closable slice

- **S5. Retarget or delete the phase-profile harness.** It covers `graph::test_only_phase_profile`
  and the deleted `plumbing_profile.rs`.

Ruling: retarget, do not delete. `graph::test_only_phase_profile` (test-support only) stays, and a `gain_pan_profile` harness replaces the deleted `plumbing_profile.rs`, so the pure-path target can be diagnosed per phase.

## Dependencies

#956.

