#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Public bank preparation and track-local state ownership. Both-width variable-block lane
//! identity and hostile recovery are covered in `src/padding_tests.rs`; the randomized public
//! full-bank differential also compares PCM, reports and every track's state.

mod common;

use common::*;
use effect_contract::{
    EffectBankProcessBlock, EffectProcessBlock, LinkMode, ResetKind, StatePayloadInput,
};

/// Per-track parameter sets for state ownership: every lane a different program point.
fn track_values(lanes: usize) -> Vec<[effect_contract::InitialParameterValue; 6]> {
    (0..lanes)
        .map(|track| {
            let attack = -0.75 + track as f32 * 0.2;
            let sustain = 0.6 - track as f32 * 0.15;
            values_of(attack, sustain, 0.25 + track as f32 * 0.1)
        })
        .collect()
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
    let prepared = values
        .iter()
        .map(|values| prepared_with(values, 48_000, false, LinkMode::DualMono))
        .collect::<Vec<_>>();
    let sizes = prepared[0].metadata.state_sizes;
    let mut scalar = prepared
        .into_iter()
        .map(|prepared| prepared.processor)
        .collect::<Vec<_>>();

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
    let saved = &peers[target];
    bank.reset(ResetKind::DiscontinuityKeepParameters);
    bank.restore_track_state_payload(
        target as u32,
        1,
        StatePayloadInput::new(&[], &saved.0, &saved.1, sizes).expect("saved payload"),
    )
    .expect("track restore");
    assert_eq!(&bank_snapshot(bank.as_ref(), target as u32, sizes), saved);
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
