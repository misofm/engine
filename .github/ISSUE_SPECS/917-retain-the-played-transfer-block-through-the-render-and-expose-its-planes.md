# Retain the played transfer block through the render and expose its planes

## Product outcome

The source consumer pops a `TransferBlock` at `begin_block`, copies its channels into the arena,
and recycles it as soon as the last claim has copied -- before the graph renders. Nothing can
read the block in place because it is already back with the producer. Make the ring's ownership
rule explicit and one block longer: the consumer retains the played block until the next
`begin_block`, exposes its channel planes as quantum-long read-only slices (tail zeroed in
place on a short block), and the ring allocates one block more than the configured capacity so
the producer's admission depth is unchanged. This is the ownership contract "Gather banked
source inputs from the played transfer block" needs; on its own it moves no copy and no
rendered bit. Class A in audio; a ring contract change in ownership timing and memory, stated
below and ruled (`count + 1`, charged to overhead) in `PLAN.md`.

## Root evidence

- `crates/source/src/lib.rs:1125` `PcmSourceConsumer::begin_block` starts with `self.end_block()`
  (release the previous `played`), `flush_deferred_recycle`, then `acquire_current_block`
  (`:1300`) and moves the matching block into `played` (`:1141-1150`).
- `:1188` `copy_channel` copies `played.samples[offset..offset + frames]` into the destination
  and zero-fills `destination[frames..]`; `:1231` `end_block` resets metadata and recycles.
- `:1613` `SourceGraphSourceSetDriver::copy_track_input` calls `end_block` on every source when
  `copied_claims == mappings.len()` (`:1642-1646`), i.e. during the executor's copy loop
  (`crates/graph/src/lib.rs:2226-2231`), before any unit runs. `begin_block` (`:1582`) also
  releases immediately when there are no mappings (`:1601-1605`).
- Ring shape: `PreparedShape::validate` (`:687-691`) sets `transfer_block_count = frame_capacity /
  quantum`; `prepare` builds the data and recycle queues at that capacity (`:594-599`) and
  pre-fills the recycle queue with exactly that many blocks (`:633-639`); `resource_report`
  (`:519-560`) charges `pcm = frame_capacity * channels * 4` as
  `pcm_payload_already_charged_bytes` and everything else as overhead. Tests pin these numbers:
  `report_separates_session_pcm_from_source_overhead` (`:1830`, count 2, 64 bytes),
  `prepared_contiguous_native_submission_matches_planar_ring_shape` (`:2405`, count 1).
- `docs/REALTIME_MEMORY.md` "SPSC cursor protocol": the ring never allocates on push/pop; the
  block count is fixed at prepare. The audited-in-render allocation gates
  (`crates/source` tests, `tools/audit`) must stay green.
- Web hosts submit through `submit_planes` (`:824`) on the same consumer type; the browser feed
  is copied into a recycled block on submit and read by the same consumer, so this contract
  applies to native and web alike.

## Smallest closable slice

Authorized paths: `crates/source/src/lib.rs`, `crates/source/src/native_source.rs` (only if
`worker_render_wait` or the resource report's block count needs the `+1`), their tests,
`docs/REALTIME_MEMORY.md` (one paragraph under "SPSC cursor protocol" stating the hold rule), and
this spec.

1. **Hold rule.** `SourceGraphSourceSetDriver::copy_track_input` no longer calls `end_block`;
   the previous block is released by the next `begin_block`, which already does so, or by
   `prepare_seek` (`:1051`, which calls `end_block` first; render thread, between blocks). Keep
   the no-mappings release in `begin_block`. Document on `PcmSourceConsumer`: "the played block
   is the consumer's from `begin_block` until the next `begin_block`, `prepare_seek`,
   `end_block`, or drop".
2. **Planes.** `pub fn played_plane(&self, channel: u32) -> Option<&[f32]>` returns the
   quantum-long channel plane of the played block, or `None` when no block was played this
   quantum (underrun, end of region). `begin_block` zero-fills `samples[offset + frames ..
   offset + quantum]` for every channel of a short block, in place, once, so the slice is valid
   for the whole quantum; `copy_channel` keeps its own tail fill (it must stay correct for
   callers that never call `played_plane`). Add the driver-level
   `SourceGraphSourceSetDriver::played_planes(&self, claim_index) -> Option<(&[f32], &[f32])>`
   mapping the claim's `(left_channel, right_channel)`; this is a source-crate method until the
   graph issue lifts it onto the graph trait.
3. **Capacity.** `PreparedShape` allocates `transfer_block_count + 1` blocks and sizes both
   queues at `transfer_block_count + 1`, so with one block retained by the consumer the producer
   can still admit `transfer_block_count` blocks. `SourceResourceReport` keeps
   `transfer_block_count` as the configured number and adds `retained_block_count: 1`; the
   extra block's PCM and metadata are charged to `overhead_bytes` (not to
   `pcm_payload_already_charged_bytes`, which the session accounting already charged). The only
   pinned numbers are the two source tests named above (`:1830`, `:2405`); update them.
   `crates/host-core/tests/prepare.rs:178` checks source overhead against a cap, not a pin, and
   `hosts/host-web/src/lib.rs:7906` copies `source_overhead_bytes` through; neither needs an
   edit unless the cap is exceeded, which the implementer states.

## Non-goals

No change to what the render reads (the graph still copies through `copy_track_input`; the next
issue removes that), to the producer side, to seek semantics, to the SPSC protocol, or to the
web submit path.

## Objective gates

1. New test (source): with a ring of configured count `N` and a producer submitting continuously,
   the consumer retains the played block through a simulated render (call `begin_block`, read
   `played_plane`, then `begin_block` again) and the producer observes `N` admissions in flight
   at every point -- the same admission sequence as before this change, recorded as an oracle
   from the pre-change ring (fixed submit/pop script, compare the `SubmitReport`/`Full` sequence
   word for word).
2. New test: `played_plane` on a short block returns `frames` samples followed by exact `+0.0`
   words to the quantum; on an underrun it returns `None`; `copy_channel` output is unchanged
   (bit compare against the pre-change oracle `copy_channel_oracle` at `:2081`).
3. Existing: `copy_channel_zeroes_only_the_tail_the_full_fill_used_to_reach` (`:2127`), the
   driver tests at `:2252-2285`, every `crates/source` test, `cargo test -p source -p graph
   -p host-core --all-features`; `scripts/check-realtime-policy.sh` (no allocation on
   `begin_block`/`played_plane`); the native worker's audited-hold tests (`native_source.rs`,
   `test-support`) still pass with the `+1` block.
4. `docs/REALTIME_MEMORY.md` paragraph added; `scripts/check-workspace-policy.sh` and
   `scripts/check-session-policy.sh` unaffected (run them).

## Console benchmark rows

None. The console benchmark binds `FrozenGraphSource` processors
(`tools/console-workload/src/lib.rs:1499`), not the source ring; no row can move and none is
expected to.

## Dependencies

None. Unblocks "Gather banked source inputs from the played transfer block". Shares
`crates/source/src/lib.rs` with "Decode native sources straight into the recycled transfer
block" (consumer side here, producer side there; the second to merge rebases).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "bit-identical" or "unchanged" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written justification that no in-place or direct-write form exists. This issue prepares the removal of the ring-to-arena copy; it must not add a copy anywhere.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). The ring's `unsafe` stays inside `crates/engine/src/realtime/spsc.rs`.
- The acked-batch question applies to every queue change: an admission is acked only after the block is in the data ring; the `+1` block must never let an ack precede a drop.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. No benchmark row is listed.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary; do not repin here.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (§1.2 step 2, §1.3, S2/S6, PR #879) and tracker #349 (IO-12).

## What the implementer will hit

- `read_block_contiguous` (`:1091`) calls `begin_block`/`copy_channel`/`end_block` itself and is
  unaffected; do not change it.
- `worker_render_wait` (defined at `native_source.rs:1561`, called at `:1029-1033`) derives
  the worker's idle wait from `transfer_block_count`; keep it on the configured count.
- The `+1` block raises `SourceResourceReport::overhead_bytes`; the ruling is to charge it to
  overhead (coordinator decision recorded in `PLAN.md`). `crates/host-core/tests/prepare.rs:360`
  is the source-id arena test and is unrelated; `scripts/check-web-boot-budget.mjs` never
  mentions source overhead.

## Attempt 1 evidence

Implementer: Terra. Branch `codex/917-retain-played-transfer-block`, based on `12b621f2`
(synchronized `main` plus the cycle's briefs). Commits: `a7f8ce93` (ring shape, hold rule,
planes, tests), `9cbbc8ad` (`docs/REALTIME_MEMORY.md` paragraph), `5e24b839` (realtime marker on
`played_plane`), and this record. Every `file:line` anchor in the body was verified on `12b621f2`
before editing; none had drifted.

### Design

1. **Hold rule.** `SourceGraphSourceSetDriver::copy_track_input` no longer calls `end_block` and
   the `copied_claims` counter it existed for is gone. The played block stops being readable at
   the next `begin_block` (which still starts with `end_block`), at `prepare_seek` (which still
   starts with `end_block`), at an explicit `end_block`, or at drop. The no-mappings release in
   the driver's `begin_block` is kept. `PcmSourceConsumer` carries the documented rule.
2. **Planes.** `PcmSourceConsumer::played_plane(&self, channel) -> Option<&[f32]>` returns the
   quantum-long plane of the played block in place; `None` when nothing was played this quantum
   (underrun, end of region, after `end_block`/`prepare_seek`) or the channel is out of range.
   `begin_block` hands a matching block to a new private `play`, which zero-fills
   `samples[offset + frames .. offset + quantum]` of every channel of a short block, in place, once.
   `copy_channel` is untouched and keeps its own tail fill. The driver gets
   `pub(crate) fn played_planes(&self, claim_index) -> Option<(&[f32], &[f32])>` mapping the
   claim's `(left_channel, right_channel)`; it is `#[cfg_attr(not(test), allow(dead_code))]` until
   the graph issue lifts it onto the trait.
3. **Capacity.** `PreparedShape` carries `allocated_block_count = transfer_block_count + 1`
   (`RETAINED_TRANSFER_BLOCKS = 1`); `prepare` sizes both queues at it and allocates that many
   blocks. `SourceResourceReport` keeps `transfer_block_count` (and `PcmSourceShape`,
   `transfer_block_capacity()`, the `acquire_current_block` pop bound and `worker_render_wait`) on
   the configured count, and adds `retained_block_count: 1` and `retained_block_pcm_bytes`. The
   extra block's PCM, its `TransferBlock` metadata (`transfer_block_metadata_bytes` now counts
   `N + 1` blocks) and the two extra queue slots are in `overhead_bytes`;
   `pcm_payload_already_charged_bytes` and `transfer_block_pcm_bytes` are unchanged. The
   test-support `native_source_allocation_layout` counts `transfer_block_count +
   retained_block_count` blocks so `tools/audit source-duration`'s layout/report equality holds
   (the one `native_source.rs` edit; `worker_render_wait` is unchanged on the configured count).

**Where the extra block lives -- the one mechanism choice the body leaves open.** Allocating
`N + 1` blocks into the recycle queue and recycling the played block at `end_block` would give the
producer `N + 1` admissions whenever the consumer holds nothing: on a fresh ring, after any
underrun or end of region, and after every `read_block` / `end_block`. That fails gate 1 word for
word (red mutation M1 below) and, by reading (not run under the mutation), would also contradict
admission pins outside this issue's paths (`hosts/host-web/src/tests.rs` `source_backpressure_seek_render_and_stable_output_are_bounded`
expects backpressure after one submission into a one-quantum ring; the independent seek model in
`tools/audit/src/source_fixture.rs` models `free_blocks = capacity`). So the extra block is born
in the consumer (`retained_idle`), and the consumer owns **exactly one block outside both queues
at every block boundary**: the played block while one is readable, otherwise the same storage
idle. `end_block` ends the played quantum (planes `None`, `copy_channel` writes silence) and keeps
the storage as `retained_idle`; `play` pushes the idle block to the recycle queue only when a newer
block becomes the played block. `played` and `retained_idle` are never both occupied
(`debug_assert!`, and the unreachable branch recycles rather than frees).

### Ownership argument

- **Who holds the played block.** The consumer, in `played`, from the `begin_block` that played it
  until the next `begin_block`, `prepare_seek`, `end_block` or drop; after that, the same storage
  sits in `retained_idle` until a newer block is played.
- **Release points.** The played block's data stops being readable at `end_block` (called first
  by `begin_block` and `prepare_seek`, by the driver's zero-mapping path, by `read_block` /
  `read_block_contiguous`, which are unchanged). Its storage reaches the recycle queue at exactly
  one place: `play`, inside `begin_block`, after `end_block` has already moved it to idle and after
  the new block is in `played`. Stale blocks (`discard_block`) and the defensive branch of
  `end_block` never touch the played block. Drop frees blocks off the render thread with the plan.
- **No recycle while a render reads.** A plane is a `&[f32]` borrowed from `&self` (consumer or
  driver). Every release point takes `&mut self`, so the borrow checker ends every plane borrow
  before its block can move; the producer can only reach a block by popping the recycle queue,
  which happens-after the consumer's push (SPSC release/acquire). While planes are borrowed the
  producer fills its configured depth into the other `N` blocks and the borrowed words do not move
  (`played_planes_stay_intact_while_the_producer_fills_its_configured_depth`).
- **Ack ordering unchanged.** The producer side is not edited: `publish_block` acks (`Ok`) only
  after `data_producer.try_push` succeeds, and `take_recycled_block` / `Full` are as before. The
  retained block enters the data queue only as an ordinary submission after the consumer recycled
  it. Both queues hold `N + 1` = every allocated block, so a block being pushed is never refused
  (it is not in the queue it is pushed to): no push can fail, so nothing is deferred and nothing
  is dropped; `deferred_block` / `deferred_recycle` remain unreachable. An ack cannot precede a
  drop because no block is ever dropped.
- **Admission depth.** Consumer hold after the change = hold before + 1 at every boundary (before:
  `current` only; after: `current` plus the one retained block), so with one more block the
  producer's free count, and therefore every `SubmitReport`/`Full`, is the pre-change one on every
  consumer path. Mid-render, before the pre-change driver's last claim, the producer used to be one
  block short; it now has its configured depth throughout.
- **Memory delta per source (fixed, independent of stem duration).** One block's PCM
  (`channels * quantum * 4` bytes), one `TransferBlock` (48 bytes on 64-bit), one
  pointer slot in each of the two queues (2 x 8 bytes on 64-bit), and 8 bytes of
  `PcmSourceConsumer` (`retained_idle`); per source set, the driver loses `copied_claims` (8 bytes
  on 64-bit). For the 2-channel, 128-frame, 8-block ring: ring overhead 1,328 -> 2,416 bytes
  (+1,088 = 1,024 + 48 + 16); `pcm_payload_already_charged_bytes` stays 8,192.

### Tests (all in `crates/source/src/lib.rs`)

New:
- `tests::played_block_retention_keeps_the_pre_change_admission_sequence` -- gate 1. A fixed
  script (`AdmissionScript`, three configured blocks, two channels): prefill to `Full`, steady
  state, two renders then refill, drain into an underrun then refill, a mid-stream seek with stale
  blocks queued, a paused `prepare_seek` twice (the second with a current block held), a short
  end-of-region block and renders past the end. Every `SubmitReport`/`Full`, seek, prepared seek,
  render report and plane word is compared word for word with `PRE_RETENTION_ADMISSION_ORACLE`,
  604 words recorded on `12b621f2` by the same script with the render `begin_block;
  copy_channel x2; end_block` (the pre-change driver's release point). Three legs: the consumer
  holding through `played_plane` (and asserting each plane equals `copy_channel`'s output bit for
  bit), a caller that still ends its blocks, and the graph driver (`played_planes` +
  `copy_track_input`, held after the last claim, released by `prepare_source_seek`).
- `tests::played_planes_stay_intact_while_the_producer_fills_its_configured_depth` -- gate 1
  "at every point": for `N` = 1, 2, 3, with the planes borrowed the producer admits exactly `N`
  more blocks then `Full`, the borrowed words are unchanged, and the next `begin_block` opens one
  admission.
- `tests::played_plane_is_the_quantum_with_a_zeroed_short_tail_and_none_without_a_block` -- gate
  2: every allocated block is rotated through poison (`-0.0`, `NaN`, `1e30`), then a short block
  lands in poisoned storage; `played_plane` is the two frames then exact `0x00000000` words, twice;
  `None` before any block, on channel 2, after `end_block`, past the end of the region and on an
  underrun; `copy_channel` equals `copy_channel_oracle` bit for bit at each step.
- `tests::graph_driver_played_planes_map_claims_until_the_next_begin_or_seek` -- the driver maps
  `(0, 1)` and `(3, 2)` of a four-channel source, holds through every claim copy, is `None` on an
  out-of-range claim, after an underrun `begin_block`, after `prepare_source_seek`, and at once
  with zero mappings.

Changed:
- `tests::graph_driver_last_claim_recycles_in_call_and_incomplete_paths_recycle_next_begin` is
  restated as `tests::graph_driver_retains_played_blocks_to_next_begin_on_every_claim_path`. Its
  incomplete-claims leg asserted `Full` after one of two claims copied on a one-block ring, i.e.
  that the render starved the producer; under this issue that assertion must invert, so each leg
  (all claims, a missing claim, a failed claim copy) now asserts the planes are still held, the
  producer admits its configured one block and then `Full`, and the next `begin_block` releases.
  This is the only existing assertion whose meaning changed.
- `tests::report_separates_session_pcm_from_source_overhead` pins `retained_block_count == 1`,
  `retained_block_pcm_bytes == 32`, both queues and the metadata at three blocks, `overhead_bytes`
  as the exact sum of its rows, the delta over the pre-change two-block report (`32 + metadata +
  2 slots`), and `largest_allocation_bytes` exactly.
- `tests::prepared_contiguous_native_submission_matches_planar_ring_shape` also pins
  `transfer_block_count == 1` in the shape, `transfer_block_capacity()` and the report, and
  `retained_block_count == 1`.

Unchanged and green: `copy_channel_zeroes_only_the_tail_the_full_fill_used_to_reach`,
`graph_driver_forwards_underrun_and_seek_generation_facts`,
`graph_driver_zero_claims_recycles_in_begin_and_retains_no_plane_class`,
`retained_played_block_supports_repeat_fanout_and_auto_recycles_once`,
`paused_seek_prepares_full_queues_without_consuming_target`,
`host_submission_is_fifo_wraparound_and_never_accepts_a_prefix`, and every `native_source.rs`
test including the `test-support` audit-hold tests.

### Gates

| gate | result |
|---|---|
| `cargo test -p source --all-features` | ok, 64 lib tests (native worker and `test-support` audit-hold tests included) |
| `cargo test -p source -p graph -p host-core --all-features` | ok, 23 test targets, 0 failed |
| `cargo test -p host-web` (native; the web submit/backpressure pins) | ok, 204 passed, 2 ignored |
| `cargo clippy -p source -p host-core --all-targets --all-features -- -D warnings` | clean |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| `cargo clippy -p source --target wasm32-unknown-unknown -- -D warnings` | clean for this crate (only the pre-existing `math::fast_db` doc-link warnings) |
| `cargo fmt --all --check` | clean |
| `bash scripts/check-realtime-policy.sh` | ok, 51 regions in 15 files (`played_plane` is now a marked region; red-checked with a `Vec::new()` probe) |
| `bash scripts/check-workspace-policy.sh` | ok |
| `bash scripts/check-session-policy.sh` | ok |
| `audit source` (100,000 blocks through the graph source set with the held block, native worker hold/release) | `allocations 0, deallocations 0, locks 0, syscalls 0, total_violations 0` |
| `audit source-duration` | `layout_equal`, `source_report_equal`, `graph_report_equal` all true, 17 entries |
| `audit fixture-source` (independent seek/admission model) | ok |
| `crates/capi/tests/resource_lifecycle.rs` | **red, out of this issue's paths** -- see below |
| `scripts/check-browser-expected-resources.py` | **red, out of this issue's paths** -- the only stale rows are the two source rows (see below); self-test passed (26 red mutations) |

`scripts/check-realtime-policy.sh` does not scan `begin_block` (the source crate had no marked
region and `begin_block`'s pre-existing `.expect` calls would trip the scan); its
allocation-freedom is evidenced by the `audit source` run above, which drives it 100,000 times.

### Mutations (each applied alone, `cargo test -p source --all-features --lib`)

| mutation | result |
|---|---|
| M1: the extra block starts in the recycle queue and `end_block` recycles (the literal "+1 in the ring") | red: gate 1, `host_submission_is_fifo_wraparound_and_never_accepts_a_prefix`, `native_source::tests::single_worker_seek_resumes_contiguously_at_the_exact_frame`, `native_source::tests::worker_coalesces_provider_backpressure_to_latest_exact_frame_without_intermediate_pcm` |
| M2: the hold rule without the extra block | red: gate 1, the mid-render depth test, gate 2, both driver tests, the zero-claims and FIFO tests, `idle_decode_thread_cpu_is_bounded_and_one_thread_serves_a_set`; one native worker test hung and was killed |
| M3: no in-place tail fill in `play` | red: gate 1 (plane != copy) and gate 2 |
| M4: the last claim ends the block again | red: both driver tests and gate 1's driver leg |
| M5: `play` keeps the idle block | red: 24 tests (the `debug_assert!` and lost admissions) |
| M6: both queues sized at the configured count | **green** -- an equivalent mutant: because the consumer always holds one of the `N + 1` blocks, neither queue ever holds more than `N`. The `N + 1` sizing the body asks for makes every push infallible by the local argument "a block being pushed is not in its queue", independent of the consumer's holding discipline, for 2 x 8 bytes per source. |

### Deviations and out-of-scope pins

- Mechanism: the retained block is born in the consumer and `end_block` keeps the storage idle
  rather than recycling it (above). The release points of the played data are exactly the body's.
- `copied_claims` is removed from the driver (it had no reader left).
- Added `SourceResourceReport::retained_block_pcm_bytes` beside the required
  `retained_block_count`, so `overhead_bytes` stays the exact sum of named rows.
- `played_plane` is placed under a `REALTIME_POLICY` marker (not required by the body).
- **Pinned numbers outside the authorized paths move** (the body's "only pinned numbers" list is
  incomplete). Not edited here; they need an owner-approved amendment:
  - `crates/capi/tests/resource_lifecycle.rs` fails `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps`
    (`primitive source total: left 11062, right 11054` -- the mirror's real `PcmSourceConsumer`
    grew 8 bytes). The re-pin: in `source_owners()` size both queue mirrors and the metadata row at
    `blocks + 1`, add a retained-block PCM row of `bytes::<f32>(128 * channels)` (1,024), and drop
    `copied_claims` from `SourceGraphSourceSetDriverMirror`; then `source_overhead_bytes` 2,862 ->
    3,950 and `source_total_bytes` 11,054 -> 12,142 in the expected report, and the primitive
    oracle's 11,054 / 2,862 / 22,108 / 5,724 -> 12,142 / 3,950 / 24,284 / 7,900 (+1,088 per
    source: +1,024 PCM, +48 metadata, +16 queue slots, +8 consumer, -8 driver).
  - `hosts/host-web/tests/browser-v1/expected.json` `directOracle` rows `sourceTotalBytes` /
    `sourceOverheadBytes` (checked by `scripts/check-browser-expected-resources.py` in the
    required qualification workflow) move `sourceOverheadBytes` 1,186 -> 2,262 and
    `sourceTotalBytes` 2,210 -> 3,286, measured from the built simd128 module (+1,076; consistent with +1,024 PCM, a 40-byte
    wasm32 `TransferBlock`, 2 x 4-byte slots, +8 consumer after padding and -4 driver, which are
    derived, not measured individually). No other row moved. This belongs with the batch-boundary browser re-pin in `PLAN.md` or an amendment here.
  - The capi numbers above were confirmed by applying that re-pin as a transient, uncommitted
    edit (all four `resource_lifecycle` tests green) and reverting it; nothing outside the
    authorized paths is changed on this branch.
- The `M2` run's hung binary was a native worker test waiting on a ring with no free block; it
  was killed by hand, and the mutation was reverted like the others.
