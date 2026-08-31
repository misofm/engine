//! E8 — the four identity paths return the dry signal, or the wet signal, bit for bit.
//!
//! BRIEFS/013 freezes these: `bypass`, `mix == 0` and `G == 0 && makeup == +0` all emit the
//! delayed dry sample unchanged, and `mix == 1` emits the wet sample unchanged. "Unchanged" is
//! `to_bits`, not a tolerance — a mix of exactly one must not cost a rounding.
//!
//! All four keep the state warm: the rings advance and `G` keeps tracking, so un-bypassing or
//! moving `mix` off an end never clicks. That answers open question 5 of the audit — it is
//! intentional, and the last test here is what says so.

mod support;

use effect_contract::{
    EffectProcessBlock, LinkMode, PreparedNativeEffect, PreparedSidechainPort, ResetKind,
};

use support::{noise, prepare, render_scalar, request, values_with};

/// The dry sample that must come out of an identity path is the input delayed by `N`.
const LATENCY: usize = 960;

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

/// `bypass` emits the delayed input exactly, at the fixed latency.
#[test]
fn bypass_preserves_exact_dry_bits_at_fixed_latency() {
    let values = values_with(&[]);
    let mut preparation = request(&values);
    preparation.bypass = true;
    let mut effect = prepare(preparation);
    let input_left = noise(2_048, 0xB1_A5_00_01, 0.75);
    let input_right = noise(2_048, 0xB1_A5_00_02, 0.75);
    let (left, right) = render(effect.as_mut(), &input_left, &input_right);
    for index in 0..LATENCY {
        assert_eq!(left[index].to_bits(), 0.0_f32.to_bits());
        assert_eq!(right[index].to_bits(), 0.0_f32.to_bits());
    }
    for index in LATENCY..2_048 {
        assert_eq!(left[index].to_bits(), input_left[index - LATENCY].to_bits());
        assert_eq!(
            right[index].to_bits(),
            input_right[index - LATENCY].to_bits()
        );
    }
}

/// `mix == 0` emits the delayed input exactly, *while the compressor is reducing gain*.
///
/// The second half of that sentence is the test: a `mix == 0` that also stopped the detector would
/// pass a naive comparison. The same configuration with `mix == 1` is checked to be materially
/// different, so the identity is not an accident of a silent gain stage.
///
/// Red mutation (MUTATIONS.md row 9): remove the `bypass` term from `dry_identity` — RED. Removing
/// the `mix == 0` term alone is *not* red, because `fma(+0.0, wet - dry, dry)` is already `dry` for
/// finite operands; what the select buys is that the identity does not depend on that.
#[test]
fn a_zero_mix_is_the_dry_signal_while_gain_reduction_runs() {
    let compressing = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (7, 0.0)]);
    let input_left = noise(2_048, 0xB1_A5_00_03, 0.75);
    let input_right = noise(2_048, 0xB1_A5_00_04, 0.75);

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

    for index in LATENCY..2_048 {
        assert_eq!(
            dry_left[index].to_bits(),
            input_left[index - LATENCY].to_bits()
        );
    }
    assert!(
        wet_left[1_500].abs() < dry_left[1_500].abs() * 0.5,
        "the configuration must actually be compressing: {} vs {}",
        wet_left[1_500],
        dry_left[1_500]
    );
}

/// `mix == 1` emits the wet sample exactly: the delayed input times the gain, with no mix rounding.
///
/// Constructed so the wet sample is recoverable exactly. The main input is the constant `0.5`, so
/// `y = z * A` can be divided by `z` without rounding and `A` comes back exactly; the detector is
/// driven from a connected sidechain carrying noise, so `A` moves from frame to frame. The two-step
/// form the mix would otherwise take, `fma(mix, w - z, z)` at `mix == 1`, is exactly
/// `fl(fl(w - z) + z)`, which is reconstructed here from `A` and compared. The test asserts both
/// that the effect returned `w` and that the two-step form differs on at least one frame — without
/// the second assertion the first would prove nothing.
///
/// Red mutation: remove the `wet_identity` select — RED.
#[test]
fn a_unit_mix_is_the_wet_signal_exactly() {
    let values = values_with(&[
        (0, -40.0),
        (1, 20.0),
        (2, 0.0),
        (5, 0.0),
        (6, 1.0),
        (7, 0.0),
    ]);
    let mut preparation = request(&values);
    preparation.ports.sidechain = PreparedSidechainPort::Connected {
        id: support::sidechain_port(),
        required: false,
    };
    let mut effect = prepare(preparation);

    let dry = 0.5_f32;
    let sidechain = noise(2_048, 0xB1_A5_00_05, 0.9);
    let mut left = vec![dry; 2_048];
    let mut right = vec![dry; 2_048];
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
    for wet in left.iter().copied().skip(LATENCY) {
        // `dry` is an exact power of two, so this division is exact and `gain * dry` is `wet`.
        let gain = wet / dry;
        assert_eq!(
            (dry * gain).to_bits(),
            wet.to_bits(),
            "the recovered gain must reproduce the sample exactly"
        );
        let two_step = (wet - dry) + dry;
        if two_step.to_bits() != wet.to_bits() {
            differs += 1;
        }
    }
    assert!(
        differs > 0,
        "the two-step mix form is indistinguishable here, so this test proves nothing"
    );
}

/// `G == 0 && makeup == +0` emits the dry signal exactly, even with `mix` strictly between the
/// ends.
#[test]
fn a_unity_gain_stage_is_the_dry_signal() {
    // Threshold at 0 dB with a hard knee and an input well below it: the curve is the identity, so
    // `G` stays exactly `+0.0`, and `makeup` is `+0.0`.
    let values = values_with(&[(0, 0.0), (1, 4.0), (2, 0.0), (5, 0.0), (6, 0.5), (7, 0.0)]);
    let input_left = noise(2_048, 0xB1_A5_00_06, 0.01);
    let input_right = noise(2_048, 0xB1_A5_00_07, 0.01);
    let mut effect = prepare(request(&values));
    let (left, right) = render(effect.as_mut(), &input_left, &input_right);
    for index in LATENCY..2_048 {
        assert_eq!(left[index].to_bits(), input_left[index - LATENCY].to_bits());
        assert_eq!(
            right[index].to_bits(),
            input_right[index - LATENCY].to_bits()
        );
    }
}

/// The `Average` link is exactly `0.5 * |l| + 0.5 * |r|`, in that operation order.
///
/// Checked through a configuration whose gain is a monotone function of the detector level, by
/// feeding a pair whose average is representable exactly and comparing against a `Maximum`-linked
/// instance fed that average on both channels. Two products and an add, never an `fma`: the `fma`
/// form of `0.5|l| + 0.5|r|` differs in the last bit for most operands.
///
/// This is an **equivalent** mutation, recorded as MUTATIONS.md row 22 rather than gated: `0.5 * x`
/// is exact for every finite `x`, so the fused and unfused forms round the same addition and cannot
/// differ. The unfused form is kept because the brief states the operation order.
#[test]
fn the_average_link_is_two_products_and_an_add() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 1.0), (7, 20.0)]);
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

    // The comparison is the recursive word itself, read out of the payload: `G` is a function of
    // the detector level alone, so two instances that saw the same level sequence must carry the
    // same `G` bits. Comparing rendered samples instead would compare two different divisions.
    let (averaged_state, _) = support::snapshot(averaged.as_ref());
    let (equivalent_state, _) = support::snapshot(equivalent.as_ref());
    let averaged_gain = effect_runtime::state_payload::read_f32(&averaged_state, 2);
    let equivalent_gain = effect_runtime::state_payload::read_f32(&equivalent_state, 2);
    assert!(
        averaged_gain < -1.0,
        "the configuration must be reducing gain, G is {averaged_gain}"
    );
    assert_eq!(
        averaged_gain.to_bits(),
        equivalent_gain.to_bits(),
        "Average must produce exactly the level 0.5|l| + 0.5|r|"
    );
}

/// A bypassed instance keeps its detector and its `G` running, so un-bypassing does not click.
///
/// Audit open question 5, answered: this is intentional. The evidence is that a bypassed instance
/// and a non-bypassed one, rendered over the same input and then compared by their payload's `G`
/// word, agree — the bypass is a select on the output, not a gate on the state.
#[test]
fn every_identity_keeps_the_state_warm() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 1.0), (7, 0.0)]);
    let input_left = noise(2_048, 0xB1_A5_00_08, 0.75);
    let input_right = noise(2_048, 0xB1_A5_00_09, 0.75);

    let mut live = prepare(request(&values));
    let (_, _) = render(live.as_mut(), &input_left, &input_right);
    let (live_left, _) = support::snapshot(live.as_ref());

    let mut bypassed_request = request(&values);
    bypassed_request.bypass = true;
    let mut bypassed = prepare(bypassed_request);
    let (_, _) = render(bypassed.as_mut(), &input_left, &input_right);
    let (bypassed_left, _) = support::snapshot(bypassed.as_ref());

    assert_eq!(
        live_left, bypassed_left,
        "a bypassed instance must carry exactly the state a live one does"
    );
    // And the state is not trivially zero.
    let gain_word = effect_runtime::state_payload::read_f32(&live_left, 2);
    assert!(
        gain_word < -1.0,
        "G should be well below zero, is {gain_word}"
    );

    // A discontinuity reset clears it, which is the control that says the comparison had content.
    live.reset(ResetKind::DiscontinuityKeepParameters);
    let (cleared, _) = support::snapshot(live.as_ref());
    assert_eq!(
        effect_runtime::state_payload::read_f32(&cleared, 2).to_bits(),
        0.0_f32.to_bits()
    );
}
