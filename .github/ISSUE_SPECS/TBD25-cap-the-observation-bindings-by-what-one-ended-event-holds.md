# Cap the observation bindings by what one ended event holds

Successor of #1584 D6a (batch 12a, root 2026-10-15), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `45a2ab9df`
(PR #1675). Issue number: assigned at filing (written here as TBD25). Feature sprint mode (owner,
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
