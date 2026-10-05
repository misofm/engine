# Add miso_engine_v1_service for bounded control work between edits

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The engine owns no thread, so the host drives control work. A C host calls
`miso_engine_v1_service(session)` from its control thread whenever it likes (for example once per
UI frame), and every other session control call performs the same work first. Each call does a
bounded amount: it reclaims retired plans, brings provider epochs up to render, and stages render
telemetry. Later work (catch-up slices and the catch-up deadline check) joins the same step, so a
host that services keeps every pending edit progressing without ever waiting on render. The browser
Worker calls the same engine-side step.

## Context

- Today the step exists in pieces. `synchronize_plan_epochs`
  (`crates/capi/src/runtime/control.rs:783-826`) promotes a pending provider when render has
  adopted its plan and reclaims retired plans (`retirer.try_reclaim`, loop at `:791-805`;
  `PlanRetirer::try_reclaim`, `crates/engine/src/realtime/plan_exchange.rs:512`).
  `collect_render_activity` (`control.rs:522`) stages the render peak and counter telemetry.
- Which control calls run them: `command` runs both (`:848-849`); `dequeue_event` runs both
  (`:1397-1402`); `submit` (`:1506`), `seek` (`:1520`) and `seek_at` (`:1537`) run only the
  epoch synchronization. A host that renders but makes no control call never reclaims a retired
  plan, so the plan's memory stays allocated until its next call, and with retirement capacity 1
  (`crates/capi/src/runtime/compile.rs:755-761`) the next swap waits for that call.
- The engine handle owns no session (`crates/capi/src/abi.rs:364-369`); the control state is the
  session's `SessionState`. Decision 15 records that the entry point takes the session handle (the
  agreed plan's `miso_engine_v1_service(engine)` spelling is superseded).
- Session thread rule: `crates/capi/include/miso_engine_v1.h:18-22`.
- The control path moves to `crates/control-plane` under
  *Extract the C ABI control plane into a portable crate both hosts call* (#1309, D1-D2).

## Decisions frozen for this slice

- **D1. One step.** `SessionState::service(&mut self) -> Result<(), CommandError>` in the
  control-plane crate runs, in order: `synchronize_plan_epochs`, then `collect_render_activity`.
  It is `pub`: it is the engine-side hook the browser Worker calls
  (*Swap and retire browser plans through the Worker's service loop*, #1381).
- **D2. Every control call services.** `command`, `dequeue_event`, `submit`, `seek`, `seek_at` (and
  any session query added later, such as #1316's seek report) call `service()` first, replacing
  their individual calls. No call services twice.
- **D3. Bounded.** One call reclaims at most the retirement queue's capacity of plans and stages at
  most one render observation; it never loops on render progress and never waits. Its cost does
  not depend on how long the host went without calling.
- **D4. C ABI.** `uint32_t miso_engine_v1_service(miso_engine_v1_session *session)`. Thread:
  session, serialized with the other session calls (added to the list at `miso_engine_v1.h:18-22`).
  Results: `OK`; a dead or wrong handle as every session call; an internal epoch failure as
  `MISO_ENGINE_V1_INTERNAL` with the session diagnostic the other calls already set for it
  (`SourceFailure::Internal`'s text). Feature bit `MISO_ENGINE_V1_FEATURE_SERVICE`: the next free
  bit (256 after #1314's 64 and #1316's 128; never reuse one).
- **D5. Duty text** in the header and `C_ABI_V1_QUALIFICATION.md`: a pending edit progresses only
  while the host makes control calls, the same duty as draining events. A host that renders but
  never services keeps retired plans allocated, and (with #1356) holds live edits made during a
  catch-up until its next control call; the watermark (#1314) shows it. Call `service` at least
  once per render-buffer period while edits are pending.
- **D6. Acked-batch question.** Service commits nothing and acknowledges nothing; it adds no queue.

## Deliverables

1. D1-D3 in the control path; D4 export, constant and bit; `abi_smoke.c` and `header_smoke.cpp`;
   the frozen symbol list; D5 text.

## Authorized paths

- `crates/control-plane/src/control.rs` (or `crates/capi/src/runtime/control.rs` if #1309 has not
  landed)
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `runtime/tests.rs`, `include/miso_engine_v1.h`,
  `tests/c/abi_smoke.c`, `tests/c/header_smoke.cpp`
- `scripts/check-capi-abi.sh` (frozen list only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Catch-up slices and the deadline check (*Fall back from a missed catch-up deadline: bounded
  render-thread pre-roll, then the transition*, #1358; *Run the browser catch-up in the Worker's
  service loop*, #1361). The browser Worker loop itself (#1381). A thread owned by the engine.

## Objective gates

1. **Service reclaims.** C ABI test: commit a rebuild, render the swap block, then call only
   `miso_engine_v1_service`: the retired plan is disposed (the `current_plan_disposed` test
   counter grows by 1) and the provider epoch is promoted; a second rebuild then reserves without
   `BACKPRESSURE`.
2. **Every call services.** Same setup, but call `miso_engine_v1_source_submit_planar_f32` instead
   of `service`: with a meter handle configured, the retired plan is disposed and a render-peak meter record is staged, as with
   `service`.
3. **Bounded and idle-safe.** `service` on a session with nothing pending returns `OK` and changes
   no counter; 1,000 consecutive calls with no render change nothing.
4. **ABI.** `abi_smoke.c` tests the bit with `&` and calls `service(NULL)` → `INVALID_ARGUMENT`;
   frozen list grows by one.
5. Commands:
   - `cargo test --locked -p capi` and `cargo test --locked -p control-plane --features test-support`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if `service` does not reclaim or does not promote (the second rebuild would then hit
  the pending-candidate refusal); no call exists today that does only this work.
- Gate 2: red if a control call skips the telemetry half of the step, as `submit` does today.
- Gate 3: red if the step does work that grows with idle time or mutates state with nothing pending.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): feature-bit
  order and the duty text's reference to the watermark.
