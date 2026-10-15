# Retire the AUTOMATION_CANCELED event and the cancellation path

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A8 and A9, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1.

## Product outcome

After #1627 *Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch*, no
batch can enter the automation queue, so nothing can be canceled. This slice removes the
cancellation: the event `AUTOMATION_CANCELED` (`0x8002`) and its reason enum are retired, a
revision change or a transport locate cancels nothing, and a transaction commit reserves only its
`SESSION_COMMITTED` event. No endpoint advertises `0x8002`. The retired ID is refused and never
reallocated.

This is the second of three slices that retire the command, all in batch P1 (one push). #1629
*Delete the protocol automation queue, its records and counters* follows it.

## Context

- **Wire identity.** `MessageId::AutomationCanceled = 0x8002` (`crates/protocol/src/wire.rs:123-124`,
  parsed at `:157`).
- **The capabilities rule.** `check_capabilities_invariants`
  (`crates/protocol/src/message_wire.rs:3493-3530`) ties `0x8002` to `0x8001` (`:3496`, `:3522`);
  `allocated_id` (`:3532`) lists `0x8002`. `capability_registry`
  (`crates/protocol/src/controller.rs:3219-3289`) lists it among its event candidates.
- **The cancellation path.** `cancel_pending_automation` and `cancel_queued_automation_reserved`
  (`controller.rs:3423-3490`) cancel on a revision change (`:1877`, `:2005`, `:2931`) and on a
  locate (`:3051`). Event reservations and token staleness count queue occupancy (`:1733`,
  `:1953-1963`, `:2881`, `:3010`); the event is emitted at `:2667-2685`. The provider hook
  `ControlProvider::record_canceled_automation` (`:578-589`) has `MockProvider`'s implementation
  (`:769-778`) and host-core's (`crates/host-core/src/control_provider.rs:369-380`, test `:923`).

**Inventory for this slice:**

| Item | Where |
|---|---|
| Message ID `0x8002` | `crates/protocol/src/wire.rs:123-124`, `:157` |
| Event payload and its reason enum | `AutomationCancellationReason`, `AutomationCanceled` (`message_wire.rs:523-554`); codec `:1475-1535`; `parse_automation_cancellation_reason` (`:1961-1970`); `write_automation_canceled` (`:2526`); `ReliablePayload::AutomationCanceled` and `ReliableSlot::automation_canceled` (`queue.rs:336-350`, `:419-444`) |
| Schema | `schema::automation_canceled` (`crates/protocol/src/schema.rs:488-507`); its entry in the registry list `:1634-1640` |
| Typed frames | `typed_frame.rs:102`, `:172`, `:226`, `:533-535`, `:739-740` |
| Capabilities | `message_wire.rs:3496`, `:3522`, `:3532`; the event candidates of `capability_registry` |
| Cancellation | `controller.rs:3423-3490`, `:1877`, `:2005`, `:2931`, `:3051`; occupancy terms `:1733`, `:1953-1963`, `:2881`, `:3010`; emission `:2667-2685` |
| Provider hook | `record_canceled_automation` (`controller.rs:578-589`, `:769-778`; `crates/host-core/src/control_provider.rs:369-380`, test `:923`) |
| Exports | `crates/protocol/src/lib.rs:17-38` (the event and reason names) |
| Conformance corpus | `event.automation_canceled` (`protocol_corpus.rs:1054-1059`) |
| Docs | `docs/CONTROL_PROTOCOL_REGISTRY.md:9`, `:41-43`; `docs/CONTROL_PROTOCOL_SEMANTICS.md:19` |

**Tests that raise or expect the event:**

- protocol: `controller/tests.rs`
  `transport_state_only_preserves_automation_and_locate_starts_new_ordering_epoch` (`:2433`)
  exists only for the cancellation; `c2b2_event_egress_emits_all_six_schema_closed_full_frames`
  (`:3185`) touches it; the `typed_frame.rs` tests that #1627 rewrote lose their event rows.
- C ABI (#1315 gate 4 names the first): `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
  (`crates/capi/src/runtime/tests.rs:1857`, pinned `RESPONSES` at `:1858-1873`) and
  `a_live_edit_emits_the_commit_events_of_a_rebuild`
  (`crates/capi/src/runtime/live_tests.rs:1371`, which queues a batch and expects
  `AUTOMATION_CANCELED`).

## Decisions frozen for this slice

- **D1. Retired identity.** `MessageId` loses `AutomationCanceled`; `from_raw` returns
  `UnsupportedMessage` for `0x8002`, and a comment names it retired and never reallocated.
- **D2. The session family** is `0x0003` with `0x8001`: `check_capabilities_invariants` drops
  `has_event(0x8002)` from `session_family` and the `0x8001`/`0x8002` tie; `allocated_id` drops
  the ID; `capability_registry`'s event candidates drop it.
- **D3. Nothing cancels.** A revision change and a transport locate cancel nothing; the
  cancellation path, the occupancy terms in reservations and token staleness, and the event
  emission leave. A transaction commit reserves only its `SESSION_COMMITTED`. The provider trait
  loses `record_canceled_automation`, with both implementations.
- **D4. Tests.** A test that exists only for the cancellation is deleted. The C ABI event test
  becomes `all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes` (no path raises
  `0x8002`); `a_live_edit_emits_the_commit_events_of_a_rebuild` expects `SESSION_COMMITTED` alone.
- **D5. Re-pins**, each with the reason "#1628 retires `AUTOMATION_CANCELED`":
  `COMPLETE_SCHEMA_HASH` and its history line (the corpus loses the event row), the wasm-parity
  self-test strings, the fuzz manifest, the conformance record, and the C ABI vectors.
  `retired_code_rows` gains a row for `0x8002`.
- **D6. Docs.** The registry lists `0x8002` as retired with this slice as the reason; the
  semantics doc loses the cancellation paragraph (`:19`).
- **D7. The acked-batch question: can an ack ever precede a drop?** No. No batch can be admitted
  (#1627), so there is nothing to cancel, and #1629 removes the queue.

## Deliverables

1. D1-D3 in `crates/protocol` and host-core's provider.
2. D4's tests, D5's re-pins and retired row, D6's docs.

## Authorized paths

- `crates/protocol/**` (in `queue.rs`, only the event's reliable payload and constructor)
- `crates/host-core/src/control_provider.rs` (the `record_canceled_automation` hook and its test)
- `crates/capi/src/runtime/{tests,live_tests}.rs` (#1309 D8 keeps the C ABI tests in capi)
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`
- `scripts/check-protocol-wasm-parity.sh` (the self-test hash strings),
  `fuzz/corpus/complete-schema-manifest.md`
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`

## Non-goals

- The command (#1627). The queue, its records, config and counters (#1629). The transport
  position (draft 25).

## Hazards

- **Batch P1.** After this slice the queue has no producer and no consumer; #1629 deletes it
  in the same push.
- **Corpus re-pin order** as in #1627.

## Objective gates

1. **Refused** (`crates/protocol/src/controller/tests.rs`, through `retired_code_rows`). An event
   frame with `0x8002` fails typed event decode.
2. **Capabilities** (`crates/protocol/src/message_wire/tests.rs`). The record does not list
   `0x8002` and passes `check_capabilities_invariants`; a record that adds it back is refused.
3. **Nothing cancels** (`crates/protocol/src/controller/tests.rs`, new). A transaction that changes
   the revision and a `TRANSPORT_SET` reserve and emit exactly the events they emit without an
   automation family: `SESSION_COMMITTED` and `TRANSPORT_STATE`, and no other.
4. **C ABI** (`crates/capi/src/runtime/tests.rs`, `live_tests.rs`, rewritten): D4's two tests.
   Every re-pinned vector is listed in the PR with D5's reason.
5. **Corpus and audits.** `cargo test --locked -p conformance` with the new
   `COMPLETE_SCHEMA_HASH`; `bash scripts/check-protocol-wasm-parity.sh`;
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `bash scripts/run-protocol-allocation-audit.sh target/release/audit` and
   `./target/release/audit capi` with zero allocations, locks and syscalls, and the same
   `pcm_digest` as base (PR evidence).
6. **Commands:** as #1627's gate 6, which includes `bash scripts/check-cross-targets.sh`
   (README F19, the iOS memset rule: no new `memset_pattern16` call; fix one in code, never by a
   ceiling).

## Test value

- Gate 1 turns red if the retired event still decodes.
- Gate 2 turns red if the invariant check still demands `0x8002` beside `0x8001`.
- Gate 3 turns red if a commit or a locate still reserves or emits a cancellation, which would cost
  a reliable slot for nothing.
- Gate 4 turns red if the C ABI still raises or expects `AUTOMATION_CANCELED`.

## Dependencies

Batch P1. Direct dependency:

- #1627 *Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch*.

#1629 *Delete the protocol automation queue, its records and counters* depends on this draft.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **The event test** is already `all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes`
  (`crates/capi/src/runtime/tests.rs:2194`, renamed by #1315). D4's rename is done; its doc comment
  (`:2187-2191`, "the sixth family returns with stored automation") is rewritten: the family is
  retired (README F24).
- **`a_live_edit_emits_the_commit_events_of_a_rebuild`** is `crates/capi/src/runtime/live_tests.rs:2324`;
  it still decodes `AUTOMATION_CANCELED` (`:2308`).
- **Anchors** in the inventory have moved; re-read at filing. Batch P1, ready.

## Decision record: #1628 retire AUTOMATION_CANCELED (batch 10)

**What shipped.** The event `AUTOMATION_CANCELED` (`0x8002`) and the whole cancellation path are
gone from `protocol`, host-core and the conformance corpus; nothing else changed on the wire.

- `MessageId::AutomationCanceled`, `AutomationCancellationReason`, `AutomationCanceled`, its schema,
  codec (`encode`/`decode`/`write`, `parse_automation_cancellation_reason`), the `EventPayload` and
  `DecodedEventPayload` arms, `ReliablePayload::AutomationCanceled`, `ReliableSlot::automation_canceled`
  and the controller's event encoding are deleted. `0x8002` joins `RETIRED_MESSAGE_IDS`
  (`[0x0004, 0x0005, 0x0006, 0x8002]`), so `from_raw` and `allocated_id` refuse it with no new code.
- `cancel_pending_automation`, `cancel_queued_automation_reserved`,
  `ProtocolQueues::reset_automation_ordering_after_cancellation` and the provider hook
  `record_canceled_automation` (trait default, `MockProvider`, host-core) are deleted. A transaction
  commit (planned, prepared and legacy paths) reserves only its `SESSION_COMMITTED` (`required_events`
  is 0 for a monitor-only transaction, else 1); a `TRANSPORT_SET` reserves only its `TRANSPORT_STATE`.
  `PreparedStructuralCommand` and `StructuralCommandPlan` lose `cancellation_batches`, and a token no
  longer goes stale when the automation queue's occupancy changes.
- `check_capabilities_invariants` drops `has_event(0x8002)` from the session family and the
  `0x8001`/`0x8002` tie; `allocated_id` refuses `0x8002`; `capability_registry` lists five events.
- Docs: registry (table, retired paragraph, event payload sentence), semantics (cancellation
  paragraph), conformance manifest, `C_ABI_V1_QUALIFICATION.md` ("Same events").

**Tests.** Deleted (existed only for the cancellation): `transport_state_only_preserves_automation_and_locate_starts_new_ordering_epoch`,
the `cancel_pending_automation` assertion of `structural_token_cancel_owner_generation_and_serial_rules_are_affine`,
the cancellation codec block of `b4_event_payloads_are_typed_canonical_and_truncation_safe`. New:
`a_revision_change_and_a_locate_reserve_and_emit_only_their_own_events` (gate 3: with a batch queued and
a two-slot reliable event queue, a revision change emits `SESSION_COMMITTED` alone and a locate
`TRANSPORT_STATE` alone) and `an_event_frame_under_the_retired_cancellation_id_is_refused` (gate 1,
event side: a valid event frame with its ID patched to `0x8002` fails typed decode with
`UnsupportedMessage`). Rewritten: `c2b2_event_egress_emits_all_five_schema_closed_full_frames`,
`retired_message_ids_are_never_reused_or_advertised` (4 IDs; `0x8002` not an allocated event),
`capability_id_views_reject_order_allocation_and_session_family_identically` (a record that adds
`0x8002` back is refused: gate 2), the `[0x8001]` event lists of the capabilities tests,
`retired_and_console_refused_codes_meet_their_conformance_rows` (`retired_code_rows` gains
`retired.message_id_0x8002`, a command frame under the retired event ID: 16 + 4 + 7 + 13 + 1 + 4 + 1),
C ABI `all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes` (doc comment: the family is
retired, README F24; the capabilities response vector now lists five events) and
`a_live_edit_emits_the_commit_events_of_a_rebuild` (no `AUTOMATION_CANCELED` arm).

**Re-pins** (reason: draft 21b retires `AUTOMATION_CANCELED`): `COMPLETE_SCHEMA_HASH`
`0xcfa5_8ab7_b7bb_b7c4` to `0x4b21_d9e6_f520_afd9` (corpus 46 to 45 frames: the
`event.automation_canceled` row is gone and the capabilities response lists no `8002`), the parity
self-test strings (digest, and the length-assertion mutation 45 to 44), the fuzz manifest, the
conformance manifest, `main.rs`/`conformance_corpus.rs`/`mutation_million.rs` frame counts, and C ABI
vectors: the capabilities response (events TLV `0c` bytes to `0a`, five IDs, same padded length) in
the dispatch test and in the five-family event test. No other C ABI vector moved.

**Gates.** `cargo test --locked` for protocol (`--features test-support`), control-plane
(`--features test-support`), host-core (`--all-features`), capi, conformance, host-web
(`--features host-web/test-support`): all green. Wasm parity and `--self-test`, both vocabulary
scripts, `check-protocol-control-policy.sh`, `run-protocol-allocation-audit.sh`, fuzz `cargo check`,
clippy `--workspace --all-targets --all-features -D warnings`, `cargo fmt --check`,
`check-sdk-generated.sh` and `check-cross-targets.sh` (no new `memset_pattern16`): pass. `audit capi`:
0 allocations, locks and syscalls; `pcm_digest` `76525e7ef9e78cf4`, equal to base.

**Departures.**

- The counters `CANCELED_AUTOMATION` (12), `AUTOMATION_TIME_PAST` (13) and `AUTOMATION_ORDER_REJECT`
  (14) stay in `CounterId`, `parse_counter_id` and host-core's served set: #1629's spec retires
  counters 2 and 11-14 with a gate of its own. After this slice counter 12 always reads 0 (host-core's
  test asserts that); #1629 removes it. #1627's record listed them under #1628, but this spec's
  inventory and #1629's spec say otherwise, and a counter retirement needs the `retired_code_rows`
  rows that #1629 owns.
- `retired_code_rows()` rows are command frames, so the `0x8002` row is a command-header frame under
  the retired event ID (refused `UnsupportedMessage`); the event-side refusal is the separate
  `an_event_frame_under_the_retired_cancellation_id_is_refused` test.
- Mechanical edits outside the authorized paths: `crates/conformance/src/main.rs` and
  `tests/mutation_million.rs` (frame count), `docs/C_ABI_V1_QUALIFICATION.md` (one stale sentence),
  `crates/protocol/src/typed_frame.rs` fixture rows.

**Left for #1629.** `ProtocolQueues::try_dequeue_automation` now has no production caller (its doc
says so); the gate-3 test enqueues a batch to prove nothing cancels it and loses that line when the
queue goes; `tools/realtime-policy/src/policy.rs:63` still names `try_dequeue_automation`;
`queue.rs:202` ("identity for cancellation") describes the batch slot.
