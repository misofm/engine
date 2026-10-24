# Pin the input-section symmetry witness in the pool-class grouping

## Product outcome

No compile-level test and no console row has a track whose prepared input section is channel-asymmetric while its source is mono, so the input-builtins symmetry witness (the loop at `crates/graph-compiler/src/compile.rs:442` after #959) is unpinned: deleting it leaves all 411 graph-compiler, console-workload, host-core, capi, bench and audit tests green (mutation 959-4, pre-existing on `792a1e87`). Rendered bits are not at stake, since pool class regroups lanes, but bank grouping moves silently. Found by the #959 implementation and verification.

## Smallest closable slice

Add a second arm to `a_single_odd_track_no_longer_strands_a_pool_remainder` (`crates/graph-compiler/src/lib.rs`; renamed by #971), with the odd track made odd by its input section rather than its source mapping: clone the parsed `console-sixty-four-track-mono.json`, keep the mono source, set `tracks[7].builtins.left.polarity_invert = true`, compile through `compile_console_model_with_builtins` at `host_dispatch()`, and assert, through a local helper shared with the first arm, `prepared_bank_count() == 3 * (ceil(63 / W) + 1)` and `pooled_tracks() == [all but ch07, {ch07}]`. No second PCM render.

## Objective gates

- The new arm passes, and mutation 959-4 (drop the witness loop in `crates/graph-compiler/src/compile.rs`) turns it red; recorded below and in the PR body.
- fmt, clippy `-D warnings`, the named test, `scripts/check-workspace-policy.sh`.

## Decision record

- 2026-10-24 (scope note, root): the brief's "21 against 24" is pre-#1098 (every console group now pads), so the count is `3 * (ceil(63 / W) + 1)`; `Simd4` is refused on an AVX2 build for this fixture's EQ (#1694), so the test runs at `host_dispatch()` only; graph-compiler has no `MUTATIONS.md` and ledgers are not grown, so the red-on-revert is PR evidence. Lever: one-side polarity (a sign bit in the trim word), same test as a second arm (one claim).
- 2026-10-24 (attempt 1): green at `host_dispatch()` (W=8). With the `input_channel_symmetry()` conjoin loop commented out, the new arm fails `input section: ...`, left 24, right 27 (ch07 stays in the mono pool); restored.
