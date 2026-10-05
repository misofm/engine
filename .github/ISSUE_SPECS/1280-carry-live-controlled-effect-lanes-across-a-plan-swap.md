# Carry live-controlled effect lanes across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-7, D15-8).
Slice 11 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A banked effect lane that a live control drives keeps its exact state through a plan swap. The
browser always attaches live controls, and the C ABI does since #1263. The lane keeps its live
state too: its live bypass, its dry shunt, its unread live values and its pending Observe
records. No acknowledged live edit is lost and none is refused: a value committed but not yet
rendered at the swap is rendered by the successor, at any quantum, including a ramp in flight
across the swap. A structural transaction that also changes a
carried lane's live parameter or bypass sounds exactly like "live edit, then structural edit"
(D15-7). The same holds in copy mode, where the predecessor keeps rendering.

## Context

- A live-controlled bank slot is `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:937`, impl
  `:1151`). Its `drain` (`:1257`, run from `begin_block`, `:1211`) only **stages** values. Each
  lane's `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:341`) packs automation
  spans or prepared targets, and `process_inner` (`rack/src/lib.rs:1294`) applies them in the same
  block. Draining at the carry would stage values into a plan that never renders again (#1269's
  blocker X3).
- On `main`, `EffectControlLane` (`live.rs:125`) holds a queue consumer, a prepared-target FIFO,
  the live `bypass` flag and the live symmetry terms. Carrying a queue would need an inherited
  consumer and an admission limit on the successor, which is a new BACKPRESSURE for a live value.
  Decision 15 forbids that (D15-2). So this slice lands after *Hold effect parameter, bypass and
  EQ-target values in latest-target cells* (#1345), and carries **cells**:
  - #1345 D1 gives each lane one cell per `(parameter_index, channel)` of a block-rate parameter,
    one bypass cell, and for the EQ one cell per `(target slot, channel)`. Observe records stay in a
    small FIFO (#1345 D2).
  - #1312 D1-D2 make each cell a triple buffer whose slots carry a sequence. Render keeps the last
    sequence it applied (`p`); reading sequence `s` adds `s - p - 1` to `live_values_superseded`.
    #1312 D1 also gives the reader `peek_unread`, which returns the newest unread words and their
    sequence without consuming them, valid once the writer is quiescent.
  - The control plane writes only the newest plan's cells (D15-17), and holds live edits from a
    `CopyAndReturn` publication until adoption (*Hold live edits during a catch-up and apply them
    at the adoption sample*, #1356). So no control write reaches a predecessor's cells once its
    successor is published, and render reads them whole.
- The slot's dry shunt is one interleaved AoSoA line for the whole bank, with one shared cursor
  (`shunt`, `rack/src/lib.rs:956-958`; `BypassShunt`, `live.rs:868-890`). Observation taps are
  `ObservationLane` (`live.rs:588`), one per bank lane (`rack/src/lib.rs:961`).
- The console-lane carry through the payload scratch, the effect rule D1 and the refusal counter
  come from *Carry console effect lanes across a plan swap* (#1279). The base, the retarget pattern
  and the restart set come from #1277 (D2-D6), and the copy-mode rules from #1322.

## Decisions frozen for this slice

- **D1. Rule.** As #1279 D1, with live controls attached in both plans or in neither. Live
  parameter values and a lowered bypass are live, because the lane exists. Everything #1279 D1
  lists as prepared stays prepared.
- **D2. Carried cells (one function, both modes).** `EffectControlLane::carry_from(&mut self,
  predecessor: &EffectControlLane, mode: CarryMode)` (`CarryMode::{Move, Copy}`, #1322 D1's two
  modes) takes the predecessor by shared reference and writes only the
  successor lane:
  - it copies the applied live `bypass` flag and the live symmetry terms;
  - for every predecessor cell where `peek_unread` (#1312 D1) returns `Some((words, s))`, it stores
    the words in the successor lane's render-owned **carried slot** for that cell, with the unread
    count `n = s - p`, where `p` is the predecessor cell's last applied sequence. The writer is
    quiescent: no control write reaches a predecessor's cells once its successor is published;
  - it consumes nothing: the predecessor's cells stay unread, which D6 needs;
  - it writes no cell, so the control thread stays each cell's only writer.

  `carry_from` records `mode` in each carried slot. Each carried slot resolves at the **successor's first rendered block**, before that block
  stages anything, and a `render_slice` block of a catch-up counts (*Catch up a returned successor
  and adopt it exactly at a scheduled sample*, #1355). This is not a live-lane drain, so #1355 D4's
  closed lanes do not hold it: the carried slot is render-owned state, like a ramp in flight. The
  predecessor would apply the same value at its next block, which is the same boundary B, so the
  adopted output is exact (#1322 D2(b)). It resolves in #1345 D3's order:
  - if the successor's live lanes are open (#1355 D4) and its own cell holds an unread value, that
    value is newer (written after the successor's publication or by D4's retarget). It applies;
  - otherwise the carried value applies, and the successor's own cell, if unread, stays unread
    until its lanes open (at the adoption sample S for a warm successor, #1355 D4).

  Counting (#1312 D2, one unit, each superseded value once):
  - **Move mode.** The predecessor never reads again, so the successor counts for it: it adds `n`
    when its own newer value wins, and `n - 1` when the carried value applies.
  - **Copy mode.** The predecessor keeps its cells unread and counts `s - p - 1` itself at its next
    block (D6). The carried slot adds 0 in both branches. A successor's own cell counts only its
    own values, as every cell does.

  The carried slots are preallocated with the lane, one per cell, and cleared after that block.
  Nothing is refused and nothing is dropped.
- **D3. Pending Observe records.** The carry drains the predecessor's observation FIFO with the
  bounded drain and applies each record to the predecessor's `ObservationLane`, at the boundary
  where the predecessor's next block would apply it. Observation output is not audio, and the
  observer carry (*Carry meter and effect observation state across a plan swap*, #1327, and
  #1287 in copy mode) then carries the tap state. The predecessor's later blocks see an empty FIFO
  and the same tap state, so copy mode moves none of its bits (#1322 D2).
- **D4. Carry, then retarget (D15-7).** The per-effect value derivation inside
  `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) is extracted into one per-owner
  function, as #1277 D5 did for strips. For every carried lane whose live values differ, the
  successor entry points write its cells before they return: the bypass cell, the parameter cells,
  or an EQ's target cells designed from the base values. A cell write cannot fail (#1345 D5). In
  copy mode the writes are kept for publication (#1277 D5, #1356 D3).
- **D5. Shunt.** A carried lane's shunt words move with it, as a copy of that lane's words of the
  interleaved line, taken relative to the shared cursor of each bank. The same holds in both modes.
- **D6. Copy mode (#1322 D7).** D2, D4 and D5 are read-only on the predecessor, and D3's drain is
  bit-neutral for it (#1322 D2). The successor lane also copies the payload (#1279 D4). A
  predecessor lane with unread cells copies exactly: its unread values go to the successor's
  carried slots and stay unread in the predecessor, which applies them at its own next block and
  counts what they superseded there. The successor applies the same values at its first rendered
  block (D2), a catch-up block included, and counts nothing for them. There is no copy refusal.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** This slice adds no admission
  check and no refusal. Every acked value is in a cell of the plan it was written to. A carried
  value reaches the successor through D2, and a value superseded there is counted exactly.

## Deliverables

1. D2, D3 and D6's lane copy in `crates/effect-contract/src/live.rs`. D4 in `crates/host-core`. D5
   in `crates/rack`. The location table and both modes in `crates/graph`. The inventory rows and
   the join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/effect-contract/src/live.rs`
- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/host-core/src/live_delta.rs`: D4's extraction only. This is stream B's file.

## Non-goals

- No bank to per-node flips (#1281), and no per-node moves (#1282).
- No admission change on either host. Cells (#1345) already admit every live value.
- No observer window carry (#1327, #1287).

## Objective gates

1. **No acked value lost.** Prepare with live controls and a console EQ. An EQ gain target
   `v0 → v1` is rendered in A. A target `v1 → v0` is written to A's cells just before the swap and
   not yet rendered. B is prepared from the base (`v0`). Every block equals the reference A plus the
   muted track, fed the same values at the same blocks. Repeat with a compressor threshold.
2. **Ramp across the swap at a small quantum.** At quantum 32, a compressor threshold ramp of 64
   samples in flight at the swap is bit-identical to the reference.
3. **Newest value wins, counted exactly.** Write 40 values of one compressor parameter to A's cell
   after A's last block, then publish B, then write 40 values of the same parameter to B's cell.
   Every write succeeds. B's first block equals a twin that wrote only the last value, and
   `live_values_superseded` grows by exactly 79 (A's 40 carried values and 39 of B's). A
   second case writes nothing to B: the carried value applies and the counter grows by 39. A
   copy-mode case writes the 40 values to A, copies into B, renders A's next block and B's first
   block, and writes nothing to B: both apply the last value, and the counter grows by exactly 39
   in total (A's 39; B's carried slot adds 0).
4. **Live bypass and shunt.** A true-peak limiter insert on four tracks (a full bank at
   `Backend::Simd4`; eight tracks at `Backend::Simd8`), with one lane bypassed live before the swap
   and the cursor at a nonzero offset. The bypassed lane keeps its delayed dry signal across the
   swap.
5. **Carry, then retarget.** B also changes a carried lane's compressor threshold and an EQ band
   gain (live values). Run 1 is the structural swap with D4's retarget. Run 2 writes the same edits
   to A just before the swap and prepares B from a base that holds them. The two runs are
   bit-identical in every block, and the strip is not in the restart set.
6. **Copy mode.** Gates 2 and 4 with a copy after the swap block's predecessor block, then at least
   two blocks rendered on B directly by the test before B is adopted (the shape of a catch-up
   block; #1355's `render_slice` reuses this rule and owns the end-to-end catch-up gate), then the
   adoption. Every block equals the
   move run, and A equals its uncopied twin. A second case leaves one unread compressor value in A's
   cell and one Observe record in its FIFO at the copy, then renders at least two catch-up blocks
   before the adoption: the copy succeeds, B applies the value at its first rendered block,
   every adopted block equals the move run, and A, rendered on, equals its uncopied twin bit for
   bit, observation readings included.
7. **Realtime.** Every swap block and copy call makes zero allocations and frees.
8. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && bash scripts/trace-effect-contract-audit.sh target/release/audit`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that drops the predecessor's unread cells, or stages them into the retired plan,
  renders `v1` forever. It turns red.
- Gate 2: a payload that re-derives a ramp's step moves bits when the ramp crosses the swap. It
  turns red at quantum 32; every other gate runs at 128.
- Gate 3: a carried value that overrides the successor's newer cell applies an old value, and a
  miscounted carry moves the exact count. A copy-mode carried slot that counts as in move mode
  counts the superseded values twice (+78, not +39). Any of these turns it red.
- Gate 4: a shunt copy that ignores the shared cursor turns it red.
- Gate 5: a retarget that loses to the carried value applies the old value last. A missing retarget
  keeps the old threshold. Either turns it red.
- Gate 6: a copy that consumes the predecessor's unread value or Observe record moves A's bits or
  readings. A copy that skips them loses the value in B. A carried slot that waits for the live
  lanes to open applies the value at S instead of B, so the adopted blocks differ from the move
  run. Any of these turns it red.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279).
- *Hold live values in latest-target cells on both hosts* (#1312): `peek_unread`, which D2 reads.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): this slice
  carries the cells #1345 creates, so #1345 lands first.

Dependent: *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355) keeps
its live lanes closed until adoption (its D4) and resolves carried slots by this issue's D2 rule in
its `render_slice` blocks; it lands after this issue.
