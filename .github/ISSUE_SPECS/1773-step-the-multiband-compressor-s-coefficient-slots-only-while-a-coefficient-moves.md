# Step the multiband compressor's coefficient slots only while a coefficient moves

Weekly optimisation pass. Follows #1069 attempt 2 (2026-10-23).

## Problem

#1069 made the multiband compressor's ratio, attack and release automation partition-exact: per-frame `1/R - 1`, and four coefficient ramps per side per track (slots 10-13) walked with `ramp_toward`. Every ramped frame now steps all 14 slots, also when only a threshold or makeup ramp moves. On the frozen bench1069 workload (128-frame blocks, a retarget every block) the ramped rows cost +4-5 % against main at W8 and W4; the settled rows do not move. The attack/release parameter ramps (slots 2, 3, 7, 8) are also pure bookkeeping now and are still stepped every ramped frame.

## Smallest closable slice

- Add a second const generic, `COEF: bool`, to `run_segment`. `plan_segment` sets it uniformly per segment from the slots that feed the coefficients (1, 6, 10-13). `process_block` dispatches it in its `segment!` macro. With `COEF = false` the step loop is a constant `0..RAMP_COUNT` and `frame_coefficients` is not emitted; `debug_assert` that the coefficients are at rest. The bits are unchanged by the same identity argument as the flat path.
- If it measures as a gain, also stop stepping slots 2, 3, 7, 8 where nothing reads them.
- Delete `conformance::Known::RampCutsMoveBits`, which has no user since #1069.

## Authorized paths

- `crates/multiband-compressor/`, `crates/conformance/src/randomized.rs` (the dead variant only)
- this spec

## Objective gates

1. Class A: no rendered bit moves (the crate's randomized differential, E4, ramp_endpoint, the cross-target corpus).
2. The frozen bench1069 workload, one descriptive run: report every row against main, and check that the W8 None row does not move (the reverted 4ca41ebd7 attempt showed a steady +4 % there that the code did not explain).
3. fmt, clippy, cargo doc -D warnings, the AArch64 test check, `scripts/check-web-audioworklet.sh`.

## Test value

No new test: the change is exact, and the existing differential and partition tests defend the bits.

## Dependencies

#1069 merged.

## Evidence and decisions (attempt 1, 2026-10-23)

- D1. `run_segment` takes `COEF`; `SegmentPlan::coefficients` comes from the same scan as `ramping`. At `RAMPING && !COEF` the step loop skips every coefficient-path slot (a compile-time `continue`, so the ratio slots 1 and 6 are skipped too, not only 10-13) and `frame_coefficients` runs once per segment. `FORCE_RAMPING` forces `COEF`, so the split test also compares the new arm with full stepping. Debug builds assert `Instance::coefficients_at_rest`.
- D2. The coefficient path includes the attack and release parameter ramps (slots 2, 3, 7, 8): nothing in the kernel reads them, so a `!COEF` segment steps only threshold and makeup (slots 0, 4, 5, 9). This is exact only because the predicate covers them: a time parameter ramp can move while its coefficient ramp rests (`ramp_endpoint` reaches that case; a mutant that leaves them out of the predicate only is red there in release).
- D3. `conformance::Known::RampCutsMoveBits` and its `Shape::automate` narrowing are deleted. No draw is consumed differently for any other effect.
- Bench (frozen bench1069 workload, scratch, one run per binary): ThresholdAll -4.8 % at W8 and W4 (D1); D2 adds about -3 % at W4 and nothing at W8, on a noisy run. W8 None is +3.5 % against main, although the flat loop's source is unchanged: LLVM schedules that loop differently (llvm-mca znver3: 220 against 205 cycles per frame). Aligning loops to 64 bytes does not remove the difference, so it is not code placement. Not tuned (brief).
- Correction (root ruling, one bounded try, 890518e01): the ramped arms out of line (`run_ramped_segment`, `#[inline(never)]`). Merge gate fixed before timing: both None rows within 1 % of main in both rounds, no row above +2 %. One run against main: W8 None +1.06 % / +1.71 % (fail); W4 None -0.16 % / +0.27 %; ThresholdAll -7.2 % (W8), -8.3 % (W4); RatioAll +1.3 % (W8), +0.2 % (W4); RatioLane0 +1.0 % (W8), +0.2 % (W4). **The gate fails on W8 None; not merged, not iterated.** Records: scratchpad bench1069/main-1773-c.txt, fix-1773-c.txt.
