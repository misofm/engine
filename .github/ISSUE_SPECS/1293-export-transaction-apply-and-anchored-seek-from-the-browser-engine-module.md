# Export transaction apply and anchored seek from the browser engine module

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-12, D15-17).
Formerly slice B4 of *Swap a rebuilt plan without an audio gap* (#1269).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The shipped engine module exports the browser's one edit API to its control half: apply a session
transaction, read the outcome `{result, revision, path}` with its diagnostic, and read the
committed document. It also exports the anchored seek and the per-source seek report. Every
published mirror of the export list (ABI layout, SDK, checkers) follows. Decision 15 D15-11
supersedes the old `replace_document_ptr` / `replace` pair. The document convenience is
*Diff a replacement document against the committed model and export replace from the browser engine
module* (#1386), which uses this slice's staging and outcome record.

## Context

- Today `miso_engine_web_v1_document_ptr` refuses while a host is live
  (`hosts/host-web/src/ffi.rs:3500-3511`), so no export takes an edit while audio plays. A live
  host's diagnostic is `AudioWorkletEngineHost::diagnostic` (`hosts/host-web/src/lib.rs:2872`), read
  through `miso_engine_web_v1_buffer_ptr(handle, BUFFER_DIAGNOSTIC)` (`ffi.rs:599`).
- `miso_engine_web_v1_source_seek` (`ffi.rs:3789`) takes the source ID through the
  `BUFFER_SOURCE_ID` staging. Its host method `seek_source` (`lib.rs:3175`) calls
  `plan.prepare_source_seek` after the producer's seek (`lib.rs:3193-3203`). There is no browser
  `seek_at` export.
- **The #1293 D4 trap** (adversary round 1, section D): an anchored-seek export copied from the
  plain one would report every accepted future anchor as `RESULT_INTERNAL`, because
  `PcmSourceConsumer::prepare_seek` returns `false` while an anchored seek is held
  (`crates/source/src/lib.rs:1043-1065`). Its fix and regression test belong to *Test held seeks
  across swaps and supersession, and add a seek to audit capi* (#1319). This slice does not
  duplicate them.
- On the C ABI the anchored seek is a control-thread call on the source producer only
  (`crates/control-plane/src/control.rs:1671`); *Extract the C ABI control plane into a portable
  crate both hosts call* (#1309) moves it into the shared crate.
- The export list is published and checked in several places: `scripts/check-web-audioworklet.sh`
  (`expected_exports`, `:203`), `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS`, `:135`),
  `scripts/check-abi-layout-v1.py` (e.g. `:191`; run by `check-web-audioworklet.sh:562`),
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`, and
  `scripts/check-sdk-generated.sh`.
- *Replace the running browser session in the Rust host* (#1290) gives the control half
  `apply(transaction) -> Result<{revision, path}, refusal>` (its D1); in `single` mode it runs on
  the worklet thread (its D6).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) sets the
  modes: `worker` (control half in the Worker) and `single` (one instance in the worklet).
  *Swap and retire browser plans through the Worker's service loop* (#1381) puts
  `control_plane::SessionState` in the Worker and boots it with the producers there (its D1),
  preparing through the browser's preparer from *Prepare through an adapter-supplied preparer in
  the control-plane crate* (#1400).
  *Admit browser live edits in the Worker through the committed model* (#1562) moves live commands
  there.
- **Source producers.** #1332 D3 item 6 keeps source submission and seeks on the worklet thread,
  but #1290 D4 moves the persisting producers into the candidate on the control thread at commit,
  and #1381 D1 hands the Worker's producers to `SessionState`, which prepares through #1400's
  preparer. Only the C ABI shape is consistent: producers belong to the control half. *Move
  browser source submission and seeks into the Worker* (#1387) makes that move; this slice needs
  it (see Dependencies).
- The path codes come from *Report each transaction's edit path in its response* (#1313).
  Completion is the watermark, in the browser status (*Publish the applied-revision watermark in
  the browser status*, #1349); it is not part of this record. The control half's two other exports,
  `miso_engine_web_v1_service(handle)` (the service step) and the watermark-and-counters export,
  come from #1381 with their mirrors; this slice adds neither.
- *Anchor every seek on the plan's source-read clock* (#1316) adds a per-ring seek report
  (`generation`, `first_sample`, `source_frame`, cumulative underrun and held frames) behind
  `SourceControlSet::seek_report(id)` and assigns its browser export to this slice.

## Decisions frozen for this slice

- **D1. Staging.** `miso_engine_web_v1_edit_ptr(handle, len) -> u32` returns a pointer to a
  staging buffer for one transaction frame or one document (#1386) of `len` bytes, bounded by
  `MAXIMUM_DOCUMENT_BYTES`, or 0 with a typed result. It may allocate. Every export in this slice
  runs on the control half (the Worker in `worker` mode, the worklet in `single` mode). On an
  instance that holds no control half each refuses with `RESULT_REFUSED_LIFECYCLE` (a pointer
  export returns 0).
- **D2. Apply.** `miso_engine_web_v1_apply(handle, len) -> u32` decodes the staged bytes as one
  `SESSION_TRANSACTION_APPLY` frame (`crates/protocol/src/session_wire.rs`; the bytes the C ABI
  accepts) and runs #1290's `apply`. It returns as soon as the transaction commits (D15-17) and
  never waits for render, a swap or a warm adoption.
- **D3. Outcome.** `miso_engine_web_v1_edit_outcome_ptr(handle) -> u32` points at a fixed
  `#[repr(C)]` record rewritten by every apply (and by #1386's replace): `struct_size u32`,
  `abi_version u32`, `result u32`, `path u32` (#1313's codes), `revision u64` (the committed
  revision; unchanged on a refusal). A refusal also writes the host diagnostic and leaves the
  committed model, the revision, the running plan and the sticky state unchanged.
- **D4. Committed document.** `miso_engine_web_v1_committed_document_ptr(handle) -> u32` and
  `miso_engine_web_v1_committed_document_bytes(handle) -> u32` expose the canonical snapshot of the
  committed model (`SessionStore::canonical_snapshot`, `crates/protocol/src/model.rs:900`), valid
  until the next commit.
- **D5. Anchored seek.** `miso_engine_web_v1_source_seek_at(handle, source_id_bytes, generation,
  source_frame, anchor_sample) -> u32`, the source ID through `BUFFER_SOURCE_ID`, with the plain
  seek's argument checks. It calls the shared crate's producer-side `seek_at` (#1309), never
  `plan.prepare_source_seek`. A held seek's result codes are those #1319 fixes; an accepted future
  anchor returns `RESULT_OK`; an anchor that is not a quantum multiple returns
  `RESULT_INVALID_ARGUMENT`.
- **D6. Seek report.** `miso_engine_web_v1_source_seek_report(handle, source_id_bytes) -> u32`
  copies #1316's report for the staged source ID into a fixed `#[repr(C)]` record at
  `miso_engine_web_v1_source_seek_report_ptr(handle)`: `struct_size u32`, `reserved0 u32`,
  `generation u64`, `first_sample u64`, `source_frame u64`, `underrun_frames u64`,
  `held_frames u64`. A busy read returns `RESULT_BACKPRESSURE`; an unknown source
  `RESULT_INVALID_ARGUMENT`.
- **D7. Mirrors.** Add the eight exports to every list in the context, regenerate the SDK mirrors
  with the repository's generators, and keep every checker's self-test passing. Both records join
  the ABI layout document with their field offsets.

## Deliverables

1. D1-D6 in `hosts/host-web/src/ffi.rs` and `lib.rs`.
2. D7 in the checkers, the parameter-metadata generator and the generated SDK files.
3. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py` and its
  self-test fixture, `tools/parameter-metadata/src/abi_layout.rs`
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`,
  `scripts/check-sdk-generated.sh`

## Non-goals

- No document diff or `replace` export (#1386). No control-plane logic (#1290, #1309). No
  service or watermark export: `miso_engine_web_v1_service(handle)` and the watermark-and-counters
  export are #1381's, and the status words are #1349's. No Worker or worklet JavaScript (*Send a session
  transaction to the browser control plane*, #1294). No SDK API (*Apply session transactions from
  the browser SDK*, #1296).
- No change to the plain seek export's semantics; its anchoring and report are #1316's.

## Objective gates

1. **Apply through the exports.** A native test calls the `extern "C"` functions on the control
   half: boot A (four tracks), stage and apply a transaction that adds a muted track with an EQ
   insert. The outcome record reads `RESULT_OK`, path `rebuild`, revision +1, and the
   committed-document export reads the canonical text of A plus that track. A transaction that
   changes only one fader reads path `live`.
2. **Refusals change nothing.** A malformed frame, a stale expected revision and a frame over
   `MAXIMUM_DOCUMENT_BYTES` each refuse with their typed result and a non-empty diagnostic; the
   outcome's revision, the committed document and the next rendered blocks equal a run without the
   call.
3. **Anchored seek.** `source_seek_at` with an anchor two quanta ahead returns `RESULT_OK`; the
   source renders silence until the anchor and frame F from it on. An unaligned anchor returns
   `RESULT_INVALID_ARGUMENT`.
4. **Seek report.** After gate 3's seek renders, `source_seek_report` reads `(generation,
   first_sample = anchor, source_frame = F)`; an unknown source ID returns
   `RESULT_INVALID_ARGUMENT`.
5. **Role.** Each export called on an instance without a control half refuses with
   `RESULT_REFUSED_LIFECYCLE` and changes nothing.
6. **Mirrors agree.** `python3 -B scripts/check-abi-layout-v1.py` with its self-test,
   `bash scripts/check-sdk-generated.sh <artifacts>` and the parameter-metadata `--check` pass,
   and each fails if one list omits one export.
7. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `rm -rf target/ci/h1293 && mkdir -p target/ci/h1293/a target/ci/h1293/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/h1293/n target/ci/h1293/a && bash scripts/check-web-audioworklet.sh target/ci/h1293/a target/ci/h1293/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh target/ci/h1293/a`, `bash scripts/test-web-audioworklet.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
     `bash scripts/check-workspace-policy.sh`
   - The shipped artifact changes: report its new digest and size in the PR.

## Test value

- Gate 1: an apply export that waits for render or writes the wrong transaction's path, or a
  committed-document export that still serves the booted text (the SDK would bind live controls to
  a session that no longer runs), turns it red. No Rust-level test reads these records.
- Gate 2: a refusal that wrote the outcome's revision or moved the committed model turns it red.
- Gate 3: an export wired through `prepare_source_seek` (the D4 trap's shape) returns
  `RESULT_INTERNAL` for the future anchor. #1319 owns the shared-path regression; this gate defends
  the browser wiring only.
- Gate 4: an export that reads another source's record, or its words in the wrong order,
  misaligns every stem the SDK starts from it (#1297).
- Gate 5: an edit export that runs on the render half would allocate on the audio thread.
- Gate 6: a mirror that misses one export lets the SDK call a name the module lacks.

## Dependencies

- *Replace the running browser session in the Rust host* (#1290).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Report each transaction's edit path in its response* (#1313).
- *Anchor every seek on the plan's source-read clock* (#1316).
- *Test held seeks across swaps and supersession, and add a seek to audit capi* (#1319).
- *Move browser source submission and seeks into the Worker* (#1387).

**Decision 17 amendment (2026-10-13).** #1562 adds one generic command-frame entry on the control half (`miso_engine_web_v1_control_request_ptr`, `_control_command`, `_control_response_ptr`, `_control_response_bytes`) that runs `SessionState::command` for any frame, transactions included. D2 (`miso_engine_web_v1_apply`) and D3 (the outcome record) are dropped: a transaction is applied through that entry and its outcome is the response frame (revision, `edit_path`, diagnostics). D4-D7 (committed document, anchored seek, seek report, mirrors) and D1's staging for #1386's document replace stay.

**Decision 17 amendment (2026-10-13), second line.** D4 is withdrawn too: the committed document reaches the browser as `SESSION_SNAPSHOT_GET` frames through #1562's generic command entry, with the same bytes as on the C ABI, and the SDK reads it with `engine.committedDocument()` (#1571). No `committed_document_ptr` or `committed_document_bytes` export is added. D5-D7 (the anchored seek, the seek report and the mirrors) stay.

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.**
- `miso_engine_web_v1_document_ptr` is `hosts/host-web/src/ffi.rs:2959`; its live refusal is
  `:2960-2970`.
- `AudioWorkletEngineHost::diagnostic` is `hosts/host-web/src/lib.rs:3175`.
  `miso_engine_web_v1_buffer_ptr` is `ffi.rs:2996`, and its `BUFFER_DIAGNOSTIC` arm `:723`.
- `miso_engine_web_v1_source_seek` is `ffi.rs:3251`. Its host method `seek_source` is
  `lib.rs:3322`. It calls `ready.control.seek_unserviced` and then `plan.prepare_source_seek`
  (`:3340-3352`).
- `PcmSourceConsumer::prepare_seek` is `crates/source/src/lib.rs:1049` (doc `:1040-1048`). The
  D4 trap still holds.
- The producer-side anchored seek is `SessionState::seek_at`,
  `crates/control-plane/src/control.rs:3028` (#1309 moved it). It runs the service step first.
- The mirrors: `expected_exports` is `scripts/check-web-audioworklet.sh:274`, and its call of
  `check-abi-layout-v1.py` is `:635`. `EXPORTS` is `tools/parameter-metadata/src/abi_layout.rs:125`
  (102 names) and `scripts/check-abi-layout-v1.py:120`.
- #1562's generic entry exists: `control_request_ptr` (`ffi.rs:3347`), `control_command`
  (`:3366`), `control_response_ptr` (`:3399`) and `control_response_bytes` (`:3412`). `service` is
  `:3336`.

**The control half today.** The browser runs one instance. Since #1381a it holds
`control_plane::SessionState` (`hosts/host-web/src/lib.rs:6-16`). Today, "no control half" means
a host that is not ready: `control_event` returns `RESULT_REFUSED_LIFECYCLE` for it
(`lib.rs:2397-2415`). That code equals `RESULT_WRONG_STATE` (`lib.rs:146`). Gate 5 uses that case
until #1332 adds `worker` mode.

**After the two decision 17 amendments.**
- D2, D3 and D4 are dropped. Gates 1 and 2 test only those, so they go too. #1562's gates cover
  apply through the generic entry.
- Deliverable 1 is now D1, D5 and D6. D7's "eight exports" is now four: `edit_ptr`,
  `source_seek_at`, `source_seek_report` and `source_seek_report_ptr`. The one record is D6's. If
  D1 goes (below), three.
- D5 and D6 hold against decision 17. The C ABI has both as C exports:
  `miso_engine_v1_source_seek_at` (header `:425`) and #1316 D6's
  `miso_engine_v1_source_seek_report`. So these exports put the same function on the second host
  (C1). They are not a browser-only feature, and source transport is not a control.

**B15-R8, root decision under owner delegation (2026-10-15): drop D1 from this slice. Re-scope #1386 under C1.** #1386's diff
goes in `control-plane` as a protocol frame through `submit_command` and `control_command` on both
hosts. Its staging is `control_request_ptr`, and its outcome is the response frame. #1386 and #1643
record this in their own amendments. Deliverable 1 is now D5 and D6. D7's exports are three:
`source_seek_at`, `source_seek_report` and `source_seek_report_ptr`. Only D1's staging export
`edit_ptr` goes. D1's other rule stays: each remaining export runs on the control half and refuses
with `RESULT_REFUSED_LIFECYCLE` (a pointer export returns 0) on an instance with no control half.
The finding follows. D1's staging stays only for #1386's document replace. But #1386 (open) also
uses D3's outcome record, which is dropped. And #1386's diff sits in `hosts/host-web` with no C ABI
form, which is a browser-only feature under decision 17 C1. The generic entry's staging
(`control_request_ptr`, bounded by the boot's `maximum_control_frame_bytes`) already carries a
frame. Proposed: drop D1 from this slice. Re-scope #1386 under C1: the diff goes in the core
(`control-plane`) as a protocol frame through `submit_command` and `control_command` on both hosts.
Its staging is `control_request_ptr`, and its outcome is the response frame.

**Hazard on D5.** The SDK PCM feed calls the plain seek from `process()` too: `applySharedSeek` is
"shared by render and the between-block control handler"
(`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:465-470`). That is why `seek_source`
uses `seek_unserviced`. `SessionState::seek_at` runs the service step, which `process()` forbids
(`control.rs:2983-2990`). D1's rule (control half only) holds for `source_seek_at`, so the SDK
calls it from the message handler only (#1294, #1297). A render-side caller would need a
`seek_at_unserviced` in `crates/control-plane`, which is outside this slice's paths.

**Dependencies.**
- Closed: #1309, #1313, #1381 (as #1381a) and #1562.
- In the batch: #1316 and #1319.
- #1290 (open) was needed only for D2. #1313 was needed only for D3. Proposed: remove both.
- #1332 and #1387 are open and not in batch 15/16. In `single` mode, #1381a already routes every
  source export through `SessionState`, and #1387's amendment only moves that routing to the
  Worker. Proposed: make #1332 and #1387 ordering-only, for the `worker`-mode case of gate 5. D5 and
  D6 in `single` mode do not wait for them. Root confirms.
- **B15-R8, root decision under owner delegation (2026-10-15):** #1293's dependencies drop #1290 and #1313. #1332 and #1387
  are ordering-only.

**Other specs that name dropped exports.** #1386 names `miso_engine_web_v1_edit_outcome_ptr`.
#1643 gate uses `miso_engine_web_v1_apply` and cites this slice for it. Root fixes those specs.
Done under B15-R8: #1386 and #1643 each carry an amendment of 2026-10-15.

**Gates and commands.** Every named script and file exists. `build-web-audioworklet.sh
--named-twin`, `check-sdk-generated.sh [DIR]`, the parameter-metadata `--check`
(`tools/parameter-metadata/src/lib.rs:1143`) and `check-abi-layout-v1.py --self-test` are valid.

## Amendment (root ruling, batch 15c, 2026-10-16): D6 owns the browser seek report

- #1316 (batch 15c) shipped the core seek report and its C ABI export,
  `miso_engine_v1_source_seek_report` (struct `miso_engine_v1_seek_report`, 64 bytes; the core
  reads it through `SessionState::seek_report`, which runs the service step first). The browser
  export is this issue's D6, in batch 16. It does not wait for #1332 or #1387, because #1332 waits
  on the owner's iOS test and a parity gap there could wait indefinitely. In `single` mode D6
  ships on its own (B15-R8).
- D6 reads the same core call as the C ABI (thick core, thin clients): no seek-report logic in
  host-web. Its record carries the same fields as the C struct, in the same order and with the
  same meanings (generation in effect, source-read sample and source frame of its first block,
  cumulative underrun and held frames). A busy read is `RESULT_BACKPRESSURE` and an unknown source
  `RESULT_INVALID_ARGUMENT`, as the C ABI's `source.report.busy` and `source.id.unknown`.
- **New gate (parity row).** In `hosts/host-web/tests/control_parity.rs`: a plain seek and a late
  `seek_at` on both hosts at 48 and 96 kHz; after each block, the browser record and the C ABI
  struct are equal field by field, from the seek call to the block that applies it and after.
  Red if either host reports at the call, names a different block, or drops the lateness.
