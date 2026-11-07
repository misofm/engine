# Control plane: the render-diagnostic egress mirrors the controller's pending reliable event, so a diagnostic can overtake it

Tracker #2258, item PR-4. Found by the implicit-state-machine audit of 2026-11-06.

## Problem
The control plane's reliable egress interleaves its own render-activity diagnostics
(`capi.render.activity`) with the protocol controller's reliable events. Each staged diagnostic
carries a barrier, `protocol_events_before`: the number of protocol events it must wait behind.

`ProtocolController::dequeue_reliable_event_frame_into` (`crates/protocol/src/controller.rs`) pops
an event into its private `pending_reliable_event`, and keeps it there after *any* error until a
complete frame reaches the caller. The control plane cannot see that cell, so on base `de0878e23`
it mirrored it in `SessionState::protocol_reliable_pending: bool`
(`crates/control-plane/src/control.rs:1333`). `dequeue_reliable_event` (`control.rs:7506-7570`)
set the mirror only from `Err(Encode(OutputTooSmall))` and cleared it on every other error.
`stage_render_diagnostic` (`control.rs:3192-3197`) computed the barrier as queue occupancy plus the
mirror. After any other dequeue error, the controller still held the event, but the barrier missed
it, and the next diagnostic overtook an older protocol event. That breaks reliable FIFO.

The staged slot was also described twice: `occupied: bool` and `reservation: Option<_>` said the
same thing, and its barrier and revision outlived a free slot (`control.rs:2590-2596`).

## Reachability today
Not reachable through the public API. Every error that leaves the event pending, other than a short
buffer, is a broken invariant of the controller:
- `Encode(LimitExceeded | InvalidField)`. Every enqueue checks first that its frame encodes:
  `enqueue_diagnostic_event` calls `encoded_diagnostic_event_len`, and both
  `enqueue_response_ready_event` and `attach_observation_ended` make a probe encode. The
  `SESSION_COMMITTED` and `TRANSPORT_STATE` payloads are fixed-size scalars, far below any frame
  limit. The codec's limits do not change after boot.
- `Encode(MessageKindMismatch)`. Only an `EmptyResponse` slot in the event queue gives this, and no
  enqueue makes one.
- `ObservationEndedCellMissing`, `ResponseReadyMissing`, `DiagnosticStorageFull`. The backing cell
  is filled with the slot's enqueue, and emptied only after that slot's frame is delivered.

So the desync takes a controller defect, or a future error path. The proof of the desync is the
test below, which puts an event that cannot encode (`EmptyResponse`) at the head of the queue. The
mirror is still the wrong model: it copies another component's state and can only be kept right by
reading every error the way the controller does (tracker pattern 2).

## Decision
- **The controller owns the count.** `ProtocolController::reliable_backlog()` returns the reliable
  events it holds whose frames have not reached caller output: queue occupancy, plus one for the
  pending event. The mirror is deleted, and the barrier is `reliable_backlog()` when the diagnostic
  is staged.
- **One slot state.** `RenderDiagnosticSlot { diagnostic, state }` with
  `enum DiagnosticSlotState { Free, Staged { _reservation, protocol_events_before, revision } }`.
  The reservation, barrier and revision exist only while a diagnostic is staged. Freeing the slot
  drops the reservation. The diagnostic text stays in the slot's preallocated storage.
- **Egress.** If the head slot's barrier is nonzero, the controller's event goes first. An error
  leaves it pending and first. Only a delivered frame lowers the barrier. The barrier never exceeds
  the controller's backlog, so the controller always has an event to give. Base returned `Ok(None)`
  with a diagnostic still staged if it ever had none. Now, if the controller is empty, nothing
  older remains and the diagnostic follows in the same call. This cannot happen while the
  accounting is exact.
- **Kept in scope.** The slot enum is in the same two functions as the barrier, and the barrier is
  the FIFO mechanism. Folding it into `Staged` means a free slot has no barrier at all. It is part
  of the same outcome, FIFO-correct reliable egress.

## Non-goals
- The provider's `DIAGNOSTICS_GET` projection (`ControlProvider::set_render_diagnostic`,
  `crates/host-core/src/control_provider.rs`). It serves a different reader, the history query,
  and is not a copy that egress reads.
- The ring's head/len.
- The wire and rendered output. Everything here runs in the control-thread service and egress, not
  in render.

## Gates
- A regression test red on base, green at head, with mutations of the fix turning it red.
- `cargo test --locked` for `protocol`, `control-plane`, and the egress consumers `capi` and
  `host-web`.
- `cargo clippy --locked -p protocol -p control-plane --all-targets -- -D warnings`, with and
  without `control-plane/test-support`.
- `cargo fmt --check`, `bash scripts/check-workspace-policy.sh`, and the realtime policy tool.

## Evidence (attempt 1)

**Test.**
`control::tests::rendered_meters::a_render_diagnostic_waits_behind_the_event_the_controller_holds_pending`
(`crates/control-plane/src/control.rs`, under `control-plane/test-support`). It has two cases:
- A short buffer leaves `SESSION_COMMITTED` pending. A block then stages a diagnostic, and a newer
  `TRANSPORT_STATE` follows. The drained order must be committed, diagnostic, transport. This is
  the reachable path, and it guards the accessor's pending term and the barrier's decrement.
- An `EmptyResponse` slot injected at the head fails with `Internal` and stays pending. A block
  then stages a diagnostic. The next dequeue must fail again, because the pending event still heads
  the lane. It must not deliver the diagnostic.

**Red on base `de0878e23`** (the test alone, with only the `EventError` import added):
```
thread '...a_render_diagnostic_waits_behind_the_event_the_controller_holds_pending' panicked at crates/control-plane/src/control.rs:9166:13:
the pending event still heads the lane, so the diagnostic waits: Ok(Some(144))
test result: FAILED. 0 passed; 1 failed
```
The 144 bytes are the render diagnostic's frame, which overtook the pending event. Case 1 passes on
base, because the mirror covers a short buffer. At head both cases pass.

**Test value.** It goes red if a diagnostic's barrier misses the event the controller holds
pending. That happens if the barrier counts only the queued events, or learns of the pending event
only from a short-buffer error (the deleted mirror). It also goes red if a delivered event fails to
lower the barrier. Either way, a diagnostic is reordered against a protocol event. No existing test
stages a render diagnostic while a protocol event is pending: the #2198 slot test and the capi
egress tests drain with a roomy buffer.

**Mutations** (each applied alone to head, then the test run):
- m1: `reliable_backlog` without the pending term. Red at the first `committed` assertion: the
  diagnostic came first.
- m2: an egress error falls through to the diagnostic, as `Ok(None)` does. Red with `Ok(Some(144))`.
- m3: a delivered protocol event does not lower the barrier. Red at the `diagnostic` assertion:
  `TRANSPORT_STATE` overtook it.

**No bit moved.** Not applicable: no render-path code changed. `stage_render_diagnostic` and
`dequeue_reliable_event` run in the control thread's service step and egress.

**Cross-thread.** None. The barrier, the slots and the controller are all owned by the control
thread, so no loom model is needed.

**Gates (head).**
- `cargo test --locked --all-targets -p protocol -p control-plane -p capi -p host-web --features protocol/test-support,control-plane/test-support,host-web/test-support,host-web/render-lock-probe,host-core/test-support`:
  57 test binaries, 744 tests, all pass.
- `cargo clippy --locked -p protocol -p control-plane --all-targets --all-features -- -D warnings`:
  clean. With the same features, adding `-p capi -p host-web`: clean.
- `cargo clippy` without `control-plane/test-support` fails on base too:
  `compile_children_warm` is `#[cfg(test)]`, but only test-support tests use it. This change
  did not touch it, and CI runs clippy with `--all-features`.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.

## Follow-ups
- The provider's `DIAGNOSTICS_GET` projection mirrors the staged slots through
  `set_render_diagnostic(slot, sample, sequence, live)`. That is a second writer of the same
  facts. A provider view that reads the slots directly would remove it, but it crosses into
  `host-core`.
