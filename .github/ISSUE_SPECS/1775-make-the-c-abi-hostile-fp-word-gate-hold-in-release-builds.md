# Make the C ABI hostile-FP-word gate hold in release builds

Found by the #1767 verifier (2026-10-23); pre-existing on main 49d32884d.

## Problem

`a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits` (`crates/capi/src/runtime/live_tests.rs`, near 6570) passes in debug, which is how CI runs it (test-debug-a), but fails under `cargo test --release -p capi --lib`: "the gate is vacuous: S sums alike under the hostile word" (both sums 4626322717216342016). The test's vacuity guard is right to fail: in release the hostile caller floating-point control word does not change the reference sum S, so the test no longer proves that the render ignores the caller's FP word. The likely cause is that the optimizer folds or reorders the reference computation (a missing `black_box`, or a computation done at compile time), so the hostile word never reaches it.

## Smallest closable slice

- Find why the reference sum is insensitive to the hostile word in release (inspect the computation and its codegen), and make the guard's reference computation run at run time under the hostile word in both profiles (for example `core::hint::black_box` on its inputs).
- The test then passes in debug and release, and its guard still fails if the hostile word is not applied.

## Authorized paths

- `crates/capi/src/runtime/live_tests.rs`
- this spec

## Objective gates

1. `cargo test -p capi --lib a_live_fader_and_vca_move_under_a_hostile_caller_word` passes in debug and in `--release`.
2. A mutant that skips installing the hostile word turns the guard red in both profiles.

## Test value

No new test; the fix restores an existing gate's power in release builds.

## Dependencies

None.
