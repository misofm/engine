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
retarget while the lane is muted or a mute ramp is in flight. Nothing feeds operations yet;
#1647 drives them from stored fader automation. The live path is unchanged.

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
  the settled kernel does (`crates/builtins/src/lib.rs:2649-2652`). *Keep every trim, fader and
  matrix ramp inside its endpoints* (#1408, its site 1) changes `gain_mute_ramp_block`'s last-step
  law; it is a per-sample rule inside the kernel, so each lane still advances by its own additions
  and a split still moves no bit for the same events.
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
  so a caller (#1637's `hold_until`) knows when the lane's ramp ends. The live path's
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
  declines a ramping lane). The bank processors keep calling `process`; #1647 switches the fader
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

- Producing operations from stored automation (#1646 and #1647); the matrix, input and route
  stages (#1651-#1653, #1656-#1657, #1658 and #1659-#1660).
- Changing the live path's records, drains or `set_fader_gain` (#1054 D10 as amended, finding
  F12).
- The mute-cell composition with VCA and solo terms (#1651-#1653).

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
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
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
- Batch: R1, with #1646 *Prepare stored fader automation and render it flat* (which uses
  `SetGain`) and #1647 *Render moving stored fader automation, seeks and latency*.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **F12 never landed.** #1054 is closed and its D10 was not amended: `set_fader_gain` still
  retargets a muted lane toward 0 (`crates/builtins/src/lib.rs:3083-3098`). This slice takes the
  live rule as well (README F22): while a lane is muted or a mute ramp toward 0 is in flight, a live
  gain change only remembers the gain; during an unmute ramp it retargets as today. D2's sentence
  "This slice does not change the live path" is replaced by this rule, and gate 2 gains the live
  case. Authorized paths are unchanged (`crates/builtins/src/lib.rs`).
- **Anchors moved.** `FaderRampStage`'s setters are now near `crates/builtins/src/lib.rs:3083-3140`,
  `BuiltinFaderBank` `:4314`, `FaderMuteRampBuiltins` `:4732`; re-read at filing. #1408 has landed.
- **Realtime policy** as in #1637's amendment.

## Decision record: #1638 (batch 13b)

**Shipped** (`crates/builtins/src/lib.rs`). `FaderOperation` (D1), `FaderOperationError`
(`Lane`, `Gain`, `Duplicate`), `FaderOperationSink` (fixed size, on the render stack; one gain and
one mute slot per lane and channel; `push` refuses a padding lane, a gain that is not finite or is
negative, and a second operation of one kind at one offset, and a refusal queues nothing on either
channel), and the pull trait `FaderOperationSource { next_offset, apply }`. `FaderRampStage` gains
one flag per lane and channel, `mute_ramp` (D2): set by a mute change with a nonzero window, cleared
by a zero-window mute change, in the bookkeeping step that ends the countdown, by a live gain
retarget, and by `reset`; it is carried by `export_lane`/`import_lane` (a new private field of
`FaderChannelState`), so a plan swap during a mute ramp keeps the hold. `process_with` cuts the
block at each offset and runs `process_plane` over each piece, so a split moves no bit (D6); the
sink applies gains before mutes, left before right, whatever the push order (D4). Exposure (D5):
`BuiltinFaderBank::{process_with, apply_fader_operation, gain_held, ramp_remaining,
try_process_settled_with_matrix_and_operations}` and `FaderMuteRampBuiltins::{process_with,
apply_fader_operation, gain_held, ramp_remaining}`. No production path calls `process_with`.

**Live rule (amendment, README F22).** `set_fader_gain` on a muted lane (settled or ramping toward
`0`) now only remembers the gain; on an unmuted lane, including one in an unmute ramp, it retargets
and clears `mute_ramp`. Timed `RetargetGain`/`SetGain` remember only while the gain is held (muted
or `mute_ramp`), so they also remember during an unmute ramp, as D2 says.

**Departures.**
- Offsets: an offset at or past the block is not pulled and stays with the source; offsets must
  rise strictly, and a repeated or lower offset stops pulling for the block (debug assertion),
  which bounds the loop at `frames` pieces.
- D5's decline is a new method, `try_process_settled_with_matrix_and_operations`, which declines
  when the source has an operation inside the block and otherwise forwards; the existing method and
  its callers are unchanged.
- Gate 4 "every existing test passes unchanged" cannot hold with the amended live rule: in
  `crates/builtins-compiler/src/lib.rs`,
  `the_drain_applies_the_fader_before_the_mute_whatever_the_write_order` relied on a fader move
  after a *mute* being observable; under F22 both orders are now alike. The test now starts the lane
  muted and writes an *unmute*, where the order is still observable (fader-then-unmute ramps to the
  new gain; unmute-then-fader cuts the unmute ramp short). Test-only edit; no wiring.
- `tools/realtime-policy/src/policy.rs`: `marked_regions_floor` 144 -> 147 (the sink, the bank's and
  the scalar track's `process_with`); files stay 32.

**Gates.**
1. `an_operation_inside_a_block_renders_the_split_blocks_bits` (Simd4, Simd8, scalar; q 128 at
   o in {1, 63, 64, 127} and q 100 at o in {1, 50, 99}; six operations; settled, ramping, ramp
   ending before o, muted, mute ramp, unmute ramp; one lane and two lanes at different offsets;
   the next block compared too), `an_operation_at_offset_zero_is_block_entry_and_one_past_the_block_waits`.
2. `gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it` (timed `RetargetGain`,
   `RememberGain` and the live setter, every 64 samples in a 480-sample mute ramp).
3. `at_one_offset_the_gain_applies_before_the_mute`.
4. Existing `fader_ramp.rs` tests unchanged and green; builtins-compiler as above.
5. `process_with_operations_never_allocates` (1,000 blocks per subject, thread counters: 0 and 0).
- D2 flag: `the_held_flag_follows_the_mute_ramp`; sink refusals:
  `the_fader_operation_sink_refuses_without_queuing` (in-crate).
- Red on revert (copied aside, not stashed): live muted retarget restored -> gate 2 red; timed held
  retarget -> gate 2 red; mutes applied before gains -> gate 3 red; operations one sample late ->
  gate 1 red; flag not cleared at the countdown end -> gate 2 and the flag test red.

**Successor obligation (#1647).** D2 holds a lane's gain during an *unmute* ramp too: a
`SetMute` with a nonzero window sets `mute_ramp`, and a timed `RetargetGain` or `SetGain` on a held
lane only remembers the gain (`FaderRampStage::timed_gain`, `crates/builtins/src/lib.rs`), while a
live gain change during an unmute ramp retargets. #1637 holds only `Grid` events: a `Jump` (on a
cell that is not `no_restart`) and a seek `Set` always emit and commit the cell's target with
`held = false`. So when #1647 converts a `Jump` or a `Set` that lands inside an unmute ramp to
`RetargetGain` or `SetGain`, the stage only remembers it; the unmute ramp settles at the
pre-unmute gain while `fader_gain` and the cell hold the new value, and on a flat curve the cell
emits nothing more (its target equals the curve and `held` is false), so the lane stays wrong with
no end. #1647 D1.2 (`hold_until`) covers only `Grid` events. #1647's amendment from this review
records the obligation and its gate.

**Attempt 2 (review fold).**
- Docs: the successor obligation above; #1647 gains an amendment (the reissue at the end of a held
  ramp, and its gate). `CellState::hold_until` only sets the in-flight completion, and `held` is set
  only by a held `Grid` event whose value differs from the cell's target, so #1637 cannot re-emit a
  committed `Jump` or `Set`; the amendment therefore makes #1647 reissue the cell's target itself.
- `FaderOperationSource` rustdoc states the contract: strictly rising offsets, and a source keeps
  every operation it did not apply. `BuiltinProcessReport` gains `fader_operations_stopped` (`1`
  when a `process_with` block stopped pulling on a repeated or lower offset, else `0`), set by both
  `process_with` methods from a `bool` the stage now returns; the debug assertion stays. Not
  tested: a debug test build fails the assertion first. No rendered bit moves.
- Test: `the_fused_settled_path_declines_only_an_operation_inside_the_block`
  (`crates/builtins/tests/fader_ramp.rs`; Simd4, and Simd8 with AVX2): an operation inside the
  block (declines, the block and every fader and matrix word unchanged, the source keeps the
  operation, then `process_with` equals a twin), at `offset == frames` (forwards: equals a twin on
  `try_process_settled_with_matrix`, the source keeps it), and no operation (forwards). Red, file
  copied aside and restored with `cp`: `<=` for `<` (the block-end case), no decline (the inside
  case), a decline that first runs the fused path (block untouched fails), a decline that applies
  the operation (stage words fail).

**Attempt 3 (record fold, coordinator).** The attempt-2 review measured that #1647's D1.6 as first
written was not quantum-invariant: refreshing `hold_until` at every block entry inside a held ramp
overwrote a cell `Jump`'s own ramp end. #1647's amendment now says when D1.2 reports the hold (once,
at the block entry that starts or restarts the ramp, and after an adoption that carries it), holds
the grid for the reissue's own ramp, composes the reissued gain as D1.4, and adds the moving-curve
cases to gate 8 (cases (c) and (e), whose samples `[E, E + J - 1]` equal one direct `RetargetGain`
at `E` over `J`). No code change. The counter `fader_operations_stopped` is to be summed by #1647
when it wires `process_with` into production.

**Attempt 3 review and rescope (coordinator).** The attempt-3 review confirmed the hold rule is
quantum-invariant and found two more design gaps in #1647's D1.6 (a grid event that is not held
inside the hold's last 64 samples; stopping the cell exactly at the ramp end). Three reviews found
this slice's code correct; every failure was in the successor's mechanism. The mechanism is
therefore #1647's design: #1647's amendment now states the requirement, the measured constraints and
gate 8, and #1648's amendment the carry. This slice's obligation is that record.

**Gates 6 and 7 on the integrated tree (batch 13b, cddff37d9).** Gate 6: `audit capi` gives
`pcm_digest` `bf7dcedd1d92621f`, as on main; the browser fixtures' digests and exact rows are
unchanged (`check-browser-expected-resources`), and chromium 151, firefox 153 and webkit 26.5
qualification pass with 0 render allocations. No fixture moves a fader on a muted lane, so the F22
live rule moves no pinned digest; its behaviour is held by gate 2. Gate 7: the batch gate set
(workspace tests, clippy, `check-cross-targets.sh`, the policies) passes. The AArch64 legs
(`aarch64-debug`, Simd4 on NEON) run only in CI: this host has no AArch64 toolchain.
