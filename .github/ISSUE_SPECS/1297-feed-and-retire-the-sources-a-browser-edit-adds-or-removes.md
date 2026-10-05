# Feed and retire the sources a browser edit adds or removes

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-12, D15-17).
Formerly slice B8 of *Swap a rebuilt plan without an audio gap* (#1269).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A browser app that feeds stems through the SDK PCM feed can add a stem with `engine.apply` (or
`replaceSession`) while audio plays: the SDK attaches a ring for the new source once the edit
commits, primes it, and starts it with an anchored seek, in exact time with the stems already
playing. A source the edit removes keeps being fed until its strip's removal is in effect, then its
ring is detached. No gap on the playing stems, no misaligned new stem, no starved removed strip
during its ramp-out.

## Context

- The SDK feed wraps the engine processor class (`MisoSabFeedProcessor`,
  `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:243`). Binding is additive: a second
  attach node adds its rings and leaves earlier ones alone (`attachSharedRings`, `:265`); a
  detach withdraws exactly one node's rings (`detachSharedRings`, `:297`; ops `attach` / `detach` /
  `prepare-seek` at `:533-541`). `bindRing` validates magic and version (`:154-186`).
- The drain applies each ring's published seek at a block boundary through the plain
  `miso_engine_web_v1_source_seek` (`applySharedSeek`, `:441-468`). A second seek channel per source
  (a separate message) would race these in-ring seeks.
- `attachEngineFeed` (`sdk/src/browser/pcm-feed.ts:285`) creates one ring per listed source
  (`FeedOptions.sources`, `:66`). `EngineFeed` (`:72-85`) has no per-source detach.
- The MSB1 control block is full: `MSB1_CONTROL_BYTES = 128`, 28 `i32` words and two `i64` at
  offset 112, the ID at 128 and the slot headers at 256 (`sdk/src/browser/pcm-ring.ts:1-12`; the
  reader's copies at `miso-engine-v1-pcm-feed-worklet.js:110-116`). Both sides check the version
  word, which is 1.
- `AGENTS.md`'s version rule: a wire identity's prelaunch version is V1, and no live product surface
  may claim a later generation before launch. The ring writer and reader ship in the same SDK
  package.
- The engine has no playhead: the frame an added stem should start at is the app's mapping. But
  #1316 makes render publish, per source, where its last seek landed (`generation`,
  `first_sample`, `source_frame`), and *Export transaction apply and anchored seek from the browser
  engine module* (#1293, D5b) exports it. For a playing reference stem with no underrun since that
  report, the frame it reads at sample `A` is `source_frame + (A - first_sample)` (source rate equals
  render rate in this version, D15-12).
- Seek contract (decision 15 D15-12, *Anchor every seek on the plan's source-read clock*, #1316):
  the anchor is on the plan's source-read clock, equal to the render clock in this version; a
  late-reached anchor starts the stem at F plus the lateness; a held `seek_at` moves with its
  producer when a candidate is superseded (tested by *Test held seeks across swaps and
  supersession, and add a seek to audit capi*, #1319). From commit on, submissions and seeks
  address the newest committed session (`crates/capi/include/miso_engine_v1.h:85-89`, the shared
  rule the browser control plane adopts).
- A removed strip's source retires with its phase-2 swap, not with the commit (D15-9, *Remove a
  strip in two phases: ramp out, then a scheduled swap*, #1325).
- The source producers belong to the control half, the Worker in `worker` mode: *Replace the
  running browser session in the Rust host* (#1290) D4 moves them into the candidate on the
  control thread at commit, and *Swap and retire browser plans through the Worker's service loop*
  (#1381) D1 prepares with them in the Worker. *Run the browser control plane in a Worker and keep
  the AudioWorklet render-only* (#1332) D3 item 6 still keeps submission and seeks on the worklet;
  the move into the Worker is *Move browser source submission and seeks into the Worker* (#1387). The producer is single: submissions and seeks of one
  source run on one thread, and the MSB1 drain runs there. This slice changes no thread
  assignment.

## Decisions frozen for this slice

- **D1. Order.** The SDK attaches an added source's ring after `apply` resolves (the commit), not
  after the watermark: the source's producer exists from commit.
- **D2. Anchor in the ring, MSB1 V1 amended in place.** The control block grows to 144 bytes with
  an `i64` anchor at offset 128 (`-1` = no anchor); the ID moves to 144 and the slot headers to 272.
  The version word stays 1 (AGENTS.md version rule). The reader refuses a ring whose
  `HEADER_OFFSET` word is not 272, so a ring written to the old layout is refused on attach. Writer
  (`pcm-ring.ts`) and reader change in the same PR.
- **D3. One seek channel.** The writer publishes generation, frame and anchor, then bumps the seek
  epoch. `applySharedSeek` calls `miso_engine_web_v1_source_seek_at` when the anchor is not `-1`,
  else the plain seek. No message-based seek is used for a fed source.
- **D4. Anchor and frame.** For each added source the SDK chooses `A` = the status's next absolute
  sample plus at least the ring's prime length, rounded up to a quantum multiple. The frame comes
  from one of two options, exactly one of which is given:
  - `alignTo: sourceId`: the SDK reads that playing source's seek report (#1293 D5b) and uses
    `source_frame + (A - first_sample)`. A report with generation 0 (not yet rendered), a busy read
    after its bounded retries, or underrun or held frames that changed since that report's block
    reject with a typed `MisoUsageError` naming the cause; the app then uses `frameAt`.
  - `frameAt(anchorSample)`: the app's own mapping.

  The SDK primes the ring from that frame at a new generation, then publishes the anchored seek
  once.
- **D5. API.** `FeedSource` gains an optional `start: { alignTo: string } | { frameAt(anchorSample:
  bigint): bigint }`;
  `attachEngineFeed` with one added source runs D4 for it. `EngineFeed` gains
  `detachSource(sourceId)`, which withdraws one ring (a new `detach-source` op) and resolves after
  the drain stopped reading it.
- **D6. Removed sources.** After a commit whose transaction removes a fed source, the SDK keeps
  feeding that ring until `applied(revision)` resolves (#1325 completes a removal's revision at its
  phase-2 swap), then calls `detachSource` and releases the ring.
- **Superseded.** The old D2 MSB1 version 2 (contradicts the version rule) and the old D3 wait for
  the replacement to resolve before attaching (a commit is now the resolution) are replaced above.

## Deliverables

1. D1-D6 in `sdk/src/browser/pcm-ring.ts`, `sdk/src/browser/pcm-feed.ts`,
   `sdk/src/browser/engine.ts` and the feed worklet.
2. Tests in `sdk/test/browser-pcm-evals.mjs`; the README's feed section.

## Authorized paths

- `sdk/src/browser/`, `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`
- `sdk/test/`, `sdk/README.md`

## Non-goals

- No engine, Worker or engine-worklet change. No engine playhead. No thread move of the drain.

## Objective gates

1. **Aligned stem.** In `browser-pcm-evals.mjs` (the feed's test double over the real ring code):
   after an edit that adds a source, the drain calls `source_seek_at` exactly once for it, with the
   computed `A` and the app's frame, and never the plain seek for that source.
1a. **Align to a playing stem.** With a fake seek report `(g, first_sample = 1024, source_frame =
   48000)` for stem X, `alignTo: "X"` and `A = 4096` publish frame `51072`. A report with generation
   0, or with underrun frames changed since it, rejects with the named cause and publishes nothing.
2. **No recompute on a late drain.** With the drain observing the seek after `A`, the published
   `A` and frame are the original ones (the engine compensates lateness, D15-12).
3. **Layout.** The anchor word is read at offset 128 and the ID at 144; a ring with
   `HEADER_OFFSET` 256 is refused on attach; an unanchored seek (`-1`) takes the plain path.
4. **Removed source.** The removed source's ring is drained and accepted until `applied(revision)`
   resolves, then detached; the drain never reads it after `detachSource` resolves, and the other
   rings keep draining.
5. Commands, with an artifact directory built as in `qualification.yml`'s `artifact` job and
   `npm ci` in `sdk/`: `bash scripts/check-sdk-generated.sh <artifacts>`,
   `python3 -B scripts/check-sdk-deletions.py`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-sdk-headless.sh <artifacts>`, `bash scripts/sdk-package.sh check <artifacts>`.

## Test value

- Gate 1: a stem started with a plain seek, or through a second channel racing the ring's seek,
  turns it red.
- Gate 1a: an SDK that adds `A` to the frame without subtracting `first_sample`, or aligns to a
  stem that has underrun since its report, starts the new stem off by that many samples (comb
  filtering on correlated drum mics, round-1 finding D1-b); it turns red.
- Gate 2: an SDK that recomputes the frame from the drain time misaligns the stem; it turns red.
- Gate 3: a reader that keeps the old offsets reads the ID bytes as the anchor; it turns red.
- Gate 4: an SDK that detaches at commit starves the strip during its ramp-out (the ramp plays
  silence, then the swap); one that never detaches leaks a ring per removed stem. Both turn it red.

## Dependencies

- *Apply session transactions from the browser SDK* (#1296).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).
- *Anchor every seek on the plan's source-read clock* (#1316).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Move browser source submission and seeks into the Worker* (#1387).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
