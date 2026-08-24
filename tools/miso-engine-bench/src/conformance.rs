//! Bounded descriptive benchmark for conformance primitives.

use miso_engine_bench_support::json::escape;
use miso_engine_bench_support::stats::per_mille as percentile_nearest_rank;
use std::{
    collections::BTreeSet,
    env, fs,
    hint::black_box,
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use miso_engine_conformance::{
    ComparisonTolerance, PcmFixtureV1, PlanarBlock, SampleRateHz, compare_f32_to_f64,
};

const WARMUP_BATCHES: usize = 512;
const TIMED_BATCHES: usize = 4_096;
const FRAMES: usize = 4_096;
const CHANNELS: usize = 2;
const SAMPLE_COUNT: usize = CHANNELS * FRAMES;

pub(crate) fn main() {
    let rounds = parse_rounds();
    let samples = generated_samples();
    let reference = samples.iter().copied().map(f64::from).collect::<Vec<_>>();
    let fixture = PcmFixtureV1::encode(
        SampleRateHz(48_000),
        CHANNELS as u16,
        FRAMES as u64,
        &samples,
    )
    .expect("fixture encode");
    let fixture_crc = PcmFixtureV1::parse(&fixture, Default::default())
        .expect("fixture parse")
        .checksum();
    let metadata = Metadata::gather();

    for (name, method) in [
        ("fixture_decode_crc32c_2x4096", Method::Decode),
        ("compare_f32_f64_2x4096", Method::Compare),
    ] {
        for round in 1..=rounds {
            let record = run_round(method, &samples, &reference, &fixture);
            println!(
                "{}",
                json_record(name, round, rounds, &record, fixture_crc, &metadata)
            );
        }
    }
}

#[derive(Clone, Copy)]
enum Method {
    Decode,
    Compare,
}

struct Round {
    durations: Vec<u128>,
    total: u128,
}

fn run_round(method: Method, samples: &[f32], reference: &[f64], fixture: &[u8]) -> Round {
    for _ in 0..WARMUP_BATCHES {
        execute(method, samples, reference, fixture);
    }
    let mut durations = Vec::with_capacity(TIMED_BATCHES);
    let all = Instant::now();
    for _ in 0..TIMED_BATCHES {
        let started = Instant::now();
        execute(method, samples, reference, fixture);
        durations.push(started.elapsed().as_nanos());
    }
    Round {
        durations,
        total: all.elapsed().as_nanos(),
    }
}

fn execute(method: Method, samples: &[f32], reference: &[f64], fixture: &[u8]) {
    match method {
        Method::Decode => {
            black_box(
                PcmFixtureV1::parse(black_box(fixture), Default::default()).expect("fixture parse"),
            );
        }
        Method::Compare => {
            let actual =
                PlanarBlock::try_new(SampleRateHz(48_000), CHANNELS, FRAMES, black_box(samples))
                    .expect("actual block");
            let expected =
                PlanarBlock::try_new(SampleRateHz(48_000), CHANNELS, FRAMES, black_box(reference))
                    .expect("reference block");
            black_box(
                compare_f32_to_f64(
                    actual,
                    expected,
                    ComparisonTolerance {
                        absolute: 1e-6,
                        relative: 2e-5,
                        relative_floor: 1e-12,
                    },
                )
                .expect("compare"),
            );
        }
    }
}

fn json_record(
    name: &str,
    round: u8,
    rounds: u8,
    record: &Round,
    fixture_crc: u32,
    metadata: &Metadata,
) -> String {
    let mut values = record.durations.clone();
    values.sort_unstable();
    let per_sample = |value: u128| value as f64 / SAMPLE_COUNT as f64;
    format!(
        concat!(
            "{{\"schema_version\":1,\"benchmark_id\":\"{}\",\"round\":{},\"rounds\":{},",
            "\"timestamp_epoch_seconds\":{},\"git_commit\":\"{}\",\"workspace_dirty\":\"{}\",",
            "\"cpu_model\":\"{}\",\"architecture\":\"{}\",\"physical_cores\":\"{}\",",
            "\"logical_cores\":\"{}\",\"os\":\"{}\",\"kernel\":\"{}\",",
            "\"power_source\":\"{}\",\"governor_or_power_mode\":\"{}\",",
            "\"compiler\":\"{}\",\"llvm_version\":\"{}\",\"cargo_profile\":\"release\",",
            "\"opt_level\":\"{}\",\"lto\":\"{}\",\"codegen_units\":\"{}\",",
            "\"target_triple\":\"{}\",\"target_cpu\":\"{}\",\"compile_target_features\":\"{}\",",
            "\"runtime_or_browser\":\"native-cli\",\"sample_rate_hz\":48000,",
            "\"quantum_frames\":4096,\"channels\":2,\"fixture_path\":\"generated:2x4096\",",
            "\"fixture_crc32c\":\"{:08x}\",\"prng_algorithm\":\"splitmix64-v1\",",
            "\"prng_seed\":\"0x4D49534F454E4732\",\"warmup_batches\":512,",
            "\"measured_batches\":4096,\"batch_samples\":8192,\"total_duration_ns\":{},",
            "\"timer\":\"std::time::Instant\",\"unit\":\"ns/sample\",",
            "\"percentile_method\":\"nearest-rank over 4096 batch durations, divided by 8192 samples\",",
            "\"p50\":{:.9},\"p95\":{:.9},\"p99\":{:.9},\"p99_9\":{:.9},",
            "\"min\":{:.9},\"max\":{:.9},\"sample_count\":4096,",
            "\"background_load_note\":\"{}\",\"metadata_incomplete\":{},",
            "\"missing_metadata\":{}}}"
        ),
        name,
        round,
        rounds,
        metadata.timestamp_epoch_seconds,
        escape(&metadata.git_commit),
        escape(&metadata.workspace_dirty),
        escape(&metadata.cpu_model),
        env::consts::ARCH,
        escape(&metadata.physical_cores),
        escape(&metadata.logical_cores),
        env::consts::OS,
        escape(&metadata.kernel),
        escape(&metadata.power_source),
        escape(&metadata.governor_or_power_mode),
        escape(&metadata.compiler),
        escape(&metadata.llvm_version),
        escape(&metadata.opt_level),
        escape(&metadata.lto),
        escape(&metadata.codegen_units),
        escape(&metadata.target_triple),
        escape(&metadata.target_cpu),
        escape(&metadata.compile_target_features),
        fixture_crc,
        record.total,
        per_sample(percentile_nearest_rank(&values, 500)),
        per_sample(percentile_nearest_rank(&values, 950)),
        per_sample(percentile_nearest_rank(&values, 990)),
        per_sample(percentile_nearest_rank(&values, 999)),
        per_sample(values[0]),
        per_sample(values[values.len() - 1]),
        escape(&metadata.background_load_note),
        !metadata.missing_metadata.is_empty(),
        json_string_array(&metadata.missing_metadata),
    )
}

struct Metadata {
    timestamp_epoch_seconds: u64,
    git_commit: String,
    workspace_dirty: String,
    cpu_model: String,
    physical_cores: String,
    logical_cores: String,
    kernel: String,
    power_source: String,
    governor_or_power_mode: String,
    compiler: String,
    llvm_version: String,
    opt_level: String,
    lto: String,
    codegen_units: String,
    target_triple: String,
    target_cpu: String,
    compile_target_features: String,
    background_load_note: String,
    missing_metadata: Vec<String>,
}

impl Metadata {
    fn gather() -> Self {
        let compiler_verbose = command(&["rustc", "-Vv"]);
        let git_commit = command(&["git", "rev-parse", "HEAD"]);
        let workspace_dirty = command_allow_empty(&["git", "status", "--porcelain"]).map_or_else(
            || "unknown".to_owned(),
            |value| {
                if value.is_empty() {
                    "false".to_owned()
                } else {
                    "true".to_owned()
                }
            },
        );
        let cpu_model = fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|text| {
                text.lines()
                    .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
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
        let compiler = command(&["rustc", "-V"]);
        let llvm_version = field(&compiler_verbose, "LLVM version: ");
        let target_triple = field(&compiler_verbose, "host: ");
        let opt_level = variable("MISO_ENGINE_BENCH_OPT_LEVEL");
        let lto = variable("MISO_ENGINE_BENCH_LTO");
        let codegen_units = variable("MISO_ENGINE_BENCH_CODEGEN_UNITS");
        let target_cpu = variable("MISO_ENGINE_BENCH_TARGET_CPU");
        let compile_target_features = variable("MISO_ENGINE_BENCH_TARGET_FEATURES");
        let background_load_note = variable("MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE");
        let fields = [
            ("git_commit", &git_commit),
            ("workspace_dirty", &workspace_dirty),
            ("cpu_model", &cpu_model),
            ("physical_cores", &physical_cores),
            ("logical_cores", &logical_cores),
            ("kernel", &kernel),
            ("power_source", &power_source),
            ("governor_or_power_mode", &governor_or_power_mode),
            ("compiler", &compiler),
            ("llvm_version", &llvm_version),
            ("opt_level", &opt_level),
            ("lto", &lto),
            ("codegen_units", &codegen_units),
            ("target_triple", &target_triple),
            ("target_cpu", &target_cpu),
            ("compile_target_features", &compile_target_features),
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
                .expect("clock is after Unix epoch")
                .as_secs(),
            git_commit,
            workspace_dirty,
            cpu_model,
            physical_cores,
            logical_cores,
            kernel,
            power_source,
            governor_or_power_mode,
            compiler,
            llvm_version,
            opt_level,
            lto,
            codegen_units,
            target_triple,
            target_cpu,
            compile_target_features,
            background_load_note,
            missing_metadata,
        }
    }
}

fn variable(name: &str) -> String {
    miso_engine_bench_support::metadata::Metadata::gather()
        .var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command(args: &[&str]) -> String {
    command_allow_empty(args)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command_allow_empty(args: &[&str]) -> Option<String> {
    let output = Command::new(args[0]).args(&args[1..]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_owned())
}

fn field(text: &str, prefix: &str) -> String {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix).map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn physical_core_count() -> String {
    let Some(output) = command_allow_empty(&["lscpu", "-p=CORE,SOCKET"]) else {
        return "unknown".to_owned();
    };
    let cores = output
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split(',');
            Some((fields.next()?.to_owned(), fields.next()?.to_owned()))
        })
        .collect::<BTreeSet<_>>();
    if cores.is_empty() {
        "unknown".to_owned()
    } else {
        cores.len().to_string()
    }
}

fn json_string_array(values: &[String]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn generated_samples() -> Vec<f32> {
    let mut output = Vec::with_capacity(SAMPLE_COUNT);
    for channel in 0..CHANNELS {
        for frame in 0..FRAMES {
            output.push(
                (core::f32::consts::TAU * (997.0 + channel as f32 * 31.0) * frame as f32
                    / 48_000.0)
                    .sin()
                    * 0.5,
            );
        }
    }
    output
}

fn parse_rounds() -> u8 {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => 2,
        [flag, value] if flag == "--rounds" && value == "1" => 1,
        [flag, value] if flag == "--rounds" && value == "2" => 2,
        _ => panic!("usage: miso_engine_conformance_bench [--rounds 1|2]"),
    }
}

#[cfg(test)]
mod tests {
    use super::{escape, percentile_nearest_rank};

    #[test]
    fn percentile_is_nearest_rank_and_escape_is_json_safe() {
        let values = (1..=1_000).collect::<Vec<u128>>();
        assert_eq!(percentile_nearest_rank(&values, 500), 500);
        assert_eq!(percentile_nearest_rank(&values, 999), 999);
        assert_eq!(escape("\"\\\n\u{0001}"), "\\\"\\\\\\n\\u0001");
    }
}
