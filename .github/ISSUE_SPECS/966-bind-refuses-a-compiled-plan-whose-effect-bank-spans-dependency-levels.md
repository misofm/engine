# Bind refuses a compiled plan whose effect bank spans dependency levels

## Product outcome

The compiler accepts a session that bind then refuses with `graph.scheduler.layout`, so a host fails prepare on a session the compiler said was valid. Found by the #962 verification's randomized probe: 44 of 4,800 binds (22 seed-width pairs), identical on the pre-#962 base `0109d841`, so the defect is pre-existing.

`has_valid_structural_layout` (`crates/graph/src/lib.rs:1233`) rejects an effect bank whose members sit at different dependency levels. Minimal reproducer (seed 412, `Simd8`): nine tracks, seven carrying `soft-clip` at dynamic slot 0 and one carrying it at slot 1 behind an EQ; all eight are banked together, at levels 5,5,5,6,5,5,5,5. The same session binds at `Simd4`.

## Smallest closable slice

1. Commit the reproducer as a failing test in `crates/graph-compiler` (compile then bind at `Simd8`).
2. Decide where the invariant belongs and make compile and bind agree: either the bank planner never groups members that will land at different dependency levels (they form separate cohorts, or the shorter chain is padded with identity slots so the levels align), or the layout check accepts such a bank if the schedule is still valid. The choice must keep class A for every plan that binds today (every console digest and unit census unchanged) and be argued in the evidence.
3. Add the #962 verification's randomized compile-then-bind probe as a committed test over a fixed seed range, asserting every compiled plan binds.

## Objective gates

- The reproducer compiles and binds at `Simd4` and `Simd8`, and renders bit-identically to the same tracks banked the way today's `Simd4` banks them, or the choice's documented equivalent.
- The randomized probe binds every compiled plan.
- Every console workload digest and unit census unchanged; fmt, clippy `-D warnings`, graph policy and determinism scripts.
