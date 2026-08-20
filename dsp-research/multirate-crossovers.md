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

## Units, mappings, automation and smoothing

Crossover points are Hz; orders and quality are discrete; band gains/thresholds use declared units. Frequency movement either names a stable smoothing scheme or is structural-only.

## Definitions and assumptions

Each L/R path has its own filter histories. A cross-band or detector link is explicit; bands do not silently mix L/R channels or alter route topology.

## Adopted V2 decisions

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

## Benchmark plan

Benchmark configured band counts/topologies on impulse and multitone blocks in exactly two clean baseline rounds, using common JSON percentiles and metadata.

## Listening protocol or evidence

Use blinded evaluation only after unity reconstruction and response gates pass; log crossover topology, gain match, and band settings.

## 17. Decision record

Fact: filter-bank design trades response, phase, and delay [VAIDYANATHAN-MULTIRATE]. Adoption: explicit topology/reconstruction contract. Measurable reason: summed impulse/sweep fixtures. Open question: launch topology set.
