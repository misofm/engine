//! The delay's cross-target corpus can fail: every case is finite, alive and distinct.
//!
//! The corpus's cross-target claim -- one pinned digest per case, natively and under wasmtime --
//! has one owner, gate G5 (issue #1048): `g5_native_digests_match_pins` in
//! `tools/wasm-gates/tests/g5_native_corpus.rs` compares every case against `corpus::G5_DIGESTS` in
//! the shipping profile, and the wasm guests of `scripts/run-wasm-gates.sh` compare the same cases
//! against the same pins. The delay is a `W = 1` effect (master plan §4.1), so there is no width to
//! compare; what the corpus carries across the target boundary is the kernel's `Lane::fma` sites.

use delay::corpus;

/// A digest of silence, of NaN payloads, or of two identical cases would pass vacuously. None of
/// those is what the corpus renders.
///
/// Claim: a corpus change that makes a case emit NaN or an infinity, silences it, collapses it to
/// few words, or makes the two cases the same computation turns this red.
#[test]
fn corpus_cases_are_finite_distinct_and_alive() {
    let cases: Vec<Vec<u32>> = (0..corpus::CASE_COUNT)
        .map(|case| {
            let mut words = vec![0_u32; corpus::POINTS];
            corpus::run_case(case, &mut words);
            words
        })
        .collect();
    // Compare every ordered word before sorting; sorting only serves the distinct-word count.
    assert!(cases[0] != cases[1], "the two corpus cases are identical");
    for (case, mut words) in cases.into_iter().enumerate() {
        assert!(
            words.iter().all(|word| f32::from_bits(*word).is_finite()),
            "case {} is not finite",
            corpus::CASE_NAMES[case]
        );
        let peak = words
            .iter()
            .fold(0.0_f32, |peak, word| peak.max(f32::from_bits(*word).abs()));
        assert!(peak > 0.25, "case {} is silent", corpus::CASE_NAMES[case]);
        words.sort_unstable();
        words.dedup();
        let distinct = words.len();
        assert!(
            distinct > corpus::POINTS / 2,
            "case {} has only {distinct} distinct words",
            corpus::CASE_NAMES[case]
        );
    }
}
