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
  edits* (#1348) after every Worker message and on a one-quantum timer. In `single` mode the
  worklet's control handler runs it, never inside `process()`: after every control message, and
  on the `miso.service.v1` tick that the main-realm host posts from its one pinned `setInterval`
  (#1381 D6). #1381 pairs the render-side meter, observation and spectrum readers at adoption in
  Rust (its D3). #1332 D7 creates the two ports this slice uses: the page-to-control-plane port
  and the control-plane-to-worklet `MessageChannel` port. *Admit browser live edits in the Worker
  through the committed model* (#1382) moves live commands into the Worker over the first.
- The exports come from *Export transaction apply and anchored seek from the browser engine
  module* (#1293). The `replace` export is *Diff a replacement document against the committed
  model and export replace from the browser engine module* (#1386). The service and watermark
  exports come from #1381: `miso_engine_web_v1_service(handle)` runs #1348's service step, and the
  watermark-and-counters export copies the full watermark record (`SessionState::watermark`) and
  the control counters. The watermark's status words in the worklet are *Publish the
  applied-revision watermark in the browser status* (#1349).
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
- **D3. Watermark.** After each call of `miso_engine_web_v1_service(handle)` (#1381; scheduled
  as #1381 D6 says), the control half reads the watermark through #1381's watermark-and-counters
  export; when it advanced, it posts `miso.watermark.v1 { tag, revision, sample, outcome }` to the page. Posts coalesce to
  the latest value; when one post replaces an unsent one, its `outcome` is the OR of both, so no
  outcome flag is lost (D15-17). `miso.status.v1` replies already carry `appliedRevision`,
  `appliedSample` and `appliedOutcome` from #1349's status words (#1349 D2a); this slice adds
  none.
- **D4. Plan shape, staged outside `process()`.** When a `rebuild` commits, the control half posts
  `miso.plan-shape.v1 { tag, revision, supersedes, trackCount, submixCount, submixIds,
  meterHeaderPointer, meterFramePointer, meterFrameCapacity }` to the render worklet on #1332 D7's
  control-plane-to-worklet port; this slice opens no port of its own. `supersedes` is `true` when
  the commit withdrew an unadopted candidate (*Supersede an unadopted candidate plan by
  compare-and-swap*, #1310 D1 `Withdrawn` or `Returned`). In `single` mode the control handler and
  the render worklet are one realm, so it is a direct call to the same staging function.
  - The worklet's message handler builds the complete view set and meter message object for that
    shape and stages it, keyed by `revision`. First it applies D5's install rule against the
    current watermark, so a set whose plan render already took (#1310 D1 `Taken`: that block has
    returned and written its watermark before the handler can run) is installed, not dropped. A
    `supersedes` set then drops every staged set with a lower revision: their plans never render.
    So at most one older set waits beside the new one.
  - A staged set is cut over the current buffer, and a growth (#1332 D5) rebuilds it with the
    current set.
  - **Every commit is announced.** After every other commit the control half makes (`live` or
    `model_only`, from D1 or from #1382's live-edit messages), it posts
    `miso.committed.v1 { tag, revision }` on the same port, after the export returns; in `single`
    mode it is the same direct call. The worklet's handler keeps `knownRevision`, the highest
    revision announced by either message (boot sets it to the boot revision). One port delivers in
    order, so every revision up to `knownRevision` that changed the plan's shape has a staged or
    installed set.
- **D5. Switch at adoption, by revision.** In `process()`, after a successful render, the worklet
  reads the watermark revision `W` from the status view (#1349: a per-instance record, not a
  per-plan one).
  - It installs the newest staged set whose revision is at most `W`, by assignment only (no view,
    array or object is built), and drops the staged sets below it.
  - If `W > knownRevision`, an announcement is late, and the worklet cannot tell whether a plan of
    another shape renders. It does not call `meter_poll` and reads no meter header or frame for
    that block, and increments `meterShapeWaits`, reported in telemetry. A view of the previous
    plan's companions is never read once a newer plan may render: that plan is retired and
    reclaimed in the Worker (#1381 D3). The meter windows wait in their bounded queues (a queue
    that overflows counts its loss as today), and the first block after the late message arrives
    polls through the right set. The late handler installs its set directly when `W` already
    covers it.
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

- No Rust change: every export used here comes from #1293, #1381 or #1386. No SDK API (*Apply session transactions from the browser SDK*, #1296). No real-browser
  leg (#1295). No growth handling (#1332).

## Objective gates

1. **Protocol.** A well-formed `miso.apply.v1` calls `edit_ptr` then `apply` and replies with the
   outcome record's fields; `miso.replace.v1` calls `replace`. Each malformed shape (extra field,
   missing field, non-`Uint8Array` payload) is refused and calls no export.
2. **Watermark coalescing.** Two advances observed before the page drains the first post produce
   one post whose `revision` is the later one and whose `outcome` is the OR of both.
3. **Switch at the adoption block.** With fake exports whose watermark covers revision R at block
   k: the meter message of block k-1 has the old track count and the one of block k the new count.
   A shape message for R' staged before R's, when R's message has `supersedes`, is never
   installed.
4. **Late announcement.** The fake watermark covers a rebuild R before R's shape message
   arrives: for those blocks the worklet calls no `meter_poll`, reads no meter header (the fake
   records every read of the old set's header and frame, and there is none), and
   `meterShapeWaits` counts them exactly. After the handler runs, the next block polls and posts
   with R's shape. The same with a late `miso.committed.v1` for a live revision: polling waits and
   then resumes with the unchanged shape. A `supersedes` shape message drops an older staged set;
   one without it keeps it, and a watermark that covers the older revision for one block installs
   it before R's.
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
- Gate 4: a worklet that detects a late shape through the old views reads a retired plan's
  header, which the Worker may already have reclaimed, and posts a frame of the wrong shape; one
  that treats every unknown revision as unchanged posts it too; one that drops staged sets without
  `supersedes` skips a plan render adopted. Each turns it red.
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
- *Move browser source submission and seeks into the Worker* (#1387).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
