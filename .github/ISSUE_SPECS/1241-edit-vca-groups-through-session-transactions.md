# Edit VCA groups through session transactions

Slice V2 of *VCA groups* (#1239). It is in the VCA batch, after *Declare VCA groups in the session*
(#1240). It is the wire half of the VCA grammar, split from #1240 at filing on the precedent of
*Declare the submix strip in the session grammar and wire* (#1199) and *Address submix strips in
session edits* (#1204): a VCA becomes editable through the control protocol's session transactions,
`0700`-`0702`, in a new `07xx` family. Read #1216's and #1218's attempt records (git history) for
the count and hash re-pin chain this slice repeats.

The design record cited below (`DESIGN` 5.10, 5.11) is committed in
`docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

An agent edits VCAs with session transactions, like every other session entity:

- `0700` upserts a VCA (its fader and its members), `0701` removes one, and `0702` sets a VCA's
  fader;
- each edit is validated with the whole transaction (namespace, references, acyclicity, range), and
  the committed snapshot carries the result;
- the strip edits never reach a VCA.

## Context (verified on `8c6268967`)

- **After #1240:** `session::Vca { id: StableId, fader: DualMonoFader, members: Vec<StableId> }`,
  `SessionModel.vcas`, the `vca` key module (`id` 1, `fader` 2, `members` 3), and session validation
  of VCAs (`vca.cycle`, `reference.missing_entity`, `id.duplicate`, `numeric.out_of_schema_range`).
- **The wire carries session edits**; the whole session travels as canonical JSON. The submix
  message is `schema::session::submix` (`crates/protocol/src/schema.rs:1011-1026`), carried by
  `upsert_submix` (`:1380-1387`) and `remove_submix` (`:1388-1395`); the fader message is
  `schema::session::fader` (`:939-948`); `set_track_fader` (`:1353-1360`) is the template for a
  set-fader edit. The dispatch table is `payload_spec` (`:1523-1567`).
- **A repeated UTF-8 field** has one precedent: `set_track_effect_order::EFFECT_ID`
  (`schema.rs:1259-1273`), a hand-built `FieldSpec { id: 3, wire: Wire::Utf8, mandatory: true,
  repeated: true, nested: None }`. A repeated field may occur zero times
  (`MessageSpec::field_count`, `:216-254`; the submix's repeated `CONSOLE` decodes empty). Its codec
  is `crates/protocol/src/session_wire.rs:342-344` (count), `:444-457` (encode) and `:1238-1246`
  (decode, `values_spec!` then `stable_id`).
- **The codec**: `tx_submix` (`session_wire.rs:1003`), `parse_submix` (`:1709`); the edit encode
  arms `:552-555` (the last at `:588`), the edit decode arms `:1328-1333` (the last at `:1370`).
- **Opcodes** (`crates/protocol/src/model.rs`): `SessionEditOpcode` (`:22-111`), families `00xx`
  root to `06xx` automation (`0600`-`0603` last); **`07xx` is free and was never used**. `from_raw`
  (`:122-170`), `enum SessionEdit` (`:176-415`), `SessionEdit::opcode` (`:421-470`),
  `apply_session_edit` (`:503-762`; the submix arms at `:698-704`). Edit errors map to strings in
  `crates/protocol/src/controller.rs:3509-3517` (`session.edit.not_found`).
- **Strip edits.** `0203`-`0211` resolve a strip ID tracks first, then submixes (`strip_mut`,
  `model.rs:997-1024`); any other ID is `session.edit.not_found`. `SetTrackFader` is `0x020f`.
- **The opcode count is 43.** It is pinned at: `crates/protocol/src/model.rs:1422` (with the doc
  comment `:1410-1413` and the registry test's code assertions `:1423-1440`);
  `crates/protocol/src/controller/tests.rs:327`, with the transaction-edit limit at `:328` kept one
  below the fixture's count (42); `crates/conformance/src/protocol_corpus.rs:707` (doc comment);
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`; `docs/CONTROL_PROTOCOL_REGISTRY.md:81` (and the edit
  table, whose last row is `0600`-`0603` at `:72`); `fuzz/corpus/complete-schema-manifest.md:4` and its re-pin history `:24-26`.
- **The corpus.** `complete_all_opcode_fixture` (`crates/conformance/src/protocol_corpus.rs:16`)
  holds exactly one edit per allocated opcode, in opcode order; its last edit is
  `SetAutomationSegments` (`:255-258`). Three new opcodes add three edits after it and re-pin
  `COMPLETE_SCHEMA_HASH` (`0x95c1_ceb6_8e44_f6e2`), spelled at `protocol_corpus.rs:708`,
  `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` and
  `fuzz/corpus/complete-schema-manifest.md:10`. The frame count 46 (`crates/conformance/src/main.rs:31`,
  `crates/conformance/tests/conformance_corpus.rs:14`, the parity script `:174`) does not move: the
  edits ride the one transaction frame.
- **Existing edit tests** to extend: `crates/protocol/src/session_wire/tests.rs` (round trips),
  `crates/protocol/tests/console_session_edits.rs` and `crates/protocol/tests/submix_strip_edits.rs`
  (transactions against a committed snapshot).

## Decisions frozen for this slice

- **D1. Wire.** A new message `schema::session::vca`: `ID` 1 (`Wire::Utf8`, required), `FADER` 2
  (the `fader` message, required), `MEMBERS` 3 (repeated UTF-8, built exactly as
  `set_track_effect_order::EFFECT_ID`). An empty `members` encodes zero occurrences and decodes to
  `[]`; each member decodes through `stable_id`.
- **D2. Edits**, in a new `07xx` family, appended:
  - `0700` `UpsertVca { vca }` (message `upsert_vca`, `VALUE` 1): a plain upsert by ID, as
    `UpsertSubmix`.
  - `0701` `RemoveVca { vca_id }` (message `remove_vca`, `ID` 1): an unknown ID is
    `session.edit.not_found`.
  - `0702` `SetVcaFader { vca_id, fader }` (message `set_vca_fader`, `VCA_ID` 1, `VALUE` 2 the fader
    message): replaces the VCA's fader; an unknown ID is `session.edit.not_found`.
  - Final validation applies, as for every edit: removing a VCA, track or submix that a VCA lists as
    a member fails with `reference.missing_entity` unless the same transaction rewrites that VCA
    (`0700`). The strip edits `0203`-`0211` never resolve a VCA ID (`session.edit.not_found`).
  - The count rises from 43 to 46 at every pin; the controller's edit limit moves to 45.

## Deliverables

1. The `vca` message, the `upsert_vca`, `remove_vca` and `set_vca_fader` specs and their dispatch
   rows, the codec (`tx_vca`, `parse_vca`, the three encode and decode arms).
2. The three opcodes: enum, `from_raw`, `SessionEdit` variants, `opcode()`, `apply_session_edit`
   arms; the registry test asserts `0x0700`-`0x0702`.
3. Every count pin and the edit limit; three edits after `SetAutomationSegments` in
   `complete_all_opcode_fixture` (one VCA with two members, one removal, one fader); the
   `COMPLETE_SCHEMA_HASH` re-pin at its four sites, reason "VCA message, opcodes `0700`-`0702`".
4. Docs: `docs/CONTROL_PROTOCOL_REGISTRY.md` (a `0700`-`0702` family row, the count 46 and its
   history, and that root field 16 is `vcas`); `docs/CONTROL_PROTOCOL_CONFORMANCE.md` and
   `fuzz/corpus/complete-schema-manifest.md` (the count, the hash and a re-pin sentence).

## Authorized paths

- `crates/protocol/src/{schema.rs,session_wire.rs,session_wire/tests.rs,model.rs,controller.rs}`,
  `crates/protocol/src/controller/tests.rs`, `crates/protocol/tests/`
- `crates/conformance/src/{protocol_corpus.rs,main.rs}`, `crates/conformance/tests/conformance_corpus.rs`
- `fuzz/corpus/complete-schema-manifest.md`, `scripts/check-protocol-wasm-parity.sh` (the hash
  literals only)
- `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md`
- this spec

## Non-goals

- No change to the session grammar or validation (#1240), to preparation (#1242) or to live controls.
- No C ABI value-only path: on the C ABI every VCA edit is structural until #1247.
- No new diagnostic code, and no SDK encoder (the SDK writes whole sessions).

## Hazards

- **Never renumber or reuse.** The `07xx` family is new; retired codes `0006`, `0102`, `0104` stay
  refused.
- **Edits are validated with the transaction**, not one by one: a transaction that upserts a parent
  VCA naming a child it upserts later in the same transaction must commit.

## Objective gates

1. **Round trips.** In `crates/protocol/src/session_wire/tests.rs`: `UpsertVca` with random VCAs
   (empty and non-empty `members`, both lanes, muted and unmuted), `RemoveVca` and `SetVcaFader`
   encode and decode to equal edits; an `UpsertVca` with an empty `members` decodes to `[]`.
   *Test value: it turns red if field 3 is lost, mis-tagged, or refused when empty, or if the fader
   is read from the wrong message.*
2. **Transactions.** In `crates/protocol/tests/` (a new file or `submix_strip_edits.rs`), against a
   committed snapshot:
   - `0700` adds a nested VCA and a parent that lists it in one transaction, and the snapshot's
     canonical JSON carries both, sorted; `0702` changes a fader; `0701` removes a leaf;
   - `0701` and `0702` at an unknown ID are `session.edit.not_found`; `SetTrackFader` (`020f`) at a
     VCA ID is `session.edit.not_found`;
   - removing a member track without rewriting its VCA refuses the whole transaction with
     `reference.missing_entity`, and `0700` making a cycle refuses with `vca.cycle`; the snapshot and
     revision are unchanged after each refusal.

   *Test value: it turns red if an apply arm edits the wrong entity, if a strip edit reaches a VCA,
   or if a VCA edit commits without final validation.*
3. **Registry and corpus.** `opcode_registry_...` reads 46 and asserts `0x0700`-`0x0702`; the
   controller test's fixture count reads 46 with the edit limit at 45; the conformance corpus
   carries the three edits.
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-protocol-wasm-parity.sh` (scalar and simd128 guests agree on the re-pinned
     hash)

   *Test value (registry): it turns red if an opcode is added to the enum but not to `from_raw`, or
   reuses a retired code.*
4. **Workspace and policy.**
   - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in protocol-control workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh` (`protocol` is a product crate; new code stores no
     splatted non-zero constant to memory, which the iOS `memset_pattern16` rule counts per crate)
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the
     batch push (recorded "at batch push").

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer, and the mutation that turned it red.
- The count and hash re-pins, old and new values.

## Dependencies

- *Declare VCA groups in the session* (#1240)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- In-place V1 amendment: append the message and the opcodes; never renumber or reuse.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
