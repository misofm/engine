# Bound offline limiter construction before window/allocation arithmetic

## Finding and smallest successor slice

The #1141 whole-package audit found a source-level refusal gap in `crates/dsp-reference/src/true_peak_limiter.rs::ReferenceTruePeakLimiter::new`. Root independently inspected the constructor and all current callers. This issue owns safe, typed input refusal in that offline reference constructor; it does not change the engine limiter, the DSP law or accepted launch-rate rendering.

The constructor documents `InvalidInput` outside the frozen launch domain, but its rate guard accepts every finite positive rate. At100 Hz, `n=1` and `ring_length=2`, so the next `clamp(32,2)` has reversed bounds. At sufficiently large finite rates, the saturating float-to-usize conversion can produce `usize::MAX`, after which unchecked latency/ring additions overflow or enormous vectors are requested. These are source-derived paths, not an executed reproducer or a claim about accepted engine sessions. Current reference callers use the four launch rates or48 kHz; all existing supported-rate tests remain green.

## Frozen boundaries / decision before implementation

Determine and record the constructor's intended rate domain from its existing documented launch-domain contract and current consumers before implementation; do not extend it for this fix. Validate that domain and all required window/capacity arithmetic before constructing vectors. Refused input returns the existing typed `InvalidInput`; no algorithm, coefficient, operation order, latency, supported-rate state or generated corpus changes. Keep this issue bounded to the reference constructor and its focused refusal owner. Shared reference/rate architecture is a separate issue if needed.

## Objective gates and test value

A minimal constructor regression must prove typed refusal without panic at the discovered low-rate boundary and relevant nonfinite/huge-rate boundary, while preserving current supported-rate construction. It must be red when the constructor fix is reverted; current tests cover bad lookahead at48 kHz and do not reach the reversed clamp bounds. Retain existing48-kHz latency/ramp-law owners and the four-rate independent product gain-law gate. Use focused locked package tests and lint; no new harness, timing, corpus digest or expanded target matrix. Reference numerical equations and the independently typed detector table remain unchanged; their current source/issue016 and issue090 documentation remains the algorithm evidence. No listening or sound-quality change is claimed.

## Evidence / authorization

Finding supplied by worker B (requested GPT-6.1 Sol xhigh), independently checked by root Sol on 2026-10-01 during #1141. No fix, runtime reproducer or implementation attempt is authorized by this finding record. A bounded Sol brief must freeze the rate-domain decision and gates before a later attempt. This is a correctness follow-up, not an owner API design question.

## Decision record (attempt 1, 2026-10-24)

- Rate domain (root decision, from the scope note): exactly 44 100, 48 000, 88 200 and 96 000 Hz
  (owner ruling R5), checked in the same form as `dsp-reference/src/delay.rs`
  (`matches!(rate as u32, ...)` and an integral-value check). Every caller uses these rates. The
  module law already assumes `R <= 961`.
- With the guard, `n` is 441..=960, so the `n == 0` branch was dead and was deleted. No checked
  arithmetic was added (it would be dead). The law, coefficients, latency and ramp windows did not change.
- Regression: `out_of_domain_construction_is_rejected` has rows `1_000.0` and `f64::MAX` (must be
  `InvalidInput`) and one positive row for the four launch rates. No new test function.
- Red on the reverted guard (debug): the `1_000.0` row panics in `clamp` (reversed bounds); with
  only the `f64::MAX` row, `n + 6` panics with "attempt to add with overflow". Green with the fix.
