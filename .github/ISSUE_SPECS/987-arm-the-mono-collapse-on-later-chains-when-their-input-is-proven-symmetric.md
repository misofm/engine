# Arm the mono collapse on later chains when their input is proven symmetric

## Product outcome

#970 fixes a wrong-audio bug by arming the mono collapse only on the chain that gathers a track's input; every later chain of a split strip renders dual. The #970 verification measured what that costs on realistic mono sessions: 64 mono tracks where half have no effects run +42% at 8 lanes and +48% at 4; one track in eight lacking the limiter runs +25% and +24%. Recover it exactly: arm a later chain when its input planes are proven identical (the upstream chain or per-node op collapsed and nothing between wrote L and R differently), rather than trusting the track's source witness.

## Smallest closable slice

Carry an exact per-chain "input is symmetric" fact from the producing chain's own collapse decision (and each per-node op's symmetry) to the consuming chain at bind or per block, and arm the later chain only on that fact. Research and evidence: `docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md` and `VERIFY-970-971.md`.

## Objective gates

- The #970 reproducers (asymmetric EQ plus a `PostSimd1` meter; asymmetric per-node delay; sends at `post_input_builtins` or `post_simd1`; a live left-only trim write on misaligned cohorts) stay bit-correct.
- The two cost shapes above return to within noise of today's armed cost, with identical bits to the dual render.
- Every console digest unchanged; red mutations for each proof step.

## Dependencies

#970.
