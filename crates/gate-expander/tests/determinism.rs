//! Gate 7.9's corpus can fail: every case is finite and mostly non-zero.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `f32`, `Simd4` and
//! `Simd8`, natively and under wasmtime (master plan #83 §1.7 and §8) -- has one owner, gate G5
//! (issue #1048): `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs`
//! compares every case at every width against `corpus::GATE_DIGESTS` (`src/gate_digests.in`) in
//! the shipping profile, and the wasm guests of `scripts/run-wasm-gates.sh` compare the same cases
//! against the same pins.

use gate_expander::corpus::{CASE_NAMES, POINTS, run_case};

/// Claim: a corpus change that makes a case emit NaN or an infinity, or leaves three quarters of
/// its results at zero, turns this red.
#[test]
fn no_case_is_vacuous() {
    // A corpus of zeros, or of NaN, would agree at every width for the wrong reason. D5 also
    // excludes NaN payloads outright, because wasm canonicalises them.
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut words = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut words);
        let values: Vec<f32> = words.iter().map(|word| f32::from_bits(*word)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "{name}: a non-finite value reached the digest"
        );
        let distinct = values.iter().filter(|value| **value != 0.0).count();
        assert!(
            distinct > POINTS / 4,
            "{name}: only {distinct} of {POINTS} results are non-zero"
        );
    }
}
