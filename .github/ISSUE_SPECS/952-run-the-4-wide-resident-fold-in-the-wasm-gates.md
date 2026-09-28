# Run the 4-wide resident fold in the wasm gates

## Product outcome

The browser ships 4-lane banks, so `ArenaMembers::fold_resident_tiles::<4, Simd4>` (`crates/graph/src/runtime.rs`) is the fold every browser session runs. Only native graph unit tests exercise that instantiation; no wasm gate executes it, so a wasm-only miscompile or lowering difference in the fold would reach users unseen. Found by the #945 verification.

## Smallest closable slice

Add a `tools/wasm-gate-corpus` case (the existing wasm-versus-native differential mechanism) that runs the 4-wide resident fold on a fixed hostile corpus (signed zeros, subnormals, magnitudes 2^-24..2^25, a ragged frame count) and compares the master words bit for bit with the native result.

## Objective gates

- The new case passes in `scripts/run-wasm-gates.sh`.
- A red mutation in the fold (swap two rows of a tile at W=4) turns it red.
- fmt, clippy `-D warnings`.
