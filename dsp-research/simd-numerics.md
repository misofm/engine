# SIMD numerical rules

## Scope and engineering question

Use scalar semantics as the correctness baseline, with AoSoA banks of four Wasm/NEON lanes or eight AVX2 lanes and scalar tails. SIMD optimizes prepared homogeneous work; it does not change audio semantics.

## Algorithm and equations

Vector lanes evaluate the same lane-local equations as scalar. Base Wasm computes `a*b + c` as multiply then add because fused behavior is not required; native FMA is a separate runtime-dispatched implementation [WASM-SIMD] [INTEL-INTRINSICS].

## Coefficients and update rules

Compile cohort signatures (slot type/order, quality, compatible routing) before render. Parameter targets and state remain per lane; absent slots are identity kernels and incompatible tracks use another cohort or scalar/dynamic fallback.

## Numerical and stability limits

Finite input and parameter policy is common across backends. Scalar repeated renders are byte-identical; SIMD/scalar uses the effect-declared tolerance, initially for linear kernels `abs <= 1e-6 + 2e-5*max(abs(reference), abs(actual))` unless a stricter fixture states otherwise.

## Latency and tail

Backend selection cannot alter prepared effect latency or tail. Bypass and scalar fallback retain identical graph PDC semantics.

## Units, mappings, automation and smoothing

SIMD introduces no user-facing units. Effect metadata remains authoritative for parameter domains, event rate, smoothing, and quality; dispatch capability is diagnostic metadata.

## Definitions and assumptions

L and R occupy separate vectors/state. Cross-channel behavior occurs only through declared link mode or smoothed 2x2 matrix; vector adjacency never creates a signal link.

## Adopted V2 decisions

Prepare cohorts, buffers, and dispatch choice before render. The callback does not feature-probe, allocate, lock, log, or alter structure; its work is bounded by quantum and prepared plan capacity.

## Denormal, signed-zero and NaN policy

Every backend performs the same finite-input/state policy. Fixtures inject NaN/infinities and subnormals and assert finite sanitized outputs plus deterministic reset/counters.

## Primary and official sources

[WASM-SIMD] and [WASM-CORE] define Wasm SIMD/floating semantics. [INTEL-INTRINSICS] documents native intrinsic availability. [RUST-WASM-TARGET] bounds the supported browser target environment.

## Fixtures

Run deterministic scalar/SIMD full-bank and tail fixtures with impulse, random seeded finite blocks, automation, asymmetric dual-mono, subnormal, and non-finite cases at required rates.

## Objective tests and tolerances

Assert scalar repeat byte identity, backend/scalar declared tolerance, exact PDC, finite output, cohort identity-slot behavior, and deterministic reduction order. Record unavailable backend as skipped, not passed.

## Rejected alternatives and tradeoffs

`f64` references test equations independently of production kernels. SIMD comparison additionally uses scalar production only as a backend differential test and never substitutes it for the reference model.

## Known gaps and follow-up

AVX2 and FMA have separate runtime dispatch; AArch64 NEON/Wasm use four lanes; AVX2 uses eight; scalar supports all counts. Browser launch remains single render thread unless separately proven.

## Benchmark plan

For scalar and each enabled backend run two clean baseline rounds at 48/96 kHz, 64/256 frames, full cohort and tail; emit percentiles, cycles when available, and complete JSON environment metadata.

## Listening protocol or evidence

N/A — SIMD is an implementation strategy; numerical backend equivalence is the acceptance evidence. Any audible comparison belongs to the owning effect record.

## 17. Decision record

Fact: Wasm does not require fused multiply-add semantics [WASM-SIMD]. Adoption: non-fused base Wasm and separate FMA dispatch. Measurable reason: scalar/SIMD tolerance fixtures. Open question: future relaxed-SIMD opt-in evidence.
