# Filters and parametric EQ

## Scope and engineering question

Adopt normalized RBJ biquad forms for launch HPF, LPF, and parametric EQ; use transposed direct form II state per dual-mono lane. This note does not select an effect UX or analog-model mode.

## Algorithm and equations

For normalized coefficients, `y[n] = b0*x[n] + z1`, `z1 = b1*x[n] - a1*y[n] + z2`, and `z2 = b2*x[n] - a2*y[n]`. Coefficients derive from `w0 = 2*pi*f0/Fs`, `alpha = sin(w0)/(2Q)` where the selected RBJ filter form applies [RBJ-COOKBOOK].

## Coefficients and update rules

Validate/clamp parameter values off render, calculate normalized coefficients off render or at a bounded event boundary, and linearly ramp stable coefficient targets over a declared sample count. Do not interpolate an invalid coefficient set.

## Numerical and stability limits

Require finite `Fs > 0`, `0 < f0 < 0.5*Fs`, and finite positive Q. Reject instead of silently accepting a denominator that is zero or non-finite. Fixture tests bound output and assert no non-finite state.

## Latency and tail

Launch IIR filters declare zero algorithmic latency. Tail is state-dependent and is flushed by an explicit reset; bypass retains any graph-level latency compensation.

## Units, mappings, automation and smoothing

Frequency is Hz, Q is dimensionless, gain is dB, and slope uses the named filter form. Parameter metadata declares domain, default, event rate, and coefficient-ramp samples.

## Definitions and assumptions

L/R have separate state and independently automatable coefficients. A linked control is an explicit control-plane convenience, not shared DSP state; filters do not create cross-channel routing.

## Adopted V2 decisions

Each prepared instance owns fixed coefficient/ramp/state storage. Render performs only bounded arithmetic over supplied frames and no allocation, lock, I/O, or logging.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input to zero and reset non-finite state deterministically. Suppress subnormal state through a documented finite threshold or platform-safe strategy; never allow it to propagate indefinitely.

## Primary and official sources

[RBJ-COOKBOOK] supplies the coefficient families and normalization convention. [SMITH-SASP] supports IIR realization/state analysis. [ORFANIDIS-ISP] is a scholarly cross-check for digital-filter constraints.

## Fixtures

Use impulse, DC, sine, swept-sine, stepped-frequency/Q/gain, near-Nyquist, silence/subnormal, non-finite input, asymmetric L/R, and abrupt-bypass fixtures at every required rate.

## Objective tests and tolerances

Compare impulse and sine responses to the independent `f64` model using declared tolerance; assert zero reported latency, finite output/state, scalar repeat byte identity, and corrupt-manifest rejection.

## Rejected alternatives and tradeoffs

Test-only `f64` code re-derives the normalized equations and owns separate state types. CI rejects imports or dependencies from the production filter kernel into the reference crate.

## Known gaps and follow-up

Vectorize independent lanes only after scalar semantics pass. AVX2/FMA dispatch is separate; base Wasm uses multiply then add, and all backends compare to scalar with the declared tolerance.

## Benchmark plan

Run two clean native rounds at 48/96 kHz and 64/256 frames for a full bank and scalar tail; emit median, p95, p99, p99.9 ns/block and cycles/block when available with required machine metadata.

## Listening protocol or evidence

Record blinded ABX or randomized A/B of matched filter moves using `listening/TEMPLATE.md` after objective gates pass; evidence is descriptive, not an acceptance substitute.

## 17. Decision record

Fact: normalized biquad coefficient families are documented [RBJ-COOKBOOK]. Adoption: bounded transposed DF-II per dual-mono lane. Measurable reason: compact fixed state and reference impulse agreement. Open question: whether a future high-order mode warrants state-space interpolation.
