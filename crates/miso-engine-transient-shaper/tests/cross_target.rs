//! Width identity and the cross-target digest pins (master plan D5, §10 G2/G5).
//!
//! `corpus::run_case` renders [`LANES`](miso_engine_transient_shaper::corpus::LANES) independent
//! tracks at widths 1, 4 and 8 and reads the result back lane-major, so the word stream describes
//! the arithmetic and not the layout. This gate asserts the three widths agree and that they match
//! the pins; `tools/miso-engine-wasm-gates` replays the identical corpus under wasmtime against the
//! same pins, which is the other half of the claim.

use miso_engine_transient_shaper::corpus::{
    CASE_COUNT, CASE_NAMES, CROSS_TARGET_DIGESTS, WIDTHS, WORDS, run_case,
};
use sha2::{Digest, Sha256};

fn digest(case: usize, width: usize) -> ([u8; 32], Vec<u32>) {
    let mut words = vec![0_u32; WORDS];
    run_case(case, width, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    (hasher.finalize().into(), words)
}

/// One body, three widths, identical bits — and the corpus is not vacuous.
///
/// Red mutation: any width-dependent edit to the kernel, or `Ramps::advance` packing lane `0` for
/// every lane.
#[test]
fn every_width_produces_the_same_words() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let (scalar_digest, scalar_words) = digest(case, 1);
        assert!(
            scalar_words
                .iter()
                .all(|word| f32::from_bits(*word).is_finite()),
            "{name}: the corpus must be NaN-free (D5)"
        );
        let distinct = scalar_words
            .iter()
            .collect::<std::collections::HashSet<_>>();
        assert!(
            distinct.len() > WORDS / 4,
            "{name}: only {} distinct words -- the case is degenerate",
            distinct.len()
        );
        for width in WIDTHS {
            let (candidate, words) = digest(case, width);
            assert_eq!(
                words, scalar_words,
                "{name} at width {width}: lane identity"
            );
            assert_eq!(candidate, scalar_digest, "{name} at width {width}: digest");
        }
    }
}

/// The pinned digests. Regenerate only from the scalar `Lane` instantiation (master plan §8).
///
/// Red mutation: perturb `DB_PER_OCTAVE` by one ulp.
#[test]
fn the_pinned_digests_hold() {
    let mut actual = Vec::new();
    let mut failures = Vec::new();
    for case in 0..CASE_COUNT {
        let (candidate, _) = digest(case, 1);
        actual.push(hex(&candidate));
        if candidate != CROSS_TARGET_DIGESTS[case] {
            failures.push(format!(
                "{}: expected {}, got {}",
                CASE_NAMES[case],
                hex(&CROSS_TARGET_DIGESTS[case]),
                hex(&candidate)
            ));
        }
    }
    if !failures.is_empty() {
        for line in &actual {
            println!("PIN {line}");
        }
        panic!("{}", failures.join("\n"));
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
