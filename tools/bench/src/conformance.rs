//! Bounded descriptive benchmark for conformance primitives.

use bench_support::json::escape;
use bench_support::stats::per_mille as percentile_nearest_rank;
use bench_support::sysinfo::HostToolchainFacts;
use std::{
    env,
    hint::black_box,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use conformance::{ComparisonTolerance, PcmFixture, PlanarBlock, SampleRateHz, compare_f32_to_f64};

const WARMUP_BATCHES: usize = 512;
const TIMED_BATCHES: usize = 4_096;
const FRAMES: usize = 4_096;
const CHANNELS: usize = 2;
const SAMPLE_COUNT: usize = CHANNELS * FRAMES;

pub(crate) fn main() {
    let rounds = parse_rounds();
    let samples = generated_samples();
    let reference = samples.iter().copied().map(f64::from).collect::<Vec<_>>();
    let fixture = PcmFixture::encode(
        SampleRateHz(48_000),
        CHANNELS as u16,
        FRAMES as u64,
        &samples,
    )
    .expect("fixture encode");
    let fixture_crc = PcmFixture::parse(&fixture, Default::default())
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
                PcmFixture::parse(black_box(fixture), Default::default()).expect("fixture parse"),
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
        let facts = HostToolchainFacts::gather();
        Self::from_facts(
            facts,
            git_commit,
            workspace_dirty,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock is after Unix epoch")
                .as_secs(),
        )
    }

    fn from_facts(
        facts: HostToolchainFacts,
        git_commit: String,
        workspace_dirty: String,
        timestamp_epoch_seconds: u64,
    ) -> Self {
        let cpu_model = facts
            .raw_cpuinfo
            .as_deref()
            .and_then(|text| {
                text.lines()
                    .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
            })
            .unwrap_or_else(|| "unknown".to_owned());
        let HostToolchainFacts {
            physical_cores,
            logical_cores,
            kernel,
            power_source,
            governor_or_power_mode,
            rustc_version: compiler,
            llvm_version,
            opt_level,
            lto,
            codegen_units,
            target_triple,
            target_cpu,
            compile_target_features,
            background_load_note,
            ..
        } = facts;
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
            timestamp_epoch_seconds,
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

fn command(args: &[&str]) -> String {
    command_allow_empty(args)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command_allow_empty(args: &[&str]) -> Option<String> {
    let (program, rest) = args.split_first()?;
    bench_support::sysinfo::command_output(program, rest)
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
        _ => panic!("usage: conformance_bench [--rounds 1|2]"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Metadata, Round, escape, json_record, percentile_nearest_rank};
    use bench_support::sysinfo::HostToolchainFacts;
    use std::env;

    #[test]
    fn percentile_is_nearest_rank_and_escape_is_json_safe() {
        let values = (1..=1_000).collect::<Vec<u128>>();
        assert_eq!(percentile_nearest_rank(&values, 500), 500);
        assert_eq!(percentile_nearest_rank(&values, 999), 999);
        assert_eq!(escape("\"\\\n\u{0001}"), "\\\"\\\\\\n\\u0001");
    }

    #[test]
    fn shared_host_toolchain_facts_preserve_conformance_projection() {
        let facts = HostToolchainFacts {
            raw_cpuinfo: Some("model name : Session CPU\nmodel name\t: Conformance CPU\n".into()),
            physical_cores: "8".into(),
            logical_cores: "unknown".into(),
            kernel: "Linux-test".into(),
            power_source: "AC".into(),
            governor_or_power_mode: String::new(),
            rustc_version: "rustc test 1.0".into(),
            llvm_version: "LLVM test".into(),
            target_triple: "x86_64-test".into(),
            opt_level: "2".into(),
            lto: "thin".into(),
            codegen_units: "16".into(),
            target_cpu: "native".into(),
            compile_target_features: "avx2-µ".into(),
            background_load_note: "quiet".into(),
        };
        let metadata =
            Metadata::from_facts(facts, "deadbeef".into(), "false".into(), 1_700_000_000);
        assert_eq!(metadata.cpu_model, "Conformance CPU");
        assert_eq!(metadata.compiler, "rustc test 1.0");
        assert_eq!(metadata.governor_or_power_mode, "");
        assert_eq!(metadata.missing_metadata, vec!["logical_cores"]);
        assert_eq!(metadata.workspace_dirty, "false");
        let record = json_record(
            "conformance_test",
            1,
            2,
            &Round {
                durations: vec![10, 20],
                total: 30,
            },
            0x1234,
            &metadata,
        );
        let expected_metadata = format!(
            "\"timestamp_epoch_seconds\":1700000000,\"git_commit\":\"deadbeef\",\"workspace_dirty\":\"false\",\"cpu_model\":\"Conformance CPU\",\"architecture\":\"{}\",\"physical_cores\":\"8\",\"logical_cores\":\"unknown\",\"os\":\"{}\",\"kernel\":\"Linux-test\",\"power_source\":\"AC\",\"governor_or_power_mode\":\"\",\"compiler\":\"rustc test 1.0\",\"llvm_version\":\"LLVM test\",\"cargo_profile\":\"release\",\"opt_level\":\"2\",\"lto\":\"thin\",\"codegen_units\":\"16\",\"target_triple\":\"x86_64-test\",\"target_cpu\":\"native\",\"compile_target_features\":\"avx2-µ\"",
            env::consts::ARCH,
            env::consts::OS,
        );
        assert!(record.contains(&expected_metadata));
        assert!(record.contains("\"background_load_note\":\"quiet\",\"metadata_incomplete\":true,\"missing_metadata\":[\"logical_cores\"]"));
    }

    #[cfg(unix)]
    #[test]
    fn metadata_command_projection_preserves_empty_status_and_unknown_failures() {
        assert_eq!(
            super::command_allow_empty(&["/bin/sh", "-c", "true"]),
            Some(String::new())
        );
        assert_eq!(
            super::command_allow_empty(&["/bin/sh", "-c", "printf ' \\n\\t'"]),
            Some(String::new())
        );
        assert_eq!(super::command(&["/bin/sh", "-c", "true"]), "unknown");
        assert_eq!(
            super::command(&["/bin/sh", "-c", "printf '  dirty  \\n'"]),
            "dirty"
        );
        assert_eq!(
            super::command_allow_empty(&["/bin/sh", "-c", "printf plausible; exit 7"]),
            None
        );
        assert_eq!(
            super::command_allow_empty(&["/definitely/missing/metadata-command"]),
            None
        );
        assert_eq!(
            super::command_allow_empty(&["/bin/sh", "-c", r"printf '\377'"]),
            None
        );
    }
}
