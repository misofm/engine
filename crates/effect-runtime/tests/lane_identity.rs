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

use effect_runtime::bank::{check_block, nonfinite_lane_mask};
use effect_runtime::corpus::{CASE_NAMES, POINTS, run_case};
use effect_runtime::ramp::LinearRamp;
use lane::kernels::RampSegment;
use lane::{Lane, Simd4};

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
    let mut vector = vec![0u32; POINTS];
    for (case, name) in CASE_NAMES.iter().enumerate() {
        run_case::<f32>(case, &mut scalar);
        lane::each_vector_lane!(|L, N| {
            run_case::<L>(case, &mut vector);
            for point in 0..POINTS {
                assert_eq!(
                    scalar[point], vector[point],
                    "{name} point {point}: W=1 {:#010x} vs W={N} {:#010x}",
                    scalar[point], vector[point]
                );
            }
        });
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
            // The build's own width: eight lanes where `avx2` is enabled, four otherwise (#1112).
            let native: RampSegment<lane::Native> = c.advance_block::<lane::Native>(frames);

            assert_eq!(a, b);
            assert_eq!(a, c);
            assert_eq!(scalar.ramp_frames, four.ramp_frames);
            assert_eq!(scalar.ramp_frames, native.ramp_frames);
            for word in lane_words(four.start) {
                assert_eq!(word, scalar.start.to_bits());
            }
            for word in lane_words(native.step) {
                assert_eq!(word, scalar.step.to_bits());
            }
            for word in lane_words(native.target) {
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
    let mut block = [0.25f32; 64];
    lane::each_lane!(|L| assert!(check_block::<L>(&block)));
    for position in 0..64usize {
        for value in dirty {
            block[position] = value;
            lane::each_lane!(|L, N| assert!(!check_block::<L>(&block), "W={N} at {position}"));
        }
        block[position] = 0.25;
    }
}

/// The failing-lane mask is the same set of tracks, expressed at each width.
#[test]
fn the_lane_mask_agrees_across_widths() {
    let mut block = vec![0.0f32; 8 * 8];
    let index = 8 * 2 + 5;
    block[index] = f32::NAN;
    // The same buffer read as `N`-lane frames: index 21 is lane `21 % N` of its frame (lane 5 of
    // frame 2 at eight lanes, lane 1 of frame 5 at four).
    lane::each_vector_lane!(|L, N| assert_eq!(nonfinite_lane_mask::<L>(&block), 1 << (index % N)));
    assert_eq!(nonfinite_lane_mask::<f32>(&block), 1);
}
