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
    transport path); (3) an engine primitive (Rust). At 0 frames it would be red about once in 47
    seeks; the root ruling below splits the claim instead.
- **Minor 4:** the Worker headroom rig no longer repeats the row's "no seek before its
  generation's first chunk" assertion (its anchored-seek repair stays). The Worker's own seek
  smoke publishes a runway (its 4-slot ring) and now checks four submitted chunks.
- **Test rates.** Both adapters run the case table with B = 4 quanta (the prelude fake at 320 Hz,
  the Worker fake's engine ring at 8 quanta), so a case can publish a runway (`publish`) inside
  its 8-slot rings. Rows that seek and expect the seek to apply in one pass now publish a runway.
  Rows that drain over several passes let the bound apply the seek.

## Status after attempt 2: single mode met, worker mode open

- **Met: single mode.** A live plain ring seek during playback underruns 0 frames on Chromium in
  single mode: 0 misses in 190 seeks (38 runs). The qualification's Chromium single live-seek
  phase is a gate at 0 frames.
- **Not met: worker mode.** Chromium worker mode misses 128 frames in 4 of 190 seeks. The
  qualification's Chromium worker live-seek phase is reported only, and its output names it as
  the open #2185 worker residual. The 0-frame claim is not declared met for worker mode.
- **Cause (worker mode).** The control Worker applies the plain `source_seek` from its own thread
  while render runs. Render can pop that seek between the Worker's `source_seek` call and its first
  `source_submit` in the same pass. That block then renders the new generation with no chunk, and
  one quantum is lost. The order cannot change, because the engine refuses a chunk of the new
  generation before its seek. Single mode has no such window, because it runs on one thread.
- **Options, for root to decide together with #2205 (none is picked here):**
  1. Read the report and the fill before the seek. This narrows the window, so a miss is rarer,
     but it does not remove it.
  2. The Worker applies a live plain seek as an anchored seek (`source_seek_at`) a few quanta
     ahead. The engine keeps the old generation playing until the anchor. This is JS only, but it
     changes the seek's semantics and latency.
  3. An engine primitive that applies a seek and its first chunk together (a Rust change).

## Decision (attempt 2, root ruling 2026-10-29)

Split the claim. Do not tune the threshold, and do not change the seek semantics. The batch PR
closes #2184 and delivers #2185's single mode, with the Chromium single live-seek phase gated at 0
frames. #2185 stays open for worker mode: its residual (above) waits for root's choice among the
three options, together with #2205. Until then, the Chromium worker phase is reported only.

## Attempt 3 (2026-10-29, verify of 5aeb11375: FAIL)

- **Major 1: the runway rule added silence to the transport seek.** `engine.seek(frame,
  { producers })` (the console's seek) sends a session seek at g first, so render moves every
  source to g and drops its old chunks. Each producer's plain ring seek at g was then deferred until
  `min(B, ring)` chunks of g existed or B passed, and every block of that wait was new silence. At
  88.2 and 96 kHz the runway (35 and 38 chunks) is more than one 4096-frame pump window (32 chunks),
  so every transport seek waited for the second Blob read or the bound. Fix, in both consumers: a
  ring seek at the generation of the last session seek the engine accepted (#1644's
  `lastSessionSeekGeneration`) is never deferred; it applies at once and drains, as before #2185.
  The other paths where the engine holds no old audio of the source take no deferral already: a
  prepared or suspended seek (`prepare-seek`) applies directly, and an anchored seek is never
  deferred. New row "#2185: after a session seek at g, a ring seek at g with one chunk published
  submits it in the first pass". Red on 5aeb11375 on both adapters ("the first pass submits it").
- **Minor 2:** `sdk/README.md` now says that a live plain seek does not cut to silence only in
  single mode, and that worker mode is the open residual. It also says that the transport seek's
  ring seeks apply at once (#2205 still owns that path's gap).

## Attempt 4 (2026-10-29): the worker residual through #2205's engine rule

- **Decision.** Owner decision (2026-10-29) and root rulings 2-5 of the #2205 plan (recorded in
  the #2205 spec). The engine holds a live plain seek until its runway is in the engine ring,
  bounded by 50 ms, and then lands it in one cut at its target. That closes the render-side race
  the Worker could not close (option 3 above, as a landing rule in `crates/source`). The JS
  deferral is deleted in both consumers (`deferSharedSeek`, `sessionMovedSharedSeek`,
  `seekDeferralBlocks`/`seekDeferralQuanta`); `sharedSlotAge`, the #1644 stale-refusal rule and
  #1387's headroom admission stay.
- **Root ruling 3: H = R.** The Worker's headroom is the engine's runway by the engine's formula
  (`feedHeadroomQuanta`, from the ring the boot resolved), so the runway always fits in one pass.
  Soundness against #1387's own bound (one Worker tick plus a 2048-frame render burst plus one
  block): R meets it at 48 kHz (19 = 19), 88.2 kHz (35 > 20) and 96 kHz (38 > 20). At 44.1 kHz,
  R is 18 quanta, one quantum below the conservative bound of 19, but above the measured burst
  (13 quanta + tick + 1 = 16). Recorded for root.
- **Deleted tests (the deferral they described is gone):** the case-table rows "#2185: a live
  plain seek over a full ring of the old generation frees its slots and waits for its runway; the
  bound applies it" and "#2185: after a session seek at g, a ring seek at g with one chunk
  published submits it in the first pass"; the deferral phase of "#2185: a newer seek and its
  chunk published after a pass's snapshot survive..." (its drain phase stays); the runway helper
  `publish` and the adapters' `seekBound`. The Worker headroom rig's #2185 comments are gone; its
  numbers follow H = R (38 at 48 kHz/64). The MSB1 table passes on both adapters.
- **Qualification.** The Chromium worker live-seek phase is gated at 0 (the "reported, not gated:
  open #2185 worker residual" label is gone). Null sink, Chromium 151, 10 runs x 5 seeks per mode:

  | per-source live seek | 2b0a64d88 | this branch |
  | --- | --- | --- |
  | Chromium worker | 3 / 50 seeks underrun (128 frames each) | 0 / 50 |
  | Chromium single | 0 / 50 | **46 / 50 (128-3200 frames): red** |
  | WebKit worker (reported, 3 runs) | - | 0 / 15 |
  | WebKit single (reported, 3 runs) | - | 3 / 15 (128 frames) |

## Open after attempt 4: single mode regressed; root decision needed

- **Cause.** In `single` mode, `AudioWorkletEngineHost::seek_source`
  (`hosts/host-web/src/lib.rs`) prepares the plan's consumer at once (`prepare_source_seek`, a
  `SeekClock::Hold` apply) whenever the render half is on the same instance. That is how the
  suspended `prepare-seek` op (and the headless SDK's `seekSource`) frees a full engine ring, so
  new PCM can enter before the first block. The prelude's live drain calls the same export
  (`miso_engine_web_v1_source_seek`). So a live ring seek still drops the old audio at once, and
  the engine's landing rule never sees it. The deleted prelude deferral was what kept single mode
  at 0. Worker mode is not affected: its `source_seek` reaches the control instance, which has no
  render half to prepare.
- **No fix was made.** Every candidate goes against an instruction: the two callers are the same
  export with the same arguments, so host-web cannot tell a live seek from a suspended one.
  Options for root:
  1. Split the browser export, as the C ABI already behaves: `miso_engine_web_v1_source_seek`
     never prepares (the landing rule), and a new export prepares a suspended or headless seek.
     This adds a browser ABI symbol (abi-layout, generated SDK, realtime-policy tables, check
     lists).
  2. Keep the prelude's deferral for `single` mode only (revert the prelude half of S3; the Worker
     half stays deleted). No ABI change; single mode was 0 in 190 seeks with it (attempt 2).
- Until root decides, the Chromium single per-source phase is red. The rest of that leg passes
  with the phase ungated (one run). The transport phase and worker mode are met.

## Decision (root ruling 2026-10-29, still attempt 1): option 1, split the browser export

- **The split.** `miso_engine_web_v1_source_seek` never prepares the consumer, also in `single`
  mode, so a live ring seek from the PCM feed's drain reaches the engine's landing rule. This is
  the C ABI's behaviour. One new export, `miso_engine_web_v1_source_prepare_seek` (same arguments,
  results and thread rule), seeks and prepares the consumer at once (`SeekClock::Hold`). The seek
  applies before the next block and the full engine ring is freed for the refill. With the render
  half on another thread it is exactly `source_seek`.
- **Callers of the prepare export:** the suspended `prepare-seek` op (`applySharedSeek(...,
  prepare = true)` in the prelude and in the Worker) and the headless SDK's `seekSource`
  (`sdk/src/core/boundary.ts`). The live callers stay on `source_seek`: the drain, the engine
  worklet's `seekSource` and the Worker's drain.
- **Where it is listed:** documented beside `source_seek` (`hosts/host-web/src/ffi.rs`, host
  `.d.ts`); in the export lists (`check-web-audioworklet.sh`, `check-abi-layout-v1.py`,
  `parameter-metadata` `EXPORTS`, the layout self-test fixture); in the regenerated SDK layout
  (`sdk/assets/...abi-layout.json`, `sdk/src/generated/abi.ts`); and in the realtime-policy
  render-thread table (feed, `applySharedSeek`, wrapped in `render_locked`, as `source_seek` is).
  `check-browser-expected-resources.py` pins no export set.
- **Worker headroom (root ruling, replacing H = R):** `H = max(R, B)`, computed once in
  `feedHeadroomQuanta`. `B` is #1387's proven bound (one tick plus a 2048-frame burst plus one
  block). H is 19 at 44.1 kHz and 48 kHz, 38 at 48 kHz/64 in the headroom rig.
- **The test (one claim):** host-web
  `session_seek::a_live_source_seek_keeps_the_old_audio_and_the_prepared_form_applies_at_once`.
  It is red on a mutant whose live export prepares at once (164ced262's behaviour: "the old
  generation plays on", left 2, right 1). On 164ced262 itself the prepare export does not exist.
  `boot_transient_budget`'s paused-seek row now calls the prepare form, since it is a paused seek.

## Evidence after the split

Null sink, Chromium 151, 10 runs x 5 seeks per mode (head `1d8f2ea76` artifacts):

| phase, mode | underrun seeks | added latency ms (min/median/max) |
| --- | --- | --- |
| per-source, worker | 0 / 50 | - |
| per-source, single | **0 / 50** (164ced262: 46 / 50) | - |
| transport, worker | 0 / 50 | 13.3 / 24.0 / 45.3 |
| transport, single | 0 / 50 | 48.0 / 48.0 / 50.7 |

- WebKit (reported, one run): 0 underrun in all four rows; transport latency 18.7-26.7 ms
  (worker) and 40.0-45.3 ms (single).
- `--check-matrix --sdk-root` passes on Chromium worker and single, and on WebKit worker and
  single. WebKit worker failed once on `sdk-protocol-lossy` (#2198) and passed on its rerun.
- The full check list passes again: fmt, clippy (`-D warnings`, all features), doc, the feature
  test set (105 binaries), workspace and realtime policy, the release audit's C ABI leg (0
  violations), the artifact build and `check-web-audioworklet.sh`, `test-web-audioworklet.mjs`,
  the SDK headless/types/generated/deletions/package checks, and the layout self-test.

## Status

Met in both modes: a live plain ring seek during playback underruns 0 frames on Chromium.

## Attempt 2 of the #2205 batch (2026-10-29)

Root ruling: the landing rule covers live seeks only. The #2205 spec records the at-once session
seek, `miso_engine_web_v1_session_prepare_seek` and its routing. For this issue:
- **Routing assertion:** the case table's new row asserts the per-source routing, `prepare-seek`
  to `source_prepare_seek` and the drain's live seek to `source_seek`, on both adapters.
- **Never-fed rule:** it is now a `crates/source` test.
- **Worker runway:** the formula is pinned to the engine's values at all four rates (MINOR 4).
- **Per-source live seek, Chromium, 5 runs x 5 seeks per mode:** 0 of 25 underrun in both
  worker and single mode.

## Attempt 3 of the #2205 batch (2026-10-29)

Root ruling: the C ABI is in scope. `miso_engine_v1_session_seek_at_once` gives a stopped mobile
host the at-once landing (see the #2205 spec). The per-source live seek is unchanged on the C ABI:
it lands when ready, which is this issue's rule. A per-source seek while stopped has no at-once
form there; the restart-refill gap (rings filled before the stop) is root's mobile successor, a
residual of this batch. Chromium per-source live seek, 3 runs x 5 seeks per mode: 0 of 15
underrun in both modes.
