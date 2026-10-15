# Delete the protocol automation queue, its records and counters

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A8 and A9, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P1.

## Product outcome

After drafts 21a *Retire the AUTOMATION_ENQUEUE command from the protocol wire and dispatch* and
21b *Retire the AUTOMATION_CANCELED event and the cancellation path*, nothing enqueues into the
protocol's automation queue. This slice deletes it: the
32-byte record codec, the batch slot, the queue and its admission state, its configuration and
report, queue kind 2 and the five automation counters. Every C ABI session stops allocating the
queue, so `capi_retained_bytes` falls. Queue kind 2 and counters 2 and 11-14 are retired: refused,
never reallocated, never renumbered.

This is the third of three slices that retire the command, all in batch P1 (one push), so `main`
never holds a queue that nothing calls.

## Context

- **The queue and its record** (`crates/protocol/src/queue.rs`): `AutomationKind`,
  `AutomationRecord` and its manual 32-byte codec (`:30-154`), `AutomationBatchError`
  (`:157-195`), `AutomationBatchSlot` (`:197-283`), `AutomationEnqueueError` (`:619-636`), the
  automation pair, frontier, density and interval rows of `ProtocolQueues` (`:643`, `:651-653`,
  `:657-679`), sized in `resource_report_for_config` and `prepare` (`:683-735`, `:738-765`),
  `try_enqueue_automation`, `try_dequeue_automation` and
  `reset_automation_ordering_after_cancellation` (`:771-813`), `validate_automation_admission` and
  its helpers (`:1062-1194`). `AUTOMATION_RECORD_BYTES` is `crates/protocol/src/lib.rs:64-65`.
- **Configuration.** `ProtocolQueueConfig::{automation_batch_slots, per_block_automation_density,
  quantum_frames}` (`queue.rs:527-528`, `:535-538`); `quantum_frames` is "used only for
  control-side density admission". After draft 21a no capability field reads them.
- **Queue kind 2.** `QueueKind::Automation` (`queue.rs:576-577`), mapped to
  `BackpressureQueueKind::Automation = 2` (`crates/protocol/src/message_wire.rs:769-770`, decode
  `:785`) at `crates/protocol/src/controller.rs:3169`; `docs/CONTROL_PROTOCOL_REGISTRY.md:13`.
- **Counters 2, 11-14.** `CounterId::{AutomationBackpressure = 2, LateAutomation = 11,
  CanceledAutomation = 12, AutomationTimePast = 13, AutomationOrderReject = 14}`
  (`message_wire.rs:628`, `:637-640`; `parse_counter_id`, `:2035`, `:2044-2047`). After drafts
  21a and 21b nothing writes any of them (21b removes `record_canceled_automation`). No corpus row
  or test configures
  them (`tools/audit/src/protocol.rs:305` and `crates/capi/src/runtime/tests.rs:2030` use counter
  1).
- **The page bound.** `AUTOMATION_BATCH_RECORDS = 256` (`crates/protocol/src/lib.rs:62-63`) sizes
  the batch and also the meter and counter pages (`controller.rs:1232`, `:1238`, `:2741`, `:2788`).
- **The C ABI.** `protocol_queue_config` builds the queue from the limits
  (`crates/capi/src/runtime/compile.rs:114-138`: one batch slot, the density from S, the
  quantum); `capi_resources` charges its rows (`:151`); the test-support report carries an
  `automation` row (`crates/capi/src/runtime/control.rs:156`, `:665-668`). #1309 moves both files
  to `crates/control-plane/src/`. The exact-charge oracle is
  `capi_retained_bytes_charge_every_byte_the_compile_retains`
  (`crates/capi/tests/resource_lifecycle.rs:926`).
- **The protocol audit** configures the queue (`tools/audit/src/protocol.rs:115-119`, `:284-288`).
- **Tests that exist only for the queue:** `queue.rs` tests
  `queue_resource_projection_is_nonzero_and_shares_prepare_overflow_rules` (`:1257`),
  `transient_record_has_exact_manual_le_codec` (`:1286`),
  `ten_thousand_events_fit_as_exactly_forty_atomic_batches` (`:1329`),
  `past_order_overlap_and_density_reject_wholly` (`:1353`),
  `overlap_is_rejected_across_separately_queued_batches` (`:1386`),
  `invalid_and_out_of_order_slots_return_whole_unchanged_batches` (`:1418`),
  `zero_record_and_aggregate_frontier_density_reject_wholly` (`:1456`); and
  `full_empty_wrap_and_generation_preserve_original_items` (`:1307`), which also covers the other
  queues.
- **Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md:13` (queue kind), `:45` (the record);
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:23`; `docs/CONTROL_PROTOCOL_SIZING.md:19`, `:25`;
  `docs/REALTIME_MEMORY.md:13`.
- **#1351** (*Report each configured counter's own value in the C ABI counter snapshot*) D1 serves
  `CANCELED_AUTOMATION` from construction.

## Decisions frozen for this slice

- **D1. The queue leaves.** Every item in Context's queue and record list leaves, with
  `AUTOMATION_RECORD_BYTES` and the exports at `crates/protocol/src/lib.rs:34-38` that name them.
  `ProtocolQueueConfig` loses its three automation fields.
- **D2. Retired codes.** `QueueKind::Automation` leaves; `BackpressureQueueKind` decode refuses 2.
  The five counters leave `CounterId`; `parse_counter_id` refuses 2 and 11-14. None is
  reallocated: the next additions take new numbers (#1312's 16, #1351's next free value).
  `retired_code_rows` gains a row for queue kind 2 in a backpressure payload and for each counter
  ID in a `COUNTERS_GET` and a `TELEMETRY_CONFIGURE` request.
- **D3. The page bound** is renamed to a name of its own for the meter and counter pages (for
  example `MAXIMUM_TELEMETRY_PAGE_RECORDS`), still 256, at every use.
- **D4. The C ABI** builds no automation queue and reads neither S nor the quantum for it; the
  test-support report loses its row. `capi_retained_bytes` falls by the removed rows; the
  exact-charge oracle stays exact, and the reference budgets are ceilings that still hold. The
  compile limit S keeps its validation until draft 22 reserves it.
- **D5. Tests.** The queue-only tests above are deleted; the wrap test keeps its other queues.
- **D6. Docs.** The registry lists queue kind 2 and the five counters as retired with this slice
  as the reason, and removes the record line (`:45`); the semantics, sizing and realtime-memory
  lines lose the queue.
- **D7. The acked-batch question** no longer has a subject on the protocol: no queue remains to
  hold a batch, so no ack can precede a drop.

## Deliverables

1. D1-D3 and D5 in `crates/protocol`; D4 in the C ABI control plane; the audit configuration.
2. D2's retired rows; D6's docs.
3. The gates below.

## Authorized paths

- `crates/protocol/**`
- `crates/control-plane/src/`, the files that hold capi's compile and control code after #1309
  (which lands before batch P1; `crates/capi/src/runtime/{compile,control}.rs` on `6ee64f484`)
- `crates/conformance/src/protocol_corpus.rs` (retired rows only)
- `tools/audit/src/protocol.rs` (the queue configuration)
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/CONTROL_PROTOCOL_SIZING.md`, `docs/REALTIME_MEMORY.md`
- `docs/C_ABI_V1_QUALIFICATION.md` (the retained-byte rows that included the queue; Hazards)
- `crates/capi/src/runtime/tests.rs` (only a use of `protocol::AutomationRecord` that drafts 21a
  and 21b leave, such as `:1915` and `:2370` on `6ee64f484`)

## Non-goals

- Wire identities, dispatch, capabilities and the corpus hash (drafts 21a and 21b).
- The C limit `maximum_automation_spans_per_block` (draft 22).
- The other queues and their configuration.

## Hazards

- **Batch P1.** This slice deletes what drafts 21a and 21b leave unreachable; it cannot merge
  before them (21a's dispatch enqueues, 21b's cancellation dequeues), and they must not reach
  `main` without it.
- **#1351.** *Report each configured counter's own value in the C ABI counter snapshot* (#1351) is
  amended (README amendment row) never to serve `CANCELED_AUTOMATION`, so the merge order of #1351
  and this slice does not matter and no served counter is removed later.
- **Retained-byte rows.** Any exact C ABI resource figure in docs that included the queue
  (`docs/C_ABI_V1_QUALIFICATION.md` tables) moves; list each with the reason "draft 21c deletes the
  automation queue". A ceiling that still holds is not lowered here.

## Objective gates

1. **Retired codes refused** (`crates/protocol/src/controller/tests.rs`, through
   `retired_code_rows`). A backpressure payload with queue kind 2 fails decode; a
   `COUNTERS_GET` or `TELEMETRY_CONFIGURE` naming counter 2, 11, 12, 13 or 14 is refused as an
   invalid field, with no state change.
2. **Queues still work** (`crates/protocol/src/queue.rs` tests). The rewritten wrap test passes for
   the control, response, event and telemetry queues; `resource_report_for_config` and `prepare`
   agree on every remaining row.
3. **C ABI charges stay exact.** `capi_retained_bytes_charge_every_byte_the_compile_retains`
   (`crates/capi/tests/resource_lifecycle.rs:926`) and
   `reference_session_retained_rows_stay_within_their_budgets` pass; the PR records
   `capi_retained_bytes` on the reference session before and after.
4. **No rendered bit moves.** `cargo build --locked --release -p audit -p bench -p capi -p
   session-validator`, then `./target/release/audit capi` (zero allocations, locks and syscalls,
   the same `pcm_digest` as base, PR evidence) and
   `bash scripts/run-protocol-allocation-audit.sh target/release/audit`.
5. **Commands:**
   - `cargo test --locked -p protocol --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p conformance`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-protocol-control-policy.sh`, `bash scripts/test-protocol-control-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke -p protocol`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a retired queue kind or counter still decodes, or is reused. No test refuses
  them today.
- Gate 2 turns red if removing the automation rows breaks another queue's sizing or wrap.
- Gate 3 turns red if the C ABI still allocates a row it no longer charges, or charges a row it no
  longer allocates.

## Dependencies

Batch P1. Direct dependency:

- Draft 21b *Retire the AUTOMATION_CANCELED event and the cancellation path*, which brings draft
  21a.

*Extract the C ABI control plane into a portable crate both hosts call* (#1309) arrives through
draft 21a. Draft 22 *Reserve the C ABI's automation span limit* follows this draft.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Moved files.** The queue configuration is `protocol_queue_config`
  (`crates/control-plane/src/compile.rs:185-205`), which both hosts build: the browser passes its S
  through `control_limits` (`hosts/host-web/src/lib.rs:4494`). The report row is
  `crates/control-plane/src/control.rs:252-253`, `:929`. D4 applies to both hosts.
- **#1351 has landed** and serves `CANCELED_AUTOMATION`; this slice retires counter 12 as written.
- **The record line** is `docs/CONTROL_PROTOCOL_REGISTRY.md:93` (not `:45`); draft 01 gives the hold
  rule its new home first.
- Batch P1, ready.

## Decision record: #1629 delete the protocol automation queue (batch 10)

**What shipped.** The automation queue and everything that fed it are gone from `protocol`, the
control plane, host-core, the audit and the policy tool; no wire identity other than the retired
codes below changed.

- `crates/protocol/src/queue.rs`: `ParameterHandle`, `AutomationKind`, `AutomationRecord` (and its
  32-byte codec), `AutomationBatchError`, `AutomationBatchSlot`, `AutomationEnqueueError`, the
  automation `QueuePair`, frontier, density and interval rows, `try_enqueue_automation`,
  `try_dequeue_automation`, `validate_automation_admission` and its helpers,
  `QueueKind::Automation`, and the config fields `automation_batch_slots`,
  `per_block_automation_density` and `quantum_frames`. `resource_report_for_config` and `prepare` no
  longer size or allocate them. The remaining queues keep their generation numbers (1, 3, 4, 5, 6):
  a gap at 2 is the retired queue's.
- `crates/protocol/src/lib.rs`: `AUTOMATION_RECORD_BYTES` and the exports leave.
  `AUTOMATION_BATCH_RECORDS` is renamed `MAXIMUM_TELEMETRY_PAGE_RECORDS` (still 256) at its four
  uses, the meter and counter pages (D3).
- `message_wire.rs`: `CounterId::{AutomationBackpressure = 2, LateAutomation = 11,
  CanceledAutomation = 12, AutomationTimePast = 13, AutomationOrderReject = 14}` and
  `BackpressureQueueKind::Automation = 2` leave; `parse_counter_id` refuses 2 and 11-14 (and every
  other unallocated ID) as `InvalidField`, the backpressure decode refuses 2 as `InvalidTlv`.
  `controller.rs` drops the `QueueKind::Automation` arm and a stale `large_enum_variant` allow.
- Control plane (`compile.rs`, `control.rs`): `protocol_queue_config` takes only the limits and
  builds no automation row; the quantum is no longer read for the queue and `capi_resources` loses
  its `quantum_frames` argument; the test-support report loses its `automation` row. Both hosts
  build the queue here (the browser through `control_limits`), so D4 applies to both. The compile
  limit S (`maximum_automation_spans_per_block`) keeps its validation, untouched (draft 22).
- host-core: `SERVED_COUNTERS` is four IDs (`TELEMETRY_COALESCED`, `TELEMETRY_DROPPED`,
  `LIVE_VALUES_SUPERSEDED`, `TRANSITION_REPREPARE_REFUSALS`); `CANCELED_AUTOMATION` is no longer
  served.
- `tools/realtime-policy/src/policy.rs`: the `try_dequeue_automation` allowlist row leaves
  (13 rows to 12). `tools/audit/src/protocol.rs`: the queue configuration loses the three fields.
- Docs: registry (queue kind line, the retired paragraph for kind 2 and the five counters, the
  record line, counter ID range), semantics (queue and late-counter sentences), sizing (the
  automation and density rows and the 10,000-record paragraph), realtime memory.

**Tests.** Deleted (existed only for the queue): `transient_record_has_exact_manual_le_codec`,
`ten_thousand_events_fit_as_exactly_forty_atomic_batches`, `past_order_overlap_and_density_reject_wholly`,
`overlap_is_rejected_across_separately_queued_batches`,
`invalid_and_out_of_order_slots_return_whole_unchanged_batches`,
`zero_record_and_aggregate_frontier_density_reject_wholly`, and the controller test helpers `batch`
and `controller_at_sample`. Rewritten: `queue_resource_projection_is_nonzero_and_shares_prepare_overflow_rules`
(the overflow now comes from the event queue's capacity), `full_empty_wrap_and_generation_preserve_original_items`
(gate 2: response, event and telemetry queues, each full outcome returning the original item with its
report; the control queue has no enqueue method, so it is checked by report only),
`every_counter_id_round_trips_and_the_next_is_refused` (the allocated set is pinned by value; 2, 11-14
and 18 are refused one ID per request, so the sorted-unique rule cannot be the cause),
`a_revision_change_and_a_locate_reserve_and_emit_only_their_own_events` (loses its enqueue line),
`retired_and_console_refused_codes_meet_their_conformance_rows` (16 + 4 + 7 + 13 + 1 + 4 + 10 + 1),
and the C ABI counters tests and the pinned counters response vector (request 8: length 0x100 to
0xd0, TLV count 6 to 5, one 40-byte value message fewer).

**Gates.**

1. Retired codes refused. `retired_code_rows` gains ten rows: `retired.counter_{2,11,12,13,14}.telemetry_configure`
   (`INVALID_FIELD`, `protocol.failure`) and `retired.counter_{...}.counters_get` (`NOT_FOUND`,
   `protocol.failure`), judged by `retired_and_console_refused_codes_meet_their_conformance_rows`.
   Queue kind 2 in a backpressure payload: `a_backpressure_payload_naming_the_retired_automation_queue_is_refused`
   (message_wire tests; 1, 3, 4, 5 decode, 0, 2 and 7 fail).
2. Queues still work: `full_empty_wrap_and_generation_preserve_original_items` and
   `queue_resource_projection_is_nonzero_and_shares_prepare_overflow_rules`.
3. C ABI charges stay exact: `capi_retained_bytes_charge_every_byte_the_compile_retains` and
   `reference_session_retained_rows_stay_within_their_budgets` pass. `capi_retained_bytes` on the
   reference session: 197,473 before, 164,249 after (33,224 bytes: the batch ring, the density and
   interval rows); the ceiling 216,832 stands, not lowered. `largest_named_allocation_bytes` is 48,915
   both ways.
4. No rendered bit moves: `audit capi` `pcm_digest` `76525e7ef9e78cf4`, equal to base, with 0
   allocations, locks and syscalls; `run-protocol-allocation-audit.sh target/release/audit` passes.
5. Commands: protocol (`--features test-support`), control-plane (`--features test-support`),
   host-core (`--all-features`), capi, conformance, host-web (`--features host-web/test-support`) and
   parameter-metadata tests; clippy `--workspace --all-targets --all-features -D warnings`; fmt;
   wasm parity and `--self-test`; both vocabulary scripts; `check-protocol-control-policy.sh` and its
   test; `check-workspace-policy.sh`; `check-sdk-generated.sh`; `realtime-policy` (131 regions);
   fuzz `cargo check`; `check-cross-targets.sh` (no new `memset_pattern16`).

**Departures.**

- **A `COUNTERS_GET` naming a retired counter is `NOT_FOUND`, not `INVALID_FIELD`.** Its IDs are raw
  `u32`s that the provider looks up (`CountersRequest`: "unknown values return typed provider
  NOT_FOUND"); refusing five numbers at decode while 99 stays `NOT_FOUND` would make the registry
  inconsistent. A `TELEMETRY_CONFIGURE` is a decode refusal and is `INVALID_FIELD`: `parse_counter_id`
  now returns `InvalidField` (was `InvalidTlv`) for every unallocated ID, as the control tables do.
  Both leave no state change.
- **Response-side retirements are not conformance rows** (as in #1627): queue kind 2 in a backpressure
  payload is a response field, so it is a message_wire decoder test, not a `RetiredCodeRow`.
- **The wrap test covers response, event and telemetry, not the control queue's enqueue**, which has
  no method (removed unused in #1034).
- **No re-pin.** No corpus frame carried the queue, the record or a retired counter, so
  `COMPLETE_SCHEMA_HASH` (`0x4b21_d9e6_f520_afd9`), the parity self-test strings, the fuzz manifest and
  the conformance manifest stand. Only the C ABI counters-response vector moved (above). The
  conformance record's frame count is unchanged (45).
- **The hold rule has no new home yet.** The spec deletes the registry's record line; its one rule
  not stated elsewhere, "a segment applies on `[start,end)` and holds its end value until replaced",
  was waiting for draft 01 (#1626) to give it a home in `docs/SESSION_SCHEMA_V1.md`, which has not
  landed on this base. The registry keeps that sentence alone, pointing at #1626; #1626 moves it.
- Mechanical edits outside the authorized paths: `crates/host-core/src/control_provider.rs`
  (the served counter set and its test), `tools/realtime-policy/src/policy.rs` (the allowlist row),
  `crates/conformance/src/protocol_corpus.rs` (the former-payload row no longer builds an
  `AutomationRecord`; its 32 bytes are written out literally, so the row's bytes are unchanged),
  `crates/protocol/src/controller.rs` and `crates/protocol/src/message_wire.rs` (named in the spec),
  `docs/CONTROL_PROTOCOL_SIZING.md` line 34 (the word "density").
- `docs/C_ABI_V1_QUALIFICATION.md` states no exact retained-byte figure that included the queue, so
  it is unchanged (Hazards: nothing moved there).
