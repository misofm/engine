//! E8 / A1 — the render path allocates nothing.
//!
//! `prepare` and `bind_homogeneous_bank` allocate (three histories, the ramp table, the boxed
//! instance) and that is the whole budget. After that, `process`, `process_bank`, `reset`,
//! `snapshot_*` and `restore_*` must not reach the allocator at all, because the render thread has
//! no lock, no syscall and no allocator (`AGENTS.md`, master plan §1.5).
//!
//! The tracking allocator below counts every allocation that happens while an explicit guard is
//! armed, so the count is attributed to the calls under test and not to the test harness.

#![allow(unsafe_code)]

mod support;

use core::alloc::Layout;
use core::cell::Cell;
use std::alloc::{GlobalAlloc, System};

use effect_contract::{BankWidth, ParameterChannel, ResetKind};
use support::{bank_available, initial_values, prepare, prepare_bank, process, process_bank};

thread_local! {
    /// Whether this thread is inside a measured region. `const`-initialised so that reading it
    /// from inside the allocator cannot itself allocate a lazy thread-local.
    static ARMED: Cell<bool> = const { Cell::new(false) };
    /// Allocations this thread made while armed.
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

struct CountingAllocator;

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

// SAFETY: every request is forwarded to the system allocator unchanged; the only side effect is a
// thread-local counter increment while this thread's guard is armed.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record();
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn record() {
    // `try_with` rather than `with`: a thread-local is unavailable during thread teardown, and an
    // allocation there must not panic inside the allocator.
    let armed = ARMED.try_with(Cell::get).unwrap_or(false);
    if armed {
        let _ = COUNT.try_with(|count| count.set(count.get() + 1));
    }
}

/// Runs `body` with this thread's allocation counter armed and returns how many allocations it
/// made. Counting is per thread, so the harness's other tests cannot contaminate the number.
fn count(body: impl FnOnce()) -> usize {
    COUNT.with(|count| count.set(0));
    ARMED.with(|armed| armed.set(true));
    body();
    ARMED.with(|armed| armed.set(false));
    COUNT.with(Cell::get)
}

#[test]
fn the_scalar_render_path_never_allocates() {
    let values = initial_values();
    let mut effect = prepare(&values);
    let mut left = vec![0.0_f32; 128];
    let mut right = vec![0.0_f32; 128];
    for (index, sample) in left.iter_mut().enumerate() {
        *sample = (index as f32 * 0.05).sin() * 0.8;
    }
    right.copy_from_slice(&left);
    let spans = [support::point(0, ParameterChannel::Left, 12.0, 0)];
    // Warm once outside the guard so nothing lazily initialises inside it.
    process(effect.as_mut(), &mut left, &mut right, 0, &spans);

    let sizes = effect.metadata().state_sizes;
    let mut common = vec![0_u8; sizes.common_bytes as usize];
    let mut snapshot_left = vec![0_u8; sizes.left_bytes as usize];
    let mut snapshot_right = vec![0_u8; sizes.right_bytes as usize];

    let allocations = count(|| {
        let mut first_sample = 128_u64;
        for round in 0..1_000 {
            let automation: &[_] = if round % 128 == 0 { &spans } else { &[] };
            process(
                effect.as_mut(),
                &mut left,
                &mut right,
                first_sample,
                automation,
            );
            first_sample += 128;
        }
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        effect.reset(ResetKind::FullToDefaults);
        effect
            .snapshot_state_payload(
                effect_contract::StatePayloadOutput::new(
                    &mut common,
                    &mut snapshot_left,
                    &mut snapshot_right,
                    sizes,
                )
                .expect("sizes"),
            )
            .expect("snapshot");
        effect
            .restore_state_payload(
                2,
                effect_contract::StatePayloadInput {
                    common: &common,
                    left: &snapshot_left,
                    right: &snapshot_right,
                },
            )
            .expect("restore");
    });
    assert_eq!(allocations, 0, "render path allocated {allocations} times");
}

#[test]
fn the_bank_render_path_never_allocates() {
    let width = if bank_available(BankWidth::Eight) {
        BankWidth::Eight
    } else if bank_available(BankWidth::Four) {
        BankWidth::Four
    } else {
        return;
    };
    let lanes = width.lanes() as usize;
    let values = initial_values();
    let per_lane: Vec<Vec<_>> = (0..lanes).map(|_| values.to_vec()).collect();
    let mut bank = prepare_bank(width, &per_lane).expect("bank binds");
    let mut left = vec![0.0_f32; 128 * lanes];
    let mut right = vec![0.0_f32; 128 * lanes];
    for (index, sample) in left.iter_mut().enumerate() {
        *sample = (index as f32 * 0.03).sin() * 0.7;
    }
    right.copy_from_slice(&left);
    let offsets = vec![0_u32; lanes + 1];
    process_bank(
        bank.as_mut(),
        width,
        &mut left,
        &mut right,
        128,
        0,
        &[],
        &offsets,
    );

    let allocations = count(|| {
        let mut first_sample = 128_u64;
        for _ in 0..1_000 {
            process_bank(
                bank.as_mut(),
                width,
                &mut left,
                &mut right,
                128,
                first_sample,
                &[],
                &offsets,
            );
            first_sample += 128;
        }
        bank.reset(ResetKind::DiscontinuityKeepParameters);
    });
    assert_eq!(allocations, 0, "bank path allocated {allocations} times");
}

/// Preparation is where the budget is spent, and it is bounded and small.
#[test]
fn preparation_allocates_a_bounded_number_of_times() {
    let values = initial_values();
    let allocations = count(|| {
        let _effect = prepare(&values);
    });
    let again = count(|| {
        let _effect = prepare(&values);
    });
    assert_eq!(allocations, again, "preparation is not deterministic");
    assert!(
        (1..=32).contains(&allocations),
        "prepare made {allocations} allocations"
    );
    println!("issue_091_prepare allocations={allocations}");
}
