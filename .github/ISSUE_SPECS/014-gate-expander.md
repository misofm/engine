# 014 Gate/expander

## Outcome

Implement a dual-mono gate/expander for console-style cleanup with stable state transitions.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement threshold, range, ratio, hysteresis, attack, hold, release, detector filters, lookahead, external sidechain and explicit detector link modes.

## Required public interfaces/contracts

`GateExpander` implements `NativeEffect`; state machine transitions and gain floor are numeric/schema-visible; sidechain port requirements and latency/tail are declared.

## Deliverables

Effect code/note, metadata, state-transition fixtures, bank kernels, tests, benchmark and listening evidence.

## Explicit non-goals

Spectral denoising, arbitrary program-dependent learning, hidden sidechain routing, or allocation during process.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

Gate timing requires explicit sample-rate coefficient rules and finite behavior at silence/denormals; use the dynamics corpus established in issue 002.

## Acceptance gates with objective measurements

Static expansion/range curves match the independent design within 0.1 dB; threshold-crossing fixtures hit attack/hold/release timing within the greater of one sample or 2%; hysteresis prevents the frozen chatter fixture without hidden state transitions; sidechain route/PDC and lookahead/bypass impulses align exactly; 0 render alloc.

## Target matrix

Scalar, 4-lane, AVX2; all native/mobile/browser core paths.

## Required evidence

Transition traces, sidechain fixtures, stability/denormal results, benchmark, and listening evidence.
