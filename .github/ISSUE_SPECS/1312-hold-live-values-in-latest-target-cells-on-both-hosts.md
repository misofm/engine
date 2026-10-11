# Hold live values in latest-target cells on both hosts

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A strip's live fader, mute and pan or matrix values are held in latest-target cells instead of
bounded FIFO queues, on the C ABI and in the browser, through the same shared code. A host can
send any number of fader, mute or pan edits between two render calls, a paused host included,
and none is refused for room. Render applies the last committed value of each cell at the next
block and never skips it, and every value of one transaction takes effect in the same block
(Amendment 2). An exact `live_values_superseded` counter, readable through
`COUNTERS_GET`, reports how many committed cell values a later one replaced before render read
them. The ack bytes do not change.

This is the first lane slice of D15-2: the strip fader/mute and matrix/pan lanes, on the cell
primitive of *Add the latest-target cell primitive and its loom model* (#1432, split out by
Amendment 1). The remaining cell slices reuse the primitive and the counter: *Hold effect parameter,
bypass and EQ-target values in latest-target cells* (#1345), *Hold strip input-lane values in
latest-target cells* (#1346) and *Hold route-lane values in latest-target cells* (#1347). Under
AGENTS.md's half-day rule, the browser status field and the cross-host agreement test are split
out into the successor *Report live_values_superseded in the browser status and prove both hosts
drain strip cells alike* (#1399).

## Context

- **Records and rings.** `TrackControlRecord` (matrix, `crates/builtins-compiler/src/lib.rs:105`)
  and `TrackFaderRecord` (fader or mute with a `BuiltinLaneSelector`, `:129`) ride bounded SPSC
  rings created per strip (`:3583-3587`), charged at `:3777-3790`, and held by
  `TrackControlProducer::{producer, fader}` (`:254-270`).
- **Render drains.** `drain_fader_controls` (`:1064`) and `drain_matrix_controls` (`:1108`) pop
  every record available at block entry and apply each in order (`set_fader_db`, `set_mute`,
  `set_target_smoothed`, `crates/builtins/src/lib.rs:3869`, `:3891`, `:4056`). The test-only
  scalar processors do the same (`LiveControlMatrixProcessor`
  `crates/builtins-compiler/src/lib.rs:4316`, `LiveControlFaderProcessor` `:4377`). Both stages
  are seam-side, so no channel-symmetry witness reads them (`builtins-compiler/src/lib.rs:758-771`,
  `:816-823`).
- **No record carries a sample time.** Every record popped at one block entry lasts zero samples
  except the last per address, so a FIFO and a latest-value cell differ only when two records for
  one address meet in one drain: FIFO snaps to the earlier value, then ramps from it.
- **C ABI admission.** `commit_live` checks room (`crates/control-plane/src/control.rs:1325-1334`,
  `LiveBackpressure`, `control.live.backpressure`) and pushes after every fallible check
  (`:1424-1439`); the live lane depth is 16 (`crates/control-plane/src/compile.rs:17`). Push
  order is `FaderDb` then `Mute`, left before right (`crates/host-core/src/live_delta.rs:62-77`).
  The header promises 16 pending values and the backpressure string
  (`crates/capi/include/miso_engine_v1.h:50-55`).
- **Browser admission.** The matrix and fader bands use the same producers, with a free-room pass
  and `in_flight` accounting (`hosts/host-web/src/lib.rs:1548-1559`, `queue_available`
  `:1762-1789`, `push` `:1800-1815`, room check `:5568-5580`); solo and VCA mutes compose into
  fader records.
- **Counters.** `CounterId` ends at `ValidationFailures = 15`
  (`crates/protocol/src/message_wire.rs:626-642`). The control plane refreshes provider counters
  in the service step of *Add miso_engine_v1_service for bounded control work between edits*
  (#1348 D1), which every control call runs first (#1348 D2).
- **Realtime root.** `scripts/check-realtime-policy.sh:29` admits no `unsafe` in a new realtime
  file; `crates/engine/src/realtime/observe.rs` shows the safe-Rust atomics-only style.
- **Swap carry.** *Carry fader, mute and pan ramps across a plan swap* (#1277) follows this slice
  and writes its carry-then-retarget values into these cells (root ruling: cells come before the
  carry slices that write into them).

## Decisions frozen for this slice

- **D1. The cell** is #1432's D1 (#1432 Amendment 1): a revision-bounded three-slot cell in
  `crates/engine/src/realtime/latest_cell.rs`, behind the plan's revision gate (#1502 D1).
  - A write cannot fail.
  - A block applies, per cell, the newest value whose revision is `<=` the block's snapshot `S`.
    So every value of one committed revision takes effect in the same block: the first block
    whose snapshot covers it, or, while a successor is pending, the successor's adoption block.
  - `peek_unread` changes nothing.
  - That is D15-2 condition 2 and its ack meaning as amended (D9). This slice builds the strip
    stages' cells and dirty words on it (D3).
- **D2. One counter unit: writes to an adopted plan, superseded before a covering snapshot.**
  - `live_values_superseded` counts, per cell, writes to a plan that render adopts which the first
    block covering them (its snapshot `>=` their revision) does not apply, because a later write
    to the **same cell** with a revision `<=` that snapshot exists.
  - When render applies sequence `s` after `p`, it adds `s - p - 1` (the count #1432 D2 reports).
  - Excluded by unit (root ruling 4, 2026-10-05):
    - writes to a candidate that is withdrawn and dropped. Their revisions complete through the
      watermark's `SUPERSEDED` outcome (#1310 D6, D15-17);
    - values on lanes a swap restarts. They are in the successor's prepared model (#1277 D6).
  - The carry counts a predecessor's unread writes into the successor (#1280 D2). A
  record that writes two cells (a `Both` record writes the left and right cells) counts in each.
  So 40 `Both` fader edits before one block add `2 × 39 = 78`. #1345, #1346 and #1347 count in
  this unit. The counter is one
  `Arc<AtomicU64>` per session, passed to every plan the session prepares. Render is its only
  writer (load, saturating add, store) and the control thread reads it.
- **D3. Cells per strip.** Fader stage: four cells (fader left, fader right, mute left, mute
  right), each two words (value bits, ramp), and one dirty word. Matrix stage: one cell of five
  words (four coefficients, ramp) and one dirty word. A `Both` record writes both channel cells.
- **D4. Canonical drain order.**
  - Fader before mute, left before right, in every block, under the block's snapshot.
  - When both channel cells of a kind were `Applied` in this block with equal words, render
    applies one `Both` call, as today's single record does.
  - A cell whose read is `Unchanged` in this block (for example because its newest revision is
    `> S`) is not paired.
- **D5. Producers.** `TrackControlProducer::{producer, fader}` become cell writers with infallible
  writes; the input lane stays a queue until #1346. The rings at
  `crates/builtins-compiler/src/lib.rs:3583-3587` for these two lanes and their resource rows are
  replaced by the cells' rows, computed from the types.
- **D6. The contract the carry slices rely on.**
  - A stage exposes `apply_pending(&mut self)`: the block drain with `LiveSnapshot::ALL`, in D4's
    order, the same code the block drain runs.
  - #1277's carry calls it on the predecessor before exporting a lane. Two facts make `ALL` equal
    to "the newest `<=` the adoption block's `S`" there:
    - control writes only the newest plan's cells, so every predecessor write happened before the
      successor's publication, which render's claim acquired;
    - every predecessor write carries a revision below the successor's revision.
  - #1277 writes its retarget values into the successor's cells with the D5 writers, which cannot
    fail. They are stamped from the successor's own gate handle with the structural revision,
    before that revision is published on the successor's gate.
- **D7. Validate before any write (D15-2 condition 3).** `commit_live`'s order stays: every
  fallible check, then the cell writes, then the protocol commit. The fader and matrix room terms
  of step 4 go; nothing else moves. The browser writes cells only after its whole batch passed
  every check; its fader and matrix room terms go too.
  - **Stamps follow the producers: one routing.**
    - Each cell write carries the transaction's stamp, taken from the gate handle of the provider
      epoch whose producers it writes (#1502 D6).
    - The stamp's revision is the revision the transaction commits as. The revision publication
      on the same handle stays the commit's last write (#1314 D2's position).
    - `commit_live` debug-asserts that the stamped revision equals the revision it publishes
      (review m2). #1432's `GatePin` checks that a stamp is above the last published revision.
  - **Where the stamp is taken (root, 2026-10-11; #1432 follow-ups m3).**
    - *The handle of a published plan* (the current epoch, a pending epoch, or the worklet's plan before
      #1382). A live or model-only commit, and a browser admission, takes its stamp as its last fallible
      step, after step 5's predicate (including #1503's bound). A refused stamp (#1432 D1) returns
      the transaction's internal-error refusal with nothing written or pushed and no revision used.
      A commit that publishes takes the stamp even when it writes no cell, and publishes the stamp's
      revision. On the C ABI step 5 is the protocol commit's predicate, which includes #1503's
      revision bound; the stamp's revision is taken after that bound. Its first cell write comes
      after the stamp. Every step from the stamp to the revision publication on that handle cannot
      fail: the cell writes, the pushes whose room was checked, the protocol commit and the owner
      commits, each an `expect` on a checked invariant. So every revision stamped on such a handle is
      published before a higher one is stamped, and a refused transaction stamps nothing and writes
      nothing.
    - *The handle of a candidate that is not yet published.* A rebuild's carry retargets (#1277 D5,
      #1280) are stamped with the structural revision `r` inside preparation, and fallible steps
      follow: the replacement peak, the pending check, `reserve_replacement`, the report table, the
      protocol commit, and #1310 D4's checks. Render cannot reach the candidate before its publication. A rebuild stamps `r`
      on the candidate's handle before its protocol commit, inside preparation when it writes
      retargets. A refused stamp refuses the rebuild like any other refusal of this list. On a fresh
      gate with `1 <= r <= GATE_REVISION_MAX` (#1503 D1, checked when the transaction is prepared),
      it cannot be refused.
      On any refusal, the same call drops the candidate plan and its provider epoch, with the gate
      handle and the cells. A refused candidate, handle or cell is never kept, cached or reused. On
      success, `r` is published on that handle before `reservation.commit()` (#1502 D6), and nothing
      else is stamped on it between.
    - *Handles that persist.* A handle moves to another thread (#1502 D1), or is withdrawn (#1343,
      #1310 D1), only with no open stamp. Before #1382, the Worker publishes every revision it stamped
      on a candidate's handle before it hands that handle to the worklet. A withdrawn candidate, which
      #1310 D4 may republish, holds no open stamp, because its rebuild published `r`.
    - *The browser's revision.* The worklet takes its host-local revision after the batch's last check
      (for example `last_published + 1`), so a refused batch uses no revision. Before #1382 the
      browser's host-local counter is not bounded by #1503's store, so the ceiling compare in `stamp`
      is its only always-on guard; keep the ceiling in the check.
    - *What checks the rule.* The stamp's revision bound is always on and returns a refusal, not a
      panic (#1432 D1). Only the open-stamp rule stays debug-only. `GatePin`'s debug assertions check the order only: a higher revision
      stamped while one is open. They cannot see a cell written before a fallible check. A refused C ABI
      transaction uses no revision, so the next transaction stamps the same `r` again, which `GatePin`
      allows, and its publication would make a refused write visible (#1432: an abandoned value's leak
      is the caller's contract). Gate 2 defends that on the C ABI, and gate 9 defends it in the browser.
      Release builds keep the open-stamp check debug-only: a violation never loses an acked value,
      because the newest published value and the pinned value stay protected (#1432 D1); its effect
      would be a refused value that is applied, which the structure above and gates 2 and 9 prevent.
      An always-on assert is not used for it: it still cannot see the same-revision leak, and on the
      C ABI `catch_unwind` would turn a mid-commit panic into RESULT_INTERNAL on a half-committed
      session.
  - **The browser (review M1).** The thread that writes a plan's cells publishes on that plan's
    gate.
    - Until *Admit browser live edits in the Worker through the committed model* (#1382), the
      worklet's admission holds its plan's gate handle.
    - After an admitted batch's cell writes, it publishes a host-local revision on that handle.
      The revision is strictly above every revision published on that gate.
    - Render takes its snapshot from the same gate, as on the C ABI, so no browser render path
      uses `LiveSnapshot::ALL`.
    - A handle moves between threads only by ownership transfer (#1502 D1). While #1381 and
      #1290 run before #1382, the Worker publishes only on a candidate's gate before that
      candidate's publication.
- **D8. Counter on the C ABI.** `CounterId::LiveValuesSuperseded = 16` in the frozen registry. The
  control plane copies the session counter into its provider in `SessionState::service` (#1348
  D1), beside the telemetry counter refresh, so `COUNTERS_GET` after any control call reports it.
  The browser's status field is #1399's.
- **D9. Ack meaning (unchanged bytes).** Header text: a committed live value reaches render no
  later than the first block whose render call begins after the submit returns, or, while a
  successor is pending, at its adoption, which the watermark reports (D15-2, D15-17); every live
  value of one transaction takes effect in the same block, and the watermark names that block's
  first sample; a later value for the same lane committed before that block replaces it, and
  `LIVE_VALUES_SUPERSEDED` counts it.
  - The snapshot wording ("the first block whose live snapshot covers it") stays in engine docs.
    A host cannot observe a snapshot (review m8). The
  16-value room and the backpressure string are removed for fader, mute and pan or matrix; effect
  lanes keep theirs until #1345.
- **D10. FIFO stays FIFO.** Effect records (including `Observe`), input records, route records,
  automation and structural edits are untouched here (D15-2).
- **D11. The acked-batch question: can an ack ever precede a drop? No.**
  - Every fallible check precedes the first cell write (D7), and writes cannot fail.
  - A write never overwrites the newest published value of its cell, nor the value that render's
    open snapshot needs (#1432 D1). So a value not yet applied is either applied by a later block
    or superseded by a newer committed value. The committed model holds the newer value, render
    applies it, and the counter records the replacement (D2).
  - A value whose revision is above a block's snapshot is not dropped: its cell stays pending.
  - A value written to a plan that is swapped out before it drains is applied by the carry's
    `apply_pending` (D6), or is in the committed model its successor is prepared from (#1053 D7,
    D15-17).
- **D12. Where the drains run.**
  - The fader and matrix drains move from `process` to the bank's `begin_block`, which already
    carries the drain contract (`GraphPreparedBuiltinBankProcessor::begin_block`). The fused
    `FaderMatrixBankProcessor` forwards to both. The test-only scalar processors drain at the top
    of `process`.
  - Each drain reads under the block's snapshot `S`, which #1504 hands to `begin_block` and to
    `GraphBindingBlock`.
  - Every dirty take of a block follows the block's snapshot.
  - Every drain runs in every rendered block (#1504 D3).

## Deliverables

1. (Moved to #1432 by Amendment 1: the cell module, `peek_unread` and its loom model.)
2. Fader and matrix lanes on cells in `builtins-compiler` (bank and test-only scalar processors),
   with D6's `apply_pending`.
3. The C ABI and browser admission changes, the C ABI counter, header and docs text.

## Authorized paths

- Amendment 1, to move existing fader and matrix pushes onto cell writes only:
  `crates/builtins-compiler/tests/allocation_tracker.rs`; `crates/graph-compiler/src/lib.rs` (the
  test `a_banked_fader_command_lands_on_the_block_it_was_admitted_in` and its drain-contract doc
  text; the test keeps a red mutation, a drain after `bank.process`, now against a cell read);
  `crates/host-core/tests/live_lanes.rs`, `randomized.rs` (its accept/refuse model for fader and
  matrix edits becomes "always accepted"), `live_delta.rs`, `symmetry_witness.rs`, `vca_live.rs`,
  `strip_controls.rs`, `prepare.rs`. No `try_push`-shaped compatibility shim.
- Amendment 2: a test that writes cells publishes its revision on the plan's gate handle, in the
  same files. New: `crates/host-core/tests/live_transaction.rs` (gate 8).
- `crates/builtins-compiler/src/lib.rs`.
- `crates/host-core/src/live_delta.rs`, `crates/host-core/src/control_provider.rs` (the counter
  setter), `crates/host-core/src/prepare.rs` (the producer and counter plumbing only; stream A's
  file, sequenced by the coordinator), `crates/host-core/tests/successor_swap.rs` and
  `crates/host-core/tests/support/successor.rs` (only to move existing tests that push fader or
  matrix records onto cell writes).
- `crates/control-plane/src/`, `crates/capi/` (header prose and tests), `crates/protocol/src/`
  (D8).
- `hosts/host-web/src/lib.rs` (the matrix and fader bands' admission only),
  `hosts/host-web/src/tests.rs`. These are stream H's; if #1332 has landed, the browser admission
  is the control plane's.
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md` (counter 16).

## Non-goals

- Effect, input and route lanes (#1345, #1346, #1347). Ramp defaults (#1054).
- The swap carry of these lanes (#1277, which follows this slice).
- The browser status field and the cross-host agreement test (#1399).
- Atomicity for lanes still on queues (effect lanes until #1345).
- The race-converse test under a real render thread: it is
  *Prove under a racing render that a live transaction lands in one block and the watermark names
  it* (#1505).

## Hazards

- **Digests move only where two records met in one drain.** Re-pin each such fixture with its
  reason ("step then ramp" now ramps from the running value); never in bulk.
- **Fader and mute share one gain ramp** (`set_mute` is "a retarget of the same gain"). D4's order
  keeps a mute ramp from being cut short by a fader move in the same block.

## Objective gates

1. **Many edits, one block (new capi test).** Paused host: 40 track fader edits, each a `Both`
   record, each `RESULT_OK`; one render. The output equals a twin that made only the last edit;
   `COUNTERS_GET` reports `LIVE_VALUES_SUPERSEDED` 78. *Red if a lane is still a queue (refusal at
   17) or the per-cell gap arithmetic is off.*
2. **No write before the predicate (new capi test).** Arm `TestStructuralFaultPhase::BeforeLivePush`
   on a fader-and-pan edit: every cell's sequence and the output are unchanged. Mutation (PR
   evidence): move the cell writes above step 5; this test turns red.
3. (Moved to #1432 by Amendment 1: the cell's loom model and its `peek_unread` unit test.)
4. **`apply_pending` equals the drain (new builtins-compiler test).** Dirty fader, mute and matrix
   cells, then `apply_pending` and an empty-drain block, render bit-identically to a twin whose
   block drains them.
5. **Superseded tests.** Delete `a_full_live_lane_refuses_before_anything_changes` and
   `a_live_transaction_with_one_full_lane_pushes_to_no_lane`
   (`crates/capi/src/runtime/live_tests.rs:609`, `:692`); gate 1 replaces them. Adapt
   `a_replayed_live_edit_pushes_nothing` (`:726`) and
   `a_fault_before_the_live_push_leaves_every_queue_and_the_model_alone` (`:1092`) to cell
   sequences. Rewrite `command_flood_is_typed_backpressure_and_leaves_the_render_untouched` and
   `overfilling_a_bus_queue_is_typed_backpressure_with_no_push`
   (`hosts/host-web/src/tests.rs:4775`, `:10187`) on observation records
   (`COMMAND_OBSERVE_SUBSCRIBE`), which stay a FIFO after #1345, or delete one that would
   duplicate an existing observation test.
6. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   (all violation counts 0); `bash scripts/check-realtime-policy.sh`; `bash scripts/check-web-audioworklet.sh` (its call-graph gate); the browser legs of
   `qualification.yml`'s `browser` job.
7. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features
   test-support`; `cargo test --locked -p builtins-compiler --features test-support`;
   `cargo test --locked -p host-web --features test-support`;
   `cargo test --locked -p host-core --features control-provider,test-support`;
   `cargo test --locked -p protocol --features test-support`; `bash scripts/check-capi-abi.sh`;
   `bash scripts/check-workspace-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
8. **One transaction, one block (new host-core test, deterministic; Amendment 2).**
   - Two strips `A` and `B` render through the exchange (`RealtimePlanOwner`).
   - Write one transaction (mute `A`, unmute `B`) into their cells, stamped with revision `r`
     from the plan's gate handle, and do not publish `r`. Render one block: neither change is
     applied, and the watermark stays below `r`.
   - Publish `r` and render the next block: both are applied from its first sample, and the
     watermark reads `(r, that block's first sample)`.
   - Mutation (PR evidence): the drains read with `LiveSnapshot::ALL`. The first block turns red.
   - The race form of this gate is #1505's.
9. **A refused browser batch leaves no trace (new host-web test, debug build; Amendment 3).**
   - Run it on the worklet admission before #1382, and through the Worker's committed model after
     #1382.
   - Batch 1 carries a fader edit on strip A and an edit refused at the batch's last check (for
     example an effect record whose queue is full; effect lanes stay queues until #1345).
   - Batch 2 carries a fader edit on strip B and is accepted.
   - Render two blocks: B's edit applies, A's edit never applies, and no debug assertion fires.
   - Mutations (PR evidence): A's cell write moved before the last check: A's edit is heard. The
     browser stamp taken before the checks: `GatePin`'s open-stamp assert panics at batch 2.
   - Test value: a browser path that writes or stamps before its last check. Gate 2 sees only the C
     ABI, and no browser gate covers cell writes.

## Test value

- Gate 1: a lane left a queue, or a counter in a unit other than D2's.
- Gate 2: a cell write moved before a fallible check, which with cells destroys an acked value.
- Gate 4: a carry path that applies pending cells differently from the block drain, which #1277
  relies on.
- Gate 8: a drain that applies a value above the block's snapshot, so a transaction lands across
  two blocks, or a watermark that names a block before the one that applied the revision.
- Gate 9: a browser path that writes or stamps before its last check, so a refused batch's edit
  is heard or the next batch trips `GatePin`'s open-stamp assert.

## Amendment 1 (root, 2026-10-05)

The first implementation run stopped before any change; it is not an attempt. Two findings:
D5's infallible cell writers break every existing `try_push` caller of fader and matrix records
in nine files outside the authorized paths, and with them the slice exceeds AGENTS.md's half-day
size. Root ruled: split up front. The cell primitive, `peek_unread`, its loom model and its unit
test (old D1, deliverable 1 and gate 3) move to *Add the latest-target cell primitive and its loom
model* (#1432), on which this issue now depends. This issue keeps deliverables 2-3 and gates 1, 2
and 4-7, and gains the nine files for moving existing fader and matrix pushes onto cell writes,
with `randomized.rs`'s model becoming "always accepted" and the graph-compiler drain-contract test
keeping a red mutation against a cell read. A `try_push` shim that always returns `Ok` is refused
as an interim shortcut.

## Amendment 2 (root, 2026-10-05)

Root's binding requirement: a live transaction committed while render is mid-block must not be
torn across blocks, and the watermark's `first_sample` is exact. #1432 Amendment 1 replaced its
triple buffer with the revision-bounded cell, and root ruled one revision gate per plan
(#1502). This issue changes accordingly:

- **Decisions.**
  - D1 (the cell under the block's snapshot).
  - D2 (the counter's unit: writes to a plan that render adopts; root ruling 4).
  - D4 (the order under the snapshot).
  - D6 (`apply_pending` with `LiveSnapshot::ALL` on a quiescent predecessor).
  - D7 (stamps from the provider epoch's gate handle, the browser's gate rule; review M1, M2 and
    m2).
  - D9 (the header keeps D15-2's host-observable bound and adds the same-block sentence; review
    m8).
  - D11 (the acked-batch answer under the snapshot).
  - New D12 (drain placement).
- **Gates.** New gate 8 (deterministic). The race converse moves to the successor #1505
  (review m6), because this issue has already been split once for size.
- **Non-goals.** "Per-transaction atomicity across cells" is deleted.
- **Dependencies.** #1502 (the plan gate and the snapshot), #1503 (the revision
  ceiling) and #1504 (the snapshot reaches every drain).

## Amendment 3 (root, 2026-10-11)

Trigger: #1432 follow-ups verdict m3 (`GatePin` now debug-refuses a higher stamp while a stamped
revision is unpublished). Root ruling under the owner's delegation of 2026-10-11, adopted after a
fresh adversarial review (ACCEPT WITH CHANGES):

- D7 gains "Where the stamp is taken": a published plan's handle stamps after the last fallible
  check, and nothing after the stamp can fail; a candidate's retargets are stamped inside
  preparation, which is safe only because a refused candidate (plan, epoch, handle, cells) is
  dropped by the same call and never kept, cached or reused; a handle moves threads or is withdrawn
  only with no open stamp; the browser takes its host-local revision after its last check; what
  checks the rule, and why no always-on assert.
- New gate 9 (a refused browser batch leaves no trace).
- #1277 gains the matching gate (a refused candidate is never reused).

## Dependencies

- *Add the latest-target cell primitive and its loom model* (#1432): the cell (D1).
- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): the gate handle, the snapshot and the stamps' routing (D7).
- *Bound committed revisions at the plan gate's ceiling* (#1503) (stream order).
- *Hand each block's live snapshot to every live drain* (#1504): the snapshot at every drain
  (D12).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the service step
  that refreshes the counter (D8).

Dependents: #1277 (which writes into these cells), #1280 (which peeks them), #1345, #1346, #1347,
#1399 and #1505, each on this primitive and counter.
