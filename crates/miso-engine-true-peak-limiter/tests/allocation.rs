//! A1: the render path allocates nothing.
//!
//! A counting global allocator, armed only around the calls under test, watches `process` and
//! `process_bank` over a hundred blocks that include automation, a boundary-check failure and both
//! resets. The limiter allocates at preparation and at restore — both control plane — and never
//! again; before #90 the render path was allocation-free too, and this gate is what keeps it so
//! now that the arena, the ramps and the payload codec all changed hands.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use miso_engine_lane::Backend;
use std::alloc::{GlobalAlloc, System};

use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedPorts, PreparedSidechainPort, ResetKind,
};
use miso_engine_true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR_V1, TRUE_PEAK_LIMITER_PARAMETERS_V1, TruePeakLimiterFactory,
};

struct TrackingAllocator;

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

fn when_active(action: impl FnOnce()) {
    ACTIVE.with(|active| {
        if active.get() {
            action();
        }
    });
}

// SAFETY: every operation delegates its original pointer and layout unchanged to `System`; the
// thread-local counters are observational and armed only around a single test-thread call.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            when_active(|| ALLOCATIONS.set(ALLOCATIONS.get() + 1));
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            when_active(|| ALLOCATIONS.set(ALLOCATIONS.get() + 1));
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        when_active(|| DEALLOCATIONS.set(DEALLOCATIONS.get() + 1));
        // SAFETY: delegates the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegates the original pointer, layout and requested size unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            when_active(|| {
                ALLOCATIONS.set(ALLOCATIONS.get() + 1);
                DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
            });
        }
        replacement
    }
}

fn measure(operation: impl FnOnce()) -> (u64, u64) {
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ACTIVE.set(true);
    operation();
    ACTIVE.set(false);
    (ALLOCATIONS.get(), DEALLOCATIONS.get())
}

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRUE_PEAK_LIMITER_PARAMETERS_V1[index / 2].default_value,
    })
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn automation(block: usize) -> [PreparedAutomationSpan; 1] {
    let value = if block.is_multiple_of(2) { -3.0 } else { -9.0 };
    [PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Left,
        parameter_index: 0,
        start_sample: (block * 128) as u64,
        end_sample: (block * 128) as u64,
        start_value: value,
        end_value: value,
    }]
}

#[test]
fn the_render_path_allocates_nothing() {
    let values = values();
    let mut scalar = TruePeakLimiterFactory
        .prepare(request(&values))
        .expect("prepare");
    let mut left = vec![0.0_f32; 128];
    let mut right = vec![0.0_f32; 128];
    // Warm every lazily initialised path once before arming the counter.
    scalar
        .process(EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128).expect("block"));

    let (allocations, deallocations) = measure(|| {
        for block in 0..100 {
            for (index, sample) in left.iter_mut().enumerate() {
                // A hostile magnitude every twentieth block: the FIR overflows to infinity and the
                // once-per-block boundary check zeroes, resets and counts — still without a heap.
                *sample = if block % 20 == 19 {
                    3.0e38
                } else {
                    ((index % 17) as f32 - 8.0) * 0.25
                };
            }
            right.copy_from_slice(&left);
            let spans = automation(block);
            scalar.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * 128) as u64,
                    &spans,
                    128,
                )
                .expect("block"),
            );
            if block % 33 == 32 {
                scalar.reset(ResetKind::DiscontinuityKeepParameters);
            }
        }
        scalar.reset(ResetKind::FullToDefaults);
    });
    assert_eq!(
        (allocations, deallocations),
        (0, 0),
        "scalar render path allocated"
    );

    let requests: Vec<PrepareEffectRequest<'_>> = (0..8).map(|_| request(&values)).collect();
    let mut bank = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("bank binding")
        .expect("bank available");
    let mut left = vec![0.0_f32; 128 * 8];
    let mut right = vec![0.0_f32; 128 * 8];
    let offsets = [0_u32, 1, 1, 1, 1, 1, 1, 1, 1];
    let spans = automation(0);
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            128,
            BankWidth::Eight,
            0,
            &spans,
            &offsets,
            128,
        )
        .expect("bank block"),
    );

    let (allocations, deallocations) = measure(|| {
        for block in 0..100 {
            for (index, sample) in left.iter_mut().enumerate() {
                *sample = if block % 20 == 19 {
                    3.0e38
                } else {
                    ((index % 23) as f32 - 11.0) * 0.2
                };
            }
            right.copy_from_slice(&left);
            let spans = automation(block);
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    128,
                    BankWidth::Eight,
                    (block * 128) as u64,
                    &spans,
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
            if block % 33 == 32 {
                bank.reset(ResetKind::DiscontinuityKeepParameters);
            }
        }
        bank.reset(ResetKind::FullToDefaults);
    });
    assert_eq!(
        (allocations, deallocations),
        (0, 0),
        "bank render path allocated"
    );
}
