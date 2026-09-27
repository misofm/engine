# Keep the unmoved plan when the whole-mono-cohort re-plan fails to bind

## Product outcome

#971 keeps the mono pool a whole number of cohorts by re-planning the stranded remainder and binding both plans to compare the banks they actually produce. A bank-bind error on the speculative re-plan fails the whole compile (`crates/graph-compiler/src/banks.rs`, `bind(&replan)?`). The #971 attempt-2 verification showed this is real: with a test factory whose bank bind always errors, a session of `ch00` mono and `ch01..` stereo carrying only `dynamic: [comp]` compiles with the move disabled (the unmoved plan has no full group and never calls bind), and is refused at head, at 8 and 4 lanes. No shipped factory can trigger it today (all eight launch factories' bank paths were read), but an optimisation's trial plan must never cost the user the compile. This matches #95's stance that a cohort a factory cannot bank must not fail the compile.

## Smallest closable slice

On a `bind(&replan)` error, keep the unmoved plan and its already-bound banks, and continue. No diagnostic is needed for correctness; if the compiler already records non-fatal planning notes, add one.

## Objective gates

1. The erroring-factory session above (a test-only factory) compiles at `Simd8` and `Simd4`, keeps `ch00` mono, and renders the bits of the move-disabled plan.
2. Mutation: restoring `?` on the re-plan bind turns gate 1 red.
3. Every #971 gate, the #971 discriminating pool test and every standing console digest are unchanged; `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` pass.
