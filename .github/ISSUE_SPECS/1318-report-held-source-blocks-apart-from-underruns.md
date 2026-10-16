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
   - `cargo run --locked --release -q -p realtime-policy`
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

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.** Each fact in Context still holds; only the lines moved.

- `held_seek` is still `crates/source/src/lib.rs:1018`; `HeldSeek` is `:1022-1027`.
  `observe_seek_at_block_boundary` is `:1323-1365`. The `begin_block_at` doc is `:1106-1114`.
- The missing-PCM branch of `begin_block_play` is `:1154-1178` (the function is `:1135-1190`).
  `SourceReadReport` is `:404-423`, `SourceConsumerTelemetry` `:439-448`, `telemetry()`
  `:1290-1299`.
- The graph source driver's block entry is `:1752-1785`. The vacant-entry branch is `:1765-1769`;
  the fold of `report.underrun_event` is `:1773`.
- `copy_after_disarm_telemetry` is `:1854-1875`. It still writes five slots per source.
- `GraphObservationValidity` is `crates/graph/src/lib.rs:2927-2940`.
- Spectrum windows moved into the core service (#1591): the flag is retained in
  `crates/host-core/src/spectrum_service.rs` (`:496`, `:517`, `:1206`), not `spectrum.rs`. The
  browser copies it into its window record at `hosts/host-web/src/ffi.rs:1048` and `:1174`. No
  code changes there: a held block simply stops setting the flag.
- The header clause is `crates/capi/include/miso_engine_v1.h:214-215` ("and those blocks / count
  as source underruns").
- The host-core facade's `seek_at` doc is `crates/host-core/src/source.rs:209-222`; "reports an
  underrun" is `:222`.
- Tests: `prepare_seek_holds_an_anchored_seek` is `:2245`. The anchored-seek tests are now
  `:2160-2445`. The held-block underrun assertion that this slice changes is `:2364`
  (`assert_eq!(consumer.underrun_events, u64::from(anchored))` in
  `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm`). The region-end tests are
  `:2264-2310`. `graph_driver_forwards_underrun_and_seek_generation_facts` is `:2748`.

**Decisions.**

- D1-D6 hold against the code and decision 17.
- D1, made precise. `begin_block_play`'s acquire step can re-observe a command and set or apply
  a held seek (`acquire_current_block`, `:1394-1440`, #1274 MINOR-3). So read `held_seek` in the
  missing-PCM branch, after the acquire. That is the "after this block's observe step" of D1.
- The missing-PCM branch also advances `next_frame` by one quantum while held. D2 keeps that, so
  no bit moves.

**Dependencies on GitHub.** None. Downstream and open: #1316, #1317, #1320, #1387, #1537 and
#1640. `crates/source/src/lib.rs` is a hot file: #1640's hazard note orders stream B's source
slices (#1316, #1318, #1320) before it.

**Gates and commands.** Gate 4's tests exist (`crates/host-core/tests/successor_swap.rs`,
`anchored_swap_run`, `:629-830`). That file also compares spectrum windows' `source_underrun`
between a run and its reference (`:9438`). Both runs hold the same way, so it stays green. Every
named command exists.

**Realtime-path edits.** None under `crates/engine/src/realtime`. The change is on the source
consumer's render path in `crates/source/src/lib.rs`.

## Decision record: #1318 (batch 15b)

**Shipped.** D1-D4 and D6 in `crates/source/src/lib.rs` and the one header clause.

- `begin_block_play`'s missing-PCM branch reads `held_seek` after the acquire step (the batch 15
  precision of D1). Held: the missing frames go to new `held_frames` / `held_blocks` counters and
  `underrun_*` stays untouched. Otherwise it is an underrun as before. `next_frame` still advances by
  one quantum in both cases, so no rendered bit moves (D2).
- `SourceReadReport` gains `held_frames`, `held`, `cumulative_held_frames`, `cumulative_held_blocks`;
  `SourceConsumerTelemetry` gains `held_frames`, `held_blocks` (D3). Held frames use the same
  region-bounded count as underrun frames (a held block past the end is neither, unchanged).
- The graph driver needed no change: it folds only `report.underrun_event` and the vacant entry, so a
  held block sets nothing (D4). `GraphObservationValidity` and `copy_after_disarm_telemetry` are
  untouched (D5). The browser spectrum record and the C ABI read nothing new, so both hosts behave the
  same: a held block stops marking spectrum windows as starved on both. The held counters reach hosts
  with #1316 (C ABI) and #1537 (browser status), which are the non-goals here.
- Header: "and those blocks count as source underruns" deleted (`miso_engine_v1.h`, lines ~250-253,
  rewrapped at 100 columns). #1317 owns the full replacement text.
- D6 facade doc (`crates/host-core/src/source.rs` `seek_at`) left to #1316 as the spec says.

**Departures.** None. The B15-R/B15-Q rulings the task text names are not in this spec file.

**Gates -> tests** (all in `crates/source/src/lib.rs`):

1. `a_wait_for_an_anchor_is_held_and_starvation_after_it_is_an_underrun` (both the delivered and the
   undelivered case).
2. `queued_pcm_of_the_playing_generation_plays_while_a_seek_is_held`.
3. `graph_driver_does_not_flag_a_held_block_as_an_underrun` (held source plus a starved second source
   in one block).
4. Updated, not deleted: `a_primed_block_waits_for_its_anchor` and
   `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm` now assert held, not underrun.

**Red evidence** (temporary mutations, restored): classification forced to never-held turns five tests
red (the three new ones plus the two updated); the driver also OR-ing `report.held` turns only gate 3
red; deciding played-vs-missing by `held_seek` alone turns gate 2 and
`prepare_seek_holds_an_anchored_seek` red.

**Measured numbers.** None re-pinned. `TransferBlock` is unchanged; the consumer gains two `u64`
fields, with no resource-report row that counts them.
