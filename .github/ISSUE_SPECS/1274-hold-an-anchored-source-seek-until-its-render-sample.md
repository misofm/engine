# Hold an anchored source seek until its render sample

Slice 5 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A host can say "source frame `F` enters the graph in the block that starts at absolute render sample
`A`", prime the ring from `F` ahead of time, and have the stem start exactly there. If the render side
sees the seek only after that block has passed, the stem starts at `F + (block start - A)`, so it is
aligned either way. This is what lets a stem added by a structural edit join the playing stems in
time; it also lets a host move several playing sources to one exact block. This slice adds it to the
source crate and the host-core facade; *Start a newly added C ABI source at an exact render sample*
(slice 6) exports it.

## Context

- A seek is `SourceCommand::Seek { generation, frame }` (`crates/source/src/lib.rs:51-58`). The
  producer switches its own generation at once (`PcmSourceProducer::try_seek`, `:725-751`, sets
  `active_generation` at `:739-744`), so the host may submit the new generation immediately. The
  consumer applies the command at the start of the next block it renders
  (`observe_seek_at_block_boundary`, `:1219`, from `begin_block`, `:1028`), which thread timing
  decides.
- The consumer discards every queued block whose generation is not its active one, or whose start
  is behind `next_frame` (`acquire_current_block`, `:1233-1256`). An underrun advances `next_frame`
  by a quantum (`:1063-1083`).
- `prepare_seek` (`:985-1003`) pops the command through `observe_seek_at_block_boundary`.
- The graph driver receives each block's first sample and ignores it
  (`SourceGraphSourceSetDriver::begin_block(&mut self, _first_sample, frames)`, `:1494`).
- Facade: `SourceControlSet::seek` (`crates/host-core/src/source.rs:188`).
- Successor preparation and the test harness come from *Prepare a successor plan whose unchanged
  sources keep playing* (#1272).

## Decisions frozen for this slice

- **D1. Command.** `SourceCommand::SeekAt { generation, frame, anchor_sample }`. `try_seek` accepts
  it under the generation rule of `Seek` (strictly newer, nonzero), sets `next_write_frame = frame`,
  and refuses an `anchor_sample` that is not a multiple of the quantum with
  `SourceSeekError::AnchorUnaligned`.
- **D2. Clock.** `anchor_sample` is the plan's absolute render sample: the `absolute_sample` a C ABI
  host renders and the browser's `next_absolute_sample`, which every swap continues. It names when
  `F` enters the graph; the output hears it `latency_samples` later.
- **D3. Consumer.** `PcmSourceConsumer::begin_block_at(first_sample)`; the graph driver calls it with
  the block's first sample. On observing a `SeekAt`:
  - if `first_sample >= anchor_sample`: apply it as a seek to `frame + (first_sample -
    anchor_sample)` (saturating; past the region end it is the end of region);
  - otherwise **hold** it. While it is held, the consumer plays any queued block of the old
    generation, and keeps (does not discard) the first block of the held generation as its pending
    current block, leaving later blocks in the queue. With nothing playable it underruns. At the first
    block whose `first_sample >= anchor_sample` it applies the seek. Applying it must not go through
    today's discard of `current` (`observe_seek_at_block_boundary`, `:1227-1229`): the primed block
    is kept when its start equals the new `next_frame`, and the behind-frame rule discards it
    otherwise. `current_matches_next_frame` (`:1257-1261`) must also compare the generation;
  - a newer command replaces a held one.

  `begin_block()` without a time keeps today's behaviour and applies a `SeekAt` at once. No
  allocation: the held seek and the pending block are fixed fields.
- **D4. Between-block preparation.** `prepare_seek` that observes a `SeekAt` holds it (as in D3) and
  returns `false`; it never consumes and drops it.
- **D5. Facade.** `SourceControlSet::seek_at(id, generation, frame, anchor_sample)`, with `seek`'s
  region checks, and diagnostic `"source.seek.anchor_unaligned"` for D1's refusal.

## Deliverables

1. D1-D4 in `crates/source/src/lib.rs`; the driver passes the first sample.
2. D5 in `crates/host-core/src/source.rs`.
3. Tests (below).

## Authorized paths

- `crates/source/src/lib.rs`
- `crates/host-core/src/source.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No C ABI or browser export (slice 6, B4).
- No transport or playhead concept in the engine.

## Objective gates

1. **Gap-free acceptance, future anchor, primed.** In `crates/host-core/tests/successor_swap.rs`:
   session A has one track on source `s1` (no stateful DSP). Render 6 blocks. Prepare successor B
   that adds source `s2`, a track on it and its route; swap. Then `seek_at(s2, 2, A, A)` with `A`
   three blocks past the next render, and submit `s2` from frame `A` at generation 2 **before** block
   `A` renders. Reference: B fresh, `s1` fed from frame 0, `s2` fed zeros for frames `< A` and the
   same PCM from `A`. Every block bit-identical.
2. **Past anchor.** The same with `A` two blocks before the swap block; submit from `A`. Output equals
   the reference whose `s2` is zero before the swap block and the same PCM from the swap block's
   frame on.
3. **Two playing sources moved to one block.** Two sources given `seek_at` to one anchor at different
   moments between renders start in the same block.
4. **Unit rules** in `source`: unaligned anchor refused; a held seek replaced by a newer one;
   `prepare_seek` holds a `SeekAt`; a late offset past the region end gives `end_of_region`.
5. **Realtime.** Blocks that hold, then apply, an anchored seek make zero allocations and frees
   (`bench_support::alloc` thread counters and the engine render audit in the host-core test).
6. Commands:
   - `cargo test --locked -p source -p host-core --features host-core/test-support,graph/test-support`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a held seek that discards the primed generation (today's discard rule) leaves the stem
  silent at `A`, and a seek applied when observed starts it early; both turn it red.
- Gate 2: an offset computed from the wrong clock plays the stem late; it turns red.
- Gate 4: a `prepare_seek` that pops and drops a `SeekAt` loses an accepted seek; it turns red.

## Dependencies

- *Prepare a successor plan whose unchanged sources keep playing* (#1272).
