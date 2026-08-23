//! Lane identity: a bank of `W` tracks equals `W` scalar products, bit for bit.
//!
//! Master plan D5 and §10 G2. One generic `#[inline(always)]` body is instantiated at `f32` and at
//! the build's production width, so this is a property of the code; the gate is here to prove the
//! comparison is live and that the *plumbing* around the body — automation offsets, per-track
//! defaults, per-track state sections — is lane local too.

mod common;

use common::*;
use miso_engine_effect_contract::{
    EffectBankProcessBlock, EffectProcessBlock, LinkMode, ParameterChannel, ProcessReport,
    ResetKind, StatePayloadInput,
};

/// The eight per-track parameter sets the bank gates use: every lane a different program point,
/// including a `mix = 0` identity lane.
fn track_values(lanes: usize) -> Vec<[miso_engine_effect_contract::InitialParameterValue; 6]> {
    (0..lanes)
        .map(|track| {
            let attack = -0.75 + track as f32 * 0.2;
            let sustain = 0.6 - track as f32 * 0.15;
            values_of(attack, sustain, 0.25 + track as f32 * 0.1)
        })
        .collect()
}

/// Red mutation: render the scalar side with `bypass = true`. The property holds by construction,
/// so the mutation is what proves the comparison is not vacuous.
#[test]
fn bank_matches_scalar_pcm_state_and_reports_for_every_link_mode() {
    let Some((_, width)) = native_bank() else {
        println!("no bank width on this build; skipping");
        return;
    };
    let lanes = width.lanes() as usize;
    for link_mode in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let values = track_values(lanes);
        let mut bank = bind_native_bank(&values, link_mode).expect("bank");
        let mut scalar = values
            .iter()
            .map(|values| prepare_with(values, 48_000, false, link_mode))
            .collect::<Vec<_>>();
        let spans = (0..lanes)
            .map(|track| {
                let value = if track % PARAMETER_COUNT == 2 {
                    0.5
                } else {
                    -0.5 + track as f32 * 0.1
                };
                point(
                    if track % 2 == 0 {
                        ParameterChannel::Left
                    } else {
                        ParameterChannel::Right
                    },
                    (track % PARAMETER_COUNT) as u32,
                    0,
                    value,
                )
            })
            .collect::<Vec<_>>();
        let offsets = (0..=lanes as u32).collect::<Vec<_>>();
        let frames = 96;
        let mut bank_left = vec![0.0_f32; frames * lanes];
        let mut bank_right = vec![0.0_f32; frames * lanes];
        for frame in 0..frames {
            for track in 0..lanes {
                bank_left[frame * lanes + track] =
                    (frame as f32 * 0.037 + track as f32 * 0.11).sin() * 0.8;
                bank_right[frame * lanes + track] =
                    (frame as f32 * 0.029 - track as f32 * 0.07).cos() * 0.55;
            }
        }
        // The signed-zero identity row, and a subnormal, which is no longer sanitised (D7).
        bank_left[0] = -0.0;
        bank_right[0] = 0.0;
        bank_right[lanes + 3] = f32::from_bits(1);

        let mut scalar_left: Vec<Vec<f32>> = (0..lanes)
            .map(|track| (0..frames).map(|f| bank_left[f * lanes + track]).collect())
            .collect();
        let mut scalar_right: Vec<Vec<f32>> = (0..lanes)
            .map(|track| (0..frames).map(|f| bank_right[f * lanes + track]).collect())
            .collect();
        let mut scalar_reports = vec![ProcessReport::default(); lanes];
        for (track, effect) in scalar.iter_mut().enumerate() {
            scalar_reports[track] = effect.process(
                EffectProcessBlock::new(
                    &mut scalar_left[track],
                    &mut scalar_right[track],
                    None,
                    0,
                    core::slice::from_ref(&spans[track]),
                    128,
                )
                .expect("scalar block"),
            );
        }
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                frames as u32,
                width,
                0,
                &spans,
                &offsets,
                128,
            )
            .expect("bank block"),
        );
        assert_eq!(&report.reports[..lanes], &scalar_reports[..]);
        for frame in 0..frames {
            for track in 0..lanes {
                assert_eq!(
                    bank_left[frame * lanes + track].to_bits(),
                    scalar_left[track][frame].to_bits(),
                    "left {link_mode:?} frame={frame} track={track}"
                );
                assert_eq!(
                    bank_right[frame * lanes + track].to_bits(),
                    scalar_right[track][frame].to_bits(),
                    "right {link_mode:?} frame={frame} track={track}"
                );
            }
        }
        for (track, scalar) in scalar.iter().enumerate() {
            assert_eq!(
                bank_snapshot(bank.as_ref(), track as u32, scalar.metadata().state_sizes),
                snapshot(scalar.as_ref()),
                "state {link_mode:?} track={track}"
            );
        }
    }
}

/// A track's snapshot, restore and both resets touch that track only.
///
/// Red mutation: `replace_lane` writing lane 0 instead of `lane` — the restored track's peers move.
#[test]
fn bank_snapshot_restore_and_resets_are_track_local() {
    let Some((_, width)) = native_bank() else {
        println!("no bank width on this build; skipping");
        return;
    };
    let lanes = width.lanes() as usize;
    let values = track_values(lanes);
    let mut bank = bind_native_bank(&values, LinkMode::DualMono).expect("bank");
    let mut scalar = values
        .iter()
        .map(|values| prepare_with(values, 48_000, false, LinkMode::DualMono))
        .collect::<Vec<_>>();
    let sizes = scalar[0].metadata().state_sizes;

    let frames = 40;
    let mut bank_left = vec![0.0_f32; frames * lanes];
    let mut bank_right = vec![0.0_f32; frames * lanes];
    for frame in 0..frames {
        for track in 0..lanes {
            bank_left[frame * lanes + track] = 0.6 - 0.01 * frame as f32 + 0.02 * track as f32;
            bank_right[frame * lanes + track] = -0.3 + 0.005 * frame as f32;
        }
    }
    let offsets = vec![0_u32; lanes + 1];
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut bank_left,
            &mut bank_right,
            None,
            frames as u32,
            width,
            0,
            &[],
            &offsets,
            128,
        )
        .expect("bank block"),
    );
    for (track, effect) in scalar.iter_mut().enumerate() {
        let mut left = (0..frames)
            .map(|f| 0.6 - 0.01 * f as f32 + 0.02 * track as f32)
            .collect::<Vec<_>>();
        let mut right = (0..frames)
            .map(|f| -0.3 + 0.005 * f as f32)
            .collect::<Vec<_>>();
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("scalar"),
        );
    }

    let peers: Vec<_> = (0..lanes)
        .map(|track| bank_snapshot(bank.as_ref(), track as u32, sizes))
        .collect();
    let target = lanes - 3;
    let saved = peers[target].clone();
    bank.reset(ResetKind::DiscontinuityKeepParameters);
    bank.restore_track_state_payload(
        target as u32,
        1,
        StatePayloadInput::new(&[], &saved.0, &saved.1, sizes).expect("saved payload"),
    )
    .expect("track restore");
    assert_eq!(bank_snapshot(bank.as_ref(), target as u32, sizes), saved);
    for (track, peer) in peers.iter().enumerate() {
        if track == target {
            continue;
        }
        let after = bank_snapshot(bank.as_ref(), track as u32, sizes);
        assert_eq!(
            state_f32(&after.0, 0).to_bits(),
            0.0_f32.to_bits(),
            "peer {track} must still be reset"
        );
        assert_ne!(&after, peer, "peer {track} was reset, not restored");
    }

    bank.reset(ResetKind::FullToDefaults);
    for (track, scalar) in scalar.iter_mut().enumerate() {
        scalar.reset(ResetKind::FullToDefaults);
        assert_eq!(
            bank_snapshot(bank.as_ref(), track as u32, sizes),
            snapshot(scalar.as_ref()),
            "full reset track={track}"
        );
    }

    // Out-of-range tracks are rejected, not wrapped.
    assert!(
        bank.restore_track_state_payload(
            lanes as u32,
            1,
            StatePayloadInput::new(&[], &saved.0, &saved.1, sizes).expect("payload"),
        )
        .is_err()
    );
}
