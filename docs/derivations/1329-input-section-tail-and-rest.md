# Issue #1329: the builtin input section's tail and exact-rest bounds

Decision 15 D15-4(b) asks every node to state how long its output stays audible after the input
stops, and when it reaches exact rest. This note derives both for the builtin input section (trim,
polarity, HPF, LPF), as `math::tail` and `crates/builtins/src/tail.rs` compute them at preparation.
Nothing here runs on the render thread.

## The contract

For an input of peak `P` that is zero from sample `N` on, with no control event at or after `N`
(a ramp may be in flight at `N`; every builtin ramp completes by `N + 64`), and `eps = 10^(-144/20)`:

| value | name in code (`builtins::InputSectionBound`) | meaning |
|---|---|---|
| `T_decay` | `tail`, `TailSamples` | `|y[n]| < P eps` for `n >= N + T_decay`, for every `P >= P*` |
| `T_rest = max(T_decay, R(P*))` | `tail_every_peak` | as above for `P >= P*`; exact `+-0.0` for `P < P*` |
| `R` | `rest`, `RestSamples` | from `N + R` every output is `+-0.0` and every integrator is `+0.0` |

`builtins::input_section_bounds` computes them once per distinct design at preparation, and
`builtins-compiler` keeps each strip's beside its tail (`PreparedBuiltinsSession::input_bounds`),
control-side; the render-owned input section carries none of them (#1329 Amendment 4, R5).

`P*` is the **flush floor**: the smallest peak for which the per-word flush's absolute deviation
fits the `-144 dB` budget. Below it the `f32` output near `FLUSH_EPS = 1e-20` is not relative to
`P` any more (attempt 3, F3: a 1 kHz section at `P = 1.26e-13` still had `|y| >= P eps` 346 frames
after `N`, against an exact-arithmetic tail of 175), so the contract proves exact rest there
instead. Tail reporting (PDC, the C ABI, the browser) uses `T_decay`; silence skipping (#1107) uses
`RestSamples`, which holds for every input up to its stated peak (+24 dBFS, or below the
sanitizer's `1e30`). Disabled filters and gain-only parts report `0` for all three.

## The kernel and its norm

One section, stored-form TPT SVF [SIMPER-SVF] [ZAVALISHIN-TPT], with state `s = (ic1, ic2)`:

```text
v3 = x - ic2;  d1 = a2 v3 - c1 ic1;  d2 = a3 v3 + a2 ic1
v1 = ic1 + d1; v2 = ic2 + d2;        s' = flush(ic1 + 2 d1, ic2 + 2 d2)
y  = m2 v2 + (m1 v1 + m0 x)
```

so `s' = A s + b x`, `y = c . s + d x` with `A = [[1 - 2c1, -2a2], [2a2, 1 - 2a3]]`,
`b = (2a2, 2a3)`, `c = (m1 (1 - c1) + m2 a2, -m1 a2 + m2 (1 - a3))`, `d = m0 + m1 a2 + m2 a3`.
Every Butterworth design (`k = sqrt(2)`) is the bilinear map of one continuous-time matrix, so all
share one eigenbasis. The bounds use the norm `||x||_V = ||R x||_2`, `R = [[1, r], [0, r]]`,
`r = 1/sqrt(2)`, in which an exact design's `A` is a scaled rotation: `||A^n||_V = rho^n` with
`rho(g) = sqrt(1 + g^4) / (1 + sqrt(2) g + g^2)`, `g = tan(pi f / Fs)`, and no Putzer factor
`n rho^n` [ORFANIDIS-ISP]. Conversions to per-word magnitudes cost `||R^-1||_2 = 1.848` and
`kappa = ||R|| ||R^-1|| = 1 + sqrt(2)`. In closed form `||b||_V = 2g / sqrt(1 + t)` and, for
both mixes, `||c||_V* = sqrt(2) / sqrt(1 + t)`, `t = g (g + sqrt(2))`.

## Rounding and the flush

* **State step.** Counting every rounding of the step in its frozen order
  (`gamma_n = n u / (1 - n u)`, `u = 2^-24`) gives a relative error
  `||fl(step) - (A s + b x)||_V <= mu_state ||s||_V + mu_input |x|`; the final rounding of `n1`,
  `n2` adds `u q kappa`, so at the top of the domain `mu_state = 7 u kappa = 1.0073e-6`
  (Amendment 2). No target fuses a multiply-add: `Lane::fma` rounds the product and the sum
  separately on every backend (`crates/lane/src/wide_impl.rs`, `scalar.rs`), so the count matches
  the kernel's roundings exactly on every target.
* **Output mix.** The same count for `y` gives `omega_state ||s||_V + omega_input |x|`.
* **Per-word flush.** A word below `FLUSH_EPS` is zeroed, an absolute perturbation of less than
  `FLUSH_EPS` per word per step: `F = ||R|| sqrt(2) (FLUSH_EPS + 16 * 2^-126)` per step (the second
  term covers gradual underflow). It is never modelled as relative error.
* **Joint flush (#1328 A9).** On a frame whose effect input has been exactly zero for `N_SILENCE`
  frames, a section whose two words are both below `REST_EPS = 1e-14` is zeroed. Both builtin
  sections read the same input counter, so after `N` both are armed from `N + N_SILENCE`.
* **Trim.** `fl(x trim)` is within `(1 + u)` of the product; the trim magnitude is the gain `g_t`
  on `P`. Polarity is its sign.

## D3, D4: a fixed design

`math::tail::fixed_cascade`. The split is `eps / 2` for exact arithmetic and `eps / 2` for `f32`.

* **The contraction `q = ||A||_V`.** In the `V`-basis, `R A R^-1` is exactly
  `[[al + r ga, sqrt(2) be + de - al - r ga], [r ga, de - r ga]]` for `A = [[al, be], [ga, de]]`
  (`sqrt(2) r = 1`). Its larger singular value is `s1 = (sqrt((a + d)^2 + (c - b)^2) +
  sqrt((a - d)^2 + (b + c)^2)) / 2`, a form with no cancelling operation, so its `f64` value is
  within `(1 +- u64)^4` of the exact `s1` of the computed entries. Each entry's own `f64` error is
  at most `16 u64` times the sum of its terms' magnitudes (one rounding of the caller's entry, the
  constant, the product, three additions), and moves `s1` by at most the Frobenius norm of those
  errors. `q` is `(s1 + ||E||_F) (1 + 16 u64)`, a certified upper bound by derivation. The
  textbook form `sqrt((F + sqrt(F^2 - 4 det^2)) / 2)` cancels for every design (the two singular
  values are almost equal) and fell below the exact norm by up to `6.0e-9` (attempt 3, M1;
  `math::tail`'s unit test pins six near-top designs against 60-digit references).

* **Exact half, reset-aware (Amendment 2, F1).** Compare the kernel with an exact-arithmetic
  **reference** that resets each section at the same instants as the kernel (joint flush,
  per-block non-finite recovery). For any reset pattern, by the triangle inequality, the
  reference's state of section `k` at `N + m` is at most `g_t P sum_{t > m} Gbar_k(t)` and its
  output at most `g_t P sum_{t > m} o(t)`, where `Gbar_1(t) = ||A_1^(t-1) b_1||_V` (exact),
  `o_1 = |h_1|`, and for later sections `Gbar_k(t+1) = q_k Gbar_k(t) + beta_k a_k(t)`,
  `a_{k+1}(t) = gamma_k Gbar_k(t) + |d_k| a_k(t)`, `a_2 = |h_1|`, `o = a_{K+1}`. A reset only
  drops non-negative terms. The sum runs exactly to a horizon whose contraction remainder
  `(I - M)^-1 m(now)` is below a sixteenth of the threshold. The exact half is the first `t0` with
  `g_t sum_{t >= t0} o(t) < eps / 2`; the reference output at `N + m` sums from `t = m + 1`, so D1
  needs only `m >= t0 - 1`, and the bound states `t0`, one frame more, so it reads on the same
  index as gate 1(a)'s brute-force suffix `S_b(j)`. For one section the majorant is exact and
  `t0 = T_b(eps / 2)`; for a cascade the triangle inequality on the second section costs 4 % to
  10 % (gate 1(a) record).
* **`f32` half.** With `E_k` the kernel's state deviation from the reference and `D_k` that of its
  input, propagated from their fixed points under the pre-`N` suprema:
  `E_k' <= (q_k + mu_k) E_k + (beta_k + mu_x) D_k + mu_k x_k + mu_x v_k + F` and
  `D_{k+1} <= (gamma_k + omega_k) E_k + (|d_k| + omega_x) D_k + omega_k x_k + omega_x v_k`. The
  relative part (per unit of `g_t P`) must fall below `eps / 4` for good: the propagation is a
  non-negative linear map, so once every component falls it keeps falling. It is walked frame by
  frame until it falls, then its crossing of `eps / 4` is found by powers of the step matrix
  (exponential search and bisection over the non-increasing values). `T_decay` is the later of the
  two halves. The first section's state majorant is summed in closed form,
  `sum_t ||A_1^(t-1) b_1||_V <= beta_1 / (1 - q_1)`, which is exact for an exact design (its `A` is
  a scaled rotation in the `V`-norm).
* **Flush floor.** The absolute part settles at `F a` (`a` the output of the fixed point per unit
  `F`), so the total is below `P eps` from `T_decay` on whenever
  `P >= P* = F a / (eps / 2 - g_t dev_sup)`.
* **Rest, section by section.** The kernel state at `N` is at most
  `g_t P (x_k + E_k) + F E_abs_k`, capped at the `V`-norm of finite `f32` words (a state that
  overflows is cleared by the per-block recovery). Section 1 has zero input after `N`; its state
  bound `rho^m (z0 - z_s) + z_s`, `z_s = F / (1 - rho)`, falls below `REST_EPS / ||R^-1||`, and
  `N_SILENCE` frames are added for the arming (Amendment 2's A9 term, conservative: both sections
  are armed together). Once it rests its output is exactly zero, so section 2 then decays freely,
  propagated through the interval with section 1 still driving it, and rests the same way.
  `R(P)` is that time for peak `P`; `R(P*)` gives `T_rest`, and the two stated peaks give
  `RestSamples`. Every enabled section therefore has `T_rest >= N_SILENCE`.
* A fixed design takes the maximum over its two channels (D4); the prepared trim word is the gain.

## D5: a live input lane

`builtins::input_section_live_bound` / `math::tail::envelope_cascade`. The same kernel
propagation, from per-section suprema over every word a live history can load
(`input_section_live_envelope`):

* **Recursion words.** Under #1407 every word is a design, the identity at rest, or within the
  allowance `E = 64 h + u D` (`h = (u/2, u/4, u/2)`, `D = (1, 0.3, 1)`, at block size 1, its
  worst) of the convex hull of the `f32` designs the history used. `||A(w)||_V + mu_state(w)` is
  convex in `w`, so over the hull it peaks at a design. `rho(g)` is symmetric under `g -> 1/g` and
  smallest at `g = 1`, so on `[10 Hz, max]` it peaks at the maximum cutoff. The designs split at
  `g1 = g_max / 2`: above it `||A||_V <= rho(g_max) + P(h)` and `a2 <= a2(g1)`; below it
  `||A||_V <= max(rho(g1), rho(g_min)) + P(h)` and `a2 <= 1 / (2 + sqrt(2))`. `P(.)` is the
  vertex supremum of `2 ||[[e1, e2], [-e2, e3]]||_V` over a word box. Then
  `rho_ramp = max(top, rest) + P(E)` with the rounding counts at the largest word magnitudes plus
  `E + h`, and `rho_settled` the same without `E`. Certified margin `1 - rho_ramp`: `3.673e-5`
  (44.1, 88.2 kHz) and `3.701e-5` (48, 96 kHz).
* **Input column, output row, feedthrough.** `||b||_V < 2`, `||c||_V* <= sqrt(2) / sqrt(1 + t_min)`
  (10 Hz), `|d| <= 1`, each plus the `E + h` word perturbation, the mix words' own ramp allowance
  (a mix ramp toward or from the identity scales the row by its weight) and
  `|fl(sqrt(2)) - sqrt(2)|` for the HPF band mix.
* **Trim.** `g_t <= fl(10^(24/20))`: a trim ramp stays inside its endpoints (#1408).
* **Before `N`.** Each section's state lies in its invariant ball `B_k = (input_k V_k + F) /
  (1 - rho_ramp)` with `V_1 = g_t P` and `V_{k+1} = output_state_k B_k + output_input_k V_k`: the
  LPF's input is the HPF's output ball, not `g_t P` (Amendment 2, F2, the crude-but-sound
  cascade; #1433 may tighten it).
* **After `N`.** 64 frames under `rho_ramp` (a ramp in flight), then `rho_settled`. `T_decay` is
  the first frame from which the relative output bound is below `eps / 2` for good; `P*` comes
  from the absolute stall through the cascade (about `1.7e-3`: the HPF's stall amplified by the
  LPF's ball); rest is section by section as above, each adding `N_SILENCE`.

Per rate (44.1 / 48 / 88.2 / 96 kHz): `T_decay` 904,785 / 899,524 / 904,795 / 899,533; `T_rest`
1,081,764 / 1,075,575 / 1,085,641 / 1,079,792; `peak_plus_24_dbfs` 1,264,736 / 1,257,840 /
1,268,585 / 1,262,029; `any_sanitized_input` 2,583,197 / 2,569,016 / 2,587,000 / 2,573,156. These
are the module's computed outputs, recorded as evidence; nothing pins them.

## Numerical limits

* Every `f64` rounding is bounded explicitly or covered by an inflation that provably exceeds it.
  The operator norm carries its own error term (above). A short, non-cancelling evaluation of a
  constant (`beta`, `gamma`, `|d|`, the rounding counts, the balls) and every step of a
  propagation (the deviation recursion, the envelope propagator, its powers) is inflated by
  `1 + 2^-30`, far above its few `2^-53` roundings. Where rounding compounds over many frames it
  is bounded separately: the first section's `f64` impulse response carries an error radius
  `e(t + 1) = q e(t) + 8 u64 ||R|| (|s1| + |s2|)` added through its output row, the later
  sections' majorant recursions step up by `1 + 4 u64` per frame, and a sum of `n` non-negative
  terms is inflated by `1 + 2 n u64`.
* The radius's `8 u64 ||R||`: one zero-input step rounds the entry `1 - 2 c1` or `1 - 2 a3`, each
  product and the sum, so each state word errs by at most `3.01 u64 (|A_i1| |s1| + |A_i2| |s2|)`.
  For a section of damping `k >= 0`, `c1 = t / (1 + t)` and `a3 = g a2` lie in `[0, 1)` and
  `2 a2 <= 2 g / (1 + g^2) <= 1`, so every entry of `A` is at most `1` in magnitude. The two
  words' error has a 2-norm of at most `sqrt(2) 3.01 u64 (|s1| + |s2|)`, and a `V`-norm of at most
  `||R||` times that: `4.26 u64 ||R|| (|s1| + |s2|)`, below the `8 u64 ||R||` used (a margin above
  1.8).
* `1 + 4 u64` restores every summand of a recursion step that reaches the result through at most
  two roundings before the product with it (`(1 - u64)^3 (1 + 4 u64) >= 1`). The radius's growth
  term passes through three (a sum, a product, the outer addition); its relative shortfall of at
  most `10 u64^2` is absorbed by the margin of `8 u64 ||R||` above.
* The `1 + 2^-30` step inflation compounds: over the live bound's 0.9M-frame decay it is a factor
  of about `1 + 8.4e-4`, which lengthens the stated values by a few tens of frames (the live
  `T_decay` is 17 frames above an independent frame-by-frame recomputation without it, every
  rest 14 to 42 frames above; `tail_contract` checks the difference stays within 0.01 % plus 64
  frames). It only ever lengthens a bound.
* The longest horizon evaluated is `2^26` frames; a bound beyond it is not stated (`Infinite`, no
  rest bound). No launch-rate design comes near it.
* Every section must contract with its rounding (`q + mu_state < 1`); every flush stall must be
  below `REST_EPS`. At the top of the domain the stall radius is about `6.7e-16` per section, and
  `REST_EPS = 1e-14` clears it.
* Logarithms use `math::log` (vendored, bit-identical across targets), so every target computes
  the same values and the canonical plan text carrying them is target-independent.

## NaN and non-finite behaviour

A non-finite or `|x| >= 1e30` input sample is sanitized to `+0.0` before the trim. A non-finite
state is reset by the once-per-block boundary check, so it never starts a tail; the reset-aware
reference resets with it. The bounds cap the state at the `V`-norm of finite `f32` words.

## Evidence and gates

`crates/builtins/tests/tail_contract.rs`: gate 1(a) against an independent `f64` brute force to
4,000,000 samples; 1(b) over 165,000 designs per section and the kernel's own ramp words; 1(c) and 2
on the real kernel; 1(c)'s identity for the live bound; the live envelope's terms and the live
figures against an independent frame-by-frame recomputation of this derivation (the real kernel
cannot see an omitted rounding, ramp, stall or A9 term at the live bound: the crude cascade's
slack absorbs each); 3 and 7 on the figures. `math::tail`'s unit test checks the operator norm
against 60-digit references. The required CI job `test-release` runs `tail_contract` at release
scale. The measurements are in the #1329 spec's attempt record.

## Citations

[SIMPER-SVF] and [ZAVALISHIN-TPT] for the TPT SVF and its stored-integrator form; [ORFANIDIS-ISP]
for finite-wordlength limit cycles and state-space norms; [SMITH-SASP] for impulse-response and
state-space analysis of IIR filters. The W3C Web Audio `AudioNode.tailTime` notion (a node keeps
producing non-silent output for a stated time after its input becomes silent) is the external
analogue of `T_decay`.
