# 063 Complete builtin meter, diagnostic, and resource corpus semantics

## Outcome

Complete independent typed checking of the frozen builtin meter, diagnostic and resource JSONL
payloads left by stopped Issue 060.

## Context

Issue 060's interrupted Checkpoint-2B candidate explored typed records and an independent meter
recurrence, but it stopped on the exact 13-row diagnostics comparison and never became accepted
evidence. Its checker-only diff is technical input, not authority. This issue owns a fresh bounded
completion of exactly these three JSONL classes.

It permits exactly one Terra attempt and one bounded Sol correction/review. A second failure stops.
Workload, timing and benchmark invocations are forbidden and remain zero.

## Scope

Validate exactly seven graph-tap meter records, fifteen window/drop meter records, thirteen sorted
diagnostic records and nine resource rows. Preserve existing schemas, payload counts and files.
Use an independent meter recurrence and checked resource arithmetic; do not repair graph PCM or
perform the final cross-format corruption seal.

## Required public interfaces/contracts

The read-only checker parses canonical exact-key JSONL records and rejects missing, extra,
duplicated, reordered, malformed or semantically changed tuples after manifest recomputation.
Expected meter results come from an independent scalar recurrence. Diagnostics use frozen stable
code/path/error tuples. Resources use the pinned fixture ABI and checked totals, maximum single
allocation and retained allocation count for the exact 3-by-3 grid.

## Deliverables

Typed canonical parsers, independent expected tuples, focused semantic-hole mutations and strict
evidence for the meter/diagnostic/resource classes.

## Explicit non-goals

Response/cases, scalar or graph PCM authoring, graph/PDC repair, production DSP, final 24-format
seal, audits, targets, instructions, benchmarks, timing or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Issue-007 builtin qualification tooling, audits, and benchmark
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- Meter files contain exactly seven graph and fifteen window/drop canonical records with exact
  identities, order, IEEE bit fields, windows, counters, resets, wrap, drain/drop, discontinuity,
  overflow, hold/decay and sanitation semantics.
- Diagnostics contain exactly thirteen sorted stable case/code/path/error tuples.
- Resources contain exactly nine rows for tracks `1,4,65537` by meter sets `0,1,7`, logical
  capacity four where applicable, with checked total, largest allocation and allocation count.
- A manifest-valid removal or semantic change in each owned class rejects; focused
  fixture/reference tests, format, warning-denied package Clippy and diff checks pass.

## Required evidence

Exact `7/15/13/9` counts and payload hashes; meter-reference provenance; resource arithmetic and
ABI identity; focused mutation results; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.
