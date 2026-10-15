# Diff a replacement document against the committed model and export replace from the browser engine module

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11).
Code anchors verified on `main` at `6fb211594`.

Cut from *Export transaction apply and anchored seek from the browser engine module* (#1293).

## Product outcome

The browser engine module accepts a whole session document while it plays and applies it as one
transaction: the control half diffs the document against the committed model and runs the normal
transaction apply. The SDK's `replaceSession` (*Apply session transactions from the browser SDK*,
#1296) calls this and nothing else, so there is still one edit path (D15-11).

## Context

- `SessionModel` (`crates/session/src/model.rs:89`) holds the session-level fields (`session_id`,
  `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`, `output_profile`, `console`)
  and the keyed lists `sources`, `tracks`, `submixes`, `vcas`, `outputs`, `routes`, `automation`.
- `SessionEdit` (`crates/protocol/src/model.rs:185`) has an `Upsert*` and a `Remove*` for each keyed
  kind, `SetConsole`, and a setter for each session-level field. `SessionStore::apply_transaction`
  (`model.rs:906`) resolves every edit in wire order and validates only the final candidate, so a
  transaction may remove a source before it removes the track that references it.
- The classifier compares models (`classify_live_delta`, `crates/host-core/src/live_delta.rs:211`),
  so a whole-entity upsert classifies exactly as the field edits it contains.
- *Replace the running browser session in the Rust host* (#1290) delivers the control half's
  `apply(transaction)` (its D1). Its D8 sketches `apply_document` (parse, diff, apply) and its gate
  9 tests it. **Responsibility split, so nothing is done twice:** this slice owns the diff,
  `apply_document` and the export. #1290 keeps `apply` only; root removes #1290's D8 and gate 9
  in the same checkpoint that files this spec.
- Export staging and the outcome record come from #1293 (`miso_engine_web_v1_edit_ptr`,
  `miso_engine_web_v1_edit_outcome_ptr`). The export-list mirrors are the ones #1293 lists
  (`scripts/check-web-audioworklet.sh:203`, `tools/parameter-metadata/src/abi_layout.rs:135`,
  `scripts/check-abi-layout-v1.py`, `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/check-sdk-generated.sh`).

## Decisions frozen for this slice

- **D1. Diff.** `hosts/host-web/src/document_diff.rs`:
  `fn diff(committed: &SessionModel, next: &SessionModel) -> Vec<SessionEdit>` emits, in order:
  1. a setter for each changed session-level field (`session_id`, `sample_rate_hz`,
     `quantum_frames`, `render_profile`, `output_profile`);
  2. a `Remove*` for each keyed entity present only in `committed`, in the order routes,
     automation, VCAs, tracks, submixes, outputs, sources;
  3. `SetConsole` when the console differs;
  4. an `Upsert*` for each keyed entity that is new or differs, in the order sources, outputs,
     submixes, tracks, VCAs, routes, automation.

  It never emits a finer edit. `revision` and `schema_version` are not diffed: the store owns the
  revision, and a document with another schema version is refused at parse.
- **D2. Method.** `AudioWorkletEngineHost::apply_document(&mut self, document: &[u8])` on the control
  half runs the boot's document size and parse budgets, parses into a model, runs D1, and applies
  the edits through #1290's `apply` at the committed revision. An empty diff commits nothing and
  returns the unchanged revision with path `model_only`. A rate or quantum change is refused by
  `apply` as #1290 D1 states.
- **D3. Export.** `miso_engine_web_v1_replace(handle, len) -> u32` runs D2 on the bytes staged
  through `miso_engine_web_v1_edit_ptr` and writes #1293's outcome record and, on refusal, the host
  diagnostic. Like #1293's exports, it runs on the control half only.
- **D4. Mirrors.** Add the export to every list in the context.

## Deliverables

1. D1 in `document_diff.rs`; D2 in `hosts/host-web/src/lib.rs`; D3 in `hosts/host-web/src/ffi.rs`.
2. D4 in the checkers, the generator and the generated SDK files.
3. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/document_diff.rs` (new), `hosts/host-web/src/lib.rs`,
  `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/tests.rs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py` and its
  fixture, `tools/parameter-metadata/src/abi_layout.rs`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`,
  `scripts/check-sdk-generated.sh`

## Non-goals

- No change to `apply` or classification (#1290, #1309). No SDK API (#1296).

## Objective gates

1. **Diff coverage.** For each keyed kind (source, track, submix, output, route, automation, VCA)
   one case adds, one removes and one changes an entity; one case changes the console; one the
   session ID. For each, `apply_document(B)` commits a snapshot equal to B's canonical text, and
   `diff` returns exactly one edit per differing entity.
2. **Dependent removal.** Removing a source and the track that reads it in one document commits.
3. **Same as a transaction.** `replace` with A plus a muted track returns the same outcome record
   (path `rebuild`, revision +1) and renders the same blocks as #1290's gate 1 transaction. A
   document that changes only one fader returns path `live`. An identical document returns
   `model_only`, the unchanged revision, and publishes no candidate.
4. **Refusals.** A malformed document, a document over `MAXIMUM_DOCUMENT_BYTES`, another schema
   version and another sample rate each refuse with their typed result and a diagnostic; the
   committed snapshot and revision are unchanged.
5. **Mirrors.** `python3 -B scripts/check-abi-layout-v1.py` with its self-test and
   `bash scripts/check-sdk-generated.sh <artifacts>` pass, and fail if one list omits the export.
6. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `rm -rf target/ci/h1386 && mkdir -p target/ci/h1386/a target/ci/h1386/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/h1386/n target/ci/h1386/a && bash scripts/check-web-audioworklet.sh target/ci/h1386/a target/ci/h1386/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh target/ci/h1386/a`, `bash scripts/test-web-audioworklet.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a diff that misses an entity kind silently leaves the committed model different from the
  document the app asked for; one that emits unchanged entities fails the exact edit count, which
  keeps the `live` and `model_only` paths reachable through `replace`.
- Gate 2: a diff that orders a source removal so that an intermediate check refuses it turns red.
- Gate 3: a `replace` that reboots or takes a second edit path diverges from the transaction's
  outcome or blocks.
- Gate 4: a refusal that moved the committed model before failing turns red.

## Dependencies

- *Replace the running browser session in the Rust host* (#1290).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).

## Amendment (controls coordinator, root rulings, 2026-10-15)

- **Re-scoped under decision 17 C1** (root ruling R8, 2026-10-15, recorded in #1293). The diff and
  `apply_document` go in the core, `crates/control-plane/src/`, not in `hosts/host-web`. A document
  replace is a protocol frame through `miso_engine_v1_submit_command` on the C ABI and
  `miso_engine_web_v1_control_command` on the browser (#1562's generic entry). Its staging is
  `miso_engine_web_v1_control_request_ptr`, bounded by the boot's `maximum_control_frame_bytes`. Its
  outcome is the response frame (revision, `edit_path`, diagnostics).
- **Stale references.** `miso_engine_web_v1_edit_ptr` and `miso_engine_web_v1_edit_outcome_ptr`
  are not built: #1293 dropped D1's staging (R8) and D3's outcome record (decision 17). D3's export
  `miso_engine_web_v1_replace` and D4's mirrors go with them. D2's "#1290's `apply`" is the control
  plane's transaction apply.
- The new message ID, its fields, both hosts' gates and the new authorized paths are re-briefed
  before implementation. That re-brief is root's.

## Re-brief (controls coordinator, batch 15 docs, 2026-10-15; root reviews at the push)

Code anchors verified on `c43b65870`. This section is the brief. Where it and the filed text
above differ, this section governs.

**Superseded filed text (kept above, not deleted).**
- Title and Product outcome, "export replace from the browser engine module": superseded; the
  replace is a protocol command on both hosts (decision 17 C1, root ruling R8).
- Context bullets 4 and 5 (#1290's `apply` and D8, #1293's staging, outcome record and mirrors):
  superseded; both hosts already run the core's transaction path.
- D1's path `hosts/host-web/src/document_diff.rs`: superseded by `crates/control-plane/src/document.rs`.
  D1's edit order stays, with the additions below.
- D2 (`AudioWorkletEngineHost::apply_document`): superseded by D2 below.
- D3 (`miso_engine_web_v1_replace`) and D4 (export mirrors): superseded; no export is added.
- Deliverables, Authorized paths, Objective gates 3, 5 and 6, Test value and Dependencies:
  superseded by the sections below. Gates 1, 2 and 4 are restated below.

### Product outcome

An app replaces the whole running session with a new session document while it plays. The engine
diffs the document against the committed model and applies the result as one transaction. The
outcome is the same as the equivalent transaction: the same revision, edit path, events and
rendered bits. The C ABI and the browser serve it through the same core function, as the protocol
command `SESSION_REPLACE` (`0x0013`). The SDK's `replaceSession` (#1296) sends this frame and
nothing else, so there is still one edit path (D15-11).

### Context (verified code anchors)

- **Message IDs.** `MessageId` is `crates/protocol/src/wire.rs:96-134`; the last command is
  `ResponseGet = 0x000e` (`:120`). `from_raw` is `:165-193`. `RETIRED_MESSAGE_IDS` is `:142`:
  `0x0004`, `0x0005`, `0x0006` and event `0x8002`. The registry is
  `docs/CONTROL_PROTOCOL_REGISTRY.md` (retired IDs `:11-15`, the command table `:31-45`).
- **IDs that open specs already claim.** `0x000f` is `OBSERVATION_READ` (#1586 D1, open).
  `0x0010` is `SPECTRUM_CONFIGURE` (#1594 D1, open). `0x0011` and `0x0012` are `SPECTRUM_CAPTURE`
  and `SPECTRUM_STREAM` (#1595 D1, open). Event `0x8040` is `RESPONSE_READY` (#763). Capability
  bits 14, 15 and 16 are the response, observation and spectrum slices'.
- **Capabilities.** `CapabilityFlags` is `crates/protocol/src/message_wire.rs:52-61`.
  `check_capabilities_invariants` is `:4095-4131`. `allocated_id` is `:4134-4143` and covers
  commands `1..=14`. `capability_registry` is `crates/protocol/src/controller.rs:3614`; its command
  array is `[0_u16; 12]` and its loop is `for id in 1_u16..=14` (`:3620`).
- **The transaction path.** `ControlCommand` is `controller.rs:934-966`. The controller's
  structural preparation is `plan_structural_command` (`:1880`); it accepts only
  `SessionTransactionApply` (`:1889`), and it runs the replay preflight, the exact-revision check,
  the decode and the edit-count check before it prepares. `prepare_command_frame` is `:1685` and
  `prepare_command_frame_in_place` is `:1717`. `ProtocolControllerConfig` is `:60-70`; the control
  plane builds it in `crates/control-plane/src/compile.rs:1153-1166`.
- **The core entry both hosts call.** `SessionState::command`
  (`crates/control-plane/src/control.rs:1510`) runs the controller's preparation, then the live
  commit (`commit_live`) or the rebuild. A rate or quantum change is refused there before anything
  is prepared, with `CommandError::CompileRejected` and `control.apply.reprepare_required`
  (`:1559-1577`, the constant `:859`). The C ABI calls it from `miso_engine_v1_submit_command`
  (`crates/capi/src/ffi.rs:681`). The browser calls it from `miso_engine_web_v1_control_command`
  (`hosts/host-web/src/ffi.rs:3366`), with the frame staged at
  `miso_engine_web_v1_control_request_ptr` (`:3347`).
- **Frame bounds.** The control plane's codec takes `max_frame_bytes` and `max_string_bytes` from
  the boot's `maximum_control_frame_bytes` (`compile.rs:1085-1091`). The replay cache stores each
  new request's whole bytes (`controller.rs:336-366`). The browser's frame bound is 16 KiB
  (`CONTROL_FRAME_BYTES`, `hosts/host-web/src/lib.rs:5087`); its document bound is 1 MiB
  (`MAXIMUM_DOCUMENT_BYTES`, `:74`). The C ABI's are caller-chosen (`CompileLimits`).
- **Parse.** `host_core::parse_host_session` (`crates/host-core/src/prepare.rs:1145`) is what the
  boot uses (`compile.rs:1054`).
- **The model.** `SessionModel` is `crates/session/src/model.rs:89-127`. Every keyed list is sorted
  by stable ID in the compiled model (`crates/session/src/compile.rs:140-164`).
  `control_smoothing` (`model.rs:116-120`) is `None` for `{ "kind": "default" }`. `SessionEdit` is
  `crates/protocol/src/model.rs:154`. No edit sets `control_smoothing` whole: `SET_CONTROL` of the
  session rows `fader_ms`, `mute_ms` and `pan_ms` writes the explicit arm, and no transaction
  writes the default arm back (registry `:139`).
- **Classification.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:344`) compares
  models (steps `:259-280`), so a whole-entity upsert that changes only values classifies as live.
- **Response.** `TransactionApplied` (`message_wire.rs:198-201`) is `applied_operations` and
  `edit_path`. The response header carries the revision (`wire.rs:330-344`).
- **Pins.** `COMPLETE_SCHEMA_HASH` is `crates/conformance/src/protocol_corpus.rs:928`, with its
  re-pin history in the doc comment above it. Its other spellings are
  `scripts/check-protocol-wasm-parity.sh:172-173`, `fuzz/corpus/complete-schema-manifest.md:10`
  and `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`. The protocol allocation audit lists one frame per
  command (`tools/audit/src/protocol.rs:142-200`).
- **SDK.** The SDK encodes its frames by hand (`sdk/src/core/control-frame.ts:32` for `0x0003`).
  No generated SDK table lists message IDs (`sdk/src/generated/`: `abi.ts`, `catalog.ts`,
  `control-builders.ts`, `controls.ts`, `provenance.ts`, `session-keys.ts`).
- **Parity test.** `hosts/host-web/tests/control_parity.rs` boots the same session on the C ABI
  (`CAbi`, `:899`) and the browser (`Rig`, `hosts/host-web/tests/common/mod.rs:349`) and sends the
  same frames to both.

### Decisions frozen for this slice

- **D1. The message.** `MessageId::SessionReplace = 0x0013` (`SESSION_REPLACE`), command and
  response. Why `0x0013`: it is the lowest command ID that is not allocated, not retired and not
  claimed by an open spec. `0x0001`-`0x0003` and `0x0007`-`0x000e` are allocated; `0x0004`-`0x0006`
  are retired and never reallocated; `0x000f`-`0x0012` are claimed by #1586, #1594 and #1595
  (open). A search of the code, the docs and `.github/ISSUE_SPECS/` finds no other use of `0x0013`
  as a message ID. If this slice lands before those three, `0x000f`-`0x0012` stay unallocated and
  decode as `UNSUPPORTED_MESSAGE` until they land.
  - Command: `1:document BYTES R`, a session JSON document as the boot takes it. Exact revision
    only (`requires_exact_revision`). An empty document is `INVALID_FIELD`.
  - Success response: the `0003` payload, `1:applied-operations U32 R`, `2:edit-path U8 R`
    (`TransactionApplied`). `edit_path` is live, model_only or rebuild. It is never monitor: a
    document does not carry solo, and the monitor overlay is kept.
  - Capability: no new flag bit. `SESSION_REPLACE` is listed only with the session family
    (`0003` and `8001`, bit 3). The decode invariant gains: `has_command(0x0013)` implies the
    session family. `allocated_id` and the registry loop gain `0x0013`, and the registry's command
    array grows to fit.
- **D2. The diff, in the core.** `crates/control-plane/src/document.rs` (new):
  `pub fn diff(committed: &SessionModel, next: &SessionModel) -> Vec<SessionEdit>`. It emits the
  filed D1 order (session-level setters; removals in the order routes, automation, VCAs, tracks,
  submixes, outputs, sources; `SetConsole`; upserts in the order sources, outputs, submixes,
  tracks, VCAs, routes, automation), then the `control_smoothing` step below. Rules:
  - An entity differs when its bits differ, not when `PartialEq` says so: a zero's sign counts,
    as in the classifier's step 1. One edit per differing entity, never a finer edit.
  - `revision` and `schema_version` are not diffed. The header's exact revision is the guard, and
    a document of another schema version is refused at parse.
  - `control_smoothing`: when `next` holds an explicit arm whose lengths differ from the
    committed lengths, the diff emits one `SetControl` per changed session row (`fader_ms`,
    `mute_ms`, `pan_ms`). When `next` holds the default arm and the committed model holds an
    explicit arm, no edit can express it. The replace path then sets the candidate's
    `control_smoothing` to `None` after the edits and before the final validation, through one
    new `SessionStore` preparation entry for the replace path only. No wire edit is added. The
    change is model-only (no prepared plan reads the key). **Root checks this point at the
    push.**
- **D3. The path.** The controller does the frame work; the control plane owns parse and diff.
  - `ProtocolControllerConfig` gains `document_replace: Option<DocumentReplaceFn>`, a plain `fn`
    pointer that takes the committed `SessionModel`, the document bytes and the document byte
    bound, and returns the edits plus the replace's `control_smoothing` value, or a typed refusal
    (status and diagnostic). The control plane sets it to its own function in
    `compile_children`. The protocol crate holds no diff code. `0x0013` is advertised only when
    it is set and session transactions are on.
  - `plan_structural_command` accepts `SessionReplace` beside `SessionTransactionApply`. It runs
    the same replay preflight, exact-revision check and output reservation first, then calls
    `document_replace`, then the same edit-count check and the same preparation. Every refusal goes
    through the controller's non-OK response, so the replay cache holds it like any other reply.
  - An empty diff (and no smoothing change) commits nothing. The reply is OK with
    `applied_operations` 0, `edit_path` model_only and the unchanged revision. No
    `SESSION_COMMITTED` is queued and no candidate is published.
  - Otherwise the prepared token goes to `SessionState::command` exactly as a transaction's does:
    the live commit or the rebuild, the rate and quantum refusal, the plan admission and the
    `SESSION_COMMITTED` event (operation count = `applied_operations`). The reply's message ID is
    `0x0013`.
  - Both hosts call `SessionState::command`. Neither host adds code for this message, and no
    export is added.
- **D4. Refusals, all before commit.** Each one leaves the revision, the committed document and
  the rendered bits unchanged, and queues no event:
  - a document longer than the boot's `maximum_document_bytes`: `LIMIT_EXCEEDED`,
    `session.replace.document_too_large`;
  - a frame over the codec's bound: the codec's own refusal, as for any frame;
  - a document that does not parse or validate: `VALIDATION_FAILED` with the parser's
    diagnostics;
  - more edits than `maximum_transaction_edits`: `LIMIT_EXCEEDED`, as for a transaction;
  - a stale revision: `REVISION_CONFLICT`, as for a transaction;
  - a rate or quantum change: `CompileRejected`, `control.apply.reprepare_required`, as for a
    transaction (`control.rs:1559-1577`);
  - a structural replace while a candidate is pending: whatever a transaction gets at that time
    (today `BACKPRESSURE`; after #1310, supersession).
- **D5. The acked-batch question.** The OK reply is written only after the commit. A refusal is
  returned before any state changes. No queue is added. An ack can never precede a drop.
- **D6. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md` (the command row, the appended-ID note, the
  capability rule), `docs/CONTROL_PROTOCOL_SEMANTICS.md` (the replace rules of D2-D5),
  `docs/CONTROL_PROTOCOL_SIZING.md` (the request is the document plus the header; the bound is
  the smaller of the frame bound and `maximum_document_bytes`), the C ABI header prose and
  `docs/C_ABI_V1_QUALIFICATION.md`.
- **Open for root at the push (not frozen).** The browser's frame bound is 16 KiB, so on the
  browser a document larger than about 16 KiB cannot be replaced; the 1 MiB document bound does
  not help. This slice does not change `CONTROL_FRAME_BYTES`. Root decides whether a successor
  raises the browser's frame bound (it is charged in the memory budget) or adds a chunked form.

### Deliverables

1. D1 in `crates/protocol` (wire, schema, codec, typed frame, controller, capabilities) and the
   corpus row and re-pin in `crates/conformance`.
2. D2 in `crates/control-plane/src/document.rs`; D3 in `crates/protocol/src/controller.rs`,
   `crates/protocol/src/model.rs` (the replace preparation entry only) and
   `crates/control-plane/src/{compile.rs,control.rs,lib.rs}`.
3. D6 and the gates below.

### Authorized paths

- `crates/protocol/src/{wire.rs,message_wire.rs,message_wire/tests.rs,schema.rs,typed_frame.rs,controller.rs,controller/tests.rs,model.rs,lib.rs}`.
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`, `scripts/check-protocol-wasm-parity.sh` (the
  self-test hash strings only).
- `crates/control-plane/src/{document.rs,compile.rs,control.rs,lib.rs}`,
  `crates/control-plane/tests/session_replace.rs` (new).
- `crates/capi/tests/session_replace.rs` (new), `crates/capi/include/miso_engine_v1.h` (prose),
  `tools/audit/src/protocol.rs` (one frame).
- `hosts/host-web/tests/control_parity.rs`.
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING,CONFORMANCE}.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`.

### Non-goals

- No SDK change. `replaceSession` and its frame encoder are #1296's
  (`sdk/src/core/control-frame.ts`). No generated SDK table lists message IDs, so none moves.
- No browser export, no export mirror, no change to `hosts/host-web/src/`.
- No change to classification, the live commit or the rebuild.
- No change to the browser's frame bound (see the open point in D6).
- No supersession of a pending candidate (#1310).

### Objective gates

Commands:
- `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`
- `cargo test --locked -p control-plane --features control-plane/test-support`
- `cargo test --locked -p capi`
- `cargo test --locked -p host-web --features host-web/test-support`
- `cargo build --locked --release -p audit && bash scripts/run-protocol-allocation-audit.sh target/release/audit`
- `bash scripts/check-protocol-wasm-parity.sh`, `bash scripts/check-protocol-control-policy.sh`,
  `bash scripts/check-conformance-boundaries.sh`, `bash scripts/check-capi-abi.sh`,
  `bash scripts/check-workspace-policy.sh`
- `CARGO_TARGET_DIR=target/ci/wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke -p protocol`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
  `cargo fmt --all -- --check`

1. **Diff coverage (control-plane test).** For each keyed kind (source, track, submix, output,
   route, automation, VCA), one case adds, one removes and one changes an entity; one case changes
   the console; one the session ID; one a fader from `0.0` to `-0.0`; one each way between the
   default and an explicit `control_smoothing`. For each, `diff` returns exactly one edit per
   differing entity (or the smoothing rows), and a replace commits a canonical snapshot equal to
   that of a session booted from the document, except the revision.
2. **Dependent removal.** A document that removes a source and the track that reads it commits.
3. **Same as a transaction (C ABI, `crates/capi/tests/session_replace.rs`).** Through
   `miso_engine_v1_submit_command`: a replace with the boot document plus a muted track returns
   OK, path rebuild, revision +1, and renders the same blocks as the equivalent
   `SESSION_TRANSACTION_APPLY` on a twin session. A document that changes one fader returns path
   live. An identical document returns OK, model_only, the unchanged revision; no
   `SESSION_COMMITTED` is dequeued and no candidate is published.
4. **Refusals before commit (C ABI).** One case for each refusal of D4 except the pending
   candidate. Each returns its typed status (or result) and diagnostic. After each, the revision
   and the `SESSION_SNAPSHOT_GET` bytes are unchanged, no event is dequeued, and the next 8 blocks
   are bit-identical to a twin that got no command.
5. **Replay.** The same request ID with the same bytes returns the cached reply and commits once.
   The same request ID with other bytes is `REQUEST_ID_REUSE`.
6. **Both hosts (parity row).** A new test in `hosts/host-web/tests/control_parity.rs` sends the
   same `SESSION_REPLACE` frames to the C ABI and the browser: one rebuild, one live, one
   identical document and one refusal. The two hosts return the same reply bytes (request ID
   aside), the same revision, the same `SESSION_SNAPSHOT_GET` bytes and bit-identical blocks, at
   44.1 and 48 kHz.
7. **Wire and pins.** Codec round trips of the command and the reply. Decode refuses a missing
   field 1 and a capability record that lists `0x0013` without the session family. The corpus
   gains `command.session_replace` and `response.session_replace`. `COMPLETE_SCHEMA_HASH` is
   re-pinned once, with a history line naming #1386, and its other spellings (parity script,
   manifest, conformance doc) move with it. The allocation audit includes the new frame.

### Test value

- Gate 1: a diff that misses a kind, emits an unchanged entity or compares with `PartialEq`
  (the sign of zero) turns red; so does a replace that cannot return to the default smoothing.
- Gate 2: a diff whose removal order makes an intermediate check refuse turns red.
- Gate 3: a replace that takes a second edit path, reboots, or commits on an identical document
  turns red.
- Gate 4: a refusal that moves the model, the revision or a rendered bit before it fails turns
  red.
- Gate 5: a replace path that skips the replay preflight commits twice.
- Gate 6: a host that runs its own replace code, or answers other bytes, turns red.
- Gate 7: a frame shape or capability rule that differs from the registry turns red.

### Dependencies (checked with `gh issue view` on 2026-10-15)

- Closed and needed: #1309 (the control plane both hosts call), #1562 (the browser's generic
  command entry), #1381 (the browser runs `SessionState`).
- Not dependencies any more: #1290 (open; its `apply` is now the core's transaction path) and
  #1293 (open; R8 dropped the staging this slice used).
- Ordering only: #1586, #1594 and #1595 (open) hold `0x000f`-`0x0012`. #1310 (open, batch 15a)
  changes what a structural replace gets while a candidate is pending; this slice inherits it.
- Downstream: #1296 (open) sends this frame from `replaceSession`.
