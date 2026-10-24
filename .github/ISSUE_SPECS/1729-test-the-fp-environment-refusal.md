# Test that the fp-environment attestation refuses a bad readback

Found by the TA4 survivor audit: the mutant `lane::fpenv::attest_fp_environment -> Ok(())` survives every test.

## Problem

`attest_fp_environment` (`crates/lane/src/fpenv.rs`) refuses a render thread when the canonical control word does not read back after it is installed (`CanonicalWordRefused`), or when the caller's word is not restored (`RestoreRefused`). On conforming x86-64 and AArch64 hardware both writes always take, so no test can reach either refusal: every test calls the function on a thread where it passes. A change that makes the function always return `Ok(())` stays green, and `host-core`'s mapping of the refusal (`crates/host-core/src/render_session.rs`) is never exercised.

## Smallest closable slice

Split the decision out of the hardware access: a pure function that takes the three observed words (`observed`, `readback`, `restored`) and returns the same `Result`. `attest_fp_environment` reads the words and calls it. No behaviour change on any target.

## Gates

1. A unit test in `crates/lane` calls the pure function with a readback whose control bits differ from the canonical word and gets `CanonicalWordRefused`; with a `restored` word that differs from `observed` it gets `RestoreRefused`; with matching words it gets `Ok`.
2. The mutant `judge_fp_environment -> Ok(())` is red, measured once (PR evidence, not a committed test). The literal `attest_fp_environment -> Ok(())` mutant still survives after the split: its body is then three reads, the guard and the call, and no test can make conforming hardware refuse a write. It is recorded as hardware-unreachable, not as a gap.
3. No rendered bit moves; `cargo test -p lane -p host-core` is green.

## Related: the browser side of the hostile-FP live gate

TA4 deleted host-web `tests::canonical_live::a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits` (#1709 D2, browser); the capi test of the same name stays (`crates/capi/src/runtime/live_tests.rs:8158`), so only the C ABI side of that check remains. If the owner wants the browser side back, restore it from `b9135008d` (`hosts/host-web/src/tests.rs`) with the `lane` test-support dev-dependency.

Decision (root, 2026-10-24, a test-value call under AGENTS.md's one-parity-row-per-host rule): no. The capi twin keeps the claim.

## Evidence and decision record

- Decisions (root, from the scope note): the pure function is `judge_fp_environment(observed, readback, restored)`, placed directly above `attest_fp_environment` so the `fpenv.rs:145`/`:195` anchors in `docs/REALTIME_DEPENDENCY_POLICY.md` hold. It stays under the native `x86_64`/`aarch64` cfg: on wasm `FpControlWord = ()`, and comparing `()` trips `clippy::unit_cmp`. The wasm `attest_fp_environment` is unchanged. The test is one arch-neutral test in `crates/lane/tests/fp_env.rs`; gate 2 names the pure function's mutant (above).
- Behaviour: the native shell now reads `restored` also when the readback check fails; the returned `Result` is the same in every case.
- Gate 1: `the_attestation_refuses_an_unpinned_readback_and_an_unrestored_word` passes on x86_64; `cargo check --tests` and clippy `--all-targets` pass for `aarch64-unknown-linux-gnu`, `aarch64-apple-ios` and `aarch64-linux-android`.
- Gate 2 (measured once, x86_64): with `judge_fp_environment` returning `Ok(())`, the new test fails (`left: Ok(())`, `right: Err(... CanonicalWordRefused)`); the other 5 `fp_env` tests stay green.
- Gate 3: `cargo test --locked -p lane -p host-core` green. The change touches no render path, so no rendered bit can move.
