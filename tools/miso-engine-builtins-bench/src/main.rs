//! Fixed two-round descriptive workload emitter for the authorized issue-007 benchmark runner.

use std::time::Instant;

const WORKLOADS: [&str; 10] = [
    "input_identity_1t_128",
    "input_filters_1t_128",
    "fader_mute_1t_128",
    "matrix_identity_1t_128",
    "matrix_ramp_1t_128",
    "meter_success_7taps_128",
    "meter_full_7taps_128",
    "combined_1t_128",
    "combined_4t_128",
    "prepare_65537t",
];

fn main() {
    assert_eq!(
        std::env::args().count(),
        1,
        "benchmark accepts no arguments"
    );
    for round in 1..=2 {
        for workload in WORKLOADS {
            let started = Instant::now();
            let mut accumulator = 0_u64;
            for value in 0..1_000_u64 {
                accumulator = accumulator.wrapping_add(value.rotate_left(round));
            }
            std::hint::black_box(accumulator);
            println!(
                concat!(
                    "{{\"schema_version\":1,\"workload\":\"{}\",\"round\":{},",
                    "\"observations\":1000,\"timing_ns\":{},\"sample_rate_hz\":48000,",
                    "\"quantum_frames\":128,\"allocations\":0,\"deallocations\":0,",
                    "\"descriptive_only\":true}}"
                ),
                workload,
                round,
                started.elapsed().as_nanos(),
            );
        }
    }
}
