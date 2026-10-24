# Add a live-producer ring row to the console benchmark

**Ruled** (coordinator, 2026-09-26): finding 1 of the adversarial verification of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md` (`DIAGNOSIS-2-VERIFY.md`, section 5).
Benchmark tooling only; no engine source changes. **Re-based** (coordinator, 2026-09-27, issue
comments): #956 retired the plumbing session, so the row is `sixty_four_track_gain_pan_live` on the
gain/pan session, and its 64-block digest must equal `sixty_four_track_gain_pan_ring`'s. After #965
every row uses the production feed; this row stays the one that measures a producer on another core.

## Product outcome

`sixty_four_track_gain_pan_ring` feeds the graph from a `FrozenSourceDriver` (a console-workload
shim, not the source crate's ring) that lends the same frozen block every block. That block stays
warm in the render core's cache, so the row never pays for input written by another core. In a
product session a producer (the browser's PCM pump in a Worker, a mobile decode thread) writes every
source's ring from a different core. The verification's cross-core probe (64 rings, a producer
writing 1 KiB per source per block) measured the render-side read of those blocks:

| producer | read of 64 x 1 KiB per block |
|---|---:|
| none (frozen, like the ring row) | 2,261 cycles |
| SMT sibling (shared L2) | 3,152 |
| same CCD, other core | 5,894-6,416 |
| other CCD | 20,572-20,979 |

No row measures this term. Add one row that renders the gain/pan session from the production
source path with a live producer thread on another core of the same CCD.

## Split (root, 2026-10-24)

- **938a (this tranche):** the row, the producer thread and in-bench thread pinning.
- **938b (successor):** the runner picks `producer_cpu`, a core that shares `bench_cpu`'s L3
  (`cache/index3/shared_cpu_list`) and is not its SMT sibling (`topology/thread_siblings_list`),
  widens the `taskset` mask to `bench_cpu,producer_cpu`, refuses the run with a named reason when no
  eligible core exists (never one core), and its preflight dry run shows the mask and
  `producer_cpu`.

## 938a slice

Paths: `tools/console-workload/` (`src/lib.rs`, `Cargo.toml`), `tools/bench/` (`src/console.rs`,
`src/floor.rs`, `Cargo.toml`), `Cargo.lock`, `scripts/console-benchmark-record-lib.jq`,
`scripts/console-benchmark-validator.jq`, `scripts/operator/run-console-benchmark.sh` and
`scripts/operator/preflight-console-benchmark.sh` (record count only),
`scripts/test-console-benchmark.sh`, this spec.

1. **The workload.** `SixtyFourTrackGainPanLive` (`sixty_four_track_gain_pan_live`,
   `source_feed` = `live_producer`) compiles the ring row's session and binds the production source
   set: `source::prepare_session_graph_source_set` over one `PcmSourceRing::prepare_host_region`
   ring per track (two channels, `LIVE_RING_FRAMES` = the host default of 40 quanta), a
   `SourceTimeline`, one published generation, built as `crates/host-core/src/prepare.rs` builds a
   session's.
2. **The producer.** One thread, started before warmup and joined after the last measured block,
   submits each track's frozen block through `HostChunkProvider::submit`, tops every ring up, and
   sleeps 100 us when all are full. It allocates nothing after it starts. Outside the clock the
   driver waits until every ring holds the next block plus `LIVE_LEAD_BLOCKS` (2). The timed
   region is `render` alone.
3. **Pinning.** The bench pins its render thread (every row) to `MISO_ENGINE_BENCH_CPU_AFFINITY`
   and the producer to the one other core of the process mask, then classifies the producer's
   placement from sysfs. Linux only, behind `cfg(target_os = "linux")` in `tools/bench`.
4. **Records.** The row's records carry `producer_cpu`, `producer_placement` and `underruns`. Any
   underrun is refused. A controlled record needs `same_ccd` and a core other than `cpu_affinity`;
   an uncontrolled one may state `unpinned` with no core. Record count 72 -> 74, session 44 -> 46.

## Objective gates

1. **Same bits.** The live row's digest equals the ring row's: console-workload test over 256
   blocks, the bench's in-run `LIVE_PAIR` assertion, and the aggregate validator.
2. **No underrun.** The same test asserts zero underrun frames with the producer live.
3. **Existing rows unchanged.** Every other row's digest and record shape are unchanged.
4. `cargo test --release -p console-workload -p bench`, `scripts/test-console-benchmark.sh`, the
   preflight, `check-console-benchmark-fixture.sh`, `check-bench-policy.sh` and its test, fmt,
   clippy `-D warnings`. No timed run by the implementer.

## Non-goals

No engine change. No software prefetch, worker-placement policy, or cross-CCD row. No decode cost:
the producer submits already-decoded PCM, as a browser or mobile host does.

## Decision record (938a attempt 1, 2026-10-24)

- **rustix, not libc** (deviation from root's "libc"): `libc::sched_setaffinity` is an `unsafe fn`,
  and `tools/bench` is not an approved unsafe owner (`check-bench-policy.sh`'s owner set and
  `tools/realtime-policy`'s `UNSAFE_ALLOWLIST`, both outside this slice). `rustix` 1.1.4 is already
  in `Cargo.lock` and its `thread` feature wraps the same syscalls safely, so the bench owns no
  `unsafe`. Linux-only target dependency of `tools/bench`; the workspace policy accepts it.
  `scripts/check-conformance-boundaries.sh` pins the bench's dependency set, so it and its fixture
  in `scripts/test-conformance-boundaries.sh` gain `rustix` (outside the scope note's list; any new
  bench dependency, libc included, forces the same edit).
- **`LIVE_FED_WORKLOADS`, not `DRIVER_FED_WORKLOADS`**: the live row renders its input only while
  its producer runs, and `tests/chain_shape.rs` iterates the driver-fed rows without one. Emission
  order is the same (right after the ring row). Its plan is the ring row's.
- **No new env name**: the producer core is the process mask less the render core, so 938b's
  runner hands it over by widening `taskset`. A controlled run whose mask holds no second core is
  refused by the bench before timing (`no_producer_cpu`). Until 938b lands, every controlled runner
  invocation refuses: the timed run waits for 938b.
- **Floor**: the live row states the ring row's identity floor and names no control (the two rows
  share one floor). `tools/bench/src/floor.rs` was edited for this (exhaustive match).
- **Ring depth**: `LIVE_RING_FRAMES` mirrors `host_core::default_source_ring_frames(48_000, 128)`;
  the live test pins the two equal (`host-core` is a console-workload dev-dependency).
- Fact: no first-block latency. Timeline and rings start at frame zero and the plan has no input
  arrival, so the live digest equals the ring digest unshifted.

## Console benchmark rows

Adds `sixty_four_track_gain_pan_live`. No existing row may move a bit.
