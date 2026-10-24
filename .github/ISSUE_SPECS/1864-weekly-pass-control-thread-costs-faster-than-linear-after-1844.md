# Weekly pass: the control-thread costs that stay faster than linear after #1844

Filed by root from #1844's successor note (backlog sprint, 2026-10-24). Weekly optimization pass: descriptive, no release budget is missed.

Three control-thread costs stay
  faster than linear after this issue, measured on the session above (release, one warmup, two
  rounds):
  1. First-plan preparation: 28.2 / 28.2 ms at 256 strips and 154.6 / 153.3 ms at 1024 (5.5x for
     4x). The same session with its automation table cleared prepares in 23.0 / 22.9 ms and
     100.4 / 99.4 ms (4.3x), so about 5 ms at 256 and about 54 ms at 1024 are automation-specific
     and grow faster than linear. The cause was not found: every automation reader this issue
     names is indexed, and no profiler was available (`perf_event_paranoid` 4, no valgrind).
  2. `controls::continued_fader_lanes` and `continued_mute_lanes` scan both models' automation
     tables per strip in a successor's strip join and per following route in its route loop.
  3. `LevelRules::vca_offset_moved` scans the committed table per reaching VCA of every strip
     whose level records are asked (a successor's strip join, the classifier's changed strips).
  Items 2 and 3 are the measured successor gap: successor preparation is 32.6 / 32.7 ms at 256
  and 224.0 / 222.8 ms at 1024 strips (6.8x), against 23.9 / 24.0 and 106.0 / 105.5 ms (4.4x)
  without automation.

## Gates

Index items 2 and 3 through `AutomationIndex`/`ModelView` (the #1710/#1844 pattern), then measure item 1 with a profiler if one is available. Class A: the list rows, catalog and live-lane handles stay byte-equal (one-time PR evidence). One descriptive timing at 256 and 1024 strips.

## Decision and evidence record

- Item 2 (root): `controls::continued_fader_lanes_in`/`continued_mute_lanes_in` read both models
  through `ModelView`s and take each model's VCA reach slice; the free functions stay as wrappers
  (whole views, `vca_reach_of`). `AutomationIndex` gains `any_vca_offset`, so the indexed "any VCA
  row" early-out is a flag read. The strip join and the route loop ask
  `EffectiveDiff::continued_lanes` (the preparation's indexed reads, `reach`/`reach_before`); the
  route's committed position is `controls::strip_index` of its source.
- Item 3 (root): `LevelRules` owns the two `AutomationReads`; `vca_offset_moved` reads `current`
  through its reads and `next` through `AutomationReads::view_of(self.next)` (the edited view, a
  hand move's offset). The classifier's local diff adds a lazy `current` reader at
  `STAGE_INDEX_OWNERS`, so a one-owner edit keeps the scan (#1710).
- Item 1 (bounded stage split, scratch harness, not committed): at 1024 strips the
  automation-specific first-plan stage was `controls::automation_diagnostics` (41.7 ms with the
  table, 0.04 ms without; 2.5 ms at 256). Its HPF/LPF order rule scanned the whole table twice per
  strip and lane; it now looks the filter entries up from one pass (`filter_entries`), and skips
  the strip walk when no entry is on a filter row. The stage is now 6.0 ms at 1024 and 0.4 ms at
  256: the rest is `registry::resolve` per entry, whose fader row reads
  `builtin_rows::model_fader_offsets` (a table scan) and whose VCA `offset_db` row reads
  `vca_offset_automation` (a scan), O(entries^2). Those files (`strip.rs`, `builtin_rows.rs`,
  `vca.rs`) are outside this slice's paths: a successor candidate, not fixed here.
- Tests: one row in the #1710 block (`event_lanes_take_the_own_entry_and_every_reaching_vca_ride`):
  the indexed `_in` forms with `vca_reach()` slices equal the scans on every ordered pair of five
  tables (the block's two, a moved VCA ride, a member leaving its VCA, VCA `offset_db` rides only).
  Red when the indexed "any VCA row" drops `any_vca_offset` (the first four tables all carry a VCA
  mute entry, so the fifth is what makes it red). The filter index is held by the existing order
  tests (red when a `both` entry no longer drives each lane). No new test file, no counter.
- Class A, one-time PR evidence: on #1710's generated session (256 tracks) and #1844's VCA-mute
  variant, the first plan's and five successors' live-lane handles, every `StripJoin`
  (`continued_cells`, fader and matrix family records), every `RouteJoin` retarget and every
  following route's `continued_vca` are byte-equal before (main 87b5ee63c) and after. The successor
  bases: #1844's mute/fader edit, a moved VCA ride (86 strips' cells 15 -> 12), a member leaving
  the VCA, a VCA hand offset (86 live `FaderOffsetsRecord`s with the VCA length, item 3's path) and
  a moved own ride; the VCA-mute variant has `continued_vca` false on the member that leaves.
- The class A dumps cover only successors whose strips keep their position (committed position
  == new index), and the reaches were not read by any host-core test: a mutant that gives
  `EffectiveDiff::continued_lanes` empty reaches left every host-core test green (verifier MAJOR).
  A row in `added_removed_and_changed_vca_mute_entries_carry_the_member_stage` closes both: the
  member keeps its own flat mute entry, `va`'s mute entry changes and the successor adds a muted
  track ahead of it (position 0, index 1). A stale continuation changes output only inside the
  `VcaMute` ramp window (from the next block boundary after it the run equals a fresh plan), so
  the row asserts every sample equal to the same swap without the own entry (a VCA-only lane,
  which repositions whatever the reach), and from the settled boundary equal to a fresh plan. Red
  under the empty-reach mutant. A mutant reading the committed reach at the new index stays green
  here (the committed model has no strip there, so it reads an empty reach on one side only,
  which also repositions). A following route reads the mute half alone
  (`EffectiveDiff::continued_mute_lanes`).
- Timing (release, one warmup, two rounds, `flock -x`, load average about 4-5 on 32 cores; the
  session as #1844's rows), ms, rounds 1 / 2:

  | | 256 before | 256 after | 1024 before | 1024 after |
  |---|---|---|---|---|
  | first-plan preparation | 30.1 / 28.8 | 26.7 / 26.7 | 159.2 / 160.8 | 122.5 / 121.4 |
  | successor preparation | 34.5 / 33.8 | 28.5 / 28.4 | 230.4 / 226.2 | 134.1 / 133.0 |
  | no automation, first plan | 24.0 / 23.9 | 23.9 / 23.9 | 105.1 / 105.9 | 105.4 / 105.3 |
  | no automation, successor | 25.5 / 25.1 | 25.2 / 25.2 | 121.4 / 113.3 | 112.0 / 112.3 |

  Successor stages at 1024: strip/route joins 47.0 -> 5.1 ms, following-route cells 20.0 -> 1.2 ms.
  First plan with automation is now 4.6x for 4x strips (no automation: 4.4x); the successor is
  4.7x (4.5x). No profile was possible (`perf_event_paranoid` 4).
