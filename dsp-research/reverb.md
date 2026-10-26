# Reverb

## Scope and engineering question

Which stereo reverberator does `miso.reverb` (#2114) run as the console's aux return, and what
exact equations, coefficients, bounds and tests make it correct on every target? The note fixes the
algorithm to the level that S2 can implement the kernel and another agent can write an independent
`f64` oracle (`crates/dsp-reference/src/reverb.rs`) from the same text. It gives the stability and
tail argument as an outline with estimates. S3 writes the certified derivation
(`docs/derivations/2114-reverb-tail-and-rest.md`).

Not decided here: the descriptor bytes, the state-payload layout, the composition-value code, the
SDK catalog (S3 and S5), a mode enum, freeze or an output EQ (owner answers Q1-Q3: none), early
reflections as a separate stage, and listening verdicts (S4).

Measurement source: the scratch script
`/tmp/claude-1002/-home-bl-misofm-engine/258223fb-6f51-4ebb-b550-04d18c009589/scratchpad/opt/2114/measure.py`
(an `f64` numpy/scipy model of this design; not tracked; numbers below are its output). Command:
`venv/bin/python measure.py lengths tail ned color corr` in that directory, plus the probes
`probe3.py` (matrix held-out coloration), `probe_mod.py` (modulation), `probe_rt3.py` (RT60 grid),
`probe_n8.py` (N = 8), `check_distinct.py` (length rule), `pairs.py`, `pairs_measure.py` and
`pairs_cells2.py` (output pair) in the same directory. Revised 2026-10-26 after the oracle review:
the model now takes the outputs exactly as R8 (from the slot-ordered butterfly), and it runs R2's
allpasses at `k = 0` as pure delays. Every figure below is from the revised model, except M3
(see there).

## Adopted decisions

1. **A 32-line feedback delay network (FDN)** (#2114 attempt 5, root's ruling; attempts 1-4 had
   16 lines). Fact: an FDN with a unitary feedback matrix and per-line delays is lossless, and
   per-line gains set a frequency-dependent decay [JOT-CHAIGNE-FDN] [STAUTNER-PUCKETTE]
   [GERZON-UNITARY] [SCHLECHT-LOSSLESS] [SMITH-PASP-FDN]. Choice: N = 32. Two facts set it. The
   modal density of a delay network is the sum of its delays in seconds, modes per Hz
   [JOT-CHAIGNE-FDN] [SCHROEDER-1962], and the echo density after `m` trips grows with the number
   of lines (each trip multiplies the distinct path lengths by N) [JOT-CHAIGNE-FDN]. Measured
   reason: with 16 lines the late tail fails root's diffuse-noise coloration rule at size 1
   (test 4: worst std 1.45 dB against the reference's 1.03 dB), because the size-1 lengths give
   only 0.75 modes per Hz (attempt 3: the 16-line design meets the rule from size 1.5, 1.13 modes
   per Hz, up). Longer 16-line lengths buy the modal density but lengthen every trip, and the
   diffusion-0 mixing time fails test 2 (attempt 4: 324 ms against 200 ms). Doubling the line count
   at about the same mean length doubles the modal density without lengthening a trip: the
   32-line design has 1.50 modes per Hz at size 1 and passes both (test 2: 60 / 130 ms; test 4:
   2.34 / 0.95 dB against 2.83 / 1.03 dB). The 1.13 modes-per-Hz threshold is this project's
   measurement on the noise-reference rule (attempt 3), not a published figure.
2. **The feedback matrix is `H32 P / sqrt 32`.** `H32` is the 32 x 32 Sylvester Hadamard matrix,
   and P is the fixed permutation `(P f)_j = f_{pi(j)}`, `pi(j) = (29 j + 31) mod 32`. It is
   applied as a fixed five-stage butterfly (stage order 4, 3, 2, 1, 0), which gives identical bits
   for W = 1, 4 and 8; the `1 / sqrt 32` is folded into the line gains. Measured reason (attempt 5,
   criterion fixed before the run): among the 512 affine maps `(a j + b) mod 32`, `a` odd, the
   lowest held-out matrix coloration (the 64-path mean against the noise reference, +0.435 dB) with
   the diffusion-0 mixing time of every output within 200 ms; the identity gives +1.571 dB, the
   16-line map's analogue `(31, 30)` +0.675 dB. The 16-line selection (candidate E among six,
   `(14 - j) mod 16`) is in M1-M3.
3. **Absorption gains follow Jot.** `g_i = 10^(-3 D_i / (fs T))` with T = `decay_ms / 1000`
   [JOT-CHAIGNE-FDN] [SCHLECHT-RT60]. Every line then decays at the same per-sample rate
   `r0 = 10^(-3 / (fs T))`. The gain is applied **after** the matrix, on the line being written.
4. **Each line has a TPT one-pole low-pass and a TPT first-order high-pass in the loop**
   [ZAVALISHIN-TPT]. The damping cutoff is capped at `min(19,845 Hz, fs / 4)`, exactly as #1374
   (owner ruling 2026-10-25), so the coefficient is at most 1/2.
5. **The input diffusers are outside the loop.** Each channel has four Schroeder allpasses,
   **pipelined**: each stage reads the previous stage's output of the previous sample. The eight
   allpasses are then one 8-lane elementwise vector. This costs a fixed 3-sample wet-path delay and
   satisfies the rule that vectorisable render arithmetic must not stay scalar [DATTORRO-1997]
   [SCHROEDER-1962].
6. **Clear-on-read rings (a change to the scope note).** Every ring word of the 32 lines is set to
   `+0.0` when a tap reads it, so a non-zero word reaches the loop at most once. Without this, a
   size crossfade can read a word through both taps, and back-to-back crossfades can read it up to
   about `R / 128` times. No l2 bound then holds under an adversarial size history
   (Numerical and stability limits, S1-S3). Cost: one scalar store per line per sample, two in a
   crossfade.
7. **No modulation.** The measurement does not favour it. Over 32 conditions it changes the mean
   coloration per size by -0.17 to +0.51 dB (worse on balance) and NED by at most 3 ms (M4). This is provisional
   pending the S4 listening record. Dropping it removes the fractional reads and the proof's
   `(1 + m)` factor.
8. **The 32 line lengths are pairwise coprime primes at every size and rate.** `D_i` is the nearest
   prime to the scaled base length (Coefficients). With plain rounding, size 2 doubles every prime,
   so every pair would share the factor 2. With the prime rule, the shared-factor count over the
   496 pairs is zero at every `f32` size at all four rates (the unit test, exhaustive over the
   length breakpoints).
9. **The design is never banked and insert-only, with latency 0** (scope note section 3). The
   kernel is one `W`-generic body. Lanes are the 32 lines (and the 8 allpasses), not strips.

## Definitions and assumptions

- `fs` is one of 44,100, 48,000, 88,200 and 96,000 Hz. `n` is the sample index. `u = 2^-24`.
- Ports: `main-in` / `main-out` `DualMonoPlanar` (L, R). The effect is a stereo processor with an
  explicit 2 x 2 output matrix (width). It declares `DUAL_MONO` link mode only, `lane_link: false`,
  and `supports_mono_collapse: false` (stereo out from any input).
- **Line** `i` (0-31) is a delay ring. **Slot** `j` (0-31) is a position in the 32-word vector that
  the matrix acts on. Line `pi(j)` feeds slot `j`, with `pi(j) = (29 j + 31) mod 32`, so line `i`
  sits in slot `pi^-1(i) = (21 i + 21) mod 32` (`21 = 29^-1 mod 32`). The filter state of slot `j`
  belongs to line `pi(j)`.
- **Tap**: the read of line `i` at delay `D_i` samples. **Crossfade**: 128 samples
  (`TRANSITION_SAMPLES`), with weight `alpha_j = j / 128` (exact), at most one in flight and one
  queued, as in the delay. **Ramp**: a `LinearRamp` of 64 samples (`RAMP_SAMPLES`), advanced with
  `ramp_toward` (#1409).
- `fl(.)` is one `f32` rounding. `Lane::fma(a, b, c)` is `fl(fl(a b) + c)`, never fused.
- `lane::flush(x)` sets `|x| < FLUSH_EPS = 1e-20` to `+0.0` (D7).
- `X` is an input peak. `eps = 10^(-144/20)`. `G_p`, `G_t`, the decay `D` and the stalls are #1379's
  (`docs/derivations/1379-graph-tail-composition.md`).

## Algorithm and equations

### Constants

```text
BASE (48 kHz, size 1), line 0..31:
  953 1061 1163 1283 1409 1559 1721 1889 2087 2297 2531 2791 3079 3389 3733 4111 |
  1009 1109 1223 1361 1487 1637 1801 1987 2179 2411 2659 2927 3229 3557 3919 4327
  (line i < 16 takes RANKED[2i], line 16 + i takes RANKED[2i + 1];
   RANKED[k] = nearest prime to floor(960 * 4.5^(k / 31) + 0.5), k = 0..31: 19.9 ms to 90.1 ms)
DIFF_L (48 kHz) = 229 173 613 449        (4.77 3.60 12.77 9.35 ms: Dattorro's 142 107 379 277
DIFF_R (48 kHz) = 233 179 617 457         at 29,761 Hz, scaled, nearest prime; R = next prime above L)
B   = 0.5                                 input gain per line (exact); L into lines 0-15, R into 16-31
S   = fl32(1 / sqrt 32)                   output scale (not exact; 16 S^2 B^2 = 1/8, the 16-line level)
OUT = (14, 15)                            output rows: o_L from w_14, o_R from w_15
```

`sum BASE = 71,878` samples, about 1.50 modes per Hz at size 1 at every rate (1.4975 / 1.4975 /
1.4969 / 1.4973 at 44.1 / 48 / 88.2 / 96 kHz) [JOT-CHAIGNE-FDN] [SCHROEDER-1962]; the 16-line
design had 36,180 (0.75). Neighbouring lengths differ by a ratio of at least 1.035; the mean
length, 2,246 samples, is that of the 16-line design (2,261). The outputs read the butterfly,
which acts on the slot-ordered vector. So, per line, output `a` carries the sign pattern
`H32[a][pi^-1(l)]` on line `l`. A pair `(a, b)` is admissible when the two per-line patterns are
orthogonal on lines 0-15 and on lines 16-31: 480 ordered pairs. (14, 15) has the lowest held-out
coloration among the admissible pairs that pass test 3's grid and test 2 on `o_L` (attempt 5, D4;
+0.013 dB, `|r(0)|` at most 0.075). With an admissible pair, even the first trip of a one-sided
input reaches the two outputs with orthogonal sign patterns.

### Per sample (frame `n`), in this order

All coefficient words are the ramped values of this sample. Indices are per channel `c` in {L, R}.

```text
(R1) pre-delay      ring_pc[n] = x_c[n];  p_c = ring_pc[n - P]          (P = pre-delay samples, 0 allowed)
                    in a crossfade: p_c = fl(o + fl(alpha fl(nw - o)))  (o, nw the old and new taps;
                    the last fade sample delivers nw exactly, as the delay's tap_sample)
(R2) diffusers      eight lanes (c, s), s = 0..3; input a_{c,0} = p_c, a_{c,s} = y_{c,s-1}[n-1] (s >= 1)
                    d = ring_{c,s}[n - M_{c,s}]
                    v = fl(fl(k d) + a)                                   (Lane::fma(k, d, a))
                    y = fl(d - fl(k v))
                    ring_{c,s}[n] = flush(v)
                    e_c = y_{c,3}                                          (wet injection; 3-sample pipeline)
(R3) taps           t_i = ring_i[n - D_i]; then ring_i[n - D_i] = +0.0  (clear-on-read)
                    in a crossfade: t_i = fl(o + fl(alpha fl(nw - o))), both read words set to +0.0
(R4) slots          s_j = t_{pi(j)},  pi(j) = (29 j + 31) mod 32
(R5) low-pass       delta = fl(s_j - a_j); h = fl(g1 delta); l_j = fl(h + a_j); a_j = flush(fl(l_j + h))
(R6) high-pass      delta2 = fl(l_j - b_j); h2 = fl(g2 delta2); m_j = fl(h2 + b_j);
                    b_j = flush(fl(m_j + h2)); f_j = fl(l_j - m_j)
(R7) matrix         w = f; for k in (4, 3, 2, 1, 0): for every j with bit k of j clear, m = j + 2^k:
                        (w_j, w_m) = (fl(w_j + w_m), fl(w_j - w_m))
(R8) outputs        o_L = fl(S w_14), o_R = fl(S w_15),  S = fl32(1 / sqrt 32)
(R9) feedback       q_L = fl(B e_L), q_R = fl(B e_R)
                    u_j = flush(fl(fl(c_j w_j) + q_{L if j < 16 else R}));  ring_j[n] = u_j
(R10) width         mu = fl(0.5 fl(o_L + o_R)); sd = fl(width fl(0.5 fl(o_L - o_R)))
                    wet_L = fl(mu + sd), wet_R = fl(mu - sd)
(R11) mix           y_c = dry if mix == 0 or bypass; wet_c if mix == 1;
                    else fl(x_c + fl(mix fl(wet_c - x_c)))                 (the delay's mix_sample, selects)
(R12) ramps         every ramped word steps once (ramp_toward), as the delay's chunk kernel
```

`w_j` in R9 is written to line `j`. The matrix output index is the line index, and the slot index
belongs to R4-R6 only. In matrix form, with `F` the vector of filtered slot values:

```text
u = diag(c) H32 F + q,   F_j = HP(LP(t_{pi(j)})),   c_j = fl32(g_j / sqrt 32)
```

So the realised feedback matrix is `diag(g) (H32 / sqrt 32) P`, which is orthogonal up to the
gains (and the one rounding of each `c_j`).

**Bit identity across W.** The vectors hold slots in natural order (vector `v` holds slots
`vW .. vW + W - 1`). A butterfly stage on bit `k` pairs two whole vectors when `2^k >= W`.
Otherwise the kernel stores the vectors into a 32-word scratch array, reloads them in an order
that makes bit `k` a vector bit, and continues ("re-layout"). The stage order (4, 3, 2, 1, 0) and
each pair's `(sum, difference)` are fixed, so every target computes the same `f32` values:
W = 1 (all scalar), W = 4 (stages 4, 3, 2 vector, one re-layout, stages 1, 0 vector), W = 8
(stages 4, 3 vector, a re-layout, stages 2, 1 vector, a re-layout, stage 0 vector). R5, R6 and R9 are elementwise in the
slot or line layout that is current, with their per-slot constants (`c_j`) stored in that layout.
R3 and R9's ring access is scalar per line, because every line has its own length. The 8 diffuser
lanes are elementwise.

## Coefficients and update rules

Everything below is computed in `f64` with `math::` functions at prepare and at an accepted
block-start event, then rounded once to `f32`. The per-sample kernel has no transcendental.

- **Lengths.** `D_i(size, fs) = Prime(floor(BASE_i * size * fs / 48000 + 0.5))`, with `size` the
  `f32` control widened to `f64`. `Prime(n)` is the nearest prime to `n`, and on a tie the smaller.
  A sieve up to `Prime(floor(4327 * 2 * fs / 48000 + 0.5)) + 64` is built at prepare (at most
  17,372 bits, 2.2 KB at 96 kHz). The search is bounded: the largest prime gap below 17,500 is 44 (after 15,683).
  Checked at every `f32` size in `[0.25, 2]` at all four rates (the unit test; 14,877 / 16,029 /
  27,377 / 29,514 length breakpoints): 32 pairwise coprime primes, monotone in size. Range: 223
  (44.1 kHz, size 0.25) to 17,299 (96 kHz, size 2).
- **Ring capacity.** Line `i`: `R_i = D_i(2.0, fs) + 128` words, which leaves room for the delay's
  copy-out chunk shape. Totals: 136,124 / 147,834 / 268,248 / 291,576 words (544 / 591 / 1,073 /
  1,166 KB) at the four rates, twice the 16-line design's (587 KB at 96 kHz). The pre-delay ring is `floor(0.25 fs) + 1 + 128` words per channel.
  Diffuser `(c, s)` has `M_{c,s}(fs) = floor(DIFF * fs / 48000 + 0.5)` words; this does not scale
  with size.
- **Pre-delay.** `P = floor(pre_delay_ms * fs / 1000 + 0.5)`, in `[0, floor(0.25 fs)]`.
- **Absorption.** `g_i = exp(-3 ln(10) D_i / (fs T))`, `T = decay_ms / 1000`, and
  `c_i = fl32(g_i / sqrt 32)`, `g_i` and the quotient in `f64`, rounded once. Redesigned on every
  `decay_ms` or `size` event. `c_i` is in `(0, 0.17648]`, `sqrt(32) c_i <= g_i (1 + u)`; the largest
  `g` is 0.998282 (decay 20 s, size 0.25, the shortest line).
- **Damping.** `fc = min(damping_hz, 19,845, fs / 4)`, `G1 = tan(pi fc / fs)`,
  `g1 = G1 / (1 + G1)`, so `g1` is in `[0.0317, 0.5]`. `g1 = 0.5` exactly at the cap, which is
  `tan(pi / 4) = 1`, as #1374.
- **Low cut.** `G2 = tan(pi low_cut_hz / fs)`, `g2 = G2 / (1 + G2)`, so `g2` is in
  `[6.5e-4, 0.0345]`. No cap is needed, because 500 Hz is far below `fs / 4`.
- **Diffusion.** `k = fl32(0.7 * diffusion)`, in `[0, 0.7]`.
- **Width, mix.** The control values themselves.
- **Updates.** One event per parameter per block start (the delay's D6). `g1`, `g2`, `k`, `width`,
  `mix` and the 32 `c_i` ramp over 64 samples. `size` starts 32 tap crossfades of 128 samples and
  re-targets the 32 `c_i` ramps. `pre_delay_ms` starts two pre-delay crossfades. A tap or
  pre-delay crossfade that is already running queues the next one (one in flight, one queued).
  Settle time after the last event: `max(64, 2 * 128) = 256` samples.

## Numerical and stability limits

**S1: storage function and the settled contraction (the l2 argument).** The TPT one-pole
`v = g(x - s)`, `y = s + v`, `s' = y + v` with `g = G / (1 + G)` satisfies `y - s = G (x - y)`.
Hence `s'^2 - s^2 = 4 G y (x - y)`. With `V = s^2 / (2G)` this gives the low-pass output
`y^2 + dV <= x^2`, and the high-pass output `h = x - y` gives `h^2 + dV = x^2 - y^2 <= x^2`. Both
filters are passive with storage `s^2 / (2G)` [ZAVALISHIN-TPT]. The storage function is

```text
E = sum over lines of (unread ring words)^2 + sum_j ( a_j^2 / (2 G1) + b_j^2 / (2 G2) )
```

`H32 / sqrt 32` is orthogonal [GERZON-UNITARY] [SCHLECHT-LOSSLESS] and P is a permutation, so
`||u - q||^2 <= 32 c_max^2 ||F||^2 = g_max^2 ||F||^2 <= g_max^2 (||t||^2 - dV)` (up to the `c_i`
rounding). With zero input,
`dE = ||u||^2 - ||t||^2 + dV <= (g_max^2 - 1)(||t||^2 - dV) <= 0`. So E does not increase
under **any** control history, provided each word is read at most once (clear-on-read; in a
crossfade `||t||^2` is the blend energy, at most the energy of the words cleared) and every
`g_i <= 1`.

For a decay rate, take the weighted storage that weights a word `m` samples before its read by
`r^(-2m)`. With the gain on the written line and `g_j = r0^(D_j)`, the step multiplies the ring
part by `r^2`. The filters' storage needs the #1374-style majorant, because their group delay
lengthens a trip: `r` is found by bisection on `F(r) <= 1`, slightly above `r0`. S3 states the
certificate. Rounding: per trip, the tap blend `(1 + 6u)`, LP and HP `(1 + 8u)` each (#1374 F2-F3
form), five butterfly stages `(1 + u)^5` in l2, the gain `(1 + u)`, and the injection add
`(1 + u)`; `flush` only lowers a magnitude. These are collected in a `kappa` as #1374.

**S2: which histories the contraction covers.** The per-trip contraction (rate `r`) holds only on a
*settled* history: no event for 256 samples, every ramp landed, no crossfade. During ramps, the
`c_i` may belong to a `D` that the tap has not reached yet. During a crossfade, the tap reads two
words of one line. Both are covered by the non-expansion above, but not by the rate.
The ratio `c_j / r0^(D_tap)` is at most `r0^(-(D_new - D_old))`, which is bounded (`D <= R`).

**S3: the size re-read problem, and why clear-on-read.** Without clear-on-read, a crossfade from
`D1` to `D2 > D1` makes the new tap re-read words that the old tap already fed back. A sequence of
`size` events every 128 samples, each raising `D` by about 128, holds the read point on the same
128 words for up to `(R - D_min) / 128` (about 110) events. The loop then sees a gain of about
`c^2 * 110` in energy per trip. No finite peak bound linear in X exists under such a history, so
neither N1 nor a `G_p` could be stated. With clear-on-read, the re-read words are `+0.0`, so every
non-zero word enters the loop at most once with blend weight at most 1. The audible effect: a large
size increase leaves a gap in each line instead of a repeat (Known gaps).

**N1, the unconditional peak (outline and estimate).** *Superseded by `docs/derivations/2114-reverb-tail-and-rest.md` (S3): the storage `E` below is not passive under `damping_hz` or `low_cut_hz` automation (the TPT weight `1/(2G)` moves), so no peak bound linear in `X` is proved, and the certified tail starts from the block check's limit instead. The outline is kept as the record of the S1 estimate.* Under any admitted history, with
clear-on-read: the total read energy is at most the total written energy. Each word is at most
`R_max` samples old, and every `c_i <= c_max = g_max / 4` (16 lines) with `g_max = 0.998282` (decay 20 s,
size 0.25). A discounted l2 sum (`lambda^(n - k)`, with `c_max lambda^(-R_max) < 1`) then bounds
every word linearly in `X`. With `lambda` set so that `c_max lambda^(-R_max) = (1 + c_max) / 2`,
the estimate is a word bound of 137 dB (48 kHz) and 140 dB (96 kHz) above X, with fixed
`diffusion`. Under `diffusion` automation, the allpasses are no longer lossless. A crude `l-inf`
stage bound `(1 + k) / (1 - k) = 5.67` (four stages: 1,033) adds 60 dB. S3 should replace it with
an l2 bound: for one stage, the energy gained over any `k` history is at most `0.49 sum v^2`.
Clear-on-read is what makes a statable `G_p` possible at all (S3 above). The alternative of
bounding every word by `f32::MAX` after the finiteness check gives no gain linear in X, so N2's
`(3/4) eps 10^-k X` clause fails for small X.

**Diffusers and pre-delay.** The pre-delay is a convex tap blend of input words: its peak is at
most `(1 + 6u) X + 3U`, and it has no feedback, so a re-read cannot pump. At fixed `k`, each
allpass satisfies `y^2 - x^2 = (1 - k^2)(v_D^2 - v^2)`, so it is lossless with storage
`(1 - k^2) sum v^2`. Its impulse response decays by `k <= 0.7` per `M` samples. Both have finite
tails and add no term to the loop proof.

**Domains.** All controls are finite and in their descriptor domains. `g_i < 1` strictly, so there
is no freeze (owner Q2). Every in-loop filter coefficient is in `(0, 1/2]`.

## Latency and tail

- **Latency 0.** The dry path is direct. The wet path's first output arrives after
  `P + 3 + D_min(size)` samples (pre-delay, the diffuser pipeline, then the shortest line, 19.9 ms
  at size 1). Bypass is the dry input, with the same latency 0.
- **Tail estimate** to -144 dB below X at decay 20 s, size 2, damping 20 kHz. "Total" adds the
  settle time, `D_max`, 250 ms of pre-delay and the diffuser ringing (each stage to -170 dB).

| history | 48 kHz | 96 kHz |
|---|---|---|
| (a) settled history, l2 margin 54.4 / 57.4 dB | 3,427,695 samples (71 s, 3.41 T) | 6,951,468 (72 s, 3.46 T) |
| (b) any history, fixed diffusion (N1 above) | 5,531,999 (115 s, 5.60 T) | 11,255,536 (117 s, 5.70 T) |
| (b') any history, diffusion automated, crude | 6,496,258 (135 s) | 13,184,055 (137 s) |

The scope note's "about 2.4 T" (2,400,330 / 4,800,659 samples) omits the build-up margin.
A sustained input at a resonance stores `16 B^2 X^2 / (1 - r0^2)` of energy (16 lines), which is 54-57 dB
above X at T = 20 s. These are estimates. S3 certifies the actual values by bisection with roundings
counted, and states which row is the contract's `T`. The contract needs (b) or (b'), because N1
and the tail must hold for every admitted history.
- **Composition values (#1379).** The decay `D` (samples per 20 dB on the settled loop) is about
  `fs T / 3` (320,000 samples at 48 kHz, 20 s) and is statable. `G_p` is statable with
  clear-on-read, but it is large: about the N1 word bound, 137-200 dB. `G_t` comes from the settled
  margin plus the epsilon path. Whether a `G_p` this large is acceptable for graph composition is an
  open question for root (Known gaps, OQ1).
- **Rest.** `RestBound::Bounded`. Ring words are flushed on write and cleared on read, so every
  ring is `+0.0` within one ring length after the writes fall below `FLUSH_EPS`. The filter states
  then walk to `+0.0`; the high-pass state is the slowest, because `g2` can be as low as 6.5e-4.
  S3 uses the #1374 rate-follower walk. The diffuser rings and the pipeline registers are flushed
  on write. Cursors, ramps, crossfade state and the history counter are not signal state.
- **Reset.** Rings, filter states, diffuser rings and registers are set to `+0.0`; ramps are set to
  the current controls.

## Units, mappings, automation and smoothing

Every parameter is `Shared`, `AutomationRate::Block` and live (decision 14 rule 2). None is a
rebuild row.

| id | name | unit, mapping | domain | default | live mechanism |
|---|---|---|---|---|---|
| 1 | `pre_delay_ms` | ms, linear | 0-250 | 20 | 2 pre-delay crossfades, 128 samples |
| 2 | `decay_ms` | ms, logarithmic | 100-20,000 | 1,800 | 32 `c_i` redesigned, 64-sample ramps |
| 3 | `size` | ratio, linear | 0.25-2.0 | 1.0 | 32 tap crossfades (128) + `c_i` ramps (64); memory for 2.0 |
| 4 | `damping_hz` | Hz, logarithmic | 1,000-20,000 | 6,000 | `g1` ramp; cutoff capped at `min(19,845, fs/4)` |
| 5 | `low_cut_hz` | Hz, logarithmic | 20-500 | 20 | `g2` ramp |
| 6 | `diffusion` | linear | 0-1 | 0.7 | `k = 0.7 diffusion` ramp |
| 7 | `width` | linear | 0-1 | 1.0 | ramp; M/S 2 x 2 (R10) |
| 8 | `mix` | linear | 0-1 | 1.0 | ramp; exact dry at 0 and exact wet at 1 (selects) |

At 44.1 and 48 kHz, `damping_hz` is clamped at 11,025 and 12,000 Hz by the cap, so the top of its
range has no effect at the two base rates (#1374's documented loss).

## Denormal, signed-zero and NaN policy

The delay's D7, at instance granularity:

- **Flush sites, once per sample:** the 32 ring writes `u_j`, the 64 filter states `a_j` and
  `b_j`, and the 8 diffuser writes `v`. The pre-delay ring holds input words and is not recursive.
  `flush` maps `-0.0` to `+0.0`.
- **Finiteness, once per block:** a check over both outputs, the block's write window of every
  ring (32 lines, 8 diffusers, 2 pre-delay), the filter states and the pipeline registers. A
  non-finite word in either channel reaches both channels after one trip. So a failing block
  zeroes **both** outputs and drops **all** signal history, and increments both
  `nonfinite_left_blocks` and `nonfinite_right_blocks`. To keep the work bounded, history is dropped
  with the delay's `valid_history`-style counter: a tap whose word was written before the fault
  reads `+0.0`. The 32 rings are not cleared in a loop on the render thread.
- **Input:** a non-finite input enters the rings and is caught by the block check. No per-sample
  classification.
- **Output at rest:** `+-0.0`.

## Fixtures

The existing `fixtures/conformance/v1` impulses at the four rates suffice; no new `.mepcm`.
The oracle (`crates/dsp-reference/src/reverb.rs`, `f64`, imports no production code) implements
R1-R12 from this note. The determinism corpus (`src/corpus.rs`) has two cases: the default hall,
and automated `size` and `decay_ms`.

## Objective tests and tolerances

Each test is stated with the setting, the measured value from the scratch model and the bound.
The real `f32` kernel is expected within the stated margins.

1. **RT60 per band.** Settings: 48 and 96 kHz, decay 0.5 / 2 / 8 s, damping 20 k / 6 k / 2 k,
   size 1, diffusion 0.7, pre-delay 20 ms (default), all other controls at default.
   **Render length:** `L = floor((0.2 + 2 T) * fs)` samples from the impulse at `n = 0` (T in
   seconds). That is about 120 dB of decay after the earliest fit start, so the truncation of the
   backward integral does not bias the fit. The test renders four unit impulse responses of length
   L (L to L, L to R, R to L, R to R; the impulse in one input, zeros in the other).
   **Band filter:** `scipy.signal.butter(3, [fc / sqrt 2, min(fc sqrt 2, 0.45 fs)], btype="band",
   fs=fs, output="sos")`, run forward from zero state (`sosfilt`) on each response, for octave bands
   at fc = 125 / 500 / 2,000 / 8,000 Hz. The band energy is `e[n] = sum over the four responses of
   y[n]^2`.
   **EDC and fit (T40)** [SCHROEDER-1965]: `E[n] = sum_{m = n}^{L-1} e[m]` (no noise
   compensation), and `EDC[n] = 10 log10(E[n] / E[0])`. **Start:**
   `i0 = max(first n with EDC[n] <= -5, floor(0.2 fs))`. 0.2 s is the mixing time: test 2's
   bound at diffusion 0 (the 16-line design measured 193 ms, the 32-line design 130 ms). Before it the energy is not yet diffuse.
   **End:** `i1` is the first n with `EDC[n] <= EDC[i0] - 40`. Fit a least-squares line to
   `(n / fs, EDC[n])` for `n` in `[i0, i1)`; `T_meas = -60 / slope`.
   **Prediction:** the 32 lines' Jot values are combined delay-weighted, which holds for a
   well-mixed network: `T(f) = -3 sum_i D_i / (fs sum_i log10 |g_i H(f)|)`, with `H = HP(LP)` from
   the designed `f64` coefficients. It is evaluated on `f_k = geomspace(20, 0.45 fs, 600)` with
   weights `w_k = |H_band(f_k)|^2 f_k` (the `f_k` factor for the log spacing). The predicted band
   energy `e_p[n] = sum_k w_k 10^(-6 (n / fs) / T(f_k))` for `n < L` goes through the identical
   EDC, start, end and fit, which gives `T_pred`.
   **Gated cells:** bands with `fc <= min(damping_hz, 19,845, fs / 4)`, the designed damping
   cutoff. Above it the per-trip loss at the band centre is over 3 dB (4.5-13 dB in the excluded
   cells), which is outside the small-loss regime of Jot's smooth-decay model. I chose this
   criterion after the measurement; it is tied to the design, not to the error values.
   **Bound:** `|T_meas / T_pred - 1| <= 5 %` in every gated cell. Measured on the 32-line kernel
   (attempt 5): worst +4.16 % (125 Hz, decay 2 s, damping 20 kHz, 48 kHz); at 8 kHz worst +3.64 %
   (damping 20 kHz, 48 kHz, decay 8 s). Excluded cells, recorded: 8 kHz at damping 6 kHz -1.3 % to
   -12.0 %, and at damping 2 kHz -16 % to -67 %. The 16-line design measured 3.2 % (125 Hz-2 kHz)
   and 4.4 % (8 kHz).
   **Filter-free control** (both loop filters removed, so every mode decays at exactly `r0` and the
   true T is known): worst 1.7 % (decay 0.5 s, 125 Hz, 48 kHz), 1.4 % at 96 kHz, and at most 0.9 %
   at decay 2 s and 8 s. The 1 % target is not met at decay 0.5 s. The residual is modal beating
   (about 66 modes in the 125 Hz octave at size 1): moving the start from 0.2 s to 0.3 s swings
   that cell from +1.7 % to -3.1 %. So a start chosen to hit 1 % would be fitted to noise.
   **Rules this replaces:** the earlier fit (start at the later of -5 dB and 150 ms, 20 dB span,
   length `max(1.5, 0.8 T + 0.4)` s) read +7.3 % on the control. A 20 dB or 30 dB span from 0.2 s
   still read 3.2-3.8 %. Red if a gain, filter, length or size mapping is wrong.
2. **Echo density** [ABEL-HUANG-NED]: `eta(t)` with a 20 ms Hann window normalised to unit sum,
   divided by `erfc(1 / sqrt 2)`, on a 1 ms hop. Settings: L impulse, L output, 48 kHz, size 1,
   decay 1.8 s, damping 6 kHz, pre-delay 0. Time counts from the impulse at `n = 0`. **Bound:** the
   50 ms moving mean of `eta` reaches 0.9 by 150 ms at diffusion 0.7 (measured 60 ms on the 32-line
   kernel; 104 ms with 16 lines) and by 200 ms at diffusion 0 (measured 130 ms; 193 ms with 16). At diffusion 0, R2's allpasses are pure delays, so the
   wet path starts `sum M + 3 = 1,467` samples (30.6 ms) later than with no diffusers. Diffusion 0
   isolates the matrix: B 324 ms, C 228 ms, Householder-16 279 ms, N = 8 341 ms and N = 4
   Householder 740 ms all fail it (16-line candidates, M1). Plain H16 passes (181 ms); it is
   rejected on coloration (test 4). At diffusion 0.7 the diffusers dominate, and every 16-line
   candidate passes. The 16-line design with lengths scaled up to 1.13 modes per Hz fails it
   (324 ms, attempt 4), which is why the line count doubled instead. The
   Gaussian-noise reference's 50 ms mean never falls below 0.946.
3. **Inter-channel correlation** of the wet output after 100 ms, at lag 0
   (`sum L R / sqrt(sum L^2 sum R^2)` from `floor(0.1 fs)` to the end). Render length
   `max(1, 0.5 T)` s. **Bounds:** `|r(0)| <= 0.1` at width 1 for a centred (L = R) impulse over
   size {0.25, 1, 2} x diffusion {0, 0.7, 1} x decay {1.8, 8} s (measured on the 32-line kernel:
   worst 0.064 at size 1, diffusion 0, decay 1.8 s; 0.030 at defaults; 16 lines: 0.073 and
   0.029). At width 0, `r(0) >= 0.99`; it is exactly 1 by construction, since
   `wet_L = wet_R = mu`. Not gated: a one-sided impulse (measured worst 0.098 on the same grid,
   0.001 at defaults), and decay 0.5 s (measured worst 0.137). After 100 ms at 0.5 s, the tail is
   still mostly the first trips, whose echoes carry unequal gains `g_i`, so the orthogonal sign
   patterns do not cancel. Red if the output rows are not orthogonal or R10 is wrong.
4. **Coloration.** Settings: 48 kHz, size 1, decay 1.8 s, damping 20 kHz, low cut 20 Hz,
   diffusion 0.7, all four paths (L to L, L to R, R to L, R to R). For each path, take the tail
   from 0.2 to 1.7 s and multiply it by `10^(3 t / T)`. Compute the power spectrum, average it in
   1/24-octave bands from 200 Hz to 8 kHz, and divide by the 1-octave average at the same centre.
   **Rule (root's, attempt 2; fixed before the design was measured):** the mean over the four
   paths of the peak-to-median and the worst path's std are at most the maxima, over 32 fixed
   seeds, of the same two figures of a perfectly diffuse reference: white Gaussian noise (four
   independent paths per seed) whose every frequency decays at the design's predicted `T(f)` (test
   1's prediction from the 32 line lengths), through the same segment, bins and smoothing. The
   window, 0.2-1.7 s, starts after the mixing-time bound (150 ms at diffusion 0.7). The claim is
   only modal-density collapse. **Measured on the 32-line kernel (attempt 5):** paths 2.66 / 2.43 /
   1.84 / 2.43 dB, mean **2.34 dB** against the reference's 2.83 dB; std 0.94 / 0.95 / 0.78 / 0.85,
   worst **0.95 dB** against 1.03 dB. Pass. The 16-line design fails the same rule (mean 2.831 dB
   against 2.826, worst std 1.45 dB on the R-L path, attempt 2): its 0.75 modes per Hz at size 1
   are too few (attempt 3 measured the worst std falling with size and meeting the reference from
   1.13 modes per Hz). `P` = identity fails it (model: 3.59 / 1.34 dB). One path's peak-to-median
   is noisy, so the bound is on the four-path mean. The attempt-1 statistic (fixed 3.5 / 1.5 dB
   bounds) is replaced. This gates only size 1.
5. **Tail contract** (`tests/tail_contract.rs`, the delay's file as template), at decay 20 s,
   size 2 and damping 20 kHz, at 48 and 96 kHz, on the real kernel. The output must be below
   -144 dB by `tail` and bit-exact at rest by `rest`. Include one run with a size crossfade and a
   decay ramp in flight at `N`, and one run of adversarial size steps of +128 samples per block
   followed by silence, which defends clear-on-read.
6. **Partition invariance** over block lengths 1 / 7 / 64 / 128 / 512, and the randomized
   differential against the oracle.
7. **NaN and denormal:** an injected NaN or infinity zeroes the block's outputs, increments both
   counters and recovers. No non-finite word survives into a ring. State below `FLUSH_EPS` is
   `+0.0`.
8. **Determinism corpus and conformance**, including zero allocations under `bench_support`.

## Benchmark plan

One descriptive console-workload row: an aux-return submix with `miso.reverb` at defaults, 48 kHz,
128-frame blocks, on the Wasm `simd128`, AArch64 NEON and AVX2 targets. Frozen workload, one
warmup, two measured rounds, `isolated_cycles_per_lane_sample`. No threshold.

Expected work per sample (counted from R1-R12; not a gate):
- Elementwise per line: 26 ops (LP 8, HP 9, gain 1, injection 2, flush 3, `c` ramp 3), 29 in a
  crossfade. For 32 lines that is 832 lane-ops: 208 vector ops at W = 4, 104 at W = 8 (16 lines:
  416 lane-ops, 104 / 52).
- Matrix: `5 * 32 = 160` lane-ops (16 lines: 64). At W = 4: 40 vector ops plus one 32-word
  re-layout (16 lines: 16 plus one 16-word). At W = 8: 20 vector ops plus two 32-word re-layouts,
  64 scalar moves (16 lines: 8 plus three 16-word re-layouts, 48 moves).
- Output scale: two scalar multiplies (16 lines: two exact `0.25` multiplies, same count).
- Diffusers: 7 ops x 8 lanes (14 vector ops at W = 4, 7 at W = 8), unchanged.
- Scalar memory: 32 tap loads, 32 clears and 32 writes per sample (twice as many in a crossfade),
  plus 8 diffuser and 4 pre-delay accesses (16 lines: 16 / 16 / 16).
- Total vector ops per sample, settled: W = 4 262 (16 lines 134), W = 8 131 (16 lines 67); scalar
  ring accesses 96 (48). So the per-sample cost about doubles with N.

The rejected B would save 12 vector ops (W = 4) but would need 30 scalar adds for the two output
rows, which E gets free from `w_14` and `w_15`.

## Listening protocol or evidence

S4 records blinded ABX on `dsp-research/listening/TEMPLATE.md`. Items: drums, vocal, piano, pad.
Three listeners. Evidence only, never a gate. Comparisons:
(1) modulation on against off, rendered from the scratch `f64` model, since the kernel has none.
This decides whether decision 7 is reopened.
(2) N = 32 against N = 16 (scratch renders; the coloration rule decided N, the listening record
confirms or reopens it).
(3) A size sweep 0.5 to 2.0 over 2 s, clear-on-read against read-side re-read (scratch renders),
to hear decision 6's gap artefact.
No listening claim is made in this note.

## Primary and official sources

- [JOT-CHAIGNE-FDN]: FDN structure; per-line absorption from a target reverberation time
  (paywalled preprint; cited for provenance, and the absorption rule is also given in
  [SMITH-PASP-FDN]).
- [STAUTNER-PUCKETTE]: multichannel recirculating delays with an orthogonal mixing matrix.
- [GERZON-UNITARY]: unitary (energy-preserving) feedback networks.
- [SMITH-PASP-FDN]: FDN tutorial, Hadamard and Householder matrices, mutually prime lengths.
- [SCHLECHT-LOSSLESS]: the lossless condition for FDN matrices and delays.
- [SCHLECHT-ECHO-DENSITY]: echo density and mixing time as functions of the matrix.
- [SCHLECHT-TIME-VARYING]: time-varying feedback matrices. Context for decision 7 and the gaps;
  not adopted.
- [SCHLECHT-RT60]: per-line absorption filters only approximate a target RT60. This is context
  for test 1, which measures this design's own deviation instead of assuming it.
- [ROCCHESSO-SMITH-CIRCULANT]: circulant matrices, a rejected alternative.
- [VALIMAKI-FIFTY]: survey of reverberation algorithms, for the comparison of candidates.
- [SCHROEDER-1962], [MOORER-1979]: comb and allpass reverberators (rejected loop) and the allpass
  section (adopted diffuser).
- [DATTORRO-1997]: the plate tank (rejected) and the input-diffuser lengths (adopted, scaled).
- [SCHROEDER-1965]: backward integration (test 1).
- [ABEL-HUANG-NED]: normalised echo density (test 2).
- [ZAVALISHIN-TPT]: TPT one-pole and its state form (S1).

## Rejected alternatives and tradeoffs

- **Schroeder/Moorer comb-allpass loop** [SCHROEDER-1962] [MOORER-1979]: low modal density and
  coloration (survey [VALIMAKI-FIFTY]). Its parallel combs are an N-line FDN with a diagonal matrix,
  which has no mixing.
- **Dattorro plate tank** [DATTORRO-1997]: a figure-eight of nested allpasses, which is two serial
  scalar chains and a poor SIMD fit. Its l2 argument needs a storage term per nested stage. Only its
  input-diffuser idea is kept.
- **N = 8.** Coloration 3.42 / 1.64 dB against 2.39 / 0.95 (size 1), and mixing 341 ms against
  193 ms at diffusion 0. The `1 / sqrt 8` scale is also inexact.
- **N = 16 (attempts 1-4).** Fails root's noise-reference coloration rule at size 1 (worst std
  1.45 dB against 1.03, 0.75 modes per Hz); with lengths scaled to 1.13 modes per Hz it fails the
  diffusion-0 mixing bound (324 ms against 200). Half the cost of N = 32 (Benchmark plan).
- **Matrix A, plain Sylvester H16.** It is an involution. Four-path coloration mean 4.67 dB against
  E's 2.85 dB, and the held-out mean (M3) is 6.18 dB against E's 3.63 dB.
- **Matrix B, `(H2 x I8) P / sqrt 2` (Pease, one stage per trip).** Cheapest, but it mixes slowly:
  324 ms against 193 ms at diffusion 0, and `eta` is 0.27 against 0.41 at 100-150 ms. The
  `1 / sqrt 2` scale is inexact (folded into `c_i`, one extra `u`). It needs 30 scalar adds for the
  outputs. Held-out coloration 5.75 dB.
- **Matrix C, `(H4 x I4) P2 / 2`.** 228 ms at diffusion 0, and it needs a re-layout at W = 8
  anyway. Held-out coloration 5.87 dB.
- **Householder 16, `I - 11^T / 8`.** Mostly diagonal (7/8): 279 ms at diffusion 0, held-out
  coloration 5.88 dB.
- **Circulant** [ROCCHESSO-SMITH-CIRCULANT]: needs a cyclic shift, which `Lane` lacks; not measured.
- **Delay-length modulation:** no measured gain (M4); deferred to listening.
- **Read-side crossfade without clearing:** no bound under size automation (S3).
- **Write-side size change with a read-and-clear ring:** also provable, but it needs a
  read-modify-write blend for two write heads. Clear-on-read with read-side crossfades is the
  smaller change.
- **Gain before the matrix:** the poles are the same, but the output rows would then carry the
  unequal `g_i` weights. Output orthogonality would no longer be exact, and the weighted-storage
  rate pairing (`g_j` with the written line's `D_j`) would no longer apply.

## Known gaps and follow-up

- **OQ1, for root:** `G_p` is finite under any history only with clear-on-read, and it is about
  137-200 dB. Is a node peak gain this large acceptable to #1379's composition? Otherwise the
  contract needs a different clause for a resonant node. Not guessed here.
- **OQ2, for root:** the design change of decision 6 (clear-on-read) and the size-up gap artefact
  replace the scope note's plain crossfade.
- The tail values are estimates (Latency and tail). S3 certifies them, tightens (b') with an l2
  bound for automated diffusion, and fixes the per-word discount.
- Coloration at size 0.25 has a four-path mean of 7.65 dB (std up to 4.4) with 16 lines: low
  modal density (0.19 modes per Hz; 0.37 with 32 lines, not re-measured) is inherent to a small
  room. It is not gated.
- Test 3 does not gate decay 0.5 s or a one-sided input (measured 0.137 and 0.098).
- Test 1 gates only bands at or below the damping cutoff. Above it, the decay is not one
  smooth exponential. Two fixes were measured and do not close it:
  - A one-third-octave analysis band at 8 kHz is worse: +10 % to +12 % at decay 8 s, damping
    6 kHz.
  - Proportional absorption (per-line cutoff `fc sqrt(D_mean / D_i)`, Jot's equal-loss-per-sample
    rule in first order) improves 2 kHz (worst 1.9 %) but gives -6.8 % in the 8 kHz / 6 kHz / 8 s
    cell.
  Proportional absorption, or a higher-order absorption filter, is a follow-up for root, because
  it changes the damping design (`probe_rt7.py`).
- The modulation decision is provisional until the S4 listening record.
- Size changes do not move the diffusers.
- There are no early reflections apart from the diffusers and the first trip. The first wet sample
  comes `D_min(size)` after the pre-delay, so pre-delay 0 still sounds about 20 ms late at size 1.

## Decision record

2026-10-26, attempt 5 (root's ruling: N = 32, every bound unchanged). Scratch design record
`scratchpad/opt/2114/att5_decisions.md` (criteria fixed before each run; `att5_*.py`, `f64`):
D1 lengths (ranked 32-prime rule, coprime over all 496 pairs at every `f32` size and rate, 1.50
modes per Hz); D2 `B = 0.5`, `S = fl32(1 / sqrt 32)`, `c_j = fl32(g_j / sqrt 32)`; D3 slot map
`(29 j + 31) mod 32` (best of 512 affine maps on held-out matrix coloration, +0.435 dB, 503 of
512 within the mixing bound); D4 outputs `(14, 15)` (best of 334 eligible of 480 admissible ordered
pairs, +0.013 dB held-out). The gated run, once, on the model: test 2 60 / 130 ms; test 4 2.345 /
0.952 dB against 2.833 / 1.028 dB; test 3 0.075; test 1 4.16 %. The real `f32` kernel then gave
the values in "Objective tests" (the same to the printed digits except test 3, 0.064).

2026-10-26, S1 (this note), from the scratch measurements. Settings, unless stated otherwise:
48 kHz, size 1, decay 1.8 s, impulse into L, `f64` model.

- **M1, mixing (NED, 50 ms mean of `eta`, time to reach 0.9; diffusion 0.7 / 0, R2 at k = 0):**
  A 96 / 181 ms, B 144 / 324, C 105 / 228, Householder-16 127 / 279, **E 104 / 193**,
  E with modulation 101 / 193, N = 8 121 / 341, N = 4 Householder 241 / 740. At diffusion 0.7 the
  diffusers dominate. At diffusion 0 the matrix decides: `eta` at 100-150 ms is A 0.46, B 0.27,
  C 0.38, Householder 0.35, E 0.41.
- **M2, coloration (test 4, four paths; mean peak-to-median / largest std, dB):** A 4.67 / 1.69,
  B 3.38 / 1.37, C 4.72 / 1.65, Householder 3.23 / 1.18, **E 2.85 / 1.44**, E with modulation
  3.32 / 1.37, N = 4 Householder 5.26 / 3.06; Gaussian noise (one path) 1.63-2.82 / 0.64-0.89.
  For A-D, the outputs are rows 14 and 15 of `H16` on the line-ordered vector.
- **M3, selecting P without overfitting:** a sweep over all 128 affine permutations
  `j -> (a j + b) mod 16` on four conditions found the minimum at `a = 15, b = 14` (E). It was then
  re-scored on 24 held-out conditions (size 0.5 / 0.75 / 1.5, diffusion 0.3 / 1, decay 1 / 2.5 s,
  L or R). Held-out mean peak-to-median: **E 3.63**, next 4.05, plain A 6.18, B 5.75, C 5.87,
  Householder 5.88. This sweep predates the revision: every candidate used the same fixed output
  pattern (rows 5 and 10 of `H16` on the line-ordered vector), so it compares matrices like for like,
  but its absolute values are not test 4's.
- **M4, modulation** (4 lines, linear interpolation, at most 8 samples at 48 kHz, 0.30-0.87 Hz,
  integer triangle). Over 32 conditions (size x decay x channel x diffusion), the mean
  peak-to-median per size, off / on: size 0.25 6.87 / 7.38, 0.5 4.04 / 3.87, 1 2.10 / 2.25,
  2 2.08 / 2.14. NED moved by at most 3 ms. **Dropped,
  provisional pending listening.**
- **M5, N = 8 against N = 16 coloration (L to L, peak-to-median / std):** size 1, decay 1.8 s:
  3.42 / 1.64 against 2.39 / 0.95; decay 6 s: 2.79 / 1.74 against 1.92 / 0.88. Size 2:
  2.20-3.19 against 1.68-2.17. Size 0.25: 8.31-8.73 against 7.73-7.82.
- **M6, RT60 (second revision; T40 from the later of -5 dB and 0.2 s, length 0.2 + 2 T, gated
  bands at or below the damping cutoff):** worst 3.2 % (125 Hz-2 kHz) and 4.4 % (8 kHz). The
  filter-free control is within 1.7 %. Each earlier fit was off by up to 8.8 % (`probe_rt5.py`,
  `probe_rt6.py`, `probe_rt7.py`; `out_rt60_v3.txt`, `out_rt60_v4.txt`, `out_rt60_prop.txt`).
- **M7, length rule:** plain rounding gave 120 of 120 pairs with a common factor at size 2. The
  nearest-prime rule gives 0 at every rate and size checked. Attempt 2 checks every `f32` size of
  the domain at the four rates on the production rule (`gcd`, unit test
  `line_lengths_are_pairwise_coprime_at_every_size_and_rate`): 0 pairs.
- **M8, output pair (revision).** 56 pairs are admissible (`a xor b` in 1..7). First screen over all
  56, on 9 correlation cells and 8 coloration cases: the best by worst-case coloration were (13, 14),
  (3, 7), (14, 15), (13, 15), (6, 7), (4, 7), (12, 14), (12, 15). Re-scored on test 3's full grid
  restricted to decay >= 1.8 s (centred input), (13, 14) reaches `|r|` 0.157, (3, 7) 0.254, (4, 7)
  0.133 and (12, 15) 0.123, so they fail. The three that pass at decay >= 1.8 s (`|r|` centred / one-sided; four-path
  peak-to-median, mean over 16 cases of size {1, 2} x decay {1.8, 4} s):
  (14, 15) 0.073 / 0.098, 2.55; (6, 7) 0.054 / 0.094, 2.92; (12, 14) 0.081 / 0.097, 2.88.
  **Chosen: (14, 15)**, for the lowest coloration. The oracle review found the error: the earlier
  (5, 10) was measured on the line-ordered vector, which is not R8's tap.
- Delivered to the coordinator as OQ1 and OQ2 above; root decides.
