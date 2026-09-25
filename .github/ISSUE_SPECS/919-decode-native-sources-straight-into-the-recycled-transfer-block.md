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
