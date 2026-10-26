# Loudness measurement

## Scope and engineering question

How does the engine measure BS.1770-5 loudness and true peak of the master, for every session rate,
with a realtime half that is allocation-free and a statistics half whose memory does not grow with
programme length? Issue #2115 slice S1 answers this for one boundary: the session's rendered output
pair (the master), stereo L/R. The meter observes PCM and never changes it.

This note does not decide: loudness of a submix or of any other boundary, multichannel layouts
(5.0, 5.1 and more), loudness-driven gain (normalisation or a loudness limiter), the transport and
display format of readings, or file-based conformance with the BS.2217 material. Each is a successor
(see "Known gaps and follow-up").

## Adopted decisions

1. **Boundary.** One meter on the master output pair, channel weights `G_L = G_R = 1` [ITU-BS1770-5].
   Measurable reason: PCM out of the master is bit-identical with and without the meter.
2. **Two halves.** The render half filters, squares and sums into fixed 10 ms hops and emits one
   record per hop. The control half, off the render thread, turns hop records into M, S, I, LRA and
   true-peak readings. Measurable reason: render allocation count is exactly zero.
3. **10 ms hop.** A hop is `fs / 100` samples: 441, 480, 882 and 960 at 44.1, 48, 88.2 and 96 kHz,
   all exact. Source fact: Tech 3341 cases 13 and 14 place a 400 ms burst at offsets `i * 20 ms`
   and require Max M within 0.1 LU [EBU-TECH3341]. A 100 ms grid can catch only 350 ms of such a
   burst, `10 log10(350/400) = -0.58 LU`, which fails. Every Tech 3341/3342 recipe duration is a
   multiple of 10 ms, so the 10 ms grid aligns with each of them. On arbitrary programme a window
   misses at most 5 ms of a burst: `-0.055 LU` for a 400 ms burst and `-0.007 LU` for a 3 s burst.
4. **K-weighting at the session rate.** Two `f64` biquads per channel, designed by the bilinear
   transform from one analog prototype at each launch rate (44.1, 48, 88.2, 96 kHz). Every other
   rate is refused, as every host path already refuses it. Measurable reason: at 48 kHz the design
   reproduces the BS.1770-5 coefficient table (gate `1e-6` per coefficient).
5. **Stage-2 gain kept at its 48 kHz value at every rate** (see "Coefficients and update rules").
   Measurable reason: one analog signal reads the same at every rate, within the response test.
6. **True peak** uses the limiter's exact Annex-2 detector (`true_peak_limiter::annex2`: four
   phases, 48 taps, `f32`), with L and R as two lanes of one native-width bank plus padding lanes,
   and a per-hop maximum per channel [ITU-BS1770-5]. Measurable reason: Tech 3341 cases 15-23 within
   `+0.2/-0.4 dBTP` and agreement with the `f64` Annex-2 reference.
7. **Bounded statistics.** Integrated loudness and LRA use 0.1 LU histograms over `[-70, +10)` LUFS,
   each bin a `u64` count and an `f64` energy sum, about 26 KB in all for any programme length.
   Measurable reason: oracle tests against exact `f64` block lists (`|dI| <= 0.05 LU`,
   `|dLRA| <= 0.1 LU`).
8. **Measurement runs on silence.** Skip-on-silence does not apply to a measurement: silent hops are
   real data for gating and for M/S. A zero-energy window has no finite loudness and is reported as
   absent, never as the logarithm of zero.

## Definitions and assumptions

- `fs`: session sample rate, one of 44 100, 48 000, 88 200, 96 000 Hz.
- `x_c[n]`: master output sample of channel `c` in `{L, R}`, `f32`, after the master chain.
- `y_c[n]`: K-weighted sample, `f64`.
- **Hop:** `H = fs / 100` contiguous samples. A hop record holds its first sample index, its end
  sample index (exclusive), energy `E_c = sum y_c[n]^2` for each channel (`f64`), true peak
  `P_c = max |tp_c|` for each channel (`f32`, linear), and the count of sanitized non-finite samples.
- **M (momentary):** loudness of the last 400 ms (40 hops). **S (short-term):** loudness of the
  last 3 s (300 hops) [EBU-TECH3341]. Max M and Max S are the maxima since reset.
- **I (integrated):** BS.1770 gated loudness from the reset point [ITU-BS1770-5].
- **LRA:** Tech 3342 loudness range from the reset point [EBU-TECH3342].
- **Gating block:** 400 ms (I) or 3 s (LRA) window, started every 100 ms (10 hops) on a grid that
  begins at the reset point.
- **Gap:** a hop record whose first sample is not the previous record's end sample.
- The two channels are a stereo pair for the sum; this meter is not a per-strip dual-mono meter.
  The per-channel L/R true peaks are reported separately.

## Algorithm and equations

Per channel, per sample (render half):

1. Sanitize: a non-finite `x_c[n]` becomes 0 and increments the hop's sanitized count.
2. K-weight: `y_c = H2(H1(x_c))`, two direct-form biquads in `f64`.
3. Accumulate `E_c += y_c^2` in `f64` into the open hop.
4. True peak: feed `x_c[n]` to the Annex-2 four-phase interpolator; `P_c = max(P_c, |phase outputs|)`.
5. At the hop's end, emit the record and open the next hop with `E_c = 0`, `P_c = 0`.

Analog prototypes [ITU-BS1770-5] [DEMAN-R128-2018]:

- Stage 1, high shelf: `H1(s) = (Vh s^2 + (Vb/Q1) s + 1) / (s^2 + s/Q1 + 1)`, with `s` normalized
  to `2 pi f1`.
- Stage 2, high-pass: `H2(s) = g48 s^2 / (s^2 + s/Q2 + 1)`, with `s` normalized to `2 pi f2`.

Loudness of a window `W` of `N` samples (control half), as BS.1770 defines it with `G_c = 1`:

`L(W) = -0.691 + 10 log10( (E_L(W) + E_R(W)) / N )` LUFS,

where `E_c(W)` is the sum of the hop energies in `W`. M uses the last 40 hops, S the last 300 hops;
both update at each hop.

Integrated loudness [ITU-BS1770-5]: for each gating block `j` with mean square
`z_j = (E_L + E_R) / N_block` and level `l_j = -0.691 + 10 log10 z_j`:

- absolute gate: keep blocks with `l_j > -70 LUFS` (strict);
- `Gamma_r = -0.691 + 10 log10(mean of z_j over the absolute-gated blocks) - 10 LU`;
- `I = -0.691 + 10 log10(mean of z_j over blocks with l_j > -70 and l_j > Gamma_r)`.

Each admitted block goes into the bin `floor((l_j + 70) / 0.1)` of 800 bins; the bin adds 1 to its
count and `z_j` to its energy sum. A block at or above +10 LUFS goes into the top bin with its exact
`z_j`. The relative gate is applied to whole bins: the bin that straddles `Gamma_r` is included or
excluded as a unit, and the error that this causes is bounded by the oracle test.

Loudness range [EBU-TECH3342]: 3 s blocks every 100 ms; absolute gate -70 LUFS; relative gate
20 LU below the power mean of the absolute-gated block levels; LRA is `P95 - P10` of the gated
levels, where percentile `p` is the element of rank `round((n - 1) p / 100 + 1)` in the sorted list
of `n` values (Tech 3342's rank rule). On the histogram, the rank walks the cumulative bin counts;
the bin that holds the rank gives its mean level, `-0.691 + 10 log10(energy sum / count)`.

True peak since reset: `TP_c = max over hops of P_c`, reported as `20 log10(TP_c)` dBTP on the
control plane only.

## Coefficients and update rules

Coefficients are computed once at plan preparation, in `f64`, from the session rate. They never
change while the plan runs; a rate change is a new plan. Prewarp: `K = tan(pi f0 / fs)`.

Stage 1 (high shelf): `f1 = 1681.974450955533 Hz`, `G = 3.999843853973347 dB`,
`Q1 = 0.7071752369554196`, `Vh = 10^(G/20)`, `Vb = Vh^0.4996667741545416`,
`a0 = 1 + K/Q1 + K^2`, and

- `b0 = (Vh + Vb K/Q1 + K^2) / a0`, `b1 = 2 (K^2 - Vh) / a0`, `b2 = (Vh - Vb K/Q1 + K^2) / a0`,
- `a1 = 2 (K^2 - 1) / a0`, `a2 = (1 - K/Q1 + K^2) / a0`.

Stage 2 (high-pass): `f2 = 38.13547087602444 Hz`, `Q2 = 0.5003270373238773`, `a0(fs) =
1 + K/Q2 + K^2`, `a1`, `a2` as for stage 1 with this `K` and `Q2`, and
`b0, b1, b2 = (g48 / a0(fs)) * (1, -2, 1)`.

Stage-2 gain decision. The BS.1770-5 48 kHz table gives stage 2 the numerator exactly `(1, -2, 1)`.
After the bilinear transform that numerator means a passband gain `g48 = a0(48 kHz) =
1.0049948987146884` (+0.0433 dB), and the -0.691 offset assumes this gain. The engine keeps `g48`
at every rate, so one analog signal reads the same at every rate. An unnormalized `(1, -2, 1)` at
every rate gives a passband gain of `a0(fs)`: +0.0471 dB at 44.1 kHz, +0.0433 dB at 48 kHz,
+0.0236 dB at 88.2 kHz and +0.0217 dB at 96 kHz, a 0.025 dB spread across the launch rates.

Parameter provenance. De Man reverse-engineered analog parameters of this form from the 48 kHz
table [DEMAN-R128-2018]; his published code carries the same form with slightly shorter digits,
which reproduce the table to about `1e-12`. The digits above reproduce the 48 kHz BS.1770-5 table
to about `1e-15` per coefficient (stage-1 maximum error `8.9e-16`, stage-2 poles `2.2e-16`, checked
2026-10-26). The table match, not the paper, is the evidence.

Update timing. M and S update at each hop. A gating block closes every 10 hops once 40 (I) or 300
(LRA) contiguous hops exist since the reset point or the last gap. Max M, Max S and the true-peak
maxima update at each hop.

## Numerical and stability limits

- Both biquads are stable at every launch rate. Stage 2 has the slowest poles: radius 0.99751 at
  96 kHz. Filter state and energy sums are `f64`; the filters run on finite input only.
- Hop energy is an `f64` sum of at most 960 terms. Bin energy sums and counts are `f64` and `u64`,
  so programme length does not limit them in practice.
- Histogram resolution: 0.1 LU bins. The relative-gate straddle error is bounded by the oracle test
  at `|dI| <= 0.05 LU`. A percentile reads its bin's mean level, so it is within half a bin of the
  exact block value, and LRA is bounded at `|dLRA| <= 0.1 LU` by the oracle test.
- A block level at or below -70 LUFS is gated out and not stored; a block level at or above
  +10 LUFS is in the top bin with exact energy, so its energy contribution to I stays exact.
- A window with zero energy has no finite loudness: the control half reports it as absent (the
  transport may floor it); it never computes `log10(0)`.
- Memory: two histograms of 800 bins at 16 bytes each (about 26 KB) plus the fixed hop windows for
  M, S and block assembly. Memory does not depend on programme length.
- The 10 ms grid can miss up to 5 ms of a burst on arbitrary programme (see decision 3).

## Latency and tail

Audio latency is zero and the audio tail is zero: the meter does not change PCM and reports no
latency to PDC. Bypass is not applicable; the meter is not in the signal path.

Reading latency: a hop is visible to the control half only after it closes (one hop, 10 ms, plus
the host's service cadence). M needs 400 ms and S 3 s of contiguous hops to cover a full window.
The K-filter and interpolator states continue across hops, so filter ringing is counted in the hop
in which it happens.

Reset (control plane only): the control half drops hops before the reset sample, starts the gating
grid at the first hop boundary at or after it, and clears both histograms, true peak since reset,
Max M and Max S. M and S keep running across a reset; render state is not reset. A gap restarts
the M/S windows and the gating-block assembly.

## Units, mappings, automation and smoothing

- M, S, I, Max M, Max S: LUFS (-23 LUFS is 0 LU on the EBU relative scale; the relative scale is
  a display mapping for apps) [EBU-R128] [EBU-TECH3341].
- LRA: LU [EBU-TECH3342].
- True peak: dBTP, `20 log10` of a linear `f32` peak, computed on the control plane only.
- Nothing is automatable: the meter has no live parameters. It has no smoothing; display ballistics
  belong to apps and do not change the measured values.

## Denormal, signed-zero and NaN policy

- Input: a non-finite sample (NaN, +inf, -inf) becomes 0 before filtering and increments the hop's
  sanitized count. The count is in each hop record.
- State: the filters run in `f64` under the engine's canonical floating-point environment (no
  flush-to-zero, no denormals-are-zero). Subnormal values are finite and take part as they are.
- Signed zero: the sign of zero does not affect a square or an absolute value.
- Output: all readings are finite or absent. A zero-energy window is absent, not `-inf`.
- Reset: control-plane statistics only; render filter state is not cleared by a reset.

## Fixtures

All fixtures are generated in `crates/dsp-reference` from the recipes of the cited documents:

1. The BS.1770-5 48 kHz K-weighting table [ITU-BS1770-5].
2. The analog prototypes `H1`, `H2` evaluated at test frequencies for each launch rate.
3. Tech 3341 cases 1-5 and 9-14 at all four launch rates [EBU-TECH3341].
4. Tech 3342 cases 1-4 [EBU-TECH3342].
5. Tech 3341 true-peak cases 15-23 [EBU-TECH3341].
6. Stepped noise for the histogram-versus-oracle tests.
7. Non-finite injections, a reset inside a programme and a record gap.

Excluded: Tech 3341 case 6 (5.0 channel layout; this meter measures stereo), Tech 3341 cases 7-8
and Tech 3342 cases 5-6 (authentic programme files). File-based conformance with the BS.2217
material [ITU-BS2217] is a successor.

## Objective tests and tolerances

As implemented in the `crates/loudness` tests, with fixtures from `crates/dsp-reference`:

1. 48 kHz coefficients equal the BS.1770-5 table within `1e-6` per coefficient.
2. Digital response matches the analog prototype at every launch rate.
3. Tech 3341 cases 1-5 and 9-14 at all four rates: within `+-0.1 LU` (LUFS) of the expected value.
4. Tech 3342 cases 1-4: within `+-1 LU`.
5. True-peak cases 15-23: within `+0.2/-0.4 dBTP`, and agreement with the `f64` Annex-2 reference.
6. Histogram I and LRA against an exact `f64` oracle on stepped noise: `|dI| <= 0.05 LU`,
   `|dLRA| <= 0.1 LU`.
7. Zero render allocations; the sanitized count equals the number of injected non-finite samples;
   the reset boundary; the gap restart.

Measured values for each conformance case: see #2115 spec evidence.

## Benchmark plan

None in S1. The cost is about two `f64` biquads per channel plus the 48 multiply-add Annex-2 FIR
per sample on a native lane bank, with no PCM change. A descriptive benchmark, under the AGENTS.md
protocol (frozen workload, one warmup, two measured rounds), is later work.

## Listening protocol or evidence

N/A — the meter is measurement only and leaves PCM unchanged; its correctness is numerical and the
objective tests cover it.

## Primary and official sources

- [ITU-BS1770-5]: K-weighting table, channel sum, -0.691 offset, 400 ms gating blocks with 75 %
  overlap, absolute and relative gates, Annex-2 true-peak method.
- [EBU-TECH3341]: M (400 ms), S (3 s), I and true-peak conformance cases 1-23 and tolerances.
- [EBU-TECH3342]: LRA definition, gates, percentile rank rule and cases 1-6.
- [EBU-R128]: target level and the LU relative scale.
- [ITU-BS2217]: the BS.1770 compliance file material (successor scope).
- [DEMAN-R128-2018]: provenance of the analog prototype form that reproduces the 48 kHz table
  (paywalled; the table match is the evidence).

## Rejected alternatives and tradeoffs

- **100 ms hop.** Fails Tech 3341 cases 13/14: up to -0.58 LU on a 400 ms burst.
- **`f32` filter state.** The stage-2 poles sit at radius 0.99751 at 96 kHz; the engine's precision
  study ranks low-frequency recursive filters as an `f64` candidate and names measurement output
  as a place where `f32` floors matter (`docs/research/f64-introduction.md`, rank 4 and the
  "Deep notch / measurement output" row). Its rank 1 requires `f64` sums and `u64` counts for
  unbounded statistics, which the histograms follow.
- **Store every gating block.** Exact, but memory grows with programme length.
- **A per-strip meter-metric bit.** Puts K-filters and the FIR on every strip; S1 measures the
  master only.
- **Unnormalized stage 2 `(1, -2, 1)` at every rate** (as some implementations do). Moves the
  reading by up to 0.025 dB between launch rates.

## Known gaps and follow-up

- Loudness at a submix or any other boundary, and multichannel layouts with BS.1770 channel weights.
- File-based conformance with the BS.2217 files and the Tech 3341/3342 authentic programmes.
- Descriptive benchmark of the render half.
- Absolute gate comparison for LRA: BS.1770 writes a strict `>` for I; Tech 3342's MATLAB writes
  `>=`. The difference matters only for a block at exactly -70 LUFS; the crate's choice is recorded
  in the #2115 spec evidence.

## 17. Decision record

Fact: BS.1770-5 defines K-weighting, gating and true peak [ITU-BS1770-5]; Tech 3341 and 3342
define M, S, LRA and conformance cases [EBU-TECH3341] [EBU-TECH3342]. Adoption (#2115 S1, root
decisions): a master-only stereo meter, render half with 10 ms hops, `f64` K-weighting designed at
the session rate with the 48 kHz stage-2 gain at every rate, the limiter's Annex-2 detector,
control half with bounded 0.1 LU histograms. Measurable reasons: PCM equality with and without the
meter; zero render allocations; the 48 kHz table within `1e-6`; Tech 3341/3342 cases within their
tolerances at every launch rate; histogram oracle bounds. Open: the successors in "Known gaps and
follow-up".
