# 056 Complete independent builtin corpus and corruption proof

## Outcome

Complete and seal the independent checked builtin fixture corpus on top of Issue 035's accepted
read-only manifest/path validator, without running realtime audits or benchmarks.

## Context

Issue 035 stopped after its Terra and Sol attempts. Clean checkpoint
`0edc51c6ff60aa8f4a31df73cf73bc2b52e4436e` is accepted technical input only: it provides a
typed, read-only V1 manifest/tree checker and executed proof that all 24 format-class
delete/alter/add/manifest-valid-coverage-hole mutations reject. It does not establish independent
expected-output provenance or complete machine qualification.

This stateless successor permits exactly two total attempts: Terra attempt 1 and one bounded Sol
correction/review. A second failure stops. Launch-rate scope is exactly 44,100, 48,000, 88,200 and
96,000 Hz; higher rates are out of scope. Timed/workload benchmark invocations are forbidden and
start at zero.

## Scope

Complete the checked `fixtures/builtins/v1` corpus and its non-production oracle path: expanded
cases, response CSV, canonical PCM, meter JSONL, diagnostics, resources, metadata and the ten
already-frozen benchmark input bundles. Make `--check` read-only and independent of production
fixture regeneration. Preserve the exact response grid, functional tuples, file formats and
tolerances frozen in Issue 035; do not add a second corpus or broaden the matrix.

## Required public interfaces/contracts

`miso_engine_builtins_fixture --check FIXTURE_DIRECTORY` parses and validates existing bytes only;
it never calls production DSP to recreate expected output and never writes. `--write` remains an
explicit off-repository scratch authoring tool and cannot make a generated/observed candidate its
own oracle. Every manifest entry is a regular safe relative path with exact length/hash, and every
required tuple resolves to one checked payload.

## Deliverables

Completed V1 corpus and manifest; independent-oracle provenance; compact coverage/corruption tests;
and a deterministic coverage/hash report.

## Explicit non-goals

Production DSP changes, realtime audits, graph lifecycle proof, target builds, object inspection,
benchmark schema/runner/preflight/timing, listening, or V1/legacy inspection.

## Dependencies by exact issue title

- Issue-007 builtin qualification tooling, audits, and benchmark
- Representable TPT cutoff domain and builtin contract acceptance
- DSP research corpus and conformance harness

## Acceptance gates with objective measurements

- The independent response corpus covers the exact four launch rates and Issue-035 quanta,
  section/cutoff/probe grid and frozen analytic/finite-window/sustained/tail tolerances.
- Every declared functional PCM, meter, diagnostic and resource tuple is present, canonical and
  checked; all ten benchmark input bundles are complete inputs, not timed results.
- Manifest deletion/grammar/order/path/length/hash/unlisted-file mutations reject. For TOML,
  `f32le`, CSV, meter JSONL, diagnostics JSONL and resources JSONL, delete/alter/add/manifest-valid
  coverage-hole mutations all reject: exactly 24/24.
- A source/static test proves `--check` cannot reach production fixture generation or write APIs.
- Focused fixture/reference tests, format, warning-denied package Clippy and applicable
  nonbenchmark workspace/policy checks pass.

## Target matrix

Host-executed deterministic tooling over fixtures for exactly 44.1/48/88.2/96 kHz. Cross-target
engine qualification belongs to the next successor.

## Required evidence

Candidate and manifest hashes; independent-oracle dependency/provenance check; exact row/path and
24/24 mutation counts; tolerance maxima; read-only/no-production-reachability proof; commands and
strict Terra/Sol verdicts; `workload_invocations=0`; `timed_benchmark_invocations=0`.
