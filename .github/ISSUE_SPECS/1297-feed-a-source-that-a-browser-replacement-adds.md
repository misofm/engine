# Feed a source that a browser replacement adds

Slice B8 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A browser app can add a new stem to a playing session with `replaceSession(...)`: the SDK attaches a
PCM ring for the new source after the engine has it, primes it, and starts it with an anchored seek
in exact time with the stems already playing. No gap on the playing stems and no misalignment of the
new one.

## Context

- The SDK's shared-memory feed wraps the engine processor class
  (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:243`) and applies each ring's seek
  itself, through `miso_engine_web_v1_source_seek`, at block boundaries (`applySharedSeek`,
  `:441-468`). A separate `miso.seek-at.v1` message would race those in-ring seeks.
- The feed validates a ring against the engine's source table on attach (`:176-184`) and pre-cuts its
  views there; after a memory growth its drains refuse until re-cut (`:283-290`, `:334-338`). The
  engine processor calls `afterReplace()` after a replacement (*Send a replacement session to the
  AudioWorklet*, #1294).
- The engine has no playhead: the frame an added stem should start at is the app's mapping (the frame
  the playing stems read at a render sample). The worklet reports the next absolute sample in its
  status.
- *Replace the session from the browser SDK* (#1296) refuses a document that adds a source.

## Decisions frozen for this slice

- **D1. Order.** Post the replacement first. Attach the new ring only after it resolves, when the
  engine's source table lists the new source.
- **D2. Anchored seek in the ring: MSB1 version 2.** The MSB1 control block is full
  (`MSB1_CONTROL_BYTES = 128`: 28 `i32` words and two `i64` at offset 112; `sdk/src/browser/pcm-ring.ts:1-12`),
  and both sides check the version word. Version 2 grows the control block to 144 bytes with an `i64`
  anchor at offset 128 (`-1` = no anchor) and shifts the ID and header offsets by 16. The writer
  (`pcm-ring.ts`) creates version 2 rings; the feed worklet accepts version 1 (no anchored seek) and
  version 2, and applies a ring's seek through `miso_engine_web_v1_source_seek_at` when the anchor is
  set, in the same `applySharedSeek` path, so no second seek channel exists.
- **D3. Anchor and frame.** The SDK chooses `A` = the worklet's next absolute sample plus a margin of
  at least the ring's prime length, rounded up to a quantum. The app supplies the frame through a
  callback `frameAt(anchorSample)` passed to `replaceSession` for each added source (the SDK does not
  invent a playhead). The SDK primes the ring from that frame at a new generation, then publishes the
  anchored seek.

## Deliverables

1. D1-D3 in `sdk/src/browser/pcm-feed.ts`, `sdk/src/browser/pcm-ring.ts`, the feed worklet and
   `sdk/src/browser/engine.ts`.
2. Tests and docs.

## Authorized paths

- `sdk/src/`, `sdk/test/`, `sdk/README.md`
- `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`

## Non-goals

- No engine or engine-worklet change. No engine playhead.

## Objective gates

1. **Aligned stem.** In `sdk/test/browser-pcm-evals.mjs` (the feed's test double over the real ring
   code): after a replacement that adds a source, the feed applies exactly one anchored seek with the
   computed `A` and the app's frame, through the in-ring path, and no plain seek for that source.
2. **Late anchor.** With the anchored seek observed after `A`, the engine-side rule (slice 5's late
   anchor) keeps the stem aligned: covered by B4's native gate; here, the feed still publishes the
   original `A` and frame (it does not recompute from the post time).
3. **Ring versions.** A version 1 ring still attaches and plays; a version 2 ring's anchor word is
   read at offset 128; a ring with any other version is refused on attach.
4. Commands: the SDK package qualification as in `qualification.yml`, and the umbrella's inherited
   gates.

## Test value

- Gate 1: a stem started with a plain seek, or through a second message channel racing the ring's
  seek, turns it red.
- Gate 2: an SDK that recomputes the frame from the post time misaligns the stem; it turns red.
- Gate 3: a reader that keeps version 1 offsets on a version 2 ring reads the ID region as the anchor;
  it turns red.

## Dependencies

- *Replace the session from the browser SDK* (#1296).
