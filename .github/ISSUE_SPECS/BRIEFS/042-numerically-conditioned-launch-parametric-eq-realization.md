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

## Selected realization decision — endpoint-conditioned delta operator

**SOL ATTEMPT-2 SELECTION PASS; production implementation may proceed.** The complete rerun selected
one second-order delta family with an exact endpoint anchor. This does not pass the issue; all
post-selection gates below remain mandatory.

For normalized independent `f64` RBJ words `(b0,b1,b2,a1,a2)`, select the exact `f32` anchor
`a=+1` when `f0 <= Fs/4`, otherwise `a=-1`, and define `delta_a=z^-1-a`. Retain exactly seven
`f32` words per section/lane, in this order:

```text
a  = +/-1 exactly
n0 = f32(b0 + a*b1 + b2)       d0 = f32(1 + a*a1 + a2)
n1 = f32(b1 + 2*a*b2)          d1 = f32(a1 + 2*a*a2)
n2 = f32(b2)                    d2 = f32(a2)
```

RBJ design, normalization and the displayed transforms execute in `f64` off render; each displayed
result is rounded once to `f32` in field order and then checked finite. Reconstruct
`A1=d1-2*a*d2`, `A2=d2`; require finite nonzero `scale=(d0-a*d1)+d2` and strict Jury stability.
Numeric ramps advance in the existing frequency, gain, Q, slope order at each sample; if any
advances, redesign all seven words before processing that sample. Anchor changes require no state
conversion because the histories below remain direct input/output histories. A redesign failure
rolls all four ramp updates back, clears that section history, emits positive zero and increments
the lane recovery counter once for the sample.

Per section/lane retain exactly four `f32` histories `(x1,x2,y1,y2)`. With temporaries evaluated in
the following exact order, the scalar recurrence is:

```text
t0 = a*x
dx = x1-t0
t1 = a*x1
t2 = x2-t1
t3 = a*dx
ddx = t2-t3
p0 = n0*x
p1 = n1*dx
s0 = p0+p1
p2 = n2*ddx
num = s0+p2
q0 = a*d1
scale = (d0-q0)+d2
q1 = a*d2
q2 = (d1-q1)-q1
h0 = q2*y1
h1 = d2*y2
history = h0+h1
y = (num-history)/scale
x2=x1; x1=x; y2=y1; y1=y
```

Every scalar, Wasm `simd128`, NEON, AVX2 and scalar-tail path uses that noncontracting graph
lane-wise. The separately dispatched AVX2+FMA contract permits **zero contractions** for V1; it
uses the same multiply/add/subtract/divide graph and must be bit-identical. No relaxed-SIMD or
implicit compiler contraction may affect correctness.

Scalar storage is 28 coefficient bytes plus 16 history bytes per section/lane: four sections are
112 plus 64 bytes per lane. Width-W banks transpose each named field into `[f32; W]`: per
section/channel `7W` coefficient and `4W` history words, plus one exact W-byte derived identity mask.
Thus coefficient/history storage per four-section dual-mono bank is 896/512 bytes at W=4 and
1,792/1,024 bytes at W=8, excluding separately accounted configurations/ramps/masks. Absent and
scalar-tail lanes use the same representation and operation order; no lane shares state.

State layout remains V1 with zero common bytes and exactly 16 little-endian 32-bit words per
section/lane: histories `(x1,x2,y1,y2)`, then `(current,target,remaining)` for frequency, gain, Q
and slope in that order. That is 64 bytes/section, 256 bytes/lane and 512 bytes/dual-mono instance.
Coefficients, anchor and identity masks are derived, never serialized. Restore parses both complete
lanes into temporaries, validates finite normal-or-zero histories, parameter domains and
`remaining<=64`, redesigns every section and validates the selected invariants, then commits both
lanes atomically; any failure changes neither lane.

Identity/disabled sections return the input bits directly and warm `(x2,x1,y2,y1)` to
`(prior_x1,input,prior_x1,input)`. Whole bypass returns dry bits and performs the same warming for
all four sections. `FullToDefaults` clears histories and restores default fixed ramps;
`DiscontinuityKeepParameters` clears histories, snaps each ramp to its target and redesigns.
Nonfinite/subnormal input becomes positive zero with sanitation telemetry. A nonfinite/subnormal
output or history clears only that lane/section, emits positive zero and increments recovery once
for that lane/sample; valid paths never recover. Latency remains zero, tail Infinite, fixed scratch
zero, and all coefficient/state/bank storage is prepared and exactly accounted off render.

The selection rerun hash is `9ae58ca1fca97d4f`: 1,488 designs, zero delta failures, worst response
error `0.000552061269 dB`, worst stability margin `7.823109626770e-8` and maximum bounded state
`37.05598831177`. Any material change to these equations, seven words, four histories, operation
order or zero-FMA rule requires a new Sol decision before production code changes.

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
