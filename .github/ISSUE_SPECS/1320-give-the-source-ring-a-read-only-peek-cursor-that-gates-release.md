# Give the source ring a read-only peek cursor that gates release

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 3, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

While a successor plan catches up off the render thread (*Pre-roll a successor whose latency
grows*, #1287; the catch-up is #1355), it reads exactly the PCM the render thread plays and will play, from the same
source rings, without a copy on render and without a second consumer. The producer never
overwrites a block the catch-up still needs. Render pays one atomic load per block while a peek is
armed, and nothing extra while none is.

## Context

- A source ring is two block queues, not a frame-indexed buffer. `PcmSourceRing::prepare_at_source_frame`
  (`crates/source/src/lib.rs:568`) allocates `transfer_block_count + 1` boxed `TransferBlock`s
  (`:450`, `RETAINED_TRANSFER_BLOCKS` at `:654`) and moves them through a data queue (producer to
  consumer) and a recycle queue (consumer to producer), both `bounded_spsc_move` of
  `Box<TransferBlock>`.
- The producer writes only into a block it popped from the recycle queue
  (`take_recycled_block`, `:800`; `publish_block`, `:838`). So the **release point** is the
  consumer's push to the recycle queue (`recycle_block`, `:1486`). Round 2's phrase "release index
  `min(render read, peek cursor)`" maps onto that push here.
- The consumer pops in `acquire_current_block` (`:1389`), plays in `play` (`:1466`), and zeroes a
  short block's tail **in place** there (`:1473`). Blocks are owned through `Box`, so no third
  party can read a block today, queued or played.
- Host chunks are exactly one quantum, except a short end-of-region chunk
  (`validate_submission_metadata`, `:924`). So block boundaries fall on quantum multiples from the
  generation's start frame.
- Anchored seeks are held in the consumer and applied at `begin_block_at(first_sample)` (`:1115`);
  the graph driver calls it per source each block (`:1721`).
- `crates/source` has no `loom` or `bench-support` dev-dependency today (`crates/source/Cargo.toml`).
  `crates/engine` models its SPSC under `--cfg loom` (`crates/engine/src/realtime/spsc.rs:14-50`).

## Decisions frozen for this slice

- **D1. Shared, immutable blocks.** This slice builds on the shared block pool of #1353: blocks
  live in a ring-owned pool, queues carry block indices, and a
  block is immutable from publication until its release. The short-tail zeroing moves to the
  producer.
- **D2. Publication sequence.** The producer stamps each published block with a monotone `u64`
  sequence number. Before the data-queue push it writes `(sequence, block index)` into a
  publication log with `allocated_block_count` entries, then stores `published_sequence` with
  `Release`. All of it is allocated at ring preparation.
- **D3. One peek endpoint per ring, and its lifecycle.**
  - `PcmSourceProducer::take_peek(&mut self) -> Option<PcmSourcePeek>` hands out the ring's single
    peek endpoint (an `Arc` clone of the shared state, a reference-count increment and no
    allocation). It sets `peek_outstanding` and returns `None` while one is outstanding.
  - A peek ends when it is dropped. `Drop for PcmSourcePeek` does what `abandon` does if the peek is
    still armed (D8), then clears `peek_outstanding` (`Release`). After that, `take_peek` succeeds
    again, so every later latency growth can take a new peek.
  - Where each peek ends (fixed by the catch-up slices, never left open): a catch-up that diverges,
    is superseded, falls back or is stopped drops its peeks on the control thread (#1355, #1357,
    #1358, #1359). At adoption, render moves each peek out of the successor's source entry into the
    retiring predecessor's entry, in exchange for the consumer (#1355). The predecessor is
    reclaimed off render, and its drop ends the peeks there.
  - `PcmSourcePeek` is `Send`, not `Sync`. It is used by whichever thread owns the plan that holds
    it: the control thread during a catch-up, and the render thread during a bounded pre-roll
    (#1358) and at adoption (#1355). Its operations are loads, stores and lent slices: no
    allocation, free, lock or syscall. So they meet render's rules on either thread. Two threads
    never use one peek at once.
- **D4. Arming, on render, at a block boundary.** `PcmSourceConsumer::arm_peek(&mut self) -> bool`
  records the sequence and source frame of the next block it will play, and its active
  generation. It then stores `Armed` (`Release`). If the state is `Abandoned`, it first completes
  the abandon exactly as `begin_block` would (D8), so a catch-up abandoned in one service call can
  be armed again at the very next block. It returns `false` and changes nothing if a peek is armed,
  or if the ring's hold cap (D6) is 0. Stores and bounded pushes only: no allocation, no lock.
- **D5. Reading.** `PcmSourcePeek::begin_block(&mut self) -> PeekRead` returns:
  - `Ready`: the next block of the armed generation, contiguous with the peek's next frame. Its
    planes are lent by `played_plane(channel)`, with the same shape as the consumer's.
  - `NotYet`: the producer has not yet published the next contiguous block. No frame is consumed.
  - `EndOfRegion`: past the region end the peek reads `+0.0`, as the consumer does.
  - `Diverged`: see D7.

  A block of the armed generation that starts behind the peek's next frame is skipped, as the
  consumer discards it. The log entry's stored sequence is checked before each read; a mismatch is
  `Diverged`. Finishing a block stores the peek's released sequence (`Release`).
- **D6. Gated release, on render.** While armed, a block the consumer would recycle (played,
  idle-retained or discarded) is recycled only if its sequence is at or below the peek's released
  sequence, loaded once per `begin_block` (`Acquire`). Every other block waits in a FIFO of block
  indices with `allocated_block_count` slots, preallocated at ring preparation. The same load drains
  the FIFO front. The FIFO cannot overflow, because no more blocks exist.
  - **Render-side cap.** `PcmSourceRingConfig` gains `peek_hold_cap_blocks: u32`, the most blocks
    the gate may hold. If gating one more block would exceed it, the consumer stores `Diverged`,
    recycles every gated block and disarms, all in that `begin_block`. So a lagging catch-up costs
    the predecessor at most the cap's blocks of admission, never its playback. Host-core sets the
    cap to the ring's blocks above `default_source_ring_frames` for the session's rate and quantum
    (`crates/host-core/src/prepare.rs:65`, the config built at `:1207`). A ring with no headroom
    has cap 0 and is never armed. *Fall back from a missed catch-up deadline: bounded render-thread
    pre-roll, then the transition* (#1358) D2 adds that headroom to the default ring.
- **D7. Divergence.** While armed, the consumer stores `Diverged` when it underruns, observes a
  seek command, discards a block, changes generation, or reaches the hold cap (D6). The peek then answers `Diverged`. The
  caller abandons that catch-up (#1355 D2 decides what follows); exactness is never assumed past a
  divergence.
- **D8. Disarming.**
  - `PcmSourceConsumer::disarm_peek(&mut self)` recycles every gated block and returns to unarmed.
  - `PcmSourcePeek::abandon(&mut self)` is for the control thread when a catch-up is abandoned or
    superseded. It stores a released sequence of `u64::MAX` and the state `Abandoned`. At its next
    `begin_block` the consumer recycles every gated block and disarms. The peek can be armed again
    afterwards.
  - `PcmSourceConsumer::advance_past_peek(&mut self, frames: u64) -> bool` is for exact adoption.
    `frames` is a multiple of the quantum and no more than the peek has read beyond the consumer.
    It releases the blocks of the next `frames` frames unplayed, moves `next_frame` on by
    `frames`, and disarms. Otherwise it returns `false` and changes nothing. It is bounded by
    `allocated_block_count` pops.
- **D9. Acked-batch question.** No command queue changes. A producer's ack still follows its
  data-queue push. Gating only holds blocks longer, so a producer meets the existing typed `Full`
  sooner. Nothing is dropped. `advance_past_peek` releases only frames the peek has already read.
  An ack can never precede a drop.
- **D10. Cost when unarmed.** The armed state is a render-owned `bool` mirror, so an unarmed
  consumer pays one branch and no atomic.

## Deliverables

1. D2-D10 in `crates/source/src/lib.rs`, or in a new `crates/source/src/peek.rs` module it declares.
2. The resource report gains the log, FIFO and shared-state rows. Every test that asserts the
   source ring's report is updated in the same PR.
3. A loom model of the gate protocol: the producer, the consumer and the peek, under
   `--cfg loom`, with the shared state on loom types the way `spsc.rs` does it. Add `loom = "=0.7.2"`
   as a `crates/source` dev-dependency and one step to the loom leg of
   `.github/workflows/qualification.yml`.
   - `RUSTFLAGS='--cfg loom'` reaches every crate in the build. `engine` swaps its SPSC onto
     `loom::` types under `cfg(loom)` (`crates/engine/src/realtime/spsc.rs:36-40`), but `loom` is
     only its dev-dependency (`crates/engine/Cargo.toml:16-18`), so `engine` built as a dependency
     of `source` fails with E0433. Add `[target.'cfg(loom)'.dependencies] loom = "=0.7.2"` to
     `crates/engine/Cargo.toml`, beside the dev-dependency, with its comment. It is inert without
     `--cfg loom`, so production, Wasm and render never link it. The existing engine leg is
     unchanged.
4. `bench-support` as a `crates/source` dev-dependency, for gate 7.

## Authorized paths

- `crates/source/src/lib.rs`, `crates/source/src/peek.rs` (new), `crates/source/Cargo.toml`,
  `crates/source/tests/peek.rs` (new), `Cargo.lock`
- `crates/engine/Cargo.toml` (the `cfg(loom)` target dependency only)
- `crates/host-core/src/prepare.rs` (the hold cap in the ring config only)
- The source resource-report assertions in `crates/source/src/lib.rs` tests,
  `crates/host-core/tests/` and `crates/capi/tests/`
- `.github/workflows/qualification.yml`: the loom step only

## Non-goals

- No catch-up, plan or graph change. *Snapshot a running plan into a returned successor at a
  block* (#1354) arms the peeks, and *Catch up a returned successor and adopt it exactly at a
  scheduled sample* (#1355) binds them into a successor's source set.
- No change to seek semantics (stream B, #1316-#1319).
- No second peek per ring at a time. No catch-up render on the render thread here: the pre-roll
  that reads peeks there is #1358's, under D3's thread rule.

## Objective gates

1. **The peek reads what render plays.** Feed blocks whose samples encode `(frame, channel)`.
   Arm at block B. Run in both orders: the peek reads 6 blocks before render plays them, and render
   plays 4 blocks before the peek reads them. In both, every frame's planes from the peek are
   bit-identical to `played_plane` for the same frame.
2. **The gate holds released storage.** Hold cap 8. Arm. Render plays 4 blocks while the peek reads none. The
   producer then refills every block it can get with different content. The peek's 4 reads still
   equal the original content. The producer's admissions drop by exactly the held blocks. After
   the peek reads them, admissions return to the configured depth.
3. **Divergence and abandon.** While armed, each of these makes the next peek read `Diverged`: a
   plain seek, an anchored seek observed, an underrun, a stale discard. Afterwards `disarm_peek`
   restores the producer's configured admission depth. `abandon` from the peek side does the same
   one block later, and a second `arm_peek` then succeeds.
4. **Adoption advance.** After the peek has read `P = 3` quanta beyond render,
   `advance_past_peek(3 * quantum)` makes the next played block start at the old next frame plus
   `3 * quantum` and releases the 3 blocks. Asking for 4 quanta returns `false` and changes nothing.
5. **Lagging peek is capped.** Cap 3. Arm. The peek reads nothing while render plays 4 blocks and
   the producer keeps every free block filled. At the fourth, the peek reads `Diverged`, the
   consumer is unarmed, and render never underruns. The producer's admission depth is back to
   configured at the next submit. With cap 0, `arm_peek` returns `false`.
6. **Lifecycle.** `take_peek` returns `None` while a peek is outstanding. Dropping the peek, armed
   or not, makes the next `take_peek` succeed; dropping it armed releases its gated blocks at the
   consumer's next `begin_block`. An `arm_peek` right after `abandon` succeeds in the same block.
7. **Realtime.** `arm_peek`, a gated `begin_block_at`, `disarm_peek` and `advance_past_peek` make
   zero allocations and frees on the render thread. Measure with `bench_support::alloc`'s
   current-thread counters after warm-up.
8. **Loom.** `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p source --lib peek_loom`
   passes. Its invariant is that no block is published again while the peek can still read it.
9. Commands:
   - `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: if the peek starts one block off from the consumer's arm point, or uses its own frame
  arithmetic instead of the consumer's accept rule, a plane differs and the gate turns red.
- Gate 2: if a recycle ignores the gate (for example the idle-retained block recycled in `play`),
  the producer overwrites a block and the peek reads the new content. Red.
- Gate 3: a path that changes the consumer's frame map without storing `Diverged` would let a
  catch-up read PCM that render never played. Red.
- Gate 4: an advance that drops one block too many or too few makes the successor read a frame
  twice or skip one. Red.
- Gate 5: a gate with no render-side cap holds every block for a lagging peek, and render
  underruns. Red.
- Gate 6: a peek whose drop never clears `peek_outstanding` makes the second latency growth
  impossible; an `arm_peek` that refuses on `Abandoned` loses one republication. Red.
- Gate 7: a FIFO grown on demand, or an `Arc` clone on arm, counts an allocation. Red.
- Gate 8: a `Relaxed` publication or release store lets the model read a reused block. Red.

## Dependencies

- *Pre-roll a successor whose latency grows* (#1287), its first slice: the recorded proof fixes the
  frame the peek starts at and the lead it must reach.
- *Keep source transfer blocks in a shared pool, immutable from publication to release* (#1353).
- *Anchor every seek on the plan's source-read clock* (#1316), *Report held source blocks apart
  from underruns* (#1318) and *Test held seeks across swaps and supersession, and add a seek to
  audit capi* (#1319): stream B's edits to `crates/source/src/lib.rs` land first.
