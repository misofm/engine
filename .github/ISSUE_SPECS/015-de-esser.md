# 015 De-esser

## Outcome

Implement a transparent dual-mono de-esser with disclosed detection and attenuation behavior.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement wideband and split-band attenuation, selectable band detection, frequency/bandwidth, threshold, ratio/range, attack/release, listen output, detector link mode, external sidechain, optional lookahead and declared quality modes.

## Required public interfaces/contracts

`DeEsser` implements `NativeEffect`; detector/filter topology and listen port are exposed; latency/tail and all frequency/dynamics units are numeric metadata.

## Deliverables

Algorithm note, effect code, fixtures with sibilance/noise controls, scalar/4-lane/8-lane kernels, tests, benchmark and listening evidence.

## Explicit non-goals

Machine-learning speech classification, undocumented dynamic EQ replacement, implicit stereo detection, or realtime FFT allocation.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Parametric EQ
- Compressor

## Hazards/decisions

Document the chosen filter/detector equations and cite primary sources in the DSP note; tests must distinguish desired-band attenuation from broadband gain change.

## Acceptance gates with objective measurements

In split mode, tones outside the affected band change by no more than 0.1 dB where the documented crossover response is nominal; in-band burst gain reduction matches the independent design within 0.1 dB and timing within the greater of one sample or 2%; wideband, split, listen, sidechain and link outputs are deterministic; one million samples produce no NaN/Inf; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm core.

## Required evidence

Band-response plots, burst traces, scalar/SIMD comparisons, cycles/frame, and blinded vocal listening record.
