# State the delay's bounded tail and exact-rest bound

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan with a delay reports a finite tail at its rate: the feedback loop loses at least 0.45 dB per
pass, so its echoes fall below -144 dB in a bounded time. The delay also states when its ring and
damping state reach exact rest. Today it declares `Infinite`.

## Context

- **The effect.** Fixed two-second integer-time dual-mono and ping-pong delay
  (`crates/delay/src/lib.rs:1-35`). Parameters (`:185-235`): time 1 to 2,000 ms, feedback -0.95 to
  0.95, damping 0 to 0.995, mix 0 to 1, cross feedback 0 to 1. Latency 0, `tail: Infinite`
  (`:257-264`); `prepared.metadata.tail` is asserted `Infinite` in a unit test (`:1798`).
- **Numerics.** `lane::flush` on the two recursive words per lane, the damping state and the ring
  write, once per sample (crate doc `:19-21`). Damping is a TPT one-pole whose control keeps its
  48 kHz cutoff at every rate, `y = (1 - c) x + c y` (`damping_coefficient`, `:355-370`). Ramps are
  64 samples (`RAMP_SAMPLES`, `:64`); a time change crossfades between taps (`:434-440`, `:499`).
- The home for the values is `tail_and_rest` (#1377); gain and decay come from #1379.

## Decisions frozen for this slice

- **D1. The argument is ℓ∞, not spectral.** Every loop element is non-expansive in the sample
  maximum: the damping one-pole (a convex combination), the cross-feedback matrix
  `[[1-c, c], [c, 1-c]]` (rows sum to 1, entries non-negative), the tap crossfade (a convex
  combination of two taps), and feedback `|f| <= 0.95`. So for any parameter history, including
  ramps and crossfades in flight, ring values satisfy `|w| <= P / (1 - 0.95) = 20 P`, and from `N`
  on every value written is at most `0.95` times a value written at least `d_min = 1 ms` earlier
  and at most `D_max = 2 s` earlier. The derivation
  (`docs/derivations/1374-delay-tail-and-rest.md`) turns this into
  `T = D_max_samples * ceil(ln(eps / (20 * G_wet)) / ln(0.95))` plus one ramp and one crossfade, with
  `f32` rounding absorbed as in #1329 D3 (each element's rounding is relative, so the per-pass factor
  becomes `0.95 * (1 + c * 2^-24)` with `c` the element count). Expected size: about 73M samples at
  96 kHz.
- **D2. Rest.** From `20 P` down to `FLUSH_EPS` at the same per-pass factor, for #1329 D2's two
  peaks, plus the damping state's own decay (`one_pole_rest_samples`, #1375) and its stall check
  (#1375 D3). The damping recurrence must be checked for a fixed point above the flush at
  `c = 0.995` at every rate; a stall is reported to Sol as a class-B defect and no bound is stated.
- **D3. Gain and decay** (#1379): `PeakGain` from mix and the loop's `1 / (1 - 0.95)`; `TailDecay`
  = `ceil(D_max_samples * ln(10) / -ln(0.95_eff))` samples per 20 dB.

## Deliverables

1. `tail_and_rest` for the delay; the derivation note; `dsp-research/delay.md` "latency and tail".
2. Tests (gates 1-3); the `:1798` assertion changes to the computed value.

## Authorized paths

- `crates/delay/src/lib.rs` (descriptor, `tail_and_rest`, the `:1798` test only),
  `crates/delay/tests/tail_contract.rs` (new)
- `docs/derivations/1374-delay-tail-and-rest.md` (new), `dsp-research/delay.md`, this spec

## Non-goals

- Changing the loop, the damping design or any rendered bit. Stream A owns payload code here.

## Hazards

- The tail is minutes long. Hosts cap tails at `u64::MAX` (`crates/host-core/src/prepare.rs:1559`);
  `graph_fixture` caps at 10,000,000 and a fixture graph with a delay would hit `graph.tail.limit`.

## Objective gates

1. **Recompute**: the formula re-derived in the test from the descriptor's domain bounds and the rate
   equals `tail_and_rest` exactly (integer arithmetic, no digest).
2. **Soundness, real kernel** (release, 48 kHz): time 2,000 ms, feedback 0.95, damping 0, cross
   feedback 0 and 1, mix 1; a 1-sample impulse of `P = 1`; render to `T`, checking `|y| < eps` over
   the last 2 s before `T` (the slowest echo) and recording the measured -144 dB crossing in the
   evidence. A second run with feedback ramping 0.5 → 0.95 and a time crossfade in flight at `N`.
3. **Rest, real kernel** (release, 48 kHz, time 1 ms so the run is short): ring, feedback words and
   damping state all `+0.0` by the rest bound's internal function evaluated at `D = 48` samples
   (the function `tail_and_rest` evaluates at `D_max`), with `P = 10^(24/20)` and feedback 0.95;
   and still non-zero at 80 % of it, so the bound is not vacuous.
4. Commands: the `test-debug-b` command; `cargo test --locked --release -p delay --test tail_contract`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail computed from the minimum delay time, or without the loop's `20 P` state bound,
  disagrees with the re-derivation.
- Gate 2: a bound that ignores the cross-feedback path or a crossfade in flight is violated by the
  kernel.
- Gate 3: a damping state that stalls above the flush, or a ring write that skips the flush, never
  rests and is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  one-pole helper
