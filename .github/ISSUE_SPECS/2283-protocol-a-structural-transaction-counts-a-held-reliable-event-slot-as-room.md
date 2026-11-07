# Protocol: a structural transaction counts a held reliable-event slot as room and fails INTERNAL

Found by the adversarial review of #2268, filed under tracker #2258. It is the tracker's pattern 2:
a slot a reservation holds is state that the admission precheck does not see.

## Mission
Make every reliable-event admission read the queue's room from one owner check that counts held
slots, so that a read-only precheck and the reservation it predicts cannot disagree. A transaction
refused for lack of room is then a typed `BACKPRESSURE`, refused before any state changes, as
AGENTS.md requires for queue saturation.

## Verified defect (base `1aee7e20d`)
- `ProtocolController::plan_structural_command` (`crates/protocol/src/controller.rs` 2998-3008)
  prechecks the room as `report.capacity - report.occupancy`. `occupancy` counts queued events only.
- The queue's reservations (`crates/protocol/src/queue.rs` 475-518) also subtract
  `reliable_event_reservations`, the slots that outstanding holds own. The unreserved
  `try_enqueue_event` (445-468) does the same, in a third form of the formula.
- The control plane holds one such reservation for each render diagnostic it stages
  (`crates/control-plane/src/control.rs` 3198, `stage_render_diagnostic`). With a slot held, the
  precheck passes, and then:
  - **prepared path** (`prepare_structural_plan`, controller.rs 2857-2860): `reserve_reliable_events`
    fails and is mapped to `CommandFrameProcessError::Internal`. The control plane's
    `map_command_process_error` turns that into `CommandError::Internal`.
  - **one-call path** (`process_structural_plan_into`, controller.rs 3136-3139):
    `.expect("read-only structural plan reserved exact reliable-event capacity")` panics. This is
    reachable through the public protocol API, which a reservation taken by `queues_mut()` and then
    `process_command_frame_into` drive. No host calls the one-call path today; the control plane
    prepares.
- The legacy `ControllerRequest` paths (`TransportSet`, `SessionTransactionApply` in `execute`)
  reserve directly, so their refusal is already typed. Their report, though, said
  `occupancy 0 of capacity 1` for a held slot, so the refusal looked as if the queue had room.
- No other site in the workspace prechecks reliable-event room. `stage_render_diagnostic` reserves
  directly. `attach_observation_ended` reserves directly and maps `NoRoom` to `Backpressure`. The
  egress refusals while a token is outstanding report but do not admit.

## Decision / model
- One owner check: `ProtocolQueues::check_reliable_event_room(&self, count) -> Result<(), QueueReport>`.
  It reads the queued events plus the held slots, and is read-only. Its refusal report's
  `occupancy` counts the held slots, so `capacity - occupancy` is the room the refusal saw.
- `reserve_reliable_event`, `reserve_reliable_events` and `try_enqueue_event` call it and nothing
  else. The structural plan's precheck calls it in place of its own formula. The precheck and the
  reservation are now one function, called twice under one `&mut self`. Between the two calls only
  a release (a drop on any thread) can change the count, and a release only adds room. So the
  prepared path's `Internal` mapping and the one-call `expect` stay as defect reports and cannot be
  reached. Their comments say so.
- `ProtocolQueues::report` is unchanged: it still counts queued events only. The control plane's
  FIFO barrier (`protocol_events_before`) and #2268's `reliable_backlog()` read it as queued events.
- Docs: one sentence in `docs/CONTROL_PROTOCOL_SEMANTICS.md` (the queue-saturation paragraph). A held
  slot is full; a transaction refused for it is `BACKPRESSURE`; the refusal's pre-attempt occupancy
  includes it. The wire field and the registry are unchanged.

## Non-goals
- No change to the render-diagnostic slots, the `protocol_reliable_pending` mirror, or the
  `reliable_backlog()` / `reliable_events_delivered()` accessors. Those belong to #2268, on
  `codex/issue-2268-reliable-backlog`, which edits other regions of `controller.rs` and
  `control.rs`. This fix needs neither accessor.
- No render path is touched: the queue's render-side consumer is unchanged, and admission is
  control-side.
- No new cross-thread state. The shared reservation count already exists; it is only read through
  the one check now, so no loom model is needed.

## Gates
- A regression test red on base, green at head, through the public protocol API, which owns this
  admission logic.
- Mutation tests of the fix.
- `cargo test --locked -p protocol -p control-plane --features test-support`, plus `-p capi -p host-web`
  because the reported occupancy reaches their wire.
- `cargo clippy --locked -p protocol --all-targets --all-features -- -D warnings`, `cargo fmt --check`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p protocol`,
  `bash scripts/check-workspace-policy.sh`, `./target/release/realtime_policy`.

## Evidence (attempt 1)

**Red on base.** `controller::tests::a_held_reliable_event_slot_refuses_a_structural_transaction_as_backpressure`
(`crates/protocol/src/controller/tests.rs`). The test holds the one slot of a one-slot queue with
`queues_mut().reserve_reliable_event()`. It then applies `SetSessionId` through
`prepare_command_frame` and through `process_command_frame_into`, and asserts the following:
- both replies are `BACKPRESSURE`, with the queue kind, capacity 1, occupancy 1, one requested
  item, and generation 4;
- the revision and the canonical snapshot are unchanged;
- both refusals are in the replay cache;
- nothing was queued;
- after the hold is released, the same edit commits at revision 8.

On base `1aee7e20d`:
```
panicked at crates/protocol/src/controller/tests.rs:3626:10:
a typed reply: Internal
```
A scratch, uncommitted variant on base that drives only the one-call path:
```
panicked at crates/protocol/src/controller.rs:3139:14:
read-only structural plan reserved exact reliable-event capacity: QueueReport { kind: ReliableEvent, capacity: 1, occupancy: 0, requested_slots: 1, generation: QueueGeneration(4) }
```

**The control-plane repro.** A scratch test in `crates/control-plane/src/control.rs`
(`rendered_meters`, never committed) enables diagnostics, renders block 0, and applies `SetSessionId`
twice through `ControlSession::command`. On base: request 3 is OK, and request 4 is
`Err(Internal)`. At head: request 3 is OK, and request 4 is `Ok`, a `BACKPRESSURE` frame with
`{ queue_kind: ReliableEvent, capacity: 2, occupancy: 2, requested_items: 1, generation: Some(4) }`.
This is evidence, not a second committed test. The claim is the protocol's admission, and it is
tested once, in the crate that owns it.

**Mutations (each reverted).**
- The structural precheck restored to `capacity - occupancy` from `report()`: the new test is red
  (`a typed reply: Internal`).
- `check_reliable_event_room` with the held count zeroed: the new test is red, and so are three
  existing tests (`the_end_needs_room_with_the_commit_and_a_dropped_token_releases_it`,
  `reliable_event_reservation_excludes_commit_capacity` and
  `a_reservation_committed_to_another_queue_changes_neither_queue`).
- `check_reliable_event_room` refusing with the raw `report()` (held slots not in the reported
  occupancy): only the new test is red (`tests.rs:3642`, the backpressure payload).

**Test value.** The test is red if the structural plan's room check and the queue's reservation
read the room differently, so that a held slot passes the check and the transaction fails
`INTERNAL` (prepared) or panics (one-call). It is also red if a refusal reports a held slot as free.
No existing test holds a reservation across a structural frame. The existing tests either fill the
queue with a queued event, which both formulas see, or reserve on the legacy path, which never
prechecked. No test is made useless.

**Gates (head).**
- `cargo test --locked -p protocol`: 216 passed, 0 failed.
- `cargo test --locked -p control-plane --features test-support`: 11 binaries, 99 passed, 0 failed.
- `cargo test --locked -p capi -p host-web`: 30 binaries, 356 passed, 0 failed.
- `cargo check --locked --workspace --all-targets`: clean, apart from one pre-existing host-web
  test warning (`set_state_for_test` is never used), which this change does not touch.
- `cargo clippy --locked -p protocol --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p protocol`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.
