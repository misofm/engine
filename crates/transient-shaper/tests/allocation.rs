//! The render path allocates nothing (master plan §10 A1, `AGENTS.md` render rules).
//!
//! A counting global allocator is armed only around the render loop, so preparation — which does
//! allocate, once, for the boxed product — is outside the measurement. Automation is applied on
//! every block, so the ramp retarget path is inside it too.

#![allow(unsafe_code)]

mod common;

use core::alloc::Layout;
use core::cell::Cell;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::alloc::{GlobalAlloc, System};

use common::*;
use effect_contract::{EffectBankProcessBlock, EffectProcessBlock, LinkMode, ParameterChannel};

thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

/// A zero-filled offset table for the blocks that carry no automation, allocated as a `const` so
/// slicing it inside the armed region cannot allocate.
const ZERO_OFFSETS: [u32; 9] = [0; 9];

struct CountingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

fn record(counter: &'static std::thread::LocalKey<Cell<u64>>) {
    // Allocations can occur while a thread-local is being destroyed. `try_with` makes the
    // allocator observational even during teardown instead of panicking recursively.
    if ARMED.try_with(Cell::get).unwrap_or(false) {
        let _ = counter.try_with(|count| count.set(count.get() + 1));
    }
}

// SAFETY: every request is forwarded to `System` with its original pointer, layout and size; the
// thread-local counters are observational and armed only on the thread running `measure`.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwards the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record(&ALLOCATIONS);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwards the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record(&ALLOCATIONS);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        record(&DEALLOCATIONS);
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: forwards the original allocation arguments unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, size) };
        if !replacement.is_null() {
            record(&ALLOCATIONS);
            record(&DEALLOCATIONS);
        }
        replacement
    }
}

fn measure(body: impl FnOnce()) -> (u64, u64) {
    struct Disarm;

    impl Drop for Disarm {
        fn drop(&mut self) {
            let _ = ARMED.try_with(|armed| armed.set(false));
        }
    }

    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ARMED.set(true);
    let disarm = Disarm;
    body();
    drop(disarm);
    (ALLOCATIONS.get(), DEALLOCATIONS.get())
}

fn allocate_and_free_once() {
    let layout = Layout::new::<[u8; 32]>();
    // SAFETY: the non-zero layout is used for exactly one allocation, the returned pointer is
    // checked before use, and the same pointer and layout are passed once to `dealloc`.
    unsafe {
        let pointer = std::alloc::alloc(layout);
        assert!(!pointer.is_null(), "positive-control allocation failed");
        pointer.write(0x5a);
        std::hint::black_box(pointer);
        std::alloc::dealloc(pointer, layout);
    }
}

#[test]
fn the_counter_observes_same_thread_allocation_and_free() {
    assert_eq!(measure(allocate_and_free_once), (1, 1));
}

#[test]
fn foreign_thread_allocations_do_not_enter_the_callers_counts() {
    let ready = AtomicBool::new(false);
    let start = AtomicBool::new(false);
    let caller_is_measuring = AtomicBool::new(false);
    let finished = AtomicBool::new(false);
    let foreign_started_during_measurement = AtomicBool::new(false);
    let foreign_finished_during_measurement = AtomicBool::new(false);
    let foreign_allocations = AtomicU64::new(0);
    let foreign_deallocations = AtomicU64::new(0);

    std::thread::scope(|scope| {
        let worker = scope.spawn(|| {
            ready.store(true, Ordering::Release);
            while !start.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
            foreign_started_during_measurement.store(
                caller_is_measuring.load(Ordering::Acquire),
                Ordering::Release,
            );
            let (allocations, deallocations) = measure(allocate_and_free_once);
            foreign_finished_during_measurement.store(
                caller_is_measuring.load(Ordering::Acquire),
                Ordering::Release,
            );
            foreign_allocations.store(allocations, Ordering::Release);
            foreign_deallocations.store(deallocations, Ordering::Release);
            finished.store(true, Ordering::Release);
        });

        while !ready.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }
        let caller_counts = measure(|| {
            caller_is_measuring.store(true, Ordering::Release);
            start.store(true, Ordering::Release);
            while !finished.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
            caller_is_measuring.store(false, Ordering::Release);
        });
        worker.join().expect("foreign allocation worker");

        assert!(
            foreign_started_during_measurement.load(Ordering::Acquire),
            "foreign allocation did not begin during the caller's measurement"
        );
        assert!(
            foreign_finished_during_measurement.load(Ordering::Acquire),
            "foreign allocation did not finish during the caller's measurement"
        );
        assert_eq!(
            foreign_allocations.load(Ordering::Acquire),
            1,
            "foreign positive control must allocate once"
        );
        assert_eq!(
            foreign_deallocations.load(Ordering::Acquire),
            1,
            "foreign positive control must free once"
        );
        assert_eq!(
            caller_counts,
            (0, 0),
            "foreign allocation contaminated the caller's counters"
        );
    });
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

    let (allocations, deallocations) = measure(|| {
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
    });

    assert_eq!(allocations, 0, "the render path must not allocate");
    assert_eq!(deallocations, 0, "the render path must not free");
}
