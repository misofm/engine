//! E12: one pinned digest per corpus case, identical at every width.
//!
//! The pins in `miso_engine_true_peak_limiter::corpus::D90_DIGESTS` are generated from the scalar
//! `Lane` instantiation, which is the master plan's oracle for a lane-identity property; `Simd4`
//! and `Simd8` are compared against them, and `tools/miso-engine-wasm-gate-corpus` replays the same
//! cases under wasmtime with and without `simd128` against the same pins.

use miso_engine_lane::{Lane, Simd4, Simd8};
use miso_engine_true_peak_limiter::corpus;
use sha2::{Digest, Sha256};

fn digest<L: Lane>(case: usize) -> ([u8; 32], Vec<u32>) {
    let mut out = vec![0_u32; corpus::POINTS];
    corpus::run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    (hasher.finalize().into(), out)
}

#[test]
fn every_case_has_one_digest_at_every_width() {
    for case in 0..corpus::CASE_COUNT {
        let (scalar, words) = digest::<f32>(case);
        let name = corpus::CASE_NAMES[case];

        // The case has to be non-vacuous: finite, not all one value, and actually limiting.
        assert!(
            words.iter().all(|word| f32::from_bits(*word).is_finite()),
            "{name} produced a non-finite sample"
        );
        let distinct = words
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert!(
            distinct > 64,
            "{name} produced only {distinct} distinct words"
        );

        assert_eq!(
            scalar,
            corpus::D90_DIGESTS[case],
            "{name}: scalar digest moved; re-pin only from this oracle (master plan §8). \
             Measured {}",
            hex(&scalar)
        );
        assert_eq!(digest::<Simd4>(case).0, scalar, "{name} at Simd4");
        assert_eq!(digest::<Simd8>(case).0, scalar, "{name} at Simd8");
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
