# Filters and parametric EQ

## Scope and engineering question

Adopt the normalized RBJ response for launch HPF, LPF, and parametric EQ. Launch HPF/LPF use a two-integrator trapezoidal/TPT state-variable realization per dual-mono lane; parametric-EQ realization remains owned by its issue.

## Algorithm and equations

For launch HPF/LPF, `g=tan(pi*f0/Fs)`, `k=1/Q`, `d=1+g*(g+k)`, `c1=g*(g+k)/d` (the conditioned form of `1-1/d`), `a2=g/d`, and `a3=g*g/d`. For stored integrator states `ic1,ic2`, `v3=x-ic2`, `d1=a2*v3-c1*ic1`, `d2=a2*ic1+a3*v3`, `v1=ic1+d1`, `v2=ic2+d2`, `ic1'=ic1+2*d1`, and `ic2'=ic2+2*d2`; lowpass is `v2`, highpass is `x-k*v1-v2` [SIMPER-SVF] [ZAVALISHIN-TPT]. This is algebraically the trapezoidal/TPT SVF, but the incremental form avoids forming a rounded large `v` and then subtracting the old state. At `Q=1/sqrt(2)` its bilinear Butterworth response is independently checked against RBJ [RBJ-COOKBOOK].

## Coefficients and update rules

Validate/clamp parameter values off render, calculate normalized coefficients off render or at a bounded event boundary, and linearly ramp stable coefficient targets over a declared sample count. Do not interpolate an invalid coefficient set.

## Numerical and stability limits

Require finite `Fs > 0`, the owning issue's explicit cutoff domain, and finite positive Q. Reject invalid or unstable cast state-transition coefficients. Relative dB error is used only above a declared reference-magnitude floor; below it use absolute stopband/noise gates. Fixtures bound output and state.

## Latency and tail

Launch IIR filters declare zero algorithmic latency. Tail is state-dependent and is flushed by an explicit reset; bypass retains any graph-level latency compensation.

## Units, mappings, automation and smoothing

Frequency is Hz, Q is dimensionless, gain is dB, and slope uses the named filter form. Parameter metadata declares domain, default, event rate, and coefficient-ramp samples.

## Definitions and assumptions

L/R have separate state and independently automatable coefficients. A linked control is an explicit control-plane convenience, not shared DSP state; filters do not create cross-channel routing.

## Adopted V2 decisions

Each prepared instance owns fixed coefficient/ramp/state storage. Issue-007 prepares conditioned `c1` directly in `f64`, casts it once, and stores `c1/a2/a3` as `f32`; render never recomputes `1-a1`. Production state, audio, and intermediates are `f32`; its independent oracle is `f64`. Render remains bounded and allocation/lock/I/O/log free.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input to zero and reset non-finite state deterministically. Suppress subnormal state through a documented finite threshold or platform-safe strategy; never allow it to propagate indefinitely.

## Primary and official sources

[RBJ-COOKBOOK] supplies the coefficient families and normalization convention. [SMITH-SASP] supports IIR realization/state analysis. [ORFANIDIS-ISP] is a scholarly cross-check for digital-filter constraints.

## Fixtures

Use impulse, DC, sine, swept-sine, stepped-frequency/Q/gain, near-Nyquist, silence/subnormal, non-finite input, asymmetric L/R, and abrupt-bypass fixtures at every launch rate.

## Objective tests and tolerances

Compare analytic state-space, impulse DFT, and coherent sustained-sine response to the independent `f64` model with distinct relative-response, residual-noise, and absolute-stopband gates; also assert zero latency, finite state, scalar repeat identity, and manifest integrity.

## Rejected alternatives and tradeoffs

`f32` TDF-II was rejected for issue-007 HPF/LPF after sustained high-rate stopband tests exposed state-rounding error hidden by coefficient and impulse-only checks. The first direct TPT graph was also rejected: `v=s+d` followed by `2*v-s` rounded a small integrator increment into a larger state before cancellation and failed the frozen residual gate. The algebraically equivalent incremental update passed the full prescribed rate/filter/cutoff/probe matrix without wider state. Test-only `f64` owns separate equations/state. Wider production state requires a separate issue and portable SIMD/resource evidence.

## Known gaps and follow-up

Vectorize only after scalar semantics pass. AVX2/FMA dispatch is separate; base Wasm/NEON/AVX2 preserve the non-fused `f32` graph. Higher-precision production is deferred to issue 031.

## Benchmark plan

Run two clean native rounds at 48/96 kHz and 64/256 frames for a full bank and scalar tail; emit median, p95, p99, p99.9 ns/block and cycles/block when available with required machine metadata.

## Listening protocol or evidence

Record blinded ABX or randomized A/B of matched filter moves using `listening/TEMPLATE.md` after objective gates pass; evidence is descriptive, not an acceptance substitute.

## 17. Decision record

Fact: RBJ documents the response family, and trapezoidal/TPT sources derive an equivalent two-state realization with limited-precision motivation [RBJ-COOKBOOK] [SIMPER-SVF] [ZAVALISHIN-TPT]. Adoption: issue-007 HPF/LPF use the exact non-fused incremental `f32` TPT recurrence per lane and a stored conditioned complement. Measurable reason: the direct TPT update failed at `-94.244 dB` residual while the incremental candidate passed all 232 launch-matrix single-section cases; a 464-case superset including the four deferred extended rates also passed, with worst residual `-116.346 dB`. Its superset worst analytic/cutoff errors (`0.00000176 dB`) were materially below recomputing `1.0_f32-a1` (`0.001324/0.001413 dB`). Issue 031 evaluated one portable retained-`f64` candidate and did not adopt it: despite material time-domain improvement, 38 analytic rows and the frozen impulse/DFT tolerance failed, so launch `f32` remains unchanged.
