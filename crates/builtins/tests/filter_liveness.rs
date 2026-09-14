#![allow(missing_docs)]

use builtins::{
    BuiltinChain, BuiltinInputBank, BuiltinLaneSelector, BuiltinParameters, DualMonoBlock,
    InputBuiltins, PreparedInputFilterTarget, prepare_input_filter_pair,
};
use effect_contract::{ResponseSnapshotRequest, ResponseSnapshotSection};
use lane::Backend;

fn disabled() -> InputBuiltins {
    BuiltinChain::new(48_000, BuiltinParameters::default())
        .expect("prepared")
        .into_input_builtins()
}

fn bank() -> BuiltinInputBank {
    BuiltinInputBank::new(
        Backend::Simd4,
        effect_contract::BankWidth::Four,
        (0..4).map(|_| disabled()).collect(),
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
        for index in 0..3 {
            words[index] = prepared.targets[0].coefficients[index].to_bits();
        }
        words[3] = f32::from_bits(0x3fb5_04f3).to_bits();
        for index in 0..3 {
            words[index + 4] = prepared.targets[0].coefficients[index + 3].to_bits();
        }
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
        dual.apply_prepared_filter(target).expect("target");
        mono.apply_prepared_filter(target).expect("target");
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
