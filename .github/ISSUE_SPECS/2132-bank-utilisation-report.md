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

## Evidence and decision record (attempt 1, 2026-10-26)

### What was built

- **Report.** `graph_compiler::BankUtilisation { banks, mergeable_partial_pairs, lane_use_permille }`,
  in `GraphRackBankReport::utilisation` (every rack) and `utilisation_by_rack` (one per
  `RackLocation`), computed at the end of `bind_rack_banks_indexed` by the pure
  `BankUtilisation::of(groups, bound_slots, W, fold_targets, rack)`. `banks` is the plan groups with
  a bound slot; `lane_use_permille` is the bound slots' active members over `bound slots x W`
  (padded lanes are not active); a mergeable pair is two partial bound groups of one rack, pool
  class and dependency level, with one fold target (the sorted destinations of every member's
  routes, muted routes included, equal across the group, or "mixed"), whose members are all
  subsequences of the other's leader program and whose active lanes together fit `W`.
- **Counter.** `graph::test_only_bank_slot_runs(&mut plan)` returns one row per pack group, then per
  bank unit in no group, with an effect slot: units, `W`, effect slots per unit, runs, whole-chain
  skips. **No render-path state was added.** Every slot of a chain carries the chain's lane mask
  (`bank_chain_with_fold`), so a rendered block runs every slot and a skipped one none; a unit's
  runs are therefore its effect slots (slots that are not a `BuiltinStage`) times its existing
  `BankChain::transposes` (one per rendered block, none per skipped block). When #2129 lands a
  per-slot skip, its `bypass_skips` must be subtracted in `Runtime::bank_slot_runs`
  (`crates/graph/src/runtime.rs`); its own tests will see a disagreement with gate 2 otherwise.
  The reader is **not** gated on `test-support` (unlike the other `test_only_` readers): the bench
  reads it, and building the bench with `test-support` would put that feature's render-path
  switches into the measured binary.
- **Fixtures.** `console-c1-{16,32,64}.json` are the output of the console app's own builder
  (`misofm/console` at `43e1f6f`, `app/src/studio/console-session.ts`, `buildConsoleSession`, SDK
  `@misofm/engine` 0.6.0) for 16, 32 and 64 stereo stems `ch00..`, track `i` assigned to group
  `i mod 4`: every console slot bypassed, the limiter bypassed, sends muted (the load state).
  `irregular-graph.json` is built with the same SDK: 24 tracks (`i mod 3 == 0` mono), C1's console
  active on every strip, `i mod 4` inserts from (EQ, de-esser, transient shaper), `ch05`'s one
  insert a compressor sidechained from `ch00`'s `post_fader`, groups `a, b, c` -> `bus.inst`
  (a, b; with a delay insert) -> `master` (with an active limiter), `ch22` and `ch23` to the master
  directly. The generator scripts are not committed (they import the console repository by path);
  the fixtures are copies.
- **Workload.** `Workload::C1Console { tracks, state }` (`Load`, `Typical`, `AllActive`, `Sparse`,
  `Toggle`) and `Workload::IrregularGraph`, listed in `BANK_REPORT_WORKLOADS` and in no standing
  list, so the 78-record standing benchmark and its validator are unchanged. `Typical` draws each
  strip's slots with xorshift64* from seed 2132 (EQ 75 %, comp 50 %, gate 10 %, drive 10 %), limiter
  bypassed; `AllActive`, `Sparse` (odd tracks exact zeros) and `Toggle` un-bypass every slot and the
  limiter; `Toggle` attaches live controls and writes `ch00`'s gate bypass every 64 blocks
  (`SessionRuntime::before_block`). Each row's ideal per pack group is computed at bind from its own
  model: `sum over slots of ceil(A / W)`, `A` the group's lanes whose effect is not bypassed and
  whose strip is not silent (a track by its input; a submix when no unmuted route from a non-silent
  strip reaches it, so both auxes are silent: their sends are muted).
- **Bench.** `bench console --bank-report` (one invocation: one untimed warmup round, two measured
  rounds, 128 untimed blocks then 1000 timed blocks per row and round) and
  `--bank-report-preflight` (builds every row, renders 128 untimed blocks, prints the static facts,
  times nothing).

### Decisions taken without a ruling (for review)

1. **Gate 1's one-level pair is built from a compiled plan, not a session.** `plan_bank_groups`
   pools every compatible candidate of a level together and emits at most one partial group per
   leader, so no session can produce two partial banks of one key at one level: the count is
   structurally zero today on every row. The test compiles the standing 64-track console (pairs
   0, lane use 1000) and splits one of its groups into two partial groups (pairs 1; at two levels
   0; another class 0; another fold target 0).
2. **Gate 2 is in `crates/graph-compiler/tests/silent_bank_skip.rs`, not a `crates/graph` unit
   test.** The reader lives in `graph`, but a skipping effect bank needs a real effect stage that
   reports rest; that harness compiles one. The test checks every block: runs per group equal
   effect slots x (units - chains skipped), and per block over the plan the effect slots x units
   equal the compile's bound effect banks.
3. **BSR counts effect slots only.** The design's predicted BSR (21 at 16 tracks, W=8) is the
   console effect slots; builtin stages fused into the same chains are not counted.
4. **Sparse and Toggle are on the all-active base**; Typical keeps the limiter bypassed.

### Red evidence (each mutant run against the gate, then reverted)

- Gate 1, `the_bank_report_counts_mergeable_pairs`: drop the pool-class clause -> red ("another
  pool class is another key", left 1); compare `members.len()` (padded lanes) instead of active
  lanes -> red (pairs 0 at one level); count every lane of a bound slot as active -> red (lane use
  1000, expected 438 at W=8).
- Gate 2, `bank_slot_runs_follow_the_skips`: count a skipped chain's block as a run (transposes +
  skips) -> red at block 54, units 20..=22 (9 against 6); count builtin stages as effect slots ->
  red (18 slot-units against 9 bound banks).
