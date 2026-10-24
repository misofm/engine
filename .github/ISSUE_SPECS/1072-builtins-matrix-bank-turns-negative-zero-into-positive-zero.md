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
- Tests: the differential's #1072 allowance is removed, so it compares signed zeros by bits. The ignored reproducer was then the same test and is deleted as a duplicate, so the red reproducer is the strict per-PR differential `the_banks_render_their_scalar_sections_under_random_retargets`. Red on the pre-fix code (87b818f17): `W8 seed 15 block 12 ... the matrix stage, lane 1 left frame 77: the banks rendered 0x00000000 where the scalar strip rendered 0x80000000`. After the fix: 24 seeds (debug) and 4800 seeds (`MISO_ENGINE_RANDOMIZED_SCALE=200`, release) pass at W4 and W8.
- New test `a_banked_identity_lane_keeps_signed_zero_while_another_lane_ramps` (crates/builtins/tests/matrix.rs) pins the boundary frame: a mutant that starts the pass-through on the ramp's last frame (`done` for `settled`) passes 4800 differential seeds but fails this test. The test is also red on the base code (matrix.rs:346).
- Not in the wasm `KERNEL_ROSTER`.
- Native console `--step` (one invocation each, controlled, one warmup and two rounds): candidate `artifacts/steps/opt8-1072` at dea8057cb against base `artifacts/steps/opt8-1072-base` at 87b818f17 (the newest committed step, `opt6-1107-fix`, predates #1784's class-B compressor change, so it is not a digest base). All 104 output digests are unchanged.
  - Timing, paired round by round (round 1 of the candidate against round 1 of the base, round 2 against round 2): the 46 `console_session` rows' `p50_us_per_block` have a median of -0.09 %. The worst per round: round 1 `sixty_four_track_gain_pan_only` +1.59 %; round 2 `sixty_four_track_dispatch_only` +3.28 % (its round 1 is -1.19 %). The fader ride's `paired_ramp_delta_median_ns` is -0.74 % (round 1) and +2.68 % (round 2).
  - An earlier aggregation (51 keys: each record's `p50_ns_per_block` and `*_p50_ns` fields, the minimum of the two rounds per side) gave a worst of +1.68 % (`sixty_four_track_gain_pan_ring`) and a median of +0.04 %.
  - No console row reaches the `PASS = true` arm (verifier's finding): every ramping row is 64 tracks in 8 full banks with pan-law targets, so `PASS = false` everywhere, and that arm's release codegen is the old body. So these deltas are noise.
  - On x86-64-v3 the `PASS = true` arm compiles to one masked store per frame on the left plane (the #944 fold; `BuiltinMatrixBank::process` has 6 pointer-addressed `vmaskmovps` against 4 at base). It is the same store the settled identity path already pays on partial banks. Its cost is not measured.
