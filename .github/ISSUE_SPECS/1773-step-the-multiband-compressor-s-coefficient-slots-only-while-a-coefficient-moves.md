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
