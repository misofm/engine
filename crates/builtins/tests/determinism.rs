//! Cross-target determinism: one pinned digest per corpus case, at every lane width.
//!
//! Master plan #83 D5. The pins come from the scalar `Lane` instantiation -- never from a vector
//! or a wasm run (§8) -- and `tools/wasm-gates` replays the identical corpus under
//! `wasmtime` against these same constants, so a browser build that stopped agreeing with a native
//! one moves a digest here rather than going unnoticed.

use builtins::corpus::{BUILTINS_DIGESTS, CASE_COUNT, CASE_NAMES, case_values};
use lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};

/// SHA-256 over the little-endian bits of every result word.
fn digest(values: &[f32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value.to_bits().to_le_bytes());
    }
    hasher.finalize().into()
}

fn digest_at<L: Lane>(case: usize) -> [u8; 32] {
    digest(&case_values::<L>(case))
}

/// Every case agrees with its pin at `f32`, `Simd4` and `Simd8`.
///
/// Set `MISO_ENGINE_REPIN_BUILTINS_CORPUS=1` to print the scalar pins in `BUILTINS_DIGESTS` form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else; the
/// vector widths and the wasm legs *confirm* them. Re-pin mode suppresses only the comparison
/// against the pin: `Simd4` and `Simd8` are still required to agree with the scalar oracle, so a
/// width disagreement cannot be laundered into a fresh pin.
#[test]
fn every_corpus_case_matches_its_pin_at_every_width() {
    let repinning = std::env::var_os("MISO_ENGINE_REPIN_BUILTINS_CORPUS").is_some();
    let mut repin = String::new();
    for case in 0..CASE_COUNT {
        let name = CASE_NAMES[case];
        let scalar = digest_at::<f32>(case);
        if !repinning {
            assert_eq!(
                scalar, BUILTINS_DIGESTS[case],
                "case {name}: scalar digest moved -- regenerate only from the scalar oracle"
            );
        }
        assert_eq!(digest_at::<Simd4>(case), scalar, "case {name}: Simd4");
        assert_eq!(digest_at::<Simd8>(case), scalar, "case {name}: Simd8");
        let bytes: Vec<String> = scalar.iter().map(|byte| format!("0x{byte:02x}")).collect();
        repin.push_str(&format!("    // {name}\n"));
        repin.push_str(&format!("    [{}],\n", bytes.join(", ")));
    }
    if repinning {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into BUILTINS_DIGESTS in src/corpus.rs");
    }
}

/// No case is vacuous: every one produces finite, non-constant output.
///
/// Without this a digest could agree on every target and every width while proving nothing -- the
/// failure mode 83d found in the lane corpus, where a ramp starting at zero multiplied an
/// impulse's only non-zero sample away.
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
