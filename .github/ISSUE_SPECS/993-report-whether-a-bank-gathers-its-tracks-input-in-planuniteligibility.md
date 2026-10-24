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

## Decision and evidence record

- Scope (root, 2026-10-24): field `gathers_track_input`, set in `graph`'s `unit_eligibility` from `UnitBanking::gathers_track_input` (already `pub(crate)`), so `crates/graph/src/runtime.rs` is not edited. No `armable()` helper.
- Gate 1: `chain_shape`'s written-out mono rows carry the field (`true` on every cohort chain, `false` on the input ops and `main-out`); the #970 split-strip reproducer asserts one gathering bank per cohort, the earlier of its two chains.
- Gate 2 (PR evidence): the builder copying `banked()` (old every-lane rule) turns `only_the_input_gathering_chain_of_a_split_strip_is_armed` red at the new count.
- Gate 3 (PR evidence): `UnitBanking::of` `(true, true) => Bank` is red in both suites. `(false, true) => BankGatheringTrackInput` stays green: it changes only a plain op whose node is a `PostInputBuiltins` stage, and no unit the tests build is one, so no assertion can discriminate it. Recorded as undiscriminated; no test added.
