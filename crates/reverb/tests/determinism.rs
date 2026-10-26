//! The reverb's cross-target corpus can fail: every case is finite and alive.
//!
//! The corpus's cross-target claim -- one pinned digest per case, at every width, natively and
//! under wasmtime -- has one owner, gate G5 (issue #1048): `g5_native_digests_match_pins` in
//! `tools/wasm-gates/tests/g5_native_corpus.rs` compares every case against `corpus::G5_DIGESTS`
//! in the shipping profile, and the wasm guests of `scripts/run-wasm-gates.sh` compare the same
//! cases against the same pins.

use reverb::corpus;

/// A digest of silence or of NaN payloads would pass vacuously. Neither is what the corpus renders.
/// That the two cases differ is `g5_case_digests_are_distinct`'s claim, not this test's.
///
/// Claim: a corpus change that makes a case emit NaN or an infinity, silences it, or collapses it
/// to few words turns this red.
#[test]
fn corpus_cases_are_finite_and_alive() {
    let cases: Vec<Vec<u32>> = (0..corpus::CASE_COUNT)
        .map(|case| {
            let mut words = vec![0_u32; corpus::POINTS];
            corpus::run_case::<lane::Native>(case, &mut words);
            words
        })
        .collect();
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
