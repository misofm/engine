# 013 Compressor

## Outcome

Implement a dual-mono compressor with explicit detector linking and fully numeric control surface.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement feed-forward and feedback topologies; peak and RMS detector options; threshold, ratio, knee, attack/release, hold, makeup, mix, lookahead, detector HPF/LPF, external sidechain, link modes and quality modes with declared latency/tail.

## Required public interfaces/contracts

`Compressor` implements `NativeEffect`; `DetectorLinkMode` and timing units are explicit; all sample-time updates derive from provided sample rate; lookahead reports integer latency.

## Deliverables

Effect implementation/note, metadata, bank kernels, deterministic envelope fixtures, stability tests, benchmark and listening evidence.

## Explicit non-goals

Auto-gain guesses, hidden channel coupling, implicit lookahead compensation, multiband behavior, or realtime allocation.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Hazards/decisions

Ground detector/envelope choice in Giannoulis et al.: https://eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf.

## Acceptance gates with objective measurements

Static gain curves match the independent design within 0.1 dB; step/sine envelope attack and release timing match within the greater of one sample or 2%; reported lookahead equals impulse delay exactly and bypass preserves it; feed-forward/feedback, peak/RMS, sidechain and every link mode match fixtures; one million extreme samples remain finite; 0 render alloc.

## Target matrix

Native scalar/AVX2, 4-lane ARM/Wasm SIMD; no host-specific dynamics path.

## Required evidence

Envelope traces, latency impulse data, scalar/SIMD differential report, CPU benchmark, and listening evidence.
