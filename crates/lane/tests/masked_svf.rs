//! Assignment 1 gates for the additive masked SVF kernel entrypoints.

use lane::kernels::{
    SvfCoef, SvfCoefStep, SvfState, svf_block, svf_block_ramped, svf_block_ramped_with_dry_mask,
    svf_cascade_interleaved, svf_cascade_interleaved_with_dry_masks, svf_step,
};
use lane::{Lane, Simd4, Simd8};

const DEPTH: usize = 2;
const FRAMES: usize = 9;

fn mask<L: Lane>(dry: bool) -> L::Mask {
    if dry {
        L::zero().eq(L::zero())
    } else {
        L::zero().lt(L::zero())
    }
}

fn packed<L: Lane>(value: impl Fn(usize) -> f32) -> L {
    let mut values = [0.0_f32; 8];
    for (lane, slot) in values.iter_mut().enumerate().take(L::WIDTH) {
        *slot = value(lane);
    }
    L::load(&values[..L::WIDTH])
}

fn bits<L: Lane>(value: L) -> [u32; 8] {
    let mut result = [0_u32; 8];
    value.store_bits(&mut result);
    result
}

fn assert_block_bits(actual: &[f32], expected: &[f32], label: &str) {
    assert_eq!(actual.len(), expected.len(), "{label}: block length");
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{label}: sample {index}"
        );
    }
}

fn assert_state_bits<L: Lane>(actual: &SvfState<L>, expected: &SvfState<L>, label: &str) {
    assert_eq!(
        bits(actual.ic1),
        bits(expected.ic1),
        "{label}: first integrator"
    );
    assert_eq!(
        bits(actual.ic2),
        bits(expected.ic2),
        "{label}: second integrator"
    );
}

fn identity_coef<L: Lane>() -> SvfCoef<L> {
    SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::zero(),
        m1: L::zero(),
        m2: L::splat(1.0),
    }
}

fn highpass_zero_state<L: Lane>() -> SvfCoef<L> {
    SvfCoef {
        c1: L::zero(),
        a2: L::zero(),
        a3: L::zero(),
        m0: L::splat(1.0),
        m1: L::splat(-1.0),
        m2: L::splat(-1.0),
    }
}

fn scalar_coef(slot: usize, lane: usize) -> SvfCoef<f32> {
    let n = (slot * 5 + lane) as f32;
    SvfCoef {
        c1: 0.025 + n * 0.001,
        a2: 0.050 + n * 0.001,
        a3: 0.004 + n * 0.0002,
        m0: 0.20 + n * 0.005,
        m1: -0.30 + n * 0.004,
        m2: 0.70 - n * 0.003,
    }
}

fn packed_coef<L: Lane>(slot: usize) -> SvfCoef<L> {
    SvfCoef {
        c1: packed(|lane| scalar_coef(slot, lane).c1),
        a2: packed(|lane| scalar_coef(slot, lane).a2),
        a3: packed(|lane| scalar_coef(slot, lane).a3),
        m0: packed(|lane| scalar_coef(slot, lane).m0),
        m1: packed(|lane| scalar_coef(slot, lane).m1),
        m2: packed(|lane| scalar_coef(slot, lane).m2),
    }
}

fn scalar_state(slot: usize, lane: usize) -> SvfState<f32> {
    let n = (slot * 3 + lane) as f32;
    SvfState {
        ic1: if lane == 0 { -0.0 } else { 0.01 + n * 0.002 },
        ic2: if lane == 1 {
            f32::from_bits(1)
        } else {
            -0.02 - n * 0.001
        },
    }
}

fn packed_state<L: Lane>(slot: usize) -> SvfState<L> {
    SvfState {
        ic1: packed(|lane| scalar_state(slot, lane).ic1),
        ic2: packed(|lane| scalar_state(slot, lane).ic2),
    }
}

fn source<const S: usize>(width: usize) -> [Vec<f32>; S] {
    core::array::from_fn(|stream| {
        (0..FRAMES * width)
            .map(|index| {
                let frame = index / width;
                let lane = index % width;
                match (stream + frame + lane) % 7 {
                    0 => -0.0,
                    1 => f32::from_bits(1),
                    2 => 0.25,
                    3 => -0.75,
                    4 => 1.5,
                    5 => -2.0,
                    _ => 0.0,
                }
            })
            .collect()
    })
}

fn packed_coefs<L: Lane, const S: usize>() -> [[SvfCoef<L>; DEPTH]; S] {
    core::array::from_fn(|stream| {
        core::array::from_fn(|section| packed_coef(stream * DEPTH + section))
    })
}

fn packed_states<L: Lane, const S: usize>() -> [[SvfState<L>; DEPTH]; S] {
    core::array::from_fn(|stream| {
        core::array::from_fn(|section| packed_state(stream * DEPTH + section))
    })
}

fn false_cascade_equivalence<L: Lane, const S: usize>() {
    let mut old_io = source::<S>(L::WIDTH);
    let mut masked_io = old_io.clone();
    let coefficients = packed_coefs::<L, S>();
    let mut old_state = packed_states::<L, S>();
    let mut masked_state = old_state;
    let dry_masks = core::array::from_fn(|_| core::array::from_fn(|_| mask::<L>(false)));

    let mut old_iter = old_io.iter_mut();
    let old_blocks: [&mut [f32]; S] =
        core::array::from_fn(|_| old_iter.next().expect("stream").as_mut_slice());
    svf_cascade_interleaved::<L, S, DEPTH>(old_blocks, FRAMES, &coefficients, &mut old_state);
    let mut masked_iter = masked_io.iter_mut();
    let masked_blocks: [&mut [f32]; S] =
        core::array::from_fn(|_| masked_iter.next().expect("stream").as_mut_slice());
    svf_cascade_interleaved_with_dry_masks::<L, S, DEPTH>(
        masked_blocks,
        FRAMES,
        &coefficients,
        &mut masked_state,
        &dry_masks,
    );

    for stream in 0..S {
        assert_block_bits(
            &masked_io[stream],
            &old_io[stream],
            "all-false cascade output",
        );
        for section in 0..DEPTH {
            assert_state_bits(
                &masked_state[stream][section],
                &old_state[stream][section],
                "all-false cascade state",
            );
        }
    }
}

fn mixed_cascade_matches_scalar<L: Lane, const S: usize>() {
    let mut actual_io = source::<S>(L::WIDTH);
    let input = actual_io.clone();
    let coefficients = packed_coefs::<L, S>();
    let mut actual_state = packed_states::<L, S>();
    let dry_masks = core::array::from_fn(|stream| {
        core::array::from_fn(|section| {
            let mut flags = [false; 8];
            for (lane, flag) in flags.iter_mut().enumerate().take(L::WIDTH) {
                *flag = (stream + section + lane) % 3 == 0;
            }
            packed::<L>(|lane| if flags[lane] { 1.0 } else { 0.0 }).gt(L::zero())
        })
    });

    let mut actual_iter = actual_io.iter_mut();
    let actual_blocks: [&mut [f32]; S] =
        core::array::from_fn(|_| actual_iter.next().expect("stream").as_mut_slice());
    svf_cascade_interleaved_with_dry_masks::<L, S, DEPTH>(
        actual_blocks,
        FRAMES,
        &coefficients,
        &mut actual_state,
        &dry_masks,
    );

    for stream in 0..S {
        for lane_index in 0..L::WIDTH {
            let mut expected = (0..FRAMES)
                .map(|frame| input[stream][frame * L::WIDTH + lane_index])
                .collect::<Vec<_>>();
            let mut expected_state = [
                scalar_state(stream * DEPTH, lane_index),
                scalar_state(stream * DEPTH + 1, lane_index),
            ];
            for (section, state) in expected_state.iter_mut().enumerate().take(DEPTH) {
                let mut coefficient = scalar_coef(stream * DEPTH + section, lane_index);
                svf_block_ramped_with_dry_mask::<f32>(
                    &mut expected,
                    FRAMES,
                    &mut coefficient,
                    &SvfCoefStep::default(),
                    0,
                    state,
                    mask::<f32>((stream + section + lane_index) % 3 == 0),
                );
            }
            for (frame, expected) in expected.iter().enumerate() {
                assert_eq!(
                    actual_io[stream][frame * L::WIDTH + lane_index].to_bits(),
                    expected.to_bits(),
                    "mixed cascade output: width {}, stream {stream}, lane {lane_index}, frame {frame}",
                    L::WIDTH
                );
            }
            for section in 0..DEPTH {
                let actual_ic1 = bits(actual_state[stream][section].ic1)[lane_index];
                let actual_ic2 = bits(actual_state[stream][section].ic2)[lane_index];
                assert_eq!(actual_ic1, expected_state[section].ic1.to_bits());
                assert_eq!(actual_ic2, expected_state[section].ic2.to_bits());
            }
        }
    }
}

fn dry_state_matches_unmasked<L: Lane>() {
    let mut old_io = source::<1>(L::WIDTH)[0].clone();
    let mut dry_io = old_io.clone();
    let coefficient = identity_coef::<L>();
    let mut old_state = SvfState {
        ic1: packed(|lane| match lane % 3 {
            0 => -0.0,
            1 => f32::from_bits(1),
            _ => 0.125,
        }),
        ic2: packed(|lane| match lane % 3 {
            0 => f32::from_bits(1),
            1 => -0.25,
            _ => 0.5,
        }),
    };
    let mut dry_state = old_state;
    svf_block::<L>(&mut old_io, FRAMES, &coefficient, &mut old_state);
    let mut dry_coefficient = coefficient;
    svf_block_ramped_with_dry_mask::<L>(
        &mut dry_io,
        FRAMES,
        &mut dry_coefficient,
        &SvfCoefStep::default(),
        0,
        &mut dry_state,
        mask::<L>(true),
    );
    assert_block_bits(&dry_io, &source::<1>(L::WIDTH)[0], "dry output bits");
    assert_state_bits(&dry_state, &old_state, "executed dry recurrence");
    for (actual, expected) in [
        (dry_coefficient.c1, coefficient.c1),
        (dry_coefficient.a2, coefficient.a2),
        (dry_coefficient.a3, coefficient.a3),
        (dry_coefficient.m0, coefficient.m0),
        (dry_coefficient.m1, coefficient.m1),
        (dry_coefficient.m2, coefficient.m2),
    ] {
        assert_eq!(bits(actual), bits(expected), "stationary dry coefficients");
    }
}

fn downstream_signed_zero<L: Lane>() {
    let mut unmasked = vec![-0.0_f32; L::WIDTH];
    let mut masked = unmasked.clone();
    let coefficients = [[identity_coef::<L>(), highpass_zero_state::<L>()]];
    let mut unmasked_state = [[SvfState::<L>::default(); DEPTH]; 1];
    let mut masked_state = unmasked_state;
    let dry_masks = [[mask::<L>(true), mask::<L>(false)]];
    svf_cascade_interleaved::<L, 1, DEPTH>([&mut unmasked], 1, &coefficients, &mut unmasked_state);
    svf_cascade_interleaved_with_dry_masks::<L, 1, DEPTH>(
        [&mut masked],
        1,
        &coefficients,
        &mut masked_state,
        &dry_masks,
    );
    assert!(unmasked.iter().all(|sample| sample.to_bits() == 0));
    assert!(
        masked
            .iter()
            .all(|sample| sample.to_bits() == (-0.0_f32).to_bits())
    );
}

fn scalar_step_coef(coefficient: &mut SvfCoef<f32>, step: &SvfCoefStep<f32>) {
    coefficient.c1 += step.c1;
    coefficient.a2 += step.a2;
    coefficient.a3 += step.a3;
    coefficient.m0 += step.m0;
    coefficient.m1 += step.m1;
    coefficient.m2 += step.m2;
}

fn scalar_step(lane_index: usize) -> SvfCoefStep<f32> {
    if lane_index == 1 {
        return SvfCoefStep::default();
    }
    let n = lane_index as f32;
    SvfCoefStep {
        c1: 0.0007 + n * 0.00003,
        a2: -0.0004 + n * 0.00002,
        a3: 0.0002 + n * 0.00001,
        m0: 0.003 + n * 0.0001,
        m1: -0.002 + n * 0.00007,
        m2: 0.001 + n * 0.00005,
    }
}

fn packed_step<L: Lane>() -> SvfCoefStep<L> {
    SvfCoefStep {
        c1: packed(|lane| scalar_step(lane).c1),
        a2: packed(|lane| scalar_step(lane).a2),
        a3: packed(|lane| scalar_step(lane).a3),
        m0: packed(|lane| scalar_step(lane).m0),
        m1: packed(|lane| scalar_step(lane).m1),
        m2: packed(|lane| scalar_step(lane).m2),
    }
}

fn ramp_oracle(
    input: &[f32],
    mut coefficient: SvfCoef<f32>,
    step: SvfCoefStep<f32>,
    ramp_frames: usize,
    state: &mut SvfState<f32>,
    dry: bool,
) -> (Vec<u32>, SvfCoef<f32>) {
    let mut output = Vec::with_capacity(input.len());
    for (index, &v0) in input.iter().enumerate() {
        let (v1, v2) = svf_step(v0, -coefficient.c1, coefficient.a2, coefficient.a3, state);
        let wet = <f32 as Lane>::fma(
            coefficient.m2,
            v2,
            <f32 as Lane>::fma(coefficient.m1, v1, <f32 as Lane>::mul(coefficient.m0, v0)),
        );
        output.push(<f32 as Lane>::select(mask::<f32>(dry), v0, wet).to_bits());
        if index < ramp_frames {
            scalar_step_coef(&mut coefficient, &step);
        }
    }
    (output, coefficient)
}

fn ramped_masks_and_partitions<L: Lane>() {
    let input = source::<1>(L::WIDTH)[0].clone();
    let step = packed_step::<L>();
    let initial_coefficient = packed_coef::<L>(11);
    let initial_state = packed_state::<L>(11);
    let mut actual = input.clone();
    let mut coefficient = initial_coefficient;
    let mut state = initial_state;
    let mut flags = [false; 8];
    for (lane, flag) in flags.iter_mut().enumerate().take(L::WIDTH) {
        *flag = lane % 3 == 0;
    }
    let dry_mask = packed::<L>(|lane| if flags[lane] { 1.0 } else { 0.0 }).gt(L::zero());

    let mut old = input.clone();
    let mut false_mask = input.clone();
    let mut old_coefficient = initial_coefficient;
    let mut false_coefficient = initial_coefficient;
    let mut old_state = initial_state;
    let mut false_state = initial_state;
    svf_block_ramped(
        &mut old,
        FRAMES,
        &mut old_coefficient,
        &step,
        5,
        &mut old_state,
    );
    svf_block_ramped_with_dry_mask(
        &mut false_mask,
        FRAMES,
        &mut false_coefficient,
        &step,
        5,
        &mut false_state,
        mask::<L>(false),
    );
    assert_block_bits(&false_mask, &old, "all-false ramp output");
    assert_state_bits(&false_state, &old_state, "all-false ramp state");
    assert_eq!(bits(false_coefficient.c1), bits(old_coefficient.c1));
    assert_eq!(bits(false_coefficient.a2), bits(old_coefficient.a2));
    assert_eq!(bits(false_coefficient.a3), bits(old_coefficient.a3));
    assert_eq!(bits(false_coefficient.m0), bits(old_coefficient.m0));
    assert_eq!(bits(false_coefficient.m1), bits(old_coefficient.m1));
    assert_eq!(bits(false_coefficient.m2), bits(old_coefficient.m2));

    svf_block_ramped_with_dry_mask::<L>(
        &mut actual,
        FRAMES,
        &mut coefficient,
        &step,
        5,
        &mut state,
        dry_mask,
    );
    for lane_index in 0..L::WIDTH {
        let lane_input = (0..FRAMES)
            .map(|frame| input[frame * L::WIDTH + lane_index])
            .collect::<Vec<_>>();
        let (expected, expected_coefficient) = ramp_oracle(
            &lane_input,
            scalar_coef(11, lane_index),
            scalar_step(lane_index),
            5,
            &mut scalar_state(11, lane_index),
            flags[lane_index],
        );
        for (frame, expected) in expected.iter().enumerate() {
            assert_eq!(
                actual[frame * L::WIDTH + lane_index].to_bits(),
                *expected,
                "ramped output: width {}, lane {lane_index}, frame {frame}",
                L::WIDTH
            );
        }
        let actual_c = [
            bits(coefficient.c1)[lane_index],
            bits(coefficient.a2)[lane_index],
            bits(coefficient.a3)[lane_index],
            bits(coefficient.m0)[lane_index],
            bits(coefficient.m1)[lane_index],
            bits(coefficient.m2)[lane_index],
        ];
        let expected_c = [
            expected_coefficient.c1.to_bits(),
            expected_coefficient.a2.to_bits(),
            expected_coefficient.a3.to_bits(),
            expected_coefficient.m0.to_bits(),
            expected_coefficient.m1.to_bits(),
            expected_coefficient.m2.to_bits(),
        ];
        assert_eq!(actual_c, expected_c, "ramped final coefficients");
    }

    let mut whole = input.clone();
    let mut whole_coefficient = initial_coefficient;
    let mut whole_state = initial_state;
    let mut split = input;
    let mut split_coefficient = initial_coefficient;
    let mut split_state = initial_state;
    svf_block_ramped_with_dry_mask::<L>(
        &mut whole,
        FRAMES,
        &mut whole_coefficient,
        &step,
        5,
        &mut whole_state,
        dry_mask,
    );
    svf_block_ramped_with_dry_mask::<L>(
        &mut split[..3 * L::WIDTH],
        3,
        &mut split_coefficient,
        &step,
        3,
        &mut split_state,
        dry_mask,
    );
    svf_block_ramped_with_dry_mask::<L>(
        &mut split[3 * L::WIDTH..],
        FRAMES - 3,
        &mut split_coefficient,
        &step,
        2,
        &mut split_state,
        dry_mask,
    );
    assert_block_bits(&split, &whole, "split ramp equivalence");
    assert_state_bits(&split_state, &whole_state, "split ramp state");
    assert_eq!(bits(split_coefficient.c1), bits(whole_coefficient.c1));
    assert_eq!(bits(split_coefficient.a2), bits(whole_coefficient.a2));
    assert_eq!(bits(split_coefficient.a3), bits(whole_coefficient.a3));
    assert_eq!(bits(split_coefficient.m0), bits(whole_coefficient.m0));
    assert_eq!(bits(split_coefficient.m1), bits(whole_coefficient.m1));
    assert_eq!(bits(split_coefficient.m2), bits(whole_coefficient.m2));
}

fn run_width<L: Lane>() {
    false_cascade_equivalence::<L, 1>();
    false_cascade_equivalence::<L, 2>();
    mixed_cascade_matches_scalar::<L, 1>();
    mixed_cascade_matches_scalar::<L, 2>();
    dry_state_matches_unmasked::<L>();
    downstream_signed_zero::<L>();
    ramped_masks_and_partitions::<L>();
}

#[test]
fn masked_svf_assignment1_gates_all_widths() {
    run_width::<f32>();
    run_width::<Simd4>();
    run_width::<Simd8>();
}
