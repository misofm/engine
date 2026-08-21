# 056 Complete independent builtin corpus and corruption proof

## Status

**STOPPED / RESCOPED — NO OVERALL PASS.** Terra attempt 1 and the single bounded Sol correction
are exhausted. The accepted benchmark-input checkpoint is clean commit `3aeb39c`; the current
fixed-grid response/checker/CSV candidate is failed technical input only. Issue 059 owns the
cascade recovery contract and any bounded product correction. Issue 060 then owns completion and
sealing of the corpus before audit/target qualification may start.

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

## Terra attempt 1 — benchmark-input checkpoint

The first bounded checkpoint adds the ten frozen input bundles for five workload kinds at 48 and
96 kHz, updates the V1 manifest, and extends the typed checker with exact kind/rate/ID/field and
referenced-PCM manifest-hash validation plus mutation coverage. These files are deterministic
inputs only; no benchmark process or timing path ran.

Focused PASS: package format and locked check; four focused V1 checker tests; warning-denied
all-target Clippy; `scripts/check-builtins-fixtures.sh` over 50 manifest-listed files; and
`git diff --check`. Response/oracle, remaining PCM/meter/diagnostic/resource completeness and the
final corpus seal remain pending. `workload_invocations=0`; `timed_benchmark_invocations=0`.

## Sol correction — response checkpoint and final verdict

The fixed four-rate response grid, independent analytic checks and serialized production
measurements were added without changing the frozen rates, domains or tolerances. The original
checker incorrectly required `recovery_count == 0`; read-only diagnosis proved the legal 44.1-kHz
20-Hz single HPF row reports exactly one subnormal-state recovery per lane, or aggregate `2`.

The bounded correction instead enforced one recovery per enabled section/lane: aggregate maximum
`2` for one HPF or LPF and `4` for the fixed HPF-then-LPF cascade, with exact recovery-count
equality across every probe/quantum row sharing `(rate_hz, section, cutoff_bits)`. The checked
candidate then failed:

```text
reference/filter-response.csv recovery count exceeds one per section/lane:
response-cascade-44100-1-fixed-0 has 34, limit 4
```

The read-only checked-corpus command and focused valid-corpus test both failed on that same row.
Workspace format, warning-denied package Clippy and `git diff --check` passed. The response values,
CSV, manifest, production DSP, tolerances and domains were not changed by the Sol correction.
Whether repeated downstream cascade recoveries are valid deterministic underflow handling or a
production defect is unresolved; therefore the corpus is not sealed and Issue 056 has no overall
PASS. `workload_invocations=0`; `timed_benchmark_invocations=0`; benchmark count remains zero.
