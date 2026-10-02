# Declare VCA groups in the session

Slice V1 of *VCA groups* (the umbrella the root files when batch K3 of *Submix strips and live aux
sends* closes). It is in the VCA batch, pushed once with V2, V3 and V4. Implementation-ready: anchors
were read on `fe8ac679`, and the Context states what batches K1-K3 changed in them. Line numbers in
files K1-K3 edited may have drifted by a few lines; the named functions and literals have not.

## Product outcome

A session document can declare VCA groups: `vcas: [{ id, fader, members }]`.

- They are parsed, validated, canonicalised and carried on the wire.
- They are editable through session transactions.
- They are authored from the SDK and the `enginectl` CLI.

This slice is the grammar and the migration. *Apply VCA offsets and mutes at preparation* makes them
audible; between the two, inside the VCA batch only, VCAs parse and are inert.

## Context (verified on `fe8ac679`, with the K1-K3 changes stated)

- **Root keys are all required** (V1 has no optional fields; `docs/SESSION_SCHEMA_V1.md:147-149`).
  - The root field registry is `crates/session/src/visit.rs:85`: `schema_version` 1, `session_id` 2,
    `revision` 3, `sample_rate_hz` 4, `quantum_frames` 5, `render_profile` 6, `output_profile` 7,
    `sources` 9, `tracks` 10, `submixes` 11, `outputs` 12, `routes` 13, `automation` 14, `console` 15.
  - **ID 8 is an unrecorded gap.** It was `limits`, removed in `04d291dd`, with no retirement note.
    It must never be reused.
  - **16 is free** after K3 (K1-K3 add no root key). #1054's draft claims no field ID; its optional
    camelCase `controlSmoothing` object contradicts the schema (`DESIGN.md` R5). Take the next
    unallocated ID at implementation, never 8; check `visit.rs:85` first.
  - The canonical root walk is the `SessionModel=>session` record at `visit.rs:194-198`: its record
    count is `8 + sources + tracks + submixes + outputs + routes + automation`, and its order is
    `schema_version, session_id, revision, sample_rate_hz, quantum_frames, render_profile,
    output_profile, sources, console, tracks, submixes, outputs, routes, automation`.
  - The JSON parser's root key list is `crates/session/src/parse.rs:729` area (`"submixes"`), and the
    submix list is parsed at `:746`.
- **The namespace.** Tracks, submixes and outputs share one graph-entity namespace, built in
  `crates/session/src/validate.rs:70-100` over `enum GraphEntity` (`:11`). After K1, `GraphEntity`'s
  submix variant may carry the submix; read it before adding a variant.
- **Fader.** `DualMonoFader { left_db, right_db, left_mute, right_mute }`
  (`crates/session/src/model.rs:537-546`).
  - The session crate checks a track's `fader_db` for **finiteness only** (`validate.rs:349-356`,
    `validate_finite`). After K1 the same strip check runs for submixes.
  - The `[-144, 24]` range is enforced by builtins at preparation: `checked_fader_gain`
    (`crates/builtins/src/lib.rs:4108`) and `prepare_sections` (`:3131`).
  - A VCA's range check is therefore new session validation, with `validate_finite_range`
    (`validate.rs:830-849`), which refuses `numeric.out_of_schema_range`.
- **Protocol.**
  - The wire carries session **edits**, not a session root message. A submix is the message
    `schema::session::submix` (`crates/protocol/src/schema.rs:1011`) carried by `upsert_submix`
    (`:1361-1367`) and `remove_submix` (`:1369-1375`); the dispatch table is at `:1513-1514`; the codec
    is `tx_submix` (`crates/protocol/src/session_wire.rs:991`), the edit encode arms at `:552-555` and
    the decode arms at `:1289-1294`.
  - The fader message is `schema::session::fader` (`schema.rs:939-948`). A repeated UTF-8 field has
    one precedent: `set_track_effect_order::EFFECT_ID` (`schema.rs:1244-1250`), a hand-built
    `FieldSpec` with `repeated: true`.
  - Opcodes: the enum is `crates/protocol/src/model.rs:20-106`, `from_raw` is `:117`, and
    `apply_session_edit` is `:480` (the submix arms are around `:675-680`). Families run up to
    automation, `0600`-`0603` (`docs/CONTROL_PROTOCOL_REGISTRY.md:72`). `07xx` is free and was never
    used.
  - **The opcode count is 43 after K3** (41 at `fe8ac679`; *Mute a route in the session* adds `0506`
    and *Let a route into a submix follow its source strip's mute in the session* adds `0507`). It is
    pinned at:
    - `crates/protocol/src/model.rs:1315`;
    - `crates/protocol/src/controller/tests.rs:326`, with the transaction-edit limit at `:327`, kept
      at one below the fixture's count;
    - `crates/conformance/src/protocol_corpus.rs:664` (doc comment);
    - `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`;
    - `docs/CONTROL_PROTOCOL_REGISTRY.md:74`;
    - `fuzz/corpus/complete-schema-manifest.md:4`, `:19`.
  - `complete_all_opcode_fixture` (`crates/conformance/src/protocol_corpus.rs:16`) holds exactly one
    edit per allocated opcode, so three new opcodes add three edits and re-pin
    `COMPLETE_SCHEMA_HASH`, spelled at `protocol_corpus.rs:666`,
    `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and
    `fuzz/corpus/complete-schema-manifest.md:10`. The corpus frame count (46,
    `crates/conformance/src/main.rs:31`, `crates/conformance/tests/conformance_corpus.rs:14`, the
    parity script `:174`) re-pins only if it moves (one transaction carries every edit, so it is not
    expected to).
  - Edit errors map to strings in `crates/protocol/src/controller.rs:3509-3517`.
- **SDK.**
  - The writer: `ROOT_KEYS` (`sdk/src/internal/session-json.ts:13`), `OBJECT_KEY_ORDERS` (`:67-88`;
    `fader` is `:78`), and the numeric leaf lists (`INTEGER_KEYS` `:37`, `FLOAT_KEYS` `:49`); an
    unlisted numeric leaf throws.
  - The builder (`sdk/src/core/session.ts`): `submix(id, spec?)` (`:726` at `fe8ac679`; *Build submix
    strips and bus taps in the SDK and teach agents to author them* widened it), the model record
    type (`:100`), the empty state (`:897`) and the emitter (`:1475`).
  - The `enginectl` CLI accepts root keys at `sdk/src/cli/session-request.ts:418`.
  - The writer corpus is asserted by `sdk/test/builder-evals.mjs:818-830`; `sdk/test/enginectl-cli.mjs`
    builds sessions with `submixes` at `:61`, `:463`.
- **Session documents that need `"vcas": []`** (`git grep -l '"submixes"' -- '*.json' ':!artifacts/*'`
  lists 22 files: these 21 sessions plus `docs/session-v1.schema.json`):
  - `.claude/skills/author-session/worked-session.json`;
  - `crates/graph-compiler/tests/data/reduced-nobus-from-970-verify.json`;
  - `fixtures/session/v1/`: `builtins-automation`, `canonical-minimal`, `canonical`,
    `compressor-bank-observation`, `compressor-dynamic-bank-observation`,
    `compressor-dynamic-observation`, `console-sixty-four-track`, `console-sixty-four-track-app`,
    `console-sixty-four-track-intended`, `console-sixty-four-track-mono`, `observation-frame-shape`,
    `parametric-eq-bank-console`, `parametric-eq-nine-track` (`.json`);
  - `hosts/host-web/qualification/{live-control,observation,stall}-session.json`;
  - `hosts/host-web/tests/browser-v1/{command-session,observation-session,session}.json`;
  - `fixtures/session/v1/console-sixty-four-track-sends.json`, **if** *Add a bus-and-send row to the
    native console benchmark* (BM1) has landed;
  - the embedded writer-corpus document `fixtures/session-canonical/v1/canonical-writer-corpus.json`,
    regenerated by `crates/session/src/canonical.rs` with
    `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session canonical_writer_corpus_is_rust_generated_and_current`
    (`canonical.rs:337`, `:406-412`).

  Historical records under `artifacts/` are evidence and are not migrated.
- **Inline sessions** (`git grep -n '"submixes"'` and `submixes:` outside JSON):
  - `sdk/test/support.mjs:96` (a JS object literal `submixes: [], outputs: [...]`, used by every SDK
    eval that boots the shipped module);
  - `tools/parameter-metadata/tests/abi_layout.rs:166` and `tools/parameter-metadata/tests/round_trip.rs:293`,
    `:699` (raw-string sessions; the inline sessions are in `tests/`, not `src/`);
  - `crates/protocol/tests/console_session_edits.rs:60`.

  These lists are `fe8ac679`'s (VERIFY-3 N-G). Re-run `git grep -l '"submixes"'` and
  `git grep -n 'submixes:'` at the VCA base before editing: batches K1-K3 add sessions in host-core,
  host-web, capi and SDK tests.
- **Derived fixtures.** `console-sixty-four-track-{intended,mono,app}.json` (and `-sends.json` if BM1
  landed) are regenerated by `scripts/derive-*-console-fixture.py` from their parents
  (`intended` from `console-sixty-four-track.json`; `mono` and `app` from `intended`; `sends` from
  `intended`) and held by `scripts/check-console-fixtures.sh`. Migrate each parent, then regenerate.
- **Pinned bytes that move.**
  - The SHA-256 chain over `fixtures/session/v1/canonical.json`:
    `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml:12`;
    `tools/audit/src/fixture_builtins.rs:5102`; `fixtures/builtins/v1/MANIFEST.tsv:10-11`; the MANIFEST
    digest at `tools/audit/src/builtins_graph.rs:52` and `fixture_builtins.rs:5275`. These are
    tested only by `cargo test --locked --release -p audit -p bench -p console-workload`
    (`.github/workflows/qualification.yml:710`).
  - `fixtures/graph/v1/direct-route.*` are generated from `canonical.json`
    (`crates/graph-compiler/src/bin/graph_fixture.rs`) and pinned in `fixtures/graph/MANIFEST.tsv`;
    regenerate with `cargo run --locked -p graph-compiler --bin graph_fixture -- --write`.
  - The browser's staged document size `"sessionDocumentBytes"` in
    `hosts/host-web/tests/browser-v1/expected.json:57` (1905 at `fe8ac679`; slices 18b and 20 moved it)
    grows with `session.json`, and the self-test row of `scripts/check-browser-expected-resources.py`
    (`:569-573`, "the staged document size moves by one byte") prints that value plus one.

## Decisions frozen for this slice

- **D1. Schema.** Root key `vcas`, required (`[]` when empty), at the next unallocated root field ID
  (16 if still free; never 8). Its canonical position is after `submixes`.
  - Each VCA is `{ id, fader, members }`: `fader` is a `DualMonoFader` read as an offset; `members`
    is an array of IDs, possibly empty.
  - Canonical form: `vcas` sorted by ID, and each `members` sorted by ID.
- **D2. Wire.** A new `schema::session::vca` message: `id` 1 (UTF-8), `fader` 2 (the fader message),
  `members` 3 (repeated UTF-8, on the `set_track_effect_order::EFFECT_ID` precedent). An empty
  `members` round-trips exactly; follow the codec's existing rule for an empty repeated field.
- **D3. Validation** (index paths, as session validation reports them).
  - VCA IDs join the graph-entity namespace, so a duplicate across tracks, submixes, outputs and VCAs
    is `id.duplicate` at `$.vcas[<i>].id`.
  - Every member names a track, a submix or a VCA, else `reference.missing_entity` at
    `$.vcas[<i>].members[<j>]`. An output is not a member (`reference.missing_entity`).
  - A member repeated within one VCA is `id.duplicate` at `$.vcas[<i>].members[<j>]`.
  - Membership is acyclic. A cycle refuses with the new code `vca.cycle` (a new `DiagnosticCode`
    variant appended at the end, `crates/session/src/diagnostic.rs:17-74`), at `$.vcas[<i>]` for the
    lowest-index VCA on the cycle. A self-member is a cycle.
  - Each VCA fader dB is in `[-144, 24]`, else `numeric.out_of_schema_range`, through
    `validate_finite_range`; a non-finite value keeps `numeric.non_finite`.
- **D4. Opcodes**, appended to a new `07xx` family:
  - `0700` `UpsertVca { vca }`;
  - `0701` `RemoveVca { vca_id }`;
  - `0702` `SetVcaFader { vca_id, fader }`.

  An unknown VCA ID is `session.edit.not_found`. Removing a VCA that another VCA lists as a member
  fails final validation (`reference.missing_entity`) unless the same transaction removes the
  reference. The count rises from 43 to 46 at every pin; the controller test's edit limit moves to
  45.
- **D5. SDK.** `vca(id, { fader?, members })`: `fader` defaults to 0 dB unmuted on both lanes;
  every member must already be declared (track, submix or an earlier VCA), else `MisoUsageError` with
  the engine's code. The writer emits `vcas` after `submixes`, with key order `["id", "fader",
  "members"]`. The `enginectl` CLI accepts an optional `vcas` request array of
  `{ id, fader?, members }`.
- **D6. Inert until V2.** Preparation ignores `vcas` in this slice. That is legal only inside the VCA
  batch; the batch pushes after V4.

## Deliverables

1. Session: model, parse, validate (D3), canonical writer and field keys (`visit.rs`: root registry,
   the root walk's order and record count, a `vca` key module), the estimate (`estimate.rs`, beside
   the submix count at `:60` and `:132`), and `compile_session`'s sort (D1).
2. Protocol: the `vca` message, `upsert_vca`, `remove_vca` and `set_vca_fader` specs and dispatch
   rows, the codec, the three opcodes with `from_raw` and `apply_session_edit` arms, every count pin,
   three edits in `complete_all_opcode_fixture` and the `COMPLETE_SCHEMA_HASH` re-pin (D2, D4).
3. SDK: D5, plus writer-parity and builder-refusal evals.
4. Migration: `"vcas": []` in every session document and inline session listed in the Context; the
   derived fixtures through their derive scripts; the writer corpus regenerated after giving
   `canonical.rs`'s corpus model a non-trivial nested VCA forest. Re-pin the `canonical.json` chain,
   the graph-fixture manifest and `expected.json`'s document size, each with the reason "root key
   `vcas` added, empty". No render digest moves.
5. Docs:
   - `docs/SESSION_SCHEMA_V1.md`: a VCA section, and a root field registry note that 8 is retired
     and never reused;
   - `docs/session-v1.schema.json`: the `vcas` root property;
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: `0700`-`0702`, the `07xx` family row, the root field and
     the count 46;
   - `docs/CONTROL_PROTOCOL_CONFORMANCE.md`: the count and the hash;
   - `.claude/skills/author-session/SKILL.md`: one paragraph on `vcas` (the skill's validator lines
     must keep running, gate 6).

## Authorized paths

- `crates/session/**`
- `crates/protocol/src/{schema.rs,session_wire.rs,session_wire/tests.rs,model.rs,controller.rs}`,
  `crates/protocol/src/controller/tests.rs`, `crates/protocol/tests/`
- `crates/conformance/src/{protocol_corpus.rs,main.rs}`, `crates/conformance/tests/conformance_corpus.rs`
- `fuzz/corpus/complete-schema-manifest.md`, `scripts/check-protocol-wasm-parity.sh` (the pinned hash
  and counts only)
- `sdk/src/core/{session.ts,types.ts}`, `sdk/src/internal/session-json.ts`,
  `sdk/src/cli/session-request.ts`, `sdk/README.md` (the `enginectl` request keys)
- `sdk/test/{builder-evals,console-evals,enginectl-cli}.mjs`, `sdk/test/support.mjs`
- every session document listed in the Context, and every file the two Context greps list at the
  VCA base (for the added key only); `scripts/derive-*-console-fixture.py` only if one
  hard-codes the root key list
- `fixtures/session-canonical/v1/canonical-writer-corpus.json` (regenerated)
- `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `tools/audit/src/{fixture_builtins.rs,builtins_graph.rs}` (the
  pins only), `fixtures/graph/v1/`, `fixtures/graph/MANIFEST.tsv`
- `tools/parameter-metadata/tests/{abi_layout.rs,round_trip.rs}` (the inline sessions only)
- `hosts/host-web/tests/browser-v1/expected.json` (`sessionDocumentBytes` only),
  `scripts/check-browser-expected-resources.py` (the document-size self-test row only)
- `docs/SESSION_SCHEMA_V1.md`, `docs/session-v1.schema.json`, `docs/CONTROL_PROTOCOL_REGISTRY.md`,
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`, `.claude/skills/author-session/SKILL.md`
- this spec

## Non-goals

- No effect on rendering (V2).
- No live controls (V3, V4) and no C ABI value-only path (V5).
- No VCA solo, no send trim and no automation target.

## Hazards

- **ID 8.** The root field ID gap must not be reused. Assert it in the field-key test.
- **Migration discipline.** The adversarial review diffs every migrated document for anything other
  than the added key.
- **Shared namespace.** A VCA ID colliding with a track ID must refuse, not shadow.
- **Diamonds are not cycles.** A VCA reachable from another along two paths is legal; only a
  membership cycle refuses.
- **Numeric leaves.** The SDK writer throws on an unlisted numeric leaf; the VCA fader's keys are the
  track fader's, which are listed.

## Objective gates

1. **Grammar and wire.** New tests in `crates/session/tests/` and
   `crates/protocol/src/session_wire/tests.rs`:
   - `vcas` is required: a document without it refuses at `$` with `schema.missing_field`;
   - canonical text and BTLV round-trip exactly for random VCA forests (nested, overlapping, empty
     `members`);
   - `0700`, `0701` and `0702` edit the committed snapshot as expected;
   - the opcode count reads 46 at every pin, and the field-key test asserts root field 8 is
     unallocated.

   *Test value: it turns red if the root key is optional, lost or reordered by the codec or the
   canonical writer, if an opcode is unregistered, or if field 8 is reused.*
2. **Refusals at their paths.** `vca.cycle` (a self-member and a three-VCA cycle), a missing member,
   an output as a member, a duplicate ID across the namespace, a repeated member, and VCA fader values
   of 24.5 and -144.5 each refuse with their code at their index path; 24.0 and -144.0 are accepted,
   and a diamond is accepted.
   *Test value: it turns red if a cyclic, dangling or out-of-range VCA reaches preparation, or if a
   legal diamond is refused as a cycle.*
3. **The migration moved no render.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `cargo test --locked --release -p audit -p bench -p console-workload` (the pin chain in
     `tools/audit`)

   Each passes with only the listed re-pins, and every `output_sha256` is unchanged.
4. **Wire and conformance.**
   - the DSP-crates-and-conformance command (`DESIGN.md` section 7)
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-protocol-wasm-parity.sh`
5. **SDK and browser.**
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` (the migrated
     `session.json` size)
   - `bash scripts/check-sdk-headless.sh <A>`, including a writer-parity eval in `console-evals.mjs` of a session with
     nested VCAs and a builder refusal of an undeclared member.
     *Test value (writer parity): it turns red if the SDK writer orders `vcas` or a VCA's keys
     differently from the engine, or drops `members`.*
     *Test value (builder refusal): it turns red if the builder emits a VCA the engine refuses.*
   - `bash scripts/sdk-package.sh check <A>` (runs `enginectl-cli.mjs`, with a `vcas` request).
6. **Workspace and policy.**
   - the workspace test command (`DESIGN.md` section 7), which includes
     `tools/session-validator/tests/skill.rs`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-session-policy.sh` and `bash scripts/test-session-policy.sh`
   - `bash scripts/check-protocol-control-policy.sh` and `bash scripts/test-protocol-control-policy.sh`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The re-pin list, with reasons.
- A diff summary showing every migrated document changed only by the added key.

## Dependencies

- *VCA groups* (umbrella) filed
- *Let a send follow its source strip's mute live in the browser* (batch K3 of *Submix strips and
  live aux sends* closed and pushed)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: append the root field, the message, the code and the opcodes; never renumber
  or reuse (never field 8).
- "Unchanged" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
