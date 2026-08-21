# Issue 007 matrix-ramp listening preregistration

## Identity and status

- Schema version: 1
- Record ID: `issue007-matrix-ramp-preregistration-v1`
- Evidence kind: real listening
- Status: preregistered
- Date and time (UTC): 2026-08-21T00:00:00Z
- Sound-quality claim: none

## Preregistered question

- Hypothesis: no claim before a completed blinded session; this record only fixes the procedure.
- Test type: randomized A-B
- Trial count: 20
- Stopping rule fixed before reveal: stop after exactly 20 valid trials.
- Statistical method and confidence interval: descriptive preference counts with a 95% interval.
- Multiple-comparison correction, if any: one preregistered matrix-ramp question.

## Exact candidate and stimulus identity

- Candidate build IDs, commit hashes, target, and target features: fill from the committed release artifact before playback.
- Complete processor/session parameters: identity input; 64-sample retargeted bounded 2x2 matrix ramp; comparator fixed before rendering.
- Sample rate and render quantum: 48,000 Hz and 128 frames.
- Stimulus provenance and license: repository-owned deterministic asymmetric-lane impulse and an explicitly licensed music excerpt selected before blinding.
- Fixture paths and CRC-32C values: `fixtures/builtins/v1/meter-window-cases.csv`; SHA-256 is fixed by its manifest.
- Render hashes: record both hashes before randomization.
- Level/gain matching method and measured tolerance: RMS match within 0.1 dB, recorded before concealment.
- Peak/clipping check: both rendered candidates must remain below 0 dBFS.

## Blinding and randomization

- Facilitator or automation owner: unassigned.
- Anonymized listener ID and relevant experience: unassigned.
- Randomization algorithm/version and seed: SplitMix64-v1; seed recorded before the first trial.
- Candidate mapping held by: facilitator until reveal.
- Training/familiarization: no answer-bearing training trials.
- Reveal time and reveal-log location: append only after all 20 valid trials.

## Playback chain and conditions

- Source format: rendered planar f32 PCM converted by an external playback tool.
- DAC/interface, driver, and operating mode: unassigned.
- Transducer and calibration method/level: unassigned.
- Room or headphone conditions: unassigned.
- Background or environmental notes: unassigned.

## Raw responses

No human trial has been run. A completed record must contain 20 rows with the hidden assignment
withheld until reveal; synthetic answers are prohibited.

## Result and bounded conclusion

- Counts and computed result: pending; no result exists.
- Confidence interval/p-value where applicable: pending; no result exists.
- Adverse observations and known confounds: pending.
- Objective gates already passed (artifact IDs): pending release artifact identification.
- Conclusion limited to the preregistered question: none before completion.
- Reproducibility artifact locations: this preregistration and the later completed record.
- Conflicts of interest: record before completion.

## Sign-off

- Facilitator: unassigned.
- Listener: unassigned.
- Reveal verifier: unassigned.
