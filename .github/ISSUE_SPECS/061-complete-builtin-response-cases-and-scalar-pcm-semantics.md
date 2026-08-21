# 061 Complete builtin response cases and scalar PCM semantics

## Outcome

Complete the independent typed checks for the frozen builtin response cases and the 32 non-graph
PCM payloads left by stopped Issue 060.

## Context

Issue 059 accepted zero recovery for every legal response row. Stopped Issue 060 produced the
four-rate zero-recovery response candidate and partial checker at clean checkpoint `10f0235`, but
Sol found unparsed response-case fields, skipped response gates, unchecked scalar PCM and an
incomplete reset fixture. Those bytes and findings are technical input, not acceptance.

This stateless issue permits exactly one Terra attempt and one bounded Sol correction/review. A
second failure stops. Launch rates remain exactly 44,100, 48,000, 88,200 and 96,000 Hz. Workload,
timing and benchmark invocations are forbidden and remain zero.

## Scope

Close only response/case parsing and scalar PCM semantics. Preserve the frozen 1,630-row grid,
1,652 case declarations, numerical tolerances, 50-payload corpus shape and production DSP. Validate
the 32 existing non-graph PCM paths, including the unsuffixed matrix-ramp payload and an executed
fixture sequence covering both builtin reset kinds. Do not edit graph taps or JSONL payloads.

## Required public interfaces/contracts

The read-only checker parses every response-case field and requires the exact canonical tuple
encoded by its ID. Every serialized response measurement is checked against the independent
oracle and the frozen gate applicable to that probe; partitions of one coordinate have identical
measurements and recovery. Scalar PCM expectations use closed-form arithmetic or the accepted
independent retained-`f32` reference and never production regeneration on the check path.

## Deliverables

Typed response-case/CSV checking, complete independent scalar PCM checking, corrected reset
fixture bytes if required, focused mutations for each repaired semantic hole and strict evidence.

## Explicit non-goals

Production DSP, graph/PDC fixtures, meter/diagnostic/resource JSONL, final 24-format corruption
seal, audits, targets, instruction inspection, benchmarks, timing or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Issue-007 builtin qualification tooling, audits, and benchmark
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- Exactly 1,630 response rows and 1,652 cases retain the frozen four rates, five quanta,
  section/cutoff/probe grid and zero recovery.
- Every response case has exactly its canonical ID/category/rate/quantum/section/cutoff/probe/oracle
  tuple; every CSV numeric field has the frozen canonical decimal representation.
- Analytic, cast-state, one-second impulse and final-4096 tail gates run wherever frozen; coherent
  rows additionally run the sustained fundamental/residual/total gates. All five partitions agree
  exactly for one render coordinate.
- All 32 non-graph PCM paths are independently checked. The unsuffixed ramp is no longer
  manifest-only, and the reset fixture executes and proves both `DiscontinuityKeepTargets` and
  `FullToPrepared` without adding a payload.
- Focused fixture/reference tests, format, warning-denied package Clippy and diff checks pass.

## Required evidence

Exact row/case/PCM counts and hashes; tolerance maxima; partition equality and recovery total zero;
mutation identities; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.
