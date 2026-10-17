# Render stored send gain, mute and matrix automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), owner answer OQ2 = yes (2026-10-14, README "Owner answers"), under decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`, C3, C6 and C10). A draft for root to file.
Written by the controls coordinator on 2026-10-14. Code anchors verified on `12d4a4f9d` (batch 9
head). Batch R2, after #1651-#1653.

## Product outcome

A producer's stored send rides play on every host: a route into a submix follows its stored
`gain_db`, `mute` and channel-matrix curves (route rows 1-6) in the node time of its tap, so the
send moves with the audio it carries, latency included. A send automation edit is a carried
rebuild. A hand edit of an automated send value follows the owner's OQ1 rule: automation wins on
the gain and the matrix (the edit is stored and commits `model_only`), and a hand send mute ORs
with the automated send mute (D5, a reading for root to confirm). A live edit of a send value the
session does not automate, on a send that has an automated value, is a live group-cell write. A
route into the output stays prepared: its rows are not automatable, as today.

## Context

- **The owner answer.** README "Owner answers (2026-10-14)": OQ2 = yes. A route into a submix gets
  `gain_db`, `mute` and `channel_matrix` as automation targets; routes into the output stay
  prepared (decision 13 O9, decision 14's fold reason).
- **The rows exist and are automatable.** Route rows 1 `gain_db`, 2 `mute`, 3-6 `matrix_ll`,
  `matrix_lr`, `matrix_rl`, `matrix_rr`, 7 `follows_mute`, all shared (`lane: both`), family
  `Route`; on a route into a submix they are `live`, on a route into the output
  `rebuild_only(optimisation)` (`crates/controls/src/route.rs:20-176`, `OUTPUT_FOLD` `:31-37`).
  `automatable` is computed (`crates/controls/src/descriptor.rs:128-132`), so the stored automation
  of a send already validates (#1566; `fixtures/session/v1/builtins-automation.json` holds
  `route-ride`), and an output route's automation is refused `control.automation.target`. Nothing
  renders it: the classifier masks the whole table (`crates/host-core/src/live_delta.rs:359-361`).
- **The route lane.** Every route into a submix has a live lane on every plan (decision 17 C6,
  #1557). Its cell is `ROUTE_CELL_WORDS = 6`: the four folded coefficients, the mute flag and the
  ramp (`crates/graph/src/lib.rs:1004-1047`). The control plane folds gain, matrix and the follow
  gate with `graph::gated_route_coefficients` (`crates/graph/src/lib.rs:800`) after
  `route_values` (`crates/graph-compiler/src/ids.rs:322-338`); records come from `route_records`
  (`crates/host-core/src/live_delta.rs:1416`) and `RouteControlProducer`
  (`crates/host-core/src/route_controls.rs:201`).
- **#1652 and #1653** give a route whose source mute is automated a seven-word cell (the open
  transform `gain, ll, lr, rl, rr`, one flags word and the ramp), make the route op evaluate curves
  in the tap's node time `τ_tap(r) = timeline(r + ΣP - a(tap))` (README A1.3, A6), retarget inside
  a block on the indexed route ramp, and write the cell from the shared commit.
- **Ramp rows.** `SendGain` (key `fader_ms`), `SendMute` (`mute_ms`), `SendMatrix` (`pan_ms`)
  (`crates/effect-contract/src/control.rs:70-102`), resolved by `LiveRamps`
  (`crates/controls/src/live_ramps.rs`).

## Decisions frozen for this slice

- **D1. Cells.** Each automated route row is one cell (shared rows: one cell per entry), with
  #1637's builder, in the node time of the route's tap (`a(tap)`, #1652's rule):
  - `gain_db` (row 1): grid period and ramp 64, completion `End`, jump key `SendGain`; the event
    value goes through the route's own dB conversion (`route_transform`, the code `route_values`
    calls), so a flat curve at `v` gives the bits of a static `v`;
  - `mute` (row 2): `step` only (amended #1645's shape rule), jumps over `SendMute`;
  - `matrix_*` (rows 3-6): grid period and ramp 64, completion `End`, jump key `SendMatrix`.
- **D2. The route is one target group.** A route with any automated row uses #1652's
  seven-word cell, also when its source mute is not automated. At each event, at a seek step, and
  at the next block entry after the cell changes, the route op builds the open transform (each
  automated value from its curve at the completion sample, every other value from the cell), the
  gate `RouteGate { mute: curve_mute || cell_mute, follow_zeroed }` with #1652 D2's follow term,
  and the target `gated_route_coefficients(&transform, gate)`. Only a target whose bits change
  retargets, on the indexed route ramp from `coefficients_at(position)` at the event offset (
  #1652 D2's in-block retarget). A seek sets the target exactly where it reaches the tap.
- **D3. Rendering rows.** Route rows 1-6 join #1648 D1's `automation_row_renders`. A route
  automation edit is a carried rebuild: the route lane carries by route ID (#1284 D2, as #1652 D3
  extends it) and each cell's event state by `(route ID, row)`, as #1648 D2 carries a fader cell.
- **D4. Hand edits where automation wins** (OQ1 = C, "other rows"). A `SET_CONTROL` on an automated
  `gain_db` or matrix row stores the fallback value, gives no record and commits `model_only` (
  #1648 D3's rule); the control list shows `automated = true` (#1650 D4).
- **D5. A hand send mute ORs** with the automated send mute: the route's own static `mute` stays a
  live value, written into the cell's flags word, and the gate is `curve_mute || static_mute`. This
  is the reading of OQ1's "mute ORs with the automated mute" for a send mute; root confirms it at
  filing. If root reads the answer as "strip mute only", D5 becomes #1648 D3's `model_only` rule
  and gate 4's mute case expects that.
- **D6. Values the session does not automate.** On a route with an automated row, a live edit of
  another row (gain, a matrix coefficient, mute, or `follows_mute`) writes the seven-word cell
  (`live`), with the ramp `LiveRamps::resolve` gives that row. It never writes the folded six-word
  form for such a route.
- **D7. `follows_mute` (row 7).** The row is automatable by decision 17's computed rule, but the
  owner's answer names only gain, mute and matrix. This slice does not render it: an entry on row 7
  stays masked and inert, as every row no slice renders (A10), and README finding F20 lists it for
  root. If root extends the answer to it, it is one more boolean in D2's flags word and one more
  row in D3, in this slice.
- **D8. Routes into the output.** Unchanged: their rows are `rebuild_only(optimisation)`, so the row
  rule refuses their automation, and the output fold stays.
- **D9. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; every check of a transaction precedes the first cell write; a cell write cannot fail.

## Deliverables

1. D1-D2 in host-core preparation (cells, seven-word cells for automated routes, their seed) and in
   the graph's route op.
2. D3-D6 in `crates/controls` (the rendering rows) and the classifier's route lowering.
3. `docs/SESSION_SCHEMA_V1.md`: send rows render; output routes stay prepared.

## Authorized paths

- `crates/host-core/src/prepare.rs` (route cells and seed), `crates/host-core/src/live_delta.rs`,
  `crates/host-core/src/route_controls.rs`, `crates/host-core/tests/{live_delta.rs,route_mute.rs}`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the route cell and the route op only)
- `crates/controls/src/{automation.rs,route.rs}` and their tests
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `crates/control-plane/tests/send_automation_realtime.rs` (new)
- `docs/SESSION_SCHEMA_V1.md`

## Non-goals

- `follows_mute` automation (D7). VCA automation (README F20).
- Any change to routes into the output (D8).
- Sidechain routes: a sidechain is not a route row.

## Hazards

- **The route op is shared with #1652.** This slice extends the evaluation #1652 builds; it adds
  no second route evaluator.
- **Latency.** The send changes at its tap's arrival, which for a pre-insert tap is earlier in render
  time than the strip's events; after the send's compensation delay both meet on one timeline
  sample (README A6). Gate 2 holds it.

## Objective gates

Commands: `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p graph
--features test-support`, `cargo test --locked -p controls`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --features test-support --test control_parity`,
`./target/release/realtime_policy`, `bash scripts/check-graph-policy.sh`,
`bash scripts/check-cross-targets.sh`, clippy `-D warnings`, `cargo fmt --all -- --check`.

1. **Flat equals static.** A send with a flat gain ride at `-6` dB and a flat matrix entry renders,
   bit for bit, the session with those static values, on the C ABI and in the browser.
2. **A ride is heard at the tap's time.** A linear gain ride on a pre-insert send behind a latent
   insert reaches each grid value at the submix input on the same timeline sample as a twin whose
   send has no latency; a post-fader send's ramps start on the strip's render samples.
3. **Edits.** A send automation edit gives path `rebuild` and completes `exact`; a `SET_CONTROL` on an
   automated send gain gives `model_only` and moves no bit; a `SET_CONTROL` on the same send's
   un-automated `matrix_lr` gives `live` and, after its ramp, the bits of a fresh plan with that
   value.
4. **Mute.** With D5 as written, a hand send mute on a send whose mute curve is open closes the send
   (`live`); when the curve closes it, the hand unmute leaves it closed.
5. **Output routes.** An automation entry on a route into the output is refused
   `control.automation.target`, unchanged.
6. **Realtime.** The route events and cell writes allocate and free nothing on the render thread.

## Test value

- Gate 1 turns red if the route op converts dB or folds the matrix with other arithmetic than the
  control plane.
- Gate 2 turns red if the send's events use the fader's arrival instead of the tap's.
- Gate 3 turns red if a hand edit fights the curve or an un-automated value of the group is lost.
- Gate 4 turns red if the mute composition is not an OR.

## Dependencies

- #1651, #1652 and #1653 (the route op's evaluation, the seven-word cell and its commit path).
- #1648 D1 (`automation_row_renders`), #1650 D4 (`automated` in the list), amended #1645
  (the shape rule).
- Batch R2, after #1653. It is a complete outcome alone; it is not in a must-land group.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Root's rulings (README "Root's rulings (2026-10-14)"): a send gain is a level control and takes a
hand offset; a hand send mute ORs (D5 confirmed); on the send matrix automation wins;
`follows_mute` is not automatable (#1632).

- **D1, gain cell.** For an automated `gain_db`, the seven-word cell's `gain` word holds the hand
  offset in dB (`f32` bits; #1650's offset field, amended for the send gain), not the static
  gain, which is the fallback and is not heard. At each gain event the route op computes
  `g = clamp(f64(curve) + f64(offset), -144, 24)`, rounded once to `f32`, and passes `g` to the
  route's own dB conversion (`route_transform`). An offset of `±0.0` gives the bits of no offset:
  `f64(v) + (±0.0)` differs from `v` only in the sign of a zero, the clamp changes nothing for a
  curve value in the route gain domain, and the conversion of `±0` is the same gain (gate 1 holds
  it).
- **D4 is replaced for the gain.** A `SET_CONTROL` on an automated send `gain_db` writes the entry's
  offset (#1650 D3, amended) and the seven-word cell's `gain` word, with the ramp
  `LiveRamps::resolve(SendGain, the edit's ramp)`; the path is `live`. Render reads the new word at
  the next block entry and retargets on the indexed route ramp over the cell's ramp word, only if
  the coefficients' bits change (#1649 D3's rule on the route). On an automated matrix row D4
  stands: the edit stores the fallback and commits `model_only`.
- **D5 is confirmed** by root: a hand send mute ORs with the automated send mute. D5's "a
  reading for root to confirm" and its alternative are void.
- **D7 is replaced.** `follows_mute` is not automatable (#1632): the row
  rule refuses an entry on route row 7 with `control.automation.target`, so this slice renders none
  and its flags word gains no automated `follows_mute` bit.
- **Gate 3** becomes: a send automation edit gives path `rebuild` and completes `exact`; a
  `SET_CONTROL` on an automated send gain of `+3` gives `live` and, after its ramp, the bits of a
  fresh plan whose entry carries offset `+3`; a `SET_CONTROL` on an automated send matrix row gives
  `model_only` and moves no bit; a `SET_CONTROL` on the same send's un-automated `matrix_lr` gives
  `live` and, after its ramp, the bits of a fresh plan with that value. Gate 4 tests D5 as written.
  Gate 1 adds a flat gain ride with offset `0` and with offset `-0.0`.
- **Test value** gains: gate 3 turns red if a hand send gain move writes the fallback (automation
  wins) instead of the offset, or if the offset reaches the matrix.
- **Non-goals.** "VCA automation (README F20)" is #1655.
- **Dependencies** gain #1650's amended field and rule (batch R1) and #1632 (ordering only: its
  row rule refuses the `follows_mute` entry this slice no longer renders).

## Rulings (root, batch 17, 2026-10-16)

These rulings override the filed text and every amendment above where they differ.

### R17-6: this issue owns the hand offset for the send gain

Under OQ1 = C, a hand move of an automated send `gain_db` adds a stored dB offset. The send offset
(its field use, its write and its gates) moved here from #1650. #1650 now delivers the fader offset
only. This issue depends on #1650, which ships the shared mechanism for the fader: the offset
field and grammar (#1650a), and the offset write, the in-place owner rule and the list flag
(#1650b). The batch 10 amendment above still gives the render half (the seven-word cell and
`g = clamp(f64(curve) + f64(offset), -144, 24)`). Where it says "#1650's offset field, amended for
the send gain", read: the field of #1650 D1, with the rule below.

The rule is the one of #1650, for route row 1 `gain_db` on a route into a submix:

- **The offset field.** The field is the one of #1650 D1. The row is shared (`lane: both`), so its
  entry holds one offset: `left_db` and `right_db` are equal. This issue adds route `gain_db` on a
  route into a submix to `takes_offset`, in the same slice that adds route rows 1 to 6 to
  `automation_row_renders`. `control.automation.offset` also refuses an entry on this shared row
  whose two fields differ. The domain is `[-144, 24]` dB (`ROUTE_GAIN_DB_MINIMUM`,
  `ROUTE_GAIN_DB_MAXIMUM`). Before this issue a non-zero offset on a send entry is refused. The
  matrix rows, `mute` and `follows_mute` take no offset: a non-zero offset on a send matrix row is
  refused.
- **The write.** On a send whose `gain_db` automation renders, `SET_CONTROL` on route row 1 writes
  the entry's offset (both fields, one value), not the static gain. The path is `live`. The control
  list reports the offset as the row's `value` and `automated = true`. The seven-word cell's `gain`
  word and its ramp are written as in the batch 10 amendment.
- **The in-place owner rule (R17-1).** The automation entries that drive the route row are an owner
  of the in-place edit for route row 1: `OwnerEdits`, `OwnerSlots`, `ModelView`, and the row's read
  and write with model scope. A hand move reaches the offset on every path. The reply and the sound
  agree. An ack of a value that does not sound is a defect. The matrix rows keep D4: automation
  wins, the edit is stored and commits `model_only`.
- **Equivalence gate.** The same `SET_CONTROL` on an automated send gain through the in-place path
  and through the full path gives the same reply and the same bits. It runs on both hosts: the C
  ABI (`crates/capi/src/runtime/live_tests.rs`) and the browser
  (`hosts/host-web/tests/control_parity.rs`).
- **Gates moved here from #1650.** An entry on a send `gain_db` whose two offset fields differ is
  refused `control.automation.offset`. A non-zero offset on a send matrix row is refused. Gate 3 of
  the batch 10 amendment (the `+3` hand move) is the render check of this rule.
- **Paths added to this issue's list:** `crates/controls/src/{builtin_rows.rs,owner_edits.rs}` (the
  route row 1 binding and its owner), `crates/controls/tests/**`, and the fields #1650 introduces.
  `docs/SESSION_SCHEMA_V1.md` names the send gain as taking the offset. The batch R2 order stands:
  this issue follows #1650 and #1651 to #1653.

## Amendment (controls coordinator, batch 19 spec check, 2026-10-17)

Checked against `323433f6a`. The filed text and the amendments above stay; where they disagree,
this section wins. #1651, #1652 and #1653 are closed; read their decision records by number.

**Anchors now.**
- Rows: `crates/controls/src/route.rs:22-28` (IDs), `:31` (`OUTPUT_FOLD`), `:101-218` (`ROWS`).
  Row 7 `follows_mute` is `AutomationRule::Excluded` (#1632 landed), so D7 holds as amended.
  `automatable`: `crates/controls/src/descriptor.rs:154`.
- Rendered rows and offsets: `takes_offset` `crates/controls/src/automation.rs:51`,
  `offset_faults` `:62`, `automation_row_renders` `:400`, `mute_row` `:407`, `fader_row` `:419`,
  `automated_cell` `:484`, `rendered_automation_changed` `:527`, `continued_fader_lanes` /
  `continued_mute_lanes` `:573`/`:586`. Code `control.automation.offset`:
  `crates/controls/src/error.rs:37,74`. Docs: `docs/SESSION_SCHEMA_V1.md:337-341`.
- Owner edits (R17-1): `OwnerEdits::set` `crates/controls/src/owner_edits.rs:84` (fader branch
  `:92-97`), `take_fader_automation` `:105`; row read `crates/controls/src/registry.rs:224`, write
  `:565-567`; `ModelView::fader_automation` `crates/controls/src/view.rs:111`;
  `write_fader_offset` `crates/controls/src/builtin_rows.rs:155`; `mask_fader_offsets`
  `crates/controls/src/mask.rs:138`.
- The classifier no longer masks at `live_delta.rs:359`. Step 3 (`rendered_automation_changed`) is
  `crates/host-core/src/live_delta.rs:423`, `lower_routes` `:995`, `route_records` `:1517`.
- Records: `route_record_between` `crates/host-core/src/prepare.rs:4288`, `follow_route_record`
  `:4371`, `follow_route_record_between` `:4410`; predicate `following_route_lanes` `:5081`;
  attach `:2873-2897`; join `join_routes` `:5829`, `resolve_route_carry` `:5919`.
- One program: `prepare_fader_automation`, `crates/builtins-compiler/src/lib.rs:2618-2752`
  (fader cells, then mute cells); `strip_mute_cells` `:5808`.
- Graph: `ROUTE_CELL_WORDS` `crates/graph/src/lib.rs:1010`; `FOLLOW_ROUTE_CELL_WORDS` `:1136`;
  `RouteFollowFlags` `:1146` (bits 0-5 used, bits 6-31 spare and refused by `from_word`);
  `FollowRouteControlRecord` `:1221`; `GraphFollowRouteSource::new` `:1390` (refuses a route with
  no automated source lane). Render: `crates/graph/src/runtime.rs` `LiveFollowRoute` `:1287`,
  `ColumnRamps` `:1318` (`retarget` `:1374`, `one_ramp` `:1413`), `FollowCursors` `:1442`
  (`block` `:1568`), `follow_target`, `follow_walk`, `apply_cell` `:1698`, `begin_block` `:1745`,
  `mix` `:1815`, `finish_six_word_carry` `:5733`. Kernel `route_mix_column_ramps_block`
  `crates/lane/src/kernels.rs:1820`.
- dB law: `route_transform` `crates/graph-compiler/src/ids.rs:293` (`math::db_to_gain_f32`),
  `route_values` `:330`. Ramp rows: `crates/effect-contract/src/control.rs:76,89,101`.
- Commit: pre-write check `crates/control-plane/src/control.rs:3260-3290` (compares the record's
  `automated` with `GraphFollowRouteControlProducer::automated()`); #1713 flag `:2540`, `:2659`;
  B17-R14 term `automates = fader_row` `:2648-2651`.

**How the slice maps onto the landed code.**
- **Rows (D3, R17-6).** `automation_row_renders` gains route rows 1-6 on an `Owner::Route`,
  `Section::Own` target (an output route's entry is refused at `.target` first, D8). `fader_row`
  stays the fader's only predicate, so no route entry reaches the fader cell, the fader offset or
  B17-R14's window. `takes_offset` gains route row 1. `offset_faults` also refuses a route row 1
  entry whose `left_db` and `right_db` bits differ (`control.automation.offset`, at
  `.offset.right_db`). New `route_automation(model, route) -> [Option<&Automation>; 6]` (rows 1-6)
  is the one enumeration that preparation, the classifier and owner edits read.
- **Offset (R17-6).** Mirror the fader: `OwnerEdits::set` takes the route's row 1 entry with the
  route; `registry` reads and writes the offset of an automated row 1 (both fields, one value);
  `mask_values` copies route row 1 offsets as it copies fader offsets; the list sets `automated`
  on rows 1-6 that an entry drives.
- **Program.** The route cells join the plan's one program in `prepare_fader_automation`, after
  the mute cells, in `model.routes` order: rows 1 and 3-6 with `FADER_CELL_RULE` (grid 64, `End`),
  row 2 with `MUTE_CELL_RULE` (step). New accessor `route_cells(route_id)`. The function now also
  runs for a session whose only rendered entries are route entries.
- **Seven-word predicate (D2).** `following_route_lanes` becomes the predicate of "the plan gives
  this route the seven-word cell": source mute automated, or any route row 1-6 automated. It returns
  the plan's full automated set. `GraphFollowRouteSource::new` accepts a route with no source mute
  cell when it has a route cell.
- **Flags word.** Bits 0-5 keep their meaning. New bits, fixed by the plan like bits 4-5:
  bit 6 `automated_gain`, bit 7 `automated_mute` (the route's own row 2), bits 8-11
  `automated_matrix[ll, lr, rl, rr]`. `DEFINED` becomes `(1 << 12) - 1`. When bit 6 is set, word 0
  holds the hand offset in dB (`f32` bits), not a linear gain. `FollowRouteControlRecord::new`
  still checks only finiteness, so the offset word fits. #1653's pre-write check compares the whole
  automated set (bits 4-11) with the producer's, before any write; it stays D9's defence.
- **Records.** `follow_route_record` takes the route's row 1 offset when bit 6 is set. A
  word of an automated matrix row is the static fallback and render ignores it, so a change of
  only such words gives no record and commits `model_only` (D4). A change of the offset is a
  record over `SendGain` (`live`).
- **The dB law.** Graph does not link `math` (pinned by `scripts/check-graph-policy.sh:21`).
  Host-core hands the route op `GraphRouteGainLaw = fn(curve_db: f32, offset_db: f32) -> f32`
  (`db_to_gain_f32(clamp(f64(curve) + f64(offset), -144, 24) as f32)`), defined in graph-compiler
  next to `route_transform`, which calls it for the static gain. One derivation, no policy change,
  in its own realtime-policy region (the `follow_mute_jump` pattern).
- **Route op (D2).** `FollowCursors` becomes one table of up to eight cursors: source mute L/R,
  route gain, route mute, matrix ll/lr/rl/rr, each with its row's jump length (mute rule for the
  mute cells, `SendGain` for gain, `SendMatrix` for the matrix; the jump-length rule returns all
  three). `follow_walk` walks the table in offset order; the values it carries are `f32` (a mute
  value is `!= 0`). The composition at each offset: open transform with
  `gain = law(curve_gain, offset)` if bit 6, else word 0, each automated matrix entry from its
  curve, others from the cell; gate `mute = static_mute || curve_mute` (D5), follow terms as
  #1652. One `follow_target`; no second evaluator. A held event changes no value. The
  retarget length of a column is the event's own `ramp` (`0` for a `Set`), see ruling R19-A for a
  column with a move in flight. `ColumnRamps::retarget` takes one length per column.
- **Record jump (batch 10 amendment, #1649 D3 on the route).** At a record that changes the
  offset or a value word, the target takes each automated value from its curve at
  `completion_of(End, start, R)` (the `curve_toward` rule), and the route's value cells
  `hold_until` that completion, so grid events inside the jump are held. A record that changes
  only gate words keeps today's #1652 rule.
- **Carry (D3).** Today the follow cursors do not carry: a carried route places them at
  `τ_tap` and compares over the mute `J`. For a gain or matrix cell that re-times a running
  ride. Fix, as #1648 D2 / #1651 T4: a route cell whose table both plans hold with the same
  segments (`continued_route_rows`) carries its `CellState` and heard value verbatim, by
  `(route ID, row)`; another one is repositioned and jumps over its own row's `J` (specified
  behaviour, B17-R14 note 1). Source mute cells keep today's placement (a step: exact). A live move
  in flight at a Reposition is ruling R19-B.
- **#1713 flag.** `resolve_route_carry` reads the plan's `GraphLiveRoute::following`, so the
  widened predicate flags every seven -> six carry, including the removal of a send's last route
  entry. Conservative, no new code; #1713's exact form must cover route-row-only routes too.
- **Node time (R17-3), oracle (R17-4), FP (R17-7).** The route op takes `node_span(a(tap))` as
  today; it mixes at `FrameLane` on every backend, so the scalar oracle and both bank widths run
  the same op. Seeds and records compose on the control thread inside the existing
  `CanonicalFpEnv` guards (prepare entry, commit entry); no new guard.

**Gates (additions).** Gate 1 adds a flat route mute curve at `0` and a flat matrix entry, and
runs in host-core at `Simd8` (avx2), `Simd4` and the scalar oracle, and on both hosts. New gate 7:
a running gain ride across an unrelated rebuild (continued table) is bit for bit the
never-swapped twin (host-core, three widths). New gate 8: an entry on route row 1 with
`left_db != right_db` is refused `control.automation.offset`; a non-zero offset on rows 2-6 is
refused. Gate 3's `+3` move runs through the in-place and the full path with equal reply and
bits on both hosts (R17-6). Gates whose result depends on R19-A or R19-B are written after root
rules.

**Rulings needed.** R19-A (a value move and a gate move on one folded column ramp) and R19-B (a
live send move in flight at a Reposition, once #1711 deletes B17-R14's window). See the batch 19
report.

**Authorized paths after the check.**
- `crates/controls/src/{automation.rs,route.rs,registry.rs,view.rs,owner_edits.rs,mask.rs,
  builtin_rows.rs,lib.rs}`, `crates/controls/tests/**`.
- `crates/builtins-compiler/src/lib.rs` (route cells in the one program, `route_cells`; shared
  with #1711 and #1655).
- `crates/graph-compiler/src/{ids.rs,lib.rs}` (the gain law and its re-export).
- `crates/graph/src/lib.rs` (flags, source, record, law and jump-rule types),
  `crates/graph/src/runtime.rs` (the route op, its cursors and the route carry only). Render code.
- `crates/host-core/src/{prepare.rs,live_delta.rs,route_controls.rs,lib.rs}`,
  `crates/host-core/tests/{live_delta.rs,live_routes.rs,route_mute.rs,in_place_commit.rs,
  support/edit_sessions.rs}`.
- `crates/control-plane/src/control.rs` (the pre-write check width; the R19-B term if ruled),
  `crates/control-plane/tests/send_automation_realtime.rs` (new).
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `hosts/host-web/src/tests.rs`.
- `docs/SESSION_SCHEMA_V1.md`.
- Nothing under `crates/engine/src/realtime/`. No `crates/lane` kernel change unless R19-A picks
  option (b).

## Rulings (root, batch 19, 2026-10-17)

These override the batch 19 spec-check amendment where they differ.

- **R19-3 = (d).** A route column uses the fader's deferred-jump rule. While a column jump runs (a gate
  change over the mute `J`, a record ramp `R`, or `0` for a Set), a gain or matrix grid event updates
  the heard values but does not retarget that column; it sets the column's owed bit. At the jump's end
  the column composes again; if the bits differ it lands over the value row's own `J` (`SendGain` or
  `SendMatrix`, the longer one if both are owed), as #1711's deferred jump. A free column retargets at
  once over the event's own ramp. A gate event always retargets at once. The hold is per column, never
  per cell. No `max(own, remaining)` anywhere. Gates: a gain ride across an unmute fade ends the fade at
  exactly `J` and lands at its end; a ride on the left column keeps moving while the right column fades.
- **R19-4.** Events of several curves at one offset take the longest jump among them. Gate it.
- **R19-5.** A live send move (hand offset or a D6 matrix edit) in flight at a Reposition is exact under
  R19-3 and #1711's deferred jump, with no flag. If its gate fails, a send-only conservative
  `TransitionFallback` term (the B17-R14 pattern) is added, and root is told.
- **K1, the shared walk.** The route table uses the merged walk of `crates/automation/src/compose.rs`
  (built after #1711, before T3). The composition (law, matrix, gate) stays in the route op. The cursor
  table is a boxed slice sized at bind and charged by `size_of`, not a fixed array.
- **Two PRs.** This issue ships in batch 19a with #1711 and K1.

## Decision record: T1 (batch 19)

`crates/controls` only. What shipped:

- **Rows (D3).** `automation_row_renders` admits route rows 1-6 through the new `route_row`
  (`Owner::Route`, `Section::Own`, `ROUTE_AUTOMATED_ROWS`); row 7 `follows_mute` stays out (still
  `AutomationRule::Excluded`). The predicate reads no model: an output route's entry is refused at
  `.target` first (D8). `fader_row` is unchanged, so no route entry reaches a fader reader.
- **Offset (R17-6).** `takes_offset` = `fader_row || route_gain_row`. `offset_faults` judges a
  shared row's `left_db` against the domain (at lane `both`) and refuses a `right_db` whose bits
  differ from `left_db`'s at `.offset.right_db` (`+0.0` with `-0.0` is refused). A non-zero offset
  on route rows 2-6 stays `control.automation.offset`.
- **Accessors.** `route_automation(model, route) -> [Option<&Automation>; 6]` (rows 1-6, first entry
  per row in table order), `ModelView::route_automation` (edited entry where the view holds one),
  `route_gain_row`, `route_row`, and `continued_route_rows` (the Carry paragraph's predicate, by
  the rule of `continued_fader_lanes`; added here for the render tranche, not yet read).
- **Owner rule (R17-1).** `OwnerEdits::set` on route row 1 clones the row 1 entry with the route.
  `route::resolve` reads an automated row 1 as the entry's `offset.left_db`; `registry::write` on
  route row 1 writes both offset fields through `route::write_gain_offset` when an entry drives it,
  else the static gain. The list sets `automated` on each of rows 1-6 an entry drives; row 1 lists
  the offset, rows 2-6 their static values (hand mute; matrix fallback, automation wins).
- **Mask.** `mask_fader_offsets` copies the offset of every entry on a row that takes one (fader and
  send gain). The static `gain_db` of a send whose row 1 is driven stays as `into` holds it
  (fallback = structure, as the fader's #1650 departure 2); mute and matrix stay masked values.
- No row class or table changed: no SDK regeneration.

Gates (`crates/controls/tests/automation.rs`): `the_send_gain_takes_one_shared_offset` (gate 8),
`send_rows_render_and_enumerate_in_row_order`, `a_hand_move_on_an_automated_send_gain_writes_its_offset`
(R17-6 write, list flag, in-place path equal to the whole `set`), and
`the_mask_copies_a_send_offset_and_keeps_its_fallback`. `cargo test --locked -p controls`,
`-p host-core --features test-support`, `-p control-plane --features test-support` and
`-p builtins-compiler`: all green.

For T2: with this change a session whose only rendered entries are route entries enters
`prepare_fader_automation` (it builds an empty program today, tests green); a send automation edit
is now step 3 `Automation` (rebuild) in the classifier; a hand move on an automated send gain now
diffs as a route row 1 change whose value is the offset, and an automated matrix row's hand edit is a
static-value change the classifier does not yet commit `model_only` (not checked here): the route lowering (offset record, matrix `model_only`) is
host-core's work.

## Decision record: T2 (batch 19)

Graph types only; no render behaviour changes and no six-word route code, byte or bit moves.

- **Flags word.** `RouteFollowFlags` gains `automated_gain` (bit 6), `automated_mute` (bit 7, the
  route's own row 2) and `automated_matrix[ll, lr, rl, rr]` (bits 8-11), plan-fixed like bits 4-5;
  `DEFINED = (1 << 12) - 1`. `from_word` and `FollowRouteControlRecord::{new, from_words}` accept and
  round-trip them; bits 0-5 are unchanged. `route_rows()` gives bits 6-11 in row order. When bit 6 is
  set, word 0 is the hand offset in dB (the record checks only finiteness, so it fits). `gate` is
  unchanged (the route mute curve's OR is T3's composition).
- **Source.** `GraphFollowRouteSource::new(program, cells, route_cells, arrival, jumps, gain_law,
  flags)`: `route_cells: [Option<u32>; GRAPH_ROUTE_ROWS]` (rows 1-6, `GRAPH_ROUTE_ROWS = 6`). It
  accepts a route with no source mute cell when a route row cell is present, and refuses a route cell
  outside the program and `flags.route_rows()` that disagree with `route_cells`. Host-core passes
  `[None; 6]` and flags bits 6-11 clear until T3 gives the plan route cells.
- **Jump rule.** `GraphMuteJumpLength` is replaced by `GraphRouteJumpRule = fn(GraphJumpLengths) ->
  GraphRouteJumpLengths { mute, gain, matrix }`, with `GraphRouteCursorRow { SourceMute, Gain, Mute,
  Matrix }` and `GraphRouteJumpLengths::for_row`. Host-core's `follow_mute_jump` became
  `follow_route_jumps` (`LiveRamps::for_row` of `Mute`, `SendGain`, `SendMatrix`, one realtime-policy
  region as before). Departure: rather than a second fn pointer beside the mute one, the route op's
  two reads (`FollowCursors::block`, `finish_six_word_carry`) now take
  `for_row(SourceMute)` of the one rule, the same value as before; the field is renamed `jumps`.
- **dB law.** `graph_compiler::route_gain_law(curve_db, offset_db) = db_to_gain_f32(clamp(f64(curve)
  + f64(offset), -144, 24) as f32)` (bounds `session::ROUTE_GAIN_DB_{MINIMUM,MAXIMUM}`), in its own
  realtime-policy region in `ids.rs`, re-exported; `route_transform` calls it with offset `-0.0` (the
  exact `f64` additive identity, so the static gain is `db_to_gain_f32(gain_db)` bit for bit). Graph
  holds it as `GraphRouteGainLaw`; `check-graph-policy.sh` is unchanged and passes.
- **Unused until T3** (`#[expect(dead_code)]`): `GraphFollowRouteSource::route_cells` and
  `GraphFollowRouteSource::gain_law`. The pub types (`GraphRouteCursorRow`, `GraphRouteJumpLengths::for_row`
  for the gain and matrix rows, `RouteFollowFlags::route_rows`) need no expect.

Tests: `graph` `follow_route_tests::{the_flags_word_round_trips_its_twelve_bits_and_refuses_every_other_bit,
the_record_round_trips_its_seven_words_and_refuses_out_of_bounds_words,
the_source_refuses_no_lane_a_missing_cell_and_disagreeing_flags, each_cursor_row_takes_its_own_jump_length}`;
`graph-compiler` `ids::gain_law_tests::{a_zero_offset_gives_the_static_gain_bits,
the_law_sums_in_f64_and_clamps_to_the_route_domain}` (flat `v` with offset `+0.0` and `-0.0` gives
`route_values(v)`'s gain bits and `db_to_gain_f32(v)`'s, over the route domain, both zeros included).

Sizes (x86-64, measured): `RouteFollowFlags` 6 -> 12 bytes, `FollowRouteControlRecord` 32 -> 36,
`GraphFollowRouteSource` 48 -> 112, `LIVE_FOLLOW_ROUTE_OWNER_BYTES` (`size_of::<LiveFollowRoute>()`,
the bind charge) 352 -> 360. Charges read `size_of`, so no pinned number moved. Realtime policy: 170
marked regions in 34 files (168 before: `GraphRouteJumpLengths::for_row` and `route_gain_law` added;
host-core's jump-rule region replaced in place).

## Decision record: K1 (batch 19)

`crates/automation` only (R19-2 = (b), K1). No render consumer yet: T3 (the route op) and #1655
(batch 19b) bind it.

- **Shipped.** New `crates/automation/src/compose.rs`: `Composed<T>` (one output), `ComposeCursor`
  (one curve: its cell, `CursorKind::{Value, Gate}`, its own `CellState`, its block jump length,
  its heard value), `Heard` (the values a composition reads), `ComposedEvent<T>`, `Composable`
  (`same_bits`; `f32`, `bool`, arrays, pairs) and `ComposeError`. The cursor table is a boxed
  slice allocated at bind (`Composed::new`, `try_reserve_exact`, `Composed::bytes(K) = K * 104` on
  x86-64); no fixed array, no cap on `K`. The walk is one realtime-policy region: no index or
  unwrap paths, bounded by `Composed::max_events` (the sum of the cursors' bounds; each step merges
  at least one cursor event).
- **How the walk works.** Each cursor's events are `CellEvents`, unchanged, pulled one at a time
  over a window of the block (`NodeSpan::window`, new `pub(crate)`) from the offset after its last
  event; with the state as its only memory this is the cell's uninterrupted stream (gate 1 holds
  it). At each offset the composition (a caller callback over the heard values) gives `T`, and
  the output's own `CellState` decides with the cell's rules, now `pub(crate)` in `events.rs` and
  called by `CellEvents` itself (`unchanged_with`, `lagging`, `hold_event`, `commit_ramp`,
  `clear_for_landing`, `free_after`, `target_toward`): change only on the target's bits, a jump
  also when the ramp in flight lags, a held-only offset emits a held event by the held-event rule,
  a seek landing always emits. The G5 corpus digests are unchanged (`wasm-gates`
  `g5_native_corpus`: 7 passed).
- **R19-4.** One offset's events are one retarget over the longest ramp among them (Set > Jump >
  Grid for the kind).
- **R19-3, and the owed landing: K1 owns it, through the cursors' own `CellState` deferral; the
  fader's `HeldRampObligation` is not copied and stays the fader's.** A gate cursor's jump that
  retargets the output, and a caller column jump (`Composed::retarget(target, start, n)`: a
  record's ramp, a gate change of the cell), defer every value cursor's jump to the column ramp's
  first free sample (`CellState::defer_jump`, #1711 D1). While it is pending, the value cursor's
  grid is held (#1711's rule): the heard value and the output do not move. At the free sample each
  value cursor whose curve moved fires the deferred jump over its own jump length to the curve at
  the jump's completion (change only, or always with a held event outstanding), and coincident
  landings take the longest (R19-4): "the value row's own J, the longer one if both are owed". The
  owed bit is the cursor's held flag. A gate jump is never held; a value cursor's own jumps hold
  its own grid by the cell's rule (a late grid restarts it, as one cell's does); a seek landing is
  not a column jump. A cursor that pulled past the offset of a column jump is returned to the
  state its pull started from and replayed up to that offset (it gave no event there), then
  deferred and pulled again, so the deferral acts exactly at the offset.
- **Departure: value jumps are not column jumps.** R19-3 lists a gate change, a record ramp and a
  Set. A gain or matrix jump (a discontinuity, a landing) does not hold the other value cursors: a
  landing that held the others would make two riding curves land in turn, each over its own `J`,
  for ever. During a value jump the column is free (R19-3: "a free column retargets at once").
- **Departure: no caller `hold_until` on the output.** R19-3 makes every caller ramp a column jump
  (`retarget`). The one-cell hold semantics is used only by gate 1's corpus case (the test sets the
  output's and the cursor's states directly).
- **API for T3 and #1655.** `seed`/`seed_target`, `compose_heard`, `compose_toward(program, start,
  n)` (each curve toward a ramp's completion: a record's target, #1649 D3), `retarget`,
  `defer_jump(at)`/`clear_deferred_jump()` (value cursors, R19-5 at a Reposition),
  `cursor_mut`/`cursors_mut` (`set_jump` per block, `state_mut` for a verbatim carry or a
  reposition, `set_heard`), `copy_from` (a preallocated probe twin), `max_events`.

Gates (`cargo test --locked -p automation`, all green):
`compose::tests::one_cursor_gives_its_cells_events_over_the_corpus` (K = 1 equals `CellEvents`
over the six G5 cases, holds and both seek modes),
`one_cursor_gives_its_cells_events_over_random_tables` (400 seeded draws: all shapes, gaps, both
completions, `no_restart`, fixed and block jumps, seeks, reported ramps, deferrals),
`flat_curves_leave_the_moving_curves_events` (K = 3, two flat, 150 draws, every place and kind),
`a_grid_event_inside_a_gate_jump_is_held_and_lands_at_its_end` (unmute J 480 at 1,000: no
retarget in between, landing at 1,480 over 256 to the curve at 1,735, grid again from 1,728),
`coincident_jumps_take_the_longest` (480/960 both ways: one Jump over 960),
`a_deferral_holds_the_grid_until_it_fires`, `a_caller_retarget_holds_the_value_grid_until_its_end`,
and `tests/allocation.rs::the_merged_walk_allocates_nothing` (10,000 blocks, seeks, retargets,
deferrals: 0 allocations, 0 frees). Realtime policy: 171 marked regions in 35 files (170 before).

## Decision record: T3 (batch 19)

`crates/graph` (the route op, its cursors, the route carry, the source and the charge) and
`crates/automation/src/compose.rs` (K1, two additions). No six-word route code, byte or bit moved.
Nothing under `crates/engine/src/realtime/`; no `crates/lane` kernel change.

- **Route op on K1.** `LiveFollowRoute` holds one `RouteColumn` per source column that hears a
  curve: a `Composed<([f32; 2], bool)>` walk over that column's cursors (source mute of its lane
  and the route's own mute as `Gate`, gain and the column's two matrix entries as `Value`, each
  with its row's jump length per block, `GraphRouteJumpLengths::for_row`), a probe twin for the
  block entry's activity probe, its roles and its carry placement. The two columns' events are
  merged in offset order (`walk_columns`, over K1's new pull form `start`/`next_event`). One
  composition, `column_target`: `law(curve, word 0)` when the plan automates the gain, each
  automated matrix entry from its curve, gate `mute = static mute || route mute curve` (D5) and the
  column's follow term, `gated_route_coefficients`, the column's entries. A column's entries do not
  depend on the other column's gate, so the columns compose apart; the route's silence is both
  columns silent. Retarget per column over the event's own ramp (`ColumnRamps::retarget_column`,
  replacing `retarget`; `route_mix_column_ramps_block` unchanged). Every #1652 graph test is green
  unchanged on the new op.
- **Records.** A record that changes a value word takes each automated value toward the record
  ramp's completion (`Composed::compose_toward`; gates as heard) and a gate-only record composes
  with the heard values; a column the record moves is a column jump (`Composed::retarget`: its
  value cursors defer to the ramp's end, R19-3). A predecessor's last read at a swap composes with
  the heard values and defers nothing (it is retired).
- **Carry (D3).** `GraphFollowRouteSource::with_continued_rows([bool; 6])` (new, default none;
  T4 sets it from `continued_route_rows`). Seven -> seven: per column and role the predecessor's
  cursor state and heard value are copied; a continued row is kept verbatim, any other cursor is
  repositioned at the successor's first node time (its target stays the one the column heard; a
  row the predecessor lacked starts with no target). At the first block: equal column bits leave
  the ramp; a moved gate retargets the column at once over the mute length (a column jump); moved
  values alone defer the placed value cursors to the column ramp's first free sample, at once when
  settled (R19-5, #1711 D1: exact, no flag). Seven -> six keeps `heard_zeroed` over the source
  mute cursors.
- **Coordinator question (B lands at an old free time inside A's jump): it can happen, fixed.**
  K1 now extends a value cursor's pending deferral to the end of a column ramp that another
  cursor's retarget replaced and outlasts (never shortens; the cursor whose own event it is keeps
  its cell's rule, so K = 1 stays exactly `CellEvents`: all K1 gates green). Gate:
  `a_held_value_waits_for_the_end_of_a_jump_that_replaced_the_column_ramp` (red without the
  extension: block 18). Compatible with the U1 fold to `next_in_run` (a cursor's own discontinuity
  jump clears its own deferral). `events.rs` is not edited by T3.
- **K1 additions.** Pull form (`start`, `next_event`; `walk` is built on them); `compose_toward`
  reads `Gate` cursors as heard and repositions a copy of each value cursor's state at `start`.
- **Charge.** The producer carries `cursor_bytes` (per column with a curve, two tables of
  `Composed::bytes(K)`), added to `owner_bytes` and to the largest allocation.
- **Departure: no graph-level allocation gate.** Linking `bench_support`'s audited allocator into
  graph's lib test binary makes an existing test abort
  (`runtime::tests::a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit`,
  green without the allocator). The walk's gate is K1's
  (`automation/tests/allocation.rs::the_merged_walk_allocates_nothing`); host-core's
  `route_mute.rs` render gates run the new op for mute-only routes; the send-row render gate is T4.

Gates (`cargo test --locked -p graph --features test-support`, 130 lib tests green), module
`runtime::send_automation_tests`: `a_flat_send_ride_mixes_the_static_route_bits` (offsets `+0.0`
and `-0.0`, 40 blocks bit-equal), `a_gain_ride_across_an_unmute_fade_ends_the_fade_at_its_length_and_lands_at_its_end`,
`a_ride_on_the_left_column_keeps_moving_while_the_right_column_fades`,
`coincident_send_events_take_the_longest_jump`, `a_hand_offset_record_aims_at_the_curve_at_its_end_and_holds_the_grid`,
`a_held_value_waits_for_the_end_of_a_jump_that_replaced_the_column_ramp`,
`a_continued_send_ride_carries_its_cursor_verbatim` (red with no row continued),
`a_moved_send_table_jumps_over_its_own_length_at_the_first_sample`,
`a_hand_move_in_flight_at_a_swap_keeps_its_length_and_the_moved_table_lands_after_it`. The route
op mixes at `FrameLane` on every backend: no bank width applies. host-core `--features
test-support`: all green. Sizes (x86-64): `LIVE_FOLLOW_ROUTE_OWNER_BYTES` 360 -> 944,
`GraphFollowRouteSource` 112 -> 120, `GraphFollowRouteControlProducer` 104, `ComposeCursor` 104.
Realtime policy: 171 regions in 35 files.

## Decision record: T4 (batch 19)

Preparation, records and the classifier; no render code (`crates/graph/src/runtime.rs` and
`crates/automation` untouched). What shipped:

- **Program.** `prepare_fader_automation` appends the route cells after every mute cell, in
  `model.routes` order (routes into a submix only) and row order: rows 1 and 3-6 with
  `FADER_CELL_RULE`, row 2 with `MUTE_CELL_RULE`. A session without route automation builds the
  same program. New `PreparedBuiltinsSession::route_cells(route_id) -> Option<(Arc<Program>,
  [Option<u32>; 6])>`. A refusal at a route cell is `host.automation.route` (each table now
  carries its own code).
- **Predicate.** `following_route_lanes` returns `Option<graph::RouteAutomatedSet>` (new graph
  type: flags bits 4-5 `lanes`, bits 6-11 `rows`): `Some` when the route goes into a submix and
  its source mute or any route row 1-6 is automated. `follows_automated_mute` follows it, so the
  #1713 flag and the `LocalDiff` follower scan widen with no code of their own.
- **Attach and seed.** The attach reads the source's mute cells and the route's cells (one
  program, checked against the set), builds the flags with the new `follow_route_flags` (bits
  6-11 from the set; both "#1654 T2" literals gone) and seeds the row 1 hand offset through the
  new `GraphFollowRouteSource::with_gain_offset` (`gain_offset()` reads it; `+0.0` by default).
- **Records.** `follow_route_record(route, terms, automated, gain_offset, length)` writes the
  offset in word 0 when bit 6 is set. `follow_route_record_between` takes the offsets
  `[before, after]` (`route_gain_offset`, the view's row 1 entry, so the in-place path reads the
  edited entry, R17-1) and compares only the words render hears: an automated matrix word is
  masked, so a hand edit of an automated matrix row gives no record (`model_only`, D4). The ramp
  of an offset change is `SendGain` (the edit's own length when given).
- **Classifier.** A send automation edit is step 3 (`LiveRebuild::Automation`); a hand gain move
  on an automated send is one seven-word record over `SendGain`; an un-automated row of an
  automated send (gain, matrix, mute) writes the seven-word cell (D6), never the folded form.
- **Join.** `RouteJoin::continued_rows` (`controls::continued_route_rows`) reaches the move as the
  new `GraphRouteMove::continued_rows: [bool; 6]`, for T3's carry.
- **Commit.** `GraphFollowRouteControlProducer::automated()` returns the whole bound set;
  `RouteFollowFlags::automated_set()`; new `host_core::follow_route_producer_for`, which the
  pre-write check in `control.rs` now calls (whole set before any write, D9).

Departures: the pre-write refusal is gated at the helper (`follow_route_producer_for`) and the
producer's write count, not through a commit: a valid session cannot produce a disagreeing set.
`docs/SESSION_SCHEMA_V1.md` is not touched here (T5 or the docs tranche).

Gates: `builtins-compiler` `the_route_cells_follow_the_mute_cells_in_the_one_program`; `graph`
`follow_route_tests::the_automated_set_is_bits_4_to_11_and_only_an_automated_gain_takes_a_seed_offset`;
host-core `tests/live_delta.rs` `a_send_automation_edit_is_a_rebuild`,
`a_sends_hand_edits_lower_its_offset_and_its_unautomated_rows_only` (offset word, flags, lengths,
`model_only` matrix, D6, fast path equal), `a_route_row_send_binds_its_set_and_the_commit_refuses_another`,
and the join's `continued_rows` pass-through in `prepare.rs`'s route-carry unit test;
control-plane `tests/send_automation_realtime.rs`
`a_send_with_stored_row_automation_renders_allocation_free` (live, model_only, live, live; zero
allocations); capi `a_hand_move_on_an_automated_send_gain_commits_alike_in_place_and_full` and
`removing_a_sends_only_route_entry_reports_a_fallback` (#1713 flag on a route-row-only send);
host-web `a_hand_move_on_an_automated_send_gain_is_an_offset_alike_on_both_hosts` (R17-1, both
hosts). Realtime policy 170 regions in 34 files (unchanged).

Sizes (x86-64, measured): `GraphFollowRouteSource` 112 (unchanged, the offset fills padding),
`GraphRouteMove` 12 -> 16 (control side only; the render step `RouteCarryStep` is unchanged),
`GraphFollowRouteControlProducer` 88, `RouteAutomatedSet` 8, `LIVE_FOLLOW_ROUTE_OWNER_BYTES` 360
(unchanged).

For T3/T5: the route op must read `source.gain_offset()` as the seeded `gain` word when bit 6 is
set (the bound transform's `gain` is the compiler's linear static gain) and
`GraphRouteMove::continued_rows` for the carry. A route-row-only send now binds the seven-word
lane with no source cells, and renders with the old (static) route op until T3 lands.

## Decision record: T5 (batch 19)

Integration of T3 and T4, the gates and the doc. Nothing under `crates/engine/src/realtime/`; no
`crates/lane` change; no row class or table changed (no SDK regeneration).

- **One continued-rows path.** Render reads `GraphFollowRouteSource::continued` (T3), so that path
  stays: host-core's attach now sets `with_continued_rows(controls::continued_route_rows(
  base.committed, model, route))` for a successor (the route joins are computed before the attach).
  T4's `GraphRouteMove::continued_rows` and `RouteJoin::continued_rows` are deleted;
  `GraphRouteMove` 16 -> 12 bytes (x86-64), `RouteCarryStep` unchanged.
- **Gain word seed (T4 note).** `LiveFollowRoute::new` replaces the bound transform's `gain` (the
  compiler's linear static gain, a fallback) with `source.gain_offset()` when the flags automate
  the gain. Before it, a flat ride at -6 dB rendered `law(-6, 1.0)` instead of the static bits.
- **T3 edge: it was a heard difference, fixed.** A row the predecessor did not automate
  (`Placement::New`) kept its value cursor's bind seed (the curve at node time 0) through the
  carried first block's cell, so a gate-only join record (for example a hand unmute in the rebuild
  that adds a gain ride) aimed the column at the ride's value at 0 dB, no plan's value, and then
  jumped to the curve. Now `FollowCursors::place_new_values` places such value cursors at the first
  block's node time before the cell applies, and `place_carried` does not place them again (that
  dropped the record's column-jump deferral, so the ride's grid cut into the unmute, R19-3).
  Repositioned and continued cursors keep the value the column heard, as T3 specified.
- **R19-5 holds: exact, no flag.** Both moves (hand offset, D6 `matrix_lr`) with their own 4,000
  samples run to their end across a table-moving rebuild, `EXACT`, no transition fallback, on both
  hosts. While writing the gate a first failure was the test's: an `UpsertAutomation` built from the
  pre-move document re-authors the entry with offset 0, which is a real offset change (the join
  writes a record); the gate re-authors from the committed model.

Gates (all green), with red-without-the-feature evidence (once each):
- Gate 1: host-core `route_mute.rs` `a_flat_send_ride_renders_the_static_sends_bits` (Simd8 when
  avx2, Simd4, scalar oracle; three live forms; offsets `+0.0`/`-0.0`; flat mute and `rr`; zero
  render allocations); capi `a_flat_send_ride_renders_the_static_sends_bits_on_the_c_abi`; host-web
  `a_flat_send_ride_renders_the_static_sends_bits_on_both_hosts`. Red without the seed fix: sample 0
  -0.13376695 != -0.11921991 (host-core), block 0 (capi).
- Gate 2: host-core `a_send_ride_is_heard_at_its_taps_time` (input and post-fader taps behind the
  486-sample limiter vs the twin with no latency, shifted by `L`, bit for bit; quanta 128 and 100;
  three widths). Red with the route arrival forced to 0: post-fader sample 0 0.1787 != 0.2498.
- Gate 3: capi `send_row_edits_rebuild_or_commit_as_the_rows_rule_says_on_the_c_abi` ((a) rebuild
  `EXACT`, fresh seeked plan after the jump; (b) `+3` live then the offset plan's bits; (c) automated
  `matrix_rr` `model_only`, no bit; (d) un-automated `matrix_lr` live then a fresh plan's bits;
  (e) gate 7 on the C ABI); host-web `send_row_edits_rebuild_or_commit_as_the_rows_rule_says_on_both_hosts`.
  Red without the seed fix: (b) differs from the fresh plan after the ramp. The in-place/full
  equivalence is T4's (`a_hand_move_on_an_automated_send_gain_*`).
- Gate 4: capi `a_hand_send_mute_ors_with_its_curve_on_the_c_abi`; host-web
  `a_hand_send_mute_ors_with_its_curve_on_both_hosts`. Red with the curve replacing the static mute
  (not ORed): "the hand mute shuts the send while the curve is open".
- Gate 5: capi `an_output_route_entry_is_refused_on_the_c_abi`; host-web
  `an_output_route_entry_is_refused_alike_on_both_hosts` (one diagnostic,
  `control.automation.target`).
- Gate 6: zero render allocations in host-core gates 1, 2 and 7 (`bench_support::alloc` thread
  counters, the swap block included); control-plane `send_automation_realtime.rs` (T4).
- Gate 7: host-core `a_running_send_ride_across_an_unrelated_rebuild_is_the_never_swapped_twin`
  (three widths); capi gate 3 (e); host-web
  `send_rides_and_live_send_moves_cross_rebuilds_exactly_on_both_hosts`. Red without the attach's
  `with_continued_rows`: host-core sample 768 -0.053634815 != -0.053632714; host-web block 6; capi (e).
- Gate 8: the controls gates (T1, `the_send_gain_takes_one_shared_offset`), reused.
- R19-5: capi `a_live_send_move_in_flight_at_a_reposition_keeps_its_length_on_the_c_abi`; host-web
  (in the gate 7 test); graph `a_hand_move_in_flight_at_a_swap_of_a_riding_table_is_the_twin_until_its_end`.
- T3 edge: graph `a_new_send_row_at_a_swap_composes_the_join_record_with_its_curve_at_the_first_block`
  (red before the fix: target bits of `law(0 dB)`, then red with only the pre-placement: the grid
  retargeted inside the unmute).

Departures: gate 2 runs in host-core only (three widths); the hosts share that route op and its
arrival, and neither host fixture carries a latent insert. Host-web's gate 7 uses an added muted
(inactive) send as the unrelated rebuild, because a flat fader ride on `t3` is not bit-neutral at a
swap there (outside this issue; not investigated). `docs/SESSION_SCHEMA_V1.md`: send rows render,
the send gain takes the shared offset, routes into the output stay prepared.

Realtime policy: 171 marked regions in 35 files (unchanged). `scripts/check-graph-policy.sh`: PASS.
