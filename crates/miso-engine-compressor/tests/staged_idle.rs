//! The staged idle body and the per-frame body are the same renderer.
//!
//! `kernel::process_block` sends an idle segment to `idle_frames_staged` when every live lane's
//! detector distance `D` is at least the segment length, and to `frames_loop` when it is not. The
//! first pre-gathers the whole segment's detector taps and visits the segment three times; the
//! second reads one tap per frame and visits it once. They are claimed bit-identical, and which
//! one runs is a function of the lanes' lookahead and the caller's block size alone.
//!
//! This file is that claim as a gate. `D = N - L` with `N = 960` at 48 kHz, so a lookahead chosen
//! in milliseconds picks `D` directly, and rendering the same input at partitions that straddle
//! `D` puts the same frames through both bodies. The 512-frame partition is always the per-frame
//! body — it is longer than the staged body's 128-frame bound — so it is the reference every other
//! partition is compared against.
//!
//! Red mutations (MUTATIONS.md rows 25 to 27): loosen the guard by one frame, drop the `D == 0`
//! case, or gather the taps after the segment's first ring write instead of before it.

mod support;

use miso_engine_effect_contract::{
    EffectProcessBlock, LinkMode, PreparedSidechainPort, ProcessReport,
};

use support::{
    accumulate, noise, prepare, render_scalar, request_with_quantum, sidechain_port, snapshot,
    values_with,
};

/// Frames rendered per case.
const FRAMES: usize = 4_096;

/// Prepared quantum. Every partition below is at most this.
const QUANTUM: u32 = 512;

/// Block partitions, straddling the detector distances of [`LOOKAHEAD_MS`].
///
/// 512 is longer than the staged body's bound, so it is always the per-frame body and is used as
/// the reference. 63/64/65 and 127/128/129 bracket the two guard boundaries exactly.
const PARTITIONS: [usize; 9] = [1, 7, 63, 64, 65, 127, 128, 129, 512];

/// Lookaheads in ms and the detector distance each one produces at 48 kHz with `N = 960`.
///
/// `D = 960 - round(ms * 48)`: 960, 720, 120, 64, 24, 0. The last is the maximum lookahead, whose
/// tap is the row the frame itself has just written — the case that can never be pre-gathered.
const LOOKAHEAD_MS: [(f32, usize); 6] = [
    (0.0, 960),
    (5.0, 720),
    (17.5, 120),
    (18.666_667, 64),
    (19.5, 24),
    (20.0, 0),
];

/// Rendered bits and payload bytes of one run: left samples, right samples, left state, right
/// state.
type Rendered = (Vec<u32>, Vec<u32>, Vec<u8>, Vec<u8>);

/// Renders `FRAMES` frames of one configuration at one partition and returns its bits and state.
fn render(lookahead_ms: f32, partition: usize) -> Rendered {
    let values = values_with(&[(0, -20.0), (1, 5.0), (2, 6.0), (5, 3.0), (7, lookahead_ms)]);
    let mut effect = prepare(request_with_quantum(&values, QUANTUM));
    let mut left = noise(FRAMES, 0x5A_6E_D0_01, 0.8);
    let mut right = noise(FRAMES, 0x5A_6E_D0_02, 0.8);
    let report = render_scalar(
        effect.as_mut(),
        &mut left,
        &mut right,
        partition,
        QUANTUM,
        &[],
    );
    assert_eq!(report.invalid_spans, 0);
    let (state_left, state_right) = snapshot(effect.as_ref());
    (
        left.iter().map(|sample| sample.to_bits()).collect(),
        right.iter().map(|sample| sample.to_bits()).collect(),
        state_left,
        state_right,
    )
}

/// Every partition of every lookahead renders the bits and the state the per-frame body renders.
///
/// At `D = 64` the partitions 63 and 64 are staged and 65 is not, which is the guard's `D >= len`
/// boundary on the nose; at `D = 120` the boundary falls between 127 and 128; at `D = 0` nothing
/// is ever staged, so that row asserts the fallback is still reachable and still right.
#[test]
fn every_partition_agrees_with_the_per_frame_body() {
    for (lookahead_ms, delay) in LOOKAHEAD_MS {
        let (expected_left, expected_right, expected_state_left, expected_state_right) =
            render(lookahead_ms, 512);
        assert!(
            expected_left[2_000..]
                .iter()
                .any(|sample| *sample != 0_f32.to_bits()),
            "D = {delay} must render content"
        );
        for partition in PARTITIONS {
            let (bits_left, bits_right, state_left, state_right) = render(lookahead_ms, partition);
            assert_eq!(
                bits_left, expected_left,
                "left, D = {delay}, partition {partition}"
            );
            assert_eq!(
                bits_right, expected_right,
                "right, D = {delay}, partition {partition}"
            );
            assert_eq!(
                state_left, expected_state_left,
                "left state, D = {delay}, partition {partition}"
            );
            assert_eq!(
                state_right, expected_state_right,
                "right state, D = {delay}, partition {partition}"
            );
        }
    }
}

/// The same property for a bank whose lanes disagree about their lookahead.
///
/// The guard is whole-bank: one lane with a short `D` keeps the whole bank on the per-frame body,
/// because the pre-gather is one buffer per channel and not one per lane. Lane 0 is given the
/// maximum lookahead in the second configuration, so the bank is forced onto the fallback while
/// every other lane would have qualified.
#[test]
fn a_ragged_bank_agrees_with_the_per_frame_body() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("no bank width on this backend; the scalar case carries this gate");
        return;
    };
    let lanes = width.lanes() as usize;
    for forced in [false, true] {
        let values: Vec<_> = (0..lanes)
            .map(|track| {
                let lookahead = if forced && track == 0 {
                    20.0
                } else {
                    // 0, 2.5, 5, 7.5, ... ms: D of 960, 840, 720, 600, ...
                    2.5 * (track % 8) as f32
                };
                values_with(&[
                    (0, -18.0 - track as f32),
                    (1, 3.0 + track as f32),
                    (5, 2.0),
                    (7, lookahead),
                ])
            })
            .collect();
        let requests: Vec<_> = values
            .iter()
            .map(|v| request_with_quantum(v, QUANTUM))
            .collect();
        let signal = noise(FRAMES * lanes, 0x5A_6E_D0_03, 0.8);

        let mut reference: Option<Vec<u32>> = None;
        for partition in [512, 1, 7, 64, 65, 128, 129] {
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
                &[],
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
                Some(expected) => {
                    assert_eq!(
                        &bits, expected,
                        "forced {forced}, bank partition {partition}"
                    )
                }
            }
        }
    }
}

/// The same property across the three link laws, the bypass flag and a connected sidechain.
///
/// `link_frame` is shared source between the two bodies, so this is a regression guard rather than
/// a second derivation — but the pre-gather and the staging sit either side of it, and a body that
/// linked the wrong pair of magnitudes into the ring would be invisible to a `DualMono` corpus.
/// The frozen cross-target corpus renders in 100-frame blocks against detector distances as short
/// as 16, so it takes the per-frame body on every case and covers none of this.
#[test]
fn the_link_laws_bypass_and_a_sidechain_all_agree() {
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        for bypass in [false, true] {
            for sidechain in [false, true] {
                let mut reference: Option<Rendered> = None;
                for partition in [512, 1, 7, 64, 128] {
                    let rendered = render_case(link, bypass, sidechain, partition);
                    match &reference {
                        None => reference = Some(rendered),
                        Some(expected) => assert_eq!(
                            &rendered, expected,
                            "{link:?}, bypass {bypass}, sidechain {sidechain}, partition {partition}"
                        ),
                    }
                }
            }
        }
    }
}

/// One configuration of [`the_link_laws_bypass_and_a_sidechain_all_agree`], rendered at one
/// partition. The sidechain is a second noise plane, so the detector and the delayed output are
/// different signals and a body that confused them would not survive.
fn render_case(link: LinkMode, bypass: bool, sidechain: bool, partition: usize) -> Rendered {
    let values = values_with(&[(0, -22.0), (1, 4.0), (2, 6.0), (5, 2.0), (7, 4.0)]);
    let mut preparation = request_with_quantum(&values, QUANTUM);
    preparation.link_mode = link;
    preparation.bypass = bypass;
    if sidechain {
        preparation.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    let mut effect = prepare(preparation);
    let mut left = noise(FRAMES, 0x5A_6E_D0_04, 0.8);
    let mut right = noise(FRAMES, 0x5A_6E_D0_05, 0.6);
    let detector_left = noise(FRAMES, 0x5A_6E_D0_06, 0.9);
    let detector_right = noise(FRAMES, 0x5A_6E_D0_07, 0.4);

    let mut total = ProcessReport::default();
    let mut offset = 0;
    while offset < FRAMES {
        let end = (offset + partition).min(FRAMES);
        let ports = sidechain.then(|| (&detector_left[offset..end], &detector_right[offset..end]));
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                ports,
                offset as u64,
                &[],
                QUANTUM,
            )
            .expect("bounded block"),
        );
        accumulate(&mut total, report);
        offset = end;
    }
    assert_eq!(total.invalid_spans, 0);
    let (state_left, state_right) = snapshot(effect.as_ref());
    (
        left.iter().map(|sample| sample.to_bits()).collect(),
        right.iter().map(|sample| sample.to_bits()).collect(),
        state_left,
        state_right,
    )
}

/// Frames rendered before the partition is varied.
///
/// Eight 128-frame blocks. The latency is 960 samples, so a rejection triggered at sample 0 lands
/// in block 7 and the prefix ends one block boundary past it — with the rejection, and the reset it
/// causes, already behind us.
const REJECTION_PREFIX_FRAMES: usize = 1_024;

/// Block size the prefix is always cut at, so every run reaches the *same* diverged state.
///
/// The prefix cannot be re-partitioned along with the rest: `bank::finish_channel` zeroes the whole
/// block it rejects, so moving the block boundaries would move which samples are zeroed and
/// partition invariance would fail for a reason that has nothing to do with the staged body.
const REJECTION_PREFIX_PARTITION: usize = 128;

/// A rejection on one channel diverges the two cursors, and the staged body still agrees.
///
/// `Channel::clear_state` zeroes a channel's rings **and** its cursor, and
/// `kernel::finish_channel` runs it per channel, so a block that is out of bounds on the left alone
/// leaves `left.cursor == 0` next to a right cursor that has kept counting. The frozen behaviour is
/// that the left cursor is the shared write index for both channels — `frames_loop` reads
/// `channel_left.cursor` and hands it to `gather_detector` for the right channel too — and
/// `idle_frames_staged` pre-gathers both channels' taps from that same index.
///
/// Which channel is rejected is the whole point. Reject the **right** one and the divergence is
/// unobservable: the reset zeroed right's rings, so every candidate tap row reads `+0.0` and any
/// cursor gives the same answer. Reject the **left** one and right keeps both its diverged cursor
/// and a ring full of real signal, so the two candidate rows hold different samples. The cursors
/// re-converge at the end of the very next block — every body writes both channels at the shared
/// index and leaves both at `next` — so that one block is the entire window in which this is
/// observable, and the prefix is cut to land exactly on its boundary.
///
/// Both channels' rings are `D = 960` deep here and the quantum is 512, so the 512- and 256-frame
/// partitions exceed the staged body's 128-frame bound and take `frames_loop`, while the 128- and
/// 64-frame partitions stage. The first block after the prefix is therefore rendered by the
/// per-frame body in the reference run and by the staged body in the others, from byte-identical
/// diverged state.
///
/// Red mutation (MUTATIONS.md row 31, the adversarial verifier's V3): pre-gather the right
/// channel's taps from `channel_right.cursor` instead of the shared index.
#[test]
fn a_left_only_rejection_diverges_the_cursors_and_the_staged_body_still_agrees() {
    let mut reference: Option<Rendered> = None;
    for partition in [512, 256, 128, 64] {
        let rendered = render_across_a_left_only_rejection(partition);
        match &reference {
            None => {
                assert!(
                    rendered.1[REJECTION_PREFIX_FRAMES..]
                        .iter()
                        .any(|sample| *sample != 0_f32.to_bits()),
                    "the right channel must carry content across the diverged block"
                );
                reference = Some(rendered);
            }
            Some(expected) => assert_eq!(&rendered, expected, "partition {partition}"),
        }
    }
}

/// Renders [`FRAMES`] frames whose left channel is rejected once, cutting the prefix identically
/// and only the remainder at `partition`.
///
/// The `NaN` is the same injection `tests/nonfinite.rs` uses, moved to the left plane. `DualMono`
/// keeps it there: the right channel's detector is its own magnitude, so the right block stays in
/// bounds and is never reset. The two rejection counters are asserted rather than assumed, because
/// a run in which *both* channels were rejected, or neither, would leave the cursors equal and this
/// test would be measuring nothing.
fn render_across_a_left_only_rejection(partition: usize) -> Rendered {
    let values = values_with(&[(0, -30.0), (1, 4.0), (2, 6.0), (5, 2.0), (7, 0.0)]);
    let mut effect = prepare(request_with_quantum(&values, QUANTUM));
    let mut left = noise(FRAMES, 0x5A_6E_D0_08, 0.7);
    let mut right = noise(FRAMES, 0x5A_6E_D0_09, 0.7);
    left[0] = f32::NAN;

    let mut total = ProcessReport::default();
    let mut offset = 0;
    while offset < FRAMES {
        let block = if offset < REJECTION_PREFIX_FRAMES {
            REJECTION_PREFIX_PARTITION
        } else {
            partition
        };
        let end = (offset + block).min(FRAMES);
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[offset..end],
                &mut right[offset..end],
                None,
                offset as u64,
                &[],
                QUANTUM,
            )
            .expect("bounded block"),
        );
        accumulate(&mut total, report);
        offset = end;
    }
    assert_eq!(
        total.nonfinite_left_blocks, 1,
        "exactly one left rejection, partition {partition}"
    );
    assert_eq!(
        total.nonfinite_right_blocks, 0,
        "the right channel is never reset, partition {partition}"
    );

    let (state_left, state_right) = snapshot(effect.as_ref());
    (
        left.iter().map(|sample| sample.to_bits()).collect(),
        right.iter().map(|sample| sample.to_bits()).collect(),
        state_left,
        state_right,
    )
}
