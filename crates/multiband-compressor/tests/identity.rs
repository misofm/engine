//! E3 and E4: one body, every width, and no observable block boundary.
//!
//! Both gates are `to_bits` equality, never a tolerance. Version 1 shipped a W8 bank that agreed
//! with its scalar path only to `1e-6 + 2e-5 * |x|` and said so in its own assertion (#94 F10);
//! that tolerance is deleted here rather than loosened, because the scalar product and the two
//! banks are now literally the same generic body at `WIDTH = 1`, 4 and 8.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, NativeEffectFactory, ParameterChannel,
    PreparedAutomationSpan, PreparedNativeEffect, ResetKind,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{point, process, request_with, snapshot, snapshot_track, varied_values};

/// Twelve blocks of 128 frames over eight tracks, with a threshold point on track 0 at block 0.
const BLOCKS: usize = 12;
const FRAMES: usize = 128;
const TRACKS: usize = 8;

fn track_signal(track: usize) -> (Vec<f32>, Vec<f32>) {
    let left = support::signal(BLOCKS * FRAMES, 0xA5A5_0001 + track as u64 * 7);
    let right = support::signal(BLOCKS * FRAMES, 0x5A5A_1001 + track as u64 * 11);
    (left, right)
}

/// Runs the eight tracks as `TRACKS / lanes` banks of `lanes` and returns their interleaved PCM,
/// their per-track snapshots and their per-track reports.
#[allow(clippy::type_complexity)]
fn run_banks(
    width: BankWidth,
    link: LinkMode,
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
    let lanes = width.lanes() as usize;
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    let mut channels = vec![Vec::new(); TRACKS * 2];
    for track in 0..TRACKS {
        let (left, right) = track_signal(track);
        channels[track * 2] = left;
        channels[track * 2 + 1] = right;
    }
    let mut snapshots = Vec::new();
    let mut reports = vec![effect_contract::ProcessReport::default(); TRACKS];
    for group in 0..TRACKS / lanes {
        let requests = (0..lanes)
            .map(|lane| request_with(&sets[group * lanes + lane], link, FRAMES as u32, false))
            .collect::<Vec<_>>();
        let mut bank = support::bank(width, &requests);
        let sizes = requests[0]
            .initial_values
            .first()
            .map(|_| {
                MultibandCompressorFactory
                    .prepare(requests[0])
                    .expect("scalar")
                    .metadata()
                    .state_sizes
            })
            .expect("sizes");
        for block in 0..BLOCKS {
            let mut left = vec![0.0f32; FRAMES * lanes];
            let mut right = vec![0.0f32; FRAMES * lanes];
            for frame in 0..FRAMES {
                for lane in 0..lanes {
                    let track = group * lanes + lane;
                    left[frame * lanes + lane] = channels[track * 2][block * FRAMES + frame];
                    right[frame * lanes + lane] = channels[track * 2 + 1][block * FRAMES + frame];
                }
            }
            let spans: Vec<PreparedAutomationSpan> = if block == 0 && group == 0 {
                vec![point(2, ParameterChannel::Left, 0, -30.0)]
            } else {
                Vec::new()
            };
            let mut offsets = vec![0u32; lanes + 1];
            for slot in offsets.iter_mut().skip(1) {
                *slot = spans.len() as u32;
            }
            let report = bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    FRAMES as u32,
                    width,
                    (block * FRAMES) as u64,
                    &spans,
                    &offsets,
                    FRAMES as u32,
                )
                .expect("bank block"),
            );
            for frame in 0..FRAMES {
                for lane in 0..lanes {
                    let track = group * lanes + lane;
                    channels[track * 2][block * FRAMES + frame] = left[frame * lanes + lane];
                    channels[track * 2 + 1][block * FRAMES + frame] = right[frame * lanes + lane];
                }
            }
            if block + 1 == BLOCKS {
                for lane in 0..lanes {
                    reports[group * lanes + lane] = report.reports[lane];
                }
            }
        }
        for lane in 0..lanes {
            snapshots.push(snapshot_track(bank.as_ref(), lane as u32, sizes));
        }
    }
    (channels, snapshots, reports)
}

/// The eight tracks run one at a time through the scalar product.
#[allow(clippy::type_complexity)]
fn run_scalar(
    link: LinkMode,
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    let mut channels = vec![Vec::new(); TRACKS * 2];
    let mut snapshots = Vec::new();
    let mut reports = Vec::new();
    for track in 0..TRACKS {
        let (mut left, mut right) = track_signal(track);
        let mut effect: Box<dyn PreparedNativeEffect> = MultibandCompressorFactory
            .prepare(request_with(&sets[track], link, FRAMES as u32, false))
            .expect("scalar");
        let mut last = effect_contract::ProcessReport::default();
        for block in 0..BLOCKS {
            let start = block * FRAMES;
            let spans: &[PreparedAutomationSpan] = if block == 0 && track == 0 {
                &[point(2, ParameterChannel::Left, 0, -30.0)]
            } else {
                &[]
            };
            last = effect.process(
                effect_contract::EffectProcessBlock::new(
                    &mut left[start..start + FRAMES],
                    &mut right[start..start + FRAMES],
                    None,
                    start as u64,
                    spans,
                    FRAMES as u32,
                )
                .expect("block"),
            );
        }
        snapshots.push(snapshot(effect.as_ref()));
        reports.push(last);
        channels[track * 2] = left;
        channels[track * 2 + 1] = right;
    }
    (channels, snapshots, reports)
}

/// E3. `WIDTH = 1`, 4 and 8 render the same bits, keep the same state and file the same reports.
///
/// Red mutation: in `lr4_step`, write the all-pass as `x.sub(k2.mul(v1))` instead of
/// `nk2.fma(v1, x)` — algebraically the same, one extra rounding, and the widths still agree with
/// *each other* but no longer with the pinned digest of E5. To make **this** gate red the body has
/// to become width-dependent: give `detector_tap` a `if W == 1` shortcut that skips the wrap.
#[test]
fn lane_identity_across_widths() {
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let (scalar_pcm, scalar_state, scalar_reports) = run_scalar(link);
        for width in [BankWidth::Four, BankWidth::Eight] {
            let (bank_pcm, bank_state, bank_reports) = run_banks(width, link);
            for channel in 0..TRACKS * 2 {
                for frame in 0..BLOCKS * FRAMES {
                    assert_eq!(
                        bank_pcm[channel][frame].to_bits(),
                        scalar_pcm[channel][frame].to_bits(),
                        "link={link:?} width={width:?} channel={channel} frame={frame}"
                    );
                }
            }
            assert_eq!(bank_state, scalar_state, "link={link:?} width={width:?}");
            assert_eq!(
                bank_reports, scalar_reports,
                "link={link:?} width={width:?}"
            );
        }
    }
}

/// E4. Splitting a block anywhere leaves the output and the state bit-identical.
///
/// Automation points land on samples 0 and 3584, which are block starts in every partition, and
/// the ramps they start are what the segment splitter has to get right: a 64-sample ramp that
/// straddles a partition boundary must still snap on exactly its own last sample.
///
/// Red mutation: drop the `- 1` from `segment_length`, so a ramp's snap happens inside a
/// vectorised run instead of at a segment boundary.
#[test]
fn partition_invariance() {
    const TOTAL: usize = 4_096;
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    let spans = [
        point(2, ParameterChannel::Left, 0, -30.0),
        point(2, ParameterChannel::Right, 0, -30.0),
        point(3, ParameterChannel::Left, 0, 8.0),
        point(6, ParameterChannel::Left, 0, 3.0),
    ];
    let later = [
        point(4, ParameterChannel::Left, 3_584, 25.0),
        point(7, ParameterChannel::Right, 3_584, -12.0),
    ];
    let reference = {
        let mut effect = MultibandCompressorFactory
            .prepare(request_with(&sets[3], LinkMode::Maximum, 512, false))
            .expect("scalar");
        let mut left = support::signal(TOTAL, 0xFEED_0001);
        let mut right = support::signal(TOTAL, 0xFEED_0002);
        let mut position = 0;
        while position < TOTAL {
            let frames = core::cmp::min(512, TOTAL - position);
            let block_spans: &[PreparedAutomationSpan] = match position {
                0 => &spans,
                3_584 => &later,
                _ => &[],
            };
            process(
                effect.as_mut(),
                &mut left[position..position + frames],
                &mut right[position..position + frames],
                position as u64,
                block_spans,
                512,
            );
            position += frames;
        }
        (left, right, snapshot(effect.as_ref()))
    };

    for partition in [1usize, 7, 64, 128, 512] {
        let mut effect = MultibandCompressorFactory
            .prepare(request_with(&sets[3], LinkMode::Maximum, 512, false))
            .expect("scalar");
        let mut left = support::signal(TOTAL, 0xFEED_0001);
        let mut right = support::signal(TOTAL, 0xFEED_0002);
        let mut position = 0;
        while position < TOTAL {
            let frames = core::cmp::min(partition, TOTAL - position);
            let block_spans: &[PreparedAutomationSpan] = match position {
                0 => &spans,
                3_584 => &later,
                _ => &[],
            };
            process(
                effect.as_mut(),
                &mut left[position..position + frames],
                &mut right[position..position + frames],
                position as u64,
                block_spans,
                512,
            );
            position += frames;
        }
        for frame in 0..TOTAL {
            assert_eq!(
                left[frame].to_bits(),
                reference.0[frame].to_bits(),
                "partition={partition} frame={frame}"
            );
            assert_eq!(
                right[frame].to_bits(),
                reference.1[frame].to_bits(),
                "partition={partition} frame={frame}"
            );
        }
        assert_eq!(
            snapshot(effect.as_ref()),
            reference.2,
            "partition={partition} state"
        );
    }
}

/// Both resets bring a bank and the scalar product to the same state, from the same history.
#[test]
fn resets_agree_across_widths() {
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    for kind in [
        ResetKind::DiscontinuityKeepParameters,
        ResetKind::FullToDefaults,
    ] {
        let requests = (0..TRACKS)
            .map(|track| request_with(&sets[track], LinkMode::DualMono, FRAMES as u32, false))
            .collect::<Vec<_>>();
        let mut bank = support::bank(BankWidth::Eight, &requests);
        let mut scalars = requests
            .iter()
            .map(|request| {
                MultibandCompressorFactory
                    .prepare(*request)
                    .expect("scalar")
            })
            .collect::<Vec<_>>();
        let sizes = scalars[0].metadata().state_sizes;
        let mut bank_left = vec![0.0f32; FRAMES * TRACKS];
        let mut bank_right = vec![0.0f32; FRAMES * TRACKS];
        for track in 0..TRACKS {
            let (left, right) = track_signal(track);
            for frame in 0..FRAMES {
                bank_left[frame * TRACKS + track] = left[frame];
                bank_right[frame * TRACKS + track] = right[frame];
            }
            let mut scalar_left = left[..FRAMES].to_vec();
            let mut scalar_right = right[..FRAMES].to_vec();
            process(
                scalars[track].as_mut(),
                &mut scalar_left,
                &mut scalar_right,
                0,
                &[],
                FRAMES as u32,
            );
        }
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                FRAMES as u32,
                BankWidth::Eight,
                0,
                &[],
                &[0u32; TRACKS + 1],
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        bank.reset(kind);
        for scalar in &mut scalars {
            scalar.reset(kind);
        }
        for (track, scalar) in scalars.iter().enumerate() {
            assert_eq!(
                snapshot_track(bank.as_ref(), track as u32, sizes),
                snapshot(scalar.as_ref()),
                "{kind:?} track={track}"
            );
        }
    }
}

/// A track snapshotted from one instance and restored into another continues identically, even
/// though the two rings are at different cursors.
///
/// This is what the time-ordered ring buys. The current payload writes both rings oldest-first and
/// carries no cursor word, so a restore rotates the history into whatever position the receiving
/// instance's shared cursor happens to be at. A bank has **one** cursor for eight tracks, so
/// without the rotation a track could only ever be restored into an instance that had processed
/// exactly as many samples.
///
/// Red mutation: drop the `wrap(cursor + 1 + index, ring_len)` rotation from `write_side` and
/// `commit_side` and write the rings in raw slot order.
#[test]
fn a_restored_track_is_rotated_into_the_receiving_cursor() {
    let sets = (0..4).map(varied_values).collect::<Vec<_>>();
    let requests = (0..4)
        .map(|track| request_with(&sets[track], LinkMode::DualMono, 512, false))
        .collect::<Vec<_>>();

    // The donor: a scalar instance of track 1's program, 300 samples in.
    let mut donor = MultibandCompressorFactory
        .prepare(requests[1])
        .expect("scalar");
    let sizes = donor.metadata().state_sizes;
    let mut left = support::signal(1_324, 0xD0D0_1111);
    let mut right = support::signal(1_324, 0xD0D0_2222);
    process(
        donor.as_mut(),
        &mut left[..300],
        &mut right[..300],
        0,
        &[],
        512,
    );
    let saved = snapshot(donor.as_ref());

    // The receiver: a four-lane bank, 700 samples in, so its shared cursor is elsewhere.
    let mut bank = support::bank(BankWidth::Four, &requests);
    let mut bank_left = vec![0.0f32; 700 * 4];
    let mut bank_right = vec![0.0f32; 700 * 4];
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut bank_left,
            &mut bank_right,
            None,
            700,
            BankWidth::Four,
            0,
            &[],
            &[0u32; 5],
            700,
        )
        .expect("bank block"),
    );
    bank.restore_track_state_payload(
        1,
        1,
        effect_contract::StatePayloadInput::new(&saved.0, &saved.1, &saved.2, sizes)
            .expect("payload"),
    )
    .expect("restore");

    // Both now continue over the same 512 frames; track 1 of the bank must match the donor.
    let tail_left = support::signal(512, 0xD0D0_3333);
    let tail_right = support::signal(512, 0xD0D0_4444);
    let mut donor_left = tail_left.clone();
    let mut donor_right = tail_right.clone();
    process(
        donor.as_mut(),
        &mut donor_left,
        &mut donor_right,
        300,
        &[],
        512,
    );
    let mut bank_left = vec![0.0f32; 512 * 4];
    let mut bank_right = vec![0.0f32; 512 * 4];
    for frame in 0..512 {
        bank_left[frame * 4 + 1] = tail_left[frame];
        bank_right[frame * 4 + 1] = tail_right[frame];
    }
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut bank_left,
            &mut bank_right,
            None,
            512,
            BankWidth::Four,
            700,
            &[],
            &[0u32; 5],
            512,
        )
        .expect("bank block"),
    );
    for frame in 0..512 {
        assert_eq!(
            bank_left[frame * 4 + 1].to_bits(),
            donor_left[frame].to_bits(),
            "frame {frame}: the restored track must continue the donor's history"
        );
    }
}
