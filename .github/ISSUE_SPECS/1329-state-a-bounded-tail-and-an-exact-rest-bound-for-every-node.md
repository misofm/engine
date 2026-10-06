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
    composed by PDC, the C ABI and browser reports, #1261, #1262); **silence skipping uses
    `T_rest`** (#1107). `TailSamples::Finite` carries `T_decay`, and its doc and
    `docs/EFFECT_CONTRACT_V1.md` state the peak range `P >= P*` and the exact-zero branch from
    `T_rest` below it. `T_rest` is stored and exposed beside `T_decay` wherever `T_decay` is
    (`InputBuiltins`, `BuiltinChain`, `input_section_live_bound`); the implementer names the field
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
3. Plumbing: `builtins-compiler` and `graph-compiler` on `TailSamples`; `BuiltinTail` removed.
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
  the live bound, summed along a path. Values here are below 2.6M (Amendment 2); that figure is the
  envelope of this slice's values, not a limit the documents name, and which limit it was meant to
  protect is open for root (Amendment 3, G5). Gate 7 holds the headroom.
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
   (one run with polarity inverted, so `-0.0` is reached), and the eight SVF integrator words
   equal those of a freshly reset `InputBuiltins` with the same targets (`Z`), under `f32` `==`.
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
   tail-entry/`InputBuiltins` size delta.
6. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml` (it covers
     `builtins-compiler`, `graph-compiler`, `host-core` and `capi` with their `test-support` features)
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-effect-contract.sh`, `bash scripts/check-dsp-research.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo build --locked --release -p audit && ./target/release/audit capi` (`qualification.yml:715`)
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`
7. *(Amendment 3, G5.)* **Headroom.** In `tail_contract.rs`, at every launch rate: `T_decay` and
   `T_rest` of `input_section_live_bound(rate)` are below 10,000,000, the smallest finite
   `maximum_finite_tail_samples` the tree sets (a strip's builtin path has one input section, so
   one term); and `any_sanitized_input` is below 2,600,000, the envelope "Hazards" states. Record
   each headroom per rate in the evidence. If root names another limit behind 2.6M, it is added
   here.

## Test value

- Gate 1(a): a tail computation that drops the trim gain or the cascade's second section falls
  below `T_b(eps / 2)`, and a needlessly loose one (a Putzer factor) exceeds `T_b(eps * 2^-5)`; no
  test computes a tail today.
- Gate 1(b): a live bound evaluated at the wrong extreme (for example the minimum cutoff, which
  is not the slowest pole) is exceeded by a scanned design.
- Gate 2: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel at the worst-case input, and a state that settles at a non-reset value (a
  limit cycle the flush misses) fails the fresh-instance comparison; gate 1 only checks `f64`
  arithmetic.
- Gate 1(c): a `T_rest` that drops its `R(P*)` term (reports `T_decay` as the time to exact zero)
  is beaten by the real kernel at small peaks (attempt 3: 346 frames against 175 for a 1 kHz
  section); no other gate drives peaks below `P*`.
- Gate 7: a bound that grows past a configured tail cap, or past the stated envelope, turns red
  before a plan is refused at preparation.
- Gate 3: a sound but uselessly loose bound (for example a Putzer `m * rho^m` factor where the
  eigenvector bound applies) breaks the restated D15-4 figures, and an unsound one (a dropped A9
  term) falls below the measured real rest.

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
