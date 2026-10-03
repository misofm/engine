# Declare VCA groups in the session

Slice V1 of *VCA groups* (#1239). It is in the VCA batch, pushed once with #1241-#1246. It is the
grammar, the validation, the canonical form, the SDK writer and builder, and the migration. *Edit VCA
groups through session transactions* (#1241) adds the wire message and the edits; *Apply VCA offsets
and mutes at preparation* (#1242) makes VCAs audible. Between this slice and #1242, inside the batch
only, VCAs parse and are inert.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`) is committed in
`docs/handoffs/submix-sends-2026-10-02/`. The closest precedents are *Mute a route in the session*
(#1216) and *Let a route into a submix follow its source strip's mute in the session* (#1218): read
their attempt records in git history for the re-pin chain this slice repeats.

## Product outcome

A session document can declare VCA groups: `vcas: [{ id, fader, members }]`.

- They are parsed, validated (namespace, references, acyclicity, range) and canonicalised, and
  every snapshot and saved session carries them.
- They are authored from the SDK (`vca(id, { fader?, members })`) and the `enginectl` CLI.

## Context (verified on `8c6268967`)

- **Root keys are all required** (V1 has no optional fields; `docs/SESSION_SCHEMA_V1.md:35-40`).
  - The root field registry is `key_module!(session, ...)` (`crates/session/src/visit.rs:85`):
    `schema_version` 1, `session_id` 2, `revision` 3, `sample_rate_hz` 4, `quantum_frames` 5,
    `render_profile` 6, `output_profile` 7, `sources` 9, `console` 15, `tracks` 10, `submixes` 11,
    `outputs` 12, `routes` 13, `automation` 14. **16 is free.** *Session `controlSmoothing`* (#1054)
    claims no field ID (its spec names none), so this slice takes 16 and #1054 takes the next free
    one when it lands.
  - **ID 8 is an unrecorded gap**: it was `limits`, removed in `04d291dd` with no retirement note.
    It must never be reused. The field-key test is `crates/session/tests/visit_model.rs:171-177`
    (root IDs) and `:188-189` (`console` is 15).
  - The canonical root walk is the `SessionModel=>session` record (`visit.rs:195-200`): its record
    count is `8 + sources + tracks + submixes + outputs + routes + automation` (each array element
    counts as a repeated field), and its order is `schema_version, session_id, revision,
    sample_rate_hz, quantum_frames, render_profile, output_profile, sources, console, tracks,
    submixes, outputs, routes, automation`. `sorted_array` (`:172-190`) sorts by `CanonicalOrd`
    (`:151-156`, an `ids!` macro for ID-keyed records).
  - **The visitor has no array of scalars.** `ModelVisitor` (`visit.rs:33-68`) emits arrays of
    records only: `JsonWriter::field` (`crates/session/src/canonical.rs:66-71`) asserts an object
    parent, and array items are `record_begin(None, ..)`. `members` is the schema's first array of
    strings. Its implementors are `JsonWriter` (`canonical.rs:87-173`), `StringBytes`
    (`crates/session/src/estimate.rs:327-367`) and the test `Trace` (`crates/session/tests/visit_model.rs:13-70`).
  - The JSON parser's root key list is `parse_root` (`crates/session/src/parse.rs:716-735`); the
    lists are parsed at `:743-749` with `parse_list` (`:844-873`, records only); the model literal is
    `:781`. `Parser::id` (`:487-505`) reads one ID field and refuses `id.invalid`.
  - Normalization sorts every entity list by ID in `compile_session` (`crates/session/src/compile.rs:136-151`).
  - The resource estimate counts entities at `estimate.rs:58-62` and `:103-107` and model vectors
    with `vector!` at `:127-134`. `scripts/check-session-policy.sh:32` forbids allocation vocabulary
    (`format!`, `to_owned`, `Vec::with_capacity`, `collect`) in `estimate.rs`.
- **The model.** `SessionModel` (`crates/session/src/model.rs:89-119`); `DualMonoFader { left_db,
  right_db, left_mute, right_mute }` (`:639-648`); `Submix` (`:685-700`). `SessionModel` has one
  struct literal, in `parse.rs:781`.
- **The namespace.** `enum GraphEntity { Track, Submix, Output }` (`crates/session/src/validate.rs:11-16`),
  built at `:70-100`; a duplicate is `id.duplicate` at the second entity's `id`. Route sources and
  destinations (`validate_route_source` `:588`, `validate_route_destination` `:620`), a sidechain
  source (through `validate_route_source`, `:505-511`) and automation targets (`validate_automation`
  `:750`) look entities up in it and refuse anything else with `reference.missing_entity`.
- **Fader validation.** A strip's `fader` is checked for finiteness only (`validate.rs:436-443`); the
  `[-144, 24]` range is enforced by builtins at preparation (`checked_fader_gain`,
  `crates/builtins/src/lib.rs:4108`). `validate_finite_range` (`validate.rs:925-944`) refuses
  `numeric.out_of_schema_range` (finite) or `numeric.non_finite`.
- **Diagnostics.** `DiagnosticCode` (`crates/session/src/diagnostic.rs:17-76`, `#[non_exhaustive]`,
  last variant `ConsoleSlotNotNative`) and `as_str` (`:80-112`). The author-session skill's code
  table is `.claude/skills/author-session/SKILL.md:171-190`.
- **Protocol.** The wire carries session **edits**; the whole session travels as canonical JSON
  (a snapshot). This slice adds no message or opcode (#1241 does), so `COMPLETE_SCHEMA_HASH` does not
  move: the conformance corpus parses `fixtures/session/v1/canonical.json` for its edits
  (`crates/conformance/src/protocol_corpus.rs:18`, `:293`) and its snapshot response carries an
  empty JSON chunk (`:867-871`). The protocol crate's inline sessions still need the key (below).
- **SDK.**
  - The writer (`sdk/src/internal/session-json.ts`): `ROOT_KEYS` (`:13-28`), `INTEGER_KEYS` (`:37`),
    `FLOAT_KEYS` (`:49`; an unlisted numeric leaf throws), `OBJECT_KEY_ORDERS` (`:67-87`; `fader` at
    `:78`), `objectOrder` (`:110-133`, keyed by the parent key), and `jsonValue` (`:148-166`), which
    already writes an array of strings one item per line.
  - The builder (`sdk/src/core/session.ts`): the model type (`:98-112`), `BuilderState` (`:653-663`),
    `submix(id, spec?)` (`:759-775`), `#graphIds` (`:859-865`), the empty state in `session()` (`:933-960`),
    `normalize` (`:1520-1568`), and the engine-code map `CODE` (`:199-212`, no range code yet).
    `FaderSpec` is `sdk/src/core/types.ts:107-112`.
  - The `enginectl` request reader: root keys at `sdk/src/cli/session-request.ts:451`, the submix
    list at `:477-478`.
  - Evals: `sdk/test/console-evals.mjs` (writer parity against `engineCanonical()`),
    `sdk/test/builder-evals.mjs`, `sdk/test/enginectl-cli.mjs`.
- **Session documents that need `"vcas": []`** (`git grep -l '"submixes"' -- '*.json' ':!artifacts/*'`
  lists 22 files: these 21 sessions and `docs/session-v1.schema.json`):
  - `.claude/skills/author-session/worked-session.json`;
  - `crates/graph-compiler/tests/data/reduced-nobus-from-970-verify.json`;
  - `fixtures/session/v1/{builtins-automation,canonical-minimal,canonical,compressor-bank-observation,compressor-dynamic-bank-observation,compressor-dynamic-observation,console-sixty-four-track,console-sixty-four-track-app,console-sixty-four-track-intended,console-sixty-four-track-mono,observation-frame-shape,parametric-eq-bank-console,parametric-eq-nine-track}.json`;
  - `hosts/host-web/qualification/{live-control,observation,stall}-session.json`;
  - `hosts/host-web/tests/browser-v1/{command-session,observation-session,session}.json`;
  - the writer-corpus document `fixtures/session-canonical/v1/canonical-writer-corpus.json`,
    regenerated by `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session
    canonical_writer_corpus_is_rust_generated_and_current` (`crates/session/src/canonical.rs:424`,
    `:493-499`; its corpus model adds a submix at `:253`).

  Historical records under `artifacts/` are evidence and are not migrated.
- **Inline sessions:**
  - `sdk/test/support.mjs:96` (a JS object literal; without `vcas` every SDK eval that boots the
    shipped module goes red, VERIFY-2 M19);
  - `tools/parameter-metadata/tests/abi_layout.rs:166-177` and
    `tools/parameter-metadata/tests/round_trip.rs:293`, `:699`;
  - `crates/protocol/tests/console_session_edits.rs:61` and `crates/protocol/tests/submix_strip_edits.rs:51`.

  Re-run `git grep -n '"automation"' -- '*.rs' '*.mjs' '*.ts' '*.js'` and
  `git grep -n 'submixes:' -- sdk/test hosts tools crates` before editing; a session a test builds
  that lacks the key refuses with `schema.missing_field` at `$`, so a missed one goes red.
- **Derived fixtures.** `console-sixty-four-track-{intended,mono,app}.json` are regenerated by
  `scripts/derive-{intended,mono,app}-console-fixture.py` (none hard-codes the root keys) and held
  byte for byte by `scripts/check-console-fixtures.sh`. Migrate the parent
  `console-sixty-four-track.json`, then regenerate in this order (`mono` and `app` read the
  checked-in `intended`):
  - `python3 -I -B scripts/derive-intended-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-intended.json`
  - `python3 -I -B scripts/derive-mono-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-mono.json`
  - `python3 -I -B scripts/derive-app-console-fixture.py --validator target/release/session_validator > fixtures/session/v1/console-sixty-four-track-app.json`
- **Pinned bytes that move** (each only by the added empty key):
  - The SHA-256 chain over `fixtures/session/v1/canonical.json` (`d946f463...` today):
    `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml:12` and
    `tools/audit/src/fixture_builtins.rs:5102`; their rows `fixtures/builtins/v1/MANIFEST.tsv:10-11`;
    the MANIFEST digest `ACCEPTED_MANIFEST_SHA256` (`tools/audit/src/builtins_graph.rs:51-52`) and
    its copy in `fixture_builtins.rs`. Only `cargo test --locked --release -p audit -p bench -p
    console-workload` (`.github/workflows/qualification.yml:710`) runs those tests.
  - `fixtures/graph/v1/direct-route.*` are generated from `canonical.json` and pinned in
    `fixtures/graph/MANIFEST.tsv`; #1216 and #1218 found they did not move. Regenerate with
    `cargo run --locked -p graph-compiler --bin graph_fixture -- --write` only if `--check` fails.
  - `"sessionDocumentBytes": "1955"` (`hosts/host-web/tests/browser-v1/expected.json:57`, the size
    of `session.json`), with its self-test row `printed_row("sessionDocumentBytes", "1956")`
    (`scripts/check-browser-expected-resources.py:572`, the pinned value plus one).
  - `ALL_COMMAND_RESPONSE_VECTORS[1]` (`crates/capi/src/runtime/tests.rs:87-94`): the C ABI
    snapshot response, whose length word is the nine-track fixture's canonical snapshot size
    (14,179 bytes, `0x3763`). #1216 and #1218 each missed this pin at first (their deviation 1).
- **Base dependence.** *Let a strip override a console slot's link mode* (#1236, open) migrates every
  session through the same pin chain. If it lands first, re-read every pin at this slice's base;
  the pins move only by the added key relative to that base.

## Decisions frozen for this slice

- **D1. Schema.** Root key `vcas`, required (`[]` when empty), root field ID **16**; canonically
  after `submixes`, before `outputs`. If 16 has been allocated when this slice starts (only #1054
  could), take the next unallocated ID, never 8, and record it in the PR.
  - `pub struct Vca { pub id: StableId, pub fader: DualMonoFader, pub members: Vec<StableId> }`, and
    `SessionModel.vcas: Vec<Vca>` after `submixes`. `fader` is read as an offset; `members` may be
    empty.
  - JSON key order of a VCA: `id`, `fader`, `members`. Field keys: a `vca` key module, `id` 1,
    `fader` 2, `members` 3 (#1241 uses the same numbers on the wire). The walk's VCA record declares
    `2 + members.len()` fields, each member counting as a repeated field.
  - Canonical form: `vcas` sorted by ID (`compile_session`'s sort and `CanonicalOrd`), and each
    `members` list sorted by ID, in the canonical writer and in normalization.
  - **The visitor gains one method**, `fn id_item(&mut self, value: &StableId)`: one stable ID as an
    element of the open array. `JsonWriter` writes it as an array item (`"kick"` on its own line,
    the indentation of any array item); `StringBytes` counts it; the test `Trace` records it. The
    walk visits `members` as `array_begin(members, n)`, `n` x `id_item`, `array_end`.
  - The parser gains an ID-list reader for `members`: a non-array is `schema.wrong_type` at
    `$.vcas[<i>].members`, a non-string item `schema.wrong_type` and a malformed ID `id.invalid` at
    `$.vcas[<i>].members[<j>]`.
- **D2. Validation** (index paths, as session validation reports them).
  - `GraphEntity` gains `Vca(&Vca)`; VCA IDs join the namespace, so a duplicate across tracks,
    submixes, outputs and VCAs is `id.duplicate` at the later entity's `id` (for a VCA,
    `$.vcas[<i>].id`).
  - A VCA ID anywhere a strip or an output is required -- a route source or destination, a
    sidechain source, an automation target's `entity_id` -- is `reference.missing_entity` at that
    path, exactly as an unknown ID is.
  - Every member names a track, a submix or a VCA, else `reference.missing_entity` at
    `$.vcas[<i>].members[<j>]`. An output is not a member (`reference.missing_entity`).
  - A member repeated within one VCA is `id.duplicate` at the later occurrence,
    `$.vcas[<i>].members[<j>]`. One strip in several VCAs is legal.
  - **Acyclic.** A VCA that lies on a membership cycle among VCAs (a self-member included) is
    refused with a new code, `vca.cycle` (variant `DiagnosticCode::VcaCycle`, appended after
    `ConsoleSlotNotNative`), at `$.vcas[<i>]`, one diagnostic per VCA on a cycle, in ascending
    declared index. Cycle detection runs over resolvable VCA-to-VCA edges only. A diamond (one VCA
    reachable from another along two paths) is legal.
  - Each VCA fader dB is checked with `validate_finite_range(.., -144.0, 24.0, ..)`:
    `numeric.out_of_schema_range` when finite and outside, `numeric.non_finite` otherwise.
- **D3. SDK.**
  - `SessionBuilder.vca(id, spec)`, `spec: VcaSpec = { fader?: FaderSpec, members: readonly
    string[] }`, exported from `sdk/src/core/types.ts`. `fader` defaults to 0 dB unmuted on both
    lanes.
  - It refuses, with `MisoUsageError` carrying the engine's code: an ID that is not a stable ID
    (`id.invalid`); an ID already in the namespace (`id.duplicate`); a member that is not an
    already-declared track, submix or VCA (`reference.missing_entity`; so a builder can never make
    a cycle); a repeated member (`id.duplicate`); a fader dB outside `[-144, 24]`
    (`numeric.out_of_schema_range`, added to `CODE`).
  - `#graphIds` includes VCA IDs, so `track`, `submix` and `output` refuse a VCA's ID too.
  - The writer emits `vcas` after `submixes`, sorted by ID, each `["id", "fader", "members"]`, with
    `members` sorted.
  - The `enginectl` request accepts an optional `vcas` array of `{ id, fader?, members }`, read into
    the builder after the tracks and submixes, in an order that declares every member VCA before a
    VCA that lists it (the builder refuses a forward reference); `sdk/README.md` lists the request
    key and that rule.
- **D4. Inert until #1242.** Preparation ignores `vcas` in this slice. That is legal only inside the
  VCA batch.

## Deliverables

1. Session: the model (`Vca`, `SessionModel.vcas`), parse (D1), validate (D2), the visitor method
   and the walk (root registry `VCAS` 16, the root record count `+ s.vcas.len()`, the order, a `vca`
   key module), the estimate (`vcas` vector bytes, member vector bytes, VCA entities counted beside
   submixes; no allocation), `compile_session`'s sorts (D1), and the new diagnostic code.
2. SDK: D3, with a writer-parity eval, builder-refusal evals and an `enginectl` case.
3. Migration: `"vcas": []` after `submixes` in every document and inline session in the Context
   (and every one the re-run greps list); the derived fixtures by their three commands; the writer
   corpus regenerated after its corpus model (`full_surface_document`, `canonical.rs:245-`) gains a non-trivial VCA
   forest (nested, overlapping, one empty `members`). Re-pin the `canonical.json` chain,
   `expected.json`'s `sessionDocumentBytes` with its self-test row, and the capi snapshot vector,
   each with the reason "root key `vcas` added, empty". No render digest moves.
4. Docs:
   - `docs/SESSION_SCHEMA_V1.md`: `vcas` in the root key sentence (`:35-37`), a VCA paragraph
     (grammar, namespace, acyclicity, range; VCAs are inert until #1242, which rewrites the
     sentence), and a registry note that root field 8 is retired and never reused and 16 is `vcas`;
   - `docs/session-v1.schema.json`: the `vcas` property and its `required` entry;
   - `.claude/skills/author-session/SKILL.md`: one paragraph on `vcas`, and `vca.cycle` in a small
     "VCA refusals, by code" table of its own (the existing table, `:169-181`, is "Console
     refusals, by code"); the skill's validator lines must keep running (gate 5).

## Authorized paths

- `crates/session/**`
- `crates/protocol/tests/{console_session_edits,submix_strip_edits}.rs` (the inline sessions only)
- `crates/capi/src/runtime/tests.rs` (`ALL_COMMAND_RESPONSE_VECTORS[1]` and its comment only)
- `sdk/src/core/{session.ts,types.ts}`, `sdk/src/internal/session-json.ts`,
  `sdk/src/cli/session-request.ts`, `sdk/README.md` (the `enginectl` request keys)
- `sdk/test/{builder-evals,console-evals,enginectl-cli,support}.mjs`
- every session document listed in the Context, and every file the Context's re-run greps list (for
  the added key only)
- `fixtures/session-canonical/v1/canonical-writer-corpus.json` (regenerated)
- `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `tools/audit/src/{fixture_builtins.rs,builtins_graph.rs}` (the
  pins only), `fixtures/graph/v1/`, `fixtures/graph/MANIFEST.tsv` (only if `--check` fails)
- `tools/parameter-metadata/tests/{abi_layout.rs,round_trip.rs}` (the inline sessions only)
- `hosts/host-web/tests/browser-v1/expected.json` (`sessionDocumentBytes` only),
  `scripts/check-browser-expected-resources.py` (the `:572` self-test row only)
- `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`,
  `.claude/skills/author-session/{SKILL.md,worked-session.json}`
- this spec

## Non-goals

- No wire message and no edit opcode (#1241): a VCA is changed only by replacing the session.
- No effect on rendering (#1242), no caps (#1243), no live controls (#1244-#1246) and no C ABI
  value-only path (#1247).
- No VCA solo, no send trim and no automation target: an automation target naming a VCA refuses.

## Hazards

- **ID 8.** The root field gap must not be reused. The field-key test asserts root ID 8 is
  unallocated and `vcas` is 16.
- **Migration discipline.** The adversarial review diffs every migrated document for anything other
  than the added key.
- **Shared namespace.** A VCA ID colliding with a track ID must refuse, not shadow; a VCA ID must
  never satisfy a lookup that wants a strip or an output.
- **Diamonds are not cycles.** Only a membership cycle refuses.
- **The estimate may not allocate** (`check-session-policy.sh:32`): count members by iteration.
- **One string array.** The Rust writer and the SDK writer must spell `members` identically; the
  writer-parity eval, and `sdk/test/builder-evals.mjs`'s check that the Rust-generated writer corpus
  matches the SDK writer (`:995`), pin it.
- **The iOS memset rule.** `check-cross-targets.sh` (`:121-139`) counts `bl _memset_pattern16` per
  product crate against `scripts/lib/aarch64-known-defects.py`; `session` has no row, so one call
  fails it. New code stores no splatted non-zero constant to memory.

## Objective gates

1. **Grammar and canonical form.** New `crates/session/tests/vca.rs`:
   - a document without `vcas` refuses with `schema.missing_field` at `$`; a non-array `members`, a
     non-string member and a malformed member ID refuse at their D1 paths;
   - canonical text round-trips exactly (parse, write, parse, equal model; write, parse, write, equal
     text) for random VCA forests (nested, overlapping, empty `members`, up to depth 4 and 16
     members, 32 seeds), with `vcas` and `members` written sorted whatever the declared order;
   - `visit_model.rs` asserts root field 8 is unallocated and `vcas` is 16, and that the walk visits
     `members` as `id_item`s inside its array.

   *Test value: it turns red if the root key is optional, lost or misplaced by the canonical writer,
   if `members` is not canonicalised or spelled as records, or if field 8 is reused.*
2. **Refusals at their paths.** In `crates/session/tests/vca.rs`: `vca.cycle` for a self-member and
   for a three-VCA cycle (each VCA on it reported once, ascending index); a missing member; an output
   as a member; a duplicate ID across the namespace; a repeated member; a VCA ID as a route source,
   a route destination, a sidechain source and an automation target; VCA fader values of 24.5 and
   -144.5 (`numeric.out_of_schema_range`); each with its code at its index path. 24.0 and -144.0 are
   accepted, and a diamond is accepted.
   *Test value: it turns red if a cyclic, dangling, out-of-range or misused VCA reaches preparation,
   or if a legal diamond is refused as a cycle.*
3. **The migration moved no render and no wire byte.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh` (and `target/issue6/fresh-process-determinism.json`
     identical to the base's)
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `./target/release/audit capi`
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-protocol-wasm-parity.sh` (`COMPLETE_SCHEMA_HASH` unchanged)

   Each passes with only the listed re-pins, and every `output_sha256` is unchanged.
4. **SDK and browser** (`npm ci` in `sdk/` first; `<A>` and `<B>` are fresh empty directories):
   - `bash scripts/check-sdk-types.sh`
   - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --self-test` and
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (the migrated
     `session.json` size)
   - `bash scripts/check-sdk-headless.sh <A>`, including, in `sdk/test/console-evals.mjs`, a
     writer-parity eval of a session with nested VCAs declared out of ID order, against
     `engineCanonical()`, and in `sdk/test/builder-evals.mjs` the five D3 refusals and a default
     `fader`.
     *Test value (writer parity): it turns red if the SDK writer orders `vcas`, a VCA's keys or
     `members` differently from the engine, or drops `members`.*
     *Test value (builder refusals): it turns red if the builder emits a VCA the engine refuses, or
     refuses with a code the engine would not use.*
   - `bash scripts/sdk-package.sh check <A>` (runs `sdk/test/enginectl-cli.mjs`, with a `vcas`
     request whose VCAs nest and are listed parent first).
     *Test value (`enginectl`): it turns red if the CLI drops `vcas`, or reads them before the
     tracks and submixes or in an order the builder refuses.*
   - The browser legs, because the browser fixtures' session documents change: in
     `hosts/host-web/qualification`, `npm ci`, `npx playwright install <browser>`, then
     `npm run qualify -- --artifacts <abs A> --sdk-root <abs sdk> --browser <browser> --check-matrix --self-test-mutations`
     for `chromium`, `firefox` and `webkit`, under a private PulseAudio null sink exactly as the
     `browser` job of `.github/workflows/qualification.yml` (`:362-430`) sets it up.
5. **Workspace and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
     (CI's `test-debug-a`; it runs `tools/session-validator/tests/skill.rs`)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in session protocol-control workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh` (`session` is a product crate)
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the
     batch push (recorded "at batch push").

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- The re-pin list, each with its reason, and the old and new values.
- A diff summary showing every migrated document changed only by the added key.

## Dependencies

- *VCA groups* (#1239) filed.
- *Let a send follow its source strip's mute live in the browser* (#1224; batch K3 of #1196
  delivered).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: append the root field and the code; never renumber or reuse (never
  field 8).
- "Unchanged" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
