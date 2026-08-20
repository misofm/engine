# 019 Antialiased saturator/clipper

## Outcome

Implement a dual-mono saturator/clipper with disclosed nonlinearity and bounded antialiasing quality modes.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement drive, bias, symmetry, curve selection, threshold/ceiling, oversample factor, mix, DC handling, link mode and latency/tail declaration.

## Required public interfaces/contracts

`SaturatorClipper` implements `NativeEffect`; curve identifier and coefficients are numeric/versioned; oversampling state preallocates during prepare; output safety policy is explicit.

## Deliverables

Algorithm note, curve/oversampling code, spectral fixtures, metadata, SIMD kernels, benchmarks and listening evidence.

## Explicit non-goals

Undocumented magic curves, automatic normalization, runtime resampler allocation, or true-peak guarantee unless issue 016 is composed.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Hazards/decisions

State equation, discontinuity handling, oversampling filter and DC policy must be disclosed; test aliases rather than asserting subjective quality.

## Acceptance gates with objective measurements

Before production code, the Sol-approved DSP note freezes a signal corpus and research-justified alias-energy improvement required of each advertised antialiased mode relative to 1x; transfer curve and THD match the independent `f64` design within its frozen tolerance; zero drive/null mix nulls within 1e-6; one million extreme finite inputs remain finite; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm with declared quality limits.

## Required evidence

FFT/spectrum data, null/stress fixtures, cycles/frame/RAM table, and listening evidence.
