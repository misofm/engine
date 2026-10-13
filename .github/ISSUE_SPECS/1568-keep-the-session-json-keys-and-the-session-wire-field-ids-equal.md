# Keep the session JSON keys and the session wire field IDs equal

Slice S13a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, "How to add a
control", step 2.1; evidence in `17-s13-assessment.md`). Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review;
no per-attempt verifier or mutation runs.

## Product outcome

A new session value field is declared with one field ID in the session key registry, and a test
turns red if the protocol's session wire schema gives that record a different set of field IDs. Today
the two lists are separate copies that nothing compares, so a new key can reach the canonical JSON
with no wire field, or with a wire field under another ID, and the two forms of one session drift.

## Context

- **The key registry.** `session::keys` (`crates/session/src/visit.rs:84-125`): one `key_module!`
  per record, each field a `FieldKey { name, id }` (`:7-12`). The canonical JSON walk and the
  estimator read it. Its doc says the IDs are "transcribed from `protocol/src/schema.rs`" (`:83`).
  The macros are `fields!` and `key_module!` (`:80-81`).
- **The wire schema.** `protocol::schema::session` (`crates/protocol/src/schema.rs:737` onward):
  one `pub(crate) mod` per record with `FieldSpec { id, wire, mandatory, repeated, nested }`
  (`:162-168`) and a `MessageSpec { name, fields }` (`:209-212`). The wire encoder and decoder
  (`crates/protocol/src/session_wire.rs`) read only this. Its existing structural test is
  `migrated_specs_are_sorted_unique_and_structurally_consistent` (`schema.rs:1716` onward), which
  never looks at `session::keys`.
- **The record pairs** (keys module → wire spec): `console`, `console_slot`, `console_entry`,
  `render_profile`, `output_profile`, `source`, `track` (`track::SPEC`; the retired fields 6 and 8
  are in `track::RETIRED`, `:1003-1009`, and have no key), `builtins`, `channel_builtins`, `rack`,
  `effect`, `identity` → `effect_identity`, `param`, `sidechain` → `sidechain::KNOWN`, `fader`,
  `matrix_or_pan` → `matrix_or_pan::KNOWN`, `submix`, `vca`, `control_smoothing`, `output`,
  `route`, `route_source` → `route_source::KNOWN`, `route_destination`, `channel_matrix`,
  `automation`, `target` → `automation_target`, `segment` → `automation_segment`. The root module
  `session` has no wire message (the root is not sent whole).
- **Wire-only fields.** `matrix_or_pan`'s field 1 is a wire-only tag: the walk emits it with
  `wire_tag`, not with a key (`visit.rs:282-287`). Every other tagged record names its tag as a
  `kind` key with ID 1.
- **No shared ID test exists** (assessment, row 7).

## Decisions frozen for this slice

- **D1. The registry lists itself.** Replace the per-module `key_module!` invocations with one
  macro invocation, `key_modules! { (name, "doc"; FIELD = "json" : id, ...) ... }`, that emits the
  same modules and constants as today plus, in each module, `pub const ALL: &[FieldKey]` (its
  fields in declaration order) and, in `keys`, `pub const MODULES: &[(&str, &[FieldKey])]` (every
  module's name and `ALL`, in declaration order). Every existing constant keeps its name and value;
  the canonical bytes and the walk do not change.
- **D2. The test.** A unit test in `crates/protocol/src/schema.rs`'s test module,
  `session_keys_and_wire_fields_have_equal_ids`:
  - a fixed table pairs each keys module name with its wire spec (Context) and with its wire-only
    IDs (`{1}` for `matrix_or_pan`, empty for every other);
  - for each pair: the set of `ALL` IDs, plus the wire-only IDs, equals the set of the spec's field
    IDs;
  - every name in `keys::MODULES` except `session` has a row in the table, and every row names a
    module in `keys::MODULES`. So a new keys module without a wire pairing, or a removed module,
    turns the test red.
  It compares ID sets, not wire types: a type mismatch is caught by the existing codec round-trip
  tests.
- **D3. The doc line.** `visit.rs:83`'s "transcribed from" text says instead: the IDs are the wire
  schema's, and `session_keys_and_wire_fields_have_equal_ids` holds them equal.
- **D4. Order with #1566.** #1566 replaces the `automation_target` spec with #1551's
  `control_address` message and gives `keys::target` that message's IDs. Whichever of the two
  slices lands second updates the `target` row of D2's table.

## Deliverables

1. D1 in `crates/session/src/visit.rs`.
2. D2's test and D3's doc line.

## Authorized paths

- `crates/session/src/visit.rs`.
- `crates/protocol/src/schema.rs` (its test module only).
- `crates/session/tests/visit_model.rs` (only if a test there names a macro that D1 renames).

## Non-goals

- No change to any field ID, key name, canonical byte or wire byte.
- No generation of the wire schema from the keys (the assessment explains why).
- No check of wire types or of edit payload specs.

## Objective gates

Commands: `cargo test --locked -p session -p protocol`, `cargo test --locked -p conformance`.

1. `session_keys_and_wire_fields_have_equal_ids` passes on `main`'s IDs.
2. **It catches drift** (PR evidence, not a committed test): in a scratch commit, change
   `keys::route::FOLLOWS_MUTE` to ID 8 and record that the test is red; add a key to
   `keys::fader` with no wire field and record that it is red; restore.
3. Every existing session, protocol and conformance test passes, and `COMPLETE_SCHEMA_HASH` does
   not move.

## Test value

- Gate 1 turns red if a session record's JSON keys and its wire fields name different IDs, or if a
  new record is added to one registry and not the other; no existing test compares them.
- Gate 2 records that the test sees both kinds of drift.
- Gate 3 turns red if D1's macro rewrite changes a key or a byte.

## Dependencies

- None. It can land at any point of decision 17; see D4 for its order with #1566.

## Supersedes and shrinks

- None.
