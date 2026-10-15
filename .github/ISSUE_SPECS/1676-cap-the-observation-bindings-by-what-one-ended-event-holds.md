# Cap the observation bindings by what one ended event holds

Successor of #1584 D6a (batch 12a, root 2026-10-15), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `45a2ab9df`
(PR #1675). Issue number: assigned at filing (written here as #1676). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A subscription the engine acknowledges can always be ended honestly: no commit that removes a
subscribed target is refused because its `OBSERVATION_ENDED` event would not fit one control
frame, and the memory reserved for that event follows the bindings the session can hold, not the
frame size.

## Context

- #1584 D6a ends every binding whose target is gone with one `OBSERVATION_ENDED` per commit
  (root ruling A). The event must fit `maximum_control_frame_bytes`; a commit whose ended keys do
  not fit is refused as a resource limit (`crates/control-plane/tests/observation_bindings.rs`,
  `an_end_event_over_one_control_frame_refuses_the_removal_and_changes_nothing`). So a valid
  removal can be refused, and the app must disarm first.
- The batch 12a review fold charges each `OBSERVATION_ENDED` cell a flat ceiling bounded by the
  frame size (`ended_event_ceiling`, `crates/control-plane/src/compile.rs:572`;
  `observation_reserve_bytes`, `:553`). The reserve grows with `maximum_control_frame_bytes`
  (about 4x at the browser's 16 KiB frame against the 4 KiB reference); `capi_retained_bytes`'
  ceiling rose from 181,696 to 205,312 with it.

## Decisions frozen for this slice

- **D1. Cap at admission.** A subscribe (an Observe that would add a binding) is refused with a
  typed refusal before any ack when the bindings table already holds as many keys as one ended
  event can carry in one control frame (computed once at preparation from the frame limit and the
  largest encoded key the session's IDs allow). Nothing is queued and no state moves.
- **D2. Reserve by capacity.** The ended-event reserve per cell is the smaller of the frame bound
  and the encoded size of the bindings capacity (the slots the plan has), so a session with few
  observation slots reserves little whatever the frame size.
- **D3. Same on both hosts**, with a parity row.

## Objective gates

1. With a small `maximum_control_frame_bytes`, the subscribe that would exceed the cap is refused
   with the typed code, and a removal of every subscribed target afterwards commits with one event
   holding every key (the over-frame refusal can no longer be reached by a valid removal).
2. `capi_retained_bytes` on the reference session falls back toward its pre-fold figure at the
   4 KiB frame, and the reserve no longer scales with the frame for a plan with few slots; the
   ceiling is re-pinned to the measured value (plus 10 %, rounded up to 64) at both lane widths.
3. The charge-equals-retained capi tests stay exact; `cargo test --locked -p control-plane
   --features test-support -p capi -p host-web --features host-web/test-support`; clippy.

## Authorized paths

- `crates/control-plane/src/{compile.rs,control.rs}`, `crates/controls/src/observation.rs`,
  `crates/protocol` (the refusal code, if a new one is needed), their tests,
  `crates/capi/tests/resource_lifecycle.rs` (the ceiling), docs, this spec's decision record.

## Dependencies

- #1584 D6a (PR #1675).

## Size

Half a day.

## Decision record: #1676 (batch 13b)

**What shipped.**

- **D1, the cap.** `control_plane::compile::observation_binding_cap(frame_bytes)` measures, with the
  real codec, the `OBSERVATION_ENDED` frame of one key and what a second adds, with both IDs at
  `STABLE_ID_MAXIMUM_BYTES` (127), and returns how many keys one control frame holds (1 KiB frame:
  3, 4 KiB: 12, the browser's 16 KiB: 51). `SessionState` keeps it (`observation_cap()`), with an
  exact `observation_armed()` count. Each observation slot now carries `armed` (the table's state
  with every push folded in) beside its pending push, so a subscription is judged without a key
  lookup or an allocation inside `process()`. A rebuild rebuilds the slots from the bindings it
  keeps (`observation_slots(producers, bindings)`), so the count survives a swap and drops the
  taps a transaction ended. `SessionState::admit_observation_changes(changes)` admits a batch by
  the taps it leaves armed (arms of unarmed taps add, disarms of armed taps remove, a window change
  adds nothing) and returns `ObservationLimit { index, cap }` for the first arm that crosses. It
  moves nothing, so a refusal is whole and precedes every ack.
- **Browser mapping.** `admit_commands` calls it after pass one and before the room pass; a
  refusal is `RESULT_BACKPRESSURE` / `COMMAND_REASON_BACKPRESSURE` at the record that crossed it
  (doc comments on the Rust constant and both `.d.ts` copies say so). Nothing else in the adapter.
- **D2, the reserve.** The ended-event heap per cell is now `min(slots, cap)` keys times
  (`size_of::<ObservationTapKey>()` + two maximum IDs), not the frame size. The binding table and
  its transient copies use the same `min(slots, cap)`.
- **Charge by high water.** The slots in that charge are the most any committed plan of the session
  has had (`observation_slots_high_water`, passed to `prepare_runtime`), not only the plan being
  prepared. A cell holds an event until its frame is delivered, so an event a larger earlier plan
  emitted can outlive that plan; charging the shrunk plan by its own slots would let a cell hold
  more than its row says. The first plan charges its own slots, so the reference figures are exact.
- **Docs.** `docs/CONTROL_PROTOCOL_SEMANTICS.md` (observation paragraph) states the cap and its
  figures.

**Departures.**

1. **Refusal code.** The browser's `COMMAND_REASON_*` vocabulary is gate-pinned as contiguous from
   0 across six files, and 12-14 are retired and never reused; a new reason would need a vocabulary
   decision. The typed refusal is `ObservationLimit` in the core; the browser maps it to the
   existing backpressure reason (unsubscribe a tap, then retry succeeds). No `crates/protocol`
   change and no re-pin. A dedicated reason is successor work if the owner wants one.
2. **Parity row.** Only the browser subscribes today (the C ABI's `Observe` edit is #1584 D1-D5 and
   `observation_taps` stays 0 there), so there is no C ABI path to put in
   `hosts/host-web/tests/control_parity.rs`. Both hosts share the one core cap, the one reserve
   and the one slot state; the C ABI edit must call `admit_observation_changes` (or its own edit
   check against `observation_cap()`).
3. `push_effect_observe` does not itself refuse past the cap: a batch can push its arms before its
   disarms, so the cap is judged on the batch, by `admit_observation_changes`, which every caller
   must call first. The over-frame refusal in the commit stays as a defence; its existing test now
   says it pushes past the cap on purpose.

**Gates and tests.**

- Gate 1: `control-plane/tests/observation_bindings.rs`
  `a_subscription_past_the_cap_is_refused_and_the_removal_of_all_still_commits` (small frame; the
  cap+1 arm is refused with index and cap, a disarm in the batch makes room, re-arming an armed tap
  adds nothing, admission moves nothing, and removing the strip commits with one event holding all
  `cap` keys), `the_cap_is_what_one_frame_carries_with_the_longest_ids` (cap longest keys fit the
  frame, one more does not, at 1/4/16 KiB), `the_armed_count_follows_the_bindings_across_a_rebuild`;
  browser: `host-web` `a_subscription_past_the_cap_is_refused_and_an_unsubscribe_makes_room`.
- Gate 2: `the_reserve_follows_the_slots_and_a_shrunk_plan_keeps_the_larger_reserve` (the reserve
  at 4 KiB equals the reserve at 16 KiB for a few slots; a plan that shrank from 14 to 8 slots
  charges at least the larger reserve). `capi_retained_bytes` on the reference session measured
  165,782 (x86-64, 4 KiB frame; the row does not depend on the lane width; it was 186,614 after the
  12a fold and 165,758 before it), so the ceiling is re-pinned to 165,782 plus 10 %, rounded up to
  64: 182,400, at both widths (was 205,312). The four-lane figure is not measured here (no AArch64).
- Gate 3: `capi` `resource_lifecycle` (charge equals retained, exact) and the full crate suites
  pass; clippy `-D warnings` on control-plane, controls, capi and host-web, all targets and
  features.

**Successor candidates.** A dedicated browser command reason for the cap (needs a vocabulary
decision). The C ABI `Observe` edit must admit against `observation_cap()`. 51 simultaneous taps at
the browser frame is the product limit; a larger frame raises it linearly.
