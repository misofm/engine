# Red-mutation log — native PCM reference runner

## Issue #241 — declared depth is checked against decoded depth

Applied on 2026-08-29, run, observed RED, and reverted.

| gate | mutation | observed red |
|---|---|---|
| `resolver_rejects_integer_and_float_depth_mismatches_both_directions_precompile` | invert the resolver's decoded-vs-declared depth comparison (`!=` → `==`) | the f32 WAVE declared as integer reaches output instead of returning `source.depth`; `expect_err("f32-declared-integer")` panics before compile-call/output-call zero assertions |

The unmutated fixture also runs the opposite direction using the canonical PCM16 WAVE declared as
`32f`; both refuse `source.depth` before any engine compile or output publication.
