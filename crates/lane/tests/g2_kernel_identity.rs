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

use lane::kernels::{
    SvfState, mix2x2_block, ordered_accumulate_block, sum_into_block, sum2_block, svf_step,
};
use lane::{Lane, Simd4, Simd8, flush};
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

fn check_ordered_accumulation<L: Lane>() {
    for len in [
        1,
        L::WIDTH.saturating_sub(1).max(1),
        L::WIDTH,
        L::WIDTH + 3,
        128,
    ] {
        for count in [1, 3, 8] {
            let contributors: Vec<Vec<f32>> = (0..count)
                .map(|lane| {
                    (0..len)
                        .map(|frame| match (lane + frame) % 7 {
                            0 => -0.0,
                            1 => f32::from_bits(1),
                            2 => 16_777_216.0,
                            3 => 1.0,
                            4 => -16_777_216.0,
                            5 => f32::INFINITY,
                            _ => f32::from_bits(0x7fc0_4190),
                        })
                        .collect()
                })
                .collect();
            let refs: Vec<&[f32]> = contributors.iter().map(Vec::as_slice).collect();
            for initial in [true, false] {
                let mut expected = vec![16777216.0; len];
                if initial {
                    expected.copy_from_slice(&contributors[0]);
                }
                for input in &refs[usize::from(initial)..] {
                    sum_into_block::<L>(&mut expected, input);
                }
                let mut actual = vec![16777216.0; len];
                assert!(ordered_accumulate_block::<L>(&mut actual, &refs, initial));
                assert_eq!(
                    block_bits(&actual),
                    block_bits(&expected),
                    "width {} len {len} count {count} initial {initial}",
                    L::WIDTH
                );
            }
        }
    }

    let one = vec![1.0; L::WIDTH + 1];
    let negative = vec![-16_777_216.0; L::WIDTH + 1];
    let refs: [&[f32]; 2] = [&one, &negative];
    let mut actual = vec![16_777_216.0; L::WIDTH + 1];
    assert!(ordered_accumulate_block::<L>(&mut actual, &refs, false));
    assert!(actual.iter().all(|value| value.to_bits() == 0));
    let wrong = 16_777_216.0_f32 + (1.0_f32 + -16_777_216.0_f32);
    assert_eq!(wrong.to_bits(), 1.0_f32.to_bits());

    let minus_zero = vec![-0.0; L::WIDTH + 1];
    let mut initial = vec![f32::from_bits(0x7fc0_4190); L::WIDTH + 1];
    assert!(ordered_accumulate_block::<L>(
        &mut initial,
        &[&minus_zero],
        true
    ));
    assert!(initial.iter().all(|value| value.to_bits() == 0x8000_0000));

    for inputs in [
        Vec::<&[f32]>::new(),
        vec![&one[..]; 9],
        vec![&one[..], &one[..one.len() - 1]],
    ] {
        let mut poisoned = vec![f32::from_bits(0x7fc0_4190); one.len()];
        let before = block_bits(&poisoned);
        assert!(!ordered_accumulate_block::<L>(
            &mut poisoned,
            &inputs,
            false
        ));
        assert_eq!(block_bits(&poisoned), before);
    }
}

#[test]
fn ordered_accumulation_matches_the_existing_d9_primitives_and_rejects_shapes() {
    check_ordered_accumulation::<f32>();
    check_ordered_accumulation::<Simd4>();
    check_ordered_accumulation::<Simd8>();
}

fn caught(f: impl FnOnce() + std::panic::UnwindSafe) -> bool {
    std::panic::catch_unwind(f).is_err()
}

fn lane2_value(index: usize) -> f32 {
    const VALUES: [f32; 10] = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        16_777_216.0,
        -16_777_216.0,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    VALUES[index % VALUES.len()]
}

fn check_lane2_bounds_and_identity<L: Lane>() {
    let hostile: Vec<f32> = (0..10).map(lane2_value).collect();
    assert!(hostile.iter().any(|value| value.to_bits() == 0));
    assert!(hostile.iter().any(|value| value.to_bits() == 0x8000_0000));
    assert!(hostile.iter().any(|value| value.is_subnormal()));
    assert!(hostile.iter().any(|value| value.is_normal()));
    assert!(hostile.iter().any(|value| value.is_infinite()));

    let mut lengths = vec![
        0,
        1,
        L::WIDTH.saturating_sub(1),
        L::WIDTH,
        L::WIDTH + 1,
        2 * L::WIDTH + 1,
    ];
    lengths.sort_unstable();
    lengths.dedup();
    for len in lengths {
        let a: Vec<f32> = (0..len).map(lane2_value).collect();
        let b: Vec<f32> = (0..len).map(|index| lane2_value(index + 3)).collect();

        let expected_sum: Vec<u32> = a
            .iter()
            .zip(&b)
            .map(|(a, b)| <f32 as Lane>::add(*a, *b).to_bits())
            .collect();
        let mut sum = vec![f32::from_bits(0x7fc0_4630); len];
        sum2_block::<L>(&mut sum, &a, &b);
        assert_eq!(
            block_bits(&sum),
            expected_sum,
            "sum2 width {} len {len}",
            L::WIDTH
        );

        let mut expected_acc = a.clone();
        for (acc, x) in expected_acc.iter_mut().zip(&b) {
            *acc = <f32 as Lane>::add(*acc, *x);
        }
        let mut acc = a.clone();
        sum_into_block::<L>(&mut acc, &b);
        assert_eq!(
            block_bits(&acc),
            block_bits(&expected_acc),
            "sum_into width {} len {len}",
            L::WIDTH
        );

        let mut expected_left = a.clone();
        let mut expected_right = b.clone();
        for (left, right) in expected_left.iter_mut().zip(&mut expected_right) {
            let old_left = *left;
            let old_right = *right;
            *left = <f32 as Lane>::fma(-0.1, old_right, <f32 as Lane>::mul(0.9, old_left));
            *right = <f32 as Lane>::fma(0.8, old_right, <f32 as Lane>::mul(0.2, old_left));
        }
        let mut left = a;
        let mut right = b;
        mix2x2_block::<L>(&mut left, &mut right, [0.9, -0.1, 0.2, 0.8]);
        assert_eq!(
            block_bits(&left),
            block_bits(&expected_left),
            "matrix left width {} len {len}",
            L::WIDTH
        );
        assert_eq!(
            block_bits(&right),
            block_bits(&expected_right),
            "matrix right width {} len {len}",
            L::WIDTH
        );
    }

    let len = L::WIDTH + 1;
    let short = vec![1.0; len - 1];
    let full = vec![2.0; len];
    let poison = f32::from_bits(0x7fc0_4630);
    for first_short in [true, false] {
        let mut out = vec![poison; len];
        let before = block_bits(&out);
        let panicked = caught(std::panic::AssertUnwindSafe(|| {
            let (a, b) = if first_short {
                (&short[..], &full[..])
            } else {
                (&full[..], &short[..])
            };
            sum2_block::<L>(&mut out, a, b);
        }));
        assert!(panicked);
        assert_eq!(block_bits(&out), before, "sum2 must reject before writing");
    }
    let mut acc = vec![poison; len];
    let before = block_bits(&acc);
    assert!(caught(std::panic::AssertUnwindSafe(
        || sum_into_block::<L>(&mut acc, &short)
    )));
    assert_eq!(
        block_bits(&acc),
        before,
        "sum_into must reject before writing"
    );
    let mut left = vec![poison; len];
    let mut right = short.clone();
    let before_left = block_bits(&left);
    let before_right = block_bits(&right);
    assert!(caught(std::panic::AssertUnwindSafe(|| mix2x2_block::<L>(
        &mut left,
        &mut right,
        [0.9, -0.1, 0.2, 0.8]
    ))));
    assert_eq!(
        block_bits(&left),
        before_left,
        "matrix must reject before writing left"
    );
    assert_eq!(
        block_bits(&right),
        before_right,
        "matrix must reject before writing right"
    );

    let long = vec![1.0; len + 1];
    let mut sum_long_panicked = false;
    for first_long in [true, false] {
        let mut out = vec![poison; len];
        sum_long_panicked |= caught(std::panic::AssertUnwindSafe(|| {
            let (a, b) = if first_long {
                (&long[..], &full[..])
            } else {
                (&full[..], &long[..])
            };
            sum2_block::<L>(&mut out, a, b);
        }));
        if !cfg!(debug_assertions) {
            assert!(out.iter().all(|value| value.to_bits() == 3.0_f32.to_bits()));
        }
    }
    let mut acc = vec![1.0; len];
    let acc_long_panicked = caught(std::panic::AssertUnwindSafe(|| {
        sum_into_block::<L>(&mut acc, &long)
    }));
    let mut left = vec![1.0_f32; len];
    let mut right = vec![2.0_f32; len + 1];
    let suffix = right[len].to_bits();
    let matrix_long_panicked = caught(std::panic::AssertUnwindSafe(|| {
        mix2x2_block::<L>(&mut left, &mut right, [0.9, -0.1, 0.2, 0.8])
    }));
    if cfg!(debug_assertions) {
        assert!(sum_long_panicked && acc_long_panicked && matrix_long_panicked);
    } else {
        assert!(!sum_long_panicked && !acc_long_panicked && !matrix_long_panicked);
        assert!(acc.iter().all(|value| value.to_bits() == 2.0_f32.to_bits()));
        assert_eq!(
            right[len].to_bits(),
            suffix,
            "matrix right excess suffix changed"
        );
    }

    let longer_than_zero = [1.0_f32];
    let zero_sum_panicked = caught(|| {
        let mut out = [];
        sum2_block::<L>(&mut out, &longer_than_zero, &longer_than_zero);
    });
    let zero_acc_panicked = caught(|| {
        let mut acc = [];
        sum_into_block::<L>(&mut acc, &longer_than_zero);
    });
    let zero_matrix_panicked = caught(|| {
        let mut left = [];
        let mut right = longer_than_zero;
        mix2x2_block::<L>(&mut left, &mut right, [0.9, -0.1, 0.2, 0.8]);
        assert_eq!(right[0].to_bits(), 1.0_f32.to_bits());
    });
    assert_eq!(
        (zero_sum_panicked, zero_acc_panicked, zero_matrix_panicked),
        if cfg!(debug_assertions) {
            (true, true, true)
        } else {
            (false, false, false)
        },
        "zero controlling spans preserve debug equality and release prefix behavior"
    );

    let nan = f32::from_bits(0x7fc0_4630);
    let mut nan_out = [0.0];
    sum2_block::<L>(&mut nan_out, &[nan], &[1.0]);
    assert!(nan_out[0].is_nan(), "NaN case must be exercised separately");
}

#[test]
fn lane2_kernels_preserve_original_words_and_reject_short_inputs_before_writing() {
    check_lane2_bounds_and_identity::<f32>();
    check_lane2_bounds_and_identity::<Simd4>();
    check_lane2_bounds_and_identity::<Simd8>();
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

/// [`svf_cascade_interleaved`] is a chain of [`svf_block`] calls, bit for bit (issue #163 phase 3).
///
/// The interleaved kernel exists to hide recurrence latency, and its whole claim is that it does
/// so *without touching a single lane's operation sequence*: merging the loops of independent
/// chains, and keeping a cascade's intermediate output in a register instead of round-tripping it
/// through the block, must not move one bit. This gate is that claim stated against the reference
/// shape the parametric EQ ran before phase 3 -- four serial `svf_block` passes per channel, each
/// writing its output back into the block for the next pass to read.
///
/// It is checked at every width, at every legal depth (1, 2 and 4 all divide a four-section
/// cascade), on every corpus signal, and with a distinct coefficient set and a seeded non-zero
/// state in each of the eight `(channel, section)` slots -- so a body that leaked one chain's state
/// or coefficients into another's, or that ran the sections in the wrong order, cannot pass.
///
/// Red mutations: swap two sections' coefficient sets inside the frame body; write
/// `state[stream][section]` back to `state[stream][0]`; reassociate the output mix.
#[test]
fn g2_interleaved_cascade_equals_a_chain_of_blocks() {
    for signal in ALL_SIGNALS {
        let lanes = lane_signals(*signal);
        check_cascade::<f32>("Scalar", *signal, &lanes);
        check_cascade::<Simd4>("Simd4", *signal, &lanes);
        check_cascade::<Simd8>("Simd8", *signal, &lanes);
    }
}

/// Sections in the cascade this gate models: the parametric EQ's.
const CASCADE_SECTIONS: usize = 4;

/// A distinct coefficient set per `(channel, section)` slot, from the frozen G2 set.
fn cascade_coefficients<L: Lane>(slot: usize) -> lane::kernels::SvfCoef<L> {
    const BASE: [f32; 6] = [0.088_412_71, 0.059_749_45, 0.003_916_28, 0.25, -0.5, 1.0];
    let detune = 1.0 + (slot % 7) as f32 * 0.05;
    lane::kernels::SvfCoef {
        c1: L::splat(BASE[0] * detune),
        a2: L::splat(BASE[1] * detune),
        a3: L::splat(BASE[2] * detune),
        m0: L::splat(BASE[3]),
        m1: L::splat(BASE[4] * detune),
        m2: L::splat(BASE[5] * detune),
    }
}

/// A seeded, non-zero starting state per slot, so a body that dropped the incoming state passes
/// nothing.
fn cascade_state<L: Lane>(slot: usize) -> SvfState<L> {
    SvfState {
        ic1: L::splat(1.0e-3 * (slot as f32 + 1.0)),
        ic2: L::splat(-2.0e-3 * (slot as f32 + 1.0)),
    }
}

/// Runs both reference and interleaved forms at one width and asserts bit equality of the audio
/// **and** of every integrator word left behind.
fn check_cascade<L: Lane>(width: &str, signal: Signal, lanes: &[Vec<f32>]) {
    use lane::kernels::{SvfCoef, svf_block};

    let coefficients: [[SvfCoef<L>; CASCADE_SECTIONS]; 2] = core::array::from_fn(|channel| {
        core::array::from_fn(|section| cascade_coefficients(channel * CASCADE_SECTIONS + section))
    });
    // The second channel gets the lane signals rotated, so the two channels never carry the same
    // audio and a body that mixed them up cannot pass.
    let rotated: Vec<Vec<f32>> = lanes[1..]
        .iter()
        .chain(lanes[..1].iter())
        .cloned()
        .collect();
    let blocks: [Vec<f32>; 2] = [
        interleave(lanes, L::WIDTH, FRAMES),
        interleave(&rotated, L::WIDTH, FRAMES),
    ];

    // Reference: four serial whole-block passes per channel, exactly as the EQ ran before #163
    // phase 3.
    let mut reference = blocks.clone();
    let mut reference_state: [[SvfState<L>; CASCADE_SECTIONS]; 2] =
        core::array::from_fn(|channel| {
            core::array::from_fn(|section| cascade_state(channel * CASCADE_SECTIONS + section))
        });
    for channel in 0..2 {
        for section in 0..CASCADE_SECTIONS {
            svf_block::<L>(
                &mut reference[channel],
                FRAMES,
                &coefficients[channel][section],
                &mut reference_state[channel][section],
            );
        }
    }

    for depth in [1_usize, 2, 4] {
        let mut audio = blocks.clone();
        let mut state: [[SvfState<L>; CASCADE_SECTIONS]; 2] = core::array::from_fn(|channel| {
            core::array::from_fn(|section| cascade_state(channel * CASCADE_SECTIONS + section))
        });
        match depth {
            1 => run_cascade::<L, 1>(&mut audio, &coefficients, &mut state),
            2 => run_cascade::<L, 2>(&mut audio, &coefficients, &mut state),
            _ => run_cascade::<L, 4>(&mut audio, &coefficients, &mut state),
        }
        for channel in 0..2 {
            assert_eq!(
                block_bits(&audio[channel]),
                block_bits(&reference[channel]),
                "G2: interleaved cascade at {width} depth {depth} moved audio on channel \
                 {channel} of {}",
                signal.name()
            );
            for section in 0..CASCADE_SECTIONS {
                let (left, right) = (state[channel][section], reference_state[channel][section]);
                assert_eq!(
                    (bits::<L>(left.ic1), bits::<L>(left.ic2)),
                    (bits::<L>(right.ic1), bits::<L>(right.ic2)),
                    "G2: interleaved cascade at {width} depth {depth} moved the integrators of \
                     section {section}, channel {channel}, on {}",
                    signal.name()
                );
            }
        }
    }
}

/// Drives [`svf_cascade_interleaved`] over a whole cascade at one depth.
fn run_cascade<L: Lane, const DEPTH: usize>(
    audio: &mut [Vec<f32>; 2],
    coefficients: &[[lane::kernels::SvfCoef<L>; CASCADE_SECTIONS]; 2],
    state: &mut [[SvfState<L>; CASCADE_SECTIONS]; 2],
) {
    use lane::kernels::{SvfCoef, svf_cascade_interleaved};

    let (left, right) = audio.split_at_mut(1);
    for pass in 0..CASCADE_SECTIONS / DEPTH {
        let base = pass * DEPTH;
        let pass_coefficients: [[SvfCoef<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| coefficients[0][base + k]),
            core::array::from_fn(|k| coefficients[1][base + k]),
        ];
        let mut pass_state: [[SvfState<L>; DEPTH]; 2] = [
            core::array::from_fn(|k| state[0][base + k]),
            core::array::from_fn(|k| state[1][base + k]),
        ];
        svf_cascade_interleaved::<L, 2, DEPTH>(
            [&mut left[0], &mut right[0]],
            FRAMES,
            &pass_coefficients,
            &mut pass_state,
        );
        for k in 0..DEPTH {
            state[0][base + k] = pass_state[0][k];
            state[1][base + k] = pass_state[1][k];
        }
    }
}

/// The raw bits of a whole AoSoA block.
fn block_bits(block: &[f32]) -> Vec<u32> {
    block.iter().map(|sample| sample.to_bits()).collect()
}

/// The raw bits of every lane of one vector.
fn bits<L: Lane>(value: L) -> Vec<u32> {
    let mut words = vec![0.0f32; L::WIDTH];
    value.store(&mut words);
    words.iter().map(|sample| sample.to_bits()).collect()
}
