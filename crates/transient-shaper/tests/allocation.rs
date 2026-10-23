//! The render path allocates nothing (master plan §10 A1, `AGENTS.md` render rules).
//!
//! The workspace's audited global allocator (`bench_support::alloc`) counts every allocation and
//! free per thread. The render loop is bracketed by a snapshot of this thread's counters, so
//! preparation -- which does allocate, once, for the boxed product -- is outside the measurement,
//! and no other test thread can enter it. Sample-zero points are offered every fourth block:
//! the first batch is accepted, later batches are rejected because their start sample is stale.
//! Accepted retargeting and rejection are both measured. The counters' own positive controls (same-thread allocation
//! and free are counted; a foreign thread's are not) are `bench-support`'s tests.

mod common;

use bench_support::alloc as bench_alloc;
use common::*;
use effect_contract::{EffectBankProcessBlock, EffectProcessBlock, LinkMode, ParameterChannel};

/// A zero-filled offset table for the blocks that carry no automation, allocated as a `const` so
/// slicing it inside the measured region cannot allocate.
const ZERO_OFFSETS: [u32; 9] = [0; 9];

/// Red mutation: `std::hint::black_box(Vec::<u8>::with_capacity(1));` inside
/// `Shaper::process_block`, before or inside the silent fast path's early return.
#[test]
fn the_render_path_allocates_nothing() {
    let blocks = 1_000;
    let frames = 128;
    let values = values_of(0.5, -0.25, 0.75);

    // Everything the loop touches is allocated before the counters are read.
    let mut effect = prepare(&values);
    let mut left = vec![0.25_f32; frames];
    let mut right = vec![-0.125_f32; frames];
    let spans = [point(ParameterChannel::Left, 0, 0, 0.5)];

    let lanes = native_bank().map_or(0, |(_, width)| width.lanes() as usize);
    let bank_values = vec![values; lanes.max(1)];
    let mut bank = if lanes > 0 {
        bind_native_bank(&bank_values[..lanes], LinkMode::Maximum)
    } else {
        None
    };
    let mut bank_left = vec![0.25_f32; frames * lanes.max(1)];
    let mut bank_right = vec![-0.125_f32; frames * lanes.max(1)];
    let bank_spans: Vec<_> = (0..lanes)
        .map(|track| point(ParameterChannel::Left, 0, 0, -0.5 + track as f32 * 0.125))
        .collect();
    let offsets: Vec<u32> = (0..=lanes as u32).collect();
    let width = native_bank().map_or(effect_contract::BankWidth::Four, |(_, w)| w);

    bench_alloc::assert_installed();
    let mark = bench_alloc::current_thread_counters();
    for block in 0..blocks {
        let first = (block * frames) as u64;
        let spans: &[_] = if block % 4 == 0 { &spans } else { &[] };
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, first, spans, frames as u32)
                .expect("scalar block"),
        );
        if let Some(bank) = bank.as_mut() {
            let (spans, offsets): (&[_], &[u32]) = if block % 4 == 0 {
                (&bank_spans, &offsets)
            } else {
                (&[], &ZERO_OFFSETS[..lanes + 1])
            };
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut bank_left,
                    &mut bank_right,
                    None,
                    frames as u32,
                    width,
                    first,
                    spans,
                    offsets,
                    frames as u32,
                )
                .expect("bank block"),
            );
        }
    }
    // Issue #1746 gate 3: a silent stretch, rendered until every lane of the bank is at rest and
    // then on, so the silent fast path's blocks (the rest query and the skip) are measured too.
    // The fast path's taking is the crate's own `rest_tests`' claim; here the rest is the reach.
    left.fill(0.0);
    right.fill(0.0);
    bank_left.fill(0.0);
    bank_right.fill(0.0);
    let all = (1_u32 << lanes) - 1;
    let mut rest_blocks = 0;
    for block in blocks..blocks + 20_000 {
        let first = (block * frames) as u64;
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, first, &[], frames as u32)
                .expect("scalar block"),
        );
        let Some(bank) = bank.as_mut() else {
            continue;
        };
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                frames as u32,
                width,
                first,
                &[],
                &ZERO_OFFSETS[..lanes + 1],
                frames as u32,
            )
            .expect("bank block"),
        );
        if bank.lanes_at_rest() == all {
            rest_blocks += 1;
            if rest_blocks == 64 {
                break;
            }
        }
    }
    let counted = bench_alloc::current_thread_delta_since(mark);

    if bank.is_some() {
        assert_eq!(
            rest_blocks, 64,
            "the silent stretch never brought the bank to rest"
        );
    }
    assert_eq!(counted.allocations, 0, "the render path must not allocate");
    assert_eq!(counted.deallocations, 0, "the render path must not free");
}
