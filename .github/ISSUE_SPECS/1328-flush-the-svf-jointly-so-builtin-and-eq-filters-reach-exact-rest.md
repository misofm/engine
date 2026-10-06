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
- **A4. D6 restated (class B, re-accepted; premises corrected by A7; acceptance withdrawn and restated by A8 below; A8's law superseded by A9).** The joint flush moves
  output bits by a bounded amount, at any input level. While every input sample satisfies
  |x| < L* = REST_EPS/(2·max(a2,a3)), the section stays at rest and outputs only its direct term
  (m0 + m1·a2 + m2·a3)·x; the change at the section's output is at most ‖h − g_direct·δ‖₁·L*. L* is
  at most 3.05e-11 (−210.3 dBFS) (EQ low shelf 10 Hz +24 dB 96 kHz); worst change about 5.0e-10
  (−186 dBFS), measured 4.94e-10. For input at or above −140 dBFS every change measured is below
  3.2e-13 (−250 dBFS). Below L* a section loses its whole response (up to +24.5 dB; the HPF passes
  DC below about −216 dBFS). The bound 5.0e-10 is below the f32 rounding error of any signal above
  about −30 dBFS passing the same section (half an ulp at 0.03 is about 1e-9), so the dead zone sits
  under the arithmetic noise the engine already accepts; it is a defined rest threshold, not a
  deferred defect. Accepted under D15-4(a). The listening line reads "every change is below
  −186 dBFS at the section's output; no listening run".
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

- **A7. Root decisions for attempt 3 (2026-10-05; doc + one test; no DSP change).** Decided by
  the decision-15 root coordinator under the owner's delegation of math decisions
  (`no-shortcuts-correctness-first`, `user-math-expertise`), after attempt 2's verdict
  (`/home/bl/misofm/submix-verdicts/1328-attempt2.md`, FAIL on M1 only).
  1. A4 re-affirmed on this corrected text (replace A4's premises and the Listening line): "The
     joint flush moves output bits by a bounded amount, at any input level. While every input
     sample satisfies |x| < L* = REST_EPS/(2·max(a2,a3)), the section stays at rest and outputs only
     its direct term (m0 + m1·a2 + m2·a3)·x; the change at the section's output is at most
     ‖h − g_direct·δ‖₁·L*. L* is at most 3.05e-11 (−210.3 dBFS) (EQ low shelf 10 Hz +24 dB 96 kHz);
     worst change about 5.0e-10 (−186 dBFS), measured 4.94e-10. For input at or above −140 dBFS
     every change measured is below 3.2e-13 (−250 dBFS). Below L* a section loses its whole
     response (up to +24.5 dB; the HPF passes DC below about −216 dBFS). The bound 5.0e-10 is below
     the f32 rounding error of any signal above about −30 dBFS passing the same section (half an ulp
     at 0.03 is about 1e-9), so the dead zone sits under the arithmetic noise the engine already
     accepts; it is a defined rest threshold, not a deferred defect. Accepted under D15-4(a)."
     Listening line: "every change is below −186 dBFS at the section's output; no listening run".
     Rejected alternative: gating the flush on x == 0 exactly (no dead zone) -- adds a lane compare
     to the mask #1328 fought to keep spill-free, buys nothing measurable at −186 dBFS. The
     dead-zone limit (L* formula, worst case, the HPF DC note, LR4 ~1.9e-12 at 80 Hz, builtin 10 Hz
     ~1.5e-11) goes into `dsp-research/filters.md` (numerical limits).
  2. Ratified: A3 (cached dry mask) satisfies A2, given the held dual-tail row goes red when the
     spill returns. Keep the honest note that the allocation is not structural. Correct the stated
     cause per the verifier (m3): reverting only the tail's own mask build (parametric-eq `lib.rs`
     ~:2587) brings the slot back; reverting only the dual masked pair's masks leaves the tail clean.
  3. Authorized: `tools/wasm-gates/MUTATIONS.md` and the spill gate's re-pin paragraph name #1328
     attempt 1's code (dual tail `[rbp-0xc8]`, masked mono pair `[rbp-0x220]`) and #977 attempt 1
     (module `0db9b2f5`, `[rbp-0xa0]`) as the red arms; record the bisect: one-token arm red at
     `6f4c0379e` (#1009) and `d09d50248`, green since the #999 merge `27cf24132` (tail 84 -> 112
     instructions), never red on `main` (green at `a9414c0c6`).
  4. Authorized: one EQ test covering the desymmetrize copies of the dry and identity caches; must
     go red on a stale-identity-flag mutant (and on a dropped dry copy); record red/green.
  5. Fold NITs: the self-test pins the memory-operand arm (dropping `not from_memory` must turn a
     case red); the spec's "five held rows" vs "four" -> four held rows + one reported; note that T2
     (builtins) no longer exercises the twin's per-word flush arm (fix or record).
  6. Do NOT edit decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`);
     the root relayed A4 to S0. A7 stays in this spec and the note in `dsp-research/filters.md` only.

- **A8. Root decisions for attempt 4 (2026-10-05; supersedes A4/A7's acceptance of the dead zone; its law is superseded by A9 below).**
  Decided by the decision-15 root coordinator under the owner's delegation
  (`no-shortcuts-correctness-first`), after attempt 3's verdict
  (`/home/bl/misofm/submix-verdicts/1328-attempt3.md`, FAIL: M1 chain bound, M2 prose). Root
  withdraws A4/A7's acceptance: "below the arithmetic noise the engine already accepts" is disproved
  by the chain measurement (four +24 dB shelves at 10 Hz, input `3e-11`, change about `1.5e-6`,
  -116 dBFS, above a 24-bit LSB and growing with every boosting section). An EQ must apply its
  response to whatever it is given; silence is exact zero, and "skip work on silence" needs only
  that.
  1. The joint flush (the `REST_EPS` pair rule) fires only when the section's input is exactly zero
     (`+0.0` or `-0.0`) for the sample, together with the existing `REST_EPS` state condition. The
     per-word `FLUSH_EPS` law is unchanged. If denormal inputs reach the section and must also be
     gated, the threshold may be `|x| < f32::MIN_POSITIVE`, with the reason recorded; nothing
     larger. This supersedes D1's `rest` term and D2's call (`flush_pair` takes the section input).
  2. Re-measure and restate the class-B bound (A4) for the new law at section and chain level (four
     +24 dB shelves at 10 Hz and at 100 Hz, 96 kHz, inputs down to `1e-30`), and replace the
     dead-zone text in `dsp-research/filters.md`. Gates 1-2 must still hold.
  3. New gate: a chain test where tiny non-zero input through boosting shelves gets its full boost,
     red on attempt 3's law.
  4. The V8 spill gate stays clean on all held rows. If the extra lane compare causes a spill,
     report it with the measured cost before working around it; do not weaken the gate. Up to 2 %
     render cost is acceptable (owner allowance).
  5. Every `svf_step` call site threads its section input to the flush (builtins, EQ, LR4
     crossover), and so do the twin, the g2 oracle, the audit's `unfused_fma` model and the EQ
     elision predicates and proof where affected. Op counts in the floor ruling and the floors
     (`tools/bench/src/floor.rs`, `scripts/console-benchmark-record-lib.jq`,
     `scripts/test-console-benchmark.sh`) follow; G5 re-pins only for cases that move,
     individually with reasons.
  6. Fold M2: a stale identity flag *can* elide a live section; correct the test doc and the
     attempt-3 record, and add a left-only phase without `-0.0` input so the EQ test is red in
     release too (record red/green in debug and release).
  7. Fold the NITs: `filters.md` "5 Hz square" -> 3.84 Hz; qualify "V8 sank their construction into
     it" as unmeasured; add a parametric-eq `MUTATIONS.md` row for the new test.
  8. Amend decision 15's "Root decisions after S0" entry for #1328 to point at A8 (authorized), and
     sync GitHub #1328's body to this spec.

  **A4 restated under A8 (measured in attempt 4; class B, the bound the root asked for).** The joint
  flush moves output bits only on samples whose section input is exactly zero: while the input is
  not zero every section runs the per-word law bit for bit, so no output sample moves while a
  signal is present, at any level (measured on the real EQ, 96 kHz, inputs from 1 down to `1e-30`,
  section and four-shelf chain). In tails it ends a decay below `REST_EPS`: at most `1.9e-13`
  (-254.5 dBFS) at one +24 dB low shelf's output, and at most `3.4e-10` (-189.4 dBFS) at the output
  of four cascaded +24 dB low shelves (10 Hz and 100 Hz), where every downstream section boosts
  the tail an upstream one drops. A non-zero sample followed by exact zeros is such a tail: an
  isolated impulse below `L*` (`3.05e-11` at the 10 Hz +24 dB shelf) loses its tail after the first
  zero sample, at most `1.6e-10` (-195.8 dBFS) through four shelves. The per-word law's own dead
  zone (inputs below about `3e-17`, -330 dBFS) is unchanged. Listening line: "the joint flush moves
  bits only after an exact-zero input; largest change measured -189 dBFS at a four-shelf chain's
  output (-254 dBFS at one section's); no listening run".

- **A9. Root decisions for attempt 5 (2026-10-05, the last attempt; supersedes A8's law).** Decided
  by the decision-15 root coordinator under the owner's delegation (`no-shortcuts-correctness-first`),
  after attempt 4's verdict (`/home/bl/misofm/submix-verdicts/1328-attempt4.md`, FAIL): A8 still lost
  a sparse signal's boost (non-zero samples below `L*` with exact zeros between them) and moved bits
  inside a live EQ whose own high-pass cancels to an exact zero (M1). Option (b2), with the
  `max_u32` saving; not option (a), which would land a -113 dBFS sparse dead zone to fix later.
  1. **Silence is a time property.** Per effect input and channel, `z' = (x == ±0.0) ? z + 1 : 0`
     (saturating); a section may apply the `REST_EPS` joint rule only when `z' >= N_SILENCE` and
     both words are below `REST_EPS`. The per-word `FLUSH_EPS` law is unchanged. "Effect input" is
     the builtin input stage's input, the parametric EQ's input and the multiband compressor's
     input, per channel; every SVF user reads its own effect input's counter. Whether A8's section
     input `x == 0` term is also kept is decided by evidence (kept if needed for correctness,
     dropped otherwise, the reason recorded). This supersedes D1's and A8's `rest` term and D2's call.
  2. **`N_SILENCE`** is a stated constant, not a knob: 1,024 frames at every launch rate unless a
     rate's measured residual exceeds A8's tail level, in which case it is stated in time and
     derived in frames per rate; the choice and its residual at all four launch rates are recorded.
  3. **Gates.** `[a, 0, a, 0, ...]` and `[a, 0, 0, 0, ...]` with `a < L*` through four +24 dB
     shelves (10 Hz and 100 Hz, every launch rate) get every boost (red on A8 and on attempt 3's
     law); the HPF-cancellation case moves nothing while the effect input is live (red on A8);
     continuous-then-silence tails stated as reachable bounds (analytic or worst-case search), not
     sampled maxima (m2); gates 1-2 still hold with their rest samples updated.
  4. **Subnormals.** Show that no section state goes subnormal inside the window on the wasm path
     (no FTZ there), or add a per-word `|w| < f32::MIN_POSITIVE` flush with its reason.
  5. **`max_u32`**: a new `Lane` op (`vpmaxud` / `i32x4.max_u` / `umax`), NaN-exact, with its own
     lane test at every width; `flush_pair`'s joint test is one compare of the larger magnitude.
  6. **Spill gate**: the four held rows stay clean; report the codegen op counts per SVF step (V8
     and AVX2) and the descriptive p50 (one warmup, two rounds).
  7. **State**: one word per channel per effect input; every sealed size or payload change is
     recorded with its reason, and the counter carries across plan swaps and restores consistently
     (or resets with a stated, safe semantics).
  8. Floors and op counts follow (`docs/rulings/effect-floor-accounting.md`,
     `tools/bench/src/floor.rs`, `scripts/console-benchmark-record-lib.jq`,
     `scripts/test-console-benchmark.sh`); the twin, the g2 oracle and the audit's `unfused_fma`
     model follow the law; G5 re-pins only for cases that move, individually with reasons.
  9. Fold m2, n1 (dropping the audit model's silence term must turn something red) and n3 (document
     or fix `lane_is_inert`'s refusal of elision under continuous input). Amend A4/A7/A8's text here,
     in `dsp-research/filters.md`, `docs/BUILTINS_AND_METERING_V1.md`, `crates/lane/src/lib.rs` and
     decision 15's "Root decisions after S0" #1328 entry (authorized), and sync GitHub #1328's body.

  **A4 restated under A9 (measured in attempt 5; class B).** The joint flush moves output bits only
  after the effect's input has been exactly zero for `N_SILENCE` samples, `4096 / 48000` s
  (85 1/3 ms: 3,764 / 4,096 / 7,527 / 8,192 samples at 44.1 / 48 / 88.2 / 96 kHz). While the effect
  input is live -- any non-zero sample in the last `N_SILENCE`, however small or sparse -- every
  section runs the per-word law bit for bit, and a section fed an exact zero by the section before
  it is not armed by that zero. In tails, one section's change is at most `1.95e-13` (-254.2 dBFS)
  analytically at a +24 dB 10 Hz or 100 Hz low shelf, S 1 (`3.37e-13` at S 0.1); the largest found
  at four cascaded shelves is `3.47e-10` (-189.2 dBFS), and the analytic chain bound for one tail is
  `3.1e-7` (four 10 Hz shelves, 96 kHz). The per-word law's own dead zone (about `3e-17`,
  -330 dBFS) is unchanged. Listening line: "the joint flush moves bits only after `N_SILENCE` of
  exact-zero effect input; largest change found -189 dBFS at a four-shelf chain's output (-254 dBFS
  at one section's, analytic); no listening run".

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF in stored A1 form, `v3 = v0 - ic2`, `d1 = -c1·ic1 + a2·v3`,
  `d2 = a3·v3 + a2·ic1`, `n1 = ic1 + 2d1`, `n2 = ic2 + 2d2`, then `flush_pair(n1, n2, rest)`, with
  `rest = silence_step(x, run, N_SILENCE)` from the effect input `x` (A9) [SIMPER-SVF]
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
  interleaved sections; the floor-accounting rows record +4 ops per section (`flush_pair` 10
  against 6) and 5 per channel sample for the silence counter (A9).
  **Listening:** the joint flush moves bits only after `N_SILENCE` of exact-zero effect input;
  largest change found -189 dBFS at a four-shelf chain's output (-254 dBFS at one section's,
  analytic); no listening run (A4 as restated under A9; the rest rule and its bounds are in
  `dsp-research/filters.md`, numerical limits).

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
   `REST_EPS` on an armed threshold (`REST_EPS`) → both `+0.0`; on an unarmed threshold (`+0.0`)
   or with one word at or above `REST_EPS` → each word follows the per-word law, bit for bit; NaN
   in either word passes through; `-0.0` → `+0.0` (A9). The counter law (`run' = (x == 0) ? run + 1
   : 0`, saturating at `2^24`; the threshold armed from `N_SILENCE`), `silence_frames` per rate,
   `silence_armable` exactness and `silence_advance` are held at every width beside it, and
   `max_u32` in `g1_op_identity.rs`.
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
5. **Chain boost (A8)** (`crates/parametric-eq/tests/exact_rest.rs`): four +24 dB low shelves at
   10 Hz and at 100 Hz, 96 kHz, fed a 3.84 Hz square at `0.03 * 2^-k` (`k` = 30, 34, 36, below one
   section's old rest limit), scaled back by `2^k`, match the same square at `0.03` within `1e-4`
   of its peak. Red on attempt 3's law.
6. **Sparse boost (A9)** (same file): the same shelves at every launch rate, fed `[a, 0, a, 0, ...]`
   and `[a, 0, 0, 0, ...]` (`a` the square at `0.03 * 2^-36`, below `L*` everywhere), after a lead
   of armed silence, scaled back by `2^36`, match the ordinary run within `1e-6` of its peak. Red on
   A8's law and attempt 3's.
7. **Cancellation (A9)** (same file): the EQ's 40 Hz high-pass ahead of four +24 dB 100 Hz shelves
   on a DC and a slow-square input; the tiny run scaled back matches the ordinary run within
   `1e-9` of its peak on every sample. Red on A8's law.

## Test value

- Gate 1: a kernel whose flush lets a near-Nyquist input filter cycle forever (the period-2 limit
  cycle) is red here; no test drives a section to rest at the top of the cutoff domain.
- Gate 2: an SVF that sticks at a non-zero fixed point (the EQ shelf trap) is
  red here; `silent_fixed_point.rs` only checks the fast path is bit-neutral, not that it is reached.
- Gate 3: a pair rule built from OR instead of AND (zeroes an audible partner word: the click the
  plan rejected), or from `Lane::max` (hides a NaN), is red here; gates 1-2 stay green on both.
- Gate 5 (A8): a joint flush that ignores the section input (attempt 3's law) zeroes a tiny
  signal's state in each boosting section and loses the cascade's boost; red here (misses by the
  whole peak), while gates 1-2 stay green on it.
- Gate 6 (A9): a joint flush armed by a zero section input (A8's) zeroes the first shelf's state
  between a sparse signal's samples and loses its boost; red here, while gates 1-2 and 5 stay green
  on A8's law. It also catches an EQ block that reads a stale armed rest plane after silence.
- Gate 7 (A9): a joint flush armed by a section's own input (A8's) moves bits inside a live EQ whose
  high-pass cancels a DC input to exact zero; red here, while every other EQ test stays green on it.
- The counter law, `silence_armable` and `max_u32` tests (gate 3): a counter that does not reset, an
  off-by-one arming, a late block-level arming decision, or a `max_u32` built from the float `max`
  (drops a NaN, misorders `-0.0`) is red there and nowhere cheaper.
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

### Attempt 2 (Terra, 2026-10-05, branch `codex/d15-stream-g-1328`)

Host: AMD EPYC 7313P, Node v22.23.2 (V8 12.4.254.21-node.56), rustc 1.97.1. Commits: `691f4c1be`
(A1) and the attempt-2 commit that carries this record.

**A1, the select classifier (`691f4c1be`).** `selects` in `scripts/check-web-audioworklet-v8-spill.py`:
a blend is always a select; an `or` is one unless both inputs are lane masks the loop itself
computed (compares, or bitwise ops over such masks alone). Anything it cannot see through counts as
a select, so a doubtful loop reads as masked and a held row fails closed. Structural rather than a
count of `or`s per SVF step, because a count would also pass a loop where a real select replaced
one of the flush's `or`s. Self-test 19 -> 26 cases (four classifier cases, three masked-row
verdicts). Mutations, each red on the self-test: the old "every `or` is a select" (`flush pair`);
"no `or` is a select" (four cases); "one mask input suffices" (three); an outside register taken as
a mask (`outside mask`); "every logic op makes a mask" (two); a row ignoring `masked` (verdicts).
On the base (`0e3e21b68`) listings every row's verdict is unchanged.

**A3, the dry masks in channel state.** `Channel::dry[s]` holds `dry_mask(s)` as of the last
`refresh_identity`, which every coefficient- or `remaining`-change site already calls; the
stationary cascades (`interleave`, `interleave_mono`) read it instead of building it in place.
Cause, from the V8 listing: the wasm computed section 0's mask once before the loop (a splat, three
`replace_lane`s from scalar decisions and `eq 1.0`), and V8's scheduler sank that pure construction
into the masked mono pair's loop, rebuilding it every frame from four scalar spill slots; with the
joint flush's extra live values that pushed `ic1` through `[rbp-0x220]`. A load from channel state
cannot be sunk. `identity_flags_agree` now re-derives `dry` too (asserted on every stationary block
in debug). `Lane::Mask` gains `Send` so the masks can live in effect state. The other encodings
tried first, none clean: attempt 1's `flush_pair`, compares reordered, `r1 & (f1 | r2)`, chained
`andnot`, per-word `flush` then the rest test on the flushed words, the skewed kernel in forward
section order, the flush after the output mix.

**A2, the dual tail's integer slot: no change to the check.** With A3 in, the per-channel fold
stays in a register: the slot came from the same function's masked loops rebuilding their dry masks
in the loop, not from the check. Folding both channels into one flag (authorized by A6, built and
measured) is worse: with A3 it moves the slot to `[rbp-0xf0]`. A loop with one induction register
(also built) is clean as well but adds a frames-below-depth loop to the function; not taken. So the
check, its semantics and `[bool; S]` API are untouched, and the authorized `g2_kernel_identity.rs`
and `interleave` edits for a joint fold were not needed. **Stability, honestly:** not structural.
The general-purpose slot follows the whole function's register use (the joint fold flips it), so
the gate's dual-tail row holds it. Proven by mutation: the joint-fold build fails that row with
`[rbp-0xf0]`; attempt 1's code (A3 reverted) fails it with `[rbp-0xc8]`.

**The gate holds a fifth row**, `mono depth-2 pair, masked` (A6). Mutation: the mono masked pair
reading `channel.dry_mask(at[k])` again fails it (`[rbp-0x180]`, 113 instructions), every other row
green; attempt 1's code fails it with `[rbp-0x220]`.

**Spill-gate rows** (carried slots; H held):

| loop | base `0e3e21b68` | attempt 1 | attempt 2 |
|---|---:|---:|---:|
| dual depth-1 tail, select-free (H) | 0 | 1 (`[rbp-0xc8]`) | 0 |
| mono depth-2 pair, select-free (H) | 0 | 0 | 0 |
| mono depth-1 tail, select-free (H) | 0 | 0 | 0 |
| mono depth-2 pair, masked (H since attempt 2) | 0 | 1 (`[rbp-0x220]`) | 0 |
| dual depth-1 tail, masked | 0 | 1 | 0 |
| dual depth-2 pair, select-free (reported) | 10 | 13 | 13 |
| dual depth-2 pair, masked | 11 | 12 | 13 |

**The #1000 red arms, rebuilt.** #977 attempt 1 (`f1bf752c`, module `0db9b2f5…`, the same bytes as
#1009's record): red on the dual tail, `[rbp-0xa0]`, as recorded. The one-token tail edit
(`if !admitted && (…)`) applied to this tree: **green**, and green on the base `0e3e21b68` as
well, so it turned green before this slice (not investigated where); the rule is kept, per the
protocol.

**No rendered bit moved (A2, A3), one-time evidence.** With the A3 change in, every pinned corpus
passes unchanged and nothing was re-pinned: `g5_native_digests_match_pins` and the delegated
`BUILTINS_DIGESTS`, `E9_DIGESTS` and multiband `DIGESTS` at every width and on wasm simd128
(`run-wasm-gates.sh`), `conformance_fixtures --check`, `check-builtins-fixtures.sh`,
`check-graph-determinism.sh`; the EQ's own bit-identity gates (elision, interleave identity, mono
collapse, randomized restores, the bounded-verdict-equals-scan check) pass. A2 changed no code.
Mutation of the cache: never refreshing `dry` turns at least twelve EQ unit tests red
(`identity_flags_agree`'s debug assertion, and the dry-select bit-identity tests). Dropping `dry`'s
copy in `desymmetrize` turns no test red; the doc there says so and why.

**A4, D6 restated (scratch harness on the real kernels, not committed).** Per-word flush (attempt-1
kernel with `flush_pair` replaced by two `flush`) against the joint flush, left output bit for bit:
4,864 samples of uniform noise (LCG, peak at the stated level) then 1.2 M samples of silence.
- **Case 1, EQ low shelf 10 Hz, +24 dB, S 0.1, 96 kHz.** At 0 to -200 dBFS no sample moves while
  input is live; in the tail the first moved sample is at -258.6 dBFS and the largest change is
  `1.179e-13` (-258.6 dBFS). At -220, -240 and -260 dBFS every sample moves from sample 1, while
  input is live: both state words sit below `REST_EPS` and are zeroed together. The largest change
  is `1.506e-12` (**-236.4 dBFS**) at -220 dBFS input, `1.506e-13` at -240 and `1.506e-14` at
  -260. *Deviation from A4:* attempt 1 and its verifier saw input-time moves at -160/-180 dBFS with
  their noise; this harness's onset is between -200 and -220 dBFS, and A4's "every change below
  2.2e-13 (< -253 dBFS)" does **not** hold: the largest change measured is `1.5e-12`, -236 dBFS.
  The D3 bell (10 Hz, Q 18, +24 dB) moves while live from -220 dBFS too, largest `4.3e-14`.
- **Case 2, builtin HPF 100 Hz + LPF 22 kHz at 44.1 kHz.** No sample moves while input is live; the
  first moved sample sits in the tail at a loud level for the tail: -159.3 dBFS for 0 dBFS input
  (sample 7,898, 3,034 samples after input stops), -179.3 at -20, -199.3 at -40, -180.5 at -60:
  the HPF rests while the LPF still carries signal. Largest change `2.84e-14` (-270.9 dBFS).
- **Listening line** (corrected in attempt 3): this white-noise harness missed the dead zone's
  peak, and its `1.5e-12` bound was 50 dB short. The verifier measured `4.94e-10` (-186.1 dBFS);
  A7 restates A4 and the Listening line on that number.

**A5.**
- Floors: `tools/bench/src/floor.rs` (`EQ_LANE_OPS` 32, `BUILTINS_LANE_OPS` 79, the strip test),
  `scripts/console-benchmark-record-lib.jq` (79, 32), `scripts/test-console-benchmark.sh` (the
  synthetic records at 79, 322 and `79 + 32 + 81.5`). Each with its reason (#1328's `flush_pair`:
  `svf_step` 19 -> 24, a section 24 -> 29). A2/A3 change no per-sample op count, so the ruling's
  derived 79 / 32 / 322 stand. The ruling's interim "until their follow-up re-pins them" text is
  removed in both places.
- Oracles: `g2_svf_step_yields_both_taps_of_one_state` restates the joint flush from its definition
  and now runs noise then silence, asserting the decay reaches the joint band. Mutation: `svf_step`
  with two per-word `flush` calls is red there (band tap, `g2_kernel_identity.rs:650`), green
  reverted. The audit's `SvfF32` arms restate `flush_pair`; `model_conformance`'s SVF pass appends
  4,096 silent samples and asserts the band is reached. Mutation: the audit's pair without the
  rest arm makes production match NEITHER arm (439 mismatches; the audit panics), green reverted.
- Memset ratchet: `check-cross-targets.sh` asked builtins 194 -> 186 and parametric-eq
  132 -> 122 at this tree (122 rather than A5's 128: the dry masks in state remove six more stored
  splat calls); the rows are lowered to what it reports, and it passes at 186 / 122.
- Stale prose: the ruling's "inert" definition and appendix steps 6-7 (`flush_pair`), the
  spill-gate sentence (now four held rows and one reported row; corrected in attempt 3), the EQ's leg-(c) description, and
  `crates/builtins/tests/MUTATIONS.md` M2. M2 re-run: dropping `s1`'s whole flush is red at
  `index=2645`; dropping only its per-word arm stays green (the row says why).
- NIT: `dsp-research/filters.md` states the rest point as the end of the resting block: samples
  773,888 and 421,888 (blocks 6,045 and 3,295). Attempt 1's 773,760 / 421,760 were those blocks'
  first samples.

**Gates (final tree).** All green, each run once on the committed tree:
- the gate-4 `cargo test` set: 817 passed, 0 failed;
- the release `lane`/`math`/`wasm-gates` set: 107 passed;
- `run-wasm-gates.sh`: native and simd128 legs, and the V8 spill gate with all four held rows clean (the
  rows above);
- `conformance_fixtures --check`, `check-builtins-fixtures.sh`, `check-graph-determinism.sh`;
- `check-cross-targets.sh` (with the lowered rows), `check-lane-policy.sh`, `check-dsp-research.sh`,
  `check-workspace-policy.sh`;
- clippy `-D warnings`, `cargo fmt --check`;
- the worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`;
- also `test-console-benchmark.sh` and `audit unfused-fma conformance` (SVF `unfused`, 0 mismatches).

**Open items.**
- A4's change bound: measured `1.5e-12` (-236 dBFS) at -220 dBFS input, above the spec's 2.2e-13; the
  class-B acceptance needs the root to re-affirm on this number. *(Closed by A7: the true worst
  change is `4.94e-10`, -186 dBFS, through the dead zone; A4 is re-affirmed on that.)*
- The one-token #1000 arm is green at this tree and at base; when it turned green is not
  investigated. *(Closed in attempt 3: green since the #999 merge `27cf24132`, never red on
  `main`; see A7 item 3.)*
- The dual-tail and masked-mono-pair allocations are observed, not structural; the gate rows hold
  them.

### Attempt 3 (2026-10-05, branch `codex/d15-stream-g`; doc + tests, no DSP change)

Host: AMD EPYC 7313P, Node v22.23.2 (V8 12.4.254.21-node.56). Applies A7 items 1-6. No kernel,
coefficient or render code changed: `crates/parametric-eq/src/lib.rs` changes only in two doc
comments, so no rendered bit and no wasm instruction can move, and nothing was re-pinned.

**M1, A4 and the dead zone (A7 item 1).** A4's premises and the DSP-evidence Listening line are
replaced by A7's corrected text ("every change is below −186 dBFS at the section's output; no
listening run"). Attempt 2's white-noise Listening bullet and its open item are marked superseded,
with the verifier's number, rather than deleted. `dsp-research/filters.md`, numerical limits,
gains the dead zone: `L* = REST_EPS/(2·max(a2, a3))`, the direct-term output and the
`‖h − g_direct·δ‖₁·L*` bound, the worst case (`L* = 3.05e-11`, -210.3 dBFS, EQ low shelf 10 Hz
+24 dB 96 kHz; change about `5.0e-10`, -186 dBFS, measured `4.94e-10`), the builtin 10 Hz
(`1.5e-11`) and LR4 80 Hz (`1.9e-12`) cases, the loss of the whole response below `L*` and the HPF
DC note, the -140 dBFS bound, the f32-rounding comparison, and the rejected `x == 0` gate. The
numbers are the attempt-2 verifier's measurements and analysis, not re-measured here.

**m3, the dual-tail slot's cause (A7 item 2).** The `interleave` doc in
`crates/parametric-eq/src/lib.rs` and the spill gate's dual-tail paragraph now say the slot follows
the tail site's own in-place dry masks (which also feed the masked tail loop), with the verifier's
two reverts: tail-only brings `[rbp-0xc8]` back, pair-only leaves the tail clean. The
"observed, not structural" statement stands. Not re-measured here.

**m1, the red arms (A7 item 3).** `tools/wasm-gates/MUTATIONS.md` names #977 attempt 1
(`0db9b2f5`, `[rbp-0xa0]`) and #1328 attempt 1's code (dual tail `[rbp-0xc8]`, masked mono pair
`[rbp-0x220]`) as the red arms, records the one-token arm's bisect (red at `6f4c0379e` and
`d09d50248`, green from `27cf24132`, tail 84 -> 112 instructions, never red on `main`, green at
`a9414c0c6`) and keeps its #1009 listing as history. It also states the four held rows and the
select rule (it still said three select-free rows). The gate script's re-pin paragraph names the
same arms and the bisect. The verifier's builds; not rebuilt here.

**m2, the desymmetrize copies (A7 item 4).** New test
`crates/parametric-eq/tests/mono_collapse.rs::a_desymmetrized_bank_carries_the_collapsed_channels_identity_flags_and_dry_masks`:
every lane switches the dedicated HPF (section 0) on both channels while the bank runs collapsed,
off -> on and on -> off; the ramp ends on the left channel only; `desymmetrize_channels`; then
dual blocks, both planes compared bit for bit with a never-collapsed bank. Mutations (introduce,
run, revert):

| mutant in `desymmetrize` | debug (`cargo test`, the gate profile) | release |
|---|---|---|
| `self.right.identity = self.left.identity;` dropped (stale identity flag) | red: `assertion failed: channels.1.identity_flags_agree()` (`lib.rs:1815`) | green |
| `self.right.dry = self.left.dry;` dropped | red: same assertion | green |
| none | green | green |

`a_desymmetrized_bank_is_a_never_collapsed_bank` stays green on both mutants (its ramp moves a
general band's gain, which changes neither cache). Release stays green, as the test's doc says: a
stale `identity` only chooses a schedule (the crate's gates prove the schedules render the same
bits), and
a stale `dry` runs the HPF wet at the identity words, which can move at most `-0.0` -> `+0.0`; four
HPF cutoffs (30, 500, 2,000, 9,000 Hz) all left the bits equal. *(Corrected in attempt 4, A8
item 6: the stale-`identity` reason is false. The dual elision gate drops a section when both
channels' flags say identity, so a stale right flag beside a fresh left one skips a section that
is live on the right, in release. Release stayed green here only because every block of the input
carries `-0.0`, which refuses elision, and because directly after `desymmetrize` the left
channel's correct flag guards the AND. Attempt 4 adds a left-only phase on a `-0.0`-free input,
where the identity mutant is red in release too.)* The defence is the debug
re-derivation, which is what this test reaches and nothing did before. The `desymmetrize` doc now
names this test as the gate for `identity` and `dry`. **Test value:** a disengage copy that drops
or stales the identity flags or the dry masks turns this test red; no existing test reaches a
stale copy of either.

**NITs (A7 item 5).**
- n1: self-test case `memory operand` (an `or` of a loop-computed mask and a non-stack memory word
  counts as a select). Mutation: `not from_memory` dropped from `selects` ->
  `self-test FAIL memory operand: got [(2, 2, [], True)], want [(2, 2, [], False)]`; reverted, 27
  cases ok. **Test value:** a classifier that sees through a memory operand reads a loaded data
  word as a mask and a masked loop as select-free; no other case has a memory operand on an `or`.
- n2: "five held rows" corrected to four held rows and one reported row (attempt 2 record).
- n3, fixed: new `crates/builtins/tests/stage.rs::scalar_stage_is_the_reference_recurrence_where_the_per_word_flush_fires`
  (T2b). Ordinary signals reach the per-word arm only by coincidence (a scratch sweep of impulse,
  DC, sine and noise over cutoffs and rates found `ic1` hits only for DC through HPF 10 Hz + LPF
  20 kHz at 88.2/96 kHz, and two `ic2` hits in the whole sweep), so T2b constructs its input: a
  1 kHz high-pass takes an impulse, and on silent frames it feeds the `f32` value, searched within
  ±64 ulps of the cancelling input, that leaves one new word non-zero inside the flush band beside a
  partner at or above `REST_EPS`; it asserts each arm fired, at every launch rate, and holds output
  and state to the twin after every one-frame block. Mutations in `crates/dsp-reference/src/tpt.rs`:
  `s1`'s per-word arm dropped (`else { n1 }`) -> T2b red at `rate=44100, index=257`; `s2`'s
  (`else { n2 }`) -> red at `rate=44100, index=276`; T2 green on both; reverted, green. Recorded as
  row M2b in `crates/builtins/tests/MUTATIONS.md`. **Test value:** a reference (or production
  kernel) whose per-word arm is dropped or wrong on either word turns T2b red; T2 cannot see it.

**A7 item 6.** Decision 15 is not edited.

**Gates.**
- `cargo test --locked -p parametric-eq -p builtins --features parametric-eq/test-support,builtins/test-support`:
  every suite ok, 0 failed (`mono_collapse` 4, `stage` 12).
- `check-web-audioworklet-v8-spill.py --check-toolchain` and `--self-test` (27 cases ok), and the
  gate itself on a fresh `build-web-audioworklet.sh --module-only --named-twin` build: ok, the four
  held rows clean (dual tail 123 instructions, mono pair 93, mono tail 60, masked mono pair 103),
  dual pair reported with 13 slots, as in attempt 2.
- `check-dsp-research.sh`, `check-workspace-policy.sh`: ok.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`: ok.
- Not run: the worklet chain (no browser-compiled code changed beyond two comments) and the rest
  of gate 4 (no code under them changed).

**Open items.** None from the attempt-2 verdict. The dual-tail and masked-mono-pair allocations
stay observed, not structural; the gate rows hold them.

### Attempt 4 (2026-10-05, branch `codex/d15-stream-g`; applies A8)

Host: AMD EPYC 7313P (x86-64-v3), Node v22.23.2 (V8 12.4.254.21-node.56), rustc 1.97.1.

**The law (A8 item 1).** `lane::flush_pair(n1, n2, x)`: `rest = (|n1| < REST_EPS) & (|n2| < REST_EPS)
& (x == 0)`, then the per-word law as before. `svf_step` passes its section input `v0`, so every
caller (`svf_block`, the ramped and masked blocks, the interleaved and skewed cascades, the builtin
fused chains, the multiband `lr4_step`) threads its own section's input with no call-site change.
The twin (`ReferenceRetainedTptF32::process`), the g2 oracle and the audit's `unfused_fma` model
restate the gate. 13 lane-ops for the pair (an `eq` and a second `mask_and`), so `svf_step` 26 and a
section 31 (select-free) / 32 (masked).

**Threshold: exactly zero (`x == 0`, IEEE: `+0.0` and `-0.0`), not `|x| < f32::MIN_POSITIVE`.**
Subnormal inputs can reach a section (wasm and the canonical native environment keep IEEE
subnormals; trim, fader and section outputs can underflow), but they need no gate: a subnormal sample
adds at most `2·max(a2, a3)·2^-126` to a state word, far below `FLUSH_EPS`, so the per-word law
already gives it no response; and a silent source reaches every downstream section as an exact
`±0.0` once the upstream sections rest (gates 1-2, the chain measurement below: every tail rests).
The gate also costs one op fewer than `abs` + `lt`.

**EQ predicates (A8 item 5).** `lane_is_inert` keeps its pair term: gate (a) admits `+0.0` input
words, on which an executed identity section zeroes a both-below-`REST_EPS` pair; its doc now says
the kernel can write such a pair under a tiny non-zero input, which then refuses elision (never a
bit) until a zero input word of an executed block zeroes it. `lane_is_flush_shaped` **drops** its
pair term: the kernel now writes such pairs, and the `-0.0` induction leg (c) serves needs only the
per-word shape, so refusing them refused kernel-written states. The proof text names `v0`. Unit tests
follow: `leg_c_refuses_below_flush_eps_admits_it_and_re_engages` admits `±FLUSH_EPS`, `1.5e-20`,
`-1.25e-20` and the words either side of `REST_EPS` again (as before attempt 1), and the elision
seeds return to `(1.5e-20, -1.25e-20)`.

**Section and chain re-measure (A8 item 2; one-time scratch harness on the real EQ, not committed).**
96 kHz, A8 against the per-word law (and attempt 3's law for comparison), left output compared bit
for bit. Inputs: a 3.84 Hz square (half period 12,488) for 96,000 samples then 384,000 of silence,
and a single impulse; amplitudes 1, 1e-3, 1e-6, 1e-9, 3e-11, 1e-11, 3e-12, 1e-12, 1e-14, 1e-15,
1e-17, 1e-20, 1e-25, 1e-30. "Live" is while the input is non-zero.

| +24 dB low shelf | A8 live, max change | A8 tail, max change | attempt-3 law live, max change |
|---|---|---|---|
| 1 section, 10 Hz, S 1 | 0 (every input) | `1.89e-13` (-254.5 dBFS, impulse 1e-6) | `4.88e-10` (-186.2, square 3e-11) |
| 1 section, 10 Hz, S 0.1 | 0 | `1.85e-13` (-254.7, impulse 3e-11) | `2.58e-10` (-191.8, square 3e-11) |
| 1 section, 100 Hz, S 1 | 0 | `1.90e-13` (-254.4, square 1e-3) | `4.86e-11` (-206.3, square 3e-12) |
| 4 sections, 10 Hz, S 1 | 0 | `3.39e-10` (-189.4, impulse 1e-6); square `3.16e-10` (-190.0, at 1e-3) | `1.51e-6` (-116.4, square 3e-11) |
| 4 sections, 100 Hz, S 1 | 0 | `3.41e-10` (-189.4, square 1e-3) | `2.21e-7` (-133.1, square 3e-12) |

- No sample moves while the input is non-zero, in any case, at section or chain level.
- For inputs at or above 1e-9 the A8 and attempt-3 tails are identical: A8 removes only the live
  moves. The chain's tail change is larger than one section's because downstream shelves, still
  decaying, boost the tail an upstream section drops; the first moved sample can sit on a loud tail
  (four shelves at 10 Hz, 0 dBFS square: sample 236,920, output -116.0 dBFS, change `6.7e-11`,
  -203.5 dBFS).
- Sparse input: a lone impulse below `L*` is a tail after its first zero sample, so the joint rule
  ends its response; through four 10 Hz shelves a `3e-11` impulse loses all of it (`1.63e-10`,
  -195.8 dBFS, equal to the per-word run's peak). This is the residual of the rule, not a dead zone
  for continuous signal; recorded in `filters.md`.
- Inputs `1e-17` (10 Hz) / `1e-20` (100 Hz) and below move nothing under either law: the per-word
  law's own dead zone (about `3e-17`, -330 dBFS), unchanged by #1328.
- Gates 1-2: green (rest unchanged: the traps are impulse tails on a zero input).

A4 is restated on these numbers under A8 (above); `dsp-research/filters.md`'s dead-zone paragraph is
replaced by the rest rule, these numbers, the sparse-input residual, the per-word dead zone, the
threshold reason and the superseded rule (3.84 Hz, n1 folded); the law paragraph says 13 ops.
`docs/BUILTINS_AND_METERING_V1.md` states the input gate.

**The chain gate (A8 item 3).** `exact_rest::a_tiny_input_through_four_boosting_shelves_gets_every_boost`
(gate 5). Tolerance chosen from measurement: with the A8 law the scaled tiny runs miss the ordinary
run by 0 (10 Hz, k 30/34/36) and at most `1.3e-6` of the peak (100 Hz, k 36; per-word `FLUSH_EPS`
crossings); at k 38-45 the per-word law's own crossings reach `4e-7` to `1.6e-3`, so those levels are
not used. Mutation: attempt 3's law (`flush_pair` without its input term) -> red in debug and release,
`10 Hz, input 2.7939677e-11 … misses the ordinary run by 1.5091782e3 (ordinary peak 1.5091482e3)`;
gate 2 stays green on it. Reverted: green. **Test value:** a joint flush that ignores the section
input, and so drops a tiny signal's boost in each section of a cascade, is red here; gates 1-2 and
every other EQ test stay green on it.

**Other new or changed tests, with mutations** (each introduced, run, reverted):
- `g4_flush` pair law (gate 3) now takes the input: the edge sweep crosses nine inputs (both zeros,
  the smallest subnormal, `-f32::MIN_POSITIVE`, `1e-30`, `-0.5`, an infinity, a NaN, `3e-11`) and the
  random sweep draws zero inputs half the time; the cases test asserts the small pairs rest on `±0.0`
  and keep the per-word law on seven non-zero inputs. Mutants: no input term -> both pair tests red
  (`flush_pair(9.999999e-15, -9.999999e-15) on input 1e-45 must follow the per-word law`); the gate
  widened to `|x| < f32::MIN_POSITIVE` -> both red at the same subnormal input. **Test value:** a pair
  rule that fires on a non-zero (including subnormal) input is red here.
- `g2_svf_step_yields_both_taps_of_one_state`: a quarter of noise scaled by `1e-13` holds both words
  below `REST_EPS` on a non-zero input (asserted reached); mutant without the input term -> red
  (`g2_kernel_identity.rs:677`). **Test value:** an `svf_step` whose flush ignores its input is red in
  the lane crate's own identity gate, independent of the EQ.
- Predicates: `lane_is_inert` pair term dropped -> `a_non_inert_state_in_a_dead_section_refuses_elision`,
  the randomized restore differential and `ramping_elision::a_ramping_list_matches_the_full_section_path_scalar`
  red (the generator still reaches the band under A8). `lane_is_flush_shaped` pair term re-added ->
  `leg_c_refuses_below_flush_eps_admits_it_and_re_engages` and `an_elided_cascade_is_the_full_cascade_bit_for_bit` red.

**M2 (A8 item 6).** The test doc of
`mono_collapse::a_desymmetrized_bank_carries_the_collapsed_channels_identity_flags_and_dry_masks` and
the attempt-3 record are corrected (a stale `identity` can skip a live section in release; release
stayed green only because every block carried `-0.0`). The test gains blocks 24-47 on a `-0.0`-free
input (`clean_block`) with a left-only HPF retarget in block 26:

| mutant in `desymmetrize` | debug | release |
|---|---|---|
| `self.right.identity = self.left.identity;` dropped | red (`identity_flags_agree`, `lib.rs:1818`) | **red**: `HPF false -> true, block 27 word 0: right plane` |
| `self.right.dry = self.left.dry;` dropped | red (same assertion) | green (a stale dry lane moves no bit here) |
| none | green | green |

Rows 1328-M1..M3 in `crates/parametric-eq/tests/MUTATIONS.md` (n4).

**NITs (A8 item 7).** `filters.md` now says 3.84 Hz (the paragraph was rewritten). The dual tail's
"V8 sank their construction into it" is qualified as an inference from the two reverts, not a
measured listing, in `interleave`'s doc and the spill gate's docstring (the masked mono pair's
sinking, in `Channel::dry`'s doc, was read from a listing in attempt 2 and stays). Decision 15's
"Root decisions after S0" entry now points at A8 (item 8).

**V8 spill gate (A8 item 4): held rows clean; the reported row carries more slots.**

| loop | attempt 3 | attempt 4 |
|---|---:|---|
| dual depth-1 tail, select-free (H) | 0 (123 instr.) | 0 (132 instr.) |
| mono depth-2 pair, select-free (H) | 0 (93) | 0 (99) |
| mono depth-1 tail, select-free (H) | 0 (60) | 0 (63) |
| mono depth-2 pair, masked (H) | 0 (103) | 0 (109) |
| dual depth-2 pair, select-free (reported, not held) | 13 slots | **17 slots** (240 instr.) |

`run-wasm-gates.sh` exits 0. The dual depth-2 pair was already starved (10 slots at base, 13 since
attempt 1) and the gate reports it by design; the input compare adds four. **Measured cost**
(descriptive; `web-mixing-automation-benchmark.mjs run` on two `--module-only` builds of this tree,
A8 and A8 with the input term removed (attempt 3's law), one warmup and two measured rounds each,
interleaved, pinned to cpu 31, **uncontrolled host**: load average 12-13 on 32 threads; not retried):

| p50, ns per render (round 1 / round 2) | attempt-3 law | A8 | change |
|---|---|---|---|
| mono console, quiet | 158,392 / 158,723 | 161,859 / 159,764 | +2.2 % / +0.7 % |
| mono console, restated | 161,327 / 161,338 | 162,991 / 162,650 | +1.0 % / +0.8 % |
| mono console, automated | 165,135 / 165,235 | 166,597 / 166,527 | +0.9 % / +0.8 % |
| sixty-four-track console | 242,943 / 241,981 | 248,925 / 248,964 | +2.5 % / +2.9 % |
| app shape | 157,440 / 156,628 | 162,549 / 162,600 | +3.2 % / +3.8 % |
| bus-and-send console | 347,993 / 345,658 | 354,816 / 353,414 | +2.0 % / +2.2 % |

Every output digest is equal between the two modules for these documents (their inputs never sit in
the band). The three documents exceed the owner's 2 % allowance by up to 1.8 points on this host;
per A8 item 4 this is reported, and **no workaround was attempted** (the floors record +2 ops per
section, 29 -> 31). Open for root.

**Floors and op counts.** `docs/rulings/effect-floor-accounting.md`: `flush_pair` 13, `svf_step` 26,
section 31 / 32, EQ standing 34 (1.149 cycles), the `active` table `31·active + 3` (195 for six),
builtins sections 31 and chain 83 (2.804), `builtins_only − gain_pan_only` 83 − 22 = 61, strip 328
(11.081), appendix step 6 with `v0`. `tools/bench/src/floor.rs` (`EQ_LANE_OPS` 34,
`BUILTINS_LANE_OPS` 83, the strip test), `scripts/console-benchmark-record-lib.jq` (83, 34),
`scripts/test-console-benchmark.sh` (83, 61, 328, `83 + 34 + 81.5`): reason, A8's input gate adds
two lane-ops per SVF section. `check-cross-targets.sh` asked for no ratchet change.

**Bits moved and re-pins.** None. `g5_native_digests_match_pins` and every delegated pin
(`BUILTINS_DIGESTS`, `E9_DIGESTS`, multiband `DIGESTS`), `conformance_fixtures --check`,
`check-builtins-fixtures.sh` and `check-graph-determinism.sh` pass unchanged: their only joint-flush
moves are tails on zero input, which A8 keeps. Against attempt 3 the law moves bits only where a
section's words are both below `REST_EPS` on a non-zero input, which no pinned corpus reaches.

**Gates (this tree).**
- gate-4 `cargo test --all-targets` set (14 packages, spec features, `--no-fail-fast`): 836 passed,
  0 failed;
- `cargo test --release -p lane -p math -p wasm-gates --features math/lane`: all ok;
- release `parametric-eq` `exact_rest` and `mono_collapse`: ok;
- `run-wasm-gates.sh`: exit 0 (native, simd128, V8 spill gate with the rows above; self-test 27 ok);
- `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files), `audit unfused-fma
  conformance` (SVF `unfused`, 0 mismatches), `check-graph-determinism.sh` (100/100),
  `check-cross-targets.sh`, `check-lane-policy.sh`, `check-dsp-research.sh`,
  `check-workspace-policy.sh`: ok;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`: ok;
- `cargo test -p bench floor`, `test-console-benchmark.sh`: ok;
- worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`: ok.

**Open items.**
- The browser cost above (+2.0 % to +3.8 % p50 on the three console documents, uncontrolled host)
  exceeds the 2 % allowance; root decides (accept, a controlled re-measure, or an encoding issue).
- Sparse tiny input (non-zero samples separated by exact zeros, below `L*`) loses its tail through
  the joint rule, up to `1.6e-10` (-196 dBFS) at four shelves; recorded, not gated.
- The builtin HPF/LPF and the LR4 crossover were not re-measured at chain level (A8 asked for the
  shelves); their tails follow the same rule.

### Attempt 5 (2026-10-05, branch `codex/d15-stream-g`; applies A9; the last attempt)

Host: AMD EPYC 7313P (x86-64-v3), Node v22.23.2 (V8 12.4.254.21-node.56), rustc 1.97.1.

**The law (A9 items 1, 5).** `lane::silence_step(x, run, armed_after)` advances one counter word per
lane, `run = (x == 0) ? run + 1 : 0` (an `f32` add: exact to `2^24`, where it saturates by itself),
and returns the frame's rest threshold, `REST_EPS` once `run >= N_SILENCE` and `+0.0` otherwise
(5 lane-ops: `eq`, `add`, `select`, `lt`, `andnot`). `lane::flush_pair(n1, n2, rest)` is the
per-word law plus `joint = max_u32(|n1|, |n2|) < rest` (10 lane-ops, against 13 under A8 and 6 for
two `flush`). `Lane::max_u32` is new: `vpmaxud` / `i32x4.max_u` / `umax`, implemented through the
bit pattern at every width (`LANE-OPS-OK` reviewed); the unsigned maximum of two magnitudes' bits is
the larger magnitude bit for bit and every NaN magnitude sorts above `+inf`, so a NaN in either word
fails the compare. `svf_step(v0, nc1, a2, a3, rest, s)` takes the threshold; every caller passes the
threshold of its **effect input**, never its section input.

**Where the counters live (A9 item 7).**
- Builtin input stage: `InputChainState::silence: [L; 2]` (`crates/lane/src/kernels/builtins.rs`),
  advanced by every body from the sample it loads (before sanitising and trim), the all-identity
  bodies included (`silence_skip_block`); `InputChainCoef::silence` holds `N_SILENCE` at the
  stage's rate (`PreparedInputTrack::silence_frames`).
- Parametric EQ: `Channel::silence` per channel and two heap rest planes per prepared EQ
  (`quantum * W` words each, allocated at preparation) plus one shared all-`+0.0` plane:
  `silence_block` writes a channel's plane from the EQ's input before the first section runs, only
  on a block in which some lane can arm (`silence_armable`, exact: one compare and one `mask_any`
  per block); every other block reads the zero plane and advances the counter with
  `silence_skip_block` (one load and one `mask_any` when the block's last frame is non-zero on every
  lane). The silent fixed point advances the counter without a frame loop (`silence_advance`) and
  is earned only when every lane is armed throughout the block. The cascades load one threshold per
  stream-frame (skewed cascades load frame `i - k`).
- Multiband compressor: `Side::silence` per channel; `run_segment` steps both counters per frame
  and both `lr4_step` stages read that frame's threshold.
- The twin (`dsp_reference::ReferenceSilenceRun`, `reference_silence_frames`), the g2 oracle and the
  audit's `unfused_fma` model restate the counter independently.

**Two forms per builtin chain body; carried constants (`check-cross-targets.sh`).** Each builtin
input-chain kernel tests once per block whether some lane's counter can arm (`silence_armable`) and
runs its body in one of two forms through `lane::kernels::svf_step_when(armable, flush_eps, ...)`:
the armed form (counter per frame, joint flush) or, on every other block -- every block of live
audio -- the unarmed form (per-word flush, no counter in the loop; the counter advances once per
block through `silence_skip_block`). The two forms are the same bits (`flush_pair(n1, n2, +0.0)` is
`(flush(n1), flush(n2))`). The first cut of this split doubled the bodies' vector constants, and on
`aarch64-apple-ios` every splatted constant is a `memset_pattern16` call (#1018): the builtins'
count rose from 186 to 397 and the multiband compressor's from 566 to 1128 against those ceilings.
A run-time flag instead of a const generic changed nothing (LLVM unswitches the loop: 397). The
builtin chain bodies now load their four constants (`NONFINITE_LIMIT`, `1.0`, `FLUSH_EPS`,
`REST_EPS`) from `InputChainCoef::constants` (`InputChainConstants`, the precedent being the #1407
filter-ramp countdown carried as a word), and `lane::flush_with` / `flush_pair_with` take
`FLUSH_EPS` as a word: the builtins' count falls to 71 (ceiling lowered from 186, as the check
asks). A one-body alternative (counter and joint flush on every frame) gave 182 but cost about
3 points more p50 than the two forms on the 64-track documents (measured with one warmup and two
rounds against A8: app shape +12.9 % / +10.6 %, console +7.6 % / +5.4 %), so it was not kept. The
multiband compressor runs the counter and the joint flush on every frame in one body: its dynamics
carry dozens of constants, a second form would double its count (1128), and the multiband is not
in the browser documents measured here. The EQ keeps its zero-plane path (88, ceiling lowered from
122). `lane::silence_step_hoisted` takes the counter's `1.0` and `REST_EPS` from the caller.

**Section input term (A9 item 1): dropped.** It can only matter on a frame whose effect input has
been silent for `N_SILENCE` samples, where a later section's non-zero input is an upstream tail.
Measured at 1,024 samples, with and without it ANDed in, on every EQ case of the search below
(every rate and configuration): the largest change against the per-word law and its position were
the same. Dropping it saves two lane-ops per section.

**`N_SILENCE` (A9 item 2): a time, `4096 / 48000` s.** `lane::silence_frames(rate)` =
`ceil(rate * 4096 / 48000)`: 3,764 / 4,096 / 7,527 / 8,192 samples at 44.1 / 48 / 88.2 / 96 kHz.
One-time scratch search on the real EQ (not committed), A9 against the per-word law, left output bit
for bit: four +24 dB low shelves (10 Hz and 100 Hz, S 1) and one (10 Hz at S 1 and S 0.1, 100 Hz at
S 1); continuous inputs (the 3.84 Hz-at-96-kHz square, DC, an impulse) then silence, a ternary
random input, and sparse impulse trains with gaps on both sides of the window, at levels from 1 down
to `1e-14` and fractions of `L*`. Changes of relative size below `1e-4` on a live sample (rounding-
scale flips, up to `4.66e-10` at a relative `1e-7`) are classified apart.

| largest change found | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---:|---:|---:|---:|
| 1,024 samples, four 10 Hz shelves | `1.10e-9` | `1.18e-9` | `1.31e-9` | `1.33e-9` |
| 1,024 samples, one 10 Hz shelf, S 1 | `4.2e-13` | | | `4.6e-13` |
| 1,024 samples, one 10 Hz shelf, S 0.1 | up to `6.6e-13` | | | |
| 42.7 ms (4,096 samples at 96 kHz) | `4.19e-10` | `3.85e-10` | | |
| 85 1/3 ms, four 10 Hz shelves | `3.414e-10` | `3.411e-10` | `3.405e-10` | `3.471e-10` |
| 85 1/3 ms, four 100 Hz shelves | `3.40e-10` | | | |
| 85 1/3 ms, one 10 Hz shelf, S 1 | `1.904e-13` | `1.904e-13` | `1.903e-13` | `1.947e-13` |
| 85 1/3 ms, one 10 Hz shelf, S 0.1 | `1.52e-13` | `1.37e-13` | `1.22e-13` | `1.39e-13` |
| 85 1/3 ms, one 100 Hz shelf, S 1 | `1.90e-13` | | | |

At 1,024 samples a train whose gaps just exceed the window ends a tail in every gap, and the lost
tails add up beyond one tail's worth (A8's tail level was `3.4e-10` at four shelves); at 85 1/3 ms
every case is back to one tail's worth. Blank cells were not searched separately at that size.

**Reachable bounds (m2; analytic, `f64` from the cast words, ignoring `f32` rounding).** The rule
zeroes a pair only when both words are below `REST_EPS` and the effect input is silent, so the
output change is a sum of zero-input responses to injected states of at most `REST_EPS` per word.
- One tail of one section (it flushes once): `REST_EPS * max_m ||C A^m||_1` = `1.953e-13`
  (-254.2 dBFS) for a +24 dB low shelf at 10 Hz or 100 Hz, S 1, and `3.37e-13` at S 0.1, at every
  rate. Attempt 4's measured `2.41e-13` at S 0.1 sits under it.
- One tail through a chain (the first section flushes once; a later one may flush repeatedly while
  an upstream tail feeds it): `REST_EPS * (max_m ||G_1(m)||_1 + sum_{k>=2} ||G_k||_1)`: `3.1e-7`
  (-130.2 dBFS) at four 10 Hz shelves and `3.1e-8` at four 100 Hz shelves at 96 kHz (`1.43e-7` and
  `1.46e-8` at 44.1 kHz). The first section's share alone is `3.479e-10`, which is what the search
  reaches.
- Any input, repeated tails included: `REST_EPS * sum_k ||G_k||_1`, `5.0e-6` at four 10 Hz shelves,
  96 kHz.
- Builtin chain (one tail): HPF 10 Hz into the LPF at the cutoff maximum `3.6e-14`, into a 12 Hz LPF
  `3.4e-11`. Largest found through the bit-identical twin at every rate: `2.8e-14`, except one
  rounding-scale `2.27e-13` on a live sample at 48 kHz (LPF maximum). LR4 (`lr4_step`): 80 Hz at most
  `2.84e-14`, 8 kHz nothing.

**Subnormals (A9 item 4): no per-word `f32::MIN_POSITIVE` flush is needed.** The per-word law runs
on every state word on every sample whatever the counter says, so each word is `+0.0` or at least
`FLUSH_EPS = 1e-20` in magnitude, `2^60` above the subnormal range, on wasm (no FTZ) as on native.
The window delays only the joint rule. A subnormal input sample is live to the counter (`x == 0` is
false) and gives no response through the per-word law (it adds at most `2 * max(a2, a3) * 2^-126`
to a word). G4/G6 hold.

**Gates 1-2 (A9 item 3).** Green. Gate 1 (input LPF at the cutoff maximum, 44.1 kHz) rests in block
6,045 (sample 773,888 at the block's end), gate 2 (EQ shelf, 96 kHz) in block 3,295 (421,888); both
inside their bounds (2,400,000 and 9,600,000).

**New gates (A9 item 3)** in `crates/parametric-eq/tests/exact_rest.rs`:
- `a_sparse_input_through_four_boosting_shelves_gets_every_boost_at_every_rate` (gate 6):
  `[a, 0, a, 0, ...]` and `[a, 0, 0, 0, ...]`, `a` the 3.84 Hz square at `0.03 * 2^-36`
  (`4.4e-13`, below `L*` at both frequencies and every rate), after a lead of armed silence, through
  four +24 dB shelves at 10 Hz and 100 Hz, at 44.1, 48, 88.2 and 96 kHz: the tiny run scaled back by
  `2^36` misses the ordinary run by exactly 0 in every case (tolerance `1e-6` of the peak).
  A8's law: red (`… misses the ordinary run by 1.0375549e3`); attempt 3's: red.
- `an_exact_zero_inside_the_cascade_moves_nothing_while_the_input_is_live` (gate 7): the EQ's 40 Hz
  high-pass ahead of four +24 dB shelves at 100 Hz, DC and slow-square inputs, tiny run `2^-20`
  scaled back: within `1e-9` of the peak on every sample. A8's law: red (`DC at 1e-6 … misses the
  ordinary run by 6.996817e-5 … (peak 7.826055e3)`).
- `a_fixed_point_inside_the_joint_band_is_cleared_once_the_flush_arms`: every band disabled and
  band one's integrators restored to `(1e-15, -1e-15)`, a pair an identity section holds unchanged
  on a zero input with an output of exactly `+0.0`; the EQ's silent fast path must not be earned
  before the counter arms (8,192 frames at 96 kHz), and the joint flush must then clear the pair and
  the counter advance on skipped blocks.

**State and sizes (A9 item 7), each with its reason (one counter word per channel per effect
input).**
- EQ state payload: one `f32` word per channel per lane (`STATE_SILENCE_WORD`), 936 → 944 bytes for
  the standing payload (`LANE_BYTES` 468 per lane in the test support); decode accepts `+0.0` or an
  integer in `1..=2^24` and refuses anything else (`a_malformed_silence_word_is_refused`).
- Multiband state payload: the lane header grows by one word per side (`LANE_HEADER_WORDS` 48,
  `SILENCE_WORD = FILTER_WORD + 4`), 188 → 192 bytes per channel, 376 → 384 per lane; same validation
  (`the_silence_counter_is_carried_and_validated`).
- Builtins: `InputChannelState::silence` rides the carry export/import and the mono collapse's
  disengage copy; `channels_agree` compares the counters. Prepared sizes: `InputBuiltins` 688 → 712
  (+8 the two counters, +16 the four carried constants; the `N_SILENCE` word lands in padding), the
  boxed input entry 704 → 728, the strip preparation 1,072 → 1,096 bytes
  (`tools/audit/src/fixture_builtins.rs`); `PreparedInputTrack::silence_frames`,
  `InputChainCoef::silence` and `InputChainCoef::constants`.
- Resource fixtures, each regenerated by its own generator: `fixtures/builtins/v1/resources.jsonl`
  (+24 bytes per input stage: processor and retained payload 1,957 → 2,005 per two-track row,
  largest allocation 1,072 → 1,096; manifest re-pinned), `fixtures/graph/v1/direct-route.resources.json`
  (builtin bank bytes 6,503 → 6,727, +224: the per-channel counters and the four eight-lane
  constant words; graph `MANIFEST.tsv` re-pinned), `crates/graph-compiler/tests/track_delay.rs`
  (digest re-pinned with its derivation: +72 bytes, 8,424 → 8,496 and 150,559 → 150,631),
  `tools/audit/src/builtins_graph.rs` and `fixture_builtins.rs` `ACCEPTED_MANIFEST_SHA256` (the
  joined manifest).

**Counter semantics across swap, restore and reset.** Carried: the EQ and multiband payloads, the
builtins carry and the mono-collapse disengage copies move the counter with the integrators, so a
plan swap or restore mid-silence arms on the same frame the continuing lane would
(`carry::a_restore_mid_silence_arms_the_joint_flush_on_the_continuing_lanes_frame`,
`exact_rest::a_carried_lane_arms_its_joint_flush_on_the_frame_the_exported_lane_would`). Reset to
`+0.0` by a discontinuity reset and at preparation (safe: the rule then waits a full `N_SILENCE`).
Frozen while an EQ or multiband instance is bypassed at preparation (no crossover runs, as the
filter state is frozen). The D7 recovery path (a non-finite lane block) zeroes the sections and
leaves the counter, which counts the input, not the state.

**V8 spill gate (A9 item 6).** `run-wasm-gates.sh` exits 0; the four held rows are clean.

| loop (V8, simd128) | A8 | A9 |
|---|---:|---:|
| mono depth-1 tail, select-free (H) | 0 (63 instr.) | 0 (58) |
| mono depth-2 pair, select-free (H) | 0 (99) | 0 (97) |
| mono depth-2 pair, masked (H) | 0 (109) | 0 (101) |
| dual depth-1 tail, select-free (H) | 0 (132) | 0 (127) |
| dual depth-2 pair (reported, not held) | 17 slots (240) | no select-free loop of its shape: 15 slots (226 instr.) in the listing, read as masked |

The dual depth-2 pair now spills its select-free compare mask, so the classifier reads it as
masked and the reported row finds no select-free loop. The gate's tail anchor is corrected to find a held tail by its shape (steps and streams)
whatever the select classification of the loop beside it (`scripts/check-web-audioworklet-v8-spill.py`,
docstring and a new self-test case; the A1 select rule is unchanged). Mutation: anchoring on
select-free loops only turns the self-test red (exit 1).

**Codegen per SVF step.** AVX2 (`svf_block`, one frame): the flush is 10 instructions (`vandps` x2,
`vpmaxud`, three `vcmpltps`, one from memory for the threshold, `vorps` x2, `vandnps` x2) beside 7
`vmulps`, 10 `vaddps` and 1 `vsubps`; `silence_block` is 6 per frame (`vcmpeqps`, `vaddps`,
`vpcmpgtd` + `vpand` for the select, `vcmpltps`, `vandnps`). V8: `silence_block`'s loop is 5 ops per
frame (unrolled twice); the held loops are in the table above.

**Measured cost (A9 item 6; descriptive).** `web-mixing-automation-benchmark.mjs run` on the shipped
module of this tree (`target/ci/qualification-artifacts`) against A8 (the tree before this attempt),
one warmup and two measured rounds each, interleaved, pinned to cpu 31, **uncontrolled host** (load
average 2-3 on 32 threads); not retried. Every output digest is equal between the two modules for
these documents (their inputs never sit silent for `N_SILENCE`).

| p50, ns per render (round 1 / round 2) | A8 | A9 | change |
|---|---|---|---|
| mono console, quiet | 150,075 / 150,847 | 155,044 / 154,894 | +3.3 % / +2.7 % |
| mono console, restated | 153,372 / 153,351 | 158,501 / 158,892 | +3.3 % / +3.6 % |
| mono console, automated | 157,348 / 157,359 | 162,298 / 162,168 | +3.1 % / +3.1 % |
| sixty-four-track console | 234,255 / 234,746 | 239,865 / 243,563 | +2.4 % / +3.8 % |
| app shape | 146,198 / 146,338 | 154,624 / 158,852 | +5.8 % / +8.6 % |
| bus-and-send console | 337,250 / 338,783 | 349,734 / 353,251 | +3.7 % / +4.3 % |

The change is above the owner's 2 % allowance on every document; it is reported, not attributed
further (no per-part timing was made) and not worked around further (A9 item 6). For scale, the
one-body builtin chain (counter and joint flush on every frame, no unarmed form) measured +5.4 % to
+12.9 % on the same documents and host, and the two-form chain with splatted constants (the form
the iOS ratchet refused) about +1.6 % to +4.5 %.

**Floors (A9 item 8).** `docs/rulings/effect-floor-accounting.md`: `flush_pair` 10, `silence_step` 5,
`max_u32` 1, `svf_step` 23, EQ section 28 / 29, standing EQ 31 (1.047 cycles), the `active` table
`28 * active + 3` (3 / 31 / 59 / 87 / 115 / 143, 177 for six), builtins sections 28 and the counter
5 on a block that can arm (82), and on live audio the per-word flush as before #1328: chain 69
(2.331), `builtins_only - gain_pan_only` 69 - 22 = 47, strip 311 (10.507).
`tools/bench/src/floor.rs` (`EQ_LANE_OPS` 31, `BUILTINS_LANE_OPS` 69, the strip test),
`scripts/console-benchmark-record-lib.jq` (69, 31), `scripts/test-console-benchmark.sh` (69, 47,
311, `69 + 31 + 81.5`).

**Bits moved and re-pins (A9 item 8), individually.**
- G5 (`tools/wasm-gate-corpus/src/lane_digests.in`): six cases move back to their pins before #1328
  (byte-identical to the file at `1975fc44f`): `svf_block/low/impulse`, `svf_block/high/impulse`,
  `svf_block/band/impulse`, `svf_block/bell/impulse`, `svf_block_ramped/impulse`,
  `svf_block_ramped/idle/impulse`. Reason: their impulse tails are shorter than `N_SILENCE`, so the
  joint rule no longer acts and the per-word bits return. Every other G5 pin and every delegated pin
  (`BUILTINS_DIGESTS`, `E9_DIGESTS`, multiband `DIGESTS`) is unchanged.
- `fixtures/builtins/v1/reference/filter-response.csv`: `impulse_dft_magnitude_db` only, 565 of
  1,630 rows, at most `3.2e-5` dB at about -187 dB (the joint rule now ends the impulse tails after
  `N_SILENCE`); `MANIFEST.tsv` re-pinned. Generated by the audit's own generator.
- The resource fixtures and digests listed under "State and sizes" (sizes only, no rendered bit).
- `conformance_fixtures --check`, `check-builtins-fixtures.sh` and `check-graph-determinism.sh` pass.

**Folds.** m2: the tails above are stated as analytic bounds and the search's largest found, not
sampled maxima. n1: the audit's `unfused_fma` model runs the counter, and a slow 10 Hz silence pass
("F1/F2 svf_block silence") reaches an armed frame; dropping the model's silence term turns the
conformance run red (`F1/F2 svf_block … NEITHER`, exit 134). n3: documented, not changed:
`lane_is_inert`'s pair term refuses elision of a dead section holding a both-below-`REST_EPS` pair
until an executed block reaches an armed frame; under input that is never silent for `N_SILENCE`
that refusal lasts as long as the input does. It costs only the elision, never a bit; admitting the
pair on unarmed blocks would need the rest plane before every stationary elision decision for a
state only a band switched off at a tiny level, or a restore, holds.

**Docs (A9 item 9).** A4/A7/A8 here, `dsp-research/filters.md` (rest rule, bounds, adopted law),
`docs/BUILTINS_AND_METERING_V1.md`, the `crates/lane/src/lib.rs` docs and decision 15's #1328 entry
are amended to A9.

**Mutations** (each introduced, run, reverted; `scratchpad/mut/run.sh`):

| id | mutant (final code unless noted) | red | verdict |
|---|---|---|---|
| M1 | attempt 3's law: every threshold armed, `silence_armable` always true | builtins `exact_rest` carry test, twin (`stage`), `g2_svf_step_yields_both_taps_of_one_state`, multiband arming test, EQ fixed point, tiny, sparse and cancellation gates | RED |
| M2 | A8's law: armed by the section input `v0 == 0` (in `svf_step_when`), `silence_armable` always true | builtins carry, twin, g2, multiband arming, EQ fixed point; sparse gate (`… misses the ordinary run by 1.0375549e3 … (ordinary peak 1.102915e3)`) and cancellation gate (`DC at 1e-6 … misses … by 6.996817e-5 … (peak 7.826055e3 …)`); the tiny gate (A8 item 3) stays green | RED |
| M3 | counter never resets on a live sample | `g4_silence_counter_law_at_every_width`, twin, `input_chain_elision` (4) | RED |
| M4 | arms one frame late (`le`) | g4 counter law, builtins carry, twin, multiband arming | RED |
| M5 | `silence_armable` one frame late (`gt`) | `g4_silence_armable_is_exact`, builtins carry, EQ fixed point | RED |
| M6 | `silence_skip_block` fast path inverted | builtins `exact_rest` (3), `input_chain_elision` (4) | RED |
| M7 | `silence_skip_block` never resets | builtins carry, `input_chain_elision` (4) | RED |
| M8 | builtins carry import drops the counter (attempt-5 run before the final restructure; code unchanged) | builtins carry test | RED |
| M9 | builtins disengage copy drops the counter (same) | `the_disengage_copy_and_the_agreement_proof_carry_the_silence_counter` | RED |
| M10 | `channels_agree` ignores the counter (same) | the same test | RED |
| M11 | EQ commit drops the counter (same) | `carry::a_malformed_silence_word_is_refused`, `carry::a_restore_mid_silence_arms_…` | RED |
| M12 | EQ desymmetrize drops the counter (same) | `mono_collapse::a_desymmetrized_bank_carries_the_collapsed_channels_silence_counter` | RED |
| M13 | multiband commit drops the counter (same) | `the_silence_counter_is_carried_and_validated` | RED |
| M14 | EQ silent fast path earned unarmed (same) | `exact_rest::a_fixed_point_inside_the_joint_band_is_cleared_once_the_flush_arms` | RED |
| M14b | EQ skipped block does not advance the counter (same) | the same test | RED |
| M15 | audit model always armed / armed on `v0 == 0` / never armed (n1) | `audit unfused-fma conformance`: `F1/F2 svf_block … NEITHER` (exit 134) for the first two; the reach assertion (`armed (0)`) for the third | RED |
| M16 | twin always armed (same as M8) | builtins `stage` twin | RED |
| M17 | `flush_pair` joint test with `Lane::max` (same) | `g4_pair_law_cases_at_every_width`, `g4_pair_law_holds_at_every_width` | RED |
| M18 | `max_u32` implemented as the float `max` (same) | `g1_max_u32_is_the_unsigned_maximum_of_the_bits` and the two g1 sweeps | RED |
| M19 | `lane_is_inert` pair term dropped (same) | EQ `elision::a_non_inert_state_in_a_dead_section_refuses_elision`, fixed point | RED |
| M20 | EQ unarmed block reads the last armed plane (same) | sparse gate | RED |
| M21 | spill gate anchors the held tail on select-free loops only (same) | `check-web-audioworklet-v8-spill.py --self-test` (exit 1) | RED |
| M22 | builtins `rest_eps` constant `+0.0` (never armed) | builtins `stage` twin | RED |
| M23 | multiband: the right channel counts the left input, and the reverse | `each_channel_counts_its_own_input` | RED |
| M24 | builtins counter steps from the sanitised, trimmed sample | `input_chain_elision::mixed_elision_matches_frozen_bodies_*` (3) | RED |
| M25 | multiband right crossover never armed | `the_crossover_joint_flush_arms_after_its_inputs_silence` | RED |
| M26 | multiband second LR4 stage never armed | the same test | RED |
| M27 | multiband arms one frame early | the same test (its `N_SILENCE - 2` case) | RED |
| M28 | builtins unarmed form does not advance the counter | `input_chain_elision::mixed_elision_matches_frozen_bodies_*` (3) | RED |
| M29 | builtins carried `flush_eps` word `1e-21` | `stage::scalar_stage_is_the_reference_recurrence_where_the_per_word_flush_fires`, twin | RED |
| M30 | builtins unarmed form runs the joint flush with its `+0.0` threshold | none: equivalent by construction (`flush_pair(n1, n2, +0.0)` is the per-word law); the split is a cost saving only | equivalent |

Every new or rewritten test answers the test-value question: the counter law, `silence_armable`,
`silence_advance` and `silence_frames` (g4) catch a counter that does not reset, arms early or late
or saturates wrongly; `max_u32` (g1) catches a float-`max` implementation; the sparse and
cancellation gates catch A8's law and a stale rest plane; the carry, disengage, restore, mono
collapse and multiband payload tests catch a dropped or mixed-up counter; the multiband arming test
catches a crossover that never arms or arms early; the fixed-point test catches a silent fast path
earned before the rule arms.

**Gates (this tree).**

- gate-4 `cargo test --all-targets` set (14 packages, spec features, `--no-fail-fast`): 853 passed,
  0 failed;
- `cargo test -p audit -p graph-compiler -p builtins-compiler -p graph -p host-core -p host-web`: ok;
- `cargo test --release -p lane -p math -p wasm-gates --features math/lane`: ok (G5 green with the
  six pins above); release `parametric-eq` `exact_rest` and `mono_collapse`: ok;
- `run-wasm-gates.sh`: exit 0 (native, simd128, V8 spill gate with the rows above; self-test 28 ok);
- `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files), `graph_fixture --check`,
  `audit unfused-fma conformance` (SVF `unfused`, 0 mismatches, the silence pass included),
  `check-graph-determinism.sh` (100/100), `check-cross-targets.sh` (builtins 71, parametric-eq 88,
  multiband 566 `memset_pattern16` calls), `check-lane-policy.sh`, `check-dsp-research.sh`,
  `check-workspace-policy.sh`: ok;
- `cargo clippy --locked --workspace --all-targets -- -D warnings` (and with the test-support
  features), `cargo fmt --all -- --check`: ok;
- `cargo test -p bench floor`, `test-console-benchmark.sh`: ok;
- worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`: ok.

**Open items.**
- Browser cost above the owner's 2 % allowance on every measured document (see the p50 table); root
  decides.
- The multiband compressor pays the counter and the joint flush on every frame (one body, #1018).
- The dual depth-2 pair (reported, not held) reads masked because its compare mask spills: 15
  slots, two fewer than A8.
- n3 is documented, not fixed (above).

### Follow-up (2026-10-06, branch `codex/d15-stream-g`; root cost rulings after the attempt-5 PASS)

Host: AMD EPYC 7313P (x86-64-v3), Node v22.23.2 (V8 12.4.254.21-node.56), rustc 1.97.1. Rulings:
the attempt-5 verdict's m1-m7 and two root rulings (cost; then the four items below). Commits:
`4264bb190` (checkpoint: EQ unarmed form, arming only lanes that hold state, m3, m4, m6),
`6293440ff` (whole-block zero scan), `8aa7c5ab5` (spill-gate ceilings), `5b887edb9` (`track_delay`
derivation), `37d8583f8` (floors, m5, m7, docs) and the commit that adds this record.

**What changed, and why (bits unchanged throughout).**
- **(a) The EQ has an unarmed form.** Its cascades run in two forms per block, as the builtin
  chain's bodies do: the armed form (rest-plane threshold load and `flush_pair`'s joint term) and
  the unarmed form (two per-word `flush`, no plane read; `lane::kernels::RestThresholds`,
  `ArmedRest`, `UnarmedRest`). `FLUSH_EPS` is carried as a word from the prepared EQ, so neither
  form splats it. Live audio no longer pays `flush_pair`'s joint term or the threshold load.
- **(b) A block arms only when a lane that can arm holds state.** `lane::silence_armable_holding`
  (exact; proof in its doc): a lane whose state is `±0.0` at block start sees only zero input up to
  any armed frame of a block no longer than `N_SILENCE`, so the two forms agree on it. Builtins
  (`channel_arms`, per channel) and EQ (`Channel::arms`, per channel, every section) use it.
  `lane::kernels::svf_state_held` is the holding mask.
- **Whole-block zero scan (root item 2).** `lane::kernels::silence_skip_block`, the counter of
  every block whose thresholds no section reads (builtins and EQ), has three forms: live (last
  frame non-zero on every lane; one compare, unchanged), settled (a whole-block zero scan, a load,
  an `eq` and a `mask_and` per frame in four independent chains; when every lane is silent
  throughout or live on its last frame the counter is `min(run + frames, 2^24)` or `+0.0`) and
  transition (the counter's frame loop, only when a lane's silence starts inside the block). The
  settled and transition forms are one outlined function per lane width (`silence_skip_settle`),
  which also lowers the iOS memset counts (below).
- **m6.** The EQ declares its two rest planes as scratch: `REST_PLANE_BYTES_PER_FRAME` = 8 bytes per
  frame per prepared lane (one `f32` threshold per channel), in its descriptor
  (`scratch_bytes_per_frame`), held by `the_rest_planes_are_the_scratch_the_descriptor_declares`.
  The third (all-`+0.0`) plane the ruling counted is gone: the unarmed form reads no plane.

**Root item 1: the accepted app-shape cost (recorded, not tuned).** Against the per-word build P0
(the parent with `flush_pair` as two `flush`, the verifier's recipe), the checkpoint measured the
64-track app-shape document at +3.5 % / +3.9 % p50 (rounds 1 / 2), above the owner's 2 %. Cause:
the builtin chain's four vector constants are carried as words (`InputChainConstants`) for the iOS
`memset_pattern16` ratchet (#1018), and V8 then compiles the builtins' dual loop to 218 instructions
against 196 with splatted constants; splatting them instead raised the builtins' iOS count from 186
to 397 `memset_pattern16` calls, which the ratchet refuses. The root accepted the cost now; #1451
(*Let the builtins splat their chain constants without iOS memset calls*) is filed to remove the
cause. Benchmark (descriptive; `web-mixing-automation-benchmark.mjs run`, one warmup and two
measured rounds per module, interleaved, `taskset -c 31`, uncontrolled host, not retried; every
output digest equal across modules):

| p50 vs P0, round 1 / round 2 | A9 (`a124b40be`) | checkpoint `4264bb190` | after the scan (this tree) |
|---|---|---|---|
| mono console, quiet | +2.6 % / +2.5 % | +9.4 % / +1.2 % | +1.4 % / +0.5 % |
| mono console, restated | +2.6 % / +2.0 % | +7.8 % / +0.4 % | +2.5 % / +0.9 % |
| mono console, automated | +2.9 % / +1.9 % | +7.6 % / +0.3 % | +2.2 % / +2.3 % |
| sixty-four-track console | +2.8 % / +3.4 % | +1.0 % / +0.4 % | +12.8 % / +2.1 % |
| app shape | +4.8 % / +4.3 % | +3.5 % / +3.9 % | +26.1 % / +3.2 % |
| bus-and-send console | +3.5 % / +3.2 % | +1.8 % / +0.6 % | +13.7 % / +2.6 % |

The first two columns are the checkpoint's run (P0 148,873-344,506 ns; load 10.5-12.6 on 32 threads);
the last is a second run of P0, the checkpoint and this tree (load 2.8-3.7), in which the
checkpoint's own round 1 also read +4.5 % / +11.6 % / +5.0 % on the three 64-track documents and its
round 2 +2.2 % / +3.7 % / +3.2 %. Round 1 of that run is disturbed on the 64-track documents for
both the checkpoint and this tree (a verifier was building on the host); round 2 is consistent with
the checkpoint's run. On live documents the scan cannot cost anything (their banks' last frames are
non-zero), so the app-shape cost is the accepted #1018 one.

**Root item 2: the silent-lane penalty after the scan (native, descriptive).** A scratch harness
(not committed): the builtin input chain at `Simd8`, a 30 Hz high-pass into an 18 kHz low-pass,
128-frame blocks, eight live lanes against seven live and one silent (`input_chain_block`, 400,000
blocks per point); the parametric EQ bank at eight lanes (four bells and both cuts, 48 kHz,
128-frame quantum), all live against one silent lane and against two padding lanes (200,000 blocks).
One warmup and two measured rounds, `taskset -c 31`, one invocation per build, not retried:

| silent lane / all live, round 1 / round 2 | A9 `a124b40be` | checkpoint | this tree |
|---|---|---|---|
| builtins, 8 lanes, one silent | 1.270 / 1.388 | 1.152 / 1.153 | 1.030 / 1.035 |
| EQ bank, one silent lane | 1.081 / 0.981 | 1.062 / 1.046 | 0.996 / 1.009 |
| EQ bank, two padding lanes | 1.080 / 0.996 | 1.060 / 1.048 | 0.996 / 1.009 |

All live, measured rounds: builtins 1,764-1,766 ns per block, EQ bank 4,429-4,491 ns (this tree). The EQ's
penalty is gone (within run-to-run noise). The builtins keep about 3 % (55-60 ns per block):
attributed by a timing-only build with the scan removed (silent/live 1.001 / 1.001), so the residual
is the scan itself, 2 x 128 frames of load, `eq` and `mask_and`, throughput-bound. Halving it would
need a bitwise OR of sample words across frames before one compare, a new `Lane` operation; not
done (it was not ruled and is a cross-cutting lane-op change).

**Root item 2: bit identity.** One-time differential (scratch, not committed): the EQ (scalar
effect and native bank, dual and collapsed, restores of in-band pairs and counters up to
`16,777,100`, ramps, 128- and 8,192-frame quanta, 44.1-96 kHz) and every builtin input-chain entry
point at `f32`, `Simd4` and `Simd8`, over nine input programs (live noise, silence, impulse then
long silence, sparse tiny trains either side of the window, live/silence with `-0.0`, a stopping
tone, a NaN in silence, noise with an exact zero every sixteenth sample so blocks end on a zero
without being silent, and block-aligned silence), plus counters seeded at `16,777,000..16,777,216`
on lanes at rest; one FNV-1a digest per scenario over every output word, state word and counter.
Base `a124b40be` (the A9 bits) against this tree: **908 of 908 scenarios identical**. Mutants of the
scan (each applied, harness re-run, reverted) move scenarios: select dropped (130 moved), saturation
dropped (21), transition check dropped (548), two of the four scan chains dropped (126); dropping
the scan's remainder frames moves none of the harness's scenarios (its block lengths leave no
such lane) and is caught by the new g4 test.

**Root item 3: the V8 spill gate's armed rows.** Reported with a ceiling equal to today's count, so
a rise fails: dual armed tail 2 (two general-purpose words, `[rbp-0x110]`, `[rbp-0xc8]`), dual
armed pair 11, mono armed pair 0, mono armed tail 0, mono armed masked pair 0. A ceiling row is
checked like a held row (fails closed on a missing or ambiguous loop) and fails above its ceiling;
the four held unarmed rows stay at zero. The dual unarmed pair stays reported (10 slots).
`scripts/check-web-audioworklet-v8-spill.py` (rows, rule, docstring, three self-test cases).

**Root item 4: `track_delay`'s zero-delay digest.** Re-pinned individually (`4264bb190`) for one
reason: the EQ's rest planes are declared scratch, which the estimate charges in
`declared_effect_bytes` (nine EQs at a 128-frame quantum, +9,216 bytes): `declared_effect_bytes`
8,496 -> 17,712 and both plan-byte totals 150,631 -> 159,847, three tokens of the `estimate` row.
Re-derived on the dumped canonical text: reversing the three tokens hashes to the A9 pin
`bb25028730eb...ffbfc40bf` exactly. The test's comment named the A9 pin with the new pin's tail;
corrected.

**iOS `memset_pattern16` ceilings** (`scripts/lib/aarch64-known-defects.py`, lowered as the check
asks): parametric-eq 88 -> 48 (checkpoint: `FLUSH_EPS` carried as a word in both cascade forms),
then builtins 71 -> 40 and parametric-eq 48 -> 47 (the scan's settled and transition forms are one
outlined function per width, so their constants are not materialised at each call site; the first
inline cut of the scan rose to 104 and 51 and was refused by the check). Multiband 566 unchanged.

**m1.** The attempt-5 re-pin of `fixtures/builtins/v1/reference/filter-response.csv` was made while
#1427's Hazards forbade any `--write` re-pin of that file, and attempt 5's record did not
acknowledge that order or ask root. The re-pinned content is the genuine A9 output (the verifier
reproduced it). #1427's premise was then disproved: release builds of the same commit at two
checkout paths 41 characters apart write byte-identical fixture trees (and so does the parent);
the supposed second variant `d577dab4` is the output of #1328 attempts 1-3's law (an independent
scalar twin reproduces it on 1,630 of 1,630 rows), observed through a stale build (a `git archive`
export carries commit-time mtimes, so a shared target directory reused an older build). Root closed
#1427 as not planned; its STREAMS rows were removed in `50103f0dc`.

**m5.** Corrected: decision 15's #1328 entry and `crates/lane/src/kernels.rs` no longer say live
audio pays nothing for the rule; they state what remains (the per-block armability test and
last-frame compare, the silent-lane scan, the #1018 constants' browser cost). `dsp-research/filters.md`
states rounding changes as relative to the signal: once a flush has changed a state word the two
runs round differently, and later loud audio can differ by about one ulp of the signal (the
verifier: `9.3e-10` at a relative `1.6e-7` with the final window, `3.7e-9` at 2,048 samples); the
attempt-5 "up to `4.66e-10` at a relative `1e-7`" is such a change. The 1,024-sample residuals are
search maxima: attempt 5 found `1.31e-9` / `1.33e-9` at 88.2 / 96 kHz, the verifier's independent
`f32` simulation `1.53e-9` / `1.68e-9` (`filters.md`, `lane::SILENCE_TIME_FRAMES`'s doc). The
conclusion (a time window, not 1,024 samples) holds either way.

**m7.** Dropping the section-input term loosens the provable one-tail chain bound from `3.74e-10`
(each section flushes once) to `3.1e-7` (-130.2 dBFS, above a 24-bit LSB); the largest change
measured, with and without the term, at 1,024 samples and at the final window, at every rate, is
the same (attempt 5 and the verifier's independent simulation). Recorded in `filters.md`.

**Floors (re-derived after the scan, which adds no lane-op per lane-sample on live audio).** EQ
standing inventory 27 (the value before #1328; 31 on a block that can arm, plus 5 for the rest
plane), strip 307 (10.372 cycles/lane-sample), `floor_lane_ops(active) = 24 * active + 3` on live
audio (153 for six or refused) and `28 * active + 3` on a block that can arm (177); builtins 69
unchanged. `tools/bench/src/floor.rs`, `scripts/console-benchmark-record-lib.jq`,
`scripts/test-console-benchmark.sh` (the two "costed at the wrong inventory" cases now use 307 and
`69 + 27 + 81.5`), `docs/rulings/effect-floor-accounting.md`.

**Docs.** `crates/lane/src/kernels/builtins.rs`: six links to the removed `svf_step_armable` now
name `svf_step_when` and `channel_arms`; `svf_step_when`'s doc states both conditions for the
unarmed form.

**Tests and mutation evidence** (each mutant applied, the named tests run, reverted; the final tree
is green):

| id | mutant | red tests | verdict |
|---|---|---|---|
| F1 | `silence_skip_block` settled form drops the per-lane select (every lane gets the advanced count) | `g4_silence_skip_block_is_the_frame_loop`; differential: 130 of 908 scenarios move | RED |
| F2 | settled form without the `2^24` saturation (`run + frames`) | `g4_silence_skip_block_is_the_frame_loop` only (no other `lane`, `parametric-eq` or `builtins` test); differential: 21 move | RED |
| F3 | no transition check (settled form on every non-live block) | `g4_silence_skip_block_is_the_frame_loop`; differential: 548 move | RED |
| F4 | the scan drops its remainder frames | `g4_silence_skip_block_is_the_frame_loop`; differential: none move | RED |
| F5 | the scan combines two of its four chains | `g4_silence_skip_block_is_the_frame_loop`; differential: 126 move | RED |
| F6 | spill gate ignores a row's ceiling (99) | `--self-test` (verdicts `..., 0, 0, 1`) | RED |
| F7 | spill gate reports ceiling rows as info | `--self-test` (`..., 0, 0, 0`) | RED |
| F8 | spill gate ceiling compared off by one (`<`) | `--self-test` (`..., 1, 1, 1`) | RED |
| F9 | dual armed pair ceiling 10 on the real module | the gate: `FAIL dual armed depth-2 pair ... above the row's ceiling of 10` (exit 1) | RED |
| MK | skewed cascade reads frame `i`'s threshold for every section | `g2_skewed_cascade_arms_each_section_on_its_own_frame` | RED |
| MI | `input_chain_block` decides armability from channel 0 only | `the_right_channel_arms_its_joint_flush_alone`, `any_state_word_in_the_band_arms_the_block` | RED |
| MW | `svf_state_held` ignores `ic2` | `any_state_word_in_the_band_arms_the_block` | RED |
| ML | `silence_armable_holding` drops the long-block term | builtins `a_block_longer_than_the_window_arms_from_rest`, EQ `a_block_longer_than_the_window_renders_as_its_quantum_sized_parts` | RED |
| ME | EQ arms from the left channel only | `the_joint_flush_arms_for_whichever_channel_and_section_holds_the_band` | RED |
| MS | EQ holding mask reads section 0 only | the same, the long-block test, `a_fixed_point_inside_the_joint_band_is_cleared_once_the_flush_arms`, `the_low_shelf_fixed_point_reaches_exact_rest` | RED |
| MM | EQ collapsed body never arms | `mono_collapse::the_collapsed_body_arms_its_joint_flush` | RED |
| MD | EQ descriptor declares no scratch (m6) | `the_rest_planes_are_the_scratch_the_descriptor_declares` | RED |
| MG | joint test reads the first magnitude only (`a1 < rest`) | `g5_native_digests_match_pins` (the armed case) | RED |
| MF | spill gate rows and their anchors ignore the form | `--self-test` (`..., 2, 2, 1, 1, 1`); either check alone stays green, since the other keeps the forms apart in the synthetic listing | RED |
| -- | `silence_skip_block` always takes the transition loop | none: equivalent by construction (the loop is the reference); the scan is a cost saving | equivalent |

Test value, one sentence each:
- `g4_silence_skip_block_is_the_frame_loop` (new): a skip form that leaves a counter anywhere other
  than the frame loop does -- a settled form that does not saturate at `2^24` (no other test reaches
  it), drops the per-lane select, skips the transition check or loses scan frames.
- `g2_skewed_cascade_arms_each_section_on_its_own_frame` (m3, MK): a skewed cascade that reads frame
  `i`'s threshold for section `k` instead of frame `i - k`'s.
- `input_chain_arming` (m3, MI; three tests): an input-chain body that decides armability from
  channel 0 alone, a holding mask that skips a state word or section, or an armability test without
  the long-block term.
- EQ `exact_rest::the_joint_flush_arms_for_whichever_channel_and_section_holds_the_band` and
  `a_block_longer_than_the_window_renders_as_its_quantum_sized_parts`,
  `mono_collapse::the_collapsed_body_arms_its_joint_flush`: the EQ's arming decision from one
  channel or one section, without the long-block term, or never in the collapsed body.
- `the_rest_planes_are_the_scratch_the_descriptor_declares` (m6): a descriptor that under-declares
  the rest planes.
- G5 `svf_block/armed/impulse_tails` (m4; pin added individually, reason: the only corpus case whose
  joint test runs against armed thresholds, so `i32x4.max_u` and the armed compare are exercised on
  wasm): a joint test that does not take the larger magnitude.
- Spill self-test, two forms (checkpoint) and ceilings (this tree): rows that ignore the form, and a
  ceiling that is ignored, unchecked for missing loops, or compared off by one.

**Gates (this tree).**

- gate-4 `cargo test --all-targets` set (14 packages, spec features, `--no-fail-fast`): 879 passed,
  0 failed; `cargo test -p audit -p graph-compiler -p builtins-compiler -p graph -p host-core
  -p host-web -p capi -p bench`: ok;
- `cargo test --release -p lane -p math -p wasm-gates --features math/lane`: ok (G5 green);
- `run-wasm-gates.sh`: exit 0 (native, simd128, V8 spill gate with the rows above; self-test 33 ok);
- `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files), `graph_fixture --check`,
  `audit unfused-fma conformance` (SVF `unfused`, the silence pass included, 0 mismatches),
  `check-graph-determinism.sh` (100/100), `check-cross-targets.sh` (builtins 40, parametric-eq 47,
  multiband 566 `memset_pattern16` calls): ok; no fixture or digest moved beyond the
  `track_delay` re-pin above;
- `check-lane-policy.sh`, `check-dsp-research.sh`, `check-workspace-policy.sh`,
  `check-realtime-policy.sh`: ok;
- `cargo clippy --locked --workspace --all-targets -- -D warnings` (and with the test-support
  features), `cargo fmt --all -- --check`: ok;
- `cargo test -p bench floor`, `test-console-benchmark.sh`: ok;
- worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`: ok.

**Open items.**
- The builtins keep about 3 % per block in an eight-lane bank with one silent lane (the scan).
- The app-shape browser cost (+3 % to +4 % p50 against P0) stands until #1451.
- The multiband compressor still runs the counter and the joint flush on every frame (#1018).
- n3 (attempt 5) stands: documented, not changed.
