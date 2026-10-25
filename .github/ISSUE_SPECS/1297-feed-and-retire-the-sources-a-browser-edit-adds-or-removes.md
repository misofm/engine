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
  engine module* (#1293, D6) exports it. For a playing reference stem with no underrun since that
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
  - `alignTo: sourceId`: the SDK reads that playing source's seek report (#1293 D6) and uses
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

## Amendment (controls coordinator, batch 16a review, 2026-10-16)

Anchors checked against `90b76cfdf` (batch 16a with its first fold). Earlier text is kept.

**Finding (review M4).** D3 has `applySharedSeek` call `miso_engine_web_v1_source_seek_at`. That is
wrong for the feed worklet's render path.
- `applySharedSeek` is called from `drainSharedRing`, which runs in `process()`
  (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js:366`; the helper is `:466`, its
  comment at `:465` says "shared by render and the between-block control handler").
- The feed worklet is a render-thread caller of the plain seek: `tools/realtime-policy/render-thread-exports.json:10`
  lists `source_seek` with the callers `engine/receiveSeek` and `feed/applySharedSeek`. `source_seek_at`
  is not on that table and must not be added.
- `source_seek_at` (`hosts/host-web/src/ffi.rs:3313`) calls `SessionState::seek_at`
  (`crates/control-plane/src/control.rs:4073-4081`). That runs `service()` first (`:4080`; `service` is
  `:1930`). The service step drops retired plans, takes the shared report lock and may allocate. The
  doc comment on the export says the same: control half only, never `process()` (`ffi.rs:3301-3305`).
  The plain seek uses `seek_unserviced` (`hosts/host-web/src/lib.rs:3630`; `control.rs:4059`) for this
  reason.

**Rule, replacing D3's call.**
- A render-thread caller keeps the plain `miso_engine_web_v1_source_seek`. `applySharedSeek` called from
  `drainSharedRing` never calls `source_seek_at`.
- An anchored seek (anchor not `-1`) is applied by the control half's message handler, between blocks:
  the Worker in `worker` mode, the worklet's control handler in `single` mode. The writer that
  publishes an anchored seek on the ring also posts the same seek (generation, frame, anchor) to the
  control half as `miso.seek-at.v1` (#1294 D2 and its amendment of 2026-10-16). That message is what
  wakes the handler, and the handler calls `source_seek_at` once for it. It does not run in
  `process()`.
- The drain must not consume a ring whose newest seek epoch is anchored and not yet applied by the
  handler. It leaves the epoch unseen and reads no slot of that ring (the same wait the drain does
  today on `RESULT_BACKPRESSURE`, `:476-480`). Otherwise the drain would feed old-generation PCM to
  a source that is about to seek.
- "One seek channel" (D3) still holds: the ring publishes the seek once. The `miso.seek-at.v1` post
  carries the same seek to the handler that applies it; it is not a second seek. #1294 D2's sentence that shared-ring feeds do not use `miso.seek-at.v1` is
  amended in #1294 in the same batch.
- A render-side anchored seek would need `seek_at_unserviced` in `crates/control-plane`. No slice builds
  it (#1293 record, "Successors"). If this issue needs one, it names that successor first and waits
  for it. Do not call `source_seek_at` from `process()` meanwhile.

**Other text that carries the same assumption.**
- Context, `applySharedSeek` bullet (`:23-25`): true for the plain seek only. Read it with the rule above.
- D4 (`:80-81`, "then publishes the anchored seek once"): the publish is the writer's. Application is
  the handler's, so `A` must still be ahead of the render clock when the handler runs. The prime
  length in `A` (D4) covers the handler latency. It stays an engine-compensated lateness otherwise (gate 2).
- D4 `alignTo` reads the seek report. `source_seek_report` (`ffi.rs:3346`) runs
  `SessionState::seek_report`, which also runs the service step. So the report is read from the control
  half, not from `process()`. It is not on a render path in this spec; keep it so.
- Gate 1 (`:110-112`) says "the drain calls `source_seek_at` exactly once". Read it as "the control
  handler calls `source_seek_at` exactly once, and the drain never calls it". Add a check that the
  drain's recorded export calls contain no `source_seek_at`.
- Gate 3 (`:118-119`, "an unanchored seek takes the plain path"): the plain path is the only seek the
  drain may call. An anchored epoch seen by the drain is left unapplied, not sent to the plain seek.
- Test value, gate 1 (`:130-131`): add the defect "an SDK that calls `source_seek_at` from the drain".
  It runs the service step in `process()`.
- Non-goals (`:106`, "No engine, Worker or engine-worklet change"): the handler path lives in the
  SDK-side feed worklet and in #1294's control half. If it needs a change in
  `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, that file is outside this issue's authorized
  paths. Name the owner before implementation.
- Dependencies: #1294 (the message handler path, D2) is a dependency of D3 now. Add it.

## Record: slice #1297a (2026-10-25)

Scope note R1 (root): #1297a owns the engine worklet's two messages, so its authorized paths add the
engine worklet, its wrapper, the two `.d.ts` twins, `scripts/test-web-audioworklet.mjs` and the
realtime table (with `tools/realtime-policy/src/policy.rs`'s `UNWRAPPED_EXPORTS`). The ring layout
(D2), the drain rule and the pump copy are #1297b's; `alignTo` is #1297c's.

- `miso.seek-at.v1 { tag, requestId, sourceId, generation, sourceFrame, anchorSample }`: the
  handler calls `source_seek_at` once; the reply is `miso.ack.v1` with the engine's result (not
  sticky). A malformed message is a sticky `invalidArgument` with no export call.
- `miso.seek-report.v1 { tag, requestId, sourceId }`: the handler calls `source_seek_report` once
  and replies `{ tag, requestId, result, report }`, `report` the record's five words for an OK,
  else `null`. The record (`source_seek_report_ptr`) is bound at boot; a size word other than 64 is
  a sticky internal fault.
- Wrapper: `seekSourceAt` (shares the per-source seek bound with `seekSource`) and
  `seekReport(sourceId)`, both `@internal`. Table rows `wrapped: false`, engine callers only; the
  feed drain is not a caller (R2).

## Record: slice #1297b (2026-10-25)

Scope note R3/R4 and the amendment rule. Paths: `sdk/src/browser/{pcm-ring.ts,pcm-feed.ts,engine.ts}`,
the feed prelude, `hosts/host-web/web/stem-store/{pcm-pump.js,index.d.ts}`, the two eval files, the
README.

- D2: control block 144 bytes: three `i64` at 112 (generation, frame, anchor at 128; `-1` none),
  eight reserved zero bytes at 136, the ID at 144, the headers at 272; version word 1. The SDK
  writer, the prelude and the pump's writer copy refuse a header offset other than 272. The pump
  publishes plain seeks only and writes the anchor `-1` with each.
- Drain rule: an unseen epoch whose anchor is not `-1` calls no export and reads no slot until the
  engine handler answered its `miso.seek-at.v1` OK for exactly that `(generation, frame, anchor)`;
  then the drain consumes it as an applied seek. The prelude records the OK by overriding the
  engine's `receiveSeekAt`/`acknowledge` (no engine-worklet edit). Composition with #1644 D2: the
  stale-generation rule stays on the plain seek's path only; an anchored epoch never reaches an
  export, so no refusal can consume it, and a refused `seek-at` leaves it unseen until a newer ring
  seek supersedes it. Acked batch: an OK ack means the engine holds the seek (applied at its anchor
  or reported late by the seek report); the drain then only feeds its PCM, and no ack precedes a
  drop.
- D4/D5 as `start: { frameAt, prime }` (decision of this slice: the SDK cannot write PCM, so the
  app's `prime(seek)` writes the ring from the frame at the generation; it is called after the ring
  seek is published and before the `seek-at` is sent, so the drain cannot count the prime as stale).
  `A` = `nextAbsoluteSample` + capacity x quantum, rounded up to a quantum, read once. The generation
  comes from the engine's session-seek sequence (`highestSeekGeneration + 1`, reserved), so a later
  `engine.seek` is above it. A start needs `FeedOptions.engine` (what `createEngine` resolved with);
  `linkFeedEngine` is the SDK's `@internal` seam. A refused start rejects `ready()` with a
  `MisoEngineError` and closes the feed. `alignTo` stays #1297c's.
- D5 `EngineFeed.detachSource(sourceId)`: op `detach-source`, reply `source-detached` from the
  attach port's handler (between blocks, on the drain's thread); then the writer is released. Timeout
  closes the feed. `EngineFeed.rings` is the delivered set.
- D6: `engine.apply` retires the fed sources its `removeSource` edits name; `replaceSession` the
  fed sources the committed session map no longer lists (R4). Each is detached after
  `engine.applied(revision)` resolves.
- Tests (red first on mutants, all 13 red): old header offset refused at attach; anchored start
  (one status read, `A` rounded, `frameAt` once, handler `seek_at` once, no slot read and no plain
  seek before it, a refused `seek-at` never consumed); `detachSource`; the pump's plain seek in the
  new layout; D6 over the real core (apply and replace).

Attempt 2 (verifier findings of attempt 1):
- `detach-source` names the ring by its index in the attach list: a port clones a
  SharedArrayBuffer into a new object, so object identity never matched in a browser. The worklet
  clears that entry and replies `detached: true|false`; the SDK rejects `detachRefused` and closes
  the feed on false. Both fake ports now `structuredClone`; the detach test was red on attempt 1's
  code under the cloning port.
- The browser barrel exports `pcm-feed.ts` by name, without `linkFeedEngine` and `FeedRetirement`
  (`@ts-expect-error` pins in `sdk/test/barrel-surface.ts`; `sdk/src/browser/index.ts` and that
  test file are outside the scope note's list, edited for this finding).
- One test composes the prelude with the shipped engine worklet class (stub exports): one
  `miso.seek-at.v1` through `receive`, then the drain consumes the epoch (red when the engine's
  handler is renamed). `Msb1RingWriter.seek`'s anchor is documented as the SDK start's.

Attempt 3 (test gap of attempt 2): the detach test runs three sources, detaches the first and then
the last, checks that the middle ring keeps draining, and runs `prepareSeek` over the ring that
remains. It is red when the worklet compacts its attach list or passes withdrawn slots to
prepare-seek. `attachEngineFeed` refuses duplicate source IDs, and a start whose source was detached
during `ready()` is skipped. Each was red first. The paused-seek test's port clones its messages.

## Record: slice #1297c (2026-10-25)

Paths: `sdk/src/browser/pcm-feed.ts`, `sdk/test/browser-pcm-evals.mjs`, and the README's feed
section (the spec's deliverable 2; outside the scope note's 1297c list).

- `FeedStart` is `{ frameAt, prime } | { alignTo, prime }`; exactly one of `frameAt` and `alignTo`,
  else `attachEngineFeed` throws. `alignTo` may name `$timeline` (1297a's reserved report ID).
- Decision of this slice ("changed since that report"): one report has no baseline for its own
  counters, so the SDK reads the reference's report before the one status read and again after `A`
  is computed. The frame is `sourceFrame + (A - firstSample)` from the first. Refusals, each a
  `MisoUsageError` with `diagnosticCode`, before any ring publish: `feed.align.unrendered`
  (generation 0), `feed.align.busy` (`RESULT_BACKPRESSURE` on 4 reads, a local or an engine 6,
  one macrotask between reads), `feed.align.reseeked` (generation, first sample or frame differ),
  `feed.align.held` (the held counter moved between the reads, or a pending seek below),
  `feed.align.domain`. Another engine result is a `MisoEngineError`. Each rejects `ready()` and
  closes the feed (1297b's rule).
- A start whose feed closed, or whose source was detached, during these reads publishes nothing.

Attempt 2 (verifier findings of attempt 1, root rulings):
- A pending start of the reference: a source's first generation is 1, not 0, so a reference whose
  own anchored start is not rendered yet reports generation 1 at frame 0 and nothing refused. The
  engine's feed link records each published anchored start `(generation, F_X, A_X)` by source ID
  (removed with its ring on detach or close). When that record's generation is above the
  reference's report, the frame is `F_X + (A_Y - A_X)`, exact, with no second report read. Any
  other seek published in a fed reference's ring above the report's generation (read from the
  ring's published seek words in `pcm-feed.ts`'s `seekSnapshot`; `Msb1RingObserver` has no getter
  for it and `pcm-ring.ts` is not in this slice's paths) refuses `feed.align.held`.
- Underrun is not a refusal: the consumer advances its frame on an underrun (`crates/source`,
  the missing-PCM branch adds the quantum to `next_frame`), so the formula stays exact; the
  attempt-1 test-value claim "off by that many samples" (gate 1a) is false for underruns. Held
  frames stay a refusal (an anchored seek of the reference is waiting).
- A stem whose region starts after its frame at `A` is refused by the engine's `seek_at` (outside
  the region), not clamped: `ready()` rejects with that `MisoEngineError`.

Attempt 3 (verifier finding of attempt 2, root ruling: an aligned source listed before its
reference in one feed played 50,000 frames off):
- One feed starts its sources in a stable dependency order: a source whose `alignTo` names another
  started source of the feed starts after it. A self-alignment or an `alignTo` cycle throws a
  `MisoUsageError` at attach.
- Across feeds: each linked feed reports the sources whose start it has not published yet (from
  attach until the record exists, or the source is detached or the feed closed). Aligning to one
  refuses `feed.align.unstarted`; there is no waiting across feeds.
- On the record path, the record is read again after the status read; another record refuses
  `feed.align.reseeked`.
- A reference whose own `seek-at` the engine later refuses leaves the aligned stem at the frame the
  reference's `frameAt` mapping gave (README).
