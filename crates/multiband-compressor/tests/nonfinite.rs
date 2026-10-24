//! E8: a non-finite block is caught once, at the boundary, and the next block starts clean.
//!
//! Decision D7 deletes every per-value `is_finite`, `sanitize` and `recover` from this crate;
//! version 1 had about seventy-six of them per stereo frame (#94 F5). What replaces them is
//! `effect_runtime::bank`'s once-per-block scan: a block whose *output* leaves the
//! bounds is zeroed, the state is reset and the failure is counted.
//!
//! The effect is causal with zero declared latency, so a bad input is attributed to the current
//! block. The once-per-block output boundary still owns the rejection, zeroing and reset.

mod support;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, LinkMode, NativeEffectFactory,
    PreparedEffect,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{request_with, snapshot, snapshot_track, varied_values};

const FRAMES: usize = 128;
/// A freshly prepared effect: its processor and the metadata its preparation returned.
fn prepared() -> PreparedEffect {
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
    let mut effect = prepared().processor;
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
        recovered.push(report.nonfinite_left_blocks);
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
        let failing = AT / FRAMES;
        assert_eq!(
            recovered.iter().filter(|count| **count != 0).count(),
            1,
            "{injected}: expected exactly one rejected block, got {recovered:?}"
        );
        assert_eq!(
            recovered[failing], 1,
            "{injected}: the failing block counts one block, not its frames"
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
    let mut faulted = prepared().processor;
    let fresh = prepared();
    let sizes = fresh.metadata.state_sizes;
    let mut fresh = fresh.processor;
    let mut left = support::signal(BLOCKS * FRAMES, 0x0505_0505);
    let mut right = support::signal(BLOCKS * FRAMES, 0x0606_0606);
    left[37] = f32::NAN;
    let failing = 37 / FRAMES;
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
        snapshot(faulted.as_ref(), sizes),
        snapshot(fresh.as_ref(), sizes),
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

/// A level below the shared limit is accepted.
#[test]
fn the_boundary_is_the_shared_limit() {
    const BLOCKS: usize = 12;
    let mut effect = prepared().processor;
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
            report.nonfinite_left_blocks, 0,
            "1e29 is below the 1e30 limit and must pass (block {block})"
        );
    }
}

/// One lane's failure in a bank is recovered on that lane alone (issue #1340, D1).
///
/// Lane 3 gets a `NaN` at frame 5 of block 0, then exact zeros on both channels for the rest of
/// the block (so its silence clock has run), and a threshold point whose 64-sample ramp is still
/// running when the 32-frame block fails. Its block is `+0.0` on both channels and charged once; right after it, its
/// state is exactly a fresh scalar instance's at the ramp's target, and its later blocks are
/// that instance's render. Every other lane is bit-identical to the same bank without the `NaN`.
#[test]
fn a_failing_bank_lane_recovers_alone() {
    const BLOCKS: usize = 6;
    // Blocks shorter than the 64-sample parameter ramp, so the point is still ramping when its
    // block fails.
    const BLOCK: usize = 32;
    const FAILING: usize = 3;
    const THRESHOLD: f32 = -30.0;
    // The bank runs at the build's own bank width (issue #1112).
    let width = BankWidth::for_backend(lane::Backend::current()).expect("a vector backend");
    let lanes = width.lanes() as usize;
    let sets = (0..lanes).map(varied_values).collect::<Vec<_>>();
    let requests = sets
        .iter()
        .map(|set| request_with(set, LinkMode::DualMono, FRAMES as u32, false))
        .collect::<Vec<_>>();
    let mut clean_left = support::signal(BLOCK * lanes * BLOCKS, 0x1111_2222);
    let mut clean_right = support::signal(BLOCK * lanes * BLOCKS, 0x3333_4444);
    for frame in 6..BLOCK {
        clean_left[frame * lanes + FAILING] = 0.0;
        clean_right[frame * lanes + FAILING] = 0.0;
    }
    let mut left = clean_left.clone();
    let mut right = clean_right.clone();
    left[5 * lanes + FAILING] = f32::NAN;
    // One point on the failing track's left low-band threshold, landing on block 0.
    let spans = [support::point(
        1,
        effect_contract::ParameterChannel::Left,
        0,
        THRESHOLD,
    )];
    let offsets = (0..=lanes)
        .map(|track| u32::from(track > FAILING))
        .collect::<Vec<_>>();
    // The reference: a fresh scalar instance of the failing track at the ramp's target.
    let mut target = sets[FAILING];
    target[2].value = THRESHOLD;
    let fresh = MultibandCompressorFactory
        .prepare(request_with(
            &target,
            LinkMode::DualMono,
            FRAMES as u32,
            false,
        ))
        .expect("prepare");
    let sizes = fresh.metadata.state_sizes;
    let mut fresh = fresh.processor;
    let mut faulted = support::bank(width, &requests).processor;
    let mut clean = support::bank(width, &requests).processor;
    for block in 0..BLOCKS {
        let start = block * BLOCK * lanes;
        let range = start..start + BLOCK * lanes;
        let block_spans: &[_] = if block == 0 { &spans } else { &[] };
        let block_offsets = if block == 0 {
            offsets.clone()
        } else {
            vec![0u32; lanes + 1]
        };
        let report = faulted.process_bank(
            EffectBankProcessBlock::new(
                &mut left[range.clone()],
                &mut right[range.clone()],
                None,
                BLOCK as u32,
                width,
                (block * BLOCK) as u64,
                block_spans,
                &block_offsets,
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        clean.process_bank(
            EffectBankProcessBlock::new(
                &mut clean_left[range.clone()],
                &mut clean_right[range],
                None,
                BLOCK as u32,
                width,
                (block * BLOCK) as u64,
                block_spans,
                &block_offsets,
                FRAMES as u32,
            )
            .expect("bank block"),
        );
        for (track, item) in report.reports.iter().enumerate() {
            let charged = u64::from(block == 0 && track == FAILING);
            assert_eq!(
                (item.nonfinite_left_blocks, item.nonfinite_right_blocks),
                (charged, 0),
                "block {block}, track {track}"
            );
        }
        if block == 0 {
            assert_eq!(
                snapshot_track(faulted.as_ref(), FAILING as u32, sizes),
                snapshot(fresh.as_ref(), sizes),
                "the recovered lane is exactly a fresh instance: filter, smoother, clock, ramps"
            );
        }
    }
    for (index, (sample, clean)) in left
        .iter()
        .zip(&clean_left)
        .chain(right.iter().zip(&clean_right))
        .enumerate()
    {
        if index % lanes != FAILING {
            assert_eq!(
                sample.to_bits(),
                clean.to_bits(),
                "word {index}: a neighbour of the failing lane moved"
            );
        }
    }
    let lane_of = |pcm: &[f32], block: usize| {
        (block * BLOCK..(block + 1) * BLOCK)
            .map(|frame| pcm[frame * lanes + FAILING])
            .collect::<Vec<_>>()
    };
    assert!(
        lane_of(&left, 0)
            .iter()
            .chain(&lane_of(&right, 0))
            .all(|sample| sample.to_bits() == 0),
        "the failing lane's block is +0.0 on both channels"
    );

    let (input_left, input_right) = (
        support::signal(BLOCK * lanes * BLOCKS, 0x1111_2222),
        support::signal(BLOCK * lanes * BLOCKS, 0x3333_4444),
    );
    for block in 1..BLOCKS {
        let mut fresh_left = lane_of(&input_left, block);
        let mut fresh_right = lane_of(&input_right, block);
        support::process(
            fresh.as_mut(),
            &mut fresh_left,
            &mut fresh_right,
            (block * BLOCK) as u64,
            &[],
            FRAMES as u32,
        );
        for (frame, (bank, alone)) in lane_of(&left, block)
            .iter()
            .zip(&fresh_left)
            .chain(lane_of(&right, block).iter().zip(&fresh_right))
            .enumerate()
        {
            assert_eq!(
                bank.to_bits(),
                alone.to_bits(),
                "block {block}, word {frame}: the recovered lane is not the fresh instance"
            );
        }
    }
}
