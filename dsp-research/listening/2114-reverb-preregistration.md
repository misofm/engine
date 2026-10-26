# Issue 2114 reverb listening preregistration

This record fixes the procedure before any listening. It is evidence, never a gate: no merge,
release or issue state waits on it, and no objective gate of `dsp-research/reverb.md` is replaced
or relaxed by it. A human facilitator runs it later; no trial has been run and no result exists.

## Identity and status

- Schema version: 1
- Record ID: `2114-reverb-preregistration-v1`
- Evidence kind: real listening
- Status: preregistered
- Date and time (UTC): 2026-10-26T00:00:00Z
- Sound-quality claim: none

## Preregistered question

Three questions, each one ABX block per item per listener, then a forced preference
(`dsp-research/reverb.md`, "Listening protocol or evidence"):

- **Q1, modulation (decision 7).** Is the shipped design without modulation distinguishable from
  the same design with the measured modulation (4 of 16 lines, linear interpolation, at most
  8 samples at 48 kHz, 0.30-0.87 Hz incommensurate triangle LFOs; the M4 setting), and if so,
  which is preferred?
- **Q2, line count (decision 1).** Is N = 16 distinguishable from N = 8 (the M1/M5 N = 8 design,
  same parameters), and if so, which is preferred?
- **Q3, size increase (OQ2b, decision 6).** Under a large size increase, is the clear-on-read ring
  (a short gap per line) distinguishable from a read-side crossfade without clearing (a repeat of
  stale ring words), and if so, which is preferred?

- Hypothesis: no claim before a completed blinded session; this record only fixes the procedure.
- Test type: ABX for audibility; a forced A-B preference after each ABX block, descriptive.
- Trial count: 16 ABX trials per (question, item, listener) cell. Four items and three listeners
  give 192 ABX trials per question and 576 in all (192 per listener, in three sessions of one
  question each). Preference: one forced choice per (question, item, listener), 12 per question.
- Stopping rule fixed before reveal: stop after exactly 16 valid trials per cell. A trial is invalid
  only for a playback fault the facilitator logs before reveal; an invalid trial is repeated once
  at the end of the same cell. No early stop on success or failure.
- Statistical method and confidence interval: per question, the pooled correct count over its 192
  trials, exact binomial against p = 0.5, one-sided (an ABX listener cannot do worse than chance
  except by chance), with a two-sided 95% Clopper-Pearson interval on the proportion correct.
  Secondary, descriptive only: per listener (64 trials) and per item (48 trials) counts with the
  same interval.
- Multiple-comparison correction, if any: Holm-Bonferroni over the three primary tests at family
  alpha 0.05. For n = 192 the critical counts are 112 (alpha 0.05/3), 111 (0.05/2) and
  108 (0.05).

### Decision rule (fixed now)

- A question is **audible** when its pooled count passes its Holm threshold.
- Q1: decision 7 (no modulation) is **reopened** only if Q1 is audible and at least 9 of the 12
  preferences favour modulation on. Otherwise decision 7 stands.
- Q2: decision 1 (N = 16) is **reopened** only if Q2 is audible and at least 9 of the 12
  preferences favour N = 8. Otherwise decision 1 stands.
- Q3: OQ2b (clear-on-read kept) is **reopened** only if Q3 is audible and at least 9 of the 12
  preferences favour the repeat (no clearing). Otherwise the choice stands. Dropping clear-on-read
  moves rendered bits of the kernel and the oracle and changes the S1-S3 argument, so a reopened
  OQ2b goes to root as a class-B question; this record does not decide it.
- "Reopened" means a new or amended issue with this record attached. It never edits a gate.

## Exact candidate and stimulus identity

- Candidate build IDs, commit hashes, target, and target features: every candidate for Q1-Q3 is a
  scratch render from the `f64` model, not the shipped kernel: the kernel has no modulation, no
  N = 8 form and no non-clearing ring. Both sides of each question come from the same model, so
  they differ only in the property under test. The model (`measure.py`, `probe_mod.py`,
  `probe_n8.py` in the S1 scratch directory named in `dsp-research/reverb.md`) is not in the
  repository; before rendering, the facilitator archives the exact scripts used and records their
  SHA-256 here. Also record the engine commit whose kernel the model was checked against
  (`kernel_matches_the_f64_oracle`).
- Complete processor/session parameters: `miso.reverb` defaults for every candidate:
  `pre_delay_ms` 20, `decay_ms` 1,800, `size` 1.0, `damping_hz` 6,000, `low_cut_hz` 20,
  `diffusion` 0.7, `width` 1.0, `mix` 1.0 (wet only, as on an aux return). Q3 changes `size`
  only, in two histories rendered for each item: (a) a sweep 0.5 to 2.0 over 2 s as one event per
  128-frame block, starting 1 s into the item; (b) one step 0.5 to 2.0 at 1 s. Q3 uses (a) for
  items drums and vocal and (b) for piano and pad, fixed now.
- Presentation: each stimulus is the dry item plus the wet return at -6 dB, summed after the
  render; the dry part is bit-identical between A and B.
- Sample rate and render quantum: 48,000 Hz and 128 frames.
- Stimulus provenance and license: four licensed excerpts chosen before blinding, one each of
  drums (a dry kit with snare and toms), vocal (a dry solo voice with consonants), piano (solo,
  sustained and staccato notes) and pad (a sustained synthesizer with slow harmonics), 10 s each,
  48 kHz, mono or stereo source. Record title, owner, licence and excerpt times for each.
  Unassigned.
- Fixture paths and CRC-32C values: the four excerpts and the rendered candidates are not
  repository fixtures; record each file's SHA-256 and CRC-32C before randomization.
- Render hashes: record both candidates' hashes per (question, item) before randomization: 12
  pairs in all.
- Level/gain matching method and measured tolerance: integrated loudness (ITU-R BS.1770-5) of the
  two candidates of a pair matched within 0.1 LU by a gain on the whole stimulus, recorded before
  concealment; program level -23 LUFS.
- Peak/clipping check: every presented stimulus below -1 dBFS true peak after matching; a pair
  that fails is matched lower, never limited.

## Blinding and randomization

- Facilitator or automation owner: unassigned.
- Anonymized listener ID and relevant experience: three listeners, `L1`-`L3`, each with mixing or
  critical-listening experience stated in one line. Unassigned.
- Randomization algorithm/version and seed: SplitMix64-v1; one seed per listener, recorded before
  the first trial. It draws X per trial and the order of items within a session; the question
  order across sessions is a Latin square over the three listeners.
- Candidate mapping held by: facilitator until reveal; the listener's tool shows A, B and X only.
- Training/familiarization: before each session, the listener may play the two labelled
  candidates of one item not used in that session's scored trials (for Q3, a fifth excerpt of the
  same kind), with no scored answers.
- Reveal time and reveal-log location: append to the completed record only after all 576 valid
  trials and all 36 preferences.

## Playback chain and conditions

- Source format: 48 kHz 24-bit or float WAV made from the rendered `f32` PCM, no resampling.
- DAC/interface, driver, and operating mode: unassigned; record bit-transparent mode.
- Transducer and calibration method/level: closed or open-back headphones, one model for all
  listeners; -23 LUFS pink noise calibrated to 75 dB SPL (A) at the ear simulator or by the
  facilitator's stated method. The listener may not change the level.
- Room or headphone conditions: a quiet room; record the background level.
- Background or environmental notes: unassigned. Breaks between sessions of at least 15 minutes.

## Raw responses

No human trial has been run. A completed record contains 576 ABX rows (question, item, listener,
trial) and 36 preference rows, with the hidden assignment withheld until reveal. Synthetic answers
are prohibited.

| Trial | Hidden assignment | Listener answer | Correct after reveal | Confidence | Observation |
|---:|---|---|---|---:|---|
| 1 | withheld |  |  |  |  |

## Result and bounded conclusion

- Counts and computed result: pending; no result exists.
- Confidence interval/p-value where applicable: pending; no result exists.
- Adverse observations and known confounds: pending. Known now: the candidates are model renders,
  not the shipped kernel (the model matches the kernel only within the oracle test's tolerance);
  Q3 hears the two ring policies under one size history per item only; three listeners bound the
  claim to them.
- Objective gates already passed (artifact IDs): record the commit at which tests 1-8 of
  `dsp-research/reverb.md` passed.
- Conclusion limited to the preregistered question: none before completion.
- Reproducibility artifact locations: this preregistration, the archived scratch scripts and the
  later completed record.
- Conflicts of interest: record before completion.

## Sign-off

- Facilitator: unassigned.
- Listener: unassigned.
- Reveal verifier: unassigned.
