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

Add an explicit engine-owned metric selection to `builtins_compiler::MeterRequest` and its sealed
preparation identity. The selection has independent stable bits for sample peak, energy/RMS,
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
