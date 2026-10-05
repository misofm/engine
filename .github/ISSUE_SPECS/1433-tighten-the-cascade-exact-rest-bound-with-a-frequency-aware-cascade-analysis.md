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
  be proven before the HPF's.
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
  the margin `q_ramp + μ_s < 1` by at least `3.69e-5`, as #1329's Amendment 2 records; the bound is
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
   Assert `R_meas <= peak_plus_24_dbfs <= 1.10 · R_meas` (the stated margin: 10 %, which the
   estimate `1.02M-1.07M` plus `2 · N_SILENCE` meets against `983,374`). Record `R_meas` and the
   certified value per rate. A certified value above the margin is a finding for Sol (D4), not a
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
  tighten (the universal ball left in place) misses the 10 % margin. No test compares #1329's
  certified rest with the measured one from above.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329)
