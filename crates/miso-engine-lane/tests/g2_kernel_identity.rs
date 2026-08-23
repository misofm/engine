//! Gate G2: a kernel body is width-independent.
//!
//! Master plan #83 §3.6. Every kernel of the `kernels` module is instantiated at `f32`, at
//! `Simd4` and at `Simd8` over the same eight per-lane signals, with the same coefficients and the
//! same seeded state, and the results must be bit-identical lane by lane. The scalar run is the
//! oracle: eight independent single-lane runs, one per lane, which is also what proves that no
//! lane leaks into another.
//!
//! Corpora: seeded noise, an impulse, DC, and a subnormal signal with a subnormal-seeded state
//! (the case where the D7 flush decides the bits).
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): reassociate `svf_block`'s first
//! fused multiply-add into a multiply and an add.

mod support;

use miso_engine_lane::{Lane, Simd4, Simd8};
use support::{
    ALL_KERNELS, ALL_SIGNALS, Kernel, MAX_WIDTH, Signal, deinterleave, interleave, run_kernel,
};

/// Frames per case. The `--release` count is the gate; a debug run keeps the workspace suite quick.
const FRAMES: usize = if cfg!(debug_assertions) {
    1_024
} else {
    16_384
};

/// Builds the eight per-lane signals of one case; each lane gets a different seed so that a
/// cross-lane leak cannot hide behind identical inputs.
fn lane_signals(signal: Signal) -> Vec<Vec<f32>> {
    (0..MAX_WIDTH)
        .map(|lane| {
            let mut samples = vec![0.0f32; FRAMES];
            signal.fill(&mut samples, 0x51ED_0000 + lane as u64);
            samples
        })
        .collect()
}

/// Runs one case at one width and returns the per-lane output bits.
fn run_at_width<L: Lane>(kernel: Kernel, signal: Signal, lanes: &[Vec<f32>]) -> Vec<Vec<u32>> {
    let width = L::WIDTH;
    let mut outputs = Vec::with_capacity(MAX_WIDTH);
    let mut first_lane = 0;
    while first_lane < MAX_WIDTH {
        let group: Vec<Vec<f32>> = lanes[first_lane..first_lane + width].to_vec();
        let mut block = interleave(&group, width, FRAMES);
        run_kernel::<L>(kernel, &mut block, FRAMES, signal.state_seed(), FRAMES);
        for lane in 0..width {
            outputs.push(
                deinterleave(&block, width, FRAMES, lane)
                    .into_iter()
                    .map(f32::to_bits)
                    .collect(),
            );
        }
        first_lane += width;
    }
    outputs
}

/// Compares one width against the scalar oracle, reporting the first difference.
fn compare(
    kernel: Kernel,
    signal: Signal,
    width_name: &str,
    oracle: &[Vec<u32>],
    actual: &[Vec<u32>],
) {
    for lane in 0..MAX_WIDTH {
        for frame in 0..FRAMES {
            assert_eq!(
                actual[lane][frame],
                oracle[lane][frame],
                "G2 {kernel} / {signal} at {width_name}: lane {lane}, frame {frame}: \
                 {actual:#010x} != oracle {oracle:#010x}",
                kernel = kernel.name(),
                signal = signal.name(),
                actual = actual[lane][frame],
                oracle = oracle[lane][frame],
            );
        }
    }
}

#[test]
fn g2_kernels_are_bit_identical_at_every_width() {
    for kernel in ALL_KERNELS {
        for signal in ALL_SIGNALS {
            let lanes = lane_signals(*signal);
            let oracle = run_at_width::<f32>(*kernel, *signal, &lanes);
            compare(
                *kernel,
                *signal,
                "Simd4",
                &oracle,
                &run_at_width::<Simd4>(*kernel, *signal, &lanes),
            );
            compare(
                *kernel,
                *signal,
                "Simd8",
                &oracle,
                &run_at_width::<Simd8>(*kernel, *signal, &lanes),
            );
        }
    }
}

#[test]
fn g2_idle_ramped_svf_equals_the_plain_svf() {
    // Amendment A2: `svf_block_ramped` with no ramp is `svf_block`, bit for bit, not merely close.
    for signal in ALL_SIGNALS {
        let lanes = lane_signals(*signal);
        let plain = run_at_width::<Simd8>(Kernel::SvfLow, *signal, &lanes);
        let idle = run_at_width::<Simd8>(Kernel::SvfRampedIdle, *signal, &lanes);
        assert_eq!(
            plain,
            idle,
            "G2: an idle ramped SVF must equal the plain SVF on {}",
            signal.name()
        );
    }
}

#[test]
fn g2_subnormal_state_is_flushed_at_every_width() {
    // Non-vacuity for the subnormal case: with a subnormal seed and a silent input the SVF state
    // has to reach exactly zero, at every width, rather than decaying through the subnormal range.
    let silence = vec![vec![0.0f32; FRAMES]; MAX_WIDTH];
    for width_outputs in [
        run_at_width::<f32>(Kernel::SvfLow, Signal::Subnormal, &silence),
        run_at_width::<Simd4>(Kernel::SvfLow, Signal::Subnormal, &silence),
        run_at_width::<Simd8>(Kernel::SvfLow, Signal::Subnormal, &silence),
    ] {
        for lane in width_outputs {
            assert_eq!(
                lane[FRAMES - 1],
                0,
                "G2: a subnormal-seeded state must flush to +0.0"
            );
        }
    }
}
