# Loudness measurement

## Scope and engineering question

Implement loudness metering as a measurement contract separate from gain control and limiting. Use BS.1770 K-weighting, channel weighting, gating, and declared measurement windows where applicable.

## Algorithm and equations

The meter filters channels with the standard K-weighting cascade, applies channel weights, forms block energies, then applies BS.1770 absolute/relative gating for integrated measurement [ITU-BS1770-5].

## Coefficients and update rules

K-weighting coefficients derive at the declared sample rate according to the standard's rate handling. Block/window updates occur at documented sample boundaries; no unspecified continuously moving integration state is implied.

## Numerical and stability limits

Accumulate energy in `f64` in the offline reference and use a documented finite accumulator strategy in production. Silence has a defined report value/state rather than taking log of zero.

## Latency and tail

Meters do not alter audio and therefore add zero audio latency/tail. Their reporting window and post-stop finalization behavior are separately declared telemetry semantics.

## Units, mappings, automation and smoothing

Report LUFS/LKFS and true-peak dBTP where enabled; windows and gates follow the named standard/version. Meter display smoothing is presentation-layer metadata and must not modify measurement values.

## Definitions and assumptions

Measure a declared graph boundary and channel layout. Track dual-mono meters retain distinct L/R observations; any combined measure names its channel weighting and routing boundary.

## Adopted V2 decisions

Prepared bounded window/ring state and counters are used. Audio render only updates bounded meter state; report formatting and serialization stay off the callback.

## Denormal, signed-zero and NaN policy

Sanitize non-finite samples before energy accumulation, increment a telemetry counter, and never emit non-finite measurements. Subnormal samples participate as finite values or are handled by the global input policy.

## Primary and official sources

[ITU-BS1770-5] is the primary measurement algorithm. [EBU-R128] specifies the associated loudness-normalisation practice and terminology. [AES17-PREVIEW] bounds the role of digital-audio measurement methodology.

## Fixtures

Use standard-derived K-weighting/energy vectors, silence, DC, channel-asymmetric streams, gated passages, long-duration synthetic blocks, and non-finite sample injections.

## Objective tests and tolerances

Validate K-weighting response and integrated/short-term values against an independent `f64` reference and published/derived vectors; assert meter transparency to PCM and finite outputs.

## Rejected alternatives and tradeoffs

Reference weighting and gating are separately coded from the standard in test-only `f64`; it imports no production meter filters, coefficient routines, or aggregation types.

## Known gaps and follow-up

PCM weighting can be lane-parallel but reductions use deterministic channel/node order. Browser/mobile reduced fixtures preserve exact test semantics; no FMA-specific result is assumed.

## Benchmark plan

Measure PCM-through-meter overhead in fixed blocks, recording the same percentile and machine metadata schema as every launch kernel.

## Listening protocol or evidence

N/A — loudness metering is non-audio-transforming; validate numerically and record listening only when a later gain-control feature uses its output.

## 17. Decision record

Fact: BS.1770 defines the measurement method [ITU-BS1770-5]. Adoption: keep measurement separate from limiter behavior. Measurable reason: PCM equality before/after a meter. Open question: which telemetry cadence is useful to agents.
