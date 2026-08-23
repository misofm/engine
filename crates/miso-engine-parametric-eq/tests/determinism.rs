//! E9: the frozen cross-target corpus, digested at every width.
//!
//! `tools/miso-engine-wasm-gate-corpus` replays the identical corpus under wasmtime against these
//! same pins, so the native leg here and the wasm leg there are one gate. A mismatch is never fixed
//! by re-pinning (master plan §8 and the §10 fallback): it means a backend or a target stopped
//! agreeing with the scalar `Lane` oracle.

use miso_engine_lane::{Lane, Simd4, Simd8};
use miso_engine_parametric_eq::corpus;
use sha2::{Digest, Sha256};

fn digest<L: Lane>(case: usize) -> [u8; 32] {
    let mut words = vec![0_u32; corpus::POINTS];
    corpus::run_case::<L>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

fn hex(digest: [u8; 32]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Every case is identical at `WIDTH` 1, 4 and 8 and equal to its pin.
#[test]
fn the_corpus_digests_match_the_pins_at_every_width() {
    for case in 0..corpus::CASE_COUNT {
        let scalar = digest::<f32>(case);
        let simd4 = digest::<Simd4>(case);
        let simd8 = digest::<Simd8>(case);
        assert_eq!(
            hex(simd4),
            hex(scalar),
            "{} simd4 vs scalar",
            corpus::CASE_NAMES[case]
        );
        assert_eq!(
            hex(simd8),
            hex(scalar),
            "{} simd8 vs scalar",
            corpus::CASE_NAMES[case]
        );
        assert_eq!(
            hex(scalar),
            hex(corpus::E9_DIGESTS[case]),
            "{} pin",
            corpus::CASE_NAMES[case]
        );
    }
}

/// The corpus is NaN-free, non-trivial and genuinely different per case and per lane.
///
/// Without this, a corpus that silently produced zeros everywhere would agree with itself on every
/// target and prove nothing.
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
        digests.push(hex(digest::<f32>(case)));
    }
    digests.sort();
    digests.dedup();
    assert_eq!(
        digests.len(),
        corpus::CASE_COUNT,
        "two cases are the same case"
    );
}

/// Prints the pins in the form `src/corpus.rs` holds them. Run with `--ignored --nocapture` after a
/// deliberate corpus change, never to "fix" a mismatch.
#[test]
#[ignore = "pin generator"]
fn print_pins() {
    for case in 0..corpus::CASE_COUNT {
        let digest = digest::<f32>(case);
        let bytes: Vec<String> = digest.iter().map(|byte| format!("0x{byte:02x}")).collect();
        println!("    // {}", corpus::CASE_NAMES[case]);
        println!("    [{}],", bytes.join(", "));
    }
}
