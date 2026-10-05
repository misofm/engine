# State the parametric EQ's bounded tail and exact-rest bound

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan with a parametric EQ, as an insert or a console slot, reports a finite tail at its rate. The
EQ also states when its cascade reaches exact rest. Today it declares `Infinite`, so every console
with an EQ slot reports `Infinite`.

## Context

- **The cascade.** Six TPT SVF sections per channel: HPF, four bands, LPF (`EQ_SECTION_COUNT = 6`,
  `crates/parametric-eq/src/lib.rs:77-84`). Band domain: frequency 10 Hz to 20 kHz, gain ±24 dB,
  Q 0.1 to 18, shelf slope 0.1 to 1 (`MINIMA`/`MAXIMA`, `:471-478`); kinds bell, low/high shelf,
  low/high pass, notch (`EqBandKind`, `:275-288`). Design in `f64`, one rounding to `f32`:
  `design_svf_words_f64` and `design_svf` (`:747`, `:853`). Unlike the builtin input section, `k`
  varies with Q, gain and slope.
- **Live edits.** Block-rate parameters ramp the words over `RAMP_SAMPLES = 64` (`:117`).
  *Make a parametric EQ band's enabled and kind live* (#1337) adds live kind and enable.
- **Declared tail.** `Infinite` at every rate (`:648-653`), moved into `tail_and_rest` by
  *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377).
- **Evidence (decision 15, round 2 B4(a)).** With the joint flush of #1328, the EQ's slowest exact
  rest is about 89 s at every rate for a +24 dBFS peak (bell 10 Hz, Q 18, +24 dB). Its -144 dB tail
  has not been computed. Every SVF step matrix is non-expansive in the 2-norm (norm exactly 1), so
  bounding the state under arbitrary earlier edits needs a strict-decay argument.
- *State a bounded tail and an exact-rest bound for every node* (#1329) builds the certified
  machinery for one fixed-`k` section family; this slice extends it to varying `k` and six sections.

## Decisions frozen for this slice

- **D1. Same contract and method as #1329 D1-D3**: certified `T` (exact tail sum below `eps / 2`,
  `f32` deviation below the other half), certified `R` for the two peak cases, `TailDecay` and
  `PeakGain` (#1379), all from the designer, per rate, over the whole parameter domain, on the
  control thread inside `tail_and_rest`.
- **D2. Varying `k`.** The shared-eigenvector argument of #1329 D5 does not apply. The derivation
  (`docs/derivations/1372-parametric-eq-tail-and-rest.md`) must give a common quadratic Lyapunov
  function (a matrix `Q` with `A^T Q A <= q^2 Q`, `q < 1`) over every designed step and every
  64-sample ramp step in the domain at each rate, or another certified strict-decay bound, and
  identify the domain extreme production evaluates. A ramp may run between *any* two designed
  points (an edit jumps anywhere in the domain), not only neighbours. The derivation covers this
  by convexity: the SVF step matrix is affine in the stored words (`d1 = -c1*ic1 + a2*v3`,
  `d2 = a3*v3 + a2*ic1`), so `||Q^(1/2) A(w) Q^(-1/2)||_2 <= q` is a convex constraint in the word
  vector `w`; it holds on every interpolated ramp word once it holds on every designed word, and
  the `f32` rounding of an interpolated word is absorbed by the D1 rate inflation. The note proves
  the affine form and states a Lipschitz constant of the designer in each parameter, so a check on
  a finite grid plus that margin covers the continuous domain. If no such bound exists with a
  finite value, the slice stops and reports to Sol; there is no fallback value.
- **D3. Cascade.** Six sections compose by the union bound of #1329 D3, generalized to `k` terms
  (`a_1 + ... + a_k >= H` implies some `a_i >= H / k`).
- **D3a. Rest** is #1329 D2's definition: output `±0.0` and the twelve SVF integrator words per
  channel equal to a freshly reset EQ's (all `+0.0`) under `f32` `==`, with #1328's joint flush.
- **D4. Expected magnitude.** `R` near 89 s (about 8.5M samples at 96 kHz) and `T` shorter. Hosts cap
  tails at `u64::MAX` (`crates/host-core/src/prepare.rs:1559`); `graph_fixture`'s 10,000,000 cap
  holds them.

## Deliverables

1. `tail_and_rest` for the EQ, with its computation in `crates/parametric-eq/src/tail.rs` (new),
   built on #1329's `math::tail` section bounds (`crates/math/src/tail.rs`, extended there for
   varying `k` if needed).
2. The derivation note; `dsp-research/filters.md` "Latency and tail".
3. Tests (gates 1-3).

## Authorized paths

- `crates/parametric-eq/src/tail.rs` (new), `crates/parametric-eq/src/lib.rs` (descriptor,
  `tail_and_rest` and module line only), `crates/parametric-eq/tests/tail_contract.rs` (new)
- `crates/math/src/tail.rs` (varying-`k` extension only)
- `docs/derivations/1372-parametric-eq-tail-and-rest.md` (new), `dsp-research/filters.md`, this spec
- Tests that assert the EQ's `Infinite` tail, each rewritten to the computed value with the reason:
  `crates/parametric-eq/tests/bank.rs:203` (`program_key.tail`) and
  `crates/host-core/tests/live_lanes.rs:202-211` (the EQ fixture's lanes-free tail). The
  `graph-compiler` fixtures at `crates/graph-compiler/src/lib.rs:12321` and `:12735` are the
  limiter's and the multiband compressor's, not the EQ's; #1375 and #1373 own them.
- Canonical plan fixtures whose graph holds an EQ, if any moves: `fixtures/graph/v1/` and the
  digests that pin them in `crates/graph-compiler/tests/`, re-pinned one at a time

## Non-goals

- Any kernel or designer change. Stream A owns payload code in the same crate.

## Objective gates

1. **Recompute**: for each design, a brute force computes the `f64` cascade impulse response `h`
   (designed `f32` words, `f64` arithmetic) to 12,000,000 samples and `S_b(j) = sum_{j <= m < 1.2e7} |h[m]|`;
   `T_b(e)` is the smallest `j` with `S_b(j) < e`. Points: the D2 extreme and 24 single-band points
   (each kind at 10 Hz and 20 kHz, Q 0.1 and 18, gain +24 dB, slope 0.1). Soundness at every point:
   `T_b(eps / 2) <= T` (`T` is one value over the domain, #1377 D5). Tightness at the extreme only:
   `T <= 2 * T_b(eps / 2)`. The factor 2 is the slack a domain-wide `q` and the condition number of
   `Q` may cost against the extreme's own pole; a certified `T` above it is a finding for Sol with
   the measured ratio, not a gate change. Release at every rate; debug at 48 kHz only.
2. **Domain scan** (closed form, no rendering), checking the derivation's claim
   `||Q^(1/2) A(w) Q^(-1/2)||_2 <= q - m` (`m` the stated Lipschitz margin): (a) on a grid of kind ×
   frequency (100 log-spaced) × Q (20 log-spaced) × gain (13) × slope (5); (b) on 100,000 seeded
   uniform random domain points; (c) on the interpolated words at steps 1, 32 and 63 of a 64-sample
   ramp between 100,000 seeded random pairs of domain points of any kinds, far apart included.
   (c) is the regression check of D2's convexity argument, not its proof.
3. **Soundness, real kernel** (release): the extreme with a worst-sign input and a live target 32
   samples before `N`: `|y| < P * eps` from `N + T`; from `N + R` on, D3a's rest (output `±0.0`,
   integrator words equal to a freshly reset EQ's under `==`), for `P = 10^(24/20)` and `1e29`.
4. Commands: the `test-debug-b` command; `cargo test --locked --release -p parametric-eq --features parametric-eq/test-support --test tail_contract`;
   the `test-debug-a` command; `bash scripts/check-graph-determinism.sh`;
   `bash scripts/check-dsp-research.sh`; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail computed from the wrong section or ignoring the cascade disagrees with brute force.
- Gate 2: a Lyapunov constant fitted on grid points only, which an off-grid design or a ramp
  between distant points violates, is red.
- Gate 3: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel.

## Dependencies

- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328)
- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)
