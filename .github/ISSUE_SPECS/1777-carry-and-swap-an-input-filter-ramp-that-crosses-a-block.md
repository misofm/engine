# Carry and swap an input filter ramp that crosses a block

Found by the #1765 verifier (2026-10-23).

## Problem

The input section's filter ramp lasts 64 updates. With the 128-frame quantum that every carry and swap test uses, a filter ramp is never in flight at a block boundary, so no test reaches a plan-swap carry or a render-time lane swap with a filter ramp in flight. A mutant that drops the `filter_remaining` import (`crates/builtins/src/lib.rs`, the input bank's `import_lane`) stays green in the whole builtins and builtins-compiler suites. Hosts may run smaller quanta (the C ABI accepts host-chosen render quanta; the browser's is 128), so the case is reachable in product.

## Smallest closable slice

- Fold one case into an existing carry test and into #1765's builtin lane-swap scenario: a quantum below 64 (for example 32), a filter retarget whose ramp spans the carry or swap block, and the twin bit check after it.
- Show the `filter_remaining` mutant red.

## Authorized paths

- the tests of `crates/builtins` and `crates/builtins-compiler`
- this spec

## Objective gates

1. The folded case passes; the mutant turns it red.
2. fmt, clippy, the AArch64 test check.

## Test value

The fold turns red when a carry or swap drops an input filter ramp in flight, which no test reaches today.

## Dependencies

#1765 merged.

## Evidence (attempt 1, 2026-10-23)

- Carry: `a_carried_lane_arms_its_joint_flush_on_the_frame_the_exported_lane_would` (`crates/builtins/tests/exact_rest.rs`) gains a 32-frame pass that retargets the high-pass (10 Hz to 10.5 Hz) before block 7, so the ramp has 32 samples left at the block-8 carry.
- Swap: `test_only_builtin_lane_swap` (`crates/builtins-compiler/src/lib.rs`) runs at quanta 128 and 32. Before each swap, strips `t01`, `t05`, `t09` (every fourth from `t01`) retarget both filter sections to a pair no other write holds; the strips they trade with have no new ramp.
- Mutant (drop `self.filter_remaining[channel][section][lane] = words.remaining;` in the input stage's `import_lane`): the carry test is red at block 8; both swap tests are red at `quantum 32, strip t01, block 5` (Simd8, and Simd4 with Simd8 removed). Restored: green. Production code is unchanged.
