# Issue #2114: the reverb's tail and exact-rest bounds

Decision 15 D15-4(b) asks every node to state how long its output stays audible after its input
stops, and when it reaches exact rest. This note derives both for `miso.reverb`
(`crates/reverb`), as `reverb::tail_bound_at(sample_rate, decay_max_ms)` computes them on the
control thread (about 25 µs per rate in release). The descriptor states it at the longest decay,
20,000 ms. The contract and its notation are #1329's
(`docs/derivations/1329-input-section-tail-and-rest.md`, "The contract"): `N` the first sample of
silence, `eps = 10^(-144/20)`, `u = 2^-24`, `U = 2^-126`, `FLUSH_EPS = 1e-20`. The kernel is R1-R12
of `dsp-research/reverb.md`; every operation counted below was checked against
`crates/reverb/src/kernel.rs`. The method follows #1374 (`docs/derivations/1374-delay-tail-and-rest.md`):
every rounding counted, every `f64` constant rounded by `2^-24` toward its bound, a bisection
certificate for the rate, the rate-follower walk for the filter states.

## Result

| rate | `T` = `tail_every_peak` = `R` (samples) | seconds | certified `delta` | `a = 3 ln 10 / (fs T60)` | `delta / a` |
|---|---|---|---|---|---|
| 44.1 kHz | 17,114,518 | 388.1 | 7.5623e-6 | 7.8319e-6 | 0.966 |
| 48 kHz | 18,656,990 | 388.7 | 6.9416e-6 | 7.1956e-6 | 0.965 |
| 88.2 kHz | 34,595,085 | 392.2 | 3.7613e-6 | 3.9160e-6 | 0.961 |
| 96 kHz | 37,711,916 | 392.8 | 3.4527e-6 | 3.5978e-6 | 0.960 |

`rest` is `RestBound::Bounded` with both peaks equal to `R` (the bound does not depend on the
input). `composition` is `CompositionBound::Unstated` (OQ1). The settled loop decays at least as
fast as `r = 1 - delta` per sample; `a` is the designed rate at decay 20 s (`r0 = e^(-a)`).

## Why the bound does not start from the input peak

The note's S1 storage `E` weights a filter state by `1 / (2G)`. That is a storage function only at
a fixed coefficient. Under `damping_hz` or `low_cut_hz` automation the weight changes from sample
to sample, and the time-varying TPT section is not passive: a scratch measurement
(`scratchpad/opt/2114/hp_pump2.py`, exact arithmetic, the kernel's 64-sample ramps) charges the
high-pass state at 500 Hz, ramps to 20 Hz, flips the input sign and releases for 150 samples,
repeated. Its output energy is 1.63 times its input energy. The loop's margin per trip is only
`1 - g_max^2 = 0.34 %` (`g_max = 0.99828`, decay 20 s, size 0.25, the shortest line), so any filter
`l2` gain above 1.0017 breaks the argument, and an `l-inf` argument fails on the matrix (row sum
`4 g_max`). No peak bound linear in the input `X` is proved under every history, and none is
stated. The bound below starts instead from the block check, which holds under every history.

## B. The block invariant (every history, every input)

The block check (`PreparedReverb::block_is_finite`) tests, once per block, both outputs, every word
each ring received in the block (16 lines, 8 diffusers, 2 pre-delay rings), the 32 filter states
and the 8 pipeline registers against `|x| < L = BLOCK_LIMIT = 1e30`. A failing block zeroes both
outputs, zeroes the filter states and registers and sets `history = 0`. A tap at delay `d` reads a
word only when `d <= history`, which is a word written after the last drop. Call such a word
*readable*; the state payload writes every other ring cell as `+0.0`, and only readable words are
signal state.

1. Every intermediate value of a sample ends in a checked word of the same block: a ring write,
   an output, or (the last sample's) filter state or register. An `f32` overflow gives `+-inf`, and
   it or the `NaN` it makes reaches a checked word. So in a passing block no operation overflows,
   and every operation obeys `|fl(z) - z| <= u |z| + U`.
2. At every block boundary, every readable word, filter state and register is below `L`, or the
   state is the rest state `Z` (all `+0.0`, nothing readable).
3. After a failing block that ends at or after `N`, the output is exactly `+-0.0` and the state is
   `Z` for good: zero input and zero readable state compute only zeros (`fl(k (+0)) = +0`,
   `+0 + -0 = +0`, `flush` maps `-0` to `+0`).

A restored payload is assumed to be a snapshot this engine wrote at a block boundary (the
assumption #1374 makes), so it also satisfies 2.

**Settle.** Control events land at block starts before `N`. Every ramp (the five coefficient ramps
and the sixteen gain ramps) takes 64 samples. A line or pre-delay crossfade takes 128, and one in
flight plus the one queued behind it end within 256 (`kernel.rs` starts the queued fade on the
first sample after the running one ends). So from `t0 = N + 256` every coefficient is exactly its
design, every tap is one ring word at a fixed `D_i`, and the pre-delay reads one word at a fixed
`P`.

**The state at `t0`.** Let `B0` be the block that holds `t0`. If `B0` or a later block fails, 3
gives the result from there. Otherwise, by 2, every readable ring word written before `t0` is below
`L` (it was written in `B0` or in an earlier passing block). The filter states inside `B0` follow
#1374's hull rule (F2), which holds for every coefficient word in `[0, 1/2]`: every damping and
low-cut word is a design or a ramp word inside the hull of two designs (#1409), and every design
is at most `1/2` (the cutoff cap). With every tap blend at most `S_t = (1 + 6u) L + 3U` (#1374 F1):
low-pass states at most `S_a = (S_t + 4U) / (1 - 7u)`, their outputs at most
`(1 + 3u) S_a + 2U`, high-pass states at most `S_b = ((1 + 3u) S_a + 6U) / (1 - 7u)`. A pipeline
register is `fl(d - fl(k v))` of two words below `L`: at most `(1 + u)(1 + (1 + u) k) L + 3U`.

## S1. The settled storage

From `t0`, with `r = 1 - delta`, let

```text
Phi[n] = sum_i sum_(k=1..D_i) r^(-2 (D_i - k)) W_i(n, k)^2 + gamma_c^2 sum_j Psi_j
Psi_j  = Gamma_2c^2 theta_1c a_j^2 + theta_2c b_j^2
```

`W_i(n, k)` is the readable word of line `i` written `k` samples before `n` (the tap reads
`k = D_i`, weight 1; clear-on-read and the history rule make an unread word count once), `a_j`,
`b_j` are slot `j`'s low-pass and high-pass states, and the constants are S2-S3's. One sample
shifts every pending word one step closer to its read (weight times `r^2`), removes the read word,
and adds the sixteen new words with weight `r^(-2 (D_i - 1))`. So `Phi[n+1] <= r^2 Phi[n]`
(no injection) follows from

```text
(*)  sum_i r^(-2 (D_i - 1)) u_i^2 + gamma_c^2 sum_j Psi'_j <= r^2 (||t||^2 + gamma_c^2 sum_j Psi_j)
```

`u_i = c_i w_i` with `w = H16 F` and `||H16 F|| = 4 ||F||`, so the left ring part is at most
`r^2 gamma^2 ||F||^2` with `gamma = max_i g_i r^(-D_i)`. Every design's gain is
`g_i = fl32(e^(-a_T D_i)) <= e^(-a D_i) (1 + u)(1 + EVAL)` (`a_T >= a`, the decay at most 20 s), so
for `r > e^(-a)` the maximum is at the shortest line, `D_min` (line 0 at size 0.25: 223, 239, 439,
479 samples). `(*)` then needs, per slot, the `r`-scaled dissipation of the filter cascade with gain
`1 / gamma`.

At the start, `sqrt(Phi[t0]) <= L sqrt(sum_i (r^(-2 D_i) - 1) / (r^(-2) - 1)) + 4 sqrt(Gamma_2c^2
theta_1c S_a^2 + theta_2c S_b^2)`, the `D_i` at size 2 and the `theta` at their maxima: 3.5e32 at
48 kHz, 5.0e32 at 96 kHz.

## S2. The r-scaled TPT section

One section (R5, or R6's inner low-pass) has `s' = A s + B x`, `A = 1 - 2g`, `B = 2g`; its output is
`y = (1 - g) s + g x` (low-pass, `C = 1 - g`, `D = g`) or `f = x - y = (1 - g)(x - s)` (high-pass,
`C = -(1 - g)`, `D = 1 - g`). With `rho = 1 - delta`, the claim is

```text
y^2 + theta (s' / rho)^2 <= Gamma^2 x^2 + theta s^2      for all s, x
low-pass:  Gamma = g (2 - delta) / (2g - delta),           theta = (1 - g)(2 - delta) rho / (2 (2g - delta))
high-pass: Gamma = (1 - g)(2 - delta) / (2 - delta - 2g),  theta = (1 - g)^2 (2 - delta) rho / (2g (2 - delta - 2g))
```

`Gamma` is the scaled response at `z = 1` (low-pass) or `z = -1` (high-pass). The quadratic form
`Gamma^2 x^2 + theta s^2 - y^2 - theta (s'/rho)^2` vanishes on that steady state `(v_s x, x)`, and
`theta` is the value that makes `(v_s, 1)` a kernel vector of its matrix (first row:
`theta = C Gamma / (B / rho)`, sign-adjusted for the high-pass). With a kernel vector the matrix is
positive semidefinite exactly when its first diagonal entry is non-negative:
`M11 = (1 - g)(2 (1 - g) - delta (2 - delta)) / (2 rho)` (low-pass) and
`M11 = (1 - g)^2 (2g - delta (2 - delta)) / (2 g rho)` (high-pass). Both hold for
`delta <= delta_cap = g_2,lo / 2`, half the smallest high-pass design. At `delta = 0` both give
`Gamma = 1`, `theta = 1 / (2G)`: the note's storage. The matrices were also checked positive
semidefinite in exact rationals on 4,000 random `(g, delta)` (`scratchpad/opt/2114/section_check.py`).

`Gamma_LP` falls and `Gamma_HP` rises with `g`, so the worst designs are damping 1 kHz
(`g_1,lo`) and low cut 500 Hz (`g_2,hi`). The two sections combine with the storage `Psi`: the
high-pass inequality plus `Gamma_HP^2` times the low-pass one gives `f^2 + Psi'/rho^2 <=
(Gamma_LP Gamma_HP)^2 t^2 + Psi`.

## S3. Rounding, and the certificate

**A section.** The kernel's order is `d = fl(x - s)`, `h = fl(g d)`, `y = fl(h + s)`,
`s' = flush(fl(y + h))`, and for the high-pass also `f = fl(x - y)`. Against the exact step from
the same `(s, x)`, each operation adds at most `u |z| + U`:

```text
|y - y*|  <= alpha_y |s| + beta_y |x| + nu_y U,  alpha_y = (1+u) g k2 + u (1-g), beta_y = (1+u) g k2 + u g
|s' - s'*| <= alpha_s |s| + beta_s |x| + nu_s U, alpha_s = (1+u)(alpha_y + g k2) + u (1-2g), beta_s = (1+u)(beta_y + g k2) + 2ug
|f - f*|  <= ((1+u) alpha_y + u (1-g)) |s| + ((1+u) beta_y + u (1-g)) |x| + ((1+u) nu_y + 1) U
```

with `k2 = 2u + u^2`; the flush only lowers a magnitude. By the triangle inequality in the norm
`sqrt(y^2 + theta (s'/rho)^2)` and Cauchy-Schwarz, the computed section satisfies S2's inequality
with `(1 + lambda) Gamma` and an added `N U` inside the norm, where
`lambda = sqrt((alpha_o / sqrt(theta) + alpha_s / rho)^2 + (beta_o + sqrt(theta) beta_s / rho)^2)`.
The factor `(1 + lambda)` on the state is a rate: the section is exact at `rho' = 1 - delta - lambda`
and the rounding brings it back to at most `r`. Every coefficient rises with `g`, so `lambda` is
taken at the top of each range with `theta` at its range's extremes: `lambda_1 = 7.2e-7` (12.1u),
`lambda_2 = 3.6e-7` (6.0u) at 48 kHz.

**The loop path.** The butterfly's four stages each round every word by at most `u` in `l2`
(`(1 + u)^4`), the gain product and the injection add round once each, and the gain word's `f32`
rounding is one more: `gamma_c = (1 + u)^7 (1 + EVAL) e^(D_min (nu+(delta) - a))`,
`nu+(delta) = delta + delta^2 / (1 - delta) >= -ln r`.

**The certificate.** `gamma_c Gamma_1c Gamma_2c <= 1`, with
`Gamma_1c = (1 + lambda_1) Gamma_LP(g_1,lo, delta + lambda_1)` and
`Gamma_2c = (1 + lambda_2) Gamma_HP(g_2,hi, delta + lambda_2)`, every factor rounded up. It rises
with `delta`; `delta` is bisected to adjacent `f64`s on `[0, min(a, delta_cap)]`, keeping the lower
end certified. The worst design is the corner decay 20 s, size 0.25 (`D_min`: the per-trip margin
`D_min (a - delta)` must pay the low-pass group delay, about `1 / (2 g_1)` samples), damping 1 kHz,
low cut 500 Hz. Every design in the domain is dominated by it, so one `delta` covers them all. The
certified rate is 3.4-4.0 % slower than `a` (table).

## S4. The loop with injection and underflow

The new words are `u_i = flush(fl(fl(c_i w_i) + q_i))`. Collecting S3 and the absolute terms
(the butterfly's `U` per word and stage: `4 (1 + sqrt 2 + 2 + 2 sqrt 2) (1 + u)^3 U` in `l2`), each
sample gives

```text
sqrt(Phi[n+1]) <= r sqrt(Phi[n]) + C_q |q[n]| + C_U
C_q = (1 + u) sqrt(sum_i r^(-2 (D_i - 1))),   C_U = (4 gamma_c N_slot + A_U) U
```

(`r^2 Z + (r sqrt(Y) + C)^2 <= (r sqrt(Z + Y) + C)^2` joins the shifted and the new parts).
`C_q` = 4.13 at decay 20 s. The output reads `w_14`, `w_15` of the same butterfly:
`|o| <= (1 + u)^4 ||f|| + ...` and `||f|| <= sqrt(Phi) / gamma_c + 4 N_slot U`; width and mix
(dry `+-0.0` after `N`) add `2 (1 + u)^5`. So `|y[n]| <= (2 (1+u)^9 / gamma_c) sqrt(Phi[n]) + A_out U`.

## S5. The pre-delay drain and the diffusers

**Phase A, `[t0, t_p)`, `t_p = N + floor(fs / 4)`.** The pre-delay still reads input words (below
`L`). A diffuser stage at fixed `k` (R2) satisfies `|v| <= (1+u)^2 k |d| + (1 + u) |a| + (2 + u) U`
before its flush, and, from the allpass form `y* = (1 - k^2) d - k a`,
`|y| <= c_d |d| + c_a |a| + c_U U` with `c_d = (1 - k^2)(1 + u) + O(u)`, `c_a = k + O(u)`. These give
`l-inf` invariants per stage (`1 + 2k = 2.4` per stage, `33 L` at stage 3), rising with `k`, so
taken at `k_max = fl32(0.7)`. The injection `q = fl(0.5 e)` is at most `0.5 Y_3 + U`, and

```text
sqrt(Phi[t_p]) <= sqrt(Phi[t0]) + (floor(fs/4) - 256)(C_q Q_A + C_U)     (8.0e35 at 48 kHz)
```

**Phase B, from `t_p`.** Stage 0's input is zero. With a rate `rho_d = 1 - delta_d`, every stage
word is at most `V_s rho_d^(n - t_p) + V_s^U` when `v_d rho_d^(-M_s) < 1`, `M_s` the stage's longer
ring over the two channels: `V_s = max(V_s^A, v_a A_s / (1 - v_d rho_d^(-M_s)))`, its output
coefficient `c_d V_s rho_d^(-M_s) + c_a A_s`, and the next stage's input coefficient the larger of
phase A's output and that output over `rho_d` (the register is one sample old). Then

```text
sqrt(Phi[t_p + m]) <= max(r, rho_d)^m K + Phi_U,   K = sqrt(Phi[t_p]) + C_q Q_hat / |delta - delta_d|,
Phi_U = (C_q Q_U + C_U) / delta
```

`delta_d` is tried at 64 fractions of its feasible range and the smallest result is kept (each is a
valid bound). At decay 20 s the best is `delta_d` = 2.26e-4 (48 kHz), `K = 1.52e36`; the loop rate
`delta` decides.

## S6. The tail

From `t_p`, `|y| <= (2 (1+u)^9 / gamma_c)(max(r, rho_d)^m K + Phi_U) + A_out U`. The audible tail is
`T_maj = floor(fs/4) + m`, `m` the first with the decaying part below the absolute part, which then
gives `|y| < P eps` for `P >= P* = 2 (absolute part) / eps` (8.0e-23 at 48 kHz). `T_maj` =
21,932,254 at 48 kHz, longer than the rest bound `R` of S7: from `N + R` the output is exactly
`+-0.0`, below `P eps` for every `P > 0`. So the stated `T = min(T_maj, R) = R`, and
`tail_every_peak = max(T, R(P*)) = R`.

## S7. Exact rest

The rest state `Z` is the reset state: every readable word, filter state and register `+0.0`
(pre-delay words are the input's own zeros, equal under `==`). Cursors, taps, the history counter,
ramps and crossfade state are not signal state.

1. **Diffusers.** From `t_p + m_d`, `m_d` the first with `V_s rho_d^(m) + V_s^U < FLUSH_EPS` for
   every stage, every diffuser write is `+0.0`; one ring length later every diffuser word is; stage
   `s`'s output is `+0.0` `s + 1` samples after that (`fl(+0 - fl(k (+0)))`). So `q = +0.0` from
   `t_p + m_d + M_max + 4`.
2. **Ring writes.** A new word is at most `r^(D_i - 1)` times the right side of S4's step, so from
   `t_p + m_w`, `m_w` the first with `K max(r, rho_d)^m < FLUSH_EPS - Phi_U`, every line write is
   `+0.0`, and stays so (the majorant only falls).
3. **Taps** read `+0.0` from `t_p + m_w + D_max`; every line word is `+0.0` one ring length after
   `t_p + m_w`.
4. **Filter walks.** Then `Phi <= FLUSH_EPS^2`, so `|a_j| <= FLUSH_EPS / (gamma_c sqrt(theta_1,min))`.
   With a `+0.0` tap the low-pass step is #1374's `s' = flush(fl(fl(s - p) - p))`, `p = fl(g s)`,
   dominated by `effect_runtime::tail::rate_follower_rest_samples` at the largest `f32` at or below
   `2 g_1,lo - 3u`: 3-6 samples. Meanwhile the high-pass input is that low-pass output, so by F2 its
   state is at most `max(FLUSH_EPS / (gamma_c sqrt(theta_2,min)), ((1 + 3u) a_0 + 6U) / (1 - 7u))`;
   once the low-pass rests its input is `+0.0` and it walks at `2 g_2,lo - 3u`: 122-266 samples.
   Then every `f_j` is `+0.0` and the output `+-0.0`.
5. **Pre-delay.** Every word holds an input word written at or after `N` by `N + pre_len`.

`R = max(t_p + m_d + M_max + 4, t_p + m_w + max(line ring, D_max + walks), N + pre_len) - N`.
At decay 20 s, `m_w` decides (18,636,207 of 18,656,990 at 48 kHz): the ring decays from `K` near
`1e36` to `1e-20` at `delta`.

## Histories the bound covers

Every admitted control history before `N`: any automation of the eight controls inside their
domains at any block start, back-to-back size and pre-delay crossfades, every ramp and crossfade in
flight at `N`, resets, any block partition, and any input, including non-finite and above-limit
input (the block that carries it fails). Assumed: a restored payload is a snapshot this engine
wrote at a block boundary. Not covered, by the contract: a control event at or after `N`.

## What the gates hold (`crates/reverb/tests/tail_contract.rs`)

1. **Soundness**, release, the real kernel at 48 and 96 kHz: decay 20 s, size 2, damping 20 kHz,
   diffusion 1, low cut 20 Hz, pre-delay 250 ms, 5 s of noise at `P = 1`, then zeros to `T`; a
   second run moves all six controls 32 samples before `N` from decay 10 s, size 1, damping 6 kHz,
   diffusion 0.5, low cut 500 Hz, pre-delay 0, so crossfades and ramps are in flight at `N`. The last
   2 s before `T` are below `eps P`. Record (2026-10-26): the last sample at or above `eps P` is
   2,183,474 (45.5 s) and 2,082,101 (in flight) at 48 kHz, 4,412,872 (46.0 s) and 4,267,583 at
   96 kHz, against `T` = 388.7 s and 392.8 s (slack 8.5 times).
2. **Rest**, both profiles, 48 kHz, the decay domain restricted to 100 ms
   (`tail_bound_at(48_000, 100.0)`, `R` = 264,636): diffusion 1, pre-delay 250 ms, damping 1 kHz,
   low cut 20 Hz, size 0.25 and 2, 1 s of noise at `P = 1e29`, then zeros. By `R` the output is
   `+-0.0` and every signal-state word of the payload equals a fresh instance's; at size 2 not at
   80 % of `R`. Record: at rest after 209,920 (79.3 %) and 216,828 samples (81.9 %).

The stated values are not re-derived in a test: soundness and the 80 % non-vacuity check hold them.

## Open questions for root

- **OQ1, composition.** The bound starts from `BLOCK_LIMIT`, not from the input peak, because no
  linear peak bound is proved under coefficient automation (the section above). So no `G_p`, `G_t`
  or flush stall is stated, and `composition` is `Unstated`. A linear bound would need either a
  proof that the closed loop cannot be pumped by `damping_hz` / `low_cut_hz` automation (the open
  filter can, by 1.63 in energy), or a design change (for example coefficient ramps slow enough, or
  a storage that tolerates them). Not guessed here.
- **OQ2, the length of `T`.** `T` is about 6.5 minutes at every rate, 8.5 times the measured tail
  from `P = 1`. It is certified, but tail reporting and PDC will carry it. A peak-relative `T`
  (the note's row (b), about 5.6 decay times) needs OQ1 first.
