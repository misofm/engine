# Nonlinear antialiasing

## Scope and engineering question

Launch nonlinear processors use an oversampled, measured reference path before any claim of antialiasing. Antiderivative antialiasing (ADAA) is a future quality mode only after its singularity handling passes dedicated tests.

## Algorithm and equations

A memoryless nonlinearity computes `y[n] = f(x[n])`; launch comparison uses explicit interpolation/process/decimation stages. ADAA replaces direct evaluation with an antiderivative difference quotient where defined [BILBAO-ADAA].

## Coefficients and update rules

Prepare oversampling filters, drive, bias, shape, and mix smoothing. Near an ADAA denominator singularity, the future implementation must declare a finite limiting branch rather than divide by a small difference.

## Numerical and stability limits

Input/drive/bias domains are finite; `f`, antiderivative, and all intermediates must remain finite. Alias-spectrum and near-singularity acceptance limits are effect/quality-specific and cannot be inferred from a marketing label.

## Latency and tail

Latency equals declared multirate/filter delay plus any lookahead. A memoryless waveshaper has no intrinsic tail, while filter histories have declared reset/tail behavior; bypass retains PDC.

The launch soft clip (#1376) declares latency 31 and tail 29, recomputed from its layout: the
interpolator and the decimator each read 30 even-tap ages, so the last non-zero output is 60 samples
after the last non-zero input, 29 beyond the latency. Its exact rest is the reset state, every row of
its three 32-row rings `+0.0` (mirrors included, the words `lanes_at_rest` pins): the last non-zero
shaped sample is pushed 30 samples after the input stops and leaves its ring 32 pushes later, so
`R = 30 + 32 - 31 = 31` for every input peak, and the tail over every peak is 31. The composition
gain is 7,025 mB: the +24 dB output word times `2 g_drive (l1e^2 + 1/4)` at the +36 dB drive, with
`l1e` the even taps' `l1` norm, the centre tap `1/2`, the cubic's Lipschitz constant `1` (its clamp
is an absolute bound and is not used) and 129 roundings; the flush stall is 129 underflows carried
through two half-band stages and the output word, -68,344 mB.

## Units, mappings, automation and smoothing

Drive, output, and bias specify dB or linear units; shape and quality are enums or normalized domains with mappings. Parameter changes use documented bounded smoothing.

## Definitions and assumptions

Process L/R independently by default. Any linked drive/detector or 2x2 crossfeed is explicit metadata and fixtures must include asymmetric channel inputs.

## Adopted decisions

All multirate scratch, FIR, and state buffers are prepared. The callback executes fixed bounded work and never invokes an allocator, I/O, logger, or runtime filter designer.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input before nonlinear functions, choose finite safe output/state on bad values, and document subnormal handling across filter and nonlinear intermediates.

## Primary and official sources

[BILBAO-ADAA] is the peer-reviewed ADAA source. [VAIDYANATHAN-MULTIRATE] supports interpolation/decimation analysis. [AES17-PREVIEW] bounds measurement-method terminology without supplying uninspected normative parameters.

## Fixtures

Use swept sine, near-Nyquist sine, intermodulation multitone, drive/bias steps, zero crossings, ADAA near-singularity vectors if enabled, DC, asymmetric L/R, and high-rate reference renders.

## Objective tests and tolerances

Compare output against a declared oversampled `f64` reference, measure FFT-bin alias energy with a stated band/threshold, check finite results, exact latency/PDC, and scalar repeat identity.

## Rejected alternatives and tradeoffs

Test-only `f64` oversampling and nonlinear equations use separate source/tables/state. CI rejects any import of a production saturator, clipper, ADAA, or oversampler kernel.

## Known gaps and follow-up

Only homogeneous shape/quality programs bank together. SIMD uses the same prepared factor and lane-local state; base Wasm avoids FMA-dependent correctness and scalar tail remains available.

## Benchmark plan

Run exactly two clean native rounds per enabled quality/factor with deterministic multitone and near-Nyquist blocks; emit standard percentile JSON and metadata.

## Listening protocol or evidence

After spectral gates pass, perform blinded gain-matched A/B or ABX against the declared candidate/reference and retain all reveal/randomization details.

## 17. Decision record

Fact: nonlinearities generate aliases and ADAA has nontrivial numerical conditions [BILBAO-ADAA]. Adoption: oversampled reference first, ADAA deferred. Measurable reason: alias spectrum and singularity fixtures. Open question: final launch oversampling factors.
