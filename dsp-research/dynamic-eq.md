# Dynamic EQ

## Scope and engineering question

How should the launch dynamic EQ (issue #17) move a bell band's gain with a detector, with zero
latency, dual-mono state, live parameters and bit-identical scalar, four-lane and eight-lane
renders, so that a band whose detector is idle is exactly the parametric EQ's bell (#87)? This note
does not decide dynamic shelves (slice S4, owner-gated), console eligibility (S3, owner), lookahead
(deferred), or the certified tail bound (deferred with the parametric EQ's #1372, see Known gaps).

## Adopted decisions

- **Four bell bands**, cascaded in index order (root; the parametric EQ's general band count).
- **Static poles, modulated mix** (`docs/research/f64-introduction.md` section 4, rules 1 to 3).
  Each band's pole words `c1, a2, a3` and its mix word `m1` are `parametric_eq::design_svf(Bell, f,
  G, Q)` at the **static** gain `G`, designed in `f64` and rounded once [SIMPER-SVF;
  ZAVALISHIN-TPT]. The dynamics move only the band mix: `m1[n] = m1 + k A^2 (g[n] - 1)`, so at
  `g = 1` the word is the EQ's `m1` exactly and the band is #87's bell. No per-sample
  transcendental beyond the shared fast dB tier.
- **Detector**: the same SVF's unity-peak band-pass `k v1` (no second filter), or, with the external
  detector and a connected sidechain, a second SVF with the same words on the sidechain. The level,
  link, gain law and dB smoother are the shared `effect_runtime::dynamic_band` (also the
  de-esser's, #15) [REISS-COMP eq. 4, with a knee].
- **Direction and range**: downward cuts above the threshold, upward boosts above it; the change is
  clamped to `[-range, range]`.
- **Zero latency, one quality, no lookahead** (root decision for S1).

## Definitions and assumptions

`x` the band's input (the effect input for band 1, the previous band's output after it), `s` the
sidechain word, `v1, v2` the SVF band and low outputs, `G` the static gain in dB, `A = 10^(G/40)`,
`k = 1/(Q A)` the bell's damping, `b = k v1` the unity-peak band-pass at quality `Q A`, `L` the
detector level in dB, `T, R, W` threshold, ratio and knee, `D` the smoothed change in dB, `g =
10^(D/20)`. Dual mono: each channel has its own filters, envelopes and parameters; a link mode (dual
mono, maximum, average) combines the two channels' detector magnitudes of the same band before the
dB conversion.

## Algorithm and equations

The cookbook bell is `H(s) = (s^2 + s A/Q + 1) / (s^2 + s/(A Q) + 1) = 1 + (A^2 - 1) B(s)`, `B` the
0 dB-peak band-pass at quality `A Q` [RBJ-COOKBOOK]. The band keeps `B` and scales it:
`y = x + (A^2 g - 1) b = x + k (A^2 g - 1) v1`, so the centre gain is `G + D` dB exactly; away from
the centre the bandwidth stays the static design's (a fixed-pole dynamic bell [ORFANIDIS-ISP
ch. 11], and Zoelzer, DAFX, ch. 4).

Frozen per-frame order, per channel: the ramps advance (and, while ramping, the knee words are
redesigned); the effect input's and the sidechain's silence clocks step; then band by band:
`(v1, v2) = svf_step(x)`, `b = k v1`, detector `d = b` (or the sidechain SVF's tap, crossfaded by the
detector word); after both channels of the band: `L = clamp(fast_level_db(max(link(|d|), 1e-8)),
-160, 24)`; target `t = +-gain_delta_db(L)` (negated upward) clamped to `[-range, range]`;
`D = snap(D + c (t - D))`, `c` the attack rate while `|t| > |D|`, else release; `g =
fast_gain_from_db(D)`; `m = k A^2 (g - 1) + m1` (with `g - 1 = 0` selected at `D = 0`); `x <- x` if
`m = 0`, else `x + m v1`. The output is the last band's word, or the dry word when bypassed.

## Coefficients and update rules

A parameter record commits its value at the block's first sample and retargets the ramps whose
design moved: per band the threshold, the slope `1/R - 1`, the knee width, the range, the rate
coefficients `1 - exp(-1000 / (t fs))`, the six filter words `c1, a2, a3, k, m1, k A^2` (`k` and
`k A^2 = A/Q` rounded once from the same `f64` amplitude as the design), and the direction and
detector switches. Every ramp is 64 samples, `ramp_toward` with the endpoint clamp (#1409), the
target on the final sample. While a ramp runs, the knee words `(W/2, 1/(2W))` are redesigned per
frame from the ramped width with `knee_coefficients`'s operations, so the curve stays continuous;
the one division is in ramping frames only. The direction switches at its ramp's midpoint. A
disabled band ramps its six filter words to `+0.0`, the EQ's identity section; once settled its
frozen integrators are cleared at the next block boundary (unobservable: its tap and mix are zero).

## Numerical and stability limits

Frequency 20 Hz to 20 kHz, Q 0.1 to 16, gain +-24 dB (inside the EQ's design domain, below every
launch rate's Nyquist): `c1, a2, a3` in `(0, 1)`, `k` and `k A^2` in `[0.0157, 39.8]`, `|m1| < 37.3`.
Interpolated `(c1, a2, a3)` stay contractive (the transition norm is convex in the words,
`effect_runtime::svf`). The change lies in `[-24, 24]` dB; the level is floored at `1e-8` and
clamped to `[-160, 24]` dB; the fast dB tier is within `1e-5` dB.

## Latency and tail

Latency 0; bypass is the dry word. Every band rings out after the input stops. The descriptor states
`NodeTailBound::UNBOUNDED` (no certified tail or exact-rest bound) until the live-SVF bound of
#1372 exists; that is the de-esser's ruling, with its reopening condition. The state still reaches
exact rest: the joint flush is armed by the effect input's silence clock (#1328), the change snaps to
`+0.0` within `5.38e-7` dB (#1784), and a disabled band's frozen state is cleared. Measured: four
bands at Q 16 and +12 dB, 200 ms release, a band disabled mid-burst, 0.9 full-scale noise: every
lane at rest 138,752 frames (2.9 s) after the input stops at 48 kHz.

## Units, mappings, automation and smoothing

Band `b` (0 to 3) has ids `b*16 +` 1 enabled (Boolean, on), 2 frequency_hz (log, 200/1k/3k/8k),
3 q (log, 1), 4 gain_db (-24..24, 0), 5 threshold_db (-60..0, -20), 6 ratio (1..20, log, 2),
7 range_db (0..24, 12), 8 attack_ms (0.1..200, log, 5), 9 release_ms (5..2000, log, 100),
10 direction {down 1, up 2}, 11 detector {internal 1, external 2}, 12 knee_db (0..12, 6). All are
block-rate, automatable, per lane, linearly smoothed over 64 samples. One observation tap per band:
its change in dB (resident, per block, per lane, peak magnitude).

## Denormal, signed-zero and NaN policy

SVF integrators flush per word below `1e-20` and jointly after `N_SILENCE` zero input frames; the
change word snaps to `+0.0`. A zero mix word selects the input word itself, so an idle 0 dB or
disabled band passes `-0.0` and every bit. Once per block a lane whose output or state words are
non-finite or at least `1e30` is zeroed for the block, re-seeded from its prepared values and
reported per channel (D7). Parameter values `-0.0` are normalised to `+0.0`.

## Fixtures

Deterministic in-test signals only: unit impulses for the static response, a 1 kHz burst at
-6 dBFS on a 30,000-sample period (1,000,000 samples), a -30 dBFS 1 kHz main tone with the burst
on the sidechain, and xorshift noise for the bank and rest tests.

## Objective tests and tolerances

`tests/oracle.rs`: static mode (range 0), 16 bell rows in four four-band cascades against the
product of `ReferenceParametricEqCoefficients` magnitudes within 0.05 dB (measured `1.5e-5` dB);
dynamic mode against `dsp_reference::ReferenceDynamicEqBand`, downward with a +3 dB static bell and
a 6 dB knee and upward at the +6 dB range clamp: change within 0.1 dB per sample (measured
`2.8e-4` dB), output within `2e-4` (measured `1.2e-6`), attack (90 %) and release (10 %) within
max(1 sample, 2 %) (measured 472/472 and 6645/6645, 251/251 and 6673/6673), a million samples
finite; the external detector: bit-exact before the sidechain burst, within `2e-4` of the
sidechain-driven reference (measured `2.4e-7`). `tests/bank.rs`: each bank lane bit-equal to its
scalar instance under automation (band moves, disables, re-enables, direction and knee flips) and
every link mode; a collapsed then desymmetrized bank bit-equal to a dual one. `tests/rest.rs`. The
conformance macro at four enabled bands. The link modes are the shared step's rows
(`effect-runtime/tests/dynamic_band.rs`): the dynamic EQ links per band with the same function.

## Benchmark plan

None in S1 (not registered, so no production path runs it). A console record of
`isolated_cycles_per_lane_sample` and its class-A floor is a stateless successor (S5).

## Listening protocol or evidence

Not yet: a blinded listening record (folds into #26) is a stateless successor (S5).

## Primary and official sources

- [RBJ-COOKBOOK] the peaking bell and the band-pass identity `H = 1 + (A^2 - 1) B`.
- [SIMPER-SVF] and [ZAVALISHIN-TPT] the TPT SVF, its band output and its word design.
- [REISS-COMP] the static curve with a knee and the branching dB-domain smoother.
- [ORFANIDIS-ISP] ch. 11, the parametric bell as a band-pass added to the direct path.
- U. Zoelzer (ed.), *DAFX*, 2nd ed., Wiley 2011, ch. 4 (dynamics with a band detector). Not yet in
  the bibliography (as the de-esser note).

## Rejected alternatives and tradeoffs

- Redesigning the bell's poles per sample from the dynamic gain: rejected (f64-introduction rule 3;
  a per-sample `tan`/`pow` and a time-varying pole set, Wishnick's concern, for a bandwidth change
  the product does not ask for).
- A second detector filter on the internal path: rejected; `k v1` is the unity-peak band-pass at the
  band's own (static) bandwidth.
- Ramping the knee words linearly: rejected; it breaks the curve's continuity during the ramp.
- Lookahead and a second quality: deferred.

## Known gaps and follow-up

- The certified tail and exact-rest bound of a live, Q-variable SVF (#1372); until then the
  descriptor is `UNBOUNDED`, which #1378 must retire.
- The detector bandwidth is the bell's `Q A` at the static gain, not `Q`; recorded for review.
- Registry and consumer wiring (S2), banking surfaces and console eligibility (S3), dynamic shelves
  (S4), fixtures, plots, benchmark and listening (S5). The observation kind is `GainReductionDb`
  (the only one in V1), whose peak-magnitude fold drops the sign of an upward band's boost.
