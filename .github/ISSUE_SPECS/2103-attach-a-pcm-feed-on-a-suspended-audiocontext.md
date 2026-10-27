# Attach a PCM feed on a suspended AudioContext

Filed by root from the console sprint (2026-10-26). Source: the scope note `scratchpad/opt/scope/sdk-console-gaps.md`, item 2. Comes after the boot-ergonomics issue (it uses `pcmFeed: true`).

## Problem
The console saw `feed.ready()` time out when the feed attached while the AudioContext was suspended. It had to resume, attach, suspend, then seek to 0 to discard what rendered meanwhile. Boot control replies flow while suspended, so the attach node (`miso-sab-feed-attach`) should too. The cause is unproven.

## Smallest slice
- First a reproducer: one qualification row in Chromium and WebKit. It attaches a feed on a suspended context, and `ready()` must resolve within its default 2 s.
- If the row is green at HEAD, the issue closes as console misuse, the row stays and the README gives the right call.
- If it is red, route the attach through the engine node's port (the prelude already hooks it for `miso.seek-at.v1`; `linkFeedEngine`, pcm-feed.ts ~188).

## Gates
- The qualification row in Chromium and WebKit. It catches an attach that depends on render.

## Paths
hosts/host-web/qualification/ (one row), sdk/src/browser/pcm-feed.ts, sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js, this spec.

## Decision and implementation (Sol 6.1, 2026-10-27)

The timeout is not reproduced using the documented supported sequence: boot with `pcmFeed`
enabled before the engine worklet, attach with the engine and its context, then await
`feed.ready()` with its default 2 s deadline while that context remains suspended. The original
console call and its cause are not established. Follow the brief's baseline-green branch: keep
one browser qualification row and document this sequence; do not reroute or otherwise change
the production feed implementation.

The row runs on an opt-in cross-origin-isolated qualification server because MSB1 requires
SharedArrayBuffer; the existing deployment matrix page remains non-isolated. Normal
`--sdk-root` qualification invokes it, and `--pcm-feed-suspended-only` runs just this row in
Chromium and WebKit sequentially. The fixture uses the existing observation document, the real
shipped host/worklet and source SDK/prelude (an explicit prelude module URL serves the same boot
option as README's bundled `pcmFeed: true`). Scratch metadata only is injected, as in the
existing browser SDK rows. No resume or seek is performed. Assertions require an active feed,
acknowledged ring attachment, an unchanged suspended context clock and render sample zero,
zero drain blocks, and a successful zero render-allocation count.

Test value: an attach that requires the AudioWorklet render callback, or accidentally advances
the audio clock to initialize the feed, turns this row red; existing qualification has no
shared-memory feed row and package browser smoke covers Chromium only.

## Validation evidence

Evidence directory: `/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/2103/`.
All validation runs through the shared serial wrapper; browser legs are sequential. The
reused reviewed #1668 browser artifact directory is
`/home/bl/misofm/engine-work-evidence/1668/astra/browser-artifacts`. Its host and worklet JS are
byte-identical to base `7be08dd18`; its Wasm predates later DSP banking changes. This scoped
reproduction qualifies feed/control attachment, not a new current-Wasm deployment matrix.

Initial harness bring-up logs (`baseline-chromium.log`, `baseline-webkit.log`) already show
successful attachment with no clock/render/drain advancement (0.285 ms Chromium, 5.84 ms
WebKit). Their only failing predicate compared the allocation response object to numeric zero;
the row now checks its `result` and `count`. These failures are harness errors, not product
regression evidence.

Clean focused gate (`baseline-both.log`, exit 0):

```sh
node hosts/host-web/qualification/run.mjs \
  --artifacts /home/bl/misofm/engine-work-evidence/1668/astra/browser-artifacts \
  --browser all --pcm-feed-suspended-only
```

- Chromium 151.0.7922.34: ready in 4.285 ms.
- WebKit 26.5: ready in 6.18 ms.
- Both: suspended before/after; currentTime and nextAbsoluteSample remain zero; feed active,
  ATTACHED=1, DRAIN_BLOCKS=0, render-allocation reply result=0 and count=0.

Ad-hoc strict DOM/WebWorker typechecking of the new entry reaches one existing error:
`default-host.ts:38` TS2740, structural `AudioContextLike` passed as `BaseAudioContext`.
Running the identical compiler flags on unchanged `sdk/src/browser/engine.ts` produces the
byte-identical diagnostic (`typecheck.log`, `typecheck-baseline.log`, exit 2 both).
No diagnostic names the new harness. This is not the canonical SDK WebWorker-only typecheck;
no SDK implementation or ambient types were changed to accommodate the harness.
`git diff --check` passes.

Fresh independent verdict and integrated artifact verification are owned by the batch
coordinator; no PASS, upstream synchronization or closure is claimed here.
