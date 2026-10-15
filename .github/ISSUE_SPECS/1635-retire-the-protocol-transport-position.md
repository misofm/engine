# Retire the protocol's stored transport position

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), README finding F4 as decided, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1.

## Product outcome

The control protocol no longer stores a playhead that nothing plays. `TRANSPORT_SET` takes only the
transport state; its position field is retired and refused in either flag form. The transport
snapshot and the `TRANSPORT_STATE` event carry the state and the effective sample, with no position.
The engine has one playhead: the host moves it with source seeks today and with the session seek
of #1642 and #1643 later. Every retired field is refused, never reallocated
and never renumbered.

## Context

- **The stored position.** `TRANSPORT_SET` (`0x0008`) takes state `1:U8 R` and an optional
  absolute position `2:U64 O` (`docs/CONTROL_PROTOCOL_REGISTRY.md:32`;
  `crates/protocol/src/schema.rs:448-456`). The snapshot carries state, position and effective
  sample (`schema.rs:458-467`), and so does the `TRANSPORT_STATE` event, with sequence and origin
  (`schema.rs:469-486`). The typed forms are `TransportSetRequest`, `TransportSnapshot` and
  `TransportStateEvent` (`crates/protocol/src/message_wire.rs:488-521`).
- **Nothing reads it.** The host-core provider stores the position and echoes it
  (`crates/host-core/src/control_provider.rs:424-438`, field `:59`, seed `:161`), as the protocol's
  `MockProvider` does (`crates/protocol/src/controller.rs:638-639`, `:848-860`). No render path and
  no seek reads it; the boundary doc says seek execution stays outside the provider
  (`docs/CONTROL_PROVIDER_BOUNDARY.md:5`). *Refuse commands that would be acknowledged with no
  effect* (#1315) kept `TRANSPORT_SET` because its readback reaches the stored state.
- **Why it goes.** A1.2 of the #1058 note gives the engine a timeline that sources and automation
  follow. A second, stored position that a peer can set and read back would contradict it, and it
  cannot move the timeline: a seek needs the host to refill every ring at a new generation, which
  only the host can do. The owner ruled that code nothing uses is removed
  (`docs/rulings/engine-footprint-2026-09-28.md:20-21`).
- **The locate cancellation** that read the position (`controller.rs:3050-3056`) is gone after
  #1628.
- **Retirement machinery** as in #1627: `schema_spec_retiring`
  (`crates/protocol/src/btlv.rs:240-248`) and `retired_code_rows`
  (`crates/conformance/src/protocol_corpus.rs:359-372`).
- **Corpus rows:** `command.transport_set` (`protocol_corpus.rs:827`), `response.transport_get`
  (`:935`), `response.transport_set` (`:943`), `event.transport_state` (`:1065`).
- **Tests that set a position:** `crates/protocol/src/controller/tests.rs:2421`,
  `crates/host-core/src/control_provider.rs:910`, `crates/capi/src/runtime/tests.rs:1900`, `:2403`,
  `:2557`.
- **The audit corpus** builds `TransportSetRequest { state, position: None }`
  (`tools/audit/src/protocol.rs:206`), so D2 changes that literal too.
- **The reliable event slot.** The `TRANSPORT_STATE` event's queued payload carries the position:
  `ReliablePayload::TransportState { event_sequence, state, position, effective_sample,
  origin_request_id }` (`crates/protocol/src/queue.rs:323-335`) and its constructor
  `ReliableSlot::transport_state(revision, event_sequence, state, position, effective_sample,
  origin_request_id)` (`:395-417`). `crates/protocol/src/controller.rs` builds it (`:3040`) and
  reads it (`:2688`, `:3398`); `crates/protocol/src/controller/tests.rs:3206` and `:3416` call the
  constructor.
- **Every user.** `git grep` on `423b9d4a1` (code equal to `6ee64f484`) for `TransportSetRequest`,
  `TransportSnapshot`, `TransportStateEvent`, `ReliablePayload::TransportState`,
  `transport_state(`, `transport_position`, `TRANSPORT_SET` and `TRANSPORT_STATE` (outside
  `docs/handoffs/` and `.github/`) finds them only in `crates/protocol/`, `crates/host-core/src/control_provider.rs`,
  `crates/capi/src/runtime/tests.rs`, `crates/conformance/src/protocol_corpus.rs` and
  `tools/audit/src/protocol.rs`; the fuzz manifest names the position in prose
  (`fuzz/corpus/complete-schema-manifest.md:7`).

## Decisions frozen for this slice

- **D1. Retired fields.** `TRANSPORT_SET` field 2, snapshot field 2 and event field 3 join
  retired-field specs read through `schema_spec_retiring`, so a peer that sends one is refused in
  either flag form. The remaining field IDs keep their numbers.
- **D2. Types.** `TransportSetRequest`, `TransportSnapshot` and `TransportStateEvent` lose
  `position`; `ReliablePayload::TransportState` and `ReliableSlot::transport_state` lose it too
  (`crates/protocol/src/queue.rs:323-335`, `:395-417`), and the controller stops passing
  `snapshot.position`; the provider trait's `transport_get` and `transport_set` keep their shape without it;
  host-core's provider and `MockProvider` lose `transport_position`.
- **D3. Re-pins**, each with the reason "#1635 retires the stored transport position":
  `COMPLETE_SCHEMA_HASH` and its history line, the wasm-parity self-test strings, the fuzz manifest,
  the conformance record and the C ABI vectors that carry a transport frame. `retired_code_rows`
  gains one row per retired field.
- **D4. Docs.** The registry rows for `0008`, the snapshot and the event lose the position and list
  the retired fields with this slice as the reason. `docs/CONTROL_PROVIDER_BOUNDARY.md:5` says
  transport is an absolute state and effective sample, and that the playhead is moved by host
  seeks, not by the transport.
- **D5. The acked-batch question: can an ack ever precede a drop?** No. A set with a position is
  refused at decode, before any change.

## Deliverables

1. D1 and D2 in `crates/protocol` and host-core's provider.
2. D3's re-pins and retired rows, D4's docs, the rewritten tests.

## Authorized paths

- `crates/protocol/**`; in `crates/protocol/src/queue.rs` only the `TransportState` reliable
  payload and its `transport_state` constructor (`:323-335`, `:395-417`)
- `crates/host-core/src/control_provider.rs`
- `crates/capi/src/runtime/tests.rs` (#1309 D8 keeps the C ABI tests in capi)
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`
- `tools/audit/src/protocol.rs` (the `TransportSetRequest` literal only)
- `scripts/check-protocol-wasm-parity.sh` (the self-test hash strings),
  `fuzz/corpus/complete-schema-manifest.md`
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROVIDER_BOUNDARY.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`

## Non-goals

- The transport state (stopped or playing), which stays endpoint-local and readable.
- The timeline and the session seek (#1640-#1644). A protocol command that seeks.

## Hazards

- **Corpus re-pin order** as in #1627.

## Objective gates

1. **Refused** (`crates/protocol/src/controller/tests.rs`, through `retired_code_rows`). A
   `TRANSPORT_SET` with field 2, in either flag form, is refused as an invalid field, and the
   transport state, the revision and the reliable lane are unchanged. A snapshot or event frame
   that carries the retired field fails typed decode.
2. **State still works** (same file). `TRANSPORT_SET` with state playing returns a snapshot with
   that state and the effective sample, and emits one `TRANSPORT_STATE` event with the same two
   values.
3. **Corpus and audits.** `cargo test --locked -p conformance` with the new
   `COMPLETE_SCHEMA_HASH`; `bash scripts/check-protocol-wasm-parity.sh`;
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `bash scripts/run-protocol-allocation-audit.sh target/release/audit` and
   `./target/release/audit capi` with zero allocations, locks and syscalls, and the same
   `pcm_digest` as base (PR evidence).
4. **Commands:** as #1627's gate 6, which includes `bash scripts/check-cross-targets.sh`
   (README F19, the iOS memset rule: no new `memset_pattern16` call; fix one in code, never by a
   ceiling).

## Test value

- Gate 1 turns red if a peer can still store a position, or if the refusal changes state first.
- Gate 2 turns red if removing the position breaks the state round trip.

## Dependencies

Batch P1. Direct dependency:

- #1628 *Retire the AUTOMATION_CANCELED event and the cancellation path*: it removes the locate
  cancellation, the last reader of a set position.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** The stored position is `transport_position` (`crates/host-core/src/control_provider.rs:339`,
  seed `:405`, set `:1186-1187`, snapshot `:1179`). #1315 has landed and kept `TRANSPORT_SET`.
  Decision 17 does not touch the transport. Batch P1, ready.

## Decision record: #1635 (batch 13b)

**Shipped.** `TRANSPORT_SET` field 2, `TransportSnapshot` field 2 and `TransportStateEvent` field 3
(the stored position) are retired. Each message has a `RETIRED` spec in `crates/protocol/src/schema.rs`
(`transport_set`, `transport_snapshot`, `transport_state_event`) and its decoder reads through
`schema_spec_retiring`, so a retired field is refused in either flag form: optional as
`InvalidTlv` (`MALFORMED_FRAME`), mandatory as `UnknownRequiredField`
(`UNKNOWN_REQUIRED_FIELD`). The remaining field IDs keep their numbers (snapshot 1 and 3; event 1, 2,
4 and 5). `TransportSetRequest`, `TransportSnapshot`, `TransportStateEvent`,
`ReliablePayload::TransportState` and `ReliableSlot::transport_state` lose `position`; the encoded
lengths are 16 (set), 48 and 64 (event without and with origin; were 64 and 80). The provider trait
keeps its shape; host-core's provider and `MockProvider` lose `transport_position`. D5: a set with a
position is refused at decode, before any change and before any event reservation, so no ack
precedes a drop.

**Re-pins** (reason "#1635 retires the stored transport position"): `COMPLETE_SCHEMA_HASH`
`0xcc9a_7bcc_502c_4936` to `0x3316_54b3_24d0_76e8` with its history line in
`crates/conformance/src/protocol_corpus.rs`; the two parity strings in
`scripts/check-protocol-wasm-parity.sh`; `fuzz/corpus/complete-schema-manifest.md` (hash, history
line, and the prose that named an optional transport position); `docs/CONTROL_PROTOCOL_CONFORMANCE.md`;
the C ABI vectors with a transport frame in `crates/capi/src/runtime/tests.rs` (the two snapshot
vectors, `TRANSPORT_20`/`TRANSPORT_21`, the `RESPONSES` snapshot, and the three event vectors),
re-derived mechanically by deleting the position field and fixing the payload length and field count.
The corpus frame count stays 46.

**Departures.**

- D3 says `retired_code_rows` gains one row per retired field. A `RetiredCodeRow` is a command frame
  an endpoint answers, so only `TRANSPORT_SET` field 2 can be a row. It has two rows, one per flag
  form (`retired.transport_set_field_2.mandatory`, `.optional`). The snapshot and event fields are
  refused by `b3a_transport_goldens_truncation_and_direct_codec_are_strict` in both flag forms.
- Gate 1's status: the refusals answer `UNKNOWN_REQUIRED_FIELD` (mandatory) and `MALFORMED_FRAME`
  (optional), the statuses the retiring machinery maps to for the track fields, not `INVALID_FIELD`.
- Corpus `counter_command_frame` now gives `TRANSPORT_SET` an exact revision 7 (a set requires one).
- Anchors had moved (queue.rs `:61`/`:138`, host-core `:337`); followed the code. Mechanical edits
  outside the listed paths: `hosts/host-web/tests/protocol_cross_host.rs` (the `TransportSetRequest`
  literal and the unused `SampleTime` import). The SDK mirrors no transport frame; no SDK input changed.

**Gates.**

1. Refused: `a_transport_set_with_the_retired_position_changes_nothing` (both flag forms: status, revision,
   session, reliable lane and `TRANSPORT_GET` state unchanged) and
   `retired_and_console_refused_codes_meet_their_conformance_rows` (rows 16+4+7+13+1+4+10+2+1).
   Decoder: `b3a_transport_goldens_truncation_and_direct_codec_are_strict`.
2. State still works: `a_transport_set_answers_the_state_and_queues_one_event_with_it`,
   `b3a_transport_btlv_is_typed_idempotent_and_emits_reliable_state` (event golden re-pinned, 64 bytes).
3. `cargo test --locked -p protocol` (163), `-p conformance` (`frozen_corpus_bytes_and_typed_decoders_are_unchanged`),
   `-p host-core --lib control_provider`, `-p capi --lib runtime` (114), all green;
   `bash scripts/check-protocol-wasm-parity.sh` ok (simd128); `scripts/run-protocol-allocation-audit.sh`
   ok; `audit capi`: zero allocations, locks and syscalls, `pcm_digest` `bf7dcedd1d92621f`, equal to base.
4. `scripts/check-cross-targets.sh` was not run here (coordinator's batch gate); the change adds no
   memset-style code.

**Successor.** `docs/CONTROL_PROTOCOL_SEMANTICS.md:23` still says "locating transport cancels
nothing"; reword when that file is next in scope (outside this slice's authorized paths).
