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
