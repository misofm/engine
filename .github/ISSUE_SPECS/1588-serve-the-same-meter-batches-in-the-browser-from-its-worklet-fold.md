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
