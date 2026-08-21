//! Deterministic issue-007 expected-output fixture generator and checker.

use core::fmt::Write as _;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameterError, BuiltinParameters, BuiltinResetKind, ChannelParameters,
    DualMonoBlock, Matrix2x2, MeterAccumulator, MeterConfig, MeterHandle, MeterTap,
};
use miso_engine_builtins_compiler::{BuiltinCompileCaps, MeterRequest, prepare_session_builtins};
use miso_engine_core::realtime::{PlanarBufferMut, RenderError, RenderIo, RenderTime};
use miso_engine_dsp_reference::{
    ReferenceFilterKind, ReferenceRetainedTptF32, ReferenceTptOutput, rbj_butterworth_magnitude_db,
};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};
use sha2::{Digest, Sha256};

const MANIFEST_HEADER: &str = "path\tlength\tsha256\n";
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const QUANTA: [u32; 5] = [1, 127, 128, 255, 1_024];
const CASE_COUNT_V1: usize = 1_652;
const RESPONSE_CASE_COUNT_V1: usize = 1_630;
const RESPONSE_ROW_COUNT_V1: usize = 1_630;
const PCM_PAYLOAD_COUNT_V1: usize = 33;
const BENCHMARK_RATES_V1: [u32; 2] = [48_000, 96_000];
const BENCHMARK_KINDS_V1: [&str; 5] = [
    "full_chain_filters",
    "identity_chain",
    "matrix_ramp",
    "meter_success_full",
    "prepare_256_tracks",
];
const RESPONSE_CAST_STATE_TOLERANCE_DB_V1: f64 = 0.005;
const RESPONSE_IMPULSE_DFT_TOLERANCE_DB_V1: f64 = 0.05;
const RESPONSE_FUNDAMENTAL_TOLERANCE_DB_V1: f64 = 0.05;
const RESPONSE_RESIDUAL_LIMIT_DB_V1: f64 = -100.0;
const RESPONSE_ATTENUATED_TOTAL_LIMIT_DB_V1: f64 = -88.0;
const RESPONSE_RBJ_SERIALIZATION_TOLERANCE_DB_V1: f64 = 5e-12;
const METADATA_V1: &str = concat!(
    "fixture_schema = 1\n",
    "producer = \"miso-engine-builtins-fixture\"\n",
    "production_pcm = \"miso-engine-builtins scalar f32, planar L then R\"\n",
    "independent_response_oracle = \"miso-engine-dsp-reference::rbj_butterworth_magnitude_db\"\n",
    "oracle_dependency_rule = \"miso-engine-dsp-reference has no production-builtin dependency\"\n",
    "launch_rates_hz = [44100, 48000, 88200, 96000]\n",
    "quanta_frames = [1, 127, 128, 255, 1024]\n"
);
const FUNCTIONAL_CASES_V1: [(&str, &str, &str); 22] = [
    (
        "identity-signed-zero",
        "pcm",
        "planar_l_then_r; signed_zero",
    ),
    ("polarity-gain", "pcm", "per_lane_polarity_trim_fader"),
    ("mute", "pcm", "per_lane_mute"),
    ("filters-asymmetric", "pcm", "left_hpf_right_lpf"),
    ("matrix-corners", "pcm", "all_16_binary_2x2_matrices"),
    (
        "matrix-ramp",
        "pcm",
        "updates=0,1,2,127,128,u32_max_bounded_prefix",
    ),
    ("matrix-retarget", "pcm", "mid_ramp_target_replacement"),
    ("reset", "pcm", "discontinuity_and_full_reset"),
    ("lr-isolation", "pcm", "independent_filter_state"),
    ("partition", "pcm", "all_declared_block_splits"),
    ("graph-taps", "graph", "seven_taps_and_output_pcm"),
    ("meter-partial", "meter", "incomplete_window"),
    ("meter-multiple", "meter", "multiple_windows"),
    ("meter-wrap", "meter", "ring_wrap_with_interleaved_drain"),
    ("meter-full-drop", "meter", "full_queue_and_loss_counter"),
    ("meter-drain", "meter", "consumer_drain"),
    ("meter-discontinuity", "meter", "noncontiguous_sample_time"),
    ("meter-reset", "meter", "both_reset_modes"),
    ("meter-overflow", "meter", "sample_time_overflow"),
    ("meter-sanitization", "meter", "nan_and_infinity"),
    ("diagnostics", "diagnostic", "exact_code_and_path_tuples"),
    (
        "resource-grid",
        "resource",
        "tracks=1,4,65537; meters=0,1,7",
    ),
];
const PCM_INPUT_LEFT_V1: [f32; 8] = [0.0, -0.0, 0.25, -0.5, 1.0, -1.0, 0.125, -0.25];
const PCM_INPUT_RIGHT_V1: [f32; 8] = [-0.0, 0.0, -0.125, 0.5, -1.0, 1.0, -0.25, 0.25];

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [mode, root] if mode == "--write" => write_and_verify(Path::new(root)),
        [mode, root] if mode == "--check" => check_read_only_fixture_root_v1(Path::new(root)),
        _ => {
            Err("usage: miso_engine_builtins_fixture --write|--check SCRATCH_DIRECTORY".to_owned())
        }
    }
}

/// The checked-in builtin fixture layout.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum FixturePathClassV1 {
    /// The expanded fixture tuple list.
    Cases,
    /// Headerless planar PCM expected bytes.
    Pcm,
    /// A frozen benchmark input bundle.
    Benchmark,
    /// Independent response data.
    Reference,
    /// Expected meter records.
    Meter,
    /// Exact invalid-input records.
    Diagnostics,
    /// Prepared-resource records.
    Resources,
    /// Fixture provenance and schema metadata.
    Metadata,
}

/// A parsed, byte-addressable fixture-manifest row.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FixtureManifestEntryV1 {
    path: String,
    length: u64,
    sha256: String,
    class: FixturePathClassV1,
}

/// The strictly sorted manifest that names every fixture payload.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FixtureManifestV1 {
    entries: Vec<FixtureManifestEntryV1>,
}

/// One exact non-response declaration from `cases.toml`.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FunctionalCaseV1 {
    id: String,
    category: String,
    rate_hz: u32,
    quantum_frames: u32,
    detail: String,
}

/// The two typed subsets of the complete cases declaration file.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VerifiedCasesV1 {
    response_ids: BTreeSet<String>,
    functional_cases: BTreeMap<String, FunctionalCaseV1>,
}

/// One parsed independent response row from the checked V1 CSV.
#[derive(Clone, Debug)]
struct ResponseCsvRowV1 {
    id: String,
    rate_hz: u32,
    section: ResponseSectionV1,
    cutoff_hz: f64,
    probe_hz: f64,
    quantum_frames: u32,
    rbj_magnitude_db: f64,
    cast_state_magnitude_db: f64,
    impulse_dft_magnitude_db: f64,
    sustained_fundamental_db: f64,
    sustained_residual_db: f64,
    sustained_total_db: f64,
    tail_energy: f64,
    recovery_count: u64,
}

/// The frozen response topology encoded by one CSV row.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ResponseSectionV1 {
    /// One high-pass Butterworth section.
    HighPass,
    /// One low-pass Butterworth section.
    LowPass,
    /// A fixed 100-Hz HPF followed by a fixed 1-kHz LPF.
    Cascade,
}

/// One rate/quantum/section/cutoff/probe response coordinate.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ResponseCoordinateV1 {
    rate_hz: u32,
    section: ResponseSectionV1,
    cutoff_bits: u64,
    probe_bits: u64,
    quantum_frames: u32,
}

/// One strict, complete benchmark input declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
struct BenchmarkInputV1 {
    kind: BenchmarkKindV1,
    rate_hz: u32,
    fields: Vec<(String, String)>,
}

/// The frozen benchmark input kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BenchmarkKindV1 {
    /// One asymmetric filter/matrix render track.
    FullChainFilters,
    /// One exact-identity render track.
    IdentityChain,
    /// One matrix-ramp render track.
    MatrixRamp,
    /// One drain/full seven-tap meter render track.
    MeterSuccessFull,
    /// One off-render 256-track preparation workload.
    Prepare256Tracks,
}

fn generated() -> Vec<(String, Vec<u8>)> {
    let (graph_pcm, graph_meters) = graph_tap_fixtures();
    let mut files = vec![
        ("cases.toml".to_owned(), cases().into_bytes()),
        ("diagnostics.jsonl".to_owned(), diagnostics().into_bytes()),
        ("metadata.toml".to_owned(), metadata().into_bytes()),
        (
            "meters/window-and-drop.jsonl".to_owned(),
            meters().into_bytes(),
        ),
        (
            "reference/filter-response.csv".to_owned(),
            responses().into_bytes(),
        ),
        (
            "meters/graph-taps.jsonl".to_owned(),
            graph_meters.into_bytes(),
        ),
        ("pcm/graph-taps.f32le".to_owned(), graph_pcm),
        ("resources.jsonl".to_owned(), resources().into_bytes()),
    ];
    for (id, pcm) in pcm_cases() {
        files.push((format!("pcm/{id}.f32le"), pcm));
    }
    for kind in BENCHMARK_KINDS_V1 {
        let kind = BenchmarkKindV1::parse(kind).expect("frozen benchmark kind");
        for rate_hz in BENCHMARK_RATES_V1 {
            files.push((
                format!("benchmark/{}-{rate_hz}.toml", kind.as_str()),
                canonical_benchmark_input_v1(kind, rate_hz).into_bytes(),
            ));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

struct FixtureSource;
impl GraphRuntimeProcessor for FixtureSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            *left = 0.125 + index as f32 * 0.001;
            *right = -0.25 - index as f32 * 0.002;
        }
        Ok(())
    }
}

struct FixtureIdentity;
impl GraphRuntimeProcessor for FixtureIdentity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

fn graph_tap_fixtures() -> (Vec<u8>, String) {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    model.tracks[0].simd1.effects.clear();
    model.tracks[0].dynamic.effects.clear();
    model.tracks[0].simd2.effects.clear();
    model.automation.clear();
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
    .expect("compile session");
    let config = MeterConfig {
        period_frames: core::num::NonZeroU32::new(session.quantum().0).expect("quantum"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: core::num::NonZeroUsize::new(4).expect("capacity"),
        reset_generation: 0,
    };
    let requests: Vec<_> = [
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
        handle: MeterHandle(
            core::num::NonZeroU64::new(u64::try_from(index).expect("bounded") + 1)
                .expect("nonzero"),
        ),
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect();
    let builtins = prepare_session_builtins(
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
    .expect("prepare builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 7,
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
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|_| panic!("compile graph"));
    let envelope = artifact.envelope();
    let bindings = artifact
        .external_binding_nodes()
        .cloned()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = matches!(
                node,
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                }
            )
            .then(|| Box::new(FixtureSource) as Box<dyn GraphRuntimeProcessor>)
            .unwrap_or_else(|| Box::new(FixtureIdentity));
            GraphNodeBinding::new(node, processor)
        })
        .collect();
    let bound = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers: Vec::new(),
        })
        .unwrap_or_else(|_| panic!("bind graph"));
    let mut plan = bound.plan;
    let frames = envelope.quantum.0 as usize;
    let mut pcm = vec![0.0_f32; frames * 2];
    plan.render(
        RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
        },
        RenderTime { absolute_sample: 0 },
    )
    .expect("render graph");
    let mut records = Vec::new();
    for mut consumer in bound.meter_consumers {
        let snapshot = consumer
            .consumer
            .try_pop()
            .expect("one window per graph tap");
        records.push(format!(
            "{{\"tap\":\"{:?}\",\"snapshot\":{}}}",
            consumer.tap,
            meter_snapshot_json("graph-taps", snapshot)
        ));
    }
    records.sort();
    (
        pcm.into_iter().flat_map(f32::to_le_bytes).collect(),
        records.join("\n") + "\n",
    )
}

fn metadata() -> String {
    METADATA_V1.to_owned()
}

fn cases() -> String {
    let mut entries = Vec::new();
    for rate in RATES {
        for quantum in QUANTA {
            for section in ["high_pass", "low_pass"] {
                for (cutoff_index, cutoff) in response_cutoffs(rate).into_iter().enumerate() {
                    for (probe_index, probe) in frozen_single_section_probes_v1(rate, cutoff)
                        .into_iter()
                        .enumerate()
                    {
                        entries.push((
                            format!("response-{section}-{rate}-{quantum}-{cutoff_index}-{probe_index}"),
                            format!(
                                "category = \"filter_response\"\nrate_hz = {rate}\nquantum_frames = {quantum}\nsection = \"{section}\"\ncutoff_hz = {cutoff:.17}\nprobe_hz = {probe:.17}\noracle = \"rbj_f64_and_cast_state\"\n"
                            ),
                        ));
                    }
                }
            }
            for (probe_index, probe) in frozen_cascade_probes_v1(rate).into_iter().enumerate() {
                entries.push((
                    format!("response-cascade-{rate}-{quantum}-fixed-{probe_index}"),
                    format!(
                        "category = \"filter_response\"\nrate_hz = {rate}\nquantum_frames = {quantum}\nsection = \"cascade\"\ncutoff_hz = 100.00000000000000000\nprobe_hz = {probe:.17}\noracle = \"rbj_f64_and_cast_state\"\n"
                    ),
                ));
            }
        }
    }
    for (id, category, detail) in FUNCTIONAL_CASES_V1 {
        entries.push((id.to_owned(), format!("category = \"{category}\"\nrate_hz = 48000\nquantum_frames = 128\ndetail = \"{detail}\"\n")));
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut output = String::from("fixture_schema = 1\n\n");
    for (index, (id, body)) in entries.into_iter().enumerate() {
        if index != 0 {
            output.push('\n');
        }
        write!(output, "[[case]]\nid = \"{id}\"\n{body}").expect("string");
    }
    output
}

fn response_cutoffs(rate: u32) -> [f64; 6] {
    [
        10.0,
        20.0,
        100.0,
        1000.0,
        20_000.0_f64.min(0.1 * f64::from(rate)),
        0.45 * f64::from(rate),
    ]
}

fn responses() -> String {
    let mut output = String::from(
        "case,rate_hz,section,cutoff_hz,probe_hz,quantum_frames,rbj_magnitude_db,cast_state_magnitude_db,impulse_dft_magnitude_db,sustained_fundamental_db,sustained_residual_db,sustained_total_db,tail_energy,recovery_count\n",
    );
    for rate in RATES {
        for (section, kind) in [
            ("high_pass", ReferenceFilterKind::HighPass),
            ("low_pass", ReferenceFilterKind::LowPass),
        ] {
            for (cutoff_index, cutoff) in response_cutoffs(rate).into_iter().enumerate() {
                for (probe_index, probe) in frozen_single_section_probes_v1(rate, cutoff)
                    .into_iter()
                    .enumerate()
                {
                    let rbj = rbj_butterworth_magnitude_db(f64::from(rate), cutoff, kind, probe)
                        .expect("independent RBJ oracle");
                    for quantum in QUANTA {
                        let measurement = measure_response(rate, section, cutoff, probe, quantum);
                        writeln!(output, "response-{section}-{rate}-{quantum}-{cutoff_index}-{probe_index},{rate},{section},{cutoff:.17},{probe:.17},{quantum},{rbj:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{}", measurement.0, measurement.1, measurement.2, measurement.3, measurement.4, measurement.5, measurement.6).expect("string");
                    }
                }
            }
        }
        for (probe_index, probe) in frozen_cascade_probes_v1(rate).into_iter().enumerate() {
            let rbj = rbj_butterworth_magnitude_db(
                f64::from(rate),
                100.0,
                ReferenceFilterKind::HighPass,
                probe,
            )
            .expect("independent HPF RBJ oracle")
                + rbj_butterworth_magnitude_db(
                    f64::from(rate),
                    1_000.0,
                    ReferenceFilterKind::LowPass,
                    probe,
                )
                .expect("independent LPF RBJ oracle");
            for quantum in QUANTA {
                let measurement = measure_response(rate, "cascade", 100.0, probe, quantum);
                writeln!(output, "response-cascade-{rate}-{quantum}-fixed-{probe_index},{rate},cascade,100.00000000000000000,{probe:.17},{quantum},{rbj:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{}", measurement.0, measurement.1, measurement.2, measurement.3, measurement.4, measurement.5, measurement.6).expect("string");
            }
        }
    }
    output
}

fn measure_response(
    rate: u32,
    section: &str,
    cutoff: f64,
    probe: f64,
    quantum: u32,
) -> (f64, f64, f64, f64, f64, f64, u64) {
    let mut parameters = BuiltinParameters::default();
    match section {
        "high_pass" => {
            parameters.left.hpf_hz = cutoff as f32;
            parameters.right.hpf_hz = cutoff as f32;
        }
        "low_pass" => {
            parameters.left.lpf_hz = cutoff as f32;
            parameters.right.lpf_hz = cutoff as f32;
        }
        "cascade" => {
            debug_assert_eq!(cutoff.to_bits(), 100.0_f64.to_bits());
            parameters.left.hpf_hz = 100.0;
            parameters.left.lpf_hz = 1_000.0;
            parameters.right.hpf_hz = 100.0;
            parameters.right.lpf_hz = 1_000.0;
        }
        _ => panic!("unknown response section"),
    }
    let mut chain = BuiltinChain::new(rate, parameters).expect("fixture filter");
    let mut impulse = Vec::with_capacity(rate as usize);
    let mut left = vec![0.0_f32; quantum as usize];
    let mut right = vec![0.0_f32; quantum as usize];
    let mut recoveries = 0_u64;
    for start in (0..rate as usize).step_by(quantum as usize) {
        let frames = (rate as usize - start).min(quantum as usize);
        left[..frames].fill(0.0);
        right[..frames].fill(0.0);
        if start == 0 {
            left[0] = 1.0;
            right[0] = 1.0;
        }
        let report = chain
            .process_dual_mono(
                DualMonoBlock::new(&mut left[..frames], &mut right[..frames], start as u64)
                    .expect("block"),
            )
            .expect("render");
        recoveries += report.recovered_left_state + report.recovered_right_state;
        impulse.extend_from_slice(&left[..frames]);
    }
    let impulse_db = dft_magnitude_db(&impulse, f64::from(rate), probe);
    let tail_energy = impulse[impulse.len().saturating_sub(4096)..]
        .iter()
        .map(|value| f64::from(*value).powi(2))
        .sum();
    let (fundamental, residual, total) = sustained_metrics(rate, quantum, parameters, probe);
    (
        cast_state_response_db(rate, section, cutoff, probe),
        impulse_db,
        fundamental,
        residual,
        total,
        tail_energy,
        recoveries,
    )
}

fn cast_state_response_db(rate: u32, section: &str, cutoff: f64, probe: f64) -> f64 {
    match section {
        "high_pass" => cast_state_magnitude_db(rate, cutoff as f32, true, probe),
        "low_pass" => cast_state_magnitude_db(rate, cutoff as f32, false, probe),
        "cascade" => {
            debug_assert_eq!(cutoff.to_bits(), 100.0_f64.to_bits());
            cast_state_magnitude_db(rate, 100.0, true, probe)
                + cast_state_magnitude_db(rate, 1_000.0, false, probe)
        }
        _ => panic!("unknown response section"),
    }
}

fn dft_magnitude_db(samples: &[f32], rate: f64, frequency: f64) -> f64 {
    let phase = -core::f64::consts::TAU * frequency / rate;
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
    for sample in samples {
        let value = f64::from(*sample);
        real += value * unit_re;
        imaginary += value * unit_im;
        (unit_re, unit_im) = (
            unit_re * step_re - unit_im * step_im,
            unit_re * step_im + unit_im * step_re,
        );
    }
    20.0 * real.hypot(imaginary).log10()
}

#[allow(clippy::needless_range_loop)]
fn sustained_metrics(
    rate: u32,
    quantum: u32,
    parameters: BuiltinParameters,
    frequency: f64,
) -> (f64, f64, f64) {
    let mut chain = BuiltinChain::new(rate, parameters).expect("fixture filter");
    let settle = rate as usize / 2;
    let frames = rate as usize / 4;
    let mut samples = Vec::with_capacity(frames);
    let mut input_energy = 0.0;
    let mut output_energy = 0.0;
    let mut left = vec![0.0_f32; quantum as usize];
    let mut right = vec![0.0_f32; quantum as usize];
    for start in (0..settle + frames).step_by(quantum as usize) {
        let count = (settle + frames - start).min(quantum as usize);
        for (index, (left, right)) in left[..count]
            .iter_mut()
            .zip(right[..count].iter_mut())
            .enumerate()
        {
            let input = (0.5
                * (core::f64::consts::TAU * frequency * (start + index) as f64 / f64::from(rate))
                    .sin()) as f32;
            *left = input;
            *right = input;
            if start + index >= settle {
                input_energy += f64::from(input).powi(2);
            }
        }
        chain
            .process_dual_mono(
                DualMonoBlock::new(&mut left[..count], &mut right[..count], start as u64)
                    .expect("block"),
            )
            .expect("render");
        for index in 0..count {
            if start + index >= settle {
                samples.push(f64::from(left[index]));
                output_energy += f64::from(left[index]).powi(2);
            }
        }
    }
    let count = frames as f64;
    let (mut sin_sum, mut cos_sum, mut dc_sum) = (0.0, 0.0, 0.0);
    for (index, sample) in samples.iter().enumerate() {
        let phase = core::f64::consts::TAU * frequency * (settle + index) as f64 / f64::from(rate);
        sin_sum += sample * phase.sin();
        cos_sum += sample * phase.cos();
        dc_sum += sample;
    }
    let dc = dc_sum / count;
    let sine = 2.0 * sin_sum / count;
    let cosine = 2.0 * cos_sum / count;
    let residual: f64 = samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            let phase =
                core::f64::consts::TAU * frequency * (settle + index) as f64 / f64::from(rate);
            (sample - (dc + sine * phase.sin() + cosine * phase.cos())).powi(2)
        })
        .sum();
    let input_rms = (input_energy / count).sqrt();
    (
        20.0 * (sine.hypot(cosine) / 0.5).log10(),
        20.0 * ((residual / count).sqrt() / input_rms).log10(),
        20.0 * ((output_energy / count).sqrt() / input_rms).log10(),
    )
}

fn cast_state_magnitude_db(rate: u32, cutoff: f32, high_pass: bool, frequency: f64) -> f64 {
    let g = (core::f64::consts::PI * f64::from(cutoff) / f64::from(rate)).tan();
    let k = core::f64::consts::SQRT_2 as f32;
    let denominator = 1.0 + g * (g + core::f64::consts::SQRT_2);
    let (c1, a2, a3) = (
        (g * (g + core::f64::consts::SQRT_2) / denominator) as f32,
        (g / denominator) as f32,
        (g * g / denominator) as f32,
    );
    let (c1, a2, a3, k) = (f64::from(c1), f64::from(a2), f64::from(a3), f64::from(k));
    let (a00, a01, a10, a11) = (1.0 - 2.0 * c1, -2.0 * a2, 2.0 * a2, 1.0 - 2.0 * a3);
    let (b0, b1) = (2.0 * a2, 2.0 * a3);
    let (c0, c_1, direct) = if high_pass {
        (-k * (1.0 - c1) - a2, k * a2 - (1.0 - a3), 1.0 - k * a2 - a3)
    } else {
        (a2, 1.0 - a3, a3)
    };
    let phase = core::f64::consts::TAU * frequency / f64::from(rate);
    let (zr, zi) = (phase.cos(), phase.sin());
    let (m00, m01, m10, m11) = (zr - a00, -a01, -a10, zr - a11);
    let (det_re, det_im) = (m00 * m11 - zi * zi - m01 * m10, zi * (m00 + m11));
    let norm = det_re * det_re + det_im * det_im;
    let divide = |real: f64, imaginary: f64| {
        (
            (real * det_re + imaginary * det_im) / norm,
            (imaginary * det_re - real * det_im) / norm,
        )
    };
    let (s0r, s0i) = divide(m11 * b0 - m01 * b1, zi * b0);
    let (s1r, s1i) = divide(-m10 * b0 + m00 * b1, zi * b1);
    20.0 * (direct + c0 * s0r + c_1 * s1r)
        .hypot(c0 * s0i + c_1 * s1i)
        .log10()
}

fn probes(rate: u32, cutoff: f64) -> Vec<f64> {
    let nyquist = 0.5 * f64::from(rate);
    let mut values: Vec<_> = [
        0.25 * cutoff,
        cutoff,
        4.0 * cutoff,
        0.2 * f64::from(rate),
        0.45 * f64::from(rate),
    ]
    .into_iter()
    .map(|value| (value.clamp(4.0, nyquist - 4.0) / 4.0).round() * 4.0)
    .collect();
    values.sort_by(f64::total_cmp);
    values.dedup();
    values
}

fn diagnostics() -> String {
    // Do not hand-maintain diagnostic declarations: each tuple below is obtained from
    // the public production API that emits it.  The path is the stable request/session
    // path supplied to that API, so this fixture catches both code and path drift.
    let mut rows = Vec::new();
    let direct = |case: &str, parameters: BuiltinParameters, path: &str| {
        let Err(error) = BuiltinChain::new(48_000, parameters) else {
            panic!("invalid fixture input accepted");
        };
        let code = match error {
            BuiltinParameterError::FilterCutoff
            | BuiltinParameterError::FilterOrder
            | BuiltinParameterError::FilterCoefficients => "builtin.filter.cutoff",
            BuiltinParameterError::GainDomain => "builtin.gain.domain",
            BuiltinParameterError::MatrixCoefficient | BuiltinParameterError::MatrixSmoothing => {
                "builtin.matrix.coefficient"
            }
            BuiltinParameterError::LaneLength | BuiltinParameterError::EmptyBlock => {
                "builtin.block.length"
            }
            BuiltinParameterError::SampleTimeOverflow => "builtin.block.sample_time_overflow",
        };
        format!("{{\"case\":\"{case}\",\"code\":\"{code}\",\"path\":\"{path}\"}}")
    };
    rows.push(direct(
        "filter-cutoff",
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: -1.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].builtins.left.hpf_hz",
    ));
    rows.push(direct(
        "filter-order",
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 1_000.0,
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].builtins.left",
    ));
    rows.push(direct(
        "gain-domain",
        BuiltinParameters {
            left: ChannelParameters {
                fader_db: 24.5,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].fader.left_db",
    ));
    rows.push(direct(
        "matrix-coefficient",
        BuiltinParameters {
            matrix: Matrix2x2 {
                ll: 1.1,
                ..Matrix2x2::IDENTITY
            },
            ..BuiltinParameters::default()
        },
        "$.tracks[id=vocal].matrix_or_pan.ll",
    ));
    let mut left = [0.0_f32];
    let mut right = [0.0_f32, 0.0];
    let Err(block) = DualMonoBlock::new(&mut left, &mut right, 0) else {
        panic!("mismatched lanes accepted");
    };
    rows.push(format!(
        "{{\"case\":\"block-length\",\"code\":\"builtin.block.length\",\"path\":\"$.render.block\",\"error\":\"{block:?}\"}}"
    ));
    let mut left = [0.0_f32];
    let mut right = [0.0_f32];
    let Err(block) = DualMonoBlock::new(&mut left, &mut right, u64::MAX) else {
        panic!("overflow accepted");
    };
    rows.push(format!(
        "{{\"case\":\"block-overflow\",\"code\":\"builtin.block.sample_time_overflow\",\"path\":\"$.render.first_sample\",\"error\":\"{block:?}\"}}"
    ));
    let session = fixture_session();
    let config = meter_config(2, 1, 3);
    let duplicate = [
        MeterRequest {
            handle: MeterHandle(core::num::NonZeroU64::new(1).expect("constant")),
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config,
        },
        MeterRequest {
            handle: MeterHandle(core::num::NonZeroU64::new(1).expect("constant")),
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config,
        },
        MeterRequest {
            handle: MeterHandle(core::num::NonZeroU64::new(3).expect("constant")),
            track_id: "missing".to_owned(),
            tap: MeterTap::PostFader,
            config,
        },
    ];
    let Err(error) = prepare_session_builtins(&session, &duplicate, unlimited_builtin_caps())
    else {
        panic!("duplicate and unknown request accepted");
    };
    for diagnostic in error.0 {
        rows.push(format!(
            "{{\"case\":\"meter-request\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    let mut caps = unlimited_builtin_caps();
    caps.maximum_total_state_bytes = 1;
    let Err(error) = prepare_session_builtins(&session, &[], caps) else {
        panic!("resource cap accepted");
    };
    for diagnostic in error.0 {
        rows.push(format!(
            "{{\"case\":\"resource-cap\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    let prepared = prepare_session_builtins(&session, &[], unlimited_builtin_caps())
        .expect("valid prepared artifact");
    let mismatched = fixture_session_tracks(4);
    for diagnostic in prepared.validate_for_session(&mismatched).0 {
        rows.push(format!(
            "{{\"case\":\"seal-session-mismatch\",\"code\":\"{}\",\"path\":\"{}\"}}",
            diagnostic.code, diagnostic.path
        ));
    }
    rows.sort();
    rows.dedup();
    rows.join("\n") + "\n"
}

fn meter_config(period: u32, capacity: usize, reset_generation: u64) -> MeterConfig {
    MeterConfig {
        period_frames: core::num::NonZeroU32::new(period).expect("period"),
        peak_hold_frames: 1,
        peak_decay_db_per_second: 12.0,
        queue_capacity: core::num::NonZeroUsize::new(capacity).expect("capacity"),
        reset_generation,
    }
}

fn meters() -> String {
    let mut rows = Vec::new();
    let handle = MeterHandle(core::num::NonZeroU64::new(1).expect("handle"));

    // A partial interval intentionally produces no snapshot; retain the observed state as
    // an expected record so it cannot be mistaken for missing fixture coverage.
    let mut partial =
        MeterAccumulator::prepare(handle, meter_config(4, 4, 3), 48_000).expect("partial meter");
    partial
        .accumulator
        .observe(&[1.0, 0.5], &[0.0, -1.0], 3)
        .expect("partial");
    rows.push("{\"case\":\"partial\",\"snapshot\":null,\"observed_frames\":2,\"period_frames\":4,\"start\":\"0000000000000003\"}".to_owned());

    let mut multiple =
        MeterAccumulator::prepare(handle, meter_config(2, 8, 3), 48_000).expect("multiple meter");
    multiple
        .accumulator
        .observe(&[1.0, 0.5, 0.25, 0.0], &[0.0, -1.0, 0.5, -0.25], 3)
        .expect("multiple");
    while let Ok(snapshot) = multiple.consumer.try_pop() {
        rows.push(meter_snapshot_json("multiple", snapshot));
    }

    // Interleaved draining forces the ring to wrap without a drop.
    let mut wrap =
        MeterAccumulator::prepare(handle, meter_config(2, 2, 3), 48_000).expect("wrap meter");
    wrap.accumulator
        .observe(&[1.0, 0.5, 0.25, 0.0], &[0.0, -1.0, 0.5, -0.25], 3)
        .expect("wrap first");
    rows.push(meter_snapshot_json(
        "wrap",
        wrap.consumer.try_pop().expect("first wrap snapshot"),
    ));
    wrap.accumulator
        .observe(&[0.75, 0.25, 0.5, 0.0], &[-0.5, 0.0, 0.25, -0.25], 7)
        .expect("wrap second");
    while let Ok(snapshot) = wrap.consumer.try_pop() {
        rows.push(meter_snapshot_json("wrap", snapshot));
    }

    let mut full =
        MeterAccumulator::prepare(handle, meter_config(2, 1, 3), 48_000).expect("full meter");
    full.accumulator
        .observe(&[1.0, 0.0, 0.5, 0.0], &[0.0, 1.0, 0.0, 0.5], 3)
        .expect("full");
    rows.push(meter_snapshot_json(
        "full",
        full.consumer.try_pop().expect("full first"),
    ));
    full.accumulator
        .observe(&[0.25, 0.0], &[0.0, -0.25], 7)
        .expect("after drop");
    while let Ok(snapshot) = full.consumer.try_pop() {
        rows.push(meter_snapshot_json("drop", snapshot));
    }

    let mut discontinuity = MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000)
        .expect("discontinuity meter");
    discontinuity
        .accumulator
        .observe(&[1.0, 0.0], &[0.0, 1.0], 3)
        .expect("first");
    discontinuity
        .accumulator
        .observe(&[0.5, 0.0], &[0.0, 0.5], 9)
        .expect("discontinuous");
    while let Ok(snapshot) = discontinuity.consumer.try_pop() {
        rows.push(meter_snapshot_json("discontinuity", snapshot));
    }

    let mut reset =
        MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000).expect("reset meter");
    reset
        .accumulator
        .observe(&[1.0, 0.0], &[0.0, 1.0], 3)
        .expect("reset first");
    rows.push(meter_snapshot_json(
        "drain",
        reset.consumer.try_pop().expect("drain snapshot"),
    ));
    reset
        .accumulator
        .reset(BuiltinResetKind::DiscontinuityKeepTargets);
    reset
        .accumulator
        .observe(&[0.5, 0.0], &[0.0, 0.5], 9)
        .expect("keep reset");
    rows.push(meter_snapshot_json(
        "reset-discontinuity",
        reset.consumer.try_pop().expect("keep reset snapshot"),
    ));
    reset.accumulator.reset(BuiltinResetKind::FullToPrepared);
    reset
        .accumulator
        .observe(&[0.25, 0.0], &[0.0, 0.25], 11)
        .expect("full reset");
    rows.push(meter_snapshot_json(
        "reset-full",
        reset.consumer.try_pop().expect("full reset snapshot"),
    ));

    let mut sanitized = MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000)
        .expect("sanitization meter");
    sanitized
        .accumulator
        .observe(&[f32::NAN, 0.5], &[f32::INFINITY, -0.5], 3)
        .expect("sanitize");
    rows.push(meter_snapshot_json(
        "sanitization",
        sanitized.consumer.try_pop().expect("sanitized snapshot"),
    ));

    let mut overflow =
        MeterAccumulator::prepare(handle, meter_config(2, 4, 3), 48_000).expect("overflow meter");
    let Err(error) = overflow.accumulator.observe(&[0.0], &[0.0], u64::MAX) else {
        panic!("meter overflow accepted");
    };
    rows.push(format!(
        "{{\"case\":\"overflow\",\"error\":\"{error:?}\",\"path\":\"$.meter.first_sample\"}}"
    ));
    rows.sort();
    rows.join("\n") + "\n"
}

fn meter_snapshot_json(case: &str, snapshot: miso_engine_builtins::MeterSnapshot) -> String {
    format!(
        "{{\"case\":\"{case}\",\"handle\":\"{:016x}\",\"reset_generation\":\"{:016x}\",\"sequence\":\"{:016x}\",\"start\":\"{:016x}\",\"end\":\"{:016x}\",\"frames\":{},\"left_peak\":\"{:08x}\",\"right_peak\":\"{:08x}\",\"left_held_peak\":\"{:08x}\",\"right_held_peak\":\"{:08x}\",\"left_energy\":\"{:016x}\",\"right_energy\":\"{:016x}\",\"left_rms\":\"{:016x}\",\"right_rms\":\"{:016x}\",\"clipped\":\"{:016x}\",\"sanitized\":\"{:016x}\",\"dropped\":\"{:016x}\",\"discontinuities\":\"{:016x}\"}}",
        snapshot.handle.0.get(),
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        snapshot.frames,
        snapshot.left.sample_peak.to_bits(),
        snapshot.right.sample_peak.to_bits(),
        snapshot.left.held_peak.to_bits(),
        snapshot.right.held_peak.to_bits(),
        snapshot.left.energy.to_bits(),
        snapshot.right.energy.to_bits(),
        snapshot.left.rms.to_bits(),
        snapshot.right.rms.to_bits(),
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_dropped_snapshots,
        snapshot.cumulative_discontinuities,
    )
}

fn resources() -> String {
    let mut output = String::new();
    for tracks in [1_usize, 4, 65_537] {
        for meters in [0_usize, 1, 7] {
            let capacity = if meters == 7 { 4 } else { 1 };
            let session = fixture_session_tracks(tracks);
            let config = MeterConfig {
                period_frames: core::num::NonZeroU32::new(session.quantum().0).expect("quantum"),
                peak_hold_frames: 0,
                peak_decay_db_per_second: 0.0,
                queue_capacity: core::num::NonZeroUsize::new(capacity).expect("capacity"),
                reset_generation: 0,
            };
            let requests: Vec<_> = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ][..meters]
                .iter()
                .copied()
                .enumerate()
                .map(|(index, tap)| MeterRequest {
                    handle: MeterHandle(
                        core::num::NonZeroU64::new(u64::try_from(index).expect("bounded") + 1)
                            .expect("nonzero"),
                    ),
                    track_id: if tracks == 1 { "vocal" } else { "track-0" }.to_owned(),
                    tap,
                    config,
                })
                .collect();
            let report = prepare_session_builtins(&session, &requests, unlimited_builtin_caps())
                .expect("resource fixture")
                .resource_report();
            writeln!(output, "{{\"tracks\":{tracks},\"meters\":{meters},\"queue_capacity\":{capacity},\"meter_items\":{},\"engine_owned_processor_payload_bytes\":{},\"engine_owned_meter_payload_bytes\":{},\"engine_owned_retained_payload_bytes\":{},\"maximum_single_allocation_bytes\":{},\"retained_allocation_count\":{}}}", report.meter_items, report.engine_owned_processor_payload_bytes, report.engine_owned_meter_payload_bytes, report.engine_owned_retained_payload_bytes, report.maximum_single_allocation_bytes, report.retained_allocation_count).expect("string");
        }
    }
    output
}

fn fixture_session() -> miso_engine_session::CompiledSession {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    model.tracks[0].simd1.effects.clear();
    model.tracks[0].dynamic.effects.clear();
    model.tracks[0].simd2.effects.clear();
    model.automation.clear();
    compile_session(
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
    .expect("compile session")
}

fn fixture_session_tracks(count: usize) -> miso_engine_session::CompiledSession {
    if count == 1 {
        return fixture_session();
    }
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("fixture session");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.tracks.clear();
    model.tracks.reserve(count);
    for index in 0..count {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("stable track ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("track ID"),
        tap: SendTap::PostMatrix,
    };
    model.automation.clear();
    compile_session(
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
    .expect("compile session")
}

fn unlimited_builtin_caps() -> BuiltinCompileCaps {
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

fn pcm_cases() -> Vec<(String, Vec<u8>)> {
    let filters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            lpf_hz: 1000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let gain = BuiltinParameters {
        left: ChannelParameters {
            polarity_invert: true,
            trim_db: -6.0,
            fader_db: -3.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            trim_db: 3.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let matrix = BuiltinParameters {
        matrix: Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        },
        ..BuiltinParameters::default()
    };
    let ramp = BuiltinParameters {
        smoothing_samples: 8,
        ..matrix
    };
    let mut fixtures = vec![
        (
            "identity-signed-zero".to_owned(),
            render_pcm(BuiltinParameters::default()),
        ),
        ("filters-asymmetric".to_owned(), render_pcm(filters)),
        ("polarity-gain".to_owned(), render_pcm(gain)),
        ("matrix-corner".to_owned(), render_pcm(matrix)),
        ("matrix-ramp".to_owned(), render_pcm(ramp)),
        (
            "mute".to_owned(),
            render_pcm(BuiltinParameters {
                left: ChannelParameters {
                    muted: true,
                    ..ChannelParameters::default()
                },
                right: ChannelParameters {
                    muted: true,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            }),
        ),
        ("matrix-retarget".to_owned(), render_matrix_retarget()),
        ("reset".to_owned(), render_reset()),
        ("lr-isolation".to_owned(), render_lr_isolation()),
        ("partition".to_owned(), render_partition()),
    ];
    for bits in 0_u8..16 {
        let matrix = Matrix2x2 {
            ll: f32::from(bits & 1),
            lr: f32::from((bits >> 1) & 1),
            rl: f32::from((bits >> 2) & 1),
            rr: f32::from((bits >> 3) & 1),
        };
        fixtures.push((
            format!("matrix-corner-{bits:02}"),
            render_pcm(BuiltinParameters {
                matrix,
                ..BuiltinParameters::default()
            }),
        ));
    }
    for updates in [0_u32, 1, 2, 127, 128, u32::MAX] {
        fixtures.push((
            format!("matrix-ramp-{updates}"),
            render_matrix_ramp(updates),
        ));
    }
    fixtures.sort_by(|left, right| left.0.cmp(&right.0));
    fixtures
}

fn render_pcm(parameters: BuiltinParameters) -> Vec<u8> {
    let mut chain = BuiltinChain::new(48_000, parameters).expect("fixture parameters");
    let mut left = [0.0_f32, -0.0, 0.25, -0.5, 1.0, -1.0, 0.125, -0.25];
    let mut right = [-0.0_f32, 0.0, -0.125, 0.5, -1.0, 1.0, -0.25, 0.25];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("fixture block"))
        .expect("fixture render");
    left.into_iter()
        .chain(right)
        .flat_map(f32::to_le_bytes)
        .collect()
}

fn pack_pcm(left: &[f32], right: &[f32]) -> Vec<u8> {
    left.iter()
        .chain(right)
        .copied()
        .flat_map(f32::to_le_bytes)
        .collect()
}

fn render_matrix_ramp(updates: u32) -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples: updates,
            ..BuiltinParameters::default()
        },
    )
    .expect("ramp fixture");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        })
        .expect("ramp target");
    let mut left = [1.0_f32; 128];
    let mut right = [-0.5_f32; 128];
    chain
        .process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("ramp block"))
        .expect("ramp render");
    pack_pcm(&left, &right)
}

fn render_matrix_retarget() -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            smoothing_samples: 8,
            ..BuiltinParameters::default()
        },
    )
    .expect("retarget fixture");
    chain
        .set_matrix_target(Matrix2x2 {
            ll: 0.0,
            lr: 1.0,
            rl: 1.0,
            rr: 0.0,
        })
        .expect("first target");
    let mut left = vec![1.0_f32; 4];
    let mut right = vec![-0.5_f32; 4];
    chain
        .process_matrix(DualMonoBlock::new(&mut left, &mut right, 0).expect("first block"))
        .expect("first render");
    chain
        .set_matrix_target(Matrix2x2::IDENTITY)
        .expect("second target");
    let mut tail_left = [1.0_f32; 8];
    let mut tail_right = [-0.5_f32; 8];
    chain
        .process_matrix(
            DualMonoBlock::new(&mut tail_left, &mut tail_right, 4).expect("second block"),
        )
        .expect("second render");
    left.extend(tail_left);
    right.extend(tail_right);
    pack_pcm(&left, &right)
}

fn render_reset() -> Vec<u8> {
    let parameters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let mut chain = BuiltinChain::new(48_000, parameters).expect("reset fixture");
    let mut left = vec![1.0_f32, 0.0, 0.0, 0.0];
    let mut right = vec![0.0_f32; 4];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("pre-reset"))
        .expect("pre-reset render");
    chain.reset(BuiltinResetKind::DiscontinuityKeepTargets);
    let mut post_left = [1.0_f32, 0.0, 0.0, 0.0];
    let mut post_right = [0.0_f32; 4];
    chain
        .process_dual_mono(
            DualMonoBlock::new(&mut post_left, &mut post_right, 4).expect("post-reset"),
        )
        .expect("post-reset render");
    left.extend(post_left);
    right.extend(post_right);
    pack_pcm(&left, &right)
}

fn render_lr_isolation() -> Vec<u8> {
    let mut chain = BuiltinChain::new(
        48_000,
        BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 100.0,
                ..ChannelParameters::default()
            },
            right: ChannelParameters {
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        },
    )
    .expect("isolation fixture");
    let mut left = [1.0_f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut right = [0.0_f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    chain
        .process_dual_mono(DualMonoBlock::new(&mut left, &mut right, 0).expect("isolation block"))
        .expect("isolation render");
    pack_pcm(&left, &right)
}

fn render_partition() -> Vec<u8> {
    let parameters = BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            lpf_hz: 1_000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    };
    let mut chain = BuiltinChain::new(48_000, parameters).expect("partition fixture");
    let mut left = [1.0_f32, 0.0, -0.5, 0.25, 0.0, 0.0, 0.75, -0.25];
    let mut right = [-0.5_f32, 0.0, 1.0, -0.25, 0.0, 0.5, 0.0, 0.25];
    for (offset, width) in [(0, 1), (1, 2), (3, 1), (4, 4)] {
        chain
            .process_dual_mono(
                DualMonoBlock::new(
                    &mut left[offset..offset + width],
                    &mut right[offset..offset + width],
                    offset as u64,
                )
                .expect("partition block"),
            )
            .expect("partition render");
    }
    pack_pcm(&left, &right)
}

fn manifest(files: &[(String, Vec<u8>)]) -> String {
    let mut output = String::from(MANIFEST_HEADER);
    for (path, bytes) in files {
        writeln!(output, "{path}\t{}\t{}", bytes.len(), sha256(bytes)).expect("string");
    }
    output
}

fn write_and_verify(root: &Path) -> Result<(), String> {
    let files = generated();
    for (path, bytes) in &files {
        let destination = root.join(path);
        let parent = destination
            .parent()
            .ok_or_else(|| "fixture path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(|error| format!("create fixture directory: {error}"))?;
        fs::write(destination, bytes).map_err(|error| format!("write fixture: {error}"))?;
    }
    fs::write(root.join("MANIFEST.tsv"), manifest(&files))
        .map_err(|error| format!("write manifest: {error}"))?;
    verify_generated_scratch(root, &files)?;
    check_read_only_fixture_root_v1(root)
}

fn verify_generated_scratch(root: &Path, expected: &[(String, Vec<u8>)]) -> Result<(), String> {
    if fs::read(root.join("MANIFEST.tsv")).map_err(|error| format!("read manifest: {error}"))?
        != manifest(expected).as_bytes()
    {
        return Err("builtins fixture manifest mismatch".to_owned());
    }
    for (path, bytes) in expected {
        if fs::read(root.join(path)).map_err(|error| format!("read {path}: {error}"))? != *bytes {
            return Err(format!("builtins fixture content mismatch: {path}"));
        }
    }
    let mut actual = list_files(root)?;
    actual.sort();
    let expected_paths: Vec<_> = expected.iter().map(|(path, _)| path.clone()).collect();
    if actual != expected_paths {
        return Err("builtins fixture missing or unlisted file".to_owned());
    }
    Ok(())
}

fn check_fixture_root_v1(root: &Path) -> Result<(), String> {
    let manifest = parse_manifest_v1(root)?;
    verify_manifest_bytes_v1(root, &manifest)?;
    verify_path_class_coverage_v1(&manifest)?;
    let cases = verify_cases_v1(root)?;
    let csv_response_ids = verify_reference_oracle_v1(root)?;
    if cases.response_ids != csv_response_ids {
        let missing: Vec<_> = csv_response_ids
            .difference(&cases.response_ids)
            .take(1)
            .collect();
        let unexpected: Vec<_> = cases
            .response_ids
            .difference(&csv_response_ids)
            .take(1)
            .collect();
        return Err(format!(
            "cases.toml and reference/filter-response.csv response IDs differ; missing={missing:?} unexpected={unexpected:?}"
        ));
    }
    verify_metadata_v1(root)?;
    verify_functional_fixture_completeness_v1(root, &manifest, &cases.functional_cases)?;
    verify_jsonl_payloads_v1(root)?;
    verify_benchmark_inputs_v1(root, &manifest)?;
    Ok(())
}

/// Runs the supplied-root checker while proving the read-only path did not mutate any byte.
fn check_read_only_fixture_root_v1(root: &Path) -> Result<(), String> {
    let before = fixture_tree_hash_v1(root)?;
    check_fixture_root_v1(root)?;
    let after = fixture_tree_hash_v1(root)?;
    if before != after {
        return Err("--check mutated the fixture tree".to_owned());
    }
    Ok(())
}

fn parse_manifest_v1(root: &Path) -> Result<FixtureManifestV1, String> {
    let bytes = read_regular_file(&root.join("MANIFEST.tsv"), "manifest")?;
    let text = std::str::from_utf8(&bytes).map_err(|_| "manifest is not UTF-8".to_owned())?;
    let mut lines = text.split_inclusive('\n');
    if lines.next() != Some(MANIFEST_HEADER) {
        return Err("manifest has an invalid header".to_owned());
    }

    let mut previous = None;
    let mut entries = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let line = line
            .strip_suffix('\n')
            .ok_or_else(|| format!("manifest line {} is not LF terminated", line_number + 2))?;
        let mut fields = line.split('\t');
        let path = fields.next().unwrap_or_default();
        let length = fields.next().unwrap_or_default();
        let sha256 = fields.next().unwrap_or_default();
        if path.is_empty() || fields.next().is_some() {
            return Err(format!("manifest line {} is malformed", line_number + 2));
        }
        if previous.is_some_and(|previous: &str| previous >= path) {
            return Err(format!("manifest path is not strictly sorted: {path}"));
        }
        let length = length
            .parse::<u64>()
            .map_err(|_| format!("manifest length is not an unsigned integer: {path}"))?;
        if !is_lower_sha256(sha256) {
            return Err(format!("manifest sha256 is not lowercase hex: {path}"));
        }
        entries.push(FixtureManifestEntryV1 {
            path: path.to_owned(),
            length,
            sha256: sha256.to_owned(),
            class: classify_fixture_path_v1(path)?,
        });
        previous = Some(path);
    }
    if entries.is_empty() {
        return Err("manifest has no payload entries".to_owned());
    }
    Ok(FixtureManifestV1 { entries })
}

fn is_lower_sha256(value: &str) -> bool {
    value.len() == 64
        && value.bytes().all(|byte| {
            byte.is_ascii_digit() || (byte.is_ascii_lowercase() && byte.is_ascii_hexdigit())
        })
}

fn classify_fixture_path_v1(path: &str) -> Result<FixturePathClassV1, String> {
    if !is_safe_fixture_relative_path(path) {
        return Err(format!("manifest path is unsafe: {path}"));
    }
    match path {
        "cases.toml" => Ok(FixturePathClassV1::Cases),
        "reference/filter-response.csv" => Ok(FixturePathClassV1::Reference),
        "diagnostics.jsonl" => Ok(FixturePathClassV1::Diagnostics),
        "resources.jsonl" => Ok(FixturePathClassV1::Resources),
        "metadata.toml" => Ok(FixturePathClassV1::Metadata),
        "meters/graph-taps.jsonl" | "meters/window-and-drop.jsonl" => Ok(FixturePathClassV1::Meter),
        _ if benchmark_path_v1(path).is_some() => Ok(FixturePathClassV1::Benchmark),
        _ if path.starts_with("pcm/")
            && path.ends_with(".f32le")
            && is_fixture_case_id(&path[4..path.len() - 6]) =>
        {
            Ok(FixturePathClassV1::Pcm)
        }
        _ => Err(format!("manifest path has no V1 fixture class: {path}")),
    }
}

fn is_safe_fixture_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && path.split('/').all(|component| {
            !component.is_empty()
                && component != "."
                && component != ".."
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        })
}

fn is_fixture_case_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn verify_manifest_bytes_v1(root: &Path, manifest: &FixtureManifestV1) -> Result<(), String> {
    let expected: BTreeSet<_> = manifest
        .entries
        .iter()
        .map(|entry| entry.path.as_str())
        .collect();
    let actual = list_files(root)?;
    let actual: BTreeSet<_> = actual.iter().map(String::as_str).collect();
    if actual != expected {
        return Err("fixture tree has missing or unlisted payload files".to_owned());
    }
    for entry in &manifest.entries {
        let bytes = read_regular_file(&root.join(&entry.path), &entry.path)?;
        if u64::try_from(bytes.len()).ok() != Some(entry.length) {
            return Err(format!("fixture byte length mismatch: {}", entry.path));
        }
        if sha256(&bytes) != entry.sha256 {
            return Err(format!("fixture sha256 mismatch: {}", entry.path));
        }
    }
    Ok(())
}

fn read_regular_file(path: &Path, label: &str) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| format!("read {label}: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("fixture is not a regular file: {label}"));
    }
    fs::read(path).map_err(|error| format!("read {label}: {error}"))
}

fn verify_path_class_coverage_v1(manifest: &FixtureManifestV1) -> Result<(), String> {
    let mut counts = BTreeMap::new();
    for entry in &manifest.entries {
        *counts.entry(entry.class.clone()).or_insert(0_usize) += 1;
    }
    for (class, expected) in [
        (FixturePathClassV1::Cases, 1),
        (FixturePathClassV1::Reference, 1),
        (FixturePathClassV1::Diagnostics, 1),
        (FixturePathClassV1::Resources, 1),
        (FixturePathClassV1::Metadata, 1),
        (FixturePathClassV1::Meter, 2),
        (FixturePathClassV1::Pcm, PCM_PAYLOAD_COUNT_V1),
        (FixturePathClassV1::Benchmark, 10),
    ] {
        if counts.get(&class) != Some(&expected) {
            return Err(format!(
                "fixture coverage requires {expected} {class:?} payloads"
            ));
        }
    }
    Ok(())
}

fn verify_cases_v1(root: &Path) -> Result<VerifiedCasesV1, String> {
    let bytes = read_regular_file(&root.join("cases.toml"), "cases.toml")?;
    let text = std::str::from_utf8(&bytes).map_err(|_| "cases.toml is not UTF-8".to_owned())?;
    let mut blocks = text.split("[[case]]\n");
    if blocks.next() != Some("fixture_schema = 1\n\n") {
        return Err("cases.toml does not have the canonical V1 header".to_owned());
    }

    let mut case_count = 0_usize;
    let mut previous = None::<String>;
    let mut response_ids = BTreeSet::new();
    let mut functional_cases = BTreeMap::new();
    for block in blocks {
        if block.is_empty() {
            continue;
        }
        case_count += 1;
        let id = quoted_case_field(block, "id")
            .ok_or_else(|| "cases.toml case is missing canonical id".to_owned())?;
        if previous.as_deref().is_some_and(|previous| previous >= id) {
            return Err(format!("cases.toml IDs are not strictly sorted: {id}"));
        }
        previous = Some(id.to_owned());
        if quoted_case_field(block, "category") == Some("filter_response") {
            if !response_ids.insert(id.to_owned()) {
                return Err(format!("cases.toml duplicate response ID: {id}"));
            }
        } else {
            let functional = parse_functional_case_v1(block)?;
            if functional.id != id {
                return Err(format!("functional case ID parse mismatch: {id}"));
            }
            if functional_cases.insert(id.to_owned(), functional).is_some() {
                return Err(format!("cases.toml duplicate functional ID: {id}"));
            }
        }
    }
    if case_count != CASE_COUNT_V1 || response_ids.len() != RESPONSE_CASE_COUNT_V1 {
        return Err(format!(
            "cases.toml coverage count differs: cases={case_count} responses={}",
            response_ids.len()
        ));
    }
    verify_functional_cases_v1(&functional_cases)?;
    Ok(VerifiedCasesV1 {
        response_ids,
        functional_cases,
    })
}

fn quoted_case_field<'a>(block: &'a str, key: &str) -> Option<&'a str> {
    block.lines().find_map(|line| {
        let value = line.strip_prefix(key)?.strip_prefix(" = \"")?;
        value.strip_suffix('"')
    })
}

fn parse_functional_case_v1(block: &str) -> Result<FunctionalCaseV1, String> {
    let lines: Vec<_> = block.lines().filter(|line| !line.is_empty()).collect();
    let [id, category, rate_hz, quantum_frames, detail] = lines.as_slice() else {
        return Err("functional case does not have exactly five canonical fields".to_owned());
    };
    let id = parse_case_string_field_v1(id, "id")?;
    let category = parse_case_string_field_v1(category, "category")?;
    let rate_hz = parse_case_u32_field_v1(rate_hz, "rate_hz")?;
    let quantum_frames = parse_case_u32_field_v1(quantum_frames, "quantum_frames")?;
    let detail = parse_case_string_field_v1(detail, "detail")?;
    Ok(FunctionalCaseV1 {
        id,
        category,
        rate_hz,
        quantum_frames,
        detail,
    })
}

fn parse_case_string_field_v1(line: &str, key: &str) -> Result<String, String> {
    let value = line
        .strip_prefix(key)
        .and_then(|line| line.strip_prefix(" = \""))
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| format!("functional case has invalid {key} field"))?;
    if value.contains('"') || value.contains('\\') {
        return Err(format!("functional case has noncanonical {key} string"));
    }
    Ok(value.to_owned())
}

fn parse_case_u32_field_v1(line: &str, key: &str) -> Result<u32, String> {
    let value = line
        .strip_prefix(key)
        .and_then(|line| line.strip_prefix(" = "))
        .ok_or_else(|| format!("functional case has invalid {key} field"))?;
    let parsed = value
        .parse::<u32>()
        .map_err(|_| format!("functional case {key} is not a canonical u32"))?;
    if parsed.to_string() != value {
        return Err(format!("functional case {key} is not canonical"));
    }
    Ok(parsed)
}

fn verify_functional_cases_v1(cases: &BTreeMap<String, FunctionalCaseV1>) -> Result<(), String> {
    if cases.len() != FUNCTIONAL_CASES_V1.len() {
        return Err(format!(
            "cases.toml functional coverage differs: cases={} expected={}",
            cases.len(),
            FUNCTIONAL_CASES_V1.len()
        ));
    }
    for (id, category, detail) in FUNCTIONAL_CASES_V1 {
        let case = cases
            .get(id)
            .ok_or_else(|| format!("cases.toml missing functional case: {id}"))?;
        if case.category != category
            || case.rate_hz != 48_000
            || case.quantum_frames != 128
            || case.detail != detail
        {
            return Err(format!("cases.toml functional tuple differs: {id}"));
        }
    }
    Ok(())
}

fn verify_metadata_v1(root: &Path) -> Result<(), String> {
    let bytes = read_regular_file(&root.join("metadata.toml"), "metadata.toml")?;
    if bytes != METADATA_V1.as_bytes() {
        return Err(
            "metadata.toml does not contain the exact canonical seven-key V1 metadata".to_owned(),
        );
    }
    Ok(())
}

fn verify_functional_fixture_completeness_v1(
    root: &Path,
    manifest: &FixtureManifestV1,
    cases: &BTreeMap<String, FunctionalCaseV1>,
) -> Result<(), String> {
    let ownership = functional_payload_ownership_v1();
    let case_ids: BTreeSet<_> = cases.keys().map(String::as_str).collect();
    let ownership_ids: BTreeSet<_> = ownership.keys().copied().collect();
    if case_ids != ownership_ids {
        return Err("functional case payload ownership is incomplete".to_owned());
    }

    let expected_payloads: BTreeSet<_> = ownership
        .values()
        .flat_map(|paths| paths.iter().map(String::as_str))
        .collect();
    let actual_payloads: BTreeSet<_> = manifest
        .entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.class,
                FixturePathClassV1::Pcm
                    | FixturePathClassV1::Meter
                    | FixturePathClassV1::Diagnostics
                    | FixturePathClassV1::Resources
            )
        })
        .map(|entry| entry.path.as_str())
        .collect();
    if expected_payloads != actual_payloads {
        let missing: Vec<_> = expected_payloads
            .difference(&actual_payloads)
            .take(1)
            .collect();
        let orphaned: Vec<_> = actual_payloads
            .difference(&expected_payloads)
            .take(1)
            .collect();
        return Err(format!(
            "functional payload paths differ; missing={missing:?} orphaned={orphaned:?}"
        ));
    }

    verify_pcm_semantics_v1(root)?;
    verify_graph_tap_output_relation_v1(root)?;
    Ok(())
}

fn functional_payload_ownership_v1() -> BTreeMap<&'static str, BTreeSet<String>> {
    let mut ownership = BTreeMap::new();
    for id in [
        "identity-signed-zero",
        "polarity-gain",
        "mute",
        "filters-asymmetric",
        "matrix-retarget",
        "reset",
        "lr-isolation",
        "partition",
    ] {
        ownership.insert(id, BTreeSet::from([format!("pcm/{id}.f32le")]));
    }
    let mut corners = BTreeSet::from(["pcm/matrix-corner.f32le".to_owned()]);
    for bits in 0_u8..16 {
        corners.insert(format!("pcm/matrix-corner-{bits:02}.f32le"));
    }
    ownership.insert("matrix-corners", corners);

    let mut ramps = BTreeSet::from(["pcm/matrix-ramp.f32le".to_owned()]);
    for updates in [0_u32, 1, 2, 127, 128, u32::MAX] {
        ramps.insert(format!("pcm/matrix-ramp-{updates}.f32le"));
    }
    ownership.insert("matrix-ramp", ramps);
    ownership.insert(
        "graph-taps",
        BTreeSet::from([
            "pcm/graph-taps.f32le".to_owned(),
            "meters/graph-taps.jsonl".to_owned(),
        ]),
    );
    for id in FUNCTIONAL_CASES_V1
        .iter()
        .filter_map(|(id, category, _)| (*category == "meter").then_some(*id))
    {
        ownership.insert(
            id,
            BTreeSet::from(["meters/window-and-drop.jsonl".to_owned()]),
        );
    }
    ownership.insert(
        "diagnostics",
        BTreeSet::from(["diagnostics.jsonl".to_owned()]),
    );
    ownership.insert(
        "resource-grid",
        BTreeSet::from(["resources.jsonl".to_owned()]),
    );
    ownership
}

fn verify_pcm_semantics_v1(root: &Path) -> Result<(), String> {
    verify_pcm_words_v1(
        root,
        "pcm/identity-signed-zero.f32le",
        &pcm_words_v1(&PCM_INPUT_LEFT_V1, &PCM_INPUT_RIGHT_V1),
    )?;

    let polarity_gain_left: Vec<_> = PCM_INPUT_LEFT_V1
        .iter()
        .copied()
        .map(|sample| {
            let signed = -sample;
            let trimmed = signed * independent_db_gain_v1(-6.0);
            trimmed * independent_db_gain_v1(-3.0)
        })
        .collect();
    let polarity_gain_right: Vec<_> = PCM_INPUT_RIGHT_V1
        .iter()
        .copied()
        .map(|sample| sample * independent_db_gain_v1(3.0))
        .collect();
    verify_pcm_words_v1(
        root,
        "pcm/polarity-gain.f32le",
        &pcm_words_v1(&polarity_gain_left, &polarity_gain_right),
    )?;
    verify_pcm_words_v1(root, "pcm/mute.f32le", &[0.0_f32.to_bits(); 16])?;

    verify_filter_pcm_semantics_v1(root)?;
    verify_matrix_pcm_semantics_v1(root)?;
    Ok(())
}

fn independent_db_gain_v1(db: f32) -> f32 {
    10.0_f64.powf(f64::from(db) / 20.0) as f32
}

fn verify_filter_pcm_semantics_v1(root: &Path) -> Result<(), String> {
    let filters_left =
        retained_tpt_outputs_v1(&PCM_INPUT_LEFT_V1, 100.0, ReferenceTptOutput::HighPass)?;
    let filters_right =
        retained_tpt_outputs_v1(&PCM_INPUT_RIGHT_V1, 1_000.0, ReferenceTptOutput::LowPass)?;
    verify_pcm_words_v1(
        root,
        "pcm/filters-asymmetric.f32le",
        &pcm_words_v1(&filters_left, &filters_right),
    )?;

    let reset_input = [1.0, 0.0, 0.0, 0.0];
    let mut reset_left =
        retained_tpt_outputs_v1(&reset_input, 100.0, ReferenceTptOutput::HighPass)?;
    reset_left.extend(retained_tpt_outputs_v1(
        &reset_input,
        100.0,
        ReferenceTptOutput::HighPass,
    )?);
    verify_pcm_words_v1(
        root,
        "pcm/reset.f32le",
        &pcm_words_v1(&reset_left, &[0.0; 8]),
    )?;

    let isolation_left = retained_tpt_outputs_v1(
        &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        100.0,
        ReferenceTptOutput::HighPass,
    )?;
    let isolation_right = retained_tpt_outputs_v1(
        &[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        1_000.0,
        ReferenceTptOutput::LowPass,
    )?;
    verify_pcm_words_v1(
        root,
        "pcm/lr-isolation.f32le",
        &pcm_words_v1(&isolation_left, &isolation_right),
    )?;

    let partition_left = retained_tpt_outputs_v1(
        &[1.0, 0.0, -0.5, 0.25, 0.0, 0.0, 0.75, -0.25],
        100.0,
        ReferenceTptOutput::HighPass,
    )?;
    let partition_right = retained_tpt_outputs_v1(
        &[-0.5, 0.0, 1.0, -0.25, 0.0, 0.5, 0.0, 0.25],
        1_000.0,
        ReferenceTptOutput::LowPass,
    )?;
    verify_pcm_words_v1(
        root,
        "pcm/partition.f32le",
        &pcm_words_v1(&partition_left, &partition_right),
    )
}

fn retained_tpt_outputs_v1(
    input: &[f32],
    cutoff_hz: f32,
    output: ReferenceTptOutput,
) -> Result<Vec<f32>, String> {
    let mut reference = ReferenceRetainedTptF32::conditioned_butterworth(48_000, cutoff_hz, output)
        .ok_or_else(|| {
            "independent retained-f32 TPT design rejected a frozen PCM coordinate".to_owned()
        })?;
    Ok(input
        .iter()
        .copied()
        .map(|sample| f32::from_bits(reference.process(sample).output_bits))
        .collect())
}

fn verify_matrix_pcm_semantics_v1(root: &Path) -> Result<(), String> {
    let swap = [0.0, 1.0, 1.0, 0.0];
    verify_pcm_words_v1(
        root,
        "pcm/matrix-corner.f32le",
        &matrix_pcm_words_v1(swap, &PCM_INPUT_LEFT_V1, &PCM_INPUT_RIGHT_V1),
    )?;
    for bits in 0_u8..16 {
        let matrix = [
            f32::from(bits & 1),
            f32::from((bits >> 1) & 1),
            f32::from((bits >> 2) & 1),
            f32::from((bits >> 3) & 1),
        ];
        verify_pcm_words_v1(
            root,
            &format!("pcm/matrix-corner-{bits:02}.f32le"),
            &matrix_pcm_words_v1(matrix, &PCM_INPUT_LEFT_V1, &PCM_INPUT_RIGHT_V1),
        )?;
    }
    for updates in [0_u32, 1, 2, 127, 128, u32::MAX] {
        let (left, right) = matrix_ramp_outputs_v1(updates);
        verify_pcm_words_v1(
            root,
            &format!("pcm/matrix-ramp-{updates}.f32le"),
            &pcm_words_v1(&left, &right),
        )?;
    }

    let (left, right) = matrix_retarget_outputs_v1();
    verify_pcm_words_v1(
        root,
        "pcm/matrix-retarget.f32le",
        &pcm_words_v1(&left, &right),
    )
}

fn matrix_pcm_words_v1(matrix: [f32; 4], left: &[f32], right: &[f32]) -> Vec<u32> {
    let (left, right) = matrix_outputs_v1(matrix, left, right);
    pcm_words_v1(&left, &right)
}

fn matrix_outputs_v1(matrix: [f32; 4], left: &[f32], right: &[f32]) -> (Vec<f32>, Vec<f32>) {
    if matrix == [1.0, 0.0, 0.0, 1.0] {
        return (left.to_vec(), right.to_vec());
    }
    let mut output_left = Vec::with_capacity(left.len());
    let mut output_right = Vec::with_capacity(right.len());
    for (&left, &right) in left.iter().zip(right) {
        output_left.push(matrix[0] * left + matrix[1] * right);
        output_right.push(matrix[2] * left + matrix[3] * right);
    }
    (output_left, output_right)
}

fn matrix_ramp_outputs_v1(updates: u32) -> (Vec<f32>, Vec<f32>) {
    let target = [0.0, 1.0, 1.0, 0.0];
    let mut current = [1.0, 0.0, 0.0, 1.0];
    let mut remaining = updates;
    if remaining == 0 {
        current = target;
    }
    let mut left = Vec::with_capacity(128);
    let mut right = Vec::with_capacity(128);
    for _ in 0..128 {
        advance_matrix_v1(&mut current, target, &mut remaining);
        let (output_left, output_right) = matrix_outputs_v1(current, &[1.0], &[-0.5]);
        left.push(output_left[0]);
        right.push(output_right[0]);
    }
    (left, right)
}

fn matrix_retarget_outputs_v1() -> (Vec<f32>, Vec<f32>) {
    let swap = [0.0, 1.0, 1.0, 0.0];
    let identity = [1.0, 0.0, 0.0, 1.0];
    let mut current = identity;
    let mut remaining = 8;
    let mut left = Vec::with_capacity(12);
    let mut right = Vec::with_capacity(12);
    for _ in 0..4 {
        advance_matrix_v1(&mut current, swap, &mut remaining);
        let (output_left, output_right) = matrix_outputs_v1(current, &[1.0], &[-0.5]);
        left.push(output_left[0]);
        right.push(output_right[0]);
    }
    remaining = 8;
    for _ in 0..8 {
        advance_matrix_v1(&mut current, identity, &mut remaining);
        let (output_left, output_right) = matrix_outputs_v1(current, &[1.0], &[-0.5]);
        left.push(output_left[0]);
        right.push(output_right[0]);
    }
    (left, right)
}

fn advance_matrix_v1(current: &mut [f32; 4], target: [f32; 4], remaining: &mut u32) {
    if *remaining == 0 {
        return;
    }
    let divisor = *remaining as f32;
    for (current, target) in current.iter_mut().zip(target) {
        *current += (target - *current) / divisor;
    }
    *remaining -= 1;
    if *remaining == 0 {
        *current = target;
    }
}

fn pcm_words_v1(left: &[f32], right: &[f32]) -> Vec<u32> {
    left.iter()
        .chain(right)
        .copied()
        .map(f32::to_bits)
        .collect()
}

fn verify_pcm_words_v1(root: &Path, path: &str, expected: &[u32]) -> Result<(), String> {
    let actual = read_pcm_words_v1(root, path)?;
    if actual == expected {
        return Ok(());
    }
    let index = actual
        .iter()
        .zip(expected)
        .position(|(actual, expected)| actual != expected)
        .unwrap_or_else(|| actual.len().min(expected.len()));
    Err(format!(
        "PCM semantic mismatch: {path}: word={index} actual={:?} expected={:?}",
        actual.get(index).map(|bits| format!("{bits:08x}")),
        expected.get(index).map(|bits| format!("{bits:08x}")),
    ))
}

fn read_pcm_words_v1(root: &Path, path: &str) -> Result<Vec<u32>, String> {
    let bytes = read_regular_file(&root.join(path), path)?;
    let chunks = bytes.chunks_exact(4);
    if !chunks.remainder().is_empty() || bytes.len() % 8 != 0 {
        return Err(format!(
            "PCM is not a planar dual-mono f32le payload: {path}"
        ));
    }
    Ok(chunks
        .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("fixed f32 word")))
        .collect())
}

fn verify_graph_tap_output_relation_v1(root: &Path) -> Result<(), String> {
    let bytes = read_regular_file(
        &root.join("meters/graph-taps.jsonl"),
        "meters/graph-taps.jsonl",
    )?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| "meters/graph-taps.jsonl is not UTF-8".to_owned())?;
    let records: Vec<_> = text
        .split_inclusive('\n')
        .map(|line| {
            line.strip_suffix('\n')
                .ok_or_else(|| "graph tap record is not LF terminated".to_owned())
        })
        .collect::<Result<_, _>>()?;
    let expected_taps = BTreeSet::from([
        "Input",
        "PostInputBuiltins",
        "PostSimd1",
        "PostDynamic",
        "PostSimd2PreFader",
        "PostFader",
        "PostMatrix",
    ]);
    let actual_taps: BTreeSet<_> = records
        .iter()
        .map(|record| json_string_field_v1(record, "tap"))
        .collect::<Result<_, _>>()?;
    if actual_taps != expected_taps || records.len() != expected_taps.len() {
        return Err(
            "graph-taps does not contain the exact seven distinct stage records".to_owned(),
        );
    }
    let post_matrix = records
        .iter()
        .find(|record| json_string_field_v1(record, "tap") == Ok("PostMatrix"))
        .ok_or_else(|| "graph-taps has no PostMatrix record".to_owned())?;
    if json_string_field_v1(post_matrix, "case")? != "graph-taps"
        || json_u64_field_v1(post_matrix, "frames")? != 128
    {
        return Err("graph-taps PostMatrix declaration is invalid".to_owned());
    }
    let samples = read_pcm_words_v1(root, "pcm/graph-taps.f32le")?;
    if samples.len() != 256 {
        return Err("graph-taps PCM frame count differs".to_owned());
    }
    let (left, right) = samples.split_at(128);
    verify_graph_lane_summary_v1(post_matrix, "left", left)?;
    verify_graph_lane_summary_v1(post_matrix, "right", right)
}

fn verify_graph_lane_summary_v1(record: &str, lane: &str, words: &[u32]) -> Result<(), String> {
    let peak = words
        .iter()
        .fold(0.0_f32, |peak, bits| peak.max(f32::from_bits(*bits).abs()));
    let energy = words.iter().fold(0.0_f64, |energy, bits| {
        let sample = f64::from(f32::from_bits(*bits));
        energy + sample * sample
    });
    let peak_field = format!("{lane}_peak");
    let energy_field = format!("{lane}_energy");
    if json_u32_hex_field_v1(record, &peak_field)? != peak.to_bits()
        || json_u64_hex_field_v1(record, &energy_field)? != energy.to_bits()
    {
        return Err(format!(
            "graph-taps PostMatrix {lane} summary does not match output PCM"
        ));
    }
    Ok(())
}

fn json_string_field_v1<'a>(record: &'a str, key: &str) -> Result<&'a str, String> {
    let prefix = format!("\"{key}\":\"");
    let value = record
        .split_once(&prefix)
        .map(|(_, value)| value)
        .and_then(|value| value.split_once('"').map(|(value, _)| value))
        .ok_or_else(|| format!("JSONL record is missing string field: {key}"))?;
    Ok(value)
}

fn json_u64_field_v1(record: &str, key: &str) -> Result<u64, String> {
    let prefix = format!("\"{key}\":");
    let value = record
        .split_once(&prefix)
        .map(|(_, value)| value)
        .and_then(|value| value.split([',', '}']).next())
        .ok_or_else(|| format!("JSONL record is missing numeric field: {key}"))?;
    value
        .parse()
        .map_err(|_| format!("JSONL field is not a decimal u64: {key}"))
}

fn json_u32_hex_field_v1(record: &str, key: &str) -> Result<u32, String> {
    u32::from_str_radix(json_string_field_v1(record, key)?, 16)
        .map_err(|_| format!("JSONL field is not a u32 hex word: {key}"))
}

fn json_u64_hex_field_v1(record: &str, key: &str) -> Result<u64, String> {
    u64::from_str_radix(json_string_field_v1(record, key)?, 16)
        .map_err(|_| format!("JSONL field is not a u64 hex word: {key}"))
}

fn verify_reference_oracle_v1(root: &Path) -> Result<BTreeSet<String>, String> {
    let bytes = read_regular_file(
        &root.join("reference/filter-response.csv"),
        "reference/filter-response.csv",
    )?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| "reference/filter-response.csv is not UTF-8".to_owned())?;
    const HEADER: &str = "case,rate_hz,section,cutoff_hz,probe_hz,quantum_frames,rbj_magnitude_db,cast_state_magnitude_db,impulse_dft_magnitude_db,sustained_fundamental_db,sustained_residual_db,sustained_total_db,tail_energy,recovery_count\n";
    let mut lines = text.split_inclusive('\n');
    if lines.next() != Some(HEADER) {
        return Err("reference/filter-response.csv has an invalid V1 header".to_owned());
    }
    let mut rows = Vec::new();
    let mut ids = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let line = line.strip_suffix('\n').ok_or_else(|| {
            format!(
                "reference/filter-response.csv row {} is not LF terminated",
                index + 2
            )
        })?;
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 14 || fields[0].is_empty() {
            return Err(format!(
                "reference/filter-response.csv row {} is malformed",
                index + 2
            ));
        }
        let row = parse_response_csv_row_v1(fields, index + 2)?;
        if !ids.insert(row.id.clone()) {
            return Err(format!(
                "reference/filter-response.csv has duplicate case: {}",
                row.id
            ));
        }
        rows.push(row);
    }
    if rows.len() != RESPONSE_ROW_COUNT_V1 {
        return Err(format!(
            "reference/filter-response.csv coverage count differs: rows={} expected={RESPONSE_ROW_COUNT_V1}",
            rows.len()
        ));
    }
    if ids != expected_response_ids_v1() {
        let expected = expected_response_ids_v1();
        let missing: Vec<_> = expected.difference(&ids).take(1).collect();
        let unexpected: Vec<_> = ids.difference(&expected).take(1).collect();
        return Err(format!(
            "reference/filter-response.csv IDs differ from the frozen grid; missing={missing:?} unexpected={unexpected:?}"
        ));
    }
    verify_response_grid_v1(&rows)?;
    verify_response_oracle_tolerances_v1(&rows)?;
    Ok(ids)
}

fn parse_response_csv_row_v1(
    fields: Vec<&str>,
    line_number: usize,
) -> Result<ResponseCsvRowV1, String> {
    let rate_hz = fields[1].parse::<u32>().map_err(|_| {
        format!("reference/filter-response.csv rate is not a u32 at row {line_number}")
    })?;
    let section = match fields[2] {
        "high_pass" => ResponseSectionV1::HighPass,
        "low_pass" => ResponseSectionV1::LowPass,
        "cascade" => ResponseSectionV1::Cascade,
        _ => {
            return Err(format!(
                "reference/filter-response.csv section is invalid at row {line_number}"
            ));
        }
    };
    let cutoff_hz = parse_response_f64_v1(fields[3], "cutoff_hz", line_number)?;
    let probe_hz = parse_response_f64_v1(fields[4], "probe_hz", line_number)?;
    let quantum_frames = fields[5].parse::<u32>().map_err(|_| {
        format!("reference/filter-response.csv quantum is not a u32 at row {line_number}")
    })?;
    Ok(ResponseCsvRowV1 {
        id: fields[0].to_owned(),
        rate_hz,
        section,
        cutoff_hz,
        probe_hz,
        quantum_frames,
        rbj_magnitude_db: parse_response_f64_v1(fields[6], "rbj_magnitude_db", line_number)?,
        cast_state_magnitude_db: parse_response_f64_v1(
            fields[7],
            "cast_state_magnitude_db",
            line_number,
        )?,
        impulse_dft_magnitude_db: parse_response_f64_v1(
            fields[8],
            "impulse_dft_magnitude_db",
            line_number,
        )?,
        sustained_fundamental_db: parse_response_f64_v1(
            fields[9],
            "sustained_fundamental_db",
            line_number,
        )?,
        sustained_residual_db: parse_response_f64_v1(
            fields[10],
            "sustained_residual_db",
            line_number,
        )?,
        sustained_total_db: parse_response_f64_v1(fields[11], "sustained_total_db", line_number)?,
        tail_energy: parse_response_f64_v1(fields[12], "tail_energy", line_number)?,
        recovery_count: fields[13].parse::<u64>().map_err(|_| {
            format!(
                "reference/filter-response.csv recovery_count is not a u64 at row {line_number}"
            )
        })?,
    })
}

fn parse_response_f64_v1(value: &str, field: &str, line_number: usize) -> Result<f64, String> {
    let value = value.parse::<f64>().map_err(|_| {
        format!("reference/filter-response.csv {field} is not an f64 at row {line_number}")
    })?;
    if !value.is_finite() {
        return Err(format!(
            "reference/filter-response.csv {field} is not finite at row {line_number}"
        ));
    }
    Ok(value)
}

fn verify_response_grid_v1(rows: &[ResponseCsvRowV1]) -> Result<(), String> {
    let actual: BTreeSet<_> = rows.iter().map(response_coordinate_v1).collect();
    if actual.len() != rows.len() {
        return Err("reference/filter-response.csv has duplicate response coordinates".to_owned());
    }
    let expected = expected_response_coordinates_v1();
    if actual != expected {
        let missing: Vec<_> = expected.difference(&actual).take(1).collect();
        let unexpected: Vec<_> = actual.difference(&expected).take(1).collect();
        return Err(format!(
            "reference/filter-response.csv frozen grid differs; missing={missing:?} unexpected={unexpected:?}"
        ));
    }
    Ok(())
}

fn response_coordinate_v1(row: &ResponseCsvRowV1) -> ResponseCoordinateV1 {
    ResponseCoordinateV1 {
        rate_hz: row.rate_hz,
        section: row.section,
        cutoff_bits: row.cutoff_hz.to_bits(),
        probe_bits: row.probe_hz.to_bits(),
        quantum_frames: row.quantum_frames,
    }
}

fn expected_response_coordinates_v1() -> BTreeSet<ResponseCoordinateV1> {
    let mut expected = BTreeSet::new();
    for rate_hz in RATES {
        for quantum_frames in QUANTA {
            for cutoff_hz in response_cutoffs(rate_hz) {
                let probes = frozen_single_section_probes_v1(rate_hz, cutoff_hz);
                for section in [ResponseSectionV1::HighPass, ResponseSectionV1::LowPass] {
                    for probe_hz in &probes {
                        expected.insert(ResponseCoordinateV1 {
                            rate_hz,
                            section,
                            cutoff_bits: cutoff_hz.to_bits(),
                            probe_bits: probe_hz.to_bits(),
                            quantum_frames,
                        });
                    }
                }
            }
            for probe_hz in frozen_cascade_probes_v1(rate_hz) {
                expected.insert(ResponseCoordinateV1 {
                    rate_hz,
                    section: ResponseSectionV1::Cascade,
                    cutoff_bits: 100.0_f64.to_bits(),
                    probe_bits: probe_hz.to_bits(),
                    quantum_frames,
                });
            }
        }
    }
    expected
}

fn expected_response_ids_v1() -> BTreeSet<String> {
    let mut expected = BTreeSet::new();
    for rate_hz in RATES {
        for quantum_frames in QUANTA {
            for (cutoff_index, cutoff_hz) in response_cutoffs(rate_hz).into_iter().enumerate() {
                for (probe_index, _) in frozen_single_section_probes_v1(rate_hz, cutoff_hz)
                    .into_iter()
                    .enumerate()
                {
                    for section in ["high_pass", "low_pass"] {
                        expected.insert(format!(
                            "response-{section}-{rate_hz}-{quantum_frames}-{cutoff_index}-{probe_index}"
                        ));
                    }
                }
            }
            for (probe_index, _) in frozen_cascade_probes_v1(rate_hz).into_iter().enumerate() {
                expected.insert(format!(
                    "response-cascade-{rate_hz}-{quantum_frames}-fixed-{probe_index}"
                ));
            }
        }
    }
    expected
}

fn frozen_single_section_probes_v1(rate_hz: u32, cutoff_hz: f64) -> Vec<f64> {
    let mut probes = probes(rate_hz, cutoff_hz);
    probes.push(cutoff_hz);
    probes.push(0.49 * f64::from(rate_hz));
    sort_and_deduplicate_f64_v1(&mut probes);
    probes
}

fn frozen_cascade_probes_v1(rate_hz: u32) -> Vec<f64> {
    let mut values = probes(rate_hz, 100.0);
    values.extend(probes(rate_hz, 1_000.0));
    sort_and_deduplicate_f64_v1(&mut values);
    values
}

fn sort_and_deduplicate_f64_v1(values: &mut Vec<f64>) {
    values.sort_by(f64::total_cmp);
    values.dedup_by(|left, right| left.to_bits() == right.to_bits());
}

fn verify_response_oracle_tolerances_v1(rows: &[ResponseCsvRowV1]) -> Result<(), String> {
    for row in rows {
        if row.recovery_count != 0 {
            return Err(format!(
                "reference/filter-response.csv legal recovery count is nonzero: {} has {}",
                row.id, row.recovery_count
            ));
        }
        let rbj = independent_rbj_magnitude_db_v1(row)?;
        if (row.rbj_magnitude_db - rbj).abs() > RESPONSE_RBJ_SERIALIZATION_TOLERANCE_DB_V1 {
            return Err(format!(
                "reference/filter-response.csv independent RBJ provenance differs: {}",
                row.id
            ));
        }
        if rbj >= -120.0
            && (row.cast_state_magnitude_db - rbj).abs() > RESPONSE_CAST_STATE_TOLERANCE_DB_V1
        {
            return Err(format!(
                "reference/filter-response.csv cast-state tolerance exceeds 0.005 dB: {}",
                row.id
            ));
        }
        if !is_coherent_measurement_probe_v1(row) {
            continue;
        }
        if rbj >= -120.0
            && (row.impulse_dft_magnitude_db - rbj).abs() > RESPONSE_IMPULSE_DFT_TOLERANCE_DB_V1
        {
            return Err(format!(
                "reference/filter-response.csv impulse-DFT tolerance exceeds 0.05 dB: {}",
                row.id
            ));
        }
        if rbj >= -90.0 {
            if (row.sustained_fundamental_db - rbj).abs() > RESPONSE_FUNDAMENTAL_TOLERANCE_DB_V1 {
                return Err(format!(
                    "reference/filter-response.csv sustained fundamental exceeds 0.05 dB: {}",
                    row.id
                ));
            }
            if row.sustained_residual_db > RESPONSE_RESIDUAL_LIMIT_DB_V1 {
                return Err(format!(
                    "reference/filter-response.csv sustained residual exceeds -100 dB: {}",
                    row.id
                ));
            }
        } else if row.sustained_total_db > RESPONSE_ATTENUATED_TOTAL_LIMIT_DB_V1 {
            return Err(format!(
                "reference/filter-response.csv attenuated total exceeds -88 dB: {}",
                row.id
            ));
        }
        if row.tail_energy < 0.0 {
            return Err(format!(
                "reference/filter-response.csv tail energy is negative: {}",
                row.id
            ));
        }
    }
    Ok(())
}

fn is_coherent_measurement_probe_v1(row: &ResponseCsvRowV1) -> bool {
    match row.section {
        ResponseSectionV1::HighPass | ResponseSectionV1::LowPass => {
            probes(row.rate_hz, row.cutoff_hz)
                .into_iter()
                .any(|probe_hz| probe_hz.to_bits() == row.probe_hz.to_bits())
        }
        ResponseSectionV1::Cascade => frozen_cascade_probes_v1(row.rate_hz)
            .into_iter()
            .any(|probe_hz| probe_hz.to_bits() == row.probe_hz.to_bits()),
    }
}

fn independent_rbj_magnitude_db_v1(row: &ResponseCsvRowV1) -> Result<f64, String> {
    let rate_hz = f64::from(row.rate_hz);
    let magnitude = match row.section {
        ResponseSectionV1::HighPass => rbj_butterworth_magnitude_db(
            rate_hz,
            row.cutoff_hz,
            ReferenceFilterKind::HighPass,
            row.probe_hz,
        ),
        ResponseSectionV1::LowPass => rbj_butterworth_magnitude_db(
            rate_hz,
            row.cutoff_hz,
            ReferenceFilterKind::LowPass,
            row.probe_hz,
        ),
        ResponseSectionV1::Cascade => {
            if row.cutoff_hz.to_bits() != 100.0_f64.to_bits() {
                return Err(format!(
                    "reference/filter-response.csv cascade cutoff is not fixed 100 Hz: {}",
                    row.id
                ));
            }
            let hpf = rbj_butterworth_magnitude_db(
                rate_hz,
                100.0,
                ReferenceFilterKind::HighPass,
                row.probe_hz,
            );
            let lpf = rbj_butterworth_magnitude_db(
                rate_hz,
                1_000.0,
                ReferenceFilterKind::LowPass,
                row.probe_hz,
            );
            hpf.zip(lpf).map(|(hpf, lpf)| hpf + lpf)
        }
    };
    magnitude.ok_or_else(|| {
        format!(
            "independent RBJ oracle rejected frozen response coordinate: {}",
            row.id
        )
    })
}

fn verify_jsonl_payloads_v1(root: &Path) -> Result<(), String> {
    for path in [
        "meters/graph-taps.jsonl",
        "meters/window-and-drop.jsonl",
        "diagnostics.jsonl",
        "resources.jsonl",
    ] {
        let bytes = read_regular_file(&root.join(path), path)?;
        let text =
            std::str::from_utf8(&bytes).map_err(|_| format!("JSONL is not UTF-8: {path}"))?;
        let mut records = 0_usize;
        for (index, line) in text.split_inclusive('\n').enumerate() {
            let record = line.strip_suffix('\n').ok_or_else(|| {
                format!("JSONL record is not LF terminated: {path}:{}", index + 1)
            })?;
            if record.is_empty() || !record.starts_with('{') || !record.ends_with('}') {
                return Err(format!(
                    "JSONL record is not a canonical object: {path}:{}",
                    index + 1
                ));
            }
            records += 1;
        }
        if records == 0 {
            return Err(format!("JSONL payload has no records: {path}"));
        }
    }
    Ok(())
}

fn benchmark_path_v1(path: &str) -> Option<(BenchmarkKindV1, u32)> {
    let name = path.strip_prefix("benchmark/")?.strip_suffix(".toml")?;
    let (kind, rate_hz) = name.rsplit_once('-')?;
    let rate_hz = rate_hz.parse().ok()?;
    let kind = BenchmarkKindV1::parse(kind)?;
    BENCHMARK_RATES_V1
        .contains(&rate_hz)
        .then_some((kind, rate_hz))
}

impl BenchmarkKindV1 {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "full_chain_filters" => Some(Self::FullChainFilters),
            "identity_chain" => Some(Self::IdentityChain),
            "matrix_ramp" => Some(Self::MatrixRamp),
            "meter_success_full" => Some(Self::MeterSuccessFull),
            "prepare_256_tracks" => Some(Self::Prepare256Tracks),
            _ => None,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::FullChainFilters => "full_chain_filters",
            Self::IdentityChain => "identity_chain",
            Self::MatrixRamp => "matrix_ramp",
            Self::MeterSuccessFull => "meter_success_full",
            Self::Prepare256Tracks => "prepare_256_tracks",
        }
    }

    const fn references_pcm(self) -> bool {
        !matches!(self, Self::Prepare256Tracks)
    }
}

fn verify_benchmark_inputs_v1(root: &Path, manifest: &FixtureManifestV1) -> Result<(), String> {
    let manifest_entries: BTreeMap<_, _> = manifest
        .entries
        .iter()
        .map(|entry| (entry.path.as_str(), entry))
        .collect();
    let actual: BTreeSet<_> = manifest
        .entries
        .iter()
        .filter_map(|entry| match entry.class {
            FixturePathClassV1::Benchmark => Some(entry.path.as_str()),
            _ => None,
        })
        .collect();
    let expected: BTreeSet<_> = BENCHMARK_KINDS_V1
        .into_iter()
        .flat_map(|kind| {
            BENCHMARK_RATES_V1
                .into_iter()
                .map(move |rate_hz| format!("benchmark/{kind}-{rate_hz}.toml"))
        })
        .collect();
    let expected: BTreeSet<_> = expected.iter().map(String::as_str).collect();
    if actual != expected {
        return Err("benchmark input paths differ from the frozen V1 grid".to_owned());
    }
    for kind in BENCHMARK_KINDS_V1 {
        for rate_hz in BENCHMARK_RATES_V1 {
            let path = format!("benchmark/{kind}-{rate_hz}.toml");
            let input = parse_benchmark_input_v1(root, &path)?;
            if input.kind.as_str() != kind || input.rate_hz != rate_hz {
                return Err(format!("benchmark input identity mismatch: {path}"));
            }
            if input.kind.references_pcm() {
                let pcm_path = benchmark_field_v1(&input, "input_pcm_path")
                    .ok_or_else(|| format!("benchmark input has no PCM path: {path}"))?;
                let pcm_sha256 = benchmark_field_v1(&input, "input_pcm_sha256")
                    .ok_or_else(|| format!("benchmark input has no PCM hash: {path}"))?;
                let pcm_path = quoted_toml_string_v1(pcm_path)
                    .ok_or_else(|| format!("benchmark input PCM path is not quoted: {path}"))?;
                let pcm_sha256 = quoted_toml_string_v1(pcm_sha256)
                    .ok_or_else(|| format!("benchmark input PCM hash is not quoted: {path}"))?;
                let manifest_entry = manifest_entries.get(pcm_path).ok_or_else(|| {
                    format!("benchmark input references unlisted PCM: {path} -> {pcm_path}")
                })?;
                if manifest_entry.class != FixturePathClassV1::Pcm
                    || manifest_entry.sha256 != pcm_sha256
                {
                    return Err(format!(
                        "benchmark input PCM hash does not match manifest: {path} -> {pcm_path}"
                    ));
                }
            }
        }
    }
    Ok(())
}

fn parse_benchmark_input_v1(root: &Path, path: &str) -> Result<BenchmarkInputV1, String> {
    let (_, path_rate_hz) = benchmark_path_v1(path)
        .ok_or_else(|| format!("benchmark input path is invalid: {path}"))?;
    let bytes = read_regular_file(&root.join(path), path)?;
    let text =
        std::str::from_utf8(&bytes).map_err(|_| format!("benchmark input is not UTF-8: {path}"))?;
    let mut fields = Vec::new();
    let mut seen = BTreeSet::new();
    for (line_number, line) in text.split_inclusive('\n').enumerate() {
        let line = line.strip_suffix('\n').ok_or_else(|| {
            format!(
                "benchmark input line is not LF terminated: {path}:{}",
                line_number + 1
            )
        })?;
        let (key, value) = line.split_once(" = ").ok_or_else(|| {
            format!(
                "benchmark input line is not canonical: {path}:{}",
                line_number + 1
            )
        })?;
        if !key
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
            || !seen.insert(key)
        {
            return Err(format!(
                "benchmark input key is invalid or duplicate: {path}:{key}"
            ));
        }
        if value.is_empty() {
            return Err(format!("benchmark input value is empty: {path}:{key}"));
        }
        fields.push((key.to_owned(), value.to_owned()));
    }
    let kind = benchmark_field_from_pairs_v1(&fields, "workload_kind")
        .and_then(quoted_toml_string_v1)
        .and_then(BenchmarkKindV1::parse)
        .ok_or_else(|| format!("benchmark input workload_kind is invalid: {path}"))?;
    let rate_hz = benchmark_field_from_pairs_v1(&fields, "sample_rate_hz")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| format!("benchmark input sample_rate_hz is invalid: {path}"))?;
    if rate_hz != path_rate_hz {
        return Err(format!("benchmark input rate does not match path: {path}"));
    }
    let expected = expected_benchmark_fields_v1(kind, rate_hz);
    if fields != expected {
        return Err(format!(
            "benchmark input fields are incomplete or noncanonical: {path}"
        ));
    }
    Ok(BenchmarkInputV1 {
        kind,
        rate_hz,
        fields,
    })
}

fn benchmark_field_v1<'a>(input: &'a BenchmarkInputV1, key: &str) -> Option<&'a str> {
    benchmark_field_from_pairs_v1(&input.fields, key)
}

fn benchmark_field_from_pairs_v1<'a>(fields: &'a [(String, String)], key: &str) -> Option<&'a str> {
    fields
        .iter()
        .find_map(|(field, value)| (field == key).then_some(value.as_str()))
}

fn quoted_toml_string_v1(value: &str) -> Option<&str> {
    value.strip_prefix('"')?.strip_suffix('"')
}

fn expected_benchmark_fields_v1(kind: BenchmarkKindV1, rate_hz: u32) -> Vec<(String, String)> {
    let mut fields = vec![
        benchmark_field_pair_v1("fixture_schema", "1"),
        benchmark_field_pair_v1("issue", "35"),
        benchmark_field_pair_v1("workload_kind", &format!("\"{}\"", kind.as_str())),
        benchmark_field_pair_v1(
            "workload_id",
            &format!("\"issue035.{}.{}hz.q128\"", kind.as_str(), rate_hz),
        ),
        benchmark_field_pair_v1("sample_rate_hz", &rate_hz.to_string()),
        benchmark_field_pair_v1("quantum_frames", "128"),
    ];
    match kind {
        BenchmarkKindV1::FullChainFilters => fields.extend([
            benchmark_field_pair_v1("tracks", "1"),
            benchmark_field_pair_v1("meter_observers", "0"),
            benchmark_field_pair_v1("meter_queue_capacity", "0"),
            benchmark_field_pair_v1("state_mode", "\"continuous\""),
            benchmark_field_pair_v1("input_pcm_path", "\"pcm/filters-asymmetric.f32le\""),
            benchmark_field_pair_v1("input_pcm_sha256", "\"7b78746567c4c36d221643d359840968e832925661354009cb0b31e45d914fa3\""),
            benchmark_field_pair_v1("left_hpf_hz", "100.0"),
            benchmark_field_pair_v1("right_hpf_hz", "200.0"),
            benchmark_field_pair_v1("left_lpf_hz", "1000.0"),
            benchmark_field_pair_v1("right_lpf_hz", "2000.0"),
            benchmark_field_pair_v1("left_trim_db", "-3.0"),
            benchmark_field_pair_v1("right_trim_db", "2.0"),
            benchmark_field_pair_v1("left_fader_db", "-1.0"),
            benchmark_field_pair_v1("right_fader_db", "-4.0"),
            benchmark_field_pair_v1("matrix_ll", "0.8"),
            benchmark_field_pair_v1("matrix_lr", "0.2"),
            benchmark_field_pair_v1("matrix_rl", "-0.3"),
            benchmark_field_pair_v1("matrix_rr", "0.7"),
        ]),
        BenchmarkKindV1::IdentityChain => fields.extend([
            benchmark_field_pair_v1("tracks", "1"),
            benchmark_field_pair_v1("meter_observers", "0"),
            benchmark_field_pair_v1("meter_queue_capacity", "0"),
            benchmark_field_pair_v1("state_mode", "\"continuous\""),
            benchmark_field_pair_v1("input_pcm_path", "\"pcm/identity-signed-zero.f32le\""),
            benchmark_field_pair_v1("input_pcm_sha256", "\"602eb824699c16d5c423a0196bc71b02d207783d0007fcfd7ed9566784709e99\""),
            benchmark_field_pair_v1("left_hpf_hz", "0.0"),
            benchmark_field_pair_v1("right_hpf_hz", "0.0"),
            benchmark_field_pair_v1("left_lpf_hz", "0.0"),
            benchmark_field_pair_v1("right_lpf_hz", "0.0"),
            benchmark_field_pair_v1("left_trim_db", "0.0"),
            benchmark_field_pair_v1("right_trim_db", "0.0"),
            benchmark_field_pair_v1("left_fader_db", "0.0"),
            benchmark_field_pair_v1("right_fader_db", "0.0"),
            benchmark_field_pair_v1("matrix_ll", "1.0"),
            benchmark_field_pair_v1("matrix_lr", "0.0"),
            benchmark_field_pair_v1("matrix_rl", "0.0"),
            benchmark_field_pair_v1("matrix_rr", "1.0"),
        ]),
        BenchmarkKindV1::MatrixRamp => fields.extend([
            benchmark_field_pair_v1("tracks", "1"),
            benchmark_field_pair_v1("meter_observers", "0"),
            benchmark_field_pair_v1("meter_queue_capacity", "0"),
            benchmark_field_pair_v1("state_mode", "\"continuous\""),
            benchmark_field_pair_v1("input_pcm_path", "\"pcm/matrix-ramp-128.f32le\""),
            benchmark_field_pair_v1("input_pcm_sha256", "\"4b302238e21a45301a1faca72b292d92feacde0dd17df7ddc8f9c271bc693fb8\""),
            benchmark_field_pair_v1("smoothing_updates", "128"),
            benchmark_field_pair_v1("target_selection", "\"alternating_by_operation\""),
            benchmark_field_pair_v1("initial_matrix_ll", "0.7"),
            benchmark_field_pair_v1("initial_matrix_lr", "0.3"),
            benchmark_field_pair_v1("initial_matrix_rl", "-0.2"),
            benchmark_field_pair_v1("initial_matrix_rr", "0.8"),
            benchmark_field_pair_v1("even_target_ll", "0.6"),
            benchmark_field_pair_v1("even_target_lr", "0.4"),
            benchmark_field_pair_v1("even_target_rl", "-0.4"),
            benchmark_field_pair_v1("even_target_rr", "0.6"),
            benchmark_field_pair_v1("odd_target_ll", "0.9"),
            benchmark_field_pair_v1("odd_target_lr", "-0.1"),
            benchmark_field_pair_v1("odd_target_rl", "0.2"),
            benchmark_field_pair_v1("odd_target_rr", "0.8"),
        ]),
        BenchmarkKindV1::MeterSuccessFull => fields.extend([
            benchmark_field_pair_v1("tracks", "1"),
            benchmark_field_pair_v1("meter_observers", "14"),
            benchmark_field_pair_v1("meter_queue_capacity", "1"),
            benchmark_field_pair_v1("state_mode", "\"continuous\""),
            benchmark_field_pair_v1("input_pcm_path", "\"pcm/graph-taps.f32le\""),
            benchmark_field_pair_v1("input_pcm_sha256", "\"e07cfb2696b6eb2d8114ab84653186395694ba9c16904b70d8b0238903cad46f\""),
            benchmark_field_pair_v1("meter_period_frames", "128"),
            benchmark_field_pair_v1("meter_peak_hold_frames", "0"),
            benchmark_field_pair_v1("meter_peak_decay_db_per_second", "0.0"),
            benchmark_field_pair_v1("meter_reset_generation", "7"),
            benchmark_field_pair_v1("success_taps", "\"input,post_input_builtins,post_simd1,post_dynamic,post_simd2_pre_fader,post_fader,post_matrix\""),
            benchmark_field_pair_v1("full_taps", "\"input,post_input_builtins,post_simd1,post_dynamic,post_simd2_pre_fader,post_fader,post_matrix\""),
            benchmark_field_pair_v1("success_drain_per_operation", "true"),
            benchmark_field_pair_v1("full_prefill", "true"),
        ]),
        BenchmarkKindV1::Prepare256Tracks => fields.extend([
            benchmark_field_pair_v1("tracks", "256"),
            benchmark_field_pair_v1("meter_observers", "56"),
            benchmark_field_pair_v1("meter_queue_capacity", "4"),
            benchmark_field_pair_v1("state_mode", "\"new_per_prepare\""),
            benchmark_field_pair_v1("session_template_path", "\"fixtures/session/v1/canonical.toml\""),
            benchmark_field_pair_v1("session_template_sha256", "\"1ff2db241f84b1a641b50c69c4fd09eda0a1baa0a5735d3769c056212927f31a\""),
            benchmark_field_pair_v1("track_id_prefix", "\"benchmark-track-\""),
            benchmark_field_pair_v1("track_id_count", "256"),
            benchmark_field_pair_v1("empty_effect_racks", "true"),
            benchmark_field_pair_v1("route_source_track_id", "\"benchmark-track-0\""),
            benchmark_field_pair_v1("route_source_tap", "\"post_matrix\""),
            benchmark_field_pair_v1("meter_track_ids", "\"benchmark-track-0,benchmark-track-1,benchmark-track-2,benchmark-track-3,benchmark-track-4,benchmark-track-5,benchmark-track-6,benchmark-track-7\""),
            benchmark_field_pair_v1("meter_taps", "\"input,post_input_builtins,post_simd1,post_dynamic,post_simd2_pre_fader,post_fader,post_matrix\""),
            benchmark_field_pair_v1("meter_period_frames", "128"),
            benchmark_field_pair_v1("meter_peak_hold_frames", "0"),
            benchmark_field_pair_v1("meter_peak_decay_db_per_second", "0.0"),
            benchmark_field_pair_v1("meter_reset_generation", "7"),
        ]),
    }
    fields
}

fn benchmark_field_pair_v1(key: &str, value: &str) -> (String, String) {
    (key.to_owned(), value.to_owned())
}

fn canonical_benchmark_input_v1(kind: BenchmarkKindV1, rate_hz: u32) -> String {
    let mut output = String::new();
    for (key, value) in expected_benchmark_fields_v1(kind, rate_hz) {
        writeln!(output, "{key} = {value}").expect("string");
    }
    output
}

fn list_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    fn visit(root: &Path, path: &Path, files: &mut Vec<String>) -> Result<(), String> {
        for entry in
            fs::read_dir(path).map_err(|error| format!("read fixture directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("read fixture entry: {error}"))?;
            if entry
                .file_type()
                .map_err(|error| format!("read fixture type: {error}"))?
                .is_dir()
            {
                visit(root, &entry.path(), files)?;
            } else {
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .map_err(|_| "fixture relative path".to_owned())?
                    .to_str()
                    .ok_or_else(|| "fixture path is not UTF-8".to_owned())?
                    .replace('\\', "/");
                if relative != "MANIFEST.tsv" {
                    files.push(relative);
                }
            }
        }
        Ok(())
    }
    visit(root, root, &mut files)?;
    Ok(files)
}

fn fixture_tree_hash_v1(root: &Path) -> Result<[u8; 32], String> {
    let mut paths = list_files(root)?;
    paths.push("MANIFEST.tsv".to_owned());
    paths.sort();
    let mut digest = Sha256::new();
    for path in paths {
        let bytes = read_regular_file(&root.join(&path), &path)?;
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update(
            u64::try_from(bytes.len())
                .map_err(|_| "fixture length overflows u64")?
                .to_le_bytes(),
        );
        digest.update(bytes);
    }
    Ok(digest.finalize().into())
}

fn sha256(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(output, "{byte:02x}").expect("string");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn v1_check_accepts_complete_generated_corpus_without_writing() {
        let root = temporary_root("valid");
        let files = complete_files();
        write_fixture(&root, &files);
        let before = read_fixture_tree(&root);

        check_read_only_fixture_root_v1(&root).expect("complete V1 fixture corpus");

        assert_eq!(
            before,
            read_fixture_tree(&root),
            "--check mutated the fixture root"
        );
        remove_temporary_root(root);
    }

    #[test]
    fn v1_check_rejects_all_twenty_four_format_mutations() {
        let files = complete_files();
        let targets = [
            ("toml", "cases.toml"),
            ("f32le", "pcm/identity-signed-zero.f32le"),
            ("csv", "reference/filter-response.csv"),
            ("meter_jsonl", "meters/window-and-drop.jsonl"),
            ("diagnostics_jsonl", "diagnostics.jsonl"),
            ("resources_jsonl", "resources.jsonl"),
        ];
        for (class, path) in targets {
            reject_payload_mutation(&files, class, "delete", |root| {
                fs::remove_file(root.join(path)).expect("delete fixture payload");
            });
            reject_payload_mutation(&files, class, "alter", |root| {
                let mut bytes = fs::read(root.join(path)).expect("read fixture payload");
                bytes.push(0);
                fs::write(root.join(path), bytes).expect("alter fixture payload");
            });
            reject_payload_mutation(&files, class, "add", |root| {
                fs::write(root.join(format!("unlisted-{class}")), b"unexpected\n")
                    .expect("add fixture payload");
            });
            reject_coverage_hole(&files, class, |mutated| match class {
                "toml" => {
                    let cases =
                        String::from_utf8(mutated.get("cases.toml").expect("cases").clone())
                            .expect("utf8 cases");
                    mutated.insert(
                        "cases.toml".to_owned(),
                        remove_first_response_case(&cases).into_bytes(),
                    );
                }
                "f32le" => {
                    mutated.remove("pcm/identity-signed-zero.f32le");
                }
                "csv" => {
                    let csv = String::from_utf8(
                        mutated
                            .get("reference/filter-response.csv")
                            .expect("reference")
                            .clone(),
                    )
                    .expect("utf8 reference");
                    mutated.insert(
                        "reference/filter-response.csv".to_owned(),
                        remove_first_data_row(&csv).into_bytes(),
                    );
                }
                "meter_jsonl" | "diagnostics_jsonl" | "resources_jsonl" => {
                    mutated.insert(path.to_owned(), Vec::new());
                }
                _ => unreachable!("frozen format class"),
            });
        }
    }

    #[test]
    fn v1_check_rejects_manifest_grammar() {
        let root = temporary_root("manifest");
        let files = complete_files();
        write_fixture(&root, &files);

        fs::remove_file(root.join("MANIFEST.tsv")).expect("remove manifest");
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted missing manifest"
        );
        write_fixture(&root, &files);

        fs::write(root.join("MANIFEST.tsv"), "path\tlength\tsha256\n../unsafe\t1\t0000000000000000000000000000000000000000000000000000000000000000\n")
            .expect("unsafe manifest");
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted unsafe manifest path"
        );
        write_fixture(&root, &files);

        let manifest = fs::read_to_string(root.join("MANIFEST.tsv")).expect("manifest");
        let first_entry = manifest.lines().nth(1).expect("manifest entry");
        fs::write(
            root.join("MANIFEST.tsv"),
            format!("{manifest}{first_entry}\n"),
        )
        .expect("duplicate manifest entry");
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted duplicate manifest entry"
        );
        remove_temporary_root(root);
    }

    #[test]
    fn v1_check_rejects_benchmark_identity_parameter_and_pcm_hash_mutations() {
        let files = complete_files();
        for (name, mutate) in [
            (
                "workload_id",
                benchmark_text_mutation_v1(
                    "workload_id = \"issue035.full_chain_filters.48000hz.q128\"",
                    "workload_id = \"issue035.full_chain_filters.96000hz.q128\"",
                ),
            ),
            (
                "missing_parameter",
                benchmark_text_mutation_v1("matrix_rr = 0.7\n", ""),
            ),
            (
                "declared_pcm_hash",
                benchmark_text_mutation_v1(
                    "input_pcm_sha256 = \"7b78746567c4c36d221643d359840968e832925661354009cb0b31e45d914fa3\"",
                    "input_pcm_sha256 = \"0000000000000000000000000000000000000000000000000000000000000000\"",
                ),
            ),
        ] {
            let root = temporary_root(&format!("benchmark-{name}"));
            let mut mutated = files.clone();
            mutate(&mut mutated);
            write_fixture(&root, &mutated);
            assert!(
                check_fixture_root_v1(&root).is_err(),
                "accepted benchmark {name} mutation"
            );
            remove_temporary_root(root);
        }

        let root = temporary_root("benchmark-pcm-content");
        let mut mutated = files;
        mutated
            .get_mut("pcm/filters-asymmetric.f32le")
            .expect("fixture PCM")
            .push(0);
        write_fixture(&root, &mutated);
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted benchmark PCM manifest-hash mismatch"
        );
        remove_temporary_root(root);
    }

    fn reject_payload_mutation(
        files: &BTreeMap<String, Vec<u8>>,
        class: &str,
        mutation: &str,
        mutate: impl FnOnce(&Path),
    ) {
        let root = temporary_root(&format!("{class}-{mutation}"));
        write_fixture(&root, files);
        mutate(&root);
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted {class} {mutation} mutation"
        );
        remove_temporary_root(root);
    }

    fn reject_coverage_hole(
        files: &BTreeMap<String, Vec<u8>>,
        class: &str,
        mutate: impl FnOnce(&mut BTreeMap<String, Vec<u8>>),
    ) {
        let root = temporary_root(&format!("{class}-coverage-hole"));
        let mut mutated = files.clone();
        mutate(&mut mutated);
        write_fixture(&root, &mutated);
        assert!(
            check_fixture_root_v1(&root).is_err(),
            "accepted {class} manifest-valid coverage hole"
        );
        remove_temporary_root(root);
    }

    fn complete_files() -> BTreeMap<String, Vec<u8>> {
        let mut files: BTreeMap<_, _> = generated().into_iter().collect();
        for kind in BENCHMARK_KINDS_V1 {
            let kind = BenchmarkKindV1::parse(kind).expect("frozen benchmark kind");
            for rate_hz in BENCHMARK_RATES_V1 {
                files.insert(
                    format!("benchmark/{}-{rate_hz}.toml", kind.as_str()),
                    canonical_benchmark_input_v1(kind, rate_hz).into_bytes(),
                );
            }
        }
        files
    }

    fn benchmark_text_mutation_v1(
        from: &'static str,
        to: &'static str,
    ) -> impl FnOnce(&mut BTreeMap<String, Vec<u8>>) {
        move |files| {
            let path = "benchmark/full_chain_filters-48000.toml";
            let input = String::from_utf8(files.get(path).expect("benchmark input").clone())
                .expect("benchmark input UTF-8");
            assert!(input.contains(from), "frozen benchmark field");
            files.insert(path.to_owned(), input.replacen(from, to, 1).into_bytes());
        }
    }

    fn write_fixture(root: &Path, files: &BTreeMap<String, Vec<u8>>) {
        if root.exists() {
            fs::remove_dir_all(root).expect("replace fixture root");
        }
        fs::create_dir_all(root).expect("fixture root");
        for (path, bytes) in files {
            let destination = root.join(path);
            fs::create_dir_all(destination.parent().expect("fixture parent"))
                .expect("fixture directory");
            fs::write(destination, bytes).expect("fixture bytes");
        }
        let manifest_files: Vec<_> = files
            .iter()
            .map(|(path, bytes)| (path.clone(), bytes.clone()))
            .collect();
        fs::write(root.join("MANIFEST.tsv"), manifest(&manifest_files)).expect("manifest");
    }

    fn read_fixture_tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        let mut files = BTreeMap::new();
        for path in list_files(root).expect("list fixtures") {
            files.insert(
                PathBuf::from(&path),
                fs::read(root.join(path)).expect("fixture bytes"),
            );
        }
        files.insert(
            PathBuf::from("MANIFEST.tsv"),
            fs::read(root.join("MANIFEST.tsv")).expect("manifest bytes"),
        );
        files
    }

    fn remove_first_response_case(cases: &str) -> String {
        let category = cases
            .find("category = \"filter_response\"")
            .expect("response case");
        let start = cases[..category].rfind("[[case]]\n").expect("case start");
        let remainder = &cases[start..];
        let end = remainder
            .find("\n[[case]]\n")
            .map(|offset| start + offset + 1)
            .unwrap_or(cases.len());
        format!("{}{}", &cases[..start], &cases[end..])
    }

    fn remove_first_data_row(csv: &str) -> String {
        let first_newline = csv.find('\n').expect("csv header");
        let remainder = &csv[first_newline + 1..];
        let second_newline = remainder.find('\n').expect("csv data");
        format!(
            "{}{}",
            &csv[..first_newline + 1],
            &remainder[second_newline + 1..]
        )
    }

    fn temporary_root(label: &str) -> PathBuf {
        let sequence = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "miso-engine-builtins-fixture-v1-{}-{sequence}-{label}",
            std::process::id()
        ))
    }

    fn remove_temporary_root(root: PathBuf) {
        fs::remove_dir_all(root).expect("remove fixture root");
    }
}
