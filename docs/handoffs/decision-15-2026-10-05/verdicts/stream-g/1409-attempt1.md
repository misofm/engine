FAIL

# #1409 attempt 1 -- adversarial verdict

Commit under review: `8deee2c5e` (branch `codex/d15-stream-g`), parent `c58c4e55f` (#1328 attempt 5).
Reviewed `git diff c58c4e55f 8deee2c5e` against the spec body at `8deee2c5e` (D0-D9, root
decisions, Attempt record), the #1408 spec and verdict (shared `ramp_toward`, its D1-D3), the #1301
amendment note, AGENTS.md, decision 15 and the no-shortcuts principle. All builds and mutations ran
in private `git archive` exports under `/tmp/claude-1002/v1409/` (`tree` = the commit, `base` = the
parent, `mut` = a mutation copy), each with its own `CARGO_TARGET_DIR`. Nothing was built, edited or
checked out in `/home/bl/misofm/wt-d15-g`.

The production code is correct. All nine sites use `ramp_toward(current, step, target)` with the
right operand order and keep their own snap. The site list is complete. The D2 restore argument
holds. No pinned bit moved, and no memset count rose. Every gate is green. One gap fails the
attempt: in the delay's gate-2 test, three of the delay's four ramped words are never observed at
their render site (site 7). A per-word revert of the clamp on feedback, damping `g` or cross
feedback keeps the whole delay suite green. A one-line test change fixes this (MAJOR 1).

## Findings

### MAJOR

1. **The delay's gate-2 test does not reach site 7 for feedback, damping `g` or cross feedback**
   (`crates/delay/tests/ramp_endpoint.rs:3-10`, `:151-164`, `:56`).
   - The test claims that each lane's feedback, damping coefficient `g` and mix, and the shared
     cross feedback, are guarded at site 7 (`delay_chunk`). It also claims that "a render site left
     on (or reverted to) the unclamped update passes its target on its move".
   - For the delay, the snapshot's ramp `current` is the `LinearRamp` word, which is site 2. It is
     never the kernel's iterated word. So site 7 is visible only through the partition half's
     output and ring comparison.
   - The test renders 72 frames from a fresh instance at the default delay time, 250 ms
     (`crates/delay/src/lib.rs:183-193`, 12,000 samples at 48 kHz). The taps are silent for the
     whole window. Feedback, damping and the cross matrix multiply a zero tap, so their kernel words
     cannot reach the output or the ring. Only the mix word can.
   - I reverted one word of site 7 at a time to `value + step` (left feedback, left damping, cross
     position). Each mutant stays green in `ramp_endpoint`, `conformance`, `determinism`,
     `randomized`, the delay `--lib` unit tests and the whole `conformance` package. The
     implementer's site-7 rows (the whole helper reverted, and D5 inverted) are red only through
     the mix move.
   - This matters because the issue promises that every word stays inside its endpoints on every
     frame. Gate 2 asks for one move per ramped word of sites 4-9. In every other effect the
     snapshot word is the render word, and every per-word mutant I tried there is red (the soft
     clip's three words separately, the multiband's right channel alone).
   - The fix is one line. Give the move live taps: in `values_with`, also set the delay time
     (parameter 0) to its 1 ms minimum, or pre-roll the ring past the delay time. I applied the
     first option in the mutation copy. The test stayed green unmutated, and each of the three
     per-word mutants turned red at `ramp_endpoint.rs:312`. Then re-record the per-word mutation
     rows.

### MINOR

1. **The multiband "whole" deviation is wider than its justification**
   (`crates/multiband-compressor/tests/ramp_endpoint.rs`, `MOVES` entries with `whole: false`;
   spec "Deviation, for the verifier").
   - The spec says that for every ratio, attack and release move, one-frame and one-block renders
     "differ in output whatever the ramp law". I set `whole: true` on each move in turn. Only high
     ratio and high attack fail (left output). Low ratio, low attack, low release and high release
     pass the full output-and-snapshot comparison.
   - Ruling: the deviation is accepted for high ratio and high attack. The frozen segment-start
     coefficient cache explains them, as `tests/identity.rs:792-796` documents.
   - Restore `whole: true` on the other four moves, and correct the spec sentence.
   - Little coverage is lost, because the ramp words, which are site 6's own words, are still
     compared on every move.
2. **In this commit, soft clip's `ramp_current_valid` line clause has no test**
   (`crates/soft-clip/src/lib.rs:310-322`).
   - The deleted `a_drive_overshoot_retargeted_inward_restores_and_continues` was the guard against
     flipping the sign or dropping the slope of `target - remaining * step`.
   - The deletion is correct. After the clamp, the effect's own snapshots never reach that clause,
     so the test's premise cannot happen.
   - This is acceptable only because #1411 (spec line 99) reduces `ramp_current_valid` to
     `converted_value_valid` in the same PR. #1411's verifier must confirm that the clause is gone.

### NIT

1. **Some doc text is false at this commit, and #1411 owns it in the same PR.** These passages
   describe the effect's own ramps crossing their edges, or the slack existing for the iterated
   `current + step`:
   - the soft clip `ramp_current_valid` doc (`lib.rs:278-284`, "Those are the effect's own words");
   - `crates/compressor/src/state.rs:120-122`, `:140`;
   - `crates/gate-expander/src/lib.rs:781`;
   - `crates/multiband-compressor/src/lib.rs:1432`;
   - `crates/transient-shaper/src/lib.rs:658-660`.

   The #1408 NIT 4 statements (`crates/builtins/src/lib.rs:4335`,
   `docs/BUILTINS_AND_METERING_V1.md:343`, `current += step`) are still open. They are outside
   #1409's paths.
2. **Gate 1's 100,000-ramp size runs nowhere in CI.** `effect-runtime` is not in `test-release` or
   in the spec's own release command, so per-PR CI runs only the 4,000-ramp debug prefix. The prefix
   checks its own reach (both halves must reach the clamp). I ran the full size by hand:
   `cargo test --release -p effect-runtime --test ramp_endpoint` passes in 6.0 s. Either add the
   release run or state 4,000 as the per-PR gate. #1408's law test runs in `test-release` because
   it lives in `lane`.
3. **Some doc lines are too long or badly wrapped.**
   - Four added doc lines exceed the 100-column width:
     - `crates/lane/src/kernels.rs` `IndexedRamp` doc (117);
     - multiband `store_segment` doc (107);
     - the delay test doc (103);
     - multiband `run_segment` flat-path doc (`lib.rs:899`, 102).
   - Two docs end a line early, mid-sentence: the compressor `advance_where` doc ("Returns the
     lanes") and multiband `lib.rs:1226` ("the identity on").
4. **The multiband `Segment` doc is split by the derive.** The new paragraph sits after
   `#[derive(Clone, Copy)]` (`crates/multiband-compressor/src/lib.rs:647-651`). Move it above the
   attribute.
5. **The gate-2 `request` doc says "the first quality row" but takes `qualities[1]`.** This is in
   all seven harness copies. The harness is copied seven times (about 400 lines each). The spec
   authorizes one file per crate, but a shared helper would stop the copies drifting apart; that can
   be a follow-up.
6. **Soft clip's settled path now has a branch.** `if ramping` (`crates/soft-clip/src/kernel.rs:208`)
   is a predictable branch inside the frame loop, so the settled path pays it on every frame. D5's
   hazard says the settled cost must not change. The cost is negligible, and the memset evidence
   shows the loop was not unswitched, but no cost evidence is recorded.
7. **Wasm reach is the same as #1408's.** The gate-2 tests are native only. `aarch64-debug` runs them
   at NEON width (the bank half at four lanes). On wasm simd128, the clamp acting in an effect
   kernel is covered only through `lane`'s D8 `pmin`/`pmax` lowering. The G5 corpus has no
   effect-ramp case in which the clamp acts (see "Bits moved"). A corpus case is optional.

## The ten points in the brief

1. **The nine sites. Correct, and the list is complete.**
   - Each site calls `ramp_toward(current, step, target)` in #1408's operand order. The strict D8
     `min`/`max` keep an in-range word's bits, and a NaN `next` passes through. Each site keeps its
     own snap:
     - the compressor `advance_where` select;
     - the gate prologue select;
     - the multiband segment split (no `remaining == 1` inside a segment);
     - the delay `ramp_bound` chunking (`advance_block` gives a zero step to a snap-only chunk);
     - the soft clip pre-segment `snap`;
     - the limiter `remaining > 0` select;
     - `next_value`;
     - `advance_block`'s first word.
   - **Zero steps.** For a zero step, `ramp_toward(c, ±0, T)` returns `c + step` bit for bit for
     every non-NaN `T`. This holds even when `T != c` (a restored ramp at rest) and for signed
     zeros, because the add result equals `c` in value and is never strictly outside
     `[min(c, T), max(c, T)]`.
   - **Soft clip.** The per-block `mask_any` choice is therefore correct per lane. In a bank where
     some lanes ramp and some are settled, the settled lanes take the clamp, and the clamp equals
     their old add exactly.
   - **Delay.** On a settled chunk all seven steps are `±0` (compared with `!= 0.0`), so `value +
     step == value` in value. A settled frame cannot move, so it cannot overshoot.
   - **Multiband.** `Segment.target` is gathered per track and per index. A right-channel-only
     revert is red.
   - **Search.** I searched every `+ step`, `+= step` and `.add(..step)` in the source tree. The
     remaining hits are outside this issue's scope:
     - non-goals: the EQ and input-filter SVF coefficient words (`lane/src/kernels.rs:895-900`,
       `builtins.rs:1237`, #1407) and the delay crossfade `alpha`, which is exact;
     - the #1301 probe's model walk (`conformance/src/randomized.rs:2842`), which D8 does not
       touch;
     - test witnesses (`multiband-compressor/src/split.rs:569`);
     - the f64 reference limiter's `ReferenceRamp` (`dsp-reference/src/true_peak_limiter.rs:180`),
       which is compared by tolerance, where an f64 overshoot is immaterial.
   - **Callers.** Every `ramp_path_inside` and `ramp_path_within` caller (compressor, gate,
     multiband, delay, transient shaper, limiter) restores a ramp that is now iterated by a clamped
     site. No caller is an unclamped (EQ) ramp.
2. **The restore walk. D2 holds.**
   - With D8 `min`/`max`, the result lies in `[min(c, T), max(c, T)]` in value whenever `next` is
     not NaN.
   - That interval nests inside the previous one, because `T` is a fixed endpoint. So by induction
     every walked word, and the snap's `T`, lies between the restored `current` and `target`.
   - The validator's remaining preconditions rule out a NaN `next`: `current` finite and inside,
     and the step finite. An overflow to `±inf` is clamped back to an endpoint.
   - The endpoint-only `ramp_path_inside` is therefore exactly the walk. Gate 1's restore half
     checks this against a walking reference over 200,000 forged ramps (any finite bit pattern,
     subnormal steps, `remaining` up to 65), and it passes.
3. **Deleted tests and rows. Each is truly superseded.**
   - The delay's two "path past the domain" rows, the limiter's `-1e30` row and the compressor's
     two payload tests/rows now describe payloads whose clamped render stays inside its endpoints.
   - The soft clip overshoot tests assert a premise that can no longer occur.
   - The randomized `crossed` counter was deleted as D8(b) directs. The bit-for-bit continuation
     stays.
   - Lost coverage: only the soft clip line clause (MINOR 2).
   - Rewritten oracles: `effect-runtime/tests/partition.rs` and the delay unit test restate the law
     with `ramp_toward`. Both are faithful, and the whole suite is green.
   - "No other effect test row rested on the walk" is confirmed: every DSP and workspace suite
     passes with the walk deleted.
4. **Edits outside the listed paths. All are legitimate.**
   - **Soft clip `corpus.rs` case 4.** The kernel API gained a target (D4). The fixture models a
     ramp in flight for the whole case, with no snap. The old kernel had no target, so there is no
     "natural" target whose digest is being hidden. A target `2 * FRAMES` steps out keeps the pinned
     words the same in-flight words, and the case now runs the clamp branch. The digest is unmoved,
     and the corpus does not hide a move: with every site reverted to the old law, `g5_native_corpus`
     still matches every pin (see 6).
   - **`polyphase_identity.rs`.** Zero steps, with targets equal to the case values. The clamp is
     the identity here.
   - **The `IndexedRamp` doc and `ramp_path_within` doc edits.** Accuracy fixes in authorized
     files.
5. **New tests.** Gate 1 and gate 2 test value is in the section below. Mutation table (each
   mutant applied alone, the named tests run, then reverted):

   | Mutant | Result |
   | --- | --- |
   | site 1 `ramp_block` add | gate 1 red (`ramp_block at f32 differs`) |
   | site 2 `next_value` `+=` | gate 1 (3 tests), compressor connected (threshold, frame 33, `0xc2a00001`), transient (attack, frame 48), delay (feedback, frame 33) red |
   | site 2 `advance_block` first word | gate 1, delay (mix, left output) red |
   | site 3 smoother | gate 1 red |
   | `ramp_path_inside` without the target clause | gate 1 restore test red |
   | site 4 `advance_where` add | compressor red |
   | site 5 gate prologue add | gate red |
   | site 6 `run_segment` add | multiband red |
   | site 6 right channel only on the old law | multiband red (section 1) |
   | site 6 `Side::segment` target from lane `W-1-track` | multiband bank test red |
   | site 7 `ramp_word` add / D5 inverted | delay red (mix, left output) |
   | site 7 left feedback / left damping / cross position alone | **green in all delay suites** (MAJOR 1); red with a 1 ms delay time |
   | site 7 D4 wiring (feedback carries mix target; cross carries its start) | green in `ramp_endpoint`; red in the existing `partition_invariance_over_1_7_64_128_512` |
   | site 8 drive / output / mix add, each alone | soft clip red (each word) |
   | site 8 D5 negated | soft clip red |
   | site 8 `target_vector` wrong lane | soft clip bank test red |
   | site 9 limiter add | limiter red (limit coefficient, frame 33, `0x3d6655c2`) |

   - **"No existing test catches it."** The parent commit runs all nine sites on the old law and
     passes the same pre-existing suites. So apart from the two rewritten oracles, no pre-existing
     test can be red on a site revert.
   - **Deviations.**
     - The multiband deviation is accepted for high ratio and high attack only (MINOR 1).
     - The delay damping interior move (`0x3e800021 -> 0.25`) is accepted. No edge move exists
       within 4096 word ulps, it is recorded, and it is in domain. But it reaches site 7 only with
       live taps (MAJOR 1).
     - The limiter move at -24 dB is accepted. -24 dB is the ceiling's lower domain edge, and the
       0 dB edge designs a single word.
6. **Bits moved. None.** I checked this two ways.
   - At the commit, `g5_native_digests_match_pins` passes (release, 7/7). That covers `D1`, `C1`,
     `GATE`, multiband, `G5`, `SOFT_CLIP`, transient, `D90`, `LANE`, `E9`, `M3` and `BUILTINS`. The
     browser `expected.json` digests also agree.
   - With every site reverted to the old law at once, the same corpus test still matches every
     pin. So the pinned corpora never reach an overshooting move: "no moved pin" means no reach,
     not a hidden move.
   - **Production reach.** It is real but narrow. I ran an f32 probe at `n = 64` over UI grids:
     - threshold at 0.1 dB, ratio at 0.1, mix at 0.01, makeup at 0.1 dB, feedback at 0.01;
     - about 100k adjacent and random moves;
     - result: 0 overshoot.

     The reason is that `step = Δ / 64` is exact and `|Δ|` is far larger than an ulp. An overshoot
     needs a start within about 33-4096 ulps of the target. Production reaches that through dense
     point batches or agent-computed values that converge on an edge. The engine does not enforce
     the parameter lattice on automation values, which is consistent with the probe's reachable
     pairs. The reach lives only in gates 1-2, as in #1408.
7. **Memset. No count rose.** I emitted iOS release assembly (`aarch64-apple-ios`, rlib) for both
   trees. Counts of `bl _memset_pattern16` are identical, base and commit:

   | Crate | Count |
   | --- | --- |
   | delay | 0 |
   | soft-clip | 22 |
   | compressor | 970 |
   | gate-expander | 91 |
   | multiband-compressor | 566 |
   | true-peak-limiter | 104 |
   | transient-shaper | 268 |
   | effect-runtime | 0 |
   | effect-contract | 0 |
   | lane | 0 |

   `check-cross-targets.sh` passes.
8. **#1301 counts are confirmed at 0/0/0.**
   - I re-measured on the clamped tree, exactly as #1301 gate 1 defines the mutations:
     - delay M18: 9.68 s per PR, 73.07 s full;
     - gate strict current;
     - limiter 4-ulp budget.
   - Each gave 0 "its own snapshot is refused" lines per PR and in the full run. Both lists are
     identical (empty), and the test is green after revert.
   - Positive control: with each mutation and the old law restored at that crate's site, the
     probes refuse 24 / 32 / 4, exactly #1301's recorded counts. So the clamp alone removes the
     catches, and the mutations were applied correctly. (The limiter refusals now fall after sample
     62, not 0, because the validator no longer walks.)
   - The #1301 amendment note matches. The PR-pair rationale (D9) stands.
9. **Gates. All green in the export** (results below).
10. **GitHub sync.** #1409's body equals its spec at `8deee2c5e` byte for byte (42,347 bytes, OPEN).
    #1301's body equals its spec byte for byte (20,316 bytes, CLOSED, amendment note included).

## Test value (new or rewritten tests)

- `every_statement_of_the_law_stays_inside_its_endpoints_and_agrees`
  (`crates/effect-runtime/tests/ramp_endpoint.rs`): red when `LinearRamp::next_value`,
  `advance_block`'s first word, `ParameterSmoother`'s `Linear` arm or `ramp_block` is left on the
  unclamped law, or when they disagree. Each of the four was verified red. No pre-existing test
  bounds an effect ramp word on an edge move.
- `the_ratio_floor_example_holds_its_target`: a regression reproducer of the probe's ratio example.
  It is red on the `next_value` revert (verified), so it is exempt as a reproducer.
- `a_restored_ramp_with_any_finite_step_stays_inside_and_the_validator_agrees`: red when
  `ramp_path_inside` loses an endpoint clause (verified), or when `next_value` stops holding a
  forged step between the endpoints (verified on the site-2 revert). Together these are what make
  the endpoint-only validator sound.
- `every_ramped_word_stays_inside_its_endpoints`, one per effect crate: red when that crate's render
  site is on the old law (verified per site, per word for soft clip, and per channel for multiband).
  The delay's copy reaches site 7 only through mix (MAJOR 1).
- `every_ramped_word_of_a_bank_lane_follows_its_own_target`, six crates: red when a D4 target
  gather takes another lane's target (verified for multiband `Side::segment` and soft clip
  `target_vector`). A scalar instance cannot see this.
- `crates/effect-runtime/tests/partition.rs` and the delay unit test (rewritten oracles): each keeps
  its own claim, the composition's and the chunked kernel's partition invariance, now stated in the
  clamped law. Neither ramp overshoots, so their bits are unchanged.

## Gates run (export `/tmp/claude-1002/v1409/tree`, fresh target dir)

| Gate | Result |
| --- | --- |
| `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p effect-contract -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support` | pass (168 result lines, 0 failed) |
| `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass (`g5_native_digests_match_pins` ok) |
| `cargo test --locked --release -p effect-runtime --test ramp_endpoint` (gate 1 at 100,000) | pass, 6.0 s |
| `test-debug-a` workspace command | pass |
| `bash scripts/run-wasm-gates.sh` | pass (native, simd128, V8 spill gate ok) |
| `conformance_fixtures -- --check` | pass |
| Worklet chain: build `--named-twin`, check `--without-metadata-regeneration`, `check-browser-expected-resources.py`, `test-web-audioworklet.sh` | pass (the output dirs were pre-created as `qualification.yml` does) |
| `check-cross-targets.sh` | pass |
| `check-lane-policy.sh`, `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh .`, `check-effect-contract.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh` | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | pass |
| `cargo fmt --all -- --check` | pass |
| #1301 gate-1 re-measurement (PR evidence) | 0/0/0, lists identical, green after revert; control 24/32/4 |

## To pass attempt 2

- Give the delay's gate-2 moves live taps: delay time at its 1 ms minimum, or a ring pre-roll.
- Record the per-word site-7 mutants red (left and right feedback, damping, cross position).
- Apply MINOR 1: restore `whole: true` on the four multiband moves that pass it, and correct the
  deviation sentence.

The NITs are optional, or belong to #1411.
