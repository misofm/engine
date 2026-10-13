# Generate the SDK's session key tables from the engine's walk

Slice S13b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, "How to add a
control", step 2.1; evidence in `17-s13-assessment.md`, rows 11 and 12). Code anchors verified on
`main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

When a session value field is added in Rust, the SDK's canonical session writer learns the key, its
position and its number spelling by regeneration, with no hand edit. The checked JSON Schema
document can no longer miss a key: a test turns red until the document lists every key the engine
writes, in its record, and no other.

## Context

- **The SDK's hand copy.** `sdk/src/internal/session-json.ts` holds `ROOT_KEYS` (`:13-30`),
  `INTEGER_KEYS` and `FLOAT_KEYS` (`:39-70`, "transcribed from the Rust field walk"),
  `OBJECT_KEY_ORDERS` (`:72-93`) and the hand-written variant and parent rules `taggedOrder` and
  `objectOrder` (`:95-144`). The canonical writer corpus
  (`fixtures/session-canonical/v1/canonical-writer-corpus.json`) holds SDK and Rust output equal
  (`sdk/test/builder-evals.mjs`).
- **The engine's walk.** `session::VisitModel` walks a model and calls a `ModelVisitor`
  (`crates/session/src/visit.rs:33-79`) with typed calls (`u8`, `u32`, `u64`, `f32`, `bool`,
  `id`, `text`, `token`, `source_bit_depth`) and record and array events. The records and their
  key modules are bound in the `records!` and `variants!` macros (`visit.rs:142-160`, invoked at
  `:220` and `:273`), which know
  each record's key module and, for a variant, its arm token.
- **The generator chain.** `tools/parameter-metadata` emits JSON documents (`--print`,
  `--print-abi-layout`; `--write DIR` and `--check DIR` cover all of them,
  `tools/parameter-metadata/src/lib.rs:851-905`); `sdk/codegen/assets.mjs` copies them into
  `sdk/assets/` (its `documents` list); `sdk/codegen/generate.mjs` renders
  `sdk/src/generated/*.ts` (`expectedModules`, `:150-165`); `scripts/check-sdk-generated.sh` checks
  both arrows byte for byte.
- **The JSON Schema document.** `docs/session-v1.schema.json` (`$defs` per record, every object
  closed) is hand-authored: it carries types, patterns, domains and cross-field rules (for example
  a route into the output must have `follows_mute: false`) that no key list holds. It is checked by
  `crates/session/tests/json_contract_artifacts.rs` (closed objects, the `u64` pattern, the
  revision ceiling). Nothing checks its key lists against the engine.
- **#1568** adds `keys::MODULES` and each module's `ALL`.

## Decisions frozen for this slice

- **D1. The walk names its records.** `ModelVisitor` gains one defaulted method,
  `fn record_schema(&mut self, module: &'static str, arm: Option<&'static str>) ->
  Result<(), Self::Error> { Ok(()) }`. The `records!` macro calls it right after `record_begin`
  with the key module's name and `None`; the `variants!` macro with the module's name and the
  arm's token text (`"pan"`, `"matrix"`, `"native"`, `"routed"`, `"default"` and so on). Existing
  visitors keep the default, so no canonical byte, wire byte or estimate changes.
- **D2. A third generated document.** `parameter_metadata --print-session-keys` emits
  `miso-engine-v1-session-keys.json`, and `--write`/`--check` cover it. It walks every document of
  the canonical writer corpus with `WalkOrder::Canonical` and a recording visitor and writes:
  - `records`: per `(module, arm)`, the keys in emit order, each with its kind: `integer` (`u8`,
    `u32`, `u64`, `source_bit_depth`), `float` (`f32`), `boolean`, `id`, `text`, `token`, `record`
    or `array`;
  - `parents`: per parent JSON key (the key of the field or array that holds the record; `""` for
    the root), the `(module, arm)` entries found under it.
  - The generator fails, naming what is missing, when a key of some `keys::MODULES` entry is never
    walked, when one `(module, arm)` is walked with two different key lists, or when one leaf name is
    walked as both `integer` and `float`. If the corpus does not reach a key, the fix is a new
    corpus document written by the Rust canonical writer.
- **D3. The SDK reads it.** `sdk/codegen/assets.mjs` adds the document; `generate.mjs` renders
  `sdk/src/generated/session-keys.ts` from it. `session-json.ts` deletes `ROOT_KEYS`,
  `INTEGER_KEYS`, `FLOAT_KEYS`, `OBJECT_KEY_ORDERS`, `taggedOrder` and `objectOrder`. Its one rule:
  for a record under parent key `k`, take the candidates `parents[k]` and use the one whose key set
  equals the record's key set exactly; none or more than one throws `MisoUsageError`. A leaf
  number is written as an integer or a float by its walked kind. The canonical output of every
  corpus document does not change. `generate.mjs`'s provenance artifact set gains the new
  document if it lists the asset documents.
- **D4. The JSON Schema document's key lists are checked, not generated.** Its types, patterns,
  domains and conditionals are real authored content, so the document stays hand-written. A new
  test in `crates/session/tests/json_contract_artifacts.rs`,
  `session_schema_lists_exactly_the_walked_keys`, holds its keys to the walk: a fixed table maps
  each `$defs` name to its `(module, arm)` entries (for example `route` → `(route, None)`; `pan`
  → `(matrix_or_pan, "pan")`; `effect_identity` → `(identity, "native")` and `(identity, "cid")`);
  for each entry, the union of every `required` array in the def (top level and inside `oneOf`,
  `anyOf` and `allOf` arms) equals the union of the walked key lists of its entries, and so do the
  def's `properties` keys. Every walked `(module, arm)` must appear in the table. A new key
  therefore turns the test red until its property schema, with its type and domain, is written.

## Deliverables

1. D1 in `crates/session/src/visit.rs`.
2. D2 in `tools/parameter-metadata` (a new module beside `abi_layout.rs`).
3. D3 in `sdk/codegen/{assets.mjs,generate.mjs}`, `sdk/src/internal/session-json.ts`, the new
   asset and generated module.
4. D4's test.

## Authorized paths

- `crates/session/src/visit.rs`, `crates/session/tests/json_contract_artifacts.rs`.
- `tools/parameter-metadata/src/**`, `tools/parameter-metadata/tests/**`.
- `sdk/codegen/assets.mjs`, `sdk/codegen/generate.mjs`, `sdk/src/internal/session-json.ts`,
  `sdk/src/generated/session-keys.ts` (new, generated), `sdk/src/generated/provenance.ts`
  (regenerated), `sdk/assets/miso-engine-v1-session-keys.json` (new, generated), `sdk/test/**`.
- `scripts/build-web-audioworklet.sh` and `scripts/sdk-package.sh`, only if they list the asset
  documents by name.
- `fixtures/session-canonical/v1/canonical-writer-corpus.json`, only to add a document D2 needs.
- `docs/session-v1.schema.json`, only to fix a key-list difference that D4's test finds on `main`.

## Non-goals

- No generation of the JSON Schema document's types, domains or rules, and none of
  `docs/SESSION_SCHEMA_V1.md` (prose).
- No change to the SDK's session builder API or its authoring checks in `sdk/src/core/session.ts`.
- No change to a canonical byte, a wire byte or a field ID.

## Objective gates

Commands: `cargo test --locked -p session -p parameter-metadata`,
`bash scripts/check-sdk-generated.sh`, `bash scripts/check-sdk-types.sh`,
`bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
`bash scripts/sdk-package.sh check target/ci/qualification-artifacts`.

1. **The SDK writer is generated.** `check-sdk-generated.sh` passes with the new document and
   module, and `sdk/test/builder-evals.mjs`'s canonical writer corpus eval passes unchanged (every
   document written byte for byte as the Rust writer writes it).
2. **The generator refuses gaps.** Unit tests in `tools/parameter-metadata`: a corpus with no
   `matrix` arm fails naming `matrix_or_pan`/`matrix`; a visitor stub that walks `trim_db` once as
   `f32` and once as `u32` fails naming the key.
3. **The schema document is held.** `session_schema_lists_exactly_the_walked_keys` passes. In a
   scratch commit, removing `follows_mute` from the `route` def's `required` makes it red (PR
   evidence).
4. **A new key reaches the SDK mechanically** (PR evidence): in a scratch commit, add a key to
   `keys::route` and to the `Route` walk; regenerating changes only the generated document and
   module, and the SDK writes the key in the right place with no hand edit.

## Test value

- Gate 1 turns red if the generated table writes a document in a different key order or number
  spelling from the Rust writer.
- Gate 2 turns red if the generator silently omits a record arm or picks one spelling for an
  ambiguous leaf.
- Gate 3 turns red if the JSON Schema document misses a walked key or lists one the engine does not
  write; no test compares them today.
- Gate 4 records that a new key needs no SDK hand edit.

## Dependencies

- #1568 (`keys::MODULES` and each module's `ALL`).

## Supersedes and shrinks

- None.
