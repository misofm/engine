//! Fixed-work bootstrap capability-query benchmark.
//!
//! Its results are descriptive harness evidence only; this is not a render-performance claim.

use miso_engine_lane::Backend;
use std::{
    env,
    fmt::Write as _,
    hint::black_box,
    time::{Duration, Instant},
};

const WARMUP_ITERATIONS: u64 = 10_000;
const ITERATIONS_PER_ROUND: u64 = 1_000_000;

fn main() {
    let rounds = parse_rounds();

    for _ in 0..WARMUP_ITERATIONS {
        black_box(Backend::current());
    }

    let durations = (0..rounds)
        .map(|_| benchmark_round())
        .collect::<Vec<Duration>>();
    let ns_per_call = durations
        .iter()
        .map(|duration| duration.as_nanos() as f64 / ITERATIONS_PER_ROUND as f64)
        .collect::<Vec<f64>>();
    let backend = Backend::current();

    println!(
        concat!(
            "{{\"schema_version\":1,\"benchmark\":\"bootstrap_target_capability_query\",",
            "\"cpu\":\"{}\",\"os\":\"{}\",\"power_mode\":\"{}\",",
            "\"compiler\":\"{}\",\"llvm_version\":\"{}\",",
            "\"target_triple\":\"{}\",\"compile_target_features\":\"{}\",",
            "\"runtime_or_browser\":\"{}\",\"sample_rate_hz\":48000,",
            "\"quantum_frames\":128,\"fixture\":\"bootstrap_target_capability_query\",",
            "\"warmup_iterations\":{},\"iterations_per_round\":{},\"rounds\":{},",
            "\"round_duration_ns\":{},\"ns_per_call\":{},",
            "\"statistical_method\":\"median of per-round ns/call; descriptive only\",",
            "\"capabilities\":{{\"backend\":\"{:?}\",\"width\":{}}}}}"
        ),
        metadata("MISO_ENGINE_BENCH_CPU"),
        env::consts::OS,
        metadata("MISO_ENGINE_BENCH_POWER_MODE"),
        metadata("MISO_ENGINE_BENCH_COMPILER"),
        metadata("MISO_ENGINE_BENCH_LLVM_VERSION"),
        metadata("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
        metadata("MISO_ENGINE_BENCH_TARGET_FEATURES"),
        metadata("MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER"),
        WARMUP_ITERATIONS,
        ITERATIONS_PER_ROUND,
        rounds,
        json_u128_array(durations.iter().map(Duration::as_nanos)),
        json_f64_array(ns_per_call.iter().copied()),
        backend,
        backend.width(),
    );
}

fn benchmark_round() -> Duration {
    let started = Instant::now();

    for _ in 0..ITERATIONS_PER_ROUND {
        black_box(Backend::current());
    }

    started.elapsed()
}

fn parse_rounds() -> u8 {
    let arguments = env::args().skip(1).collect::<Vec<String>>();
    match arguments.as_slice() {
        [] => 2,
        [flag, value] if flag == "--rounds" && value == "1" => 1,
        [flag, value] if flag == "--rounds" && value == "2" => 2,
        _ => panic!("usage: miso_engine_bootstrap_bench [--rounds 1|2]"),
    }
}

fn metadata(name: &str) -> String {
    env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .map(|value| json_escape(&value))
        .unwrap_or_else(|| "unknown".to_owned())
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{00}'..='\u{1f}' => {
                write!(&mut escaped, "\\u{:04x}", character as u32)
                    .expect("writing to a String cannot fail");
            }
            _ => escaped.push(character),
        }
    }

    escaped
}

fn json_u128_array(values: impl Iterator<Item = u128>) -> String {
    format!(
        "[{}]",
        values
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn json_f64_array(values: impl Iterator<Item = f64>) -> String {
    format!(
        "[{}]",
        values
            .map(|value| format!("{value:.6}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::json_escape;

    #[test]
    fn json_escape_preserves_unicode_and_escapes_json_syntax() {
        assert_eq!(
            json_escape("quote=\" slash=\\ newline=\n apostrophe=' café \u{0001}"),
            "quote=\\\" slash=\\\\ newline=\\n apostrophe=' café \\u0001"
        );
    }
}
