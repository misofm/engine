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
  identify the domain extreme production evaluates. If none exists with a finite bound, the slice
  stops and reports to Sol; there is no fallback value.
- **D3. Cascade.** Six sections compose by the union bound of #1329 D3, generalized to `k` terms
  (`a_1 + ... + a_k >= H` implies some `a_i >= H / k`).
- **D4. Expected magnitude.** `R` near 89 s (about 8.5M samples at 96 kHz) and `T` shorter. Hosts cap
  tails at `u64::MAX` (`crates/host-core/src/prepare.rs:1559`); `graph_fixture`'s 10,000,000 cap
  holds them.

## Deliverables

1. `tail_and_rest` for the EQ, with its computation in `crates/parametric-eq/src/tail.rs` (new),
   built on #1329's `lane::tail` section bounds (extended there for varying `k` if needed).
2. The derivation note; `dsp-research/filters.md` "Latency and tail".
3. Tests (gates 1-3).

## Authorized paths

- `crates/parametric-eq/src/tail.rs` (new), `crates/parametric-eq/src/lib.rs` (descriptor,
  `tail_and_rest` and module line only), `crates/parametric-eq/tests/tail_contract.rs` (new)
- `crates/lane/src/tail.rs` (varying-`k` extension only)
- `docs/derivations/1372-parametric-eq-tail-and-rest.md` (new), `dsp-research/filters.md`, this spec
- Tests and fixtures that assert the EQ's `Infinite` tail
  (`crates/host-core/tests/live_lanes.rs:202-211`, `crates/graph-compiler/src/lib.rs` test fixtures
  at `:12321`, `:12735`), each re-pinned with the reason

## Non-goals

- Any kernel or designer change. Stream A owns payload code in the same crate.

## Objective gates

1. **Recompute**: a brute-force `f64` cascade impulse response (to 12,000,000 samples, suffix sums)
   for the D2 extreme and 24 single-band points (each kind at 10 Hz and 20 kHz, Q 0.1 and 18,
   gain +24 dB, slope 0.1) gives `T_b <= T`; at the extreme `T <= T_b + T / 100`. Release at every
   rate; debug at 48 kHz only.
2. **Domain scan** (closed form, no rendering): over a grid of kind × frequency (100 log-spaced) ×
   Q (20 log-spaced) × gain (13) × slope (5), and every 64-sample ramp between neighbouring grid
   points, no step exceeds the D2 constants (`q`, gains) production uses.
3. **Soundness, real kernel** (release): the extreme with a worst-sign input and a live target 32
   samples before `N`: `|y| < P * eps` from `N + T`, all state words `+0.0` from `N + R`, for
   `P = 10^(24/20)` and `1e29`.
4. Commands: the `test-debug-b` command; `cargo test --locked --release -p parametric-eq --features parametric-eq/test-support --test tail_contract`;
   the `test-debug-a` command; `bash scripts/check-graph-determinism.sh`;
   `bash scripts/check-dsp-research.sh`; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail computed from the wrong section or ignoring the cascade disagrees with brute force.
- Gate 2: a Lyapunov constant fitted on designed points only, which a ramp step violates, is red.
- Gate 3: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel.

## Dependencies

- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328)
- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)
