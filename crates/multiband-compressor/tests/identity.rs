//! E3 and E4: one body, every width, and no observable block boundary.
//!
//! Both gates are `to_bits` equality, never a tolerance. Version 1 shipped a W8 bank that agreed
//! with its scalar path only to `1e-6 + 2e-5 * |x|` and said so in its own assertion (#94 F10);
//! that tolerance is deleted here rather than loosened, because the scalar product and the two
//! banks are now literally the same generic body at `WIDTH = 1`, 4 and 8.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, NativeEffectFactory, ParameterChannel,
    PreparedAutomationSpan, PreparedNativeEffect, PreparedNativeEffectBank, ResetKind,
    StatePayloadInput,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{
    PARAMETER_COUNT, point, process, request_with, restore, snapshot, snapshot_track, varied_values,
};

/// Twelve blocks of 128 frames over eight tracks, with a threshold point on track 0 at block 0.
const BLOCKS: usize = 12;
const FRAMES: usize = 128;
const TRACKS: usize = 8;

fn track_signal(track: usize) -> (Vec<f32>, Vec<f32>) {
    let left = support::signal(BLOCKS * FRAMES, 0xA5A5_0001 + track as u64 * 7);
    let right = support::signal(BLOCKS * FRAMES, 0x5A5A_1001 + track as u64 * 11);
    (left, right)
}

fn profile_automation() -> [PreparedAutomationSpan; 4] {
    [
        point(1, ParameterChannel::Left, 0, -30.0),
        point(1, ParameterChannel::Right, 0, -27.0),
        point(10, ParameterChannel::Left, 0, 2.0),
        point(10, ParameterChannel::Right, 0, -3.0),
    ]
}

fn assert_populated(label: &str, channels: &[Vec<f32>], states: &[(Vec<u8>, Vec<u8>, Vec<u8>)]) {
    assert!(
        channels.iter().any(|channel| {
            channel.iter().any(|sample| {
                let bits = sample.to_bits();
                bits != 0 && bits != 0x8000_0000
            })
        }),
        "{label}: output must be populated"
    );
    for (track, (_, left, right)) in states.iter().enumerate() {
        assert!(
            state_populated(left),
            "{label}: track {track} left state must be populated"
        );
        assert!(
            state_populated(right),
            "{label}: track {track} right state must be populated"
        );
    }
}

fn state_populated(section: &[u8]) -> bool {
    let word_populated = |word: &[u8]| {
        let bits = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
        bits != 0 && bits != 0x8000_0000
    };
    [1..3, 43..47].into_iter().any(|range| {
        section[range.start * 4..range.end * 4]
            .chunks_exact(4)
            .any(word_populated)
    })
}

fn assert_state_populated(label: &str, state: &(Vec<u8>, Vec<u8>, Vec<u8>)) {
    assert!(
        state_populated(&state.1),
        "{label}: left state must contain a numeric nonzero word"
    );
    assert!(
        state_populated(&state.2),
        "{label}: right state must contain a numeric nonzero word"
    );
}

fn process_scalar_frames(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
) {
    assert_eq!(left.len(), right.len());
    let mut position = 0;
    while position < left.len() {
        let frames = (left.len() - position).min(FRAMES);
        process(
            effect,
            &mut left[position..position + frames],
            &mut right[position..position + frames],
            first_sample + position as u64,
            &[],
            FRAMES as u32,
        );
        position += frames;
    }
}

fn process_bank_frames(
    bank: &mut dyn PreparedNativeEffectBank,
    width: BankWidth,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
) {
    let lanes = width.lanes() as usize;
    assert_eq!(left.len(), right.len());
    assert_eq!(left.len() % lanes, 0);
    let total = left.len() / lanes;
    let offsets = vec![0u32; lanes + 1];
    let mut position = 0;
    while position < total {
        let frames = (total - position).min(FRAMES);
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[position * lanes..(position + frames) * lanes],
                &mut right[position * lanes..(position + frames) * lanes],
                None,
                frames as u32,
                width,
                first_sample + position as u64,
                &[],
                &offsets,
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        position += frames;
    }
}

fn bank_signal(frames: usize, lanes: usize, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![0.0f32; frames * lanes];
    let mut right = vec![0.0f32; frames * lanes];
    for lane in 0..lanes {
        let lane_left = support::signal(frames, seed + lane as u64 * 7);
        let lane_right = support::signal(frames, seed + 0x1000 + lane as u64 * 11);
        for frame in 0..frames {
            left[frame * lanes + lane] = lane_left[frame];
            right[frame * lanes + lane] = lane_right[frame];
        }
    }
    (left, right)
}

/// Runs the eight tracks as `TRACKS / lanes` banks of `lanes` and returns their interleaved PCM,
/// their per-track snapshots and their per-track reports.
#[allow(clippy::type_complexity)]
fn run_banks_with_sets(
    width: BankWidth,
    link: LinkMode,
    sets: &[[effect_contract::InitialParameterValue; PARAMETER_COUNT * 2]],
    automation: &[PreparedAutomationSpan],
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
    let lanes = width.lanes() as usize;
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
            let spans: &[PreparedAutomationSpan] = if block == 0 && group == 0 {
                automation
            } else {
                &[]
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
                    spans,
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

#[allow(clippy::type_complexity)]
fn run_banks(
    width: BankWidth,
    link: LinkMode,
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    let automation = [point(1, ParameterChannel::Left, 0, -30.0)];
    run_banks_with_sets(width, link, &sets, &automation)
}

/// The eight tracks run one at a time through the scalar product.
#[allow(clippy::type_complexity)]
fn run_scalar_with_sets(
    link: LinkMode,
    sets: &[[effect_contract::InitialParameterValue; PARAMETER_COUNT * 2]],
    automation: &[PreparedAutomationSpan],
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
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
                automation
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

#[allow(clippy::type_complexity)]
fn run_scalar(
    link: LinkMode,
) -> (
    Vec<Vec<f32>>,
    Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    Vec<effect_contract::ProcessReport>,
) {
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    let automation = [point(1, ParameterChannel::Left, 0, -30.0)];
    run_scalar_with_sets(link, &sets, &automation)
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

/// Heterogeneous band programs keep the public scalar and bank products bit-identical. The first
/// block carries settled-to-ramped control points; later blocks exercise the settled segment path.
#[test]
fn heterogeneous_programs_preserve_public_identity() {
    let automation = profile_automation();
    let sets = (0..TRACKS).map(varied_values).collect::<Vec<_>>();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let (scalar_pcm, scalar_state, scalar_reports) =
            run_scalar_with_sets(link, &sets, &automation);
        assert_populated("scalar programs", &scalar_pcm, &scalar_state);
        for width in [BankWidth::Four, BankWidth::Eight] {
            let (bank_pcm, bank_state, bank_reports) =
                run_banks_with_sets(width, link, &sets, &automation);
            assert_populated("bank programs", &bank_pcm, &bank_state);
            for channel in 0..TRACKS * 2 {
                for frame in 0..BLOCKS * FRAMES {
                    assert_eq!(
                        bank_pcm[channel][frame].to_bits(),
                        scalar_pcm[channel][frame].to_bits(),
                        "link={link:?} width={width:?} channel={channel} frame={frame}"
                    );
                }
            }
            assert_eq!(bank_state, scalar_state, "link={link:?}");
            assert_eq!(
                bank_reports, scalar_reports,
                "link={link:?} width={width:?}"
            );
        }
    }
}

const RESTORE_PREFIX: usize = BLOCKS * FRAMES;
const RESTORE_TAIL: usize = 4_096;

fn transition_values(
    track: usize,
    variant: usize,
) -> [effect_contract::InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut values = varied_values(track);
    if variant == 0 {
        values[0].value = 800.0;
        values[1].value = 1_200.0;
    } else {
        values[0].value = 2_000.0;
        values[1].value = 2_400.0;
        values[10].value = 3.0;
        values[11].value = -2.0;
    }
    values
}

fn assert_scalar_restore_transition(source: usize, destination: usize, link: LinkMode) {
    let source_values = transition_values(3, source);
    let destination_values = transition_values(3, destination);
    let mut donor = MultibandCompressorFactory
        .prepare(request_with(&source_values, link, FRAMES as u32, false))
        .expect("source scalar");
    let sizes = donor.metadata().state_sizes;
    let mut donor_left = support::signal(RESTORE_PREFIX, 0xC0DE_0101);
    let mut donor_right = support::signal(RESTORE_PREFIX, 0xC0DE_0202);
    process_scalar_frames(donor.as_mut(), &mut donor_left, &mut donor_right, 0);
    let saved = snapshot(donor.as_ref());
    assert_state_populated("scalar donor", &saved);

    let mut reference = MultibandCompressorFactory
        .prepare(request_with(&source_values, link, FRAMES as u32, false))
        .expect("source reference");
    let mut reference_left = support::signal(RESTORE_PREFIX, 0xC0DE_0101);
    let mut reference_right = support::signal(RESTORE_PREFIX, 0xC0DE_0202);
    process_scalar_frames(
        reference.as_mut(),
        &mut reference_left,
        &mut reference_right,
        0,
    );

    let mut receiver = MultibandCompressorFactory
        .prepare(request_with(
            &destination_values,
            link,
            FRAMES as u32,
            false,
        ))
        .expect("destination scalar");
    let mut warm_left = support::signal(RESTORE_PREFIX, 0xBEEF_0303);
    let mut warm_right = support::signal(RESTORE_PREFIX, 0xBEEF_0404);
    process_scalar_frames(receiver.as_mut(), &mut warm_left, &mut warm_right, 0);
    assert_ne!(
        snapshot(receiver.as_ref()),
        saved,
        "{source:?}->{destination:?} link={link:?}: warm state must differ"
    );
    assert_eq!(
        restore(receiver.as_mut(), 1, &saved, sizes),
        Ok(()),
        "{source:?}->{destination:?} link={link:?}: restore"
    );
    assert_eq!(snapshot(receiver.as_ref()), saved);

    let mut expected_tail_left = support::signal(RESTORE_TAIL, 0xC0DE_0505);
    let mut expected_tail_right = support::signal(RESTORE_TAIL, 0xC0DE_0606);
    process_scalar_frames(
        reference.as_mut(),
        &mut expected_tail_left,
        &mut expected_tail_right,
        RESTORE_PREFIX as u64,
    );
    let mut restored_tail_left = support::signal(RESTORE_TAIL, 0xC0DE_0505);
    let mut restored_tail_right = support::signal(RESTORE_TAIL, 0xC0DE_0606);
    process_scalar_frames(
        receiver.as_mut(),
        &mut restored_tail_left,
        &mut restored_tail_right,
        RESTORE_PREFIX as u64,
    );
    assert!(
        expected_tail_left.iter().any(|sample| {
            let bits = sample.to_bits();
            bits != 0 && bits != 0x8000_0000
        }),
        "{source:?}->{destination:?} link={link:?}: restored tail must be populated"
    );
    for frame in 0..RESTORE_TAIL {
        assert_eq!(
            restored_tail_left[frame].to_bits(),
            expected_tail_left[frame].to_bits(),
            "{source:?}->{destination:?} link={link:?} left frame={frame}"
        );
        assert_eq!(
            restored_tail_right[frame].to_bits(),
            expected_tail_right[frame].to_bits(),
            "{source:?}->{destination:?} link={link:?} right frame={frame}"
        );
    }
    assert_eq!(
        snapshot(receiver.as_ref()),
        snapshot(reference.as_ref()),
        "{source:?}->{destination:?} link={link:?}: restored state"
    );

    receiver.reset(ResetKind::FullToDefaults);
    let mut reset_reference = MultibandCompressorFactory
        .prepare(request_with(
            &destination_values,
            link,
            FRAMES as u32,
            false,
        ))
        .expect("reset reference");
    let mut reset_left = support::signal(RESTORE_PREFIX, 0xD00D_0707);
    let mut reset_right = support::signal(RESTORE_PREFIX, 0xD00D_0808);
    let mut reset_reference_left = reset_left.clone();
    let mut reset_reference_right = reset_right.clone();
    process_scalar_frames(receiver.as_mut(), &mut reset_left, &mut reset_right, 0);
    process_scalar_frames(
        reset_reference.as_mut(),
        &mut reset_reference_left,
        &mut reset_reference_right,
        0,
    );
    assert_populated(
        "full reset scalar",
        &[reset_left.clone(), reset_right.clone()],
        &[snapshot(receiver.as_ref())],
    );
    assert_eq!(reset_left, reset_reference_left);
    assert_eq!(reset_right, reset_reference_right);
    assert_eq!(
        snapshot(receiver.as_ref()),
        snapshot(reset_reference.as_ref()),
        "{source:?}->{destination:?} link={link:?}: full reset state"
    );
}

fn assert_bank_restore_transition(
    source: usize,
    destination: usize,
    link: LinkMode,
    width: BankWidth,
) {
    let lanes = width.lanes() as usize;
    let source_sets = (0..lanes)
        .map(|track| transition_values(track, source))
        .collect::<Vec<_>>();
    let destination_sets = (0..lanes)
        .map(|track| transition_values(track, destination))
        .collect::<Vec<_>>();
    let source_requests = source_sets
        .iter()
        .map(|values| request_with(values, link, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let destination_requests = destination_sets
        .iter()
        .map(|values| request_with(values, link, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let mut donor = support::bank(width, &source_requests);
    let sizes = donor.metadata().program_key.state_sizes;
    let (mut donor_left, mut donor_right) = bank_signal(RESTORE_PREFIX, lanes, 0xABCD_0101);
    process_bank_frames(donor.as_mut(), width, &mut donor_left, &mut donor_right, 0);
    let saved = (0..lanes)
        .map(|lane| snapshot_track(donor.as_ref(), lane as u32, sizes))
        .collect::<Vec<_>>();
    for (lane, state) in saved.iter().enumerate() {
        assert_state_populated(&format!("bank donor lane {lane}"), state);
    }

    let mut reference = support::bank(width, &source_requests);
    let (mut reference_left, mut reference_right) = bank_signal(RESTORE_PREFIX, lanes, 0xABCD_0101);
    process_bank_frames(
        reference.as_mut(),
        width,
        &mut reference_left,
        &mut reference_right,
        0,
    );

    let mut receiver = support::bank(width, &destination_requests);
    let (mut warm_left, mut warm_right) = bank_signal(RESTORE_PREFIX, lanes, 0xDCBA_0202);
    process_bank_frames(receiver.as_mut(), width, &mut warm_left, &mut warm_right, 0);
    let warm_state = (0..lanes)
        .map(|lane| snapshot_track(receiver.as_ref(), lane as u32, sizes))
        .collect::<Vec<_>>();
    assert_ne!(
        warm_state, saved,
        "{source:?}->{destination:?} width={width:?}"
    );
    for (lane, state) in saved.iter().enumerate() {
        receiver
            .restore_track_state_payload(
                lane as u32,
                1,
                StatePayloadInput::new(&state.0, &state.1, &state.2, sizes)
                    .expect("restore payload"),
            )
            .expect("restore track");
    }
    for (lane, state) in saved.iter().enumerate() {
        assert_eq!(
            snapshot_track(receiver.as_ref(), lane as u32, sizes),
            *state,
            "{source:?}->{destination:?} link={link:?} width={width:?} lane={lane} restore"
        );
    }

    let (mut expected_tail_left, mut expected_tail_right) =
        bank_signal(RESTORE_TAIL, lanes, 0xABCD_0303);
    process_bank_frames(
        reference.as_mut(),
        width,
        &mut expected_tail_left,
        &mut expected_tail_right,
        RESTORE_PREFIX as u64,
    );
    let (mut restored_tail_left, mut restored_tail_right) =
        bank_signal(RESTORE_TAIL, lanes, 0xABCD_0303);
    process_bank_frames(
        receiver.as_mut(),
        width,
        &mut restored_tail_left,
        &mut restored_tail_right,
        RESTORE_PREFIX as u64,
    );
    assert!(
        expected_tail_left.iter().any(|sample| {
            let bits = sample.to_bits();
            bits != 0 && bits != 0x8000_0000
        }),
        "{source:?}->{destination:?} link={link:?} width={width:?}: restored tail must be populated"
    );
    for index in 0..expected_tail_left.len() {
        assert_eq!(
            restored_tail_left[index].to_bits(),
            expected_tail_left[index].to_bits(),
            "{source:?}->{destination:?} link={link:?} width={width:?} left index={index}"
        );
        assert_eq!(
            restored_tail_right[index].to_bits(),
            expected_tail_right[index].to_bits(),
            "{source:?}->{destination:?} link={link:?} width={width:?} right index={index}"
        );
    }
    for lane in 0..lanes {
        assert_eq!(
            snapshot_track(reference.as_ref(), lane as u32, sizes),
            snapshot_track(receiver.as_ref(), lane as u32, sizes),
            "{source:?}->{destination:?} link={link:?} width={width:?} lane={lane} final state"
        );
    }
}

#[test]
fn restored_programs_continue_across_scalar_and_bank() {
    for (source, destination) in [(0, 1), (1, 0)] {
        for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
            assert_scalar_restore_transition(source, destination, link);
            for width in [BankWidth::Four, BankWidth::Eight] {
                assert_bank_restore_transition(source, destination, link, width);
            }
        }
    }
}

/// State payloads keep their meaning when moving in either direction between the scalar and bank
/// implementations. The continuation probes use nonzero filter and gain history after restore.
#[test]
fn scalar_and_bank_state_interchange_continues() {
    let values = transition_values(3, 0);
    let mut scalar = MultibandCompressorFactory
        .prepare(request_with(
            &values,
            LinkMode::Maximum,
            FRAMES as u32,
            false,
        ))
        .expect("scalar donor");
    let sizes = scalar.metadata().state_sizes;
    let mut prefix_left = support::signal(RESTORE_PREFIX, 0x1357_0001);
    let mut prefix_right = support::signal(RESTORE_PREFIX, 0x1357_0002);
    process_scalar_frames(&mut *scalar, &mut prefix_left, &mut prefix_right, 0);
    let scalar_saved = snapshot(scalar.as_ref());

    let requests = (0..4)
        .map(|_| request_with(&values, LinkMode::Maximum, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let mut bank = support::bank(BankWidth::Four, &requests);
    let (mut warm_left, mut warm_right) = bank_signal(RESTORE_PREFIX, 4, 0x2468_0001);
    process_bank_frames(
        bank.as_mut(),
        BankWidth::Four,
        &mut warm_left,
        &mut warm_right,
        0,
    );
    bank.restore_track_state_payload(
        0,
        1,
        StatePayloadInput::new(&scalar_saved.0, &scalar_saved.1, &scalar_saved.2, sizes)
            .expect("scalar-to-bank payload"),
    )
    .expect("scalar-to-bank restore");

    let (mut scalar_tail_left, mut scalar_tail_right) = bank_signal(RESTORE_TAIL, 1, 0x2468_1001);
    process_scalar_frames(
        &mut *scalar,
        &mut scalar_tail_left,
        &mut scalar_tail_right,
        RESTORE_PREFIX as u64,
    );
    let (mut bank_tail_left, mut bank_tail_right) = bank_signal(RESTORE_TAIL, 4, 0x2468_1001);
    process_bank_frames(
        bank.as_mut(),
        BankWidth::Four,
        &mut bank_tail_left,
        &mut bank_tail_right,
        RESTORE_PREFIX as u64,
    );
    assert!(scalar_tail_left.iter().any(|sample| *sample != 0.0));
    for frame in 0..RESTORE_TAIL {
        assert_eq!(
            bank_tail_left[frame * 4].to_bits(),
            scalar_tail_left[frame].to_bits(),
            "scalar-to-bank left frame={frame}"
        );
        assert_eq!(
            bank_tail_right[frame * 4].to_bits(),
            scalar_tail_right[frame].to_bits(),
            "scalar-to-bank right frame={frame}"
        );
    }

    let mut bank_donor = support::bank(BankWidth::Four, &requests);
    let (mut donor_left, mut donor_right) = bank_signal(RESTORE_PREFIX, 4, 0x9753_0001);
    process_bank_frames(
        bank_donor.as_mut(),
        BankWidth::Four,
        &mut donor_left,
        &mut donor_right,
        0,
    );
    let bank_saved = snapshot_track(bank_donor.as_ref(), 0, sizes);
    let mut scalar_receiver = MultibandCompressorFactory
        .prepare(request_with(
            &values,
            LinkMode::Maximum,
            FRAMES as u32,
            false,
        ))
        .expect("scalar receiver");
    let mut receiver_left = support::signal(RESTORE_PREFIX, 0x8642_0001);
    let mut receiver_right = support::signal(RESTORE_PREFIX, 0x8642_0002);
    process_scalar_frames(
        scalar_receiver.as_mut(),
        &mut receiver_left,
        &mut receiver_right,
        0,
    );
    restore(&mut *scalar_receiver, 1, &bank_saved, sizes).expect("bank-to-scalar restore");
    let (mut bank_expected_left, mut bank_expected_right) =
        bank_signal(RESTORE_TAIL, 4, 0x9753_1001);
    process_bank_frames(
        bank_donor.as_mut(),
        BankWidth::Four,
        &mut bank_expected_left,
        &mut bank_expected_right,
        RESTORE_PREFIX as u64,
    );
    let (mut scalar_tail_left, mut scalar_tail_right) = bank_signal(RESTORE_TAIL, 1, 0x9753_1001);
    process_scalar_frames(
        scalar_receiver.as_mut(),
        &mut scalar_tail_left,
        &mut scalar_tail_right,
        RESTORE_PREFIX as u64,
    );
    assert!(scalar_tail_left.iter().any(|sample| *sample != 0.0));
    for frame in 0..RESTORE_TAIL {
        assert_eq!(
            scalar_tail_left[frame].to_bits(),
            bank_expected_left[frame * 4].to_bits(),
            "bank-to-scalar left frame={frame}"
        );
        assert_eq!(
            scalar_tail_right[frame].to_bits(),
            bank_expected_right[frame * 4].to_bits(),
            "bank-to-scalar right frame={frame}"
        );
    }
}

/// Full reset restores prepared coefficients for lanes with different crossover frequencies.
#[test]
fn bank_full_reset_restores_different_crossover_defaults() {
    let mut sets = (0..4).map(|track| varied_values(track)).collect::<Vec<_>>();
    for (track, values) in sets.iter_mut().enumerate() {
        values[0].value = 700.0 + track as f32 * 300.0;
        values[1].value = 900.0 + track as f32 * 300.0;
    }
    let requests = sets
        .iter()
        .map(|values| request_with(values, LinkMode::DualMono, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let mut bank = support::bank(BankWidth::Four, &requests);
    let sizes = bank.metadata().program_key.state_sizes;
    let (mut left, mut right) = bank_signal(RESTORE_PREFIX, 4, 0xCAFE_0001);
    process_bank_frames(bank.as_mut(), BankWidth::Four, &mut left, &mut right, 0);
    bank.reset(ResetKind::FullToDefaults);
    let fresh = support::bank(BankWidth::Four, &requests);
    for track in 0..4 {
        assert_eq!(
            snapshot_track(bank.as_ref(), track, sizes),
            snapshot_track(fresh.as_ref(), track, sizes),
            "different-crossover full reset track={track}"
        );
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
        point(1, ParameterChannel::Left, 0, -30.0),
        point(1, ParameterChannel::Right, 0, -30.0),
        point(2, ParameterChannel::Left, 0, 8.0),
        point(5, ParameterChannel::Left, 0, 3.0),
    ];
    let later = [
        point(3, ParameterChannel::Left, 3_584, 25.0),
        point(6, ParameterChannel::Right, 3_584, -12.0),
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
