# Issue 496 integrated qualification

Frozen source94e1f3c3 passes workspace tests, scalar/SIMD Wasm target checks, native C API release build and ABI verification with individual exit0 and clean-source assertions. The workspace transcript has277 result blocks,1645 passes,0 failures,24 ignored; these include isolated-child results and are not a deduplicated unique-test count.

The ordinary worklet compilation succeeds and its digest comparison exits1 against the previous pin. Astra approved only the actual observed digest. Root waited for all immutable checks to finish before committing the exact pin at e9a6519e. The subsequent verified builder, static artifact/object/ABI checks, resource rejection controls, hermetic worklet tests, npm-ci, current three-browser qualification with self-tests and matrix check all exit0. Published module bytes and hash are independently recorded. Consumer numerical expectations remain unchanged; only generated candidate/hash identities may differ.

The original Luna/Sol packages retain the actual old-refresh240-versus30 source-level extraction mutant and field/PCM/state proof. No benchmark or timing was run, and no universal instruction-count or microseconds/block reduction is claimed. Runtime ramp simplification does not close every RT5 measurement/accounting obligation. Actual-PR review and required CI remain pending.
