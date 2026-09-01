//! The frozen corpus: one digest per case, identical at every width, and never NaN.
//!
//! This is the native leg of the cross-target gate. `tools/wasm-gates` replays the
//! same corpus inside a WebAssembly module against these same pins, so a rendered block that
//! differed between a browser and a native host would move a digest here first.

use lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};
use soft_clip::corpus::{
    CASE_COUNT, CASE_NAMES, FRAMES, LANES, POINTS, SOFT_CLIP_DIGESTS, run_case,
};

fn digest<L: Lane>(case: usize) -> ([u8; 32], Vec<u32>) {
    let mut words = vec![0_u32; POINTS];
    run_case::<L>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    (hasher.finalize().into(), words)
}

fn render(digest: [u8; 32]) -> String {
    let mut text = String::from("    [");
    for (index, byte) in digest.into_iter().enumerate() {
        if index.is_multiple_of(15) {
            text.push_str("\n        ");
        }
        text.push_str(&format!("0x{byte:02x}, "));
    }
    text.push_str("\n    ],");
    text
}

#[test]
fn every_case_matches_its_pin_at_every_width() {
    let mut printed = String::new();
    let mut failures = Vec::new();
    for case in 0..CASE_COUNT {
        let (scalar, _) = digest::<f32>(case);
        printed.push_str(&format!(
            "    // {}\n{}\n",
            CASE_NAMES[case],
            render(scalar)
        ));
        let (simd4, _) = digest::<Simd4>(case);
        let (simd8, _) = digest::<Simd8>(case);
        assert_eq!(scalar, simd4, "{} at Simd4", CASE_NAMES[case]);
        assert_eq!(scalar, simd8, "{} at Simd8", CASE_NAMES[case]);
        if scalar != SOFT_CLIP_DIGESTS[case] {
            failures.push(CASE_NAMES[case]);
        }
    }
    assert!(
        failures.is_empty(),
        "digest mismatch in {failures:?}; the scalar oracle produces:\n[\n{printed}]"
    );
}

/// The corpus has to be able to fail: no NaN, and every case has to move.
#[test]
fn the_corpus_is_not_vacuous() {
    for (case, name) in CASE_NAMES.into_iter().enumerate() {
        let (_, words) = digest::<f32>(case);
        assert_eq!(words.len(), LANES * FRAMES);
        let values: Vec<f32> = words.iter().map(|word| f32::from_bits(*word)).collect();
        assert!(
            values.iter().all(|value| value.is_finite()),
            "{name} produced a non-finite sample"
        );
        let distinct = values
            .iter()
            .map(|value| value.to_bits())
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            distinct.len() > 16,
            "{name} has only {} distinct outputs; a vacuous case would pass any pin",
            distinct.len()
        );
    }
}
