//! Gate G5, native leg: the delay's cross-target corpus produces its pinned digests.
//!
//! `tools/wasm-gate-corpus` replays these same cases under wasmtime against the same
//! pins, so this test is what makes that replay meaningful: it proves the pins still describe the
//! corpus on the host the pins were generated on.

use delay::corpus;
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

/// Set `MISO_ENGINE_REPIN_DELAY_CORPUS=1` to print the scalar pins in `G5_DIGESTS` form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else. The
/// delay is a `W = 1` effect (§4.1), so there is no vector width to confirm against here; the wasm
/// leg in `tools/wasm-gate-corpus` is what confirms these pins on another target, and
/// it never sources them.
#[test]
fn corpus_digests_match_their_pins() {
    let repinning = std::env::var_os("MISO_ENGINE_REPIN_DELAY_CORPUS").is_some();
    let mut repin = String::new();
    for case in 0..corpus::CASE_COUNT {
        let scalar = digest(case);
        if !repinning {
            assert_eq!(
                scalar,
                corpus::G5_DIGESTS[case],
                "case {}",
                corpus::CASE_NAMES[case]
            );
        }
        let bytes: Vec<String> = scalar.iter().map(|byte| format!("0x{byte:02X}")).collect();
        repin.push_str(&format!("    // {}\n", corpus::CASE_NAMES[case]));
        repin.push_str(&format!("    [{}],\n", bytes.join(", ")));
    }
    if repinning {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into G5_DIGESTS in src/corpus.rs");
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
