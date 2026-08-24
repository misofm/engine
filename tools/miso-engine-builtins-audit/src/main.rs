//! One-million-call allocation and forbidden-operation audit for prepared scalar builtins.

#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, Ordering};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameters, BuiltinProcessReport, BuiltinResetKind, ChannelParameters,
    DualMonoBlock, Matrix2x2,
};
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;
static ABORT_ALLOCATOR_VIOLATION: AtomicBool = AtomicBool::new(true);
const BLOCKS: u64 = 1_000_000;
const QUANTUM: usize = 128;
const EXPECTED_SCHEDULE_PCM: &[u8] = include_bytes!("../fixtures/v1/direct-schedule.pcm.f32le");
const EXPECTED_RESULT: &str = include_str!("../fixtures/v1/direct-result.json");
const AUDIT_MANIFEST_SHA256: &str =
    "065aa23474266e9882853ffea3220fc8ce9559596c42e937a7a9b6fe4b369942";
const AUDIT_RESULT_SHA256: &str =
    "91f326645f8ddd0fd5edb4d8c476bfce24830dec3c1b0d3fcf73f49e6da201c8";

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
    let mode = parse_arguments();
    match mode {
        Mode::Audit => run_audit(),
        Mode::Probe(operation) => run_probe(operation),
    }
}

fn run_audit() {
    let mut chain = prepare();
    let mut left = [0.25_f32; QUANTUM];
    let mut right = [-0.5_f32; QUANTUM];
    let left_address = left.as_ptr() as usize;
    let right_address = right.as_ptr() as usize;
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    let mut total = BuiltinProcessReport::default();
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        })
        .expect("first matrix target");
    audit::warm_up();
    audit::reset();

    for block in 0..6 {
        match block {
            1 => chain
                .set_matrix_target(Matrix2x2 {
                    ll: 0.9,
                    lr: 0.1,
                    rl: -0.1,
                    rr: 0.9,
                })
                .expect("retarget"),
            2 => assert_eq!(
                chain.set_matrix_target(Matrix2x2 {
                    ll: f32::NAN,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                }),
                Err(miso_engine_builtins::BuiltinParameterError::MatrixCoefficient)
            ),
            4 => chain.reset(BuiltinResetKind::DiscontinuityKeepTargets),
            5 => chain.reset(BuiltinResetKind::FullToPrepared),
            _ => {}
        }
        prepare_input(&mut left, &mut right, block);
        let report = traced_process(&mut chain, &mut left, &mut right, block);
        add_report(&mut total, report);
        assert_schedule_block(block, &left, &right);
        fold_pcm(&mut digest, &left, &right);
    }

    eprintln!("MISO_ENGINE_BUILTINS_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 6..BLOCKS {
            prepare_input(&mut left, &mut right, block);
            let first_sample = block
                .checked_mul(QUANTUM as u64)
                .expect("bounded audit sample time");
            let report = chain.process_dual_mono(
                DualMonoBlock::new(&mut left, &mut right, first_sample).expect("fixed block"),
            );
            add_report(&mut total, report);
            fold_pcm(&mut digest, &left, &right);
        }
    });
    eprintln!("MISO_ENGINE_BUILTINS_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(right.as_ptr() as usize, right_address);
    assert_eq!(snapshot.total(), 0);
    let deterministic = format!(
        "{{\"schema_version\":1,\"calls\":1000000,\"sample_rate_hz\":48000,\"quantum_frames\":128,\"pcm_digest\":\"{digest:016x}\",\"sanitized_input\":\"{:016x}\",\"sanitized_output\":\"{:016x}\",\"recovered_left\":\"{:016x}\",\"recovered_right\":\"{:016x}\"}}\n",
        total.sanitized_input,
        total.sanitized_output,
        total.recovered_left_state,
        total.recovered_right_state,
    );
    assert_eq!(deterministic, EXPECTED_RESULT);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue069_direct_realtime_audit\",",
            "\"calls\":1000000,\"sample_rate_hz\":48000,\"quantum_frames\":128,",
            "\"schedule_blocks\":6,\"pcm_digest\":\"{:016x}\",",
            "\"audit_manifest_sha256\":\"{}\",\"audit_result_sha256\":\"{}\",",
            "\"stable_left_address\":true,\"stable_right_address\":true,",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"feature_detection\":{},",
            "\"logs\":{},\"file_io\":{},\"network_io\":{},",
            "\"syscalls\":{},\"panic_unwinds\":{},\"total_violations\":{}}}"
        ),
        digest,
        AUDIT_MANIFEST_SHA256,
        AUDIT_RESULT_SHA256,
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.feature_detection,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.panic_unwinds,
        snapshot.total(),
    );
}

fn prepare() -> BuiltinChain {
    BuiltinChain::new(
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
            smoothing_samples: 257,
            ..BuiltinParameters::default()
        },
    )
    .expect("prepare chain")
}

fn prepare_input(left: &mut [f32; QUANTUM], right: &mut [f32; QUANTUM], block: u64) {
    left.fill(0.25);
    right.fill(-0.5);
    if block == 2 {
        left[0] = f32::NAN;
        right[0] = f32::INFINITY;
    }
}

fn traced_process(
    chain: &mut BuiltinChain,
    left: &mut [f32; QUANTUM],
    right: &mut [f32; QUANTUM],
    block: u64,
) -> BuiltinProcessReport {
    eprintln!("MISO_ENGINE_BUILTINS_RT_BEGIN");
    let report = audit::in_render_scope(|| {
        chain.process_dual_mono(
            DualMonoBlock::new(left, right, block * QUANTUM as u64).expect("fixed block"),
        )
    });
    eprintln!("MISO_ENGINE_BUILTINS_RT_END");
    report
}

fn add_report(total: &mut BuiltinProcessReport, report: BuiltinProcessReport) {
    total.sanitized_input += report.sanitized_input;
    total.sanitized_output += report.sanitized_output;
    total.recovered_left_state += report.recovered_left_state;
    total.recovered_right_state += report.recovered_right_state;
}

fn fold_pcm(digest: &mut u64, left: &[f32], right: &[f32]) {
    for sample in left.iter().chain(right) {
        *digest ^= u64::from(sample.to_bits());
        *digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

fn assert_schedule_block(block: u64, left: &[f32; QUANTUM], right: &[f32; QUANTUM]) {
    let start = usize::try_from(block).expect("six blocks") * QUANTUM * 2 * 4;
    let expected = &EXPECTED_SCHEDULE_PCM[start..start + QUANTUM * 2 * 4];
    for (sample, word) in left.iter().chain(right).zip(expected.chunks_exact(4)) {
        assert_eq!(
            sample.to_bits(),
            u32::from_le_bytes(word.try_into().expect("word"))
        );
    }
}

fn run_probe(operation: ForbiddenOperation) -> ! {
    audit::warm_up();
    audit::reset();
    ABORT_ALLOCATOR_VIOLATION.store(false, Ordering::Relaxed);
    audit::in_render_scope(|| {
        if operation == ForbiddenOperation::PanicUnwind {
            panic!("deliberate panic/unwind detector probe");
        }
        audit::forbidden(operation);
    });
    panic!("forbidden-operation probe unexpectedly survived")
}

fn parse_arguments() -> Mode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut mode = Mode::Audit;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
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
    mode
}

fn parse_operation(value: &str) -> ForbiddenOperation {
    match value {
        "allocation" => ForbiddenOperation::Allocation,
        "deallocation" => ForbiddenOperation::Deallocation,
        "lock" => ForbiddenOperation::Lock,
        "feature-detection" => ForbiddenOperation::FeatureDetection,
        "log" => ForbiddenOperation::Log,
        "file-io" => ForbiddenOperation::FileIo,
        "network-io" => ForbiddenOperation::NetworkIo,
        "syscall" => ForbiddenOperation::Syscall,
        "panic-unwind" => ForbiddenOperation::PanicUnwind,
        _ => panic!("unknown forbidden operation"),
    }
}
