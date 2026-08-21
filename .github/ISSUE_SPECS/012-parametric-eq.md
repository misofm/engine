# 012 Parametric EQ

## Outcome

Implement a high-quality dual-mono parametric EQ as a bankable native effect.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Provide bell, low/high shelf, low/high pass and notch sections with numeric section count/ordering, bypass, gain, Q/bandwidth/slope, frequency, per-channel state and explicit link mode.

## Required public interfaces/contracts

`ParametricEq` implements `NativeEffect`; `EqBandDescriptor` uses Hz/dB/Q-or-bandwidth/unit fields; coefficient changes use declared smoothing/interpolation and expose exact latency/tail.

## Deliverables

Effect code, research note, metadata, scalar/4/8-lane kernels, fixtures, tests, benchmark and listening evidence.

## Explicit non-goals

Dynamic EQ, linear-phase/FIR modes, third-party Wasm, implicit channel links, or arbitrary section allocation in process.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels

## Hazards/decisions

Use RBJ equations and document numerical handling: https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html. Process is no-alloc and L/R state is independent.

## Acceptance gates with objective measurements

Magnitude response matches the independent `f64` design within 0.05 dB where defined and cutoff/center frequency within 0.1% at all four launch rates; extreme legal parameters stay finite for one million samples; scalar/4/8-lane residual tolerance is frozen in the Sol brief before production code; parameter automation is continuous under the documented smoothing bound; 0 render alloc.

## Target matrix

Native scalar/AVX2 and portable 4-lane/Wasm SIMD; host adapters consume same core.

## Required evidence

Coefficient tables, response plots, randomized stability tests, cycles/frame results, and blinded listening notes.
