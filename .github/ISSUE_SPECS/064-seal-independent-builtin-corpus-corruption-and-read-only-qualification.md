# 064 Seal independent builtin corpus corruption and read-only qualification

## Outcome

Join the three completed builtin corpus semantics slices and seal one immutable, read-only checked
corpus through the exact 24/24 corruption matrix and final nonbenchmark policy gates.

## Context

Stopped Issue 060 could not combine response/scalar PCM, graph/PDC, typed JSONL and final seal in
one correction. Issues 061–063 independently close those semantic surfaces. This issue changes no
expected DSP value; it validates their one joined corpus and becomes the sole corpus dependency of
Issue 057.

It permits exactly one Terra attempt and one bounded Sol correction/review. A second failure stops.
Workload, timing and benchmark invocations are forbidden and remain zero.

## Scope

Join the exact accepted Issue-061/062/063 payloads without regeneration or tuning. Execute the
frozen six-class, four-mutation corruption matrix; prove the supplied-root checker cannot reach
generation/production-render/write APIs and leaves a valid tree byte-identical; then run the
focused nonbenchmark repository/policy seal.

## Required public interfaces/contracts

`miso_engine_builtins_fixture --check FIXTURE_DIRECTORY` reads and validates supplied regular
files only. It never calls `generated`, authoring, production rendering or filesystem writes. One
accepted manifest identifies exactly 50 payloads and all checker reports are deterministic.

## Deliverables

Exactly 24 meaningful semantic corruption results, read-only/no-production-reachability proof,
complete candidate/manifest coverage report and strict final corpus verdict.

## Explicit non-goals

Expected-value changes, new cases/payloads/formats, production DSP, realtime audits, graph
lifecycle, targets, instructions, benchmark runner/workload/timing, performance or listening.

## Dependencies by exact issue title

- Complete builtin response cases and scalar PCM semantics
- Complete builtin graph-tap and PDC fixture semantics
- Complete builtin meter, diagnostic, and resource corpus semantics

## Acceptance gates with objective measurements

- The joined corpus has exactly 50 manifest-listed payloads and all accepted semantic checks pass
  without changing bytes.
- For each of TOML, `f32le`, CSV, meter JSONL, diagnostics JSONL and resources JSONL, delete, byte
  alter, unlisted add and manifest-valid semantic coverage hole reject: exactly 24/24.
- Every coverage hole removes one required tuple/path while leaving the payload syntactically
  valid and recomputing its manifest entry; an empty file or stale-manifest rejection does not
  count.
- Static/unit call-graph proof shows `--check` cannot reach generation, production rendering or
  writes; complete tree hashes before and after a valid check are identical.
- Focused fixture/reference tests, format, warning-denied package Clippy and applicable
  nonbenchmark workspace/policy checks pass on one clean candidate.

## Required evidence

Accepted dependency commits; candidate, manifest and payload hashes; exact case/row/path/record
counts; all 24 class/mutation/error identities; read-only proof; strict Terra/Sol verdicts;
`workload_invocations=0`; `timed_benchmark_invocations=0`.
