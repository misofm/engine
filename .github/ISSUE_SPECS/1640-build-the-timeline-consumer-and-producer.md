# Build the timeline consumer and producer in the source crate

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2, A1.3), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R1.

## Product outcome

`crates/source` gains the session's playhead as two types: a **timeline consumer**, a source
consumer with no PCM and no ring, and a **timeline producer**. The consumer starts at generation
1, timeline sample 0, advances one quantum on every block, and takes plain and anchored seeks
through the same seek state machine a source uses, now factored so that no rule is written twice.
It keeps a short history, so a reader can ask "what was the timeline at this source-read sample?"
for any sample a node of the plan can still be processing.

This slice is crate-local. #1641 *Give every plan a timeline that carries like a source* puts
the consumer in every host-core plan and carries it; #1647 is its first render reader. Both are
in batch R1, so `main` never holds the types with no caller.

## Context

- **Every seek names one source.** `SourceCommand::{Seek, SeekAt}`
  (`crates/source/src/lib.rs:51-78`). The producer checks generation zero, a strictly increasing
  generation and an anchor that is a quantum multiple, then pushes into a one-slot command queue
  and switches its own generation (`try_seek`, `:753-785`; anchor check `:771-775`; push `:776-784`).
  A full slot is `SourceSeekError::Backpressure` and changes nothing (`:783`).
- **The consumer's seek state machine.** The consumer holds `active_generation` (`:1001`) and an
  optional `HeldSeek` (`held_seek`, `:1018`; struct `:1021-1027`). `observe_seek_at_block_boundary`
  (`:1323-1365`) pops one command: a plain seek clears any held seek and applies at once; an
  anchored seek replaces the held one; a held seek applies in the first block whose sample is at
  or past its anchor, at `frame + lateness` (`:1350-1364`). The source part of the same function
  discards a pending block of a replaced generation (`:1340-1346`); `apply_seek` repositions the
  PCM (`:1367-1386`).
- **Underrun still advances.** A block with no playable PCM advances `next_frame` by one quantum
  (`:1154-1168`). The timeline advances the same way on every block.
- **Source bytes.** The ring's one-slot command queue is a reported row
  (`SourceResourceReport::command_queue_bytes`, `crates/source/src/lib.rs:221-222`). The source
  set's overhead is summed in `prepare_graph_source_set` (`:1904-1936`).
- **Sources have no offset.** Every region is `0..frames`
  (`crates/host-core/src/prepare.rs:1231-1253`), so timeline sample `t` is source frame `t` when
  the sources play in step (README A1.2).

## Decisions frozen for this slice

- **D1. One seek state machine, factored.** `crates/source` gains a private seek-state type that
  owns `active_generation` and the held anchored seek, and one function that observes one popped
  command against a `SeekClock` and returns the seek to apply now, if any: generation and frame,
  lateness added. The source consumer and the timeline consumer both call it. Source-only work
  (discarding a pending PCM block of a replaced generation, `apply_seek`) stays in the source
  consumer and runs on the result. The producer side is factored the same way: one seek-producer
  type owns the command producer and the active generation, with `check_seek(command)` (every
  `try_seek` rule except the push) and `try_seek(command)` (`check_seek`, then the push). The PCM
  producer and the timeline producer both hold one. No rule is written twice.
- **D2. The timeline consumer.** `TimelineConsumer`: the D1 seek state, a command consumer (the
  one-slot queue a ring uses), `position: u64` (the timeline sample of the next block) and the
  history (D3). No PCM, no data queue, no recycle queue.
  - At construction it is generation 1, position 0.
  - `begin_block_at(source_read_sample)`: observe one command (D1); a seek to apply sets
    `position` to its frame; record `(source_read_sample, position)` in the history; then
    `position += quantum`. It runs on every block, silent or underrun, so the timeline never
    stops. Allocation-free, lock-free, bounded.
- **D3. History.** A fixed ring of `K` entries of 16 bytes,
  `(source_read_sample: u64, timeline: u64)`, one per block, newest last. `K` is a constructor
  argument (#1641 sets `K = ⌈A_max / q⌉ + 1`).
  - Reader `timeline_at(source_read_sample) -> i64`: search the entries newest first for the
    block that contains the sample (`entry.source_read_sample <= s < entry.source_read_sample + q`)
    and return `entry.timeline + (s - entry.source_read_sample)`. A sample older than the oldest
    entry extrapolates back from the oldest entry: `oldest.timeline - (oldest.source_read_sample -
    s)`, which is negative before a fresh plan's first block. So a fresh plan gives node time
    `-a(n)` at render sample 0 (README A1.4, "New lanes").
  - Reader `timeline_block(first_source_read_sample, q) -> TimelineSpan`: the node-time mapping of
    one node block: `start: i64` and an optional step `(offset: u32, timeline: i64)` at the first
    sample of a history entry whose timeline is not its previous entry's `+ q` (a seek). A node
    block covers at most two source-read blocks, so it has at most one step. The step may fall at
    the node block's first sample, when the node block starts exactly where the seek's source-read
    block starts: the reader then reports `step = (0, t)`, `t` the seek's timeline value, and
    `start` equal to `t`. So a seek that reaches a node block always yields a step, at any offset
    from 0 to `q - 1`, and a reader never has to compare `start` with the previous block's end to
    find a seek. #1647 copies it into #1637's `NodeSpan`.
  - `copy_history_from(&other)`: copies the newest `min(K_other, K_self)` entries, oldest first, a
    bounded copy with no allocation (#1641's carry).
  - Every value is an exact integer. No floating point.
- **D4. The timeline producer.** `TimelineProducer`, the D1 seek producer plus nothing else. Plain
  and anchored seeks, generation strictly above the current one, anchor a quantum multiple, one
  slot: a full slot returns `SourceSeekError::Backpressure` and changes nothing. It exposes
  `check_seek` (D1), so an all-or-nothing caller (#1642) checks every consumer before it pushes
  any. A seek frame above `TIMELINE_SAMPLE_LIMIT = 2^53` is refused with a new
  `SourceSeekError::FrameOutOfRange`. The limit is above any session's length (2^53 samples is
  about 2,970 years at 96 kHz) and leaves `2^63 - 2^53` samples of headroom, so every timeline
  value and every node time derived from it stays an exact `i64` for any run a host can render.
- **D5. Bytes.** `TimelineResourceReport` states the timeline's fixed bytes row by row: the one-slot
  command queue (the ring's `command_queue_bytes` formula), the history (`16·K`) and the consumer.
  #1641 adds them to the source set's overhead.
- **D6. The acked-batch question.** The timeline's command queue is a source's one-slot queue: a
  seek that does not fit is refused with backpressure before anything changes, and a held seek is
  never dropped. No ack can precede a drop.

## Deliverables

1. D1-D5 in `crates/source/src/lib.rs`, with unit tests.

## Authorized paths

- `crates/source/src/lib.rs`, `crates/source/tests/`
- `crates/host-core/src/source.rs` (the `FrameOutOfRange` arm of `SourceControlError::diagnostic`,
  `:78-104`, only: that match has no wildcard and `SourceSeekError` is not `#[non_exhaustive]`,
  `crates/source/src/lib.rs:381-393`, so host-core does not compile without it) and
  `crates/host-core/tests/source_diagnostics.rs` (its `variant_index` and `TABLE`, `:27`,
  `:176-198`, gain the new row)

## Non-goals

- Putting the consumer in a plan, its carry, its producer in `SourceControlSet` and the diagnostic
  name (#1641). The C call (#1642), the browser export and SDK `seek` (#1643 and #1644),
  and any automation reader (#1637, #1646 and #1647).

## Hazards

- **Hot file.** `crates/source/src/lib.rs` is stream B's (#1316, #1318, #1320). Their source slices
  merge first (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`); this slice rebases onto them and
  factors their seek code, never a copy of it.

## Objective gates

1. **Seek state machine** (`crates/source/src/lib.rs` unit tests, new). On a `TimelineConsumer`
   at quantum 128:
   - a plain seek to `(2, 10_000)` reads 10,000 at the next block's first sample;
   - an anchored seek `(2, F, A)` is held while the block sample is below `A` (the timeline keeps
     advancing) and applies at `A` with frame `F`; observed late at `A + 256`, it applies at
     `F + 256`;
   - a newer seek, plain or anchored, replaces a held one;
   - a stale or zero generation and an unaligned anchor are refused by the producer, which keeps
     its generation; a second seek before the consumer pops the first is `Backpressure` and
     changes nothing; a frame above `2^53` is `FrameOutOfRange`;
   - the timeline advances by 128 on every block;
   - `timeline_at` reads the right value on both sides of a seek inside its window and
     extrapolates below the oldest entry; `timeline_block` returns the step offset for a node
     block that straddles the seek's block, and `step = (0, t)` for a node block that starts
     exactly at the seek's block;
   - `copy_history_from` from a larger and from a smaller `K` keeps the newest entries in order.
2. **Shared rules.** The existing seek tests of the source consumer
   (`a_newer_anchored_seek_replaces_a_held_one`, `a_primed_block_waits_for_its_anchor`,
   `prepare_seek_holds_an_anchored_seek`, `an_unaligned_anchor_is_refused_without_a_generation_switch`,
   `crates/source/src/lib.rs:2112-2218`) and `crates/source/tests/seek_schedule_model.rs` pass
   unchanged.
3. **No rendered bit moves.** No render arithmetic changes; every existing render test passes with
   unchanged digests.
4. **Commands:**
   - `cargo test --locked -p source`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule),
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the timeline applies an anchored seek without its lateness, lets a held seek
  survive a newer command, reads the wrong side of a seek in its history, reports no step for a
  seek at a node block's first sample, or copies the wrong end of a history. Nothing tests a
  consumer with no PCM today.
- Gate 2 turns red if the factoring changes a source's seek behaviour.

## Dependencies

- Ordering only: stream B's `crates/source` slices, *Anchor every seek on the plan's source-read
  clock* (#1316), *Report held source blocks apart from underruns* (#1318) and *Let a source
  consumer check and replay its next blocks for a prime* (#1320), merge first: D1 factors their
  seek code (Hazards).
- Batch: R1. #1641 depends on this draft.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Realtime policy.** `scripts/check-realtime-policy.sh` is deleted; the check is
  `cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`, and its
  floors are `marked_files_floor` and `marked_regions_floor` in `tools/realtime-policy/src/policy.rs:161-222`.
- **Dependencies.** #1316, #1318 and #1320 are still open; the ordering rule stands. The source
  crate's seek anchors have moved; re-read them at filing.
- No decision 17 change: the timeline is source transport, not a control.

## Amendment (controls coordinator, batch 16 check, 2026-10-16)

Checked against `35ac17545`.

- **Landed deps.** #1316, #1318 and #1320 are closed and on `main`. The ordering rule is met.
- **Moved anchors (`crates/source/src/lib.rs`).** `SourceCommand` `:56-87`. `SourceSeekError`
  `:394-405` (still not `#[non_exhaustive]`). `PcmSourceProducer` `:819-835`; `try_seek`
  `:888-919` (anchor check `:905-910`, push `:911-918`). `HostChunkProvider::try_seek` `:1039-1041`
  forwards to it. `PcmSourceConsumer` `:1154-1194`; `held_seek` `:1181`; `HeldSeek` `:1199-1205`;
  `SeekClock` `:1208-1222`. `observe_seek_at_block_boundary` `:1591-1633`; `apply_seek`
  `:1638-1654`. Underrun advance: `begin_block_play` `:1372-1404`. `prepare_graph_source_set`
  `:2421-2497`. Gate 2's four tests are at `:2654-2770`.
- **`SeekClock` has three cases now:** `Now`, `At { source_read_sample }` and `Hold` (#1320
  preparation and prime). D1's shared function takes all three.
- **The source observes a command at two points.** `begin_block_observe` and the acquire's one
  re-observe (`acquire_current_block`, `:1772-1838`, #1274 MINOR-3). Both call the D1 function. The
  timeline has no acquire, so it observes once per block.
- **The seek state is read outside the observe step too:** `settle_block`,
  `is_unobserved_generation`, `prime_ready` and the held/underrun split in `begin_block_play`. The
  D1 type gives read access to the held seek; no rule moves out of it.
- **Producer factoring.** `try_seek`'s post-push work (`next_write_frame`, `end_of_region_submitted`)
  is PCM-only and stays in `PcmSourceProducer`. The shared seek producer needs the quantum for the
  anchor check.
- **`check_seek` on the PCM side (for #1642 D1 step 2).** `PcmSourceProducer` and
  `HostChunkProvider` (the type `SourceControlSet` holds) both expose `check_seek(command)`,
  forwarding to the shared seek producer, as `try_seek` already does.
- **The seek report (#1316) is per ring, not a table.** One seqlock record
  (`PublishedWordsWriter<SEEK_REPORT_WORDS>`, 5 words, `crates/engine/src/realtime/seqlock.rs`)
  per ring. The consumer publishes it (`publish_seek_report`, `:1575-1589`) and the producer reads
  it (`seek_report`, `:924-929`). If #1641 D6 stands, the timeline gets the same record through
  the same writer and reader, unchanged. D5 then also needs a seek-report row
  (`published_words_retained_bytes::<5>()`, as `SourceResourceReport::seek_report_bytes`).
- **Diagnostic name.** The Non-goals say "the diagnostic name (#1641)". #1641 D2 overrides that:
  this slice adds the arm as `source.seek.frame_out_of_range`. Host-core anchors moved:
  `SourceControlError::diagnostic` `crates/host-core/src/source.rs:86-118`;
  `crates/host-core/tests/source_diagnostics.rs` `TABLE` `:27`, `variant_index` `:182-205`
  (the new row is index 20). No other crate matches on `SourceSeekError`.
- **No `crates/engine/src/realtime/` edit.** The SPSC and seqlock types are used as they are.
- **Decision 17.** No change. C13 (ruling 2026-10-15) calls seeks and the seek report transport.

## Decision record: #1640 (batch 16)

Built on `eab696d09` (worktree `codex/controls-batch-16a`).

**What shipped** (`crates/source/src/lib.rs`):

- **D1.** `SeekState` (owns `active_generation` and `held_seek`) with one `observe(command,
  clock)` over all three `SeekClock` cases, plus the shared reads `holds_anchor_before`,
  `holds_generation` and `is_unobserved`. The PCM consumer calls `observe` at both of its points
  (`begin_block_observe` and the acquire's re-observe, both through
  `observe_seek_at_block_boundary`) and runs only source work on the result: the stale pending
  block discard on a newly held seek, and `apply_seek`. `prime_ready`, `settle_block` and
  `is_unobserved_generation` read the held seek through the shared methods, so B16-R3's readiness
  rule exists once for #1641. `SeekProducer` (command producer, generation, quantum, optional frame
  limit) has `check_seek` and `try_seek`. `PcmSourceProducer::try_seek` is the shared `try_seek`
  plus its PCM-only work (`next_write_frame`, `end_of_region_submitted`).
  `PcmSourceProducer::check_seek` and `HostChunkProvider::check_seek` forward to the shared check.
- **D2-D3.** `TimelineConsumer` (`begin_block_at`, `timeline_at`, `timeline_block`,
  `copy_history_from`, `position`, `active_generation`, `history_capacity`), with history entries
  `(u64, u64)` in a fixed `K`-entry ring. `SourceTimeline::prepare(quantum, K)` builds it at
  generation 1, position 0. `TimelineSpan { start, step: Option<TimelineStep { offset, timeline }> }`.
- **D4.** `TimelineProducer` (`try_seek`, `check_seek`, `active_generation`),
  `TIMELINE_SAMPLE_LIMIT = 1 << 53`, `SourceSeekError::FrameOutOfRange`.
- **D5.** `TimelineResourceReport`: `history_blocks`, `command_queue_bytes` (the ring formula),
  `command_queue_largest_allocation_bytes` (#1641 D7) and `history_bytes` (`16·K`). No
  seek-report row (B16-R4). The batch 16a fold removed `consumer_bytes`, `overhead_bytes` and
  `largest_allocation_bytes` (see the fold paragraph).
- **Host-core.** The `FrameOutOfRange` arm, `source.seek.frame_out_of_range`, and its row 20 in
  `crates/host-core/tests/source_diagnostics.rs`.

**Choices the text left open:**

- `check_seek` reports a full slot as `Backpressure` with the current `full_count`, through
  `available_capacity`. It does not count a refusal. `try_seek` still gets backpressure from its own
  push, so the PCM producer's full count moves exactly as before.
- The frame limit applies to the timeline only (`frame_limit: None` for a PCM source). The rule is
  written once in the shared check.
- A seek in the history's oldest entry is still a step: the consumer keeps one `u64`,
  `oldest_expected_timeline`, which is the evicted entry's timeline plus `q`, or the starting
  position. `copy_history_from` carries it.
- `timeline_at` with no entry treats the position as standing at source-read sample 0. A sample in
  a gap or after the newest entry extrapolates forward from the newest entry that starts at or
  before it. A step is reported at the first sample `s` of the node block where
  `timeline_at(s) != timeline_at(s - 1) + 1` (a seek, or a source-read step-back's overlap; fold
  M1), so for a node block with at most one step, which is every block without a step-back inside
  it, the step and `timeline_at` agree on every sample. Of two steps, the first is reported.
- `timeline_block(first, frames)` takes the node block's length as `frames`.

**Departures:**

- Gate 2 "unchanged": the assertions are unchanged. Two of the four named tests, seven others and
  the shared `twin` helper read the moved fields, so they now read `consumer.seek.active_generation` and
  `consumer.seek.held_seek` (a path rewrite only). `crates/source/tests/seek_schedule_model.rs` is
  textually unchanged. `crates/source/tests/randomized.rs` gains one `unreachable!` arm for the new
  variant (exhaustive match).
- `tools/realtime-policy/src/policy.rs`: `marked_regions_floor` 151 -> 153, for the two new regions
  (mechanical, outside the authorized paths).

**For #1641:** the consumer value is inline. #1641 stores it inside an allocation it already
counts (the driver), so no row charges it (fold MINOR 6 removed `consumer_bytes`).

**Gates:**

- Gate 1, `cargo test --locked -p source`, `tests::timeline::*` (11 new; the fold replaced the
  first two, see below):
  `an_anchored_seek_is_held_until_its_anchor_and_applies_with_its_lateness`,
  `a_newer_seek_plain_or_anchored_replaces_a_held_one`,
  `the_producer_refuses_without_changing_anything`,
  `the_history_reads_both_sides_of_a_seek_and_extrapolates_below_its_oldest_entry`,
  `a_node_block_reports_the_step_of_a_seek_inside_it_or_at_its_first_sample`,
  `a_seek_in_the_oldest_entry_is_still_a_step`,
  `copying_a_history_keeps_its_newest_entries_in_order`,
  `the_resource_report_states_each_row` and
  `a_source_check_seek_agrees_with_its_try_seek_and_pushes_nothing`. PASS.
- Gate 2: all 68 lib tests (the four named seek tests among them), `seek_schedule_model`,
  `randomized` and `prime`. PASS.
- Gate 3: no render arithmetic changed. The source tests, plus host-core `successor_swap` (14)
  and `prepare` (139), PASS.
- Commands, all PASS: `realtime_policy` (153 regions in 33 files), `check-cross-targets.sh`,
  `check-workspace-policy.sh`, `cargo test -p realtime-policy`, `source_diagnostics` (2),
  clippy `-p source -p host-core --all-targets --all-features -D warnings`, and
  `cargo fmt --all -- --check`. The workspace-wide clippy was not run (worker rules).

**Fold (batch 16a review).**

- **M1.** `timeline_block` reported a step only at a seek entry, so a node block that straddles a
  source-read step-back (#1641 D4) read part of its samples a whole overlap off (`q = 128`, blocks
  at 0, 128, 256, 384, 256: `timeline_block(216, 128)` gave no step while `timeline_at(256)` is
  512). It now checks every entry's first sample and its end (`sr + q`) inside the node block and
  reports the first sample `s` where `timeline_at(s) != timeline_at(s - 1) + 1`. Where no held entry
  starts at or before `s - 1` (or `s` is 0), the entry at `s` is a step when it is a seek, read
  against `oldest_expected_timeline` as before. Bound: at most `6K² + 6K` entry reads,
  allocation-free. Test `a_node_block_reports_the_steps_of_a_source_read_step_back` (red on the
  unfixed code: `{start: 216, step: None}`), which also checks every block's span against
  `timeline_at` sample by sample.
- **MINOR 4.** `the_reader_reads_the_newest_overlap_and_reports_the_first_step` (`K` 4 and 3): red
  for a `resolve` searched oldest first, a later step reported instead of the first, and an
  eviction that does not update `oldest_expected_timeline` (each mutation run).
- **MINOR 5.** D6 and `TimelineProducer::try_seek` now read: an accepted seek applies unless a newer
  accepted seek replaces it first (#1274); it is never dropped silently.
- **MINOR 6.** `TimelineResourceReport` drops `consumer_bytes`, `overhead_bytes` and
  `largest_allocation_bytes`: nothing read them after #1641, and the two sums counted the inline
  consumer, which no allocation of the timeline holds. No charged byte moved (the source set never
  read them).
- **MINOR 7.** `offset_timeline` is inside the marked region (its END marker moved below it; 153
  regions, unchanged).
- **Test value.** `a_fresh_timeline_starts_at_zero_and_advances_a_quantum_every_block` and
  `a_plain_seek_reads_its_frame_at_the_next_blocks_first_sample` are deleted (no unique catch: the
  anchored, newer-seek and history tests cover both). `the_resource_report_states_each_row` lost
  its `consumer_bytes` check with the row. New: `a_vacant_timeline_never_adopted_advances_from_zero`
  (#1641's, in this module).
- Rustdoc: the public docs no longer link the private `SeekState`, `SeekClock::At` and
  `SeekState::prime_window_clear` (`cargo doc -D warnings`).
