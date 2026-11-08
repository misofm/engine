# Builtins: one rule for the stages' `f32` countdown residue, and one exact-countdown constant

Successor of #2390, from tracker #2258 GR-7 (lineage #2368, #2376, #2390). #2390 recorded why the builtins stages do not build their lanes through `LinearRamp::from_words`. It also found that the three stages treat the kernel's `f32` countdown word three different ways, and that four constants spell one bound.

## Problem
Each builtins ramp stage keeps two countdowns. The authoritative one is a `u32` per lane. The ramp kernel reads an `f32` lane word, the residue. Before every ramp kernel the stage rebuilds the residue from the `u32`, clamped to `2^24`, and the kernel then runs it below zero (`remaining - 1`, done at `<= 0`). So between blocks the residue is not state. But the stages disagree about it (all in `crates/builtins/src/lib.rs`):

1. **Fader** (`FaderRampStage`). Its export does not carry the residue and its import does not write it (#2377 T2). `fader_bank_lane_words` leaves it out. But the stage still *stores* it: `ramp[c].remaining`, the `GainMuteRamp::remaining` word of each channel's `ramp: [GainMuteRamp<L>; 2]`. `process_plane` rebuilds that word from the `u32` countdown at the top of every ramping block (`crates/builtins/src/lib.rs` about lines 3770-3776 at `231494259`), and `gain_mute_ramp_block` (`crates/lane/src/kernels/builtins.rs` about line 305) reads it and writes it back. Nothing else reads it, so it is per-block scratch held as stage state. This is #2378 row C15, carried here from #2392, whose spec handed it on.
2. **Matrix** (`MatrixStage`).
   - Its export does not carry the residue.
   - Its import writes the residue anyway (`state.remaining.min(MATRIX_RAMP_COUNTDOWN_MAXIMUM) as f32`). That write is dead: `process` rebuilds the word before every ramp kernel. #2390 recorded the mutation that writes `0.0` there (S2) as an equivalent mutant: green in `builtins` and in `builtins-compiler`.
   - `matrix_bank_lane_words` and `scalar_matrix_words` still report the residue (word 12).
3. **Input trim** (`InputStage`).
   - Its export carries the residue: `InputChannelState::ramp[3]`, read from `self.ramp.remaining[channel]` in `export_lane`. Its import writes it back (`put(&mut self.ramp.remaining[channel], lane, carried.ramp[3])`), although `load_countdown` rebuilds it before every ramp kernel.
   - The `exact rest` doc of `InputStage::lanes_at_rest` gives "a word the lane export carries" as one reason a ramping bank declines rest.

Four constants spell one bound, `1 << 24`, the longest countdown an `f32` carries exactly:
- `FADER_RAMP_COUNTDOWN_MAXIMUM`;
- `MATRIX_RAMP_COUNTDOWN_MAXIMUM`;
- `InputStage::RAMP_COUNTDOWN_MAXIMUM`, all in `crates/builtins/src/lib.rs`;
- `EXACT_COUNTDOWN` in `crates/effect-runtime/src/ramp.rs`.

`builtins` does not depend on `effect-runtime`. So a shared home is `lane`, where the kernels that read the word live (`crates/lane/src/kernels/builtins.rs`, whose ramp structs already say "the caller clamps to `2^24`"), or the three builtins constants become one.

## Contract
- **One residue rule for every builtins stage.** A lane export does not carry the kernel's `f32` countdown word. An import does not write it. The stage rebuilds it from the `u32` before every ramp kernel. The fader already follows the export and import half of this rule, and stops storing the word (below). The matrix and the input trim adopt the rule.
  - Fader (#2378 C15): the stage stops storing the residue. `ramp[c].remaining` goes, and `process_plane` builds the countdown word as a local before each ramp kernel. `gain_mute_ramp_block` takes `&mut GainMuteRamp<L>`, of which `remaining` is a field, so there are two ways to do it. The implementer picks one and records the reason:
    - change the kernel's signature in `lane` so that it takes the countdown separately. `remaining` then leaves `GainMuteRamp`, and the stage stores the rest. The `lane` crate is touched, and so is every other caller of the kernel (`crates/lane/src/kernels.rs` docs and the `builtins` tests that cite it);
    - or keep the kernel and build a scoped `GainMuteRamp` copy in `process_plane` from the stage's stored words. The kernel runs on the copy, and only `current` is copied back. That `current` is the one word the stage reads after the block. `ramp` then stores no `remaining` word.
    Either way `fader_bank_lane_words` stays as it is, and no rendered bit moves.
  - Matrix: delete the import's write.
  - Input: drop `ramp[3]` from `InputChannelState`, and the export's read and the import's write of it.
  - Update each doc that cites the old behaviour. These include the input stage's `lanes_at_rest` terms, the `InputLaneState` doc ("the kernel's countdown word"), `InputStage::trim_ramp_words` and the residue notes on the matrix and fader stages.
- **The coupling with the trim evidence helpers is decided, not broken by accident.**
  - `InputStage::trim_ramp_words` (`crates/builtins/src/lib.rs`, about lines 2414-2430 at `3d0e01385`) returns the residue as words 6..8. `test_support::input_trim_ramp_words` and `bank_trim_ramp_words` expose those words to `crates/builtins/tests/input_liveness.rs` and `filter_response.rs`.
  - `input_liveness::the_settled_arm_leaves_the_ramp_words_untouched` asserts the residue is exactly `+0.0`, which proves that no ramping block ran. `crates/builtins/tests/MUTATIONS.md` row P3-M2 records that this residue is the only observable of an always-ramping dispatch. The output planes are bit-identical by the elision proof, so no digest sees it.
  - `trim_ramp_channels_agree` compares `current`, `target`, `step`, `coef.trim` and the `u32` countdowns, never the residue. The residue does not affect its verdict either way.
  - The rule changes what crosses a lane carry, not what a stage holds. So `trim_ramp_words` may keep reading the stage's own residue, and P3-M2 must stay red.
  - If the issue removes the residue from the helpers instead, as #2377 T2 did for the fader, it must give P3-M2 another discriminating observable, such as a ramp-dispatch counter in the test-support walk counters. It must show that row red on the new observable.
- **One exact-countdown constant.** The four `1 << 24` constants are replaced by one, with one doc of why the bound is exact and invisible: a lane reaches zero inside a block only when its countdown is at most the block length. Where it lives (`lane::kernels::builtins`, or one builtins constant plus a cited `effect-runtime` copy) is the implementer's choice, with the reason recorded.
- **No rendered bit moves.**
- **`FaderRampStage` shrinks by one lane word per channel** (the dropped residue). That is an intended size change, like the input's below, and the same fixture rule applies.
- **`InputLaneState` shrinks by one `f32` per channel.** That is an intended size change. If a plan fixture's bytes include it, directly or through a stage or processor size, regenerate `graph_fixture` with `cargo run -p graph-compiler --bin graph_fixture -- --write` intentionally and record which fixture moved and why. Otherwise `graph_fixture --check` must pass. Check the builtins-compiler mutation-matrix transcript's `engine_owned_processor_payload_bytes` too.

## Non-goals
- Changing the countdown authority, the kernels' frame walk, or any ramp law (#2390's record).
- A lane-level helper for the kernels' inline D11 walk. #2390 records it as deliberately not filed: there is no defect, and it would put an abstraction across the SIMD kernels.
- The stale settled step of the matrix, trim and filter. #2390 records it as dead, and clearing it would move exported state words.

## Gates
- **No bit moved.** For the input, fader and matrix sections, scalar and both bank widths, rendered output is identical to base. #2390's builtins harness can be reused: #1051's randomized differential with lane swaps. Every exported lane word except the removed residue is also identical to base. This is PR evidence, not a committed digest.
- **Mutation evidence.**
  - The fader: writing a stale or zero countdown into the local before the kernel turns the fader suites red (`crates/builtins/tests/fader_ramp.rs`). This shows the local is the word the kernel reads.
  - Restoring the matrix import's residue write and the input payload word changes no output. This confirms they were dead.
  - P3-M2 stays red on whatever observable `the_settled_arm_leaves_the_ramp_words_untouched` uses after the change.
- **Realtime.** Render stays allocation-, lock- and syscall-free, and `realtime_policy` passes.
- **Downstream.** builtins, builtins-compiler, graph-compiler and host-core are green. `graph_fixture` is `--check`-clean or regenerated intentionally, as above.

## Evidence / attempt record
(empty)
