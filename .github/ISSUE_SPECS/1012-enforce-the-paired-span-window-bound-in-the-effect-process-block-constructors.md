# Enforce the paired-span window bound in the effect process-block constructors

## Product outcome

#1004 keeps the mono collapse when a both-channel `Parameter` edit arrives as twin Left and Right spans. Its exactness rests on one precondition: a lane's staging window is never larger than the effect's `automation_capacity`, so a twin pair can never straddle the cut-off. Today both constructors size the window to the capacity, but nothing enforces it, and the `debug_assert!` meant to guard it can never fire. A future caller that sized the window one larger would render the left channel's state on the right and lose an acknowledged right-channel write. The #1004 Sol verification (attempt 1, PASS) raised this and two smaller documentation findings, quoted below from `.github/ISSUE_SPECS/1004-*.md`.

## Findings to close

1. **Low: the A3 debug assertion is vacuous.**
   * `debug_assert!(staged <= staging.len())` can never fire, because `staged` is incremented
     only while it is below `staging.len()`.
   * So the precondition the pair rests on, `staging.len() <= automation_capacity`, is enforced
     nowhere. Neither `EffectBankProcessBlock::new` nor `EffectProcessBlock::new` checks a lane's
     span count against capacity.
   * Today it holds, because both constructors size the window to the capacity.
   * **Failure scenario.** A future caller sizes its window to capacity + 1. A drain stages
     capacity + 1 spans, and the last twin straddles the cut-off: the effect applies `Left p v` at
     index `capacity − 1` and refuses `Right p v` at index `capacity`. `LIVE` is kept, the
     collapse renders the left channel's state for the right channel, and the right-channel write
     is lost when the collapse disengages.
   * **Fix.** Assert `window == automation_capacity` where both are known, in the two
     constructors.
2. **Low: stale prose in `host-core/tests/symmetry_witness.rs`.**
   `two_per_lane_writes_that_agree_still_decline_the_lane` says:
   * that a `Left` then a `Right` write to the same value "is how the ABI addresses a `PerLane`
     parameter";
   * that the witness "cannot see two writes cancel".

   Both statements are now false for `Parameter` spans. The test still passes only because it
   drives EQ targets.
   * **Failure scenario.** A maintainer reads the test as the contract and "restores" the decline
     for twin spans.
   * **Fix.** Correct it in a follow-up, since the file is outside this slice's paths.
4. **Info: an undocumented change on an invariant-failure path.** A lone one-channel `Parameter`
   record that reaches a target owner now keeps `LIVE`, because the window is empty and so pairs.
   Base cleared `LIVE` there.
   * This is sound: the record is refused and never applied, and `stage` returns `target_error`,
     so the block's render fails.
   * No fix is needed. It is noted so that nobody reads it as the pairing rule firing.

## Objective gates

1. `EffectBankProcessBlock::new` and `EffectProcessBlock::new` refuse (with a typed error at preparation, never on the render thread) a window whose span count differs from `automation_capacity`; the vacuous `debug_assert!` is removed or replaced by an assertion that can fire.
2. A test builds a window of capacity + 1 and shows it refused; a mutation that removes the check turns the test red.
3. `two_per_lane_writes_that_agree_still_decline_the_lane` in `crates/host-core/tests/symmetry_witness.rs` states the contract as it is after #1004: twin `Parameter` spans keep the collapse; one-channel EQ targets still decline it. A test for twin spans keeping `LIVE` sits beside it.
4. The lone one-channel record that now keeps `LIVE` on the invariant-failure path (finding 4) is documented where it happens.
5. All #1004 gates, every standing digest and the `console_mixing_automation` preflight are unchanged; clippy, fmt and the policy scripts pass. Render stays allocation-free.
