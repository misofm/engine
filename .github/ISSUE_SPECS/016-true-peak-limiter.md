# 016 True-peak limiter

## Outcome

Implement a safety limiter with explicit oversampling, true-peak measurement, lookahead and reported latency.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement ceiling, threshold, release shapes, lookahead, explicit 1x/2x/4x/8x quality availability, detector link, ISP/true-peak mode, gain law and metering; all oversampling buffers preallocate in prepare and any latency-changing mode requires plan reprepare.

## Required public interfaces/contracts

`TruePeakLimiter` implements `NativeEffect`; `OversampleFactor` and true-peak definition are explicit; `latency_samples` includes lookahead/filter delay; ceiling unit is dBTP/dBFS as declared.

## Deliverables

Algorithm/research note, oversampling kernels, true-peak fixtures, SIMD path, benchmark, listening evidence and metadata.

## Explicit non-goals

Claiming a sample-peak limiter is true peak, undeclared latency, dynamic oversampling allocation, or delivery encoding.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Compressor

## Hazards/decisions

Use the BS.1770-5 true-peak measurement method and conformance context: https://www.itu.int/rec/R-REC-BS.1770-5-202311-I. Document interpolation/filter and numerical limits; do not infer a limiter gain-control design from the measurement standard.

## Acceptance gates with objective measurements

The current ITU/EBU conformance sequences and adversarial intersample fixtures stay at or below the configured ceiling +0.1 dBTP in every mode advertised as true peak; reported latency equals impulse measurement exactly and bypass preserves it; oversample/lookahead changes require safe plan reprepare; one million finite stress samples produce no NaN/Inf; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm with quality capability disclosure.

## Required evidence

True-peak fixture report, impulse latency plot, CPU/RAM per quality table, stability results, and listening evidence.
