# 025 Optional binary WebSocket sidecar

## Outcome

Implement an optional remote-control sidecar that adapts the shared binary protocol without placing transport work in the engine render path.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Build authenticated/version-negotiated WebSocket service, session/control endpoint mapping, binary framing/backpressure limits, local engine IPC adapter, meter/diagnostic subscriptions and observability.

## Required public interfaces/contracts

`WebSocketSidecar` maps only `ProtocolCodec` frames to local control operations; it declares max frame/queue/subscription limits and never accepts/forwards PCM render buffers.

## Deliverables

Sidecar crate, authentication/configuration docs, protocol compatibility tests, load/backpressure tests, local IPC boundary and deployment example.

## Explicit non-goals

Replacing in-process API, streaming PCM, invoking engine render from socket callbacks, or making WebSocket mandatory for browser/native embedding.

## Dependencies by exact issue title

- Transport-neutral binary control protocol
- Stable C ABI and native PCM reference runner

## Hazards/decisions

RFC 6455 is a bidirectional TCP-framed remote transport, not local render IPC: https://www.rfc-editor.org/rfc/rfc6455.html.

## Acceptance gates with objective measurements

Golden protocol frames round-trip unchanged; unauthenticated, unauthorized, oversized, rate-limited and malformed requests reject without engine mutation; reconnect/revision-conflict behavior is deterministic; saturated clients drop/coalesce only declared telemetry with counters and receive typed control backpressure without delaying the engine control worker; render allocation and P99.99 latency stay within the no-sidecar baseline’s statistical confidence interval under maximum configured sidecar load.

## Target matrix

Cloud/native sidecar only; browser uses browser transport adapter; no audio-thread use.

## Required evidence

Protocol traces, load-test p50/p99/queue data, security configuration test, and render isolation audit.
