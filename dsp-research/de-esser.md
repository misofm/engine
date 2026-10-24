# De-esser

## Scope and engineering question

How should the launch de-esser (issue #15) detect sibilance and attenuate it, with zero latency,
dual-mono state, live parameters and bit-identical scalar, four-lane and eight-lane renders? This
note does not decide lookahead (deferred), console eligibility (an owner question for S3) or the
certified tail bound (deferred with the parametric EQ's #1372, see Known gaps).

## Adopted decisions

- **Topology.** One TPT state-variable filter per channel on the main input; the selected tap `f`
  (band-pass or high-pass) is the detector and the attenuated band. Split mode `y = x - (1 - g) f`,
  wideband mode `y = g x`, listen `y = f` (Zoelzer, DAFX, ch. 4: the de-esser as a compressor whose
  side chain, and in split form whose processed band, is a band-pass of the input). With `g = 1`
  the split output is the input exactly (a zero gain word selects the dry word).
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
unity-peak band-pass tap, `h = x - k v1 - v2` the high-pass tap, `f` the selected tap, `L` the
detector level in dB, `T` threshold in dB, `R` ratio, `G` the smoothed change in dB (`<= 0`),
`g = 10^(G/20)`. Dual mono: each channel has its own filter, envelope and parameters; a link mode
(dual mono, maximum, average) combines the two detector magnitudes before the dB conversion.
A connected sidechain feeds a second SVF with the same words for the detector only.

## Algorithm and equations

Per channel and sample, in this order: ramps advance; `v1, v2 = svf_step(x)`; `b = k v1`;
`h = x - b - v2`; `f = b + D (h - b)` (`D` the detector switch word, selected at 0 and 1);
detector tap `d = f` (or the sidechain SVF's tap); after both channels:
`L = clamp(fast_level_db(max(link(|d_own|, |d_partner|), 1e-8)), -160, 24)`;
`t = max(min(0, (1/R - 1)(L - T)) , -range)`; `c = attack if |t| > |G| else release`;
`G = snap(G + c (t - G))`; `g = fast_gain_from_db(G)`; `wide = g x`; `split = (g - 1) f + x`;
`y = blend(blend(wide, split, M), f, Λ)`, `y = x` when bypassed. `M` and `Λ` are the mode and
listen words; `blend(a, b, w)` is `a` at `w = 0`, `b` at `w = 1`, `a + w (b - a)` between.

## Coefficients and update rules

A parameter record commits its value at the block's first sample and retargets the ramps whose
design moved: threshold and range in dB, the slope `1/R - 1`, the rate coefficients
`1 - exp(-1000 / (t fs))`, the four SVF words, and the switch words (`1.0` for split, high, listen
on). Every ramp is 64 samples, `ramp_toward` with the endpoint clamp (#1409) and the target on the
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

## Objective tests and tolerances

`effect-runtime/tests/dynamic_band.rs`: the band step against the oracle, change within 0.1 dB per
sample, attack (to 90 %) and release (to 10 %) within max(1 sample, 2 %), downward hard knee and
upward 6 dB knee, a million samples finite. `de-esser/tests/output_law.rs`: every arm within `1e-5`
of the oracle; split isolation (1 kHz moves at most 0.1 dB, measured `-0.020` dB; 7 kHz cut at
least 3 dB, measured `-7.0` dB). `bank.rs`: each bank lane bit-equal to its scalar instance under
automation and every link mode; collapsed then desymmetrized bank bit-equal to a dual one.
`rest.rs`: exact rest after silence; a skipped block equals a rendered one. The conformance macro.

## Benchmark plan

None in S1 (not registered, so no production path runs it). A console record of
`isolated_cycles_per_lane_sample` and its class-A floor is a stateless successor (S5).

## Listening protocol or evidence

Not yet: a blinded vocal listening record (folds into #26) is a stateless successor (S5).

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

## Known gaps and follow-up

- The certified tail and exact-rest bound of a live, Q-variable SVF (#1372's derivation); until
  then the descriptor is `UNBOUNDED`, which #1378 must retire.
- Registry and consumer wiring (S2), banking surfaces and console eligibility (S3), lookahead (S4),
  fixtures, plots, benchmark and listening (S5).
