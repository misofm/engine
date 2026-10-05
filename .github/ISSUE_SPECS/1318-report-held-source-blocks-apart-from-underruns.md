# Report held source blocks apart from underruns

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A block that a source renders as silence because the host asked it to wait (an anchored seek is
held until its anchor block) is reported as **held**, never as an **underrun**. Underrun keeps its
one meaning: the host did not deliver PCM it owed. A planned wait no longer looks like starvation
in the per-block report, the cumulative counters or the graph-wide observation flag that browser
spectrum windows carry.

## Context

- An anchored seek is popped and held in `PcmSourceConsumer::held_seek`
  (`crates/source/src/lib.rs:1018`, `HeldSeek` at `:1022-1028`) until the block whose first sample
  reaches its anchor (`observe_seek_at_block_boundary`, `:1323-1363`). While it is held, queued
  PCM of the playing generation still plays, and with nothing playable the block underruns
  (`begin_block_at` doc, `:1108-1114`).
- That underrun is counted in `begin_block_play`'s missing-PCM branch (`:1153-1176`):
  `underrun_frames`, `underrun_events` and `SourceReadReport::underrun_event`. The report type is at
  `:404-423`, the render-owner telemetry at `:438-448` and `:1290-1300`.
- The graph source driver folds `report.underrun_event` into the graph-wide
  `GraphObservationValidity::source_underrun` (`crates/source/src/lib.rs:1722`; the type is
  `crates/graph/src/lib.rs:2514-2522`), which host-core spectrum windows retain
  (`crates/host-core/src/spectrum.rs:409`, `:448`). So a held block marks the window as starved.
- The header says of the wait before an anchor: "those blocks count as source underruns"
  (`crates/capi/include/miso_engine_v1.h:106-107`). The C ABI exposes no underrun counter at all,
  so the clause promises a report that does not exist.
- Existing tests that read underrun counts around held seeks: the anchored-seek tests near
  `crates/source/src/lib.rs:2265-2330` (`consumer.underrun_events` asserted at `:2316`) and
  `prepare_seek_holds_an_anchored_seek` (`:2197`).

## Decisions frozen for this slice

- **D1. Definition.** A missing-PCM block is **held** when `held_seek.is_some()` after this block's
  observe step (the held seek did not apply at this block). Otherwise it is an underrun, as today.
  The end-of-region rule is unchanged (past the end is neither).
- **D2. No bit moves.** Held changes classification only. The state machine (`next_frame` advance,
  block retention, generation handling) is untouched, so every rendered sample is bit-identical to
  today.
- **D3. Report fields.** `SourceReadReport` gains `held_frames: u32`, `held: bool`,
  `cumulative_held_frames: u64` and `cumulative_held_blocks: u64`. A held block has
  `underrun_frames == 0` and `underrun_event == false`. `PcmSourceConsumer` keeps the two saturating
  cumulative counters; `SourceConsumerTelemetry` gains `held_frames` and `held_blocks`.
- **D4. Graph-wide flag.** The source driver sets `block_validity.source_underrun` only from
  `report.underrun_event` and from a vacant entry (`:1715-1717`, unchanged). A held block sets
  nothing; `GraphObservationValidity` gets no new field (it is stream A's type, and a held block is
  not an invalidity).
- **D5. Disarm telemetry layout unchanged.** `copy_after_disarm_telemetry` (`:1803-1820`) keeps its
  five slots per source; held counts are not added there. The C host reads held and underrun
  counts through the per-source seek report of *Anchor every seek on the plan's source-read clock*
  (#1316).
- **D6. Docs.** The host-core facade's `seek_at` doc ("reports an underrun",
  `crates/host-core/src/source.rs:219-221`) is corrected by #1316, which owns that file's change.
  Delete "and those blocks count as source underruns" from
  `miso_engine_v1.h:106-107`. The full replacement text is #1317's.

## Deliverables

1. D1-D4 in `crates/source/src/lib.rs`, with the existing held-seek tests updated to the new split.
2. D6 in the header.

## Authorized paths

- `crates/source/src/lib.rs`
- `crates/capi/include/miso_engine_v1.h` (the one clause)

## Non-goals

- Exposing the counters through the C ABI (#1316) or the browser status (stream H).
- Changing underrun behaviour, adding a discard of behind-position blocks, or touching
  `GraphObservationValidity`.

## Objective gates

1. **Held is not an underrun.** Unit test in `crates/source/src/lib.rs`: a consumer playing
   generation 1 with no PCM queued; `try_seek(SourceCommand::SeekAt { generation: 2, .. })` anchored at block 4; render blocks 0..3
   with `begin_block_at`: every report has `held == true`, `held_frames == quantum`,
   `underrun_event == false`; cumulative held blocks 4, underrun events 0. Block 4 applies the seek;
   with generation 2's first block queued it plays (no held, no underrun); without it, block 4 is an
   underrun (the seek applied, the host owes PCM).
2. **Old PCM still plays while held.** Same setup with generation 1 PCM queued for blocks 0..1:
   blocks 0-1 copy PCM (neither held nor underrun), blocks 2-3 are held.
3. **Graph flag.** Driver test beside `graph_driver_forwards_underrun_and_seek_generation_facts`
   (`:2658`): a held block leaves `observation_validity().source_underrun` false; a true underrun on
   another source in the same block sets it.
4. **No bit moved.** The existing `crates/source` and `crates/host-core/tests/successor_swap.rs`
   anchored-seek bit-identity tests pass unchanged (their PCM assertions untouched).
5. Commands:
   - `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if a held block still counts as an underrun, or if the classification wrongly keeps
  "held" after the seek applied (the block-4 starvation case must stay an underrun).
- Gate 2: red if held is decided by `held_seek` alone instead of "missing PCM while held", which
  would hide playing PCM blocks as held.
- Gate 3: red if the driver still ORs held blocks into the graph-wide underrun flag.
- The amended assertions near `:2316` replace the old underrun-count expectations for held blocks;
  no test is deleted.

## Dependencies

- none
