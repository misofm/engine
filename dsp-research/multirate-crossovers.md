# Multirate and crossover design

## Scope and engineering question

Any multiband effect must declare crossover topology, response, phase behavior, reconstruction test, and delay. It may not call bands “transparent” without summed-response evidence.

## Algorithm and equations

An analysis bank splits `x` into bands `bi = Hi{x}`; a unity processing reconstruction checks `sum(bi)` against the delayed input. Multirate stages use explicit interpolation/decimation FIRs and polyphase forms [VAIDYANATHAN-MULTIRATE].

## Coefficients and update rules

Crossover frequencies, orders, filter tables, and quality are prepared/validated at plan compile. Structural topology or band-count changes create a new plan; automation has a stated transition/smoothing rule.

## Numerical and stability limits

Frequency ordering and Nyquist margins are validated. State passband ripple, stopband attenuation, phase/group delay, sum-error, and finite accumulator limits for the selected topology.

## Latency and tail

Declare all crossover and multirate group delay in integer output samples, plus any filter tail. A unity/bypass path preserves that latency via PDC.

The launch two-band multiband compressor has latency 0 and states a certified tail and exact-rest bound per rate (#1373): `T` = 3,810 samples and `R` about 12.2 M samples at 48 kHz (the smoother words' rest), over every crossover in 80 Hz to 8 kHz and any 64-sample crossover glide in flight (`docs/derivations/1373-multiband-compressor-tail-and-rest.md`).

## Units, mappings, automation and smoothing

Crossover points are Hz; orders and quality are discrete; band gains/thresholds use declared units. Frequency movement either names a stable smoothing scheme or is structural-only.

## Definitions and assumptions

Each L/R path has its own filter histories. A cross-band or detector link is explicit; bands do not silently mix L/R channels or alter route topology.

## Adopted decisions

All band buffers, delay alignment, polyphase state, and scratch are bounded/preallocated. Render has fixed work for configured bands and no callback allocation or I/O.

## Denormal, signed-zero and NaN policy

Sanitize non-finite input before analysis filters, reset non-finite state, and apply the finite/subnormal policy to every band history.

## Primary and official sources

[VAIDYANATHAN-MULTIRATE] provides filter-bank/multirate analysis. [SMITH-SASP] provides filter and delay-line treatment. [ORFANIDIS-ISP] is a cross-check for filter response/stability concepts.

## Fixtures

Use impulse, log sweep, multitone, phase-sensitive sine, all-bands-unity reconstruction, per-band impulses, crossover moves, near-Nyquist, and asymmetric dual-mono fixtures.

## Objective tests and tolerances

Verify summed unity reconstruction against delayed reference within named tolerance, measure band response/crossover nulls, exact PDC, finite state, and scalar repeat identity.

## Rejected alternatives and tradeoffs

Independent `f64` analysis/synthesis filter-bank code owns separate coefficients/state and has no dependency on production crossover or multiband kernels.

## Known gaps and follow-up

Bank tracks with identical band topology/quality only. Process bands with deterministic reduction order; scalar tail is required and base Wasm makes no FMA rounding assumption.

Live bypass crossfade (#1903, accepted). The multiband compressor's bypass crossfade is the one launch case whose click exceeds a mute's of the same length: on mix material its HF splatter is 0.8 to 2.3 dB above the mute reference at every crossfade length from 2 to 20 ms and at every launch rate, and every other effect, material and measure stays under the reference (`docs/handoffs/control-smoothing-defaults/FINDINGS.md`, 9.4). The click is still small: at 10 ms the mix click is 2.7 to 3.6 dB under the click of a 5 ms mute. The cause is the crossover, not the compression. With no gain reduction the effect's output is the second-order Butterworth all-pass of its input (`low + high = ap`, `crates/multiband-compressor/tests/lr4_two_section_mapping_f64.rs`), and that all-pass has a phase of 180 degrees at the crossover (1 kHz by default). The bypass shunt's dry path is the input delayed by the declared latency, so near the crossover `wet - dry` is up to twice the programme; a run with both ratios at 1 gives the same excess. A longer crossfade does not remove a phase difference. Phase-matching was not adopted. To run the dry path through the all-pass only during the fade moves the phase step to the end of the fade, where the settled dry output must be the raw input (the bypass null), and the step is then a hard discontinuity. To keep the all-pass on the bypassed output permanently breaks the bypass null and the identity gates. To glide the crossover to a band edge during the fade is a new class-B design with its own transition artefacts. The owner accepted the margin on 2026-10-25 (option 1). Reopen on a listening complaint about the multiband bypass, or a product need for a phase-matched bypass.

## Benchmark plan

Benchmark configured band counts/topologies on impulse and multitone blocks in exactly two clean baseline rounds, using common JSON percentiles and metadata.

## Listening protocol or evidence

Use blinded evaluation only after unity reconstruction and response gates pass; log crossover topology, gain match, and band settings.

## 17. Decision record

Fact: filter-bank design trades response, phase, and delay [VAIDYANATHAN-MULTIRATE]. Adoption: explicit topology/reconstruction contract. Measurable reason: summed impulse/sweep fixtures. Open question: launch topology set.
