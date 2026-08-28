//! Partition invariance (master plan §10 P1): a block boundary must not be observable.
//!
//! The two places a boundary could show are the D11 ramp — whose iterated additions and exact snap
//! `LinearRamp::next_value` performs identically however the frames are grouped — and the ramp
//! prefix, which is recomputed per block from the ramps' own `remaining`.

mod common;

use common::*;
use miso_engine_effect_contract::{
    EffectBankProcessBlock, EffectProcessBlock, LinkMode, NativeEffectFactory, ParameterChannel,
};
use miso_engine_transient_shaper::TransientShaperFactory;

/// Deterministic, integer-built noise with impulses; no platform math anywhere in the signal.
fn signal(seed: u64, frames: usize) -> Vec<f32> {
    let mut state = seed | 1;
    (0..frames)
        .map(|index| {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let draw = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            if index % 128 == 0 {
                return 0.95;
            }
            ((draw >> 40) as i32 - 8_388_608 / 2) as f32 / 16_777_216.0
        })
        .collect()
}

const PARTITIONS: [usize; 5] = [1, 7, 64, 128, 512];

/// One render's result: the two output channels and the state payload sections it left behind.
type Rendered = (Vec<f32>, Vec<f32>, (Vec<u8>, Vec<u8>));

/// One bank render's result: the two interleaved output blocks and one payload pair per track.
type RenderedBank = (Vec<f32>, Vec<f32>, Vec<(Vec<u8>, Vec<u8>)>);

/// Red mutation: hoist `ramp_prefix` out of `run` and compute it once at prepare — the 7-frame and
/// 64-frame partitions then evaluate the ramp differently from the one-shot render.
#[test]
fn the_scalar_product_is_partition_invariant() {
    let frames = 512;
    let left = signal(0x1234_5678, frames);
    let right = signal(0x9abc_def0, frames);
    let spans = [
        point(ParameterChannel::Left, 0, 0, 0.875),
        point(ParameterChannel::Right, 0, 0, -0.375),
        point(ParameterChannel::Left, 1, 0, -0.625),
        point(ParameterChannel::Right, 1, 0, 0.25),
        point(ParameterChannel::Left, 2, 0, 0.5),
        point(ParameterChannel::Right, 2, 0, 0.75),
    ];

    let render = |block: usize| -> Rendered {
        let mut effect = TransientShaperFactory
            .prepare(request_full(
                &values_of(-0.5, 0.75, 1.0),
                48_000,
                512,
                false,
                LinkMode::DualMono,
            ))
            .expect("prepare");
        let mut out_left = left.clone();
        let mut out_right = right.clone();
        let mut start = 0;
        while start < frames {
            let end = (start + block).min(frames);
            let spans: &[_] = if start == 0 { &spans } else { &[] };
            let mut chunk_left = out_left[start..end].to_vec();
            let mut chunk_right = out_right[start..end].to_vec();
            effect.process(
                EffectProcessBlock::new(
                    &mut chunk_left,
                    &mut chunk_right,
                    None,
                    start as u64,
                    spans,
                    512,
                )
                .expect("partition block"),
            );
            out_left[start..end].copy_from_slice(&chunk_left);
            out_right[start..end].copy_from_slice(&chunk_right);
            start = end;
        }
        let state = snapshot(effect.as_ref());
        (out_left, out_right, state)
    };

    let reference = render(frames);
    // Non-vacuity: the effect must actually have shaped the signal, or every partition agrees on
    // having done nothing.
    let changed = reference
        .0
        .iter()
        .zip(&left)
        .filter(|(a, b)| a.to_bits() != b.to_bits())
        .count();
    assert!(
        changed > frames / 2,
        "only {changed} of {frames} samples moved"
    );
    for block in PARTITIONS {
        let candidate = render(block);
        assert_eq!(
            candidate.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            reference.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            "left PCM at block size {block}"
        );
        assert_eq!(
            candidate.1.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            reference.1.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            "right PCM at block size {block}"
        );
        assert_eq!(candidate.2, reference.2, "state at block size {block}");
    }
}

/// The same property at the build's bank width, where the ramp prefix is the maximum over lanes.
#[test]
fn the_bank_is_partition_invariant() {
    let Some((_, width)) = native_bank() else {
        println!("no bank width on this build; skipping");
        return;
    };
    let lanes = width.lanes() as usize;
    let frames = 512;
    let planar: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
        .map(|track| {
            (
                signal(0x1000 + track as u64 * 17, frames),
                signal(0x2000 + track as u64 * 23, frames),
            )
        })
        .collect();
    let values: Vec<_> = (0..lanes)
        .map(|track| values_of(-0.5 + track as f32 * 0.1, 0.75 - track as f32 * 0.1, 1.0))
        .collect();
    let spans: Vec<_> = (0..lanes)
        .map(|track| point(ParameterChannel::Left, 0, 0, -0.875 + track as f32 * 0.125))
        .collect();
    let offsets: Vec<u32> = (0..=lanes as u32).collect();

    let render = |block: usize| -> RenderedBank {
        let mut bank = bind_native_bank_quantum(&values, LinkMode::Maximum, 512).expect("bank");
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        for frame in 0..frames {
            for track in 0..lanes {
                left[frame * lanes + track] = planar[track].0[frame];
                right[frame * lanes + track] = planar[track].1[frame];
            }
        }
        let mut start = 0;
        while start < frames {
            let end = (start + block).min(frames);
            let (spans, offsets): (&[_], &[u32]) = if start == 0 {
                (&spans, &offsets)
            } else {
                (&[], &[0; 9][..lanes + 1])
            };
            let mut chunk_left = left[start * lanes..end * lanes].to_vec();
            let mut chunk_right = right[start * lanes..end * lanes].to_vec();
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut chunk_left,
                    &mut chunk_right,
                    None,
                    (end - start) as u32,
                    width,
                    start as u64,
                    spans,
                    offsets,
                    512,
                )
                .expect("bank partition block"),
            );
            left[start * lanes..end * lanes].copy_from_slice(&chunk_left);
            right[start * lanes..end * lanes].copy_from_slice(&chunk_right);
            start = end;
        }
        let sizes =
            miso_engine_transient_shaper::TRANSIENT_SHAPER_DESCRIPTOR.qualities[1].maximum_state;
        let states = (0..lanes)
            .map(|track| bank_snapshot(bank.as_ref(), track as u32, sizes))
            .collect();
        (left, right, states)
    };

    let reference = render(frames);
    let changed = (0..frames * lanes)
        .filter(|index| {
            reference.0[*index].to_bits() != planar[index % lanes].0[index / lanes].to_bits()
        })
        .count();
    assert!(
        changed > frames * lanes / 2,
        "only {changed} of {} bank samples moved",
        frames * lanes
    );
    for block in PARTITIONS {
        let candidate = render(block);
        assert_eq!(
            candidate.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            reference.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            "bank left PCM at block size {block}"
        );
        assert_eq!(
            candidate.1.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            reference.1.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
            "bank right PCM at block size {block}"
        );
        assert_eq!(candidate.2, reference.2, "bank state at block size {block}");
    }
}
