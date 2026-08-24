//! Fixed-work, exactly-two-round descriptive benchmark driver for issue 006.
#![allow(missing_docs)]

use miso_engine_bench_support::json::escape;
use miso_engine_bench_support::stats::per_mille as percentile;
use miso_engine_graph_compiler::Backend;
use std::{
    env,
    fmt::Write as _,
    fs,
    hint::black_box,
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_effect_compiler::{
    EffectCompileCaps, EffectPreparedSession, prepare_native_session_effects,
};
use miso_engine_effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use miso_engine_graph::{GraphCompileCaps, GraphResourceEstimate};
use miso_engine_graph_compiler::{GraphCompileRequest, GraphCompiler};
use miso_engine_session::{
    ChannelMatrix, CompileCaps, CompiledSession, EffectIdentity, EffectParam, ParameterChannel,
    ParameterUnit, Route, RouteDestination, RouteSource, SendTap, SessionTomlV1, Sidechain,
    SidechainDeclaration, StableId, Submix, canonical_session_toml, compile_session,
    parse_session_toml,
};
use sha2::{Digest, Sha256};

const SEED: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
const ROUNDS: u8 = 2;

#[derive(Clone, Copy)]
enum Workload {
    CanonicalCompile,
    ScaleValidation,
    CanonicalDebug,
}

impl Workload {
    const ALL: [Self; 3] = [
        Self::CanonicalCompile,
        Self::ScaleValidation,
        Self::CanonicalDebug,
    ];

    const fn id(self) -> &'static str {
        match self {
            Self::CanonicalCompile => "graph_compile_256t_1024r_32s",
            Self::ScaleValidation => "graph_validate_65537_tracks",
            Self::CanonicalDebug => "graph_debug_sha_dot_256t_1024r_32s",
        }
    }

    const fn warmups(self) -> usize {
        match self {
            Self::ScaleValidation => 0,
            _ => 1,
        }
    }

    const fn iterations(self) -> usize {
        match self {
            Self::ScaleValidation => 1,
            _ => 5,
        }
    }
}

struct Fixture {
    label: &'static str,
    canonical_bytes: Vec<u8>,
    session: CompiledSession,
    tracks: usize,
    routes: usize,
    submixes: usize,
    effects: usize,
    sidechains: usize,
}

#[derive(Clone)]
struct Sample {
    total_ns: u128,
    effect_prepare_ns: u128,
    graph_compile_ns: u128,
    graph_sha256: String,
    canonical_debug_bytes: usize,
    dot_bytes: usize,
    estimate: GraphResourceEstimate,
}

struct Metadata {
    timestamp_epoch_seconds: u64,
    cpu: String,
    os: String,
    governor_or_power_mode: String,
    power_source: String,
    rustc: String,
    llvm: String,
    target_triple: String,
    target_features: String,
    opt_level: String,
    lto: String,
    codegen_units: String,
    background_load: String,
    missing: Vec<String>,
}

pub(crate) fn main() {
    let arguments: Vec<_> = env::args().skip(1).collect();
    assert!(arguments.is_empty(), "usage: miso_engine_graph_bench");
    let canonical = representative_fixture();
    let scale = scale_fixture();
    let metadata = metadata();
    for round in 1..=ROUNDS {
        for workload in Workload::ALL {
            let fixture = match workload {
                Workload::ScaleValidation => &scale,
                Workload::CanonicalCompile | Workload::CanonicalDebug => &canonical,
            };
            let samples = run(workload, fixture);
            println!("{}", record(workload, round, fixture, &samples, &metadata));
        }
    }
}

fn representative_fixture() -> Fixture {
    let mut model = parse_session_toml(SEED).expect("seed session");
    model.limits.memory_bytes = i64::MAX as u64;
    let track_template = model.tracks.pop().expect("seed track");
    let route_template = model.routes.pop().expect("seed route");
    model.automation.clear();
    model.submixes = (0..32)
        .map(|index| Submix {
            id: stable(&format!("submix-{index:02}")),
        })
        .collect();
    model.tracks = (0..256)
        .map(|index| {
            let mut track = track_template.clone();
            track.id = stable(&format!("track-{index:03}"));
            track.simd1.effects.clear();
            track.simd2.effects.clear();
            track.dynamic.effects.clear();
            if index % 4 == 1 {
                let mut effect = track_template.dynamic.effects[0].clone();
                effect.id = stable("delay");
                effect.identity = EffectIdentity::Native {
                    effect_id: stable("conformance.delay"),
                };
                effect.params = vec![
                    EffectParam {
                        parameter_id: 1,
                        channel: ParameterChannel::Left,
                        unit: ParameterUnit::Linear,
                        value: 1.0,
                    },
                    EffectParam {
                        parameter_id: 1,
                        channel: ParameterChannel::Right,
                        unit: ParameterUnit::Linear,
                        value: 1.0,
                    },
                ];
                if index % 8 == 1 {
                    effect.sidechain = SidechainDeclaration::Routed(Sidechain {
                        source: RouteSource::Track {
                            track_id: stable("track-000"),
                            tap: SendTap::Input,
                        },
                        port_id: stable("sidechain-in"),
                    });
                } else {
                    effect.sidechain = SidechainDeclaration::None;
                }
                track.dynamic.effects.push(effect);
            }
            track
        })
        .collect();
    model.routes.clear();
    for index in 0..992 {
        let mut route = route_template.clone();
        route.id = stable(&format!("track-route-{index:04}"));
        route.source = RouteSource::Track {
            track_id: stable(&format!("track-{:03}", index % 256)),
            tap: taps()[index % taps().len()],
        };
        route.destination = RouteDestination::SubmixInput {
            submix_id: stable(&format!("submix-{:02}", index % 32)),
        };
        model.routes.push(route);
    }
    for index in 0..32 {
        model.routes.push(Route {
            id: stable(&format!("submix-route-{index:02}")),
            source: RouteSource::SubmixOutput {
                submix_id: stable(&format!("submix-{index:02}")),
            },
            destination: RouteDestination::OutputInput {
                output_id: model.outputs[0].id.clone(),
            },
            channel_matrix: ChannelMatrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
            gain_db: 0.0,
        });
    }
    fixture("canonical-mixed", model)
}

fn scale_fixture() -> Fixture {
    let mut model = parse_session_toml(SEED).expect("seed session");
    model.limits.memory_bytes = i64::MAX as u64;
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.submixes.clear();
    model.tracks = (0..65_537)
        .map(|index| {
            let mut track = template.clone();
            track.id = stable(&format!("track-{index}"));
            track
        })
        .collect();
    model.routes.truncate(1);
    model.routes[0].source = RouteSource::Track {
        track_id: stable("track-0"),
        tap: SendTap::PostMatrix,
    };
    fixture("scale-sparse", model)
}

fn fixture(label: &'static str, model: SessionTomlV1) -> Fixture {
    let tracks = model.tracks.len();
    let routes = model.routes.len();
    let submixes = model.submixes.len();
    let effects = model
        .tracks
        .iter()
        .map(|track| {
            track.simd1.effects.len() + track.dynamic.effects.len() + track.simd2.effects.len()
        })
        .sum();
    let sidechains = model
        .tracks
        .iter()
        .flat_map(|track| &track.dynamic.effects)
        .filter(|effect| matches!(effect.sidechain, SidechainDeclaration::Routed(_)))
        .count();
    let canonical = canonical_session_toml(&model).expect("canonical benchmark fixture");
    let session = compile_session(&model, unlimited_session_caps()).expect("benchmark session");
    Fixture {
        label,
        canonical_bytes: canonical.into_bytes(),
        session,
        tracks,
        routes,
        submixes,
        effects,
        sidechains,
    }
}

fn run(workload: Workload, fixture: &Fixture) -> Vec<Sample> {
    for _ in 0..workload.warmups() {
        black_box(run_once(fixture));
    }
    (0..workload.iterations())
        .map(|_| run_once(fixture))
        .collect()
}

fn run_once(fixture: &Fixture) -> Sample {
    let total = Instant::now();
    let prepare_started = Instant::now();
    let effects = prepared_effects(fixture);
    let effect_prepare_ns = prepare_started.elapsed().as_nanos();
    let compile_started = Instant::now();
    let artifact = GraphCompiler::compile(GraphCompileRequest {
        // The host's dispatch, deliberately: bank planning and `bind_homogeneous_bank` are part
        // of compile, so a scalar dispatch would quietly remove them from the timed workload and
        // make the number incomparable with every earlier run (#99 F6).
        dispatch: Backend::current(),
        plan_id: 6,
        effects,
        caps: unlimited_graph_caps(),
    })
    .unwrap_or_else(|failure| panic!("benchmark graph: {:?}", failure.diagnostics));
    let graph_compile_ns = compile_started.elapsed().as_nanos();
    // #99 F5: the evidence payload is produced here, strictly AFTER `graph_compile_ns` has been
    // taken. Before this it was built inside `GraphCompiler::compile`, so every compile -- and
    // therefore this number -- carried a multi-megabyte canonical dump, its SHA-256 and a
    // Graphviz string that no production caller ever read. The record still reports its sizes and
    // hash, because the benchmark's jq validators pin them; they are just no longer timed.
    let report = artifact.report;
    let evidence = GraphCompiler::evidence(&artifact.graph, &report);
    black_box((&evidence.canonical_bytes, &evidence.dot));
    Sample {
        total_ns: total.elapsed().as_nanos(),
        effect_prepare_ns,
        graph_compile_ns,
        graph_sha256: evidence.sha256,
        canonical_debug_bytes: evidence.canonical_bytes.len(),
        dot_bytes: evidence.dot.len(),
        estimate: report.estimate,
    }
}

fn prepared_effects(fixture: &Fixture) -> EffectPreparedSession {
    if fixture.effects == 0 {
        return EffectPreparedSession {
            session: fixture.session.clone(),
            entries: Vec::new(),
        };
    }
    let registry = NativeEffectRegistry::new([
        Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>
    ])
    .expect("benchmark registry");
    prepare_native_session_effects(
        &fixture.session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
    )
    .unwrap_or_else(|diagnostics| panic!("benchmark effects: {diagnostics:?}"))
}

fn record(
    workload: Workload,
    round: u8,
    fixture: &Fixture,
    samples: &[Sample],
    metadata: &Metadata,
) -> String {
    let mut totals: Vec<_> = samples.iter().map(|sample| sample.total_ns).collect();
    let mut prepare: Vec<_> = samples
        .iter()
        .map(|sample| sample.effect_prepare_ns)
        .collect();
    let mut compile: Vec<_> = samples
        .iter()
        .map(|sample| sample.graph_compile_ns)
        .collect();
    totals.sort_unstable();
    prepare.sort_unstable();
    compile.sort_unstable();
    let sample = &samples[0];
    let estimate = &sample.estimate;
    format!(
        concat!(
            "{{\"schema_version\":1,\"benchmark_id\":\"{}\",\"round\":{},\"rounds\":2,",
            "\"fixture_label\":\"{}\",\"fixture_sha256\":\"{}\",\"fixture_bytes\":{},",
            "\"fixture_counts\":{{\"tracks\":{},\"routes\":{},\"submixes\":{},",
            "\"effects\":{},\"sidechains\":{}}},\"sample_rate_hz\":48000,",
            "\"quantum_frames\":128,\"warmup_iterations\":{},\"measured_iterations\":{},",
            "\"percentile_method\":\"nearest-rank\",\"timing_ns\":{{\"min\":{},",
            "\"p50\":{},\"p95\":{},\"p99\":{},\"p99_9\":{},\"max\":{},",
            "\"effect_prepare_p50\":{},\"graph_compile_p50\":{}}},",
            "\"output_graph_sha256\":\"{}\",\"output_counts\":{{\"logical_nodes\":{},",
            "\"materialized_nodes\":{},\"edges\":{},\"schedule_items\":{},",
            "\"dependency_levels\":{},\"routes\":{},\"effects\":{},",
            "\"canonical_debug_bytes\":{},\"dot_bytes\":{}}},",
            "\"memory\":{{\"peak_resident_bytes\":{},\"estimated_plan_bytes\":{},",
            "\"estimated_session_plus_plan_bytes\":{},\"largest_allocation_bytes\":{}}},",
            "\"timestamp_epoch_seconds\":{},\"cpu\":\"{}\",\"os\":\"{}\",",
            "\"governor_or_power_mode\":\"{}\",\"power_source\":\"{}\",",
            "\"rustc\":\"{}\",\"llvm\":\"{}\",\"target_triple\":\"{}\",",
            "\"target_features\":\"{}\",\"opt_level\":\"{}\",\"lto\":\"{}\",",
            "\"codegen_units\":\"{}\",\"background_load\":\"{}\",",
            "\"metadata_incomplete\":{},\"missing_metadata\":{},",
            "\"errors\":0,\"descriptive_only\":true,\"threshold\":null}}"
        ),
        workload.id(),
        round,
        fixture.label,
        sha256_hex(&fixture.canonical_bytes),
        fixture.canonical_bytes.len(),
        fixture.tracks,
        fixture.routes,
        fixture.submixes,
        fixture.effects,
        fixture.sidechains,
        workload.warmups(),
        workload.iterations(),
        totals[0],
        percentile(&totals, 500),
        percentile(&totals, 950),
        percentile(&totals, 990),
        percentile(&totals, 999),
        totals[totals.len() - 1],
        percentile(&prepare, 500),
        percentile(&compile, 500),
        sample.graph_sha256,
        estimate.logical_nodes,
        estimate.materialized_nodes,
        estimate.edges,
        estimate.schedule_items,
        estimate.dependency_levels,
        estimate.routes,
        estimate.effects,
        sample.canonical_debug_bytes,
        sample.dot_bytes,
        peak_resident_bytes(),
        estimate.incremental_plan_bytes,
        estimate.session_plus_plan_bytes,
        estimate.largest_allocation_bytes,
        metadata.timestamp_epoch_seconds,
        escape(&metadata.cpu),
        escape(&metadata.os),
        escape(&metadata.governor_or_power_mode),
        escape(&metadata.power_source),
        escape(&metadata.rustc),
        escape(&metadata.llvm),
        escape(&metadata.target_triple),
        escape(&metadata.target_features),
        escape(&metadata.opt_level),
        escape(&metadata.lto),
        escape(&metadata.codegen_units),
        escape(&metadata.background_load),
        !metadata.missing.is_empty(),
        string_array(&metadata.missing),
    )
}

fn taps() -> [SendTap; 7] {
    [
        SendTap::Input,
        SendTap::PostInputBuiltins,
        SendTap::PostSimd1,
        SendTap::PostDynamic,
        SendTap::PostSimd2PreFader,
        SendTap::PostFader,
        SendTap::PostMatrix,
    ]
}

fn stable(value: &str) -> StableId {
    StableId::parse(value).expect("generated stable ID")
}

fn unlimited_session_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn unlimited_graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: u64::MAX,
        maximum_edges: u64::MAX,
        maximum_schedule_items: u64::MAX,
        maximum_dependency_levels: u64::MAX,
        maximum_audio_buffer_samples: u64::MAX,
        maximum_delay_samples_per_edge: u64::MAX,
        maximum_total_delay_samples: u64::MAX,
        maximum_graph_bytes: u64::MAX,
        maximum_plan_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_finite_tail_samples: u64::MAX,
    }
}

fn metadata() -> Metadata {
    let mut missing = Vec::new();
    let value = |name: &str, missing: &mut Vec<String>| {
        match miso_engine_bench_support::metadata::Metadata::gather().var(name) {
            Ok(value)
                if !value.is_empty()
                    && !matches!(
                        value.as_str(),
                        "unknown" | "not measured" | "default" | "target-default"
                    ) =>
            {
                value
            }
            Ok(value) if !value.is_empty() => {
                missing.push(name.to_owned());
                value
            }
            _ => {
                missing.push(name.to_owned());
                "unknown".to_owned()
            }
        }
    };
    let rustc_verbose = command("rustc", &["-vV"]);
    let rustc = rustc_verbose.lines().next().unwrap_or("unknown").to_owned();
    let llvm = rustc_verbose
        .lines()
        .find_map(|line| line.strip_prefix("LLVM version: "))
        .unwrap_or("unknown")
        .to_owned();
    let target_triple = rustc_verbose
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap_or("unknown")
        .to_owned();
    Metadata {
        timestamp_epoch_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_secs(),
        cpu: command(
            "sh",
            &["-c", "awk -F: '/model name/{print $2; exit}' /proc/cpuinfo"],
        )
        .trim()
        .to_owned(),
        os: format!("{} {}", env::consts::OS, command("uname", &["-r"])),
        governor_or_power_mode: value("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE", &mut missing),
        power_source: value("MISO_ENGINE_BENCH_POWER_SOURCE", &mut missing),
        rustc,
        llvm,
        target_triple,
        target_features: value("MISO_ENGINE_BENCH_TARGET_FEATURES", &mut missing),
        opt_level: value("MISO_ENGINE_BENCH_OPT_LEVEL", &mut missing),
        lto: value("MISO_ENGINE_BENCH_LTO", &mut missing),
        codegen_units: value("MISO_ENGINE_BENCH_CODEGEN_UNITS", &mut missing),
        background_load: value("MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE", &mut missing),
        missing,
    }
}

fn command(program: &str, arguments: &[&str]) -> String {
    Command::new(program)
        .args(arguments)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_owned())
        .trim()
        .to_owned()
}

fn peak_resident_bytes() -> u64 {
    fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|status| {
            status.lines().find_map(|line| {
                line.strip_prefix("VmHWM:")?
                    .split_whitespace()
                    .next()?
                    .parse::<u64>()
                    .ok()
            })
        })
        .and_then(|kib| kib.checked_mul(1024))
        .unwrap_or(0)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("String write");
    }
    output
}

fn string_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", escape(value)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_rank_is_ordered_at_all_frozen_percentiles() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(percentile(&values, 500), 3);
        assert_eq!(percentile(&values, 950), 5);
        assert_eq!(percentile(&values, 990), 5);
        assert_eq!(percentile(&values, 999), 5);
    }

    #[test]
    fn canonical_benchmark_fixture_prepares_and_compiles() {
        let fixture = representative_fixture();
        assert_eq!(fixture.tracks, 256);
        assert_eq!(fixture.routes, 1_024);
        assert_eq!(fixture.submixes, 32);
        assert_eq!(fixture.effects, 64);
        assert_eq!(fixture.sidechains, 32);
        let artifact = GraphCompiler::compile(GraphCompileRequest {
            // See above: the timed workload keeps the host's banks.
            dispatch: Backend::current(),
            plan_id: 6,
            effects: prepared_effects(&fixture),
            caps: unlimited_graph_caps(),
        })
        .unwrap_or_else(|failure| panic!("benchmark fixture: {:?}", failure.diagnostics));
        assert_eq!(artifact.report.estimate.routes, 1_024);
        assert_eq!(artifact.report.estimate.effects, 64);
        let evidence = GraphCompiler::evidence(&artifact.graph, &artifact.report);
        assert!(!evidence.canonical_bytes.is_empty());
        assert!(!evidence.dot.is_empty());
    }
}
