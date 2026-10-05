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

## Amendment 1 (root decisions, 2026-10-05, after attempt 1's verdict)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(a). Attempt 1's verdict
(`/home/bl/misofm/submix-verdicts/1328-attempt1.md`) found the code sound and the V8 spill
gate red, the D6 premises false and the authorized paths short. These decisions supersede the
Hazards line "report it; do not weaken the gate" and the D6 premises below; everything else stands.

- **A1. The spill gate's classifier is corrected, not weakened.** `flush_pair`'s two `mask_or` per
  SVF step lower to bitwise `vpor`, which is not a select. `SELECT_OPS` in
  `scripts/check-web-audioworklet-v8-spill.py` is corrected so `flush_pair`'s OR is not counted as
  a select while a real select (the dry-mask bitselect, a blend) still is. The checker's
  self-test gains a case proving a flush-pair `vpor` loop is select-free and a real-select loop is
  still masked. The carried-slot rule is unchanged, and the #1000 red arms are rebuilt and shown
  still red per the gate's own re-pin protocol.
- **A2. The dual depth-1 tail must not spill.** The integer flag that attempt 1's build carried in
  `[rbp-0xc8]` is the EQ's per-frame output-limit check for the two channels. That check is
  restructured in `crates/parametric-eq` so the dual depth-1 tail carries no stack slot. This
  must not change the check's semantics or any rendered bit. Acceptance by ruling is refused. Stream A
  has not started on this file; the touch point is recorded in `STREAMS.md`'s hot-file notes.
- **A3. The masked mono depth-2 pair's `ic1` state spill is eliminated too.** Only if the
  implementer proves with evidence that no correct encoding avoids it does the slice measure the
  browser cost (V8, the existing timing method) and record it here as an explicit accepted
  exception with that evidence; the gate then holds that row with its reason.
- **A4. D6 restated (class B, re-accepted).** The joint flush moves bits by change size, not only
  in tails: below about -160 dBFS input a low-frequency state word can stay under `REST_EPS` and be
  zeroed while input is live (10 Hz, S 0.1, +24 dB low shelf at 96 kHz fed -160/-180 dBFS), and in a
  multi-section chain the first moved sample can sit at a loud level (about -169 dBFS in the input
  HPF 100 Hz + LPF 22 kHz chain, where the HPF rests while the LPF still carries signal). Every
  change is below 2.2e-13 (< -253 dBFS; largest measured -256.7 dBFS). Accepted: the slice fixes a
  correctness defect (a limit cycle and a stuck fixed point) and the change is far below
  audibility. The listening line reads "every *change* is below -253 dBFS"; the PR evidence
  reproduces both named cases.
- **A5. Authorized paths extended** (each re-pin or constant change with its own reason, never bulk):
  the test modules of `crates/parametric-eq` (the four attempt-1 test edits are ratified) and, for A2,
  the EQ's per-frame output-check code in `crates/parametric-eq/src/lib.rs`;
  `scripts/check-web-audioworklet-v8-spill.py` (A1); `tools/bench/src/floor.rs`,
  `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh` (the floors 69 / 27
  / 307 become the ruling's derived values, and the ruling's interim "until their follow-up
  re-pins them" text is removed); `crates/lane/tests/g2_kernel_identity.rs` and
  `tools/audit/src/unfused_fma.rs` (their recurrence restatements adopt the joint flush);
  `scripts/lib/aarch64-known-defects.py` (memset ratchet rows parametric-eq 132 → 128, builtins
  194 → 186, as `check-cross-targets.sh` asks); stale prose in `docs/rulings/effect-floor-accounting.md`
  (appendix and "inert" definition), `crates/parametric-eq/src/lib.rs` (leg-(c) description) and
  `crates/builtins/tests/MUTATIONS.md`.
- **A6. Root decisions for attempt 2 (2026-10-05, no-shortcuts delegation).** Attempt 2 found
  that A2's integer spill is the EQ's two-channel output-limit flag and that A3's `ic1` spill is
  not caused by the flush encoding but by V8 rebuilding section 0's dry mask
  (`Channel::dry_mask`) inside the loop.
  - **A2, authorized.** The bounded lane kernel (`StoreBound`, `crates/lane/src/kernels.rs`)
    returns one folded `bool`; the EQ rescans with `check_block` only when it fails, so each
    channel's verdict is still the scan's. The required edits to
    `crates/lane/tests/g2_kernel_identity.rs` and the `interleave` adapter in
    `crates/parametric-eq` are authorized. Condition: a fix that unrelated edits can silently undo
    is acceptable only if a gate guards it. (1) The evidence identifies what makes the allocation
    stable, or states honestly that it is not structurally stable. (2) The V8 spill gate holds an
    explicit row for the dual depth-1 tail that goes red if the `[rbp-0xc8]`-class spill returns,
    proven by a mutation that re-introduces the spill. A structurally stable encoding, if found,
    is preferred.
  - **A3, authorized (not the timing fallback).** Each section's dry mask is kept in `Channel`
    state, updated alongside the identity flags (payload code in
    `crates/parametric-eq/src/lib.rs`), so the loop never rebuilds it. Output is bit-identical,
    and the spill gate's row for the masked mono depth-2 pair holds clean.

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

## Attempt record

### Attempt 1 (Terra, 2026-10-05, branch `codex/d15-stream-g`)

**Implemented.** D1 `REST_EPS`/`flush_pair` in `lane` (two ordered compares, `mask_and`,
`mask_or`, `andnot`; one generic body); D2 `svf_step` calls it, frozen-order docs updated in
`svf_block` (steps 7-8 -> 7) and `svf_step` (6-7 -> 6); D4 twin with its own private `REST_EPS`,
`Flushed` = "at least one word zeroed by either rule"; D5 `lane_is_inert`/`lane_is_flush_shaped`
now take the pair (per-word test **and** not "both below `REST_EPS` with a word non-zero"),
`section_state_is_*` call them, proof text updated; generator arm with `REST_EPS - 1 ulp`,
`REST_EPS`, `5e-15`; gates 1-3; D7 re-pin; docs.

**Outside the listed paths, forced by D5 (flagged for review).** Four unit tests in
`crates/parametric-eq/src/lib.rs`'s test module encoded the per-word law and went red:
`elision::a_non_inert_state_in_a_dead_section_refuses_elision` and
`stationary_subnormal::leg_c_refuses_below_flush_eps_admits_it_and_re_engages` admitted lone words
in `[FLUSH_EPS, REST_EPS)` beside a `+0.0` partner (the joint rule now zeroes that pair, so they
move to "refused" and `±REST_EPS` becomes the admitted lone-word floor);
`elision::an_elided_cascade_is_the_full_cascade_bit_for_bit` (and the interleave `compare`) seeded
`(1.5e-20, -1.25e-20)` as "a pair the kernel can write" (it no longer can; seed `ic1` is now
`2.0e-14`); `interleave_identity::prepared_hpf_and_last_lpf_are_reached_at_every_backend_width`
asserted a 1 kHz HPF still holds state 512 frames after an impulse (it now rests exactly; the
input is a unit step instead).

**D3 recomputed** (scratch, f64 over the cast `f32` words; `A = [[1-2c1, -2a2], [2a2, 1-2a3]]`,
unit-column eigenvector condition number): input HPF/LPF worst `6.659e-16` (44.1 kHz max cutoff,
rho 0.99994785, kappa 2.415); EQ grid (81 freq x 17 gain x 25 Q x 4 slope x 6 kinds x 4 rates)
worst `3.385e-15` (bell 10 Hz, Q 18, +24 dB, 96 kHz; rho 0.99999543, kappa 1.007); LR4 worst
`9.24e-18` (80 Hz, 96 kHz). Matches D3.

**Gates 1-2, rest sample** (end of the 128-frame block that reaches rest): input LPF 773,760;
EQ shelf 421,760. Both red on the per-word flush: gate 1 never rests within 2.4M samples (state
`l_lpf_ic1 = 0xa4c99363`), gate 2 stuck at `ic2 = 6.0120676e-20`.

**Mutation evidence** (introduce, run, revert):
- gate 3, rest test `mask_or` instead of `mask_and`: `g4_pair_law_cases_at_every_width` and
  `g4_pair_law_holds_at_every_width` red; `g4_flush_*` green.
- gate 3, rest test `a1.max(a2).lt(REST_EPS)`: both pair tests red (the cases test at the NaN
  arm).
- gate 3, rest test `le` instead of `lt`: both pair tests red.
- D5, pair term dropped from `lane_is_inert`: `a_non_inert_state_in_a_dead_section_refuses_elision`,
  the randomized restore differential and the three `ramping_elision` width tests red. The same
  mutation **without** the new generator arm leaves the randomized differential green: the
  `REST_EPS`-adjacent values are what make it reach the joint band.
- D5, pair term dropped from `lane_is_flush_shaped`: only `leg_c_refuses_below_flush_eps_admits_it_and_re_engages`
  is red (a kept section runs either way, so no rendered bit or integrator differs; the term is
  D5's mandated exactness of the predicate, not a bit-identity defence).
- Gates 1 and 2: red on the per-word kernel (above).

**Class-B evidence (one-time, scratch harness on the real kernels, not committed).** 240 runs:
4,800 samples of white noise then 1.2M samples of silence, at 0, -20, ..., -180 dBFS; 12 builtin
chains (`BuiltinChain`, LPF/HPF at both domain maxima at 44.1 and 96 kHz, HPF 10/20/80 Hz, LPF
20 Hz/1 kHz/20 kHz, two HPF+LPF bands) and 12 single-band EQs (`ParametricEqFactory`, including
the D3 bell and the gate-2 shelf), per-word flush vs joint flush, left output compared bit for bit.
- Largest change anywhere: `1.21e-13` (-258.3 dBFS), EQ low shelf 10 Hz S 0.1 at -160 dBFS.
  Builtins: `2.29e-14` (-272.8 dBFS).
- **Deviation from D6:** two runs moved samples while the input was non-silent: the 10 Hz +24 dB
  S 0.1 low shelf at 96 kHz driven at -160 and -180 dBFS (4,795 samples each, from sample 5; change
  at most `1.21e-13`, -258 dBFS, on output near -159/-179 dBFS). At that level both integrators sit
  below `REST_EPS` while signal is present, so the joint rule zeroes them every sample. No other
  run (and no run at -140 dBFS or louder) moved a sample during input.
- **Deviation from D6:** measured as the level of the first moved output sample, single-section
  runs move first at -267.5 dBFS or lower, but the HPF 100 Hz + LPF 22 kHz chain at 0 dBFS moves
  first at -156.7 dBFS (sample 7,637): the HPF's pair rests while the LPF still carries signal; the
  change itself is `3.55e-15` (-289 dBFS).
- Never-resting runs within the window: builtins 21 -> 0; EQ 29 -> 10 (the 10 remaining are the
  10 Hz Q 18 bell, still decaying above `REST_EPS` at 12.5 s, not stalled).

**D7 re-pin** (scalar oracle digests printed by `g5_native_digests_match_pins`; the three widths
agreed): `tools/wasm-gate-corpus/src/lane_digests.in` cases 1 `svf_block/low/impulse`, 5
`svf_block/high/impulse`, 9 `svf_block/band/impulse`, 13 `svf_block/bell/impulse`, 17
`svf_block_ramped/impulse`, 21 `svf_block_ramped/idle/impulse`. Each moved 1,152 of 8,192 samples
from frame index 331 on, largest change `5.8e-15` to `9.2e-15` (-285 to -281 dBFS). Reason: joint
SVF flush, D15-4(a): tail samples below -200 dBFS. `BUILTINS_DIGESTS`, `E9_DIGESTS`, the multiband
`DIGESTS`, `fixtures/builtins/v1/MANIFEST.tsv` and `conformance_fixtures --check` did not move.

**Gates.** Green: the gate-4 `cargo test` set (816 passed), the release `lane`/`math`/`wasm-gates`
set (107 passed, after the re-pin), `conformance_fixtures --check`, `check-builtins-fixtures.sh`
(50 files), `check-graph-determinism.sh` (100/100), `check-cross-targets.sh`,
`check-lane-policy.sh`, `check-dsp-research.sh`, `check-workspace-policy.sh`, clippy `-D warnings`,
`cargo fmt --check`, and the worklet chain (`build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`,
`test-web-audioworklet.sh`).

**Red: `scripts/run-wasm-gates.sh`, V8 spill gate (the Hazards case; not weakened).** Three held
rows fail closed: `scripts/check-web-audioworklet-v8-spill.py` classifies a loop as select-free
only if it has no `vpor`/`vorps`, and `flush_pair`'s two `mask_or` per step lower to `vpor`, so no
SVF loop is select-free any more. Spelling the pair as two chained `andnot` does not help (LLVM
refolds it to the `or`). With a scratch classifier that discounts two `or` per SVF step, the mono
depth-2 pair and mono depth-1 tail carry no stack slot, but **the dual depth-1 tail carries one
(`[rbp-0xc8]`)** where the base build carries none: a genuine V8 spill from the five extra
lane-ops. The dual depth-2 pair (reported, not held) goes from 10 to 13 carried slots. Both the
classifier change and the spill need a decision outside this slice's paths.

**Open items outside the listed paths.** `tools/bench/src/floor.rs` and
`scripts/console-benchmark-record-lib.jq` still pin builtins 69 / EQ 27 / strip 307 (the ruling
now states 79 / 32 / 322 and says so); `scripts/lib/aarch64-known-defects.py`'s parametric-eq
memset ceiling can drop from 132 to 128 (`check-cross-targets.sh` asks for it); the recurrence
restatements in `crates/lane/tests/g2_kernel_identity.rs` (oracle of
`g2_svf_step_yields_both_taps_of_one_state`) and `tools/audit/src/unfused_fma.rs` still use the
per-word flush (green, since their inputs never reach the joint band).

### Attempt 2 (Terra, 2026-10-05, branch `codex/d15-stream-g-1328`) -- partial, blocked

**A1 done (`691f4c1be`).** `selects` in `scripts/check-web-audioworklet-v8-spill.py`: a blend is
always a select; an `or` is one unless both inputs are lane masks computed in the loop (compares,
or bitwise ops over such masks alone). Unseen inputs count as a select (fails closed). Self-test 19
-> 23 cases; five classifier mutations each turn at least one case red (old "every or" rule: `flush
pair`; "no or": four cases; "any mask input": three; outside register as mask: `outside mask`;
"every logic op is a mask": two). Base (`0e3e21b68`) listings: every verdict unchanged. Head
listings now match every row; the dual tail's `[rbp-0xc8]` is reported as the real carry. The
#1000 red arms are not yet rebuilt (they need the final A2/A3 code).

**V8 rows (pinned Node, AMD EPYC 7313P, held rows marked H):**

| loop | base | attempt 1 | joint `bool` fold (scratch) | joint fold + masks laundered (scratch) |
|---|---|---|---|---|
| dual tail, select-free (H) | 0 | 1 (`[rbp-0xc8]`) | 0 | 0 |
| dual tail, masked | 0 | 1 | 0 | 0 |
| mono pair, select-free (H) | 0 | 0 | 0 | 0 |
| mono tail, select-free (H) | 0 | 0 | 0 | 0 |
| mono pair, masked | 0 | 1 (`[rbp-0x220]`) | 1 | 0 |
| dual pair (reported) | 10 | 13 | 13 | 13 |
| dual pair, masked | 11 | 12 | 11 | 15 |

**A2 finding.** Folding both channels into one `bool` in `StoreBound` (and letting the EQ rescan
the planes only when that fold fails, so the per-channel verdict stays the scan's) clears the dual
tail's integer slot. Not yet committed: the native gates cannot run (below).

**A3 finding: the spill is not the encoding's.** Every bit-identical encoding and schedule tried
leaves the masked mono pair carrying an integrator slot: attempt 1's `flush_pair`; compares
reordered; `r1 & (f1 | r2)`; chained `andnot`; per-word `flush` then rest on the flushed words; the
skewed kernel with forward section order; the flush after the output mix. The cause is V8's
scheduler sinking the section-0 dry mask -- built in `Channel::dry_mask` from four scalars and
`eq 1.0` -- into the loop (four scalar reloads, a broadcast, three inserts and a compare every
iteration; already so at base). The wasm computes it once before the loop. With the masks made
opaque before the loop (scratch `black_box`, not shippable) the pair is clean and every held row
stays clean. A shippable fix is to hold each section's dry mask in memory (a `Channel` field kept
with the identity flags), which is payload code in `crates/parametric-eq/src/lib.rs` outside A5's
paths: it needs a root decision.

**Blocked:** the host disk had 0.2-6 GB free during this attempt (other worktrees' targets hold
~75 GB); a wasm build failed with ENOSPC, and the gate-4 native set cannot be built in this
worktree. Not started: A4, A5 items, D6 restatement.
