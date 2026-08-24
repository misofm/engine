//! Frozen issue-035 benchmark emitter. The runner is the sole authorized timing entrypoint.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_bench_support::json;
use miso_engine_bench_support::stats;
use miso_engine_graph_compiler::Backend;
use std::time::Instant;

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameters, BuiltinTail, ChannelParameters, DualMonoBlock, Matrix2x2,
    MeterConfig, MeterHandle, MeterSnapshot, MeterTap,
};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, prepare_session_builtins,
};
use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_core::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime,
    audit::{self},
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
const INPUT_MANIFEST_SHA256: &str =
    "bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff";
const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

const WORKLOADS: [Workload; 5] = [
    Workload::FullChainFilters,
    Workload::IdentityChain,
    Workload::MatrixRamp,
    Workload::MeterSuccessFull,
    Workload::Prepare256Tracks,
];

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

struct BenchmarkIdentities {
    candidate_commit: String,
    binary_sha256: String,
}

impl BenchmarkIdentities {
    fn from_environment() -> Self {
        Self {
            candidate_commit: required_identity("MISO_ENGINE_BENCH_CANDIDATE_COMMIT", 40),
            binary_sha256: required_identity("MISO_ENGINE_BENCH_BINARY_SHA256", 64),
        }
    }

    #[cfg(test)]
    fn synthetic() -> Self {
        Self {
            candidate_commit: "a".repeat(40),
            binary_sha256: "b".repeat(64),
        }
    }
}

struct InputFixture {
    id: &'static str,
    bytes: &'static [u8],
}

impl InputFixture {
    fn field(&self, name: &str) -> &str {
        let prefix = format!("{name} = ");
        let text = std::str::from_utf8(self.bytes).expect("checked benchmark TOML is UTF-8");
        let mut matches = text.lines().filter_map(|line| line.strip_prefix(&prefix));
        let value = matches
            .next()
            .unwrap_or_else(|| panic!("checked benchmark TOML is missing {name}"));
        assert!(matches.next().is_none(), "duplicate benchmark field {name}");
        value
    }

    fn text(&self, name: &str) -> &str {
        self.field(name)
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .unwrap_or_else(|| panic!("benchmark field {name} is not a quoted string"))
    }

    fn u32(&self, name: &str) -> u32 {
        self.field(name)
            .parse()
            .unwrap_or_else(|_| panic!("benchmark field {name} is not u32"))
    }

    fn usize(&self, name: &str) -> usize {
        self.field(name)
            .parse()
            .unwrap_or_else(|_| panic!("benchmark field {name} is not usize"))
    }

    fn f32(&self, name: &str) -> f32 {
        let value: f32 = self
            .field(name)
            .parse()
            .unwrap_or_else(|_| panic!("benchmark field {name} is not f32"));
        assert!(value.is_finite(), "benchmark field {name} is not finite");
        value
    }

    fn boolean(&self, name: &str) -> bool {
        match self.field(name) {
            "true" => true,
            "false" => false,
            _ => panic!("benchmark field {name} is not bool"),
        }
    }

    fn validate_common(&self, workload: Workload, rate_hz: u32) {
        assert_eq!(sha256(self.bytes), manifest_input_sha256(self.id));
        assert_eq!(self.u32("fixture_schema"), 1);
        assert_eq!(self.u32("issue"), ISSUE);
        assert_eq!(self.text("workload_kind"), workload.kind());
        assert_eq!(
            self.text("workload_id"),
            format!("issue035.{}.{}hz.q128", workload.kind(), rate_hz)
        );
        assert_eq!(self.u32("sample_rate_hz"), rate_hz);
        assert_eq!(self.usize("quantum_frames"), QUANTUM);
    }

    fn pcm(&self) -> FixturePcm {
        let (path, bytes): (&str, &[u8]) = match self.text("input_pcm_path") {
            "pcm/filters-asymmetric.f32le" => (
                "pcm/filters-asymmetric.f32le",
                include_bytes!("../../../fixtures/builtins/v1/pcm/filters-asymmetric.f32le"),
            ),
            "pcm/identity-signed-zero.f32le" => (
                "pcm/identity-signed-zero.f32le",
                include_bytes!("../../../fixtures/builtins/v1/pcm/identity-signed-zero.f32le"),
            ),
            "pcm/matrix-ramp-128.f32le" => (
                "pcm/matrix-ramp-128.f32le",
                include_bytes!("../../../fixtures/builtins/v1/pcm/matrix-ramp-128.f32le"),
            ),
            "pcm/graph-taps.f32le" => (
                "pcm/graph-taps.f32le",
                include_bytes!("../../../fixtures/builtins/v1/pcm/graph-taps.f32le"),
            ),
            path => panic!("unsupported checked benchmark PCM path {path}"),
        };
        assert_eq!(self.text("input_pcm_path"), path);
        assert_eq!(sha256(bytes), self.text("input_pcm_sha256"));
        FixturePcm::from_planar_f32le(bytes)
    }
}

#[derive(Clone)]
struct FixturePcm {
    left: Box<[f32]>,
    right: Box<[f32]>,
}

impl FixturePcm {
    fn from_planar_f32le(bytes: &[u8]) -> Self {
        assert!(bytes.len().is_multiple_of(8), "stereo planar f32le PCM");
        let words: Vec<_> = bytes
            .chunks_exact(4)
            .map(|word| f32::from_bits(u32::from_le_bytes(word.try_into().expect("f32 word"))))
            .collect();
        let frames = words.len() / 2;
        assert!(frames > 0, "nonempty benchmark PCM");
        Self {
            left: words[..frames].into(),
            right: words[frames..].into(),
        }
    }

    fn fill(&self, left: &mut [f32], right: &mut [f32], first_sample: u64) {
        assert_eq!(left.len(), right.len(), "dual-mono benchmark block");
        let frames = self.left.len();
        let offset = usize::try_from(first_sample % frames as u64).expect("PCM offset");
        for (index, (left_out, right_out)) in left.iter_mut().zip(right).enumerate() {
            let source = (offset + index) % frames;
            *left_out = self.left[source];
            *right_out = self.right[source];
        }
    }
}

fn input_fixture(workload: Workload, rate_hz: u32) -> InputFixture {
    match (workload, rate_hz) {
        (Workload::FullChainFilters, 48_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml"
            ),
        },
        (Workload::FullChainFilters, 96_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/full_chain_filters-96000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/full_chain_filters-96000.toml"
            ),
        },
        (Workload::IdentityChain, 48_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/identity_chain-48000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/identity_chain-48000.toml"
            ),
        },
        (Workload::IdentityChain, 96_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/identity_chain-96000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/identity_chain-96000.toml"
            ),
        },
        (Workload::MatrixRamp, 48_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/matrix_ramp-48000.toml",
            bytes: include_bytes!("../../../fixtures/builtins/v1/benchmark/matrix_ramp-48000.toml"),
        },
        (Workload::MatrixRamp, 96_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/matrix_ramp-96000.toml",
            bytes: include_bytes!("../../../fixtures/builtins/v1/benchmark/matrix_ramp-96000.toml"),
        },
        (Workload::MeterSuccessFull, 48_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/meter_success_full-48000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/meter_success_full-48000.toml"
            ),
        },
        (Workload::MeterSuccessFull, 96_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/meter_success_full-96000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/meter_success_full-96000.toml"
            ),
        },
        (Workload::Prepare256Tracks, 48_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/prepare_256_tracks-48000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/prepare_256_tracks-48000.toml"
            ),
        },
        (Workload::Prepare256Tracks, 96_000) => InputFixture {
            id: "fixtures/builtins/v1/benchmark/prepare_256_tracks-96000.toml",
            bytes: include_bytes!(
                "../../../fixtures/builtins/v1/benchmark/prepare_256_tracks-96000.toml"
            ),
        },
        _ => panic!("unsupported frozen benchmark input"),
    }
}

fn render_parameters_from_fixture(
    fixture: &InputFixture,
    workload: Workload,
) -> (BuiltinParameters, Option<[Matrix2x2; 2]>) {
    match workload {
        Workload::FullChainFilters | Workload::IdentityChain => (
            BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: fixture.f32("left_hpf_hz"),
                    lpf_hz: fixture.f32("left_lpf_hz"),
                    trim_db: fixture.f32("left_trim_db"),
                    fader_db: fixture.f32("left_fader_db"),
                    ..ChannelParameters::default()
                },
                right: ChannelParameters {
                    hpf_hz: fixture.f32("right_hpf_hz"),
                    lpf_hz: fixture.f32("right_lpf_hz"),
                    trim_db: fixture.f32("right_trim_db"),
                    fader_db: fixture.f32("right_fader_db"),
                    ..ChannelParameters::default()
                },
                matrix: Matrix2x2 {
                    ll: fixture.f32("matrix_ll"),
                    lr: fixture.f32("matrix_lr"),
                    rl: fixture.f32("matrix_rl"),
                    rr: fixture.f32("matrix_rr"),
                },
                smoothing_samples: 0,
            },
            None,
        ),
        Workload::MatrixRamp => (
            BuiltinParameters {
                matrix: Matrix2x2 {
                    ll: fixture.f32("initial_matrix_ll"),
                    lr: fixture.f32("initial_matrix_lr"),
                    rl: fixture.f32("initial_matrix_rl"),
                    rr: fixture.f32("initial_matrix_rr"),
                },
                smoothing_samples: fixture.u32("smoothing_updates"),
                ..BuiltinParameters::default()
            },
            Some([
                Matrix2x2 {
                    ll: fixture.f32("even_target_ll"),
                    lr: fixture.f32("even_target_lr"),
                    rl: fixture.f32("even_target_rl"),
                    rr: fixture.f32("even_target_rr"),
                },
                Matrix2x2 {
                    ll: fixture.f32("odd_target_ll"),
                    lr: fixture.f32("odd_target_lr"),
                    rl: fixture.f32("odd_target_rl"),
                    rr: fixture.f32("odd_target_rr"),
                },
            ]),
        ),
        Workload::MeterSuccessFull | Workload::Prepare256Tracks => {
            (BuiltinParameters::default(), None)
        }
    }
}

fn meter_config_from_fixture(fixture: &InputFixture) -> MeterConfig {
    MeterConfig {
        period_frames: NonZeroU32::new(fixture.u32("meter_period_frames"))
            .expect("checked nonzero meter period"),
        peak_hold_frames: fixture.u32("meter_peak_hold_frames"),
        peak_decay_db_per_second: fixture.f32("meter_peak_decay_db_per_second"),
        queue_capacity: NonZeroUsize::new(fixture.usize("meter_queue_capacity"))
            .expect("checked nonzero meter capacity"),
        reset_generation: fixture
            .field("meter_reset_generation")
            .parse()
            .expect("checked meter reset generation"),
    }
}

struct RenderRoundStates {
    workload: Workload,
    rate_hz: u32,
    rounds: [RenderRuntime; 2],
}

#[derive(Clone, Copy)]
struct RecordPlan {
    workload: Workload,
    rate_hz: u32,
    round: u32,
    round_index: usize,
}

fn measured_record_plans() -> Vec<RecordPlan> {
    let mut plans = Vec::with_capacity(WORKLOADS.len() * RATES.len() * ROUNDS.len());
    for (round_index, round) in ROUNDS.into_iter().enumerate() {
        for rate_hz in RATES {
            for workload in WORKLOADS {
                plans.push(RecordPlan {
                    workload,
                    rate_hz,
                    round,
                    round_index,
                });
            }
        }
    }
    plans
}

/// Two independently prepared graph artifacts for the frozen meter success/full workload.
///
/// The bound runtime owns the separately prepared plans, pre-fills only the capacity-one plan
/// off timing, and drains the success plan through compiler-owned consumers.
struct RealMeterTapArtifactPair {
    success: PreparedGraphBuiltinsArtifact,
    full: PreparedGraphBuiltinsArtifact,
}

fn prepare_real_meter_tap_artifacts(rate_hz: u32, config: MeterConfig) -> RealMeterTapArtifactPair {
    RealMeterTapArtifactPair {
        success: prepare_real_meter_tap_artifact(rate_hz, config),
        full: prepare_real_meter_tap_artifact(rate_hz, config),
    }
}

fn prepare_real_meter_tap_artifact(
    rate_hz: u32,
    config: MeterConfig,
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
        &real_meter_tap_requests(&session, config),
        unbounded_builtin_caps(),
    )
    .expect("frozen builtin tap requests");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: ISSUE.into(),
        effects,
        builtins,
        caps: unbounded_graph_caps(),
    })
    .unwrap_or_else(|_| panic!("frozen real-tap graph"))
}

struct BenchmarkGraphSource {
    pcm: FixturePcm,
}

impl GraphRuntimeProcessor for BenchmarkGraphSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.pcm.fill(block.left, block.right, block.first_sample);
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
    fn bind(artifact: PreparedGraphBuiltinsArtifact, pcm: FixturePcm) -> Self {
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = match node {
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    } => Box::new(BenchmarkGraphSource { pcm: pcm.clone() }),
                    _ => Box::new(BenchmarkGraphIdentity),
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

    fn drain_all_direct(&mut self, mut consume: impl FnMut(MeterConsumerSnapshot)) -> usize {
        assert!(
            !audit::is_render_scope_active(),
            "meter evidence collection must be outside render"
        );
        let mut count = 0;
        for consumer in &mut self.consumers {
            consume(MeterConsumerSnapshot {
                handle: consumer.handle,
                tap: consumer.tap,
                snapshot: consumer
                    .consumer
                    .try_pop()
                    .expect("one completed window per real tap"),
            });
            count += 1;
        }
        count
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
    full_drop_attempts: u64,
    measurement_start_full_drop_attempts: u64,
}

impl RealMeterTapRuntime {
    fn new(rate_hz: u32, fixture: &InputFixture, pcm: &FixturePcm) -> Self {
        let config = meter_config_from_fixture(fixture);
        assert_eq!(config.period_frames.get(), QUANTUM as u32);
        assert_eq!(config.queue_capacity.get(), 1);
        assert_eq!(config.reset_generation, 7);
        assert!(fixture.boolean("success_drain_per_operation"));
        assert!(fixture.boolean("full_prefill"));
        let expected_taps = "input,post_input_builtins,post_simd1,post_dynamic,post_simd2_pre_fader,post_fader,post_matrix";
        assert_eq!(fixture.text("success_taps"), expected_taps);
        assert_eq!(fixture.text("full_taps"), expected_taps);
        let pair = prepare_real_meter_tap_artifacts(rate_hz, config);
        let mut success = RealMeterTapPlan::bind(pair.success, pcm.clone());
        let mut full = RealMeterTapPlan::bind(pair.full, pcm.clone());
        success.render(0);
        let prefill_success = success.drain_all_direct(|_| {});
        assert_eq!(prefill_success, OBSERVERS, "frozen success prefill");
        full.render(0);
        Self {
            success,
            full,
            output: Sha256::new(),
            full_drop_attempts: 0,
            measurement_start_full_drop_attempts: 0,
        }
    }

    fn begin_measurement(&mut self) {
        self.output = Sha256::new();
        self.measurement_start_full_drop_attempts = self.full_drop_attempts;
    }

    fn render_one(&mut self, first_sample: u64) {
        self.success.render(first_sample);
        self.full.render(first_sample);
    }

    fn collect_operation_evidence(&mut self, retain: bool) -> usize {
        assert!(
            !audit::is_render_scope_active(),
            "meter drain and hashing must be outside render"
        );
        let output = &mut self.output;
        let success_taps = self.success.drain_all_direct(|record| {
            if retain {
                output.update(b"success");
                hash_meter_snapshot(output, record);
            }
        });
        self.full_drop_attempts = self
            .full_drop_attempts
            .checked_add(1)
            .expect("bounded full/drop operation count");
        if retain {
            for value in self.success.pcm.iter().chain(&self.full.pcm) {
                self.output.update(value.to_bits().to_le_bytes());
            }
        }
        success_taps
    }

    fn output_sha256(&mut self) -> String {
        assert!(
            !audit::is_render_scope_active(),
            "final meter evidence must be outside render"
        );
        let output = &mut self.output;
        let full_taps = self.full.drain_all_direct(|record| {
            output.update(b"full");
            hash_meter_snapshot(output, record);
        });
        assert_eq!(full_taps, OBSERVERS, "frozen full tap count");
        self.output.update(b"full_drop_attempts_before");
        self.output
            .update(self.measurement_start_full_drop_attempts.to_le_bytes());
        self.output.update(b"full_drop_attempts_after");
        self.output.update(self.full_drop_attempts.to_le_bytes());
        self.output.update(b"full_drop_attempts_measured");
        self.output.update(
            self.full_drop_attempts
                .checked_sub(self.measurement_start_full_drop_attempts)
                .expect("monotonic full/drop count")
                .to_le_bytes(),
        );
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
    config: MeterConfig,
) -> Vec<MeterRequest> {
    assert_eq!(config.period_frames.get(), session.quantum().0);
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
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(
        std::env::args().count(),
        1,
        "benchmark accepts no arguments"
    );
    let identities = BenchmarkIdentities::from_environment();
    let metadata = Metadata::collect();
    let fixture_sha256 = sha256(INPUT_MANIFEST);
    assert_eq!(
        fixture_sha256, INPUT_MANIFEST_SHA256,
        "frozen fixture manifest"
    );
    eprintln!("MISO_ENGINE_BENCH_PHASE workload_started");
    let mut render_states = prepare_render_round_states();
    for rate_hz in RATES {
        warmup_prepare(rate_hz);
    }
    eprintln!("MISO_ENGINE_BENCH_PHASE warmup_complete");
    eprintln!("MISO_ENGINE_BENCH_PHASE timed_started");
    for (index, plan) in measured_record_plans().into_iter().enumerate() {
        let measurement = if plan.workload.is_prepare() {
            measure_prepare(plan.rate_hz)
        } else {
            let state = render_states
                .iter_mut()
                .find(|state| state.workload == plan.workload && state.rate_hz == plan.rate_hz)
                .expect("global render warmup state");
            measure_render(&mut state.rounds[plan.round_index])
        };
        println!(
            "{}",
            record_json(
                plan.workload,
                plan.rate_hz,
                plan.round,
                &fixture_sha256,
                &identities,
                &metadata,
                &measurement
            )
        );
        if (index + 1) % (WORKLOADS.len() * RATES.len()) == 0 {
            eprintln!("MISO_ENGINE_BENCH_PHASE round_{}_complete", plan.round);
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
    runtime.begin_measurement();
    audit::warm_up();
    audit::reset();
    for batch in 0..RENDER_MEASURED_BATCHES {
        let mut batch_ns = 0_u64;
        for operation in 0..OPERATIONS_PER_RENDER_BATCH {
            let logical_batch = (RENDER_WARMUP_BATCHES + batch) as u64;
            runtime.prepare_operation_input(logical_batch, operation as u64);
            let started = Instant::now();
            runtime.run_render_operation(logical_batch, operation as u64);
            let elapsed = started.elapsed();
            runtime.collect_operation_evidence(true);
            batch_ns = batch_ns.saturating_add(
                u64::try_from(elapsed.as_nanos()).expect("benchmark duration fits u64"),
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
    let mut output = Sha256::new();
    for operation in 0..PREPARE_MEASURED_BATCHES {
        let started = Instant::now();
        let prepared = prepare_256_tracks(rate_hz);
        let elapsed = started.elapsed();
        retained_payload_bytes = prepared
            .resource_report()
            .engine_owned_retained_payload_bytes;
        output.update((operation as u64).to_le_bytes());
        hash_preparation_projection(&mut output, &prepared);
        std::hint::black_box(prepared); // destruction is deliberately outside the elapsed interval.
        samples_ns.push(u64::try_from(elapsed.as_nanos()).expect("benchmark duration fits u64"));
    }
    Measurement {
        samples_ns,
        output_sha256: hex_digest(output.finalize()),
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
    output: Sha256,
    input: FixturePcm,
    matrix_targets: Option<[Matrix2x2; 2]>,
}
impl RenderRuntime {
    fn new(workload: Workload, rate_hz: u32) -> Self {
        let fixture = input_fixture(workload, rate_hz);
        fixture.validate_common(workload, rate_hz);
        assert_eq!(fixture.usize("tracks"), 1);
        assert_eq!(fixture.text("state_mode"), "continuous");
        let input = fixture.pcm();
        let (parameters, matrix_targets) = render_parameters_from_fixture(&fixture, workload);
        let meter_runtime = if matches!(workload, Workload::MeterSuccessFull) {
            assert_eq!(fixture.usize("meter_observers"), OBSERVERS * 2);
            Some(RealMeterTapRuntime::new(rate_hz, &fixture, &input))
        } else {
            assert_eq!(fixture.usize("meter_observers"), 0);
            assert_eq!(fixture.usize("meter_queue_capacity"), 0);
            None
        };
        Self {
            workload,
            chain: BuiltinChain::new(rate_hz, parameters).expect("frozen parameters"),
            left: [0.0; QUANTUM],
            right: [0.0; QUANTUM],
            meter_runtime,
            output: Sha256::new(),
            input,
            matrix_targets,
        }
    }

    fn begin_measurement(&mut self) {
        self.output = Sha256::new();
        if let Some(meter_runtime) = &mut self.meter_runtime {
            meter_runtime.begin_measurement();
        }
    }
    fn run_batch(&mut self, batch: u64) {
        for operation in 0..OPERATIONS_PER_RENDER_BATCH {
            let operation = operation as u64;
            self.prepare_operation_input(batch, operation);
            self.run_render_operation(batch, operation);
            self.collect_operation_evidence(false);
        }
    }
    fn prepare_operation_input(&mut self, batch: u64, operation: u64) {
        if self.meter_runtime.is_none() {
            let first_sample = operation_first_sample(batch, operation);
            self.input
                .fill(&mut self.left, &mut self.right, first_sample);
        }
        if matches!(self.workload, Workload::MatrixRamp) {
            let target = matrix_target_for_operation(
                batch,
                operation,
                self.matrix_targets.expect("checked matrix targets"),
            );
            self.chain
                .set_matrix_target(target)
                .expect("frozen matrix target");
        }
    }
    fn run_render_operation(&mut self, batch: u64, operation: u64) {
        audit::in_render_scope(|| self.render_product(batch, operation));
    }
    fn render_product(&mut self, batch: u64, operation: u64) {
        let first_sample = operation_first_sample(batch, operation);
        if let Some(meter_runtime) = &mut self.meter_runtime {
            meter_runtime.render_one(first_sample);
            return;
        }
        self.chain.process_dual_mono(
            DualMonoBlock::new(&mut self.left, &mut self.right, first_sample).expect("fixed block"),
        );
    }
    fn collect_operation_evidence(&mut self, retain: bool) -> usize {
        assert!(
            !audit::is_render_scope_active(),
            "PCM and meter evidence must be outside render"
        );
        if let Some(meter_runtime) = &mut self.meter_runtime {
            return meter_runtime.collect_operation_evidence(retain);
        }
        if retain {
            for value in self.left.iter().chain(&self.right) {
                self.output.update(value.to_bits().to_le_bytes());
            }
        }
        0
    }
    fn output_sha256(&mut self) -> String {
        if let Some(meter_runtime) = &mut self.meter_runtime {
            return meter_runtime.output_sha256();
        }
        hex_digest(self.output.clone().finalize())
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

fn operation_first_sample(batch: u64, operation: u64) -> u64 {
    batch
        .checked_mul(OPERATIONS_PER_RENDER_BATCH as u64)
        .and_then(|value| value.checked_add(operation))
        .and_then(|value| value.checked_mul(QUANTUM as u64))
        .expect("bounded benchmark sample position")
}

fn matrix_target_for_operation(batch: u64, operation: u64, targets: [Matrix2x2; 2]) -> Matrix2x2 {
    let global_operation = batch
        .checked_mul(OPERATIONS_PER_RENDER_BATCH as u64)
        .and_then(|value| value.checked_add(operation))
        .expect("bounded benchmark operation index");
    if global_operation.is_multiple_of(2) {
        targets[0]
    } else {
        targets[1]
    }
}

fn prepare_256_tracks(rate_hz: u32) -> miso_engine_builtins_compiler::PreparedBuiltinsSession {
    let fixture = input_fixture(Workload::Prepare256Tracks, rate_hz);
    fixture.validate_common(Workload::Prepare256Tracks, rate_hz);
    assert_eq!(fixture.text("state_mode"), "new_per_prepare");
    assert_eq!(
        fixture.text("session_template_path"),
        "fixtures/session/v1/canonical.toml"
    );
    assert_eq!(
        sha256(SESSION.as_bytes()),
        fixture.text("session_template_sha256")
    );
    assert!(fixture.boolean("empty_effect_racks"));
    let track_count = fixture.usize("track_id_count");
    assert_eq!(fixture.usize("tracks"), track_count);
    assert_eq!(track_count, PREPARE_TRACKS);
    let track_prefix = fixture.text("track_id_prefix");
    let mut model = parse_session_toml(SESSION).expect("frozen session");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.limits.memory_bytes = i64::MAX as u64;
    model.tracks.clear();
    model.tracks.reserve(track_count);
    for index in 0..track_count {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("{track_prefix}{index}")).expect("stable ID");
        model.tracks.push(track);
    }
    model.sample_rate_hz = rate_hz;
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse(fixture.text("route_source_track_id")).expect("route track"),
        tap: match fixture.text("route_source_tap") {
            "post_matrix" => SendTap::PostMatrix,
            value => panic!("unsupported checked route source tap {value}"),
        },
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
    let config = meter_config_from_fixture(&fixture);
    assert_eq!(config.queue_capacity.get(), 4);
    let meter_track_ids: Vec<_> = fixture.text("meter_track_ids").split(',').collect();
    let taps: Vec<_> = fixture
        .text("meter_taps")
        .split(',')
        .map(meter_tap_from_name)
        .collect();
    assert_eq!(
        meter_track_ids.len() * taps.len(),
        fixture.usize("meter_observers")
    );
    let requests: Vec<_> = meter_track_ids
        .into_iter()
        .enumerate()
        .flat_map(|(track, track_id)| {
            let taps = taps.clone();
            let tap_count = taps.len();
            taps.into_iter()
                .enumerate()
                .map(move |(tap_index, tap)| MeterRequest {
                    handle: MeterHandle(
                        core::num::NonZeroU64::new(
                            u64::try_from(track * tap_count + tap_index).expect("bounded") + 1,
                        )
                        .expect("nonzero"),
                    ),
                    track_id: track_id.to_owned(),
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

fn meter_tap_from_name(name: &str) -> MeterTap {
    match name {
        "input" => MeterTap::Input,
        "post_input_builtins" => MeterTap::PostInputBuiltins,
        "post_simd1" => MeterTap::PostSimd1,
        "post_dynamic" => MeterTap::PostDynamic,
        "post_simd2_pre_fader" => MeterTap::PostSimd2PreFader,
        "post_fader" => MeterTap::PostFader,
        "post_matrix" => MeterTap::PostMatrix,
        value => panic!("unsupported checked meter tap {value}"),
    }
}

fn hash_preparation_projection(
    hash: &mut Sha256,
    prepared: &miso_engine_builtins_compiler::PreparedBuiltinsSession,
) {
    hash.update(b"issue035.prepare_256_tracks.address_free.v1");
    for count in [
        prepared.processor_count(),
        prepared.tail_count(),
        prepared.observer_count(),
        prepared.meter_consumer_count(),
    ] {
        hash.update(
            u64::try_from(count)
                .expect("preparation count fits u64")
                .to_le_bytes(),
        );
    }
    for (track_id, tail) in prepared.tails() {
        hash.update(
            u64::try_from(track_id.len())
                .expect("track ID length fits u64")
                .to_le_bytes(),
        );
        hash.update(track_id.as_bytes());
        hash.update([match tail {
            BuiltinTail::FiniteZero => 0,
            BuiltinTail::Infinite => 1,
        }]);
    }
    let report = prepared.resource_report();
    for value in [
        report.engine_owned_processor_payload_bytes,
        report.engine_owned_meter_payload_bytes,
        report.engine_owned_retained_payload_bytes,
        report.meter_items,
        report.maximum_single_allocation_bytes,
        report.retained_allocation_count,
        u64::from(report.retained_layout_class_count),
    ] {
        hash.update(value.to_le_bytes());
    }
    for layout in report.retained_layouts() {
        hash.update(layout.size_bytes.to_le_bytes());
        hash.update(layout.align_bytes.to_le_bytes());
        hash.update(layout.allocation_count.to_le_bytes());
    }
}

struct Metadata {
    cpu_model: Option<String>,
    cpu_architecture: Option<String>,
    logical_core_count: Option<u32>,
    physical_core_count: Option<u32>,
    os: Option<String>,
    kernel: Option<String>,
    governor_or_power_mode: Option<String>,
    rust_version: Option<String>,
    llvm_version: Option<String>,
    target_triple: Option<String>,
    target_features: Option<String>,
    profile: Option<String>,
    opt_level: Option<String>,
    lto: Option<String>,
    codegen_units: Option<String>,
    background_load_note: Option<String>,
    missing: Vec<&'static str>,
}

impl Metadata {
    fn collect() -> Self {
        let mut missing = Vec::new();
        let cpu_model = text_metadata("MISO_ENGINE_BENCH_CPU_MODEL", "cpu_model", &mut missing);
        let cpu_architecture = text_metadata(
            "MISO_ENGINE_BENCH_CPU_ARCHITECTURE",
            "cpu_architecture",
            &mut missing,
        );
        let logical_core_count = number_metadata(
            "MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT",
            "logical_core_count",
            &mut missing,
        );
        let physical_core_count = number_metadata(
            "MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT",
            "physical_core_count",
            &mut missing,
        );
        let os = text_metadata("MISO_ENGINE_BENCH_OS", "os", &mut missing);
        let kernel = text_metadata("MISO_ENGINE_BENCH_KERNEL", "kernel", &mut missing);
        let governor_or_power_mode = text_metadata(
            "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
            "governor_or_power_mode",
            &mut missing,
        );
        let rust_version = text_metadata(
            "MISO_ENGINE_BENCH_RUST_VERSION",
            "rust_version",
            &mut missing,
        );
        let llvm_version = text_metadata(
            "MISO_ENGINE_BENCH_LLVM_VERSION",
            "llvm_version",
            &mut missing,
        );
        let target_triple = text_metadata(
            "MISO_ENGINE_BENCH_TARGET_TRIPLE",
            "target_triple",
            &mut missing,
        );
        let target_features = text_metadata(
            "MISO_ENGINE_BENCH_TARGET_FEATURES",
            "target_features",
            &mut missing,
        );
        let profile = text_metadata("MISO_ENGINE_BENCH_PROFILE", "profile", &mut missing);
        let opt_level = text_metadata("MISO_ENGINE_BENCH_OPT_LEVEL", "opt_level", &mut missing);
        let lto = text_metadata("MISO_ENGINE_BENCH_LTO", "lto", &mut missing);
        let codegen_units = text_metadata(
            "MISO_ENGINE_BENCH_CODEGEN_UNITS",
            "codegen_units",
            &mut missing,
        );
        let background_load_note = text_metadata(
            "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
            "background_load_note",
            &mut missing,
        );
        missing.sort_unstable();
        missing.dedup();
        Self {
            cpu_model,
            cpu_architecture,
            logical_core_count,
            physical_core_count,
            os,
            kernel,
            governor_or_power_mode,
            rust_version,
            llvm_version,
            target_triple,
            target_features,
            profile,
            opt_level,
            lto,
            codegen_units,
            background_load_note,
            missing,
        }
    }

    #[cfg(test)]
    fn all_missing() -> Self {
        Self {
            cpu_model: None,
            cpu_architecture: None,
            logical_core_count: None,
            physical_core_count: None,
            os: None,
            kernel: None,
            governor_or_power_mode: None,
            rust_version: None,
            llvm_version: None,
            target_triple: None,
            target_features: None,
            profile: None,
            opt_level: None,
            lto: None,
            codegen_units: None,
            background_load_note: None,
            missing: METADATA_FIELDS.to_vec(),
        }
    }
}

#[cfg(test)]
const METADATA_FIELDS: [&str; 16] = [
    "background_load_note",
    "codegen_units",
    "cpu_architecture",
    "cpu_model",
    "governor_or_power_mode",
    "kernel",
    "llvm_version",
    "logical_core_count",
    "lto",
    "opt_level",
    "os",
    "physical_core_count",
    "profile",
    "rust_version",
    "target_features",
    "target_triple",
];

fn text_metadata(
    environment: &'static str,
    field: &'static str,
    missing: &mut Vec<&'static str>,
) -> Option<String> {
    match std::env::var(environment) {
        Ok(value) if usable_metadata(&value) => Some(value),
        _ => {
            missing.push(field);
            None
        }
    }
}

fn number_metadata(
    environment: &'static str,
    field: &'static str,
    missing: &mut Vec<&'static str>,
) -> Option<u32> {
    match std::env::var(environment)
        .ok()
        .and_then(|value| value.parse().ok())
    {
        Some(value) if value > 0 => Some(value),
        _ => {
            missing.push(field);
            None
        }
    }
}

fn usable_metadata(value: &str) -> bool {
    !value.is_empty()
        && !matches!(value, "unknown" | "default")
        && value.chars().all(|character| !character.is_control())
}

fn required_identity(environment: &'static str, length: usize) -> String {
    let value = std::env::var(environment).unwrap_or_else(|_| panic!("missing {environment}"));
    assert!(
        value.len() == length && is_lower_hex(&value),
        "invalid {environment}"
    );
    value
}

fn is_lower_hex(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn record_json(
    workload: Workload,
    rate_hz: u32,
    round: u32,
    fixture_sha256: &str,
    identities: &BenchmarkIdentities,
    metadata: &Metadata,
    measurement: &Measurement,
) -> String {
    let shape = measurement.shape;
    let expected_shape = workload_shape(workload);
    assert_eq!(
        (shape.tracks, shape.meters, shape.meter_capacity),
        (
            expected_shape.tracks,
            expected_shape.meters,
            expected_shape.meter_capacity,
        ),
        "frozen workload shape"
    );
    assert_eq!(
        fixture_sha256, INPUT_MANIFEST_SHA256,
        "frozen manifest identity"
    );
    let input = input_fixture(workload, rate_hz);
    let input_sha256 = manifest_input_sha256(input.id);
    assert_eq!(sha256(input.bytes), input_sha256, "frozen input identity");
    assert!(is_lower_hex(&measurement.output_sha256), "output hash");
    let p = Percentiles::from_samples(&measurement.samples_ns);
    let (warmup_batches, measured_batches, operations_per_batch, frames_per_operation) =
        if workload.is_prepare() {
            (
                PREPARE_WARMUP_BATCHES,
                PREPARE_MEASURED_BATCHES,
                1,
                "null".to_owned(),
            )
        } else {
            (
                RENDER_WARMUP_BATCHES,
                RENDER_MEASURED_BATCHES,
                OPERATIONS_PER_RENDER_BATCH,
                QUANTUM.to_string(),
            )
        };
    let mut fields = vec![
        json_number("schema_version", 2),
        json_number("issue", ISSUE),
        json_string_field("workload_kind", workload.kind()),
        json_string_field(
            "workload_id",
            &format!("issue035.{}.{}hz.q128", workload.kind(), rate_hz),
        ),
        json_number("sample_rate_hz", rate_hz),
        json_number("quantum_frames", QUANTUM),
        json_number("round", round),
        json_string_field(
            "render_scope",
            if workload.is_prepare() {
                "not_applicable_preparation"
            } else {
                "render"
            },
        ),
        json_number("warmup_batches", warmup_batches),
        json_number("measured_batches", measured_batches),
        json_number("operations_per_batch", operations_per_batch),
        json_number("total_operations", measured_batches * operations_per_batch),
        json_raw("frames_per_operation", frames_per_operation),
        json_number("tracks", shape.tracks),
        json_number("meter_observers", shape.meters),
        json_optional_number("meter_queue_capacity", workload_meter_capacity(workload)),
        json_number("retained_payload_bytes", shape.retained_payload_bytes),
        json_string_field("percentile_method", "nearest_rank"),
        json_string_field("units", "ns_per_operation"),
        json_number("min_ns", p.min),
        json_number("p50_ns", p.p50),
        json_number("p95_ns", p.p95),
        json_number("p99_ns", p.p99),
        json_number("p99_9_ns", p.p999),
        json_number("max_ns", p.max),
        json_raw("descriptive_only", "true".to_owned()),
        json_string_field("candidate_commit", &identities.candidate_commit),
        json_string_field("binary_sha256", &identities.binary_sha256),
        json_string_field("fixture_manifest_id", "fixtures/builtins/v1/MANIFEST.tsv"),
        json_string_field("fixture_manifest_sha256", fixture_sha256),
        json_string_field("input_fixture_id", input.id),
        json_string_field("input_fixture_sha256", input_sha256),
        json_string_field("output_sha256", &measurement.output_sha256),
    ];
    append_audit_fields(&mut fields, measurement.audit);
    append_metadata_fields(&mut fields, metadata);
    assert_record_keys(&fields);
    format!("{{{}}}", fields.join(","))
}

fn workload_shape(workload: Workload) -> WorkloadShape {
    match workload {
        Workload::Prepare256Tracks => WorkloadShape {
            tracks: PREPARE_TRACKS,
            meters: OBSERVERS * 8,
            meter_capacity: 4,
            retained_payload_bytes: 0,
        },
        Workload::MeterSuccessFull => WorkloadShape {
            tracks: 1,
            meters: OBSERVERS * 2,
            meter_capacity: 1,
            retained_payload_bytes: 0,
        },
        _ => WorkloadShape {
            tracks: 1,
            meters: 0,
            meter_capacity: 0,
            retained_payload_bytes: 0,
        },
    }
}

fn manifest_input_sha256(input_id: &str) -> &'static str {
    let manifest_path = input_id
        .strip_prefix("fixtures/builtins/v1/")
        .expect("frozen benchmark input path");
    std::str::from_utf8(INPUT_MANIFEST)
        .expect("UTF-8 fixture manifest")
        .lines()
        .skip(1)
        .find_map(|line| {
            let mut fields = line.split('\t');
            let path = fields.next()?;
            let _length = fields.next()?;
            let sha256 = fields.next()?;
            (path == manifest_path).then_some(sha256)
        })
        .unwrap_or_else(|| panic!("missing frozen manifest row for {input_id}"))
}

fn workload_meter_capacity(workload: Workload) -> Option<usize> {
    match workload {
        Workload::MeterSuccessFull => Some(1),
        Workload::Prepare256Tracks => Some(4),
        _ => None,
    }
}

fn append_audit_fields(fields: &mut Vec<String>, audit: Option<audit::AuditSnapshot>) {
    let values = match audit {
        Some(snapshot) => [
            snapshot.allocations,
            snapshot.deallocations,
            snapshot.locks,
            snapshot.logs,
            snapshot.file_io,
            snapshot.network_io,
            snapshot.syscalls,
            snapshot.feature_detection,
            snapshot.panic_unwinds,
        ],
        None => {
            fields.push(json_string_field("render_errors", "not_applicable"));
            for name in RENDER_AUDIT_FIELDS {
                fields.push(json_string_field(name, "not_applicable"));
            }
            fields.push(json_string_field(
                "render_total_forbidden_operations",
                "not_applicable",
            ));
            return;
        }
    };
    fields.push(json_number("render_errors", 0));
    for (name, value) in RENDER_AUDIT_FIELDS.into_iter().zip(values) {
        fields.push(json_number(name, value));
    }
    fields.push(json_number(
        "render_total_forbidden_operations",
        values.into_iter().sum::<u64>(),
    ));
}

const RENDER_AUDIT_FIELDS: [&str; 9] = [
    "render_allocations",
    "render_deallocations",
    "render_locks",
    "render_logs",
    "render_file_io",
    "render_network_io",
    "render_syscalls",
    "render_feature_detection",
    "render_panic_unwind",
];

const RECORD_KEYS: [&str; 61] = [
    "schema_version",
    "issue",
    "workload_kind",
    "workload_id",
    "sample_rate_hz",
    "quantum_frames",
    "round",
    "render_scope",
    "warmup_batches",
    "measured_batches",
    "operations_per_batch",
    "total_operations",
    "frames_per_operation",
    "tracks",
    "meter_observers",
    "meter_queue_capacity",
    "retained_payload_bytes",
    "percentile_method",
    "units",
    "min_ns",
    "p50_ns",
    "p95_ns",
    "p99_ns",
    "p99_9_ns",
    "max_ns",
    "descriptive_only",
    "candidate_commit",
    "binary_sha256",
    "fixture_manifest_id",
    "fixture_manifest_sha256",
    "input_fixture_id",
    "input_fixture_sha256",
    "output_sha256",
    "render_errors",
    "render_allocations",
    "render_deallocations",
    "render_locks",
    "render_logs",
    "render_file_io",
    "render_network_io",
    "render_syscalls",
    "render_feature_detection",
    "render_panic_unwind",
    "render_total_forbidden_operations",
    "cpu_model",
    "cpu_architecture",
    "logical_core_count",
    "physical_core_count",
    "os",
    "kernel",
    "governor_or_power_mode",
    "rust_version",
    "llvm_version",
    "target_triple",
    "target_features",
    "profile",
    "opt_level",
    "lto",
    "codegen_units",
    "background_load_note",
    "missing_metadata",
];

fn assert_record_keys(fields: &[String]) {
    assert_eq!(fields.len(), RECORD_KEYS.len(), "exact record key count");
    for (field, expected) in fields.iter().zip(RECORD_KEYS) {
        assert!(
            field.starts_with(&format!("\"{expected}\":")),
            "unexpected record field: {field}"
        );
    }
}

fn append_metadata_fields(fields: &mut Vec<String>, metadata: &Metadata) {
    fields.extend([
        json_optional_string("cpu_model", metadata.cpu_model.as_deref()),
        json_optional_string("cpu_architecture", metadata.cpu_architecture.as_deref()),
        json_optional_number("logical_core_count", metadata.logical_core_count),
        json_optional_number("physical_core_count", metadata.physical_core_count),
        json_optional_string("os", metadata.os.as_deref()),
        json_optional_string("kernel", metadata.kernel.as_deref()),
        json_optional_string(
            "governor_or_power_mode",
            metadata.governor_or_power_mode.as_deref(),
        ),
        json_optional_string("rust_version", metadata.rust_version.as_deref()),
        json_optional_string("llvm_version", metadata.llvm_version.as_deref()),
        json_optional_string("target_triple", metadata.target_triple.as_deref()),
        json_optional_string("target_features", metadata.target_features.as_deref()),
        json_optional_string("profile", metadata.profile.as_deref()),
        json_optional_string("opt_level", metadata.opt_level.as_deref()),
        json_optional_string("lto", metadata.lto.as_deref()),
        json_optional_string("codegen_units", metadata.codegen_units.as_deref()),
        json_optional_string(
            "background_load_note",
            metadata.background_load_note.as_deref(),
        ),
        format!(
            "\"missing_metadata\":[{}]",
            metadata
                .missing
                .iter()
                .map(|field| json_string(field))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);
}

fn json_number(name: &str, value: impl core::fmt::Display) -> String {
    format!("\"{name}\":{value}")
}

fn json_optional_number(name: &str, value: Option<impl core::fmt::Display>) -> String {
    match value {
        Some(value) => json_number(name, value),
        None => json_raw(name, "null".to_owned()),
    }
}

fn json_string_field(name: &str, value: &str) -> String {
    format!("\"{name}\":{}", json_string(value))
}

fn json_optional_string(name: &str, value: Option<&str>) -> String {
    match value {
        Some(value) => json_string_field(name, value),
        None => json_raw(name, "null".to_owned()),
    }
}

fn json_raw(name: &str, value: String) -> String {
    format!("\"{name}\":{value}")
}

fn json_string(value: &str) -> String {
    format!("\"{}\"", json::escape(value))
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
        let rank = |n: usize, d: usize| stats::nearest_rank(&sorted, n, d);
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
#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_measurement(workload: Workload) -> Measurement {
        Measurement {
            samples_ns: vec![3, 1, 2],
            output_sha256: "c".repeat(64),
            shape: workload_shape(workload),
            audit: if workload.is_prepare() {
                None
            } else {
                Some(audit::AuditSnapshot::default())
            },
        }
    }

    #[test]
    fn issue035_record_has_exact_render_identity_shape_and_typed_missing_metadata() {
        let record = record_json(
            Workload::FullChainFilters,
            48_000,
            1,
            INPUT_MANIFEST_SHA256,
            &BenchmarkIdentities::synthetic(),
            &Metadata::all_missing(),
            &synthetic_measurement(Workload::FullChainFilters),
        );

        assert_eq!(record.matches("\":").count(), RECORD_KEYS.len());
        assert!(record.contains("\"issue\":35"));
        assert!(record.contains("\"workload_id\":\"issue035.full_chain_filters.48000hz.q128\""));
        assert!(record.contains("\"total_operations\":4096"));
        assert!(record.contains("\"units\":\"ns_per_operation\""));
        assert!(record.contains("\"frames_per_operation\":128"));
        assert!(record.contains("\"meter_queue_capacity\":null"));
        assert!(record.contains(
            "\"input_fixture_id\":\"fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml\""
        ));
        assert!(record.contains("\"input_fixture_sha256\":\"178b35953960ded3166157b3d781d2aeac0d033789925d77aec6f57bab084d7d\""));
        assert!(record.contains("\"render_errors\":0"));
        assert!(record.contains("\"render_feature_detection\":0"));
        assert!(record.contains("\"render_panic_unwind\":0"));
        assert!(record.contains("\"render_total_forbidden_operations\":0"));
        assert!(record.contains("\"cpu_model\":null"));
        assert!(record.contains("\"logical_core_count\":null"));
        assert!(record.contains("\"missing_metadata\":[\"background_load_note\",\"codegen_units\",\"cpu_architecture\",\"cpu_model\",\"governor_or_power_mode\",\"kernel\",\"llvm_version\",\"logical_core_count\",\"lto\",\"opt_level\",\"os\",\"physical_core_count\",\"profile\",\"rust_version\",\"target_features\",\"target_triple\"]"));
    }

    #[test]
    fn issue035_preparation_record_uses_only_required_null_and_not_applicable_values() {
        let mut measurement = synthetic_measurement(Workload::Prepare256Tracks);
        measurement.shape.retained_payload_bytes = 123;
        let record = record_json(
            Workload::Prepare256Tracks,
            96_000,
            2,
            INPUT_MANIFEST_SHA256,
            &BenchmarkIdentities::synthetic(),
            &Metadata::all_missing(),
            &measurement,
        );

        assert!(record.contains("\"frames_per_operation\":null"));
        assert!(record.contains("\"total_operations\":128"));
        assert!(record.contains("\"tracks\":256"));
        assert!(record.contains("\"meter_observers\":56"));
        assert!(record.contains("\"meter_queue_capacity\":4"));
        assert!(record.contains("\"retained_payload_bytes\":123"));
        assert!(record.contains("\"render_scope\":\"not_applicable_preparation\""));
        assert!(record.contains("\"render_errors\":\"not_applicable\""));
        assert!(record.contains("\"render_feature_detection\":\"not_applicable\""));
        assert!(record.contains("\"render_total_forbidden_operations\":\"not_applicable\""));
        assert!(record.contains(
            "\"input_fixture_id\":\"fixtures/builtins/v1/benchmark/prepare_256_tracks-96000.toml\""
        ));
    }

    #[test]
    fn measured_plan_is_the_frozen_twenty_row_cartesian_set() {
        let plans = measured_record_plans();
        assert_eq!(plans.len(), 20);
        for workload in WORKLOADS {
            for rate_hz in RATES {
                let matching: Vec<_> = plans
                    .iter()
                    .filter(|plan| plan.workload == workload && plan.rate_hz == rate_hz)
                    .collect();
                assert_eq!(matching.len(), 2);
                assert_eq!(matching[0].round, 1);
                assert_eq!(matching[0].round_index, 0);
                assert_eq!(matching[1].round, 2);
                assert_eq!(matching[1].round_index, 1);
            }
        }
    }

    #[test]
    fn matrix_targets_alternate_by_global_operation_across_batch_boundaries() {
        let fixture = input_fixture(Workload::MatrixRamp, 48_000);
        let (_, targets) = render_parameters_from_fixture(&fixture, Workload::MatrixRamp);
        let targets = targets.expect("checked matrix targets");
        assert_eq!(
            matrix_target_for_operation(0, 6, targets).ll.to_bits(),
            0.6_f32.to_bits()
        );
        assert_eq!(
            matrix_target_for_operation(0, 7, targets).ll.to_bits(),
            0.9_f32.to_bits()
        );
        assert_eq!(
            matrix_target_for_operation(1, 0, targets).ll.to_bits(),
            0.6_f32.to_bits()
        );
        assert_eq!(
            matrix_target_for_operation(1, 1, targets).ll.to_bits(),
            0.9_f32.to_bits()
        );
    }

    #[test]
    fn checked_toml_and_referenced_pcm_drive_render_configuration_bit_exactly() {
        for workload in [
            Workload::FullChainFilters,
            Workload::IdentityChain,
            Workload::MatrixRamp,
            Workload::MeterSuccessFull,
        ] {
            for rate_hz in RATES {
                let fixture = input_fixture(workload, rate_hz);
                fixture.validate_common(workload, rate_hz);
                let pcm = fixture.pcm();
                assert_eq!(pcm.left.len(), pcm.right.len());
                assert!(!pcm.left.is_empty());
            }
        }

        let fixture = input_fixture(Workload::IdentityChain, 48_000);
        let pcm = fixture.pcm();
        // The identity chain's two disabled sections are the arithmetic identity, and their
        // trailing `+ 0.0` normalises a negative zero (#85, class B): the fixture's `-0.0` inputs
        // therefore arrive here as `+0.0`, uniformly at every width and on every target.
        assert_eq!(pcm.left[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(pcm.left[1].to_bits(), 0.0_f32.to_bits());
        assert_eq!(pcm.right[0].to_bits(), 0.0_f32.to_bits());
        assert_eq!(pcm.right[1].to_bits(), 0.0_f32.to_bits());
        let (parameters, _) = render_parameters_from_fixture(&fixture, Workload::IdentityChain);
        assert_eq!(parameters.matrix, Matrix2x2::IDENTITY);
        assert_eq!(parameters.left.trim_db.to_bits(), 0.0_f32.to_bits());
    }

    #[test]
    fn all_render_workloads_arm_only_product_render_without_timing() {
        audit::warm_up();
        for workload in [
            Workload::FullChainFilters,
            Workload::IdentityChain,
            Workload::MatrixRamp,
            Workload::MeterSuccessFull,
        ] {
            for rate_hz in RATES {
                let mut rounds = [
                    RenderRuntime::new(workload, rate_hz),
                    RenderRuntime::new(workload, rate_hz),
                ];
                for batch in 0..RENDER_WARMUP_BATCHES {
                    for runtime in &mut rounds {
                        runtime.run_batch(batch as u64);
                    }
                }

                let mut first_output = None;
                for runtime in &mut rounds {
                    runtime.begin_measurement();
                    audit::reset();
                    let logical_batch = RENDER_WARMUP_BATCHES as u64;
                    runtime.prepare_operation_input(logical_batch, 0);
                    assert!(!audit::is_render_scope_active());
                    runtime.run_render_operation(logical_batch, 0);
                    assert!(!audit::is_render_scope_active());
                    assert_eq!(audit::snapshot(), audit::AuditSnapshot::default());

                    let success_taps = runtime.collect_operation_evidence(true);
                    assert_eq!(audit::snapshot(), audit::AuditSnapshot::default());
                    if let Some(meter) = &runtime.meter_runtime {
                        assert_eq!(success_taps, OBSERVERS);
                        assert_eq!(
                            meter.full_drop_attempts,
                            meter.measurement_start_full_drop_attempts + 1
                        );
                    } else {
                        assert_eq!(success_taps, 0);
                    }

                    let output = runtime.output_sha256();
                    if let Some(first) = &first_output {
                        assert_eq!(&output, first, "identically warmed round state");
                    } else {
                        first_output = Some(output);
                    }
                }
            }
        }
    }

    #[test]
    fn benchmark_inputs_take_their_hashes_from_the_checked_manifest_rows() {
        for plan in measured_record_plans() {
            let input = input_fixture(plan.workload, plan.rate_hz);
            assert_eq!(sha256(input.bytes), manifest_input_sha256(input.id));
        }
        assert_eq!(
            manifest_input_sha256("fixtures/builtins/v1/benchmark/meter_success_full-48000.toml"),
            "95904e939716b6dd8de19c5cc92050ba13ef7e4b9d41a212b135c559d0b032a0"
        );
    }

    #[test]
    fn real_meter_tap_plans_use_the_compiled_seven_taps_and_preserve_full_queue_state() {
        let fixture = input_fixture(Workload::MeterSuccessFull, 48_000);
        let pcm = fixture.pcm();
        let pair = prepare_real_meter_tap_artifacts(48_000, meter_config_from_fixture(&fixture));
        assert_eq!(pair.success.report(), pair.full.report());
        let mut success = RealMeterTapPlan::bind(pair.success, pcm.clone());
        let mut full = RealMeterTapPlan::bind(pair.full, pcm.clone());
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
        let full_windows = full.drain_all_direct(|record| {
            assert_eq!(record.snapshot.end_sample, QUANTUM as u64);
            assert_eq!(record.snapshot.reset_generation, 7);
        });
        assert_eq!(full_windows, OBSERVERS);
        full.render(QUANTUM as u64 * 2);
        let post_drop = full.drain_all_direct(|record| {
            assert_eq!(record.snapshot.cumulative_dropped_snapshots, 1);
        });
        assert_eq!(post_drop, OBSERVERS);

        success.render(QUANTUM as u64);
        let success_windows = success.drain_all_direct(|record| {
            assert_eq!(record.snapshot.end_sample, QUANTUM as u64 * 2);
            assert_eq!(record.snapshot.reset_generation, 7);
        });
        assert_eq!(success_windows, OBSERVERS);
    }

    #[test]
    fn preparation_projection_is_complete_address_free_and_deterministic() {
        let first = prepare_256_tracks(48_000);
        let second = prepare_256_tracks(48_000);
        let mut first_hash = Sha256::new();
        let mut second_hash = Sha256::new();
        hash_preparation_projection(&mut first_hash, &first);
        hash_preparation_projection(&mut second_hash, &second);
        assert_eq!(first_hash.finalize(), second_hash.finalize());
        assert_eq!(first.processor_count(), PREPARE_TRACKS * 3);
        assert_eq!(first.tail_count(), PREPARE_TRACKS);
        assert_eq!(first.observer_count(), OBSERVERS * 8);
        assert_eq!(first.meter_consumer_count(), OBSERVERS * 8);
        assert_eq!(
            first.resource_report().meter_items,
            (OBSERVERS * 8 * 5) as u64
        );
        assert!(!first.resource_report().retained_layouts().is_empty());
    }
}
