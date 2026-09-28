# Keep the whole-plan scalar backend as a test-only oracle, out of shipped builds

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 3): the scalar (one-track-at-a-time, unbanked) renderer stays as the **test-only correctness reference** for vector banking ("a placement change must not move a rendered bit", AGENTS.md). It is a deliberate, named exception to "modes production never needs are removed". After the 64-bit-only ruling (#1041) no shipped platform selects it. This issue replaces draft R8 (`docs/handoffs/dead-code-2026-09-28/issues/R8-…`, which proposed removal; its amendments explain why the Simd4-against-Simd8 comparison alone is weaker).

## Smallest closable slice

1. Make `Backend::Scalar` and the whole-plan scalar path compile only for tests and the `test-support` features, so shipped artifacts (the AudioWorklet module, `capi` for iOS and Android, native) do not contain it. The per-effect one-lane scalar leg (W=1, `FrameLane`) is separate and stays wherever it is used.
2. Remove the now-dead x86 arms of `Backend::current()` noted in the #1041 verdict.
3. Coordinate with the scalar-wasm test builds (decision 7 and draft 05 of the test-value audit): if those retire first, the `#1041` scalar-wasm exception in `lane` goes with them.

## Objective gates

- The banking oracle tests (the forced-scalar differentials used by #966, #970, #971 and the console `Backend::Scalar` comparisons) still build and pass, and still go red under a planted banking-key mutation that is identical at both widths.
- Shipped artifacts contain no scalar whole-plan path: a check over the built AudioWorklet module and a release `capi` library (symbol or roster check), and the artifact change, if any, is explained.
- All four targets build; console digests unchanged; clippy, fmt and policy scripts pass.
