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

## Decision and evidence record

- Slice (root, from the scope note): `AutomationIndex` keeps its model (the event lanes read a
  reach's VCA IDs) and gains `automated_fader_lanes`, `fader_event_lanes`, `mute_event_lanes` and
  `any_vca_mute`; `ModelView` mirrors them (index when carried, else the scan). The scope note's
  `strip_index` replaces the spec's ID-to-position map in `ModelInputs::new` (O(log n), no
  allocation, the same answer since strip IDs are unique).
- Preparation builds one index per model (and one of the predecessor's base for a successor) and
  reads the following routes, their count, the VCA mute cells and the strip and send joins through
  it. `FaderEventLanes::of` and `following_route_lanes` take a `ModelView`. The classifier's diffs
  and its route lowering read through `AutomationReads::lazy`, which indexes once past
  `STAGE_INDEX_OWNERS` reads, so a one-owner edit keeps the scan (#1710's measurement).
  `controls::live_lanes` walks its owners through one index.
- Root option taken: the 1024-strip preparation stayed faster than linear after the host-core
  slice, so `builtins-compiler` (the fader/mute automation plan and the offsets-cell set) and
  `graph-compiler` (`StripGains`) each build a local index. No signature change across crates.
- Class A, one-time PR evidence: the 256-strip `controls::list` rows, the catalog tables (owners,
  sections, descriptors, rows, value bits), `live_lanes` and the prepared first plan's and a
  successor's live-lane handles (strips, offsets cells, route and following-route producers with
  their `automated` lanes) are byte-equal before and after, on #1710's generated session and on a
  variant with a VCA `mute` entry and own mute entries on half the tracks.
- Timing (release, one warmup, two rounds, `flock -x`, load average about 5-8 on 32 cores; #1710's
  generated session: one fader entry, one mute entry and one following send per track, one VCA
  ride), ms, rounds 1 / 2:

  | | 256 before | 256 after | 1024 before | 1024 after |
  |---|---|---|---|---|
  | first-plan preparation | 31.1 / 30.8 | 28.2 / 28.2 | 212.0 / 209.3 | 154.6 / 153.3 |
  | successor preparation | 38.4 / 38.1 | 32.6 / 32.7 | 322.9 / 323.7 | 224.0 / 222.8 |
  | `ControlCatalog::build` | 2.66 / 2.67 | 2.56 / 2.46 | 13.3 / 13.5 | 10.1 / 10.2 |
  | `controls::live_lanes` | 1.59 / 1.57 | 0.74 / 0.75 | 18.0 / 18.3 | 3.19 / 3.19 |

  The catalog build and `live_lanes` are now linear (4.0x and 4.3x for 4x strips). The same
  session with no automation prepares in 23.0 / 22.9 ms and 100.4 / 99.4 ms (4.3x), so the
  remaining superlinear part of a first plan (about 5 ms at 256, 54 ms at 1024) is
  automation-specific but outside the readers this issue names; no profile was possible
  (`perf_event_paranoid` 4, no valgrind). A successor still scans per strip in
  `controls::continued_fader_lanes`/`continued_mute_lanes` (the strip join and each following
  route) and in `LevelRules::vca_offset_moved` (per reaching VCA): successor candidates.
