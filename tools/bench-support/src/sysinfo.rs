//! Host and toolchain facts shared by benchmark subjects.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::Command;

/// The common host and toolchain facts gathered for one benchmark subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostToolchainFacts {
    /// Raw readable `/proc/cpuinfo` text, retained for subject-specific parsing.
    pub raw_cpuinfo: Option<String>,
    /// Number of distinct physical core/socket pairs.
    pub physical_cores: String,
    /// Number of logical cores reported by the standard library.
    pub logical_cores: String,
    /// Kernel release.
    pub kernel: String,
    /// Power source metadata.
    pub power_source: String,
    /// CPU governor or power mode.
    pub governor_or_power_mode: String,
    /// `rustc -V` output.
    pub rustc_version: String,
    /// LLVM version parsed from `rustc -Vv`.
    pub llvm_version: String,
    /// Host target parsed from `rustc -Vv`.
    pub target_triple: String,
    /// Benchmark optimization level.
    pub opt_level: String,
    /// Benchmark LTO setting.
    pub lto: String,
    /// Benchmark codegen-units setting.
    pub codegen_units: String,
    /// Benchmark target CPU setting.
    pub target_cpu: String,
    /// Benchmark compile target features.
    pub compile_target_features: String,
    /// Benchmark background-load note.
    pub background_load_note: String,
}

impl HostToolchainFacts {
    /// Gather one control-plane snapshot of the common host and toolchain facts.
    #[must_use]
    pub fn gather() -> Self {
        Self::from_sources(Sources::live())
    }

    fn from_sources(sources: Sources) -> Self {
        let compiler_verbose = command_value(sources.rustc_verbose);
        let raw_cpuinfo = utf8_file(sources.cpuinfo);
        let physical_cores = sources
            .lscpu
            .and_then(command_stdout)
            .map_or_else(|| "unknown".to_owned(), |text| count_cores(&text));
        let logical_cores = sources
            .logical_cores
            .map_or_else(|| "unknown".to_owned(), |value| value.to_string());
        let governor_or_power_mode = sources
            .governor
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .map(|text| text.trim().to_owned())
            .unwrap_or_else(|| source_variable(&sources.environment, GOVERNOR_VARIABLE));

        Self {
            raw_cpuinfo,
            physical_cores,
            logical_cores,
            kernel: command_value(sources.kernel),
            power_source: source_variable(&sources.environment, "MISO_ENGINE_BENCH_POWER_SOURCE"),
            governor_or_power_mode,
            rustc_version: command_value(sources.rustc_version),
            llvm_version: field(&compiler_verbose, "LLVM version: "),
            target_triple: field(&compiler_verbose, "host: "),
            opt_level: source_variable(&sources.environment, "MISO_ENGINE_BENCH_OPT_LEVEL"),
            lto: source_variable(&sources.environment, "MISO_ENGINE_BENCH_LTO"),
            codegen_units: source_variable(&sources.environment, "MISO_ENGINE_BENCH_CODEGEN_UNITS"),
            target_cpu: source_variable(&sources.environment, "MISO_ENGINE_BENCH_TARGET_CPU"),
            compile_target_features: source_variable(
                &sources.environment,
                "MISO_ENGINE_BENCH_TARGET_FEATURES",
            ),
            background_load_note: source_variable(
                &sources.environment,
                "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
            ),
        }
    }
}

const GOVERNOR_VARIABLE: &str = "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE";

const COMMON_VARIABLES: [&str; 8] = [
    "MISO_ENGINE_BENCH_POWER_SOURCE",
    GOVERNOR_VARIABLE,
    "MISO_ENGINE_BENCH_OPT_LEVEL",
    "MISO_ENGINE_BENCH_LTO",
    "MISO_ENGINE_BENCH_CODEGEN_UNITS",
    "MISO_ENGINE_BENCH_TARGET_CPU",
    "MISO_ENGINE_BENCH_TARGET_FEATURES",
    "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
];

/// The number of distinct physical cores this host reports, or `"unknown"` when unavailable.
#[must_use]
pub fn physical_core_count() -> String {
    Command::new("lscpu")
        .arg("-p=CORE,SOCKET")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map_or_else(|| "unknown".to_owned(), |text| count_cores(&text))
}

fn count_cores(lscpu_stdout: &str) -> String {
    let trimmed = lscpu_stdout.trim();
    if trimmed.is_empty() {
        return "unknown".to_owned();
    }
    let cores = trimmed
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

fn field(text: &str, prefix: &str) -> String {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix).map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command_value(command: Option<CommandOutput>) -> String {
    command
        .and_then(command_stdout)
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn command_stdout(command: CommandOutput) -> Option<String> {
    command
        .successful
        .then(|| String::from_utf8(command.stdout).ok())
        .flatten()
}

fn utf8_file(file: Option<Vec<u8>>) -> Option<String> {
    file.and_then(|bytes| String::from_utf8(bytes).ok())
}

fn source_variable(environment: &BTreeMap<&'static str, SourceValue>, name: &str) -> String {
    match environment.get(name) {
        Some(SourceValue::Value(value)) if !value.is_empty() => value.clone(),
        _ => "unknown".to_owned(),
    }
}

struct Sources {
    cpuinfo: Option<Vec<u8>>,
    governor: Option<Vec<u8>>,
    lscpu: Option<CommandOutput>,
    rustc_version: Option<CommandOutput>,
    rustc_verbose: Option<CommandOutput>,
    kernel: Option<CommandOutput>,
    logical_cores: Option<usize>,
    environment: BTreeMap<&'static str, SourceValue>,
}

impl Sources {
    fn live() -> Self {
        let environment = bench_environment();
        Self {
            cpuinfo: fs::read("/proc/cpuinfo").ok(),
            governor: fs::read("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").ok(),
            lscpu: run_command("lscpu", &["-p=CORE,SOCKET"]),
            rustc_version: run_command("rustc", &["-V"]),
            rustc_verbose: run_command("rustc", &["-Vv"]),
            kernel: run_command("uname", &["-r"]),
            logical_cores: std::thread::available_parallelism()
                .ok()
                .map(|value| value.get()),
            environment,
        }
    }
}

fn bench_environment() -> BTreeMap<&'static str, SourceValue> {
    let snapshot = crate::metadata::Metadata::gather();
    COMMON_VARIABLES
        .into_iter()
        .map(|name| (name, SourceValue::from_snapshot(snapshot, name)))
        .collect()
}

fn run_command(program: &str, args: &[&str]) -> Option<CommandOutput> {
    let output = Command::new(program).args(args).output().ok()?;
    Some(CommandOutput {
        successful: output.status.success(),
        stdout: output.stdout,
    })
}

struct CommandOutput {
    successful: bool,
    stdout: Vec<u8>,
}

enum SourceValue {
    Value(String),
    Unavailable,
}

impl SourceValue {
    fn from_snapshot(snapshot: &crate::metadata::Metadata, name: &'static str) -> Self {
        snapshot.var(name).map_or(Self::Unavailable, Self::Value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommandOutput, HostToolchainFacts, SourceValue, Sources, count_cores, physical_core_count,
    };
    use std::collections::BTreeMap;

    fn env(
        values: impl IntoIterator<Item = (&'static str, SourceValue)>,
    ) -> BTreeMap<&'static str, SourceValue> {
        values.into_iter().collect()
    }

    fn command(successful: bool, stdout: &[u8]) -> Option<CommandOutput> {
        Some(CommandOutput {
            successful,
            stdout: stdout.to_vec(),
        })
    }

    fn populated_sources() -> Sources {
        Sources {
            cpuinfo: Some(b"model name\t: Test CPU\n".to_vec()),
            governor: Some(b"\n".to_vec()),
            lscpu: command(true, b"# comment\n0,0\n1,0\n0,0\n"),
            rustc_version: command(true, b"rustc test 1.0\n"),
            rustc_verbose: command(true, b"LLVM version: LLVM test\nhost: test-target\n"),
            kernel: command(true, b"test-kernel\n"),
            logical_cores: Some(12),
            environment: env([
                (
                    "MISO_ENGINE_BENCH_POWER_SOURCE",
                    SourceValue::Value("AC".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
                    SourceValue::Value("fallback".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_OPT_LEVEL",
                    SourceValue::Value("2".into()),
                ),
                ("MISO_ENGINE_BENCH_LTO", SourceValue::Value("thin".into())),
                (
                    "MISO_ENGINE_BENCH_CODEGEN_UNITS",
                    SourceValue::Value("12".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_TARGET_CPU",
                    SourceValue::Value("native".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_TARGET_FEATURES",
                    SourceValue::Value("avx2-µ".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
                    SourceValue::Value("quiet".into()),
                ),
            ]),
        }
    }

    #[test]
    fn populated_injected_sources_are_projected_once() {
        let facts = HostToolchainFacts::from_sources(populated_sources());
        assert_eq!(
            facts.raw_cpuinfo.as_deref(),
            Some("model name\t: Test CPU\n")
        );
        assert_eq!(facts.physical_cores, "2");
        assert_eq!(facts.logical_cores, "12");
        assert_eq!(facts.governor_or_power_mode, "");
        assert_eq!(facts.target_triple, "test-target");
        assert_eq!(facts.compile_target_features, "avx2-µ");
    }

    #[test]
    fn unavailable_commands_files_parallelism_and_environment_use_unknown() {
        let facts = HostToolchainFacts::from_sources(Sources {
            cpuinfo: Some(vec![0xff]),
            governor: Some(vec![0xff]),
            lscpu: command(false, b"ignored"),
            rustc_version: command(true, b""),
            rustc_verbose: command(true, b"LLVM version: \xff"),
            kernel: None,
            logical_cores: None,
            environment: env([
                ("MISO_ENGINE_BENCH_POWER_SOURCE", SourceValue::Unavailable),
                (
                    "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
                    SourceValue::Value("".into()),
                ),
                ("MISO_ENGINE_BENCH_OPT_LEVEL", SourceValue::Value("".into())),
                ("MISO_ENGINE_BENCH_LTO", SourceValue::Unavailable),
                (
                    "MISO_ENGINE_BENCH_CODEGEN_UNITS",
                    SourceValue::Value("valid".into()),
                ),
                ("MISO_ENGINE_BENCH_TARGET_CPU", SourceValue::Unavailable),
                (
                    "MISO_ENGINE_BENCH_TARGET_FEATURES",
                    SourceValue::Value("unicode-µ".into()),
                ),
                (
                    "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
                    SourceValue::Value("note".into()),
                ),
            ]),
        });
        assert_eq!(facts.raw_cpuinfo, None);
        assert_eq!(facts.physical_cores, "unknown");
        assert_eq!(facts.logical_cores, "unknown");
        assert_eq!(facts.kernel, "unknown");
        assert_eq!(facts.governor_or_power_mode, "unknown");
        assert_eq!(facts.rustc_version, "unknown");
        assert_eq!(facts.llvm_version, "unknown");
        assert_eq!(facts.power_source, "unknown");
        assert_eq!(facts.opt_level, "unknown");
        assert_eq!(facts.target_cpu, "unknown");
        assert_eq!(facts.compile_target_features, "unicode-µ");
    }

    #[test]
    fn unsuccessful_and_non_utf8_commands_are_unknown() {
        let mut sources = populated_sources();
        sources.lscpu = command(false, b"0,0");
        sources.rustc_version = command(false, b"rustc");
        sources.rustc_verbose = command(true, &[0xff]);
        sources.kernel = command(true, &[0xff]);
        let facts = HostToolchainFacts::from_sources(sources);
        assert_eq!(facts.physical_cores, "unknown");
        assert_eq!(facts.rustc_version, "unknown");
        assert_eq!(facts.llvm_version, "unknown");
        assert_eq!(facts.target_triple, "unknown");
        assert_eq!(facts.kernel, "unknown");
    }

    #[test]
    fn governor_file_failure_uses_fallback_but_empty_success_is_retained() {
        let mut sources = populated_sources();
        sources.governor = None;
        let facts = HostToolchainFacts::from_sources(sources);
        assert_eq!(facts.governor_or_power_mode, "fallback");
        let facts = HostToolchainFacts::from_sources(populated_sources());
        assert_eq!(facts.governor_or_power_mode, "");
    }

    #[test]
    fn missing_cpu_and_governor_files_use_their_unavailable_fallbacks() {
        let mut sources = populated_sources();
        sources.cpuinfo = None;
        sources.governor = None;
        let facts = HostToolchainFacts::from_sources(sources);
        assert_eq!(facts.raw_cpuinfo, None);
        assert_eq!(facts.governor_or_power_mode, "fallback");
    }

    #[test]
    fn distinct_core_socket_pairs_are_counted_once_each() {
        assert_eq!(count_cores("0,0\n1,0\n0,0\n"), "2");
    }

    #[test]
    fn empty_output_is_unknown() {
        assert_eq!(count_cores(""), "unknown");
    }

    #[test]
    fn whitespace_only_output_is_unknown() {
        assert_eq!(count_cores("  \n"), "unknown");
    }

    #[test]
    fn header_only_output_is_unknown() {
        assert_eq!(count_cores("# header only\n"), "unknown");
    }

    #[test]
    fn returns_a_decimal_count_or_the_unknown_sentinel() {
        let count = physical_core_count();
        assert!(count == "unknown" || count.parse::<usize>().is_ok());
    }
}
