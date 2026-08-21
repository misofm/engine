# Sol implementation brief — issue 042 numerically conditioned launch parametric EQ realization

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1, PREIMPLEMENTATION PHASE ONLY.** Issue 042 has exactly two total
attempts: one Terra implementation/review attempt and, if necessary, one bounded Sol correction.
A second failure stops. Do not edit production EQ/core kernels or state layout until the comparison
checkpoint below passes and Sol replaces the unresolved decision block with one exact realization.

Issue 012 is STOPPED/RESCOPED without overall PASS. Its checkpoints `46b4a37`, `7b9c01b` and
`cf739ef`, plus Issue-008 checkpoint `87783c5`, are technical inputs only. Preserve their failure
records. Never inspect V1/legacy and never run or create a benchmark;
`timed_benchmark_invocations=0` is invariant.

## Preserved launch product surface

Freeze independently of the realization decision:

```text
effect ID             miso.parametric-eq
contract              1.0
quality               Normal
ports/link             main-in/main-out; DualMono only
sections/order         exactly four; 0 -> 1 -> 2 -> 3
kinds                  bell, low/high shelf, low/high pass, notch
rates                  44100, 48000, 88200, 96000 Hz
frequency              10..=20000 Hz
gain/Q/S               -24..=24 dB / 0.1..=18 / 0.1..=1
automation             block Point; exact linear 64-update numeric ramps
latency/tail            0 / Infinite
```

Retain the 24 Issue-012 per-lane parameter IDs, mappings/defaults, immutable enabled/kind and whole
bypass. Preserve input sanitation, lane-local bounded recovery, exact dry identity, all-or-none
state operations, scalar tails and no compiled track limit. Failed DF-I cache/state bytes are not
a compatibility contract; state layout V1 is frozen only after selection.

## Mandatory preimplementation comparison

Build comparison code only in a test/reference boundary that production cannot import. Evaluate
TPT/state-variable, coupled-form and delta-operator realization families. Each family must have
either executable retained-`f32` equations or a cited, reproducible rejection showing it cannot
meet a frozen numerical, bounded-state or SIMD requirement. Do not use timing as a selector.

For every launch rate, use the full applicable Cartesian grid:

```text
f0    10,20,100,1000,10000,20000 Hz
Q     0.1,0.7071067811865476,1,18
gain  -24,-6,0,6,24 dB
S     0.1,0.5,1
```

Bell uses f0*Q*gain; shelves f0*gain*S; pass/notch f0*Q. Probe a 2,048-point log grid
10–20,000 Hz plus exact f0 and DC/Nyquist. Compare retained-`f32` analytic/state-transition
response with an independent f64 transfer and run a bounded scalar recurrence/impulse probe.
Record, per candidate, exact retained words, worst response row/probe, strict stability margin,
null depth, cutoff/center/midpoint/minimum error, state range and fixed scalar/four/eight-lane
storage shape.

A candidate is selectable only if every applicable row has:

- finite stable retained state/coefficients and no legal rejection;
- <=0.005 dB analytic/state-transition error where reference >=-120 dB;
- <=0.05 dB one-second impulse/DFT error in that region;
- theoretical null <=-100 dB;
- LP/HP Butterworth crossing, bell center, shelf midpoint and notch minimum within 0.1%; and
- a finite normal-or-zero scalar recurrence with no recovery on the bounded valid probe.

Do not delete endpoint, f0 or near-null probes, raise the 10 Hz minimum, alter tolerances, compare
only f64 formulas, or special-case the known low-shelf row. If no candidate passes, stop Issue 042
without production implementation.

## Unresolved realization decision — mandatory Sol amendment

After the comparison, pause. Sol must record exactly one selected family and freeze all of the
following before production work resumes:

1. design equations, normalization/prewarp and retained `f32` coefficient words/domains;
2. per-section/per-lane state words, byte layout/version, derived versus serialized values and
   all-or-none validation;
3. scalar recurrence in exact operation order and the point at which smoothed values redesign it;
4. four/eight-lane coefficient/state transposition, masks and scalar-tail behavior;
5. base Wasm/NEON/AVX2 noncontraction graph and every separately gated AVX2+FMA contraction;
6. exact identity hidden-state warming, bypass advancement, reset/discontinuity, invalid-state and
   redesign-failure recovery; and
7. fixed latency, tail, scratch/state ceilings and NaN/subnormal behavior.

This section may not be filled by implementation convention or retroactive tests. Any selected
surface that materially changes effect IDs/parameters, graph architecture or Issue-011 APIs needs
a separate rescope rather than silent expansion.

## Post-selection product gates

After the Sol amendment, implement only the selected four-section scalar and homogeneous-bank
vertical. Reuse landed Issue-012 descriptor/automation/dispatch code only where the amended brief
proves it compatible.

Run the full comparison grid against production, exactly 10,000 legal designs with seed
`0x000000000012e911`, and the 48 frozen million-sample valid-stability sequences. Prove exact
64-update/restart automation, malformed-span rejection, input/output recovery, identity/bypass,
both resets, all-or-none state continuation and one-lane/track isolation. Base same-target paths
must be bit-identical; the amended brief must name any FMA differential formula before its first
production use.

One compact nine-track registry/session/graph fixture proves host-selected full bank(s) plus scalar
tail, deterministic output and unchanged graph/PDC/schedule/observer behavior. Exactly 100,000
prepared 128-frame renders prove zero forbidden operations while armed and off-render destruction.
Run locked focused/full tests, warning-denied Clippy/rustdoc, policies and native/x86/AArch64/Wasm
compile/instruction gates. There is no benchmark, tuning, completed listening or graph/control
automation scope.

## Evidence and stop rules

Append candidate/reference hashes, complete comparison results, Sol selection amendment, final
descriptor/coefficient/state/operation tables, response and seeded maxima, fixture/graph/audit and
target evidence, and Terra/final Sol verdicts to Issue 042. State
`timed_benchmark_invocations=0`.

FAIL immediately for production edits before selection, a missing candidate/rejection rationale,
any tolerance/probe/domain weakening, f64 production lanes, an unfrozen recurrence/state payload,
runtime allocation/feature detection, cross-lane/track state, or a benchmark. Preserve the
evidence and stop after the second failed total attempt.
