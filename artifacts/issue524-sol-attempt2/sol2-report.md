# Issue #524 Sol attempt 2 evidence report

Candidate head: `7190a6a3ae6222f97a284237287c640e99ea1c07` (`codex/native-compressor-points`). Final worktree source status was clean. No production transition/DSP/layout/descriptor/payload logic changed in Sol attempt 2; corrections are confined to API/compressor rustdoc and the two existing test files.

## Correction coverage

- `native_parameter_access_is_typed_and_transactional` now reads all eight indices for asymmetric Left/Right values against payload bits; covers all seven writable domains at boundaries and outside/nonfinite values, invalid indices, lookahead, Both and combined precedence, signed zero, a positive subnormal under `CanonicalFpEnv`, rejection payload/continuation invariance, and default-Unsupported forwarding around a real compressor.
- `native_point_changes_target_without_advancing_samples` uses independently prepared, identically warmed compressors; observes both ordered no-sample targets; admits only the selected target/remaining payload words to differ; preserves currents, cursors, detector/rings, other parameters/channel and resident observations; then compares real PCM/report/observation/complete payload against old Point-span processing.
- `native_point_retarget_obeys_current_and_sample_count_laws` uses an independent iterated-f32 oracle at 0/1/17/63/64 samples with remaining words and exact snap, same-target and different-target in-flight restart, stationary cancellation and asymmetric Right invariance.
- `native_point_matches_existing_span_pcm_and_state` covers all seven writable indices and both channels on independently prepared and identically warmed compressors with nonzero delay histories; reports, observations, complete payloads and both PCM planes compare bitwise. Makeup/Left also differs from an identically warmed no-Point reference.
- `native_point_invalidates_silent_fixed_point_without_advancing_it` establishes silent eligibility and compares stationary, moving and rejected-hook continuations with old-span/reference processing without exposing a private flag.
- The installed allocation audit repeats accepted/rejected hooks, readback, invalid reads and real 128-frame empty-span processing 32 times inside the existing measured scope; preparation and buffers remain outside, positive controls remain, and allocation/free totals remain zero.
- Rustdoc records zero-based descriptor index versus ID, units, current/target timing, Unsupported/default and rejection no-op behavior, channel policy, exclusive caller scheduling/ownership, target-transition success versus admission/ack, compressor domains/precedence, and the inherited canonical native FP environment precondition.

## Passing captures

Every label below has `.command.json`, `.stdout`, `.stderr`, and `.status` under `/tmp/issue524-sol2/`.

- `focused-debug-corrected`: 5 passed, status 0. Captured as dirty source over `6ac08c8f`; the identical source was checkpointed as `d384d363`.
- `focused-release`: 5 passed, status 0 at clean `d384d363`.
- `compressor-debug-corrected`: 75 passing entries across 19 result blocks, status 0 at clean `3b9a5fb8`.
- `compressor-release`: 75 passing entries across 19 result blocks, status 0 at clean `3b9a5fb8`.
- `effect-contract-debug` and `effect-contract-release`: 40 passing entries across five result blocks in each profile, status 0 at clean `3b9a5fb8`.
- `allocation-debug`: isolated child and parent each report the one selected exact test passing, status 0 on the conformance-only correction subsequently checkpointed as `3b9a5fb8`.
- `allocation-release`: isolated child and parent each report the one selected exact test passing, status 0 at clean `3b9a5fb8`.
- `clippy-affected-corrected`: strict `-D warnings` for compressor/effect-contract all targets, status 0; identical corrected source was checkpointed as `7190a6a3`.
- `policy-conformance`, `policy-effect-contract`, `policy-effect-runtime`, `policy-lane`, `policy-realtime`, `policy-workspace`: all status 0 at clean `7190a6a3`.
- `wasm-scalar-corrected`: scalar wasm32 release build, status 0 at clean `7190a6a3`.
- `wasm-simd`: simd128 wasm32 check, status 0 at clean `7190a6a3`.
- `fmt-check` and `diff-check`: status 0 at clean `7190a6a3`.

The full native suites ran before the final lint-only seed spelling change. `7190a6a3` changes only hexadecimal grouping (for example `0x5240_11` to the numerically identical `0x0052_4011`); root explicitly ruled that repeated suites were unnecessary for this literal-format-only change. Strict Clippy and all final policy/target/fmt/diff captures use the final hashes.

## Preserved failures and resolutions

- `focused-debug`: status 101, compile-only misuse of `CanonicalFpEnv::enter()` as a Result. Removed `.expect`; `focused-debug-corrected` passed 5/5.
- `compressor-debug`: status 101, test-only redundant zero-frame process rejected with `ZeroFrames`. Removed that call; the existing following 128-frame process already has an empty span slice. `allocation-debug` and `compressor-debug-corrected` passed.
- `clippy-affected`: status 101, twelve `unusual_byte_groupings` diagnostics in test seed literals. Re-grouped the same numeric literals; `clippy-affected-corrected` passed.
- `wasm-scalar`: status 101 before a valid Cargo target compile because the shell split the `RUSTFLAGS` environment value. `wasm-scalar-corrected` passes the assignment as one `env` argument and built successfully.

## Final source identities

| path | SHA-256 | Git blob |
|---|---|---|
| `crates/effect-contract/src/lib.rs` | `48a7b230da74e1a91c4d535f051df8bee58ecf53af0d0bf0b1b8db9e672f9ae3` | `3e87af9c93b4aa1ab052657256047f36721fc26b` |
| `crates/compressor/src/lib.rs` | `b32b49de02d5e0f907cc49485c9f702e9f25c2999a956b41234a9f0ff3346b87` | `a3fea590674b83e1f8ee0b7f07ce3040f5ad4aae` |
| `crates/compressor/tests/native_points.rs` | `d4f402634acbdacbbf7436ae05cf2051543ea7b6ae674e73a578b97b5fe88119` | `87dad726fb287a3626b4616d152e347cc01be8f1` |
| `crates/compressor/tests/conformance.rs` | `a038491a6bab7a38cb8dc0b89c257184d6ffcb5474051c0ed21e78b94b17c2ec` | `505062e5af1eff876a3f9a77344c76c43712ffb3` |
