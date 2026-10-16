# Classify fader automation edits as carried rebuilds

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A2, A10 and A1.8, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

On both hosts, a transaction that adds, changes or removes stored fader automation on a playing
engine takes the rebuild path. Both hosts run the shared commit of `crates/control-plane`: the C
ABI directly, the browser in its Worker (#1382, which lands before #1646). The successor plan holds the new automation program, every owner
carries, the fader lane continues from the value it has, and the revision completes `exact` at
adoption. The response reports `rebuild`. A transaction that changes only the static fader value
of a lane the session automates commits as `model_only`: that value is the lane's fallback, and no
plan renders it while the automation exists. A browser live fader edit on such a lane reaches
the same shared commit through the Worker's apply (#1382) and meets the same rule: the reply reports
`model_only`, and the curve keeps playing. This is one permanent rule in the shared commit, for
both hosts (README A2). Every other automation row stays model-only and inert
until its own rendering slice.

Today every automation edit is "live with no records": it commits with no plan change, so after
#1646 and #1647 render fader automation, an edit would be acked and never heard.

## Context

- **The mask.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:211-215`) copies
  `current.automation` into the masked model (`:234-236`). Its documentation says the first issue
  that renders stored automation must drop that line (`:181-185`). The numbered steps are
  `:149-179`; `LiveRebuild` is `:117-141`. These anchors are those of `6ee64f484`. On `main`,
  *Refuse automation on effect parameters that are not block-rate* (#1335, commit `0c19119d0`) has
  since added `LiveRebuild::AutomationTarget` and inserted its check as step 4, so the per-track
  step is step 5. This draft's step numbers follow #1335's.
- **Fader records.** The per-track step (step 5; step 4 at `6ee64f484`) emits one `FaderDb` per lane
  whose gain bits change and one `Mute` per lane whose mute changes (`live_delta.rs:269-306`).
- **The edit path.** *Report each transaction's edit path in its response* (#1313) D2 reports
  `rebuild` for `Err(_)` and `model_only` for an empty delta.
- **The rebuild path.** `commit_live` returns `LiveCommit::Rebuild` when the classifier refuses
  (`crates/capi/src/runtime/control.rs:1065-1077`); the successor is prepared with
  `prepare_host_runtime_with_live_lanes_successor` (`crates/capi/src/runtime/compile.rs:593`).
  #1309 moves both into `crates/control-plane`.
- **What carries.** The plan inventory has one row list per carried family
  (`PlanStateInventory`, `crates/host-core/src/prepare.rs:555-630`); a successor joins it against
  `SuccessorBase` (`:641-647`) and installs a carry program into the graph (`:1791-1793`,
  `:1820-1821`). *Carry fader, mute and pan ramps across a plan swap* (#1277) adds the fader and
  matrix stages, with move mode at the swap block (#1277 D8).
- **Tests that pin today's rule.** The "automation upsert" case of `model_only_edits`
  (`crates/host-core/tests/live_delta.rs:530-545`, the ride at `:508-528`, the removal case at
  `:582-588`) and of `model_only_edits` in `crates/capi/src/runtime/live_tests.rs:1419-1470`
  (run by `model_only_edits_commit_without_a_plan_rebuild`, `:1481`) use a fader ride (row 5).
- **Docs that say automation edits are model-only:** `crates/capi/include/miso_engine_v1.h:41-43`,
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`, `docs/C_ABI_V1_QUALIFICATION.md:268-274`.

## Decisions frozen for this slice

- **D1. The mask becomes per target row.**
  - One function in host-core, `automation_row_renders(target: &AutomationTarget) -> bool`, names
    the rows that render. In this slice it is true only for `rack = builtins`, `parameter_id = 5`.
    #1646's preparation compiles cells only for the targets it accepts, so the classifier and
    preparation read one list. Each later rendering slice adds its row; #1667 adds the last.
  - The masked JSON comparison keeps copying `current.automation` (`:234-236`). A new step, after
    step 4, #1335 D4's `AutomationTarget` check (which #1645 D5 extends to the builtin rules),
    and before the per-track step, compares the entries of rendered rows in `current` and `next`,
    in table order, by canonical bytes. Any difference returns a new `LiveRebuild::Automation`.
    The numbered rules and the model-only paragraph (`:181-185`) are rewritten to say so.
- **D2. The rebuild carries every owner** (A1.8; D15-7).
  - **The stage.** This slice extends the carry predicate of #1277 D4 (which lands first): gaining,
    losing or changing stored automation is neither a prepared value nor a change of control kind,
    so the fader stage carries. #1277 itself is not amended.
  - **The cell.** A cell's whole event state (#1637 D2: cursor, current target bits, in-flight
    ramp end, held grid, last seek boundary) carries by its stable address `(strip ID, parameter ID,
    lane)`. #1646 D1 keeps that state in the fader bank stage beside the lane's ramp state, so
    the fader stage's carry program under #1277 moves it, with the ramp state, at the swap block in
    move mode (#1277 D8). The stage's inventory rows gain the cell addresses, and the join pairs
    them by address. No second carry program exists for cell state.
  - **At adoption.** The successor repositions each carried cursor in its own table (#1637 D4).
    If the new curve's value at the adoption block's first sample differs in bits from the carried
    current target, the cell takes a jump there, over the fader jump length. (Amended by U3 fold
    2 and B17-R13: on a free stage still in a ramp, the curve is compared where that ramp ends and
    the landing is over `max(J, remaining)`; a step of the cell's own at the first sample keeps
    the comparison there and the landing over `J`. See "U3 fold 2" and "Split (root ruling
    B17-R13)".)
  - **A cell gained.** A lane that gains automation on a carried stage starts its cell with the
    carried stage target as its current target, then applies the adoption rule above. It is not set
    exactly: the strip is not added (A1.4 "New lanes").
  - **A cell lost.** A lane whose entry the edit removes gets a release cell in the successor: its
    static effective value (the `next` fader composed with the VCA offsets, as preparation bakes
    it), no table. At adoption it takes the carried state; if the carried target differs from the
    static value, it jumps to it over the fader jump length. Then it is inert. No control-plane
    record is written: render alone knows the carried target.
- **D3. A static fader edit on an automated lane gives no record** (A2).
  - One predicate, `automated_cell(model: &SessionModel, address: CellAddress) -> bool`, built on
    the same enumeration #1646's preparation compiles cells from. The later slices reuse it.
  - The per-track step (step 5) emits no `FaderDb` for a lane that `next` automates. After D1,
    `current` automates the same lanes. Since *Deliver value-only VCA edits to the running C ABI
    plan* (#1247) D2 that step diffs the effective faders, so the rule covers a VCA move that
    reaches an automated lane too; #1649 makes that move reach the lane's offsets cell. Mute
    records are unchanged. A transaction that changes only such a value yields an empty delta and
    commits `model_only` (#1313 D2); D2's release cell reads the value when the automation goes.
  - **One rule, both hosts.** The rule lives in the shared classifier, which both hosts' commits
    call (#1309; the Worker's apply, #1382, for the browser). No host refuses such an edit and no host has its own
    admission for automated lanes. README A2 and finding F9 give why `model_only` is not
    "acknowledged with no effect" in #1315's sense: the edit sets the committed document's fallback
    value, the response names the path, and the value is rendered when the automation goes.
- **D4. Docs.** The sentences that say automation edits are model-only, in the text that P1
  (#1627 D5) and *Report each transaction's edit path in its response* (#1313, which rewrites
  the header paragraph) leave in the C ABI header, `CONTROL_PROTOCOL_SEMANTICS.md` and
  `C_ABI_V1_QUALIFICATION.md` (on `6ee64f484`: `miso_engine_v1.h:41-43`,
  `CONTROL_PROTOCOL_SEMANTICS.md:15`, `C_ABI_V1_QUALIFICATION.md:268-274`), say: a fader automation edit rebuilds and carries; a
  static fader edit on an automated lane is model-only; the other rows stay model-only until they
  render.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** A rebuild acks only after
  its successor is prepared, and the carry moves every value. No queue is added.

## Deliverables

1. D1 and D3 in `crates/host-core/src/live_delta.rs`, with the classifier tests.
2. D2: the inventory rows, join, release cells and carry-program move.
3. D4.

## Authorized paths

- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`
- `crates/host-core/src/prepare.rs` (the inventory rows, the join and the release cells only),
  `crates/host-core/tests/successor_swap.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`, `crates/builtins-compiler/src/lib.rs`
  (the fader stage carry's move of cell event state only; stream A's files, sequenced by root)
- `crates/control-plane/src/` (the commit path; #1309 is on `main` through #1646's
  dependency), `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs` (gate 5)
- `crates/capi/include/miso_engine_v1.h` (comment text), `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Rendering fader automation (#1646 and #1647).
  VCA offsets on an automated member (#1649). The jump length cell (#1639). Mute, pan, matrix,
  trim, polarity, filter and effect rows (#1651 to #1667).
- Options B, C and D of OQ1. The browser's Worker commit itself (#1382, which lands before
  #1646).

## Hazards

- **Superseded cases.** The "automation upsert" and removal cases named in Context now rebuild.
  This slice moves them to a parametric EQ band-gain entry, a valid `Block` target whose row stays
  masked until #1667, the last row to leave the mask. So no slice between #1648 and #1667 touches
  them, and #1667, after which every automation edit is a rebuild, deletes them.
- **Order of the two automation checks.** #1645 D5 routes a refusable builtin entry to
  `LiveRebuild::AutomationTarget`. D1's check runs after it, so a refusable fader entry keeps that
  variant and its refusal; gate 1 holds the order.
- **A redundant jump moves bits.** The adoption rule compares bits; a jump to the same target
  re-enters the ramp kernel. Gate 3 holds it.

## Objective gates

1. **Classifier** (`crates/host-core/tests/live_delta.rs`, new tests). Adding, changing or removing
   a fader entry gives `Err(LiveRebuild::Automation)`. A mute (row 6) entry change gives
   `Ok` with no records. A static `left_db` change on a lane a `left` fader entry automates gives
   `Ok` with no records; the same change on the right lane gives one `FaderDb` for `Right`. A
   static change together with an automation change gives `Automation`. A fader entry in unit
   `linear` gives `AutomationTarget` (#1645 D5), not `Automation`.
2. **A changed curve rebuilds and carries** (`crates/capi/src/runtime/live_tests.rs`, new). A
   playing `long_session` with a linear fader ride on track 0 takes an `UpsertAutomation` that
   changes the ride. The response path is `rebuild`; the watermark publishes `EXACT` at adoption
   (#1314); no source is reseeked. From the completion sample of the first grid ramp that starts
   after the adoption jump ends, every block is bit-identical to a fresh plan of the new session,
   session-seeked (#1642) to the adoption block's timeline sample and fed the same PCM.
3. **An unchanged curve gives no jump** (same file). A transaction that adds a muted track and keeps
   the ride: every block from the swap is bit-identical to the uninterrupted render of the old
   session.
4. **Removal** (same file). Removing the ride: from the end of the release jump, every block equals
   a fresh plan of the new session (static fader) from the same timeline sample.
5. **Static edit, both hosts** (same file; `hosts/host-web/src/tests.rs`, new). A `left_db` change
   on the automated lane: path `model_only`, the same epoch, every block bit-identical to the
   uninterrupted render. In the browser, a live fader edit on the automated lane, through the
   Worker's apply, replies `path: "model_only"` with the revision advanced by 1,
   and every block is bit-identical to the uninterrupted render; the same edit on the other
   lane replies `live`.
6. **Realtime.** The swap block allocates and frees nothing (`bench_support::alloc` thread counters
   in `crates/host-core/tests/successor_swap.rs`, statics warmed).
7. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p host-core --features host-core/test-support --test successor_swap`,
     `cargo test --locked -p capi`, `cargo test --locked -p graph --features test-support`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (all
     violation counts 0)
   - `bash scripts/check-capi-abi.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-graph-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
8. **No rendered bit moves** for a session with no stored automation: `audit capi` reports the same
   `pcm_digest` at base and head (PR evidence), and the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if the mask still hides fader entries, hides nothing (a mute entry would rebuild), or
  if a static edit on an automated lane still emits a record that fights the curve, or if D1's check
  runs before the refusal route (a refusable entry would report `Automation`).
- Gate 2: red if the cell does not carry (the lane restarts), the cursor is not repositioned, or
  the adoption jump is missing or has the wrong length.
- Gate 3: red if adoption emits a jump to an unchanged target.
- Gate 4: red if a removed entry leaves the lane on its last automated value.
- Gate 5: red if a static edit on an automated lane rebuilds or retargets, or if either host has a
  path that writes a live record onto an automated lane.
- Gate 6: red if the carry of cell event state allocates or frees on the swap block.

## Dependencies

- #1647 *Render moving stored fader automation, seeks and latency* (same batch). It brings
  #1646's cells and event state, #1637's reposition, #1642's session seek (gate 2's
  reference) and #1645's D5 route (which runs first).
- *Carry fader, mute and pan ramps across a plan swap* (#1277): the stage carry D2 extends.
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314), for `EXACT`.
- *Refuse automation on effect parameters that are not block-rate* (#1335): its step 4 and
  `LiveRebuild::AutomationTarget`.
- Batch: R1, in one push with #1646, #1647 and #1649 (README "Must-land-together groups"): D3 drops
  the `FaderDb` that a VCA ride gives an automated member, and #1649 adds the offsets-cell write
  that makes the ride heard. *Admit browser live edits in the Worker through the committed model*
  (#1382) arrives through #1646.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **The classifier changed shape** (decision 17 C6, #1554). `classify_live_delta` masks control
  values with `controls::mask_values`, diffs rows with `controls::changed_controls` and lowers by
  family (`crates/host-core/src/live_delta.rs:240-300`); the stored automation is masked whole at
  `:359-361` and re-checked by the row rule at `:372-376`. D1's predicate becomes
  `controls::automation_row_renders(address: &ControlAddress) -> bool` (classification rules live in
  `controls`, C1), true only for builtins control 5 in this slice. D1's new step compares the
  entries whose rows render; `LiveRebuild` gains `Automation` beside `AutomationTarget`.
- **D3 is split by OQ1 = C.** For the fader row, the static edit is no longer `model_only`:
  #1650 makes `SET_CONTROL fader_db` on an automated lane write the lane's offset (live). D3's rule
  ("a static edit of an automated cell gives no record and commits `model_only`") stays the rule
  for the rows where automation wins, and its predicate `automated_cell` (in `controls`, over
  `ControlAddress`) is the one later slices use (#1654, #1657, #1658 for polarity, #1660, #1665, #1667). Gate 1's
  "static `left_db` change ... gives `Ok` with no records" and gate 5 move: the fader case becomes
  #1650's gate 2; the first automation-wins row that renders (pan, #1657) takes gate 5's
  `model_only` case on both hosts.
- **VCA rides.** #1247's step is now the effective diff (`EffectiveDiff`, C6 step 3); D3's note on
  VCA moves reads against it.
- **The browser** commits through the same `SessionState` (#1562a). An automation edit is a rebuild,
  which the browser refuses with `web.apply.rebuild_pending` until #1562b; so #1562b is a dependency
  of batch R1, and gate 5's browser leg runs after it, through
  `hosts/host-web/tests/control_parity.rs`.
- **Moved files.** `crates/capi/src/runtime/{control.rs,compile.rs}` are in `crates/control-plane/src/`.
  The tests that pin today's model-only automation edit are `model_only_edits`
  (`crates/host-core/tests/live_delta.rs:1125-1154`) and its C ABI twin in
  `crates/capi/src/runtime/live_tests.rs`. Realtime policy as in #1637's amendment.
- **Dependencies.** #1277, #1313, #1314 and #1335 have landed; add #1562b. Must-land group: #1646,
  #1647, #1648, #1649, #1650.

## Amendment (controls coordinator, batch 13b, from #1638's reviews)

- **D2 also carries #1647 D1.6's state.** The per-lane, per-channel obligation to move to the
  automation value at a held ramp's end carries with the cell's event state, and the successor
  re-reports a carried held ramp to the cell with `hold_until` at adoption (`CellState::reposition`
  drops it).
- **Release cells too.** A release cell (D2, "A cell lost") takes the same carried obligation and
  the same `hold_until` report at adoption. Its jump to the static value is a gain event: inside a
  held ramp the stage only remembers it, so the obligation must move the lane to the static value at
  the ramp's end, as #1647 D1.6 does for a carried cell.
- **Seeding a cell's target.** D2 starts a gained cell from the carried stage target and a release
  cell from the carried state, and #1646 D2 starts each cell at its prepared value. #1637's
  `CellState` (`crates/automation/src/events.rs:106-155`) has no way to set its current target from
  outside: `new` leaves it empty and `reposition` keeps it. Authorized paths gain
  `crates/automation/src/events.rs` (one seeding method on `CellState` that sets the current
  target, and its test), because no other path can give a cell the target these rules start it
  from.

## Amendment (controls coordinator, batch 13b, from #1639's review; root ruling 2026-10-15)

- **Jump lengths follow a rebuild.** #1648 D1 makes every automation edit on a row that renders
  a rebuild, which seeds the plan's jump-length cell from the committed model (#1639 D4). A row that
  does not render yet reads no length, and its rendering slice (#1651, #1657, #1658) makes its
  edits rebuilds the same way, so the cell is right from the first block any row reads it.
- **Gate.** Add to the objective gates: `control_smoothing` changed on a session with no reading
  automation, then a fader entry added: from the adoption block, the block holds the new lengths
  (`graph::test_only_block_jump_lengths`).

## Amendment (controls coordinator, batch 17 spec check, 2026-10-16)

Checked against `31fa33fee`. This amendment overrides the filed text and the earlier amendments
where they differ.

- **Anchors moved.** `classify_live_delta` `crates/host-core/src/live_delta.rs:341`, its body
  `classify_live_delta_with_monitor` `:360-466`; the automation mask line `:376-378`; the step list
  in the rustdoc `:252-285` and the model-only paragraph `:287-302`. The steps are now: 1
  `Structure`, 2 `AutomationTarget` (`:388-395`, #1335, #1566, #1631, #1645), 3 changed controls, 4
  unresolvable effect params, 5 lowering. D1's new check runs after step 2 and before step 3.
  `LiveRebuild` is `crates/host-core/src/prepare.rs:4818-4838`; `PlanStateInventory` `:728-789`;
  `SuccessorBase` `:1287`; `across_withdrawn` `:1240`; the carry program `:2467-2486` and its
  install `:3147-3150`; the strip join `join_strips` `:5087-5160` (`StripJoin` `:5031`).
  `CellState` is `crates/automation/src/events.rs:91-155`. Tests: `fader_ride`
  `crates/host-core/tests/live_delta.rs:1103`, `model_only_edits` `:1282-1303`,
  `model_only_edits_are_live_with_no_records` `:1307`, `eq_automation` `:3232`; the C ABI twin
  `model_only_edits` `crates/capi/src/runtime/live_tests.rs:2356-2420` (the `control_smoothing`
  case now comes before the upsert, #1639) and `model_only_edits_commit_without_a_plan_rebuild`
  `:2430`. Docs: `crates/capi/include/miso_engine_v1.h:46-47`,
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:21` (the "#1260" sentence),
  `docs/C_ABI_V1_QUALIFICATION.md:623-630`.
- **Landed deps and what they give.** #1645 (step 2 route), #1637 (`CellState`, `reposition`,
  `hold_until`; no seeding method yet), #1638 (the timed fader stage), #1639 (the jump-length cell;
  a successor seeds it from its own model, `a_successor_prepared_with_a_control_smoothing_edit_holds_the_new_lengths`),
  #1277/#1280 (the strip join and carry retarget), #1312 (cells; `LatestCell<N>` is generic, so no
  new cell form), #1309 (`commit_live_once`, `crates/control-plane/src/control.rs:2905`: any
  classifier `Err` is `LiveCommit::Rebuild`, so D1 needs no control-plane edit), #1641 (every plan
  carries the timeline), #1642/#1643 (session seek: `miso_engine_v1_session_seek`,
  `SessionState::session_seek`, gate 2's reference), #1698 (`across_withdrawn`), #1562b (the
  browser rebuild path; on a page without cross-origin isolation it is a blocking rebuild, counted).
- **Departure 1: two classifier paths.** #1612/#1613 added a fast path (`classify_rows`,
  `:557-734`, and the in-place `classify_live_staged`) for transactions of `SetControl`s. It
  assumes "the edits leave the automation table as committed" (`:576-577`). D1 holds there today.
  It stops holding when #1650 makes `SET_CONTROL fader_db` write an automation entry's offset; see
  departure 3.
- **Departure 2: D3 goes in the shared level lowering.** VCA and own-fader records come from
  `LevelDiff::level_records` (`live_delta.rs:1015-1019`), implemented by `EffectiveDiff`
  (`prepare.rs:4526`, the full path and the strip join) and `LocalDiff` (`live_delta.rs:1246`, the
  fast path). D3's "no `FaderDb` for an automated lane" must be applied there, once, so the full
  path, the fast path and a successor's carry retarget (`join_strips`, #1277 D5) all obey it. A
  suppressed lane gives no record, not an error, so the stage still joins as `Same` and carries.
  No #1277 predicate edit is needed: an automation change moves no static level value.
- **Departure 3: D1's comparison excludes the offset.** #1650 stores the hand offset in the entry
  and makes an offset-only change `live` (#1650 D5). D1 compares each rendered entry's `id`,
  `target` and `segments` (canonical bytes), never its `offset`.
- **Departure 4: the adoption rule compares the composed gain.** A rebuild can also change `S`
  (a VCA or hand-offset edit in the same transaction). D2's adoption test is: if
  `checked_fader_gain(vca_compose_db(curve(t), S_successor))` differs in bits from the carried
  stage gain target, jump. A comparison on the curve value alone would keep the old `S` heard.
  The same holds for gained and release cells.
- **Departure 5: D3's gate cases move.** Under #1650 the static fallback of an automated fader lane
  is no control value (the row reads the offset), so a static change by any edit other than
  `SET_CONTROL` is step 1 `Structure`, a rebuild that seeds the release value. Gate 1's static
  `left_db` case and gate 5 are therefore not this slice's: the fader hand move is #1650 gate 2, the
  VCA case is #1649 gate 5, the `model_only` case is #1657's. Gate 1 keeps the add, change, remove,
  mute-row, combined and `linear`-unit cases. Under #1650's binding the combined case (a static
  change and an automation change) can be step 1 `Structure` before D1's check; it asserts a
  rebuild, not the variant. A test written here and superseded by #1649 or #1650 in the same push
  is not written.
- **Inventory and supersession.** The cell-address rows join `PlanStateInventory` and must pass
  through `across_withdrawn` unchanged, so a superseding candidate carries cell state from P0 as
  #1698 carries every other owner.
- **The #1639 gap closes here.** A fader automation edit is a rebuild, and the successor seeds its
  jump-length cell from its own model, so an entry added after a `control_smoothing` change that
  was not written (no reading entry then) reads the new lengths from its first block. The batch
  13b gate stands, in `crates/capi/src/runtime/live_tests.rs`.
- **`automation_row_renders`** is one function in `crates/controls/src/automation.rs`. #1646's
  preparation and this slice's classifier both call it; whichever lands first adds it.
- **Authorized paths, as they now must read:**
  - `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`
  - `crates/host-core/src/prepare.rs` (`LiveRebuild::Automation`, the inventory cell rows, the
    join, `across_withdrawn`, release cells, the `level_records` suppression in `EffectiveDiff`)
  - `crates/controls/src/automation.rs`, `crates/controls/src/lib.rs` (`automation_row_renders`,
    `automated_cell` and their exports), `crates/controls/tests/automation.rs`
  - `crates/automation/src/events.rs` (one seeding method, batch 13b amendment)
  - `crates/builtins-compiler/src/lib.rs` (the fader and fused bank processors' carry of cell event
    state), `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the strip move only)
  - `crates/host-core/tests/successor_swap.rs`, `crates/capi/src/runtime/live_tests.rs`,
    `hosts/host-web/tests/control_parity.rs`
  - `crates/capi/include/miso_engine_v1.h` (comment), `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
    `docs/C_ABI_V1_QUALIFICATION.md`
  - No path under `crates/engine/src/realtime/`. `crates/control-plane/src/` drops out (no edit
    needed); `hosts/host-web/src/tests.rs` drops out.
- **Gates' test files.** Gate 1 `crates/host-core/tests/live_delta.rs`; gates 2-4 and the #1639
  gap gate `crates/capi/src/runtime/live_tests.rs`; the browser leg of gate 2
  `hosts/host-web/tests/control_parity.rs`; gate 6 `crates/host-core/tests/successor_swap.rs`.
  `bash scripts/check-realtime-policy.sh` no longer exists: use
  `cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`.
- **Size and split.** About 16-20 implementer hours: D1 and the docs 3 h, the suppression hook 1 h,
  D2's carry (rows, join, reposition, composed-gain jump, gained and release cells, the held-ramp
  obligation, `across_withdrawn`) 8-10 h, tests 5 h. Split into two attempt-bounded slices in the
  same push: **#1648a** classification (D1, D3's hook and predicate, D4, gate 1, the #1639 gap
  gate) and **#1648b** carry (D2 and the batch 13b items, gates 2-4 and 6).

## Rulings (root, batch 17, 2026-10-16)

No change to the decisions, paths or gates of this issue. Two notes, so a brief does not assume the
other options.

- **R17-1 = A closes departure 1.** The automation entries that drive a lane are an owner of the
  in-place edit (`OwnerEdits`, `OwnerSlots`, `ModelView`). The fast classifier path (`classify_rows`)
  therefore sees the automation table, and its assumption "the edits leave the automation table as
  committed" holds with the offset write of #1650. D1's comparison still excludes the offset
  (departure 3).
- **R17-6 narrows #1650 to the fader.** The 2026-10-14 amendment lists #1654 and #1658 among the
  rows that use `automated_cell` for the "automation wins" rule. For #1654 that is the send matrix
  rows and the send mute only. The send gain (#1654) and the trim (#1658) take a hand offset, not
  `model_only`, from their own slices, and their rule is #1650's. Polarity stays "automation wins".
  No step here changes.

## Decision record: #1648a (batch 17)

Classification only (D1, D3's hook and predicate, D4, gate 1, the #1639 gap gate). Uncommitted on
`codex/controls-batch-17w` over `b84450183` for the coordinator.

**What shipped.**

- **`controls` (thick core).** `automation_row_renders` was already added by #1646b and is reused.
  New: `automated_cell(model, &ControlAddress) -> bool` (a rendered entry on the same owner,
  section and row whose lane drives the address's lane; a `both` address is automated when either
  lane is), `automated_fader_lanes(model, strip) -> [bool; 2]` (the fader row through
  `automated_cell`, early out on an empty table), and `rendered_automation_changed(current, next)`
  (the rendered entries in table order, compared by `id`, `target` and `segments`, every `f32` by
  bits; the offset is not compared, departure 3). `fader_automation` shares the private lane rule
  `drives`. Exported from `crates/controls/src/lib.rs`.
- **D1.** `LiveRebuild::Automation` (`crates/host-core/src/prepare.rs`). In
  `classify_live_delta_with_monitor` the new check is step 3, after step 2 (`AutomationTarget`)
  and before the changed controls (now step 4); the later steps renumber to 5 and 6. Step 1 keeps
  masking the whole table; its comment and the rustdoc steps and model-only paragraph now say that
  steps 2 and 3 judge it. No control-plane edit: any classifier `Err` is `LiveCommit::Rebuild`.
  The fast path (`classify_rows`) needs no edit: it only runs on transactions of `SetControl`s,
  which leave the table as committed (R17-1 keeps that true under #1650b).
- **D3 hook.** `fader_records` gains `automated: impl FnOnce() -> [bool; 2]`, called only when a
  lane's gain bits change; an automated lane's `gain_changed` is cleared, so it gives no `FaderDb`
  and no error, and the mute records are unchanged. Both `LevelDiff` forms pass
  `controls::automated_fader_lanes(self.rules.next.structure(), strip id)`:
  `EffectiveDiff::level_records` (full path and `join_strips`, which then reads the strip as
  `Same` and carries) and `LocalDiff::level_records` (fast path).
- **D4.** `crates/capi/include/miso_engine_v1.h` (live-edits paragraph),
  `docs/CONTROL_PROTOCOL_SEMANTICS.md` (the "#1260" sentence), `docs/C_ABI_V1_QUALIFICATION.md`
  (the model-only item narrowed, a new "Stored fader automation (#1648)" item). The text states
  the carry ("the successor carries every owner"), which #1648b delivers in the same push.

**Departures and readings.**

- *Superseded cases moved, per the hazard.* `model_only_edits` in
  `crates/host-core/tests/live_delta.rs` now upserts and removes an EQ band-1 gain entry
  (`eq_automation(.., 4, Db)`), and `model_only_edits_are_live_with_no_records` runs on
  `with_effects()` (it has the EQ insert; `two_tracks` has none). The C ABI twin upserts an entry
  on the console EQ slot's band-1 gain (`Section::Console("eq")`, control 4, left, dB).
- *No static `left_db` case* (departure 5): a test superseded by #1650 is not written. D3's hook
  is gated through the VCA move, the one edit in this slice that reaches an automated lane and
  stays a no-`FaderDb` case after #1649 and #1650.
- *Mechanical, outside the path list:* `crates/host-core/tests/live_edit_classifier.rs` gains the
  `LiveRebuild::Automation` arm of its exhaustive verdict match.

**Gates and tests (all pass).**

- Gate 1: `a_fader_automation_edit_is_a_rebuild` (`crates/host-core/tests/live_delta.rs`): add
  (both lanes, one lane), remove, other segments, a zero's sign, retarget to the other track,
  rename are `Automation`; an offset-only change is `Ok`; a mute entry added, changed and removed
  is `Ok` with no records; a static change with an automation change is a rebuild; a `linear` fader
  entry is `AutomationTarget`.
- D3: `an_automated_fader_lane_takes_no_fader_record` (same file): a VCA move under a left-lane
  fader entry gives only the right lane's `FaderDb`; the left lane alone gives no record; the left
  mute keeps its record; with `test-support`, `classify_live_edit` takes the fast path and returns
  the same delta.
- Both hosts: `a_fader_automation_edit_rebuilds_alike_on_both_hosts`
  (`hosts/host-web/tests/control_parity.rs`, four rates): a fader entry replies `rebuild` with the
  same bytes on both hosts and the same bits across the swap; a mute entry replies `model_only`.
- #1639 gap: `a_fader_entry_added_after_an_unwritten_smoothing_change_reads_the_new_lengths`
  (`crates/capi/src/runtime/live_tests.rs`): the smoothing change is `model_only` and leaves the
  prepared lengths; the fader entry is `rebuild`, and after adoption the block holds the new lengths.
- Gate 8: `./target/release/audit capi` `pcm_digest` `bffc21e857b5e134` (unchanged), 0
  violations, 0 allocations.

**Checks run.** `cargo test --locked -p controls`; `-p host-core --all-features` (all binaries);
`-p control-plane`; `-p capi` (175 + integration binaries); `-p host-web --features
host-web/test-support --test control_parity` (22) and `--lib -- automation`;
`bash scripts/check-capi-abi.sh`; `cargo fmt --all`; `cargo clippy --locked -p controls -p
host-core -p capi -p host-web -p control-plane --all-targets --all-features -- -D warnings`; the
same crates with `cargo check --locked --all-targets` and no features.

**For #1648b.** Until the carry lands, a fader automation edit rebuilds with today's join: the
fader stage carries its ramp as `Same` when no static level moved, but the cell event state does
not carry (`adopt_lane`), so the successor's cell starts at its prepared value. The D3 hook already
makes the join read a static change on an automated lane as `Same`, so #1648b's carry sees no
retarget record for it.

## Decision record: #1648b (batch 17)

The carry (D2, the batch 13b items, gates 2-4 and 6). Uncommitted on `codex/controls-batch-17k`
over `5e695ecb6` for the coordinator.

**What shipped.**

- **The seam (`builtins-compiler`).** `FaderBankProcessor::adopt_lane` (both bank kinds: the fused
  fader-matrix bank forwards to it) calls `LaneFaderAutomation::adopt` after the stage import. Each
  automated channel of the successor lane takes, by its stable address (the strip move pairs the
  strip; the channel index is the lane), the carried `CellState` and `HeldRampObligation`
  (`owed`, `due_at_entry`), and `fresh` is cleared. It moves only the event state and the
  obligation: the successor keeps its own offsets cell and `S`. Plain copies at the swap block, no
  allocation. Each channel then owes its first block an `Adoption`:
  - `None`: the table continues bit for bit (`continued_cells`), at the same `a(n)` and the same
    `S` bits. The state continues verbatim, so the events are the uninterrupted ones (gate 3).
  - `Compare`: the same table and `a(n)`, but `S` moved. Only the adoption rule runs.
  - `Reposition`: a changed table, a moved `a(n)`, or a gained cell (no carried cell). The cursor
    is repositioned in the cell's own table at the block's node time. The stage's held ramp is
    reported again with `hold_until`, because `reposition` drops it. Then the adoption rule runs.
- **The adoption rule (departure 4).** At the first block, if the span has no seek step, this
  test runs: `checked_fader_gain(vca_compose_db(curve(t0), S_successor))` against the stage's
  fader gain (`BuiltinFaderBank::fader_gain`, the gain the lane aims for whatever its mute). If
  the bits differ, the block opens with a ramped landing (`SeekMode::Ramp` step at offset 0, over
  the block's fader jump length, to the curve at the jump's completion). That sets the cell's
  target and holds the grid for the jump. Inside a held ramp the stage only remembers it, and
  #1647c's obligation moves the lane at `E`. If the bits are equal, a repositioned cell is seeded
  with `curve(t0)` (`seed_target`).
- **The join (`host-core`).** `StripJoin::continued_cells` comes from
  `controls::continued_fader_lanes(committed, model, strip)`: both models drive the lane with
  bit-identical segments. It reaches the fader move as `graph::GraphStripMove::continued_cells`,
  and `adopt_strip_lane` gains that argument. `committed` is P0's model across a withdrawn plan,
  so the flag needs no `across_withdrawn` change (#1698).
- **The release ("a cell lost").** `EffectiveDiff::join_level_records` (used by `join_strips`
  only) adds a `FaderDb` of the lane's effective `next` value, over the lane's fader length. It
  does this for each lane that `current` automates and `next` does not, and that has no
  `FaderDb` yet. The successor's cells receive it as a carry retarget (#1277 D5).

**Departures.**

1. **No release cell.** A released lane is moved by the join's carry retarget record, not by a
   table-less cell in the successor. The successor has no slot storage for a lane it does not
   automate, and a cell would need new preparation plumbing and bytes for every such lane. The
   record goes to the static value over the fader length, from whatever gain the stage holds.
   The stage's live-write rule then gives the batch 13b behaviour: inside an unmute ramp the write
   retargets and ends the hold, and while muted the stage remembers it for the unmute. If the
   target is already equal, the ramp goes from `g` to `g`. A plan with no live cells cannot
   receive a record, so there the strip restarts at its prepared static value.
2. **Verbatim continuation for an unchanged table.** The filed rule ("reposition each carried
   cursor; jump if `curve(t0)` differs") jumps on every swap of a moving curve. Mid-ride the
   carried target is the grid's completion value, not `curve(t0)`, and gate 3 measured this (red
   with `Reposition` forced). So a cell whose table, `a(n)` and `S` all continue keeps its state
   as it is. The table identity is decided at preparation from the two models (an exact segment
   compare, off render) and carried in the strip move, so render never compares tables.
3. **A seek in the adoption block wins.** If the adoption block has a seek step, the seek lands
   exactly, as #1647 D3 says, and no adoption jump is made.
4. **Superseded #1647b test rewritten.** `a_lane_a_successor_adds_starts_at_its_curve` (a gained
   lane on a carried stage landed a `Set`) is now `a_lane_a_successor_adds_jumps_to_its_curve`
   (`crates/host-core/tests/stored_fader_automation.rs`). Per D2 the lane ramps over `J` and then
   equals the fresh reference. The `Set` of a restarted stage's fresh lane (#1647 D2) is no longer
   gated by a host-core test; the C ABI raised-floor seek test still reaches the fresh-lane path.
5. **Gates 2 and 4 use a stateless session** (`stateless_session`, a long source). A fresh
   reference after a session seek matches a playing plan only when no strip has IIR or other
   history state. Gate 3 runs on the same session.
6. **Out-of-path edits, one each:** `crates/builtins/src/lib.rs`
   (`BuiltinFaderBank::fader_gain`, the one accessor) and
   `crates/host-core/tests/stored_fader_automation.rs` (deviation 4). The scalar oracle never
   adopts (strip carry is bank-only), so its `ChannelHold::fader_gain` is `None`.

**Gates and tests (all pass, x86-64 AVX2).**

- Gate 2: `a_changed_fader_ride_rebuilds_and_carries_into_its_jump`
  (`crates/capi/src/runtime/live_tests.rs`). Path `rebuild`, watermark `EXACT` at `T`, source
  generation still 1. In the jump's last block the lane still differs from the seeked fresh
  reference. From `T + 960 + 63` every sample is equal.
- Gate 3: `an_unchanged_fader_ride_carries_with_no_jump` (same file). Every block from the swap is
  bit-identical to the uninterrupted render.
- Gate 4: `a_removed_fader_ride_releases_to_the_static_fader` (same file). The lane differs inside
  the release. From the release's last sample it equals the seeked fresh static plan.
- Browser leg: `a_fader_automation_edit_rebuilds_alike_on_both_hosts`
  (`hosts/host-web/tests/control_parity.rs`, four rates) also changes and then removes the ride.
  Both get the same verdict and reply (`rebuild`), and the same bits across each carried swap.
- Gate 6: `fader_automation_carry::carrying_fader_automation_allocates_and_frees_nothing`
  (`crates/host-core/tests/successor_swap.rs`, Simd8 and Simd4, live controls). It covers a
  changed ride, a gained left cell and a released right cell. The hand-over plus the successor's
  first block give `((0, 0, 0), (0, 0))` and `Carried`. A kept ride with an added muted track is
  bit-identical to the uninterrupted run.
- Red on revert, checked once each: with no `adopt` call, gates 2 and 3 and gate 6's kept-ride
  check go red. With `Reposition` forced, gate 3 and gate 6's kept-ride check go red. With the
  join reading `level_records` (no release), gate 4 goes red.
- `realtime-policy`: ok, 159 regions in 33 files (unchanged). `./target/release/audit capi`:
  `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0 allocations.
  `check-capi-abi`, `check-host-core-policy`, `check-graph-policy` and `check-workspace-policy`
  all pass.
- Bytes: `Adoption` (one byte) sits in `AutomatedFaderCell`'s padding and
  `GraphStripMove::continued_cells` in the move's padding. No existing byte gate moved.

## U3 fold (batch 17)

Fold of the U3 review, part U3a (render and carry), on `codex/controls-batch-17k` over
`86271e096`, committed as `9dcf288de`. **Attempt count:** #1648 is at attempt 2 of 5 with
this fold (the implementation #1648a/#1648b, then this fold).

**MAJOR 1: a live level value pending at a swap lost its ramp, and a superseded write went
uncounted.** The plan-swap carry drain (`FaderBankProcessor::apply_pending`, which the fused bank
calls too) did not read the offsets cells, and adoption compared the successor's prepared `S`
with the predecessor's heard one, jumping over the fader length `J`. Now:

- The carry drain reads each carried automated lane's offsets cells with `LiveSnapshot::ALL`
  through `read_counted` (`LaneFaderAutomation::carry_offsets`) and keeps the newest record in
  the cell (`AutomatedFaderCell::carried_record`), without moving the stage.
- `adopt` carries that record, the heard `S` and its ramp word, and remembers the successor's
  prepared `S` (`adopted_sum`) until the first block.
- At the successor's first block the drain applies the record through `offsets_jump` from the
  carried `S`, as the uninterrupted block does. A record in the successor's own cell (the join's
  retarget, or a live write committed after the rebuild) replaces it unheard and counts one
  superseded value.
- The join now delivers an `S` change as an offsets record. `join_level_records` returns
  `LevelRecords`, and the carry retarget writes the offsets records to the successor's cells,
  with the edit's length (`LevelRules::lengths`, the live path's rule). So a VCA move in the same
  transaction as a structural edit ramps over its own length, not `J`.

**Departure (Compare).** `Adoption::Compare` is no longer decided at `adopt`. With no record of
its own, the first block raises it when the `S` left after the carried record still differs
from the prepared one. Live plans always deliver a record, so this happens only on a plan with
no live cells: the join keeps an offsets-only change `Same` there, and the lane takes the
adoption rule over `J`. There a VCA edit's own length is not honoured. That is the one case
found, and no production host runs it (both hosts attach live cells). No watermark flag reports
it, because a plan with no live cells has no live values. A table change (`Reposition`) still
takes its adoption jump over `J`, even when the same transaction also moves `S` (D2).
`Adoption` is now ordered, and `adopt` keeps the stronger of its own adoption and one the
carried cell still owed (NIT 3).

**Compare reachability (root ruling B17-R12): unreachable from every shipped host.** Proof: the
join (`join_strips` -> `join_level_records`) writes an offsets record to the successor's cell
whenever the transaction moves a carried automated lane's effective `S`, and it is `Live` (not
`Same`) exactly when the successor attaches live controls (`live_strips = attach_live`). Every
host preparation attaches them: the C ABI (`crates/capi/src/runtime/mod.rs`, both
`prepare_host_runtime_with_live_lanes[_successor]`) and the browser (`prepare_web_host`, its
three branches, `..._with_live_controls_and_spectrum[_collection]_successor` and
`prepare_host_runtime_with_selected_meters_successor`) pass `Some(live_controls)`, whatever the
`HostLiveLanes` selection, and `live_control_strips` lists every track and submix. So on a
shipped plan the first block always reads the cell's own record and never raises `Compare` for
an `S` change. Only host-core's plain entries (`prepare_host_runtime_successor` and the other
`None` forms, which no shipped host calls) prepare a lane with no live cells, where `Compare` is
the backstop and ramps over `J`. `read_offsets` now carries
`debug_assert!(superseded.is_none(), ...)` at the raise with this proof (`superseded` is `Some`
exactly on a lane with live cells); in release the action stays the `J` ramp. Every test of
builtins-compiler, host-core, control-plane, capi and host-web passes with it.

**MINOR 8.** The held-ramp re-report under `Reposition` is in `settle_adoption`, not D1.2, so it
survives U2 fold 2. It changes no sample: a held stage only remembers either operation, and the
obligation follows `free_from`. It is documented there as bookkeeping (M14 stays green). The
reposition's `seed_target` is gated.

**Gates (all pass; each red under its mutation, checked once).** In
`crates/capi/src/runtime/live_tests.rs` (`set_control::automated_carry`, not `canonical_live`):

- `vca_moves_pending_at_a_swap_keep_their_length_and_their_count` (R1c, R9, R10; a moving ride
  and a flat ride; own 4,000 samples, the session's length, two moves, the rebuild committed
  first): bits equal to the uninterrupted engine and `LIVE_VALUES_SUPERSEDED` equal. It is red
  with no carry read, with the carry read uncounted, and with the heard `S` not carried.
- `a_vca_move_inside_a_rebuild_keeps_its_own_length` (reviewer C's case with a VCA move, 4,096
  samples and the session's length, both rides): red with no join offsets write, and red with no
  heard `S`.
- `a_heard_vca_move_then_a_rebuild_continues_the_ride` (R1, mid-jump and settled).
- `an_offset_only_upsert_inside_a_rebuild_renders_as_it_does_alone` (R4).
- `a_held_ramp_obligation_owed_at_a_swap_is_carried` (R6, R7): red with the obligation not
  carried (M15).
- `a_gained_ride_on_a_moving_fader_lets_the_fader_finish_its_ramp` (MINOR 8): red with no
  `seed_target` (M13).

In `crates/host-core/tests/stored_fader_automation.rs`:

- `a_carried_lane_whose_s_moves_with_no_record_takes_the_adoption_rule` (the `Compare` path on
  plans with no live cells): red with the raise removed (M1).

**Not done here.** R2b and reviewer C's hand-move form need #1650b's hand offset, which is not on
this branch. The VCA forms above exercise the same records. R4 is not a discriminating test on
this branch, because the offset is not yet heard (#1650a). Its red check belongs to the merged
tree. (Done on the merged tree: see "U3 fold residue" below.)

**Numbers.** `lane_slot_bytes` 224 -> 304 (the carried record and the adopted `S`, both
channels), measured by `fader_automation_sizes()`. No byte gate moved. `realtime-policy`: ok, 160
regions in 33 files. `audit capi`: `pcm_digest` `bffc21e857b5e134`, 0 violations, 0 allocations.
No edit under `crates/engine/src/realtime/`.

## U3 fold residue (batch 17, on the merged tree with #1650b)

- **R2b.** `a_hand_move_pending_at_a_swap_keeps_its_length` (`set_control::automated_carry`): a
  live hand move of the riding fader (`fader_db`, both lanes, `-6` dB over 4,000 samples), unread
  when a rebuild commits; moving and flat ride. Every block equals the uninterrupted engine, the
  move is heard, the successor is adopted, `LIVE_VALUES_SUPERSEDED` is equal. Red with the carry
  drain (`carry_offsets`) reading nothing, checked once.
- **Reviewer C's case as a hand move.** `a_hand_move_inside_a_rebuild_keeps_its_own_length`: the
  same hand move with an explicit 4,096-sample ramp in the transaction that adds a track (rebuild)
  equals the hand move sent alone (`live`), moving and flat ride. Red with the join's offsets
  records dropped, checked once. No hand-move defect found.
- **R4 red check (merged tree).** `an_offset_only_upsert_inside_a_rebuild_renders_as_it_does_alone`
  is red when the join's offsets record takes ramp 0 (a jump over another length), and red when
  the record is dropped and the `Compare` backstop removed. It stays green when only the record
  is dropped: the upsert sent alone ramps over the session's length `J`, and the `Compare`
  backstop also ramps over `J` to the same bits, so a lost record alone is not observable in that
  case (the hand-move and VCA forms with their own lengths are red under it).
- Part C (B17-R12) is recorded in the U3 fold section above.

## U3 fold 2 (batch 17): attempt 3 of 5

Defect (verifier, at 93146057e): at a `Reposition` adoption (a table-only rebuild of the ride), a
live level edit's own ramp was cut to the fader length `J` on a moving curve, while the watermark
reported EXACT. `reposition` dropped the hold of the edit's ramp, and the adoption rule compared
the curve at the block's first sample with a stage target taken at the ramp's end, so they
differed and the block opened with a landing over `J` (0.047 from the uninterrupted engine).

Fix (`crates/builtins-compiler/src/lib.rs`, `settle_adoption` and the driver's first block; no
edit under `crates/automation` or `crates/engine/src/realtime/`):
- On a free stage still in a ramp (`remaining > 0`), the repositioned cell holds that ramp again
  until its last sample, and the adoption rule compares the curve where the ramp ends (with the
  cell's discontinuity clamp, `curve_toward`, now shared with `offsets_jump`) with the stage's
  target. An unmoved target keeps the edit's own ramp with no landing (an edit applied at the
  first block, pending or in the rebuild's transaction).
- When that target moved (a heard edit whose target the new table gives in other bits, even by
  `f32` rounding), the landing length is `max(J, remaining)`: the edit still ends when it was due,
  on the new table's curve, and a moved target never lands faster than `J`. A landing longer than
  `J` is landed alone as a one-sample piece (`SeekMode::Ramp` over that length), and the rest of
  the block runs from its second sample with `J`, so the block's own discontinuities still jump
  over `J`. A landing of `J` keeps the old path.
- A held stage is unchanged (the batch 13b report of the held ramp).

Gates (`crates/capi/src/runtime/live_tests.rs`, `automated_carry`, C ABI plans):
- `a_level_move_across_a_table_change_keeps_its_length`: a VCA move and a hand move, each with its
  own 4,000 samples; the ride split at 10,000 (equal values up to rounding) and a change only in
  its last 2,000 samples; the move pending at the swap, in the rebuild's transaction, and heard 1
  and 12 blocks before it. 44 blocks, allocation-free, adopted, max `|a - u| < 1e-3`. All 16 cases
  red at 93146057e (0.047, 0.046 heard 1, 0.024 heard 12). With the fix: 0 in 12 cases, 4.6e-6
  heard 12 (the new table's rounding). The hand-move split keeps the offset the move stored.
- `a_carried_move_replaced_unheard_counts_as_superseded` (MINOR): "a rebuild that moves again" and
  "a move, a rebuild, a move", moving and flat ride: every block and `LIVE_VALUES_SUPERSEDED` equal
  the uninterrupted engine's. Both cases red with `superseded.add_carried(1)` removed from
  `read_offsets`, checked once each.
- NIT: the two hand-move residue gates run on C ABI plans (the shipped preparer). Under the
  carry-drain and join-write mutations a debug build stops first at the B17-R12 `debug_assert`
  (the panic in render aborts the test process, red); a release build reaches the bit assert (red),
  checked once each. Recorded in their test value.

Also in this attempt (coordinator): `crates/host-core/tests/stored_fader_automation.rs` selected
`Backend::Simd8` in `form_rig` on every target; the forms are now a `FORMS` list whose eight-lane
entries carry `cfg(target_feature = "avx2")`, so `acted` compares five forms on x86-64 and three
elsewhere. `scripts/check-cross-targets.sh` passes.

## Split (root ruling B17-R13, batch 17)

- #1648 closes on its passing gates. The in-flight Reposition length case moves to
  `TBD50-keep-a-live-level-moves-own-length-across-a-reposition-swap.md`, following the "split a
  blocking gate early" practice (a gate that holds a passed batch goes to a successor after one
  fix round).
- Current difference (verifier, at 0fac00ee9): a live level move still ramping with
  `remaining < J` at a Reposition swap is stretched to `J` by the landing `max(J, remaining)`.
  Gate setup: heard 24, 26 and 30 blocks (928, 672 and 160 samples left; `J` = 960). The rebuilt
  rig differs from the uninterrupted engine by 4.8e-4, 3.1e-3 and 2.1e-3. The watermark marks it
  as a fallback under root ruling B17-R14 (control-time, conservative), which is not landed yet in
  this record: see its decision record when it lands. TBD50 replaces that flag with the exact
  behaviour.
- The step guard (regression fix, `settle_adoption` in `crates/builtins-compiler/src/lib.rs`):
  when the cell's program has a discontinuity exactly at the block's first sample
  (`program.discontinuities().binary_search(&span.start)` finds it), the curve is compared there
  and the landing is `J`. `curve_toward` looks only past `start`, so before the guard the landing
  ran over `remaining` and missed the step's `J` jump (8.6e-2 from a fresh plan).
- Gate `a_step_at_a_reposition_swap_jumps_over_the_fader_length`
  (`crates/capi/src/runtime/live_tests.rs`, `automated_carry`): VCA and hand moves of 4,000
  samples, heard 1 and 12 blocks, then a rebuild whose ride steps to -20 dB at the swap's first
  sample or one sample after it. The reference is a fresh plan prepared with the stepped ride and
  given the same move. From the step plus `J`, max |a - r| < 1e-3 (measured: 7.8e-5 heard 1,
  5.2e-4 heard 12), allocation-free, adopted. Red without the guard (8.6e-2, step at the swap).
  Red when the long landing runs over the whole block (verifier's mutation m5: 8.6e-2, step one
  sample after the swap; this is MINOR 3). Each was checked once.
- MINOR 4 (`at = end` has no gate) moves to TBD50. Probe finding: for every heard move on the
  moving ride, the stage's `fader_gain` at the swap is not the composed curve at the ramp's end
  (0.398 at 12 blocks heard, 0.335 at 24, against 0.3018). The comparison therefore differs at
  either point, and `at` cannot be seen by the current tables.
- NIT 5: the doc of `a_level_move_across_a_table_change_keeps_its_length` now gives the measured
  numbers (0, and 4.6e-6 at 12 blocks heard, where the landing recomputes the stage's ramp step).
  NIT 6: D2 has a note that points to the fold's longer landing and the step rule.
- Attempt count: attempt 3 of 5 plus this ruling's landing. There is no attempt 4.
