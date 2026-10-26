# Issue #1374: the delay's tail and exact-rest bounds

Decision 15 D15-4(b) asks every node to state how long its output stays audible after its input
stops, and when it reaches exact rest. This note derives both, and the four composition values of
#1379, for the launch delay (`crates/delay`), as `delay::tail_bound_at` computes them on the
control thread. The contract and its notation are #1329's
(`docs/derivations/1329-input-section-tail-and-rest.md`, "The contract"): `eps = 10^(-144/20)`,
`N` the first sample of silence, `P` the input peak, `u = 2^-24`, `U = 2^-126` (one operation's
underflow, gradual or flush-to-zero). The argument is in the sample maximum (`l-inf`), not
spectral: a tap crossfade reads one ring word through two taps, so no fixed-parameter spectral
bound covers a control history.

## The owner ruling: the damping cutoff cap

The damping one-pole is the TPT low-pass `v = g (x - s)`, `y = s + v`, `s' = y + v`, with
`g = G / (1 + G)` and `G = tan(pi fc / fs)`. Its output is a convex combination of `s` and `x` for
every `g <= 1`, but its state `s' = (1 - 2g) s + 2g x` is one only for `g <= 1/2`. Before this
issue the cutoff was capped at 19,845 Hz at every rate, so `g` reached 0.863 at 44.1 kHz and
0.782 at 48 kHz: the state's gain from the tap was `G` (6.3 and 3.6) and the loop's sample-maximum
gain `0.95 * 2g` was 1.64 and 1.49, above 1. No bound could be stated there.

On 2026-10-25 the owner approved option 1 (class B): the cutoff is capped at
`min(19,845 Hz, fs / 4)` (`damping_cutoff_max_hz`). At `fs / 4`, `G = tan(pi / 4) = 1` and the
`f32` coefficient is exactly `1/2`. So every coefficient at every launch rate is in `[0, 1/2]`,
including every word of a damping ramp (the ramp stays inside the hull of its endpoints, #1409)
and every restored word (the restore domain is `[0, damping_coefficient_max]`).

**Which bits move.** Only at 44.1 and 48 kHz, and only for damping controls whose 48 kHz cutoff
`-ln(c) 48,000 / (2 pi)` is above `fs / 4`: every non-zero `f32` control `c <= 0.23617806`
(`0x3e71d8a9`) at 44.1 kHz and every non-zero `c <= 0.20787954` (`0x3e54de60`) at 48 kHz, found
by comparing the old and the new mapping on every `f32` in `(0, 0.995]` (1,047,648,425 and
1,045,749,344 values, all of them in those ranges, none outside). Those controls now give
`g = 1/2` (cutoff 11,025 Hz and 12,000 Hz) instead of up to 0.863 and 0.782. At 88.2 and 96 kHz
`fs / 4` is above 19,845 Hz and no value moves; the default `c = 0.25` (10,591 Hz) moves at no
rate. The loss is the top of the damping's brightness range at the two base rates, an octave the
bilinear warp already distorted (#93 F5 gave up rate independence only there).

## Rounding, step by step

Every operation is `f32` with separate roundings (`Lane::fma` never fuses). A relative error is
at most `u` per rounding, an absolute one at most `U` per operation; `lane::flush` only lowers a
magnitude. Per sample and lane, with `M = max(|s|, |x|)`:

* **(F1) Tap.** The active tap is a ring word or the `+0.0` fill. A crossfade blend
  `fl(old + fl(alpha fl(new - old)))`, `alpha` in `[0, 1]`, is at most
  `(1 + 6u) max(|old|, |new|) + 3U`.
* **(F2) Damping state, hull form.** For `g` in `[0, 1/2]`, `s'` lies between `s` and `x`, except
  that it may pass `x` by `7u M + 4U`: `fl(s + h)` is no further from `s + h` than `s` is, `h` has
  the sign of `x - s`, and `s + 2h` exceeds `x` by at most `2.01u |x - s|`. So
  `|s'| <= max(|s|, |x| + 7u M + 4U)`; the identity arm (`g = 0`) gives `|s'| <= |x|`.
* **(F2') Damping state, linear form.** For `g` in `(0, 1/2]`,
  `|s'| <= a |s| + b |x| + 4U` with `a = 1 - 2g + 7u`, `b = 2g + 7u` (`1 - 2g >= 0`).
* **(F3) Damping output.** `|y| <= (1 + 3u) M + 2U`, and `|y| <= alpha |s| + beta |x| + 2U` with
  `alpha = 1 - g + 3u`, `beta = g + 3u`.
* **(F4-F6) Feedback, matrix, write.** `|fl(f y)| <= f_max (1 + u) |y| + U` with
  `f_max = 0.95` (the `f32` domain bound); the matrix rows are non-negative and sum to at most
  `1 + u`, so a fed word is at most `(1 + u)^3` times the larger sent word plus `3U`; the write
  `flush(fl(x_in + fed))` is at most `(1 + u) (|x_in| + |fed|) + U`.
* **(F7) Output.** `fl(dry + fl(mix fl(tap - dry)))` (or the exact dry or tap arm) is at most
  `(1 + 6u) max(|dry|, |tap|) + 3U`; with a zero dry it is at most `|tap|` exactly.

`f64` evaluation: every computed constant is rounded by `2^-24` toward its bound (`up`, `down`),
far above the few `2^-53` units of one short expression. No rounded expression cancels.

## (N1) The peak, under any admitted control history

Let `X` bound the input. By F1-F6 and induction on the sample index, the set "every ring word is
at most `W` and every damping state at most `S_b`" is invariant, with

```text
kappa = (1 + u)^5 (1 + 3u) (1 + 8u) (1 + 6u)
W     = ((1 + u) X + 16 U) / (1 - f_max kappa)
S_b   = ((1 + 6u) W + 7 U) / (1 - 7u)
```

(F2's overshoot is relative to the tap, not to the state, so it does not compound.) By F7 the
output is at most `g_p X + sigma_p`, `g_p = (1 + 6u)^2 (1 + u) / (1 - f_max kappa)` (20.0 at every
rate) and `sigma_p = ((1 + 6u)^2 16 / (1 - f_max kappa) + 3 (1 + 6u) + 3) U`. This is D1 of the
spec, `|w| <= P / (1 - 0.95)`, with every rounding counted. A restored payload is assumed to be a
snapshot this engine wrote: it holds words some admitted history reached.

## The settled loop

`SETTLE_SAMPLES = max(64, 2 * 128) = 256` samples after the last control event every ramp has
landed exactly on its target and every crossfade (one in flight and one queued) has ended. From
then on each lane's damping coefficient is fixed: `0` or a design of a control in `(0, 0.995]`,
at least `g_lo = g(0.995) (1 - 2^-20)` (`g(0.995)` the 38.3 Hz design: 2.72e-3, 2.50e-3,
1.36e-3, 1.25e-3 at the four rates; the `2^-20` covers any ulp the `f64` design or its `f32`
rounding moves). The taps are exact ring words at most `D` samples old (`D = 2 fs` for the
descriptor).

F2', F3 and F4-F6 make a positive linear majorant of the loop (the two lanes share it: the matrix
takes the larger lane). Take a word written `n` samples after the start at most `c r^n` and the
damping state at most `c sigma r^n`. Both stay so for the next sample when

```text
F(r) = A r^-D (beta + alpha b / (r - a)) <= 1,   A = f_max (1 + u)^5,
sigma = b r^-D / (r - a)
```

because a tap at most `D` samples old is at most `c r^(n - D)` (a younger tap is smaller). `F`
falls as `r` rises, and `r` is found by bisection to adjacent `f64`s with `F(r) <= 1` evaluated
rounded up: a certificate, not an estimate of the root. It is the decay rate of the real loop's
slowest pole up to rounding (gate 3 measures 0.18 % of slack at `D = 48`): the damping's group
delay, about `(1 - g) / (2g)` samples, lengthens each pass. The worst lane is `g = g_lo`: with
`e = 1 - r`, `d/dg (beta + alpha b / (r - a))` has the sign of
`e^2 - 2e + 15ue - 28u + 14u^2 < 0`, so `F` falls as `g` rises, and `sigma`, `b / (r - a)` and the
steady terms below fall too. The identity arm is dominated (`H >= 1`, `sigma r^(D+1) >= 1`). The
start is `c = S_b(P)`: every readable word and both states are at most `S_b`, and `sigma >= 1`.

Underflow adds a constant majorant: `W_U = (8 + 4 A alpha / (1 - a)) U / (1 - A H(1))` per ring
word and `S_U = (b W_U + 4U) / (1 - a)` per state, `H(1) = beta + alpha b / (1 - a)`; the
majorant is `c r^n + W_U` and `c sigma r^n + S_U`.

## The tail

With zero input from `N`, the output is at most the tap (F7), so from `N + 256 + D + n` it is at
most `c_rel P r^n + c_abs U + W_U`, `c = c_rel P + c_abs U`. The tail is

```text
T = 256 + D + n_r,   n_r the smallest n with r^n c_rel < eps / 2
```

and `P* = 2 (c_abs U + W_U) / eps` is the flush floor below which the underflow part exceeds the
other half. `T` = 35,116,372 / 38,231,966 / 70,442,653 / 76,712,855 samples (13.3 minutes at
every rate). The spec's period form (`V(t + L) <= q V(t)`, `L = D + K_d`) gives about 2 % more at
`D = 2 fs`, but 6 to 8 times more at `D = 48` (tail and rest), where gate 3's 80 % non-vacuity check could not hold;
the per-sample majorant replaces it (decision record, A2).

## (N2), the decay and the gains

The majorant is linear in its sources, so it splits into the part from the state at `M + 256`
(at most `c_rel X + c_abs U`, by N1), the part from an input of at most `epsilon` after `M`
(constant: `W_eps = (1 + u) epsilon / (1 - A H(1))`) and the underflow part. So from
`M + T + k D_20` the output is at most `(eps / 2) (1 + 6u) 10^-k X + g_t epsilon + sigma_t`, with
`D_20` the smallest `n` with `r^n < 1/10` (`TailDecay`, 3,979,509 / 4,332,584 / 7,982,893 /
8,693,476), `g_t = (1 + 6u) (1 + u) / (1 - A H(1))` and
`sigma_t = (1 + 6u) (c_abs U + W_U) + 3U`. `g_t` is a little above `g_p` (the damping's `7u / g`
rounding headroom) and `sigma_t` far above `sigma_p` (the state's underflow over `1 / (2g)`
samples), so the delay states `G_p = G_t = ceil_mB(max(g_p, g_t))` (2,605 mB at 44.1 and 48 kHz,
2,608 at 88.2 and 96) and `sigma_p = sigma_t = ceil_mB(max(sigma_p, sigma_t))`: a larger value
still bounds both clauses, and the registry's `G_t <= G_p` and `sigma_t <= sigma_p` hold.

## Exact rest

The rest state `Z` is the reset state: every ring word and both damping states `+0.0` (cursors,
`valid_history`, tap geometry, counters and ramps are not signal state). For a peak `P`, with
`c = S_b(P)`:

1. From `N + 256 + j_w`, `j_w` the smallest `j` with `c r^j < FLUSH_EPS - W_U`, every ring write
   is below `FLUSH_EPS` before its flush, so it is `+0.0`, and stays so.
2. `D` samples later every tap reads `+0.0` and the output is `+-0.0`. The damping state is then
   at most `FLUSH_EPS b / (r - a) + S_U` (`sigma r^D = b / (r - a)`).
3. With a `+0.0` tap the state step is `s' = flush(fl(fl(s - p) - p))`, `p = fl(g s)`. A state
   that is not flushed is at least `FLUSH_EPS`, so `p >= 1e-23` is normal. Since
   `p >= g s (1 - u)` and each difference rounds up by at most `u` of itself,
   `s' <= s (1 - c')` with `c' = 2g (1 - u)(1 + u/2)(1 + u) - 2u - u^2 >= 2g - 3u`, and it never
   passes zero by more than `2.1u s`. So the step is dominated by the rate-form follower walk of
   `effect_runtime::tail::rate_follower_rest_samples` at `c_eff`, the largest `f32` at or below
   `2 g_lo - 3u`, snapped `Below(FLUSH_EPS)`: its majorant `round_ties_up(e - c_eff (1 - u) e)` is
   monotone in `e`, and every larger `g` only steps faster. The walk has no stall
   (`c_eff >> 2^-24`). At `g = 1/2` the state is `+0.0` in one step; at `g = 0` it is the flushed
   tap.
4. One ring length (`2 fs + 3` words) after step 1 every ring word, readable or not, is `+0.0`.

`R(P) = 256 + j_w + max(2 fs + 3, D + walk)`, at `P = 10^(24/20)` and at `P = 1e30` (every
sanitized input); `tail_every_peak = max(T, R(P*))`. The bound is `RestBound::Bounded`.

## What the gates hold (`crates/delay/tests/tail_contract.rs`)

1. **Recompute**: every stated value, re-derived in the test from the descriptor's domain bounds
   and the rate, equals `tail_and_rest` exactly, at every rate.
2. **Soundness**, 48 kHz release, the real kernel: 2,000 ms, feedback 0.95, mix 1, cross 0 and 1,
   damping 0 and 0.995, 60 s of the in-phase constant `P = 1`, then zeros to `T`: every sample is
   within `G_p P + sigma_p`, and the last 2 s before `T` are below `eps P`; a fifth run has a
   crossfade and a feedback ramp in flight at `N`. Record (2026-10-26): largest `|y|` 15.71
   against 20.07; the last sample at or above `eps` 36,191,999 (damping 0) and 36,269,709 (0.995)
   samples after `N`, against `T` = 38,231,966; the in-flight run 32,421,496.
3. **Rest**, 48 kHz, `D = 48` (1 ms): by `tail_bound_at(48_000, 48)`'s `R` = 351,302 the output is
   `+-0.0` and every ring word and damping state equals a fresh delay's under `==`; at damping
   0.995 not at 80 % of it. Record: at rest after 145,408 samples (damping 0) and 350,673
   (damping 0.995, 0.18 % below the bound).
