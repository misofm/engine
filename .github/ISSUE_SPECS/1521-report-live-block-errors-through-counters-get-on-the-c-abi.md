# Report live_block_errors through COUNTERS_GET on the C ABI

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Successor of *Give each plan its own revision gate and take each block's live snapshot from it*
(#1502, D7), which counts the blocks but gives hosts no way to read the count (#1502 "Batch
follow-ups", NIT 3; #1312 "Batch follow-ups", NIT 3). Code anchors verified on `main` at
`8c0deee2c`.

Root decisions 2026-10-11 after an adversarial review (choices 1-7 decided as recorded in the decisions below).

## Product outcome

A C ABI host reads `LIVE_BLOCK_ERRORS` with `COUNTERS_GET` or a configured `COUNTER_SNAPSHOT`,
like every other session counter. The value is the number of blocks that the render call refused
after the block took its live snapshot. Such a block can have applied live values and rendered no
output, and the watermark does not advance on it. The counter stays 0 in a correct build. Today the
plan's `miso_engine_v1_last_error` names such a block (`render.plan.rejected`) only until the next
render call overwrites it, and the control thread has no lasting count of these blocks.

## Context

All anchors are on `main` at `8c0deee2c`.

- **Where the count is made.** `RealtimePlanOwner` holds `live_block_errors: u64`
  (`crates/engine/src/realtime/plan_exchange.rs:162-163`). `render_active` adds 1 (saturating) when
  the plan's `render_inner` fails with `after_snapshot` set (`:783-801`, the add at `:796`), and it
  does not advance the watermark on that block. `RealtimePlanOwner::live_block_errors()` reads it
  (`:704-712`).
- **What it counts.** `render_inner` (`crates/engine/src/realtime/plan.rs:1078-1112`) refuses the
  output shape and the clock overflow before the snapshot (`after_snapshot: false`, not counted).
  It then takes `S = gate.snapshot()` and maps every error of `render_block` (`:1113-1133`:
  `begin_live_block`, then `executor.render`) to `after_snapshot: true`. `render_contiguous`'s
  `TimeDiscontinuity` check (`plan_exchange.rs`, before `render_active`) is also not counted. No
  valid plan reaches a counted error: `InvalidEnvelope` and `Buffer` are invariant failures
  (#1502 D7, root ruling 3 of 2026-10-05). With no executor, the only failure after the snapshot is `plane_mut`, which cannot fail
  once the output-shape check has passed. So every counted block is an executor invariant failure
  (`InvalidEnvelope`, `Buffer`). The count is "blocks failed after the snapshot".
- **Where it is mirrored.** `SharedPlanState::live_block_errors: AtomicU64`
  (`crates/control-plane/src/plan.rs:30-32`) is set to the owner's count by `PlanState::render` on
  the error path only (`:262-268`, `Release`). `PlanQueries::live_block_errors()` reads it
  (`:101-105`). The unit test `the_live_block_error_count_reaches_the_any_thread_query`
  (`:340-392`) covers the mirror.
- **Per session, not per plan (verified).** `compile_children` makes one plan exchange
  (`crates/control-plane/src/compile.rs:825`, `plan_exchange_at_revision`) and one
  `SharedPlanState` (`:839-850`) per session. A rebuild swaps plans inside the same
  `RealtimePlanOwner`, so the count covers every plan the session adopts and never resets.
  `SessionState` holds the same `Arc` (`crates/control-plane/src/control.rs:367`; `compile.rs:907`),
  and so does the plan's `PlanState` (`compile.rs:914`) and the capi `Plan`'s `queries` field
  (`crates/capi/src/abi.rs:472-488`).
- **Nothing reads it today.** No code outside `crates/control-plane/src/plan.rs` and the engine
  tests calls either accessor (`grep -rn live_block_errors crates hosts`). The
  larger `PROVIDER_COUNTER_SLOTS` changes only retained bytes; those are ceilings, and
  there are no exact pins (#1312's growth from 3 to 4 slots touched only the files this draft
  lists). The C ABI maps the
  error to `RESULT_RENDER_REJECTED` with the plan diagnostic `PLAN_REJECTED`
  (`crates/capi/src/ffi.rs:965-968`, `crates/capi/src/runtime/error.rs:125-132`), the same as for
  other refusals.
- **How a comparable counter reaches the host (the model for this slice).**
  `LIVE_VALUES_SUPERSEDED` (#1312 D8):
  1. registry: `CounterId::LiveValuesSuperseded = 16` in
     `crates/protocol/src/message_wire.rs:622-643` and its arm in `parse_counter_id` (`:2033-2052`);
  2. provider: `SessionControlProvider::set_live_values_superseded`
     (`crates/host-core/src/control_provider.rs:266-275`) writes it through `set_counter`
     (`:611-621`), which inserts in ID order into `counter_snapshot`. That vector is reserved at
     `max(counter_ids, PROVIDER_COUNTER_SLOTS)` (`:21-23`, `:118`, `:168`), so
     `PROVIDER_COUNTER_SLOTS` must count every provider-owned counter, or an insert allocates;
  3. refresh: `SessionState::refresh_provider_counters` (`control.rs:1068-1075`) copies it in; the
     service step runs the refresh twice (`:1056-1062`), and every argument-valid session call
     runs the service step first (#1348 D2);
  4. header: one prose sentence (`crates/capi/include/miso_engine_v1.h:57-59`); registry doc:
     `docs/CONTROL_PROTOCOL_REGISTRY.md:37` ("Counter IDs run 1 to 16"); qualification doc:
     `docs/C_ABI_V1_QUALIFICATION.md:242-245` ("four provider-owned counters").
  `TRANSITION_REPREPARE_REFUSALS`, the D15-17 defect counter, takes the same path (#1351 D6 adds
  the ID, #1360 D5 the refresh).
- **Pinned wire bytes.** `ALL_COMMAND_RESPONSE_VECTORS[9]` (`crates/capi/src/runtime/tests.rs:106-109`,
  used by `capi_controller_dispatches_every_advertised_command_family`, `:2264`) is the
  `COUNTERS_GET` `all` response. A new served counter changes its bytes; #1312 re-pinned it the
  same way.
- **The browser (updated 2026-10-24).** #1381 closed: host-web renders through
  `PlanState::render` (`WebPlan = PlanState<WebAdapter>`) and its session is
  `control_plane::SessionState`, so the mirror and the refresh this slice adds already run in the
  browser; see the browser non-goal.

## Decisions frozen for this slice

- **D1. Registry.** `CounterId::LiveBlockErrors` joins the frozen registry and `parse_counter_id`
  at the next free value, 19 (#1768 took 18 for `STRIPS_RESTARTED`; root, 2026-10-24), never a
  reused one. Its wire name is `LIVE_BLOCK_ERRORS`. Tests use the name, never the
  number. It is a non-resetting, saturating `u64`, like every registered counter.
- **D2. Served from construction.** The provider serves it at 0 from construction, in #1351 D1's
  fixed served set. `PROVIDER_COUNTER_SLOTS` grows by one. Add
  `SessionControlProvider::set_live_block_errors(count)`, the same shape as
  `set_live_values_superseded`.
- **D3. Refresh.** `refresh_provider_counters` reads `self.shared.live_block_errors` (`Acquire`)
  and calls `set_live_block_errors`. No other call site. The render side does not change.
- **D4. Unit: blocks, per session.** One count per session, over every plan the session adopts,
  never reset (Context, "Per session"). No per-plan count. `refresh_provider_counters` copies the
  level, not a delta, so a provider promoted at an epoch also serves the full value, because
  `service()` refreshes after `synchronize_plan_epochs`.
- **D5. No watermark outcome flag.** An error block advances no
  watermark and covers no revision. D15-17's flags are an OR over the revisions an advance covers,
  for fallbacks decided after submit. A defect path is a counter, as `transition_reprepare_refusals`
  is.
- **D6. Test seam.** Under `test-support`, `control_plane::PlanQueries` gains
  `test_store_live_block_errors(&self, count: u64)`, which stores `count` into the mirror
  (`Release`). The chain has three links, and each has a test of the real production function: engine
  counting (`a_block_the_executor_fails_after_the_snapshot_is_counted`); owner to mirror through
  `PlanState::render`, which `miso_engine_v1_render_f32_planar` calls
  (`the_live_block_error_count_reaches_the_any_thread_query`); and mirror to wire (gate 1). Gate 1
  stores through the capi `Plan`'s own `queries` field and reads through the session, so it also
  proves that the plan and the session share one `SharedPlanState`. The seam is `#[doc(hidden)]`,
  exists under `test-support` only, and its value differs from the owner's count until the next
  error block overwrites it. No valid plan reaches a counted error, so a real fault would need a
  test hook in a hot render file (`crates/graph/src/lib.rs` or `crates/engine/src/realtime/plan.rs`)
  for no added catch.
- **D7. Header.** In the `miso_engine_v1_last_error` paragraph, after the `render.plan.rejected`
  list (`miso_engine_v1.h:145-149`), not in "Completion (issue 1314)", one sentence: "The counter
  LIVE_BLOCK_ERRORS (read with COUNTERS_GET) counts the render calls that returned
  MISO_ENGINE_V1_RENDER_REJECTED with "render.plan.rejected": blocks the plan refused after the
  block took its live snapshot. Such a block may have applied live values, its output is not a
  rendered block, and the watermark does not advance on it. No valid plan reaches this path, so a
  nonzero value is an engine defect." No new symbol and no feature bit.
- **D8. The acked-batch question: can an ack ever precede a drop? No.** This slice adds no queue
  and no ack. The counter is a level the control thread copies; render changes nothing.
- **D9. Surface: `COUNTERS_GET`, not a watermark word.** An error block advances no watermark, so
  the counter has no place in the watermark record; #1381 D5 defines `WebPlanWatermark` to mirror
  `miso_engine_v1_watermark` field for field, so a defect word in one record would also force it
  into the other.
- **D10. Placement: stream B, after #1351.** #1351 D1 (the fixed served set), D2 (`NOT_FOUND` for
  counters outside the set) and D6 (ID 17) are prerequisites; without them gate 1's "0, not
  `NOT_FOUND`" cannot hold and the ID would be 17, not 18.

## Deliverables

1. D1 in `crates/protocol/src/message_wire.rs`; D2 in `crates/host-core/src/control_provider.rs`;
   D3 in `crates/control-plane/src/control.rs`; D6 in `crates/control-plane/src/plan.rs`.
2. D7 in `crates/capi/include/miso_engine_v1.h`.
3. Gates 1 and 2; the re-pin in gate 3.
4. Docs: `docs/CONTROL_PROTOCOL_REGISTRY.md:37` names the new ID and the range at merge;
   `docs/C_ABI_V1_QUALIFICATION.md:242-245` counts the new provider-owned counter: one more than
   the count at merge, and `live_block_errors` joins the list.

## Authorized paths

- `crates/protocol/src/message_wire.rs` (`CounterId` and `parse_counter_id` only).
- `crates/host-core/src/control_provider.rs` (`PROVIDER_COUNTER_SLOTS`, the new setter, the
  served set, and the provider's served-set test as it stands after #1351).
- `crates/control-plane/src/control.rs` (`refresh_provider_counters` only).
- `crates/control-plane/src/plan.rs` (the `test-support` seam of D6 only).
- `crates/capi/include/miso_engine_v1.h` (D7's sentence only).
- `crates/capi/src/runtime/live_tests.rs` (gate 1), `crates/capi/src/runtime/tests.rs`
  (`ALL_COMMAND_RESPONSE_VECTORS[9]` re-pin only).
- `docs/CONTROL_PROTOCOL_REGISTRY.md` (line 37), `docs/C_ABI_V1_QUALIFICATION.md` (the
  provider-owned counter sentence).

## Non-goals

- **Browser: no browser code.** #1381 closed: host-web renders through `PlanState::render` and its
  session is `control_plane::SessionState`, so the mirror store and the service-step refresh run in
  the browser too, and both hosts serve the counter from the one host-core served set (one control
  abstraction). A browser render error is still terminal (`STATE_FAILED`), so its value is 0 or 1.
  No browser parity row: the value is 0 on both hosts in a correct build and gate 1 proves
  mirror-to-wire. No change to `WebStatus`, `WebPlanWatermark`, any JS, `.d.ts` or SDK mirror.
- A watermark outcome flag (D5) or a word in `miso_engine_v1_watermark` (D9).
- Any change to what is counted, to the render path or to the engine crate.
- Fault injection in the engine to drive a real after-snapshot failure through
  `miso_engine_v1_render_f32_planar` (D6).

## Objective gates

1. **The count reaches `COUNTERS_GET` (new capi test, `live_tests.rs`).**
   - On a fresh session, before any render, `COUNTERS_GET` for `LIVE_BLOCK_ERRORS` answers 0
     (not `NOT_FOUND`).
   - Render 5 blocks. Store 3 through D6's seam on the rig's plan `queries`.
   - `COUNTERS_GET` for `LIVE_BLOCK_ERRORS` answers 3 (5 is the render sequence and 0 the carry
     count, so a counter fed from either is red).
   - Configuring `[LIVE_BLOCK_ERRORS]` with period 1 is accepted, and the next control call stages a
     snapshot whose value is 3.
   - Mutation (PR evidence): delete the `set_live_block_errors` call in
     `refresh_provider_counters`; the test reads 0 and is red.
2. **No allocation for the new slot (extend the provider's served-set test as it stands after
   #1351).** After the existing setters, call `set_live_block_errors(17)`.
   The `all` snapshot has `LiveBlockErrors` with value 17 in ID order, and
   `retained_capacities().1` still equals `PROVIDER_COUNTER_SLOTS`.
   - Mutation (PR evidence): leave `PROVIDER_COUNTER_SLOTS` at its old value; the insert grows the
     vector and the capacity assertion is red. This works only if `retained.counter_ids` stays
     below the slot count in that test.
3. **Wire re-pin.** Re-pin `ALL_COMMAND_RESPONSE_VECTORS[9]` with a comment like #1312's: the
   snapshot gains `LIVE_BLOCK_ERRORS`, value 0, one `CounterValue` message more; give the length
   and TLV-count change. This is a wire-format pin (AGENTS.md test-value rule), not a new test.
4. **Commands.**
   - `cargo test --locked -p capi`
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p host-core --features control-provider,test-support`
   - `cargo test --locked -p protocol --features test-support`
   - `bash scripts/check-protocol-control-policy.sh`, `bash scripts/check-capi-abi.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - No realtime gate: the render path does not change. Run
     `cargo run --locked --release -q -p realtime-policy` once to show it.

## Test value

- Gate 1: red if the service step never copies the mirror into the provider, if the counter is
  copied from the wrong source, or if `parse_counter_id` lacks the new arm (the request is refused
  as malformed), or if the counter is missing from the served set that `telemetry_configure`
  checks. No existing test reads the mirror outside `crates/control-plane/src/plan.rs`.
- Gate 2: red if `PROVIDER_COUNTER_SLOTS` does not count the new counter, so the first refresh
  allocates on the control path past the retained capacity that resource admission charged.

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it* (#1502):
  the counter and its mirror. Landed.
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the service step
  and its counter refresh. Landed.
- *Hold live values in latest-target cells on both hosts* (#1312): the `LIVE_VALUES_SUPERSEDED`
  path this slice copies. Landed.
- *Report each configured counter's own value in the C ABI counter snapshot* (#1351): the fixed
  served set (D1), the typed refusal of unserved counters (D2), and ID 17. This slice comes after
  it.

## Proposed placement (root edits `STREAMS.md`)

- Stream B, after #1351 (order 23).
- `crates/control-plane/src/*` row: this slice (`refresh_provider_counters` and D6's seam only)
  after B #1351, in any order with the rest of the row, the later slice rebases. C #1360 adds a
  line to the same function.
- `crates/capi/include/miso_engine_v1.h` row: after B #1351, in any order with C #1360, the later
  slice rebases.
- `crates/protocol/src/message_wire.rs` and `crates/host-core/src/control_provider.rs`: after
  #1351, which takes ID 17 and edits the served set first.

## Attempt record

- Attempt 1 (2026-10-24, on `main` `82f987296`): implemented as the scope note's slice. ID 19;
  `SERVED_COUNTERS` is 6 long with `LiveBlockErrors` last; `set_live_block_errors`; one refresh line;
  D6 seam; gate 1 is `the_live_block_error_mirror_reaches_counters_get_and_the_snapshot`
  (`crates/capi/src/runtime/live_tests.rs`); gate 2 extends
  `clock_transport_counters_and_diagnostics_are_session_backed`; the protocol round-trip test takes
  ID 19 and refuses 20. Gate 3: `ALL_COMMAND_RESPONSE_VECTORS[9]` length 0x100 -> 0x130 (a 40-byte
  value message plus its 8-byte TLV header), TLV count 6 -> 7. Mutants (PR evidence): the refresh
  line deleted -> gate 1 reads 0, not 3 (red); `LiveBlockErrors` left out of `SERVED_COUNTERS` ->
  gate 1's configure is refused (red) and gate 2's capacity grows 5 -> 10 (red). Reads alone do not
  catch the second mutant: `set_counter` inserts an unserved ID. Gate 1 stores through a `#[cfg(test)]` helper in
  `crates/capi/src/ffi.rs` (root authorized the path), because realtime-policy refuses `unsafe` in
  `live_tests.rs`.
