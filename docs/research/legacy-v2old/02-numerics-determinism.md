<!--
Provenance: copied from misofm/engine-v2-old docs/research/02-numerics-determinism.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Numerics and determinism

V0.1 exactness means native AVX2 and browser Wasm SIMD produce identical bits for accepted finite input and the declared partitions. It uses IEEE round-to-nearest, ties-to-even behavior ([IEEE 754-2019](https://doi.org/10.1109/IEEESTD.2019.8766229)); Rust’s `f64` behavior remains a language reference, not a license to vary operation ordering ([Rust f64](https://doc.rust-lang.org/stable/std/primitive.f64.html)).

Rules:

- Evaluate separate multiply then add; never `mul_add`, FMA, contraction, fast-math, relaxed SIMD, approximate reciprocal/rsqrt, target libm, reassociation, or horizontal contributor folds.
- Own constants, polynomial/order choices, and summation order. Contributor order is fixed by the mounted plan, never hash or physical traversal order.
- Reject or canonicalize non-finite ingress before it reaches state; finite subnormals are accepted. Any software `+0` state flush has an exact threshold and identical dataflow position on both targets; it never substitutes for FP-environment enforcement.
- At every native render-thread entry/start, require and attest round-to-nearest-even with FTZ/DAZ disabled. If the state is wrong or cannot be attested, refuse before audio with stable `fp_environment_invalid`; test/debug may check per process where thread entry is absent. Wasm already uses its specified RNE/subnormal behavior.
- Pin the Rust and target flags; audit generated behavior and reject settings permitted by [LLVM fast-math](https://llvm.org/docs/LangRef.html) that violate the profile.

Wasm numeric execution is specified, but APIs must avoid relaxed operations and unowned transcendental behavior ([Wasm numerics](https://webassembly.github.io/spec/core/exec/numerics.html), [core spec](https://www.w3.org/TR/wasm-core/)). `wide::f64x4::mul_add` is excluded because its behavior can use hardware FMA; emulating correctly rounded FMA in browser WASM is intentionally rejected as too complex and slow for launch ([wide f64x4 1.6.1](https://docs.rs/wide/1.6.1/wide/struct.f64x4.html)).

Exactness is a versioned posture, not a promise that every future quality mode or platform shares V0.1 arithmetic. Gates cover partitions 1/7/64/128/512, subnormal input/state/result, occupancy, tails, event timing, and serialized state.
