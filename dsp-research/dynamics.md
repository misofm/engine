# Dynamics processors

## Scope and engineering question

Use explicit feed-forward dynamics building blocks for compressor, gate/expander, de-esser, dynamic EQ, and multiband designs. This note does not choose a particular product preset set.

## Algorithm and equations

Detector and gain computer are separately declared: an example one-pole envelope is `e[n] = a*e[n-1] + (1-a)*u[n]`, with `a = exp(-1/(tau*Fs))`; the static curve maps declared detector level to gain [REISS-COMP].

## Coefficients and update rules

Attack, release, hold, lookahead, detector mode, knee, and sidechain filters are parameterized explicitly. Convert time units to coefficients at prepare/event update and use bounded smoothing for gain targets.

## Numerical and stability limits

Times must be finite and nonnegative; ratio, threshold, knee, and range use bounded declared domains. Floor logarithmic detector input before log conversion and reject non-finite parameter updates.

## Latency and tail

Lookahead declares exact integer samples and bypass retains that latency. Envelope/hold state creates a declared finite tail policy and reset clears it deterministically.

## Units, mappings, automation and smoothing

Threshold/makeup/range use dB, times use ms, ratio is dimensionless, detector mode and curve family are enums. Every automated parameter names its smoothing and update rate.

## Definitions and assumptions

L/R detector and gain states are independent by default. Peak, RMS, average, or max detector links must be explicit metadata; sidechain ports and their PDC are explicit graph inputs.

## Adopted V2 decisions

Prepared fixed-size detector, delay, and filter state only; lookahead rings are allocated before render. The callback has bounded work per frame and performs no allocation or blocking.

## Denormal, signed-zero and NaN policy

Non-finite input is sanitized before detector math; non-finite envelope/gain resets to unity-safe state. Subnormal detector state is flushed under the documented policy.

## Primary and official sources

[REISS-COMP] supports detector, gain-computer, and time-constant analysis. [SMITH-SASP] supports delay/filter state treatment. [VST3-LATENCY] supports fixed reported processor-latency semantics.

## Fixtures

Use level steps, attack/release bursts, threshold sweeps, hold transitions, sidechain impulses, asymmetric channels, lookahead impulses, silence/subnormal, and non-finite input fixtures.

## Objective tests and tolerances

Check `f64` envelope/gain agreement within effect-declared tolerance, exact reported lookahead/PDC, finite output, detector-link behavior, scalar repeat identity, and bypass latency preservation.

## Rejected alternatives and tradeoffs

The reference uses separate `f64` detector, curve, and delay types, not production structs or tables. A dependency/source scan fails if production dynamics kernels enter the reference path.

## Known gaps and follow-up

Bank only homogeneous dynamics programs; parameters and detector state remain lane-local. Scalar tail is mandatory; Wasm avoids FMA-dependent correctness and native FMA is separately dispatched.

## Benchmark plan

Measure each detector/link variant with deterministic level-step automation at fixed launch benchmark cases and complete JSON host/runtime metadata.

## Listening protocol or evidence

Use blinded matched-loudness tests on attack/release and link-mode candidates after all envelope and latency gates pass.

## 17. Decision record

Fact: dynamics behavior depends on explicitly chosen detector, curve, and time constants [REISS-COMP]. Adoption: expose them in metadata rather than imply stereo linking. Measurable reason: fixtures can isolate each transfer/state rule. Open question: future program-dependent release policies.
