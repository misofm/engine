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

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **Anchors moved.** `parameter_metadata`'s `run` is `tools/parameter-metadata/src/lib.rs:1107-1160`
  (the `--write`/`--check` document list at `:1138-1141`); `expectedModules` is
  `sdk/codegen/generate.mjs:888`; the asset list is `sdk/codegen/assets.mjs:42-46`. The `visit.rs` and
  `session-json.ts` anchors held.
- **#1568** is in batch 9 (`codex/controls-batch-9s`). After it merges, `keys::MODULES` and each
  `ALL` exist. Its table keeps a `control_smoothing` row with no wire message; that does not affect
  this slice.
- **D3 needs a tie rule.** #1566 walks the automation target as one variant arm per section. The `own`
  and `builtins` arms have the same key set and the same order, so D3's "more than one candidate
  throws" would throw for them. Amended rule: when more than one candidate has the record's key set,
  all of them must have the same key order and the same leaf kinds, and the writer uses that order.
  D2's generator checks this and fails, naming the candidates, when they differ. So the SDK never
  meets the tie at run time.
- **The generator needs `session`.** `tools/parameter-metadata/Cargo.toml` has no direct `session`
  dependency today. Add it (and update `Cargo.lock`); both are authorized paths for this slice.
- **Edit order.** After #1568. Before #1566 (shared `visit.rs`, `session-json.ts` and the canonical
  writer corpus): #1566 then regenerates the key tables and adds its target arms to D4's table.

## Decision record: S13b session key tables (batch 9)

**Shipped.** D1: `ModelVisitor::record_schema(module, arm)` (defaulted), called by `records!` and
`rec!` right after `record_begin`; `session::VARIANT_ARMS` lists every variant record's arm. D2:
`tools/parameter-metadata/src/session_keys.rs` (`--print-session-keys`, `Recorder`, `build`,
`load_corpus`). D3: `sdk/assets/miso-engine-v1-session-keys.json`, `sdk/src/generated/session-keys.ts`,
and `session-json.ts` now holds no key list. D4: `session_schema_lists_exactly_the_walked_keys`.
`docs/session-v1.schema.json` needed no edit: the test is green on the current document.

**Departures from the text, and why.**

- **Track and submix are variant records.** Their key list differs by `pan` or `matrix`, so the walk
  names them with arms `pan` and `matrix` (`records!` takes an optional arm expression). Without it
  the generator's "one shape, two key lists" rule fails on any corpus that has both.
- **A new corpus document.** No corpus document walked a pan track or a matrix submix. The Rust test
  that writes the corpus gained `canonical-exchanged-strips` (`crates/session/src/canonical.rs`, a
  mechanical addition to an unlisted path; the corpus JSON is regenerated from it).
- **`u64` is its own kind.** Kinds are `integer` (`u8`, `u32`, `source_bit_depth`), `u64`, `float`,
  `boolean`, `id`, `text`, `token`, `record`, `array`. A `u64` is a JSON string, so the SDK keeps
  refusing a bare number there, as the hand copy did.
- **The key tables are not in `--write`/`--check DIR`.** The release artifact set is exactly six
  files (`check-web-audioworklet.sh`, the package manifest) and the key tables are an SDK source, not
  a shipped artifact. `assets.mjs` renders the document with cargo and leaves it out of a check
  against an artifact directory (that check runs no cargo); `session_keys_asset_is_current` holds
  the asset to the generator in `cargo test`.
- **Variant-arm coverage.** The generator also fails when an arm in `VARIANT_ARMS` is never walked
  (message `matrix_or_pan/matrix`), before the key-level check.
- `serde_json` moved from a dev dependency to a dependency of `parameter-metadata` (the corpus
  manifest is read as data); `session` was added; `Cargo.lock` updated.

**Gates.** (1) `check-sdk-generated.sh` with and without the artifact directory, and the SDK
headless suite (`check-sdk-headless.sh`, 524 pass, 0 fail, including the canonical writer corpus
eval), `check-sdk-types.sh` and `sdk-package.sh check`. (2) `a_corpus_without_the_matrix_arm_is_refused_by_name`,
`a_numeric_key_walked_as_two_kinds_is_refused_by_name`, `one_shape_walked_with_two_key_lists_is_refused`,
`a_tie_must_agree_on_key_order_and_kinds`. (3) `session_schema_lists_exactly_the_walked_keys`;
renaming `follows_mute` in the `route` def's `required` makes it red (`$defs/route: required
differs`). (4) A scratch swap of `mute` and `follows_mute` in the `Route` walk changed only the
generated document (and, by regeneration, its module); reverted.
