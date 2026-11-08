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

### Attempt 1 (Terra), branch `codex/issue-2410-countdown-residue`, base `231494259`

**Claims re-verified at `231494259`.** All hold.
- Fader: `FaderRampStage::process_plane` wrote `self.ramp[channel].remaining` from the `u32` at the top of every ramping block (`crates/builtins/src/lib.rs:3770-3774`), and `gain_mute_ramp_block` read it and wrote it back (`crates/lane/src/kernels/builtins.rs:309`, `:320`). It had no other reader.
- Matrix: `MatrixStage::import_lane` wrote the residue (`lib.rs:4307-4311`), and `process` rebuilt it before every ramp kernel (`lib.rs:4241-4244`).
- Input: `export_lane` read `ramp.remaining` into `InputChannelState::ramp[3]` (`lib.rs:2946`), and `import_lane` wrote it back (`lib.rs:3021`). `load_countdown` rebuilt it before every ramp kernel (`lib.rs:1867-1876`).
- The four `1 << 24` constants were at `lib.rs:1771`, `:3420` and `:3942`, and `crates/effect-runtime/src/ramp.rs:337`.

**What changed.**
- **One constant: `lane::kernels::EXACT_COUNTDOWN`.** `lane` is the one crate that every user depends on: `builtins` and `effect-runtime` both depend on `lane`, and the kernels that read the word live there. `lane::kernels` is chosen over `lane::kernels::builtins` because `effect_runtime::ramp::RampLanes` is not a builtins kernel. The constant's doc gives the one reason the bound is exact and invisible. The three builtins constants and effect-runtime's private copy are gone. The builtins stages clamp to the constant, and `RampLanes::gather` asserts it. `lane::kernels::trailing_count`'s `1 << 24` was left alone. It is the silence counter's saturation, a different claim.
- **Fader (#2378 C15): the kernel takes the countdown as an argument.** `GainMuteRamp` loses its `remaining` field. `gain_mute_ramp_block(io, frames, remaining: L, r)` counts the word down in a register and writes back only `current`. `process_plane` builds the word as a local from the `u32` countdown, clamped, before each ramp kernel. Of the spec's two ways, this one was picked because a scoped copy would keep a field that every caller has to fill and that the stage would then throw away. With the signature change the type cannot hold a stale countdown. `process_plane` is the kernel's only caller (`grep` over the workspace). The `lane` docs that cite it (`kernels.rs`'s D11 note, `InputTrimRamp`'s shape note) were checked and updated. `fader_bank_lane_words` is unchanged.
- **Matrix:** the import's residue write is deleted, and the import doc says why.
- **Input:** `InputChannelState::ramp` is `[f32; 3]`. The export no longer reads the residue, and the import no longer writes it.
- **Docs updated:** `InputLaneState`, `InputChannelState::ramp`, `InputStage::remaining` (it cited `GainMuteRamp::remaining`), the input's `lanes_at_rest` terms, `InputStage::trim_ramp_words`, the `settled_on_target` notes, `FaderRampStage`'s ramp-independence section, its `remaining`, `lanes_at_rest`, `import_lane` and `reset` docs, the matrix import doc, `fader_bank_lane_words`, and `rest_tests.rs`'s fader probe.
- **Trim evidence helpers: kept, as the spec allows.** `trim_ramp_words` still reads the stage's own residue, because the rule changes what crosses a lane carry and not what the stage holds. Its doc now says that a lane swap or a carry leaves each lane position's residue in place. P3-M2 therefore keeps its observable.

**No bit moved** (one-time PR evidence, not committed). The scratch harness `builtins/tests/zz_digest_2410.rs` is outside the tree. It is #1051's randomized differential at 96 seeds, with W4 and W8 banks checked against scalar strips; both widths run on this AVX2 host. On a third of the blocks it swaps two lanes in all four banks (input, twin, fader, matrix). Half of those swaps are single, with the scalar strips swapped too so the bank-against-scalar oracle still holds. The other half are double. Each swap drives `export_lane` and `import_lane`. Two digests are taken:
- `gate`: every stage's bank and scalar output and the bank report after every block. It also covers, for every lane before every block, the `export_lane` record of all four banks, with the input's `ramp` list cut to three words so base and fix are comparable. It also covers `bank_trim_ramp_words[0..6]`, `bank_section_words`, `bank_lane_state_words`, `bank_lane_silence_words`, `fader_bank_lane_words`, `matrix_bank_lane_words` without word 12, the scalar matrix words and the scalar trim ramp words.
- `residue`: the bank residue words, which are `bank_trim_ramp_words[6..8]` and `matrix_bank_lane_words[12]`.

| | base `231494259` | fix |
|---|---|---|
| gate (release and debug) | `b0288cae591c18d4` | `b0288cae591c18d4` |
| residue | `658e956a3c2ff0f4` | `6a1bffaff17fc172` |

The residue digest is expected to move. A swap no longer carries a lane's residue, so each lane position keeps its own. Every rendered word and every exported word except the removed one is identical to base.

**Mutation evidence** (each applied to the fix alone, then reverted):
| | mutation | result |
|---|---|---|
| F1 | the fader's local countdown is `L::zero()` | RED: `fader_ramp::a_windowed_move_is_monotone_and_lands_exactly_on_its_target` and `mute_is_a_fader_endpoint_and_settles_to_exact_positive_zero` |
| F2 | the local is built one frame stale (`remaining.saturating_sub(1)`) | RED: `fader_ramp::a_windowed_move_is_monotone_and_lands_exactly_on_its_target` |
| M1 | the matrix import's residue write is restored | equivalent, as expected: `builtins` (all suites) and `builtins-compiler` (with test-support) green. Harness gate `b0288cae591c18d4` is unchanged, and only the residue digest moves |
| I1 | the input record's `ramp[3]` is restored (export read, import write) | equivalent, as expected: `builtins` and `builtins-compiler` green. Gate `b0288cae591c18d4` is unchanged |
| P3-M2 | `MUTATIONS.md`'s row: the dual dispatch is always-ramping | RED, as before: `input_liveness::the_settled_arm_leaves_the_ramp_words_untouched`. Words 6..8 come back as `0xc2000000` (`-32.0`, `-frames`) against `0`. `an_uncommanded_bank_keeps_its_elision_plan` also turns red |

No new test was written: this is a refactor, and the mutation rows above are its evidence.

**Intended size changes.**
- `FaderRampStage<L>` holds one `L` lane word less per channel: 64 bytes at W8, 32 at W4 and 8 at scalar.
- `InputLaneState` is 8 bytes smaller. It is a transient carry record and is not stored in any plan.
- `graph_fixture --check` failed on `v1/direct-route.resources.json` and was regenerated with `--write` (commit `a1e76056e`). `builtin_bank_bytes` went from `7223` to `7159`, and `incremental_plan_bytes` and `session_plus_plan_bytes` from `75191` to `75127`. That is the W8 fader stage's 64 bytes. No other graph fixture moved.
- The builtins fixture corpus (`fixtures/builtins/v1`, with `resources.jsonl`'s `engine_owned_processor_payload_bytes`) passes `scripts/check-builtins-fixtures.sh` with the release `audit` binary unchanged (50 files). `builtins-compiler`'s `allocation_tracker` suite, which checks the report's layouts against the allocations it tracks and takes `engine_owned_processor_payload_bytes` as an exact cap, passes. Its `scale` test is `#[ignore]`d and was not run.

**Gates** (fix head, one crate at a time, `-j 4`, private target dir):
- `cargo test --locked`:
  - `builtins --features builtins/test-support`: lib 27, and all 16 integration suites;
  - `builtins-compiler`, both with default features and with `builtins-compiler/test-support,graph/test-support`;
  - `graph-compiler`;
  - `host-core --features host-core/test-support,graph/test-support,builtins-compiler/test-support`;
  - `lane` and `effect-runtime` (171 tests).
- `cargo clippy --locked -p builtins -p lane -p effect-runtime --all-targets -- -D warnings`, with and without `builtins/test-support`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins -p lane -p effect-runtime`, and for `builtins` with `--document-private-items`.
- `cargo fmt --check`.
- `cargo check --locked --workspace`, with no test-support features.
- `cargo check --target wasm32-unknown-unknown` with `+simd128` for builtins, lane, effect-runtime, builtins-compiler, de-esser and dynamic-eq.
- `scripts/check-workspace-policy.sh` and `scripts/check-lane-policy.sh`.
- `graph_fixture --check` after the regeneration.
- `scripts/check-builtins-fixtures.sh` with the release `audit` binary.
- `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (129 tests, including G5 `g5_native_digests_match_pins`).
- `cargo build --locked --release -p realtime-policy -p bench`. `realtime_policy` reports ok (299 marked regions in 48 files).

**Follow-ups (not filed).**
1. The matrix and the input trim still *store* their residue word: `Matrix2x2Ramp::remaining` and `InputTrimRamp::remaining`. The fader no longer does. The matrix's word could become a local the way the fader's did. It is reported only as `matrix_bank_lane_words` and `scalar_matrix_words` word 12. The input's word cannot become a local without first giving P3-M2 another observable, such as a ramp-dispatch counter. Neither is a defect.
2. `lane::kernels::trailing_count` spells `1 << 24` for the silence counter's saturation. That is a separate claim from the countdown bound, so it was left alone.
