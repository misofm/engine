# Move browser source submission and seeks into the Worker

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-12).
Code anchors verified on `main` at `6fb211594`.

Root ruling for this slice: the C ABI shape governs. Source producers, and therefore submission
and seeks, belong to the control half (the Worker), never to the render worklet. This slice comes
directly after *Run the browser control plane in a Worker and keep the AudioWorklet render-only*
(#1332) and before *Swap and retire browser plans through the Worker's service loop* (#1381), so
the producers have exactly one owner at every commit:
- through #1332: the worklet, as today;
- from this slice: the Worker, which holds the `SourceControlSet` and calls it directly;
- from #1381: the same set, now inside the Worker's `control_plane::SessionState`, which routes
  every source call (#1381 D1-D2).

## Product outcome

On a cross-origin-isolated page, the Worker submits decoded PCM and applies plain and anchored
seeks. The worklet keeps only the ring consumers and calls no source entry point.

The feeds keep working unchanged: the shared MSB1 rings and the `postMessage` feed play the same
bits with no new underruns. Every seek lands where #1316's contract says it does.

On a page that is not isolated (`single` mode), everything stays in the one worklet instance, as
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
- **The producer half is one field.** host-core's prepared runtime holds `sources:
  SourceControlSet` (`crates/host-core/src/prepare.rs:529`). `SourceControlSet::submit`, `seek`
  and `seek_at` (`crates/host-core/src/source.rs:152`, `:200`, `:223`) are producer-side calls.
  Each source has one producer thread.
- **After #1332** the Worker boots the host and moves all of it into the worklet with
  `host_release` and `host_adopt` (#1332 D2-D3). #1332 D3 item 6 leaves submission and seeks on
  the worklet thread, as ordering only.
- **Seek contract.** *Anchor every seek on the plan's source-read clock* (#1316):
  - a `seek_at` anchor is a sample of the plan's source-read clock;
  - a plain seek applies at the first block whose render begins after it is popped;
  - each ring publishes a seek report (#1316 D2-D3, facade `seek_report`, D7).
- **Ring headroom.** The default ring hides about 100 ms (`default_source_ring_frames`,
  `crates/host-core/src/prepare.rs:65`).

## Decisions frozen for this slice

- **D1. The Worker keeps the producer half.** In `worker` mode, `host_release` moves the host's
  `SourceControlSet` out before it builds the transfer token. The Worker keeps the set in its own
  handle table, under the host's handle. The token carries the rest of the host, and the worklet's
  host has no producer. Dispose rejoins them in the Worker before `miso_engine_web_v1_dispose`
  (#1332 D4). In `single` mode the host keeps its set.
- **D2. Entry points.** In `worker` mode the Worker calls every source operation on its set:
  - `miso_engine_web_v1_source_submit`;
  - `miso_engine_web_v1_source_seek`;
  - `seek_at` and the #1316 seek report, natively.

  The worklet's post-boot set (#1332 D3 item 5) drops the source exports. The
  `miso_engine_web_v1_source_seek_at` export (#1293) and the `miso.seek-at.v1` message (#1294)
  come later and attach to the Worker's routing. In `single` mode, the worklet's own control
  handler calls the exports, outside the render-locked window.
- **D3. The clock.** `seek_at` and the seek report read the plan's source-read clock (#1316). The
  render half stores it in an atomic after every block, as capi's `SharedPlanState::render_sample`
  does (`crates/control-plane/src/plan.rs:13`), and the Worker's set reads it there.
- **D4. Messages.** In `worker` mode, `miso.source.v1` and `miso.seek.v1` travel on #1332's
  page-to-control-plane port to the Worker. Their fields and replies are unchanged.
- **D5. The MSB1 drain moves with the producer.**
  - The SDK's feed module is loaded into the control Worker as a module, not as a worklet
    prelude.
  - The Worker starts a repeating timer of one quantum's duration. The drain runs on each tick and
    after each control message. *Swap and retire browser plans through the Worker's service loop*
    (#1381 D6) adds its service step to the same timer.
  - The drain keeps today's rules: apply a published seek, drop stale generations, stop at
    `RESULT_BACKPRESSURE`, and use the same staging copy and the same export.
  - In `single` mode the worklet prelude stays as it is.
- **D6. Underruns.** A Worker that falls behind empties the ring. That emits zeros plus the
  existing underrun counter (`AGENTS.md`). Held blocks stay apart from underruns (#1318). Nothing
  blocks render.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Submission and seek replies
  keep today's meaning: a submit is acknowledged only after its chunk is in the ring, and a full
  ring returns `RESULT_BACKPRESSURE` with nothing written. Moving the caller to another thread
  changes no queue.

## Deliverables

1. D1-D4 in `hosts/host-web/src/` and the control Worker and worklet scripts.
2. D5 in the SDK feed: the Worker-loaded drain module, and the routing in
   `sdk/src/browser/pcm-feed.ts`.
3. A test-support API, public only under `host-web/test-support`: `boot_split_for_test(document)
   -> (ControlHalf, RenderHalf)`, where `ControlHalf` holds the producer set and `RenderHalf` the
   rest. #1381 extends `ControlHalf` with the session state.
4. The integration test binary `hosts/host-web/tests/worker_source_producer.rs`.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/tests/worker_source_producer.rs` (new),
  `hosts/host-web/Cargo.toml` (the `bench-support` dev-dependency if no earlier slice added it),
  `hosts/host-web/web/`, `hosts/host-web/qualification/`
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` and a new Worker drain module
  beside it, `sdk/src/browser/pcm-feed.ts`, `sdk/src/assets.ts`
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- `scripts/test-web-audioworklet.mjs` (the fake exports and messages for source submission and
  seeks moving to the Worker)

## Non-goals

- No change to the MSB1 layout (#1297 amends it) or the seek contract (#1316).
- No session-state routing: #1381 D2 puts the set inside `SessionState`, after which a removed
  source follows #1325 D4. That routing is tested on the shared code by #1325 and on the browser
  by *Feed and retire the sources a browser edit adds or removes* (#1297).
- No direct page-to-engine-ring writes. The copy from MSB1 into the engine ring stays as it is
  today, now off the audio thread.

## Hazards

- **A plain seek's landing block now depends on Worker timing.** #1316's report tells the host
  where it landed. Hosts that need an exact block use `seek_at`.
- **Each source keeps one producer thread.** Feeding one source from both realms is refused, never
  interleaved.

## Objective gates

1. **Producer off the render thread.** Integration test binary
   `hosts/host-web/tests/worker_source_producer.rs`. Decision 15 rules that host-web's native
   allocation-count gates live in an integration binary, never in `src/tests.rs`. It links
   `bench_support::alloc` and calls `assert_installed()` first.
   - The control half runs on thread A and the render half on thread B, nine tracks. A submits
     every chunk and two plain seeks while B renders 64 blocks.
   - B's thread-scoped counters read `allocations == 0 && frees == 0` around every render call.
   - The output is bit-identical to the same feed and seeks run single-threaded, with each seek
     applied at the block its #1316 seek report names.
2. **Anchored seek.** A's `seek_at` at source-read sample `S`. The seek report names `S`. The
   output from `S` is bit-identical to a fresh boot fed from the target frame.
3. **Disposal rejoins the halves.** After gate 1, dispose through the Worker path. The Worker's
   handle table holds no set afterwards, and a `source_submit` on the old handle is refused as an
   unknown handle.
4. **Browsers.** On the isolated leg in all three browsers:
   - the MSB1 feed and the `postMessage` feed render the qualification digests unchanged, with
     zero underruns;
   - `miso_engine_web_v1_render_allocation_count` reads 0.

   The non-isolated leg passes unchanged.
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gate 1's binary)
   - the artifact build and `bash scripts/check-web-audioworklet.sh` in both forms that #1332
     gate 7 spells; `bash scripts/test-web-audioworklet.sh`
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
  source-read clock (D3).
- Gate 3: turns red if dispose leaves the Worker's producer set behind, or frees it twice.
- Gate 4: turns red if the Worker drain cannot keep the ring fed in a real browser, or if a
  source export stays in the worklet's locked window.

## Dependencies

- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Anchor every seek on the plan's source-read clock* (#1316).
- *Report held source blocks apart from underruns* (#1318).

Dependents: #1381 (it moves this slice's set into `SessionState`), #1293 and #1294 (their
`seek_at` export and message use this routing), and #1297 (its drain runs in the Worker).
