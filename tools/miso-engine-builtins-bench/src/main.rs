//! Frozen issue-035 benchmark emitter. The runner is the sole authorized timing entrypoint.

#![allow(unsafe_code)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    time::Instant,
};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameters, DualMonoBlock, Matrix2x2, MeterConfig, MeterHandle,
    MeterSnapshot, MeterTap,
};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, prepare_session_builtins,
};
use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_core::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime,
    audit::{self, ForbiddenOperation, record_allocator_violation},
};
use miso_engine_effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
use miso_engine_effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use miso_engine_graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use miso_engine_graph_compiler::{
    GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use miso_engine_session::{
    CompileCaps, EffectIdentity, RouteSource, SendTap, StableId, compile_session,
    parse_session_toml,
};
use sha2::{Digest, Sha256};

const QUANTUM: usize = 128;
const RATES: [u32; 2] = [48_000, 96_000];
const ROUNDS: [u32; 2] = [1, 2];
const RENDER_WARMUP_BATCHES: usize = 64;
const RENDER_MEASURED_BATCHES: usize = 512;
const OPERATIONS_PER_RENDER_BATCH: usize = 8;
const PREPARE_WARMUP_BATCHES: usize = 16;
const PREPARE_MEASURED_BATCHES: usize = 128;
const PREPARE_TRACKS: usize = 256;
const OBSERVERS: usize = 7;
const ISSUE: u32 = 35;
const INPUT_MANIFEST: &[u8] = include_bytes!("../../../fixtures/builtins/v1/MANIFEST.tsv");
const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

const WORKLOADS: [Workload; 5] = [
    Workload::FullChainFilters,
    Workload::IdentityChain,
    Workload::MatrixRamp,
    Workload::MeterSuccessFull,
    Workload::Prepare256Tracks,
];

struct AuditedAllocator;
#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: forwards valid layouts to the system allocator; armed render allocation aborts.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Workload {
    FullChainFilters,
    IdentityChain,
    MatrixRamp,
    MeterSuccessFull,
    Prepare256Tracks,
}
impl Workload {
    const fn kind(self) -> &'static str {
        match self {
            Self::FullChainFilters => "full_chain_filters",
            Self::IdentityChain => "identity_chain",
            Self::MatrixRamp => "matrix_ramp",
            Self::MeterSuccessFull => "meter_success_full",
            Self::Prepare256Tracks => "prepare_256_tracks",
        }
    }
    const fn is_prepare(self) -> bool {
        matches!(self, Self::Prepare256Tracks)
    }
}

#[derive(Clone, Copy)]
struct WorkloadShape {
    tracks: usize,
    meters: usize,
    meter_capacity: usize,
    retained_payload_bytes: u64,
}
struct Measurement {
    samples_ns: Vec<u64>,
    output_sha256: String,
    shape: WorkloadShape,
    audit: Option<audit::AuditSnapshot>,
}

struct RenderRoundStates {
    workload: Workload,
    rate_hz: u32,
    rounds: [RenderRuntime; 2],
}

/// Two independently prepared graph artifacts for the frozen meter success/full workload.
///
/// The bound runtime owns the separately prepared plans, pre-fills only the capacity-one plan
/// off timing, and drains the success plan through compiler-owned consumers.
struct RealMeterTapArtifactPair {
    success: PreparedGraphBuiltinsArtifact,
    full: PreparedGraphBuiltinsArtifact,
}

fn prepare_real_meter_tap_artifacts(rate_hz: u32) -> RealMeterTapArtifactPair {
    RealMeterTapArtifactPair {
        success: prepare_real_meter_tap_artifact(rate_hz, 4, 35_001),
        full: prepare_real_meter_tap_artifact(rate_hz, 1, 35_002),
    }
}

fn prepare_real_meter_tap_artifact(
    rate_hz: u32,
    queue_capacity: usize,
    plan_id: u64,
) -> PreparedGraphBuiltinsArtifact {
    let mut model = parse_session_toml(SESSION).expect("frozen benchmark session");
    model.sample_rate_hz = rate_hz;
    model.sources[0].sample_rate_hz = rate_hz;
    model.automation.clear();
    let mut delay = model.tracks[0].dynamic.effects[0].clone();
    delay.identity = EffectIdentity::Native {
        effect_id: StableId::parse("conformance.delay").expect("frozen effect ID"),
    };
    delay.params.clear();
    delay.id = StableId::parse("benchmark-simd1-delay").expect("frozen effect ID");
    model.tracks[0].simd1.effects = vec![delay.clone()];
    delay.id = StableId::parse("benchmark-dynamic-delay").expect("frozen effect ID");
    model.tracks[0].dynamic.effects = vec![delay.clone()];
    delay.id = StableId::parse("benchmark-simd2-delay").expect("frozen effect ID");
    model.tracks[0].simd2.effects = vec![delay];

    let session = compile_session(&model, unbounded_compile_caps()).expect("frozen session");
    let registry = NativeEffectRegistry::new([
        Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>
    ])
    .expect("frozen effect registry");
    let effects = prepare_native_session_effects(&session, &registry, unbounded_effect_caps())
        .expect("frozen effects");
    let builtins = prepare_session_builtins(
        &effects.session,
        &real_meter_tap_requests(&session, queue_capacity),
        unbounded_builtin_caps(),
    )
    .expect("frozen builtin tap requests");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id,
        effects,
        builtins,
        caps: unbounded_graph_caps(),
    })
    .unwrap_or_else(|_| panic!("frozen real-tap graph"))
}

struct BenchmarkGraphSource;

impl GraphRuntimeProcessor for BenchmarkGraphSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let sample = block.first_sample + index as u64;
            let value = sample as f32 * 0.0001 + 0.125;
            *left = value;
            *right = -value * 0.75;
        }
        Ok(())
    }
}

struct BenchmarkGraphIdentity;

impl GraphRuntimeProcessor for BenchmarkGraphIdentity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

struct RealMeterTapPlan {
    plan: PreparedRenderPlan,
    consumers: Vec<MeterConsumer>,
    pcm: [f32; QUANTUM * 2],
}

impl RealMeterTapPlan {
    fn bind(artifact: PreparedGraphBuiltinsArtifact) -> Self {
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = match node {
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    } => Box::new(BenchmarkGraphSource),
                    _ => Box::new(BenchmarkGraphIdentity),
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|_| panic!("frozen real-tap graph binding"));
        Self {
            plan: bound.plan,
            consumers: bound.meter_consumers,
            pcm: [0.0; QUANTUM * 2],
        }
    }

    fn render(&mut self, first_sample: u64) {
        self.plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.pcm, 2, QUANTUM, QUANTUM)
                        .expect("fixed benchmark output"),
                },
                RenderTime {
                    absolute_sample: first_sample,
                },
            )
            .expect("frozen real-tap render");
    }

    fn drain_all(&mut self) -> Vec<MeterConsumerSnapshot> {
        self.consumers
            .iter_mut()
            .map(|consumer| MeterConsumerSnapshot {
                handle: consumer.handle,
                tap: consumer.tap,
                snapshot: consumer
                    .consumer
                    .try_pop()
                    .expect("one completed window per real tap"),
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct MeterConsumerSnapshot {
    handle: MeterHandle,
    tap: MeterTap,
    snapshot: MeterSnapshot,
}

struct RealMeterTapRuntime {
    success: RealMeterTapPlan,
    full: RealMeterTapPlan,
    output: Sha256,
}

impl RealMeterTapRuntime {
    fn new(rate_hz: u32) -> Self {
        let pair = prepare_real_meter_tap_artifacts(rate_hz);
        let mut success = RealMeterTapPlan::bind(pair.success);
        let mut full = RealMeterTapPlan::bind(pair.full);
        success.render(0);
        let prefill_success = success.drain_all();
        assert_eq!(prefill_success.len(), OBSERVERS, "frozen success prefill");
        full.render(0);
        Self {
            success,
            full,
            output: Sha256::new(),
        }
    }

    fn render_one(&mut self, first_sample: u64) {
        self.success.render(first_sample);
        for record in self.success.drain_all() {
            hash_meter_snapshot(&mut self.output, record);
        }
        self.full.render(first_sample);
        for value in self.success.pcm.iter().chain(&self.full.pcm) {
            self.output.update(value.to_bits().to_le_bytes());
        }
    }

    fn output_sha256(&self) -> String {
        hex_digest(self.output.clone().finalize())
    }
}

fn hash_meter_snapshot(hash: &mut Sha256, record: MeterConsumerSnapshot) {
    hash.update(record.handle.0.get().to_le_bytes());
    hash.update([record.tap as u8]);
    let snapshot = record.snapshot;
    for value in [
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        u64::from(snapshot.frames),
        snapshot.left.sample_peak.to_bits().into(),
        snapshot.left.rms.to_bits(),
        snapshot.left.energy.to_bits(),
        snapshot.left.held_peak.to_bits().into(),
        snapshot.left.clipped_samples,
        snapshot.left.sanitized_samples,
        snapshot.right.sample_peak.to_bits().into(),
        snapshot.right.rms.to_bits(),
        snapshot.right.energy.to_bits(),
        snapshot.right.held_peak.to_bits().into(),
        snapshot.right.clipped_samples,
        snapshot.right.sanitized_samples,
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_discontinuities,
        snapshot.cumulative_dropped_snapshots,
    ] {
        hash.update(value.to_le_bytes());
    }
}

fn real_meter_tap_requests(
    session: &miso_engine_session::CompiledSession,
    queue_capacity: usize,
) -> Vec<MeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(session.quantum().0).expect("frozen quantum"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(queue_capacity).expect("frozen meter capacity"),
        reset_generation: 35,
    };
    [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, tap)| MeterRequest {
        handle: MeterHandle(NonZeroU64::new(index as u64 + 1).expect("one-based handle")),
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect()
}

const fn unbounded_compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

const fn unbounded_effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_scratch_bytes: u64::MAX,
        maximum_automation_spans_per_block: u32::MAX,
    }
}

const fn unbounded_builtin_caps() -> BuiltinCompileCaps {
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

const fn unbounded_graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: 10_000,
        maximum_edges: 10_000,
        maximum_schedule_items: 10_000,
        maximum_dependency_levels: 10_000,
        maximum_audio_buffer_samples: 10_000_000,
        maximum_delay_samples_per_edge: 1_000_000,
        maximum_total_delay_samples: 10_000_000,
        maximum_graph_bytes: 10_000_000,
        maximum_plan_bytes: 100_000_000,
        maximum_single_allocation_bytes: 10_000_000,
        maximum_finite_tail_samples: 10_000_000,
    }
}

fn main() {
    assert_eq!(
        std::env::args().count(),
        1,
        "benchmark accepts no arguments"
    );
    let metadata = Metadata::collect();
    let fixture_sha256 = sha256(INPUT_MANIFEST);
    let mut render_states = prepare_render_round_states();
    for rate_hz in RATES {
        warmup_prepare(rate_hz);
    }
    for (round_index, round) in ROUNDS.into_iter().enumerate() {
        for rate_hz in RATES {
            for workload in WORKLOADS {
                let measurement = if workload.is_prepare() {
                    measure_prepare(rate_hz)
                } else {
                    let state = render_states
                        .iter_mut()
                        .find(|state| state.workload == workload && state.rate_hz == rate_hz)
                        .expect("global render warmup state");
                    measure_render(&mut state.rounds[round_index])
                };
                println!(
                    "{}",
                    record_json(
                        workload,
                        rate_hz,
                        round,
                        &fixture_sha256,
                        &metadata,
                        &measurement
                    )
                );
            }
        }
    }
}

fn prepare_render_round_states() -> Vec<RenderRoundStates> {
    let mut states = Vec::with_capacity((WORKLOADS.len() - 1) * RATES.len());
    for workload in WORKLOADS {
        if workload.is_prepare() {
            continue;
        }
        for rate_hz in RATES {
            let mut first = RenderRuntime::new(workload, rate_hz);
            let mut second = RenderRuntime::new(workload, rate_hz);
            for batch in 0..RENDER_WARMUP_BATCHES {
                first.run_batch(batch as u64);
                second.run_batch(batch as u64);
            }
            states.push(RenderRoundStates {
                workload,
                rate_hz,
                rounds: [first, second],
            });
        }
    }
    states
}

fn measure_render(runtime: &mut RenderRuntime) -> Measurement {
    let mut samples_ns = Vec::with_capacity(RENDER_MEASURED_BATCHES);
    audit::warm_up();
    audit::reset();
    for batch in 0..RENDER_MEASURED_BATCHES {
        let mut batch_ns = 0_u64;
        for operation in 0..OPERATIONS_PER_RENDER_BATCH {
            let logical_batch = (RENDER_WARMUP_BATCHES + batch) as u64;
            runtime.prepare_operation_input(logical_batch, operation as u64);
            let started = Instant::now();
            runtime.run_operation(logical_batch, operation as u64);
            batch_ns = batch_ns.saturating_add(
                u64::try_from(started.elapsed().as_nanos()).expect("benchmark duration fits u64"),
            );
        }
        samples_ns.push(batch_ns / OPERATIONS_PER_RENDER_BATCH as u64);
    }
    let snapshot = audit::snapshot();
    assert_eq!(snapshot.total(), 0, "render audit must remain clean");
    Measurement {
        samples_ns,
        output_sha256: runtime.output_sha256(),
        shape: runtime.shape(),
        audit: Some(snapshot),
    }
}

fn warmup_prepare(rate_hz: u32) {
    for _ in 0..PREPARE_WARMUP_BATCHES {
        std::hint::black_box(prepare_256_tracks(rate_hz));
    }
}

fn measure_prepare(rate_hz: u32) -> Measurement {
    let mut samples_ns = Vec::with_capacity(PREPARE_MEASURED_BATCHES);
    let mut retained_payload_bytes = 0;
    let mut output = String::new();
    for _ in 0..PREPARE_MEASURED_BATCHES {
        let started = Instant::now();
        let prepared = prepare_256_tracks(rate_hz);
        let elapsed = started.elapsed();
        retained_payload_bytes = prepared
            .resource_report()
            .engine_owned_retained_payload_bytes;
        output =
            sha256(format!("{}:{retained_payload_bytes}", prepared.processor_count()).as_bytes());
        std::hint::black_box(prepared); // destruction is deliberately outside the elapsed interval.
        samples_ns.push(u64::try_from(elapsed.as_nanos()).expect("benchmark duration fits u64"));
    }
    Measurement {
        samples_ns,
        output_sha256: output,
        shape: WorkloadShape {
            tracks: PREPARE_TRACKS,
            meters: OBSERVERS * 8,
            meter_capacity: 4,
            retained_payload_bytes,
        },
        audit: None,
    }
}

struct RenderRuntime {
    workload: Workload,
    chain: BuiltinChain,
    left: [f32; QUANTUM],
    right: [f32; QUANTUM],
    meter_runtime: Option<RealMeterTapRuntime>,
}
impl RenderRuntime {
    fn new(workload: Workload, rate_hz: u32) -> Self {
        let mut parameters = BuiltinParameters::default();
        if matches!(workload, Workload::FullChainFilters) {
            parameters.left.hpf_hz = 100.0;
            parameters.right.hpf_hz = 200.0;
            parameters.left.lpf_hz = 1_000.0;
            parameters.right.lpf_hz = 2_000.0;
            parameters.left.trim_db = -3.0;
            parameters.right.trim_db = 2.0;
            parameters.left.fader_db = -1.0;
            parameters.right.fader_db = -4.0;
            parameters.matrix = Matrix2x2 {
                ll: 0.8,
                lr: 0.2,
                rl: -0.3,
                rr: 0.7,
            };
        }
        if matches!(workload, Workload::MatrixRamp) {
            parameters.matrix = Matrix2x2 {
                ll: 0.7,
                lr: 0.3,
                rl: -0.2,
                rr: 0.8,
            };
            parameters.smoothing_samples = QUANTUM as u32;
        }
        let meter_runtime = if matches!(workload, Workload::MeterSuccessFull) {
            Some(RealMeterTapRuntime::new(rate_hz))
        } else {
            None
        };
        Self {
            workload,
            chain: BuiltinChain::new(rate_hz, parameters).expect("frozen parameters"),
            left: [0.0; QUANTUM],
            right: [0.0; QUANTUM],
            meter_runtime,
        }
    }
    fn run_batch(&mut self, batch: u64) {
        for operation in 0..OPERATIONS_PER_RENDER_BATCH {
            let operation = operation as u64;
            self.prepare_operation_input(batch, operation);
            self.run_operation(batch, operation);
        }
    }
    fn prepare_operation_input(&mut self, batch: u64, operation: u64) {
        if self.meter_runtime.is_none() {
            self.fill_input(batch, operation);
        }
    }
    fn run_operation(&mut self, batch: u64, operation: u64) {
        audit::in_render_scope(|| self.render_one(batch, operation));
    }
    fn render_one(&mut self, batch: u64, operation: u64) {
        let first_sample =
            (batch * OPERATIONS_PER_RENDER_BATCH as u64 + operation) * QUANTUM as u64;
        if let Some(meter_runtime) = &mut self.meter_runtime {
            meter_runtime.render_one(first_sample);
            return;
        }
        if matches!(self.workload, Workload::MatrixRamp) {
            let target = if (batch + operation).is_multiple_of(2) {
                Matrix2x2 {
                    ll: 0.6,
                    lr: 0.4,
                    rl: -0.4,
                    rr: 0.6,
                }
            } else {
                Matrix2x2 {
                    ll: 0.9,
                    lr: -0.1,
                    rl: 0.2,
                    rr: 0.8,
                }
            };
            self.chain
                .set_matrix_target(target)
                .expect("frozen matrix target");
        }
        self.chain
            .process_dual_mono(
                DualMonoBlock::new(&mut self.left, &mut self.right, first_sample)
                    .expect("fixed block"),
            )
            .expect("frozen process");
    }
    fn fill_input(&mut self, batch: u64, operation: u64) {
        let phase = (batch * OPERATIONS_PER_RENDER_BATCH as u64 + operation) as f32 * 0.001;
        for (index, (left, right)) in self.left.iter_mut().zip(&mut self.right).enumerate() {
            let value = phase + index as f32 * 0.0001;
            *left = value;
            *right = -value * 0.75;
        }
    }
    fn output_sha256(&self) -> String {
        if let Some(meter_runtime) = &self.meter_runtime {
            return meter_runtime.output_sha256();
        }
        let mut hash = Sha256::new();
        for value in self.left.iter().chain(&self.right) {
            hash.update(value.to_bits().to_le_bytes());
        }
        hex_digest(hash.finalize())
    }
    fn shape(&self) -> WorkloadShape {
        WorkloadShape {
            tracks: 1,
            meters: if self.meter_runtime.is_some() {
                OBSERVERS * 2
            } else {
                0
            },
            meter_capacity: usize::from(self.meter_runtime.is_some()),
            retained_payload_bytes: 0,
        }
    }
}

fn prepare_256_tracks(rate_hz: u32) -> miso_engine_builtins_compiler::PreparedBuiltinsSession {
    let mut model = parse_session_toml(SESSION).expect("frozen session");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.limits.memory_bytes = u64::MAX;
    model.tracks.clear();
    model.tracks.reserve(PREPARE_TRACKS);
    for index in 0..PREPARE_TRACKS {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("benchmark-track-{index}")).expect("stable ID");
        model.tracks.push(track);
    }
    model.sample_rate_hz = rate_hz;
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("benchmark-track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
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
    .expect("prepared session");
    let config = MeterConfig {
        period_frames: NonZeroU32::new(QUANTUM as u32).expect("quantum"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).expect("capacity"),
        reset_generation: 7,
    };
    let taps = [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ];
    let requests: Vec<_> = (0..8)
        .flat_map(|track| {
            taps.into_iter()
                .enumerate()
                .map(move |(tap_index, tap)| MeterRequest {
                    handle: MeterHandle(
                        core::num::NonZeroU64::new(
                            u64::try_from(track * taps.len() + tap_index).expect("bounded") + 1,
                        )
                        .expect("nonzero"),
                    ),
                    track_id: format!("benchmark-track-{track}"),
                    tap,
                    config,
                })
        })
        .collect();
    prepare_session_builtins(
        &session,
        &requests,
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
    .expect("prepared workload")
}

struct Metadata {
    cpu_model: String,
    logical_cores: String,
    os: String,
    kernel: String,
    governor: String,
    rust_version: String,
    llvm_version: String,
    target: String,
    target_features: String,
    profile: String,
    opt_level: String,
    lto: String,
    codegen_units: String,
    background_load: String,
    missing: Vec<&'static str>,
}
impl Metadata {
    fn collect() -> Self {
        let mut missing = Vec::new();
        let take = |key: &'static str, missing: &mut Vec<&'static str>| match std::env::var(key) {
            Ok(value) if !value.is_empty() => json_safe(&value),
            _ => {
                missing.push(key);
                "unknown".to_owned()
            }
        };
        let cpu_model = take("MISO_ENGINE_BENCH_CPU_MODEL", &mut missing);
        let logical_cores = take("MISO_ENGINE_BENCH_LOGICAL_CORES", &mut missing);
        let os = take("MISO_ENGINE_BENCH_OS", &mut missing);
        let kernel = take("MISO_ENGINE_BENCH_KERNEL", &mut missing);
        let governor = take("MISO_ENGINE_BENCH_GOVERNOR", &mut missing);
        let rust_version = take("MISO_ENGINE_BENCH_RUST_VERSION", &mut missing);
        let llvm_version = take("MISO_ENGINE_BENCH_LLVM_VERSION", &mut missing);
        let target = take("MISO_ENGINE_BENCH_TARGET", &mut missing);
        let target_features = take("MISO_ENGINE_BENCH_TARGET_FEATURES", &mut missing);
        let profile = take("MISO_ENGINE_BENCH_PROFILE", &mut missing);
        let opt_level = take("MISO_ENGINE_BENCH_OPT_LEVEL", &mut missing);
        let lto = take("MISO_ENGINE_BENCH_LTO", &mut missing);
        let codegen_units = take("MISO_ENGINE_BENCH_CODEGEN_UNITS", &mut missing);
        let background_load = take("MISO_ENGINE_BENCH_BACKGROUND_LOAD", &mut missing);
        missing.sort_unstable();
        Self {
            cpu_model,
            logical_cores,
            os,
            kernel,
            governor,
            rust_version,
            llvm_version,
            target,
            target_features,
            profile,
            opt_level,
            lto,
            codegen_units,
            background_load,
            missing,
        }
    }
}

fn record_json(
    workload: Workload,
    rate_hz: u32,
    round: u32,
    fixture_sha256: &str,
    metadata: &Metadata,
    measurement: &Measurement,
) -> String {
    let p = Percentiles::from_samples(&measurement.samples_ns);
    let shape = measurement.shape;
    let missing = metadata
        .missing
        .iter()
        .map(|key| format!("\"{key}\""))
        .collect::<Vec<_>>()
        .join(",");
    let render_fields = match measurement.audit { Some(a) => format!("\"render_scope\":\"render\",\"render_allocations\":{},\"render_deallocations\":{},\"render_locks\":{},\"render_logs\":{},\"render_file_io\":{},\"render_network_io\":{},\"render_syscalls\":{}", a.allocations, a.deallocations, a.locks, a.logs, a.file_io, a.network_io, a.syscalls), None => "\"render_scope\":\"not_applicable_preparation\",\"render_allocations\":\"not_applicable\",\"render_deallocations\":\"not_applicable\",\"render_locks\":\"not_applicable\",\"render_logs\":\"not_applicable\",\"render_file_io\":\"not_applicable\",\"render_network_io\":\"not_applicable\",\"render_syscalls\":\"not_applicable\"".to_owned() };
    format!(
        concat!(
            "{{\"schema_version\":2,\"issue\":{},\"workload_kind\":\"{}\",\"workload_id\":\"issue035.{}.{}hz.q128\",\"sample_rate_hz\":{},\"quantum_frames\":128,\"round\":{},\"warmup_batches\":{},\"measured_batches\":{},\"operations_per_batch\":{},\"frames_per_operation\":128,\"tracks\":{},\"meter_observers\":{},\"meter_queue_capacity\":{},\"retained_payload_bytes\":{},\"percentile_method\":\"nearest_rank\",\"min_ns\":{},\"p50_ns\":{},\"p95_ns\":{},\"p99_ns\":{},\"p99_9_ns\":{},\"max_ns\":{},\"fixture_manifest_id\":\"fixtures/builtins/v1/MANIFEST.tsv\",\"fixture_manifest_sha256\":\"{}\",\"input_fixture_id\":\"fixtures/builtins/v1/MANIFEST.tsv\",\"input_fixture_sha256\":\"{}\",\"output_sha256\":\"{}\",{},\"cpu_model\":\"{}\",\"logical_cores\":\"{}\",\"os\":\"{}\",\"kernel\":\"{}\",\"governor_or_power_mode\":\"{}\",\"rust_version\":\"{}\",\"llvm_version\":\"{}\",\"target_triple\":\"{}\",\"target_features\":\"{}\",\"profile\":\"{}\",\"opt_level\":\"{}\",\"lto\":\"{}\",\"codegen_units\":\"{}\",\"background_load_note\":\"{}\",\"missing_metadata\":[{}]}}"
        ),
        ISSUE,
        workload.kind(),
        workload.kind(),
        rate_hz,
        rate_hz,
        round,
        if workload.is_prepare() {
            PREPARE_WARMUP_BATCHES
        } else {
            RENDER_WARMUP_BATCHES
        },
        if workload.is_prepare() {
            PREPARE_MEASURED_BATCHES
        } else {
            RENDER_MEASURED_BATCHES
        },
        if workload.is_prepare() {
            1
        } else {
            OPERATIONS_PER_RENDER_BATCH
        },
        shape.tracks,
        shape.meters,
        shape.meter_capacity,
        shape.retained_payload_bytes,
        p.min,
        p.p50,
        p.p95,
        p.p99,
        p.p999,
        p.max,
        fixture_sha256,
        fixture_sha256,
        measurement.output_sha256,
        render_fields,
        metadata.cpu_model,
        metadata.logical_cores,
        metadata.os,
        metadata.kernel,
        metadata.governor,
        metadata.rust_version,
        metadata.llvm_version,
        metadata.target,
        metadata.target_features,
        metadata.profile,
        metadata.opt_level,
        metadata.lto,
        metadata.codegen_units,
        metadata.background_load,
        missing
    )
}

struct Percentiles {
    min: u64,
    p50: u64,
    p95: u64,
    p99: u64,
    p999: u64,
    max: u64,
}
impl Percentiles {
    fn from_samples(samples: &[u64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable();
        assert!(!sorted.is_empty(), "measured batches");
        let rank = |n: usize, d: usize| sorted[(sorted.len() * n).div_ceil(d).saturating_sub(1)];
        Self {
            min: sorted[0],
            p50: rank(50, 100),
            p95: rank(95, 100),
            p99: rank(99, 100),
            p999: rank(999, 1000),
            max: *sorted.last().expect("nonempty"),
        }
    }
}
fn sha256(bytes: &[u8]) -> String {
    hex_digest(Sha256::digest(bytes))
}
fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    use core::fmt::Write;
    let mut output = String::with_capacity(64);
    for byte in bytes.as_ref() {
        write!(&mut output, "{byte:02x}").expect("String write");
    }
    output
}
fn json_safe(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_graphic() && *character != '"' && *character != '\\')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_meter_tap_plans_use_the_compiled_seven_taps_and_preserve_full_queue_state() {
        let pair = prepare_real_meter_tap_artifacts(48_000);
        let mut success = RealMeterTapPlan::bind(pair.success);
        let mut full = RealMeterTapPlan::bind(pair.full);
        let expected_taps = [
            MeterTap::Input,
            MeterTap::PostInputBuiltins,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostSimd2PreFader,
            MeterTap::PostFader,
            MeterTap::PostMatrix,
        ];

        assert_eq!(success.consumers.len(), OBSERVERS);
        assert_eq!(full.consumers.len(), OBSERVERS);
        assert_eq!(
            success
                .consumers
                .iter()
                .map(|consumer| consumer.tap)
                .collect::<Vec<_>>(),
            expected_taps
        );
        assert_eq!(
            full.consumers
                .iter()
                .map(|consumer| consumer.tap)
                .collect::<Vec<_>>(),
            expected_taps
        );

        full.render(0);
        full.render(QUANTUM as u64);
        let full_windows = full.drain_all();
        assert!(
            full_windows
                .iter()
                .all(|record| record.snapshot.end_sample == QUANTUM as u64)
        );

        success.render(QUANTUM as u64);
        let success_windows = success.drain_all();
        assert!(
            success_windows
                .iter()
                .all(|record| record.snapshot.end_sample == QUANTUM as u64 * 2)
        );
    }
}
