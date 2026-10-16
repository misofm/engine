# Store a hand move on an automated fader or trim as a dB offset

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), owner answer OQ1 = C (2026-10-14, README "Owner answers"), under decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`, C2, C3, C5, C6 and C10). A draft for root to
file. Written by the controls coordinator on 2026-10-14. Code anchors verified on `12d4a4f9d`
(batch 9 head). Batch R1, in one push with #1646, #1647, #1648 and #1649.

## Product outcome

A producer automates a vocal fader. A fan, or the producer in the browser mixer, moves the same
fader by hand. On every host the move is heard on top of the ride: the lane plays the curve plus a
stored dB offset, as the trim modes of Logic Pro, Pro Tools and Cubase do (README [S3], [S4],
[S6]). The move is one `SET_CONTROL` on the row the move addresses today (`fader_db`, builtins
control 5), it is live (one cell write, no rebuild), and the offset is saved in the session: in its
canonical JSON, on the wire and in the control list. A later re-authoring of the curve
(`SetAutomationSegments`) keeps the offset. The same rule holds for the input trim (`trim_db`,
builtins control 2) from the slice that renders trim automation (#1658). The owner's other two
rules are not in this slice: a hand mute ORs with the automated mute (#1651, amended), and on
every other row automation wins (#1648 D3).

## Context

- **The owner answer.** README "Owner answers (2026-10-14)": OQ1 = C. A hand move of an automated
  fader dB or input trim dB adds a stored dB offset to the curve; mute ORs with the automated mute;
  on other rows automation wins.
- **The stored automation.** `Automation { id, target: ControlAddress, segments }`
  (`crates/session/src/model.rs:969-977`). JSON keys `id`, `target`, `segments`
  (`crates/session/src/visit.rs:143`, walk `:299`; parse `crates/session/src/parse.rs:1569`); wire
  message `automation` with fields 1-3 (`crates/protocol/src/schema.rs:1275-1287`). The edits
  `UpsertAutomation` (`0x0600`), `RemoveAutomation` (`0x0601`), `SetAutomationTarget` (`0x0602`) and
  `SetAutomationSegments` (`0x0603`) (`crates/protocol/src/model.rs:81-88`, `:316-327`).
- **One row rule.** `controls::automation_diagnostics` (`crates/controls/src/automation.rs:72`)
  judges every automation against the row its target names (#1566 D6). Preparation
  (`crates/host-core/src/prepare.rs:1890`), the classifier (`crates/host-core/src/live_delta.rs:372-376`)
  and `session-validator` (`tools/session-validator/src/lib.rs:396`) call it.
- **The row binds the model.** Builtin rows bind to their model field through one accessor arm per
  builtin ID in `controls` (decision 17 C3; `crates/controls/src/builtin_rows.rs`). `fader_db` and
  `trim_db` are per-lane, live, domain `[-144, 24]` dB (`crates/builtins/src/lib.rs:509-530`,
  `:591-607`). `controls::mask_values` and `controls::changed_controls` read every row through its
  accessor (`crates/controls/src/mask.rs`, `diff.rs`), and the classifier lowers a changed row by its
  family (`crates/host-core/src/live_delta.rs:240-300`).
- **The control list.** A control row carries `automatable` (field 31), `active` (33) and `value`
  (34) (`crates/protocol/src/schema.rs:358-433`). No field says whether stored automation drives the
  row.
- **The fader composition.** #1646 D1 gives every automated fader lane an offsets cell that
  holds `S`, the `f64` sum of the reaching VCA offsets, and composes each event as
  `checked_fader_gain(vca_compose_db(v, S))`; #1649 writes the cell on a VCA edit. A lane no VCA
  reaches holds `S = +0.0`.
- **The fallback rule of A2.** #1648 D3 commits a static edit of an automated cell as
  `model_only`: the fallback value, stored and not heard. Under OQ1 = C that rule stays for every
  row except the fader and trim dB rows (this slice) and the mute row (#1651).

## Decisions frozen for this slice

- **D1. The stored field.** `Automation` gains `offset: AutomationOffset { left_db: f32, right_db:
  f32 }`, required in every entry (the V1 grammar has no optional key; decision 17, "How to add a
  control", step 2.1):
  - JSON: `"offset": { "left_db": <f32>, "right_db": <f32> }`, key ID 4 in the automation walk,
    appended after `segments`, with the record's two keys IDs 1 and 2; the canonical writer, the
    parser, `keys::automation`, the wire message `automation` (field 4, a required nested message
    `automation_offset` with two required `F32` fields 1 and 2), its encode and decode, the
    schema-hash re-pin, and #1568's equal-key test if it has landed.
  - The SDK's key tables (generated after #1569) and its session builder (`automation()` takes an optional
    `offset`, default both lanes `0`). Every checked-in document that carries automation gains
    `"offset": {"left_db": 0.0, "right_db": 0.0}`, rewritten by the engine's canonical writer.
  - The offset of a lane belongs to the entry that drives that lane. An entry whose `lane` is `left`
    has `right_db = +0.0`, and the reverse; a `both` entry holds one offset per lane. So a
    lane-granular hand move on a `both` entry needs no entry split.
  - `RemoveAutomation` removes the offset with the entry. `SetAutomationSegments` keeps it.
    `SetAutomationTarget` keeps it, and D2 judges it against the new row.
- **D2. The row rule.** `controls::automation_diagnostics` gains two checks, with the session paths
  `$.automation[id=<id>].offset.left_db` and `.right_db`:
  - `control.automation.offset` (new, one code of the control set): a non-zero offset on a row that
    takes none. Only `fader_db` and `trim_db` take an offset (one `takes_offset` rule in `controls`,
    by builtin ID); a lane the entry does not drive must hold `+0.0`.
  - `control.domain`: an offset that is not finite or lies outside `[-144, 24]` dB, the VCA offset
    domain (`crates/session/src/validate.rs`, VCA offsets).
  The session crate checks nothing new: it cannot see rows.
- **D3. The write is the row.** On a lane whose fader (or, from #1658, trim) automation renders
  (#1648 D1's `automation_row_renders`), the row's accessor reads and writes that lane's offset
  in the entry that drives the lane, not the static field. So:
  - `SET_CONTROL { address: fader_db of (strip, lane), value: v }` sets that lane's offset to `v`.
    `controls::check` applies the row's own domain, `[-144, 24]`, which is the offset domain of D2.
    A `both` write sets both lanes' offsets, as decision 17's lane rule says.
  - `CONTROL_STATE_GET` and the control list report the offset as the row's `value`, so a read-back
    equals the write, and the default `0` is "no trim", as a DAW fader in trim mode shows it.
  - The static fader value of an automated lane (its fallback) is not reachable by `SET_CONTROL`
    while the automation exists. A host changes it in one transaction: `RemoveAutomation`, the
    `SET_CONTROL`, then `UpsertAutomation`; edits apply in order to the running model (decision 17
    C5). A document replacement also sets it.
  - On a lane whose automation does not render yet (trim before #1658), the accessor keeps the
    static field and the stored offset is inert, as every not-yet-rendered row is (A10).
- **D4. The list says so.** The control row gains `automated BOOL` (field 35 of `control_entry`,
  optional on the wire, written on every control row): true when a rendered stored automation drives
  the row's lane. It is a descriptor field in decision 17's sense, so an automation edit that changes
  it advances `structure_epoch` (C4). The SDK's generated control type gains it (#1550).
- **D5. The classifier.** Because the row's accessor reads the offset, `controls::mask_values`
  copies the offset with every other control value, and step 1 of `classify_live_delta` does not
  see an offset change as a change of the automation table. Step 3 sees a changed `fader_db` value
  on an automated lane, and the strip-level lowering writes the lane's offsets cell (#1649 D4)
  with the new `S` and the ramp `LiveRamps::resolve(Fader, the edit's ramp)`, instead of a `FaderDb`
  record. The path is `live`. An `UpsertAutomation` whose only change is the offset is classified the
  same way. A change of segments or target stays a rebuild (#1648 D1).
- **D6. Render.** The hand offset joins #1646's sum: `S = offset + Σ reaching VCA offsets`, in
  `f64`, the lane's own offset first, then the VCA offsets in ascending VCA ID. Each fader event is
  `checked_fader_gain(vca_compose_db(curve, S))`, unchanged; a cell change jumps at the next block
  entry over the cell's ramp word (#1649 D3). An offset of `±0.0` and no VCA give the gain bits of
  no offset (`f64(v) + (±0.0)` differs from `v` only in the sign of a zero, and `db_gain(±0) = 1`).
  All per-sample work stays in the existing unfused fader kernels; the offset adds one `f64`
  addition per event. Trim's render half (its own offsets cell, the same composition with
  `checked_trim_gain`) is #1658's, amended.
- **D7. Mute and other rows.** Not in this slice. #1651 (amended) ORs the static mute into the
  automated mute. On every other row #1648 D3's rule stays: a static edit of an automated cell is
  stored, not heard, and commits `model_only`.
- **D8. The acked-batch question: can an ack ever precede a drop? No.** The write is checked whole
  before the first cell write (decision 17 C6), a cell write cannot fail, and the committed model
  holds the offset.

## Deliverables

1. D1 in `crates/session`, `crates/protocol`, the SDK and every document that carries automation.
2. D2-D4 in `crates/controls` and the control-list codec.
3. D5 in `crates/host-core/src/live_delta.rs`; D6 in the fader bank's event composition and the
   offsets-cell write of #1649.
4. `docs/SESSION_SCHEMA_V1.md` (the automation section: the offset, its rule and the write rule) and
   `docs/CONTROL_PROTOCOL_REGISTRY.md` (field 35).

## Authorized paths

- `crates/session/src/{model.rs,parse.rs,visit.rs,canonical.rs,lib.rs}`, `crates/session/tests/**`
- `crates/protocol/src/{schema.rs,session_wire.rs,message_wire.rs,control_rows.rs}` and their tests
- `crates/controls/**`
- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`
- `crates/builtins-compiler/src/lib.rs` (the fader event composition only)
- `crates/conformance/src/protocol_corpus.rs` (the re-pin), `fixtures/session/**`,
  `fixtures/session-canonical/**`, `.claude/skills/author-session/{SKILL.md,worked-session.json}`
- `sdk/src/core/session.ts`, `sdk/src/internal/session-json.ts`, `sdk/src/generated/*`
  (regenerated), `sdk/test/**`
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`
- `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`, `docs/CONTROL_PROTOCOL_REGISTRY.md`

## Non-goals

- The mute OR (#1651) and the trim render (#1658).
- An offset on any other row: owner answer C names only the fader and the input trim. Send gain is
  "automation wins" (#1654).
- Touch, latch or override modes (OQ1 options B and D, not chosen).
- Reading the instantaneous automated value. A moving value is observation (decision 17 C9), not a
  control value; `effective_fader_db` keeps its static meaning.

## Hazards

- **One push with #1646, #1647, #1648 and #1649** (README "Must-land-together groups", amended). Before
  this slice a hand fader move on an automated lane is heard as a static fader (nothing renders the
  curve); after #1646-#1648 without this slice it would be `model_only`; with this slice it is an offset.
  `main` must never hold the middle state.
- **Document bytes move.** Every document with automation gains the `offset` key. Each pinned digest
  that moves is re-pinned with the reason "#1650 adds the automation offset".
- **The binding changes with the table.** A transaction that adds or removes a lane's automation
  also changes what the row's accessor reads. #1648 D1 makes every such transaction a rebuild,
  so the classifier never diffs a row whose binding changed between `current` and `next`; gate 4
  holds the removal case.
- **The `-0.0` offset.** An authored `-0.0` is a valid zero offset (D6 shows the gain bits stay);
  D2's "lane not driven" rule asks for `+0.0` bits, so the canonical form has one spelling there.

## Objective gates

Commands: `cargo test --locked -p session -p protocol -p controls`, `cargo test --locked -p host-core
--features test-support`, `cargo test --locked -p capi`, `cargo test --locked -p conformance`,
`cargo test --locked -p host-web --features test-support --test control_parity`,
`bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
`./target/release/realtime_policy`, `bash scripts/check-cross-targets.sh`, clippy `-D warnings`,
`cargo fmt --all -- --check`.

1. **Grammar.** A document with offsets round-trips byte for byte through JSON and the wire; a
   missing `offset` key, an unknown key in it, a non-zero `right_db` on a `left` entry and a non-zero
   offset on a `pan` entry are refused at their paths with their codes.
2. **The hand move is heard** (C ABI and the browser, one test each through `control_parity.rs`'s
   harness). A track with a linear fader ride takes `SET_CONTROL fader_db = +3` on its left lane:
   path `live`, one revision, no plan swap; after the jump ramp the left lane equals, bit for bit, a
   fresh plan of a session whose ride is the same and whose left offset is `+3`; the right lane is
   unchanged.
3. **Offset 0 is no offset.** A flat ride at `-7.25` dB with offset `0` renders the bits of static
   `-7.25` (#1646 gate 1's twin), with and without a reaching VCA.
4. **Re-authoring keeps it.** `SetAutomationSegments` on the ride keeps the offset (control list
   value unchanged); `RemoveAutomation` removes it and the lane returns to its static value.
5. **The list.** The row shows `automated = true` and `value = +3` after gate 2; a list fetched
   before the automation edit is refused `control.list.stale` after it.
6. **Realtime.** The offsets-cell write and the jump allocate and free nothing on the render thread
   (`bench_support::alloc` thread counters, in `crates/control-plane/tests/`).

## Test value

- Gate 1 turns red if the field is optional, if the lane rule is missing, or if a non-level row
  accepts an offset.
- Gate 2 turns red if the move writes the fallback (option A), rebuilds, or reaches the wrong lane.
- Gate 3 turns red if the offset term changes the gain bits of a zero offset.
- Gate 4 turns red if a curve edit drops the fan's offset, the product reason for answer C.
- Gate 5 turns red if a client cannot tell that its write is an offset.

## Dependencies

- #1646, #1647, #1648 and #1649 (same push). #1648 D1's `automation_row_renders` is the predicate
  D3 reads.
- #1566 (batch 9): the address target and the row rule. *Keep the session JSON keys and the session
  wire field IDs equal* (#1568) and *Generate the SDK's session key tables from the engine's walk*
  (#1569) are open; whichever lands first, the other covers the new keys (ordering only).
- #1658 (amended) gives the trim its render half and flips D3 for trim when it adds the trim row.
- Batch R1.

## Amendment (controls coordinator, batch 10, 2026-10-14)

Root's ruling (README "Root's rulings (2026-10-14)"): a send's `gain_db` is a level control, so a
hand move of an automated send gain writes an offset, the same as the fader and the trim. The
owner's answer covers level controls, and the DAW trim modes include sends.

- **D1, a shared row.** Route row 1 `gain_db` is shared (`lane: both`). Its entry holds one offset:
  `left_db` and `right_db` are equal. A `SET_CONTROL` on it writes both fields with the one value.
- **D2.** `takes_offset` is true for builtins `fader_db` and `trim_db` and for route `gain_db` on a
  route into a submix (the only routes whose rows are automatable). `control.automation.offset`
  also refuses an entry on a shared row whose two fields differ. The domain is the route gain's,
  `[-144, 24]` dB (`ROUTE_GAIN_DB_MINIMUM`, `ROUTE_GAIN_DB_MAXIMUM`), the same as the fader's.
- **D3.** The route `gain_db` row's accessor reads and writes the offset from the slice that renders
  send automation (#1654), as the trim's does from #1658. Before #1654 the stored offset is
  inert, as D3's last bullet says for every not-yet-rendered row.
- **D5.** For an automated send gain, the classifier's route lowering writes the route's
  seven-word cell (#1654, amended), with `LiveRamps::resolve(SendGain, the edit's ramp)`. That
  lowering is #1654's; this slice adds only the field, the rule and the row binding switch.
- **D7 and Non-goals.** "Send gain is 'automation wins' (#1654)" is replaced: send gain takes
  an offset; on a send's matrix rows automation wins (#1654 D4). A VCA's `offset_db` is a level
  control too; #1655 adds it to `takes_offset` (a reading of root's ruling for root to confirm).
- **Gate 1** gains: an entry on a send `gain_db` whose two offset fields differ is refused
  `control.automation.offset`; a non-zero offset on a send matrix row is refused.

## Amendment (controls coordinator, batch 17 spec check, 2026-10-16)

Checked against `31fa33fee`. This amendment overrides the filed text and the earlier amendments
where they differ.

- **Anchors moved.** `Automation` `crates/session/src/model.rs:977-985`; the automation walk keys
  `crates/session/src/visit.rs:143`, walk `:299`; `parse_automation`
  `crates/session/src/parse.rs:1569`; wire message `automation`
  `crates/protocol/src/schema.rs:1321-1333`, encode `crates/protocol/src/session_wire.rs:1072-1080`;
  the edits `crates/protocol/src/model.rs:81-89`, `:315-327`. `automation_diagnostics`
  `crates/controls/src/automation.rs:89`; its callers `crates/host-core/src/prepare.rs:2529`,
  `crates/host-core/src/live_delta.rs:388-395` (step 2), `tools/session-validator/src/lib.rs:396`.
  Builtin rows: `read` and `write` `crates/controls/src/builtin_rows.rs:121-188`; the model write
  `crates/controls/src/registry.rs:518-560`. `trim_db` `crates/builtins/src/lib.rs:514-540`,
  `fader_db` `:596-619`. `control_entry` `crates/protocol/src/schema.rs:407-441` (31
  `automatable`, 33 `active`, 34 `value`; 35 is free); the catalog row
  `crates/host-core/src/control_provider.rs` (`automatable` `:1969`, `:2072`), the structure epoch
  `:441-453`; the SDK decode `sdk/src/core/control-list.ts:275`, `:586`. Schema hash
  `crates/conformance/src/protocol_corpus.rs:928`.
- **Landed deps.** #1566 (address target, row rule), #1568 (equal keys) and #1569 (generated SDK
  key tables, `sdk/src/generated/session-keys.ts` from `sdk/assets/miso-engine-v1-session-keys.json`)
  have landed, so D1's "whichever lands first" is moot: regenerate. #1631/#1632 (the row rule on
  activity changes; `follows_mute` and `matrix_mode` not automatable). #1612/#1613/#1620 (the fast
  and in-place `SetControl` paths). #1562b (the browser rebuild path).
- **Departure 1: the binding needs the model; the in-place path does not have it.** `builtin_rows`
  reads and writes through a `StripRef`/`StripMut`, and the in-place path (#1613,
  `crates/protocol/src/model.rs:1435-1500`) applies `SET_CONTROL` to an owners model
  (`controls::OwnerEdits`, `crates/controls/src/owner_edits.rs`) whose `automation` is empty and
  whose `OwnerSlots` (`crates/session/src/in_place.rs:30`) hold no automation entry. D3 as written
  would make the in-place path see "not automated" and write the static field, and the fast
  classifier would lower a `FaderDb` onto the automated lane: an ack of a value the curve then
  overrides. **Ruling question R17-1** below.
- **Departure 2: the static fallback is no control value on an automated lane.** Because the row
  reads the offset there, `controls::mask_values` copies the offset, and a change of the static
  field by any edit is step 1 `Structure` (a rebuild that seeds the release value, #1648). This is
  D3's intent ("not reachable by `SET_CONTROL`"); record it in `docs/SESSION_SCHEMA_V1.md`.
- **Departure 3: #1648 D1 excludes the offset** (#1648's amendment, departure 3), so an offset-only
  `UpsertAutomation` stays `live` as D5 says.
- **Departure 4: D6 is no render change.** Render composes with `S` from the cell (#1646, #1649).
  This slice changes who computes `S`: `controls::effective` adds the lane's own offset first, then
  the reaching VCA offsets (#1649's amendment, departure 3), and #1646's preparation seeds it. So
  `crates/builtins-compiler/src/lib.rs` drops out of this slice.
- **Trim.** Trim automation renders in #1658, which is not in this batch. This slice can close the
  trim part as grammar and row rule only: a trim entry carries an offset that is checked and stored
  and is inert (D3's last bullet); `SET_CONTROL trim_db` keeps writing the static field until #1658
  adds trim to `automation_row_renders`. No trim gate is needed here; #1658 owns the flip and its
  gates. The same holds for the send gain (#1654).
- **Split (recommended).** **#1650a** D1, D2, the docs and gate 1: the field, the grammar, the wire,
  the SDK, the fixtures and the row rule. It is inert, so it can land in its own push before
  #1646, which then seeds `S` from a field that exists. **#1650b** D3-D5, the list flag and gates
  2-6, in the R1 push with #1646-#1649 (the "no middle state" hazard applies to #1650b only).
- **Authorized paths, as they now must read:**
  - #1650a: `crates/session/src/{model.rs,parse.rs,visit.rs,lib.rs}`, `crates/session/tests/**`;
    `crates/protocol/src/{schema.rs,session_wire.rs,model.rs}`, `crates/protocol/src/session_wire/**`
    and their tests; `crates/controls/src/automation.rs`, `crates/controls/tests/automation.rs`;
    `crates/conformance/src/protocol_corpus.rs` (re-pin); `fixtures/session/v1/canonical.json`,
    `fixtures/session/v1/builtins-automation.json`, `fixtures/session-canonical/**`;
    `.claude/skills/author-session/{SKILL.md,worked-session.json}`;
    `sdk/assets/miso-engine-v1-session-keys.json`, `sdk/src/generated/*` (regenerated),
    `sdk/src/core/session.ts`, `sdk/src/internal/session-json.ts`, `sdk/test/**`;
    `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`.
  - #1650b: `crates/controls/src/{builtin_rows.rs,registry.rs,strip.rs,mask.rs,diff.rs,effective.rs,view.rs,owner_edits.rs,lib.rs}`
    and `crates/controls/tests/**`; `crates/session/src/in_place.rs` (if R17-1 = A);
    `crates/protocol/src/model.rs` (`prepare_in_place`), `crates/protocol/src/message_wire.rs`
    and `crates/protocol/src/schema.rs` (field 35); `crates/host-core/src/control_provider.rs` (the
    `automated` row field); `crates/host-core/src/live_delta.rs`,
    `crates/host-core/tests/live_delta.rs`; `sdk/src/core/control-list.ts`,
    `sdk/src/generated/*`, `sdk/test/**`; `crates/capi/src/runtime/live_tests.rs`,
    `hosts/host-web/tests/control_parity.rs`, `crates/control-plane/tests/` (gate 6);
    `docs/CONTROL_PROTOCOL_REGISTRY.md`.
  - No path under `crates/engine/src/realtime/`; no render file.
- **Gates' test files.** Gate 1 `crates/session/tests/` and `crates/controls/tests/automation.rs`
  (the wire round trip in `crates/protocol`'s tests); gates 2 and 4
  `hosts/host-web/tests/control_parity.rs` (both hosts) and `crates/capi/src/runtime/live_tests.rs`;
  gate 3 `crates/capi/src/runtime/live_tests.rs`; gate 5 `crates/host-core` (control provider) and
  `control_parity.rs`; gate 6 `crates/control-plane/tests/vca_automation_realtime.rs` (shared with
  #1649). Add to gate 2: the same `SET_CONTROL` through the in-place path (a transaction of only
  that edit) gives the same reply and bits as through the full path. `./target/release/realtime_policy`
  needs `cargo build --locked --release -p realtime-policy` first.
- **Size.** #1650a about 8-10 implementer hours (grammar, wire, SDK regeneration, fixtures, row
  rule, docs). #1650b about 10-14 hours with R17-1 = A (binding with model scope, the in-place
  owners, the list flag on both sides, the classifier, tests on both hosts); about 7-9 with
  R17-1 = B.

### Ruling question R17-1: how a hand move on an automated lane reaches the offset on every path

- **A. The automation entry is an owner of the in-place edit.** `OwnerEdits` clones, with the
  strip, the rendered entries that drive the written lane; `OwnerSlots` and `ModelView` carry them;
  the builtin row's read and write for rows 2 and 5 take the entry. Keeps D1, keeps the in-place
  cost (independent of the session's size) for the most frequent hand move, a fan riding a fader.
  Widest edit (`session::in_place`, `controls` view, owner edits, effective inputs).
- **B. The in-place path declines it.** A `controls` check on the committed model declines a
  `fader_db` or `trim_db` write on a lane a rendered entry drives; the full path (model clone,
  canonical JSON) commits it. Smallest edit; every hand move on an automated lane then pays the
  full-path cost that #1613 and #1620 removed for large sessions.
- **C. The offset lives on the strip** (`DualMonoFader` and `ChannelBuiltins` gain offset fields).
  In-place works unchanged, but D1 changes: `RemoveAutomation` must reset the offset and the
  binding still needs the table to choose offset or static.
- **Recommendation: A.** It is the correct long-term shape (no path-dependent cost, no D1 change)
  and keeps decision 17 C3's one binding place in `controls`. B would be a shortcut that a later
  issue must remove.

## Rulings (root, batch 17, 2026-10-16)

These rulings override the filed text and every amendment above where they differ.

**New title: "Store a hand move on an automated fader as a dB offset".** The file name does not
change. Reason: ruling R17-6 below.

### R17-1: option A

The automation entries that drive a lane become an owner of the in-place edit. `OwnerEdits`,
`OwnerSlots` and `ModelView` carry the rendered entries that drive the written lane. The builtin
row's read and write take the entry, with model scope.

- A hand move on an automated fader writes the offset (OQ1). The reply and the sound agree.
- An ack of a value that does not sound is a defect.
- `crates/session/src/in_place.rs` and `crates/protocol/src/model.rs` (`prepare_in_place`) are
  authorized paths now, not "if R17-1 = A". The #1650b size of 10-14 hours (R17-1 = A) stands.
- The fast classifier path (`classify_rows`) then reads the lane's entries through the same
  `ModelView`. The assumption of #1648's departure 1 holds on that path. The brief for #1650b must
  say this and test it.
- **Gate kept and widened.** The same `SET_CONTROL` through the in-place path (a transaction of only
  that edit) and through the full path gives the same reply and the same bits. It runs on both
  hosts: the C ABI (`crates/capi/src/runtime/live_tests.rs`) and the browser
  (`hosts/host-web/tests/control_parity.rs`). It is part of gate 2.

### R17-6: this issue delivers the fader offset only

The trim offset moves to #1658 (render stored input trim and polarity automation). The send offset
moves to #1654 (render stored send gain, mute and matrix automation). No inert field stays here.

**What leaves this issue.**

- Product outcome: the sentence "The same rule holds for the input trim ... from the slice that
  renders trim automation (#1658)" leaves. The trim rule is #1658's.
- Deliverables and decisions:
  - D2: `takes_offset` is true for `fader_db` only. `trim_db` leaves (to #1658). Route `gain_db`
    leaves (to #1654).
  - D3: the words "(or, from #1658, trim)" and the last bullet (trim before #1658) leave. After
    this ruling `SET_CONTROL trim_db` is untouched by this issue. A trim entry is not rendered
    (A10), so it stays inert as every unrendered row is, and its offset must be `+0.0`.
  - Batch 10 amendment: the shared-row rule (one offset, `left_db` equals `right_db`), the route
    `gain_db` accessor, the route lowering of D5 and the domain note for the route gain leave
    (to #1654). Its remark on a VCA `offset_db` (#1655) is not changed by this ruling.
  - Batch 17 amendment: the "Trim" bullet (a trim entry carries a checked, stored, inert offset)
    is void. The binding switch of R17-1 covers row 5 only, not rows 2 and 5.
- Authorized paths: no path leaves, because the fader needs every one of them. This issue adds no
  trim or route file (`crates/controls/src/route.rs` is #1654's). The work in each path is smaller.
- Gates:
  - Gate 1 loses the batch 10 additions: a send `gain_db` entry whose two fields differ, and a
    non-zero offset on a send matrix row, move to #1654.
  - Gate 1 keeps: a non-zero offset on any row that does not take one is refused
    `control.automation.offset`. A trim entry and a send entry are such rows until #1658 and #1654
    add them to `takes_offset`. This keeps the field from being an inert, accepted value.
  - Gates 2 to 6 are about the fader only, as they read already. No trim gate and no send gate stay.
- The field is one struct for all entries (the V1 grammar has no optional key; D1). It holds `+0.0`
  on every entry that is not a fader entry, until the slice that renders that row adds the row to
  `takes_offset`. The field is heard for the fader in this issue, so it is not inert.
- Non-goals now also name: an offset on `trim_db` (#1658) and on a send `gain_db` (#1654).
- Dependencies: the line "#1658 (amended) gives the trim its render half and flips D3 for trim"
  now reads: #1658 and #1654 depend on this issue. This issue ships the shared mechanism, tested
  with the fader: the field and grammar (#1650a), the offset write, the owner rule of R17-1 and the
  list flag (#1650b).

**How the split reads now.**

- **#1650a** (the field, the grammar, the wire, the SDK, the fixtures, the docs): D1; D2 with
  `takes_offset` = `fader_db` only, the domain check and the lane rule; gate 1 as kept above. It
  is inert for every row until #1650b, so it can land before #1646. The batch 17 path list for
  #1650a does not change.
- **#1650b** (the fader write, the in-place owner, the list flag, the classifier): D3 to D5 for
  `fader_db`, R17-1 = A, the `automated` list field, and gates 2 to 6 with the widened gate 2. The
  batch 17 path list for #1650b does not change. It lands in the R1 push with #1646 to #1649, with
  the "no middle state" hazard.
- Neither part has a trim or send step. #1658 and #1654 add their rows after this issue lands.

## Decision record: #1650a (batch 17)

Implemented on `codex/controls-batch-17w` over `ba240b6e1`, committed as `5c8781b7e`.

**What shipped.**

- **D1, the field.** `session::Automation` gains `offset: AutomationOffset { left_db, right_db }`
  (`f32`, `Default` is `+0.0` on both lanes), required in every entry. JSON key `offset` 4, after
  `segments`, a record with `left_db` 1 and `right_db` 2 (`keys::automation`,
  `keys::automation_offset`; the walk emits it, so the canonical writer, the estimator and the SDK
  key tables follow). `parse_automation` requires it (`schema.missing_field`,
  `schema.unknown_field`, `schema.wrong_type` at its paths). Wire: `Automation` field 4, a required
  nested `AutomationOffset` with two required `F32` fields 1 and 2, encoded and decoded in
  `session_wire.rs` (a non-finite lane is refused, as every session `f32`); the equal-key test
  pairs the new module. `SetAutomationSegments` and `SetAutomationTarget` keep the offset (they
  write one field of the entry); `RemoveAutomation` removes it with the entry. No code change was
  needed for that.
- **D2, the row rule** (`controls::automation_diagnostics`, `crates/controls/src/automation.rs`).
  `takes_offset(target)` is true only for a strip's builtin `fader_db` (R17-6). Per lane, after the
  entry's segments, at `$.automation[id=<id>].offset.left_db` / `.right_db`: a lane that takes no
  offset (a lane the entry does not drive, or any lane of a row that takes none) must hold exactly
  `+0.0` bits, else `control.automation.offset` (new `ControlError::AutomationOffset`); a lane a
  `fader_db` entry drives is judged by the row's own `check`, so `[-144, 24]` dB, else
  `control.domain`. An offset fault marks the entry faulted for the filter order rule, as a segment
  fault does. `takes_offset` is `pub(crate)`; #1650b exports it if a crate outside `controls` needs it.
- **SDK.** `AutomationSpec.offset?: { leftDb?, rightDb? }` (default `0` per lane); the builder
  writes `offset` after `segments`; the `enginectl` request decoder accepts an optional `offset`
  with those two keys. Key tables regenerated (`sdk/assets/miso-engine-v1-session-keys.json`,
  `sdk/src/generated/session-keys.ts`).
- **Documents.** `fixtures/session/v1/canonical.json` (1 entry), `builtins-automation.json` (6) and
  `.claude/skills/author-session/worked-session.json` (1) gain `"offset": {"left_db": 0.0,
  "right_db": 0.0}`; their canonical bytes are checked by the existing round-trip tests. The
  canonical writer corpus (`fixtures/session-canonical/v1/canonical-writer-corpus.json`) is
  regenerated by the Rust writer; its full-surface model stores a `+3.0` left offset on the left
  `fader-ride`, so the SDK writer is held to a non-zero offset too. `docs/session-v1.schema.json`
  (`automation_offset`, `offset` required), `docs/SESSION_SCHEMA_V1.md` (the offset paragraph and
  the code list), the author-session skill (the key and two diagnostic rows).
- **Inert until #1650b.** Nothing reads the field yet: render's `S` does not include it, and
  `SET_CONTROL fader_db` still writes the static fader. Its production reader is #1650b (the
  offset write, R17-1 = A, `controls::effective` adding the lane's offset first, the list flag),
  which lands in the R1 push with #1646 to #1649. `docs/SESSION_SCHEMA_V1.md` says the offset is
  stored and checked and not yet heard.

**Departures and readings.**

- *`-0.0`.* "A non-zero offset on a row that takes none" is read with the hazard's one-spelling
  rule: every lane that takes no offset must be `+0.0` bits, so `-0.0` is refused there as well
  (R17-6: "It holds `+0.0` on every entry that is not a fader entry"). On a driven fader lane
  `-0.0` is a valid zero.
- *Domain source.* The domain check uses the fader row's own descriptor (`check`), which is
  `[-144, 24]`, the VCA offset domain the spec names; one source, so a change of the row's domain
  moves the offset domain with it (D3's "the row's own domain").
- *Paths outside the #1650a list, all mechanical:* `crates/session/src/canonical.rs` (the corpus
  model gains the field), `crates/session/tests/json_contract_artifacts.rs` (the schema record
  table row), `crates/controls/src/error.rs` (the new code), `crates/controls/src/builtin_rows.rs`
  (`FADER_DB` made `pub(crate)`), struct-literal callers
  (`crates/host-core/tests/{launch_registry_once,live_delta,stored_automation}.rs`,
  `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`), the two
  Rust-built JSON documents (`hosts/host-web/src/tests.rs`,
  `tools/session-validator/tests/validate.rs`), `sdk/src/core/types.ts` and
  `sdk/src/cli/session-request.ts` (the SDK surface), and the re-pin's mirrors
  (`scripts/check-protocol-wasm-parity.sh`, `fuzz/corpus/complete-schema-manifest.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`).
- *Re-pin.* `COMPLETE_SCHEMA_HASH` `0x0c71_96f8_6abe_6d57` -> `0x5eb9_ad2e_d137_f3be`, reason:
  #1650 adds the automation offset (the `Automation` message gains required field 4, which the
  corpus transaction's `UpsertAutomation` carries); 46 frames still. The capi `pcm_digest` is
  unchanged (`bffc21e857b5e134`).

**Gate 1 (as kept by R17-6) and its tests.**

- JSON grammar: `crates/session/tests/invalid_matrix.rs`
  `automation_offset_is_a_required_two_lane_record` (missing record, missing lane, unknown key,
  wrong type; a `3.5` / `-0.0` offset round-trips through the canonical writer with its bits).
- Wire: `crates/protocol/src/session_wire/tests.rs`
  `every_route_and_automation_opcode_round_trips_canonically` (now with a `2.5` / `-0.0` offset)
  and `automation_offset_is_two_required_finite_lanes`; the corpus round trip
  (`crates/session/src/canonical.rs` `canonical_writer_corpus_is_rust_generated_and_current`).
- Row rule: `crates/controls/tests/automation.rs` `the_offset_belongs_to_a_driven_fader_lane`
  (driven fader lanes and the domain ends accepted; a non-driven lane, `-0.0` on a non-driven lane,
  out of domain, a trim, a send gain and a pan entry refused at their paths and codes).
- SDK: `sdk/test/builder-evals.mjs` "issue #1650 -- the builder authors an automation entry's
  offset" (three tests, the third through the engine's `validate()`).

**Checks run.** `cargo test --locked -p session -p protocol -p controls` (378 passed);
`-p host-core --all-features --test live_delta --test launch_registry_once --test stored_automation`
(69); `-p capi` (197); `-p host-web --features host-web/test-support --test control_parity` (21)
and `--lib dense` (2); `-p session-validator` (19); `-p conformance`; `-p parameter-metadata` (20);
`check-sdk-generated.sh`, `check-sdk-types.sh`, `check-protocol-wasm-parity.sh` (and
`--self-test`), `check-command-reason-vocabulary.py` (no change: `control.automation.offset` is a
control diagnostic code, not a `COMMAND_REASON`), `check-builtins-fixtures.sh`,
`check-console-fixtures.sh`, `check-workspace-policy.sh`; SDK `builder-evals.mjs` (56),
`console-evals`, `structure-evals`, `boot-evals`, `live-controls-evals` (131) against a locally
built AudioWorklet module; `audit capi` (`pcm_digest` `bffc21e857b5e134`, 0 violations), `audit
protocol` ok; fmt, clippy `-D warnings` (all features) and the no-feature `cargo check` on the
touched crates.
