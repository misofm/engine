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
