# 057 Builtin direct and graph realtime audit and target qualification

## Outcome

Qualify the sealed builtin candidate through direct and production-graph realtime audits plus the
required native/mobile/Wasm nonbenchmark target matrix.

## Context

This issue starts only after **Complete independent builtin corpus and corruption proof** passes.
It consumes that exact candidate/corpus without regenerating or changing expected values. It
permits exactly one Terra attempt and one bounded Sol correction; a second failure stops.
Launch-rate scope is exactly 44,100, 48,000, 88,200 and 96,000 Hz. Benchmark binaries, workloads
and timing are forbidden; invocation counts start and remain zero.

## Scope

Complete the Issue-035 frozen direct-chain and production-graph audits, graph tap/PDC/swap/
retirement evidence, detector probes, launch-rate correctness rows, target builds/instruction
checks and clean nonbenchmark workspace/policy seal. Use the sealed Issue-056 corpus as expected
output.

## Required public interfaces/contracts

Audit tools emit deterministic checksummed records and accept only their frozen configurations.
The direct audit uses the public builtin API. The graph audit renders through the compiled/bound
`PreparedRenderPlan` ownership path and never substitutes direct `BuiltinChain` calls. Armed render
performs zero forbidden operations; retirement/destruction occurs after disarm.

## Deliverables

Direct and graph audit tools/records, syscall/probe evidence, four-rate correctness report,
native/AArch64/Wasm target and instruction evidence, and final nonbenchmark workspace/policy seal.

## Explicit non-goals

Corpus redesign, DSP repair, benchmark schema/runner/preflight/timing, performance claims,
listening, deployment adapters, or V1/legacy inspection.

## Dependencies by exact issue title

- Complete independent builtin corpus and corruption proof
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Production SIMD builtin bank graph retention and reachability qualification

## Acceptance gates with objective measurements

The frozen launch-rate rows pass at exactly 44.1/48/88.2/96 kHz. At 48 kHz/q128, direct and graph
audits each execute exactly 1,000,000 calls/renders with exact corpus PCM/state/meter/PDC/lifecycle
results, stable storage, zero allocation/free/lock/log/I/O/network/syscall/feature-detection/panic
counts and zero total. The graph record proves the frozen A/B/C swap, retirement-full deferral and
off-render destruction. Marker-delimited trace and all nine deliberate detector probes pass.
Native scalar/AVX2, AArch64 NEON and Wasm scalar/simd128 build/selection/instruction gates pass,
followed by locked warning-denied workspace/policy gates.

## Target matrix

Pinned native debug/scalar release and runtime-gated AVX2; compile evidence for Android/iOS AArch64;
wasm32 scalar and base `simd128`. Compile evidence is not device/runtime listening evidence.

## Required evidence

Exact candidate/corpus hashes, four-rate rows, both million-count audit records, trace/probe hashes,
target/instruction results, workspace/policy commands, strict verdicts and attempt count;
`workload_invocations=0`; `timed_benchmark_invocations=0`.
