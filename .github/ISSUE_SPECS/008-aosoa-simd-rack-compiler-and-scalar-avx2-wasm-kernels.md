# 008 AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Outcome

Build the bankable effect-rack execution substrate without sacrificing dual-mono semantics or portability.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; extended-rate SIMD qualification is deferred. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its
change follows the tracked Sol brief → Terra attempt 1 with evidence → Sol adversarial review.
There are at most **two total attempts**: Terra attempt 1 and one bounded Sol correction/review.
A second failure stops and requires a new stateless rescope rather than weakened gates.

## Scope

Implement planar `f32` AoSoA across tracks so each vector at a sample contains the same L or R lane from four Wasm/NEON tracks or eight AVX2 tracks. Compile cohorts sharing effect types/order, quality and compatible routing; keep per-track parameters/state; use identity kernels for absent slots; and implement scalar/tail execution plus separate AVX2/FMA dispatch. Base Wasm SIMD uses multiply-plus-add; optional relaxed SIMD cannot change correctness requirements.

## Required public interfaces/contracts

The accepted effect contract's `BankWidth` remains exactly `Four` or `Eight`; scalar is an
execution fallback, not a one-lane bank value. `RackProgramSignatureV1` declares slot types/order,
layout, quality and routing compatibility; `RackKernel` holds distinct vectors/state for both
dual-mono lanes. `KernelDispatch::select(TargetCapabilities)` selects scalar, AArch64 NEON,
AVX2-without-FMA and AVX2+FMA without a global target-feature assumption, while Wasm scalar/SIMD
artifacts are host-selected. No public contract exposes unsafe SIMD registers.

## Deliverables

AoSoA buffers, rack compiler, scalar/SIMD kernel traits, runtime dispatch, feature tests, kernel benchmarks, and debug layout validator.

## Explicit non-goals

Placing arbitrary third-party Wasm in a SIMD bank, fixed global eight-track assumption, global AVX2 requirement, or cross-channel state sharing.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Native effect runtime contract and conformance
- Representable TPT cutoff domain and builtin contract acceptance
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/008-aosoa-simd-rack-compiler-and-scalar-avx2-wasm-kernels.md`.
It freezes the current effect/builtin/graph seams, the two-attempt budget, one representative
fixture/audit vertical, exact target and instruction gates, and one descriptive benchmark
invocation with one warmup and two measured rounds only after workload-free preflight.

## Hazards/decisions

Wasm SIMD vectors are v128/4 f32 lanes; core spec: https://webassembly.github.io/spec/core/. AVX2/FMA need runtime detection: https://doc.rust-lang.org/std/arch/macro.is_x86_feature_detected.html.

Issue 007's recurrence-rescoped HPF/LPF contract is an input: each enabled filter/lane has the
three stored `f32` coefficients `c1/a2/a3`, shared-or-lane `f32 k`, and two `f32` TPT integrator
state words. `c1` is the conditioned `f64`-prepared complement cast once to `f32`; SIMD must not
reconstruct it from `a1`. Four/eight-track adapters transpose `c1/a2/a3` and both states into
`f32x4`/`f32x8` vectors and preserve issue 007's exact incremental scalar operation graph. Base
scalar, Wasm SIMD, NEON, and AVX2 do not fuse; AVX2+FMA is separate. No backend may substitute
TDF-II, `f64`, double-single, compensated, or shared L/R state. Given identical coefficient bits,
finite-normal input, and no sanitation, base non-FMA scalar and SIMD are bit-identical on the same
target; cross-target and FMA paths also pass issue 007's response gates and the declared samplewise
tolerance. A future precision mode belongs to issue 031 and would require a new cohort/ABI decision.

The issue-007 post-stop rescope accepts that DSP operation graph only as technical input. Issue
034 stopped after landing bounded contract corrections; this issue waits for **Representable TPT
cutoff domain and builtin contract acceptance** because bank adapters consume its accepted
per-lane preparation/metadata and sealed graph/resource contract. It does not wait for
**Issue-007 builtin qualification tooling, audits, and benchmark**: that successor owns scalar
expected-output/audit/benchmark evidence and does not define SIMD semantics.

## Acceptance gates with objective measurements

Track-count fixtures 1–3, 4, 5–7, 8 and 9+ plus 100 deterministic randomized cohort/tail
layouts preserve independent L/R state and agree with scalar within the frozen backend tolerance;
AVX2-without-FMA and AVX2+FMA dispatch are both tested; native disassembly and Wasm inspection
prove the intended vector instructions; render reports 0 forbidden operations; and no compiled
track ceiling exists. Performance is descriptive: after preflight, run exactly one benchmark
invocation containing one warmup and two measured rounds. No speedup threshold, tuning or retry is
an acceptance gate; unexpected measurements become weekly optimization evidence.

## Target matrix

Native scalar/AVX2/FMA; ARM 4-lane equivalent; wasm32 scalar and simd128.

## Required evidence

Capability dispatch table, randomized differential results, layout assertions, allocation audit, and cycles/frame benchmarks.
