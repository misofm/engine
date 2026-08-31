//! E9: the descriptive before/after measurement. Not a gate (AGENTS.md); run with `--ignored`.
//!
//! ```text
//! cargo test --release -p multiband-compressor \
//!     --test descriptive_frame_cost -- --ignored --nocapture
//! ```
//!
//! Version 1 measured 132.2 ns per stereo frame per track scalar and 134.9 ns at W8 — the bank was
//! *slower* than the scalar path (#94 F2). Nothing is hashed or checked inside the timed region.

mod support;

use std::time::Instant;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, LinkMode, NativeEffectFactory,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{request_with, varied_values};

const FRAMES: usize = 128;
const SECONDS: usize = 20;
const BLOCKS: usize = 48_000 * SECONDS / FRAMES;

#[test]
#[ignore = "descriptive measurement, not a gate"]
fn descriptive_frame_cost() {
    let initial = varied_values(3);
    let mut effect = MultibandCompressorFactory
        .prepare(request_with(
            &initial,
            LinkMode::Maximum,
            FRAMES as u32,
            false,
        ))
        .expect("prepare");
    let mut left = support::signal(FRAMES, 0x9999_1111);
    let mut right = support::signal(FRAMES, 0x9999_2222);
    let mut best = f64::INFINITY;
    for round in 0..3 {
        let start = Instant::now();
        for block in 0..BLOCKS {
            effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * FRAMES) as u64,
                    &[],
                    FRAMES as u32,
                )
                .expect("block"),
            );
        }
        let elapsed = start.elapsed().as_secs_f64();
        if round > 0 {
            best = best.min(elapsed / (BLOCKS * FRAMES) as f64 * 1.0e9);
        }
    }
    println!("scalar   {best:8.2} ns/frame/track   (version 1: 132.2)");

    for width in [BankWidth::Four, BankWidth::Eight] {
        let lanes = width.lanes() as usize;
        let sets = (0..lanes).map(varied_values).collect::<Vec<_>>();
        let requests = sets
            .iter()
            .map(|set| request_with(set, LinkMode::Maximum, FRAMES as u32, false))
            .collect::<Vec<_>>();
        let mut bank = support::bank(width, &requests);
        let mut left = support::signal(FRAMES * lanes, 0x9999_3333);
        let mut right = support::signal(FRAMES * lanes, 0x9999_4444);
        let offsets = vec![0u32; lanes + 1];
        let mut best = f64::INFINITY;
        for round in 0..3 {
            let start = Instant::now();
            for block in 0..BLOCKS {
                bank.process_bank(
                    EffectBankProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        FRAMES as u32,
                        width,
                        (block * FRAMES) as u64,
                        &[],
                        &offsets,
                        FRAMES as u32,
                    )
                    .expect("bank block"),
                );
            }
            let elapsed = start.elapsed().as_secs_f64();
            if round > 0 {
                best = best.min(elapsed / (BLOCKS * FRAMES * lanes) as f64 * 1.0e9);
            }
        }
        let reference = if lanes == 8 { "134.9" } else { "n/a" };
        println!("{width:?}  {best:8.2} ns/frame/track   (version 1: {reference})");
    }
}
