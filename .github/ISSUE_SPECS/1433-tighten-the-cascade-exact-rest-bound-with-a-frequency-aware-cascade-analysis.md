# Tighten the cascade exact-rest bound with a frequency-aware cascade analysis

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b), D15-4(c)).
Filed 2026-10-05 as the successor that *State a bounded tail and an exact-rest bound for every
node* (#1329) names in its Amendment 2 (finding F2, option (iii)). Code anchors are those #1329
creates; verify them on the branch where #1329 has landed before starting.

## Product outcome

A strip with a live input lane reports a shorter, still certified, exact-rest bound. #1329 ships a
sound but crude bound for the HPF→LPF input cascade (about 1.19M samples at +24 dBFS, against a
measured real rest of 983,374 at 44.1 kHz). After this slice the certified
`RestSamples::peak_plus_24_dbfs` of `input_section_live_bound(rate)` is within a stated margin of
the measured real rest at every launch rate, and never below it. No kernel, law or render path
changes: only the control-plane bound and the figures that quote it.

## Context

- **What #1329 ships (its Amendment 2).** The live bound treats the cascade crudely: the HPF's
  output is bounded by a universal invariant ball over every reachable recursion word,
  `Y_H ≈ 7.7e4 · g · P` after a top-to-low retarget, and that ball is the LPF's input, so the LPF's
  ball is `B_L ≈ 4.2e9 · g · P` (`g = 10^(24/20)`). Certified (estimates from #1329 attempt 2):
  `R(+24 dBFS) ≈ 1.19M`, any sanitized input `≈ 2.5M`, `T_live ≈ 640k` (each `R` plus
  `2 · N_SILENCE`, below). Rest is proven in sequence: the HPF rests first (its input is exactly
  zero in the tail; stall at most `3.6e-16`), then the LPF. The HPF's universal stall carried
  through the LPF gives an LPF stall radius `3.8e-11`, far above `REST_EPS`, so the LPF's rest cannot
  be proven before the HPF's. *(#1329 Amendment 3 restates the certified values, unsampled, per
  rate: `peak_plus_24_dbfs` 1,264,736 / 1,257,840 / 1,268,585 / 1,262,029 and
  `any_sanitized_input` 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156 at 44.1 / 48 / 88.2 / 96 kHz;
  live `T_decay` about 905k and `T_rest` about 1.08M. Gate 1's "crude values" are these.)*
- **The A9 term.** *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328,
  Amendment A9): a section may take the joint `REST_EPS` flush only after its effect input has been
  exactly zero for `N_SILENCE` frames (1024 at every launch rate unless #1328's measurement states
  otherwise). Every exact-rest bound carries `+N_SILENCE` per section in sequence, `+2 · N_SILENCE`
  for the pair. This slice keeps that term exactly; it tightens only the decay part.
- **Measured (#1329 attempt 2, release, real kernel, before A9).** Top pair (HPF one ulp below the
  maximum into LPF at the maximum, alternating input at +24 dBFS): exact rest at 983,374 samples
  (44.1 kHz) and 978,319 (48 kHz); LPF at the maximum alone 925,361. `|h|_1 = 0.697` for the pair.
- **Why the crude bound is loose.** The universal ball ignores frequency: the HPF's output has
  little energy near the LPF's slow pole, and a retarget does not move a state that sits at the
  section's DC equilibrium, so `Y_H · B_L` overstates the LPF's state at `N` by orders of
  magnitude. Each factor of ten in the LPF's initial state costs about `ln(10)/(1 - q) ≈ 44,000`
  samples at the top of the domain (`1 - q ≈ 5.2e-5`).
- **Code (created by #1329).** `crates/math/src/tail.rs` (certified `f64` bounds for TPT SVF
  sections and cascades), `crates/builtins/src/tail.rs` (`input_section_live_bound`,
  `input_section_worst_case_pair`), `crates/builtins/tests/tail_contract.rs` (gates 1-3 of #1329),
  `docs/derivations/1329-input-section-tail-and-rest.md`.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled on #1329's finding F2 with
  option (iii): #1329 ships the crude-but-sound cascade bound and restates the D15-4 figures to its
  certified values plus `2 · N_SILENCE`; this slice then tightens the bound with a frequency-aware
  cascade analysis. Reason: a sound, looser bound is correct; tightening only shortens reported
  tails, which is a cost and not a correctness gap, so it is a successor issue and not a
  placeholder in #1329. Stream G files and owns it, after #1329.
- **D1. Method (sketch from #1329 attempt 2; the derivation proves it or the slice stops).**
  - Change variables per section to the deviation from the DC equilibrium: `z = s - e2 · v`, where
    `s` is the two-word integrator state, `v` the section input and `e2` the unit vector of the
    second integrator. For every design word `w` of the shared-`k` family, `G(w) b(w) = e2` (the
    DC state per unit input is `e2`), so a retarget does not kick `z`; it moves only through the
    input's change.
  - Write the section output in the TPT trapezoid form `c = ½ (m1, m2)(I + A)` (with `A` the step
    matrix and `m1`, `m2` the mix words), so the output row is bounded with the same `V`-norm as the
    state.
  - Bound the LPF's state at `N` by telescoping the HPF's output differences through the LPF's
    recursion instead of by `Y_H · B_L`. The result is a frequency-aware bound for the cascade
    under every history #1329's D5 admits (#1407's words, 64-sample ramps, trim `+24 dB` by #1408).
  - Estimated result: `R(+24 dBFS) ≈ 1.02M-1.07M` before the A9 term.
  - The rate inflation, the reset-aware exact reference and the sequential rest (HPF first) stay as
    #1329's Amendment 2 states them. The bound remains computed at preparation on the control
    thread, in `f64`, with `math::log`/`math::exp`.
- **D2. Never below the measured value.** A certified bound below a real-kernel rest is unsound and
  stops the slice. The gate measures the real rest with the A9 law in place, not the pre-A9
  figures above.
- **D3. Figures follow the certified values.** The implementer restates decision 15's D15-4
  rationale figures and #1329's recorded `T`/`R` values to the new certified values (each `R` with
  `+2 · N_SILENCE`), on this slice's branch (root-authorized). Nothing is pinned: tests compare the
  computed values with measured and brute-force references.
- **D4. Stop rule.** If the derivation cannot prove the telescoped bound, or the certified value
  misses gate 2's margin, stop and report the measured ratio to Sol. #1329's crude bound stays in
  production; it is sound. There is no fallback value and no gate change.

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF state step `s[n+1] = A(w) s[n] + b(w) v[n]`, output
  `y[n] = c(w) s[n] + d(w) v[n]`; deviation `z = s - e2 · v` with `G(w) b(w) = e2`; cascade input
  `v_L = y_H`; D1.
- **Coefficient and update rules:** unchanged from #1329, #1407 and #1408 (designed `f32` words,
  64-sample ramps through designs and their mixtures).
- **Numerical limits:** the `f32` rate inflation `7 · 2^-24 · κ` (`μ_s = 1.007e-6` at the top) and
  the margin `q_ramp + μ_s < 1` by at least `3.69e-5`, as #1329's Amendment 2 records (certified
  `3.673e-5` at 44.1 and 88.2 kHz, `3.701e-5` at 48 and 96 kHz, #1329 Amendment 3); the bound is
  valid for every reset pattern of the joint flush.
- **Latency and tail:** latency 0; the reported tail `T` may shrink with the same analysis but must
  not grow; exact rest `R` is the target of this slice.
- **Units and smoothing:** samples at the plan's rate; peak in linear full-scale units.
- **Denormal/NaN:** unchanged; a non-finite state is reset by the per-block check and never starts
  a tail.
- **Citations:** [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP], [SMITH-SASP] (as in #1329), and
  N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., for the rounding model.
- **Fixtures and objective tests:** gates 1-3. **Benchmarks:** none (control-plane only; the bound
  is computed once per preparation). **Listening:** none; no rendered bit moves.

## Candidate (root, 2026-10-06; #1329 Amendment 3, G3)

Not a frozen decision: a method this slice may adopt if its derivation proves it and it meets
D2-D4. **A two-level `P*`** (#1329 attempt 3's option 3). #1329 states two tail values per node:
`T_decay` (the bound for every peak `P >= P*`) and `T_rest = max(T_decay, R(P*))`, where `P*` comes
from the per-word flush stall and `R(P*)` is the exact-rest bound at `P*`. In a cascade whose slow
LPF follows a section with a large output row, the HPF's flush stall, amplified by the LPF, raises
`P*` (about `5e-9` for a 10 Hz HPF into the LPF at the maximum, 44.1 kHz, 0 dB), and `R(P*)` then
exceeds `T_decay` by far (498,399 against 362,155). The HPF rests first, so after its rest only the
LPF's own stall applies: a second `P*` for the LPF alone would bring the cascade rows of #1329
attempt 3's table back near `T_decay` (that table: 1 kHz LPF 174 / 3,842; 10 Hz HPF into 1 kHz LPF
18,271 / 20,373; 10 Hz HPF into LPF max 362,155 / 498,399; 1 kHz HPF into LPF max at +24 dB
415,269 / 462,674; decay part / rest at `P*`, 44.1 kHz). Every enabled section still keeps
`T_rest >= N_SILENCE`. If adopted, it shortens `T_rest` only; `T_decay` must not grow, and #1329's
gate 1(c) (`T_rest` against the real kernel's rest) must stay green.

## Deliverables

1. The frequency-aware cascade bound in `crates/math/src/tail.rs`, composed for the input section
   in `crates/builtins/src/tail.rs`; `input_section_live_bound` returns it.
2. The derivation (D1) in `docs/derivations/1329-input-section-tail-and-rest.md`, as a new section
   that replaces the crude cascade step and keeps the rest of #1329's derivation.
3. Gates 1-3 as tests in `crates/builtins/tests/tail_contract.rs`.
4. Figures restated per D3: decision 15's D15-4 rationale line, and the `T`/`R` values recorded in
   `docs/BUILTINS_AND_METERING_V1.md` and `dsp-research/filters.md` where #1329 put them.
5. If `T` changes: the tail re-pins of #1329's gate 5 (`fixtures/graph/v1/direct-route.*` with
   `graph_fixture`, `ZERO_DELAY_CANONICAL_SHA256`), one at a time, the only moved bytes being the
   `finite:<T>` values; and the finite tail values in the `builtins-compiler` tail unit tests and
   `crates/host-core/tests/live_lanes.rs`.

## Authorized paths

- `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs`
- `docs/derivations/1329-input-section-tail-and-rest.md`, `docs/BUILTINS_AND_METERING_V1.md`,
  `dsp-research/filters.md` (the tail and rest figures only)
- `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (the D15-4
  rationale figures only)
- Only if `T` changes: `crates/builtins-compiler/src/lib.rs` (tail unit tests only),
  `crates/host-core/tests/live_lanes.rs` (tail assertions only),
  `crates/graph-compiler/tests/track_delay.rs` (the digest), `fixtures/graph/v1/direct-route.*`
- this spec

`builtins-compiler`, `graph-compiler`, `host-core` and `fixtures` sit outside stream G's column;
coordinate with stream F (#1261, #1262) as #1329 does.

## Non-goals

- Any kernel, flush or rest-law change (#1328 owns the A9 law and `N_SILENCE`); any change to the
  retarget law (#1407) or the ramp law (#1408).
- The fixed-design bound (no live input lane) of #1329's D4, unless the same analysis applies
  unchanged; the EQ's and the multiband crossover's cascades (#1372, #1373) may reuse the helper in
  their own slices.
- Tightening `any_sanitized_input` beyond what the same analysis gives; it must only not grow.
- Rejected alternatives:
  - Lower the figures without a proof (a sampled maximum): unsound; D2.
  - Keep #1329's crude bound and stop: correct but costs reported tail length; this slice exists
    to remove that cost (D0).

## Hazards

- The bound must hold for every reset pattern of the joint flush and every history of #1329's D5,
  not only the worst-case pair; gate 1 scans the domain for that reason.
- A shorter `T` moves canonical plan text and digests (Deliverable 5); any other moved byte stops
  the slice.
- `maximum_finite_tail_samples` can only admit more plans when values shrink; no new refusal.

## Objective gates

1. **Live bound still covers the domain** (`tail_contract.rs`, every launch rate in release, 48 kHz
   in debug): #1329's gate 1(b) scan (100,000 log-spaced cutoffs plus the last 65,536 `f32` values
   below each rate's maximum, both sections) passes against the new `input_section_live_bound`.
   The new `peak_plus_24_dbfs` and `any_sanitized_input` are each at most the crude values #1329
   recorded per rate in its evidence, and the tail `T` is at most #1329's recorded `T`.
2. **Tightness and soundness against the real kernel** (release, every launch rate): measure the
   real exact rest `R_meas` of the top pair (`input_section_worst_case_pair(rate)`) at +24 dBFS, as
   #1329 attempt 2 did (alternating input, `+24 dB` trim, the A9 law in place; first sample from
   which every output sample is `±0.0` and the eight integrator words equal the reset state).
   Assert `R_meas <= peak_plus_24_dbfs <= 1.15 · R_meas` (the stated margin: 15 %, root decision
   2026-10-05: the estimate `1.02M-1.07M` plus `2 · N_SILENCE` sits within 1 % of a 10 % ceiling,
   so a sound, useful result could fail on calibration rather than substance; the issue's purpose is
   to tighten, and the reported ratio keeps it honest). Record `R_meas`, the certified value and the
   achieved ratio `peak_plus_24_dbfs / R_meas` per rate. A certified value above the margin is a finding for Sol (D4), not a
   gate change.
3. **#1329's gates unchanged and green:** its gate 1(a) (fixed design), gate 2 (soundness on the
   real kernel, with the new `R` as the rest sample) and gate 3 (the restated figures, now the new
   certified values per D3).
4. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features math/lane,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml` (only if
     Deliverable 5 applies)
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-dsp-research.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a tightened bound that holds at the worst-case pair but not over the whole domain (for
  example one that assumes the HPF output is high-passed at the pair's cutoff only) is exceeded by
  a scanned design; #1329's crude-value comparison catches a change that loosens instead.
- Gate 2: an unsound tightening (a dropped `2 · N_SILENCE`, a missed reset pattern, a telescoping
  step that loses a boundary term) certifies below the real kernel's rest; a step that does not
  tighten (the universal ball left in place) misses the 15 % margin. No test compares #1329's
  certified rest with the measured one from above.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329)

## Attempt record

### Attempt 1 (2026-10-06, branch `codex/d15-stream-g`)

**What changed.** `math::tail::envelope_cascade` (#1329's crude live cascade) is replaced by
`math::tail::live_cascade`, with `PoleDomain`, `LiveCascade`, `live_zones` / `LiveZones` /
`PoleZone` and `pole_real`. `builtins::input_section_live_envelope` keeps its name (the export list
in `builtins/src/lib.rs` lies outside the authorized paths) and now returns the whole
`LiveCascade` (#1329's envelope plus the pole domain, the HPF mix row and its output rounding);
`input_section_live_bound` and `input_section_live_cascade` read it. No kernel, law, render path or
fixed-design bound changes. The derivation is the new "#1433: the frequency-aware cascade"
section of `docs/derivations/1329-input-section-tail-and-rest.md` (D1, in short):

* **Poles.** For any words, `R A R^-1 = (Re p) I + (Im p) J + kappa K0` exactly, so the section
  is a complex one-pole up to `sqrt(2)|kappa|`; exact designs lie on the circle
  `|p + i| = sqrt(2)` and their hull under its arc. A reachable word is a hull point moved by
  `E + h` (#1407), a settled one by `h`.
* **Zones and moves.** 146 zones of `Re p` (widths from `1e-6`, growing by 1.5 up to 0.02, from
  each end to 0), each with certified `rho`, `||I + A||`, `||b|| + mu_input` and settled `r`,
  `||I + A||`, `||A^2 - I||`; a frame moves `Re p` by at most `(high - low + 2 eta) / 64 + 32 u`
  (one ramp step), or replaces the word with zero state.
* **`Phi` (state by zone)** and **`Psi` (potential)** as least fixed points over neighbouring
  zones; the HPF's output `1/2 (m1, m2)(I + A) s` (the TPT trapezoid form of D1) then has a
  weighted mass that telescopes (Abel summation) to `sup Psi Phi` plus per-frame charges.
  Zones at `Re p >= 0` are charged directly (their `Psi` would be the frequency-blind LPF's
  `2 / (1 - rho_low)`).
* **Settled.** From `N + 64` the LPF in `tau = sigma - e2 y_prev` (D1's `z = s - e2 v`, with
  `b = (I - B) e2` exactly) is driven by the HPF's output change `1/2 (m1, m2)(A^2 - I) s`,
  small at both ends of the domain; per quarter-octave zone group a four-term non-negative
  system, with one-off joint-flush terms for either section.
* **Rest** stays sequential (HPF, then LPF), each with `+ N_SILENCE` (the A9 term kept exactly).
  The candidate two-level `P*` was not needed: with the frequency-aware stall, `P*` is about
  `1.2e-7` and `R(P*)` already lies below `T_decay`, so `T_rest = T_decay` live.

**Soundness finding (fixed in this attempt, recorded for #1329).** `output_rounding` has
`|1 - c1|` and `|1 - a3|` terms, so #1329's `omega_state` at the largest words is not a supremum
(a 2,848 Hz HPF exceeds it); #1329's envelope row absorbs that in its slack (`gamma` at 10 Hz,
`row_correction`), but #1433 needs `omega` alone, so it takes its value at the largest words plus
its value at the opposite corner. Gate 1(b) now checks it on every scanned design (mutation M4).

**Figures (certified; 44.1 / 48 / 88.2 / 96 kHz), against #1329's crude bound and the real
kernel.**

| value | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz | crude (#1329) |
|---|---|---|---|---|---|
| `T_decay` | 704,010 | 699,952 | 704,018 | 699,960 | 904,785 / 899,524 / 904,795 / 899,533 |
| `T_rest` | 704,010 | 699,952 | 704,018 | 699,960 | 1,081,764 / 1,075,575 / 1,085,641 / 1,079,792 |
| `P*` | 1.2025e-7 | 1.1959e-7 | 1.2037e-7 | 1.2213e-7 | about 1.7e-3 |
| `R(P*)` | 693,189 | 689,433 | 697,084 | 694,069 | -- |
| `peak_plus_24_dbfs` | 1,067,207 | 1,061,497 | 1,071,057 | 1,065,688 | 1,264,736 / 1,257,840 / 1,268,585 / 1,262,029 |
| `any_sanitized_input` | 2,384,997 | 2,372,008 | 2,388,800 | 2,376,147 | 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156 |
| `R_meas` (exact frame) | 983,374 | 978,319 | 983,374 | 978,319 | -- |
| ratio `R / R_meas` | 1.0853 | 1.0850 | 1.0892 | 1.0893 | 1.29 (44.1 kHz) |

Each `R` includes `2 N_SILENCE`. `R_meas` is gate 2's: the top pair, alternating +24 dBFS input
through the +24 dB trim for 1,000,000 frames, A9 in place, the first frame from which every
integrator is `+-0.0` (the run repeated with its last blocks frame by frame). The real 1e29 rest
is 2,236,997 / 2,225,069 / 2,237,005 / 2,225,077 (64-frame resolution), below
`any_sanitized_input`; the real last `|y| >= P eps` after `N` is at most 436,362, below `T_decay`.

**Independent recomputation.** `tail_contract`'s rewritten oracle (plain `f64`, no shared code:
pole domain and mix row from first principles, zones, `Phi`, `Psi` by plain iteration, the window
frame by frame, the settled phase in closed form) and a 60-digit decimal recomputation of the same
derivation (`verify_1433.py`, from the module's terms as exact bit patterns; `tan` and `pi` in
decimal) agree: the module is 18-19 frames above in `T_decay`, 17 in `R(P*)`, 18-19 in
`peak_plus_24_dbfs`, 42-43 in `any_sanitized_input`, and `P*` within `1e-6` relative, at every
rate, as its `1 + 2^-30` inflations predict.

**Tests (new or rewritten) and their test value.**

* `live_bound_covers_every_scanned_design_and_ramp_word` (gate 1): adds every scanned design and
  every real-kernel ramp word (including per-frame disable/re-enable/retarget histories) inside
  its zone's constants, the HPF mix row and output rounding, and the per-frame pole step
  (largest observed 0.031233 against the stated 0.031235 at 44.1 kHz). Catches a zone constant
  taken at the wrong end, a wrong step, an `omega` that is not a supremum (M1-M4).
* `live_bound_carries_every_term_an_independent_recomputation_requires`: the oracle above.
  Catches a dropped charge, a dropped `N_SILENCE`, a frequency-blind settled drive (M5-M7), which
  the real kernel cannot see.
* `live_bound_holds_on_the_real_kernel_at_the_domain_extreme` (gate 2): adds
  `R_meas <= peak_plus_24_dbfs <= 1.15 R_meas` on the exact rest frame. Catches an unsound
  tightening (rest stopped after the HPF: below the real rest, M8) and one that does not tighten
  (the universal ball left in place: ratio 1.2142, M9).
* `live_rest_bounds_are_within_the_restated_contract_figures` (gate 1's ceilings and #1329's
  gate 3 restated): every value at most #1329's per rate, `R` at most 1,075,000 and 2,390,000.
  Catches a looser settled decay (M10).
* `live_tail_every_peak_is_the_rest_at_the_flush_floor`: rewired to `live_cascade`. Catches a
  `T_rest` taken from the wrong rest (M11).

**Mutation runs** (each applied, the named test run, the tree restored; all named tests green
on revert):

| defect | test | result |
|---|---|---|
| M1 zone contraction at the inner end only | live scan | RED: a 12,060 Hz HPF above its zone's contraction |
| M2 pole step halved | live scan | RED: a ramp word moved 0.0312347 > 0.0156193 |
| M3 `\|1 + p\|` taken at the zone's low end | live scan | RED: the 10.3 Hz HPF's `\|\|I + A\|\|` above its zone's |
| M4 `omega_state` at the largest words only | live scan | RED: the 3,100.7 Hz HPF's mix row exceeds the live terms |
| M5 the potential's per-frame charge dropped | oracle | RED: `T_decay` 703,120 below the recomputed 703,991 |
| M6 the second section's `N_SILENCE` dropped | oracle | RED: `R(P*)` 689,425 below 693,172 |
| M7 settled drive by the output, not its change | oracle | RED: `P*` 4.6e-5 against 1.2e-7 |
| M8 rest stops after the HPF | gate 2 | RED: integrators not at rest at `N + R = 972,956` |
| M9 the universal ball left in place at `N` | gate 2 | RED: ratio 1.2142 > 1.15 |
| M10 settled decay at `rho_ramp` | gate 3 | RED: `peak_plus_24_dbfs <= 1_075_000` |
| M11 `T_rest` from the +24 dBFS rest | live `T_rest` identity | RED |

**Cost.** Preparation computes the live bound once per session with a live input lane: native
(release, one pinned core, best of 50) 42 us before, 240-254 us after (zones about 85 us, the rest
the settled zone groups, about 60 after a quarter-octave grouping that leaves `R` unchanged and
adds about 1,100 frames to `T_decay` against per-zone systems). Browser boot with live controls
(`web-mixing-automation-benchmark.mjs rebuild-round`, two rounds alternated, base `38ef4fe7a`
against this attempt's final module, p50): nine-track EQ 2.219 / 2.215 ms -> 2.463 / 2.476 ms;
sixty-four-track console 20.28 / 20.21 -> 20.55 / 20.62 ms; app shape 19.73 / 19.73 -> 20.07 /
20.13 ms; sends 33.90 / 33.77 -> 34.19 / 34.20 ms. An audio-only boot computes no live bound. The
shipped worklet module grows by 16,822 bytes (3,056,452 -> 3,073,274). #1457 tracks the
preparation budget.

**Wasm memory (found by the worklet chain).** A first version kept two power tables per zone
group alive for the whole bound (about 340 KB); the browser fixture's exact `memoryBytes` then
grew by five pages (1,376,256 -> 1,703,936, `check-browser-expected-resources.py` red). Each
table is now built for one group and dropped, and the fixture's rows are back at their pins (the
gate is green). The bound's peak allocation is therefore observable in that pin: a later change
that holds more at once moves it.

**Deliverable 5** does not apply: fixed-design bounds (and so every graph fixture tail token and
digest) are unchanged; the live `T` is read, not pinned, by `live_lanes.rs` and the
`builtins-compiler` tests.

**Gates run (all green on the final tree).** `cargo fmt --all -- --check`; workspace clippy
with and without `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
--no-deps`; the spec's `cargo test` command for `lane`, `math`, `builtins`, `dsp-reference`;
`test-debug-a` and `test-debug-b` (qualification.yml's commands) and `conformance_fixtures
--check`; release `tail_contract` with the ignored gate 2 and release `filter_liveness`;
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`,
`check-builtins-policy.sh`, `check-dsp-research.sh`; `graph_fixture --check`,
`check-graph-determinism.sh` (100/100); `check-builtins-fixtures.sh` with the release audit;
`audit capi`, `check-capi-abi.sh` and its self-test; `check-cross-targets.sh`; the worklet chain
(`build-web-audioworklet.sh --named-twin`, `strip-wasm-names.py` self-test and check,
`check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts` (after the memory fix above),
`check-scalar-oracle-absent.py` wasm and native, `test-web-audioworklet.sh`); `run-wasm-gates.sh
--without-v8-spill --without-native`. The debug suites and the policy scripts ran on the tree
before the memory restructuring (a change inside `LiveBound` only); doc, clippy, fmt, release
`tail_contract` and the browser gate were rerun after it.

**Open items.** (1) The cost above, for #1457. (2) Further tightening is possible but small: a
partition four times finer moved the prototype's `R` by 121 frames; a frequency-aware LPF before
`N` (not taken) would mostly shrink the `sup Psi` flush charges, which only affect `P*`'s
pre-`T_decay` part. (3) #1329's `omega_state` finding above.
