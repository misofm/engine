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

## Evidence (attempt 1, 2026-10-23)

- Cause: test-only. `assert_the_hostile_word_rounds_the_offsets_sum` called `session::vca_offsets_sum([HAND_DB, VCA_DB, 0.0])` on literal constants with no `black_box`, so under `--release` LLVM folded the `f64` sum at compile time (round to nearest) and the hostile word never reached it. The sibling `assert_the_hostile_word_rounds_the_composition` already used `black_box` for the same reason. Production is unaffected: its inputs are run-time model values, and the guard in `SessionState::command` is unchanged.
- Fix: `black_box` on the three inputs and on the sum, as in the sibling guard.
- Gate 1: the test passes in debug and in `--release` (reproduced red in `--release` before the fix).
- Gate 2: mutant `under_hostile_fp_env` skips `write_fp_control_word(hostile...)` (crates/lane/src/fpenv.rs, restored): red in both profiles at the composition guard; with that guard's call also removed, red in both profiles at "S sums alike under the hostile word".

## Verifier (attempt 1, 2026-10-23)

PASS (Fable). Root also black-boxed the parse vacuity guard of `a_session_booted_under_a_hostile_caller_word_is_the_canonical_session` (same class; it passed in release only because LLVM does not fold the parser today). The sweep found no other guard of this class: the remaining FP-environment guards use `black_box` or run on run-time buffers.
