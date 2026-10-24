# Issue #1373: the multiband compressor's tail and exact-rest bounds

Part 1 (tranche 1373a, math only): the output decomposition and the first-stage term, with the
`math::tail` bound that states it. Part 2 (tranche 1373b) adds the effect's envelope over the
crossover domain and its iterated ramp law, the band-gain and smoother terms, the composition of
the two terms into the node's `NodeTailBound`, and the pinned per-rate table.

The contract and the method are #1329's (`docs/derivations/1329-input-section-tail-and-rest.md`,
"The contract" and "#1433: the frequency-aware cascade"); this note uses its notation: `s` the
first stage's state, `E = ||s||_V`, `F` the per-step flush perturbation, `eps = 10^(-144/20)`,
`N` the first frame of the silence, `R = ramp_frames` (64).

## The kernel

Per channel and frame (`multiband_compressor::lr4_step_when`, both stages with `k = sqrt(2)` and
the same words `(c1, a2, a3)`):

```text
(v1, lp1) = svf(x)            first stage, state s = (ic1, ic2)
ap1       = fl(fl(m1 v1) + x) m1 = fl(-2k), the lane's two-rounding fma
(_, lp2)  = svf(lp1)          second stage, state sigma
low = lp2,  high = fl(ap1 - lp2)
y   = fl(fl(low g_lo) + fl(high g_hi))
```

`g_b` is each band's amplitude, `gain_from_db` of its smoothed reduction (at most `0` dB) plus its
makeup (at most `+24` dB). Whatever the detector link and the smoothers do, `|g_b| <= G_max`, with
`G_max` the `f32` amplitude of `+24` dB rounded up (part 2 states it), and the gains are memoryless
multipliers of the band signals.

## Decision (a): the decomposition

`high = ap1 - lp2` mixes a tap of the first stage with the cascade's output, so it is not a row of
either stage alone. Bound it by the triangle inequality:

```text
|y| <= (1 + u)^2 (G_max |lp2| + G_max (1 + u) (|ap1| + |lp2|))
    <= (1 + u)^3 G_max (2 |lp2| + |ap1|)
```

(`u = 2^-24`; one rounding for the subtraction, one per product, one for the sum). Part 2 folds
`(1 + u)^3` into `G_max`. Each term is then a bound `math::tail` already states or now states:

* **`lp2`**: the cascade's output, both stages, the second stage's low-pass row:
  `math::tail::live_cascade` and `live_cascade_composition`.
* **`ap1`**: a row on the first stage's state and input alone:
  `math::tail::live_first_section` and `live_first_section_composition` (below).

The decomposition costs at most a factor `3` (`9.5` dB) over `|lp2| + |high|` and gives up any
cancellation between the bands, which only lengthens the tail by the frames that factor buys at
the slowest pole. It keeps one owner of the zone machinery (`math::tail`) and needs no new
two-stage analysis. Exact rest of `y` follows from exact rest of both terms: when `lp2` and `ap1`
are `+-0`, `high` is `+-0` and `y` is `+-0` for any finite gain.

## The first-stage term

**The row.** The stored-form step gives `v1 = ic1 + d1 = (1 - c1) ic1 - a2 ic2 + a2 x` exactly up
to rounding, so `ap1 = x - 2k v1` is the SVF output with the mix `(m0, m1, m2) = (1, -2k, 0)`.
Every mix of a TPT SVF is `y = c . s + d x` with `c = 1/2 (m1, m2) (I + A)` and
`d = m0 + m1 a2 + m2 a3` (`math::tail`'s kernel note: `v1 = (s1 + s1') / 2`,
`v2 = (s2 + s2') / 2` at zero input), so

```text
|ap1| <= (1/2 ||(m1, 0)||_V* ||I + A(w)||_V + omega) E + delta |x|
```

with `omega` the output rounding on the state (`math::tail::output_rounding`'s `omega_state` for
this mix, which counts more roundings than the two of the fma) and `delta >= |1 + m1 a2| +
omega_input`. For `k = sqrt(2)`, `||(-2k, 0)||_V* = 4` and `0 < 1 - 2k a2 <= 1` over every
Butterworth design (`2k a2 = 2k g / (1 + k g + g^2) <= 2k / (2 + k) < 0.83`). These three suprema
over every word the stage can load are `math::tail::FirstSectionRow`; part 2 computes them over
the crossover domain.

**The state, by zone.** Everything else is #1433's, unchanged: the first stage's words lie in the
zones of `live_zones`, and while they lie in zone `k` the state is at most
`E <= Phi_k G + Phi_k^F F` (`G` the input's scaled peak). `||I + A||_V <= q_k` over the zone. So
before `N` and at every frame of any admitted history

```text
|ap1| <= max_k (1/2 |m| q_k + omega) Phi_k G + delta G + (flush part)        (G_p)
```

**The window `N ..= N + R`.** A ramp may be in flight. The input is zero, so the row is
`(1/2 |m| q_max + omega) E_n + F` (`q_max` the largest zone's; the `+ F` covers the evaluation's
underflow), with `E_N = sup Phi` and `E_(n+1) = rho_ramp E_n + F`, capped at the `V`-norm of finite
`f32` words. A reset or a joint flush only lowers `E`.

**Settled, from `N + R`.** The words are one `f32` design, in one zone group of #1433 (consecutive
zones in one quarter octave of `1 - r`, each term the largest of its members). With `H_0` the
smaller of `E_(N+R)` and the group's `Phi`, and `r` its settled contraction,
`H_(N+R+m) <= r^m H_0 + F / (1 - r)`, and the row is at most `c_y H + F`,
`c_y = 1/2 |m| q^s + omega` (`q^s` the group's settled `||I + A||_V`). Without the flush the bound
`c_y r^m H_0` falls in `m`, so its crossing of a limit is the closed form
`m = floor(ln(limit / (c_y H_0)) / ln r) + 2` (`+2` covers the logarithm's rounding,
`free_decay_frames`). Unlike the cascade's second stage, no input-change term and no one-off
joint-flush term enter: the row reads only the first stage's state.

**Tail, stall, rest.**

* `T_decay`: the first frame from which the relative bound (per unit of the peak, no flush) stays
  below `eps / 2`: the last window frame at or above it, and each group's closed-form crossing.
* `P*`: the flush part from `T_decay` on, over `eps / 2`: the window's outputs from there, and
  per group `c_y (r^m H_0^F + F / (1 - r)) + F` at the first settled frame at or after `T_decay`
  (`r^m` by repeated squaring, each product rounded up, then `tau = 2^-600` added, as `carry`).
  For `P >= P*` the row is below `P eps` from `N + T_decay`.
* Rest at peak `P`: per group, the first stage's state bound falls below `REST_EPS` per word
  (`H < REST_EPS / ||R^-1||`) after `m` frames, then the joint flush arms after `N_SILENCE`, so
  the stage rests by `N + R + m + N_SILENCE`; the largest over the groups. From then on its state
  is exactly zero, the input is zero, and so `ap1 = fl(fl(m1 (+-0)) + 0) = +-0`.
* `T_rest = max(T_decay, R(P*))`: below `P*` the row is exactly zero from there.

**Composition (#1379 Amendment 1 H1).**

* `G_p`: `(G_p)` above, per unit of the peak; `sigma_p` the flush part from `N` (it starts from
  `sup Phi`, which bounds every earlier frame too); `sigma_t` the flush part from `T_decay`, at
  most `sigma_p`.
* `D`: one certificate per group on the one-term system `H' = r H`, row `c_y`, anchored at
  `T_decay` or, when `T_decay` lies in the window, at its end (#1465's `HalfSystem`, with its
  rates, its verification and its rounding).
* `G_t`: a late input from `M >= N` with zero state at `M`: the first stage along every in-flight
  ramp path through the window (#1485's paths), then per settled zone
  `c_k max(E*_k, beta_k / (1 - r_k)) + delta`.

**Hazard check.** The scope note asked whether `math::tail`'s zone constants bound `ap1` within
the window. They do: `q_max` bounds `||I + A(w)||_V` for every reachable word (each zone's
`sum_norm` carries the ramp box), and the window's state recursion is the cascade's own. No
constant was tuned.

## How part 2 composes the two terms

Write each term's bound as `|z_n| <= P r_z(n) + s_z(n)`, relative and flush parts. A `math::tail`
call at input scale `gain = c` scales `r` only: it states `T` with `c r(n) < eps / 2` from `T`,
and `P* = s(T) / (eps / 2)`; `s` is non-increasing in its start frame. With
`c_lp = 4 G_max` and `c_ap = 2 G_max`:

* `T_y = max(T_lp, T_ap)`: from there `G_max (2 r_lp + r_ap) < eps / 2`.
* `P*_y = G_max (2 P*_lp + P*_ap)`: for `P >= P*_y` the flush part
  `G_max (2 s_lp + s_ap) <= P eps / 2`.
* `D_y = max(D_lp, D_ap)` from the same scaled calls: each states `c r_z(n) < (eps / 2) 10^-k`
  from `T_z + k D_z`, so `G_max (2 r_lp + r_ap) < (eps / 2) 10^-k` from `T_y + k D_y`.
* `G_p`, `G_t` are linear in the scale: `G_y = G_max (2 G_lp + G_ap)` at `gain = 1`; the stalls
  add the same way, `sigma_y = G_max (2 sigma_lp + sigma_ap)`.
* `R_y(P) = max(R_lp(P), R_ap(P), R_smoothers)`, the rests at the real input scale.

## Evidence (tranche a)

`crates/math/tests/live_first_section.rs` renders the `f32` first stage and its all-pass tap at
48 kHz over a test envelope for 80 Hz to 8 kHz with a 64-frame iterated ramp (it checks every
loaded word against the envelope's box), under the worst-sign input for the 80 Hz design aimed
at `N + T`, settled and with a glide 8 kHz -> 80 Hz in flight at `N`. On that envelope the bound
is `T_decay` 2,613, `T_rest` 5,673, `R(+24 dBFS)` 9,000, `R(1e29)` 17,646, `P*` 3.2e-10, `D` 311,
`G_p` 8.88, `G_t` 13.07; the kernel's last frame at or above `P eps` is `N + 2,334` (settled) and
`N + 1,546` (glide), its rest `N + 4,716` / `N + 4,096` (+24 dBFS) and `N + 13,344` /
`N + 12,564` (`1e29`), its peak `2.2 P` and its late peak `3.67`. These are the test envelope's
figures, not the effect's (part 2 states those).

## Part 2 (tranche 1373b): the effect's bound

Code: `crates/multiband-compressor/src/tail.rs`; gate: `crates/multiband-compressor/tests/tail_contract.rs`.

**The envelope** (`crossover_live_envelope`). Both stages load the same `(c1, a2, a3)`: an `f32`
design of a crossover in `[80 Hz, 8 kHz]`, or a word of the iterated ramp of #1338
(`s = fl(t - c) / 64`, exact; `w = clamp(fl(w + s))` toward the target; the target assigned on the
64th frame). After `j` frames an unclamped word lies within `j h + (j / 64) |t - c| u` of the chord
point `c + (j / 64)(t - c)`; the endpoint clamp only moves a word toward that point, or holds it
where its step is below its rounding (`|s| <= h`), so the bound holds with `h (1 + 3u)`. A retarget
restarts from the word in force with its distance carried at weight `1 - j / 64`, so
`E = 64 h (1 + 3u) + 2 u w_max` is a fixed point (`|t - c| <= w_max + E`), and every word is within
`E + h` of the hull of the exact designs. On `g < 1` (the whole domain at every launch rate) the
pole radius falls in `g` (80 Hz is the slowest), the words rise in `g` (8 kHz is the largest),
`||b||_V` rises in `g`, and the low-pass row's `||c||_V* = sqrt(2) / sqrt(1 + t)` falls in `g`.
The rest follows the builtin input section's envelope with fixed mix words (no mix ramp): both
stages' output rows are the low-pass mix `(0, 0, 1)`, and the tap's row is part 1's.

**The band gains.** A band amplitude is `fast_gain_from_db(fl(smoothed + makeup))` with the
smoothed reduction in `[-100, 0]` and the makeup word in `[-24, 24]`, so the argument lies in
`[-124, 24]`, inside `math::fast_db`'s exhaustive F1 sweep, whose error is below `1e-5` dB. With
the output's three roundings, `G = 10^((24 + 1e-5) / 20) (1 + u)^3 (1 + 2^-30)` (`output_gain`).

**Composition of the two terms** (as "How part 2 composes" above). `T` from `live_cascade` at gain
`4 G` and `live_first_section` at `2 G`; the rests and the gains at gain `1` (the crossover's input
is the effect's input; the band gains follow it); `P* = (G (2 P*_lp + P*_ap) + 4 UNDERFLOW G /
(eps / 2)) (1 + 2^-30)`, the last term the output chain's absolute underflow; `D` from the scaled
compositions, the gains and stalls from the unit ones (each term's tail stall is from its own `T`,
at most the node's).

**The smoother words.** Once both bands are `+-0` the detector floors at -160 dB, below every
threshold's knee, so the target is `+0.0`; a word `y <= 0` takes the release arm and steps
`flush(fl(fl(c y) + 0))`, the pole form of `effect_runtime::tail::pole_follower_rest_samples`,
from at most `100` dB. The largest retention `c` any release word can hold is found exactly
(`smoother_retention_max`): `x = -1 / fl(fl(t 0.001) fs)` does not decrease with `t`, `expf` is
within one ulp of `exp`, so every `x` below `x_max - 2^-21` gives an `expf` below the slowest
design's, and the `f32` arguments in `[x_max - 2^-21, x_max]` are scanned. The D3 stall check
holds (`1 - c` is about `2e-6` at 96 kHz, far above `2^-24`).

**The statement** (`multiband_live_bound`). `tail = T`; `tail_every_peak = max(T, R_x(P*))`
(below `P*` both taps are exactly zero from `R_x(P*)`, and so is the output, whatever the gains);
`R(P) = max(R_x(P), 64) + R_s`, `R_x` the larger of the two terms' rests, `R_s` the smoother's.
`Z` is the reset state, which #893's `rest_lanes` names: the four crossover words and both
smoother words `+0.0`, every ramp settled, the silence counter a clock. The composition is
`Stated` (step 8 of `graph-compiler/src/pdc.rs` makes a plan's tail `Infinite` on an `Unstated`
one), with the builtin input section's checks `G_t <= G_p` and `sigma_t <= sigma_p`.

**The pinned values** (`multiband_live_bound_table`, held equal to the recompute by T1):

| rate | `T` | `T_rest` | `R(+24 dBFS)` | `R(any)` | `D` | `G_p` mB | `G_t` mB | `sigma_p` mB | `sigma_t` mB |
|---|---|---|---|---|---|---|---|---|---|
| 44.1 kHz | 3,505 | 9,579 | 11,195,358 | 11,203,587 | 288 | 6,834 | 6,259 | -27,515 | -29,780 |
| 48 kHz | 3,810 | 10,434 | 12,155,124 | 12,164,081 | 314 | 6,884 | 6,261 | -27,429 | -29,719 |
| 88.2 kHz | 6,990 | 19,334 | 22,394,952 | 22,411,410 | 578 | 7,341 | 6,279 | -26,815 | -29,177 |
| 96 kHz | 7,600 | 21,063 | 24,301,393 | 24,319,307 | 627 | 7,409 | 6,281 | -26,732 | -29,116 |

The rest is almost all the smoother's (`R_s` 11,183,325 frames at 44.1 kHz, 24,275,194 at 96 kHz,
equal to the brute-force count from -100 dB at the 5,000 ms release; the crossover's `R_x(any)` is
20,262 and 44,113): a dB-domain word flushed at `1e-20` decays about 51 natural logs at the slowest
release. A rest snap would shorten it but moves rendered bits (class B, not adopted). `T` lies far
past the 64-frame window at every rate, so the composition branch with `T` in the window is not
reached by this envelope.

**Evidence (T2, release, every launch rate).** The kernel at 80 Hz, deepest curve, fastest
attack, slowest release, +24 dB makeup, worst-sign input for the band sum's response: the last
frame at or above `P eps` is `N + 1,228` settled and `N + 431` / `N + 467` with the glide in flight
(48 kHz, against `T = 3,810`); `N + 2,462` / `N + 1,561` at 96 kHz (`T = 7,600`). At `N + R` the
state payload equals a fresh instance's, and both then render the same noise to the bit.
Construction cost: about 0.1 to 0.25 s per rate in release (test-only; the descriptor reads the
pin).

## Citations

[SIMPER-SVF] A. Simper, "Linear trapezoidal integrated state variable filter", Cytomic technical
paper, 2013, for the stored-integrator SVF and its mixes. [ZAVALISHIN-TPT] V. Zavalishin, *The Art
of VA Filter Design*, rev. 2.1.0, 2018, ch. 4, for the TPT SVF and its all-pass `x - 2R`-scaled
band-pass mix. [LINKWITZ-1976] S. H. Linkwitz, "Active crossover networks for noncoincident
drivers", *J. Audio Eng. Soc.* 24(1), 1976, for the fourth-order Linkwitz-Riley split as two
cascaded Butterworth sections whose bands sum to an all-pass. [HIGHAM-ASNA] N. J. Higham,
*Accuracy and Stability of Numerical Algorithms*, 2nd ed., SIAM 2002, §2.1-2.2, for the rounding
model; [IEEE-754] IEEE Std 754-2019 for round-to-nearest and underflow. [ORFANIDIS-ISP]
S. J. Orfanidis, *Introduction to Signal Processing*, for state-space norms of IIR sections.
