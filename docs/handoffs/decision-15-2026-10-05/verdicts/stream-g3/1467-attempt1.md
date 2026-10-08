FAIL

# #1467 attempt 1 (slice B2): derive the live input section's tail gain and state its composition

Commit under review: `ae054b277` (parent `215dbfda5`; diff reviewed against `ff9bdbc28`), branch
`codex/d15-stream-g3`. Verifier: opus-xhigh, 2026-10-08. Exported with `git archive` into
`/tmp/claude-1002/v1467/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1467/target`. The worktree was
not touched.

One MAJOR, one MINOR, four NITs. The MAJOR is the one thing root asked to be exact: L2's stated
scan coverage claims two things it does not cover. The proof holds when checked as a proof, `G_t`
is sound, every gate passes, and every recorded mutant I re-ran reproduces. No counterexample
exists on the kernel: my own oracle, fed with the words the real kernel loads, finds at most
`trim x sup` = 60.17 (+35.59 dB) against `g_t` = 4.178e4 (+92.42 dB), a 56.83 dB margin.

## MAJOR 1: L2's scan claims the production disable, enable and latest-event histories, and covers none of them

Root's amendment (spec :189-197): the record states the scan precisely, why it covers the ramp
space it claims, "and claims no more than it covers". Two claims are false.

**(a) The identity endpoints are not the kernel's disable or enable.**
- The scan forms every ramp, the identity endpoints included, as the all-six linear mixture
  (`mixture`, `crates/builtins/tests/tail_contract.rs:2974-2992`). Its doc says "#1407: ... a
  disable or enable mixes toward or from the identity" (`:2977`). The record says "Why these: ...
  the identity (the disable/enable mixtures)" (spec :357).
- #1407's law is different (`crates/builtins/src/lib.rs:1431-1441`,
  `docs/rulings/builtins-input-liveness-d2.md:22-31`):
  - rule 2, a disable: `c1`, `a2`, `a3` freeze at the current design, only the mix ramps, and the
    completion snaps to the identity and clears the integrators;
  - rule 3, an enable from rest: the recursion jumps to the design at once, only the mix ramps.
- I recorded the real kernel's words frame by frame (verifier probe, export only). A disable of a
  100 Hz HPF at `N - 1`: `c1 = 0.010024` from `N` to `N + 62`, `m1` from -1.3921 to -0.0221, the
  identity and a clear at `N + 63`. An enable: the design's recursion already at `N`, the mix from
  weight 1/64. Neither is in the scan.
- The all-six identity mixture the scan does run is rule 4 from an identity section with non-zero
  integrators. #1407's record found no production writer of that state (test-support injection
  only; `docs/handoffs/decision-15-2026-10-05/verdicts/stream-g/1407-attempt1.md:59`, #1329 R8). So
  the 100 identity-endpoint histories per rate (10 ramps per section, 5 offsets) are, at best, a non-production rule-4 path, and the
  production disable and enable histories are not scanned.

**(b) The event offsets are one frame off the kernel's timing.**
- The scan's word at `N - s + i` has weight `min(i + 1, 64) / 64` (`tail_contract.rs:3196`). The
  kernel is current-then-advance: the event frame uses the current words, frame `A + i` weight
  `i / 64`, `A + 64` the target (`crates/lane/src/kernels/builtins.rs:1145-1146`, D2 ruling).
- Recorded on the real kernel: a rule-4 event at `N - 1` gives weight 1/64 at `N` and the target
  at `N + 63`, so 63 frames are in flight from `N`. The scan's "event at `N - 1`" has weight 2/64
  at `N` and 62 frames in flight: it is the kernel's event at `N - 2`.
- So "the offsets span the ramp's position at `N` from just started" (spec :358) is false: the
  latest admitted event (`N - 1`) is not scanned, and every "N - s" label is the kernel's
  `N - s - 1`.

**Why MAJOR although no number moves.** The amendment made the truth of this statement part of
L2's definition; L2 has no other test value beyond M2. The numbers are safe: my oracle over the
kernel's own words (rules 2, 3 and 4, restarts, both sections moving, every event offset 1-63)
stays at 60.17 at most. But the committed record and a committed doc comment misstate #1407 and
claim coverage the gate does not have.

**Fix (bounded; L2 and its record only).** Build the scan's histories from the #1407 law, best by
recording the words the real kernel loads per frame (rule-2 completion clears included), and
include the event at `N - 1`. Or restate the coverage exactly: the all-six mixtures with the stated
weight, which for an identity endpoint are rule 4 from a restored identity, not the production
disable or enable. Correct the `mixture` doc either way.

## MINOR 1: the gate-2 evidence statement is not true

The record says a `--help` probe "may have started it a second time" (spec :373-376). It could
not: `input_bound_budget` refuses any argument other than `calibrate` and exits 2 before any work
(`crates/builtins/examples/input_bound_budget.rs:1138-1147`). Verified: `--help` prints "unknown
argument" and exits 2 at once. The `tail -8` truncation statement is true. Restate the sentence.
Gate 2 is descriptive; my one run is below; the batch verifier's run is the figure of record.

## NITs

1. The record says the derivation was written before the code; the derivation and the code land
   in one commit (`ae054b277`), so history cannot show the order. Not a defect; unverifiable.
2. `arrival_window` drops a NaN through `f64::max` in its state-output fold
   (`crates/math/src/tail.rs` `arrival_window`, as #1466's `window`). Unreachable: `live_zones`
   refuses a NaN contraction and caps every input. No change needed; noted for #1485.
3. `input_section_live_bound` turns an `Err` from `live_cascade_composition` into `Unstated` with
   `.ok()` (`crates/builtins/src/tail.rs:520-524`). That is the correct result, but the table test
   is the only thing that would notice; a one-line comment would say it is deliberate.
4. The derivation names the wrong settled pair as the kernel's supremum
   (`docs/derivations/1379-graph-tail-composition.md:648-651`: "close to the settled pair's `l1`
   (+35.0 dB with the trim for the 10 Hz HPF into the top LPF)"). The largest settled pair is not
   the 10 Hz one: my scan finds 3.7961 (+35.59 dB with the trim) for a 200 Hz HPF into the top LPF
   at 44.1 and 88.2 kHz, and 3.7933 for 500 Hz at 96 kHz, against 3.5287 for 10 Hz. L2's endpoints
   (10, 100, 1k, 10k) step over it. The "at most +35.6 dB on L2's scanned histories" clause is
   true and no bound depends on the sentence, but #1485 will read it as the kernel's target.

## The proof, checked as a proof

The derivation is "The live tail gain `G_t` and the statement (issue #1467, slice B2)",
`docs/derivations/1379-graph-tail-composition.md:496-688`, against `LiveBound::arrival_window`,
`arrival_continuation`, `settled_input_gain`, `composition` (`crates/math/src/tail.rs:1640-1746`,
`:2012-2023`) and #1433's derivation.

**(P3) the split at `M`: sound.**
- The kernel is the exact map plus perturbations. Each relative perturbation is bounded by
  `mu ||s|| + mu_x |v|` of the kernel's own state and input; the triangle inequality splits that
  bound into `a_1 + a_2 + a_3`, and the proportional split `e^i = e a_i / sum a` gives each part a
  perturbation at most `a_i`. The parts are defined by induction, step by the same linear map,
  and sum to the kernel. This is an existence construction; it needs no linearity of `f32`.
- Resets (joint flush, per-block recovery, rule-2 completion, rule 3) zero every part at once;
  the mono collapse's copy carries parts with state and words. The flush part alone carries `F`.
- The late part's state is zero through `M`. Its input is `fl(x trim)` from `M`, at most `g eps`.
- The output is at most the sum of the three parts. The early part is B1's relative part (input
  zero from `M`), the flush part B1's flush part. So `g_t` must bound `sup |y_late| / eps`. Correct.
- The second split at `M + 64` (window input `M ..= M + 63`, settled input from `M + 64`) is the
  same construction. Words are fixed from `N + 63` on (kernel), so from `M + 64`: correct.

**(W) the window input's part: sound.**
- `Phi^(j)`: the induction holds for every zone that contains the word. The move from frame to
  frame is a hold, one ramp step (zones within `step`, neighbour relation symmetric; I checked the
  `first_neighbour`/`last_neighbour` construction), or a zeroing. Rule 3 happens only at an event,
  before `N`. A first section at the identity keeps zero late-part state (`A = I`, `b = 0`).
- The first section's output `(h q_k + omega) Phi^(j)_k G + delta [j < 64] G`, maximum over every
  zone, `F = 0`: correct. The LPF frequency-blind with `rho_ramp` and `iota`; its output
  `gamma_2 S_j + delta Y_j`: correct. The code computes `O_0 .. O_64`, `S_65`, `E_64`: indexes
  match (`frame < ramp_frames` carries the input; `energy` at `frame == ramp_frames`).
- After the window: `tau = sigma - e2 y_1[n-1]` from `n0 = M + 65` needs `B` fixed from `n0` and
  zero window input at `n0 - 1 = M + 64`: both hold. The start `S_65 G + c_y H`,
  `H = min(E_64 G, Phi_c G)` (`Phi_c` is a fixed point over any history with input at most `G`, so
  it bounds this part too), `X = 2 c_y H`: as #1433, with `F = 0`.
- The stopping rule: a computed `M u <= u` (rounded up) gives the exact `M^i u <= u`, and the
  output was read at that `u` before the break. Covered groups: #1433's `covers`. Correct.

**(Z) the settled input's part: sound.**
- Zero state at `M + 64`, fixed words, input at most `G`: `||s|| <= beta_k G / (1 - r_k)` by
  induction; `L_1(k)` with the settled `q^s`; the identity `delta`.
- The LPF's output row is `1/2 (m1, m2)(I + A)` for any mix words (checked from `c`'s formula).
  Its mix words are `theta (0, 1)` plus the allowance: `||(0, 1)||_V* = sqrt(2)` exactly, and
  `||(-k, -1)||_V*` with the `f32` `k` is about `2.4e-8` below it, so `first_mix_row` does not
  cover the LPF. The module's values: `first_mix_row` 1.4142212747, `second_mix_row` 1.4142212976
  (both above `sqrt(2)` by the mix box). The separate `second_mix_row` is required and correct.
- `L_1 L_2` over every zone with settled constants: the LPF's designs share the HPF's pole domain
  (same recursion words per cutoff); #1433's scan holds every design of both sections in a zone.

**`G_t` and the statement: sound.**
- `tail_gain = (g W* + g L_1 L_2) SLACK`, every product and sum rounded up; `g W*` at the input
  scale `g`. `G_t = ceil_mB` (`ceil_millibels`, `Rounding::Computed`: `x + |x| 2^-30 + 2^-30`,
  upward for a negative stall too).
- `live_stated_composition` checks `tail_gain > peak_gain || tail_stall > peak_stall` on the
  stated millibels and states `Unstated` then; equality is admitted. Raw: `sigma_t <= sigma_p` by
  construction (`min`), `G_t <= G_p` measured (51.7-51.8 dB); `ceil_mB` is monotone.
- Values (L3' print, my run): `g_t` 4.174511e4 / 4.174672e4 / 4.175823e4 / 4.175878e4, parts as
  the record; stated `G_t = 9,242` mB at every rate.

## Counterexample attempt: none found

- My own oracle (`verifier_1467.rs`, run in the export only; source and log kept at
  `/tmp/claude-1002/v1467/verifier_1467.rs` and `/tmp/claude-1002/v1467/verifier-scan.log`): the words are recorded from the real
  `InputBuiltins` frame by frame (so the real #1407 law, its `f32` ramp words, rules 2, 3 and 4,
  restarts and the event timing), and the row `l1` is computed by forward impulse columns, not by
  the adjoint. Per rate 3,954-3,972 histories: HPF moves (LPF at the maximum or the identity), LPF
  moves (HPF at 10 Hz or the identity) between 14 endpoints (identity, 10 Hz to 20 kHz, one `f32`
  below the maximum, the maximum) at events `N - 1, 2, 8, 32, 63`; every valid settled pair; 400
  random both-sections histories; 400 random restarts; 287-312 per rate with a rule-2 clear.
- Largest `trim x sup`: 60.164 / 60.172 / 60.120 / 60.120 (+35.59 / 35.59 / 35.58 / 35.58 dB);
  margin to `g_t` at least 56.83 dB. Residual state at the end below `1.0e-15`.
- The real `f32` kernel, driven by the worst-sign input of `2^-12` for the top three histories per
  rate, gives `|y(n*)|` within `1e-4` of `trim eps row l1` (ratios 0.99961-1.00008). So the oracle
  is the kernel, and the kernel's late part is 56.8 dB below `g_t eps`.
- A full (N2) counterexample needs the late part above `g_t eps`; the kernel cannot reach it. The
  early and flush parts are B1's, verified in #1466.

## The committed oracle (L2)

- Correct for what it models: `cascade_frame` composes the two sections exactly; the backward rows
  inside the ramp, the ramp-end columns, the partial `l1` and both remainders
  (`free_response_bounds`: the `V`-norm sup and sum bounds, checked) are right. Its self-checks
  pass: settled supremum against the impulse response's `l1` (to `1e-9`), ramp rows at `N + 40`
  and `N + 200` against forward sums (to `1e-12`). O1 is red at the second.
- What it models is not what the record says it covers: MAJOR 1.

## Mutation runs (re-done in the export; release `tail_contract` with `test-support`)

| mutant | red | green |
|---|---|---|
| M1 window part dropped | L3' only (`2.5126e2` against `4.1745e4`); M1t: the table test | L2, L5', rest |
| M2 trim and window dropped, mirrored | L2 only | L3' (mirrored), L5', rest |
| M2b the same, not mirrored | L2, L3' | L5', rest |
| M3 settled input term dropped | L3' only (`4.1494e4`) | rest |
| M5 window frames' outputs dropped | none (equivalent, as recorded) | all |
| M7 `second_mix_row = first_mix_row` | L3' at its mix-row check (1.4142212747 < 1.4142212976) | rest |
| S1 statement crosses the gains | L5', the table test | rest |
| S2 statement `tail_gain = peak_gain` | L5', the table test | rest |
| S3 statement swaps the stalls | L5', the table test | rest |
| S4 check `>=` (equality refused) | the statement's unit test | all 22 `tail_contract` |
| S5 check removed | the statement's unit test | all 22 `tail_contract` |
| S6 `math` stalls swapped | L3, L5', L2, the table test | rest |
| S7 `math` `sigma_t = sigma_p (1 + 1e-5)` | L3 (order), L5' (raw order), the table test | rest |
| S8 `math` `tail_gain = peak_gain` | L3', the table test | L2, L5' |
| O1 L2 oracle drops the ramp columns | L2 at its `N + 200` forward-sum check | rest |
| X1 (mine) `W* + L_1 L_2` as a max | L3' only | rest |
| X2 (mine) window input on 65 frames | none: it only raises `G_t`, below 1 mB (sound direction) | all |
| X3 (mine) `Phi^(j)` without neighbour moves | L3' only (`2.245e4`) | rest |
| X4 (mine) continuation start without `c_y H` | L3' only (`3.895e4`) | rest |
| X5 (mine) continuation read at its first frame only | L3' only (`4.073e4`) | rest |

Every recorded result I re-ran matches the record.

## Test value (one sentence per new test)

- **L2** `live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`: red for a
  derivation error mirrored into the recomputation that puts `G_t` below the exact late-input
  supremum (M2), which L3' and L5' cannot see; it does not defend the window term (as recorded).
- **L3'** (the `G_t` row and mix-row check in
  `live_bound_carries_every_term_an_independent_recomputation_requires`): red for a dropped window
  term (M1, the only value gate red), a dropped settled input term (M3), a neighbour-free window
  recursion (X3), a continuation without its start term or its growth (X4, X5), a max for the sum
  (X1), and the LPF mix row replaced by the HPF's (M7).
- **L5'** `live_bound_states_the_composition_rounded_up_with_its_orders`: red for a statement that
  crosses the gains, sets the tail gain to the peak gain or swaps the stalls (S1-S3), and for a raw
  tail stall above the peak stall inside one millibel (S7), which only the table pin also sees.
- **Unit** `the_live_statement_admits_equality_and_refuses_a_tail_value_above_its_peak_value`: red
  for a strict check (S4, the equality boundary) and for no check (S5); nothing in `tail_contract`
  sees either.

## Other checks

- **The re-pin.** One table, four entries, each listed with its values and one reason (this slice
  states the live composition; the table test holds table = computed). The tail and rest entries
  are unchanged (diff checked). Individually reasoned: yes.
- **Render memory and bytes.** No render code changes. `NodeTailBound` and `CompositionBound` keep
  their size; the new `LiveCascade` fields are control-plane only. `audit capi` `pcm_digest`
  `cb10fbface44a3a4` (same as #1466 and the stream-G2 batch); builtins fixtures ok (50 files);
  `run-wasm-gates.sh` ok; the worklet chain passed (`check-browser-expected-resources --artifacts`: every retained row within its budget; source total 3,358 of 3,648, as recorded).
- **Fixed design over the budget.** It reports the live bound, so it now states the live
  composition (`EFFECT_CONTRACT_V1.md`, edited to say so). Sound: the live bound covers every
  fixed design. Nothing reads compositions before #1379.
- **Amendment points.** Hazard 1 restated as a property of the bound: yes. L3' unique catch for a
  dropped window term with its run: yes (M1). Tightening folded into #1485, #1485 not edited: yes.
  L-D7 millibels of both gains and both stalls: yes.
- **AGENTS.md.** No version-suffixed names. No queue (the acked-batch question does not apply). No
  source-grepping test. No byte pin beyond the table. No interim shortcut: the 56.8 dB looseness is
  stated and routed to #1485.

## Gates run (export)

| gate | result |
|---|---|
| `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract` | 22 passed (13.5 s) |
| `cargo test --locked --release -p builtins --features builtins/test-support --lib tail::` | 1 passed |
| debug `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features math/lane,builtins/test-support,lane/test-support` | exit 0; 45 test binaries, 0 failed |
| gate 8 `every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate` | 1 passed |
| gate 2 `input_bound_budget`, one invocation, `taskset -c 7`, loadavg 3.48 before, 3.45 after | exit 0; 16.5 ns needed (17 committed); worst median 24.787 ms, 96.6 % of 25.67 ms (descriptive) |
| `audit capi` | `pcm_digest cb10fbface44a3a4`, 100,000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls |
| `check-builtins-fixtures.sh . <audit>` | ok (50 files) |
| `run-wasm-gates.sh` (native + wasm simd128 + V8 spill) | ok |
| worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` (private TMPDIR, left empty) | all exit 0; source budget 3,358 of 3,648; expected.json digests and rows agree |
| `check-cross-targets.sh` | PASS (x86-64-v3; aarch64 iOS and Android checked and linted; #1018 expected failures; wasm simd128) |
| `check-workspace-policy.sh` | ok |
| `check-realtime-policy.sh` | ok (89 regions, 25 files) |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean (223 crates) |
| `cargo fmt --all -- --check` | clean |
| independent oracle (export only) | no counterexample; margin >= 56.83 dB; kernel = oracle to 1e-4 |
