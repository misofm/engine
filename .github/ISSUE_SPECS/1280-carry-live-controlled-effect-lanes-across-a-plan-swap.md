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
(D15-7).

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
  - #1312 D1-D2 make each cell a revision-bounded three-slot cell (#1432 D1, Amendment 1). Each
    slot carries its revision and a sequence. Render applies, per cell, the newest value whose
    revision is `<=` the block's snapshot `S`, and keeps the last sequence it applied (`p`).
    Applying sequence `s` adds `s - p - 1` to `live_values_superseded`.
  - #1432 D1 also gives the reader `peek_unread`. It returns the newest unread words and their
    sequence without consuming them, and is valid once the writer is quiescent.
  - The control plane writes only the newest plan's cells: the pending candidate's if there is
    one (D15-17; #1053 D7). So no control write reaches a predecessor's cells once its successor is
    published, and render reads them whole.
- The slot's dry shunt is one interleaved AoSoA line for the whole bank, with one shared cursor
  (`shunt`, `rack/src/lib.rs:956-958`; `BypassShunt`, `live.rs:868-890`). Observation taps are
  `ObservationLane` (`live.rs:588`), one per bank lane (`rack/src/lib.rs:961`).
- The console-lane carry through the payload scratch, the effect rule D1 and the refusal counter
  come from *Carry console effect lanes across a plan swap* (#1279). The base, the retarget pattern
  and the restart set come from #1277 (D2-D6).

## Decisions frozen for this slice

- **D1. Rule.** As #1279 D1, with live controls attached in both plans or in neither. Live
  parameter values and a lowered bypass are live, because the lane exists. Everything #1279 D1
  lists as prepared stays prepared.
- **D2. Carried cells.** `EffectControlLane::carry_from(&mut self, predecessor:
  &EffectControlLane)` takes the predecessor by shared reference and writes only the successor
  lane:
  - it copies the applied live `bypass` flag and the live symmetry terms;
  - for every predecessor cell where `peek_unread` (#1312 D1) returns `Some((words, s))`, it stores
    the words in the successor lane's render-owned **carried slot** for that cell, with the unread
    count `n = s - p`, where `p` is the predecessor cell's last applied sequence. The writer is
    quiescent: no control write reaches a predecessor's cells once its successor is published;
  - it consumes nothing: the predecessor never renders again, so nothing reads its cells after the
    carry;
  - it writes no cell, so the control thread stays each cell's only writer.

  Each carried slot resolves at the **successor's first rendered block**, before that block stages
  anything. The carried slot is render-owned state, like a ramp in flight. The predecessor would
  apply the same value at its next block, which is the same boundary B, so the adopted output is
  exact. It resolves in #1345 D3's order:
  - if the successor's own cell's read under the adoption block's snapshot is `Applied` (an
    unread value whose revision is `<= S`), that value is newer (written by D4's retarget or after
    the successor's publication). It applies;
  - otherwise the carried value applies, and a successor value whose revision is `> S` stays
    pending for a later block (Amendment 1).

  Every predecessor value is `<= S`: every write to the predecessor carries a revision below the
  successor's revision (#1312 D6).

  Counting (#1312 D2, one unit, each superseded value once): the predecessor never reads again,
  so the successor counts for it. It adds `n` when its own newer value wins, and `n - 1` when the
  carried value applies. A successor's own cell counts only its own values, as every cell does.

  The carried slots are preallocated with the lane, one per cell, and cleared after that block.
  Nothing is refused and nothing is dropped.
- **D3. Pending Observe records.** The carry drains the predecessor's observation FIFO with the
  bounded drain and applies each record to the predecessor's `ObservationLane`, at the boundary
  where the predecessor's next block would apply it. Observation output is not audio, and the
  observer carry (*Carry meter and effect observation state across a plan swap*, #1327) then
  carries the tap state.
- **D4. Carry, then retarget (D15-7).** The per-effect value derivation inside
  `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) is extracted into one per-owner
  function, as #1277 D5 did for strips. For every carried lane whose live values differ, the
  successor entry points write its cells before they return: the bypass cell, the parameter cells,
  or an EQ's target cells designed from the base values. A cell write cannot fail (#1345 D5). The
  writes happen exactly once, at preparation (#1277 D5).
- **D5. Shunt.** A carried lane's shunt words move with it, as a copy of that lane's words of the
  interleaved line, taken relative to the shared cursor of each bank.
- **D6. Acked-batch question: can an ack ever precede a drop? No.** This slice adds no admission
  check and no refusal. Every acked value is in a cell of the plan it was written to. A carried
  value reaches the successor through D2, and a value superseded there is counted exactly.

## Deliverables

1. D2 and D3 in `crates/effect-contract/src/live.rs`. D4 in `crates/host-core`. D5 in
   `crates/rack`. The location table and the program section in `crates/graph`. The inventory rows
   and the join in `crates/host-core`.
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
   second case writes nothing to B: the carried value applies and the counter grows by 39.
4. **Live bypass and shunt.** A true-peak limiter insert on four tracks (a full bank at
   `Backend::Simd4`; eight tracks at `Backend::Simd8`), with one lane bypassed live before the swap
   and the cursor at a nonzero offset. The bypassed lane keeps its delayed dry signal across the
   swap.
5. **Carry, then retarget.** B also changes a carried lane's compressor threshold and an EQ band
   gain (live values). Run 1 is the structural swap with D4's retarget. Run 2 writes the same edits
   to A just before the swap and prepares B from a base that holds them. The two runs are
   bit-identical in every block, and the strip is not in the restart set.
6. **Realtime.** Every swap block makes zero allocations and frees.
7. Commands:
   - `cargo test --locked -p effect-contract -p rack -p graph -p host-core --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit -p bench && bash scripts/trace-graph-audit.sh target/release/audit && bash scripts/trace-effect-contract-audit.sh target/release/bench`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a carry that drops the predecessor's unread cells, or stages them into the retired plan,
  renders `v1` forever. It turns red.
- Gate 2: a payload that re-derives a ramp's step moves bits when the ramp crosses the swap. It
  turns red at quantum 32; every other gate runs at 128.
- Gate 3: a carried value that overrides the successor's newer cell applies an old value, and a
  miscounted carry moves the exact count. Either turns it red.
- Gate 4: a shunt copy that ignores the shared cursor turns it red.
- Gate 5: a retarget that loses to the carried value applies the old value last. A missing retarget
  keeps the old threshold. Either turns it red.

## Amendment 1 (root, 2026-10-05): revision-bounded cells

Root's binding requirement (2026-10-05) makes every live value of one revision take effect in one
block (#1432 Amendment 1, #1312 Amendment 2). Two parts change:

- the Context bullet on #1312 D1-D2;
- D2's resolution, which now reads the successor's own cell under the adoption block's snapshot.

The counting rule is unchanged.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279).
- *Hold live values in latest-target cells on both hosts* (#1312): `peek_unread`, which D2 reads.
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): this slice
  carries the cells #1345 creates, so #1345 lands first.

Under a latency growth, the warm successor adopts in move mode at its first ready block `S`
(*Grow latency during playback by adopting a primed warm successor*, #1287). That is an ordinary
move-mode carry at a later boundary, so D2's carried slots resolve at `S` by the same rule, and this
slice's gates cover the rule.
