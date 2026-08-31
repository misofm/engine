//! E12: one pinned digest per corpus case, identical at every width.
//!
//! The pins in `true_peak_limiter::corpus::D90_DIGESTS` are generated from the scalar
//! `Lane` instantiation, which is the master plan's oracle for a lane-identity property; `Simd4`
//! and `Simd8` are compared against them, and `tools/wasm-gate-corpus` replays the same
//! cases under wasmtime with and without `simd128` against the same pins.

use lane::{Lane, Simd4, Simd8};
use sha2::{Digest, Sha256};
use true_peak_limiter::corpus;

fn digest<L: Lane>(case: usize) -> ([u8; 32], Vec<u32>) {
    let mut out = vec![0_u32; corpus::POINTS];
    corpus::run_case::<L>(case, &mut out);
    let mut hasher = Sha256::new();
    for word in &out {
        hasher.update(word.to_le_bytes());
    }
    (hasher.finalize().into(), out)
}

/// Set `MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_CORPUS=1` to print the scalar pins in `D90_DIGESTS`
/// form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else; the
/// vector widths and the wasm legs *confirm* them. Re-pin mode suppresses only the comparison
/// against the pin: `Simd4` and `Simd8` are still required to agree with the scalar oracle, so a
/// width disagreement cannot be laundered into a fresh pin.
///
/// This family has no independent `f64` oracle behind it — see the note above `D90_DIGESTS` and
/// issue #90.
#[test]
fn every_case_has_one_digest_at_every_width() {
    let repinning = std::env::var_os("MISO_ENGINE_REPIN_TRUE_PEAK_LIMITER_CORPUS").is_some();
    let mut repin = String::new();
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

        if !repinning {
            assert_eq!(
                scalar,
                corpus::D90_DIGESTS[case],
                "{name}: scalar digest moved; re-pin only from this oracle (master plan §8). \
                 Measured {}",
                hex(&scalar)
            );
        }
        assert_eq!(digest::<Simd4>(case).0, scalar, "{name} at Simd4");
        assert_eq!(digest::<Simd8>(case).0, scalar, "{name} at Simd8");

        let bytes: Vec<String> = scalar.iter().map(|byte| format!("0x{byte:02x}")).collect();
        repin.push_str(&format!("    // {name}\n"));
        repin.push_str(&format!("    [{}],\n", bytes.join(", ")));
    }
    if repinning {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into D90_DIGESTS in src/corpus.rs");
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
