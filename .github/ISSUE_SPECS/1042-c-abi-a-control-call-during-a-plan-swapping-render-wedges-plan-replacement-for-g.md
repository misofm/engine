# C ABI: a control call during a plan-swapping render wedges plan replacement for good

## Problem (pre-existing, found by the #1020 verification)

`RealtimePlanOwner::enter_block` swaps plans and commits the retired plan to the retirement queue at the *start* of a render call (`crates/engine/src/realtime/plan_exchange.rs:361-424`), but capi publishes `active_epoch` only after the render call returns (`crates/capi/src/runtime/plan.rs:215-216`). A control call in that window reaches `synchronize_plan_epochs` (`crates/capi/src/runtime/control.rs:619-661`), sees `active_epoch == providers.epoch`, does not promote, reclaims the retired plan, finds no retired provider for its epoch and returns `Internal`; it also skips removing that epoch's report row because it compares against the stale atomic. On the next call the stale provider is pushed into `retired_providers` and never leaves, the report table stays full, and every later structural command returns `Backpressure`.

Reproduced on the unmodified base by a deterministic split-render test (`Err(Internal)`, then `retired [0]` forever, then permanent `Backpressure`) and by a two-thread test racing edits against a structural swap (14 of 20 runs). Every control entry point synchronizes (submit, seek, command, event), so a mobile app that feeds PCM from a decode thread while the audio callback renders can hit it on the first structural edit during playback. Evidence: `docs/handoffs/live-control-2026-09-28/VERIFY.md`, finding F1. This is the mobile playback surface (`docs/rulings/engine-footprint-2026-09-28.md`), so it is a product bug, not a cleanup item.

## Smallest closable slice

In `synchronize_plan_epochs`: a reclaimed epoch equal to `providers.epoch` proves that the one pending candidate is now active, so promote it there; then keep only the report rows that the lagging atomic, the current epoch or a pending epoch can still read. (The verifier's scratch fix is +25/-14 lines.) Commit the deterministic split-render test and the two-thread swap test with it.

## Objective gates

1. The deterministic split-render test is red on the base and green with the fix.
2. The two-thread swap race passes 20 of 20 runs (and a larger count in release) with the fix; it fails on the base.
3. No ack precedes a drop: the failing call acks nothing before or after the fix; every existing capi test, the `resource_lifecycle` oracles and `audit capi` (zero allocations, locks and syscalls on render) pass.
4. The browser is unaffected: host-web does not use plan replacement; the shipped AudioWorklet artifact bytes are unchanged.
5. Lands before the live-control slice (L1), which the #1020 findings describe.
