//! Scalar/bank identity, automation partitioning, and exact current-sample identity.

mod support;

use support::{
    active_values, assert_bits_eq, initial_values, noise, prepare, render_scalar_sidechain,
    request, request_at, retarget_spans, set_parameter, snapshot,
};

#[test]
fn partition_invariance_straddles_ramp_boundaries() {
    let values = active_values();
    let source_left = noise(5, 257, 0.4);
    let source_right = noise(7, 257, 0.4);
    let spans = retarget_spans(0);
    let mut one = prepare(request_at(&values, 48_000, 512));
    let mut one_left = source_left.clone();
    let mut one_right = source_right.clone();
    render_scalar_sidechain(
        &mut one,
        &mut one_left,
        &mut one_right,
        None,
        257,
        &spans,
        0,
    );
    for block in [1, 63, 64, 65, 127, 128, 129] {
        let mut split = prepare(request_at(&values, 48_000, 512));
        let mut left = source_left.clone();
        let mut right = source_right.clone();
        let mut start = 0;
        while start < left.len() {
            let end = (start + block).min(left.len());
            let batch = if start == 0 { &spans[..] } else { &[] };
            render_scalar_sidechain(
                &mut split,
                &mut left[start..end],
                &mut right[start..end],
                None,
                end - start,
                batch,
                start as u64,
            );
            start = end;
        }
        assert_bits_eq(&left, &one_left, &format!("left partition {block}"));
        assert_bits_eq(&right, &one_right, &format!("right partition {block}"));
    }
}

#[test]
fn bypass_preserves_current_signed_zero_and_advances_state() {
    let mut values = initial_values();
    set_parameter(&mut values, 0, -20.0, -20.0);
    set_parameter(&mut values, 1, 4.0, 4.0);
    set_parameter(&mut values, 2, 48.0, 48.0);
    set_parameter(&mut values, 5, 0.0, 0.0);
    let mut request = request(&values);
    request.bypass = true;
    let mut effect = prepare(request);
    let mut left = [f32::from_bits(0x8000_0000), 0.5, 0.0];
    let mut right = [0.0, -0.25, f32::from_bits(0x8000_0000)];
    render_scalar_sidechain(&mut effect, &mut left, &mut right, None, 3, &[], 0);
    assert_eq!(left[0].to_bits(), 0x8000_0000);
    assert_eq!(right[2].to_bits(), 0x8000_0000);
    let (_, payload, _) = snapshot(&effect);
    assert_ne!(u32::from_le_bytes(payload[0..4].try_into().unwrap()), 0);
}

#[test]
fn equal_input_is_dual_mono_and_zero_input_has_no_tail() {
    let mut effect = prepare(request(&active_values()));
    let mut equal_left = noise(701, 257, 0.3);
    let mut equal_right = equal_left.clone();
    render_scalar_sidechain(
        &mut effect,
        &mut equal_left,
        &mut equal_right,
        None,
        17,
        &[],
        0,
    );
    assert_bits_eq(&equal_left, &equal_right, "equal input remains dual-mono");

    let mut zero_left = vec![0.0_f32; 257];
    let mut zero_right = zero_left.clone();
    let report = render_scalar_sidechain(
        &mut effect,
        &mut zero_left,
        &mut zero_right,
        None,
        17,
        &[],
        257,
    );
    assert!(zero_left.iter().all(|sample| sample.to_bits() == 0));
    assert!(zero_right.iter().all(|sample| sample.to_bits() == 0));
    assert_eq!(report, effect_contract::ProcessReport::default());
}
