# Report the timeline seek position to hosts

Placeholder from root ruling B16-R4 (batch 16, 2026-10-16). A draft for root to file.

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
