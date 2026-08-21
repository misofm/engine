# 008 AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Outcome

Build the bankable effect-rack execution substrate without sacrificing dual-mono semantics or portability.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; extended-rate SIMD qualification is deferred. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement planar `f32` AoSoA across tracks so each vector at a sample contains the same L or R lane from four Wasm/NEON tracks or eight AVX2 tracks. Compile cohorts sharing effect types/order, quality and compatible routing; keep per-track parameters/state; use identity kernels for absent slots; and implement scalar/tail execution plus separate AVX2/FMA dispatch. Base Wasm SIMD uses multiply-plus-add; optional relaxed SIMD cannot change correctness requirements.

## Required public interfaces/contracts

`BankWidth` is 1/4/8; `RackProgramSignature` declares slot types/order, layout, quality and routing compatibility; `RackKernel` holds distinct vectors/state for both dual-mono lanes; `KernelDispatch::select(CpuCaps)` selects scalar, AArch64 NEON, AVX2 and FMA independently, while Wasm scalar/SIMD artifacts are host-selected; no public contract exposes unsafe SIMD registers.

## Deliverables

AoSoA buffers, rack compiler, scalar/SIMD kernel traits, runtime dispatch, feature tests, kernel benchmarks, and debug layout validator.

## Explicit non-goals

Placing arbitrary third-party Wasm in a SIMD bank, fixed global eight-track assumption, global AVX2 requirement, or cross-channel state sharing.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Dual-mono builtins and metering

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

## Acceptance gates with objective measurements

Track-count fixtures 1–3, 4, 5–7, 8 and 9+ plus 100 randomized cohort/tail layouts preserve independent L/R state and agree with scalar within the frozen per-effect tolerance; AVX2-without-FMA and AVX2+FMA dispatch are both tested; disassembly/Wasm inspection proves the intended vector instructions; on a pinned canonical eight-track bank each SIMD backend is statistically no slower than scalar and must show a positive speedup or produce a profile-backed Sol-approved rescope before acceptance; 0 render alloc/free; no compiled track ceiling.

## Target matrix

Native scalar/AVX2/FMA; ARM 4-lane equivalent; wasm32 scalar and simd128.

## Required evidence

Capability dispatch table, randomized differential results, layout assertions, allocation audit, and cycles/frame benchmarks.
