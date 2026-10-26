# A descriptive console benchmark row for the reverb

Successor of #2114 (the engine reverb), filed 2026-10-26, so that benchmark machinery does not keep
the reverb's product slice open (AGENTS.md, "Keep feature issues small enough to ship"). GitHub
issue #2147; this file is its spec (written by the finish map of 2026-10-26 from the GitHub body,
amended where the tree moved).

## Class

A: tooling only. No engine render change.

## Problem

The reverb has no benchmark row. The delay has no row of its own either: it runs only as the `fx-a`
insert inside `sixty_four_track_console_sends`. The console's Aux B return is a reverb (console C1),
so the browser console's real cost includes the reverb, and no record shows it. The C1-16 fixture
(`fixtures/session/v1/console-c1-16.json`, #2132) has `aux.b` with no inserts and every send muted
(the load state), so it does not measure the reverb.

## Smallest closable slice

- One workload variant: `Workload::C1Console` extended with a state or a sibling kind (one row,
  name `c1_console_16_reverb`) built from the C1-16 fixture in code: `miso.reverb` at mix 1 inserted
  on the `aux.b` return, every track's post-fader send to `aux.b` unmuted at -6 dB, the console
  slots in the `AllActive` state and in the `Load` state (two rows if both are kept; one row,
  `AllActive`, is the smallest slice). Add no new fixture: build the variant from the existing C1
  fixture through the session model, as `C1State` already builds `Typical` and `Toggle`.
- The row joins `BANK_REPORT_WORKLOADS` (`tools/console-workload/src/lib.rs`), where the C1 rows
  live, so `tools/bench console --bank-report` emits it natively (W=8) and the **80-record standing
  list and its validator are untouched** (`scripts/console-benchmark-validator.jq:21`, `:44`: the
  list is 80 since #2129 S1b, not 78 as the GitHub body says). V8 has no C1 document (#2143), so the
  V8 column reads "not measured" unless a document is added to the V8 runner, which is out of this
  slice.
- Descriptive only: one invocation, one warmup and two measured rounds, with no tuning and no retry;
  the twin of main measured in the same invocation is not possible for a row main does not have, so
  the record is the row alone plus the C1-16 `AllActive` row beside it (the same session without
  the reverb and with sends muted) as the reference.
- Preflight the arguments, schema and output persistence first (`--bank-report-preflight`).
- No floor in `tools/bench/src/floor.rs`: #2114's spec states none.

## Paths

`tools/console-workload/src/lib.rs` (the variant, its feed-test row), `tools/bench/src/console.rs`
only if the report needs a column, this spec. Not touched: `tools/bench/src/floor.rs`, the
`scripts/console-benchmark-*` files, the standing list.

## Gates

1. `tools/console-workload`'s feed test (`every_row_is_fed_through_a_source_set_and_gathered_in_place`)
   covers the new row (the existing claim, one new row); the row's plan binds one per-node reverb
   (`NEVER_BANKED_EFFECTS`) on `aux.b` and sixteen unmuted sends into it. Red on a variant whose
   sends stay muted (the reverb's input is silent and its cost is the skip's). Test value: red when
   the row stops measuring a fed reverb.
2. `bench console --bank-report-preflight` passes on the row; the row's output digest equals itself
   across both measured rounds (determinism).

## Evidence (recorded, not gated)

The row's p50 us per block, native W=8, under `artifacts/steps/bank-design-2147/`, beside the C1-16
`AllActive` reference from the same invocation.

## Dependencies

None. Parallel to every engine slice (disjoint files). Lands any time after #2154.
