# Give every plan a timeline that carries like a source

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2, A1.3), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R1.

## Product outcome

Every prepared plan, on both hosts, carries one timeline consumer (#1640): the session's
playhead. The graph's source driver advances it at every block entry, before the sources, from the
plan's source-read clock. A successor plan carries it, history included, exactly as it carries an
unchanged source: across a rebuild (#1273), a declared discontinuity (#1323) and a prime adoption
(#1320), and the seek report (#1316) has a row for it. Its producer sits in `SourceControlSet` and
moves with the persisting producers. Its fixed bytes are charged to the source overhead cap.

Nothing renders from it yet. The C call that seeks it is #1642, the browser export and the SDK
#1643 and #1644, and the first reader #1647. All are in batch R1.

## Context

- **The graph's source driver.** `SourceGraphSourceSetDriver` (`crates/source/src/lib.rs:1611-1617`)
  calls `consumer.begin_block_at(first_sample)` for each source at block entry
  (`:1701-1736`, the call `:1720-1721`). The graph executor calls the driver from its block entry
  (`crates/graph/src/lib.rs:3254-3256`). The source set is an `Option` on the executor
  (`:3254`), but host-core always builds one (`crates/host-core/src/prepare.rs:1635-1636`), and
  `prepare_graph_source_set` refuses an empty source list (`crates/source/src/lib.rs:1901-1903`),
  so every host-core plan has a source set.
- **How an unchanged source carries (#1273).** Preparation finds the predecessor's inventory row,
  allocates no ring and pushes a vacant entry (`crates/host-core/src/prepare.rs:1213-1240`). The
  carry program lists `(successor, predecessor)` source pairs (`crates/graph/src/lib.rs:2750-2771`)
  and is built only when something carries (`crates/host-core/src/prepare.rs:1294-1302`). At the
  swap block `adopt_predecessor` (`crates/graph/src/lib.rs:3139-3177`) calls the driver's
  `adopt_sources`, which swaps each consumer into its vacant entry with no allocation
  (`crates/source/src/lib.rs:1845-1888`). The host moves the producers first:
  `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:273-301`); the C ABI calls
  it at commit (`crates/capi/src/runtime/control.rs:1019`). `SourceControlSet` is the per-session
  producer table (`crates/host-core/src/source.rs:127-130`); its diagnostics are
  `SourceControlError::diagnostic` (`:78-103`).
- **Source bytes.** host-core checks the source set's overhead, plus the carried rings' overhead,
  against `maximum_source_overhead_bytes` (`crates/host-core/src/prepare.rs:1640-1652`).
- **Node arrival.** PDC computes each node's input arrival `max` in `timings`
  (`crates/graph-compiler/src/pdc.rs:53-104`). *Keep every node's latency from dropping during
  playback* (#1285) D2 records each node's floored `max` in the plan inventory. `A_max`, the
  largest of them, sizes the history (#1640 D3).
- **The mechanisms the timeline joins**, each filed and a dependency of this slice:
  - *Anchor every seek on the plan's source-read clock* (#1316): the per-source seek report;
  - *Give a plan a source-read clock that leads its render clock* (#1396): the sample the driver
    passes at block entry, and its step back at a discontinuity (#1396 D3);
  - *Reset latency floors at a host-declared discontinuity* (#1323): `SuccessorBase::discontinuity`
    and the carry of each consumer at it (#1323 D2-D3);
  - *Let a source consumer check and replay its next blocks for a prime* (#1320): the readiness
    check and the prime that a warm adoption runs on each carried consumer.
- **Readers of the plan.** A test reaches the executor through
  `PreparedRenderPlan::executor_any_mut` (`crates/engine/src/realtime/plan.rs:912`); graph exposes
  plan facts through free functions that downcast, for example `carry_program_retained_bytes`
  (`crates/graph/src/lib.rs:2911-2927`).

## Decisions frozen for this slice

- **D1. In every plan.** Host-core preparation builds one timeline consumer in every source set,
  with `K = ⌈A_max / q⌉ + 1` (`q` the plan's quantum, `A_max` the largest floored arrival of
  #1285 D2, 0 for a plan with no latency, so `K = 1`). The driver calls its `begin_block_at` at
  block entry, before the sources, with the source-read sample it passes the sources (#1396).
- **D2. The producer.** `SourceControlSet` owns one `TimelineProducer` beside the source producers,
  and `adopt_persisting` moves it like a persisting source's producer. `FrameOutOfRange` maps to
  `source.seek.frame_out_of_range` in `SourceControlError::diagnostic`; #1640 adds that arm,
  because host-core does not compile without it. Host-core gains no public
  timeline seek here; #1642 adds the session seek.
- **D3. Carry on a rebuild.** A successor carries the timeline exactly as an unchanged source, on
  every successor (it never changes):
  - preparation builds a vacant timeline entry that owns its own history of its own `K`, and no
    command queue;
  - the carry program gains `timeline: bool`, set on every successor, so a program exists even
    when no source or input lane carries;
  - at the swap block the driver's `adopt_sources` swaps the seek state, the command consumer and
    `position` into the vacant entry and calls `copy_history_from` (#1640 D3); nothing is
    allocated or freed;
  - a vacant timeline that is never adopted (a carry mismatch, unreachable on both hosts by
    #1269 P11) advances from 0 with no command queue, so render stays defined.
  - **Why the successor's own window is enough.** The successor reads the history only for its own
    nodes at its own arrivals. Floors (#1285) never lower a carried node's arrival, and a warm
    adoption raises both a carried node's floor and the source-read clock by `P` (README A1.3), so a
    carried node never reads further back than it did on the predecessor, and the predecessor's
    `K` covered that. Only a node that the successor adds, restarts or grows can read further
    back. Such a node starts at rest at adoption (#1324, #1397), so the samples it reads before its
    first input arrives label rest state, and the extrapolation of #1640 D3 gives them a
    defined, host-independent value that is exact unless a seek fell in the missing span. From the
    successor's `⌈A_max / q⌉`-th block on, its own entries fill its window.
- **D4. Carry at a declared discontinuity** (#1323). A discontinuity successor carries the timeline
  as D3 does. Its source-read clock steps back to the render clock (#1396 D3), so the history can
  hold two entries for one source-read sample; #1640's reader searches newest first, which is
  the entry render played last. The nodes that read the overlap start at rest (#1323 D2), so no
  continuity is owed.
- **D5. Prime** (#1320). The timeline answers #1320's readiness check as always ready (it has no
  PCM to run out of), and its prime advances it by `k` blocks, recording `k` history entries, as
  `k` calls of `begin_block_at` that observe no command. So a warm adoption (#1355) moves the
  timeline exactly as it moves the sources' frames.
- **D6. The seek report** (#1316). The report gains one row for the timeline, of the same form as
  a source's row (generation in effect, the source-read sample of the first block it played in,
  the timeline value that block started at); the underrun and held counts are 0.
- **D7. Bytes.** `prepare_graph_source_set` adds #1640's `TimelineResourceReport` to the source
  set's `overhead_bytes` and `largest_allocation_bytes`, so the source overhead cap bounds them on
  both hosts. A vacant (carried) timeline charges only its history; its carried command queue is
  counted with the carried source overhead, as a carried ring's is
  (`crates/host-core/src/prepare.rs:1220-1230`).
- **D8. Test reader.** `graph` cannot name a `crates/source` type (`source` depends on `graph`),
  so the source-set driver trait (`crates/graph/src/lib.rs:2161-2232`) gains one defaulted method,
  `timeline_at(&self, source_read_sample: u64) -> Option<i64>` (`None` by default), which the source
  driver implements with #1640's reader. `graph::test_only_timeline_at(plan: &mut
  PreparedRenderPlan, source_read_sample: u64) -> Option<i64>` (`test-support`) downcasts as
  `carry_program_retained_bytes` does and calls it. The render-side block reader comes with
  #1647.
- **D9. The acked-batch question.** A held seek moves with its consumer across every carry; no
  ack can precede a drop.

## Deliverables

1. D1, D3-D5 and D7 in `crates/host-core/src/prepare.rs` and in the source driver
   (`crates/source/src/lib.rs`).
2. D2 in `crates/host-core/src/source.rs`; D6 in the seek report; D8 and the carry flag in
   `crates/graph/src/lib.rs`.
3. Host-core tests in `crates/host-core/tests/timeline.rs` (new) and one case in
   `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/source/src/lib.rs` (the driver, the vacant entry, the prime and the report row only)
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/source.rs`,
  `crates/host-core/tests/timeline.rs` (new), `crates/host-core/tests/successor_swap.rs`
- `crates/graph/src/lib.rs`: `GraphCarryProgram`'s `timeline` field, the `adopt_predecessor`
  condition that calls `adopt_sources`, the driver trait's `timeline_at` and D8's test reader only
- the seek report's files as #1316 leaves them (the timeline row only)
- Tests that pin a source overhead or carry-program byte count that D7 or D3 moves (re-pin each
  with its reason, never in bulk)
- `crates/host-core/src/lib.rs` (a `test-support` re-export of `graph::test_only_timeline_at`
  only: host-web has no `graph` dependency, `hosts/host-web/Cargo.toml:25-33`, so its tests read
  the timeline through host-core) and `crates/host-core/Cargo.toml` (its `test-support` feature,
  `:16`, gains `graph/test-support`), `Cargo.lock`
- `hosts/host-web/tests/browser-v1/expected.json` (`resourceCeilings`) and
  `hosts/host-web/tests/retained_ceilings.rs`, only if D7's bytes cross the browser's
  `sourceOverheadBytes` ceiling (re-pin with the reason)

## Non-goals

- The C call (#1642), the browser export and SDK `seek` (#1643 and #1644), and any automation
  reader (#1637, #1646 and #1647).
- The protocol's transport state, which never reports the timeline (#1635 retires its stored
  position).
- A plan with no source set. Every host-core plan has one; the refusal of an empty source list
  (`crates/source/src/lib.rs:1901-1903`) is unchanged.

## Hazards

- **Order of the block entry.** The timeline must observe its command in the same block as a
  source given the same command. Both run in the one driver call at block entry; D1 puts the
  timeline first and applies no state to sources.
- **Byte rows move.** Every source overhead row grows by the timeline's bytes. Each pinned number
  that moves is re-pinned with the reason "the timeline consumer's fixed bytes (#1641 D7)".
- **Hot file.** `crates/host-core/src/prepare.rs` follows the merge order in STREAMS.md; this slice
  lands after every slice it depends on.

## Objective gates

1. **Source and timeline agree** (`crates/host-core/tests/timeline.rs`, new). A one-track session
   at 48 kHz, quantum 128, a playing source. Push the same seek, `(g, F, A)` anchored and then
   `(g + 1, F2)` plain, to the source and to the timeline (through `TimelineProducer` directly).
   For ten blocks after each, the frame each block's source read starts at equals
   `test_only_timeline_at` of that block's sample, and the seek report's timeline row equals the
   source's row in generation, sample and frame.
2. **Carried across a rebuild** (same file). Render 20 blocks, seek the timeline, render 5, then
   prepare and swap a successor (one added track). After the swap the timeline continues with no
   gap or repeat for 10 blocks, and `test_only_timeline_at` returns the predecessor's values for
   every sample in the successor's window. A successor with a latent insert (larger `A_max`) keeps
   the newest entries and extrapolates the rest. The producer moves: a seek pushed through the
   successor's `SourceControlSet` reaches the carried consumer.
3. **Carried at a discontinuity and a prime** (same file). A declared discontinuity followed by a
   session of sources and timeline seeks keeps the two in step for ten blocks; a warm adoption that
   primes `k = 2` blocks advances the timeline by `2q`, the same as the sources' frames.
4. **No allocation in render.** The swap block and the ten blocks after it allocate and free
   nothing (`bench_support::alloc` thread counters after warming, as
   `the_swap_block_allocates_and_frees_nothing`, `crates/host-core/tests/successor_swap.rs:476`).
5. **No rendered bit moves.** No render arithmetic changes. Every existing render test passes with
   unchanged digests; `./target/release/audit capi` shows the same `pcm_digest` at base and head
   (PR evidence) and 0 violations.
6. **Commands:**
   - `cargo test --locked -p source`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule),
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the timeline observes a command one block before or after a source given the
  same command (the fault that would put every automation curve one quantum off its audio), or if
  the seek report's row disagrees with the sources.
- Gate 2 turns red if a successor restarts the timeline at 0, drops its history or its producer, or
  sizes its history from the predecessor's `K`.
- Gate 3 turns red if a discontinuity or a prime moves the sources and not the timeline.
- Gate 4 turns red if the carry or the history write allocates on the render thread.

## Dependencies

- #1640 *Build the timeline consumer and producer in the source crate*.
- *Keep every node's latency from dropping during playback* (#1285): D2's recorded floored
  arrivals give `A_max` (D1).
- *Anchor every seek on the plan's source-read clock* (#1316): the seek report (D6).
- *Reset latency floors at a host-declared discontinuity* (#1323): the discontinuity successor
  (D4).
- *Give a plan a source-read clock that leads its render clock* (#1396): the sample at block entry
  (D1, D4).
- *Let a source consumer check and replay its next blocks for a prime* (#1320): readiness and prime
  (D5).
- Batch: R1, with #1647 *Render moving stored fader automation, seeks and latency*, its first
  reader. #1642, #1643 and #1644 build on it.

None of these specs is amended for the timeline: each lands before this slice, and this slice adds
the timeline's line to the mechanism each one built.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Moved files.** The C ABI's commit-time `adopt_persisting` call is in `crates/control-plane/src/control.rs`
  (#1309); `crates/capi/src/runtime/control.rs` is gone.
- **The browser** runs the same `SessionState` (#1562a, single instance), so D1-D7 reach it by
  construction; its successor (carry) path is refused `web.apply.rebuild_pending` until #1562b, so
  the browser half of gate 2 waits for #1562b.
- **Realtime policy** as in #1640's amendment (`./target/release/realtime_policy`).
- **Dependencies.** #1285, #1316, #1320, #1323 and #1396 are still open.

## Amendment (controls coordinator, batch 16 check, 2026-10-16)

Checked against `35ac17545`.

- **Landed deps.** #1285, #1309, #1316, #1318, #1320, #1323, #1396 and #1562 (with #1562b) are
  closed and on `main`. #1355 (warm adoption) and #1647 are open.
- **The browser runs successors now** (#1562b-W, `2b79cec44`), through the same control plane,
  each one counted as a blocking rebuild. The note "the browser half of gate 2 waits for #1562b"
  is void.
- **Moved anchors, `crates/source/src/lib.rs`.** `SourceGraphSource` `:1978-2021` (`vacant`
  `:2013`). `GraphSourceEntry` `:2041-2046`. Driver `SourceGraphSourceSetDriver` `:2048-2054`.
  Its `begin_block` `:2138-2174` (the call `:2159`). `adopt_sources` `:2289-2325`.
  `prepare_graph_source_set` `:2421-2497`.
- **Moved anchors, `crates/graph/src/lib.rs`.** Driver trait `:2565-2667`. `GraphCarryProgram`
  `:3310-3324`. `install_carry_program` `:3699`. `carry_program_retained_bytes` `:4349`.
  `adopt_predecessor` `:4728-4830`; it calls `adopt_sources` only `if !program.sources.is_empty()`
  (`:4794-4803`), so D3's flag goes into that condition. Block entry: `:4964-4978`, which reads
  `time.absolute_sample + source_read_offset` (#1396 D1). `PreparedRenderPlan::executor_any_mut`
  is `crates/engine/src/realtime/plan.rs:1252`, `executor_any` `:1261`.
- **Moved anchors, `crates/host-core/src/prepare.rs`.** Source loop with the carry `:2234-2295`
  (vacant push `:2273`, carried overhead `:2253-2263`). Region `0..frames`: `builder.push`
  `:2264-2271` and `:2281-2288`. Carry program `:2441-2457`. Floored arrivals
  `artifact.report().input_arrivals` `:2846-2851`. `prepare_graph_source_set` and the overhead cap
  `:2916-2932`. Offset set `:3439-3440`. `at_discontinuity` `:1032-1056`.
- **Moved anchors, host-core and control plane.** `SourceControlSet` `crates/host-core/src/source.rs:137-140`,
  `adopt_persisting` `:321-336`, `diagnostic` `:86-118`. The control plane calls `adopt_persisting`
  at three places: `crates/control-plane/src/control.rs:2705` (across a withdrawn candidate),
  `:2732` (an ordinary commit) and `:2816` (#1310 D5 step 5, the hand-back to P0). D2's timeline
  producer moves at all three, with no new call.
- **`A_max` is readable in time.** `input_arrivals` (floored, nonzero rows only) exists before
  `prepare_graph_source_set` is called. So K is `⌈max / q⌉ + 1`, or 1 when it is empty. The
  inventory keeps the same rows (`PlanStateInventory::input_floors()`, `:1000`).
- **Stale: the empty-source refusal.** `prepare_graph_source_set` no longer refuses an empty source
  list. Host-core still always builds a source set (`:2916`), so a session with no sources also
  gets a timeline. The Non-goal "a plan with no source set" stands.
- **Discontinuity (#1323).** `at_discontinuity` keeps only the source rows and sets
  `source_read_offset` to 0, so the step back is #1396 D3 as written. D4 holds. The carry program
  exists at a discontinuity only when something carries; D3's flag makes it exist on every
  successor.
- **Donation (#1344).** `swap_sources` and `donate_sources` (`graph` `:4431-4475`) exchange
  *fresh* entries between two unpublished plans. Every control-plane candidate is a successor, so
  its timeline is vacant and is never donated. The driver's `swap_sources` leaves the timeline
  where it is.
- **D3 also moves the seek report.** If D6 stands, the adopt step also swaps the timeline's
  seek-report writer and its last published copy, as a source's report moves with its consumer
  (#1316 D5).
- **D6 is per ring, not a row in a table.** There is no shared report table. Each ring has a
  5-word seqlock record (`crates/engine/src/realtime/seqlock.rs`). Host-core reads it per source
  ID (`SourceControlSet::seek_report`, `:258`); the C ABI reads it with
  `miso_engine_v1_source_seek_report` (`crates/capi/src/ffi.rs:652`). The timeline's "row" is its
  own record of the same words. #1642 lists it as a non-goal, so no host reads it after this
  slice.
- **D5: no prime path exists above the consumer.** #1320 shipped `PcmSourceConsumer::prime_ready`
  (`:1676`) and `prime_block_at` (`:1742`) and the `prime_required` flag. `GraphExecutor` does not
  implement `PreparedPlanExecutor::prime_ready` (the default `false`,
  `crates/engine/src/realtime/plan.rs:356`), and the driver has no prime call. #1355 adds them,
  and its spec does not name the timeline yet. Until then, gate 3's prime half can only compare
  the two consumers directly.
- **D7 byte pins (measured on `35ac17545`, x86-64).** `hosts/host-web/tests/retained_ceilings.rs`
  `source_overhead_bytes` is 2,658 of 2,688, and
  `crates/capi/tests/resource_lifecycle.rs` is 4,174 of 4,352. The timeline's bytes will very
  likely cross both ceilings. The browser's wasm `sourceOverheadBytes` ceiling (2,496,
  `hosts/host-web/tests/browser-v1/expected.json:73`) is likely to cross too. Re-pin each one with
  the reason. Keep the identity `source_overhead + carried_source_overhead == fresh`
  (`crates/host-core/tests/successor_swap.rs:197-200`).
- **Gate 1 hazard, plain seeks pushed while render runs.** One driver call cannot put a timeline
  and a source in the same block on its own. The two queues are popped at different instants, so
  a plain seek pushed during a block can land one block apart. Gate 1 pushes between blocks, so
  it is deterministic. Only anchored seeks are exact under concurrency (#1642's concern).
- **Realtime policy.** The floors are now `tools/realtime-policy/src/policy.rs:228-229`. New
  regions only add to them.
- **Decision 17.** No change. C13 (rulings 2026-10-15 and 2026-10-16) calls seeks and the seek
  report transport. Exposing a timeline report to a host would need a new direct export or a
  protocol message, and a new direct export needs its own root ruling.
