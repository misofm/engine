# 033 Issue-007 builtin filter and matrix human listening qualification

## Outcome

Complete the real blinded listening evidence for the exact machine-qualified issue-007 builtin
candidate before any launch or audible-quality claim.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. Sound quality and DSP correctness are release criteria.
Objective fixtures and machine conformance do not replace documented listening, and a coding
agent cannot fabricate human listeners, playback conditions or trial responses.

Issue 007 deliberately ends at a sealed *machine-qualified, human-listening pending* candidate.
This issue owns the two preregistered real listening executions against that exact candidate. It is
a hard dependency of **End-to-end release, performance, and listening qualification**. Closing
issue 007 does not imply this issue passed and does not authorize a launch claim.

This issue follows the Sol-approved brief → human/facilitator execution with raw evidence → Sol
adversarial review workflow. Any candidate code, coefficient, fixture or render-input change
invalidates the affected record and requires a new corrective implementation issue; do not edit
answers or silently substitute a new build.

## Scope

Execute and document the existing issue-007 filter ABX and matrix-ramp randomized A/B
preregistrations with real human participants, exact sealed artifacts, reproducible rendering,
level/peak checks, concealed randomization, complete raw answers, reveal logs and bounded
conclusions.

## Required public interfaces/contracts

No engine runtime interface is added. Publish a checksummed `Issue007ListeningQualificationV1`
record index naming the candidate commit and benchmark artifact, fixture/render hashes, listener
and facilitator pseudonymous IDs, playback chain, calibration, randomization/reveal artifacts,
protocol deviations, raw record paths and final launch disposition.

## Deliverables

- completed `issue007-filter-abx-v1` record with exactly 20 valid trials;
- completed `issue007-matrix-ramp-v1` record with exactly 20 valid trials;
- concealed assignment and post-trial reveal logs with hashes;
- exact candidate PCM renders and provenance/license record for every stimulus;
- statistical calculation output and a checksummed qualification index; and
- PASS/FAIL launch disposition with adverse observations and corrective issue links.

## Explicit non-goals

Changing DSP, tuning a tolerance from listener answers, claiming universal preference,
certifying loudness, replacing objective conformance, accepting synthetic trials, or treating an
agent/facilitator as a human listener.

## Dependencies by exact issue title

- Dual-mono builtins and metering
- DSP research corpus and conformance harness

## Frozen procedure requirements

Before the first answer is collected, the facilitator records:

- exact clean candidate commit, target/features, issue-007 benchmark SHA-256 and fixture manifest;
- comparator identity and complete filter/matrix/session parameters;
- both candidate render hashes, RMS match within 0.1 dB and peaks below 0 dBFS;
- SplitMix64-v1 seed, assignment owner, concealment location and stopping rule;
- pseudonymous listener/facilitator IDs and conflicts, playback hardware/driver/mode, transducer,
  calibration level/method, room/headphone conditions and environmental notes; and
- stimulus provenance and permission sufficient to retain the evidence or an exact reproducible
  private-artifact reference when redistribution is prohibited.

The filter procedure is the preregistered 20-valid-trial ABX with an exact two-sided binomial
p-value and 95% confidence interval. The matrix procedure is the preregistered 20-valid-trial
randomized A/B with preference counts and a 95% interval. No answer-bearing training trial counts;
invalid trials are retained with a reason and do not alter the fixed 20-valid-trial stopping rule.
Assignments remain concealed until all valid answers are immutable and hashed.

Statistical distinguishability or preference alone is descriptive and is not a superlative sound-
quality gate. Any repeatable click, pop, image jump, gain discontinuity, instability or other
adverse observation attributed to the candidate is launch-blocking until a stateless corrective
issue resolves it and a newly applicable listening record passes. A significant ABX result without
an adverse defect receives a bounded explanation; it is not relabeled as proof of better/worse
sound.

## Acceptance gates with objective measurements

Both records contain exactly 20 valid human responses, all required identities/conditions, raw
answers, immutable pre-reveal hashes, matching post-reveal assignments, reproducible statistical
outputs and three sign-offs (facilitator, listener and reveal verifier). Automated validation
rejects `preregistered` status, pending/unassigned placeholders, missing rows, duplicate trial IDs,
answers added after reveal, mismatched hashes, candidate/fixture drift, out-of-tolerance level/peak
checks, absent provenance, and synthetic/agent listener declarations.

The qualification PASS requires both procedures to be complete with no unresolved attributable
adverse observation and no protocol deviation capable of unblinding or changing the stopping
rule. Preference for the candidate and a nonsignificant ABX result are not required. Any unresolved
adverse observation, incomplete real trial set, candidate drift or material protocol deviation is
FAIL and blocks issue 026.

## Target matrix

One pinned native issue-007 candidate at 48 kHz/128 frames for both preregistered procedures. The
playback chain is recorded exactly; results are not generalized to untested rooms, transducers,
listeners, rates or targets.

## Required evidence

Checksummed qualification index; completed template-conformant records; raw answer and reveal
logs; exact build/benchmark/fixture/render hashes; level and peak measurements; playback/calibration
record; statistical tool/version/output; adverse-observation disposition; sign-offs; and any
corrective issue links. Concise source paraphrases use the repository listening methodology; no
subjective superlative is an acceptance gate.
