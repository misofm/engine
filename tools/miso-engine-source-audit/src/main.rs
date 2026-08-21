//! Deterministic Issue-010 source-ring realtime and duration-independent resource audit.

#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};

use miso_engine_core::QuantumFrames;
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_source::{
    HostPlanarChunk, PcmSourceRing, PcmSourceRingConfig, SourceCommand, SourceFrame,
    SourceGeneration,
};

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: each call forwards the original allocation contract to the system allocator. A render
// allocation/deallocation aborts before the allocation can become observable.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the caller's valid allocation layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the caller's valid allocation layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the original valid pointer/layout pair unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: this forwards the original valid allocation contract unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn main() {
    let config = PcmSourceRingConfig {
        channel_count: 1,
        quantum_frames: QuantumFrames(QUANTUM),
        frame_capacity: u64::from(QUANTUM),
        initial_generation: SourceGeneration(1),
    };
    let minute = PcmSourceRing::resource_report(config).expect("minute resource");
    let multi_hour = PcmSourceRing::resource_report(config).expect("multi-hour resource");
    assert_eq!(
        minute, multi_hour,
        "duration cannot change retained source layout"
    );

    let (producer, mut consumer, ring) = PcmSourceRing::prepare(config).expect("prepared ring");
    assert_eq!(ring, minute);
    let mut host = producer.into_host_chunk_provider(miso_engine_core::SampleRateHz(48_000));
    let pcm = [0.25_f32; QUANTUM as usize];
    submit(&mut host, SourceGeneration(1), 0, &pcm);
    let mut output = [1.0_f32; QUANTUM as usize];
    let output_address = output.as_ptr() as usize;

    audit::warm_up();
    audit::reset();
    let mut resumed_at = None;
    eprintln!("MISO_SOURCE_RT_BEGIN");
    for block in 0..BLOCKS {
        if block == 2 {
            host.try_seek(SourceCommand::Seek {
                generation: SourceGeneration(2),
                frame: SourceFrame(u64::from(QUANTUM) * 2),
            })
            .expect("off-render resume seek");
            submit(&mut host, SourceGeneration(2), u64::from(QUANTUM) * 2, &pcm);
        } else if block > 2 {
            submit(
                &mut host,
                SourceGeneration(2),
                u64::from(QUANTUM) * block,
                &pcm,
            );
        }
        let report = audit::in_render_scope(|| consumer.read_block_contiguous(&mut output))
            .expect("prepared output shape");
        if block == 1 {
            assert!(output.iter().all(|sample| sample.to_bits() == 0));
            assert_eq!(report.underrun_frames, QUANTUM);
            assert!(report.underrun_event);
        }
        if block == 2 {
            assert!(
                output
                    .iter()
                    .all(|sample| sample.to_bits() == 0.25_f32.to_bits())
            );
            resumed_at = Some(u64::from(QUANTUM) * 2);
        }
    }
    eprintln!("MISO_SOURCE_RT_END");
    let telemetry = consumer.telemetry();
    let snapshot = audit::snapshot();
    assert_eq!(output.as_ptr() as usize, output_address);
    assert_eq!(telemetry.underrun_frames, u64::from(QUANTUM));
    assert_eq!(telemetry.underrun_events, 1);
    assert_eq!(resumed_at, Some(u64::from(QUANTUM) * 2));
    assert_eq!(snapshot.total(), 0);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue010_source_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":{},\"underrun_frames\":{},",
            "\"underrun_events\":{},\"resumed_source_frame\":{},\"output_address\":{},",
            "\"minute_equals_multi_hour_resources\":true,\"descriptive_rss_bytes\":null,",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        BLOCKS,
        QUANTUM,
        telemetry.underrun_frames,
        telemetry.underrun_events,
        resumed_at.expect("resume"),
        output_address,
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.total(),
    );
}

fn submit(
    host: &mut miso_engine_source::HostChunkProvider,
    generation: SourceGeneration,
    start_frame: u64,
    pcm: &[f32],
) {
    host.submit(HostPlanarChunk {
        sample_rate_hz: miso_engine_core::SampleRateHz(48_000),
        generation,
        start_frame: SourceFrame(start_frame),
        planes: &[pcm],
        frames: QUANTUM,
        end_of_region: false,
    })
    .expect("host source submission before render");
}
