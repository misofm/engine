# Remove or use the public functions that only tests call

Found by the TA4 survivor audit: mutants in these functions survive because no product path calls them.

## Problem

These public functions have no caller outside tests (checked at `b9135008d` with a workspace grep):

- `conformance::PlanarBlock` and `conformance::compare_f32_to_f64` (`crates/conformance/src/block.rs`, `compare.rs`): only `crates/conformance`'s own tests use them.
- `source::PcmSourceConsumer::read_block` and its `validate_output_shape` (`crates/source/src/lib.rs`): only `crates/source`'s unit tests and `tests/` call them; the render path reads a source through `begin_block_at` and its played planes (`crates/graph/src/runtime.rs`, `crates/host-core/src/source.rs`).
- `effect_contract::map_stepped_normalized` (`crates/effect-contract/src/lib.rs`): only `crates/conformance/tests/effect_contract.rs` calls it.

Dead public code carries tests and review cost and hides real gaps from mutation testing.

## Smallest closable slice

For each item, either delete it with the tests that only test it, or name the product caller that should use it and open an issue for that. Delete nothing a fuzz target, an ABI pin or a wire format uses.

## Gates

1. A workspace grep shows no remaining test-only public function from the list above, or the issue names the product caller for each that is kept.
2. `cargo test --workspace --all-features` and `scripts/check-workspace-policy.sh` are green.
