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
