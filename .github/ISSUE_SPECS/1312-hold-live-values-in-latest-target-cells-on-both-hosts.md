# Hold live values in latest-target cells on both hosts

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A strip's live fader, mute and pan or matrix values are held in latest-target cells instead of
bounded FIFO queues, on the C ABI and in the browser, through the same shared code. A host can
send any number of fader, mute or pan edits between two render calls, a paused host included,
and none is refused for room. Render applies the last committed value of each cell at the next
block and never skips it. An exact `live_values_superseded` counter, readable through
`COUNTERS_GET`, reports how many committed cell values a later one replaced before render read
them. The ack bytes do not change.

This is the first slice of D15-2: the strip fader/mute and matrix/pan lanes, and the cell
primitive. The remaining cell slices reuse the primitive and the counter: *Hold effect parameter,
bypass and EQ-target values in latest-target cells* (#1345), *Hold strip input-lane values in
latest-target cells* (#1346) and *Hold route-lane values in latest-target cells* (#1347). Under
R10 the browser status field and the cross-host agreement test are split out into the
successor *Report live_values_superseded in the browser status and prove both hosts drain strip
cells alike* (#1399).

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
- **C ABI admission.** `commit_live` checks room (`crates/capi/src/runtime/control.rs:1189-1198`,
  `LiveBackpressure`, `control.live.backpressure`) and pushes after every fallible check
  (`:1288-1303`); the live lane depth is 16 (`crates/capi/src/runtime/compile.rs:12-13`). Push
  order is `FaderDb` then `Mute`, left before right (`crates/host-core/src/live_delta.rs:62-77`).
  The header promises 16 pending values and the backpressure string
  (`crates/capi/include/miso_engine_v1.h:50-55`).
- **Browser admission.** The matrix and fader bands use the same producers, with a free-room pass
  and `in_flight` accounting (`hosts/host-web/src/lib.rs:1548-1559`, `queue_available`
  `:1762-1772`, `push` `:1800-1815`, room check `:5568-5580`); solo and VCA mutes compose into
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

- **D1. The cell: a triple buffer that never tears and never skips.** New module
  `crates/engine/src/realtime/latest_cell.rs`, safe Rust (atomics only, no `unsafe`).
  - A cell has three slots. Each slot holds the cell's words (`AtomicU32` each: the target and its
    ramp, one unit) and an `AtomicU64` sequence number.
  - A `middle: AtomicU32` holds a slot index and a `FRESH` bit. The writer (control) owns a
    private back index and next sequence. The reader (render) owns a private front index and the
    last sequence it applied.
  - **Write:** store the words and the sequence into the back slot (`Relaxed`). Then
    `prev = middle.swap(back | FRESH, AcqRel)`; the back index becomes `prev`'s index. Then
    `fetch_or` the cell's bit into its stage's dirty word (`Release`). Writes cannot fail.
  - **Read (render, at the drain):** `dirty.swap(0, Acquire)`. For each set bit whose `middle`
    has `FRESH`: `prev = middle.swap(front, AcqRel)`; the front index becomes `prev`'s index; read
    the front slot. The three indices are always distinct, so render never reads a slot the
    writer is writing. Render reads the newest completed write in one pass: no retry, no spin, no
    skip.
  - A write that completes before a block's drain begins is applied in that block. That is D15-2
    condition 2 and the ack meaning of R1.
  - **Peek (carry only):** `peek_unread(&self) -> Option<(words, sequence)>` returns the `middle`
    slot's words and sequence when `middle` has `FRESH`, and `None` otherwise. It changes nothing:
    `middle`, the front index, the last applied sequence and the dirty word stay as they were, so
    a later read still applies the value. It is valid only once the writer is quiescent (no write
    to this cell can start before the peek ends), because a write after the peek began may reuse
    that slot. The plan-swap carry (*Carry live-controlled effect lanes across a plan swap*, #1280
    D2) is its only caller; it peeks a predecessor whose cells no control write reaches once its
    successor is published (D15-17, #1356). The reader also exposes `last_applied(&self) -> u64`.
- **D2. One counter unit: cell values replaced unread.** `live_values_superseded` counts, per
  cell, committed values that a later write to the **same cell** replaced before render read them.
  When render reads sequence `s` after `p`, it adds `s - p - 1`. A record that writes two cells
  (a `Both` record writes the left and right cells) counts in each. So 40 `Both` fader edits before
  one block add `2 × 39 = 78`. #1345, #1346 and #1347 count in this unit. The counter is one
  `Arc<AtomicU64>` per session, passed to every plan the session prepares. Render is its only
  writer (load, saturating add, store) and the control thread reads it.
- **D3. Cells per strip.** Fader stage: four cells (fader left, fader right, mute left, mute
  right), each two words (value bits, ramp), and one dirty word. Matrix stage: one cell of five
  words (four coefficients, ramp) and one dirty word. A `Both` record writes both channel cells.
- **D4. Canonical drain order.** Fader before mute, left before right. When both channel cells of
  a kind were read with equal words, render applies one `Both` call, as today's single record
  does.
- **D5. Producers.** `TrackControlProducer::{producer, fader}` become cell writers with infallible
  writes; the input lane stays a queue until #1346. The rings at `:3583-3587` for these two lanes
  and their resource rows are replaced by the cells' rows, computed from the types.
- **D6. The contract the carry slices rely on.** A stage exposes `apply_pending(&mut self)`: the
  D1 read and D4 order for every dirty cell, the same code the block drain runs. #1277's move and
  copy modes call it on the predecessor before exporting a lane; every write to the predecessor's
  cells happened before the successor's publication, so it sees them all. #1277 writes its
  retarget values into the successor's cells with the D5 writers, which cannot fail.
- **D7. Validate before any write (D15-2 condition 3).** `commit_live`'s order stays: every
  fallible check, then the cell writes, then the protocol commit. The fader and matrix room terms
  of step 4 go; nothing else moves. The browser writes cells only after its whole batch passed
  every check; its fader and matrix room terms go too.
- **D8. Counter on the C ABI.** `CounterId::LiveValuesSuperseded = 16` in the frozen registry. The
  control plane copies the session counter into its provider in `SessionState::service` (#1348
  D1), beside the telemetry counter refresh, so `COUNTERS_GET` after any control call reports it.
  The browser's status field is #1399's.
- **D9. Ack meaning (unchanged bytes).** Header text: a committed live value reaches render no
  later than the first block whose render call begins after the submit returns, or, while a
  successor is pending, at its adoption, which the watermark reports (R1); a later value for the
  same lane committed before that block replaces it, and `LIVE_VALUES_SUPERSEDED` counts it. The
  16-value room and the backpressure string are removed for fader, mute and pan or matrix; effect
  lanes keep theirs until #1345.
- **D10. FIFO stays FIFO.** Effect records (including `Observe`), input records, route records,
  automation and structural edits are untouched here (D15-2).
- **D11. The acked-batch question: can an ack ever precede a drop? No.** Every fallible check
  precedes the first cell write (D7), and writes cannot fail. A value replaced before render read
  it is not dropped: the committed model holds the newer value, render applies it, and the counter
  records the replacement (D2). A value written to a plan that is swapped out before it drains is
  applied by the carry's `apply_pending` (D6) or is in the committed model its successor is
  prepared from (#1053 D7, D15-17).

## Deliverables

1. The cell module (with `peek_unread`) and its loom model.
2. Fader and matrix lanes on cells in `builtins-compiler` (bank and test-only scalar processors),
   with D6's `apply_pending`.
3. The C ABI and browser admission changes, the C ABI counter, header and docs text.

## Authorized paths

- `crates/engine/src/realtime/latest_cell.rs` (new), `crates/engine/src/realtime/mod.rs` (exports).
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
- Per-transaction atomicity across cells: as today, one transaction's values may land one block
  apart.

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
3. **Never skip under a racing writer (loom, new `spsc_loom_cells_*` tests in `latest_cell.rs`).**
   A writer making two writes racing a reader that reads before and after: the reader never sees
   a mixed cell, applies each sequence at most once, and, once the writer's last write completed
   before a read began, that read returns it. The counter equals writes minus applies. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom`. A plain unit test in the same module: after
   three writes and no read, `peek_unread` returns the third write's words and sequence, a second
   peek returns the same, and the next read then applies that value and adds 2 to the counter.
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
   (all violation counts 0); `bash scripts/check-realtime-policy.sh` (no `unsafe` in the new
   module); `bash scripts/check-web-audioworklet.sh` (its call-graph gate); the browser legs of
   `qualification.yml`'s `browser` job.
7. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p control-plane --features
   test-support`; `cargo test --locked -p builtins-compiler --features test-support`;
   `cargo test --locked -p host-web --features test-support`;
   `cargo test --locked -p host-core --features control-provider,test-support`;
   `cargo test --locked -p protocol --features test-support`; `bash scripts/check-capi-abi.sh`;
   `bash scripts/check-workspace-policy.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a lane left a queue, or a counter in a unit other than D2's.
- Gate 2: a cell write moved before a fallible check, which with cells destroys an acked value.
- Gate 3: a cell that tears, applies twice, or skips the latest completed write (the single
  sequence-word design this replaces would skip it); judged by the interleavings loom reaches.
- Gate 3's peek case: a `peek_unread` that consumes the value (swaps `middle` or clears `FRESH`
  or the dirty bit), so the predecessor in copy mode never applies it (#1280 D6).
- Gate 4: a carry path that applies pending cells differently from the block drain, which #1277
  relies on.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): the service step
  that refreshes the counter (D8).

Dependents: #1277 (which writes into these cells), #1280 (which peeks them), #1345, #1346, #1347
and #1399, each on this primitive and counter.
