# Address enqueued automation by control address

Slice S12b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point
C10: "An `AUTOMATION_ENQUEUE` batch carries a header table of addresses; its records index that
table. No handle crosses a revision."). Code anchors verified on `main` `df585e503`. Feature
sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt
verifier or mutation runs.

## Product outcome

An `AUTOMATION_ENQUEUE` batch names its targets by control address, the same address `SET_CONTROL`
and stored automation use, in one table at the head of the batch. No list handle crosses the wire,
so a client never needs a handle to automate, and a batch can never name a handle of an old
structure. Each record is checked against its address's row. Rendering enqueued automation stays
out of scope; the control plane keeps refusing the command until automation renders (#1315).

## Context

- **The wire today.** `AutomationEnqueue { records: &[AutomationRecord] }`
  (`crates/protocol/src/message_wire.rs:415`); each record is 32 fixed bytes with a
  `handle: ParameterHandle` at bytes 4..8 (`AutomationRecord`, `crates/protocol/src/queue.rs:58`,
  decode at `:85`). A batch holds 1 to 256 records (`AUTOMATION_BATCH_RECORDS`), copied into a
  preallocated `AutomationBatchSlot` (`queue.rs:199-207`).
- **The check today.** `validate_automation_domains` (`crates/protocol/src/controller.rs:3358`)
  reads each record's handle through the provider's descriptor. #1552 D5 serves it from the
  control catalog by handle (`control_row(handle)`) and refuses a non-effect row with
  `automation.target.unsupported` until this slice.
- **Cancellation.** A structural commit cancels queued batches with `AUTOMATION_CANCELED`, reason
  `RevisionChanged` (`message_wire.rs:526-537`).
- **The command is not served.** *Refuse commands that would be acknowledged with no effect*
  (#1315 D1-D3) makes the control plane answer `AUTOMATION_ENQUEUE` with `Unavailable` while no
  render drain exists. The protocol controller still decodes and validates it when its
  `automation` feature is on, which its own tests use.

## Decisions frozen for this slice

- **D1. The address table.** `AutomationEnqueue` gains a first field, `addresses`: a repeated
  `MESSAGE` of #1551 D3's `control_address` (with `control` and `lane` required), 1 to 256 entries.
  The records field follows. A batch whose table is empty, has more than 256 entries, or repeats
  an address is refused on decode with `INVALID_FIELD`.
- **D2. The record.** Bytes 4..8 of the 32-byte record become `address_index: u32`, an index into
  the batch's table; an index at or past the table's length is refused on decode with
  `INVALID_FIELD`. `ParameterHandle` leaves the automation record. The record size and every other
  byte do not change.
- **D3. Validation.** Before admission, every address resolves through the provider to a row of
  the committed model, and every record is checked against its row, in record order; the first
  failure refuses the whole batch with `INVALID_FIELD` and a diagnostic carrying the control code:
  `control.address.unknown`, `control.automation.target` (the row is not `automatable()`),
  `control.lane` (a `left` or `right` lane on a shared row), or `control.domain` (a start or end
  value that `controls::check(descriptor, lane, value, None)` refuses). Nothing is queued before
  every record passes (the acked-batch rule). #1552 D5's `automation.target.unsupported` refusal is
  deleted: every automatable row is accepted.
- **D4. The slot holds no string.** Admission resolves each address to the committed catalog's
  handle and stores it, with the batch's `structure_epoch`, in the preallocated
  `AutomationBatchSlot`. This handle never leaves the controller. A commit that advances
  `structure_epoch` cancels every queued batch with `AUTOMATION_CANCELED` reason `RevisionChanged`,
  as a structural revision does today.
- **D5. Docs.** `docs/CONTROL_BTLV_V1.md` and `docs/CONTROL_PROTOCOL_REGISTRY.md` describe the table
  and the record's `address_index`.

## Deliverables

1. D1-D4 in `crates/protocol` (`message_wire.rs`, `queue.rs`, `schema.rs`, `controller.rs`) and
   the provider's row lookup by address in `crates/host-core/src/control_provider.rs`.
2. The conformance corpus and `tools/audit/src/protocol.rs` re-pinned once; D5.

## Authorized paths

- `crates/protocol/src/{message_wire.rs,queue.rs,schema.rs,controller.rs,typed_frame.rs,lib.rs}`,
  `crates/protocol/src/controller/tests.rs`, `crates/protocol/tests/**`.
- `crates/host-core/src/control_provider.rs`, `crates/control-plane/src/control.rs` (the provider
  call only).
- `crates/conformance/src/protocol_corpus.rs`, `tools/audit/src/protocol.rs`.
- `docs/CONTROL_BTLV_V1.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`.
- Tests in `crates/capi` that build an `AUTOMATION_ENQUEUE` frame.

## Non-goals

- No automation rendering; the control plane's `automation` feature stays off (#1315 D3).
- No change to the stored automation grammar (#1566).
- No SDK encoder for `AUTOMATION_ENQUEUE` (none exists).

## Objective gates

Commands: `cargo test --locked -p protocol -p host-core -p control-plane --features test-support`,
`cargo test --locked -p conformance`, `cargo test --locked -p capi`.

1. **Codec.** A batch of 3 addresses and 5 records round-trips; an empty table, 257 entries, a
   repeated address, and a record index equal to the table length are each refused on decode.
2. **Validation by row** (controller test with the `automation` feature on). A batch that targets a
   send's `gain_db` and a builtin `fader_db` is admitted; one record on `delay_samples`, on `solo`,
   on `effective_fader_db`, with `left` on a route `gain_db`, or with an end value of 30 dB on
   `fader_db` refuses the whole batch with the matching code, and the queue is unchanged.
3. **No stale target.** After a commit that adds a VCA (a new `structure_epoch`), a batch queued
   before it is canceled with `RevisionChanged`, and a new batch with the same addresses is
   admitted.
4. **Still not served.** Through the C ABI, `AUTOMATION_ENQUEUE` still answers `Unavailable`
   (#1315).

## Test value

- Gate 1 turns red if a record can point outside its table or a table can repeat a target.
- Gate 2 turns red if enqueued automation can target a value that is not live, or the batch is
  partly queued before a refusal.
- Gate 3 turns red if a queued batch keeps a target across a change of the row set.
- Gate 4 turns red if this slice turns on a command no render path serves.

## Dependencies

- #1551 (the `control_address` message), #1552 (the catalog and `structure_epoch`), #1566 (the row
  rule `controls::automation_diagnostics` names).
- *Refuse commands that would be acknowledged with no effect* (#1315).

## Supersedes and shrinks

- None beyond #1566's line on #1315.

## Amendment (controls coordinator, batch 8, 2026-10-14)

Since #1315 (batch 8), both hosts build their controller without the `automation` provider
feature and refuse `AUTOMATION_ENQUEUE` `UNAVAILABLE` before any check; command `0x0006` is not
advertised. This slice's address table and domain check are therefore reachable only through the
protocol crate's controller with the feature enabled (as `controller::tests` does), not through
either host, until the render-side automation drain (#1058) sets the feature. Gates that drive a
host test the refusal; the address and domain gates run on the protocol controller.

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **Anchors moved.** `AutomationEnqueue` is `message_wire.rs:856`; `RevisionChanged` and
  `AutomationCanceled` are `:969-982`. `validate_automation_domains` is `controller.rs:3861`. #1552
  D5's `automation.target.unsupported` is `controller.rs:3634-3638`, with the provider's note at
  `control_provider.rs:907`. The feature refusal is `execute`, `controller.rs:3156`; the capability
  bit is `:3758`. `queue.rs:58`, `:85` and `:199-207` held.
- **D1's codec exists.** Decode and encode each table entry with
  `message_wire::decode_set_control_address` and `write_set_control_address` (`SET_CONTROL`'s,
  `control` and `lane` required).
- **D3 follows #1566 on `control.inactive`.** `controls::check` returns `control.inactive` for an
  automatable row that the owner's shape does not use. #1566's batch 9 amendment proposes it as a
  fifth code (recommended); this slice uses the same ruling.
- **D4 in fact.** Today every committed session transaction that is not monitor-only cancels the
  queued batches with `RevisionChanged` (`controller.rs:2303` and `:3260`), live ones included. So a
  stored handle never outlives a revision. D4's epoch stamp stays as written; gate 3 is unchanged.
- **#1315, more precisely than the batch 8 amendment.** The frame decodes before `execute`'s feature
  check. Through a host, a batch that fails D1 or D2 decoding is `INVALID_FIELD`, and a well-formed
  batch is `UNAVAILABLE`. Gate 4 sends a well-formed batch in the new shape. Gate 1 stays a codec
  test in `protocol`.
- **No drain is filed.** #1058 (research) is closed and no open issue renders stored or enqueued
  automation. The refusal holds until such a slice is filed and sets the feature.
- **More files that build an `AUTOMATION_ENQUEUE` frame** (authorized):
  `crates/protocol/src/message_wire/tests.rs`, `crates/capi/src/runtime/tests.rs` (`:284-305`,
  `:2251`, `:2710`) and `hosts/host-web/tests/protocol_cross_host.rs` (`:704-730`).
- **Edit order.** After #1566. Before #1615 in `protocol/src/controller.rs` (#1615's S2 edits the
  structural prepare and commit; this slice edits admission). With #1574 and #1615 in
  `host-core/src/control_provider.rs`: this slice first.


## Parked (root, 2026-10-14)

Parked 2026-10-14: #1058 A9 retires AUTOMATION_ENQUEUE; close as superseded when #1627 lands.
