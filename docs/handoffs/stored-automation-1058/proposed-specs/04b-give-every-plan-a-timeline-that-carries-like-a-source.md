# Give every plan a timeline that carries like a source

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2, A1.3), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R1.

## Product outcome

Every prepared plan, on both hosts, carries one timeline consumer (draft 04a): the session's
playhead. The graph's source driver advances it at every block entry, before the sources, from the
plan's source-read clock. A successor plan carries it, history included, exactly as it carries an
unchanged source: across a rebuild (#1273), a declared discontinuity (#1323) and a prime adoption
(#1320), and the seek report (#1316) has a row for it. Its producer sits in `SourceControlSet` and
moves with the persisting producers. Its fixed bytes are charged to the source overhead cap.

Nothing renders from it yet. The C call that seeks it is draft 05, the browser export and the SDK
drafts 06a and 06b, and the first reader draft 09b. All are in batch R1.

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
  largest of them, sizes the history (draft 04a D3).
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
  `source.seek.frame_out_of_range` in `SourceControlError::diagnostic`; draft 04a adds that arm,
  because host-core does not compile without it. Host-core gains no public
  timeline seek here; draft 05 adds the session seek.
- **D3. Carry on a rebuild.** A successor carries the timeline exactly as an unchanged source, on
  every successor (it never changes):
  - preparation builds a vacant timeline entry that owns its own history of its own `K`, and no
    command queue;
  - the carry program gains `timeline: bool`, set on every successor, so a program exists even
    when no source or input lane carries;
  - at the swap block the driver's `adopt_sources` swaps the seek state, the command consumer and
    `position` into the vacant entry and calls `copy_history_from` (draft 04a D3); nothing is
    allocated or freed;
  - a vacant timeline that is never adopted (a carry mismatch, unreachable on both hosts by
    #1269 P11) advances from 0 with no command queue, so render stays defined.
  - **Why the successor's own window is enough.** The successor reads the history only for its own
    nodes at its own arrivals. Floors (#1285) never lower a carried node's arrival, and a warm
    adoption raises both a carried node's floor and the source-read clock by `P` (README A1.3), so a
    carried node never reads further back than it did on the predecessor, and the predecessor's
    `K` covered that. Only a node that the successor adds, restarts or grows can read further
    back. Such a node starts at rest at adoption (#1324, #1397), so the samples it reads before its
    first input arrives label rest state, and the extrapolation of draft 04a D3 gives them a
    defined, host-independent value that is exact unless a seek fell in the missing span. From the
    successor's `⌈A_max / q⌉`-th block on, its own entries fill its window.
- **D4. Carry at a declared discontinuity** (#1323). A discontinuity successor carries the timeline
  as D3 does. Its source-read clock steps back to the render clock (#1396 D3), so the history can
  hold two entries for one source-read sample; draft 04a's reader searches newest first, which is
  the entry render played last. The nodes that read the overlap start at rest (#1323 D2), so no
  continuity is owed.
- **D5. Prime** (#1320). The timeline answers #1320's readiness check as always ready (it has no
  PCM to run out of), and its prime advances it by `k` blocks, recording `k` history entries, as
  `k` calls of `begin_block_at` that observe no command. So a warm adoption (#1355) moves the
  timeline exactly as it moves the sources' frames.
- **D6. The seek report** (#1316). The report gains one row for the timeline, of the same form as
  a source's row (generation in effect, the source-read sample of the first block it played in,
  the timeline value that block started at); the underrun and held counts are 0.
- **D7. Bytes.** `prepare_graph_source_set` adds draft 04a's `TimelineResourceReport` to the source
  set's `overhead_bytes` and `largest_allocation_bytes`, so the source overhead cap bounds them on
  both hosts. A vacant (carried) timeline charges only its history; its carried command queue is
  counted with the carried source overhead, as a carried ring's is
  (`crates/host-core/src/prepare.rs:1220-1230`).
- **D8. Test reader.** `graph` cannot name a `crates/source` type (`source` depends on `graph`),
  so the source-set driver trait (`crates/graph/src/lib.rs:2161-2232`) gains one defaulted method,
  `timeline_at(&self, source_read_sample: u64) -> Option<i64>` (`None` by default), which the source
  driver implements with draft 04a's reader. `graph::test_only_timeline_at(plan: &mut
  PreparedRenderPlan, source_read_sample: u64) -> Option<i64>` (`test-support`) downcasts as
  `carry_program_retained_bytes` does and calls it. The render-side block reader comes with
  draft 09b.
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

- The C call (draft 05), the browser export and SDK `seek` (drafts 06a and 06b), and any automation
  reader (drafts 07, 09a and 09b).
- The protocol's transport state, which never reports the timeline (draft 25 retires its stored
  position).
- A plan with no source set. Every host-core plan has one; the refusal of an empty source list
  (`crates/source/src/lib.rs:1901-1903`) is unchanged.

## Hazards

- **Order of the block entry.** The timeline must observe its command in the same block as a
  source given the same command. Both run in the one driver call at block entry; D1 puts the
  timeline first and applies no state to sources.
- **Byte rows move.** Every source overhead row grows by the timeline's bytes. Each pinned number
  that moves is re-pinned with the reason "the timeline consumer's fixed bytes (draft 04b D7)".
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
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh`,
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

- Draft 04a *Build the timeline consumer and producer in the source crate*.
- *Keep every node's latency from dropping during playback* (#1285): D2's recorded floored
  arrivals give `A_max` (D1).
- *Anchor every seek on the plan's source-read clock* (#1316): the seek report (D6).
- *Reset latency floors at a host-declared discontinuity* (#1323): the discontinuity successor
  (D4).
- *Give a plan a source-read clock that leads its render clock* (#1396): the sample at block entry
  (D1, D4).
- *Let a source consumer check and replay its next blocks for a prime* (#1320): readiness and prime
  (D5).
- Batch: R1, with draft 09b *Render moving stored fader automation, seeks and latency*, its first
  reader. Drafts 05, 06a and 06b build on it.

None of these specs is amended for the timeline: each lands before this slice, and this slice adds
the timeline's line to the mechanism each one built.
