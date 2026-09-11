//! Scalar/bank identity, automation partitioning, and exact current-sample identity.

mod support;

use effect_contract::{BankWidth, EffectBankProcessBlock, LinkMode, PreparedNativeEffectBank};
use lane::Backend;
use support::{
    Values, active_values, assert_bits_eq, initial_values, noise, packed_w8, prepare, prepare_bank,
    prepare_bank_w8, render_scalar_sidechain, request, request_at, retarget_spans, set_parameter,
    snapshot, snapshot_bank, track_of,
};

fn track_values() -> [Values; 8] {
    core::array::from_fn(|track| {
        let mut values = active_values();
        set_parameter(&mut values, 0, -24.0 - track as f32, -26.0 - track as f32);
        set_parameter(&mut values, 1, 2.0 + track as f32, 3.0 + track as f32);
        set_parameter(&mut values, 2, 24.0 + track as f32, 30.0 + track as f32);
        set_parameter(&mut values, 3, 1.0 + track as f32, 2.0 + track as f32);
        set_parameter(
            &mut values,
            4,
            0.5 + track as f32 * 0.25,
            0.6 + track as f32 * 0.25,
        );
        set_parameter(&mut values, 5, track as f32, 1.0 + track as f32);
        set_parameter(&mut values, 6, 10.0 + track as f32, 12.0 + track as f32);
        values
    })
}

fn render_bank8(
    bank: &mut dyn PreparedNativeEffectBank,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
    automation: &[effect_contract::PreparedAutomationSpan],
) {
    let frames = left.len() / 8;
    let mut start = 0;
    while start < frames {
        let end = (start + block).min(frames);
        let mut spans = Vec::new();
        let mut offsets = [0_u32; 9];
        for track in 0..8 {
            if start == 0 {
                spans.extend_from_slice(automation);
            }
            offsets[track + 1] = spans.len() as u32;
        }
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[start * 8..end * 8],
                &mut right[start * 8..end * 8],
                None,
                (end - start) as u32,
                BankWidth::Eight,
                start as u64,
                &spans,
                &offsets,
                512,
            )
            .expect("bank block"),
        );
        start = end;
    }
}

#[test]
fn scalar_and_w8_are_bit_exact_for_all_link_modes_and_ramps() {
    let values = track_values();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let Some(mut bank) = prepare_bank_w8(&values, link) else {
            return;
        };
        let mut scalar = Vec::new();
        for values in &values {
            let mut request = request(values);
            request.link_mode = link;
            scalar.push(prepare(request));
        }
        let frames = 257;
        let left_source: Vec<Vec<f32>> = (0..8)
            .map(|track| noise(11 + track as u64, frames, 0.3))
            .collect();
        let right_source: Vec<Vec<f32>> = (0..8)
            .map(|track| noise(91 + track as u64, frames, 0.3))
            .collect();
        let mut bank_left = packed_w8(&left_source);
        let mut bank_right = packed_w8(&right_source);
        render_bank8(
            &mut *bank,
            &mut bank_left,
            &mut bank_right,
            17,
            &retarget_spans(0),
        );
        for track in 0..8 {
            let mut left = left_source[track].clone();
            let mut right = right_source[track].clone();
            let spans = retarget_spans(0);
            render_scalar_sidechain(
                scalar[track].as_mut(),
                &mut left,
                &mut right,
                None,
                17,
                &spans,
                0,
            );
            assert_bits_eq(
                &track_of(&bank_left, track, 8),
                &left,
                &format!("{link:?} left {track}"),
            );
            assert_bits_eq(
                &track_of(&bank_right, track, 8),
                &right,
                &format!("{link:?} right {track}"),
            );
            assert_eq!(
                snapshot_bank(&*bank, track as u32),
                snapshot(scalar[track].as_ref())
            );
        }
    }
}

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
        one.as_mut(),
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
                split.as_mut(),
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
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 3, &[], 0);
    assert_eq!(left[0].to_bits(), 0x8000_0000);
    assert_eq!(right[2].to_bits(), 0x8000_0000);
    let (_, payload, _) = snapshot(effect.as_ref());
    assert_ne!(u32::from_le_bytes(payload[0..4].try_into().unwrap()), 0);
}

#[test]
fn equal_input_is_dual_mono_and_zero_input_has_no_tail() {
    let mut effect = prepare(request(&active_values()));
    let mut equal_left = noise(701, 257, 0.3);
    let mut equal_right = equal_left.clone();
    render_scalar_sidechain(
        effect.as_mut(),
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
        effect.as_mut(),
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

#[test]
fn w4_binding_is_internal_lane_evidence_without_factory_width_claim() {
    if Backend::current() != Backend::Simd4 {
        return;
    }
    let values = track_values();
    let Some(mut bank) = prepare_bank(
        &values,
        LinkMode::DualMono,
        BankWidth::Four,
        Backend::Simd4,
        128,
    ) else {
        return;
    };
    let mut left = vec![0.0_f32; 8 * 4];
    for frame in 0..8 {
        for track in 0..4 {
            left[frame * 4 + track] = noise(3 + track as u64, 8, 0.2)[frame];
        }
    }
    let mut right = left.clone();
    let offsets = [0_u32; 5];
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            8,
            BankWidth::Four,
            0,
            &[],
            &offsets,
            128,
        )
        .unwrap(),
    );
}
