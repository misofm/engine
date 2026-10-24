# Dynamics processors

## Scope and engineering question

Use explicit feed-forward dynamics building blocks for compressor, gate/expander, de-esser, dynamic EQ, and multiband designs. This note does not choose a particular product preset set.

## Algorithm and equations

Detector and gain computer are separately declared: an example one-pole envelope is `e[n] = a*e[n-1] + (1-a)*u[n]`, with `a = exp(-1/(tau*Fs))`; the static curve maps declared detector level to gain [REISS-COMP].

## Coefficients and update rules

Attack, release, hold, detector mode, and sidechain filters are parameterized explicitly. The V1
gate/expander detects the current sample and adds zero audio latency. Convert time units to
coefficients at prepare/event update and use bounded smoothing for gain targets.

## Numerical and stability limits

Times must be finite and nonnegative; ratio, threshold, knee, and range use bounded declared domains. Floor logarithmic detector input before log conversion and reject non-finite parameter updates.

## Latency and tail

The V1 gate/expander reports exact zero added samples in enabled and bypassed modes. Envelope/hold
state creates a declared finite tail policy and reset clears it deterministically.

The compressor (#1375) has zero latency and states a zero tail: its output is the current input
times a finite gain (or that mixed with the input), so it is exactly `±0.0` from the first silent
sample. Its exact rest is the gain-reduction word's release from at most 100 dB to the #1784 rest
snap at the slowest release (5,000 ms), which `effect_runtime::tail::rate_follower_rest_samples`
counts on the word's `f32` grid: 4,569,940 samples at 48 kHz (about 95 s), within a sample of the
real recurrence. The composition gain is 2,401 mB (+24 dB makeup, the fast tier's `1e-5` dB gain
error and four roundings); the flush stall is four underflows, -74,655 mB.

The transient shaper (#1376) has zero latency and states a zero tail: its output is the current
input, or `gain_mix_step` of it with a finite gain. With a zero detector both followers release in
the two-product pole form `e' = snap(fl(c e))`, `1 - c` exact, which
`effect_runtime::tail::pole_follower_rest_samples` counts on the word's `f32` grid from the largest
envelope value, the input peak inflated by `3 u / k_min` (the follower's rounding cannot push it
further). The slow 100 ms release dominates: 145,892 samples at 48 kHz for a +24 dBFS peak and
464,191 for any sanitized input (`< 1e30`), equal to the real recurrence at every launch rate. The
composition gain is 1,801 mB (the +18 dB shape clamp, the fast tier's `1e-5` dB and four
roundings); the flush stall is four underflows, -74,655 mB.

The gate/expander (#1376) has zero latency and states a zero tail: its output is the current input
or that times the gain. It does not rest at zero. With zero input the level clamps to -160 dB, the
gate holds for at most its hold count and closes, and the gain word releases toward the curve's
silent target `t` (at most the range) but never reaches it: it stops at the first word whose rate
step rounds away, the *stall word*, up to about 0.74 dB short of `t` at 2,000 ms and 96 kHz. That
word is the rest state's gain (with the gate closed and the hold at zero), and it is the same from
every history with the same parameters: near the end every step moves one word, so every walk
passes the same words, and none jumps past the first stall. `effect_runtime::tail::rate_follower_stall_samples`
bounds the walk over every target and every release coefficient of the domain; the slowest targets
sit just above a power of two, where the walk ends on the finer grid. `R` is the longest hold plus
that walk: 602,859 samples at 48 kHz (12.6 s), 0.27 % above the longest walk measured. A parameter
edit that leaves the word deeper than its target makes it rise at the attack rate and stop on the
far side, on a fixed point that is not the fresh instance's rest state, so the gate states its `R`
as a fixed-point bound (`RestBound::FixedPoint`, root option (a)); that walk is bounded separately
and is shorter. A snap to the target would make the rest state unique and is a rendered-bit change. The composition gain is 1 mB (0 dB, the fast tier's `1e-5`
dB and one rounding) for every sidechain input; the flush stall is one underflow, -75,859 mB.

## Units, mappings, automation and smoothing

Threshold/makeup/range use dB, times use ms, ratio is dimensionless, detector mode and curve family are enums. Every automated parameter names its smoothing and update rate.

## Definitions and assumptions

L/R detector and gain states are independent by default. Peak, RMS, average, or max detector links must be explicit metadata; sidechain ports and their PDC are explicit graph inputs.

## Adopted decisions

Prepared fixed-size detector and filter state only; the causal gate owns no audio-history ring. The
callback has bounded work per frame and performs no allocation or blocking.

## Denormal, signed-zero and NaN policy

Non-finite input is sanitized before detector math; non-finite envelope/gain resets to unity-safe state. Subnormal detector state is flushed under the documented policy.

## Primary and official sources

[REISS-COMP] supports detector, gain-computer, and time-constant analysis. [SMITH-SASP] supports delay/filter state treatment. [VST3-LATENCY] supports fixed reported processor-latency semantics.

## Fixtures

Use level steps, attack/release bursts, threshold sweeps, hold transitions, sidechain impulses,
asymmetric channels, same-index causal impulses, silence/subnormal, and non-finite input fixtures.

## Objective tests and tolerances

Check `f64` envelope/gain agreement within effect-declared tolerance, exact zero reported gate PDC,
finite output, detector-link behavior, scalar repeat identity, and bypass current-sample identity.

## Rejected alternatives and tradeoffs

The reference uses separate `f64` detector and curve types, not production structs or tables. A
dependency/source scan fails if production dynamics kernels enter the reference path.

## Known gaps and follow-up

Bank only homogeneous dynamics programs; parameters and detector state remain lane-local. Scalar tail is mandatory; Wasm avoids FMA-dependent correctness and native FMA is separately dispatched.

## Benchmark plan

Measure each detector/link variant with deterministic level-step automation at fixed launch benchmark cases and complete JSON host/runtime metadata.

## Listening protocol or evidence

Use blinded matched-loudness tests on attack/release and link-mode candidates after all envelope and latency gates pass.

## 17. Decision record

Fact: dynamics behavior depends on explicitly chosen detector, curve, and time constants [REISS-COMP]. Adoption: expose them in metadata rather than imply stereo linking. Measurable reason: fixtures can isolate each transfer/state rule. Open question: future program-dependent release policies.
