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
of drafts 05 and 06a later. Every retired field is refused, never reallocated
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
  draft 21b.
- **Retirement machinery** as in draft 21a: `schema_spec_retiring`
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
- **D3. Re-pins**, each with the reason "draft 25 retires the stored transport position":
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
- The timeline and the session seek (drafts 04a-06b). A protocol command that seeks.

## Hazards

- **Corpus re-pin order** as in draft 21a.

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
4. **Commands:** as draft 21a's gate 6.

## Test value

- Gate 1 turns red if a peer can still store a position, or if the refusal changes state first.
- Gate 2 turns red if removing the position breaks the state round trip.

## Dependencies

Batch P1. Direct dependency:

- Draft 21b *Retire the AUTOMATION_CANCELED event and the cancellation path*: it removes the locate
  cancellation, the last reader of a set position.
