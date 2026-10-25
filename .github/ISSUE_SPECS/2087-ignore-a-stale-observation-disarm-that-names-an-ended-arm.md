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
