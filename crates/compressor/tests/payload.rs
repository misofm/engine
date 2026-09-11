//! E7 — the payload round trips, restores transactionally, and is bit-exact at rest.
//!
//! The word layout is frozen (master plan section 8.2). What changed under #88 is the *codec*:
//! the crate's four byte helpers are gone and `effect_runtime::state_payload` writes
//! and reads the words, with an exact-length check (`!=`, never `<`) so a payload with trailing
//! bytes is rejected instead of silently truncated.

mod support;

use effect_contract::{
    AutomationSpanKind, ParameterChannel, PreparedAutomationSpan, ResetKind, StatePayloadInput,
};
use effect_runtime::state_payload::{read_f32, read_u32, write_f32, write_u32};

use support::{
    bind_bank, initial_values, native_bank_width, noise, prepare, render_scalar, request, restore,
    restore_track, snapshot, snapshot_track, values_with,
};

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

/// A snapshot round trips, and a restore at rest is bit-exact against an uninterrupted render.
///
/// "At rest" means `remaining == 0` on every ramp, which is every sample outside the 64-sample
/// window after an automation event — well over 99 % of a session.
///
/// Red mutation (MUTATIONS.md row 14): commit the left channel before validating the right — RED
/// on the transactional test below.
#[test]
fn an_idle_restore_is_bit_exact() {
    let values = values_with(&[(0, -30.0), (1, 6.0), (2, 6.0)]);
    let input_left = noise(4_096, 0x9A_10_00_01, 0.8);
    let input_right = noise(4_096, 0x9A_10_00_02, 0.8);

    let mut uninterrupted = prepare(request(&values));
    let mut left = input_left.clone();
    let mut right = input_right.clone();
    render_scalar(uninterrupted.as_mut(), &mut left, &mut right, 128, 128, &[]);

    let mut restored = prepare(request(&values));
    let mut first_left = input_left[..2_048].to_vec();
    let mut first_right = input_right[..2_048].to_vec();
    render_scalar(
        restored.as_mut(),
        &mut first_left,
        &mut first_right,
        128,
        128,
        &[],
    );
    let (saved_left, saved_right) = snapshot(restored.as_ref());

    // A fresh instance takes the payload and continues the render.
    let mut continued = prepare(request(&values));
    restore(continued.as_mut(), 1, &saved_left, &saved_right).expect("restore");
    let mut second_left = input_left[2_048..].to_vec();
    let mut second_right = input_right[2_048..].to_vec();
    render_scalar(
        continued.as_mut(),
        &mut second_left,
        &mut second_right,
        128,
        128,
        &[],
    );

    assert_eq!(
        second_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        left[2_048..]
            .iter()
            .map(|s| s.to_bits())
            .collect::<Vec<_>>(),
        "an idle restore must continue the render bit for bit"
    );
    assert_eq!(
        second_right.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        right[2_048..]
            .iter()
            .map(|s| s.to_bits())
            .collect::<Vec<_>>()
    );
    // And the render had content.
    assert!(second_left.iter().any(|sample| *sample != 0.0));
}

/// One corrupt word in the right channel rejects the whole restore and leaves both channels
/// exactly as they were.
#[test]
fn a_restore_is_transactional_across_both_channels() {
    let values = initial_values();
    let mut effect = prepare(request(&values));
    let mut left = vec![0.5_f32; 128];
    let mut right = vec![-0.25_f32; 128];
    render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
    let (saved_left, saved_right) = snapshot(effect.as_ref());

    // The left section handed to `restore` must differ from the live state, or a commit that
    // happened before the right section was validated would be invisible.
    let mut modified_left = saved_left.clone();
    write_f32(&mut modified_left, 1, -17.5);
    assert_ne!(modified_left, saved_left);

    for (word, value, code) in [
        (0_usize, 1.0_f32.to_bits(), "effect.state.gain"),
        (1, f32::NAN.to_bits(), "effect.state.parameter"),
        (2, 1.0_f32.to_bits(), "effect.state.parameter"),
        (3, 65_u32, "effect.state.parameter"),
    ] {
        let mut corrupt = saved_right.clone();
        write_u32(&mut corrupt, word, value);
        let error = restore(effect.as_mut(), 1, &modified_left, &corrupt)
            .err()
            .unwrap_or_else(|| panic!("word {word} must be rejected"));
        assert_eq!(error.code, code, "word {word}");
        let (after_left, after_right) = snapshot(effect.as_ref());
        assert_eq!(after_left, saved_left, "left must not move (word {word})");
        assert_eq!(
            after_right, saved_right,
            "right must not move (word {word})"
        );
    }

    // A wrong section length is rejected before anything is read.
    let sizes = effect.metadata().state_sizes;
    let mut short = saved_right.clone();
    short.truncate(short.len() - 4);
    assert!(
        StatePayloadInput::new(&[], &saved_left, &short, sizes).is_err(),
        "the contract's own constructor rejects a short section"
    );

    // A version other than the descriptor's is rejected.
    assert_eq!(
        restore(effect.as_mut(), 2, &saved_left, &saved_right)
            .expect_err("version")
            .code,
        "effect.state.version"
    );
}

/// Raw payload hooks validate their public input sections themselves. A caller that bypasses the
/// convenience constructor still gets exact-length rejection, and the scalar and bank hooks both
/// validate the final right-channel word before committing a changed left section.
#[test]
fn malformed_raw_payloads_reject_transactionally_in_scalar_and_bank_hooks() {
    const OLD_CHANNEL_BYTES: usize = 7_784;
    let values = initial_values();

    let mut scalar = prepare(request(&values));
    let scalar_saved = snapshot(scalar.as_ref());
    let mut scalar_left = scalar_saved.0.clone();
    write_f32(&mut scalar_left, 1, -17.5);
    let mut scalar_short = scalar_saved.1.clone();
    scalar_short.pop();
    assert_eq!(
        restore(scalar.as_mut(), 1, &scalar_left, &scalar_short)
            .expect_err("short raw scalar payload must reach validation")
            .code,
        "effect.state.length"
    );
    assert_eq!(snapshot(scalar.as_ref()), scalar_saved);

    let old_raw = vec![0_u8; OLD_CHANNEL_BYTES];
    assert_eq!(
        restore(scalar.as_mut(), 1, &old_raw, &old_raw)
            .expect_err("old raw scalar payload must be rejected")
            .code,
        "effect.state.length"
    );
    assert_eq!(snapshot(scalar.as_ref()), scalar_saved);

    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let requests: Vec<_> = (0..lanes).map(|_| request(&values)).collect();
    let mut bank = bind_bank(&requests).expect("bank");
    let bank_scalar = prepare(request(&values));
    let bank_saved = snapshot_track(bank.as_ref(), 0, bank_scalar.as_ref());

    let mut bank_left = bank_saved.0.clone();
    write_f32(&mut bank_left, 1, -17.5);
    let mut corrupt_right = bank_saved.1.clone();
    write_u32(&mut corrupt_right, 21, 65);
    assert_eq!(
        restore_track(
            bank.as_mut(),
            0,
            1,
            &bank_left,
            &corrupt_right,
            bank_scalar.as_ref(),
        )
        .expect_err("final right-channel word must be validated")
        .code,
        "effect.state.parameter"
    );
    assert_eq!(
        snapshot_track(bank.as_ref(), 0, bank_scalar.as_ref()),
        bank_saved,
        "bank restore must be transactional across both channels"
    );

    let mut bank_short = bank_saved.1.clone();
    bank_short.pop();
    assert_eq!(
        restore_track(
            bank.as_mut(),
            0,
            1,
            &bank_saved.0,
            &bank_short,
            bank_scalar.as_ref(),
        )
        .expect_err("short raw bank payload must reach validation")
        .code,
        "effect.state.length"
    );
    assert_eq!(
        restore_track(
            bank.as_mut(),
            0,
            1,
            &old_raw,
            &old_raw,
            bank_scalar.as_ref(),
        )
        .expect_err("old raw bank payload must be rejected")
        .code,
        "effect.state.length"
    );
    assert_eq!(
        snapshot_track(bank.as_ref(), 0, bank_scalar.as_ref()),
        bank_saved
    );
}

/// A mid-ramp restore lands on the target on the same sample, and tracks within 8 ulp afterwards.
///
/// `step` is not serialised — the payload layout is a frozen contract fixture — so a restore
/// re-derives it as `(target - current) / remaining`. That is **class B**: the pre-audit law
/// recomputed exactly that quotient every sample and D11's law computes it once at the event, so a
/// ramp resumed at `remaining = 37` follows a slightly different path to the same place. The two
/// things that must still hold are asserted here: the arrival sample and the arrival value.
///
/// Red mutation (MUTATIONS.md row 13): `step = (target - current) / 64.0` on restore, ignoring
/// `remaining`. The *arrival sample* is driven by `remaining` and survives it, so what this test
/// pins is the value the ramp holds on the way there.
#[test]
fn a_mid_ramp_restore_arrives_on_the_same_sample() {
    let values = initial_values();
    let mut effect = prepare(request(&values));
    let mut left = vec![0.0_f32; 27];
    let mut right = vec![0.0_f32; 27];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        27,
        128,
        &[(0, point(0, ParameterChannel::Left, -80.0))],
    );
    let (saved_left, saved_right) = snapshot(effect.as_ref());
    assert_eq!(read_u32(&saved_left, 3), 37, "37 samples still to produce");

    let mut restored = prepare(request(&values));
    restore(restored.as_mut(), 1, &saved_left, &saved_right).expect("restore");

    // 36 more samples: still short of the target, and on the re-derived step exactly.
    let resumed_from = read_f32(&saved_left, 1);
    let resumed_step = ((-80.0_f32) - resumed_from) / 37.0;
    let mut expected = resumed_from;
    for _ in 0..36 {
        expected += resumed_step;
    }
    let mut left = vec![0.0_f32; 36];
    let mut right = vec![0.0_f32; 36];
    render_scalar(restored.as_mut(), &mut left, &mut right, 36, 128, &[]);
    let (state, _) = snapshot(restored.as_ref());
    assert_eq!(read_u32(&state, 3), 1);
    assert_ne!(read_f32(&state, 1).to_bits(), (-80.0_f32).to_bits());
    assert_eq!(
        read_f32(&state, 1).to_bits(),
        expected.to_bits(),
        "the step is re-derived from the remaining distance and the remaining count"
    );

    // The 37th lands exactly on the target.
    let mut left = vec![0.0_f32; 1];
    let mut right = vec![0.0_f32; 1];
    render_scalar(restored.as_mut(), &mut left, &mut right, 1, 128, &[]);
    let (state, _) = snapshot(restored.as_ref());
    assert_eq!(read_u32(&state, 3), 0);
    assert_eq!(read_f32(&state, 1).to_bits(), (-80.0_f32).to_bits());
}

/// Every preparation-legal parameter value survives a round trip, including a subnormal.
#[test]
fn preparation_legal_parameter_states_round_trip() {
    let values = initial_values();
    let mut effect = prepare(request(&values));
    let (mut left, right) = snapshot(effect.as_ref());
    let subnormal = f32::from_bits(1);
    // Makeup and mix admit positive subnormals in their continuous domains.
    for word in [16_usize, 19, 20] {
        write_f32(&mut left, word, subnormal);
    }
    restore(effect.as_mut(), 1, &left, &right).expect("a legal subnormal restores");
    let (restored_left, _) = snapshot(effect.as_ref());
    for word in [16_usize, 19, 20] {
        assert_eq!(
            read_f32(&restored_left, word).to_bits(),
            subnormal.to_bits()
        );
    }
}

/// Both resets clear the state; only the full one restores the preparation values.
#[test]
fn resets_clear_state_and_only_the_full_one_restores_defaults() {
    let values = initial_values();
    let mut effect = prepare(request(&values));
    let mut left = vec![0.4_f32; 1_536];
    let mut right = vec![-0.4_f32; 1_536];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        128,
        128,
        &[(0, point(0, ParameterChannel::Left, -70.0))],
    );

    effect.reset(ResetKind::DiscontinuityKeepParameters);
    let (state, _) = snapshot(effect.as_ref());
    assert_eq!(
        read_f32(&state, 0).to_bits(),
        0.0_f32.to_bits(),
        "G cleared"
    );
    assert_eq!(read_u32(&state, 3), 0, "ramps snapped");
    assert_eq!(read_f32(&state, 1).to_bits(), (-70.0_f32).to_bits(), "kept");

    effect.reset(ResetKind::FullToDefaults);
    let (state, _) = snapshot(effect.as_ref());
    assert_eq!(read_f32(&state, 1).to_bits(), (-18.0_f32).to_bits());
}
