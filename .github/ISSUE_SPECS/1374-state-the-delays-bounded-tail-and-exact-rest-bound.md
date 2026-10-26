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
  ramps and crossfades in flight, ring values and the damping state satisfy
  `|w| <= P / (1 - 0.95) = 20 P`.
- **D1a. The damping state is loop memory.** The damping one-pole sits in the loop (tap, damping,
  feedback, matrix, ring write: the frozen order at `crates/delay/src/lib.rs:1010-1017`), and its
  state `s` (`DelayLane::damping_state`, `:430-431`) carries contributions from taps older than
  `D_max`. So "every value written is at most `0.95` times a value written within the last
  `D_max`" is false, and the bound uses the potential
  `V(t) = max(|s_L(t)|, |s_R(t)|, max |ring| over the words a tap can still read at t)`. With zero
  input, `V` never increases (every element is a convex combination or a contraction), and every
  word written after `t` is at most `0.95 * V(t)`. After `D_max` samples every readable word was
  written after `t`, and the damping state has moved toward those taps by its own factor:
  `|s(t + D_max + K)| <= (1 - 2 g_min)^K * V(t) + 0.95 * V(t)`, `g_min` the smallest non-zero TPT
  coefficient over the domain at that rate (the `c = 0.995`, 38.3 Hz design; `c = 0` is the
  stateless identity path). Hence `V(t + L) <= q * V(t)` with period `L = D_max + K_d` and
  `q = 0.95 + (1 - 2 g_min)^(K_d)` (written words are at most `0.95` times that). The derivation
  (`docs/derivations/1374-delay-tail-and-rest.md`) chooses `K_d` to minimise the tail, absorbs
  `f32` rounding as in #1329 D3 (each element's rounding is relative, so each factor is inflated by
  `(1 + c * 2^-24)` with `c` the element count), and states
  `T = L * ceil(ln(eps / (20 * G_wet)) / ln(q_f))` plus one ramp and one crossfade. Expected size:
  about 73M samples at 96 kHz.
- **D2. Rest.** #1329 D2's definition: output `±0.0`, and the ring words and damping states equal
  under `f32` `==` to a freshly reset delay's (all `+0.0`; cursors, `valid_history` and the
  crossfade counters are not signal state). The bound takes `V` from `20 P` down to `FLUSH_EPS` at
  D1a's per-period factor, for #1329 D2's two peaks, then adds the damping state's own decay from
  `FLUSH_EPS`-sized taps (`one_pole_rest_samples`, #1375) and one `D_max` so the last flushed word
  leaves the readable ring. The damping recurrence in the kernel's operation order (`v = g * (x - s)`, `y = s + v` by `fma`,
  `s' = y + v`, flushed; `crates/delay/src/lib.rs:1012-1013`) must be
  checked for an `f32` fixed point above the flush with `x = 0` at `g_min` at every rate; a stall
  is reported to Sol as a class-B defect and no bound is stated.
- **D3. Gain and decay** (#1379): `PeakGain` from mix and the loop's `1 / (1 - 0.95)` (the loop is
  linear, so this is also its incremental gain); `TailDecay` = `L * ceil(ln(10) / -ln(q_f))`
  samples per 20 dB.

## Deliverables

1. `tail_and_rest` for the delay; the derivation note; `dsp-research/delay.md` "latency and tail".
2. Tests (gates 1-3); the `:1798` assertion changes to the computed value.

## Authorized paths

- `crates/delay/src/lib.rs` (descriptor, `tail_and_rest`, the `:1798` test only),
  `crates/delay/tests/tail_contract.rs` (new)
- `crates/graph-compiler/src/lib.rs` (the delay fixture's `Infinite` assertions at `:13803`,
  `:13867` and `:13878`, rewritten to the computed value)
- `docs/derivations/1374-delay-tail-and-rest.md` (new), `dsp-research/delay.md`, this spec

## Non-goals

- Changing the loop, the damping design or any rendered bit. Stream A owns payload code here.

## Hazards

- The tail is minutes long. Hosts cap tails at `u64::MAX` (`crates/host-core/src/prepare.rs:1559`);
  `graph_fixture` caps at 10,000,000 and a fixture graph with a delay would hit `graph.tail.limit`.

## Objective gates

1. **Recompute**: the formula re-derived in the test from the descriptor's domain bounds and the rate
   equals `tail_and_rest` exactly (integer arithmetic, no digest).
2. **Soundness, real kernel** (release, 48 kHz): time 2,000 ms, feedback 0.95, cross feedback 0
   and 1, mix 1, damping 0 and 0.995; input the constant `P = 1` for 60 s (the in-phase worst case
   that drives the ring toward D1's `20 P` bound), then zeros; render to `T`, checking `|y| < eps`
   over the last 2 s before `T` (the slowest echo) and recording the measured -144 dB crossing in
   the evidence. A second run with feedback ramping 0.5 → 0.95 and a time crossfade in flight at
   `N`.
3. **Rest, real kernel** (release, 48 kHz, time 1 ms so the run is short): D2's rest (output
   `±0.0`, ring and damping words equal to a freshly reset delay's under `==`) by the rest bound's
   internal function evaluated at `D = 48` samples (the function `tail_and_rest` evaluates at
   `D_max`), with `P = 10^(24/20)`, feedback 0.95 and damping 0 and 0.995; and not yet at rest at
   80 % of it, so the bound is not vacuous.
4. Commands: the `test-debug-b` command; `cargo test --locked --release -p delay --test tail_contract`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail computed from the minimum delay time, or without the loop's `20 P` state bound,
  disagrees with the re-derivation.
- Gate 2: a bound that ignores the cross-feedback path, a crossfade in flight or the damping
  state's memory beyond `D_max` (damping 0.995) is violated by the kernel.
- Gate 3: a damping state that stalls above the flush, or a ring write that skips the flush, never
  rests and is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Build the launch effect registry once per process and share it* (#1469): the shared registry
  the tail derivations run behind, once per process
- *Define how node tails compose through gain in the graph extent* (#1379)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  one-pole helper

## Decision record (attempt 1, 2026-10-25/26)

- **Owner ruling (2026-10-25), option 1, class B.** D1 was false at 44.1 and 48 kHz: the damping
  cutoff reached 19,845 Hz, so `g` reached 0.863 / 0.782 and the TPT state's gain from the tap
  exceeded 1. The cutoff is now capped at `min(19,845 Hz, fs / 4)` (`damping_cutoff_max_hz`;
  `g <= 1/2` at every launch rate, `g = 1/2` exactly at the cap). Bits move only at 44.1 and
  48 kHz, for every non-zero `f32` damping control `c <= 0.23617806` (44.1 kHz) and
  `c <= 0.20787954` (48 kHz), found by comparing both mappings on every `f32` in `(0, 0.995]`;
  the default 0.25 and every value at 88.2 and 96 kHz are unchanged.
- **A1 anchors (stale in the spec).** `tail_and_rest` was at `crates/delay/src/lib.rs:286`; the
  `Infinite` unit assertion at `:1883`; the E4 clamp test at `:2027-2055`; graph-compiler's
  delay fixture `launch_delay_fixture_closes_scalar_state_tail_pdc_and_transactional_caps`
  (three `Infinite` assertions: metadata tail, output tail, node tail), not `:13803/:13867/:13878`.
- **A2 D1a amended: a per-sample positive majorant replaces the period potential.** The spec's
  `V(t + L) <= q V(t)`, `L = D_max + K_d`, is within about 2 % at `D = 2 fs` but 6 to 8 times loose at
  `D = 48` (each period pays the damping's `K_d ~ 2,000` samples), so gate 3's 80 % check could
  not hold. The note's majorant (`F(r) = A r^-D (beta + alpha b / (r - a)) <= 1`, certified by
  bisection) is the real loop's slowest pole up to rounding: gate 3 rests 0.18 % below its bound.
  `TailDecay` is per sample (`r^D20 < 1/10`), not `L` times a period count.
- **A3 Gains and stalls stated as the maximum of both clauses.** (N1)'s hull gain (20.0) is a
  little below (N2)'s settled-loop gain (20.07-20.13), and (N1)'s stall far below (N2)'s, so
  `G_p = G_t` and `sigma_p = sigma_t` are each the larger value (2,605 / 2,608 mB).
- **A4 Gate 3 non-vacuity is checked at damping 0.995 only**, the design the bound is taken at;
  damping 0 rests at 145,408 samples, 41 % of the bound (the ring wrap of `2 fs + 3` dominates).
- **A5 Gate 2 is release-only** (`ignore` in debug, about 200M frames). `qualification.yml`'s
  release `tail_contract` step needs `-p delay` added; the stream rules forbid this stream to edit
  CI workflows, so root adds it.
- **A6 Re-pin.** G5 `delay/ping_pong_automated` (48 kHz) moved: its 0.15 damping event is inside
  the capped range. Re-pinned from the scalar-oracle digest gate G5 prints
  (`crates/delay/src/corpus.rs`). No other delay fixture moved (record below).
- **A7 Left for root** (outside the authorized paths): `crates/dsp-reference/src/delay.rs` keeps
  the old 19,845 Hz cap (no test drives it below `c = 0.375`); `crates/delay/tests/MUTATIONS.md`
  M4 describes the clamp mutant against the old expectation.

**No-bit-move record (PR evidence, base `b0dc1216f` vs branch, own target dirs, removed after).**
A scratch harness rendered the two G5 corpus cases and 23 damping values (0 to 0.995, both sides
of each boundary) x cross 0 / 0.5 / 1 x feedback 0.95 / -0.6, plus a damping automation run per
value, at all four rates (652 renders). 149 moved: at 44.1 kHz every damping value up to 0.23617
and the automated corpus case; at 48 kHz every value up to 0.2078 and the automated corpus case.
Nothing moved at 88.2 or 96 kHz, at 0.2362 (44.1 kHz) or 0.20788 (48 kHz) and above, or in the
`dual_mono` corpus case. Every moved render uses a damping control inside the capped range.
