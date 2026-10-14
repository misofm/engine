# Make the spectrum one-shot handoff safe across threads

Successor of #1591, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `5415352a9` (PR #1608).
Issue number: assigned at filing (written here as #TBD6). Feature sprint mode (owner,
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
