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

## Decision and evidence record

**Split (scope note, root, 2026-10-24).** #1578a (protocol, carries the schema-hash re-pin): D1, D2
and the controller half of D3/D4. #1578b (serving): D3's provider path, D4's delivery in
`SessionState::service`, D5, D6 and `crates/capi/tests/response_strip.rs`. Root decisions R1-R5
of the scope note stand (R1 release on an empty cell, R2 the controller's check only for a strip
target and after the replay cache, R3 the bound at 127-byte identities, R4 the two capi tests,
R5 the queue after #1586b).

**#1578a, attempt 1 (implementer, 2026-10-25).**

- D1: `MessageId::ResponseReady = 0x8040`; `allocated_id` lists it; the registry lists it exactly
  when `features.response` is set; the invariant adds `has_command(0x000e) == has_event(0x8040)`.
- D2: `ResponseReady { sequence, request_tag, strip_id, outcome }`, with `ResponseReadyOutcome::
  Ready(TrackResponseCapture)` or `Refused { status, diagnostic }`, so a refusal with curves cannot
  be built and the encoder checks the rest. Implementer reading of "the codec refuses every other
  shape": a stable strip ID; a refusal is one of `NOT_FOUND`, `UNAVAILABLE`, `LIMIT_EXCEEDED`,
  `INTERNAL` with a 1-127-byte diagnostic and no capture field or member; a capture has at least
  two finite axis points, a finite floor, one or two finite sides of the axis's length; a member's
  slot ID is present exactly for console and inserts, its effect is 1-127 bytes, `section_count`
  is at most 32 and neither mask sets a bit at or above it. `response_ready_encoded_bound` is the
  larger of the ready form and a refusal with a 127-byte diagnostic.
- D3 (controller half): a strip `RESPONSE_GET` while the cell is held is `BACKPRESSURE`
  `response.capture.pending` before the provider. Implementer decision I1: the mandatory
  backpressure record names a new queue kind, `BackpressureQueueKind::ResponseCapture = 8`
  (capacity 1, occupancy 1, no generation), on #1944's precedent for the spectrum arm slot, and
  every `RESPONSE_GET` `BACKPRESSURE` (the provider's `Pending` in #1578b too) carries it. Kind 9
  is now the first refused code. Root to confirm.
- D4 (controller half): one `response_ready: Option<ResponseReady>` cell. `enqueue_response_ready_
  event(revision, event)` writes the next reliable sequence into the event, sizes the frame, queues
  `ReliablePayload::ResponseReady { event_sequence }` and moves the event into the cell; on any
  refusal (structural transaction outstanding or queue full, cell held, not served, unencodable)
  it returns `ResponseReadyEnqueueError { reason, event }` with the event unchanged and no
  sequence used. The cell is emptied only after a successful encode in
  `dequeue_reliable_event_frame_into`; `response_ready_pending()` is R1's query.
- Acked-batch question: the event's ack is the `RESPONSE_GET` tag reply (#1578b). No path of
  #1578a drops an event: every refusal hands it back, a short output keeps it, and only a
  delivered frame empties the cell.
- Advertisement: from this slice the C ABI lists `0x8040` (it lists `0x000e`), while its provider
  still refuses a strip target with `response.target.unavailable` until #1578b, so no tag is
  issued and no event is owed.
- **Re-pin.** `COMPLETE_SCHEMA_HASH` `f4c2fe333739f44d` -> `cb8db815ed0103a0`, 56 -> 59 frames:
  `command.response_get_strip`, `event.response_ready`, `event.response_ready_refused`, and
  `response.capabilities` lists `0x8040`. Check: with the capabilities frame's event field patched
  back (length 14 -> 12, `0x8040` -> padding) and the three new frames filtered out, the 56 frames
  hash to `f4c2fe333739f44d` (scratch example, not committed).
- Files outside the spec's list, each forced by the wire change: `crates/capi/src/runtime/tests.rs`
  (two pinned capability vectors gain `0x8040`), `crates/protocol/tests/sdk_message_fixture.rs` and
  `sdk/test/fixtures/protocol-messages.json` (the full capabilities fixture lists `0x8040`, which
  the invariant now requires; re-blessed), `crates/conformance/{src/main.rs,tests/mutation_million.rs}`,
  `scripts/check-protocol-wasm-parity.sh` and `docs/CONTROL_PROTOCOL_CONFORMANCE.md` (the re-pin
  set). `crates/control-plane/src/control.rs` maps the two new `EventEgressError` variants.
- Tests and test value (each red first on a recorded mutant):
  - `response_ready_round_trips_both_forms_and_refuses_every_other_shape` (message_wire): red if a
    field or member is lost in the round trip or one D2 presence rule is not enforced in either
    direction. Mutants: a refusal that admits a capture sample; no mask check; a builtins member
    that admits a slot ID. All red.
  - `response_ready_encoded_bound_is_the_largest_frame_exactly` (message_wire): red if the bound
    drifts from the encoder. Mutants: five member scalars; the refused form ignored. Both red.
  - `a_response_ready_event_is_held_until_delivered_and_holds_off_a_strip_request` (controller):
    red if a refused enqueue loses or alters the event, the cell empties before delivery or never,
    a second event overwrites a held one, or a strip request reaches the provider while an event is
    unread. Mutants: no strip check; the queue-full refusal keeps the written sequence; no clear on
    delivery; a clear on a failed encode; no occupied check. All red.
  - Rewritten `a_capability_frame_must_tie_bit_14_to_the_response_command_and_event` (controller):
    red if the registry stops listing `0x8040` with `0x000e` or the invariant stops tying the
    flag, command and event. Mutants: no pairing clause; the registry never lists `0x8040`. Both
    red.
  - `a_backpressure_payload_naming_the_retired_automation_queue_is_refused` (one row): red if kind 8
    stops decoding or decodes with a generation. Mutant: kind 8 given a generation. Red.

**Root, 2026-10-25.** I1 accepted. #1578b lands in the same PR as #1578a, because #1578a alone
advertises `0x8040` on the C ABI before it can send one. Acked-batch lens for the delivery: an
issued tag ends in exactly one `RESPONSE_READY`, across a rebuild, a supersession and a dispose.

**#1578b, attempt 1 (implementer, 2026-10-25).** No hash re-pin.

- D3: `SessionControlProvider::response_get` serves target 3 before the shared preflight (which
  still refuses it, `response.target.unavailable`, for the session-less entry): the core's checks
  (`response.points`, `response.grid`), `response.rate.session`, the strip (`NOT_FOUND`
  `control.address.unknown`), the event bound at the strip's owner count against the smaller of
  the reply bound and field 18 (`LIMIT_EXCEEDED` `response.reply_too_large`), then `arm`, with
  `Pending` as `BACKPRESSURE` `response.capture.pending` (the controller attaches the kind-8
  record). The tag is answered after the job is in the render queue.
- D4: `SessionState.response_ready` holds `PendingResponse::Event` while the controller cannot
  take it; `service` releases the capture once the controller's cell and this field are both
  empty (R1), polls, converts and queues. A full queue, a held cell or an outstanding structural
  transaction keeps the event. Implementer decision I2: an event the endpoint cannot encode (a
  rebuild grew the strip past the frame limit after the bound check) becomes its refusal
  (`LIMIT_EXCEEDED` `response.reply_too_large`, or `INTERNAL` `response.capture` for another encode
  failure) and is queued in its place, so the tag is still answered and the capture is released.
  The #1577 test seam keeps its own variants (`SeamArmed`, `SeamResult`) under `test-support`: a
  seam-armed capture issued no protocol tag and owes no event.
- D5: `host_core::response_ready_event` moves every buffer. `response_capture_ceiling` adds the
  event's member table, which is allocated while the result's is still held.
- Dispose: a session destroyed with a capture in flight owes nothing; the last test ends that way.
- Files outside the spec's list: `crates/host-core/src/lib.rs` (the export of
  `response_ready_event`) and `crates/capi/tests/response_get.rs` (its strip-target refusal row is
  superseded and deleted).
- Tests, each red first on a recorded mutant:
  - `crates/capi/tests/response_strip.rs::end_to_end_then_removed_strip`: red if the C ABI's curve
    or owners differ from the core's composition, if the event arrives before a render, if a strip a
    superseding removal took is never answered, or if a rebuild leaves a tag without its event or
    with two. Mutants: the event's masks swapped (red only once the insert's masks differ by side);
    `StripRemoved` mapped to `INTERNAL`; no release; the strip target still refused. All red.
  - `one_in_flight_none_lost_and_replay`: red if a full queue drops an acked request's event, two
    captures run at once, a replay arms, or a delivered capture is never released. Mutants: a held
    event dropped on a refused enqueue; no release. Both red.
  - `a_capture_grown_past_the_frame_limit_is_answered_by_its_refusal`: red if an unencodable event
    is held for ever or dropped. Mutant: no refusal replacement. Red.
  - `host_core::control_provider::tests::a_strip_request_is_checked_before_it_arms`: red if a
    refused strip request arms, the bound is not checked before arming, or a refusal has the wrong
    status or code. Mutants: no bound check; unknown strip as `UNAVAILABLE`. Both red.
  - Not mutated: gate 5's replay is the controller's replay cache, which answers before `execute`;
    the test shows that the replayed frame returns tag 1, that no event follows, and that the next
    tag is 2.

**Verifier PASS, folds (2026-10-25).**

- M1: `response_strip.rs`'s `drain` returns at most `RELIABLE_EVENT_SLOTS + 2` frames and then
  panics, so a double-delivery defect fails at once instead of growing without bound. Mutant: a
  held event that is never taken out after it is queued. All three tests red, in 0.18 s.
- M2: `a_capture_at_the_bound_stays_within_its_charge` (control-plane) also arms the bound capture
  through a strip `RESPONSE_GET` and delivers it with `service`. The event held in the
  controller's cell left 49,444 bytes live, of the 50,552-byte `result_bytes` charge. Test value:
  red if the event path holds more than the result's charge. Mutant: a delivery that keeps the
  result beside its event. Red. The conversion's own member table is not separately observable at
  launch identities: the charge's 127-byte identities leave more slack than the table takes.
  Measured `capi_retained_bytes`: 335,822 (x86-64, merged tree), under the unchanged 344,192
  ceiling.
- N1: the protocol test sets an odd structural generation; the event comes back unchanged and the
  cell stays empty. Mutant: no structural check. Red.
- N2: the header names `LIMIT_EXCEEDED` `response.reply_too_large` after a rebuild among the event
  refusals. `check-capi-abi.sh` passes.
