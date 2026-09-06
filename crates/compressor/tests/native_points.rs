//! Native parameter Point/readback semantics.

mod support;

use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, ObservationSample, ParameterAccessError,
    ParameterChannel, PreparedAutomationSpan, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedParameterState, ProcessReport, ResetKind, StatePayloadError, StatePayloadInput,
    StatePayloadOutput,
};

use support::{initial_values, noise, prepare, render_scalar, request, snapshot};

const WARM_FRAMES: usize = 1_152;
const QUANTUM: u32 = 128;
const RAMP_SAMPLES: u32 = 64;

fn point_at(
    sample: u64,
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: sample,
        end_sample: sample,
        start_value: value,
        end_value: value,
    }
}

fn asymmetric_values() -> [effect_contract::InitialParameterValue; 16] {
    let pairs = [
        (-31.0, -13.0),
        (3.0, 7.0),
        (4.0, 10.0),
        (8.0, 30.0),
        (80.0, 300.0),
        (-3.0, 4.0),
        (0.25, 0.75),
        (10.0, 15.0),
    ];
    core::array::from_fn(|slot| effect_contract::InitialParameterValue {
        parameter_index: (slot / 2) as u32,
        channel: if slot % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: if slot % 2 == 0 {
            pairs[slot / 2].0
        } else {
            pairs[slot / 2].1
        },
    })
}

fn read_f32(payload: &[u8], word: usize) -> f32 {
    f32::from_le_bytes(payload[word * 4..word * 4 + 4].try_into().unwrap())
}

fn read_u32(payload: &[u8], word: usize) -> u32 {
    u32::from_le_bytes(payload[word * 4..word * 4 + 4].try_into().unwrap())
}

fn ramp_word(parameter: usize, field: usize) -> usize {
    3 + parameter * 3 + field
}

fn assert_state_bits(actual: PreparedParameterState, current: f32, target: f32) {
    assert_eq!(actual.current_value.to_bits(), current.to_bits());
    assert_eq!(actual.target_value.to_bits(), target.to_bits());
}

fn payload_state(payload: &[u8], parameter: usize) -> PreparedParameterState {
    if parameter == 7 {
        let value = read_f32(payload, 1);
        PreparedParameterState {
            current_value: value,
            target_value: value,
        }
    } else {
        PreparedParameterState {
            current_value: read_f32(payload, ramp_word(parameter, 0)),
            target_value: read_f32(payload, ramp_word(parameter, 1)),
        }
    }
}

fn assert_read_rejected(
    effect: &dyn PreparedNativeEffect,
    parameter: u32,
    channel: ParameterChannel,
    expected: ParameterAccessError,
) {
    let before = snapshot(effect);
    assert_eq!(effect.parameter_state(parameter, channel), Err(expected));
    assert_eq!(snapshot(effect), before, "rejected read changed payload");
}

fn assert_apply_rejected(
    effect: &mut dyn PreparedNativeEffect,
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
    expected: ParameterAccessError,
) {
    let before = snapshot(effect);
    assert_eq!(
        effect.apply_parameter_point(parameter, channel, value),
        Err(expected)
    );
    assert_eq!(snapshot(effect), before, "rejected apply changed payload");
}

fn assert_only_words_may_change(
    before: &(Vec<u8>, Vec<u8>),
    after: &(Vec<u8>, Vec<u8>),
    channel: ParameterChannel,
    words: &[usize],
) {
    for (lane, (old, new)) in [(&before.0, &after.0), (&before.1, &after.1)]
        .into_iter()
        .enumerate()
    {
        assert_eq!(old.len(), new.len());
        let selected = matches!(
            (lane, channel),
            (0, ParameterChannel::Left) | (1, ParameterChannel::Right)
        );
        for byte in 0..old.len() {
            if !(selected && words.contains(&(byte / 4))) {
                assert_eq!(
                    old[byte], new[byte],
                    "unexpected payload change at lane {lane}, byte {byte}"
                );
            }
        }
    }
}

fn observe(effect: &dyn PreparedNativeEffect) -> ObservationSample {
    let mut sample = ObservationSample::default();
    assert!(effect.observe_resident(0, &mut sample));
    sample
}

fn assert_observation_bits(left: ObservationSample, right: ObservationSample) {
    assert_eq!(left.left.to_bits(), right.left.to_bits());
    assert_eq!(left.right.to_bits(), right.right.to_bits());
}

fn assert_pcm_bits(left: &[f32], right: &[f32]) {
    assert_eq!(left.len(), right.len());
    for (index, (&left, &right)) in left.iter().zip(right).enumerate() {
        assert_eq!(
            left.to_bits(),
            right.to_bits(),
            "PCM differs at sample {index}"
        );
    }
}

fn warm(effect: &mut dyn PreparedNativeEffect) -> (Vec<f32>, Vec<f32>, ProcessReport) {
    let mut left = noise(WARM_FRAMES, 0x0052_4011, 0.73);
    let mut right = noise(WARM_FRAMES, 0x0052_4022, 0.61);
    let report = render_scalar(effect, &mut left, &mut right, 128, QUANTUM, &[]);
    (left, right, report)
}

fn assert_identical_continuation(
    left_effect: &mut dyn PreparedNativeEffect,
    right_effect: &mut dyn PreparedNativeEffect,
) {
    let mut left_a = noise(WARM_FRAMES, 0x0052_4033, 0.67);
    let mut right_a = noise(WARM_FRAMES, 0x0052_4044, 0.59);
    let mut left_b = left_a.clone();
    let mut right_b = right_a.clone();
    let report_a = render_scalar(left_effect, &mut left_a, &mut right_a, 128, QUANTUM, &[]);
    let report_b = render_scalar(right_effect, &mut left_b, &mut right_b, 128, QUANTUM, &[]);
    assert_eq!(report_a, report_b);
    assert_pcm_bits(&left_a, &left_b);
    assert_pcm_bits(&right_a, &right_b);
    assert_eq!(snapshot(left_effect), snapshot(right_effect));
    assert_observation_bits(observe(left_effect), observe(right_effect));
}

struct ForwardingWithoutParameterAccess {
    inner: Box<dyn PreparedNativeEffect>,
}

impl PreparedNativeEffect for ForwardingWithoutParameterAccess {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.inner.metadata()
    }
    fn reset(&mut self, kind: ResetKind) {
        self.inner.reset(kind);
    }
    fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
        self.inner.process(block)
    }
    fn observe_resident(&self, tap_index: u32, out: &mut ObservationSample) -> bool {
        self.inner.observe_resident(tap_index, out)
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.snapshot_state_payload(output)
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.inner.restore_state_payload(version, input)
    }
    fn channel_symmetry(&self) -> bool {
        self.inner.channel_symmetry()
    }
}

#[test]
fn native_parameter_access_is_typed_and_transactional() {
    let _fp = lane::CanonicalFpEnv::enter();
    let values = asymmetric_values();
    let mut rejected = prepare(request(&values));
    let mut rejected_reference = prepare(request(&values));
    let payload = snapshot(&*rejected);
    for parameter in 0..8 {
        for (channel, section) in [
            (ParameterChannel::Left, &payload.0),
            (ParameterChannel::Right, &payload.1),
        ] {
            let expected = payload_state(section, parameter as usize);
            let actual = rejected.parameter_state(parameter, channel).unwrap();
            assert_state_bits(actual, expected.current_value, expected.target_value);
        }
    }
    for parameter in [8, u32::MAX] {
        assert_read_rejected(
            &*rejected,
            parameter,
            ParameterChannel::Left,
            ParameterAccessError::InvalidParameterIndex,
        );
        assert_apply_rejected(
            &mut *rejected,
            parameter,
            ParameterChannel::Right,
            f32::NAN,
            ParameterAccessError::InvalidParameterIndex,
        );
    }
    assert_read_rejected(
        &*rejected,
        u32::MAX,
        ParameterChannel::Both,
        ParameterAccessError::InvalidParameterIndex,
    );
    assert_apply_rejected(
        &mut *rejected,
        u32::MAX,
        ParameterChannel::Both,
        f32::NAN,
        ParameterAccessError::InvalidParameterIndex,
    );
    assert_read_rejected(
        &*rejected,
        5,
        ParameterChannel::Both,
        ParameterAccessError::InvalidChannel,
    );
    assert_apply_rejected(
        &mut *rejected,
        5,
        ParameterChannel::Both,
        f32::NAN,
        ParameterAccessError::InvalidChannel,
    );
    assert_apply_rejected(
        &mut *rejected,
        7,
        ParameterChannel::Left,
        f32::NAN,
        ParameterAccessError::NotAutomatable,
    );
    assert_apply_rejected(
        &mut *rejected,
        7,
        ParameterChannel::Both,
        f32::NAN,
        ParameterAccessError::InvalidChannel,
    );

    let bounds = [
        (-80.0, 0.0, -80.5, 0.5),
        (1.0, 20.0, 0.5, 20.5),
        (0.0, 24.0, -0.5, 24.5),
        (0.1, 200.0, 0.05, 200.5),
        (5.0, 5_000.0, 4.5, 5_000.5),
        (-24.0, 24.0, -24.5, 24.5),
        (0.0, 1.0, -0.1, 1.1),
    ];
    for (parameter, &(minimum, maximum, below, above)) in bounds.iter().enumerate() {
        for invalid in [below, above, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                assert_apply_rejected(
                    &mut *rejected,
                    parameter as u32,
                    channel,
                    invalid,
                    ParameterAccessError::InvalidValue,
                );
            }
        }
        for (channel, value) in [
            (ParameterChannel::Left, minimum),
            (ParameterChannel::Right, maximum),
        ] {
            let mut valid = prepare(request(&values));
            let before = snapshot(&*valid);
            let prior = valid.parameter_state(parameter as u32, channel).unwrap();
            valid
                .apply_parameter_point(parameter as u32, channel, value)
                .unwrap();
            let after = snapshot(&*valid);
            assert_state_bits(
                valid.parameter_state(parameter as u32, channel).unwrap(),
                prior.current_value,
                value,
            );
            assert_only_words_may_change(
                &before,
                &after,
                channel,
                &[ramp_word(parameter, 1), ramp_word(parameter, 2)],
            );
            let section = if channel == ParameterChannel::Left {
                &after.0
            } else {
                &after.1
            };
            assert_eq!(read_u32(section, ramp_word(parameter, 2)), RAMP_SAMPLES);
        }
    }

    let mut equal = prepare(request(&initial_values()));
    let mut equal_reference = prepare(request(&initial_values()));
    assert_read_rejected(
        &*equal,
        5,
        ParameterChannel::Both,
        ParameterAccessError::InvalidChannel,
    );
    assert_apply_rejected(
        &mut *equal,
        5,
        ParameterChannel::Both,
        6.0,
        ParameterAccessError::InvalidChannel,
    );
    assert_eq!(snapshot(&*equal), snapshot(&*equal_reference));
    assert_identical_continuation(&mut *equal, &mut *equal_reference);
    let mut zero = prepare(request(&values));
    zero.apply_parameter_point(5, ParameterChannel::Left, -0.0)
        .unwrap();
    assert_eq!(
        zero.parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .target_value
            .to_bits(),
        0.0_f32.to_bits()
    );
    let subnormal = f32::from_bits(1);
    zero.apply_parameter_point(2, ParameterChannel::Right, subnormal)
        .unwrap();
    assert_eq!(
        zero.parameter_state(2, ParameterChannel::Right)
            .unwrap()
            .target_value
            .to_bits(),
        subnormal.to_bits()
    );

    assert_eq!(snapshot(&*rejected), snapshot(&*rejected_reference));
    assert_identical_continuation(&mut *rejected, &mut *rejected_reference);

    let mut defaulted = ForwardingWithoutParameterAccess {
        inner: prepare(request(&values)),
    };
    let mut default_reference = prepare(request(&values));
    let before = snapshot(&defaulted);
    assert_eq!(
        defaulted.apply_parameter_point(u32::MAX, ParameterChannel::Both, f32::NAN),
        Err(ParameterAccessError::Unsupported)
    );
    assert_eq!(
        defaulted.parameter_state(u32::MAX, ParameterChannel::Both),
        Err(ParameterAccessError::Unsupported)
    );
    assert_eq!(
        defaulted.apply_parameter_point(5, ParameterChannel::Left, 6.0),
        Err(ParameterAccessError::Unsupported)
    );
    assert_eq!(
        defaulted.parameter_state(5, ParameterChannel::Left),
        Err(ParameterAccessError::Unsupported)
    );
    assert_eq!(snapshot(&defaulted), before);
    assert_identical_continuation(&mut defaulted, &mut *default_reference);
}

#[test]
fn native_point_changes_target_without_advancing_samples() {
    let _fp = lane::CanonicalFpEnv::enter();
    let values = asymmetric_values();
    let mut via_point = prepare(request(&values));
    let mut via_span = prepare(request(&values));
    let warm_point = warm(&mut *via_point);
    let warm_span = warm(&mut *via_span);
    assert_eq!(warm_point.2, warm_span.2);
    assert_pcm_bits(&warm_point.0, &warm_span.0);
    assert_pcm_bits(&warm_point.1, &warm_span.1);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));

    let before = snapshot(&*via_point);
    let state_before = via_point
        .parameter_state(5, ParameterChannel::Left)
        .unwrap();
    let observation_before = observe(&*via_point);
    via_point
        .apply_parameter_point(5, ParameterChannel::Left, 9.0)
        .unwrap();
    let after_first = snapshot(&*via_point);
    assert_only_words_may_change(
        &before,
        &after_first,
        ParameterChannel::Left,
        &[ramp_word(5, 1), ramp_word(5, 2)],
    );
    for _ in 0..2 {
        assert_state_bits(
            via_point
                .parameter_state(5, ParameterChannel::Left)
                .unwrap(),
            state_before.current_value,
            9.0,
        );
    }
    assert_observation_bits(observation_before, observe(&*via_point));
    assert_eq!(read_u32(&after_first.0, ramp_word(5, 2)), RAMP_SAMPLES);

    via_point
        .apply_parameter_point(5, ParameterChannel::Left, -7.0)
        .unwrap();
    let after_second = snapshot(&*via_point);
    assert_only_words_may_change(
        &after_first,
        &after_second,
        ParameterChannel::Left,
        &[ramp_word(5, 1), ramp_word(5, 2)],
    );
    assert_state_bits(
        via_point
            .parameter_state(5, ParameterChannel::Left)
            .unwrap(),
        state_before.current_value,
        -7.0,
    );
    assert_observation_bits(observation_before, observe(&*via_point));

    let mut point_left = noise(128, 0x0052_4101, 0.71);
    let mut point_right = noise(128, 0x0052_4102, 0.63);
    let mut span_left = point_left.clone();
    let mut span_right = point_right.clone();
    let point_report = via_point.process(
        EffectProcessBlock::new(
            &mut point_left,
            &mut point_right,
            None,
            WARM_FRAMES as u64,
            &[],
            QUANTUM,
        )
        .unwrap(),
    );
    let span = [point_at(
        WARM_FRAMES as u64,
        5,
        ParameterChannel::Left,
        -7.0,
    )];
    let span_report = via_span.process(
        EffectProcessBlock::new(
            &mut span_left,
            &mut span_right,
            None,
            WARM_FRAMES as u64,
            &span,
            QUANTUM,
        )
        .unwrap(),
    );
    assert_eq!(point_report, span_report);
    assert_pcm_bits(&point_left, &span_left);
    assert_pcm_bits(&point_right, &span_right);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_observation_bits(observe(&*via_point), observe(&*via_span));
}

fn advance(effect: &mut dyn PreparedNativeEffect, first_sample: &mut u64, frames: usize) {
    if frames == 0 {
        return;
    }
    let mut left = noise(frames, 0x0052_4201 ^ *first_sample, 0.55);
    let mut right = noise(frames, 0x0052_4202 ^ *first_sample, 0.47);
    effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, *first_sample, &[], QUANTUM).unwrap(),
    );
    *first_sample += frames as u64;
}

fn oracle_advance(mut current: f32, target: f32, samples: usize) -> f32 {
    let step = (target - current) / RAMP_SAMPLES as f32;
    for rendered in 0..samples {
        if rendered + 1 == RAMP_SAMPLES as usize {
            current = target;
        } else {
            current += step;
        }
    }
    current
}

#[test]
fn native_point_retarget_obeys_current_and_sample_count_laws() {
    let _fp = lane::CanonicalFpEnv::enter();
    let values = asymmetric_values();
    let mut effect = prepare(request(&values));
    let initial = effect.parameter_state(5, ParameterChannel::Left).unwrap();
    let right_initial = effect.parameter_state(5, ParameterChannel::Right).unwrap();
    effect
        .apply_parameter_point(5, ParameterChannel::Left, 16.0)
        .unwrap();

    let mut first_sample = 0;
    let mut processed = 0;
    for checkpoint in [0_usize, 1, 17, 63, 64] {
        advance(&mut *effect, &mut first_sample, checkpoint - processed);
        processed = checkpoint;
        let state = effect.parameter_state(5, ParameterChannel::Left).unwrap();
        assert_eq!(
            state.current_value.to_bits(),
            oracle_advance(initial.current_value, 16.0, checkpoint).to_bits()
        );
        assert_eq!(state.target_value.to_bits(), 16.0_f32.to_bits());
        let payload = snapshot(&*effect);
        assert_eq!(
            read_u32(&payload.0, ramp_word(5, 2)),
            RAMP_SAMPLES - checkpoint as u32
        );
        assert_state_bits(
            effect.parameter_state(5, ParameterChannel::Right).unwrap(),
            right_initial.current_value,
            right_initial.target_value,
        );
        assert_eq!(
            read_f32(&payload.1, ramp_word(5, 0)).to_bits(),
            right_initial.current_value.to_bits()
        );
        assert_eq!(read_u32(&payload.1, ramp_word(5, 2)), 0);
    }

    let mut restart = prepare(request(&values));
    let mut restart_sample = 0;
    restart
        .apply_parameter_point(5, ParameterChannel::Left, 12.0)
        .unwrap();
    advance(&mut *restart, &mut restart_sample, 7);
    let same_current = restart
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    restart
        .apply_parameter_point(5, ParameterChannel::Left, 12.0)
        .unwrap();
    assert_eq!(read_u32(&snapshot(&*restart).0, ramp_word(5, 2)), 64);
    advance(&mut *restart, &mut restart_sample, 1);
    assert_eq!(
        restart
            .parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .current_value
            .to_bits(),
        (same_current + (12.0 - same_current) / 64.0).to_bits()
    );

    advance(&mut *restart, &mut restart_sample, 5);
    let different_current = restart
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    restart
        .apply_parameter_point(5, ParameterChannel::Left, -11.0)
        .unwrap();
    assert_eq!(read_u32(&snapshot(&*restart).0, ramp_word(5, 2)), 64);
    advance(&mut *restart, &mut restart_sample, 1);
    assert_eq!(
        restart
            .parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .current_value
            .to_bits(),
        (different_current + (-11.0 - different_current) / 64.0).to_bits()
    );

    let cancel_current = restart
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    restart
        .apply_parameter_point(5, ParameterChannel::Left, cancel_current)
        .unwrap();
    assert_state_bits(
        restart.parameter_state(5, ParameterChannel::Left).unwrap(),
        cancel_current,
        cancel_current,
    );
    assert_eq!(read_u32(&snapshot(&*restart).0, ramp_word(5, 2)), 0);
    advance(&mut *restart, &mut restart_sample, 17);
    assert_state_bits(
        restart.parameter_state(5, ParameterChannel::Left).unwrap(),
        cancel_current,
        cancel_current,
    );
    assert_state_bits(
        restart.parameter_state(5, ParameterChannel::Right).unwrap(),
        right_initial.current_value,
        right_initial.target_value,
    );
}

fn assert_nonzero_ring_history(payload: &(Vec<u8>, Vec<u8>)) {
    assert!(payload.0[24 * 4..].iter().any(|&byte| byte != 0));
    assert!(payload.1[24 * 4..].iter().any(|&byte| byte != 0));
}

fn span_equivalence(parameter: u32, channel: ParameterChannel, target: f32) {
    let values = asymmetric_values();
    let mut via_point = prepare(request(&values));
    let mut via_span = prepare(request(&values));
    let warm_point = warm(&mut *via_point);
    let warm_span = warm(&mut *via_span);
    assert_eq!(warm_point.2, warm_span.2);
    assert_pcm_bits(&warm_point.0, &warm_span.0);
    assert_pcm_bits(&warm_point.1, &warm_span.1);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_nonzero_ring_history(&snapshot(&*via_point));

    via_point
        .apply_parameter_point(parameter, channel, target)
        .unwrap();
    let mut point_left = noise(128, 0x0052_4301 ^ parameter as u64, 0.69);
    let mut point_right = noise(128, 0x0052_4302 ^ parameter as u64, 0.57);
    let mut span_left = point_left.clone();
    let mut span_right = point_right.clone();
    let point_report = via_point.process(
        EffectProcessBlock::new(
            &mut point_left,
            &mut point_right,
            None,
            WARM_FRAMES as u64,
            &[],
            QUANTUM,
        )
        .unwrap(),
    );
    let span = [point_at(WARM_FRAMES as u64, parameter, channel, target)];
    let span_report = via_span.process(
        EffectProcessBlock::new(
            &mut span_left,
            &mut span_right,
            None,
            WARM_FRAMES as u64,
            &span,
            QUANTUM,
        )
        .unwrap(),
    );
    assert_eq!(point_report, span_report);
    assert_pcm_bits(&point_left, &span_left);
    assert_pcm_bits(&point_right, &span_right);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_observation_bits(observe(&*via_point), observe(&*via_span));

    if parameter == 5 && channel == ParameterChannel::Left {
        let mut no_point = prepare(request(&values));
        let no_point_warm = warm(&mut *no_point);
        assert_pcm_bits(&warm_point.0, &no_point_warm.0);
        assert_pcm_bits(&warm_point.1, &no_point_warm.1);
        let mut unchanged_left = noise(128, 0x0052_4301 ^ parameter as u64, 0.69);
        let mut unchanged_right = noise(128, 0x0052_4302 ^ parameter as u64, 0.57);
        no_point.process(
            EffectProcessBlock::new(
                &mut unchanged_left,
                &mut unchanged_right,
                None,
                WARM_FRAMES as u64,
                &[],
                QUANTUM,
            )
            .unwrap(),
        );
        assert!(
            point_left
                .iter()
                .zip(&unchanged_left)
                .any(|(&a, &b)| a.to_bits() != b.to_bits()),
            "makeup Point did not change PCM"
        );
    }
}

#[test]
fn native_point_matches_existing_span_pcm_and_state() {
    let _fp = lane::CanonicalFpEnv::enter();
    let targets = [-20.0, 9.0, 14.0, 40.0, 600.0, 12.0, 0.9];
    for (parameter, target) in targets.into_iter().enumerate() {
        span_equivalence(parameter as u32, ParameterChannel::Left, target);
        span_equivalence(parameter as u32, ParameterChannel::Right, target);
    }
}

fn process_silence(
    effect: &mut dyn PreparedNativeEffect,
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
) -> (Vec<f32>, Vec<f32>, ProcessReport) {
    let mut left = vec![0.0; 128];
    let mut right = vec![0.0; 128];
    let report = effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, first_sample, spans, QUANTUM).unwrap(),
    );
    (left, right, report)
}

fn assert_render_equal(
    left: &(Vec<f32>, Vec<f32>, ProcessReport),
    right: &(Vec<f32>, Vec<f32>, ProcessReport),
) {
    assert_eq!(left.2, right.2);
    assert_pcm_bits(&left.0, &right.0);
    assert_pcm_bits(&left.1, &right.1);
}

#[test]
fn native_point_invalidates_silent_fixed_point_without_advancing_it() {
    let _fp = lane::CanonicalFpEnv::enter();
    let values = initial_values();
    let mut via_point = prepare(request(&values));
    let mut via_span = prepare(request(&values));

    let eligible_point = process_silence(&mut *via_point, 0, &[]);
    let eligible_span = process_silence(&mut *via_span, 0, &[]);
    assert_render_equal(&eligible_point, &eligible_span);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));

    let stationary_before = snapshot(&*via_point);
    let stationary = via_point
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    via_point
        .apply_parameter_point(5, ParameterChannel::Left, stationary)
        .unwrap();
    assert_eq!(snapshot(&*via_point), stationary_before);
    let stationary_point = process_silence(&mut *via_point, 128, &[]);
    let stationary_spans = [point_at(128, 5, ParameterChannel::Left, stationary)];
    let stationary_span = process_silence(&mut *via_span, 128, &stationary_spans);
    assert_render_equal(&stationary_point, &stationary_span);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_observation_bits(observe(&*via_point), observe(&*via_span));

    let moving_before = snapshot(&*via_point);
    via_point
        .apply_parameter_point(5, ParameterChannel::Left, 1.0)
        .unwrap();
    let moving_after = snapshot(&*via_point);
    assert_only_words_may_change(
        &moving_before,
        &moving_after,
        ParameterChannel::Left,
        &[ramp_word(5, 1), ramp_word(5, 2)],
    );
    let moving_point = process_silence(&mut *via_point, 256, &[]);
    let moving_spans = [point_at(256, 5, ParameterChannel::Left, 1.0)];
    let moving_span = process_silence(&mut *via_span, 256, &moving_spans);
    assert_render_equal(&moving_point, &moving_span);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_observation_bits(observe(&*via_point), observe(&*via_span));

    let rejected_before = snapshot(&*via_point);
    assert_eq!(
        via_point.apply_parameter_point(5, ParameterChannel::Left, 25.0),
        Err(ParameterAccessError::InvalidValue)
    );
    assert_eq!(snapshot(&*via_point), rejected_before);
    let rejected_point = process_silence(&mut *via_point, 384, &[]);
    let rejected_reference = process_silence(&mut *via_span, 384, &[]);
    assert_render_equal(&rejected_point, &rejected_reference);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
    assert_observation_bits(observe(&*via_point), observe(&*via_span));
}
