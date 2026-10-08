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

- **`G_t` is not the settled design's response alone.** The state that a post-`N` input builds
  during the ramp window can exceed it; L2 is built so that this mutant is red.
- **The `f32` deviation does not split linearly between two runs**, so `G_t` is not measured as a
  difference of two renders: L2 starts from an exactly zero state, where the whole output is the
  part `G_t` bounds.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

- **L2. Live tail gain on the real kernel.** A real live section, trim +24 dB, both sections
  designed at the worst-case pair (`input_section_worst_case_pair(rate)`), whose state is exactly
  zero (it has rested; the gate checks it). The HPF target is moved from its maximum toward 10 Hz at
  `N - 32` (also `N - 63` and `N - 1`) with zero input, then from `N` an input of peak `1e-6`:
  alternating during the ramp window, then the worst-sign pattern of the settled 10 Hz design. Every
  output sample from `N` on is at most `g_t 1e-6 + sigma_p` (an every-frame bound, so the peak
  stall). The drive is chosen so that the "settled design only" mutant below is red; the slice
  records the drive and the margin.
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

- L2: a `G_t` from the settled designs alone (the ramp window omitted) is beaten by the state a
  post-`N` input builds during the window; B1's L1 drives a pre-`N` history and cannot separate
  `G_t` from `G_p`.
- L3': a `G_t` that drops a ramp-window term or a settled group falls below the recomputation; the
  real kernel's slack cannot see it.
- L5': a statement that crosses `peak_gain` and `tail_gain`, or sets `tail_gain = peak_gain`,
  disagrees with the accessors; nothing else checks the statement before #1379 reads it.

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
