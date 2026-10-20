# Test that the fp-environment attestation refuses a bad readback

Found by the TA4 survivor audit: the mutant `lane::fpenv::attest_fp_environment -> Ok(())` survives every test.

## Problem

`attest_fp_environment` (`crates/lane/src/fpenv.rs`) refuses a render thread when the canonical control word does not read back after it is installed (`CanonicalWordRefused`), or when the caller's word is not restored (`RestoreRefused`). On conforming x86-64 and AArch64 hardware both writes always take, so no test can reach either refusal: every test calls the function on a thread where it passes. A change that makes the function always return `Ok(())` stays green, and `host-core`'s mapping of the refusal (`crates/host-core/src/render_session.rs`) is never exercised.

## Smallest closable slice

Split the decision out of the hardware access: a pure function that takes the three observed words (`observed`, `readback`, `restored`) and returns the same `Result`. `attest_fp_environment` reads the words and calls it. No behaviour change on any target.

## Gates

1. A unit test in `crates/lane` calls the pure function with a readback whose control bits differ from the canonical word and gets `CanonicalWordRefused`; with a `restored` word that differs from `observed` it gets `RestoreRefused`; with matching words it gets `Ok`.
2. The mutant `attest_fp_environment -> Ok(())` is red, measured once (PR evidence, not a committed test).
3. No rendered bit moves; `cargo test -p lane -p host-core` is green.
