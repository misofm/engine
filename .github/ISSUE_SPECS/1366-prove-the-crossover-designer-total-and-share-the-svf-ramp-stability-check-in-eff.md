# Prove the crossover designer total and share the SVF ramp stability check in effect-runtime

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

No audible change. This is the precursor of *Make the multiband compressor's crossover live*
(#1338). After it, the multiband's crossover designer has an infallible form that render may call,
proven to pass its own check for every legal crossover at every launch rate, and the SVF stability
predicate the EQ uses for ramps lives once, in `effect-runtime`, where the multiband can use it too.
No rendered bit and no EQ refusal moves.

## Context

- **The designer.** `design_lr4` (`crates/multiband-compressor/src/lib.rs:519-561`) designs
  `(c1, a2, a3)` in `f64` from `g = tan(pi fc / fs)`, `t = g (g + k)`, `c1 = t / (1 + t)`,
  `a2 = g (1 - c1)`, `a3 = g a2`, rounds once to `f32`, then checks the rounding back (half-power
  point within 0.005 dB, doc `:515-518`). It returns `None` on a zero rate, an invalid crossover
  (`parameter_value_valid(&SPECS[0], ..)`, `:520`) or a failed check. It runs only at prepare and
  restore today, so a `None` is a preparation error. Render cannot accept a fallible call: an
  acked crossover edit must never be dropped (#1338 D3).
- **The stability predicate.** The EQ's `word_spectral_norm` (`crates/parametric-eq/src/lib.rs:796-807`)
  is the spectral norm of the zero-input transition matrix `M = [[1-2c1, -2a2], [2a2, 1-2a3]]` the
  SVF iterates. It is convex in the words, so checking both ends of a linear ramp checks every point
  (doc `:790-795`). Its two bounds: `NORM_TOLERANCE` (`:810`) for a rounded design, and
  `RAMP_PATH_NORM_TOLERANCE` (`:812-835`, with its derivation) for a restored in-flight walk. The
  derivation uses only `c1, a2, a3 < 1`, which the crossover words also satisfy.
- `word_spectral_norm` is public and used by `crates/parametric-eq/tests/analytic.rs:390`, `:426`,
  `:527`, `:534`. `effect-runtime` is `no_std` (`crates/effect-runtime/src/lib.rs:43`) and depends
  on `math` (`crates/effect-runtime/Cargo.toml:14`), which provides `math::tan` and `math::sqrt`
  (`crates/math/src/lib.rs:125`, `:235`).

## Decisions frozen for this slice

- **D1. Split the designer.** `design_lr4_words(sample_rate: u32, crossover_hz: f32) -> [f32; 3]`
  is the computation alone: no domain test, no check-back, no `Option`. `design_lr4` keeps its
  signature and behaviour: it tests the domain, calls `design_lr4_words`, then runs the existing
  check-back. Every current caller keeps calling `design_lr4`.
- **D2. Shared module.** A new `effect_runtime::svf` module holds
  `transition_norm(c1: f32, a2: f32, a3: f32) -> f64` (the body of `word_spectral_norm`, operation
  for operation, with `math::sqrt`), `NORM_TOLERANCE` and `RAMP_PATH_NORM_TOLERANCE`, both public,
  with the derivation comment moved verbatim. `f64` square root is correctly rounded in both
  `math::sqrt` and `f64::sqrt`, so the result is bit-identical.
- **D3. The EQ delegates.** `word_spectral_norm(words)` stays public with the same signature and
  becomes one call to `transition_norm(words.c1, words.a2, words.a3)`. The EQ's two constants are
  deleted and its two uses (`:899`, `:2742`) and the doc link (`:816`) name the shared ones.
- **D4. Totality is a test, not a branch.** Gate 1 proves `design_lr4` returns `Some` for every
  `f32` in `[80, 8000]` at 44.1, 48, 88.2 and 96 kHz, and that every designed triple has
  `transition_norm <= NORM_TOLERANCE`. That is what licenses #1338 to call `design_lr4_words` on
  render with no failure branch.

## Effect evidence (AGENTS.md list)

- Equations and coefficients: unchanged (above). Numerical limits: D4's sweep is the evidence.
- Latency, tail, smoothing, NaN and denormal behaviour: unchanged.
- Citations: Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4 (the TPT SVF);
  Wishnick, "Time-Varying Filters for Musical Applications", DAFx-14 (a contractive transition
  matrix bounds the state under arbitrary coefficient variation); Laroche, "On the Stability of
  Time-Varying Recursive Filters", JAES 55(6), 2007.
- Listening evidence: not required; nothing renders differently.

## Deliverables

1. D1 in `crates/multiband-compressor/src/lib.rs`.
2. D2 in `crates/effect-runtime/src/svf.rs` and its `pub mod` line.
3. D3 in `crates/parametric-eq/src/lib.rs`.
4. `crates/multiband-compressor/tests/designer_total.rs` (gate 1).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (D1 only), `crates/multiband-compressor/tests/designer_total.rs` (new)
- `crates/effect-runtime/src/lib.rs` (the module line), `crates/effect-runtime/src/svf.rs` (new)
- `crates/parametric-eq/src/lib.rs` (`word_spectral_norm`, the two constants and their uses only;
  #1337 edits other parts of this file, so rebase on whichever lands first)

## Non-goals

- Making the crossover live, its descriptor, word ramps or payload (#1338).
- Any change to the EQ's designs, refusals or bits.

## Objective gates

1. **Designer total over the domain.** `#[ignore]`, run in release: for every `f32` in
   `[80.0, 8000.0]` (by bit pattern, both ends included) at 44 100, 48 000, 88 200 and 96 000 Hz,
   `design_lr4` returns `Some(w)`, `w == design_lr4_words(..)` bit for bit, and
   `transition_norm(w) <= NORM_TOLERANCE`. A per-PR test runs the same three checks on every 1024th
   `f32` of the domain plus both ends. The PR records the largest norm excess found.
2. **No bit moves.** The multiband and EQ suites pass unchanged, `corpus_digests.in` and the EQ's
   render contract are untouched.
3. Commands:
   - `cargo test --locked --all-targets -p effect-runtime -p multiband-compressor -p parametric-eq --features math/lane,parametric-eq/test-support,lane/test-support`
   - `cargo test --locked --release -p multiband-compressor --test designer_total -- --ignored`
   - `bash scripts/check-parametric-eq-render-contract.sh`
   - `bash scripts/check-effect-runtime-policy.sh`
   - `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a crossover whose rounded words fail the check-back, or a design outside the contractive
  set, turns it red. No existing test sweeps the domain, and #1338 would otherwise apply unchecked
  words on render.
- No test is superseded: `analytic.rs`'s norm tests keep calling `word_spectral_norm`, which now
  delegates, so they also guard D3.

## Dependencies

- none
