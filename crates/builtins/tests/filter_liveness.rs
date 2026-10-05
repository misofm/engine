#![allow(missing_docs)]

use builtins::{
    BuiltinChain, BuiltinInputBank, BuiltinLaneSelector, BuiltinParameters, BuiltinTail,
    DualMonoBlock, InputBuiltins, PreparedInputFilterTarget, builtin_filter_cutoff_maximum_hz,
    prepare_input_filter_pair, validate_prepared_input_filter_target,
};
use effect_contract::{ResponseSnapshotRequest, ResponseSnapshotSection};
use lane::Backend;

fn disabled() -> InputBuiltins {
    BuiltinChain::new(48_000, BuiltinParameters::default())
        .expect("prepared")
        .into_input_builtins()
}

fn bank() -> BuiltinInputBank {
    bank_with_members(4)
}

fn bank_with_members(members: usize) -> BuiltinInputBank {
    BuiltinInputBank::new(
        Backend::Simd4,
        effect_contract::BankWidth::Four,
        (0..members).map(|_| disabled()).collect(),
    )
    .expect("bank")
}

fn snapshot(input: &InputBuiltins) -> [ResponseSnapshotSection; 2] {
    let empty = ResponseSnapshotSection {
        id: 0,
        kind: 0,
        enabled: false,
        word_count: 0,
        words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
    };
    let mut left = [empty; 2];
    let mut right = [empty; 2];
    input
        .copy_response_snapshot(
            48_000,
            ResponseSnapshotRequest {
                bypassed: false,
                left: &mut left,
                right: &mut right,
            },
        )
        .expect("snapshot");
    left
}

#[test]
fn pair_preparation_accepts_launch_rates_and_rejects_crossing() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let prepared = prepare_input_filter_pair(rate, 80.0, 8_000.0).expect("pair");
        assert_eq!(prepared.pair.hpf_hz, 80.0);
        assert!(prepare_input_filter_pair(rate, 8_000.0, 80.0).is_err());
        assert!(prepare_input_filter_pair(rate, 0.0, 0.0).is_ok());
    }
}

#[test]
fn target_response_is_visible_before_the_ramp_reaches_the_endpoint() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let mut input = disabled();
    input
        .apply_prepared_filter(PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            ..prepared.targets[0]
        })
        .expect("target");
    input
        .apply_prepared_filter(PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            ..prepared.targets[1]
        })
        .expect("target");
    let response = snapshot(&input);
    assert_eq!(response[0].words, {
        let mut words = [0; 7];
        words[0] = prepared.targets[0].coefficients[0].to_bits();
        words[1] = prepared.targets[0].coefficients[1].to_bits();
        words[2] = prepared.targets[0].coefficients[2].to_bits();
        words[3] = f32::from_bits(0x3fb5_04f3).to_bits();
        words[4] = prepared.targets[0].coefficients[3].to_bits();
        words[5] = prepared.targets[0].coefficients[4].to_bits();
        words[6] = prepared.targets[0].coefficients[5].to_bits();
        words
    });
    let mut left = vec![1.0; 65];
    let mut right = left.clone();
    input.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(left.len(), 65);
    assert!(left.iter().all(|sample| sample.is_finite()));
    assert_eq!(left[0].to_bits(), 1.0_f32.to_bits());
    assert_ne!(left[64].to_bits(), left[0].to_bits());
}

#[test]
fn disabled_trim_ramp_stays_finite_and_keeps_zero_pair() {
    let mut input = disabled();
    input
        .set_trim_db(BuiltinLaneSelector::Both, -12.0, 64)
        .expect("trim");
    let mut left = vec![-0.0; 80];
    let mut right = left.clone();
    input.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert!(left.iter().all(|sample| sample.is_finite()));
    assert!(right.iter().all(|sample| sample.is_finite()));
}

#[test]
fn symmetric_bank_target_keeps_mono_and_dual_paths_equal() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let mut dual = bank();
    let mut mono = bank();
    for target in prepared.targets {
        let target = PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            ..target
        };
        for lane in 0..4 {
            dual.apply_prepared_filter(lane, target).expect("target");
            mono.apply_prepared_filter(lane, target).expect("target");
        }
    }
    assert!(dual.supports_mono_collapse());
    let mut dual_left = vec![1.0; 65 * 4];
    let mut dual_right = dual_left.clone();
    let mut mono_left = dual_left.clone();
    dual.process(&mut dual_left, &mut dual_right, 65);
    mono.process_mono(&mut mono_left, 65);
    assert_eq!(
        dual_left
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>(),
        mono_left
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>(),
    );
}

#[test]
fn prepared_target_validation_matches_disabled_identity_exactly() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let identity = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    let mut enabled_with_identity = prepared.targets[0];
    enabled_with_identity.coefficients = identity;
    assert!(validate_prepared_input_filter_target(&enabled_with_identity).is_err());

    let mut disabled_with_coefficients = prepared.targets[0];
    disabled_with_coefficients.pair = [0.0, 8_000.0];
    assert!(validate_prepared_input_filter_target(&disabled_with_coefficients).is_err());

    let mut disabled_identity = prepared.targets[0];
    disabled_identity.pair = [0.0, 8_000.0];
    disabled_identity.coefficients = identity;
    assert!(validate_prepared_input_filter_target(&disabled_identity).is_ok());

    let mut negative_zero = disabled_identity;
    negative_zero.pair[0] = -0.0;
    assert!(validate_prepared_input_filter_target(&negative_zero).is_err());
}

#[test]
fn cutoff_maximums_accept_the_maximum_and_reject_its_successor() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let maximum = builtin_filter_cutoff_maximum_hz(rate).expect("launch rate");
        let successor = f32::from_bits(maximum.to_bits() + 1);
        assert!(prepare_input_filter_pair(rate, maximum, 0.0).is_ok());
        assert!(prepare_input_filter_pair(rate, 0.0, maximum).is_ok());
        assert!(prepare_input_filter_pair(rate, successor, 0.0).is_err());
        assert!(prepare_input_filter_pair(rate, 0.0, successor).is_err());
    }
}

#[test]
fn bank_filter_target_updates_one_member_and_leaves_padding_identity() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let mut bank = bank_with_members(2);
    let mut scalar = disabled();
    for target in prepared.targets {
        let target = PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            ..target
        };
        bank.apply_prepared_filter(1, target).expect("target");
        scalar.apply_prepared_filter(target).expect("target");
    }
    assert_eq!(
        bank.apply_prepared_filter(2, prepared.targets[0]),
        Err(builtins::BuiltinParameterError::LaneLength)
    );

    let mut bank_left = vec![1.0; 65 * 4];
    let mut bank_right = bank_left.clone();
    bank.process(&mut bank_left, &mut bank_right, 65);
    let mut scalar_left = vec![1.0; 65];
    let mut scalar_right = scalar_left.clone();
    scalar.process(DualMonoBlock::new(&mut scalar_left, &mut scalar_right, 0).expect("block"));
    for frame in 0..65 {
        assert_eq!(bank_left[frame * 4].to_bits(), 1.0_f32.to_bits());
        assert_eq!(
            bank_left[frame * 4 + 1].to_bits(),
            scalar_left[frame].to_bits()
        );
        assert_eq!(bank_left[frame * 4 + 2].to_bits(), 1.0_f32.to_bits());
        assert_eq!(bank_left[frame * 4 + 3].to_bits(), 1.0_f32.to_bits());
        assert_eq!(bank_right[frame * 4].to_bits(), 1.0_f32.to_bits());
        assert_eq!(
            bank_right[frame * 4 + 1].to_bits(),
            scalar_right[frame].to_bits()
        );
        assert_eq!(bank_right[frame * 4 + 2].to_bits(), 1.0_f32.to_bits());
        assert_eq!(bank_right[frame * 4 + 3].to_bits(), 1.0_f32.to_bits());
    }
}

#[test]
fn filter_ramp_endpoint_is_partition_invariant_and_reset_honors_kind() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let mut one_block = disabled();
    let mut split_blocks = disabled();
    for target in prepared.targets {
        one_block.apply_prepared_filter(target).expect("target");
        split_blocks.apply_prepared_filter(target).expect("target");
    }
    let mut one_left = vec![1.0; 64];
    let mut one_right = one_left.clone();
    one_block.process(DualMonoBlock::new(&mut one_left, &mut one_right, 0).expect("block"));
    let mut split_left = vec![1.0; 64];
    let mut split_right = split_left.clone();
    for (offset, frames) in [(0, 1), (1, 17), (18, 46)] {
        split_blocks.process(
            DualMonoBlock::new(
                &mut split_left[offset..offset + frames],
                &mut split_right[offset..offset + frames],
                offset as u64,
            )
            .expect("block"),
        );
    }
    assert_eq!(
        one_left
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>(),
        split_left
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        builtins::test_support::input_state_words(&one_block),
        builtins::test_support::input_state_words(&split_blocks)
    );
    let one_sections = builtins::test_support::input_section_words(&one_block);
    for (section, target) in one_sections.iter().zip(prepared.targets) {
        assert_eq!(
            &section[0..3],
            &[
                target.coefficients[0].to_bits(),
                target.coefficients[1].to_bits(),
                target.coefficients[2].to_bits(),
            ]
        );
        assert_eq!(
            &section[4..7],
            &[
                target.coefficients[3].to_bits(),
                target.coefficients[4].to_bits(),
                target.coefficients[5].to_bits(),
            ]
        );
    }

    let disabled_pair = prepare_input_filter_pair(48_000, 0.0, 0.0).expect("disabled");
    let mut retargeted = one_block;
    for target in disabled_pair.targets {
        retargeted.apply_prepared_filter(target).expect("disable");
    }
    let mut disabled_left = vec![1.0; 64];
    let mut disabled_right = disabled_left.clone();
    retargeted
        .process(DualMonoBlock::new(&mut disabled_left, &mut disabled_right, 64).expect("block"));
    assert!(disabled_left.iter().all(|sample| sample.is_finite()));
    let disabled_sections = builtins::test_support::input_section_words(&retargeted);
    assert!(disabled_sections.iter().all(|section| {
        section[0..3].iter().all(|word| *word == 0) && section[4..7] == [1.0_f32.to_bits(), 0, 0]
    }));

    let mut keep = disabled();
    let mut full = disabled();
    for target in prepared.targets {
        keep.apply_prepared_filter(target).expect("target");
        full.apply_prepared_filter(target).expect("target");
    }
    keep.reset_with_kind(builtins::BuiltinResetKind::DiscontinuityKeepTargets);
    full.reset_with_kind(builtins::BuiltinResetKind::FullToPrepared);
    let mut keep_left = [1.0];
    let mut keep_right = [1.0];
    let mut full_left = [1.0];
    let mut full_right = [1.0];
    keep.process(DualMonoBlock::new(&mut keep_left, &mut keep_right, 0).expect("block"));
    full.process(DualMonoBlock::new(&mut full_left, &mut full_right, 0).expect("block"));
    assert_ne!(keep_left[0].to_bits(), full_left[0].to_bits());
    assert_eq!(full_left[0].to_bits(), 1.0_f32.to_bits());
}

#[test]
fn input_tail_is_infinite_while_a_filter_target_is_ramping() {
    let prepared = prepare_input_filter_pair(48_000, 120.0, 8_000.0).expect("pair");
    let mut input = disabled();
    assert_eq!(input.tail(), BuiltinTail::FiniteZero);
    input
        .apply_prepared_filter(prepared.targets[0])
        .expect("target");
    assert_eq!(input.tail(), BuiltinTail::Infinite);
    let mut left = vec![1.0; 64];
    let mut right = left.clone();
    input.process(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    assert_eq!(input.tail(), BuiltinTail::Infinite);

    let disabled_pair = prepare_input_filter_pair(48_000, 0.0, 0.0).expect("disabled");
    input
        .apply_prepared_filter(disabled_pair.targets[0])
        .expect("disable");
    assert_eq!(input.tail(), BuiltinTail::Infinite);
    let mut left = vec![1.0; 64];
    let mut right = left.clone();
    input.process(DualMonoBlock::new(&mut left, &mut right, 64).expect("block"));
    assert_eq!(input.tail(), BuiltinTail::FiniteZero);
}

// ---- #1407: the live retarget law (rules 1-4) ----------------------------------------------

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const IDENTITY_BITS: [u32; 6] = [0, 0, 0, 0x3f80_0000, 0, 0];

fn disabled_at(rate: u32) -> InputBuiltins {
    BuiltinChain::new(rate, BuiltinParameters::default())
        .expect("prepared")
        .into_input_builtins()
}

/// One section's design target, the other section held disabled so the pair never crosses.
fn design(rate: u32, section: usize, hz: f32) -> PreparedInputFilterTarget {
    let pair = if section == 0 {
        prepare_input_filter_pair(rate, hz, 0.0)
    } else {
        prepare_input_filter_pair(rate, 0.0, hz)
    };
    pair.expect("design").targets[section]
}

fn disable(rate: u32, section: usize) -> PreparedInputFilterTarget {
    prepare_input_filter_pair(rate, 0.0, 0.0)
        .expect("disabled")
        .targets[section]
}

/// The current `[c1, a2, a3, m0, m1, m2]` words of one section on one channel.
fn current_words(input: &InputBuiltins, channel: usize, section: usize) -> [u32; 6] {
    let words = builtins::test_support::input_section_words(input)[channel * 2 + section];
    [words[0], words[1], words[2], words[4], words[5], words[6]]
}

fn state_words(input: &InputBuiltins, channel: usize, section: usize) -> [u32; 2] {
    let words = builtins::test_support::input_state_words(input);
    [
        words[channel * 4 + section * 2],
        words[channel * 4 + section * 2 + 1],
    ]
}

fn render_frame(input: &mut InputBuiltins, frame: u64, value: f32) -> [f32; 2] {
    let mut left = [value];
    let mut right = [-value];
    input.process(DualMonoBlock::new(&mut left, &mut right, frame).expect("block"));
    [left[0], right[0]]
}

fn bits6(words: [f32; 6]) -> [u32; 6] {
    words.map(f32::to_bits)
}

/// Gate 1: a disable re-sent before every 1-frame block completes exactly 64 frames after the
/// first one, clears the section's integrators and elides it, and a further re-send keeps it so.
#[test]
fn a_disable_re_sent_every_frame_completes_clears_and_elides_on_time() {
    for rate in LAUNCH_RATES {
        for (section, hz) in [(1_usize, 15.0_f32), (0, 1_000.0)] {
            let mut input = disabled_at(rate);
            input
                .apply_prepared_filter(design(rate, section, hz))
                .expect("design");
            let mut frame = 0_u64;
            for _ in 0..256 {
                render_frame(&mut input, frame, 0.75);
                frame += 1;
            }
            assert_ne!(state_words(&input, 0, section), [0, 0], "{rate} {section}");
            let off = disable(rate, section);
            for elapsed in 1..=128_u32 {
                input.apply_prepared_filter(off).expect("disable");
                render_frame(&mut input, frame, 0.75);
                frame += 1;
                let plan = builtins::test_support::input_elision_plan(&input);
                for (channel, elided) in plan.iter().enumerate() {
                    let settled = current_words(&input, channel, section) == IDENTITY_BITS
                        && state_words(&input, channel, section) == [0, 0]
                        && elided[section];
                    assert_eq!(
                        settled,
                        elapsed >= 64,
                        "rate {rate} section {section} channel {channel} frame {elapsed}"
                    );
                }
            }

            // The same history on one member of a four-wide bank.
            let mut bank = BuiltinInputBank::new(
                Backend::Simd4,
                effect_contract::BankWidth::Four,
                (0..4).map(|_| disabled_at(rate)).collect(),
            )
            .expect("bank");
            let member = 1;
            bank.apply_prepared_filter(member, design(rate, section, hz))
                .expect("design");
            let mut left = vec![0.75_f32; 256 * 4];
            let mut right = vec![-0.75_f32; 256 * 4];
            bank.process(&mut left, &mut right, 256);
            let state = builtins::test_support::bank_lane_state_words(&bank, member);
            assert_ne!(&state[section * 2..section * 2 + 2], &[0, 0]);
            let off = disable(rate, section);
            for elapsed in 1..=128_u32 {
                bank.apply_prepared_filter(member, off).expect("disable");
                let mut left = [0.75_f32; 4];
                let mut right = [-0.75_f32; 4];
                bank.process(&mut left, &mut right, 1);
                let state = builtins::test_support::bank_lane_state_words(&bank, member);
                let plan = builtins::test_support::bank_elision_plan(&bank);
                for (channel, elided) in plan.iter().enumerate() {
                    let at = channel * 4 + section * 2;
                    // An elided section is one whose six current words are the identity on
                    // every member and whose integrators are +0.0, so the plan reads both.
                    let settled = state[at..at + 2] == [0, 0] && elided[section];
                    assert_eq!(
                        settled,
                        elapsed >= 64,
                        "bank rate {rate} section {section} channel {channel} frame {elapsed}"
                    );
                }
                if elapsed > 64 {
                    // Frame A+64, the first after completion, renders the dry, trimmed input.
                    assert_eq!(left[member].to_bits(), 0.75_f32.to_bits());
                    assert_eq!(right[member].to_bits(), (-0.75_f32).to_bits());
                }
            }
        }
    }
}

/// Gate 2: a disable freezes the recursion words, from a settled design and from the interior
/// word of a design-to-design ramp, while the mix words move to the identity mix.
#[test]
fn a_disable_freezes_the_recursion_words_and_moves_only_the_mix() {
    for rate in LAUNCH_RATES {
        for section in 0..2 {
            let (from_hz, to_hz) = if section == 0 {
                (80.0, 1_000.0)
            } else {
                (12_000.0, 2_000.0)
            };
            for interior in [false, true] {
                let mut input = disabled_at(rate);
                input
                    .apply_prepared_filter(design(rate, section, from_hz))
                    .expect("design");
                let mut frame = 0_u64;
                for _ in 0..128 {
                    render_frame(&mut input, frame, 0.5);
                    frame += 1;
                }
                if interior {
                    input
                        .apply_prepared_filter(design(rate, section, to_hz))
                        .expect("retarget");
                    for _ in 0..20 {
                        render_frame(&mut input, frame, 0.5);
                        frame += 1;
                    }
                }
                let before = current_words(&input, 0, section);
                if interior {
                    assert_ne!(
                        before[0..3],
                        bits6(design(rate, section, to_hz).coefficients)[0..3],
                        "frame 20 is an interior word"
                    );
                }
                input
                    .apply_prepared_filter(disable(rate, section))
                    .expect("disable");
                let mut previous = before;
                for elapsed in 1..=64 {
                    render_frame(&mut input, frame, 0.5);
                    frame += 1;
                    for channel in 0..2 {
                        let words = current_words(&input, channel, section);
                        if elapsed < 64 {
                            assert_eq!(
                                words[0..3],
                                before[0..3],
                                "rate {rate} section {section} interior {interior} frame {elapsed}"
                            );
                            if channel == 0 {
                                assert_ne!(words[3..6], previous[3..6], "the mix moves");
                            }
                        } else {
                            assert_eq!(words, IDENTITY_BITS);
                            assert_eq!(state_words(&input, channel, section), [0, 0]);
                        }
                    }
                    previous = current_words(&input, 0, section);
                }
            }
        }
    }
}

/// Gate 3: an enable from a settled disabled section jumps the recursion and crossfades the
/// mix, so its first sample is the dry input; restored integrators on an identity section take
/// the six-word ramp instead.
#[test]
fn an_enable_from_rest_jumps_the_recursion_only_when_the_integrators_are_zero() {
    for rate in LAUNCH_RATES {
        for section in 0..2 {
            let target = design(rate, section, if section == 0 { 200.0 } else { 3_000.0 });
            let words = target.coefficients;
            let mut input = disabled_at(rate);
            input.apply_prepared_filter(target).expect("enable");
            let output = render_frame(&mut input, 0, 0.625);
            assert_eq!(output[0].to_bits(), 0.625_f32.to_bits());
            assert_eq!(output[1].to_bits(), (-0.625_f32).to_bits());
            let identity_mix = [1.0_f32, 0.0, 0.0];
            let expected = [
                words[0],
                words[1],
                words[2],
                identity_mix[0] + (words[3] - identity_mix[0]) * (1.0 / 64.0),
                identity_mix[1] + (words[4] - identity_mix[1]) * (1.0 / 64.0),
                identity_mix[2] + (words[5] - identity_mix[2]) * (1.0 / 64.0),
            ];
            for channel in 0..2 {
                assert_eq!(
                    current_words(&input, channel, section),
                    bits6(expected),
                    "rate {rate} section {section} channel {channel}"
                );
            }

            // Restored non-zero integrators on the identity section: rule 4.
            let mut restored = disabled_at(rate);
            let mut state = [0_u32; 8];
            for channel in 0..2 {
                state[channel * 4 + section * 2] = 0.25_f32.to_bits();
                state[channel * 4 + section * 2 + 1] = (-0.125_f32).to_bits();
            }
            builtins::test_support::set_input_state_words(&mut restored, state);
            restored.apply_prepared_filter(target).expect("enable");
            render_frame(&mut restored, 0, 0.625);
            let ramped: [f32; 6] = core::array::from_fn(|index| {
                let start = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0][index];
                start + (words[index] - start) * (1.0 / 64.0)
            });
            for channel in 0..2 {
                assert_eq!(
                    current_words(&restored, channel, section),
                    bits6(ramped),
                    "restored rate {rate} section {section} channel {channel}"
                );
            }
        }
    }
}

/// `||A(w)||_V` in `f64`: the zero-input state step `A(w) = [[1 - 2 c1, -2 a2], [2 a2, 1 - 2 a3]]`
/// in the norm `||x||_V^2 = x1^2 + sqrt(2) x1 x2 + x2^2`, the shared eigenbasis norm of every
/// `k = sqrt(2)` design. With `M = R^T R`, `R = [[1, r], [0, r]]`, `r = 1/sqrt(2)`, the induced
/// norm is the spectral norm of `R A R^-1`, `R^-1 = [[1, -1], [0, 1/r]]`.
fn v_norm(c1: f32, a2: f32, a3: f32) -> f64 {
    let (c1, a2, a3) = (f64::from(c1), f64::from(a2), f64::from(a3));
    let a = [[1.0 - 2.0 * c1, -2.0 * a2], [2.0 * a2, 1.0 - 2.0 * a3]];
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let ra = [
        [a[0][0] + r * a[1][0], a[0][1] + r * a[1][1]],
        [r * a[1][0], r * a[1][1]],
    ];
    let b = [
        [ra[0][0], -ra[0][0] + ra[0][1] / r],
        [ra[1][0], -ra[1][0] + ra[1][1] / r],
    ];
    let frobenius_sq =
        b[0][0] * b[0][0] + b[0][1] * b[0][1] + b[1][0] * b[1][0] + b[1][1] * b[1][1];
    let determinant = b[0][0] * b[1][1] - b[0][1] * b[1][0];
    let discriminant = (frobenius_sq * frobenius_sq - 4.0 * determinant * determinant).max(0.0);
    (0.5 * (frobenius_sq + discriminant.sqrt())).sqrt()
}

struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / (1_u64 << 53) as f64)
    }
}

/// Gate 4: every recursion word any seeded history reaches, at every quantum below the ramp, has
/// a `V`-norm no larger than the history's slowest design plus the `f32` inflation bound. The
/// identity recursion is reachable only at rest, with both integrators `+0.0`.
#[test]
fn every_reachable_recursion_word_stays_inside_the_hull_of_the_designs() {
    const KAPPA: f64 = 2.414;
    let inflation = 6.0 * (1.0 / 16_777_216.0) * KAPPA;
    let (rates, quanta): (&[u32], Vec<usize>) = if cfg!(debug_assertions) {
        (&[48_000], vec![1, 2, 7, 63])
    } else {
        (&LAUNCH_RATES, (1..=63).collect())
    };
    for &rate in rates {
        let maximum_hz = f64::from(builtin_filter_cutoff_maximum_hz(rate).expect("rate"));
        let mut worst_norm = 0.0_f64;
        let mut worst_excess = f64::NEG_INFINITY;
        let mut worst_design = 0.0_f64;
        for &quantum in &quanta {
            for history in 0..16_u64 {
                let mut rng = SplitMix(
                    u64::from(rate) << 32 ^ (quantum as u64) << 8 ^ history ^ 0x1407_0000_0000,
                );
                let mut input = disabled_at(rate);
                let mut sent = [disable(rate, 0), disable(rate, 1)];
                let mut q_design = [0.0_f64; 2];
                let mut reached = [0.0_f64; 2];
                let mut frame = 0_u64;
                for _ in 0..512 {
                    for section in 0..2 {
                        let target = match rng.next() % 4 {
                            // Enable to (or retarget to) a design, log-uniform in [10 Hz, max].
                            // One draw in eight takes an endpoint exactly: the maximum
                            // cutoff is the design with the largest norm.
                            0 | 3 => {
                                let hz = match rng.next() % 16 {
                                    0 => 10.0,
                                    1 => maximum_hz as f32,
                                    _ => math::exp(
                                        math::log(10.0)
                                            + rng.unit()
                                                * (math::log(maximum_hz) - math::log(10.0)),
                                    ) as f32,
                                };
                                let hz = hz.clamp(10.0, maximum_hz as f32);
                                let target = design(rate, section, hz);
                                let c = target.coefficients;
                                q_design[section] = q_design[section].max(v_norm(c[0], c[1], c[2]));
                                target
                            }
                            1 => disable(rate, section),
                            _ => sent[section],
                        };
                        sent[section] = target;
                        input.apply_prepared_filter(target).expect("target");
                    }
                    for _ in 0..quantum {
                        let value = (rng.unit() * 2.0 - 1.0) as f32;
                        render_frame(&mut input, frame, value);
                        frame += 1;
                        for channel in 0..2 {
                            for (section, reached) in reached.iter_mut().enumerate() {
                                let words = current_words(&input, channel, section);
                                if words[0..3] == [0, 0, 0] {
                                    assert_eq!(
                                        state_words(&input, channel, section),
                                        [0, 0],
                                        "identity recursion with live state: rate {rate} quantum {quantum} history {history}"
                                    );
                                    continue;
                                }
                                let norm = v_norm(
                                    f32::from_bits(words[0]),
                                    f32::from_bits(words[1]),
                                    f32::from_bits(words[2]),
                                );
                                *reached = reached.max(norm);
                            }
                        }
                    }
                }
                for (section, (reached, q_design)) in reached.into_iter().zip(q_design).enumerate()
                {
                    if reached == 0.0 {
                        continue;
                    }
                    assert!(q_design < 1.0);
                    let excess = reached - q_design;
                    assert!(
                        excess <= inflation,
                        "rate {rate} quantum {quantum} history {history} section {section}: \
                         reached {:.6e} over design {:.6e} by {excess:.3e} (bound {inflation:.3e})",
                        reached - 1.0,
                        q_design - 1.0,
                    );
                    worst_norm = worst_norm.max(reached);
                    worst_design = worst_design.max(q_design);
                    worst_excess = worst_excess.max(excess);
                }
            }
        }
        println!(
            "rate {rate}: max ||A(w)||_V - 1 = {:.3e}, max design q - 1 = {:.3e}, \
             max excess over the history's design = {worst_excess:.3e} (bound {inflation:.3e})",
            worst_norm - 1.0,
            worst_design - 1.0,
        );
    }
}
