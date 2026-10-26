# Add a native stereo reverb for the console's aux return

Filed by root for the console sprint (2026-10-26). The owner chose "Build an engine reverb" for the console's Aux B return, which reopens decision 13 O2 (the owner approved this).

## Product outcome
A native, high-quality stereo reverb effect (`miso.reverb`) that the console uses as an aux return (100 % wet), with a curated set of controls: for example pre-delay, decay time, size, damping (high and low), diffusion, width and mix. It has the full native-effect contract (AGENTS.md "Effects and plugins"): stable parameter IDs, units, domains, mappings, defaults, smoothing, live versus rebuild classes, latency (zero or declared), tail, quality modes, reset semantics and NaN/denormal behaviour, a bank kernel if it is eligible, and SIMD on every target with no target-specific code.

## Research first (AGENTS.md "Realtime, quality, and research evidence")
The algorithm choice needs primary sources: feedback delay networks (Jot and Chaigne; Schlecht and Habets on FDN design), the Dattorro plate (J. Audio Eng. Soc. 1997), and allpass diffusion and modulation. Compare quality, CPU cost, the tail bound and the stability proof. Write a dsp-research note in the template before any code.

## Gates (to be set by the scope note)
Equations, coefficient rules, the stability and tail proof, fixtures, objective tests (decay time against the RT60 setting, echo density, stereo decorrelation, no metallic resonance in the modal density), benchmarks and a listening plan.
