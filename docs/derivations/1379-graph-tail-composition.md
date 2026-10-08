# Issue #1379: node tail composition values (H1), the fixed input section (issue #1465)

#1379 Amendment 1 (`docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`, H1-H3) asks
every node to state four values beside #1329's tail, so that a graph can carry a node's tail through
the gain after it. This note states the contract and derives the values of the builtin input
section with a fixed design (#1329 D4) and with both filters disabled, as `math::tail` and
`crates/builtins/src/tail.rs` compute them at preparation (issue #1465, slice A2). The live input
lane is slices B1 and B2 (#1466, #1467). Nothing here runs on the render thread. The notation and
the bounds it reuses are #1329's (`docs/derivations/1329-input-section-tail-and-rest.md`).

## The contract (H1)

`eps = 10^(-144/20)` (`TAIL_FLOOR`); `u = 2^-24`; a node with latency `L`, tail `T` (#1329's
`T_decay`), input `x`, output `y`; `N` the first sample of silence. The node states
`effect_contract::CompositionBound::Stated { decay: D, peak_gain: G_p, tail_gain: G_t, stall:
sigma }`, with `g_p = 10^(G_p/2000)`, `g_t = 10^(G_t/2000)` and `sigma` the stall's linear level:

* **(N1) Peak.** If `|x[n]| <= X` for all `n`: `|y[n]| <= g_p X + sigma` for every `n`.
* **(N2) Tail at every decade.** With no control event at or after `N`: if `|x[n]| <= X` for all
  `n` and `|x[n]| <= epsilon` for every `n >= M` (some `M >= N`), then for every integer `k >= 0`
  and every `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`.
* **(N3) Rest.** #1329's `RestSamples`, read with "zero from `M`" for "zero from `N`".

The share `3/4` is #1329's split: `T` certifies `eps / 2` for the exact reference and `eps / 4`
for the relative `f32` deviation; the last quarter is the stall's. So (N2) at `k = 0`,
`epsilon = 0` is #1329 D1 for every peak `P >= P^ := 4 sigma / eps`, and `P* <= P^ < 2 P*` with
#1329's flush floor `P* = F a / (eps / 2 - g dev_core)` (`g dev_core < eps / 4` at `T`). Nothing
#1329 certified changes: `T`, `T_rest`, both rests and `P*` are computed exactly as before.

## A fixed design (#1329 D4)

A channel is its trim word `trim` followed by its enabled sections; `g = |trim| (1 + u)` bounds
`|fl(x trim)| / |x|`. The input section has latency `0`. #1329's walk (`math::tail::fixed_cascade`)
supplies, per unit of `g X`:

* the **reference** (exact arithmetic, reset with the kernel): with the impulse majorants `o(t)`
  of the cascade (#1329, "Exact half"), every reset pattern gives
  `|y_ref[n]| <= g sum_m |x[m]| o(n - m)`, a reset only dropping non-negative terms;
* the **relative deviation** `E_k` (state) and its output term, propagated by the non-negative
  linear map `L` of #1329's `f32` half (`Deviation::step`, `linear_map`) from its fixed point at
  the pre-silence suprema (`Deviation::at_end`);
* the **absolute** deviation from the per-word flush, at most `F a` at every frame
  (`Deviation::absolute`: the fixed point under the per-step drive `F`, which holds at every frame
  because it is the supremum of the recursion started anywhere at or below it).

Define

* `O = SLACK accumulation(n) sum_t o(t)`: the walk's output majorant summed over every frame (the
  block sums, then the contraction remainder), as the module already forms `input_sup[K]`. It
  bounds `sum_t o(t)`, and so the cascade's `l1` norm, for every reset pattern.
* `dev_loud`: the output component of `Deviation::at_end`'s fixed point (the last section's
  `difference`, times `SLACK`), per unit of `g X`. Before the silence the drives are at most their
  suprema and the deviation is at most this fixed point; after it the drives only fall, so
  `dev_loud` bounds the relative output deviation at every frame. (`dev_core`, #1329's supremum
  from `T` on, feeds `P*` only.)
* `stall = F a` (`math::tail`'s `stall`).

### (N1) and the gains: `G_p = G_t = ceil_mB(g (O + dev_loud))`

For every `n`: `|y[n]| <= |y_ref[n]| + g X dev_loud + F a <= g X (O + dev_loud) + F a`, because
`sum_m |x[m]| o(n - m) <= X O`. So `g_p >= g (O + dev_loud)` and `sigma >= F a` give (N1).
`G_t = G_p`: the design's words never change, so an input after `N` meets the operator an input
before it met, and (N2)'s `epsilon` part below needs exactly this gain.

### (N2), the split at `M`

Write the input as its part before `M` (at most `X`) plus its part from `M` (at most `epsilon`).
The reference output is linear in the input with the majorant bound above, so for `n >= M`:

```text
|y_ref[n]| <= g X sum_{t > n - M} o(t) + g epsilon sum_t o(t) <= g X S(n - M + 1) + g epsilon O,
S(j) := sum_{t >= j} o(t).
```

The deviation recursion is linear with non-negative coefficients in its drives (the reference's
state and input bounds), and those drives split the same way: the part of the reference from the
input before `M` starts at `M` at most at the suprema `state_sup`, `input_sup` (per unit of
`g X`) and then contracts exactly as #1329's walk propagates it; the part from the input after
`M` is at most `epsilon / X` times the suprema at every frame. So the relative deviation at
`M + m` is at most `g X dev(m) + g epsilon dev_loud`, where `dev(m)` is #1329's walked deviation
value at frame `m` (its initial state, the fixed point under the full drives, also bounds the
deviation at `M`, because the drives before `M` are at most the suprema). With the stall:

```text
|y[M + m]| <= g X (S(m + 1) + dev(m)) + g epsilon (O + dev_loud) + F a.
```

`g (O + dev_loud) <= g_t` and `F a <= sigma`, so (N2) holds at `m >= T + k D` once
`g S(m) < (eps / 2) 10^-k` and `g dev(m) < (eps / 4) 10^-k` for every such `m` (`S` is
non-increasing, so `S(m + 1) <= S(m)`). At `k = 0` that is #1329's `T` (its two halves are exactly
these two crossings). What remains is `D`: one value with both crossings at `T + k D` for every
`k >= 1`.

### `D`: the closed form (root's amendment to F-D2, 2026-10-08)

The walk stops at `T`. Every `k >= 1` is covered by a contraction certificate anchored at `T`,
for each half separately.

**The two halves as non-negative linear systems.** Let `h_r = eps / (2 g)` and `h_d = eps / (4 g)`
(the module's `threshold` and `quarter`).

* *Reference.* With `m(t) = (||s_1(t)||_V, Gbar_2(t), ..., Gbar_K(t))` the majorant state at frame
  `t >= 1` (the first section's exact state norm, then the later sections' state majorants),
  #1329's remainder argument gives `m(t + 1) <= M m(t)` and `o(t) = alpha . m(t)` for `t >= 1`,
  with `M` the non-negative, lower-triangular `K x K` matrix `M_11 = q_1`,
  `M_ii = q_i`, `M_ij = beta_i alpha_i,j` (`j < i`), and `alpha_i` the input row of section `i`
  (`alpha_2 = gamma_1 e_1`, `alpha_{i+1} = |d_i| alpha_i + gamma_i e_i`, `alpha = alpha_{K+1}`).
  So `S(t + j) <= alpha . M^j w(t)` with `w(t) = (I - M)^-1 m(t)` (the remainder's sums vector),
  because `M` commutes with `(I - M)^-1` and `sum_{t' >= t + j} M^(t' - t) = M^j (I - M)^-1`.
* *Deviation.* With `z(m)` the walked deviation state `(E_1..E_K, x_1..x_K)` at frame `m` and
  `ell` the output row, `dev(m + j) <= ell . L^j z(m)` (#1329's closed-form powers; the floored
  walk is at least the linear one, #1474).

**Anchoring at `T` by powers of the step.** The walk already holds an exact-frame state close to
`T` for each half; it is carried to `T` by powers of the non-negative step, so no frame is walked:

* reference: the majorant state at `T_ref` (#1329's `t0`), recorded while the walk replays
  `t0`'s block (the replay is already walked; recording reads its values). If `t0 = 0` (no
  replay), the anchor is the pass's last frame `H`, where the remainder is formed anyway.
* deviation: the state at the walk's last frame `F_dev` (the frame from which it falls).

If the anchor `A` is at or before `T`, the state is carried to `T` (`w(T) = M^(T - A) w(A)`,
`z(T) = L^(T - A) z(A)`) and the offset is `o = 0`; otherwise the anchor stays at `A`, with
`o = A - T`. Both cases read `B lambda^i` from the anchor below.

**The certificate.** For a non-negative matrix `G` (`M` or `L`), a state `z >= 0` at the anchor
and a row `r >= 0`: if `v >= z` and `G v <= lambda v` with `0 < lambda < 1`, then `G^i z <= G^i v
<= lambda^i v` (induction, `G` non-negative), so the half's value `i` frames after the anchor is at
most `B lambda^i`, `B = r . v`. The least such `v` for a lower-triangular `G` is found by forward
substitution in the triangular order (`M`: section order; `L`: `x_1, E_1, x_2, E_2, ...`, since
`E_i` and `x_i` read only the current values of sections `<= i`):
`v_p = max(z_p, s_p / (lambda - G_pp))`, `s_p = sum_{q before p} G_pq v_q`, which gives
`(G v)_p = G_pp v_p + s_p <= lambda v_p` for every `lambda` above every diagonal entry. The
module computes it for rates above the largest diagonal entry `rho`,
`lambda_j = rho + (1 - rho) 2^(-j/2)` (`j = 1..32`; every fourth `j`, then every `j` within four of
the best of those), and **verifies** `G v <= lambda v` on the computed values (below); a `lambda`
that fails is skipped, and the one with the least `D_h` is kept (the least `j` on ties). Soundness
rests only on the verification, never on the rates or the search.

**The crossings.** With `B lambda^i` bounding the half `o + i` frames after `T`, the first `i`
with `B lambda^i < h 10^-k` is the first integer `i > a + k b`,

```text
a = ln(B / h) / (-ln lambda),     b = ln 10 / (-ln lambda),
```

so the half is below `h 10^-k` from `T + j_k` on, `j_k = o + (0 if a + k b < 0, else
floor(a + k b) + 1)` (its bound only falls). The certified crossing is
`T(k) = T + max(j_k^ref, j_k^dev)` for `k >= 1` and `T(0) = T` (#1329).

**`D`.** Per half `D_h = o + floor(max(a, 0) + b) + 1`, and `D = max(D_ref, D_dev)`.
*Claim:* `T + k D >= T(k)` for every `k >= 1`. *Proof:* per half,
`k D_h = k o + k (floor(max(a, 0) + b) + 1) > o + k (max(a, 0) + b) >= o + a + k b`, because
`k >= 1`, `o >= 0` and `max(a, 0) >= 0`; `k D_h - o` is an integer above `a + k b`, so it is at
least `floor(a + k b) + 1`, and at least `0`. So `k D >= k D_h >= j_k^h` for both halves. (So
`max_{k >= 1} ceil((T(k) - T) / k) = D` is reached at `k = 1`, and `D` is the least value that
the certificate supports.) The asymptotic rate is `D_inf = max_h ceil(b_h)`; the certificate's
own `k = 0` crossing is `T_lambda = T + max_h j_0^h >= T`.

**Why this form.** Root's amendment allows the certificate from `T` or powers of the step for every
`k`. Powers alone give each `T(k)` exactly but only for finitely many `k` (a bisection of
`log2(T(k) - T)` power evaluations each, about 16 x 20 evaluations of a `4 x 4` power for `k <= 16`,
more than a typical design's whole walk), and still need a rate bound for the rest. The
certificate covers every `k` in one formula at the cost of a handful of small solves. The two are
combined only where it tightens the result: powers carry the state to `T` once, so the certificate
starts from the state at `T` instead of the walk's last frame. A certificate started near
`N` must dominate the cascade's polynomial factor (`n rho^n` for two near-equal poles) over the
whole of `T` with one geometric rate, which costs up to about one natural log (0.43 decades) at
the top pair; started at `T` it only covers the factor's change after `T`. The looseness against
the walked crossings is measured in the #1465 attempt record.

**Rounding.** Every quantity is an upper bound of what the proof needs:

* `M`'s entries are products of #1329's computed constants (upper bounds of the exact
  constants), each product inflated by `SLACK`; `L` and `ell` are `linear_map`'s, whose entries
  #1329 already states as upper bounds. A larger matrix only enlarges `G^i z`, and the
  certificate holds for the larger one.
* The remainder's sums `w` are #1329's (`remainders`). Powers (`power_apply`) inflate every
  product by `SLACK`; a product that underflows loses at most `2^-1075`. Every power of a
  non-negative lower-triangular `G` with diagonal below one is entrywise at most its resolvent
  `(I - G)^-1`, whose entries are below `2^220` for these matrices (at most `2K <= 4` rows, each
  `1 / (1 - G_pp) <= 2^53`, off-diagonal entries below `4`; #1474 bounds `M`'s by `2^108`). A power
  by squaring performs fewer than `2^12` roundings per component, so a carried component errs by
  less than `2^12 2^220 2^-1075 < 2^-840` in absolute terms. The carried state is therefore
  rounded up by `tau = 2^-600` per component (`tau` is #1474's flush), which covers it.
* The anchored states are the walk's floored values (each at least `tau`, #1474), upper bounds of
  the exact ones; so is every `v_p >= z_p >= tau > 0` (a positive certificate).
* **The verification.** For each `p`, the module checks `fl(dot(G_p, v) SLACK) <= fl(lambda v_p)`,
  `dot` being the `SLACK`-inflated sum of at most `2K` non-negative products: without underflow its
  value is at least the exact `(G v)_p`, and an underflowing product loses at most `2^-1075`. With
  `fl(x SLACK) >= x SLACK (1 - 2^-53)` and `fl(lambda v_p) <= lambda v_p (1 + 2^-53)`, the check
  gives `dot <= lambda v_p (1 + 2^-53) / (1 + 2^-30 - 2^-52) <= lambda v_p (1 - 2^-31)`, and
  `(G v)_p <= dot + 2K 2^-1075 < lambda v_p`, because `lambda >= 2^-16` (the grid's least value is
  `rho + (1 - rho) 2^-16`) and `v_p >= 2^-600` make the margin `2^-31 lambda v_p >= 2^-647`.
* `B = dot(r, v) + tau`, an upper bound of `r . v` (the `tau` covers any underflowed product).
* The logarithms are `math::log` (musl, within 1 ulp; bit-identical on every target).
  `-ln lambda` is taken as `-log(lambda) / SLACK` (below the exact value), `b` as
  `LN_10 SLACK` over it, and `a` as `(log(B) - log(h) + 2^-30) / (-ln lambda)`: the absolute
  `2^-30` exceeds the logs' errors (at most `2^-52` times `|ln| <= 745` each) and the relative
  error of the computed `h` against `eps / (2 g)` or `eps / (4 g)` (a few `2^-53`). The crossings
  and `D` are `floor` of `(sum) SLACK` plus one, which covers the sum's own rounding.
* `T(k)`, `D` and every `j_k` are integers below `HORIZON_LIMIT = 2^26`; a `D` at or above it is not
  stated (the composition is unstated, as for a design no certificate is verified for).

**The `tau` floors (#1474).** The certificate never floors: the floors were made for the walk's
cost, and the closed form has no per-frame cost. Its inputs are the walk's floored values (upper
bounds), and its one absolute term is the `tau` added to a carried state, so its `D` exceeds the
unfloored value only where a component is within `tau` of zero, which no stated value can see
(`h >= TAIL_FLOOR / 4 / g > 2^-34` for every builtin trim).

### The stall `sigma`

`sigma = Level(ceil_mB(F a))`. `P^ = 4 sigma / eps >= 4 F a / eps >= P*` because
`eps / 2 - g dev_core >= eps / 4` (#1329's `T` puts `g dev_core` below `eps / 4`).

### A channel with its filters disabled

A channel with no enabled section is `y = fl(x trim)`: `D = 0` (the `X` part is exactly the
current input), `T = 0`, rest `ZERO`, and `|y| <= |trim| (1 + u) |x|`, or `|trim| |x|` exactly
when the product is exact, which it is for a power-of-two trim (0 dB included) up to underflow.
So `G_p = G_t = ceil_mB(|trim|)` for a power-of-two trim and `ceil_mB(|trim| (1 + u))` otherwise,
and `sigma` is one underflow, `2^-126`, as a `Level`. (N2) holds with `D = 0`: from `M` on,
`|y[n]| <= g_t epsilon + sigma`.

### Channels

A stereo input section states each value as the maximum over its two channels
(`NodeTailBound::max`); a channel with its filters disabled states the values above, so a
design with one disabled channel is still stated, and a design with both disabled takes the larger
trim's gain (`crates/builtins/src/tail.rs`, `memoryless_input_bound`).

## Numerical limits

* `ceil_mB(g) = ceil(2000 log(g) / ln 10)` with `math::log`. For a value computed by rounded
  operations (`g (O + dev_loud)`, `|trim| (1 + u)`, `F a`) the module adds `|x| 2^-30 + 2^-30` to
  `x = 2000 log(g) / ln 10` before the ceiling, which exceeds the few `2^-53` relative errors of the
  product, the logarithm and the quotient; for an exact value (a power-of-two trim, `2^-126`) it adds
  only `|x| 2^-30`, so `ceil_mB(1) = 0`. Every millibel value is therefore at least the exact one.
* `O` keeps #1329's per-section triangle inequality: sound, and about 14 dB above the cascade's
  exact `l1` at the top pair. Tightening it is #1468 (it would also move `T`, which this slice must
  not).

## Gates

`crates/builtins/tests/tail_contract.rs`, #1465's F1-F3 (the decade law against the independent
brute force, `D`'s tightness and asymptotic floor, the gains' soundness and tightness, the disabled
values, `dev_loud`, the stall) and F4 (every #1329 assertion unchanged); #1457's gates 2 and 8 for
the cost (F5). The measurements are in the #1465 attempt record.

## Citations

As #1329: [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP] (state-space norms), [SMITH-SASP]
(impulse response and state-space analysis); [HIGHAM-ASNA] (N. J. Higham, *Accuracy and Stability
of Numerical Algorithms*, 2nd ed., SIAM 2002, §2.1-2.2) for the rounding model; R. A. Horn and
C. R. Johnson, *Matrix Analysis*, 2nd ed., Cambridge 2013, §8.1 and §8.3 (Perron-Frobenius:
for a non-negative matrix, a positive `v` with `G v <= lambda v` bounds the spectral radius by
`lambda`, and the powers by `lambda^i v`).
