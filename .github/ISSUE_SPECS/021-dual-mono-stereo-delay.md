# 021 Dual-mono/stereo delay

## Outcome

Implement a bounded delay effect that can run independent dual-mono paths or an explicit smoothed stereo matrix.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement per-channel delay time, sync-free sample/seconds units, feedback, damping/filter, wet/dry, ping-pong or 2x2 matrix, modulation bounds, taps, tail and quality modes.

## Required public interfaces/contracts

`Delay` implements `NativeEffect`; delay-memory maximum is negotiated in prepare; `Matrix2x2`/link behavior is explicit and smoothed; `tail_samples` is conservative and finite or flagged infinite when feedback policy permits.

## Deliverables

Delay-line implementation/note, parameter metadata, interpolation fixtures, max-memory validation, SIMD strategy, benchmark and listening evidence.

## Explicit non-goals

Unbounded delay allocation, implicit stereo wiring, tempo engine, streaming source code, or fractional PDC.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Dual-mono builtins and metering

## Hazards/decisions

Delay time changes must state interpolation/crossfade and coefficient-update rules; feedback must have finite/NaN safeguards.

## Acceptance gates with objective measurements

Before production code, the Sol-approved DSP note freezes the fractional-delay passband, delay-error and magnitude-ripple limits from the chosen interpolation method; integer-delay impulses are sample-exact and fractional-delay phase/group-delay measurements meet those limits; L-only dual-mono does not excite R unless an explicit matrix does so; maximum delay uses only prepared memory; feedback stress stays finite or follows the declared safety clamp; latency/tail and bypass behavior match the contract; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm; mobile/browser honor host memory caps.

## Required evidence

Impulse/tail fixtures, memory-cap report, modulation stress data, CPU benchmark, and listening evidence.
