# Issue 524 Luna attempt 1 evidence

Source checkpoint `fb6d2d24617412ba5b946a04a9a07f3c524456a4`; source hashes accompany each command. Initial formatted focused tests select 5 passing tests in each profile. The existing allocation test passes. Remaining compressor suites contain 19 harnesses / 75 passed each in debug and release; effect-contract contains 5 / 40 passed each. Strict affected Clippy, six existing policies, formatting/diff and scalar/SIMD Wasm compilation pass.

`wasm-scalar` is an incomplete preflight capture: the helper attempted to execute an environment assignment as argv[0] and failed before Cargo launched; no workload status exists. It is preserved, not treated as a compile failure or success. Corrected `wasm-scalar-fixed` and `wasm-simd-fixed` use `env` explicitly and record successful child exit status. No unrecorded initial SIMD attempt is claimed.

These are execution results, not an assertion-coverage verdict. Astra reviews the frozen numbered behavioral requirements separately. No timing, new listening result, algorithm change, parent closure or protocol-delivery capability is claimed.
