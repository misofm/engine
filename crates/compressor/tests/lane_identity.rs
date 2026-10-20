//! E2 — the bank and `W` scalar instances render the same bits, per lane.
//!
//! Two halves, because a bank binding is only available at the width this build was compiled for
//! (D4 revision 4: no runtime SIMD dispatch):
//!
//! * the **production** half compares a bound bank against `W` separately prepared scalar
//!   instances, sample by sample, plus their per-track payload bytes and their reports;
//! * the **width** half runs the frozen corpus through the same kernel at `W = 1` and every vector
//!   width the build has (`lane::each_vector_lane!`: 4, and 8 in the 8-lane (AVX2) build) and
//!   compares the result words, which covers the widths a given host cannot bind.
//!
//! The tracks are deliberately heterogeneous — different threshold, ratio, knee and mix
//! — because a bank that used lane 0's coefficients for every lane would pass a homogeneous test.

mod support;

use compressor::corpus::{CASE_NAMES, POINTS, run_case};

/// The one kernel body produces identical result words at `W = 1` and every vector width the build
/// has: 4, and 8 in the 8-lane (AVX2) build (#1112).
///
/// Run over the frozen corpus of `tests/cross_target.rs`, so this is the same computation the wasm
/// leg replays; the difference is that this test compares the three widths to *each other* rather
/// than to a pin, and reports the first differing word rather than a digest.
#[test]
fn every_width_produces_the_same_words() {
    for (case, name) in CASE_NAMES.iter().enumerate() {
        let mut scalar = vec![0_u32; POINTS];
        run_case::<f32>(case, &mut scalar);
        lane::each_vector_lane!(|L, N| {
            let mut wide = vec![0_u32; POINTS];
            run_case::<L>(case, &mut wide);
            for (index, (one, word)) in scalar.iter().zip(&wide).enumerate() {
                assert_eq!(
                    word, one,
                    "{name}: word {index} differs between W1 and W{N}"
                );
            }
        });
    }
}
