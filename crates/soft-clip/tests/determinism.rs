//! The soft-clip corpus can fail: every case is finite and has many distinct outputs.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `f32`, `Simd4` and
//! `Simd8`, natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue
//! #1048): `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares
//! every case at every width against `SOFT_CLIP_DIGESTS` in the shipping profile, and the wasm
//! guests of `scripts/run-wasm-gates.sh` compare the same cases against the same pins.

use soft_clip::corpus::{CASE_NAMES, POINTS, run_case};

/// The corpus has to be able to fail: no NaN, and every case has to move.
///
/// Claim: a corpus change that makes a case emit NaN or an infinity, or
/// collapses it to a handful of outputs turns this red.
#[test]
fn the_corpus_is_not_vacuous() {
    for (case, name) in CASE_NAMES.into_iter().enumerate() {
        let mut words = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut words);
        assert!(
            words.iter().all(|word| f32::from_bits(*word).is_finite()),
            "{name} produced a non-finite sample"
        );
        words.sort_unstable();
        words.dedup();
        assert!(
            words.len() > 16,
            "{name} has only {} distinct outputs; a vacuous case would pass any pin",
            words.len()
        );
    }
}
