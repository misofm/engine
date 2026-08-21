# Sol implementation brief — issue 059 builtin cascade decay and recovery contract

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1.** This issue gets one Terra implementation/review and one bounded Sol
correction/review. A second failure stops. It owns one decision and, only if proved necessary, one
local TPT recovery correction. It does not complete the corpus. Workload and timed benchmark
invocations remain zero.

## Frozen experiment

Use the current V2 conditioned builtin TPT only. At each launch rate `44100,48000,88200,96000`,
render one second from one unit impulse through a 100-Hz HPF followed in production order by a
1-kHz LPF. Record for each section and lane the sample index, pre-state bits, next-state bits,
output bits, classification and report delta for every canonicalization/recovery event. Run the
same sequence at quanta `1,127,128,255,1024`; response probes are metadata consumers of this one
render and cannot trigger another semantic path.

Build the comparison from an independent retained-`f32`, non-fused recurrence using the frozen
coefficient bits and exact operation order. Do not compare a finite impulse to an infinite-only
analytic oracle. Analytic and finite-window magnitude remain guardrails at Issue-035 tolerances,
not tuning surfaces.

## Bounded decision surface

Accept repeated cascade events only if the independent recurrence proves the exact same timeline,
the states are finite, the behavior is partition/probe invariant, and the reporting meaning is
coherent for a legal decay. Freeze exact totals and whether finite subnormal decay is named
canonicalization or recovery. Otherwise classify a product defect and make only the smallest
existing-topology correction that prevents ordinary decay from repeatedly reporting invalid-state
recovery. Do not change filter coefficients, cutoff domains, recurrence order, response tolerance,
state/resource layout, latency or Infinite tail.

The accepted result must give Issue 060 an executable rule for validating every serialized
response recovery field. A descriptive count with no independent event classification is FAIL.

## Ordered gates and stop rules

Run the single focused four-rate timeline first, then partition/probe invariance and existing
scalar/bank recovery parity, followed by relevant format/test/Clippy/diff checks. Stop for a new
filter topology, corpus expansion, audit/target/benchmark work, tolerance/domain changes or any
unexplained event. Record strict PASS or FAIL with zero workload/timed invocations.
