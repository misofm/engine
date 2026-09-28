//! Issue #994 — a knee so narrow that `1 / (2 W)` overflows is the documented hard knee.
//!
//! Before #994 a knee of 2.8e-45 dB (`0x0000_0002`) passed parameter validation — finite and in
//! `[0, 24]` — the shared design `effect_runtime::dynamics::GainComputerCoef::new` computed
//! `inv_two_knee = +inf`, and a detector level exactly at the threshold computed
//! `(v * v = 0) * inf = NaN`. The kernel's reduction clamp `max(-100)` (`kernel.rs`, untouched)
//! then turned that NaN into a -100 dB target, so a full-scale sample at a 0 dB threshold was
//! ducked toward -100 dB. `effect_runtime::dynamics::knee_coefficients` now designs every width
//! whose reciprocal overflows — exactly `(0, 2^-129]` — as the hard knee.
//!
//! The fix is in the one design every entry point reaches, so each test drives a different entry:
//! preparation (a session's initial value), automation (a control message whose `Linear 64` ramp
//! crosses the overflow band between two endpoints that a parameter minimum at the derived bound
//! would admit — which is why rejecting widths below the bound could not have closed this), a
//! state-payload restore, and a homogeneous bank.
//!
//! Every test compares against the knee-0 hard knee rendered through the same path, bit for bit,
//! and against the independent `f64` reference compressor at the same pathological knee.

mod support;

use dsp_reference::{ReferenceCompressorParameters, ReferencePeakCompressor};
use effect_contract::{
    AutomationSpanKind, InitialParameterValue, ParameterChannel, PreparedAutomationSpan,
};

use support::{
    PARAMETER_COUNT, bind_bank, native_bank_width, prepare, render_bank, render_scalar, request,
    restore, snapshot, values_with,
};

/// Positive widths whose `f32` reciprocal `1 / (2 W)` overflows: the smallest subnormal, the
/// verification's 2.8e-45, the width just under the bound and `2^-129`, the widest one.
const OVERFLOWING_KNEES: [f32; 4] = [
    f32::from_bits(0x0000_0001),
    f32::from_bits(0x0000_0002),
    f32::from_bits(0x000F_FFFF),
    f32::from_bits(0x0010_0000),
];

/// The narrowest soft knees: `2^-129 + 2^-149` (the bound), the next width, and `1e-38`.
const NARROWEST_SOFT_KNEES: [f32; 3] = [
    f32::from_bits(0x0010_0001),
    f32::from_bits(0x0010_0002),
    1.0e-38,
];

/// Frames rendered per case: four 128-frame blocks.
const FRAMES: usize = 512;

/// Threshold 0 dB, the given ratio and knee, the fastest attack, no makeup, fully wet.
///
/// A full-scale input sample has a detector level of exactly `+0.0` dB, so every frame sits
/// exactly on the threshold, which is where the overflowing knee computed its NaN.
fn values(knee: f32, ratio: f32) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    values_with(&[
        (0, 0.0),
        (1, ratio),
        (2, knee),
        (3, 0.1),
        (4, 5.0),
        (5, 0.0),
        (6, 1.0),
    ])
}

/// Renders `FRAMES` full-scale frames through a scalar instance and returns the left plane.
fn render_full_scale(
    values: &[InitialParameterValue],
    spans: &[(u64, PreparedAutomationSpan)],
) -> Vec<f32> {
    let mut effect = prepare(request(values));
    let mut left = vec![1.0_f32; FRAMES];
    let mut right = vec![1.0_f32; FRAMES];
    let report = render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, spans);
    assert_eq!(report.nonfinite_left_blocks, 0);
    assert_eq!(report.nonfinite_right_blocks, 0);
    assert_eq!(
        bits(&left),
        bits(&right),
        "both channels carry the same input"
    );
    left
}

fn bits(samples: &[f32]) -> Vec<u32> {
    samples.iter().map(|sample| sample.to_bits()).collect()
}

/// The independent `f64` reference at the same parameters, over the same full-scale input.
fn reference(knee: f32, ratio: f32) -> Vec<f64> {
    let mut lane = ReferencePeakCompressor::new(
        48_000.0,
        ReferenceCompressorParameters {
            threshold_db: 0.0,
            ratio: f64::from(ratio),
            knee_db: f64::from(knee),
            attack_ms: f64::from(0.1_f32),
            release_ms: 5.0,
            makeup_db: 0.0,
            mix: 1.0,
        },
    )
    .expect("reference parameters are in the launch domain");
    (0..FRAMES).map(|_| lane.process_sample(1.0, 1.0)).collect()
}

/// Asserts that `output` is the hard knee's rendering bit for bit, and within `1e-6` of the `f64`
/// reference at `knee` — which, being `f64`, designs the pathological knee as a genuine soft knee
/// and finds a reduction of `|1/R - 1| W / 8`, about `1e-46` dB: no audible or representable duck.
fn assert_undocked(output: &[f32], hard: &[f32], knee: f32, ratio: f32, entry: &str) {
    let expected = reference(knee, ratio);
    for (frame, (sample, oracle)) in output.iter().zip(expected.iter()).enumerate() {
        assert!(
            (f64::from(*sample) - oracle).abs() <= 1.0e-6,
            "{entry}: W {knee:e} ({:#010x}) R {ratio} frame {frame}: {sample} vs f64 reference \
             {oracle}",
            knee.to_bits()
        );
    }
    assert_eq!(
        bits(output),
        bits(hard),
        "{entry}: W {knee:e} ({:#010x}) R {ratio} must render as the hard knee",
        knee.to_bits()
    );
}

/// A session that prepares an overflowing knee renders a full-scale sample at a 0 dB threshold
/// exactly as the hard knee does — no duck — and within `1e-6` of the `f64` reference.
///
/// Before #994 each of these knees ducked the output toward -100 dB (the verification measured
/// 0.8267 after eight frames at 2.8e-45).
///
/// Red mutation (MUTATIONS.md row 994-C1): drop the `is_finite` test in
/// `effect_runtime::dynamics::knee_coefficients`.
#[test]
fn a_prepared_overflowing_knee_does_not_duck_a_sample_at_the_threshold() {
    for ratio in [4.0_f32, 20.0] {
        let hard = render_full_scale(&values(0.0, ratio), &[]);
        for knee in OVERFLOWING_KNEES {
            let output = render_full_scale(&values(knee, ratio), &[]);
            assert_undocked(&output, &hard, knee, ratio, "prepare");
        }
    }
}

/// The narrowest soft knees keep their soft design and are just as undisturbed at the threshold:
/// their reduction there is `|1/R - 1| W / 8`, which rounds to zero.
#[test]
fn the_narrowest_soft_knees_do_not_duck_a_sample_at_the_threshold() {
    for ratio in [4.0_f32, 20.0] {
        let hard = render_full_scale(&values(0.0, ratio), &[]);
        for knee in NARROWEST_SOFT_KNEES {
            let output = render_full_scale(&values(knee, ratio), &[]);
            assert_undocked(&output, &hard, knee, ratio, "prepare");
        }
    }
}

fn knee_point(channel: ParameterChannel, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: 2,
        start_sample: 0,
        end_sample: 0,
        start_value: value,
        end_value: value,
    }
}

/// Automating the knee from the hard knee (0) to `1e-38` and back — two admissible endpoints,
/// both outside the overflow band — renders exactly as the hard knee.
///
/// The `Linear 64` ramp steps by `1e-38 / 64`, so its first nine values on the way up and its last
/// nine on the way down lie inside `(0, 2^-129]`, and the compressor redesigns the curve from every
/// one of them. Before #994 those samples took a NaN target and were ducked. This is why the fix
/// is in the design rather than in parameter validation: a parameter minimum at the derived bound,
/// `2^-129 + 2^-149`, admits both endpoints, and so cannot stop this ramp from passing through the
/// band.
///
/// Red mutation (MUTATIONS.md row 994-C1): drop the `is_finite` test in `knee_coefficients`.
#[test]
fn an_automated_knee_ramp_through_the_overflow_band_does_not_duck() {
    let widest_overflowing = f32::from_bits(0x0010_0000);
    assert!(
        1.0e-38_f32 / 64.0 < widest_overflowing,
        "premise: the ramp's first step lies inside the overflow band"
    );
    for ratio in [4.0_f32, 20.0] {
        let hard = render_full_scale(&values(0.0, ratio), &[]);
        let spans = [
            (0, knee_point(ParameterChannel::Left, 1.0e-38)),
            (0, knee_point(ParameterChannel::Right, 1.0e-38)),
            (256, knee_point(ParameterChannel::Left, 0.0)),
            (256, knee_point(ParameterChannel::Right, 0.0)),
        ];
        let output = render_full_scale(&values(0.0, ratio), &spans);
        assert_eq!(
            bits(&output),
            bits(&hard),
            "R {ratio}: a knee ramp through the overflow band must render as the hard knee"
        );
    }
}

/// Restoring a state payload whose knee is 2.8e-45 into an instance prepared at the default 6 dB
/// knee renders exactly as the hard knee.
///
/// Red mutation (MUTATIONS.md row 994-C1): drop the `is_finite` test in `knee_coefficients`.
#[test]
fn a_restored_overflowing_knee_does_not_duck() {
    let knee = f32::from_bits(0x0000_0002);
    for ratio in [4.0_f32, 20.0] {
        let hard = render_full_scale(&values(0.0, ratio), &[]);
        let donor = prepare(request(&values(knee, ratio)));
        let (left_payload, right_payload) = snapshot(donor.as_ref());
        let receiver_values = values(6.0, ratio);
        let mut receiver = prepare(request(&receiver_values));
        restore(receiver.as_mut(), 1, &left_payload, &right_payload)
            .expect("a valid payload restores");
        let mut left = vec![1.0_f32; FRAMES];
        let mut right = vec![1.0_f32; FRAMES];
        let report = render_scalar(receiver.as_mut(), &mut left, &mut right, 128, 128, &[]);
        assert_eq!(
            report.nonfinite_left_blocks + report.nonfinite_right_blocks,
            0
        );
        assert_undocked(&left, &hard, knee, ratio, "restore");
        assert_undocked(&right, &hard, knee, ratio, "restore");
    }
}

/// A homogeneous bank at this build's SIMD width, one knee per lane — every overflowing width,
/// the hard knee and the narrowest soft knees side by side — renders every lane exactly as the
/// scalar hard knee.
///
/// Red mutation (MUTATIONS.md row 994-C1): drop the `is_finite` test in `knee_coefficients`.
#[test]
fn a_bank_of_overflowing_knees_does_not_duck() {
    let Some((_, width)) = native_bank_width() else {
        return;
    };
    let lanes = width.lanes() as usize;
    let candidates: Vec<f32> = OVERFLOWING_KNEES
        .iter()
        .chain([0.0_f32].iter())
        .chain(NARROWEST_SOFT_KNEES.iter())
        .copied()
        .collect();
    for ratio in [4.0_f32, 20.0] {
        let hard = render_full_scale(&values(0.0, ratio), &[]);
        let knees: Vec<f32> = (0..lanes)
            .map(|lane| candidates[lane % candidates.len()])
            .collect();
        let lane_values: Vec<_> = knees.iter().map(|knee| values(*knee, ratio)).collect();
        let requests: Vec<_> = lane_values.iter().map(|values| request(values)).collect();
        let mut bank = bind_bank(&requests).expect("this build has a bank width");
        let mut left = vec![1.0_f32; FRAMES * lanes];
        let mut right = vec![1.0_f32; FRAMES * lanes];
        render_bank(
            bank.as_mut(),
            &mut left,
            &mut right,
            lanes,
            width,
            128,
            128,
            &[],
        );
        for (lane, knee) in knees.iter().enumerate() {
            for plane in [&left, &right] {
                let output: Vec<f32> = (0..FRAMES)
                    .map(|frame| plane[frame * lanes + lane])
                    .collect();
                assert_undocked(&output, &hard, *knee, ratio, "bank");
            }
        }
    }
}
