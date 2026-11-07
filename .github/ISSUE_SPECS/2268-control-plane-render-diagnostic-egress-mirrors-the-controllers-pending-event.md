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
- The ring's head/len. The empty-ring branch's `debug_assert_eq!(render_diagnostic_len, 0)` does
  not catch a `Staged` head with `len == 0` (attempt-2 review note).
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

## Attempt 2 (9b1ae7f6a): FAIL
The review accepted the absolute-target model and found no other FIFO violation. It checked three
and four diagnostics, ring reuse, and a commit staged while a plan is pending; all came out in
order. It also confirmed:
- the counter rises exactly once per delivered frame, on every path;
- `dequeue_reliable_event_frame_into` is the only egress;
- the controller is never replaced, so a session replacement cannot reset the counts.

Findings:
- **MAJOR:** no test checked that "an error counts no delivery". Moving the increment out of
  `if result.is_ok()` kept the whole gate green, yet it breaks FIFO through the public API. The
  repro: apply `SetSessionId`, render block 0 (the diagnostic's target is 1), make a short-buffer
  dequeue (counted under the mutation), then a full dequeue. The diagnostic goes before
  `SESSION_COMMITTED`.
- **MINOR:** the counter's doc did not say that saturation is unreachable.
- **Note:** the ring branch with no staged head checks `render_diagnostic_len == 0`, but a
  `Staged` head with `len == 0` passes it unseen. This is listed as a non-goal.

Out of scope, and filed separately by the coordinator: a staged diagnostic's reservation makes a
later structural commit fail with `Internal` instead of `BACKPRESSURE`. The structural precheck
(`controller.rs:3001-3010`) ignores reservations. This issue does not change it.

## Attempt 3 (14c8be408): PASS, one MINOR folded in attempt 4
The review found no FIFO violation, and passed attempt 3 with one MINOR.

**MINOR: the counting rules were tested only outside the crate that owns them.** The reviewer's
mutations M1, M3, M8 and M9 (the backlog's pending term, the delivered count's increment, an
error counted as a delivery, the backlog's occupancy term) left
`cargo test -p protocol --features test-support --lib` green. The control-plane tests caught each
one, and M9 only through the debug-only `should_panic` test.

**Folded (attempt 4).** One protocol unit test,
`controller::tests::the_reliable_backlog_and_delivered_count_follow_each_dequeue_outcome`. It
queues `SESSION_COMMITTED` and `TRANSPORT_STATE`, then dequeues:
- with a short buffer;
- successfully;
- successfully again, after queuing an `EmptyResponse` slot;
- once more, which fails with `MessageKindMismatch`.

It asserts `(reliable_backlog(), reliable_events_delivered())` after each step: `(2, 0)` queued,
`(2, 0)`, `(1, 1)`, `(1, 2)`, `(1, 2)`. Each of these mutations, applied alone, turns it red in a
plain test run, so not only in debug builds:
- n1, no pending term: `(1, 0)` against `(2, 0)`.
- n3, no increment: `(1, 0)` against `(1, 1)`.
- n8, an error counted: `(2, 1)` against `(2, 0)`.
- n9 (M9), no occupancy term: `(0, 0)` against `(2, 0)`.

**The reviewer's attempt-3 mutation table** (n1-n8 rerun as M1-M8, plus M9-M23), in summary:
- **Red through the control-plane order test:** M1-M4, M8, M10, M23.
- **Red through the `should_panic` test:** M6 and M7 (both checks removed; follow, or base's
  stall), and M9 (no occupancy term). M9 is now also red in `protocol`, as above.
- **Red through the #2198 slot test and the order test:** M12, M12b, M16, M17. M17 also turns the
  `warm_successor` tests red.
- **Red only in capi:** M14 (the slot freed before the encode's error check), through
  `runtime::tests::all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes`.
- **Green, each equivalent or out of scope:**
  - M5, the entry invariant check removed: the `Ok(None)` arm's check fires with the same message.
  - M13, `sample_time`/`provider_sequence` not reset after the encode: they are written again
    before every encode, and nothing else reads them.
  - M15, the provider's `DIAGNOSTICS_GET` projection not cleared: a non-goal.
  - M18, the `render_diagnostic_len != 0` guard dropped: a `Free` head and `len == 0` coincide.
  - M19, the `Ok(None)` arm returning `Ok(None)` while the entry check remains: the arm is
    unreachable while the invariant holds, and the entry check stops a debug build first.

## Evidence (attempts 2 and 3)

Attempt 3 (this head) extends both cases of the order test. It also documents the counter's
saturation in the field doc and the accessor doc.

**Tests** (`crates/control-plane/src/control.rs`, `control::tests::rendered_meters`, under
`control-plane/test-support`):
- **`each_render_diagnostic_goes_after_exactly_the_protocol_events_older_than_it`.** This
  replaces attempt 1's test and has two cases.
  - **The review's repro** (defect 2, through the public API, and defect 1's short-buffer path):
    an apply, then a short-buffer dequeue, then block 0. Then a second short-buffer dequeue
    (attempt 3: while `SESSION_COMMITTED` is still pending, after the diagnostic was staged), then
    block 1. It drains two events, then sends `TRANSPORT_SET`, then drains the rest. The order must
    be committed, diagnostic, diagnostic, transport.
  - **An injected `EmptyResponse` at the head** (defect 1's non-short-buffer error). After a
    block, the next dequeue must fail again rather than deliver the diagnostic. Attempt 3 added a
    second dequeue here. Attempt 4 removed it: the counting rule it checked is now tested once,
    in `protocol`, so this case keeps only the FIFO claim.
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
- **The order test** goes red if a diagnostic's target misses the pending event, if a second
  staged diagnostic waits for events that go before the first, or if a failed dequeue counts as a
  delivery. In each case a diagnostic is
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
- **n8** (attempt 3; the attempt-2 review's mutation), the increment moved out of
  `if result.is_ok()` so that a failed dequeue counts. The order test is red:
  `left: Some("diagnostic") right: Some("committed")`, from the short buffer after staging.
  - With that short buffer removed for the check (and n8 still applied), the injected case's added
    second dequeue is red on its own: `Ok(Some(144))`. Each attempt-3 addition catches n8.
  - The whole n1-n8 set was re-run at this head with the same results.

**No bit moved.** Not applicable, because no render-path code changed. `stage_render_diagnostic`
and `dequeue_reliable_event` run in the control thread's service step and egress.

**Cross-thread.** None. The target, the slots and the controller's counts are all owned by the
control thread, so no loom model is needed.

The red-on-base check was re-run with attempt 3's tests, and gave the same two failures.

**Gates (attempt-4 head):**
- `cargo test --locked --all-targets -p protocol -p control-plane --features protocol/test-support,control-plane/test-support`:
  23 test binaries, 317 tests, all pass.
- `cargo test --locked --doc` for both crates: pass.
- `cargo clippy --locked -p protocol -p control-plane -p capi -p host-web --all-targets --all-features -- -D warnings`:
  clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p protocol -p control-plane`: clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

**Gates (attempt-3 head):**
- `cargo test --locked --all-targets -p protocol -p control-plane -p capi -p host-web --features protocol/test-support,control-plane/test-support,host-web/test-support,host-web/render-lock-probe,host-core/test-support`:
  57 test binaries, 745 tests, all pass.
- `cargo clippy --locked -p protocol -p control-plane -p capi -p host-web --all-targets --all-features -- -D warnings`:
  clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p protocol -p control-plane`: clean.
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
