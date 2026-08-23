//! Every lane-generic function in this crate produces the same bits at `W = 1`, 4 and 8.
//!
//! `W = 1` (`f32`) is the oracle: it is the scalar tail, and it is what the `f64` oracles in the
//! other test binaries are compared against. Identity with the two vector widths is asserted by
//! `to_bits`, never by a tolerance (D5).
//!
//! Red mutation: reassociate one operation in any lane-generic body — for example write
//! `gain_delta_db`'s knee as `(v * v) * (inv_two_knee * inv_ratio_minus_one)`. The scalar and
//! vector instantiations still agree with each other (they share one body), so what this file
//! actually proves is that no backend-specific path exists. The mutation that *is* red here is a
//! width-dependent body: give `check_block` a `if L::WIDTH == 1` shortcut, or make `map_case`
//! stride by something other than the width.

use miso_engine_effect_runtime::bank::{check_block, nonfinite_lane_mask};
use miso_engine_effect_runtime::corpus::{CASE_NAMES, POINTS, run_case};
use miso_engine_effect_runtime::ramp::LinearRamp;
use miso_engine_lane::kernels::RampSegment;
use miso_engine_lane::{Lane, Simd4, Simd8};

fn lane_words<L: Lane>(value: L) -> Vec<u32> {
    let mut words = vec![0u32; L::WIDTH];
    value.store_bits(&mut words);
    words
}

/// The whole corpus — the gain computer at three ratios, both dB conversions, both followers and
/// the hysteresis — is width-independent, word for word.
#[test]
fn the_corpus_is_width_independent() {
    let mut scalar = vec![0u32; POINTS];
    let mut four = vec![0u32; POINTS];
    let mut eight = vec![0u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut scalar);
        run_case::<Simd4>(case, &mut four);
        run_case::<Simd8>(case, &mut eight);
        for point in 0..POINTS {
            assert_eq!(
                scalar[point], four[point],
                "{name} point {point}: W=1 {:#010x} vs W=4 {:#010x}",
                scalar[point], four[point]
            );
            assert_eq!(
                scalar[point], eight[point],
                "{name} point {point}: W=1 {:#010x} vs W=8 {:#010x}",
                scalar[point], eight[point]
            );
        }
    }
}

/// `advance_block` produces the same segment words at every width, and the same state.
#[test]
fn ramp_segments_are_width_independent() {
    for (target, samples) in [(1.0f32, 3u32), (-0.25, 64), (0.1, 5), (7.5, 500)] {
        for frames in [1usize, 7, 64, 128, 512] {
            let mut base = LinearRamp::fixed(-0.5);
            base.set_target(target, samples);

            let (mut a, mut b, mut c) = (base, base, base);
            let scalar: RampSegment<f32> = a.advance_block::<f32>(frames);
            let four: RampSegment<Simd4> = b.advance_block::<Simd4>(frames);
            let eight: RampSegment<Simd8> = c.advance_block::<Simd8>(frames);

            assert_eq!(a, b);
            assert_eq!(a, c);
            assert_eq!(scalar.ramp_frames, four.ramp_frames);
            assert_eq!(scalar.ramp_frames, eight.ramp_frames);
            for word in lane_words(four.start) {
                assert_eq!(word, scalar.start.to_bits());
            }
            for word in lane_words(eight.step) {
                assert_eq!(word, scalar.step.to_bits());
            }
            for word in lane_words(eight.target) {
                assert_eq!(word, scalar.target.to_bits());
            }
        }
    }
}

/// The boundary check answers the same question at every width, over a block that mixes clean and
/// dirty values in every lane position.
#[test]
fn the_boundary_check_is_width_independent() {
    let dirty = [f32::NAN, f32::INFINITY, 1e31, -1e31, f32::MAX];
    for position in 0..64usize {
        for value in dirty {
            let mut block = vec![0.25f32; 64];
            block[position] = value;
            assert!(!check_block::<f32>(&block), "W=1 at {position}");
            assert!(!check_block::<Simd4>(&block), "W=4 at {position}");
            assert!(!check_block::<Simd8>(&block), "W=8 at {position}");
        }
        let clean = vec![0.25f32; 64];
        assert!(check_block::<f32>(&clean));
        assert!(check_block::<Simd4>(&clean));
        assert!(check_block::<Simd8>(&clean));
    }
}

/// The failing-lane mask is the same set of tracks, expressed at each width.
#[test]
fn the_lane_mask_agrees_across_widths() {
    let mut block = vec![0.0f32; 8 * 8];
    block[8 * 2 + 5] = f32::NAN;
    assert_eq!(nonfinite_lane_mask::<Simd8>(&block), 1 << 5);
    // The same buffer read as four-lane frames: index 21 is lane 1 of frame 5.
    assert_eq!(nonfinite_lane_mask::<Simd4>(&block), 1 << 1);
    assert_eq!(nonfinite_lane_mask::<f32>(&block), 1);
}
