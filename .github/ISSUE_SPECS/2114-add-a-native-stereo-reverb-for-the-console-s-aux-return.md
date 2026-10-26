# Add a native stereo reverb for the console's aux return

Filed by root for the console sprint (2026-10-26). The owner chose "Build an engine reverb" for the console's Aux B return, which reopens decision 13 O2 (the owner approved this).

## Product outcome
A native, high-quality stereo reverb effect (`miso.reverb`) that the console uses as an aux return (100 % wet), with a curated set of controls: for example pre-delay, decay time, size, damping (high and low), diffusion, width and mix. It has the full native-effect contract (AGENTS.md "Effects and plugins"): stable parameter IDs, units, domains, mappings, defaults, smoothing, live versus rebuild classes, latency (zero or declared), tail, quality modes, reset semantics and NaN/denormal behaviour, a bank kernel if it is eligible, and SIMD on every target with no target-specific code.

## Research first (AGENTS.md "Realtime, quality, and research evidence")
The algorithm choice needs primary sources: feedback delay networks (Jot and Chaigne; Schlecht and Habets on FDN design), the Dattorro plate (J. Audio Eng. Soc. 1997), and allpass diffusion and modulation. Compare quality, CPU cost, the tail bound and the stability proof. Write a dsp-research note in the template before any code.

## Gates (to be set by the scope note)
Equations, coefficient rules, the stability and tail proof, fixtures, objective tests (decay time against the RT60 setting, echo density, stereo decorrelation, no metallic resonance in the modal density), benchmarks and a listening plan.

## Decision record
- 2026-10-26, owner: reopened decision 13 O2 (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`): build the reverb.
- 2026-10-26, root (accepting the scope note, `scratchpad/opt/scope/2114.md`, which wins for paths and decisions): `miso.reverb` is a 16-line feedback delay network with a Hadamard-based orthogonal matrix, a per-line TPT low-pass (damping, cutoff capped at `min(19,845 Hz, fs/4)` as #1374) and a first-order high-pass, absorption gains for RT60, four Schroeder allpass diffusers per channel outside the loop, pre-delay, two orthogonal output patterns and a smoothed M/S width matrix, with slow modulation on 4 lines as a measured design constant (kept only if the research measurement favours it). Latency 0; finite certified tail (#1374's method, roundings counted); insert-only, never banked (`NEVER_BANKED_EFFECTS`); not console-eligible.
- Controls (all `Shared`, block-rate, live, automatable; no rebuild row): 1 `pre_delay_ms` 0-250 (20), 2 `decay_ms` 100-20,000 log (1,800), 3 `size` 0.25-2.0 (1.0), 4 `damping_hz` 1,000-20,000 log (6,000), 5 `low_cut_hz` 20-500 log (20), 6 `diffusion` 0-1 (0.7), 7 `width` 0-1 (1.0), 8 `mix` 0-1 (1.0).
- Owner questions, answered by root 2026-10-26: Q1 one continuous design, no mode enum; Q2 no freeze, decay capped at 20 s (the tail proof needs every loop gain below 1); Q3 these eight controls are the curated set.
- Slices: S1 research note; S2 kernel and f64 oracle; S3 descriptor, registry, tail derivation, never-banked list, schema doc; S4 remaining tests, corpus, qualification rows, benchmark, listening; S5 SDK and metadata. Attempt 1 covers S1-S3.

## Evidence
