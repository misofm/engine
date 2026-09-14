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
