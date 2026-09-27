# Pin the input-section symmetry witness in the pool-class grouping

## Product outcome

No compile-level test and no console row has a track whose prepared input section is channel-asymmetric while its source is mono, so the input-builtins symmetry witness (the loop at `crates/graph-compiler/src/compile.rs:442` after #959) is unpinned: deleting it leaves all 411 graph-compiler, console-workload, host-core, capi, bench and audit tests green (mutation 959-4, pre-existing on `792a1e87`). Rendered bits are not at stake, since pool class regroups lanes, but bank grouping moves silently. Found by the #959 implementation and verification.

## Smallest closable slice

Add a second arm to `a_single_odd_track_strands_both_pools_remainders` (`crates/graph-compiler/src/lib.rs:8331`), with the odd track made odd by its input section rather than its source mapping: parse `console-sixty-four-track-mono.json`, set `tracks[7].builtins.left.polarity_invert = true`, compile through `compile_with_builtins` at `Simd8` and `Simd4`, and assert `graph().prepared_bank_count()` is below the unmodified fixture's (21 against 24 at `Simd8`) and that every `prepared_builtin_banks()` entry containing `ch07` has exactly one member.

## Objective gates

- The new arm passes, and mutation 959-4 (drop the witness loop) turns it red; record it in `MUTATIONS.md`.
- fmt, clippy `-D warnings`, `cargo test -p graph-compiler`.
