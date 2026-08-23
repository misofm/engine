//! E3 — a block boundary is not observable.
//!
//! The same 4,096 frames rendered in blocks of 1, 7, 64, 128 and 512 produce identical output bits
//! and identical state bytes. Partition invariance is what says the kernel carries no per-block
//! state it forgot to write back, and that the ramping/idle split of `process_block` cuts the
//! block at the right frame.
//!
//! Automation is delivered at sample 0 on all seven smoothed parameters and on both channels, so
//! every partition renders across the ramping body — including the partitions that cut the
//! 64-sample ramp in the middle.

mod support;

use miso_engine_effect_contract::{AutomationSpanKind, ParameterChannel, PreparedAutomationSpan};

use support::{noise, prepare, render_scalar, request_with_quantum, snapshot, values_with};

const FRAMES: usize = 4_096;
const QUANTUM: u32 = 512;
const PARTITIONS: [usize; 5] = [1, 7, 64, 128, 512];

/// A Point on every smoothed parameter, both channels, in the strictly increasing order the
/// contract requires.
fn every_parameter() -> Vec<(u64, PreparedAutomationSpan)> {
    let targets = [-52.0_f32, 11.0, 15.0, 3.0, 700.0, -7.0, 0.4];
    let mut spans = Vec::new();
    for (parameter, value) in targets.iter().copied().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            spans.push((
                0_u64,
                PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index: parameter as u32,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: value,
                    end_value: value,
                },
            ));
        }
    }
    spans
}

/// Output bits and state bytes are identical at every partition.
///
/// Red mutation (MUTATIONS.md row 4): drop the ring wrap, so `next` runs past `B`. Two mutations
/// that might be expected here are recorded elsewhere instead, honestly: keeping `g` in a local is
/// invisible to *this* test (both partitions lose it identically) and is gated by `cross_target`
/// (row 7), and `ramping = frames` is an equivalent mutation (row 24) because advancing a ramp with
/// `remaining == 0` is a no-op.
#[test]
fn block_partitions_are_invariant() {
    let values = values_with(&[(0, -24.0), (1, 6.0), (2, 9.0), (7, 5.0)]);
    let input_left = noise(FRAMES, 0x9A_27_10_01, 0.85);
    let input_right = noise(FRAMES, 0x9A_27_10_02, 0.85);
    let spans = every_parameter();

    /// Rendered bits and payload bytes of one run: left samples, right samples, left state,
    /// right state.
    type Rendered = (Vec<u32>, Vec<u32>, Vec<u8>, Vec<u8>);
    let mut reference: Option<Rendered> = None;
    for partition in PARTITIONS {
        let mut effect = prepare(request_with_quantum(&values, QUANTUM));
        let mut left = input_left.clone();
        let mut right = input_right.clone();
        let report = render_scalar(
            effect.as_mut(),
            &mut left,
            &mut right,
            partition,
            QUANTUM,
            &spans,
        );
        assert_eq!(report.invalid_spans, 0, "partition {partition}");
        let (state_left, state_right) = snapshot(effect.as_ref());
        let bits_left: Vec<u32> = left.iter().map(|sample| sample.to_bits()).collect();
        let bits_right: Vec<u32> = right.iter().map(|sample| sample.to_bits()).collect();
        match &reference {
            None => {
                assert!(
                    left[1_000..].iter().any(|sample| *sample != 0.0),
                    "the render must have content"
                );
                reference = Some((bits_left, bits_right, state_left, state_right));
            }
            Some((expected_left, expected_right, expected_state_left, expected_state_right)) => {
                assert_eq!(&bits_left, expected_left, "left, partition {partition}");
                assert_eq!(&bits_right, expected_right, "right, partition {partition}");
                assert_eq!(
                    &state_left, expected_state_left,
                    "left state, partition {partition}"
                );
                assert_eq!(
                    &state_right, expected_state_right,
                    "right state, partition {partition}"
                );
            }
        }
    }
}

/// The same property for the bank, at this build's width.
#[test]
fn bank_block_partitions_are_invariant() {
    let Some((_, width)) = support::native_bank_width() else {
        println!(
            "no bank width on this backend; partition invariance is covered by the scalar case"
        );
        return;
    };
    let lanes = width.lanes() as usize;
    let values: Vec<_> = (0..lanes)
        .map(|track| {
            values_with(&[
                (0, -12.0 - 4.0 * track as f32),
                (1, 2.0 + track as f32),
                (2, 3.0 * (track % 3) as f32),
                (7, 2.5 * (track % 4) as f32),
            ])
        })
        .collect();
    let requests: Vec<_> = values
        .iter()
        .map(|v| request_with_quantum(v, QUANTUM))
        .collect();

    let signal = noise(FRAMES * lanes, 0x9A_27_10_03, 0.85);
    let spans: Vec<(u64, usize, PreparedAutomationSpan)> = (0..lanes)
        .map(|track| {
            (
                0_u64,
                track,
                PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel: ParameterChannel::Left,
                    parameter_index: 0,
                    start_sample: 0,
                    end_sample: 0,
                    start_value: -30.0 - track as f32,
                    end_value: -30.0 - track as f32,
                },
            )
        })
        .collect();

    let mut reference: Option<Vec<u32>> = None;
    for partition in PARTITIONS {
        let mut bank = support::bind_bank(&requests).expect("bank");
        let mut left = signal.clone();
        let mut right = signal.clone();
        support::render_bank(
            bank.as_mut(),
            &mut left,
            &mut right,
            lanes,
            width,
            partition,
            QUANTUM,
            &spans,
        );
        let bits: Vec<u32> = left
            .iter()
            .chain(right.iter())
            .map(|sample| sample.to_bits())
            .collect();
        match &reference {
            None => {
                assert!(left[2_000 * lanes..].iter().any(|sample| *sample != 0.0));
                reference = Some(bits);
            }
            Some(expected) => assert_eq!(&bits, expected, "bank partition {partition}"),
        }
    }
}
