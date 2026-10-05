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

/// Gate 2, the countdown: a disable always restarts the 64-update countdown, even when the mix it
/// ramps already holds the identity. An enable from rest and a disable drained at the same block
/// boundary leave the recursion at the design and the mix at the identity; the disable must still
/// count down to the completion snap, which writes the identity recursion and clears the
/// integrators, so the section elides from frame 64.
///
/// Red: rule 2 restarting the countdown only when a mix word differs from its target. The section
/// then settles at the design recursion under the identity mix, with live integrators and an
/// identity target, and never elides.
#[test]
fn a_disable_in_the_enables_drain_still_completes_and_elides() {
    for rate in LAUNCH_RATES {
        for section in 0..2 {
            let mut input = disabled_at(rate);
            let hz = if section == 0 { 1_000.0 } else { 2_000.0 };
            input
                .apply_prepared_filter(design(rate, section, hz))
                .expect("enable");
            input
                .apply_prepared_filter(disable(rate, section))
                .expect("disable");
            for elapsed in 1..=128_u64 {
                render_frame(&mut input, elapsed, 0.5);
                let elided = builtins::test_support::input_elision_plan(&input);
                for (channel, elided) in elided.iter().enumerate() {
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
            // The kernel's frame-0 word, as it computes it: the start plus one step.
            let after_one = |start: f32, target: f32| start + (target - start) * (1.0 / 64.0);
            let identity_mix = [1.0_f32, 0.0, 0.0];
            let expected = [
                words[0],
                words[1],
                words[2],
                after_one(identity_mix[0], words[3]),
                after_one(identity_mix[1], words[4]),
                after_one(identity_mix[2], words[5]),
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
                after_one(start, words[index])
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

/// `||M||_V` in `f64` for a 2x2 matrix `M`: the operator norm induced by
/// `||x||_V^2 = x1^2 + sqrt(2) x1 x2 + x2^2`, the shared eigenbasis norm of every `k = sqrt(2)`
/// design. With `R = [[1, r], [0, r]]`, `r = 1/sqrt(2)` (so `||x||_V = ||R x||_2`), it is the
/// spectral norm of `R M R^-1`, `R^-1 = [[1, -1], [0, 1/r]]`.
fn v_norm_of(m: [[f64; 2]; 2]) -> f64 {
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let rm = [
        [m[0][0] + r * m[1][0], m[0][1] + r * m[1][1]],
        [r * m[1][0], r * m[1][1]],
    ];
    let b = [
        [rm[0][0], -rm[0][0] + rm[0][1] / r],
        [rm[1][0], -rm[1][0] + rm[1][1] / r],
    ];
    let frobenius_sq =
        b[0][0] * b[0][0] + b[0][1] * b[0][1] + b[1][0] * b[1][0] + b[1][1] * b[1][1];
    let determinant = b[0][0] * b[1][1] - b[0][1] * b[1][0];
    let discriminant = (frobenius_sq * frobenius_sq - 4.0 * determinant * determinant).max(0.0);
    (0.5 * (frobenius_sq + discriminant.sqrt())).sqrt()
}

/// `||A(w)||_V` in `f64` for the zero-input state step
/// `A(w) = [[1 - 2 c1, -2 a2], [2 a2, 1 - 2 a3]]` of the recursion words `w = (c1, a2, a3)`.
fn v_norm(c1: f32, a2: f32, a3: f32) -> f64 {
    let (c1, a2, a3) = (f64::from(c1), f64::from(a2), f64::from(a3));
    v_norm_of([[1.0 - 2.0 * c1, -2.0 * a2], [2.0 * a2, 1.0 - 2.0 * a3]])
}

/// The proven rounding allowance of the live retarget law at block size `quantum`: how far, per
/// recursion word `(c1, a2, a3)`, an `f32` word the kernel loads can lie from the convex hull of
/// the designs its history used (#1407 attempt 2; the proof is in the spec's "Numerical limits"
/// and in `docs/rulings/builtins-input-liveness-d2.md`).
///
/// A ramp from the word `c` to a design `t` has step `s = fl(t - c) / 64`. Its word after frame
/// `j` is the exact mixture `(1 - j/64) c + (j/64) t` up to
///
/// * `rho_j = j h + (j/64) |t - c| u` for `j <= 4`, stepped from `c` as `fl(w + s)`, and
/// * `rho_j = (1 - j/64) |t - c| (2u + u^2) + h` for `5 <= j <= 63`, computed from the target as
///   `fl(t - fl(s * (64 - j)))`,
///
/// with `u = 2^-24` and `h` the half-ulp of the word: `2^-25` for `c1` and `a3`, which never
/// exceed `1`, and `2^-26` for `a2`, which never exceeds `1 / (2 + sqrt(2))`. Every word lies
/// between its ramp's start and target, so `|t - c|` is at most the word's range over every design,
/// `D = (1, 0.3, 1)`. A ramp restarted from a word `e` off the hull carries `(1 - j/64) e` of it
/// (the contraction), so with restarts at least `quantum` frames apart a restart's start word is
/// off by at most `E_start = max over quantum <= j <= 63 of (64 / j) rho_j` (zero from `64`), and
/// any word by at most `E = max over 1 <= j <= 63 of (1 - j/64) E_start + rho_j`. A rule-2 freeze
/// holds a word, a rule-3 jump writes a design and the completion snap writes the target: none
/// adds error. At block sizes 1 to 4 this is `E = 64 h + u D`, the floor every law has (64
/// restarts' final roundings, contracted by `63/64`).
fn word_allowance(quantum: usize) -> [f64; 3] {
    // `2^-24`, `2^-25`, `2^-26`, exact in `f64`.
    let u = 1.0 / 16_777_216.0;
    let range = [1.0, 0.3, 1.0];
    let half_ulp = [u / 2.0, u / 4.0, u / 2.0];
    core::array::from_fn(|index| {
        let rho = |j: usize| {
            let j_f = j as f64;
            if j <= 4 {
                j_f * half_ulp[index] + j_f / 64.0 * range[index] * u
            } else {
                (64.0 - j_f) / 64.0 * range[index] * (2.0 * u + u * u) + half_ulp[index]
            }
        };
        let start = (quantum.max(1)..64)
            .map(|j| 64.0 / j as f64 * rho(j))
            .fold(0.0_f64, f64::max);
        (1..64)
            .map(|j| (64 - j) as f64 / 64.0 * start + rho(j))
            .fold(0.0_f64, f64::max)
    })
}

/// The `V`-norm the allowance can add: `A` is affine in the words, so a word `h + e` with `h` in
/// the hull has `||A(h + e)||_V <= ||A(h)||_V + 2 ||[[e1, e2], [-e2, e3]]||_V`, and the norm is
/// convex, so its supremum over the box `|e_i| <= E_i` is at a vertex.
fn norm_allowance(quantum: usize) -> f64 {
    let e = word_allowance(quantum);
    let mut worst = 0.0_f64;
    for signs in 0..8_u32 {
        let sign = |bit: u32| if signs & (1 << bit) == 0 { 1.0 } else { -1.0 };
        let (e1, e2, e3) = (sign(0) * e[0], sign(1) * e[1], sign(2) * e[2]);
        worst = worst.max(2.0 * v_norm_of([[e1, e2], [-e2, e3]]));
    }
    worst
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

/// The history kinds gate 4 draws, each judged against its own designs.
#[derive(Clone, Copy, Debug)]
enum History {
    /// Enables, disables, re-sends and retargets to designs log-uniform in `[10 Hz, max]`.
    LogUniform,
    /// The same, with one design draw in four taking `10 Hz` or the maximum cutoff exactly.
    Endpoints,
    /// Close retargets near the maximum cutoff (the design with the largest norm), restarted
    /// every block: `0` alternates the maximum and the next lower `f32` cutoff, `1` drags the
    /// cutoff up and down by three ulps a block, `2` alternates the maximum and 1 Hz below it,
    /// `3` is `0` with a disable every sixteenth block and an enable back on the next.
    Close(u8),
}

/// Gate 4: every recursion word any seeded history reaches, at every block size below the ramp,
/// has a `V`-norm no larger than the largest design norm that history used plus the proven
/// rounding allowance at that block size ([`norm_allowance`]). The identity recursion is
/// reachable only at rest, with both integrators `+0.0`, and no recursion word is subnormal.
///
/// Each history is judged against its own designs, so a log-uniform history is held at its own
/// largest norm (the lowest or highest cutoff it drew) and the maximum-cutoff design is reached by
/// the endpoint and close-retarget histories, not by every history. The numeric condition of the
/// stability proof -- the maximum-cutoff design's norm plus the allowance at block size 1 is below
/// 1 -- is asserted per rate.
#[test]
fn every_reachable_recursion_word_stays_inside_the_hull_of_the_designs() {
    let (rates, quanta): (&[u32], Vec<usize>) = if cfg!(debug_assertions) {
        (&[48_000], vec![1, 2, 7, 63])
    } else {
        (&LAUNCH_RATES, (1..=63).collect())
    };
    let histories: Vec<History> = (0..16)
        .map(|_| History::LogUniform)
        .chain([History::Endpoints, History::Endpoints])
        .chain((0..4).map(History::Close))
        .collect();
    for &rate in rates {
        let maximum = builtin_filter_cutoff_maximum_hz(rate).expect("rate");
        let maximum_hz = f64::from(maximum);
        let top = design(rate, 1, maximum).coefficients;
        let q_top = v_norm(top[0], top[1], top[2]);
        // The norm against the spectral radius at the maximum cutoff: they differ because the
        // `f32` words are not exactly `k = sqrt(2)`-consistent. Reported, not asserted.
        let radius = {
            let (c1, a2, a3) = (f64::from(top[0]), f64::from(top[1]), f64::from(top[2]));
            let (p, q, r, s) = (1.0 - 2.0 * c1, -2.0 * a2, 2.0 * a2, 1.0 - 2.0 * a3);
            let (trace, determinant) = (p + s, p * s - q * r);
            let discriminant = trace * trace - 4.0 * determinant;
            if discriminant < 0.0 {
                determinant.sqrt()
            } else {
                ((trace.abs() + discriminant.sqrt()) / 2.0).max(0.0)
            }
        };
        let q_ramp_top = q_top + norm_allowance(1);
        assert!(
            q_ramp_top < 1.0,
            "rate {rate}: the stability proof needs {q_top} + {} < 1",
            norm_allowance(1)
        );
        // `[log-uniform, endpoints, close]`: the worst excess of a reached norm over its history's
        // largest design norm, and the allowance it was held to.
        let mut worst = [(f64::NEG_INFINITY, 0.0_f64); 3];
        for &quantum in &quanta {
            let allowance = norm_allowance(quantum);
            for (history, kind) in histories.iter().enumerate() {
                let mut rng = SplitMix(
                    u64::from(rate) << 32
                        ^ (quantum as u64) << 8
                        ^ history as u64
                        ^ 0x1407_0000_0000,
                );
                let mut input = disabled_at(rate);
                let mut sent = [disable(rate, 0), disable(rate, 1)];
                let mut q_design = [0.0_f64; 2];
                let mut reached = [0.0_f64; 2];
                let mut frame = 0_u64;
                let draw = |rng: &mut SplitMix, rate: u32, section: usize, endpoints: bool| {
                    let hz = match (endpoints, rng.next() % 8) {
                        (true, 0) => 10.0,
                        (true, 1) => maximum,
                        _ => math::exp(
                            math::log(10.0)
                                + rng.unit() * (math::log(maximum_hz) - math::log(10.0)),
                        ) as f32,
                    };
                    design(rate, section, hz.clamp(10.0, maximum))
                };
                if let History::Close(_) = kind {
                    // Settle both sections on the maximum-cutoff design first.
                    for (section, sent) in sent.iter_mut().enumerate() {
                        *sent = design(rate, section, maximum);
                        input.apply_prepared_filter(*sent).expect("start");
                    }
                    for _ in 0..128 {
                        render_frame(&mut input, frame, 0.25);
                        frame += 1;
                    }
                }
                for block in 0..512_u32 {
                    for section in 0..2 {
                        let target = match *kind {
                            History::LogUniform | History::Endpoints => match rng.next() % 4 {
                                0 | 3 => draw(
                                    &mut rng,
                                    rate,
                                    section,
                                    matches!(kind, History::Endpoints),
                                ),
                                1 => disable(rate, section),
                                _ => sent[section],
                            },
                            History::Close(pattern) => {
                                let lower = f32::from_bits(maximum.to_bits() - 1);
                                match pattern {
                                    3 if block % 16 == 15 => disable(rate, section),
                                    0 | 3 => design(
                                        rate,
                                        section,
                                        if block % 2 == 0 { lower } else { maximum },
                                    ),
                                    1 => {
                                        let k = block % 200;
                                        let k = if k < 100 { k } else { 200 - k };
                                        design(
                                            rate,
                                            section,
                                            f32::from_bits(maximum.to_bits() - 3 * k),
                                        )
                                    }
                                    _ => design(
                                        rate,
                                        section,
                                        if block % 2 == 0 {
                                            maximum - 1.0
                                        } else {
                                            maximum
                                        },
                                    ),
                                }
                            }
                        };
                        let c = target.coefficients;
                        if c[0..3] != [0.0, 0.0, 0.0] {
                            q_design[section] = q_design[section].max(v_norm(c[0], c[1], c[2]));
                        }
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
                                        "identity recursion with live state: rate {rate} \
                                         quantum {quantum} history {history}"
                                    );
                                    continue;
                                }
                                let [c1, a2, a3] =
                                    [words[0], words[1], words[2]].map(f32::from_bits);
                                assert!(
                                    [c1, a2, a3].iter().all(|word| word.is_normal()),
                                    "subnormal or non-finite recursion word {words:08x?}: \
                                     rate {rate} quantum {quantum} history {history}"
                                );
                                *reached = reached.max(v_norm(c1, a2, a3));
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
                        excess <= allowance,
                        "rate {rate} quantum {quantum} history {history} ({kind:?}) section \
                         {section}: reached {:.6e} over design {:.6e} by {excess:.3e} \
                         (allowance {allowance:.3e})",
                        reached - 1.0,
                        q_design - 1.0,
                    );
                    let slot = match kind {
                        History::LogUniform => 0,
                        History::Endpoints => 1,
                        History::Close(_) => 2,
                    };
                    if excess > worst[slot].0 {
                        worst[slot] = (excess, allowance);
                    }
                }
            }
        }
        println!(
            "rate {rate}: max-cutoff design q - 1 = {:.4e} (norm - spectral radius {:.2e}), \
             allowance at quantum 1 = {:.4e}, \
             q_ramp - 1 = {:.4e}; worst excess over the history's design (allowance at that \
             quantum): log-uniform {:.3e} ({:.3e}), endpoints {:.3e} ({:.3e}), close {:.3e} \
             ({:.3e})",
            q_top - 1.0,
            q_top - radius,
            norm_allowance(1),
            q_ramp_top - 1.0,
            worst[0].0,
            worst[0].1,
            worst[1].0,
            worst[1].1,
            worst[2].0,
            worst[2].1,
        );
    }
}

// ---- #1407 follow-up: the owner's leading countdown, per lane, at every width ------------

/// One section's ramp record as the A3 law defines it, for the oracle of gate 8.
#[derive(Clone, Copy)]
struct RampOracle {
    current: [f32; 6],
    target: [f32; 6],
    step: [f32; 6],
    remaining: u32,
}

impl RampOracle {
    fn at_rest(words: [u32; 7]) -> Self {
        let current =
            [words[0], words[1], words[2], words[4], words[5], words[6]].map(f32::from_bits);
        Self {
            current,
            target: current,
            step: [0.0; 6],
            remaining: 0,
        }
    }

    /// Rules 1 and 4 of the live retarget law; gate 8 draws only design-to-design retargets, so
    /// rules 2 and 3 never apply (asserted).
    fn retarget(&mut self, target: [f32; 6]) {
        let identity = [0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0].map(f32::to_bits);
        assert_ne!(target.map(f32::to_bits), identity);
        assert_ne!(self.current.map(f32::to_bits), identity);
        if self.remaining != 0 && self.target.map(f32::to_bits) == target.map(f32::to_bits) {
            return;
        }
        let mut changed = false;
        for ((step, current), target) in self.step.iter_mut().zip(self.current).zip(target) {
            if current.to_bits() == target.to_bits() {
                *step = 0.0;
            } else {
                *step = (target - current) * (1.0 / 64.0);
                changed = true;
            }
        }
        self.target = target;
        self.remaining = if changed { 64 } else { 0 };
    }

    /// One frame of the A3 law: the first four words step from the current word, every later
    /// word is `target - step * remaining`, a recursion word whose step is `+0.0` holds, and the
    /// last frame snaps to the target.
    fn frame(&mut self) {
        if self.remaining == 0 {
            return;
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            self.current = self.target;
            return;
        }
        let leading = self.remaining >= 64 - 4;
        for index in 0..6 {
            if index < 3 && self.step[index] == 0.0 {
                continue;
            }
            self.current[index] = if leading {
                self.current[index] + self.step[index]
            } else {
                self.target[index] - self.step[index] * self.remaining as f32
            };
        }
    }

    fn words(&self) -> [u32; 6] {
        self.current.map(f32::to_bits)
    }
}

/// What gate 8 renders: a bank through its dual or its collapsed body, or one scalar chain.
enum RampSubject {
    Bank {
        bank: Box<BuiltinInputBank>,
        mono: bool,
    },
    Scalar(Box<InputBuiltins>),
}

impl RampSubject {
    fn lanes(&self) -> usize {
        match self {
            Self::Bank { bank, .. } => bank.active_lanes(),
            Self::Scalar(_) => 1,
        }
    }

    fn apply(&mut self, lane: usize, target: PreparedInputFilterTarget) {
        match self {
            Self::Bank { bank, .. } => bank.apply_prepared_filter(lane, target).expect("target"),
            Self::Scalar(input) => input.apply_prepared_filter(target).expect("target"),
        }
    }

    fn render(&mut self, frames: usize, at: u64) {
        match self {
            Self::Bank { bank, mono } => {
                let lanes = bank.width().lanes() as usize;
                let mut left = vec![0.25_f32; frames * lanes];
                if *mono {
                    bank.process_mono(&mut left, frames as u32);
                } else {
                    let mut right = vec![-0.25_f32; frames * lanes];
                    bank.process(&mut left, &mut right, frames as u32);
                }
            }
            Self::Scalar(input) => {
                let mut left = vec![0.25_f32; frames];
                let mut right = vec![-0.25_f32; frames];
                input.process(DualMonoBlock::new(&mut left, &mut right, at).expect("block"));
            }
        }
    }

    /// `[channel * 2 + section]` sections of one lane.
    fn words(&self, lane: usize) -> [[u32; 7]; 4] {
        match self {
            Self::Bank { bank, .. } => builtins::test_support::bank_section_words(bank, lane),
            Self::Scalar(input) => builtins::test_support::input_section_words(input),
        }
    }
}

/// Gate 8 (#1407 verdict MINOR 1): the owner's per-lane leading countdown
/// (`InputStage::load_filter_leading`) gives every lane of every width exactly four stepped
/// words, through the dual body and the collapsed one. Each lane is retargeted design to design
/// on its own staggered schedule, re-sends and restarts included, so the lanes of one bank are
/// out of phase; blocks of 1 to 16 frames put block ends on every ramp frame, the four leading
/// ones among them. After every block, every current word of every lane, channel and section
/// must equal, bit for bit, an oracle of the A3 law ([`RampOracle::frame`]).
///
/// A floor off by one (three stepped words, so the proven `1.419e-5` no longer holds) or one
/// countdown shared by the bank (an out-of-phase lane steps from the target from its first word,
/// `P = 6.10e-5`, above the stability margin) moves a word at rounding level, and only this gate
/// sees it: gate 7 builds its own countdown and gate 3 checks frame 0 alone.
#[test]
fn every_lane_steps_exactly_four_words_from_its_own_countdown() {
    let mut subjects: Vec<(String, u32, RampSubject)> = Vec::new();
    for rate in [44_100_u32, 48_000] {
        let parameters = |lane: usize| {
            let channel = builtins::ChannelParameters {
                polarity_invert: false,
                trim_db: 0.0,
                hpf_hz: 30.0 + 17.0 * lane as f32,
                lpf_hz: 6_000.0 + 701.0 * lane as f32,
                fader_db: 0.0,
                muted: false,
            };
            BuiltinParameters {
                left: channel,
                right: channel,
                matrix: builtins::Matrix2x2::IDENTITY,
                smoothing_samples: 0,
            }
        };
        let chain = |lane: usize| {
            BuiltinChain::new(rate, parameters(lane))
                .expect("prepared")
                .into_input_builtins()
        };
        for width in effect_contract::BankWidth::ALL {
            for mono in [false, true] {
                let lanes = width.lanes() as usize;
                let bank =
                    BuiltinInputBank::new(width.backend(), *width, (0..lanes).map(chain).collect())
                        .expect("bank");
                assert!(bank.supports_mono_collapse());
                let name = format!(
                    "{width:?} {}",
                    if mono { "process_mono" } else { "process" }
                );
                subjects.push((
                    name,
                    rate,
                    RampSubject::Bank {
                        bank: Box::new(bank),
                        mono,
                    },
                ));
            }
        }
        subjects.push((
            "scalar".to_owned(),
            rate,
            RampSubject::Scalar(Box::new(chain(0))),
        ));
    }
    const HPF_HZ: [f32; 6] = [20.0, 45.0, 80.0, 160.0, 400.0, 700.0];
    const LPF_HZ: [f32; 6] = [800.0, 2_000.0, 5_000.0, 9_000.0, 15_000.0, 18_000.0];
    const BLOCKS: [usize; 9] = [1, 1, 2, 3, 4, 5, 7, 9, 16];
    for (name, rate, mut subject) in subjects {
        let mono = matches!(subject, RampSubject::Bank { mono: true, .. });
        let lanes = subject.lanes();
        let mut oracle: Vec<[RampOracle; 4]> = (0..lanes)
            .map(|lane| subject.words(lane).map(RampOracle::at_rest))
            .collect();
        let mut random = SplitMix(u64::from(rate) ^ ((lanes as u64) << 32) ^ u64::from(mono));
        let mut at = 0_u64;
        let mut leading_frames_seen = 0_usize;
        for block in 0..400 {
            for (lane, lane_oracle) in oracle.iter_mut().enumerate() {
                // Lane `lane` retargets on its own schedule: about one block in five, never in
                // step with its neighbours.
                if !random.next().is_multiple_of(5) {
                    continue;
                }
                let section = (random.next() % 2) as usize;
                let hz = if section == 0 {
                    HPF_HZ[(random.next() % 6) as usize]
                } else {
                    LPF_HZ[(random.next() % 6) as usize]
                };
                // The collapsed body is only ever handed a symmetric bank.
                let lanes_selector = match (mono, random.next() % 4) {
                    (false, 0) => BuiltinLaneSelector::Left,
                    (false, 1) => BuiltinLaneSelector::Right,
                    _ => BuiltinLaneSelector::Both,
                };
                let target = PreparedInputFilterTarget {
                    lanes: lanes_selector,
                    ..design(rate, section, hz)
                };
                subject.apply(lane, target);
                for channel in 0..2 {
                    let covered = match lanes_selector {
                        BuiltinLaneSelector::Left => channel == 0,
                        BuiltinLaneSelector::Right => channel == 1,
                        BuiltinLaneSelector::Both => true,
                    };
                    if covered {
                        lane_oracle[channel * 2 + section].retarget(target.coefficients);
                    }
                }
            }
            let frames = BLOCKS[(random.next() % BLOCKS.len() as u64) as usize];
            subject.render(frames, at);
            at += frames as u64;
            for (lane, lane_oracle) in oracle.iter_mut().enumerate() {
                let words = subject.words(lane);
                for (index, section) in lane_oracle.iter_mut().enumerate() {
                    for _ in 0..frames {
                        section.frame();
                    }
                    if (60..64).contains(&section.remaining) {
                        leading_frames_seen += 1;
                    }
                    let got = [
                        words[index][0],
                        words[index][1],
                        words[index][2],
                        words[index][4],
                        words[index][5],
                        words[index][6],
                    ];
                    assert_eq!(
                        got,
                        section.words(),
                        "{name} at {rate} Hz, block {block} ({frames} frames): lane {lane} \
                         channel {} section {} (countdown {})",
                        index / 2,
                        index % 2,
                        section.remaining,
                    );
                }
            }
        }
        // The schedule must end blocks inside the leading window, or the gate proves nothing.
        assert!(
            leading_frames_seen >= 4 * lanes,
            "{name} at {rate} Hz: {leading_frames_seen} block ends in a leading window"
        );
    }
}
