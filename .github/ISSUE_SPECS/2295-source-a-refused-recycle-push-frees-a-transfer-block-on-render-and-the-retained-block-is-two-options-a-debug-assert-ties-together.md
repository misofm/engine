# Source: a refused recycle push frees a transfer block on render, and the retained block is two Options a debug_assert ties together

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item WEB-10).

## Mission
Make the source consumer's block custody explicit. The one block it retains outside both queues
becomes a single always-present slot with a phase. A recycle push the queue refuses must never free
a block on the render thread. The refusal is proven unreachable and, if the proof is ever broken,
is counted, not hidden.

## Verified defect (base `875a4c17e`)
`crates/source/src/lib.rs`, `PcmSourceConsumer`:
- Custody is spread over four fields: `played: Option<Box<TransferBlock>>`, `played_frames: u32`,
  `retained_idle: Option<Box<TransferBlock>>` (1729, documented "`Some` exactly when `played` is
  `None`") and `deferred_recycle: Option<Box<TransferBlock>>` (1730). The pairing rule is held only
  by two `debug_assert!`s: `end_block` (2457, "a played block and an idle retained block never
  coexist") and `play` (2913, "begin_block ended the previous block"). In a release build, a
  `play` that ran without the preceding `end_block` would overwrite `played` and free the old block
  on render. `played_frames` is a hand-kept copy of `played.frames`.
- `recycle_block` (2921-2927) parks a refused push in `deferred_recycle = Some(value)`. A second
  refusal before `flush_deferred_recycle` (2930, called at five entry points) overwrites the slot,
  and the block already parked there is freed on the render thread.

**Reachability: not reachable with any configuration the public API allows.** The proof:
- Both block queues are built with capacity `allocated_block_count` (765, 768). That count is
  `transfer_block_count + RETAINED_TRANSFER_BLOCKS` (912), every block the ring allocates, for
  every config `PreparedShape::validate` accepts. No API resizes a queue or adds a block afterwards.
- Render is the recycle queue's only pusher. While it pushes, it holds the block it pushes and,
  beside it, the retained block. The capacity is `allocated`, and at most `allocated - 2` other
  blocks exist.
- Render's view of the producer's pops can be stale (`Producer::is_full_at`, `spsc.rs` 472, reloads
  the pop cursor with `Acquire` only when the cached view says full). That view never lags behind a
  pop of a block render holds. Such a block was either never pushed (the ring's first retained
  block), or it reached render through the data queue after its last recycle push was popped. The
  producer pops recycle, then pushes data with `Release`, and render pops data with `Acquire`, so
  that pop is ordered before render's reload. So each push render sees as still queued is of a
  block render does not hold. A block pushed twice came back to render in between, so its first
  push is seen popped. Render therefore sees at most `allocated - 2` queued pushes. The push always
  finds a slot, with two to spare, so `deferred_recycle` is never `Some`.
- There is slack beyond the sizing. Prepare's initial fill pushes `transfer_block_count =
  allocated - 1` blocks into the recycle queue and fails with `InternalQueueCapacity` on a
  refusal. Any capacity that could refuse a render push is at most `allocated - 2`, so prepare
  would already have failed for it.
- The pairing rule holds today: `play` has one call site (`begin_block_play`). Every entry that
  reaches it (`begin_block_observe`, `prime_block_at`, the set driver's per-consumer observe) calls
  `end_block` first.

**Severity.** L: structural hardening. No reachable failure today. The illegal combinations are
representable, though, and the only guard against a free on render is a sizing fact stated
nowhere near the code that relies on it.

## Decision / model
- **`RetainedBlock { block: Box<TransferBlock>, phase: RetainedPhase }`**, with
  `enum RetainedPhase { Played, Idle }`. It replaces `played`, `played_frames` and `retained_idle`.
  The block is always present and there is only one, so "no retained block" and "played and idle at
  once" cannot be represented. `played()` lends it only in `Played`. `release()` resets its
  metadata and makes it idle (it does nothing when already idle). `play(block)` returns the
  storage it displaces, reset, whatever the phase was. Even a `play` without `end_block` hands the
  old block to the recycle queue instead of freeing it. `copy_channel` reads `block.frames`
  directly, so the `played_frames` copy is gone.
- **Why not `enum Held { Played(Box), Idle(Box) }`**, as the tracker suggested. A variant change
  would have to move the `Box` out of `&mut self`, which needs a placeholder block (an allocation)
  or `unsafe`. A phase beside an always-present block rules out the same illegal states.
- **No deferral slot.** The proof shows a slot that could never be filled, so it and its five
  `flush_deferred_recycle` calls are deleted. `recycle_block` now documents the proof. If the push
  is ever refused (the capacity rule broke), it leaks the block with `core::mem::forget` (never a
  free on render, never parked where a later refusal could overwrite it) and counts it in the new
  `SourceConsumerTelemetry::recycle_refusals`. Leaking is the realtime-safe end state for a broken
  invariant: freeing is forbidden on render, and a panic would trap the worklet.
- `RetainedBlock`'s methods and `recycle_block` are now inside `REALTIME_POLICY` regions.
- No ring handoff changes: `try_push`/`try_pop` and their orderings are untouched, so `spsc_loom`
  still covers the queues. `crates/source` has no loom model of its own, and none is needed.

## Non-goals
- Host plumbing of `recycle_refusals` (`crates/host-core`, `hosts/host-web`).
- Any change to queue capacities, the resource report, or any rendered bit.

## Gates
- A regression test that forces a refusal, red on base as a free on the render thread
  (`bench_support::alloc` thread-scoped counters), and green at head.
- No-bit-moved: a one-time digest of the consumer's emitted PCM, underrun/held counters and seek
  outcomes over seeded scenarios, identical on base and head (PR evidence, not committed).
- `cargo test --locked -p source`, clippy `-D warnings`, `cargo fmt --check`, rustdoc
  `-D warnings`, `scripts/check-workspace-policy.sh`, the realtime policy tool.

## Evidence (attempt 1)
**Red/green.** New test:
`tests::a_refused_recycle_push_frees_nothing_on_render_and_is_counted`. It submits two
generation-1 blocks and seeks to generation 2. It then swaps in a one-slot recycle queue that is
already full, the only way to break the capacity rule, and runs one `begin_block_at`, which
discards both stale blocks. The thread-scoped allocator counters must show no allocation and no
free, and `recycle_refusals` must be 2.
- On base `875a4c17e` (without the `recycle_refusals` assert, since the field does not exist
  there), it fails because the second refusal overwrote the parked block:
  ```
  assertion `left == right` failed: render allocates and frees nothing
    left: (0, 2)
   right: (0, 0)
  ```
- At head it passes.

**Test value.** The test turns red if a recycle push that the queue refuses frees a transfer block
on the render thread, or goes uncounted. No existing test reaches a refusal: the capacity rule
makes one impossible without breaking it by hand.

**Mutations (each reverted).**
- Drop the refused block instead of `forget`: red, `left: (0, 4)` (two blocks, each a box and its
  samples).
- Remove the `recycle_refusals` increment: red, `left: 0, right: 2`.
- `RetainedBlock::played()` lends the idle block: six existing tests go red (`played_plane_is_the_quantum_with_a_zeroed_short_tail_and_none_without_a_block`, `retained_played_block_supports_repeat_fanout_and_auto_recycles_once`, `graph_driver_played_planes_map_claims_until_the_next_begin_or_seek`, `a_played_block_is_silent_iff_its_producer_copied_no_nonzero_word`, and two `seek_landing` tests).

**No bit moved.** A scratch integration test was added for this run only, never committed, and
deleted. It ran 4,096 seeds: random rings with quantum 1-16, capacity 1-8 quanta, 1-3 channels and
seek runway 0-2. Each seed ran 160 random steps of valid, gapped, stale and short or terminal
submits of hostile words, plain and anchored seeks, `prepare_seek`, `prime_ready` and
`prime_block_at`, and blocks read through `copy_channel`, `played_plane` and `played_block_silent`,
with and without `end_block`. It folded every word, every read report, `unread_frames`, the
consumer telemetry (its pre-existing fields), the producer's seek report, admission and telemetry,
and every submit and seek verdict into one FNV-1a hash. The run covered 246,226 blocks, with up to
65 underrun events, 17 held blocks and 48 stale discards in one ring, 7,699 primed blocks and 8,281
ready `prepare_seek`s. The digest was `fbb419b8d21088fa` on base `875a4c17e` and at head.

**Gates (head).**
- `cargo test --locked -p source`: all pass (77 lib, 6 `prime`, 1 `randomized`).
- `cargo test --locked -p host-core --features test-support` (host-core is unchanged, but it is
  the production caller): all 58 test binaries pass.
- `cargo clippy --locked -p source --all-targets -- -D warnings`: clean (only the pre-existing
  `clippy.toml` `fast_db` path warnings, which are not lints).
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p source`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (294 marked regions in
  46 files)`.

**Plumbing.** No field of `SourceConsumerTelemetry` reaches a host today, so `recycle_refusals`
opens no wiring gap of its own. It is read where its siblings are, and plumbing that record is not
this issue's scope.

## Verdict (attempt 1): PASS
The adversarial review checked the following:
- **The proof holds, with slack.** Render holds the retained block beside the one it pushes, so
  the tight bound is `allocated - 2` queued, two below capacity. Prepare's initial fill of
  `allocated - 1` blocks already refuses any capacity that could refuse a render push. The
  `recycle_block` doc now states both points.
- **Leak-and-count is the right model** for a broken invariant: no free on render, no panic, and
  no slot a later refusal could overwrite. A leak takes the block out of circulation, so refusals
  are bounded by the excess of blocks over the broken capacity.
- **Deleting `flush_deferred_recycle` lost nothing.** The reviewer's mutation that makes `play`
  skip the recycle push was caught, so recycling is still defended without the flushes.
- **No bit moved.** The reviewer reproduced the digest `fbb419b8d21088fa` on both trees.

Folded after the verdict:
- the tight bound and the prepare-fill slack in the `recycle_block` doc and in the proof above;
- this plumbing note in place of a follow-up claim.
