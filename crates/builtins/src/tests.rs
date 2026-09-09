use super::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameters, BuiltinProcessReport, BuiltinResetKind,
    CHANNEL_SYMMETRY_LAST_POST_RAMP_READS, CHANNEL_SYMMETRY_OBSERVE_POST_RAMP,
    CHANNEL_SYMMETRY_PREDICATE_CALLS, Cell, ChannelParameters, DualMonoBlock, InputStage,
    Matrix2x2, Simd4, Simd8, prepare_sections, test_support,
};

fn selected_snapshot(metrics: super::MeterMetricSet) -> super::MeterSnapshot {
    let config = super::MeterConfig {
        period_frames: core::num::NonZeroU32::new(8).unwrap(),
        peak_hold_frames: 2,
        peak_decay_db_per_second: 12.0,
        queue_capacity: core::num::NonZeroUsize::new(2).unwrap(),
        reset_generation: 7,
    };
    let handle = super::MeterHandle(core::num::NonZeroU64::new(9).unwrap());
    let mut prepared = super::MeterAccumulator::prepare_selected(handle, config, 48_000, metrics)
        .expect("valid meter selection");
    let samples = [
        0.5,
        -1.0,
        f32::NAN,
        f32::from_bits(1),
        0.25,
        f32::INFINITY,
        -0.75,
        -0.0,
    ];
    prepared
        .accumulator
        .observe(&samples[..3], &samples[..3], 10)
        .unwrap();
    prepared
        .accumulator
        .observe(&samples[3..], &samples[3..], 13)
        .unwrap();
    prepared.consumer.try_pop().expect("one complete window")
}

#[test]
fn meter_metric_subsets_match_full_and_peak_only_omits_work() {
    use core::sync::atomic::Ordering;

    let full = selected_snapshot(super::MeterMetricSet::ALL);
    for bits in 1..=super::MeterMetricSet::ALL.bits() {
        let metrics = super::MeterMetricSet::from_bits_retain(bits);
        let snapshot = selected_snapshot(metrics);
        assert_eq!(snapshot.present_metrics, metrics);
        assert_eq!(snapshot.frames, full.frames);
        assert_eq!(snapshot.start_sample, full.start_sample);
        assert_eq!(snapshot.end_sample, full.end_sample);
        if metrics.contains(super::MeterMetricSet::SAMPLE_PEAK) {
            assert_eq!(
                snapshot.left.sample_peak.to_bits(),
                full.left.sample_peak.to_bits()
            );
        }
        if metrics.contains(super::MeterMetricSet::ENERGY_RMS) {
            assert_eq!(snapshot.left.energy.to_bits(), full.left.energy.to_bits());
            assert_eq!(snapshot.left.rms.to_bits(), full.left.rms.to_bits());
        }
        if metrics.contains(super::MeterMetricSet::COUNTS) {
            assert_eq!(snapshot.left.clipped_samples, full.left.clipped_samples);
            assert_eq!(snapshot.left.sanitized_samples, full.left.sanitized_samples);
        }
        if metrics.contains(super::MeterMetricSet::HELD_PEAK) {
            assert_eq!(
                snapshot.left.held_peak.to_bits(),
                full.left.held_peak.to_bits()
            );
        }
    }

    super::meter_work_probe::ENERGY.store(0, Ordering::Relaxed);
    super::meter_work_probe::COUNTS.store(0, Ordering::Relaxed);
    super::meter_work_probe::HELD.store(0, Ordering::Relaxed);
    super::meter_work_probe::SQRT.store(0, Ordering::Relaxed);
    let peak = selected_snapshot(super::MeterMetricSet::SAMPLE_PEAK);
    assert_eq!(
        peak.left.sample_peak.to_bits(),
        full.left.sample_peak.to_bits()
    );
    assert_eq!(super::meter_work_probe::ENERGY.load(Ordering::Relaxed), 0);
    assert_eq!(super::meter_work_probe::COUNTS.load(Ordering::Relaxed), 0);
    assert_eq!(super::meter_work_probe::HELD.load(Ordering::Relaxed), 0);
    assert_eq!(super::meter_work_probe::SQRT.load(Ordering::Relaxed), 0);
}

#[test]
fn meter_rejects_empty_and_unknown_metric_bits_before_queue_allocation() {
    let config = super::MeterConfig {
        period_frames: core::num::NonZeroU32::MIN,
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: core::num::NonZeroUsize::MIN,
        reset_generation: 0,
    };
    let handle = super::MeterHandle(core::num::NonZeroU64::MIN);
    for metrics in [
        super::MeterMetricSet::from_bits_retain(0),
        super::MeterMetricSet::from_bits_retain(1 << 7),
    ] {
        assert!(matches!(
            super::MeterAccumulator::prepare_selected(handle, config, 48_000, metrics),
            Err(super::MeterConfigError::Metrics)
        ));
    }
}

#[test]
fn post_ramp_symmetry_mask_matches_lane_oracle() {
    let parameters = BuiltinParameters::default();
    let track = prepare_sections(48_000, parameters)
        .unwrap()
        .0
        .stage
        .lane_track(0);
    let tracks = [track; 8];
    check_post_ramp_mask::<f32>(&tracks[..1]);
    check_post_ramp_mask::<Simd4>(&tracks[..4]);
    check_post_ramp_mask::<Simd8>(&tracks[..8]);
}

fn check_post_ramp_mask<L: super::Lane>(tracks: &[super::PreparedInputTrack]) {
    let mut stage = InputStage::<L>::new(tracks);
    stage.refresh_channel_symmetry_post_ramp();
    let oracle = (0..stage.members).fold(0_u8, |mask, lane| {
        mask | u8::from(stage.compute_lane_channel_symmetry(lane)) << lane
    });
    assert_eq!(stage.symmetry, oracle);
}

#[test]
fn post_ramp_symmetry_handles_differing_words_countdowns_and_padding() {
    let track = prepare_sections(48_000, BuiltinParameters::default())
        .unwrap()
        .0
        .stage
        .lane_track(0);
    fn active_mask<L: super::Lane>(stage: &InputStage<L>) -> u8 {
        if stage.members == 8 {
            u8::MAX
        } else {
            (1_u8 << stage.members) - 1
        }
    }
    fn assert_matches<L: super::Lane>(stage: &mut InputStage<L>, expected: u8) {
        stage.refresh_channel_symmetry_post_ramp();
        let oracle = (0..stage.members).fold(0_u8, |mask, lane| {
            mask | u8::from(stage.compute_lane_channel_symmetry(lane)) << lane
        });
        assert_eq!(stage.symmetry, oracle);
        assert_eq!(stage.symmetry, expected);
        let active = active_mask(stage);
        assert_eq!(stage.symmetry & !active, 0);
    }
    fn assert_independent_case<L: super::Lane>(
        tracks: &[super::PreparedInputTrack],
        toggle: fn(&mut InputStage<L>, usize),
    ) {
        let mut stage = InputStage::<L>::new(tracks);
        let lane = 0;
        let active = active_mask(&stage);
        assert_matches(&mut stage, active);

        toggle(&mut stage, lane);
        assert_matches(&mut stage, active & !(1 << lane));
        if stage.members > 1 {
            assert_ne!(
                stage.symmetry & (1 << 1),
                0,
                "an unaffected member stays set"
            );
        }

        toggle(&mut stage, lane);
        assert_matches(&mut stage, active);
    }
    fn exercise<L: super::Lane>(tracks: &[super::PreparedInputTrack]) {
        macro_rules! toggled_word {
            ($field:expr, $lane:expr) => {{
                let mut words = super::lane_read($field);
                words[$lane] = f32::from_bits(words[$lane].to_bits() ^ 1);
                $field = super::lane_words(&words);
            }};
        }
        assert_independent_case::<L>(tracks, |stage, lane| {
            toggled_word!(stage.coef.trim[1], lane);
        });
        assert_independent_case::<L>(tracks, |stage, lane| {
            toggled_word!(stage.ramp.target[1], lane);
        });
        assert_independent_case::<L>(tracks, |stage, lane| {
            toggled_word!(stage.ramp.step[1], lane);
        });
        assert_independent_case::<L>(tracks, |stage, lane| {
            stage.remaining[1][lane] ^= 1;
        });
        macro_rules! coefficient_cases {
            ($section:expr) => {
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].c1, lane);
                });
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].a2, lane);
                });
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].a3, lane);
                });
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].m0, lane);
                });
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].m1, lane);
                });
                assert_independent_case::<L>(tracks, |stage, lane| {
                    toggled_word!(stage.coef.section[1][$section].m2, lane);
                });
            };
        }
        coefficient_cases!(0);
        coefficient_cases!(1);

        let mut signed_zero = InputStage::<L>::new(tracks);
        let active = active_mask(&signed_zero);
        let mut left = super::lane_read(signed_zero.coef.trim[0]);
        let mut right = super::lane_read(signed_zero.coef.trim[1]);
        left[0] = 0.0;
        right[0] = -0.0;
        signed_zero.coef.trim = [super::lane_words(&left), super::lane_words(&right)];
        assert_matches(&mut signed_zero, active & !1);
        right[0] = 0.0;
        signed_zero.coef.trim[1] = super::lane_words(&right);
        assert_matches(&mut signed_zero, active);
    }
    let tracks = [track; 8];
    exercise::<f32>(&tracks[..1]);
    exercise::<Simd4>(&tracks[..3]);
    exercise::<Simd4>(&tracks[..4]);
    exercise::<Simd8>(&tracks[..5]);
    exercise::<Simd8>(&tracks[..8]);
}

#[test]
fn post_ramp_symmetry_extracts_each_word_once() {
    let parameters = BuiltinParameters::default();
    let track = prepare_sections(48_000, parameters)
        .unwrap()
        .0
        .stage
        .lane_track(0);
    let mut stage = InputStage::<Simd8>::new(&[track; 8]);
    for lane in 0..8 {
        stage.set_trim_db(lane, BuiltinLaneSelector::Both, 2.0, 8);
    }
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| observe.set(true));
    let mut left = vec![0.0; 8 * 8];
    let mut right = vec![0.0; 8 * 8];
    stage.process(&mut left, &mut right, 8);
    let extractions = CHANNEL_SYMMETRY_LAST_POST_RAMP_READS.with(Cell::get);
    let oracle = (0..stage.members).fold(0_u8, |mask, lane| {
        mask | u8::from(stage.compute_lane_channel_symmetry(lane)) << lane
    });
    assert_eq!(stage.symmetry, oracle);
    assert_eq!(
        extractions, 30,
        "one extraction per side of each of 15 word pairs"
    );

    for lane in 0..8 {
        stage.set_trim_db(lane, BuiltinLaneSelector::Both, 3.0, 8);
    }
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| observe.set(true));
    let mut mono = vec![0.0; 8 * 8];
    stage.process_mono(&mut mono, 8);
    let mono_extractions = CHANNEL_SYMMETRY_LAST_POST_RAMP_READS.with(Cell::get);
    let mono_oracle = (0..stage.members).fold(0_u8, |mask, lane| {
        mask | u8::from(stage.compute_lane_channel_symmetry(lane)) << lane
    });
    assert_eq!(stage.symmetry, mono_oracle);
    assert_eq!(mono_extractions, 30);
}

#[test]
fn post_ramp_symmetry_helper_is_off_for_settled_blocks() {
    let track = prepare_sections(48_000, BuiltinParameters::default())
        .unwrap()
        .0
        .stage
        .lane_track(0);
    let mut stage = InputStage::<Simd8>::new(&[track; 8]);
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| observe.set(true));
    let mut left = vec![0.0; 8 * 4];
    let mut right = vec![0.0; 8 * 4];
    stage.process(&mut left, &mut right, 4);
    assert_eq!(
        CHANNEL_SYMMETRY_LAST_POST_RAMP_READS.with(Cell::get),
        usize::MAX
    );
    CHANNEL_SYMMETRY_OBSERVE_POST_RAMP.with(|observe| observe.set(false));
}

fn process_reference(
    chain: &mut BuiltinChain,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
) -> BuiltinProcessReport {
    let report = chain.process_input(DualMonoBlock::new(left, right, first_sample).unwrap());
    chain.process_fader_mute(DualMonoBlock::new(left, right, first_sample).unwrap());
    chain.process_matrix(DualMonoBlock::new(left, right, first_sample).unwrap());
    report
}

#[test]
fn lane_symmetry_retarget_updates_only_the_addressed_predicate() {
    fn track(left_db: f32, right_db: f32) -> super::PreparedInputTrack {
        let parameters = BuiltinParameters {
            left: ChannelParameters {
                trim_db: left_db,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                trim_db: right_db,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        };
        prepare_sections(48_000, parameters)
            .unwrap()
            .0
            .stage
            .lane_track(0)
    }

    let tracks = [
        track(1.0, 0.0),
        track(1.0, 3.0),
        track(2.0, 2.0),
        track(-4.0, 1.0),
        track(5.0, 5.0),
        track(7.0, -2.0),
        track(-8.0, -8.0),
        track(9.0, 4.0),
    ];

    enum Retarget {
        Trim(BuiltinLaneSelector, f32, u32),
        Polarity(BuiltinLaneSelector, bool, u32),
    }

    fn retarget_and_check<L: super::Lane>(
        stage: &mut InputStage<L>,
        lane: usize,
        retarget: Retarget,
    ) {
        let before = stage.symmetry;
        CHANNEL_SYMMETRY_PREDICATE_CALLS.with(|calls| calls.set(0));
        match retarget {
            Retarget::Trim(selector, gain, smoothing_samples) => {
                stage.set_trim_db(lane, selector, gain, smoothing_samples);
            }
            Retarget::Polarity(selector, inverted, smoothing_samples) => {
                stage.set_polarity_invert(lane, selector, inverted, smoothing_samples);
            }
        }

        // Capture the setter's interval before the independent full-definition oracle below
        // evaluates any predicates of its own. This named assertion is also the mutation
        // control: restoring the old full refresh must fail here on a multi-member bank.
        let setter_predicate_calls = CHANNEL_SYMMETRY_PREDICATE_CALLS.with(Cell::get);
        assert_eq!(
            setter_predicate_calls, 1,
            "one retarget evaluates only the addressed lane"
        );

        let active_mask = if stage.members == u8::BITS as usize {
            u8::MAX
        } else {
            (1_u8 << stage.members) - 1
        };
        let mut oracle = 0_u8;
        for candidate in 0..stage.members {
            if stage.compute_lane_channel_symmetry(candidate) {
                oracle |= 1 << candidate;
            }
        }
        assert_eq!(stage.symmetry & active_mask, oracle);
        assert_eq!(
            stage.symmetry & !(1 << lane),
            before & !(1 << lane),
            "a retarget changed an unaddressed or padding bit"
        );
        assert_eq!(
            stage.symmetry & !active_mask,
            0,
            "padding bits must stay clear"
        );
    }

    macro_rules! check_width {
        ($lane:ty, $members:expr, $addressed:expr, $different:expr) => {{
            let mut stage = InputStage::<$lane>::new(&tracks[..$members]);
            let addressed_bit = 1_u8 << $addressed;
            let mut saw_false_to_true = false;
            let mut saw_true_to_false = false;

            macro_rules! operation {
                ($target:expr, $operation:expr) => {{
                    let before = stage.symmetry;
                    retarget_and_check(&mut stage, $target, $operation);
                    saw_false_to_true |=
                        before & (1 << $target) == 0 && stage.symmetry & (1 << $target) != 0;
                    saw_true_to_false |=
                        before & (1 << $target) != 0 && stage.symmetry & (1 << $target) == 0;
                }};
            }

            // Re-equalize, split, and re-equalize the same nonzero lane through the actual
            // trim/polarity operations and all three selectors.
            operation!(
                $addressed,
                Retarget::Trim(BuiltinLaneSelector::Right, super::db_gain(1.0).unwrap(), 0,)
            );
            operation!(
                $addressed,
                Retarget::Polarity(BuiltinLaneSelector::Left, true, 0)
            );
            operation!(
                $addressed,
                Retarget::Polarity(BuiltinLaneSelector::Left, false, 0)
            );
            operation!(
                $addressed,
                Retarget::Trim(BuiltinLaneSelector::Both, super::db_gain(2.0).unwrap(), 8,)
            );

            // Multi-member banks then retarget a different lane and return to the first one;
            // the scalar case repeats its sole lane instead.
            operation!(
                $different,
                Retarget::Trim(BuiltinLaneSelector::Right, super::db_gain(1.0).unwrap(), 0,)
            );
            operation!(
                $different,
                Retarget::Polarity(BuiltinLaneSelector::Right, true, 8)
            );
            operation!(
                $different,
                Retarget::Polarity(BuiltinLaneSelector::Both, false, 0)
            );
            operation!(
                $addressed,
                Retarget::Polarity(BuiltinLaneSelector::Both, false, 0)
            );

            assert!(
                saw_false_to_true,
                "members={}, addressed={}, mask={:#x} never became symmetric",
                $members, $addressed, stage.symmetry
            );
            assert!(
                saw_true_to_false,
                "members={}, addressed={}, mask={:#x} never became asymmetric",
                $members, $addressed, stage.symmetry
            );
            if $members > 1 {
                assert_ne!(
                    addressed_bit,
                    1_u8 << $different,
                    "multi-member evidence must address different lanes"
                );
            }
        }};
    }

    check_width!(Simd4, 3, 1, 0);
    check_width!(Simd4, 4, 1, 0);
    check_width!(Simd8, 5, 1, 0);
    check_width!(Simd8, 8, 1, 0);
    check_width!(f32, 1, 0, 0);
}

fn assert_pair(
    dut: &mut BuiltinChain,
    reference: &mut BuiltinChain,
    mut left: Vec<f32>,
    mut right: Vec<f32>,
    first_sample: u64,
    fused: bool,
) {
    let mut old_left = left.clone();
    let mut old_right = right.clone();
    let before = dut.fused_dispatches;
    let report =
        dut.process_dual_mono(DualMonoBlock::new(&mut left, &mut right, first_sample).unwrap());
    let old_report = process_reference(reference, &mut old_left, &mut old_right, first_sample);
    assert_eq!(
        left.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        old_left.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(
        right.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        old_right.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(report, old_report);
    assert_eq!(
        test_support::input_state_words(test_support::chain_input(dut)),
        test_support::input_state_words(test_support::chain_input(reference)),
    );
    assert_eq!(
        test_support::matrix_current(test_support::chain_matrix(dut)),
        test_support::matrix_current(test_support::chain_matrix(reference)),
    );
    assert_eq!(
        dut.fused_dispatches - before,
        u32::from(fused),
        "selected path witness"
    );
}

fn set_matrix(
    dut: &mut BuiltinChain,
    reference: &mut BuiltinChain,
    target: Matrix2x2,
    samples: u32,
) {
    dut.matrix.set_target_smoothed(target, samples).unwrap();
    reference
        .matrix
        .set_target_smoothed(target, samples)
        .unwrap();
}

#[test]
fn full_public_chain_matches_the_three_section_reference() {
    let enabled = BuiltinParameters {
        left: ChannelParameters {
            polarity_invert: true,
            trim_db: -3.0,
            hpf_hz: 80.0,
            lpf_hz: 16_000.0,
            fader_db: -4.0,
            muted: false,
        },
        right: ChannelParameters {
            polarity_invert: false,
            trim_db: 2.0,
            hpf_hz: 140.0,
            lpf_hz: 12_000.0,
            fader_db: -9.0,
            muted: true,
        },
        matrix: Matrix2x2 {
            ll: 0.75,
            lr: -0.2,
            rl: 0.35,
            rr: 0.9,
        },
        smoothing_samples: 0,
    };
    let mut dut = BuiltinChain::new(48_000, enabled).unwrap();
    let mut reference = BuiltinChain::new(48_000, enabled).unwrap();
    assert_pair(
        &mut dut,
        &mut reference,
        vec![
            0.25,
            -0.5,
            f32::NAN,
            0.75,
            f32::INFINITY,
            -0.0,
            0.125,
            -0.25,
        ],
        vec![-0.75, 0.5, 0.25, f32::NEG_INFINITY, 1.0, 0.0, -0.125, 0.375],
        0,
        true,
    );

    let bad = [f32::INFINITY.to_bits(); 8];
    test_support::set_input_state_words(test_support::chain_input_mut(&mut dut), bad);
    test_support::set_input_state_words(test_support::chain_input_mut(&mut reference), bad);
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.1, -0.2, 0.3, -0.4],
        vec![0.4, -0.3, 0.2, -0.1],
        8,
        true,
    );

    let disabled = BuiltinParameters {
        left: ChannelParameters {
            fader_db: -6.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            polarity_invert: true,
            trim_db: 1.0,
            fader_db: 3.0,
            ..ChannelParameters::default()
        },
        matrix: Matrix2x2 {
            ll: 0.6,
            lr: 0.25,
            rl: -0.4,
            rr: 0.8,
        },
        smoothing_samples: 0,
    };
    let mut dut = BuiltinChain::new(96_000, disabled).unwrap();
    let mut reference = BuiltinChain::new(96_000, disabled).unwrap();
    assert_pair(
        &mut dut,
        &mut reference,
        vec![-0.0, 0.0, 0.5, -0.25, 0.75],
        vec![0.0, -0.0, -0.5, 0.25, -0.75],
        100,
        true,
    );
}

#[test]
fn eligibility_sequence_uses_whole_call_fallback_then_fuses_the_next_call() {
    let parameters = BuiltinParameters::default();
    let mut dut = BuiltinChain::new(48_000, parameters).unwrap();
    let mut reference = BuiltinChain::new(48_000, parameters).unwrap();
    let a = Matrix2x2 {
        ll: 0.8,
        lr: 0.2,
        rl: -0.1,
        rr: 0.9,
    };
    let b = Matrix2x2 {
        ll: 0.5,
        lr: -0.3,
        rl: 0.4,
        rr: 0.7,
    };

    set_matrix(&mut dut, &mut reference, a, 0);
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.2; 3],
        vec![-0.4; 3],
        0,
        true,
    );

    set_matrix(&mut dut, &mut reference, b, 6);
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.3; 2],
        vec![-0.2; 2],
        3,
        false,
    );
    set_matrix(&mut dut, &mut reference, a, 5);
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.4; 2],
        vec![0.1; 2],
        5,
        false,
    );
    assert_pair(
        &mut dut,
        &mut reference,
        vec![-0.25; 7],
        vec![0.5; 7],
        7,
        false,
    );
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.75; 3],
        vec![-0.5; 3],
        14,
        true,
    );

    dut.input
        .set_trim_db(BuiltinLaneSelector::Left, -3.0, 4)
        .unwrap();
    reference
        .input
        .set_trim_db(BuiltinLaneSelector::Left, -3.0, 4)
        .unwrap();
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.1; 5],
        vec![0.2; 5],
        17,
        true,
    );

    set_matrix(&mut dut, &mut reference, b, 9);
    dut.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    reference.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    assert_pair(
        &mut dut,
        &mut reference,
        vec![0.6; 4],
        vec![-0.3; 4],
        22,
        true,
    );
}
