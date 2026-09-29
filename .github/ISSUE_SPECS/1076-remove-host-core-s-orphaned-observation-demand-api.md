# Remove host-core's orphaned observation-demand API

Successor of #1064 (Sol verdict attempt 2, finding 2). Owner decision 9 (`docs/rulings/engine-footprint-2026-09-29.md`) keeps the ordinary observation path and removes the unadopted protected path. #1064 removed the browser adapter's side; host-web was the only non-test consumer of host-core's observation-demand API, which is now orphaned in production. The compiler does not flag it only because the items are `pub`.

## Smallest closable slice

Remove, after proving non-use by making them crate-private and compiling every target:
- `crates/host-core/src/observation_demand.rs` (about 2,500 lines);
- the four `prepare_host_runtime_with_observation_demand*` functions;
- the controlled-spectrum machinery in `spectrum.rs` (`ControlledSpectrumCandidate`, `ControlledSpectrumSlot`, `reset_for_controlled_stage`, `retire_controlled_after_receipt`) if nothing on the ordinary path uses it;
- `crates/host-core/tests/observation_demand.rs` (25 tests), which is the only remaining caller.

Keep everything the ordinary observation path, capi or host-web uses.

## Gates

1. `cargo check --workspace --all-targets --all-features`; the wasm `simd128` check; the aarch64 iOS/Android checks; clippy and fmt.
2. host-core, host-web and capi tests; the SDK tests; `check-web-audioworklet.sh`; console digests unchanged; the shipped module changes only by removed code.
3. Every policy script.
