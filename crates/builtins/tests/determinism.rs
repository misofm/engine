//! The builtin chain's cross-target corpus can fail: every case is finite and non-constant.
//!
//! The corpus's cross-target claim -- one pinned digest per case, identical at `f32`, `Simd4` and
//! `Simd8`, natively and under wasmtime (master plan #83 D5) -- has one owner, gate G5 (issue
//! #1048): `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares
//! every case at every width against `BUILTINS_DIGESTS` in the shipping profile, and the wasm
//! guests of `scripts/run-wasm-gates.sh` compare the same cases against the same pins. What stays
//! here is the claim G5 does not make for a delegated family.

use builtins::corpus::{CASE_NAMES, case_values};

/// No case is vacuous: every one produces finite, non-constant output.
///
/// Claim: a corpus change that makes a case silent, constant or NaN turns this red. A digest of
/// such a case would agree on every target and every width while proving nothing -- the failure
/// mode 83d found in the lane corpus, where a ramp starting at zero multiplied an impulse's only
/// non-zero sample away -- and a NaN would be canonicalised by wasm, which D5 excludes.
#[test]
fn no_corpus_case_is_vacuous_or_carries_a_nan() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let values = case_values::<f32>(case);
        assert!(!values.is_empty(), "case {name} is empty");
        assert!(
            values.iter().all(|value| value.is_finite()),
            "case {name} carries a non-finite word; the D5 claim excludes NaN payloads"
        );
        let distinct = values
            .iter()
            .map(|value| value.to_bits())
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            distinct.len() > 64,
            "case {name} has only {} distinct words",
            distinct.len()
        );
    }
}
