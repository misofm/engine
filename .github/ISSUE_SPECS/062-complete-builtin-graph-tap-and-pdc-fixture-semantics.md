# 062 Complete builtin graph-tap and PDC fixture semantics

## Outcome

Replace stopped Issue 060's vacuous graph fixture with one independently checked seven-boundary
graph-tap/output fixture that also proves fixed positive-latency PDC coexistence.

## Context

Checkpoint `10f0235` clears all rack effects, repeats intermediate tap values, has no positive-
latency side route and derives only the PostMatrix summary from candidate PCM. Issue 035 instead
froze pairwise-distinguishable production graph boundaries, exact output and PDC coexistence. This
issue repairs only those existing graph-tap payloads and checker semantics.

It permits exactly one Terra attempt and one bounded Sol correction/review. A second failure stops.
Workload, timing and benchmark invocations are forbidden and remain zero.

## Scope

Use the accepted session/compiler/graph interfaces and deterministic fixture processors to make
Input, PostInputBuiltins, PostSimd1, PostDynamic, PostSimd2PreFader, PostFader and PostMatrix
pairwise distinguishable. Add a fixed positive-latency side route to the fixture session and prove
integer PDC. Regenerate only the existing graph-tap PCM/meter payloads and their manifest entries.

## Required public interfaces/contracts

The authoring path renders a compiled/bound production graph; the read-only checker uses a
separate closed-form/retained-`f32` model of the frozen source, builtin filters, deterministic rack
transforms, fader/matrix and delay alignment. It must not use candidate PCM to derive expected
output or candidate meter records to derive expected snapshots.

## Deliverables

One corrected `pcm/graph-taps.f32le`, one corrected seven-record
`meters/graph-taps.jsonl`, exact independent checker and focused semantic mutations.

## Explicit non-goals

Production effect DSP, realtime million-render audit, swap/retirement lifecycle, general graph
fixtures, scalar PCM, other JSONL classes, final 24-format seal, targets, benchmarks or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

## Acceptance gates with objective measurements

- The exact seven stable tap identities occur once each in canonical order and their left/right
  sample summaries are pairwise distinguishable at the frozen fixture boundary.
- Independent expected PCM and all seven exact meter snapshots match; changing any tap field or
  output word with a recomputed manifest is rejected.
- The fixture includes one fixed positive-latency route and proves its exact integer PDC in both
  output alignment and compiled graph metadata without creating a realtime audit.
- The valid checker remains read-only; focused graph/compiler/fixture tests, format,
  warning-denied relevant-package Clippy and diff checks pass.

## Required evidence

Fixture topology and latency; exact PCM/meter/manifest hashes; seven distinct snapshot identities
and values; mutation results; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.
