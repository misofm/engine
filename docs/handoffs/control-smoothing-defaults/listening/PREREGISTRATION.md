# Issue 1055 control-smoothing listening preregistration

Format: `dsp-research/listening/TEMPLATE.md`. This record fixes the procedure and the decision
rules before any human trial. It is evidence only once a completed record replaces the pending
fields; it never replaces the objective gates in `../FINDINGS.md`.

## Identity and status

- Schema version: 1
- Record ID: `issue1055-control-smoothing-v1`
- Evidence kind: real listening
- Status: preregistered
- Date and time (UTC): 2026-09-28T13:00:00Z
- Sound-quality claim: none

## Preregistered question

- Hypotheses, one per block, each about the engine's own linear ramps:
  - **M (mute click).** On an exposed sustained bass note, a mute and unmute with a 5 ms (and a
    10 ms) ramp can be told from the same edit with a 50 ms ramp by a click at the transition. The
    kick-body and dense-mix contrasts check whether masking material hides it.
  - **F (fader zipper).** With control updates at 30 Hz, a 20 ms fader ramp can be told from a 40 ms
    ramp (which the measurements put at the continuous-drag floor) by stepping, buzz or roughness
    during a 30 dB/s move. The 60 Hz and 120 dB/s contrasts bound the answer.
  - **R (responsiveness).** On a 16th-note mute stutter over the dense mix, a 10 ms or 20 ms mute ramp
    sounds softer or less tight than 5 ms.
- Test type: randomized A-B two-interval forced choice with a directed question per block (the
  question is printed with every trial). The candidate's position is balanced within each contrast.
- Trial count: 68 (M 34, F 24, R 10), per contrast:

  | contrast | block | candidate | reference | trials | role |
  |---|---|---|---|---:|---|
  | M-bass-0 | M | mute-bass-0ms | mute-bass-50ms | 2 | positive control |
  | M-bass-5 | M | mute-bass-5ms | mute-bass-50ms | 8 | primary |
  | M-bass-10 | M | mute-bass-10ms | mute-bass-50ms | 8 | primary |
  | M-bass-20 | M | mute-bass-20ms | mute-bass-50ms | 4 | secondary |
  | M-kick-5 | M | mute-kick-5ms | mute-kick-50ms | 4 | secondary |
  | M-mix-2 | M | mute-mix-2ms | mute-mix-50ms | 4 | secondary |
  | M-mix-5 | M | mute-mix-5ms | mute-mix-50ms | 4 | secondary |
  | F-30-0 | F | drag-bass-30hz-0ms | drag-bass-30hz-40ms | 2 | positive control |
  | F-30-10 | F | drag-bass-30hz-10ms | drag-bass-30hz-40ms | 4 | secondary |
  | F-30-20 | F | drag-bass-30hz-20ms | drag-bass-30hz-40ms | 8 | primary |
  | F-60-10 | F | drag-bass-60hz-10ms | drag-bass-60hz-40ms | 4 | secondary |
  | F-60-20 | F | drag-bass-60hz-20ms | drag-bass-60hz-40ms | 2 | secondary |
  | F-flick-20 | F | flick-bass-30hz-20ms | flick-bass-30hz-40ms | 4 | secondary |
  | R-mix-10 | R | chop-mix-10ms | chop-mix-5ms | 4 | secondary |
  | R-mix-20 | R | chop-mix-20ms | chop-mix-5ms | 4 | secondary |
  | R-mix-50 | R | chop-mix-50ms | chop-mix-5ms | 2 | positive control |

- Stopping rule fixed before reveal: exactly these 68 trials. At most two attempts per trial; an
  invalid attempt (playback fault, interruption) records a reason and no answer. No trial is added
  or repeated after reveal. Blocks may be run in separate sittings, in the order M, F, R.
- Statistical method: per contrast, the exact one-sided binomial test of the correct count against
  chance (p = 1/2). A primary or secondary contrast is **detected** when p <= 0.05, which for 8
  trials means 7 or 8 correct. A 4-trial contrast cannot reach 0.05 (4/4 gives p = 0.0625); it is
  descriptive, and 4/4 counts as **suggestive** where a rule below says so. Each positive control
  must be 2/2; if any is missed the session is **inconclusive** for every non-detection, because the
  playback chain or level did not reveal even the hard switch.
- Multiple-comparison correction: none. The decisions use the three primary contrasts as ordered
  gates plus two suggestive 4/4 checks; the other secondary contrasts are descriptive and support no
  claim on their own. Family-wise error across all 13 non-control contrasts is not controlled.
- Decision rules (applied mechanically by `listening.py reveal`):
  - `muteMs` (mute and solo): 5 if M-bass-5 is not detected; else 10 if M-bass-10 is not detected;
    else 20 if neither M-bass-20 nor R-mix-20 is 4/4; else 10, with the residual tick recorded as a
    known limit of the linear ramp shape.
  - `faderMs` and `panMs`: 20 if F-30-20 is not detected; else 35 (one 30 Hz update interval plus
    margin), or 20 with a documented requirement that hosts send fader and pan updates at 60 Hz or
    faster, which F-60-10 and F-60-20 inform.

## Exact candidate and stimulus identity

- Candidate build IDs, commit hashes, target, and target features: `control_smoothing_measure
  stimuli` built from `../measure` at the commit recorded as `engine_commit` in
  `public/preparation.json`; x86-64-v3 (AVX2 and FMA pinned by `.cargo/config.toml`). The ramp
  kernels use unfused multiply and add per lane, so the rendered samples do not depend on the target.
- Complete processor/session parameters: one track, default builtins, live fader/mute
  (`FaderMuteRampBuiltins`) then 2x2 matrix, records applied at the top of the first 128-frame block
  at or after admission. Mute stimuli: mute at 0.80 s, unmute at 1.30 s, 2.0 s long. Drag: 0 -> -24 ->
  0 dB, 0.8 s each way from 0.3 s with a 0.2 s hold, UI updates at 30 or 60 Hz, 2.4 s. Flick: the
  same over 0.2 s each way, twice, 30 Hz, 1.6 s. Chop: the mute toggles every 125 ms from 0.25 s to
  1.75 s, 2.0 s. Ramps convert from milliseconds by `round_half_up(ms * 48000 / 1000)`.
- Sample rate and render quantum: 48,000 Hz and 128 frames.
- Stimulus provenance and license: repository-owned deterministic synthesis (`../measure/src/
  material.rs`), no third-party audio. If `--mix-wav` substitutes an owner-supplied mix for the mute
  and chop conditions, record its source, permission and offset here before preparation.
- Fixture paths and hashes: every condition's SHA-256 is in `public/preparation.json`
  (`stimulus_sha256`); the design is frozen by `design_sha256`.
- Render hashes: as above; a re-render at the same commit reproduces them.
- Level/gain matching method and measured tolerance: the two conditions of a contrast share the
  material and the gain trajectory and differ only in ramp length; they are not normalised, because
  the ramp's own energy is part of the question. Measured RMS differences (`manifest.tsv`, 48 kHz,
  engine commit `ed0556a9`): at most 0.07 dB for every M and F contrast and for R-mix-10, 0.20 dB for
  R-mix-20, and 0.70 dB for the R-mix-50 control.
- Peak/clipping check: every stimulus peaks at or below -1.9 dBFS (the renderer refuses to write a
  stimulus at or above 0 dBFS).

## Blinding and randomization

- Facilitator or automation owner: `listening.py prepare` (automation). A second person acting as
  facilitator and reveal verifier is preferred; if the owner self-administers, they must not open
  `private/` or the rendered stimulus directory before reveal.
- Anonymized listener ID and relevant experience: recorded at reveal.
- Randomization algorithm/version and seed: SplitMix64-v1 with a Fisher-Yates shuffle (the
  repository's listening convention), seeded from 8 bytes of `os.urandom` at preparation. The seed
  is stored only in `private/assignment-key.json`.
- Candidate mapping held by: `private/assignment-key.json` (mode 0600, directory 0700), committed to
  by its SHA-256 in `public/preparation.json`.
- Training/familiarization: `public/training/*.wav`, six labelled files (hard-switch and 50 ms mute
  on bass, stepped and 40 ms drag at 30 Hz, 5 ms and 50 ms chop), played before the trials. No
  answers are recorded for them.
- Reveal time and reveal-log location: `listening.py reveal` after all trials, writing `reveal.json`
  (mode 0600) with the response and key hashes.

## Playback chain and conditions

- Source format: 48 kHz stereo 32-bit float WAV, played by the system player (`afplay`, `paplay`,
  `aplay` or `ffplay`) or any player the listener chooses.
- DAC/interface, driver, and operating mode: record at reveal.
- Transducer and calibration method/level: record at reveal. Set the level once, on
  `training/02-mute-bass-50ms.wav`, to the loudest level the listener would normally mix at, and
  leave it. If an SPL meter is available, record the C-weighted slow level of that file.
- Room or headphone conditions: record at reveal (closed headphones or near-field monitors).
- Background or environmental notes: record at reveal.

## Raw responses

No human trial has been run. A completed record lists 68 valid rows from `responses.jsonl`, with the
hidden assignment withheld until reveal. Synthetic answers are prohibited.

## Result and bounded conclusion

- Counts and computed result: pending; no result exists.
- Confidence interval/p-value where applicable: pending.
- Adverse observations and known confounds: the materials are synthetic; one listener; one playback
  chain and level; the R block has a 0.20 dB level difference in R-mix-20.
- Objective gates already passed (artifact IDs): `../data/*.csv` at the commit that adds this record.
- Conclusion limited to the preregistered question: none before completion.
- Reproducibility artifact locations: `../measure`, `listening.py`, this record.
- Conflicts of interest: record before completion.

## Sign-off

- Facilitator: unassigned.
- Listener: unassigned.
- Reveal verifier: unassigned.
