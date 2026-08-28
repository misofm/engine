//! B1: one descriptive throughput measurement. Not a gate, and `#[ignore]`d by default.
//!
//! AGENTS.md's benchmark protocol: this is a single deliberate invocation, run by hand, reported as
//! a number and never tuned against. The baseline it is compared with is the #90 audit's replica of
//! the layout-1 path: **25.1 ns per lane-sample** on a Zen 5 host at `x86-64-v2`, of which about
//! 7 ns was the per-sample `powf` + `expf` and about 10 ns the unvectorised strided scalar FIR.
//!
//! Run with `cargo test --release -p miso-engine-true-peak-limiter --test bench -- --ignored
//! --nocapture`.

use miso_engine_lane::Backend;
use std::time::Instant;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
};
use miso_engine_true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR, TRUE_PEAK_LIMITER_PARAMETERS, TruePeakLimiterFactory,
};

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: TRUE_PEAK_LIMITER_PARAMETERS[index / 2].default_value,
    })
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// Seeded noise at about +3 dBFS, so the limiter is actually limiting for the whole run.
fn noise(length: usize, seed: u64) -> Vec<f32> {
    let mut state = seed;
    (0..length)
        .map(|_| {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut mixed = state;
            mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            mixed ^= mixed >> 31;
            (((mixed >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0) * 1.4125
        })
        .collect()
}

#[test]
#[ignore = "descriptive benchmark, run by hand (AGENTS.md benchmark protocol)"]
fn bench_w8_ns_per_lane_sample() {
    const SECONDS: usize = 20;
    const FRAMES: usize = 48_000 * SECONDS;
    const BLOCK: usize = 128;

    let values = values();
    let requests: Vec<PrepareEffectRequest<'_>> = (0..8).map(|_| request(&values)).collect();
    let mut bank = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("bank binding")
        .expect("bank available");
    let source = noise(FRAMES * 8, 0x5150_0090);
    let offsets = [0_u32; 9];

    let round = |bank: &mut Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>| {
        let mut left = source.clone();
        let mut right = source.clone();
        let start = Instant::now();
        for block in 0..FRAMES / BLOCK {
            let range = block * BLOCK * 8..(block + 1) * BLOCK * 8;
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left[range.clone()],
                    &mut right[range],
                    None,
                    BLOCK as u32,
                    BankWidth::Eight,
                    (block * BLOCK) as u64,
                    &[],
                    &offsets,
                    128,
                )
                .expect("bank block"),
            );
        }
        let elapsed = start.elapsed();
        // Two channels per lane-sample: the limiter's cost is per stereo frame per track.
        let lane_samples = (FRAMES / BLOCK * BLOCK * 8) as f64;
        elapsed.as_secs_f64() * 1.0e9 / lane_samples
    };

    let _warm_up = round(&mut bank);
    let first = round(&mut bank);
    let second = round(&mut bank);
    println!("W8 bank: {first:.2} ns and {second:.2} ns per lane-sample (stereo)");

    let mut scalar = TruePeakLimiterFactory
        .prepare(request(&values))
        .expect("prepare");
    let mut left = noise(FRAMES, 0x9001);
    let mut right = noise(FRAMES, 0x9002);
    let start = Instant::now();
    for block in 0..FRAMES / BLOCK {
        let range = block * BLOCK..(block + 1) * BLOCK;
        let (left, right) = (&mut left[range.clone()], &mut right[range]);
        scalar.process(
            EffectProcessBlock::new(left, right, None, (block * BLOCK) as u64, &[], 128)
                .expect("block"),
        );
    }
    let scalar_ns = start.elapsed().as_secs_f64() * 1.0e9 / (FRAMES / BLOCK * BLOCK) as f64;
    println!("scalar (W=1): {scalar_ns:.2} ns per lane-sample (stereo)");
}
