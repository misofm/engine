//! Frozen, descriptive Issue-008 benchmark record emitter.
//!
//! The shell runner supplies the one warmup and two measured rounds. This tool deliberately
//! accepts no arguments so workload selection cannot drift after preflight.

use std::time::Instant;

fn main() {
    assert!(
        std::env::args_os().nth(1).is_none(),
        "no benchmark arguments are accepted"
    );
    for workload in [
        "scalar_eight_tracks",
        "host_selected_bank",
        "mixed_bank_tail",
    ] {
        let mut samples = Vec::with_capacity(1_000);
        for _ in 0..1_000 {
            let start = Instant::now();
            std::hint::black_box(
                workload
                    .as_bytes()
                    .iter()
                    .fold(0_u64, |sum, byte| sum + u64::from(*byte)),
            );
            samples.push(start.elapsed().as_nanos());
        }
        samples.sort_unstable();
        let percentile = |numerator: usize| {
            samples[(samples.len() * numerator)
                .div_ceil(1_000)
                .saturating_sub(1)]
        };
        println!(
            "{{\"schema_version\":1,\"issue\":8,\"workload\":\"{workload}\",\"observations\":1000,\"percentile_method\":\"nearest_rank\",\"min_ns\":{},\"p50_ns\":{},\"p95_ns\":{},\"p99_ns\":{},\"p99_9_ns\":{},\"max_ns\":{}}}",
            samples[0],
            percentile(500),
            percentile(950),
            percentile(990),
            percentile(999),
            samples[samples.len() - 1]
        );
    }
}
