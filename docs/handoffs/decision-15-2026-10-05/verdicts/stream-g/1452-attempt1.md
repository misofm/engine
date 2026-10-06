PASS

# #1452 attempt 1 -- adversarial verdict

Verifier: opus-xhigh, 2026-10-06. Reviewed `git diff 6a2c5216a 30cfece25` (commits `045a0dbb3`,
`862afc03a`, `ca57f942c`, `00a0445c5`, `271fdaca8`, `766d6c95e`, `0f7c90ba9`, `45cf6506b`,
`517b339e2`, `30cfece25`) against the spec at `30cfece25` (incl. Amendment 1). Every build and test
ran on `git archive` exports of `30cfece25` (head) and `6a2c5216a` (base) under
`/tmp/claude-1002/v1452/`; the worktree was never edited, built or checked out. Evidence kept in
`/tmp/claude-1002/v1452/evidence/`.

## Summary

- No rendered bit moves. Five independent base-versus-head differentials (own harness, own
  generators) are identical under the class-A rule for all three kept undos. Each differential is
  sensitive to its site.
- The two equivalence arguments hold:
  - Undo 1: `60 <= remaining` selects the same frames as the removed `0 <= min(R, 64) - 60 - k`,
    for every lane, both bodies and every width. This depends on the invariant
    `filter_remaining <= 64`.
  - Undo 5: holding the words equals adding a zero step. This depends on no ramp word ever being
    `-0.0` or non-finite.

  Both preconditions hold on every path the effect can reach.
- The iOS counts did not move. `check-cross-targets.sh` passes with builtins 5, host-core 4,
  soft-clip 1 and true-peak-limiter 6; graph and parametric-eq have 0.
- The V8 spill rows are identical, base to head. Every gate passes except `check-lane-policy.sh`,
  which is red at the base too, on #1329's line.
- The codegen numbers claimed for undos 1, 2, 3, 4, 5 and 6 reproduce exactly, with one omission
  (MINOR 1).
- The reasons to keep the current shape for undos 3, 4 and 6 reproduce.
- Gate 4's premise is false (verified), and root must rule on it before merge (ROOT 1).

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **Undo 5: one loop got longer, and the record does not name it.**
   - The record says "every changed loop shorter on every target". That is false for soft clip's
     scalar (`f32`) instantiation, `Channel<f32>::process`. This is the per-node instance in the
     shipped worklet, and it is reachable when a soft-clip insert does not bank.
   - On `simd128` its ramping frame loop goes 509 -> 512 (+3). The settled copy is 448.
   - On x86-64-v3, one latch path of the ramping loop goes 246 -> 247. The settled copy is
     194/191.
   - The record gives only the `f32x4`/`f32x8` loops and the `f32` function size (1,056 -> 1,667).
     D3 requires each increase to be named. The +3 is 0.6 % of a loop whose settled sibling is
     12 % shorter, so this is not a revert reason. Fix: add the `f32` rows to the undo-5 table
     and correct the sentence.
   - Evidence: `evidence/codegen-simd128.txt` and `evidence/codegen-x86-ios.txt`. The ramping and
     settled loops are identified by opcode mix (only the ramping loop has `pmin`/`pmax`).

## NIT

1. **Stale "21 vector" prose for the route kernel.** The roster now reads `vector=22 scalar=0`
   (verified). It is stale in four places, one more than the implementer listed:
   - `scripts/check-web-audioworklet-callgraph.py:146` ("the route ramp mix, 21 vector
     operations")
   - `scripts/check-web-audioworklet-callgraph.py:242-245`
   - `scripts/check-web-audioworklet.sh:463-465`
   - `crates/lane/src/kernels.rs:1704-1706` (the doc of `route_mix_settled_tail`; its "18 scalar"
     figure was measured beside 21 vector and is not re-measured)

   No budget changes: max(0.10 x 22, 8) = 8. All four are outside the Authorized paths (ROOT 3).
2. **Edits outside the Authorized paths.** Each is a forced consequence and none changes
   behaviour:
   - `crates/lane/tests/input_chain_arming.rs`: compile.
   - The `INPUT_FILTER_LEADING_UPDATES` import in `crates/builtins/src/lib.rs`: the unused-import
     lint.
   - Step 1 of the soft-clip module doc (`crates/soft-clip/src/kernel.rs:21-23`): it described
     the removed branch.
   - The section header at `crates/builtins/tests/filter_liveness.rs:934`. The implementer did
     not list this one; it is beside the authorized gate-8 doc (`:1062-1063`, the spec's `:1088`).

   I accept all four (ROOT 2).
3. **`ca57f942c` (undo 1) alone fails `cargo doc -D warnings`.** `45cf6506b` repairs it, so undo 1
   is two commits. If root reverts undo 1, both must go together. The record says this. A
   one-line note at the undo-1 row would help a bisect.
4. **Undo 1 hides its coupling to the ramp length.** `FILTER_LEADING_FLOOR`
   (`crates/lane/src/kernels/builtins.rs:1221`) spells the ramp length as `64`.
   - The removed builder derived its words from builtins' `INPUT_FILTER_RAMP_SAMPLES` and clamped
     them with `min(64)`. The bodies now read the countdown that `load_filter_countdown`
     (`crates/builtins/src/lib.rs:1333`) clamps only at `2^24`.
   - Equivalence therefore rests on the invariant `filter_remaining <= 64`, which no code
     asserts. It holds today: retarget writes 64 or 0, `settle_filter` saturates down, and
     `import_lane` takes only words `export_lane` produced, because `InputLaneState`'s fields
     are private.
   - Gate 8 would catch a change to the ramp length. A `debug_assert!` or a const tie between
     `INPUT_FILTER_RAMP_SAMPLES` and the lane floor would make the coupling explicit. Optional.

## Equivalence proofs (verified)

- **Undo 2 (route index).**
  - Base: `index_i = splat((position + i*W) as f32) + [1..W]`.
  - Head: `index_0 = splat(position as f32) + [1..W]`, then `index += W`.
  - The loop runs only when `position + 1 < length <= 2^22`. Every value is an integer below
    `2^23`, so every conversion and add is exact. The two index vectors are equal, so the same
    `fma`s follow.
  - The final post-loop add is dead.
- **Undo 1 (leading window).**
  - Base: `lead = (min(R,64) - 60) - k`, with leading `0 <= lead`.
  - Head: `remaining = min(R, 2^24) - k`, with leading `60 <= remaining`.
  - With `R <= 64` (invariant above) both are exact small integers, and
    `0 <= R-60-k <=> 60 <= R-k`.
  - It is the same per lane, in the dual and mono bodies, and at `f32`/`Simd4`/`Simd8`.
- **Undo 5 (holding the words).** `x + (+0.0) == x` for all `x` except `-0.0`, and
  `x + (-0.0) == x` always. An sNaN would be quieted, but no word is NaN. No word is ever `-0.0`
  or non-finite:
  - Preparation: `convert_parameter` normalises zero, and `converted_value_valid` refuses `-0.0`
    and non-finite values.
  - Restore: `decode_lane_words` refuses `-0.0`/non-finite current and target words.
  - Runtime points: `normalize_zero` before conversion.
  - `ramp_toward` (`crates/lane/src/kernels/builtins.rs:171`) returns `current + step` or an
    operand of `min`/`max`. Neither is `-0.0` when the inputs are not, because an exact nonzero
    sum never rounds to zero.
  - The snap assigns the target.
  - Padding lanes rest at validated defaults, and `step_vector` writes `+0.0` for a lane at rest.
  - Subnormal mix words are legal. `d + 0 == d` holds in the canonical FP environment that #146
    installs (the same premise `LinearRamp::stationary_at` documents).
  - Under a browser's forced FTZ/DAZ the old add flushed a subnormal mix word to `+0.0` and the
    hold keeps it. The output bits are the same (every use reads it as zero under DAZ), and the
    held word now matches the native state.
  - Measured check: M5 below (the settled copy with the old zero-step add restored) equals base on
    every owner-level and kernel-level scenario, including the `-0.0` adversarial ones.

## Differentials (independent harness: `evidence/differential-main.rs`)

The harness was built twice, against base crates (`--features base`) and against head crates,
in separate artifact sets. Rows below are scenario lines (one FNV digest each, over every output
word, every mutated state word and every report). NaNs are folded per `dsp_reference::class_a`.

| set | what | rows | base vs head |
|---|---|---|---|
| route | `route_mix_ramp_block` at `f32`/`Simd4`/`Simd8`, 8 calls per row, lengths to `2^22`, positions incl. past `length` and near `u32::MAX`, hostile planes and ramps (NaN, sNaN, inf, `-0.0`, subnormals) | 9,000 | identical |
| fk | both filter-ramp kernels called directly, dual and mono, `f32`/`Simd4`/`Simd8`; per-lane countdowns 0..=64 staggered, 6 blocks with restarts, armable path, trim ramps; base fed `min(R,64)-60` exactly as the owner did | 9,000 | identical |
| fo | the owner: `BuiltinInputBank` Four/Eight × dual / collapsed / collapsed-then-disengaged, plus scalar `InputBuiltins`; 4 launch rates; dense staggered retargets (Left/Right/Both), trims, lane export/import, blocks of 1-128 | 700 | identical |
| sk | `soft_clip_block` with valid words (gains at the limits, mix 0 / 1 / subnormal / min-subnormal), steps of both zero signs, bypass, `f32`/`Simd4`/`Simd8` | 4,500 | identical |
| skadv | as sk with a `-0.0` mix word and `+0.0` steps (outside the reachable domain) | 900 | 552 differ (expected: the harness sees hold-vs-add) |
| so | soft clip through the effect contract: scalar (3 rates × 3 quanta × 20) and Eight-lane banks with padding; automation points incl. `-0.0` and subnormal mix; both resets; snapshot/restore and lane-to-lane restore | 360 live (540 are "declined"/"no-mono": Four banks are not bound on this AVX2 host, and soft clip has no mono collapse) | identical |

- Raw, before the NaN fold, 849 of the 3,000 `f32`-width route rows differ, in NaN payload only
  (a NaN coefficient times a NaN input). `Simd4`/`Simd8` are raw-identical. Class-A excludes
  NaN payloads, and the graph instantiates only the native width (`f32x8` x86, `f32x4`
  wasm/AArch64). Informational.
- Sensitivity (harness against mutated head):

  | mutant | differing rows |
  |---|---|
  | M1 | route 6,423/9,000 |
  | M2 | fk 8,589/9,000; fo 683/700 |
  | M4 | so 360/360 live; sk 5,205/5,400 |
  | M5 | 0 |

## Mutations (each applied to the exported head, run, then restored and re-verified identical)

| id | mutant | result |
|---|---|---|
| M1 | route: `index = index.add(advance)` removed | `lane/tests/route_ramp.rs`: `the_kernel_is_the_indexed_ramp_law_at_every_width` and `a_retarget_starts_from_the_exact_current_coefficients` RED |
| M2 (#1407 LF1) | `FILTER_LEADING_FLOOR` = 61 | `filter_ramp_line::every_filter_ramp_word_lies_on_the_line_to_its_target` RED; gate 8 `every_lane_steps_exactly_four_words_from_its_own_countdown` RED |
| M3 (#1407 LF2) | every lane's leading window from lane 0's countdown, in both bodies | both tests above RED (gate 8: Four `process`, 44.1 kHz, lane 2) |
| M4 | soft clip: ramping blocks dispatched to the settled copy | `ramp_law`: `the_ramp_divides_once_and_snaps_exactly_whatever_the_block_size`, `the_driver_reproduces_the_runtime_ramp_law_bit_for_bit` and `a_new_point_restarts_from_the_current_value` RED |
| M5 | settled copy keeps the old zero-step add | all differentials equal base (equivalence evidence; no test can or should see it) |

## Codegen (re-measured; base worklet rebuilt by me: module `e25b045d...` = implementer's base)

- **Undo 2.** x86-64-v3 `f32x8` ramp loop 25 -> 22 (function 123 -> 123); `simd128` loop
  62 -> 57, function 291 -> 290; roster `route-mix-ramp f32x4` 22/0.
- **Undo 1, x86 loops.**
  - `f32x8`: dual 292/283/295 -> 287/278/290, mono 284/273 -> 278/267.
  - `f32x4`: dual 293/282/294 -> 289/279/291, mono 283/272 -> 279/267.
- **Undo 1, simd128 loops.** Dual 775/709/827/757 -> 756/694/808/742, mono 447/496 -> 442/491.
- **Undo 1, functions.**
  - `simd128` `process` 15,986 -> 15,876, `process_mono` 8,713 -> 8,685.
  - x86 `f32x8` `process_mono` 4,665 -> 4,678 (+13, named).
  - iOS 6,930 at head.
- **Undo 5.**
  - x86 `f32x8` loop 201 -> 187 ramping / 171 settled, function 805 -> 960.
  - `simd128` `f32x4` loop 441 -> 421 / 374, function 1,548 -> 2,034.
  - `f32` instantiation: see MINOR 1.
- **Undo 3** (natural shape rebuilt from `858ccb848`'s parent form). `recover_failed_lanes`:
  x86 `f32x4` 89 -> 77, `f32x8` 147 -> 126; iOS 95 -> 104; iOS count 0 -> 0. The record's reason
  ("longer on both shipped targets") holds for iOS. I did not re-measure the `simd128` figure
  (351 -> 360), because the site is off the frame loop.
- **Undo 4.** `#[inline(never)]` removed: iOS `true-peak-limiter` 6 -> **9**. D4 refuses, so the
  reason holds.
- **Undo 6.** `#[inline(always)]` on `silence_skip_settle`:
  - x86 `InputStage::process` `f32x4` 7,805 -> 9,615, `f32x8` 7,357 -> 9,005, `f32` 8,168 -> 11,169.
  - iOS `f32x4` 6,930 -> 8,780.
  - EQ `process_bank` +117.
  - builtins iOS count 5 -> 5.

  The reason ("multiplies the callers' code ... no measured render-time gain") holds.
- **V8 spill rows.** Base twin and head twin are identical. Held:
  - dual tail 110;
  - mono pair 79;
  - mono tail 52;
  - masked mono pair 86.

  Reported:
  - dual pair 187 (10 slots);
  - armed rows 128/219/92/58/102.

## Test value (tests edited, none added)

- `crates/lane/tests/filter_ramp_line.rs` (the owner-side leading builder is dropped). It now turns
  red when the kernel derives its leading window wrongly from its own countdown: an off-by-one
  floor (M2) or one countdown shared across a bank (M3). The pre-edit test could not see that,
  because it supplied the leading words itself. The edit strengthens it.
- `crates/lane/tests/input_chain_arming.rs` (argument removed only). Its claims are unchanged, and
  it neither gains nor loses a defect.
- `crates/builtins/tests/filter_liveness.rs` (doc and header only). Gate 8 is still red on LF1 and
  LF2 (M2, M3).

## Gates run (head export `30cfece25`)

- Spec gate-5 effect-crate `cargo test` (incl. `graph`): pass. CI `test-debug-b` exact: pass.
  `test-debug-a` and `builtins-compiler --no-run`: pass.
- `cargo test --release -p lane -p math -p wasm-gates --features math/lane`
  (`g5_native_digests_match_pins` ok): pass. Release `filter_liveness`: pass.
- `conformance_fixtures --check`: pass. `audit` release and `check-builtins-fixtures.sh`: pass.
- `check-graph-determinism.sh`: PASS 100/100. The first run failed only because the script
  hardcodes `target/debug` and I had set `CARGO_TARGET_DIR`; it was rerun with the default.
- `check-builtins-policy.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`, the
  known-defects `--self-test`: pass.
- `check-lane-policy.sh`: RED at head **and at base 6a2c5216a**, only on
  `crates/builtins/tests/tail_contract.rs:161` (`mul_add`). With that line neutralised in a
  scratch copy, head passes (ROOT 4).
- `cargo fmt --check`; `cargo clippy --workspace --all-targets -D warnings` in both the CI form
  (`--all-features`) and the spec form; `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps`:
  all pass.
- `check-cross-targets.sh`: PASS (counts above).
- `run-wasm-gates.sh`: exit 0 (native + `simd128` + V8).
- The worklet chain all passes (head module `7709b0e6...`):
  - `build-web-audioworklet.sh --named-twin`;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py`;
  - `check-scalar-oracle-absent.py`;
  - `test-web-audioworklet.sh`;
  - the spill self-test and the spill gate.

  My first chain run was aborted because I deleted a directory during the build. The clean rerun
  is the one recorded.
- Gate 4 premise checked:
  - None of the four benchmark fixtures has a soft clip.
  - The control table moves only the EQ band-1 gain, the compressor threshold and the limiter
    ceiling.
  - The three ride-along documents run with no control traffic.

  So no input filter retargets and no route ramps.

## Items for ROOT

1. **Gate 4 / D3's p50 clause for undos 1, 2 and 5 (ruling needed before merge).**
   - The answer to the question as asked: keeping them on codegen alone is *not* the letter of
     D3. D3 makes "p50 at most +2 % of base" a keep condition. The recorded tables exceed it in
     some cells:
     - undo 2, round 1: every row, +2.4 to +4.9 %;
     - undo 1, round 1: +2.4 % and +2.2 %;
     - undo 5, round 2: +2.4 to +4.0 %.
   - No p50 evidence that the spec requires is missing. Gate 4 asks for these tables, and they
     were taken as specified, once, with the load recorded.
   - The tables cannot measure the sites. I verified that no shipped benchmark document executes
     them (see Gates).
   - Recommendation: rule that codegen decides for on-loop sites that no shipped benchmark
     document reaches. That is consistent with D3's own off-loop clause and
     `benchmark-real-paths-only`; every changed loop is shorter except MINOR 1's +3.
   - Alternatively, order a measurement that reaches the sites as its own tooling issue: a native
     per-kernel p50, or a browser document with a soft-clip insert, filter retargets and send
     ramps.
   - Each undo is its own commit, so a contrary ruling is a clean revert. Undo 1 is two commits;
     see NIT 3.
2. **Ratify the path deviations in NIT 2.** All are forced.
3. **Authorize the four-place prose fix in NIT 1**, or assign it to a follow-up.
4. **The red `check-lane-policy.sh` belongs to #1329.** I confirmed it red at 6a2c5216a on
   `tail_contract.rs:161`. #1329 attempt 4 works in the same worktree.
