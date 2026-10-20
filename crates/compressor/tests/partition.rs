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

use effect_contract::{
    EffectProcessBlock, LinkMode, ParameterChannel, PreparedAutomationSpan, PreparedSidechainPort,
};

use support::{noise, prepare, request_with_quantum, sidechain_port, snapshot, values_with};

const FRAMES: usize = 4_096;
const QUANTUM: u32 = 512;
const PARTITIONS: [usize; 9] = [1, 7, 63, 64, 65, 127, 128, 129, 512];

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
                    channel: ParameterChannel::Left,
                    parameter_index: 0,
                    sample: 0,
                    value: -30.0 - track as f32,
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

/// A linked detector with an explicit sidechain is partition invariant too. This is the
/// ordinary live path that replaced the old staged/tap partition assertion.
#[test]
fn linked_sidechain_partitions_are_invariant() {
    let values = values_with(&[(0, -36.0), (1, 12.0), (2, 0.0), (6, 1.0)]);
    let main_left = noise(FRAMES, 0x9A_27_10_11, 0.7);
    let main_right = noise(FRAMES, 0x9A_27_10_12, 0.7);
    let detector = noise(FRAMES, 0x9A_27_10_13, 0.9);

    type Rendered = (Vec<u32>, Vec<u32>, Vec<u8>, Vec<u8>);
    let mut reference: Option<Rendered> = None;
    for partition in PARTITIONS {
        let mut preparation = request_with_quantum(&values, QUANTUM);
        preparation.link_mode = LinkMode::Average;
        preparation.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
        let mut effect = prepare(preparation);
        let mut left = main_left.clone();
        let mut right = main_right.clone();
        let mut offset = 0;
        while offset < FRAMES {
            let end = (offset + partition).min(FRAMES);
            effect.process(
                EffectProcessBlock::new(
                    &mut left[offset..end],
                    &mut right[offset..end],
                    Some((&detector[offset..end], &detector[offset..end])),
                    offset as u64,
                    &[],
                    QUANTUM,
                )
                .expect("bounded sidechain block"),
            );
            offset = end;
        }
        let (state_left, state_right) = snapshot(effect.as_ref());
        let rendered = (
            left.iter().map(|sample| sample.to_bits()).collect(),
            right.iter().map(|sample| sample.to_bits()).collect(),
            state_left,
            state_right,
        );
        match &reference {
            None => reference = Some(rendered),
            Some((expected_left, expected_right, expected_state_left, expected_state_right)) => {
                assert_eq!(&rendered.0, expected_left, "left, partition {partition}");
                assert_eq!(&rendered.1, expected_right, "right, partition {partition}");
                assert_eq!(
                    &rendered.2, expected_state_left,
                    "left state, partition {partition}"
                );
                assert_eq!(
                    &rendered.3, expected_state_right,
                    "right state, partition {partition}"
                );
            }
        }
    }
}
