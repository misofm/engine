

### AArch64 fix (PR #2430's NEON Simd4 shard 1/3)
**The failure.** At W4, `a_refused_carry_restore_reaches_the_defect_watch` panicked on "per node to bank": it got `(1, 1)` and expected `(2, 2)`. The watch and `carry_refusal_count` agreed, so the mirror was right and the expectation was the x86 W8 value.
- An insert form change moves only the per-node side's inserts.
- That side has one insert fewer than the build width's padding threshold. The threshold is two at four lanes and three at eight.
- So at W4 the carry makes one effect move, and the seam's second refusal has nothing to refuse.

**The fix.** Each arm now expects the seam's two refusals or the carry's effect moves, whichever is fewer:
- the console arms (live and plain): one EQ lane per strip, `a.model.tracks.len()`, which is nine, so 2 at either width;
- the two form arms: `insert_pad_threshold(backend) - 1`, which is 1 at W4 and 2 at W8.

The arm still asserts the watch equal to `carry_refusal_count`, and both equal to that exact count.

**The other tests in the diff do not depend on the width:**
- the control-plane carry test, the capi row and the host-web row all carry the nine-track session's EQ console lanes, so they refuse 2 at either width;
- the control-plane lane-swap test counts strands, and each strand's helper asserts the plan's total rises by exactly one.

**W4 could not be run locally.** This host is x86-64-v3, where `Backend::current()` is `Simd8`.
- Forcing `Backend::Simd4` through the test's width seam does not bind banks. The plain console preparation fails in `support/successor.rs:82`, and `form_pair`'s own check finds B's inserts per-node ("B's inserts are one bank": `[true, true]` against `[false, false]`). A W4 bank binds only in a NEON or `simd128` build, as #1017 says.
- So the W4 evidence is the CI shard's own reading, `(1, 1)`, which is what the expectation now gives at W4.
- The form mutants still turn W4 red, since one refusal is enough:
  - T3 and T5 store nothing, so the watch reads 0;
  - T4 does not tally, so the watch reads 0;
  - T6 leaves out the per-node term, so the watch reads 0.
- At W4 the form arms cannot tell "store one refusal" apart from the total. The console arms still can, at 2.

**Verified at W8 after the fix:**
- `successor_swap` passes in debug (127 passed) and the watch test passes in release.
- clippy `-D warnings` on host-core (all targets, test-support) and `cargo fmt --check` pass.
- T1 to T6 are still red in debug and release: all twelve runs red.
