# Remove graph's and builtins-compiler's orphaned controlled observation activation

Successor of #1076 (Sol verdict attempt 1, finding 3). Owner decision 9 (`docs/rulings/engine-footprint-2026-09-29.md`) keeps the ordinary observation path and removes the unadopted protected path. After #1064 and #1076, the controlled observation-activation machinery below them has no production caller (proved by making each entry crate-private and compiling every target: the workspace, host-web on wasm32 `simd128`, and the 25 product crates on `aarch64-apple-ios` and `aarch64-linux-android`).

## Smallest closable slice

Remove: the bind/into-bound activation entries (`bind_with_observation_activation`, `bind_with_source_set_and_observation_activation`, `into_bound_with_observation_activation`, `into_bound_with_source_set_and_observation_activation`), `GraphNodeObserverBinding::controlled`, `prepare_controlled_session_builtins_*`, `MeterBindingPolicy::Controlled`, graph's `observation_activation` module with the executor's per-block activation hook (`Runtime::begin_observation_block`'s `apply_candidate`), and host-core's observer half (`activation_changed`, `observation_generation`, the `maximum_pops` bound), plus builtins-compiler test-support helpers that exist only for them.

## Gates

1. Crate-private-then-compile on every target proves non-use before deletion.
2. The ordinary-path differential (all shipped exports, seeded boots; #1076's method) shows zero mismatches; console digests unchanged; the three browsers pass in CI mode.
3. The shipped module changes only by removed code (about 34 activation functions); the render closure loses only the removed hook; render stays allocation-, lock- and syscall-free.
4. clippy, fmt, the affected tests in dev and release, every policy script.
