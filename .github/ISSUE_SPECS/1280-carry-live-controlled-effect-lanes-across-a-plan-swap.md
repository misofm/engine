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
- On `main` before #1345, `EffectControlLane` (`live.rs:125`) held a queue consumer, a
  prepared-target FIFO, the live `bypass` flag and the live symmetry terms. **After #1345
  (root note, 2026-10-12, stream B batch-3 verdict): the observation queue's consumer is no longer
  in `EffectControlLane`. It lives in `LiveControlEffectBankStage::observe_queues`
  (`crates/rack/src/lib.rs` ~1162, taken out of each lane at bind, drained at ~1530).** Anything
  below that says the carry reaches the observation FIFO through `EffectControlLane` reaches it
  through that stage field. Carrying a queue would need an inherited
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
- **D3. Pending Observe records.** (After #1345 the FIFO's consumer is
  `LiveControlEffectBankStage::observe_queues`, not `EffectControlLane`; root note, 2026-10-12.)
  The carry drains the predecessor's observation FIFO with the
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
   - `cargo run --locked --release -q -p realtime-policy`, `bash scripts/check-workspace-policy.sh`,
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

## Amendment (decision 17, root 2026-10-13)

**Split.** Both pieces stay in this issue.

- **(a)** D2 (carried cells) and D4 (carry, then retarget). Gates 1, 2, 3, 5 and 6.
- **(b)** D5 (shunt) and D3 (pending Observe records). Gate 4 and the new gate 8. Dependency: (a).

**Gate 8 (D3), new.** In `crates/host-core/tests/successor_swap.rs`: a compressor insert with
live controls. After A's last block and before B is published, admit Observe records to the
lane's observation queue (`LiveControlEffectBankStage::observe_queues`): one subscribe, and in a
second case the queue filled to its capacity. At the carry, every record is applied to A's
`ObservationLane` with the bounded drain: A's queue reads empty, and A's lane holds the state that
A's next block would have reached by applying the same records. Zero allocations and frees on the
swap block. It turns red if the carry leaves records in the retired queue (an acknowledged
subscribe is lost) or drains past the queue's count at entry.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Checked against `12d4a4f9d`.

- **Anchors moved.** `LiveControlEffectBankStage` is `crates/rack/src/lib.rs:1154` (`new` `:1218`,
  `begin_block` `:1459`, `drain` `:1526`, `process_inner` `:1574`; fields `observe_queues` `:1165`,
  `shunt` `:1178`, `observations` `:1184`, `designed` `:1194`). `EffectControlLane` is
  `crates/effect-contract/src/live.rs:748` (`bypass` `:770`, `mix` `:773`, `mix_position` `:776`,
  `symmetry` `:792`), `stage` `:1035`, `ObservationLane` `:1321`, `BypassShunt` `:1621`.
  `classify_live_delta` is `crates/host-core/src/live_delta.rs:324`; the per-instance derivation D4
  extracts is `effect_records` (`:1689`), `parameter_records` (`:1743`) and `design_targets`
  (`:1793`). #1279's carry is `graph::bank_effect_lanes` and `install_effect_carry`
  (`crates/graph/src/lib.rs:3229`, `:3246`), `Runtime::carry_effect_lanes`
  (`crates/graph/src/runtime.rs:4726`), `EffectBankStage::carry_lane_from`
  (`crates/rack/src/lib.rs:945`), `join_chains` and `resolve_effect_carry`
  (`crates/host-core/src/prepare.rs:3596`, `:3683`).
- **This slice is now the production effect carry.** Since #1555 and #1381a, both hosts attach
  `controls::live_lanes(model)` on every plan, and `LiveLanes::effects` is true for any session with
  an effect that has a live row. So every effect lane of a production plan is a
  `LiveControlEffectBankStage` lane or a per-node `LiveControlEffect`. #1279's carry
  (`EffectBankStage` only) is reached only by test entries that attach no effect lanes. Today every
  structural transaction on either host restarts each strip that holds a console slot or an insert.
- **D1 for decision 17.** A difference is live exactly when the per-instance function of D4 returns
  records for it, and prepared when it refuses it (a rebuild-only row: the delay and multiband
  bypass, a `Sample`-rate parameter, the link mode; a structure change; a domain error). The join
  calls that one function, as the strip join calls `fader_records` (#1277 D5). `same_console_entry`
  (`prepare.rs:3550`) stays for the non-live stage only. A move also needs the same stage kind in
  both plans (D1's "in both or in neither"): `GraphEffectLane` and `EffectLaneInventoryRow`
  (`prepare.rs:723`) gain the kind.
- **Banked inserts are in this slice.** No slice carries an insert that banks in both plans: #1281
  is bank to per-node, #1282 is per-node to per-node, and `resolve_effect_carry` joins console
  owners only. `bank_effect_lanes` already lists banked insert lanes under `RackId::Dynamic`, and
  gate 4 uses a banked insert. So the join adds the owner key (strip, `Dynamic`, insert ID) for a
  surviving insert that is banked in both plans.
- **D2 gains #1341 D7.** `carry_from` also copies `mix` and `mix_position` (#1341a1, landed in batch
  9), so a swap inside a bypass crossfade continues the ramp. #1341's gate 6 runs here.
- **D2, where the carried slots resolve.** `stage` returns early when no dirty bit is set
  (`live.rs:1070`). The carried slots resolve before that return. Carried parameter and target
  words go through the same per-group `applied` arrays as cell reads, so the #1004 pairing rule
  and the witness admit them exactly as cell reads.
- **D2, the count.** Counting a carried value needs one engine method, which the owner approved on
  2026-10-14 (one attempt; if the permission system refuses it, stop and report to root):
  `pub fn add_carried(&self, count: u64)` on `LiveValuesSuperseded`
  (`crates/engine/src/realtime/latest_cell.rs:946`), in the realtime-policy region beside the
  private `add` (`:982`), with the same body (load, saturating add, store; zero changes nothing).
  The doc of `add` changes to name it as the second render path that writes the count. It is the
  only `crates/engine/src/realtime` edit of this issue. Its unit test goes in `latest_cell.rs`'s
  tests.
- **D2, keys after #1603.** A cell slot is keyed by publication (`GateKey`), and a monitor tick
  publishes the committed revision again under a new publication. "Every predecessor value is
  `<= S`" reads "every predecessor write was published on the predecessor's gate before the
  successor was published". The carried slot resolves with no key compare across the two gates,
  so the rule does not change.
- **D4, the write.** The retarget writes through the successor's `EffectControlProducer` with
  `PreparedRenderPlan::stamp_held_gate(base.revision)` (`crates/engine/src/realtime/plan.rs:813`),
  in the block that writes the strip retargets (`prepare.rs:2644`). A bypass goes as a
  `BypassSwitch` with the session's `mute_ms` ramp (#1341 D1): `write_bypass` with
  `bypass_ramp_samples` until #1622 folds the ramp into the record. Parameter ramps come from
  `SuccessorBase::edit_ramps` (#1394) where the edit gave one.
- **The live-stage carry.** `LiveControlEffectBankStage` gains `layout()`, `can_carry_lane_from` and
  `carry_lane_from`: the payload through the plan's scratch as `EffectBankStage` does, then
  `EffectControlLane::carry_from`, then the shunt words (D5), then the lane's `designed` flag
  recomputed after the restore, as `EffectBankStage` refreshes its witness. `effect_stage_mut` and
  `effect_lane_mut` (`runtime.rs:4319`, `:4328`) reach both stage types.
- **Split (replaces the decision-17 split; root confirms).** Each piece stays in this issue.
  - **#1280a:** D2 (with #1341 D7), D5, the live-stage carry, the join for console slots and banked
    inserts, and `add_carried`. A live difference is treated as prepared until #1280c: the strip
    restarts, as today, and no value is lost, because the successor is prepared from the committed
    model. Gates 1, 2, 3, 4, 6 and #1341 gate 6.
  - **#1280c:** D4 (the extraction and the retarget). Gate 5.
  - **#1280b:** D3 (pending Observe records). Gate 8.
  - Reason: D5 moves into (a) because a carried crossfade on a latency effect reads the dry line
    in its first blocks after the swap, so (a) without D5 is not exact for #1341 gate 6 with the
    limiter. D4 can stand apart, because before it a live difference restarts the strip, which is
    today's behaviour.
- **Gate 7** gains `cargo test --locked -p engine` (the `add_carried` test). In gates 1, 3 and 5,
  "write to A's cells" means through A's `EffectControlProducer`, stamped on A's gate: the control
  plane is the only writer in production (decision 17).
- **Dependencies.** #1279, #1312 and #1345 are closed. #1341a1 and #1341a2 landed in batch 9.
  #1622 (open) edits the same bypass write in `live.rs` and `live_delta.rs`: if both are in one
  batch, #1622 lands before #1280c.

## Ruling (root, 2026-10-14)

The batch-10 split is approved: #1280a (D2 with #1341 D7, D5, the live-stage carry, the console and
banked-insert join, `add_carried`; gates 1-4, 6 and #1341's gate 6), #1280c (D4, gate 5), #1280b
(D3, gate 8). #1341 closes when #1280a lands. #1622 follows #1280a in batch 10 lane K and rebases
on it.
