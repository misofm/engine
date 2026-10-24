# Index the automation entries per strip once per build

Filed by root from batch 17 (#1650b's successor note; controls coordinator, 2026-10-16), for the
weekly performance pass. It is control-thread cost only; render is not touched.

## Product outcome

Building the control list and the control catalog costs O(strips + entries) in the automation
table, not O(strips × entries). The rows, their values and the `automated` flag do not change.

## Context

- Since #1650b, a strip's `fader_db` row reads the offset of the rendered automation entry that
  drives its lane, and the list and catalog carry an `automated` flag per row (decision 17 rows in
  `crates/controls`; catalog in `crates/host-core/src/control_provider.rs`, `ControlCatalog::build`
  and `stage_live`).
- Each strip finds its driving entries by a scan of the whole `automation` table
  (`controls::automated_fader_lanes`, `fader_automation`, and the row bindings), so a build over a
  session with many strips and many entries scans the table once per strip.
- #1651 to #1667 add more automated rows (mute, pan, matrix, sends, trim, effects); each one that
  binds the same way multiplies the cost.

## Decisions to make at filing

- D1. One pass over the table per build (and per staged live edit when the owners model holds the
  entries) builds a per-owner index of the rendered entries that drive each control address; every
  row binding reads the index. The index is built on the control thread, before any reply, and is
  charged where the catalog's transient is charged (#1400b).
- D2. Gate: a session of 256 strips with one rendered fader entry each builds the list with the
  table walked once (a counter or a bound on comparisons), and the list bytes equal the unindexed
  build's bytes.

## Dependencies

- #1650 (the binding and the flag). Measure first in the weekly pass; no change unless the
  measured build time of a large session warrants it.

## Amendment (controls coordinator, batch 18)

- Mute entries render since #1651 (row 6). The per-owner index also serves the mute row's readers
  (`controls::mute_automation`, the listing's `automated` flag on a driven mute row, R18-2) and the
  following-route predicate of #1652 (`following_route_lanes`), so the gate's session also carries
  one rendered mute entry per strip and a following send per strip.

## Decision record (attempt 1, 2026-10-24)

- R1 (root, measure first): `ControlCatalog::build`, release, one warmup and two rounds, on a
  generated session of N tracks with one fader entry, one mute entry and one following send per
  track (plus one VCA ride): before, 256 strips 4.0 ms and 1024 strips 39 ms (load average about 8
  on 32 cores). Over the 5 ms stop line, so implemented.
- After (same session, same runner, load average about 6): 256 strips 2.7 ms, 1024 strips
  13.5 ms. The 256-strip list and catalog entries are byte-equal before and after. The build
  still grows faster than linear (about 5x for 4x strips); see the successors.
- D1 as `controls::AutomationIndex<'a>` (`crates/controls/src/automation.rs`): one pass over the
  table fills a map of (rendered row, owner ID) to the first entry per lane in table order, for
  the strip `fader_db` and `mute` rows and the VCA `offset_db` and `mute` rows. It is a value of
  one build (R3), not a cache on `SessionModel`.
- The index rides in the `ModelView` the row readers already take (`ModelView::indexed`), so
  `owner_rows_in`, `ModelView::fader_automation`/`vca_automation`, the mute readers and
  `LocalEffective` (the stage's effective values) read it with no signature change. `list`,
  `ControlCatalog::build` and `stage_live` build it once each. A view without an index (the
  one-owner readers: `owner_rows_with`, `live_delta`, `owner_edits`) keeps the scan. The dense
  `controls::effective` already walked the table once for the strip faders (`model_fader_offsets`).
- R2: no production counter; the timings and a one-time byte comparison of the 256-strip list
  and catalog are PR evidence. Rows, values and the `automated` flag are unchanged.
- Verifier fold: `stage_live` for a one-owner fader edit (the submix's left fader) at 1024
  strips, release, one warmup and two rounds: before 5.05 / 5.03 ms. With the index built on every
  stage it was 5.40 / 5.32 ms (+4 %, over the 2 % allowance), so a stage builds the index only
  when it reads more than `STAGE_INDEX_OWNERS` = 4 owners (changed plus reached derived owners).
  `model_vca_offsets` (the dense effective values) now fills every VCA's offsets in one pass
  over the table, not one scan per VCA. With both changes: before 5.03 / 5.00 ms, after 5.07 /
  5.01 ms (+0.5 %; load average about 8).
- Successors (not in this issue; filed as #1844): preparation's `FaderEventLanes::of` and
  `following_route_lanes` (`prepare.rs`) and the live classifier (`live_delta.rs`) still scan the
  table per strip and per route; `controls::live_lanes` walks every owner through the unindexed
  `owner_rows_with`; and `controls::ModelInputs::new` finds each route's source strip by a linear
  `strips().position`, which keeps the catalog build faster than linear after this change.
