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
cross-builds, Wasm scalar/base-`simd128` builds, frozen TPT builtin-kernel instruction inspection,
four-rate nonbenchmark correctness rows and the final target/policy seal. The build closure is
exactly `miso-engine-core`, `miso-engine-builtins`, `miso-engine-builtins-compiler`,
`miso-engine-graph` and `miso-engine-graph-compiler` plus their transitive dependencies.

## Required public interfaces/contracts

Backend choice is prepared off render and retained in safe prepared tokens. Tests cover every
one of the 16 injected boolean capability tuples and prove unsupported requests fall back or reject
exactly as declared; render performs no feature detection. Native AVX2 and AVX2+FMA remain separately
gated. Wasm scalar is built with `-simd128`, Wasm SIMD with `+simd128` and no relaxed-SIMD
dependency. AArch64 uses the existing four-lane NEON path. Object inspection must name the frozen
production TPT builtin kernel symbols and reject scalarized vector paths, unexpected fused
operations or an unapproved instruction family. Every target/object build uses a fresh empty
`mktemp -d` root as `CARGO_TARGET_DIR`; each selector requires exactly one current-package object
and one named symbol before inspection, so a stale artifact cannot satisfy a gate.

Before and after the matrix, seal the clean candidate commit, a deterministic SHA-256 manifest of
the exact source/build inputs, `Cargo.lock`, and the accepted corpus/graph payloads. The corpus
manifest hash is `bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`;
graph PCM is `508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`;
graph meters are `958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

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
   rows at 44.1/48/88.2/96 kHz; the exact 16-row injected capability table proves the frozen
   `X86Avx2Fma > X86Avx2 > Aarch64Neon > WasmSimd128 > Scalar` precedence, width and
   preparation-only feature detection.
2. Release checks pass for `aarch64-linux-android` and `aarch64-apple-ios`; named NEON kernels use
   four-lane operations and preserve their frozen fusion contract.
3. `wasm32-unknown-unknown` scalar and base-`simd128` artifacts both build; named SIMD kernels
   contain the required `f32x4` operations, the scalar artifact does not retain those symbols, and
   neither claim depends on relaxed SIMD.
4. Named TPT inspection proves a clean scalar symbol; eight-lane AVX2 `vmulps/vaddps/vsubps`
   without fusion; exactly three AVX2+FMA sites (`vfmsub`, `vfmadd`, `vfnmadd`); four-lane NEON
   `fmul/fadd/fsub` without fusion; Wasm scalar without SIMD/relaxed opcodes; and Wasm SIMD
   `f32x4.mul/add/sub` without relaxed SIMD. Compile/object-only cross-target evidence is labeled as
   such and is not device/browser execution or listening evidence.
5. Focused target/runtime and exact package-closure check/tests, format, warning-denied
   Clippy/rustdoc, applicable workspace/realtime/rack/builtin policies and static
   no-artifact/no-workload checks pass on one unchanged sealed candidate.

## Target matrix

Native x86-64 scalar plus runtime-selected AVX2 and AVX2+FMA; Android and iOS AArch64 NEON cross
builds; `wasm32-unknown-unknown` scalar and base `simd128`. All four launch rates are represented in
the native correctness evidence; cross-compilation alone makes no device-runtime claim.

## Required evidence

Exact Issue-057 candidate/audit/corpus identities; before/after source and `Cargo.lock` seals;
unique scratch identity; four-rate rows; 16-row selection matrix; target/build and named-instruction
transcripts; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.
