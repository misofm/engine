# 068 Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Outcome

Qualify the realtime-audited sealed builtin candidate across the launch native/AArch64/Wasm build,
backend-selection and instruction matrix without executing a benchmark.

## Context

This issue starts only after **Quiescent builtin graph retirement-worker trace closure** passes and consumes
that exact candidate plus the preserved Issue-069 direct/functional evidence and sealed corpus. It permits exactly one Terra attempt and
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

- Quiescent builtin graph retirement-worker trace closure
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

Exact Issue-070 PASS candidate plus preserved Issue-069 audit/corpus identities; before/after source and `Cargo.lock` seals;
unique scratch identity; four-rate rows; 16-row selection matrix; target/build and named-instruction
transcripts; strict Terra/Sol verdicts; `workload_invocations=0` and
`timed_benchmark_invocations=0`.

## Terra attempt 1 evidence — STOP (2026-08-22)

Terra stopped this single bounded attempt at the required Wasm SIMD named-symbol uniqueness gate.
No DSP, corpus, audit, host, benchmark, preflight, workload, or timing work was performed.

Two candidate-script harness repairs preceded the final candidate run and are not target evidence:

- `61cfc069ad0744eea1029d293de92e1e7f724083`: the first clean run passed the three semantic test
  stages, then stopped before a target build because a combined `local` declaration expanded
  `name` under `set -u` (`name: unbound variable`). The script was corrected to assign local
  variables sequentially.
- `a86aaae3df5216d6a1f1c844b21ee0adad202eb4`: the second clean run again passed the semantic
  stages, then rejected scalar `%xmm` instructions through an over-broad `%[xyz]mm` pattern. The
  frozen scalar contract bans packed AVX/FMA, not scalar SSE; the harness was narrowed to
  `%[yz]mm`, matching the established instruction check.

Final candidate run: `ae57aefe41d127b5ca92625f2ab71a228ae658bf`; source-manifest seal
`83e09d3c2088a9a18db9de3ebfcae3786800157b91141cd6fcf28b32521b33e6`; `Cargo.lock` seal
`96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`; scratch basename
`miso-engine-issue068.Hc3DsO`. The three pre-run corpus identities matched the frozen values:
`MANIFEST.tsv=bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`,
`graph-taps.f32le=508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19`, and
`graph-taps.jsonl=958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

PASS before the stop:

- all 16 injected capability tuples selected the frozen backend/width precedence; the typed
  unavailable-backend preparation test passed;
- native four-rate public bank/scalar rows passed: 44,100 Hz `b1dc6cb4340e2587`, 48,000 Hz
  `880b5d4b2bc6cce7`, 88,200 Hz `be67b6b958f1df14`, and 96,000 Hz `c4d6558079359c99`;
- native scalar, AVX2, and AVX2+FMA named TPT object checks passed; all five-package release
  closure builds passed for native scalar, Android AArch64, iOS AArch64, Wasm scalar, and Wasm
  `simd128` (cross-target rows are compile/object-only, not device or browser execution);
- AArch64 NEON and Wasm-scalar object checks passed.

The exact first final-run failure was:

`issue068 target qualification failure: expected exactly one Wasm SIMD TPT symbol, found 2`

Terra did not inspect or repair that gate after the failure. Consequently the Wasm-SIMD opcode
inspection, post-run candidate/source/lock/corpus re-seal, checked-corpus validation, full exact-
closure test/Clippy/rustdoc/format/policy/static gates, and final PASS verdict remain unrun.
`workload_invocations=0`; `timed_benchmark_invocations=0`.
