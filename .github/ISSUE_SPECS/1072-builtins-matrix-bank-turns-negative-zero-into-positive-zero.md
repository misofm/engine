# Builtins matrix bank turns -0.0 into +0.0

Found by #1051's randomized differential (`crates/builtins/tests/randomized.rs`, "#1051 defect 4", seed 7).

## Problem

In the builtins 2x2 matrix/pan bank, a settled lane whose output is -0.0 renders +0.0, while the per-node path keeps -0.0. That breaks bit identity between banked and unbanked rendering (class A). The per-PR randomized test currently compares the two zeros as one value until this is fixed.

## Smallest closable slice

Make the banked matrix preserve the sign of zero exactly as the per-node path does (likely an added +0.0 or a select that normalizes). No target-specific code. Un-ignore the reproducer and remove the per-PR test's signed-zero allowance.

## Gates

1. The reproducer passes at seed 7 and across the differential's seeds at Simd4 and Simd8 and against the scalar oracle, with signed zeros compared bit-exactly.
2. No regression on the native console `--step` rows beyond the owner's +2% allowance for a genuinely better implementation.
3. Console digests unchanged, or each moved digest explained.

## Evidence and decisions (attempt 1)

- Cause: `matrix2x2_ramp_block` (crates/lane/src/kernels/builtins.rs) ran the ramp arithmetic on every lane of the bank while any lane ramped. A settled identity lane then rendered `1 * -0.0 + 0 * x`, which is `+0.0` for `x >= +0.0`; its own `MatrixBuiltins` is on `matrix2x2_block`, whose identity select passes `-0.0` through. The per-node path is correct and is not changed: at width 1 the ramp kernel never runs a frame after the lane's own ramp ends.
- Fix: the ramp kernel applies the settled kernel's identity select on the frames a lane is settled (`remaining <= 0` before the frame's decrement) and its target is the identity. The ramp's own last frame keeps the arithmetic, as the lane alone does. The identity mask comes from the targets once per call; with no identity target over all lanes the loop runs without the select. One shape on every target.
- Tests: the differential's #1072 allowance is removed, so it compares signed zeros by bits. The ignored reproducer was then the same test and is deleted. Reproducer red on the pre-fix code (87b818f17): `W8 seed 15 block 12 ... the matrix stage, lane 1 left frame 77: the banks rendered 0x00000000 where the scalar strip rendered 0x80000000`. After the fix: 24 seeds (debug) and 4800 seeds (`MISO_ENGINE_RANDOMIZED_SCALE=200`, release) pass at W4 and W8.
- New test `a_banked_identity_lane_keeps_signed_zero_while_another_lane_ramps` (crates/builtins/tests/matrix.rs) pins the boundary frame: a mutant that starts the pass-through on the ramp's last frame (`done` for `settled`) passes 4800 differential seeds but fails this test.
- Not in the wasm `KERNEL_ROSTER`.
- Native console `--step` (one invocation each, controlled, one warmup and two rounds): candidate `artifacts/steps/opt8-1072` at dea8057cb against base `artifacts/steps/opt8-1072-base` at 87b818f17 (the newest committed step, `opt6-1107-fix`, predates #1784's class-B compressor change, so it is not a digest base). All 104 output digests are unchanged. Of 51 p50 rows, none is slower than +2 %: the worst is `sixty_four_track_gain_pan_ring` at +1.68 %, the median is +0.04 %. The console workload runs no matrix ramp onto an identity lane, so no row runs the new select.
