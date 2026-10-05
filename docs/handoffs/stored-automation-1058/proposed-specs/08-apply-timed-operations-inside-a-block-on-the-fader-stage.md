# Apply timed operations inside a block on the strip fader stage

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.5), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

The strip fader and mute stage can apply a lane's fader and mute changes at any sample inside a
block, not only at block entry. A block with an operation at offset `o` renders, bit for bit, what
the same block split at `o` renders with the operation applied at the second part's entry, at
every width. A new "remember only" operation updates a lane's remembered fader gain without a
retarget while the lane is muted or a mute ramp is in flight. Nothing feeds operations yet; draft
09b drives them from stored fader automation. The live path is unchanged.

## Context

- **The stage.** `FaderRampStage<L>` (`crates/builtins/src/lib.rs:2653-2663`) holds per channel a
  `GainMuteRamp` (current gain, target, step, an `f32` countdown, the mute mask), each lane's
  remembered `fader_gain`, its `muted` flag and a `u32` countdown `remaining`.
  - `retarget` (`:2730-2750`) sets one lane's target and computes one step per event; a window of
    0 assigns at once.
  - `set_fader_gain` (`:2752-2771`) remembers the gain and retargets; on a muted lane it retargets
    toward `0.0` again (`:2768`), which restarts a mute ramp in flight (README finding F12).
  - `set_mute` (`:2773-2794`) retargets to `0.0` or back to `fader_gain` and updates the mask.
  - `process_plane` (`:2813-2865`) runs the settled kernel when no lane ramps, else the ramp kernel
    over `min(max remaining, frames)` frames, then bookkeeping, then the settled kernel over the
    rest. The `f32` countdown is recomputed from the `u32` one at each ramping call (`:2829-2834`).
- **The kernels.** `gain_mute_block` (`crates/lane/src/kernels/builtins.rs:127-140`) and
  `gain_mute_ramp_block` (`:159-202`). The ramp kernel advances each lane by its own additions and
  assigns the target exactly on the last frame, so partition and cohort invariance hold by
  construction (`:180-183`). A settled lane in the ramp kernel multiplies by its exact target, as
  the settled kernel does (`crates/builtins/src/lib.rs:2649-2652`).
- **Three users of the stage.** `BuiltinFaderBank` at Simd4 and Simd8
  (`crates/builtins/src/lib.rs:3777-3783`, `:3808-3968`), and `FaderMuteRampBuiltins`, the same
  stage at `f32` (`:4163-4233`), which is the scalar track and the scalar oracle.
- **Bank processors.** The fader bank processor drains its lanes' live records at block entry and
  processes the bank (`crates/builtins-compiler/src/lib.rs:744-752`; the drain `:1063-1105`). The
  fused fader-and-matrix processor drains both and takes the fused settled path only when no fader
  or matrix lane ramps (`:1198-1219`; `try_process_settled_with_matrix`,
  `crates/builtins/src/lib.rs:3926-3956`).
- **Order at one entry.** Live records apply fader before mute, left before right
  (`crates/host-core/src/live_delta.rs:66-68`).

## Decisions frozen for this slice

- **D1. Operations.** `FaderOperation` (in `crates/builtins`), per lane and channel selector:
  - `RetargetGain { gain, samples }`: as `set_fader_gain` when the lane's gain is free, as
    `RememberGain` when it is held (D2);
  - `SetGain { gain }`: the same with a window of 0 (an exact set);
  - `SetMute { muted, samples }`: as `set_mute`;
  - `RememberGain { gain }`: update `fader_gain` only; no retarget.
  Gains are linear (the caller converts dB with `checked_fader_gain`, `crates/builtins/src/lib.rs:4292-4297`).
- **D2. Held gain.** A lane's gain is held while it is muted, or while a mute ramp is in flight on
  it. The stage tracks the second with one flag per lane and channel: `SetMute` with a nonzero
  window sets it; the lane's countdown reaching 0, or a later gain retarget on that lane through
  the live path, clears it. `gain_held(lane, channel)` and `remaining(lane, channel)` are readable,
  so a caller (draft 07's `hold_until`) knows when the lane's ramp ends. The live path's
  `set_fader_gain` keeps its own rule, which *Session controlSmoothing: configurable ramp lengths for
  live mute, fader and pan changes* (#1054) D10 sets as amended by the #1058 note (finding F12):
  while the lane is muted (settled, or ramping toward 0) a live gain change only remembers the gain;
  during an unmute ramp it retargets as today. This slice does not change the live path.
- **D3. In-block application.** `FaderRampStage::process_with` takes the block and a pull source of
  operations: `next_offset() -> Option<u32>` and `apply(offset, &mut FaderOperationSink)`, which
  applies every operation at that offset. The stage runs `process_plane`'s existing logic over each
  piece between two offsets, then applies the operations at the next offset, and so on. The
  countdown words are recomputed at each piece, as at each block today. No buffer is sized
  `q × lanes`; nothing is allocated. A source with no operation gives today's single call.
- **D4. Fixed order at one offset.** Gain operations before mute operations, left before right, as
  the live drain (`crates/host-core/src/live_delta.rs:66-68`). So an unmute and a fader move at one
  sample unmute toward the new gain, and a mute and a fader move at one sample end muted.
- **D5. Exposure.** `BuiltinFaderBank::process_with` and `FaderMuteRampBuiltins::process_with`
  forward to D3. `try_process_settled_with_matrix` declines a block with any operation (it already
  declines a ramping lane). The bank processors keep calling `process`; draft 09b switches the fader
  bank processor to `process_with`.
- **D6. Class A.** No new arithmetic. The pieces run the existing kernels in the existing order, so
  a split moves no bit.
- **D7. The acked-batch question.** No queue is added or changed.

## Deliverables

1. D1-D5 in `crates/builtins/src/lib.rs`.
2. Tests in `crates/builtins/tests/fader_ramp.rs` and the in-crate tests module.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins/src/tests.rs` (the in-crate tests module, `lib.rs:5627`),
  `crates/builtins/tests/fader_ramp.rs`
- `crates/builtins-compiler/src/lib.rs`: only if a bank processor must name the new type to compile;
  no wiring
- `crates/lane/src/kernels/builtins.rs`: only if a range entry to an existing kernel is needed;
  no new arithmetic

## Non-goals

- Producing operations from stored automation (drafts 09a and 09b); the matrix, input and route
  stages (drafts 13a-13c, 14a-14b, 15 and 16a-16b).
- Changing the live path's records, drains or `set_fader_gain` (#1054 D10 as amended, finding
  F12).
- The mute-cell composition with VCA and solo terms (drafts 13a-13c).

## Hazards

- **The settled tail.** `process_plane` runs the settled kernel after the last ramp ends. A piece
  boundary inside that tail must not run the ramp kernel again with a stale countdown; D3's
  recomputation at each piece covers it, and gate 1 checks offsets past a ramp's end.
- **Padding lanes** take no operation and stay inert.
- **The flag of D2** is new state. It must be cleared exactly when the lane's countdown reaches 0,
  in the same bookkeeping step that assigns the target (`crates/builtins/src/lib.rs:2840-2854`).

## Objective gates

1. **Partition identity** (`crates/builtins/tests/fader_ramp.rs`, new). For a bank at Simd4 and
   Simd8 and for `FaderMuteRampBuiltins` (scalar), at quantum 128 with an operation at offset `o` in
   {1, 63, 64, 127}, and at quantum 100 with `o` in {1, 50, 99}: one `process_with` call equals, bit
   for bit, `process` over `[0, o)`, the operation applied through the existing setters, then
   `process` over `[o, q)`. Run for each D1 kind, on a settled lane, a ramping lane, a lane whose
   ramp ends before `o`, and two lanes with operations at different offsets in one block.
2. **Remember only.** A lane muted over 480 samples, with `RetargetGain` operations every 64
   samples inside the ramp, renders the bits of the mute ramp alone; after unmute, the lane ramps to
   the last remembered gain. The same with `RememberGain`.
3. **Order at one offset.** Unmute and a gain move at one offset end at the new gain; mute and a
   gain move end muted.
4. **Existing behaviour.** Every existing test in `crates/builtins/tests/fader_ramp.rs` and
   `crates/builtins-compiler` passes unchanged; `process` with no operations is the existing code
   path.
5. **No allocation.** 1,000 blocks of `process_with` with operations allocate and free nothing
   (`bench_support::alloc` thread counters after warming).
6. **No rendered bit moves.** No production path calls `process_with` yet; every render digest is
   unchanged (`cargo test --locked -p builtins-compiler --features test-support`, the browser legs of
   the `browser` job in `.github/workflows/qualification.yml`).
7. **Commands:**
   - `cargo test --locked -p builtins --features builtins/test-support`
   - `cargo test --locked -p builtins-compiler --features test-support`
   - `bash scripts/run-aarch64-tests.sh debug` (the `aarch64-debug` job: Simd4 on NEON)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-unfused-seal.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a piece reuses a stale countdown, applies an operation one sample early or
  late, or if a width diverges from the scalar oracle. The existing partition test
  (`a_cross_block_ramp_is_partition_invariant`, `crates/builtins/tests/fader_ramp.rs:284`) has no
  operation inside a block.
- Gate 2 turns red if an automation event during a mute ramp restarts the ramp toward 0 (finding
  F12's artefact) or loses the remembered gain.
- Gate 3 turns red if the order at one offset differs from the live drain's.

## Dependencies

- None.
- Batch: R1, with drafts 09a *Prepare stored fader automation and render it flat* (which uses
  `SetGain`) and 09b *Render moving stored fader automation, seeks and latency*.
