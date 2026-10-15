# Expose the armed tap mask from host-core

Successor of the batch 12b review (m5), decision 17. Code anchors verified on the batch 12b branch `codex/controls-batch-12b`. Issue number: assigned at filing (written here as #1679). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

The browser copies a successor's armed taps from host-core instead of re-deriving host-core's
match, so one function decides which tap a binding arms.

## Context

- #1584 D6b's `build_plan_shape` (`hosts/host-web/src/lib.rs`, near `:5768-5790`) re-derives the
  match host-core makes between bindings and taps (`crates/host-core/src/prepare.rs`, near
  `:2271-2283`, `ObservationArm`).

## Decisions frozen for this slice

- **D1.** host-core's `EffectObservationHandle` exposes the armed tap mask it prepared; the
  browser's table copies it; the duplicate match is deleted.

## Objective gates

1. The #1584 D6b browser tests pass unchanged; a test shows the browser's arm bits equal host-core's
   mask for a session with several bindings, including an insert ID reused for another effect.
2. host-core and host-web tests; clippy.

## Dependencies

- #1584 D6b.

## Size

A few hours.

## Decision record: expose the armed tap mask (batch 13a)

- `EffectObservationHandle` is defined in `crates/effect-compiler/src/prepare.rs` (host-core re-exports it), so the field `armed_taps: u32` (bit `i` = `descriptor.observations[i]`) was added there. `attach_effect_observation` sets a bit exactly when `ObservationLane::arm_prepared` armed that tap, so the mask is the lane's own arming, not a second match.
- host-web's `build_plan_shape` copies `handle.armed_taps` into `observation_armed`; the duplicate binding match and its `bindings` parameter (and the caller's lookup) are deleted. An arm for a tap past bit 31 has no bit, as before.
- Gate: `tests::structural_apply::the_browsers_armed_bits_are_the_mask_host_core_prepared` (a console slot `comp` and insert `comp` on two tracks, one insert and one console tap armed, restart rebuild; every slot's word equals the handle's mask; 4 observed, 2 armed). It goes red when the copy is replaced by 0. The #1584 D6b browser tests pass unchanged.
