FAIL

# #1328 attempt 4: adversarial verdict

- **Reviewed:** `git diff 12f1b40dc 8a06cbd22` on `codex/d15-stream-g`. The commit was exported
  with `git archive` to `/tmp/claude-1002/v1328a4/tree`. Two more exports took the mutations
  (`.../mut`) and a measurement copy with a scratch law switch in `flush_pair` (`.../meas`). I did
  not build or change anything in `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, Node v22.23.2 (V8 12.4.254.21-node.56), rustc 1.97.1. The host had a
  load of 10-16 (other builds were running), so I did not time anything.
- **Why it fails:** one MAJOR finding, in the A4 bound that A8 item 2 asked for. The code
  implements A8 exactly as written, and every gate is green. The restated bound and the claim
  "no output sample moves while the input is non-zero" are false at the effect's output, and the
  sparse-input residual is understated by about 76 dB. The root must decide again; see M1.
- **What passes:** the law and its threading, the threshold, gates 1-2, the twin, the g2 oracle,
  the EQ predicates and the elision proof, the op counts and floors, the spill-gate rows, the M2
  fold, the NIT folds, every gate I ran, and the GitHub sync.
- Small evidence files are in `/tmp/claude-1002/v1328a4/evidence/`: harness diffs, the f32 law
  simulator, the L1-bound computation, the max_u equivalence check, the V8 and AVX2 loop listings
  and the EQ measurement log.

## MAJOR

**M1. The joint flush moves bits on non-zero input samples, and the sparse-input loss is the
same dead zone that A8 withdrew: −120 dBFS measured at four shelves, not −196 dBFS.**

Affected text:
- spec A4 under A8 :245-257, the Listening line :256 and :275, the attempt record :753 and
  :759-761, and the open item :883;
- `dsp-research/filters.md:19`;
- decision 15 :589-594 ("an EQ applies its whole response to any non-zero input and the joint
  rule moves bits only on exact-zero input samples, in tails");
- `docs/BUILTINS_AND_METERING_V1.md:63` ("a non-zero input, however small, gets the filter's
  whole response");
- the `lane::flush_pair` doc, `crates/lane/src/lib.rs:219-224`.

**What is true.** On a sample whose *section* input is non-zero, `flush_pair` follows the per-word
law bit for bit (g4 proves this). For a continuous input with no exact zeros, followed by silence,
I reproduced the implementer's table on the real EQ at 96 kHz. I used a 3.84 Hz square, DC and an
impulse, from 1 down to 1e-30:

| config | max change, A8 vs per-word | attempt-3 law, live |
|---|---|---|
| one shelf, 10 Hz, S 1 | 0 live; tail ≤ 1.90e-13 | 4.88e-10 |
| one shelf, 100 Hz, S 1 | 0 live; tail ≤ 1.90e-13 | 4.86e-11 |
| four shelves, 10 Hz | 0 live; tail ≤ 3.42e-10 | 1.51e-6 |
| four shelves, 100 Hz | 0 live; tail ≤ 3.41e-10 | 2.21e-7 |

**What is false, mechanism 1: sparse input.** A section's output depends on its state. The joint
rule zeroes the state on any zero-input sample while both words are below `REST_EPS`. So for an
input whose non-zero samples are each below `L*` and have exact zeros between them, the state is
erased at every zero. The section never builds its response, and every *later non-zero* sample
renders differently. Measured on the real EQ (four +24 dB low shelves, S 1, 96 kHz; independent
f32 simulator in agreement to 3 digits):

| input (non-zero samples at a = 3e-11 unless noted) | 4 × 10 Hz | 4 × 100 Hz (a = 3e-12) | 1 × 10 Hz |
|---|---|---|---|
| `[a, 0, a, 0, …]` | **9.6e-7 (−120.3 dBFS)**, first move at sample 2 (non-zero input) | 9.6e-8 (−140.3) | 2.3e-10 (−192.7) |
| `[a, 0, 0, 0, …]` | 5.1e-7 (−125.8), i.e. all four boosts lost (A8 output ≈ 0.04 % of per-word) | 5.1e-8 (−145.8) | 1.2e-10 |
| impulse every 128 samples | 1.6e-8 (−155.9) | 1.6e-9 (−175.9) | 3.6e-12 |
| random {−a, 0, +a} (a = 3e-12) | 2.7e-9 (−171.3) | 2.4e-9 (−172.5) | 1.3e-12 |
| lone impulse (the record's case) | 1.6e-10 (−195.8) | 1.6e-10 | 1.9e-13 |

- In `[a, 0, a, 0]`, section 1's output is never exactly zero, because the zero sample still
  outputs the old state. So sections 2-4 keep their boosts, and the loss is section 1's whole
  +24 dB: `(63,096 − 3,981)·a/2 ≈ 8.9e-7`.
- In `[a, 0, 0, 0]`, every section sees exact zeros, so every boost is lost.
- The bound is the withdrawn A4/A7 chain bound `‖h_chain − G_direct·δ‖₁·L*`:
  - **2.27e-6 (−112.9 dBFS)** at 4 × 10 Hz (‖·‖₁ = 74,581);
  - 2.28e-7 (−132.8 dBFS) at 4 × 100 Hz.
- Relative to the input peak, the loss is about +90 dB, so it is not a tail under D15-4(b)
  (`|y| < P·10^(−144/20)`). It is above a 24-bit LSB (−138.5 dBFS), which is the criterion A8
  used to reject attempt 3 at −116 dBFS.
- The product reaches this path: `trim_db` goes down to −144 dB, so a dithered quiet 16-bit
  passage (exact zeros between ±1 LSB samples) trimmed by about 120 dB reaches an EQ at about `L*`.

**What is false, mechanism 2: an exact zero inside the chain.** The section input is the output
of the section upstream, and that output can be exactly zero by cancellation while the effect
input is not. An example is the EQ's own HPF cut once a DC or slow-square input has been blocked.
Then the downstream shelves' rule fires while the EQ input is live. Measured at the EQ output while
its input is non-zero:
- HPF 40 Hz + four 100 Hz shelves, square 1e-6: **1.86e-9 (−174.6 dBFS)**;
- HPF 20 Hz + four 10 Hz shelves, DC 1e-8: 3.1e-10;
- builtin HPF 10 Hz → LPF, DC 1.0: 6e-15.

**Builtins and LR4 (A8 item 2 asked; the implementer did not measure them).** All changes are
small, because these filters do not boost:

| chain | continuous: live / tail | sparse (worst) |
|---|---|---|
| builtin HPF 10 Hz → LPF max | 0 / 1.9e-14 | 9.1e-12 (−220.9 dBFS) |
| builtin HPF 10 Hz → LPF 12 Hz | ≤ 6e-15 / 1.5e-14 | 9.0e-12 |
| HPF 10 → LPF 20, trim +24 dB | ≤ 1.0e-14 / 1.6e-14 | 9.7e-14 |
| LR4 80 Hz, both bands | 0 / 2.8e-14 | 1.2e-12 (−238 dBFS) |
| LR4 8 kHz, both bands | 0 / 2.0e-14 | 2.3e-14 |

**Fix (root decision first).**
- (a) Accept, with text that is true. Replace the bound and the claims with: "bits move only on
  or after a sample whose *section* input is exactly zero; continuous input then silence: ≤ 1.9e-13
  (largest measured 2.4e-13) at one shelf and 3.4e-10 at four; sparse input below `L*`: up to
  `‖h_chain − G_direct·δ‖₁·L*` = 2.3e-6 (−112.9 dBFS) at four 10 Hz +24 dB shelves, measured
  9.7e-7 (−120.3 dBFS); an in-chain cancellation zero: measured 1.9e-9 (−174.6 dBFS)". Say plainly
  that this is the A4/A7 dead zone for sparse signals.
- (b) Make the law need a time scale of silence. No memoryless per-sample rule can tell "a tiny
  sample, then a zero, then a tiny sample" from "a tiny sample, then silence". The minimal correct
  form is a zero-run gate:
  - `z' = (x == 0) ? z + 1 : 0` (an f32 lane word saturates at 2^24 by itself);
  - `rest = (z' ≥ N) & |n1| < R & |n2| < R`.
  - Simulator, four 10 Hz shelves, worst over every sparse pattern: N = 128 → 2.0e-9 (an impulse
    every 1,024 samples); N = 1,024 → 2.2e-10 (the A8 tail level); N = 65,536 → 1.8e-12.
  - The shelf trap still rests at sample 421,696, unchanged.
  - Placement option 1, per section inside `svf_step`: +3 lane-ops over A8, plus a third state
    word per section (the state payloads change, with more spill pressure).
  - Placement option 2, one counter per channel lane at each effect's input, ANDed with A8's
    per-section `x == 0`: this also removes mechanism 2. It costs about 4 ops per frame per channel
    per effect, plus one state word per channel.
  - Either way, a new gate is needed: `[a, 0, a, 0]` and `[a, 0, 0, 0]` at `a < L*` through four
    boosting shelves must get every boost. It is red on A8.
  - A decay-based law ("zero only if the state is not growing") is not recommended. Second-order
    states do not decay monotonically, and the costs (+6 ops) and bound are not derivable.

## MINOR

- **m1 (open for root): the cost is real and above the 2 % allowance.** The process A8 item 4
  asked for was followed: the cost was reported, with no workaround.
  - Codegen, attempt 3 → attempt 4, per SVF section-step:
    - V8: +1 `vcmpps (eq)`, +1 `vpand`, and about +1 stack reload. The zero vector is a spilled
      loop invariant, `[rbp-0x40]`, reloaded per use.
    - Held loops: mono tail 60→63, mono pair 93→99, masked pair 103→109, dual tail 123→132
      instructions. Dual pair 218→240, carried slots 13→17.
    - AVX2: +`vcmpeqps`, +`vandps`, and a `vxorps` zero idiom (free at rename).
  - The SVF loops therefore grow by 5-7 % in instructions, which is consistent with the +2.0-3.8 %
    p50 on EQ-heavy documents.
  - **No exact fold of the input test into an existing compare exists.** A bitwise fold of `x`
    into `|n|` admits tiny non-zero `x < REST_EPS`, which is a dead zone. Any exact fold needs an
    op that maps every non-zero `x` to at least `REST_EPS`, and that op itself costs one.
  - **One law-preserving saving exists elsewhere in `flush_pair`:** replace
    `lt(|n1|,R) & lt(|n2|,R)` (3 ops) with
    `lt(f32::from_bits(max_u32(bits|n1|, bits|n2|)), R)` (2 ops).
    - Then `flush_pair` = 12 (`abs`×2, `max_u32`, `lt`, `eq`, `and`, `lt`×2, `or`×2, `andnot`×2),
      `svf_step` 25, section 30/31, EQ 33, builtins 81, strip 325.
    - It is exact, NaN included, because NaN magnitude bits sort above +inf. I checked it on
      2.0e8 pairs, including the specials.
    - It needs a new `Lane` op with per-width gates: `vpmaxud` / `i32x4.max_u` / `umax`, one
      instruction on each target.
- **m2.** "At most 1.9e-13 at one +24 dB shelf's output" is a sampled maximum, not a bound. An
  impulse at 1e-10 through the 10 Hz S 0.1 shelf gives 2.41e-13 (−252.4 dBFS). The record's sweep
  skipped 1e-10. Say "largest measured".

## NIT

- **n1.** The audit `unfused_fma` model's new `v0` term is untested. With it dropped,
  `audit unfused-fma conformance` still reports 0 mismatches, because its input never puts a tiny
  non-zero sample on a small pair. The twin's term is held: builtins T2 goes red.
- **n2.** The chain gate's test-value line says "gates 1-2 stay green". That is true, but g2, g4
  and builtins T2 also go red on attempt 3's law. The chain gate's own catch is an effect-level
  defect: an EQ that zeroes near-silent blocks (|x| < 1e-10) is caught semantically only by it.
- **n3.** Under A8, `lane_is_inert` keeps refusing elision for a dead section that froze a
  sub-`REST_EPS` pair, until an executed block has an exact-zero frame. With continuous music this
  can be indefinite. It costs time only, and the doc states it.

## Judgments on the points asked

1. **The law.**
   - **`v0` threading:** correct at every caller, because `svf_step` passes its own `v0`. That
     covers `svf_block`, the ramped, masked and dry kernels (a dry lane still runs the recurrence
     on its section input), the interleaved and skewed cascades, all five builtin fused or elided
     bodies, and the LR4 second stage (`lp1`).
   - **Threshold:** exact zero is right. An SVF or one-pole output in a tail is normal or exactly
     zero, never subnormal, so a silent source reaches each section as ±0.
   - **Gates 1-2:** green, and the rest samples are unchanged. Impulse-then-zeros trajectories are
     identical under the A3 and A8 laws.
   - **Twin, g2 oracle and audit model:** they agree with the kernel. The twin and the oracle are
     held by tests; the audit model is not (n1).
2. **EQ predicates.**
   - `lane_is_inert` keeps the pair term, and this is necessary. Dropping it turns
     `randomized_restores` (W1 seed 1, block 10), `a_non_inert_state_in_a_dead_section_refuses_elision`
     and `ramping_elision` red, so the generator reaches the band under A8.
   - Dropping the pair term from `lane_is_flush_shaped` is sound. The `-0.0` induction uses only
     the per-word shape. Re-adding the term turns `leg_c_refuses_below_flush_eps_admits_it_and_re_engages`
     and `an_elided_cascade_is_the_full_cascade_bit_for_bit` red.
   - An elided inert identity section equals an executed one for every admitted `v0`, zero or not.
3. **Re-measure:** see M1.
4. **Cost:** see m1. The floor counts are verified:
   - `flush_pair` 2+4+1+2+2+2 = 13, so `svf_step` 26, section 31/32, EQ 34 (1.149), `active`
     table `31a+3` (195 for six), builtins 83 (2.804), the 61 control, and strip 328 (11.081).
   - The jq file, floor.rs and test-console-benchmark.sh agree.
5. **M2 and NITs.**
   - Mutation rows 1328-M1..M3 are reproduced exactly:
     - identity copy dropped: debug red; release red at block 27 on the right plane;
     - dry copy dropped: debug only.
   - The `filters.md` frequency is now 3.84 Hz, and the "sank" wording is qualified in both places.
   - Decision 15's entry points to A8, but its sentence carries M1's false claim.
6. **Gates:** all green (table below).
7. **GitHub:** the #1328 body equals the spec at 8a06cbd22 byte for byte (67,665 bytes). The issue
   is OPEN.

## Test value

- **`exact_rest::a_tiny_input_through_four_boosting_shelves_gets_every_boost`:** an EQ that drops
  a tiny continuous signal's boost turns it red. That covers attempt 3's input-blind flush (red in
  dev and release: "misses the ordinary run by 1.5091782e3") and an effect-level near-silence
  shortcut, which no other EQ test catches semantically. Gate 2 stays green on both.
- **g4 `pair_law_holds` / `pair_law_cases`:** a pair rule that fires on a non-zero input turns them
  red. Both the dropped input term and a gate widened to `|x| < MIN_POSITIVE` do.
- **`g2_svf_step_yields_both_taps_of_one_state`:** an `svf_step` whose flush ignores its input
  turns it red (:677).
- **`mono_collapse::a_desymmetrized_bank_carries_…`:** a stale `identity` copy is now red in
  release too. A stale `dry` copy is red in debug only, as recorded.
- **`leg_c_refuses_below_flush_eps_admits_it_and_re_engages` and the elision seeds:** a leg (c)
  that refuses kernel-written sub-`REST_EPS` pairs turns them red.

## Gates run (export at 8a06cbd22)

| gate | result |
|---|---|
| gate-4 debug `cargo test --all-targets` (14 packages, spec features) | 836 passed, 0 failed |
| release `-p lane -p math -p wasm-gates` | ok |
| release `--all-targets -p parametric-eq -p builtins -p dsp-reference` (test-support) | ok |
| `run-wasm-gates.sh` | exit 0. Held rows clean (132 / 99 / 63 / 109 instructions); dual pair 240 instructions, 17 slots; shipped module `086293e4…` |
| worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` | all ok |
| `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files), `audit unfused-fma conformance` (0 mismatches) | ok |
| `check-graph-determinism.sh` | 100/100 |
| `check-cross-targets.sh` | PASS, no ratchet change |
| `check-lane-policy.sh`, `check-dsp-research.sh`, `check-workspace-policy.sh` | ok |
| `cargo test -p bench floor`, `test-console-benchmark.sh` | ok |
| `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings` | ok |

Mutations (each applied in `.../mut`, run, and restored from `git show 8a06cbd22:<path>`):
- **attempt 3's law** (input term dropped), debug: red on the chain gate, g4 ×2, g2 and builtins
  T2. Everything else in lane, parametric-eq, builtins, dsp-reference and multiband is green
  (407 pass). Release: chain gate red.
- **`|x| < MIN_POSITIVE`:** g4 ×2 red.
- **twin input term dropped:** builtins T2 red. **audit model input term dropped:** no catch (n1).
- **`lane_is_inert` pair term dropped:** 3 red. **leg (c) pair term re-added:** 2 red.
- **`desymmetrize`, identity copy dropped:** debug red, release red (block 27). **dry copy
  dropped:** debug red, release green.
- **EQ zeroes blocks below 1e-10:** chain gate red, plus two bank-versus-scalar differentials.
