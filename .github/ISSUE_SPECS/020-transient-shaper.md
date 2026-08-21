# 020 Transient shaper

## Outcome

Implement a dual-mono transient shaper with explicit envelope, separation and lookahead behavior.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement attack/sustain amounts, detector speed, sensitivity, range, link mode, optional lookahead, mix and quality modes using bounded state.

## Required public interfaces/contracts

`TransientShaper` implements `NativeEffect`; envelope/difference equation and timing units are documented; latency/tail and parameter smoothing are declared.

## Deliverables

Effect/note, impulse/decay fixtures, metadata, SIMD kernels, tests, benchmark and listening evidence.

## Explicit non-goals

Source classification, unbounded history, hidden auto-gain, or implicit channel linkage.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- Launch feed-forward peak compressor

## Hazards/decisions

Transient processing must document how attack and sustain paths are separated and safe behavior for zero/silence/denormals.

## Acceptance gates with objective measurements

Impulse and decaying-envelope gain match the independent design within 0.1 dB and timing within the greater of one sample or 2%; behavior is invariant across all four launch rates and tested quanta within the frozen scalar tolerance; null setting preserves signal within 1e-6; no NaN/Inf after one million samples; 0 render alloc.

## Target matrix

Native scalar/AVX2 and 4-lane portable/Wasm.

## Required evidence

Envelope plots, null/stress results, performance data, and listening evidence.
