# Get a track's response on the C ABI

Slice S20b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13), second half of the planner's S20: the protocol and the C ABI. Code anchors verified on
`main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

An iOS or Android app asks for the live response of a track (the combined curve of its input
filters and EQs, from the values its plan renders, with the list of the owners that make it) with
`RESPONSE_GET` and a `strip_id`. The reply carries a request tag at once; the curve arrives as a
`RESPONSE_READY` event after the next render block, through `miso_engine_v1_dequeue_event`. The
browser gets the same frames in #1579. A mobile app now has the track response that only the
browser had.

## Context

- **The wire is half frozen.** #1573 D2 and D3: target 3 `strip_id`; the reply's field 1
  `request_tag`; #1573 D7 refuses target 3 with `response.target.unavailable`. #1573 D1 leaves
  event `0x8040` to this slice.
- **The capture is built.** #1577: `ResponseCaptureClient::{arm, poll, release}` in the provider,
  the render step in `PlanState::render`, the pending result in `SessionState.response_ready`.
- **Events.** `MessageId` events (`crates/protocol/src/wire.rs:122-133`; `allocated_id`,
  `crates/protocol/src/message_wire.rs:3534-3540`); the capability registry's event loop
  (`crates/protocol/src/controller.rs:3240-3256`) and invariants
  (`crates/protocol/src/message_wire.rs:3495-3532`). The reliable queue holds only fixed payloads
  (`ReliablePayload`, `crates/protocol/src/queue.rs:309`); a large event is held by reference in
  controller storage, as a diagnostic is: `enqueue_diagnostic_event`
  (`crates/protocol/src/controller.rs:2524-2568`), its slot cleared after a successful encode in
  `dequeue_reliable_event_frame_into` (`:2573-2593`), encoded at `:2707`. A reliable event carries
  an endpoint-monotonic sequence (`docs/CONTROL_PROTOCOL_REGISTRY.md:43`).
- **The C ABI entries.** `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:685`),
  `miso_engine_v1_render_f32_planar` (`:850`), `miso_engine_v1_service` (`:652`),
  `miso_engine_v1_dequeue_event` (`:768`, served by `SessionState::dequeue_event`,
  `crates/control-plane/src/control.rs:1673`).

## Decisions frozen for this slice

- **D1. The event.** `MessageId::ResponseReady = 0x8040` (`RESPONSE_READY`, reliable). The
  allocated events add `0x8040`. The capability registry lists it exactly when `features.response`
  is set; the invariants add `has_command(0x000e) == has_event(0x8040)`, and bit 14 means both.
- **D2. Event payload** (`response_ready`): 1 `sequence` `U64` R; 2 `request_tag` `U64` R;
  3 `strip_id` `Utf8` R; 4 `outcome` `U16` R (a `StatusCode` value: `OK` 0, `NOT_FOUND` 13,
  `UNAVAILABLE` 14, `LIMIT_EXCEEDED` 6, `INTERNAL` 18); 5 `diagnostic` `Utf8` O (the code;
  present exactly when `outcome` is not 0); 6 `captured_sample` `U64` O; 7 `sample_rate_hz` `U32`
  O; 8 `floor_db` `F32` O; 9 `frequencies_hz` `PackedF32` O; 10 `left_db` `PackedF32` O;
  11 `right_db` `PackedF32` O; 12 `excluded_count` `U32` O; 13 `member` `Message` O*
  (`response_member`: 1 `section` `U8` R (builtins 4, console 5, inserts 2), 2 `slot_id` `Utf8` O
  (present exactly for console and inserts), 3 `effect` `Utf8` R (native ID), 4 `kind` `U8` R
  (parametric EQ 1, input filters 2, unavailable 3), 5 `bypassed` `Bool` R, 6 `enabled_left` `U32`
  R, 7 `enabled_right` `U32` R, 8 `section_count` `U8` R, the number of valid mask bits). Fields 6-9 and 12 are present exactly when `outcome` is 0, with
  at least one of 10 and 11 of the length of 9. The codec refuses every other shape.
  `protocol` gains `response_ready_encoded_bound(points, channels, owners) -> Option<usize>`
  (identities at 127 bytes).
- **D3. The request.** `SessionControlProvider::response_get` serves target 3: the controller
  first refuses with `BACKPRESSURE` `response.capture.pending` while its `RESPONSE_READY` slot is
  occupied; then the provider refuses a bound from `response_ready_encoded_bound` above the
  smaller of the endpoint frame limit and request field 18 (`LIMIT_EXCEEDED`
  `response.reply_too_large`) and maps `arm`'s refusals:
  `UnknownStrip` -> `NOT_FOUND` `control.address.unknown`; `Pending` -> `BACKPRESSURE`
  `response.capture.pending`; `Grid` -> `INVALID_FIELD` `response.grid` (`response.points` for
  the count). On success the reply is `response_result { request_tag }`. The ack is written only
  after the job is in the queue.
- **D4. Delivery, never a drop.** `SessionState::service` moves `response_ready` into the
  controller with `enqueue_response_ready_event(revision, event)`, which holds the event in one
  retained slot and queues `ReliablePayload::ResponseReady { event_sequence }`. If the slot is
  full, the queue is full or a structural transaction is outstanding, the event stays in
  `SessionState.response_ready` and the next `service` retries. After a successful encode in
  `dequeue_reliable_event_frame_into`, the slot is cleared and `SessionState` calls the client's
  `release`. Every accepted request gets exactly one `RESPONSE_READY`.
- **D5. Outcome mapping.** `TrackResponse` fills fields 6-13; `ResponseCaptureFailure` maps to
  `NOT_FOUND` `response.strip.removed`, `UNAVAILABLE` `response.effect.unsupported`,
  `LIMIT_EXCEEDED` `response.limit` and `INTERNAL` `response.capture`.
- **D6. Docs.** The registry rows for target 3 and event `8040`; the semantics: one capture in
  flight per session; the result comes from the first successful render block of the newest plan
  after the request; a host that does not render does not get it; a replayed request returns the
  same tag and arms nothing. The C header's comments on `miso_engine_v1_submit_command` and
  `miso_engine_v1_dequeue_event` name the pair. No C symbol or struct changes.

## Deliverables

1. D1 and D2 in `crates/protocol` (wire, schema, codec and bound, typed event frame, controller
   slot, enqueue, dequeue and capability).
2. D3 and D5 in `crates/host-core/src/control_provider.rs`; D4 in
   `crates/control-plane/src/control.rs`.
3. The conformance corpus (an OK and a refused `RESPONSE_READY`, a target-3 request and reply) and
   the re-pinned `COMPLETE_SCHEMA_HASH`; D6.

## Authorized paths

- `crates/protocol/src/{wire.rs,schema.rs,message_wire.rs,message_wire/tests.rs,typed_frame.rs,
  queue.rs,controller.rs,controller/tests.rs,lib.rs}`.
- `crates/host-core/src/{control_provider.rs,response_capture.rs}`.
- `crates/control-plane/src/control.rs`.
- New `crates/capi/tests/response_strip.rs`; `crates/capi/include/miso_engine_v1.h` (comments).
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`, `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING}.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No browser change (#1579). No committed-model track response (the semantics stay applied).
- No new C export.

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`bash scripts/check-capi-abi.sh`.

In `crates/capi/tests/response_strip.rs`, on a 48 kHz session (1 MiB frame limit) whose track 1
has asymmetric input filters, an EQ insert and an EQ console slot, and a compressor insert:

1. **End to end.** `RESPONSE_GET` with `strip_id` track 1 replies `request_tag` 1. Before a
   render, `miso_engine_v1_dequeue_event` returns no `RESPONSE_READY`. After one render and one
   `miso_engine_v1_service`, it returns one `RESPONSE_READY` with tag 1, outcome 0, four members
   in signal order (builtins, console, inserts, inserts with kind 3) and curves bit-equal to #1576's
   `compose_track_response` over a `PreparedHost` capture of a host prepared from the same
   document.
2. **One in flight, none lost.** A second strip request before the event is dequeued is
   `BACKPRESSURE` `response.capture.pending`. With the reliable event queue filled by session
   events that the host does not dequeue, a completed capture is held; after the host drains the
   queue and calls `service`, exactly one `RESPONSE_READY` with tag 2 arrives.
3. **Removed strip.** A request on track 2, then a transaction that removes track 2, then two
   renders with `service`: one event, outcome `NOT_FOUND`, diagnostic `response.strip.removed`.
4. **Codec and capability.** Round trip of both event forms; refusal of an OK event without
   frequencies and of a refused event with curves; the capability lists `0x8040` with `0x000e`,
   and a peer capability that lists one without the other is refused.
5. **Replay.** The same request ID and bytes replay tag 1 and produce no second event.

## Test value

- Gate 1 turns red if the C ABI's curve or owner list differs from the core's composition, or if
  the event arrives before the plan has rendered the requested state.
- Gate 2 turns red if an acked request can lose its event, or two captures run at once.
- Gate 3 turns red if a request for a removed strip is never answered.
- Gate 4 turns red if the new event shape or its capability pairing is not enforced.
- Gate 5 turns red if a replay arms a second capture.

## Dependencies

- #1577 (the capture), #1576 (the composer), #1573 (the wire and the provider path).
- Stream B owns `crates/protocol`, `crates/control-plane` and `crates/capi`: rebase after its open
  slices that touch the controller, the queue and the capability registry.

## Supersedes and shrinks

- Shrinks #763 (see #1573's amendment record): the track response reaches the C ABI.
