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
