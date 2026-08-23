//! Gate G5, native leg: the delay's cross-target corpus produces its pinned digests.
//!
//! `tools/miso-engine-wasm-gate-corpus` replays these same cases under wasmtime against the same
//! pins, so this test is what makes that replay meaningful: it proves the pins still describe the
//! corpus on the host the pins were generated on.

use miso_engine_delay::corpus;
use sha2::{Digest, Sha256};

fn digest(case: usize) -> [u8; 32] {
    let mut words = vec![0_u32; corpus::POINTS];
    corpus::run_case(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

#[test]
fn corpus_digests_match_their_pins() {
    for case in 0..corpus::CASE_COUNT {
        assert_eq!(
            digest(case),
            corpus::G5_DIGESTS[case],
            "case {}",
            corpus::CASE_NAMES[case]
        );
    }
}

/// A digest of silence, of NaN payloads, or of two identical cases would pass vacuously. None of
/// those is what the corpus renders.
#[test]
fn corpus_cases_are_finite_distinct_and_alive() {
    let mut digests = Vec::new();
    for case in 0..corpus::CASE_COUNT {
        let mut words = vec![0_u32; corpus::POINTS];
        corpus::run_case(case, &mut words);
        let values: Vec<f32> = words.iter().map(|word| f32::from_bits(*word)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "case {} is not finite",
            corpus::CASE_NAMES[case]
        );
        let peak = values
            .iter()
            .fold(0.0_f32, |peak, value| peak.max(value.abs()));
        assert!(peak > 0.25, "case {} is silent", corpus::CASE_NAMES[case]);
        let distinct = words
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert!(
            distinct > corpus::POINTS / 2,
            "case {} has only {distinct} distinct words",
            corpus::CASE_NAMES[case]
        );
        digests.push(digest(case));
    }
    assert_ne!(digests[0], digests[1]);
}

/// The pins are generated from the scalar run, in the form the source carries.
#[test]
#[ignore = "regeneration helper; run with --ignored to print the pins"]
fn print_pins() {
    for case in 0..corpus::CASE_COUNT {
        let bytes = digest(case);
        let words: Vec<String> = bytes.iter().map(|byte| format!("0x{byte:02X}")).collect();
        println!("    // {}", corpus::CASE_NAMES[case]);
        println!("    [{}],", words.join(", "));
    }
}
