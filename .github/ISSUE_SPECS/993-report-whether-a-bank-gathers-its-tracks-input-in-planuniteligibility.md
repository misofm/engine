# Report whether a bank gathers its tracks' input in PlanUnitEligibility

## Product outcome

#970 made the mono collapse arm only on chains that gather their tracks' input (`UnitBanking::BankGathersTrackInput`). `PlanUnitEligibility` still has no bit for that fact, so eligibility reports over-state which chains of a split strip can collapse. Found by the #970 verification (low finding 2).

## Smallest closable slice

Add `gathers_track_input: bool` to `PlanUnitEligibility`, set from the same bind-time decision #970 made, and assert it in the #970 reproducers (`crates/host-core/tests/collapse_arming.rs`): true for the chain that gathers the input, false for every later chain.

## Objective gates

- The new field matches `UnitBanking` for every unit in the #970 reproducers and every standing console row.
- A mutation that sets it from the old "every lane mono" rule turns a reproducer's assertion red.
- Also cover the one surviving mutation in `UnitBanking::of` the #970 verification found (it changes no audio but should change this field).
- fmt, clippy `-D warnings`.
