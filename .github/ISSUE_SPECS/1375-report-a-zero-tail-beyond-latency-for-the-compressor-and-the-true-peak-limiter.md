# Report a zero tail beyond latency for the compressor and the true-peak limiter

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan whose only recursive effects are compressors and true-peak limiters reports a finite tail:
both effects multiply a (delayed) input by a gain, so their output is exactly zero once the input
has been zero for their latency. Both also state when their detector and gain state reach exact
rest. Today both declare `Infinite`, which makes every plan that holds one report `Infinite`.

## Context

- **Compressor.** Feed-forward peak compressor; zero latency (`crates/compressor/src/lib.rs:291-296`,
  `tail: Infinite`). Its frozen identities include `mix == 0` and `mix == 1` (`:25-30` of the crate
  doc); the gain path is `effect_runtime::envelope::rms_follow` and `lane::flush` on `g` (crate doc
  table, `:11-22`). Output is a gain-and-mix of the current input sample (`gain_mix_block` form).
- **Limiter.** `y[n] = x[n - T] * g[n]`, `T = N + 6`, `N = Fs / 100`
  (`crates/true-peak-limiter/src/lib.rs:11-24`), declared latency `lookahead_maximum + 6`
  (`:235-243`, `tail: Infinite`). The gain recurrence is
  `d[n] = max(1 - s[n], fma(c, (1 - s[n]) - d[n-1], d[n-1]))` (`:21`): the `e + c * (u - e)` form.
- **One-pole followers.** `ar_one_pole_step` is `e' = flush(c * e + k * u)`
  (`crates/effect-runtime/src/envelope.rs:130-161`); its doc records that the `e + k * (u - e)` form
  stalls in `f32` when `|u - e| < ulp(e) / (2k)`. With `u = 0` a stall above `FLUSH_EPS` would be a
  state that never rests.
- The home for the values is `tail_and_rest` (#1377); the contract and `RestSamples` are #1329's;
  gain and decay are #1379's.
- Round-1 evidence (decision 15): "the compressor and limiter only multiply a delayed input, the
  argument the transient shaper used for `Finite(0)`".

## Decisions frozen for this slice

- **D1. Tail.** Both report `TailSamples::Finite(0)` and `TailDecay(0)` at every rate: output is
  exactly zero (either sign) from `N + latency` on. The implementer confirms
  by reading each kernel that no output term other than `gain * delayed input` exists (dry and wet
  of the mix both read the same delayed input; a sidechain only feeds the detector). If any other
  term exists, the slice stops and reports it.
- **D2. Rest.** `RestBound::Bounded` from each recursive state word's decay with zero input
  (detector envelopes, gain smoother `d`, lookahead ring, sliding-minimum window): ring and window
  words are zero after their length; each one-pole word rests after
  `ceil(ln(E_max / FLUSH_EPS) / -ln(c_eff))` samples, where `E_max` is the word's largest reachable
  value for the two peak cases of #1329 D2 and `c_eff` its slowest coefficient over the parameter
  domain at that rate, inflated by `f32` rounding. A shared helper
  `effect_runtime::tail::one_pole_rest_samples(c_max, e_max) -> u64` holds the formula for #1376.
- **D3. Stall check.** For every one-pole word, prove (in the doc comment of the helper's caller) that
  no `f32` fixed point exists above `FLUSH_EPS` with zero input at the slowest coefficient. For the
  two-product form the condition is `c < 1 - 2^-24`; for the `e + c (u - e)` form it is
  `c > 2^-24`, scale-free. If a word fails, stop and report it to Sol as a class-B defect, as in
  *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328); do not state a bound.
- **D4. Gain.** `PeakGain` is the compressor's maximum makeup plus `mix` (it never raises above
  makeup), and the limiter's `0 dB` (`g <= 1`).

## Deliverables

1. `tail_and_rest` bodies for both effects; `effect_runtime::tail` (new module) with the helper.
2. Tests (gates 1-3). Docs: `dsp-research/dynamics.md` and `dsp-research/true-peak.md` (tail and rest lines).

## Authorized paths

- `crates/compressor/src/lib.rs`, `crates/true-peak-limiter/src/lib.rs` (descriptor and
  `tail_and_rest` only), `crates/effect-runtime/src/tail.rs` (new), `crates/effect-runtime/src/lib.rs`
  (module line)
- `crates/compressor/tests/tail_contract.rs`, `crates/true-peak-limiter/tests/tail_contract.rs` (new)
- `dsp-research/dynamics.md`, `dsp-research/true-peak.md`, this spec

## Non-goals

- Other effects (#1372-#1374, #1376). Any rendered-bit change.

## Objective gates

1. **Zero tail, real kernel**: for each effect, at every rate, parameters at the domain extremes
   (fastest/slowest attack and release, maximum makeup, `mix` 0, 0.5, 1, ratio maximum, limiter
   lookahead 0 and maximum), feed 4,096 samples of full-scale noise then zeros: every output sample
   from `latency` after the last non-zero input is exactly zero.
2. **Rest, real kernel**: the same runs, at `P = 10^(24/20)` and `P = 1e29`, reach all-zero state
   words (through the state payload snapshot) by `latency + R`, `R` the stated bound.
3. **Recompute**: the helper's value for each word equals a brute-force `f32` iteration of that word's
   recurrence from `E_max` with zero input, counted to `+0.0` (allowed slack: the helper is at most
   1 % above, never below).
4. Commands: the `test-debug-b` command from `.github/workflows/qualification.yml`; the conformance
   fixtures check; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a mix or makeup path that leaks a delayed or filtered term past latency is red; no test
  checks output beyond latency after silence.
- Gate 2: a state word that stalls above the flush (D3) or a bound computed from the wrong
  coefficient is red.
- Gate 3: a helper formula off by the rounding inflation under-reports and is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)
