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
