# Issue #1379: node tail composition values (H1), the input section (issues #1465, #1466)

#1379 Amendment 1 (`docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`, H1-H3) asks
every node to state five values beside #1329's tail, so that a graph can carry a node's tail through
the gain after it. This note states the contract and derives the values of the builtin input
section with a fixed design (#1329 D4) and with both filters disabled, as `math::tail` and
`crates/builtins/src/tail.rs` compute them at preparation (issue #1465, slice A2), and the decay,
the peak gain and both stalls of a live input lane (issue #1466, slice B1; its tail gain and the
statement are slice B2, #1467). Nothing here runs on the render thread. The notation and the bounds
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
(`math::tail::live_cascade_composition`, `LiveComposition`). `G_t` is slice B2's (#1467), and
`input_section_live_bound` states `CompositionBound::Unstated` until B2 states all five values (H2:
no partial statement); preparation reads the live bound from its table and never computes these
values.

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

At a frame `n` of an admitted history with `|x| <= X` at every frame, (P2) bounds both sections'
states as at `N`. The output at `n` differs from the window's first frame only by the current input
`x[n]`: the trimmed input `|x'| = |fl(x trim)| <= g |x|` enters the first section's output through
its feedthrough (`|y_1| <= 1/2 |m| q E + delta |x'| + omega E + F`, #1433's inequality before
`N`) and that output enters the second section's output through its feedthrough
(`|y| <= gamma_2 sigma + delta |y_1| + F`), so the current input adds at most `delta^2 g X`. With
(P1):

```text
|y[n]| <= X (W_0(g, 0) + delta^2 g) + W_0(0, F)        for every frame n of every admitted history.
```

So (N1) holds with `g_p >= W_0(g, 0) + delta^2 g` and `sigma_p >= W_0(0, F)`. The module computes
`peak_gain = (W_0(g, 0) + delta^2 g SLACK) SLACK` (the computed `W_0` is an upper bound of the exact
one, #1433; each product and sum is inflated by `SLACK`) and `G_p = ceil_mB(peak_gain)`.

* *`sigma_p`* (root's ruling of 2026-10-08): `sigma_p = ceil_mB(stall(0))`, #1433's flush part with
  the tail at frame `0` (`LiveBound::stall(0)`: the largest of the flush window's `W_j(0, F)` over
  every window frame and of every group's fixed point plus its decaying start from the settled
  phase's first frame, times `SLACK`). By construction `stall(0) >= W_0(0, F)`, so it bounds the
  flush part at every frame. Measured, `stall(0)` is `W_0(0, F)` plus 0.08 % to 0.16 %, at most
  1.4 mB.
* *One frame suffices.* L-D2 asks for the supremum over every admitted history, over the window and
  every settled group. Every frame `n` of (N1) is the frame `N` of the history before it, so `W_0`
  bounds it; the window's later frames and the settled groups bound frames under more hypotheses
  (silence and no control event from `N`), and a frame they bound is a frame `W_0` already bounds.
* *The live loud-input deviation.* The live bound carries the kernel's `f32` deviation inside its
  constants (#1433: `rho` includes `mu_state`, the input column `mu_input`, the output row
  `omega_state`, the feedthrough `omega_input`), so `W_0(g, 0)` includes the deviation of a loud
  input; `delta^2 g` adds the current input's.
* *Not the settled designs' `l1`.* `W_0` contains `sup Psi Phi`: the state a slow pole built while
  the input was loud, which a later retarget exposes. The settled designs' `l1` (3.51 for the
  10 Hz HPF into the top LPF; +34.9 dB with the trim) does not bound it: #1379 H9's `f64` probe
  reaches +67 dB before the trim, and the real kernel under gate L1's history reaches +90.75 dB with
  it.
* *Looseness.* The real-kernel peak under gate L1's history is 53.5 dB below `G_p` (the measured
  ratio per rate is in the #1466 attempt record). The term that sets `G_p` is the second section's
  state bound through `sup Psi Phi`; tightening it is #1485.

### (N2): `sigma_t` and `D`

Under (N2)'s hypotheses with the input zero from `M` (the `epsilon` part is slice B2's), #1433
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
| `g_p` raw | 1.629239e7 | 1.613073e7 | 1.631644e7 | 1.615350e7 |
| `sigma_p` raw (`stall(0)`) | 2.041952e-12 | 2.207801e-12 | 4.142272e-12 | 4.486749e-12 |
| `sigma_t` raw (`stall(T)`) | 3.793538e-15 | 3.772735e-15 | 3.797574e-15 | 3.852977e-15 |
| `G_p` mB | 14,424 | 14,416 | 14,426 | 14,417 |
| `sigma_p` mB | -23,379 | -23,312 | -22,765 | -22,696 |
| `sigma_t` mB | -28,841 | -28,846 | -28,840 | -28,828 |

The millibels are `ceil(2000 log10)` of the raw values; slice B2 states them with the module's
`ceil_mB` (below, "Numerical limits"). The groups, the certificate rates and the cost are in the
#1466 attempt record.

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
the cost (F5). The measurements are in the #1465 attempt record. The live part: #1466's L1 (`G_p`
and `sigma_p` against the real kernel's peak after a Nyquist drive and a retarget), L3 (`D`, `G_p`,
`sigma_p` and `sigma_t` against an independent plain-`f64` recomputation of this part), L4 (`D`
and each certificate's crossing against the module's directly searched crossings for `k = 0..64`,
which must increase strictly in `k`, and `D_inf` against the settled contraction's floor), L5
(nothing certified moves) and the refusal test (a capped or NaN window and a carry outside its
rounding argument give no values); the measurements are in the #1466 attempt
record.

## Citations

As #1329: [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP] (state-space norms), [SMITH-SASP]
(impulse response and state-space analysis); [HIGHAM-ASNA] (N. J. Higham, *Accuracy and Stability
of Numerical Algorithms*, 2nd ed., SIAM 2002, §2.1-2.2) for the rounding model; R. A. Horn and
C. R. Johnson, *Matrix Analysis*, 2nd ed., Cambridge 2013, §8.1 and §8.3 (Perron-Frobenius:
for a non-negative matrix, a positive `v` with `G v <= lambda v` bounds the spectral radius by
`lambda`, and the powers by `lambda^i v`).
