# Run the 4-wide resident fold in the wasm gates

## Product outcome

The browser ships 4-lane banks, so `ArenaMembers::fold_resident_tiles::<4, Simd4>` (`crates/graph/src/runtime.rs`) is the fold every browser session runs. Only native graph unit tests exercise that instantiation; no wasm gate executes it, so a wasm-only miscompile or lowering difference in the fold would reach users unseen. Found by the #945 verification.

## Smallest closable slice

Add a `tools/wasm-gate-corpus` case (the existing wasm-versus-native differential mechanism) that runs the 4-wide resident fold on a fixed hostile corpus (signed zeros, subnormals, magnitudes 2^-24..2^25, a ragged frame count) and compares the master words bit for bit with the native result.

## Objective gates

- The new case passes in `scripts/run-wasm-gates.sh`.
- A red mutation in the fold (swap two rows of a tile at W=4) turns it red.
- fmt, clippy `-D warnings`.

## Decision and evidence record

- Delegated-corpus shape (root, from the scope note): `crates/graph/src/corpus.rs` owns two cases
  (`resident_fold/store`, `resident_fold/accumulate`; 8 lanes, 131 frames, inputs built from
  `xorshift64*` bits) and their pins; it reaches the fold through one `pub(crate)` hook,
  `runtime::run_resident_fold`, and `SourceGather::NONE` lost its `#[cfg(test)]`. `Case::GraphFold`
  is appended last in `wasm-gate-corpus` (indices 153-154), so no existing index moves.
- Width 1 is an independent scalar reference, not the fold's body; W4 runs two cohorts onto one
  master (the browser's shape), W8 one. The sample range is `2^-24 .. 2^25` as the spec says.
- Pins came from the scalar reference; Simd4 and Simd8 agreed with it on the first run.
- The simd128 guest now compiles `graph` and `rack` for wasm32 (accepted, scope R4).
- Red mutation (W4 row swap after the transpose) is red at simd4 only, on both legs:
  `tools/wasm-gates/MUTATIONS.md`, "The 4-wide resident route fold".
