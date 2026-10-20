# Remove or use the public functions that only tests call

Found by the TA4 survivor audit: mutants in these functions survive because no product path calls them.

## Problem

This public function has no caller outside tests (checked at `b9135008d` with a workspace grep):

- `source::PcmSourceConsumer::read_block` and its `validate_output_shape` (`crates/source/src/lib.rs`): only `crates/source`'s unit tests and `tests/` call them; the render path reads a source through `begin_block_at` and its played planes (`crates/graph/src/runtime.rs`, `crates/host-core/src/source.rs`).

TA4 (`TBD-TA4-delete-the-measured-redundant-tests.md`, item 4) already deleted the other items the survivor audit found: `conformance::PlanarBlock` and `compare_f32_to_f64` (survivors m9108, m9109, m9111, m9118) and the `effect_contract` `map_stepped_normalized` family (m16329, m16330). What is left is the source item above (m42034).

Dead public code carries tests and review cost and hides real gaps from mutation testing.

## Smallest closable slice

Either delete it with the tests that only test it, or name the product caller that should use it and open an issue for that. Delete nothing a fuzz target, an ABI pin or a wire format uses.

## Gates

1. A workspace grep shows that `read_block` and `validate_output_shape` are gone, or the issue names the product caller that keeps them.
2. `cargo test --workspace --all-features` and `scripts/check-workspace-policy.sh` are green.
