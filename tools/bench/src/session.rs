//! Fixed-work descriptive benchmark for issue-004 session control-plane operations.

use bench_support::digest::sha256_hex;
use bench_support::json::escape;
use bench_support::stats::per_mille as percentile_nearest_rank;
use bench_support::sysinfo::physical_core_count;
use std::{
    env, fs,
    hint::black_box,
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use session::{
    CompileCaps, RouteSource, SessionModel, StableId, canonical_session_json, compile_session,
    parse_session_json,
};

const CANONICAL_EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const TRACK_COUNT: usize = 256;
const ROUNDS: u8 = 2;
const WARMUP_BATCHES: usize = 64;
const MEASURED_BATCHES: usize = 512;
const OPERATIONS_PER_BATCH: usize = 8;

pub(crate) fn main() {
    let (fixture, model) = representative_fixture();
    let counts = FixtureCounts::from_session(&model);
    let metadata = Metadata::gather();
    let fixture_sha256 = sha256_hex(fixture.as_bytes());
    for method in [Method::ParseCanonical, Method::Compile] {
        for round in 1..=ROUNDS {
            let record = run_round(method, &fixture, &model);
            println!(
                "{}",
                json_record(
                    method,
                    round,
                    &record,
                    &fixture,
                    &fixture_sha256,
                    counts,
                    &metadata,
                )
            );
        }
    }
}

#[derive(Clone, Copy)]
enum Method {
    ParseCanonical,
    Compile,
}

impl Method {
    const fn id(self) -> &'static str {
        match self {
            Self::ParseCanonical => "session_parse_canonical_256_tracks",
            Self::Compile => "session_compile_256_tracks",
        }
    }
}

struct Round {
    batch_ns_per_operation: Vec<u128>,
    total_ns: u128,
}

impl Round {
    fn percentile(&self, per_mille: usize) -> u128 {
        percentile_nearest_rank(&self.batch_ns_per_operation, per_mille)
    }

    fn minimum(&self) -> u128 {
        self.batch_ns_per_operation[0]
    }

    fn maximum(&self) -> u128 {
        self.batch_ns_per_operation[self.batch_ns_per_operation.len() - 1]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FixtureCounts {
    sources: usize,
    tracks: usize,
    submixes: usize,
    outputs: usize,
    routes: usize,
    automation_programs: usize,
    effects: usize,
    effect_parameters: usize,
    automation_segments: usize,
}

impl FixtureCounts {
    fn from_session(session: &SessionModel) -> Self {
        let racks = session
            .tracks
            .iter()
            .flat_map(|track| [&track.simd1, &track.dynamic, &track.simd2]);
        let effects = racks.clone().map(|rack| rack.effects.len()).sum();
        let effect_parameters = racks
            .flat_map(|rack| &rack.effects)
            .map(|effect| effect.params.len())
            .sum();
        Self {
            sources: session.sources.len(),
            tracks: session.tracks.len(),
            submixes: session.submixes.len(),
            outputs: session.outputs.len(),
            routes: session.routes.len(),
            automation_programs: session.automation.len(),
            effects,
            effect_parameters,
            automation_segments: session
                .automation
                .iter()
                .map(|automation| automation.segments.len())
                .sum(),
        }
    }
}

fn representative_fixture() -> (String, SessionModel) {
    let mut model = parse_session_json(CANONICAL_EXAMPLE).expect("canonical seed fixture parses");
    let track_template = model.tracks.pop().expect("seed has one track");
    let route_template = model.routes.pop().expect("seed has one route");
    let automation_template = model
        .automation
        .pop()
        .expect("seed has one automation program");

    model.tracks.reserve_exact(TRACK_COUNT);
    model.routes.reserve_exact(TRACK_COUNT);
    model.automation.reserve_exact(TRACK_COUNT);
    for index in 0..TRACK_COUNT {
        let track_id = stable_id(&format!("track-{index:03}"));
        let mut track = track_template.clone();
        track.id = track_id.clone();
        model.tracks.push(track);

        let mut route = route_template.clone();
        route.id = stable_id(&format!("route-{index:03}"));
        route.source = RouteSource::Track {
            track_id: track_id.clone(),
            tap: session::SendTap::PostMatrix,
        };
        model.routes.push(route);

        let mut automation = automation_template.clone();
        automation.id = stable_id(&format!("automation-{index:03}"));
        automation.target.entity_id = track_id;
        model.automation.push(automation);
    }

    let fixture = canonical_session_json(&model).expect("representative fixture canonicalizes");
    let reparsed = parse_session_json(&fixture).expect("representative fixture reparses");
    assert_eq!(FixtureCounts::from_session(&reparsed).tracks, TRACK_COUNT);
    (fixture, reparsed)
}

fn stable_id(value: &str) -> StableId {
    StableId::parse(value).expect("generated benchmark ID is schema-valid")
}

fn run_round(method: Method, fixture: &str, model: &SessionModel) -> Round {
    for _ in 0..WARMUP_BATCHES {
        run_batch(method, fixture, model);
    }
    let mut durations = Vec::with_capacity(MEASURED_BATCHES);
    let total = Instant::now();
    for _ in 0..MEASURED_BATCHES {
        let started = Instant::now();
        run_batch(method, fixture, model);
        durations.push(started.elapsed().as_nanos() / OPERATIONS_PER_BATCH as u128);
    }
    let total_ns = total.elapsed().as_nanos();
    durations.sort_unstable();
    Round {
        batch_ns_per_operation: durations,
        total_ns,
    }
}

fn run_batch(method: Method, fixture: &str, model: &SessionModel) {
    for _ in 0..OPERATIONS_PER_BATCH {
        match method {
            Method::ParseCanonical => {
                let parsed = parse_session_json(black_box(fixture)).expect("fixture parses");
                black_box(canonical_session_json(&parsed).expect("fixture canonicalizes"));
            }
            Method::Compile => {
                black_box(
                    compile_session(black_box(model), unlimited_caps()).expect("fixture compiles"),
                );
            }
        }
    }
}

fn json_record(
    method: Method,
    round: u8,
    record: &Round,
    fixture: &str,
    fixture_sha256: &str,
    counts: FixtureCounts,
    metadata: &Metadata,
) -> String {
    format!(
        concat!(
            "{{\"schema_version\":1,\"benchmark_id\":\"{}\",\"round\":{},\"rounds\":2,",
            "\"timestamp_epoch_seconds\":{},\"cpu_model\":\"{}\",",
            "\"architecture\":\"{}\",\"physical_cores\":\"{}\",",
            "\"logical_cores\":\"{}\",\"os\":\"{}\",\"kernel\":\"{}\",",
            "\"power_source\":\"{}\",\"governor_or_power_mode\":\"{}\",",
            "\"rustc_version\":\"{}\",\"llvm_version\":\"{}\",",
            "\"cargo_profile\":\"release\",\"opt_level\":\"{}\",",
            "\"lto\":\"{}\",\"codegen_units\":\"{}\",",
            "\"target_triple\":\"{}\",\"target_cpu\":\"{}\",",
            "\"compile_target_features\":\"{}\",\"runtime_or_browser\":\"{}\",",
            "\"sample_rate_hz\":48000,\"quantum_frames\":128,",
            "\"fixture_path\":\"generated:canonical-v1-256-tracks\",",
            "\"fixture_sha256\":\"{}\",\"fixture_size_bytes\":{},",
            "\"fixture_counts\":{{\"sources\":{},\"tracks\":{},\"submixes\":{},",
            "\"outputs\":{},\"routes\":{},\"automation_programs\":{},",
            "\"effects\":{},\"effect_parameters\":{},\"automation_segments\":{}}},",
            "\"warmup_batches\":64,\"measured_batches\":512,",
            "\"operations_per_batch\":8,\"total_operations\":4096,",
            "\"total_duration_ns\":{},\"timer\":\"std::time::Instant\",",
            "\"unit\":\"ns/operation\",",
            "\"percentile_method\":\"nearest-rank over 512 batch durations divided by 8 operations\",",
            "\"p50\":{},\"p95\":{},\"p99\":{},\"p99_9\":{},",
            "\"min\":{},\"max\":{},",
            "\"background_load_note\":\"{}\",\"metadata_incomplete\":{},",
            "\"missing_metadata\":{},\"descriptive_only\":true,",
            "\"decision_threshold\":null}}"
        ),
        method.id(),
        round,
        metadata.timestamp_epoch_seconds,
        escape(&metadata.cpu_model),
        env::consts::ARCH,
        escape(&metadata.physical_cores),
        escape(&metadata.logical_cores),
        env::consts::OS,
        escape(&metadata.kernel),
        escape(&metadata.power_source),
        escape(&metadata.governor_or_power_mode),
        escape(&metadata.rustc_version),
        escape(&metadata.llvm_version),
        escape(&metadata.opt_level),
        escape(&metadata.lto),
        escape(&metadata.codegen_units),
        escape(&metadata.target_triple),
        escape(&metadata.target_cpu),
        escape(&metadata.compile_target_features),
        escape(&metadata.runtime_or_browser),
        fixture_sha256,
        fixture.len(),
        counts.sources,
        counts.tracks,
        counts.submixes,
        counts.outputs,
        counts.routes,
        counts.automation_programs,
        counts.effects,
        counts.effect_parameters,
        counts.automation_segments,
        record.total_ns,
        record.percentile(500),
        record.percentile(950),
        record.percentile(990),
        record.percentile(999),
        record.minimum(),
        record.maximum(),
        escape(&metadata.background_load_note),
        !metadata.missing_metadata.is_empty(),
        json_string_array(&metadata.missing_metadata),
    )
}

struct Metadata {
    timestamp_epoch_seconds: u64,
    cpu_model: String,
    physical_cores: String,
    logical_cores: String,
    kernel: String,
    power_source: String,
    governor_or_power_mode: String,
    rustc_version: String,
    llvm_version: String,
    opt_level: String,
    lto: String,
    codegen_units: String,
    target_triple: String,
    target_cpu: String,
    compile_target_features: String,
    runtime_or_browser: String,
    background_load_note: String,
    missing_metadata: Vec<String>,
}

impl Metadata {
    fn gather() -> Self {
        let compiler_verbose = command(&["rustc", "-Vv"]);
        let cpu_model = fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|text| {
                text.lines().find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    (name.trim() == "model name").then(|| value.trim().to_owned())
                })
            })
            .unwrap_or_else(|| "unknown".to_owned());
        let physical_cores = physical_core_count();
        let logical_cores = std::thread::available_parallelism()
            .map(|value| value.get().to_string())
            .unwrap_or_else(|_| "unknown".to_owned());
        let kernel = command(&["uname", "-r"]);
        let power_source = variable("MISO_ENGINE_BENCH_POWER_SOURCE");
        let governor_or_power_mode =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
                .map(|text| text.trim().to_owned())
                .unwrap_or_else(|_| variable("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"));
        let rustc_version = command(&["rustc", "-V"]);
        let llvm_version = field(&compiler_verbose, "LLVM version: ");
        let target_triple = field(&compiler_verbose, "host: ");
        let opt_level = variable("MISO_ENGINE_BENCH_OPT_LEVEL");
        let lto = variable("MISO_ENGINE_BENCH_LTO");
        let codegen_units = variable("MISO_ENGINE_BENCH_CODEGEN_UNITS");
        let target_cpu = variable("MISO_ENGINE_BENCH_TARGET_CPU");
        let compile_target_features = variable("MISO_ENGINE_BENCH_TARGET_FEATURES");
        let runtime_or_browser = variable("MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER");
        let background_load_note = variable("MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE");
        let fields = [
            ("cpu_model", &cpu_model),
            ("physical_cores", &physical_cores),
            ("logical_cores", &logical_cores),
            ("kernel", &kernel),
            ("power_source", &power_source),
            ("governor_or_power_mode", &governor_or_power_mode),
            ("rustc_version", &rustc_version),
            ("llvm_version", &llvm_version),
            ("opt_level", &opt_level),
            ("lto", &lto),
            ("codegen_units", &codegen_units),
            ("target_triple", &target_triple),
            ("target_cpu", &target_cpu),
            ("compile_target_features", &compile_target_features),
            ("runtime_or_browser", &runtime_or_browser),
            ("background_load_note", &background_load_note),
        ];
        let missing_metadata = fields
            .iter()
            .filter(|(_, value)| value.as_str() == "unknown")
            .map(|(name, _)| (*name).to_owned())
            .collect();
        Self {
            timestamp_epoch_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("wall clock follows Unix epoch")
                .as_secs(),
            cpu_model,
            physical_cores,
            logical_cores,
            kernel,
            power_source,
            governor_or_power_mode,
            rustc_version,
            llvm_version,
            opt_level,
            lto,
            codegen_units,
            target_triple,
            target_cpu,
            compile_target_features,
            runtime_or_browser,
            background_load_note,
            missing_metadata,
        }
    }
}

fn variable(name: &str) -> String {
    bench_support::metadata::Metadata::gather()
        .var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command(args: &[&str]) -> String {
    let Some((program, arguments)) = args.split_first() else {
        return "unknown".to_owned();
    };
    Command::new(program)
        .args(arguments)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_owned())
        .filter(|output| !output.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn field(text: &str, prefix: &str) -> String {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix).map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn json_string_array(values: &[String]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

const fn unlimited_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FixtureCounts, TRACK_COUNT, percentile_nearest_rank, representative_fixture, sha256_hex,
    };
    use session::{canonical_session_json, parse_session_json};

    #[test]
    fn representative_fixture_has_the_frozen_workload_and_stable_bytes() {
        let (fixture, model) = representative_fixture();
        assert_eq!(model.sample_rate_hz, 48_000);
        assert_eq!(model.quantum_frames, 128);
        assert_eq!(
            FixtureCounts::from_session(&model),
            FixtureCounts {
                sources: 1,
                tracks: TRACK_COUNT,
                submixes: 0,
                outputs: 1,
                routes: TRACK_COUNT,
                automation_programs: TRACK_COUNT,
                effects: TRACK_COUNT,
                effect_parameters: TRACK_COUNT,
                automation_segments: TRACK_COUNT,
            }
        );
        let reparsed = parse_session_json(&fixture).expect("generated fixture reparses");
        assert_eq!(
            fixture,
            canonical_session_json(&reparsed).expect("canonical bytes are stable")
        );
    }

    #[test]
    fn percentile_and_sha256_are_contract_stable() {
        let values = (1..=1_000).collect::<Vec<u128>>();
        assert_eq!(percentile_nearest_rank(&values, 500), 500);
        assert_eq!(percentile_nearest_rank(&values, 999), 999);
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
