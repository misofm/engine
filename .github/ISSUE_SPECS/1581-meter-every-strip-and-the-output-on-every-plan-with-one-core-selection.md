# Meter every strip and the output on every plan with one core selection

Slice S22 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

Every plan, on every host and from every preparation entry, carries the same meters: one left and
right sample peak per strip at the post-matrix boundary, and one windowed left and right peak of
the output. One core function chooses the strip meters and one core type measures the output. A
browser session that boots with a spectrum capture now reports its submix peaks; today they stay
`0`. A C ABI plan carries the same meters, which #1587 delivers.

## Context

- **The browser chooses its own meters.** `compile_ready` builds one `PostMatrix`
  `MeterMetricSet::SAMPLE_PEAK` request per strip, tracks then submixes
  (`hosts/host-web/src/lib.rs:6977-6992`), but passes it only on the no-spectrum branch
  (`prepare_host_runtime_with_selected_meters_between_render_calls`, `:7028`). The two spectrum
  branches (`:6995`, `:7012`) call entries that pass `selected_meters: None`
  (`crates/host-core/src/prepare.rs:1022`, the `None` at `:1034`; `:1056`).
- **The default set is different.** With `None`, the policy function
  (`prepare.rs:1387`) meters only the tracks, at `live_controls.meter_tap`, with
  `MeterMetricSet::ALL` (`:1736-1745`). The browser frame has a slot pair for every strip
  (`lib.rs:7573`, `boxed_zero_meter_frame` `:7667`), so on a spectrum boot the submix pairs stay
  `0`, and every track meter computes RMS, counts and held peak that nobody reads.
- **The request.** `HostLiveControlRequest` (`prepare.rs:325`): `meter_period_frames` (`:339`,
  `None` means no meters), `meter_queue_depth`, `meter_tap` (`:343`). Meter handles are the request
  index plus one (`:1746-1775`). `HostLiveControlHandles::meters` (`:464`).
- **The C ABI has no strip meter.** `compile.rs:616-619` (`crates/control-plane/src/compile.rs`)
  passes the default request, so `meter_period_frames` is `None`, and `:646-651` drops every
  handle it does not name (`..`).
- **The window length.** The browser boot word `live_control_meter_blocks` (`lib.rs:1258`): `0`
  binds no meter at all (`:7607`, `:7618-7623`); the default options use `DEFAULT_METER_BLOCKS`
  `= 12` (`:1090`, `:1298`). The observation default window follows it (`prepare.rs:1685`).
- **Two output peaks.**
  - The browser folds a master peak over its output planes in `render_next` while the lease is
    held (`lib.rs:3410-3452`), into a ring of `MasterMeasurement` windows (`:1806`, push `:1848`,
    pop `:1862`, fields `:1764-1774`), with `meter_sample_magnitude` (`:6304`).
  - #1570 D3 moves the C ABI's per-block scan (`crates/capi/src/ffi.rs:939-955`) unchanged into
    `control_plane::PlanState::observe_rendered_block`, which both hosts call after each
    successful render; it publishes one combined peak per block through `render_peak_bits`
    (`crates/control-plane/src/plan.rs:14`, gate `:29`, `publish_render_observation` `:290`).
- **The peak kernel exists.** `lane::kernels::builtins::meter_sample_peak_block`
  (`crates/lane/src/kernels/builtins.rs:2174`) is the builtin meter's sanitized peak: a non-finite
  or subnormal magnitude counts as `+0.0`, which is exactly `meter_sample_magnitude`.
- **Strip order.** `SessionModel::strips` (`crates/session/src/model.rs:490`) yields tracks, then
  submixes; host-core's strip list is that order (`prepare.rs:1710-1722`).

## Decisions frozen for this slice

- **D1. One strip meter selection.** New module `crates/host-core/src/meters.rs` with
  `pub fn meter_selection(model: &SessionModel) -> Vec<HostMeterRequest>`: one request per strip,
  in `SessionModel::strips` order, `tap: MeterTap::PostMatrix`, `metrics:
  MeterMetricSet::SAMPLE_PEAK`. The policy function uses it whenever `selected_meters` is `None`;
  the per-track `ALL` default (`prepare.rs:1736-1745`) is deleted. `HostLiveControlRequest::meter_tap`
  is deleted. `prepare_host_runtime_with_selected_meters_between_render_calls` keeps its `meters`
  argument for the host-core tests that prove the other taps and metric sets; no host calls it
  with its own list. host-web's no-spectrum branch passes `&meter_selection(model)`.
- **D2. Meters on every plan.** `meter_period_frames: Option<NonZeroU32>` is replaced by
  `meter_window_blocks: u32`. `0` means the core default, `default_meter_window_blocks(rate_hz,
  quantum_frames) -> NonZeroU32` in `meters.rs`: `max(1, (rate_hz * 32 + 500 * quantum) /
  (1000 * quantum))` in `u64` (about 32 ms; 12 blocks at 48 kHz and 128 frames, 11 at 44.1 kHz,
  24 at 96 kHz, 3 at 512 frames). Every preparation entry prepares `meter_selection` with that
  window. There is no "no meters" form. The browser passes its boot word unchanged (`0` now means
  the core default, not "no meters"); the C ABI passes `0`.
- **D3. One output meter.** In `meters.rs`:
  - `output_peak_meter(window_frames: NonZeroU32) -> (OutputPeakObserver, OutputPeakReader,
    OutputPeakGate)`. `OutputPeakGate` is an `Arc<AtomicU64>`, the first absolute sample the meter
    may observe; `u64::MAX` (the start value) means inactive. `activate(from_sample)` and
    `deactivate()` are control-side stores (`Release`); render loads it once per block
    (`Acquire`).
  - `OutputPeakObserver::observe(&mut self, left: &[f32], right: &[f32], first_sample: u64,
    next_sample: u64)`: when `first_sample >= gate`, it folds each plane's sanitized peak with
    `meter_sample_peak_block` (the plane read as consecutive lane-wide chunks, then the lanes'
    maximum; the tail with the same rule), opens a window at the first observed block, and closes
    it when `next_sample - start >= window_frames`. A closed `OutputPeakWindow { start_sample,
    end_sample, peak: [f32; 2] }` goes into a bounded SPSC queue of depth `METER_QUEUE_DEPTH`
    (`8`, moved here from `hosts/host-web/src/lib.rs:1189`). A full queue drops the new window
    and adds one to `OutputPeakReader::dropped_windows()`. When the gate is inactive the call
    loads one word and returns, and any open window is discarded.
  - It allocates nothing and is bounded by the block length.
- **D4. Where it runs.** `control_plane::PlanState` owns the `OutputPeakObserver` for the
  session's life (it is created with the session's first plan and is not part of any plan, so a
  swap does not touch it). `observe_rendered_block` (#1570 D3) takes `&mut self`, gains
  `first_sample` and `next_sample` arguments, and calls the observer first. The legacy per-block peak for `render_peak_bits`
  stays in that function, gated as today, until #1583 deletes it. `SessionState` hands out the
  reader and the gate once (`take_output_meter() -> Option<(OutputPeakReader, OutputPeakGate)>`).
  The browser's render half takes them; the C ABI's control half keeps them for #1583.
- **D5. The browser's master ring goes.** `MasterMeasurement`, `master_peak`,
  `master_start_sample`, `master_end_sample`, `master_windows`, `push_master_window`,
  `pop_master_window`, `meter_sample_magnitude` and the scan in `render_next` are deleted.
  `set_meter_lease` activates the gate at `meter_activation_sample` and deactivates it on release;
  `poll_meters` reads `OutputPeakWindow`s from the reader where it read `MasterMeasurement`s, with
  the same span checks and loss counts. A dropped window adds to `meter_loss_count`. Browser bytes
  do not change.
- **D6. The C ABI keeps its meter handles.** `compile.rs` keeps `HostLiveControlHandles::meters`
  in `PreparedRuntime` (a new field `meters: Box<[MeterConsumer]>`) instead of dropping them.
  Nothing reads them until #1587; a full strip queue drops and counts on the render side as it
  does today. Their bytes join the plan's resource rows.
- **D7. Evidence, not a gate.** The PR records once, on the 64-track console session
  (`tools/console-workload`), the per-block render cost and the retained bytes that the strip
  meters add to a C ABI plan. *Skip per-track web meter work while no one holds the meter lease*
  (#882, amended) decides from it whether the strip meters get a render-side gate.

## Deliverables

1. D1-D3 in `crates/host-core/src/meters.rs` (new), `prepare.rs` and `lib.rs` (re-exports).
2. D4 in `crates/control-plane/src/{plan.rs,control.rs}`; D6 in `crates/control-plane/src/compile.rs`.
3. D5 in `hosts/host-web/src/lib.rs`.
4. Every caller of the changed request fields updated (host-core tests, host-web, control-plane).
5. The tests below and the D7 evidence line in the PR.

## Authorized paths

- `crates/host-core/src/{meters.rs,prepare.rs,lib.rs}`, `crates/host-core/tests/{strip_meters.rs,prepare.rs,spectrum.rs,meters.rs}`
  (`meters.rs` new).
- `crates/control-plane/src/{plan.rs,control.rs,compile.rs}`, `crates/capi/src/ffi.rs` (the new
  `observe_rendered_block` arguments only), `crates/capi/src/runtime/tests.rs`.
- `hosts/host-web/src/{lib.rs,tests.rs}` (the meter list, the master ring, the boot-word comment).
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`
  (the `liveControlMeterBlocks === 0n` sentence only, byte-identical twins).

## Non-goals

- No meter delivery change: the fold is #1582, the C ABI batches are #1583 and #1587.
- No render-side gate for the strip meters (#882).
- No change to meter kernels, to `MeterSnapshot`, or to the tap or metric set of the
  selected-meter entry's tests.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --all-features`, `cargo run --locked --release -q -p realtime-policy`,
`bash scripts/check-sdk-generated.sh <artifacts>`, and the worklet gates of #1534 gate 3
(`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `test-web-audioworklet.sh`).

1. **Same meters on every entry.** In `crates/host-core/tests/meters.rs`, on a session with two
   tracks and two submixes, every public preparation entry (the live-lanes entries, both spectrum
   entries, the between-render-calls entry and their successor forms) returns one meter consumer
   per strip, in strip order, each `PostMatrix` with only `SAMPLE_PEAK` present in its snapshots.
2. **Submix peaks on a spectrum boot.** A host-web test boots the bus fixture of
   `the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (`tests.rs:6048`) with a
   spectrum capture, holds the lease, renders a tone on the submix, and reads a nonzero submix
   peak pair that equals the no-spectrum boot's frame bit for bit. Record it red on the base.
3. **Output meter.** Unit tests of `OutputPeakObserver`: windows tile `[start, end)` with no gap
   at quanta 64, 128 and 512; each window's peak equals a scalar fold of
   `meter_sample_magnitude`'s rule over the same samples, NaN, infinities and subnormals included;
   an inactive gate folds nothing; a full queue drops the newest window and counts it.
4. **Browser bytes unchanged.** The existing meter tests stay green unchanged:
   `meter_frames_equal_an_offline_fold_and_cost_the_render_nothing` (`tests.rs:5192`),
   `incomplete_master_periods_skip_all_track_pops_and_effect_scans` (`:5282`),
   `meter_invalid_master_rejects_transactionally_then_recovers_with_loss` (`:5415`),
   `meter_lease_reacquisition_waits_for_a_clean_boundary` (`:5556`) and
   `bus_meters_render_and_poll_without_allocating` (`:6115`).
5. **Window default.** `default_meter_window_blocks` returns 12, 11, 22, 24, 3 and 1 for
   (48000, 128), (44100, 128), (88200, 128), (96000, 128), (48000, 512) and (48000, 4096).
6. **C ABI plans carry meters.** A capi runtime test prepares a two-strip session and reads one
   meter consumer per strip from `PreparedRuntime`; its resource report counts their bytes.
7. **Realtime.** Rendering with the output gate active makes `allocations == 0 && frees == 0`
   (`bench_support::alloc`, after warm-up) on both hosts' render calls.

## Test value

- Gate 1 turns red if any entry keeps its own meter set, so two hosts or two boot shapes meter
  different strips.
- Gate 2 turns red on today's defect: a spectrum boot leaves the submix peaks at `0`.
- Gate 3 turns red if the output windows leave a gap, count a sanitized sample, or measure while
  inactive.
- Gate 4 turns red if moving the master scan into the core changes a published browser byte.
- Gate 5 turns red if the default window drifts from the browser's present 12 blocks at 48 kHz.
- Gate 6 turns red if the C ABI still drops its meter handles.
- Gate 7 turns red if the output meter allocates on render.

## Dependencies

- #1570 (`PlanState::observe_rendered_block` on both hosts).
- *Prepare every browser preparation branch concurrently, as a successor too, in host-core*
  (#1401): its four new entries take D1 and D2; this slice rebases after it.
- *Swap and retire browser plans through the Worker's service loop* (#1381) and
  *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332):
  `hosts/host-web/src/lib.rs` is stream H's file (`STREAMS.md` row 101); this slice rebases after
  them.

## Supersedes and shrinks

- *Report each configured meter handle's own meter in the C ABI meter batch* (#1352): its D2 and
  D3 (per-plane, windowed output peaks) are this slice's output meter; the issue is superseded by
  #1583 and #1587, which deliver the rest, and closes when #1587 merges.
