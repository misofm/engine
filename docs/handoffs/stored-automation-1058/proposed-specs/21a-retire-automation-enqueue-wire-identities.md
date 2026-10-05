# Retire AUTOMATION_ENQUEUE and AUTOMATION_CANCELED from the protocol wire and dispatch

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A8 and A9, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1, after batch R1.

## Product outcome

The control protocol has one way to make a sample-timed change: a stored automation edit, which
the engine renders (A1). A peer that sends `AUTOMATION_ENQUEUE` (`0x0006`) is refused at decode as
an unsupported message, before anything changes, on every endpoint. No endpoint advertises
`0x0006`, its event `AUTOMATION_CANCELED` (`0x8002`), flag bit 5 or the capability fields that
describe the queue. Statuses 15 and 16 are retired. Every retired code is refused, never
reallocated and never renumbered. A revision change or a transport locate cancels nothing, because
nothing can be queued.

This slice and draft 21b *Delete the protocol automation queue, its records and counters* are one
change split for review: they land in one push (batch P1), so `main` never holds the queue with
nothing to enqueue into it. Together they replace D1-D3 and gates 1, 2 and 4 of *Refuse commands
that would be acknowledged with no effect* (#1315): instead of a feature switch that is never set
true, the command is retired. This slice lands no later than #1315's C ABI half. #1315 keeps D4
(the browser bypass lift), D5 (docs, reworded: the refusal is permanent) and D6.

## Context

- **Why.** The owner ruled that sample-timed `AutomationEnqueue` delivery has no product consumer
  (`docs/rulings/engine-footprint-2026-09-28.md:58-59`) and that code nothing uses is removed
  (`:20-21`). Every sample-timed outcome is a stored automation edit (opcodes `0x0600`-`0x0603`,
  which stay). Today the C ABI acks a batch that reaches no PCM: the controller enqueues it
  (`crates/protocol/src/controller.rs:2961-2999`), and its only dequeue is the cancellation
  (`crates/protocol/src/queue.rs:799-806`; #1315 Context item 1;
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`).
- **The in-place amendment rule.** Before launch an owner ruling may amend v1 in place: nothing is
  renumbered, a retired code is refused and never reallocated, a retired field ID is refused in
  either flag form (`docs/CONTROL_BTLV_V1.md:61`; precedents at
  `docs/CONTROL_PROTOCOL_REGISTRY.md:83`, `:89`). The machinery exists: `schema_spec_retiring`
  (`crates/protocol/src/btlv.rs:240-248`) with a retired-field spec such as the track's
  (`crates/protocol/src/schema.rs:1000-1010`), and `retired_code_rows`
  (`crates/conformance/src/protocol_corpus.rs:359-372`), run by the controller tests
  (`crates/protocol/src/controller/tests.rs:1546`).
- **Wire identity.** `MessageId::AutomationEnqueue = 0x0006` and `AutomationCanceled = 0x8002`
  (`crates/protocol/src/wire.rs:109-110`, `:123-124`), parsed by `MessageId::from_raw`
  (`:142-165`), which returns `DecodeError::UnsupportedMessage` for an unassigned ID (`:163`).
  Status `TIME_IN_PAST = 15` and `AUTOMATION_ORDER = 16` (`:210-213`, parsed at `:238-239`).
- **The capabilities rule.** `check_capabilities_invariants`
  (`crates/protocol/src/message_wire.rs:3493-3530`) ties `0x8002` to `0x8001` (`:3496`, `:3522`),
  flag bit 5 to command `0x0006` (`:3512`), requires `maximum_automation_records == 256` (`:3503`)
  and `maximum_tlvs >= 27`, the field count (`:3501`); `allocated_id` (`:3532`) lists both IDs.
  `CapabilityFlags::{B4_BASE, KNOWN}` include bit 5 (`:57`, `:60`). `capability_registry`
  (`crates/protocol/src/controller.rs:3219-3289`) always lists `0x0006` and sets bit 5
  (`flags = (1 << 7) - 1`, `:3257`); `capabilities` fills the queue fields (`:3299-3329`, the
  automation ones at `:3309-3320`).

**Inventory for this slice** (wire, schema, dispatch, cancellation; the queue itself is 21b's):

| Item | Where |
|---|---|
| Message IDs `0x0006`, `0x8002` | `crates/protocol/src/wire.rs:109-110`, `:123-124`, `:150`, `:157` |
| Status codes 15, 16 | `wire.rs:210-213`, `:238-239`; `status_for_automation`, `controller.rs:3596-3607` |
| Command and success payloads, decoded view | `AutomationEnqueue`, `DecodedAutomationEnqueue`, `AutomationEnqueued` (`message_wire.rs:413-475`); codec `:1236-1305` and `write_automation_enqueue`/`_enqueued` (`:2397` and siblings) |
| Event payload and its reason enum | `AutomationCancellationReason`, `AutomationCanceled` (`message_wire.rs:523-554`); codec `:1475-1535`; `parse_automation_cancellation_reason` (`:1961-1970`); `write_automation_canceled` (`:2526`); `ReliablePayload::AutomationCanceled` and `ReliableSlot::automation_canceled` (`queue.rs:336-350`, `:419-444`) |
| Schemas | `schema::automation_enqueued`, `automation_enqueue`, `automation_canceled` (`crates/protocol/src/schema.rs:417-438`, `:488-507`); registry list `:1634-1640` |
| Typed frames | `typed_frame.rs:48`, `:72`, `:102`, `:124`, `:147`, `:172`, `:194`, `:212`, `:226`, `:433-435`, `:494-496`, `:533-535`, `:562`, `:663-664`, `:705-706`, `:739-740` |
| Capability fields | `maximum_automation_records` (field 9), `automation_batch_slots` (12), `per_block_automation_density` (19), `admission_quantum_frames` (20): `message_wire.rs:80-92`, `:119-130`, codec `:3218`, `:3349`; `schema.rs:684`, `:687`, `:694-695`; `controller.rs:3309-3320` |
| Flag bit 5 | `message_wire.rs:57`, `:60`, `:3512`; `controller.rs:3257` |
| Controller dispatch | `ControlCommand::AutomationEnqueue` (`controller.rs:885-902`, `:924`, `:937`), `Body::AutomationEnqueued` (`:993`), decode routes (`:1499-1504`, `:2142-2152`), encode (`:2246-2247`), execute arm (`:2961-2999`), `validate_automation_domains` (`:3358-3390`) |
| Cancellation | `cancel_pending_automation`, `cancel_queued_automation_reserved` (`controller.rs:3423-3490`); cancels on revision change (`:1877`, `:2005`, `:2931`) and on locate (`:3051`); queue-occupancy terms in event reservations and token staleness (`:1733`, `:1953-1963`, `:2881`, `:3010`); event emission (`:2667-2685`) |
| Provider hooks | `ControlProvider::parameter_descriptor` (only caller `controller.rs:3365`) and `record_canceled_automation` (`:578-589`); `MockProvider`'s `automation_parameter` and implementations (`:623-650`, `:731-744`, `:769-778`); host-core's (`crates/host-core/src/control_provider.rs:334-343`, `:369-380`, test `:923`) |
| Exports | `crates/protocol/src/lib.rs:17-38` (the payload, event and reason names) |
| Conformance corpus | `complete_schema_corpus` rows `command.automation_enqueue` (`protocol_corpus.rs:820-822`), `response.automation` (`:926-927`), statuses 15 and 16 (`:984-985`, `:1180-1181`), `event.automation_canceled` (`:1054-1059`), capability literals (`:857-875`) |
| Protocol audit, wire leg | `tools/audit/src/protocol.rs:41-42`, `:93-134`, `:194`, `:434-456` (10,000 records encoded, decoded and enqueued in 40 batches); its success line `:510`, matched by `scripts/run-protocol-allocation-audit.sh:37-39` |
| Docs | `docs/CONTROL_PROTOCOL_REGISTRY.md:9`, `:11`, `:25`, `:30`, `:41-43`; `docs/CONTROL_PROTOCOL_SEMANTICS.md:13`, `:15`, `:19`; `docs/CONTROL_PROTOCOL_SIZING.md:9`; `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`, `:7`, `:13`; `docs/CONTROL_PROVIDER_BOUNDARY.md:9`; `docs/C_ABI_V1_QUALIFICATION.md:300` |
| Fuzz | `fuzz/corpus/complete-schema-manifest.md:3-10` (frame counts and the hash); no seed file carries the command |

**Not in this slice:** the record codec, the batch, the queue, its config, its report, queue kind 2
and the counters (draft 21b). **Never retired:** `ParameterHandle`, `ParameterAutomationRate`
(`message_wire.rs:239-247`), `ControlProvider::current_sample`, `reserve_reliable_events`, the
stored automation edits `0x0600`-`0x0603`.

**Tests that enqueue, cancel or pin the command today:**

- protocol: `controller/tests.rs` `public_b1b_automation_replay_uses_the_ordinary_queue` (`:341`),
  `backpressure_is_cached_without_second_enqueue` (`:1862`),
  `decoded_invalid_automation_rolls_back_after_final_compilation` (`:2115`),
  `endpoint_owned_current_sample_rejects_past_automation_without_client_time` (`:2153`),
  `b2b_btlv_automation_uses_header_identity_typed_domains_and_backpressure` (`:2166`),
  `transport_state_only_preserves_automation_and_locate_starts_new_ordering_epoch` (`:2433`)
  exist only for the command; `full_frame_ingress_dispatches_every_registered_command_through_typed_frames`
  (`:570`), `full_frame_ingress_replays_exact_bytes_and_rejects_changed_request_reuse` (`:723`),
  `structural_token_cancel_owner_generation_and_serial_rules_are_affine` (`:1213`),
  `every_non_ok_status_has_a_canonical_common_response_payload` (`:1697`),
  `provider_feature_matrix_derives_capabilities_and_refuses_disabled_dispatch` (`:2789`),
  `c2b2_event_egress_emits_all_six_schema_closed_full_frames` (`:3185`) touch it;
  `message_wire/tests.rs` `b2b_automation_goldens_truncation_and_malformed_cases_are_functional`
  (`:943`) exists only for it, and `:250`, `:316`, `:532`, `:721`, `:1554` touch it;
  `typed_frame.rs` tests `:923`, `:1072`, `:1236`, `:1319` touch it.
- C ABI (#1315 gate 4 names the first two):
  `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
  (`crates/capi/src/runtime/tests.rs:1857`, pinned `RESPONSES` at `:1858-1873`) and
  `capi_controller_dispatches_every_advertised_command_family` (`:2250`, pinned
  `ALL_COMMAND_RESPONSE_VECTORS` at `:89`, entry 0 the capabilities response, the automation
  response among them); also `exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact`
  (`:2485`, reads the capabilities vector at `:2501`) and
  `a_live_edit_emits_the_commit_events_of_a_rebuild`
  (`crates/capi/src/runtime/live_tests.rs:1371`, queues a batch and expects
  `AUTOMATION_CANCELED`).

**Pins this slice moves:** `COMPLETE_SCHEMA_HASH` (`crates/conformance/src/protocol_corpus.rs:737`,
its re-pin history `:706-736`), its self-test strings in `scripts/check-protocol-wasm-parity.sh:172-173`,
the manifest `fuzz/corpus/complete-schema-manifest.md:10`, the record
`docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`, and the C ABI vectors above.

## Decisions frozen for this slice

- **D1. Retired identities.** `MessageId` loses `AutomationEnqueue` and `AutomationCanceled`;
  `from_raw` returns `UnsupportedMessage` for `0x0006` and `0x8002`, as for any unallocated ID, and
  a comment names them retired and never reallocated. `StatusCode::parse` refuses 15 and 16 as
  `InvalidStatus`.
- **D2. Capabilities.** Fields 9, 12, 19 and 20 leave `Capabilities`, `DecodedCapabilities` and the
  schema, and join a retired-field spec read through `schema_spec_retiring`, so a peer that sends
  one is refused in either flag form. The record has 23 fields; `maximum_tlvs >= 23` replaces
  `>= 27`. Bit 5 leaves `KNOWN` and `B4_BASE`, so a set bit 5 is an unknown flag and refused;
  `capability_registry` starts from `((1 << 7) - 1) & !(1 << 5)`.
- **D3. The session family** is `0x0003` with `0x8001`: `check_capabilities_invariants` drops
  `has_event(0x8002)` from `session_family` and the `0x8001`/`0x8002` tie; `allocated_id` drops both
  IDs; `capability_registry`'s command loop skips 6 and its event candidates drop `0x8002`.
- **D4. Nothing cancels.** A revision change and a transport locate cancel nothing; the
  cancellation path, the occupancy terms in reservations and token staleness, and the event leave.
  A transaction commit reserves only its `SESSION_COMMITTED`.
- **D5. Dispatch and hooks leave.** The command's dispatch, its domain check and the event leave;
  the provider trait loses `parameter_descriptor` and `record_canceled_automation`, and host-core
  and `MockProvider` lose their implementations. A test that exists only for the command is
  deleted; a test that touches it is rewritten without it. The audit's wire leg leaves and its
  success line drops "10,000 automation records in 40 batches".
- **D6. Re-pins**, each listed with the reason "draft 21a retires `AUTOMATION_ENQUEUE`, its event,
  statuses 15 and 16 and capability fields 9, 12, 19 and 20": `COMPLETE_SCHEMA_HASH` and its
  history line (the corpus becomes 41 frames: 10 commands, 10 success responses, 16 non-OK statuses
  and 5 events), the wasm-parity self-test strings, the fuzz manifest, the conformance record, and
  the C ABI vectors. `retired_code_rows` gains one row per retired message, status, capability
  field and flag bit.
- **D7. Docs.** The registry lists each retired code with this slice as the reason; the semantics
  doc states that sample-timed changes are stored automation edits and that the command is retired
  (it replaces #1315 D5's wording); the sizing, conformance, provider-boundary and C ABI
  qualification lines above lose the command and the event.
- **D8. The acked-batch question: can an ack ever precede a drop?** No. `0x0006` is refused at
  frame decode, before the revision check and before any state changes. Draft 21b removes the
  queue, so none remains to hold a batch.

## Deliverables

1. D1-D5 in `crates/protocol` and host-core's provider.
2. D6's corpus rows, re-pins and retired rows; D7's docs; the audit change.
3. The rewritten and deleted tests (D5) and the gates below.

## Authorized paths

- `crates/protocol/**` (not `queue.rs` beyond the event's reliable payload and constructor)
- `crates/host-core/src/control_provider.rs` (the two provider hooks and their test)
- `crates/capi/src/runtime/{tests,live_tests}.rs`, or their `crates/control-plane/` successors
  after #1309
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`
- `tools/audit/src/protocol.rs` (the wire leg and success line), `scripts/run-protocol-allocation-audit.sh`
  (the success line), `scripts/check-protocol-wasm-parity.sh` (the self-test hash strings)
- `fuzz/corpus/complete-schema-manifest.md`
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/CONTROL_PROTOCOL_SIZING.md`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
  `docs/CONTROL_PROVIDER_BOUNDARY.md`, `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The queue, its records, config, report, queue kind 2 and the counters (draft 21b).
- The C limit `maximum_automation_spans_per_block` (draft 22). The browser bypass lift (#1315 D4).
- The stored automation edits `0x0600`-`0x0603`. `TRANSPORT_SET`'s position (finding F4).

## Hazards

- **One push with 21b.** After this slice nothing enqueues; the queue is dead code until 21b. The
  two land together in batch P1, so `main` never holds it.
- **Corpus re-pin order.** #1313, #1365, #1369 and #1394 also re-pin `COMPLETE_SCHEMA_HASH`.
  Whichever merges second recomputes from the other's value and adds its own history line; never
  merge two hashes by hand.
- **#1315's C ABI half** must not land before this slice: its feature switch would be code this
  slice deletes. Dependencies order it after this slice, and root amends #1315 before either
  lands.
- **The hold rule's citation.** Note A3 cites the record rule at `CONTROL_PROTOCOL_REGISTRY.md:45`;
  draft 01 states the hold rule in the schema doc first, and draft 21b removes that line.

## Objective gates

1. **Refused at decode** (`crates/protocol/src/controller/tests.rs`, new). A complete command
   frame with message ID `0x0006` and a valid former payload is answered `UNSUPPORTED_MESSAGE` (3);
   the revision, the reliable event lane, the counters and the replay cache are unchanged. An
   event frame with `0x8002` fails typed event decode. Each retired status, capability field (in
   both flag forms) and flag bit is refused by its decoder (rows in `retired_code_rows`).
2. **Capabilities** (`crates/protocol/src/message_wire/tests.rs`, and the C ABI dispatch test). The
   record lists neither `0x0006` nor `0x8002`, clears bit 5, carries 23 fields, and passes
   `check_capabilities_invariants`; a record that adds either ID or bit 5 back is refused.
3. **C ABI** (`crates/capi/src/runtime/tests.rs`, `live_tests.rs`, rewritten). The dispatch test
   sends `0x0006` and gets `UNSUPPORTED_MESSAGE` with nothing changed; the event test becomes
   `all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes` (no path raises `0x8002`);
   `a_live_edit_emits_the_commit_events_of_a_rebuild` expects `SESSION_COMMITTED` alone. Every
   re-pinned vector is listed in the PR with D6's reason.
4. **Corpus and audits.** `cargo test --locked -p conformance` with the new
   `COMPLETE_SCHEMA_HASH`; `bash scripts/check-protocol-wasm-parity.sh`;
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `bash scripts/run-protocol-allocation-audit.sh target/release/audit` and
   `./target/release/audit capi` with zero allocations, locks and syscalls.
5. **No rendered bit moves.** `./target/release/audit capi` shows the same `pcm_digest` at base and
   head (PR evidence); nothing that renders reads the command.
6. **Commands:**
   - `cargo test --locked -p protocol --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p conformance`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-protocol-control-policy.sh`, `bash scripts/test-protocol-control-policy.sh`,
     `bash scripts/check-capi-abi.sh`, `bash scripts/check-conformance-boundaries.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke -p protocol`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a retired message, status, field or flag still decodes, if a retired
  capability field is skipped as an unknown optional field, or if the refusal changes state first
  (an ack before a drop). No test refuses `0x0006` today.
- Gate 2 turns red if an endpoint still advertises a retired ID or bit, or if the invariant check
  still demands `0x8002` beside `0x8001`.
- Gate 3 turns red if the C ABI dispatches the command, or still raises or expects
  `AUTOMATION_CANCELED`.

## Dependencies

Batch P1, after batch R1. Direct dependencies:

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule*: the
  hold rule's new home in `docs/SESSION_SCHEMA_V1.md`, which must exist before draft 21b deletes
  `docs/CONTROL_PROTOCOL_REGISTRY.md:45`.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for the C ABI
  paths.
- Ordering: this draft lands no later than the C ABI half of *Refuse commands that would be
  acknowledged with no effect* (#1315). This draft and 21b amend #1315: they replace its D1-D3 and
  gates 1, 2 and 4.

Draft 21b *Delete the protocol automation queue, its records and counters* depends on this draft
and lands in the same push. Draft 22 *Reserve the C ABI's automation span limit* follows both.
