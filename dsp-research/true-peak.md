# True peak

## Scope and engineering question

Define true peak as an inter-sample measurement/limiting requirement with declared interpolation accuracy and latency. It is not synonymous with sample peak or loudness.

## Algorithm and equations

Sample peak is `max(abs(x[n]))`; true-peak estimation evaluates an interpolated waveform at higher temporal resolution using a declared FIR/polyphase method, as motivated by BS.1770 Annex 2 [ITU-BS1770-5].

## Coefficients and update rules

Prepare interpolation filters and lookahead gain-control state for the sample rate/quality. Ceiling and release changes use declared bounded smoothing; oversampling factor changes are structural.

## Numerical and stability limits

Ceiling and detector values must be finite; clamp gain to a finite safe interval. State a maximum permitted estimated overshoot relative to the chosen reference and test near-Nyquist cases.

## Latency and tail

Limiter lookahead plus interpolation group delay is a fixed integer latency reported to PDC. Bypass preserves it; release/history have a documented finite reset/tail policy.

## Units, mappings, automation and smoothing

Ceiling and measured output use dBTP; threshold/makeup use dB; attack/release use ms; quality/factor are discrete enums. Metadata names all smoothing rules.

## Definitions and assumptions

Per-channel true-peak detection is default. Any linked detector mode is explicit and declares max/average weighting; it does not imply an unannounced stereo topology.

## Adopted V2 decisions

Prepared bounded delay, FIR, detector, and gain state only. The render loop has no allocation, I/O, or runtime filter construction.

## Denormal, signed-zero and NaN policy

Sanitize non-finite samples before interpolation; reset bad detector/gain state to safe unity or ceiling-protecting state as specified. Avoid log/ratio operations on zero.

## Primary and official sources

[ITU-BS1770-5] supplies true-peak interpolation context. [EBU-R128] supplies loudness/maximum-level practice. [VAIDYANATHAN-MULTIRATE] supports the multirate filter analysis.

## Fixtures

Use inter-sample-peaking near-Nyquist sine phases, impulse, multitone, ceiling steps, lookahead impulses, linked/unlinked asymmetric channels, bypass, and non-finite tests.

## Objective tests and tolerances

Compare estimated peaks and gain trajectory with independent `f64` interpolation, verify exact latency/PDC and finite output, and state effect-specific overshoot and error tolerances.

## Rejected alternatives and tradeoffs

The reference implements a separate `f64` interpolation/filter/gain chain and never imports production oversampler, limiter, or coefficient code.

## Known gaps and follow-up

Bank lanes only for homogeneous limiter quality/latency. SIMD/scalar differences use declared tolerance; base Wasm must not rely on relaxed SIMD or fused rounding.

## Benchmark plan

Measure peak-heavy deterministic inputs at each enabled quality using the fixed two-round benchmark protocol and record full metadata.

## Listening protocol or evidence

Use blinded, ceiling-matched comparison of distortion behavior only after numerical peak/latency gates pass, with listener/facilitator blinding recorded.

## 17. Decision record

Fact: sample peak can under-read inter-sample maxima [ITU-BS1770-5]. Adoption: declared interpolation and fixed lookahead latency. Measurable reason: phase-swept peak fixtures. Open question: launch error budget by quality mode.
