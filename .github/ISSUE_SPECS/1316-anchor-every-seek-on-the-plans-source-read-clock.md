# Anchor every seek on the plan's source-read clock

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every seek is anchored, and the host can see where it landed. A `seek_at` anchor is a sample of the
plan's **source-read clock** ("equal to the render clock in this version"). A plain seek during
playback means "the next block render begins". For both, render publishes a per-source seek report:
the generation in effect, the source-read sample of the first block it played in, and the source
frame that block started at, plus the source's cumulative underrun and held frames. A host that
scrubs one stem with a plain seek and then starts another with `seek_at` can compute the second
stem's frame exactly, instead of guessing one or two quanta (round-1 finding D1-b).

## Context

- `begin_block_at(first_sample)` (`crates/source/src/lib.rs:1108-1117`) applies a held anchored
  seek in the block whose first sample reaches the anchor, at `frame + lateness`
  (`observe_seek_at_block_boundary`, `:1323-1363`). The graph driver passes the render block's
  first sample (`:1719-1721`). D1-a: once #1396 makes the graph read sources ahead of render, an
  anchor on the render clock would drift by that read-ahead; anchoring on the source-read clock
  keeps a host that never learns the read-ahead aligned.
- A plain seek applies at the first block render begins after the command is popped. Nothing
  reports which: the producer flips its `active_generation` at the call
  (`crates/source/src/lib.rs:776-779`), and the C ABI's submit report returns that producer value
  (`crates/capi/src/ffi.rs:496-499`). There is no applied-seek event.
- The ring's resources are reported row by row in `SourceResourceReport`
  (`crates/source/src/lib.rs:208-235`, built in `resource_report` at `:490-547`).
- The C ABI reaches producers through `SourceControlSet` (`crates/host-core/src/source.rs`, `seek`
  `:200`, `seek_at` `:223`) of the newest committed session (`newest_providers`,
  `crates/capi/src/runtime/control.rs:1488`).
- The engine's single-writer seqlock with bounded reads is `crates/engine/src/realtime/observe.rs`;
  #1314 adds `crates/engine/src/realtime/watermark.rs` on the same pattern.
- The header defines the anchor on "the clock miso_engine_v1_render_f32_planar takes"
  (`crates/capi/include/miso_engine_v1.h:96-99`). The header rewrite is #1317's.

## Decisions frozen for this slice

- **D1. Clock name.** `begin_block_at`'s parameter is renamed `source_read_sample` and documented as
  "the plan's source-read clock: the absolute sample of source time this block reads; equal to the
  render clock in this version". `SeekAt::anchor_sample` and `HeldSeek::anchor_sample` are
  documented on the same clock. The graph driver passes its `first_sample` unchanged; *Give a plan
  a source-read clock that leads its render clock* (#1396) is the issue that makes the two clocks
  differ, and it must pass the source-read sample here.
- **D2. Per-ring seek report.** Each ring gets one shared record, allocated with the ring on the
  control thread and charged as a new `SourceResourceReport::seek_report_bytes` row (in overhead
  and total). Words: `generation`, `first_sample`, `source_frame`, `cumulative_underrun_frames`,
  `cumulative_held_frames`. Single writer: the consumer, on the render thread. Reads: the producer
  side, bounded retries, "busy" on give-up. Reuse #1314's record module: generalize it to a
  fixed-width word record (`PublishedWords<const N: usize>`) that both the plan watermark and this
  report instantiate. No `unsafe`.
- **D3. When render publishes.** At the end of `begin_block_play`, the consumer publishes when its
  `active_generation` differs from the published generation (the ring's first block, and every
  block a plain or anchored seek applies at) or when a cumulative counter changed. `first_sample`
  is this block's `source_read_sample`; `source_frame` is the frame the block started reading
  (`next_frame` before this block's advance; for a late anchored seek, `frame + lateness`).
  Otherwise nothing is written. Before the ring's first block the record reads generation 0,
  meaning "not yet rendered".
- **D4. Plain seeks are anchored at the next block.** No state-machine change: a plain seek keeps
  applying at the first block that observes it. D3 makes that block visible. A seek prepared
  between blocks by an exclusive host (`prepare_seek`, `:1049`) is reported at the next block,
  because that is the first block that plays it.
- **D5. Carry.** The record belongs to the ring, so it moves with the consumer across a plan swap
  (the carry swaps the whole consumer, `crates/source/src/lib.rs:1880`). A source that a transaction adds or changes gets a new ring
  and starts at generation 0.
- **D6. C ABI.** `uint32_t miso_engine_v1_source_seek_report(miso_engine_v1_session *session,
  const uint8_t *source_id, uint64_t source_id_bytes, miso_engine_v1_source_seek_report *out)`,
  thread: session (control), serialized with the other session calls. Struct (64 bytes,
  `MISO_ENGINE_V1_SOURCE_SEEK_REPORT_SIZE`): `uint32_t struct_size; uint32_t reserved0;
  uint64_t generation; uint64_t first_sample; uint64_t source_frame;
  uint64_t cumulative_underrun_frames; uint64_t cumulative_held_frames; uint64_t reserved[2];`.
  It reads the newest committed session's ring (like `seek`). Unknown source:
  `INVALID_ARGUMENT`, `source.id.unknown`. Wrong size or nonzero reserved: `INVALID_ARGUMENT`.
  Busy read: `BACKPRESSURE`, `source.report.busy`. A malformed source ID (null, empty, non-UTF-8,
  over 127 bytes) returns `INVALID_ARGUMENT` here as on `seek`; its `source.id.invalid` diagnostic
  is #1350's, which lands after this slice and covers this entry point. Feature bit
  `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_REPORT`: the next free bit when this merges, never a reused
  one; tests and the smoke program use the symbol, never a number. The frozen symbol list grows
  by one.
- **D7. Facade.** `SourceControlSet::seek_report(id) -> Result<SourceSeekReport, SourceControlError>`
  in host-core, reading the producer's reader; a vacated or vacant entry is `source.ring.vacated`
  as for `seek`. Its `seek_at` doc (`crates/host-core/src/source.rs:209-221`) says the source clock
  and "holds" instead of "reports an underrun" (#1318).
- **D8. Acked-batch question.** No queue is added; the report is a level written by render. Seek
  admission and its BACKPRESSURE are unchanged.

## Deliverables

1. D1-D5 in `crates/source/src/lib.rs`; D2's generalization in `crates/engine/src/realtime/`.
2. D7 in host-core; D6 entry point, struct, constants, bit; `abi_smoke.c`, `header_smoke.cpp`, the
   frozen list; a two-line header note of the new call (the full seek text is #1317's).

## Authorized paths

- `crates/source/src/lib.rs`, `crates/source/tests/*.rs`
- `crates/engine/src/realtime/watermark.rs`, `mod.rs`
- `crates/host-core/src/source.rs` (`seek_report` and the `seek_at` doc only)
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `runtime/control.rs` (or its successor file in the
  crate that #1309 creates), `runtime/error.rs`, `runtime/tests.rs`, `include/miso_engine_v1.h`,
  `tests/c/abi_smoke.c`, `tests/c/header_smoke.cpp`
- `scripts/check-capi-abi.sh` (frozen list only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The source-read offset itself (#1396). The browser export of the report (stream H, #1332 and the
  rewritten #1293). The seek header text (#1317).
- The `source.id.invalid` diagnostic for a malformed ID on this entry point: *Tighten the seek
  entry points: source.id.invalid, a typed held preparation, timed reads only* (#1350), which
  depends on this slice and gates it. The shared service step that this session call runs first:
  *Add miso_engine_v1_service for bounded control work between edits* (#1348).

## Objective gates

1. **Plain seek reported.** C ABI test: render blocks 0..5 of a playing source, plain-seek it to
   generation 2 frame 4096 (any in-region frame), submit generation 2 from it, render block 6:
   the report is `(2, 768, 4096, 0, 0)`.
2. **Late anchored seek reported.** `seek_at(id, 2, F = 384, A = 896)` after block 9 has rendered
   (A already past), render block 10: the report is `(2, 1280, 768, ..)`, matching the
   `F + (1280 - 896)` start that the PCM bit-identity check of the existing #1275 tests asserts.
3. **Scrub, then align.** One test: plain-seek stem X during playback, read its report
   `(g, s, f)`; add stem Y by transaction and `seek_at(Y, 2, f + (A - s), A)`; the output from A on
   is bit-identical to a fresh session fed both stems from the aligned frames.
4. **Carried and restarted rings.** The report of a persisting source survives a plan swap
   unchanged; an added source reads generation 0 until its first block, then `(1, swap block, 0)`.
5. **Counters.** A held block raises `cumulative_held_frames`, a true underrun
   `cumulative_underrun_frames` (needs #1318).
6. **Render stays clean.** `./target/release/audit capi` keeps `"total_violations":0`; the
   `crates/source` resource test that pins row sums adds the new row.
7. Commands:
   - `cargo test --locked -p source` and `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if render publishes the producer's generation at the call, or the block after the
  one that applied the seek; nothing today reports where a plain seek landed.
- Gate 2: red if the report omits the lateness (reports F, not `F + 384`).
- Gate 3: red if the report's sample and frame are not on the same clock the anchor uses; it is
  the host workflow D1-b names.
- Gate 4: red if the record lives on the plan instead of the ring (lost on swap) or an added ring
  inherits a predecessor's record.
- Gate 5: red if the report reads the per-block flags instead of the cumulative counters.

## Dependencies

- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the record
  module this generalizes.
- *Report held source blocks apart from underruns* (#1318): the held counter this reports.
