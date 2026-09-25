# Decode native sources straight into the recycled transfer block

## Product outcome

The native source worker decodes each quantum into a private `planar_staging` block, then
`submit_native_planar` copies it channel by channel into a recycled `TransferBlock`. Decode into
the recycled block itself: take the block first, decode into `block.samples`, validate, publish.
One `channels·Q` copy per block per source disappears from the decode thread, and the worker's
staging allocation with it. Class A (the decoder's words are unchanged); off the render path.

## Root evidence

- `crates/source/src/native_source.rs:640` `SourceJob.planar_staging`, allocated at
  `:1007-1014` (`channels * quantum` words) and charged as `worker_planar_staging_bytes`
  (`:106-107`, `:274-275`, `:1411-1441`, `:1516`).
- `:1690-1704`: `decode_planar(&mut job.planar_staging, frames)` fills the staging block and
  records a `PendingBlock` (`:620-626`: metadata only); `:1637-1642` submits it through
  `HostChunkProvider::submit_native_planar` (`crates/source/src/lib.rs:927`), which calls
  `submit_planes` (`:824`): `validate_submission_metadata`, `take_recycled_block` (`:793`),
  stamp, then `copy_from_slice` per channel (`:846-852`), then `publish_block` (`:865`).
- `TransferBlock.samples` (`:478-484`) is laid out `[channel][quantum]`, the same planar layout
  `decode_planar` writes (`native_wave.rs:337`), so the decoder can target it directly.
- `producer_submission_has_one_stamping_and_copy_body` (`lib.rs:2506`) pins the text of
  `submit_planes`: exactly one stamping of each field, one `copy_from_slice`, one validation,
  and no `submit_contiguous_planar` symbol. It guards the *host* submit path and stays true;
  the native path gets its own reserve/commit pair.
- On backpressure the worker keeps `pending` and retries the submit (`:1667-1673`); on a seek it
  drops `pending` (`:1622`). The reserved block must survive both.

## Smallest closable slice

Authorized paths: `crates/source/src/lib.rs` (producer side: `HostChunkProvider` /
`PcmSourceProducer` native reserve/commit; `submit_native_planar` removed),
`crates/source/src/native_source.rs`, their tests, and this spec.

1. `PcmSourceProducer` (native only, `cfg(not(target_arch = "wasm32"))`):
   `reserve_block(&mut self) -> Result<ReservedBlock, HostChunkError>` wraps
   `take_recycled_block` and exposes `planes_mut(&mut self) -> &mut [f32]` (the whole
   `[channel][quantum]` sample block); `commit_block(&mut self, block: ReservedBlock,
   generation, start_frame, frames, end_of_region, sanitized) -> Result<SubmitReport,
   HostChunkError>` runs `validate_submission_metadata`, stamps, and `publish_block`s. A failed
   validation or a `Full` publish returns the block to `deferred_block` exactly as
   `publish_block` does today, so the acked-batch rule holds: the ack is the `Ok` of
   `publish_block` and nothing else. `HostChunkProvider` forwards both.
2. The worker: before decoding, `reserve_block`; if none is available, return the same
   `Idle::WaitingForRender` the failed submit returns today (no decode into nowhere). With both
   queues sized at `transfer_block_count`, a full data queue implies an empty recycle queue, so
   `reserve_block` returning `Full` is the live backpressure point and is where gate 1's stall
   is scripted. Decode with `decode_planar(reserved.planes_mut(), quantum)`. `commit_block`'s
   `Full` arm defers the block inside the producer exactly as `publish_block` does
   (`source:865-893`, `deferred_block`); the worker keeps `pending` and on retry calls a new
   `commit_deferred()` (push `deferred_block`, then ack), never a second `commit_block`. On a
   seek, a reserved but uncommitted block is kept for the next decode (its words are
   overwritten). Delete `planar_staging`, the `worker_planar_staging_bytes` field and the
   `worker.planar_staging` allocation row (every reader is inside `native_source.rs`), and
   update the report tests that pin them (`:2045`, `:2060`, `:2309`).
3. `submit_native_planar` and its callers are removed; `submit_planes` keeps serving host
   chunks unchanged.

## Non-goals

No change to the host (web/mobile) submit copy, to the consumer, to the decoder, to seek
semantics, or to the ring capacity. The web "reserve/commit" FFI that would remove the browser's
staging-to-block copy is a web ABI addition and is put to the owner in `PLAN.md`, not done here.

## Objective gates

1. New test: for a fixture WAV and a scripted consumer, the sequence of published blocks
   (generation, start frame, frames, end flag, sanitized count, and every sample word) is
   identical to the pre-change worker's, including a backpressure stall (ring full for two
   pops) and a seek mid-stream. Record the oracle from the pre-change worker in the test (a
   digest per block is acceptable).
2. `producer_submission_has_one_stamping_and_copy_body` passes unchanged: it scans only the
   text between `fn submit_planes` and `fn publish_block` (`source:2509-2520`), so place
   `commit_block` and `commit_deferred` **after** `publish_block`. A new sibling pins that
   `commit_block` contains no `copy_from_slice` and one `validate_submission_metadata`.
3. Existing native worker tests, including the `test-support` audit-hold tests, and
   `cargo test -p source --all-features`; the resource-report tests updated with the staging
   bytes gone; `scripts/check-realtime-policy.sh`, `scripts/check-native-pcm-runner.sh`,
   `scripts/test-native-pcm-runner-v1-policy.sh`.
4. Red mutations recorded in the evidence: commit without validation (red on the stale-generation
   case of gate 1), decode after stamping the old metadata (red), drop the reserved block on
   `Full` (red: the ring loses a block and the stall case never recovers).

## Console benchmark rows

None: the console rows do not use the ring, and the decode thread is not the render thread.

## Dependencies

None to implement. Shares `crates/source/src/lib.rs` with "Retain the played transfer block
through the render and expose its planes" (producer side here, consumer side there): may be
implemented concurrently; the second to merge rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate above that says "identical" is a hard stop, not a tolerance.
- The owner's copy rule: a block-sized copy exists only with a written justification that no in-place or direct-write form exists. This issue removes the decode-staging copy; the host submit copy that remains is justified by the host owning the chunk memory it hands over, and by the ABI question in `PLAN.md`.
- The acked-batch question: can an ack ever precede a drop? `commit_block` acks only after `publish_block` succeeds; a reserved block that is never committed is returned, never dropped.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory); this issue touches the worker thread only, and the ring's `unsafe` stays in `spsc.rs`.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. No benchmark row is listed.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (S2, PR #879) and tracker #349 (IO-12).

## What the implementer will hit

- `worker_planar_staging_bytes` and the `worker.planar_staging` row are read nowhere outside
  `native_source.rs`; remove them outright rather than zeroing them.
- `PendingBlock` stays as the metadata record; the reserved block is a second field beside it,
  and after a `Full` commit the block lives in the producer's `deferred_block`, not in the job.
- `decode_planar` requires `out.len() == channels * frames`; pass `quantum` frames and read
  `decoded_frames` from the report as today.

## Attempt 1 evidence

### Scope amendment (Sol)

Sol authorized two files beyond this brief's paths, for the repin only (commit `672a70fa`). The
change removes the 512-byte `worker.planar_staging` layout row, and `worker.job_array` shrinks
by 8 bytes (the job's 16-byte staging slice out, one 8-byte reserved-block slot in):

- `tools/audit/src/source_duration.rs`:
  - The cited lines are `before`/`after` the repin. `:324`/`:325`: the row count, 17 → 16.
  - `:326`/`:330`: the layout total, `6_416` → `5_896` (-520 = 512 + 8).
  - `:329`/`:334`: the canonical accounting digest, `0xfc47_9666_aec5_0448` → `0xafc6_12be_270e_b257`,
    the value the live `audit source-duration` run produces.
  - Comments name the removed row and the job-array shrink.
- `.github/workflows/qualification.yml:660`: `"layout_entries": 17, "layout_total_bytes": 6416` →
  `16, 5896`. Nothing else in the file changed.

Terra, branch `codex/919-decode-into-recycled-block` on `12b621f2`. Commits: `c03848a3` (gate-1
oracle recorded against the pre-change worker, before any production edit), `92905c91`
(implementation), `22126bdc` (block-conservation check in the harness), `18eca7a8` (rejected
block stays with the caller; see deviation 1), `7d498ed3` (evidence), `bbee135b` (block count
from the prepared ring), `672a70fa` (the amendment's repin), `54ab2cf9` (fixture and
constants independent of the circulating block count), `8e27b804` (amendment evidence),
`f4b8317b` (Sol's should-fixes: capacity-aware injector, N1 and N2 docs), and this evidence
update. Paths touched:
`crates/source/src/lib.rs`, `crates/source/src/native_source.rs`, this spec, and the two
amendment files. No performance claim.

### Design

- Producer (`lib.rs`, all `cfg(not(target_arch = "wasm32"))`; placed after `publish_block`, so
  `submit_planes`..`publish_block` is untouched). `take_recycled_block`, `submit_planes` and
  `publish_block` are byte-identical to `12b621f2` (sha256 of the span compared).
  - `struct ReservedBlock { block: Box<TransferBlock> }` (`:980`), `planes_mut()` = the whole
    `[channel][quantum]` `samples` slice.
  - `reserve_block()` (`:903`) = `take_recycled_block()` wrapped: a deferred block is pushed
    first, then one recycled block is popped; `Full` is the same error a submission reports.
  - `commit_block(&mut Option<ReservedBlock>, generation, start_frame, frames, end_of_region,
    sanitized)` (`:916`): `validate_submission_metadata(..)?` (block still in the caller's slot),
    then `reserved.take()`, stamp the five fields, raise the producer watermark, then
    `publish_block`. Its `Full` arm stores the stamped block in `deferred_block` (unchanged
    `publish_block`).
  - `commit_deferred()` (`:953`): take `deferred_block` (none: `InternalInvariant`), revalidate its
    stamp against the producer (fail: the block goes back to `deferred_block`, error returned),
    then `publish_block` it.
  - `HostChunkProvider` forwards all three (`:1023-1052`); `submit_native_planar` is deleted.
- Worker (`native_source.rs`): `SourceJob.planar_staging` is gone and `reserved:
  Option<ReservedBlock>` sits beside `pending` (`:637`). The staging allocation in
  `prepare_native_source_job`, the `worker_planar_staging_bytes` report field (and its
  contribution to `total_engine_owned_bytes` and `largest_allocation_bytes`, and in
  `base_source_resources`), and the `worker.planar_staging` layout row are deleted. Nothing
  outside `native_source.rs` read them.

### The reserve/decode/commit sequence (`service_job`, `:1546`)

1. Commands and seek admission are unchanged. An admitted seek sets `pending = None` and leaves
   `reserved` alone: a block that holds a discarded quantum is kept for the next decode, which
   overwrites it.
2. `pending` is `Some`:
   - `reserved` is `Some` (the decode filled it): `commit_block(&mut job.reserved, ..)`.
   - `reserved` is `None` (an earlier commit returned `Full`, and its stamped block waits in
     `deferred_block`): `commit_deferred()`. It never calls `commit_block` a second time.
   - `Ok` takes the unchanged ack arm (audit acknowledgements, `end_submitted`, `SourceReady`).
     `Full` keeps `pending` and returns `WaitingForRender` (or `Progress` if a command was
     popped), exactly as before. Any other error is `SubmitFailed`.
3. `end_submitted` returns idle before any reservation.
4. The block is taken from `job.reserved` (kept across a seek) or `reserve_block()`. On `Full`
   the worker returns the same `WaitingForRender`/`Progress` a failed submission returned and
   decodes nothing. With both queues sized to the block count, a full data queue implies an
   empty recycle queue, so this is where the ring's backpressure is met.
5. `decode_planar(reserved.planes_mut(), quantum)`. The block goes back into `job.reserved`
   *before* the decode result is propagated, so a decode error cannot drop it. `pending`
   records the metadata as before, and the call returns `Progress`. The commit happens on the
   next call, so blocks are still published on the same service calls as before.

### Why the ack cannot precede a drop

- The only acks are `publish_block`'s `Ok` arm, which runs after `try_push` put the block in the
  data queue. The producer's `next_write_frame`, `cumulative_written_frames` and
  `end_of_region_submitted` move only there. The worker's ack effects run only on
  `commit_block`/`commit_deferred` `Ok`, and those return `Ok` only from that arm.
- Every block is always in exactly one place: the data queue, the recycle queue, the render, the
  job's `reserved`, or the producer's `deferred_block`. `commit_block` either leaves the block in
  `reserved` (failed validation) or moves it to the data queue or `deferred_block`.
  `commit_deferred` moves it to the data queue or back to `deferred_block`. A seek and a decode
  error both leave it in `reserved`. The harness asserts this count after every script step
  (`run_worker_script`, `:4239`). The count is measured on the recycle queue before the first step,
  so the check holds for any prepared block count.
- No duplicate: the retry after `Full` pushes the deferred block itself (M6 below). No stale
  ack: a seek admitted between a `Full` commit and its retry makes the retry fail validation.
  The block then stays deferred and is published unacked by the next reservation, and the
  consumer discards it as stale (M7).

### Tests (exact names)

- Gate 1, `native_source.rs`. `run_worker_script` drives a prepared job's `service_job` on the
  test thread. A scripted render takes blocks raw off the data queue, so stale blocks are
  recorded too; it observes seeks and holds the played block until the next boundary. Fixture:
  stereo float32; region `[1, 43)`; quantum 4; three blocks. It carries a subnormal, NaN, ±inf
  and -0.0, all outside the frames either ring shape decodes ahead (21..=28, 30..=33).
  Script: fill, two more service rounds on a full ring, a boundary that plays a block but frees
  none, a seek on a stalled ring, a newer seek backpressured on the seek slot, run to a one-frame
  end-of-region block, drain, an in-phase restart whose first decoded quantum a second seek
  discards (this exercises the kept reserved block), and run to a three-frame end-of-region
  block.
  - `native_worker_publishes_the_recorded_block_sequence_through_stall_and_seeks`: 23
    `(generation, start, frames, end, sanitized watermark, FNV-1a of the published words)` rows
    (`PUBLISHED_SEQUENCE_ORACLE`) and the 36 Run/ServiceOnce idle states
    (`PUBLISHED_SEQUENCE_IDLE_RUNS`, run-length). Both were recorded from the pre-change
    production worker: first in `c03848a3`, where the test passes against the old code; then,
    for `54ab2cf9`'s fixture move (-inf from frame 27 to 29), from `c03848a3`'s code with only
    that move applied.
  - `native_worker_matches_the_pre_change_worker_block_for_block`: `pre_change_service_job`
    and `pre_change_submit_native_planar` are the parent's code verbatim. The only differences:
    staging passed as a parameter, and the removed method inlined as a test helper (a diff of
    the text shows exactly those lines). Every published word, stamp and idle state is equal,
    and the live oracle equals the recorded rows. Producer telemetry is equal except
    `recycle_empty_count`, which the test pins to exactly +1 from step 10 (deviation 3).
  - `a_stalled_seek_no_longer_counts_the_quantum_only_the_old_worker_decoded_ahead`
    (deviation 2).
  - `worker_retries_a_full_commit_through_the_deferred_block_without_loss_or_duplication`. The
    `OverfillDataQueue` step (injector rule below) lets the worker fill the data queue to its
    logical capacity and then meet it full at a commit. Nothing past the full queue is acked
    (cumulative = 4 × data-queue capacity, `data_full_count > 0`), the retry publishes the block
    once, and the six published blocks equal the old worker's no-stall stream word for word.
- Injector rule (both levels). A prepared ring never overfills its data queue, so the
  `Full`-path tests inject blocks past the ring. The count is (free data-queue slots + 1 −
  blocks already on the recycle queue), read from the live queues at injection time with
  `data_producer.available_capacity()` and `recycle_consumer.available_at_entry()`, never from
  the configured block count. The fillers are then committed until the data queue is at its
  logical capacity, and the last injected block is the one that meets it full. This holds for
  main's ring (queues = circulating blocks) and #917's (queues = circulating + 1).
  - Harness: `ScriptStep::OverfillDataQueue`.
  - `lib.rs`: `inject_blocks_to_overfill_the_data_queue`, `fill_the_data_queue`,
    `commit_until_reservation_is_full`.
- Producer, `lib.rs`:
  - `native_commit_publishes_the_decoded_block_without_a_copy` (gate 2 sibling): no
    `copy_from_slice`, one `validate_submission_metadata`, one `publish_block`; order is
    validate < take < stamp < publish.
  - `native_commit_rejects_a_stale_generation_and_keeps_the_block_unpublished`: the stale
    generation is rejected, telemetry is unchanged, the same block stays with the caller and is
    committed after the seek, and the render discards nothing.
  - `native_full_commit_defers_the_block_and_its_retry_acks_it_exactly_once`.
  - `native_deferred_retry_never_acks_a_block_a_seek_made_stale`.
- Ported off `submit_native_planar`:
  - `prepared_contiguous_native_submission_matches_planar_ring_shape`.
  - `host_and_native_submission_share_exact_short_eof_metadata_and_validation_order`. The
    `InternalInvariant` wrong-length case is unrepresentable now, so it is replaced by a
    `FrameCount` rejection whose block is then reused for the short end-of-region commit.
  - `stamped_native_watermark_survives_seek_stale_discard_and_saturates`.
- Report tests updated: `resolver_preparation_validates_identity_rate_channels_region_and_fixed_caps`,
  `native_worker_and_host_provider_produce_identical_prepared_ring_pcm`,
  `native_queue_layout_and_per_source_caps_use_exact_requests`,
  `prepared_source_job_is_inert_until_the_single_start_boundary` (staging assertions removed;
  `reserved.is_none()` asserted).
- `producer_submission_has_one_stamping_and_copy_body` is unchanged and passes.

### Gates

| gate | result |
|---|---|
| `cargo test -p source --all-features` | 68 + 1 doc passed (3 runs) |
| `cargo test -p source` | 65 + 1 passed |
| `cargo test --no-fail-fast -p capi -p host-core -p native-pcm-runner -p stem-hasher` | 28 binaries, 291 passed |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `cargo clippy -p source --target wasm32-unknown-unknown -- -D warnings` | clean |
| `cargo fmt --all --check` | clean |
| `scripts/check-realtime-policy.sh` | ok (50 regions, 14 files) |
| `scripts/check-native-pcm-runner.sh` | ok (V1 and portability) |
| `scripts/test-native-pcm-runner-v1-policy.sh` | ok |
| `audit source` (live, audited allocator) | 0 violations; underrun 128/1; resume 384 |
| `audit source-duration` | layout equal across durations: 16 rows, 5 896 bytes |
| `cargo test -p audit -p source --all-features` (after the amendment) | see "Amendment gates" |

### Mutations (all red; each applied alone, then restored)

| mutation | red tests |
|---|---|
| M1 commit without validation | `native_commit_rejects_a_stale_generation_and_keeps_the_block_unpublished`, `host_and_native_submission_share_exact_short_eof_metadata_and_validation_order`, the gate-2 sibling |
| M2 stamp the metadata before the decode (quantum frames, no end flag, pre-decode watermark) | `native_worker_publishes_the_recorded_block_sequence_through_stall_and_seeks`, `native_worker_matches_the_pre_change_worker_block_for_block`, `a_stalled_seek_...`, `worker_retries_a_full_commit_...`, `native_worker_and_host_provider_produce_identical_prepared_ring_pcm`, `multiblock_native_watermark_does_not_readd_the_cumulative_decoder_report` |
| M3 drop the block on a `Full` commit | `worker_retries_a_full_commit_...` (the retry finds nothing and the worker terminates), `native_full_commit_defers_...`, `native_deferred_retry_never_acks_...` |
| M4 a seek drops the reserved block | the three gate-1 script tests ("a transfer block left circulation at Run": 2 of 3) |
| M5 failed validation moves the block to `deferred_block` (the brief's wording) | `native_commit_rejects_a_stale_generation_...`, `host_and_native_submission_...` |
| M6 the retry re-reserves and commits again instead of `commit_deferred` | `worker_retries_a_full_commit_...` (the quantum is published twice) |
| M7 `commit_deferred` without revalidation | `native_deferred_retry_never_acks_a_block_a_seek_made_stale` |
| M8 failed validation drops the block | `native_commit_rejects_a_stale_generation_...`, `host_and_native_submission_...` |

Brief gate 4 wording: M1 is red on the stale-generation case at the producer. The worker-level
gate-1 tests stay green under M1 because the worker cannot commit stale metadata: an admitted
seek clears `pending` before any commit. Validation is the producer's defence for every caller.
With a prepared ring the `Full` commit is unreachable, since the reservation absorbs the full
ring. So M3, M6 and M7 are red only through the capacity-aware injector. They were re-run red on
this branch and on a scratch merge with #917 (see "Review should-fixes (Sol)").

### Deviations and findings

1. **A failed validation does not go to `deferred_block`** (the brief says it does). The next
   reservation's `take_recycled_block` publishes whatever `deferred_block` holds, with no ack.
   A rejected block there would reach the render still carrying its last trip's metadata (M5 is
   red). Attempt 1 first added a producer `returned_block` slot. That grew `PcmSourceProducer`
   by one pointer, which `crates/capi/tests/resource_lifecycle.rs` pins to the byte (two tests
   read +8). So `commit_block` instead takes the worker's `&mut Option<ReservedBlock>` and
   takes the block only after validation, leaving a rejected block with the worker as a seek
   does. The producer is main's size. The brief's by-value `block: ReservedBlock` became
   `&mut Option<ReservedBlock>`.
2. **No decode ahead of a full ring** (the brief's "no decode into nowhere"). The old worker
   decoded one quantum into staging while stalled. A seek that discarded that quantum still left
   its replacements in the cumulative sanitation watermark. The new worker never decodes it.
   After such a seek, stamps and worker events can be lower by that quantum's replacement count.
   No PCM word changes. Pinned by
   `a_stalled_seek_no_longer_counts_the_quantum_only_the_old_worker_decoded_ahead` (old = new + 1
   on every post-seek block).

   The watermark is host-visible only through worker events (`SourceReady`,
   `SanitationSnapshot`, `Terminal`) and this crate's own `SourceConsumerTelemetry`. It has no
   current reader: the only mention outside the crate, `crates/capi/tests/resource_lifecycle.rs:783`,
   is a layout mirror for a size pin and reads no value. No pinned fixture, digest, browser
   `expected.json` or SDK test observes it.

   Reviewer's ruling (Sol): class A. No rendered bit moves, and the new count is the more
   accurate one, because it counts replacements only in quanta the worker actually delivers or
   decodes.
3. **`recycle_empty_count` counts one more failed poll** after a seek admitted on a full ring.
   On that call the old worker decoded, while the new one polls for a block first. It is a
   producer telemetry poll counter. For a native source it is not even host-visible through
   worker events, because the producer lives inside the worker job. It has no current reader,
   and no pinned fixture, digest, browser `expected.json` or SDK test observes it.

   Reviewer's ruling (Sol): class A. No rendered bit moves, and the new count is the more
   accurate one: it records the poll the worker really made.
4. **A failing read is discovered when a block is free.**
   `decoder_failure_after_accepted_seek_keeps_typed_terminal` never rendered, so it relied on
   decode-ahead; unchanged, it waits forever for its terminal. It now takes one render boundary
   (`read_one`) after the seek, which recycles the stale blocks, and then gets the same typed
   `DecodeFailed(Io(Other))`. In a running render the terminal comes one boundary later. With
   the render stopped it waits until rendering resumes. This is documented on
   `NativeSourceController::wait_for_event` (N1). No current host surface waits for that event
   without rendering; `tools/audit/src/source.rs` is the only caller outside the crate.
5. **Published-sequence identity is per schedule.** Script steps land where both workers are
   in the same phase: after a `Run`, or single-stepped from a drained ring. After a stall is
   released, the old worker is one service call ahead (it had decoded ahead). A seek injected
   one service call after that release would make the old worker publish one more stale-
   generation block, which the consumer discards. Real threaded interleavings were never
   deterministic.
6. **Allocation-layout pins.** These were out of scope and are repinned under the scope
   amendment above.
7. **Merge with #917.** The reviewer's scratch merge of `2016de49` into this branch was clean,
   and so was mine (below). #917 sizes both queues at `count + 1` but pushes only the
   configured `count` blocks onto the recycle queue. Its extra block stays with the consumer,
   so the worker still circulates exactly `count` blocks, the reservation stays the
   backpressure point, and gate 1's recorded constants do not change. The one piece of test
   code that assumed a shape was the `Full`-path injector: it assumed the data queue's capacity
   equals the blocks on the recycle queue. It is now capacity-aware (injector rule above).

### Gate 1 after #917

The merge is clean and gate 1 needs no regeneration. With #917, the printer
`print_published_sequence_constants_from_the_pre_change_worker` still reports
`prepared transfer blocks: 3`, and its output equals the committed constants byte for byte.
`native_worker_publishes_the_recorded_block_sequence_through_stall_and_seeks` and
`native_worker_matches_the_pre_change_worker_block_for_block` pass unchanged. The only
shape-dependent code was the `Full`-path injector, now fixed.

The printer (`#[ignore]`) stays for a deliberate re-record. Use it only if the fixture, the
script, or the number of blocks a prepared ring circulates ever changes: run it with
`--ignored --nocapture` and paste its output over the two adjacent constants. The fixture keeps
every quantum a stalled old worker could decode ahead and drop free of replacement samples:
21..=24 with three circulating blocks, 25..=28 with four, and 30..=33.

### Amendment gates

These results are for `54ab2cf9`, on top of `672a70fa`.

| gate | result |
|---|---|
| `cargo test -p audit -p source --all-features` | audit 49 passed (including `exact_duration_independent_accounting_serialization_is_canonical`); source 68 passed, 1 ignored (the printer), doc 1 |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked -p audit -p source --all-targets --all-features -- -D warnings` | exit 0 |
| `python3 -B scripts/check-ci-path-routing.py` (qualification.yml's own self-check) | contract passed |
| `python3 -B scripts/test-ci-path-routing.py` | classifier and mutation tests passed |
| `bash scripts/check-realtime-policy.sh` | ok (50 regions, 14 files) |
| mutations M1-M8 re-run after the fixture move | all red, same tests as the table above |
| live `audit source-duration` | 16 rows, 5 896 bytes, layout equal across durations |

### Review should-fixes (Sol)

These results are for `f4b8317b`.

| gate | result |
|---|---|
| `cargo test -p source --all-features` | 68 passed, 1 ignored (the printer), doc 1 |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked -p source --all-targets --all-features -- -D warnings` | exit 0 |
| mutations M1-M8 on this branch | all red, same tests as the table above |
| trial merge of `origin/codex/917-retain-played-transfer-block` (`2016de49`) into a throwaway copy of `f4b8317b` | clean merge; `cargo test -p source --all-features`: 72 passed, 1 ignored, doc 1; `cargo clippy -p source` clean; printer `prepared transfer blocks: 3`, output equal to the committed constants; M1-M8 all red with the same tests (the M3/M6/M7 `Full` path included). The scratch worktree was then removed; no merge is on this branch. |

## Sol attempt 1 verdict: PASS

Adversarial review (Fable 5.1, high effort) against `c03848a3` through `8e27b804` on base
`12b621f2`, then a focused re-check of the should-fix commits `f4b8317b` and `5f0a715e`.
No blocking findings. The acked-batch question, traced in code: the only ack is still
`publish_block`'s `Ok` after the data push; `take_recycled_block`, `submit_planes` and
`publish_block` are byte-identical to base; `commit_block` validates before taking the block,
stamps, and calls `publish_block` once; the worker calls `commit_block` only with a reserved block
and `commit_deferred` otherwise, never twice; `commit_deferred` with nothing deferred is
unreachable by the state machine; a seek between a `Full` commit and its retry leaves the block
to be pushed unacked and discarded as stale by the consumer, not leaked (conservation asserted
after every scripted step); a decode error after reserve returns the block to the job before the
error propagates. Deviation 1 is correct and the brief was wrong: a validation-rejected block in
`deferred_block` would be published unacked on the next reservation carrying its previous trip's
stamp over fresh samples. PCM bytes: the block-for-block test compares against a verbatim copy of
the old worker whose only diff is the staging parameter and the inlined submit, recorded before
the production change; the fixture carries subnormal, NaN, ±inf and `-0.0`. Deviations 2 and 3
(a seek on a full ring no longer counts a quantum the old worker decoded ahead; one more empty
recycle poll) move no rendered bit, are read by no fixture, digest, browser `expected.json` or SDK
test, and the new count is the more accurate one: class A, no ruling needed. The repin
(16 rows, 5,896 bytes, digest `0xafc6_12be_270e_b257`) reproduced by the reviewer's live
`audit source-duration` run; `qualification.yml` changes one value and both path-routing
scripts pass. Mutations M1, M2, M3, M6 re-applied and reverted, red as claimed.

Should-fixes found by a trial merge with #917 and applied by the implementer: the merge is clean
and gate 1's constants do not change (the retained block never enters the recycle queue, so three
blocks circulate on both shapes); the only shape-dependent code was the `Full`-path test injector,
now derived from the live queue cursors (free data-queue slots plus one, minus blocks on the
recycle queue) and red-mutation-proof on both shapes; the stale-retry test was strengthened, not
weakened (it now also asserts every generation-1 block is discarded and none is played). The
re-check trial merge: 72 source tests green, constants byte-identical. Not re-verified by the
reviewer: workspace-wide and wasm clippy, the native-pcm-runner scripts, and the audited-allocator
`audit source` run (the implementer reports all green).
