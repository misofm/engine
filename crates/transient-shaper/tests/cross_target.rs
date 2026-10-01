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

use transient_shaper::corpus::{CASE_NAMES, WIDTHS, WORDS, run_case};

fn render(case: usize, width: usize) -> Vec<u32> {
    let mut words = vec![0_u32; WORDS];
    run_case(case, width, &mut words);
    words
}

/// Three widths agree on every corpus word, with bounded mismatch diagnostics and a
/// nonvacuity check. All render buffers have the fixed `WORDS` length.
///
/// Red mutation: any width-dependent edit to the kernel, or `Ramps::advance` packing lane `0` for
/// every lane.
#[test]
fn every_width_produces_the_same_words() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut scalar_words = render(case, 1);
        assert!(
            scalar_words
                .iter()
                .all(|word| f32::from_bits(*word).is_finite()),
            "{name}: the corpus must be NaN-free (D5)"
        );
        for width in WIDTHS.into_iter().filter(|width| *width != 1) {
            let words = render(case, width);
            let mismatch = words.iter().zip(&scalar_words).position(|(a, b)| a != b);
            assert_eq!(
                mismatch, None,
                "{name} at width {width}: first unequal word"
            );
        }
        scalar_words.sort_unstable();
        scalar_words.dedup();
        assert!(
            scalar_words.len() > WORDS / 4,
            "{name}: only {} distinct words -- the case is degenerate",
            scalar_words.len()
        );
    }
}
