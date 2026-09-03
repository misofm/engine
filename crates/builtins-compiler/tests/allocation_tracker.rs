//! Independent phase-two layout accounting for the sealed builtin artifact.

#![cfg(feature = "test-support")]
#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    num::{NonZeroU32, NonZeroU64, NonZeroUsize},
};
use std::alloc::{GlobalAlloc, System};

use builtins::{MeterConfig, MeterTap};
use builtins_compiler::{
    BuiltinCompileCaps, MeterRequest, prepare_session_builtins,
    test_only_phase_two_allocation_snapshot, test_only_record_phase_two_allocation,
    test_only_reset_phase_two_allocation_tracker,
};
use session::{CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_json};

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

fn session(track_count: u32) -> session::CompiledSession {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("fixture parse");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.tracks.clear();
    model
        .tracks
        .reserve(usize::try_from(track_count).expect("u32 fits usize on supported targets"));
    for index in 0..track_count {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated stable ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
    compile_session(
        &model,
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

fn requests(count: usize) -> Vec<MeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(128).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).expect("constant"),
        reset_generation: 7,
    };
    [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ][..count]
        .iter()
        .copied()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: builtins::MeterHandle(
                NonZeroU64::new(u64::try_from(index).expect("bounded") + 1).expect("nonzero"),
            ),
            track_id: "track-0".to_owned(),
            tap,
            config,
        })
        .collect()
}

fn assert_zero_phase_two_allocations() {
    let snapshot = test_only_phase_two_allocation_snapshot();
    assert_eq!(snapshot.total_bytes, 0);
    assert_eq!(snapshot.largest_allocation_bytes, 0);
    assert_eq!(snapshot.allocation_count, 0);
    assert!(snapshot.layouts.is_empty());
    assert!(!snapshot.overflowed);
}

fn assert_rejects_in_phase_one(
    session: &session::CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) {
    test_only_reset_phase_two_allocation_tracker();
    let error = prepare_session_builtins(session, requests, caps)
        .err()
        .expect("one-below cap must reject");
    assert!(
        error
            .0
            .iter()
            .all(|diagnostic| diagnostic.code == "builtin.resource.limit")
    );
    assert_zero_phase_two_allocations();
}

fn caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
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
    for track_count in [1, 4, 65_537] {
        let session = session(track_count);
        for meter_count in [0, 1, 7] {
            let requests = requests(meter_count);
            test_only_reset_phase_two_allocation_tracker();
            let prepared = prepare_session_builtins(&session, &requests, caps()).expect("prepare");
            let snapshot = test_only_phase_two_allocation_snapshot();
            let report = prepared.resource_report();
            assert!(!snapshot.overflowed);
            assert_eq!(
                snapshot.total_bytes,
                report.engine_owned_retained_payload_bytes,
                "tracks={track_count}, meters={meter_count}, observed={:?}, reported={:?}",
                snapshot.layouts,
                report.retained_layouts()
            );
            assert_eq!(
                snapshot.largest_allocation_bytes, report.maximum_single_allocation_bytes,
                "tracks={track_count}, meters={meter_count}"
            );
            assert_eq!(
                snapshot.allocation_count, report.retained_allocation_count,
                "tracks={track_count}, meters={meter_count}"
            );
            assert_eq!(
                snapshot.layouts,
                report.retained_layouts(),
                "tracks={track_count}, meters={meter_count}"
            );

            let mut exact = caps();
            exact.maximum_total_state_bytes = report.engine_owned_processor_payload_bytes;
            exact.maximum_total_retained_payload_bytes = report.engine_owned_retained_payload_bytes;
            exact.maximum_total_meter_items = report.meter_items.max(1);
            exact.maximum_total_meter_bytes = report.engine_owned_meter_payload_bytes.max(1);
            exact.maximum_single_allocation_bytes = report.maximum_single_allocation_bytes;
            exact.maximum_meter_streams = u64::try_from(meter_count).expect("bounded").max(1);
            prepare_session_builtins(&session, &requests, exact).expect("equal caps accept");

            let mut below = exact;
            below.maximum_total_state_bytes = report
                .engine_owned_processor_payload_bytes
                .checked_sub(1)
                .expect("processor payload is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            let mut below = exact;
            below.maximum_total_retained_payload_bytes = report
                .engine_owned_retained_payload_bytes
                .checked_sub(1)
                .expect("retained payload is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            let mut below = exact;
            below.maximum_single_allocation_bytes = report
                .maximum_single_allocation_bytes
                .checked_sub(1)
                .expect("largest allocation is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            if meter_count > 0 {
                let mut below = exact;
                below.maximum_total_meter_items = report
                    .meter_items
                    .checked_sub(1)
                    .expect("meter item payload is nonzero");
                assert_rejects_in_phase_one(&session, &requests, below);

                let mut below = exact;
                below.maximum_total_meter_bytes = report
                    .engine_owned_meter_payload_bytes
                    .checked_sub(1)
                    .expect("meter payload is nonzero");
                assert_rejects_in_phase_one(&session, &requests, below);

                let mut below = exact;
                below.maximum_meter_streams = u64::try_from(meter_count)
                    .expect("bounded")
                    .checked_sub(1)
                    .expect("positive meter count");
                if below.maximum_meter_streams > 0 {
                    assert_rejects_in_phase_one(&session, &requests, below);
                }
            }
        }
    }
}
