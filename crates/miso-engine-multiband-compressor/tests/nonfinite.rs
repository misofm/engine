//! E8: a non-finite block is caught once, at the boundary, and the next block starts clean.
//!
//! Decision D7 deletes every per-value `is_finite`, `sanitize` and `recover` from this crate;
//! version 1 had about seventy-six of them per stereo frame (#94 F5). What replaces them is
//! `miso_engine_effect_runtime::bank`'s once-per-block scan: a block whose *output* leaves the
//! bounds is zeroed, the state is reset and the failure is counted.
//!
//! The declared latency is `Fs/50`, so a bad input sample is not a bad output sample in the same
//! block: it reaches the output 960 samples later at 48 kHz. That is the whole reason the check
//! belongs at the output and not at the input, and it is why these gates run enough blocks for the
//! fault to arrive.

mod support;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, LinkMode, NativeEffectFactory,
    PreparedNativeEffect,
};
use miso_engine_multiband_compressor::MultibandCompressorFactory;
use support::{request_with, snapshot, varied_values};

const FRAMES: usize = 128;
const LATENCY: usize = 960;

fn prepared() -> Box<dyn PreparedNativeEffect> {
    let initial = varied_values(1);
    MultibandCompressorFactory
        .prepare(request_with(
            &initial,
            LinkMode::DualMono,
            FRAMES as u32,
            false,
        ))
        .expect("prepare")
}

/// Runs `blocks` blocks of the seeded signal, injecting `injected` at absolute sample `at`, and
/// returns the rendered PCM together with the per-block recovery counts.
fn run(injected: Option<(usize, f32)>, blocks: usize) -> (Vec<f32>, Vec<u64>) {
    let mut effect = prepared();
    let mut left = support::signal(blocks * FRAMES, 0x0101_0101);
    let mut right = support::signal(blocks * FRAMES, 0x0202_0202);
    if let Some((at, value)) = injected {
        left[at] = value;
    }
    let mut recovered = Vec::with_capacity(blocks);
    for block in 0..blocks {
        let start = block * FRAMES;
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[start..start + FRAMES],
                &mut right[start..start + FRAMES],
                None,
                start as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        recovered.push(report.recovered_left_samples);
    }
    (left, recovered)
}

/// Exactly one block is rejected, it is zeroed on both channels, and its lane is attributed.
#[test]
fn a_nonfinite_block_is_zeroed_reset_and_counted() {
    const AT: usize = 37;
    const BLOCKS: usize = 12;
    // `1e30` is deliberately absent: the compressor's own gain reduction clamps at -100 dB, so a
    // lone `1e30` impulse leaves the output at about `1e25` and is *not* a fault. The boundary
    // check exists for a diverging recurrence, not for a loud sample.
    for injected in [f32::NAN, f32::INFINITY, -1.0e31] {
        let (left, recovered) = run(Some((AT, injected)), BLOCKS);
        let failing = (AT + LATENCY) / FRAMES;
        assert_eq!(
            recovered.iter().filter(|count| **count != 0).count(),
            1,
            "{injected}: expected exactly one rejected block, got {recovered:?}"
        );
        assert_eq!(
            recovered[failing], FRAMES as u64,
            "{injected}: the failing block is attributed its whole frame count"
        );
        for (offset, sample) in left[failing * FRAMES..(failing + 1) * FRAMES]
            .iter()
            .enumerate()
        {
            assert_eq!(
                sample.to_bits(),
                0u32,
                "{injected}: frame {offset} of the rejected block"
            );
        }
        assert!(
            left[(failing + 1) * FRAMES..]
                .iter()
                .all(|sample| sample.is_finite()),
            "{injected}: nothing survives the reset"
        );
    }
}

/// After a rejection the effect is exactly a freshly prepared one.
#[test]
fn recovery_restores_a_fresh_instance() {
    const BLOCKS: usize = 12;
    let mut faulted = prepared();
    let mut fresh = prepared();
    let mut left = support::signal(BLOCKS * FRAMES, 0x0505_0505);
    let mut right = support::signal(BLOCKS * FRAMES, 0x0606_0606);
    left[37] = f32::NAN;
    let failing = (37 + LATENCY) / FRAMES;
    for block in 0..=failing {
        let start = block * FRAMES;
        faulted.process(
            EffectProcessBlock::new(
                &mut left[start..start + FRAMES],
                &mut right[start..start + FRAMES],
                None,
                start as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
    }
    assert_eq!(
        snapshot(faulted.as_ref()),
        snapshot(fresh.as_ref()),
        "the reset must leave exactly a prepared instance"
    );

    // And the block after the rejection is what the fresh instance renders from the same input.
    let start = (failing + 1) * FRAMES;
    let mut tail_left = support::signal(BLOCKS * FRAMES, 0x0707_0707)[..FRAMES].to_vec();
    let mut tail_right = support::signal(BLOCKS * FRAMES, 0x0808_0808)[..FRAMES].to_vec();
    let mut fresh_left = tail_left.clone();
    let mut fresh_right = tail_right.clone();
    faulted.process(
        EffectProcessBlock::new(
            &mut tail_left,
            &mut tail_right,
            None,
            start as u64,
            &[],
            FRAMES as u32,
        )
        .expect("block"),
    );
    fresh.process(
        EffectProcessBlock::new(
            &mut fresh_left,
            &mut fresh_right,
            None,
            0,
            &[],
            FRAMES as u32,
        )
        .expect("block"),
    );
    for frame in 0..FRAMES {
        assert_eq!(
            tail_left[frame].to_bits(),
            fresh_left[frame].to_bits(),
            "frame {frame} after recovery"
        );
    }
}

/// A level below the shared limit is accepted; one lane's failure zeroes the whole bank block.
#[test]
fn the_boundary_is_the_shared_limit_and_a_bank_shares_its_reset() {
    const BLOCKS: usize = 12;
    let mut effect = prepared();
    let mut left = vec![1.0e29f32; BLOCKS * FRAMES];
    let mut right = vec![1.0e29f32; BLOCKS * FRAMES];
    for block in 0..BLOCKS {
        let start = block * FRAMES;
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[start..start + FRAMES],
                &mut right[start..start + FRAMES],
                None,
                start as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        assert_eq!(
            report.recovered_left_samples, 0,
            "1e29 is below the 1e30 limit and must pass (block {block})"
        );
    }

    let sets = (0..8).map(varied_values).collect::<Vec<_>>();
    let requests = sets
        .iter()
        .map(|set| request_with(set, LinkMode::DualMono, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let mut bank = support::bank(BankWidth::Eight, &requests);
    let mut left = support::signal(FRAMES * 8 * BLOCKS, 0x1111_2222);
    let mut right = support::signal(FRAMES * 8 * BLOCKS, 0x3333_4444);
    left[5 * 8 + 3] = f32::NAN;
    let failing = (5 + LATENCY) / FRAMES;
    for block in 0..BLOCKS {
        let start = block * FRAMES * 8;
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[start..start + FRAMES * 8],
                &mut right[start..start + FRAMES * 8],
                None,
                FRAMES as u32,
                BankWidth::Eight,
                (block * FRAMES) as u64,
                &[],
                &[0u32; 9],
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        if block != failing {
            assert!(
                report
                    .reports
                    .iter()
                    .all(|track| track.recovered_left_samples == 0),
                "block {block} should be clean"
            );
            continue;
        }
        assert!(
            left[start..start + FRAMES * 8]
                .iter()
                .chain(right[start..start + FRAMES * 8].iter())
                .all(|sample| *sample == 0.0),
            "a bank shares its reset, so a failing lane zeroes the whole block"
        );
        assert_eq!(report.reports[3].recovered_left_samples, FRAMES as u64);
        for (track, item) in report.reports.iter().enumerate() {
            if track != 3 {
                assert_eq!(
                    item.recovered_left_samples, 0,
                    "track {track} was not the failing lane"
                );
            }
        }
    }
}
