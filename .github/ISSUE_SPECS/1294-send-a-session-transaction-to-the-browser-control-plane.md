# Send a session transaction to the browser control plane

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-10, D15-11, D15-17).
Formerly slice B5 of *Swap a rebuilt plan without an audio gap* (#1269).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A page sends a session transaction (or a whole document, diffed into one transaction) to the
running engine's control plane and gets `{result, revision, path, diagnostic}` back as soon as it
commits. The page is told each time the applied-revision watermark advances. The render worklet
keeps rendering through the swap, and its meters switch to the successor's shape at the exact block
render adopts it. The message protocol is internal to the SDK (D15-11).

Decision 15 supersedes this issue's old shape: the replacement no longer runs inside the
AudioWorklet between two `process()` calls. The control plane runs in a Worker (D15-10), and the
worklet only renders and swaps.

## Context

- The worklet processor (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, class at `:242`)
  boots in `initialize` (`:275`) and builds fixed meter views once for the booted shape
  (`:763-830`): one view per section over the meter frame, sized by the track and submix counts read
  from the meter header (`miso_engine_web_v1_meter_header_ptr`, `hosts/host-web/src/ffi.rs:3952`),
  plus one reused `miso.meter.v1` message object. The frozen render-callback policy forbids building
  a view inside `process()` (comment at `:792`; checked by `scripts/check-web-audioworklet.sh:531`).
- `process()` (`:1813`) renders, copies the output and posts meters. Its memory-buffer check
  (`:1823`) and the one in `receive` (`:1005`) are made growth-tolerant by *Run the browser control
  plane in a Worker and keep the AudioWorklet render-only* (#1332), which also owns re-cutting views
  after a growth the Worker causes.
- Messages are dispatched in `receive` (`:983`) by `tag` with exact field sets. The host wrapper
  (`miso-engine-v1-audio-worklet-host.js`) sends requests through `#request` (`:1203`); `status()`
  is at `:1294`. `scripts/check-sdk-generated.sh:64-77` requires `sdk/src/browser/shipped-host.d.ts`
  to equal `miso-engine-v1-audio-worklet-host.d.ts`.
- #1332 selects the mode once (`worker` when `crossOriginIsolated`, else `single`, its D1) and
  adds the control Worker `hosts/host-web/web/miso-engine-v1-control-worker.js` (its D2). Its
  Worker and worklet messages are internal and stay out of `shipped-host.d.ts` (its D7). *Swap and
  retire browser plans through the Worker's service loop* (#1381) puts the control plane in the
  Worker and runs the service step of *Add miso_engine_v1_service for bounded control work between
  edits* (#1348) after every Worker message and on a one-quantum timer; in `single` mode the
  worklet runs it after a block that adopted a successor (#1381 D4). #1381 rebinds the render-side
  meter, observation and spectrum companions at adoption in Rust (its D3); it adds no
  Worker-to-worklet message channel. *Admit browser live edits in the Worker through the committed
  model* (#1382) moves live commands into the Worker over #1332's page-to-control-plane port.
- The exports come from *Export transaction apply and anchored seek from the browser engine
  module* (#1293). The `replace` export is *Diff a replacement document against the committed
  model and export replace from the browser engine module* (#1386). The watermark is in the
  worklet's status words, and the Worker reads the full record through `SessionState::watermark`
  (*Publish the applied-revision watermark in the browser status*, #1349, D2 and D3).
- The hermetic harness `scripts/test-web-audioworklet.mjs` (run by
  `scripts/test-web-audioworklet.sh`) drives the worklet and wrapper against fake exports, so it
  proves message, ordering and view logic, not rendered bits. Real bits are *Qualify a structural
  browser edit in real browsers* (#1295).

## Decisions frozen for this slice

- **D1. Edit messages.** `miso.apply.v1 { tag, requestId, transaction }` and
  `miso.replace.v1 { tag, requestId, document }`, each a `Uint8Array`, with exact field sets, sent
  to the control half: the Worker in `worker` mode, the worklet's control handler in `single`
  mode. The handler stages the bytes through `miso_engine_web_v1_edit_ptr`,
  calls `miso_engine_web_v1_apply` or `miso_engine_web_v1_replace` (#1386), and replies
  `miso.edit.v1 { tag, requestId, result, revision, path, diagnostic }` from the outcome record
  (`revision` a `bigint`, `diagnostic` the UTF-8 text, empty on success). Malformed messages are
  refused without calling an export. `miso.document.v1 { tag, requestId }` replies with
  `{ tag, requestId, revision, document }`, a copy of the committed canonical document read through
  `miso_engine_web_v1_committed_document_ptr` and `_bytes`.
- **D2. Anchored seek message.** `miso.seek-at.v1 { tag, requestId, sourceId, generation,
  sourceFrame, anchorSample }`, handled by the control half, which owns the source producers once they move to the Worker
  (*Move browser source submission and seeks into the Worker*, #1387), through
  `miso_engine_web_v1_source_seek_at`. Feeds that use the shared PCM ring do not use it (*Feed and
  retire the sources a browser edit adds or removes*, #1297).
- **D3. Watermark.** After each #1348 service step (run as #1381 D4 schedules it), the control
  half reads `SessionState::watermark` (#1349 D3); when it advanced, it posts `miso.watermark.v1 { tag, revision, sample, outcome }` to the page. Posts coalesce to
  the latest value; when one post replaces an unsent one, its `outcome` is the OR of both, so no
  outcome flag is lost (D15-17). `miso.status.v1` replies gain `appliedRevision`, `appliedSample`
  and `appliedOutcome`, read from #1349's status words.
- **D4. Plan shape, staged outside `process()`.** When a `rebuild` commits, the control half posts
  `miso.plan-shape.v1 { tag, revision, trackCount, submixCount, submixIds, meterHeaderPointer,
  meterFramePointer, meterFrameCapacity }` to the render worklet on a dedicated `MessageChannel` this slice creates at boot: the main-realm
  wrapper posts one port to the Worker and transfers the other over the worklet node's port (in
  `single` mode, a direct call). Its message handler builds the
  complete view set and meter message object for that shape and stages it, keyed by `revision`. A
  newer staged set replaces an older one (a superseded candidate's set is dropped there).
  A staged set is cut over the current buffer, and a growth (#1332 D5) rebuilds it with the
  current set.
- **D5. Switch at adoption.** In `process()`, after a successful render, the worklet reads the
  watermark revision from the status view. When it covers the staged revision, the worklet swaps
  its view references to the staged set (assignment only; no view, array or object is built). If
  the meter header read in `process()` disagrees with the current set's track or submix count (a
  late shape message), the block posts no meter frame and increments a `meterShapeLosses` counter
  reported in telemetry; the late handler then installs the set directly.
- **D6. Wrapper.** The host wrapper gains `apply(transaction)`, `replace(document)`,
  `seekSourceAt(request)`, `committedDocument()` and a `watermark` event, each marked `@internal` in the declarations (the
  SDK is the public surface). Copy the declarations to `sdk/src/browser/shipped-host.d.ts`.
- **Superseded.** The old D2 refresh after an in-worklet replacement, the old D3 feed re-cut (now
  #1332's growth handling) and the old D5 audio-thread timing (preparation no longer runs on the
  audio thread when isolated) are superseded by decision 15 D15-10.

## Deliverables

1. D1-D5 in the worklet and in `miso-engine-v1-control-worker.js`; D6 in the wrapper and its
   declarations.
2. Hermetic tests in `scripts/test-web-audioworklet.mjs`.

## Authorized paths

- `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, `miso-engine-v1-audio-worklet-host.js`,
  `miso-engine-v1-audio-worklet-host.d.ts`, `miso-engine-v1-control-worker.js`
- `sdk/src/browser/shipped-host.d.ts`
- `scripts/test-web-audioworklet.mjs`, `scripts/test-web-audioworklet.sh`

## Non-goals

- No Rust change. No SDK API (*Apply session transactions from the browser SDK*, #1296). No real-browser
  leg (#1295). No growth handling (#1332).

## Objective gates

1. **Protocol.** A well-formed `miso.apply.v1` calls `edit_ptr` then `apply` and replies with the
   outcome record's fields; `miso.replace.v1` calls `replace`. Each malformed shape (extra field,
   missing field, non-`Uint8Array` payload) is refused and calls no export.
2. **Watermark coalescing.** Two advances observed before the page drains the first post produce
   one post whose `revision` is the later one and whose `outcome` is the OR of both.
3. **Switch at the adoption block.** With fake exports whose watermark covers revision R at block
   k: the meter message of block k-1 has the old track count and the one of block k the new count.
   A shape message for a superseded R' staged before R's is never installed.
4. **Late shape.** Watermark covers R before R's shape message arrives: no meter frame is posted
   for those blocks, `meterShapeLosses` counts them exactly, and meters resume with the new shape
   after the handler runs.
5. **Refusal.** A refused apply replies with the typed result and diagnostic, stages no shape and
   leaves the sticky result unchanged.
6. **Render-callback policy and declarations.** `scripts/check-web-audioworklet.sh` (its process
   policy) and `bash scripts/check-sdk-generated.sh <artifacts>` pass.
7. Commands:
   - `bash scripts/test-web-audioworklet.sh`
   - `rm -rf target/ci/h1294 && mkdir -p target/ci/h1294/a target/ci/h1294/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/h1294/n target/ci/h1294/a && bash scripts/check-web-audioworklet.sh target/ci/h1294/a target/ci/h1294/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh target/ci/h1294/a` and `bash scripts/check-sdk-types.sh`

## Test value

- Gate 1: a handler that calls `apply` on a malformed message, or replies before the outcome record
  is written, turns it red.
- Gate 2: coalescing that keeps only the last post's flags drops a `transition_fallback` the page
  must see; it turns red.
- Gate 3: a worklet that switches views at commit instead of adoption, or installs a superseded
  candidate's shape, posts frames of the wrong shape; it turns red.
- Gate 4: a worklet that posts a frame read with the old views over the new layout reports the
  wrong strip's level; it turns red.
- Gate 5: a refused edit that still stages a shape switches meters to a plan that never comes.

## Dependencies

- *Export transaction apply and anchored seek from the browser engine module* (#1293).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1382).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Diff a replacement document against the committed model and export replace from the browser
  engine module* (#1386).
- *Move browser source submission and seeks into the Worker* (#1387) (D2 only).
