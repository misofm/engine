# 045 Launch parametric EQ recurrence derivation and runtime proof

## Outcome

Derive and prove one numerically stable `f32`-lane parametric-EQ runtime topology before any further
production EQ implementation.

## Context

Issues **Numerically conditioned launch parametric EQ realization** and **Conditioned time-domain
launch parametric EQ recurrence** both stopped after their attempt budgets. Issue 042 proved a
conditioned transfer and broad integration surface but its direct-history recurrence recovered on
the first legal impulse. Issue 044 corrected its finite-window oracle and then showed that scaled
direct, transposed and subnormal-flush variants all fail the frozen time-domain selection gate.

This is a research/proof issue, not a product implementation issue. It has one Terra investigation
and at most one bounded Sol correction. A second failure stops. Production, graph, audit, benchmark
and timing changes are forbidden; `timed_benchmark_invocations=0`.

## Scope

- Re-derive candidate transfer-to-recurrence mappings independently and first prove f64 recurrence
  equivalence to the f64 reference impulse before any retained-f32 comparison.
- Compare a bounded set of genuinely different stable runtime structures: orthogonal/coupled or
  lattice state, error-compensated/double-single f32 state, and one additional cited second-order
  structure chosen by Sol.
- Preserve the public four-section EQ surface, four launch rates, domains and response/time-domain
  tolerances from Issues 042/044.
- Select only a fixed-size scalar/W4/W8-feasible candidate that passes the complete frozen analytic,
  48 impulse and 48 million-sample reference matrix with zero recovery.
- Freeze a machine-checkable derivation, equations, words/state, operation/FMA graph and transcript.

## Required public interfaces/contracts

No production interface changes are allowed in this issue. The proof must target the existing
`miso.parametric-eq` surface and `f32` audio/SIMD lanes. Multiword state may use a fixed number of
`f32` words per value, but f64 production state, dynamic storage and hidden scalar-only behavior are
forbidden.

## Deliverables

- cited derivation note and independent f64 algebra/impulse equivalence tests;
- complete retained-f32 candidate comparison over all frozen gates;
- exact hashes, first failures and storage/SIMD feasibility table; and
- Sol PASS selection amendment or final STOP with no production edits.

## Explicit non-goals

Production EQ code; graph/fixture/audit/target reruns; tolerance/domain changes; extended rates;
new filter kinds; dynamic EQ; performance tuning; listening; or benchmarks.

## Dependencies by exact issue title

- DSP research corpus and conformance harness
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral
- Numerically conditioned launch parametric EQ realization
- Conditioned time-domain launch parametric EQ recurrence

Stopped dependencies contribute evidence only, not PASS.

## Sol implementation brief

**READY FOR RESEARCH BRIEFING, not implementation.** The tracked brief is
`.github/ISSUE_SPECS/BRIEFS/045-launch-parametric-eq-recurrence-derivation-and-runtime-proof.md`.

## Hazards/decisions

Do not compare a finite impulse with an infinite analytic response, assume algebraic equivalence,
reuse a failed recurrence under a renamed state, flush instability, or select from one repaired row.
Every candidate must pass f64 equivalence before f32 retention is evaluated.

## Acceptance gates with objective measurements

1. Symbolic/numeric expansion and f64 impulse tests prove each candidate realizes the intended
   transfer to <=1e-12 over all frozen rows before f32 testing.
2. Retained-f32 candidates pass the complete analytic/search gates plus all 48 one-second and 48
   million-sample cases with zero recovery and unchanged 0.005/0.05 dB and 0.1% tolerances.
3. State/output is finite normal-or-positive-zero; any underflow policy is explicit and cannot hide
   nonfinite or unstable state. Scalar/W4/W8 shapes and operation/FMA sites are fixed and bounded.
4. Results record exact candidate words/state, maxima, first failures, deterministic hashes and
   storage costs. Exactly one candidate may be selected; otherwise stop.
5. Focused reference tests, warning-denied Clippy, format and diff checks pass. No production diff
   or benchmark/timing artifact exists.

## Target matrix

Reference tests run natively. SIMD feasibility is a static fixed-layout/operation-graph proof only;
production cross-target work belongs to a later implementation issue after selection.

## Required evidence

Issue-042/044 hashes and failures; citations/derivations; f64-equivalence and retained-f32 matrices;
storage/operation tables; deterministic hashes; Terra and Sol verdicts; production no-diff proof;
and `timed_benchmark_invocations=0`.
