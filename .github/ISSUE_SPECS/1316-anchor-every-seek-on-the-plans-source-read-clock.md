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
  `crates/control-plane/src/control.rs:1628`).
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
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `runtime/error.rs`, `runtime/tests.rs`,
  `include/miso_engine_v1.h`, `tests/c/abi_smoke.c`, `tests/c/header_smoke.cpp`
- `crates/control-plane/src/control.rs` (the successor #1309 made of
  `crates/capi/src/runtime/control.rs`)
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
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-cross-targets.sh`
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

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.** Each fact in Context still holds; only the lines moved.

- `begin_block_at` is `crates/source/src/lib.rs:1115` (doc `:1106-1114`). It and
  `begin_block_play` (`:1135`) now take a private `SeekClock` (`:1030-1038`, the enum at `:1031`). D1's rename is
  `SeekClock::At`'s field and `begin_block_at`'s parameter.
- `observe_seek_at_block_boundary` is `:1323-1365`. The acquire step can re-observe a command
  inside `begin_block_play` (`acquire_current_block`, `:1394-1440`, #1274 MINOR-3). D3's
  publication at the end of `begin_block_play` sees both.
- The graph driver's block entry is `:1752-1785`; the call is `:1772`.
- The producer flips `active_generation` in `try_seek` at `:778` (`try_seek` `:753-787`). The
  C ABI submit report copies it at `crates/capi/src/ffi.rs:501`.
- `SourceResourceReport` is `:209-236`; `resource_report` is `:490-546`. The row-sum test of
  gate 6 is `report_separates_session_pcm_from_source_overhead` (`:2056`).
- `prepare_seek` is still `:1049`. The carry's `mem::swap` is `:1931` (`adopt_sources`,
  `:1902-1938`).
- `SourceControlSet::seek` is `crates/host-core/src/source.rs:200`, `seek_at` `:223`, its doc
  `:209-222`.
- `newest_providers` is `crates/control-plane/src/control.rs:2557`, `newest_providers_mut`
  `:2578`. `SessionState::seek` is `:3003`, `seek_at` `:3028`.
- The header's anchor text is `crates/capi/include/miso_engine_v1.h:204-216` (the clock clause
  `:205-207`). The session-thread table is `:18-22`.

**Dependencies on GitHub.**

- #1314 is closed. `crates/engine/src/realtime/watermark.rs` is on this tree.
- #1309 is closed. `crates/control-plane/src/control.rs` is correct in Authorized paths.
- #1318 is open. It is in the batch-15/16 list.
- #1348 is closed. Its service step exists (`SessionState::service`, `control.rs:1486`).
- Downstream, not dependencies: #1350, #1293, #1332, #1387, #1537, #1640 and #1641 (open). #1641
  plans a seek-report row for the timeline, which has no ring. D2's record shape must not stop a
  later non-ring owner from publishing the same words.

**Decisions.**

- D1, D3, D4, D5, D7 and D8 hold against the code.
- D6 and #1348. The header (`:148-153`) says every session call that passes its argument checks
  runs the service step first. Now that #1348 has landed, the new call follows that rule:
  `SessionState::seek_report` calls `self.service()` first, as `seek` does, and the header's
  thread table and service paragraph list the new call. This applies #1348 D2. It does not change
  D6's results.
- D6 needs a re-export in `crates/capi/src/runtime/mod.rs` and `crates/control-plane/src/lib.rs`.
  Add both to Authorized paths for re-exports only.
- D6's feature bit: the mask is 255 today (`MISO_ENGINE_V1_FEATURE_SERVICE` is 128). The rule
  "next free bit when it merges, by symbol" stands.
- **B15-R1, root decision under owner delegation (2026-10-15): one shared seqlock module, `crates/engine/src/realtime/seqlock.rs`.
  #1316 lands first and creates it. #1482 D3 later moves `observe.rs` onto it.** (D2 against
  #1482 D3.) D2 puts the generic part in `seqlock.rs`, not in `watermark.rs`, and `watermark.rs`
  uses it. Add `crates/engine/src/realtime/seqlock.rs` (new) to Authorized paths. The finding
  follows. D2 makes #1314's record a generic `PublishedWords<const N: usize>` in `watermark.rs`. *Attribute each claim's revisions exactly in
  the watermark and share one seqlock* (#1482, open, not in the batch-15/16 list) D3 puts the
  seqlock protocol in a new `crates/engine/src/realtime/seqlock.rs` that `observe.rs` and
  `watermark.rs` both use. Done in either order without a rule, the two slices build two shared
  abstractions or a third copy of the seqlock. Proposed resolution: one shared seqlock module.
  If #1482 lands first, D2 builds the seek report on `seqlock.rs` and leaves `watermark.rs`
  untouched. If this slice lands first, D2 puts the generic part in `seqlock.rs` (added to
  Authorized paths) and #1482 D3 then moves `observe.rs` onto it. Root picks the order.
- **B15-R2, root decision under owner delegation (2026-10-15): keep D6 as a direct C export, `miso_engine_v1_source_seek_report`.**
  (D6 against decision 17 C13.) The logic is one core function,
  `SourceControlSet::seek_report`, behind both hosts. Source submission, seeks and the seek report
  are transport under decision 17 C1, so their direct exports are not C13 exceptions. Decision 17's
  C13 line records this (`docs/rulings/one-control-abstraction-2026-10-13.md`). The finding
  follows. C13 says a new feature gets "its C ABI
  form (a protocol message through `miso_engine_v1_submit_command`, no new C export)". D6 adds
  a new C export, `miso_engine_v1_source_seek_report`. Root's decision-17 amendments to #1293 kept
  that slice's browser seek-report export (#1293 D6), so root seems to treat source seeks and
  their reports as host transport (C1: "frame staging"), like `seek` and `seek_at`. Proposed
  resolution: keep D6 as a direct export. Record that source submission, seeks and the seek
  report are transport under C1, with one core function (`SourceControlSet::seek_report`) behind
  both hosts. The alternative is a protocol message, which would need a new message ID and a
  source-ID field in the protocol.

**Gates and commands.** Every named script, package and feature exists. `crates/source/tests/`
holds `randomized.rs` and `seek_schedule_model.rs`. Gate 6's "`crates/source` resource test" is
the unit test named above. Moved pins from the new per-ring row can reach
`crates/capi/tests/resource_lifecycle.rs` and `hosts/host-web/tests/browser-v1/expected.json`.
Re-pin from the measurement, with the reason.

**Realtime-path edits.** D2 edits `crates/engine/src/realtime/watermark.rs` and `mod.rs` and adds
the new `seqlock.rs` (B15-R1). D3 adds a render-side publication in the source consumer. That slice gets
one attempt.

## Decision record: #1316 (batch 15c)

**Shipped.** D1-D8 and B15-R1/R2 on `460fca68b`.

- `crates/engine/src/realtime/seqlock.rs` (new, B15-R1): `PublishedWords<const N: usize>`, a
  non-`Clone` `PublishedWordsWriter::publish(&mut self, &[u64; N])` (marked realtime region,
  wait-free) and a `Clone` `PublishedWordsReader::read` bounded at `MAXIMUM_READ_ATTEMPTS` (64),
  then `PublishedWordsBusy`. No `unsafe`. `watermark.rs` now stores its six words through it; its
  record size is unchanged (72 bytes). #1314's torn-read test moved to `seqlock.rs` with the
  protocol and the watermark copy was deleted.
- `crates/source`: `begin_block_at(source_read_sample)` and `SeekClock::At { source_read_sample }`
  with D1's text; `SourceCommand::SeekAt` and `HeldSeek` anchors documented on the source-read
  clock. `SourceSeekReport` (five words) and `SourceSeekReportBusy`; one record per ring made in
  `prepare_at_source_frame`, writer in the consumer, reader in the producer
  (`PcmSourceProducer::seek_report`, `HostChunkProvider::seek_report`). D3's publication is
  `publish_seek_report` at the end of `begin_block_play`, inside the existing marked region.
  `SourceResourceReport::seek_report_bytes` is in overhead, total and the largest-allocation list.
- host-core: `SourceControlSet::seek_report`, the `seek_at` doc (D7), and a new
  `SourceControlError::ReportBusy` (`source.report.busy`, backpressure).
- control-plane: `SessionState::seek_report` runs `service()` first (amendment), reads the newest
  providers; `pub use source::SourceSeekReport`.
- capi: `miso_engine_v1_source_seek_report`, `SOURCE_SEEK_REPORT_SIZE` 64,
  `FEATURE_SOURCE_SEEK_REPORT` = `1 << 9` (mask 1023), header thread table, service
  paragraph and a two-sentence note, both smoke programs, the frozen list (19 symbols) and
  `docs/C_ABI_V1_QUALIFICATION.md`.

**Departures.**

- The C struct is `miso_engine_v1_seek_report`, not D6's `miso_engine_v1_source_seek_report`: C
  and C++ give a typedef and a function one name space, so D6's two names do not compile
  (`redeclared as different kind of entity`). The size macro and feature bit keep D6's names.
- D3 read: a block that changes only a counter rewrites the two counters and keeps the
  generation's `first_sample` and `source_frame`, so `(g, s, f)` always names the generation's
  first block (the host workflow of gate 3).
- A clockless `begin_block` (tests and `read_block` only) continues its own clock: the block after
  a block at `t` is at `t + quantum`, from 0.
- No re-export in `crates/capi/src/runtime/mod.rs`: the export reads the report's fields without
  naming the type.
- Mechanical edits outside Authorized paths: `crates/host-core/tests/source_diagnostics.rs` gains
  the `ReportBusy` row (its exhaustive table does not compile without it), and
  `tools/realtime-policy/src/policy.rs` raises the floors to the tree's counts (33 files, 151
  regions), as its comment requires.

**Gates.**

1. `capi` `runtime::tests::a_plain_seek_is_reported_at_the_block_that_applied_it`: `(2, 768,
   4096, 0, 0)` after block 6, and `(1, 0, 0, 0, 0)` between the seek call and block 6.
2. `a_late_anchored_seek_reports_its_frame_plus_the_lateness`: `(2, 1280, 768, 0, 0)`, and block
   10 is bit-identical to block 6 (frames 768 on).
3. and 4. `a_scrubbed_stem_and_a_stem_started_from_its_report_stay_aligned`: Y anchored at
   `f + (A - s)`; blocks 10-15 bit-identical to a fresh session of the committed document fed both
   stems from the aligned frames; X unchanged across the carrying swap and its underrun counter
   moves after it; Y `(0, ..)` before the swap block, `(1, 896, 0, 0, held)` while held, then
   `(2, 1280, 4608, 0, 384)`. Source-level: `a_carried_consumer_continues_the_predecessor_audio_gap_free`
   now also checks the report across the swap.
5. `source` `the_seek_report_carries_the_cumulative_held_and_underrun_frames`; also
   `the_seek_report_names_the_block_and_frame_each_generation_started_at` and
   `a_clockless_consumer_reports_consecutive_quanta`.
6. `report_separates_session_pcm_from_source_overhead` adds the row. `./target/release/audit capi`:
   0 allocations, 0 deallocations, 0 syscalls, `"total_violations":0`, `pcm_digest`
   `4f52089e4a8cf83c` (unchanged).
7. `ffi::tests::seek_report_refusals_leave_out_untouched_and_name_their_rule`; `cargo test -p
   source`, `-p engine --features engine/realtime-audit`, `-p host-core --features
   host-core/test-support`, `-p capi`; `check-capi-abi.sh` and `--self-test`; `realtime-policy`;
   `check-cross-targets.sh`; `check-workspace-policy.sh`; clippy of the touched crates; fmt; doc.

**Measured.** `seek_report_bytes` is 64 per ring on 64-bit targets, 56 on wasm32 (`Arc`'s two
counters plus six `u64` words). host-web's `retained_ceilings` stays inside its budgets.

**Successors.** The browser export is #1332 and #1293 (Non-goals). `observe.rs` onto
`seqlock.rs` is #1482 D3. `source.id.invalid` on this entry point is #1350.

**Successor owner (root ruling, 2026-10-16).** The browser export of the seek report is #1293 D6
(batch 16) alone, with a parity row against the C ABI struct, independent of #1332 (which waits
on the owner's iOS test). Until #1293 lands, the browser cannot seek through the core, so it has
no seek to report; the C ABI reads the report today. #1293's amendment of 2026-10-16 records it.

Bit move: bit 9, because #1323 took bit 8 first; merge of 15b into 15c.
