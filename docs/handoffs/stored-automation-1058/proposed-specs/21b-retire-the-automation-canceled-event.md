# Retire the AUTOMATION_CANCELED event and the cancellation path

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A8 and A9, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1.

## Product outcome

After draft 21a *Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch*, no
batch can enter the automation queue, so nothing can be canceled. This slice removes the
cancellation: the event `AUTOMATION_CANCELED` (`0x8002`) and its reason enum are retired, a
revision change or a transport locate cancels nothing, and a transaction commit reserves only its
`SESSION_COMMITTED` event. No endpoint advertises `0x8002`. The retired ID is refused and never
reallocated.

This is the second of three slices that retire the command, all in batch P1 (one push). Draft 21c
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
  (`:3185`) touches it; the `typed_frame.rs` tests that 21a rewrote lose their event rows.
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
- **D5. Re-pins**, each with the reason "draft 21b retires `AUTOMATION_CANCELED`":
  `COMPLETE_SCHEMA_HASH` and its history line (the corpus loses the event row), the wasm-parity
  self-test strings, the fuzz manifest, the conformance record, and the C ABI vectors.
  `retired_code_rows` gains a row for `0x8002`.
- **D6. Docs.** The registry lists `0x8002` as retired with this slice as the reason; the
  semantics doc loses the cancellation paragraph (`:19`).
- **D7. The acked-batch question: can an ack ever precede a drop?** No. No batch can be admitted
  (draft 21a), so there is nothing to cancel, and draft 21c removes the queue.

## Deliverables

1. D1-D3 in `crates/protocol` and host-core's provider.
2. D4's tests, D5's re-pins and retired row, D6's docs.

## Authorized paths

- `crates/protocol/**` (in `queue.rs`, only the event's reliable payload and constructor)
- `crates/host-core/src/control_provider.rs` (the `record_canceled_automation` hook and its test)
- `crates/capi/src/runtime/{tests,live_tests}.rs`, or their `crates/control-plane/` successors
  after #1309
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`
- `scripts/check-protocol-wasm-parity.sh` (the self-test hash strings),
  `fuzz/corpus/complete-schema-manifest.md`
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`

## Non-goals

- The command (draft 21a). The queue, its records, config and counters (draft 21c). The transport
  position (draft 25).

## Hazards

- **Batch P1.** After this slice the queue has no producer and no consumer; draft 21c deletes it
  in the same push.
- **Corpus re-pin order** as in draft 21a.

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
6. **Commands:** as draft 21a's gate 6.

## Test value

- Gate 1 turns red if the retired event still decodes.
- Gate 2 turns red if the invariant check still demands `0x8002` beside `0x8001`.
- Gate 3 turns red if a commit or a locate still reserves or emits a cancellation, which would cost
  a reliable slot for nothing.
- Gate 4 turns red if the C ABI still raises or expects `AUTOMATION_CANCELED`.

## Dependencies

Batch P1. Direct dependency:

- Draft 21a *Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch*.

Draft 21c *Delete the protocol automation queue, its records and counters* depends on this draft.
