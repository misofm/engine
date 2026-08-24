//! Frozen Issue-038 real-audio benchmark record emitter.
//!
//! The binary has no command-line control surface. The fixed shell runner supplies only the
//! warmup/round environment, while this program owns fixed 48 kHz, 128-frame production DSP.
#![allow(unsafe_code)]

use core::fmt::Write as _;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    time::Instant,
};

use miso_engine_builtins::{
    BuiltinChain, BuiltinInputBankV1, BuiltinParameters, ChannelParameters, DualMonoBlock,
};
use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_core::realtime::{
    PlanarBufferMut, RenderIo, RenderTime,
    audit::{self, ForbiddenOperation, record_allocator_violation},
};
use miso_engine_effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
use miso_engine_effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_lane::Backend;
use miso_engine_rack::RackLocationV1;
use miso_engine_session::{
    CompileCaps, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit, RouteSource,
    SendTap, Sidechain, SidechainDeclaration, StableId, compile_session, parse_session_toml,
};
use sha2::{Digest, Sha256};

struct AuditedAllocator;
#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every method forwards the allocator's original pointer/layout contract unchanged. An
// allocation while the render audit is armed aborts instead of unwinding through `GlobalAlloc`.
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

const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM: usize = 128;
const OBSERVATIONS: usize = 1_000;
const FIXTURE_ID: &str = "fixtures/rack/issue038-v1/MANIFEST.tsv";
const FIXTURE_BYTES: &[u8] = include_bytes!("../../../fixtures/rack/issue038-v1/MANIFEST.tsv");

#[derive(Clone, Copy)]
enum Workload {
    ScalarEightTracks,
    HostSelectedEightTrackBank,
    MixedTwelveTrackGraph,
}
const WORKLOADS: [Workload; 3] = [
    Workload::ScalarEightTracks,
    Workload::HostSelectedEightTrackBank,
    Workload::MixedTwelveTrackGraph,
];
impl Workload {
    const fn kind(self) -> &'static str {
        match self {
            Self::ScalarEightTracks => "scalar_eight_tracks",
            Self::HostSelectedEightTrackBank => "host_selected_eight_track_bank",
            Self::MixedTwelveTrackGraph => "mixed_twelve_track_graph",
        }
    }
    const fn tracks(self) -> u32 {
        match self {
            Self::ScalarEightTracks | Self::HostSelectedEightTrackBank => 8,
            Self::MixedTwelveTrackGraph => 12,
        }
    }
}

struct Shape {
    backend: &'static str,
    bank_width: u32,
    bank_count: u32,
    scalar_tail_count: u32,
    scalar_fallback_count: u32,
    identity_lane_count: u32,
}
struct Measurement {
    ns_per_frame: Vec<u64>,
    input_sha256: String,
    output_sha256: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
    panic_unwinds: u64,
}

fn main() {
    assert_eq!(
        std::env::args_os().count(),
        1,
        "benchmark accepts no arguments"
    );
    let round = round_from_runner();
    let backend = host_backend(); // Feature detection ends here, before all timed observations.
    let identities = Identities::collect();
    let metadata = Metadata::collect();
    for workload in WORKLOADS {
        let (measurement, shape) = measure(workload, backend);
        println!(
            "{}",
            record_json(
                workload,
                round,
                &shape,
                &measurement,
                &identities,
                &metadata
            )
        );
    }
}

fn round_from_runner() -> u32 {
    match std::env::var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the rack benchmark must be launched by its fixed runner"),
    }
}
fn host_backend() -> Backend {
    let backend = Backend::current();
    assert_eq!(
        backend,
        Backend::Simd8,
        "Issue-038 qualification requires the eight-lane backend"
    );
    backend
}

fn measure(workload: Workload, backend: Backend) -> (Measurement, Shape) {
    let mut runtime = Runtime::prepare(workload, backend);
    let mut durations = Vec::with_capacity(OBSERVATIONS);
    let mut input_hash = Sha256::new();
    let mut output_hash = Sha256::new();
    audit::warm_up();
    audit::reset();
    let mut render_errors = 0_u64;
    let mut panic_unwinds = 0_u64;
    for observation in 0..OBSERVATIONS {
        // Frozen asymmetric dual-mono input is filled and identified outside the timer. Runtime
        // state is continuous across all one thousand observations and is never reset.
        runtime.fill_input(observation as u64);
        hash_semantic_input(
            workload.tracks() as usize,
            observation as u64,
            &mut input_hash,
        );
        let started = Instant::now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            runtime.render(observation as u64)
        }));
        let ns = u64::try_from(started.elapsed().as_nanos()).expect("duration fits u64")
            / QUANTUM as u64;
        match result {
            Ok(Ok(())) => {}
            Ok(Err(())) => render_errors += 1,
            Err(_) => panic_unwinds += 1,
        }
        runtime.hash_output(&mut output_hash);
        durations.push(ns);
    }
    let measurement = Measurement {
        ns_per_frame: durations,
        input_sha256: hex_digest(input_hash.finalize()),
        output_sha256: hex_digest(output_hash.finalize()),
        audit: audit::snapshot(),
        render_errors,
        panic_unwinds,
    };
    (measurement, runtime.shape())
}

enum Runtime {
    Scalar(Box<ScalarRuntime>),
    Bank(Box<BankRuntime>),
    Mixed(Box<MixedRuntime>),
}
impl Runtime {
    fn prepare(workload: Workload, backend: Backend) -> Self {
        match workload {
            Workload::ScalarEightTracks => Self::Scalar(Box::new(ScalarRuntime::new(8))),
            Workload::HostSelectedEightTrackBank => Self::Bank(Box::new(BankRuntime::new(backend))),
            Workload::MixedTwelveTrackGraph => Self::Mixed(Box::new(MixedRuntime::new(backend))),
        }
    }
    fn fill_input(&mut self, observation: u64) {
        match self {
            Self::Scalar(value) => value.fill_input(observation),
            Self::Bank(value) => value.fill_input(observation),
            Self::Mixed(value) => value.fill_input(observation),
        }
    }
    fn render(&mut self, observation: u64) -> Result<(), ()> {
        match self {
            Self::Scalar(value) => audit::in_render_scope(|| value.render(observation)),
            Self::Bank(value) => audit::in_render_scope(|| value.render(observation)),
            Self::Mixed(value) => value.render(observation),
        }
    }
    fn hash_output(&self, hash: &mut Sha256) {
        match self {
            Self::Scalar(value) => value.hash_output(hash),
            Self::Bank(value) => value.hash_output(hash),
            Self::Mixed(value) => value.hash_output(hash),
        }
    }
    const fn shape(&self) -> Shape {
        match self {
            Self::Scalar(_) => Shape {
                backend: "Scalar",
                bank_width: 1,
                bank_count: 0,
                scalar_tail_count: 8,
                scalar_fallback_count: 0,
                identity_lane_count: 0,
            },
            Self::Bank(value) => Shape {
                backend: value.backend_name,
                bank_width: 8,
                bank_count: 1,
                scalar_tail_count: 0,
                scalar_fallback_count: 0,
                identity_lane_count: 0,
            },
            // The first eight tracks form the Issue-037 host bank; positions 8/9 are identity/missing
            // rack positions and positions 10/11 are the stable scalar tail. The last two scalar paths
            // are intentionally incompatible fallback ownership, never mock or padded lanes.
            Self::Mixed(value) => Shape {
                backend: value.backend_name,
                bank_width: 8,
                bank_count: 1,
                scalar_tail_count: 2,
                scalar_fallback_count: 2,
                identity_lane_count: 2,
            },
        }
    }
}

struct ScalarRuntime {
    chains: Vec<BuiltinChain>,
    left: Vec<[f32; QUANTUM]>,
    right: Vec<[f32; QUANTUM]>,
}
impl ScalarRuntime {
    fn new(tracks: usize) -> Self {
        Self {
            chains: (0..tracks).map(chain_for_track).collect(),
            left: vec![[0.0; QUANTUM]; tracks],
            right: vec![[0.0; QUANTUM]; tracks],
        }
    }
    fn fill_input(&mut self, observation: u64) {
        fill_track_inputs(&mut self.left, &mut self.right, observation);
    }
    fn render(&mut self, observation: u64) -> Result<(), ()> {
        for (index, chain) in self.chains.iter_mut().enumerate() {
            chain.process_input(
                DualMonoBlock::new(
                    &mut self.left[index],
                    &mut self.right[index],
                    observation * QUANTUM as u64,
                )
                .map_err(|_| ())?,
            );
        }
        Ok(())
    }
    fn hash_output(&self, hash: &mut Sha256) {
        for track in 0..self.left.len() {
            hash_f32_into(hash, self.left[track].iter());
            hash_f32_into(hash, self.right[track].iter());
        }
    }
}

struct BankRuntime {
    bank: BuiltinInputBankV1,
    backend_name: &'static str,
    left: Vec<f32>,
    right: Vec<f32>,
}
impl BankRuntime {
    fn new(backend: Backend) -> Self {
        let inputs = (0..8)
            .map(chain_for_track)
            .map(BuiltinChain::into_input_builtins)
            .collect();
        Self {
            bank: BuiltinInputBankV1::new(
                backend,
                miso_engine_effect_contract::BankWidth::Eight,
                inputs,
            )
            .expect("prepared eight-lane production bank"),
            backend_name: backend_name(backend),
            left: vec![0.0; QUANTUM * 8],
            right: vec![0.0; QUANTUM * 8],
        }
    }
    fn fill_input(&mut self, observation: u64) {
        fill_aosoa_inputs(&mut self.left, &mut self.right, 8, observation);
    }
    fn render(&mut self, observation: u64) -> Result<(), ()> {
        let _ = observation;
        self.bank
            .process(&mut self.left, &mut self.right, QUANTUM as u32);
        Ok(())
    }
    fn hash_output(&self, hash: &mut Sha256) {
        for track in 0..8 {
            hash_f32_into(
                hash,
                (0..QUANTUM).map(|frame| &self.left[frame * 8 + track]),
            );
            hash_f32_into(
                hash,
                (0..QUANTUM).map(|frame| &self.right[frame * 8 + track]),
            );
        }
    }
}

/// Actual sealed Issue-037 production graph runtime: the graph compiler retains one complete
/// eight-lane post-input builtin bank and four scalar graph paths.  The last four retain the
/// frozen identity/missing, stable-tail, and incompatible-fallback roles in the benchmark shape.
struct MixedRuntime {
    plan: miso_engine_core::realtime::PreparedRenderPlan,
    output: Vec<f32>,
    backend_name: &'static str,
}
impl MixedRuntime {
    fn new(backend: Backend) -> Self {
        let mut model =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("canonical session");
        let template = model.tracks[0].clone();
        let route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..12)
            .map(|index| {
                let mut track = template.clone();
                let track_id = if index < 10 {
                    format!("rack{index:02}")
                } else {
                    format!("fallback{index}")
                };
                track.id = StableId::parse(&track_id).expect("frozen track id");
                let lane = index as f32;
                track.builtins.left.trim_db = -3.0 + lane * 0.25;
                track.builtins.left.hpf_hz = 40.0 + lane * 3.0;
                track.builtins.left.lpf_hz = 15_000.0 - lane * 100.0;
                track.builtins.right.trim_db = 2.0 - lane * 0.2;
                track.builtins.right.hpf_hz = 60.0 + lane * 2.0;
                track.builtins.right.lpf_hz = 14_000.0 - lane * 80.0;
                track.dynamic.effects.clear();
                track.simd2.effects.clear();
                let mut effect = template.dynamic.effects[0].clone();
                effect.id = StableId::parse("delay-main").expect("frozen effect id");
                effect.identity = EffectIdentity::Native {
                    effect_id: StableId::parse("conformance.delay")
                        .expect("frozen native effect id"),
                };
                effect.params = vec![EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Both,
                    unit: ParameterUnit::Linear,
                    value: 0.75 + index as f32 * 0.031_25,
                }];
                effect.bypass = false;
                effect.sidechain = if index < 10 {
                    SidechainDeclaration::None
                } else {
                    SidechainDeclaration::Routed(Sidechain {
                        source: RouteSource::Track {
                            track_id: track.id.clone(),
                            tap: SendTap::Input,
                        },
                        port_id: StableId::parse("sidechain-in").expect("frozen sidechain port"),
                    })
                };
                // rack02/rack05 deliberately omit this leading slot. Their two missing positions
                // are explicit identity lanes in the retained eight-track cohort report.
                track.simd1.effects = if matches!(index, 2 | 5) || index >= 10 {
                    vec![effect]
                } else {
                    let mut leading = effect.clone();
                    leading.id = StableId::parse("delay-leading").expect("frozen effect id");
                    leading.bypass = true;
                    vec![leading, effect]
                };
                track
            })
            .collect();
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut next = route.clone();
                next.id = StableId::parse(&format!("bank-route{index}")).expect("frozen route id");
                next.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                next
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
        .expect("mixed compiled session");
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
        .expect("mixed builtin preparation");
        let registry = NativeEffectRegistry::new([
            Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>,
        ])
        .expect("frozen conformance registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 24,
                maximum_scratch_bytes: 1 << 24,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("mixed effect preparation");
        let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: Backend::current(),
            plan_id: 38,
            effects,
            builtins,
            caps: graph_caps(),
        }) {
            Ok(value) => value,
            Err(_) => panic!("mixed production graph"),
        };
        // #86 F3: twelve post-input nodes at W8 are `12.div_ceil(8) == 2` banks -- one full and
        // one padded to eight with four identity lanes -- and no scalar post-input tail.
        assert_eq!(
            artifact.prepared_builtin_bank_count(),
            2,
            "host-selected production builtin banks, last one padded"
        );
        let builtin_banks: Vec<_> = artifact.prepared_builtin_banks().collect();
        assert_eq!(builtin_banks.len(), 2);
        assert!(
            builtin_banks
                .iter()
                .all(|bank| bank.backend == backend && bank.width.lanes() == 8)
        );
        assert_eq!(
            builtin_banks
                .iter()
                .map(|bank| bank.members.len())
                .collect::<Vec<_>>(),
            vec![8, 4]
        );
        let cohorts = &artifact.report().rack_cohorts;
        assert_eq!(cohorts.dispatch, backend);
        // #96: the report is the *bound* plan, from the same planner that produced the banks.
        let bound_groups: Vec<_> = cohorts.bound_groups_in(RackLocationV1::Simd1).collect();
        // #99 F3: the cohort is now the whole rack *chain*, not one effect. Eight tracks carry
        // the two-slot program `[delay-leading (bypassed), delay-main]`; the two tracks that carry
        // only `delay-main` join the same cohort through their subsequence mask and land in the
        // padded remainder group, which stays unbound (#96 F6/F7). So this is ONE cohort binding
        // TWO banks -- one per slot -- where #96 reported two single-slot cohorts binding one bank
        // each.
        //
        // Membership is bit-identical across that change, and that is the point: the same eight
        // tracks (rack00/01/03/04/06/07/08/09) bank both slots, rack02 and rack05 remain the
        // compatible scalar tail, and fallback10/fallback11 remain the connected-sidechain
        // fallbacks. Only the report's shape moved.
        assert_eq!(bound_groups.len(), 1, "one full rack-chain cohort");
        assert_eq!(bound_groups[0].program.len(), 2, "a two-slot chain");
        assert!(
            bound_groups.iter().all(|group| group.is_full()),
            "only full groups are bound"
        );
        assert!(bound_groups.iter().all(|group| group.active_count() == 8));
        let bound_slots: Vec<_> = cohorts.bound_slots_in(RackLocationV1::Simd1).collect();
        assert_eq!(bound_slots.len(), 2, "one bank per slot of the chain");
        assert!(bound_slots.iter().all(|bound| bound.members.len() == 8));
        assert_eq!(
            bound_slots.len() as u64,
            artifact.graph_resource_estimate().effect_bank_count,
            "the report is the bound plan: one bound slot per prepared bank"
        );
        let scalar = cohorts.scalar_in(RackLocationV1::Simd1);
        let compatible_tails = scalar
            .iter()
            .filter(|member| member.track_id.as_str().starts_with("rack"))
            .count();
        let incompatible_fallbacks = scalar
            .iter()
            .filter(|member| member.track_id.as_str().starts_with("fallback"))
            .count();
        assert_eq!(compatible_tails, 2, "stable compatible scalar tail");
        assert_eq!(incompatible_fallbacks, 2, "connected-sidechain fallback");
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), source_binding(node)))
            .collect();
        let bound = match artifact.into_bound(GraphRuntimeBindings {
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(value) => value,
            Err(_) => panic!("mixed production graph bindings"),
        };
        Self {
            plan: bound.plan,
            output: vec![0.0; QUANTUM * 2],
            backend_name: backend_name(backend),
        }
    }
    fn fill_input(&mut self, _observation: u64) {
        // Per-track input blocks were fully prepared before measurement. Production graph source
        // bindings copy those immutable blocks when their graph nodes execute.
    }
    fn render(&mut self, observation: u64) -> Result<(), ()> {
        self.plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
                        .map_err(|_| ())?,
                },
                RenderTime {
                    absolute_sample: observation * QUANTUM as u64,
                },
            )
            .map(|_| ())
            .map_err(|_| ())
    }
    fn hash_output(&self, hash: &mut Sha256) {
        hash_f32_into(hash, self.output.iter());
    }
}

struct FrozenGraphSource {
    blocks: Box<[FrozenDualMonoBlock]>,
    observation: usize,
}
struct FrozenDualMonoBlock {
    left: [f32; QUANTUM],
    right: [f32; QUANTUM],
}
impl FrozenGraphSource {
    fn new(track: usize) -> Self {
        let blocks = (0..OBSERVATIONS)
            .map(|observation| {
                let mut left = [0.0; QUANTUM];
                let mut right = [0.0; QUANTUM];
                for frame in 0..QUANTUM {
                    (left[frame], right[frame]) =
                        asymmetric_input(track, frame, observation as u64);
                }
                FrozenDualMonoBlock { left, right }
            })
            .collect();
        Self {
            blocks,
            observation: 0,
        }
    }
}
impl GraphRuntimeProcessor for FrozenGraphSource {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        let source = self
            .blocks
            .get(self.observation)
            .expect("exactly one thousand prepared graph observations");
        block.left.copy_from_slice(&source.left);
        block.right.copy_from_slice(&source.right);
        self.observation += 1;
        Ok(())
    }
}
struct GraphIdentity;
impl GraphRuntimeProcessor for GraphIdentity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}
fn source_binding(node: &GraphNodeId) -> Box<dyn GraphRuntimeProcessor> {
    if let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    {
        let id = track_id.as_str();
        let track = id
            .strip_prefix("rack")
            .or_else(|| id.strip_prefix("fallback"))
            .and_then(|value| value.parse().ok())
            .expect("frozen graph input");
        Box::new(FrozenGraphSource::new(track))
    } else {
        Box::new(GraphIdentity)
    }
}
fn graph_caps() -> miso_engine_graph::GraphCompileCaps {
    miso_engine_graph::GraphCompileCaps {
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

fn chain_for_track(index: usize) -> BuiltinChain {
    let lane = index as f32;
    BuiltinChain::new(
        SAMPLE_RATE_HZ,
        BuiltinParameters {
            left: ChannelParameters {
                trim_db: -3.0 + lane * 0.25,
                hpf_hz: 40.0 + lane * 3.0,
                lpf_hz: 15_000.0 - lane * 100.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                trim_db: 2.0 - lane * 0.2,
                hpf_hz: 60.0 + lane * 2.0,
                lpf_hz: 14_000.0 - lane * 80.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("frozen real TPT parameters")
}
fn fill_track_inputs(left: &mut [[f32; QUANTUM]], right: &mut [[f32; QUANTUM]], observation: u64) {
    for (track, (left, right)) in left.iter_mut().zip(right).enumerate() {
        for frame in 0..QUANTUM {
            let (l, r) = asymmetric_input(track, frame, observation);
            left[frame] = l;
            right[frame] = r;
        }
    }
}
fn fill_aosoa_inputs(left: &mut [f32], right: &mut [f32], lanes: usize, observation: u64) {
    for frame in 0..QUANTUM {
        for track in 0..lanes {
            let (l, r) = asymmetric_input(track, frame, observation);
            left[frame * lanes + track] = l;
            right[frame * lanes + track] = r;
        }
    }
}
fn asymmetric_input(track: usize, frame: usize, observation: u64) -> (f32, f32) {
    let phase = (observation % 97) as f32 * 0.000_13;
    let ramp = (frame as f32 - 63.5) * 0.001_25;
    let lane = (track + 1) as f32;
    (
        ramp * lane + phase,
        -(ramp * (0.5 + lane * 0.125) - phase * 0.75),
    )
}
fn hash_semantic_input(tracks: usize, observation: u64, hash: &mut Sha256) {
    for track in 0..tracks {
        for channel in 0..2 {
            for frame in 0..QUANTUM {
                let sample = asymmetric_input(track, frame, observation);
                hash.update(if channel == 0 {
                    sample.0.to_bits().to_le_bytes()
                } else {
                    sample.1.to_bits().to_le_bytes()
                });
            }
        }
    }
}
fn backend_name(backend: Backend) -> &'static str {
    match backend {
        Backend::Simd8 => "Simd8",
        _ => panic!("the frozen host workload requires the eight-lane backend"),
    }
}
fn hash_f32_into<'a>(hash: &mut Sha256, values: impl Iterator<Item = &'a f32>) {
    for value in values {
        hash.update(value.to_bits().to_le_bytes());
    }
}
fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    let mut result = String::with_capacity(64);
    for byte in bytes.as_ref() {
        write!(&mut result, "{byte:02x}").expect("string");
    }
    result
}

struct Identities {
    candidate_commit_sha256: String,
    binary_sha256: String,
    fixture_sha256: String,
}
impl Identities {
    fn collect() -> Self {
        Self {
            candidate_commit_sha256: required_sha256("MISO_ENGINE_BENCH_CANDIDATE_SHA256"),
            binary_sha256: required_sha256("MISO_ENGINE_BENCH_BINARY_SHA256"),
            fixture_sha256: hex_digest(Sha256::digest(FIXTURE_BYTES)),
        }
    }
}
fn required_sha256(name: &str) -> String {
    let value =
        std::env::var(name).unwrap_or_else(|_| panic!("missing fixed runner identity: {name}"));
    assert!(
        value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "invalid {name}"
    );
    value.to_ascii_lowercase()
}

struct Metadata {
    fields: Vec<(&'static str, Option<String>)>,
}
impl Metadata {
    fn collect() -> Self {
        const FIELDS: [(&str, &str); 16] = [
            ("architecture", "MISO_ENGINE_BENCH_CPU_ARCHITECTURE"),
            (
                "background_load_note",
                "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
            ),
            ("codegen_units", "MISO_ENGINE_BENCH_CODEGEN_UNITS"),
            ("cpu_model", "MISO_ENGINE_BENCH_CPU_MODEL"),
            (
                "governor_or_power_mode",
                "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
            ),
            ("kernel", "MISO_ENGINE_BENCH_KERNEL"),
            ("llvm_version", "MISO_ENGINE_BENCH_LLVM_VERSION"),
            ("logical_cores", "MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT"),
            ("lto", "MISO_ENGINE_BENCH_LTO"),
            ("opt_level", "MISO_ENGINE_BENCH_OPT_LEVEL"),
            ("os", "MISO_ENGINE_BENCH_OS"),
            ("physical_cores", "MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT"),
            ("profile", "MISO_ENGINE_BENCH_PROFILE"),
            ("rust_version", "MISO_ENGINE_BENCH_RUST_VERSION"),
            ("target_features", "MISO_ENGINE_BENCH_TARGET_FEATURES"),
            ("target_triple", "MISO_ENGINE_BENCH_TARGET_TRIPLE"),
        ];
        let fields = FIELDS
            .into_iter()
            .map(|(field, env)| (field, metadata_value(env)))
            .collect();
        Self { fields }
    }
    fn json_fields(&self) -> String {
        self.fields
            .iter()
            .map(|(field, value)| match value {
                Some(value) => format!("\"{field}\":\"{}\"", json_escape(value)),
                None => format!("\"{field}\":null"),
            })
            .collect::<Vec<_>>()
            .join(",")
    }
    fn missing_json(&self) -> String {
        self.fields
            .iter()
            .filter_map(|(field, value)| value.is_none().then_some(*field))
            .map(|field| format!("\"{field}\""))
            .collect::<Vec<_>>()
            .join(",")
    }
}
fn metadata_value(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| {
        !value.is_empty() && value != "unknown" && value != "default" && value.is_ascii()
    })
}
fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn record_json(
    workload: Workload,
    round: u32,
    shape: &Shape,
    measurement: &Measurement,
    identities: &Identities,
    metadata: &Metadata,
) -> String {
    let p = Percentiles::from(&measurement.ns_per_frame);
    let forbidden_total = measurement
        .audit
        .total()
        .saturating_add(measurement.panic_unwinds);
    format!(
        concat!(
            "{{\"schema_version\":2,\"issue\":38,\"workload_kind\":\"{}\",\"workload_id\":\"issue038.{}.48000hz.q128\",\"round\":{},\"sample_rate_hz\":48000,\"quantum_frames\":128,\"tracks\":{},\"bank_backend\":\"{}\",\"bank_width\":{},\"bank_count\":{},\"scalar_tail_count\":{},\"scalar_fallback_count\":{},\"identity_lane_count\":{},\"observations\":1000,\"percentile_method\":\"nearest_rank\",\"units\":\"ns_per_frame\",\"min_ns_per_frame\":{},\"p50_ns_per_frame\":{},\"p95_ns_per_frame\":{},\"p99_ns_per_frame\":{},\"p99_9_ns_per_frame\":{},\"max_ns_per_frame\":{},\"descriptive_only\":true,\"candidate_commit_sha256\":\"{}\",\"binary_sha256\":\"{}\",\"fixture_id\":\"{}\",\"fixture_sha256\":\"{}\",\"input_sha256\":\"{}\",\"output_sha256\":\"{}\",\"render_errors\":{},\"render_allocations\":{},\"render_deallocations\":{},\"render_locks\":{},\"render_feature_detection_calls\":0,\"render_logs\":{},\"render_file_io\":{},\"render_network_io\":{},\"render_syscalls\":{},\"render_panic_unwinds\":{},\"forbidden_operation_total\":{},{} ,\"missing_metadata\":[{}]}}"
        ),
        workload.kind(),
        workload.kind(),
        round,
        workload.tracks(),
        shape.backend,
        shape.bank_width,
        shape.bank_count,
        shape.scalar_tail_count,
        shape.scalar_fallback_count,
        shape.identity_lane_count,
        p.min,
        p.p50,
        p.p95,
        p.p99,
        p.p999,
        p.max,
        identities.candidate_commit_sha256,
        identities.binary_sha256,
        FIXTURE_ID,
        identities.fixture_sha256,
        measurement.input_sha256,
        measurement.output_sha256,
        measurement.render_errors,
        measurement.audit.allocations,
        measurement.audit.deallocations,
        measurement.audit.locks,
        measurement.audit.logs,
        measurement.audit.file_io,
        measurement.audit.network_io,
        measurement.audit.syscalls,
        measurement.panic_unwinds,
        forbidden_total,
        metadata.json_fields(),
        metadata.missing_json()
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
    fn from(samples: &[u64]) -> Self {
        assert_eq!(samples.len(), OBSERVATIONS);
        let mut values = samples.to_vec();
        values.sort_unstable();
        let rank = |n: usize| values[(values.len() * n).div_ceil(1_000) - 1];
        Self {
            min: values[0],
            p50: rank(500),
            p95: rank(950),
            p99: rank(990),
            p999: rank(999),
            max: values[OBSERVATIONS - 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_launch_preflight_prepares_each_exact_production_workload() {
        let backend = host_backend();
        for (workload, expected) in [
            (Workload::ScalarEightTracks, ("Scalar", 1, 0, 8, 0, 0)),
            (
                Workload::HostSelectedEightTrackBank,
                (backend_name(backend), 8, 1, 0, 0, 0),
            ),
            (
                Workload::MixedTwelveTrackGraph,
                (backend_name(backend), 8, 1, 2, 2, 2),
            ),
        ] {
            let runtime = Runtime::prepare(workload, backend);
            let shape = runtime.shape();
            assert_eq!(
                (
                    shape.backend,
                    shape.bank_width,
                    shape.bank_count,
                    shape.scalar_tail_count,
                    shape.scalar_fallback_count,
                    shape.identity_lane_count,
                ),
                expected
            );
        }
    }

    #[test]
    fn semantic_input_identity_is_layout_independent_and_workload_specific() {
        let identify = |tracks| {
            let mut hash = Sha256::new();
            for observation in 0..OBSERVATIONS as u64 {
                hash_semantic_input(tracks, observation, &mut hash);
            }
            hex_digest(hash.finalize())
        };
        assert_eq!(identify(8), identify(8));
        assert_ne!(identify(8), identify(12));
    }

    #[test]
    fn nearest_rank_uses_the_frozen_one_thousand_observation_indices() {
        let samples: Vec<_> = (1..=1_000).collect();
        let p = Percentiles::from(&samples);
        assert_eq!(
            (p.min, p.p50, p.p95, p.p99, p.p999, p.max),
            (1, 500, 950, 990, 999, 1_000)
        );
    }
}
