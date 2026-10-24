# Index the remaining per-owner automation and route scans

Filed by root from #1710's successor note (backlog sprint, 2026-10-24).

## Product outcome

Preparing a session, classifying a live edit and listing its live lanes cost time roughly linear in
the session's size, not quadratic. #1710 added `controls::AutomationIndex` (one pass over the
automation table per list, catalog build and multi-owner stage), but four readers still scan:

- preparation's `FaderEventLanes::of` and `following_route_lanes` (`crates/host-core/src/prepare.rs`)
  scan the automation table once per strip and once per route;
- the live classifier (`crates/host-core/src/live_delta.rs`) scans per strip and per route;
- `controls::live_lanes` walks every owner through the unindexed `owner_rows_with`;
- `controls::ModelInputs::new` finds each route's source strip by a linear `strips().position`, so
  the catalog build still grows about 5x for 4x strips after #1710.

## Smallest closable slice

Thread one `AutomationIndex` (built once per call) into the first three readers, and replace the
linear source-strip search in `ModelInputs::new` with one ID-to-position map built once. No cache on
`SessionModel`; no counter. Class A: no rendered bit and no listed row may move.

## Gates

- The 256-strip `controls::list` rows, the catalog entries and the prepared plan's live-lane table are
  byte-equal before and after (one-time PR evidence, not a committed digest).
- One descriptive timing of preparation and of the catalog build at 256 and 1024 strips (one warmup,
  two rounds), recorded in this spec.
- Existing controls and host-core tests pass; a new test only if a reader's equivalence with its scan
  is not already covered by #1710's index assertion block.

## Non-goals

No change to the automation grammar or to render. No new public API beyond what the readers need.
