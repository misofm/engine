//! Fixed 10,000-callback audit of the shared Issue-039 q128 native graph fixture.

#![allow(unsafe_code)]

use core::num::NonZeroUsize;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::mpsc,
};

use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RenderIo, RenderTime, SwapOutcome,
    audit::{self, AuditSnapshot, ForbiddenOperation, record_allocator_violation},
    plan_exchange,
};
use miso_engine_graph::SchedulerSelectionV1;
use miso_engine_scheduler_fixture::{
    PreparedQ128Fixture, Q128_QUANTUM_FRAMES, Q128RenderMode, prepare_q128_fixture,
};

const CALLBACKS: u64 = 10_000;
const OBSERVERS_PER_CALLBACK: usize = 2;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every operation forwards the allocator's unchanged pointer/layout contract to System.
// An allocation or free on any armed render worker aborts instead of unwinding through GlobalAlloc.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the allocator-provided layout is forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the allocator-provided layout is forwarded unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: the original pointer/layout pair is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the original allocation contract and requested size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

fn main() {
    assert_eq!(std::env::args_os().count(), 1, "audit accepts no arguments");

    let initial = q128_fixture(39_901);
    let replacement = q128_fixture(39_902);
    assert_eq!(initial.metadata, replacement.metadata);
    assert_eq!(initial.report.sha256, replacement.report.sha256);
    assert!(initial.pdc_samples > 0);
    assert!(initial.prepared_builtin_bank_count > 0);
    // #86 F3: every post-input node is a bank member on a vector host; the last bank of the
    // level is padded with identity lanes, so the audit exercises a padded bank and no scalar
    // post-input tail survives.
    assert_eq!(initial.scalar_builtin_tail_count, 0);
    assert!(
        initial.prepared_builtin_bank_lanes > 0
            && initial.prepared_builtin_bank_member_count % initial.prepared_builtin_bank_lanes
                != 0,
        "the audited layout must contain a padded bank"
    );
    assert_eq!(initial.metadata.selection, SchedulerSelectionV1::Parallel);
    assert_eq!(initial.metadata.resources.scheduler.selected_lanes, 4);
    assert_eq!(initial.metadata.resources.scheduler.worker_count, 3);

    let fixture_id = miso_engine_scheduler_fixture::Q128_FIXTURE_ID;
    let pdc_samples = replacement.pdc_samples;
    let preparation_hash = replacement.metadata.test_preparation_transcript.hash;
    let replacement_observers = replacement.observer_transcript();
    assert_eq!(replacement_observers.record_count(), 0);

    // This marker is emitted after both worker sets are prepared and before any render scope.
    eprintln!("MISO_039_PHASE_PREPARED");
    let (mut publisher, mut owner, retirer) = plan_exchange(
        initial.plan,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("plan exchange");
    publisher
        .publish(replacement.plan)
        .unwrap_or_else(|_| panic!("replacement publication"));

    let mut output = vec![0.0_f32; Q128_QUANTUM_FRAMES * 2];
    let output_address = output.as_ptr() as usize;
    let mut output_hash = 0xcbf2_9ce4_8422_2325_u64;
    audit::warm_up();
    audit::reset();

    // The armed interval is delimited outside `RealtimePlanOwner::render` and worker dispatch.
    eprintln!("MISO_039_PHASE_ARMED");
    for block in 0..CALLBACKS {
        let report = owner
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(
                        &mut output,
                        2,
                        Q128_QUANTUM_FRAMES,
                        Q128_QUANTUM_FRAMES,
                    )
                    .expect("fixed q128 output"),
                },
                RenderTime {
                    absolute_sample: block * Q128_QUANTUM_FRAMES as u64,
                },
            )
            .expect("q128 native graph render");
        assert_eq!(report.render.plan_id, 39_902);
        assert_eq!(
            report.swap,
            if block == 0 {
                SwapOutcome::Applied
            } else {
                SwapOutcome::None
            }
        );
        for sample in &output {
            output_hash =
                (output_hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    // All coordinator/worker audit reads occur only after every render scope returned.
    eprintln!("MISO_039_PHASE_DISARMED");
    assert_eq!(output.as_ptr() as usize, output_address);
    let coordinator = audit::snapshot();
    assert_eq!(coordinator.total(), 0);
    let mut workers = [AuditSnapshot::default(); 3];
    assert_eq!(owner.copy_worker_audit_snapshots(&mut workers), 3);
    assert!(workers.iter().all(|snapshot| snapshot.total() == 0));
    assert_eq!(
        replacement_observers.record_count(),
        CALLBACKS as usize * OBSERVERS_PER_CALLBACK
    );
    let observer_hash = replacement_observers.stable_hash();

    drop(publisher);
    let (sender, receiver) = mpsc::sync_channel(0);
    let retirement = std::thread::spawn(move || {
        let mut retirer = retirer;
        let retired = retirer.try_reclaim().expect("one displaced plan");
        drop(retired);
        drop(owner);
        // The active replacement and its scheduler are destroyed off the render thread.
        eprintln!("MISO_039_PHASE_RETIRED");
        sender
            .send(std::thread::current().id())
            .expect("retirement result");
    });
    let retirement_thread_id = receiver.recv().expect("retirement thread ID");
    assert_eq!(retirement.join().expect("retirement join"), ());
    println!(
        concat!(
            "{{\"schema_version\":2,\"kind\":\"native_scheduler_realtime_audit\",",
            "\"fixture_id\":\"{}\",\"callbacks\":{},\"sample_rate_hz\":48000,",
            "\"quantum_frames\":{},\"render_lanes\":4,\"worker_count\":3,",
            "\"plan_swaps\":1,\"pdc_samples\":{},\"preparation_hash\":{},",
            "\"observer_records\":{},\"observer_hash\":{},",
            "\"retired_on_thread\":\"{:?}\",\"output_address\":{},",
            "\"output_hash\":{},\"coordinator_forbidden_total\":{},",
            "\"worker_forbidden_totals\":[{},{},{}]}}"
        ),
        fixture_id,
        CALLBACKS,
        Q128_QUANTUM_FRAMES,
        pdc_samples,
        preparation_hash,
        replacement_observers.record_count(),
        observer_hash,
        retirement_thread_id,
        output_address,
        output_hash,
        coordinator.total(),
        workers[0].total(),
        workers[1].total(),
        workers[2].total(),
    );
}

fn q128_fixture(plan_id: u64) -> PreparedQ128Fixture {
    prepare_q128_fixture(
        48_000,
        4,
        Q128RenderMode::DependencyWaves,
        plan_id,
        CALLBACKS as usize * OBSERVERS_PER_CALLBACK,
    )
    .unwrap_or_else(|error| panic!("q128 audit preparation failed: {error}"))
}
