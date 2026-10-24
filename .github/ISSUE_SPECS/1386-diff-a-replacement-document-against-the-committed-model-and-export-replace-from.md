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

- **Re-scoped under decision 17 C1** (root ruling B15-R8, 2026-10-15, recorded in #1293). The diff and
  `apply_document` go in the core, `crates/control-plane/src/`, not in `hosts/host-web`. A document
  replace is a protocol frame through `miso_engine_v1_submit_command` on the C ABI and
  `miso_engine_web_v1_control_command` on the browser (#1562's generic entry). Its staging is
  `miso_engine_web_v1_control_request_ptr`, bounded by the boot's `maximum_control_frame_bytes`. Its
  outcome is the response frame (revision, `edit_path`, diagnostics).
- **Stale references.** `miso_engine_web_v1_edit_ptr` and `miso_engine_web_v1_edit_outcome_ptr`
  are not built: #1293 dropped D1's staging (B15-R8) and D3's outcome record (decision 17). D3's export
  `miso_engine_web_v1_replace` and D4's mirrors go with them. D2's "#1290's `apply`" is the control
  plane's transaction apply.
- The new message ID, its fields, both hosts' gates and the new authorized paths are re-briefed
  before implementation. That re-brief is root's.

## Re-brief (controls coordinator, batch 15 docs, 2026-10-15; root reviews at the push)

Code anchors verified on `c43b65870` (the staging anchors on `bed928efb`). This section is the
brief. Where it and the filed text above differ, this section governs.

**Superseded filed text (kept above, not deleted).**
- Title and Product outcome, "export replace from the browser engine module": superseded; the
  replace is a protocol command on both hosts (decision 17 C1, root ruling B15-R8).
- Context bullets 4 and 5 (#1290's `apply` and D8, #1293's staging, outcome record and mirrors):
  superseded; both hosts already run the core's transaction path.
- D1's path `hosts/host-web/src/document_diff.rs`: superseded by `crates/control-plane/src/document.rs`.
  D1's edit order stays, with the additions below.
- D2 (`AudioWorkletEngineHost::apply_document`): superseded by D2 below.
- D3 (`miso_engine_web_v1_replace`) and D4 (export mirrors): superseded; no replace export is
  added. Each host gains one document transport export only (D7).
- Deliverables, Authorized paths, Objective gates 3, 5 and 6, Test value and Dependencies:
  superseded by the sections below. Gates 1, 2 and 4 are restated below.

### Product outcome

An app replaces the whole running session with a new session document while it plays. The engine
diffs the document against the committed model and applies the result as one transaction. The
outcome is the same as the equivalent transaction: the same revision, edit path, events and
rendered bits. The C ABI and the browser serve it through the same core function, as the protocol
command `SESSION_REPLACE` (`0x0013`). The SDK's `replaceSession` (#1296) sends this frame and
nothing else, so there is still one edit path (D15-11). A replace accepts every document the boot
accepts, up to the boot's `maximum_document_bytes` (1 MiB on the browser), on both hosts. The
document does not go in the frame: the frame names a document the host passes beside it (D7), and
the control plane validates and parses it off the render thread.

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
  (`MAXIMUM_DOCUMENT_BYTES`, `:74`). The C ABI's are caller-chosen (`CompileLimits`). So a
  document cannot go in a frame: 16 KiB would cap the browser's replace far below its boot.
- **The browser's boot document staging** (anchors verified on `bed928efb`). It is the
  module's thread-local `BOOT_STAGING` (`hosts/host-web/src/ffi.rs:566`), a `BootStaging`
  (`:105-115`) whose `document: ManuallyDrop<Vec<u8>>` is freed only by assignment, never by a
  thread-local destructor (#1333 D5). `miso_engine_web_v1_document_ptr(len)` (`:2957-2992`)
  refuses a live instance with `RESULT_REFUSED_LIFECYCLE` (`:2964-2969`), refuses
  `len > MAXIMUM_DOCUMENT_BYTES` (`:2971`), then `try_reserve_exact`s a `Vec` of exactly `len`
  bytes, assigns it over the old one and returns its address (`:2976-2990`). `boot_staged`
  (`:3006-3124`) parses the staged bytes and, on success, clears `document_valid` but keeps the
  bytes (`:3051-3055`): after boot the instance retains a copy of the boot document, not charged
  in the budget's bridge rows (`hosts/host-web/src/lib.rs:4774-4790`); dispose frees it
  (`ffi.rs:526-554`, `reset_after_dispose`). The page hands the document over in the worklet's
  message handler (`port.onmessage`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js:287`):
  boot writes it at `document_ptr` and calls `boot` (`:360-369`), and every control frame goes
  through `miso.control.v1` (`CONTROL_FIELDS = ["tag", "requestId", "frame"]`, `:93`;
  `receiveControl` `:1699`; `readControlCommand` `:1727-1735`). `process()` (`:1911`) never
  stages or parses. The live-instance refusal of `document_ptr` is pinned by
  `sdk/test/boot-evals.mjs:325-350`. The frame path is `control_request_mut` (`lib.rs:2305`,
  bounded by `CONTROL_FRAME_BYTES`), `control_command` (`lib.rs:2345`) and `run_command`
  (`lib.rs:1604`), which calls `SessionState::command` (`:1615`).
- **The C ABI's boot document.** It has no staging. `miso_engine_v1_compile_session`
  (`crates/capi/src/ffi.rs:311`) takes `document: *const u8, document_bytes: u64`, refuses
  `document_bytes > limits.maximum_document_bytes` (`:347`) and borrows the bytes for the call
  only (`borrowed_bytes`, `:361`). `CompileLimits::maximum_document_bytes` is
  `crates/capi/src/abi.rs:122`. `miso_engine_v1_submit_command` (`ffi.rs:681`) borrows only the
  frame. The bound reaches the control plane as `ControlLimits::maximum_document_bytes`
  (`crates/control-plane/src/adapter.rs:35`, set at `crates/capi/src/runtime/mod.rs:187` and
  `hosts/host-web/src/lib.rs:5057`), but the control plane does not read it today.
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
  - Command: `1:document-bytes U32 R`, the exact length of the document the host passes beside
    the frame (D7), and `2:document-sha256 BYTES R`, exactly 32 bytes, the SHA-256 of that
    document. The frame carries no document bytes. Exact revision only
    (`requires_exact_revision`). `document-bytes` 0 or a digest that is not 32 bytes is
    `INVALID_FIELD`. Why the digest: the replay cache compares frame bytes, and the document is
    not in the frame. With the digest, the same request ID with another document is other frame
    bytes, so it is `REQUEST_ID_REUSE`, never a cached reply for a document it did not apply.
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
    explicit arm, no edit can express it. Decided (root review, 2026-10-15): a replace-only
    preparation step applies the document's `control_smoothing` to the candidate (an absent key
    is the session default, `None`) after the edits and before the final validation, through one
    new `SessionStore` preparation entry that only the replace path calls. No wire edit is
    added. The change is model-only (no prepared plan reads the key).
- **D3. The path.** The controller does the frame work; the control plane owns parse and diff.
  - The core entry. `SessionState` gains `command_with_document(&mut self, request: &[u8],
    document: &[u8], output_capacity: u64)`; `command(request, output_capacity)` is
    `command_with_document(request, &[], output_capacity)`. The document is borrowed for the call
    only; nothing in the control plane or the protocol crate keeps it. The controller's frame
    preparation (`prepare_command_frame`, `prepare_command_frame_in_place`) threads the same
    borrowed slice to `plan_structural_command`. `SessionState` keeps `maximum_document_bytes`
    from its `ControlLimits` (today unread) as the replace's bound.
  - `ProtocolControllerConfig` gains `document_replace: Option<DocumentReplaceFn>`, a plain `fn`
    pointer that takes the committed `SessionModel`, the borrowed document, the frame's
    `document-bytes` and `document-sha256`, and the document byte bound, and returns the edits
    plus the replace's `control_smoothing` value, or a typed refusal (status and diagnostic). The
    control plane sets it to its own function in `compile_children`. The protocol crate holds no
    diff, parse or hash code. `0x0013` is advertised only when it is set and session
    transactions are on.
  - `document_replace` checks, in this order: the document length equals `document-bytes`; the
    length is at most `maximum_document_bytes`; the SHA-256 of the document equals
    `document-sha256` (`sha2`, the workspace pin, added to `crates/control-plane/Cargo.toml`);
    then it parses with `parse_host_session` and diffs. All of it is control work, never render.
  - `plan_structural_command` accepts `SessionReplace` beside `SessionTransactionApply`. It runs
    the same replay preflight, exact-revision check and output reservation first, then calls
    `document_replace`, then the same edit-count check and the same preparation. A replayed
    request returns its cached reply before the document is read, so a retry needs only the
    same frame bytes. Every refusal goes through the controller's non-OK response, so the replay
    cache holds it like any other reply.
  - An empty diff (and no smoothing change) commits nothing. The reply is OK with
    `applied_operations` 0, `edit_path` model_only and the unchanged revision. No
    `SESSION_COMMITTED` is queued and no candidate is published.
  - Otherwise the prepared token goes to `SessionState::command` exactly as a transaction's does:
    the live commit or the rebuild, the rate and quantum refusal, the plan admission and the
    `SESSION_COMMITTED` event (operation count = `applied_operations`). The reply's message ID is
    `0x0013`.
  - Both hosts call `SessionState::command_with_document`. Neither host decodes the frame or
    runs replace code; each host adds only the document transport of D7.
- **D4. Refusals, all before commit.** Each one leaves the revision, the committed document and
  the rendered bits unchanged, and queues no event:
  - no document beside the frame (the plain `submit_command` or a browser command with nothing
    staged), or a document whose length is not `document-bytes`: `INVALID_FIELD`,
    `session.replace.document_length`;
  - a document longer than the boot's `maximum_document_bytes`: `LIMIT_EXCEEDED`,
    `session.replace.document_too_large` (the browser's D7 staging refuses this length first);
  - a document whose SHA-256 is not `document-sha256`: `INVALID_FIELD`,
    `session.replace.document_digest`;
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
  `docs/CONTROL_PROTOCOL_SIZING.md` (the request is the header plus two fixed fields, under
  every frame bound; the document's only bound is `maximum_document_bytes`), the C ABI header
  prose for D7's export and `docs/C_ABI_V1_QUALIFICATION.md`.
- **D7. The document transport: one core rule, each host's existing document shape.** The
  core rule is D3's: the host passes the document as a borrowed `&[u8]` beside the frame, and
  the control plane checks, hashes, parses and diffs it on the control side. A host differs only
  in how the bytes reach that borrow, as it already does for frames (the browser stages frames,
  the C ABI borrows them). Neither host keeps the document after the call.
  - **Browser: reuse the boot document staging.** New export
    `miso_engine_web_v1_replace_document_ptr(handle: u32, len: u32) -> u32`. It returns the
    address of the same `BOOT_STAGING.document` buffer the boot uses, sized exactly as
    `document_ptr` sizes it (`try_reserve_exact`, assignment over the old `Vec`), and sets
    `document_valid`. It is the live twin of `document_ptr`: it returns 0, and stages nothing,
    for a wrong handle, a host that is not ready, `len` 0, `len > MAXIMUM_DOCUMENT_BYTES` or a
    failed reservation. It does not touch the boot result word. `document_ptr` stays pre-boot
    only (its lifecycle refusal is pinned, `sdk/test/boot-evals.mjs:325-350`).
    `miso_engine_web_v1_control_command(handle, len)` keeps its signature: it passes
    `&BOOT_STAGING.document` when `document_valid` (else `&[]`) to
    `SessionState::command_with_document` through `run_command`, and after every command,
    whatever its message, it frees the staging (`*document = Vec::new()`) and clears
    `document_valid`. So a staged document serves exactly the next command and is never
    retained; the first command after boot also frees the boot document copy that boot kept.
    The worklet's `miso.control.v1` message accepts an optional `document` field (a
    `Uint8Array`, the same checks as `frame`); `readControlCommand` stages it with
    `replace_document_ptr` before it stages the frame. This is the message handler's work
    (`single` mode) or the control side's; `process()` never stages, hashes or parses.
    **ROOT APPROVED (2026-10-15): a new browser export.** Reason: `document_ptr` refuses a live instance and
    that refusal is a pinned contract; relaxing it would change the single-live-handle
    lifecycle rule. The export moves the ABI layout mirrors (D7 paths).
  - **C ABI: borrow the document beside the frame.** The C ABI has no staging a frame can name
    (Context). New export `miso_engine_v1_submit_document_command(session: *mut Session,
    request: *const u8, request_bytes: u64, document: *const u8, document_bytes: u64, response:
    *mut BytesOut) -> u32`, Thread: control. It is `miso_engine_v1_submit_command`'s contract
    plus the document borrowed for this call only under `compile_session`'s pointer-plus-length
    rule (`borrowed_bytes`, `ffi.rs:361`); `document` must be nonnull when `document_bytes` is
    nonzero, else `RESULT_INVALID_ARGUMENT`. It calls `command_with_document` and maps results
    exactly as `submit_command` does. `submit_command` is unchanged; a `SESSION_REPLACE` frame
    sent through it is the D4 `session.replace.document_length` refusal. The C ABI retains
    nothing. A session-owned copy staging (a stage export plus a buffer) was rejected: it would
    retain up to `maximum_document_bytes` per session and add a stale-staging state.
    **ROOT APPROVED (2026-10-15): a new C export.**

### Deliverables

1. D1 in `crates/protocol` (wire, schema, codec, typed frame, controller, capabilities) and the
   corpus row and re-pin in `crates/conformance`.
2. D2 in `crates/control-plane/src/document.rs`; D3 in `crates/protocol/src/controller.rs`,
   `crates/protocol/src/model.rs` (the replace preparation entry only) and
   `crates/control-plane/src/{compile.rs,control.rs,lib.rs}`.
3. D7: the browser's export, staging consumption and worklet message field; the C ABI's export.
4. D6 and the gates below.

### Authorized paths

- `crates/protocol/src/{wire.rs,message_wire.rs,message_wire/tests.rs,schema.rs,typed_frame.rs,controller.rs,controller/tests.rs,model.rs,lib.rs}`.
- `crates/conformance/src/protocol_corpus.rs`, `crates/conformance/tests/conformance_corpus.rs`,
  `fuzz/corpus/complete-schema-manifest.md`, `scripts/check-protocol-wasm-parity.sh` (the
  self-test hash strings only).
- `crates/control-plane/src/{document.rs,compile.rs,control.rs,lib.rs}`,
  `crates/control-plane/Cargo.toml` (`sha2` only), `Cargo.lock`,
  `crates/control-plane/tests/session_replace.rs` (new).
- `crates/capi/src/ffi.rs` (D7's export only), `crates/capi/tests/session_replace.rs` (new),
  `crates/capi/include/miso_engine_v1.h` (D7's declaration and prose),
  `crates/capi/tests/c/abi_smoke.c` (the declaration's link check, if the smoke lists every
  export), `scripts/check-capi-abi.sh` (the export list), `tools/audit/src/protocol.rs` (one
  frame).
- `hosts/host-web/src/{ffi.rs,lib.rs}` (D7 only), `hosts/host-web/web/miso-engine-v1-audio-worklet.js`
  (the optional `document` field only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
  (the field's type, if the message is typed there),
  `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` (re-pinned by
  `scripts/build-web-audioworklet.sh`), `scripts/check-web-audioworklet.sh` (the export list),
  `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json` and the generated mirrors
  `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts` (the export name
  only, regenerated, never hand-edited), `scripts/test-web-audioworklet.mjs` (the fake module's
  export and the message-field cases).
- `hosts/host-web/tests/control_parity.rs`.
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS,SIZING,CONFORMANCE}.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`.

### Non-goals

- No hand-written SDK change. `replaceSession`, its frame encoder and its use of the worklet's
  `document` field are #1296's (`sdk/src/core/control-frame.ts`, `sdk/src/core/boundary.ts`).
  No generated SDK table lists message IDs, so none moves; only the generated ABI export list
  gains D7's browser export.
- No browser replace export and no change to the browser's boot path or `document_ptr`.
- No change to classification, the live commit or the rebuild.
- No change to the browser's frame bound (`CONTROL_FRAME_BYTES` stays 16 KiB) and no chunked
  frame: D7 carries the document.
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
- The web artifact gates with the arguments `.github/workflows/qualification.yml` gives them:
  `scripts/build-web-audioworklet.sh` (`:132`), `scripts/check-web-audioworklet.sh` (`:342`),
  `scripts/test-web-audioworklet.mjs` (`:356`); then `bash scripts/check-sdk-generated.sh`,
  `python3 scripts/check-abi-layout-v1.py` and
  `node scripts/check-web-boot-budget.mjs <the built module>.wasm`
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
   `miso_engine_v1_submit_document_command`: a replace with the boot document plus a muted track returns
   OK, path rebuild, revision +1, and renders the same blocks as the equivalent
   `SESSION_TRANSACTION_APPLY` on a twin session. A document that changes one fader returns path
   live. An identical document returns OK, model_only, the unchanged revision; no
   `SESSION_COMMITTED` is dequeued and no candidate is published.
4. **Refusals before commit (C ABI).** One case for each refusal of D4 except the pending
   candidate. Each returns its typed status (or result) and diagnostic. After each, the revision
   and the `SESSION_SNAPSHOT_GET` bytes are unchanged, no event is dequeued, and the next 8 blocks
   are bit-identical to a twin that got no command.
5. **Replay.** The same request ID with the same frame bytes returns the cached reply and commits
   once, also when the retry passes no document. The same request ID with another document (so
   another `document-sha256`) is `REQUEST_ID_REUSE` and commits nothing.
6. **Both hosts (parity row).** A new test in `hosts/host-web/tests/control_parity.rs` sends the
   same `SESSION_REPLACE` frames and documents to the C ABI (D7's export) and the browser (D7's
   staging, then `control_command`): one rebuild, one live, one identical document and one
   refusal. The two hosts return the same reply bytes (request ID aside), the same revision, the
   same `SESSION_SNAPSHOT_GET` bytes and bit-identical blocks, at 44.1 and 48 kHz.
7. **Wire and pins.** Codec round trips of the command and the reply. Decode refuses a missing
   field 1 or 2, a digest that is not 32 bytes, and a capability record that lists `0x0013`
   without the session family. The corpus gains `command.session_replace` and
   `response.session_replace`. `COMPLETE_SCHEMA_HASH` is re-pinned once, with a history line
   naming #1386, and its other spellings (parity script, manifest, conformance doc) move with
   it. The allocation audit includes the new frame. The web artifact digest and the ABI layout
   mirrors move once, regenerated by their own tools.
8. **A document larger than the frame (parity row, root review a).** A new test in
   `hosts/host-web/tests/control_parity.rs` boots a small session on both hosts and replaces it
   with a document of more than 64 KiB (four times the browser's 16 KiB frame bound; a valid
   session whose size comes from more tracks, not padding), then with a second large document
   that changes one fader (path live). Both hosts return OK, the same reply bytes (request ID
   aside), revision, snapshot bytes and bit-identical blocks. A browser case at exactly
   `MAXIMUM_DOCUMENT_BYTES` stages and commits, and one byte more gets 0 from
   `replace_document_ptr`. The gate records, per host, from a measurement in the test:
   - bytes the host retains for the document after the reply: browser, the capacity of
     `BOOT_STAGING.document` (expected 0, D7 frees it, including the boot copy); C ABI, 0
     (borrowed, nothing kept);
   - bytes the host holds for the document during the call: browser, the staged length; C ABI,
     none of its own (the caller's buffer);
   - on the browser, the wasm linear memory growth across one replace of the 1 MiB boot ceiling
     document (`scripts/check-web-boot-budget.mjs` method), against the boot's projection
     `document_bytes * PARSE_TRANSIENT_MULTIPLIER` (`hosts/host-web/src/lib.rs:76-98`).
   Any retained byte row these numbers move (`exact_retained_bytes`, the boot budget script's
   pin) is re-pinned from the measurement, with the measured figure in its history comment. A
   replace growth above the boot projection is reported to root, not pinned.

### Test value

- Gate 1: a diff that misses a kind, emits an unchanged entity or compares with `PartialEq`
  (the sign of zero) turns red; so does a replace that cannot return to the default smoothing.
- Gate 2: a diff whose removal order makes an intermediate check refuse turns red.
- Gate 3: a replace that takes a second edit path, reboots, or commits on an identical document
  turns red.
- Gate 4: a refusal that moves the model, the revision or a rendered bit before it fails turns
  red.
- Gate 5: a replace path that skips the replay preflight commits twice; a frame without the
  digest answers a cached reply for another document.
- Gate 6: a host that runs its own replace code, or answers other bytes, turns red.
- Gate 7: a frame shape or capability rule that differs from the registry turns red.
- Gate 8: a replace bounded by the frame instead of `maximum_document_bytes`, a browser staging
  that keeps the document after the command, or a host whose large-document reply differs,
  turns red.

### Dependencies (checked with `gh issue view` on 2026-10-15)

- Closed and needed: #1309 (the control plane both hosts call), #1562 (the browser's generic
  command entry), #1381 (the browser runs `SessionState`).
- Not dependencies any more: #1290 (open; its `apply` is now the core's transaction path) and
  #1293 (open; B15-R8 dropped the staging this slice used).
- Ordering only: #1586, #1594 and #1595 (open) hold `0x000f`-`0x0012`. #1310 (open, batch 15a)
  changes what a structural replace gets while a candidate is pending; this slice inherits it.
- Downstream: #1296 (open) sends this frame from `replaceSession`.

### Root review (2026-10-15)

Root approved D1 (`0x0013` `SESSION_REPLACE`), the diff in the control plane (D2), the
`TransactionApplied` reply and the gates, and ruled the two open points. The sections above
already carry both rulings.
- **(a) Document size.** A replace accepts every document the boot accepts, up to
  `maximum_document_bytes`, on both hosts; it is not capped by the control frame. The frame names
  the document and does not carry it; the bytes are validated and parsed off the render thread,
  by one rule on both hosts. Applied as D1's fields, D3's core entry, D4's refusals, D7 and
  gate 8. D7 adds one export per host; root approved both (2026-10-15).
- **(b) `control_smoothing`.** Approved: a replace-only preparation step applies the document's
  `control_smoothing` (absent = the session default), with no new wire edit. Applied in D2.
- **Second review (root, 2026-10-15).** Both exports are approved
  (`miso_engine_web_v1_replace_document_ptr`, `miso_engine_v1_submit_document_command`), with the
  frame naming the document by length and SHA-256, and `SessionState::command_with_document` with a
  borrowed document. Freeing the browser's uncharged boot-document copy at the first command is
  kept: gate 8 records the browser's retained bytes before and after that first command. Gate 8
  adds one case: on a page without cross-origin isolation, the 1 MiB replace runs in the worklet's
  message handler, so it is recorded as a counted blocking rebuild (the documented exception), and
  `process()` still allocates and frees nothing (the render-locked count stays 0).

## Amendment (controls coordinator, batch 15a fold, root ruling 2026-10-16): the split

**#1386 is now part (a), core and C ABI.** The browser transport moves to the placeholder
*Export the browser session replace transport* (#1697, `.github/ISSUE_SPECS/1697-export-the-browser-session-replace-transport.md`).

- **(a) stays here.** D1 (`SESSION_REPLACE` `0x0013` in `crates/protocol`), D2 (the diff in
  `crates/control-plane/src/document.rs`), D3 (`SessionState::command_with_document`, the one core
  path both hosts call), D4, D5, D6 and D7's C ABI half
  (`miso_engine_v1_submit_document_command`). The browser keeps calling `SessionState::command`,
  which is `command_with_document(request, &[], ...)`, so a `SESSION_REPLACE` frame sent through
  `miso_engine_web_v1_control_command` today gets D4's `session.replace.document_length` refusal
  through the same core path. That is the both-host core path of (a).
- **#1697 takes** D7's browser half (`miso_engine_web_v1_replace_document_ptr`, the staging
  consumption in `control_command`, the worklet message's `document` field), both `.d.ts` twins,
  the ABI layout mirrors of that export, gate 6 and gate 8's browser half.
- **Gates of (a).** Gates 1-5 and 7 as filed. Gate 6 becomes a C ABI and browser parity row for
  the refusal only: the same `SESSION_REPLACE` frame with no document gives the same reply bytes
  (request ID aside) on both hosts, and changes nothing. Gate 8 keeps its C ABI half: a document of
  more than 64 KiB and the large live replace through `miso_engine_v1_submit_document_command`,
  with 0 retained bytes after the reply. Gate 7's "the ABI layout mirrors move" is #1697's.
- **Authorized paths of (a).** As filed, without `hosts/host-web/src/{ffi.rs,lib.rs}`, the
  worklet, the `.d.ts`, the artifact digest, `scripts/check-web-audioworklet.sh`, the ABI layout
  tools and mirrors and `scripts/test-web-audioworklet.mjs` (all #1697's). Added: see the feature
  bit below. `hosts/host-web/tests/control_parity.rs` stays, for gate 6's refusal row.

**Corrections to the re-brief (it predates #1310, which ships in batch 15a).**

- **D4, last bullet.** "Today `BACKPRESSURE`" is void. Since #1310, a structural replace while a
  candidate is pending gets what a structural transaction gets: it supersedes the unadopted
  candidate by compare and swap. Gate 4 adds that case: a replace while a candidate is pending
  commits once and its plan supersedes the candidate, as #1310's transaction case does.
- **Non-goal "No supersession of a pending candidate (#1310)"** (`:433`) is void. The replace
  inherits #1310's supersession through the shared path and adds no supersession code of its own.
- **Non-goal "No browser replace export"** (`:429`) contradicted D7 and its root approval. It now
  reads: no browser export in (a); the browser export is #1697's.
- **The C export needs a feature bit** (#1317 D2: each added symbol gets its own bit, the next
  free one when it merges). `miso_engine_v1_submit_document_command` gets
  `MISO_ENGINE_V1_FEATURE_SESSION_REPLACE`, and `FEATURE_MASK` grows to include it. Added to
  Authorized paths: `crates/capi/src/abi.rs` (the bit and the mask, and its unit-test pins at
  `abi.rs:548-553`), `crates/capi/src/ffi.rs` (also `version_and_capabilities_are_exact`,
  `:1558`), `crates/capi/include/miso_engine_v1.h` (the bit and `MISO_ENGINE_V1_FEATURE_MASK`,
  `:265`) and `crates/capi/tests/c/abi_smoke.c` (the mask pin `:23` and a bit check, no longer
  conditional). The mask value is the one at merge (#1316 may take a bit first).
- **D3, the identical document.** The re-brief answers `model_only` with no commit. That conflicts
  with the wire: `EditPath::ModelOnly` says "the transaction committed"
  (`crates/protocol/src/message_wire.rs:161-165`), and the registry's `model_only` is a commit
  that render need not act on. Proposed: an identical document is a refusal-free no-op with a
  dedicated path, `EditPath::Unchanged = 5` (registry row, decode, corpus, schema re-pin in gate
  7): OK, `applied_operations` 0, the unchanged revision, no `SESSION_COMMITTED`, no candidate.
  The alternative is to commit it (`model_only`, revision + 1, one `SESSION_COMMITTED` with 0
  operations). Gate 3's identical case and gate 6 follow the ruling.
  ROOT DECISION (2026-10-16): an identical-document replace replies with a new `EditPath::Unchanged` (5): OK, 0 operations, unchanged revision, no event. Nothing changed, so the revision does not move. The `edit_path` vocabulary (protocol, registry, the vocabulary gates), the SDK decoder and the parity row all include 5.

## Evidence and decisions, tranche a1 (attempt 1, 2026-10-24)

Scope note `opt/scope/1386.md` governs; a1 is the core (protocol, control plane, corpus, docs).
a2 (C ABI export, feature bit `1 << 11`, capi tests, the parity refusal row, the audit frame) is
still open.

- **Hook shape.** `ProtocolControllerConfig::document_replace: Option<DocumentReplace>`, a plain
  `fn` plus the boot's `maximum_document_bytes` (0 unbounded). The hook gets the committed
  `SessionStore`, so it can compile the document under the store's own caps: the document is
  validated exactly as a boot validates it, and both sides of the diff are normalized models.
  `SessionStore::caps()` is added for that.
- **Bits, not `PartialEq`.** Entities are compared by a byte stream of the session's own model
  walk (`VisitModel`, floats by `to_bits`), not by canonical JSON text.
- **Smoothing.** The diff emits `SetControl` rows for an explicit arm as D2 says, and
  `SessionStore::prepare_replacement` then sets the document's `control_smoothing` whole, so both
  arms and both directions land exactly. A smoothing-only change commits with 0 operations
  (`model_only`).
- **One added diagnostic code.** A document that is not UTF-8 is `VALIDATION_FAILED`
  `session.replace.document_utf8` (D4 lists "does not parse" with the parser's diagnostics; the
  parser takes `&str`, so this case needs its own code).
- **No document on the controller's other paths.** `ProtocolController::process` and the one-call
  `process_command_frame_into` carry no document, so a `SESSION_REPLACE` there gets the
  `document_length` refusal from the same hook.
- **Paths outside the a1 list, needed by the change:** `crates/protocol/tests/set_control_frames.rs`
  (one `EditPath::Unchanged` match arm), `crates/conformance/src/main.rs` and
  `crates/conformance/tests/mutation_million.rs` (the corpus frame count 46 -> 48, part of the
  re-pin).
- **Re-pin.** `COMPLETE_SCHEMA_HASH` `5cab2cf3b4e4767f` -> `c85b2a0dff5d2a92`, 48 frames
  (`command.session_replace`, `response.session_replace` with `unchanged`, and `0x0013` in the
  capabilities response), in every spelling.
- **Open for a2 (blocks merging a1 alone).** The C ABI endpoint is built by `compile_children`,
  so it now advertises `0x0013`. Three tests in `crates/capi/src/runtime/tests.rs` pin the
  capabilities reply bytes and fail until re-pinned:
  `all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes`,
  `capi_controller_dispatches_every_advertised_command_family` and
  `exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact`. That file is on
  neither tranche's path list; a2 needs it added. host-web's suite passes unchanged.
