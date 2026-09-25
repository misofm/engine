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
