//! E12's corpus can fail: every case is finite and not vacuous.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `f32`, `Simd4` and
//! `Simd8`, natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue
//! #1048): `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares
//! every case at every width against `corpus::D90_DIGESTS` in the shipping profile, and the wasm
//! guests of `scripts/run-wasm-gates.sh` compare the same cases against the same pins.
//!
//! This family has no independent `f64` oracle behind its pins -- see the note above `D90_DIGESTS`
//! and issue #90.

use true_peak_limiter::corpus;

/// Every case renders finite words, and more than 64 distinct ones.
///
/// Claim: a corpus change that makes a case emit NaN or an infinity, or collapses it to a handful
/// of words (a case that no longer limits anything), turns this red. Either would let a digest
/// agree across targets for the empty reason, and wasm canonicalises NaN payloads (D5).
#[test]
fn every_case_is_finite_and_not_vacuous() {
    for case in 0..corpus::CASE_COUNT {
        let mut words = vec![0_u32; corpus::POINTS];
        corpus::run_case::<f32>(case, &mut words);
        let name = corpus::CASE_NAMES[case];
        assert!(
            words.iter().all(|word| f32::from_bits(*word).is_finite()),
            "{name} produced a non-finite sample"
        );
        let distinct = words
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert!(
            distinct > 64,
            "{name} produced only {distinct} distinct words"
        );
    }
}
