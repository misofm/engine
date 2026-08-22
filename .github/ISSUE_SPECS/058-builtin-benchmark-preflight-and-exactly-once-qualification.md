# 058 Builtin benchmark preflight and exactly-once qualification

## Sol briefing checkpoint — 2026-08-22

**READY FOR TERRA ATTEMPT 1, WITH NO BENCHMARK AUTHORIZATION.** The authoritative tracked brief is
`BRIEFS/058-builtin-benchmark-preflight-and-exactly-once-qualification.md`. This issue permits one
Terra implementation/review attempt and one bounded Sol correction/review; a second failure stops.
At briefing, `runner_invocations=0`, `workload_invocations=0`, and
`timed_benchmark_invocations=0`.

The rejected draft `main.rs` patch left no repository mutation, checkpoint, focused gate or
adversarial implementation verdict, so it was preimplementation exploration and Terra attempt 1
remains unused. The preimplementation root `Cargo.lock` SHA-256 is
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`. The frozen real-tap graph
cannot be constructed through the benchmark crate's old direct dependencies: implementation may
add only its required direct graph/effect/conformance dependencies and update only the
`miso-engine-builtins-bench` dependency stanza in root `Cargo.lock`. Every unrelated lock stanza
and every existing version, source and checksum remains byte-for-byte unchanged. Preflight must
freeze the resulting post-change lock SHA; any other lock drift stops the issue.

The accepted Issue-068 dependency is closed and its target/source/corpus evidence is technical
input. The current benchmark binary, validators, runner and preflight are scaffolding, not accepted
evidence: they still emit the superseded Issue-007 record shape, do not consume the ten checked
benchmark inputs, do not observe seven real graph taps, repeat warmup within both rounds, and do
not implement the frozen raw/stderr/disposition lifecycle. Terra must close only those tooling
gaps. Production DSP, corpus bytes and accepted audit/target evidence remain immutable.

## Outcome

Seal the nonbenchmark-qualified builtin candidate with the frozen schema-v2 benchmark lifecycle and
one exactly-once descriptive invocation, producing the machine-qualified candidate for listening.

## Context

This issue starts only after **Builtin native, AArch64, and Wasm runtime-selection and instruction
qualification** passes. It consumes that exact candidate, corpus, audit and target evidence. It permits one Terra
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
contains one fixed launch of the preflight-sealed release binary, refuses overwrite/retry/resume,
preserves raw output on every outcome, validates before atomic byte-identical accepted publication
and writes a checksummed disposition. The single binary process performs one untimed warmup pass
and exactly two measured rounds; it emits no warmup records and exactly 20 measured JSONL records.
Preflight and tests use counted stubs/synthetic records only and report zero workload launches.

## Deliverables

Benchmark binary, schema-v2 validators/mutations, safe runner, zero-launch preflight/sealed hashes,
and—only after authorization—raw/accepted/stderr/disposition artifacts with final machine verdict.

## Explicit non-goals

DSP/corpus/audit/target changes, timing thresholds, tuning/optimization, retries, expanded rate or
workload matrices, listening execution, deployment work, or V1/legacy inspection.

## Dependencies by exact issue title

- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Acceptance gates with objective measurements

The exact Issue-035 schema rejects every missing/extra/wrong-type/identity/shape/percentile/
metadata/audit/hash/cardinality/cross-round mutation. Hermetic runner tests prove argument,
missing-tool, workload/validator failure, interruption/partial output, overwrite and artifact
lifecycle semantics with zero real launches. Preflight seals clean candidate/binary/corpus/audit/
runner/validator hashes and reports zero launches. Only then may root authorize exactly one
external runner invocation; it emits exactly 20 validator-valid records for five kinds, rates
48/96 kHz and rounds 1/2, preserves byte-identical raw/accepted output and has no threshold or
retry. Any launch or post-launch failure consumes authorization and is strict FAIL.

The Issue-035 record ownership remains exact: `issue=35`, `issue035.*` workload IDs and
`target/issue35/` artifact paths. Issue 058 owns the qualification disposition and counters but
does not silently rename that frozen schema. The meter workload preserves
`builtin.meter.duplicate`: it uses two separately prepared otherwise-identical accepted graph
plans, each with seven unique real tap requests, for success-drain versus capacity-one full/drop
outcomes. No production duplicate-meter API change is permitted.

## Target matrix

One pinned native benchmark host recorded completely in schema metadata. Accepted Issue-068 target
evidence covers all four launch rates/targets and is not rerun here.

## Required evidence

Sealed identities; validator/mutation/preflight transcripts; exact invocation/warmup/round/record
counts; raw/accepted/disposition hashes; environment metadata; strict verdicts and attempt count.
On PASS set `machine_qualification=PASS`, `human_listening_status=pending` with Issue 033,
`workload_invocations=1`, and `timed_benchmark_invocations=1`.

## Terra attempt 1 finalization — 2026-08-22

**FAIL; THE TIMED COMMAND REMAINS UNAUTHORIZED.** Terra adversarially reviewed clean committed
candidate `f15a7aefc7379b585508673823bbdaf89c238cd2` and found that the green tooling gates do not
implement the frozen Issue-035 workload:

- the three direct render workloads synthesize phase/ramp input instead of consuming their checked
  referenced PCM, including replacing the required signed-zero identity input; the graph-meter
  source likewise synthesizes samples instead of consuming `pcm/graph-taps.f32le`; the workload
  TOML is only hashed while record JSON is assembled, not used as the workload authority;
- matrix targets use `batch + operation` parity, so the last operation of one batch and first of
  the next receive the same target instead of alternating on every operation;
- the success meter plan has queue capacity four while the otherwise-identical full plan has
  capacity one, and both use reset generation 35 although the checked workload freezes generation
  seven; the meter output hash omits the full plan's emitted snapshots and cumulative drop outcome;
- direct-render output hashes cover only the final planar block rather than every measured block,
  and the preparation hash covers only processor count plus retained bytes rather than the frozen
  address-free processor/meter/resource projection; and
- the validators accept an arbitrary stable 64-hex manifest/input hash instead of binding the
  frozen manifest and per-pair input identities. The hermetic suite does not exercise its required
  missing-tool case, and FAIL dispositions unconditionally claim one completed warmup and two
  completed measured rounds even for partial workload failure/interruption.

This is not one small bounded correction, so Terra made no implementation change. Nonexecuting
evidence: `cargo fmt --all -- --check`, locked all-target package check, five package tests,
warning-denied all-target Clippy and no-dependency rustdoc passed; both JQ programs parsed; shell
syntax passed; the synthetic validator/stub-runner suite passed; workspace, realtime, builtins,
graph and rack policies passed; all 50 manifest rows (including exactly ten benchmark inputs) were
verified read-only; and the lock transition is limited to the five permitted dependencies in the
`miso-engine-builtins-bench` stanza. Preimplementation lock SHA-256 remains
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`, frozen lock-diff SHA-256 is
`5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`, and candidate lock SHA-256 is
`da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`.

The fixture executable was not run because this finalization explicitly prohibited `cargo run`;
the actual preflight, public runner, benchmark binary, workload, timing, audit, trace and target
gates were not invoked. Final Terra attempt-1 counters are exactly `runner_invocations=0`,
`workload_invocations=0`, and `timed_benchmark_invocations=0`. One bounded Sol correction/review
remains; no benchmark authorization follows from this attempt.

## Sol attempt 2 nonexecuting verdict — 2026-08-22

**PASS TO PREFLIGHT; THE PUBLIC RUNNER AND TIMED COMMAND REMAIN UNAUTHORIZED.** Sol reviewed clean
candidate `3f4fd34f81e7e2205503887c03ad27f3aad69c8a` requirement by requirement. The correction now
uses each checked TOML and referenced PCM as workload authority (including signed zero), alternates
matrix targets by global operation, prepares identical capacity-one/reset-generation-seven real
seven-tap meter plans, hashes complete measured PCM/meter/drop and address-free preparation
projections, binds the frozen manifest and all ten per-pair input hashes, reports truthful partial
lifecycle progress, and seals a post-gate-clean candidate without executing its binary.

Identity evidence is exact: candidate source-manifest SHA-256
`34e40ddcce0b51b53aa58629894332a0ee045e4bf4ea5a5a7ca0fffbb59c4a62`; accepted Issue-068
reconstruction `0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`;
candidate lock `da662dd70c21ae844f551e5f2ed6ef97c52982fc9f8b86d19c1776e57e0a576f`;
frozen lock diff `5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e`;
manifest `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
graph PCM `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
and graph meter `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.
The checked manifest contains exactly ten benchmark inputs and `target/issue35` contained no file
or symlink before this review.

Nonexecuting gates passed: `cargo fmt --all -- --check`; shell/JQ syntax through the complete
synthetic suite; `bash scripts/test-builtins-benchmark.sh` with scratch-only runner and preflight
stubs; read-only `bash scripts/check-builtins-fixtures.sh` over all 50 files; locked benchmark
package check/tests (8/8); the exact compiler allocation-layout test; warning-denied package
Clippy/rustdoc; locked all-target/all-feature workspace check and tests; warning-denied workspace
Clippy/rustdoc; workspace, realtime, builtins, graph and rack policy checks; workspace, realtime,
builtins and rack mutation suites; static no-stale-identity/no-artifact/no-workload scans; and
`git diff --check`. Test harnesses compiled audit/benchmark targets but no audit main, benchmark
main, trace, target workload, public runner, or actual preflight was invoked.

Preflight may now be run once on this evidence commit after root confirms it is clean and has no
Issue-035 artifacts. Sol must inspect the resulting binary/seal identities and zero counters before
separately authorizing the sole public runner command. Current counters remain exactly
`runner_invocations=0`, `workload_invocations=0`, and `timed_benchmark_invocations=0`; there is no
machine-qualification, listening, capacity, threshold, or performance claim yet.

## Final Sol disposition after the sole runner invocation — 2026-08-22

**FAIL / STOPPED / RESCOPE REQUIRED; NO OVERALL PASS.** The sole authorized command
`bash scripts/run-builtins-benchmark.sh` was consumed on clean candidate
`79c1872753aa4943761f31a77aac98eaa633c31e` and terminated with status 134 before emitting a
record. No retry, resume, direct-binary run, replacement timing, tuning run, or artifact deletion is
authorized. `machine_qualification=FAIL`; human listening remains blocked.

The preserved artifacts are internally consistent:

- sealed binary: 3,191,104 bytes, SHA-256
  `242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944`;
- preflight seal: 2,211 bytes, SHA-256
  `85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d`;
- raw JSONL: 0 bytes, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- validator stderr: 0 bytes with the same empty-file SHA-256;
- accepted JSONL: absent; and
- FAIL disposition: 974 bytes, SHA-256
  `e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce`.

The disposition has the exact closed key set and binds the candidate, binary, runner, both
validators, and preflight seal. It truthfully reports `status=FAIL`,
`reason=workload_interrupted`, `workload_exit_status=134`, `runner_invocations=1`,
`workload_invocations=1`, `timed_benchmark_invocations=1`, `warmup_passes=0`, and
`measured_rounds_completed=0`. The runner's reason is its stable status-128-or-greater lifecycle
classification; the source-level cause is narrower and provable without rerunning.

The first `meter_success_full` warmup reaches `RenderRuntime::run_operation`, which arms
`audit::in_render_scope`, and then calls `RealMeterTapRuntime::render_one`. After the successful
plan renders, `drain_all` constructs a new `Vec<MeterConsumerSnapshot>` with iterator `collect`.
The benchmark's audited global allocator sees that allocation while render is armed and calls
`std::process::abort`, explaining status 134 plus zero raw and stderr bytes before the first record.
The same code also places queue draining and evidence hashing inside the audited/timed operation,
contradicting the frozen rule that input generation, evidence hashing, queue draining, metadata,
and destruction stay outside the measured interval. This is a real benchmark-harness defect, not
a runner, product DSP, corpus, tolerance, or evidence-serialization failure.

Issue 058 has exhausted Terra attempt 1, the single bounded Sol correction, and its exactly-once
authorization. Because Issues 033 and 026 require a completed machine candidate, a new stateless
successor is required. Its minimum scope is: preserve these artifacts; separate product render
from preallocated/off-scope meter drain and all evidence hashing for warmup and measurement; add a
nonexecuting test that arms the allocator around every render workload while keeping drain/hash
outside; reseal a clean candidate; and own one newly frozen no-retry preflight plus exactly-once
descriptive run. It must not change DSP, corpus, schema, rates, workload counts, tolerances,
targets, audits, listening, or the one-warmup/two-round contract. Downstream exact-title
dependencies must move to that successor before further qualification.
