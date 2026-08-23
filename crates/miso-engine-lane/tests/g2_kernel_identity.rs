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

use miso_engine_lane::kernels::{SvfState, svf_step};
use miso_engine_lane::{Lane, Simd4, Simd8, flush};
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

/// [`svf_step`] delivers both taps of **one** state.
///
/// The audit of the multiband compressor (#94 F4) found four SVF sections where two suffice,
/// because the section that produces the low-pass tap also produces the band-pass tap that forms
/// the all-pass `x - 2k*v1`. This gate is that claim, stated as bits.
///
/// The oracle is a transcription of Simper's recurrence written here, in the test, from the
/// equations — not `svf_block`, which is *defined* by [`svf_step`] and would therefore agree with
/// any mutation of it. It runs one lane at a time, so it also proves that neither tap depends on
/// the width. Non-vacuity is asserted too: the two taps must actually differ, or a body that
/// returned the same value twice would pass.
///
/// Red mutation: return `(v2, v1)` from `svf_step`; swap `a2` and `a3` in its `d2`; drop one of
/// the two `flush` calls.
#[test]
fn g2_svf_step_yields_both_taps_of_one_state() {
    // A 1 kHz Butterworth low-pass at 48 kHz: g = tan(pi * 1000 / 48000), k = sqrt(2),
    // t = g * (g + k), c1 = t / (1 + t), a2 = g * (1 - c1), a3 = g * a2.
    const C1: f32 = 0.086_269_25;
    const A2: f32 = 0.059_915_63;
    const A3: f32 = 0.003_927_913_5;
    const FRAMES: usize = 4_096;

    /// Simper's recurrence, transcribed from the equations, one scalar lane at a time.
    fn oracle(input: &[f32], stride: usize, lane: usize) -> (Vec<u32>, Vec<u32>) {
        let (mut ic1, mut ic2) = (0.0f32, 0.0f32);
        let mut band = Vec::with_capacity(FRAMES);
        let mut low = Vec::with_capacity(FRAMES);
        for frame in 0..FRAMES {
            let v0 = input[frame * stride + lane];
            let v3 = v0 - ic2;
            let d1 = <f32 as Lane>::fma(-C1, ic1, A2 * v3);
            let v1 = ic1 + d1;
            let d2 = <f32 as Lane>::fma(A3, v3, A2 * ic1);
            let v2 = ic2 + d2;
            ic1 = flush(ic1 + (d1 + d1));
            ic2 = flush(ic2 + (d2 + d2));
            band.push(v1.to_bits());
            low.push(v2.to_bits());
        }
        (band, low)
    }

    fn check<L: Lane>() {
        let mut input = vec![0.0f32; FRAMES * L::WIDTH];
        Signal::Noise.fill(&mut input, 0x5F5F_0001);
        let mut state = SvfState::<L>::default();
        let nc1 = L::splat(C1).neg();
        let (a2, a3) = (L::splat(A2), L::splat(A3));
        let mut band = vec![0u32; FRAMES * L::WIDTH];
        let mut low = vec![0u32; FRAMES * L::WIDTH];
        for frame in 0..FRAMES {
            let (v1, v2) =
                svf_step::<L>(L::load(&input[frame * L::WIDTH..]), nc1, a2, a3, &mut state);
            v1.store_bits(&mut band[frame * L::WIDTH..]);
            v2.store_bits(&mut low[frame * L::WIDTH..]);
        }
        for lane in 0..L::WIDTH {
            let (expected_band, expected_low) = oracle(&input, L::WIDTH, lane);
            let mut differing = 0usize;
            for frame in 0..FRAMES {
                assert_eq!(
                    band[frame * L::WIDTH + lane],
                    expected_band[frame],
                    "svf_step band-pass tap at width {}, lane {lane}, frame {frame}",
                    L::WIDTH
                );
                assert_eq!(
                    low[frame * L::WIDTH + lane],
                    expected_low[frame],
                    "svf_step low-pass tap at width {}, lane {lane}, frame {frame}",
                    L::WIDTH
                );
                if expected_band[frame] != expected_low[frame] {
                    differing += 1;
                }
            }
            assert!(
                differing > FRAMES / 2,
                "the two taps must differ: only {differing} of {FRAMES} frames do"
            );
        }
    }

    check::<f32>();
    check::<Simd4>();
    check::<Simd8>();
}
