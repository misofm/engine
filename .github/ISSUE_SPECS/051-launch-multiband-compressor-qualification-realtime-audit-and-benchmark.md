# 051 Launch multiband compressor qualification, realtime audit, and benchmark

## Outcome

Qualify the accepted launch two-band LR4 multiband compressor with expanded independent DSP
fixtures, adversarial matrices, production realtime proof, cross-target instruction evidence, one
controlled descriptive benchmark and an honest audition/listening handoff.

## Context

Issue **Launch two-band LR4 multiband compressor** owns the launch product: descriptor, fixed
two-band topology, scalar/W4/W8 processing, state/resources, registry, graph, scalar tails,
latency/bypass and PDC. This successor adds evidence only and cannot change those contracts.

There are exactly **two total attempts**: one Terra qualification/review and one bounded Sol
test/tool correction. A second failure or production defect stops. No benchmark is authorized at
creation; `timed_benchmark_invocations=0` until all nonbenchmark gates pass and root explicitly
authorizes the sole frozen invocation.

## Scope and deliverables

- Build one checked production-independent `f64` LR4/two-band compressor corpus and expanded
  crossover, dynamics, lookahead, link, automation, state and recovery matrices.
- Add deterministic 10,000-configuration and frozen million-sample rows plus expanded cohort,
  scalar-tail and determinism evidence.
- Run exactly 100,000 real prepared-graph renders under forbidden-operation counters without
  recording timing.
- Prove native scalar/x86 AVX2/AVX2+FMA, AArch64 NEON and Wasm scalar/base-`simd128` build and named
  instruction contracts, including zero FMA contractions.
- Build workload-free benchmark preflight; only after authorization run one invocation containing
  one untimed warmup and two measured rounds, without threshold or retry.
- Produce checksummed level-matched audition PCM and blinded listening preregistration.
- Record nonlaunch research for three through eight bands, phase-compensation trees and alternate
  quality/topology candidates without adding production modes. Product expansion requires a later
  issue and brief.

## Required public interfaces/contracts

No production interface or DSP change is allowed. Reference, corpus, instrumentation and runners
are test/tool-only and unreachable from production. Reuse accepted qualification facilities.

## Explicit non-goals

Product repair/redesign; adding launch bands, sidechain, topology, quality or FIR/multirate
framework; changed domains/tolerances; optimization; unexecuted device/browser claims; benchmark
retry; or fabricated listening.

## Dependencies by exact issue title

- Launch two-band LR4 multiband compressor

## Sol implementation brief

**REQUIRES A TRACKED SOL BRIEF AFTER ISSUE 018 PASSES.** Freeze corpus identities, seeds, long rows,
audit counters, target/object commands, benchmark workload/schema and audition handoff before
implementation. This issue authorizes no benchmark.

## Acceptance gates with objective measurements

1. Checked independent fixtures cover the frozen crossover/recombination, dynamics, lookahead/link,
   automation/state/recovery boundaries and reject oracle/corpus dependency mutations.
2. Exactly 10,000 frozen legal configurations and named million-sample rows stay finite and
   deterministic with zero recovery for valid inputs.
3. Expanded cohorts preserve bank membership, scalar tails, graph/PDC bytes and transactional
   ownership under repeated equivalent preparation.
4. Exactly 100,000 real graph renders exercise bank/scalar paths with stable addresses and zero
   armed forbidden-operation counters; disarm before observation/destruction and record no timing.
5. Native/AArch64/Wasm compile and instruction gates prove scalar/W4/W8 graphs, selection, zero FMA
   and no relaxed-SIMD dependency.
6. Focused/full locked tests, warning-denied Clippy/rustdoc, formatting, policies/mutations and
   workload-free preflight pass with `workload_launches=0`.
7. Only then, with root authorization, run the frozen command once: one warmup and two measured
   rounds, no threshold, tuning, retry or overwrite; preserve first raw output.
8. Checksummed audition PCM and blinded preregistration cover crossover phase and band compression
   without claiming completed listeners.

## Target matrix

Native scalar and runtime-gated x86 AVX2/AVX2+FMA; AArch64 NEON W4; wasm32 scalar and base
`simd128` W4. Cross-compilation is not device/runtime execution.

## Required evidence

Accepted Issue-018 candidate; oracle/corpus hashes and maxima; seed/row/transcript identities;
cohort/graph/audit counters; target/instruction outputs; preflight `workload_launches=0`;
authorization and sole benchmark result if run; audition hashes; attempt count;
`timed_benchmark_invocations`; and strict Terra/final Sol verdicts.
