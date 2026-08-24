//! E5: the corpus digests, pinned from the scalar oracle and checked at every width.
//!
//! The wasm leg of this gate lives in `tools/miso-engine-wasm-gates`, which replays
//! `miso_engine_multiband_compressor::corpus` inside a WebAssembly module against these same pins.
//! Keeping the pins in this crate rather than copying them into the gate tool is deliberate: a
//! second copy could drift away from the gate it is meant to replay.

use miso_engine_lane::{Lane, Simd4, Simd8};
use miso_engine_multiband_compressor::corpus::{CASE_COUNT, CASE_NAMES, DIGESTS, POINTS, run_case};
use sha2::{Digest, Sha256};

fn digest<L: Lane>(case: usize) -> ([u8; 32], Vec<u32>) {
    let mut words = vec![0u32; POINTS];
    run_case::<L>(case, &mut words);
    let mut hasher = Sha256::new();
    for word in &words {
        hasher.update(word.to_le_bytes());
    }
    (hasher.finalize().into(), words)
}

/// Set `MISO_ENGINE_REPIN_MULTIBAND_CORPUS=1` to print the scalar pins in `corpus_digests.in` form.
///
/// Per master plan §8.3 the pins come from the `f32` instantiation and from nowhere else; the
/// vector widths and the wasm legs *confirm* them.
#[test]
fn the_corpus_digests_are_pinned_and_width_independent() {
    let mut repin = String::from("[\n");
    for case in 0..CASE_COUNT {
        let (scalar, words) = digest::<f32>(case);
        assert!(
            words.iter().any(|word| *word != words[0]),
            "{}: every point produced the same word",
            CASE_NAMES[case]
        );
        for word in &words {
            let value = f32::from_bits(*word);
            assert!(
                value.is_finite(),
                "{}: produced {value}, and the cross-target claim excludes NaN",
                CASE_NAMES[case]
            );
        }
        repin.push_str("    [\n        ");
        for (index, byte) in scalar.iter().enumerate() {
            repin.push_str(&format!("0x{byte:02x}, "));
            if index % 8 == 7 {
                repin.push_str("\n        ");
            }
        }
        repin.push_str("\n    ],\n");
        assert_eq!(
            digest::<Simd4>(case).0,
            scalar,
            "{} differs between W=1 and W=4",
            CASE_NAMES[case]
        );
        assert_eq!(
            digest::<Simd8>(case).0,
            scalar,
            "{} differs between W=1 and W=8",
            CASE_NAMES[case]
        );
        if std::env::var_os("MISO_ENGINE_REPIN_MULTIBAND_CORPUS").is_none() {
            assert_eq!(
                scalar, DIGESTS[case],
                "{} moved: re-pin only from an oracle, never from a run",
                CASE_NAMES[case]
            );
        }
    }
    repin.push(']');
    if std::env::var_os("MISO_ENGINE_REPIN_MULTIBAND_CORPUS").is_some() {
        println!("{repin}");
        panic!("re-pin mode: copy the block above into src/corpus_digests.in");
    }
}
