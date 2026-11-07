# Control plane: the render-diagnostic egress mirrors the controller's pending reliable event, so a diagnostic can overtake it

Tracker #2258, item PR-4. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The control plane's reliable egress interleaves its own render-activity diagnostics
(`capi.render.activity`) with the protocol controller's reliable events. On base `de0878e23`, each
staged diagnostic carried a barrier, `protocol_events_before`: the number of protocol events it must
wait behind. Two defects broke reliable FIFO.

1. **The pending event was mirrored.** `ProtocolController::dequeue_reliable_event_frame_into`
   (`crates/protocol/src/controller.rs`) pops an event into its private `pending_reliable_event`. It
   keeps the event there after *any* error, until a complete frame reaches the caller. The control
   plane cannot see that cell, so it mirrored it in `SessionState::protocol_reliable_pending: bool`
   (`crates/control-plane/src/control.rs:1333`). `dequeue_reliable_event` (`control.rs:7506-7570`)
   set the mirror only from `Err(Encode(OutputTooSmall))`, and cleared it on every other error.
   `stage_render_diagnostic` (`control.rs:3192-3197`) took the barrier as queue occupancy plus the
   mirror. After any other dequeue error the controller still held the event, but the barrier
   missed it, so the next diagnostic overtook an older protocol event.
2. **A second staged diagnostic waited for the wrong events.** This was found by the attempt-1
   review, and it is reachable. Each slot stored the backlog at staging as a count of events still
   to wait for, and only the head slot's count was lowered on delivery. A diagnostic staged while
   another is staged therefore also counted the events that go before the first diagnostic. After
   those were delivered it waited for that many more. Either a newer protocol event overtook it,
   or, with none queued, the egress returned `Ok(None)` while it stayed staged.
   - Repro: an apply, then a short-buffer dequeue (`SESSION_COMMITTED` stays pending), then two
     blocks (D1 and D2, each with barrier 1). Drain `SESSION_COMMITTED` and D1, then send
     `TRANSPORT_SET`, then drain.
   - Base and attempt 1 deliver `TRANSPORT_STATE` before D2.

The staged slot was also described twice: `occupied: bool` and `reservation: Option<_>` said the
same thing. Its barrier, revision and the diagnostic's `sample_time`/`provider_sequence` lived on a
free slot (`control.rs:2590-2596`).

## Reachability of defect 1 today
Defect 1 is not reachable through the public API. Every error that leaves the event pending, other
than a short buffer, is a broken invariant of the controller:
- **`Encode(LimitExceeded | InvalidField)`.**
  - `enqueue_diagnostic_event` calls `encoded_diagnostic_event_len`.
  - `enqueue_response_ready_event` and `attach_observation_ended` each make a probe encode.
  - `SESSION_COMMITTED` and `TRANSPORT_STATE` are enqueued without a probe. They are guarded
    incidentally: both are 112-byte frames of fixed-width scalars, measured with the frame's
    origin request ID present, which a `TRANSPORT_SET` always sets. The command that makes each
    one must itself fit the same codec frame limit (`maximum_control_frame_bytes`). The
    attempt-1 review measured the smallest such commands at 120 bytes for `TRANSPORT_SET` and
    112 for `SESSION_TRANSACTION_APPLY`.
  - The codec's limits do not change after boot.
- **`Encode(MessageKindMismatch)`.** Only an `EmptyResponse` slot in the event queue gives this,
  and no enqueue makes one.
- **`ObservationEndedCellMissing`, `ResponseReadyMissing`, `DiagnosticStorageFull`.** The backing
  cell is filled with the slot's enqueue, and emptied only after that slot's frame is delivered.

So defect 1 takes a controller defect or a future error path. The test proves it by putting an
`EmptyResponse` slot, which cannot encode, at the head of the queue. The mirror is still the wrong
model: it copies another component's state, and stays right only by reading every error the way the
controller does (tracker pattern 2).

## Decision
- **The controller owns both counts.** `ProtocolController::reliable_backlog()` returns the
  reliable events it holds whose frames have not reached caller output: queue occupancy, plus one
  for the pending event. `ProtocolController::reliable_events_delivered()` is a monotone count of
  the frames delivered; an error counts nothing. The mirror is deleted.
- **An absolute target.** A diagnostic is staged with
  `deliver_after = reliable_events_delivered() + reliable_backlog()`. It is due once
  `reliable_events_delivered() >= deliver_after`. Nothing is decremented, so every staged slot's
  target stays right as events are delivered, and defect 2 cannot recur. The ring keeps the
  diagnostics in their own order.
- **One slot state.** `RenderDiagnosticSlot { diagnostic, state }` with
  `enum DiagnosticSlotState { Free, Staged { _reservation, deliver_after, revision, sample, sequence } }`.
  - Everything per staging exists only while the slot is staged. Freeing the slot drops the
    reservation.
  - The diagnostic's text stays in the slot's storage, allocated at boot. Its `sample_time` and
    `provider_sequence` are written in only for the encode, and reset to `None` after it.
- **Egress.** While the head is not due, the controller's event goes first, and an error leaves
  that event pending and first.
  - **Invariant:** no staged slot waits for more events than the controller holds
    (`deliver_after - delivered <= reliable_backlog()`). A `debug_assert!` checks it at every
    reliable dequeue.
  - **The empty-controller branch** (`Ok(None)` while the head is not due) can only be reached by
    breaking the invariant. A debug build stops there, with the same message. A release build
    recovers defensively: nothing older remains, so the diagnostic follows. Attempt 1 claimed
    this branch "cannot happen", but defect 2 reached it.
- **Kept in scope.** The slot enum is in the same two functions as the target, and the target is
  the FIFO mechanism. Folding it into `Staged` means a free slot has no target at all. This is the
  same outcome: FIFO-correct reliable egress.

## Non-goals
- The provider's `DIAGNOSTICS_GET` projection (`ControlProvider::set_render_diagnostic`,
  `crates/host-core/src/control_provider.rs`). It serves a different reader, the history query,
  and egress does not read it as a copy.
- The ring's head/len.
- A probe encode for `TRANSPORT_SET`'s event (see Follow-ups).
- The wire and rendered output. Everything here runs in the control-thread service and egress, not
  in render.

## Gates
- A regression test red on base and on attempt 1, green at head, and mutations of the fix that
  turn it red.
- `cargo test --locked` for `protocol` and `control-plane`, and for the egress consumers `capi`
  and `host-web`.
- `cargo clippy --locked ... --all-targets --all-features -- -D warnings`.
- `cargo fmt --check`, `bash scripts/check-workspace-policy.sh`, and the realtime policy tool.

## Attempt 1 (a8f775379): FAIL
Adversarial review findings:
- **BLOCKER:** defect 2 above. The per-slot count, lowered only at the head, made a second staged
  diagnostic wait for events that go before the first. It is reachable through the public API.
- **MAJOR:** the new `Ok(None) => {}` "follow" branch was reachable through the blocker and hid
  it. No test covered it: restoring base's `Ok(None) => return Ok(None)` stayed green.
- **MINOR:** the reachability record called `SESSION_COMMITTED`/`TRANSPORT_STATE` "far below any
  frame limit". The real guard is incidental, and is recorded above.
- **MINOR:** `sample_time`/`provider_sequence` were `Some` only while staged, but lived outside
  `Staged`.

The attempt-1 record follows unchanged, apart from the gates' heading.

- **Test:**
  `rendered_meters::a_render_diagnostic_waits_behind_the_event_the_controller_holds_pending`. It
  drained a short-buffer case (committed, diagnostic, transport), then an injected `EmptyResponse`
  case.
- **Red on base:** `the pending event still heads the lane, so the diagnostic waits: Ok(Some(144))`.
- **Mutations m1-m3**, all red:
  - the backlog without its pending term;
  - an error falling through to the diagnostic;
  - no barrier decrement.
- **Gates** were green: 744 tests in `protocol`, `control-plane`, `capi` and `host-web`, then
  clippy, fmt, workspace policy and realtime policy.

## Evidence (attempt 2)

**Tests** (`crates/control-plane/src/control.rs`, `control::tests::rendered_meters`, under
`control-plane/test-support`):
- **`each_render_diagnostic_goes_after_exactly_the_protocol_events_older_than_it`.** This
  replaces attempt 1's test and has two cases.
  - **The review's repro** (defect 2, through the public API, and defect 1's short-buffer path):
    an apply, then a short-buffer dequeue, then blocks 0 and 1 (two diagnostics). It drains two
    events, then sends `TRANSPORT_SET`, then drains the rest. The order must be committed,
    diagnostic, diagnostic, transport.
  - **An injected `EmptyResponse` at the head** (defect 1's non-short-buffer error). After a
    block, the next dequeue must fail again rather than deliver the diagnostic.
- **`a_diagnostic_waiting_for_events_the_controller_lost_stops_a_debug_build`** (debug builds
  only, `should_panic`). A `TRANSPORT_STATE` is queued and a diagnostic staged behind it. The event
  is then popped from the queue directly, which no product path does, so the controller no longer
  holds what the diagnostic waits for. The next reliable dequeue must stop on the invariant.

**Red on base `de0878e23`** (both tests, with only the `EventError` import added):
```
test ...each_render_diagnostic_goes_after_exactly_the_protocol_events_older_than_it ... FAILED
  left: Some("transport")
 right: Some("diagnostic")
test ...a_diagnostic_waiting_for_events_the_controller_lost_stops_a_debug_build - should panic ... FAILED
test result: FAILED. 0 passed; 2 failed
```
- On base, `TRANSPORT_STATE` overtook D2.
- On base, the lost event left the egress stalled in silence (`Ok(None)`), with nothing to stop it.

**Red on attempt 1 `a8f775379`** (the order test): the same `left: Some("transport") right:
Some("diagnostic")` at the second `diagnostic` assertion.

**Test value:**
- **The order test** goes red if a diagnostic's target misses the pending event, or if a second
  staged diagnostic waits for events that go before the first. Either way a diagnostic is
  reordered against a protocol event. No existing test stages a diagnostic while a protocol event
  is pending, or stages two behind one: the #2198 slot test and the capi egress tests drain with a
  roomy buffer.
- **The `should_panic` test** goes red if the egress stops checking the invariant. A lost or
  uncounted delivery would then be followed or stalled on in silence.

**Mutations** (each applied alone to head, both tests run; script and output in the session
scratchpad):
- **n1**, `reliable_backlog` without its pending term: the order test is red (diagnostic before
  committed).
- **n2**, a relative target (the backlog alone, not `delivered + backlog`): the order test is red.
- **n3**, the controller does not count a delivery: the order test is red. The invariant
  `debug_assert!` fires.
- **n4**, an egress error falls through to the diagnostic: the order test is red (injected case).
- **n5**, the dequeue-entry invariant check removed: both tests stay green. This is equivalent
  detection: the `Ok(None)` branch's own check fires with the same message.
- **n6**, both checks removed, with the release fallback (follow) kept: the `should_panic` test is
  red.
- **n7**, both checks removed, with base's `Ok(None) => return Ok(None)` restored: the
  `should_panic` test is red.
- Attempt 1's per-slot model is attempt 1 itself: the order test is red there, as recorded above.

**No bit moved.** Not applicable, because no render-path code changed. `stage_render_diagnostic`
and `dequeue_reliable_event` run in the control thread's service step and egress.

**Cross-thread.** None. The target, the slots and the controller's counts are all owned by the
control thread, so no loom model is needed.

**Gates (head):**
- `cargo test --locked --all-targets -p protocol -p control-plane -p capi -p host-web --features protocol/test-support,control-plane/test-support,host-web/test-support,host-web/render-lock-probe,host-core/test-support`:
  57 test binaries, 745 tests, all pass.
- `cargo clippy --locked -p protocol -p control-plane -p capi -p host-web --all-targets --all-features -- -D warnings`:
  clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.

## Follow-ups
- The provider's `DIAGNOSTICS_GET` projection mirrors the staged slots through
  `set_render_diagnostic(slot, sample, sequence, live)`, a second writer of the same facts. A
  provider view that reads the slots directly would remove it, but that crosses into `host-core`.
- `TRANSPORT_SET` and the structural `SESSION_COMMITTED` enqueue their events without a probe
  encode. They are guarded only incidentally, by their command's own size. A probe, like
  `enqueue_response_ready_event`'s, would make the guard explicit.
- `compile_children_warm` (`crates/control-plane/src/compile.rs`) is `#[cfg(test)]` but only
  test-support tests use it. As a result, clippy on `control-plane` without `test-support` fails
  with dead code, on base as well.
