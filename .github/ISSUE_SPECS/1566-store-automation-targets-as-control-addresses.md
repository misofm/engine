# Store automation targets as control addresses

Slice S12a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C2
and C10). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Decision 17's automation slice is split in two: this slice changes the stored target in the
session; #1567 changes `AUTOMATION_ENQUEUE`.

## Product outcome

A stored automation's target is a control address, the same address `SET_CONTROL` and the control
list use. A session can store automation for any automatable control, a send's gain or a VCA's
offset included, and the engine refuses a target on a control that is not automatable
(`rebuild_only`, `derived`, `monitor` or a `setting`) with one rule read from the row. Rendering
stored automation stays out of scope (#1315, #1058's successors).

## Context

- **The stored target** is `AutomationTarget { entity_id, rack, effect_id, parameter_id, channel }`
  (`crates/session/src/model.rs:980-991`), a strip-only shape: `rack` is `inserts`, `builtins` or
  `console` (`RackName`, `:999`), and a `builtins` target carries the literal `effect_id: "strip"`
  (`BUILTIN_AUTOMATION_EFFECT_ID`, `crates/session/src/validate.rs:844`).
- **Its grammar.** JSON: `parse_target` (`crates/session/src/parse.rs:1584-1605`); keys and
  canonical walk: `keys::target` (`crates/session/src/visit.rs:121`) and the `AutomationTarget`
  walk in the same file; wire: `schema::session::automation_target`
  (`crates/protocol/src/schema.rs:1089-1100`), `tx_automation_target` and
  `parse_automation_target` (`crates/protocol/src/session_wire.rs:1137`, `:1903`), carried by
  `SetAutomationTarget` (`0x0602`) and `UpsertAutomation` (`0x0600`). The SDK writes the keys in
  `sdk/src/internal/session-json.ts:91` and resolves targets in `sdk/src/core/session.ts:1389-1490`.
- **Three validators, one rule each.**
  - `validate_automation` (`crates/session/src/validate.rs:935`) requires the owner to be a track
    or submix and, for an effect target, a `params` entry with the same `(parameter_id, channel)`.
  - `BUILTIN_AUTOMATION_TARGETS` (`validate.rs:868`) is a second spelling of the builtin
    descriptors, held to them by `crates/builtins-compiler/tests/builtin_automation_targets.rs`.
  - `effect_automation_diagnostics` (`crates/effect-compiler/src/prepare.rs:370`) refuses a
    non-block-rate effect target with `effect.automation.rate`; effect preparation calls it
    (`:675`), and so does the classifier when the automation table changes
    (`crates/host-core/src/live_delta.rs:273-278`).
- **Layering.** `session` sits below `controls`, and `controls` depends on `effect-compiler` (#1549).
  So `session` cannot name `controls::ControlAddress`, and `effect-compiler` cannot call
  `controls`. #1546 put `ControlAddress`, `Owner` and `Section` in `controls`; their fields are all
  `session` types (`StableId`, `RackName` codes, `ParameterChannel`).
- **Documents that carry automation:** `fixtures/session/v1/canonical.json`,
  `fixtures/session/v1/builtins-automation.json`,
  `fixtures/session-canonical/v1/canonical-writer-corpus.json`, the author-session skill
  (`.claude/skills/author-session/{SKILL.md,worked-session.json}`), and inline documents in
  `crates/session/tests/*.rs`, `tools/session-validator/tests/validate.rs` and
  `hosts/host-web/src/tests.rs`. The schema is described by `docs/SESSION_SCHEMA_V1.md` and
  `docs/session-v1.schema.json`.

## Decisions frozen for this slice

- **D1. The address moves down.** `ControlAddress`, `Owner` and `Section` move from `controls` to
  a new module `session::address` (`crates/session/src/address.rs`), unchanged. `controls`
  re-exports them (`pub use session::{ControlAddress, Owner, Section}`), so no call site changes.
- **D2. The model.** `Automation.target` becomes a `ControlAddress`. `AutomationTarget`,
  `BUILTIN_AUTOMATION_EFFECT_ID` and `BUILTIN_AUTOMATION_TARGETS` are deleted, with
  `crates/builtins-compiler/tests/builtin_automation_targets.rs`.
- **D3. The JSON grammar.** A target is a record tagged by `section`, in the way `sidechain` is
  tagged by `kind` (`crates/session/src/parse.rs`, `parse_sidechain`): its keys, in canonical
  order, are `owner` (`"strip"`, `"route"` or `"vca"`), `owner_id` (a stable ID), `section`
  (`"own"`, `"builtins"`, `"console"` or `"inserts"`), `slot_id` (a stable ID: the console slot or
  the insert), `control` (a `u32`) and `lane` (`"left"`, `"right"` or `"both"`). `slot_id` is
  required when `section` is `console` or `inserts` and refused (`reject_key`) when it is `own` or
  `builtins`; every other key is always required. The old keys are unknown keys and refuse.
  `owner: "session"` is not a token: no session row is automatable.
- **D4. One address message on the wire.** The wire form of `automation.target` is #1551 D3's
  `control_address` message, the one `SET_CONTROL` carries (`owner_kind` session 1, strip 2,
  route 3, VCA 4; `section` own 6, builtins 4, console 5, inserts 2; `slot_id` present exactly for
  console and inserts; `control` and `lane` required here). `schema::session::automation_target`
  and its two codecs are deleted; the session wire calls the `control_address` codec and refuses
  `owner_kind` session. `keys::target` becomes the JSON names of D3 with `control_address`'s field
  ids (`OWNER="owner":1`, `OWNER_ID="owner_id":2`, `SECTION="section":3`, `SLOT_ID="slot_id":4`,
  `CONTROL="control":5`, `LANE="lane":6`), so the canonical walk and the wire agree.
  `UpsertAutomation` and `SetAutomationTarget` keep their opcodes.
- **D5. Session validation is structural only.** `validate_automation` checks that the owner
  exists in its namespace (a track or submix for `strip`, a route, a VCA), that a route or VCA owner
  uses only section `own`, that a `console` slot is declared in the session console and an
  `inserts` slot is one of that strip's inserts. The rule that an
  effect target needs a `params` entry is deleted: the row exists whether or not the session states
  the value.
- **D6. One row rule.** `controls::automation_diagnostics(model: &SessionModel, sample_rate: u32)
  -> Vec<ControlDiagnostic>` (`ControlDiagnostic { code: &'static str, path: String }`, new in
  `controls`) checks every automation against its row, with path
  `$.automation[id=<id>].target` or `.segments[<i>]`. It uses the code set of `controls::check`
  and #1567 D3, so stored and enqueued automation are refused with the same four codes:
  - `control.address.unknown`: no row at the address (an unknown builtin ID, effect parameter
    or own row);
  - `control.automation.target`: the row is not `automatable()`;
  - `control.lane`: `left` or `right` on a shared row;
  - `control.domain`: a segment's `unit` is not the row's unit, or a segment value that
    `controls::check(descriptor, lane, value, None)` refuses.

  Amended by root's answer to Q13 (2026-10-13): one code set with #1567.
- **D7. Where it runs.** `effect_automation_diagnostics` and its call in effect preparation are
  deleted. host-core's preparation calls `controls::automation_diagnostics` where it collects the
  effect diagnostics and reports them as preparation diagnostics, so every host refuses alike.
  The classifier calls it when the automation table changes (in place of
  `effect_automation_diagnostics`). `tools/session-validator`'s stage 5 (prepare-effects) reports
  them.
- **D8. The SDK.** `sdk/src/core/session.ts` takes automation targets as `ControlAddress` (#1550's
  generated type) and writes D3's keys through `session-json.ts`. Its own target resolution
  checks only what D5 checks; the row rule is the engine's.

## Deliverables

1. D1-D5 in `crates/session` and `crates/protocol`.
2. D6 and D7 in `crates/controls`, `crates/host-core`, `crates/effect-compiler` (the deletion) and
   `tools/session-validator`.
3. D8 in the SDK.
4. Every document of Context migrated; `docs/SESSION_SCHEMA_V1.md` and `docs/session-v1.schema.json`
   updated; the conformance corpus re-pinned once.

## Authorized paths

- `crates/session/src/{address.rs,model.rs,parse.rs,visit.rs,validate.rs,lib.rs,canonical.rs}`,
  `crates/session/tests/**`.
- `crates/protocol/src/{schema.rs,session_wire.rs,model.rs}`, `crates/protocol/src/session_wire/tests.rs`,
  `crates/protocol/tests/**`.
- `crates/controls/**`.
- `crates/effect-compiler/src/prepare.rs` (the deletion), `crates/host-core/src/{prepare.rs,live_delta.rs}`.
- `crates/builtins-compiler/tests/builtin_automation_targets.rs` (deleted).
- `crates/capi/src/runtime/live_tests.rs` (its two inline targets), `hosts/host-web/src/tests.rs`
  (inline documents only).
- `tools/session-validator/**`, `crates/conformance/src/protocol_corpus.rs`.
- `fixtures/session/v1/{canonical.json,builtins-automation.json}`,
  `fixtures/session-canonical/v1/canonical-writer-corpus.json`, and any fixture manifest that pins
  their bytes.
- `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`,
  `.claude/skills/author-session/{SKILL.md,worked-session.json}`.
- `sdk/src/core/session.ts`, `sdk/src/internal/session-json.ts`, `sdk/src/generated/*`
  (regenerated), `sdk/test/**`.

## Non-goals

- No automation rendering and no change to `AUTOMATION_ENQUEUE` (#1567).
- No automation on a monitor or setting row, ever (D6 refuses it by the row).
- No new automation shape or unit.

## Objective gates

Commands: `cargo test --locked -p session -p protocol -p controls -p effect-compiler`,
`cargo test --locked -p host-core --all-features`, `cargo test --locked -p session-validator`,
`cargo test --locked -p conformance`, `cargo test --locked -p capi`,
`bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`.

1. **The row decides.** A session whose automation targets, one each, a `rebuild_only` row
   (`delay_samples`), a derived row (`effective_fader_db`), the monitor row (`solo`) and a setting
   row (`pan_window_samples`) is refused with `control.automation.target` at each target; a send's
   `gain_db` on a route into a submix, a VCA's `offset_db`, a builtin `fader_db` and a block-rate
   effect parameter are accepted.
2. **Lane, unit and domain.** `left` on a route `gain_db` is `control.lane`; a segment
   in `hz` on `fader_db` is `control.domain`; a segment value of 30 dB on `fader_db` is
   `control.domain`; an automation whose address has no row is `control.address.unknown`.
3. **Structure.** An unknown owner, a route owner with section `builtins`, an undeclared console
   slot and a `slot_id` on section `own` are each refused by session
   validation with the existing `MissingEntityReference` or `InvalidEnum` codes.
4. **Round trip.** Every migrated fixture parses, writes back byte-identical, and round-trips
   through the session wire codec; a target with an old key is refused as an unknown key.
5. **One rule on every path.** The same session is refused with the same codes by host-core
   preparation, by a C ABI `SetAutomationTarget` transaction (the rebuild's diagnostic), and by
   `session-validator` stage 5.
6. **The SDK writes the grammar.** An SDK-built session with a route and a VCA automation target
   serializes to bytes the engine parses and writes back unchanged.

## Test value

- Gate 1 turns red if automation can target a value that is not live, or cannot target a live send
  or VCA value.
- Gate 2 turns red if a segment can carry a lane, unit or value its row refuses, or if stored
  automation is refused with a code that #1567 does not use for the same fault.
- Gate 3 turns red if a target names an entity, section or slot that does not exist.
- Gate 4 turns red if the grammar is not strict or the canonical form is not stable.
- Gate 5 turns red if two hosts or the validator tool judge one target differently.
- Gate 6 turns red if the SDK writes the old shape.

## Dependencies

- #1546, #1549 (every row), #1550 (`ControlAddress` in the SDK), #1557 and #1558 (send and VCA rows are live,
  so automatable).
- *Refuse commands that would be acknowledged with no effect* (#1315), for the automation-serving
  rule it states; not a code dependency.

## Supersedes and shrinks

- Shrinks *Refuse commands that would be acknowledged with no effect* (#1315): its stored-target
  half (which targets automation may name) is this slice's row rule; its `AUTOMATION_ENQUEUE`
  refusal stays until automation renders.

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **Anchors moved.** `BUILTIN_AUTOMATION_EFFECT_ID` is `validate.rs:870`, `BUILTIN_AUTOMATION_TARGETS`
  `:894`, `validate_automation` `:961`. `schema::session::automation_target` is `schema.rs:1255-1265`
  (the session specs start at `:903`). `tx_automation_target` is `session_wire.rs:1011`,
  `parse_automation_target` `:1689`. The classifier's call is `live_delta.rs:370`. The SDK resolver is
  `session.ts:1389-1502`. `parse.rs:1584`, `visit.rs:121`, `prepare.rs:370`/`:675` and
  `session-json.ts:91` held.
- **D1 moves the whole module.** `crates/controls/src/address.rs` now also holds `Lane` (the
  `ParameterChannel` alias), `OwnerKind`, `SectionKind` and their `code`/`name`/`kind` impls (#1546
  S7c). An inherent impl cannot stay behind its type, so all of it moves to `session::address`
  (it uses only `session` types), and `controls` re-exports all six names, as `controls/src/lib.rs:46`
  exports them today.
- **D4's codec exists.** `SET_CONTROL`'s edit already reads and writes the address through
  `message_wire::write_set_control_address` and `decode_set_control_address`
  (`session_wire.rs:1296-1305`, `control` and `lane` required). The automation target calls the same
  two functions and then refuses owner kind session. The spec is the top-level
  `crate::schema::control_address` (`schema.rs:344-356`), not a `schema::session` module.
- **#1568 lands first (batch 9).** Its test `session_keys_and_wire_fields_have_equal_ids` pairs the
  `target` row with `automation_target::SPEC`. This slice changes that row to
  `crate::schema::control_address::SPEC` with no wire-only IDs (both sides `{1..6}`), in
  `schema.rs`'s test module (already an authorized path).
- **D3's walk.** Walk the target with `variants!`, one arm per section (the arm token is the section
  name), so each arm has one fixed key list. #1569 D2 refuses a record walked with two key lists.
  The `own` and `builtins` arms have the same key list; #1569's amendment accepts that. The migrated
  corpus walks all four arms.
- **D6 has a gap: `control.inactive`.** `controls::check` returns `control.inactive` for a row that the
  owner's shape does not use (a matrix row on a pan strip). Such a row is `automatable()`, so it
  reaches `check`, and D6's four codes do not include it. Options: (A) add `control.inactive` as a
  fifth code, for stored and enqueued automation alike; (B) report it as `control.automation.target`.
  Recommendation: A. It is `check`'s own code for the same fault in `SET_CONTROL`, so one fault has one
  code everywhere. `control.read_only` cannot occur, because a derived row fails `automatable()`
  first. `ControlError` has no `control.automation.target` variant today; this slice adds one.
- **D7's call sites.** Host-core preparation collects effect diagnostics at `prepare.rs:1908`
  (`prepare_native_session_effects(...).map_err(effect_diagnostics)`); the automation diagnostics
  join that set. `tools/session-validator` has no `controls` dependency yet: add it in its
  `Cargo.toml` (under `tools/session-validator/**`) and update `Cargo.lock`.
- **The browser.** Since #1562a the browser applies session edits through the core. An automation edit
  that the row rule refuses classifies as a rebuild (`LiveRebuild::AutomationTarget`), so until
  #1562b the browser refuses it with `web.apply.rebuild_pending`, not with the row code. An accepted
  edit commits on the live path. Gate 5 keeps its three paths; no browser gate is added.
- **#1315 is closed (batch 8).** Its `AUTOMATION_ENQUEUE` refusal is on `main`; this slice does not
  change it. "Supersedes and shrinks" needs no action.
- **More files that name the old target** (authorized): `crates/effect-compiler/tests/native_session.rs`,
  `crates/host-core/tests/{launch_registry_once,live_delta,live_edit_classifier}.rs`,
  `sdk/src/core/types.ts`, `docs/EFFECT_CONTRACT_V1.md` (its `effect.automation.rate` lines) and
  `Cargo.lock`. Closed handoff records under `docs/handoffs/` stay unedited.
- **Edit order.** After #1568 (`visit.rs`, `schema.rs`); after #1341a1/a2 and #1611
  (`effect-compiler/src/prepare.rs`; #1341a1 also `host-core/src/live_delta.rs`); after #1569
  (shared `visit.rs`, `session-json.ts` and the corpus: this slice then regenerates the SDK key
  tables instead of editing them by hand, and adds the target arms to #1569 D4's `$defs` table);
  after #1602 (`host-core/src/prepare.rs`, `hosts/host-web/src/tests.rs`, different functions).
  Before #1567, and before #1615 (`session/src/canonical.rs`).

## Ruling (root, 2026-10-14): `control.inactive` (the stored path only)

`controls::check` returns `control.inactive` for a row that is automatable but inactive (for
example a matrix row on a pan strip). Root ruled option A: `control.inactive` is the fifth code a
stored automation target can be refused with, the same code `SET_CONTROL` uses for the same fault.
D6's code set is the four filed codes plus `control.inactive`, and a gate shows one inactive
automatable row refused with it. `ControlError` gains the automation-target variant this slice
needs.

## Decision record: S12a (batch 9)

**What shipped.** A stored automation's target is a `session::ControlAddress`. D1-D8 as filed and
amended, with root's ruling: `control.inactive` is the fifth code of the stored-target rule, and
`ControlError` gains `AutomationTarget` (`control.automation.target`).

- **Model and grammar.** `address.rs` moved whole to `session::address` (`ControlAddress`, `Owner`,
  `Section`, `Lane`, `OwnerKind`, `SectionKind`; `controls` re-exports all six). `AutomationTarget`,
  `BUILTIN_AUTOMATION_EFFECT_ID` and `BUILTIN_AUTOMATION_TARGETS` are deleted, with
  `builtin_automation_targets.rs`. `parse_target` reads D3's six keys (`slot_id` required for
  `console` and `inserts`, `reject_key` for `own` and `builtins`; `owner: "session"` is an invalid
  enum). `keys::target` has `control_address`'s field IDs, and the walk is `variants!` with one arm
  per section (`own`, `builtins`, `console`, `inserts`), so the SDK key tables regenerate to four
  `target` arms.
- **Wire.** `schema::session::automation_target` and its two codecs are deleted; `tx_automation_target`
  and `parse_automation_target` call `write_set_control_address` and `decode_set_control_address` and
  refuse owner kind session. The #1568 pairing row is `target` -> `control_address::SPEC`.
- **Validation.** `validate_automation_target` (session) checks structure only: owner exists in its
  namespace (an `Index.routes` set joins the graph map), a route or VCA owner uses `own`
  (`schema.invalid_enum`), a console slot is one the strip carries, an insert is one of the strip's.
- **The row rule.** `controls::automation_diagnostics(model, sample_rate)` with `ControlDiagnostic`:
  `control.address.unknown`, `control.automation.target`, `control.inactive`, `control.lane`,
  `control.domain` (unit differs from the row's, or `check` refuses a segment value). A target is
  reported once and hides its segments' faults.
- **Where it runs.** `effect_automation_diagnostics` is deleted from `effect-compiler`. host-core's
  session preparation merges the rule's diagnostics with the effect diagnostics (one sorted
  `EffectDiagnosticSet`); the classifier calls it in step 2; `session-validator` stage 5 merges it
  the same way (new `controls` dependency, `Cargo.lock`). `effect.automation.rate` no longer exists.
- **SDK.** `AutomationTarget` is `ControlAddress | ControlWireAddress`; `AutomationRack` is removed.
  The builder checks D5 only, finds the row (by machine name or numeric ID) in `CONTROLS` for the
  span's unit, and writes D3's keys; domain checks of span values are gone (the engine's rule).
  `enginectl`'s request parser takes the same address.

**Departures from the text.**

1. Gate 3, `slot_id` on `own`: it is a parse refusal (`schema.unknown_field` at `target.slot_id`,
   D3's `reject_key`), not a validation code; the model cannot hold a slot on `Own`.
2. A target in an effect that has no rows (a `cid` effect, or a native effect the launch registry
   lacks) is not reported by the rule: effect preparation refuses that effect with its own code
   (`effect.third_party.unavailable_at_launch`, `effect.native.unavailable`), as #1335 D2 did for
   the rate code. One fault, one report.
3. `fixtures/session/v1/builtins-automation.json`: the `matrix-ll` automation on a pan strip is
   `control.inactive` now, so it became `pan-ride` (builtin 12, left). The fixture also gained a
   transparent submix `bus`, a send `send` into it, a VCA `riders` and `route-ride` and `vca-ride`
   automations, so one engine-written document holds a route and a VCA target (gate 6). The
   canonical-writer corpus's full-surface document gained the VCA, route, builtins and console
   targets (`canonical-full-surface` and `canonical-exchanged-strips` changed; they walk all four
   arms). `canonical.json` changed only in its target.
4. `COMPLETE_SCHEMA_HASH` is re-pinned once (`0x4f6a_80fb_52c1_2aed`, from `0xcdfd_b054_0f09_3424`): the
   transaction fixture's automation edits carry the address message. The retired-rack rows for the
   target (`retired.rack_code_{1,3}.set_automation_target`) patch the address `section` and meet
   `INVALID_FIELD` (the address decoder's status for an unallocated section), not `MALFORMED_FRAME`.
5. Effect parameters with `AutomationRate::Sample`: the old rule refused them, and the row says
   `Live` for any rate but `None` (`effect_rows::parameter_descriptor`), so the new rule accepts a
   target on one. No launch effect has such a parameter; the test double that held the case is
   deleted with its test. Successor: decide whether the row class must follow `Block` only.
6. Files outside the authorized list, all mechanical: `sdk/src/cli/session-request.ts` (the request
   parser's target), `scripts/check-effect-runtime-policy.sh` and
   `scripts/test-effect-runtime-policy.sh` (the required-diagnostic list lost
   `effect.automation.rate`).

**Gates and tests.**

1. `controls/tests/automation.rs::the_row_decides_what_can_be_automated`.
2. `controls/tests/automation.rs::lane_unit_domain_and_unknown_rows_are_refused_with_the_validators_codes`
   (includes `control.inactive`), `::every_automation_is_judged_and_a_refused_target_reports_once`,
   `::a_target_in_an_unavailable_effect_is_reported_by_the_effect_alone`.
3. `session/tests/automation_target.rs` (`a_target_must_name_things_that_exist`,
   `slot_id_exists_exactly_in_a_console_or_inserts_section`) and the migrated `invalid_matrix`,
   `diagnostic_parity`, `submix_strip`, `vca` rows.
4. `session/tests/automation_target.rs::{the_retired_target_keys_are_unknown,
   every_owner_and_section_round_trips}`; `canonical_writer_corpus_is_rust_generated_and_current`;
   `session_schema_lists_exactly_the_walked_keys`; `protocol` session-wire corpus round trip with a
   target per owner kind and section; `an_automation_target_cannot_be_owned_by_the_session`;
   `session-validator` `canonical_output_reproduces_the_checked_in_canonical_fixtures`.
5. `host-core/tests/stored_automation.rs::preparation_refuses_an_automation_the_row_refuses`;
   `capi` `an_automation_on_a_prepared_effect_parameter_is_refused_before_any_ack` (the C ABI
   transaction's diagnostic equals preparation's, `control.automation.target`);
   `session-validator` `an_automation_on_a_prepared_effect_parameter_fails_effect_preparation`.
6. `sdk/test/builder-evals.mjs` "the builder reproduces a checked-in canonical fixture byte for byte"
   (route and VCA targets included) and "the builder resolves a target's row and leaves the row rule
   to the engine" (the engine's own codes through `validate()`).

Measured numbers: none (no performance claim).

## Fold (batch 9 review)

Departure 5 above is resolved. A row is `Live` only when `automation_rate == Block`
(`effect_rows::parameter_class`): a `None`-rate parameter keeps its prepared reason, and a
`Sample`-rate parameter is `RebuildOnly(Correctness)` with the reopening condition "sample-accurate
render-path delivery, a later protocol capability" (`EFFECT_CONTRACT_V1.md`). So a `Sample`-rate
parameter is no longer automatable and `automation_diagnostics` refuses a target on it with
`control.automation.target`, as the old `effect.automation.rate` did. The classifier's guard in
`live_delta::parameter_records` now describes a defence that the rows already satisfy, and
`EFFECT_CONTRACT_V1.md` states the rule.

Test: `controls::effect_rows::tests::a_sample_rate_parameter_is_rebuild_only_and_not_automatable`
holds the `Sample`-rate double. It lives in the crate (a unit test) and not in
`controls/tests/automation.rs` because no launch effect declares a `Sample`-rate parameter and the
integration tests reach only the launch registry; the unit test calls `parameter_descriptor` on the
double. No launch row changed, so the control catalog and the generated SDK files are unchanged.
