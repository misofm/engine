# Fold meter windows in one core delivery that the browser calls

Slice S23 of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

One core type, `MeterDelivery`, turns the meters' closed windows into one meter frame: every
strip's peak pair, the output peak pair, every strip's gain reduction, the window span, a loss
count and a delivery generation. It holds the meter level (on or off) and its activation boundary.
The browser's meter lease and meter poll call it and keep only the copy into their frame buffer and
header, so the browser publishes the same bytes as today. #1583, #1587 and #1588 reuse it for the
C ABI and for the browser's meter batches.

## Context

- **The browser's fold.** `AudioWorkletEngineHost::poll_meters` (`hosts/host-web/src/lib.rs:3585`,
  region `:3577-3928`) does all of it: per-queue entry counts (#1448), pending snapshots, the
  group check (handle, `reset_generation`, `window_sequence`, span, frames), sequence-gap recovery,
  the activation boundary, the producer-reset epoch, the master span check, the loss count, the
  gain-reduction fold over armed taps with `observed_decibels` (`:6273`), the master designation,
  and the header and validity bits (`METER_VALID_*`, `:1177-1185`).
- **The lease.** `set_meter_lease` (`:2969`) refuses without meters, advances the generation,
  calls `reset_meter_delivery` (`:1816`) and sets `meter_activation_sample` from
  `next_meter_boundary` (`:2992`). The flag `meter_lease` is host state (`:2131`,
  `meter_activation_sample` `:2134`).
- **The state.** `ReadyOwnership` fields `meters`, `effect_observations`, `observation_tracks`,
  `observation_present`, `observation_armed`, `observation_arm_samples`, `master_track`,
  `meter_frame`, `meter_header`, `meter_pending`, `meter_remaining`, `meter_generation`,
  `meter_loss_count`, `meter_snapshot_drops_seen`, `meter_snapshot_generation`, `meter_windows`
  (`:1727-1792`).
- **The gain-reduction rule.** #1534 makes the fold count only windows with
  `first_sample >= arm_sample` through one helper that `read_observation_values` (`:6207`) also
  calls. The arm mask and the arm samples are host-web arrays written at admission
  (`ReadyOwnership::push`, `:1936`, the `Observe` arm `:1956-1985`) until #1585 moves them into
  the tap's slot.
- **The master designation.** `HostLiveControlRequest::master_track` (`crates/host-core/src/prepare.rs:366`),
  validated at `:2080`, returned in the handles (`:488`); the browser copies the designated
  strip's gain word into the master slot (`lib.rs:3892-3906`), before the header write at `:3907`.
- **The output meter** is #1581's `OutputPeakReader` and `OutputPeakGate`.
- **The tests that pin the bytes.** `hosts/host-web/src/tests.rs`: `:5192`, `:5240`, `:5282`,
  `:5382`, `:5415`, `:5448`, `:5477`, `:5497`, `:5533`, `:5556`, `:5581`, `:5980`, `:6048`,
  `:6115`, `:6670`, `:10868`.

## Decisions frozen for this slice

- **D1. The type.** `host_core::MeterDelivery` in `crates/host-core/src/meters.rs`:
  - `MeterDelivery::new(meters: Vec<MeterConsumer>, output: OutputPeakReader, gate: OutputPeakGate,
    window_frames: NonZeroU32, designated_strip: Option<u32>) -> Result<Self, MeterDeliveryError>`.
    It allocates every buffer here, on the control thread, sized by the meter count.
  - `set_level(&mut self, on: bool, next_absolute_sample: u64) -> bool`: idempotent; a real
    transition advances the generation, clears the pending state (today's
    `reset_meter_delivery(true)`), and on `on` activates the output gate at the next window
    boundary of `next_absolute_sample` (today's `next_meter_boundary`). It returns whether the
    level changed.
  - `poll(&mut self, taps: &impl GainReductionTaps) -> Option<&MeterFrame>`: today's
    `poll_meters` logic, moved unchanged, including #1448's entry counts and #1534's predicate.
    It allocates nothing, takes no lock, uses no runtime-checked `[]` index, and is bounded by
    `1 + sum of the entry counts`. It returns `None` when nothing new is complete, and then
    leaves the last frame unchanged.
  - `frame(&self) -> &MeterFrame`: the last published frame.
- **D2. The frame.** `MeterFrame { first_sample: u64, end_sample: u64, windows: u32, sequence: u64,
  generation: u64, validity: u32, loss_count: u64, strip_peaks: Box<[[f32; 2]]>, output_peak:
  [f32; 2], strip_gain_reduction_db: Box<[f32]>, strip_gain_reduction_present: Box<[bool]>,
  designated_gain_reduction_db: Option<f32> }`. Strips are in `SessionModel::strips` order.
  `validity` bits are `MeterValidity::{COMPLETE = 1, OUTPUT = 2, LOSS = 4, GAIN_REDUCTION = 8}`,
  the values of `METER_VALID_*`, which become aliases of them.
- **D3. Gain-reduction input.** A trait `GainReductionTaps` with one method that visits each
  observed tap: `fn for_each_tap(&self, visit: &mut impl FnMut(TapView<'_>))`, where `TapView {
  strip: u32, descriptor: &'static ObservationDescriptor, reader: &ObservationReader, armed: bool,
  arm_sample: u64 }`. host-web implements it over its arrays in this slice; #1585 implements it
  over the tap's slot and deletes the arrays. `observed_decibels` moves to
  `host_core::meters::reduction_db(descriptor, magnitude) -> f32`, unchanged.
- **D4. The browser keeps only transport.** `set_meter_lease` keeps its state check, its
  `RESULT_UNSUPPORTED` rule and its result codes, and calls `set_level`. `poll_meters` calls
  `poll` and, on `Some`, copies the frame into `meter_frame` and `meter_header` exactly as today
  (`reserved[0]` the generation, `reserved[1]` the validity bits and the loss count shifted by
  32). The fields of D1 leave `ReadyOwnership` and the host. Every browser byte is unchanged.
- **D5. Realtime.** `poll` is called from the worklet's `process()` after render. It stays inside
  the `REALTIME_POLICY_BEGIN`/`END` markers in its new file, and the worklet call-graph gate still
  covers it.

## Deliverables

1. D1-D3 in `crates/host-core/src/meters.rs` and `lib.rs` (re-exports).
2. D4 in `hosts/host-web/src/lib.rs`.
3. Unit tests of `MeterDelivery` in `crates/host-core/tests/meters.rs`.

## Authorized paths

- `crates/host-core/src/{meters.rs,lib.rs}`, `crates/host-core/tests/meters.rs`, `crates/host-core/Cargo.toml`
  (only if a dev-dependency is needed).
- `hosts/host-web/src/{lib.rs,tests.rs}` (the fold, the lease and their state; tests only where a
  helper reads a moved field).
- `tools/realtime-policy/` (the new file's region, if the tool lists files).

## Non-goals

- No C ABI change (#1583, #1587), no browser meter batch (#1588).
- No change to the frame layout, the header, the lease export or the SDK.
- No change to where the arm state comes from (#1585).

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p host-web --all-features`, `cargo run --locked --release -q -p realtime-policy`,
`bash scripts/check-workspace-policy.sh`, and #1534 gate 3's worklet commands.

1. **Browser bytes unchanged.** Every test listed in Context passes unchanged. The PR records once
   that a scripted run (lease, 64 blocks, a re-subscribe, a lease release and re-take) gives
   byte-identical `meter_frame` and `meter_header` sequences before and after the move (evidence,
   not a committed test).
2. **Core unit tests.** On meter consumers from a real two-strip preparation with a fixed PCM
   script: equal groups publish one frame per window with the right span; a queue gap drops only
   the older head and counts one loss; a producer reset starts a new generation and returns
   `None` once; a frame never mixes two spans; a tap window older than its arm sample is left out.
3. **Level.** `set_level(true)` twice changes the generation once; a window that opened before the
   activation boundary is rejected and counted.
4. **Realtime.** `poll` over a 64-strip delivery, after warm-up, makes `allocations == 0 &&
   frees == 0` (`bench_support::alloc`).

## Test value

- Gate 1 turns red if the move changes any published browser byte.
- Gate 2 turns red if the core fold mixes spans, loses the gap recovery or folds a stale tap.
- Gate 3 turns red if a repeated lease starts a new epoch or a pre-activation window leaks out.
- Gate 4 turns red if the fold allocates on the audio thread.

## Dependencies

- #1581 (the output meter and the strip selection).
- *Fold only windows published since a tap's arm into the meter frame's gain reduction* (#1534):
  it lands first (stream B, before H #1332); this slice moves its predicate unchanged.
- `hosts/host-web/src/lib.rs` is stream H's file (`STREAMS.md` row 101); this slice rebases after
  #1332 and #1381.

## Supersedes and shrinks

- None.
