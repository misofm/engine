//! The render path allocates nothing (master plan §10 A1, `AGENTS.md` render rules).
//!
//! A counting global allocator is armed only around the render loop, so preparation — which does
//! allocate, once, for the boxed product — is outside the measurement. Automation is applied on
//! every block, so the ramp retarget path is inside it too.

#![allow(unsafe_code)]

mod common;

use core::alloc::Layout;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::alloc::{GlobalAlloc, System};

use common::*;
use effect_contract::{EffectBankProcessBlock, EffectProcessBlock, LinkMode, ParameterChannel};

static ARMED: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);

/// A zero-filled offset table for the blocks that carry no automation, allocated as a `const` so
/// slicing it inside the armed region cannot allocate.
const ZERO_OFFSETS: [u32; 9] = [0; 9];

struct CountingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

// SAFETY: every request is forwarded to `System` unchanged; the only added work is two relaxed
// atomic counters, which are read by the test after the render loop and never affect the
// allocation itself.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ARMED.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if ARMED.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if ARMED.load(Ordering::Relaxed) {
            DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if ARMED.load(Ordering::Relaxed) {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

/// Red mutation: a `Vec::with_capacity(1)` anywhere inside `Shaper::process_block`.
#[test]
fn the_render_path_allocates_nothing() {
    let blocks = 1_000;
    let frames = 128;
    let values = values_of(0.5, -0.25, 0.75);

    // Everything the loop touches is allocated before the counters are armed.
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

    ARMED.store(true, Ordering::Relaxed);
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
    ARMED.store(false, Ordering::Relaxed);

    assert_eq!(
        ALLOCATIONS.load(Ordering::Relaxed),
        0,
        "the render path must not allocate"
    );
    assert_eq!(
        DEALLOCATIONS.load(Ordering::Relaxed),
        0,
        "the render path must not free"
    );
}
