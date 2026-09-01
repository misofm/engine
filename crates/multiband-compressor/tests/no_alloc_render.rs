//! E6: the render path, both resets and both state-payload directions allocate nothing.
//!
//! Version 1's `reset(FullToDefaults)` was `*self = Self::new(..)`, which freed three boxed rings
//! per lane and allocated three more — sixteen lanes' worth for a W8 bank — and its restore
//! replaced the boxes instead of writing into them (#94 F9). `reset` is a `PreparedNativeEffect`
//! method owned by the realtime plane, so that was a latent violation of the AGENTS.md rule that
//! render performs no allocation. Everything below now allocates only in `prepare`.

#![allow(unsafe_code)]

mod support;

use core::alloc::Layout;
use core::cell::Cell;
use std::alloc::{GlobalAlloc, System};

use effect_contract::{
    BankWidth, EffectBankProcessBlock, LinkMode, NativeEffectFactory, ParameterChannel, ResetKind,
    StatePayloadInput, StatePayloadOutput,
};
use multiband_compressor::MultibandCompressorFactory;
use support::{new_sections, point, process, request_with, varied_values};

struct TrackingAllocator;

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static EVENTS: Cell<u64> = const { Cell::new(0) };
}

fn record() {
    ACTIVE.with(|active| {
        if active.get() {
            EVENTS.with(|events| events.set(events.get() + 1));
        }
    });
}

// SAFETY: every operation delegates its original pointer and layout unchanged to `System`. The
// thread-local counter is observational and is active only around one call on one test thread.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record();
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record();
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        record();
        // SAFETY: delegates the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegates the original pointer, layout and requested size unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            record();
        }
        replacement
    }
}

/// Counts allocator events during `operation`.
fn events(operation: impl FnOnce()) -> u64 {
    EVENTS.with(|events| events.set(0));
    ACTIVE.with(|active| active.set(true));
    operation();
    ACTIVE.with(|active| active.set(false));
    EVENTS.with(Cell::get)
}

#[test]
fn the_scalar_render_path_allocates_nothing() {
    let initial = varied_values(1);
    let mut effect = MultibandCompressorFactory
        .prepare(request_with(&initial, LinkMode::Maximum, 128, false))
        .expect("prepare");
    let sizes = effect.metadata().state_sizes;
    let mut left = support::signal(128, 0x0BAD_C0DE);
    let mut right = support::signal(128, 0x0BAD_BEEF);
    let spans = [point(2, ParameterChannel::Left, 0, -30.0)];
    // Warm the allocator's own lazy state outside the measured region.
    process(effect.as_mut(), &mut left, &mut right, 0, &spans, 128);
    let mut sections = new_sections(sizes);

    let counted = events(|| {
        process(effect.as_mut(), &mut left, &mut right, 128, &spans, 128);
        effect.reset(ResetKind::DiscontinuityKeepParameters);
        process(effect.as_mut(), &mut left, &mut right, 256, &[], 128);
        effect.reset(ResetKind::FullToDefaults);
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
        effect
            .restore_state_payload(
                2,
                StatePayloadInput::new(&sections.0, &sections.1, &sections.2, sizes)
                    .expect("payload"),
            )
            .expect("restore");
    });
    assert_eq!(
        counted, 0,
        "the scalar render path allocated {counted} times"
    );
}

#[test]
fn the_bank_render_path_allocates_nothing() {
    for width in [BankWidth::Four, BankWidth::Eight] {
        let lanes = width.lanes() as usize;
        let sets = (0..lanes).map(varied_values).collect::<Vec<_>>();
        let requests = sets
            .iter()
            .map(|set| request_with(set, LinkMode::Average, 128, false))
            .collect::<Vec<_>>();
        let mut bank = support::bank(width, &requests);
        let sizes = MultibandCompressorFactory
            .prepare(requests[0])
            .expect("scalar")
            .metadata()
            .state_sizes;
        let mut left = support::signal(128 * lanes, 0x00C0_FFEE);
        let mut right = support::signal(128 * lanes, 0x00DE_CAF0);
        let offsets = vec![0u32; lanes + 1];
        let mut sections = new_sections(sizes);
        let run = |bank: &mut dyn effect_contract::PreparedNativeEffectBank,
                   left: &mut [f32],
                   right: &mut [f32],
                   first: u64| {
            bank.process_bank(
                EffectBankProcessBlock::new(
                    left,
                    right,
                    None,
                    128,
                    width,
                    first,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
        };
        run(bank.as_mut(), &mut left, &mut right, 0);

        let counted = events(|| {
            run(bank.as_mut(), &mut left, &mut right, 128);
            bank.reset(ResetKind::DiscontinuityKeepParameters);
            bank.reset(ResetKind::FullToDefaults);
            run(bank.as_mut(), &mut left, &mut right, 256);
            bank.snapshot_track_state_payload(
                1,
                StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                    .expect("payload"),
            )
            .expect("snapshot");
            bank.restore_track_state_payload(
                1,
                2,
                StatePayloadInput::new(&sections.0, &sections.1, &sections.2, sizes)
                    .expect("payload"),
            )
            .expect("restore");
        });
        assert_eq!(counted, 0, "{width:?} allocated {counted} times");
    }
}
