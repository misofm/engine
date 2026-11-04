# Issue #2091: the parametric EQ as a switched system, and strict decay

#1372 D2 asks for a certified strict-decay bound over every designed SVF step and every 64-frame
ramp step of the parametric EQ, at each launch rate. This note answers the question as a
switched-system problem. It is research only: no kernel, designer or table changes, and no rendered
bit moves. Notation follows the #1372 note (`docs/derivations/1372-parametric-eq-tail-and-rest.md`):
`s = (ic1, ic2)`, `A(w) = I - 2 N(w)`, `N(w) = [[c1, a2], [-a2, a3]]`, `b(w) = 2 (a2, a3)`,
`||x||_Q = ||L x||_2`, `Q = L^T L`, `L = [[1, l12], [0, l22]]`, `u = 2^-24`. The margin of a word
is `m(w) = 1 - ||A(w)||_Q`.

## The answer

1. **Under production's premises, no strict-decay bound exists, in any form.** No norm, no
   piecewise-quadratic or polyhedral Lyapunov function, no joint-spectral-radius bound below 1 and
   no dwell-time bound gives `q < 1`. A disabled band runs the exact identity (`A = I`), and with
   a quantum below 64 frames a production history holds ramp words arbitrarily close to it (§4).
2. **Strict decay is not what the tail needs; `f32` rounding is the obstruction.** In exact
   arithmetic the affine split keeps a peak-relative state bound with no strict decay (§4.2). In
   `f32` the kernel's final rounding has a floor that does not shrink with the design weight
   `omega`. Below a weight `delta_div(fs)` the standard rounding model admits growth, and with any
   quantum below 64 frames a production history reaches such weights and stays there. So no bound
   can be certified in the standard rounding model under production's premises (§5).
3. **Without the identity, a common quadratic norm exists at every rate** (§3), with an exact
   extremal argument instead of a grid. This corrects two earlier statements: the #2091 spec ("none
   exists at 44.1 and 48 kHz") and the #1372 note's evidence, which missed the shelves.
4. **Recommendation for #1372: a contract change for the owner, class B.** On a ramp toward the
   identity, hold the three recursion words at design weight `delta_s = 1/16` while the three mix
   words finish today's 64-frame ramp (§6). Every executed non-identity step then has a certified
   contraction below 1 at every rate. P5's timing does not change. The hold makes no step larger
   than today's own ramp increments (its largest per-frame change is at most 1.064 times
   today's), and during a disable its output differs from today's by at most 0.036x the input
   amplitude, on frames 61 to 63 of a disable from a design. §6.5 states the full bit-move scope.
   Attempt 1's end snap is rejected (§6.2).

| Rate | `q_D` (designs, §3) | `delta_div` (§5) | `delta*_cert` in `L*` (§5) | `q_s`, hold at 1/16, no walk loss | `q_s`, walk loss 0.6 |
|---|---|---|---|---|---|
| 44.1 kHz | 0.9999934300 | 0.00544 | 0.00916 | 1 - 3.50e-7 | 1 - 1.04e-7 |
| 48 kHz | 0.9999939638 | 0.00597 | 0.00997 | 1 - 3.17e-7 | 1 - 9.05e-8 |
| 88.2 kHz | 0.9999967150 | 0.01142 | 0.01832 | 1 - 1.45e-7 | 1 - 2.18e-8 |
| 96 kHz | 0.9999969819 | 0.01247 | 0.01994 | 1 - 1.28e-7 | 1 - 1.52e-8 |

The domain extreme of `q_D` and `q_s` is a tie of two designs at 10 Hz: the bell at +24 dB and
Q 18, and the low shelf at +24 dB and slope 0.1 (the high shelf at -24 dB and slope 0.1 has the
same words). The witness for `delta_div` is the bell at +24 dB, Q 18, 10 Hz, whose pole is the
nearest to `z = 1`.

## 1. The system and production's premises

One section of one lane, input zero, is `s' = A(w) s`. The kernel is `lane::kernels::svf_step`
(`crates/lane/src/kernels.rs:1111`): `d1 = fma(-c1, ic1, a2 v3)`, `d2 = fma(a3, v3, a2 ic1)`, and
the state is `n = fl(ic + fl(d + d))`, then the flushes. Every design of every kind has the words
`c1 = t / (1 + t)`, `a2 = g / (1 + t)`, `a3 = g^2 / (1 + t)` with `t = g (g + k)`
(`design_svf_words_f64`, `crates/parametric-eq/src/lib.rs:845`). With `w = tan(pi f / fs)` and
`A = 10^(gain / 40)`:

| Kind | `g` | `k` |
|---|---|---|
| low pass, high pass, notch | `w` | `1 / Q` |
| bell | `w` | `1 / (Q A)` |
| low shelf | `w / sqrt(A)` | `k_s = sqrt((A + 1/A)(1/S - 1) + 2)` |
| high shelf | `w sqrt(A)` | `k_s` |

The domain is 10 Hz to 20 kHz, -24 to +24 dB, Q 0.1 to 18 and slope 0.1 to 1 (`MINIMA`/`MAXIMA`,
`lib.rs:544`). So `k` spans `k_lo = 1/(18 A_max) = 0.013955` to `k_hi = A_max / 0.1 = 39.81`.

Production's premises, read at `main` d4956341c:

- **P1, identity.** A disabled band's words are `EqSvfWords::IDENTITY` (`BandTarget::words`,
  `lib.rs:1008`). Then `A = I` and `b = 0` exactly, the output is the input, and the state holds.
- **P2, ramp law.** A retarget starts a 64-frame lockstep ramp from the words in force, with step
  `fl(t - w) 2^-6` (`start_ramp`, `lib.rs:1565`). Each frame uses the words in force and then adds
  the step, with no clamp (`svf_block_ramped_form`, `kernels.rs:1250`). The exact target applies
  from frame 64 (`process_section`, `lib.rs:1780`). So the executed words of a ramp from `w0` are
  the `f32` walk `w0 + j s` for `j = 0..63`.
- **P3, retarget timing.** Targets apply at block boundaries (`crates/rack/src/lib.rs:3003`,
  `crates/graph/src/runtime.rs:6331`). The quantum `B` only has to be nonzero
  (`crates/session/src/validate.rs:58`).
- **P4, restart.** Every touched row of a section makes a target (`prepared_sections` and
  `section_touched`, `crates/parametric-eq/src/control.rs:198`, `:226`), and a disabled band's
  target words are the identity. So automation of a disabled band's frequency restarts its
  disable ramp at every block. Only a target equal to the words in force is hoisted.
- **P5, re-enable from rest.** A target that is not the identity, sent to a lane that is settled at
  the exact identity, first sets the state to `+0.0` (`apply_prepared_target`, `lib.rs:1606`,
  #1337 D4).
- **P6, rounding.** Round to nearest in `f32`, with the per-word flush and the joint flush (#1328).

**Design weight.** Every executable word is, up to the `f32` walk excess, `omega v` with `v` in the
convex hull of the designs and `omega` in `[0, 1]`. A design and every chord between designs have
`omega = 1`; the identity has `omega = 0`. A ramp toward the identity from weight `omega0` executes
`omega0 (1 - j/64)`; a ramp toward a design executes `omega0 + (1 - omega0) j/64`.

## 2. The candidate methods

For a linear system that switches among the matrices of a set `S`, the joint spectral radius
`rho(S)` is the exact worst-case rate. Every norm has `max_S ||A|| >= rho(S)`, and for every
`e > 0` some norm has `max_S ||A|| <= rho(S) + e` [ROTA-STRANG], [JUNGERS] (ch. 1-2). A common
quadratic norm [SHORTEN-NARENDRA], a piecewise-quadratic function [JOHANSSON-RANTZER] and a
polyhedral function [BLANCHINI] are ways to find such a norm. Polyhedral functions are universal:
a linear difference inclusion is stable under arbitrary switching if and only if a polyhedral
Lyapunov function exists [MOLCHANOV-PYATNITSKIY], [BLANCHINI-MIANI]. Dwell-time results relax
arbitrary switching, but each needs every mode to be stable on its own: Schur stable in discrete
time [MORSE], [HESPANHA-MORSE], [GEROMEL-COLANERI]. Surveys: [LIBERZON], [LIN-ANTSAKLIS],
[SHORTEN-ET-AL].

So the question splits into three parts: the designs and their chords (§3), the identity mode
and the chords to it (§4), and the `f32` rounding (§5).

## 3. Designs only: a common quadratic norm exists at every rate

### 3.1 The bilinear map keeps a Lyapunov inequality

With `M_k = [[-k, -1], [1, 0]]` (the analog SVF), the step is the bilinear map
`A = (I - g M_k)^-1 (I + g M_k)` (checked against the words to 1e-12). With `B = I - g M_k`,
`C = I + g M_k` and `R(k) = -(M_k^T Q + Q M_k)`:

```text
C^T Q C - B^T Q B = 2 g (M_k^T Q + Q M_k)   so   A^T Q A - Q = -2 g B^-T R(k) B^-1.
```

So `R(k) > 0` gives `||A||_Q < 1` for every `g > 0`. The rotation per step, which passes 90
degrees for a pole above `fs / 4`, has no part in it. [LAROCHE] uses the same transition-matrix
view for recursive filters whose coefficients vary. `R(k)` is affine in `k`, so `R > 0` at `k_lo`
and `k_hi` gives it on the whole interval. For `Q = [[1, e], [e, 1]]`, `R(k) > 0` holds exactly
when `0 < e < k` and `e (4 + k^2) < 4 k`. So every `e` below `min(k_lo, 4 k_hi / (4 + k_hi^2)) =
0.013955` gives a common quadratic norm at every rate. The 2-norm (`e = 0`) has
`R(k) = diag(2k, 0)`, which is singular: that is why every design has `||A||_2 = 1`.

### 3.2 The norm `L*`

`L* = [[1, 0.0093375], [0, 0.9998114]]` maximises the smallest margin over the eight corners of
§3.3 (a compass search; the optimum is the same at all four rates to seven digits). Its constants
are `from_words = 1.004586`, `to_words = 1.004776`, `kappa = 1.009384`. The smallest eigenvalue of
`R(k)` is 9.2e-3 at `k_lo` and 1.7e-2 at `k_hi`.

### 3.3 The extreme is a corner

- **Lemma A1 (fixed `g`).** With `tau = 1 / (1 + g^2 + g k)`, the words are
  `(1 - tau, g tau, g^2 tau)`: a straight segment in word space, monotone in `k`. `||A||_Q` is
  convex in the words, so its maximum over `k` in `[k_a, k_b]` is at an end.
- **Lemma A2 (fixed `k`, `R(k) > 0`).** From §3.1, `1 - ||A||_Q^2 = 2 g / h(g)` with
  `h(g) = lambda_max(R^-1/2 (Q + g R + g^2 M^T Q M) R^-1/2)` (checked numerically). `h` is convex
  and positive, so `{g : 2 g >= c h(g)}` is an interval. Thus `1 - ||A||_Q^2` is quasi-concave in
  `g`, and its minimum over `g` in `[g_a, g_b]` is at an end.

Together: on a rectangle `[g_a, g_b] x [k_a, k_b]` with `R > 0`, `||A||_Q` is largest at a corner.
Two rectangles hold every design: bell, pass and notch, `g` in `[w(10), w(20000)]` and `k` in
`[k_lo, k_hi]`; and both shelves, `g` in `[w(10) / sqrt(A_max), w(20000) sqrt(A_max)]` and `k` in
`[sqrt(2), k_s(A_max, 0.1)] = [1.4142, 6.3317]`.

### 3.4 Result

`q_D` is the largest corner value in `L*`, plus the `f32` word box (each design word is within
`u |w|` of the exact one; the box term is taken per cell of a 300 x 300 grid of each rectangle,
with monotone bounds on the word magnitudes), plus `1e-13` for `f64` evaluation (the measured error
against 60-digit `mpmath` is 1.4e-15). The box term adds at most 3e-10 at the extreme. `q_D` bounds
`||A||_Q` on every `f32` design and, by convexity, on every exact chord between designs. The values
are in the first table. At 10 Hz the two binding corners agree to 6e-12. The tie is the optimum
of the norm search: a larger `l12` helps the shelf corner and hurts the bell corner. The shelf
corner competes at all because its `g` is `w / sqrt(A)`, half the bell's.

**Corrections.**

- The #2091 spec states that no common quadratic Lyapunov function exists at 44.1 and 48 kHz. One
  exists at every rate, with a margin of 6.6e-6 and 6.0e-6 there.
- The #1372 note reports `q = 0.99999015` at 44.1 kHz with `l12 ~ k_min / 2`. That value is the
  bell's corner. The shelves' `g = w / sqrt(A)` is lower than the bell's, and in that norm the
  10 Hz shelf gives 0.99999507 (44.1 kHz), 0.99999547 (48), 0.99999754 (88.2) and 0.99999774 (96).
- The cost of one norm: `L*`'s margin at the bell is 0.66 of the bell's own pole gap (6.57e-6
  against 9.94e-6 at 44.1 kHz). No norm can do better than that gap. So a polyhedral norm could
  gain at most a factor 1.51 in rate for the designs; that search is not pursued here.

### 3.5 Numeric check

At each rate, with its own seed (`2091 fs`): the #1372 gate-2 grid (6 kinds x 100 log-spaced
frequencies x 20 log-spaced Q x 13 gains x 5 slopes = 780,000 `f32` designs), 100,000 random
designs, and the `f32` walk of P2 between 100,000 random pairs at every stepping frame, 1 to 63.
All are at or below `q_D`:

| Rate | grid max | random max | walk max |
|---|---|---|---|
| 44.1 kHz | 0.9999934297 | 0.9999932189 | 0.9999925122 |
| 48 kHz | 0.9999939636 | 0.9999938075 | 0.9999933962 |
| 88.2 kHz | 0.9999967149 | 0.9999966311 | 0.9999964534 |
| 96 kHz | 0.9999969818 | 0.9999967831 | 0.9999966621 |

**Lipschitz margin.** None is needed for the exact words: Lemmas A1 and A2 make the corner value
exact for every off-grid design, so the grid is a regression check, not the proof. The only margin
is the `f32` word box above. A dense 2,000 x 2,000 interior scan of each rectangle found no margin
below its corner minimum.

## 4. The identity: no strict-decay bound exists

### 4.1 The proof

Let `S(fs)` be the set of step matrices the kernel can execute at rate `fs`. By P1, `I` is in `S`.
By P2 to P4, for every quantum `B < 64`, a disable ramp restarted at every block executes the
weights `(1 - B/64)^n omega0`, which tend to 0.

**Theorem B1.** For every `V >= 0` and every `q < 1` with `V(A s) <= q V(s)` for all `A` in `S`,
`V = 0`. The joint spectral radius of `S` is 1. No dwell-time result gives a decay bound.

*Proof.* `A = I` gives `V(s) <= q V(s)`. `rho(I) = 1` gives `rho(S) >= 1`, and in `L*` every
executable chord has `||A(omega v)|| <= (1 - omega) + omega q_D <= 1`, so `rho(S) = 1`. Every
dwell-time theorem in §2 needs each mode Schur stable, and `I` is not. Without the mode `I` the
result is the same for `B < 64`: `rho(A(omega v)) >= 1 - 2 omega`, which tends to 1 on the
restarted chords. Polyhedral functions are universal (§2), so no other function class can help.

### 4.2 Exact arithmetic does not need strict decay

For `w = omega v`: `A(w) = (1 - omega) I + omega A(v)` and `b(w) = omega b(v)`. In `L*`,
`||s'|| <= (1 - omega m(v)) ||s|| + omega beta |x|`. So `||s||_Q <= beta P / m_D` is invariant
under every executable step, the identity included, because the identity holds the state and adds
nothing. Strict decay is sufficient, not necessary. What the tail needs is a contraction weighted
like the drive: both scale with `omega`.

## 5. `f32`: no bound can be certified under production's premises

### 5.1 The floor

The standard rounding model [HIGHAM] (ch. 2) is `fl(a + b) = (a + b)(1 + e)`, `|e| <= u`. Applied
to the kernel's last operation `n = fl(ic + 2d)`, it contains the map `s -> (1 + u) A(w) s` (the
errors of `d` scale with `omega` and only add). So any certificate built on it has a per-step
constant of at least `(1 + u) rho(A(w))`, in every norm. On the witness ray (bell +24 dB, Q 18,
10 Hz, pole `p`), `A(omega v)` has the eigenvalues `1 - omega (1 - p)`, and
`(1 + u) |1 - omega (1 - p)| > 1` for every `omega < delta_div(fs)`. `delta_div` is in the first
table (solved in 60-digit `mpmath`; with the `f32` witness words the sign changes between
`0.99 delta_div` and `1.01 delta_div`). In `L*` itself the certification fails below
`delta*_cert = kappa u / (m_D (1 - C/m)) `, where `C/m <= 5.7e-4` is the weight-scaled part of
`math::tail::hull`'s state rounding over the margin.

### 5.2 Production reaches the zone and stays there

**Theorem B2.** For every quantum `B < 64` there is a history after which every executed word has
`omega < delta_div` for all later time, and along which the standard model admits unbounded
growth. So no peak-relative state bound can be certified in that model.

*Witness history.* Settle the band at the witness design, disable it, then touch its frequency at
every block (P4). Then `omega_n = (1 - B/64)^n`. It falls below `delta_div` after 332, 326, 285
and 279 frames for `B = 1` (44.1, 48, 88.2, 96 kHz) and after 126 frames for `B = 63`; the `f32`
walk follows the law (for example `omega = 5.2e-3` after 334 frames at 44.1 kHz with `B = 1`, and
6e-8 after 4 blocks with `B = 63`). The chord matrices commute (each is a polynomial in `A(v)`) and
are scaled rotations in `v`'s eigenbasis norm, so the model's product grows at least like
`prod (1 + u) |1 - omega_n (1 - p)|`, which diverges because `omega_n -> 0`. The walk excess
changes each factor by `O(omega_n)` times the excess, which grows at most linearly in `n`, so its
sum is finite and the product still diverges.

The theorem is about certificates. It does not claim that the real kernel grows: deep in the zone
the kernel freezes (`|2d|` below half an ulp leaves `n = ic`). A bound that used this would rest on
the exact rounding pattern, which is option (c) of root's ruling of 2026-10-24.

### 5.3 A dwell premise of 64 frames

If every quantum were at least 64 frames (Web Audio's 128 is), no ramp could restart in flight, and
every executed weight would be at least `1/64`, which is above `delta_div` at every rate. In `L*`
the certified per-step constant at `omega = 1/64` is `1 - 4.2e-8` (44.1 kHz), `1 - 3.4e-8` (48),
`1 + 8.9e-9` (88.2) and `1 + 1.3e-8` (96). It closes only at 44.1 and 48 kHz, with no room for the
walk loss of §7.1. Production does not guarantee it: a C-ABI host chooses its quantum.

### 5.4 A componentwise rounding model does not help

The bound `|n_i - (ic_i + 2 d_i)| <= min(u |n_i|, |2 d_i|)` does scale with `omega`. But it lets
each word's increment be `r_i 2 d_i` with `r_i` in `[0, 2]` independently, including `(1, 0)` and
`(0, 1)`. Long runs of those two modes converge to two projections whose product has the
eigenvalue `-1/t`, `t = g (g + k)`. At the witness, `1/t` is 9.6e4 to 2.1e5 (checked with
`2^24`-step matrix powers). That model diverges even at `omega = 1`, so it cannot certify the
designs either.

### 5.5 Conclusion

Under production's premises no strict-decay bound exists (B1), and for every quantum below 64
frames no peak-relative bound can be certified in the standard rounding model (B2). The #1372
note's option (c), a lemma on the ramp law and the minimum block size, fails for every `B < 64`
through the restart path of P4, and for `B >= 64` it closes only at 44.1 and 48 kHz.

## 6. Recommendation for #1372: hold the recursion words at weight 1/16 (class B)

### 6.1 Two rules

The certificate needs one thing: the three recursion words `(c1, a2, a3)`, which alone set `A`
and `b`, must not run below weight `delta_s = 1/16` while the state is live. The three mix words
`(m0, m1, m2)` only scale the output. Two rules give that:

- **End snap (attempt 1; rejected, §6.2).** All six words take the identity when the weight would
  fall below `delta_s`, and the ramp ends early.
- **Hold (recommended, §6.3).** The recursion words stop at weight `delta_s`; the mix words finish
  today's 64-frame ramp; the ramp ends at frame 64, as today.

Both keep the per-step certificate of the first table (`q_s`), so the choice rests on what each
does to the output. §6.6 has the measurements; they are `f64` simulations of the section (P2's
ramp, P5's reset) with a unit complex exponential input, so every figure is relative to the input
amplitude. The domain is 6 kinds, 26 frequencies from 10 Hz to 20 kHz, gains -24 to +24 dB, Q 0.1
to 18 and slopes 0.1 to 1, each with 49 signal frequencies including its own frequency: 99,372
lanes per rate, at all four rates (Appendix C).

### 6.2 Why the end snap is rejected

1. **It changes P5's timing.** After the snap, frames 61 to 63 of a disable have `remaining = 0`
   and identity words. So a target toward a design that lands there zeroes the state
   (`lib.rs:1613`); today it does not. With a quantum below 64 that can happen, and then all later
   output of the band changes: by up to 18x the input amplitude (a high pass at 10 Hz, Q 18, at
   its resonance), and still by 16.5x to 17.2x after 3,000 frames.
2. **It makes a step.** Its output differs from today's by up to 1.08x the input amplitude in one
   frame (a +24 dB high shelf at 0.3 to 0.7 kHz), about 3 times the largest per-frame change that
   today's ramp makes on the same lane. Its largest gain change at frames 61 to 64 exceeds today's
   largest per-frame gain change by up to 9.4 dB.

### 6.3 The hold rule

Each section of each lane carries its recursion weight `omega` (one word, with the ramp law of §1).

1. On a ramp toward the identity, the mix words step for 64 frames, as today. The recursion words
   step as today while their weight stays at or above `delta_s`, and then hold: a ramp from weight
   `omega0` gives them `floor(64 (1 - delta_s / omega0))` steps. At frame 64 all six words take the
   identity, as today.
2. A retarget toward the identity while the recursion words hold keeps them held (zero steps);
   only the mix words restart.
3. If a ramp toward the identity starts while `omega < delta_s`, the recursion words keep their
   ramp toward the design until `omega` reaches `delta_s`, and then hold. This happens only within
   3 frames after an enable from rest.
4. Everything else is today's: ramps toward a design, the ramp end, the stationary hoist and P5.

For a disable from a settled design, frames 0 to 60 keep today's bits. On frames 61 to 63 the
recursion words hold at weight 4/64 (today 3/64, 2/64 and 1/64) and the mix words are today's. From
frame 64 the section is the identity, as today. The same rule applies to the HPF and LPF sections.

### 6.4 Why the hold keeps the certificate

1. **The exact identity** occurs only after a ramp ends, as today: the state holds and nothing is
   added. At frame 64 the recursion words jump from weight `delta_s` to 0, but the mix words are
   exactly the identity's on that frame, so the output is `y = x`, as today.
2. **`omega >= delta_s`:** the certified constant is at most `1 - omega m(v)(1 - C/m - phi) +
   kappa u`, where `phi` is the walk loss of §7.1. At `omega = delta_s` this is `q_s` in the first
   table, below 1 at every rate for `phi` up to 0.85, 0.84, 0.71 and 0.68 (44.1, 48, 88.2,
   96 kHz). The per-word flush adds an absolute drive `F` per step, which a uniform constant below
   1 absorbs; that was the #1372 note's second gap.
3. **`0 < omega < delta_s`** occurs only on a ramp toward a design that starts at the settled
   identity, where P5 has set the state to `+0.0`. Rule 3 keeps the weight rising, by at least
   `(15/16) / 64` per frame, so there are at most 4 such frames, from the zero state.

Restarts toward the identity cannot lower the recursion weight below `delta_s` (rule 2), and
restarts toward a design raise it. The recursion words are never the identity before the ramp
ends, so P5 sees the same `remaining` and the same identity test as today at every block boundary:
**P5's timing is unchanged.**

The weighted invariant of §4.2 then holds in `f32`:
`Z = (delta_s beta P + F) / (delta_s mu - kappa u)`, with `mu = m_D (1 - C/m - phi)`. Against the
same bound without the identity, the factor and the cost in `T` (at the extreme's rate in `L*`)
are:

| Rate | `phi = 0` | `phi = 0.6` |
|---|---|---|
| 44.1 kHz | x1.16, `T` + 0.52 s | x1.54, `T` + 1.50 s |
| 48 kHz | x1.18, `T` + 0.57 s | x1.62, `T` + 1.67 s |
| 88.2 kHz | x1.39, `T` + 1.13 s | x3.58, `T` + 4.40 s |
| 96 kHz | x1.44, `T` + 1.26 s | x4.72, `T` + 5.35 s |

For scale, `T_b` at the extreme is about 46 s (the #1372 note's a2, 2,212,870 frames at 48 kHz),
so `T` in `L*` is about 70 s.

### 6.5 The exact bit-move scope

Output bits move on these histories, and on no others:

1. **Held frames.** On a ramp toward the identity, every frame on which today's recursion weight is
   below 1/16 while the ramp is in flight: frames 61 to 63 of a disable from a settled design, and
   the matching frames of a disable that starts at a partial weight or is restarted. On these frames
   the output differs from today's by at most 0.036x the input amplitude for a disable from a
   settled design, and by at most 0.19x during a disable restarted at every frame for 400 frames.
2. **A disable within 3 frames of an enable from rest** (possible only with a quantum of 3 frames or
   less): the recursion words rise to 1/16 instead of falling. The output differs from today's
   during that ramp by at most 0.155x the input amplitude.
3. **A ramp toward a design that arrives while case 1 or 2 is in flight** (possible only with a
   quantum below 64): its recursion words start from weight 1/16 instead of today's lower weight.
   All later output of that band then differs, and the difference decays at the band's own rate
   until both states are cleared (P5 after a completed disable, the joint flush at rest, or a
   reset). After a normal disable it is at most 2.80x the input amplitude (a +24 dB, Q 18 bell at
   its resonance, whose own output is 15.8x the input), and on every lane it is no more than the
   difference today's engine makes when the same re-enable lands one to three frames earlier.
   After a disable that was restarted for a long time (a disabled band whose frequency is automated
   every block), it reaches about 13x the input amplitude for the same kind of bell, near the
   band's own output level: today's recursion weight has decayed toward 0 and the state has frozen,
   while the hold has kept the state decaying and following the input at weight 1/16.

Nothing else moves: designs, ramps between designs, enables, settled sections and every frame on
which today's recursion weight is at least 1/16 keep today's bits. In cases 1 and 2 the held state
of the disabled band also differs from today's, which the state payload shows.

### 6.6 The worst-case step

"Added step" is the largest per-frame change of `y_rule - y_today`, over the domain of §6.1.

| Rate | end snap: added step | hold, disable: added step, max difference | hold, disable: excess over today's largest per-frame dB change | hold, enable then disable in 1 to 3 frames: added step, max difference |
|---|---|---|---|---|
| 44.1 kHz | 1.068 | 0.036, 0.036 | 0.052 dB | 0.022, 0.155 |
| 48 kHz | 1.067 | 0.035, 0.035 | 0.054 dB | 0.021, 0.154 |
| 88.2 kHz | 1.076 | 0.030, 0.030 | 0.048 dB | 0.018, 0.151 |
| 96 kHz | 1.077 | 0.029, 0.029 | 0.048 dB | 0.017, 0.150 |

The hold's worst disable case is a +24 dB high shelf at 20 kHz. Its largest per-frame change is at
most 1.064 times the largest per-frame change today's ramp makes on the same lane. Measured against
today's own sensitivity to control timing, on every one of the 99,372 lanes:

- disable (all four rates): the hold's difference is no larger than today's difference between a
  disable at frame 0 and one at frame 1;
- enable then disable (all four rates): at most 0.46 times today's difference when the disable
  lands one to three frames later;
- re-enable in the window (44.1 and 96 kHz): at most 0.998 times today's difference when the
  re-enable lands earlier (at frame 60 instead of 61, or at frames 60 to 62 instead of 63).

### 6.7 Does the ruling need a listening check?

For a disable from a settled design, an enable then disable within 3 frames, and a re-enable
during such a disable, no: the choice can be made on analysis. There the hold changes the output by
no more than today's engine already changes it when the same edit lands a frame or a few frames
earlier or later, and its largest per-frame change is at most 1.064 times today's own largest ramp
step. That is an objective, relative bound. It is not a claim of inaudibility in absolute terms, and
no agent can run a listening test.

Restarted disables are not covered by that bound: the hold's output differs by up to 0.19x the
input before a re-enable and by about 13x after it. That is a behaviour choice, not a step: today
revives a state frozen at an earlier moment, and the hold revives a state that kept decaying and
following the input. Neither output has a discontinuity, and there is no reference a listening
test could compare them against, so a listening check would not decide it either. The owner rules
on the behaviour. The restart path is a finding of its own (§7.4).

The end snap would need a listening check before it could be accepted; this note rejects it on its
step and its P5 change instead.

### 6.8 What #1372 must carry

- **(W)** The walk loss `phi` is at most 0.6, proved by a relative bounded-excess lemma (§7.1). If
  the lemma gives more than 0.68 at 96 kHz, `delta_s = 1/8` closes up to 0.84; it moves 7 frames
  of each disable instead of 3.
- **Kernel shape.** One weight word per section and lane, and a recursion step count. The ramp
  segment is cut where the recursion words stop, as it is cut today where a ramp ends. The state
  payload must carry both, or a restored ramp toward the identity must start from a conservative
  weight.
- **A variant without a weight word.** The hold can use the margin of the words in force instead:
  at a retarget toward the identity, compute `mu0 = m(w0) - C(w0)` in `L*` and stop the recursion
  words at the last frame with `(1 - j/64) mu0 (1 - phi) > kappa u` plus a margin. Bits then move
  only for disables of designs with a small margin (low frequencies, mostly at 88.2 and 96 kHz) and
  for restarted disables. It costs one 2 x 2 norm per such retarget at the block boundary, which is
  bounded arithmetic with no allocation. The weight rule is the simpler one; both need the owner.
- The same rule would let the dynamic EQ (PR #1919) qualify; its disabled bands ramp to the
  identity too.

## 7. Further findings for #1372

### 7.1 The walk box must be relative

The #1372 note's caller obligation cites the absolute walk box of the multiband compressor,
`E = 64 h (1 + 3u) + u |t - c|` with `h = (u/2, u/4, u/2)`
(`crates/multiband-compressor/src/tail.rs:183`), which serves crossovers from 80 Hz to 8 kHz. For
the EQ that box adds 6.11e-6 to `||A||_{L*}`: 0.93 of `m_D` at 44.1 kHz, and 1.01, 1.86 and 2.02
of `m_D` at 48, 88.2 and 96 kHz. So it cannot certify the EQ's designs at 48 to 96 kHz, even with
no identity. Each `f32` add errs by at most `u` times its result, so the box must be relative to
the word magnitudes along the walk. Bounds for one ramp, per cell and conservative: a ramp between
far designs, at its last stepping frame (where the bound is largest), loses at most 0.20, 0.17,
0.22 and 0.23 of the chord's margin (44.1 to 96 kHz); a ramp toward the identity under the hold
of §6.3 loses at most 0.16. Measured over simulated `f32` histories (Appendix B: 256 lanes,
2,000 frames, ten schedules per rate, quanta 1 to 64, random design and identity targets, 30%
domain corners), the loss is at most 1.2e-3 of the chord's margin. #1372 must prove the lemma;
premise W states the value it needs.

### 7.2 The tail in `L*` is about 1.5 `T_b`

After the ramp ends, a settled section is time-invariant. In `L*` alone its rate at the extreme is
0.66 of the bell's own gap, so `T / T_b` is about 1.51: inside gate 1's factor 2, with less room.
Changing to the settled design's own eigenbasis norm after the ramp ends (one conversion factor)
brings `T` back near `T_b`.

### 7.3 Exact rest of a disabled band

A disabled section keeps its held state until P5 clears it, so D3a rest is never reached while a
disabled band holds a state of `REST_EPS` or more. Setting the state to `+0.0` when the words
become the exact identity changes no output value (the identity's mix words do not read the state,
and P5 clears it on departure anyway). It can change the sign of a zero output for a `-0.0` input:
an executed identity section adds `0 * v1` and `0 * v2`, whose signs follow the state, and an inert
state lets the section be elided. It also changes the payload and the rest and elision predicates.
So it is a class-B question too. #1372 has to decide it for its rest bound; it is outside the decay
question.

### 7.4 A disabled band's parameter change restarts its disable ramp

By P4, every touched row of a disabled band restarts its ramp toward the identity. Under automation
with a quantum below 64, the disable never completes: the recursion weight decays toward 0, the
state freezes, P5 never fires, and a later re-enable revives the frozen state. That is today's
behaviour, with or without the hold. A target whose words equal the target already in flight could
keep the ramp instead (a target-side stationary hoist); that is a separate class-B question. It
would remove Theorem B2's witness, but not the conclusion: the enable-disable cycle of the #1372
note (one frame toward a design, 63 back) still grows in the standard model at 88.2 and 96 kHz
(model product per 64-frame cycle `1 + 1.1e-6` and `1 + 1.4e-6` at the witness; it contracts at
44.1 and 48 kHz).

## 8. Gate record

- Every claimed bound was checked at 44.1, 48, 88.2 and 96 kHz over the whole domain: `q_D` by the
  corner theorem plus the cell-wise word box (300 x 300 cells per rectangle), cross-checked by the
  780,000-point grid, 100,000 random designs and 100,000 random `f32` walks (§3.5); `delta_div` and
  `delta*_cert` at the witness and at `m_D`; `q_s` at `omega = delta_s` from `m_D`, `C/m` and
  `kappa u`. The grid and the margin are stated in §3.4 and §3.5.
- The ramp-rule comparison of §6 was run at all four rates over 99,372 lanes per rate (Appendix C).
- No rendered bit moves; no code is committed. The scripts are Appendices A to C, as text only.

## Sources

- [ROTA-STRANG] G.-C. Rota and G. Strang, "A note on the joint spectral radius", *Indagationes
  Mathematicae* 22:379-381, 1960: the joint spectral radius as the infimum over norms of the
  largest matrix norm.
- [JUNGERS] R. M. Jungers, *The Joint Spectral Radius: Theory and Applications*, LNCIS 385,
  Springer, 2009, ch. 1-2: properties of the joint spectral radius and extremal norms.
- [MOLCHANOV-PYATNITSKIY] A. P. Molchanov and Ye. S. Pyatnitskiy, "Criteria of asymptotic stability
  of differential and difference inclusions encountered in control theory", *Systems & Control
  Letters* 13(1):59-64, 1989: stability under arbitrary switching holds exactly when a polyhedral
  Lyapunov function exists.
- [BLANCHINI] F. Blanchini, "Nonquadratic Lyapunov functions for robust control", *Automatica*
  31(3):451-461, 1995: polyhedral Lyapunov functions and their construction.
- [BLANCHINI-MIANI] F. Blanchini and S. Miani, *Set-Theoretic Methods in Control*, Birkhauser,
  2008: polyhedral functions as universal Lyapunov functions for linear uncertain and switched
  systems.
- [JOHANSSON-RANTZER] M. Johansson and A. Rantzer, "Computation of piecewise quadratic Lyapunov
  functions for hybrid systems", *IEEE Trans. Automatic Control* 43(4):555-559, 1998.
- [SHORTEN-NARENDRA] R. N. Shorten and K. S. Narendra, "Necessary and sufficient conditions for the
  existence of a common quadratic Lyapunov function for a finite number of stable second order
  linear time-invariant systems", *Int. J. Adaptive Control and Signal Processing* 16(10):709-728,
  2002.
- [MORSE] A. S. Morse, "Supervisory control of families of linear set-point controllers, Part 1:
  Exact matching", *IEEE Trans. Automatic Control* 41(10):1413-1431, 1996: dwell time for switching
  among stable systems.
- [HESPANHA-MORSE] J. P. Hespanha and A. S. Morse, "Stability of switched systems with average
  dwell-time", *Proc. 38th IEEE Conference on Decision and Control*, pp. 2655-2660, 1999.
- [GEROMEL-COLANERI] J. C. Geromel and P. Colaneri, "Stability and stabilization of discrete time
  switched systems", *Int. J. Control* 79(7):719-728, 2006: minimum dwell time for Schur-stable
  discrete-time modes.
- [LIBERZON] D. Liberzon, *Switching in Systems and Control*, Birkhauser, 2003.
- [LIN-ANTSAKLIS] H. Lin and P. J. Antsaklis, "Stability and stabilizability of switched linear
  systems: a survey of recent results", *IEEE Trans. Automatic Control* 54(2):308-322, 2009.
- [SHORTEN-ET-AL] R. Shorten, F. Wirth, O. Mason, K. Wulff and C. King, "Stability criteria for
  switched and hybrid systems", *SIAM Review* 49(4):545-592, 2007.
- [HIGHAM] N. J. Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., SIAM, 2002,
  ch. 2: the standard model of floating-point arithmetic.
- [LAROCHE] J. Laroche, "On the stability of time-varying recursive filters", *J. Audio Eng. Soc.*
  55(6):460-471, 2007: a transition matrix that contracts in a fixed norm keeps the state bounded
  under any coefficient variation.
- [ZAVALISHIN-TPT] V. Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4: the TPT SVF.
- [SIMPER-SVF] A. Simper, "Solving the continuous SVF equations using trapezoidal integration and
  equivalent currents", Cytomic, 2013: the stored-form state update.

## Appendix A: the check script (text only)

Python 3 with `numpy` and `mpmath`. It reproduces §3 to §5 and the certificate rows of §6.

```python
import numpy as np, mpmath as mp

RATES = [44100, 48000, 88200, 96000]
U = 2.0**-24                                   # f32 unit roundoff (math::tail::U)
UP = U*(1 + 2.0**-20)                          # f32 word rounding box, with f64-design slack
A_MAX = 10**(24/40)                            # gain +24 dB
K_LO, K_HI = 1/(18*A_MAX), A_MAX/0.1           # bell +24 dB Q 18; bell -24 dB Q 0.1
S_HI = np.sqrt((A_MAX + 1/A_MAX)*(1/0.1 - 1) + 2)   # shelf damping at slope 0.1, gain +-24 dB
L12, L22 = 0.0093375, 0.9998114                # the common norm L* = [[1, L12], [0, L22]]
f32 = np.float32

def words(g, k):                               # design_svf_words_f64, recursion words
    t = g*(g + k); c1 = t/(1 + t); a2 = g*(1 - c1); return c1, a2, g*a2

def P(c1, a2, a3):                             # L N L^-1, as math::tail::hull::transformed
    return (c1 - L12*a2, (a2 + L12*a3 - L12*c1 + L12*L12*a2)/L22, -L22*a2, L12*a2 + a3)

def margin(c1, a2, a3):                        # 1 - ||A||_Q, A = I - 2N, without cancellation
    p11, p12, p21, p22 = P(c1, a2, a3)
    T, K = p11 + p22, p12 - p21
    half = np.sqrt((1 - T)**2 + K*K)
    s2 = np.sqrt(4*(p22 - p11)**2 + 4*(p12 + p21)**2)
    return (2*T - T*T - K*K)/(1 + half) - 0.5*s2

def opn(p11, p12, p21, p22):
    return 0.5*(np.sqrt((p11 + p22)**2 + (p21 - p12)**2) + np.sqrt((p11 - p22)**2 + (p21 + p12)**2))

def box(b1, b2, b3):                           # sup over |e_i| <= b_i of 2 ||L N(e) L^-1||
    return np.maximum.reduce([2*opn(*P(s1*b1, s2*b2, s3*b3))
                              for s1 in (1, -1) for s2 in (1, -1) for s3 in (1, -1)])

Lm = np.array([[1, L12], [0, L22]]); FROM = np.linalg.norm(Lm, 2); TO = np.linalg.norm(np.linalg.inv(Lm), 2)
KAPPA = FROM*TO*(1 + 1e-12)
G2, G3 = 2*U/(1 - 2*U), 3*U/(1 - 3*U)
def C(c1, a2, a3):                             # weight-scaled part of math::tail::state_rounding_in
    a, b, c, d = 2*(1+U)*G2*c1, 2*(1+U)*G3*a2, 2*(1+U)*G2*a2, 2*(1+U)*G3*a3
    f, s, x = a*a + c*c, b*b + d*d, a*b + c*d
    return FROM*np.sqrt(0.5*(f + s + np.sqrt((f - s)**2 + 4*x*x)))*TO

def rects(fs):                                 # (g, k) rectangles holding every design
    w10, w20 = np.tan(np.pi*10/fs), np.tan(np.pi*20000/fs)
    return [((w10, w20), (K_LO, K_HI)),                                  # bell, pass, notch
            ((w10/np.sqrt(A_MAX), w20*np.sqrt(A_MAX)), (np.sqrt(2), S_HI))]  # both shelves

def cells(fs, n=300):                          # corner theorem per cell + monotone word sups
    m, rb, nu, cc = [], [], [], []
    for (ga, gb), (ka, kb) in rects(fs):
        ge, ke = np.geomspace(ga, gb, n + 1), np.geomspace(ka, kb, n + 1)
        G0, K0 = np.meshgrid(ge[:-1], ke[:-1], indexing='ij'); G1, K1 = np.meshgrid(ge[1:], ke[1:], indexing='ij')
        mm = np.minimum.reduce([margin(*words(g, k)) for g in (G0, G1) for k in (K0, K1)])
        den = 1 + G0*G0 + G0*K0
        sup = (words(G1, K1)[0], np.where(G1 <= 1, G1/den, np.where(G0 >= 1, G0/den, 1/(2 + K0))), G1*G1/den)
        m.append(mm); rb.append(box(*(UP*s for s in sup))); nu.append(box(*sup)); cc.append(C(*sup))
    cat = lambda x: np.concatenate([v.ravel() for v in x])
    return cat(m), cat(rb), cat(nu), cat(cc)

def design_f32(kind, f, G, Q, S, fs):          # kind 0..5 = LP, HP, notch, bell, low shelf, high shelf
    A = 10**(G/40); w = np.tan(np.pi*f/fs); sk = np.sqrt((A + 1/A)*(1/S - 1) + 2)
    g = np.select([kind <= 3, kind == 4, kind == 5], [w, w/np.sqrt(A), w*np.sqrt(A)])
    k = np.select([kind <= 2, kind == 3, kind >= 4], [1/Q, 1/(Q*A), sk])
    return np.stack(words(g, k), -1).astype(f32)

def mrg(W):
    W = W.astype(np.float64); return margin(W[..., 0], W[..., 1], W[..., 2])

def delta_div(fs):                             # (1+u) |1 - d (1 - p)| = 1 on the witness ray
    mp.mp.dps = 60
    g = mp.tan(mp.pi*10/fs); k = 1/(18*mp.power(10, mp.mpf(24)/40))
    t = g*(g + k); c1 = t/(1 + t); a2 = g/(1 + t); a3 = g*a2
    tr = 2 - 2*c1 - 2*a3; det = (1 - 2*c1)*(1 - 2*a3) + 4*a2*a2
    p = tr/2 + mp.sqrt(tr*tr/4 - det)
    lo, hi = mp.mpf(0), mp.mpf(1)
    for _ in range(200):
        mid = (lo + hi)/2
        lo, hi = (mid, hi) if (1 + mp.mpf(U))*abs(1 - mid + mid*p) > 1 else (lo, mid)
    return float(lo), float(1 - abs(p))

q12, q22 = L12, L12*L12 + L22*L22              # R(k) = -(M_k^T Q + Q M_k), M_k = [[-k, -1], [1, 0]]
for k in (K_LO, K_HI):
    R = np.array([[2*(k - q12), 1 + k*q12 - q22], [1 + k*q12 - q22, 2*q12]])
    print('R(%.6f) min eigenvalue %.4e' % (k, np.linalg.eigvalsh(R).min()))
for fs in RATES:
    rng = np.random.default_rng(2091*fs)       # one seed per rate
    m, rb, nu, cc = cells(fs)
    qD = (1 - m + rb).max() + 1e-13; mD = m.min(); gC = (cc/m).max(); gam = (nu/m).max()
    K, F, QQ, GG, SS = np.meshgrid(np.arange(6), np.geomspace(10, 20000, 100), np.geomspace(0.1, 18, 20),
                                   np.linspace(-24, 24, 13), np.linspace(0.1, 1, 5), indexing='ij')
    grid = 1 - mrg(design_f32(K.ravel(), F.ravel(), GG.ravel(), QQ.ravel(), SS.ravel(), fs)).min()
    n = 100000
    Wr = design_f32(rng.integers(0, 6, n), np.exp(rng.uniform(np.log(10), np.log(20000), n)), rng.uniform(-24, 24, n),
                    np.exp(rng.uniform(np.log(0.1), np.log(18), n)), rng.uniform(0.1, 1, n), fs)
    rand = 1 - mrg(Wr).min()
    w, Wb = Wr.copy(), Wr[rng.permutation(n)]; step = ((Wb - Wr)*f32(1/64)).astype(f32); walk = 0.0
    for j in range(1, 64):                     # every stepping frame of the walk
        w = (w + step).astype(f32)
        walk = max(walk, 1 - mrg(w).min())
    dd, pm = delta_div(fs)
    dcert = KAPPA*U/(mD*(1 - gC))
    print('\nfs %d: q_D = %.10f (m_D %.4e, max C/m %.1e, max nu/m %.0f); grid %.10f, random %.10f, walks %.10f, all <= q_D: %s'
          % (fs, qD, mD, gC, gam, grid, rand, walk, max(grid, rand, walk) <= qD))
    print('  witness 1-|p| %.4e, delta_div %.5f, delta*_cert %.5f; restarts B=1 cross delta_div after %d frames'
          % (pm, dd, dcert, np.ceil(np.log(dd)/np.log(63/64))))
    for name, om in [('dwell B>=64', 1/64), ('weight 1/32', 1/32), ('weight 1/16', 1/16), ('weight 1/8', 1/8)]:
        print('  %-12s per-step certified contraction - 1: phi=0 %+.3e, phi=0.6 %+.3e'
              % (name, KAPPA*U - om*mD*(1 - gC), KAPPA*U - om*mD*(1 - gC - 0.6)))
hbox = np.array([U/2, U/4, U/2]); E = 64*hbox*(1 + 3*U) + U
print('\nabsolute walk box (multiband crossover_envelope) excess in L*: %.4e' % box(*(E + hbox)))
```

Its output (abridged to 44.1 and 96 kHz):

```text
R(0.013955) min eigenvalue 9.2162e-03
R(39.810717) min eigenvalue 1.6936e-02
fs 44100: q_D = 0.9999934300 (m_D 6.5703e-06, max C/m 5.7e-04, max nu/m 4713); grid 0.9999934297, random 0.9999932189, walks 0.9999925122, all <= q_D: True
  witness 1-|p| 9.9411e-06, delta_div 0.00544, delta*_cert 0.00916; restarts B=1 cross delta_div after 332 frames
  dwell B>=64  per-step certified contraction - 1: phi=0 -4.244e-08, phi=0.6 +1.916e-08
  weight 1/16  per-step certified contraction - 1: phi=0 -3.502e-07, phi=0.6 -1.039e-07
fs 96000: q_D = 0.9999969819 (m_D 3.0182e-06, max C/m 5.7e-04, max nu/m 4765); grid 0.9999969818, random 0.9999967831, walks 0.9999966621, all <= q_D: True
  witness 1-|p| 4.5667e-06, delta_div 0.01247, delta*_cert 0.01994; restarts B=1 cross delta_div after 279 frames
  dwell B>=64  per-step certified contraction - 1: phi=0 +1.303e-08, phi=0.6 +4.133e-08
  weight 1/16  per-step certified contraction - 1: phi=0 -1.284e-07, phi=0.6 -1.518e-08
absolute walk box (multiband crossover_envelope) excess in L*: 6.1066e-06
```

## Appendix B: the walk simulation (method, text only)

The simulation runs the P2 law in `numpy.float32`: at each block boundary every lane takes a
target (the identity with probability `p`, else a random design from a pool with 30% domain
corners); the step is `float32((t - w) * float32(1/64))`; each frame uses the words in force and
then adds the step; the exact target applies after 64 frames, or earlier when the weight floor is
on. Next to the `f32` words it carries the exact chord in `f64` and the linear chord margin
`sum lambda_v m(v)` (the identity counts 0). The loss is `(m_lin - m(w_f32)) / m_lin`. Ten
schedules per rate, 256 lanes and 2,000 frames each, with `(B, p)` in `(64, 0)`, `(63, 0)`,
`(1, 0.5)`, `(2, 0.5)`, `(7, 0.5)`, `(16, 0.5)`, `(31, 0.7)`, `(63, 0.5)`, `(1, 0.9)` and
`(4, 0.9)`. With the weight floor at 1/16 (run with attempt 1's end snap) the largest loss is
1.2e-3 (44.1 kHz, `B = 64`) and the smallest executed weight is 1/64, on the zero-state frames of
class 3. The figure carries over to the hold of §6.3: the hold executes the same `f32` recursion
walk up to the floor and then repeats the last word, which adds no rounding; its class-3 rise is
part of an ordinary walk toward a design. Without the floor the same schedules reach `omega` of
1e-19 to 1e-26 with `B = 63`, far below `delta_div`.

## Appendix C: the ramp-rule simulation (text only)

`f64`, vectorised over lanes, one section per lane. Each lane is one design of the §6.1 domain with
one signal frequency; the input is `z^n`, `|z| = 1`, and the state starts at its steady state (or at
zero, settled at the identity, for the enable cases). The cases are: a disable at frame 0; an enable
from rest at frame 0 with a disable at frame 1, 2 or 3; a disable at frame 0 with a re-enable at
frame 61 or 63, run for 3,000 frames; and a disable restarted every frame for 400 frames, then a
re-enable, run for 3,000 more. The timing comparisons run today's rule twice with the same edits
one to three frames apart. "Added step" is the largest `|D_n - D_(n-1)|` of `D = y_rule - y_today`.

```python
"""#2091 attempt 2: output effect of three ramp rules toward the identity, in f64, vectorised.

today: the shipped law (P2, P5).
S:     attempt 1's end snap (all six words take the identity when the weight would fall below 1/16).
H:     hold. The recursion words (c1, a2, a3) stop stepping when their weight would fall below 1/16;
       the mix words (m0, m1, m2) finish today's 64-frame ramp; at the ramp end all six take the
       identity as today. If a ramp toward the identity starts with a recursion weight below 1/16
       (only within 3 frames of an enable from rest), the recursion words keep their design-ward
       step until the weight reaches 1/16, then hold. P5 is unchanged.
Input: a unit complex exponential, so |y_var - y_today| is relative to the input amplitude."""
import numpy as np

RATES = [44100, 48000, 88200, 96000]
DELTA = 1/16
IDENT = np.array([0, 0, 0, 1, 0, 0.0])

def design(kind, f, G, Q, S, fs):
    A = 10**(G/40); w = np.tan(np.pi*f/fs); sk = np.sqrt((A + 1/A)*(1/S - 1) + 2)
    one = np.ones_like(w)
    if kind == 'lp':
        g, k, m = w, 1/Q, (0*one, 0*one, one)
    elif kind == 'hp':
        g, k, m = w, 1/Q, (one, -(1/Q)*one, -one)
    elif kind == 'notch':
        g, k, m = w, 1/Q, (one, -(1/Q)*one, 0*one)
    elif kind == 'bell':
        k = 1/(Q*A); g, m = w, (one, k*(A*A - 1), 0*one)
    elif kind == 'ls':
        k = sk; g, m = w/np.sqrt(A), (one, k*(A - 1), (A*A - 1)*one)
    else:
        k = sk; g, m = w*np.sqrt(A), (A*A*one, k*(1 - A)*A, (1 - A*A)*one)
    t = g*(g + k); c1 = t/(1 + t); a2 = g*(1 - c1); a3 = g*a2
    return np.stack([c1, a2, a3, m[0]*one, m[1]*one, m[2]*one], -1)

def domain(fs):
    rows, labels = [], []
    F = np.concatenate([[10.0], np.geomspace(10, 20000, 26)[1:-1], [20000.0]])
    for kind in ('lp', 'hp', 'notch', 'bell', 'ls', 'hs'):
        Gs = (-24, -12, 0, 12, 24) if kind in ('bell', 'ls', 'hs') else (0,)
        Qs = (0.1, 0.3, 0.707, 2, 6, 18) if kind in ('lp', 'hp', 'notch', 'bell') else (0.707,)
        Ss = (0.1, 0.3, 1.0) if kind in ('ls', 'hs') else (1.0,)
        for G in Gs:
            for Q in Qs:
                for S in Ss:
                    rows.append(design(kind, F, G, Q, S, fs))
                    labels += [(kind, f, G, Q, S) for f in F]
    return np.concatenate(rows), labels

class Bank:
    def __init__(self, words, z, variant, state=None):
        n = len(words)
        self.v = variant; self.c = words.copy(); self.step = np.zeros_like(words)
        self.tgt = words.copy(); self.rem = np.zeros(n, int); self.rec_left = np.zeros(n, int)
        self.om = np.where(np.all(words == IDENT, 1), 0.0, 1.0); self.om0 = self.om.copy(); self.omt = self.om.copy()
        self.j = np.zeros(n, int); self.z = z; self.n = 0
        if state is None:
            c1, a2, a3 = words[:, 0], words[:, 1], words[:, 2]
            a, b, c, d = 1 - 2*c1, -2*a2, 2*a2, 1 - 2*a3
            det = (z - a)*(z - d) - b*c
            s1 = ((z - d)*2*a2 + b*2*a3)/det; s2 = (c*2*a2 + (z - a)*2*a3)/det
            self.s = np.stack([s1, s2], -1)
        else:
            self.s = state.astype(complex).copy()

    def retarget(self, t, ident):
        c = self.c
        settled_id = (self.rem == 0) & np.all(c == IDENT, 1)
        self.s[settled_id & (not ident)] = 0                      # P5, unchanged in every variant
        stat = np.all(c == t, 1)
        om_now = self.om.copy()
        keep = np.zeros(len(c), bool)
        if ident and self.v == 'H':
            # recursion words below the hold weight keep their design-ward ramp up to DELTA
            keep = ~stat & (om_now < DELTA) & (self.rem > 0)
        newstep = np.where(stat[:, None], 0.0, (t - c)/64)
        self.step[:, 3:] = newstep[:, 3:]
        self.step[:, :3] = np.where(keep[:, None], self.step[:, :3], newstep[:, :3])
        self.tgt = t.copy(); self.rem = np.where(stat, 0, 64)
        self.om0 = np.where(keep, self.om0, om_now)
        self.omt = np.where(keep, self.omt, 0.0 if ident else 1.0)
        self.j = np.where(keep, self.j, 0)
        self.rec_left = np.where(stat, 0, 64)
        if ident and self.v == 'S':
            r = np.where(om_now >= DELTA, np.floor(64*(1 - DELTA/np.maximum(om_now, 1e-300))) + 1, 0).astype(int)
            r = np.minimum(r, 64)
            now = ~stat & (r == 0)
            self.c[now] = IDENT; self.om[now] = 0; self.rem[now] = 0
            self.rem = np.where(~stat & ~now, r, self.rem)
        if ident and self.v == 'H':
            down = np.floor(64*(1 - DELTA/np.maximum(om_now, 1e-300))).astype(int)
            up = np.ceil(np.minimum((DELTA - om_now)*64/np.maximum(1 - self.om0, 1e-3), 64) - 1e-12).astype(int)
            self.rec_left = np.where(stat, 0, np.where(keep, np.maximum(up, 0), np.minimum(np.maximum(down, 0), 64)))

    def frame(self, x):
        c1, a2, a3, m0, m1, m2 = self.c.T
        ic1, ic2 = self.s[:, 0], self.s[:, 1]
        v3 = x - ic2; d1 = -c1*ic1 + a2*v3; v1 = ic1 + d1; d2 = a3*v3 + a2*ic1; v2 = ic2 + d2
        self.s = np.stack([ic1 + 2*d1, ic2 + 2*d2], -1)
        y = m0*x + m1*v1 + m2*v2
        ramp = self.rem > 0
        recstep = ramp & (self.rec_left > 0)
        self.c[:, 3:] += np.where(ramp[:, None], self.step[:, 3:], 0)
        self.c[:, :3] += np.where(recstep[:, None], self.step[:, :3], 0)
        self.j = np.where(recstep, self.j + 1, self.j)
        self.om = np.where(recstep, self.om0 + (self.omt - self.om0)*self.j/64, self.om)
        self.rec_left = np.where(recstep, self.rec_left - 1, self.rec_left)
        self.rem = np.where(ramp, self.rem - 1, self.rem)
        end = ramp & (self.rem == 0)
        self.c[end] = self.tgt[end]; self.om[end] = np.where(np.all(self.tgt[end] == IDENT, 1), 0.0, 1.0)
        self.n += 1
        return y

def db(g):
    return 20*np.log10(np.maximum(np.abs(g), 1e-300))

def schedule(words, z, variant, events, frames, state=None, start_ident=False):
    """events: {frame: ('id'|'design')}. Returns gains y/x per frame."""
    b = Bank(IDENT[None, :].repeat(len(words), 0) if start_ident else words, z, variant,
             None if not start_ident else np.zeros((len(words), 2)))
    out = np.empty((frames, len(words)), complex)
    for n in range(frames):
        if n in events:
            if events[n] == 'id':
                b.retarget(IDENT[None, :].repeat(len(words), 0), True)
            else:
                b.retarget(words, False)
        x = z**n
        out[n] = b.frame(x)/x
    return out
```
