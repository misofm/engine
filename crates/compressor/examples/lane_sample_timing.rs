//! Descriptive lane-sample timing for the launch compressor (#88 step 1 / E11).
//!
//! Measured once before the rewrite and once after, on the same host, and reported verbatim
//! (AGENTS.md: a descriptive benchmark is never tuned toward). Nothing here is a gate.
//!
//! The timed region is exactly `process` / `process_bank` over `BLOCKS` blocks of `FRAMES`
//! frames. Buffers are filled before the region and reused, so the number is the render path and
//! not an allocator. The reported unit is nanoseconds per *lane-sample*: one sample of one channel
//! of one track, which is the only unit in which a scalar instance and a `W`-wide bank compare.

use compressor::{COMPRESSOR_PARAMETERS, CompressorFactory};
use conformance::SplitMix64;
use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PortId, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedPorts, PreparedSidechainPort,
};
use lane::Backend;

const FRAMES: usize = 128;
const BLOCKS: usize = 4_096;
const WARMUP_BLOCKS: usize = 512;
const ROUNDS: usize = 2;

fn initial_values() -> [InitialParameterValue; 16] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: COMPRESSOR_PARAMETERS[index / 2].default_value,
    })
}

fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: PortId::new("sidechain-in").expect("port id"),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn noise(samples: usize, seed: u64) -> Vec<f32> {
    let mut generator = SplitMix64::new(seed);
    (0..samples)
        .map(|_| generator.next_bipolar_f32() * 0.5)
        .collect()
}

fn scalar_nanoseconds_per_lane_sample() -> f64 {
    let values = initial_values();
    let mut effect = CompressorFactory
        .prepare(request(&values))
        .expect("scalar prepare");
    let mut left = noise(FRAMES, 0x5EED_0001);
    let mut right = noise(FRAMES, 0x5EED_0002);
    let mut best = f64::INFINITY;
    for round in 0..=ROUNDS {
        let blocks = if round == 0 { WARMUP_BLOCKS } else { BLOCKS };
        let start = std::time::Instant::now();
        for block in 0..blocks {
            effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    (block * FRAMES) as u64,
                    &[],
                    FRAMES as u32,
                )
                .expect("block"),
            );
        }
        let elapsed = start.elapsed().as_nanos() as f64;
        if round != 0 {
            best = best.min(elapsed / (blocks * FRAMES * 2) as f64);
        }
    }
    best
}

fn bank_nanoseconds_per_lane_sample() -> Option<(usize, f64)> {
    let backend = Backend::current();
    let width = BankWidth::for_backend(backend)?;
    let lanes = width.lanes() as usize;
    // Ragged lookahead, 1 ms per lane, matching the standing console fixture's per-track spread.
    // A bank whose lanes agree about their detector tap is not the bank the fixture renders, and
    // the per-lane tap is the part of the ring walk that costs.
    let values: Vec<_> = (0..lanes)
        .map(|lane| {
            let mut values = initial_values();
            for value in values.iter_mut() {
                if value.parameter_index == 7 {
                    value.value = (lane + 1) as f32;
                }
            }
            values
        })
        .collect();
    let requests = values.iter().map(|v| request(v)).collect::<Vec<_>>();
    let mut bank = CompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
        })
        .expect("bank bind")?;
    let mut left = noise(FRAMES * lanes, 0x5EED_0003);
    let mut right = noise(FRAMES * lanes, 0x5EED_0004);
    let offsets = vec![0_u32; lanes + 1];
    let mut best = f64::INFINITY;
    for round in 0..=ROUNDS {
        let blocks = if round == 0 { WARMUP_BLOCKS } else { BLOCKS };
        let start = std::time::Instant::now();
        for block in 0..blocks {
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    FRAMES as u32,
                    width,
                    (block * FRAMES) as u64,
                    &[],
                    &offsets,
                    FRAMES as u32,
                )
                .expect("bank block"),
            );
        }
        let elapsed = start.elapsed().as_nanos() as f64;
        if round != 0 {
            best = best.min(elapsed / (blocks * FRAMES * 2 * lanes) as f64);
        }
    }
    Some((lanes, best))
}

fn main() {
    let _: Backend = Backend::current();
    let scalar = scalar_nanoseconds_per_lane_sample();
    println!("compressor scalar: {scalar:.3} ns/lane-sample");
    match bank_nanoseconds_per_lane_sample() {
        Some((lanes, bank)) => {
            println!("compressor bank W{lanes}: {bank:.3} ns/lane-sample");
            println!("ratio scalar/bank: {:.2}x", scalar / bank);
        }
        None => println!("compressor bank: not available on this backend"),
    }
}
