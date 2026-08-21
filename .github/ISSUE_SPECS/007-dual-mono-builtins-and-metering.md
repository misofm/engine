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

## Sol implementation brief (2026-08-21)

**READY for Terra attempt 1.** The normative implementation-grade brief is
`target/issue7-sol-brief.md`. It freezes the three distinct builtin graph sections, RBJ
second-order Butterworth HPF/LPF scalar reference semantics, explicit matrix/pan equations and
N-update smoothing, lane-isolated reset/sanitization, transparent observers at all seven accepted
track boundaries, interval peak/RMS/energy and held-peak meter state, transactional preparation,
resource/no-allocation gates, target/fixture/listening evidence, and one exactly-once benchmark
invocation containing two descriptive internal rounds.

The brief does not change the accepted V1 TOML schema, issue-006 graph topology/PDC/reduction
contract, issue-011 effect contract, or issue-008 SIMD ownership. “Loudness-ready” is explicitly
bounded to timestamped per-lane energy observations and loss accounting; BS.1770 K-weighting,
gating, LUFS/LKFS, true peak, and certification are not issue-007 claims. No implementation or
benchmark was performed during briefing, and no V1/legacy source was inspected.

## Terra attempt 1 evidence (2026-08-21; partial)

Implementation added scalar dual-mono input and output builtin sections, explicit matrix/pan
smoothing, bounded meter accumulators, and prepared graph observer bindings without changing the
V1 TOML schema, graph topology, effect contract, or SIMD ownership. The graph compiler accepts a
complete prepared-builtin artifact transactionally, verifies canonical session/rate/quantum
identity, binds exactly the three internal stages, returns meter consumers, and propagates the
prepared filter tail to the post-input graph node.

PASS so far: focused unit/integration tests for gain/matrix/meter behavior, all seven prepared
tap requests, transactional invalid requests, internal binding ownership, tail propagation, and
the existing 65,537-track graph resource test; format, diff check, and warning-denied focused
Clippy also pass. Release scalar-builtins checks pass for `aarch64-linux-android`,
`aarch64-apple-ios`, and `wasm32-unknown-unknown` both with `-simd128` and `+simd128`. The
separate `miso-engine-dsp-reference` f64 RBJ oracle now covers an impulse through both filters at
all eight required rates; 10,000 deterministic scalar parameter/block mutations remain finite.
The complete workspace test suite and warning-denied all-target Clippy pass. The exactly-once
benchmark invocation count is **0**.

NOT YET SATISFIED: fixture-manifest corpus and independent f64 sweep oracle, full rate/quantum
matrix/meter mutation and allocation audits, issue-specific one-million-call tooling, full
workspace cross-target checks, real blinded listening records, and the authorized single
benchmark. These remain gates;
this evidence does not claim issue completion.
