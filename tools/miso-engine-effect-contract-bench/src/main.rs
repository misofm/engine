//! Native-effect conformance, realtime audit, fixture check, and two-round benchmark driver.
#![allow(missing_docs, unsafe_code)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    env,
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use miso_engine_conformance::{
    ConformanceConfig, DualAccumulatorDelayFactory, run_effect_conformance,
};
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, PreparedBankMetadata,
    PreparedNativeEffect, PreparedNativeEffectBank, ProcessReport, ResetKind, StatePayloadError,
    StatePayloadInput, StatePayloadOutput,
};
use miso_engine_effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedPortsV1,
    PreparedSidechainPort,
};

struct AuditedAllocator;
#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;
static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static DEALLOCATIONS: AtomicU64 = AtomicU64::new(0);

// SAFETY: all methods forward the original pointer/layout contract to the system allocator. The
// armed path terminates immediately rather than unwinding through `GlobalAlloc`.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the valid layout is forwarded unchanged to the system allocator.
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the valid layout is forwarded unchanged to the system allocator.
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        DEALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: this pointer/layout pair came from this allocator and is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the original allocation contract and requested size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

struct NoopBank {
    metadata: PreparedBankMetadata,
}
impl PreparedNativeEffectBank for NoopBank {
    fn metadata(&self) -> PreparedBankMetadata {
        self.metadata.clone()
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        black_box((&block.left, &block.right));
        BankProcessReport::empty(self.metadata.width)
    }
    fn snapshot_track_state_payload(
        &self,
        track: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        if track >= self.metadata.width.lanes() {
            return Err(StatePayloadError {
                code: "effect.state.track",
            });
        }
        output.common.fill(0);
        output.left.fill(0);
        output.right.fill(0);
        Ok(())
    }
    fn restore_track_state_payload(
        &mut self,
        track: u32,
        version: u32,
        _: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        if track >= self.metadata.width.lanes()
            || version != self.metadata.program_key.state_layout_version
        {
            Err(StatePayloadError {
                code: "effect.state.track",
            })
        } else {
            Ok(())
        }
    }
}

fn prepared(bypass: bool) -> Box<dyn PreparedNativeEffect> {
    let factory = DualAccumulatorDelayFactory::correct();
    let initial = [
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 1.0,
        },
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Right,
            value: 1.0,
        },
    ];
    factory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: miso_engine_conformance::DUAL_ACCUMULATOR_DELAY_DESCRIPTOR.ports[1].id,
                    required: false,
                },
            },
            initial_values: &initial,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 8,
            },
        })
        .expect("valid bounded conformance processor")
}

fn audit_process(blocks: u64, markers: bool) {
    let mut effect = prepared(false);
    let mut left = [0.0_f32; 128];
    let mut right = [0.0_f32; 128];
    let mut totals = ProcessReport::default();
    audit::warm_up();
    audit::reset();
    if markers {
        eprintln!("MISO_ENGINE_EFFECT_RT_BEGIN");
    }
    for block_index in 0..blocks {
        let extreme = if block_index & 1 == 0 {
            f32::MAX
        } else {
            -f32::MAX
        };
        left.fill(extreme);
        right.fill(-extreme);
        let block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            block_index.saturating_mul(128),
            &[],
            128,
        )
        .expect("fixed audit block");
        let report = audit::in_render_scope(|| effect.process(block));
        totals.sanitized_main_samples = totals
            .sanitized_main_samples
            .saturating_add(report.sanitized_main_samples);
        totals.invalid_spans = totals.invalid_spans.saturating_add(report.invalid_spans);
        assert!(left.iter().chain(&right).all(|value| value.is_finite()));
        black_box((&left, &right));
    }
    if markers {
        eprintln!("MISO_ENGINE_EFFECT_RT_END");
    }
    let snapshot = audit::snapshot();
    assert_eq!(snapshot.total(), 0);
    assert_eq!(totals.sanitized_main_samples, 0);
    assert_eq!(totals.invalid_spans, 0);
    println!(
        "{{\"schema_version\":1,\"kind\":\"effect_realtime_audit\",\"blocks\":{blocks},\"frames_per_block\":128,\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}",
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.total()
    );
}

fn conformance() {
    let report = run_effect_conformance(
        &DualAccumulatorDelayFactory::correct(),
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(
        report.passed(),
        "failed launch gates: {:?}",
        report.launch_gates.failures
    );
    println!(
        "{{\"schema_version\":1,\"kind\":\"effect_conformance\",\"launch_prepared_configurations\":{},\"launch_process_calls\":{},\"launch_failed_gates\":0,\"extended_compatibility_prepared_configurations\":{},\"extended_compatibility_process_calls\":{},\"extended_compatibility_failed_probes\":{}}}",
        report.launch_gates.prepared_configurations,
        report.launch_gates.process_calls,
        report.extended_compatibility_probes.prepared_configurations,
        report.extended_compatibility_probes.process_calls,
        report.extended_compatibility_probes.failures.len()
    );
}

fn benchmark() {
    for round in 1..=2 {
        let mut effect = prepared(false);
        let mut left = [0.0_f32; 128];
        let mut right = [0.0_f32; 128];
        let key = effect.metadata().program_key();
        let mut bank4 = NoopBank {
            metadata: PreparedBankMetadata {
                width: BankWidth::Four,
                program_key: key.clone(),
            },
        };
        let mut bank8 = NoopBank {
            metadata: PreparedBankMetadata {
                width: BankWidth::Eight,
                program_key: key,
            },
        };
        let mut bank_left = [0.0_f32; 1024];
        let mut bank_right = [0.0_f32; 1024];
        let offsets4 = [0_u32; 5];
        let offsets8 = [0_u32; 9];
        let mut common = [0_u8; 4];
        let mut state_left = [0_u8; 56];
        let mut state_right = [0_u8; 56];
        for workload in [
            "scalar_noop",
            "bank4_noop",
            "bank8_noop",
            "descriptor_validation",
            "factory_prepare",
            "state_snapshot_restore",
        ] {
            let observations = 1000_u64;
            let mut samples = Vec::with_capacity(observations as usize);
            ALLOCATIONS.store(0, Ordering::Relaxed);
            DEALLOCATIONS.store(0, Ordering::Relaxed);
            for index in 0..observations {
                let started = Instant::now();
                match workload {
                    "scalar_noop" => {
                        let block = EffectProcessBlock::new(
                            &mut left,
                            &mut right,
                            None,
                            index * 128,
                            &[],
                            128,
                        )
                        .unwrap();
                        black_box(effect.process(block));
                    }
                    "bank4_noop" => {
                        let block = EffectBankProcessBlock::new(
                            &mut bank_left[..512],
                            &mut bank_right[..512],
                            None,
                            128,
                            BankWidth::Four,
                            index * 128,
                            &[],
                            &offsets4,
                            128,
                        )
                        .unwrap();
                        black_box(bank4.process_bank(block));
                    }
                    "bank8_noop" => {
                        let block = EffectBankProcessBlock::new(
                            &mut bank_left,
                            &mut bank_right,
                            None,
                            128,
                            BankWidth::Eight,
                            index * 128,
                            &[],
                            &offsets8,
                            128,
                        )
                        .unwrap();
                        black_box(bank8.process_bank(block));
                    }
                    "descriptor_validation" => {
                        black_box(miso_engine_effect_contract::validate_descriptor_v1(
                            effect.metadata().descriptor,
                        ))
                        .unwrap();
                    }
                    "factory_prepare" => {
                        black_box(prepared(false));
                    }
                    _ => {
                        let metadata = effect.metadata();
                        let output = StatePayloadOutput::new(
                            &mut common,
                            &mut state_left,
                            &mut state_right,
                            metadata.state_sizes,
                        )
                        .unwrap();
                        effect.snapshot_state_payload(output).unwrap();
                        let input = StatePayloadInput::new(
                            &common,
                            &state_left,
                            &state_right,
                            metadata.state_sizes,
                        )
                        .unwrap();
                        effect
                            .restore_state_payload(metadata.descriptor.state_layout_version, input)
                            .unwrap();
                    }
                }
                samples.push(started.elapsed().as_nanos());
            }
            samples.sort_unstable();
            let min = samples[0];
            let p50 = nearest_rank(&samples, 500);
            let p95 = nearest_rank(&samples, 950);
            let p99 = nearest_rank(&samples, 990);
            let p99_9 = nearest_rank(&samples, 999);
            let max = samples[samples.len() - 1];
            let allocations = ALLOCATIONS.load(Ordering::Relaxed);
            let deallocations = DEALLOCATIONS.load(Ordering::Relaxed);
            let width = match workload {
                "bank4_noop" => 4,
                "bank8_noop" => 8,
                _ => 1,
            };
            let items = width;
            let bytes = if workload == "state_snapshot_restore" {
                116
            } else {
                0
            };
            println!(
                "{{\"schema_version\":1,\"workload\":\"{workload}\",\"round\":{round},\"observations\":{observations},\"warmup\":0,\"units\":\"ns\",\"min\":{min},\"p50\":{p50},\"p95\":{p95},\"p99\":{p99},\"p99_9\":{p99_9},\"max\":{max},\"frames\":128,\"items\":{items},\"bytes\":{bytes},\"allocations\":{allocations},\"deallocations\":{deallocations},\"fixture_hash\":\"26e35dacebe4922d7fd7bf63d6cdc6c7084128bf64390a35a17907e249cb1e0b\",\"fixture_count\":4,\"sample_rate\":48000,\"quantum\":128,\"width\":{width},\"cpu\":\"{}\",\"os\":\"{}\",\"governor\":\"{}\",\"rust\":\"{}\",\"llvm\":\"{}\",\"target\":\"{}\",\"features\":\"{}\",\"opt\":\"3\",\"lto\":\"off\",\"codegen_units\":16,\"missing_metadata\":\"environment values reported as unknown\"}}",
                metadata("MISO_ENGINE_BENCH_CPU_MODEL"),
                env::consts::OS,
                metadata("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"),
                metadata("MISO_ENGINE_BENCH_RUST_VERSION"),
                metadata("MISO_ENGINE_BENCH_LLVM_VERSION"),
                metadata("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
                metadata("MISO_ENGINE_BENCH_TARGET_FEATURES")
            );
        }
    }
}

fn nearest_rank(sorted: &[u128], permille: usize) -> u128 {
    let rank = (permille * sorted.len()).div_ceil(1000).max(1);
    sorted[rank - 1]
}

fn metadata(name: &str) -> String {
    env::var(name)
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
        .replace('"', "'")
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode] if mode == "--conformance" => conformance(),
        [mode] if mode == "--benchmark-two-rounds" => benchmark(),
        [mode, blocks] if mode == "--audit" => {
            audit_process(blocks.parse().expect("block count"), false)
        }
        [mode, blocks, marker] if mode == "--audit" && marker == "--trace-markers" => {
            audit_process(blocks.parse().expect("block count"), true)
        }
        _ => {
            eprintln!(
                "usage: miso_engine_effect_contract_bench --conformance | --audit BLOCKS [--trace-markers] | --benchmark-two-rounds"
            );
            std::process::exit(2);
        }
    }
}
