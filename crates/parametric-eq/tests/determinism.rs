//! E9's corpus can fail: every case is finite, non-silent and distinct per case and per lane.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `WIDTH` 1, 4 and 8,
//! natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue #1048):
//! `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares every
//! case at every width against `corpus::E9_DIGESTS` in the shipping profile, and the wasm guests of
//! `scripts/run-wasm-gates.sh` compare the same cases against the same pins. A mismatch there is
//! never fixed by re-pinning from a vector or wasm run (master plan §8 and the §10 fallback).

use parametric_eq::corpus;
use sha2::{Digest, Sha256};

fn digest(case: usize) -> [u8; 32] {
    let mut words = vec![0_u32; corpus::POINTS];
    corpus::run_case::<f32>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(digest: [u8; 32]) -> String {
    bench_support::digest::hex(&digest)
}

/// The corpus is NaN-free, non-trivial and genuinely different per case and per lane.
///
/// Without this, a corpus that silently produced zeros everywhere would agree with itself on every
/// target and prove nothing.
///
/// Claim: a corpus change that emits a non-finite sample, silences a lane, duplicates lane 0 into
/// another lane or makes two cases the same computation turns this red.
#[test]
fn the_corpus_is_finite_and_discriminating() {
    let mut digests = Vec::new();
    for case in 0..corpus::CASE_COUNT {
        let mut words = vec![0_u32; corpus::POINTS];
        corpus::run_case::<f32>(case, &mut words);
        let values: Vec<f32> = words.iter().map(|bits| f32::from_bits(*bits)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "{} produced a non-finite sample",
            corpus::CASE_NAMES[case]
        );
        for lane in 0..corpus::LANES {
            let window = &values[lane * corpus::FRAMES..(lane + 1) * corpus::FRAMES];
            assert!(
                window.iter().any(|value| value.abs() > 1.0e-6),
                "{} lane {lane} is silent",
                corpus::CASE_NAMES[case]
            );
        }
        for lane in 1..corpus::LANES {
            let first = &values[..corpus::FRAMES];
            let other = &values[lane * corpus::FRAMES..(lane + 1) * corpus::FRAMES];
            assert_ne!(
                first,
                other,
                "{} lane {lane} duplicates lane 0",
                corpus::CASE_NAMES[case]
            );
        }
        digests.push(hex(digest(case)));
    }
    digests.sort();
    digests.dedup();
    assert_eq!(
        digests.len(),
        corpus::CASE_COUNT,
        "two cases are the same case"
    );
}
