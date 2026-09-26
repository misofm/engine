# Add a live-producer ring row to the console benchmark

**Ruled** (coordinator, 2026-09-26): finding 1 of the adversarial verification of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md` (`DIAGNOSIS-2-VERIFY.md`, section 5).
Benchmark tooling only; no engine source changes.

## Product outcome

`sixty_four_track_plumbing_ring` feeds the graph from a `FrozenSourceDriver` that lends the same
frozen 64 KiB block every block. That block stays warm in the render core's L2, so the row never pays
for input written by another core. In a production session a producer (a decode worker, or a host
feeding decoded chunks) writes every source's ring each block from a different core. The
verification's cross-core probe (64 rings, a producer writing 1 KiB per source per block) measured
the render-side read of those blocks:

| producer | read of 64 x 1 KiB per block |
|---|---:|
| none (frozen, like the ring row) | 2,261 cycles |
| SMT sibling (shared L2) | 3,152 |
| same CCD, other core | 5,894-6,416 |
| other CCD | 20,572-20,979 |

That is before `begin_block`'s own per-source work (pop, recycle, generation check;
`crates/source/src/lib.rs:1301`). On a live feed this term is larger than every remaining plumbing
change combined, and no row measures it. Add one row that renders the same 64-track plumbing session
from the production source path with a live producer thread on another core of the same CCD.

## Smallest closable slice

Authorized paths: `tools/console-workload/` (the new workload and its driver), `tools/bench/` (the
row's timing loop and producer thread, and `Cargo.toml`), `scripts/run-console-benchmark.sh` and
`scripts/operator/preflight-console-benchmark.sh` (CPU selection and record count only), their tests,
and this spec.

1. **The workload.** Add `sixty_four_track_plumbing_live` to `DRIVER_FED_WORKLOADS`
   (`tools/console-workload/src/lib.rs:415`), with `source_feed` = `"live_producer"`. It compiles the
   same plumbing session as the ring row and binds a real source set: `prepare_graph_source_set`
   (`crates/source/src/lib.rs:1925`) over 64 host-region sources, each fed by a
   `HostChunkProvider::submit` (`lib.rs:1048`) with the same per-track content the frozen driver lends
   (`frozen_track_source`, `tools/console-workload/src/lib.rs:1742`). Copy the construction chain the
   host already uses rather than inventing one: `PcmSourceRing::prepare_host_region`
   (`crates/source/src/lib.rs:599`) as called at `crates/host-core/src/prepare.rs:953`, the provider
   wrapper in `crates/host-core/src/source.rs`, and the source-fed session in
   `crates/host-core/tests/source_in_place.rs`.
2. **The producer.** One thread, started before warmup and joined after the last measured block,
   submits each source's next chunk so every ring stays at least two blocks ahead. It runs off the
   render thread and may allocate only before timing starts. The timed region is `render` alone,
   including the source set's `begin_block`, exactly as the other driver-fed row times it.
3. **Placement.** The runner already pins the bench process with `taskset -c "$bench_cpu"`
   (`scripts/run-console-benchmark.sh:459-463`). Choose `producer_cpu` as a core that shares
   `bench_cpu`'s L3 (`/sys/devices/system/cpu/cpu$bench_cpu/cache/index3/shared_cpu_list`) and is
   not its SMT sibling (`topology/thread_siblings_list`). Widen the process mask to both CPUs, pass
   both to the bench, and have the bench pin its render thread to `bench_cpu` for every row and the
   producer thread to `producer_cpu`. `libc` 0.2.189 is already in `Cargo.lock`; a direct Linux-only
   dependency of `tools/bench` is acceptable if `scripts/check-workspace-policy.sh` accepts it. If the
   policy refuses it, stop and report that as a blocker; do not work around it.
4. **Records.** The row's records carry `producer_cpu`, `producer_placement` = `"same_ccd"`, and the
   source set's underrun count. A record with any underrun is refused, never accepted. Update the
   runner's record count and `records_required` in the preflight for the new row.

## Non-goals

No engine change. No software prefetch, worker-placement policy, or cross-CCD row (successors, after
this row gives a number). No decode cost: the producer submits already-decoded PCM, as a browser or
mobile host does.

## Objective gates

1. **Same bits.** The live row's 64-block digest equals the ring row's. If the production source path
   legitimately differs (for example a first-block latency), pin the live row's own digest with the
   reason written in the test, and prove it equals the ring row's output shifted by that latency.
2. **No underrun.** A console-workload test renders 256 blocks with the producer thread live and
   asserts zero underruns and the digest.
3. **Existing rows unchanged.** Every other workload's digest and record shape are unchanged, and the
   render thread is still pinned to `bench_cpu` alone for them.
4. **Runner.** `scripts/operator/preflight-console-benchmark.sh` passes; a runner self-test (or dry
   run) shows the widened mask and the recorded `producer_cpu`, and shows that a host with no eligible
   producer core refuses the live row with a named reason rather than running it on one core.
5. fmt, clippy with `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`, `cargo test -p console-workload -p bench`.

## Console benchmark rows

Adds `sixty_four_track_plumbing_live`. No existing row may move a bit.

## Dependencies

Independent in code. Implement after "Build the console benchmark without graph test-support and
align its frozen buffers" so the new row starts from the corrected harness.

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Commit on `codex/<issue>-<slug>` from synchronized `main`.
- Benchmarks are descriptive. Do not run the timed runner; the coordinator runs it once after the
  batch. Do not tune or retry a timing.
