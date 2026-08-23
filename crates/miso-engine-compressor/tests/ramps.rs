//! E6 — D11: one division at the event, iterated additions, an exact snap on the last sample.
//!
//! The ramp state is visible through the payload: word `3 + 3i` is parameter `i`'s `current`,
//! `4 + 3i` its `target` and `5 + 3i` its `remaining`. `step` is deliberately **not** serialised —
//! the layout is a frozen contract fixture — which is what makes a mid-ramp restore class B
//! (`tests/payload.rs`).

mod support;

use miso_engine_effect_contract::{
    AutomationSpanKind, ParameterChannel, PreparedAutomationSpan, StatePayloadOutput,
};
use miso_engine_effect_runtime::state_payload::{read_f32, read_u32};

use support::{initial_values, prepare, render_scalar, request_with_quantum};

const THRESHOLD_DEFAULT: f32 = -18.0;
const THRESHOLD_TARGET: f32 = -80.0;
const SMOOTHING: u32 = 64;

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

fn state(effect: &dyn miso_engine_effect_contract::PreparedNativeEffect) -> Vec<u8> {
    let sizes = effect.metadata().state_sizes;
    let mut left = vec![0_u8; sizes.left_bytes as usize];
    let mut right = vec![0_u8; sizes.right_bytes as usize];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("payload"),
        )
        .expect("snapshot");
    left
}

/// A block-rate Point steps by a precomputed increment and lands exactly on its target.
///
/// The expected value is computed here with the same `f32` operations the ramp performs — one
/// division at the event, then iterated additions — so this is a bit equality. The pre-audit law,
/// `current += (target - current) / remaining` every sample, produces a *different* sequence, so
/// it is this test that pins which of the two the crate implements.
///
/// Red mutations (MUTATIONS.md rows 8 and 16), both proven: never run the ramping body
/// (`ramping = 0`), and ramp over 63 samples instead of the descriptor's 64.
#[test]
fn block_point_steps_by_a_precomputed_increment_and_snaps_exactly() {
    let step = (THRESHOLD_TARGET - THRESHOLD_DEFAULT) / SMOOTHING as f32;
    for updates in [1_usize, 2, 17, 63] {
        let values = initial_values();
        let mut effect = prepare(request_with_quantum(&values, 128));
        let span = point(0, ParameterChannel::Left, THRESHOLD_TARGET);
        let mut left = vec![0.0_f32; updates];
        let mut right = vec![0.0_f32; updates];
        render_scalar(
            effect.as_mut(),
            &mut left,
            &mut right,
            updates,
            128,
            &[(0, span)],
        );
        let mut expected = THRESHOLD_DEFAULT;
        for _ in 0..updates {
            expected += step;
        }
        let payload = state(effect.as_ref());
        assert_eq!(
            read_f32(&payload, 3).to_bits(),
            expected.to_bits(),
            "after {updates} updates"
        );
        assert_eq!(read_u32(&payload, 5), SMOOTHING - updates as u32);
    }

    // The 64th update assigns the target exactly, whatever the accumulated sum was.
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let span = point(0, ParameterChannel::Left, THRESHOLD_TARGET);
    let mut left = vec![0.0_f32; 64];
    let mut right = vec![0.0_f32; 64];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        64,
        128,
        &[(0, span)],
    );
    let payload = state(effect.as_ref());
    assert_eq!(
        read_f32(&payload, 3).to_bits(),
        THRESHOLD_TARGET.to_bits(),
        "the last update is an assignment, not an addition"
    );
    assert_eq!(read_u32(&payload, 5), 0);

    // And it did not arrive early: 63 additions do not reach the target.
    let mut sum = THRESHOLD_DEFAULT;
    for _ in 0..63 {
        sum += step;
    }
    assert_ne!(sum.to_bits(), THRESHOLD_TARGET.to_bits());
}

/// A Point that arrives while a ramp is in flight restarts from the value reached, not from the
/// original one.
#[test]
fn a_restarting_point_ramps_from_the_current_value() {
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let mut left = vec![0.0_f32; 20];
    let mut right = vec![0.0_f32; 20];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        20,
        128,
        &[(0, point(0, ParameterChannel::Left, THRESHOLD_TARGET))],
    );
    let mid = read_f32(&state(effect.as_ref()), 3);

    let mut left = vec![0.0_f32; 1];
    let mut right = vec![0.0_f32; 1];
    render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        1,
        128,
        &[(0, point(0, ParameterChannel::Left, -30.0))],
    );
    let payload = state(effect.as_ref());
    let restarted_step = (-30.0_f32 - mid) / SMOOTHING as f32;
    assert_eq!(
        read_f32(&payload, 3).to_bits(),
        (mid + restarted_step).to_bits()
    );
    assert_eq!(read_u32(&payload, 4), (-30.0_f32).to_bits());
    assert_eq!(read_u32(&payload, 5), SMOOTHING - 1);
}

/// Automation is per channel and per parameter, and an out-of-order or duplicate span is counted
/// and ignored rather than partly applied.
#[test]
fn automation_validation_is_unchanged() {
    let values = initial_values();
    let mut effect = prepare(request_with_quantum(&values, 128));
    let mut left = vec![0.0_f32; 1];
    let mut right = vec![0.0_f32; 1];

    // Out of order: parameter 2 before parameter 0.
    let spans = [
        point(2, ParameterChannel::Left, 12.0),
        point(0, ParameterChannel::Left, THRESHOLD_TARGET),
    ];
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left, &mut right, None, 0, &spans, 128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1, "the out-of-order span is rejected");
    let payload = state(effect.as_ref());
    // Parameter 2 was applied, parameter 0 was not.
    assert_eq!(read_u32(&payload, 5 + 3 * 2), SMOOTHING - 1);
    assert_eq!(read_u32(&payload, 5), 0);

    // `Both` is not a channel this effect accepts.
    let mut effect = prepare(request_with_quantum(&values, 128));
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            0,
            &[point(0, ParameterChannel::Both, THRESHOLD_TARGET)],
            128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1);
    assert_eq!(read_u32(&state(effect.as_ref()), 5), 0);

    // `lookahead` is not automatable: parameter index 7 is out of the ramped range.
    let mut effect = prepare(request_with_quantum(&values, 128));
    let report = effect.process(
        miso_engine_effect_contract::EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            0,
            &[point(7, ParameterChannel::Left, 0.0)],
            128,
        )
        .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1);
    assert_eq!(
        read_f32(&state(effect.as_ref()), 1).to_bits(),
        5.0_f32.to_bits()
    );
}

/// A finished ramp leaves exactly the coefficients a fresh preparation at that value would.
///
/// This is the "one design function" property: the ramping body and the preparation path share
/// `design::design_lane`, so a rendered instance whose ramp has completed and a freshly prepared
/// instance at the target value must render identical bits from then on.
///
/// Red mutation (MUTATIONS.md row 11): design the ballistic coefficients from an `f32`
/// `0.001 * ms * fs` product instead of the `f64` one.
#[test]
fn a_finished_ramp_equals_a_fresh_preparation() {
    let mut values = initial_values();
    for entry in values.iter_mut() {
        if entry.parameter_index == 7 {
            entry.value = 0.0;
        }
    }
    let mut ramped = prepare(request_with_quantum(&values, 128));
    let mut warm_left = vec![0.0_f32; 64];
    let mut warm_right = vec![0.0_f32; 64];
    render_scalar(
        ramped.as_mut(),
        &mut warm_left,
        &mut warm_right,
        64,
        128,
        &[
            (0, point(0, ParameterChannel::Left, -36.0)),
            (0, point(0, ParameterChannel::Right, -36.0)),
        ],
    );

    let mut target_values = values;
    target_values[0].value = -36.0;
    target_values[1].value = -36.0;
    let mut fresh = prepare(request_with_quantum(&target_values, 128));
    let mut discard_left = vec![0.0_f32; 64];
    let mut discard_right = vec![0.0_f32; 64];
    render_scalar(
        fresh.as_mut(),
        &mut discard_left,
        &mut discard_right,
        64,
        128,
        &[],
    );

    let signal = support::noise(2_048, 0x5A_11_9E_01, 0.8);
    let mut ramped_left = signal.clone();
    let mut ramped_right = signal.clone();
    let mut fresh_left = signal.clone();
    let mut fresh_right = signal.clone();
    render_scalar(
        ramped.as_mut(),
        &mut ramped_left,
        &mut ramped_right,
        128,
        128,
        &[],
    );
    render_scalar(
        fresh.as_mut(),
        &mut fresh_left,
        &mut fresh_right,
        128,
        128,
        &[],
    );
    assert_eq!(
        ramped_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        fresh_left.iter().map(|s| s.to_bits()).collect::<Vec<_>>(),
        "a finished ramp must leave the coefficients a fresh preparation has"
    );
}
