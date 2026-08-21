# 058 Builtin benchmark preflight and exactly-once qualification

## Outcome

Seal the nonbenchmark-qualified builtin candidate with the frozen schema-v2 benchmark lifecycle and
one exactly-once descriptive invocation, producing the machine-qualified candidate for listening.

## Context

This issue starts only after **Builtin direct and graph realtime audit and target qualification**
passes. It consumes that exact candidate, corpus, audit and target evidence. It permits one Terra
attempt and one bounded Sol correction; a second failure stops. The engine's launch-rate scope is
exactly 44,100, 48,000, 88,200 and 96,000 Hz. The frozen descriptive benchmark samples only its
declared 48/96-kHz workloads and makes no claim about capacity at the other two launch rates.

`timed_benchmark_invocations=0` and `workload_invocations=0` at issue creation. No workload is
authorized until all schema/runner/preflight and clean-candidate gates pass and root Sol explicitly
authorizes the sole command.

## Scope

Implement/finish only the Issue-035 frozen five-kind/two-rate/two-round schema-v2 benchmark binary,
record/aggregate validators, hermetic mutations, safe no-argument runner, zero-launch preflight,
clean identity seal and the sole exactly-once invocation/artifact disposition.

## Required public interfaces/contracts

`bash scripts/run-builtins-benchmark.sh` is the only workload entrypoint. It accepts no arguments,
contains one fixed launch, refuses overwrite/retry/resume, preserves raw output on every outcome,
validates before atomic byte-identical accepted publication and writes a checksummed disposition.
Preflight and tests use counted stubs/synthetic records only and report zero workload launches.

## Deliverables

Benchmark binary, schema-v2 validators/mutations, safe runner, zero-launch preflight/sealed hashes,
and—only after authorization—raw/accepted/stderr/disposition artifacts with final machine verdict.

## Explicit non-goals

DSP/corpus/audit/target changes, timing thresholds, tuning/optimization, retries, expanded rate or
workload matrices, listening execution, deployment work, or V1/legacy inspection.

## Dependencies by exact issue title

- Builtin direct and graph realtime audit and target qualification

## Acceptance gates with objective measurements

The exact Issue-035 schema rejects every missing/extra/wrong-type/identity/shape/percentile/
metadata/audit/hash/cardinality/cross-round mutation. Hermetic runner tests prove argument,
missing-tool, workload/validator failure, interruption/partial output, overwrite and artifact
lifecycle semantics with zero real launches. Preflight seals clean candidate/binary/corpus/audit/
runner/validator hashes and reports zero launches. Only then may root authorize exactly one
external runner invocation; it emits exactly 20 validator-valid records for five kinds, rates
48/96 kHz and rounds 1/2, preserves byte-identical raw/accepted output and has no threshold or
retry. Any launch or post-launch failure consumes authorization and is strict FAIL.

## Target matrix

One pinned native benchmark host recorded completely in schema metadata. Accepted Issue-057 target
evidence covers all four launch rates/targets and is not rerun here.

## Required evidence

Sealed identities; validator/mutation/preflight transcripts; exact invocation/warmup/round/record
counts; raw/accepted/disposition hashes; environment metadata; strict verdicts and attempt count.
On PASS set `machine_qualification=PASS`, `human_listening_status=pending` with Issue 033,
`workload_invocations=1`, and `timed_benchmark_invocations=1`.
