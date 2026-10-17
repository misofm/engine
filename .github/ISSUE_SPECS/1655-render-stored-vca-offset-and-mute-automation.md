# Render stored VCA offset and mute automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), root's ruling on finding F20 (2026-10-14, README "Root's rulings (2026-10-14)"),
under decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, C3, C6 and C7). A draft for
root to file. Written by the controls coordinator on 2026-10-14. Code anchors verified on
`2e6343a99`. Batch R2, after #1651-#1653.

## Product outcome

A producer automates the "vocals" VCA: an `offset_db` ride and `mute` steps. Every member lane
plays the ride and the steps on every host, in the member's own node time, so a member behind a
latent insert hears the VCA move with its own audio. Sends that follow a member's mute follow the
automated VCA mute at their tap. A hand move of an automated VCA offset writes a stored dB offset,
and a hand VCA mute ORs with the automated VCA mute (D5). A VCA automation edit is a carried
rebuild. A session with no VCA automation renders the bits it renders today.

## Context

- **The rows.** A VCA has two rows, `offset_db` (1) and `mute` (2), both per lane, family
  `StripLevel`; `offset_db` has the fader domain and the ramp row `VcaOffset`, `mute` is a toggle
  with the ramp row `VcaMute` (`crates/controls/src/vca.rs:13-72`). A VCA has no render cell: a
  change reaches render through its members' effective fader and mute (#1558). `automatable` is
  computed (`crates/controls/src/descriptor.rs:124-132`), so both rows are targets today, and
  `fixtures/session/v1/builtins-automation.json` holds the entry `vca-ride` (target owner `vca`,
  `riders`). Nothing renders it: the classifier masks the whole table
  (`crates/host-core/src/live_delta.rs:359-361`).
- **The fader composition.** #1646 D6 (README F16, root's decision): a member's gain is
  `checked_fader_gain(vca_compose_db(member, S))`, `S` the `f64` sum of the reaching offsets in
  ascending VCA ID, offsets first. #1646 D1 gives each automated fader lane an offsets cell (`S`
  as one `f64`, one ramp word); #1649 writes it on every VCA edit; #1650 puts the lane's hand
  offset first in `S`.
- **The mute composition.** #1651 (amended): the stage mute of an automated mute lane is
  `curve || terms`, `terms = static_mute || reaching VCA mutes || solo term` in one cell word.
  #1652 (amended) gives a following send of an automated source lane a seven-word route cell
  and evaluates the source curve in the tap's node time.
- **The reach.** Nested VCAs reach members through `session::vca`; the host caps the reach pairs
  (`maximum_vca_reach_pairs`; the browser's `MAXIMUM_VCA_REACH_PAIRS`).
- **The shape rule.** Amended #1645: a toggle row takes only `step`.

## Decisions frozen for this slice

- **D1. Programs.** Each automated VCA lane compiles once, with #1637's builder: `offset_db`
  with grid period and ramp 64, completion `End`, jump row `VcaOffset`; `mute` with jumps only,
  row `VcaMute`. The segment table is plan-owned once per VCA lane. Each member lane the VCA reaches
  holds its own cursor and event state for it and evaluates it in the member's fader node time
  `a(fader)` (#1647), because a VCA has no audio of its own.
- **D2. Offset composition.** A member lane's fader stage runs events when its own fader is
  automated or when an automated VCA offset reaches it ("event lanes"). Events are the union of its
  own curve's events and each reaching automated VCA's events. At each event:
  `S_t = S + c_1(τ) + … + c_m(τ)` in `f64`, the `c_k` the reaching automated VCAs' curve values in
  ascending VCA ID, then `gain = checked_fader_gain(vca_compose_db(own, S_t))`. `S` (the cell) holds
  the lane's own hand offset, then the static offsets of the reaching VCAs that are not automated,
  then the hand offsets of the reaching automated VCAs, each group in ascending VCA ID. `own` is
  the member's curve value when its fader is automated, else the cell's `own_db` word (D3).
  - A session with no VCA automation has no `c_k` and no `own_db` word: the composition is
    #1646's and #1649's, and no bit moves.
  - A flat VCA curve at `v` gives the bits of a static VCA offset `v` whenever F16's exactness
    condition holds (each nonzero term at least `k·2^-20` dB, so every partial sum is exact); the
    order "static terms, then automated terms" can differ from ascending-ID order only in the
    degenerate cases F16 lists. Root records it as a class B order under the standing
    summation-order ruling, with F16.
- **D3. The cell of a member that is not automated.** An event lane whose own fader is not
  automated gets #1646's offsets cell plus one word, `own_db` (the member's static fader dB,
  `f32`): four words, 48 bytes in #1312's three slots. A hand fader move on that lane is a live
  value, as today, but the classifier writes `own_db` through the cell (#1649 D4's write, ramp
  `LiveRamps::resolve(Fader, the edit's ramp)`) instead of a `FaderDb` record, and render jumps at
  the next block entry only if the gain bits change (#1649 D3).
- **D4. Mute composition.** A member lane reached by an automated VCA mute uses #1651's
  `terms` word whether or not its own mute is automated. Its stage mute is
  `own_curve || terms || v_1(τ) || … || v_m(τ)`, the `v_k` the reaching automated VCA mute curves
  in the member's node time, with `own_curve` false when the own mute is not automated. `terms`
  holds the static mute, the static mutes of every reaching VCA (automated or not: D5) and the solo
  term. Events are the curves' discontinuities; only a change emits #1638's `SetMute`, over the
  `VcaMute` jump row for a VCA curve event. A following send of such a member uses #1652's
  seven-word cell, and its follow term ORs the same `v_k` evaluated in the tap's node time
  `τ_tap`, so the send closes on the timeline sample where the member mutes (README A6).
- **D5. Hand edits** (OQ1 = C with root's ruling on level controls).
  - `SET_CONTROL` on an automated VCA `offset_db` writes the entry's stored offset (#1650's
    field; `takes_offset` gains VCA `offset_db`, per lane), and the classifier rewrites `S` of every
    member lane the VCA reaches (#1649 D4's write). This reads root's ruling "send gain is a
    level control, so it takes an offset" as covering the VCA offset, a level control too; root
    confirms at filing. If root reads it otherwise, the edit stores the fallback and commits
    `model_only` (#1648 D3), and gate 4's offset case expects that.
  - `SET_CONTROL` on an automated VCA `mute` ORs: the VCA's static mute stays live and is written
    into `terms` of each reached member lane (#1651's terms write).
  - Edits of a VCA that is not automated, and reach changes that involve only such VCAs, stay
    #1649's live `S` write.
  - A change of which member lanes an automated VCA reaches (a membership or nesting edit, adding or
    removing an automated VCA) needs new event state for the new pairs: a carried rebuild, for the
    new-memory reason (decision 14 rule 1). It narrows D15-6's live set for automated VCAs only;
    root sees it at filing.
- **D6. Rendering rows.** VCA rows 1 and 2 join #1648 D1's `automation_row_renders`. A VCA
  automation edit is a carried rebuild; each member pair's event state carries by `(VCA ID, row,
  member strip, lane)`, as #1648 D2 carries a fader cell.
- **D7. Memory.** Per automated VCA lane one program (`32 + 32·n` bytes, A4's header and
  segments); per (member lane, automated VCA lane) pair one event state (48 bytes); per event lane
  whose own fader is not automated 12 bytes more (D3's word in three slots). Every term is bounded
  by the session and the host's reach-pair cap; none reads the song length.
- **D8. Work.** A member lane takes at most `(1 + m)(2⌈q/64⌉ + 3)` fader events per block, `m` the
  automated VCA offset lanes that reach it, and `m` more `f64` additions per event.
- **D9. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; every check of a transaction precedes the first cell write; a cell write cannot fail.

## Deliverables

1. D1-D4 in host-core preparation (programs, member pairs, cells and seeds), the fader stage's
   events and mute terms, and the route op's follow term.
2. D5-D6 in `crates/controls` (`takes_offset`, the rendering rows) and the classifier.
3. `docs/SESSION_SCHEMA_V1.md`: VCA rows render; the composition order of D2.

## Authorized paths

- `crates/host-core/src/{prepare.rs,live_delta.rs}`, `crates/host-core/tests/{vca.rs,live_delta.rs}`
- `crates/builtins-compiler/src/lib.rs` (the fader stage's VCA events, the `own_db` word and the
  mute terms only)
- `crates/graph/src/{lib.rs,runtime.rs}` (the route op's follow term only)
- `crates/controls/src/{automation.rs,vca.rs}` and their tests
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `crates/control-plane/tests/vca_automation_realtime.rs` (#1649's binary, extended)
- `docs/SESSION_SCHEMA_V1.md`

## Non-goals

- A VCA with an audio path; VCA solo.
- Any change to a session without VCA automation (D2).
- Sends' own rows (#1654).

## Hazards

- **Class B order.** D2's order equals ascending-ID order except in F16's degenerate cases; root
  records it with F16. Gates use values on the 0.1 dB lattice.
- **Hot files.** `crates/host-core/src/prepare.rs` and `crates/builtins-compiler/src/lib.rs` are
  shared with #1651-#1654; root sequences the merge.

## Objective gates

Commands: `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p
builtins-compiler --features test-support`, `cargo test --locked -p controls`, `cargo test --locked
-p graph --features test-support`, `cargo test --locked -p capi`, `cargo test --locked -p host-web
--features test-support --test control_parity`, `cargo test --locked -p control-plane --features
test-support`, `./target/release/realtime_policy`, `bash scripts/check-cross-targets.sh` (README
F19), clippy `-D warnings`, `cargo fmt --all -- --check`.

1. **Flat equals static** (C ABI and browser). A flat VCA offset ride at `-6` dB and a flat VCA mute
   at 0 render, bit for bit, the session with those static values: for a member with its own fader
   ride and for a member with none.
2. **Member node time.** A linear VCA ride reaches two members, one behind the true-peak limiter.
   Each member's gain at each grid completion equals `checked_fader_gain(vca_compose_db(own, S +
   c(τ)))` computed on the test thread, and the latent member's events fall `L` samples later in
   render time.
3. **Mute.** A VCA mute step at a sample that is not a grid sample mutes each member at its node
   time over the `VcaMute` ramp; a following send of a member closes at its tap (#1652's rule);
   after the ramps the output equals a static-mute plan.
4. **Hand edits.** On an automated VCA: `SET_CONTROL offset_db = +3` gives `live`, stores the
   offset and, after the jump, equals a fresh plan with that offset; a hand mute ORs (closes an
   open curve, leaves a closed curve closed). A hand fader move on a member with no own automation
   gives `live` and one `own_db` write, no `FaderDb`. A membership edit of the automated VCA gives
   `rebuild` and completes `exact`; the same edit of a VCA that is not automated gives `live`.
5. **Realtime.** The events, cell reads and writes allocate and free nothing on the render thread
   (`bench_support::alloc` thread counters).
6. **No rendered bit moves** for a session without VCA automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1 turns red if the VCA curve composes with other arithmetic than the static path.
- Gate 2 turns red if VCA events use the render clock or the VCA's own (non-existent) arrival
  instead of each member's.
- Gate 3 turns red if a following send of a member keeps passing audio under an automated VCA mute.
- Gate 4 turns red if a hand VCA move writes the fallback, if a plain member's hand move fights the
  VCA curve with a `FaderDb` record, or if an automated VCA's reach change commits live with no
  event state for the new pair.

## Dependencies

- #1651, #1652 and #1653 (the terms word, the seven-word route cell, the follow evaluation).
- #1649 and #1650 (the offsets cell, its write and the offset field), #1648 D1
  (`automation_row_renders`), #1639 (jump rows), amended #1645 (the shape rule), #1647
  (member node time).
- Batch R2, after #1653. A complete outcome alone; not in a must-land group.

## Ruling (root, 2026-10-14)

- **Hand move.** A hand move of an automated VCA `offset_db` writes an offset, the same rule as
  fader, trim and send gain (a level control).
- **Summation order.** Static terms are summed first, then the automated VCA curves: a class B
  change under the standing summation-order ruling, recorded with F16.
- **Membership of an automated VCA.** A change to which members an automated VCA reaches is a
  carried rebuild. Decision 14, rule 3 entry, for automated VCAs only: the reach change needs new
  memory (an `own_db` cell word per newly reached lane and the member's curve tables), so it cannot
  be a live update. Non-automated VCAs keep D15-6's live set.

## Amendment (controls coordinator, batch 19 spec check, 2026-10-17)

Checked against `323433f6a`. #1646-#1653 have landed (batches 17 and 18). The filed text stays;
where it disagrees, this section wins. Rulings R19-V1 and R19-V2 below are open; the rest are fixes.

**Anchors now.**
- VCA rows: `crates/controls/src/vca.rs:14` (`OFFSET_DB`), `ROWS` `:37-73`, `write` `:111`.
  `automatable`: `crates/controls/src/descriptor.rs:154`. The classifier masks the table at
  `crates/host-core/src/live_delta.rs:404`; step 3 is `:422-425`.
- `controls` (`crates/controls/src/automation.rs`): `takes_offset` `:51`, `automation_row_renders`
  `:400`, `mute_row` `:407`, `fader_row` `:419`, `automated_fader_lanes` `:502`,
  `rendered_automation_changed` `:527`, `continued_fader_lanes` `:573`, `continued_mute_lanes`
  `:586`. `effective_strip` `crates/controls/src/effective.rs:326-366` (`offsets_sum` `:207`). The
  hand-offset write: `crates/controls/src/registry.rs:565` (`builtin_rows::write_fader_offset`),
  `ModelView::fader_automation` `crates/controls/src/view.rs:111`,
  `OwnerEdits::take_fader_automation` `crates/controls/src/owner_edits.rs:105`,
  `mask_fader_offsets` `crates/controls/src/mask.rs:138`.
- builtins-compiler (`crates/builtins-compiler/src/lib.rs`): `OFFSETS_CELL_WORDS` `:719`,
  `OffsetsLane` `:727`, `AutomatedFaderCell` `:753`, `AutomatedMuteCell` `:839`,
  `LaneFaderAutomation` `:858` (`adopt` `:956`, `mute_terms` `:1062`, `read_offsets` `:1128`),
  `offsets_jump` `:1285`, `block_jumps` `:1404`, `FaderCellCursor` `:1458`, `FaderEventDriver`
  `:1709` (`new` `:1755`), `settle_adoption` `:2146`, `prepare_fader_automation` `:2618`,
  `fader_automation_sizes` `:2582`, `strip_mute_cells` `:5808`.
- `automation::CellState` and `CellEvents`: `crates/automation/src/events.rs:91`, `:217`.
- host-core (`crates/host-core/src/prepare.rs`): `MAXIMUM_VCA_REACH_PAIRS` `:144`, the following
  route attach `:2873-2897`, `fader_records` `:4493`, `level_records_between` `:4567`,
  `StripChange` `:4637`, `join_level_records` `:4928`, `following_route_lanes` `:5081`,
  `join_strips` `:5638`, `join_routes` `:5829`.
- graph: `RouteFollowFlags` `crates/graph/src/lib.rs:1146`; `LiveFollowRoute`
  `crates/graph/src/runtime.rs:1287`, `FollowCursors` `:1442`, `follow_walk` `:1649`.
- B17-R14's `automates`: `crates/control-plane/src/control.rs:2651`.

**How the slice maps onto the landed code.**
- **Event lanes.** A member lane is a fader event lane when its fader is automated or an automated
  VCA `offset_db` lane reaches it, and a mute event lane when its mute is automated or an automated
  VCA `mute` lane reaches it. Two new `controls` predicates give them (one rule for preparation,
  the classifier, the join and the route attach). The own-entry readers of #1651 T2
  (`fader_automation`, `model_fader_offsets`, `mask_fader_offsets`, the listing flags) stay
  own-entry only: a member's own fader is not automated by a VCA ride.
- **Program.** The VCA cells join the plan's one program after the mute cells, so a session
  without VCA automation builds the same program. One cell per automated VCA lane, shared by
  every member. `prepare_fader_automation`'s early return already reads `automation_row_renders`.
- **Slot.** An event lane gets a `LaneFaderAutomation` slot and an offsets cell, also when its own
  fader is not automated. Its channel's own source is the cell (own automation) or `own_db`.
- **Driver.** The one `FaderEventDriver` (bank widths, fused pair fallback, scalar oracle: R17-4).
  Same `a(n)` and span (R17-2, R17-3); no new node-time context. `block_jumps` also resolves
  `LiveRamps::for_row(VcaOffset)` and `for_row(VcaMute)` (equal to the fader and mute lengths
  today; resolved by row, not assumed).
- **Mute.** `terms` is already `EffectiveStrip::mute` (static mute, every reaching VCA's static
  mute, solo term). So D4 needs no new record and no classifier change: the `Mute` record is the
  terms write, and `mute_terms` (the drain branch) composes it with the lane's composed curve.
- **Route.** `following_route_lanes` (the one predicate of the attach and of #1653's record) widens
  to mute event lanes. `RouteFollowFlags` keeps its six bits; `automated[lane]` now means "the
  lane has a mute curve, own or VCA". No spare bit is used (#1654 keeps bits 6 and up). The route
  op evaluates the reaching VCA mute cells at `τ_tap` with its own cursors (`FollowCursors`,
  `follow_walk`). `route_mix_column_ramps_block` and the per-column ramps (R18-5) do not change.
  #1653's pre-write checks hold: plan and record read the same predicate.
- **FP (R17-7).** The `f64` sums of D2 run on render inside the render entry's `CanonicalFpEnv`;
  the seed runs inside preparation's guard; the classifier's `S` inside the commit guard.
- **Classifier.** VCA rows 1 and 2 join `automation_row_renders`, so a VCA entry change is
  `Err(LiveRebuild::Automation)` at step 3. The reach rule (D5 last bullet, root's ruling) goes in
  `controls::rendered_automation_changed`: it also returns `true` when the `(member strip, lane)`
  set that an automated VCA lane reaches differs (membership, nesting, an added or removed VCA).
  Step 3's call in `live_delta.rs` does not change (docs only). The fast path (`classify_rows`)
  takes `SetControl` only, which cannot change a reach, so it needs no change.
- **Hand edits (D5).** `takes_offset` gains VCA `offset_db`. As for the fader (#1650 D3, D4,
  R18-2): `registry::write` sends a hand move on an automated VCA lane to the entry's offset;
  `ModelView` and `OwnerEdits` read and take the VCA entry for the in-place edit (R17-1);
  the VCA row's value is the offset and the list marks it `automated` (field 35); `mask_values`
  copies the entry's offset and leaves the automated lane's static offset (its fallback) as
  structure. Without these a write and a read of the same row disagree.
- **`S` (D2).** `effective_strip` puts, on a fader event lane: the own hand offset, then the static
  offsets of the reaching VCA lanes that are not automated, then the hand offsets of the automated
  ones, each group in ascending VCA ID. With no automated VCA lane this is today's sum, bit for bit.
- **Records (D3).** `level_records_between` and `fader_records` take the fader event lanes as
  `automated` (no `FaderDb` there). The offsets record of an event lane carries `own_db` from
  `StripChange::own_after` (no new `EffectiveStrip` field).

**Fixes.**
- **D3 and D7 bytes.** `LatestCell<N>` is const-generic. A slot of three words and a slot of four
  words are both 32 bytes (two `AtomicU64` set the alignment), so a four-word cell costs nothing
  more. Every offsets cell becomes four words (`OFFSETS_CELL_WORDS = 4`; the `own_db` word is
  unread on a lane whose fader is automated). One cell shape, one reader. No edit under
  `crates/engine/src/realtime/`. `offsets_cell_bytes` stays 120 (x86-64); the gate measures it.
- **D7 terms.** D7 leaves out that an event lane with no own automation also takes a slot
  (`lane_slot_bytes`) and an offsets cell. The per-pair term depends on R19-V1. All terms are
  bank payload, charged by `fader_automation_sizes()` and checked as ceilings, never literals.
- **D6 carry.** A carried channel continues verbatim when its own table and each reaching automated
  VCA table (in ascending VCA ID) are the same set, bit for bit, and `a(n)` did not move; else it
  repositions. This is `continued_cells` (bits 0-3) from the widened `continued_fader_lanes` and
  `continued_mute_lanes`; graph passes the byte unchanged. No per-pair carry key is needed, because
  a repositioned channel places each cursor again. A lost event lane is released by the join, as
  today (`join_level_records`, widened to event lanes).
- **Reposition and #1711 (R18-4 withdrawn).** A VCA offset table edit repositions every reached
  member channel. Its landing can cut a live move in flight (a member's `own_db` move, a VCA hand
  move), as a fader table edit does. So: land #1711 first and use its deferred jump for every
  repositioned channel; a VCA case joins its G1 here (gate 7 below). If #1711 does not land, the
  B17-R14 flag stays and `automates` (`control.rs:2651`) widens to VCA `offset_db` entries, so
  a VCA-only session's table edit is flagged, never `EXACT` when wrong. A VCA mute table edit
  lands only a `SetMute` (the stage keeps `fader_gain`, #1651 T4); U1 fold 1's rule must hold
  for the composed mute: a terms record at the swap lands over its own ramp word (gate 8).

**R19-V1 (ruling needed): how a member channel holds and walks its curves.** D1's per-pair state
(one `CellState` per member and VCA lane) cannot hold a correct composition. Each `CellState` keeps
its own ramp in flight and its own `held` rule, but the stage has one ramp. A member grid event
inside a VCA jump retargets the stage over 64 samples, so the VCA step ends in 64 samples, not in
`J` (and the reverse). Also the driver's arrays are fixed (`MAXIMUM_FADER_CELLS` on the render
stack), and `m` has no compiled bound.
- (a) D1 as filed: per-pair `CellState`, events merged by offset. Small. A VCA step is shortened
  whenever the member's own ride moves. Not correct.
- (b) One composite cell per member channel: one `CellState` over the composed curve
  `vca_compose_db(own(t), S + c_1(t) + … + c_m(t))` (for mute: `own || v_1 || … || v_m`).
  Events at the union of the curves' discontinuities and the shared node-time grid; target, held
  and lagging rules on the composed value. A new K-curve event source in `crates/automation`
  (`events.rs`, render code, not under `crates/engine/src/realtime/`): per pair one discontinuity
  cursor and a cell index (8 bytes) in a boxed slice of the slot, allocated at preparation. With
  `m = 0` the driver keeps today's `CellEvents` path, so no fader-only or mute-only bit moves. The
  route op uses the same source per lane (OR). D8 holds (events at most the sum of the curves').
  About 8 h more than (a).
- (c) Split: this slice renders VCA `mute` (D4, the reach rule, D5's mute bullet, D6 for row 2);
  VCA `offset_db` (D2, D3, D5's offset bullet) moves to a phase-2 successor with (b).
- Recommendation: (b). Fall back to (c) if the offset tranche fails its second fix round.

**R19-V2 (ruling needed): the reach-pair memory.** The host cap is 16,384 (strip, VCA) pairs
(`MAXIMUM_VCA_REACH_PAIRS`), per lane and per row. Under (b) that is at most about 512 KiB of pair
state per plan; under (a) about 3 MiB. Options: (i) charge it as bank payload under the existing
caps (no new cap); (ii) add a host cap on automated pairs. Recommendation: (i) with (b).

**Gates (fixes; both hosts and both bank widths).**
- Gates 1-3: both hosts in one binary, `hosts/host-web/tests/control_parity.rs`, at quanta 128,
  100 and 37. The same scripts in `crates/host-core/tests/stored_vca_automation.rs` (new) at
  `Backend::Simd8` (`cfg(target_feature = "avx2")`) and `Backend::Simd4`, split and fused, live and
  between render calls, and on the scalar oracle; bits equal. Gate 1 adds two VCAs on one member
  (one automated, one static) and a member with its own ride. Gate 3 adds a member with no own
  mute entry (its send becomes seven-word) and a nested VCA.
- Gate 4 reads: a hand fader move on a member with no own automation gives `live`, one offsets
  write carrying `own_db` and no `FaderDb`. Add: the VCA row lists the offset as its value and is
  marked `automated`; a replace that changes only an automated VCA lane's static offset is
  `rebuild`. In `crates/host-core/tests/live_delta.rs` (run with
  `--features host-core/test-support,host-core/control-provider`) and both hosts.
- Gate 5: `crates/control-plane/tests/vca_automation_realtime.rs` gains the curve, a hand VCA move,
  a hand member move and a VCA mute, each block `allocations == 0 && frees == 0`.
- Gate 7 (new, Reposition): a VCA ride table edit while a member's hand move (and a VCA hand move)
  ramps, heard 1, 12 and 26 blocks: with #1711, every block after the swap equals the
  uninterrupted engine and the watermark reports `EXACT`; without #1711, the flag. Both hosts,
  both widths.
- Gate 8 (new, carry): a VCA mute table edit with a hand VCA mute in the same transaction lands the
  hand mute over its own ramp; added, removed and changed VCA entries each rebuild and carry the
  member stage; the swap block allocates nothing. Bank widths (the oracle restarts at a swap,
  #1651 T4 departure).
- Gate 6 command: `audit capi` `pcm_digest` stays `bffc21e857b5e134`. Add `cargo test --locked -p
  automation --all-features`; `cargo build --locked --release -p realtime-policy &&
  ./target/release/realtime_policy` (the script is deleted).

**Authorized paths after the check (this list replaces the filed one).** (R) marks render code.
None is under `crates/engine/src/realtime/`, so no root notice is needed.
- (R) `crates/automation/src/{events.rs,lib.rs}`, `crates/automation/tests/events.rs` (only under
  R19-V1 (b); a path extension root approves with the ruling)
- (R) `crates/builtins-compiler/src/lib.rs` (VCA cells, event lanes, slot, offsets cell shape,
  driver, drain, adopt, export to host-core)
- (R) `crates/graph/src/runtime.rs` (`FollowCursors`, `follow_walk`, the follow route's carry
  only), `crates/graph/src/lib.rs` (`GraphFollowRouteSource`'s cells, docs)
- `crates/controls/src/{automation.rs,vca.rs,effective.rs,registry.rs,view.rs,owner_edits.rs,mask.rs,lib.rs}`,
  `crates/controls/tests/{automation.rs,mask_and_diff.rs}`
- `crates/host-core/src/prepare.rs` (event-lane predicates, follow attach, level records, strip
  and route joins), `crates/host-core/src/live_delta.rs` (docs only)
- `crates/host-core/tests/{vca.rs,live_delta.rs,stored_vca_automation.rs,route_mute.rs}`
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `crates/control-plane/tests/vca_automation_realtime.rs`
- `crates/control-plane/src/control.rs` (only if #1711 does not land: `automates`)
- `docs/SESSION_SCHEMA_V1.md` (`:339-340` the offset rows; the VCA rows render; D2's order),
  `docs/REALTIME_MEMORY.md` (the pair term)
- No SDK regeneration: no row class or table changes.

**Edit order with the batch.** #1711 first (driver adoption, `events.rs`, `control.rs`), then #1654
(`following_route_lanes`, the route op, `takes_offset` and its test), then this slice.
Program cell order: fader cells, mute cells, #1654's route cells, then the VCA cells (VCA ID,
row, lane), so each earlier session builds its program cell for cell. If #1654 generalizes
`follow_walk` to a table of cursors, this slice adds the VCA mute cursors to that table and
writes no second walk.

## Rulings (root, batch 19, 2026-10-17)

These override the batch 19 spec-check amendment where they differ.

- **R19-2 = (b).** One composed `CellState` per output (a member fader channel, a member mute channel)
  over all its curves: the merged walk of `crates/automation/src/compose.rs` (K1, built in batch 19a),
  sum for offsets, OR for mutes. The route op adds the VCA mute curves as more cursors in #1654's route
  table, ORed into the follow term; no second walk. Fallback (c) if U3 fails its second fix round: VCA
  mute ships, VCA offset automation goes to phase 2; root is told.
- **R19-4.** Events of several curves at one offset take the longest jump among them. Gate it.
- **R19-6.** Reach-pair memory is charged as bank payload under the existing caps; no new host cap. A
  gate at the cap: one pair over is refused with a typed diagnostic, not a panic.
- **Two PRs.** This issue ships alone in batch 19b, on top of 19a (#1711, K1, #1654). Edit order: every
  shared file after 19a.

## Decision record: V0 (batch 19)

`crates/controls` only. No render, host-core, graph or builtins-compiler code; no row class or
table change, so no SDK regeneration.

- **Rows (D6).** `automation_row_renders` admits VCA row 1 (`vca_offset_row`) and row 2
  (`vca_mute_row`). `fader_row` is unchanged, so a VCA entry reaches no fader-only reader. New
  accessors: `vca_offset_automation`, `vca_mute_automation` (per lane, first entry in table order;
  the strip readers now share one `owner_row_automation` walk).
- **Offset (D5).** `takes_offset` = `fader_row || route_gain_row || vca_offset_row`. The VCA row is
  per lane, so `offset_faults` judges each driven lane against the row's own domain
  (`[-144, 24]` dB, the fader domain); a lane the entry does not drive holds `+0.0`; the mute row
  takes no offset.
- **Owner rule (R17-1).** `registry::write` on an automated VCA `offset_db` lane writes that lane's
  entry offset (`vca::write_offset`) and the static offset only on lanes no entry drives;
  `vca::resolve` and the list read the offset as the row's value; both VCA rows list
  `automated = true` on a driven lane (the `mute` row keeps its static hand mute as its value).
  `OwnerEdits::set` on VCA row 1 clones the driving entries; `ModelView::vca_automation` reads the
  edited entry. A member's own rows are not marked by a VCA ride.
- **Mask.** `mask_fader_offsets` copies VCA-offset entry offsets (through `takes_offset`);
  `mask_values` keeps `into`'s static offset on each automated VCA lane (fallback = structure),
  and copies the rest of the VCA list (other lanes, mutes, members) as before.
- **Event lanes.** `fader_event_lanes(model, strip, reach)` and `mute_event_lanes(model, strip,
  reach)`: own entry on the row, or a reaching VCA with an entry on the matching VCA row and lane.
  `reach` is the caller's VCA reach of the strip (`vca_reach` / `vca_reach_of`), so preparation and
  the classifier, which already hold it, build no second table. The own-entry readers
  (`fader_automation`, `model_fader_offsets`, `mask_fader_offsets`, the listing flags) stay
  own-entry only.
- **Reach rule.** `rendered_automation_changed` also returns `true` when, with equal rendered
  entries, the strip IDs reached by some VCA that a rendered entry drives differ (inverse of
  `SessionModel::vca_reach`, which reads a model in any strip order). No table is built when no
  rendered entry is on a VCA row.
- **`S` (D2).** `EffectiveInputs` gains `vca_offset(vca)` (the driving entries' hand offsets per
  lane; `ModelInputs` precomputes it, `LocalInputs` reads the view). `effective_strip` sums: own
  hand offset, then static offsets of reaching VCAs whose lane is not automated, then hand offsets
  of those whose lane is, each in ascending VCA ID. With no VCA offset entry the second pass does
  not run and `S` is today's sum. `fader_db` stays the static composition (a lane an automated VCA
  reaches is a fader event lane, which takes no `FaderDb` record). Departure: none from the text;
  `EffectiveInputs` is a public trait with no implementor outside the crate.
- **Not in V0** (render tranche): `continued_fader_lanes` / `continued_mute_lanes` are not widened
  to the reaching VCA tables (D6 carry).

Gates (`crates/controls/tests/automation.rs`): `vca_rows_render_and_take_a_per_lane_offset`,
`a_hand_move_on_an_automated_vca_offset_writes_its_offset` (whole and in-place paths, list flags,
`S` on both effective paths), `the_mask_copies_a_vca_offset_and_keeps_its_fallback`,
`event_lanes_take_the_own_entry_and_every_reaching_vca_ride` (lanes, rows, nesting, own-entry
readers), `a_reach_change_of_an_automated_vca_rebuilds` (members added or removed, nesting, a
non-automated VCA, a hand offset, the mute row), `the_offset_sum_puts_the_static_vca_offsets_first`
(values `1, 2^-60, -1` that tell the two orders apart; no VCA offset automation and a VCA mute
entry alone keep every bit of `S` and `fader_db`).

Commands: `cargo test --locked -p controls` (all green), `cargo clippy --locked -p controls
--all-targets --all-features -- -D warnings`, `cargo check --locked -p controls --all-targets`,
`cargo test --locked -p host-core --features test-support` (467 passed),
`--features test-support,control-provider` (587 passed), `cargo test --locked -p control-plane
--features test-support` (28 passed). No red: no host-core or control-plane test edits a VCA
automation entry or an automated VCA's reach today.

For V1: a VCA automation edit and an automated VCA's reach change are now step 3 `Automation`
(rebuild) in the classifier; a session whose only rendered entries are VCA entries now enters
`prepare_fader_automation` (it builds no VCA cell yet; tests green); a hand move on an automated
VCA offset diffs as a VCA row 1 change whose value is the offset and moves `S` of each reached
lane, which host-core lowers only on lanes that have an offsets cell until V1 gives every event
lane one.

## Decision record: V2 (batch 19)

The fader stage (`crates/builtins-compiler/src/lib.rs`), one additive rule in K1
(`crates/automation/src/compose.rs`), the carry predicates (`crates/controls/src/automation.rs`).
No edit under `crates/engine/src/realtime/`; no row class or table change (no SDK regeneration).

- **Program (D1).** The VCA cells follow every route cell, in ascending VCA ID, then row
  (`offset_db` with `FADER_CELL_RULE`, `mute` with `MUTE_CELL_RULE`), then lane; one cell per
  automated VCA lane, shared by its members (refusal code `host.automation.vca`). The reach is read
  only when an entry drives a VCA row, so a session without VCA automation builds its program cell
  for cell. Export for V3/V4: `PreparedBuiltinsSession::vca_cells(vca_id) -> Option<(Arc<Program>,
  VcaCells)>` (`[row][lane]`). `strip_mute_cells` stays own-entry cells (a lane only a VCA mute
  reaches reads `None` there).
- **Event lanes (D2, D4, R19-2 = (b)).** Preparation reads `controls::fader_event_lanes` and
  `mute_event_lanes`. A lane no automated VCA curve reaches keeps `FaderCurve::Cell` /
  `MuteCurve::Cell`: the same `CellEvents` path and bits as before. A reached lane is
  `FaderCurve::Composed` (one `automation::Composed<f32>` over its own cell, when automated, then
  the VCA cells in ascending VCA ID) or `MuteCurve::Composed` (`Composed<bool>`, gate cursors, value
  `own || v_1 || ... || v_m`; the stage mute is that `|| terms`). The composed value is the fader dB
  `vca_compose_db(own, S + c_1 + ... + c_m)` (`composed_db`: `f64` fold, `S` first, inside the
  render entry's `CanonicalFpEnv`; the prepared value inside preparation's), `own` the own curve or
  `own_db`; the stage gain is `checked_fader_gain` of it.
- **One driver (R17-4).** `FaderEventDriver` (bank, fused pair fallback, scalar oracle) holds a
  `ChannelEvents::{Cell, Composed}` per channel: each piece begins the walk (`Composed::start`), the
  cut at `E` is a one-sample `Ramp` landing on every cursor (longest jump, R19-4), and the held-ramp
  obligation, the R17-10 release, the T3b re-hold release, the held-stage offsets jump and the free
  offsets jump act on every cursor (`FaderComposite::each_state`). `settle_adoption` is generic over
  `AdoptedCurve` (`CellAdoption`, `ComposedAdoption`): one adoption rule (F0 ramp target, the
  deferred jump at the ramp's first free sample, grid hold, clears) for both shapes.
  `block_jumps` resolves `LiveRamps::for_row(VcaOffset)` and `for_row(VcaMute)` (slot field
  `vca_jumps`); a VCA cursor takes its row's length, the own cursor the fader or mute length.
- **D3.** `OFFSETS_CELL_WORDS = 4`. `FaderOffsetsRecord` gains `own_db: Option<f32>` (word 3, a
  quiet NaN for `None`); a composed lane whose own fader is not automated takes `Some(own_db)` and
  jumps at the block entry only when the gain bits change; `None` keeps it.
- **Carry (D6).** `continued_fader_lanes` / `continued_mute_lanes` now continue a lane only when its
  own table (absent in both, or the same) and each reaching automated VCA table (same VCAs, ascending
  ID, same segments) continue; with no VCA row automated the rule is the old one. `FaderCurve::carry`
  / `MuteCurve::carry`: same shape copies the cell state, or the composed output's state and target
  and, with one layout, the cursors; a changed shape takes the cell's ramp or starts fresh (always a
  reposition). A composed reposition places every cursor afresh (no target, hears its curve at the
  block start), so a deferred jump or a landing reaches every curve. `own_db` is carried like `S`
  (`adopted_own_db`, the `Compare` backstop).
- **Memory (R19-6).** Each composed channel charges its boxed curve (`fader_composite_bytes` 128,
  `mute_composite_bytes` 112) and its cursor table (`cursor_bytes` 104 per pair) as bank payload and
  in the scalar owner row (`FaderAutomationPlan::composed_bytes`), under the existing caps; the
  offsets-writer charge widens to fader event lanes. The cap gate is V5's.

**Departures.**
1. K1 additive rule `Composed::with_held_grid` (opt-in; the route keeps R19-3). Without it a grid
   event of one curve retargets the composed lane inside another curve's jump (the R19-V1 (a)
   defect: the step ends at the next grid point, not at `J`). With it each retarget reports its ramp
   to every value cursor whose own ramp ends earlier, and the cell's own grid rule holds or restarts
   it. Also `clear_target`, `output_state`, `set_output` (carry). Test
   `compose::tests::a_step_keeps_its_jump_length_while_another_curve_rides_on_a_held_grid_output`.
2. An offsets record on a composed lane takes the single cell's rule (seed, clear the deferral,
   every cursor holds to the jump's completion), not `Composed::retarget` (deferral then each
   cursor's own `J`), so a composed lane moves as one cell does in holds and deferrals.
3. `own_db` is `Option<f32>`: today's host-core record sites write `None` (mechanical, correct for
   every lane that has a cell today); V3 writes `Some` on static-own event lanes.
4. `LaneFaderAutomation::new` returns `Result` (the cursor table allocates); bind
   (`bound_lane_automation`) panics only on an allocation failure, as a `Box::new` there does.

**Gates** (`cargo test --locked -p builtins-compiler --all-features`, Simd8 under AVX2 and Simd4):
`a_flat_vca_ride_renders_the_static_vca_offset_bits` (flat ride `-6` dB and flat mute `0` equal the
static VCA bits over 40 blocks, a member with and without its own ride, fallback `+3` unused),
`a_vca_ride_composes_with_each_members_own_value_at_every_grid_event`,
`a_step_inside_another_curves_ride_keeps_its_jump_length` (both ways; red without `with_held_grid`:
`(1024, Grid)` after the step), `coincident_member_and_vca_events_take_the_longest_jump` (fader and
mute, 480/960 both ways), `a_session_without_vca_offset_automation_keeps_its_lone_fader_cells`,
`a_carried_composed_lane_renders_the_uninterrupted_bits` (continued and repositioned, swap inside a
grid ramp and inside a VCA step's jump; red without the cursor carry). `controls`:
`a_member_lane_continues_only_while_its_reaching_vca_tables_do`.

Commands: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` and no-feature
`cargo check --locked --all-targets` on automation, builtins, builtins-compiler, host-core, controls;
tests automation (all features), builtins (all features), builtins-compiler (all features, 76 lib),
controls, host-core `--features test-support` `stored_fader_automation` (36), `stored_mute_automation`
(13), `successor_swap` (140): all green. Realtime policy: 172 regions in 35 files (171 before: the
composed-curve impls). Sizes (x86-64, measured): `Option<LaneFaderAutomation>` 392 -> 416 B
(`vca_jumps`), `offsets_cell_bytes` 120 (unchanged), `ComposeCursor` 104.

For V3: write `own_db = Some(static fader dB)` on fader event lanes with no own automation; take the
fader event lanes as `automated` in `level_records_between` / `fader_records` (no `FaderDb` must reach
a composed lane: the drain still applies a `FaderDb` to the stage); `has_offsets` is true on every
fader event lane. For V4: a source lane's VCA mute cells are its reach (ascending VCA ID) mapped
through `vca_cells(..)[1][lane]`, jump row `VcaMute`.

## Decision record: V3 (batch 19)

host-core preparation and the classifier (`crates/host-core/src/{prepare.rs,live_delta.rs}`). No
render code, no edit under `crates/engine/src/realtime/`, no `control.rs` edit (#1711 landed in
19a, so `automates` stays), no row class or table change (no SDK regeneration).

- **Records (D3).** `level_records_between` takes the strip's own faders before and after and a
  `FaderEventLanes` (`controls::fader_event_lanes` as `events`, `automated_fader_lanes` as `own`).
  `fader_records` reads `events` as its automated lanes, so no `FaderDb` reaches an event lane. An
  event lane gets one offsets write when its `S` moves or, on a lane whose own fader is not
  automated, when its static own dB moves; that write carries `own_db = Some(own after)` (else
  `None`) and the lane's fader length (`LevelRules::lengths`: `resolve(Fader, edit)` for a hand
  move). Dense diff, local (fast-path) diff and the strip join all call it with the reach they hold.
- **Clamp.** `level_differs` also compares the own static fader dB: at the effective clamp a hand
  move leaves `fader_db` and `S` as they were but a composed lane hears it. On any other lane the
  extra "moved" yields no record, so no non-VCA-automation answer changes.
- **VCA hand offset (D5).** `LevelRules::vca_changed` (offset) also compares the driving entry's
  hand offset (`vca_offset_moved`: `vca_offset_automation(current)` against
  `ModelView::vca_automation(next)`), so members' writes take the VCA edit's length, not the
  session's. `S` itself is V0's (`effective_strip`).
- **VCA mute (D4, D5).** No new record: the effective mute (`terms`) change is the `Mute` record,
  which V2's composed mute lane takes as `terms`.
- **Reach rule.** No classifier change: V0's `rendered_automation_changed` gives step 3 `Automation`;
  step 3's doc now names the VCA rows and the reach rule.
- **Join (D6).** `join_level_records` releases lanes that stop being fader / mute event lanes
  (`fader_event_lanes` / `mute_event_lanes` over `reach_before` and `reach`). The carry byte is V2's
  widened `continued_*_lanes`, unchanged here.
- **Slot.** V2 already gives every fader event lane its `PreparedFaderCell` and offsets cell
  (`attach_offsets` over `cells.fader`); V3 only gates it.
- **Route (D4).** `following_route_lanes` widens `lanes` to `mute_event_lanes` (reach read through
  `vca_reach_of` only when an entry drives a VCA `mute` row and an own lane is not automated), so the
  attach, #1653's records, the local route scan and `join_routes` read one predicate. The attach
  checks `own cell || VCA cells` per lane against it, with the new
  `follow_vca_mute_cells(vca_cells, model, source, program) -> Option<[Vec<u32>; 2]>`: per source
  lane, `PreparedBuiltinsSession::vca_cells(vca)[1][lane]` of each reaching automated VCA mute, in
  ascending VCA ID, `None` on a plan/model mismatch.

**Departure / interim.** Until V4, a following send whose source lane only a VCA mute reaches has
`automated[lane] = true` and no own cell, so `GraphFollowRouteSource::new` refuses it and preparation
fails with `host.route_control.attach` (typed, no panic). V4 passes `follow_vca_mute_cells`' lists
to the source (graph) and relaxes `new`'s `flags.automated == cells` check to `own || VCA`. A lane
with its own mute cell and a VCA mute prepares, and until V4 its send follows only the own curve.

**Gates** (`crates/host-core/tests/stored_vca_automation.rs`, renders on Simd8 under AVX2, Simd4 and
the scalar oracle, live and between render calls, tail bits equal to a fresh plan, the edit heard,
render allocations and frees 0): `every_fader_event_lane_has_an_offsets_cell` (test-support);
under `control-provider`, full and fast path equal:
`a_hand_move_on_a_member_only_a_vca_ride_reaches_writes_its_own_db`,
`a_hand_move_on_an_automated_vca_offset_rewrites_every_reached_lanes_s`,
`a_hand_mute_of_an_automated_vca_writes_each_reached_lanes_terms`,
`a_reach_change_of_an_automated_vca_rebuilds_and_of_a_plain_vca_is_live`,
`a_hand_move_hidden_by_the_static_clamp_still_writes_own_db`.

Commands: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` and no-feature
`cargo check --locked --all-targets` on host-core, control-plane, builtins-compiler; tests host-core
`--features test-support` (477 passed) and `test-support,control-provider` (605), control-plane
`--features test-support`, builtins-compiler, capi lib (218): all green. Realtime policy: 172
regions in 35 files (unchanged).

For V4: read `follow_vca_mute_cells` in the attach loop (`prepare.rs`, the `follow_cells` push) and
pass its lists into `GraphFollowRouteSource`; jump row `VcaMute`. For V5: host parity, the cap
gate (R19-6), gates 1-3 and 7-8 on both hosts.

## Decision record: V4 (batch 19)

The route op's VCA mute follow term (`crates/graph/src/{lib.rs,runtime.rs}`) and the attach in
`crates/host-core/src/prepare.rs` (the `follow_cells` loop and `follow_route_jumps`). No edit under
`crates/engine/src/realtime/`, in `crates/automation` or in any row class or table (no SDK
regeneration).

- **Source (D4).** `GraphFollowRouteSource::new` takes `vca_mutes: [Vec<u32>; 2]` (per source lane,
  V3's `follow_vca_mute_cells`, ascending VCA ID; stored boxed) and checks `flags.automated[lane] ==
  own cell || VCA cells`, every cell in the program. This closes V3's interim refusal: a send whose
  lane only a VCA mute reaches now prepares. `with_continued_vca_mutes([bool; 2])` (host-core:
  `controls::continued_mute_lanes` of the successor) marks lanes whose VCA mute tables continue.
  When the source strip has no own mute cell and the route no row cell, the attach takes the VCA
  cells' program (`follow_vca_mute_cells` now takes `&mut Option<Arc<P>>`).
- **Jump row (data).** `GraphRouteCursorRow::VcaMute` and `GraphRouteJumpLengths::vca_mute`;
  host-core's rule reads `LiveRamps::for_row(VcaMute)`.
- **Walk.** The VCA mute cursors are `Gate` cursors appended after the five role cursors in each
  column's one K1 table (`ColumnRoles { fixed, vca_start, vca_count }`); no second walk. The follow
  term is `follows_mute && (own curve || v_1 || ... || v_m || terms)` in `column_target`, in the
  tap's node time. Because they are gates, a VCA mute fade holds the column's value events and
  lands them at its end (R19-3, K1's deferral). The table (and its probe twin) is boxed, allocated
  at attach, charged by `Composed::bytes(K)`; the source's lists are added to the producer's
  `cursor_bytes`.
- **Carry (D6, U2 folds).** A lane's VCA mute cursors carry verbatim by position (`Continued`) when
  the lane continues and `a(tap)` did not move; else they start fresh and are placed at the first
  block like the source mute. At the first block the follow curves are one gate: its OR is
  compared with the OR the predecessor heard (with the own mute alone this is the old per-cursor
  comparison, so no bit moves). A gate move takes the longest own length among the curves that
  close the column (for an open, among all follow curves; which were closed is not kept across a
  carry), R19-4. A follow curve is "dropped" (#1654 U2 fold 1 MAJOR-3) only when no own or VCA mute
  curve is left; a dropped VCA mute lands at once over `VcaMute` at the carry (B19-R7). A seven -> six carry of
  a predecessor that heard VCA curves reopens over `max(mute, VcaMute)`.

**Departure.** In `place_carried` a moved gate now retargets over its own row's length (was the
mute length for every gate; equal for the source and route mutes, so no existing bit moves).

Gates (`cargo test --locked -p graph --features test-support`, 149 lib tests green), module
`runtime::send_automation_tests`, `VcaMute` = 320 against the mute rule's 200:
`a_vca_mute_step_closes_a_following_send_at_its_tap_time` (a lane only a VCA mute reaches, `a(tap)`
0 and 100: the close lands at render sample `1,000 + a(tap)` over 320),
`a_vca_mute_ors_with_the_source_lanes_own_mute`, `a_vca_unmute_fade_holds_the_columns_value_events_to_its_end`
(R19-3), `a_carried_vca_mute_renders_the_twins_bits_continued_or_repositioned` (red with the
continued cursor's state dropped: block 21), `a_moved_vca_mute_table_lands_over_its_own_length_at_the_first_sample`
(both ways, the OR with a closed own mute, a removed VCA mute); `follow_route_tests`: the source
accepts VCA-only lanes and refuses a VCA cell outside the program and disagreeing flags.

Commands: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` on graph, host-core,
automation; `cargo check --locked --all-targets` (workspace, no features); tests automation, graph
`--features test-support`, host-core `--features test-support` (all, incl. `route_mute` 26,
`stored_vca_automation`; with `control-provider` 6), capi lib (220), host-web `--features
test-support --test control_parity` (44): all green. Realtime policy: 172 regions in 35 files
(unchanged). `scripts/check-graph-policy.sh`: PASS. Sizes (x86-64, measured):
`LIVE_FOLLOW_ROUTE_OWNER_BYTES` 1,072 -> 1,120, `GraphFollowRouteSource` 120 -> 152,
`GraphFollowRouteControlBinding` 808 -> 872, `GraphFollowRouteControlProducer` 104; charges read
`size_of`.

For V5: host gates for gate 3 (a following send of a member closes at its tap under a VCA mute,
both hosts, latent tap), a member with no own mute entry, a nested VCA, the carry (continued and
moved VCA mute tables) and the reach-pair cap gate (R19-6).

## Decision record: V5 (batch 19)

The objective gates and the doc. Tests and `docs/SESSION_SCHEMA_V1.md` only: no product code, no
edit under `crates/engine/src/realtime/`, no row class or table change (no SDK regeneration). The
gates found no defect.

- **Hosts.** Every both-host gate is in `hosts/host-web/tests/control_parity.rs`, which drives the
  C ABI's exported `miso_engine_v1_*` entries and the browser host in one binary (the amendment's
  "both hosts in one binary"); `crates/capi/src/runtime/live_tests.rs` gains nothing (departure
  from the brief's file list, not from the gates). Parity runs at quantum 128 and the native width;
  widths and quanta are host-core's.
- **Widths.** `crates/host-core/tests/stored_vca_automation.rs` runs Simd8 (when `avx2`), Simd4 and
  the scalar oracle (`backends()`), live and between render calls; the carry gates (8) run the bank
  widths only (the oracle restarts at a swap, #1651 T4).

Gates (test names; all green):
1. Flat equals static: host-core `a_flat_vca_ride_and_mute_render_the_static_vca_bits` (quanta 128,
   100, 37; a member with its own ride, one with none and in a second static VCA; fallback +3
   unused); both hosts `a_flat_vca_ride_and_mute_render_the_static_vca_bits_on_both_hosts`.
2. Member node time: host-core `a_vca_ride_reaches_each_member_in_its_own_node_time` (each grid
   event's gain equals `checked_fader_gain(vca_compose_db(own, S + c(τ)))` computed on the test
   thread; the latent member's events `L` later); both hosts
   `a_linear_vca_ride_renders_alike_on_both_hosts` (alike and heard).
3. Mute: host-core `a_vca_mute_step_mutes_each_member_at_its_node_time` (off-grid step at 1,000;
   `SetMute` at `1,000` and `1,000 + L` over `VcaMute`; a member with no own mute entry, one with,
   a nested VCA; static-mute and open plans after the ramps); both hosts
   `a_vca_mute_curve_closes_a_following_send_at_its_latent_tap_on_both_hosts` (also the V4 note: the
   send follows a lane only a VCA mute reaches, through a nested VCA, behind a latent tap).
4. Hand edits: both hosts `hand_edits_of_an_automated_vca_commit_as_d5_says_on_both_hosts` (offset
   +3 live and heard; hand mute over an open curve heard, over a closed one no bit moves; a hand
   move of a member only the ride reaches live and heard; membership edit of the automated VCA
   `rebuild` and `EXACT`, no fallback; of a plain VCA `live`) and
   `a_hand_move_on_an_automated_vca_offset_is_in_place_as_the_full_path_on_both_hosts` (R17-1; the
   document stores the offset). The `own_db` record and "no `FaderDb`" stay V3's classifier gates.
5. Realtime: `crates/control-plane/tests/vca_automation_realtime.rs`
   `a_vca_curve_and_hand_edits_over_it_render_allocation_free` (curve, hand VCA move, hand member
   move, hand VCA mute and unmute; every block `allocations == 0 && frees == 0`).
6. No bit moves: `./target/release/audit capi` `pcm_digest` = `bffc21e857b5e134`.
7. Reposition: both hosts
   `a_vca_ride_table_edit_while_a_level_move_ramps_completes_exactly_on_both_hosts` (a member's
   `own_db` move and a VCA hand move of 4,000 samples, heard 1, 12, 26 blocks; a table change after
   1,000,000 repositions every member; each host equals its own twin without the rebuild; `EXACT`).
8. Carry: host-core `a_hand_vca_mute_in_a_vca_mute_curve_rebuild_lands_over_its_own_ramp` (ramp
   `M`, 0, 2,000) and `added_removed_and_changed_vca_mute_entries_carry_the_member_stage` (swap
   block allocation-free, strip carried); both hosts
   `a_hand_vca_mute_in_a_vca_mute_curve_rebuild_lands_over_its_own_ramp_on_both_hosts`.
- R19-4: host-core `coincident_member_and_vca_events_take_one_longest_jump` (one fader jump over
  `J`, one `SetMute` over `M`; bits of one curve making the sum). At host level the `VcaOffset` and
  `VcaMute` lengths equal the fader and mute lengths, so "longest" is V2's
  `coincident_member_and_vca_events_take_the_longest_jump` (480/960) at the stage.
- R19-6: both hosts `the_reach_pair_cap_bounds_vca_automation_alike_on_both_hosts` (256 nested VCAs,
  the outermost automated: 64 tracks, 16,384 pairs, boot; 65 tracks refused
  `host.resource.vca_reach_pairs` alike, no panic).

Red without the feature (once each, reverted): gates 1-3 with the VCA reach dropped from
`controls::event_lanes` (VCA curves reach no member): host-core gates 1, 2, 3 red (and gate 8's
carry case); gate 1 and 3 parity red; gate 4 red with the same knockout (a hand VCA move is not
heard: `automated_vca_alike`'s heard assert); gate 7 red with `settle_adoption`'s stage hold
ignored (the move is cut at the swap: the browser twin comparison). Knocking out
`automation_row_renders` for VCA rows alone leaves gates 1-3 green: render reads the event-lane
predicates, the classifier the rows.

Commands: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` on host-core,
control-plane, host-web; `cargo check --locked --all-targets` (workspace, no features); tests
automation (44), builtins-compiler `--features test-support` (94), graph `--features test-support`
(159), host-core `--features test-support` (485) and `test-support,control-provider` (613),
control-plane `--features test-support` (30), capi lib (220), host-web `--features test-support
--test control_parity` (52): all green. Realtime policy: 172 regions in 35 files (unchanged).

## Decision record: U3 fold 1 (batch 19)

U3 MAJOR-1 and NIT-1. No edit under `crates/engine/src/realtime/`, no row class or table change (no
SDK regeneration).

- **Fix (K1, opt-in).** `automation::Composed::with_jump_release` (new flag, copied by `copy_from`):
  every jump the output emits (not a seek landing, not a held event) shortens each restartable value
  cursor's hold to the jump's completion (`CellState::release_hold_by`) and drops its deferral
  (`clear_deferred_jump`), before the `with_held_grid` hold and the column-jump deferral; a cursor
  that pulled ahead is replayed first, as for a hold. A jump that emits nothing changes nothing
  (#1711 U1 fold 2). It reaches every hold and deferral the lane put on every cursor: the record
  hold in `offsets_jump`, the adoption hold and deferral of `settle_adoption`, and the holds
  `with_held_grid` propagates, so a composed lane moves as one cell does in holds and deferrals
  (V2 departure 2).
- **Who uses it.** Only the fader composed lanes (`FaderCurve::prepared`: `.with_held_grid()
  .with_jump_release()`). The composed mute lanes have gate cursors only. The route column
  (`crates/graph`, #1654) does not call it and keeps its T3 rule: a held value follows the column's
  current free time and never moves earlier. No graph file changed; graph tests and #1654 gates
  unchanged.
- **NIT-1.** `follow_route_flags`' doc lines moved back above it; `follow_vca_mute_cells` keeps its
  own doc.

**Departures.**
1. The host-core live gate compares from the step's jump's end (3,240), not from 0. Before the step
   the composed lane and one cell over the sum differ: a record's ramp target composes each curve
   toward the ramp's completion clipped at that curve's own next discontinuity, where one cell clips
   the sum at the lane's next discontinuity (`FaderComposite::toward` against `curve_toward`). The
   jump's end equalizes the gains, and the defect window (3,328 to 9,024) lies after it.
   Defined behaviour: on a composed fader lane a record's or an adoption's ramp target composes each
   curve toward the ramp's completion, each clipped at its own next discontinuity (K1
   `compose_toward`, #1649 D3), so between the record and another curve's next discontinuity the
   lane differs from one cell over the summed curve, and that curve's jump makes them equal; a
   successor plan uses the same rule.
2. The both-host gate runs at the native width only (control_parity's rule, V5); the two bank widths
   are the host-core twin's.

**Gates** (red before: the gate with `.with_jump_release()` removed from `FaderCurve::prepared`, or
from the K1 probe; green after):
- K1: `compose::tests::a_step_replaces_a_hold_the_lane_put_on_every_cursor_as_one_cell_does` (every
  cursor held to 2,999, then also deferred to 3,000; the composed retargets equal one cell over the
  summed curve with the same hold and deferral). Red: `(2944, Grid)` after the step instead of
  `(1472, Grid)`.
- host-core live: `stored_vca_automation::classifier::
  a_vca_step_inside_a_hand_moves_ramp_lets_the_own_ride_move_as_one_cell` (own ride on a 2^-12 dB
  lattice, VCA step at 3,000, hand VCA move to +3 over 8,000 at 1,024; Simd8, Simd4, scalar, live and
  between render calls; same classifier records as the summed single-cell session). Red: first
  differing sample 3,328 (Simd8).
- host-core rebuild twin: `stored_fader_automation::
  a_table_step_inside_a_deferred_moves_ramp_releases_every_cursor_of_a_composed_lane` (Simd8 and
  Simd4; track 0 also in an automated VCA riding 0 to -6 dB). Red: first differing sample 4,544.
- both hosts: `control_parity::
  a_table_step_inside_a_deferred_moves_ramp_releases_a_composed_lane_alike_on_both_hosts` (the
  #1711 body over a session with the VCA ride; four rates). Red: block 39 against the reference at
  44.1 kHz. The single-cell test now calls the shared body `table_step_inside_a_deferred_move`.

Commands: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` and no-feature
`cargo check --locked --all-targets` on automation, builtins-compiler, host-core, graph, host-web;
tests automation (all features), builtins-compiler `--features test-support` (76 lib), graph
`--features test-support` (149 lib, unchanged), host-core `--features test-support`
`stored_vca_automation` (7; 13 with `control-provider`), `stored_fader_automation` (37),
`stored_mute_automation` (13), `route_mute` (26), `successor_swap` (140), control-plane
`--features test-support` (30), capi lib (220), host-web `--features test-support --test
control_parity` (54): all green. Realtime policy: 172 regions in 35 files (unchanged).
