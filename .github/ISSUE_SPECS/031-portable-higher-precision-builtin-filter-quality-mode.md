# 031 Portable higher-precision builtin filter quality mode

## Outcome

Determine whether Engine V2 should offer an explicit production higher-precision HPF/LPF mode,
and add it only if measured audio benefit justifies its portable state, ABI, memory, and CPU cost.

## Context

Issue 007 uses a conditioned two-state `f32` trapezoidal/TPT realization for launch Butterworth
HPF/LPF. Its independent `f64` implementation is an oracle, not production. Issue 008 banks lanes
as Wasm/NEON `f32x4` and AVX2 `f32x8`. This issue is stateless and does not assume wider precision
is better or required. Never inspect, copy, benchmark, or inherit V1/legacy work.

## Scope

Compare scalar `f64`, paired-`f32`/double-single or compensated state, and accepted `f32` TPT.
Decide whether one candidate becomes an explicit prepared quality mode. Define session/control
metadata, cohort signature, reset/state serialization, target lowering, resources, and fallback.

## Explicit non-goals

Silently widening launch mode, changing response/cutoff semantics, treating the oracle as
production, requiring nonexistent Wasm `f64x4` or AVX2 `f64x8`, or weakening issue-007 gates.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Dual-mono builtins and metering
- Production SIMD builtin bank graph retention and reachability qualification

## Required evidence and acceptance

Freeze equations, rounding, state representation/conversion, and scalar/Wasm/NEON/AVX2/FMA
lowering. Preregister a falsifiable adoption threshold, then collect all-rate/all-cutoff analytic,
impulse, coherent-sine, residual-noise, DC, stopband, fault, listening, memory, and cycles/frame
evidence against issue 007 and an independent oracle. An adopted mode is explicit in immutable
metadata and cohort signature, never auto-selected; it preserves zero latency, tail/reset/sanitize
semantics, dual-mono isolation, deterministic scalar repetition, bounded SIMD differential error,
zero render allocation/free, and no track ceiling. If benefit is not repeatable or portable cost is
disproportionate, close with launch `f32` unchanged.
