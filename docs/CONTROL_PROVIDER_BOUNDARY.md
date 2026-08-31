# MISO control provider and adapter boundary

`protocol` is the controller and codec boundary, not a host adapter. A provider offers typed bounded capabilities, parameter pages/state/descriptor lookup, counters, diagnostics, absolute transport get/set, endpoint-local telemetry configuration, and endpoint current sample. It never accepts or returns a raw BTLV body, arbitrary bytes, PCM, a renderer, or a plan.

Providers cap fixture/catalog/page storage at construction and return typed `not found`, `unavailable`, or limit errors. The controller constructs capabilities from effective limits and provider feature flags; it owns canonical encoding and any replay-response bytes. Parameter metadata, state, counters, and diagnostics honor their typed cursor/ID/limit requests. Transport uses an absolute state/position/effective-sample triple only; seek execution and sources remain outside this issue.

An in-process caller, local IPC sidecar, browser message/shared-ring adapter, or future C ABI may adapt a complete BTLV frame. The adapter owns framing, authentication, reconnect, multiplexing, and peer lifetime; none is implied by BTLV. WebSocket is only an optional remote boundary and never a render-path dependency. A future ABI uses caller spans and opaque handles under the ownership rules in [the wire specification](CONTROL_BTLV_V1.md#codec-and-ffi-ownership).

The only v1 `BYTES` schema fields are canonical TOML snapshot chunks and validated fixed arrays for parameter state, automation, and meter records. They are not an escape hatch for media. Message IDs `0x6000..=0x6fff` reject as `PCM_FORBIDDEN`; no field describes audio channels, frames, encoded media, or an opaque provider payload.
