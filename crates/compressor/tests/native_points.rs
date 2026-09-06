//! Native parameter Point/readback semantics.

mod support;

use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, ParameterAccessError, ParameterChannel,
    PreparedAutomationSpan,
};

use support::{initial_values, noise, prepare, request, snapshot, values_with};

fn point(parameter: u32, channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: 0,
        end_sample: 0,
        start_value: value,
        end_value: value,
    }
}

#[test]
fn native_parameter_access_is_typed_and_transactional() {
    let mut effect = prepare(request(&initial_values()));
    for index in 0..8 {
        assert!(
            effect
                .parameter_state(index, ParameterChannel::Left)
                .is_ok()
        );
        assert!(
            effect
                .parameter_state(index, ParameterChannel::Right)
                .is_ok()
        );
    }
    assert_eq!(
        effect.apply_parameter_point(8, ParameterChannel::Left, 0.0),
        Err(ParameterAccessError::InvalidParameterIndex)
    );
    assert_eq!(
        effect.apply_parameter_point(7, ParameterChannel::Left, 0.0),
        Err(ParameterAccessError::NotAutomatable)
    );
    assert_eq!(
        effect.apply_parameter_point(5, ParameterChannel::Both, f32::NAN),
        Err(ParameterAccessError::InvalidChannel)
    );
    assert_eq!(
        effect.apply_parameter_point(5, ParameterChannel::Left, f32::NAN),
        Err(ParameterAccessError::InvalidValue)
    );
    effect
        .apply_parameter_point(5, ParameterChannel::Left, -0.0)
        .unwrap();
    assert_eq!(
        effect
            .parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .target_value
            .to_bits(),
        0.0_f32.to_bits()
    );
}

#[test]
fn native_point_changes_target_without_advancing_samples() {
    let mut effect = prepare(request(&values_with(&[(5, -3.0)])));
    let before = snapshot(&*effect);
    let state_before = effect.parameter_state(5, ParameterChannel::Left).unwrap();
    effect
        .apply_parameter_point(5, ParameterChannel::Left, 9.0)
        .unwrap();
    let state_after = effect.parameter_state(5, ParameterChannel::Left).unwrap();
    assert_eq!(
        state_after.current_value.to_bits(),
        state_before.current_value.to_bits()
    );
    assert_eq!(state_after.target_value, 9.0);
    assert_ne!(snapshot(&*effect), before);
    effect
        .apply_parameter_point(5, ParameterChannel::Left, -7.0)
        .unwrap();
    assert_eq!(
        effect
            .parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .target_value,
        -7.0
    );
}

#[test]
fn native_point_retarget_obeys_current_and_sample_count_laws() {
    let mut effect = prepare(request(&initial_values()));
    effect
        .apply_parameter_point(5, ParameterChannel::Left, 16.0)
        .unwrap();
    let initial = effect
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    let step = (16.0 - initial) / 64.0;
    let mut left = noise(1, 0xA5A5, 0.25);
    let mut right = noise(1, 0x5A5A, 0.25);
    effect.process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).unwrap());
    let after = effect
        .parameter_state(5, ParameterChannel::Left)
        .unwrap()
        .current_value;
    assert_eq!(after.to_bits(), (initial + step).to_bits());
    assert_eq!(
        effect
            .parameter_state(5, ParameterChannel::Right)
            .unwrap()
            .target_value,
        0.0
    );
    effect
        .apply_parameter_point(5, ParameterChannel::Left, after)
        .unwrap();
    assert_eq!(
        effect
            .parameter_state(5, ParameterChannel::Left)
            .unwrap()
            .current_value
            .to_bits(),
        after.to_bits()
    );
}

#[test]
fn native_point_matches_existing_span_pcm_and_state() {
    let values = values_with(&[(7, 5.0)]);
    let mut via_point = prepare(request(&values));
    let mut via_span = prepare(request(&values));
    via_point
        .apply_parameter_point(5, ParameterChannel::Left, 12.0)
        .unwrap();
    let mut point_left = noise(128, 0x1010, 0.7);
    let mut point_right = noise(128, 0x2020, 0.7);
    let mut span_left = point_left.clone();
    let mut span_right = point_right.clone();
    via_point.process(
        EffectProcessBlock::new(&mut point_left, &mut point_right, None, 0, &[], 128).unwrap(),
    );
    via_span.process(
        EffectProcessBlock::new(
            &mut span_left,
            &mut span_right,
            None,
            0,
            &[point(5, ParameterChannel::Left, 12.0)],
            128,
        )
        .unwrap(),
    );
    assert_eq!(point_left, span_left);
    assert_eq!(point_right, span_right);
    assert_eq!(snapshot(&*via_point), snapshot(&*via_span));
}

#[test]
fn native_point_invalidates_silent_fixed_point_without_advancing_it() {
    let mut effect = prepare(request(&initial_values()));
    let mut left = vec![0.0; 128];
    let mut right = vec![0.0; 128];
    effect.process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).unwrap());
    let before = snapshot(&*effect);
    effect
        .apply_parameter_point(5, ParameterChannel::Left, 1.0)
        .unwrap();
    assert_ne!(snapshot(&*effect), before);
    assert_eq!(
        effect.apply_parameter_point(5, ParameterChannel::Left, 25.0),
        Err(ParameterAccessError::InvalidValue)
    );
}
