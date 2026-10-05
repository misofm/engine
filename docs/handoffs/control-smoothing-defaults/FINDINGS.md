# Default ramp lengths for live mute, fader and pan changes (#1055)

Dates: 2026-09-28 (sections 1-8) and 2026-10-05 (section 9, every live row of decision 15).
Research only, for #1055; it sets the `controlSmoothing` defaults #1054 ships. Sections 1-8: base
`ed0556a9`, evidence `7b12cc26`; section 9: base `main` at `8be19c86e`. No product code, test,
gate, script or workflow changed. `measure/` is the harness (a standalone package with its own
`[workspace]`), `data/` the CSVs, `listening/` the blinded packet. The workspace-policy and
env-vocabulary scripts pass with these files. (Sections 1-8: root saved the research agent's
text, lightly condensed; every number, table and citation is kept.)

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
These are the measured defaults; *Run the blinded listening session for the live ramp defaults*
(#1388) runs the preregistered session on them, and its result changes default values only, never
a key or a rule (decision 15, D15-1). Section 9 gives every other live row's key and default.

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
  three runs; stimuli re-render identically; 9 unit tests (section 9 adds 8); clippy clean.
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
The defaults are the measured ones; #1388 runs this preregistered session on them, and a listening
result changes values only (D15-1). Section 9 adds no contrast, for the reasons in 9.6.

## 7. Verified and not verified

Verified: the ramp law and block-boundary timing (source and unit tests); every rendered case equals
`f32(g * x)` bit for bit; the CSVs reproduce byte for byte over three runs and the stimuli over two;
the SPL calibration; the rounding rule's ties at 44.1 kHz; the listening tool's prepare, validate,
tamper detection and scoring; the workspace-policy and env-vocabulary gates with these files.
Not verified: whether any human hears these differences (packet unrun); real programme material
(the packet accepts `--mix-wav`); banked, AArch64 or wasm renders of the stimuli (per-lane and
unfused, so they should match); the primary texts of Terhardt, Zwicker, Schroeder, Viemeister,
Kohlrausch et al. and Katz's printed JAES paper (restatements read, as marked); console behaviour
documentation does not state. Section 9's verified and open items are in 9.7.

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

## 9. Every live row (decision 15)

Date 2026-10-05; base `main` at `8be19c86e`; #1055's decision-15 addition (attempt 2 corrects
9.2's parity statement and adds the 40 ms flip). Decision 15 ramps every live value over the
session's `controlSmoothing` (D15-1, D15-13 E5). This section gives each row's kernel, ramp law,
key and default. **The defaults are the measured ones**: sections 5-6
and this section set them; *Run the blinded listening session for the live ramp defaults* (#1388)
runs the preregistered session (`listening/`) on the shipped table; a listening result changes
default values only, never a key or a rule (D15-1). Every `path:line` was read on `8be19c86e`. No
file named by the issue's own anchors changed since `6fb211594`, and each still reads as the issue
quotes it (`crates/builtins/src/lib.rs:1434-1478` and `:1436-1439`, `crates/graph/src/runtime.rs:861`,
`crates/lane/src/kernels.rs:1073-1168`, `crates/host-core/src/vca.rs:1-13`,
`crates/effect-contract/src/live.rs:841-843`).

### 9.1 The table (D1)

Abbreviations: `B` = `crates/builtins/src/lib.rs`, `K` = `crates/lane/src/kernels/builtins.rs`,
`W` = `hosts/host-web/src/lib.rs`. D11 = linear in amplitude from the current value over a fresh
window of `N` samples, one division per record, the exact target on update `N` (section 3).

| row | kernel anchor | ramp law | key | default ms | evidence |
|---|---|---|---|---:|---|
| fader | `FaderRampStage::retarget`, `B:2730-2750`; `gain_mute_ramp_block`, `K:159-202` | D11 | `faderMs` | 20 | `fader_drag.csv` (5.2) |
| mute | `FaderRampStage::set_mute`, `B:2773-2794`; the kernel's `andnot` clear, `K:159-178` | D11 to `+0.0` | `muteMs` | 10 | `mute_click.csv`, `mute_timing.csv` (5.1) |
| solo | `LiveControlSoloState::effective_mute`, `crates/host-core/src/solo.rs:255-263`: solo reaches render as the strip's mute record | the mute's | `muteMs` | 10 | source: it is a mute record |
| pan | `pan_matrix`, `B:4363`; `MatrixBuiltins::set_target_smoothed`, `B:4345`; `matrix2x2_ramp_block`, `K:422-470` | D11 on the four coefficients | `panMs` | 20 | `pan_drag.csv`, `pan_jump.csv` (5.4) |
| raw matrix | `MatrixStage::set_target_over`, `B:3008-3049` | the pan's (same stage) | `panMs` | 20 | source: the stage and law pan was measured on; a coefficient through zero is 9.2's flip (below) |
| input trim | `InputStage::set_trim_db` → `set_trim_signed`, `B:1480-1500`, `B:1434-1478`; `input_chain_ramp_block`, `K:618-726` | D11, bit-identical to the fader | `faderMs` | 20 | `law_transfer.csv` (`input-trim`: difference 0) and `fader_drag.csv` (9.3) |
| polarity invert | `InputStage::set_polarity_invert`, `B:1502-1519`: `set_trim_signed` carries the coefficient through zero (`B:1436-1439`) | D11 from +1 to -1 or back | `muteMs` **x 2** | 20 | `polarity_click.csv` (9.2) |
| send gain | `LiveRoute` drain and mix, `crates/graph/src/runtime.rs:861-931`; `IndexedRamp`, `crates/lane/src/kernels.rs:1073-1168`; `route_mix_ramp_block`, `:1177-1263` | indexed; equals D11 within D11's rounding | `faderMs` | 20 | `law_transfer.csv` (`route-indexed`) and `fader_drag.csv` (9.3) |
| send mute | the same route ramp to `[+0.0; 4]` (`RouteControlRecord`, `crates/graph/src/lib.rs:922-967`) | indexed | `muteMs` | 10 | `law_transfer.csv` and `mute_click.csv` (9.3) |
| send matrix | the same route ramp, `runtime.rs:861-931`, its four coefficients from `gated_route_coefficients`, `crates/graph/src/lib.rs:785-812` | indexed | `panMs` | 20 | `law_transfer.csv` and `pan_drag.csv` (9.3); a coefficient through zero is 9.2's flip (below) |
| send `follows_mute` toggle | the follow record, `W:5274-5364` (its ramp is the strip mute record's); `RouteControlProducer::record`, `crates/host-core/src/route_controls.rs:82-107`; `RouteGate`, `gated_route_coefficients`, `crates/graph/src/lib.rs:759-812` | the send mute's: a source column to or from `+0.0` | `muteMs` | 10 | source (9.3) |
| VCA offset | `crates/host-core/src/vca.rs:1-13`; the VCA fader pass stages member `FaderDb` records, `W:5179-5214` | the member fader's (D11) | `faderMs` | 20 | source (9.3) and `fader_drag.csv` |
| VCA mute | `vca.rs:1-13`; `LiveControlSoloState::set_vca_mute`, `solo.rs:311`; member `Mute` records, `W:5215-5273` | the member mute's (D11) | `muteMs` | 10 | source (9.3) and `mute_click.csv` |
| effect bypass crossfade | today a whole-block select, `crates/effect-contract/src/live.rs:841-843` (`BypassShunt`, `:822-990`); #1341 D2-D3 | indexed crossfade `dry + (wet - dry) m(k)`, emulated (`measure/src/crossfade.rs`) | `muteMs` | 10 | `bypass_crossfade.csv` (9.4) |
| detector link glide | compressor `link_frame`, `crates/compressor/src/kernel.rs:317-340`, `curve_target`, `:349-361`, `ballistic`, `:364-369`; #1370 D2-D3 | linear ramp of the detector weights, emulated as an output crossfade | `faderMs` | 20 | `link_glide.csv` (9.5) |

Samples at 44.1 / 48 / 88.2 / 96 kHz: 10 ms is 441 / 480 / 882 / 960 and 20 ms is 882 / 960 /
1764 / 1920 (section 1's rounding). The polarity row is twice the rounded mute length,
`2 * mute_samples` (882 / 960 / 1764 / 1920 at the default), so its per-sample step, `2 / (2N)`,
is exactly the mute's, `1 / N`. Everything else matches #1054 D3 and #1341; the one difference is
9.8.

**Matrix coefficients through zero.** Pan's coefficients stay in [0, 1], so a pan never reaches
the polarity case. A raw matrix coefficient may be any value in [-1, 1] (`Matrix2x2::checked`,
`B:88-102`, called by `set_target_over` at `B:3019`), and a send matrix coefficient any finite,
non-subnormal value (`route_transform`, `crates/graph-compiler/src/ids.rs:288-304`, behind
`route_coefficients`, `:341`). A matrix record that carries a coefficient from `+a` to `-a` is
therefore a polarity flip of that path, 9.2's case, on the `panMs` ramp: at the defaults `panMs`
(20 ms) is twice `muteMs` (10 ms), so the flip clicks like the 10 ms mute (9.2); a session that
sets `panMs` below twice `muteMs` gets a louder one (at `panMs` equal to `muteMs`, 6 dB more: the
click of a mute half as long). That case can also be the defaults themselves: one preregistered
#1388 outcome (`muteMs` 20 ms with `panMs` 20 or 35 ms) puts `panMs` below twice `muteMs`, so a
matrix coefficient flip at those defaults clicks louder than the mute. This is the same argument
9.2 makes against `faderMs` as the polarity key; this record does not add a rule for it, and root
weighs it with the #1388 result (9.2, 9.8). The two rows keep `panMs`, because a row's length is
per row, not per value (#1054 D4).

### 9.2 Polarity invert (question 4): `muteMs` is the key, but twice its length

**The law.** The flip retargets the signed trim from `+1` to `-1` (or back) over `N` samples. Its
gain is exactly `2 g_mute - 1`, where `g_mute` is a mute's ramp over the same `N`. Two facts follow,
both exact for the linear law (`T = N / fs`):

1. *At the same length the flip is the mute plus 6.02 dB.* A constant gain moves nothing, so the
   flip spreads twice the mute's amplitude. Measured over every length, rate, material and
   transition: +6.02 to +6.03 dB on total splatter, +6.00 to +6.03 dB on HF splatter, +5.45 to
   +6.17 dB on OOB, worst OOB and CTR (these read the output, which also holds the source's own floor and frame leakage).
2. *A flip over `2N` is two mutes over `N` back to back.* Its slope, `2 / (2N)`, is the mute's
   `1 / N`, held twice as long: `+1` to `0`, then at once `0` to `-1`. So its gain spectrum is the
   `N`-mute's times `1 + e^(-j 2 pi f T)`, of magnitude `|2 cos(pi f T)|`: the flip's
   `|sin(2 pi f T)| / (2 pi^2 f^2 T)` against the mute's `|sin(pi f T)| / (2 pi^2 f^2 T)`
   [SMITH-SASP].
   - *Far from the source* (`f T` well above 1) the two share one envelope, set by the slope, and
     differ only in where their lobes fall, so over a band many lobes wide their energies are
     equal: the two slope corners set the click, and they are the mute's. OOB and worst OOB (at or
     above 1.5 kHz on the bass, whose partials stop at 1 kHz; 1 kHz on the kick), CTR (the critical
     bands above that edge) and HF splatter (1.5-20 kHz) read there. On the mix, HF splatter also
     holds near-source spreading of the mix's own high partials (half a critical bandwidth is
     113 Hz at 1.5 kHz), so it shows the largest of these differences.
   - *Near the source* (offsets below about `1 / T`) the factor falls from 2 (+6 dB: the two halves
     change the gain the same way, a total change of 2) to a null at `1 / (2T)` and returns to 2 at
     `1 / T`. Total splatter counts all that the gain moves more than half a critical bandwidth
     from its source sub-band: from 56-82 Hz off each of the bass's partials below 600 Hz (up to
     112 Hz near 1 kHz). At these lengths that is this region (`1 / T` is 100 Hz at 10 ms and 50 Hz
     at 20 ms), so the slope corners do not set it.

**Measured** (`polarity_click.csv`, the mute method: the same 16 bass, 12 kick and 24 mix events;
restore mirrors invert to 0.00 dB on every measure). The flip at 48 kHz:

| flip ramp ms | bass OOB / CTR dB | kick OOB / CTR dB | mix HF splatter dB | bass OOB / CTR against the mute at **half** the length |
|---:|---:|---:|---:|---:|
| 0 | -30.3 / +68.4 | -28.5 / +70.0 | -16.5 | n/a |
| 2 | -57.5 / +41.0 | -52.5 / +47.4 | -35.7 | -1.25 / -1.88 dB (mute 1 ms) |
| 5 | -64.8 / +32.3 | -60.3 / +35.4 | -45.6 | n/a |
| 10 | -71.3 / +24.3 | -66.2 / +29.2 | -50.8 | -0.46 / -1.97 dB (mute 5 ms) |
| 20 | -77.3 / +18.8 | -72.3 / +23.1 | -56.4 | +0.03 / +0.55 dB (mute 10 ms) |
| 40 | -82.9 / +13.1 | -78.5 / +17.0 | -62.8 | +0.35 / +0.25 dB (mute 20 ms) |
| 50 | -84.6 / +10.9 | -80.5 / +15.1 | -64.5 | n/a |

Across the launch rates within 1.1 dB (OOB), 1.4 dB (CTR) and 2.5 dB (mix). The 40 ms row (added
in attempt 2) is twice the largest `muteMs` the preregistered rules can choose (5, 10 or 20 ms).
The flip at twice the mute's length minus the mute, over the four rates, every material and both
transitions (invert against mute, restore against unmute; `summarise.py`), in dB:

| flip / mute ms | OOB | worst OOB | CTR | HF splatter | total splatter, bass | kick | mix |
|---|---:|---:|---:|---:|---:|---:|---:|
| 2 / 1 | -1.27 to +0.56 | -3.12 to +0.40 | -2.42 to +1.62 | -1.28 to +2.60 | +4.46 to +4.75 | +4.81 to +4.85 | +4.78 to +5.18 |
| 10 / 5 | -0.46 to +0.37 | -1.28 to +2.40 | -1.98 to +0.00 | -0.47 to +1.19 | -3.23 to -3.14 | -0.49 to -0.39 | -2.12 to -1.10 |
| 20 / 10 | -0.34 to +0.27 | -0.47 to +0.67 | -0.14 to +0.55 | -0.34 to +0.80 | +2.11 to +2.20 | -0.99 to -0.94 | +0.30 to +0.94 |
| 40 / 20 | -0.46 to +0.35 | -1.18 to +1.41 | -0.04 to +1.34 | -0.53 to +1.00 | -1.06 to -0.75 | +0.48 to +0.50 | -0.62 to +0.10 |

- **Parity holds on the four click measures.** At the default, the 20 ms flip is the 10 ms mute
  within 0.80 dB on OOB, worst OOB, CTR and HF splatter at every rate (the largest is HF splatter on
  the mix at 96 kHz). At the rules' other two outcomes it is within 1.19 dB (10 / 5) and 1.00 dB
  (40 / 20) on the two mean measures, OOB and HF splatter; the per-event peaks, worst OOB and CTR,
  move more (up to +2.40 dB on the kick's worst event at 10 / 5, +1.41 and +1.34 dB on the bass at
  96 kHz at 40 / 20). Against these, the flip at the same length is 5.45 to 6.17 dB louder on
  every measure. At 2 / 1 ms the far-from-source condition is weak (1.5 kHz is only `0.5 / T` to
  `1.4 / T` above the bass's partials) and the differences reach -3.12 and +2.60 dB.
- **Total splatter differs, and its sign changes with length.** At the default the flip is
  +2.11 to +2.20 dB over the mute on the bass, -0.94 to -0.99 dB under it on the kick and +0.30 to
  +0.94 dB over it on the mix (the largest at 48 kHz). On the bass: +4.5 to +4.8 dB at 2 / 1,
  -3.1 to -3.2 dB at 10 / 5, +2.1 to +2.2 dB at 20 / 10, -0.8 to -1.1 dB at 40 / 20. This is the
  near-source factor, between 0 and 2, weighed inside the measure's window, not the slope. A
  one-partial model of the measure (`flip_model_db` in `summarise.py`: the factor applied to the
  mute's spectrum `sinc(f T) / (2 pi f)` beyond a hard edge) gives, for an edge 56-82 Hz from the
  partial, +4.6 to +5.1, -4.8 to -1.0, -0.4 to +2.6 and -1.9 to +1.2 dB for the four pairs. These
  ranges hold the measured bass values at 10 / 5, 20 / 10 and 40 / 20 and miss them by 0.2 dB at
  2 / 1. For an edge 510 Hz away (the bass's top partial to its out-of-band edge) it gives -2.10,
  +0.25, -0.01 and -0.12 dB: the click measures' parity, and its failure at 2 / 1 ms. No length
  factor gives parity on total splatter at every length.
- **What the difference is.** Near the source the flip is what it is meant to be: a reversal of
  every partial's phase, a change of 2 where the mute's is 1. A real gain carries the reversal out
  by passing through silence, which is the dip below. Total splatter weighs that region against the
  region around the null, so it is not a measure section 1 decides by.

**Answer.** `muteMs` is the right key and not enough length. At 10 ms the flip clicks like a 5 ms
mute (bass CTR +24.3 dB; about +4 dB over the threshold in quiet at 20 dB quieter playback), which is
the case section 1 rejected for the mute. Section 1 set `muteMs` by the click a listener hears out of
band and over the threshold in quiet (OOB and CTR on the exposed bass) and by the click against the
mix's own high band (HF splatter). On those measures a flip over twice the mute's length is the
mute: within 0.80 dB at the default, and at the rules' other outcomes within 1.19 dB on the mean
measures and 2.40 dB on the per-event peaks, against 6 dB at the same length. So section 9 sets the
polarity row to **twice the mute length**: 20 ms at the default. The total-splatter difference does
not change the factor: it is not the click section 1 decides by, and no single factor equalises it
at every length (its sign changes with length).

**The alternatives.** Root confirms the choice (9.8):

- *`muteMs` at the mute's own length* (#1054's present text). The flip clicks about 6 dB more than
  the mute, the click of a mute half as long (a 5 ms mute's at the default). Its dip is half as
  long.
- *`faderMs`* (also 20 ms by default). It holds the parity only while `faderMs` happens to be twice
  `muteMs`: a listening result that moves `muteMs` (values only, D15-1), or a session that shortens
  `faderMs` for responsive faders, would make the flip click with no rule left to fix it.
- *A fourth key, `polarityMs`* (default 20 ms). It would let a session trade the click against the
  dip for polarity alone. Its costs: a fourth session key in #1054's schema and in every host, SDK
  and C ABI path that carries the three (#1054, #1261, #1364, #1394); and a second default kept at
  twice `muteMs` by hand. A listening result that moves `muteMs` would leave it behind (the
  preregistered rules move only the three keys), and the parity would then hold only while two
  defaults agree. No measurement asks for the separate control.
- *Derived, `2 x muteMs`* (chosen). The mute's click, within the margins above, at each `muteMs`
  the rules can choose, and it follows any session value with no new key. Its costs: polarity has
  no length of its own, and the dip lasts twice as long as at `muteMs`.

**The cost: a longer dip through zero.** A polarity change by a real gain must pass through
silence; only its length is a choice. The flip's level is `|1 - 2k / N'|` over its length `N'`:
below -6 dB for half of it, below -20 dB for a tenth of it, and silent at the midpoint. At the default
20 ms that is 10 ms below -6 dB and 2 ms below -20 dB (at 10 ms: 5 ms and 1 ms). In level it is a
10 ms mute followed at once by a 10 ms unmute, with the phase reversed between them. The click
measures do not show whether the dip is heard as a dip on an exposed sustained note (9.7).
The factor trades a measured click excess (+6 dB, the case section 1 rejected) for a dip twice as
long, whose audibility is not measured at either length.

A combined trim and polarity edit on one lane (#1261 D2 orders the trim record, then the polarity
record, on the one coefficient) ramps over the polarity record's length; with the defaults both are
20 ms.

### 9.3 Trim, sends, `follows_mute` and VCA (question 4): laws already measured

- **Input trim.** `set_trim_db` reaches `set_trim_signed` (`B:1434-1478`): one division per record,
  then `input_chain_ramp_block`'s steps 1-3 are `gain_mute_ramp_block`'s steps 1-3 (`K:638-650`).
  Measured: the same moves in dB through the trim and through the fader give bit-identical
  coefficient trajectories (`law_transfer.csv`, `input-trim`: largest difference 0 at every length
  and rate, for a 0 to -24 dB jump and the 800 ms drags at 30 and 60 Hz; harness test
  `trim_and_fader_ramps_are_bit_identical`). Every fader result is a trim result: `faderMs`, 20 ms.
- **Sends (gain, mute, matrix).** The live route ramps by the indexed law, not D11: `LiveRoute::drain`
  starts `IndexedRamp::new(coefficients_at(position), target, length)` per record and `mix` runs
  `route_mix_ramp_block` (`runtime.rs:896-931`). The harness drives the shipped kernel the same way
  (`route_probe`) and compares it with the D11 fader, mute and matrix trajectories over the same
  moves: mute, unmute, a 0 to -24 dB jump, a full pan jump, and the 800 ms fader and pan drags at 30
  and 60 Hz, at every length the earlier files measured (0-50 ms) and every launch rate
  (`law_transfer.csv`, 352 rows). The trajectories differ by at most 1.4e-4 (-77 dB re unity; 96 kHz,
  50 ms drag), which is D11's own accumulated rounding (a running sum of `N` steps, at most about
  `N * 2^-25`); the indexed law is a pure function of the index. Both end on the same bits in every
  case, and the bass out-of-band energy and click-to-threshold ratio agree within 0.03 dB and
  0.08 dB. So the fader, mute and pan results transfer: send gain `faderMs`, send mute `muteMs`, send
  matrix `panMs`.
- **`follows_mute` toggle.** Source argument. A follow record is built by
  `RouteControlProducer::record` from the send's mirrored gain, matrix and mute and the new source
  lanes (`route_controls.rs:82-107`); `gated_route_coefficients` zeroes the column of each followed
  muted lane (`graph/src/lib.rs:759-812`), and the record ramps on the same `LiveRoute` indexed ramp
  as a send mute: one column to or from `+0.0`, a lane-wise send mute. The browser's follow pass
  already gives it the strip mute record's ramp (`W:5274-5364`), and #1226 D5 gives the C ABI's the
  session mute length. The toggle itself is not live on this tree (`follows_mute` is "Fixed for the
  plan", `crates/host-core/src/live_route_state.rs:46-47`); #1226 and #1342 make it live with the same
  record. `muteMs`, 10 ms.
- **VCA offset and mute.** Source argument. A VCA has no audio path (`vca.rs:1-13`). Its offset
  reaches render as member `FaderDb` records (the VCA fader pass, `W:5179-5214`) and its mute as
  member `Mute` records through the one strip-mute owner (`solo.rs:311`, `W:5215-5273`): the fader
  and mute kernels measured in sections 5.1-5.2. VCA offset `faderMs` 20 ms, VCA mute `muteMs` 10 ms
  (#1247 D5 maps the C ABI rows the same way).

### 9.4 The bypass crossfade (question 5): `muteMs`, 10 ms

**Method.** For each launch effect with a shunt (every native effect but the delay and the
multiband compressor, `effect_compiler::lowers_session_bypass`), the effect's own scalar instance
renders the material from its first frame (wet), and the engine's `BypassShunt` gives the
latency-matched dry signal. The switch lands at the materials' event blocks and is emulated with
#1341's law: `out = dry + (wet - dry) * m(k)`, `m` word 0 of an `IndexedRamp`, frame `f` of the
applying block at `k = f + 1`, one `f32` subtract, multiply and add, both ends exact copies; 0 ms is
today's whole-block step. A constant gain moves nothing, so the click is the splatter of
`m (wet - dry)`; OOB, CTR and HF splatter are read on it, relative to the dry programme
(`src/live_rows.rs`). `mute-reference` rows crossfade the programme to silence under the same
measure. They reproduce `mute_click.csv` (bypass against its mute rows, unbypass against its unmute
rows) within 0.01 dB on splatter and HF splatter, and within 0.20 dB on OOB (48 kHz, bass, unbypass,
20 ms: -83.39 against -83.19 dB), 0.09 dB on worst OOB and 0.12 dB on CTR: above the edge
`mute_click.csv` reads the output `y`, which also holds the source's own floor and frame leakage,
and these rows read the splatter `c`, which does not.

**Representative settings** (calibrated per material and rate; values in the CSV's `setting`):
compressor at its defaults (ratio 4, knee 6 dB, attack 10 ms, release 100 ms, no makeup), threshold
set for 6.0 dB peak gain reduction (about -20.8, -12.8 and -19.4 dBFS on bass, kick and mix at
48 kHz); true-peak limiter at its defaults, ceiling set for 6.0 dB peak reduction (-14.6, -6.9 and
-8.0 dBTP); EQ band 1 a bell at 100 Hz, Q 0.707, +6 dB and -6 dB; soft clip driven +12 dB with its
output level-matched to the dry RMS (-9.4, -6.2 and -9.9 dB); transient shaper attack +50 % (kick
and mix); gate closing on the kick (threshold -6 dBFS, ratio 20, range 80 dB, no hysteresis,
attack 1 ms, hold 20 ms, release 50 ms; it opens on each hit and closes in the body, where the
switches fall).

**Results**, 48 kHz (`bypass_crossfade.csv`; 2 and 5 ms in the CSV; bypass and unbypass agree to
0.00 dB; across the launch rates OOB within 2.4 dB, CTR and the mix's HF splatter within 6 dB):

| effect | step dB, bass / kick / mix | today, 0 ms: bass OOB / CTR, kick OOB / CTR, mix HF | 10 ms | 20 ms |
|---|---|---|---|---|
| mute-reference | 0 / 0 / 0 | -36.3 / +62.3, -34.5 / +64.0, -22.5 | -77.4 / +18.3, -72.2 / +23.3, -56.8 | -83.4 / +12.8, -78.4 / +17.1, -62.4 |
| compressor | -6.3 / -9.6 / -9.3 | -42.6 / +56.0, -43.7 / +55.8, -31.6 | -83.7 / +12.0, -81.7 / +14.9, -65.1 | -89.7 / +6.7, -88.0 / +8.8, -71.2 |
| limiter | -6.3 / -11.8 / -14.4 | -43.0 / +55.7, -46.4 / +54.3, -39.0 | -83.6 / +13.0, -84.5 / +13.5, -69.3 | -90.0 / +6.5, -90.9 / +7.3, -74.9 |
| EQ +6 dB | -3.4 / -5.8 / -3.6 | -39.6 / +59.7, -40.7 / +57.3, -25.2 | -80.6 / +15.8, -78.5 / +16.9, -65.9 | -86.4 / +9.6, -84.6 / +10.7, -73.1 |
| EQ -6 dB | -7.3 / -8.3 / -7.6 | -43.8 / +55.8, -43.1 / +55.1, -29.7 | -84.7 / +11.8, -80.9 / +14.2, -70.0 | -90.5 / +5.8, -87.0 / +8.6, -76.9 |
| saturator | -13.7 / -11.2 / -13.5 | -50.9 / +50.1, -46.8 / +52.3, -36.3 | -90.5 / +9.4, -83.8 / +12.4, -65.2 | -96.2 / +2.3, -89.3 / +7.1, -72.0 |
| transient shaper | - / -62.5 / -15.0 | -, -105.7 / -6.1, -39.1 | -, -116.7 / -22.9, -72.4 | -, -117.3 / -22.9, -73.3 |
| gate | - / -0.8 / - | -, -34.7 / +63.2, - | -, -72.4 / +23.1, - | -, -78.5 / +17.1, - |

`step` is the energy of `wet - dry` around the switch relative to the dry programme. The crossfade's
click is the mute's at the same length scaled by that step: on the bass and kick, wherever the step
is above -40 dB, the click's OOB is the mute reference's plus the step within -1.1 to +1.8 dB, at
every length from 2 ms and every rate. No effect's click exceeds the mute reference's at the same
length, rate, material and transition on any measure. The largest excess on each measure is below
zero, and every one is the gate on the kick, which switches between a closed gate and the open kick,
which is a mute: -0.01 dB on worst OOB (44.1 kHz at 2 and 10 ms, 48 kHz at 0 and 10 ms), -0.04 dB on
CTR (48 kHz, 20 ms), -0.14 dB on HF splatter, -0.15 dB on OOB and -0.69 dB on total splatter. The
saturator's own harmonics
fill the out-of-band region 30 dB above its 10 ms click (`wet_oob_db` -58.9 dB on the bass). The
transient shaper's kick switches fall in the body, after its attack boost has decayed (step
-62 dB); its mix row, whose switches fall at random times, carries its transients.

**Answer.** `muteMs` is the right key and 10 ms the right default. A bypass toggle is a switch
between two latency-matched versions of one programme, and its crossfade clicks like a mute of the
same length scaled by the size of the change; every representative setting changes the programme
by at most a mute's step (steps of -0.8 to -15 dB), so at 10 ms its click is at most a 10 ms mute's,
the criterion that set `muteMs` (section 1). Over the four rates the 10 ms crossfade's click is
36.8-41.2 dB below today's step out of band on the bass and kick and 26.2-40.8 dB below it in the
mix's HF splatter (48 kHz: 37.0-41.1 and 28.9-40.8 dB; the lowest, 26.2 dB, is the transient shaper
on the mix at 44.1 kHz). The transient shaper on the kick is apart: its switches fall in the body,
where its step is -60.3 to -62.5 dB and today's click is already -105.7 to -107.3 dB out of band, so
the crossfade lowers it by only 10.2-11.2 dB. A setting that changes the level by more than a mute
does (an EQ boost above +6 dB, or large makeup gain) clicks more in proportion to its step, as a
hand-switched bypass would, and the crossfade still removes the same 36.8-41.2 dB of today's step out
of band.

### 9.5 The detector link glide (question 6): `faderMs`, 20 ms

**Method.** The compressor at its defaults with the threshold set for 6.0 dB peak gain reduction
under `maximum`, prepared once with `dual_mono` and once with `maximum`; the switch is emulated as
the same output crossfade as 9.4 with the `dual_mono` output as dry and the `maximum` output as wet.
This emulates #1370's detector blend (D2-D3: the lane's detector moves linearly from its own
magnitude to the linked one). Materials: the mix (channel correlation 0.95), `mix-wide` (its right
channel 0.37 s later, correlation 0.10, so the channels are uncorrelated) and `bass-kick` (bass
left, kick right, correlation 0.01, band-limited so OOB and CTR exist).

**Why the emulation's click bounds the real glide's.** In #1370 the blend acts on the detector,
before the static curve (`curve_target`, `kernel.rs:349-361`) and the attack/release smoother
(`ballistic`, `:364-369`: `y += c (target - y)` through `effect_runtime::envelope::rms_follow`,
`crates/effect-runtime/src/envelope.rs:163-176`, with `c = 1 - exp(-1 / (tau fs))` evaluated in
`f64` and rounded once, `rate_coefficient`, `crates/compressor/src/design.rs:129-143`, whose reason
is `:18-30`). The emulation moves the output gain linearly
between the two modes' already-smoothed gains, so its gain path has two slope corners and a
spectrum falling 12 dB per octave [SMITH-SASP]. In the real glide the blend's corners reach the gain
only through the one-pole: its response `c / |1 - (1 - c) e^(-j 2 pi f / fs)|` is at most 1 and
falls 6 dB per octave above `1 / (2 pi tau)` (16 Hz for the default 10 ms attack), and its output
slope is continuous, so the real gain path falls at least 18 dB per octave there. At 1.5 kHz, where
OOB and HF splatter read, the attack smoother alone divides the corners by `2 pi f tau`, about 94
(39 dB). The static curve can make the start of a `dual_mono` to `maximum` glide steeper than a
linear crossfade: above the knee the gain is a power of the detector, so with `a = 1 - 1/ratio` and a
channel ratio `rho` the initial slope is `a (rho - 1) / (1 - rho^-a)` times the crossfade's mean
slope, about 8 (18 dB) for ratio 4 at 20 dB of channel difference. The smoother covers that for
attack times of about 1 ms and more; at the 0.1 ms minimum it does not, and the real glide can then
click up to about 18 dB more than the emulation. No #1370 gate measures a click (its gates check
the blend against an `f64` reference, bank-mates' bits, the limiter's ceiling and linked path, the
payload and allocations); only its listening item, a blinded A/B of a `dual_mono` to `maximum` switch
over the default ramp against a step on a wide stereo bus, listens to the real glide.

**Scope: the compressor only.** #1370's glide and #1054 D3's row cover four effects: the
compressor, the gate-expander, the transient shaper and the true-peak limiter. Only the compressor
was measured and bounded here, as the brief asks. The bound does not carry to the other three as it
stands. Below its threshold the gate-expander's gain is the detector raised to the power
`ratio - 1` (up to 19), clamped by its range (`crates/gate-expander/src/kernel.rs:240-245`), against
the compressor's `1 - 1/ratio` (below 1); a link change can also move a lane across the gate's
open/close threshold (`:217-238`); and its attack is 1 ms by default, 0.1 ms at least. The transient
shaper's and the limiter's gain paths follow their own detector laws. Their link-glide clicks are
not assessed (9.7).

**Results**, 48 kHz (`link_glide.csv`; link and unlink agree to 0.00 dB; across rates OOB within
2.0 dB, CTR 2.4 dB, HF splatter 4.2 dB):

| material | step dB | 0 ms | 5 ms | 10 ms | 20 ms | 50 ms |
|---|---:|---:|---:|---:|---:|---:|
| mix, HF splatter | -33.1 | -55.4 | -85.4 | -90.9 | -96.5 | -104.4 |
| mix-wide, HF splatter | -21.0 | -46.8 | -74.3 | -79.4 | -85.1 | -91.6 |
| bass-kick, OOB / CTR | -18.8 | -54.7 / +51.2 | -90.0 / +13.5 | -95.3 / +7.2 | -103.3 / +1.2 | -110.2 / -6.8 |

**Answer.** `faderMs` is right. A link change moves gain reduction, continuously, like a fader,
and is not time-critical like a mute. At 20 ms the bound's click on the most exposed material is
-103.3 dB out of band and CTR +1.2 dB at the loud calibration (48 kHz; 1-2 of 10 events above the
threshold in quiet over the four rates, none at 20 dB quieter playback). Over the four rates it is
24.3-25.9 dB out of band, 22.6-23.7 dB on the worst event and 16.8-18.2 dB in CTR under the bass's
10 ms mute; on the mixes its HF splatter is 25.8-28.2 dB (`mix-wide`) and 38.2-40.0 dB (`mix`) under
the mix's 10 ms mute, and its total splatter 26.2-27.3 and 37.7-38.5 dB. `muteMs` would also pass
with the default ballistics (bass-kick -95.3 dB, CTR +7.2 dB at 10 ms, 48 kHz), but `faderMs` keeps
the margin the bound's limit needs: even 18 dB worse than the emulation (0.1 ms attack, channels
20 dB apart), a 20 ms glide comes within 1.2 dB of the 10 ms mute's CTR and stays 6 dB or more
under it out of band, where a 10 ms glide (CTR margin 10.8-12.3 dB) would pass it by 5.7-7.2 dB,
close to a 5 ms mute's click.

### 9.6 Listening (D3): no new contrast

No contrast is added to `listening/PREREGISTRATION.md`, and the packet is unchanged:

- **Polarity.** The factor of two comes from section 1's criterion, not from a listening question.
  On the four click measures (OOB, CTR and HF splatter, by which section 1 set `muteMs`, and worst
  OOB) the flip at twice `muteMs` is the mute at `muteMs` within 0.80 dB at the default, and at the
  rules' other outcomes within 1.19 dB on the mean measures and 2.40 dB on the per-event peaks
  (9.2), so the M block's decision on `muteMs` carries over through the factor. The one measure on
  which the two differ, total splatter (on the bass, +2.11 to +2.20 dB at the default), holds the
  near-source part of the flip's spectrum, the phase reversal that a real gain carries out through
  its dip; its sign changes with length and no single factor equalises it (9.2), so a contrast
  could not set the factor by it either. A listening result changes values only (D15-1) and the
  factor is a rule, so a polarity contrast's only possible decision would move `muteMs` for
  polarity's sake, which the M block decides on the mute itself. Whether the dip is heard as a dip
  is a question about the kernel, not a value (9.7).
- **Bypass crossfade.** Its click is at most the mute's at the same length for every representative
  setting (9.4): the M block bounds it.
- **Link glide.** At the defaults its bound sits 16.8 dB or more under the 10 ms mute's on every
  measure (the smallest margin is CTR at 44.1 and 88.2 kHz; 9.5): there is nothing for a listener to
  find that the M block does not bound. This covers the compressor only (9.5, 9.7).

### 9.7 Objective, pending, verified and not verified

**Objective, on this tree:** every law and anchor in 9.1; the trim's bit identity with the fader;
the route law's agreement with D11; the polarity factor on the click measures; the bypass and link
clicks of the emulations. **Awaits #1388:** only the values of `muteMs`, `faderMs` and `panMs`
(sections 1 and 6). Every row follows its key's value, the polarity row as twice `muteMs`; no row
awaits a listening result of its own.

Verified: the four section-9 CSVs reproduce byte for byte over two four-rate runs (attempt 2 adds
the 40 ms rows to `polarity_click.csv`; with them removed the file is attempt 1's byte for byte, and
the other three files are attempt 1's); the seven earlier CSVs reproduce byte for byte with the
extended harness; 17 harness tests pass, clippy clean; mutation runs, recorded in the #1055 spec's
attempt record, turn the crossfade tests red when the emulation accumulates `m` or computes either
end, and turn the route-probe retarget test red when the probe keeps its position on a new record;
every crossfade case asserts that the `f32` emulation copies both ends and stays within four `f32`
epsilons (relative to the larger plane) of the analysed `dry + m (wet - dry)`; the mute-reference
rows reproduce `mute_click.csv` within 0.20 dB (9.4); every effect block rendered reports nothing
(no invalid span, no sanitised or non-finite sample); `summarise.py` prints the tables and ranges
of 9.2-9.5 from the committed CSVs.

Not verified:

- the real #1341 crossfade and #1370 glide (both are emulated; their issues' gates test the
  kernels, and no #1370 gate measures a click);
- the link bound below about 1 ms of attack (9.5);
- the link glide of the gate-expander, the transient shaper and the true-peak limiter: only the
  compressor was measured and bounded, and the bound does not carry to the gate's steep curve and
  threshold crossing as it stands (9.5); #1370's listening item is the only check on them;
- the bypass crossfade of the delay and the multiband compressor: they have no shunt yet (*Give the
  delay a live bypass shunt*, #1339; *Give the multiband compressor a live bypass shunt*, #1340), so
  nothing was measured; #1341 says its law covers them with no further edit once they land;
- whether a polarity flip's dip through zero (at the default 20 ms: 10 ms below -6 dB, 2 ms below
  -20 dB, silent at the midpoint; 9.2) is heard as a dip on an exposed sustained note. The click
  measures do not assess it, and #1388 cannot act on it (its outcomes are values; the factor is a
  rule). A through-zero law that does not dip is a kernel change, outside this issue (non-goals);
  the check belongs with a polarity-kernel successor, which root decides whether to open;
- effect settings beyond the representative ones (the click scales with the step); a transient
  shaper switched during a kick's attack;
- banked, AArch64 or wasm renders (per-lane, unfused kernels, so they should match).

### 9.8 For #1054, #1341 and the other dependants (D2)

- **Keys:** every row's key matches #1054 D3 and #1341: `fader_ms` for fader, input trim, send
  gain, VCA offset and the detector link glide; `mute_ms` for mute, solo, polarity invert, send mute,
  the send's `follows_mute` toggle, VCA mute and the bypass crossfade; `pan_ms` for pan, matrix and
  send matrix. `CONTROL_SMOOTHING_DEFAULT` stays mute 10, fader 20, pan 20.
- **One length differs: polarity invert is twice the mute ramp.** `LiveRamps::for_row(PolarityInvert)`
  returns `2 * mute_samples` (192,000 samples at the 1000 ms bound and 96 kHz, inside the trim
  kernel's exact `f32` countdown, `2^24`). An edit that carries its own length keeps it. This
  changes wording, not only a number, in each dependant:
  - #1054 D3: a note in the table (polarity invert, `mute_ms` times two); D4: "`for_row` ...
    returns that row's key field" and "No row has a separate default" gain the polarity
    exception; gate 3: "`for_row` returns the D3 key's field for every `LiveRampRow`" becomes "...,
    and twice `mute_samples` for `PolarityInvert`"; the schema document's table says so.
  - #1261 D2, the Ramps bullet: a `PolarityInvert` record carries `for_row(PolarityInvert)`, not
    `LiveRamps::mute_samples`. #1261 gate 1 (`1261-*.md:146-147`): "a PolarityInvert record its
    `mute_samples`" becomes twice `mute_samples` (`for_row(PolarityInvert)`); left as it is, the
    gate tests the rule this record removes.
  - #1364: its row map ("mute, solo, polarity ... use `mute`") and its gate-1 example of a wrong row
    map ("polarity on the fader key"). At the defaults `2 * mute == fader` (960 samples at 48 kHz
    either way), so that test needs non-default keys to tell the two apart. #1364 gate 1
    (`1364-*.md:131-132`): "480 for the mute rows" at 48 kHz holds for mute, solo, send mute,
    `follows_mute` and VCA mute, but polarity resolves to 960. #1364 D2 (`:62`): admission replaces
    the sentinel with "the field that #1054 D3 names", which becomes `for_row(row)` (twice
    `mute_samples` for polarity).
  - #1394 D5: `resolve` falls back to `for_row`, so an absent polarity length resolves to twice
    `mute_samples` with no change of its own; its gates test no polarity row. Only its background
    sentence changes (`1394-*.md:26-27`): "`for_row(row)` returns the D3 key field for a row"
    gains "(twice `mute_samples` for `PolarityInvert`)".
- **Root confirms one choice (9.2):** the derived rule `2 * mute_samples` inside the three-key
  schema, over #1054's present text (polarity at `mute_samples`) and over a fourth key
  (`polarityMs`).
- **#1341:** `mute_ms` and its 10 ms default are confirmed for the bypass crossfade. **#1370 and
  #1371:** `fader_ms` and 20 ms are confirmed for the link glide.
