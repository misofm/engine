# De-esser

## Scope and engineering question

How should the launch de-esser (issue #15) detect sibilance and attenuate it, with zero latency,
dual-mono state, live parameters and bit-identical scalar, four-lane and eight-lane renders? This
note does not decide lookahead (deferred), console eligibility (an owner question for S3) or the
certified tail bound (deferred with the parametric EQ's #1372, see Known gaps).

## Adopted decisions

- **Topology.** One TPT state-variable filter per channel on the main input; the selected tap `f`
  (band-pass or high-pass) is the detector and the listen output. Wideband mode `y = g x`, listen
  `y = f`. Split mode with the band-pass detector `y = x - (1 - g) b` (Zoelzer, DAFX, ch. 4: the
  de-esser as a compressor whose side chain, and in split form whose processed band, is a band-pass
  of the input). Split mode with the high-pass detector is a dynamic high shelf (#1984):
  `y = g x + (1 - g) l`, `l` the low-pass output of a second, shelf SVF per channel on the main
  input at the same frequency with fixed Butterworth damping `k = sqrt 2`; Q shapes only the
  detector there. With `g = 1` the split output is the input exactly (a zero gain word selects the
  dry word).
- **Shared band step.** The detector and gain law are `effect_runtime::dynamic_band`, shared with
  the dynamic EQ (#17); its `f64` oracle is `dsp_reference::dynamic_band`. The de-esser owns only
  its output law.
- **Gain law.** Hard knee, downward: the change is `min(0, (1/R - 1)(L - T))`, clamped at
  `-range` [REISS-COMP eq. 4 with W = 0], smoothed in dB by a branching one-pole (attack while the
  change grows). Measured against the oracle: within `2.9e-5` dB per sample on an 8 kHz burst,
  attack and release crossing times equal to the sample.
- **Filter words.** `g = tan(pi f / fs)`, `k = 1/Q`, `c1 = t/(1+t)`, `a2 = g(1-c1)`, `a3 = g a2` with
  `t = g(g+k)`, designed in `f64` by `parametric_eq::design_svf_words_f64` and rounded once
  [SIMPER-SVF; ZAVALISHIN-TPT]; the same transfer functions as the cookbook band-pass (0 dB peak)
  and high-pass [RBJ-COOKBOOK], which the oracle uses.
- **Zero latency, one quality, no lookahead** (root decision for S1).

## Definitions and assumptions

`x` main input word, `s` sidechain word, `v1, v2` the SVF band and low outputs, `b = k v1` the
unity-peak band-pass tap, `h = x - k v1 - v2` the high-pass tap, `f` the selected tap, `l` the
shelf SVF's low-pass output (`Q = 1/sqrt 2`, same frequency, same input), `L` the
detector level in dB, `T` threshold in dB, `R` ratio, `G` the smoothed change in dB (`<= 0`),
`g = 10^(G/20)`. Dual mono: each channel has its own filter, envelope and parameters; a link mode
(dual mono, maximum, average) combines the two detector magnitudes before the dB conversion.
A connected sidechain feeds a second SVF with the same words for the detector only.

## Algorithm and equations

Per channel and sample, in this order: ramps advance; `v1, v2 = svf_step(x)`; `b = k v1`;
`h = x - b - v2`; `f = b + D (h - b)` (`D` the detector switch word, selected at 0 and 1);
`l = svf_step_shelf(x).v2` (the same flush threshold as the main SVF) on a live lane (`M` and `D`
both nonzero; any other lane holds `l = 0` and a `+0.0` shelf state, and a block in which no lane
can be live skips the step); detector tap `d = f` (or the sidechain SVF's tap); after both channels:
`L = clamp(fast_level_db(max(link(|d_own|, |d_partner|), 1e-8)), -160, 24)`;
`t = max(min(0, (1/R - 1)(L - T)) , -range)`; `c = attack if |t| > |G| else release`;
`G = snap(G + c (t - G))`; `g = fast_gain_from_db(G)`; `wide = g x`;
`split = blend((g - 1) b + x, g (x - l) + l, D)` (`x` when `G = 0`; the band cut takes the band tap
`b`, not the crossfaded `f`, so every point of a detector switch is a convex mix of two responses
with `|H| <= 1` and stays `<= 1`);
`y = blend(blend(wide, split, M), f, Λ)`, `y = x` when bypassed. `M` and `Λ` are the mode and
listen words; `blend(a, b, w)` is `a` at `w = 0`, `b` at `w = 1`, `a + w (b - a)` between.

## Coefficients and update rules

A parameter record commits its value at the block's first sample and retargets the ramps whose
design moved: threshold and range in dB, the slope `1/R - 1`, the rate coefficients
`1 - exp(-1000 / (t fs))`, the four SVF words, the switch words (`1.0` for split, high, listen
on) and the shelf SVF's three words (the same design at `Q = 1/sqrt 2`, which move only with the
frequency). Every ramp is 64 samples, `ramp_toward` with the endpoint clamp (#1409) and the target on the
final sample. Linear interpolation of `(c1, a2, a3)` between two stable designs stays contractive:
the transition norm is convex in the words (`effect_runtime::svf`).

## Numerical and stability limits

Frequency 1 to 16 kHz, Q 0.5 to 8 at the four launch rates: `c1, a2, a3` lie in `(0, 1)` and `k` in
`[1/8, 2]`. The level is floored at `1e-8` and clamped to `[-160, 24]` dB; the change lies in
`[-40, 0]` dB. The fast dB tier is within `1e-5` dB (`crates/math/tests/f1_fast_db_bounds.rs`).
Out-of-band transparency of the split arm: the change at frequency `f` is
`20 log10 |1 - (1 - g) H(f)|`, `H` the band-pass response. It is within 0.1 dB, for every range up
to 40 dB, at least this far from the centre (in octaves, either side): Q 0.5: 3.72, Q 1: 2.75,
Q 2: 1.83, Q 4: 1.08, Q 8: 0.58. This is the "nominal" region of the acceptance gate; nearer the
centre the band-pass skirt (6 dB per octave) is attenuated by design.

The high shelf (#1984). With `w` the prewarped frequency over f_c, `s = jw` and the Butterworth
`LP = 1 / (s^2 + sqrt2 s + 1)`, the split response is `H = g + (1 - g) LP =
(g s^2 + g sqrt2 s + 1) / (s^2 + sqrt2 s + 1)`. For any numerator `g s^2 + b s + 1` over the
denominator `s^2 + k s + 1`, `|H|^2 - 1 = -w^2 ((1 - g^2) w^2 + k^2 - b^2 - 2 (1 - g)) / |D|^2`, so
`|H| <= 1` at every frequency exactly when `k^2 >= b^2 + 2 (1 - g)`. With `k = sqrt 2` and
`b = g k` that is `2 >= 2 / (1 + g)`, true for every `g` in `[0, 1]`: no boost at any frequency and
any held gain. The response is unity at DC, `g` at Nyquist and 12 dB per octave between. Its known
limit: above about 1.5 octaves over f_c the low-pass phase nears -180 degrees and `|H|` dips
under `g` -- a deeper cut, never a boost: -27.3 dB at +1.74 octaves for -20 dB held (-25.8 dB at
+2 octaves), 2.9 dB under `g` at -10 dB held, 17 dB under `g` (at +3.3 octaves) at -40 dB held.
At f_c itself the cut is `|g - j (1 - g)/sqrt2|`: -4.77 dB at -10 dB held, -3.82 dB at -20 dB,
-3.1 dB at -40 dB, so the held gain is reached about an octave above f_c, not at it.

## Latency and tail

Latency 0 in every mode; bypass is the dry word (latency preserved). The split and listen arms
carry the SVF's ring-out after the input stops. The descriptor states `NodeTailBound::UNBOUNDED`
(no certified tail or exact-rest bound) until the live-SVF bound of #1372 exists (Known gaps). The
state still reaches exact rest: the SVF's joint flush is armed by the input's silence clock (#1328)
and the change word snaps to `+0.0` within `5.38e-7` dB (the compressor's plateau edge, #1784).
Measured: Q 8, 50 ms release, 0.9 full-scale noise burst: every lane at rest 38,784 frames after the
input stops at 48 kHz.

## Units, mappings, automation and smoothing

ids 1 frequency_hz (Hz, log, 6500), 2 q (log, 2), 3 threshold_db (-60..0, -20), 4 ratio (1..20, log,
4), 5 range_db (0..40, 20), 6 attack_ms (0.1..50, log, 1), 7 release_ms (5..500, log, 50), 8 mode
{wideband 1, split 2} (split), 9 detector {band 1, high 2} (band), 10 listen (Boolean, off). Every
parameter is block-rate, automatable, per lane, linearly smoothed over 64 samples. One observation
tap: Gain Reduction (dB, resident, per block, per lane, peak magnitude).

## Denormal, signed-zero and NaN policy

SVF integrators flush below `1e-20` per word and jointly below `1e-14` after `N_SILENCE` zero input
frames; the change word snaps to `+0.0` (so `-0.0` never persists). Once per block a lane whose
output or state words (gain, integrators) are non-finite or at least `1e30` is zeroed for the
block, re-seeded from its prepared values and reported per channel (D7); a NaN state word is
caught even when the wideband arm never reads it. Parameter values `-0.0` are normalised to `+0.0`.

## Fixtures

Deterministic in-test signals only (no corpus, per the 2026-08-23 issue comment): an 8 kHz burst at
-6 dBFS on a 24,000-sample period (1,000,000 samples), the same burst over a -30 dBFS 1 kHz tone, a
7 kHz burst over the 1 kHz tone, and xorshift noise for the bank and rest tests.

**Measured response (#1921 O1).** One run of a throwaway harness (not committed) on
`tests/support` at `origin/main` 4448e90b3, `cargo test --release -p de-esser`, scalar instance,
48 kHz, dual mono, both channels equal (their output and change words are bit-equal). Every
parameter not named is the default (Q 2, threshold -20 dB, ratio 4, range 20 dB, attack 1 ms,
release 50 ms). The change is the resident Gain Reduction observation (the change word `G`), read
after each one-frame block; output gains are DFT-bin ratios against the dry input.

Detector taps, read through listen (`y = f`), unit impulse, 65,536 frames, Q 2, in dB; the
difference to the cookbook band-pass (0 dB peak) and high-pass at the same f_c and Q is at most
`2.9e-6` dB at all 18 points:

| f_c | Tap | -1 octave | f_c | +1 octave |
|---|---|---|---|---|
| 4 kHz | band-pass | -10.226 | +0.000 | -10.946 |
| 4 kHz | high-pass | -10.378 | +6.021 | +1.743 |
| 6.5 kHz | band-pass | -10.612 | +0.000 | -12.780 |
| 6.5 kHz | high-pass | -11.027 | +6.021 | +1.257 |
| 10 kHz | band-pass | -11.530 | +0.000 | -19.435 |
| 10 kHz | high-pass | -12.593 | +6.021 | +0.325 |

The high-pass tap peaks at `20 log10 Q = +6.02` dB at f_c, so with it the detector reads the band
6 dB hotter than the band-pass does. The +1 octave column narrows toward Nyquist as the bilinear
design does.

Output modes: f_c 8 kHz, an 8 kHz burst at the stated peak level from 50 ms to 250 ms over a
-30 dBFS 1 kHz tone, 24,000 frames. "Held" is the mean change over the burst's last 10 ms (the
150-250 ms mean is the same to 0.001 dB); the gains are over 150-250 ms. "Curve" is the static
hard-knee change at the tap's peak level (+6.021 dB at f_c for the high-pass tap). The detector
reads the instantaneous magnitude of the sampled tap, so the smoother settles short of the curve:
about 1 dB with the band-pass tap, about 0.17 dB with the high-pass tap. The band-pass shortfall is
mostly sampling: 8 kHz is fs/6, and the band-pass tap, with 0 degrees of phase at f_c, is sampled
at `|sin| <= 0.866` (-1.25 dB) and never at its peak; measured from that sampled peak, the
band-pass shortfall is about 0.02 dB at -14 dBFS (0.09 dB at -6 dBFS). Attack is the first frame after the onset where the
change reaches 90 % of held, release the first frame after the burst where it falls to 10 %.

| Mode, detector | Burst (dBFS) | Curve | Held | 8 kHz gain | 1 kHz gain | Attack 90 % | Release 10 % |
|---|---|---|---|---|---|---|---|
| split, band-pass | -26 | 0 | 0.000 | +0.000 | +0.000 | none | none |
| split, band-pass | -14 | -4.500 | -3.538 | -3.538 | -0.008 | 173 (3.60 ms) | 5,527 (115.15 ms) |
| split, band-pass | -6 | -10.500 | -9.468 | -9.469 | -0.013 | 169 (3.52 ms) | 5,528 (115.17 ms) |
| split, high-pass (#1984) | -26 | -0.015 | -0.033 | -0.033 | -0.000 | 831 (17.31 ms) | 5,526 (115.12 ms) |
| split, high-pass (#1984) | -14 | -9.015 | -8.842 | -4.755 | -0.026 | 291 (6.06 ms) | 5,528 (115.17 ms) |
| split, high-pass (#1984) | -6 | -15.015 | -14.842 | -4.340 | -0.017 | 252 (5.25 ms) | 5,530 (115.21 ms) |
| wideband, band-pass | -26 | 0 | 0.000 | +0.000 | +0.000 | none | none |
| wideband, band-pass | -14 | -4.500 | -3.538 | -3.538 | -3.538 | 173 (3.60 ms) | 5,527 (115.15 ms) |
| wideband, band-pass | -6 | -10.500 | -9.468 | -9.469 | -9.468 | 169 (3.52 ms) | 5,528 (115.17 ms) |
| wideband, high-pass | -26 | -0.015 | -0.033 | -0.033 | -0.033 | 831 (17.31 ms) | 5,526 (115.12 ms) |
| wideband, high-pass | -14 | -9.015 | -8.842 | -8.841 | -8.842 | 291 (6.06 ms) | 5,528 (115.17 ms) |
| wideband, high-pass | -6 | -15.015 | -14.842 | -14.841 | -14.842 | 252 (5.25 ms) | 5,530 (115.21 ms) |
| listen, band-pass | -26, -14, -6 | as split | as split | +0.000 | -24.820 | as split | as split |

The detector and the change do not depend on the output mode: each detector row gives the same
change in split, wideband and listen. In split mode with the band-pass detector the 8 kHz gain
equals the held change and the 1 kHz tone moves at most 0.013 dB; wideband applies the held change
at every frequency; listen is the band-pass tap (0 dB at f_c and -24.820 dB at 1 kHz, the cookbook band-pass
value three octaves below f_c). The release matches the one-pole `50 ms * ln 10 = 115.13 ms`; the 90 % attack (3.5 ms with
the band-pass tap) is longer than the one-pole `1 ms * ln 10 = 2.30 ms` because the attack branch
runs only while the instantaneous target exceeds the change, part of each cycle. In the -26 dBFS
high-pass row the tap's peak is about `-26 + 6.02 = -19.98` dB, a hair over the threshold, hence
its small, slow change.

**Split with the high-pass detector (#1984).** The three "split, high-pass" rows are re-recorded
on the dynamic high shelf (the same harness shape, #1984's branch; the wideband rows reproduced to
the digit). Before #1984 the arm was `x - (1 - g) h` and boosted: +4.202 and +5.661 dB at 8 kHz and
+0.072 and +0.092 dB at 1 kHz at -14 and -6 dBFS. Now 8 kHz is cut by `|g - j (1 - g)/sqrt2|`, the
shelf's value at f_c (-4.755 dB at the -8.842 dB change, -4.340 dB at -14.842 dB; the deeper cut
lies above f_c, Numerical limits), and 1 kHz moves at most 0.026 dB. The detector and the change
columns are unchanged, as the detector path is.

## Objective tests and tolerances

`effect-runtime/tests/dynamic_band.rs`: the band step against the oracle, change within 0.1 dB per
sample, attack (to 90 %) and release (to 10 %) within max(1 sample, 2 %), downward hard knee and
upward 6 dB knee, a million samples finite. `de-esser/tests/output_law.rs`: every arm within `1e-5`
of the oracle (the high shelf against an RBJ Butterworth low-pass, `g x + (1 - g) l`); split
isolation with both detectors (1 kHz moves at most 0.1 dB, measured `-0.020` dB; 7 kHz cut at least
3 dB, measured `-7.0` dB); the high shelf's response on the product (#1984: a held -20 dB, probes
from -2 to +2 octaves at 96 kHz, Q 0.5 to 4, each at most +0.02 dB and within 0.5 dB of
`|g + (1 - g) LP|`; measured equal to the analytic value to 0.001 dB at every probe). `bank.rs`: each bank lane bit-equal to its scalar instance under
automation and every link mode; collapsed then desymmetrized bank bit-equal to a dual one.
`rest.rs`: exact rest after silence; a skipped block equals a rendered one. The conformance macro.

## Benchmark plan

None in S1 (not registered, so no production path runs it). A console record of
`isolated_cycles_per_lane_sample` and its class-A floor is an owner item, not #1921's: no
post-limiter effect has a floor row, and the measured column needs a controlled console record
with a core clock, which this host does not produce (#1921 decision record).

## Listening protocol or evidence

Not yet: a blinded vocal listening session needs a person; it is an owner item (#1921 decision
record), in one packet with the dynamic EQ's if the owner wants it.

## Primary and official sources

- [REISS-COMP] the static curve (equation 4) and the branching dB-domain smoother.
- [RBJ-COOKBOOK] the band-pass (constant 0 dB peak) and high-pass responses the oracle realises.
- [SIMPER-SVF] and [ZAVALISHIN-TPT] the TPT SVF, its taps and its word design.
- U. Zoelzer (ed.), *DAFX: Digital Audio Effects*, 2nd ed., Wiley 2011, ch. 4 (dynamics
  processing; the de-esser topology). Not yet in the bibliography.

## Rejected alternatives and tradeoffs

- A dynamic bell (the dynamic EQ's law) for the split arm: `x - (1 - g) b` is the same family with
  a fixed-pole band; kept for the stated `g = 1` identity and one output-law line.
- A `listen` port role: rejected for a Boolean parameter (no new port role, root decision).
- Lookahead: deferred (latency 0 at launch).
- `ar_one_pole_step` (two-product retention form): the rate form is used because the attack and
  release coefficient ramps (#1336) are designed for it, as in the gate.
- The high shelf on the detector SVF's own poles, `g x + (1 - g) v2` (#1984's first proposal):
  by the bound in Numerical limits it is boost-free only for `Q <= sqrt((1 + g)/2)` (0.742 at
  -20 dB held). At the default Q 2 it boosts +5.1 dB at f_c (+11.1 dB at Q 4). No second-order
  shelf sharing those poles is boost-free for `Q > 1/sqrt 2`; the boost is the poles' resonance.
- `Q_shelf = min(Q, 1/sqrt 2)` instead of a fixed Butterworth shelf: it follows Q below the
  Butterworth value, so the shelf's corner shape would change with a parameter the user sets for
  the detector; fixed damping keeps one shelf shape and the same bound.
- A first-order shelf: no dip under `g`, but 6 dB per octave, which leaves the 1 kHz tone at
  about -0.1 dB at the defaults (the isolation gate's margin).
- An RBJ shelf with moving poles: needs a per-sample coefficient redesign as `g` moves.
- A shelf that steps in every mode (#1984 attempt 1, so that a switch never starts it cold): it
  cost about 15 % of the `audit de-esser` workload in the modes that never read it. Rejected for
  the gated shelf below; a per-frame runtime branch on the gate also cost about 9 %, so the gate is
  a const segment parameter (`SHELF`).
- Cost and cold start of the adopted gated shelf: the shelf steps only on a live lane (split and
  the high-pass detector), and a block in which no lane can be live compiles it out. The
  `cargo run -p audit -- de-esser` workload (an eight-lane bank and a connected scalar instance,
  100,000 blocks of 128; wall time of the whole process, one warmup and two rounds, descriptive,
  not a floor row), base f62e4312d against #1984: wideband 1,644 / 1,636 against 1,655 / 1,655 ms
  (+0.9 %), split + band-pass 1,638 / 1,639 against 1,656 / 1,651 ms (+0.9 %), split + high-pass
  1,638 / 1,631 against 1,857 / 1,867 ms (+14 %, the shelf's own work). A switch onto the path
  starts the shelf from `+0.0`: against an always-on shelf, on a -1.8 dBFS-peak mix (150 Hz at
  0.3, 1 kHz, noise, an 8 kHz burst) the output differs by at most -49.7 dB re the input peak for
  at most 112 samples (mode switch at 1 kHz) and -47.5 dB for 17 samples at 6.5 kHz; a detector
  switch -74.4 dB (1 kHz) and -50.0 dB (6.5 kHz). The detector and mode words ramp over 64
  samples, which fades the shelf arm in over the transient.

## Known gaps and follow-up

- The certified tail and exact-rest bound of a live, Q-variable SVF (#1372's derivation); until
  then the descriptor is `UNBOUNDED`, which #1378 must retire.
- Registry and consumer wiring (S2), banking surfaces and console eligibility (S3), lookahead (S4);
  the measured response tables are recorded (#1921 O1, Fixtures); the floor row and blinded
  listening are owner items.
