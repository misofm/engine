# State a bounded tail and an exact-rest bound for every node

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b), D15-4(c)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan's reported tail means one thing everywhere: after the input stops, the output stays below
-144 dB relative to the input's peak from a stated sample on. This slice defines that contract and
the exact-rest bound, and delivers both for the builtin input section. With it, a strip with
enabled input filters, or with a live input lane, reports a finite tail at its rate instead of
`Infinite`, so the C ABI and browser hosts can report bounded tails (#1261, #1262). Each effect
gets its own slice (see "Non-goals").

## Context

- **The type.** `TailSamples::{Finite(u64), Infinite}` (`crates/effect-contract/src/lib.rs:131-134`)
  has no stated meaning beyond "terminates". It is carried per quality descriptor
  (`QualityDescriptor::tail`, `:513-521`), composed by PDC with `Infinite` winning and finite
  tails added along a path after latency (`crates/graph-compiler/src/pdc.rs:105-137`), and capped by
  `maximum_finite_tail_samples` (`:132-135`). The C ABI reports it as `TAIL_FINITE`/`TAIL_INFINITE`
  plus samples (`crates/capi/src/abi.rs:36-39`, `crates/capi/src/runtime/compile.rs:649-650`). No
  render path reads it.
- **The builtins today.** `BuiltinTail::{FiniteZero, Infinite}` (`crates/builtins/src/lib.rs:224-227`).
  `InputBuiltins::tail` (`:3433-3445`) says `Infinite` while any HPF/LPF is enabled or a filter
  target is ramping. `BuiltinChain::tail` (`:3262-3264`) returns it. The compiler forces `Infinite`
  for every strip with a live input lane (#1254 D1; `crates/builtins-compiler/src/lib.rs:3256-3266`
  in `expected_tails`, and `:3560-3568`), and `graph-compiler` maps the two variants
  (`crates/graph-compiler/src/compile.rs:141-150`). `docs/BUILTINS_AND_METERING_V1.md:50-52` says
  "Enabled filters declare an infinite tail".
- **The input section.** Trim (`-144..=24` dB, `crates/builtins/src/lib.rs:3284`), polarity, then
  HPF then LPF, both Butterworth TPT SVF sections with fixed `k = sqrt(2)` designed by
  `SvfSection::design` (`:722-760`). Cutoff domain: 10 Hz (`crates/builtins/src/filter_control.rs:41-42`)
  to `builtin_filter_cutoff_maximum_hz` (`lib.rs:322-330`), or disabled. Live filter targets ramp
  their words over `INPUT_FILTER_RAMP_SAMPLES = 64` (`filter_control.rs:9`).
- **Evidence (decision 15, round 1 B4 and round 2 B4(a)).** The exact-zero tail is infinite without
  *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328). Worst case over the
  reachable domain, -144 dB re input peak: 383,571 samples at 44.1/88.2 kHz, 381,428 at 48/96 kHz
  (slowest pole at the *maximum* cutoff, radius 1 - 5.2e-5). With #1328, exact rest at +24 dBFS:
  ≤ 934,193 (44.1/88.2 kHz), ≤ 929,225 (48/96 kHz); any sanitized input: ≤ 2.27M. D15-4 states the
  contract figures 1.0M and 2.4M; Amendment 2 (F2) supersedes these estimates and figures with the
  certified values plus `2 * N_SILENCE`, which Amendment 3 (G4) states: about 1.27M and 2.59M.
  Every step matrix of these sections is non-expansive in the 2-norm (bilinear map of a passive `M`).
- **Tests that pin today's rule:** `input_tail_is_infinite_while_a_filter_target_is_ramping`
  (`crates/builtins/tests/filter_liveness.rs:313-336`); `builtins-compiler` unit tests at
  `crates/builtins-compiler/src/lib.rs:11885`, `:11915`, `:11931`; `crates/host-core/tests/live_lanes.rs:202-211`
  and `:251-265` (`Infinite` under `HostLiveLanes::ALL`). Canonical plan text carries tails:
  `fixtures/graph/v1/direct-route.canonical.txt:4,59` (`infinite`) and the digest
  `ZERO_DELAY_CANONICAL_SHA256` (`crates/graph-compiler/tests/track_delay.rs:244-266`).

## Decisions frozen for this slice

- **D1. Tail.** `TailSamples::Finite(T)` means: for every input of peak `P > 0` that is zero from
  sample `N` on, under any control history the node admits before `N` (block-boundary targets
  under #1407's retarget law; a ramp may be in flight at `N` and completes by `N + 64`), and with no
  control event at or after `N`,
  `|y[n]| < P * 10^(-144/20)` for every `n >= N + latency + T`. `T` is counted beyond latency, as
  PDC already composes it. Document it on the type and in `docs/EFFECT_CONTRACT_V1.md`.
  `Infinite` keeps its meaning ("no bound stated") for nodes whose slice has not landed.
  *(Amended by Amendment 3, G1: the reported `T` is `T_decay`, the bound for every `P >= P*`; for
  `P < P*` the output is exactly `±0.0` from `N + latency + T_rest`, `T_rest = max(T_decay, R(P*))`.
  Both are stated.)*
- **D2. Exact rest.** New `pub struct RestSamples { pub peak_plus_24_dbfs: u64, pub any_sanitized_input: u64 }`
  in `effect-contract`: under D1's conditions, with input peak at most +24 dBFS (respectively below
  the input sanitizer's `1e30`), from `N + latency + R` on (a) every output sample is `+0.0` or
  `-0.0` (a polarity-inverted zero is `-0.0`), and (b) every *signal-state word* equals, under `f32`
  `==`, the same word of the node's *rest state* `Z`. `Z` is the state a freshly reset instance
  with the same parameters and settled ramps reaches after `R` zero input samples, and it must be a
  fixed point of the zero-input step (the derivation of each slice proves it; a node whose
  zero-input step has no reachable fixed point states no bound). Signal-state words are the words
  an input sample can reach (filter integrators, envelopes, gain smoothers, hold counters, rings
  and their running sums). Parameter words, ramp words, payload headers and ring cursors are not
  signal state. `Z` need not be zero: the true-peak limiter's rings rest at `1.0` and its box sum
  at `Wb`, which is its reset state (`crates/true-peak-limiter/src/lib.rs:624-643`); the gate
  resets open (`gain_db = 0`, `crates/gate-expander/src/lib.rs:504-515`) but rests closed, its gain
  word at the range floor. Documented beside `TailSamples`. For the builtin input section `Z` is
  the reset state: the eight SVF integrator words at `+0.0`.
- **D3. Certified, computed, never pinned.** Both values are computed on the control thread at
  preparation from the designed `f32` words (evaluated in `f64`), per rate. The reported value is a
  certified upper bound on D1's smallest `T`: the exact-arithmetic tail sum
  `S(j) = sum_{m >= j} |h[m]|` must fall below `eps / 2` (`eps = 10^(-144/20)`), and the derivation
  proves that the `f32` kernel's deviation, with #1328's flush, stays below the other `eps / 2`
  (rate inflation `rho_f = rho + 7 * 2^-24 * kappa` per Amendment 2, the stall radius, and exact rest below
  `REST_EPS`). `S` is summed exactly to a horizon `H` plus a closed-form remainder; for the HPF→LPF
  cascade the remainder uses the union bound
  `sum_{m>=H} |h| <= |h_L|_1 * S_H(ceil(H/2)) + S_L(ceil(H/2)) * |h_H|_1`. The derivation goes in
  `docs/derivations/1329-input-section-tail-and-rest.md`. *(Amended by Amendment 2, F1.)* The
  `f32` deviation half compares the `f32` kernel with a **reset-aware exact reference**: an
  exact-arithmetic run that resets its sections at the same instants as the kernel (the joint
  `REST_EPS` flush of #1328). The reference's states are bounded by triangle-inequality state-norm
  suffix sums that hold for every reset pattern: `W_H(k) = sum_{j >= k} ||G_H(j)||_V` for the HPF,
  and for the LPF `W̄_L` with kernel `Ḡ_L(j+1) = q_L Ḡ_L(j) + beta_L |h_H(j)|`. The exact half of the
  split is that reference (`S <= Ref`); the remainder past the horizon uses contraction on the
  actual state. The deviation from the reference then has only relative rounding (`rho_f`, D5) and
  the absolute per-word flush: at most `sqrt(2)*FLUSH_EPS` per section per step, accumulated to the
  stall radius `kappa*sqrt(2)*FLUSH_EPS/(1 - rho_f)` and carried through the section's output row.
  Per rate define `P*` from the per-word flush alone, as the smallest peak for which that term fits
  the deviation budget left after rounding. For `P >= P*` the bound is the sum; for `P < P*` the
  derivation proves exact rest (all integrator words `+0.0` via `REST_EPS`, plus the A9 term of
  Amendment 2) at or before `N + latency + T`. *(Amendment 3, G1: the bound for `P >= P*` is
  `T_decay`; the `T` of the `P < P*` branch is `T_rest = max(T_decay, R(P*))`.)* The flush is
  never modelled as relative error, and a joint-flush reset is never modelled as an absolute kick on
  an unreset reference. Gate 1(a)'s
  tightness line is checked against the bound computed with this reference.
- **D4. Fixed design (no live input lane).** `T` and `R` come from the strip's own designed sections
  (max over left and right), with the prepared trim as a gain on `P`. Disabled filters: `T = 0`,
  `R = 0` (trim and polarity are memoryless).
- **D5. Live input lane.** `pub fn input_section_live_bound(rate) -> (TailSamples, RestSamples)` in
  `builtins`: the bound over the whole cutoff domain (each section disabled or in `[10 Hz, max]`),
  trim +24 dB, any history of filter targets and their 64-sample ramps. All builtin sections share
  `k = sqrt(2)`, so every designed step matrix is the bilinear map of one fixed `M` and they share
  eigenvectors `V`. Under #1407 (its Amendment 1 and "Numerical limits"), every recursion word any
  history reaches is the identity with +0.0 integrators, or lies componentwise within the proven
  `f32` allowance `E = 64 h + u D = (1.967e-6, 9.716e-7, 1.967e-6)` (for `c1`, `a2`, `a3`, at
  every block size) of the convex hull of the designs the history used, so
  `||A(w)||_V <= q_design + 1.419e-5`. Over the whole domain the largest design norm is the
  maximum-cutoff design's, `q - 1 = -5.213e-5` (44.1 and 88.2 kHz) and `-5.241e-5` (48 and
  96 kHz), so `q_ramp = max ||A(w)||_V <= 1 - 3.794e-5` (44.1, 88.2 kHz) and `1 - 3.822e-5` (48,
  96 kHz); with the kernel inflation `7*2^-24*kappa` (*amended by Amendment 2*: the rigorous
  rounding count of the SVF step and output mix, where the final rounding of `n1`/`n2` adds
  `u*q*kappa`, gives `mu_s = 1.007e-6` at the top), `q_ramp + mu_s < 1` with at least `3.69e-5`
  (measured) to spare at every launch rate (*restated by Amendment 3, G4*: certified margin
  `3.673e-5` at 44.1 and 88.2 kHz, `3.701e-5` at 48 and 96 kHz). The derivation uses this
  computed `q_ramp`, not a sampled maximum. The state at N is bounded by the invariant ball `R = g *
  max_w b(w)/(1 - q(w))`, with `b`
  the V-norm of the input column. *(Amended by Amendment 2, F2.)* That per-section ball covers the
  HPF only: the LPF's input is the HPF's output, not `g * P`. The cascade uses the crude-but-sound
  bound: the HPF's universal output ball `Y_H` (about `7.7e4 * g * P` after a top-to-low retarget)
  is the LPF's input, giving the LPF's ball `B_L` (about `4.2e9 * g * P`). Rest is proven in
  sequence: the HPF rests first (its input is exactly zero in the tail; stall at most `3.6e-16`),
  then the LPF (while the HPF is not at rest, its universal stall gives the LPF a stall radius
  `3.8e-11`, far above `REST_EPS`). Every exact-rest bound carries the A9 term `+2 * N_SILENCE`
  (Amendment 2). The free response starts at N + 64. `g = 10^(24/20)`, valid by
  #1408. Production
  evaluates the closed form at the worst-case pair below (round 1: HPF one ulp below the maximum
  into LPF at the maximum, both channels). #1262's gate 4 enables this pair live:

  | rate | HPF (Hz, bits) | LPF (Hz, bits) |
  |---|---|---|
  | 44,100 | 22049.48046875 `0x46ac42f6` | 22049.482421875 `0x46ac42f7` |
  | 48,000 | 23999.431640625 `0x46bb7edd` | 23999.43359375 `0x46bb7ede` |
  | 88,200 | 44098.9609375 `0x472c42f6` | 44098.96484375 `0x472c42f7` |
  | 96,000 | 47998.86328125 `0x473b7edd` | 47998.8671875 `0x473b7ede` |

  Expose it as `pub fn input_section_worst_case_pair(rate) -> Option<(f32, f32)>` (`None` off the
  launch rates). Gate 1(b) confirms the pair. If the scan finds a worse one, this table and the
  function change in this slice; #1262 reads the function, not the numbers. If the derivation
  cannot prove a finite bound, the slice stops and reports it to Sol; there is no fallback value.
- **D6. Gain-only parts** (trim, polarity, fader, mute, matrix) report `T = 0`, `R = 0` beyond their
  latency (0).
- **D7. One value per plan.** `BuiltinTail` is deleted. `InputBuiltins::tail()` returns `TailSamples`
  and a new `InputBuiltins::rest()` returns `RestSamples`, both computed once at preparation and
  stored. The dynamic "ramping ⇒ `Infinite`" rule (`lib.rs:3433-3445`) goes: a live target cannot
  change a plan's tail, because D5 already covers it. `builtins-compiler` replaces both
  forced-`Infinite` sites with `input_section_live_bound(rate)`: the seal check's `expected_tails`
  (`crates/builtins-compiler/src/lib.rs:3261-3267`) and preparation (`:3562-3568`). Both hosts
  prepare builtins through this crate (`crates/host-core/src/prepare.rs` for the C ABI,
  `hosts/host-web/src/lib.rs` for the browser), so both report the bound for every strip with a
  live input lane. `graph-compiler` takes `TailSamples` directly. Render reads
  neither value; nothing is computed or allocated on the render thread.
  *(Amendment 5, MJ1:)* preparation computes a design bound only for a strip without a live
  input lane; a live strip's design bound is never computed.
- **D8. Composition is unchanged.** PDC still adds node tails along a path. Gain in other nodes is
  not folded into a node's floor; the graph extent's meaning across gain, and the tail term of a
  strip's `delay_samples` line (a pure delay, not latency), are
  *Define how node tails compose through gain in the graph extent* (#1379).

## Amendment 1 (root decisions, 2026-10-05, after attempt 1's blocker)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b). Attempt 1 stopped
before code: under today's live filter retarget law the reachable recursion words leave the stable
set in `f32` (see "Attempt record"), and a long trim ramp overshoots the trim domain, so D5 had no
finite bound to prove. The root approved option (m), the live filter retarget law, as
*Retarget a live input filter only through its designs and their mixtures* (#1407): every
reachable recursion word becomes a design or a mixture of designs, the #808 contract is restored,
and it adds no state, no latency and no sealed-size change. It also approved *Keep every trim,
fader and matrix ramp inside its endpoints* (#1408), which makes D5's `g = 10^(24/20)` valid. This
slice now follows both. D1 and D5, gates 1(b), 2, 5 and 6, the authorized paths and the
dependencies are amended accordingly; the D7 reach (finding 2) is added to the authorized paths,
and D3 states how the flush enters the `f32` deviation (finding 1). Rejected: holding a target until
its ramp completes, a budget argument, live filter lanes only at quanta of 64 or more, and
duplicate-resend removal alone (#1407 "Non-goals").

## Amendment 2 (root rulings, 2026-10-05, after attempt 2's findings)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b). Attempt 2 paused before
production code (exploration tests only) with two findings and a rate-inflation correction; see
"Attempt record". D3, D5, gates 1(a), 2 and 3, the deliverables, the authorized paths, the
non-goals, the hazards and the dependencies are amended accordingly.

- **F1. D3 uses a reset-aware exact reference.** D3's exact/`f32` split is unsound once the joint
  rest acts: a reset removes state, so the HPF can rest while the LPF runs free, the H/L
  cancellation is lost, and repeated rests before `N` leave no absolute bound. Modelling a reset as
  an absolute `REST_EPS` kick gives `P*` about `1e-10` and a rest-at-`P*` tail about 510k, which
  fails gate 1(a)'s tightness (451,007 at 0 dB, 507,413 at +24 dB). Ruling: the exact half is an
  exact reference that resets at the same instants as the kernel, bounded by triangle-inequality
  state-norm suffix sums valid for every reset pattern (`W_H(k) = sum ||G_H(j)||_V`; `W̄_L` with
  kernel `Ḡ_L(j+1) = q_L Ḡ_L(j) + beta_L |h_H(j)|`); `P*` comes from the per-word flush alone; gate
  1(a)'s tightness line is checked against that reference. Measured with it: state-based bound
  1.4-2.3x `S` at `T`; certified `T` 8k-16k above `T_b(eps/2)` and inside `T_b(eps/32)`: pair
  404,077 (0 dB) / 459,255 (+24 dB); LPF at the maximum 344,462 / 397,443; 1 kHz-2 kHz pair 184 /
  211 against 205 / 232.
- **F2. Option (iii): ship the crude-but-sound cascade bound now.** D5's per-section ball does not
  cover the cascade (the LPF's input is the HPF's output). The crude bound (`Y_H` about
  `7.7e4 * g * P`, `B_L` about `4.2e9 * g * P`) gives certified `R(+24 dBFS)` about 1.19M and any
  sanitized input about 2.5M, over the earlier 1.0M and 2.4M figures, and `T_live` about 640k.
  Ruling: ship it; restate the D15-4 figures to the certified values plus the A9 term
  (Deliverable 7); prove rest in sequence, HPF first, then LPF. Reason: a sound, looser bound is
  correct; tightening it only shortens reported tails, which is a cost and not a correctness gap,
  so it belongs in a successor issue (#1433) rather than being a placeholder here. Measured real
  rest of the top pair (HPF maximum minus one ulp into LPF maximum, +24 dBFS, before A9): 983,374
  samples at 44.1 kHz and 978,319 at 48 kHz.
- **The A9 term.** #1328's Amendment A9 lets a section flush only after its effect input has been
  exactly zero for `N_SILENCE` frames (1024 at every launch rate unless #1328's measurement states
  otherwise). Every exact-rest bound (`peak_plus_24_dbfs`, `any_sanitized_input`, the `P < P*`
  branch of `T`) and the gate 2/3 figures gain `+N_SILENCE` per section in sequence: the HPF rests,
  then the LPF's input is exactly zero for up to `N_SILENCE` more frames, so `+2 * N_SILENCE` for
  the HPF→LPF pair. The tail bound `T` itself does not depend on it.
- **Rate inflation.** D5 is restated to `7 * 2^-24 * kappa` (`mu_s = 1.007e-6` at the top; the
  final rounding of `n1`/`n2` adds `u * q * kappa`, which `6 * 2^-24 * kappa = 8.63e-7` missed), and
  the margin to the measured value, at least `3.69e-5` (D5's "at least `3.70e-5`" was slightly
  optimistic). `q_ramp + mu_s < 1` still holds at every launch rate.
- **Dependencies.** This slice now also depends on #1328 passing with A9.

## Amendment 3 (root rulings, 2026-10-06, after attempt 3's findings)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b). Attempt 3 stopped before
plumbing: the real kernel exceeds gate 1(a)'s tightness line at small peaks (F3), and D3's `P < P*`
branch makes the certified tail at least the rest bound at `P*`, which includes A9's `N_SILENCE`
(F4); see "Attempt record". Root chose option 1. D1, D3, D5's figures, gates 1(a), 2 and 3, a new
gate 1(c) and a new gate 7, the deliverables, the authorized paths, the non-goals, the hazards and
the test value are amended accordingly. This is still attempt 3 (no verdict yet).

- **G1. Two tail values (option 1).** Every node states both, computed at preparation (D3):
  - `T_decay`, the decay part: the exact half below `eps / 2` (D3's reset-aware reference) and
    the relative rounding below the other half from then on. It is D1's bound for every input of
    peak `P >= P*`. Gate 1(a)'s soundness and tightness lines apply to `T_decay`.
  - `T_rest = max(T_decay, R(P*))`, at least `N_SILENCE` for every enabled filter section (A9).
    From `N + latency + T_rest` on, for every peak `P > 0`, the output is below `P * eps`
    (`P >= P*`, by `T_decay`) or exactly `±0.0` (`P < P*`, D3's exact-rest branch at `P*`): it is
    D1's bound over every peak, the value D3's `T` was before this amendment. Exact zero; the work
    can be skipped.
  - Disabled filters and gain-only parts: both `0` (D4, D6).
  - Each consumer names which value it uses: **tail reporting uses `T_decay`** (`TailSamples` as
    composed by PDC, the C ABI and browser reports, #1261, #1262); ~~silence skipping uses
    `T_rest` (#1107)~~ *(replaced by the addendum, A2: silence skipping uses D2's `RestSamples`;
    `T_rest` is its low-peak case)*. `TailSamples::Finite` carries `T_decay`, and its doc and
    `docs/EFFECT_CONTRACT_V1.md` state the peak range `P >= P*` and the exact-zero branch from
    `T_rest` below it. `T_rest` is stored and exposed beside `T_decay` wherever `T_decay` is
    (`InputBuiltins`, `BuiltinChain`, `input_section_live_bound`; *Amendment 4, R5:*
    `PreparedBuiltinsSession` and `input_section_live_bound`); the implementer names the field
    or accessor and states it in `docs/EFFECT_CONTRACT_V1.md`. D2's `RestSamples` is unchanged.
- **G2. Option 2 refused.** An absolute floor (`|y[n]| < max(P * eps, eps_abs)`) makes the tail a
  fixed-level one: below `eps_abs` whatever the input's peak. #1328's A8 and A9 removed exactly
  that (an effect applies its response to whatever it is given; only exact zero is silence), so
  the contract does not bring it back through the tail.
- **G3. Option 3 moved to #1433 as a candidate.** A two-level `P*` (the HPF rests first, then only
  the LPF's own stall applies) would bring the cascade rows of attempt 3's table back near the
  decay part, though every enabled section keeps `T_rest >= N_SILENCE`. It is a tightening, not a
  correctness change, so it is recorded in *Tighten the cascade exact-rest bound with a
  frequency-aware cascade analysis* (#1433) as a candidate, not done here.
- **G4. Figures restated to the certified, unsampled values** (attempt 3's module; D5's crude
  cascade, ramp window 64, settled contraction after it, each `R` including `2 * N_SILENCE`), at
  44.1 / 48 / 88.2 / 96 kHz:

  | live bound | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz | D15-4 figure |
  |---|---|---|---|---|---|
  | `peak_plus_24_dbfs` | 1,264,736 | 1,257,840 | 1,268,585 | 1,262,029 | about 1.27M |
  | `any_sanitized_input` | 2,583,197 | 2,569,016 | 2,587,000 | 2,573,156 | about 2.59M |
  | `T_decay` (expected) | 904,785 | 899,524 | 904,795 | 899,533 | -- |
  | `T_rest` (expected) | 1,081,764 | 1,075,575 | 1,085,641 | 1,079,792 | -- |

  These replace Amendment 2's "about 1.19M and 2.5M" and D15-4's "≤1.0M"; the `T` rows are attempt
  3's measured outputs of the module, recorded as evidence, not pinned. The certified contraction
  margin `1 - rho_ramp` (exact design radius `rho(g)` at the maximum cutoff, plus the `f32` design
  box `P(h) = 2.16e-7`, the ramp allowance `P(E) = 1.419e-5` and the state rounding
  `mu_state = 1.0073e-6`) is **`3.673e-5` (44.1, 88.2 kHz) and `3.701e-5` (48, 96 kHz)**; it
  replaces D5's and Amendment 2's "at least `3.69e-5` (measured)", which used the `f32` top
  design's norm.
- **G5. The 2.6M hazard.** Root's ruling asks to name the limit the "below 2.6M" of "Hazards"
  stands for and to gate its headroom. The documents name no limit at 2.6M: the figure entered
  "Hazards" with Amendment 2 as the envelope of this slice's values (then about 2.5M for
  `any_sanitized_input`), beside the one limit the hazard does name, `maximum_finite_tail_samples`
  (`crates/graph-compiler/src/pdc.rs:132-135`), which compares composed `TailSamples` (`T_decay`
  after G1), not `RestSamples`, and which the hosts set to `u64::MAX` and `graph_fixture`, the audit
  tools and `console-workload` to 10,000,000. Which limit, if any, 2.6M was meant to protect is
  open for root. Gate 7 gates the headroom to the limits the documents name and records the
  envelope's headroom (`any_sanitized_input` is 13,000 samples, 0.5 %, below 2.6M at 88.2 kHz).
  *(Closed by the addendum, A1: no limit stands at 2.6M.)*

## Amendment 3 addendum (root answers, 2026-10-06)

Root's answers to the implementer's questions after Amendment 3, binding on this attempt. They
amend G1's consumer line, G5, "Hazards" and gate 7.

- **A1. No 2.6M limit.** The phrase came from Amendment 2's value range; no limit stands behind it.
  The 2.6M check is dropped. Gate 7 checks `T_decay` and the exact-rest bound against the real
  configured limit, `maximum_finite_tail_samples` = 10,000,000 (`graph_fixture`, the audit tools,
  `console-workload`), and records the headroom. G5's open question is closed.
- **A2. Silence skipping uses `RestSamples`.** #1107 uses D2's `RestSamples`, the exact-rest bound
  valid for every input. `T_rest = max(T_decay, R(P*))` is only the low-peak case: the time to
  exact zero for an input of peak `P < P*` (for `P >= P*` it is the decay bound, not exact zero).
  G1's "silence skipping uses `T_rest`" is replaced by this, and #1107's spec records it.
- **A3. The name.** The implementer names the exposed `T_rest` consistently with `RestSamples` and
  the existing `TailSamples`: no version suffix, no ad-hoc abbreviation. The chosen name is stated
  in `docs/EFFECT_CONTRACT_V1.md` and in the attempt record.
- **A4. Hot-file order.** `docs/handoffs/decision-15-2026-10-05/STREAMS.md` lists #1455 in the
  multiband `run_segment` hot-file row: after #1409, and in either order with #1338 (neither
  depends on the other; the later slice rebases).

## Amendment 4 (root rulings, 2026-10-06, after attempt 3's verdict)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b). Attempt 3 failed
adversarial review (`/home/bl/misofm/submix-verdicts/1329-attempt3.md`): one MAJOR finding (M1,
the fixed-design certificate's `f64` operator norm cancels and falls below the exact norm by up to
`6.4x` its inflation), three MINOR findings (m1 gate 2 not run at the domain extreme and its `1e29`
sign-pattern run vacuous through the non-finite recovery; m2 the live `T_rest` untested; m3
derivation and research-note errors), two NITs, and eight items for root (ROOT-1 to ROOT-8). Root
ruled on the eight items; attempt 4 fixes M1, m1-m3 and the NITs under these rulings. D7, G1's
storage line, Deliverables 2 and 3, gates 2, 5 and 6, the authorized paths and the test value are
amended accordingly.

- **R1. Outside-path edits.** `tools/audit/src/builtins_graph.rs` (the joined-corpus manifest
  identity that moves with `resources.jsonl`), `fixtures/builtins/v1/MANIFEST.tsv` and
  `fixtures/graph/MANIFEST.tsv` (generator companions of the authorized regenerations) are
  ratified. `crates/builtins/src/filter_response.rs` is **reverted** to its pre-#1329 content: it
  was not needed under the shipped design.
- **R2. Gate 5 names `maximum_single_allocation_bytes`.** Attempt 3 moved it (+72 bytes per track,
  the strip vector) where gate 5 named only the `engine_owned_*` counts. Gate 5 now names it, with
  the delta measured after R5 and recorded in the attempt record; `resources.jsonl` is re-pinned
  once more after R5 and every moved size is audited individually with its reason.
- **R3. Every test-value claim is true.** Gate 2's claim (a bound that ignores `f32` rounding, the
  ramp in flight or the flush stall), gate 3's lower side (a dropped A9 term) and gate 7's claim
  each get a discriminating test that is red on that defect; a claim is amended only where no
  discriminating test is possible, and the amendment says why.
- **R4. Gate 2's `-0.0` wording.** "(one run with polarity inverted, so `-0.0` is reached)" asks
  for what the node cannot produce: every enabled section's output mix, and the identity's
  trailing `+0.0`, normalize a `-0.0`. Gate 2 states what the node produces (below).
- **R5. The bounds live beside `tails` in `PreparedBuiltinsSession`.** The prepared bounds are
  control-side data: `PreparedBuiltinsSession` keeps every strip's `T_decay`, `T_rest` and
  `RestSamples` beside its tail (#1107 reads them there). `InputBuiltins` and `BuiltinChain` carry
  no bound and no accessor for one; there is no lazy, allocating accessor (the `OnceCell` path
  goes). Render-owned memory carries no control-only data. This replaces D7's "`InputBuiltins::tail()`
  ... and a new `InputBuiltins::rest()` ..., both computed once at preparation and stored" and G1's
  "stored and exposed beside `T_decay` wherever `T_decay` is (`InputBuiltins`, `BuiltinChain`,
  `input_section_live_bound`)": the owners are now `PreparedBuiltinsSession` (per strip, as
  reported) and `input_section_live_bound` (the live bound).
- **R6. CI runs `tail_contract` at release scale.** `.github/workflows/qualification.yml` gains one
  step in the required `test-release` job: `cargo test --locked --release -p builtins --features
  builtins/test-support --test tail_contract`, as #1428 added `filter_liveness`.
  `scripts/check-ci-path-routing.py` stays green (its router expectations and the verdict table
  change only if it needs them).
- **R7. Preparation cost.** Each distinct design's bound is computed once per preparation and
  reused for the seal check (`validate_for_session`); nothing is computed twice. The worst-case
  preparation cost is measured and recorded. A stream G issue, *Cache design bounds across
  preparations within a stated preparation budget*, is filed for the cross-preparation cache and
  its budget (it notes that stream H's #1332 moves browser preparation off the AudioWorklet
  thread).
- **R8. D2 across a swap.** A plan swap that carries non-zero integrators into a disabled section
  leaves them frozen (#1407 rule 4: the identity recursion keeps them and the output ignores them),
  so D2's rest is not reached there. *Swap a rebuilt plan without an audio gap* (#1269) records
  this and must state whether a carried state counts as admitted history; a root-authorized edit to
  #1269's spec and its GitHub body.

## Amendment 5 (root rulings, 2026-10-06, after attempt 4's verdict)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b). Attempt 4 failed
adversarial review (`/home/bl/misofm/submix-verdicts/1329-attempt4.md`): one MAJOR finding (MJ1,
preparation computes every strip's design bound and discards it for a strip whose input lane is
live, about +75 % per 64-track browser boot, where every strip is live *(corrected by Follow-up
C: true only of a browser boot with live controls; an audio-only browser boot attaches no input
lane)*), three MINOR findings
(m1 the seal check's live-strip rule untested; m2 the shared-designs test blind to the right
channel; m3 one false test-value line), five NITs and items ROOT-A to ROOT-F. Attempt 5 is the
last permitted attempt; its scope is exactly the list below, and nothing in `crates/math` or the
certified figures moves. D7, the authorized paths and the test value are amended accordingly.

- **MJ1 (in #1329's scope: a regression this slice introduced).** Preparation computes design
  bounds only for strips without a live input lane. The seal, the seal check, `input_bounds()` and
  every fixture are unchanged. A counted test (a test-support counter of design-bound
  computations) is zero when every strip is live and exactly the number of distinct designs when
  none is, and is red on attempt 4's behaviour. The browser boot is re-measured with
  `scripts/web-mixing-automation-benchmark.mjs rebuild-round MODULE 1` and `2` (the #1289
  rebuild-cost proxy) for the head module against pre-#1329 `0725a8949`'s, with the target within
  noise of pre-#1329.
- ~~**Root ruling A.** The browser computes no design bound; #1457's worst case is the C ABI's
  control thread. #1457's "Where preparation runs" is amended to say so.~~ *(Corrected by
  Follow-up C's root correction below: a browser boot with live controls computes no design bound,
  but an audio-only browser boot, the SDK default, bounds every distinct design, and #1457's
  budget binds both hosts.)*
- **m1.** A builtins-compiler unit test forges a live strip's entry to its design bound in both
  `tails` and `seal.tails` and expects `builtin.prepared.tail_set` from `validate_for_session`;
  red when the seal check skips its live rule.
- **m2.** `each_strip_is_bounded_by_its_own_design_when_designs_are_shared` gains a strip that
  differs from the first only in its right channel; red on a key that drops the right channel.
- **m3.** The live `T_rest` identity's test-value line is restated truthfully (below).
- **NITs.** The `nu` proof text in `math::tail` and the derivation's "Numerical limits" state the
  `sqrt(2)` and the sound argument (doc text only); `STEP_UP`'s doc states the per-summand
  argument (the constant stays `1 + 4 u` where a correct argument exists); the attempt-4 record's
  module digest is marked as `feefbe166`'s, and the final digest is recorded at the batch boundary
  (**root ruling D**); the superseded gate 2, 3 and 7 test-value lines are marked as superseded
  with a pointer to their restatements (**root ruling C**); #1457's STREAMS row reference is
  corrected.
- **Root ruling B.** `crates/host-core/src/prepare.rs`'s `HostLiveLanes::strip_input` doc (stale
  since attempt 3: a live input lane no longer makes the strip's builtin tail infinite) states what
  the lane does now.
- **Root ruling E.** `docs/rulings/effect-floor-accounting.md`'s citation of the removed
  `InputBuiltins::tail()` is updated, marked as a citation update, not a ruling change.

## Deliverables

1. `effect-contract`: D1 docs on `TailSamples`, the `RestSamples` type (D2).
2. `crates/math/src/tail.rs` (new, `pub mod tail`): the certified `f64` bounds for TPT SVF
   sections and their cascades, control-plane only, so the EQ and the multiband crossover reuse
   them. It lives in `math`, not `lane`: `lane` is `no_std` with `wide` as its only dependency
   (`crates/lane/Cargo.toml`), so it has no `f64` logarithm, while `math` exports `log`/`exp`
   (`crates/math/src/lib.rs:68`, `:86`) and `builtins`, `parametric-eq` and
   `multiband-compressor` already depend on it (`effect-runtime` is not a `builtins` dependency).
   It uses `math::log`/`math::exp`, never `f64::ln`/`f64::exp`;
   `builtins`: `crates/builtins/src/tail.rs` (new) composing them for the input section;
   `input_section_live_bound`, `input_section_worst_case_pair`; `InputBuiltins::{tail, rest}` and `BuiltinChain::{tail, rest}`.
   *(Amendment 3, G1:)* every owner of `T_decay` also states `T_rest`.
   *(Amendment 4, R5:)* the per-strip owner is `PreparedBuiltinsSession` (beside `tails`), not
   `InputBuiltins` or `BuiltinChain`; `math::tail`'s operator norm is a certified upper bound with
   its own `f64` rounding bounded explicitly (M1).
3. Plumbing: `builtins-compiler` and `graph-compiler` on `TailSamples`; `BuiltinTail` removed.
   *(Amendment 4, R5, R7:)* `builtins-compiler` computes each distinct design's bound once per
   preparation, keeps every strip's bounds beside its tail, and its seal check reuses them.
   *(Amendment 4, R6:)* the release `tail_contract` step in `qualification.yml`'s `test-release`.
4. Tests (gates 1-3); superseded tests replaced (gate 4).
5. Docs: `docs/EFFECT_CONTRACT_V1.md` (with Amendment 3's two values and which consumer uses each),
   `docs/BUILTINS_AND_METERING_V1.md:50-52`,
   `dsp-research/filters.md` "Latency and tail", the derivation note (D3, D5) with equations,
   numerical limits, NaN behaviour (a non-finite state is reset by the per-block check, so it never
   starts a tail) and citations ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP], [SMITH-SASP];
   the W3C Web Audio `tailTime` notion as the external analogue).
6. Canonical plan digests re-pinned one by one (gate 5), with the reason in the commit message.
7. *(Amendment 2, F2.)* The #1329 implementer restates decision 15's D15-4 rationale figures
   (currently "≤1.0M samples at +24 dBFS") to the certified values plus `2 * N_SILENCE`, on this
   branch (root-authorized). *(Amendment 3, G4: done with Amendment 3 on this branch: about 1.27M
   and 2.59M, the per-rate values, the certified margin, and the `T_decay`/`T_rest` definitions in
   D15-4(b). If the implemented values differ from G4's table, the implementer restates them
   again.)*

## Authorized paths

- `crates/effect-contract/src/lib.rs` (the tail types and docs only), `docs/EFFECT_CONTRACT_V1.md`
- `crates/math/src/tail.rs` (new), `crates/math/src/lib.rs` (module line)
- `crates/builtins/src/lib.rs`, `crates/builtins/src/tail.rs` (new), `crates/builtins/tests/tail_contract.rs` (new),
  `crates/builtins/tests/filter_liveness.rs`
- `crates/builtins-compiler/src/lib.rs` (tail rule, tail type, its unit tests)
- `crates/graph-compiler/src/compile.rs` (the tail mapping), `crates/graph-compiler/src/lib.rs`
  (the unit test `builtins_replace_only_the_three_internal_track_bindings`, whose `Infinite`
  assertions at `:14130` and `:14175` come from the fixture's input HPF),
  `crates/graph-compiler/tests/track_delay.rs` (the digest), `fixtures/graph/v1/direct-route.*`
  (regenerated by `crates/graph-compiler/src/bin/graph_fixture.rs`)
- `crates/host-core/tests/live_lanes.rs` (the tail assertions)
- `tools/audit/src/fixture_builtins.rs`; `fixtures/builtins/v1/resources.jsonl` (regenerated by
  `audit fixture-builtins`); `crates/builtins-compiler/tests/metered_preparation.rs`;
  `crates/graph-compiler/tests/MUTATIONS.md`; `docs/rulings/builtins-input-liveness-d2.md`
- `docs/BUILTINS_AND_METERING_V1.md`, `dsp-research/filters.md`,
  `docs/derivations/1329-input-section-tail-and-rest.md` (new), this spec
- `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (the D15-4
  rationale figures and, after Amendment 3, its tail and rest definitions only; Deliverable 7,
  root-authorized)
- *(Amendment 4.)* `tools/audit/src/builtins_graph.rs` (the manifest identity),
  `fixtures/builtins/v1/MANIFEST.tsv`, `fixtures/graph/MANIFEST.tsv` (R1, ratified);
  `crates/builtins/src/filter_response.rs` (R1, the revert only); `.github/workflows/qualification.yml`
  (R6, one `test-release` step) and `scripts/check-ci-path-routing.py` (only if R6 needs it);
  `.github/ISSUE_SPECS/1269-swap-a-rebuilt-plan-without-an-audio-gap.md` (R8, the D2-across-swap
  statement only); the R7 issue's new spec in `.github/ISSUE_SPECS/` and its row under Stream G
  in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

- *(Amendment 5.)* `crates/host-core/src/prepare.rs` (the `HostLiveLanes::strip_input` doc only,
  root ruling B); `docs/rulings/effect-floor-accounting.md` (the citation only, root ruling E);
  `.github/ISSUE_SPECS/1457-cache-design-bounds-across-preparations-within-a-stated-preparation-budget.md`
  ("Where preparation runs", root ruling A)

`builtins-compiler`, `graph-compiler`, `host-core` and `fixtures` sit outside stream G's column:
coordinate with stream F (#1261, #1262 edit `builtins-compiler`).

## Non-goals

- Any effect's tail or rest, the composition rule, and retiring `Infinite`. Each is its own slice:
  - First: *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377). The
    per-effect slices need the home it creates.
  - Second: *Define how node tails compose through gain in the graph extent* (#1379). It adds the
    per-node gain and decay that the per-effect slices then state once.
  - After #1379, in any order: *State the parametric EQ's bounded tail and exact-rest bound* (#1372),
    *State the multiband compressor's bounded tail and exact-rest bound* (#1373), *State the
    delay's bounded tail and exact-rest bound* (#1374), *Report a zero tail beyond latency for the
    compressor and the true-peak limiter* (#1375), and *State exact-rest bounds for the gate,
    transient shaper and soft clip* (#1376). #1373, #1374 and #1376 reuse #1375's one-pole helper,
    so they follow #1375; #1373 also follows *Make the multiband compressor's crossover live*
    (#1338), because it owns the glide-in-flight case.
  - Last, after #1372-#1376: *Retire the Infinite tail* (#1378).
- A render-side consumer (silence skipping is #1107). Any C ABI or browser wire change.
- A tighter cascade exact-rest bound: *Tighten the cascade exact-rest bound with a frequency-aware
  cascade analysis* (#1433), after this slice (Amendment 2, F2), including Amendment 3's option 3
  (a two-level `P*`) as a candidate there.
- An absolute output floor in place of the `P < P*` branch (Amendment 3, G2: refused).

## Hazards

- `maximum_finite_tail_samples` (`pdc.rs:132-135`) can now refuse a plan that `Infinite` passed;
  hosts set `u64::MAX` (`crates/host-core/src/prepare.rs:1559`), `graph_fixture`, the audit tools
  (`tools/audit/src/graph.rs:285`, `fixture_builtins.rs:709`, `builtins_graph.rs:641`) and
  `console-workload` (`tools/console-workload/src/lib.rs:2013`) 10,000,000. It compares the
  composed `TailSamples`, which carry `T_decay` (Amendment 3, G1): about 0.9M per input section at
  the live bound, summed along a path. No limit stands at 2.6M (addendum, A1). Gate 7 holds the
  headroom to 10,000,000.
- Depends on #1328's `REST_EPS` and its A9 `N_SILENCE` law; without them D2 has no finite value at
  the top of the domain. Use #1328's `N_SILENCE` constant, never a copy of its value.

## Objective gates

1. **Recomputation** (`tail_contract.rs`, every launch rate in release, 48 kHz only in debug).
   (a) Fixed design: for every HPF/LPF pair from {disabled, 10 Hz, 1 kHz, one ulp below maximum,
   maximum} and trims {0, +24 dB}, an independent brute force computes the `f64` cascade impulse
   response `h` (designed `f32` words, `f64` arithmetic) to 4,000,000 samples and its suffix sums
   `S_b(j) = g * sum_{j <= m < 4e6} |h[m]|` (`g` the linear trim). Define `T_b(e)` as the smallest
   `j` with `S_b(j) < e`. Assert soundness `T_b(eps / 2) <= tail` (the exact-arithmetic half of D3
   is covered) and tightness `tail <= T_b(eps * 2^-5)`: the certified tail is never longer than the
   exact tail at a floor 30 dB lower. The 30 dB margin absorbs the `eps / 2` split (6 dB), the
   cascade union bound and the `f32` rate inflation (`7 * 2^-24 * kappa` against `1 - rho = 5.2e-5`
   at the extreme: about 0.8 % of the decay rate per unit of modal condition number `kappa`, so the
   margin holds for `kappa` up to roughly 13, a proportional placeholder: the measured value replaces this figure); a Putzer `m * rho^m` factor costs about 13 nats, roughly
   250,000 samples at the extreme against the margin's roughly 66,000, so it is red. Disabled
   filters assert `tail == 0`. The certified tail is the one computed with D3's reset-aware exact
   reference (Amendment 2, F1). A certified value above the tightness line is a finding for Sol
   with the measured ratio, not a gate change. *(Amendment 3, G1: every assertion of (a) is on
   `T_decay`, written `tail` above.)* (b) Live bound:
   assert no scanned design exceeds `input_section_live_bound(rate)`: 100,000 log-spaced cutoffs
   plus the last 65,536 `f32` values below each rate's maximum, for both sections, and every
   scanned interior ramp word has `q_f32(w) <= q_ramp < 1`.
   (c) *(Amendment 3, new.)* **`T_rest` on the real kernel and by its formula.** For every enabled
   pair of (a) at both trims, (i) `T_rest` equals `max(T_decay, R(P*))` recomputed from the
   module's own `T_decay`, `P*` and rest bound at `P*`, and `T_rest >= N_SILENCE`; (ii) a real
   `InputBuiltins` (release, every launch rate) driven by random and alternating inputs of peak `P`
   in `{P*, P* / 10, 1e-13, 1e-20}` and then silence reaches exact rest (every output `±0.0` and the
   eight integrator words equal `Z`) at or before `N + T_rest`, and the last sample with
   `|y| >= P * eps` falls before `N + T_rest`. Record the measured rest per design and peak (attempt
   3 measured 346 frames for the 1 kHz sections at `P = 1.26e-13` against `T_b(eps / 2)` = 175).
2. **Soundness on the real kernel** (release): the domain extreme of D5 at every rate, `P` in
   `{1.0, 10^(24/20), 1e29}`, input = `P * sign(h[T + i])` reversed over the last 1,000,000 samples
   before `N`, one live filter target applied 32 samples before `N`; and a quantum-1 history that
   re-sends and alternates the worst-case pair with disable, with a disable in flight at `N`. Assert `|y[n]| < P * eps` for
   every `n` from `N + T` until exact rest (`T` = `T_decay`; every `P` here is above the live
   `P* = 1.7e-3`, Amendment 3), and from `N + R` on (`R` = `peak_plus_24_dbfs` for the
   first two `P`, `any_sanitized_input` for the last; both include the A9 term `2 * N_SILENCE`) D2's rest: every output sample is `±0.0`
   (one run with polarity inverted; *Amendment 4, R4:* the builtin input section's output mix
   normalizes `-0.0`, so that run shows `+0.0`, which D2 admits, and the gate asserts `±0.0`, not
   that a `-0.0` is seen), and the eight SVF integrator words
   equal those of a freshly reset `InputBuiltins` with the same targets (`Z`), under `f32` `==`.
   *(Amendment 4, m1:)* "the domain extreme" means the whole 1,000,000-sample history runs with the
   worst-case pair designed (HPF one `f32` below the maximum into the LPF at the maximum); the live
   HPF target 32 samples before `N` retargets it from there.
3. **Contract figures** (*amended by Amendment 2, F2*): `input_section_live_bound` at every rate has
   `peak_plus_24_dbfs` and `any_sanitized_input` at or below the figures restated in decision 15's
   D15-4 rationale (Deliverable 7: the certified values, each including `2 * N_SILENCE`; restated
   by Amendment 3, G4: at most 1,270,000 and 2,590,000, the per-rate values in G4's table), and
   `peak_plus_24_dbfs` at or above the measured real rest of the top
   pair under the A9 law (gate 2's run; before A9, 983,374 at 44.1 kHz and 978,319 at 48 kHz).
   Record `T_decay`, `T_rest` and `R` per rate in the evidence. Tightening is #1433, not a gate
   change here.
4. **Superseded:** delete `input_tail_is_infinite_while_a_filter_target_is_ramping`; rewrite the
   three `builtins-compiler` tail tests to the finite values; `live_lanes.rs:265` asserts the live
   bound at the fixture's rate and `:202-211` keeps the plain fixture finite and the EQ fixture
   `Infinite` (the EQ slice has not landed).
5. **Re-pins, one at a time:** regenerate `fixtures/graph/v1/direct-route.*` with `graph_fixture`
   and update `ZERO_DELAY_CANONICAL_SHA256`; the only expected byte change is `infinite` →
   `finite:<T>` on post-input-builtins rows. Any other moved byte stops the slice.
   `resources.jsonl`: only the `engine_owned_*` byte counts move, by exactly the
   tail-entry/`InputBuiltins` size delta. *(Amendment 4, R2:)* and
   `maximum_single_allocation_bytes`, by the measured delta recorded in the attempt record, after
   R5; each moved size is audited individually with its reason.
6. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`
     (*Amendment 4, R6:* also a step of `qualification.yml`'s required `test-release` job), and
     `python3 scripts/check-ci-path-routing.py`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml` (it covers
     `builtins-compiler`, `graph-compiler`, `host-core` and `capi` with their `test-support` features)
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-effect-contract.sh`, `bash scripts/check-dsp-research.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo build --locked --release -p audit && ./target/release/audit capi` (`qualification.yml:715`)
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`
7. *(Amendment 3, G5; restated by the addendum, A1.)* **Headroom.** In `tail_contract.rs`, at
   every launch rate: `T_decay`, `T_rest` and both exact-rest bounds (`peak_plus_24_dbfs`,
   `any_sanitized_input`) of `input_section_live_bound(rate)` are below 10,000,000, the smallest
   finite `maximum_finite_tail_samples` the tree sets (a strip's builtin path has one input
   section, so one term). Record each headroom per rate in the evidence. There is no 2.6M check.

## Test value

- Gate 1(a): a tail computation that drops the trim gain or the cascade's second section falls
  below `T_b(eps / 2)`, and a needlessly loose one (a Putzer factor) exceeds `T_b(eps * 2^-5)`; no
  test computes a tail today.
- Gate 1(b): a live bound evaluated at the wrong extreme (for example the minimum cutoff, which
  is not the slowest pole) is exceeded by a scanned design.
- ~~Gate 2: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel at the worst-case input, and a state that settles at a non-reset value (a
  limit cycle the flush misses) fails the fresh-instance comparison; gate 1 only checks `f64`
  arithmetic.~~ *(Superseded: restated by Amendment 4, R3, below.)*
- Gate 1(c): a `T_rest` that drops its `R(P*)` term (reports `T_decay` as the time to exact zero)
  is beaten by the real kernel at small peaks (attempt 3: 346 frames against 175 for a 1 kHz
  section); no other gate drives peaks below `P*`.
- ~~Gate 7: a bound that grows past the configured tail cap turns red before a plan is refused at
  preparation.~~ *(Superseded: restated by Amendment 4, R3, below.)*
- Gate 3: a sound but uselessly loose bound (for example a Putzer `m * rho^m` factor where the
  eigenvector bound applies) breaks the restated D15-4 figures, ~~and an unsound one (a dropped A9
  term) falls below the measured real rest.~~ *(The struck lower side is superseded: restated by
  Amendment 4, R3, below; the upper side's claim stands. Follow-up C narrowed the strike to the
  lower side.)*
- *(Amendment 4, R3: restated after attempt 4.)* On the real kernel the crude cascade's slack
  (live `T_decay` about 905k against a last output above `P eps` near 438k; `R` 1.26M against a
  rest near 0.98M) absorbs an omitted rounding, ramp, stall or A9 term, so gate 2's real-kernel
  runs and gate 3's lower side cannot discriminate them; no real-kernel test can. They are caught
  by `live_bound_carries_every_term_an_independent_recomputation_requires`, which recomputes the
  derivation independently: a live envelope without the state rounding or the ramp allowance falls
  below the recomputed contractions, a bound without the flush stall below the recomputed `P*`,
  and a rest without the A9 term below the recomputed rests. Gate 2's real-kernel runs are red on
  a bound that drops the cascade's second section or states `any_sanitized_input` at +24 dBFS, and
  refuse a history the non-finite recovery reset (attempt 3's vacuous `1e29` run).
- *(Amendment 4, R3.)* Gate 7: no plausible single-term defect drives a finite live bound past
  10,000,000. Every omission or doubling of a term moves the live values by well under 2.5x (the
  largest, a ramp contraction kept for the whole decay, gives `T_decay` 1,263,135), and a defect
  that costs the contraction its margin makes the bound unstated (`Infinite`, no rest), which every
  live gate reports through its finiteness checks. So gate 7 is a headroom record and a ceiling,
  with no discriminating claim of its own; a bound that grows, as that defect does, is red on the
  independent recomputation's upper side (`T_decay` at most the recomputed value plus 0.01 % and
  64 frames), which gate 7 alone does not see.
- ~~*(Amendment 4.)* The live `T_rest` identity (`live_tail_every_peak_is_the_rest_at_the_flush_floor`):
  a live `T_rest` that drops `R(P*)` reports `T_decay`; no other test computes the live `R(P*)`.~~
  *(Amendment 5, m3: that line was false.)* The live `T_rest` identity: a live `T_rest` that is not
  exactly `max(T_decay, R(P*))` fails it. `live_bound_carries_every_term_an_independent_recomputation_requires`
  also computes the live `R(P*)` and is also red on a `T_rest` that drops it; the identity's own
  catch, which no other test makes, is a `T_rest` off by less than that oracle's tolerance (0.01 %
  plus 64 frames).
- *(Amendment 4.)* `math::tail`'s
  `the_operator_norm_bounds_the_exact_norm_of_every_near_top_design`: an operator norm whose `f64`
  evaluation cancels falls below the exact norm of a near-top design; gate 1(a)'s 2-10 % slack
  cannot see a `6e-9` error in `q`.
- *(Amendment 4.)* `each_strip_is_bounded_by_its_own_design_when_designs_are_shared`: a design key
  that omits a term (the trim magnitude or a section's words) hands one strip another design's
  bound when a session repeats designs. *(Amendment 5, m2, completed by Follow-up C:)* including
  either channel: a key that drops the right channel hands the strip that differs only there the
  first strip's bound (C3), and a key that drops the left channel does the same to the strip that
  differs only in its left channel (M5).
- *(Amendment 5, MJ1.)* builtins-compiler
  `design_bounds_are_computed_only_for_strips_without_a_live_input_lane`: a preparation that
  computes a live strip's design bound (attempt 4's, the browser's wasted boot cost) counts six
  computations where it must count none, and one that bounds a design once per strip rather than
  once per distinct design counts nine where it must count six. No other test counts computations.
- *(Amendment 5, m1.)* builtins-compiler `live_input_lane_reports_the_live_bound_and_plain_input_its_own`:
  a seal check that takes a live strip's bound from its own seal entry, rather than requiring the
  sealed live bound, accepts a payload and seal forged alike to the design bound. No other test
  forges a seal entry.

## Dependencies

- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328), passing with its
  Amendment A9 (rest only after the effect input has been exactly zero for `N_SILENCE` frames)
- *Retarget a live input filter only through its designs and their mixtures* (#1407)
- *Keep every trim, fader and matrix ramp inside its endpoints* (#1408)

## Attempt record

### Attempt 1 (2026-10-05, branch `codex/d15-stream-g`)

**Blocked, no code.** Verification before implementation (opus-xhigh; evidence in
`/tmp/claude-1002/v1329-blocker/`) found D5 unprovable under today's laws:

- **Live filter retarget law.** A 15 Hz LPF at 48 kHz, disabled and re-sent every 1-frame block:
  worst one-step `f32` V-norm ratio over 2M states against the exact operator norm, block 800
  `+9.40e-9` (exact `-4.76e-9`), block 1000 `+4.08e-10` (`-2.04e-10`), block 1400 `+7.49e-13`
  (`-3.75e-13`); trajectory frames 700-1500 `f32` net `+1.45e-6` where exact is `-1.45e-6`. The
  ramp never completes while re-sends continue (`c1`, `a2` stuck at `0x00000020`, `m0` at
  `0x3f7fffe0`), the section never elides, the integrators are never cleared. Without re-sends, at
  quantum 1, one block of a 10 Hz design then a disable gives exact `q - 1` of `-1.13e-7` (96 kHz),
  `-1.23e-7` (88.2 kHz), `-2.26e-7` (48 kHz), `-2.46e-7` (44.1 kHz), inside D3's inflation `8.6e-7`.
- **Finding 1 (gate 1(a) tightness), reproduced:** LPF at the maximum, `T_b(eps/2)` /
  `T_b(eps/32)`: 44.1 kHz 0 dB 337,306 / 381,702, +24 dB 381,544 / 436,557; 48 kHz 0 dB
  335,140 / 379,557, +24 dB 379,402 / 434,156. `P*` about `6e-13`, consistent with the stall radius
  `6.66e-16` through the LPF-at-maximum output row (about `6e-5`).
- **Finding 2 (D7 reach):** `tools/audit/src/fixture_builtins.rs` (names `builtins::BuiltinTail`;
  pins `BOXED_TAIL_ENTRY_BYTES = 24`, `BOXED_INPUT_ENTRY_BYTES = 704`,
  `INPUT_PROCESSOR_BYTES = 688`, `STRIP_PREPARATION_BYTES = 1072`),
  `fixtures/builtins/v1/resources.jsonl` (checked by `scripts/check-builtins-fixtures.sh`),
  `crates/builtins-compiler/tests/metered_preparation.rs:12,104`,
  `crates/graph-compiler/tests/MUTATIONS.md` rows 964-1, 964-3 and the 964-3 recheck row, and
  `docs/rulings/builtins-input-liveness-d2.md` ("retain a conservative Infinite tail") were outside
  the authorized paths.
- **Finding 3 (trim overshoot):** a real `InputBuiltins` polarity flip at +24 dB with
  `smoothing_samples = 22,137,669` reaches trim word `21.04` (+26.46 dB) before the snap; host-web
  admits any `u32` window.

Outcome: Amendment 1; #1407 and #1408 filed and made dependencies.

### Attempt 2 (2026-10-05, branch `codex/d15-stream-g`)

**Paused, no production code** (exploration tests only, not committed). Measured in release on the
real kernel, 44.1 kHz unless stated:

- Top pair (HPF maximum minus one ulp into LPF maximum, alternating input at +24 dBFS through the
  +24 dB trim): exact rest at 983,374 samples (978,319 at 48 kHz); LPF at the maximum alone
  925,361. The 1.0M figure leaves about 2 %.
- Brute-force pair tail `T_b(eps/2)` / `T_b(eps/32)`: 393,996 / 451,007 (0 dB), 450,872 / 507,413
  (+24 dB); `|h|_1 = 0.697`.
- Rate inflation: `mu_s = 1.007e-6` at the top (`7 * 2^-24 * kappa`), margin about `3.69e-5`.
- F1 (D3 split unsound under the joint rest) and F2 (D5's ball does not cover the cascade; crude
  bound about 1.19M / 2.5M; a frequency-aware bound estimated at 1.02M-1.07M), with the measured
  values recorded in Amendment 2.

Outcome: Amendment 2; #1433 filed as the successor for the tighter cascade bound.

### Attempt 3 (2026-10-06, branch `codex/d15-stream-g`)

**Stopped before plumbing: gate 1(a)'s tightness line is unreachable as written.** The certified
bounds of D3 (reset-aware reference, Amendment 2 F1) and D5 (crude cascade, F2) were implemented
as an `f64` control-plane module (`math::tail`, not committed; the work-in-progress diff is kept
outside the tree as `1329-attempt3-wip.diff` in the session scratchpad) and measured against the
brute-force `T_b` of gate 1(a) and against the real kernel. Two findings, the first decisive:

- **F3. The real kernel exceeds the tightness line at small peaks.** Through a real
  `InputBuiltins` (44.1 kHz, trim 0 dB, release), random and alternating inputs of peak `P` from
  `1e-11` down to `1e-20.6`, then silence: the last output sample with `|y| >= P * eps` falls at
  frame 346 after `N` for the 1 kHz LPF and for the 1 kHz HPF (`P = 1.26e-13`), and 353 for the
  1 kHz-2 kHz pair, against `T_b(eps / 2)` = 175 / 176 and `T_b(eps * 2^-5)` = 203 / 199 for the
  single sections. So D1's smallest `T` for these designs is at least 346, and no sound certified
  tail satisfies gate 1(a)'s `tail <= T_b(eps * 2^-5)` for them. The cause is the `f32` kernel at
  states near `FLUSH_EPS` (`1e-20`), where the output is no longer relative to `P`. The 10 Hz LPF
  and HPF reach 4,436 (`P = 7.9e-12`) and the 10 Hz-1 kHz pair 3,764, which is `N_SILENCE` at
  44.1 kHz: a state held in `[FLUSH_EPS, REST_EPS)` until the joint flush arms. Both are below
  their tightness lines (20,259 and 19,770).
- **F4. D3's `P < P*` branch makes the certified tail at least the rest bound at `P*`, which
  includes A9's `N_SILENCE` per section.** With the per-word flush modelled as D3 prescribes (an
  absolute `sqrt(2) * FLUSH_EPS` per step, stall `F / (1 - rho_f)` carried through each output
  row), every enabled section gets `T >= R(P*) >= N_SILENCE`: 3,842 for a 1 kHz section at 0 dB
  (4,182 at 48 kHz) against `T_b(eps * 2^-5)` = 203 (221). In cascades whose slow LPF follows a
  section with a large output row, the HPF's flush stall, amplified by the LPF, raises `P*` to
  about `5e-9`, and the rest at `P*` exceeds the tightness line by far more than `N_SILENCE`:

  | 44.1 kHz pair, trim | decay part (D3 sums and rounding) | rest at `P*` | `T_b(eps/2)` / `T_b(eps/32)` |
  |---|---|---|---|
  | LPF 1 kHz, 0 dB | 174 | 3,842 (`P* = 8.8e-12`) | 175 / 203 |
  | HPF 10 Hz into LPF 1 kHz, 0 dB | 18,271 | 20,373 (`P* = 1.6e-9`) | 17,482 / 19,770 |
  | HPF 10 Hz into LPF max, 0 dB | 362,155 | 498,399 (`P* = 4.9e-9`) | 337,306 / 381,702 |
  | HPF 1 kHz into LPF max, +24 dB | 415,269 | 462,674 (`P* = 5.6e-11`) | 381,544 / 436,558 |
  | HPF max - 1 ulp into LPF max, 0 dB | 420,121 | 429,335 (`P* = 9.6e-12`) | 393,996 / 451,007 |
  | HPF max - 1 ulp into LPF max, +24 dB | 478,305 | 485,928 | 450,872 / 507,413 |
  | LPF max, +24 dB | 398,850 | 384,977 | 381,544 / 436,557 |

  The decay part alone (the exact half below `eps / 2` and the relative rounding below `eps / 4`
  from then on) is inside the tightness line for every pair of gate 1(a) at 44.1 and 48 kHz, both
  trims; only the rest branch breaks it. (48 kHz figures follow the same pattern: 1 kHz 4,182 vs
  221; 10 Hz into the maximum 497,585 vs 379,557.)

**What does fit (measured with the same module, for the root's ruling):**

- D5's live bound, crude cascade, ramp window 64, settled contraction after it, `+2 * N_SILENCE`
  (44.1 / 48 / 88.2 / 96 kHz): `peak_plus_24_dbfs` 1,264,736 / 1,257,840 / 1,268,585 / 1,262,029;
  `any_sanitized_input` 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156 (capped at finite `f32`
  states, below the 2.6M of "Hazards" but close); `T_live` 1,081,764 / 1,075,575 / 1,085,641 /
  1,079,792, of which the decay part is 904,785 / 899,524 / 904,795 / 899,533 and the rest is the
  rest at `P* = 1.7e-3` (the HPF flush stall through the crude cascade, about `5.5e-11` at the
  output). Gate 3's lower side holds (above the measured 983,374 / 978,319). The restated D15-4
  figures would be about 1.27M and 2.59M, not "about 1.19M and 2.5M": the crude cascade here
  takes `beta <= 2`, `gamma <= sqrt(2)` and `|d| <= 1` as suprema over every design (closed forms
  `beta = 2 g / sqrt(1 + t)`, `gamma = sqrt(2) / sqrt(1 + t)`, `t = g (g + sqrt(2))`, `d` in
  `[0, 1]`), plus the #1407 allowance, the mix-ramp allowance and `|fl(k) - sqrt(2)|`.
- Rate inflation and margin. The kernel contraction is computed without sampling: the exact
  design radius `rho(g) = sqrt(1 + g^4) / (1 + sqrt(2) g + g^2)` is symmetric under `g -> 1/g`
  and unimodal, so its maximum over the domain is at the maximum cutoff; the `f32` designs add
  `P(h) = 2.16e-7` (half-ulp word box) and the ramp words `P(E) = 1.419e-5`; the state rounding is
  `mu_state = 1.0073e-6` (`7 * 2^-24 * kappa`) for the designs above half the maximum `g` and
  `1.120e-6` below it, where the radius is far smaller. Certified margin `1 - rho_ramp`:
  `3.673e-5` (44.1, 88.2 kHz), `3.701e-5` (48, 96 kHz). D5's "at least `3.69e-5` (measured)" uses
  the `f32` top design's norm instead of `rho(g_max) + P(h)`; it reproduces as `3.694e-5` and
  `3.721e-5`. Both prove `rho_ramp < 1`; the certified figure is the one a derivation can stand
  on, so D5's figure should read `3.67e-5` (certified).
- The fixed-design rest bounds (+24 dBFS through +24 dB trim, top pair): 1,051,297 (44.1 kHz),
  1,045,991 (48 kHz), against the measured real rest 983,374 / 978,319 before A9.
- A9 observation: both builtin sections are armed by the same effect-input counter, so after the
  HPF rests the LPF is already armed; the second `+N_SILENCE` of Amendment 2 is conservative
  (sound, not needed). Not acted on.

**Options for the root (none taken; each changes a frozen decision or a gate):**

1. Keep D1 and D3 as written and amend gate 1(a)'s tightness line to apply to the decay part
   only, stating the rest branch (`T = max(decay part, R(P*))`) as part of the contract. F3 shows
   that any sound bound has `T` above `T_b(eps / 32)` for 1 kHz sections, so the line cannot stay
   on the whole tail in any form.
2. Amend D1 with an absolute floor: `|y[n]| < max(P * eps, eps_abs)`, `eps_abs` at or above twice
   the flush stall at the output (fixed designs: stall at most about `1.6e-16`, so a floor of
   about `3.2e-16`, -310 dBFS; the live crude cascade: stall about `5.5e-11`, floor about
   `1.1e-10`, -199 dBFS, which #1433 would lower). The `P < P*` branch then
   goes, `T` is the decay part, and gate 1(a) passes as written (table above). D2 is unchanged.
3. Option 1 with a two-level `P*` (the HPF rests first, then only the LPF's own stall applies),
   which brings the cascade rows of the table back near the decay part but still leaves every
   enabled section at `T >= N_SILENCE`.

Gates run: none of the objective gates (no production code). Measurements: the module's own
exploration tests in release, and the real-kernel probe above.

Outcome so far: Amendment 3 (root rulings, option 1). Attempt 3 resumes under it, after #1451 and
#1328's follow-up fix (root's order); no verdict yet.

#### Attempt 3, resumed under Amendment 3 and its addendum (2026-10-06)

Implemented on `codex/d15-stream-g` (spec edits `1df306d97`; implementation checkpoint `11f59fa42`
and the evidence commit after it). Release measurements on this host (x86-64-v3), every launch rate
unless stated.

**What shipped.**

- `effect-contract`: D1 on `TailSamples` (with Amendment 3's peak range and exact-zero branch) and
  D2's `RestSamples` (with `RestSamples::ZERO`).
- `math::tail` (control plane, `f64`, `math::log` only): `fixed_cascade` (D3: the reset-aware
  reference by impulse majorants, the `f32` deviation with relative rounding and the per-word
  flush's absolute stall, `P*`, rest section by section with A9's `N_SILENCE`) and
  `envelope_cascade` (D5: invariant balls through the crude cascade, 64 ramp frames, then settled
  contraction). Each returns `T_decay` (`tail`), `T_rest` (`tail_every_peak`), both rests, `P*`
  (`flush_floor`) and `R(P*)`.
- `builtins::tail`: `InputSectionBound { tail, tail_every_peak, rest }`, `input_section_live_bound`,
  `input_section_live_envelope`, `input_section_live_cascade`, `input_section_worst_case_pair`
  (derived from `builtin_filter_cutoff_maximum_hz`: the maximum minus one `f32` into the maximum,
  D5's table), `prepare_input_bound(s)`, `BuiltinChain::with_prepared_bound`;
  `InputBuiltins`/`BuiltinChain::{tail, tail_every_peak, rest}`. `BuiltinTail` is deleted.
- **Name (addendum A3):** `T_rest` is `tail_every_peak`, a `TailSamples` beside `tail` wherever
  `tail` is stated (stated in `docs/EFFECT_CONTRACT_V1.md`, "Tail and exact rest").
- `builtins-compiler` reports the prepared design's `T_decay`, or the live bound's for a strip
  with a live input lane, at both D7 sites; `graph-compiler` takes `TailSamples` directly.
- Docs: `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`, `dsp-research/filters.md`,
  `docs/derivations/1329-input-section-tail-and-rest.md`, `docs/rulings/builtins-input-liveness-d2.md`.

**Decisions the implementation had to take (for Sol):**

1. **`t0`, one frame conservative.** The reference output at `N + m` sums `|h|` from `m + 1`, so D1
   needs `T >= T_b(eps / 2) - 1`; gate 1(a) asserts `T_b(eps / 2) <= T`. The exact half is stated as
   `t0` (one frame more than D1 needs) so the gate's index and D1's agree. One sample, sound.
2. **When the bound is computed (D7).** Computing it in every `BuiltinChain::new` broke two
   existing invariants: the phase-two retained-allocation account
   (`builtins-compiler/tests/allocation_tracker.rs`, three tests red: the computation's transient
   `Vec`s are not retained storage) and test time (graph-compiler's 65,537-track debug `scale`
   tests 16.5 s -> 531.8 s; `audit fixture-builtins` 8 s -> 6 min). Now: the compiler computes each
   **distinct** design's bound once, before the phase-two account (`prepare_input_bounds`), and
   the chains store it (`with_prepared_bound`, which checks the design key: rate, both channels'
   section words and trim magnitude, and otherwise computes its own); a chain built by
   `BuiltinChain::new` computes on the first `tail`/`tail_every_peak`/`rest` request (a
   `OnceCell`, control thread only; render never reads it). The bound is still computed once,
   on the control thread, never on render. Scale tests back to 16.5 s, fixture generation 8.3 s.
   Cost per distinct design: about 145 us (canonical 20 Hz HPF into 20 kHz LPF), 7 us (18 kHz
   LPF), up to about 10 ms at the top of the domain; the live bound about 100 us (release).
3. **Canonical floating-point environment.** The `f64` decays underflow and raise MXCSR's
   underflow flag, which turned `host-core/tests/fp_environment.rs`'s
   `the_callers_word_is_restored_bit_exactly_after_every_block` red (caller's word `0xFFE0`, after
   preparation `0xFFF0`). Every bound now runs under `lane::CanonicalFpEnv` (round to nearest, full
   subnormals as on Wasm; the caller's word restored bit for bit).
4. **Outside the listed paths (each a mechanical consequence, flagged here):**
   `crates/builtins/src/filter_response.rs` reads designs through `prepare_input_track`, so the
   allocation-free response query (`malformed_grids_shapes_and_budgets_are_atomic_and_allocation_free`)
   computes no bound; `tools/audit/src/builtins_graph.rs` carries the joined-corpus manifest
   identity that the `resources.jsonl` re-pin moves (as #1451 did).
5. **`InputBuiltins` grows 72 bytes** (`OnceCell<InputSectionBound>` and the rate and two trim
   words it is computed from; the prepared sections are the stage's own `filter_initial`).

**Gate 1(a)** (`fixed_design_tail_is_sound_and_within_thirty_db_of_the_exact_tail`): 112 rows
(4 rates x 14 enabled pairs x 2 trims), all `T_b(eps / 2) <= T_decay <= T_b(eps / 32)`; disabled
filters report `0`, `0` and `RestSamples::ZERO`. `T_decay / T_b(eps / 2)` is 1.0000 for single
sections and at most 1.0977 (96 kHz, 10 Hz HPF into the maximum, +24 dB: 416,484 against 379,402;
tightness headroom 4.1 % below `T_b(eps / 32)` = 434,156, the smallest). 44.1 kHz at +24 dB, for
example: LPF 1 kHz 203 / 203 / 232; top pair 478,315 / 450,872 / 507,413.

**Gate 1(b)** (`live_bound_covers_every_scanned_design_and_ramp_word`): 165,461 to 165,473 designs
per section per rate; every design inside the live envelope (contraction with rounding, input
column, output row, feedthrough). The slowest design is the maximum cutoff for both sections
(`q + mu_state - 1` = -5.1125e-5 at 44.1/88.2 kHz, -5.1403e-5 at 48/96 kHz) and the slowest HPF
below it is the maximum minus one `f32`: the worst-case pair is confirmed. Envelope
`rho_ramp - 1` = -3.67309e-5 / -3.70137e-5 (G4's certified margins), `rho_settled - 1` =
-5.09193e-5 / -5.12021e-5. Sampled designs' own bounds at +24 dB never exceed the live bound.
Ramp words read from the real kernel (consecutive designs at the top, the maximum to log-spaced
designs and back, per-frame restarts maximum/neighbour and maximum/10 Hz): worst contraction
-5.10228e-5 / -5.13534e-5, inside `rho_ramp`.

**Gate 1(c)** (`tail_every_peak_is_the_rest_at_the_flush_floor_and_holds_on_the_real_kernel`):
(i) for all 112 rows the prepared values equal the module's on the kernel's words, `T_rest` equals
`max(T_decay, R(P*))` with `R(P*)` recomputed through the rest bound at the peak `P*`, and
`T_rest >= N_SILENCE`. (ii) the real kernel at `P* , P*/10, 1e-13, 1e-20`, random and
alternating, then silence: at rest (state checked at a block boundary exactly at `N + T_rest`) and
last `|y| >= P eps` before `T_rest` in every row. Selected (44.1 kHz, `T_decay` / `P*` / `R(P*)` /
`T_rest`; real last-above / rest by, 64-frame resolution): LPF 1 kHz 0 dB 175 / 8.8e-12 / 3,842 /
3,842; 150 / 384. HPF 10 Hz into LPF 1 kHz 0 dB 18,282 / 1.6e-9 / 20,373 / 20,373; 7,696 / 7,744.
HPF 10 Hz into the maximum +24 dB 417,748 / 5.0e-9 / 553,003 / 553,003; 363,017 / 482,112. Top
pair +24 dB 478,315 / 9.6e-12 / 485,928 / 485,928; 406,802 / 428,736. (Attempt 3's 346 frames for
1 kHz came from a 4,000-trial probe; the gate's four peaks measure 150.)

**Gate 2** (`live_bound_holds_on_the_real_kernel_at_the_domain_extreme`, release only; ignored in
debug): extreme pair, trim +24 dB, input `P sign(h[T + 1 + i])` over the last 1,000,000 samples,
a live HPF retarget (maximum minus two `f32`) 32 samples before `N`. Last `|y| >= P eps` at
366,583-366,585 (44.1/88.2 kHz) and 364,538-364,542 (48/96 kHz) against `T_decay` 904,785-904,795 /
899,524-899,533; rest by 875,328 (`P = 1`) and 926,865 (+24 dBFS) at 44.1 kHz, 921,604 at 48 kHz,
926,811 at 88.2 kHz, 921,549 at 96 kHz, against `peak_plus_24_dbfs` 1.26M; integrators equal the
fresh section's (`+0.0`) at `N + R`. Inverted polarity: identical; every output after rest is
`+0.0` (no `-0.0` is reached through an enabled section: the mix normalizes it), so the run shows
`+-0.0` but not a `-0.0`. `P = 1e29` with the sign pattern: no output at or above `P eps` after
`N` and at rest by `N + 64` (mechanism not investigated); the same test therefore also drives the
fixed top pair with an alternating `+-1e29` input for 1,000,000 samples, which leaves the
integrators near `4e34` (no recovery fires) and decays: last `|y| >= P eps` at 436,322 (44.1,
88.2 kHz) / 433,861 (48, 96 kHz), rest by 2,237,009 / 2,225,028 / 2,237,019 / 2,225,037 against
`any_sanitized_input` 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156 (13.4 % margin at 44.1 kHz).
Block-size-1 history (pair/disable alternating every frame, a disable in flight at `N`): at rest
by 64.

**Gate 3:** live bounds at or below 1,270,000 and 2,590,000 (values below, unchanged from G4); the
lower side, the fixed top pair's real rest under an alternating +24 dBFS input through +24 dB
under A9 (in gate 2's test): 983,377 / 978,372 / 983,387 / 978,381 (44.1 / 48 / 88.2 / 96 kHz),
below `peak_plus_24_dbfs` (before A9 attempt 2 measured 983,374 / 978,319).

| rate | `T_decay` | `T_rest` | `peak_plus_24_dbfs` | `any_sanitized_input` | `P*` |
|---|---|---|---|---|---|
| 44.1 kHz | 904,785 | 1,081,764 | 1,264,736 | 2,583,197 | 1.735e-3 |
| 48 kHz | 899,524 | 1,075,575 | 1,257,840 | 2,569,016 | 1.709e-3 |
| 88.2 kHz | 904,795 | 1,085,641 | 1,268,585 | 2,587,000 | 1.736e-3 |
| 96 kHz | 899,533 | 1,079,792 | 1,262,029 | 2,573,156 | 1.709e-3 |

G4's table holds as implemented; Deliverable 7's figures need no restatement.

**Gate 7** (headroom to `maximum_finite_tail_samples` = 10,000,000, per rate): `T_decay` 9,095,215
/ 9,100,476 / 9,095,205 / 9,100,467 (91.0 %); `T_rest` 8,918,236 / 8,924,425 / 8,914,359 /
8,920,208 (89.2 %); `peak_plus_24_dbfs` 8,735,264 / 8,742,160 / 8,731,415 / 8,737,971 (87.4 %);
`any_sanitized_input` 7,416,803 / 7,430,984 / 7,413,000 / 7,426,844 (74.1-74.3 %).

**Gate 4:** `input_tail_is_infinite_while_a_filter_target_is_ramping` deleted; the two
`builtins-compiler` tests rewritten (`prepares_three_sections_and_each_named_meter_tap` asserts the
fixture strip's own finite non-zero tail; `live_input_lane_reports_the_live_bound_and_plain_input_its_own`
asserts `Finite(0)` plain and the live bound live); `live_lanes.rs` keeps the plain fixture finite
and the EQ fixture `Infinite`, and under `HostLiveLanes::ALL` asserts `Finite(2 * live T_decay)`
(the deepest path crosses the last track and the bus); `graph-compiler`'s
`builtins_replace_only_the_three_internal_track_bindings` asserts the prepared finite tail on the
node and the output.

**Gate 5:** `fixtures/graph/v1/direct-route.*`: only `infinite` -> `finite:10048` on the
post-input-builtins `node` and `tail` rows (and the derived hashes and lengths);
`ZERO_DELAY_CANONICAL_SHA256` `bf2dfd6c…ad7b10d8b3` -> `60cae21e…fc6c362`, diffed on the dumped
canonical text: the eighteen tail tokens are the only change. `resources.jsonl`: +160 bytes per
track in both payload counts (72 in the strip preparation, 72 in the bank input entry, 8 in each
of the two tail entries); **`maximum_single_allocation_bytes` also moves**, +72 per track (the
strip vector), which the gate did not predict but which is the same size delta.

**Mutation runs** (each applied, the named test run, the tree restored; RED = the test fails):

| defect | test | result |
|---|---|---|
| fixed bound drops the trim gain | 1(a) | RED: `T_b(eps/2) 22044 > T 18948` (48 kHz, LPF 10 Hz, +24 dB) |
| fixed bound drops the cascade's second section | 1(a) | RED: `T_b(eps/2) 334036 > T 19051` |
| a Putzer factor `m` on the impulse majorant | 1(a) | RED: `T 22528 > T_b(eps/32) 22051` |
| live envelope at the wrong extreme (10 Hz for the maximum) | 1(b) | RED: the design at 23,995 Hz exceeds the envelope |
| `T_rest` drops `R(P*)` | 1(c) | RED: 175 against 3,842 |
| A9 term dropped from every rest | 1(c)(i) | RED: `t_rest >= law.silence_frames` |
| A9 term dropped | 2, 3 | **GREEN**: the crude cascade leaves about 330k samples between the bound and the real rest, far more than `2 N_SILENCE` |
| live bound with one section (cascade dropped) | 2 | RED: the real kernel is not at rest by the mutated `R` |
| a Putzer-like `ln(m)/(1 - rho)` in the rest decay | 3 | RED: `peak_plus_24_dbfs <= 1_270_000` |
| `any_sanitized_input` stated at the +24 dBFS peak | 2 (alternating `1e29` run) | RED: `44100 Hz P 1e29: not at rest by R` |
| both contractions lose `4e-5` of margin | 7 | RED, but through the missing bound (`rho_ramp >= 1`, no rest stated), which gate 3 also catches |
| plain strips report `Finite(0)` (both D7 sites) | `prepares_three_sections_and_each_named_meter_tap` | RED: `Finite(0)` against `Finite(10048)` |
| a live lane reports `Finite(0)` | `live_input_lane_reports_…`, `live_lanes.rs` | RED: `Finite(0)` against `Finite(899524)` / `Finite(1799048)` |
| graph maps every builtin tail to `Finite(0)` | `builtins_replace_only_…`, `track_delay` digest | RED (MUTATIONS.md 964-3 #1329 recheck) |
| graph maps every builtin tail to `Infinite` | `direct_graph_report_exposes_zero_output_latency_…` | RED (MUTATIONS.md 964-1 #1329 recheck) |
| `with_prepared_bound` ignores the design key | `a_prepared_bound_is_stored_only_for_its_own_design` | RED: `Finite(192)` against `Finite(19051)` |

Test value, against the spec's section: gates 1(a), 1(b), 1(c) as stated. Gate 2 is not
sensitive to "a bound that ignores `f32` rounding, the ramp in flight or the flush stall": the
crude cascade's slack (live `T_decay` about 905k against a real last-above of about 367k; `R`
1.26M against a real rest of about 0.93M) absorbs each; it is red for a bound that drops the
cascade. Gate 3's lower side does not catch a dropped A9 term (GREEN above); 1(c)(i) does. Gate 7
is dominated by gate 3 for the rest bounds and by the finiteness checks for `T_decay`/`T_rest`:
with the coupled ramp and settled margins no single-margin defect drives a finite live bound past
10,000,000 without breaking contraction first.

**Gates run (all green unless stated):** `cargo fmt --all -- --check`; `cargo clippy --locked
--workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked
--workspace --no-deps`; `cargo test --release -p builtins --test tail_contract` (7 tests, about 7 s; 6 in debug, gate 2 ignored, 16 s);
the `test-debug-a` and `test-debug-b` workspace commands; `check-workspace-policy.sh`,
`check-realtime-policy.sh` (89 regions), `check-builtins-policy.sh`, `check-dsp-research.sh`,
`check-graph-determinism.sh` (100/100), `check-effect-contract.sh`, `check-cross-targets.sh`
(builtins `memset_pattern16` 5, at its ceiling, unchanged), `check-builtins-fixtures.sh` with the
release audit (50 files), `audit capi` (0 allocations, 0 syscalls, `total_violations` 0),
`check-capi-abi.sh`, `run-wasm-gates.sh --without-v8-spill --without-native`, and the worklet chain
(`build-web-audioworklet.sh --named-twin`, `strip-wasm-names.py check`,
`check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`,
`test-web-audioworklet.sh`). The shipped worklet module's bytes change (the bound code is linked
into preparation), as `artifact-identity` will report. AArch64 is CI-only.

**Open items for root and Sol:**

- `tail_contract`'s release scale (gate 2 and 1(c)(ii)) runs in no CI job: `qualification.yml` is
  outside this slice's paths. A `test-release` step (`cargo test --locked --release -p builtins
  --features builtins/test-support --test tail_contract`, about 19 s here with the build) would carry it, as #1428 does
  for `filter_liveness`.
- D2 holds for histories within one prepared plan. A swap that carries non-zero integrators into a
  disabled (identity) section leaves them frozen (#1407 rule 4: the identity recursion keeps them,
  the output ignores them), so its integrators never equal `Z`; whether a carried state counts as
  "a control history the node admits" is for the swap slices (#1269) to state.
- Gate 7's test value (above) and gate 3's lower side are weaker than the spec's "Test value"
  claims; no gate was changed.

### Attempt 4 (2026-10-06, branch `codex/d15-stream-g`, under Amendment 4)

Spec edits `b9de38e70` (Amendment 4); implementation `feefbe166`; this record and the gate-5,
doc and test follow-ups in the commit after it. Release measurements on this host (x86-64-v3).

**M1 (the operator norm).** `math::tail::v_operator_norm` now forms `R A R^-1` from its exact
algebraic entries (`[[al + r ga, sqrt(2) be + de - al - r ga], [r ga, de - r ga]]`), takes the
larger singular value by the non-cancelling closed form
`(sqrt((a + d)^2 + (c - b)^2) + sqrt((a - d)^2 + (b + c)^2)) / 2`, adds the Frobenius norm of the
entries' a-priori `f64` errors (`16 u64` times each entry's term magnitudes) and inflates by
`1 + 16 u64`: a certified upper bound by derivation, so `SectionConstants::of` no longer multiplies
`q` by `SLACK` (and `word_box_norm` no longer does either). The other compounding roundings the
derivation's "Numerical limits" used to wave at are now bounded too: the first section's `f64`
impulse response carries an error radius through its output row, the later majorant recursions
step up by `1 + 4 u64` per frame, and the long sums by `1 + 2 n u64`. A unit test
(`the_operator_norm_bounds_the_exact_norm_of_every_near_top_design`) checks six designs against
60-digit references; the old form is red on it (`0.99994766942311020` against the exact
`0.99994767541470919`, 44.1 kHz HPF max-1ulp). Live figures do not move (the live bound uses the
closed-form `rho(g)`). Gate 1(a)/1(c) re-recorded (149 of 224 printed lines change); selected
44.1 kHz values, attempt 3 / attempt 4:

| design (44.1 kHz) | attempt 3 | attempt 4 |
|---|---|---|
| HPF 22049.48 alone, +24 dB, `T_decay` | 397,389 | 397,440 |
| LPF 22049.48 alone, +24 dB, `T_decay` | 397,249 | 397,300 |
| HPF 10 Hz into LPF 22049.48, +24 dB, `T_decay` / `T_rest` | 416,064 / 550,795 | 416,117 / 550,862 |
| top pair, +24 dB, `T_decay` / `T_rest` | 478,315 / 485,928 | 478,335 / 485,945 |
| top pair, 0 dB, `T_decay` | 420,130 | 420,148 |
| 96 kHz HPF 10 Hz into the maximum, +24 dB (largest ratio) | 416,484 | 416,480 |

Where attempt 3's cancelling form was below the exact norm the values rise (by up to about 60
frames); elsewhere they fall by a few frames, because `q` no longer carries the redundant `2^-30`.
Gate 1(a): all 112 rows `T_b(eps / 2) <= T_decay <= T_b(eps / 32)`, largest ratio 1.0977 (96 kHz,
10 Hz into the maximum, +24 dB). Gate 1(c)(i)/(ii): all 112 rows green; the real kernel's figures
are unchanged. `dsp-research/filters.md` now quotes 420,148; the derivation's FMA sentence (no
target fuses: `Lane::fma` rounds twice) and its "far below one sample" (it compounds, tens of
frames, always longer; checked by the independent recomputation) are corrected. The canonical
graph fixture stays `finite:10048` (`graph_fixture --check` green, no re-pin).

**R5, R7 (where the bounds live; computed once).** `InputBuiltins` and `BuiltinChain` are back to
their pre-#1329 layout: no `OnceCell`, no stored rate or trims, no `tail`/`tail_every_peak`/`rest`
accessors, no `with_prepared_bound`. `builtins::input_section_bound(rate, parameters)` and
`input_section_bounds(rate, strips)` (each distinct design once, keyed by rate, both channels'
section words and trim magnitudes) are the control-plane entry points. `builtins-compiler` calls
`input_section_bounds` once per preparation, before the phase-two account, and the live bound
once if any input lane is live; `PreparedBuiltinsSession.tails` holds `(Box<str>,
InputSectionBound)` per strip (`tails()` maps it to `T_decay` for graph lowering;
`input_bounds()` gives #1107 all three values). The seal keeps the same vector and the live bound;
`validate_for_session` reuses them: it checks the track set, that every live strip carries the
sealed live bound, and that the payload agrees with the seal, and computes no design bound. The
seal's design bounds are tied to the session by its identity hash; the check no longer recomputes
them (a defect only a recomputation would catch, a bound computed for another design, is
`each_strip_is_bounded_by_its_own_design_when_designs_are_shared`'s, below). Name: `T_rest` stays
`tail_every_peak`, now a field of `InputSectionBound`.

**Measured preparation cost** (release, one thread, best of three, scratch probe not committed):
a typical design (20 Hz HPF into 20 kHz LPF) 0.19-0.34 ms; a 1 kHz LPF 0.01-0.02 ms; 1,024 strips
sharing one design 0.35-0.49 ms; the LPF at the maximum, +24 dB, 3.5 ms; the top pair, +24 dB,
7.0 ms; 64 distinct near-top designs (HPF 10-640 Hz into the LPF 1-64 `f32` below the maximum,
+24 dB) 529-561 ms, 8.3-8.8 ms each, the worst per-design cost found; the live bound 0.04 ms. The
worst case per preparation is therefore about 8.8 ms per distinct design, bounded only by the track
count (65,537 distinct near-top designs: about 9.6 minutes, extrapolated). Filed as #1457, *Cache
design bounds across preparations within a stated preparation budget* (stream G, row 21 of
`STREAMS.md`'s current table; *Amendment 5:* the record said "row 19"), which
notes that H #1332 moves browser preparation off the AudioWorklet thread.

**R1, R2 (paths, gate 5).** `crates/builtins/src/filter_response.rs` is byte-identical to its
pre-#1329 content. `resources.jsonl`, regenerated by `audit fixture-builtins --write` into a
scratch directory (only `resources.jsonl` differed from the tree; `filter-response.csv` and every
PCM, meter and benchmark file identical), field by field against `main` (pre-#1329): both
`engine_owned_processor_payload_bytes` and `engine_owned_retained_payload_bytes` +96 per track
(1 / 4 / 65,537 tracks: +96 / +384 / +6,291,552), the two tail vectors' entries growing from 24 to
72 bytes (`(Box<str>, InputSectionBound)`); `maximum_single_allocation_bytes` **+0** (back to
`main`'s 1,080 / 4,320 / 70,779,960: the strip vector no longer grows; attempt 3 had +72 per track).
Against attempt 3: -64 per track in both payload counts, -72 per track in the largest allocation.
`fixture_builtins.rs`: `BOXED_INPUT_ENTRY_BYTES` 712, `INPUT_PROCESSOR_BYTES` 696,
`STRIP_PREPARATION_BYTES` 1,080 (each back to its pre-#1329 value), `BOXED_TAIL_ENTRY_BYTES` 72
(the ABI row now names `(Box<str>, builtins::InputSectionBound)`); `MANIFEST.tsv`'s
`resources.jsonl` row and the joined-corpus manifest identity (`1a8fd9a1...6e288`, in
`builtins_graph.rs` and the fixture test) follow it.

**R6.** `qualification.yml`'s `test-release` runs `cargo test --locked --release -p builtins
--features builtins/test-support --test tail_contract` after the `filter_liveness` step;
`check-ci-path-routing.py` green with no router or verdict change. Local cost: 7.2 s run (9 tests,
gate 2 and 1(c)(ii) included) after the release build.

**R8.** #1269's spec gains "Exact rest across a swap (from #1329, root-authorized, 2026-10-06)";
its GitHub body is synced.

**m1 (gate 2 at the extreme).** Every gate-2 history now runs with the worst-case pair designed
from its first sample (the live HPF retarget to one `f32` lower stays at `N - 32`), and the test
asserts that no non-finite recovery fired (`lifetime_recovered_state() == (0, 0)` at `N`) and that
the history left a tail (`|y| >= P eps` after `N`). Measured (44.1 / 48 / 88.2 / 96 kHz): `P = 1`
last above 437,779 / 435,263 / 437,780 / 435,264, rest by 946,193 / 940,676 / 946,203 / 940,621;
`+24 dBFS` last above 437,809 / 435,293 / 437,810 / 435,293, rest by 982,609 / 977,604 / 982,555 /
977,613 (`R` 1,264,736 / 1,257,840 / 1,268,585 / 1,262,029); `P = 1e29` (sign pattern, now
non-vacuous: the state stays finite) last above 437,772 / 435,258 / 437,773 / 435,259, rest by
2,234,641 / 2,222,916 / 2,234,651 / 2,222,925 (`R` 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156);
inverted polarity identical to `+24 dBFS` with every output `+0.0` after rest (R4); the
quantum-1 history at rest by 64. Attempt 3's record of the `1e29` run ("at rest by `N + 64`,
mechanism not investigated") described the non-finite recovery resetting a history that started
at 1 kHz; that run tested nothing after `N`. Gate 3's lower side (alternating input, top pair):
983,377 / 978,372 / 983,387 / 978,381 at +24 dBFS and 2,237,009 / 2,225,028 / 2,237,019 /
2,225,037 at `1e29`, unchanged.

**m2, R3 (new tests).** `live_tail_every_peak_is_the_rest_at_the_flush_floor`: the live `T_rest`
equals `max(T_decay, R(P*))` with `R(P*)` recomputed through `envelope_cascade` at `P*`
(1,081,764 / 1,075,575 / 1,085,641 / 1,079,792) and is at least `N_SILENCE`.
`live_bound_carries_every_term_an_independent_recomputation_requires`: the envelope's
contractions are at least the exact top radius plus `P(h)`, `kappa u rho` and (ramp) `P(E)` for
`E = 64 h + u D` (44.1 kHz: `rho_settled - 1` -5.091925e-5 against a required -5.178264e-5,
`rho_ramp - 1` -3.673087e-5 against -3.759427e-5), and `P*`, `T_decay`, `T_rest`, `R(P*)` and both
rests lie between a frame-by-frame recomputation in plain `f64` and that value plus 0.01 % and 64
frames (module minus recomputed at 44.1 kHz: `T_decay` +17, `R(P*)` +14, `peak_plus_24_dbfs` +18,
`any_sanitized_input` +42; the verifier's `verify_live.py` reproduces). Superseded and deleted:
`a_prepared_bound_is_stored_only_for_its_own_design` (its mechanism is gone); replaced by
`each_strip_is_bounded_by_its_own_design_when_designs_are_shared`. The builtins-compiler tests
assert `input_bounds()` (the plain strip `InputSectionBound::ZERO`, the live strip the whole live
bound) and that the live preparation passes the seal check. The SplitMix draw computes
`x * 2.0 - 1.0` (`x * 2` exact, one rounding, as before) instead of `mul_add` (lane policy).

**Mutation runs** (each applied in the worktree, the named test run in release unless stated,
the file restored; RED = the test fails):

| defect | test | result |
|---|---|---|
| cancelling operator-norm form (attempt 3's) | `the_operator_norm_bounds_…` | RED: 0.99994766942311020 against 0.99994767541470919 |
| live envelope drops the state rounding | `live_bound_carries_every_term_…` / gate 2 | RED (envelope omits a term) / GREEN |
| live envelope drops the ramp allowance | same / gate 2 | RED / GREEN |
| live bound drops the flush stall | same / gate 2 | RED (`P*` 0 against 1.7349e-3) / GREEN |
| A9 term dropped from every rest | same / 1(c) / gate 2 / gate 3 | RED (`T_rest` 1,077,903 against 1,081,750) / RED / GREEN / GREEN |
| ramp contraction kept for the whole live decay | same / gate 7 / gate 3 | RED (`T_decay` 1,263,135 against 904,768) / GREEN / GREEN |
| live `T_rest` drops `R(P*)` (verdict M9) | `live_tail_every_peak_…` / gate 7 | RED (904,785) / GREEN |
| design key ignores the trim | `each_strip_is_bounded_…` | RED |
| design key ignores the LPF words | `each_strip_is_bounded_…` | RED (after adding the strip that differs only in its LPF; GREEN before) |
| gate 2's history starts at 1 kHz (attempt 3's run) | gate 2 | RED (`P = 1e29`: the history overflowed) |
| a live strip keeps its design bound (preparation) | `live_input_lane_reports_…` (debug) | RED |
| the seal check takes a live strip's bound from its seal entry | `live_input_lane_reports_…` (debug) | GREEN: no test reaches it (below) |

Test value, against "Test value": all lines now hold as restated by Amendment 4 (R3). Gate 7 has
no discriminating claim (restated there, with the reason). Open: the seal check's live-strip rule
(a live strip must carry the sealed live bound) has no test that turns red without it: the payload
and the seal come from one preparation, so only a forged seal entry reaches the rule, and no
corruption case forges one.

**Gates run (all green unless stated):** `cargo fmt --all -- --check`; `cargo clippy --locked
--workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked
--workspace --no-deps` (first red on two private intra-doc links in `math::tail`, fixed);
`cargo test --release -p builtins --test tail_contract` (9 tests, 7.2 s); the `test-debug-a`
(1,441 passed, 0 failed, 10 ignored) and `test-debug-b` workspace commands;
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh` (attempt 3's
`mul_add` gone), `check-builtins-policy.sh`, `check-dsp-research.sh`, `check-effect-contract.sh`,
`check-graph-determinism.sh`, `check-ci-path-routing.py`, `graph_fixture --check`,
`check-builtins-fixtures.sh` with the release audit, `audit capi` (0 allocations, 0
deallocations, 0 syscalls, 0 violations; `pcm_digest` `cb10fbface44a3a4`, unchanged),
`check-capi-abi.sh` and `--self-test`, `check-cross-targets.sh` (PASS; the #1018 expected iOS
memset rows unchanged), `run-wasm-gates.sh --without-v8-spill --without-native`, and the worklet
chain (`build-web-audioworklet.sh --named-twin` into fresh empty directories,
`strip-wasm-names.py --self-test` and `check`, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`check-scalar-oracle-absent.py --wasm` and `--native`, `test-web-audioworklet.sh`). The shipped
worklet module is `1a833326…c397d7` (3,052,846 B; attempt 3's was `e25b045d…90d9f652`; *Amendment 5,
root ruling D:* that digest is `feefbe166`'s module, the verifier's rebuild found; `c7f7bbdfb`'s
own record commit moved panic-location line numbers and ships `d7665dbb…6465f5`; the final
digest is recorded at the batch boundary): its bytes
change with the bound code linked into preparation, as `artifact-identity` will report; no audio
bit moves (wasm G5 digests, builtins PCM fixtures, `audit capi`'s `pcm_digest`). AArch64 is
CI-only.

### Attempt 5 (2026-10-06, branch `codex/d15-stream-g`, under Amendment 5; the last permitted attempt)

Implementation and Amendment 5 `590201615`; this record in the commit after it. Scope exactly
Amendment 5's list. Release measurements on this host (x86-64-v3).

**MJ1.** `prepare_session_builtins_with_live_controls` passes `builtins::input_section_bounds` only
the strips without a live input lane (in strip order) and takes the live bound for a live strip;
a live strip's design bound is never computed. The seal, the seal check (`expected_tails`),
`input_bounds()` and every fixture are unchanged (`check-builtins-fixtures.sh`, `graph_fixture
--check`, `resources.jsonl` untouched). `builtins::tail::fixed_input_bound` counts its runs in a
thread-local counter compiled only under `test` or `builtins/test-support`, read through
`builtins::test_support::fixed_input_bounds_computed()`; the new builtins-compiler test
`design_bounds_are_computed_only_for_strips_without_a_live_input_lane` (test-support, so
`test-debug-a` runs it) prepares the nine-track EQ fixture with six distinct designs (eq0-eq2
shared, eq3 HPF, eq4 LPF, eq5 trim, eq6 right HPF only, eq7-eq8 filters off) and counts 6 with no
strip live, 0 with every strip live, 4 with eq3 and eq4 live, each preparation passing the seal
check.

Browser boot, `node --no-liftoff scripts/web-mixing-automation-benchmark.mjs rebuild-round MODULE
1|2` (the #1289 rebuild-cost proxy: Node v22.23.2, pinned with `taskset -c 7`, 25 boots per
document, every boot audible, 0 failed), rounds run alternately pre/head. Boot p50 in ms, round 1
/ round 2:

| module | 9-track EQ | 64-track console | 64-track app shape | 64-track sends |
|---|---|---|---|---|
| pre-#1329 `0725a8949` (`10a3c816…`, from a `git archive` export, deleted after) | 2.15 / 2.17 | 20.27 / 20.38 | 19.72 / 19.81 | 33.73 / 34.00 |
| attempt 5 `590201615` (`ec1d2f66…`, 3,053,802 B) | 2.25 / 2.22 | 20.40 / 20.19 | 19.89 / 19.69 | 34.39 / 33.69 |
| *(attempt 4 `c7f7bbdfb`, the verifier's table)* | 2.46 / 2.49 | 35.41 / 35.86 | 35.02 / 35.34 | 49.20 / 50.53 |

The three 64-track documents are within noise of pre-#1329 (round-to-round spread larger than the
difference). The 9-track document is about 0.07 ms (3 %) above pre-#1329, as in the verifier's
patched row (2.25 / 2.23); its source is not investigated (candidates: the live bound's 0.04 ms
and the larger module). Records kept outside the tree (scratchpad `rebuild-{pre,head}-{1,2}.json`).

**m1.** `live_input_lane_reports_the_live_bound_and_plain_input_its_own` now forges the live
strip's entry to its design bound (`InputSectionBound::ZERO`, both filters off) in `tails` and
`seal.tails` alike and expects `builtin.prepared.tail_set` from `validate_for_session`.

**m2.** `each_strip_is_bounded_by_its_own_design_when_designs_are_shared` gains a sixth strip, the
first with only its right HPF at 10 Hz; its own bound differs from the first strip's.

**m3, NITs.** Test value restated (above). `math::tail`: the `nu` text now derives `4.26 u
||R||_2` from `|1 - 2 c1|, |2 a2|, |1 - 2 a3| <= 1` (for `k >= 0`), three roundings per word and
the `sqrt(2)` of the 2-vector, below `nu = 8 u ||R||_2` by a factor above 1.8, and states the
premise; `STEP_UP`'s doc gives the per-summand argument: each summand of every step reaches the
result through at most two roundings before the product with `1 + 4u` (in `out.input[1]`, `|h|`
and the output rounding pass two additions, `gamma_1 e` a product and one addition), so
`(1 - u)^3 (1 + 4u) >= 1`; the one exception, the radius's growth term `nu (|s_1| + |s_2|)` (three
roundings), falls short by at most `10 u^2` relative, which `nu`'s margin absorbs. The output
rounding's `8 u` is stated against at most six roundings per term (`6.01 u`). The derivation's
"Numerical limits" says the same. No constant changed. The attempt-4 digest is marked
`feefbe166`'s; the superseded gate 2, 3 and 7 lines are struck through with a pointer; the
#1457 row is 21. Root rulings A (#1457 "Where preparation runs"), B (`HostLiveLanes::strip_input`
doc) and E (`effect-floor-accounting.md`'s citation, marked as a citation update) are done.

**Mutation runs** (each applied in the worktree, the named test run, the file restored; RED = the
test fails):

| defect | test | result |
|---|---|---|
| preparation bounds every strip's design and discards the live strips' (attempt 4) | `design_bounds_are_computed_only_…` (debug) | RED: 6 against 0 (every strip live); every other builtins-compiler test GREEN |
| `input_section_bounds` bounds once per strip, not per distinct design | `design_bounds_are_computed_only_…` | RED: 9 against 6 |
| the seal check skips its live rule (verdict E2) | `live_input_lane_reports_…` (debug) | RED; every other builtins-compiler test GREEN |
| the design key drops the right channel (`[lanes[0], lanes[0]]`, verdict C3) | `each_strip_is_bounded_…` | RED (`input_section_bounds` hands the sixth strip the first's bound) |

**Certified figures.** None moved: release `tail_contract` prints the attempt-4 values (for
example HPF 10 Hz into LPF 22049.48 at +24 dB 416,117; live 44.1 kHz `T_decay` 904,785, `R(P*)`
and `T_rest` 1,081,764; the other rates as recorded). No audio bit moved: wasm G5 0 mismatches
(143 cases, 252 comparisons), `audit capi` `pcm_digest` `cb10fbface44a3a4`, builtins fixtures
green. No fixture or digest re-pinned.

**Gates run (all green):** `cargo fmt --all -- --check`; release `tail_contract
--include-ignored` (9 passed, 6.9 s); `cargo clippy --locked --workspace --all-targets -- -D
warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `cargo test
--locked -p builtins-compiler --no-run`; `test-debug-a` (1,442 passed, 0 failed, 10 ignored) and
`test-debug-b` (888 passed, 0 failed, 25 ignored) with `qualification.yml`'s exact commands;
`conformance_fixtures --check`; `check-workspace-policy.sh`, `check-realtime-policy.sh`,
`check-lane-policy.sh`, `check-builtins-policy.sh`, `check-dsp-research.sh`,
`check-effect-contract.sh`, `check-ci-path-routing.py`; release build of `audit bench capi
session-validator`; `audit capi` (0 allocations, 0 deallocations, 0 syscalls, 0 violations);
`cargo test --release -p audit -p bench -p console-workload`; `check-capi-abi.sh` and
`--self-test`; `check-scalar-oracle-absent.py --native` and `--wasm`; `graph_fixture --check`;
`check-graph-determinism.sh`; `check-builtins-fixtures.sh`; `check-cross-targets.sh`; the worklet
chain (`build-web-audioworklet.sh --named-twin` into recreated empty directories,
`strip-wasm-names.py --self-test` and `check`, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`test-web-audioworklet.sh`); `run-wasm-gates.sh --without-v8-spill --without-native`. The shipped
module at `590201615` is `ec1d2f66…a58cddfb` (3,053,802 B); this record commit moves no source
line, and the final digest is recorded at the batch boundary (root ruling D). AArch64 is CI-only.

### Follow-up C (verdict follow-ups and root correction)

Attempt 5 PASSED (`/home/bl/misofm/submix-verdicts/1329-attempt5.md`; `590201615`, `fadc45df0`)
with two MINOR findings, three NITs and items ROOT-1 to ROOT-4. This follow-up folds them on
`codex/d15-stream-g`. It is not an attempt.

**Root correction of Amendment 5's ruling A (2026-10-06, binding).** Made by the decision-15 root
coordinator under the owner's no-shortcuts delegation (`no-shortcuts-correctness-first`), in the
context of decision 15 D15-4(b). Ruling A's premise was wrong. A browser boot **with live
controls** attaches an input lane to every strip and bounds no design (each strip reports the live
bound). An **audio-only** browser boot (the SDK default: `commandQueueRecords ?? 0`, so no control
requests and no input lanes) bounds every distinct design on the AudioWorklet thread until stream
H's #1332 moves preparation to a Worker. The C ABI bounds every distinct design on its control
thread. #1457's budget binds both hosts, and its gates include the audio-only browser boot
(measured +15.5 ms, +81 %, per 64-track boot today: console 19.13 -> 34.66 ms p50; the verifier's
table is in #1457's root ruling R2). Ruling A and the MJ1 summary above are marked as corrected,
not rewritten.

- **m1 / ROOT-1.** #1457: "Where preparation runs" restated (the Amendment 5 paragraph struck,
  the Worker-after-#1332 statement restored), D1 binds both hosts, gate 4 adds the audio-only
  browser boot, and root ruling R2 records the correction and the verifier's table. The comment
  in `crates/builtins-compiler/src/lib.rs` above the design-bound computation now says that a
  browser boot with live controls computes none, and that an audio-only browser boot and the C
  ABI bound every distinct design.
- **m2 / ROOT-2.** `each_strip_is_bounded_by_its_own_design_when_designs_are_shared` gains a
  seventh strip that differs from the first only in its left HPF (10 Hz), with
  `assert_ne!(own[6], own[0])`. Mutation evidence (release, `cargo test --locked --release -p
  builtins --features builtins/test-support --test tail_contract -- --include-ignored
  each_strip_is_bounded`), each mutant applied to `input_bound_key` in
  `crates/builtins/src/tail.rs` and reverted:

  | mutant | result |
  |---|---|
  | none (shipped key) | GREEN |
  | M5, key drops the left channel (`[lanes[1], lanes[1]]`) | RED: `input_section_bounds` hands the seventh (left-only) strip the first strip's bound (`T_decay` 192 against 19,051) |
  | C3, key drops the right channel (`[lanes[0], lanes[0]]`) | RED: the sixth (right-only) strip gets the first strip's bound |
  | revert | GREEN |

  The test-value line "including either channel" is now true and is restated above.
- **NIT, ROOT-3 (live bound's boot cost).** Recorded in #1457's R2: about 0.07 ms per live-control
  browser boot (9-track 2.21 / 2.25 against 2.15 / 2.16 ms; a variant with the bound as a constant
  boots like pre-#1329); a per-rate table or #1457's cache removes it.
- **NIT, gate 3 strike-through.** Only the lower-side sentence is struck; the upper-side claim is
  visible.
- **NIT, effect-floor citation.** `docs/rulings/effect-floor-accounting.md` also names the
  control-plane read of `enabled` in the response-snapshot builder
  (`InputStage::copy_response_snapshot_lane`, `crates/builtins/src/lib.rs`). A citation update,
  not a ruling change.
- **ROOT-4** (final module digest and the GitHub bodies of #1329 and #1457) stays with the batch
  boundary.

No audio bit moved and nothing was re-pinned: the only code changes are one comment and one test
strip.

**Gates run (all green):** `cargo fmt --all -- --check`; release `cargo test --locked --release -p
builtins --features builtins/test-support --test tail_contract -- --include-ignored` (9 passed,
7.0 s); `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; debug `cargo test --locked
-p builtins-compiler` and `-p builtins` (both with `builtins/test-support`; 195 passed, 0 failed);
`check-workspace-policy.sh`; `check-lane-policy.sh`.
