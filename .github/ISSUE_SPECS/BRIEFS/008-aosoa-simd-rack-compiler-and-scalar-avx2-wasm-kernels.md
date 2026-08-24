# Sol implementation brief — issue 008 AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Decision, authority and attempt budget

**READY FOR TERRA ATTEMPT 1.** Start from clean `main` at `e0565ca`. Issues 002, 003, 006, 011,
032 and 036 are accepted inputs. This workflow permits exactly Terra attempt 1 and, if needed,
one bounded Sol correction/review. A second failed verdict stops; preserve the evidence and create
a stateless rescope. Never inspect V1/legacy.

This is a working vertical, not a SIMD evidence program. Deliver one deterministic cohort compiler,
one preallocated AoSoA render path, the accepted builtin TPT adapter, the existing homogeneous
native-effect bank seam, scalar tails and exact target dispatch. Use one small checked fixture
corpus, one representative realtime audit and one descriptive benchmark. Do not add a second
fixture framework, device/browser runtime harness, generic code generator or optimization pass.

## Accepted inputs that must not be redesigned

- `miso_engine_core::TargetCapabilities` is detected off render. AVX2 and FMA are independent;
  scalar is always available. Wasm `simd128` is a module-build fact and AArch64 NEON is a target
  fact. Detection is stored in the prepared plan and never repeated in render.
- `miso_engine_effect_contract::BankWidth` is exactly `Four` or `Eight`. Scalar processing remains
  `PreparedNativeEffect`; homogeneous banks remain `PreparedNativeEffectBank` over
  `EffectBankProcessBlock` with separate sample-major AoSoA L/R slices. Extend
  `PrepareEffectBankRequest` only with the selected backend needed by issue 008; do not change its
  request equality, state, automation, bypass, latency, tail, sanitization or report semantics.
- `PreparedEffectMetadata::program_key()` / `EffectProgramKeyV1` is the semantic slot key. It is
  typed equality/ordering only: do not serialize, hash, persist or treat it as package identity.
- Issue 006's seven `TrackStage` values, effect nodes, stable topological order, PDC, reductions,
  buffer liveness, required bindings and sealed builtin attachment remain authoritative. A bank is
  an execution grouping, never a graph rewrite or a different summation/PDC order.
- Issue 036's builtin HPF/LPF preparation, cutoff table, stored `f32 c1/a2/a3/k`, independent
  `f32 s1/s2` per lane/filter, reset/recovery and scalar operation graph are immutable inputs.
  SIMD must not reconstruct `c1`, change coefficient precision or adopt TDF-II/compensation.

## Production boundary

Add `KernelBackendV1` to core beside the existing capabilities and architecture internals, so the
effect contract can carry a selected backend without a dependency cycle. Add `miso-engine-rack` /
`miso_engine_rack` as the render-reachable safe substrate. It depends only on core and
effect-contract. It owns safe AoSoA storage/views, `KernelDispatch`, typed rack signatures,
active-slot masks and prepared effect-bank adapters. Add
`miso-engine-rack-compiler` / `miso_engine_rack_compiler` as the off-render cohort compiler. It may
depend on core, session, effect-contract, effect-compiler and rack. Graph compiler consumes its
prepared result; graph render depends only on rack, core and effect-contract, never session or the
control-plane compilers.

Modify only those packages plus the narrow integration seams in core architecture internals,
effect preparation, graph/graph-compiler, builtins/builtins-compiler, conformance mocks, workspace
manifests/policies/docs, one rack fixture tool, one audit tool and one benchmark tool/runner. Add no
external dependency. Do not alter session TOML/wire, effect package/CID/state-envelope work,
multicore scheduling, streaming, PDC equations, meter math or any production effect algorithm.

The workspace unsafe exception remains narrow: architecture intrinsics live only under
`crates/miso-engine-core/src/arch/`, with local lint allowances, adjacent `SAFETY` invariants and
safe slice/value entrypoints. Rack/effect/graph public APIs expose no native register, raw pointer,
unchecked target-feature function or unsafe constructor. Update the realtime policy allowlist and
mutation test for those exact core files; no package-wide unsafe allowance is permitted.

## Frozen dispatch contract

Export from core a stable, non-exhaustive `KernelBackendV1` with these semantic variants and lane
counts:

| Backend | Lanes | Selection |
| --- | ---: | --- |
| `Scalar` | 1 | unconditional fallback |
| `WasmSimd128` | 4 | wasm32 artifact compiled with `simd128` |
| `Aarch64Neon` | 4 | AArch64 target reports NEON |
| `X86Avx2` | 8 | runtime AVX2 true, FMA false |
| `X86Avx2Fma` | 8 | runtime AVX2 and FMA both true |

`KernelDispatch::select(TargetCapabilities)` is pure/bounded and is called only while preparing a
plan. On x86, FMA without AVX2 selects scalar and AVX2 without FMA selects `X86Avx2`. No Cargo
feature may be named `simd128`, `neon`, `avx2` or `fma`; no `.cargo` or release-wide target feature
is added. Internal x86 functions use per-function `#[target_feature]`; the non-FMA function may not
contain FMA instructions. Base Wasm uses `f32x4` multiply then add/subtract and no relaxed SIMD.

The factory bank request records this backend and validates that its width matches four/eight.
Factories may return `Ok(None)` for a legal but unsupported backend/program; that cohort executes
the existing scalar processors at the same graph positions. Third-party effects and every dynamic
rack entry are scalar/dynamic only and never enter a bank request.

## AoSoA and dual-mono layout

For width `W`, each preallocated lane slice has exactly `frames * W` `f32` values and index
`sample * W + lane`. Left, right and present sidechain use separate, nonoverlapping slices. Gather
and scatter use stable track-ID lane order. Arithmetic checks reject zero frames, frames above
quantum, width mismatch and `frames * W` overflow off render. Unaligned internal loads/stores are
allowed; no public alignment promise or padded logical lane exists.

Graph bank execution gathers already-reduced member inputs into its owned AoSoA scratch, invokes
one prepared bank, then scatters to the member output buffers before any dependent node runs.
Compilation admits only members in a common deterministic dependency wave whose main and optional
sidechain inputs are already available. Any incompatible dependency/routing shape remains scalar.
Observers run once per original node in stable node/handle order after scatter. Graph schedule,
route-reduction order, sample time, PDC delays and output bytes remain otherwise unchanged.

All state and parameters are per track and per L/R lane. A linked detector may share only the
Issue-011-declared detector state; audio/filter/delay/smoother state never aliases between L/R or
tracks. A left-only perturbation must leave every right lane output and lane-state payload equal to
its control except where the declared detector-link contract permits common detector changes.

## Deterministic rack signatures and cohorts

`RackProgramSignatureV1` contains rack (`Simd1` or `Simd2`), explicit rate/quantum, ordered slot
keys, and a routing class. Each real slot key is `EffectProgramKeyV1` plus its zero-based occurrence
among equal keys. Routing class records sidechain absent/unconnected/connected shape and the graph
dependency-wave compatibility needed for safe gather; it does not contain per-track parameter
values, state, track IDs, serialized bytes or a digest.

Compile tracks in stable ID order. Exact signatures cohort directly. To support absent slots
without inventing session nodes, choose the deterministic longest compatible signature (ties by
ordered slot keys) and admit a track only when its real sequence is an ordered subsequence with the
same routing class. Missing positions become `Identity` in that track's active-slot mask. An
identity slot has zero latency/tail/state/automation, returns its input unchanged, and creates no
graph effect node. For a homogeneous factory call, a missing lane uses a validated default request
with the real slot key; its computed output/state is discarded and the saved dry lane is restored.
Its synthetic retained state/scratch is included in the plan resource estimate. If the factory
cannot bind the complete width, use scalar real slots plus identity no-ops.

Partition each cohort into full selected-width banks followed by stable scalar tails; never pad a
track count to claim a full bank. Thus counts 1–3, 5–7 and 9+ are valid without a compiled maximum.
An all-identity rack boundary aliases the graph buffer exactly as Issue 006 already permits.
Compiler diagnostics are deterministic and transactional; any arithmetic/cap failure returns all
owned prepared inputs and no publishable partial plan.

## Frozen builtin TPT bank adapter

Bank only the `post-input-builtins` polarity/trim/HPF/LPF section in this issue. Fader/mute and the
smoothed 2x2 matrix retain their accepted scalar graph processors. Transpose each track/lane's
stored `c1/a2/a3/k`, enable flags and `s1/s2` into width-four/eight vectors; L/R and HPF/LPF remain
four distinct state sets. Disabled filters are exact identity operations and do not update state.

The base scalar, Wasm SIMD, NEON and AVX2-without-FMA paths preserve these separately rounded steps:

```text
v3=x-s2; p1=a2*v3; p2=c1*s1; d1=p1-p2; v1=s1+d1;
p3=a2*s1; p4=a3*v3; d2=p3+p4; v2=s2+d2;
n1=s1+(d1+d1); n2=s2+(d2+d2);
low=v2; high=(x-k*v1)-v2
```

No reassociation or contraction is allowed. `X86Avx2Fma` is a distinct backend and may fuse only
`d1 = a2*v3 - p2`, `d2 = a2*s1 + p4`, and `th = x - k*v1`, with `p2`/`p4` already rounded and all
other steps unchanged. Sanitization/recovery is lane-local and yields the same counters and
positive-zero reset behavior as scalar.

For identical coefficient/input bits with finite-normal data and no sanitation, every base
non-FMA path is bit-identical to its same-target scalar graph. Cross-target observations and the
explicit FMA path use the existing conformance tolerance
`abs(error) <= 1e-6 + 2e-5 * abs(scalar)` samplewise and must also pass Issue 007/036's retained
analytic/cutoff, finite-state, reset and response limits. This tolerance is frozen before code; do
not tune it to an observed backend.

## One representative fixture and generated adversarial cases

Check in only `fixtures/rack/v1/{MANIFEST.tsv,cases.toml,input.f32le,scalar-expected.f32le}`.
`cases.toml` defines one 48-kHz/128-frame asymmetric dual-mono vertical containing 12 tracks, two
compatible SIMD-rack signatures, a missing middle identity slot, distinct per-track parameters,
enabled/disabled HPF/LPF, scalar tails and one scalar-only incompatible/sidechain rack. The manifest
is sorted and freezes exact length/lowercase SHA-256. `--check` is read-only and rejects changed,
missing, unlisted and coverage-inconsistent files. Expected PCM is produced by the accepted scalar
path into scratch and compared with checked bytes; SIMD never generates its own expected result.

Tests additionally generate, without checked artifacts, counts 1, 2, 3, 4, 5, 7, 8, 9 and 17 and
exactly 100 layouts from seed `0x000000008a050a08`. They cover exact/incompatible signatures,
identity positions, scalar tails, asymmetric L/R state, routing fallback, caps/overflow and stable
lane/order/signature reports. Repeat compilation/render is bit-identical for the same backend.

## Exact nonbenchmark gates

Before any timed workload, all of these pass on one clean candidate:

1. `cargo fmt --all -- --check`.
2. Focused locked tests for core, rack, rack-compiler, effect-contract/compiler, builtins/compiler,
   graph/compiler and the representative graph render.
3. Rack fixture `--check` plus changed/missing/unlisted/coverage-hole mutations.
4. The exact count set and 100-layout seeded differential suite; base native scalar/AVX2 and
   available FMA comparisons, state/counter isolation, signature determinism and scalar fallback.
5. A release audit of exactly 100,000 128-frame prepared-graph renders of the representative mixed
   bank/tail plan: zero allocation/free, lock, log, file/network I/O, syscall, feature detection,
   panic/unwind or structural mutation while armed; all drops happen after disarming.
6. `cargo check --locked --workspace --all-targets --all-features`,
   `cargo test --locked --workspace --all-targets`, warning-denied workspace Clippy and rustdoc.
7. Workspace, realtime, effect-runtime, graph and new rack policy scripts plus their narrow unsafe,
   global-feature, compiled-track-limit and render-allocation mutations.
8. Native baseline `-avx2,-fma`; separate x86 specialized object probes for AVX2/no-FMA and
   AVX2+FMA; Android ARM64 and iOS ARM64 release checks; wasm32 baseline `-simd128` and separate
   `+simd128` release builds.
9. Native object inspection proves the baseline/scalar symbols contain no AVX/FMA, the AVX2 symbol
   contains packed eight-lane arithmetic and no fused instruction, and the FMA symbol contains the
   three allowed contraction sites. `wasm-objdump` proves scalar output has no SIMD opcode and the
   SIMD artifact contains `f32x4` multiply/add/subtract with no relaxed-SIMD opcode. Cross-compiles
   are compile/instruction claims only, not Android/iOS/browser runtime claims.
10. `scripts/preflight-rack-benchmark.sh` validates no arguments, schema/record mutations, output
    persistence, shell exit propagation and overwrite refusal with `workload_launches=0`.

Record command, exit status, candidate commit/source hashes and concise results. A skipped native
FMA execution on hardware lacking FMA is allowed only when selection tests and object inspection
pass and the report says `runtime_unavailable`; it is not relabeled as executed.

## Descriptive benchmark — exactly once after preflight

After every nonbenchmark gate passes and root Sol authorizes it, invoke exactly:

```text
bash scripts/run-rack-benchmark.sh
```

The runner refuses arguments and overwrite, performs one untimed warmup, then exactly two measured
rounds with no tuning/retry loop. Freeze three 48-kHz/128-frame workloads: eight separate scalar
TPT tracks; one homogeneous host-selected eight-track bank; and the 12-track mixed graph fixture
with bank(s), identities and scalar tail/fallback. Each round reports nearest-rank
min/p50/p95/p99/p99.9/max ns/frame from 1,000 observations, backend/width, exact fixture and build
identity, allocation counts, CPU/OS/governor/Rust/LLVM/target features/optimization/LTO/codegen
units and explicit missing metadata. The accepted JSONL has six records and zero errors.

There is no timing or speedup acceptance threshold. Preserve the first raw output even if runner
promotion fails; do not rerun. A runner/promotion defect becomes a separate tooling issue. Record
the scalar-versus-bank ratios as rough numbers only and move optimization ideas to the weekly pass.

## Attempt-1 evidence and Sol stop conditions

Append to Issue 008: candidate commit; exact public API/dispatch table; cohort/layout/resource
report; fixture manifest hash; seeded case count/hash; scalar/base/FMA differential maxima;
instruction findings; audit counts; target/build claims; benchmark preflight launch count; and,
only after authorization, raw/accepted benchmark hash and invocation count.

FAIL immediately for changed Issue-036 coefficient/cutoff/recurrence semantics, shared L/R or
cross-track state, graph/PDC/reduction reorder, feature detection in render, global target features,
third-party/dynamic bank execution, missing scalar fallback, hidden compiled track ceiling,
unaccounted synthetic identity state, benchmark before all gates, a retry, or a relaxed tolerance.
Device/browser runtime qualification, generalized sparse-program optimization, more fixture
matrices and performance tuning are follow-ups, not reasons to expand this attempt.

## 2026-08-24 amendment (#84 phase A)

Superseded by #83 D4/D10 via #84 phase A: the per-sample kernel tokens
(`Prepared*KernelV1`), `KernelBackendV1`, `TargetCapabilities`,
`miso_engine_core::target_capabilities()` and `miso_engine_rack::KernelDispatch` were
deleted along with `crates/miso-engine-core/src/arch`. Kernels live in
`crates/miso-engine-lane`; the backend is the compile-time constant
`miso_engine_lane::Backend::current()`, and
`miso_engine_effect_contract::BankWidth::for_backend` is the one backend-to-width law.
The historical text above is kept as the decision record of its time and is not rewritten.
