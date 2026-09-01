#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The input stage: coefficients, the recurrence, lane identity, the boundary check, partition
//! invariance and the signed-zero laws.
//!
//! Master plan #83 D5 is the claim these gates make together: for a given session, sample rate and
//! quantum the rendered bits are the same at every width, on every target, and under every block
//! partition. The oracle for the coefficients and the recurrence is
//! `dsp_reference::ReferenceRetainedTptF32`, hand-written from the equations and
//! never calling `lane`; the oracle for lane identity is the scalar `Lane`
//! instantiation, which is the same body at `WIDTH = 1` (D4).

use builtins::*;
use dsp_reference::{ReferenceRetainedTptF32, ReferenceTptOutput};
use effect_contract::BankWidth;
use engine::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES};
use lane::Backend;

/// Both bank widths, exercised on every host: `wide` implements four and eight lanes everywhere,
/// so a width difference cannot hide behind a target difference.
const BANKS: [(Backend, BankWidth); 2] = [
    (Backend::Simd4, BankWidth::Four),
    (Backend::Simd8, BankWidth::Eight),
];

fn launch_and_extended_compatibility_rates() -> impl Iterator<Item = u32> {
    LAUNCH_SAMPLE_RATES
        .into_iter()
        .chain(EXTENDED_COMPATIBILITY_SAMPLE_RATES)
        .map(|rate| rate.0)
}

/// Xorshift64\*: seeded and portable, so a gate's corpus never depends on a system generator.
struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    /// A finite sample in `[-1, 1)`, exactly representable and never subnormal.
    fn next_sample(&mut self) -> f32 {
        (self.next_u32() as i32 as f32) / 2_147_483_648.0
    }
}

fn parameters_for(index: usize) -> BuiltinParameters {
    BuiltinParameters {
        left: ChannelParameters {
            polarity_invert: index % 2 == 1,
            trim_db: index as f32 - 2.0,
            hpf_hz: 80.0 + index as f32 * 11.0,
            lpf_hz: 2_000.0 + index as f32 * 101.0,
            fader_db: 1.0 - index as f32,
            muted: index % 5 == 4,
        },
        right: ChannelParameters {
            polarity_invert: index % 3 == 1,
            trim_db: 2.0 - index as f32,
            hpf_hz: 120.0 + index as f32 * 13.0,
            lpf_hz: 3_000.0 + index as f32 * 97.0,
            fader_db: index as f32 - 1.0,
            muted: false,
        },
        matrix: Matrix2x2::IDENTITY,
        smoothing_samples: 0,
    }
}

fn prepared_input(rate: u32, parameters: BuiltinParameters) -> InputBuiltins {
    BuiltinChain::new(rate, parameters)
        .expect("accepted input builtins")
        .into_input_builtins()
}

/// T1: the prepared section words are the reference design's words, bit for bit.
///
/// The reference computes the design from the equations, not from production, and its `tan` is the
/// platform's while production's is `math`'s. That difference is measured separately
/// below and is invisible in the cast words.
#[test]
fn prepared_sections_match_reference_coefficients() {
    for rate in launch_and_extended_compatibility_rates() {
        let mut cutoffs = vec![10.0_f32, 100.0, 1_000.0, 0.45 * rate as f32];
        if let Some(maximum) = builtin_filter_cutoff_maximum_hz(rate) {
            cutoffs.push(maximum);
        }
        for cutoff in cutoffs {
            for (high_pass, output) in [
                (true, ReferenceTptOutput::HighPass),
                (false, ReferenceTptOutput::LowPass),
            ] {
                let actual =
                    test_support::section_words(rate, cutoff, high_pass).expect("prepared section");
                let reference =
                    ReferenceRetainedTptF32::conditioned_butterworth(rate, cutoff, output)
                        .expect("reference section");
                assert_eq!(
                    actual,
                    reference.section_words(),
                    "rate={rate}, cutoff={cutoff}, high_pass={high_pass}"
                );
            }
        }
    }
}

/// D6: the engine's own `tan` agrees with the platform's to one unit in the last place.
///
/// The engine never calls the platform's, because it is not specified to agree across targets;
/// this measures how far the two are apart on the whole prepared cutoff domain, which is what
/// makes T1's bit equality a fact about the algebra rather than a coincidence.
#[test]
fn engine_tan_agrees_with_the_platform_to_one_ulp_over_the_cutoff_domain() {
    let mut worst = 0_i64;
    for rate in launch_and_extended_compatibility_rates() {
        let maximum = 0.45 * rate as f32;
        let mut bits = 10.0_f32.to_bits();
        while f32::from_bits(bits) <= maximum {
            let x = core::f64::consts::PI * f64::from(f32::from_bits(bits)) / f64::from(rate);
            let difference = (math::tan(x).to_bits() as i64 - x.tan().to_bits() as i64).abs();
            assert!(
                difference <= 1,
                "rate={rate}, cutoff={}",
                f32::from_bits(bits)
            );
            worst = worst.max(difference);
            bits += 4_096;
        }
    }
    assert_eq!(worst, 1, "the sweep must actually contain a disagreement");
}

/// T2: the scalar stage is the reference recurrence, sample for sample and word for word.
#[test]
fn scalar_stage_is_bit_identical_to_reference_recurrence() {
    for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
        for signal in 0..3 {
            let frames = rate as usize / 4;
            let mut rng = Rng(0x51ED_0007 ^ u64::from(rate) ^ signal);
            let input: Vec<f32> = (0..frames)
                .map(|index| match signal {
                    0 => rng.next_sample(),
                    1 => f32::from(u8::from(index == 0)),
                    _ => 0.25,
                })
                .collect();

            let parameters = BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    ..ChannelParameters::default()
                },
                right: ChannelParameters {
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            };
            let mut input_builtins = prepared_input(rate, parameters);
            let mut left = input.clone();
            let mut right = input.clone();
            input_builtins.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));

            let mut high = ReferenceRetainedTptF32::conditioned_butterworth(
                rate,
                100.0,
                ReferenceTptOutput::HighPass,
            )
            .expect("reference high");
            let mut low = ReferenceRetainedTptF32::conditioned_butterworth(
                rate,
                1_000.0,
                ReferenceTptOutput::LowPass,
            )
            .expect("reference low");
            for (index, sample) in input.iter().copied().enumerate() {
                let high_bits = high.process(sample).output_bits;
                let expected = low.process(f32::from_bits(high_bits)).output_bits;
                assert_eq!(
                    left[index].to_bits(),
                    expected,
                    "rate={rate}, signal={signal}, index={index}"
                );
                assert_eq!(right[index].to_bits(), expected);
            }
            let state = test_support::input_state_words(&input_builtins);
            let [high_ic1, high_ic2] = high.state_bits();
            let [low_ic1, low_ic2] = low.state_bits();
            assert_eq!(
                state,
                [
                    high_ic1, high_ic2, low_ic1, low_ic2, high_ic1, high_ic2, low_ic1, low_ic2
                ],
                "rate={rate}, signal={signal}"
            );
        }
    }
}

/// T3: a bank is the scalar stage at another width, and a padding lane changes nothing.
#[test]
fn bank_is_bit_identical_to_scalar_stage_at_every_width() {
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        for members in [1, lanes - 1, lanes] {
            let mut scalar: Vec<InputBuiltins> = (0..members)
                .map(|index| prepared_input(48_000, parameters_for(index)))
                .collect();
            let bank_inputs: Vec<InputBuiltins> = (0..members)
                .map(|index| prepared_input(48_000, parameters_for(index)))
                .collect();
            let mut bank = BuiltinInputBank::new(backend, width, bank_inputs).expect("bank");
            assert_eq!(bank.active_lanes(), members);

            const FRAMES: usize = 257;
            let mut rng = Rng(0xB00B_5EED ^ members as u64 ^ lanes as u64);
            let planar: Vec<Vec<f32>> = (0..members)
                .map(|_| (0..FRAMES * 2).map(|_| rng.next_sample()).collect())
                .collect();

            // Padding lanes are pre-filled with the bit patterns that would poison a recurrence.
            const POISON: [u32; 4] = [0x7FC0_0000, 0x7F80_0000, 0x8000_0000, 0x0000_0001];
            let mut left = vec![0.0_f32; FRAMES * lanes];
            let mut right = vec![0.0_f32; FRAMES * lanes];
            for frame in 0..FRAMES {
                for lane in 0..lanes {
                    let (l, r) = if lane < members {
                        (planar[lane][frame], planar[lane][FRAMES + frame])
                    } else {
                        let poison = f32::from_bits(POISON[(frame + lane) % POISON.len()]);
                        (poison, poison)
                    };
                    left[frame * lanes + lane] = l;
                    right[frame * lanes + lane] = r;
                }
            }
            let bank_report = bank.process(&mut left, &mut right, FRAMES as u32);

            let mut scalar_report = BuiltinProcessReport::default();
            for (lane, input) in scalar.iter_mut().enumerate() {
                let mut scalar_left = planar[lane][..FRAMES].to_vec();
                let mut scalar_right = planar[lane][FRAMES..].to_vec();
                let report = input.process(
                    DualMonoBlock::new(&mut scalar_left, &mut scalar_right, 0).expect("block"),
                );
                scalar_report.sanitized_input += report.sanitized_input;
                scalar_report.recovered_left_state += report.recovered_left_state;
                scalar_report.recovered_right_state += report.recovered_right_state;
                for frame in 0..FRAMES {
                    assert_eq!(
                        left[frame * lanes + lane].to_bits(),
                        scalar_left[frame].to_bits(),
                        "width={lanes}, members={members}, lane={lane}, frame={frame}, left"
                    );
                    assert_eq!(
                        right[frame * lanes + lane].to_bits(),
                        scalar_right[frame].to_bits(),
                        "width={lanes}, members={members}, lane={lane}, frame={frame}, right"
                    );
                }
                assert_eq!(
                    test_support::bank_lane_state_words(&bank, lane),
                    test_support::input_state_words(input),
                    "width={lanes}, members={members}, lane={lane}, state"
                );
            }
            assert_eq!(
                bank_report, scalar_report,
                "width={lanes}, members={members}"
            );

            // Padding lanes stay exactly `+0.0` in state, whatever was left in the scratch.
            for lane in members..lanes {
                assert_eq!(
                    test_support::bank_lane_state_words(&bank, lane),
                    [0; 8],
                    "width={lanes}, members={members}, padding lane={lane}"
                );
            }

            // A padding lane whose *state* is non-finite is excluded from the boundary check and
            // from every counter. Sanitisation already makes a padding lane's samples inert, so
            // this is the only way its exclusion is load-bearing -- and it is: without the
            // `active` mask, one padding lane would zero the whole block's member output.
            if members < lanes {
                let fresh = || {
                    BuiltinInputBank::new(
                        backend,
                        width,
                        (0..members)
                            .map(|index| prepared_input(48_000, parameters_for(index)))
                            .collect(),
                    )
                    .expect("fresh bank")
                };
                let mut bank = fresh();
                let mut control = fresh();
                let mut poisoned = test_support::bank_lane_state_words(&bank, members);
                poisoned[0] = f32::NAN.to_bits();
                test_support::set_bank_lane_state_words(&mut bank, members, poisoned);
                let mut poisoned_left = left.clone();
                let mut poisoned_right = right.clone();
                let mut control_left = left.clone();
                let mut control_right = right.clone();
                let poisoned_report =
                    bank.process(&mut poisoned_left, &mut poisoned_right, FRAMES as u32);
                let control_report =
                    control.process(&mut control_left, &mut control_right, FRAMES as u32);
                assert_eq!(
                    poisoned_report, control_report,
                    "width={lanes}, members={members}, padding-lane counters"
                );
                for frame in 0..FRAMES {
                    for lane in 0..members {
                        assert_eq!(
                            poisoned_left[frame * lanes + lane].to_bits(),
                            control_left[frame * lanes + lane].to_bits(),
                            "width={lanes}, members={members}, lane={lane}, frame={frame}"
                        );
                    }
                }
            }
        }
    }
}

/// T6: the block boundary check is per lane and per block, and it never crosses lanes.
#[test]
fn boundary_check_is_lane_local_per_block() {
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let build = || {
            let inputs: Vec<InputBuiltins> = (0..lanes)
                .map(|index| prepared_input(48_000, parameters_for(index)))
                .collect();
            BuiltinInputBank::new(backend, width, inputs).expect("bank")
        };
        let mut bank = build();
        let mut control = build();
        const FRAMES: usize = 64;
        let signal: Vec<f32> = {
            let mut rng = Rng(0x0BAD_F00D ^ lanes as u64);
            (0..FRAMES * lanes).map(|_| rng.next_sample()).collect()
        };

        let mut poisoned = test_support::bank_lane_state_words(&bank, 2);
        poisoned[0] = f32::NAN.to_bits();
        test_support::set_bank_lane_state_words(&mut bank, 2, poisoned);

        let (mut left, mut right) = (signal.clone(), signal.clone());
        let (mut control_left, mut control_right) = (signal.clone(), signal.clone());
        let report = bank.process(&mut left, &mut right, FRAMES as u32);
        let control_report = control.process(&mut control_left, &mut control_right, FRAMES as u32);
        assert_eq!(report.recovered_left_state, 1, "width={lanes}");
        assert_eq!(report.recovered_right_state, 0, "width={lanes}");
        assert_eq!(control_report.recovered_left_state, 0);
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                let index = frame * lanes + lane;
                if lane == 2 {
                    assert_eq!(left[index].to_bits(), 0, "zeroed lane, frame={frame}");
                } else {
                    assert_eq!(
                        left[index].to_bits(),
                        control_left[index].to_bits(),
                        "width={lanes}, lane={lane}, frame={frame}"
                    );
                }
                assert_eq!(right[index].to_bits(), control_right[index].to_bits());
            }
        }
        // The second block recovers from a reset state and reports nothing.
        let (mut left, mut right) = (signal.clone(), signal.clone());
        let second = bank.process(&mut left, &mut right, FRAMES as u32);
        assert_eq!(second.recovered_left_state, 0, "width={lanes}");
        assert_eq!(second.recovered_right_state, 0);
    }

    // The same law at W = 1, through the lifetime counters.
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 100.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare");
    let input = test_support::chain_input_mut(&mut chain);
    let mut words = test_support::input_state_words(input);
    words[0] = f32::NAN.to_bits();
    test_support::set_input_state_words(input, words);
    let mut left = [0.5_f32];
    let mut right = [0.0_f32];
    let first = chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(first.recovered_left_state, 1);
    assert_eq!(first.recovered_right_state, 0);
    assert_eq!(left[0].to_bits(), 0);
    assert_eq!(
        test_support::chain_input(&chain).lifetime_recovered_state(),
        (1, 0)
    );
    let mut left = [0.5_f32];
    let mut right = [0.0_f32];
    let second = chain.process_input(DualMonoBlock::new(&mut left, &mut right, 1).expect("block"));
    assert_eq!(second.recovered_left_state, 0);
    assert_eq!(
        test_support::chain_input(&chain).lifetime_recovered_state(),
        (1, 0)
    );
}

/// T7: block partition changes nothing -- not the samples, not the state, not the counters.
#[test]
fn partition_invariance_over_master_plan_quanta() {
    const FRAMES: usize = 1_536;
    let signal: Vec<f32> = {
        let mut rng = Rng(0xD15E_A5E5);
        (0..FRAMES * 2).map(|_| rng.next_sample()).collect()
    };
    let parameters = BuiltinParameters {
        smoothing_samples: 257,
        ..parameters_for(3)
    };

    /// Everything one render of the corpus produces: both channels, the retained state words,
    /// the lifetime recovery counters and the settled matrix.
    struct Rendered {
        left: Vec<f32>,
        right: Vec<f32>,
        state: [u32; 8],
        recovered: (u64, u64),
        matrix: Matrix2x2,
    }

    let render = |quantum: usize| -> Rendered {
        let mut chain = BuiltinChain::new(48_000, parameters).expect("prepare");
        chain
            .set_matrix_target(Matrix2x2 {
                ll: 0.25,
                lr: -0.5,
                rl: 0.75,
                rr: -0.125,
            })
            .expect("target");
        let mut left = signal[..FRAMES].to_vec();
        let mut right = signal[FRAMES..].to_vec();
        // The retarget is a control event at sample 128. A block never spans an event, so the
        // partition is bounded by the event as well as by the quantum -- which is exactly what
        // the graph executor does, and what makes the comparison meaningful.
        const RETARGET_AT: usize = 128;
        let mut start = 0;
        while start < FRAMES {
            let next_event = if start < RETARGET_AT {
                RETARGET_AT
            } else {
                FRAMES
            };
            let end = (start + quantum).min(next_event).min(FRAMES);
            chain.process_dual_mono(
                DualMonoBlock::new(&mut left[start..end], &mut right[start..end], start as u64)
                    .expect("block"),
            );
            start = end;
            if start == RETARGET_AT {
                chain
                    .set_matrix_target(Matrix2x2 {
                        ll: -1.0,
                        lr: 0.0,
                        rl: 0.0,
                        rr: 1.0,
                    })
                    .expect("retarget");
            }
        }
        let input = test_support::chain_input(&chain);
        Rendered {
            left,
            right,
            state: test_support::input_state_words(input),
            recovered: input.lifetime_recovered_state(),
            matrix: test_support::matrix_current(test_support::chain_matrix(&chain)),
        }
    };

    let oracle = render(FRAMES);
    for quantum in [1, 7, 64, 127, 128, 255, 512, 1_024] {
        let actual = render(quantum);
        for frame in 0..FRAMES {
            assert_eq!(
                actual.left[frame].to_bits(),
                oracle.left[frame].to_bits(),
                "quantum={quantum}, frame={frame}, left"
            );
            assert_eq!(
                actual.right[frame].to_bits(),
                oracle.right[frame].to_bits(),
                "quantum={quantum}, frame={frame}, right"
            );
        }
        assert_eq!(actual.state, oracle.state, "quantum={quantum}, state");
        assert_eq!(
            actual.recovered, oracle.recovered,
            "quantum={quantum}, counters"
        );
        assert_eq!(actual.matrix, oracle.matrix, "quantum={quantum}, matrix");
    }
}

/// T9: signed zero, mute and the disabled-section identity.
#[test]
fn signed_zero_and_mute_laws() {
    // Fader at unity keeps `-0.0`; a muted lane is exactly `+0.0` even for a negative input.
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            right: ChannelParameters {
                muted: true,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare");
    let mut left = [-0.0_f32, 0.25];
    let mut right = [-1.0_f32, -0.0];
    let report =
        chain.process_fader_mute(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left[0].to_bits(), (-0.0_f32).to_bits());
    assert_eq!(left[1].to_bits(), 0.25_f32.to_bits());
    assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(right[1].to_bits(), 0.0_f32.to_bits());
    assert_eq!(report, BuiltinProcessReport::default());

    // Trim with polarity: `-0.0` becomes `+0.0` through the disabled sections, and a finite input
    // is the exact product of the folded trim.
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                polarity_invert: true,
                trim_db: 6.0206,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare");
    let trim = f32::from_bits(test_support::input_trim_words(test_support::chain_input(&chain))[0]);
    assert!(trim < 0.0, "polarity is folded into the trim");
    let mut left = [-0.0_f32, 0.25];
    let mut right = [-0.0_f32, 0.25];
    chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(left[1].to_bits(), (0.25_f32 * trim).to_bits());
    assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());

    // The input stage sanitises once, and only there.
    let mut chain = BuiltinChain::new(48_000, BuiltinParameters::default()).expect("prepare");
    let mut left = [f32::NAN, 1.0e31];
    let mut right = [f32::INFINITY, 1.0];
    let report = chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(report.sanitized_input, 3);
    assert_eq!(report.sanitized_output, 0);
    assert_eq!(left[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(left[1].to_bits(), 0.0_f32.to_bits());
    assert_eq!(right[0].to_bits(), 0.0_f32.to_bits());
    assert_eq!(right[1].to_bits(), 1.0_f32.to_bits());

    // A subnormal input is no longer sanitised: it is a legal, finite sample (D7).
    let mut chain = BuiltinChain::new(48_000, BuiltinParameters::default()).expect("prepare");
    let mut left = [f32::from_bits(1)];
    let mut right = [0.0_f32];
    let report = chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(report.sanitized_input, 0);
    assert_eq!(left[0].to_bits(), 1);
}

/// The whole chain end to end, at the tolerance the pre-#83 gate used.
#[test]
fn polarity_trim_fader_and_matrix_are_exact() {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                polarity_invert: true,
                trim_db: 6.0206,
                fader_db: 0.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters::default(),
            matrix: Matrix2x2::IDENTITY,
            smoothing_samples: 0,
        },
    )
    .expect("prepare");
    let mut left = [0.5_f32];
    let mut right = [0.0_f32];
    chain.process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert!((left[0] + 1.0).abs() < 2e-5);
    assert_eq!(right, [0.0]);
}

/// The bank constructor contract owned by this crate and consumed by #86.
#[test]
fn bank_construction_accepts_one_to_width_members_only() {
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let inputs = |count: usize| -> Vec<InputBuiltins> {
            (0..count)
                .map(|index| prepared_input(48_000, parameters_for(index)))
                .collect()
        };
        assert!(BuiltinInputBank::new(backend, width, inputs(0)).is_err());
        assert!(BuiltinInputBank::new(backend, width, inputs(lanes + 1)).is_err());
        for members in 1..=lanes {
            let bank = BuiltinInputBank::new(backend, width, inputs(members)).expect("bank");
            assert_eq!(bank.active_lanes(), members);
            assert_eq!(bank.backend(), backend);
            assert_eq!(bank.width(), width);
        }
        // The scalar backend has no bank width, so it is refused whatever width is asked for.
        assert!(BuiltinInputBank::new(Backend::Scalar, width, inputs(1)).is_err());
        // So is the *other* vector backend: a bank's width must be the one its backend selects.
        let other = match backend {
            Backend::Simd4 => Backend::Simd8,
            _ => Backend::Simd4,
        };
        assert!(BuiltinInputBank::new(other, width, inputs(1)).is_err());
    }
    // #84 phase A: `BankWidth::for_backend` is the workspace's one backend-to-width law.
    assert_eq!(BankWidth::for_backend(Backend::Scalar), None);
    assert_eq!(
        BankWidth::for_backend(Backend::Simd4),
        Some(BankWidth::Four)
    );
    assert_eq!(
        BankWidth::for_backend(Backend::Simd8),
        Some(BankWidth::Eight)
    );
}

/// T11: the prepared-identity elision -- when a bank decides it, when it refuses, and that
/// deciding it moves no bit.
///
/// The lane crate's `input_chain_elision` gate proves the *rewrite* is exact at every width and
/// section pattern. This is the bank's half: that the decision is made from the prepared words at
/// construction, that it is all-lanes-or-nothing, and that the one post-preparation write to the
/// retained state -- `set_lane_state_words`, the fault-injection seam -- re-decides it rather than
/// leaving a stale `true` standing.
#[test]
fn identity_sections_are_elided_only_when_every_lane_and_word_says_so() {
    let identity = BuiltinParameters::default();
    assert_eq!(
        identity.left.hpf_hz, 0.0,
        "the default chain is the identity"
    );
    assert_eq!(identity.left.lpf_hz, 0.0);

    // A scalar chain of all-zero cutoffs elides all four sections.
    let chain = BuiltinChain::new(48_000, identity).expect("prepare");
    assert_eq!(
        test_support::input_elision_plan(test_support::chain_input(&chain)),
        [[true, true], [true, true]],
        "an all-identity chain elides every section"
    );

    // One real cutoff blocks exactly its own section.
    let one_filter = BuiltinParameters {
        left: ChannelParameters {
            lpf_hz: 12_000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let chain = BuiltinChain::new(48_000, one_filter).expect("prepare");
    assert_eq!(
        test_support::input_elision_plan(test_support::chain_input(&chain)),
        [[true, false], [true, true]],
        "a real low-pass blocks the low-pass section and nothing else"
    );

    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let build = |real_lanes: usize| {
            let inputs: Vec<InputBuiltins> = (0..lanes)
                .map(|index| {
                    let parameters = if index < real_lanes {
                        parameters_for(index)
                    } else {
                        BuiltinParameters::default()
                    };
                    prepared_input(48_000, parameters)
                })
                .collect();
            BuiltinInputBank::new(backend, width, inputs).expect("bank")
        };

        // All lanes or nothing: one populated lane with a real filter is enough to keep the whole
        // bank's sections, because the kernel body has no per-lane branch.
        assert_eq!(
            test_support::bank_elision_plan(&build(0)),
            [[true, true], [true, true]],
            "width={lanes}: an all-identity bank"
        );
        for real_lanes in 1..=lanes {
            assert_eq!(
                test_support::bank_elision_plan(&build(real_lanes)),
                [[false, false], [false, false]],
                "width={lanes}, real lanes={real_lanes}: one real lane blocks the bank"
            );
        }

        // A padding lane is a real lane for this decision: it carries `SvfSection::IDENTITY`, so a
        // partially populated identity bank still elides.
        let partial = BuiltinInputBank::new(
            backend,
            width,
            vec![prepared_input(48_000, BuiltinParameters::default())],
        )
        .expect("bank");
        assert_eq!(
            test_support::bank_elision_plan(&partial),
            [[true, true], [true, true]],
            "width={lanes}: identity members plus identity padding"
        );

        // The invalidation hook. `set_bank_lane_state_words` is the only post-preparation write to
        // the retained state, and a `-1.0` integrator under identity coefficients is exactly the
        // case that diverges: the chain emits `-0.0` for a `-0.0` input where the elided form
        // emits `+0.0`.
        const FRAMES: usize = 32;
        let seeded = (-1.0_f32).to_bits();
        let mut injected = build(0);
        test_support::set_bank_lane_state_words(&mut injected, 1, [seeded; 8]);
        assert_eq!(
            test_support::bank_elision_plan(&injected),
            [[false, false], [false, false]],
            "width={lanes}: injected state must invalidate the plan"
        );

        // ... and the bits it produces are the ones the plan-free kernel produces. The oracle is
        // the same bank with a real filter on one lane, which never elided in the first place: it
        // is fed the same injected state and must agree sample for sample.
        let mut oracle = build(0);
        test_support::set_bank_lane_state_words(&mut oracle, 1, [seeded; 8]);
        let signal: Vec<f32> = vec![-0.0; FRAMES * lanes];
        let (mut left, mut right) = (signal.clone(), signal.clone());
        let (mut oracle_left, mut oracle_right) = (signal.clone(), signal.clone());
        let report = injected.process(&mut left, &mut right, FRAMES as u32);
        let oracle_report = oracle.process(&mut oracle_left, &mut oracle_right, FRAMES as u32);
        assert_eq!(report, oracle_report, "width={lanes}");
        for index in 0..FRAMES * lanes {
            assert_eq!(
                left[index].to_bits(),
                oracle_left[index].to_bits(),
                "width={lanes}, index={index}"
            );
        }
        // The seeded state is what makes the case bite: a `-0.0` input under the *unelided*
        // identity chain comes out `-0.0` on the seeded lane, which is precisely what elision
        // would have washed away -- and `+0.0` on every other lane, which is what it would have
        // produced everywhere.
        assert_eq!(
            left[1].to_bits(),
            (-0.0_f32).to_bits(),
            "width={lanes}: the seeded lane carries the sign of a `-0.0` input"
        );
        assert_eq!(
            left[0].to_bits(),
            0,
            "width={lanes}: an unseeded lane washes it, which is why the gate is per bank"
        );

        // A reset re-decides the plan in the other direction: the state is `+0.0` again, so the
        // identity bank is elidable again.
        injected.reset();
        assert_eq!(
            test_support::bank_elision_plan(&injected),
            [[true, true], [true, true]],
            "width={lanes}: a reset restores the elision"
        );
    }
}

/// One live command in the automation script the banked-identity gates replay.
///
/// A script entry names the lane it addresses, so the same script drives the bank (by lane) and
/// the per-track sections (one section per lane) without either side reordering it.
#[derive(Clone, Copy, Debug)]
enum Command {
    FaderDb {
        lane: usize,
        channels: BuiltinLaneSelector,
        db: f32,
        smoothing_samples: u32,
    },
    Mute {
        lane: usize,
        channels: BuiltinLaneSelector,
        muted: bool,
        smoothing_samples: u32,
    },
    Pan {
        lane: usize,
        matrix: Matrix2x2,
        smoothing_samples: u32,
    },
}

/// A deterministic automation script: `(block index, command)`, ramps deliberately overlapping.
///
/// The windows are chosen against the partitions below so that ramps are **in flight across block
/// boundaries** at every partition, and so that lanes settle at different frames inside one block:
/// that is the case a banked ramp could get wrong and a per-track one cannot, because it is the
/// only case where one lane's countdown reaching zero must not disturb its neighbours'.
fn automation_script(members: usize) -> Vec<(usize, Command)> {
    let mut script = Vec::new();
    let channels = [
        BuiltinLaneSelector::Both,
        BuiltinLaneSelector::Left,
        BuiltinLaneSelector::Right,
    ];
    for lane in 0..members {
        // Windows that are not multiples of any partition, so a ramp never settles on a boundary
        // for every partition at once.
        script.push((
            0,
            Command::FaderDb {
                lane,
                channels: channels[lane % 3],
                db: -6.0 - lane as f32,
                smoothing_samples: 37 + lane as u32 * 53,
            },
        ));
        script.push((
            1,
            Command::Pan {
                lane,
                matrix: pan_matrix(-0.5 + lane as f32 * 0.1, 0.5 - lane as f32 * 0.1)
                    .expect("in-domain pan"),
                smoothing_samples: 91 + lane as u32 * 29,
            },
        ));
        // A retarget *while the first ramp is still running*: the step is recomputed from the
        // gain in flight, so a bank that shared one countdown across lanes would diverge here.
        script.push((
            2,
            Command::FaderDb {
                lane,
                channels: channels[(lane + 1) % 3],
                db: 3.0 - lane as f32 * 0.5,
                smoothing_samples: 71 + lane as u32 * 17,
            },
        ));
        script.push((
            3,
            Command::Mute {
                lane,
                channels: channels[(lane + 2) % 3],
                muted: lane % 2 == 0,
                smoothing_samples: 23 + lane as u32 * 41,
            },
        ));
        // Unmute back to the remembered fader gain, overlapping the mute ramp on odd lanes.
        script.push((
            5,
            Command::Mute {
                lane,
                channels: channels[(lane + 2) % 3],
                muted: false,
                smoothing_samples: 13 + lane as u32 * 7,
            },
        ));
        // A zero-window snap: the D11 branch that assigns instead of ramping.
        script.push((
            7,
            Command::Pan {
                lane,
                matrix: pan_matrix(0.25, -0.25).expect("in-domain pan"),
                smoothing_samples: 0,
            },
        ));
    }
    script
}

/// Renders one automation script through the banked fader and matrix, in AoSoA order.
///
/// Returns the planar per-lane output, so it can be compared word for word against the per-track
/// sections that the graph binds today.
fn render_banked(
    backend: Backend,
    width: BankWidth,
    members: usize,
    blocks: usize,
    frames: usize,
    planar: &[Vec<f32>],
) -> Vec<Vec<f32>> {
    let lanes = width.lanes() as usize;
    let mut fader =
        BuiltinFaderBank::new(backend, width, (0..members).map(parameters_for).collect())
            .expect("fader bank");
    let mut matrix = BuiltinMatrixBank::new(
        backend,
        width,
        (0..members)
            .map(|index| {
                let parameters = parameters_for(index);
                (parameters.matrix, parameters.smoothing_samples)
            })
            .collect(),
    )
    .expect("matrix bank");
    assert_eq!(fader.active_lanes(), members);
    assert_eq!(matrix.active_lanes(), members);

    let script = automation_script(members);
    let mut output: Vec<Vec<f32>> = vec![Vec::with_capacity(blocks * frames * 2); members];
    let mut left = vec![0.0_f32; frames * lanes];
    let mut right = vec![0.0_f32; frames * lanes];
    // The bit patterns a padding lane must not be able to leak out of, refreshed every block.
    const POISON: [u32; 4] = [0x7FC0_0000, 0x7F80_0000, 0x8000_0000, 0x0000_0001];
    for block in 0..blocks {
        for (at, command) in &script {
            if *at != block {
                continue;
            }
            match *command {
                Command::FaderDb {
                    lane,
                    channels,
                    db,
                    smoothing_samples,
                } => fader
                    .set_fader_db(lane, channels, db, smoothing_samples)
                    .expect("in-domain fader move"),
                Command::Mute {
                    lane,
                    channels,
                    muted,
                    smoothing_samples,
                } => fader
                    .set_mute(lane, channels, muted, smoothing_samples)
                    .expect("member lane"),
                Command::Pan {
                    lane,
                    matrix: target,
                    smoothing_samples,
                } => matrix
                    .set_target_smoothed(lane, target, smoothing_samples)
                    .expect("in-domain pan"),
            }
        }
        for frame in 0..frames {
            let sample = block * frames + frame;
            for lane in 0..lanes {
                let (l, r) = if lane < members {
                    (planar[lane][sample], planar[lane][blocks * frames + sample])
                } else {
                    let poison = f32::from_bits(POISON[(sample + lane) % POISON.len()]);
                    (poison, poison)
                };
                left[frame * lanes + lane] = l;
                right[frame * lanes + lane] = r;
            }
        }
        fader.process(&mut left, &mut right, frames as u32);
        matrix.process(&mut left, &mut right, frames as u32);
        for (lane, out) in output.iter_mut().enumerate() {
            for frame in 0..frames {
                out.push(left[frame * lanes + lane]);
            }
            for frame in 0..frames {
                out.push(right[frame * lanes + lane]);
            }
        }
    }
    output
}

/// Renders the same script through the per-track sections the graph binds today, one per lane.
fn render_per_track(
    members: usize,
    blocks: usize,
    frames: usize,
    planar: &[Vec<f32>],
) -> Vec<Vec<f32>> {
    let script = automation_script(members);
    let mut faders: Vec<FaderMuteRampBuiltins> = (0..members)
        .map(|index| FaderMuteRampBuiltins::new(parameters_for(index)).expect("fader"))
        .collect();
    let mut matrices: Vec<MatrixBuiltins> = (0..members)
        .map(|index| {
            let (_, _, matrix) = BuiltinChain::new(48_000, parameters_for(index))
                .expect("chain")
                .into_sections();
            matrix
        })
        .collect();
    let mut output: Vec<Vec<f32>> = vec![Vec::with_capacity(blocks * frames * 2); members];
    for block in 0..blocks {
        for (at, command) in &script {
            if *at != block {
                continue;
            }
            match *command {
                Command::FaderDb {
                    lane,
                    channels,
                    db,
                    smoothing_samples,
                } => faders[lane]
                    .set_fader_db(channels, db, smoothing_samples)
                    .expect("in-domain fader move"),
                Command::Mute {
                    lane,
                    channels,
                    muted,
                    smoothing_samples,
                } => faders[lane].set_mute(channels, muted, smoothing_samples),
                Command::Pan {
                    lane,
                    matrix: target,
                    smoothing_samples,
                } => matrices[lane]
                    .set_target_smoothed(target, smoothing_samples)
                    .expect("in-domain pan"),
            }
        }
        for lane in 0..members {
            let start = block * frames;
            let mut left = planar[lane][start..start + frames].to_vec();
            let mut right =
                planar[lane][blocks * frames + start..blocks * frames + start + frames].to_vec();
            faders[lane].process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
            matrices[lane].process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
            output[lane].extend_from_slice(&left);
            output[lane].extend_from_slice(&right);
        }
    }
    output
}

/// The Class-A claim of the banked strip: a banked fader and matrix render the exact bits the
/// per-track sections render, under live automation, at every width and every member count.
///
/// This is the identity the whole job rests on, and it is stated over the case that can break it
/// and nothing else: ramps in flight across block boundaries, retargets landing mid-ramp, mutes
/// and unmutes overlapping, per-lane windows that settle at different frames inside one block,
/// and padding lanes pre-filled with NaN, infinity, `-0.0` and a subnormal.
///
/// Red-mutation proven: sharing one countdown across the bank -- replacing the per-lane
/// `remaining` maximum in `FaderRampStage::process_plane` with lane 0's count -- fails on lane 1
/// of the first ragged case.
#[test]
fn banked_fader_and_matrix_are_bit_identical_to_the_per_track_sections() {
    const BLOCKS: usize = 9;
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        for members in [1, lanes - 1, lanes] {
            // Partitions chosen against the script's windows so a ramp is always mid-flight at a
            // boundary: 1 is the pathological case, 128 is the master-plan quantum.
            for frames in [1, 7, 64, 128] {
                let mut rng = Rng(0x5A17_0BED ^ members as u64 ^ (frames as u64) << 8);
                let planar: Vec<Vec<f32>> = (0..members)
                    .map(|_| {
                        (0..BLOCKS * frames * 2)
                            .map(|_| rng.next_sample())
                            .collect()
                    })
                    .collect();
                let banked = render_banked(backend, width, members, BLOCKS, frames, &planar);
                let per_track = render_per_track(members, BLOCKS, frames, &planar);
                for lane in 0..members {
                    for (index, (bank, track)) in
                        banked[lane].iter().zip(per_track[lane].iter()).enumerate()
                    {
                        assert_eq!(
                            bank.to_bits(),
                            track.to_bits(),
                            "width={lanes}, members={members}, frames={frames}, lane={lane}, \
                             sample={index}: banked {bank:?} vs per-track {track:?}"
                        );
                    }
                }
            }
        }
    }
}

/// A settled mute is exactly `+0.0` in the bank, for a negative input too, and a settled lane's
/// gain is one multiply -- the two properties banking must not quietly trade away.
///
/// Red-mutation proven: gating the kernel's clear on the frame *after* the ramp settles -- an
/// `andnot` of `done` shifted by one -- leaves the settling sample carrying the input's sign bit
/// and fails the `-0.0` assertion below.
#[test]
fn a_settled_banked_mute_is_exactly_positive_zero() {
    for (backend, width) in BANKS {
        let lanes = width.lanes() as usize;
        let mut bank = BuiltinFaderBank::new(
            backend,
            width,
            (0..lanes).map(|_| BuiltinParameters::default()).collect(),
        )
        .expect("fader bank");
        // Lane 0 mutes over a window that settles inside block 1; lane 1 is muted instantly.
        bank.set_mute(0, BuiltinLaneSelector::Both, true, 5)
            .expect("member lane");
        bank.set_mute(1, BuiltinLaneSelector::Both, true, 0)
            .expect("member lane");
        const FRAMES: usize = 16;
        for block in 0..2 {
            let mut left = vec![-1.0_f32; FRAMES * lanes];
            let mut right = vec![-1.0_f32; FRAMES * lanes];
            bank.process(&mut left, &mut right, FRAMES as u32);
            for frame in 0..FRAMES {
                // Lane 1 settled before a single sample was rendered, so every sample of both
                // blocks is `+0.0`; lane 0 is `+0.0` from the frame its ramp assigned the target.
                if block == 1 || frame >= 4 {
                    for (plane, name) in [(&left, "left"), (&right, "right")] {
                        assert_eq!(
                            plane[frame * lanes].to_bits(),
                            0,
                            "width={lanes}, block={block}, frame={frame}, {name}: a settled mute \
                             kept the input's sign"
                        );
                    }
                }
                assert_eq!(
                    left[frame * lanes + 1].to_bits(),
                    0,
                    "width={lanes}, block={block}, frame={frame}: an instant mute is not +0.0"
                );
            }
            // An unmuted, unmoved lane is one multiply by unit gain, so it is untouched.
            for frame in 0..FRAMES {
                assert_eq!(
                    left[frame * lanes + 2].to_bits(),
                    (-1.0_f32).to_bits(),
                    "width={lanes}, block={block}, frame={frame}: a settled unmuted lane moved"
                );
            }
        }
    }
}
