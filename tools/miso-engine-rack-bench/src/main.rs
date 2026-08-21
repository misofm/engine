//! Frozen Issue-038 real-audio benchmark record emitter.
//!
//! The binary has no command-line control surface. The fixed shell runner supplies only the
//! warmup/round environment, while this program owns fixed 48 kHz, 128-frame production DSP.

use core::fmt::Write as _;
use std::time::Instant;

use miso_engine_builtins::{
    BuiltinChain, BuiltinInputBankV1, BuiltinParameters, ChannelParameters, DualMonoBlock,
};
use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::realtime::{PlanarBufferMut, RenderIo, RenderTime, audit};
use miso_engine_core::{KernelBackendV1, target_capabilities};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_rack::KernelDispatch;
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};
use sha2::{Digest, Sha256};

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
    match std::env::var("MISO_ENGINE_RACK_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the rack benchmark must be launched by its fixed runner"),
    }
}
fn host_backend() -> KernelBackendV1 {
    let backend = KernelDispatch::select(target_capabilities()).backend();
    assert!(
        matches!(
            backend,
            KernelBackendV1::X86Avx2 | KernelBackendV1::X86Avx2Fma
        ),
        "Issue-038 qualification requires x86 AVX2"
    );
    backend
}

fn measure(workload: Workload, backend: KernelBackendV1) -> (Measurement, Shape) {
    let mut runtime = Runtime::prepare(workload, backend);
    let mut durations = Vec::with_capacity(OBSERVATIONS);
    audit::warm_up();
    audit::reset();
    let mut render_errors = 0_u64;
    let mut panic_unwinds = 0_u64;
    for observation in 0..OBSERVATIONS {
        // Frozen asymmetric dual-mono input is filled outside the timer; filter state is never reset.
        runtime.fill_input(observation as u64);
        let started = Instant::now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| runtime.render()));
        let ns = u64::try_from(started.elapsed().as_nanos()).expect("duration fits u64")
            / QUANTUM as u64;
        match result {
            Ok(Ok(())) => {}
            Ok(Err(())) => render_errors += 1,
            Err(_) => panic_unwinds += 1,
        }
        durations.push(ns);
    }
    let measurement = Measurement {
        ns_per_frame: durations,
        output_sha256: runtime.output_sha256(),
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
    fn prepare(workload: Workload, backend: KernelBackendV1) -> Self {
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
    fn render(&mut self) -> Result<(), ()> {
        match self {
            Self::Scalar(value) => value.render(),
            Self::Bank(value) => value.render(),
            Self::Mixed(value) => value.render(),
        }
    }
    fn output_sha256(&self) -> String {
        match self {
            Self::Scalar(value) => value.output_sha256(),
            Self::Bank(value) => value.output_sha256(),
            Self::Mixed(value) => value.output_sha256(),
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
    fn render(&mut self) -> Result<(), ()> {
        for (index, chain) in self.chains.iter_mut().enumerate() {
            chain
                .process_dual_mono(
                    DualMonoBlock::new(&mut self.left[index], &mut self.right[index], 0)
                        .map_err(|_| ())?,
                )
                .map_err(|_| ())?;
        }
        Ok(())
    }
    fn output_sha256(&self) -> String {
        hash_f32(
            self.left
                .iter()
                .flatten()
                .chain(self.right.iter().flatten()),
        )
    }
}

struct BankRuntime {
    bank: BuiltinInputBankV1,
    backend_name: &'static str,
    left: Vec<f32>,
    right: Vec<f32>,
}
impl BankRuntime {
    fn new(backend: KernelBackendV1) -> Self {
        let inputs = (0..8)
            .map(chain_for_track)
            .map(BuiltinChain::into_input_builtins)
            .collect();
        Self {
            bank: BuiltinInputBankV1::new(
                backend,
                miso_engine_effect_contract::BankWidth::Eight,
                inputs,
                &[true; 8],
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
    fn render(&mut self) -> Result<(), ()> {
        self.bank
            .process(&mut self.left, &mut self.right, QUANTUM as u32, 0)
            .map(|_| ())
            .map_err(|_| ())
    }
    fn output_sha256(&self) -> String {
        hash_f32(self.left.iter().chain(self.right.iter()))
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
    fn new(backend: KernelBackendV1) -> Self {
        let mut model =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("canonical session");
        let template = model.tracks[0].clone();
        let route = model.routes[0].clone();
        model.automation.clear();
        model.tracks = (0..12)
            .map(|index| {
                let mut track = template.clone();
                track.id = StableId::parse(&format!("bank{index}")).expect("frozen track id");
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
        let artifact = match GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            plan_id: 38,
            effects: EffectPreparedSession {
                session,
                entries: Vec::new(),
            },
            builtins,
            caps: graph_caps(),
        }) {
            Ok(value) => value,
            Err(_) => panic!("mixed production graph"),
        };
        assert_eq!(
            artifact.prepared_builtin_bank_count(),
            1,
            "host-selected full production builtin bank"
        );
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), source_binding(node)))
            .collect();
        let bound = match artifact.into_bound(GraphRuntimeBindings {
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
    fn fill_input(&mut self, _observation: u64) {}
    fn render(&mut self) -> Result<(), ()> {
        self.plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
                        .map_err(|_| ())?,
                },
                RenderTime { absolute_sample: 0 },
            )
            .map(|_| ())
            .map_err(|_| ())
    }
    fn output_sha256(&self) -> String {
        hash_f32(self.output.iter())
    }
}

struct FrozenGraphSource {
    track: usize,
}
impl GraphRuntimeProcessor for FrozenGraphSource {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        for (frame, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            (*left, *right) = asymmetric_input(self.track, frame, 0);
        }
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
        let track = track_id
            .as_str()
            .strip_prefix("bank")
            .and_then(|value| value.parse().ok())
            .expect("frozen graph input");
        Box::new(FrozenGraphSource { track })
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
fn backend_name(backend: KernelBackendV1) -> &'static str {
    match backend {
        KernelBackendV1::X86Avx2 => "X86Avx2",
        KernelBackendV1::X86Avx2Fma => "X86Avx2Fma",
        _ => panic!("frozen host workload requires x86 AVX2"),
    }
}
fn hash_f32<'a>(values: impl Iterator<Item = &'a f32>) -> String {
    let mut hash = Sha256::new();
    for value in values {
        hash.update(value.to_bits().to_le_bytes());
    }
    hex_digest(hash.finalize())
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
    input_sha256: String,
}
impl Identities {
    fn collect() -> Self {
        Self {
            candidate_commit_sha256: required_sha256("MISO_ENGINE_RACK_BENCH_CANDIDATE_SHA256"),
            binary_sha256: required_sha256("MISO_ENGINE_RACK_BENCH_BINARY_SHA256"),
            fixture_sha256: hex_digest(Sha256::digest(FIXTURE_BYTES)),
            input_sha256: hex_digest(Sha256::digest(
                b"issue038/asymmetric-dual-mono/v1/48000/128/continuous-state",
            )),
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
            ("architecture", "MISO_ENGINE_BENCH_ARCHITECTURE"),
            ("background_load_note", "MISO_ENGINE_BENCH_BACKGROUND_LOAD"),
            ("codegen_units", "MISO_ENGINE_BENCH_CODEGEN_UNITS"),
            ("cpu_model", "MISO_ENGINE_BENCH_CPU_MODEL"),
            ("governor_or_power_mode", "MISO_ENGINE_BENCH_GOVERNOR"),
            ("kernel", "MISO_ENGINE_BENCH_KERNEL"),
            ("llvm_version", "MISO_ENGINE_BENCH_LLVM_VERSION"),
            ("logical_cores", "MISO_ENGINE_BENCH_LOGICAL_CORES"),
            ("lto", "MISO_ENGINE_BENCH_LTO"),
            ("opt_level", "MISO_ENGINE_BENCH_OPT_LEVEL"),
            ("os", "MISO_ENGINE_BENCH_OS"),
            ("physical_cores", "MISO_ENGINE_BENCH_PHYSICAL_CORES"),
            ("profile", "MISO_ENGINE_BENCH_PROFILE"),
            ("rust_version", "MISO_ENGINE_BENCH_RUST_VERSION"),
            ("target_features", "MISO_ENGINE_BENCH_TARGET_FEATURES"),
            ("target_triple", "MISO_ENGINE_BENCH_TARGET"),
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
        identities.input_sha256,
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
