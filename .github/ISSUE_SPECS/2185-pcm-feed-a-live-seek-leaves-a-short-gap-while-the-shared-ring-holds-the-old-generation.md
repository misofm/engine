# PCM feed: a live seek leaves a short gap while the shared ring holds the old generation

GitHub #2185. Found while building #1387 (2026-10-28).

## Observed

A live plain ring seek during playback can underrun a few quanta in both instance modes: worker
mode 0 to 512 frames per seek in the Chromium/WebKit smoke after #1387's headroom fix; single mode
about 896 frames on Chromium in the same smoke. A suspended seek (`prepareSeek`, refill, resume) is
already gap-free in both modes.

## Root cause

The writer seeks while the shared MSB1 ring is full of the old generation. It cannot publish
new-generation PCM until the drain frees those stale slots, and the drain applies the seek in the
same pass, so render pops the seek before any new chunk exists.

## Candidate fix (not decided)

Defer applying a live plain seek until its first new-generation chunk is published: the drain
frees the stale shared slots at once, the old generation keeps playing from the engine ring
meanwhile, and a bound covers a writer that never publishes. The fix applies to both consumers (the
SDK prelude and the control Worker), through the shared MSB1 case table.

## Gate (one test per claim)

One case in the shared MSB1 consumer case table: a live seek while the shared ring is full of the
old generation applies no seek before a new-generation chunk is published (or the bound expires).

## Decision record (attempt 1, 2026-10-29)

- **The fix: defer a live plain seek to its first new-generation chunk, bounded** (both consumers:
  `deferSharedSeek` in the prelude and in the control Worker). While the ring's published plain
  seek is unapplied and no slot of its tag is published, a drain pass frees every slot of another
  tag (`STALE`), asks no seek and submits nothing; the old generation plays on from the engine's
  ring, and the block accounting continues. The first pass that finds a slot of the new tag applies
  the seek and drains from that slot. A newer seek replaces the deferred one and inherits its
  count. An anchored seek and a suspended `prepare-seek` are unchanged.
- **Root ruling 2: bound B = 50 ms of rendered audio, in quanta, capped at half the engine ring.**
  When B is reached the seek applies as before #2185. The Worker caps B at half the engine ring
  (19 quanta at 48 kHz/128 with the default ring). The prelude does not know the engine ring's
  size (only `resolved_source_ring_frames` says it, and the feed may not call it: it is not in the
  render-thread export table), so its B is not capped; an uncapped bound only keeps the old
  generation playing longer before a writer that never publishes, never a longer silence.
  Recorded for root review.
- **Root decision 3 (2026-10-29): old-position audio for up to B after a live plain seek, instead
  of silence.** Reason: a silence insert is two cuts and a hole; the deferral is one cut, at most
  50 ms late. A plain seek is a hard cut in both cases; no crossfade is claimed. The source's seek
  report names the later landing block (#1316/#1387 already allow it).
- **Root ruling 4 (single-mode one-chunk headroom through `source_fill`): not implemented; needs
  root review.** Two findings. (a) The engine already admits the new generation after
  `source_seek` while its ring is full of the old one: a probe on the built module (ring of 4
  chunks, filled to backpressure, one block rendered, `seekSource` at g2) accepted three g2
  chunks at once, and the existing real-Wasm test "actual Wasm paused seek accepts target PCM
  before the first quantum with old queues full" pins the same rule. So the applying pass submits
  its first chunk and render plays it, and the Chromium smoke measures 0 underrun frames in single
  mode without the headroom. (b) The headroom would call `source_fill` from `process()`. That
  export is not in `tools/realtime-policy/render-thread-exports.json`, is not wrapped in
  `render_locked`, and its doc says it is never called from `process()`; listing it would need a
  Rust change (a `render_locked` wrap in `hosts/host-web/src/ffi.rs`, which moves the Wasm module,
  or an entry in the realtime-policy tool's unwrapped list). The brief forbids Rust changes, so
  there is no headroom row and no per-block cost to record.
- **Root ruling 5: the transport path is successor #2205.** `engine.seek(frame, { producers })` is
  a plain `session_seek` first: render pops it at its next block and drops every source's old
  chunks before new PCM exists, so a drain deferral cannot reach it. #2205 makes it an anchored
  session seek.
- **Class.** The underrun zeros at a live plain seek become old-position PCM, and the new
  generation's first rendered sample moves later by the deferral. No pinned digest has a live plain
  seek (`--check-matrix` passes, below); no Wasm byte moves.

## Gates and evidence (attempt 1)

- New case-table row (`sdk/test/msb1-consumer-cases.mjs`), "#2185: a live plain seek over a full
  ring of the old generation frees its slots and waits for its first chunk; the bound applies it".
  Red on the #2184 checkpoint's consumers (`b482b07b9`) on both adapters: the prelude and the
  control Worker each asked `source_seek(2, 40)` in the pass that freed the stale slots ("the
  stale slots are freed and no seek is asked before a chunk of its generation"). A prelude mutant
  with no bound fails "the bound applies the seek".
- Adjusted, not deleted: two rows publish a chunk of the new generation after their seek (#1644
  "another refusal at the session generation", "a seek clears the finished mark"); the Worker's
  headroom rig publishes the anchored seek its `miso.seek-at.v1` names (it published a plain one,
  which now waits for its chunk) and asserts that the plain seek waits for its first chunk.
- Qualification smoke, live plain seek phase (`hosts/host-web/qualification/pcm-feed-suspended-
  entry.ts` `runLivePcmFeedSeek`, run by `pcm-feed-suspended.mjs` on an isolated page in both
  instance modes): a main-thread writer keeps the ring full on a 5 ms timer, 5 plain seeks 300 ms
  apart, each row the engine's underrun frames for the source over the seek (its seek report).
  Chromium is gated at 0 per seek; WebKit is reported. Underrun frames per seek, null sink:

  | browser, mode | before (`b482b07b9` consumers) | after |
  | --- | --- | --- |
  | Chromium 151, worker | 512, 0, 0, 0, 0 and 0, 0, 0, 0, 0 (two runs) | 0, 0, 0, 0, 0 (three runs) |
  | Chromium 151, single | 1152, 1920, 1152, 0, 1152 | 0, 0, 0, 0, 0 (three runs) |
  | WebKit, worker (reported) | 384, 256, 0, 0, 384 | 0, 0, 128, 256, 0 |
  | WebKit, single (reported) | 0, 0, 0, 0, 0 | 0, 0, 0, 0, 128 |

  The baseline window (500 ms of steady play before the first seek) was 0 in every run. WebKit's
  headless sink renders in bursts (#1387), so its rows are not gated.

## Attempt 2 (2026-10-29, verify of 66dd43bad: FAIL)

- **Blocker 1: a newer seek's first chunk was freed as STALE.** Root ruling: do both. (a) Both
  consumers classify each slot by its u64 generation against the snapshot's seek generation
  (`sharedSlotAge`, the rule `releaseStaleSharedSlots` states): lower is stale and freed, equal is
  current, higher belongs to a newer seek and stops the pass, unfreed and unsubmitted. Before any
  seek (epoch 0) the engaged tag decides, as before. (b) The write index is read inside the seek
  snapshot (#2184 attempt 2). New row "#2185: a newer seek and its chunk published after a pass's
  snapshot survive to be submitted after their seek", with an `onSnapshot` seam in both adapters
  (a hook right after `readSharedSeek`). It covers the drain (g2 applied, g3 and its chunk land
  behind it) and a deferral (g4 deferred, g5 and its chunk land). Red on 66dd43bad on both
  adapters; mutants of the current code with the old scan (a live write index and a tag compare)
  are red on the drain phase alone and on the deferral phase alone, on both adapters.
- **Major 3: the one-quantum misses.** One bounded investigation, with temporary counters in the
  MSB1 reserved words 34/35 and a seek-report timeline polled every 8 ms (not committed):
  - *Single mode: cause found and fixed in this JS.* Every miss came from a seek applied with no
    deferral (deferral 0, the writer's first fill right after its seek found free slots): the
    pass that saw the first new chunks applied the seek with only those few chunks published, and
    the writer's next chunks came a writer tick (5 ms, about two blocks) later, so the engine ran
    one quantum dry. Fix (both consumers): a deferred seek applies only once its generation has a
    **runway** published, `min(B, ring capacity)` chunks (B is the bound, 19 at 48 kHz/128), or
    a chunk that ends its region; the bound still applies it after 50 ms. The #2185 row now
    asserts that one chunk is not the runway: red on 296cb0298 (the prelude) and on a runway-1
    mutant (the Worker).
  - *Worker mode: cause found; not closable with a plain seek; root ruling needed.* After the
    runway fix, Chromium single mode had 0 misses in 190 seeks (38 runs: 12 on the final build, 26
    instrumented), and worker mode 4 misses of 128 frames in 190 seeks. The timeline puts each miss at the seek's landing block: the first
    report after the seek already names the new generation with 128 missing frames, and the count
    never grows. The bound never fired, and the deferral had published its runway (19 to 64 chunks
    submitted in the applying pass). Cause: the Worker applies the plain `source_seek` from its own
    thread while render runs; render pops the seek at its next block and drops the old chunks, and
    a block that starts between the Worker's `source_seek` and its first `source_submit` (the same
    pass, microseconds apart) renders the new generation with no chunk. The order cannot change
    (the engine refuses a new-generation chunk before its seek), and single mode has no such window
    (one thread). Options for root: (1) narrow the window by reading the report and the fill
    before the seek, which makes the miss rarer but does not remove it; (2) the Worker applies a
    live plain seek as an anchored seek (`source_seek_at`) at a sample a few quanta ahead: the
    engine keeps the old generation playing until the anchor, so the chunks land first. This is JS
    only, but it changes the seek's semantics and latency (#2205 makes the same choice for the
    transport path); (3) an engine primitive (Rust). The Chromium worker gate stays at 0 frames
    and is red about once in 47 seeks (a 5-seek run is red about one time in 10).
- **Minor 4:** the Worker headroom rig no longer repeats the row's "no seek before its
  generation's first chunk" assertion (its anchored-seek repair stays). The Worker's own seek
  smoke publishes a runway (its 4-slot ring) and now checks four submitted chunks.
- **Test rates.** Both adapters run the case table with B = 4 quanta (the prelude fake at 320 Hz,
  the Worker fake's engine ring at 8 quanta), so a case can publish a runway (`publish`) inside
  its 8-slot rings. Rows that seek and expect the seek to apply in one pass now publish a runway.
  Rows that drain over several passes let the bound apply the seek.
