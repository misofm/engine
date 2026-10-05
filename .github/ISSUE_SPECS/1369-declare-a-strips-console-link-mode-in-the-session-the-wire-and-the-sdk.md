# Declare a strip's console link mode in the session, the wire and the SDK

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4).
Slice L2 of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A session author sets a console slot's detector link mode per strip, or leaves the strip on the
slot's mode. The session JSON, the binary session wire and the SDK builder all carry it. Every
existing document migrates by one added key and renders the same bits. The strip's console slot
still binds one bank (#1368).

## Context

- `ConsoleSlot.link_mode` is declared once per slot (`crates/session/src/model.rs:272-283`); a
  `ConsoleEntry` has only `slot`, `bypass`, `params` (`:285-293`); `lower_section` copies the slot's
  mode into every strip (`:452-466`).
- The parser refuses `link_mode` on an entry with `CONSOLE_ENTRY_EFFECT_FIELD`
  (`crates/session/src/parse.rs:904`, `:965-969`). The canonical writer builds entries in
  `crates/session/src/canonical.rs:277-380`; the visitor lists their fields
  (`crates/session/src/visit.rs:233`). The JSON schema is `docs/session-v1.schema.json`; V1 has no
  optional fields (`docs/SESSION_SCHEMA_V1.md:210`).
- The wire: `tx_console_entry` and `parse_console_entry` (`crates/protocol/src/session_wire.rs:957`,
  `:1569`), the field table `schema::session::console_entry` (`crates/protocol/src/schema.rs:888`).
- The SDK: `ConsoleEntrySpec` (`sdk/src/core/types.ts:209`), the accepted keys
  `CONSOLE_ENTRY_KEYS` (`sdk/src/core/session.ts:178`) and their normalisation (`:1309-1320`).
  The SDK's canonical writer fixes a console entry's key set to `["slot", "bypass", "params"]`
  (`sdk/src/internal/session-json.ts:127`), so `toJson()` throws on any other key.
- The protocol corpus pins `COMPLETE_SCHEMA_HASH` (`crates/conformance/src/protocol_corpus.rs:737`)
  with a re-pin history. The same hash is spelled in `scripts/check-protocol-wasm-parity.sh:172-173`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and `fuzz/corpus/complete-schema-manifest.md:10`, and the
  registry's nested-model line lists the console entry as `1:slot,2:bypass,3*:parameter`
  (`docs/CONTROL_PROTOCOL_REGISTRY.md:85`).
- An unsupported mode on an insert refuses with `effect.link_mode.unsupported`
  (`crates/effect-compiler/src/prepare.rs:367-374`).
- 23 checked-in JSON documents declare a console (`grep -rl '"pre_insert"' --include=*.json`).

## Decisions frozen for this slice

- **D1. Grammar.** Each console entry gains a **required** key `link_mode` with the closed token
  set `slot`, `dual_mono`, `maximum`, `average`. `slot` runs the slot's declared mode. The model's
  `ConsoleEntry` gains `link_mode: ConsoleLinkMode` (`Slot` or a `LinkMode`). `lower_section`
  resolves it: `Slot` takes `slot.link_mode`, any other value overrides. Canonical key order is
  `slot`, `bypass`, `link_mode`, `params` (as an effect orders `bypass`, `link_mode`, `params`),
  in the Rust writer and the SDK writer alike; the visitor gives `link_mode` entry field 4
  (`docs/SESSION_SCHEMA_V1.md:200` lists fields 1-3 today).
- **D2. Refusal.** A resolved mode the slot's effect does not support refuses with
  `effect.link_mode.unsupported` at the entry's path
  (`$.tracks[id=..].console[slot=..].link_mode`, or the submix strip's prefix), from the same check
  that serves inserts. `CONSOLE_ENTRY_EFFECT_FIELD` no longer lists `link_mode`.
- **D3. Migration.** A one-off rewrite writes `"link_mode": "slot"` into every console entry of
  every checked-in document, fixture and embedded test session. Nothing else in any document moves.
  `slot` resolves to today's mode, so every render digest stays.
- **D4. Wire.** The console entry message appends one field, `LINK_MODE` (a closed enum with
  `slot` as 0), after its current fields: an in-place V1 amendment on the decision 12 and 13
  precedent. Nothing is renumbered and there is no `ABI_VERSION` bump.
- **D5. SDK.** `ConsoleEntrySpec` gains optional `linkMode` (`"slot"` by default, written
  explicitly into the document); the builder refuses an unknown token. The generated catalog states
  which effects accept which modes (from the descriptors).
- **D6. A change is not decided here.** Whether a changed `link_mode` rebuilds or applies live is
  #1371's. Until #1371 lands, the classifier compares it structurally like every effect field
  (`crates/host-core/src/live_delta.rs:150-156`); a change is a rebuild with a D15-9 transition.

## Deliverables

1. D1-D3 in `crates/session`, the schema docs and the fixtures.
2. D4 in `crates/protocol`, and the one re-pin of `COMPLETE_SCHEMA_HASH`
   (`crates/conformance/src/protocol_corpus.rs:737`), the single cross-target corpus owner, with
   one line in its re-pin history naming this issue (the console entry appends `LINK_MODE`). The
   same re-pin updates every other spelling of the hash: the two lines of
   `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and
   `fuzz/corpus/complete-schema-manifest.md` (hash and history line). The registry's console entry
   becomes `1:slot,2:bypass,3*:parameter,4:link-mode` (`docs/CONTROL_PROTOCOL_REGISTRY.md:85`).
   Re-pin from the value the corpus computes on `main` at merge time, never by hand-editing history.
3. The new field in every Rust `ConsoleEntry { .. }` literal (the model adds a field, so each one
   must name it; `"slot"`/`ConsoleLinkMode::Slot` everywhere).
4. D5 in `sdk/` with regenerated outputs, the CLI's console-entry request parser, and the SDK
   canonical writer's console-entry key order, which becomes `["slot", "bypass", "link_mode",
   "params"]`, D1's order (`sdk/src/internal/session-json.ts:127`).
5. The binding-text amendments (K3 verdict MINOR-2), in this PR:
   - `AGENTS.md` (`:31`), the console sentence: the slot's `link_mode` becomes the default a strip's
     entry may override.
   - `docs/rulings/engine-footprint-2026-09-29.md`, the **Shape** bullet (`:48`): an amendment note
     that the entry now also carries `link_mode` (`slot` or a link token), citing decision 13 Q1 and
     this issue.
   - `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, Q1 (`:185`): recorded as delivered.
6. The `author-session` skill and the app handoff drop the bus-compression hazard guidance
   (DESIGN 2.2b, `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`).

## Authorized paths

- `crates/session/src/`, `crates/session/tests/`, `docs/SESSION_SCHEMA_V1.md`,
  `docs/session-v1.schema.json`
- `crates/protocol/src/schema.rs`, `crates/protocol/src/session_wire.rs`,
  `crates/protocol/src/session_wire/tests.rs`, `crates/protocol/tests/`
- The re-pin set (deliverable 2 only): `crates/conformance/src/protocol_corpus.rs` (the
  console-entry literals, `COMPLETE_SCHEMA_HASH` and its history), `scripts/check-protocol-wasm-parity.sh`
  (the two pinned hash lines), `docs/CONTROL_PROTOCOL_CONFORMANCE.md`,
  `fuzz/corpus/complete-schema-manifest.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`
- `crates/effect-compiler/src/prepare.rs` (the console entry's path in D2's diagnostic only),
  `crates/effect-compiler/tests/` (gate 2), `crates/graph-compiler/tests/` (gate 5)
- the new field only, in every other `ConsoleEntry { .. }` literal on `main`:
  `crates/protocol/src/controller/tests.rs`, `crates/host-core/src/control_provider.rs` (`:782`),
  `crates/host-core/tests/` (`live_addressing.rs`, `submix_strip.rs`, `strip_meters.rs`,
  `live_delta.rs`, `randomized.rs`), `crates/capi/src/runtime/live_tests.rs`,
  `crates/graph-compiler/src/lib.rs`, `hosts/host-web/src/tests.rs`,
  `tools/audit/src/builtins_graph.rs`, `tools/audit/src/fixture_builtins.rs`,
  `tools/console-workload/src/lib.rs`
- every checked-in session JSON document and embedded test session (D3's key only)
- `sdk/src/core/types.ts`, `sdk/src/core/session.ts`, `sdk/src/internal/session-json.ts`,
  `sdk/src/cli/session-request.ts`,
  `sdk/test/`, `sdk/src/generated/`, `sdk/assets/` (generated only); stream H owns `sdk/`:
  coordinate the merge
- `AGENTS.md`, `docs/rulings/engine-footprint-2026-09-29.md`,
  `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md` (the three amendments only)
- `.claude/skills/author-session/SKILL.md` and `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`
  (deliverable 6 only)

## Non-goals

- Live link switching (#1370, #1371). Overriding a slot's effect, quality or sidechain per strip.

## Objective gates

1. **Grammar.** The parser, canonical writer and visitor accept the four tokens on an entry and
   refuse any other token and a missing key; canonical JSON round-trips byte-exact.
2. **Refusal at the entry.** A `maximum` override on an EQ slot, and an `average` override on a
   limiter slot, refuse with `effect.link_mode.unsupported` at the entry's path, for a track and a
   submix strip.
3. **Wire.** The session wire round-trips every token; a message without the field, or with an
   unknown enum value, refuses.
4. **Migration.** `git diff --stat` of the migration commit touches only the added key, and every
   checked-in render digest is unchanged.
5. **An override is an insert at that mode.** Console `pre_insert` = [compressor slot `comp`, the
   last pre-insert slot], `post_insert` empty, eight tracks. Session A: track X's entry overrides
   `comp` to `maximum`. Session B: X's entry is `slot` (`dual_mono`) and bypassed, and X carries a
   compressor insert at `maximum` with the same params as its first insert. X's output is
   bit-identical in A and B (the compressor's latency is 0, so the bypassed slot is the identity),
   every other track is bit-identical, and in A the slot binds one bank over all eight tracks (#1368).
6. **SDK.** The builder writes `"slot"` by default, writes `linkMode` when given, and refuses an
   unknown token.
7. Commands:
   - `cargo test --locked -p session -p protocol -p effect-compiler -p graph-compiler --features protocol/test-support,effect-compiler/test-support,graph/test-support`
   - `cargo test --locked -p conformance --test conformance_corpus` and
     `cargo run --locked -p conformance --example conformance_fixtures -- --check` (the re-pinned
     corpus hash); `bash scripts/check-protocol-wasm-parity.sh` (every spelling of the hash agrees)
   - `cargo test --locked -p host-core -p capi -p host-web -p audit -p console-workload --features host-core/test-support,host-core/control-provider,host-web/test-support`
     (every `ConsoleEntry` literal still builds and passes)
   - `cargo run --locked -p session-validator -- validate <file>` on each migrated document
   - `bash scripts/check-session-policy.sh`; `bash scripts/test-builtins-fixtures.sh`
   - `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`, `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 2: a check that reads the slot's mode instead of the resolved one admits an unsupported
  override; it turns red.
- Gate 5: a lowering that ignores the override, or applies it to every strip, renders the wrong
  detector on one strip; it turns red.
- Superseded in this PR: any test asserting that `link_mode` on an entry refuses with
  `CONSOLE_ENTRY_EFFECT_FIELD`; rewrite it as gate 1's acceptance.

## Dependencies

- *Lower the link mode to per-lane state in the linked effects' banks* (#1368).
