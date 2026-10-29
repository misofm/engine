# MSB1 feed drain: read the generation tag before the seek epoch

GitHub #2184. Found by the bounded verify of #1387 (2026-10-28), in both MSB1 consumers: the SDK
prelude `sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js` and the control Worker's copy.

## Root cause

The drain reads `SEEK_EPOCH` before `GENERATION_TAG`, while the writer stores the tag before it
bumps the epoch (`sdk/src/browser/pcm-ring.ts`). A drain pass can therefore observe the old epoch
with the new tag and submit new-generation chunks before it issues `source_seek`. The engine
probably refuses them, but the slot is still consumed (`REFUSED`). The window is tiny, and it
predates #1387.

## Smallest slice

Read the epoch, then the tag, then the epoch again, and retry on a change; or reorder the writer so
that the epoch bump publishes the tag. Both consumers take the same fix.

## Gate (one test per claim)

One case in the shared MSB1 consumer case table (#1387, `sdk/test/msb1-consumer-cases.mjs`) that
interleaves the writer's two stores with a drain pass: red on the current read order.

## Decision record (attempt 1, 2026-10-29)

- **Root ruling 1: a parity seqlock on `SEEK_EPOCH`, no `SEEK_SEQ` word.** The writer
  (`Msb1RingWriter.seek` in `sdk/src/browser/pcm-ring.ts`, `RingWriter.seek` in
  `hosts/host-web/web/stem-store/pcm-pump.js`) adds 1 to the epoch (odd: a publication in flight),
  stores the generation, frame and anchor with `Atomics.store`, stores the tag, and adds 1 again
  (even). One seek steps the epoch by 2; 0 still means no seek was ever published. The MSB1 layout
  does not change (V1 amended in place, as #1297b D2 did); the semantics are documented in
  `pcm-ring.ts`, both consumers, `sdk/README.md` and `docs/STEM_STORE_V1.md`.
- **Readers.** Both drains (`readSharedSeek` in the prelude and in the control Worker) load the
  epoch, skip the pass when it is odd, load the tag and the tuple, and use them only when the epoch
  loads the same again; the drain then applies the seek and filters slots by that tag, and the
  apply reads the tuple from that snapshot (no allocation: the snapshot is fields of the ring).
  `prepare-seek` matches the live publication through the same snapshot. The SDK's `seekSnapshot`
  (`pcm-feed.ts`) marks a snapshot read across a publication odd: `prepareSeek` refuses it as
  `prepareSuperseded`, and an aligned start refuses it as `feed.align.held`. The Observer skips a
  pull at an odd epoch; its existing post-copy re-check of the epoch closes the window.
- **Why not a reorder.** JS atomics are sequentially consistent; with begin and end markers, a
  pass that reads the same even epoch before and after its reads read exactly the last complete
  publication. A reader reorder alone leaves the leg "old tag, new epoch"; a writer reorder alone
  leaves the mirror leg (plan, root-approved).
- **Observer test.** The observer tests' generator never creates the odd state, but no observable
  defect follows from an observer that ignores the parity: inside the odd window the writer has
  published no chunk of the new generation, and the observer's post-copy epoch re-check already
  refuses a pass the publication straddles. No test can turn red, so none is added (one test per
  claim); the skip is kept for protocol conformance.

## Evidence (attempt 1)

- New case-table row (`sdk/test/msb1-consumer-cases.mjs`), "#2184: a seek publication in flight
  reads no slot and asks no seek; complete, the seek precedes its chunk". Red on 723595ed5's
  readers on both adapters: the prelude (`browser-pcm-evals.mjs`) asked `source_seek(2, 40)` and
  submitted the g2 chunk at the odd epoch; the control Worker (`test-web-audioworklet.mjs`) failed
  the same assertion ("a publication in flight asks no seek and reads no slot"). Green with the
  fix on both.
- The case table's literal prepare-seek epochs now read the ring's epoch back; the writer's layout
  test expects 2 and 4 after two seeks.

## Attempt 2 (2026-10-29, verify of 66dd43bad: FAIL)

- **Write index inside the window (verify blocker 1, root ruling (b)).** `readSharedSeek` now also
  loads `WRITE_INDEX` between its two epoch loads, and the drain and the deferral read slots only
  below that index. A seek publication completes before its first chunk is committed, so every
  slot below an index read inside the window belongs to the snapshot's seek or an earlier one. The
  #2185 spec has the row and its red evidence (the slot classification by generation is #2185's).
- **No BigInt on an idle block (verify major 2).** A pass whose first epoch load equals the cached
  even epoch skips the three `i64` tuple loads (the tag, the write index and the second epoch
  load are `i32`). The slot generation compare reads the `i64` as two `i32` words. The repo's
  prelude allocation probe (`runPrelude` `tracking`) counts typed-array constructions only and
  cannot see a heap BigInt, and a GC-count gate would be a new framework, so the fix is measured,
  not gated: the verify's probe (16 idle rings, 2 000 000 `process()` calls, `node:perf_hooks` gc
  entries) counts 527 GCs on 66dd43bad and 28 with the fix, the 723595ed5 baseline the verify
  reported (two runs each, identical).
- **Writer order (verify minor 5).** The pump-writer layout test now wraps `Atomics.store` (the
  writers' own store path) and asserts that each writer stores the generation, frame, anchor and
  tag while the epoch is odd. Red on the 723595ed5 writers of both `pcm-ring.ts` and `pcm-pump.js`.
- **Found while fixing:** the control Worker's snapshot field `seekGeneration` (attempt 1) had the
  name of #1387's seek-transition field in the same ring record, so the object literal kept one of
  them and the two uses overwrote each other. The snapshot's generation is now
  `publishedGeneration` in both consumers.
