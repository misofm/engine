# Issue #524 final Sol attempt 3 report

Candidate head: `d8c14721bbde58646ab837b0901f79bc8e08799f` on `codex/native-compressor-points`; tracked source clean. Root-owned `artifacts/issue524-sol-attempt3/` packaging appeared concurrently after the final gates and is outside this agent's source tranche.

## Bounded correction

Only `crates/compressor/tests/native_points.rs` changed. Inside `native_parameter_access_is_typed_and_transactional`, the existing finite table now also asserts:

- `u32::MAX` plus `Both` read and NaN apply reject as `InvalidParameterIndex` and preserve both payload sections;
- index 7 plus `Both` plus NaN rejects as `InvalidChannel`, completing index → channel → automatable → domain precedence;
- an equal-channel prepared compressor rejects a legal index-5 `Both` apply, preserves both complete payloads, and matches an independently prepared continuation;
- the forwarding wrapper's inherited defaults reject a legal automatable index-5/Left apply and readable state request as `Unsupported`, in addition to the retained malformed-argument cases, under the same complete snapshot and continuation witness.

All Sol2 assertions and the other four exact fixtures are unchanged. Production code, API/docs, DSP/layout/descriptors/payload, allocation fixture and support helpers are unchanged.

## Final attempt captures

Each label has `.command.json`, `.stdout`, `.stderr`, and `.status` under `/tmp/issue524-sol3/`.

- `focused-debug`: 5 passed, status 0; captured as the sole dirty file over `1dc01a20`, then checkpointed unchanged as `d8c14721`.
- `focused-release`: 5 passed, status 0 at clean `d8c14721`.
- `clippy-affected`: compressor/effect-contract all targets with `-D warnings`, status 0 at clean `d8c14721`.
- `fmt-check`: status 0 at clean `d8c14721`.
- `diff-check`: status 0 at clean `d8c14721`.

## Explicit prior-evidence reuse

Astra's Sol2 verdict and root's Sol3 authorization explicitly rule that no full-suite or target repeat is warranted for this typed-access-table-only correction. The prior raw/package evidence in `artifacts/issue524-sol-attempt2/` remains the evidence for unchanged production and other test binaries:

- complete compressor debug/release: 75 passing entries across 19 result blocks in each profile;
- effect-contract debug/release: 40 passing entries across five result blocks in each profile;
- isolated allocation gate debug/release, including the 32 repeated hook/read/rejection/real empty-span-block sequence and live positive controls;
- six policy checks;
- scalar wasm32 release build and simd128 wasm32 check.

The only changed test binary is directly covered by Sol3 focused debug/release (5 each) and strict all-target Clippy. Effect-contract source, compressor production source and the conformance allocation test have the same final hashes as Sol2.

## Final source identities

| path | SHA-256 | Git blob |
|---|---|---|
| `crates/effect-contract/src/lib.rs` | `48a7b230da74e1a91c4d535f051df8bee58ecf53af0d0bf0b1b8db9e672f9ae3` | `3e87af9c93b4aa1ab052657256047f36721fc26b` |
| `crates/compressor/src/lib.rs` | `b32b49de02d5e0f907cc49485c9f702e9f25c2999a956b41234a9f0ff3346b87` | `a3fea590674b83e1f8ee0b7f07ce3040f5ad4aae` |
| `crates/compressor/tests/native_points.rs` | `e931976489438af2f6a5d1e6d67d53214723da38c70c5c89cffe2dfd895abbe6` | `8641fcdc654ef49dfd1e189290afe95205eb440e` |
| `crates/compressor/tests/conformance.rs` | `a038491a6bab7a38cb8dc0b89c257184d6ffcb5474051c0ed21e78b94b17c2ec` | `505062e5af1eff876a3f9a77344c76c43712ffb3` |
