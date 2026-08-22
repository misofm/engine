# 023 iOS and Android embedding examples

## Outcome

Provide native mobile host adapters/examples that negotiate device configuration and feed the shared realtime core safely.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; actual host rates outside that set reject or report reprepare-required, with no implicit SRC or 96 kHz fallback. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement iOS Audio Unit/AVAudioSession and Android AAudio/Oboe example adapters; preallocate/interleave-planar bridges, propagate actual sample rate/quantum/route state, and feed host-supplied source chunks.

## Required public interfaces/contracts

`MobileEngineHost` exposes `prepare(actual_rate, max_frames)`, `render`, `reconfigure`, and diagnostic counters; it never asks the engine to assume requested rate equals actual rate.

## Deliverables

Buildable iOS and Android examples, adapter code, configuration/reconfigure tests, docs, and device/CI matrix.

## Explicit non-goals

Shipping a mobile UI, forcing 384k hardware I/O, implicit SRC, callback allocation, or browser AudioWorklet.

## Dependencies by exact issue title

- Stable C ABI and native PCM reference runner
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification
- Real-time memory, buffers, queues, and plan lifetime
- Exact lock-free native source sanitation telemetry handoff

## Hazards/decisions

iOS preferred rate is not guaranteed and must be queried after activation: https://developer.apple.com/library/archive/qa/qa1631/_index.html. Android low-latency uses callback/no blocking and favors native rate: https://developer.android.com/games/sdk/oboe/low-latency-audio.

## Acceptance gates with objective measurements

An instrumented one-million-call offline callback audit and ten-minute device run record 0 alloc/free/lock/I/O in callback and no deadline miss; route/rate change never renders a stale plan, and an unsupported actual rate produces typed reprepare-required state rather than implicit SRC; Android/iOS compile and simulator/device PCM agree with the common-core fixture within its pre-briefed numerical tolerance.

## Target matrix

iOS ARM64 Audio Unit/AVAudioSession; Android ARM64 AAudio (Oboe permitted adapter); no browser.

## Required evidence

Build logs, callback audit, actual-rate traces, reconfigure fixture, device/simulator test record.
