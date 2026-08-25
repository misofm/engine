//! B2: the issue #163 phase 3 sweep -- how many independent SVF recurrences a backend wants in
//! flight, and how they should be arranged.
//!
//! This is **not** a gate. It is the descriptive measurement that fixes
//! [`miso_engine_lane::Lane::SVF_CASCADE_DEPTH`], run the way B1 is run: one warmup, three
//! measured rounds, the minimum reported, nothing tuned and nothing retried. Its numbers are
//! quoted on that constant and in `artifacts/issue163-phase3/`.
//!
//! # The workload
//!
//! One parametric-EQ bank rendering one 128-frame block: a four-section TPT cascade over two
//! channels, with a distinct coefficient set in each of the eight slots (sharing one set across
//! chains would let an arm win by holding coefficients in registers that the real EQ keeps apart).
//! The **baseline** is the shape the EQ ran before phase 3 -- eight separate [`svf_block`] calls,
//! four per channel, each a full pass over the block. Every interleaved arm does exactly the same
//! arithmetic in exactly the same per-chain order; only the loop nesting differs. The ratio is
//! therefore the latency-hiding effect and nothing else.
//!
//! # The two axes
//!
//! * `S` -- independent **streams** interleaved. A bank has two of them, its channels. `S = 4` and
//!   `S = 8` are the *cross-bank* arms: they are what fusing two or four independent same-kernel
//!   banks into one loop would buy, and they are measured here so that the decision not to build
//!   that fusion is a number rather than an assumption.
//! * `D` -- cascade sections **fused** into one frame loop. Section `k`'s integrators depend on
//!   section `k`'s previous frame, never on section `k - 1`, so `D` sections are `D` more
//!   independent recurrences; section `k - 1`'s output feeds section `k` inside the same frame, so
//!   the cost is a longer forward chain in the frame body, not a longer loop-carried one.
//!
//! Run it explicitly:
//! `cargo test --release -p miso-engine-lane --test b2_interleave -- --ignored --nocapture`.

use std::hint::black_box;
use std::time::Instant;

use miso_engine_lane::kernels::{SvfCoef, SvfState, svf_block, svf_cascade_interleaved};
use miso_engine_lane::{Lane, Simd4, Simd8};

/// Cascade sections in a parametric EQ.
const SECTIONS: usize = 4;
/// Frames per block: the production quantum.
const FRAMES: usize = 128;
/// Blocks per measured round.
const BLOCKS: usize = 20_000;

/// Low-pass at 1 kHz, Q = 0.707, 48 kHz -- the G2 coefficient set.
const COEFFICIENTS: [f32; 6] = [0.088_412_71, 0.059_749_45, 0.003_916_28, 0.0, 0.0, 1.0];

/// A distinct, well-behaved coefficient set per cascade slot.
fn coefficients<L: Lane>(slot: usize) -> SvfCoef<L> {
    let detune = 1.0 + (slot % 7) as f32 * 0.05;
    SvfCoef {
        c1: L::splat(COEFFICIENTS[0] * detune),
        a2: L::splat(COEFFICIENTS[1] * detune),
        a3: L::splat(COEFFICIENTS[2] * detune),
        m0: L::splat(COEFFICIENTS[3]),
        m1: L::splat(COEFFICIENTS[4] * detune),
        m2: L::splat(COEFFICIENTS[5]),
    }
}

/// Bounded noise, so no chain ever leaves the normal range.
fn signal(len: usize) -> Vec<f32> {
    (0..len)
        .map(|index| ((index % 97) as f32 - 48.0) / 48.0)
        .collect()
}

/// Best of three rounds after one warmup, in nanoseconds per *bank-block equivalent*: one
/// four-section cascade over two channels of 128 frames, whatever the arm's stream count.
fn best(label: &str, streams: usize, mut arm: impl FnMut()) -> f64 {
    for _ in 0..2_000 {
        arm();
    }
    let mut rounds = [0.0_f64; 3];
    for round in &mut rounds {
        let start = Instant::now();
        for _ in 0..BLOCKS {
            arm();
        }
        let elapsed = start.elapsed();
        *round = elapsed.as_secs_f64() * 1.0e9 / (BLOCKS as f64 * streams as f64 / 2.0);
    }
    let value = rounds.iter().copied().fold(f64::INFINITY, f64::min);
    println!("B2 {label:<34}: {value:9.1} ns/bank-block   rounds {rounds:?}");
    value
}

/// One backend's whole sweep.
fn sweep<L: Lane>(name: &str) {
    let span = FRAMES * L::WIDTH;
    let source = signal(span);

    let baseline = {
        let coefficients: [[SvfCoef<L>; SECTIONS]; 2] = core::array::from_fn(|stream| {
            core::array::from_fn(|k| coefficients(stream * SECTIONS + k))
        });
        let mut state = [[SvfState::<L>::default(); SECTIONS]; 2];
        let (mut left, mut right) = (source.clone(), source.clone());
        best(&format!("{name} baseline: 8 serial blocks"), 2, || {
            for (stream, block) in [&mut left, &mut right].into_iter().enumerate() {
                for section in 0..SECTIONS {
                    svf_block::<L>(
                        black_box(block),
                        FRAMES,
                        &coefficients[stream][section],
                        &mut state[stream][section],
                    );
                }
            }
        })
    };

    // `S` streams, `D` sections per pass, `SECTIONS / D` passes over the block.
    macro_rules! cell {
        ($streams:literal, $depth:literal) => {{
            let passes = SECTIONS / $depth;
            let coefficients: [[[SvfCoef<L>; $depth]; $streams]; SECTIONS / $depth] =
                core::array::from_fn(|pass| {
                    core::array::from_fn(|stream| {
                        core::array::from_fn(|k| {
                            coefficients(stream * SECTIONS + pass * $depth + k)
                        })
                    })
                });
            let mut state = [[[SvfState::<L>::default(); $depth]; $streams]; SECTIONS / $depth];
            let mut blocks: [Vec<f32>; $streams] = core::array::from_fn(|_| source.clone());
            let value = best(
                &format!("{name} S={} D={}", $streams, $depth),
                $streams,
                || {
                    for pass in 0..passes {
                        let mut each = blocks.iter_mut();
                        let io: [&mut [f32]; $streams] =
                            core::array::from_fn(|_| each.next().expect("stream").as_mut_slice());
                        svf_cascade_interleaved::<L, $streams, $depth>(
                            black_box(io),
                            FRAMES,
                            &coefficients[pass],
                            &mut state[pass],
                        );
                    }
                },
            );
            println!(
                "B2 {name} S={} D={} speedup{:<9}: {:.3}x{}",
                $streams,
                $depth,
                "",
                baseline / value,
                if $streams > 2 {
                    "   (cross-bank: not built)"
                } else {
                    ""
                }
            );
        }};
    }

    cell!(2, 1);
    cell!(2, 2);
    cell!(2, 4);
    cell!(4, 1);
    cell!(4, 2);
    cell!(8, 1);
    cell!(8, 2);
    println!(
        "B2 {name} chosen depth              : {} (Lane::SVF_CASCADE_DEPTH)\n",
        L::SVF_CASCADE_DEPTH
    );
}

#[test]
#[ignore = "descriptive sweep, not a gate; run with --ignored --nocapture"]
fn b2_interleave_sweep() {
    println!("B2 workload: one EQ bank-block = {SECTIONS} sections x 2 channels x {FRAMES} frames");
    sweep::<f32>("scalar");
    sweep::<Simd4>("simd4 ");
    sweep::<Simd8>("simd8 ");
}
