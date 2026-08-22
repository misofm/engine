# 068 Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Outcome

Qualify the realtime-audited sealed builtin candidate across the launch native/AArch64/Wasm build,
backend-selection and instruction matrix without executing a benchmark.

## Context

This issue starts only after **Builtin direct and graph realtime audit closure** passes and consumes
that exact candidate, sealed corpus and audit evidence. It permits exactly one Terra attempt and
one bounded Sol correction/review; a second failure stops. Launch sample rates are exactly 44,100,
48,000, 88,200 and 96,000 Hz. Workload, benchmark and timing invocations are forbidden and remain
zero.

## Scope

Complete native scalar/AVX2/separately gated AVX2+FMA selection evidence, Android/iOS AArch64 NEON
cross-builds, Wasm scalar/base-`simd128` builds, frozen builtin-kernel instruction inspection,
four-rate nonbenchmark correctness rows and the final target/policy seal.

## Required public interfaces/contracts

Backend choice is prepared off render and retained in safe prepared tokens. Tests cover every
legal injected capability combination and prove unsupported requests fall back or reject exactly
as declared; render performs no feature detection. Native AVX2 and AVX2+FMA remain separately
gated. Wasm scalar is built with `-simd128`, Wasm SIMD with `+simd128` and no relaxed-SIMD
dependency. AArch64 uses the existing four-lane NEON path. Object inspection must name the frozen
production builtin kernel symbols and reject scalarized vector paths, unexpected fused operations
or an unapproved instruction family.

## Deliverables

Deterministic backend-selection report; four-rate native correctness rows; native/AArch64/Wasm
build records; named-symbol instruction reports; focused target tests and nonbenchmark policy seal.

## Explicit non-goals

DSP/corpus/audit changes; device or browser host adapters; AudioWorklet/device runs; a general SIMD
framework; benchmark/preflight/workload/timing; performance claims; listening; or V1 inspection.

## Dependencies by exact issue title

- Builtin direct and graph realtime audit closure
- Bootstrap Rust workspace and target matrix
- Production SIMD builtin bank graph retention and reachability qualification

## Acceptance gates with objective measurements

1. Native scalar and every available runtime-selected x86 backend reproduce the sealed correctness
   rows at 44.1/48/88.2/96 kHz; injected capability tests cover scalar, AVX2 and AVX2+FMA choice and
   prove feature detection is preparation-only.
2. Release checks pass for `aarch64-linux-android` and `aarch64-apple-ios`; named NEON kernels use
   four-lane operations and preserve their frozen fusion contract.
3. `wasm32-unknown-unknown` scalar and base-`simd128` artifacts both build; named SIMD kernels
   contain the required `f32x4` operations, the scalar artifact does not retain those symbols, and
   neither claim depends on relaxed SIMD.
4. Native AVX2/AVX2+FMA named-symbol inspection proves eight-lane width, separate runtime gating
   and each kernel's already accepted contraction count. Compile-only cross-target evidence is
   labeled as such and is not device/browser listening evidence.
5. Focused target/runtime tests, locked nonbenchmark workspace check/tests, format,
   warning-denied Clippy/rustdoc, target/workspace/realtime policies and static no-artifact/
   no-workload checks pass on one candidate.

## Target matrix

Native x86-64 scalar plus runtime-selected AVX2 and AVX2+FMA; Android and iOS AArch64 NEON cross
builds; `wasm32-unknown-unknown` scalar and base `simd128`. All four launch rates are represented in
the native correctness evidence; cross-compilation alone makes no device-runtime claim.

## Required evidence

Exact Issue-057 candidate/audit/corpus identities; four-rate rows; selection matrix; target/build
and named-instruction transcripts; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.
