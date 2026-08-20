# Synthetic A/A listening-record format example

## Identity and status

- Schema version: 1
- Record ID: `issue-002-synthetic-aa-format-v1`
- Evidence kind: synthetic format example
- Status: complete
- Date and time (UTC): 2026-08-20T00:00:00Z
- Sound-quality claim: none

## Preregistered question

- Hypothesis: demonstrate that every record field and reveal step can be represented; do not test audibility.
- Test type: synthetic ABX A/A dry run
- Trial count: 4 synthetic rows
- Stopping rule fixed before reveal: exactly four rows, no extension
- Statistical method and confidence interval: none; responses are generated examples, not observations
- Multiple-comparison correction, if any: N/A — no inferential test is performed

## Exact candidate and stimulus identity

- Candidate build IDs, commit hashes, target, and target features: `synthetic-a` and `synthetic-a-copy`; no executable build
- Complete processor/session parameters: identity, no parameters
- Sample rate and render quantum: 48,000 Hz, 128 frames
- Stimulus provenance and license: issue-002 asymmetric impulse, repository-owned synthetic data
- Fixture paths and CRC-32C values: `fixtures/conformance/v1/rate-048000-impulse-dual-mono.mepcm`; value comes from `MANIFEST.tsv`
- Render hashes: N/A — this is a format-only example
- Level/gain matching method and measured tolerance: byte-identical A/A declaration
- Peak/clipping check: synthetic fixture peak is at or below 1.0

## Blinding and randomization

- Facilitator or automation owner: synthetic example generator
- Anonymized listener ID and relevant experience: `SYNTHETIC-NOT-A-HUMAN`
- Randomization algorithm/version and seed: SplitMix64-v1, `0x4D49534F454E4732`
- Candidate mapping held by: synthetic reveal table below
- Training/familiarization: N/A — no human session occurred
- Reveal time and reveal-log location: fixed example, this file

## Playback chain and conditions

- Source format: planar f32 PCM fixture
- DAC/interface, driver, and operating mode: N/A — no playback occurred
- Transducer and calibration method/level: N/A — no playback occurred
- Room or headphone conditions: N/A — no playback occurred
- Background or environmental notes: N/A — no playback occurred

## Raw responses

| Trial | Hidden assignment | Listener answer | Correct after reveal | Confidence | Observation |
|---:|---|---|---|---:|---|
| 1 | A/A | synthetic A | N/A | 0 | generated row |
| 2 | A/A | synthetic B | N/A | 0 | generated row |
| 3 | A/A | synthetic A | N/A | 0 | generated row |
| 4 | A/A | synthetic B | N/A | 0 | generated row |

## Result and bounded conclusion

- Counts and computed result: four schema rows; no listener responses
- Confidence interval/p-value where applicable: N/A — synthetic data cannot support inference
- Adverse observations and known confounds: no acoustic event or human listener exists
- Objective gates already passed (artifact IDs): format structure only
- Conclusion limited to the preregistered question: the template can represent a completed record
- Reproducibility artifact locations: this file and `listening/TEMPLATE.md`
- Conflicts of interest: none; no evaluative claim

## Sign-off

- Facilitator: synthetic example
- Listener: `SYNTHETIC-NOT-A-HUMAN`
- Reveal verifier: synthetic example
