# 049 Launch true-peak limiter qualification, realtime audit, and benchmark

## Outcome

Qualify the accepted fixed-4x launch true-peak safety limiter with expanded independent standard
fixtures, long adversarial matrices, production realtime proof, cross-target instruction evidence,
one controlled descriptive benchmark and an honest audition/listening handoff.

## Context

Issue **Launch fixed-4x true-peak safety limiter** owns the complete product contract: estimator,
guard, gain law, parameters, latency, state/resources, scalar/W4/W8 processing and graph/PDC
vertical. This stateless successor adds qualification only. It cannot change those contracts; a
production defect fails and returns to a new product issue.

There are exactly **two total attempts**: one Terra qualification/review attempt and one bounded
Sol test/tool correction. A second failure stops. No benchmark is authorized at creation;
`timed_benchmark_invocations=0` until every nonbenchmark gate passes and root explicitly authorizes
the single frozen invocation.

## Scope and deliverables

- Build one checked true-peak corpus from official/standard-derived and independently generated
  boundary material, with a production-independent high-rate `f64` reconstruction oracle.
- Expand phase/frequency/amplitude, ceiling/release/lookahead/link, automation/state/recovery and
  all-launch-rate matrices, including deterministic long finite sequences.
- Expand bank cohort/tail/graph determinism and transactional capacity evidence.
- Run exactly 100,000 real prepared-graph renders under forbidden-operation counters without
  recording timing.
- Prove native scalar/x86 AVX2/AVX2+FMA, AArch64 NEON and Wasm scalar/base-`simd128` compile and
  named instruction contracts, including zero gain-kernel FMA contractions.
- Build workload-free benchmark preflight/validation; only after authorization run one invocation
  containing one untimed warmup and two measured rounds, without a threshold or retry.
- Produce checksummed ceiling-matched audition PCM and an answer-key-separated blinded listening
  preregistration; completed human listening remains nonblocking.

## Required public interfaces/contracts

No production interface or DSP change is allowed. Reference, corpus, instrumentation and runners
are test/tool-only and unreachable from production. Reuse the accepted conformance, graph,
realtime-audit and benchmark conventions rather than creating parallel frameworks.

## Explicit non-goals

Product repair/redesign; a different FIR/factor/guard/gain law; a reusable oversampling framework;
loudness metering; changed domains/tolerances after failure; performance optimization; device/
browser runtime claims without execution; benchmark retry; or fabricated human listening.

## Dependencies by exact issue title

- Launch fixed-4x true-peak safety limiter

## Sol implementation brief

**REQUIRES A TRACKED SOL BRIEF AFTER ISSUE 016 PASSES.** Freeze exact official fixture identities,
independent reconstruction, deterministic seeds/rows, audit graph/counters, target/object commands,
benchmark workloads/schema and listening handoff before implementation. No timing is authorized by
this issue body.

## Acceptance gates with objective measurements

1. Checked independent fixtures cover estimator under-read, high-frequency/intersample peaks,
   limiter ceiling, release/lookahead/link/automation/state/recovery boundaries and reject every
   production-oracle dependency or corpus integrity mutation.
2. Frozen deterministic long matrices at all launch rates remain finite, meet the Issue-016
   estimator/ceiling contracts and report zero recovery for valid input; invalid probes affect only
   the injected lane.
3. Expanded counts/cohorts and repeated equivalent preparation preserve bank membership, scalar
   tails, graph/PDC bytes, PCM/state/report hashes and transactional ownership.
4. Exactly 100,000 real prepared-graph renders exercise bank and scalar paths with stable addresses
   and zero armed allocation/free, lock, feature-detection, I/O, log, syscall, panic/unwind or
   structural-mutation counters. The audit records no timing.
5. Native, AArch64 and Wasm compile/instruction gates prove the frozen scalar/W4/W8 multiply/select
   graph, correct runtime selection, zero FMA contractions and no relaxed-SIMD dependency.
6. Focused/full locked tests, warning-denied Clippy/rustdoc, formatting, policies/mutations and
   workload-free benchmark preflight pass with `workload_launches=0`.
7. Only then, and only with explicit root authorization, the frozen command runs exactly once with
   one warmup and two measured rounds, no threshold, tuning, retry or overwrite. Preserve the first
   raw output if tooling promotion fails.
8. Checksummed ceiling-matched audition PCM and a blinded preregistration cover release/lookahead/
   link behavior without claiming completed listeners or certified programme delivery.

## Target matrix

Native scalar and runtime-gated x86 AVX2/AVX2+FMA; AArch64 NEON W4; wasm32 scalar and base
`simd128` W4. Cross-compilation is not device/runtime execution evidence.

## Required evidence

Accepted Issue-016 candidate identity; official/reference/corpus hashes and maxima; frozen matrix
seeds/rows/transcript hashes; exact cohort/graph/audit counters; target/instruction outputs;
preflight `workload_launches=0`; benchmark authorization and, only afterward, raw/accepted hashes
and record count; audition/preregistration hashes; attempt count; explicit
`timed_benchmark_invocations`; and Terra/final Sol PASS/FAIL verdicts.
