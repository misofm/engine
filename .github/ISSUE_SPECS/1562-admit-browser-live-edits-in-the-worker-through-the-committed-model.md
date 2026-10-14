# Admit browser live edits in the Worker through the committed model

Slice S10a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C4, C5 and C8). This is #1382, amended in place by decision 17; stream H of decision 15 (D15-1,
D15-2, D15-10, D15-11, D15-17). Code anchors verified on `main` `df585e503`. Feature sprint mode
(owner, 2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or
mutation runs.

Decision 17 splits the browser half of the program in three: this slice (the Worker commits
`SET_CONTROL` and serves the control list through the core), #1563 (the SDK sends every value as
`SET_CONTROL`), and #1565 (host-web's own command path is deleted). They land in that order.

## Product outcome

In the browser, a control edit is a `SET_CONTROL` in a `SESSION_TRANSACTION_APPLY` frame. The
Worker hands the frame, unchanged, to the same `control_plane::SessionState::command` that the
C ABI's `miso_engine_v1_submit_command` calls. A live edit advances the committed revision, its
reply carries `{revision, path}`, and render applies it from the plan's live lanes. The control
list and the control state come back through the same entry, as the same bytes as on the C ABI.
Solo is the core's monitor overlay. Nothing in the browser lowers, checks or composes a value.

## Context

- **Admission runs on the worklet thread today.** `miso_engine_web_v1_command_submit`
  (`hosts/host-web/src/ffi.rs:4015`) calls `AudioWorkletEngineHost::submit_commands`
  (`hosts/host-web/src/lib.rs:3477`), then `admit_commands` (`:4798`) and `admit_commands_staged`
  (`:5051`): host-web's own 17 kinds (`:837-949`) and its own solo, VCA and follow composition
  (after #1548, through the core's check and building blocks, over host-core's mirrors). It writes
  plan cells and never updates a session model. The static gate holds that export to the
  allocation-only call graph (`scripts/check-web-audioworklet.sh:494-511`).
- **The C ABI's one entry.** `miso_engine_v1_submit_command` (`crates/capi/src/ffi.rs:685`) runs
  `SessionState::command` (`crates/control-plane/src/control.rs:1082`): it decodes any command
  frame, applies a transaction live or as a rebuild, and serves every query. After #1551, #1552, #1553,
  #1554, #1556, #1557, #1558 and #1559 it serves `CONTROL_LIST_GET`, `CONTROL_STATE_GET` and
  `SET_CONTROL`, with solo in its monitor overlay.
- **The Worker holds that state.** *Swap and retire browser plans through the Worker's service
  loop* (#1381, D1) boots `SessionState` in the Worker through the browser's `WebRuntimePreparer`
  (#1400 D5). *Replace the running browser session in the Rust host* (#1290) makes its rebuild arm
  work in the browser. *Run the browser control plane in a Worker and keep the AudioWorklet
  render-only* (#1332) sets the `worker` and `single` modes and the page-to-control-plane port
  (its D7).
- **#1382 as filed** lowered the 48-byte records into a transaction in the Worker (its D2) and
  composed solo in the shared commit (its D3). Decision 17 drops the lowering (thin SDK builders,
  #1057 section 3.2 option A) and moves the overlay to #1559.
- **#1293 plans a narrower export.** *Export transaction apply and anchored seek from the browser
  engine module* (#1293, D2-D3) plans `miso_engine_web_v1_apply` for transaction frames only and
  a fixed outcome record. Ruling C4 needs the list and state queries on the same path, so this
  slice adds one generic entry (D1). See the open question for root on #1293.
- **Live lanes.** The browser plan's live lanes come from the boot word
  `live_control_command_queue_records` (`hosts/host-web/src/lib.rs:1251`). Ruling C6 ("live lanes
  on every plan") makes the core choose them; #1555 adds `controls::live_lanes(model)` and keeps the browser on its boot word (#1555 D4).
- **The ramp table.** `SessionRamps` (`hosts/host-web/src/lib.rs:1549`) is built once at boot;
  root ruling of 2026-10-12 (#1382 D6 as filed) says the browser must resolve an absent ramp from
  the committed model.

## Decisions frozen for this slice

- **D1. One entry for every command frame.**
  - New exports, all on the control half (the Worker in `worker` mode, the worklet's message
    handler in `single` mode): `miso_engine_web_v1_control_request_ptr(handle, len) -> u32` (a
    staging buffer for one frame of `len` bytes, at most the boot's
    `maximum_control_frame_bytes`; `0` with a typed result otherwise),
    `miso_engine_web_v1_control_command(handle, len) -> u32` (runs
    `SessionState::command(staged bytes, capacity)` and returns `RESULT_OK` or the mapped
    `CommandError`), `miso_engine_web_v1_control_response_ptr(handle) -> u32` and
    `miso_engine_web_v1_control_response_bytes(handle) -> u32` (the response frame, valid until
    the next call). On an instance with no control half each refuses with
    `RESULT_REFUSED_LIFECYCLE` (a pointer export returns 0).
  - The page-to-control-plane port (#1332 D7) gains one internal message, `control`, that carries
    the frame bytes and a message ID; its reply carries the response frame bytes unchanged and the
    export's result word. The message is internal to the SDK (D15-11).
  - The browser decodes nothing in the frame. Any frame `SessionState::command` accepts is
    accepted: transactions, `CONTROL_LIST_GET`, `CONTROL_STATE_GET`, `SESSION_SNAPSHOT_GET`, the
    counter and capability queries.
- **D2. Nothing is lowered** (replaces #1382 D2 as filed). The Worker passes the
  `SESSION_TRANSACTION_APPLY` frame to `control-plane`. The ramp of each edit is the edit's own
  `ramp_samples` field, resolved by the core (#1394 D5).
- **D3. The overlay is the core's** (replaces #1382 D3 as filed). Solo is a `SET_CONTROL` on strip
  own row 2, in #1559's overlay inside the Worker's `SessionState`. Observe subscribe and
  unsubscribe (kinds 7 and 8) and the meter lease stay browser monitor operations (ruling C9); the
  later browser-feature phase moves them into the core.
- **D4. The acked-batch rule** is the core's (#1559 D8): every fallible check of the whole frame
  runs before the first write; a refusal leaves model, revision, overlay and cells unchanged; live
  values are latest-target cells, never refused for room.
- **D5. The browser preparer takes the core's live lanes.** `WebRuntimePreparer` attaches
  `controls::live_lanes(model)` on every plan, as the C ABI does. The boot word no longer chooses
  a lane; until #1565 it only sizes the old command staging.
- **D6. The ramp table follows the model** (keeps #1382 D6 as filed, root 2026-10-12). Every
  `SET_CONTROL` resolves its ramp against the committed model inside the core.
  `SessionRamps` serves only the old command path, and #1565 deletes it with that path.
- **D7. The old path stays, unchanged, until #1565.** Kinds 1-6 and 9-17 keep working on the
  worklet in this slice, because the SDK still sends them until #1563. A value set through them
  does not reach the committed model; no SDK path sends both forms (the SDK's transaction API,
  #1296, lands after #1563). This order is forced by the SDK dependency; it is not a placeholder.
- **D8. Mirrors.** The new exports join every export list: the ABI layout document and its
  generator (`tools/parameter-metadata/src/abi_layout.rs`), `sdk/src/generated/abi.ts` and
  `sdk/assets/miso-engine-v1-abi-layout.json` (regenerated, never hand-edited),
  `scripts/check-abi-layout-v1.py` and its self-test fixture,
  `tools/realtime-policy/render-thread-exports.json` (they are control-half exports, not render
  exports), and the shipped host's `.d.ts`.

## Deliverables

1. D1 in `hosts/host-web/src/{ffi.rs,lib.rs}`, the control Worker script #1332 adds and
   `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (single mode).
2. D5 in host-web's preparer.
3. D8's mirrors.
4. The native tests, the integration test binary `hosts/host-web/tests/worker_admission_realtime.rs`
   (gate 5) and the extended parity test (gate 8).

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/`,
  `hosts/host-web/qualification/` (the gate rows of gate 7).
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`,
  `tools/realtime-policy/render-thread-exports.json`.
- Regenerated only: `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`.
- `sdk/src/browser/shipped-host.d.ts` (the new message and exports only).
- `scripts/test-web-audioworklet.mjs` (the fake exports and the `control` message).

## Non-goals

- No SDK API change (#1563, #1564). No deletion of the old command path (#1565).
- No change to any classifier row, family or record (the core owns them).
- No observation or meter-lease change (ruling C9).

## Objective gates

1. **Every live row commits to the model.** Native host-web test, control half and render half on
   two threads. One transaction per row of every live family, through D1: strip fader, mute, pan,
   matrix, `matrix_mode`, trim, polarity, HPF, LPF, one block-rate effect parameter, one EQ band
   gain, one bypass, send gain, mute and matrix, `follows_mute`, VCA offset and mute, a VCA member
   added, on tracks and on a submix. For each: the reply's path is `live`, the revision advances
   by 1, the plan epoch is unchanged, the committed document (`SESSION_SNAPSHOT_GET`) holds the
   value, and after the ramp every block is bit-identical to a plan prepared from the committed
   model with the same overlay, fed the same PCM from frame 0. A `SET_CONTROL` on `delay_samples`
   is path `rebuild` and renders as the fresh plan after adoption.
2. **Ramps carry end to end.** For fader, mute, pan and solo: `ramp_samples` 0 steps at the next
   block; 480 reaches the target after 480 samples; absent ramps over the session's
   `control_smoothing` length (solo: `mute_ms`).
3. **The overlay.** Solo replies `monitor` with the revision unchanged; soloing track 2 mutes the
   others and closes a following send; a structural transaction keeps the solo.
4. **All or nothing.** A transaction of a valid fader edit, a solo and an out-of-domain pan is
   refused with `control.domain`; model, revision, overlay and cells are unchanged; the next 8
   blocks are bit-identical to a run without the call.
5. **Realtime.** Integration test binary `hosts/host-web/tests/worker_admission_realtime.rs`. It
   links `bench_support::alloc` and calls `assert_installed()` first. The render thread's
   thread-scoped counters read `allocations == 0 && frees == 0` around every render call while the
   Worker thread runs 1,000 `SET_CONTROL` transactions. In the browser,
   `miso_engine_web_v1_render_allocation_count` stays 0 on both legs (#1333), and on the
   non-isolated leg `singleModeControlAllocations` grows with the control messages.
6. **The ramp table follows the model** (#1382 gate 9 as filed). Leg 1: commit `SET_CONTROL`
   session `mute_ms` 5, then a mute with no `ramp_samples`: it ramps over 240 samples at 48 kHz.
   Leg 2: commit `SET_CONTROL` on a strip's `pan_window_samples` 100, then a pan with no
   `ramp_samples`: it ramps over 100 samples.
7. **Browser qualification.** The live-control rows of the browser qualification pass unchanged in
   chromium, firefox and webkit, on both legs (the SDK still uses the old path), and one new row
   sends a `control` message with a `CONTROL_LIST_GET` frame and checks the decoded first row.
8. **Cross-host parity, extended** (`hosts/host-web/tests/control_parity.rs`, #1560). The browser
   leg sends `SET_CONTROL` through D1 instead of command records. The script gains the edits that
   are now expressible on both hosts: `follows_mute` off and on, a VCA member added, `matrix_mode`
   to matrix and back. After the script, every page of `CONTROL_LIST_GET` and the
   `SESSION_SNAPSHOT_GET` bytes are byte-identical across the two hosts.
9. **The C ABI is unchanged.** `cargo test --locked -p capi` passes, and `target/release/audit capi`
   reports the merge base's `pcm_digest` (PR evidence).
10. **Commands:** `cargo test --locked -p host-web --features host-web/test-support` (runs gates
    1-6 and 8), `cargo test --locked -p control-plane --features test-support`,
    `cargo test --locked -p capi`, `cargo build --locked --release -p audit -p capi &&
    target/release/audit capi`, `bash scripts/check-web-audioworklet.sh`,
    `bash scripts/test-web-audioworklet.sh`, `python3 -B scripts/check-abi-layout-v1.py`,
    `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
    `bash scripts/check-sdk-types.sh`, and `npm run qualify -- --artifacts ... --sdk-root ...
    --browser <b> --check-matrix --self-test-mutations` in `hosts/host-web/qualification` for
    chromium, firefox and webkit.

## Test value

- Gate 1 turns red if a browser edit reaches the cells without reaching the committed model, if a
  family lowers differently in the Worker, or if a rebuild-only row is live in the browser.
- Gate 2 turns red if the browser drops a per-edit ramp or reads absent as a length.
- Gate 3 turns red if the browser's solo advances the revision or is lost at a swap.
- Gate 4 turns red if any write precedes a later refusal in the Worker.
- Gate 5 turns red if control work runs on, or allocates on, the render thread, or if single
  mode's control allocations go uncounted.
- Gate 6 turns red if any browser path resolves an absent ramp from a table built at boot.
- Gate 7 turns red if the shipped module's `control` message does not reach the core.
- Gate 8 turns red if the two hosts list different controls, hold different committed documents,
  or render differently for an edit that only the core path makes expressible.
- Gate 9 turns red if the browser work changes C ABI behaviour.

## Dependencies

- #1551, #1552, #1553, #1554, #1555, #1556, #1557, #1558, #1559 (the core serves every row and chooses the
  live lanes) and #1560 (the parity test).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Replace the running browser session in the Rust host* (#1290): the rebuild arm in the browser.
  Its own dependency on #1562 (formerly #1382) is a question for root (see the report).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Report each transaction's edit path in its response* (#1313).
- *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400).

## Supersedes and shrinks

- Amends itself (#1382, *Admit browser live edits in the Worker through the committed model*): see
  the amendment record.
- Supersedes *Make a send's follows_mute live in the browser* (#1342): gate 1 makes the toggle live
  in the browser through the core.

## Amendment (decision 17, root 2026-10-13)

**Single instance first.** This slice targets `single` mode on today's one-instance module.
The four `control_*` exports run in the worklet's message handler, never in `process()`. The
`control` message travels on the node's existing port. #1332 later deploys the same exports in the
Worker, with no other logic. Dependencies: #1332 is removed; #1381 means #1381a; #1290 moves to
#1562b. **D7 is withdrawn**: the old command path is deleted in the same batch (#1565A), so it
never runs beside `SessionState`.

The slice is split. Both pieces stay in this issue.

- **#1562a**: D1-D6, D8, and D9 below.
  - **D9. Rebuild pending (root ruling B).** Until #1562b, the browser preparer's successor entry
    that the command path uses refuses a frame that classifies as a rebuild with
    `web.apply.rebuild_pending`. The refusal comes before commit: no ack precedes it, nothing is
    queued, and the model, revision and plan are unchanged. The browser has no in-place structural
    edit today, so this withdraws nothing. Test-support republish (#1381a's gates) keeps working.
  - Gates: 1 (the live rows), 2-6, 8 (all but its rebuild part) and 9. Gate 5's browser half and
    gate 7 run on the non-isolated leg only.
  - Gate 7 reads: "The live-control rows of the browser qualification pass through `SET_CONTROL`
    in chromium, firefox and webkit", plus the `CONTROL_LIST_GET` row.
  - New gate 11: a `SET_CONTROL` on `delay_samples` is refused with `web.apply.rebuild_pending`;
    the model, revision, plan epoch and cells are unchanged, and the next 8 blocks are
    bit-identical to a run without the call. It turns red if a rebuild-path frame reaches
    `SessionState`'s rebuild arm (a plan swap with no companion pairing) before #1562b.
  - Dependencies: as filed, with #1381a for #1381, and without #1332 and #1290.
- **#1562b**: gate 1's `delay_samples` row, the rebuild part of gate 8, and the deletion of D9's
  refusal and gate 11. Dependencies: #1562a, #1381b, #1290a.

**Flip batch 6W (root ruling A1).** #1381a, #1562a, #1570b, #1563B and #1565A land together, in one batch of lane W. Each slice keeps its own commits and tests and is green at its own boundary; the batch runs the full CI-equivalent gate set once. If one slice fails, the batch stops: `main` never gets a partial flip, so it never has two writers of one plan's revision gate and cells. No transitional API lends `SessionState`'s producers or revision to the old command path.
