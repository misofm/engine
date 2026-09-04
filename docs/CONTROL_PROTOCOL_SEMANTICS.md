# MISO control protocol semantics

This defines controller behavior for the [BTLV bytes](CONTROL_BTLV_V1.md) and [registry](CONTROL_PROTOCOL_REGISTRY.md). It does not publish a plan, execute a seek, define DSP parameter meanings, or transport PCM.

## Revision, transaction, and replay

The committed `SessionModel` owns its revision and may use the full `u64` domain; command `revision-any` is therefore the header flag plus a zero carrier, never a revision sentinel. Structural edits require an exact revision and at least one edit; a configured zero edit limit disables the transaction command and its paired session events in capabilities and dispatch. Transactions clone/apply/compile a candidate typed session and replace `(SessionModel, CompiledSession, SessionRevision)` only when all validation, capacity reservation, and compilation steps succeed. One success increments once; malformed input, empty input, validation/compile/resource failure, backpressure, and replay leave model and revision untouched. A mutation at `u64::MAX` returns `REVISION_EXHAUSTED`; the controller does not turn a compiled session into a `PreparedRenderPlan`.

All v1 operations are absolute and idempotent. A new logical command has a nonzero, strictly increasing endpoint-local request ID. The bounded replay cache stores exact canonical request and response bytes. Identical repeated bytes return the cached response without execution; same ID with changed bytes returns `REQUEST_ID_REUSE`; evicted/retired IDs return `REPLAY_EXPIRED`. Completed backpressure is replayed too, so retrying it uses a new ID. Cache capacity is reserved before execution; an unretainable response is rejected before state changes. The guarantee ends with the endpoint lifetime; reconnect recovery belongs to an adapter.

## Automation and events

Automation requires exact revision, does not increment revision, and is submitted wholly or not at all. The endpoint supplies current sample time; a client cannot choose submission time. A past batch rejects atomically. A later race applies deterministic catch-up and increments the late counter instead of silently dropping accepted work. Records for one parameter may not overlap within one batch or across separately queued batches; batches cannot move global time backwards, and aggregate queued starts cannot exceed prepared per-block density.

**Delivery status (v1).** Accepted automation is retained in the queue and consumed today only by cancellation (`AUTOMATION_CANCELED`). No render-side drain exists: sample-accurate application of point, step, linear and exponential records is the later protocol capability named in `EFFECT_CONTRACT_V1.md` ("V1 runtime automation is `Point` spans …"). The catch-up rule and the late counter in the paragraph above are specified but not implemented; `LATE_AUTOMATION` is registered and nothing increments it.

Changing revision or locating transport never silently deletes accepted automation. It produces reliable `AUTOMATION_CANCELED` with reason (revision change, locate, endpoint shutdown, provider unavailable, or explicit reconfiguration), increments the saturating cancellation counter, and starts a new automation-ordering epoch after the queued work is drained. A state-only transport set does not cancel automation. `SESSION_COMMITTED`, cancellation, transport state, and diagnostics are reliable; meter batches and periodic counter snapshots are the only lossy events. A reliable event has a monotonic sequence and queue capacity is reserved before its related commit.

## Queues and telemetry

Control commands, automation batches, responses, and reliable events reject before related state changes when full and preserve the original item. Automation is one fixed slot containing at most 256 records, so 10,000 records are 39 full slots plus one 16-record slot: 40 atomic admissions. Telemetry may replace only the latest same `(revision, handle/component)` value, incrementing `telemetry_coalesced`; a new key rejected at capacity increments `telemetry_dropped`. Counters are non-resetting and saturate at `u64::MAX`.

Render-side consumption has a fixed work bound and does no BTLV decode, allocation/free, lock, logging, I/O, syscall, or structural mutation. Queue configuration and formulas are in the [sizing guide](CONTROL_PROTOCOL_SIZING.md). Typed `BACKPRESSURE` identifies queue kind, capacity, pre-attempt occupancy, requested slots, generation, and applicable byte/retry values.

## Snapshots, telemetry, and deferred ownership

Snapshot chunks form canonical UTF-8 only when reassembled; a chunk may split a code point. An any-revision first page returns its observed revision; continuations are exact or conflict. Parameter and meter handles are revision-scoped and never renderer parameter slots. Telemetry configuration is endpoint-local. Providers return typed bounded records, and adapters own transport framing; their boundary is [documented here](CONTROL_PROVIDER_BOUNDARY.md).
