# Move browser source submission and seeks into the Worker

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-12).
Code anchors verified on `main` at `6fb211594`.

Root ruling for this slice: the C ABI shape governs. Source producers, and therefore submission
and seeks, belong to the control half (the Worker), never to the render worklet.

## Product outcome

On a cross-origin-isolated page, the Worker submits decoded PCM and applies plain and anchored
seeks. It does this through the control plane's source routing, the same routing the C ABI's
control thread uses. The worklet keeps only the ring consumers and calls no source entry point.

The feeds keep working unchanged: the shared MSB1 rings and the `postMessage` feed play the same
bits with no new underruns. A source that a structural edit removes keeps feeding until its
phase-2 swap. Every seek lands where #1316's contract says it does.

On a page that is not isolated (`single` mode), everything stays on the one worklet thread, as
today.

## Context

- **The worklet is the producer today.**
  - `receive` dispatches `miso.source.v1` and `miso.seek.v1`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1009-1012`). It calls
    `miso_engine_web_v1_source_submit` (`:1514`, export `hosts/host-web/src/ffi.rs:3713`) and
    `miso_engine_web_v1_source_seek` (`:1735`, export `ffi.rs:3789`).
  - The SDK's MSB1 feed is a worklet prelude. It wraps the engine processor and drains each
    `SharedArrayBuffer` ring at the top of `process()`. It calls the same two exports
    (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:55-66`, `:386`, `:445`; asset URL
    `sdk/src/assets.ts:34`; ring authority `sdk/src/browser/pcm-ring.ts`).
- **The facade.** `SourceControlSet::submit`, `seek` and `seek_at`
  (`crates/host-core/src/source.rs:152`, `:200`, `:223`) are producer-side calls. Each source has
  one producer thread.
- **The C ABI routes source calls through its control plane.** `newest_providers`, `submit`,
  `seek` and `seek_at` are at today's `crates/capi/src/runtime/control.rs:1488`, `:1501`,
  `:1514` and `:1530`; #1309 moves them into `crates/control-plane`.
- **Removed sources.** *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325) D4
  routes `submit`, `seek` and `seek_at` for a source that the newest committed session removed to
  the plan that still plays it, until the successor is adopted. After that they are refused as
  `source.id.unknown`.
- **Seek contract.** *Anchor every seek on the plan's source-read clock* (#1316):
  - a `seek_at` anchor is a sample of the plan's source-read clock;
  - a plain seek applies at the first block whose render begins after it is popped;
  - each ring publishes a seek report (#1316 D2-D3, facade `seek_report`, D7).
- **Ring headroom.** The default ring hides about 100 ms (`default_source_ring_frames`,
  `crates/host-core/src/prepare.rs:65`).
- **#1332 D3 item 6** keeps submission and seeks on the worklet thread in slice 1, as ordering
  only. **#1381** puts `control_plane::SessionState` (which holds the source providers) in the
  Worker.

## Decisions frozen for this slice

- **D1. Entry points.** In `worker` mode, the Worker calls the control plane's source routing for
  every source operation:
  - `miso_engine_web_v1_source_submit`;
  - `miso_engine_web_v1_source_seek`;
  - the control plane's `seek_at` and the #1316 seek report, natively.

  The worklet's post-boot set (#1332 D3 item 5) drops the source exports. The
  `miso_engine_web_v1_source_seek_at` export (#1293) and the `miso.seek-at.v1` message (#1294)
  come after this slice. They attach to the Worker routing that this slice creates. In `single` mode, the
  worklet's own control handler calls them outside the render-locked window.
- **D2. Messages.** In `worker` mode, `miso.source.v1` and `miso.seek.v1` travel on #1332's
  page-to-control-plane port to the Worker. Their fields and replies are unchanged.
- **D3. The MSB1 drain moves with the producer.**
  - The SDK's feed module is loaded into the control Worker as a module, not as a worklet
    prelude.
  - It drains every attached ring on each service tick of #1381 D4 and on each control message.
  - The drain keeps today's rules: apply a published seek, drop stale generations, stop at
    `RESULT_BACKPRESSURE`, and use the same staging copy and the same export.
  - In `single` mode the worklet prelude stays as it is.
- **D4. Routing.** Source calls go through `control_plane::SessionState`'s source routing, never
  straight to a `SourceControlSet`. A removed source therefore follows #1325 D4 on the browser
  as on the C ABI.
- **D5. Underruns.** A Worker that falls behind empties the ring. That emits zeros plus the
  existing underrun counter (`AGENTS.md`). Held blocks stay apart from underruns (#1318). Nothing
  blocks render.

## Deliverables

1. D1, D2 and D4 in `hosts/host-web/src/` and the control Worker and worklet scripts.
2. D3 in the SDK feed: the Worker-loaded drain module, and the routing in
   `sdk/src/browser/pcm-feed.ts`.
3. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/qualification/`
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` and a new Worker drain module
  beside it, `sdk/src/browser/pcm-feed.ts`, `sdk/src/assets.ts`
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`

## Non-goals

- No change to the MSB1 layout (#1297 amends it), the seek contract (#1316) or removed-source
  routing (#1325).
- No direct page-to-engine-ring writes. The copy from MSB1 into the engine ring stays as it is
  today, now off the audio thread.

## Hazards

- **A plain seek's landing block now depends on Worker timing.** #1316's report tells the host
  where it landed. Hosts that need an exact block use `seek_at`.
- **Each source keeps one producer thread.** Feeding one source from both realms is refused, never
  interleaved.

## Objective gates

1. **Producer off the render thread.** Native host-web test, with the control half on thread A
   and the render half on thread B, nine tracks. A submits every chunk and two plain seeks while B
   renders 64 blocks.
   - B counts `allocations == 0 && frees == 0` around every render call (`bench_support::alloc`).
   - The output is bit-identical to the same feed and seeks run single-threaded, with each seek
     applied at the block its #1316 seek report names.
2. **Anchored seek.** A's control-plane `seek_at` at source-read sample `S`. The seek report names `S`. The
   output from `S` is bit-identical to a fresh boot fed from the target frame.
3. **Removed source.** After a structural edit that removes track B's strip (#1325's two phases),
   A's submissions for B's source are accepted until adoption and refused as `source.id.unknown`
   after it. The phase-1 ramp plays B's PCM with zero underruns.
4. **Browsers.** On the isolated leg in all three browsers:
   - the MSB1 feed and the `postMessage` feed render the qualification digests unchanged, with
     zero underruns;
   - `miso_engine_web_v1_render_allocation_count` reads 0.

   The non-isolated leg passes unchanged.
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if any producer call still runs on the render thread, or if moving the
  producer reorders chunks and seeks.
- Gate 2: turns red if the Worker anchors `seek_at` on its own clock instead of the plan's
  source-read clock.
- Gate 3: turns red if the browser bypasses the control plane's routing, so a removed source is
  refused at commit and its ramp plays underrun zeros.
- Gate 4: turns red if the Worker drain cannot keep the ring fed in a real browser, or if a
  source export stays in the worklet's locked window.

## Dependencies

Order: after #1381, because the source providers live in the Worker's `SessionState`. It is
independent of #1382 in behaviour; root orders the two by their shared files. It comes before
#1293 and #1294, whose `seek_at` export and message use this routing, and before *Feed and
retire the sources a browser edit adds or removes* (#1297), whose drain then runs in the Worker.

- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Anchor every seek on the plan's source-read clock* (#1316).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Report held source blocks apart from underruns* (#1318).
