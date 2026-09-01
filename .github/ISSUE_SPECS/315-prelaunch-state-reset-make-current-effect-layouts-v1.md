# Prelaunch state reset: make current effect layouts V1

**Status: SOL PASS.**

## Owner ruling

The engine is prelaunch. A production effect state layout that currently identifies itself as
version 2 is not a published second generation; reset its current identity to version 1 while
preserving its actual payload shape and runtime behavior.

## Smallest closable product slice

Reset the five current native effect state layouts above one to version 1:

- parametric EQ;
- gate/expander;
- true-peak limiter;
- multiband compressor; and
- soft clip.

Update descriptors, payload headers, golden fixtures, tests, active explanatory prose, and mutation
witnesses that directly encode those production layouts.

Synthetic multi-revision migration-framework operands, generic package test layouts,
tooling/benchmark record schemas, historical issue/evidence records, and mathematical variables
are separate inventory classes and are not changed here.

## Decision record

1. The payload field remains structurally versioned because restore must reject incompatible bytes.
2. The sole current prelaunch layout is V1 even when its shape evolved during development.
3. No compatibility reader for the superseded prelaunch number is retained.
4. Payload shape and DSP state semantics do not change; only the current version identity and its
   directly derived witnesses change.
5. Migration-framework tests retain multiple numeric revisions in this slice because they qualify
   the generic migration mechanism rather than assert a live effect identity.

## Objective gates

- All current native effect descriptors report state layout version 1.
- Save/restore remains byte-shape preserving apart from the intended version word reset.
- Golden payloads/digests are regenerated from canonical producers, not hand-waved.
- Each affected effect's focused contract/state/product tests pass.
- Workspace check and the relevant effect-compiler observation/restore gates pass.
- A live-tree audit finds no production effect descriptor or `StateLayout` above version 1.

## Workflow

Sol briefs and approves this stateless scope. Terra attempt 1 performs the bounded reset and records
focused evidence. Sol adversarially reviews payload compatibility, fixture provenance, and scope
containment before PASS.

## Evidence record

- Sol brief approved; GitHub issue #315 matches this stateless local spec.
- Terra attempt 1 resets the five production layout constants/descriptors, in-payload header words,
  restore call sites, stale-version mutations, and active contract prose to V1 without changing any
  payload length or state word beyond the header's version word.
- The complete test targets for parametric EQ, gate/expander, multiband compressor, and soft clip
  pass; the true-peak limiter's 29 library tests pass. These cover scalar and bank save/restore,
  transactional corruption rejection, determinism, conformance, and allocation witnesses.
- The effect compiler's launch-registry assertion now requires every dynamics state layout to be
  V1 and passes. `cargo check --workspace --all-targets`, formatting, and diff hygiene pass.
- Sol review found that the first registry assertion covered only four dynamics effects. The
  corrected gate now traverses every launch-native descriptor and requires state layout V1; it
  passes, covering parametric EQ and soft clip as well as the dynamics set.
- The updated parametric-EQ header mutation was executed: forcing the current layout to stamp
  invalid version 0 makes `a_payload_with_a_stale_header_is_rejected_on_its_own_evidence` fail 0
  versus 1. Restoring the implementation makes all 16 contract tests pass.
- Sol adversarial review PASS: all launch-native descriptors are gated at state layout V1; payload
  sizes and state semantics are unchanged; each stale-version test rejects 0 rather than the now
  current 1; the only changed serialized word is the intended self-describing version header.
  Generic migration/package fixtures and tooling record schemas remain successor inventory.
