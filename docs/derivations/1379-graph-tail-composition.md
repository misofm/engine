# Issue #1379: node tail composition values (H1), the input section (issues #1465, #1466, #1467)

#1379 Amendment 1 (`docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`, H1-H3) asks
every node to state five values beside #1329's tail, so that a graph can carry a node's tail through
the gain after it. This note states the contract and derives the values of the builtin input
section with a fixed design (#1329 D4) and with both filters disabled, as `math::tail` and
`crates/builtins/src/tail.rs` compute them at preparation (issue #1465, slice A2), the decay,
the peak gain and both stalls of a live input lane (issue #1466, slice B1), and its tail gain and
the statement of all five (issue #1467, slice B2). Nothing here runs on the render thread. The notation and the bounds
it reuses are #1329's and #1433's (`docs/derivations/1329-input-section-tail-and-rest.md`).

## The contract (H1)

`eps = 10^(-144/20)` (`TAIL_FLOOR`); `u = 2^-24`; a node with latency `L`, tail `T` (#1329's
`T_decay`), input `x`, output `y`; `N` the first sample of silence. The node states
`effect_contract::CompositionBound::Stated { decay: D, peak_gain: G_p, tail_gain: G_t,
peak_stall: sigma_p, tail_stall: sigma_t }` (H1 as amended by #1484), with `g_p = 10^(G_p/2000)`,
`g_t = 10^(G_t/2000)` and `sigma_p`, `sigma_t` the two stalls' linear levels, `sigma_t <= sigma_p`:

* **(N1) Peak.** If `|x[n]| <= X` for all `n`: `|y[n]| <= g_p X + sigma_p` for every `n`.
* **(N2) Tail at every decade.** With no control event at or after `N`: if `|x[n]| <= X` for all
  `n` and `|x[n]| <= epsilon` for every `n >= M` (some `M >= N`), then for every integer `k >= 0`
  and every `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma_t`.
* **(N3) Rest.** #1329's `RestSamples`, read with "zero from `M`" for "zero from `N`".

`sigma_p` bounds the flush part of the output at every frame and `sigma_t` only from the node's
tail on; (N2)'s frames are a subset of (N1)'s, so `sigma_t <= sigma_p`. A use that bounds a signal
at every frame reads `sigma_p` (`CompositionBound::peak_clause`), and a use from the node's tail
on reads `sigma_t` (`CompositionBound::tail_clause`). The two differ where the flush part before
`T` is larger than after it (the live input section, slice B1, #1466); for the fixed design below
`F a` holds at every frame, so it is both.

The share `3/4` is #1329's split: `T` certifies `eps / 2` for the exact reference and `eps / 4`
for the relative `f32` deviation; the last quarter is the stall's. So (N2) at `k = 0`,
`epsilon = 0` is #1329 D1 for every peak `P >= P^ := 4 sigma_t / eps`, and
`P* <= P^ < 2 * 10^(1/2000) (1 + 2^-20) P* < 2.0024 P*` with #1329's flush floor
`P* = F a / (eps / 2 - g dev_core)` (`g dev_core < eps / 4` at `T`; "The stalls `sigma_p` and
`sigma_t`" below).
`P^` can exceed `2 P*`: the release `tail_contract` run gives `P^ >= 2 P*` on 16 of its 112 F3
rows, with a maximum ratio of 2.0016 (88.2 kHz, 1 kHz HPF at 0 dB; four more rows round to the
same 2.0016 at four digits: 88.2 kHz, 1 kHz LPF at 0 dB, and three 48 kHz rows). Nothing #1329 certified
changes: `T`, `T_rest`, both rests and `P*` are computed exactly as before.

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
`sum_m |x[m]| o(n - m) <= X O`. So `g_p >= g (O + dev_loud)` and `sigma_p >= F a` give (N1).
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

`g (O + dev_loud) <= g_t` and `F a <= sigma_t`, so (N2) holds at `m >= T + k D` once
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
  `t0`'s block (the replay is already walked; recording reads its values). If `t0 = 0`, the
  anchor is the pass's last frame `H`, where the remainder is formed anyway. The module tests
  `t0 = 0` itself, not "the crossing block is the first": the forward block sum and the replay's
  backward running sum round in different orders, so a threshold between the two gives the first
  block and still `t0 = 0`, and frame 0's record is the state before the impulse, which bounds
  nothing after it (the remainder needs `t >= 1`). `H` is sound for every `t0`: the pass's
  remainder sums `w(H)` bound the half from `H` on, and the offset `o = H - T` claims nothing
  before `H`. No builtin design reaches `t0 = 0`: the threshold is at most about `0.5` (the
  -144 dB trim floor) and `O >= ||h_1||_1 >= 1`, so `t0 >= 1`. (Measured: on a grid of 2,112
  builtin designs, at 4 rates, 11 x 11 cutoffs and 8 trims from -144 to +24 dB, 346 rows have
  `t0 < 256` and the least is `t0 = 2`; a design whose output majorant is concentrated in its
  first frames crosses inside block 0.) Only `math::tail`'s own `gain`
  below the trim floor reaches it (`math` test
  `the_reference_certificate_is_sound_where_the_crossing_is_at_frame_zero`).
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
least `floor(a + k b) + 1`, and at least `0`. So `k D >= k D_h >= j_k^h` for both halves. (When
the half that sets `D` has `a >= 0`, its `j_1 = D_h = D`, so
`max_{k >= 1} ceil((T(k) - T) / k) = D` is reached at `k = 1` and `D` is the least value that
the certificate supports. When that half has `a < 0`, `j_1` can be below `D`, and `D` is the
least value of the form `o + floor(max(a, 0) + b) + 1`, not of every `ceil((T(k) - T) / k)`.)
The asymptotic rate is `D_inf = max_h ceil(b_h)`; the certificate's own `k = 0` crossing is
`T_lambda = T + max_h j_0^h >= T`.

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
  product by `SLACK`; a product that underflows loses at most `2^-1075`. Only a design whose two
  certificates are both verified states a value, so only those carries need to be upper bounds.
  *The resolvent.* A verified rate has `G_pp SLACK^3 < lambda <= 1 - 2^-53` for every `p`, so
  `1 - G_pp > 3 2^-30 - 2^-52 > 2^-29` and `1 / (1 - G_pp) < 2^29` (not `2^53`: `L`'s diagonal
  `q + mu` can be at or above one, and then no certificate is verified). Every power of a
  non-negative lower-triangular `G` with diagonal below one is entrywise at most its resolvent
  `R = (I - G)^-1`. With at most `n = 2K <= 4` rows and off-diagonal entries below `4`, every
  entry of `R` (a sum over the triangular paths of the diagonal's `1 / (1 - G_pp)` and the
  off-diagonal entries) is below `(2^29)^4 (1 + 4)^3 < 2^123` (for `M`, `K <= 2`: below
  `2^29 + 2.85 2^58 < 2^60`). The powers the module forms are inflated by `SLACK` once per
  squaring, at most `SLACK^(2^26) < 1.07` times the exact powers.
  *Where a loss goes.* A loss `e` in an entry `(i, j)` of an intermediate power `G^(2^s)` reaches
  the result `G^m z` as `G^a E G^b z` with `E = e e_i e_j^T` and `a + 2^s + b = m`: two power
  factors, and proportional to the state. Its component `p` is at most
  `1.07^2 R_pi e (R z)_j <= 1.15 n e r^2 max_q z_q`, `r < 2^123` the largest entry of `R`. The
  computed `G^(2^s)` is used many times: it is squared into each higher power, so its loss reaches
  the result about `m / 2^s` times, each copy at a different left exponent `a`. The bound holds for
  the sum of the copies, because their left factors sum to at most `sum_a G^a <= R` (at most
  `1.07 R` for the computed powers), which is the `R_pi` above. A loss `e` in a product of a power
  by the vector reaches the result through one power, at most `1.07 r e`. With fewer than `2^12`
  rounding sites in all (since `m < HORIZON_LIMIT = 2^26`: at most 26 squarings of a `4 x 4`
  matrix, `26 * 16 * 4` products and `26 * 16` `SLACK` products of the entries, and 27
  applications, `27 * 4 * 4` products and `27 * 4` `SLACK` products of the `dot`s; 2,620 sites),
  each `e <= 2^-1075`, a carried component errs by less than
  `2^12 2^-1075 (2^3 2^246 max z + 2^124) = 2^-814 max z + 2^-939`.
  *The states.* Per unit of `g X`, with #1329's builtin constants (`beta_k <= 2`,
  `gamma_k <= 1.42`, `|d_k| <= 1`) and the verified gap: the reference's majorant state has
  `||s_1|| <= beta_1 <= 2` and `Gbar_2 <= beta_2 (gamma_1 2 + 1) 2^29 < 2^32`, so its sums
  `w = R m < 2 2^60 2^32 = 2^93`. The deviation's state is at most `Deviation::at_end`'s fixed point
  (it falls from there): `x_k` the summed state majorants (`state_sup`), below `2^30` and `2^61`;
  each `E_k` its drive over `1 - rho_k`, with every `mu`, `omega` below one, below `2^60` and
  `(3 2^62 + 2^61 + 2^31) 2^29 < 2^93`. So `max z < 2^94` on both halves, and a carried
  component errs by less than `2^-814 2^94 + 2^-939 < 2^-719`. On the gate rows the largest
  carried component is about `2^16` (#1465 attempt-1 verdict).
  The carried state is therefore rounded up by `tau = 2^-600` per component (`tau` is #1474's
  flush), which covers it with a factor above `2^119`.
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
* `D` (and so `j_1 <= D`) is an integer below `HORIZON_LIMIT = 2^26`; a `D` at or above it is not
  stated (the composition is unstated, as for a design no certificate is verified for). `T(k)`
  and `j_k` grow without limit in `k` and are not bounded by it; `HalfCertificate::crossing` is
  read only for the `k` a caller asks for.

**The `tau` floors (#1474).** The certificate never floors: the floors were made for the walk's
cost, and the closed form has no per-frame cost. Its inputs are the walk's floored values (upper
bounds), and its one absolute term is the `tau` added to a carried state, so its `D` exceeds the
unfloored value only where a component is within `tau` of zero, which no stated value can see
(`h >= TAIL_FLOOR / 4 / g > 2^-34` for every builtin trim).

### The stalls `sigma_p` and `sigma_t`

`sigma_p = sigma_t = Level(ceil_mB(F a))`, from the one raw value: `F a` bounds the absolute
deviation at every frame, so it serves (N1) and, a fortiori, (N2) (#1484 S-D5). Write `sigma` for
the common value. `P^ = 4 sigma_t / eps >= 4 F a / eps >= P*` because
`eps / 2 - g dev_core >= eps / 4` (#1329's `T` puts `g dev_core` below `eps / 4`). Above:
`P* >= 2 F a / eps` (`g dev_core >= 0`), and `ceil_mB` adds less than one millibel plus
`|x| 2^-30 + 2^-30` millibels, `x = 2000 log10(F a)` the level in millibels. Every positive finite
`f64` has `|x| < 650,000` mB (from about -646,600 mB at `2^-1074` to about +616,500 mB at the
largest `f64`; the stalls themselves are at about -33,000 to -76,000 mB), so the margin is below
`650,001 2^-30 < 6.1 10^-4` mB and multiplies the level by at most
`10^(6.1 10^-4 / 2000) < 1 + 7 10^-7 < 1 + 2^-20`. So `sigma < 10^(1/2000) (1 + 2^-20) F a` and
`P^ < 2 10^(1/2000) (1 + 2^-20) P* = 2.0023059 P* < 2.0024 P*`. The bound `2 P*` itself does not hold: when
`g dev_core` is very small, `P*` is near `2 F a / eps` and the millibel rounding alone lifts `P^`
above `2 P*` (16 of 112 F3 rows, at most 2.0016, which five rows reach at four digits).

### A channel with its filters disabled

A channel with no enabled section is `y = fl(x trim)`: `D = 0` (the `X` part is exactly the
current input), `T = 0`, rest `ZERO`, and `|y| <= |trim| (1 + u) |x|`, or `|trim| |x|` exactly
when the product is exact, which it is for a power-of-two trim (0 dB included) up to underflow.
So `G_p = G_t = ceil_mB(|trim|)` for a power-of-two trim and `ceil_mB(|trim| (1 + u))` otherwise,
and both stalls (`sigma_p = sigma_t`) are one underflow, `2^-126`, as a `Level`. (N2) holds with
`D = 0`: from `M` on, `|y[n]| <= g_t epsilon + sigma_t`.

### Channels

A stereo input section states each value as the maximum over its two channels
(`NodeTailBound::max`); a channel with its filters disabled states the values above, so a
design with one disabled channel is still stated, and a design with both disabled takes the larger
trim's gain (`crates/builtins/src/tail.rs`, `memoryless_input_bound`).

## The live input section (issue #1466, slice B1)

A live input lane (#1329 D5) may change its trim, polarity and filter targets at any frame before
`N`; #1433's frequency-aware cascade (`math::tail::live_cascade`) bounds it over every admitted
history. This part derives `D`, `G_p`, `sigma_p` and `sigma_t` from that bound
(`math::tail::live_cascade_composition`, `LiveComposition`). `G_t` and the statement of all five
values are slice B2's (#1467, "The live tail gain `G_t` and the statement" below; H2: no partial
statement); preparation reads the live bound from its table and never computes these values.

Notation as #1433 (the #1329 note, "#1433: the frequency-aware cascade"): the trim word's largest
magnitude `trim_max` (the `f32` word of +24 dB, #1408) and `g = trim_max (1 + u)`; `F` the per-step
flush perturbation; the envelope's suprema over both sections' reachable words, `rho = rho_ramp`,
`rho_s = rho_settled`, `iota` (input column), `gamma_2` (`output_state`) and `delta`
(`output_input`, the feedthrough `|d|` with its rounding); the window `N ..= N + 64`; the settled
groups `c` (#1433's `SettledTerms` that no other group covers, the HPF at the identity included). An
*admitted history* is any input with `|x| <= X` and any sequence of words and resets #1433 admits:
every word reachable, consecutive words related by #1433's moves, the trim at most `trim_max`.

### What #1433 bounds, and two properties this part uses

For an admitted history whose input is zero from `N` and which has no control event at or after
`N`, #1433 bounds the output at every frame from `N` on:

* *window* frame `N + j`, `0 <= j <= 64`: `|y[N + j]| <= W_j(g X, F)`, the window's output bound
  (`LiveBound::window`, `outputs[j]`);
* *settled* frame `N + 65 + m`, the HPF's settled design in a zone that group `c` covers:
  `|y| <= r_c . M_c(F)^m u_c(g X, F)`, the non-negative system on `(tau, H, X, 1)`
  (`settled_system`, `settled_start`; a covered zone has every term at most its group's, and the
  system is non-negative and increasing in every term).

**(P1) Superposition of the drives.** Every inequality #1433 propagates is a non-negative linear
recursion in its bound, driven by the input's magnitude and by `F`: the first section's
`E' <= rho_k E + beta_k |x| + F` along the zones `k` the history visits, the first section's output
`|y_1| <= 1/2 |m| q_k E + delta |x| + omega E + F`, the second section's weighted sums, the
potential's per-frame inequality, and the settled system with its joint-flush terms. Run each
recursion twice along the same history (the same zones and resets at every frame): once with the
input drive alone (`F = 0`) and once with `F` alone (input `0`). By induction over frames, the sum
of the two runs satisfies the recursion with both drives and so bounds the kernel wherever #1433's
bound does; and each run satisfies every bound #1433 derives, with its own drive, because each step
of that derivation (the zone invariant `Phi`, the potential `Psi`, the window, the settled system,
the covering of a zone by its group) holds for any non-negative drives. So at every frame the output
is at most `X R + A`, with `R` the bound at unit peak and `F = 0` (the *relative part*, per unit of
`X`) and `A` the bound at `X = 0` (the *flush part*). This is the split #1433 already reads
(`T_decay` from the relative part, `P*` from the flush part).

Two points of the split are not arithmetic of the bound's formula and must be stated:

* *Smaller of two bounds.* Where #1433 takes the smaller of two bounds of one quantity (`H_0`, the
  smaller of the window's state bound and the zone's `Phi`), each run takes its own smaller bound:
  the run is at most each of its bounds. The split value can be below the joint one: in general
  `min(a1, a2) + min(b1, b2) < min(a1 + b1, a2 + b2)`, and at every launch rate 21 of the 60 to 63
  groups take the window's bound in one part and `Phi` in the other. The split is sound because
  the runs are separate majorants, not because of the minimum.
* *The cap.* #1433 caps a state bound at the `V`-norm of finite `f32` words. The cap bounds the
  kernel's state, not a run, so a split value is a bound only where the cap was not taken. The
  module states no composition when the window took the cap in either part (`LiveWindow::capped`;
  a NaN state bound counts as capped, since `NaN.min(cap)` would continue as the cap);
  at the launch rates the relative window's largest state bound is about `1.1e7` per unit of the
  peak against a cap of `6.3e38`.

**(P2) The window's first frame bounds every frame.** `W_0(g X, F)` reads only the frames before
`N`: the first section's state at `N` (at most `sup Phi`, which holds at every frame of every
admitted history), the second section's state at `N` (the potential term `sup Psi Phi`, bounded by
Abel summation with the non-decreasing weights `rho^(N-1-j)`, and the charges, the direct zones'
term, the feedthrough, the rounding and `F_sum`, each a weighted sum over the earlier frames at most
`1 / (1 - rho)` times its largest drive; every term holds for a history of any length), and the
input at `N`, which is zero. Its hypotheses on those frames (`|x| <= X`, reachable words, #1433's
moves, the trim) are those of any admitted history; silence and the absence of control events are
hypotheses on the frames from `N` on, which `W_0` does not read. The state at a frame `n` depends
only on the frames before `n`, and the frames before `n` of any admitted history are an admitted
history; so the two sections' states at any frame `n` of any admitted history are at most #1433's
bounds at `N`.

### (N1): `G_p` and `sigma_p`

Split the output by (P3)'s construction (below, in "The live tail gain"), with two parts in place
of three: the *input part*, driven by the trimmed input `x' = fl(x trim)` (`|x'| <= g |x| <= G`,
`G = g X`) with the relative perturbations, and the *flush part*, with zero input and the flush
perturbations. Each part is an exact cascade under the history's words and resets that satisfies
every per-frame inequality of #1433 with its own drive (`F = 0` for the input part, input `0` for
the flush part), and the output is at most the sum of the two parts' magnitudes.

* *`sigma_p`* (root's ruling of 2026-10-08): by (P2) applied to the flush part, at every frame of
  every admitted history the flush part is at most `W_0(0, F)`. `sigma_p = ceil_mB(stall(0))`,
  #1433's flush part with the tail at frame `0` (`LiveBound::stall(0)`: the largest of the flush
  window's `W_j(0, F)` over every window frame and of every group's fixed point plus its decaying
  start from the settled phase's first frame, times `SLACK`). By construction
  `stall(0) >= W_0(0, F)`, so it bounds the flush part at every frame. Measured, `stall(0)` is
  `W_0(0, F)` plus 0.08 % to 0.16 %, at most 1.4 mB.

**The input part: `G_p` (issue #1485).** #1466 bounded the input part at any frame by the window's
first frame, `W_0(g, 0) + delta^2 g`, whose second section is frequency-blind: its state is at
most `iota sum_j rho^(n-1-j) |y_1[j]|` and its output `gamma_2` times that, so every frame of the
first section's output is charged `gamma_2 iota rho^m` (summing to `7.6e4`) `m` frames later.
#1485 bounds the second section along its own zones instead. Notation as #1433 per zone `k`:
`rho_k`, `q_k`, `beta_k` (ramp constants), `Phi_k`, `Psi_k`, whether it is direct, its neighbours
`nb(k)`; `h = first_mix_row / 2`, `omega = first_output_rounding`, and the second section's
`h_2 = second_mix_row / 2`, `omega_2 = second_output_rounding` ("The second section's output row"
below). Both sections' recursion words lie in the shared domain, so a zone's constants hold for
either section's words in it. At frame `j` the first section's word lies in zone `k_j` and the
second's in zone `l_j`; everything below is per unit of `G`.

* *The first section* (#1433): `E_j <= Phi_(k_j)` and `|y_1[j]| <= a_j`,
  `a_j = (h q_(k_j) + omega) E_j + delta |x'_j|`. At the identity `E = 0` and `y_1 = x'`, so
  `a_j <= delta`.
* *The second section*: `||sigma_(j+1)||_V <= rho_(l_j) ||sigma_j||_V + beta_(l_j) |y_1[j]|` (the
  state step at the zone's constants, its rounding inside them) and
  `|y[j]| <= (h_2 q_(l_j) + omega_2) ||sigma_j||_V + delta |y_1[j]|` (its output row on the state is
  `1/2 (m1, m2) (I + B)` exactly, `||I + B||_V <= q_l`; the output rounding is inside `omega_2`
  and `delta`). A reset, a rule-3 replacement or a completed disable sets `sigma` to zero; at the
  identity `sigma = 0` and `y = y_1`.

Unrolling the second section from the history's start (a zero state only drops terms; a channel
copy carries words and state together, so the copied state with its path is an admitted one, as
in (P3)), and since consecutive words are #1433's moves, so consecutive zones are neighbours:

```text
|y[n]| <= delta a_n + sum_{m >= 1} Gamma(m) a_(n-m),
Gamma(m) = max over l_0, ..., l_m with l_(i+1) in nb(l_i) of
           beta_(l_0) (prod_{i=1}^{m-1} rho_(l_i)) (h_2 q_(l_m) + omega_2).
```

*The exposure.* With `B_0(l) = h_2 q_l + omega_2` and `B_r(l) = rho_l max_{l' in nb(l)} B_(r-1)(l')`
(the largest product of `r` contractions along a path from `l`, times the exposure at its end),
`Gamma(m) = max_l beta_l max_{l' in nb(l)} B_(m-1)(l')`. The module computes it for
`m <= H + 1`, `H = 1024` (`EXPOSURE_HORIZON`), each value rounded up (`STEP_UP`, `SLACK`) and
floored at `tau`. The tail: with `c = B_H` and `lambda = rho_ramp (1 + 2^-40)`, the module checks
`rho_l max_{nb(l)} c <= lambda c(l)` at every zone (each side moved by `STEP_UP`). The step is
monotone, so by induction `B_(H+s) <= lambda^s c`, hence `Gamma(m) <= lambda^(m-H-1) Gamma(H + 1)`
and `sum_{m > H+1} Gamma(m) <= Gamma(H + 1) lambda / (1 - lambda)`. The module states no
composition when the check fails (at the launch rates it holds with the shape settled: the ratio
is `rho_ramp` to within `1e-16`).

*The split.* Let `Gammabar(m) = sup_{m' >= m} Gamma(m')`, non-increasing. For any `m0 >= 1`,
`Gamma <= Gamma_f + Gamma_s` with `Gamma_s(m) = Gammabar(max(m, m0))`, non-increasing, and
`Gamma_f(m) = Gammabar(m) - Gammabar(m0)` for `m < m0`, else `0`.

* *Fast part*: `sum_m Gamma_f(m) a_(n-m) <= A sum_{m < m0} (Gammabar(m) - Gammabar(m0))`, with
  `A = max_k (h q_k + omega) Phi_k + delta >= a_j` at every frame (the zone at a frame sets both
  `q_k` and `Phi_k`; #1466 paired `q_max` with `Phi_max`).
* *Slow part*: the weights `w_j = Gamma_s(n - j)` are non-decreasing in `j`. #1433's potential
  gives, at every frame, `[k_j not direct] q_(k_j) E_j <= P_j - P_(j+1) + Psi_(k_(j+1)) beta_(k_j)
  |x'_j|` with `P_j = Psi_(k_j) E_j >= 0` (at a direct zone the left side is `0` and the right
  side non-negative). Abel summation with non-decreasing weights and `P >= 0` gives
  `sum_j w_j (P_j - P_(j+1)) <= w_(n-1) sup P = Gammabar(m0) V`, `V = sup Psi Phi`. The rest of
  `a_j` is `[k_j direct] h q_(k_j) E_j + omega E_j + delta |x'_j|`, all at the frame's own zone, so
  `sum_m Gamma_s(m) a_(n-m) <= h Gammabar(m0) V + C sum_m Gamma_s(m)`, with
  `C = max_k (h (max_{nb(k)} Psi beta_k + [k direct] q_k Phi_k) + omega Phi_k + delta)` (#1466
  summed the three charges' separate suprema) and
  `sum_m Gamma_s(m) = (m0 - 1) Gammabar(m0) + sum_{m >= m0} Gammabar(m)`.

So (N1) holds with

```text
g_p = g min_{m0} [ delta A + A sum_{m < m0} (Gammabar(m) - Gammabar(m0)) + h Gammabar(m0) V
                   + C ((m0 - 1) Gammabar(m0) + sum_{m >= m0} Gammabar(m)) ],
```

`m0` over `1 ..= H + 1` (every `m0` gives a bound; the smallest is stated) and `sigma_p` above. The
module forms every sum and product rounded up (`SLACK`; the fast part as a running sum of
non-negative terms `(m0 - 1)(Gammabar(m0 - 1) - Gammabar(m0))`) and `G_p = ceil_mB(peak_gain)`. It
reads neither the window nor (P2): the bound holds at every frame of every admitted history.

* *The live loud-input deviation.* The constants carry the kernel's `f32` deviation (#1433: `rho`
  includes `mu_state`, `beta` `mu_input`, the output rows `omega_state`, `delta` `omega_input`),
  so the bound includes the deviation of a loud input.
* *Not the settled designs' `l1`.* `V` is `sup Psi Phi`: the state a slow pole built while the
  input was loud, which a later retarget exposes. The settled designs' `l1` (3.51 for the 10 Hz
  HPF into the top LPF; +34.9 dB with the trim) does not bound it: #1379 H9's `f64` probe reaches
  +67 dB before the trim, and the real kernel under gate 1's L1 history reaches +90.75 dB with it.
* *What sets `G_p`* (per unit of `G` at 44.1 kHz): `m0 = 16`, `Gamma(1) = 0.870`,
  `Gammabar(16) = 0.267` (a state the second section holds at the top of the domain, exposed only
  by moving through decaying zones), `sum Gamma = 7.27e3` against #1466's `7.6e4`; `A = 7.27e3`,
  `C = 5.25`, `V = 1.60e5`. The terms: `delta A` 7.27e3, the fast part 6.07e3, the potential
  `h Gammabar(m0) V` 3.02e4, the slow charges `C sum Gamma_s` 3.82e4; in all `8.17e4`, `g_p`
  `1.294e6` (+122.24 dB) against #1466's `1.629e7` (+144.24 dB).
* *Looseness.* The real kernel's largest peak over gate 1's histories (the three-event worst-sign
  history's, +95.53 dB at 44.1 kHz) is 26.7 dB below `G_p` (21.55 to 21.66 times; the ratios per
  history and rate are in the #1485 attempt record, "Follow-ups 2"). The histories come from a
  finite search, so this peak is a measured lower bound of the worst case and the looseness is an
  upper bound of the true one. The
  rest is frequency-blindness at the top of the domain: either section's state grows there in the
  `V`-norm at `rho_ramp` per frame, where the kernel's own resonance is narrower, and the
  potential charges every frame of input at the top with the mass a later retarget could expose.

### (N2): `sigma_t` and `D`

Under (N2)'s hypotheses with the input zero from `M` (the `epsilon` part is slice B2's, below), #1433
applies with its silence from `M`: no control event at or after `M`, and a retarget started before
`N <= M` completes by `M + 64`. By (P1) the output at `M + n` is at most `X R(n) + A(n)`.

* *`sigma_t`.* #1433's `stall(T)` bounds the flush part from `M + T` on (the flush window's
  outputs from `T`, and per group the driven system's fixed point plus its decaying start from `T`:
  `u = U + M_h^m (u_0 - U) <= U + M_h^m u_0`, with `M_h` the system without its constant, and the
  decaying start's supremum is taken until it falls componentwise, after which it only falls).
  `stall(0)` bounds the flush part at every frame (above), so both bound it from `T` on, and the
  module states the smaller: `sigma_t = ceil_mB(min(stall(T), stall(0)))`. So `sigma_t <= sigma_p`
  (#1484's rule (g)) holds by construction. At every launch rate `stall(T)` is the smaller, by
  54.6 to 61.3 dB, and `sigma_t = ceil_mB(stall(T))`: the stall #1433's `P*` reads
  (`P* = SLACK stall(T) / (eps / 2)`).
* *`k = 0`.* `R(n) < eps / 2` for `n >= T` (#1433's `T_decay`).
* *`k >= 1`, the certificate per group.* If `T >= 65` (every launch rate: `T` is about 700,000),
  every frame from `M + T` on is settled, and the relative part at `M + 65 + m` is at most
  `r_c . S_c^m z_c` for the group `c` that covers the HPF's settled zone: `S_c` is #1433's system
  without the flush, whose constant component is then zero and decoupled, on `(tau, H, X)`,

  ```text
  S_c = [[rho_s, a_H, 0], [0, r_c, 0], [0, 0, max(rho_s, r_c)]],
  ```

  lower triangular in the order `H, tau, X`; `z_c` its relative start (`settled_start` at
  `(g, 0)`) and `r_c` the first three entries of its output row. The fixed design's certificate
  (above, "The certificate", "The crossings", "`D`") applies to each group unchanged, with
  `G = S_c`, the threshold `h = eps / 2` per unit of `X` (`g` is inside `z_c`), and the anchor at
  `T` (`z_c(T) = S_c^(T - 65) z_c`, carried by powers of the step and rounded up by `tau`); if
  `T < 65`, the anchor is the window's end with `o = 65 - T`, so `D_c >= 66 - T` and `T + k D` lies
  after every window frame for `k >= 1`. The HPF's zone is not known, so `D = max_c D_c` and
  `T(k) = T + max_c j_k^c`; the claim `T + k D >= T(k)` and its proof are the fixed design's, per
  group. So for every `k >= 1` and `n >= M + T + k D`:
  `|y[n]| <= (eps / 2) 10^-k X + sigma_t <= (3/4) eps 10^-k X + sigma_t`.
* *The transient.* Measured at `T`, every group's `a_c` is negative (the largest per rate is
  -0.91, -0.54, -0.39 and -0.60 frames), so `D = max_c (floor(b_c) + 1)`, and `D = D_inf`. The certificate's
  rate is not the largest diagonal entry `rho_s`: the coupling `a_H` of `H` into `tau` (a
  polynomial factor `i rho_s^i` where `r_c` is near `rho_s`) makes a rate near `rho_s` cost a large
  `B`, and the search settles at `1 - lambda = 4.93e-5` (44.1 kHz), about 3 % below
  `1 - rho_s = 5.09e-5`. The module's
  directly searched crossings (#1433's search at `(eps / 2) 10^-k`) show the same: 46,099 frames for
  the first decade at 44.1 kHz, against `ceil(ln 10 / -ln rho_s) = 45,220`.

**Rounding (the live part).** The fixed design's rounding argument holds with these changes for the
carry of `z_c` to `T`. `S_c` has `n = 3` rows and one off-diagonal entry, `a_H`; the module states
no composition unless every off-diagonal entry is at most `8` (measured `0.842`) and every start
component at most `2^132` (`LIVE_STATE_LIMIT`; measured at most `1.08e7`, about `2^23.4`). A
verified rate gives `1 / (1 - G_pp) < 2^29` for every diagonal entry (as above), so the resolvent's
entries are below `2^29` on the diagonal and `a_H 2^29 2^29 <= 2^61` off it: `r < 2^61`. The powers
are within `1.07` of the exact ones (at most 26 squarings). With at most 26 squarings of a `3 x 3`
matrix (`26 * 27` products and `26 * 9` `SLACK` products) and 27 applications (`27 * 9` products
and `27 * 3` `SLACK` products), fewer than `2^11` rounding sites lose at most `2^-1075` each, and a
carried component errs by less than
`2^11 2^-1075 (1.15 * 3 * 2^122 * 2^132 + 1.07 * 2^61) < 2^-807`. The carried state is rounded up by
`tau = 2^-600` per component, and a start used without a carry (`T` at or before the anchor) is
raised to at least `tau`, so every certificate state is positive, as the verification needs. The
verification, `B`, the logarithms, `D`'s integer bound and `HORIZON_LIMIT` are the fixed design's.

### Values at the launch rates

Per rate (44.1 / 48 / 88.2 / 96 kHz), the module's raw values (`live_cascade_composition` at the
+24 dB trim word) and their millibels:

| value | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| `T` (#1433) | 704,010 | 699,952 | 704,018 | 699,960 |
| `D` (= `D_inf`) | 46,678 | 46,421 | 46,678 | 46,421 |
| `g_p` raw (#1485) | 1.294092e6 | 1.280357e6 | 1.294087e6 | 1.280352e6 |
| `sigma_p` raw (`stall(0)`) | 2.041952e-12 | 2.207801e-12 | 4.142272e-12 | 4.486749e-12 |
| `sigma_t` raw (`stall(T)`) | 3.793538e-15 | 3.772735e-15 | 3.797574e-15 | 3.852977e-15 |
| `G_p` mB (#1485) | 12,224 | 12,215 | 12,224 | 12,215 |
| `sigma_p` mB | -23,379 | -23,312 | -22,765 | -22,696 |
| `sigma_t` mB | -28,841 | -28,846 | -28,840 | -28,828 |

The millibels are `ceil(2000 log10)` of the raw values; slice B2 states them with the module's
`ceil_mB` (below, "Numerical limits"), with the same results. The groups, the certificate rates and
the cost are in the #1466 attempt record. #1466's `g_p` was `1.629e7` / `1.613e7` / `1.632e7` /
`1.615e7` (14,424 / 14,416 / 14,426 / 14,417 mB); #1485's is 22.0 dB below it.

## The live tail gain `G_t` and the statement (issues #1467, slice B2, and #1485)

(N2) needs `G_t`: at every frame `n >= M`, a bound of the part of the output that the input from
`M` on produces, for every admitted history before `N`. This part derives it from #1433's
inequalities and #1407's ramp law (`math::tail::live_cascade_composition`,
`LiveComposition::tail_gain`; #1485 restated it) and states all five values. Notation as the
live part above; in addition `G = g epsilon` (the trimmed input from `M` is at most `G`),
`h = first_mix_row / 2`, `omega = first_output_rounding` (the first section's
mix row and output rounding), `h_2 = second_mix_row / 2`, `omega_2 = second_output_rounding` (the
second section's, below), and for a zone `k`: `rho_k`, `q_k`, `beta_k` its ramp constants
(`contraction`, `sum_norm`, `input`), `r_k`, `q_k^s` its settled ones (`settled.contraction`,
`settled.sum_norm`), `nb(k)` its neighbours.

### (P3) The decomposition at `M` is linear

Fix an admitted history. The kernel's frame is exact arithmetic plus perturbations: per section,
`s_{n+1} = R_n (A_n s_n + b_n v_n + e_n)` and `w_n = c_n . s_n + d_n v_n + f_n`, with `v_n` the
section's input (the trimmed input `x'_n = fl(x_n trim)` for the first section, the first
section's output for the second), the words of frame `n`, and `R_n` the identity or zero (a joint
flush, the per-block recovery, a rule-3 replacement; #1433's moves). #1329's rounding count and
#1433's constants split each perturbation into a relative part, `||e_n^rel||_V <= mu ||s_n||_V +
mu_x |v_n|` and `|f_n^rel| <= omega ||s_n||_V + omega_x |v_n|` (the constants are taken over every
reachable word: `mu` inside `rho_k`, `r_k`, `rho_ramp`, `rho_settled`, `mu_x` inside `beta_k` and
`iota`, `omega_x` inside `delta`), and the flush part (the per-word flush and the underflow
allowance), at most `F`.

Define three *parts* of every state, input and output by induction over frames. At the history's
first frame every part is zero. At frame `n`, with the parts' states known and summing to the
kernel's: the first section's input splits as `x'_n [n < M]` (the *early* part), `x'_n [n >= M]`
(the *late* part) and `0` (the *flush* part); each later input is the previous section's output
part. The flush part of each perturbation goes to the flush part. The relative part `e_n^rel`
satisfies `||e_n^rel||_V <= a_1 + a_2 + a_3`, `a_i = mu ||s_n^i||_V + mu_x |v_n^i|` (the triangle
inequality on the sums), and is split in proportion: `e_n^i = e_n^rel a_i / (a_1 + a_2 + a_3)`
(zero when the sum is), so `||e_n^i||_V <= a_i`; the same for `f_n^rel`. Each part then steps by
the same exact map `s' = R_n (A_n s + b_n v + e)`, `w = c_n . s + d_n v + f`, which is linear in
`(s, v, e, f)`, so the parts' states and outputs again sum to the kernel's. A channel copy (the
mono collapse) replaces a channel's state and words with the other channel's, and so its parts with
the other channel's parts, which are parts of an admitted history with the same input bounds; the
bounds below hold for every channel, so they hold across a copy.

So every part is an exact cascade under the history's words and resets, driven by its own input
and by perturbations bounded relative to its own state and input; only the flush part carries
`F`. Each of #1433's per-frame inequalities holds for each part with that part's input and with
`F = 0` for the early and late parts. The early part is B1's relative part with the input zero
from `M` (B1's (N2), with #1433's silence from `M`), the flush part is B1's flush part, and the
late part's state is exactly zero up to and including frame `M` (its input and perturbations are
zero before `M`). The output is at most the sum of the three parts' magnitudes, so (N2) holds with

```text
g_t >= sup over n >= M of |y_late[n]| / epsilon,
```

and `G_t` is a bound of that supremum per unit of `epsilon`. Where the kernel's state is exactly
zero at `M`, the early and flush parts are zero from `M` on and the late part is the whole output
(H1).

### The second section's output row

The second section's output row on the state is `1/2 (m1, m2) (I + B)` exactly, as the first's
(`v1`, `v2` are the half sums of the states; the identity holds for any mix words), so its output
from the state is at most `(h_2 q + omega_2) ||sigma||_V` for a word in a zone with `q`.

*The second section's mix row.* The second section is the low-pass: its mix words are `(0, 0, 1)`
designed, the identity `(1, 0, 0)` disabled, and their mixtures within #1407's mix allowance
(`docs/derivations/1329-input-section-tail-and-rest.md`, D5), so `(m1, m2) = theta (0, 1)` plus the
allowance box, and `||(m1, m2)||_V* <= ||(0, 1)||_V* + ` the box's largest dual norm
`= sqrt(2) + mix_box` (`crates/builtins/src/tail.rs`, `second_mix_row`, times `1 + 2^-30` for its
`f64` evaluation). `first_mix_row` does not cover it: `||(-k, -1)||_V*` with the `f32` word
`k = fl(sqrt(2))` is `2.4e-8` below `sqrt(2)`. The output rounding on the state, `omega_state`, is
a supremum over every word of either section (its value at the largest words, whose mix words
dominate both mixes, plus its value at the opposite corner), so `second_output_rounding =
omega_state`, the value `first_output_rounding` also takes.

### (R) The late part along the in-flight ramps (issue #1485)

The late part has zero state at `M` and input at most `G = g epsilon` at every frame from `M`. #1467
bounded it with each section's words free to take any of #1433's moves on the window's frames and
the second section frequency-blind (`G_t = 9,242` mB); #1485 uses what (N2)'s hypotheses say about
the words and bounds the second section along its own zones, as in (N1).

*The words.* No control event lies at or after `N <= M`. By #1407's rules
(`docs/rulings/builtins-input-liveness-d2.md`; the stage's `apply_prepared_filter`), the last event
of a section before `N` either ramps all six words from the current words `c` (a reachable word) to
a design `t` over `R = 64` updates (rule 4), freezes the recursion words at `c` while the mix ramps
and replaces the state with zero and the words with the identity when the ramp completes (rule 2, a
disable), or jumps the recursion words to the design with zero state (rule 3, an enable from rest),
and a word that no event moves holds. The word after ramp update `i` is the mixture
`(1 - i/R) c + (i/R) t` within #1407's allowance (`ramp_word_allowance`, at most `E` per word), and
`Re p = 1 - c1 - a3` is linear in the words, so its `Re p` lies within `eta` (the ramp box's image)
of `(1 - i/R) Re p(c) + (i/R) Re p(t)`. Every ramp completes by `N + R <= M + R`. So on the window's
frames `M + f`, `0 <= f < R`, a section's words are, for some zone `s` holding `Re p(c)`, some zone
`t` holding `Re p(t)` (a design, so `t` has settled constants) and some `r` in `0 ..= R` (the
updates left at `M`): at frame `M + f` with `f < r`, a word whose `Re p` lies in the interval
`I(i) = (1 - i/R) [low_s, high_s] + (i/R) [low_t, high_t] +- eta`, `i = R - r + f`, hence in some
zone that meets `I(i)`, so every constant is at most its largest over those zones; from `M + r` on,
the design `t` (its zone's settled constants). Or, for a disable, a word in zone `s` (constants over
the zones meeting `[low_s, high_s] +- eta`) for `r` frames, then zero state and the identity. Or the
identity throughout, or a jump at an event before `N` (the design from `M` on: the case `r = 0`).
The mono collapse's channel copy runs only between channels whose plans are equal (the collapse
gate, `plan_is_channel_symmetric`), so a copied channel continues the same ramp. The zone each
constant is read in is where the word lies; nothing else about the path is assumed.

*The first section on the window.* For every `(s, t, r)` and the disable paths: `E_0 = 0`,
`E_(f+1) <= rho_f E_f + beta_f` (per unit of `G`, the frame's constants), output
`a_f <= (h q_f + omega) E_f + delta` at `M + f`, `f < R`. Let `A_f` be the largest `a_f` over every
path, and `E*_t` the largest `E_R` over the paths that end at the design in `t`. At the identity the
state is zero and the output its input, at most `delta`, which every path's `a_f` covers.

*The second section on the window.* Its input at `M + f` is at most `A_f`, whatever the first
section's path. For every `(s, t, r)` and the disable paths: `S_0 = 0`,
`S_(f+1) <= rho_f S_f + beta_f A_f`, output at most `(h_2 q_f + omega_2) S_f + delta A_f`; the
window part `W*` is the largest over every path and `f < R`, and `S*_t` the largest `S_R` over the
paths to `t`. At the identity the output is its input, at most `A_f <= delta A_f`.

*Settled, from `M + R`.* The words are designs: the first section's in a zone `k`, the second's in
a zone `l`, with settled constants (`r_k`, `q_k^s`; `beta_k` the zone's). With
`c_k = h q_k^s + omega`, `Ebar_k = beta_k / (1 - r_k)` and `s` frames after `M + R`:
`E_s <= r_k^s E*_k + Ebar_k (1 - r_k^s) <= max(E*_k, Ebar_k)`, the first section's output
`a_s <= c_k E_s + delta`, and

```text
S_s <= r_l^s S*_l + beta_l sum_{i < s} r_l^(s-1-i) (c_k (r_k^i E*_k + Ebar_k) + delta)
    <= S*_l + beta_l c_k E*_k / (1 - min(r_k, r_l)) + beta_l (c_k Ebar_k + delta) / (1 - r_l),
```

because each `r_l^(s-1-i) r_k^i` is at most `r_k^i` and at most `r_l^(s-1-i)`. The output is at
most `(h_2 q_l^s + omega_2) S_s + delta (c_k max(E*_k, Ebar_k) + delta)`; `Z*` is the largest over
every pair `(k, l)`. A section at the identity (state zero, output its input) is covered by any pair
(`delta >= 1`), and a reset or a joint flush only lowers a state: these plain majorants carry no
one-off term, unlike #1433's `tau` system.

### `G_t`

For every `n >= M`, `|y_late[n]| <= max(W*, Z*) G = g max(W*, Z*) epsilon`, and the module states
`tail_gain = g max(W*, Z*) SLACK` (`LiveBound::tail_gain`; every step a rounded-up product or sum,
the intervals' ends padded by `1e-15` above their `f64` rounding) and `G_t = ceil_mB(tail_gain)`.
`Z*` sets it at every launch rate (at 44.1 kHz `Z* g = 815.8` against `W* g = 433.7`).

**On the kernel.** The exact supremum of the late part over every input `|x| <= 1` from `N` (the
time-varying cascade's row `l1`, gate L2's oracle) is close to the largest settled pair's `l1` and
at most +35.59 dB on gate L2's scanned histories. The largest settled pair is not the 10 Hz HPF into
the top LPF (3.53 at 44.1 kHz, +35.0 dB with the trim, and less at the other rates): over a
61-point logarithmic grid of HPF cutoffs into the top LPF (and a coarser grid of both cutoffs),
the largest settled `l1` is about 3.798, +35.59 dB with the trim at every launch rate, on a flat
maximum near an HPF of `0.00342 fs` into the top LPF: about 151 Hz (44.1 kHz), 164 Hz (48 kHz),
301 Hz (88.2 kHz) and 328 Hz (96 kHz) (the #1467 attempt-2 verifier's golden-section refinement;
the grid's points are within `1.1e-4` of it, and no bound depends on the frequencies). `G_t`
(+58.2 to +58.3 dB) is 22.6 to 22.7 dB above it (13.6 to 13.7 times; the #1485 attempt record).
What remains is the settled pair bound's frequency-blindness (a plain `V`-norm majorant per section,
where the kernel's cascade of a low HPF into a low LPF passes little) and the window's transient
state carried into it (`E*`, `S*`), the second section's input taken as the first's largest output
over every path.

### The statement

`input_section_live_bound` states `CompositionBound::Stated` with `decay = D`,
`peak_gain = ceil_mB(G_p)`, `tail_gain = ceil_mB(G_t)`, `peak_stall = Level(ceil_mB(sigma_p))` and
`tail_stall = Level(ceil_mB(sigma_t))`, each `ceil_mB` the module's rounding up of a computed
value ("Numerical limits"). Rule (g) and A1's `tail_gain <= peak_gain` run only in the effect
registry, which never sees this bound, so the statement checks both on the millibel values and
states `Unstated` when either fails (equality is admissible). `sigma_t <= sigma_p` holds raw by B1's
construction (the smaller of `stall(T)` and `stall(0)`) and `G_t <= G_p` holds raw as measured
(63.85 to 64.01 dB apart); `ceil_mB` is non-decreasing, so the raw orders give the millibel ones.
At the launch rates the statement is made (the values below). Preparation reads the statement from
`input_section_live_bound_table`, which the table test holds equal to the computed bound.

### The tail gain at the launch rates

Per rate (44.1 / 48 / 88.2 / 96 kHz), the module's raw value (`LiveComposition::tail_gain`, at the
+24 dB trim word) with its parts from gate L3''s recomputation, and the statement:

| value | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| window part, `W* g` | 4.336808e2 | 4.336794e2 | 4.336809e2 | 4.336793e2 |
| settled part, `Z* g` | 8.157735e2 | 8.166609e2 | 8.212876e2 | 8.217405e2 |
| `g_t` raw | 8.157736e2 | 8.166610e2 | 8.212877e2 | 8.217405e2 |
| `G_t` mB (stated) | 5,824 | 5,825 | 5,829 | 5,830 |
| `G_p` mB (stated) | 12,224 | 12,215 | 12,224 | 12,215 |

The statement at every launch rate is `Stated { decay, peak_gain, tail_gain, peak_stall,
tail_stall }` with B1's `D`, the stalls of B1's table above, #1485's `G_p` and `G_t` (+58.24 to
+58.30 dB), 63.85 to 64.01 dB below `G_p`. #1467's `G_t` was 9,242 mB (+92.42 dB) at every rate. The
exact supremum of the late part on gate L2's scanned histories is at most +35.59 dB (gate L2; the
#1485 attempt record).


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
the cost (F5). The measurements are in the #1465 attempt record. The live part: #1485's gate 1,
#1466's L1 widened (`G_p` and `sigma_p` against the real kernel's peak on seven named histories:
five after a Nyquist drive and a retarget, and two driven by the exact row's own worst-sign input),
#1485's gate 2 on the same runs (`g_meas <= g_p` and the stated `G_p` at most `F_p g_meas`, with
`g_meas` the largest measured peak), L3 (`D`, `G_p`,
`sigma_p` and `sigma_t` against an independent plain-`f64` recomputation of this part), L4 (`D`
and each certificate's crossing against the module's directly searched crossings for `k = 0..64`,
which must increase strictly in `k`, and `D_inf` against the settled contraction's floor), L5
(nothing certified moves) and the refusal test (a capped or NaN window and a carry outside its
rounding argument give no values); the measurements are in the #1466 attempt
record, and #1485's restated `G_p` with its gate 1 in the #1485 attempt record. The tail gain and
the statement: #1467's L2 (`G_t` at least the exact row-`l1` supremum of
the time-varying cascade, by an adjoint oracle, over a stated scan of histories whose words are
recorded from the real kernel), #1485's gate 2t on the same scan (`s_scan <= g_t` and the stated
`G_t` at most `F_t s_scan`, with `s_scan` the largest trim x exact supremum), L3' (`G_t`
against the plain-`f64` recomputation of "The live tail gain"), L5' (the statement is the
accessors rounded up, with `G_t <= G_p` and `sigma_t <= sigma_p` raw and in millibels) and the
statement's own unit test (equality admissible, a tail value above its peak value refused); the
measurements are in the #1467 attempt record, and #1485's restated `G_t` (L2 unchanged, L3'
restated) in the #1485 attempt record.

## Citations

As #1329: [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP] (state-space norms), [SMITH-SASP]
(impulse response and state-space analysis); [HIGHAM-ASNA] (N. J. Higham, *Accuracy and Stability
of Numerical Algorithms*, 2nd ed., SIAM 2002, §2.1-2.2) for the rounding model; R. A. Horn and
C. R. Johnson, *Matrix Analysis*, 2nd ed., Cambridge 2013, §8.1 and §8.3 (Perron-Frobenius:
for a non-negative matrix, a positive `v` with `G v <= lambda v` bounds the spectral radius by
`lambda`, and the powers by `lambda^i v`).
