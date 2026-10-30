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

## Decision record (attempt 1, 2026-10-29)

- **Owner decision (2026-10-29).** During a live seek, transport or per-source, the old position
  plays on until the new generation has a runway in the engine ring, bounded by D = 50 ms. Then one
  cut lands on the target from its first sample. A session seek lands every source and the timeline
  in the same block. Class B for seek runs is owner-approved. A run without a seek moves no
  rendered bit.
- **Root rulings on the plan's open questions** (plan: render-side landing rule in `crates/source`,
  not an anchored session seek):
  - (2) `R = D` is one engine constant, `min(ceil(50 ms / Q), ring_blocks / 2)`, fixed at prepare
    in `PcmSourceShape`. No C ABI argument and no new symbol.
  - (3) One constant everywhere: the Worker's admission headroom H is derived from the same R.
  - (4) A source that was never fed counts as ready; a vacant source, or a seek past a stem's end,
    does not hold.
  - (5) The V1 seek contract prose changes before launch; no symbol moves.
- **The rule as built.**
  - `PcmSourceRingConfig::seek_runway_blocks` / `PcmSourceShape::seek_runway_blocks` hold R
    (`source::seek_runway_blocks`, wrapped for a ring of frames by
    `host_core::source_seek_runway_blocks`; 19 at 48 kHz/128 with the default ring). A ring config
    of 0 keeps the next-block landing.
  - Each ring has a producer-written readiness word (`SeekReadiness`, one `AtomicU64`): the epoch
    (the producer's seek pushes, matched against render's command pops), fed, waits (the ring was
    fed before this seek), ended, and the epoch's published blocks.
  - `SeekState` takes a plain seek observed at a block (`SeekClock::At`) as a held seek with
    `Landing::WhenReady { since }`. It lands at `frame` when the runway is queued (R blocks of its
    generation, a block that ends the region, a seek to the region's end, or a ring never fed
    before the seek), or at `since + R` blocks. Under `SeekClock::Hold` (`prepare_seek`) a plain
    seek still applies at once.
  - `SessionSeek::publish` names a plain session seek in a second session word. The source set
    observes every consumer first, then decides once (`all ready || any deadline`), then plays.
    So every source holding the session seek and the timeline land in one block.
- **"Never fed" is judged at the seek push**, not at render: a fresh source the host feeds right
  after its seek lands at once (the waits bit). Judged at render, every fresh-plan seek-and-feed
  would have held for D.

## Gates and evidence (attempt 1)

Each new gate was red on the next-block rule (a mutant of the new code that lands every live plain
seek at once, the behaviour of 2b0a64d88); the browser phases were red on 2b0a64d88 itself.

- S1 (`crates/source`, `tests::seek_landing`):
  - `a_live_plain_seek_plays_the_old_generation_until_its_runway_is_queued` (red: lands at the
    next block, underrun).
  - `a_plain_session_seek_lands_every_source_and_the_timeline_once_all_are_ready` (red: next-block
    mutant, and a per-source-landing mutant that skips the set's decision).
  - `a_plain_session_seek_lands_at_its_deadline_with_the_slow_source_underrunning` (red: both
    mutants).
  - Gate 4: `paused_seek_prepares_full_queues_without_consuming_target` now runs with a runway of 1.
  - Gate 5: `tests/prime.rs` `a_held_live_seek_and_its_landing_allocate_nothing`: 0 allocations and
    0 frees across two held blocks and the landing block.
- S2: capi `session_seek::a_plain_session_seek_lands_every_source_once_the_fed_one_has_its_runway`.
  It was red on the next-block mutant ("held: LONG's producer is late") and on a mutant where a
  never-fed source waits (the seek holds to the deadline). The source-crate test
  `a_live_plain_seek_with_no_more_pcm_to_come_lands_at_the_next_block` was red on a mutant that
  ignores the ended bit. The "lands at the next block" tests (capi `session_seek`,
  `session_seek_declares`, the scrubbed-stem and fader rows; host-web `session_seek`; host-core
  timeline, stored fader and mute rigs; control-plane mute realtime; the audit's C ABI leg) were
  rewritten to feed the runway, so each keeps its own claim. None was deleted.
- B16-R9: the discontinuity successor carries the consumers whole (held seek included) and the
  timeline's seek state, so a landing up to D later is in the successor. The capi declares tests
  pass with the runway.
- S4 qualification (`pcm-feed-suspended.mjs`/`-entry.ts`): a new transport phase
  `runLiveTransportSeek`. It calls `engine.seek(frame, { producers })` during playback with a
  producer that publishes 20 ms after the seek, 5 seeks a run. Chromium is gated at 0 underrun
  frames per seek in both modes, and every row holds the source and the timeline to one landing
  block. Null sink, Chromium 151, 10 runs per mode:

  | phase, mode | 2b0a64d88 (underrun seeks / 50) | this branch | added latency ms (min/median/max) |
  | --- | --- | --- | --- |
  | transport, worker | 50 / 50 (640-2176 frames) | 0 / 50 | 10.7 / 24.0 / 48.0 |
  | transport, single | 50 / 50 (896-3200 frames) | 0 / 50 | 34.7 / 48.0 / 50.7 |

  The latency is the landed sample minus `position().atSample` read right after `engine.seek`
  resolved. That read can land up to one block late (base reads 0 or -1 block).
  WebKit, reported (3 runs): 0 underrun in both modes; latency 16-24 ms (worker), 37-45 ms
  (single). WebKit's transport rows count about 60 refused submits a seek (result 1): these are
  old-generation chunks a drain submits between the session seek and the producer's ring seek,
  which the engine refuses as stale. They are reported, not an invariant.
- No bit moved: every `fixtures/session/v1` session (18 that prepare; `canonical.json` is refused
  on both), rendered 300 blocks with no seek through host-core, gave identical output digests on
  2b0a64d88 and this branch (one-time PR evidence, not committed). `--check-matrix` passes on
  Chromium and WebKit, both legs.

## Status after attempt 1

- **Met:** a live transport seek underruns 0 frames on Chromium in both modes; the added latency is
  reported above.
- The per-source half (#2185) has one open item in single mode; see the #2185 spec.

## Root ruling after attempt 1's report (2026-10-29): split the browser export

The single-mode per-source regression (host-web's `source_seek` prepared the consumer at once) is
fixed by splitting the export; see the #2185 spec's decision and evidence.
`miso_engine_web_v1_source_seek` now lands when ready on every host. The new
`miso_engine_web_v1_source_prepare_seek` serves the suspended `prepare-seek` and the headless
SDK. The Worker headroom is `max(R, #1387's bound)`. After the split, Chromium is 0 / 50 underrun
seeks in all four live rows; the transport latency is 13.3-45.3 ms (median 24.0) in worker mode
and 48.0-50.7 ms (median 48.0) in single mode. `--check-matrix` passes on both browsers and both
legs.
