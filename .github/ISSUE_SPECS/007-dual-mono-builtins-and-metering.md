# 007 Dual-mono builtins and metering

## Outcome

Implement the fixed per-track processing contract and objective meters before user effects.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement independent L/R input polarity, trim and HPF/LPF; output fader, mute and pan/explicit smoothed 2x2 matrix; every named tap observation point; and peak/RMS/loudness-ready meters. These are the two builtin sections surrounding the racks, not a second ambiguous all-in-one slot.

## Required public interfaces/contracts

`BuiltinChain::process_dual_mono` accepts distinct channel states; `ChannelLinkMode` is explicit; `Matrix2x2` has smoothing time and coefficient bounds; `MeterSnapshot` includes sample-time and counter reset semantics.

## Deliverables

Builtin implementation, parameter metadata, smoothing policy, meter accumulators, fixtures and documentation.

## Explicit non-goals

Implicit stereo linking, loudness certification, effect racks, graph routing, or hidden coefficient jumps.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

## Hazards/decisions

Dual mono never aliases L and R state. Matrix changes must smooth and remain finite; filter notes cite RBJ: https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html.

## Acceptance gates with objective measurements

Polarity/trim/fader impulses match analytic gain within 1e-6; conventional pan/balance adapters match their documented matrix and pan law within 1e-6; HPF/LPF magnitude matches the independent `f64` design within 0.05 dB where defined and remains stable at every required rate; matrix ramp has no NaN/Inf and obeys its frozen per-sample slew bound; L-only input never changes R absent an explicit non-diagonal matrix; render allocation count is 0.

## Target matrix

Scalar core on all targets. Issue 008 adds and qualifies the 4/8-lane bank adapters without changing builtin semantics.

## Required evidence

Impulse and sweep fixtures, meter comparison data, allocation audit, benchmark, and listening record for matrix/filter changes.
