# Filters and parametric EQ

## Scope and engineering question

Adopt the normalized RBJ response for launch HPF, LPF, and parametric EQ. Launch HPF/LPF use a two-integrator trapezoidal/TPT state-variable realization per dual-mono lane; parametric-EQ realization remains owned by its issue.

## Algorithm and equations

For launch HPF/LPF, `g=tan(pi*f0/Fs)`, `k=1/Q`, `d=1+g*(g+k)`, `c1=g*(g+k)/d` (the conditioned form of `1-1/d`), `a2=g/d`, and `a3=g*g/d`. For stored integrator states `ic1,ic2`, `v3=x-ic2`, `d1=a2*v3-c1*ic1`, `d2=a2*ic1+a3*v3`, `v1=ic1+d1`, `v2=ic2+d2`, `ic1'=ic1+2*d1`, and `ic2'=ic2+2*d2`; lowpass is `v2`, highpass is `x-k*v1-v2` [SIMPER-SVF] [ZAVALISHIN-TPT]. This is algebraically the trapezoidal/TPT SVF, but the incremental form avoids forming a rounded large `v` and then subtracting the old state. At `Q=1/sqrt(2)` its bilinear Butterworth response is independently checked against RBJ [RBJ-COOKBOOK].

## Coefficients and update rules

Validate/clamp parameter values off render, calculate normalized coefficients off render or at a bounded event boundary, and linearly ramp stable coefficient targets over a declared sample count. Do not interpolate an invalid coefficient set.

## Numerical and stability limits

Require finite `Fs > 0`, the owning issue's explicit cutoff domain, and finite positive Q. Reject invalid or unstable cast state-transition coefficients. Relative dB error is used only above a declared reference-magnitude floor; below it use absolute stopband/noise gates. Fixtures bound output and state.

Rest rule and its bound (issue #1328 amendment A8, accepted under decision 15 D15-4(a)). The joint rule zeroes the pair only on a sample whose input `x` is exactly zero (`+0.0` or `-0.0`), so while the input is not zero a section runs the per-word law bit for bit and applies its whole response, at any level; the rule only ends a decay after the input has stopped, which the per-word law would have continued below `REST_EPS`. Measured on the real EQ kernel at 96 kHz, A8 against the per-word law, for a 3.84 Hz square (half period 12,488 samples) for 1 s then 4 s of silence, and for a single impulse, at inputs from 1 down to `1e-30`: no output sample moves while the input is not zero, in any case. In the tails, at one +24 dB low shelf's output (10 Hz at S 1 and S 0.1, 100 Hz at S 1) every change is at most `1.9e-13` (-254.5 dBFS); at the output of four cascaded +24 dB low shelves (10 Hz and 100 Hz, S 1) at most `3.4e-10` (-189.4 dBFS). The chain number is larger because the tail an upstream section drops (a state below `REST_EPS` in each word) is boosted by every downstream section, which is still decaying when the upstream one rests; the first moved sample can sit on a loud tail (-116 dBFS output after a 0 dBFS square), with the change still more than 85 dB below it. A non-zero input sample followed by exact zeros counts as a tail too: an isolated impulse below `L* = REST_EPS / (2*max(a2, a3))` (`3.05e-11`, -210.3 dBFS, at the 10 Hz +24 dB shelf, 96 kHz) leaves both words below `REST_EPS`, and the next zero sample ends its response, so four such shelves lose its whole boosted tail: measured at most `1.6e-10` (-195.8 dBFS) for a `3e-11` impulse. The per-word law keeps its own dead zone, unchanged by #1328: an input below about `FLUSH_EPS / (2*max(a2, a3))` (`3.05e-17`, -330 dBFS, at the same shelf) never lifts a state word to `FLUSH_EPS`, so the section outputs only its direct term `(m0 + m1*a2 + m2*a3)*x` (inputs `1e-17` to `1e-30` render the same under both laws). Why exactly zero and not `|x| < f32::MIN_POSITIVE`: subnormal inputs can reach a section (wasm and the canonical native environment keep IEEE subnormals, and a trim, a fader or a section output can underflow), but they need no gate for rest. A subnormal sample adds at most `2*max(a2, a3)*2^-126` to a state word, far below `FLUSH_EPS`, so the per-word law already gives it no response; and a silent source reaches every downstream section as an exact zero (`+0.0` or `-0.0`) once the sections upstream rest, which gates 1-2 and the chain measurement show they do. Superseded: attempts 1-3 ignored the input. A section whose words sat below `REST_EPS` while a tiny signal drove it was zeroed every sample and passed only its direct term, losing up to 24.5 dB of response below `L*`, and through a cascade each following section saw the unboosted signal and stayed at rest too: four +24 dB low shelves lost about 96 dB (`1.5e-6` of change at the EQ's output, -116 dBFS, at 10 Hz; `2.2e-7`, -133 dBFS, at 100 Hz; the attempt-3 verdict). That rule was withdrawn, and gating on `x == 0` costs two lane operations per section.

## Latency and tail

Launch IIR filters declare zero algorithmic latency. Tail is state-dependent and is flushed by an explicit reset; bypass retains any graph-level latency compensation.

Every enabled TPT section (the builtin input HPF/LPF, the parametric-EQ bands and the multiband LR4 crossover) reaches exact rest after its input stops: both integrators `+0.0` and output `+0.0`, through the joint flush below (issue #1328, decision 15 D15-4(a)). Before it, two traps kept a section ringing forever below -200 dBFS: a period-2 limit cycle where the input filter's `c1` rounds to `1.0f32` (44.1 kHz LPF at the 22,049.482 Hz domain maximum, impulse 1234.5, cycling from sample 858,748), and a fixed point where `2*d2` is below half an ulp of `ic2` (EQ low shelf 10 Hz, +24 dB, S 0.1, 96 kHz, impulse 1.0, stuck at `ic2 = 6.01e-20`). With it those two cases are first seen at rest, state and output, at the end of the 128-frame block that ends at sample 773,888 and 421,888 respectively (blocks 6,045 and 3,295, counted from 0). The stated per-node bounds belong to issue #1329.

## Units, mappings, automation and smoothing

Frequency is Hz, Q is dimensionless, gain is dB, and slope uses the named filter form. Parameter metadata declares domain, default, event rate, and coefficient-ramp samples.

## Definitions and assumptions

L/R have separate state and independently automatable coefficients. A linked control is an explicit control-plane convenience, not shared DSP state; filters do not create cross-channel routing.

## Adopted decisions

Each prepared instance owns fixed coefficient/ramp/state storage. Issue-007 prepares conditioned `c1` directly in `f64`, casts it once, and stores `c1/a2/a3` as `f32`; render never recomputes `1-a1`. Production state, audio, and intermediates are `f32`; its independent oracle is `f64`. Render remains bounded and allocation/lock/I/O/log free.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input to zero and reset non-finite state deterministically. Suppress subnormal state through a documented finite threshold or platform-safe strategy; never allow it to propagate indefinitely.

Adopted law. Each new state pair `(n1, n2) = (ic1 + 2*d1, ic2 + 2*d2)` passes through `flush_pair` once per sample inside `svf_step`, the only copy of the recurrence: every word with `|n| < FLUSH_EPS = 1e-20` becomes `+0.0` (the per-word law, which keeps subnormals out of state and makes it FTZ-inert), and when the section's input sample is exactly zero and **both** `|n1| < REST_EPS` and `|n2| < REST_EPS`, with `REST_EPS = 1e-14`, both become `+0.0` together (issue #1328, the input gate of amendment A8). The joint test is an `and` of two ordered compares, never a `max`: one word at or above `REST_EPS` keeps the pair on the per-word law bit for bit (so an audible partner word is never zeroed), a NaN in either word fails both compares and passes through to the once-per-block finiteness check, and `-0.0` becomes `+0.0`. The cost is 13 lane operations for the two words (two of them the input compare and its `and`) against 6 for two per-word flushes.

Why `1e-14` (numerical limit). With zero input the step is the linear map `A = [[1-2*c1, -2*a2], [2*a2, 1-2*a3]]`. The per-word flush perturbs each word by at most `FLUSH_EPS` per step and the update itself rounds about six times, so a trajectory can only stall (a fixed point or a limit cycle) inside a ball of radius `kappa*sqrt(2)*FLUSH_EPS / (1 - rho - 6*2^-24*kappa)` around the origin, with `rho` the spectral radius of `A` and `kappa` the condition number of its unit-column eigenvector matrix [ORFANIDIS-ISP] (finite-wordlength limit cycles in recursive filters). Recomputed for this issue from the cast `f32` words over each domain: input HPF/LPF, every launch rate and cutoff, worst `6.66e-16` at the 44.1 kHz maximum cutoff (`rho = 0.99994785`, `kappa = 2.415`); parametric EQ over a frequency x gain x Q x slope grid at every launch rate and kind, worst `3.385e-15` (bell 10 Hz, Q 18, +24 dB, 96 kHz; `rho = 0.99999543`, `kappa = 1.007`); multiband LR4 over 80 Hz to 8 kHz, worst `9.24e-18` (80 Hz, 96 kHz). `REST_EPS = 1e-14` clears all three by at least about 3x, so every stall ball lies inside the joint-flush band; `1e-17` still left hundreds of never-resting runs per rate (decision 15, round 2). The rule only ever zeroes state, and each step matrix is non-expansive, so ramps in flight stay safe. Rejected: dropping the per-word flush (subnormal state, loses gate G6), capping the cutoff domain (misses the EQ trap), flushing `ic1` whenever `ic2` flushes (zeroed a -24 dBFS partner word, a click), and magnitude truncation (moves audible bits).

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
