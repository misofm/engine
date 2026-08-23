# Sol implementation brief — issue 107 F1 canonical f32

## Decision

**READY / SOL XHIGH PASS.** Implement the smallest independent F1 slice only. Canonical TOML must
round-trip every finite `f32` through direct `f32` and `f64`-then-cast parsing while preserving
`-0.0`. One exhaustive release invocation is authorized after all other gates are green.

## Exact implementation

Centralize all 19 canonical float call sites on `value::write_f32`. Write normal `f32` Display,
verify its suffix by parsing as `f64` and casting to `f32`, replace only a failing suffix with exact
`f64::from(value)` Display, then append `.0` when no decimal point/exponent exists. Remove zero
folding from the formatter and `bounded_f32`.

Do not always emit f64 text, use Debug, alter the parser/API/schema, touch fixtures, or change the
size estimate unless the maximal-float compile gate proves the documented adjustment necessary.

## Proof and budget

E1 covers the two double-rounding patterns, signed zero and boundary bits. E2 checks ten million
seeded patterns. E4 proves direct/compiled/recanonicalized session bits and unchanged fixture
bytes. Execute and revert all five red mutations. Only after every non-exhaustive command in the
issue spec passes, run E3 exactly once across all `2^32` patterns; require zero mismatch, exactly
two fallbacks and maximum spelling length at most 50 (expected 48). Do not retry or tune.

## Fence and stop conditions

Edit only session value/canonical/tests, session schema documentation, Issue-107 evidence, and the
conditional estimate constant. No fixture, Cargo, protocol, graph, effect or Issue-004 edit. No
benchmark, fuzz, timing or target matrix.

Stop on any unexpected finite mismatch, fallback count other than two, maximum length over 50,
fixture delta, or consumed exhaustive-runner failure. Do not add special cases or loosen gates.
