# Derive the live input section's tail gain and state its composition

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order: slice B2 of *Define how node tails compose through gain in the graph
extent* (#1379), Amendment 1 (H1, H3's live row; root ruling 4 split the live slice in two). Code
anchors verified on `main` at `7e8379523` and on `codex/d15-stream-g2` at `f3956e63c`; re-verify
every anchor at start.

## Product outcome

An input section with a live input lane states all five composition values,
`CompositionBound::Stated { decay, peak_gain, tail_gain, peak_stall, tail_stall }` (#1484 split the
stall). This slice derives the one B1 does not certify, the tail gain `G_t`: the gain that the section applies to an input arriving at or after the last
control event, which #1379 uses to carry an upstream residual through the section. `G_t` is
derived, never set to `G_p`; a live section's `G_p` covers state that a pre-`N` history builds and
a retarget exposes, so the two differ by tens of dB. No reported tail moves until #1379, and no
rendered bit moves.

## Context

- **`D`, `G_p` and the two stalls** of the live section are computed and exposed through accessors
  by slice B1 (#1466): `sigma_p`, the peak stall (every frame, (N1)), and `sigma_t`, the tail stall
  (from the node's tail on, (N2)); the live bound still states `Unstated` (no partial statement).
- **The live cascade** (#1433, `crates/math/src/tail.rs`) bounds the state by zone (`Phi`) over the
  65-frame window and every settled group; `builtins::input_section_live_bound`
  (`crates/builtins/src/tail.rs:211`) returns the bound.
- **The live words.** A live retarget moves the recursion only through designs and their mixtures,
  within 64 half-ulps plus `u D` per word of their convex hull, and completes by `N + 64` (#1407);
  the trim stays inside its endpoints, at most +24 dB (#1408).
- **The registry rule** of slice A1 (#1464): a `Stated` composition must have `tail_gain <= peak_gain`.

## The contract for `G_t` (#1379 Amendment 1, H1)

`eps = 10^(-144/20)`; `N` the first sample of silence; no control event at or after `N`;
`g_t = 10^(G_t/2000)`. Under that condition, if `|x[n]| <= X` for all `n` and `|x[n]| <= epsilon`
for every `n >= M` (some `M >= N`), then for every `k >= 0` and every `n >= M + L + T + k D`:
`|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma_t`. `G_t` bounds, at every `n >= M`, the part of
the output that the input from `M` on produces (the reference's response to that part, plus the
rounding deviation it drives), for every admitted history before `N`; where the section's state is
exactly zero at `M`, that part is the whole output. The deviation is bounded by the sum of its
drives, so the decomposition at `M` is linear.

## Decisions frozen for this slice

- **L-D6. `G_t` derived, never `G_p`.** For every admitted history before `N` and every `M >= N`: a
  bound on the output part that the input from `M` on produces. That input builds state during at
  most 64 ramp frames (words in the convex hull of the designs plus #1407's allowance), then meets
  the settled design's response, over every settled group, plus the deviation it drives, with the
  trim word of +24 dB. Rounded up to millibels with `math::log`.
- **L-D7. State all five, in millibels** (root's ruling of 2026-10-08: the millibel rounding moves
  here; #1466's L-D2 and L-D3 state raw linear values). `input_section_live_bound` returns
  `CompositionBound::Stated` with `decay` B1's `D`, and with each gain and stall rounded UP to
  millibels by the module's `ceil_mB` (`math::log`; `docs/derivations/1379-graph-tail-composition.md`,
  "Numerical limits"): `peak_gain = ceil_mB(G_p)`, `tail_gain = ceil_mB(G_t)` (this slice's),
  `peak_stall = Level(ceil_mB(sigma_p))` and `tail_stall = Level(ceil_mB(sigma_t))`, `G_p`,
  `sigma_p` and `sigma_t` as B1 (#1466) computes them raw. `G_t <= G_p` holds (else the slice stops
  and reports: the registry rule would refuse it). `sigma_t <= sigma_p` holds too, raw and in
  millibels (else the slice stops and reports), and this slice gates it itself (L5'): rule (g) runs
  only in the effect registry (`NativeEffectRegistry::new`), and an input section is not a registry
  row.
- **L-D8. Cost** within #1457's D1 budget.

## DSP evidence (AGENTS.md)

- **Equations:** the linear decomposition at `M` and L-D6, on the TPT SVF cascade under #1407's
  live words.
- **Coefficient and update rules:** #1407 and #1408, unchanged.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; `T`, `T_rest`, both rests, `D`, `G_p`, `sigma_p` and `sigma_t` unchanged.
- **Units and smoothing:** samples at the plan's rate; gains in millibels.
- **Denormal/NaN:** unchanged.
- **Citations:** as #1329 and #1433.
- **Fixtures and objective tests:** L2, L3', L5', L6'. **Benchmarks:** #1457's gates (L6').
  **Listening:** none; no rendered bit moves.

## Deliverables

1. `math::tail`'s live cascade: `G_t`, with its accessor.
2. `builtins`: the live `Stated` composition.
3. The derivation note's live part for `G_t`, in full
   (`docs/derivations/1379-graph-tail-composition.md`).
4. Gates L2, L3', L5', L6', and the mutation table.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions, stream
  A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live part), `docs/EFFECT_CONTRACT_V1.md`
  (the live section's statement)
- The browser's memory fixture (the exact `memoryBytes` pin #1433 names), only if the live bound's
  peak allocation moves, with its reason
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Non-goals

- Changing B1's `D`, `G_p`, `sigma_p` or `sigma_t`. Fixed designs (A2). Any graph or report change (#1379).

## Hazards

- **`G_t` is not the settled design's response alone.** In the bound, the state that a post-`N`
  input builds during the ramp window exceeds the settled designs' response: the window term and
  its settled continuation (about +92.4 dB with the trim) lie 44.4 dB above the settled input's term
  (+48.0 dB), so a `G_t` from the settled designs alone is not the derived bound, and L3' is red for
  it. (Root's amendment of 2026-10-08 restates this sentence as a property of the bound; the earlier
  text, "The state that a post-`N` input builds during the ramp window can exceed it; L2 is built so
  that this mutant is red", is superseded: on the real kernel no drive exceeds the settled pair's
  `l1`, attempt 1's stop.)
- **The `f32` deviation does not split linearly between two runs**, so `G_t` is not measured as a
  difference of two renders: L2 starts from an exactly zero state, where the whole output is the
  part `G_t` bounds.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

- **L2. Live tail gain against the exact supremum (root's amendment of 2026-10-08).** A test-only
  adjoint oracle in `tail_contract.rs` computes the exact row-`l1` supremum of the time-varying
  live cascade over every input `|x| <= 1` from `N` on, with zero state at `N` (the backward
  recursion, in `f64`, over the words the real kernel loads frame by frame, recorded from
  `InputBuiltins`: #1407's rules and the kernel's event timing; attempt 2), for a stated scan of
  histories; the stated `G_t` must be at least the trim word of +24 dB times that supremum, on every
  scanned history, at every launch rate. The oracle checks itself against the real `f32` kernel. Red mutant: a `G_t` below the supremum (trim and window both omitted). The
  attempt record states the scan, what it covers, and the mutation runs. (Superseded text, the
  first L2: "Live tail gain on the real kernel. A real live section, trim +24 dB, both sections
  designed at the worst-case pair (`input_section_worst_case_pair(rate)`), whose state is exactly
  zero (it has rested; the gate checks it). The HPF target is moved from its maximum toward 10 Hz
  at `N - 32` (also `N - 63` and `N - 1`) with zero input, then from `N` an input of peak `1e-6`:
  alternating during the ramp window, then the worst-sign pattern of the settled 10 Hz design.
  Every output sample from `N` on is at most `g_t 1e-6 + sigma_p` (an every-frame bound, so the
  peak stall). The drive is chosen so that the "settled design only" mutant below is red; the slice
  records the drive and the margin." No drive can make that mutant red; attempt 1's stop.)
- **L3'. Independent recomputation.** `live_bound_carries_every_term_an_independent_recomputation_requires`
  gains `G_t`: it lies between a plain-`f64` recomputation of L-D6 in the test and that value plus
  1 mB.
- **L5'. The statement.** `input_section_live_bound(rate)` returns `Stated` with `decay`,
  `peak_gain`, `peak_stall` and `tail_stall` equal to B1's `D`, `G_p`, `sigma_p` and `sigma_t` (its
  accessors, rounded up to millibels) and `tail_gain` equal to this slice's, and
  `tail_gain <= peak_gain` and `tail_stall <= peak_stall` (rule (g), gated here because the
  registry never sees this bound), at every launch rate. Every existing `tail_contract` assertion passes
  unchanged; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm G5 digests, the builtins
  PCM fixtures).
- **L6'. Budget.** #1457's gates 2 and 8 pass after this change, within #1457's D1 budget (#1457's
  gate 2 and gate 8 commands (#1457 Amendment 3: gate 2 is `cargo build --locked --release -p
  builtins --features test-support --example input_bound_budget` and one invocation of `taskset -c
  <core> target/release/examples/input_bound_budget`, descriptive; gate 8 is `cargo test --locked -p
  builtins-compiler --features test-support --lib
  every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`)).
- **Commands:** as slice B1.

The attempt record carries a mutation table with at least the mutants below; each is red.

## Test value

(Restated by the attempt under root's amendment of 2026-10-08; each claim is a mutation run in the
Attempt record. The earlier text is quoted and superseded at the end of this section.)

- L2: a derivation error that puts `G_t` below the exact supremum of the late input's part on the
  scanned histories, mirrored into the recomputation so that L3' agrees with it (trim and window
  both omitted, mirrored: `g_t` 15.87 against `trim x supremum` up to 60.17), is red only here. It
  does not defend the window term: a `G_t` from the settled designs alone (+48.0 dB) stays 12.4 dB
  above the scan's largest supremum (+35.6 dB). Its kernel check (attempt 2) is red for an oracle
  whose words are not the kernel's: a disable or an enable modelled as the all-six mixture, or the
  words read one frame late; the soundness assertion alone stays green for those (56.8 dB margin).
- L3': a `G_t` that drops the window term (settled input only), the window's settled continuation,
  the settled input's term, or the window input's feedthrough, or a second-section mix row that
  omits the low-pass row (`first_mix_row` in its place), leaves the recomputation's 1 mB band or
  its mix-row check; for a dropped window term L3' is the only red gate (L2 and L5' stay green, the
  table test aside, which pins any changed value and is restated with a deliberate change). A
  dropped settled group is red only when it is the group that sets the continuation's supremum
  (the other groups do not set `W*`), and dropping the window frames' own outputs is an equivalent
  mutant at the launch rates (the continuation is larger there).
- L5': a statement that crosses `peak_gain` and `tail_gain`, sets `tail_gain = peak_gain`, or
  swaps the stalls disagrees with the accessors rounded up; a composition whose tail stall lies
  above its peak stall (raw, within one millibel) is red at its raw order; nothing else checks the
  statement before #1379 reads it, except the table test's pin.
- The statement's unit test (`crates/builtins/src/tail.rs`): a check that refuses equal gains or
  equal stalls (`>=` for `>`) or a statement without the check is red; it pins the equality
  boundary, which no launch value reaches (the stalls are 54.6-61.3 dB apart, the gains 51.7 dB).

Superseded text: "L2: a `G_t` from the settled designs alone (the ramp window omitted) is beaten by
the state a post-`N` input builds during the window; B1's L1 drives a pre-`N` history and cannot
separate `G_t` from `G_p`." (false on the kernel; attempt 1's stop) and "L3': a `G_t` that drops a
ramp-window term or a settled group falls below the recomputation; the real kernel's slack cannot
see it." (a dropped settled group is red only when it binds).

## Amendment (root, 2026-10-08)

Root's ruling on attempt 1's stop (L2's test value cannot be met on the real kernel):

1. **L2 becomes a soundness gate against a committed adjoint oracle.** The exact row-`l1`
   supremum of the time-varying live cascade over inputs `|x| <= 1` from `N` on, computed by the
   backward (adjoint) recursion with #1407's word mixtures, lives as a test-only oracle in
   `crates/builtins/tests/tail_contract.rs`. The stated `G_t` must be at least that exact
   supremum (times the trim word) over the scanned ramp histories, at every launch rate. Red
   mutant: a `G_t` below the supremum (trim and window both omitted), with its mutation run. The
   record states the scan precisely (which ramps, which start offsets, which designs), why it
   covers the ramp space it claims, and claims no more than it covers. The record also states that
   L3' is the unique catch for a dropped window term, with its mutation run. The old L2 text
   ("Live tail gain on the real kernel ...") is superseded (Objective gates, L2).
2. **The first Hazard's second sentence** is restated as a property of the bound, not of the
   kernel (Hazards).
3. **The live `G_t` tightening** (the window term's looseness against the kernel's supremum) is
   folded into #1485, not this slice. This slice does not edit #1485.

Standing rulings applied: the derivation and the composition are written, with the full proof,
before the code (`docs/derivations/1379-graph-tail-composition.md`, "The live tail gain `G_t` and
the statement"); this slice gates `sigma_t <= sigma_p` raw and in millibels with a red mutant and
pins the equality boundary where it is admissible (the statement's own check, which admits
equality); every test-value claim is true and proved by a mutation run; byte re-pins are
individual, each with its reason; render-owned memory carries no control-only data and no parallel
API variant is added.

## Dependencies

- *Certify the live input section's decay, peak gain and flush stall* (slice B1, #1466).
- *Carry every node's tail bound in one node-neutral struct* (slice A1, #1464): the carrier and the
  registry rule.
- *Tighten the cascade exact-rest bound with a frequency-aware cascade analysis* (#1433, passed),
  *Retarget a live input filter only through its designs and their mixtures* (#1407), *Keep every
  trim, fader and matrix ramp inside its endpoints* (#1408).
- *Cache design bounds across preparations within a stated preparation budget* (#1457).
- Named exceptions: `STREAMS.md` (#1379 Amendment 1, H8).

## Attempt record

### Root ruling, 2026-10-08 (applied to L-D7)

The millibel rounding (`ceil_mB`, rounded up) of both stalls and of both gains moves into this
slice's L-D7; #1466's L-D2 and L-D3 state raw linear values, and this slice states the millibel
composition. L-D7 above is edited accordingly. (The GitHub body sync is handled separately.)

### Attempt 1 (2026-10-08, implementer): STOPPED before code, spec defect in L2

Worktree `codex/d15-stream-g3` at `ff9bdbc28`. Anchors re-verified: `input_section_live_bound`
(`crates/builtins/src/tail.rs:489`, `Unstated` through `bound_from_cascade`),
`input_section_live_bound_table` (`:505`, `Unstated`, held equal to the computed bound by
`live_bound_table_is_the_computed_live_bound_at_every_launch_rate`), `LiveComposition` /
`live_cascade_composition` (`crates/math/src/tail.rs:622`, `:1949`), #1466's L1/L3/L4 gates in
`tail_contract.rs`. No code was written: L2's stated test value cannot be met, so the slice stops
as the brief requires.

**The defect.** L2 (and the Test value and the first Hazard) require a real-kernel drive that turns
the mutant "`G_t` from the settled designs alone (the ramp window omitted)" red. No drive can. The
largest output any input from `N` on can give at frame `n` of a word sequence is the row `l1`
`sum_m |h(n, m)|` of the time-varying system with zero state at `N` (an exact supremum over every
input with `|x| <= 1`). Computed in `f64` by a backward (adjoint) recursion over the cascade's
4-state step, with #1407's mixtures as linear word interpolation (a scratch program outside the
tree, not committed), 44.1 kHz, worst-case pair, HPF moved from its maximum toward 10 Hz, before
the trim:

| ramp start | sup over `n <= 30,000` | at `n = 300,000` (window share) |
|---|---|---|
| `N - 1` | 3.3021 | 3.5287 (`1.1e-9`) |
| `N - 32` | 3.3021 | 3.5287 (`5.3e-10`) |
| `N - 63` | — | 3.5287 (`1.7e-11`) |

3.5287 is the settled pair's own `l1` (10 Hz HPF into the top LPF; #1379 H9's 3.51). The input that
arrives during the window contributes at most 1.94 at any frame and has decayed to `1e-9` by the
time the settled response reaches its `l1`; the supremum over every drive equals the settled
design's `l1`. So even a mutant equal to the *exact* settled `l1` of the very design L2 settles to
is never beaten, and any sound settled-only value (at least the supremum of the settled `l1` over
every design, with its own slack) is further above. A wider scan (HPF and LPF each ramping between
the top, 1 kHz, 10 Hz, identity and top-to-8..21.9 kHz designs, ramp start `N - 1` to `N - 48`)
gives a window-only row `l1` of at most 2.63 (LPF top to 21.5 kHz), always below 3.5287. The
`f32` deviation is about `1e-6` relative and changes nothing. The state a post-`N` input builds in
at most 64 ramp frames is real in a *bound* that ignores frequency, not on the kernel.

**What a sound `G_t` from #1433's machinery measures** (per unit of the input's peak, trim word of
+24 dB included; a scratch test, not committed): the window term with zero state at `M` and 65
frames of any reachable word is about +108.3 dB with the frequency-blind second-section sum and
+88.5 dB with #1433's potential (65 frame charges `K` and direct-zone charges); the settled term
(the per-zone section bound `(1/2 |m| q_k + omega) beta_k / (1 - r_k) + delta`, squared for the
cascade) is +48.0 dB, at every launch rate. So a derived `G_t` is about 88.5 dB, 56 dB below
`G_p` (144.2 dB) and 53.5 dB above the exact supremum (+35.0 dB = 3.5287 times the trim); the
"settled only" mutant (+48 dB) stays 13 dB above anything the kernel can produce.

**What root must decide (proposals, not applied):**
1. Rewrite or drop L2. With `G_t` about +88.5 dB and the kernel's supremum +35.0 dB, L2 can only
   turn red for a defect that loses more than about 53.5 dB: dropping the window term alone
   (+48 dB left) or the trim alone (+64.5 dB left) stays green; dropping both (+24 dB) is red. No
   single plausible defect is known that L2 alone catches, so under AGENTS.md's test-value rule
   L2 either names a mutant confirmed red by a run or is removed; the window-term mutant is
   defended by L3' only (its own test value already says so).
2. Delete the first Hazard's second sentence ("The state that a post-`N` input builds during the
   ramp window can exceed it; L2 is built so that this mutant is red") or restate it as a property
   of the bound, not of the kernel.
3. Optionally record the expected magnitudes above in the Product outcome ("differ by tens of dB"
   holds: about 56 dB) and note the window term's looseness (about 53 dB over the kernel's
   supremum) as a tightening follow-up beside #1485.

### Attempt 1, restarted under root's amendment (2026-10-08, implementer)

Root's amendment is recorded above ("Amendment (root, 2026-10-08)"); the old L2 text and the first
Hazard's second sentence are kept and marked superseded where they stood. Anchors re-verified on
`codex/d15-stream-g3` at `215dbfda5`: as in the stop record above.

**Proof first.** `docs/derivations/1379-graph-tail-composition.md`, new section "The live tail gain
`G_t` and the statement (issue #1467, slice B2)", written before the code: (P3) the decomposition
at `M` is linear at the kernel level (each relative rounding perturbation split in proportion to
the parts' own drives, the flush part to its own part; every #1433 per-frame inequality then holds
for each part, so the late part's supremum is what `G_t` must bound); (W) the window input's part
(the late input on `M ..= M + 63`): a finite-horizon zone bound `Phi^(j)` from zero state, the
first section's output `max_k (1/2 |m| q_k + omega) Phi^(j)_k`, the second section
frequency-blind, then #1433's relative settled system from `M + 65` (the same 65-frame window)
stepped until it falls, per group; (Z) the settled input's part (from `M + 64`, words fixed):
`L_1 L_2`, each section bounded by its settled design's zone with its own mix row; `G_t =
ceil_mB((g W* + g L_1 L_2) SLACK)`; the statement with its own `tail_gain <= peak_gain` and rule
(g) check on the stated millibels (equality admissible).

**Changed.**
- `crates/math/src/tail.rs`: `LiveComposition::tail_gain`; `LiveCascade::second_mix_row` and
  `second_output_rounding` (the LPF's mix row is not covered by `first_mix_row`: with the `f32`
  word `k`, `||(-k, -1)||_V*` is `2.4e-8` below `||(0, 1)||_V* = sqrt(2)`); `LiveBound::
  arrival_window`, `arrival_continuation`, `settled_input_gain`; `composition` computes the tail
  gain last (after every existing refusal; a capped arrival window refuses too). No other value
  moves: `T`, the rests, `P*`, `D`, `G_p`, `sigma_p`, `sigma_t` are bit-identical (L3, L4 and the
  table's tail and rest entries unchanged).
- `crates/builtins/src/tail.rs`: the envelope's two new fields; `live_stated_composition` (L-D7)
  and `live_composition`; `input_section_live_bound` states it; `input_section_live_bound_table`
  re-pinned (below); a unit test of the statement's check.
- `crates/builtins/tests/tail_contract.rs`: L2 (`live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`,
  with its oracle `exact_row_supremum`, `forward_response`, `free_response_bounds`), L3' (`live_oracle`
  recomputes `G_t` and checks the module's second mix row; the recomputation test compares `G_t`),
  L5' (`live_bound_states_the_composition_rounded_up_with_its_orders`).
- `docs/derivations/1379-graph-tail-composition.md` (the B2 section, the introduction, B1's
  pointers, the gates); `docs/EFFECT_CONTRACT_V1.md` (the live section states the five values).
- `STREAMS.md` has no status column for this row; unchanged. #1485 not edited.

**Re-pins** (individual): `input_section_live_bound_table`'s `composition` at each launch rate,
`Unstated` to `Stated { decay, peak_gain, tail_gain, peak_stall, tail_stall }` = 44.1 kHz
`(46,678; 14,424, 9,242, -23,379, -28,841)`, 48 kHz `(46,421; 14,416, 9,242, -23,312, -28,846)`,
88.2 kHz `(46,678; 14,426, 9,242, -22,765, -28,840)`, 96 kHz `(46,421; 14,417, 9,242, -22,696,
-28,828)`. Reason: this slice states the live composition (L-D7); the table test holds the table
equal to the computed bound. The tail and rest entries do not move. No byte pin moves: the
statement changes a value of an existing field (`NodeTailBound` is unchanged in size); the browser
expected resources check passed unchanged (`builtinRetainedBytes` not touched; source budget 3,358
of 3,648); the memory fixture is not touched.

**Values** (release; `mB` stated): `g_t` raw 4.174511e4 / 4.174672e4 / 4.175823e4 / 4.175878e4
(+92.41 / +92.41 / +92.42 / +92.42 dB), `G_t = 9,242` mB at every rate, 51.7-51.8 dB below `G_p`.
Parts (L3' recomputation, 44.1 kHz): window frames 2.7809e4 (+88.9 dB), continuation 4.1494e4
(+92.36 dB, sets `W*` at every rate), settled input `g L_1 L_2` 2.5126e2 (+48.0 dB; `L_1 = L_2 =
3.98`). Cost: `live_cascade_composition` 0.46-0.76 ms per rate (one probe run); preparation never
computes it.

**Gates.**
- L2 (release, every launch rate). The scan, per rate (336 histories): (a) a ramp of the HPF
  between every ordered pair of `{10 Hz, 100 Hz, 1 kHz, 10 kHz, the worst-case HPF, the identity}`
  with the LPF at the maximum, and of the LPF between every ordered pair of `{10 Hz, 100 Hz, 1 kHz,
  10 kHz, the maximum, the identity}` with the HPF at 10 Hz (60 ramps); (b) each with its control
  event at `N - s`, `s` in `{1, 16, 32, 48, 62}` (62, 47, 31, 15, 1 ramp frames after `N`; the word
  at `N - s + i` the exact mixture with weight `min(i + 1, 64) / 64`); (c) the 36 settled pairs.
  What it covers: those histories, in exact arithmetic, one section moving at a time, from the
  given endpoints, every input `|x| <= 1` from `N` (so every `M >= N`), every frame (exact rows
  until both rigorous remainders are below `1e-12`, then the partial `l1` plus the remainders). It
  does not cover other endpoints, both sections moving, restarts, rule-3 resets, `f32` word
  rounding or the rounding deviation; it claims no more. Why these: the endpoints span the cutoff
  domain by decades plus its slowest designs and the identity (the disable/enable mixtures), and
  the offsets span the ramp's position at `N` from just started to one frame left. Results: the
  largest `trim x supremum` 60.12 (+35.58 dB, HPF 10 Hz to 100 Hz at `N - 1`, 44.1 kHz), 60.08 /
  59.62 / 59.62 at 48 / 88.2 / 96 kHz; `g_t` 4.1783e4 (the stated 9,242 mB); margin 56.84-56.91
  dB; largest remainder `1.1e-12`. Oracle self-checks: the settled 10 Hz into the maximum equals
  the impulse response's `l1` (3.5286663711 at 44.1 kHz, to `1e-9`), and on a ramp history the
  rows at `N + 40` and `N + 200` equal forward sums of unit-impulse responses through the kernel's
  equations (to `1e-12`). 13.5 s for the four rates; ignored in debug (release scale).
  (Attempt 2, verdict MAJOR 1: this coverage statement is false in two points. The identity
  endpoints were the all-six mixture, which is not #1407's disable (rule 2) or enable (rule 3),
  and every offset was one frame off the kernel's timing, so the event before `N - 1` was not
  scanned. Attempt 2 replaces the scan; see its record.)
- L3': `G_t` above the recomputation by `3.5e-8` relative at every rate (1 mB is `1.15e-4`); the
  module's second mix row at least the derivation's `sqrt(2) + mix_box`. #1466's L3 and L4
  unchanged and passing.
- L5': `Stated` at every launch rate, each value the accessor's rounded up; `G_t <= G_p` and
  `sigma_t <= sigma_p` raw and in mB. The statement's unit test: equal gains and stalls stated,
  a tail value one millibel above its peak value refused.
- L6': gate 8 passed. Gate 2: one invocation, `taskset -c 7`, after the 1-minute load fell below
  2 (`/proc/loadavg` before 1.87 2.62 2.52, after 1.68 2.54 2.50): exit 0, worst median design work
  24.950 ms, 97.2 % of 25.67 ms. Candidly: the printout was captured through `tail -8`, so the
  `ns needed` line was not kept; that figure is not of record (the batch verifier's is).
  (Corrected in attempt 2, verdict MINOR 1: this record said a following `--help` probe "may have
  started it a second time". It did not: `input_bound_budget` refuses any argument other than
  `calibrate` and exits 2 before any work.) This
  slice changes only live code in `math::tail` (the fixed walk is untouched), so L6' is a
  regression run and claims no unique catch.
- L5 (#1466's, nothing else moves): release `tail_contract` 22 passed; `audit capi`
  `pcm_digest` `cb10fbface44a3a4`, 0 allocations, 0 deallocations, 0 syscalls;
  `check-builtins-fixtures.sh` ok (50 files); `run-wasm-gates.sh` ok; the worklet chain
  (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) passed.
- The debug gate command (`lane`, `math`, `builtins`, `dsp-reference`, all targets) passed;
  `cargo test --release -p builtins-compiler --features test-support` and `-p host-core` passed;
  `check-workspace-policy.sh` ok; `check-cross-targets.sh` PASS (the #1018 expected failures);
  workspace clippy `-D warnings` clean; `cargo fmt --check` clean.

**Mutation table** (each defect applied, the release `tail_contract` run with every test (the
table test excluded for the M rows: it pins every stated value and is restated with any
deliberate change; M1t shows it), the files restored; "mirrored" means the same defect also in
the recomputation):

| mutant | red | green |
|---|---|---|
| M1 window part dropped (`tail_gain = g L_1 L_2`) | L3' only (`2.51e2` against `4.17e4`) | L2, L5', the rest |
| M1t M1 with the table test included | L3', the table test | the rest |
| M2 trim and window dropped, mirrored | L2 only (`g_t` 15.87 < 60.12) | L3' (mirrored), L5' |
| M2b trim and window dropped | L2, L3' | L5' |
| M3 settled input term dropped | L3' only (`4.149e4`) | L2, L5' |
| M4 settled continuation dropped | L3' only (`2.806e4`) | L2, L5' |
| M5 window frames' outputs dropped | none: equivalent at the launch rates (the continuation sets `W*`) | all |
| M6 the window input's feedthrough dropped | L3' only (`3.888e4`) | L2, L5' |
| M7 `second_mix_row = first_mix_row` (builtins) | L3' at its mix-row check | L2, L5' |
| M8a, M8b first or last settled group dropped | none: neither sets the continuation's supremum | all |
| M8c the binding settled group dropped | L3' only (`3.834e4`) | L2, L5' |
| S1 statement crosses `peak_gain` and `tail_gain` | L5', the table test | L2, L3' |
| S2 statement `tail_gain = peak_gain` | L5', the table test | L2, L3' |
| S3 statement swaps the stalls | L5', the table test | L2, L3' |
| S4 statement check strict (`>=`) | the statement's unit test (equality refused) | `tail_contract` |
| S5 statement check removed | the statement's unit test | `tail_contract` |
| S6 `math` stalls swapped | L3, L5' (`Unstated`), the table test, L2 (reads the statement) | the rest |
| S7 `math` `sigma_t = sigma_p (1 + 1e-5)` (same millibel) | L3's order assertion, L5' at its raw order, the table test | the rest |
| S8 `math` `tail_gain = peak_gain` | L3', the table test | L2, L5' |
| O1 the L2 oracle drops the ramp columns after the ramp | L2 at its forward-sum self-check | the rest |

**Test value** (as the section above states; measured): L2's unique catch is M2 (a mirrored
derivation error below the exact supremum); L3' is the only value-checking gate red for a dropped
window term (M1), and also for M3, M4, M6, M7 and M8c; L5' for S1-S3 (with the table pin); the
unit test for S4 and S5, which pin the equality boundary. M5, M8a and M8b are equivalent at the
launch configuration and claim nothing.

**Open items.**
- The live `G_t` (+92.4 dB) is about 56.8 dB above the scanned exact supremum (+35.6 dB): the
  second section's frequency-blind window sum and the tau system's `rho_settled` set `W*`. Its
  tightening is folded into #1485 (root's amendment); #1485 is not edited here.
- Gate 2's figure of record is the batch verifier's (see L6' above).

### Attempt 2 (2026-10-08, implementer): L2 reads the kernel's words; verdict fixes

Worktree `codex/d15-stream-g3` at `a1a742d36`. Attempt 1 (`ae054b277`) failed on MAJOR 1: L2's
stated coverage (the identity endpoints as #1407's disable and enable, the event offsets) was not
true. `G_t`, the proof and every value are unchanged; no frame-equivalent constant moves.

**Commits.** `8d994fa55` (the derivation, first; verdict NIT 1) and the code commit after it.

**Changed.**
- `crates/builtins/tests/tail_contract.rs` (L2, MAJOR 1). The oracle's words are now recorded from
  the real kernel: `record_history` builds an `InputBuiltins` at the start pair, applies the
  retargets with `apply_prepared_filter` before their frames, and reads the HPF's and LPF's words
  with `input_section_words` before each frame is processed, zero input throughout. So the oracle
  reads #1407's rules as the kernel runs them (rule 1 re-send; rule 2 disable: `c1`, `a2`, `a3`
  frozen, the mix ramped, then the identity and cleared integrators; rule 3 enable from rest: the
  recursion jumped, the mix ramped; rule 4 the all-six ramp, from in-flight words on a restart) and
  the kernel's timing (current, then advance: an event before frame `N - s` has `64 - s` frames in
  flight at `N`; before `N - 1`, weight 1/64 at `N` and the target at `N + 63`). A clear is read
  where a section's words become the identity (only a disable's completion writes it over other
  words) and is folded into the frame before it (`history_frames`); no row depends on it, because
  an identity section's state neither moves nor reaches the output (K3 below). The adjoint
  recursion is unchanged; the settled system's prefix `l1` is computed once per settled pair per
  thread (`SettledSum`), and the window's remainder is checked every 4,096 frames. The `mixture`
  function (with its false doc comment) is deleted: nothing uses it.
- New in L2: the oracle against the real `f32` kernel. On six recorded histories per rate (a
  retarget, an HPF disable, an HPF enable and an LPF disable before `N - 1`; an HPF disable before
  `N - 40` interrupted by a retarget before `N - 8`; an HPF enable before `N - 16` interrupted by a
  disable before `N - 15`), at `N + 40` and `N + 200`: the oracle's row equals the forward impulse
  sum (to `1e-12`), and the real kernel, trim +24 dB, driven from `N` in one block by
  `2^-12 sign h(n, m)`, gives `|y(N + n)|` within `1e-3` of `trim 2^-12 row`. Measured: ratios
  0.999997 to 1.000020 at every rate. The one-block drive also shows that the words recorded one
  frame per block are the words the kernel runs inside a block.
- `crates/math/src/tail.rs` (NIT 2): `arrival_window`'s three max-folds use `nan_max`, which
  carries a NaN to the window's cap (refused as capped) instead of dropping it as `f64::max` does.
  For non-NaN values it is `f64::max`, so no value moves (the table test passes unchanged). No test
  is added: I found no public input that reaches a NaN in these folds and is not refused first.
- `crates/builtins/src/tail.rs` (NIT 3): a one-line comment that turning an error into `Unstated`
  with `.ok()` is deliberate.
- `docs/derivations/1379-graph-tail-composition.md` (NIT 4, `8d994fa55`): the kernel's late-part
  supremum is close to the largest settled pair's `l1`, not the 10 Hz pair's. Scratch scan (not
  committed; a 61-point logarithmic grid of HPF cutoffs into the top LPF and a coarser grid of both
  cutoffs, `f64` impulse responses of the designed words): the largest settled `l1` is 3.7980 /
  3.7976 / 3.7980 / 3.7976 (+35.59 dB with the trim) at an HPF of 147.98 / 173.54 / 286.93 /
  341.86 Hz into the top LPF at 44.1 / 48 / 88.2 / 96 kHz; the 10 Hz HPF gives 3.5287 / 3.5077 /
  3.3727 / 3.3516. The gate summary says L2 scans histories recorded from the real kernel.
- This spec: L2's gate text and test value; the attempt-1 record's false `--help` sentence
  (MINOR 1, corrected in place) and a note on its scan statement.

**L2's scan and coverage** (release, every launch rate, 3,213 histories per rate). The cutoffs
`{the identity, 10 Hz, 100 Hz, 200 Hz, 500 Hz, 1 kHz, 10 kHz, one f32 below the maximum, the
maximum}`, the HPF below the LPF when both are enabled:
- (a) 1,792: one section retargeted once between every ordered pair of the cutoffs (a disable, an
  enable from rest, or an all-six ramp), the other fixed (LPF at the maximum or the identity; HPF
  at 10 Hz or the identity), the event before `N - s`, `s` in `{1, 2, 8, 16, 32, 48, 63}`;
- (b) 45: every settled pair of the cutoffs;
- (c) 800: one section retargeted twice, `a -> b` then `-> c`, on `{the identity, 10 Hz, 200 Hz,
  1 kHz, one f32 below the maximum}` (HPF, LPF at the maximum) or `{the identity, 100 Hz, 1 kHz,
  10 kHz, the maximum}` (LPF, HPF at 10 Hz), `a != b`, `c = b` a re-send (rule 1), the events
  before `(N - 63, N - 1)`, `(N - 40, N - 8)`, `(N - 16, N - 15)`, `(N - 2, N - 1)`;
- (d) 576: both sections retargeted, the HPF on `{the identity, 10 Hz, 200 Hz, 1 kHz}`, the LPF on
  `{the identity, 10 kHz, one f32 below the maximum, the maximum}`, the events before
  `(N - 1, N - 1)`, `(N - 1, N - 63)`, `(N - 63, N - 1)`, `(N - 32, N - 8)`.

Per rate: 622 histories with a disable that completes after `N`, 622 with an enable from rest,
3,168 with a retarget in flight at `N`. Rules covered: 1 (re-send), 2 (disable, its completion and
clear), 3 (enable from rest), 4 (all-six ramp, from settled and from in-flight words, a frozen
disable included). What it covers: those histories only, with the kernel's own `f32` words and
timing, the input at `N` and later in exact arithmetic (every input `|x| <= 1` from `N`, so every
`M >= N`), every frame (exact rows until the window's remainder is below `1e-12`, then the settled
prefix `l1` plus both rigorous remainders). It does not cover other cutoffs or other offsets, a
retarget at or after `N`, the trim ramp, or the rounding deviation (B1's part); it claims no more.
L2's cutoffs step over the fine grid's settled maximum (3.7980 against the scan's 3.7966 at 48 kHz,
0.003 dB); no bound depends on that.

**Results.** The largest `trim x supremum`: 44.1 kHz 60.1642 (+35.59 dB; HPF identity -> 200 Hz
and LPF maximum -> one `f32` below, both before `N - 1`), 48 kHz 60.1721 (+35.59 dB; HPF identity
-> one `f32` below the maximum before `N - 2` -> 200 Hz before `N - 1`), 88.2 kHz 60.1204 (+35.58
dB), 96 kHz 60.1203 (+35.58 dB; HPF identity -> 500 Hz before `N - 1`). `g_t` 4.178304e4 (the
stated 9,242 mB, +92.42 dB); margin 56.83 / 56.83 / 56.84 / 56.84 dB. Largest remainder
`1.2e-12`. Self-checks: the settled 10 Hz into the maximum equals the impulse response's `l1`
(3.5286663711 at 44.1 kHz, to `1e-9`); the rows and the kernel as above. 9 to 10 s for the four
rates (one thread per rate, six per scan). The verifier's independent oracle (forward impulse
columns over recorded words, about 3,960 histories a rate) found at most 60.17, the same.

**Mutation runs** (each applied, the release `tail_contract` run, the files restored; M2 with every
test except the table test, the others L2 alone, since they change only L2's code):

| mutant | red | green |
|---|---|---|
| M2 trim and window dropped, mirrored (`math` `tail_gain = L_1 L_2 SLACK^2`, L3' the same) | L2 only (`g_t` 15.87; first failing history 55.93) | the other 20 |
| O1 the oracle's window part after the changing frames zeroed | L2 at its forward-sum check (`N + 200`: 2.1796 against 2.1934 at 44.1 kHz) | - |
| K1 a single retarget modelled as the all-six mixture (the kernel's timing) | L2 at its kernel check (HPF disable, `N + 40`: kernel 4.750e-3, oracle 4.520e-3 at 44.1 kHz, 4.8 % under) | L2's soundness assertion |
| K1b only an enable from rest modelled as the all-six mixture | L2 at its kernel check (HPF enable, `N + 40`: 5.439e-3 against 4.448e-3, 18 % under) | L2's soundness assertion |
| K2 the words read one frame late (after the frame is processed) | L2 at its kernel check (retarget, `N + 40`: 4.953e-3 against 4.989e-3) | L2's soundness assertion |
| K3 the clears not folded in | none: equivalent (an identity section's state neither moves nor reaches the output) | all |

So the modelling matters to L2 through its kernel check, not through its soundness assertion: a
mixture model of a disable or an enable under-reports the kernel's row (K1, K1b), and a one-frame
timing error moves it (K2); with 56.8 dB of margin, no modelling mutation turns the soundness
assertion itself red. M2 stays L2's unique catch; O1 stays red at the forward-sum check.

**Gates.**
- `cargo test --locked --release -p builtins --features builtins/test-support --test
  tail_contract`: 22 passed (9.9 s), the table test unchanged; debug: 20 passed, 2 ignored
  (release scale).
- `cargo test --locked -p math`: passed. `cargo test --locked -p builtins --features
  builtins/test-support --lib`: 16 passed.
- `cargo fmt --all -- --check` clean; `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings` clean.
- `check-workspace-policy.sh` ok; `check-realtime-policy.sh` ok (89 regions, 25 files);
  `check-cross-targets.sh` PASS (x86-64-v3; aarch64 iOS and Android checked and linted; #1018
  expected failures; wasm simd128).
- The worklet chain (`math` changed): `build-web-audioworklet.sh --named-twin` (fresh output
  directories), `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (source total 3,358 of 3,648, unchanged),
  `test-web-audioworklet.sh` (private TMPDIR, left empty): all exit 0.
- No render code changed: `audit capi`, the builtins fixtures, gate 2 and gate 8 were not re-run
  (attempt 1's and the verifier's runs stand; `nan_max` is value-neutral and control-plane only).

**Open items.** As attempt 1: the `G_t` looseness (56.8 dB) is #1485's.
