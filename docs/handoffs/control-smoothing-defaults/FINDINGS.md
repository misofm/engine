# Default ramp lengths for live mute, fader and pan changes (#1055)

Date: 2026-09-28. Research only, for open issue #1055; it sets the `controlSmoothing` defaults that
#1054 needs. Base: `research-1055` at `ed0556a9` (`codex/batch-slim-1`); evidence commit `7b12cc26`.
No product code, test, gate, script or workflow changed. `measure/` is the harness (a standalone
package with its own `[workspace]`), `data/` the CSVs, `listening/` the blinded packet.
`scripts/check-workspace-policy.sh` and `scripts/check-env-vocabulary.sh` pass with these files.
(The research agent could not write this file; root saved its text, lightly condensed: every
number, table and citation is kept.)

## 1. Answer

| field | default | samples at 44.1 / 48 / 88.2 / 96 kHz | versus the proposal |
|---|---:|---|---|
| `muteMs` (mute and solo) | **10 ms** | 441 / 480 / 882 / 960 | replaces about 5 ms (provisional) |
| `faderMs` | **20 ms** | 882 / 960 / 1764 / 1920 | confirms about 20 ms |
| `panMs` (pan and raw 2x2 matrix records) | **20 ms** | 882 / 960 / 1764 / 1920 | confirms about 20 ms |

**Rounding rule.** `smoothing_samples = floor(ms * sample_rate_hz / 1000 + 1/2)` (round half up),
evaluated in `f64` from the session's `f32` value (`f64::round` agrees for non-negative values).
Every host gets the same integer: the product `ms * rate` is exact in `f64` (24 significant bits
times at most 17), and a non-tie sits at least one `f32` ulp of `ms`, divided by 1000, away from
`k + 1/2`, more than 5,000 times the quotient's `f64` rounding error. Ties do occur at launch rates
(5 ms at 44.1 kHz is exactly 220.5 samples, which becomes 221).

**Bounds for validation.** Finite (the `f32` schema already rejects NaN and infinity); `ms >= 0`
(`-0.0` accepted, means zero, keeps its canonical spelling); `ms <= 1000` per field. Zero stays
legal: it is the explicit hard switch that sample-exact edits and bit-comparison tests need.
1000 ms is 96,000 samples at 96 kHz, inside `u32` and the kernels' exact `f32` countdown (2^24).
Past about one second a ramp is a fade, and a fader lagging half a second is broken, not smooth.

**Mute and fader/pan differ; fader and pan match.** A mute is one full-scale step: its ramp is a
declick, costing only response time, and its click falls about 6 dB per doubling. A fader or pan
move is a stream of small steps at the UI's update rate: its zipper disappears once the ramp is as
long as the update interval, after which a longer ramp only adds lag. Pan artefacts measured at or
below the fader's in every case (5.4).

**Why 10 ms and not 5 ms for mute.** Any ramp of 2 ms or more leaves the click far below what a
dense mix masks (at 5 ms, the mix's high-frequency splatter is 52 dB under its own 1.5-20 kHz
energy). The worst case is an exposed, sustained low note: 10 ms halves the click energy against
5 ms (-77 against -71 dB out of band), which at a bass level near 68 dB SPL moves the click from
about 6 dB above the threshold in quiet to about 2 dB below it. The gain still reaches -20 dB in
9 ms (4.5 ms at 5 ms), inside the 10 ms gesture-to-sound bound [WESSEL-WRIGHT]. At loud monitoring
levels no linear ramp short enough to feel like a mute removes the click on an exposed bass (5.1);
the ramp's shape, not its length, fixes that (5.5).

**Provisional on listening.** `listening/PREREGISTRATION.md` fixes the rules before any trial.
Mute: 5 ms if the owner cannot tell the 5 ms bass mute from the 50 ms one; otherwise 10 ms;
otherwise 20 ms, unless 20 ms audibly softens a rhythmic mute. Fader and pan: stay at 20 ms unless
a 30 Hz drag with 20 ms is audibly stepped; then 35 ms, or a documented requirement that hosts send
updates at 60 Hz or faster.

**For #1054.** Today the browser SDK defaults `smoothingSamples` to 0: the hard switch in row one of
every table below (a mute click 36 dB below the bass, 62 dB above the threshold in quiet). Any
default in the table is a large improvement. The SDK leaves the flush rate to the app
(`sdk/src/core/writer.ts`); flushing drags once per display frame (60 Hz or more) keeps 20 ms at the
continuous-drag floor.

## 2. What official documentation says

Official documentation only. Categories: (a) a ramp applied when a live mute, solo, fader or pan
value changes (what #1055 sets); (b) scene or snapshot fades; (c) automation features; (d)
clip-edge fades. **No DAW or console reference found documents a category (a) time for mute,
solo, fader or pan.**

| source | figure | cat. | what it says (paraphrased) |
|---|---|---|---|
| Ableton Live 12 [LIVE12] | 4 ms | d | Clip-edge fades default to 4 ms, 4 ms crossfades between clips; Session clip fade 0-4 ms; volume automation per sample; constant-power sine pan law; no mute or solo ramp stated. |
| Avid Pro Tools 2025.12.1 [PT2025] | 4 ms; 0-10 ms; 0 ms | a-like, d, c | Fixed 4 ms monitor-only crossfade at record start and stop (p. 810); clip auto-fades 0-10 ms, default 0, about 4 ms suggested; steep-breakpoint smoothing default 0 ms; no mute or solo ramp. |
| Steinberg Cubase/Nuendo 15.0.30 [CUBASE15] | 1-500 ms | d, c | Auto fades 1-500 ms; automation return time should be above 0 to avoid crackles, no number; no mute, solo, fader or pan ramp. |
| Apple Logic Pro [LOGIC] | none | c, d | "Ramp Time" and "Crossfade Time" sliders with no stated values. |
| REAPER 7.80 [REAPER] | none | c | Options to smooth abrupt volume and pan envelope changes and add tiny fades at start/stop, no numbers; no mute or solo fade. |
| DiGiCo SD & Quantum, Issue E [DIGICO] | seconds, frames | b | Snapshot crossfades cover faders and pans; mutes are not crossfaded. |
| SSL Live Help [SSL-LIVE] | 0.5 s steps | b | Levels fade over a set duration; switches such as mutes change at a "Switch At" time. |
| Yamaha CL5/CL3/CL1, DM7 [YAMAHA-CL] [YAMAHA-DM7] | 0-60 s; 0.5 s | b; automixer | Scene fade 0-60 s; the DM7 automixer's MUTE fades in 0.5 s; channel mutes and mute groups state no time. |
| Cycling '74 Max `gain~` [MAX] | 10 ms | a | Gain changes ramp over `interp`, default 10 ms. |
| JUCE [JUCE] | 0; 50 ms | a (source) | `dsp::Gain` does not smooth unless set; panner, dry/wet mixer and DSP demo use 50 ms. Library and example code. |
| nih-plug [NIH-PLUG] | 3, 5, 50 ms | a (examples) | 50 ms logarithmic in gain examples, 3 ms linear in the sine example, 5 ms in the polyphonic synth. |
| Faust `si.smoo` [FAUST] | about 7 Hz one-pole | a | About 22.7 ms time constant, derived from the documented pole. |
| SuperCollider `Lag`, `EnvGate` [SC] | 0.1 s (60 dB); 0.02 s | a, b | `Lag` about a 14.5 ms time constant (derived); `EnvGate` fade 0.02 s. |
| W3C Web Audio API 1.1 [WEBAUDIO] | none | a | Setting `value` is not smoothed; use `setTargetAtTime`. The 2013 draft required GainNode de-zippering, no number. |
| VST 3.8, CLAP, AU [VST3] [CLAP] [AU] | none | a | Hosts deliver changes; smoothing is the plug-in's job; no durations. |
| Puckette [PUCKETTE] | 0-30 ms; 5 ms | a (textbook) | Amplitude changes need 0 (noise) to about 30 ms (sinusoid) to avoid clicks; the muting example uses 5 ms. |
| Wessel and Wright, NIME 2001 [WESSEL-WRIGHT] | 10 ms; 1 ms jitter | latency | Intimate control needs at most about 10 ms latency, about 1 ms variation. |
| Lago and Kon, ICMC 2004 [LAGO-KON] | 20-30 ms | latency | 20-30 ms probably acceptable for most interactive music. |

Not found: Bitwig and Lawo mc² document no mute ramp. Could not fetch (nothing claimed): Studio
One, Allen & Heath dLive/Avantis, Yamaha RIVAGE PM, Midas PRO/HD96. Not verified: IEC 60645-1 and
ANSI S3.6 audiometer rise/fall times (paywalled); Moore's textbook not accessed.

Declick fades cluster at about 4-10 ms (Live, Pro Tools, Max, nih-plug's synth, Puckette);
continuous-control smoothing at about 15-50 ms (Faust, SuperCollider, JUCE, nih-plug gain). Vendor
manuals document interface behaviour only, not DSP evidence [CITATION-POLICY]; hence sections 4-6.

## 3. What the engine's ramps do

- **One law.** `FaderRampStage` (`crates/builtins/src/lib.rs`) and `MatrixStage` retarget from the
  current value to the new target over a fresh window of `N` samples: `step = (target - current) / N`
  once per record, `current += step` before each multiply, the exact target on update `N` (master
  plan D11). Linear in amplitude, not dB.
- **Mute is a fader endpoint.** It retargets the gain to 0 over the same window; after the last
  ramp sample the output is an exact `+0.0` (via `andnot`). Unmute retargets to the stored fader
  gain; a fader move while muted only updates the stored gain. The host composes solo into mute
  (`effective_mute = user_mute || (any_solo && !my_solo)`, `hosts/host-web`), so `muteMs` is also the
  solo ramp.
- **Pan is a 2x2 matrix.** `pan_matrix(l, r)` maps each lane to `(cos, sin)` of `(p + 1) * pi / 4`;
  the ramp interpolates the four coefficients linearly, so power is constant only at the endpoints.
- **Records land at block boundaries** (`drain_fader_controls`, `drain_matrix_controls` in
  `crates/builtins-compiler`): up to one 128-frame quantum (2.9 ms at 44.1 kHz) before a ramp
  starts, whatever `N` is.
- **Banking does not change the arithmetic**; the harness asserts every engine sample equals
  `f32(g * x)` for the engine's own gain trajectory.

## 4. Method

- **Harness.** `measure/` links `crates/builtins` and calls `FaderMuteRampBuiltins::set_fader_db`,
  `set_mute` and `MatrixBuiltins::set_target_smoothed` with `pan_matrix`; 128-frame blocks; records
  at the first block boundary at or after admission; fader before matrix; the section 1 rounding;
  all four launch rates; nothing timed; fixed-seed SplitMix64. The CSVs reproduce byte for byte over
  three runs; stimuli re-render identically; 9 unit tests; clippy clean.
- **Material** (synthesised, `measure/src/material.rs`): `bass`, a sustained A1 55 Hz additive tone
  at -15.4 dBFS RMS with all partials at or below 1 kHz (the worst case, nothing masks the click);
  `kick`, a 150-45 Hz glide with a 350 ms body and a short beater at 120 BPM, transitions 110-190 ms
  into the body; `mix`, a dense 120 BPM stereo arrangement to 12 kHz at -3 dBFS peak, about -19 dBFS
  RMS.
- **Measures.** STFT, periodic Hann, 1024 samples (2048 at 88.2/96 kHz), 75 % overlap, 0-20 kHz,
  relative to the unmodified input. *OOB*: output energy at or above a frequency the source never
  reaches (1.5 kHz bass, 1 kHz kick); floors -94.5 and -101 dB. *CTR*: largest out-of-band
  critical-band energy over the threshold in quiet [PAINTER-SPANIAS eq. (1)], calibrated so 0 dBFS
  RMS plays at 103 dB SPL (K-20 [KATZ]; SMPTE RP 200 would add 2 dB [RP200]); the bass then plays at
  87.6 dB SPL; every 10 dB quieter lowers CTR by 10 dB; about 5 dB per decade of duration
  [HEIL-NEUBAUER] makes the comparison roughly neutral for ~1 ms clicks. *Splatter*: energy a gain
  moves more than half a critical bandwidth away from quarter-bandwidth sub-bands
  [PAINTER-SPANIAS eq. (2)]; *HF splatter* is its 1.5-20 kHz part relative to the programme's.
  *Drags*: fader 0 -> -24 -> 0 dB or pan -0.5 -> +0.5 -> -0.5, 800 ms each way (30 dB/s) or 200 ms
  (120 dB/s), sampled by a UI at 30 or 60 Hz; *ripple* is RMS and peak error against the
  continuous move after best-fit delay; *added lag* is the ramp's share relative to `N = 0`.
- A full masking model (Schroeder spreading, Johnston offsets, BS.1387 NMR [BS1387]) was built and
  dropped: at ~21 ms frames it counted ordinary fades as artefacts.

## 5. Results (48 kHz values; across launch rates within about 1.5 dB, the mix 2.5 dB; timing identical)

### 5.1 Mute click (unmute mirrors mute to within 0.00 dB)

| ramp ms | bass OOB dB | bass CTR dB | kick OOB dB | kick CTR dB | mix splatter dB | mix HF splatter dB | mute to -20 dB, ms | unmute to -3 dB, ms |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | -36.3 | +62.3 | -34.5 | +64.0 | -24.1 | -22.5 | 0.00 | 0.00 |
| 1 | -56.3 | +42.9 | -53.1 | +46.3 | -25.1 | -37.3 | 0.90 | 0.69 |
| 2 | -63.5 | +35.0 | -58.5 | +41.4 | -26.0 | -41.7 | 1.79 | 1.40 |
| 3 | -66.7 | +30.5 | -61.9 | +37.4 | -27.0 | -45.0 | 2.69 | 2.10 |
| 5 | -70.9 | +26.3 | -66.3 | +29.4 | -29.8 | -51.6 | 4.50 | 3.52 |
| 10 | -77.3 | +18.3 | -72.2 | +23.2 | -37.8 | -56.8 | 8.98 | 7.06 |
| 20 | -83.3 | +12.8 | -78.3 | +17.1 | -42.9 | -62.4 | 18.00 | 14.15 |
| 50 | -90.2 | +4.9 | -86.4 | +9.1 | -51.8 | -70.5 | 44.98 | 35.40 |

Events per row: bass 16, kick 12, mix 24. Out-of-band click energy falls about 6 dB per doubling
(the 1/T² law of a linear ramp's two slope corners). At the loud calibration every ramp up to 20 ms
leaves the exposed-bass click above the threshold in quiet (CTR +26 at 5 ms, +18 at 10, +13 at 20;
even 50 ms reaches +5 dB in 6-8 of 16 events). At 20 dB quieter playback (bass near 68 dB SPL):
5 ms +6 dB, 10 ms -2 dB, 20 ms -7 dB, the decisive difference between 5 and 10 ms. In a dense mix
the click is masked from about 2 ms (HF splatter 42 dB below at 2 ms, 52 at 5, 57 at 10; the hard
switch only 22). Responsiveness after the applying block: a linear mute reaches -20 dB at 0.9 `N`
and -40 dB at 0.99 `N`; an unmute reaches -3 dB at 0.71 `N`.

### 5.2 Fader drag

Bass, 30 dB/s moderate drag (floor: OOB -101.3 dB, CTR -5.7 dB):

| ramp ms | 30 Hz OOB / CTR | 30 Hz ripple RMS / peak dB | 30 Hz added lag / settle to 0.5 dB, ms | 60 Hz OOB / CTR | 60 Hz ripple RMS / peak dB | 60 Hz added lag / settle, ms |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | -56.6 / +42.0 | 0.29 / 0.56 | 0 / 0 | -59.5 / +36.4 | 0.15 / 0.30 | 0 / 0 |
| 2 | -83.4 / +13.7 | 0.27 / 0.53 | 1.0 / 1.0 | -86.8 / +7.5 | 0.13 / 0.27 | 1.0 / 0 |
| 5 | -91.8 / +4.6 | 0.25 / 0.48 | 2.5 / 2.4 | -93.6 / -1.5 | 0.10 / 0.22 | 2.5 / 0 |
| 10 | -96.5 / -1.8 | 0.20 / 0.41 | 5.0 / 4.9 | -97.9 / -5.5 | 0.06 / 0.15 | 5.0 / 0 |
| 15 | -97.8 / -4.5 | 0.16 / 0.33 | 7.5 / 7.3 | -99.3 / -5.7 | 0.03 / 0.07 | 7.5 / 0 |
| 20 | -99.0 / -5.7 | 0.12 / 0.26 | 10.0 / 9.7 | -101.3 / -5.7 | 0.02 / 0.04 | 11.6 / 3.8 |
| 30 | -100.3 / -5.7 | 0.04 / 0.11 | 15.0 / 14.6 | -101.2 / -5.8 | 0.03 / 0.06 | 21.5 / 13.1 |
| 40 | -101.3 / -5.9 | 0.03 / 0.07 | 23.2 / 22.9 | -101.3 / -5.9 | 0.06 / 0.10 | 31.4 / 22.4 |
| 50 | -101.2 / -5.9 | 0.06 / 0.11 | 33.1 / 32.2 | -101.2 / -6.0 | 0.09 / 0.16 | 41.2 / 31.8 |

Bass, 120 dB/s fast flick (floor OOB -99.5 dB, CTR -5.1 dB); OOB / CTR, then ripple RMS / peak:

| ramp ms | 30 Hz | 60 Hz |
|---:|---|---|
| 0 | -46.4 / +49.6, 1.16 / 2.19 | -50.2 / +42.5, 0.59 / 1.17 |
| 10 | -87.0 / +7.3, 0.82 / 1.59 | -89.5 / +3.2, 0.26 / 0.57 |
| 20 | -92.5 / -0.3, 0.49 / 0.99 | -98.4 / -4.4, 0.12 / 0.24 |
| 30 | -94.4 / 0.0, 0.24 / 0.45 | -97.0 / -1.0, 0.37 / 0.56 |
| 40 | -98.5 / -5.1, 0.39 / 0.65 | -99.1 / -5.1, 0.74 / 1.13 |

The zipper disappears once `N` is as long as the update interval (33.3 ms at 30 Hz, 16.7 ms at
60 Hz): at 60 Hz, 20 ms is at the continuous-drag floor on every measure (splatter -81.6 dB against
-81.6, ripple 0.02 dB RMS); at 30 Hz, 40 ms is. 0 ms is a real zipper (45 dB above a continuous drag
at 30 Hz, CTR +42 dB). 20 ms at 30 Hz, moderate drag: HF already at the floor; a 0.12 dB RMS
(0.26 dB peak) ripple remains, below amplitude-modulation detection thresholds of about 0.6-1.6 dB
peak to peak for tonal carriers at 32-64 Hz [YOST-SHEFT] and about -23 dB at 30 Hz for noise
[VIEMEISTER, via DESLOGE], so predicted inaudible. 20 ms at 30 Hz, fast flick: 0.49 dB RMS, CTR
at threshold (-0.3 dB), borderline; 30-40 ms removes the HF part. Beyond the update interval a
longer ramp only adds lag (40 ms adds 23 ms at 30 Hz, 31 ms at 60 Hz; tracking error 1.1 dB RMS at
50 ms and 60 Hz on fast moves). The mix shows the same thresholds with more masking (at 20 ms, HF
splatter 86 dB (30 Hz) and 104 dB (60 Hz) below its HF energy; at 0 ms, 47 and 50).

### 5.3 Audible lag

| setting | the ramp's own contribution | also in the path |
|---|---|---|
| mute 5 ms | -20 dB at 4.5 ms | up to one quantum (2.7-2.9 ms at 44.1/48 kHz), UI, output buffer |
| mute 10 ms | -20 dB at 9.0 ms | same |
| fader/pan 20 ms | adds 10-12 ms tracking lag; settles within 0.5 dB 4-17 ms after the hand stops | UI sample-and-hold, about half an update interval |
| fader/pan 30-40 ms at 30 Hz | adds 15-23 ms | same |

### 5.4 Pan (bass in the left lane, -0.5 -> +0.5 -> -0.5 over 800 ms each way)

| ramp | 0 ms (30 Hz / 60 Hz) | 20 ms (30 Hz / 60 Hz) | floor |
|---|---|---|---|
| OOB | -63.3 / -65.9 dB | -97.2 / -97.4 dB | -97.4 dB |
| splatter | -51.4 / -54.3 dB | -72.5 / -91.5 dB | -92.5 dB |
| gain error RMS | 0.10 / 0.05 dB | 0.04 / 0.005 dB | n/a |
| added lag | 0 / 0 ms | 10.0 / 11.6 ms | n/a |

Pan ramps behave like fader ramps with smaller errors. Power is constant only at a pan ramp's ends:
a full-range jump dips total power 3.01 dB at the midpoint whatever the length (15 ms below -1 dB
for a 20 ms ramp); a half-range jump 0.69 dB; drag steps at most 0.02 dB (0.11 dB for fast moves at
50 ms). A property of the ramp's shape, not of the default.

### 5.5 What-if: raised-cosine ramp (not the engine), mute click OOB / CTR dB

| ramp ms | bass, linear (engine) | bass, raised cosine | kick, linear (engine) | kick, raised cosine |
|---:|---:|---:|---:|---:|
| 2 | -63.5 / +35.0 | -76.1 / +19.4 | -58.5 / +41.4 | -67.6 / +28.2 |
| 5 | -70.9 / +26.3 | -91.4 / +5.3 | -66.3 / +29.4 | -84.1 / +12.1 |
| 10 | -77.3 / +18.3 | -97.8 / -4.9 (floor) | -72.2 / +23.2 | -93.2 / +2.7 |
| 20 | -83.3 / +12.8 | -98.5 / -4.9 (floor) | -78.3 / +17.1 | -96.3 / +0.4 |

A raised cosine's continuous end slopes make its spectrum fall 18 dB per octave instead of 12
[SMITH-SASP]: at 5 ms it is as clean as a 50 ms linear ramp, and at 10 ms reaches the floor on the
exposed bass. That is how to fix the exposed-bass case; it is a kernel change with its own
bit-identity and cost questions, for a successor issue, not this default.

### 5.6 Rate independence

At a given length in milliseconds every measure agrees across 44.1, 48, 88.2 and 96 kHz within
about 1.5 dB (the mix 2.5 dB, re-synthesised noise); timing identical. Milliseconds are the right
unit for `controlSmoothing`, and half-sample rounding differences do not matter.

## 6. Listening packet

`listening/`: a blinded two-interval comparison, 68 trials in three blocks, about 15 minutes. Mute
click on bass, kick and mix (5, 10 and 20 ms against 50 ms, plus 2 ms on the mix); fader smoothness
on bass (10 and 20 ms against 40 ms at 30 and 60 Hz, and a fast flick); responsiveness on a
16th-note mute stutter over the mix (10, 20 and 50 ms against 5 ms). It follows
`dsp-research/listening/TEMPLATE.md` and the issue-033 packet (SplitMix64-v1 randomisation,
anonymous 32-hex tokens, balanced interval schedule, a private mode-0600 key committed by hash,
positive controls, exact one-sided binomial tests, decision rules in `PREREGISTRATION.md`). Stimuli
come from the engine's own ramps (`control_smoothing_measure stimuli`); `listening.py self-test`
passes including tamper detection. The owner runs `prepare`, then `run --block M|F|R`, then
`reveal` (`listening/README.md`). **No human has listened; there is no listening result.**

## 7. Verified and not verified

Verified: the ramp law and block-boundary timing (source and unit tests); every rendered case equals
`f32(g * x)` bit for bit; the CSVs reproduce byte for byte over three runs and the stimuli over two;
the SPL calibration; the rounding rule's ties at 44.1 kHz; the listening tool's prepare, validate,
tamper detection and scoring; the workspace-policy and env-vocabulary gates with these files.
Not verified: whether any human hears these differences (packet unrun); real programme material
(the packet accepts `--mix-wav`); banked, AArch64 or wasm renders of the stimuli (per-lane and
unfused, so they should match); the primary texts of Terhardt, Zwicker, Schroeder, Viemeister,
Kohlrausch et al. and Katz's printed JAES paper (restatements read, as marked); console behaviour
documentation does not state.

## 8. Bibliography

Keys follow `dsp-research/CITATION_POLICY.md`; "via" marks a restatement read in place of a primary
text.

- [LIVE12] Ableton, *Live 12 Reference Manual*, §6.8, §8.4.4, §39.3.3, §39.3.8, §39.3.9. https://www.ableton.com/en/live-manual/12/
- [PT2025] Avid, *Pro Tools Reference Guide* 2025.12.1, pp. 148, 160, 810, 929. https://resources.avid.com/SupportFiles/PT/Pro_Tools_Reference_Guide_2025.12.1.pdf
- [CUBASE15] Steinberg, *Cubase Pro / Nuendo 15.0.30 Operation Manual*, "Auto Fades and Crossfades"; Automation Panel "Settings Tab". https://steinberg.help/
- [LOGIC] Apple, *Logic Pro User Guide for Mac*, Automation and Audio Editing settings. https://support.apple.com/guide/logicpro/
- [REAPER] Cockos, *REAPER User Guide* v7.80, pp. 33, 430-443. https://www.reaper.fm/userguide/ReaperUserGuide780.pdf
- [DIGICO] DiGiCo, *SD & Quantum Software Reference*, Issue E, V1528 (2022), PDF p. 100. https://digico.biz/wp-content/uploads/2022/03/SD-Quantum-Software-Reference-Issue-E-V1528.pdf
- [SSL-LIVE] Solid State Logic, *SSL Live Help*, Scene Time Functions. https://livehelp.solidstatelogic.com/Help/AutoTime.html
- [YAMAHA-CL] Yamaha, *CL5/CL3/CL1 Reference Manual* (cl5_en_rm_c0), p. 92.
- [YAMAHA-DM7] Yamaha, *DM7 Reference Manual* (DM7_RM_En_D1), pp. 74, 215, 359.
- [MAX] Cycling '74, Max reference, `gain~`, `live.gain~`. https://docs.cycling74.com/reference/gain~/
- [JUCE] JUCE `dsp::Gain`; `juce_Panner.cpp`, `juce_DryWetMixer.h`, `DSPModulePluginDemo.h`. https://docs.juce.com/master/classjuce_1_1dsp_1_1Gain.html
- [NIH-PLUG] R. van der Helm, nih-plug, `SmoothingStyle` and `plugins/examples`. https://nih-plug.robbertvanderhelm.nl/
- [FAUST] GRAME, Faust `signals.lib` v1.7.0, `si.smoo`, `si.smooth`. https://faustlibraries.grame.fr/libs/signals/
- [SC] SuperCollider 3.14.1 help, `Lag`, `EnvGate`. https://docs.supercollider.online/
- [WEBAUDIO] W3C, *Web Audio API 1.1*, WD 22 Sep 2026 §7.3; WD 2013-10-10 §1.6. https://www.w3.org/TR/webaudio-1.1/
- [VST3] Steinberg VST 3.8 SDK, `IParamValueQueue`; "Parameters and Automation". https://steinbergmedia.github.io/vst3_doc/
- [CLAP] free-audio CLAP, `include/clap/events.h`, `params.h`. https://github.com/free-audio/clap
- [AU] Apple AudioToolbox, `AUParameterEvent.rampDurationSampleFrames`. https://developer.apple.com/documentation/audiotoolbox/auparameterevent/rampdurationsampleframes
- [PUCKETTE] M. Puckette, *The Theory and Techniques of Electronic Music*, "Synthesizing a sinusoid", "Muting". http://msp.ucsd.edu/techniques/latest/book-html/
- [SMITH-SASP] J. O. Smith III, *Spectral Audio Signal Processing*, "Rectangular Window Side Lobes"; "Hann or Hanning or Raised Cosine". https://ccrma.stanford.edu/~jos/sasp/
- [WESSEL-WRIGHT] D. Wessel and M. Wright, "Problems and Prospects for Intimate Musical Control of Computers", NIME 2001, p. 2. https://www.nime.org/proceedings/2001/nime2001_011.pdf
- [LAGO-KON] N. P. Lago and F. Kon, "The Quest for Low Latency", ICMC 2004. https://www.ime.usp.br/~kon/papers/icmc04-latency.pdf
- [PAINTER-SPANIAS] T. Painter and A. Spanias, "Perceptual coding of digital audio", Proc. IEEE 88(4):451-515, 2000, doi:10.1109/5.842996; eq. (1) after Terhardt, Hearing Research 1:155-182, 1979 (not opened); eq. (2) after Zwicker and Fastl.
- [JOHNSTON] J. D. Johnston, IEEE JSAC 6(2):314-323, 1988, doi:10.1109/49.608, Table II, after Zwicker 1961 (not opened).
- [KATZ] B. Katz, "Level Practices, Part 2", updated from J. Audio Eng. Soc. 48(9):800-809, 2000. https://www.digido.com/portfolio-item/level-practices-part-2/ (printed paper not opened).
- [RP200] SMPTE RP 200:2012, reference level 85 dBC.
- [HEIL-NEUBAUER] P. Heil and H. Neubauer, PNAS 100(10):6151-6156, 2003, doi:10.1073/pnas.1030017100 (via a summarising fetch).
- [YOST-SHEFT] W. A. Yost and S. Sheft, Auditory Neuroscience 3(4):401-414, 1997, Figs. 1-2 (read from figures, about ±1 dB).
- [VIEMEISTER] N. F. Viemeister, J. Acoust. Soc. Am. 66(5):1364-1380, 1979, doi:10.1121/1.383531 (abstract only).
- [DESLOGE] J. G. Desloge et al., J. Acoust. Soc. Am. 129(6):3884-3896, 2011, doi:10.1121/1.3583550 (via a summarising fetch).
- [BS1387] ITU-R BS.1387-1 (2001), §4.2.
- [CITATION-POLICY] `dsp-research/CITATION_POLICY.md`.
