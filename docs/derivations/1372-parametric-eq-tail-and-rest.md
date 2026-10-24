# Issue #1372: the parametric EQ's tail and exact-rest bounds

Part 1 (tranche 1372a, math only) gives the word-hull bound `math::tail::hull`. It is a common
quadratic norm over a convex hull of SVF designs whose damping `k` varies, and a `K`-section
cascade bound on that norm. Part 1 also states why a hull that holds the identity section is out
of scope. Part 2 (tranche 1372b) was planned to give the EQ's own envelope and pinned table. Part 2
is on hold, because the EQ ramps a disabled band to the identity (see "The identity is out of
scope").

The contract and the method are #1329's (`docs/derivations/1329-input-section-tail-and-rest.md`,
"The contract"). This note uses its notation: `s = (ic1, ic2)` is a section's state, `F` is the
per-step flush perturbation, `eps = 10^(-144/20)`, `N` is the first frame of the silence, and
`R = ramp_frames`.

## The kernel and its affine form

One section is the stored-form TPT SVF [SIMPER-SVF] [ZAVALISHIN-TPT]. With the input zero, the
step is `s' = A(w) s`, where:

```text
A(w) = I - 2 N(w),   N(w) = [[c1, a2], [-a2, a3]],   w = (c1, a2, a3)
b(w) = 2 (a2, a3),   y = c . s + d x
```

`A` and `b` are affine in the three recursion words. A designer of the EQ's form gives
`t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g (1 - c1)`, `a3 = g a2`. Then `(c1 - a3) / a2 = k`
exactly. The poles are those of a bilinear-transformed second-order section [ORFANIDIS-ISP] (ch. 6).
Their radius sets the decay time: a pole of radius `r` needs `ln(10^-3) / ln r` samples for
60 dB [SMITH-IDF] ("Pole radius and decay time").

## The norm

Let `||x||_Q = ||L x||_2`, with `L = [[1, l12], [0, l22]]` and `l22 > 0`. This is every quadratic
norm up to scale: `Q = L^T L`, with the Cholesky factor normalized to `l11 = 1`. For a design of
damping `k < 2`, a complex pole is `p = a + i b`. Its eigenvector `x + i y` gives `L ~ [x y]^-1`.
In the words, that is `l12 = (c1 - a3) / (2 a2) = k / 2` and `l22 = sqrt(1 - k^2 / 4)`
(`HullNorm::of_damping`). In that norm, `L A L^-1` is a scaled rotation at any cutoff `g`. So
`||A^n||_Q = |p|^n` holds with no Putzer factor. `k = sqrt(2)` gives `l12 = l22 = 1/sqrt(2)`, which
is #1329's `V`-norm.

The conversion constants are `from_words >= ||L||_2` and `to_words >= ||L^-1||_2`, each a certified
spectral-norm bound, with `kappa >= from_words * to_words`. They replace the `V`-norm's baked-in
`R_NORM`, `R_INV_NORM` and `KAPPA` in the parent module's state and output rounding
(`state_rounding_in`, `output_rounding_in`), in the flush drive `FlushLaw::per_step_in`
(`F = from_words sqrt(2) (FLUSH_EPS + 16 * 2^-126)`), and in the rest limit
`rest_frames(.., norm)` (`REST_EPS / to_words`). The `V`-norm paths pass the same three constants
as before, through `V_CONVERSION`. Their values and their order of operations do not change, so
no pinned table moves.

## The certificate (convexity)

For a fixed `L`, `w -> ||L A(w) L^-1||_2` is a norm of an affine map, so it is convex in `w`. Over
the convex hull of the designed words `{w_j}`, it is largest at some `w_j`. A word within a
per-word box `|e_i| <= m_i` of the hull adds at most `sup_box 2 ||L N(e) L^-1||_2`. That term is
convex in `e`, so it is largest at one of the box's eight corners. So:

```text
q = max_j ||A(w_j)||_Q + sup_box 2 ||N(e)||_Q
```

bounds `||A(w)||_Q` for every word in hull + margin (`HullNorm::contraction`).

### The caller's obligation: every loadable word lies in hull + margin

The bound covers a section only if **every word triple the kernel can load, under its own
per-word ramp and retarget rule, lies in hull + margin**. Nothing in `math::tail::hull` checks
this. The caller derives it for its kernel and tests it in its own crate.

A ramp that moves `(c1, a2, a3)` in lockstep along one line between two held triples stays on a
chord of the hull. A kernel that ramps the words separately (for example, it skips a word whose
target bits are unchanged, so that word holds while the others move) can load a triple that
mixes coordinates of two designs. Such a triple is not a convex combination of the designs and
can lie outside the hull by far more than any rounding box.

The margin for a lockstep `f32` walk with retarget restarts comes from a bounded-excess
induction. Two existing derivations already state it:

* `crates/multiband-compressor/src/tail.rs` (`crossover_envelope`'s "Words"). A 64-step iterated
  ramp `w = fl(w + s)` with `s = fl(t - c) / 64` (exact, since 64 is a power of two) is within
  `j h + (j / 64) |t - c| u` of its chord point after `j` frames (`h` is a design's half-ulp box).
  A retarget restarts from the word in force, and carries that start's distance `E` from the hull
  with weight `1 - j/64`. So `E = 64 h (1 + 3u) + u |t - c|` is a fixed point, and every loadable
  word is within `E + h` per word of the hull of the exact designs.
* `effect_runtime::svf::RAMP_PATH_NORM_TOLERANCE` (#1278). The same induction on the norm: a walk
  that starts at excess `E` stays below `1 + E` whenever `E >= 2^-22 + 65 * 4 * 2^-25`, through
  any chain of retargets.

Both assume a lockstep ramp of at most 64 samples with a power-of-two step. The half-ulp box
`h = (u/2, u/4, u/2)` of an `f32` design also holds only when `c1, a3 < 1` and `a2 < 1/2` (the
multiband's `crossover_envelope` checks this, `crates/multiband-compressor/src/tail.rs:216`); a
caller outside that range states its own `h`. A caller whose kernel
meets those premises can use the per-word box `E + h` as `margin`. The state rounding
(`HullNorm::state_rounding`) increases in each word's magnitude, so it must be evaluated at word
magnitudes that are suprema over hull + box, not at the vertices alone. The scope note
calls this a vertex check; it is the convex-hull form of D2. With `adj(L) = [[l22, -l12], [0, 1]]`:

```text
P = L N L^-1:  P11 = c1 - l12 a2,  P12 = (a2 + l12 a3 - l12 c1 + l12^2 a2) / l22,
               P21 = -l22 a2,      P22 = l12 a2 + a3
```

The largest singular value of `M = I - 2P` is
`(sqrt((a + d)^2 + (c - b)^2) + sqrt((a - d)^2 + (b + c)^2)) / 2`. The code forms it from `P`
directly (`a + d = 2 - 2 (P11 + P22)` and so on). It never forms `1 - 2 P_ii` and does not use the
cancelling textbook form. Each operation is relative to its own result, which gives
`(1 + u)^8 <= 1 + 32 u`. The error of `P` itself is at most `16 u` times the sum of each entry's
term magnitudes, and it adds `2 ||E||_F`.

### Existence and the search

For a pair of 2x2 stable systems, Shorten and Narendra give necessary and sufficient conditions for
a common quadratic Lyapunov function [SHORTEN-NARENDRA]. For a hull, `common_lyapunov` does not
prove existence. It searches `(l12, ln l22)` and certifies what it finds, so the search sets only
tightness, never soundness. The search starts from the identity and from the eigenbasis norm of up
to 64 vertices. A compass search then halves its step from `1/4` to `2^-30` (at most 4,096
evaluations). If the best certified `q` is not below `1`, the result is `NotContracting`.

The energy identity of the TPT trapezoid [ZAVALISHIN-TPT] (ch. 3-4) shows why the identity norm
fails: `||A||_2 = 1` exactly for every design (a zero-input state with `v1 = 0` is reflected). A
cross term `l12 != 0` is what makes the decay strict. Measured on a 25 x 25 log grid of `(g, k)`
over the EQ's domain, plus the extreme bell (no Lipschitz margin; evidence, not a gate), the search
lands near `l12 = k_min / 2 ~ 0.00704`. That is the slowest design's own eigenbasis. The certified
`q` is 0.99999015 at 44.1 kHz and 0.99999095 at 48 kHz. `T` for the extreme bell (10 Hz, Q 18,
+24 dB) is 1.032 times its brute-force `T_b` at both rates. That is far inside scope note F2's
prediction of a factor of 2 to 2.3.

## The cascade bound

`hull_cascade` takes `K` sections in signal order. Each has an `EnvelopeSection` in the hull's
norm: `rho_ramp` is at least `||A||_Q` plus the state rounding over every reachable word, and
`rho_settled <= rho_ramp` over the settled designs. The rest of the envelope is `input`,
`output_state` and `output_input`.

* **Before `N`.** `z_k' <= rho_k z_k + input_k v_k + F`, with `v_1 <= gain (1 + u) P` and
  `v_(k+1) <= output_state_k z_k + output_input_k v_k + F`. So `z_k <= (input_k v_k + F) / (1 - rho_k)`
  is invariant from the zero state, by induction over frames, for every history. A reset only
  lowers it. The invariant splits into a relative part (per unit of `gain P`) and a flush part.
* **After `N`.** The input is zero. The same non-negative lower-triangular system runs on
  `(z_1, ..., z_K, 1)`: the window's contractions for frames `0..=R`, then the settled ones.
* **`T_decay`.** The relative output bound is walked frame by frame. Suppose a settled step does
  not raise any computed component (`fl(S x) <= x`). Then for `j >= 0` the exact bound `j` frames
  later is at most `S^j x`, and `S^j x` does not increase in `j`: `S^(j+1) x = S^j (S x) <= S^j x`,
  with `S >= 0` and the exact `S x` at most the computed one. From there, an exponential search and
  a bisection over powers rounded up (with `+tau` per component against underflow) find a frame
  whose bound is below `eps / 2`. Every later frame is then below it too.
* **Stall and `P*`.** The flush part starts at its invariant `z_F`. A settled step has
  `S <= W` entrywise and the same drive, so it never exceeds `z_F`. Its output `sigma` bounds the
  flush part at every frame. With `P* = sigma / (eps / 2)`, every `P >= P*` is below `P eps` from
  `N + T` on.
* **Rest.** The kernel state at `N` is at most `gain P z_rel + z_F`, capped at the norm of finite
  `f32` words (`from_words sqrt(2) F32_MAX`). Then #1329's section-by-section rest applies, each
  section adding `N_SILENCE`. Per-word magnitudes are read through `to_words`.
* **Composition** (`hull_cascade_composition`; #1379 Amendment 1 H1). `g_p = g_t` is the output
  invariant per unit of the peak, and `sigma_p = sigma_t = sigma`. `D` comes from one decay
  certificate (#1465's `HalfSystem`) on the settled `K x K` system, anchored at the later of `T`
  and `R + 1`.

## The identity is out of scope

The identity section (`w = 0`) has `A = I`, so its norm is `1` in every norm. A chord word
`w = (1 - delta) 0 + delta w'` has exact contraction `1 - delta (1 - q)`. The affine split makes
the exact input-driven invariant uniform. But two non-exact terms do not scale with `delta`:

1. **Final rounding.** The kernel's `n = fl(ic + 2 d)` errs by up to `u |n|` per word (the
   `kappa u q` term of `state_rounding`). The error is `min(u |n|, |2 d|)`, and in the zone
   `|2 d| >> u |n|`. So the kernel contraction is not below `1` for `delta < u / (1 - q)`. At the
   extreme pole's gap (bell 10 Hz, Q 18, +24 dB, `k = 0.0140`), that threshold is at least 0.0060
   at 44.1 kHz, 0.0065 at 48 kHz, 0.0120 at 88.2 kHz and 0.0131 at 96 kHz. One ramp frame toward a
   design moves `delta` by `1/64 = 0.0156`.
2. **Per-word flush.** In a norm with a cross term, zeroing a word below `FLUSH_EPS` can raise
   `||s||_Q^2` by up to `2 |l12| |n_i| |n_j|` (`Q12 = l12`). Charging each flush to the increments since the
   last one needs a constant below `1 - q`. It fails by about a factor of `4 kappa_Q`.

`quantum_frames` only has to be nonzero, and a retarget restarts a ramp from the words in force.
So a history of one frame toward a design, then a ramp back toward the identity, can hold `delta`
in that zone without limit. At 48 kHz with 1-frame blocks, the worst-case rounding gain per cycle
(about `64 u`) exceeds the exact decay (about `0.5 (1 - q)`). A section whose words can ramp toward
the identity has no certified peak-relative bound before `N`. `HullSection::identity_reachable`
states this precondition, and the bound refuses such a section (`TailBoundError::IdentityReachable`).
`identity_reachable` is the caller's attestation: the module cannot decide it from the hull, and
each caller crate must test its own claim against its kernel's enable, kind and retarget rules.
A hull that holds the identity vertex is also refused, because `common_lyapunov` cannot certify
`q < 1` for it.

Two ways out need an owner ruling. One is a class-B kernel snap, for example snapping a ramp word
below a stated weight to the exact identity. The other is a lemma on the ramp law and the minimum
block size. At 2-frame blocks its cycle balance is within about 20%.

Which callers qualify (read at `main` 6172dc4e6 and PR #1919's head 5316681de):

* **De-esser** (`crates/de-esser/src/lib.rs`, `targets`): the SVF words are always the EQ's 0 dB
  high-pass design at the committed frequency and Q. The mode, detector and listen switches
  crossfade the output arms, not the SVF words. It has no enable or kind ramp, so it never reaches
  the identity. But at 6172dc4e6 its retarget skipped a word whose target bits were unchanged
  (`lib.rs:723`), so `(c1, a2, a3)` did not move in lockstep and the hull premise failed. **It
  qualifies only with the joint retarget of all three SVF words**, which root's fix in PR #1922
  makes before the de-esser is registered.
* **Dynamic EQ** (PR #1919, `crates/dynamic-eq/src/lib.rs`, `band_design`): a disabled band's
  design is `[0.0; 9]`, and "a disabled band ramps its six filter words to the parametric EQ's
  identity section". Its words reach the identity, so **it does not qualify**. It stays
  `UNBOUNDED`.
* **Parametric EQ**: a live kind or enable change ramps toward `EqSvfWords::IDENTITY` (#1337 D3).
  It does not qualify. It keeps its declaration until the owner rules. Its ramp-path tolerance
  (`effect_runtime::svf::RAMP_PATH_NORM_TOLERANCE`) makes the same lockstep assumption; #1928
  checks it against the EQ's per-word ramp.

## Gates (part 1)

`crates/math/tests/hull_cascade.rs`:

* a1, the norm (`contraction_bounds_every_hull_word`): 48 seeded random hulls of designs over the
  EQ's `(g, k)` ranges at every launch rate, with random margins. The certified `q` is at least
  the directly evaluated `||A(w)||_Q` (formed as `L A L^-1`, independent of the closed form) at
  every vertex, at random hull points moved within the margin, and along random switching
  trajectories. The conversion constants bound the norm both ways.
* a1, the cascade (`cascade_bound_covers_every_hull_member`): 32 random cascades of one to three
  hull sections, some one-vertex. Every fixed hull member's worst-sign output `S(m + 1)` is below
  the bound's level from each crossing on, for 0 to 3 decades. Random switching with full-scale
  inputs stays below it. An identity-reachable section is refused.
* a2 (`one_vertex_tail_is_tight`): the extreme bell at 48 kHz as a one-vertex hull. `T_b <= T + 1`
  and `T <= 2 T_b`. Measured: `T = 2,262,795`, `T_b = 2,212,870`, ratio 1.0226.
* Composition (`composition_covers_crossings`): `T + crossing(d)` and `T + d D` are at or after
  the directly searched crossing for 1 to 6 decades.
* a3: the builtins' and the multiband compressor's existing `tail_contract` tables, unchanged.

## Sources

- [ZAVALISHIN-TPT] V. Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4: the TPT
  (trapezoidal) integrator and SVF, and the trapezoid's energy behaviour.
- [SIMPER-SVF] A. Simper, "Solving the continuous SVF equations using trapezoidal integration and
  equivalent currents" (Cytomic, 2013): the stored-form state update the kernel runs.
- [ORFANIDIS-ISP] S. J. Orfanidis, *Introduction to Signal Processing*, ch. 6: second-order
  sections, pole radius and the time constant of a resonator.
- [SMITH-IDF] J. O. Smith III, *Introduction to Digital Filters*, "Pole radius and decay time":
  `n60 = ln(10^-3) / ln r`.
- [SHORTEN-NARENDRA] R. N. Shorten and K. S. Narendra, "Necessary and sufficient conditions for
  the existence of a common quadratic Lyapunov function for a finite number of stable second
  order linear time-invariant systems", *Int. J. Adaptive Control and Signal Processing*
  16(10):709-728, 2002: the existence question for 2x2 systems, which the vertex certificate here sidesteps
  by checking a found norm.
