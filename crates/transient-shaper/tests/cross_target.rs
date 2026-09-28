//! Width identity for the transient shaper's cross-target corpus (master plan D5, §10 G2).
//!
//! `corpus::run_case` renders [`LANES`](transient_shaper::corpus::LANES) independent
//! tracks at widths 1, 4 and 8 and reads the result back lane-major, so the word stream describes
//! the arithmetic and not the layout. This gate asserts the three widths agree word for word in
//! this crate's own debug run. The corpus's cross-target claim -- one pinned digest per case, at
//! every width, natively and under wasmtime (§10 G5) -- has one owner, gate G5 (issue #1048):
//! `g5_native_digests_match_pins` in `tools/wasm-gates/tests/g5_native_corpus.rs` compares every
//! case at every width against `CROSS_TARGET_DIGESTS` in the shipping profile, and the wasm guests
//! of `scripts/run-wasm-gates.sh` compare the same cases against the same pins.

use sha2::{Digest, Sha256};
use transient_shaper::corpus::{CASE_NAMES, WIDTHS, WORDS, run_case};

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

fn hex(bytes: &[u8; 32]) -> String {
    bench_support::digest::hex(bytes)
}

#[test]
fn shared_hex_adapter_matches_literal_bytes() {
    assert_eq!(
        hex(&[0; 32]),
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        hex(&[
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff,
        ]),
        "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff"
    );
}
