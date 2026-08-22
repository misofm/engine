# 072 Separate builtin benchmark render timing from meter evidence collection

## Sol briefing checkpoint — 2026-08-22

**READY FOR TERRA ATTEMPT 1; NO PREFLIGHT OR WORKLOAD IS AUTHORIZED.** The authoritative brief is
`BRIEFS/072-separate-builtin-benchmark-render-timing-from-meter-evidence-collection.md`. This
stateless successor permits one Terra implementation/review attempt and one bounded Sol
correction/review; a second failure stops. At briefing, Issue-072 counters are
`preflight_invocations=0`, `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`. The consumed Issue-058 run remains historical evidence and is not
reset or reclassified.

## Outcome

Correct the proven builtin benchmark harness boundary so only product render is audited and timed,
while meter draining and all evidence hashing occur allocation-free or off-scope as frozen. Seal
the corrected candidate, then perform one new no-retry preflight and exactly one no-retry
one-warmup/two-measured-round descriptive run.

## Context

Issue 058 stopped after its sole authorized runner invocation aborted at the first
`meter_success_full` warmup. `RealMeterTapRuntime::render_one` called
`drain_all().collect::<Vec<_>>()` inside `audit::in_render_scope`; the audited global allocator
aborted with status 134. The same path placed queue draining and evidence hashing inside the timed
operation. Issue-058 checkpoint `bd17fc1` and its preserved artifacts are technical input, not a
PASS dependency.

This issue does not reopen the accepted product, corpus, audit, target, instruction, schema, or
workload decisions. It changes only the benchmark harness boundary and the successor-owned
artifact lifecycle needed because the failed Issue-058 paths cannot be overwritten.

## Scope

- Split every render operation into outside-timing input preparation, render-only audited/timed
  execution, and outside-timing evidence collection.
- Drain the seven success meter consumers directly without `Vec`, collection, allocation, or any
  other transient retained container. Keep the capacity-one full plan prefilled and record its
  final snapshot/drop evidence outside render timing.
- Hash all direct PCM, graph PCM, meter snapshots, tap identities, counters, and continuation
  evidence only after the elapsed interval and after the render audit is disarmed.
- Add a compact nonexecuting test that exercises every render workload with the audited allocator
  armed only around product render and proves zero audit counters, off-scope drain/hash, exact
  seven-tap behavior, and unchanged output identities across the two prepared round states.
- Retarget the existing preflight/runner lifecycle to new Issue-072 artifacts without changing the
  frozen Issue-035 record schema.
- After all nonexecuting gates pass on a clean committed candidate, consume exactly one preflight
  authorization and, only after Sol validates its seal, exactly one runner authorization.

## Required public interfaces and artifact contract

No engine or DSP API changes are permitted. `miso_engine_builtins_bench` remains a private tooling
binary and the public runner remains a no-argument fixed entrypoint.

Measured JSONL remains byte-schema compatible with Issue 035:

- `schema_version=2`, `issue=35`, and `issue035.<kind>.<rate>hz.q128` IDs;
- five kinds × rates 48,000/96,000 × rounds 1/2 = exactly 20 records;
- one global untimed warmup and exactly two measured rounds;
- unchanged checked TOML/PCM inputs, output-hash definitions, audit fields, metadata mapping,
  nearest-rank percentiles, operation counts, and no threshold.

Successor-owned paths are exactly:

- `target/issue72/miso_engine_builtins_bench`;
- `target/issue72/builtins-benchmark.preflight.json`;
- `target/issue72/builtins-benchmark.raw.jsonl`;
- `target/issue72/builtins-benchmark.jsonl`;
- `target/issue72/builtins-benchmark.validator.stderr`; and
- `target/issue72/builtins-benchmark.disposition.json`.

The Issue-072 preflight and disposition use `issue=72`; measured records retain `issue=35`. The
runner accepts no arguments, path overrides, environment-selected binary, retry, or resume. It
launches the sealed binary once and preserves raw/stderr/disposition on every outcome.

## Preserved Issue-058 failure evidence

Do not delete, rewrite, move, truncate, or reuse any `target/issue35` path. Before and after every
Issue-072 preflight/run, verify:

- binary 3,191,104 bytes, SHA-256
  `242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944`;
- preflight seal 2,211 bytes, SHA-256
  `85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d`;
- raw and validator stderr each 0 bytes, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- accepted output absent; and
- FAIL disposition 974 bytes, SHA-256
  `e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce`.

## Immutable dependency identities

Preserve the Issue-058 frozen manifest, ten benchmark-input hashes, graph PCM/meter, accepted
audit/trace/target identities, record and aggregate validators, launch-rate scope, and permitted
benchmark-only lock transition. In particular:

- manifest `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
- graph PCM `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
- graph meter `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`;
- accepted Issue-068 source manifest
  `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`;
- candidate lock before implementation
  `da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`; and
- frozen benchmark-stanza-only lock diff
  `5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`.

No dependency, version, source, checksum, fixture, input, record schema, workload or product code
may change. Any required change beyond the benchmark crate and its direct scripts is a STOP.

## Deliverables

- corrected render/timing/evidence boundary in the existing benchmark tool;
- focused armed-render and off-scope evidence tests;
- successor-owned no-clobber preflight/runner lifecycle and hermetic stub coverage;
- one sealed Issue-072 preflight record with zero runner/workload/timed counters; and
- only after separate Sol authorization, one raw/accepted/stderr/disposition set for the exact
  20-record descriptive run.

## Explicit non-goals

Product DSP/core/graph/runtime/session changes; corpus, benchmark-input, schema, rate, workload,
operation-count, tolerance, audit, target, instruction, metadata, or percentile changes; a general
meter API; performance tuning; thresholds; retries; resume; direct binary execution; listening;
or altering Issue-058 artifacts.

## Dependencies by exact issue title

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

Stopped Issue 058 is consumed only as the exact technical checkpoint and failure evidence above;
it is deliberately not a PASS dependency.

## Ordered acceptance gates

1. Static review proves the Issue-058 allocator-abort path is removed without changing product
   render, inputs, schema, validators, rates, operation counts, or output-hash content/order.
2. Focused tests execute all four render workloads under the audited allocator without timing or
   benchmark main: product render alone is armed, every audit category is zero, the success queue
   drains seven exact taps outside scope, the full queue stays full/drops exactly, evidence hashing
   is outside scope, both warmup states remain identical, and no allocation is hidden by fallback.
3. Format, locked benchmark-package check/tests, warning-denied Clippy/rustdoc, read-only fixture
   validation, complete synthetic validator/lifecycle suite, locked workspace check/tests, and
   applicable workspace/realtime/builtins/graph/rack policy and mutation gates pass. Static scans
   prove benchmark main, public runner, preflight, audit/trace/target workload and timing counts are
   zero during implementation/review.
4. On a clean committed candidate with `target/issue72` absent and Issue-058 artifacts unchanged,
   Sol may authorize exactly one invocation of `bash scripts/preflight-builtins-benchmark.sh`.
   Failure consumes the authorization and stops; no retry. Preflight never executes the binary.
5. Sol validates the exact candidate/source/lock/tool/corpus/failure-artifact/binary seal, zero
   counters, and absence of Issue-072 raw/accepted/stderr/disposition. Only then may Sol authorize
   exactly one invocation of `bash scripts/run-builtins-benchmark.sh`.
6. The sole runner invocation must emit exactly 20 validator-valid rows, one warmup and two rounds;
   preserve raw bytes; atomically publish a byte-identical accepted copy; record complete host and
   audit metadata; and write a PASS disposition with Issue-072 counters exactly `1/1/1`. Any
   failure consumes authorization and is final STOP.

## Required evidence and disposition

Record attempt owner/count; clean candidate/source/lock/tool/binary seals; all immutable hashes;
before/after Issue-058 artifact hashes; focused armed-render transcript; package/workspace/policy
gates; preflight authorization and seal; sole runner authorization; exact artifact sizes/hashes;
20-row cardinality; one warmup/two rounds; zero render violations; no-threshold statement; and
strict PASS/FAIL. PASS produces the machine-qualified candidate for Issue 033 but does not claim
human listening, release readiness, capacity, or performance superiority.

