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
