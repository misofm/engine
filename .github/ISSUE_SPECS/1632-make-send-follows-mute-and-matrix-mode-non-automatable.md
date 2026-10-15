# Make a send's follows_mute and a strip's matrix_mode non-automatable

Root's ruling on finding F20 of the stored-automation check (#1058 README, "Root's rulings
(2026-10-14)"), decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, C3). Code anchors
verified on `2e6343a99`. Issue number: assigned at filing (written here as #1632). Feature sprint
mode (owner, 2026-10-13): one implementer, normal tests, a batch review. Root reports the ruling to
the owner as revisitable.

## Product outcome

A route's `follows_mute` and a strip's `matrix_mode` stay live controls but are not automation
targets: the control list says `automatable = false` for both, and a stored automation entry on
either is refused `control.automation.target` at preparation, in the classifier, in
`session-validator` and in the SDK's generated tables, on both hosts. They are a routing setting
and a mode, not performance values.

## Context

- `automatable` is computed, never declared: live, model scope and not a `Setting`
  (`ControlDescriptor::automatable`, `crates/controls/src/descriptor.rs:124-132`). Both rows pass it
  today: `matrix_mode` (`crates/controls/src/strip.rs:80-96`, family `StripMatrix`) and
  `follows_mute` on a route into a submix (`crates/controls/src/route.rs:170-190`, family `Route`).
- The row rule refuses a target that is not automatable with `control.automation.target`
  (`controls::automation_diagnostics`, `crates/controls/src/automation.rs:72`). Preparation, the
  classifier and `session-validator` call it.
- The wire check is an implication (an automatable row must be live, model scope, not a setting:
  `crates/protocol/src/message_wire.rs:3294`), so a live row that is not automatable is already a
  valid control row. `docs/CONTROL_PROTOCOL_REGISTRY.md:54` states the rule.
- No checked-in fixture automates either row (`fixtures/session/v1/builtins-automation.json` holds
  builtins rows of `vocal`, `route-ride` on route control 1 and `vca-ride`).
- The stored-automation drafts assume this ruling: #1654 (amended) renders no `follows_mute`
  entry, and a `matrix_mode` entry would make rows active and inactive over time, which no draft
  covers.

## Decisions frozen for this slice

- **D1. One declared exclusion.** `ControlDescriptor` gains one field that defaults to "computed"
  in `BASE` and that a row may set to an exclusion with its reason. `automatable()` becomes the
  computed rule **and** not excluded. `matrix_mode` declares "a mode, not a performance value";
  `follows_mute` declares "a routing setting, not a performance value". Both reasons name the
  reopening condition: an owner request. Every other row keeps its value.
- **D2. The row rule needs no new code.** Its `!descriptor.automatable()` check refuses both rows
  with `control.automation.target`.
- **D3. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md:54` and the automation-target text of
  `docs/SESSION_SCHEMA_V1.md` (near `:252-267`) name the two exclusions; decision 17's record gains
  one line under C3 (root's ruling, revisitable).
- **D4. Fixtures, corpus and SDK follow.** The SDK's generated control tables are regenerated
  (`automatable` of the two rows); every pinned digest that moves (a control-list corpus row in
  `crates/conformance/src/protocol_corpus.rs`, the qualification artifacts) is re-pinned with the
  reason "#1632 makes follows_mute and matrix_mode non-automatable".

## Objective gates

1. `crates/controls/tests/automation.rs`: an entry on a send's `follows_mute` and an entry on a
   strip's `matrix_mode` are refused `control.automation.target` at `.target`; preparation and
   `session-validator` report the same code.
2. `crates/controls/tests/list.rs`: both rows list `automatable = false`; every other row's
   `automatable` is unchanged (the full table compared with the previous one, except the two rows).
3. `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts` and
   `bash scripts/check-sdk-types.sh` pass with the regenerated tables.
4. `cargo test --locked -p controls -p protocol -p conformance -p host-core --all-features
   -p session-validator`; clippy `-D warnings`; `cargo fmt --all -- --check`.

## Authorized paths

- `crates/controls/src/{descriptor.rs,strip.rs,route.rs}`, `crates/controls/tests/{automation.rs,list.rs}`
- `crates/conformance/src/protocol_corpus.rs` (re-pins only), `sdk/src/generated/*` (regenerated)
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/SESSION_SCHEMA_V1.md`,
  `docs/rulings/one-control-abstraction-2026-10-13.md` (one line), this spec's decision record

## Dependencies

- None. #1654 of the stored-automation program (README) assumes it (ordering only).

## Size

Half a day.

## Decision record: #1632 (batch 13a)

Shipped as written (D1 to D4). `ControlDescriptor` gains `automation: AutomationRule` (`BASE` and every
other table: `Computed`); `automatable()` is the computed rule and `automation == Computed`.
`matrix_mode` ("a mode, not a performance value") and `follows_mute` ("a routing setting, not a
performance value") declare `AutomationRule::Excluded(&AutomationExclusion { reason, reopen: "an
owner request" })`. The row rule (`controls::automation_diagnostics`), preparation, the classifier
and `session-validator` need no new code: all four already refuse `!automatable()` as
`control.automation.target`. The wire check (an implication) is unchanged.

Departure from the text: the exclusion is held by reference (`Excluded(&'static AutomationExclusion)`),
not inline. A listing copies one descriptor per row, and an inline pair of strings measurably grew
the live-only `control_command` heap (#1615 gate: first peak 81,451 against a ceiling of 78,000);
the reference costs one word per row (first peak 77,347, steady 72,588; ceilings 78,000 and 73,000
hold).

Gates and tests: `crates/controls/tests/automation.rs`
`the_row_decides_what_can_be_automated` (both rows refused at `.target`),
`crates/controls/tests/list.rs` `automatable_is_exactly_the_live_model_value_rows` (previous list
less the two names) and `follows_mute_and_matrix_mode_are_live_and_not_automatable`. Re-pins: the
host-web `tests/fixtures/control-list.json` (`automatable` of the five listed rows; reason
"#1632 makes follows_mute and matrix_mode non-automatable"), `sdk/assets/miso-engine-v1-parameter-metadata.json`
and `sdk/src/generated/controls.ts` (regenerated). No conformance corpus digest moved. No checked-in
automation fixture used either row. #1631 interplay: it still covers a pan entry on a strip
switched to matrix; a `matrix_mode` entry is now refused at the target before activity is judged.
