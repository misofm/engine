# SDK: make a live engine.seek gap-free with an anchored session seek (transport path of #2185)

GitHub #2205. Successor of #2185 (found by its mapping, 2026-10-29).

## Problem

#2185 closes the gap of a live per-source ring seek by deferring it to its first new-generation
chunk. The SDK's documented transport seek, `engine.seek(frame, { producers })`
(`sdk/src/browser/engine.ts`, README "seek"), is a plain `session_seek` first: render pops it at
its next block and drops every source's old chunks before new PCM exists. A transport seek during
playback therefore still underruns for the producers' publish latency (the console's seek uses this
path). A drain deferral cannot reach it.

The host already accepts `anchorSample` on `seekSession` (`sdk/src/browser/shipped-host.d.ts`), and
the engine keeps the old generation playing until the anchor (`crates/source/src/lib.rs`).

## Scope

Make a live `engine.seek` gap-free: decide the anchor (next block plus a bound, or an anchor that
lands as soon as every producer has published, bounded), measure the latency it adds against the
gap it removes, and keep a suspended seek's behaviour unchanged.

## Gate (one test per claim)

A live transport seek during playback underruns 0 frames on Chromium in both modes in the
qualification smoke's live phase (added by #2185, `runLivePcmFeedSeek`); the added seek latency is
reported.
