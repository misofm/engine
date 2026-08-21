//! Frozen descriptive benchmark for the production native dependency-wave renderer.

#![allow(unsafe_code)]

use core::num::NonZeroUsize;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    time::Instant,
};

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::realtime::{
    PlanarBufferMut, RenderIo, RenderTime,
    audit::{self, AuditSnapshot, ForbiddenOperation, record_allocator_violation},
};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    NativeGraphBindConfigV1, NativeGraphPreparedMetadataV1, NativeGraphRenderModeV1,
    NativeSchedulerConfigV1, SchedulerSelectionV1, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};

const OBSERVATIONS: usize = 1_000;
const QUANTUM: usize = 128;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every operation forwards the allocator's unchanged pointer/layout contract to System.
// Any allocation or free on an armed render worker aborts rather than unwinding through GlobalAlloc.
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

#[derive(Clone, Copy)]
enum Mode {
    Sequential,
    TwoLane,
    FourLane,
}

impl Mode {
    const fn token(self) -> &'static str {
        match self {
            Self::Sequential => "sequential",
            Self::TwoLane => "two_lane",
            Self::FourLane => "four_lane",
        }
    }

    const fn lanes(self) -> usize {
        match self {
            Self::Sequential => 4,
            Self::TwoLane => 2,
            Self::FourLane => 4,
        }
    }

    const fn render_mode(self) -> NativeGraphRenderModeV1 {
        match self {
            Self::Sequential => NativeGraphRenderModeV1::SingleThread,
            Self::TwoLane | Self::FourLane => NativeGraphRenderModeV1::DependencyWaves,
        }
    }
}

const MODES: [Mode; 3] = [Mode::Sequential, Mode::TwoLane, Mode::FourLane];

struct Source {
    left: f32,
    right: f32,
}

impl GraphRuntimeProcessor for Source {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        block.left.fill(self.left);
        block.right.fill(self.right);
        Ok(())
    }
}

struct Identity;

impl GraphRuntimeProcessor for Identity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

fn main() {
    assert_eq!(
        std::env::args_os().count(),
        1,
        "benchmark accepts no arguments"
    );
    let round = std::env::var("MISO_ENGINE_SCHEDULER_BENCH_ROUND").expect("runner supplies round");
    let round_number = match round.as_str() {
        "warmup" => 0_u32,
        "1" => 1,
        "2" => 2,
        _ => panic!("runner supplied invalid round"),
    };
    for mode in MODES {
        let (mut plan, metadata) = prepared_graph(mode);
        let mut output = vec![0.0_f32; QUANTUM * 2];
        let mut samples = vec![0_u64; OBSERVATIONS];
        let mut output_hash = 0xcbf2_9ce4_8422_2325_u64;
        audit::warm_up();
        audit::reset();
        for (observation, elapsed) in samples.iter_mut().enumerate() {
            let start = Instant::now();
            plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM)
                        .expect("fixed benchmark output"),
                },
                RenderTime {
                    absolute_sample: observation as u64 * QUANTUM as u64,
                },
            )
            .expect("benchmark render");
            *elapsed = u64::try_from(start.elapsed().as_nanos())
                .unwrap_or(u64::MAX)
                .div_ceil(QUANTUM as u64);
            for sample in &output {
                output_hash =
                    (output_hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        let coordinator = audit::snapshot();
        let mut workers = [AuditSnapshot::default(); 3];
        let worker_count = plan.copy_worker_audit_snapshots(&mut workers);
        assert_eq!(worker_count, metadata.resources.scheduler.worker_count);
        assert_eq!(coordinator.total(), 0);
        assert!(
            workers[..worker_count]
                .iter()
                .all(|snapshot| snapshot.total() == 0)
        );
        samples.sort_unstable();
        emit_record(
            mode,
            round_number,
            &samples,
            output_hash,
            metadata,
            coordinator,
            &workers[..worker_count],
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_record(
    mode: Mode,
    round: u32,
    values: &[u64],
    output_hash: u64,
    metadata: NativeGraphPreparedMetadataV1,
    coordinator: AuditSnapshot,
    workers: &[AuditSnapshot],
) {
    let worker_forbidden_total = workers
        .iter()
        .fold(0_u64, |sum, snapshot| sum.saturating_add(snapshot.total()));
    let candidate = env("MISO_ENGINE_SCHEDULER_BENCH_CANDIDATE_SHA256");
    let binary = env("MISO_ENGINE_SCHEDULER_BENCH_BINARY_SHA256");
    let cpu = env("MISO_ENGINE_BENCH_CPU_MODEL");
    let os = env("MISO_ENGINE_BENCH_OS");
    let kernel = env("MISO_ENGINE_BENCH_KERNEL");
    let rust = env("MISO_ENGINE_BENCH_RUST_VERSION");
    let llvm = env("MISO_ENGINE_BENCH_LLVM_VERSION");
    let governor = env("MISO_ENGINE_BENCH_GOVERNOR");
    println!(
        concat!(
            "{{\"schema_version\":1,\"issue\":9,\"mode\":\"{}\",\"round\":{},",
            "\"sample_rate_hz\":48000,\"quantum_frames\":128,\"observations\":1000,",
            "\"percentile_method\":\"nearest_rank\",\"units\":\"ns_per_frame\",",
            "\"min\":{},\"p50\":{},\"p95\":{},\"p99\":{},\"max\":{},",
            "\"selected_lanes\":{},\"worker_count\":{},\"wave_count\":{},",
            "\"unit_count\":{},\"partition_count\":{},\"retained_bytes\":{},",
            "\"output_hash\":\"{:016x}\",\"render_errors\":0,",
            "\"coordinator_forbidden_total\":{},\"worker_forbidden_total\":{},",
            "\"descriptive_only\":true,\"candidate_sha256\":{:?},\"binary_sha256\":{:?},",
            "\"cpu_model\":{:?},\"os\":{:?},\"kernel\":{:?},\"rust_version\":{:?},",
            "\"llvm_version\":{:?},\"governor_or_power_mode\":{:?}}}"
        ),
        mode.token(),
        round,
        values[0],
        percentile(values, 50, 100),
        percentile(values, 95, 100),
        percentile(values, 99, 100),
        values[values.len() - 1],
        metadata.resources.scheduler.selected_lanes,
        metadata.resources.scheduler.worker_count,
        metadata.resources.scheduler.wave_count,
        metadata.resources.scheduler.unit_count,
        metadata.resources.scheduler.partition_count,
        metadata.resources.total_retained_bytes,
        output_hash,
        coordinator.total(),
        worker_forbidden_total,
        candidate,
        binary,
        cpu,
        os,
        kernel,
        rust,
        llvm,
        governor,
    );
}

fn percentile(values: &[u64], numerator: usize, denominator: usize) -> u64 {
    let rank = values.len().saturating_mul(numerator).div_ceil(denominator);
    values[rank.saturating_sub(1).min(values.len() - 1)]
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

fn prepared_graph(
    mode: Mode,
) -> (
    miso_engine_core::realtime::PreparedRenderPlan,
    NativeGraphPreparedMetadataV1,
) {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("canonical session");
    let base_track = model.tracks[0].clone();
    let base_route = model.routes[0].clone();
    model.automation.clear();
    model.tracks = (0..8)
        .map(|index| {
            let mut track = base_track.clone();
            track.id = StableId::parse(&format!("bench{index}")).expect("track ID");
            track.simd1.effects.clear();
            track.dynamic.effects.clear();
            track.simd2.effects.clear();
            track
        })
        .collect();
    model.routes = model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let mut route = base_route.clone();
            route.id =
                StableId::parse(&format!("scheduler-bench-route-{index}")).expect("route ID");
            route.source = RouteSource::Track {
                track_id: track.id.clone(),
                tap: SendTap::PostMatrix,
            };
            route
        })
        .collect();
    let session = compile_session(
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
    .expect("compiled benchmark session");
    let builtins = prepare_session_builtins(
        &session,
        &[],
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
        },
    )
    .expect("prepared benchmark builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 9_100 + mode.lanes() as u64,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: miso_engine_graph::GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 100_000_000,
            maximum_plan_bytes: 200_000_000,
            maximum_single_allocation_bytes: 100_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|_| panic!("compiled benchmark graph"));
    assert!(artifact.prepared_builtin_bank_count() >= 1);
    let envelope = artifact.envelope();
    let nodes = artifact
        .external_binding_nodes()
        .cloned()
        .enumerate()
        .map(|(index, node)| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                } => Box::new(Source {
                    left: index as f32 * 0.01 + 0.1,
                    right: index as f32 * -0.02 - 0.2,
                }),
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node, processor)
        })
        .collect();
    let bound = artifact
        .into_bound_native(
            GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            },
            NativeGraphBindConfigV1 {
                render_mode: mode.render_mode(),
                scheduler: NativeSchedulerConfigV1 {
                    render_lanes: NonZeroUsize::new(mode.lanes()).expect("nonzero lanes"),
                    enabled: true,
                },
                maximum_retained_bytes: 1 << 29,
            },
        )
        .unwrap_or_else(|failure| panic!("native benchmark bind: {}", failure.code));
    match mode {
        Mode::Sequential => assert!(matches!(
            bound.prepared.metadata.selection,
            SchedulerSelectionV1::Sequential(_)
        )),
        Mode::TwoLane | Mode::FourLane => {
            assert_eq!(
                bound.prepared.metadata.selection,
                SchedulerSelectionV1::Parallel
            )
        }
    }
    let metadata = bound.prepared.metadata;
    (bound.prepared.into_plan(), metadata)
}
