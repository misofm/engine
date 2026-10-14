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

## Decision record: #1280a (batch 10)

**Shipped.** D2 with #1341 D7, D5, the live-stage carry, the join for console slots and banked
inserts, and `add_carried`.

- `LiveValuesSuperseded::add_carried` (`crates/engine/src/realtime/latest_cell.rs`): the same body as
  `add`, public, in the realtime-policy region. One attempt, accepted by the permission system. Its
  unit test is `add_carried_adds_saturates_and_ignores_zero`.
- `EffectControlLane::carry_from` (`effect-contract/src/live.rs`) copies `bypass`, `mix`,
  `mix_position` and `symmetry`, and gives each cell a render-owned carried slot (`CarriedCell`:
  the newest unread words and the unread count `s - p`). `stage` resolves the slots before its
  `any == 0` early return, in `Reads::read`: the successor's own `Applied` read wins and adds `n`;
  otherwise the carried words apply and add `n - 1`. A slot the predecessor had not resolved yet is
  merged (its count is added), so a successor that is replaced before its first stage loses nothing.
  `can_carry_from` compares the two cell shapes (`EffectLaneShape` equality). The slots are
  preallocated with the lane and counted in `retained_resources`.
- `BypassShunt::carry_line_lane_from` (D5): one lane's samples of the interleaved line, walked from
  each ring's own cursor in age order, so another width and another cursor are correct. It copies
  nothing when neither bank keeps a latency line, and refuses a line of another latency.
- `LiveControlEffectBankStage` gains `layout`, `lanes`, `lane_kind`, `can_carry_lane_from`,
  `carry_lane_from`, `carry_refused`, `as_any_mut`, and a `RESTORED` bit per lane (a byte, no new
  allocation). The payload move is one function (`carry_payload`) shared with `EffectBankStage`.
  `EffectLaneKind { Plain, Uncontrolled, BypassOnly, Cells }` is the "kind" of the join: a pair of
  unlike kinds does not carry.
- `graph`: `EffectStageMut` reaches both stage types in `effect_stage_mut` / `effect_lane_mut`;
  `GraphEffectLane` gains `kind`; the swap block's check and carry run for both.
- `host-core`: `EffectLaneInventoryRow` gains `kind`. The join adds the owner key (strip, `Dynamic`,
  insert ID) for a surviving insert (`InsertOwnerJoin`, bit-equal declaration in both models). A
  live difference is a prepared one until #1280c: the strip restarts, as today.

**Departures.**

- The live stage had no `RESTORED` witness term. A restore whose left and
  right sections differ must decline the mono collapse as `EffectBankStage` does, so the live
  stage holds the term now (not in the text of the issue).
- A lane kind that is not `Cells` carries its payload (and a lowered bypass) and no cells; the
  swap block never refuses the hand-over for a shunt that exists on one side only.
- Gate 3's count is per cell. The compressor threshold is a `PerLane` parameter with a left and a
  right cell, so the test writes both and the counts are 2 x 79 and 2 x 39.

**Gates and tests** (`crates/host-core/tests/successor_swap.rs`):

| Gate | Test |
|---|---|
| 1 | `unrendered_live_writes_survive_the_swap_at_{eight,four}_lanes` (EQ band gain target and compressor threshold) |
| 2 | `a_live_ramp_in_flight_finishes_after_the_swap_at_{eight,four}_lanes` (quantum 32, 64-sample ramp half done) |
| 3 | `the_newest_value_wins_and_the_count_is_exact_at_{eight,four}_lanes` (158 and 78, i.e. 79 and 39 per cell) |
| 4 | `a_bypassed_lane_keeps_its_delayed_dry_signal_at_{eight,four}_lanes` (banked limiter insert, nonzero cursor) |
| 6 | `assert_clean_swap` in every case above: allocator and render audit zero over the swap block |
| #1341 6 | `a_bypass_crossfade_in_flight_continues_after_the_swap_at_{eight,four}_lanes` (comp, and limiter with latency) |
| join | `a_changed_control_kind_carries_no_effect_lane_at_{eight,four}_lanes` |
| engine | `add_carried_adds_saturates_and_ignores_zero` |
| shunt | `carry_tests::a_shunt_lane_moves_to_a_bank_of_another_width_and_cursor` |

Red under mutation (each run once, reverted): no cell carry (gate 1), no `add_carried` and a
carried value that overrides the successor's (gate 3), no shunt copy and a copy that ignores the
cursors (gate 4 and #1341 6), no `mix` copy (#1341 6). The four-lane cases are `ignore`d on this
AVX2 host (a console bank binds at the width its build executes, #1017).
`a_console_slot_with_live_controls_restarts_its_strip` is deleted: a live console slot now carries.

**Found, not fixed (successors).**

- A route's PDC delay line is a state of its own that no carry moves. A session where one strip has
  an insert with latency and another has none gets a delay line on the second strip's route, and a
  swap restarts it: the first limiter-insert case failed on exactly that until every track had the
  insert. The gates use sessions without unequal latency.
- Pending Observe records of a carried lane are still lost at the swap (D3, #1280b).
- `cargo run -p realtime-policy` fails at the base on `hosts/host-web/tests/control_command_heap.rs`
  (`unsafe` outside the approved files); it is not from this slice.

## Decision record: #1280c (batch 10)

**Shipped.** D4 (the extraction and the retarget) and gate 5. After this slice a live difference on a
carried effect lane no longer restarts the strip: the lane carries, then the successor retargets it
to the new committed value.

- **One per-instance function.** `effect_instance_records` (`host-core/src/prepare.rs`, beside the
  strip derivation it mirrors) holds what `live_delta.rs` had in `effect_records`'s body,
  `parameter_records` and `design_targets`, and `same_params`; `live_delta.rs` keeps `locate` and
  `effect_records`, which now only adds the live address. It returns `Ok(None)` for no difference,
  `Ok(Some(EffectInstanceRecords))` for a live one and `Err(LiveRebuild)` for a prepared one
  (another identity, quality, sidechain or link mode; a bypass of an effect that does not lower it,
  by `lowers_session_bypass`; a parameter that is not `Block`-rate; a parameter that does not
  resolve; a ramp the indexed law refuses). The join calls it per surviving instance through
  `join_chains`, so a difference is live in the carry exactly when it is live on the live path.
  `same_console_entry`, `same_insert` and `committed_chain` are deleted: the lowered pair
  (`SessionModel::lower_strip`) of both models is compared by that function.
- **Join states.** `ConsoleOwnerJoin` and `InsertOwnerJoin` hold `FamilyJoin<EffectInstanceRecords>`
  and the instance's `LiveEffectAddress`. A live difference of a slot or insert at another position
  is prepared (another address); a `Same` one still carries.
- **The write.** `resolve_effect_carry` takes the producer step as a callback, called only for a
  lane whose layout and kind already match: `prepare_effect_retarget` preflights every record on the
  successor's own producer (a record the lane has no cell for refuses the lane, and the strip
  restarts), and, for the EQ, begins the owner transaction, applies the edits and preflights the
  targets. The writes happen after `stamp_held_gate(base.revision)`, in the block that writes the
  strip retargets: `write_bypass` with a `BypassSwitch` at the session's `mute_ms` ramp
  (`bypass_ramp_samples`), `write` for a parameter record, `publish_candidate_targets` and
  `commit_owner` for the EQ. Nothing is written when preparation fails before that block.

**Departures.**

- Parameter ramps from `SuccessorBase::edit_ramps`: an effect parameter record has no ramp (the
  effect smooths it itself) and `EditRampRow` has no effect-parameter row, so only the bypass has a
  length, and it is the session's. Nothing to read from `edit_ramps` here.
- The test gate's successor helper (`LivePlan::successor`) now takes the commit's gate steps (stamp
  the revision again, publish it), as `CellPlan::successor_at` already did. Without them a retarget
  stamped at preparation is never published on the successor's gate.

**Gate and tests** (`crates/host-core/tests/successor_swap.rs`, `crates/host-core/src/prepare.rs`):

| Gate | Test |
|---|---|
| 5 | `a_live_difference_on_a_carried_lane_is_retargeted_at_{eight,four}_lanes`: a compressor threshold, an EQ band gain and a compressor bypass, each on three carried lanes. Run 1 is the structural swap with the retarget, run 2 writes the same edit to A before the swap and prepares B from a base that holds it. Bit-identical in every block, `restarted` empty, a clean swap block (zero allocations and frees), and not vacuous against a swap that leaves the lane at its old value. |
| join | `the_per_instance_function_separates_live_from_prepared`, `a_live_difference_carries_only_when_the_successor_accepts_its_records` |
| classifier | `a_changed_effect_row_without_a_record_is_a_rebuild` (unchanged, through the moved function) |

The four-lane case is `ignore`d on this AVX2 host (#1017), as the #1280a cases are.

**Found, not fixed.** `ChainJoin::may_carry` ignores surviving inserts, so a strip that could carry
only inserts builds no `carry_program` for sources; the insert carry installs its own table and
works, so nothing is wrong today, but the name says more than it checks.

### `audit capi` digest (batch 10)

The C ABI render audit commits one structural transaction after its first call (it adds a muted track, `tools/audit/src/capi.rs`). Before #1280a that swap restarted every strip with a console EQ slot; now the slots carry their state across it. So `audit capi`'s `pcm_digest` moves from `76525e7ef9e78cf4` (every batch through 9) to `9bc5ec3d8a4c223c`, measured at `f0e387307` and its parent `869ffb78b`; allocations, frees and render errors stay 0. CI checks the digest's format only.
