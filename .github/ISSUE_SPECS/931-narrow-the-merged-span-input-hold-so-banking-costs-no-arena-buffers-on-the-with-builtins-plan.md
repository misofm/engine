## Finding

Issue #169 ruled that banking costs no arena buffers, and its evidence was the builtins-less compile of `console-sixty-four-track.json`, where the banked and per-node arenas were both 193 buffers. While implementing #925 (identity-bound builtin stages lowered as aliases), the same fixture compiled the way every host compiles it (`compile_with_builtins`) measures **banked 256 against per-node 193**, identical at base `3c93469d`, so the claim never held on the host path. Derivation: with builtins, `PostInputBuiltins` is itself a builtin bank member; the chain `builtins -> EQ -> compressor -> fader -> matrix` merges per cohort and the eight spans overlap into one, so the `Input` slots the post-input ops free are held for the whole span (#169's hold rule, required because a merged chain runs a later cohort's compressor before later banks read their inputs): 64 inputs + 64 dedicated post-input + 64 dedicated EQ + 64 compressor outputs = 256. Per node, the effect ops break the chain and each post-input bank's window releases its eight input slots when it closes: 64 + 8 + 56 + 64 + 1 = 193. The difference is 63 stereo buffers, about 63 KiB at a 128-frame quantum on that fixture; memory, not copies, and independent of stem length.

On the builtins-less plan the identity post-input copy level absorbed the input retirements, which is why the two arms were equal there; #925 removes that level and pins the builtins-less arms at 192 banked and 129 per node (both below the old 193, so no plan grew).

## Smallest closable slice

Narrow the hold in `program::lower`'s colouring so a slot freed inside a merged span is reused once no later cohort of the span can still read it, without changing which chains merge or any rendered bit. Authorized paths: `crates/graph/src/program.rs` (colouring passes only), `crates/graph/src/program/tests.rs`, the pinned test in `crates/graph-compiler/src/lib.rs` (`banking_costs_no_arena_buffers_with_builtins_and_holds_inputs_without` or its successor name), and this spec. The random-graph corpus's dataflow proof must stay green with zero divergence.

## Non-goals

No change to cohort merging (#202), to bank eligibility, or to any kernel. No benchmark row is expected to move; the outcome is memory.

## Objective gates

1. `compile_with_builtins` on the console fixture: banked arena equals per-node arena, pinned with the corrected rationale.
2. The random-graph lowering corpus (`lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs`) green with zero dataflow divergence; `scripts/check-graph-determinism.sh` bit-identical.
3. `cargo test -p graph -p graph-compiler -p console-workload` green; `check-graph-policy.sh`, `check-realtime-policy.sh`.

## Dependencies

After "Lower identity-bound track stages as aliases" (#925), which records the measurement.
