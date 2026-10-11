# Bound the source acquire by the blocks queued at its entry, and observe a late seek outside the loop

Stream B follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Root ruling
R1 (2026-10-05), on the second review of the realtime-policy tool drafts: the source acquire is a
render drain whose bound is a capacity, not the count at entry, so the realtime-policy gate refuses
it and it can chase its producer. The fix belongs to the stream that owns `crates/source`. Root
ruling R3 adds that a call to a popping function inside a loop in a marked region is refused, so
the acquire's seek re-observe must leave the loop.

No allocation, lock or syscall is added. The rendered PCM does not change.

## Problem (verified on `codex/d15-stream-b` at `b8392df66`)

This issue lands on the stream B batch-1 tree (#1309, #1343, #1314, #1311, #1348). Every line of
`crates/source/src/lib.rs` cited below is the same there and on `origin/main` at `6d28a80ec`
(batch 1 removes one line of the file's test module, at `:3723`, below every anchor here).

- **The loop.** `PcmSourceConsumer::acquire_current_block` (`crates/source/src/lib.rs:1389-1429`)
  runs on the render thread, from `begin_block_play` (`:1135-1136`) and from `prepare_seek`
  (`:1063`). It pops with `for _ in 0..self.transfer_block_count` (`:1393`, pop `:1394`).
  - `transfer_block_count` is the ring's capacity. A C ABI host's decode thread submits while
    render runs, so a block published during the loop is popped in the same call. Each pass
    discards a stale block, so a producer that keeps the ring fed can hold the render in this loop
    for up to `transfer_block_count` pops when only one block was queued at entry.
  - The gate refuses the form. On an `origin/main` export with markers in place of the blank lines
    `:1387` and `:1430`, `bash scripts/check-realtime-policy.sh` reports
    `marked realtime unbounded try_pop drain` at `:1394` (second review, BLOCKER-1).
- **The seek re-observe in the loop.** When a popped block has a generation that no observed seek
  covers, the loop calls `self.observe_seek_at_block_boundary(clock)` (`:1399-1407`), which pops
  the one-slot command queue (`:1324`). #1274 MINOR-3 added it: the producer pushes a seek's command
  before its PCM, so the command is queued by the time its block is popped. Under root's call rule
  (R3), a call to a function that pops, inside a loop in a marked region, is refused. The second
  review measured this site on `main` (`:1406`).
- **At most one re-observe is ever needed once the loop is bounded by the count at entry.**
  - A counted block was published before the count was read, and its seek command was pushed before
    it. So that command was pushed before the count too.
  - The command queue has one slot, and only `observe_seek_at_block_boundary` pops it. So at the
    count there is at most one command that was pushed and not yet popped. Every other counted
    block's command was popped earlier, and its generation is observed (`is_unobserved_generation`,
    `:1432-1437`, compares with the active generation and the held seek, and generations only
    grow).
  - Hence at most one generation among the counted blocks is unobserved, and the first time one is
    met, one command pop observes it. A second command can only be pushed after that pop, so its
    blocks were not counted.
- **Who already depends on this function.**
  - *Let a source consumer check and replay its next blocks for a prime* (C #1320 D3) relies on
    "`acquire_current_block` never pops past block `j`, so it never meets an unobserved generation
    and never re-observes a command (`:1400-1407`)". That stays true.
  - `prepare_seek`'s doc comment (`:1046-1048`, `:1064-1066`) describes the acquire's re-observe.
    It stays true.
  - The test `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm` (`:2286`) covers the
    re-observe, including an anchored seek whose anchor has passed, where the first block of the new
    generation is discarded and the second one plays (`:2319-2340`).
- `crates/source` has no `bench-support` dev-dependency (`crates/source/Cargo.toml`).

## Decisions

- **D1. The shape.** `acquire_current_block` becomes two counted phases with the re-observe
  between them, outside both loops:

  ```rust
  fn acquire_current_block(&mut self, clock: SeekClock) {
      if self.current.is_some() {
          return;
      }
      let available = self.data_consumer.available_at_entry().min(self.transfer_block_count);
      let mut popped = 0_usize;
      let mut unobserved = None;
      for _ in 0..available {
          let Ok(block) = self.data_consumer.try_pop() else {
              break;
          };
          popped += 1;
          // (the sanitized-sample max, as today)
          if self.is_unobserved_generation(block.generation) {
              unobserved = Some(block);
              break;
          }
          if self.settle_block(block) {
              return;
          }
      }
      let Some(block) = unobserved else {
          return;
      };
      // (comment: the one-slot argument in the Problem; at most one command pop per call)
      self.observe_seek_at_block_boundary(clock);
      if self.settle_block(block) {
          return;
      }
      let unpopped = available.saturating_sub(popped);
      let rest = self.data_consumer.available_at_entry().min(unpopped);
      for _ in 0..rest {
          let Ok(block) = self.data_consumer.try_pop() else {
              break;
          };
          // (the sanitized-sample max, as today)
          debug_assert!(
              !self.is_unobserved_generation(block.generation),
              "a block counted at entry has an observed seek after one re-observe"
          );
          if self.settle_block(block) {
              return;
          }
      }
  }
  ```

  - `settle_block(&mut self, block: Box<TransferBlock>) -> bool` is today's loop tail
    (`:1408-1427`), moved without change: keep the block as `current` and return `true` (an
    active-generation block at or after `next_frame`, or a held seek's first block), or
    `note_end_and_discard` it and return `false`. It holds no pop and calls nothing that pops.
  - **Where `settle_block` sits.** Directly after `acquire_current_block`, with one blank line
    between them, and before the blank line that today follows the acquire (`:1430`, above
    `is_unobserved_generation`'s doc comment). Tool slice B2b-1 (#1443) replaces the blank line before the
    acquire's doc comment (`:1387`) with `// REALTIME_POLICY_BEGIN` and that following blank line
    with `// REALTIME_POLICY_END`, so `settle_block` is in the acquire's region and the
    forbidden-body predicate covers it. The optional sanitized-sample helper (below) sits there
    too. Nothing else goes between the acquire and that blank line.
  - The two `let` lines are the count form that the drain rule accepts
    (`scripts/check-realtime-policy.sh:208-228` on `b8392df66`, `:206-226` on `main`; tool slices
    B1a-D6 and B1a-D8). `rest` never exceeds the blocks left from the first count, so the call
    pops at most `available` blocks.
  - The rewrite is equivalent to today's loop on every block counted at entry: phase 1 is the
    loop up to the first unobserved block, the re-observe is the same call with the same clock, and
    phase 2 is the loop after it. Only blocks published after the count are left for the next call.
  - A small helper for the sanitized-sample max is allowed if it holds no pop.
- **D2. Comments.** Update the comment at `:1400-1405` (now above the re-observe) and the
  "Bounded: at most one command pop per popped block" sentence to the D1 argument. `prepare_seek`'s
  doc comment stays as it is.
- **D3. Realtime rules.** The function allocates nothing, takes no lock and makes no syscall, as
  today. No `[]` index, `.expect(..)`, `.unwrap()` or panic macro is added; the `debug_assert!`
  compiles out of release builds.
- **D4. No region markers here.** Tool slice B2b-1 marks this function (with `settle_block`, D1),
  `observe_seek_at_block_boundary` and `PcmSourceProducer::take_recycled_block`. Write the
  function so that it passes `bash scripts/check-realtime-policy.sh` when it is marked (gate 3).
- **D5. A producer/render concurrency test** in `crates/source/src/lib.rs`'s `mod tests`, built on
  `bench_support::producer::render_while_producing`. Add `bench-support.workspace = true` to
  `[dev-dependencies]` in `crates/source/Cargo.toml`.
  - **Setup.** `PcmSourceRing::prepare(config(1, 4, 8))`, with a transfer-block capacity above 1.
    The producer is the `HostChunkProvider`. Set the consumer's `next_frame` (a private field) far
    past any frame the producer will write, so every block it pops is stale and is discarded.
  - **Producer.** `produce` submits one chunk only when the data queue is empty, so at most one
    block is queued at any time. `queued` is "the data queue holds a block".
  - **Each block** (`render`) records `stale_generation_discard_count`, calls
    `consumer.begin_block()`, and asserts that the count rose by at most 1. After the change a call
    pops at most the one block it counted. Today it pops again whenever the producer refills the
    queue between two passes of the loop.
  - Run 20,000 blocks. Record the run time.
- **D6. The re-observe phases stay tested.** The existing
  `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm` (`:2286`) is the case for phase 2:
  its anchored seek with a passed anchor discards the first new-generation block after the
  re-observe and plays the second. No new single-thread test is added (gate 4 shows why).

## Authorized paths

- `crates/source/src/lib.rs`: `acquire_current_block`, the new `settle_block` (and the optional
  sanitized-sample helper), the comments in D2, and the D5 test.
- `crates/source/Cargo.toml`: the `bench-support` dev-dependency.
- `Cargo.lock`: the one dependency line that this adds to the `source` package entry. A cross-stream
  exception: STREAMS gives `Cargo.lock` to C #1320.
- This spec

## Non-goals

- `observe_seek_at_block_boundary` and `take_recycled_block`: one pop each, in no loop. The drain
  rule accepts them as they are (second review, BLOCKER-1, and a measurement on `main`).
- Region markers (tool slice B2b-1).
- Any change to seek semantics, readiness or the prime (B #1316-#1350, C #1320, #1355).

## Hazards

- **Hot file.** `crates/source/src/lib.rs` is merged as STREAMS orders it. This issue goes first in
  that row after stream B batch 1; B (#1318, #1316, #1350, #1319, #1344) and C (#1320, #1355)
  rebase over it. Find each item by name.
- **C #1320 cites `:1400-1407`.** Those lines move into the re-observe between the phases. #1320's
  claim stays true; its implementer re-reads by name.
- **`Cargo.lock`.** C #1320 may add the same dev-dependency. The later slice rebases; the lock entry
  is identical.
- **The concurrency test depends on scheduling.** Gate 2 measures how often the old code fails; it
  does not tune the test to pass.

## Objective gates

1. **Unit tests.** `cargo test --locked -p source` passes, with every existing test and D5.
2. **Red on today's code (PR evidence).** Apply only D5's test (and the dev-dependency) to the
   parent commit, and run it ten times on a machine with at least two CPUs. Record the failure
   count. It must fail at least once. If it never fails on the parent, stop and report; do not
   tune it without root.
3. **The gate accepts the new shape (PR evidence).** On an export of this change, with markers in
   place of the blank lines around `acquire_current_block` (and, in a second run, around
   `observe_seek_at_block_boundary`), `bash scripts/check-realtime-policy.sh` prints
   `realtime policy: ok (..)`. Stream J's tool is not on `main` while this issue runs (its batch
   pushes once): B2b-1 marks and checks this function, and B2b-2's gate 3 holds the re-observe red
   if it moves back into the first loop.
4. **The phases are load-bearing (PR evidence).** Apply each mutation alone; the named test fails:
   - phase 2 removed (return after the re-observe's `settle_block`):
     `a_seek_whose_pcm_arrives_inside_the_block_window_keeps_its_pcm`;
   - the re-observe removed: the same test;
   - the bound reverted to `0..self.transfer_block_count`: D5 (on at least one of ten runs).
5. **Shipped artifacts.**
   - The C ABI: `target/release/audit capi` reports `allocations 0`, `deallocations 0`,
     `syscalls 0`, and the same `pcm_digest` as the parent.
   - The browser module (worklet chain gates): the batch's `qualification` run passes `artifact`,
     `artifact-identity`, `artifact-gates` and the three browser legs. `artifact-identity` reports
     ARTIFACT CHANGED, because the acquire is in the render closure. PR evidence: the function-level
     comparison of the base and head named twins, as the stream-J2 batch verdict made it. Only the
     acquire, its inlined copies and `core::panic::Location` line fields differ; the render
     closure's trap set does not change (`check-web-audioworklet.sh`'s call-graph gate passes).
   - **No PCM moved.** No fixture, pin or expected digest changes, and every rendered-digest gate
     passes with the base's pins (`check-browser-expected-resources.py --artifacts`, the browser
     legs' native-digest gates, `wasm-gates` G5 and G6, the C ABI audit's `pcm_digest`).
6. `cargo fmt --all -- --check`, `cargo clippy --locked -p source --all-targets -- -D warnings`,
   `bash scripts/check-realtime-policy.sh` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- D5 is red if the acquire pops a block published after its count, the producer-chase defect of a
  capacity bound. No existing test runs a producer during the acquire.
- The existing in-window seek test (D6) is red if the re-observe or phase 2 is dropped; gate 4
  shows it.

## Evidence

- Gates 1-6 output, gate 2's failure count, gate 4's runs, gate 5's comparison.

## Dependencies

- After (same stream): stream B batch 1 (#1309, #1343, #1314, #1311, #1348). Before #1318.
- After (other streams): none.
- Before (same stream): #1318, #1316, #1350, #1319, #1344 (hot file, they rebase).
- Before (other streams): C #1320 and #1355 (hot file); J tool slice B2b-1, which marks the
  function.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.
