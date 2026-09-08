# Applicability assessment

The actual repeated materializations are:

- native AVX2 W8: `-1` and folded `-3`, each separately broadcast in the even
  and odd calls in one frame iteration;
- Wasm scalar: `-1`, `+1` and folded `-3`, each separately materialized in the
  even and odd calls.

Native scalar loads `+1`, `-1` and folded `-3` once and reuses registers. W4
materializes each candidate once and reuses locals. Source `±2/3` versus folded
odd `±1/3` are different values and do not count as repetition.

The repeated threshold values originate in the two inlined invocations of the
same `cubic` helper. Supplying one value to both calls could plausibly remove the
duplicate materialization while preserving the existing arithmetic operations and
order; that is a source-ownership hypothesis for a separately reviewed XHIGH
issue. This record does not move expressions, change division, claim speed, or
authorize implementation.
