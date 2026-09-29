//! E4's corpus can fail: every case is finite, busy and distinct from the others.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `W = 1`, 4 and 8,
//! natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue #1048):
//! `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares every
//! case at every width against `corpus::C1_DIGESTS` in the shipping profile, and the wasm guests of
//! `scripts/run-wasm-gates.sh` compare the same cases against the same pins.
//! `tests/lane_identity.rs` compares the widths to each other in this crate's own debug run.
//!
//! A digest is not an oracle. `tests/oracle.rs` is what says the compressor is a compressor;
//! `tests/static_curve.rs` is what says the curve is Giannoulis, Massberg and Reiss equation 4.

use compressor::corpus::{CASE_COUNT, CASE_NAMES, POINTS, run_case};
use sha2::{Digest, Sha256};

fn digest(words: &[u32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for word in words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// The corpus is NaN-free and Inf-free, so the pins survive wasm's NaN canonicalisation (D5).
///
/// Claim: a corpus change that makes a case emit NaN or an infinity turns this red.
#[test]
fn the_corpus_is_finite() {
    let mut out = vec![0_u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut out);
        for (index, word) in out.iter().enumerate() {
            let value = f32::from_bits(*word);
            assert!(value.abs() <= f32::MAX, "{name}: word {index} is {value}");
        }
    }
}

/// No case is vacuous: each one produces many distinct non-zero values, and the cases differ from
/// one another. Without this a corpus that rendered silence would agree with itself on every target
/// and prove nothing.
///
/// Claim: a corpus change that silences a case, collapses it to few words, or makes two cases the
/// same computation turns this red.
#[test]
fn no_case_is_vacuous() {
    let mut digests = Vec::new();
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut out = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut out);
        let nonzero = out
            .iter()
            .filter(|word| f32::from_bits(**word) != 0.0)
            .count();
        assert!(
            nonzero > POINTS / 2,
            "{name}: only {nonzero} of {POINTS} words are non-zero"
        );
        let mut distinct: Vec<u32> = out.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert!(
            distinct.len() > POINTS / 2,
            "{name}: only {} distinct words",
            distinct.len()
        );
        digests.push(digest(&out));
    }
    let mut unique = digests.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), CASE_COUNT, "two cases render the same words");
}
