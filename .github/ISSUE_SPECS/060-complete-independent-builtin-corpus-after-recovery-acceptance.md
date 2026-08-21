# 060 Complete independent builtin corpus after recovery acceptance

## Outcome

Complete and seal the one independent builtin V1 corpus after Issue 059 freezes the cascade
decay/recovery contract.

## Context

Issue 056 stopped after its fixed response candidate exposed unresolved repeated cascade recovery.
Its clean benchmark-input checkpoint `3aeb39c` and failed response candidate remain technical input,
not acceptance. This issue starts only after **Builtin cascade decay and recovery contract** passes
and consumes its exact candidate and executable recovery rule.

It permits exactly one Terra attempt and one bounded Sol correction/review. A second failure stops.
Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz. Workload and timed benchmark
invocations are forbidden and start at zero.

## Scope

Finish the single `fixtures/builtins/v1` corpus and read-only checker: accept the frozen response
grid under Issue 059's recovery rule, complete the already-declared PCM, meter, diagnostic,
resource and metadata tuples, retain the ten benchmark input bundles, and seal exact manifest
coverage and the existing 24/24 format corruption matrix. Preserve Issue 035's file formats,
functional cases, four-rate response grid and numerical tolerances.

## Required public interfaces/contracts

`miso_engine_builtins_fixture --check FIXTURE_DIRECTORY` reads and validates existing bytes only,
never regenerates expected output and never writes. Every expected value has independent-oracle or
exact closed-form provenance. Every required tuple maps to one manifest-checked safe regular path.

## Deliverables

Completed V1 corpus and manifest, deterministic coverage/hash report, independent provenance
proof, all declared tuple checks and 24/24 semantic corruption rejection.

## Explicit non-goals

Production DSP redesign, recovery-contract research, realtime audits, graph lifecycle proof,
target/instruction qualification, benchmark schema/runner/workload/timing, or listening.

## Dependencies by exact issue title

- Builtin cascade decay and recovery contract
- Issue-007 builtin qualification tooling, audits, and benchmark
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- The exact four-rate response grid passes the frozen analytic/finite-window/sustained/tail gates
  and Issue 059 recovery rule for every probe and partition.
- Every declared PCM, meter, diagnostic, resource, metadata and benchmark-input tuple is present,
  canonical, independently checked and manifest-covered.
- Manifest and six-format delete/alter/add/manifest-valid-coverage-hole mutations reject, exactly
  24/24 for the format matrix.
- Static/unit proof shows `--check` cannot reach production generation or write APIs, and a valid
  check leaves the corpus byte-identical.
- Focused fixture/reference tests, format, warning-denied package Clippy and applicable
  nonbenchmark workspace/policy checks pass.

## Required evidence

Candidate and manifest hashes; exact row/path/tuple and mutation counts; response tolerance maxima
and recovery totals; provenance/read-only proof; strict Terra/Sol verdicts;
`workload_invocations=0`; `timed_benchmark_invocations=0`.
