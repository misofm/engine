<!--
Provenance: copied from misofm/engine-v2-old docs/research/04-filters-and-smoothing.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Filters and smoothing

The stock EQ begins with a deterministic coefficient path and per-track f64 recursive state. Candidate cookbook biquads derive from RBJ’s published equations ([Audio EQ Cookbook](https://www.w3.org/TR/audio-eq-cookbook/)); any alternative state-variable implementation must be benchmarked and bit-gated, informed by Cytomic’s trapezoidal SVF treatment ([paper](https://cytomic.com/files/dsp/SvfLinearTrapezoidalSin.pdf)). Coefficients are generated in mount/control preparation, never by an unowned target math library during process.

Parameter descriptors declare units, min/max, scale, default, enum domain, smoothing law, smoothing time/range, mutation lifecycle, and dependencies (for example frequency relative to the immutable session rate and Q limits). Mount derives sample-time conversions and coefficients from `sample_rate_hz`, refusing unsafe values; a ramp is preallocated state, triggered by numeric events. Bypass policy is explicit: hard bypass, smoothed gain crossfade, or state-preserving behavior; no effect chooses silently.

Interpolation is an audible and stability-sensitive policy, not UI decoration. The implementation program evaluates coefficient/state interpolation using the practical review in [DAFx 2006](https://www.dafx.de/paper-archive/2006/papers/p_057.pdf). Launch favors a conservative, deterministic ramp that meets exactness and stability gates before pursuing aggressive coefficient morphing.

Before implementation, Sol approves an independent objective EQ oracle: analytic response plus frequency/Q/gain error, stability/extremes, neutral/noise/distortion, and smoothing-transition tests with derived tolerances. Acceptance tests include DC, impulse, sweep, step/ramp, extreme valid parameters, bypass transitions, and partition invariance. Non-finite parameter values are rejected on the control path. No sound-quality or class-leading claim is made before oracle receipts.
