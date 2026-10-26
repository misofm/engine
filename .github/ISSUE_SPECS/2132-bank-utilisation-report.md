# Bank utilisation report and the C1 console rows

Draft slice 0 (#2132) of the banking design (`docs/BANKING_DESIGN.md`, 2026-10-26). ## Class

A: no render arithmetic changes; counters only.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

The design's finish line is a ratio nobody measures: banks bound against the lower bound
`sum over key classes of ceil(n/W)`, and bank-slot runs executed per block against
`sum over slots of ceil(A_slot/W)`. The console benchmark has no C1-shaped row (4 groups, 2 auxes,
a master, `gate, eq, comp | drive`) and no row in the load state (every slot bypassed), which is
the state the console opens in.

## Smallest closable slice

- `GraphRackBankReport` gains `BankUtilisation { banks, mergeable_partial_pairs,
  lane_use_permille }` per rack and in total, computed at compile from the plan.
  `mergeable_partial_pairs` counts pairs of partial banks of one key class (same rack, same leader
  program or a subsequence of it, same pool class, same fold target) at one level (this slice;
  slice 6b (#2136) widens "one level" to "intersecting windows") whose union fits `W`. It is a checkable
  property, not a `ceil(n/W)` bound (window intersection is not transitive and subsequence
  members blur a class).
- The plan's render counters: `bank_slot_runs` per pack group (one per bank per slot executed; a
  skipped slot or chain counts none) beside the existing `silent_skips` and #2129's
  `bypass_skips`, read through one `test_only_bank_slot_runs(plan)` and reported by the bench
  runner per row and per pack group as `bsr_per_block` with the per-group ideal
  `sum over slots of ceil(A_{g,k}/W)` computed by the workload from its own activity state.
- `tools/console-workload`: a `C1Console { tracks, state }` family: 16, 32 and 64 tracks over the
  C1 shape read from the console app's session constants (copied as a fixture under
  `fixtures/session/v1/console-c1-*.json`, not referenced across repos), in four states: `Load`
  (every console slot bypassed, limiter bypassed), `Typical` (seeded: eq 75 %, comp 50 %, gate
  10 %, drive 10 %), `AllActive`, `Sparse` (odd tracks silent), `Toggle` (one strip's gate
  toggled every 64 blocks, so un-bypass transitions are in every later slice's `to_bits`
  evidence). And one `IrregularGraph` row:
  ragged inserts (0-3 per track), one sidechained insert, three submix depths, mono and stereo
  stems mixed, two tracks routed to the master directly.
- `tools/bench console` prints, per row, `banks`, `mergeable_partial_pairs`, `lane_use`, and per
  pack group `bsr_per_block` and `bsr_ideal`, then p50 us per block. Native (W=8) and the V8
  runner (W=4).

## Gates (one test per claim, red first)

1. `crates/graph-compiler/tests/console_banking.rs`: `the_bank_report_counts_mergeable_pairs`:
   on the 64-track console fixture `mergeable_partial_pairs == 0` and `lane_use_permille ==
   1000`; on a constructed two-level session with two partial banks of one key at one level the
   count is 1 and at two levels 0 (this slice's rule). Red on a count that ignores the class or
   counts padded lanes as active. Test value: red when the report stops seeing a pair the planner
   could have merged.
2. `crates/graph` unit test on the `silent_skips` model: `bank_slot_runs` per group is
   `banks x slots` per block on an all-active plan and `0` on a skipped one. Red on a counter that
   counts a skipped slot. Test value: red when the counter and the skips disagree.
3. The feed test of `tools/console-workload` extended by one row: every C1 row is fed through a
   source set and gathered in place (the existing claim, one new row).

## Evidence (recorded, not gated)

The C1 and irregular rows, both widths, one invocation, one warmup and two rounds, under
`artifacts/steps/bank-design-0/`. This is the baseline every later slice compares against.

## Paths

`crates/graph-compiler/src/lib.rs` (report), `crates/graph-compiler/src/banks.rs`,
`crates/graph/src/lib.rs` and `runtime.rs` (counter), `tools/console-workload/src/lib.rs`,
`tools/bench/src/console.rs`, `fixtures/session/v1/console-c1-{16,32,64}.json`,
`fixtures/session/v1/irregular-graph.json`, this spec.

## Dependencies

None. Unblocks the finish-line measurement for every later slice.
