# Oversampling

## Scope and engineering question

Treat oversampling as an explicitly designed multirate pipeline for effects that need it. Do not add implicit sample-rate conversion to the session engine.

## Algorithm and equations

For factor `L`, interpolate by zero insertion followed by FIR `hI`, process at `L*Fs`, then filter with decimation FIR `hD` and retain every `L`th sample. Each stage declares passband, ripple, stopband attenuation, and group delay [VAIDYANATHAN-MULTIRATE].

## Coefficients and update rules

Filter coefficients and polyphase layout are prepared before render. Quality-mode changes compile a replacement plan; render never redesigns filters or changes factors structurally.

## Numerical and stability limits

Specify coefficient precision, finite input bounds, accumulator behavior, and alias/response acceptance thresholds per use. Reject factors/filter tables not prepared for the active sample rate.

## Latency and tail

Declare total integer output-sample group delay and FIR tail from the actual stages. Bypass retains the declared effect latency through PDC or an equivalent delay path.

## Units, mappings, automation and smoothing

Factor is a discrete quality enum; passband/stopband edges are Hz or normalized frequency and filter delay is samples. Switching quality is structural, not a sample-by-sample smoothed control.

## Definitions and assumptions

L/R have independent multirate histories unless a named processor requires a link. Oversampling itself does not route or mix channels.

## Adopted V2 decisions

All delay lines, polyphase state, scratch, and output capacity are preallocated for quantum and factor. Render has fixed per-block bounds and no allocation/I/O.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input before FIR state, clear non-finite histories, and apply the engine's subnormal policy to state and intermediates.

## Primary and official sources

[VAIDYANATHAN-MULTIRATE] supports interpolation/decimation and polyphase design. [ITU-BS1770-5] Annex 2 gives an official true-peak interpolation context. [SMITH-SASP] supports FIR/delay-line analysis.

## Fixtures

Use impulse, swept sine, near-Nyquist sine, multitone, DC, silence, factor transitions at plan boundaries, and asymmetric L/R streams with a high-rate `f64` reference.

## Objective tests and tolerances

Measure declared group delay, passband/stopband response, reconstruction error, finite output, and bypass-PDC equality. Each use declares its own spectrum/error tolerance.

## Rejected alternatives and tradeoffs

The test reference builds independent `f64` convolution/polyphase state and filter tables; it imports neither production coefficient tables nor production kernels.

## Known gaps and follow-up

Vectorize taps/lanes only with fixed prepared bounds; scalar tail remains supported. Wasm SIMD uses non-fused arithmetic where required, while AVX2/FMA is feature-dispatched.

## Benchmark plan

Benchmark every enabled factor using deterministic near-Nyquist and multitone inputs in two clean baseline rounds, emitting standard JSON percentiles and metadata.

## Listening protocol or evidence

Use blinded comparisons only after spectrum and latency gates show a material candidate difference; record oversampling factor, filters, and gain matching.

## 17. Decision record

Fact: interpolation and decimation filters determine response and delay [VAIDYANATHAN-MULTIRATE]. Adoption: explicit prepared stages. Measurable reason: impulse/spectrum fixtures expose their costs. Open question: launch factor set per nonlinear effect.
