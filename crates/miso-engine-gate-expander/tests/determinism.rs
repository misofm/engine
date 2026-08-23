//! Gate 7.9 (native leg): the corpus digests are width independent and match their pins.
//!
//! The pins in `src/gate_digests.in` come from the scalar `Lane` instantiation, which is the
//! oracle for every other width (master plan #83 §1.7 and §8). `tools/miso-engine-wasm-gates`
//! replays exactly these cases under wasmtime and compares against exactly these pins.

use miso_engine_gate_expander::corpus::{CASE_COUNT, CASE_NAMES, GATE_DIGESTS, POINTS, run_case};
use miso_engine_lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};

fn digest<L: Lane>(case: usize) -> [u8; 32] {
    let mut words = vec![0_u32; POINTS];
    run_case::<L>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

#[test]
fn every_case_agrees_at_every_width_and_matches_its_pin() {
    for case in 0..CASE_COUNT {
        let scalar = digest::<f32>(case);
        assert_eq!(
            scalar, GATE_DIGESTS[case],
            "{}: the scalar oracle moved away from its pin",
            CASE_NAMES[case]
        );
        assert_eq!(
            digest::<Simd4>(case),
            scalar,
            "{}: Simd4 disagrees with the scalar oracle",
            CASE_NAMES[case]
        );
        assert_eq!(
            digest::<Simd8>(case),
            scalar,
            "{}: Simd8 disagrees with the scalar oracle",
            CASE_NAMES[case]
        );
    }
}

#[test]
fn no_case_is_vacuous() {
    // A corpus of zeros, or of NaN, would agree at every width for the wrong reason. D5 also
    // excludes NaN payloads outright, because wasm canonicalises them.
    for case in 0..CASE_COUNT {
        let mut words = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut words);
        let values: Vec<f32> = words.iter().map(|word| f32::from_bits(*word)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "{}: a non-finite value reached the digest",
            CASE_NAMES[case]
        );
        let distinct = values.iter().filter(|value| **value != 0.0).count();
        assert!(
            distinct > POINTS / 4,
            "{}: only {distinct} of {POINTS} results are non-zero",
            CASE_NAMES[case]
        );
    }
}
