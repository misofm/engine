# Keep source transfer blocks in a shared pool, immutable from publication to release

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 3).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A source ring's PCM blocks can be read safely by a second, read-only party while the render
consumer plays them. Nothing a host or a listener observes changes: same PCM, same admission depth,
same counters, same backpressure. This is the storage that *Give the source ring a read-only peek
cursor that gates release* (#1320) reads through.

## Context

- Today each ring allocates `transfer_block_count + 1` `Box<TransferBlock>`s
  (`crates/source/src/lib.rs:450`, `:568-640`, `RETAINED_TRANSFER_BLOCKS` at `:654`). It moves them
  through two `bounded_spsc_move` queues, data and recycle. A `Box` is exclusive, so no third party
  may read a block, and Rust's aliasing rules forbid it even through a raw pointer.
- Render writes into a block once: `play` zeroes a short block's tail in place (`:1466-1474`).
  `reset_metadata` writes the metadata when a block is ended or discarded (`:476`, `:1244`, `:1460`).
- The producer fills only a block it popped from the recycle queue (`take_recycled_block`, `:800`;
  `submit`, `:819`; `publish_block`, `:838`).
- `bounded_spsc` carries `Copy` values (`crates/engine/src/realtime/spsc.rs:236`).

## Decisions frozen for this slice

- **D1. Pool.** The ring allocates one `Arc<BlockPool>` at preparation:
  - a boxed slice of `allocated_block_count` slots, each with block metadata and its PCM;
  - interior mutability through `UnsafeCell`, under the ownership rule in D3.

  Both queues carry `BlockIndex(u32)` through `bounded_spsc`. `allocated_block_count` above
  `u32::MAX` is refused at preparation with a typed `PcmSourceRingError`.
- **D2. Immutable from publication.**
  - The producer zeroes a short block's tail before publication (the code moves from `play` to
    `submit`).
  - Metadata is written only by the producer before publication, and reset only after release.
  - The consumer never writes a published block.
  - `played_plane` and `copy_channel` keep their shapes and results.
- **D3. Ownership rule, written on the type.** A slot is writable only by whoever popped its index
  from the recycle queue (the producer), until it is published. From publication to release it is
  read-only for everyone. Release is the consumer's push of the index to the recycle queue. Every
  `unsafe` block cites this rule, and nothing else creates a reference to a slot.
- **D4. Accounting.** `SourceResourceReport` replaces the boxed-block rows with the pool row
  (header plus slots) and the two index queues. Every test that asserts the report is updated in
  the same PR.
- **D5. No behaviour change.**
  - Admission depth, the retained block (#917), held seeks (#1274), discards, counters and the
    order of every queue operation stay as they are.
  - The randomized ring schedules (`crates/source/tests/randomized.rs`) and the seek schedule
    model (`crates/source/tests/seek_schedule_model.rs`) pass unchanged.
- **D6. Acked-batch question.** No queue changes capacity or meaning. A producer's ack still
  follows its data-queue push, so an ack can never precede a drop.

## Deliverables

1. D1-D5 in `crates/source/src/lib.rs`.
2. No new test. The existing tests pass unchanged except for the report rows (D4).

## Authorized paths

- `crates/source/src/lib.rs`, `crates/source/tests/` (report assertions only)
- The source resource-report assertions in `crates/host-core/tests/` and `crates/capi/tests/`

## Non-goals

- No peek, no gate and no publication log (#1320).
- No change to the producer or consumer public API, except the report rows.

## Objective gates

1. **Unchanged behaviour.** `cargo test --locked -p source` passes, including `randomized.rs` and
   `seek_schedule_model.rs`. Only the report-row assertions are edited.
2. **Short tail at publication.** The existing
   `played_plane_is_the_quantum_with_a_zeroed_short_tail_and_none_without_a_block` (`:3270`)
   rotates poisoned blocks through every slot before a short block, so it already guards the moved
   zeroing. It passes unchanged.
3. **Render stays allocation-free.** The existing render allocation gates in host-core and capi
   pass: `cargo test --locked -p host-core --features host-core/test-support` and
   `cargo test --locked -p capi`.
4. Commands:
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- No new test. This is a storage refactor with no behaviour change, so the existing source,
  host-core and capi suites are its regression net. The poisoned-tail test (gate 2) is the one that
  defends D2's moved write.

## Dependencies

- *Anchor every seek on the plan's source-read clock* (#1316), *Report held source blocks apart
  from underruns* (#1318), *Test held seeks across swaps and supersession, and add a seek to audit
  capi* (#1319) and *Tighten the seek entry points: source.id.invalid, a typed held preparation,
  timed reads only* (#1350). These are stream B's edits to `crates/source/src/lib.rs`; they land
  first.
