# Bind only the changed cohorts when the whole-mono-cohort guard compares plans

## Product outcome

#971's guard binds both the trial plan and the re-plan to compare the banks they actually produce, so both plans' banks are alive at once. The #971 attempt-2 verification measured the cost at #962's scale (debug `compile_with_builtins`, 65,537 mono tracks, one strand, under the timing lock):

| effect per track | case | base | #971 | change |
|---|---|---:|---:|---:|
| `simd1: [comp]` | refused | 48.6 s | 50.3 s | +3.4 % |
| `simd1: [comp]` | one track in 9 stereo, kept | 48.7 s | 50.8 s | +4.3 % |
| `simd2: [limiter]` | refused | 48.5 s | 53.4 s | +10.2 % |

With the limiter, peak RSS rises from 3.21 GB to 3.99 GB (+24 %). #962's scale gates never take this path because their session has no effects. The verifier found a cheaper exact count: groups the move does not change bind to the same banks in both plans, so only the changed groups need binding for the comparison.

## Smallest closable slice

Bind the re-plan's changed groups only, reuse the trial plan's banks for unchanged groups, and compare the bound counts over the changed groups. The accepted plan must be exactly the plan #971 accepts today, with the same banks.

**Out of scope:** evaluating moves per independent component instead of all at once (#971 attempt-1 low finding 3), and changing what the guard decides.

## Objective gates

1. **Same decision.** On the #971 randomized probe (the verifier's 4,220 renders: 400 random mixed mono and stereo sessions plus the 22 named shapes) and the #971 gate sessions, the accepted plan, its pools and its bank count equal #971's at 8 and 4 lanes; rendered digests are unchanged.
2. **Scale.** The three table rows above, re-measured under the timing lock against the batch head: compile time within 1 % of base (pre-#971), and limiter peak RSS within 5 % of base.
3. **Mutations** (each alone, red): reusing a trial bank for a changed group; skipping a changed group in the count.
4. Every standing console digest unchanged; clippy and fmt pass.
