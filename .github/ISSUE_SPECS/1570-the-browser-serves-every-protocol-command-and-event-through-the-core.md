# The browser serves every protocol command and event through the core

Slice S14 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Row 1 of the cross-host audit (the protocol plane) is split in two slices. This slice is the
host half: the event lane, the render activity that stages telemetry events, and the proof that
both hosts give the same bytes. #1571 is the SDK half: typed calls for the same messages. #1562
already adds the generic command entry (`miso_engine_web_v1_control_*` and the `control` port
message), which accepts every frame `SessionState::command` accepts. This slice adds no command
export.

## Product outcome

In the browser, every protocol command (`CAPABILITIES_GET`, `SESSION_SNAPSHOT_GET`,
`TRANSPORT_GET`, `TRANSPORT_SET`, `TELEMETRY_CONFIGURE`, `COUNTERS_GET`, `DIAGNOSTICS_GET`,
`AUTOMATION_ENQUEUE`, and the control list) returns the same response bytes as on the C ABI. Every
protocol event (`SESSION_COMMITTED`, `AUTOMATION_CANCELED`, `TRANSPORT_STATE`, `METER_BATCH`,
`COUNTER_SNAPSHOT`, `DIAGNOSTIC`) reaches the page with the same bytes and in the same order. The
control half drains both event lanes itself, so a browser session never stops committing because
no one read its events. A native test proves the two hosts byte-identical for one script.

## Context

- **The C ABI's event entry.** `miso_engine_v1_dequeue_event` (`crates/capi/src/ffi.rs:768`) calls
  `SessionState::dequeue_event` (`crates/control-plane/src/control.rs:1673`). Lanes are `EventLane`
  (`control.rs:564`), with the C codes `EVENT_LANE_RELIABLE = 0` and `EVENT_LANE_LOSSY = 1`
  (`crates/capi/src/abi.rs:32`, `:34`). Failures are `EventError` (`control.rs:573`); capi spells
  their texts itself (`ffi.rs:815`, `:822`, `:829`). The event frame is written into the session's
  one `response_scratch` (`event_response`, `control.rs:1762`), which the command response also
  uses (`command_response`, `:1667`); it is allocated at `maximum_control_frame_bytes`
  (`crates/control-plane/src/compile.rs:908`).
- **The queues are small, and a full lane refuses commits.** `protocol_queue_config`
  (`compile.rs:149-172`) gives 2 reliable event slots (`:163`) and 1 telemetry slot (`:165`); the
  reliable lane also carries up to 2 render diagnostics (`RENDER_DIAGNOSTIC_SLOTS`,
  `control.rs:157`). A transaction reserves its reliable events before it commits, and answers
  `BACKPRESSURE` when the lane has no room (`crates/protocol/src/controller.rs:1736-1745`). A host
  that never drains therefore commits at most two transactions.
- **The browser drains nothing.** #1562's entry runs commands through the Worker's
  `SessionState`, but no browser export or message dequeues an event.
- **Telemetry events need render activity.** The service step (`control.rs:1059`) runs
  `collect_render_activity` (`:661`). That function stages `METER_BATCH`, `COUNTER_SNAPSHOT` and the
  render `DIAGNOSTIC` only when render advanced `render_sequence`. Only
  `PlanState::publish_render_observation` (`crates/control-plane/src/plan.rs:290`) advances it, and
  it also stores `render_sample`, which `TRANSPORT_GET` reports as `effective_sample`. capi calls it
  after its own peak scan (`ffi.rs:939-955`), gated by `render_peak_observed` (`plan.rs:285`).
  host-web's `render_next` (`hosts/host-web/src/lib.rs:3363`) never calls it, and #1381 D4 (render
  through `PlanState`) does not add it. So in the browser no telemetry event is ever staged, and
  `effective_sample` stays 0.
- **The service cadence exists.** *Swap and retire browser plans through the Worker's service
  loop* (#1381, D6) runs the service step after every control message and on a one-quantum tick,
  in `worker` and in `single` mode. This slice needs no other cadence.
- **The diagnostic vocabulary.** The control-plane crate doc says the crate never spells an
  adapter's diagnostic codes (`crates/control-plane/src/lib.rs:9-15`). #1381 D7 already keeps the
  crate's `capi.render.activity` code on both hosts.
- **Every command already reaches the core.** The registry is `MessageId`
  (`crates/protocol/src/wire.rs:98-133`): 11 commands and 6 events. Both hosts build the controller
  with `ProviderFeatures::ALL` (`compile.rs:870`). `AUTOMATION_ENQUEUE` reaches no PCM on any host
  (`docs/CONTROL_PROTOCOL_SEMANTICS.md:15`); *Refuse commands that would be acknowledged with no
  effect* (#1315) refuses it. The same controller answers on both hosts, so their answers match
  whichever state #1315 is in.
- **Frozen export lists.** `scripts/check-web-audioworklet.sh:203` (`expected_exports`),
  `tools/parameter-metadata/src/abi_layout.rs:135` (`EXPORTS`), `scripts/check-abi-layout-v1.py:117`
  (`EXPORTS`), and the render-locked table `tools/realtime-policy/render-thread-exports.json`.
- **Cross-host tests on `main`.** `hosts/host-web/tests/strip_cells_cross_host.rs` and
  `hosts/host-web/tests/control_smoothing_parity.rs` link `capi`, `control-plane` and `protocol` as
  dev-dependencies (`hosts/host-web/Cargo.toml:35-50`).

## Decisions frozen for this slice

- **D1. One event export.** `miso_engine_web_v1_control_event(handle: u32, lane: u32) -> u32`, on
  the control half only (the Worker in `worker` mode, the worklet's message handler in `single`
  mode), beside #1562's `control_*` exports.
  - `lane` is `0` (reliable) or `1` (lossy), the C ABI's codes. Any other value returns
    `RESULT_INVALID_ARGUMENT` and dequeues nothing.
  - It calls `SessionState::dequeue_event(lane, capacity)` with the full response scratch as
    capacity. On success it returns `RESULT_OK`, and #1562's `control_response_ptr` and
    `control_response_bytes` give the event frame. `0` bytes means the lane is empty.
  - `EventError::Backpressure` returns `RESULT_BACKPRESSURE`, `Internal` returns `RESULT_INTERNAL`,
    and `BufferTooSmall` returns `RESULT_BUFFER_TOO_SMALL` (it cannot occur with the full scratch;
    it is mapped all the same). A refusal sets the response length to 0 and writes D2's text into
    the control half's session diagnostic (#1381 D5).
  - On an instance with no control half it returns `RESULT_REFUSED_LIFECYCLE`.
  - The response staging is shared with #1562's command: it is valid until the next
    `control_command` or `control_event` call.
  - **C1 justification.** This is the browser form of `miso_engine_v1_dequeue_event`, not a new
    function. A Wasm caller cannot hand the module a caller-owned output buffer, so it reads the
    frame through #1562's pointer exports, which are transport only. No C ABI export is added.
- **D2. One event refusal vocabulary.** `EventError::diagnostic(self) -> &'static [u8]` in
  `control-plane` returns exactly today's texts: `control.event.output.too_small`,
  `control.event.backpressure`, `control.event.internal`. capi's `dequeue_event` and host-web's
  export both use it. The crate doc at `crates/control-plane/src/lib.rs:9-15` is rewritten: the crate
  owns every command and event refusal text (decision 17 C1); an adapter owns only its result-code
  numbering. No C ABI byte changes.
- **D3. Render activity comes from the core.** New `PlanState::observe_rendered_block(&self,
  planes: [&[f32]; 2])` in `crates/control-plane/src/plan.rs`:
  - When `render_peak_observed()` is true, the peak starts at `0.0_f32` and folds
    `peak = peak.max(sample.abs())` over plane 0 in frame order, then plane 1. This is capi's loop
    (`ffi.rs:939-954`), moved unchanged. Otherwise the peak is `f32::NAN`.
  - Then it calls `publish_render_observation(peak)`.
  - capi's render calls it after a successful render, with the two plane slices of the validated
    output layout, and deletes its own loop.
  - host-web's render half calls it after every successful render, in both modes. It runs inside
    the render-locked window: no allocation, no lock, at most `2 * quantum` loads.
  - #1581 later replaces this peak with the core's output meter. This slice moves the existing
    rule and changes no bit.
- **D4. The control half drains both lanes.** It drains after it handles each #1562 `control`
  message and after each service step (#1381 D6):
  - It dequeues the reliable lane until a call returns 0 bytes, then the lossy lane the same way.
    It makes at most 16 calls per lane per drain. More than 16 cannot be queued (2 reliable events,
    2 render diagnostics, 1 telemetry slot); any rest waits for the next drain.
  - It posts each frame to the page, in dequeue order, as the internal message
    `control-event { lane, frame }`. `lane` is `0` or `1`. `frame` is a copy in a new
    `Uint8Array`. In `worker` mode the message goes on #1332 D7's port. In `single` mode the
    worklet's control handler posts it on the node's port, never inside `process()`.
  - A refused dequeue (D1) stops that lane's drain and is reported once in the host wrapper's
    error path. It does not retry in a loop.
- **D5. The page side.** The shipped host wrapper
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`) gains `onControlEvent(listener)`,
  which returns an unsubscribe function (`@internal`, in the `.d.ts` and its SDK copy). It calls
  every listener with `{ lane, frame }` in arrival order. With no listener it drops the frame and
  adds 1 to `discardedControlEvents` (an `@internal` read-only field). An event is not the ack of
  a command (the command's reply is), so a drop at the page changes no engine state and no ack
  precedes it.
- **D6. One set of limits in the proof.** host-web's `test-support` feature exposes the
  `ControlLimits` value the browser control half was built with (#1381 D1, #1562 D1), read-only.
  Gate 1 builds the C ABI's `CompileLimits` from it field for field, so both hosts advertise the
  same capabilities.
- **D7. Mirrors.** The export joins every list #1562 D8 names: the ABI layout document and its
  generator, the regenerated `sdk/src/generated/abi.ts` and `sdk/assets/miso-engine-v1-abi-layout.json`,
  `scripts/check-abi-layout-v1.py` and its self-test fixture, `scripts/check-web-audioworklet.sh`'s
  `expected_exports`, `tools/realtime-policy/render-thread-exports.json` (as a control-half export,
  like #1562's), and the shipped `.d.ts` with `sdk/src/browser/shipped-host.d.ts`.
- **D8. Acked-batch question: can an ack ever precede a drop? No.** The engine queues and their
  reservations do not change. A dequeue removes an event only when it returns its whole frame. A
  page-side drop (D5) never touches engine state.

## Deliverables

1. D2 and D3 in `crates/control-plane/src/{control.rs,plan.rs,lib.rs}`, and their two call sites
   in `crates/capi/src/ffi.rs`.
2. D1, the D3 call and D6 in `hosts/host-web/src/{ffi.rs,lib.rs}`.
3. D4 in the control Worker script that #1332 adds and in
   `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (`single` mode); D5 in the host wrapper.
4. D7's mirrors.
5. Gates 1-3 in the new `hosts/host-web/tests/protocol_cross_host.rs`; gate 4 in
   `scripts/test-web-audioworklet.mjs`.

## Authorized paths

- `crates/control-plane/src/{control.rs,plan.rs,lib.rs}`.
- `crates/capi/src/ffi.rs` (the render scan and the event texts only).
- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/protocol_cross_host.rs`
  (new), `hosts/host-web/Cargo.toml` (the `bench-support` dev-dependency, if no earlier slice added
  it).
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/check-web-audioworklet.sh` (the export list only),
  `tools/realtime-policy/render-thread-exports.json`.
- Regenerated only: `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`.
- `sdk/src/browser/shipped-host.d.ts` (`onControlEvent` and `discardedControlEvents` only).
- `scripts/test-web-audioworklet.mjs`.

## Non-goals

- No command export or message (#1562 has them).
- No typed SDK call and no SDK event decoding (#1571).
- No change to queue sizes, event payloads or counter values. The counter-value defect is
  *Report each configured counter's own value in the C ABI counter snapshot* (#1351). Its fix is
  in `control-plane`, so it reaches both hosts.
- No meter change: the strip meters and the output meter are #1581 to #1588.
- No change to `WebStatus` or the meter lease.

## Objective gates

Commands: `cargo test --locked -p host-web --features host-web/test-support` (gates 1-3),
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi` (gate 5),
`bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh` (gate 4),
`python3 -B scripts/check-abi-layout-v1.py`,
`bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`bash scripts/check-sdk-types.sh`.

1. **Same bytes on both hosts** (`hosts/host-web/tests/protocol_cross_host.rs`). Both legs are
   native and run in one process. The C ABI leg uses the `capi` FFI with D6's limits. The browser
   leg uses host-web's control half and render half through the Rust functions that the exports
   call. Both boot `tests/browser-v1/session.json` with its source extended to 8,192 frames. Both
   are fed the same PCM. The script, in order:
   1. `CAPABILITIES_GET`.
   2. `SESSION_SNAPSHOT_GET` at offset 0 with `maximum_bytes` 64, then continuation pages until
      `eof`.
   3. `TRANSPORT_GET`.
   4. `TRANSPORT_SET` to playing at position 0.
   5. `TELEMETRY_CONFIGURE`: meter handle 1 with period 1, counter `LIVE_VALUES_SUPERSEDED` with
      period 1, diagnostics on at `Info`.
   6. `CONTROL_LIST_GET`, first page.
   7. One `SESSION_TRANSACTION_APPLY` with one `SET_CONTROL` fader edit.
   8. 8 render blocks, with a service step after each.
   9. `COUNTERS_GET` for all counters.
   10. `DIAGNOSTICS_GET` after sequence 0, limit 16, at `Info`.
   11. `AUTOMATION_ENQUEUE` with one record.
   12. A frame with message ID `0x0004`.
   13. A frame with a wrong magic.
   14. Step 3's exact bytes again (a replay).
   15. Step 3's request ID with different bytes.

   After each step, the two result codes are equal and the two response frames are byte-equal.
   After steps 4, 5, 7 and 8 and at the end, both legs drain both lanes in D4's order. The two
   sequences of `(lane, frame)` are equal. On the browser leg they contain at least one
   `SESSION_COMMITTED`, `TRANSPORT_STATE`, `METER_BATCH`, `COUNTER_SNAPSHOT` and `DIAGNOSTIC`. The
   two renders are bit-identical in every block.
2. **Draining keeps commits flowing.** On the browser leg, 20 consecutive `SET_CONTROL`
   transactions, each followed by D4's drain, all reply `OK`. On a second boot with no drain, the
   third transaction replies `BACKPRESSURE`. This shows that the drain is what lets the browser
   commit.
3. **Realtime.** The test binary links `bench_support::alloc` and calls `assert_installed()` first.
   With telemetry configured (so the peak scan runs), the thread-scoped counters read
   `allocations == 0 && frees == 0` around each of 64 render calls after warm-up, on the browser
   render half (render and D3) and on `miso_engine_v1_render_f32_planar`.
4. **The port** (hermetic, `scripts/test-web-audioworklet.mjs` with fake exports). After a
   `control` message and after a service tick, the control half calls `control_event` on lane 0
   until 0 bytes, then on lane 1. It posts the frames as `control-event` messages in that order,
   and stops at 16 calls per lane. No `control_event` call happens inside `process()`. With no
   listener, the wrapper's `discardedControlEvents` counts each frame.
5. **The C ABI is unchanged.** `cargo test --locked -p capi` passes with no test edited except a
   moved import. `target/release/audit capi` reports the merge base's `pcm_digest` (PR evidence,
   not a committed test).

## Test value

- Gate 1 turns red if any command or event takes a browser-only path, if the browser publishes no
  render activity (no telemetry events, a wrong `effective_sample`), or if the two hosts are built
  with different protocol limits.
- Gate 2 turns red if the control half stops draining, so the browser refuses its third commit.
- Gate 3 turns red if the moved peak scan or the browser's call to it allocates on the render
  thread.
- Gate 4 turns red if the drain order, the bound or the `single`-mode placement is wrong, or if the
  page drops an event without counting it.
- Gate 5 turns red if moving the scan or the texts changes a C ABI bit or byte.

## Dependencies

- #1562 (the generic command entry and the `control` message), which brings #1551, #1552 and #1553
  (`CONTROL_LIST_GET` and `SET_CONTROL` in gate 1).
- *Swap and retire browser plans through the Worker's service loop* (#1381): `SessionState` and
  `PlanState` in the browser, the service cadence and the session diagnostic.
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332): the
  modes and the port.
- File order: `hosts/host-web/src/lib.rs` is stream H's hot file (`STREAMS.md` row 101), and this
  slice rebases after #1332 and #1381. `crates/capi/src/ffi.rs` is stream B's. *Report each
  configured meter handle's own meter in the C ABI meter batch* (#1352) edits the same peak scan
  (see the question for root).

## Supersedes and shrinks

- None. #1571 shrinks #1293 and #1294.

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** D4 and D5 run in `single` mode on the node's port. #1332 later posts
the same `control-event` frames on its port. Dependencies: #1332 is removed; #1381 means #1381a.

The slice is split. Both pieces stay in this issue.

- **#1570a** (core half, batch 4): D2 (`EventError::diagnostic`), D3's
  `PlanState::observe_rendered_block`, and the C ABI rewired to them. Gate 5. Hermetic; no
  dependency on #1562.
- **#1570b** (host half): D1 (the export), D3's browser call, D4 (the drain), D5 (the wrapper),
  D6 and D7 (the mirrors). Gates 1-4. Dependencies: #1570a, #1562a, #1381a. #1570b lands in one
  batch with #1381a, #1562a, #1563B and #1565A.

**Flip batch 6W (root ruling A1).** #1381a, #1562a, #1570b, #1563B and #1565A land together, in one batch of lane W. Each slice keeps its own commits and tests and is green at its own boundary; the batch runs the full CI-equivalent gate set once. If one slice fails, the batch stops: `main` never gets a partial flip, so it never has two writers of one plan's revision gate and cells. No transitional API lends `SessionState`'s producers or revision to the old command path.

## Implementation record (#1570a, batch 4 review fold, 2026-10-14)

Gate 5 (the C ABI is unchanged), `audit capi` on the merge base `205a8d2b4` and on the batch 4 fold
head: `pcm_digest` `76525e7ef9e78cf4` on both (`cargo run --locked --release -p audit -- capi`,
`total_violations` 0). PR evidence, not a committed test. The fold also makes `control-plane` own
every command refusal text: `CommandError::diagnostic` beside `EventError::diagnostic`, which the
C ABI calls (#1562a needs it).
