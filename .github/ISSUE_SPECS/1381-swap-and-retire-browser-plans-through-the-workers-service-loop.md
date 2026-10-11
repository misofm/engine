# Swap and retire browser plans through the Worker's service loop

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This slice follows *Run the browser control plane in a Worker and keep the AudioWorklet
render-only* (#1332) and *Move browser source submission and seeks into the Worker* (#1387). It
builds on the adapter preparation hook of *Prepare through an adapter-supplied preparer in the
control-plane crate* (#1400).

## Product outcome

The browser engine holds its plan the way the C ABI does.
- The Worker owns the control plane's session state (`control_plane::SessionState`: the committed
  model at revision 0, the source producers, the plan publisher and the retirer).
- The worklet renders through `RealtimePlanOwner`. A published successor is adopted at a block
  boundary, together with its render-side companions, and the clock continues.
- The Worker runs the service step continuously through a new export. Every retired plan and its
  companions are reclaimed in the Worker, never on the audio thread.
- The Worker reads the applied-revision watermark and its counters through a new export.

Rendered bits do not change. This is the substrate that *Replace the running browser session in
the Rust host* (#1290) publishes through.

## Context

- **The browser renders the plan directly.** `render_next` calls `ready.host.plan.render`
  (`hosts/host-web/src/lib.rs:3212`, `:3234-3239`). There is no publisher, no candidate and no
  retirement.
- **The exchange carries only the plan.**
  - `plan_exchange` returns publisher, owner and retirer
    (`crates/engine/src/realtime/plan_exchange.rs:194`). The owner holds
    `active: (PlanEpoch, PreparedRenderPlan)` (`:80-92`).
  - `RealtimePlanOwner::enter_block` (`:375`) adopts a candidate, continues the clock, runs
    `carry_from`, and commits the old plan to the retirement queue.
  - `PlanRetirer::try_reclaim` (`:512`) returns `(PlanEpoch, PreparedRenderPlan)` and nothing
    else. `PlanReplacementReservation::cancel` (`:321`) hands an unpublished plan back.
- **The browser's render-side companions live beside the plan**, in `ReadyOwnership`
  (`hosts/host-web/src/lib.rs:1509`): `spectrum_capture` (`:1578`), `meters` (`:1582`),
  `effect_observations` (`:1586`) and their bookkeeping. Nothing carries them with a plan.
- **The control plane is capi's today.** *Extract the C ABI control plane into a portable crate
  both hosts call* (#1309) moves it to `crates/control-plane` (lib `control_plane`):
  - `SessionState` (today `crates/control-plane/src/control.rs:321`) owns the controller,
    provider epochs, publisher, retirer and render diagnostics;
  - its source routing is `newest_providers`, `submit`, `seek` and `seek_at` (today
    `control.rs:1628`, `:1641`, `:1655`, `:1671`);
  - `PlanState` (today `crates/control-plane/src/plan.rs:57`) wraps `RealtimePlanOwner` and
    `SharedPlanState` for render.
- **Sources.** After #1387 the producer half of every source ring (`SourceControlSet`, the
  `sources` field of host-core's prepared runtime, `crates/host-core/src/prepare.rs:529`) lives in
  the Worker, which calls it directly. The worklet holds only ring consumers.
- **The render-activity code.** #1309 D7 keeps `RENDER_DIAGNOSTIC_CODE` as
  `capi.render.activity`, because `host-core` sizes the slot by that literal
  (`crates/host-core/src/control_provider.rs:117`).
- **No browser export exists for the service step or the watermark.** #1348 D1 defines
  `SessionState::service()`. #1314 D4 defines the record and `PlanPublisher::watermark_reader()`.
  The browser's frozen export lists are `expected_exports`
  (`scripts/check-web-audioworklet.sh:203`), `EXPORTS`
  (`tools/parameter-metadata/src/abi_layout.rs:135`) and `EXPORTS`
  (`scripts/check-abi-layout-v1.py:117`, run by `check-web-audioworklet.sh:562`).

## Decisions frozen for this slice

- **D1. Boot.**
  1. The Worker's boot builds `control_plane::SessionState` from the boot document through the
     browser's `RuntimePreparer`, `WebRuntimePreparer` (#1400 D5). The committed model is at
     revision 0.
  2. `SessionState` takes the Worker's `SourceControlSet` (#1387) into its provider epochs. It is
     the one owner of the producers from then on.
  3. Boot moves the render half (`control_plane::PlanState`, whose plan carries the companions of
     D3) into #1332's transfer token.
  4. Browser-only boot steps run as today, in the same order: the document budgets, the VCA
     bound, the bridge buffers.
- **D2. Source routing.** The Worker's source entry points (`miso_engine_web_v1_source_submit`,
  `_source_seek`, and the native `seek_at` and seek report) call `SessionState`'s source routing,
  never the set directly. A source that the newest committed session removed therefore follows
  *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325) D4 on the browser as on
  the C ABI. Feeds, messages and replies are unchanged.
- **D3. Companions travel inside the plan.**
  - `PreparedRenderPlan`'s `host_attachment: Option<Box<dyn HostAttachment>>`, with
    `set_host_attachment` and `host_attachment_mut`, comes from #1400 D4. The browser preparer
    sets it at preparation, before publication.
  - `RealtimePlanOwner` gains `active_attachment_mut::<T: 'static>() -> Option<&mut T>`, a
    downcast of the active plan's attachment. It is render-side and allocation-free.
  - The browser's attachment is #1400 D5's `RenderCompanions` struct: the meter consumers, the
    effect observation handles and the spectrum capture. This slice adds to it the per-plan meter
    and observation bookkeeping that `ReadyOwnership` holds today. The Worker prepares it with the
    plan.
  - **Pairing hook.** `HostAttachment` gains `fn adopt_predecessor(&mut self, predecessor: &mut
    dyn HostAttachment, carries: &[ObserverCarry]) {}` (empty default). `PreparedRenderPlan::
    carry_from` (`crates/engine/src/realtime/plan.rs:917`) calls it after the executor hand-over
    when both plans have an attachment, with the successor's `observer_carries()` (*Carry meter
    and effect observation state across a plan swap*, #1327 D3). It therefore runs on the render
    thread, in the swap block, after #1327's and #1395's sections moved the carried observers,
    before the successor renders, and before the predecessor is committed to retirement.
  - `RenderCompanions::adopt_predecessor` downcasts the predecessor through `as_any_mut`; another
    type, or no carry record, pairs nothing. Otherwise it pairs the three reader kinds below. Every
    step is a swap of two values or a scalar copy: no allocation, no drop.
  - **Meters.** The browser's `meters` vector is host-core's `HostLiveControlHandles::meters` in
    its own order (`hosts/host-web/src/lib.rs:7188`), so the carry record's meter indices address
    it directly. It calls #1327 D3's `pair_carried_meters`, and swaps each pair's `meter_pending`
    entry (`:1632`, one per meter) the same way.
  - **Observations: two index spaces.** The carry record's observation indices are positions in
    host-core's dense `effect_observations` vector (`crates/host-core/src/prepare.rs:452`). The
    browser stores the handles in a gapped box indexed by its dense `effect_slot`
    (`hosts/host-web/src/lib.rs:1586`), filled by `dense_effect_slot` at preparation (`:7031-7058`),
    so the two indices differ and shift when a strip is added. So the companions gain
    `observation_reader_slots: Box<[u32]>`: entry `i` is the browser slot that host-core reader `i`
    was stored at, written in that same preparation loop. The browser does not call
    `pair_carried_observations` (its slices are host-core's dense order). For each `Observation`
    record `{ successor: s, predecessor: p }` it maps `s` and `p` through its own and the
    predecessor's tables and swaps the two slots' `effect_observations`, `observation_armed` and
    `observation_arm_samples` entries. `observation_tracks` and `observation_present` are never
    moved: they describe the successor's own strips, so a carried reader folds into its strip's
    new index.
  - **Spectrum.** #1395 D3's `pair_carried` for its single capture or collection.
  - **Meter delivery state.** The state that belongs to the meter stream, not to one reader, is
    taken from the predecessor's companions:
    - copied: `meter_generation`, `meter_snapshot_generation`, `meter_snapshot_drops_seen`,
      `meter_loss_count` and `meter_windows`;
    - from the meter header, only the delivery words: `windows`, `first_sample`, `end_sample`,
      `sequence`, `reserved[0]` and `reserved[1]`, and `master_gr_present` (it describes the last
      published frame). The shape words (`struct_size`, `abi_version`, `track_count`,
      `submix_count`, `master_track_plus_one`) stay the successor's (`WebMeterHeader`,
      `:1128-1156`). The `f32` frame is not copied: its shape may differ, and the next fold writes
      it whole in the successor's shape;
    - the master window state is output-level and continuous across the swap, so it moves: the
      `master_windows` ring is swapped (both are sized by the first meter's queue capacity,
      `:7107-7110`), and `master_read`, `master_write`, `master_count`, `master_peak`,
      `master_start_sample` and `master_end_sample` (`:1619-1629`) are copied. Without them the
      first fold after the swap finds no master window for its span and fails `master_valid`
      (`:3600-3603`).
  - So after the swap block the active companions hold the readers of the observers that render.
    Carried readers keep their sequence, and the browser's `meter_generation` does not advance
    (#1290 gates it on a structural apply).
  - Adoption: the companions become active at the same block as their plan, because they are part
    of it. A block whose `enter_block` returned `Applied` reads the new attachment before it
    renders.
  - Freeing: `enter_block` moves the displaced plan, attachment included, into the retirement
    queue. The Worker's service step reclaims it with `try_reclaim` and drops it there. A candidate
    that is never adopted returns to the control side (`cancel`, supersession #1310, withdrawal
    #1343) and drops there. Render never drops an attachment.
  - The vtable calls (`as_any_mut`, `Any::type_id`, `adopt_predecessor`) are new `call_indirect`
    sites in render's closure, added to #1333 D4's pinned table with this reason.
- **D4. Render.** `render_next` renders through `PlanState`'s `RealtimePlanOwner`.
- **D5. Two new exports.** Only the control side calls them: the Worker, or in `single` mode the
  worklet's control handler. Neither is in the render-locked set.
  - `miso_engine_web_v1_service(handle: u32) -> u32` runs `SessionState::service()` once
    (#1348 D1). Results: `RESULT_OK`; `RESULT_INVALID_ARGUMENT` for a wrong handle; an epoch
    failure as `RESULT_INTERNAL` with the session diagnostic. It commits and acknowledges nothing.
  - `miso_engine_web_v1_watermark_read(handle: u32) -> u32` copies the watermark and its counters
    into a fixed record, `WebPlanWatermark`, found at `miso_engine_web_v1_watermark_ptr(handle) ->
    u32`. The record is 96 bytes and mirrors `miso_engine_v1_watermark` (#1314 D6) field for
    field, in its order: `struct_size`, `reserved0`, `revision`, `first_sample`, `outcome_flags`,
    `exact_count`, `transition_fallback_count`, `superseded_count`, `reserved[5]`. It has no
    pre-roll counter (D15-8 (round-5 amendment)). It lives in the control half and is allocated
    at boot. A read that gives up after #1314 D4's attempts returns `RESULT_BACKPRESSURE` and
    leaves the record untouched: a read to retry, never a refused edit.
  - The read calls `SessionState::watermark()`, the accessor that *Publish the applied-revision
    watermark in the browser status* (#1349) D3 names: a read of #1314's `PlanWatermarkReader`,
    cloned from `PlanPublisher::watermark_reader()` when the session is built. Root orders this
    slice before #1349, so this slice adds the accessor and #1349 reuses it.
  - All three names join every frozen export list. The record joins the ABI layout. The generated
    SDK copies are regenerated.
- **D6. Service loop.**
  - `worker` mode: the Worker calls `miso_engine_web_v1_service` after every message it handles,
    and on the repeating one-quantum timer that #1387 starts for the source drain. The browser may
    clamp the timer; correctness does not depend on the period, because deadlines are counted in
    render samples (D15-17).
  - Both modes run the same service step, so any step a later slice adds to it runs the same with
    or without cross-origin isolation. The Worker runs no catch-up: a warm successor is adopted by
    render (D15-8 (round-5 amendment)), and its deadline check is #1361's.
  - `single` mode: no control work runs inside `process()`. The worklet's control handler calls
    the export after every control message. The main-realm host also posts an internal
    `miso.service.v1` message to the worklet, on #1332 D7's control-plane-to-worklet port, from a
    one-quantum `setInterval`, and the handler services on it. The worklet may not use timers.
    `check-web-audioworklet.sh` bans `setInterval` in the main-realm host today (`:512-515`); this
    slice changes that rule to exactly one pinned `setInterval(` site in the main-realm host, the
    single-mode service tick, cleared at dispose, with a red self-test mutation (a second site).
    The worklet's ban is unchanged. This runs outside the
    render-locked window, and its allocations and frees count in #1332's
    `singleModeControlAllocations`.
  - The loop starts when boot succeeds and stops before dispose.
- **D7. Render-activity code.** The browser uses the crate's `capi.render.activity` code
  unchanged. Renaming it would change a protocol-visible code on both hosts and the literal that
  host-core sizes by.
- **D8. Acked-batch question: can an ack ever precede a drop? No.** This slice adds no queue that
  carries an acknowledged command. The service step acknowledges nothing (#1348 D6). A retired
  plan is already displaced. An unadopted candidate drops only on the control side, under the
  completion rules of the issue that displaced it.

## Deliverables

1. D3's owner accessor in `crates/engine/src/realtime/plan_exchange.rs`; D3's hook in
   `crates/engine/src/realtime/plan.rs`; `RenderCompanions::adopt_predecessor` in host-web.
2. D5's `SessionState::watermark()` accessor in `crates/control-plane`.
3. D1, D2, D4-D6 in `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs`, and the control
   Worker, worklet and main-realm host scripts.
4. The three export names in every frozen export list, the record in the ABI layout, and the
   regenerated SDK copies.
5. Test-support API, public only under `host-web/test-support`: #1387's `boot_split_for_test`
   now returns a `ControlHalf` that holds the `SessionState`; add `ControlHalf::service()` and
   `ControlHalf::republish_committed_for_test()`, which prepares a successor of the committed model
   through the browser preparer and publishes it, and `ControlHalf::publish_successor_for_test(
   document)`, which prepares a given document through the browser preparer and publishes it
   without committing it. They exist only for gates 2-5; no export reaches them.
6. The integration test binary `hosts/host-web/tests/plan_exchange_threads.rs`.

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/tests/plan_exchange_threads.rs`
  (new), `hosts/host-web/Cargo.toml` (the `bench-support` dev-dependency if no earlier slice
  added it; #1400 adds the `control-plane` dependency), `hosts/host-web/web/`
- `scripts/check-web-audioworklet.sh` (`expected_exports`, the capability rule for the new
  message, and D6's one pinned main-realm `setInterval(` site with its self-test mutation),
  `scripts/check-web-audioworklet-callgraph.py` (the pinned indirect sites)
- `scripts/test-web-audioworklet.mjs` (the fake exports and the single-mode service tick)
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts` (regenerated only)
- These are outside stream H's ownership; root sequences them after #1309, #1314 and #1348:
  - `crates/engine/src/realtime/plan_exchange.rs`: D3's owner accessor only;
  - `crates/engine/src/realtime/plan.rs`: D3's `adopt_predecessor` trait method and its call in
    `carry_from` only;
  - `crates/control-plane/src/`: the `watermark()` accessor only;
  - `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS` and the record's fields),
    `scripts/check-abi-layout-v1.py` and `scripts/fixtures/abi-layout-v1-self-test.json`.

## Non-goals

- No structural edit (#1290). No live edit in the Worker (*Admit browser live edits in the Worker
  through the committed model*, #1382).
- No warm-successor deadline check (*Check the warm-successor deadline in the browser Worker's
  service loop and report its outcome*, #1361).
- No watermark words in the browser status block (#1349).
- No adapter preparation hook and no plan attachment field (#1400).

## Objective gates

1. **Same bits.** The browser legs' native-digest gate and the `direct-oracle.mjs` parity pass
   unchanged in Chromium, Firefox and WebKit, on the isolated leg and the non-isolated leg.
2. **Swap and reclaim, two threads.** Integration test binary
   `hosts/host-web/tests/plan_exchange_threads.rs`. Decision 15 rules that host-web's native
   allocation-count gates live in an integration binary, never in `src/tests.rs`, which already
   registers a `#[global_allocator]`. It links `bench_support::alloc` and calls
   `assert_installed()` first.
   - The control half runs on thread A, the render half on thread B. Boot nine tracks with meters,
     one observed effect and a spectrum capture. Render 6 blocks.
   - On A, call `republish_committed_for_test()`. Render 6 more blocks on B.
   - The block after publication reports `SwapOutcome::Applied`. The clock continues. Every block
     is bit-identical to a run without the republish.
   - Every observer carries (the model is unchanged), so the hook pairs every reader. A meter
     poll, an observation read and a spectrum read after the swap are served by the new plan's
     attachment and continue the old sequences: no sequence gap, `meter_generation` unchanged,
     no counted meter loss, no spectrum `Failed` epoch.
   - On B, the thread-scoped counters read `allocations == 0 && frees == 0` around every render
     call.
   - The next service step on A reclaims exactly one plan. The old attachment is dropped on A
     (a drop counter on `RenderCompanions`, test-support only).
3. **A shifted strip keeps its readers and its master window.** Same binary and threads as
   gate 2. Boot tracks `b` and `c`, meters on both, `c` designated master, and a live-controlled
   compressor insert on `c` with its gain-reduction tap armed. Render until a meter window is open.
   Publish, with `publish_successor_for_test`, the same document plus a muted track `a` whose ID
   sorts first, so `c` moves from strip 1 to strip 2 and its effect slot moves too. Render on.
   - The first meter poll after the swap folds a window: its header has `track_count` 3 and
     `master_track_plus_one` 3 (the successor's shape), its `sequence` is the last pre-swap
     sequence plus one, `reserved[0]` is unchanged, and `reserved[1]` has `METER_VALID_MASTER` set
     and a loss count of 0.
   - From that poll on, every frame's peak and gain-reduction words equal those of a fresh boot of
     the successor document fed the same input from frame 0. `c`'s gain reduction is in strip 2's
     word, strip 0's (`a`) and strip 1's (`b`) are 0, and `masterGrDb` is present.
4. **Single mode.** Gate 2 on one thread gives the same blocks. The retired plan and its
   attachment are reclaimed by the first service call after the swap block.
5. **Watermark export.** In gate 2's script, `miso_engine_web_v1_watermark_read` returns
   `RESULT_OK` and a record equal, field for field, to `SessionState::watermark()`. A wrong handle
   returns `RESULT_INVALID_ARGUMENT` and writes nothing.
6. **Exports are frozen.** `bash scripts/check-web-audioworklet.sh` (which runs
   `check-abi-layout-v1.py` over the shipped layout) and `bash scripts/check-sdk-generated.sh
   target/ci/qualification-artifacts` pass. `python3 -B scripts/check-abi-layout-v1.py --self-test`
   passes and turns red if one list omits an export or the record omits a field.
   `check-web-audioworklet.sh`'s self-test turns red on a second main-realm `setInterval(` site
   and on a `setInterval` in the worklet.
7. **C ABI unchanged.** `cargo test --locked -p capi` passes. `target/release/audit capi` reports
   0 allocations, locks and syscalls, with the same `pcm_digest` as the base (PR evidence).
8. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gates 2-5)
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   - the artifact build and `bash scripts/check-web-audioworklet.sh` in both forms that #1332
     gate 7 spells; `bash scripts/test-web-audioworklet.sh`
   - `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `bash scripts/check-cross-targets.sh`, `bash scripts/check-host-core-policy.sh`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 2: turns red if the browser still renders the plan directly (no adoption), if the clock
  restarts on adoption, if a swapped plan is served by the old companions, if the hook is not
  called or pairs nothing (the new attachment's readers would sit on producers that left with the
  retired plan, so every meter and the spectrum stream go silent or restart), if the render
  thread drops a retired plan or attachment, or if the Worker never reclaims them.
- Gate 3: turns red if the swap copies the whole meter header (the successor would report the old
  `track_count` and master index), if the master window state is not moved (the first fold fails
  `master_valid`, so `METER_VALID_MASTER` is clear and a loss is counted), or if observation
  readers are paired by host-core index or by `observation_tracks` moved with them (`c`'s gain
  reduction lands in the wrong strip's word). Gate 2 cannot catch these: its successor has the
  same shape and the same indices.
- Gate 4: turns red if single mode leaks retired plans, or reclaims them inside the render
  export.
- Gate 5: turns red if the export reads anything but #1314's record (for example the status
  words, which carry no counters), or writes for a wrong handle.
- Gate 6: its export-list self-test is not new. The new mutation turns red if the timer rule is
  relaxed beyond the one single-mode service tick, which would let control timers into the
  main realm unpinned.
- Gates 1 and 7 are not new. They hold the bits of both hosts across the move.

## Dependencies

- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Move browser source submission and seeks into the Worker* (#1387).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400).
- *Carry meter and effect observation state across a plan swap* (#1327).
- *Carry spectrum capture state across a plan swap* (#1395).
