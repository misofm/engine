# Retarget an SVF's ramp words jointly in every shipped effect

Filed by root from the #1372a verification (backlog sprint, 2026-10-24). **May be class B: an owner ruling is needed for any shipped effect whose rendered bits move.**

## Finding

A state-variable filter's three design words (c1, a2, a3) ramp as separate `LinearRamp`s. If a retarget restarts only the words whose target bits changed, a word that is mid-ramp keeps its old schedule while the others restart, and the loaded word triple leaves the chord between two designs. Such off-chord triples are not contractive: the #1372a verifier measured a spectral radius of 1.36 at c1 and a3 at 1 kHz with a2 at 12 kHz (k = 1/8, 48 kHz), and 1.008 at a geometric-restart point. In the de-esser this was reachable (4452 of 8000 one-ulp frequency nudges at Q 8 / 48 kHz keep a2's bits); the de-esser is being fixed before registration (PR #1922). The parametric EQ's ramp-path tolerance (`crates/effect-runtime/src/svf.rs`, `RAMP_PATH_NORM_TOLERANCE`, and #1859's ramp-path grid) assumes lockstep ramps.

## Smallest closable slice

1. For each shipped effect with SVF words or other multi-word filter designs (parametric EQ, the builtins input HPF/LPF, the multiband crossovers, the dynamic EQ), state by reading and one probe whether a retarget can restart some words and not others.
2. Where it can, the fix is a joint retarget (restart every word of the design from its current value whenever any target word changes). For a shipped effect this moves bits on retarget paths: bring the per-effect evidence to the owner before the change (class B).
3. Where it cannot, record why, and add one assertion to the effect's existing ramp-path test that would catch a future per-word skip.

## Paths

Per effect, decided by the probe; this spec.

## Evidence and decision record (attempt 1, 2026-10-24, base main 068742aba)

Verdict: no shipped effect restarts some words and not others; no rendered bit moves, so class B does not arise. No `src/lib.rs` changed in any effect crate.

| Effect | Reading | Guard |
|---|---|---|
| De-esser | `crates/de-esser/src/lib.rs` `retarget` restarts c1, a2, a3 together (PR #1922). | `crates/de-esser/tests/svf_chord.rs` |
| Dynamic EQ | `crates/dynamic-eq/src/lib.rs` restarts all nine design words of a band together (#17 S2). | `crates/dynamic-eq/tests/svf_chord.rs` |
| Parametric EQ | `start_ramp` rewrites all six words' targets and steps from the words in force under one shared `remaining`; the stationary hoist settles only when all six already match. | existing, see below |
| Builtins HPF/LPF | `apply_prepared_filter` (`crates/builtins/src/lib.rs`) skips only when the whole six-word in-flight target equals the new one; otherwise every step is recomputed from the current word under one shared `filter_remaining`. | existing, see below |
| Multiband crossover | An accepted crossover span calls `LinearRamp::set_target` on all three LR4 words unconditionally; `set_target` restarts from `current`. | new leg (c) of gate 2, `crates/multiband-compressor/tests/crossover.rs` |

Mutant runs (each applied to the committed tree, run, then restored with `git checkout` and `touch`):
- Parametric EQ, `start_ramp` skips a word whose target bits are unchanged: red on `contract.rs` gate 3 `every_live_enable_or_family_ramp_is_contractive_and_bounded` ("the in-test retarget walk is the lane's walk", 44.1 kHz 20 Hz -24 dB Q 0.1, 0 -> 4 -> next) and on `randomized.rs` `the_bank_renders_its_scalar_instances_under_random_state`.
- Builtins, an in-flight word whose target bits are unchanged keeps its old step: red on `filter_liveness.rs` gate 8 `every_lane_steps_exactly_four_words_from_its_own_countdown` (bit-exact against the joint-retarget oracle).
- Multiband, `set_target` guarded on `ramp.target.to_bits() != target.to_bits()`: no existing test red (all 14 test binaries green); the new leg (c) is red at frame 33 (word fractions 0.03125 against 0.015625).

Decision D1 (root: one assertion per crate) is applied where it adds a claim. The parametric EQ and the builtins already have a test that the mutant turns red, so by the one-test-per-claim rule no PEQ or builtins assertion is added; the multiband crossover had none, so it gets one. Nudge pair found by the leg's search: 1000 Hz -> 1000.00006 Hz at 48 kHz keeps c1's bits and moves a2 and a3.

Test value, leg (c): a crossover retarget that restarts only the words whose target bits moved leaves a mid-glide nudge off the chord, and no other multiband test retargets the crossover mid-glide.

Doc: `RAMP_PATH_NORM_TOLERANCE` (`crates/effect-runtime/src/svf.rs`) now names the lockstep precondition.
