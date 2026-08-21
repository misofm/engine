# 046 Launch compressor qualification, realtime audit, and benchmark

## Outcome

Qualify the accepted launch feed-forward peak compressor with independent DSP fixtures, expanded
adversarial matrices, production graph/realtime proof, cross-target instruction evidence, one
strictly controlled descriptive benchmark, and an honest listening handoff.

## Context

Issue **Launch feed-forward peak compressor** has accepted the launch product contract: descriptor,
scalar and homogeneous W4/W8 processing, state/resources, registry, graph banking, scalar tails,
connected-sidechain fallback and PDC. This successor adds evidence only. It cannot change the
effect's parameters, equations, state, latency, tail, automation, recovery, bank policy, graph or
public interfaces.

The render path remains allocation/free-, lock-, feature-detection-, I/O-, logging-, syscall-,
panic/unwind- and structural-mutation-free. Launch rates are exactly 44,100, 48,000, 88,200 and
96,000 Hz. This issue permits exactly **two total attempts**: one Terra qualification attempt and
one bounded Sol test/tool correction. A production defect or second failure stops and requires a
new stateless product issue; gates may not be weakened.

No benchmark has been authorized. `timed_benchmark_invocations=0` is invariant until every
nonbenchmark gate passes and root explicitly authorizes the sole frozen invocation.

## Scope

- Build one independent `f64` compressor oracle and one checked `fixtures/compressor/v1` corpus.
- Cover expanded static-curve, envelope, lookahead, bypass/mix, link, sidechain, automation,
  reset/restore, sanitation/recovery and isolation boundaries.
- Run exactly 10,000 seeded legal configurations and twelve frozen million-sample rows.
- Expand bank/cohort/tail membership and graph determinism around the accepted ten-track vertical.
- Run one exact 100,000-render production-graph functional realtime audit without timing.
- Prove native scalar/x86 AVX2/AVX2+FMA, AArch64 NEON and Wasm scalar/base-`simd128` operation and
  instruction contracts.
- Build and test a workload-free benchmark preflight, then preserve the sole future authorized
  one-warmup/two-measured-round invocation.
- Produce checksummed audition PCM and an answer-key-separated blinded listening preregistration.

## Required public interfaces/contracts

No production interface change is allowed. Qualification uses the accepted `CompressorFactory`,
prepared scalar/bank products, registry and real prepared graph. The independent reference and any
instrumentation are test/tool-only and unreachable from production builds. Instrumentation may
observe fixed addresses and forbidden-operation counters but cannot add a render synchronization,
allocation, log, syscall or feature-detection surface.

The exact algorithms, domains, tolerances, latency/lookahead, state, resource accounting,
scalar/W4/W8 graph and zero-FMA rule remain those frozen by Issue 013 and its tracked brief.

## Deliverables

- sorted checked corpus, independent reference and boundary-mutation checks;
- deterministic matrix transcripts, maxima and hashes;
- expanded cohort/graph report and 100,000-render audit evidence;
- target compile and named instruction/object evidence;
- zero-launch benchmark preflight/validator and, only after authorization, the sole raw result;
- audition PCM and blinded listening handoff; and
- candid Terra and final Sol PASS/FAIL evidence.

## Explicit non-goals

Production compressor/core/effect/rack/graph changes; a new DSP topology or parameter; tolerance,
domain, seed, count or fixture relaxation; connected-sidechain banking; performance optimization;
device/browser runtime claims without execution; a second fixture/audit/benchmark framework;
benchmark retries; completed or fabricated human listening; or dependencies from other effect
implementation issues.

## Dependencies by exact issue title

- Launch feed-forward peak compressor

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after Issue 013 and this local issue are synchronized upstream.** The
authoritative tracked brief is
`.github/ISSUE_SPECS/BRIEFS/046-launch-compressor-qualification-realtime-audit-and-benchmark.md`.
It freezes the corpus, matrices, audit, targets, benchmark protocol, two-attempt maximum and stop
conditions. This docs checkpoint performs no GitHub mutation and authorizes no benchmark.

## Hazards/decisions

A production-derived oracle is not independent. A short signal hidden behind the fixed latency is
not compressor evidence. Host omission is not a production-graph audit. Cross-compilation is not
device execution. FMA availability does not authorize contractions. Benchmark preflight must prove
failure behavior without launching a workload, and a runner/promotion fault cannot authorize a
retry. Listening materials cannot be reported as completed human evidence.

## Acceptance gates with objective measurements

1. The checked corpus and independent `f64` reference pass every frozen static/envelope/latency/
   lookahead/link/sidechain/automation/state/recovery boundary and reject all corpus/oracle
   independence mutations.
2. Exactly 10,000 legal configurations from seed `0x000000000013c0de` and twelve frozen
   million-sample rows complete with exact transcript hashes, finite state/output and zero recovery
   for valid inputs.
3. Counts `1,2,3,4,5,7,8,9,17` and the ten-track production fixture prove exact W4/W8 cohorts,
   scalar tails, connected fallback, stable membership, no padding, deterministic graph/PCM/state/
   report bytes, exact PDC and transactional cap failure.
4. Exactly 100,000 48-kHz/128-frame real prepared-graph renders exercise the bank and scalar
   fallback with stable addresses and zero forbidden-operation counters while armed. Disarm before
   observation or destruction; report no timing.
5. Native scalar, x86 AVX2 and separately gated AVX2+FMA, AArch64 NEON W4, Wasm scalar and base
   `simd128` W4 builds pass. Named inspection proves scalar and packed graphs, bit selection, zero
   compressor FMA contractions and no relaxed-SIMD dependency.
6. Focused/full locked tests, warning-denied Clippy/rustdoc, formatting and relevant policies pass.
   Benchmark preflight proves arguments, schema, persistence, shell failure and overwrite refusal
   with `workload_launches=0`.
7. Only after Gates 1–6 pass and root explicitly authorizes it, run the frozen benchmark command
   exactly once: one untimed warmup and two measured rounds, no threshold, retry, tuning or
   overwrite. Preserve raw output even if promotion fails.
8. Checksummed level-matched audition PCM and an answer-key-separated blinded preregistration cover
   the frozen audible dimensions. Completed listening remains a nonblocking human follow-up.

## Target matrix

Linux/native scalar; runtime-gated x86 AVX2 and AVX2+FMA; AArch64 NEON W4 for Android/iOS compile
evidence; wasm32 scalar and base `simd128` W4. Runtime claims require actual execution; otherwise
record compile/instruction evidence only.

## Required evidence

Issue-013 candidate identity; oracle/corpus/manifest hashes and maxima; fixed seed/count/transcript
hashes; cohort/graph/PCM/state/report identities; exact realtime counters; target/toolchain and
instruction reports; preflight `workload_launches=0`; benchmark authorization and, only afterward,
raw/accepted hashes and six-record count; audition/preregistration hashes; attempt count; explicit
`timed_benchmark_invocations`; and Terra/final Sol PASS/FAIL verdicts.
