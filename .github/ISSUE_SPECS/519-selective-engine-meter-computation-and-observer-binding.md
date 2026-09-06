# Selective engine meter computation and observer binding

## Problem

Engine meter requests currently bind every named track at one tap and every active accumulator
computes sample peak, `f64` energy/RMS, clip and sanitation counts, and held peak. The browser peak
adapter consumes only sample peak, yet preparation gives it no way to request less work. A released
browser lease is not proof that no engine consumer exists, and runtime subscription machinery is
outside this issue.

This is the first efficiency successor to #516. It preserves the corrected per-window publication
contract and all existing full-statistics callers.

## Smallest useful slice

Add an explicit engine-owned metric selection through additive
`builtins_compiler::SelectedMeterRequest` and its sealed preparation identity. Existing
`MeterRequest` literals remain the compatibility form and select all metrics. The selection has
independent stable bits for sample peak, energy/RMS,
interval counts (clipped and sanitized), and held peak. Reject an empty or unknown selection during
preparation. Keep window identity, generation, frame count, discontinuity count and dropped-snapshot
count mandatory transport metadata rather than optional metrics.

Add one narrow host-core preparation entry that accepts a caller-supplied slice of stable track/tap
and metric selections while retaining the console's common period and queue depth. Existing host
preparation delegates with its current canonical all-track/full-statistics selection, preserving
compatibility. Native, embedded and browser adapters use the same request type; the browser asks
for its existing all-track/post-matrix set with sample peak only. Do not encode consumer names or
presets in engine policy.

Carry the selected/present bits in `MeterSnapshot` without variable-size payloads. Unrequested
numeric fields have no semantic value; consumers must use presence metadata. Existing constructors
select all metrics.

Specialize outside the sample loop. A practical bounded implementation is one segment-level match
that calls const-generic/internal kernels for the finite metric groups; compilation removes energy,
count and hold operations for disabled groups. Do not add a per-sample metric branch, runtime
activation, a generic subscription transport, new meter kinds, or changes to source/tap identity.
Sanitizing nonfinite/subnormal input to zero remains necessary for every selected numeric metric;
increment the sanitized count only when counts are selected.

Update exact resource projection for any fixed snapshot/state growth before allocation. Preserve
bounded queues, preparation-time validation, arbitrary track counts, and allocation-free render.

## Acceptance evidence

- Peak-only performs no `f64` square accumulation or square root, clip-threshold count, held-state
  update or decay. Prove this with focused operation instrumentation outside production render or
  an equivalent compiled-code assertion that fails when each forbidden operation is restored.
- Peak-only sample peaks are bit-identical to the all-metrics path for finite values, signed zero,
  NaN/Inf, subnormals, full-scale threshold cases, silence, discontinuities and window splits.
- All-metrics snapshots preserve existing bit patterns for peak, energy, RMS and held peak, and
  exact count/cumulative semantics. Energy/RMS remain `sum(x²)` in `f64` and
  `sqrt(energy / frames)`.
- Presence bits distinguish unrequested values from measured zero. Queue loss, generation and
  sequence-gap behavior remain unchanged.
- Preparation rejects empty/unknown selections transactionally and includes metric selection in
  duplicate/seal identity. Unrequested observers are not bound.
- Exact retained totals, maximum single allocation, meter stream/item/byte caps and arbitrary
  track-count tails pass. Meter-off and peak-only/full enabled renders remain PCM bit-identical.
- Focused `builtins`, `builtins-compiler`, `host-core` and affected host suites pass on native and
  shipped Wasm callback gates.

Freeze one representative active-audio workload before timing. Run disabled, peak-only and full
once with one warmup and two measured rounds; report descriptive work/cycles without an optimization
claim or retry.

## Out of scope

No SDK UI controls, app presets, loudness, true peak, transport protocol, runtime reconfiguration,
poll cadence work, benchmark framework, or effect-observation redesign.

## Implementation decision and evidence

- `MeterMetricSet` assigns stable bits `1/2/4/8` to sample peak, energy/RMS, interval counts and
  held peak. `MeterSnapshot::present_metrics` is fixed-size mandatory presence metadata;
  generation, sequence, span and cumulative delivery counters remain mandatory. Empty and unknown
  sets are rejected before ring preparation.
- The compatibility entry still constructs the original full fused accumulator. Partial sets are
  selected once per segment and call independent straight-line peak, energy, count and held passes;
  there is no metric branch inside a selected pass. Test-only counters are incremented at the
  `f64` square, threshold count, held-state and square-root operation sites. The peak-only test
  observes zero for all four counters.
- The all-15-subsets test uses nonzero hold/decay, irregular observation splits, nonfinite and
  subnormal input, and threshold values. Every selected field matches the full fused reference by
  exact bits/counts. The existing full path remains the numeric oracle and is not reassociated.
- `SelectedMeterRequest` keeps existing request literals source-compatible and adds metrics to the
  preparation seal. The selected host entry accepts caller-ordered track/tap/metric records with
  common console period/depth, validates the returned bindings in that order, and binds no omitted
  track. The browser creates the same generic records for every canonical track at post-matrix with
  sample peak only; meter-off creates an empty record set.
- Fixed-size snapshot growth flows through the existing `size_of::<MeterSnapshot>()` SPSC payload,
  host pending-slot and largest-allocation calculations. Existing inclusive/one-below resource
  tests and browser retained-resource tests remain the boundary evidence rather than a parallel
  estimator.
- Focused evidence on 2026-09-06: selective builtins tests pass; the compiler's 10,000-case sealed
  mutation transcript passes with its deliberate metric-identity digest update; selected host
  ordering/validation passes; all 72 non-ignored `host-web` unit tests pass (one release benchmark
  remains ignored by design). Existing browser tests cover all four launch rates, a nine-track SIMD
  tail, meter loss/generation/empty/master behavior and PCM identity.
- The existing stage tangent ULP test fails identically on untouched baseline `c34383fc` and issue
  parent `b49f0910` at 44.1 kHz / 9,520 Hz. Independent logs are
  `/private/tmp/engine-519-baseline-tan.log` and `/private/tmp/engine-519-parent-tan.log`; #519 does
  not alter DSP math or weaken that gate.
- Timing is not yet claimed. The existing frozen benchmark hashes full-stat snapshots and has no
  selection mode; it was left unchanged rather than mutating its workload. The operation-site
  gate supplies the required work-removal evidence while a descriptive peak/full timing invocation
  remains pending owner review under the ceremony boundary.


## Timing disposition and independent review

Dedicated Astra medium source review passed, conditional on final shipped artifact qualification. The bounded timing helper was separately reviewed after correcting its JSON serializer. Release preflight passed. The sole timing invocation at `11cb3c2e` then failed during warmup: its final source chunk omitted the end-of-region marker and was correctly rejected. No measured rounds or timing figures exist. Raw evidence is committed in `docs/evidence/metering-519-520/timing-failed.log` and `timing-failed.jsonl`. No retry was performed and no measured speedup is claimed.

Per the repository's bounded benchmark-failure rule, the remaining timing requirement is explicitly transferred to [#522](https://github.com/misofm/engine/issues/522), whose stateless brief was written and reviewed by dedicated Astra medium. That tooling successor does not relax numerical, realtime, resource or work-avoidance gates for this feature.


## Final shipped-artifact qualification

Canonical Linux/amd64 Rust 1.97.1 build of candidate `a1aeefb830b2ac192274153539847765385ed7f1` (same executable source as `11cb3c2e`, reviewed pin adopted at `924e524e`) produces SHA-256 `671d615de9a74232c2af623592fc85eca68b8cd571fb5b7542ba648e621af4e3`. The ordinary fingerprint-enforcing builder passed. The unchanged static/object/callback/resource gates and hermetic worklet/mutation suite passed. Browser qualification passed Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, including mutation proofs; the committed results/matrix identifies this exact candidate and artifact.

Focused source suites passed: 73 host-web tests (two explicitly ignored release timing tests), selective builtins tests covering all 15 nonempty subsets and zero forbidden peak-only operations, compiler 38 tests, host-core unit 8 tests and preparation 13 tests (one intended ignored). Formatting, generated matrix consistency and realtime policy passed. The earlier baseline-only native tangent ULP failure remains documented; DSP arithmetic is unchanged. No callback/resource gate was weakened.

Timing remains unavailable: the single invocation failed during fixture warmup, and repair/measurement is explicitly assigned to #522. The supported performance result is avoided operations and empty scans, not a measured `µs/block` or end-to-end browser speedup. SDK/app rendering and presentation tuning remain downstream work.
