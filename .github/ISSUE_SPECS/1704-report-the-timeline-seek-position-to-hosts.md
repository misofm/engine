# Report the timeline seek position to hosts

Filed by root from ruling B16-R4 (batch 16, 2026-10-16).

## Product outcome

A host can read where the session's timeline (#1640, #1641) last took a seek, in the same form as
a source's seek report (#1316): the generation in effect, the source-read sample of the first
block it played in, and the timeline value that block started at. The underrun and held counts are
0. Both hosts read it through one core call.

## Context

- #1641 D6 planned this row. Root ruling B16-R4 deferred it, with its bytes (#1641 D7) and its
  move at a carry (#1641 D3), because no host had a reader.
- A source's report is one seqlock record per ring (`crates/engine/src/realtime/seqlock.rs`),
  read by `SourceControlSet::seek_report` and by `miso_engine_v1_source_seek_report` and
  `miso_engine_web_v1_source_seek_report` (#1293).

## Decisions to make at filing

- The host form: a reserved source ID in the existing report call, or a new call. A new direct
  export needs a root ruling under decision 17 C13.
- The record and its writer move with the timeline consumer at every carry (#1641 D3, D4).

## Dependencies

- #1641 and #1642. The first slice that needs a host reader of the timeline position (for example
  the browser SDK `seek` of #1644, or an automation reader such as #1647).

## Decision record (attempt 1, 2026-10-24)

- Host form (root): the reserved ID `$timeline` in the existing `seek_report` call on both hosts;
  no new symbol. `host_core::source::TIMELINE_SEEK_REPORT_ID`. A stable ID starts with `a`-`z`, so
  `$timeline` never names a source. `submit`, `seek` and `seek_at` refuse it as `source.id.unknown`.
- D1: `SourceTimeline::prepare` makes one 5-word seqlock record; `TimelineProducer::seek_report`
  reads it; `TimelineConsumer` holds the writer (`None` when vacant) and its published copy.
- D2: `advance` publishes `(generation, source-read sample, timeline sample, 0, 0)` when a new
  generation is in effect, so a primed and a rendered block both count, as for a ring.
- D3: `adopt_from` swaps the writer and the published copy with the command queue.
- D4: `TimelineResourceReport::seek_report_bytes` (one allocation, also its largest row). A
  vacant timeline charges it as carried, so `source_overhead + carried == fresh` holds. Browser
  `sourceOverheadBytes` is 3042 of its 3264 ceiling: no ceiling moved.
- Path note: `crates/host-core/tests/successor_swap.rs`'s carried-bytes helper adds the record
  (its exact carried-bytes equalities need it). The browser export's doc comment
  (`hosts/host-web/src/ffi.rs`) is not in the authorized paths, so it is not changed.
