# Give every plan a timeline clock that seeks and carries like a source

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2, A1.3), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

Every prepared plan, on both hosts, carries one **timeline consumer**: the session's playhead. It is
a source consumer with no PCM and no ring. It starts at generation 1, timeline sample 0, advances
one quantum on every block, and takes plain and anchored seeks through the same seek state machine
a source uses. It keeps a short history, so render can ask "what was the timeline at this
source-read sample?" for any sample a node of the plan can still be processing. A successor plan
carries it, history included, exactly as it carries an unchanged source.

Nothing renders from it yet. The C call that seeks it is draft 05, the browser export and the SDK
drafts 06a and 06b, and the first reader draft 09b.

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
- **The graph's source driver.** `SourceGraphSourceSetDriver` (`:1611-1617`) calls
  `consumer.begin_block_at(first_sample)` for each source at block entry
  (`:1701-1736`, the call `:1720-1721`). The graph executor calls the driver from its block entry
  (`crates/graph/src/lib.rs:3254-3256`). The source set is an `Option` on the executor
  (`:3254`), but host-core always builds one (`crates/host-core/src/prepare.rs:1635-1636`), and
  `prepare_graph_source_set` refuses an empty source list (`crates/source/src/lib.rs:1901-1903`),
  so every host-core plan has a source set.
- **Sources have no offset.** Every region is `0..frames`
  (`crates/host-core/src/prepare.rs:1231-1253`), so timeline sample `t` is source frame `t` when
  the sources play in step (README A1.2).
- **How an unchanged source carries (#1273).** Preparation finds the predecessor's inventory row,
  allocates no ring and pushes a vacant entry (`crates/host-core/src/prepare.rs:1213-1240`). The
  carry program lists `(successor, predecessor)` source pairs (`crates/graph/src/lib.rs:2750-2771`)
  and is built only when something carries (`crates/host-core/src/prepare.rs:1294-1302`). At the
  swap block `adopt_predecessor` (`crates/graph/src/lib.rs:3139-3177`) calls the driver's
  `adopt_sources`, which swaps each consumer into its vacant entry with no allocation
  (`crates/source/src/lib.rs:1845-1888`). The host moves the producers first:
  `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:273-301`); the C ABI calls
  it at commit (`crates/capi/src/runtime/control.rs:1019`). `SourceControlSet` is the per-session
  producer table (`crates/host-core/src/source.rs:127-130`).
- **Source bytes.** The ring's one-slot command queue is a reported row
  (`SourceResourceReport::command_queue_bytes`, `crates/source/src/lib.rs:221-222`). The source
  set's overhead is summed in `prepare_graph_source_set` (`:1904-1936`), and host-core checks it,
  plus the carried rings' overhead, against `maximum_source_overhead_bytes`
  (`crates/host-core/src/prepare.rs:1640-1652`).
- **Node arrival.** PDC computes each node's input arrival `max` in `timings`
  (`crates/graph-compiler/src/pdc.rs:53-104`). The compiled graph does not export it per node;
  *Keep every node's latency from dropping during playback* (#1285) D2 records each node's floored
  `max` in the plan inventory. `A_max`, the largest of them, sizes the history (D3).
- **Readers of the plan.** A test reaches the executor through
  `PreparedRenderPlan::executor_any_mut` (`crates/engine/src/realtime/plan.rs:912`); graph exposes
  plan facts through free functions that downcast, for example `carry_program_retained_bytes`
  (`crates/graph/src/lib.rs:2911-2927`).

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
- **D2. The timeline consumer.** `TimelineConsumer` in `crates/source`: the D1 seek state, a
  command consumer (the one-slot queue a ring uses), `position: u64` (the timeline sample of the
  next block) and the history (D3). No PCM, no data queue, no recycle queue.
  - At compile it is generation 1, position 0.
  - `begin_block_at(source_read_sample)`: observe one command (D1); a seek to apply sets
    `position` to its frame; record `(source_read_sample, position)` in the history; then
    `position += quantum`. It runs on every block, silent or underrun, so the timeline never
    stops. Allocation-free, lock-free, bounded.
  - The graph driver calls it at block entry, before the sources, with the same sample it passes
    the sources (`crates/source/src/lib.rs:1720-1721`). Until *Give a plan a source-read clock that
    leads its render clock* (#1396) lands, that sample is the render sample; #1396 makes it the
    source-read sample, and this slice needs no change for it.
- **D3. History.** A fixed ring of `K = ⌈A_max / q⌉ + 1` entries of 16 bytes,
  `(source_read_sample: u64, timeline: u64)`, one per block, newest last. `q` is the plan's
  quantum and `A_max` the plan's largest floored node arrival (#1285 D2), 0 for a plan with no
  latency (`K = 1`).
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
    find a seek. Draft 09b copies it into draft 07's `NodeSpan`.
  - Every value is an exact integer. No floating point.
- **D4. The timeline producer.** `TimelineProducer` in `crates/source`, the D1 seek producer plus
  nothing else, owned by `SourceControlSet` beside the source producers, and moved by
  `adopt_persisting` like a persisting source's producer. Plain and anchored seeks, generation
  strictly above the current one, anchor a quantum multiple, one slot: a full slot returns
  `SourceSeekError::Backpressure` and changes nothing. It also exposes `check_seek` (D1), so an
  all-or-nothing caller (draft 05) checks every consumer before it pushes any. A seek frame above
  `TIMELINE_SAMPLE_LIMIT = 2^53` is refused with a new `SourceSeekError::FrameOutOfRange`
  (`source.seek.frame_out_of_range` in `SourceControlError::diagnostic`,
  `crates/host-core/src/source.rs:78-103`). The limit is above any session's length (2^53
  samples is about 2,970 years at 96 kHz) and leaves `2^63 - 2^53` samples of headroom, so every
  timeline value and every node time derived from it stays an exact `i64` for any run a host can
  render. Host-core gains no public timeline seek here; draft 05 adds the session seek.
- **D5. Carry.** A successor plan carries the timeline consumer exactly as it carries an
  unchanged source, on every successor (it never changes):
  - preparation builds a vacant timeline entry that owns its own history of its own `K`, and no
    command queue;
  - the carry program gains `timeline: bool`, set on every successor, so a program exists even
    when no source or input lane carries (today it is built only then,
    `crates/host-core/src/prepare.rs:1294-1302`);
  - at the swap block the driver's `adopt_sources` swaps the seek state, the command consumer and
    `position` into the vacant entry and copies the predecessor's newest
    `min(K_predecessor, K_successor)` history entries into the successor's history, oldest first. A
    bounded copy; nothing is allocated or freed;
  - the producer moves through `adopt_persisting`;
  - a vacant timeline that is never adopted (a carry mismatch, unreachable on both hosts by
    #1269 P11) advances from 0 with no command queue, so render stays defined.
  - **Why the successor's own window is enough.** The successor reads the history only for its own
    nodes at its own arrivals. Floors (#1285) never lower a carried node's arrival, and a warm
    adoption raises both a carried node's floor and the source-read clock by `P` (README A1.3), so a
    carried node never reads further back than it did on the predecessor, and the predecessor's
    `K` covered that. Only a node that the successor adds, restarts or grows can read further
    back. Such a node starts at rest at adoption (#1324, #1397), so the samples it reads before its
    first input arrives label rest state, and the extrapolation of D3 gives them a defined,
    host-independent value that is exact unless a seek fell in the missing span. From the
    successor's `⌈A_max / q⌉`-th block on, its own entries fill its window.
- **D6. Bytes.** The timeline's bytes are fixed: the one-slot command queue (the ring's
  `command_queue_bytes` formula), the history (`16·K`) and the consumer. A `TimelineResourceReport`
  states them row by row; `prepare_graph_source_set` adds them to the source set's
  `overhead_bytes` and `largest_allocation_bytes`, so the source overhead cap bounds them on both
  hosts. A vacant (carried) timeline charges only its history; its carried command queue is
  counted with the carried source overhead, as a carried ring's is
  (`crates/host-core/src/prepare.rs:1220-1230`).
- **D7. Test reader.** `graph` cannot name a `crates/source` type (`source` depends on `graph`),
  so the source-set driver trait (`crates/graph/src/lib.rs:2161-2232`) gains one defaulted method,
  `timeline_at(&self, source_read_sample: u64) -> Option<i64>` (`None` by default), which the source
  driver implements with D3's reader. `graph::test_only_timeline_at(plan: &mut PreparedRenderPlan,
  source_read_sample: u64) -> Option<i64>` (`test-support`) downcasts as
  `carry_program_retained_bytes` does and calls it. The render-side block reader comes with
  draft 09b.
- **D8. The acked-batch question.** The timeline's command queue is a source's one-slot queue: a
  seek that does not fit is refused with backpressure before anything changes, and a held seek is
  never dropped (it moves with its consumer across a swap). No ack can precede a drop.

## Deliverables

1. D1-D4 and D6 in `crates/source/src/lib.rs`, with unit tests.
2. D5 in `crates/host-core/src/prepare.rs` (vacant timeline, `K` from `A_max`, the carry program
   flag), `crates/host-core/src/source.rs` (the producer in `SourceControlSet`, its move in
   `adopt_persisting`, the new diagnostic) and `crates/graph/src/lib.rs` (the carry program's
   `timeline` flag and D7).
3. Host-core tests in `crates/host-core/tests/timeline.rs` (new) and one case in
   `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/source/src/lib.rs`, `crates/source/tests/`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/source.rs`,
  `crates/host-core/tests/timeline.rs` (new), `crates/host-core/tests/successor_swap.rs`
- `crates/graph/src/lib.rs`: `GraphCarryProgram`'s `timeline` field, the `adopt_predecessor`
  condition that calls `adopt_sources`, the driver trait's `timeline_at` and D7's test reader only
- Tests that pin a source overhead or carry-program byte count that D6 or D5 moves (re-pin each
  with its reason, never in bulk)

## Non-goals

- The C call (draft 05), the browser export and SDK `seek` (drafts 06a and 06b), and any automation
  reader (drafts 07, 09a and 09b).
- The seek report (*Anchor every seek on the plan's source-read clock*, #1316), whose row for the
  timeline is an amendment to #1316 (README amendment table).
- The prime (*Let a source consumer check and replay its next blocks for a prime*, #1320), the
  declared discontinuity (*Reset latency floors at a host-declared discontinuity*, #1323) and the
  source-read offset (#1396). Each of those specs gains one line for the timeline (README amendment
  table); this slice does not implement them.
- `TRANSPORT_SET`'s stored position (README finding F4).
- A plan with no source set. Every host-core plan has one; the refusal of an empty source list
  (`crates/source/src/lib.rs:1901-1903`) is unchanged.

## Hazards

- **Hot file.** `crates/source/src/lib.rs` is stream B's (#1316, #1318, #1320). Their source slices
  merge first (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`); this slice rebases onto them and
  factors their seek code, never a copy of it.
- **A declared discontinuity steps the source-read clock back.** At a discontinuity successor's
  adoption the source-read clock steps from `render + ΣP` to `render` (#1396 D3), so the history can
  hold two entries for one source-read sample. D3 searches newest first, which is the entry render
  played last. The nodes that read the overlap start at rest (#1323 D2), so no continuity is owed.
  #1323's carry line for the timeline is the README amendment row.
- **Order of the block entry.** The timeline must observe its command in the same block as a
  source given the same command. Both run in the one driver call at block entry; D2 puts the
  timeline first and applies no state to sources.
- **`A_max` comes from #1285's inventory.** Without it the history cannot be sized from the
  floored arrivals. This slice therefore lands after #1285 (see Dependencies).
- **Byte rows move.** Every source overhead row grows by the timeline's bytes. Each pinned number
  that moves is re-pinned with the reason "the timeline consumer's fixed bytes (draft 04 D6)".

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
   - the timeline advances by 128 on every block whether the source beside it underruns or plays;
   - `timeline_at` reads the right value on both sides of a seek inside its window, and
     `timeline_block` returns the step offset for a node block that straddles the seek's block,
     and `step = (0, t)` for a node block that starts exactly at the seek's block.
2. **Shared rules.** The existing seek tests of the source consumer
   (`a_newer_anchored_seek_replaces_a_held_one`, `a_primed_block_waits_for_its_anchor`,
   `prepare_seek_holds_an_anchored_seek`, `an_unaligned_anchor_is_refused_without_a_generation_switch`,
   `crates/source/src/lib.rs:2112-2218`) and `crates/source/tests/seek_schedule_model.rs` pass
   unchanged.
3. **Source and timeline agree** (`crates/host-core/tests/timeline.rs`, new). A one-track session
   at 48 kHz, quantum 128, a playing source. Push the same seek, `(g, F, A)` anchored and then
   `(g + 1, F2)` plain, to the source and to the timeline (through `TimelineProducer` directly).
   For ten blocks after each, the frame each block's source read starts at equals
   `test_only_timeline_at` of that block's sample.
4. **Carried across a rebuild** (same file). Render 20 blocks, seek the timeline, render 5, then
   prepare and swap a successor (one added track). After the swap the timeline continues with no
   gap or repeat for 10 blocks, and `test_only_timeline_at` returns the predecessor's values for
   every sample in the successor's window. A successor with a latent insert (larger `A_max`) keeps
   the newest entries and extrapolates the rest per D3. The producer moves: a seek pushed through
   the successor's `SourceControlSet` reaches the carried consumer.
5. **No allocation in render.** The swap block and the ten blocks after it allocate and free
   nothing (`bench_support::alloc` thread counters after warming, as
   `the_swap_block_allocates_and_frees_nothing`, `crates/host-core/tests/successor_swap.rs:476`).
6. **No rendered bit moves.** No render arithmetic changes. Every existing render test passes with
   unchanged digests; `./target/release/audit capi` shows the same `pcm_digest` at base and head
   (PR evidence) and 0 violations.
7. **Commands:**
   - `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the timeline stops on a silent or underrun block, applies an anchored seek
  without its lateness, lets a held seek survive a newer command, reads the wrong side of a seek
  in its history, or reports no step for a seek at a node block's first sample. Nothing tests a
  consumer with no PCM today.
- Gate 2 turns red if the factoring changes a source's seek behaviour.
- Gate 3 turns red if the timeline observes a command one block before or after a source given the
  same command: the fault that would put every automation curve one quantum off its audio.
- Gate 4 turns red if a successor restarts the timeline at 0, drops its history or its producer, or
  sizes its history from the predecessor's `K`.
- Gate 5 turns red if the carry or the history write allocates on the render thread.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285): D2's recorded floored
  arrivals give `A_max` (D3).
- Ordering only: stream B's `crates/source` slices, *Anchor every seek on the plan's source-read
  clock* (#1316) and *Report held source blocks apart from underruns* (#1318), merge first: D1
  factors their seek code (Hazards).
- Batch: R1, with draft 09b *Render moving stored fader automation, seeks and latency*, its first
  reader. Drafts 05, 06a and 06b build on it.
