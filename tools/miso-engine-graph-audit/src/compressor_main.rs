//! Prepared compressor realtime audit (#88 E10).
//!
//! 100,000 armed blocks through the production `process` and `process_bank` entry points, with a
//! block-rate automation Point every 1,000 blocks so the ramping body — the one place a
//! coefficient is redesigned on the render thread — is audited too. Nothing may allocate,
//! deallocate, lock, log, touch a file, touch the network or make a syscall inside the render
//! scope, and the prepared objects must be destroyed outside it.
//!
//! This is not the production-graph audit; that one belongs to issue 046.

#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};

use miso_engine_compressor::CompressorFactory;
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_core::{KernelBackendV1, target_capabilities};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel, PortId,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedNativeEffect, PreparedNativeEffectBank, PreparedPortsV1, PreparedSidechainPort,
};

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;
const AUTOMATION_EVERY: u64 = 1_000;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every method forwards the unchanged allocation contract to the system allocator. An
// armed allocation aborts instead of unwinding through the allocator boundary.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: `layout` originates from the allocator caller and is forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: `layout` originates from the allocator caller and is forwarded unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: the caller's original pointer/layout contract is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the caller's original allocation contract is forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

/// Holds the prepared objects so their destruction can be asserted to happen off render.
struct OffRenderDrop {
    scalar: Box<dyn PreparedNativeEffect>,
    bank: Option<Box<dyn PreparedNativeEffectBank>>,
}

impl Drop for OffRenderDrop {
    fn drop(&mut self) {
        assert!(
            !audit::is_render_scope_active(),
            "prepared compressor destruction must stay off render"
        );
    }
}

fn sidechain_port() -> PortId {
    PortId::new("sidechain-in").expect("frozen port id")
}

fn main() {
    assert_eq!(
        parse_blocks(),
        BLOCKS,
        "the #88 allocation audit is frozen at 100,000 blocks"
    );

    let width = bank_width();
    let lanes = width.map_or(1, |(_, width)| width.lanes() as usize);
    let mut prepared = OffRenderDrop {
        scalar: prepare_scalar(),
        bank: width.and_then(|(backend, width)| bind_bank(backend, width)),
    };

    let frames = QUANTUM as usize;
    let mut left = vec![0.125_f32; frames];
    let mut right = vec![-0.25_f32; frames];
    let mut sidechain_left = vec![0.5_f32; frames];
    let mut sidechain_right = vec![-0.5_f32; frames];
    let mut bank_left = vec![0.125_f32; frames * lanes];
    let mut bank_right = vec![-0.25_f32; frames * lanes];
    let offsets = vec![0_u32; lanes + 1];
    let mut bank_offsets = vec![0_u32; lanes + 1];
    let left_address = left.as_ptr() as usize;
    let right_address = right.as_ptr() as usize;
    let bank_address = bank_left.as_ptr() as usize;

    // Two automation spans, prepared once: the block loop only chooses whether to pass them.
    let threshold_points = [
        span(ParameterChannel::Left, 0, -30.0),
        span(ParameterChannel::Right, 0, -30.0),
    ];

    audit::warm_up();
    audit::reset();
    eprintln!("MISO_COMPRESSOR_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 0..BLOCKS {
            let first_sample = block
                .checked_mul(u64::from(QUANTUM))
                .expect("frozen sample time");
            let ramping = block % AUTOMATION_EVERY == 0;
            let mut automation: &[PreparedAutomationSpan] = &[];
            let mut points = threshold_points;
            if ramping {
                // Alternate the target so the ramp restarts rather than settling once.
                let value = if (block / AUTOMATION_EVERY) % 2 == 0 {
                    -30.0
                } else {
                    -12.0
                };
                points[0].start_value = value;
                points[0].end_value = value;
                points[1].start_value = value;
                points[1].end_value = value;
                for point in &mut points {
                    point.start_sample = first_sample;
                    point.end_sample = first_sample;
                }
                automation = &points;
            }

            left.fill(if block & 1 == 0 { 0.125 } else { -0.375 });
            right.fill(if block & 1 == 0 { -0.25 } else { 0.5 });
            sidechain_left.fill(if block & 1 == 0 { 0.5 } else { -0.125 });
            sidechain_right.fill(if block & 1 == 0 { -0.5 } else { 0.25 });
            let report = prepared.scalar.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    Some((&sidechain_left, &sidechain_right)),
                    first_sample,
                    automation,
                    QUANTUM,
                )
                .expect("prepared 128-frame block"),
            );
            assert_eq!(report.sanitized_main_samples, 0);
            assert_eq!(report.sanitized_sidechain_samples, 0);
            assert_eq!(report.invalid_spans, 0);
            assert_eq!(report.recovered_left_samples, 0);
            assert_eq!(report.recovered_right_samples, 0);

            if let (Some(bank), Some((_, width))) = (prepared.bank.as_mut(), width) {
                bank_left.fill(if block & 1 == 0 { 0.125 } else { -0.375 });
                bank_right.fill(if block & 1 == 0 { -0.25 } else { 0.5 });
                bank_offsets.fill(0);
                if ramping {
                    // One track's worth of automation, so the ramping body runs in the bank too.
                    for entry in bank_offsets.iter_mut().skip(1) {
                        *entry = points.len() as u32;
                    }
                    bank_offsets[0] = 0;
                }
                let bank_report = bank.process_bank(
                    EffectBankProcessBlock::new(
                        &mut bank_left,
                        &mut bank_right,
                        None,
                        QUANTUM,
                        width,
                        first_sample,
                        if ramping { &points } else { &[] },
                        &bank_offsets,
                        QUANTUM,
                    )
                    .expect("prepared bank block"),
                );
                for track in bank_report.reports {
                    assert_eq!(track.recovered_left_samples, 0);
                    assert_eq!(track.recovered_right_samples, 0);
                }
            }
        }
    });
    eprintln!("MISO_COMPRESSOR_RT_END");

    let snapshot = audit::snapshot();
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(right.as_ptr() as usize, right_address);
    assert_eq!(bank_left.as_ptr() as usize, bank_address);
    assert_eq!(snapshot.total(), 0);
    assert_eq!(offsets.len(), lanes + 1);
    drop(prepared);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue088_compressor_realtime_audit\",",
            "\"blocks\":100000,\"quantum_frames\":128,\"bank_lanes\":{},",
            "\"automation_every_blocks\":1000,",
            "\"destruction_off_render\":true,\"left_address\":{},\"right_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        lanes,
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

/// A block-rate Point on one parameter of one channel.
fn span(channel: ParameterChannel, parameter_index: u32, value: f32) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: 0,
        end_sample: 0,
        start_value: value,
        end_value: value,
    }
}

/// This build's bank width, if it has one.
fn bank_width() -> Option<(KernelBackendV1, BankWidth)> {
    let backend = KernelBackendV1::select(target_capabilities());
    match backend.lanes() {
        4 => Some((backend, BankWidth::Four)),
        8 => Some((backend, BankWidth::Eight)),
        _ => None,
    }
}

/// The descriptor defaults, as an initial-value list.
fn initial_values() -> Vec<InitialParameterValue> {
    let factory = CompressorFactory;
    let mut values = Vec::with_capacity(factory.descriptor().parameters.len() * 2);
    for (index, parameter) in factory.descriptor().parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            values.push(InitialParameterValue {
                parameter_index: u32::try_from(index).expect("frozen descriptor count"),
                channel,
                value: parameter.default_value,
            });
        }
    }
    values
}

fn request(values: &[InitialParameterValue], connected: bool) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: if connected {
                PreparedSidechainPort::Connected {
                    id: sidechain_port(),
                    required: false,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id: sidechain_port(),
                    required: false,
                }
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 15_568,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn prepare_scalar() -> Box<dyn PreparedNativeEffect> {
    let values = initial_values();
    CompressorFactory
        .prepare(request(&values, true))
        .expect("prepared connected-sidechain compressor")
}

fn bind_bank(
    backend: KernelBackendV1,
    width: BankWidth,
) -> Option<Box<dyn PreparedNativeEffectBank>> {
    let values = initial_values();
    let lanes = width.lanes() as usize;
    let requests: Vec<PrepareEffectRequest<'_>> =
        (0..lanes).map(|_| request(&values, false)).collect();
    CompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("bank binding must not fail")
}

fn parse_blocks() -> u64 {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        None => BLOCKS,
        Some("--blocks") => arguments
            .next()
            .expect("--blocks value")
            .parse()
            .expect("integer block count"),
        Some(_) => panic!("unknown audit argument"),
    }
}
