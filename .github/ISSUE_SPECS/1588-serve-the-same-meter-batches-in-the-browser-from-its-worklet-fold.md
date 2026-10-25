# Serve the same meter batches in the browser from its worklet fold

Slice S24c of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Third part of the planner's S24, after #1583 and #1587. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

A browser or headless app that sends `TELEMETRY_CONFIGURE` with a meter table receives the same
`METER_BATCH` events, with the same bytes, as a C ABI app for the same session, PCM and table. The
browser still folds its meters once, in the worklet after render, and still posts its low-latency
meter frame under the lease. The two deliveries share one fold, one level and one window grid.

## Context

- **One fold per session, in the render half.** #1582's `MeterDelivery` runs in the worklet's
  `process()` after render (`miso_engine_web_v1_meter_poll`, `hosts/host-web/src/ffi.rs:4124`;
  worklet `postMeterFrame`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:1770`, called at
  `:1871`). It is the only consumer of the strip meter queues and of #1581's output meter on the
  browser. The lease is a render-half message (`receiveMeterLease`, `:1690`; export
  `miso_engine_web_v1_meter_lease`, `ffi.rs:4105`).
- **The browser's control half serves the protocol.** #1570 serves `TELEMETRY_CONFIGURE` and drains
  both event lanes in the browser's control half (#1570 D4). #1583 makes the browser's provider
  answer `meter_window_blocks() == None`, so a meter table is refused `meter.unavailable`, and its
  parity script configures no meter.
- **The batch rule** is #1583 D5 and #1587 D3 (`MeterBatchFold`, the record rule, the lost-window
  counter).
- **Shared memory.** The worklet and the Worker share one `WebAssembly.Memory` (*Run the browser
  control plane in a Worker and keep the AudioWorklet render-only*, #1332); in single mode both
  halves are one instance on one thread.

## Decisions frozen for this slice

- **D1. One level word.** `host_core::MeterLevel`, an `Arc<AtomicU32>` with bits `LEASE = 1` and
  `TELEMETRY = 2`. The worklet's lease export sets or clears `LEASE`; the control half sets
  `TELEMETRY` when it accepts a nonempty meter table and clears it on an empty one. Only the render
  half calls `MeterDelivery`. At each poll it loads the word; when the word differs from the one
  it last applied, it calls `set_level(word != 0, next_sample)` and, if the level stays on,
  restarts the delivery epoch (a new generation and a new activation boundary). So any change of
  either client restarts the epoch once, and a client that is alone sees exactly today's
  behaviour. The lease export keeps its result codes.
- **D2. The frame ring.** `host_core::MeterFrameRing`: an SPSC ring of 8 preallocated `MeterFrame`
  slots, built at preparation with the delivery's strip count. After each frame that `poll`
  publishes, and only while `TELEMETRY` is set, the render half copies the frame into a free slot
  (element copies into fixed slices; no allocation). A full ring drops the frame and adds one to
  the delivery's loss count. The lease frame copy (`meter_frame`, `meter_header`) happens only
  while `LEASE` is set, as today.
- **D3. The frame names its plan.** `MeterFrame` gains `plan_epoch: u64`: the plan epoch whose
  strips the frame covers. The C ABI's delivery (#1587) fills it too. The control half resolves the
  table entries (#1587 D2) again whenever a frame's epoch differs from the last one it resolved,
  against that epoch's strip list.
- **D4. The control half stages batches from the ring.** In the browser, `collect_render_activity`
  drains the ring into the same `MeterBatchFold` and staging code that the C ABI uses (one
  function, `stage_meter_frames`, shared by both hosts in `crates/control-plane`). The browser
  provider's `meter_window_blocks()` returns the plan's window; #1583's `meter.unavailable`
  refusal for the browser is deleted.
- **D5. The parity script gets its meters back.** Step 5 of `hosts/host-web/tests/protocol_cross_host.rs`
  configures `[{output, peak}, {strip t0, peak}, {strip t0, gain reduction}]` with period
  `2 × window`, and the script's later steps compare the `METER_BATCH` frames of both legs.

## Deliverables

1. D1-D3 in `crates/host-core/src/meters.rs`.
2. D1 and D2 in `hosts/host-web/src/{lib.rs,ffi.rs}` (the level word, the ring producer in the
   poll path); the ring consumer and D4 in `crates/control-plane/src/control.rs` and the browser
   provider.
3. D5 and the tests below.

## Authorized paths

- `crates/host-core/src/{meters.rs,control_provider.rs}`, `crates/host-core/tests/meters.rs`.
- `crates/control-plane/src/control.rs`.
- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/tests/protocol_cross_host.rs`.
- `tools/realtime-policy/render-thread-exports.json` only if an export's callers change.

## Non-goals

- No new browser export and no change to the lease frame's layout.
- No SDK change: #1571's typed surface decodes `METER_BATCH`.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p host-web --all-features`, `cargo run --locked --release -q -p realtime-policy`,
and #1534 gate 3's worklet commands.

1. **Same batches on both hosts.** `protocol_cross_host` with D5's step: every `METER_BATCH` frame
   is byte-identical on the two legs.
2. **Both clients at once.** In host-web: hold the lease and configure a table. Every lease frame
   and every batch describe the same windows (a batch of `k = 2` covers exactly two consecutive
   lease frames' spans, and its peaks are their maxima). Release the lease: batches continue after
   one counted epoch restart. With telemetry alone, the lease frame buffer stays unchanged.
3. **Lease alone is unchanged.** The browser meter tests listed in #1582's Context pass unchanged.
4. **Ring overflow is counted.** Configure a table and render 9 windows with no control-half
   service: `MeterWindowsLost` rises by exactly the frames that never reached a batch.
5. **Realtime.** The worklet poll with telemetry on, after warm-up, makes `allocations == 0 &&
   frees == 0` (the render-locked counter and `bench_support::alloc`).

## Test value

- Gate 1 turns red if the browser stages a batch that the C ABI would not, or with other bytes.
- Gate 2 turns red if the two clients fold different windows or one client's change silently
  changes the other's data.
- Gate 3 turns red if the shared level changes the lease-only behaviour.
- Gate 4 turns red if a dropped frame is not counted.
- Gate 5 turns red if the ring copy allocates on the audio thread.

## Dependencies

- #1583, #1587, #1582, #1570.
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) and
  *Swap and retire browser plans through the Worker's service loop* (#1381): the two halves and
  the plan epoch. `hosts/host-web/src/lib.rs` is stream H's file; this slice rebases after them.

## Supersedes and shrinks

- None.

## Evidence and decision record

- **#1588a attempt 1 (2026-10-25), core half landed.** `host_core::{MeterLevel, MeterFrameRing,
  MeterFrameProducer, MeterFrameConsumer, METER_FRAME_RING_DEPTH}`, `MeterFrame::plan_epoch`,
  `MeterTables::set_plan_epoch` and `MeterDelivery::restart` (R3: one gate move, shared with
  `set_level` through one `begin_generation`). The ring is two move-only SPSC queues of eight
  preallocated boxed frames (free and published, the `response_capture` pattern): a push copies
  elements into a free frame; strips past the frame's read as absent; no free frame drops and
  counts. `MeterFrameRing::heap_bytes` is the charge #1588b uses.
- **R4 amended (implementer, for review).** A ring drop is counted in the ring
  (`MeterFrameConsumer::dropped_windows`), not in the delivery's `loss_count`: the C ABI's
  `MeterWindowsLost` does not sum frames' `loss_count`, and adding ring drops to it would move the
  lease frame's `loss_count` bytes while both clients are on. The browser's lost-window sum is
  then the ring's drops plus the same fold, unstaged and output-drop terms as the C ABI's.
- **Re-split (root, 2026-10-25).** #1588a ships both halves: root authorized
  `crates/control-plane/src/{compile.rs,control.rs,adapter.rs}`, `hosts/host-web/src/lib.rs`
  (the adapter consts only, so far) and `hosts/host-web/tests/protocol_cross_host.rs`. #1588b
  keeps the rest of its list: the render half's level word, restart and ring push in
  `poll_meters`, the lease bit, taking each plan's ring producer
  (`SessionState::take_meter_frame_producer`) into its shape, turning `METER_DELIVERY` on,
  removing steps 1 and 5's exceptions, D5's meter entry, and gates 1, 2, 3 and 5.
- **R2 amended (implementer).** The browser's strip counts are unbounded (its caps are
  `u64::MAX`), so no ring can be sized to "the largest strip count the delivery can hold". Each
  plan gets its own `MeterFrameRing` at its strip count, built in `compile.rs` with the plan and
  charged in the plan's meter rows (`meter_table_rows`); the consumer stays in the plan's
  `ProviderEpoch`, the producer waits there for the render half. The control half stages the rings
  in epoch order (counts loaded newest first, drained oldest first), and stages a reclaimed plan's
  ring before its epoch drops, so its strips are still held.
- **D3 as built.** One staging, `MeterBatches::stage(frame, epoch, ..)`, for both sources:
  entries resolve lazily when a frame's epoch differs from the last resolved (a configure clears
  it), against the epoch's strip list (`strip_controls`, the meters' order). The C ABI passes
  `frame.plan_epoch`, which `adopt_meters` now fills (`MeterTables::set_plan_epoch`); a ring frame
  passes its ring's epoch. The eager resolves at the configure and the adoption are gone. The
  browser does not fill `plan_epoch` yet; #1588b fills it at its adoption and debug-asserts it
  against the ring's epoch at the push.
- **Mode.** `ControlAdapter::RENDER_METER_FOLD` (with `METER_DELIVERY`): the browser keeps the
  output reader and live gate (`take_output_meter`), the session holds a `MeterLevel` (one
  `Arc<AtomicU32>` row in place of the relay's word) and sets `TELEMETRY` at a configure
  (`SessionState::meter_level` hands the render half a handle). The provider gets the window only
  when the preparer binds one (`meter_window_blocks() != 0`): a browser booted with meter word 0
  still refuses meter tables `meter.unavailable`, as it refuses the lease.
- **R4 visible.** Ring drops reach `CounterId::MeterWindowsLost` (`refresh_provider_counters`:
  every held ring's `dropped_windows`, plus the reclaimed rings', plus the output queue's drops
  through `OutputPeakReader::drop_count`).
- **No acknowledged no-op (root, decision 15 E4).** Until the render push exists the browser
  sets `RENDER_METER_FOLD` only. `METER_DELIVERY` is the one constant #1588b turns on, in the
  commit that lands the push. Until then the browser's capabilities (two-slot lane), its echo (no
  meter window), its `meter.unavailable` refusal and its lease refusal are main's, and
  `protocol_cross_host` steps 1 and 5 keep their exceptions; #1588b removes them.
  `the_browser_refuses_a_meter_table_until_its_render_push_exists` pins the refusal for a browser
  booted with a meter window (red with the constant flipped early). The render-half mode runs
  today through the control plane's own test adapter (`rendered_meters`).
- **Bytes.** No rendered bit, lease-frame byte, browser reply or schema hash moved.
- **#1588b attempt 1 (2026-10-25), browser half landed.** `WebAdapter::METER_DELIVERY` is on in
  the commit that lands the push. Each `PlanShape` carries its plan's ring producer
  (`PlanMeterRing`, taken at boot and at each published rebuild), so the adoption block swaps it
  with the tables; the adoption fills `MeterFrame::plan_epoch` (`set_plan_epoch`), and the push
  debug-asserts it against the ring's epoch. `ReadyOwnership` holds the `MeterLevel` and the word
  it last applied; the lease export sets `LEASE` and applies the word at once, so a lease alone
  moves the level exactly as before (gate 3: #1582's tests unchanged).
- **D1 placement amended (implementer, for review).** The render half applies the word before
  each block (`render_next`, from the block's first sample), not at the poll. Reason: the C ABI's
  requested gate applies before the next block; a poll-time application is one block late, so a
  configure on a grid boundary opens a window later and the batches differ (gate 1 red, mutant
  recorded). The lease export still applies at once.
- **Telemetry alone folds in the render export.** The worklet polls only under its lease
  (`process()`: `if (this.meterLease) this.postMeterFrame()`, pinned by
  `check-web-audioworklet.sh`'s process policy), so with the table alone `render_next` runs the
  same fold (`poll_meter_delivery`) after its block. No JS or export change.
- **D5 amended.** Both hosts refuse a gain-reduction entry `meter.gain_reduction.unavailable`
  until #1587b, so step 5's table is `[{output, peak}, {strip track, peak}]` at period
  `2 x 12`; the browser leg boots with the default meter window; step 8 renders two batches
  (48 blocks) and drains after each block. Steps 1 and 5 have no meter exception left.
- **The shipped artifact's call-graph gate: three edits outside #1588b's paths (root authorized,
  2026-10-25; landed).**
  `check-web-audioworklet.sh` is red on this branch: (1) `MeterFrameProducer::publish_frame`
  (host-core, #1588a) drops the `Box` of its unreachable `try_push` refusal, so the render and
  `meter_poll` closures reach `__rust_dealloc`; forgetting the refusal removes it; (2) the render
  export's trap owners gain `MeterDelivery::poll=3`, `MeterDelivery::begin_generation=1` and
  `MeterFrameProducer::publish_frame=2`, and `meter_poll`'s gains `publish_frame=2`; (3)
  `INDIRECT_SITES` pins `render_next` at 4 (the inlined fold's downcast pair). With the three
  edits applied in a scratch tree the gate and `check-sdk-headless.sh` (640 pass) are green.
  Each admitted site is an engine SPSC `slots[cursor]` bounds check, bounded by
  `wrap_increment(cursor, slots_len)`; the script comments name each one.
