//! Causal identity paths preserve exact dry/wet bits while the envelope remains active.

mod support;

use effect_contract::{
    EffectProcessBlock, LinkMode, PreparedNativeEffect, PreparedSidechainPort, ResetKind,
};

use support::{noise, prepare, render_scalar, request, values_with};

const FRAMES: usize = 2_048;

fn render(
    effect: &mut dyn PreparedNativeEffect,
    input_left: &[f32],
    input_right: &[f32],
) -> (Vec<f32>, Vec<f32>) {
    let mut left = input_left.to_vec();
    let mut right = input_right.to_vec();
    render_scalar(effect, &mut left, &mut right, 128, 128, &[]);
    (left, right)
}

#[test]
fn bypass_preserves_exact_dry_bits_at_sample_zero() {
    let values = values_with(&[]);
    let mut preparation = request(&values);
    preparation.bypass = true;
    let mut effect = prepare(preparation);
    let input_left = noise(FRAMES, 0xB1_A5_00_01, 0.75);
    let input_right = noise(FRAMES, 0xB1_A5_00_02, 0.75);
    let (left, right) = render(effect.as_mut(), &input_left, &input_right);
    for index in 0..FRAMES {
        assert_eq!(left[index].to_bits(), input_left[index].to_bits());
        assert_eq!(right[index].to_bits(), input_right[index].to_bits());
    }
}

#[test]
fn zero_mix_is_dry_while_gain_reduction_runs() {
    let compressing = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0)]);
    let input_left = noise(FRAMES, 0xB1_A5_00_03, 0.75);
    let input_right = noise(FRAMES, 0xB1_A5_00_04, 0.75);

    let mut dry_values = compressing;
    dry_values[6 * 2].value = 0.0;
    dry_values[6 * 2 + 1].value = 0.0;
    let mut dry = prepare(request(&dry_values));
    let (dry_left, _) = render(dry.as_mut(), &input_left, &input_right);

    let mut wet_values = compressing;
    wet_values[6 * 2].value = 1.0;
    wet_values[6 * 2 + 1].value = 1.0;
    let mut wet = prepare(request(&wet_values));
    let (wet_left, _) = render(wet.as_mut(), &input_left, &input_right);

    for index in 0..FRAMES {
        assert_eq!(dry_left[index].to_bits(), input_left[index].to_bits());
    }
    assert!(
        wet_left[1_500].abs() < dry_left[1_500].abs() * 0.5,
        "the configuration must actually be compressing: {} vs {}",
        wet_left[1_500],
        dry_left[1_500]
    );
}

#[test]
fn unit_mix_is_the_wet_signal_exactly() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (5, 0.0), (6, 1.0)]);
    let mut preparation = request(&values);
    preparation.ports.sidechain = PreparedSidechainPort::Connected {
        id: support::sidechain_port(),
        required: false,
    };
    let mut effect = prepare(preparation);

    let dry = 0.5_f32;
    let sidechain = noise(FRAMES, 0xB1_A5_00_05, 0.9);
    let mut left = vec![dry; FRAMES];
    let mut right = vec![dry; FRAMES];
    let mut offset = 0;
    while offset < left.len() {
        effect.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                Some((
                    &sidechain[offset..offset + 128],
                    &sidechain[offset..offset + 128],
                )),
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        offset += 128;
    }

    let mut differs = 0;
    for wet in left.iter().copied() {
        let gain = wet / dry;
        assert_eq!((dry * gain).to_bits(), wet.to_bits());
        let two_step = (wet - dry) + dry;
        if two_step.to_bits() != wet.to_bits() {
            differs += 1;
        }
    }
    assert!(differs > 0, "the two-step mix form must differ somewhere");
}

#[test]
fn unity_gain_stage_is_the_dry_signal() {
    let values = values_with(&[(0, 0.0), (1, 4.0), (2, 0.0), (5, 0.0), (6, 0.5)]);
    let input_left = noise(FRAMES, 0xB1_A5_00_06, 0.01);
    let input_right = noise(FRAMES, 0xB1_A5_00_07, 0.01);
    let mut effect = prepare(request(&values));
    let (left, right) = render(effect.as_mut(), &input_left, &input_right);
    for index in 0..FRAMES {
        assert_eq!(left[index].to_bits(), input_left[index].to_bits());
        assert_eq!(right[index].to_bits(), input_right[index].to_bits());
    }
}

#[test]
fn average_link_is_two_products_and_an_add() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 1.0)]);
    let left_input = 0.3_f32;
    let right_input = -0.7_f32;
    let average = 0.5_f32 * left_input.abs() + 0.5_f32 * right_input.abs();

    let mut averaged_request = request(&values);
    averaged_request.link_mode = LinkMode::Average;
    let mut averaged = prepare(averaged_request);
    let mut left = vec![left_input; 1_536];
    let mut right = vec![right_input; 1_536];
    render_scalar(averaged.as_mut(), &mut left, &mut right, 128, 128, &[]);

    let mut equivalent_request = request(&values);
    equivalent_request.link_mode = LinkMode::Maximum;
    let mut equivalent = prepare(equivalent_request);
    let mut equivalent_left = vec![average; 1_536];
    let mut equivalent_right = vec![average; 1_536];
    render_scalar(
        equivalent.as_mut(),
        &mut equivalent_left,
        &mut equivalent_right,
        128,
        128,
        &[],
    );

    let (averaged_state, _) = support::snapshot(averaged.as_ref());
    let (equivalent_state, _) = support::snapshot(equivalent.as_ref());
    let averaged_gain = effect_runtime::state_payload::read_f32(&averaged_state, 0);
    let equivalent_gain = effect_runtime::state_payload::read_f32(&equivalent_state, 0);
    assert!(
        averaged_gain < -1.0,
        "G should be reducing gain, G is {averaged_gain}"
    );
    assert_eq!(averaged_gain.to_bits(), equivalent_gain.to_bits());
}

#[test]
fn bypass_keeps_the_state_warm() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 1.0)]);
    let input_left = noise(FRAMES, 0xB1_A5_00_08, 0.75);
    let input_right = noise(FRAMES, 0xB1_A5_00_09, 0.75);

    let mut live = prepare(request(&values));
    let _ = render(live.as_mut(), &input_left, &input_right);
    let (live_left, _) = support::snapshot(live.as_ref());

    let mut bypassed_request = request(&values);
    bypassed_request.bypass = true;
    let mut bypassed = prepare(bypassed_request);
    let _ = render(bypassed.as_mut(), &input_left, &input_right);
    let (bypassed_left, _) = support::snapshot(bypassed.as_ref());

    assert_eq!(live_left, bypassed_left);
    let gain_word = effect_runtime::state_payload::read_f32(&live_left, 0);
    assert!(gain_word < -1.0, "G should be below zero, is {gain_word}");

    live.reset(ResetKind::DiscontinuityKeepParameters);
    let (cleared, _) = support::snapshot(live.as_ref());
    assert_eq!(
        effect_runtime::state_payload::read_f32(&cleared, 0).to_bits(),
        0.0_f32.to_bits()
    );
}
