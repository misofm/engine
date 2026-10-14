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
  (`crates/control-plane/src/control.rs:661-678`: `CounterValue { id, value: sequence }`). A host
  that configures `TELEMETRY_DROPPED` reads the number of render calls. `counter_period_blocks` is
  checked only for being nonzero (`crates/protocol/src/controller.rs:2492-2497`).
- **The real values.** The provider answers `COUNTERS_GET` from `counter_snapshot`
  (`crates/host-core/src/control_provider.rs:345-367`), filled lazily by `set_counter` (`:597`):
  `TELEMETRY_COALESCED` and `TELEMETRY_DROPPED` from `set_telemetry_counters` (`:251-262`, called
  by `command` only, `control.rs:986-989`) and `CANCELED_AUTOMATION` from
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
  `StatusCode::NotFound`; the stored configuration does not change. Meter tables use the same path in #1583 (decision 17 amendment, 2026-10-13), which supersedes #1352 and adds `ParameterProviderError::InvalidField` for a meter period that is not a multiple of the plan's meter window.
- **D3. Own values, refreshed by the service step.**
  - #1348 D1 has landed the refresh. `SessionState::service` runs `refresh_provider_counters`
    before `collect_render_activity` (`crates/control-plane/src/control.rs:1028`) and again after
    it (`:1030`). Every argument-valid control call runs `service` first (#1348 D2).
  - The snapshot's values are, for each configured ID in order, the provider's current value for
    that ID, read after the first refresh in the same service step.
  - So a counter that render changes between two control calls (`LIVE_VALUES_SUPERSEDED`, #1312
    D8) reaches the snapshot that the next call stages (gate 7; Amendment 1).
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
7. **The first refresh is observable (new capi test; #1348 verdict N-3).**
   - Configure `[LIVE_VALUES_SUPERSEDED]` with period 1.
   - With no render in between, commit three fader edits on one track, then render one block.
     Each edit writes the left and right fader cells, so render adds `2 × 2 = 4` to the session
     counter (#1312 D2).
   - The next control call, a `source_submit`, stages a snapshot whose value is 4, equal to
     `COUNTERS_GET` at that call.
   - Mutation (PR evidence): removing the refresh before `collect_render_activity`
     (`control.rs:1028`) turns it red, because the snapshot then reads the stale 0.
   - No other counter discriminates this: the telemetry counters change only inside the service
     step itself.

## Test value

- Gate 1: a snapshot filled with anything but the counter's own value (today's defect).
- Gate 2: a snapshot cadence that ignores the configured period.
- Gate 3: a configuration accepted for a counter that can never report.
- Gate 4: an ID missing from the decoder or the served set makes a host's read of the counter
  `NOT_FOUND` or its configuration refused.
- Gate 7: a service step that stages the snapshot before refreshing the provider, so a counter
  render changed since the last call is reported stale. #1348's suite stays green without that
  refresh (#1348 verdict N-3).

## Amendment 1 (root, 2026-10-05)

The #1348 verifier (verdict N-3, `docs/handoffs/decision-15-2026-10-05/verdicts/stream-b/1348-attempt1.md`)
found that #1348's first counter refresh in `service()` has no observable effect today: removing
it left all 94 capi tests green. This issue owns the requirement, so it gains gate 7.

- The telemetry counters change only while the service step stages records. The test therefore
  uses `LIVE_VALUES_SUPERSEDED`, which render changes between calls.
- This issue now depends on #1312, which adds that counter. The stream order already places it
  after #1312.
- D3 states the landed refresh order.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312): `LIVE_VALUES_SUPERSEDED`, the
  counter gate 7 reads (Amendment 1).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the service step
  that refreshes the counters (D3).

## Decision record (batch 4, lane K)

- **Planner note verified.** The #1348 refresh had landed (`SessionState::service` refreshes before
  and after `collect_render_activity`); the defect was still there: `collect_render_activity`
  filled every configured counter with the render sequence. `LIVE_VALUES_SUPERSEDED = 16` was
  already in the registry, so `TransitionReprepareRefusals = 17` (D6) joins after it.
- **D2 and #1583.** `telemetry_configure` returns `Result<TelemetryConfiguration,
  ParameterProviderError>`; only the counter refusal (`NotFound`) is implemented. The meter-table
  refusals and `InvalidField` are #1583's. No meter staging changed.
- **Cadence (D4).** `SessionState::counter_snapshot_sequence` holds the render sequence at the
  last staged snapshot. While no counter snapshot is configured it follows render, so a
  configuration counts its period from the call that accepted it (the service step runs first in
  that call), not from the first block ever rendered. `stage_counter_snapshot` is a new private
  helper that `collect_render_activity` calls where it staged the snapshot before.
- **Values (D3).** The snapshot reads the provider through `ControlProvider::counters` for the
  configured IDs in order (configured IDs are ascending and unique by decode), after the first
  refresh. A snapshot therefore does not count the drops its own staging causes; the second refresh
  does, for the next call.
- **Gate 1 reading.** "COUNTERS_GET at that call" is read just before the block is rendered: the
  telemetry counters change only inside a service step, so that is the value the call's first
  refresh sees. The test uses meters `[1, 2]` beside the counters so the lossy lane overflows, and
  compares each value the lane kept (the lane may keep part of a snapshot) by ID. Every control
  call there is a `source_submit`.
- **Gate 5.** The oracle test configured `ControlCommandBackpressure`, which is now unserved
  (`NOT_FOUND`). It configures `TelemetryDropped` instead; the response echo (counter ID 6) and the
  `0x8021` event (ID 6, value 1, the real dropped count; was the render sequence 4) are re-pinned.
- `capi_controller_dispatches_every_advertised_command_family` also pins the `COUNTERS_GET` `all`
  response (now five served counters, ID 17 added); re-pinned from the run, for this issue's D1/D6.
- **Gate 7** lives in `live_tests.rs` (it needs `Rig`), not `tests.rs`; its shared helpers
  `counter_telemetry_for` and `lossy_counter_snapshots` are in `tests.rs`. Mutation (one run,
  not committed): removing the first `refresh_provider_counters` in `service` turns it red.
- **Files beyond the authorized list**, all mechanical: `crates/control-plane/src/compile.rs` (the
  new field's initializer), `crates/capi/src/runtime/live_tests.rs` (gate 7),
  `crates/protocol/src/message_wire/tests.rs` (gate 4's round trip), the host-core provider's unit
  test (fifth counter), `docs/CONTROL_PROTOCOL_REGISTRY.md` ("IDs run 1 to 17").
- No schema hash moved: `check-protocol-wasm-parity.sh` passes unchanged.
