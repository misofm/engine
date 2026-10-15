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

## Record of #1562a (flip batch, written 2026-10-14)

### What shipped

- **D1.** Five exports on the control half, all unwrapped and counted as control allocations:
  `miso_engine_web_v1_control_request_ptr(handle, len)`, `_control_command(handle, len)`,
  `_control_response_ptr(handle)`, `_control_response_bytes(handle)` and
  `_control_revision(handle) -> u64` (the fifth is carry-forward 2). `control_command` runs
  `SessionState::command` on the staged bytes and decodes nothing. The one `miso.control.v1`
  message (`{requestId, frame}`, reply `{requestId, result, response, revision}`) travels on the
  node's port; the worklet stages, runs and copies the response out in `receiveControl` /
  `readControlCommand`, in the message handler, and the service step follows as after every message.
  The main-realm host has `control(frame)`. The staging is one new 16 KiB buffer
  (`PreparedBuffers::control_request`, charged in the bridge's retained rows), not the old command
  staging: that one holds 256 records of 48 bytes, 12 KiB, under `maximum_control_frame_bytes`.
- **Result codes.** One new browser result code, `RESULT_COMPILE_REJECTED = 10`
  (`compileRejected`), for a typed refusal before commit; the other `CommandError`s map to the
  existing codes. After any failure the response bytes are the refusal's diagnostic text. A refused
  transaction that the core encodes (`control.domain`) is still `RESULT_OK` with a non-ok frame, as on
  the C ABI. The mirrors (`abi_layout.rs` and its test, `check-abi-layout-v1.py`, its fixture, the
  regenerated JSON and `abi.ts`, `render-thread-exports.json`, `policy.rs`, both `.d.ts`,
  `check-web-audioworklet.sh`) carry the exports and the code.
- **D9.** `RuntimePreparer::refuse_rebuild(&self) -> Option<&'static str>` (default `None`; the
  C ABI's preparer takes every rebuild). `SessionState::command` asks it between `commit_live`'s
  rebuild verdict and the rebuild arm (`control.rs`), drops the token and returns
  `CompileRejected(web.apply.rebuild_pending)`. `WebRuntimePreparer` refuses unless its
  `allow_rebuild` switch is set, and only the test-support republish
  (`structural_command_for_test`, #1381a gates 2 and 4) sets it, for the one call. Nothing is
  stamped, prepared, queued or acknowledged before the refusal.
- **D5, D6.** Nothing to do: #1381a's preparer already attaches `controls::live_lanes`, and the
  core resolves every ramp from the committed model.

### Rules for the next slices

- **One request-ID allocator.** The frame carries the sender's request ID and the core's replay
  cache refuses a reused ID with different bytes; it answers a byte-identical resend
  `protocol.failure` (a refused frame consumes its ID, the rebuild-pending refusal included). So one
  allocator serves the whole `SessionState::command` endpoint. For #1563B: the SDK's
  `ControlTransactionQueue` (`#nextRequestId`) is that allocator, and every catalog or state read
  the SDK sends through `host.control()` takes its ID from the same counter; the host method never
  rewrites a frame; a retry of a refused frame takes a new ID.
- **Revision seed.** `control_revision()` (and the `revision` of every `miso.control.v1` reply,
  and the header of any reply frame) is the committed revision. #1563B constructs the queue with
  `initialRevision` from it after a successful boot.
- **edit_path.** It is in the response frame (`TransactionApplied.edit_path`), unchanged: the
  parity test and the admission tests decode `live`, `model_only`, `monitor` from it. `rebuild`
  never comes back from the browser before #1562b.
- **Reliable event lane (for #1570b).** Every committed transaction queues a `SESSION_COMMITTED`
  event on the core's reliable lane, and an undrained lane refuses the next commit with
  backpressure. #1562a has no reader: the tests drain through the test-support
  `drain_reliable_events_for_test`. Until #1570b drains the lane in the message handler, a page that
  sends more transactions than the lane holds is refused; the batch lands whole, so this is
  #1570b's to close before the flip is reachable.
- **Memory growth.** `control_command` allocates (the transaction, the classifier's model clone and
  JSON, the catalog stage), and on a small session the first live transaction grows the module's
  memory (reproduced: 46 to 48 pages). Growth is not a fault (#1332 D5, root ruling A of the batch 7
  CI round, which replaces this note's earlier "a moved `memory.buffer` is sticky
  (`reprepareRequired`)"): the worklet re-binds and continues. `rebindMemoryViews` re-cuts every
  persistent view over the memory at the end of every message-handler path that can call into the
  module (control, service, event drain, observation, track response, spectrum, source submit and
  seek, and the feed's attach and prepare-seek handler), and `process()` never creates a view: it
  compares the buffers, and on a mismatch renders silence for that quantum and counts it in the
  status's `staleViewQuanta`. Growth stays counted on the control side
  (`singleModeControlAllocations`); the render-locked allocation count stays exactly 0. A refused
  or failed call keeps its previous behaviour (a typed refusal is a reply, a trap or malformed
  message is sticky); only the growth stopped being sticky. The host accepts a status whose
  `memoryBytes` is the boot size or a larger whole number of pages.

### Evidence

- Gate 1 (`tests/control_admission.rs`): 22 rows, each its own boot, one transaction. The reply is
  `live`, the revision is +1, the plan epoch is unchanged, the parsed committed document holds the
  value on each lane, and the blocks are bit-identical to a plan prepared from the committed
  document and fed the same PCM from frame 0 for 18 rows. Deviations from the filed text:
  - `hpf`, `lpf`, the effect parameter and the EQ band gain are not bit-identical to a fresh plan at
    any block: the running plan smooths the coefficient through filter or effect state, so its
    output converges on the fresh plan's (the test asserts the largest difference at the end is
    under half that at block 8; measured 7e-4 for `hpf` and 1.3e-2 for `lpf` at block 38 of 40).
    Gate 8 holds the same four rows bit-equal to the C ABI at four rates, which is the claim that
    can hold.
  - `follows_mute` and the VCA member are rows only in a state where they change a gate or a gain,
    so each has a setup transaction (mute `t1`; a VCA offset of -3 dB). Otherwise the core answers
    `model_only`.
  - The VCA member row compares from block 20 (the edit has no length of its own and ramps over the
    session's fader length).
  - The `delay_samples` part of gate 1 is #1562b's; gate 11 holds its refusal.
- Gate 2: sample-exact on a session with no insert: 0 steps at the first sample of the block
  after the commit, 480 reaches the target at the 480th sample of the ramp (sample index 479), an
  absent length equals the explicit session length and is not a step, for fader, mute and pan.
  Solo is not in this table: its `mute_ms` resolution is the core's (#1559's tests), and the browser
  adds no resolution of its own to test.
- Gate 3: solo replies `monitor` with the revision unchanged, the document is untouched, the blocks
  equal a fresh plan under the same solo, and a later live fader edit keeps the solo. "A structural
  transaction keeps the solo" is #1562b's (a structural transaction is refused here).
- Gate 4: a valid fader edit, a solo and an out-of-domain pan are refused `control.domain`; the
  revision, document and the next 8 blocks equal a run without the call.
- Gate 5 (`tests/worker_admission_realtime.rs`, single-thread form; the workspace has no
  `bench_support::alloc`, so the binary registers the module's own counting allocator as
  `plan_exchange_threads.rs` does): 1,000 `SET_CONTROL` frames between 1,000 renders, the
  render-locked count exactly zero across every render, the control count moving across the frames.
  The browser halves (the counter in a real page) are gate 7's. The two-thread form is #1332's.
- Gate 6: `mute_ms` 5 then a mute with no length equals the mute with an explicit 240 and differs
  from the default; a strip's `pan_window_samples` 100 then a pan with no length equals an explicit
  100 and differs from the default.
- Gate 8 (`tests/control_parity.rs`, un-ignored): the browser leg sends the C ABI leg's frames
  through D1; the verdicts (path and revision, or the refusal's codes), every block's bits at 44.1,
  48, 88.2 and 96 kHz, the superseded count, every row of `CONTROL_LIST_GET` and `CONTROL_STATE_GET`
  and the `SESSION_SNAPSHOT_GET` bytes are equal. The script gained `follows_mute` off and on, a VCA
  member added, and `matrix_mode` of a matrix strip to pan (live) and back to the matrix (`model_only`
  on both hosts, as the effective 2x2 is unchanged). The rows are compared one by one because the C ABI leg's 4 KiB frames
  page differently.
- Gate 9: `cargo test -p capi` passes; `target/release/audit capi` reports `pcm_digest`
  `76525e7ef9e78cf4`.
- Gate 11 (`a_rebuild_frame_is_refused_before_commit_and_changes_nothing`): a `SET_CONTROL` on
  `delay_samples` returns `RESULT_COMPILE_REJECTED` with `web.apply.rebuild_pending\t$\n`; the
  committed revision, the plan epoch and the document are unchanged, no commit event is queued, the
  host takes the next live edit at the unchanged revision, and the next blocks equal a run that never
  saw the frame.
- Worklet harness (`scripts/test-web-audioworklet.mjs`): the control message stages the frame, runs
  the export, returns the response unchanged with the revision, carries a typed refusal without
  going sticky, refuses a frame the staging refuses, goes sticky on a malformed message; a command
  that grows the memory is re-bound and continues (every registered view is cut against the new
  buffer, the reply is whole, the next `process()` renders the engine's non-silent output through
  the new views, a later request is served, and the status reports the grown memory and a zero
  `staleViewQuanta`); `process()` on a moved memory with no handler since renders silence, is not
  sticky, counts the quantum, and resumes after the next handler; the host sends a copy with the buffer transferred, refuses a bad frame locally, and
  fails on a malformed reply.
- `check-sdk-headless.sh`: 25 failing subtests, the number #1381a recorded; all of them send the old
  command kinds or read their effects. #1563B (the SDK sends `SET_CONTROL`) restores the edit-driven
  ones; #1565A deletes or rewrites the tests of the old kinds (`round_trip.rs` in
  `tools/parameter-metadata` is one: it still sends kind 4 and fails on `unsupportedKind`, as it did
  at #1381a's head).
- **Classifier cost (carry-forward 4).** `classify_live_delta_with_monitor` on a 64-track session
  (one source, route and no insert per track, one fader edit), native release build, x86-64: about
  2.0 ms per call (200 calls); a whole `control_command` with a fader edit about 1.9 ms. It clones
  the model and serializes both the committed and the masked model to canonical JSON for each edit,
  so it is linear in the session, and the same cost lands in the worklet's message handler, on the
  audio thread, in the one-instance mode: in Wasm, several times native, against a 2.7 ms quantum at
  48 kHz and 128 frames. It is slow enough to glitch a large session. A successor is opened: replace
  the whole-document comparison of step 1 by a field-wise comparison of the masked model (the same
  verdict, no JSON), then re-time. #1562a does not change the classifier (a non-goal).

### Not done here

- Browser qualification rows (gate 7) in three browsers: no local browser runs in sprint mode; the
  rows and the `CONTROL_LIST_GET` row belong to the batch's qualification run.
- `npm run qualify` and `check-sdk-types.sh` (needs `npm ci`).

## Amendment (batch 7 review, coordinator 2026-10-14): parity rows owed by #1562b

The flip's parity test (`hosts/host-web/tests/control_parity.rs`) cannot yet compare edits that the
classifier sends to the rebuild arm, because #1562a refuses them in the browser with
`web.apply.rebuild_pending` while the C ABI rebuilds. #1562b, which deletes that refusal, adds these
rows to the parity script, each compared per step by reply bytes and per block by PCM bits: a route
into the output (gain and mute), the delay and multiband prepared bypass, `delay_samples`, and an
audio-rate effect parameter if a launch effect declares one. Until #1562b lands this is the one
known host difference, and it is a sequenced refusal, not a divergent result.

## Amendment (controls coordinator, batch 8, 2026-10-14): for #1562b

- **Meter strip order after a structural edit.** Since #1564C the SDK takes strip order from the
  core's control list and re-reads it when the structure epoch moves, but the browser meter feed
  keeps the strip IDs of the boot-time shape and `meterProjection` refuses a frame of another
  shape. Nothing can move strips before #1562b, because the browser refuses rebuild-path frames.
  #1562b's gates include a rebuild that adds or removes a strip, after which meters and observation
  follow the new list order.
- **Watermark rebuild leg.** #1349's gate 1 ran with a live edit (rebuild refused). #1562b adds the
  rebuild leg: a rebuild, then a live edit before the swap; the browser status words stay at the
  initial revision until the swap block and then equal the C ABI's `miso_engine_v1_plan_watermark`.
- **Named effect rows (batch 8 review).** `sdk/src/core/live-controls.ts` (`#effectRowId`) resolves
  an effect row's name through the session the SDK built, checked against the booted bytes. After
  #1562b a rebuild can change an insert's effect, and a named edit could land on another effect's
  row with the same number. #1562b resolves named effect rows from the control list's section
  headers (the effect ID) and re-reads them when the structure epoch moves.
- **Meter frames and the structure epoch (batch 8 review).** `pollMeters` (`sdk/src/core/boundary.ts`)
  re-reads the strip order only when a frame's `trackCount` differs, so after #1562b a structural
  edit that keeps the count would name the wrong strips. #1562b carries the structure epoch in the
  meter header (or an equal signal) and re-reads the strip order when it differs.

## Amendment (controls coordinator, batch 10, 2026-10-14): #1562b

Checked against `12d4a4f9d`.

- **#1562b spans both lanes.** Lane W is the only lane that edits `hosts/host-web/**`; lane K owns
  the crates and the `sdk/` files outside `hosts/`. Split, in this order:
  - **#1562b-W:** delete D9 in the browser (`WebRuntimePreparer::refuse_rebuild`'s override,
    `allow_rebuild`, `REBUILD_PENDING` at `hosts/host-web/src/lib.rs:4802`, gate 11 and its test),
    gate 1's `delay_samples` row, gate 8's rebuild part with the batch 7 parity rows, the watermark
    rebuild leg, and the meter header's structure epoch with its ABI layout rows and the
    regenerated `sdk/src/generated` copies.
  - **#1562b-K:** delete the then-unused hook `RuntimePreparer::refuse_rebuild`
    (`crates/control-plane/src/compile.rs:676`) and its call in `SessionState::command`; the SDK's
    named effect rows from the control list's section headers (`sdk/src/core/live-controls.ts`);
    and `pollMeters`' re-read of the strip order on the structure epoch (`sdk/src/core/boundary.ts`).
    It reads the header field that #1562b-W adds.
- **Dependencies of #1562b:** #1562a, #1381b, #1290 (a) and #1290 (d+e), so the browser's first
  reachable rebuild keeps meters, observation taps and the spectrum stream continuous (D15-14); and
  #1280a, #1280c, #1281 and #1282, so it does not restart every strip that holds an effect. A strip
  in the restart set is not ducked until #1324 (open); the C ABI behaves the same today, so parity
  holds.
- **A gap no chain slice closes.** A structural transaction that also changes a strip's live input
  value (trim, polarity, a filter) restarts that strip's input section: the input join carries a
  section only when every value is equal (`same_input_section`, `crates/host-core/src/prepare.rs:2726`),
  and there is no input twin of #1277 D5's retarget. It is not a #1562b blocker (the C ABI does the
  same). Proposed successor: *Carry, then retarget a strip input section across a plan swap*
  (lane K, `prepare.rs` and `live_delta.rs`), after #1280c.

## Ruling (root, 2026-10-14): #1562b order

#1562b-W lands first, then #1562b-K, with the dependencies in the batch-10 amendment (#1290 d+e,
#1280a, #1280c, #1281, #1282).

## Amendment (controls coordinator, batch 12, 2026-10-15): #1562b

Checked against `ea336c039`.

- **Dependencies at this head.** Landed: #1562a, #1381b, #1290 (a), #1280a, #1280c, #1281, #1282,
  #1283, #1284, #1327a, #1327b and #1622. Still open: #1290 (d1) and (d2) (see #1290's batch 12
  amendment; (e) needs no code). Added: #1584 D6a and D6b. Without them, the first browser rebuild
  that can be reached loses an acknowledged subscription on a restarted effect owner, and ends a
  removed owner's subscription silently. #1630 (the input-section carry) is still not a blocker,
  because the C ABI behaves the same.
- **Anchors moved.** `REBUILD_PENDING` is at `hosts/host-web/src/lib.rs:5260`. The
  `WebRuntimePreparer` override is at `:5301`, `allow_rebuild` at `:5278`, and
  `structural_command_for_test` at `:2107`. The core hook is still `compile.rs:676`, and it is
  called at `crates/control-plane/src/control.rs:1260`. Gate 11's test is
  `hosts/host-web/tests/control_admission.rs:476`.
- **#1562b-W, additions to the batch 10 list:**
  - Delete `structural_command_for_test` and the `allow_rebuild` switch with D9. Their tests
    (#1290a, #1381b, #1584 D6b) then drive the rebuild through `control_command`. Delete each
    test that #1562b-W supersedes in the same slice.
  - The structural part of gate 3 ("a structural transaction keeps the solo"), which #1562a's
    record deferred. The batch 10 list does not have it.
  - Meters and observation follow the new strip order after a rebuild that adds or removes a strip
    (batch 8). This includes `meterProjection`'s fixed strip IDs (`sdk/src/browser/measurement.ts:208`,
    `:302`), which is lane W's.
  - The structure epoch in the meter header. `WebMeterHeader` (`hosts/host-web/src/lib.rs:937`) has
    a `reserved_pad` word that is always zero, or the header can grow at its end. The ABI layout rows
    and the regenerated `sdk/src/generated` copies change with it.
- **#1562b-K, additions:**
  - `sdk/src/core/observation-subscriptions.ts` addresses a tap by strip and effect index. After a
    rebuild, it maps its managed subscriptions to the new control list order. It ends a managed
    subscription on #1584's `OBSERVATION_ENDED` event and reports this to its subscribers.
  - It deletes the core hook only after #1562b-W has deleted the browser's override, so the two
    lanes compile at every merge.
- **Frozen decisions.** D1-D6 and the batch 10 split hold.
- **No edit under `crates/engine/src/realtime`.**

## Decision record: #1562b-W (batch 12)

Lane W. Checked against the batch 12 head `0c6f35a1d`.

### What shipped

- **D9 is gone in the browser.** `REBUILD_PENDING`, `WebRuntimePreparer`'s `refuse_rebuild`
  override, the `allow_rebuild` switch and `structural_command_for_test` (with
  `native_staging::structural_command`) are deleted. The browser prepares and publishes a
  rebuild like the C ABI: a transaction's ack still comes only after the preparer left the plan's
  tables (`run_command`). The core hook `RuntimePreparer::refuse_rebuild` and its call in
  `SessionState::command` stay for lane K to delete. Gate 11 and its test are deleted.
  The #1290a, #1381b and #1584 D6b tests, and the plan-exchange and structural-realtime integration
  tests, drive the rebuild through `control_request_mut` and `control_command` (the unit module's
  `Rig::entry`, `native_staging::control_command`).
- **Structure epoch in the meter header.** `WebMeterHeader` grew at its end: `structure_epoch: u64`
  at offset 80 (88 bytes). `run_command` stamps the newest committed plan's header (the pending
  replacement's, else the rendering plan's) with `SessionState::structure_epoch()` (a new read-only
  accessor next to `committed_session`; the only edit under `crates/control-plane`), and the boot
  stamps the boot plan's. The worklet posts it as `structureEpoch` in every `miso.meter.v1` frame
  (two 32-bit words, no BigInt) and in the `miso.observationmap.v1` reply (read from the rendering
  plan's header). Mirrors: `abi_layout.rs`, `check-abi-layout-v1.py` and its fixture, the
  regenerated `sdk/assets` and `sdk/src/generated/abi.ts`, both `.d.ts`, the host's frame validator.
  Lane K reads `meterHeader.structureEpoch` in `pollMeters`.
- **Maps follow the plan.** The worklet read `sources` and the observation bindings once at boot;
  both are now read from the module at every `miso.sessionmap.v1` / `miso.observationmap.v1`
  request (`readSourceRows`, `readObservationBindings`), so a rebuild that adds a source, a strip or
  an effect is seen. The exports those reads call after construction are now `render_locked` and
  listed (wrapped) in `render-thread-exports.json`; `check-session-map-shape.py` follows
  `readSourceRows`.
- **Browser SDK follows the strip order.** `createBrowserStripOrders` (sdk/src/browser/live-controls.ts)
  keeps the last eight strip orders by structure epoch. A meter frame is named by the order of the
  epoch it carries (`createMeasurementFeeds(host, orders, available)`, in-order delivery, a frame
  whose list the page never held is dropped), and the observation map is paired with the order of
  the epoch the reply names. No fixed boot strip IDs remain in `meterProjection`'s callers.
- **Browser qualification.** A new page row, `structural-apply` (`runStructuralApplyQualification`):
  a `delay_samples` edit through the SDK is the rebuild path, the applied revision is not early, the
  output is the input delayed by exactly the edit (computed, not read back), the meter stream is the
  successor's with one structure epoch, and the render-locked count is 0 (11 workloads now). Four
  mutations: `structural-apply-path`, `-early`, `-output`, `-meter`.

### Departures and notes

- Gate 1's `delay_samples` row is its own test (`a_delay_samples_edit_is_a_rebuild_and_renders_as_a_fresh_plan`),
  because a rebuild's reply and swap differ from the live rows' loop.
- The rebuild rows of the batch 7 parity amendment are scripted: output route gain and mute,
  `delay_samples`, the multiband and delay prepared bypass. **No launch effect declares an
  audio-rate parameter** (`AutomationRate::Sample` has no user in a launch effect), so that row has no
  edit.
- `sdk/test` repairs that the browser change forces (the module accepts the rebuild in headless
  and browser alike): `control-list-evals.mjs` (rebuild-only row commits as `rebuild`),
  `console-evals.mjs` (the delay and multiband lift render the unbypassed session), and the fake
  `observationMap` replies carry `structureEpoch`. `sdk/README.md` loses one sentence.
- Not done by lane W: the SDK's named effect rows and `pollMeters`' re-read (lane K), the managed
  observation subscription remap (lane K).

### Gates

`control_admission.rs`: `a_delay_samples_edit_is_a_rebuild_and_renders_as_a_fresh_plan` (gate 1 row),
`a_structural_transaction_keeps_the_solo` (gate 3, structural part). `control_parity.rs`:
`both_hosts_give_the_same_result_for_the_same_control_edits` (now with the six rebuild steps),
`every_scripted_edit_is_heard`, and `the_status_words_carry_a_rebuilds_watermark_the_c_abi_reports`
(#1349 rebuild leg, four rates, equal to `miso_engine_v1_plan_watermark` after every block).
`tests.rs`: `the_headers_structure_epoch_is_the_control_lists`, the epoch assertions of
`a_strip_count_change_publishes_the_successors_frame_from_the_adoption_block`, and the layout test.
Worklet harness: the epoch in the frame (including past 2^32 and the adoption block), the frame
validator, the re-read session and observation maps. SDK evals: `measurement-evals.mjs` (three new:
frames named by their epoch, in order; an unnameable frame is dropped; the engine follows a rebuild).
Browser: `npm run qualify` with `--sdk-root`, `--check-matrix`, `--self-test-mutations` passes in
chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 (the matrix rows are unchanged).

## Decision record: #1562b-K (batch 12)

Lane K. Checked against `2b79cec44` (batch 12a + #1562b-W).

### What shipped

- **The core hook is gone.** `RuntimePreparer::refuse_rebuild` (`crates/control-plane/src/compile.rs`)
  and its call in `SessionState::command` are deleted. Both hosts take every rebuild: the host-web
  rebuild tests (`control_admission`, `structural_apply_realtime`, the unit `structural_apply`) and
  the capi tests pass without the hook.
- **A meter frame names its own strips.** `WasmBoundary.pollMeters` reads the structure epoch in the
  meter header (offset 80, #1562b-W) and names the frame by the control list of that epoch. The
  boundary keeps the last eight orders by epoch; a frame whose list it never read and cannot read now
  is dropped (the browser's `createMeasurementFeeds` does the same). `MeterFrame` gains
  `structureEpoch`, `trackIds` and `submixIds` (additive), so a frame that a rebuild leaves with the
  same strip count is still named right, and a window the old plan completed is named by the old list
  even after the SDK read the new one.
- **Named effect rows follow the core's section headers.** `ControlStripOrder` (the one cached read of
  the list) now also holds the effect of each console slot and insert, from the list's section
  headers (`stripEffectOf`). `LiveControlEdits` resolves a named effect row from it first, from the
  booted session second. An edit that names an effect row remembers (effect, row name, strict or
  not); `effectRowResolver`, installed as the transaction queue's new `prepare` hook on both engines,
  checks it against the list just before the frame is encoded (a probe of the structure epoch, the
  list only when it moved; edits with no effect row cost nothing). A named section follows its header;
  a builder typed for an effect the insert no longer runs is refused with `MisoUsageError`, and
  nothing is sent. The queue draws the transaction's request ID after `prepare`, because the core takes
  IDs in ascending order and `prepare` reads the list with IDs of its own.
- **Live controls follow the strips.** `EngineLiveControls.syncStructure()` and
  `LiveControlTransport.stripOrder` re-read the order when the epoch moved; an observation record is
  re-addressed to its strip's index in the current list just before it is sent, so a manual
  `observe()` edit built before a rebuild is also right. The headless `liveControls()` and
  `withSession()` drop the booted session from the layout once a rebuild moved the strips (the engine
  is then no longer that session); the browser's memoized controls drop it in `update()`.
- **Managed subscriptions follow a rebuild and end on `OBSERVATION_ENDED`.**
  `ObservationSubscriptionOwner` takes the event through the new transport member
  `observationEnded` (the headless engine and the browser engine decode the reliable lane with
  `decodeObservationEnded`, `sdk/src/core/observation-ended.ts`). The event, one per commit with every
  key, drops the bindings and tells each subscription once: `onEnd({ revision, ended, remaining })`;
  `handle.ended` lists them; a subscription left with no selection is closed. An event that arrives
  while an operation is in flight is applied when it ends. Arm and disarm records take their strip
  index and effect index from the newest control list and observation map (`syncStructure()`, and
  `#remap()` of every held binding), not from the address they were subscribed with.

### Departures and notes

- **Files outside `sdk/src/core` and the core hook** (mechanical or forced by the above):
  `sdk/src/headless/engine.ts` and `sdk/src/browser/{engine,live-controls}.ts` (the transport wiring),
  the three index barrels (`ObservationSubscriptionEnd`), `sdk/README.md`, and the test fakes that
  must carry `onControlEvent` (hosts) and `syncStructure` (live controls). The browser's
  `createBrowserStripOrders` is now one reader for each host, shared by the queue's check, the live
  controls and the feeds.
- **A structural fixture.** The SDK has no builder for a structural edit, so
  `hosts/host-web/tests/structural_frames_fixture.rs` writes five frames with the protocol codec
  (`fixtures/structural-frames.json`: add a first strip, replace the last strip, swap an insert for
  another effect, remove an insert, remove the insert before it), and `structure-evals.mjs` sends
  them to a real core. It is a wire format, so the byte comparison is the gate.
- **The queue does not learn a foreign commit's revision.** A structural frame sent past the queue
  (as the evals do) leaves the queue one revision behind and the next transaction is refused. The
  evals call `queue.reset(controlRevision())`. A transaction API for structural edits (#1296) goes
  through the queue and has no such gap.
- The existing `set checks only encodability` eval now asserts that a named insert row resolves from a
  document-text boot (it was a refusal before) and that an unknown insert is still refused.

### Gates

`sdk/test/structure-evals.mjs`: `a rebuild that adds a strip: frames before name the old strips,
frames after the new`; `a window the old plan completed is named by the old list even after the new
list was read` (red when the frame is named by the newest cached order); `a rebuild that keeps the
strip count and changes the strips is named by the new list`; `headless` and `browser: a row named
before an insert changed effect lands on the new effect's row` (the compressor's `mix` is control 7,
the soft clip's 3; the typed builder and a name the new effect lacks are refused and send nothing);
`the end of a tap is reported once, with the rest of the subscription kept (headless)`; `one commit
that ends every tap of a subscription closes it and says so once`; `removing the insert ends its tap
the same way`; `a tap that survives a rebuild keeps publishing and is disarmed at the strip's new
index`; `an effect that moved to another index is disarmed where it is now` (red without `#remap`);
`through the browser transport, whose live controls were made before the rebuild` (red without
`syncStructure`/the record re-addressing). `check-sdk-headless.sh` 543 pass; `check-sdk-types.sh`,
`check-sdk-generated.sh`, `sdk-package.sh check` pass on `target/ci/b12k-artifacts`; chromium
151.0.7922.34 qualification with `--sdk-root`, `--check-matrix`, `--self-test-mutations` passes (all
render-allocation rows 0). Rust: control-plane (`test-support`), capi and host-web
(`host-web/test-support`) tests, clippy `-D warnings` (host-core with no features, default and all
features, control-plane, capi, host-web), fmt, `realtime-policy` (141 regions) and
`check-bench-policy.sh` pass.

### Open

- Between a rebuild's commit and the block that adopts it, `observationMap()` describes the plan that
  renders, and a kind 7/8 record is queued on the newest plan. The evals render a block before they
  subscribe. Which plan's index space the record addresses in that window is the core's to rule
  (successor, with #1589).

## Fold (batch 12b review)

- **M1.** The "Open" item above (which plan's index space a record addresses between a rebuild's
  commit and its adoption) is closed on the SDK side. The core admits against the newest plan
  (`admit_on_newest_plan`), the strip index comes from the newest list, and the effect index came
  from `observationMap()`, which describes the plan that renders. `ObservationMap` now carries the
  `structureEpoch` of that plan (the browser reply's own; the headless engine reads the meter header
  of the rendering plan and names the strips by the list of that epoch). Subscribe, update and close
  compare it with the epoch of the list used to address the record (`syncStructure()` returns it). On
  a mismatch an engine with a render wait waits for a render and reads again (at most 8 renders and
  the engine's request deadline); the headless engine, and a wait that ends, refuse with the typed
  retryable `backpressure` error (`sdk.observation.structure_behind`). `submitManagedAt(epoch, ...)`
  checks the list epoch again at the send, so a commit between the check and the send is refused, not
  misaddressed. No record is sent in the window.
- **Gates** (`sdk/test/structure-evals.mjs`, fixture frame `swap_t3_multi_and_limiter` from
  `structural_frames_fixture.rs`: two effects with the same tap change indices): subscribe, close and
  update in the window are refused and arm or disarm the right tap after a render; an owner with a
  render wait sends the record of the adopted plan; a wait that gives up sends nothing. The browser
  engine: `measurement-evals.mjs`, the map's epoch 1 against list epoch 2. All red on the previous SDK.
- **m2.** Meter frames the SDK cannot name by epoch are counted. The browser feed adds their windows to
  `lossCount` of the next update (saturating) and sets the loss bit once; the headless `MeterFrame`
  carries `unnamedWindows`. Eval: `a meter frame whose list the page cannot name is dropped, counted,
  and the stream goes on`.
- **m3.** `BindingState.armRevision` is the committed revision read just before the arm was sent; an
  `OBSERVATION_ENDED` event with a revision at or before it is ignored (it ended an earlier arm).
  Eval: `an ended event older than the arm is ignored`.
