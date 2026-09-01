# Delay

## Scope and engineering question

Use prepared bounded circular delay lines for dual-mono and explicit cross-channel/stereo delay. Integer delay is exact; a future fractional mode must explicitly name and validate its interpolation.

## Algorithm and equations

For integer delay `D`, write `x[n]` to a circular ring and read `x[n-D]`; feedback is `w[n] = x[n] + g*y[n-D]`. Fractional delay, when added, states its interpolation equation and magnitude/phase error [SMITH-SASP].

## Coefficients and update rules

Allocate for the prepared maximum delay. Delay-time/feedback changes use an explicit bounded transition strategy; no render-time resizing or unspecified discontinuity hiding is allowed.

## Numerical and stability limits

Delay frames are bounded by prepared capacity; feedback gain has a finite declared domain. Sanitize bad feedback state and define a finite maximum tail/reset policy.

## Latency and tail

The signal-path latency is declared from the selected read path; feedback tail is state-dependent and must have an explicit energy-threshold/maximum-duration reporting policy. Bypass preserves graph PDC.

## Units, mappings, automation and smoothing

Delay time is samples and/or ms with unambiguous conversion at session rate; feedback and wet/dry gain are linear or dB as metadata says. Smoothing/transition policy is a parameter contract.

## Definitions and assumptions

Dual-mono means independent rings by default. A stereo/cross-feedback mode explicitly names the 2x2 feedback/routing matrix and any smoothed coefficients.

## Adopted decisions

Rings, transition heads, and temporary output capacity are allocated during preparation. Render performs bounded index arithmetic and no allocation, locking, I/O, or unbounded work.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input and state before feedback writes; clear bad ring/state deterministically. Apply the global subnormal policy to feedback values.

## Primary and official sources

[SMITH-SASP] supports delay-line and interpolation analysis. [LAWO-FLOW] documents channel delay as an explicit console control pattern; it is workflow, not DSP proof. [VST3-LATENCY] supports explicit latency reporting.

## Fixtures

Use impulses at minimum/maximum delay, delay-time transitions, feedback decay, exact integer alignment, fractional tests if enabled, cross-feedback asymmetry, bypass, silence, and non-finite input.

## Objective tests and tolerances

Assert exact integer impulse location, bounded tail/reset, finite output/state, declared latency, dual-mono independence, and reference agreement for any fractional interpolation.

## Rejected alternatives and tradeoffs

The test reference has a separate `f64` circular buffer and interpolation implementation. It does not import production ring or processor code.

## Known gaps and follow-up

Delay is track-local and may bank homogeneous programs, while lane state/ring heads remain separate. Scalar tails are mandatory and any vector gather strategy must preserve reference semantics.

## Benchmark plan

Benchmark zero, moderate, and maximum prepared delay with feedback on deterministic blocks; emit standard two-round percentile JSON and full host metadata.

## Listening protocol or evidence

Record blinded comparison of time-change/cross-feedback modes only after exact delay and finite-tail gates pass.

## 17. Decision record

Fact: delay needs bounded history state [SMITH-SASP]. Adoption: prepared circular buffers and explicit transition policy. Measurable reason: exact impulse alignment and memory ceiling independent of stem duration. Open question: selected fractional interpolation family.
