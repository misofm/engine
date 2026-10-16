# Ask for a control queue slot only when a declared discontinuity publishes

Filed by root from ruling B16-R10 (batch 16b, 2026-10-16).

## Product outcome

A declared discontinuity that publishes nothing never refuses with backpressure on either host.
That covers the standalone declaration (`miso_engine_v1_declare_discontinuity`,
`miso_engine_web_v1_declare_discontinuity`) and the plain session seek, which declares one in the
same call (B16-R9). Only a declaration that publishes a successor asks for room in a host's
control queue, and the core decides whether it publishes before any host asks for that room.

## Context (anchors on `cebbf0df4`)

- The core decides whether a discontinuity successor is needed with #1323 D2's two terms: a floor
  above a node's natural arrival, or a source-read offset above 0
  (`PlanStateInventory::runs_behind_natural_timing`, `crates/host-core/src/prepare.rs:1022`).
  When neither holds, `SessionState::declare_discontinuity` (`crates/control-plane/src/control.rs:2154`,
  serviced body `:2161`) returns `Ok` and publishes nothing.
- The browser runs both calls through `AudioWorkletEngineHost::run_control`
  (`hosts/host-web/src/lib.rs:1737`). It refuses with its `full` error when the pending-shapes table
  queue is full (`:1745-1747`), before the core runs, so before the core knows whether anything
  would publish. The declaration (`:1695`) and the plain session seek (`:1713`, `full` =
  `SessionSeekFailure::Declaration(CommandError::Backpressure)`) both inherit that refusal.
- The C ABI has no such queue, so the same call returns `OK` there. That is a one-host difference
  (decision 17 C1: browser and C ABI behave the same).

## Decisions to make at filing

- D1. The core exposes the decision before the publish: for example a `SessionState` query
  "would a declaration publish now?" (the two #1323 D2 terms on the newest plan), or a
  declaration API that takes the host's room as a parameter and refuses with backpressure only when
  it would publish. Pick one; one core rule, both hosts.
- D2. `run_control` asks for a slot only when the core says the call publishes; a declaration or
  plain session seek that publishes nothing returns `OK` on a full queue, as on the C ABI.
- D3. Parity row in `hosts/host-web/tests/control_parity.rs`: with the browser's table queue full,
  (a) a standalone declaration and (b) a plain session seek that publish nothing return `OK` on
  both hosts and the seek applies; (c) ones that would publish (floor raised on the #1285 path)
  refuse with backpressure on the browser and nothing changes. Red if the browser asks for the slot
  first again.

## Dependencies

- #1642 and #1643 (B16-R9's plain session seek).
