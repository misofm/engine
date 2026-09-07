# Issue 542 final integrated qualification

- Candidate: `f9c279075d22fb66640df4d7cb669f7e82c62f47` (merge checkpoint `2a3c96a578c5…`, main `b95c9b7b028e…`)
- Cwd: `/home/bl/misofm/engine-protocol-conformance-boundary`
- Qualified: 2026-09-07 UTC; native target `x86_64-unknown-linux-gnu`; Wasm target `wasm32-unknown-unknown`; `wasm-interp 1.0.34`
- Verdict: **PASS**. Product source remained unchanged; only this uncommitted evidence directory was added.

## Gates

Every `.meta` file records contemporaneous UTC/cwd/HEAD/status/source hashes, the exact command, and numeric exit. Every `.raw.gz` is the lossless combined command output.

| Gate | Exit | Evidence |
|---|---:|---|
| Native corpus (46 frames, typed decoders, deep dispatch) | 0 | `native-corpus-debug.raw.gz` |
| Deterministic million mutations, debug + release | 0 | `million-mutation-{debug,release}.raw.gz` |
| Protocol default + `test-support` | 0 | `protocol-{default,test-support}-test.raw.gz` |
| Conformance full suite, debug + release | 0 | `conformance-{debug,release}-test.raw.gz` |
| Scalar/SIMD Wasm parity and required self-test rows | 0 | `wasm-parity-self-test.raw.gz` |
| Boundary/control/workspace checks | 0 | `boundary-positive.raw.gz`, `control-policy.raw.gz`, `workspace-policy.raw.gz` |
| Forbidden default export counterexample + byte-exact restore | 0 (control) | `boundary-counterexample-corrected.{meta,raw.gz}`; mutated checker exit 1; before/after SHA256 identical |
| Default protocol build, normal dependency/API absence | 0 | `protocol-default-build-dependency-api.raw.gz` |
| Bench consumer compile | 0 | `bench-consumer-check.raw.gz` |
| Strict Clippy + rustdoc | 0 | `strict-clippy-affected.raw.gz`, `rustdoc-affected.raw.gz` |
| Extracted tests and ignored census | 0 | `extracted-test-population-corrected.raw.gz`; controller 44, message wire 17, session wire 19; each ignored 0 |
| Format and base-to-head diff checks | 0 | `fmt-diff-check.raw.gz` |
| Exact path census | 0 | `exact-path-census.raw.gz` |

Wasm artifact identities from the actual parity build:

- scalar: 2,734,565 bytes, SHA256 `c881d6288a6d130c25643a0198ad6e5c0086a70f95a98f5c6fa21365109e5240`
- simd128: 2,728,061 bytes, SHA256 `765b42cc97612d79e9da775771485e1cb7b08e1d1b3c425c9109c2fef8bf98b5`

The first malformed counterexample probe and first overstrict artifact-string probe are retained as `boundary-counterexample.*` and `wasm-artifact-identities.*`; both were corrected without product edits and do not affect the PASS verdict.
