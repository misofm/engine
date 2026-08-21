//! One-million-call allocation and forbidden-operation audit for prepared scalar builtins.

#![allow(unsafe_code)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, Ordering};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameters, ChannelParameters, DualMonoBlock, Matrix2x2, MeterAccumulator,
    MeterConfig, MeterHandle,
};
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;
static ABORT_ALLOCATOR_VIOLATION: AtomicBool = AtomicBool::new(true);

// SAFETY: each operation forwards the unchanged allocation contract to the system allocator.
// The armed branch aborts instead of unwinding through the allocator implementation.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation)
            && ABORT_ALLOCATOR_VIOLATION.load(Ordering::Relaxed)
        {
            std::process::abort();
        }
        // SAFETY: the supplied layout is forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation)
            && ABORT_ALLOCATOR_VIOLATION.load(Ordering::Relaxed)
        {
            std::process::abort();
        }
        // SAFETY: the supplied layout is forwarded unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation)
            && ABORT_ALLOCATOR_VIOLATION.load(Ordering::Relaxed)
        {
            std::process::abort();
        }
        // SAFETY: the original pointer/layout contract is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation)
            && ABORT_ALLOCATOR_VIOLATION.load(Ordering::Relaxed)
        {
            std::process::abort();
        }
        // SAFETY: the original pointer/layout and requested size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

enum Mode {
    Audit,
    Probe(ForbiddenOperation),
}

fn main() {
    let (mode, blocks) = parse_arguments();
    match mode {
        Mode::Audit => run_audit(blocks),
        Mode::Probe(operation) => run_probe(operation),
    }
}

fn run_audit(blocks: u64) {
    assert!(blocks > 0, "audit requires a positive block count");
    let (mut chain, mut meters) = prepare();
    let mut left = [0.25_f32; 128];
    let mut right = [-0.5_f32; 128];
    let left_address = left.as_ptr() as usize;
    let right_address = right.as_ptr() as usize;
    audit::warm_up();
    audit::reset();
    eprintln!("MISO_BUILTINS_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 0..blocks {
            let first_sample = block.checked_mul(128).expect("bounded audit sample time");
            let report = chain
                .process_dual_mono(
                    DualMonoBlock::new(&mut left, &mut right, first_sample).expect("fixed block"),
                )
                .expect("fixed block processing");
            assert_eq!(report.sanitized_input, 0);
            for meter in &mut meters {
                meter
                    .observe(&left, &right, first_sample)
                    .expect("fixed observer lanes");
            }
        }
    });
    eprintln!("MISO_BUILTINS_RT_END");
    let snapshot = audit::snapshot();
    let expected_drops = blocks.saturating_sub(1);
    for meter in &meters {
        assert_eq!(meter.dropped_snapshots(), expected_drops);
    }
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(right.as_ptr() as usize, right_address);
    assert_eq!(snapshot.total(), 0);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"builtins_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":128,\"observers\":7,",
            "\"queue_success_windows\":7,\"queue_full_windows\":{},",
            "\"left_address\":{},\"right_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},",
            "\"logs\":{},\"file_io\":{},\"network_io\":{},",
            "\"syscalls\":{},\"total_violations\":{}}}"
        ),
        blocks,
        expected_drops.saturating_mul(7),
        left_address,
        right_address,
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

fn prepare() -> (BuiltinChain, [MeterAccumulator; 7]) {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 100.0,
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                hpf_hz: 200.0,
                lpf_hz: 2_000.0,
                ..ChannelParameters::default()
            },
            smoothing_samples: 64,
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare chain");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.9,
            lr: 0.1,
            rl: -0.1,
            rr: 0.9,
        })
        .expect("prepare matrix ramp");
    let meters = core::array::from_fn(|index| {
        MeterAccumulator::prepare(
            MeterHandle(NonZeroU64::new(index as u64 + 1).expect("one based")),
            MeterConfig {
                period_frames: NonZeroU32::new(128).expect("quantum"),
                peak_hold_frames: 32,
                peak_decay_db_per_second: 12.0,
                queue_capacity: NonZeroUsize::new(1).expect("bounded queue"),
                reset_generation: 1,
            },
            48_000,
        )
        .expect("prepare meter")
        .accumulator
    });
    (chain, meters)
}

fn run_probe(operation: ForbiddenOperation) -> ! {
    audit::warm_up();
    audit::reset();
    ABORT_ALLOCATOR_VIOLATION.store(false, Ordering::Relaxed);
    audit::in_render_scope(|| audit::forbidden(operation));
    panic!("forbidden-operation probe unexpectedly survived")
}

fn parse_arguments() -> (Mode, u64) {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut mode = Mode::Audit;
    let mut blocks = 1_000_000_u64;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--blocks" => {
                index += 1;
                blocks = arguments
                    .get(index)
                    .and_then(|value| value.parse().ok())
                    .filter(|value: &u64| *value > 0)
                    .expect("--blocks requires a positive integer");
            }
            "--probe" => {
                index += 1;
                mode = Mode::Probe(parse_operation(
                    arguments.get(index).expect("--probe requires an operation"),
                ));
            }
            _ => panic!("unknown audit argument"),
        }
        index += 1;
    }
    (mode, blocks)
}

fn parse_operation(value: &str) -> ForbiddenOperation {
    match value {
        "allocation" => ForbiddenOperation::Allocation,
        "deallocation" => ForbiddenOperation::Deallocation,
        "lock" => ForbiddenOperation::Lock,
        "log" => ForbiddenOperation::Log,
        "file-io" => ForbiddenOperation::FileIo,
        "network-io" => ForbiddenOperation::NetworkIo,
        "syscall" => ForbiddenOperation::Syscall,
        _ => panic!("unknown forbidden operation"),
    }
}
