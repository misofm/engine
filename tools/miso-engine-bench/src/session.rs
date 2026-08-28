//! Fixed-work descriptive benchmark for issue-004 session control-plane operations.

use miso_engine_bench_support::json::escape;
use miso_engine_bench_support::stats::per_mille as percentile_nearest_rank;
use std::{
    collections::BTreeSet,
    env,
    fmt::Write as _,
    fs,
    hint::black_box,
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use miso_engine_session::{
    CompileCaps, RouteSource, SessionToml, StableId, canonical_session_toml, compile_session,
    parse_session_toml,
};

const CANONICAL_EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
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
    fn from_session(session: &SessionToml) -> Self {
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

fn representative_fixture() -> (String, SessionToml) {
    let mut model = parse_session_toml(CANONICAL_EXAMPLE).expect("canonical seed fixture parses");
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
            tap: miso_engine_session::SendTap::PostMatrix,
        };
        model.routes.push(route);

        let mut automation = automation_template.clone();
        automation.id = stable_id(&format!("automation-{index:03}"));
        automation.target.entity_id = track_id;
        model.automation.push(automation);
    }

    let fixture = canonical_session_toml(&model).expect("representative fixture canonicalizes");
    let reparsed = parse_session_toml(&fixture).expect("representative fixture reparses");
    assert_eq!(FixtureCounts::from_session(&reparsed).tracks, TRACK_COUNT);
    (fixture, reparsed)
}

fn stable_id(value: &str) -> StableId {
    StableId::parse(value).expect("generated benchmark ID is schema-valid")
}

fn run_round(method: Method, fixture: &str, model: &SessionToml) -> Round {
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

fn run_batch(method: Method, fixture: &str, model: &SessionToml) {
    for _ in 0..OPERATIONS_PER_BATCH {
        match method {
            Method::ParseCanonical => {
                let parsed = parse_session_toml(black_box(fixture)).expect("fixture parses");
                black_box(canonical_session_toml(&parsed).expect("fixture canonicalizes"));
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
            "{{\"schema_version\":2,\"benchmark_id\":\"{}\",\"round\":{},\"rounds\":2,",
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
    miso_engine_bench_support::metadata::Metadata::gather()
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

fn physical_core_count() -> String {
    let output = command(&["lscpu", "-p=CORE,SOCKET"]);
    if output == "unknown" {
        return output;
    }
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

fn sha256_hex(input: &[u8]) -> String {
    const INITIAL: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];
    const K: [u32; 64] = [
        0x428a_2f98,
        0x7137_4491,
        0xb5c0_fbcf,
        0xe9b5_dba5,
        0x3956_c25b,
        0x59f1_11f1,
        0x923f_82a4,
        0xab1c_5ed5,
        0xd807_aa98,
        0x1283_5b01,
        0x2431_85be,
        0x550c_7dc3,
        0x72be_5d74,
        0x80de_b1fe,
        0x9bdc_06a7,
        0xc19b_f174,
        0xe49b_69c1,
        0xefbe_4786,
        0x0fc1_9dc6,
        0x240c_a1cc,
        0x2de9_2c6f,
        0x4a74_84aa,
        0x5cb0_a9dc,
        0x76f9_88da,
        0x983e_5152,
        0xa831_c66d,
        0xb003_27c8,
        0xbf59_7fc7,
        0xc6e0_0bf3,
        0xd5a7_9147,
        0x06ca_6351,
        0x1429_2967,
        0x27b7_0a85,
        0x2e1b_2138,
        0x4d2c_6dfc,
        0x5338_0d13,
        0x650a_7354,
        0x766a_0abb,
        0x81c2_c92e,
        0x9272_2c85,
        0xa2bf_e8a1,
        0xa81a_664b,
        0xc24b_8b70,
        0xc76c_51a3,
        0xd192_e819,
        0xd699_0624,
        0xf40e_3585,
        0x106a_a070,
        0x19a4_c116,
        0x1e37_6c08,
        0x2748_774c,
        0x34b0_bcb5,
        0x391c_0cb3,
        0x4ed8_aa4a,
        0x5b9c_ca4f,
        0x682e_6ff3,
        0x748f_82ee,
        0x78a5_636f,
        0x84c8_7814,
        0x8cc7_0208,
        0x90be_fffa,
        0xa450_6ceb,
        0xbef9_a3f7,
        0xc671_78f2,
    ];

    let bit_len = u64::try_from(input.len())
        .expect("fixture length fits u64")
        .checked_mul(8)
        .expect("fixture bit length fits u64");
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    let mut state = INITIAL;
    for chunk in padded.chunks_exact(64) {
        let mut schedule = [0_u32; 64];
        for (word, bytes) in schedule[..16].iter_mut().zip(chunk.chunks_exact(4)) {
            *word = u32::from_be_bytes(bytes.try_into().expect("four-byte word"));
        }
        for index in 16..64 {
            let s0 = schedule[index - 15].rotate_right(7)
                ^ schedule[index - 15].rotate_right(18)
                ^ (schedule[index - 15] >> 3);
            let s1 = schedule[index - 2].rotate_right(17)
                ^ schedule[index - 2].rotate_right(19)
                ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let temporary1 = h
                .wrapping_add(sum1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(schedule[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temporary2 = sum0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temporary1);
            d = c;
            c = b;
            b = a;
            a = temporary1.wrapping_add(temporary2);
        }
        for (word, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *word = word.wrapping_add(value);
        }
    }

    let mut output = String::with_capacity(64);
    for word in state {
        write!(&mut output, "{word:08x}").expect("writing to String cannot fail");
    }
    output
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
    use miso_engine_session::{canonical_session_toml, parse_session_toml};

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
        let reparsed = parse_session_toml(&fixture).expect("generated fixture reparses");
        assert_eq!(
            fixture,
            canonical_session_toml(&reparsed).expect("canonical bytes are stable")
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
