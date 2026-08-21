# 024 WASM SIMD AudioWorklet embedding

## Outcome

Embed the core in a browser AudioWorklet with a single-threaded safe baseline and optional shared-memory control acceleration.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; actual host rates outside that set reject or report reprepare-required, with no implicit SRC or 96 kHz fallback. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement `wasm32-unknown-unknown` build, AudioWorklet module/processor, preallocated Wasm memory bridge, planar f32 conversion, control/event adapter, capability detection and COOP/COEP documentation for SAB mode.

## Required public interfaces/contracts

`AudioWorkletEngineHost` accepts actual `sampleRate`/quantum and bounded command events; processor invokes only precompiled/preallocated core; `simd128` is capability/build-selected and scalar fallback is functional.

## Deliverables

Browser example, worklet/JS glue, build scripts, render/control fixtures, header/deployment docs and telemetry.

## Explicit non-goals

Wasm filesystem streaming, hardcoding 128 frames, relying on Rust std threads, third-party Wasm executor, network/WebSocket in processor, or browser multicore rendering.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Real-time memory, buffers, queues, and plan lifetime
- Transport-neutral binary control protocol
- JIT PCM streaming and host-supplied source rings
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Issue-007 builtin qualification tooling, audits, and benchmark

## Hazards/decisions

Web Audio has a render/control model and f32 worklet buffers: https://www.w3.org/TR/webaudio-1.1/. Rust browser Wasm lacks filesystem/threads baseline: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html.

## Acceptance gates with objective measurements

Worklet fixture output agrees with native scalar within the conformance tolerance frozen by issue 002 before adapter code; one million offline `process` calls and a ten-minute live worklet run produce 0 Wasm/core allocations or memory growth after prepare; scalar and SIMD128 variants load correctly; quantum test passes 64/128/256 frames without assuming the browser’s current block size; SAB and message fallback paths are tested with typed backpressure.

## Target matrix

Modern browser wasm32 scalar mandatory; simd128 optional; AudioWorklet required; single render thread at launch.

## Required evidence

Browser test logs, deployment headers, audio fixtures, allocation counters, capability matrix, and bundle-size report.
