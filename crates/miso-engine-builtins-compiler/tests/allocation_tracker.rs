//! Independent phase-two layout accounting for the sealed builtin artifact.

#![cfg(feature = "test-support")]
#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    num::{NonZeroU32, NonZeroUsize},
};
use std::alloc::{GlobalAlloc, System};

use miso_engine_builtins::{MeterConfig, MeterTap};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterRequest, prepare_session_builtins,
    test_only_phase_two_allocation_snapshot, test_only_record_phase_two_allocation,
    test_only_reset_phase_two_allocation_tracker,
};
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

struct TrackingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: TrackingAllocator = TrackingAllocator;

// SAFETY: this delegates every request unchanged and records only a fixed atomic counter in the
// compiler crate while its explicit test-only phase-two guard is active.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        test_only_record_phase_two_allocation(layout);
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        test_only_record_phase_two_allocation(layout);
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        test_only_record_phase_two_allocation(layout);
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn session() -> miso_engine_session::CompiledSession {
    compile_session(
        &parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
            .expect("fixture parse"),
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("fixture compile")
}

fn caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

#[test]
fn phase_two_allocator_layouts_match_the_checked_resource_report() {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(128).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).expect("constant"),
        reset_generation: 7,
    };
    let requests: Vec<_> = [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ]
    .into_iter()
    .map(|tap| MeterRequest {
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect();
    test_only_reset_phase_two_allocation_tracker();
    let prepared = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
    let (total, largest) = test_only_phase_two_allocation_snapshot();
    let report = prepared.resource_report();
    assert_eq!(total, report.engine_owned_retained_payload_bytes);
    assert_eq!(largest, report.maximum_single_allocation_bytes);
}
