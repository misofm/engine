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
- Amendments: Amendment 1, 2026-10-05, before any trial (block P, polarity; at the end of this
  record).
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

## Amendment 1, 2026-10-05, before any trial

- **When.** 2026-10-05. No packet had been prepared, and no trial, response or reveal existed. The
  status stays `preregistered`.
- **Wording corrected, 2026-10-05, before any trial:** the power sentence names the correct-answer
  rate, and the control and peak lines are made exact (#1055 follow-up verdict); no value changed.
- **Reason.** `../FINDINGS.md` section 9 ramps a polarity invert over twice the mute length
  (`2 x muteMs`, 20 ms at the shipped defaults), by the click measures section 1 decides by
  (9.2). The flip's gain passes through zero: below -6 dB for 10 ms of its 20 ms, below -20 dB for
  2 ms, silent at the midpoint. No measure says whether a listener hears that dip, or the click,
  on an exposed note (9.7).
- **Source.** Root's ruling on #1055, 2026-10-05: the derived rule (`2 x muteMs`, no fourth key)
  is confirmed; the session asks whether the flip at the shipped rule is heard as a click or as a
  level dip; a listening result changes default values only. #1055 D3 allows a contrast added
  before any trial as a dated amendment.
- **What changes.** Block P (36 trials, run after R) is added. The session becomes 104 trials:
  the stopping rule's "these 68 trials" reads "these 104 trials", the blocks run in the order M, F,
  R, P, and a completed record lists 104 valid rows. Three training files are added. Nothing else
  in the record above changes: the contrasts, trial counts, statistics and decision rules of
  blocks M, F and R stay as written, and their positive controls (M-bass-0, F-30-0, R-mix-50) gate
  every M, F and R non-detection and no block-P result.

### Question (block P)

- **Hypothesis P.** At the shipped rule, a polarity invert and restore, each over 20 ms (twice the
  shipped `muteMs`), on an exposed sustained bass note can be told from the same note with no flip;
  and if so, by the in-band part of the change (the dip through zero, with the phase reversal it
  carries) or by the out-of-band part (the click that section 1's measures read). The kick-body and
  dense-mix contrasts are descriptive: they show whether masking material hides the flip.
- **Reference.** The same note through the same strip with no record: the only stimulus with no
  artefact. A long flip cannot be the reference, because its dip is longer, and the dip is the
  question.
- **Why a band split and not a length contrast.** A longer flip has a longer dip and a weaker
  click, a shorter flip the reverse. Two lengths therefore differ in both cues at once, and a
  detection between them cannot say which cue the listener heard. The split keeps the engine's
  change as it renders it and gives the listener one cue at a time: with `x` the note with no flip,
  `y` the flipped note and `d = y - x`, the in-band stimulus is `x + L d` and the out-of-band
  stimulus is `y - L d`, where `L` passes the bass's band and stops the measures' out-of-band
  region (stimulus identity, below). Together they rebuild the flip:
  `(x + L d) + (y - L d) = x + y`. Their edge is the measures' own: the bass's partials stop at
  1 kHz, and section 1's out-of-band measures read from 1.5 kHz.
- **Test type.** As blocks M, F and R: randomized A-B two-interval forced choice, the candidate's
  position balanced within each contrast. The block's question, printed with every trial: "Which
  interval has a click, tick, thump or brief dip while the sound plays on?" The candidate is the
  interval with the flip, or with one part of its change.
- **Trial count.** 36:

  | contrast | block | candidate | reference | trials | role |
  |---|---|---|---|---:|---|
  | P-bass-0-oob | P | polarity-bass-0ms-oob | polarity-bass-none | 2 | positive control (click) |
  | P-bass-200-inband | P | polarity-bass-200ms-inband | polarity-bass-none | 2 | positive control (dip) |
  | P-bass-20 | P | polarity-bass-20ms | polarity-bass-none | 8 | primary (the flip) |
  | P-bass-20-inband | P | polarity-bass-20ms-inband | polarity-bass-none | 8 | primary (its dip) |
  | P-bass-20-oob | P | polarity-bass-20ms-oob | polarity-bass-none | 8 | primary (its click) |
  | P-kick-20 | P | polarity-kick-20ms | polarity-kick-none | 4 | secondary |
  | P-mix-20 | P | polarity-mix-20ms | polarity-mix-none | 4 | secondary |

- **Why these counts.** The same reasoning as blocks M, F and R. A decision rests on a detection at
  p <= 0.05, and 8 trials is the smallest count that allows one lapse: 7 of 8 gives p = 0.0352,
  where 7 trials need 7 of 7 (6 of 7 gives 0.0625). With 8 trials, a listener who answers
  correctly on 95 % of trials is detected with probability 0.94, on 90 % with 0.81, on 80 % with
  0.50. The rule needs one decision per cue, so each of the three primaries has 8 trials. Each
  control has 2 trials and must be 2/2, as in the other blocks. The kick and mix contrasts support
  no decision; with 4 trials (4/4 gives p = 0.0625) they are descriptive. At the pace the other
  blocks assume (68 trials in about 15 minutes), the block takes about 8 minutes.
- **Statistics.** As above: the exact one-sided binomial test per contrast; a primary contrast is
  **detected** when p <= 0.05. Both P controls must be 2/2. A missed P control makes every
  block-P non-detection inconclusive and leaves block-P detections standing. It does not affect
  blocks M, F and R, and a missed M, F or R control does not affect block P.
- **Multiple comparisons.** No correction. If no cue is audible, the chance that at least one of
  the three primaries is detected is 1 - (247/256)^3 = 10.2 %. A false detection costs a finding
  that root weighs, never a value (below).

### Decision rule (block P), applied mechanically by `listening.py reveal`

- **Values: none.** Block P changes no value. `muteMs`, `faderMs` and `panMs` follow the rules of
  blocks M, F and R only. The `2 x muteMs` rule and the keys are fixed (root, 2026-10-05). Why no
  value can answer block P: the flip's length is `2 x muteMs`, so any value change for polarity's
  sake moves `muteMs` off the value that blocks M and R decide on the mute itself. A dip is
  shortened only by a shorter flip, whose click is louder (the trade section 1 rejected for the
  mute). A click is lowered only by a longer flip, whose dip is longer and whose `muteMs` lengthens
  every mute. This session measures neither trade, so each is root's.
- **Each cue at the shipped flip (20 ms).** The flip is P-bass-20, the dip P-bass-20-inband and
  the click P-bass-20-oob. A cue is **heard** when its contrast is detected (7 or 8 of 8), **not
  heard** when it is not detected and both P controls are 2/2, and **inconclusive** otherwise.
- **Outcome.** **Not heard** when the flip, the dip and the click are all not heard: at the
  shipped rule, this listener, chain and level do not tell the flip from no flip on the exposed
  note, and the open question of `../FINDINGS.md` 9.7 is answered for them. **Heard** when any of
  the three is heard: heard as a dip, as a click, as both, or, when only the flip is heard, with the
  cue not separated. **Inconclusive** otherwise. The completed record states the outcome; a
  "heard" outcome is a finding for root. Every outcome leaves the defaults unchanged. P-kick-20 and
  P-mix-20 are reported with it and decide nothing.
- **At the decided `muteMs`.** If blocks M and R move `muteMs`, the polarity row's flip becomes
  twice the decided value. The rule carries each cue to that flip only where the length decides
  it: a longer flip has a longer dip (its level is below every depth for a proportionally longer
  time) and a weaker click (out-of-band energy falls with length, `../data/polarity_click.csv`).
  At a 40 ms flip (`muteMs` 20): a dip heard at 20 ms is heard, a click not heard at 20 ms is not
  heard, and anything else is not assessed. At a 10 ms flip (`muteMs` 5): a dip not heard at 20 ms
  is not heard, a click heard at 20 ms is heard, and anything else is not assessed. If blocks M, F
  and R are inconclusive, the defaults stay (`muteMs` 10) and the 20 ms result stands as it is.
- **Matrix coefficients.** A raw or send matrix coefficient through zero ramps by the same law over
  `panMs` (`../FINDINGS.md` 9.1). Block P does not test it, and this rule does not use `panMs`.

### Stimulus identity (block P)

- `control_smoothing_measure stimuli` renders block P at the same commit, rate (48 kHz), quantum
  (128 frames) and strip as the other blocks: one track, default builtins, chain input, then
  fader/mute, then matrix. The input polarity of both lanes inverts by a record admitted at 0.80 s
  (applied at sample 38,400) and restores by one admitted at 1.30 s (applied at 62,464), each
  through `InputBuiltins::set_polarity_invert` over the flip's length: 0 samples (the hard flip),
  960 samples (20 ms: twice the shipped mute's 480, the polarity row's `2 * mute_samples`,
  `../FINDINGS.md` 9.8, passed explicitly) and 9,600 samples (200 ms). `none` is the same strip
  with no record. Every condition is 2.0 s with the same 20 ms raised-cosine edge fades.
- Materials: the bass, rendered over 2.4 s and cut to its first 2.0 s after the split; the kick,
  2.0 s with onsets at 0.10 + 0.5 k s, so both records fall 200 ms into a kick body; the mix, the
  excerpt the mute and chop conditions use (`--mix-wav` replaces it here too).
- The split `L` (bass only, `../measure/src/split.rs`): a Kaiser-windowed ideal low-pass [Kaiser
  1974, as given by Oppenheim and Schafer, *Discrete-Time Signal Processing*, 3rd ed., 7.6], 751
  taps, beta 12.27, cutoff 1,250 Hz, transition 1,000 to 1,500 Hz, design attenuation 120 dB, unit
  DC gain. It is applied centred (zero phase) in `f64` to the change `d` over the whole render,
  then rounded once to `f32`. Measured on a 0.5 Hz grid: within 1.12e-5 dB of unity up to 1 kHz,
  and -119.4 dB or lower from 1.5 kHz to 24 kHz. As rendered, the in-band stimulus keeps the
  in-band change within 1e-5 dB and leaves the click 73.1 dB down (the 20 ms flip's click is
  +18.8 dB over the threshold in quiet at the loud calibration, 9.2, so this residue is far below
  it). The out-of-band stimulus keeps the click within 1e-5 dB and leaves the in-band change
  124.3 dB down (harness tests `split::tests` and `stimuli::tests`).
- Level/gain matching: as the other blocks, not normalised; the two conditions of a contrast
  share the material and differ only by the flip or one part of it. Measured RMS differences
  (`manifest.tsv`, harness at this amendment, crates at `17f0bf18c`): P-bass-20 and
  P-bass-20-inband 0.06 dB, P-bass-20-oob and P-bass-0-oob 0.00 dB, P-kick-20 0.04 dB, P-mix-20
  0.05 dB, and 0.63 dB for the P-bass-200-inband control (its dip).
- Peak/clipping check: the highest block-P peak is -1.894 dBFS (the kick); none clips.

### Training (block P)

Three labelled files join the six: `07-polarity-bass-no-flip` (the reference),
`08-polarity-bass-click-of-a-hard-flip` (the P-bass-0-oob candidate: the unflipped note with only
the hard flip's out-of-band change) and `09-polarity-bass-dip-of-a-slow-flip-200ms` (the
P-bass-200-inband candidate: the note with only the slow flip's in-band change). They carry no
answer and are played before block P.

## Sign-off

- Facilitator: unassigned.
- Listener: unassigned.
- Reveal verifier: unassigned.
