# Report each configured counter's own value in the C ABI counter snapshot

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0: the no-shortcuts principle).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The periodic `COUNTER_SNAPSHOT` event (`0x8021`) a C ABI host configures with
`TELEMETRY_CONFIGURE` reports each configured counter's own value, the same value
`COUNTERS_GET` returns for it, once per configured period of rendered blocks. Configuring a
counter the C ABI does not serve is refused with a type.

## Context

- **The defect.** `collect_render_activity` stages one snapshot per control call that sees a new
  render, and gives every configured counter the value `sequence`, the render-call count
  (`crates/capi/src/runtime/control.rs:558-575`: `CounterValue { id, value: sequence }`). A host
  that configures `TELEMETRY_DROPPED` reads the number of render calls. `counter_period_blocks` is
  checked only for being nonzero (`crates/protocol/src/controller.rs:2492-2497`).
- **The real values.** The provider answers `COUNTERS_GET` from `counter_snapshot`
  (`crates/host-core/src/control_provider.rs:345-367`), filled lazily by `set_counter` (`:597`):
  `TELEMETRY_COALESCED` and `TELEMETRY_DROPPED` from `set_telemetry_counters` (`:251-262`, called
  by `command` only, `control.rs:850-853`) and `CANCELED_AUTOMATION` from
  `record_canceled_automation` (`:369`). Before its first write a counter is absent, so
  `COUNTERS_GET` answers `NOT_FOUND` for it.
- **Configuration is never refused.** `ControlProvider::telemetry_configure` returns the
  configuration with no error path (`controller.rs:598-603`); the controller stores whatever it
  echoes (`:3059-3075`).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348) gives the control
  plane one service step, `SessionState::service` (D1: `synchronize_plan_epochs`, then
  `collect_render_activity`), which every control call runs first (D2).
- The snapshot event is lossy telemetry by contract (`docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  "Automation and events").
- **The registry.** `CounterId` is the frozen registry in
  `crates/protocol/src/message_wire.rs:622-642` (it ends at `ValidationFailures = 15`);
  `parse_counter_id` (`:2032-2051`) decodes it. *Hold live values in latest-target cells on both
  hosts* (#1312 D8) adds `LiveValuesSuperseded = 16`.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397) keeps a session counter, `transition_reprepare_refusals`:
  transition re-preparations that were refused anyway and retried at the next service step.
  *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* (#1360)
  copies it into the provider and names it in the header. The C ABI needs a counter ID to serve
  it.

## Decisions frozen for this slice

- **D1. A fixed served set.** The C ABI provider serves a fixed counter set from construction,
  each at 0 until written: `TELEMETRY_COALESCED`, `TELEMETRY_DROPPED`, `CANCELED_AUTOMATION`,
  `TRANSITION_REPREPARE_REFUSALS` (D6), and `LIVE_VALUES_SUPERSEDED` once #1312 has added it.
  `COUNTERS_GET` with `all` returns all of them.
- **D2. Typed refusal of unserved counters.** `ControlProvider::telemetry_configure` returns
  `Result<TelemetryConfiguration, ParameterProviderError>`. A configuration naming a counter ID
  outside the served set is `ParameterProviderError::NotFound`, which the controller answers with
  `StatusCode::NotFound`; the stored configuration does not change. Meter handles use the same
  path in #1352.
- **D3. Own values, refreshed by the service step.** The telemetry counter refresh of
  `control.rs:850-853` moves into `SessionState::service` (#1348 D1), before
  `collect_render_activity`, so every control call refreshes the provider's counters once, not
  only `command`. The snapshot's values are, for each configured ID in order, the provider's
  current value for that ID, read after that refresh in the same service step.
- **D4. Cadence.** The control plane stages a snapshot only when render has completed at least
  `counter_period_blocks` blocks since the last staged one (the render sequence counts blocks),
  with `observed_sample` the render sample at that point. Blocks rendered with no control call in
  between yield one snapshot at the next call; the lane is lossy, so skipped periods are not
  replayed.
- **D5. The acked-batch question** does not arise: the event is noncritical telemetry. A refused
  configuration changes nothing.
- **D6. `TRANSITION_REPREPARE_REFUSALS`.** `CounterId::TransitionReprepareRefusals` joins the
  frozen registry and `parse_counter_id` at the next free value when this merges (17 if #1312's
  16 has landed), never a reused one; tests use the name, never the number. The provider serves
  it at 0 from construction. #1360 writes it in the service step's counter refresh. It counts
  `transition_reprepare_refusals` (D15-17).

## Deliverables

1. D1 in host-core's provider, D2 in the protocol's provider trait and controller, D3-D4 in the
   control plane's `collect_render_activity`, D6 in the protocol's registry.
2. `docs/C_ABI_V1_QUALIFICATION.md`: the served counter set and the cadence.

## Authorized paths

- `crates/control-plane/src/control.rs` (`collect_render_activity` and the refresh in `service`
  only).
- `crates/host-core/src/control_provider.rs` (the counter set and `telemetry_configure`).
- `crates/protocol/src/controller.rs` (the trait signature and the `TelemetryConfigure` arm),
  every in-repo `ControlProvider` implementation for the new signature.
- `crates/protocol/src/message_wire.rs` (`CounterId` and `parse_counter_id` only, D6).
- `crates/capi/src/runtime/tests.rs`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- The meter batch (#1352). New counters other than D6's (each comes with its own issue, e.g.
  #1312). Writing a nonzero `TRANSITION_REPREPARE_REFUSALS` (#1360).

## Objective gates

1. **Own values (new capi test).** Configure `[TELEMETRY_COALESCED, TELEMETRY_DROPPED]` with
   period 1; render blocks with control calls between them until the lossy lane has dropped or
   coalesced at least one record. Every staged snapshot's values equal `COUNTERS_GET` for the same
   IDs at that call, and differ from the render-call count at least once. One of those calls is a
   `source_submit`, not a command, so a refresh limited to `command` turns this red.
2. **Cadence (same file).** Period 4: three rendered blocks and a control call stage nothing; the
   fourth block and a call stage one snapshot.
3. **Refusal (same file).** Configuring `MALFORMED_FRAMES` (7), which the C ABI does not serve,
   answers `NOT_FOUND`; `COUNTERS_GET` and the stored configuration are unchanged.
4. **Served at 0 (same file).** On a fresh session, `COUNTERS_GET` for
   `TRANSITION_REPREPARE_REFUSALS` answers 0, not `NOT_FOUND`, and configuring it is accepted; its
   ID round-trips through the wire encoder and `parse_counter_id`.
5. **Superseded oracle bytes.** `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
   (`crates/capi/src/runtime/tests.rs:1857`) pins a counter snapshot built from the render
   sequence; re-pin those wire bytes with this issue as the reason.
6. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p protocol --features
   test-support`; `cargo test --locked -p host-core --features control-provider,test-support`;
   `bash scripts/check-protocol-control-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a snapshot filled with anything but the counter's own value (today's defect).
- Gate 2: a snapshot cadence that ignores the configured period.
- Gate 3: a configuration accepted for a counter that can never report.
- Gate 4: an ID missing from the decoder or the served set makes a host's read of the counter
  `NOT_FOUND` or its configuration refused.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the service step
  that refreshes the counters (D3).
