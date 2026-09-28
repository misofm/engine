//! E5's corpus can fail: every case is finite and moves.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `f32`, `Simd4` and
//! `Simd8`, natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue
//! #1048): `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares
//! every case at every width against `corpus::DIGESTS` (`src/corpus_digests.in`) in the shipping
//! profile, and the wasm guests of `scripts/run-wasm-gates.sh` compare the same cases against the
//! same pins. The pins stay in this crate, so the gate replays them rather than a copy of them.

use multiband_compressor::corpus::{CASE_NAMES, POINTS, run_case};

/// Every case renders finite words, and not every point the same word.
///
/// Claim: a corpus change that makes a case emit NaN or an infinity, or renders it constant, turns
/// this red. Either would let a digest agree across targets for the empty reason, and the
/// cross-target claim excludes NaN because wasm canonicalises its payloads.
#[test]
fn every_case_is_finite_and_moves() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut words = vec![0u32; POINTS];
        run_case::<f32>(case, &mut words);
        assert!(
            words.iter().any(|word| *word != words[0]),
            "{name}: every point produced the same word"
        );
        for word in &words {
            let value = f32::from_bits(*word);
            assert!(
                value.is_finite(),
                "{name}: produced {value}, and the cross-target claim excludes NaN"
            );
        }
    }
}
