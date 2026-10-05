# Let a source consumer check and replay its next blocks for a prime

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287). Code anchors
verified on `main` at `6fb211594`.

## Product outcome

Render can adopt a warm successor whose sources read `P` frames further ahead (#1355). For that it
needs two things from each carried source consumer, and this issue adds both:
- a **read-only readiness check**: in the block before it claims, render asks whether the
  consumer would play each of its next `k + 1` blocks from queued PCM, with no underrun, no
  discard and no command, where `P = k * quantum`;
- a **prime**: in the adoption block, the consumer replays its next `k` blocks exactly as it
  would have played them on the predecessor's own schedule, without observing any new command.

Each ring also gets a `prime_required` flag that the control thread sets for the rings a warm
successor carries. Nothing is copied, no block is shared, and no second reader exists.

## Context

- A source ring is two block queues of `Box<TransferBlock>` (`crates/source/src/lib.rs:450`),
  a data queue (producer to consumer) and a recycle queue, plus a one-slot command queue
  (`prepare_at_source_frame`, `:568`; the queues at `:578-588`). The ring allocates
  `transfer_block_count + 1` blocks (`RETAINED_TRANSFER_BLOCKS`, `:654`).
- The queues are the engine's SPSC (`bounded_spsc_move`, `crates/engine/src/realtime/spsc.rs:249`).
  Its consumer can count what is queued (`available_at_entry`, `:420`) and pop (`try_pop`, `:440`),
  but cannot read a queued item without popping it.
- `PcmSourceConsumer` (`crates/source/src/lib.rs:993`) keeps `next_frame`, an optional prefetched
  `current` block, the `played` block and a `held_seek` (`:1018`). `begin_block_at(first_sample)`
  (`:1115`) first observes the command queue (`begin_block_observe`, `:1127`;
  `observe_seek_at_block_boundary`, `:1323`), then acquires and plays (`begin_block_play`, `:1135`).
- `acquire_current_block` (`:1389`) returns at once while `current` is held (`:1390-1392`). It pops
  blocks, discards stale or behind-`next_frame` ones, and keeps a held seek's first block as the
  pending `current`. A popped block of an unobserved generation makes it observe the command queue
  (`:1400-1407`).
- When no block matches `next_frame`, `begin_block_play` underruns. It still advances `next_frame`
  by a quantum (the underrun branch `:1154-1178`; the advance `:1164-1168`). An underrun inside a
  prime would therefore move the consumer's frame map, so the prime must be impossible to underrun.
- Host chunks are exactly one quantum, except a short end-of-region chunk
  (`validate_submission_metadata`, `:924`). A short block's tail is zeroed in `play` (`:1466`).
- The graph driver calls `consumer.begin_block_at(first_sample)` per source each block
  (`:1721`, in `begin_block`, `:1701`) and lends played planes (`played_planes`, `:1793`).
- `crates/source` has no `bench-support` dev-dependency today (`crates/source/Cargo.toml`). The
  engine SPSC loom tests are named `spsc_loom_*` and run in the CI step at
  `.github/workflows/qualification.yml:677-678`.

## Decisions frozen for this slice

- **D1. `peek(i)` on the SPSC consumer.** `Consumer::<T>::peek(&self, i: usize) -> Option<&T>`
  in `crates/engine/src/realtime/spsc.rs`, inside its realtime-policy region.
  - It loads the producer cursor once (`Acquire`), as `available_at_entry` does, and returns the
    `i`-th queued item from this consumer's cursor, or `None` when `i` is not below that count.
  - It changes no cursor, cache or counter.
  - It is sound because the producer writes only slots past the consumer's cursor. A slot the
    consumer has not popped is initialized, and nothing rewrites it until the consumer pops it.
  - The borrow is tied to `&self`, so no pop can happen while it is held.
- **D2. Readiness.** `PcmSourceConsumer::prime_ready(&self, first_sample: u64, lead_blocks: u32)
  -> bool`.
  - `first_sample` is the source-read sample of the block render is about to run on the active
    plan: `S + O`, where `O` is that plan's source-read offset (#1396).
  - With `k = lead_blocks` and `q` the quantum, it returns `true` exactly when all of these hold:
    1. The command queue is empty (`available_at_entry() == 0` on the command consumer).
    2. No held seek's anchor is below `first_sample + (k + 1) * q`. A held anchor is always past
       the last block's sample, so this is every held seek that would apply at one of the `k + 1`
       blocks. It covers the window `[S + O, S + O + P + q)`, and also an anchor in
       `(S + O - q, S + O)`, which applies late at `S + O`.
    3. For each `j` in `0..=k`, the block the consumer would play at `next_frame + j * q` is
       queued and playable:
       - either the region's end is reached there (`end_frame`, or a peeked end-of-region block's
         end, is at or before that frame);
       - or block `j` has the active generation and starts at exactly `next_frame + j * q`.

       Block `j` is taken in queue order: `current` first if it is held, then `peek(0)`,
       `peek(1)`, and so on. A short block must carry `end_of_region`.
  - Anything else is `false`. This includes a gap, a block of another generation, a block behind
    `next_frame` that the consumer would discard, a held seek's pending `current`, or fewer queued
    blocks than needed. Those cases clear as render keeps playing the predecessor, so `false`
    only delays adoption.
  - It reads only: atomics loaded with `Acquire` and the consumer's own fields. It changes no
    state. It costs at most `k + 2` loads per ring, with no allocation, lock or syscall.
  - It is exact within one render callback. Only render changes the consumer, and the producer can
    only add blocks or a command. An added block keeps `true` true. A command pushed after the
    check is not observed by the prime (D3) and is observed at the successor's first ordinary
    block, on the new clock.
- **D3. The prime.** `PcmSourceConsumer::prime_block_at(&mut self, sample: u64) ->
  SourceReadReport`.
  - It runs `begin_block_observe` without the command pop: `end_block`, `flush_deferred_recycle`
    and `generation_changed = false`. Then it runs `begin_block_play` with a clock that applies no
    seek (`SeekClock::Hold`).
  - So it plays, discards and recycles exactly as `begin_block_at(sample)` would. The difference
    is that it observes no command, so the command queue and `held_seek` are unchanged.
  - Precondition: `prime_ready(first_sample, k)` was `true` in this callback, and the calls are
    `prime_block_at(first_sample + j * q)` for `j = 0..k`, in order.
  - Under that precondition it cannot underrun: block `j` matches `next_frame` or the end is
    reached. `acquire_current_block` never pops past block `j`, so it never meets an unobserved
    generation and never re-observes a command (`:1400-1407`).
  - A debug assertion checks `underrun_frames == 0` and that no command was popped. The played
    planes are lent by `played_plane` until the next `&mut self` call, as for any block.
- **D4. `prime_required`.**
  - Ring preparation allocates one shared `AtomicBool` beside the queues, `false`, and counts it
    in the resource report.
  - `PcmSourceProducer::prime_flag(&self) -> PcmSourcePrimeFlag`, and the same on
    `HostChunkProvider`, hand the control thread a handle. A handle is an `Arc` clone, taken at
    preparation, never on render. It has `set(&self, bool)` (`Relaxed`).
  - `PcmSourceConsumer::prime_required(&self) -> bool` loads it (`Relaxed`).
  - The ordering comes from the plan mailbox. Control stores the flags before the `Release` that
    publishes the candidate, and render loads them after its `Acquire` of the cell. #1311 D2's
    generation check on the claim makes a read across a withdraw-and-republish harmless. #1355
    owns when control sets and clears them.
- **D5. Acked-batch question.** No queue changes and no command is consumed or dropped. Readiness
  reads only. The prime plays blocks the consumer would play anyway, in the same order, and leaves
  every command queued. An ack can never precede a drop.

## Deliverables

1. D1 in `crates/engine/src/realtime/spsc.rs`, with unit tests and one loom test named
   `spsc_loom_peek_*`.
2. D2-D4 in `crates/source/src/lib.rs`.
3. The resource report gains the `prime_required` row. Every test that asserts the source ring's
   report is updated in the same PR.
4. `bench-support` as a `crates/source` dev-dependency, for gate 6.

## Authorized paths

- `crates/engine/src/realtime/spsc.rs` (`peek` and its tests only)
- `crates/source/src/lib.rs`, `crates/source/Cargo.toml`, `crates/source/tests/prime.rs` (new),
  `Cargo.lock`
- The source resource-report assertions in `crates/source/src/lib.rs` tests,
  `crates/host-core/tests/` and `crates/capi/tests/`

## Non-goals

- No graph, plan-exchange or host-core change. Calling readiness before the claim, the prime in
  the source section, and setting the flags are #1355's.
- No change to seek semantics (stream B, #1316-#1319). No ring sizing (#1358).

## Objective gates

Gates 2-4 run against a ring fed blocks whose samples encode `(frame, channel)`, with quantum 128
and `k = 3`.

1. **`peek`.** `peek(i)` returns the `i`-th queued item for every `i` below `available_at_entry()`
   and `None` at it. Cursors, success counts and empty counts are unchanged by any number of
   peeks. Run it across a wrap of the cursor space. The loom test (`spsc_loom_peek_*`): a producer
   pushes 3 items through a 2-slot queue while the consumer peeks and pops. Every peeked value is
   the one later popped. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom`
   (the existing CI step, `qualification.yml:678`).
2. **Readiness at exact boundaries.** Each case is checked at the boundary and one step past it:
   - `k + 1` contiguous active blocks queued: `true`. Only `k`: `false`.
   - The region ends inside the window: a short end block at `j = 1` and nothing after it gives
     `true`. With the end block missing, `false`.
   - A gap of one quantum at `j = 2`: `false`.
   - A plain `Seek` or a `SeekAt` in the command queue: `false`. It is `true` again after one
     ordinary block observes it, once the new generation's `k + 1` blocks are queued.
   - A held seek anchored at `first_sample + (k + 1) * q - 1`: `false`. Anchored at
     `first_sample + (k + 1) * q`: `true`. Anchored at `first_sample - 1`, held because the last
     block began before it: `false`.
   - Stale-generation blocks ahead of the active ones: `false` until ordinary blocks have
     discarded them.
   - After every call, the consumer's telemetry, `next_frame`, the queue counts and the command
     queue equal a twin that made no call.
3. **The prime equals the blocks the consumer would have produced.** Two rings, fed identically.
   On one, `k` calls `prime_block_at(s + j * q)`; on its twin, `k` calls
   `begin_block_at(s + j * q)`. Every played plane, every `SourceReadReport` and the telemetry are
   bit-identical. So are the next 8 ordinary blocks on both. Also run it with the region ending at
   `j = 1`, and with a generation that a seek began 2 blocks earlier.
4. **The prime observes no command.** After a `true` readiness, push a `Seek` and then prime.
   The primed planes equal gate 3's. The command is still queued, and the next
   `begin_block_at` applies it.
5. **Flag.** A new ring reads `prime_required() == false`. After `prime_flag().set(true)` the
   consumer reads `true`, and after `set(false)`, `false`.
6. **Realtime.** `prime_ready` with `k = 16` and 16 `prime_block_at` calls make zero allocations
   and frees, measured with `bench_support::alloc`'s current-thread counters after warm-up.
7. Commands:
   - `cargo test --locked -p engine`, `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`, `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a `peek` that indexes from the cached producer cursor, or that writes the consumer
  cursor, returns a stale item or loses one. Red.
- Gate 2: a check that ignores `current`, the command queue or the held-seek window lets the
  prime underrun or skip a seek. Red at the matching boundary.
- Gate 3: a prime that uses its own frame arithmetic instead of `begin_block_play`'s accept rule
  differs by a plane or a report. Red.
- Gate 4: a prime that pops the command applies a seek inside the window, out of schedule. Red.
- Gate 5: a flag that is not shared between the handle and the consumer reads `false` on render.
  Red.
- Gate 6: a readiness walk that collects blocks into a buffer allocates. Red.

## Dependencies

- *Anchor every seek on the plan's source-read clock* (#1316): stream B's edits to
  `crates/source/src/lib.rs` land first.
- *Report held source blocks apart from underruns* (#1318).
- *Test held seeks across swaps and supersession, and add a seek to audit capi* (#1319).
