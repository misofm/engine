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

## Attempt record

### Attempt 1 (implementer)

**Implementation.**

- `crates/source/src/lib.rs`: `SourceCommand::SeekAt { generation, frame, anchor_sample }` and
  `SourceSeekError::AnchorUnaligned` (D1; `try_seek` applies `Seek`'s generation rule, then refuses
  an anchor that is not a quantum multiple before anything is queued or switched).
  `PcmSourceConsumer::begin_block_at(first_sample)`; `begin_block()` keeps applying at once. The
  consumer holds an observed `SeekAt` in a fixed `Option<HeldSeek>` field; `acquire_current_block`
  keeps the held generation's first block as the pending `current` (no end-of-region note until the
  seek applies) and leaves later blocks queued; `current_matches_next_frame` compares the
  generation. Applying (`apply_seek`, shared with `Seek`) keeps a pending block of the new
  generation that does not start behind the new frame and discards any other through
  `note_end_and_discard`, so a late seek past the region end is the end of region. A newer command
  replaces a held one and discards the replaced seek's pending block. `prepare_seek` observes with
  no clock: it holds a `SeekAt` and returns `false` (D4). The graph driver passes each block's first
  sample (`begin_block_at`).
- `crates/host-core/src/source.rs`: `SourceControlSet::seek_at` (D5), sharing `seek`'s region and
  generation checks through one private `queue_seek`; diagnostic `source.seek.anchor_unaligned`.
- **Outside the authorized paths, forced by D1's new variant** (both match `SourceSeekError`
  exhaustively and stop compiling otherwise): `crates/host-core/tests/source_diagnostics.rs` gains
  the new variant's table row and index arm (the file's own documented procedure for a new
  rejection); `crates/source/tests/randomized.rs` gains an `unreachable!` arm (it only issues plain
  `Seek`s).

**Tests** (`crates/host-core/tests/successor_swap.rs` unless noted). The anchored-run harness lives
in that file (`anchored_swap_run`, a local `render_block`/`exchange`), since `support/successor.rs`
is outside this slice's paths; the reference is `reference_run` of B fresh with spliced feeds.

| Gate | Test | Mutation that turns it red (run, red, reverted, green) |
|---|---|---|
| 1 | `a_future_anchor_starts_the_added_stem_on_its_block_at_{eight,four}_lanes` | M1 held generation's blocks discarded (today's rule): red. M2 apply when observed: red. M8 driver calls untimed `begin_block()`: red. M10 facade sends `Seek`: red. |
| 2 | `a_past_anchor_starts_the_added_stem_in_time` (both widths) | M3 lateness not added (`late_by = 0`): red. M2, M8: red. |
| 3 | `two_playing_sources_move_to_one_block` (both widths) | M2, M8, M10: red. |
| 4 | `source` unit: `an_unaligned_anchor_is_refused_without_a_generation_switch` | M10 (facade test `seek_at_refuses_an_unaligned_anchor`): red. |
| 4 | `source` unit: `a_newer_anchored_seek_replaces_a_held_one` | M5 a held seek not replaced: red (also M1, M2). |
| 4 | `source` unit: `prepare_seek_holds_an_anchored_seek` | M4 `prepare_seek` drops the observed `SeekAt`: red (also M2). |
| 4 | `source` unit: `a_late_anchored_seek_past_the_region_end_is_the_end_of_region` | M6 apply discards the pending block without noting its end: red (also M1, M2, M3). |
| D3 | `source` unit: `a_primed_block_waits_for_its_anchor` | M7 `current_matches_next_frame` without the generation compare: red (plays the primed block a block early). Also M1, M2. |
| 5 | `holding_and_applying_an_anchored_seek_allocates_nothing` (early and late anchor; `bench_support::alloc` thread counters and the engine render audit, every block after the warm-up) | M9 a `Box` allocated and freed when a `SeekAt` is held: red. |
| D5 | `seek_at_refuses_an_unaligned_anchor` | M10: red. |

Every mutation was applied alone to the committed code, the named tests ran red, the file was
restored byte-identical (`cmp`) and the suite ran green.

**Gates run** (worktree `codex/seamless-swap`, base `1338b063c`):

- `cargo test --locked -p source -p host-core --features host-core/test-support,graph/test-support`:
  254 passed, 0 failed. `cargo test --locked -p capi -p host-web`: 233 passed, 0 failed.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: clean.
- `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-realtime-policy.sh`,
  `test-realtime-policy.sh`, `check-capi-abi.sh`: exit 0.
- `audit capi` (release): 0 allocations, 0 deallocations, 0 syscalls, 0 violations.
- `check-cross-targets.sh`: PASS (the four known #1018 `memset_pattern16` expected failures only).
- Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`
  (`sourceTotalBytes` 3358 of 3648, was 3294: the consumer's held-seek field;
  `graphSessionPlusPlanBytes` 29794 of 35648, unchanged), `test-web-audioworklet.sh`: all pass.
- **ARTIFACT CHANGED**: the shipped module is
  `751122a9ecdd05e96c8d5055bbefbb56ca4c75948d9692ade32ed01fa3959743` (2870096 B; #1272's, which
  `1338b063c` still builds, was `feeb20c3...1d47`, 2869086 B). Per `docs/RELEASE.md` ("Between
  releases") the pin is not re-pinned.
