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

## Attempt evidence and final disposition

- Terra attempt 1 produced a compiling fixture/checker tranche, but its first scratch read-only
  check failed at `pcm/graph-taps.f32le` word 9: production `3eb022d1`, independent model
  `3f56aa6f`. The three conformance delays and compiled 9-sample PDC metadata were correct; the
  model incorrectly supplied external fader/matrix processors even though prepared builtins own
  those nodes.
- Sol correction 2 removed those ineffective external bindings, moved the nonidentity fader into
  the fixture session, modeled its prepared fader and canonical pan operations, retained the exact
  three-sample recurrence, and added a direct expected-output assertion that the transformed early
  route is zero through frame 8 and first nonzero at frame 9. The corrected scratch graph PCM hash
  is `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`.
- The sole corrected scratch author then stopped at the next frozen blocker: both accepted
  `benchmark/meter_success_full-{48000,96000}.toml` inputs pin the old graph PCM hash
  `e07cfb2696b6eb2d8114ab84653186395694ba9c16904b70d8b0238903cad46f`. Updating those benchmark
  payloads would exceed Issue 062's graph-PCM/meter-only deliverables; ignoring the mismatch would
  weaken the existing benchmark-input identity validator.

**FINAL: FAIL / STOPPED. No overall PASS.** The accepted benchmark-input dependency must be
explicitly decoupled or repinned in a separately authorized bounded rescope before this graph
fixture can be sealed. `workload_invocations=0`; `timed_benchmark_invocations=0`.

**RESCOPED:** **Reconcile builtin graph fixture and dependent benchmark input identities** consumes
checkpoint `2bbed6a` and alone owns the graph payload plus dependent benchmark-input identity
transaction. This stopped issue remains historical technical evidence, not a dependency PASS.
