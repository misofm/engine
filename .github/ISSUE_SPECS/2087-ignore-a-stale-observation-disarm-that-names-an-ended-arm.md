# Ignore a stale observation disarm that names an ended arm

Filed by root (backlog sprint, 2026-10-25). Source: the #1590 attempt-1 verification, finding M5.

## Problem
The core shares one observation arm per (tap, window) and keeps a subscriber count with no subscriber identity. In the browser, OBSERVATION_ENDED events reach the SDK by message, so they can arrive late. This sequence silences a live handle:
1. The core ends handle A's arm (for example, the tap's effect is removed).
2. The tap is added again, and handle B arms it with the same window. The core's count is 1.
3. A closes before its OBSERVATION_ENDED event arrives. A's disarm lowers B's count to 0, and B's rows stop.

The SDK cannot tell the two arms apart, because the disarm names only the tap and the window. On main before #1590 the case was worse: B joined A's dead binding and sent no arm at all. #1590 made the SDK send every edit itself, which leaves this residual.

## Smallest slice
A disarm carries the arm tag that its handle received (`ObserveApplied` already returns the core's tags). The core applies a disarm only when its tag equals the live arm's tag. A disarm with a stale tag is a no-op that is reported in the reply, never an error and never a decrement. One protocol edit, the SDK sends the tag, both hosts share the core path.

## Gates
- One control-plane test: arm, end, re-arm with the same window, then a disarm with the old tag. Red if the count drops or the rows stop.
- The SDK test for the browser order above (end event delayed past the close): red if B's rows stop.
- Schema regeneration and the protocol registry row; no ABI_VERSION bump unless the frame layout needs it (state which).
- Acked-batch question: a stale disarm is acked as a no-op only after the core has decided it; no record is dropped.

## Paths
To be set by the scope note: `crates/control-plane/src/control.rs`, `crates/protocol` (OBSERVE edit word), `sdk/src/core/observation-subscriptions.ts`, their tests, the generated schema and docs/CONTROL_PROTOCOL_REGISTRY.md.

## Decision record
- Scope note (`scope/2087.md`, read at 18c92e5aa, now main's content) sets the paths; root rulings of 2026-10-25:
  - A disarm carries its handle's arm tag. A tag equal to the live binding's tag applies the disarm as before (a window mismatch stays `observation.not_subscribed`). Tag `0`, or a tag at or above the host's `next_arm_tag` (never issued), is `observation.not_subscribed`. Any other tag commits as a no-op: nothing is written, no record is queued, and the reply reports the binding the transaction began with (`0` when unbound). One place: `ObservationBindings::observe` (`crates/controls/src/observation.rs`), fed the edit's tag and `ObserveContext::next_arm_tag` by the prepare loop (`crates/protocol/src/model.rs`).
  - Wire: `0801` gains `5: arm-tag U64 O`, present exactly on a disarm (its value, `0` included, is the core's to judge). An arm with field 5 or a disarm without it is `INVALID_FIELD` on encode and decode; prelaunch, no compatibility path. No frame-layout, capability-flag or `ABI_VERSION` change. `crates/protocol/tests/fixtures/observation-frames.json` re-blessed: `observe_window_change` and `observe_padded` changed bytes (they hold disarms); `observe_arm_insert` and the conformance corpus's `Observe` (an arm) did not.
  - The SDK sends the tag from `ObserveApplied` on every disarm (`close` and `update`), from the handle's own binding.
- The no-op also removes the second variant (A's disarm after its end and before the event, no re-arm): it commits instead of a spurious `MisoUsageError`.
- Known limit, not changed: the target check (`observe_target`) still runs first, so a disarm whose effect is removed and not yet back is `observation.address.unknown`; and a second disarm with the same issued tag is now a silent no-op, not `observation.not_subscribed`.

## Evidence (attempt 1)
- Control plane: `a_disarm_that_names_an_ended_arm_changes_nothing` (`crates/control-plane/tests/observation_bindings.rs`) replaces the tail of `a_removed_effect_ends_its_binding_with_an_event` (#1584 gate 4). Red with the tag check removed from `observe` (main's behaviour): the first stale disarm is refused `observation.not_subscribed`. Red with only the stale-tag no-op removed: B's arm is disarmed (reply tag 3, the disarm record, not B's 2).
- SDK: `structure-evals.mjs`, "a close behind a late end event does not silence a later subscriber of the tap". Red on a wasm built with the core's tag check removed: B's row reads `unarmed`.
- Acked-batch: the no-op is decided at preparation, before anything is written; it queues nothing, so no ack precedes a drop.
