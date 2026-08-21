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

## Sol correction evidence — 2026-08-21

**PASS.** The first correction failure was a stale aggregate resource table, not a production
resource-report defect. The accepted compiler now retains the independent bank-input candidate in
addition to the scalar processors: the projection therefore owns six processor vectors, ten
stable-ID copies and three processor objects per track, for `6 + 13 * tracks` allocations. Meter
projection independently owns six vectors plus queue header/slots, accumulator and six request-ID
copies per request. Checked arithmetic derives all nine totals, maxima and allocation counts from
named pinned 64-bit-native layout facts; public types and the SPSC payload boundary are ABI-guarded.
The production-generated scratch grid and this independent projection agree exactly.

The sealed payload counts and SHA-256 identities are:

- graph meters: `7`, `07aba03ba575577972a8d13106b4cba73b2d378d0d9a05225afe4d92304ec3db`;
- window/drop meters: `15`, `474a89159cb7cd867b01bf84649bf32982a0795ad48979db8f70affa6453c402`;
- diagnostics: `13`, `f8b43cf86100485711f213608bd3a3bfeade6ee4493b6413eb01dcea4582d6dc`;
- resources: `9`, `429b2a1a413eef7dfc7b80f3763bbdb04ada3eaa5435207c72f34deca2ec316e`.

Executed evidence:

- `cargo run --locked -p miso-engine-builtins-fixture -- --check fixtures/builtins/v1`: PASS;
- `cargo test --locked -p miso-engine-builtins-fixture`: PASS, `6/6`, including manifest-valid
  owned-class tuple removals/field mutations and duplicate/extra/reordered/wrong-variant parser
  mutations;
- `cargo test --locked -p miso-engine-dsp-reference`: PASS, `7` unit and `3` integration tests
  passed with the two separately frozen EQ matrix tests ignored;
- warning-denied all-target Clippy for both focused packages: PASS;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

Verdict is strict PASS for Issue 063 only. Issues 061, 062 and 064 remain outside this checkpoint.
`workload_invocations=0`; `timed_benchmark_invocations=0`.
