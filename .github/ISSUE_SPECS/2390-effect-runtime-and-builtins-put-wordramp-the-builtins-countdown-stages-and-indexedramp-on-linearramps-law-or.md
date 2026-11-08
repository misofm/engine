# Effect runtime and builtins: put `WordRamp`, the builtins' countdown stages and `IndexedRamp` on `LinearRamp`'s law, or record why they cannot

Successor of #2376, which landed the shared lane-vector walk `effect_runtime::ramp::RampLanes` and moved the compressor, the transient shaper and the true-peak limiter onto it. That issue's spec named three encodings outside its slice, and this issue covers them. The lineage is #2368, then #2376, from tracker #2258 item GR-7 ("one ramp state machine has about six encodings").

## Problem
`LinearRamp` (`crates/effect-runtime/src/ramp.rs`) carries the D11 law and its settled invariant: `remaining == 0` implies `current.to_bits() == target.to_bits()` and a `+0.0` step. That invariant is structural. Words from outside enter only through `LinearRamp::from_words`, and a lane-vector advance goes through `RampLanes`, which writes back through `record_advance`. Three other encodings still keep their own copy of the walk, or their own settle check, or both:

1. **`WordRamp`** (`crates/effect-runtime/src/dynamic_band.rs`, `WordRamp` and `advance_word_ramps`). These are the dynamic band's live words in de-esser and dynamic EQ. They are stored as lane words across blocks, not gathered per block.
   - `advance_word_ramps` is an ungated copy of `RampLanes::advance_where`.
   - The restores already validate through `from_words`: de-esser `parse_lane`, the ramp and link chunks; dynamic EQ `parse_lane`, the same.
   - So the walk is the remaining duplicate.
2. **The builtins' countdown stages** (`crates/builtins/src/lib.rs`: `InputStage`, `FaderRampStage`, `MatrixStage`).
   - The authoritative countdown is a `u32` per lane, beside an `f32` countdown residue that the kernel reads. The stage rebuilds that residue from the `u32` before each ramp kernel runs (issue #2377 T2).
   - Lane state enters through each stage's `import_lane`. Its only callers are `builtins-compiler`'s plan-swap carry, from a predecessor plan's `export_lane`, and the builtins' own lane swaps. So the words are engine-produced, not an external payload.
   - Each stage keeps its own settle rule. Examples: `FaderPhase::at_countdown` (#2280); `MatrixStage::import_lane` clamping the countdown to `MATRIX_RAMP_COUNTDOWN_MAXIMUM`; the input stage's `refresh_ramp_flags`.
3. **`IndexedRamp`** (`crates/lane/src/kernels.rs`).
   - This is the 2x2 route-mix law: `c(k)` is a pure function of the ramp index `k`, and a settled ramp is `length == 0` or `k >= length`.
   - Its doc already explains why it is not D11 ("Why it is not D11").
   - Its users are `crates/graph/src/runtime.rs`, the route mix, and the live crossfade in `crates/effect-contract/src/live.rs`.

## Contract
- **`WordRamp`'s walk is `RampLanes`'s walk.** One option is for `advance_word_ramps` to delegate to `RampLanes::advance_where` with an all-lanes gate. The other is for `WordRamp` to become a `RampLanes`, which needs a crate-visible per-lane accessor for the effects' `link_lane`/`set_link_lane`. Either way, the per-sample advance has one body. The restore's `from_words` validation stays.
- **Each builtins stage either shares `from_words` or records why it cannot.** "Shares" means `import_lane` decides a lane's settled phase through `LinearRamp::from_words`, or through one shared rule beside it. "Records" means the stage's doc states why the `u32` plus residue form is not `LinearRamp`'s. In both cases the doc states that `import_lane` reads only engine-produced words, and which writer upholds the invariant.
- **`IndexedRamp` records, in its own doc, that it cannot share `from_words`.** Its law is not D11, and its settled phase is decided by the index, not by a countdown word. The change is to `crates/lane` alone, unless the record finds a validating restore in `crates/graph`, which would then need its own decision.
- **SIMD lane-word encodings stay branch-free, with no enums in lane words.**
- **No rendered bit, state word or payload word moves.**

## Non-goals
- A stationary hoist for builtin gain retargets. It is cost-only. If it is wanted, it gets its own issue, and it must exclude mute moves, which would move signed-zero bits (#2368 finding 2).
- Changing the `IndexedRamp` law, or the builtins' countdown authority.

## Gates
- **Mutation evidence.** Each removed duplicate's settle and advance check is still caught by one test in the crate that owns the logic: `effect-runtime` for the walk, `builtins` for the stages. Equivalent mutants are named with their reason.
- **No bit moved.** For de-esser, dynamic EQ and the builtins sections, rendered output and snapshot or export words are identical to base, scalar and native bank width (PR evidence, not a committed digest).
- **Realtime.** Render stays allocation-, lock- and syscall-free, and `realtime_policy` passes.
- **Downstream.** The graph-compiler fixtures and host-core are green. If a struct size changes, regenerate `graph_fixture` intentionally, or `graph_fixture --check` must pass.
- **Split by encoding** (`WordRamp`, builtins, `IndexedRamp`) if one brief cannot reach a closable slice in half a day.

## Evidence / attempt record
(empty)
