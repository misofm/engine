# Sol research brief — issue 045 launch parametric EQ recurrence derivation and runtime proof

## Decision and attempt budget

Research only. One Terra investigation and one bounded Sol correction are available. Issues 042
and 044 remain stopped; do not modify production EQ/core/graph code and do not benchmark.

## Ordered proof

1. Freeze three genuinely distinct fixed-f32-state candidate structures with primary citations and
   independently derived transfer mappings. Include orthogonal/coupled or lattice state, a fixed
   double-single/error-compensated f32 state, and one Sol-approved second-order alternative.
2. For every frozen row, expand each mapping back to the intended transfer and compare a f64
   recurrence impulse to the independent f64 oracle at <=1e-12. Reject mapping defects here.
3. Only algebraically valid candidates advance to retained-f32 analytic/search, 48 one-second DFT
   and 48 million-sample sequences. Use matching finite windows for finite DFT comparisons.
4. Record zero-recovery/normal-or-zero behavior, state/output bounds, storage words, exact
   scalar/W4/W8 operation graphs, underflow events and deterministic hashes.
5. Sol selects exactly one passing candidate and freezes it for a later product issue. If none
   passes, stop; do not add candidates or relax gates in the same attempt.

## Stop conditions

Stop for production changes, a mapping that skips f64 equivalence, f64 runtime lanes, unbounded
state, renamed failed recurrences, hidden underflow/recovery, tolerance/domain changes, timing or a
third attempt. `timed_benchmark_invocations=0`.
