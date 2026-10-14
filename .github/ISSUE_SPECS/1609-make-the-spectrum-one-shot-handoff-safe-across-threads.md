# Make the spectrum one-shot handoff safe across threads

Successor of #1591, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `5415352a9` (PR #1608).
Issue number: assigned at filing (written here as #1609). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review. The realtime gates below are mandatory
all the same.

## Product outcome

A one-shot spectrum capture gives the right answer when render and the reader run on two
threads. A cancel is final: a window armed before a cancel is never returned after it, and the
next arm is never refused or failed because of it. A window is returned only after the block that
completed it rendered without failure. Render stays lock-free, allocation-free and bounded, with
no retry loop. The C ABI, browser and wire behaviour on one thread does not change.

## Context

- **The state word.** One `Arc<AtomicU8>` per capture holds `IDLE`, `ARMED`, `CAPTURING`,
  `COMPLETE` or `INVALID` (`crates/host-core/src/spectrum.rs:32-36`; shared at `:1438`). The
  result queue has one slot (`SPECTRUM_RESULT_SLOTS`, `:43`; built at `:1433-1437`).
- **Control.** `arm` is `compare_exchange(IDLE, ARMED)` (`:527-535`). `cancel` drains the queue
  and then `reset_after_cancel` does an unconditional `store(IDLE)` (`:538-564`, the store at
  `:563`); the collection's `cancel_except` does the same (`:947-968`). `try_read_record` loads the
  word, pops on `COMPLETE` and stores `IDLE` (`:573-597`, stores at `:582` and `:592`).
- **Render.** `one_shot_begin` moves `ARMED -> CAPTURING` by compare-and-swap (`:1654-1703`, CAS
  at `:1664`). Every other render transition is an unconditional store: `one_shot_finish` pushes
  the record and then `store(COMPLETE)` (`:1705-1744`, push at `:1736`, store at `:1738`);
  `one_shot_invalidate` stores `INVALID` (`:1850-1855`, `:1853`);
  `one_shot_invalidate_after_failure` moves `COMPLETE -> INVALID` when the failed block is the one
  that completed the window (`:1857-1865`, `:1863`).
- **Race 1, cancel against completion.** Render loads `CAPTURING`; control cancels (drain, then
  `IDLE`); render pushes and stores `COMPLETE`. The cancelled window comes back as `COMPLETE`, and
  the next `arm` is refused `Busy`. If render's store lands first and the push after the drain,
  the word is `IDLE` but a stale record holds the only slot, so the next arm's window cannot be
  pushed and its read is `Invalid`.
- **Race 2, a read inside a failing block.** Render publishes `COMPLETE` in the middle of a block
  and the graph calls `invalidate_after_failure` later in the same render call
  (`crates/graph/src/runtime.rs:3354-3373`; trait default `crates/graph/src/lib.rs:2846-2853`). A
  reader on another thread can take the window between the two, so a window from a failed block
  is returned.
- **Why it is latent.** Today render and the reader run on one thread (the browser worklet, and
  the C ABI's reads between render calls). #1593, #1595 and #1596 move spectrum reads to a control
  thread, which makes both races real.
- **Loom.** `engine` runs its SPSC under loom (`crates/engine/Cargo.toml`, dev-dependency
  `loom = "=0.7.2"`; the `cfg(loom)` shim at `crates/engine/src/realtime/spsc.rs:10-60`; CI step
  `.github/workflows/qualification.yml:734-735`). `host-core` has no loom dependency.
- **Existing concurrency tests.** `racing_capture` and the drain tests
  (`spectrum.rs:3255-3330`) race a producer against the control-side drain only.

## Decisions frozen for this slice

- **D1. Option (b): the arm generation, in the state word.** The one-shot state becomes one
  `AtomicU64`: the phase in bits 0-2 and the arm generation in bits 3-63. Each record carries the
  generation it was captured under (`SpectrumCapturedRecord.arm_generation`, an internal name).
  **Why (b) and not (a).** A compare-and-swap from `CAPTURING` alone (option a) keeps the word
  right, but it cannot fix the record: render must push before it publishes, so a cancel between
  the push and the CAS leaves a record in the queue, and the reader cannot tell that stale window
  from the next arm's window. The generation lets the reader discard it, and putting the
  generation in the same word as the phase makes each render transition one CAS that checks both,
  with no second atomic and no ordering between two words. 61 bits of generation cannot wrap.
- **D2. Control transitions.** Only `arm` changes the generation: `(g, IDLE) -> (g + 1, ARMED)`
  by one CAS (refused `Busy` otherwise, as today). `cancel` and `cancel_except` drain as today and
  then store `(g, IDLE)` with the current generation. `arm` keeps `&self`.
- **D3. Render transitions.** Render latches the full word at `ARMED -> CAPTURING`. Every later
  render transition is exactly one strong `compare_exchange` from the word render last loaded or
  latched: `CAPTURING -> COMPLETE`, `ARMED | CAPTURING -> INVALID`. Render never stores the word
  unconditionally. A failed CAS means control moved the word (cancel or re-arm): render abandons
  the window and does not retry. Success ordering `AcqRel`, failure `Acquire`; the push happens
  before the `COMPLETE` CAS.
- **D4. Publication at block end (race 2).** `GraphRuntimeObserver` gains
  `fn accepts_block_end(&self) -> bool` (default `false`, read once at bind, like
  `accepts_sample_peak`) and `fn block_end(&mut self)` (default empty). The graph runtime keeps a
  bind-time, preallocated list of the observers that accept it and calls `block_end` on them, in
  binding order, after a block rendered without failure; no other observer pays anything. The
  spectrum observer accepts it in one-shot mode: a window completed in a block is held in the
  observer's own buffers and is pushed and moved to `COMPLETE` only in that block's `block_end`. A
  failed block calls `invalidate_after_failure` and never `block_end`, so a held window is dropped
  with an `INVALID` CAS, and the `COMPLETE -> INVALID` path is deleted.
- **D5. A stale record never fails the next window.** A queue-full push in `block_end` is not an
  invalidation: the slot holds a stale record (D1). Render keeps the held window and the word
  `CAPTURING` and makes one `try_push` per later `block_end` until it succeeds or the word changes
  (one attempt per block, so bounded). The reader, in `ARMED`, `CAPTURING`, `COMPLETE` or
  `INVALID`, pops at most `SPECTRUM_RESULT_SLOTS` records and discards each whose generation is not
  the word's. A record of the current generation is the result: the reader moves the word to
  `(g, IDLE)` by CAS from the word it loaded and, if that fails because render moved
  `CAPTURING -> COMPLETE`, once more from `COMPLETE` (at most two CAS, no loop). A word that is
  `INVALID` answers `Invalid` and moves to `(g, IDLE)` as today.
- **D6. One thread is unchanged.** On one thread every C ABI and browser result, refusal and
  count is the same as today, including a read straight after the render call that completes a
  window (`block_end` runs inside that call). No wire, header or export changes.
- **D7. Continuous mode is not in this slice.** The continuous phase, epochs and mode word keep
  their protocol; the mode switch is not changed.

## Deliverables

1. D1-D3, D5 and D6 in `crates/host-core/src/spectrum.rs`.
2. D4 in `crates/graph/src/lib.rs` (the trait) and `crates/graph/src/runtime.rs` (the list and
   the call), and the spectrum observer's two methods.
3. The race test and the loom model below; `loom = "=0.7.2"` as a `host-core` dev-dependency
   (already in `Cargo.lock`) and one CI step next to the engine loom leg.
4. Every new or changed render function inside `REALTIME_POLICY_BEGIN`/`END` markers.
5. The decision record: the race test's seed count and run time, and the loom model's result.

## Authorized paths

- `crates/host-core/src/spectrum.rs`, `crates/host-core/Cargo.toml` (the dev-dependency only).
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (D4 only).
- `.github/workflows/qualification.yml` (one step after `:735`, same job).
- `docs/REALTIME_DEPENDENCY_POLICY.md` (one sentence for the new observer hook, if the policy
  lists observer calls).
- This spec's decision record.

## Non-goals

- No change to continuous capture (D7), to the spectrum service's API or to analysis.
- No change to the queue depth, the record size or `spectrum_capture_resources`.
- No reader on a second thread in a host: #1593, #1595 and #1596 do that.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`, `cargo test --locked -p graph`,
`cargo test --locked -p capi`, `cargo test --locked -p host-web`,
`CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p host-core --lib spectrum_handoff_loom`,
`cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`,
`bash scripts/check-host-core-policy.sh`.

1. **Two-thread race (cancel against completion).** A host-core test runs N = 1,000 seeds. Per
   seed, a render thread drives the real observer through blocks that complete one window, and a
   control thread arms, cancels at a seeded point, re-arms and reads until it has a result. It
   asserts: no read after a cancel returns a window of the cancelled generation (its
   `first_sample` is the cancelled arm's); the re-arm is never `Busy`; the re-armed window is
   returned, never `Invalid`, after at most two further blocks. The thread schedule is varied by
   the seed (spin counts), not by sleeps.
2. **Race 2.** A host-core test fails the block that completes a window (an injected
   `invalidate_after_failure` at its first sample, with no `block_end`): the read is `Invalid` and
   never the window. A second block that completes cleanly and calls `block_end` gives the window.
3. **Loom model.** `spectrum_handoff_loom` runs, under `loom::model`, the real state word and the
   real `engine` SPSC (built under `--cfg loom`) with three steps on two threads: render completes
   and publishes one window; control cancels, re-arms and reads. Every interleaving satisfies gate
   1's invariants. If `host-core`'s library does not build under `--cfg loom`, the model runs over
   the extracted handoff functions (the word transitions and the push and pop), and the decision
   record says which.
4. **One thread unchanged.** Every existing spectrum test in `host-core`, `capi` and `host-web`
   stays green unchanged, including the single-thread read-after-render tests.
5. **Realtime policy.** `realtime_policy` passes with the new and changed render functions
   (`block_end`, the CAS transitions, the held-window push) inside markers; the existing render
   allocation counters stay at exactly zero.

## Test value

- Gate 1 turns red if any render transition is still an unconditional store, if `cancel` does
  not leave a generation the stale record can be told from, or if a stale record fails the next
  window.
- Gate 2 turns red if a window is published before its block is known to be good.
- Gate 3 turns red on an interleaving the seeded test does not reach, such as a cancel between
  the push and the CAS.
- Gate 4 turns red if the fix changes a single-thread result.

## Dependencies

- #1591 (the spectrum service, in batch 6, PR #1608).
- Dependency of #1593, #1595 and #1596: each must land after this slice.

## Size

One implementer-day, one sonnet-high implementer.

## Decision record: spectrum one-shot handoff (batch 8)

**Shipped.** D1-D6 as frozen. The one-shot state is one `AtomicU64` (phase in bits 0-2, arm
generation in bits 3-63); `arm` is the only generation change; every render transition is one
strong compare-and-swap from the word render last loaded or latched; a window completed in a block
is held in the observer's buffers and pushed and moved to `COMPLETE` only in that block's
`block_end` (new `GraphRuntimeObserver::accepts_block_end` / `block_end`, default off; the runtime
calls only the units found at bind to hold an accepting observer). The reader discards records of
another generation. A queue-full push in `block_end` keeps the window held, one attempt per block.
D7 untouched.

**Gates and results.**
- Gate 1, `spectrum::tests::a_cancel_racing_completion_never_returns_the_cancelled_window_over_1000_seeds`:
  1,000 seeds, about 0.53 s debug and 0.14 s release; run eight times in a row green. Seeded spin
  counts (log-uniform 0-2,047 before the cancel, 0-511 after), no sleeps, whole-window blocks so a
  window begins and completes in one render call. It was red on a mutated `block_end` (unconditional
  store, no word check).
- Gate 2, `a_window_completed_by_a_failed_block_is_never_returned`, plus
  `a_later_failed_block_preserves_an_older_published_window` and
  `a_stale_record_delays_but_never_fails_the_next_window` (D5).
- Gate 3, `spectrum::tests::spectrum_handoff_loom::spectrum_handoff_loom`: **host-core's library
  builds under `--cfg loom`**, so the model runs the real observer, the real `SpectrumCapture`, the
  real state word (loom's `AtomicU64`) and the real `engine` SPSC, not extracted functions. Render
  runs three blocks; control cancels, reads, re-arms and reads. It passes in about 3 s. Loom found
  one real defect while this was written: the reader treated every record read in `ARMED` as stale,
  but render can latch and complete the window between the reader's word load and its pop (see
  departure 3). It is also red on the mutated `block_end` above.
- Gate 4: every existing spectrum test in host-core, capi and host-web is green; see departure 7
  for the tests whose set-up changed.
- Gate 5: `realtime_policy` ok (129 regions, one new, around the one-shot render functions);
  `scripts/check-host-core-policy.sh` ok; the render allocation counters stay at zero (host-core and
  graph suites green).
- D4 in graph: `runtime::tests::block_end_runs_on_accepting_observers_in_binding_order_after_a_good_block_only`.

**Departures and facts.**
1. **Manifest edits outside the authorized list.** `crates/engine/Cargo.toml` gains
   `[target.'cfg(loom)'.dependencies] loom = "=0.7.2"`, and `Cargo.lock` lists `loom` under
   `host-core`. Without the first, `engine` does not build as a dependency under `--cfg loom` (its
   `sync` shim names `loom`, which only its own tests see), so no crate that depends on it can run a
   loom model. Nothing reaches it without `--cfg loom`; the engine loom leg is unchanged and green.
2. **The CI command carries `-C target-feature=+avx2,+fma`.** `RUSTFLAGS` replaces
   `.cargo/config.toml`'s rustflags and `lane` refuses to compile without them, so the spec's literal
   command does not build. The step uses its own target dir (`target/ci/loom-host-core`).
3. **Reader.** Because the word is loaded before the pop, a record of the loaded generation is the
   result in `ARMED` as well as `CAPTURING` and `COMPLETE`. After the pop the reader reloads the
   word and makes at most two compare-and-swaps (`CAPTURING`, then `COMPLETE`), not two from the
   word it first loaded, since that word can still be `ARMED`.
4. **Control writes the word with `swap`, not `store`** (cancel, `commit_continuous`,
   `stop_continuous`, the `INVALID` read). Every write is then a read-modify-write in one total
   modification order; loom's partial order for a plain store racing a compare-and-swap let a render
   transition survive a cancel, which the C++ model forbids. Same bits, same cost.
5. **`accepts_block_end` is `true` in both modes.** It is read once at bind and the capture mode
   changes later, so continuous mode takes the call and returns at once.
6. **Accounting.** The runtime's bind-time unit table is charged in
   `GraphRuntimeMetadataResourceEstimate::block_end_unit_table_bytes` (one `u32` per emitted op, the
   issue #936 table rule), `GraphNodeObserverBinding` records the flag read at construction, and
   `fixtures/graph` is regenerated (`direct-route.resources.json` +36 bytes, `MANIFEST.tsv`).
   `tools/realtime-policy/src/policy.rs` raises `marked_regions_floor` 128 -> 129 for the new region.
7. **Tests changed mechanically.** Tests that built the word as `AtomicU8` use `new_state_word`;
   the two single-thread tests that complete a window through `observer.capture` call `block_end`
   (as the graph does); `failed_block_invalidates_its_window_and_drains_the_queued_result` is
   replaced by the three tests above because D4 deletes `COMPLETE -> INVALID`;
   `SpectrumCapture::new_one_shot_for_test` is removed with it.
8. The race test's "two further blocks" bound is asserted as "began within one block of the arm"
   (read after the arm). A bound on arrival time is not stable under scheduling; the stale-record
   test covers the retry deterministically.

**Successor candidates.** A mode switch (`commit_continuous`) racing a one-shot push can leave a
one-shot record in the queue for the continuous reader (D7, pre-existing, unchanged).

## Fold (batch 8 review)

- **A held window survives a later bad block.** A window that a good block completed and a stale
  record keeps held (D5: the push found the slot full) used to be dropped with `INVALID` if a later
  block failed (`invalidate_after_failure`) or was malformed (unequal plane lengths, a bad frame
  count, a non-finite sample). The reader then got `Invalid` for a window that was good. The
  one-shot state now records that the held window outlived a block end (`survived_block_end`, set
  by a failed push in `one_shot_block_end`, cleared when a new window is held), and
  `one_shot_invalidate` leaves such a window held with the word `CAPTURING`; the next block end
  tries the push again. The block that completed the window and then fails still drops it (D4
  unchanged: it has not survived a block end). This refines D4's sentence "a held window is dropped
  with an INVALID CAS": the window of the failed block is.
- **Test.** `spectrum::tests::a_held_window_survives_a_later_failed_or_malformed_block` (red
  without the guard: the read answers `Invalid`). The 1,000-seed race test and the loom model pass
  unchanged (the model never leaves a window held behind a stale record across a failed block).
- **Loom flags.** `.github/workflows/qualification.yml`'s loom step copies `-C
  target-feature=+avx2,+fma` from `.cargo/config.toml`; both files now carry a comment that names
  the coupling.

## Gate re-pins (batch 8 CI fold)

- **Cause.** The `block_end` hook added the `block_end: bool` flag to `GraphNodeObserverBinding`
  (80 -> 88 bytes, the same step as #816's flag) and the runtime's bind-time `block_end` unit table,
  charged as `GraphRuntimeMetadataResourceEstimate::block_end_unit_table_bytes` (one `u32` per
  emitted op).
- **Measured, `resources.jsonl`.** Only the meter-bearing rows move: `engine_owned_meter_payload_bytes`
  and `engine_owned_retained_payload_bytes` rise by 8 per meter (1 meter: +8, e.g. 1166 -> 1174 at
  1 track, 1178 -> 1186 at 4 and 65,537 tracks; 7 meters: +56, e.g. 11522 -> 11578, 11606 -> 11662).
  The three `meters = 0` rows, the processor payload, the largest allocation and the allocation
  count do not move (the table is graph metadata, not a builtins resource). `MANIFEST.tsv`
  `resources.jsonl` hash 57eb5083... -> 7a9c9284...; `ACCEPTED_MANIFEST_SHA256` (both
  `fixture_builtins.rs` and `builtins_graph.rs`) -> `f99fdbf2...`; `GRAPH_OBSERVER_BINDING_BYTES` 80 -> 88.
- **Test.** `console-workload`'s `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables`
  now adds the block-end table (one `u32` per emitted op, 2052 bytes on the 513-op row) to the exact sum.
- **Fuzz lock.** `fuzz/Cargo.lock` gains loom 0.7.2 and its transitive entries (additions only).
