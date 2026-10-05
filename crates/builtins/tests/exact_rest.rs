#![allow(missing_docs)]
//! Issue #1328 gate 1: a near-Nyquist input filter reaches exact rest after its input stops.
//!
//! Where the input LPF's `c1` rounds to `1.0f32` (from about 22,047.6 Hz at 44.1 kHz up to the
//! domain maximum), a per-word flush of the two integrators leaves a period-2 limit cycle: `ic1`
//! alternates in sign between about `1e-20` and `1.35e-16` and the output rings near `±2e-21`
//! forever, from sample 858,748 for this exact impulse. The joint SVF flush (`lane::flush_pair`)
//! zeroes the pair once both words are below `REST_EPS` on a sample whose section input is exactly
//! zero (amendment A8), so after the impulse every integrator reaches `+0.0` and the output reaches
//! exactly `+0.0`.

use builtins::test_support::{chain_input, input_state_words};
use builtins::{BuiltinChain, BuiltinParameters, DualMonoBlock, builtin_filter_cutoff_maximum_hz};

const RATE: u32 = 44_100;
const FRAMES: usize = 128;
/// The bound the issue states: rest within 2,400,000 samples of the impulse.
const REST_BOUND_SAMPLES: usize = 2_400_000;
/// Blocks rendered after rest that must stay at rest.
const HELD_BLOCKS: usize = 4;

fn at_rest(chain: &BuiltinChain, left: &[f32], right: &[f32]) -> bool {
    input_state_words(chain_input(chain)) == [0; 8]
        && left.iter().chain(right).all(|sample| sample.to_bits() == 0)
}

#[test]
fn the_input_lpf_at_the_cutoff_maximum_reaches_exact_rest() {
    let cutoff = builtin_filter_cutoff_maximum_hz(RATE).expect("a launch rate");
    let mut parameters = BuiltinParameters::default();
    parameters.left.lpf_hz = cutoff;
    parameters.right.lpf_hz = cutoff;
    let mut chain = BuiltinChain::new(RATE, parameters).expect("the domain maximum prepares");

    let mut left = [0.0_f32; FRAMES];
    let mut right = [0.0_f32; FRAMES];
    left[0] = 1234.5;
    right[0] = 1234.5;
    let mut rest_at = None;
    let mut last_peak = 0.0_f32;
    let mut block = 0_usize;
    while block * FRAMES < REST_BOUND_SAMPLES {
        let first_sample = (block * FRAMES) as u64;
        chain.process_dual_mono(
            DualMonoBlock::new(&mut left, &mut right, first_sample).expect("block"),
        );
        if block > 0 && at_rest(&chain, &left, &right) {
            rest_at = Some(block);
            break;
        }
        last_peak = left
            .iter()
            .chain(&right)
            .fold(0.0, |peak, x| peak.max(x.abs()));
        left.fill(0.0);
        right.fill(0.0);
        block += 1;
    }
    let rest_at = rest_at.unwrap_or_else(|| {
        panic!(
            "no exact rest within {REST_BOUND_SAMPLES} samples: state {:08x?}, last block's \
             output peak {last_peak:e}",
            input_state_words(chain_input(&chain)),
        )
    });
    for held in 1..=HELD_BLOCKS {
        left.fill(0.0);
        right.fill(0.0);
        let first_sample = ((rest_at + held) * FRAMES) as u64;
        chain.process_dual_mono(
            DualMonoBlock::new(&mut left, &mut right, first_sample).expect("block"),
        );
        assert!(
            at_rest(&chain, &left, &right),
            "rest reached at block {rest_at} must hold, broke {held} blocks later"
        );
    }
}
