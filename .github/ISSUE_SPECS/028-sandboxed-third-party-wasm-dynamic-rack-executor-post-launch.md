# 028 Sandboxed third-party WASM dynamic-rack executor (post-launch)

## Outcome

Post-launch only: execute validated third-party effects exclusively in the dynamic rack while preserving render safety and declared latency.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement off-thread compilation/validation/instantiation and processing on dedicated sandbox workers, a capability-free import policy, deterministic fuel plus wall-deadline policy, bounded queues/memory, preallocated bridge buffers, at least one-quantum declared-latency pipeline, fault/late-result bypass, state reset and telemetry. The audio callback never calls into a general-purpose Wasm runtime.

## Required public interfaces/contracts

`DynamicWasmExecutor` accepts only validated `ThirdPartyEffectAbiV1` packages; `WasmInstancePlan` is prepared before render and owned by a sandbox worker; the render path only exchanges bounded generation-tagged audio/event blocks; `FaultPolicy::BypassPreservingLatency` keeps the declared pipeline/PDC latency stable and emits a counter/event.

## Deliverables

Runtime adapter, scheduling/budget specification, fault tests, latency-preserving bypass implementation, target capability matrix, security review and performance report.

## Explicit non-goals

SIMD rack placement, WASI/syscalls/network/filesystem, JIT/compile in render, zero-latency promise, native promotion, or launch release inclusion.

## Dependencies by exact issue title

- Third-party WASM package and effect ABI conformance kit
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Real-time memory, buffers, queues, and plan lifetime
- End-to-end release, performance, and listening qualification

## Hazards/decisions

Dynamic-only decision is deliberate: opaque Wasm cannot uphold homogeneous/fused AoSoA SIMD bank assumptions. Wasm threads/shared memory do not themselves create threads: https://webassembly.github.io/threads/core/appendix/changes.html.

## Acceptance gates with objective measurements

Every malformed, trapping, fuel-exhausted or late effect produces the specified latency-preserving bypass block while late/stale generations are discarded and impulse latency remains exact; one million render calls make zero allocator/lock/I/O/Wasm-runtime-entry calls; declared maximum memory is enforced and `memory.grow` during processing traps; packages have no WASI imports; the adverse corpus cannot crash or block the render host.

## Target matrix

Post-launch native first; browser only after separate AudioWorklet/runtime budget qualification; never a SIMD rack target.

## Required evidence

Security review, adversarial package corpus results, latency/PDC fixtures, allocator audit, deadline distribution, and explicit post-launch release decision.
