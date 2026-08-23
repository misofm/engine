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

## Sol attempt 2 final evidence — PASS (2026-08-22)

Final clean candidate: `e649b6211e4c1ce7b26f6c4b3978ae59f4344564`; accepted Issue-070
dependency: `f952f20`; stopped Issue-069 technical input: `5ce93c0`. The first Terra failure was a
selector defect, not duplicate production code. Source contains exactly one `#[inline(never)]`
`process_tpt_wasm_simd128_inner` definition and one safe-wrapper call. `wasm-objdump -d` names the
inner once in its address-prefixed `func[index] <symbol>:` definition header and again at that call
reference. The prior raw-substring count therefore returned two, and its unanchored AWK stop did
not delimit an address-prefixed next function.

The bounded Sol correction changed only `scripts/check-builtins-target-instructions.sh`: it counts
exact address-plus-`func[index]` definition headers containing the frozen inner name, requires
exactly one, begins extraction only at that definition header and stops at the next function
header. Shell syntax, a synthetic definition-plus-call transcript and static one-definition/
one-wrapper checks passed before the correction was committed. No production source, DSP,
fixture, target feature, object, tolerance or API changed.

On that clean candidate, Sol invoked the complete candidate-bound script exactly once and without
retry:

```sh
scripts/check-builtins-target-instructions.sh
```

The before/after candidate remained `e649b6211e4c1ce7b26f6c4b3978ae59f4344564`; before/after
source-manifest SHA-256 was
`0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19`; before/after Cargo.lock
SHA-256 was `96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424`;
unique scratch basename was `miso-engine-issue068.VHP1R6`. Tools were Rust
`rustc 1.97.1 (8bab26f4f 2026-07-14)`, Cargo `1.97.1 (c980f4866 2026-06-30)`, GNU objdump
2.42 and wasm-objdump 1.0.34. Frozen corpus identities remained manifest
`bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff`, graph PCM
`508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19` and graph meters
`958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f`.

The executed 16-row selection table was exact (`w/a/x/f` are injected Wasm-SIMD, AArch64-NEON,
x86-AVX2 and x86-FMA booleans):

| w/a/x/f | Backend | Width |
| --- | --- | --- |
| 0/0/0/0 | Scalar | none |
| 0/0/0/1 | Scalar | none |
| 0/0/1/0 | X86Avx2 | 8 |
| 0/0/1/1 | X86Avx2Fma | 8 |
| 0/1/0/0 | Aarch64Neon | 4 |
| 0/1/0/1 | Aarch64Neon | 4 |
| 0/1/1/0 | X86Avx2 | 8 |
| 0/1/1/1 | X86Avx2Fma | 8 |
| 1/0/0/0 | WasmSimd128 | 4 |
| 1/0/0/1 | WasmSimd128 | 4 |
| 1/0/1/0 | X86Avx2 | 8 |
| 1/0/1/1 | X86Avx2Fma | 8 |
| 1/1/0/0 | Aarch64Neon | 4 |
| 1/1/0/1 | Aarch64Neon | 4 |
| 1/1/1/0 | X86Avx2 | 8 |
| 1/1/1/1 | X86Avx2Fma | 8 |

The native host prepared and executed mandatory AVX2 non-FMA and AVX2+FMA paths. Four-rate
scalar/bank rows passed: 44,100 Hz `b1dc6cb4340e2587`, 48,000 Hz `880b5d4b2bc6cce7`,
88,200 Hz `be67b6b958f1df14`, and 96,000 Hz `c4d6558079359c99`. Named object/symbol SHA-256
rows were:

| Leg | Object | Exact symbol body |
| --- | --- | --- |
| native scalar | `8a7e572d2e4a3916d43780c457eecbe2fd6aa508160d1ff736d8a303627eea35` | `cdaa8c53a4fcf9691e3b3a0800fef37645a3e1de833e45d59af0b4fdfbce1c3e` |
| native AVX2 | `2fe1d9ce9e82f57d9ba865da4c131ee6a57d9a93ac63e433116c10a7df85b724` | `54f2866f18bdd66de2fabb15e6d373f65b2627735bffd1e5952789f6063998e5` |
| native AVX2+FMA | `192b768ddfdcf6760a31cb7946a7b3f05cc4d5c1037b9af73f939e19a80b6a08` | `4a2842e9b30760f71ebb9d60266b0b412e5ec11ec754aabd7977632e857f7d5d` |
| AArch64 NEON | `187539957f7c2dacf3e160c9ce76fdaf5c7b9444fddcf07a0f6baf3975d7e878` | `4991b3367690f5606a0cb7ab67202bc7d82e184bfa087bf830db55bd44b3e75f` |
| Wasm scalar | `4cf54a3aa66bb86bb8b0bb4cabfa3aa63b76ca4ea646eaceec628eada05d7a7b` | `989140ce295edbfeae6d24e4f79e54757d6d5836f62452bd49f85492255684cb` |
| Wasm simd128 | `c27a56e0ecb1c6afb4bc9d9f176dceefb1942dffdf3b729b94d28ec77a79ea7e` | `e5ad164b7a3bb8ce0743afafc65b19d8406964243a0274a8f8ece7c8f104834e` |

The scalar body had no packed AVX/FMA; AVX2 used eight-lane `vmulps/vaddps/vsubps` without fusion;
AVX2+FMA had exactly the frozen `vfmsub/vfmadd/vfnmadd` sites; NEON used four-lane
`fmul/fadd/fsub` without fusion; Wasm scalar had no SIMD/relaxed opcode; the unique Wasm SIMD TPT
body had `f32x4.mul/add/sub` and no relaxed SIMD. Release closure builds passed for native scalar,
Android AArch64, iOS AArch64, Wasm scalar and Wasm simd128. Cross-target evidence remains
compile/object-only and makes no device/browser runtime claim.

The final nonbenchmark seal passed: read-only checked corpus (50 files); format; locked offline
all-target/all-feature check and tests for the exact five-package closure (107 tests);
warning-denied all-target/all-feature Clippy and rustdoc for that closure; workspace, realtime,
rack and builtin policy checks plus every corresponding mutation suite; shell syntax; candidate,
lockfile and corpus reseal; diff/static no-artifact and no-workload scans. The worktree remained
clean through qualification.

Sol verdict: **PASS**. Issue 068 qualifies the builtin candidate for the frozen native/AArch64/Wasm
selection, build and named-instruction contract and unblocks its exact downstream consumers.
`sol_complete_candidate_script_invocations=1`; `cumulative_candidate_script_invocations=4`;
`workload_invocations=0`;
`timed_benchmark_invocations=0`; `benchmark_invocations=0`.

## Amendment (audit #92, 2026-08-23) — the `native-scalar` closure leg

`scripts/check-builtins-target-instructions.sh` compiled the whole five-crate closure with
`-C target-feature=-avx2,-fma` as its `native-scalar` leg. Master plan #83 **D4** removed the scalar
`x86` build: `miso-engine-lane` `compile_error!`s on `x86` without `avx2`+`fma`, every host attests
the CPU once at boot, and there is no silent fallback. From wave 2 the effect crates depend on
`miso-engine-lane`, and `miso-engine-graph-compiler` reaches them through
`miso-engine-effect-compiler`, so that leg stopped compiling the moment the first effect crate was
re-landed — the configuration it probes is one the workspace has deliberately abolished.

The leg is kept, scoped to the part of the closure D4 does not touch: `miso-engine-core`,
`miso-engine-builtins`, `miso-engine-builtins-compiler` and `miso-engine-graph`, which still carry
the portable scalar TPT paths this issue asked about and still compile with `x86` SIMD off. The four
cross-target legs (`aarch64-linux-android`, `aarch64-apple-ios`, `wasm32` +/-`simd128`) are unchanged
and still cover the whole closure including `graph-compiler`, as are every object and symbol hash.
Nothing else in the script moved.

This collision is not specific to #92: it will meet #85, #87, #88, #89, #90, #91, #93 and #94
identically. Retiring or re-scoping the leg outright — with the rest of the issue-068 seal — belongs
to the **#104** evidence triage recorded on #125.
