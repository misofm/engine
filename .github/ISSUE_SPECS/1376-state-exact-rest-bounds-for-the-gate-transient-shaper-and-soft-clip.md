# State exact-rest bounds for the gate, transient shaper and soft clip

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The three effects that already declare a finite tail get it checked against the tail contract, and
each states when its whole state is exactly zero after silence. Silence skipping can then use all
three.

## Context

- **Gate/expander.** Hysteretic peak gate; latency 0, `Finite(0)`
  (`crates/gate-expander/src/lib.rs:237-244`). Output is a gain times the current input; the
  detector and gain state are one-pole recurrences with the `1e-20` flush
  (`crates/dsp-reference/src/gate_expander.rs:161`), plus integer hold counters.
- **Transient shaper.** Two switched attack/release one-pole followers (fast 0.5 ms / 20 ms, slow
  10 ms / 100 ms, `crates/transient-shaper/src/lib.rs:1-12`) through `ar_one_pole_step`
  (`crates/effect-runtime/src/envelope.rs:130-161`); latency 0, `Finite(0)` (`:147-152`).
- **Soft clip.** 2x oversampling, 63-tap Blackman half-band for interpolation and decimation, cubic
  `c(u) = u - u^3/3` clamped to `±2/3`, dry path delayed 31 samples, latency 31, tail 29
  (`crates/soft-clip/src/lib.rs:1-7`, `:174-179`). `c(0) = 0`, and `flush` guards the two history
  inputs.
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375) adds
  `effect_runtime::tail::one_pole_rest_samples` and the stall check this slice reuses.

## Decisions frozen for this slice

- **D1. Tails.** Gate and transient shaper: `Finite(0)`, `TailDecay(0)`. Soft clip: the tail is
  the number of samples after `N + latency` until the decimator's history holds only zeros, from
  the half-band length and the polyphase layout. Exact zero, so `TailDecay(0)`. The test recomputes
  it from the filter length and the latency; if the recomputation is not 29, the declared value
  changes and the commit says why.
- **D2. Rest.** Gate and transient shaper: the maximum over their one-pole words of
  `one_pole_rest_samples` at the slowest release in the domain, and the hold counter's maximum, for
  #1329 D2's two peak cases. Soft clip: its histories are FIR delay lines and its ramps hold no
  input-driven state, so `R` is the number of samples after `N` until every history word is zero,
  from the history lengths; it is the same for both peak cases.
- **D3. Stall check** as in #1375 D3 for every one-pole word. A stall is reported to Sol as a class-B
  defect; no bound is stated for that effect.
- **D4. Gain.** Gate: `0 dB`. Transient shaper: its maximum attack/sustain boost. Soft clip: the
  output parameter's maximum plus the half-band's ℓ1 gain, mixed with the dry path's.

## Deliverables

1. `tail_and_rest` bodies for the three effects.
2. Tests (gates 1-3). Docs: `dsp-research/dynamics.md` and `dsp-research/nonlinear-antialiasing.md`
   (tail and rest lines).

## Authorized paths

- `crates/gate-expander/src/lib.rs`, `crates/transient-shaper/src/lib.rs`, `crates/soft-clip/src/lib.rs`
  (descriptor and `tail_and_rest` only)
- `crates/gate-expander/tests/tail_contract.rs`, `crates/transient-shaper/tests/tail_contract.rs`,
  `crates/soft-clip/tests/tail_contract.rs` (new)
- `dsp-research/dynamics.md`, `dsp-research/nonlinear-antialiasing.md`, this spec

## Non-goals

- Changing any kernel, coefficient or declared latency.

## Objective gates

1. **Tail, real kernel**: for each effect at every rate, parameters at their domain extremes, 4,096
   samples of full-scale noise then zeros: output exactly zero from `latency + tail` after the last
   non-zero input, and (soft clip) non-zero at `latency + tail - 1` for some input, so 29 is the
   smallest.
2. **Rest, real kernel**: the same runs at `P = 10^(24/20)` and `P = 1e29` reach all-zero state
   words (state payload snapshot) by `latency + R`.
3. **Recompute**: soft clip's tail from the filter length and latency in the test; gate and shaper
   rest from a brute-force `f32` iteration of each word (helper at most 1 % above, never below).
4. Commands: the `test-debug-b` command from `.github/workflows/qualification.yml`; the conformance
   fixtures check; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a soft-clip declared tail shorter than the decimator's flush, or a gate/shaper output term
  that survives silence, is red; nothing renders past these tails today.
- Gate 2: a stalled one-pole word or a bound from the wrong release coefficient is red.
- Gate 3: a stated value that no longer follows its designer (for example a half-band length change)
  is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  helper
