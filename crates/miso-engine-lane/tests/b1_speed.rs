//! B1: descriptive speed of the block kernel against a per-sample call path.
//!
//! Master plan #83 §10, row B1. This is **not** a gate: it is one descriptive measurement, run
//! with one warmup and two measured rounds, of the shape the audit found (`fn`-pointer call per
//! sample per lane, with per-call validation) against the shape this crate introduces (one generic
//! block body at `Simd8`). Nothing is tuned or retried; if the number is disappointing, that is an
//! optimisation issue, not a change to this file.
//!
//! It is `#[ignore]`d so that it never runs inside `cargo test`; run it explicitly:
//! `cargo test --release -p miso-engine-lane --test b1_speed -- --ignored --nocapture`.
//!
//! One diagnostic use (master plan §11): if `Simd8` is not clearly faster than the per-sample path,
//! the `x86-64-v3` pin is probably not applied and `wide` has lowered `f32x8` to two SSE2 values.

mod support;

use std::hint::black_box;
use std::time::Instant;

use miso_engine_lane::kernels::{SvfCoef, SvfState, svf_block};
use miso_engine_lane::{Lane, Simd8, flush};

/// Frames per round.
const FRAMES: usize = 1_048_576;

/// Lanes per frame: the production bank width on `x86-64-v3`.
const WIDTH: usize = 8;

/// Low-pass coefficients at 1 kHz, Q = 0.707, 48 kHz (the G2 set).
const COEFFICIENTS: [f32; 6] = [0.088_412_71, 0.059_749_45, 0.003_916_28, 0.0, 0.0, 1.0];

/// One sample of the same filter, reached through a function pointer, as the deleted per-sample
/// kernels were: the call is opaque, so the state has to travel through memory every sample.
#[inline(never)]
fn svf_sample(io: &mut [f32], index: usize, coefficients: &[f32; 6], state: &mut [f32; 2]) {
    assert!(
        index < io.len(),
        "per-sample paths re-validate on every call"
    );
    let (c1, a2, a3, m0, m1, m2) = (
        coefficients[0],
        coefficients[1],
        coefficients[2],
        coefficients[3],
        coefficients[4],
        coefficients[5],
    );
    let (ic1, ic2) = (state[0], state[1]);
    let v0 = io[index];
    let v3 = v0 - ic2;
    let d1 = Lane::fma(Lane::neg(c1), ic1, a2 * v3);
    let v1 = ic1 + d1;
    let d2 = Lane::fma(a3, v3, a2 * ic1);
    let v2 = ic2 + d2;
    state[0] = flush(ic1 + (d1 + d1));
    state[1] = flush(ic2 + (d2 + d2));
    io[index] = Lane::fma(m2, v2, Lane::fma(m1, v1, m0 * v0));
}

/// Nanoseconds per frame of eight lanes through the generic block kernel.
fn block_round(block: &mut [f32]) -> f64 {
    let coefficients = SvfCoef::<Simd8> {
        c1: Simd8::splat(COEFFICIENTS[0]),
        a2: Simd8::splat(COEFFICIENTS[1]),
        a3: Simd8::splat(COEFFICIENTS[2]),
        m0: Simd8::splat(COEFFICIENTS[3]),
        m1: Simd8::splat(COEFFICIENTS[4]),
        m2: Simd8::splat(COEFFICIENTS[5]),
    };
    let mut state = SvfState::<Simd8>::default();
    let start = Instant::now();
    svf_block::<Simd8>(black_box(block), FRAMES, &coefficients, &mut state);
    let elapsed = start.elapsed();
    black_box(block[0]);
    elapsed.as_secs_f64() * 1.0e9 / FRAMES as f64
}

/// Nanoseconds per frame of eight lanes through the per-sample call path.
fn per_sample_round(block: &mut [f32]) -> f64 {
    type SampleFn = fn(&mut [f32], usize, &[f32; 6], &mut [f32; 2]);
    let call: SampleFn = svf_sample;
    let mut state = [[0.0f32; 2]; WIDTH];
    let start = Instant::now();
    for frame in 0..FRAMES {
        for (lane, lane_state) in state.iter_mut().enumerate() {
            call(
                black_box(block),
                frame * WIDTH + lane,
                &COEFFICIENTS,
                lane_state,
            );
        }
    }
    let elapsed = start.elapsed();
    black_box(block[0]);
    elapsed.as_secs_f64() * 1.0e9 / FRAMES as f64
}

#[test]
#[ignore = "descriptive benchmark, not a gate; run with --ignored --nocapture"]
fn b1_block_kernel_against_a_per_sample_path() {
    let mut block = vec![0.0f32; FRAMES * WIDTH];
    for (index, sample) in block.iter_mut().enumerate() {
        *sample = ((index % 97) as f32 - 48.0) / 48.0;
    }

    let mut warm = block.clone();
    let _ = block_round(&mut warm);
    let mut warm = block.clone();
    let _ = per_sample_round(&mut warm);

    let mut block_rounds = [0.0f64; 2];
    let mut sample_rounds = [0.0f64; 2];
    for round in 0..2 {
        let mut data = block.clone();
        block_rounds[round] = block_round(&mut data);
        let mut data = block.clone();
        sample_rounds[round] = per_sample_round(&mut data);
    }

    let block_best = block_rounds[0].min(block_rounds[1]);
    let sample_best = sample_rounds[0].min(sample_rounds[1]);
    println!("B1 svf_block  Simd8      : {block_rounds:?} ns per frame of {WIDTH} lanes");
    println!("B1 svf_sample per-sample : {sample_rounds:?} ns per frame of {WIDTH} lanes");
    println!(
        "B1 ratio                 : {:.2}x",
        sample_best / block_best
    );
    assert!(block_best > 0.0, "B1: the timer must advance");
}
