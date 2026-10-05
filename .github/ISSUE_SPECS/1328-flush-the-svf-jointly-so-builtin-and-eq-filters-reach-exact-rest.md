# Flush the SVF jointly so builtin and EQ filters reach exact rest

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(a)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

After real audio stops, every enabled TPT state-variable section (the builtin input HPF/LPF, the
parametric EQ bands, the multiband LR4 crossover) reaches exact rest, both integrators `+0.0` and
output `+0.0`, within a stated bound. Today two traps keep some of them ringing forever below
-200 dBFS, so silence skipping (#1107 and successors) can never rely on rest.

## Context

- **The recurrence.** `svf_step` (`crates/lane/src/kernels.rs:642-652`) is the only copy of the SVF
  state update. `svf_block` (`:132`), `svf_block_ramped` (`:665`), the masked variants (`:683`),
  the builtin fused chains (`crates/lane/src/kernels/builtins.rs:576`) and the multiband crossover
  (`crates/multiband-compressor/src/lib.rs:503-508`, `lr4_step`) all call it. Each new word is
  flushed alone: `ic1 = flush(n1)`, `ic2 = flush(n2)` (`kernels.rs:649-650`). The frozen operation
  order is written out at `kernels.rs:116-127` and `:634-641`.
- **The flush law.** `FLUSH_EPS = 1.0e-20` and `flush(x) = andnot(abs(x) < FLUSH_EPS, x)`
  (`crates/lane/src/lib.rs:163-188`). NaN passes (ordered compare). Gates G4
  (`crates/lane/tests/g4_flush.rs`) and G6 (`crates/lane/tests/g6_ftz_inert.rs`) hold it.
- **Trap 1, input filter (decision 15 evidence, round 1 B4 and round 2 B4(a)).** Where `c1` rounds
  to `1.0f32` (from 22,047.6 Hz at 44.1 kHz up to the domain maximum,
  `builtin_filter_cutoff_maximum_hz`, `crates/builtins/src/lib.rs:322-330`), the per-word flush
  creates a period-2 limit cycle: `ic1` alternates near `±1e-20 … 1.35e-16`, output about `±2e-21`
  forever. Measured: 44.1 kHz LPF at 22049.482 Hz, impulse 1234.5, cycles from sample 858,748.
- **Trap 2, EQ.** A low shelf 10 Hz +24 dB S 0.1 at 96 kHz, impulse 1, sticks at
  `ic1 = 0, ic2 = 6.01e-20` (a fixed point: `2*d2` is below half an ulp of `ic2`), output
  -361 dBFS forever. The EQ's silent fast path (`crates/parametric-eq/tests/silent_fixed_point.rs`)
  therefore never engages for it.
- **The oracle twin.** `ReferenceRetainedTptF32::process` (`crates/dsp-reference/src/tpt.rs:157-207`)
  restates the recurrence and the flush (`FLUSH_EPS` `:31`, helpers `:209-221`), and reports
  `ReferenceTptRetainedAction::Flushed` (`:14-25`). It feeds the builtins corpus
  (`crates/builtins/src/corpus.rs`) and the audit's builtins fixtures
  (`tools/audit/src/fixture_builtins.rs`).
- **EQ elision predicates rely on the per-word law.** `lane_is_inert`/`section_state_is_inert`
  (`crates/parametric-eq/src/lib.rs:1180-1209`) and `lane_is_flush_shaped`/
  `section_state_is_flush_shaped` (`:2147-2178`) accept any word that is `+0.0` or at least
  `FLUSH_EPS`. The elision proof (`:2200-2240`) argues `flush(ic) = ic` for a kept identity
  section. Under a joint flush that is false for a pair with both words in `[1e-20, 1e-14)`: the
  executed section zeroes it, the elided one keeps it. Only a restored payload can hold such a pair
  (the kernel never writes one), but the elision claim is exact bit identity of state, so both
  predicates must change. The randomized restore differential
  (`crates/parametric-eq/src/randomized_restores.rs:29-44`, generator) draws words near
  `FLUSH_EPS` on both words and would turn red if they did not.
- **The builtin input elision** (`section_is_identity`, `kernels/builtins.rs:1154-1163`) requires
  both integrators to be exactly `+0.0`; a joint flush keeps `(+0.0, +0.0)` there. Unchanged.
- **The cross-target corpus owner** is `g5_native_digests_match_pins`
  (`tools/wasm-gates/tests/g5_native_corpus.rs:35`, issue #1048). Its pins are
  `tools/wasm-gate-corpus/src/lane_digests.in` (`LANE_DIGESTS`, `tools/wasm-gate-corpus/src/lib.rs:1761`)
  and the delegated families' constants it reads (`g5_delegated_cases_use_the_owning_crates_pins`,
  `:230`): among the SVF users, `BUILTINS_DIGESTS` (`crates/builtins/src/corpus.rs:434`),
  `E9_DIGESTS` (`crates/parametric-eq/src/corpus.rs:205`) and the multiband `DIGESTS`
  (`crates/multiband-compressor/src/corpus.rs:252`, `src/corpus_digests.in`). G6 over the full
  corpus (`tools/wasm-gates/tests/g6_full_corpus_ftz.rs`) compares against the same pins.
- **Docs that state the law:** `docs/BUILTINS_AND_METERING_V1.md:55-59`;
  `dsp-research/filters.md` "Denormal, signed-zero and NaN policy" and "Latency and tail";
  op counts in `docs/rulings/effect-floor-accounting.md:142` (`flush` 3 ops), `:226` (`svf_step` 19)
  and `:369` (HPF section 24).

## Decisions frozen for this slice

- **D1. The law.** Add `pub const REST_EPS: f32 = 1.0e-14;` and
  `pub fn flush_pair<L: Lane>(n1: L, n2: L) -> (L, L)` beside `flush` in `crates/lane/src/lib.rs`:
  `a1 = n1.abs(); a2 = n2.abs(); rest = mask_and(a1.lt(REST_EPS), a2.lt(REST_EPS));`
  `ic1 = n1.andnot(mask_or(a1.lt(FLUSH_EPS), rest)); ic2 = n2.andnot(mask_or(a2.lt(FLUSH_EPS), rest))`.
  Two compares, never `Lane::max` (it is `select(gt)`, `lib.rs:358-360`, and would drop a NaN).
  Branch-free, one generic body at every width. `flush` and `FLUSH_EPS` stay as they are.
- **D2. One call site.** `svf_step` computes `n1 = ic1 + (d1 + d1)`, `n2 = ic2 + (d2 + d2)` and
  then `(s.ic1, s.ic2) = flush_pair(n1, n2)`. Nothing else in the frame body moves. The frozen-order
  docs at `kernels.rs:116-127` and `:634-641` change steps 7-8 / 6-7 to this one step.
- **D3. Why `1e-14` (numerical limit).** The per-word flush perturbs each word by at most `1e-20`
  per step, so a trajectory can only stall inside a ball of radius
  `κ·√2·1e-20 / (1 - ρ - 6·2^-24·κ)` (κ the eigenvector condition number of the step matrix,
  ρ its spectral radius). Largest radius per domain: input filter 6.66e-16 (maximum cutoff), EQ
  3.385e-15 (bell 10 Hz, Q 18, +24 dB, 96 kHz), LR4 9.2e-18. `1e-14` clears all three by at least
  3x; `1e-17` still left 566-614 never-resting runs per rate. The implementation note records this
  derivation with the numbers recomputed by the implementer (not copied).
- **D4. Twin.** `ReferenceRetainedTptF32::process` gets the same joint rule, with its own
  `REST_EPS` written out (the twin stays independent of `lane`). `Flushed` now means "at least one
  word was zeroed by either rule".
- **D5. EQ predicates.** `section_state_is_inert` and `section_state_is_flush_shaped` become pair
  predicates: a section's `(ic1, ic2)` is accepted iff each word passes today's per-word test **and**
  the pair is not "both magnitudes below `REST_EPS` with at least one non-zero". Update the proof
  text at `lib.rs:2200-2240` (`flush_pair(ic1, ic2) = (ic1, ic2)` for an accepted pair). Add
  `REST_EPS`-adjacent magnitudes (`f32::from_bits(REST_EPS.to_bits() - 1)`, `REST_EPS`, `5e-15`)
  to the restore generator in `randomized_restores.rs`, so the differential reaches the new band.
- **D6. Class B, accepted.** Bits move only in tails. Round-2 evidence: 240 noise-then-silence runs
  at 0 … -180 dBFS moved no sample while input was non-silent; in tails the first moved sample is at
  ≤ -204.8 dBFS and the largest change is 2.2e-13 (-253 dBFS). Accepted under decision 15 D15-4(a)
  and the class-B clause of `docs/rulings/effect-floor-accounting.md`. The PR reproduces these
  numbers on the real kernel as one-time PR evidence (not a committed test).
- **D7. Re-pin exactly one corpus.** Re-baseline only the pins `g5_native_digests_match_pins` reads,
  only for cases that move, from the scalar `Lane` oracle (never from a vector width or wasm),
  naming each moved case and the reason ("joint SVF flush, D15-4(a): tail samples below -200 dBFS")
  in the commit message. No other digest is re-pinned in bulk. If any other pinned artifact moves
  (`fixtures/builtins/v1/MANIFEST.tsv`, `conformance_fixtures --check`, a graph canonical digest),
  stop and list it in the PR with the moved bytes' level; it is re-generated one file at a time
  with its own generator and its own stated reason, or the slice reports it as a finding.
- **D8. Unchanged.** `flush`, `FLUSH_EPS`, the EQ restore admission (finiteness), `section_is_identity`,
  the delay's one-pole flush, all coefficient designers, latency (0) and parameter smoothing.

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF in stored A1 form, `v3 = v0 - ic2`, `d1 = -c1·ic1 + a2·v3`,
  `d2 = a3·v3 + a2·ic1`, `n1 = ic1 + 2d1`, `n2 = ic2 + 2d2`, then `flush_pair` [SIMPER-SVF]
  [ZAVALISHIN-TPT]. Coefficient and update rules unchanged.
- **Numerical limits:** D3. Ramps in flight are safe: the rule only zeroes state, and each step
  matrix stays non-expansive.
- **Latency and tail:** latency 0, unchanged. Exact rest becomes reachable; the stated bounds belong
  to *State a bounded tail and an exact-rest bound for every node* (#1329).
- **Denormal/NaN:** subnormals never reach state (per-word law kept, G4/G6 hold); NaN passes both
  ordered compares and is still caught by the per-block check; `-0.0` becomes `+0.0`.
- **Citations:** [SIMPER-SVF], [ZAVALISHIN-TPT] and [ORFANIDIS-ISP] (finite-wordlength effects and
  limit cycles in recursive filters), all in `dsp-research/BIBLIOGRAPHY.md`.
- **Fixtures and objective tests:** gates 1-4 below. **Benchmarks:** descriptive only; round-2
  scratch measured +10 % on one isolated section (latency-bound) and no measurable change on four
  interleaved sections; the floor-accounting rows record the +5 ops. **Listening:** none run; every
  moved sample is below -204 dBFS, under any reproduction chain's noise floor.

## Deliverables

1. `REST_EPS` and `flush_pair` in `lane`, with docs; `svf_step` calls it; frozen-order docs updated.
2. The twin updated (D4).
3. The EQ pair predicates, proof text and generator values (D5).
4. Two regression tests and one law test (gates 1-3).
5. Re-baselined G5 pins for the moved cases only (D7).
6. Doc updates: `docs/BUILTINS_AND_METERING_V1.md:55-59`, `dsp-research/filters.md` (both sections
   above), `docs/rulings/effect-floor-accounting.md` op rows (`flush_pair` 11 ops for two words, against 6 for two `flush`;
   `svf_step` 19 → 24; the HPF section and every row derived from them), and this spec's evidence.

## Authorized paths

- `crates/lane/src/lib.rs`, `crates/lane/src/kernels.rs`, `crates/lane/tests/g4_flush.rs`
- `crates/dsp-reference/src/tpt.rs`
- `crates/parametric-eq/src/lib.rs` (the four predicate functions and their docs only),
  `crates/parametric-eq/src/randomized_restores.rs`, `crates/parametric-eq/tests/exact_rest.rs` (new)
- `crates/builtins/tests/exact_rest.rs` (new)
- Pin data only: `tools/wasm-gate-corpus/src/lane_digests.in`, `crates/builtins/src/corpus.rs`
  (`BUILTINS_DIGESTS`), `crates/parametric-eq/src/corpus.rs` (`E9_DIGESTS`),
  `crates/multiband-compressor/src/corpus_digests.in`
- `docs/BUILTINS_AND_METERING_V1.md`, `dsp-research/filters.md`,
  `docs/rulings/effect-floor-accounting.md` (op-count rows), this spec

## Non-goals

- Tail or rest *values* in any descriptor (#1329). Capping the cutoff domain. A rest detector on
  exact-zero input. Any change to the delay, compressor or limiter one-pole flushes.
- Rejected alternatives (round 2): dropping the per-word flush (1,876/6,400 DC runs kept subnormal
  words; loses G6); capping the cutoff domain (empirical, leaves 48.3M-sample rests, misses trap 2);
  "flush ic1 when ic2 flushes" (zeroed a -24 dBFS partner word twice in 64M steps, a click);
  magnitude truncation (moves audible bits).

## Hazards

- `scripts/run-wasm-gates.sh`'s V8 spill gate holds the shipped EQ cascade loops to V8's register
  allocation; +5 ops per section may spill. If it fails, report it; do not weaken the gate.
- Stream A edits payload code in `crates/parametric-eq/src/lib.rs`; touch only the predicate
  functions named in D5.

## Objective gates

1. **Input LPF period-2 cycle** (`crates/builtins/tests/exact_rest.rs`): `BuiltinChain::new(44_100, …)`
   with left and right `lpf_hz = 22049.482` (the domain maximum), trim 0 dB; one block whose first
   frame is `1234.5`, then zeros in 128-frame blocks. Assert all eight
   `test_support::input_state_words` reach `0` and the output is exactly `+0.0` within 2,400,000
   samples, and stay so for 4 more blocks. Red on revert (cycle from sample 858,748).
2. **EQ shelf fixed point** (`crates/parametric-eq/tests/exact_rest.rs`): the EQ effect prepared at
   96 kHz with `single_section_values(EqBandKind::LowShelf, 10.0, 24.0, FRAC_1_SQRT_2, 0.1)`,
   impulse `1.0`, then zeros. Assert band 0's two state words (`snapshot` + `band_word`) reach `0`
   and the output is exactly `+0.0` within 9,600,000 samples (100 s; round-2 measured the EQ's
   worst rest at about 89 s). Red on revert (stuck at `ic2 = 6.01e-20`).
3. **Pair law** (`crates/lane/tests/g4_flush.rs`, at every width the build has): both words below
   `REST_EPS` → both `+0.0`; one word at or above `REST_EPS` → each word follows the per-word law,
   bit for bit; NaN in either word passes through; `-0.0` → `+0.0`.
4. Every existing gate green, re-pins limited to D7:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - `bash scripts/run-wasm-gates.sh` (native and simd128 legs, V8 spill gate)
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-dsp-research.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a kernel whose flush lets a near-Nyquist input filter cycle forever (the period-2 limit
  cycle) is red here; no test drives a section to rest at the top of the cutoff domain.
- Gate 2: an SVF that sticks at a non-zero fixed point (the EQ shelf trap) is
  red here; `silent_fixed_point.rs` only checks the fast path is bit-neutral, not that it is reached.
- Gate 3: a pair rule built from OR instead of AND (zeroes an audible partner word: the click the
  plan rejected), or from `Lane::max` (hides a NaN), is red here; gates 1-2 stay green on both.
- The generator values (D5) extend an existing randomized differential; judged by reach: they make
  it reach the joint band, where a per-word EQ predicate would elide a section the kernel changes.

## Dependencies

none
